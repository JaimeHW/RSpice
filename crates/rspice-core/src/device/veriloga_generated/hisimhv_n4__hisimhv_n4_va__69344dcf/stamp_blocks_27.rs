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
        let (eq4_e1052, eq4_e1052_d_n0, eq4_e1052_d_n1, eq4_e1052_d_n2, eq4_e1052_d_n3, eq4_e1052_d_n4, eq4_e1052_d_n5, eq4_e1052_d_n6, eq4_e1052_d_n7, eq4_e1052_d_n8, eq4_e1052_d_n9, eq4_e1052_d_n10, eq4_e1052_d_n11, eq4_e1052_d_n12, eq4_e1052_d_n13, eq4_e1052_d_n14, eq4_e1052_d_n15, eq4_e1052_d_n16, eq4_e1052_d_n17, eq4_e1052_d_b0, eq4_e1052_d_b1, eq4_e1052_d_b2, eq4_e1052_d_b3, eq4_e1052_d_b4, eq4_e1052_d_b5, eq4_e1052_d_b6, eq4_e1052_d_b7, eq4_e1052_d_b8, eq4_e1052_d_b9, eq4_e1052_d_b10, eq4_e1052_d_b11,) = {
    if s.b[3306] {
        let eq4_e1049: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, s.v[931]);let eq4_e1050: f64 = (s.v[932] + eq4_e1049);let eq4_e1050_d_n0: f64 = (s.dn[932][0] + (s.dn[931][0] * ddt_scale));let eq4_e1050_d_n1: f64 = (s.dn[932][1] + (s.dn[931][1] * ddt_scale));let eq4_e1050_d_n2: f64 = (s.dn[932][2] + (s.dn[931][2] * ddt_scale));let eq4_e1050_d_n3: f64 = (s.dn[932][3] + (s.dn[931][3] * ddt_scale));let eq4_e1050_d_n4: f64 = (s.dn[932][4] + (s.dn[931][4] * ddt_scale));let eq4_e1050_d_n5: f64 = (s.dn[932][5] + (s.dn[931][5] * ddt_scale));let eq4_e1050_d_n6: f64 = (s.dn[932][6] + (s.dn[931][6] * ddt_scale));let eq4_e1050_d_n7: f64 = (s.dn[932][7] + (s.dn[931][7] * ddt_scale));let eq4_e1050_d_n8: f64 = (s.dn[932][8] + (s.dn[931][8] * ddt_scale));let eq4_e1050_d_n9: f64 = (s.dn[932][9] + (s.dn[931][9] * ddt_scale));let eq4_e1050_d_n10: f64 = (s.dn[932][10] + (s.dn[931][10] * ddt_scale));let eq4_e1050_d_n11: f64 = (s.dn[932][11] + (s.dn[931][11] * ddt_scale));let eq4_e1050_d_n12: f64 = (s.dn[932][12] + (s.dn[931][12] * ddt_scale));let eq4_e1050_d_n13: f64 = (s.dn[932][13] + (s.dn[931][13] * ddt_scale));let eq4_e1050_d_n14: f64 = (s.dn[932][14] + (s.dn[931][14] * ddt_scale));let eq4_e1050_d_n15: f64 = (s.dn[932][15] + (s.dn[931][15] * ddt_scale));let eq4_e1050_d_n16: f64 = (s.dn[932][16] + (s.dn[931][16] * ddt_scale));let eq4_e1050_d_n17: f64 = (s.dn[932][17] + (s.dn[931][17] * ddt_scale));let eq4_e1050_d_b0: f64 = (s.db[932][0] + (s.db[931][0] * ddt_scale));let eq4_e1050_d_b1: f64 = (s.db[932][1] + (s.db[931][1] * ddt_scale));let eq4_e1050_d_b2: f64 = (s.db[932][2] + (s.db[931][2] * ddt_scale));let eq4_e1050_d_b3: f64 = (s.db[932][3] + (s.db[931][3] * ddt_scale));let eq4_e1050_d_b4: f64 = (s.db[932][4] + (s.db[931][4] * ddt_scale));let eq4_e1050_d_b5: f64 = (s.db[932][5] + (s.db[931][5] * ddt_scale));let eq4_e1050_d_b6: f64 = (s.db[932][6] + (s.db[931][6] * ddt_scale));let eq4_e1050_d_b7: f64 = (s.db[932][7] + (s.db[931][7] * ddt_scale));let eq4_e1050_d_b8: f64 = (s.db[932][8] + (s.db[931][8] * ddt_scale));let eq4_e1050_d_b9: f64 = (s.db[932][9] + (s.db[931][9] * ddt_scale));let eq4_e1050_d_b10: f64 = (s.db[932][10] + (s.db[931][10] * ddt_scale));let eq4_e1050_d_b11: f64 = (s.db[932][11] + (s.db[931][11] * ddt_scale));
        (eq4_e1050, eq4_e1050_d_n0, eq4_e1050_d_n1, eq4_e1050_d_n2, eq4_e1050_d_n3, eq4_e1050_d_n4, eq4_e1050_d_n5, eq4_e1050_d_n6, eq4_e1050_d_n7, eq4_e1050_d_n8, eq4_e1050_d_n9, eq4_e1050_d_n10, eq4_e1050_d_n11, eq4_e1050_d_n12, eq4_e1050_d_n13, eq4_e1050_d_n14, eq4_e1050_d_n15, eq4_e1050_d_n16, eq4_e1050_d_n17, eq4_e1050_d_b0, eq4_e1050_d_b1, eq4_e1050_d_b2, eq4_e1050_d_b3, eq4_e1050_d_b4, eq4_e1050_d_b5, eq4_e1050_d_b6, eq4_e1050_d_b7, eq4_e1050_d_b8, eq4_e1050_d_b9, eq4_e1050_d_b10, eq4_e1050_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1052;let eq4_node_derivatives: [f64; 18] = [eq4_e1052_d_n0, eq4_e1052_d_n1, eq4_e1052_d_n2, eq4_e1052_d_n3, eq4_e1052_d_n4, eq4_e1052_d_n5, eq4_e1052_d_n6, eq4_e1052_d_n7, eq4_e1052_d_n8, eq4_e1052_d_n9, eq4_e1052_d_n10, eq4_e1052_d_n11, eq4_e1052_d_n12, eq4_e1052_d_n13, eq4_e1052_d_n14, eq4_e1052_d_n15, eq4_e1052_d_n16, eq4_e1052_d_n17];let eq4_branch_derivatives: [f64; 12] = [eq4_e1052_d_b0, eq4_e1052_d_b1, eq4_e1052_d_b2, eq4_e1052_d_b3, eq4_e1052_d_b4, eq4_e1052_d_b5, eq4_e1052_d_b6, eq4_e1052_d_b7, eq4_e1052_d_b8, eq4_e1052_d_b9, eq4_e1052_d_b10, eq4_e1052_d_b11];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1057,) = {
    if (!s.b[3306]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1057;
        stamper.stamp_potential_const_local(
            2,
            eq5_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq6_e1061: f64 = (s.v[134] + s.v[400]);let eq6_e1061_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);let eq6_e1061_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);let eq6_e1061_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);let eq6_e1061_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);let eq6_e1061_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);let eq6_e1061_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);let eq6_e1061_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);let eq6_e1061_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);let eq6_e1061_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);let eq6_e1061_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);let eq6_e1061_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);let eq6_e1061_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);let eq6_e1061_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);let eq6_e1061_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);let eq6_e1061_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);let eq6_e1061_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);let eq6_e1061_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);let eq6_e1061_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);let eq6_e1061_d_b0: f64 = (s.db[134][0] + s.db[400][0]);let eq6_e1061_d_b1: f64 = (s.db[134][1] + s.db[400][1]);let eq6_e1061_d_b2: f64 = (s.db[134][2] + s.db[400][2]);let eq6_e1061_d_b3: f64 = (s.db[134][3] + s.db[400][3]);let eq6_e1061_d_b4: f64 = (s.db[134][4] + s.db[400][4]);let eq6_e1061_d_b5: f64 = (s.db[134][5] + s.db[400][5]);let eq6_e1061_d_b6: f64 = (s.db[134][6] + s.db[400][6]);let eq6_e1061_d_b7: f64 = (s.db[134][7] + s.db[400][7]);let eq6_e1061_d_b8: f64 = (s.db[134][8] + s.db[400][8]);let eq6_e1061_d_b9: f64 = (s.db[134][9] + s.db[400][9]);let eq6_e1061_d_b10: f64 = (s.db[134][10] + s.db[400][10]);let eq6_e1061_d_b11: f64 = (s.db[134][11] + s.db[400][11]);let eq6_e1063: f64 = (eq6_e1061 - s.v[738]);let eq6_e1063_d_n0: f64 = (eq6_e1061_d_n0 - s.dn[738][0]);let eq6_e1063_d_n1: f64 = (eq6_e1061_d_n1 - s.dn[738][1]);let eq6_e1063_d_n2: f64 = (eq6_e1061_d_n2 - s.dn[738][2]);let eq6_e1063_d_n3: f64 = (eq6_e1061_d_n3 - s.dn[738][3]);let eq6_e1063_d_n4: f64 = (eq6_e1061_d_n4 - s.dn[738][4]);let eq6_e1063_d_n5: f64 = (eq6_e1061_d_n5 - s.dn[738][5]);let eq6_e1063_d_n6: f64 = (eq6_e1061_d_n6 - s.dn[738][6]);let eq6_e1063_d_n7: f64 = (eq6_e1061_d_n7 - s.dn[738][7]);let eq6_e1063_d_n8: f64 = (eq6_e1061_d_n8 - s.dn[738][8]);let eq6_e1063_d_n9: f64 = (eq6_e1061_d_n9 - s.dn[738][9]);let eq6_e1063_d_n10: f64 = (eq6_e1061_d_n10 - s.dn[738][10]);let eq6_e1063_d_n11: f64 = (eq6_e1061_d_n11 - s.dn[738][11]);let eq6_e1063_d_n12: f64 = (eq6_e1061_d_n12 - s.dn[738][12]);let eq6_e1063_d_n13: f64 = (eq6_e1061_d_n13 - s.dn[738][13]);let eq6_e1063_d_n14: f64 = (eq6_e1061_d_n14 - s.dn[738][14]);let eq6_e1063_d_n15: f64 = (eq6_e1061_d_n15 - s.dn[738][15]);let eq6_e1063_d_n16: f64 = (eq6_e1061_d_n16 - s.dn[738][16]);let eq6_e1063_d_n17: f64 = (eq6_e1061_d_n17 - s.dn[738][17]);let eq6_e1063_d_b0: f64 = (eq6_e1061_d_b0 - s.db[738][0]);let eq6_e1063_d_b1: f64 = (eq6_e1061_d_b1 - s.db[738][1]);let eq6_e1063_d_b2: f64 = (eq6_e1061_d_b2 - s.db[738][2]);let eq6_e1063_d_b3: f64 = (eq6_e1061_d_b3 - s.db[738][3]);let eq6_e1063_d_b4: f64 = (eq6_e1061_d_b4 - s.db[738][4]);let eq6_e1063_d_b5: f64 = (eq6_e1061_d_b5 - s.db[738][5]);let eq6_e1063_d_b6: f64 = (eq6_e1061_d_b6 - s.db[738][6]);let eq6_e1063_d_b7: f64 = (eq6_e1061_d_b7 - s.db[738][7]);let eq6_e1063_d_b8: f64 = (eq6_e1061_d_b8 - s.db[738][8]);let eq6_e1063_d_b9: f64 = (eq6_e1061_d_b9 - s.db[738][9]);let eq6_e1063_d_b10: f64 = (eq6_e1061_d_b10 - s.db[738][10]);let eq6_e1063_d_b11: f64 = (eq6_e1061_d_b11 - s.db[738][11]);let eq6_e1064: f64 = (p[87] * eq6_e1063);let eq6_e1064_d_n0: f64 = (p[87] * eq6_e1063_d_n0);let eq6_e1064_d_n1: f64 = (p[87] * eq6_e1063_d_n1);let eq6_e1064_d_n2: f64 = (p[87] * eq6_e1063_d_n2);let eq6_e1064_d_n3: f64 = (p[87] * eq6_e1063_d_n3);let eq6_e1064_d_n4: f64 = (p[87] * eq6_e1063_d_n4);let eq6_e1064_d_n5: f64 = (p[87] * eq6_e1063_d_n5);let eq6_e1064_d_n6: f64 = (p[87] * eq6_e1063_d_n6);let eq6_e1064_d_n7: f64 = (p[87] * eq6_e1063_d_n7);let eq6_e1064_d_n8: f64 = (p[87] * eq6_e1063_d_n8);
        let eq6_e1064_d_n9: f64 = (p[87] * eq6_e1063_d_n9);let eq6_e1064_d_n10: f64 = (p[87] * eq6_e1063_d_n10);let eq6_e1064_d_n11: f64 = (p[87] * eq6_e1063_d_n11);let eq6_e1064_d_n12: f64 = (p[87] * eq6_e1063_d_n12);let eq6_e1064_d_n13: f64 = (p[87] * eq6_e1063_d_n13);let eq6_e1064_d_n14: f64 = (p[87] * eq6_e1063_d_n14);let eq6_e1064_d_n15: f64 = (p[87] * eq6_e1063_d_n15);let eq6_e1064_d_n16: f64 = (p[87] * eq6_e1063_d_n16);let eq6_e1064_d_n17: f64 = (p[87] * eq6_e1063_d_n17);let eq6_e1064_d_b0: f64 = (p[87] * eq6_e1063_d_b0);let eq6_e1064_d_b1: f64 = (p[87] * eq6_e1063_d_b1);let eq6_e1064_d_b2: f64 = (p[87] * eq6_e1063_d_b2);let eq6_e1064_d_b3: f64 = (p[87] * eq6_e1063_d_b3);let eq6_e1064_d_b4: f64 = (p[87] * eq6_e1063_d_b4);let eq6_e1064_d_b5: f64 = (p[87] * eq6_e1063_d_b5);let eq6_e1064_d_b6: f64 = (p[87] * eq6_e1063_d_b6);let eq6_e1064_d_b7: f64 = (p[87] * eq6_e1063_d_b7);let eq6_e1064_d_b8: f64 = (p[87] * eq6_e1063_d_b8);let eq6_e1064_d_b9: f64 = (p[87] * eq6_e1063_d_b9);let eq6_e1064_d_b10: f64 = (p[87] * eq6_e1063_d_b10);let eq6_e1064_d_b11: f64 = (p[87] * eq6_e1063_d_b11);let eq6_value: f64 = eq6_e1064;let eq6_node_derivatives: [f64; 18] = [eq6_e1064_d_n0, eq6_e1064_d_n1, eq6_e1064_d_n2, eq6_e1064_d_n3, eq6_e1064_d_n4, eq6_e1064_d_n5, eq6_e1064_d_n6, eq6_e1064_d_n7, eq6_e1064_d_n8, eq6_e1064_d_n9, eq6_e1064_d_n10, eq6_e1064_d_n11, eq6_e1064_d_n12, eq6_e1064_d_n13, eq6_e1064_d_n14, eq6_e1064_d_n15, eq6_e1064_d_n16, eq6_e1064_d_n17];let eq6_branch_derivatives: [f64; 12] = [eq6_e1064_d_b0, eq6_e1064_d_b1, eq6_e1064_d_b2, eq6_e1064_d_b3, eq6_e1064_d_b4, eq6_e1064_d_b5, eq6_e1064_d_b6, eq6_e1064_d_b7, eq6_e1064_d_b8, eq6_e1064_d_b9, eq6_e1064_d_b10, eq6_e1064_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
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
        let eq7_e1068: f64 = (s.v[424] - s.v[425]);let eq7_e1068_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);let eq7_e1068_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);let eq7_e1068_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);let eq7_e1068_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);let eq7_e1068_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);let eq7_e1068_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);let eq7_e1068_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);let eq7_e1068_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);let eq7_e1068_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);let eq7_e1068_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);let eq7_e1068_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);let eq7_e1068_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);let eq7_e1068_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);let eq7_e1068_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);let eq7_e1068_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);let eq7_e1068_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);let eq7_e1068_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);let eq7_e1068_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);let eq7_e1068_d_b0: f64 = (s.db[424][0] - s.db[425][0]);let eq7_e1068_d_b1: f64 = (s.db[424][1] - s.db[425][1]);let eq7_e1068_d_b2: f64 = (s.db[424][2] - s.db[425][2]);let eq7_e1068_d_b3: f64 = (s.db[424][3] - s.db[425][3]);let eq7_e1068_d_b4: f64 = (s.db[424][4] - s.db[425][4]);let eq7_e1068_d_b5: f64 = (s.db[424][5] - s.db[425][5]);let eq7_e1068_d_b6: f64 = (s.db[424][6] - s.db[425][6]);let eq7_e1068_d_b7: f64 = (s.db[424][7] - s.db[425][7]);let eq7_e1068_d_b8: f64 = (s.db[424][8] - s.db[425][8]);let eq7_e1068_d_b9: f64 = (s.db[424][9] - s.db[425][9]);let eq7_e1068_d_b10: f64 = (s.db[424][10] - s.db[425][10]);let eq7_e1068_d_b11: f64 = (s.db[424][11] - s.db[425][11]);let eq7_e1069: f64 = (p[87] * eq7_e1068);let eq7_e1069_d_n0: f64 = (p[87] * eq7_e1068_d_n0);let eq7_e1069_d_n1: f64 = (p[87] * eq7_e1068_d_n1);let eq7_e1069_d_n2: f64 = (p[87] * eq7_e1068_d_n2);let eq7_e1069_d_n3: f64 = (p[87] * eq7_e1068_d_n3);let eq7_e1069_d_n4: f64 = (p[87] * eq7_e1068_d_n4);let eq7_e1069_d_n5: f64 = (p[87] * eq7_e1068_d_n5);let eq7_e1069_d_n6: f64 = (p[87] * eq7_e1068_d_n6);let eq7_e1069_d_n7: f64 = (p[87] * eq7_e1068_d_n7);let eq7_e1069_d_n8: f64 = (p[87] * eq7_e1068_d_n8);let eq7_e1069_d_n9: f64 = (p[87] * eq7_e1068_d_n9);let eq7_e1069_d_n10: f64 = (p[87] * eq7_e1068_d_n10);let eq7_e1069_d_n11: f64 = (p[87] * eq7_e1068_d_n11);let eq7_e1069_d_n12: f64 = (p[87] * eq7_e1068_d_n12);let eq7_e1069_d_n13: f64 = (p[87] * eq7_e1068_d_n13);let eq7_e1069_d_n14: f64 = (p[87] * eq7_e1068_d_n14);let eq7_e1069_d_n15: f64 = (p[87] * eq7_e1068_d_n15);let eq7_e1069_d_n16: f64 = (p[87] * eq7_e1068_d_n16);let eq7_e1069_d_n17: f64 = (p[87] * eq7_e1068_d_n17);let eq7_e1069_d_b0: f64 = (p[87] * eq7_e1068_d_b0);let eq7_e1069_d_b1: f64 = (p[87] * eq7_e1068_d_b1);let eq7_e1069_d_b2: f64 = (p[87] * eq7_e1068_d_b2);let eq7_e1069_d_b3: f64 = (p[87] * eq7_e1068_d_b3);let eq7_e1069_d_b4: f64 = (p[87] * eq7_e1068_d_b4);let eq7_e1069_d_b5: f64 = (p[87] * eq7_e1068_d_b5);let eq7_e1069_d_b6: f64 = (p[87] * eq7_e1068_d_b6);let eq7_e1069_d_b7: f64 = (p[87] * eq7_e1068_d_b7);let eq7_e1069_d_b8: f64 = (p[87] * eq7_e1068_d_b8);let eq7_e1069_d_b9: f64 = (p[87] * eq7_e1068_d_b9);let eq7_e1069_d_b10: f64 = (p[87] * eq7_e1068_d_b10);let eq7_e1069_d_b11: f64 = (p[87] * eq7_e1068_d_b11);let eq7_value: f64 = eq7_e1069;let eq7_node_derivatives: [f64; 18] = [eq7_e1069_d_n0, eq7_e1069_d_n1, eq7_e1069_d_n2, eq7_e1069_d_n3, eq7_e1069_d_n4, eq7_e1069_d_n5, eq7_e1069_d_n6, eq7_e1069_d_n7, eq7_e1069_d_n8, eq7_e1069_d_n9, eq7_e1069_d_n10, eq7_e1069_d_n11, eq7_e1069_d_n12, eq7_e1069_d_n13, eq7_e1069_d_n14, eq7_e1069_d_n15, eq7_e1069_d_n16, eq7_e1069_d_n17];let eq7_branch_derivatives: [f64; 12] = [eq7_e1069_d_b0, eq7_e1069_d_b1, eq7_e1069_d_b2, eq7_e1069_d_b3, eq7_e1069_d_b4, eq7_e1069_d_b5, eq7_e1069_d_b6, eq7_e1069_d_b7, eq7_e1069_d_b8, eq7_e1069_d_b9, eq7_e1069_d_b10, eq7_e1069_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
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
        let eq8_e1073: f64 = (s.v[203] + s.v[280]);let eq8_e1073_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);let eq8_e1073_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);let eq8_e1073_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);let eq8_e1073_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);let eq8_e1073_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);let eq8_e1073_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);let eq8_e1073_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);let eq8_e1073_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);let eq8_e1073_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);let eq8_e1073_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);let eq8_e1073_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);let eq8_e1073_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);let eq8_e1073_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);let eq8_e1073_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);let eq8_e1073_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);let eq8_e1073_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);let eq8_e1073_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);let eq8_e1073_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);let eq8_e1073_d_b0: f64 = (s.db[203][0] + s.db[280][0]);let eq8_e1073_d_b1: f64 = (s.db[203][1] + s.db[280][1]);let eq8_e1073_d_b2: f64 = (s.db[203][2] + s.db[280][2]);let eq8_e1073_d_b3: f64 = (s.db[203][3] + s.db[280][3]);let eq8_e1073_d_b4: f64 = (s.db[203][4] + s.db[280][4]);let eq8_e1073_d_b5: f64 = (s.db[203][5] + s.db[280][5]);let eq8_e1073_d_b6: f64 = (s.db[203][6] + s.db[280][6]);let eq8_e1073_d_b7: f64 = (s.db[203][7] + s.db[280][7]);let eq8_e1073_d_b8: f64 = (s.db[203][8] + s.db[280][8]);let eq8_e1073_d_b9: f64 = (s.db[203][9] + s.db[280][9]);let eq8_e1073_d_b10: f64 = (s.db[203][10] + s.db[280][10]);let eq8_e1073_d_b11: f64 = (s.db[203][11] + s.db[280][11]);let eq8_e1075: f64 = (eq8_e1073 + s.v[431]);let eq8_e1075_d_n0: f64 = (eq8_e1073_d_n0 + s.dn[431][0]);let eq8_e1075_d_n1: f64 = (eq8_e1073_d_n1 + s.dn[431][1]);let eq8_e1075_d_n2: f64 = (eq8_e1073_d_n2 + s.dn[431][2]);let eq8_e1075_d_n3: f64 = (eq8_e1073_d_n3 + s.dn[431][3]);let eq8_e1075_d_n4: f64 = (eq8_e1073_d_n4 + s.dn[431][4]);let eq8_e1075_d_n5: f64 = (eq8_e1073_d_n5 + s.dn[431][5]);let eq8_e1075_d_n6: f64 = (eq8_e1073_d_n6 + s.dn[431][6]);let eq8_e1075_d_n7: f64 = (eq8_e1073_d_n7 + s.dn[431][7]);let eq8_e1075_d_n8: f64 = (eq8_e1073_d_n8 + s.dn[431][8]);let eq8_e1075_d_n9: f64 = (eq8_e1073_d_n9 + s.dn[431][9]);let eq8_e1075_d_n10: f64 = (eq8_e1073_d_n10 + s.dn[431][10]);let eq8_e1075_d_n11: f64 = (eq8_e1073_d_n11 + s.dn[431][11]);let eq8_e1075_d_n12: f64 = (eq8_e1073_d_n12 + s.dn[431][12]);let eq8_e1075_d_n13: f64 = (eq8_e1073_d_n13 + s.dn[431][13]);let eq8_e1075_d_n14: f64 = (eq8_e1073_d_n14 + s.dn[431][14]);let eq8_e1075_d_n15: f64 = (eq8_e1073_d_n15 + s.dn[431][15]);let eq8_e1075_d_n16: f64 = (eq8_e1073_d_n16 + s.dn[431][16]);let eq8_e1075_d_n17: f64 = (eq8_e1073_d_n17 + s.dn[431][17]);let eq8_e1075_d_b0: f64 = (eq8_e1073_d_b0 + s.db[431][0]);let eq8_e1075_d_b1: f64 = (eq8_e1073_d_b1 + s.db[431][1]);let eq8_e1075_d_b2: f64 = (eq8_e1073_d_b2 + s.db[431][2]);let eq8_e1075_d_b3: f64 = (eq8_e1073_d_b3 + s.db[431][3]);let eq8_e1075_d_b4: f64 = (eq8_e1073_d_b4 + s.db[431][4]);let eq8_e1075_d_b5: f64 = (eq8_e1073_d_b5 + s.db[431][5]);let eq8_e1075_d_b6: f64 = (eq8_e1073_d_b6 + s.db[431][6]);let eq8_e1075_d_b7: f64 = (eq8_e1073_d_b7 + s.db[431][7]);let eq8_e1075_d_b8: f64 = (eq8_e1073_d_b8 + s.db[431][8]);let eq8_e1075_d_b9: f64 = (eq8_e1073_d_b9 + s.db[431][9]);let eq8_e1075_d_b10: f64 = (eq8_e1073_d_b10 + s.db[431][10]);let eq8_e1075_d_b11: f64 = (eq8_e1073_d_b11 + s.db[431][11]);let eq8_e1076: f64 = (p[87] * eq8_e1075);let eq8_e1076_d_n0: f64 = (p[87] * eq8_e1075_d_n0);let eq8_e1076_d_n1: f64 = (p[87] * eq8_e1075_d_n1);let eq8_e1076_d_n2: f64 = (p[87] * eq8_e1075_d_n2);let eq8_e1076_d_n3: f64 = (p[87] * eq8_e1075_d_n3);let eq8_e1076_d_n4: f64 = (p[87] * eq8_e1075_d_n4);let eq8_e1076_d_n5: f64 = (p[87] * eq8_e1075_d_n5);let eq8_e1076_d_n6: f64 = (p[87] * eq8_e1075_d_n6);let eq8_e1076_d_n7: f64 = (p[87] * eq8_e1075_d_n7);let eq8_e1076_d_n8: f64 = (p[87] * eq8_e1075_d_n8);
        let eq8_e1076_d_n9: f64 = (p[87] * eq8_e1075_d_n9);let eq8_e1076_d_n10: f64 = (p[87] * eq8_e1075_d_n10);let eq8_e1076_d_n11: f64 = (p[87] * eq8_e1075_d_n11);let eq8_e1076_d_n12: f64 = (p[87] * eq8_e1075_d_n12);let eq8_e1076_d_n13: f64 = (p[87] * eq8_e1075_d_n13);let eq8_e1076_d_n14: f64 = (p[87] * eq8_e1075_d_n14);let eq8_e1076_d_n15: f64 = (p[87] * eq8_e1075_d_n15);let eq8_e1076_d_n16: f64 = (p[87] * eq8_e1075_d_n16);let eq8_e1076_d_n17: f64 = (p[87] * eq8_e1075_d_n17);let eq8_e1076_d_b0: f64 = (p[87] * eq8_e1075_d_b0);let eq8_e1076_d_b1: f64 = (p[87] * eq8_e1075_d_b1);let eq8_e1076_d_b2: f64 = (p[87] * eq8_e1075_d_b2);let eq8_e1076_d_b3: f64 = (p[87] * eq8_e1075_d_b3);let eq8_e1076_d_b4: f64 = (p[87] * eq8_e1075_d_b4);let eq8_e1076_d_b5: f64 = (p[87] * eq8_e1075_d_b5);let eq8_e1076_d_b6: f64 = (p[87] * eq8_e1075_d_b6);let eq8_e1076_d_b7: f64 = (p[87] * eq8_e1075_d_b7);let eq8_e1076_d_b8: f64 = (p[87] * eq8_e1075_d_b8);let eq8_e1076_d_b9: f64 = (p[87] * eq8_e1075_d_b9);let eq8_e1076_d_b10: f64 = (p[87] * eq8_e1075_d_b10);let eq8_e1076_d_b11: f64 = (p[87] * eq8_e1075_d_b11);let eq8_value: f64 = eq8_e1076;let eq8_node_derivatives: [f64; 18] = [eq8_e1076_d_n0, eq8_e1076_d_n1, eq8_e1076_d_n2, eq8_e1076_d_n3, eq8_e1076_d_n4, eq8_e1076_d_n5, eq8_e1076_d_n6, eq8_e1076_d_n7, eq8_e1076_d_n8, eq8_e1076_d_n9, eq8_e1076_d_n10, eq8_e1076_d_n11, eq8_e1076_d_n12, eq8_e1076_d_n13, eq8_e1076_d_n14, eq8_e1076_d_n15, eq8_e1076_d_n16, eq8_e1076_d_n17];let eq8_branch_derivatives: [f64; 12] = [eq8_e1076_d_b0, eq8_e1076_d_b1, eq8_e1076_d_b2, eq8_e1076_d_b3, eq8_e1076_d_b4, eq8_e1076_d_b5, eq8_e1076_d_b6, eq8_e1076_d_b7, eq8_e1076_d_b8, eq8_e1076_d_b9, eq8_e1076_d_b10, eq8_e1076_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
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
        let eq9_e1080: f64 = (s.v[204] + s.v[736]);let eq9_e1080_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);let eq9_e1080_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);let eq9_e1080_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);let eq9_e1080_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);let eq9_e1080_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);let eq9_e1080_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);let eq9_e1080_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);let eq9_e1080_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);let eq9_e1080_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);let eq9_e1080_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);let eq9_e1080_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);let eq9_e1080_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);let eq9_e1080_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);let eq9_e1080_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);let eq9_e1080_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);let eq9_e1080_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);let eq9_e1080_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);let eq9_e1080_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);let eq9_e1080_d_b0: f64 = (s.db[204][0] + s.db[736][0]);let eq9_e1080_d_b1: f64 = (s.db[204][1] + s.db[736][1]);let eq9_e1080_d_b2: f64 = (s.db[204][2] + s.db[736][2]);let eq9_e1080_d_b3: f64 = (s.db[204][3] + s.db[736][3]);let eq9_e1080_d_b4: f64 = (s.db[204][4] + s.db[736][4]);let eq9_e1080_d_b5: f64 = (s.db[204][5] + s.db[736][5]);let eq9_e1080_d_b6: f64 = (s.db[204][6] + s.db[736][6]);let eq9_e1080_d_b7: f64 = (s.db[204][7] + s.db[736][7]);let eq9_e1080_d_b8: f64 = (s.db[204][8] + s.db[736][8]);let eq9_e1080_d_b9: f64 = (s.db[204][9] + s.db[736][9]);let eq9_e1080_d_b10: f64 = (s.db[204][10] + s.db[736][10]);let eq9_e1080_d_b11: f64 = (s.db[204][11] + s.db[736][11]);let eq9_e1082: f64 = (eq9_e1080 + s.v[432]);let eq9_e1082_d_n0: f64 = (eq9_e1080_d_n0 + s.dn[432][0]);let eq9_e1082_d_n1: f64 = (eq9_e1080_d_n1 + s.dn[432][1]);let eq9_e1082_d_n2: f64 = (eq9_e1080_d_n2 + s.dn[432][2]);let eq9_e1082_d_n3: f64 = (eq9_e1080_d_n3 + s.dn[432][3]);let eq9_e1082_d_n4: f64 = (eq9_e1080_d_n4 + s.dn[432][4]);let eq9_e1082_d_n5: f64 = (eq9_e1080_d_n5 + s.dn[432][5]);let eq9_e1082_d_n6: f64 = (eq9_e1080_d_n6 + s.dn[432][6]);let eq9_e1082_d_n7: f64 = (eq9_e1080_d_n7 + s.dn[432][7]);let eq9_e1082_d_n8: f64 = (eq9_e1080_d_n8 + s.dn[432][8]);let eq9_e1082_d_n9: f64 = (eq9_e1080_d_n9 + s.dn[432][9]);let eq9_e1082_d_n10: f64 = (eq9_e1080_d_n10 + s.dn[432][10]);let eq9_e1082_d_n11: f64 = (eq9_e1080_d_n11 + s.dn[432][11]);let eq9_e1082_d_n12: f64 = (eq9_e1080_d_n12 + s.dn[432][12]);let eq9_e1082_d_n13: f64 = (eq9_e1080_d_n13 + s.dn[432][13]);let eq9_e1082_d_n14: f64 = (eq9_e1080_d_n14 + s.dn[432][14]);let eq9_e1082_d_n15: f64 = (eq9_e1080_d_n15 + s.dn[432][15]);let eq9_e1082_d_n16: f64 = (eq9_e1080_d_n16 + s.dn[432][16]);let eq9_e1082_d_n17: f64 = (eq9_e1080_d_n17 + s.dn[432][17]);let eq9_e1082_d_b0: f64 = (eq9_e1080_d_b0 + s.db[432][0]);let eq9_e1082_d_b1: f64 = (eq9_e1080_d_b1 + s.db[432][1]);let eq9_e1082_d_b2: f64 = (eq9_e1080_d_b2 + s.db[432][2]);let eq9_e1082_d_b3: f64 = (eq9_e1080_d_b3 + s.db[432][3]);let eq9_e1082_d_b4: f64 = (eq9_e1080_d_b4 + s.db[432][4]);let eq9_e1082_d_b5: f64 = (eq9_e1080_d_b5 + s.db[432][5]);let eq9_e1082_d_b6: f64 = (eq9_e1080_d_b6 + s.db[432][6]);let eq9_e1082_d_b7: f64 = (eq9_e1080_d_b7 + s.db[432][7]);let eq9_e1082_d_b8: f64 = (eq9_e1080_d_b8 + s.db[432][8]);let eq9_e1082_d_b9: f64 = (eq9_e1080_d_b9 + s.db[432][9]);let eq9_e1082_d_b10: f64 = (eq9_e1080_d_b10 + s.db[432][10]);let eq9_e1082_d_b11: f64 = (eq9_e1080_d_b11 + s.db[432][11]);let eq9_e1083: f64 = (p[87] * eq9_e1082);let eq9_e1083_d_n0: f64 = (p[87] * eq9_e1082_d_n0);let eq9_e1083_d_n1: f64 = (p[87] * eq9_e1082_d_n1);let eq9_e1083_d_n2: f64 = (p[87] * eq9_e1082_d_n2);let eq9_e1083_d_n3: f64 = (p[87] * eq9_e1082_d_n3);let eq9_e1083_d_n4: f64 = (p[87] * eq9_e1082_d_n4);let eq9_e1083_d_n5: f64 = (p[87] * eq9_e1082_d_n5);let eq9_e1083_d_n6: f64 = (p[87] * eq9_e1082_d_n6);let eq9_e1083_d_n7: f64 = (p[87] * eq9_e1082_d_n7);let eq9_e1083_d_n8: f64 = (p[87] * eq9_e1082_d_n8);
        let eq9_e1083_d_n9: f64 = (p[87] * eq9_e1082_d_n9);let eq9_e1083_d_n10: f64 = (p[87] * eq9_e1082_d_n10);let eq9_e1083_d_n11: f64 = (p[87] * eq9_e1082_d_n11);let eq9_e1083_d_n12: f64 = (p[87] * eq9_e1082_d_n12);let eq9_e1083_d_n13: f64 = (p[87] * eq9_e1082_d_n13);let eq9_e1083_d_n14: f64 = (p[87] * eq9_e1082_d_n14);let eq9_e1083_d_n15: f64 = (p[87] * eq9_e1082_d_n15);let eq9_e1083_d_n16: f64 = (p[87] * eq9_e1082_d_n16);let eq9_e1083_d_n17: f64 = (p[87] * eq9_e1082_d_n17);let eq9_e1083_d_b0: f64 = (p[87] * eq9_e1082_d_b0);let eq9_e1083_d_b1: f64 = (p[87] * eq9_e1082_d_b1);let eq9_e1083_d_b2: f64 = (p[87] * eq9_e1082_d_b2);let eq9_e1083_d_b3: f64 = (p[87] * eq9_e1082_d_b3);let eq9_e1083_d_b4: f64 = (p[87] * eq9_e1082_d_b4);let eq9_e1083_d_b5: f64 = (p[87] * eq9_e1082_d_b5);let eq9_e1083_d_b6: f64 = (p[87] * eq9_e1082_d_b6);let eq9_e1083_d_b7: f64 = (p[87] * eq9_e1082_d_b7);let eq9_e1083_d_b8: f64 = (p[87] * eq9_e1082_d_b8);let eq9_e1083_d_b9: f64 = (p[87] * eq9_e1082_d_b9);let eq9_e1083_d_b10: f64 = (p[87] * eq9_e1082_d_b10);let eq9_e1083_d_b11: f64 = (p[87] * eq9_e1082_d_b11);let eq9_value: f64 = eq9_e1083;let eq9_node_derivatives: [f64; 18] = [eq9_e1083_d_n0, eq9_e1083_d_n1, eq9_e1083_d_n2, eq9_e1083_d_n3, eq9_e1083_d_n4, eq9_e1083_d_n5, eq9_e1083_d_n6, eq9_e1083_d_n7, eq9_e1083_d_n8, eq9_e1083_d_n9, eq9_e1083_d_n10, eq9_e1083_d_n11, eq9_e1083_d_n12, eq9_e1083_d_n13, eq9_e1083_d_n14, eq9_e1083_d_n15, eq9_e1083_d_n16, eq9_e1083_d_n17];let eq9_branch_derivatives: [f64; 12] = [eq9_e1083_d_b0, eq9_e1083_d_b1, eq9_e1083_d_b2, eq9_e1083_d_b3, eq9_e1083_d_b4, eq9_e1083_d_b5, eq9_e1083_d_b6, eq9_e1083_d_b7, eq9_e1083_d_b8, eq9_e1083_d_b9, eq9_e1083_d_b10, eq9_e1083_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
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
        let eq10_e1086: f64 = (p[87] * s.v[281]);let eq10_value: f64 = eq10_e1086;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq10_value),
            &s.dn[281],
            &s.db[281],
            (multiplicity) * (p[87]),
        );let eq11_e1089: f64 = (p[87] * s.v[737]);let eq11_value: f64 = eq11_e1089;
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq11_value),
            &s.dn[737],
            &s.db[737],
            (multiplicity) * (p[87]),
        );let eq12_e1092: f64 = (p[87] * s.v[862]);let eq12_value: f64 = eq12_e1092;
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq12_value),
            &s.dn[862],
            &s.db[862],
            (multiplicity) * (p[87]),
        );let eq13_e1095: f64 = (p[87] * s.v[861]);let eq13_value: f64 = eq13_e1095;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq13_value),
            &s.dn[861],
            &s.db[861],
            (multiplicity) * (p[87]),
        );let eq14_e1098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[66]);let eq14_e1099: f64 = (p[87] * eq14_e1098);let eq14_e1099_d_n0: f64 = (p[87] * (s.dn[66][0] * ddt_scale));let eq14_e1099_d_n1: f64 = (p[87] * (s.dn[66][1] * ddt_scale));let eq14_e1099_d_n2: f64 = (p[87] * (s.dn[66][2] * ddt_scale));let eq14_e1099_d_n3: f64 = (p[87] * (s.dn[66][3] * ddt_scale));let eq14_e1099_d_n4: f64 = (p[87] * (s.dn[66][4] * ddt_scale));let eq14_e1099_d_n5: f64 = (p[87] * (s.dn[66][5] * ddt_scale));let eq14_e1099_d_n6: f64 = (p[87] * (s.dn[66][6] * ddt_scale));let eq14_e1099_d_n7: f64 = (p[87] * (s.dn[66][7] * ddt_scale));let eq14_e1099_d_n8: f64 = (p[87] * (s.dn[66][8] * ddt_scale));let eq14_e1099_d_n9: f64 = (p[87] * (s.dn[66][9] * ddt_scale));let eq14_e1099_d_n10: f64 = (p[87] * (s.dn[66][10] * ddt_scale));let eq14_e1099_d_n11: f64 = (p[87] * (s.dn[66][11] * ddt_scale));let eq14_e1099_d_n12: f64 = (p[87] * (s.dn[66][12] * ddt_scale));let eq14_e1099_d_n13: f64 = (p[87] * (s.dn[66][13] * ddt_scale));let eq14_e1099_d_n14: f64 = (p[87] * (s.dn[66][14] * ddt_scale));let eq14_e1099_d_n15: f64 = (p[87] * (s.dn[66][15] * ddt_scale));let eq14_e1099_d_n16: f64 = (p[87] * (s.dn[66][16] * ddt_scale));let eq14_e1099_d_n17: f64 = (p[87] * (s.dn[66][17] * ddt_scale));let eq14_e1099_d_b0: f64 = (p[87] * (s.db[66][0] * ddt_scale));let eq14_e1099_d_b1: f64 = (p[87] * (s.db[66][1] * ddt_scale));let eq14_e1099_d_b2: f64 = (p[87] * (s.db[66][2] * ddt_scale));let eq14_e1099_d_b3: f64 = (p[87] * (s.db[66][3] * ddt_scale));let eq14_e1099_d_b4: f64 = (p[87] * (s.db[66][4] * ddt_scale));let eq14_e1099_d_b5: f64 = (p[87] * (s.db[66][5] * ddt_scale));let eq14_e1099_d_b6: f64 = (p[87] * (s.db[66][6] * ddt_scale));let eq14_e1099_d_b7: f64 = (p[87] * (s.db[66][7] * ddt_scale));let eq14_e1099_d_b8: f64 = (p[87] * (s.db[66][8] * ddt_scale));let eq14_e1099_d_b9: f64 = (p[87] * (s.db[66][9] * ddt_scale));let eq14_e1099_d_b10: f64 = (p[87] * (s.db[66][10] * ddt_scale));let eq14_e1099_d_b11: f64 = (p[87] * (s.db[66][11] * ddt_scale));let eq14_value: f64 = eq14_e1099;let eq14_node_derivatives: [f64; 18] = [eq14_e1099_d_n0, eq14_e1099_d_n1, eq14_e1099_d_n2, eq14_e1099_d_n3, eq14_e1099_d_n4, eq14_e1099_d_n5, eq14_e1099_d_n6, eq14_e1099_d_n7, eq14_e1099_d_n8, eq14_e1099_d_n9, eq14_e1099_d_n10, eq14_e1099_d_n11, eq14_e1099_d_n12, eq14_e1099_d_n13, eq14_e1099_d_n14, eq14_e1099_d_n15, eq14_e1099_d_n16, eq14_e1099_d_n17];let eq14_branch_derivatives: [f64; 12] = [eq14_e1099_d_b0, eq14_e1099_d_b1, eq14_e1099_d_b2, eq14_e1099_d_b3, eq14_e1099_d_b4, eq14_e1099_d_b5, eq14_e1099_d_b6, eq14_e1099_d_b7, eq14_e1099_d_b8, eq14_e1099_d_b9, eq14_e1099_d_b10, eq14_e1099_d_b11];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );let eq15_e1102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[65]);let eq15_e1103: f64 = (p[87] * eq15_e1102);let eq15_e1103_d_n0: f64 = (p[87] * (s.dn[65][0] * ddt_scale));let eq15_e1103_d_n1: f64 = (p[87] * (s.dn[65][1] * ddt_scale));let eq15_e1103_d_n2: f64 = (p[87] * (s.dn[65][2] * ddt_scale));let eq15_e1103_d_n3: f64 = (p[87] * (s.dn[65][3] * ddt_scale));let eq15_e1103_d_n4: f64 = (p[87] * (s.dn[65][4] * ddt_scale));let eq15_e1103_d_n5: f64 = (p[87] * (s.dn[65][5] * ddt_scale));let eq15_e1103_d_n6: f64 = (p[87] * (s.dn[65][6] * ddt_scale));let eq15_e1103_d_n7: f64 = (p[87] * (s.dn[65][7] * ddt_scale));let eq15_e1103_d_n8: f64 = (p[87] * (s.dn[65][8] * ddt_scale));let eq15_e1103_d_n9: f64 = (p[87] * (s.dn[65][9] * ddt_scale));let eq15_e1103_d_n10: f64 = (p[87] * (s.dn[65][10] * ddt_scale));let eq15_e1103_d_n11: f64 = (p[87] * (s.dn[65][11] * ddt_scale));let eq15_e1103_d_n12: f64 = (p[87] * (s.dn[65][12] * ddt_scale));let eq15_e1103_d_n13: f64 = (p[87] * (s.dn[65][13] * ddt_scale));let eq15_e1103_d_n14: f64 = (p[87] * (s.dn[65][14] * ddt_scale));let eq15_e1103_d_n15: f64 = (p[87] * (s.dn[65][15] * ddt_scale));let eq15_e1103_d_n16: f64 = (p[87] * (s.dn[65][16] * ddt_scale));let eq15_e1103_d_n17: f64 = (p[87] * (s.dn[65][17] * ddt_scale));let eq15_e1103_d_b0: f64 = (p[87] * (s.db[65][0] * ddt_scale));let eq15_e1103_d_b1: f64 = (p[87] * (s.db[65][1] * ddt_scale));let eq15_e1103_d_b2: f64 = (p[87] * (s.db[65][2] * ddt_scale));let eq15_e1103_d_b3: f64 = (p[87] * (s.db[65][3] * ddt_scale));let eq15_e1103_d_b4: f64 = (p[87] * (s.db[65][4] * ddt_scale));let eq15_e1103_d_b5: f64 = (p[87] * (s.db[65][5] * ddt_scale));let eq15_e1103_d_b6: f64 = (p[87] * (s.db[65][6] * ddt_scale));let eq15_e1103_d_b7: f64 = (p[87] * (s.db[65][7] * ddt_scale));let eq15_e1103_d_b8: f64 = (p[87] * (s.db[65][8] * ddt_scale));let eq15_e1103_d_b9: f64 = (p[87] * (s.db[65][9] * ddt_scale));let eq15_e1103_d_b10: f64 = (p[87] * (s.db[65][10] * ddt_scale));let eq15_e1103_d_b11: f64 = (p[87] * (s.db[65][11] * ddt_scale));let eq15_value: f64 = eq15_e1103;let eq15_node_derivatives: [f64; 18] = [eq15_e1103_d_n0, eq15_e1103_d_n1, eq15_e1103_d_n2, eq15_e1103_d_n3, eq15_e1103_d_n4, eq15_e1103_d_n5, eq15_e1103_d_n6, eq15_e1103_d_n7, eq15_e1103_d_n8, eq15_e1103_d_n9, eq15_e1103_d_n10, eq15_e1103_d_n11, eq15_e1103_d_n12, eq15_e1103_d_n13, eq15_e1103_d_n14, eq15_e1103_d_n15, eq15_e1103_d_n16, eq15_e1103_d_n17];let eq15_branch_derivatives: [f64; 12] = [eq15_e1103_d_b0, eq15_e1103_d_b1, eq15_e1103_d_b2, eq15_e1103_d_b3, eq15_e1103_d_b4, eq15_e1103_d_b5, eq15_e1103_d_b6, eq15_e1103_d_b7, eq15_e1103_d_b8, eq15_e1103_d_b9, eq15_e1103_d_b10, eq15_e1103_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1109, eq16_e1109_d_n0, eq16_e1109_d_n1, eq16_e1109_d_n2, eq16_e1109_d_n3, eq16_e1109_d_n4, eq16_e1109_d_n5, eq16_e1109_d_n6, eq16_e1109_d_n7, eq16_e1109_d_n8, eq16_e1109_d_n9, eq16_e1109_d_n10, eq16_e1109_d_n11, eq16_e1109_d_n12, eq16_e1109_d_n13, eq16_e1109_d_n14, eq16_e1109_d_n15, eq16_e1109_d_n16, eq16_e1109_d_n17, eq16_e1109_d_b0, eq16_e1109_d_b1, eq16_e1109_d_b2, eq16_e1109_d_b3, eq16_e1109_d_b4, eq16_e1109_d_b5, eq16_e1109_d_b6, eq16_e1109_d_b7, eq16_e1109_d_b8, eq16_e1109_d_b9, eq16_e1109_d_b10, eq16_e1109_d_b11,) = {
    if s.b[3405] {
        let eq16_e1107: f64 = (p[87] * s.v[870]);
        (eq16_e1107, (p[87] * s.dn[870][0]), (p[87] * s.dn[870][1]), (p[87] * s.dn[870][2]), (p[87] * s.dn[870][3]), (p[87] * s.dn[870][4]), (p[87] * s.dn[870][5]), (p[87] * s.dn[870][6]), (p[87] * s.dn[870][7]), (p[87] * s.dn[870][8]), (p[87] * s.dn[870][9]), (p[87] * s.dn[870][10]), (p[87] * s.dn[870][11]), (p[87] * s.dn[870][12]), (p[87] * s.dn[870][13]), (p[87] * s.dn[870][14]), (p[87] * s.dn[870][15]), (p[87] * s.dn[870][16]), (p[87] * s.dn[870][17]), (p[87] * s.db[870][0]), (p[87] * s.db[870][1]), (p[87] * s.db[870][2]), (p[87] * s.db[870][3]), (p[87] * s.db[870][4]), (p[87] * s.db[870][5]), (p[87] * s.db[870][6]), (p[87] * s.db[870][7]), (p[87] * s.db[870][8]), (p[87] * s.db[870][9]), (p[87] * s.db[870][10]), (p[87] * s.db[870][11]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1109;let eq16_node_derivatives: [f64; 18] = [eq16_e1109_d_n0, eq16_e1109_d_n1, eq16_e1109_d_n2, eq16_e1109_d_n3, eq16_e1109_d_n4, eq16_e1109_d_n5, eq16_e1109_d_n6, eq16_e1109_d_n7, eq16_e1109_d_n8, eq16_e1109_d_n9, eq16_e1109_d_n10, eq16_e1109_d_n11, eq16_e1109_d_n12, eq16_e1109_d_n13, eq16_e1109_d_n14, eq16_e1109_d_n15, eq16_e1109_d_n16, eq16_e1109_d_n17];let eq16_branch_derivatives: [f64; 12] = [eq16_e1109_d_b0, eq16_e1109_d_b1, eq16_e1109_d_b2, eq16_e1109_d_b3, eq16_e1109_d_b4, eq16_e1109_d_b5, eq16_e1109_d_b6, eq16_e1109_d_b7, eq16_e1109_d_b8, eq16_e1109_d_b9, eq16_e1109_d_b10, eq16_e1109_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
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
        let (eq17_e1115, eq17_e1115_d_n0, eq17_e1115_d_n1, eq17_e1115_d_n2, eq17_e1115_d_n3, eq17_e1115_d_n4, eq17_e1115_d_n5, eq17_e1115_d_n6, eq17_e1115_d_n7, eq17_e1115_d_n8, eq17_e1115_d_n9, eq17_e1115_d_n10, eq17_e1115_d_n11, eq17_e1115_d_n12, eq17_e1115_d_n13, eq17_e1115_d_n14, eq17_e1115_d_n15, eq17_e1115_d_n16, eq17_e1115_d_n17, eq17_e1115_d_b0, eq17_e1115_d_b1, eq17_e1115_d_b2, eq17_e1115_d_b3, eq17_e1115_d_b4, eq17_e1115_d_b5, eq17_e1115_d_b6, eq17_e1115_d_b7, eq17_e1115_d_b8, eq17_e1115_d_b9, eq17_e1115_d_b10, eq17_e1115_d_b11,) = {
    if s.b[3405] {
        let eq17_e1113: f64 = (p[87] * s.v[869]);
        (eq17_e1113, (p[87] * s.dn[869][0]), (p[87] * s.dn[869][1]), (p[87] * s.dn[869][2]), (p[87] * s.dn[869][3]), (p[87] * s.dn[869][4]), (p[87] * s.dn[869][5]), (p[87] * s.dn[869][6]), (p[87] * s.dn[869][7]), (p[87] * s.dn[869][8]), (p[87] * s.dn[869][9]), (p[87] * s.dn[869][10]), (p[87] * s.dn[869][11]), (p[87] * s.dn[869][12]), (p[87] * s.dn[869][13]), (p[87] * s.dn[869][14]), (p[87] * s.dn[869][15]), (p[87] * s.dn[869][16]), (p[87] * s.dn[869][17]), (p[87] * s.db[869][0]), (p[87] * s.db[869][1]), (p[87] * s.db[869][2]), (p[87] * s.db[869][3]), (p[87] * s.db[869][4]), (p[87] * s.db[869][5]), (p[87] * s.db[869][6]), (p[87] * s.db[869][7]), (p[87] * s.db[869][8]), (p[87] * s.db[869][9]), (p[87] * s.db[869][10]), (p[87] * s.db[869][11]),)
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
        let eq18_e1119: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[68]);let eq18_e1120: f64 = (p[87] * eq18_e1119);let eq18_e1120_d_n0: f64 = (p[87] * (s.dn[68][0] * ddt_scale));let eq18_e1120_d_n1: f64 = (p[87] * (s.dn[68][1] * ddt_scale));let eq18_e1120_d_n2: f64 = (p[87] * (s.dn[68][2] * ddt_scale));let eq18_e1120_d_n3: f64 = (p[87] * (s.dn[68][3] * ddt_scale));let eq18_e1120_d_n4: f64 = (p[87] * (s.dn[68][4] * ddt_scale));let eq18_e1120_d_n5: f64 = (p[87] * (s.dn[68][5] * ddt_scale));let eq18_e1120_d_n6: f64 = (p[87] * (s.dn[68][6] * ddt_scale));let eq18_e1120_d_n7: f64 = (p[87] * (s.dn[68][7] * ddt_scale));let eq18_e1120_d_n8: f64 = (p[87] * (s.dn[68][8] * ddt_scale));let eq18_e1120_d_n9: f64 = (p[87] * (s.dn[68][9] * ddt_scale));let eq18_e1120_d_n10: f64 = (p[87] * (s.dn[68][10] * ddt_scale));let eq18_e1120_d_n11: f64 = (p[87] * (s.dn[68][11] * ddt_scale));let eq18_e1120_d_n12: f64 = (p[87] * (s.dn[68][12] * ddt_scale));let eq18_e1120_d_n13: f64 = (p[87] * (s.dn[68][13] * ddt_scale));let eq18_e1120_d_n14: f64 = (p[87] * (s.dn[68][14] * ddt_scale));let eq18_e1120_d_n15: f64 = (p[87] * (s.dn[68][15] * ddt_scale));let eq18_e1120_d_n16: f64 = (p[87] * (s.dn[68][16] * ddt_scale));let eq18_e1120_d_n17: f64 = (p[87] * (s.dn[68][17] * ddt_scale));let eq18_e1120_d_b0: f64 = (p[87] * (s.db[68][0] * ddt_scale));let eq18_e1120_d_b1: f64 = (p[87] * (s.db[68][1] * ddt_scale));let eq18_e1120_d_b2: f64 = (p[87] * (s.db[68][2] * ddt_scale));let eq18_e1120_d_b3: f64 = (p[87] * (s.db[68][3] * ddt_scale));let eq18_e1120_d_b4: f64 = (p[87] * (s.db[68][4] * ddt_scale));let eq18_e1120_d_b5: f64 = (p[87] * (s.db[68][5] * ddt_scale));let eq18_e1120_d_b6: f64 = (p[87] * (s.db[68][6] * ddt_scale));let eq18_e1120_d_b7: f64 = (p[87] * (s.db[68][7] * ddt_scale));let eq18_e1120_d_b8: f64 = (p[87] * (s.db[68][8] * ddt_scale));let eq18_e1120_d_b9: f64 = (p[87] * (s.db[68][9] * ddt_scale));let eq18_e1120_d_b10: f64 = (p[87] * (s.db[68][10] * ddt_scale));let eq18_e1120_d_b11: f64 = (p[87] * (s.db[68][11] * ddt_scale));
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
        let eq19_e1126: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[67]);let eq19_e1127: f64 = (p[87] * eq19_e1126);let eq19_e1127_d_n0: f64 = (p[87] * (s.dn[67][0] * ddt_scale));let eq19_e1127_d_n1: f64 = (p[87] * (s.dn[67][1] * ddt_scale));let eq19_e1127_d_n2: f64 = (p[87] * (s.dn[67][2] * ddt_scale));let eq19_e1127_d_n3: f64 = (p[87] * (s.dn[67][3] * ddt_scale));let eq19_e1127_d_n4: f64 = (p[87] * (s.dn[67][4] * ddt_scale));let eq19_e1127_d_n5: f64 = (p[87] * (s.dn[67][5] * ddt_scale));let eq19_e1127_d_n6: f64 = (p[87] * (s.dn[67][6] * ddt_scale));let eq19_e1127_d_n7: f64 = (p[87] * (s.dn[67][7] * ddt_scale));let eq19_e1127_d_n8: f64 = (p[87] * (s.dn[67][8] * ddt_scale));let eq19_e1127_d_n9: f64 = (p[87] * (s.dn[67][9] * ddt_scale));let eq19_e1127_d_n10: f64 = (p[87] * (s.dn[67][10] * ddt_scale));let eq19_e1127_d_n11: f64 = (p[87] * (s.dn[67][11] * ddt_scale));let eq19_e1127_d_n12: f64 = (p[87] * (s.dn[67][12] * ddt_scale));let eq19_e1127_d_n13: f64 = (p[87] * (s.dn[67][13] * ddt_scale));let eq19_e1127_d_n14: f64 = (p[87] * (s.dn[67][14] * ddt_scale));let eq19_e1127_d_n15: f64 = (p[87] * (s.dn[67][15] * ddt_scale));let eq19_e1127_d_n16: f64 = (p[87] * (s.dn[67][16] * ddt_scale));let eq19_e1127_d_n17: f64 = (p[87] * (s.dn[67][17] * ddt_scale));let eq19_e1127_d_b0: f64 = (p[87] * (s.db[67][0] * ddt_scale));let eq19_e1127_d_b1: f64 = (p[87] * (s.db[67][1] * ddt_scale));let eq19_e1127_d_b2: f64 = (p[87] * (s.db[67][2] * ddt_scale));let eq19_e1127_d_b3: f64 = (p[87] * (s.db[67][3] * ddt_scale));let eq19_e1127_d_b4: f64 = (p[87] * (s.db[67][4] * ddt_scale));let eq19_e1127_d_b5: f64 = (p[87] * (s.db[67][5] * ddt_scale));let eq19_e1127_d_b6: f64 = (p[87] * (s.db[67][6] * ddt_scale));let eq19_e1127_d_b7: f64 = (p[87] * (s.db[67][7] * ddt_scale));let eq19_e1127_d_b8: f64 = (p[87] * (s.db[67][8] * ddt_scale));let eq19_e1127_d_b9: f64 = (p[87] * (s.db[67][9] * ddt_scale));let eq19_e1127_d_b10: f64 = (p[87] * (s.db[67][10] * ddt_scale));let eq19_e1127_d_b11: f64 = (p[87] * (s.db[67][11] * ddt_scale));
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
        let eq20_e1133: f64 = (p[87] * s.v[200]);
        (eq20_e1133, (p[87] * s.dn[200][0]), (p[87] * s.dn[200][1]), (p[87] * s.dn[200][2]), (p[87] * s.dn[200][3]), (p[87] * s.dn[200][4]), (p[87] * s.dn[200][5]), (p[87] * s.dn[200][6]), (p[87] * s.dn[200][7]), (p[87] * s.dn[200][8]), (p[87] * s.dn[200][9]), (p[87] * s.dn[200][10]), (p[87] * s.dn[200][11]), (p[87] * s.dn[200][12]), (p[87] * s.dn[200][13]), (p[87] * s.dn[200][14]), (p[87] * s.dn[200][15]), (p[87] * s.dn[200][16]), (p[87] * s.dn[200][17]), (p[87] * s.db[200][0]), (p[87] * s.db[200][1]), (p[87] * s.db[200][2]), (p[87] * s.db[200][3]), (p[87] * s.db[200][4]), (p[87] * s.db[200][5]), (p[87] * s.db[200][6]), (p[87] * s.db[200][7]), (p[87] * s.db[200][8]), (p[87] * s.db[200][9]), (p[87] * s.db[200][10]), (p[87] * s.db[200][11]),)
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
        let eq21_e1139: f64 = (p[87] * s.v[201]);
        (eq21_e1139, (p[87] * s.dn[201][0]), (p[87] * s.dn[201][1]), (p[87] * s.dn[201][2]), (p[87] * s.dn[201][3]), (p[87] * s.dn[201][4]), (p[87] * s.dn[201][5]), (p[87] * s.dn[201][6]), (p[87] * s.dn[201][7]), (p[87] * s.dn[201][8]), (p[87] * s.dn[201][9]), (p[87] * s.dn[201][10]), (p[87] * s.dn[201][11]), (p[87] * s.dn[201][12]), (p[87] * s.dn[201][13]), (p[87] * s.dn[201][14]), (p[87] * s.dn[201][15]), (p[87] * s.dn[201][16]), (p[87] * s.dn[201][17]), (p[87] * s.db[201][0]), (p[87] * s.db[201][1]), (p[87] * s.db[201][2]), (p[87] * s.db[201][3]), (p[87] * s.db[201][4]), (p[87] * s.db[201][5]), (p[87] * s.db[201][6]), (p[87] * s.db[201][7]), (p[87] * s.db[201][8]), (p[87] * s.db[201][9]), (p[87] * s.db[201][10]), (p[87] * s.db[201][11]),)
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
        let eq22_e1145: f64 = (p[87] * s.v[202]);
        (eq22_e1145, (p[87] * s.dn[202][0]), (p[87] * s.dn[202][1]), (p[87] * s.dn[202][2]), (p[87] * s.dn[202][3]), (p[87] * s.dn[202][4]), (p[87] * s.dn[202][5]), (p[87] * s.dn[202][6]), (p[87] * s.dn[202][7]), (p[87] * s.dn[202][8]), (p[87] * s.dn[202][9]), (p[87] * s.dn[202][10]), (p[87] * s.dn[202][11]), (p[87] * s.dn[202][12]), (p[87] * s.dn[202][13]), (p[87] * s.dn[202][14]), (p[87] * s.dn[202][15]), (p[87] * s.dn[202][16]), (p[87] * s.dn[202][17]), (p[87] * s.db[202][0]), (p[87] * s.db[202][1]), (p[87] * s.db[202][2]), (p[87] * s.db[202][3]), (p[87] * s.db[202][4]), (p[87] * s.db[202][5]), (p[87] * s.db[202][6]), (p[87] * s.db[202][7]), (p[87] * s.db[202][8]), (p[87] * s.db[202][9]), (p[87] * s.db[202][10]), (p[87] * s.db[202][11]),)
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
        );let eq27_e1173: f64 = (s.v[18] + s.v[753]);let eq27_e1173_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq27_e1173_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq27_e1173_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq27_e1173_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq27_e1173_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq27_e1173_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq27_e1173_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq27_e1173_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq27_e1173_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq27_e1173_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq27_e1173_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq27_e1173_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq27_e1173_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq27_e1173_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq27_e1173_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq27_e1173_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq27_e1173_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq27_e1173_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq27_e1173_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq27_e1173_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq27_e1173_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq27_e1173_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq27_e1173_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq27_e1173_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq27_e1173_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq27_e1173_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq27_e1173_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq27_e1173_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq27_e1173_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq27_e1173_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq27_e1174: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq27_e1173);let eq27_e1175: f64 = (p[87] * eq27_e1174);let eq27_e1175_d_n0: f64 = (p[87] * (eq27_e1173_d_n0 * ddt_scale));let eq27_e1175_d_n1: f64 = (p[87] * (eq27_e1173_d_n1 * ddt_scale));let eq27_e1175_d_n2: f64 = (p[87] * (eq27_e1173_d_n2 * ddt_scale));let eq27_e1175_d_n3: f64 = (p[87] * (eq27_e1173_d_n3 * ddt_scale));let eq27_e1175_d_n4: f64 = (p[87] * (eq27_e1173_d_n4 * ddt_scale));let eq27_e1175_d_n5: f64 = (p[87] * (eq27_e1173_d_n5 * ddt_scale));let eq27_e1175_d_n6: f64 = (p[87] * (eq27_e1173_d_n6 * ddt_scale));let eq27_e1175_d_n7: f64 = (p[87] * (eq27_e1173_d_n7 * ddt_scale));let eq27_e1175_d_n8: f64 = (p[87] * (eq27_e1173_d_n8 * ddt_scale));let eq27_e1175_d_n9: f64 = (p[87] * (eq27_e1173_d_n9 * ddt_scale));let eq27_e1175_d_n10: f64 = (p[87] * (eq27_e1173_d_n10 * ddt_scale));let eq27_e1175_d_n11: f64 = (p[87] * (eq27_e1173_d_n11 * ddt_scale));let eq27_e1175_d_n12: f64 = (p[87] * (eq27_e1173_d_n12 * ddt_scale));let eq27_e1175_d_n13: f64 = (p[87] * (eq27_e1173_d_n13 * ddt_scale));let eq27_e1175_d_n14: f64 = (p[87] * (eq27_e1173_d_n14 * ddt_scale));let eq27_e1175_d_n15: f64 = (p[87] * (eq27_e1173_d_n15 * ddt_scale));let eq27_e1175_d_n16: f64 = (p[87] * (eq27_e1173_d_n16 * ddt_scale));let eq27_e1175_d_n17: f64 = (p[87] * (eq27_e1173_d_n17 * ddt_scale));let eq27_e1175_d_b0: f64 = (p[87] * (eq27_e1173_d_b0 * ddt_scale));let eq27_e1175_d_b1: f64 = (p[87] * (eq27_e1173_d_b1 * ddt_scale));let eq27_e1175_d_b2: f64 = (p[87] * (eq27_e1173_d_b2 * ddt_scale));let eq27_e1175_d_b3: f64 = (p[87] * (eq27_e1173_d_b3 * ddt_scale));let eq27_e1175_d_b4: f64 = (p[87] * (eq27_e1173_d_b4 * ddt_scale));let eq27_e1175_d_b5: f64 = (p[87] * (eq27_e1173_d_b5 * ddt_scale));let eq27_e1175_d_b6: f64 = (p[87] * (eq27_e1173_d_b6 * ddt_scale));let eq27_e1175_d_b7: f64 = (p[87] * (eq27_e1173_d_b7 * ddt_scale));let eq27_e1175_d_b8: f64 = (p[87] * (eq27_e1173_d_b8 * ddt_scale));let eq27_e1175_d_b9: f64 = (p[87] * (eq27_e1173_d_b9 * ddt_scale));let eq27_e1175_d_b10: f64 = (p[87] * (eq27_e1173_d_b10 * ddt_scale));
        let eq27_e1175_d_b11: f64 = (p[87] * (eq27_e1173_d_b11 * ddt_scale));let eq27_value: f64 = eq27_e1175;let eq27_node_derivatives: [f64; 18] = [eq27_e1175_d_n0, eq27_e1175_d_n1, eq27_e1175_d_n2, eq27_e1175_d_n3, eq27_e1175_d_n4, eq27_e1175_d_n5, eq27_e1175_d_n6, eq27_e1175_d_n7, eq27_e1175_d_n8, eq27_e1175_d_n9, eq27_e1175_d_n10, eq27_e1175_d_n11, eq27_e1175_d_n12, eq27_e1175_d_n13, eq27_e1175_d_n14, eq27_e1175_d_n15, eq27_e1175_d_n16, eq27_e1175_d_n17];let eq27_branch_derivatives: [f64; 12] = [eq27_e1175_d_b0, eq27_e1175_d_b1, eq27_e1175_d_b2, eq27_e1175_d_b3, eq27_e1175_d_b4, eq27_e1175_d_b5, eq27_e1175_d_b6, eq27_e1175_d_b7, eq27_e1175_d_b8, eq27_e1175_d_b9, eq27_e1175_d_b10, eq27_e1175_d_b11];
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
        let eq28_e1179: f64 = (s.v[19] + s.v[751]);let eq28_e1179_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);let eq28_e1179_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);let eq28_e1179_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);let eq28_e1179_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);let eq28_e1179_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);let eq28_e1179_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);let eq28_e1179_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);let eq28_e1179_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);let eq28_e1179_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);let eq28_e1179_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);let eq28_e1179_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);let eq28_e1179_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);let eq28_e1179_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);let eq28_e1179_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);let eq28_e1179_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);let eq28_e1179_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);let eq28_e1179_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);let eq28_e1179_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);let eq28_e1179_d_b0: f64 = (s.db[19][0] + s.db[751][0]);let eq28_e1179_d_b1: f64 = (s.db[19][1] + s.db[751][1]);let eq28_e1179_d_b2: f64 = (s.db[19][2] + s.db[751][2]);let eq28_e1179_d_b3: f64 = (s.db[19][3] + s.db[751][3]);let eq28_e1179_d_b4: f64 = (s.db[19][4] + s.db[751][4]);let eq28_e1179_d_b5: f64 = (s.db[19][5] + s.db[751][5]);let eq28_e1179_d_b6: f64 = (s.db[19][6] + s.db[751][6]);let eq28_e1179_d_b7: f64 = (s.db[19][7] + s.db[751][7]);let eq28_e1179_d_b8: f64 = (s.db[19][8] + s.db[751][8]);let eq28_e1179_d_b9: f64 = (s.db[19][9] + s.db[751][9]);let eq28_e1179_d_b10: f64 = (s.db[19][10] + s.db[751][10]);let eq28_e1179_d_b11: f64 = (s.db[19][11] + s.db[751][11]);let eq28_e1180: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq28_e1179);let eq28_e1181: f64 = (p[87] * eq28_e1180);let eq28_e1181_d_n0: f64 = (p[87] * (eq28_e1179_d_n0 * ddt_scale));let eq28_e1181_d_n1: f64 = (p[87] * (eq28_e1179_d_n1 * ddt_scale));let eq28_e1181_d_n2: f64 = (p[87] * (eq28_e1179_d_n2 * ddt_scale));let eq28_e1181_d_n3: f64 = (p[87] * (eq28_e1179_d_n3 * ddt_scale));let eq28_e1181_d_n4: f64 = (p[87] * (eq28_e1179_d_n4 * ddt_scale));let eq28_e1181_d_n5: f64 = (p[87] * (eq28_e1179_d_n5 * ddt_scale));let eq28_e1181_d_n6: f64 = (p[87] * (eq28_e1179_d_n6 * ddt_scale));let eq28_e1181_d_n7: f64 = (p[87] * (eq28_e1179_d_n7 * ddt_scale));let eq28_e1181_d_n8: f64 = (p[87] * (eq28_e1179_d_n8 * ddt_scale));let eq28_e1181_d_n9: f64 = (p[87] * (eq28_e1179_d_n9 * ddt_scale));let eq28_e1181_d_n10: f64 = (p[87] * (eq28_e1179_d_n10 * ddt_scale));let eq28_e1181_d_n11: f64 = (p[87] * (eq28_e1179_d_n11 * ddt_scale));let eq28_e1181_d_n12: f64 = (p[87] * (eq28_e1179_d_n12 * ddt_scale));let eq28_e1181_d_n13: f64 = (p[87] * (eq28_e1179_d_n13 * ddt_scale));let eq28_e1181_d_n14: f64 = (p[87] * (eq28_e1179_d_n14 * ddt_scale));let eq28_e1181_d_n15: f64 = (p[87] * (eq28_e1179_d_n15 * ddt_scale));let eq28_e1181_d_n16: f64 = (p[87] * (eq28_e1179_d_n16 * ddt_scale));let eq28_e1181_d_n17: f64 = (p[87] * (eq28_e1179_d_n17 * ddt_scale));let eq28_e1181_d_b0: f64 = (p[87] * (eq28_e1179_d_b0 * ddt_scale));let eq28_e1181_d_b1: f64 = (p[87] * (eq28_e1179_d_b1 * ddt_scale));let eq28_e1181_d_b2: f64 = (p[87] * (eq28_e1179_d_b2 * ddt_scale));let eq28_e1181_d_b3: f64 = (p[87] * (eq28_e1179_d_b3 * ddt_scale));let eq28_e1181_d_b4: f64 = (p[87] * (eq28_e1179_d_b4 * ddt_scale));let eq28_e1181_d_b5: f64 = (p[87] * (eq28_e1179_d_b5 * ddt_scale));let eq28_e1181_d_b6: f64 = (p[87] * (eq28_e1179_d_b6 * ddt_scale));let eq28_e1181_d_b7: f64 = (p[87] * (eq28_e1179_d_b7 * ddt_scale));let eq28_e1181_d_b8: f64 = (p[87] * (eq28_e1179_d_b8 * ddt_scale));let eq28_e1181_d_b9: f64 = (p[87] * (eq28_e1179_d_b9 * ddt_scale));let eq28_e1181_d_b10: f64 = (p[87] * (eq28_e1179_d_b10 * ddt_scale));
        let eq28_e1181_d_b11: f64 = (p[87] * (eq28_e1179_d_b11 * ddt_scale));let eq28_value: f64 = eq28_e1181;let eq28_node_derivatives: [f64; 18] = [eq28_e1181_d_n0, eq28_e1181_d_n1, eq28_e1181_d_n2, eq28_e1181_d_n3, eq28_e1181_d_n4, eq28_e1181_d_n5, eq28_e1181_d_n6, eq28_e1181_d_n7, eq28_e1181_d_n8, eq28_e1181_d_n9, eq28_e1181_d_n10, eq28_e1181_d_n11, eq28_e1181_d_n12, eq28_e1181_d_n13, eq28_e1181_d_n14, eq28_e1181_d_n15, eq28_e1181_d_n16, eq28_e1181_d_n17];let eq28_branch_derivatives: [f64; 12] = [eq28_e1181_d_b0, eq28_e1181_d_b1, eq28_e1181_d_b2, eq28_e1181_d_b3, eq28_e1181_d_b4, eq28_e1181_d_b5, eq28_e1181_d_b6, eq28_e1181_d_b7, eq28_e1181_d_b8, eq28_e1181_d_b9, eq28_e1181_d_b10, eq28_e1181_d_b11];
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
        let eq29_e1189_d_n6: f64 = (s.dn[20][6] - eq29_e1188_d_n6);let eq29_e1189_d_n7: f64 = (s.dn[20][7] - eq29_e1188_d_n7);let eq29_e1189_d_n8: f64 = (s.dn[20][8] - eq29_e1188_d_n8);let eq29_e1189_d_n9: f64 = (s.dn[20][9] - eq29_e1188_d_n9);let eq29_e1189_d_n10: f64 = (s.dn[20][10] - eq29_e1188_d_n10);let eq29_e1189_d_n11: f64 = (s.dn[20][11] - eq29_e1188_d_n11);let eq29_e1189_d_n12: f64 = (s.dn[20][12] - eq29_e1188_d_n12);let eq29_e1189_d_n13: f64 = (s.dn[20][13] - eq29_e1188_d_n13);let eq29_e1189_d_n14: f64 = (s.dn[20][14] - eq29_e1188_d_n14);let eq29_e1189_d_n15: f64 = (s.dn[20][15] - eq29_e1188_d_n15);let eq29_e1189_d_n16: f64 = (s.dn[20][16] - eq29_e1188_d_n16);let eq29_e1189_d_n17: f64 = (s.dn[20][17] - eq29_e1188_d_n17);let eq29_e1189_d_b0: f64 = (s.db[20][0] - eq29_e1188_d_b0);let eq29_e1189_d_b1: f64 = (s.db[20][1] - eq29_e1188_d_b1);let eq29_e1189_d_b2: f64 = (s.db[20][2] - eq29_e1188_d_b2);let eq29_e1189_d_b3: f64 = (s.db[20][3] - eq29_e1188_d_b3);let eq29_e1189_d_b4: f64 = (s.db[20][4] - eq29_e1188_d_b4);let eq29_e1189_d_b5: f64 = (s.db[20][5] - eq29_e1188_d_b5);let eq29_e1189_d_b6: f64 = (s.db[20][6] - eq29_e1188_d_b6);let eq29_e1189_d_b7: f64 = (s.db[20][7] - eq29_e1188_d_b7);let eq29_e1189_d_b8: f64 = (s.db[20][8] - eq29_e1188_d_b8);let eq29_e1189_d_b9: f64 = (s.db[20][9] - eq29_e1188_d_b9);let eq29_e1189_d_b10: f64 = (s.db[20][10] - eq29_e1188_d_b10);let eq29_e1189_d_b11: f64 = (s.db[20][11] - eq29_e1188_d_b11);let eq29_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq29_e1189);let eq29_e1191: f64 = (p[87] * eq29_e1190);let eq29_e1191_d_n0: f64 = (p[87] * (eq29_e1189_d_n0 * ddt_scale));let eq29_e1191_d_n1: f64 = (p[87] * (eq29_e1189_d_n1 * ddt_scale));let eq29_e1191_d_n2: f64 = (p[87] * (eq29_e1189_d_n2 * ddt_scale));let eq29_e1191_d_n3: f64 = (p[87] * (eq29_e1189_d_n3 * ddt_scale));let eq29_e1191_d_n4: f64 = (p[87] * (eq29_e1189_d_n4 * ddt_scale));let eq29_e1191_d_n5: f64 = (p[87] * (eq29_e1189_d_n5 * ddt_scale));let eq29_e1191_d_n6: f64 = (p[87] * (eq29_e1189_d_n6 * ddt_scale));let eq29_e1191_d_n7: f64 = (p[87] * (eq29_e1189_d_n7 * ddt_scale));let eq29_e1191_d_n8: f64 = (p[87] * (eq29_e1189_d_n8 * ddt_scale));let eq29_e1191_d_n9: f64 = (p[87] * (eq29_e1189_d_n9 * ddt_scale));let eq29_e1191_d_n10: f64 = (p[87] * (eq29_e1189_d_n10 * ddt_scale));let eq29_e1191_d_n11: f64 = (p[87] * (eq29_e1189_d_n11 * ddt_scale));let eq29_e1191_d_n12: f64 = (p[87] * (eq29_e1189_d_n12 * ddt_scale));let eq29_e1191_d_n13: f64 = (p[87] * (eq29_e1189_d_n13 * ddt_scale));let eq29_e1191_d_n14: f64 = (p[87] * (eq29_e1189_d_n14 * ddt_scale));let eq29_e1191_d_n15: f64 = (p[87] * (eq29_e1189_d_n15 * ddt_scale));let eq29_e1191_d_n16: f64 = (p[87] * (eq29_e1189_d_n16 * ddt_scale));let eq29_e1191_d_n17: f64 = (p[87] * (eq29_e1189_d_n17 * ddt_scale));let eq29_e1191_d_b0: f64 = (p[87] * (eq29_e1189_d_b0 * ddt_scale));let eq29_e1191_d_b1: f64 = (p[87] * (eq29_e1189_d_b1 * ddt_scale));let eq29_e1191_d_b2: f64 = (p[87] * (eq29_e1189_d_b2 * ddt_scale));let eq29_e1191_d_b3: f64 = (p[87] * (eq29_e1189_d_b3 * ddt_scale));let eq29_e1191_d_b4: f64 = (p[87] * (eq29_e1189_d_b4 * ddt_scale));let eq29_e1191_d_b5: f64 = (p[87] * (eq29_e1189_d_b5 * ddt_scale));let eq29_e1191_d_b6: f64 = (p[87] * (eq29_e1189_d_b6 * ddt_scale));let eq29_e1191_d_b7: f64 = (p[87] * (eq29_e1189_d_b7 * ddt_scale));let eq29_e1191_d_b8: f64 = (p[87] * (eq29_e1189_d_b8 * ddt_scale));let eq29_e1191_d_b9: f64 = (p[87] * (eq29_e1189_d_b9 * ddt_scale));let eq29_e1191_d_b10: f64 = (p[87] * (eq29_e1189_d_b10 * ddt_scale));let eq29_e1191_d_b11: f64 = (p[87] * (eq29_e1189_d_b11 * ddt_scale));let eq29_value: f64 = eq29_e1191;
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
        let eq30_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[743]);let eq30_e1195: f64 = (p[87] * eq30_e1194);let eq30_e1195_d_n0: f64 = (p[87] * (s.dn[743][0] * ddt_scale));let eq30_e1195_d_n1: f64 = (p[87] * (s.dn[743][1] * ddt_scale));let eq30_e1195_d_n2: f64 = (p[87] * (s.dn[743][2] * ddt_scale));let eq30_e1195_d_n3: f64 = (p[87] * (s.dn[743][3] * ddt_scale));let eq30_e1195_d_n4: f64 = (p[87] * (s.dn[743][4] * ddt_scale));let eq30_e1195_d_n5: f64 = (p[87] * (s.dn[743][5] * ddt_scale));let eq30_e1195_d_n6: f64 = (p[87] * (s.dn[743][6] * ddt_scale));let eq30_e1195_d_n7: f64 = (p[87] * (s.dn[743][7] * ddt_scale));let eq30_e1195_d_n8: f64 = (p[87] * (s.dn[743][8] * ddt_scale));let eq30_e1195_d_n9: f64 = (p[87] * (s.dn[743][9] * ddt_scale));let eq30_e1195_d_n10: f64 = (p[87] * (s.dn[743][10] * ddt_scale));let eq30_e1195_d_n11: f64 = (p[87] * (s.dn[743][11] * ddt_scale));let eq30_e1195_d_n12: f64 = (p[87] * (s.dn[743][12] * ddt_scale));let eq30_e1195_d_n13: f64 = (p[87] * (s.dn[743][13] * ddt_scale));let eq30_e1195_d_n14: f64 = (p[87] * (s.dn[743][14] * ddt_scale));let eq30_e1195_d_n15: f64 = (p[87] * (s.dn[743][15] * ddt_scale));let eq30_e1195_d_n16: f64 = (p[87] * (s.dn[743][16] * ddt_scale));let eq30_e1195_d_n17: f64 = (p[87] * (s.dn[743][17] * ddt_scale));let eq30_e1195_d_b0: f64 = (p[87] * (s.db[743][0] * ddt_scale));let eq30_e1195_d_b1: f64 = (p[87] * (s.db[743][1] * ddt_scale));let eq30_e1195_d_b2: f64 = (p[87] * (s.db[743][2] * ddt_scale));let eq30_e1195_d_b3: f64 = (p[87] * (s.db[743][3] * ddt_scale));let eq30_e1195_d_b4: f64 = (p[87] * (s.db[743][4] * ddt_scale));let eq30_e1195_d_b5: f64 = (p[87] * (s.db[743][5] * ddt_scale));let eq30_e1195_d_b6: f64 = (p[87] * (s.db[743][6] * ddt_scale));let eq30_e1195_d_b7: f64 = (p[87] * (s.db[743][7] * ddt_scale));let eq30_e1195_d_b8: f64 = (p[87] * (s.db[743][8] * ddt_scale));let eq30_e1195_d_b9: f64 = (p[87] * (s.db[743][9] * ddt_scale));let eq30_e1195_d_b10: f64 = (p[87] * (s.db[743][10] * ddt_scale));let eq30_e1195_d_b11: f64 = (p[87] * (s.db[743][11] * ddt_scale));let eq30_value: f64 = eq30_e1195;let eq30_node_derivatives: [f64; 18] = [eq30_e1195_d_n0, eq30_e1195_d_n1, eq30_e1195_d_n2, eq30_e1195_d_n3, eq30_e1195_d_n4, eq30_e1195_d_n5, eq30_e1195_d_n6, eq30_e1195_d_n7, eq30_e1195_d_n8, eq30_e1195_d_n9, eq30_e1195_d_n10, eq30_e1195_d_n11, eq30_e1195_d_n12, eq30_e1195_d_n13, eq30_e1195_d_n14, eq30_e1195_d_n15, eq30_e1195_d_n16, eq30_e1195_d_n17];let eq30_branch_derivatives: [f64; 12] = [eq30_e1195_d_b0, eq30_e1195_d_b1, eq30_e1195_d_b2, eq30_e1195_d_b3, eq30_e1195_d_b4, eq30_e1195_d_b5, eq30_e1195_d_b6, eq30_e1195_d_b7, eq30_e1195_d_b8, eq30_e1195_d_b9, eq30_e1195_d_b10, eq30_e1195_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(2),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );let eq31_e1198: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[742]);let eq31_e1199: f64 = (p[87] * eq31_e1198);let eq31_e1199_d_n0: f64 = (p[87] * (s.dn[742][0] * ddt_scale));let eq31_e1199_d_n1: f64 = (p[87] * (s.dn[742][1] * ddt_scale));let eq31_e1199_d_n2: f64 = (p[87] * (s.dn[742][2] * ddt_scale));let eq31_e1199_d_n3: f64 = (p[87] * (s.dn[742][3] * ddt_scale));let eq31_e1199_d_n4: f64 = (p[87] * (s.dn[742][4] * ddt_scale));let eq31_e1199_d_n5: f64 = (p[87] * (s.dn[742][5] * ddt_scale));let eq31_e1199_d_n6: f64 = (p[87] * (s.dn[742][6] * ddt_scale));let eq31_e1199_d_n7: f64 = (p[87] * (s.dn[742][7] * ddt_scale));let eq31_e1199_d_n8: f64 = (p[87] * (s.dn[742][8] * ddt_scale));let eq31_e1199_d_n9: f64 = (p[87] * (s.dn[742][9] * ddt_scale));let eq31_e1199_d_n10: f64 = (p[87] * (s.dn[742][10] * ddt_scale));let eq31_e1199_d_n11: f64 = (p[87] * (s.dn[742][11] * ddt_scale));let eq31_e1199_d_n12: f64 = (p[87] * (s.dn[742][12] * ddt_scale));let eq31_e1199_d_n13: f64 = (p[87] * (s.dn[742][13] * ddt_scale));let eq31_e1199_d_n14: f64 = (p[87] * (s.dn[742][14] * ddt_scale));let eq31_e1199_d_n15: f64 = (p[87] * (s.dn[742][15] * ddt_scale));let eq31_e1199_d_n16: f64 = (p[87] * (s.dn[742][16] * ddt_scale));let eq31_e1199_d_n17: f64 = (p[87] * (s.dn[742][17] * ddt_scale));let eq31_e1199_d_b0: f64 = (p[87] * (s.db[742][0] * ddt_scale));let eq31_e1199_d_b1: f64 = (p[87] * (s.db[742][1] * ddt_scale));let eq31_e1199_d_b2: f64 = (p[87] * (s.db[742][2] * ddt_scale));let eq31_e1199_d_b3: f64 = (p[87] * (s.db[742][3] * ddt_scale));let eq31_e1199_d_b4: f64 = (p[87] * (s.db[742][4] * ddt_scale));let eq31_e1199_d_b5: f64 = (p[87] * (s.db[742][5] * ddt_scale));let eq31_e1199_d_b6: f64 = (p[87] * (s.db[742][6] * ddt_scale));let eq31_e1199_d_b7: f64 = (p[87] * (s.db[742][7] * ddt_scale));let eq31_e1199_d_b8: f64 = (p[87] * (s.db[742][8] * ddt_scale));let eq31_e1199_d_b9: f64 = (p[87] * (s.db[742][9] * ddt_scale));let eq31_e1199_d_b10: f64 = (p[87] * (s.db[742][10] * ddt_scale));let eq31_e1199_d_b11: f64 = (p[87] * (s.db[742][11] * ddt_scale));let eq31_value: f64 = eq31_e1199;let eq31_node_derivatives: [f64; 18] = [eq31_e1199_d_n0, eq31_e1199_d_n1, eq31_e1199_d_n2, eq31_e1199_d_n3, eq31_e1199_d_n4, eq31_e1199_d_n5, eq31_e1199_d_n6, eq31_e1199_d_n7, eq31_e1199_d_n8, eq31_e1199_d_n9, eq31_e1199_d_n10, eq31_e1199_d_n11, eq31_e1199_d_n12, eq31_e1199_d_n13, eq31_e1199_d_n14, eq31_e1199_d_n15, eq31_e1199_d_n16, eq31_e1199_d_n17];let eq31_branch_derivatives: [f64; 12] = [eq31_e1199_d_b0, eq31_e1199_d_b1, eq31_e1199_d_b2, eq31_e1199_d_b3, eq31_e1199_d_b4, eq31_e1199_d_b5, eq31_e1199_d_b6, eq31_e1199_d_b7, eq31_e1199_d_b8, eq31_e1199_d_b9, eq31_e1199_d_b10, eq31_e1199_d_b11];
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
        let eq32_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[744]);let eq32_e1203: f64 = (p[87] * eq32_e1202);let eq32_e1203_d_n0: f64 = (p[87] * (s.dn[744][0] * ddt_scale));let eq32_e1203_d_n1: f64 = (p[87] * (s.dn[744][1] * ddt_scale));let eq32_e1203_d_n2: f64 = (p[87] * (s.dn[744][2] * ddt_scale));let eq32_e1203_d_n3: f64 = (p[87] * (s.dn[744][3] * ddt_scale));let eq32_e1203_d_n4: f64 = (p[87] * (s.dn[744][4] * ddt_scale));let eq32_e1203_d_n5: f64 = (p[87] * (s.dn[744][5] * ddt_scale));let eq32_e1203_d_n6: f64 = (p[87] * (s.dn[744][6] * ddt_scale));let eq32_e1203_d_n7: f64 = (p[87] * (s.dn[744][7] * ddt_scale));let eq32_e1203_d_n8: f64 = (p[87] * (s.dn[744][8] * ddt_scale));let eq32_e1203_d_n9: f64 = (p[87] * (s.dn[744][9] * ddt_scale));let eq32_e1203_d_n10: f64 = (p[87] * (s.dn[744][10] * ddt_scale));let eq32_e1203_d_n11: f64 = (p[87] * (s.dn[744][11] * ddt_scale));let eq32_e1203_d_n12: f64 = (p[87] * (s.dn[744][12] * ddt_scale));let eq32_e1203_d_n13: f64 = (p[87] * (s.dn[744][13] * ddt_scale));let eq32_e1203_d_n14: f64 = (p[87] * (s.dn[744][14] * ddt_scale));let eq32_e1203_d_n15: f64 = (p[87] * (s.dn[744][15] * ddt_scale));let eq32_e1203_d_n16: f64 = (p[87] * (s.dn[744][16] * ddt_scale));let eq32_e1203_d_n17: f64 = (p[87] * (s.dn[744][17] * ddt_scale));let eq32_e1203_d_b0: f64 = (p[87] * (s.db[744][0] * ddt_scale));let eq32_e1203_d_b1: f64 = (p[87] * (s.db[744][1] * ddt_scale));let eq32_e1203_d_b2: f64 = (p[87] * (s.db[744][2] * ddt_scale));let eq32_e1203_d_b3: f64 = (p[87] * (s.db[744][3] * ddt_scale));let eq32_e1203_d_b4: f64 = (p[87] * (s.db[744][4] * ddt_scale));let eq32_e1203_d_b5: f64 = (p[87] * (s.db[744][5] * ddt_scale));let eq32_e1203_d_b6: f64 = (p[87] * (s.db[744][6] * ddt_scale));let eq32_e1203_d_b7: f64 = (p[87] * (s.db[744][7] * ddt_scale));let eq32_e1203_d_b8: f64 = (p[87] * (s.db[744][8] * ddt_scale));let eq32_e1203_d_b9: f64 = (p[87] * (s.db[744][9] * ddt_scale));let eq32_e1203_d_b10: f64 = (p[87] * (s.db[744][10] * ddt_scale));let eq32_e1203_d_b11: f64 = (p[87] * (s.db[744][11] * ddt_scale));let eq32_value: f64 = eq32_e1203;let eq32_node_derivatives: [f64; 18] = [eq32_e1203_d_n0, eq32_e1203_d_n1, eq32_e1203_d_n2, eq32_e1203_d_n3, eq32_e1203_d_n4, eq32_e1203_d_n5, eq32_e1203_d_n6, eq32_e1203_d_n7, eq32_e1203_d_n8, eq32_e1203_d_n9, eq32_e1203_d_n10, eq32_e1203_d_n11, eq32_e1203_d_n12, eq32_e1203_d_n13, eq32_e1203_d_n14, eq32_e1203_d_n15, eq32_e1203_d_n16, eq32_e1203_d_n17];let eq32_branch_derivatives: [f64; 12] = [eq32_e1203_d_b0, eq32_e1203_d_b1, eq32_e1203_d_b2, eq32_e1203_d_b3, eq32_e1203_d_b4, eq32_e1203_d_b5, eq32_e1203_d_b6, eq32_e1203_d_b7, eq32_e1203_d_b8, eq32_e1203_d_b9, eq32_e1203_d_b10, eq32_e1203_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );let eq33_e1205: f64 = (-p[87]);let eq33_e1207: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[299]);let eq33_e1208: f64 = (eq33_e1205 * eq33_e1207);let eq33_e1208_d_n0: f64 = (eq33_e1205 * (s.dn[299][0] * ddt_scale));let eq33_e1208_d_n1: f64 = (eq33_e1205 * (s.dn[299][1] * ddt_scale));let eq33_e1208_d_n2: f64 = (eq33_e1205 * (s.dn[299][2] * ddt_scale));let eq33_e1208_d_n3: f64 = (eq33_e1205 * (s.dn[299][3] * ddt_scale));let eq33_e1208_d_n4: f64 = (eq33_e1205 * (s.dn[299][4] * ddt_scale));let eq33_e1208_d_n5: f64 = (eq33_e1205 * (s.dn[299][5] * ddt_scale));let eq33_e1208_d_n6: f64 = (eq33_e1205 * (s.dn[299][6] * ddt_scale));let eq33_e1208_d_n7: f64 = (eq33_e1205 * (s.dn[299][7] * ddt_scale));let eq33_e1208_d_n8: f64 = (eq33_e1205 * (s.dn[299][8] * ddt_scale));let eq33_e1208_d_n9: f64 = (eq33_e1205 * (s.dn[299][9] * ddt_scale));let eq33_e1208_d_n10: f64 = (eq33_e1205 * (s.dn[299][10] * ddt_scale));let eq33_e1208_d_n11: f64 = (eq33_e1205 * (s.dn[299][11] * ddt_scale));let eq33_e1208_d_n12: f64 = (eq33_e1205 * (s.dn[299][12] * ddt_scale));let eq33_e1208_d_n13: f64 = (eq33_e1205 * (s.dn[299][13] * ddt_scale));let eq33_e1208_d_n14: f64 = (eq33_e1205 * (s.dn[299][14] * ddt_scale));let eq33_e1208_d_n15: f64 = (eq33_e1205 * (s.dn[299][15] * ddt_scale));let eq33_e1208_d_n16: f64 = (eq33_e1205 * (s.dn[299][16] * ddt_scale));let eq33_e1208_d_n17: f64 = (eq33_e1205 * (s.dn[299][17] * ddt_scale));let eq33_e1208_d_b0: f64 = (eq33_e1205 * (s.db[299][0] * ddt_scale));let eq33_e1208_d_b1: f64 = (eq33_e1205 * (s.db[299][1] * ddt_scale));let eq33_e1208_d_b2: f64 = (eq33_e1205 * (s.db[299][2] * ddt_scale));let eq33_e1208_d_b3: f64 = (eq33_e1205 * (s.db[299][3] * ddt_scale));let eq33_e1208_d_b4: f64 = (eq33_e1205 * (s.db[299][4] * ddt_scale));let eq33_e1208_d_b5: f64 = (eq33_e1205 * (s.db[299][5] * ddt_scale));let eq33_e1208_d_b6: f64 = (eq33_e1205 * (s.db[299][6] * ddt_scale));let eq33_e1208_d_b7: f64 = (eq33_e1205 * (s.db[299][7] * ddt_scale));let eq33_e1208_d_b8: f64 = (eq33_e1205 * (s.db[299][8] * ddt_scale));let eq33_e1208_d_b9: f64 = (eq33_e1205 * (s.db[299][9] * ddt_scale));let eq33_e1208_d_b10: f64 = (eq33_e1205 * (s.db[299][10] * ddt_scale));let eq33_e1208_d_b11: f64 = (eq33_e1205 * (s.db[299][11] * ddt_scale));let eq33_value: f64 = eq33_e1208;let eq33_node_derivatives: [f64; 18] = [eq33_e1208_d_n0, eq33_e1208_d_n1, eq33_e1208_d_n2, eq33_e1208_d_n3, eq33_e1208_d_n4, eq33_e1208_d_n5, eq33_e1208_d_n6, eq33_e1208_d_n7, eq33_e1208_d_n8, eq33_e1208_d_n9, eq33_e1208_d_n10, eq33_e1208_d_n11, eq33_e1208_d_n12, eq33_e1208_d_n13, eq33_e1208_d_n14, eq33_e1208_d_n15, eq33_e1208_d_n16, eq33_e1208_d_n17];let eq33_branch_derivatives: [f64; 12] = [eq33_e1208_d_b0, eq33_e1208_d_b1, eq33_e1208_d_b2, eq33_e1208_d_b3, eq33_e1208_d_b4, eq33_e1208_d_b5, eq33_e1208_d_b6, eq33_e1208_d_b7, eq33_e1208_d_b8, eq33_e1208_d_b9, eq33_e1208_d_b10, eq33_e1208_d_b11];
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
        let nv14 = ctx.node_voltage(nodes[14]);let eq34_e1210: f64 = (-p[87]);let eq34_e1212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[301]);let eq34_e1213: f64 = (eq34_e1210 * eq34_e1212);let eq34_e1213_d_n0: f64 = (eq34_e1210 * (s.dn[301][0] * ddt_scale));let eq34_e1213_d_n1: f64 = (eq34_e1210 * (s.dn[301][1] * ddt_scale));let eq34_e1213_d_n2: f64 = (eq34_e1210 * (s.dn[301][2] * ddt_scale));let eq34_e1213_d_n3: f64 = (eq34_e1210 * (s.dn[301][3] * ddt_scale));let eq34_e1213_d_n4: f64 = (eq34_e1210 * (s.dn[301][4] * ddt_scale));let eq34_e1213_d_n5: f64 = (eq34_e1210 * (s.dn[301][5] * ddt_scale));let eq34_e1213_d_n6: f64 = (eq34_e1210 * (s.dn[301][6] * ddt_scale));let eq34_e1213_d_n7: f64 = (eq34_e1210 * (s.dn[301][7] * ddt_scale));let eq34_e1213_d_n8: f64 = (eq34_e1210 * (s.dn[301][8] * ddt_scale));let eq34_e1213_d_n9: f64 = (eq34_e1210 * (s.dn[301][9] * ddt_scale));let eq34_e1213_d_n10: f64 = (eq34_e1210 * (s.dn[301][10] * ddt_scale));let eq34_e1213_d_n11: f64 = (eq34_e1210 * (s.dn[301][11] * ddt_scale));let eq34_e1213_d_n12: f64 = (eq34_e1210 * (s.dn[301][12] * ddt_scale));let eq34_e1213_d_n13: f64 = (eq34_e1210 * (s.dn[301][13] * ddt_scale));let eq34_e1213_d_n14: f64 = (eq34_e1210 * (s.dn[301][14] * ddt_scale));let eq34_e1213_d_n15: f64 = (eq34_e1210 * (s.dn[301][15] * ddt_scale));let eq34_e1213_d_n16: f64 = (eq34_e1210 * (s.dn[301][16] * ddt_scale));let eq34_e1213_d_n17: f64 = (eq34_e1210 * (s.dn[301][17] * ddt_scale));let eq34_e1213_d_b0: f64 = (eq34_e1210 * (s.db[301][0] * ddt_scale));let eq34_e1213_d_b1: f64 = (eq34_e1210 * (s.db[301][1] * ddt_scale));let eq34_e1213_d_b2: f64 = (eq34_e1210 * (s.db[301][2] * ddt_scale));let eq34_e1213_d_b3: f64 = (eq34_e1210 * (s.db[301][3] * ddt_scale));let eq34_e1213_d_b4: f64 = (eq34_e1210 * (s.db[301][4] * ddt_scale));let eq34_e1213_d_b5: f64 = (eq34_e1210 * (s.db[301][5] * ddt_scale));let eq34_e1213_d_b6: f64 = (eq34_e1210 * (s.db[301][6] * ddt_scale));let eq34_e1213_d_b7: f64 = (eq34_e1210 * (s.db[301][7] * ddt_scale));let eq34_e1213_d_b8: f64 = (eq34_e1210 * (s.db[301][8] * ddt_scale));let eq34_e1213_d_b9: f64 = (eq34_e1210 * (s.db[301][9] * ddt_scale));let eq34_e1213_d_b10: f64 = (eq34_e1210 * (s.db[301][10] * ddt_scale));let eq34_e1213_d_b11: f64 = (eq34_e1210 * (s.db[301][11] * ddt_scale));let eq34_value: f64 = eq34_e1213;let eq34_node_derivatives: [f64; 18] = [eq34_e1213_d_n0, eq34_e1213_d_n1, eq34_e1213_d_n2, eq34_e1213_d_n3, eq34_e1213_d_n4, eq34_e1213_d_n5, eq34_e1213_d_n6, eq34_e1213_d_n7, eq34_e1213_d_n8, eq34_e1213_d_n9, eq34_e1213_d_n10, eq34_e1213_d_n11, eq34_e1213_d_n12, eq34_e1213_d_n13, eq34_e1213_d_n14, eq34_e1213_d_n15, eq34_e1213_d_n16, eq34_e1213_d_n17];let eq34_branch_derivatives: [f64; 12] = [eq34_e1213_d_b0, eq34_e1213_d_b1, eq34_e1213_d_b2, eq34_e1213_d_b3, eq34_e1213_d_b4, eq34_e1213_d_b5, eq34_e1213_d_b6, eq34_e1213_d_b7, eq34_e1213_d_b8, eq34_e1213_d_b9, eq34_e1213_d_b10, eq34_e1213_d_b11];
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
    if (p[52] != 0.0) {
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
}
