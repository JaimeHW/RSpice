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
        let (eq17_e1113, eq17_e1113_d_n0, eq17_e1113_d_n1, eq17_e1113_d_n2, eq17_e1113_d_n3, eq17_e1113_d_n4, eq17_e1113_d_n5, eq17_e1113_d_n6, eq17_e1113_d_n7, eq17_e1113_d_n8, eq17_e1113_d_n9, eq17_e1113_d_n10, eq17_e1113_d_n11, eq17_e1113_d_n12, eq17_e1113_d_n13, eq17_e1113_d_n14, eq17_e1113_d_n15, eq17_e1113_d_n16, eq17_e1113_d_n17, eq17_e1113_d_n18, eq17_e1113_d_b0, eq17_e1113_d_b1, eq17_e1113_d_b2, eq17_e1113_d_b3, eq17_e1113_d_b4, eq17_e1113_d_b5, eq17_e1113_d_b6, eq17_e1113_d_b7, eq17_e1113_d_b8, eq17_e1113_d_b9, eq17_e1113_d_b10, eq17_e1113_d_b11, eq17_e1113_d_b12,) = {
    if s.b[3409] {
        let eq17_e1111: f64 = (p.p87 * s.v[870]);
        (eq17_e1111, (p.p87 * s.dn[870][0]), (p.p87 * s.dn[870][1]), (p.p87 * s.dn[870][2]), (p.p87 * s.dn[870][3]), (p.p87 * s.dn[870][4]), (p.p87 * s.dn[870][5]), (p.p87 * s.dn[870][6]), (p.p87 * s.dn[870][7]), (p.p87 * s.dn[870][8]), (p.p87 * s.dn[870][9]), (p.p87 * s.dn[870][10]), (p.p87 * s.dn[870][11]), (p.p87 * s.dn[870][12]), (p.p87 * s.dn[870][13]), (p.p87 * s.dn[870][14]), (p.p87 * s.dn[870][15]), (p.p87 * s.dn[870][16]), (p.p87 * s.dn[870][17]), (p.p87 * s.dn[870][18]), (p.p87 * s.db[870][0]), (p.p87 * s.db[870][1]), (p.p87 * s.db[870][2]), (p.p87 * s.db[870][3]), (p.p87 * s.db[870][4]), (p.p87 * s.db[870][5]), (p.p87 * s.db[870][6]), (p.p87 * s.db[870][7]), (p.p87 * s.db[870][8]), (p.p87 * s.db[870][9]), (p.p87 * s.db[870][10]), (p.p87 * s.db[870][11]), (p.p87 * s.db[870][12]),)
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
        let eq18_e1117: f64 = (p.p87 * s.v[869]);
        (eq18_e1117, (p.p87 * s.dn[869][0]), (p.p87 * s.dn[869][1]), (p.p87 * s.dn[869][2]), (p.p87 * s.dn[869][3]), (p.p87 * s.dn[869][4]), (p.p87 * s.dn[869][5]), (p.p87 * s.dn[869][6]), (p.p87 * s.dn[869][7]), (p.p87 * s.dn[869][8]), (p.p87 * s.dn[869][9]), (p.p87 * s.dn[869][10]), (p.p87 * s.dn[869][11]), (p.p87 * s.dn[869][12]), (p.p87 * s.dn[869][13]), (p.p87 * s.dn[869][14]), (p.p87 * s.dn[869][15]), (p.p87 * s.dn[869][16]), (p.p87 * s.dn[869][17]), (p.p87 * s.dn[869][18]), (p.p87 * s.db[869][0]), (p.p87 * s.db[869][1]), (p.p87 * s.db[869][2]), (p.p87 * s.db[869][3]), (p.p87 * s.db[869][4]), (p.p87 * s.db[869][5]), (p.p87 * s.db[869][6]), (p.p87 * s.db[869][7]), (p.p87 * s.db[869][8]), (p.p87 * s.db[869][9]), (p.p87 * s.db[869][10]), (p.p87 * s.db[869][11]), (p.p87 * s.db[869][12]),)
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
        let eq19_e1123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[68]);let eq19_e1124: f64 = (p.p87 * eq19_e1123);let eq19_e1124_d_n0: f64 = (p.p87 * (s.dn[68][0] * ddt_scale));let eq19_e1124_d_n1: f64 = (p.p87 * (s.dn[68][1] * ddt_scale));let eq19_e1124_d_n2: f64 = (p.p87 * (s.dn[68][2] * ddt_scale));let eq19_e1124_d_n3: f64 = (p.p87 * (s.dn[68][3] * ddt_scale));let eq19_e1124_d_n4: f64 = (p.p87 * (s.dn[68][4] * ddt_scale));let eq19_e1124_d_n5: f64 = (p.p87 * (s.dn[68][5] * ddt_scale));let eq19_e1124_d_n6: f64 = (p.p87 * (s.dn[68][6] * ddt_scale));let eq19_e1124_d_n7: f64 = (p.p87 * (s.dn[68][7] * ddt_scale));let eq19_e1124_d_n8: f64 = (p.p87 * (s.dn[68][8] * ddt_scale));let eq19_e1124_d_n9: f64 = (p.p87 * (s.dn[68][9] * ddt_scale));let eq19_e1124_d_n10: f64 = (p.p87 * (s.dn[68][10] * ddt_scale));let eq19_e1124_d_n11: f64 = (p.p87 * (s.dn[68][11] * ddt_scale));let eq19_e1124_d_n12: f64 = (p.p87 * (s.dn[68][12] * ddt_scale));let eq19_e1124_d_n13: f64 = (p.p87 * (s.dn[68][13] * ddt_scale));let eq19_e1124_d_n14: f64 = (p.p87 * (s.dn[68][14] * ddt_scale));let eq19_e1124_d_n15: f64 = (p.p87 * (s.dn[68][15] * ddt_scale));let eq19_e1124_d_n16: f64 = (p.p87 * (s.dn[68][16] * ddt_scale));let eq19_e1124_d_n17: f64 = (p.p87 * (s.dn[68][17] * ddt_scale));let eq19_e1124_d_n18: f64 = (p.p87 * (s.dn[68][18] * ddt_scale));let eq19_e1124_d_b0: f64 = (p.p87 * (s.db[68][0] * ddt_scale));let eq19_e1124_d_b1: f64 = (p.p87 * (s.db[68][1] * ddt_scale));let eq19_e1124_d_b2: f64 = (p.p87 * (s.db[68][2] * ddt_scale));let eq19_e1124_d_b3: f64 = (p.p87 * (s.db[68][3] * ddt_scale));let eq19_e1124_d_b4: f64 = (p.p87 * (s.db[68][4] * ddt_scale));let eq19_e1124_d_b5: f64 = (p.p87 * (s.db[68][5] * ddt_scale));let eq19_e1124_d_b6: f64 = (p.p87 * (s.db[68][6] * ddt_scale));let eq19_e1124_d_b7: f64 = (p.p87 * (s.db[68][7] * ddt_scale));let eq19_e1124_d_b8: f64 = (p.p87 * (s.db[68][8] * ddt_scale));let eq19_e1124_d_b9: f64 = (p.p87 * (s.db[68][9] * ddt_scale));let eq19_e1124_d_b10: f64 = (p.p87 * (s.db[68][10] * ddt_scale));let eq19_e1124_d_b11: f64 = (p.p87 * (s.db[68][11] * ddt_scale));let eq19_e1124_d_b12: f64 = (p.p87 * (s.db[68][12] * ddt_scale));
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
        let eq20_e1130: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[67]);let eq20_e1131: f64 = (p.p87 * eq20_e1130);let eq20_e1131_d_n0: f64 = (p.p87 * (s.dn[67][0] * ddt_scale));let eq20_e1131_d_n1: f64 = (p.p87 * (s.dn[67][1] * ddt_scale));let eq20_e1131_d_n2: f64 = (p.p87 * (s.dn[67][2] * ddt_scale));let eq20_e1131_d_n3: f64 = (p.p87 * (s.dn[67][3] * ddt_scale));let eq20_e1131_d_n4: f64 = (p.p87 * (s.dn[67][4] * ddt_scale));let eq20_e1131_d_n5: f64 = (p.p87 * (s.dn[67][5] * ddt_scale));let eq20_e1131_d_n6: f64 = (p.p87 * (s.dn[67][6] * ddt_scale));let eq20_e1131_d_n7: f64 = (p.p87 * (s.dn[67][7] * ddt_scale));let eq20_e1131_d_n8: f64 = (p.p87 * (s.dn[67][8] * ddt_scale));let eq20_e1131_d_n9: f64 = (p.p87 * (s.dn[67][9] * ddt_scale));let eq20_e1131_d_n10: f64 = (p.p87 * (s.dn[67][10] * ddt_scale));let eq20_e1131_d_n11: f64 = (p.p87 * (s.dn[67][11] * ddt_scale));let eq20_e1131_d_n12: f64 = (p.p87 * (s.dn[67][12] * ddt_scale));let eq20_e1131_d_n13: f64 = (p.p87 * (s.dn[67][13] * ddt_scale));let eq20_e1131_d_n14: f64 = (p.p87 * (s.dn[67][14] * ddt_scale));let eq20_e1131_d_n15: f64 = (p.p87 * (s.dn[67][15] * ddt_scale));let eq20_e1131_d_n16: f64 = (p.p87 * (s.dn[67][16] * ddt_scale));let eq20_e1131_d_n17: f64 = (p.p87 * (s.dn[67][17] * ddt_scale));let eq20_e1131_d_n18: f64 = (p.p87 * (s.dn[67][18] * ddt_scale));let eq20_e1131_d_b0: f64 = (p.p87 * (s.db[67][0] * ddt_scale));let eq20_e1131_d_b1: f64 = (p.p87 * (s.db[67][1] * ddt_scale));let eq20_e1131_d_b2: f64 = (p.p87 * (s.db[67][2] * ddt_scale));let eq20_e1131_d_b3: f64 = (p.p87 * (s.db[67][3] * ddt_scale));let eq20_e1131_d_b4: f64 = (p.p87 * (s.db[67][4] * ddt_scale));let eq20_e1131_d_b5: f64 = (p.p87 * (s.db[67][5] * ddt_scale));let eq20_e1131_d_b6: f64 = (p.p87 * (s.db[67][6] * ddt_scale));let eq20_e1131_d_b7: f64 = (p.p87 * (s.db[67][7] * ddt_scale));let eq20_e1131_d_b8: f64 = (p.p87 * (s.db[67][8] * ddt_scale));let eq20_e1131_d_b9: f64 = (p.p87 * (s.db[67][9] * ddt_scale));let eq20_e1131_d_b10: f64 = (p.p87 * (s.db[67][10] * ddt_scale));let eq20_e1131_d_b11: f64 = (p.p87 * (s.db[67][11] * ddt_scale));let eq20_e1131_d_b12: f64 = (p.p87 * (s.db[67][12] * ddt_scale));
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
        let eq21_e1137: f64 = (p.p87 * s.v[200]);
        (eq21_e1137, (p.p87 * s.dn[200][0]), (p.p87 * s.dn[200][1]), (p.p87 * s.dn[200][2]), (p.p87 * s.dn[200][3]), (p.p87 * s.dn[200][4]), (p.p87 * s.dn[200][5]), (p.p87 * s.dn[200][6]), (p.p87 * s.dn[200][7]), (p.p87 * s.dn[200][8]), (p.p87 * s.dn[200][9]), (p.p87 * s.dn[200][10]), (p.p87 * s.dn[200][11]), (p.p87 * s.dn[200][12]), (p.p87 * s.dn[200][13]), (p.p87 * s.dn[200][14]), (p.p87 * s.dn[200][15]), (p.p87 * s.dn[200][16]), (p.p87 * s.dn[200][17]), (p.p87 * s.dn[200][18]), (p.p87 * s.db[200][0]), (p.p87 * s.db[200][1]), (p.p87 * s.db[200][2]), (p.p87 * s.db[200][3]), (p.p87 * s.db[200][4]), (p.p87 * s.db[200][5]), (p.p87 * s.db[200][6]), (p.p87 * s.db[200][7]), (p.p87 * s.db[200][8]), (p.p87 * s.db[200][9]), (p.p87 * s.db[200][10]), (p.p87 * s.db[200][11]), (p.p87 * s.db[200][12]),)
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
        let eq22_e1143: f64 = (p.p87 * s.v[201]);
        (eq22_e1143, (p.p87 * s.dn[201][0]), (p.p87 * s.dn[201][1]), (p.p87 * s.dn[201][2]), (p.p87 * s.dn[201][3]), (p.p87 * s.dn[201][4]), (p.p87 * s.dn[201][5]), (p.p87 * s.dn[201][6]), (p.p87 * s.dn[201][7]), (p.p87 * s.dn[201][8]), (p.p87 * s.dn[201][9]), (p.p87 * s.dn[201][10]), (p.p87 * s.dn[201][11]), (p.p87 * s.dn[201][12]), (p.p87 * s.dn[201][13]), (p.p87 * s.dn[201][14]), (p.p87 * s.dn[201][15]), (p.p87 * s.dn[201][16]), (p.p87 * s.dn[201][17]), (p.p87 * s.dn[201][18]), (p.p87 * s.db[201][0]), (p.p87 * s.db[201][1]), (p.p87 * s.db[201][2]), (p.p87 * s.db[201][3]), (p.p87 * s.db[201][4]), (p.p87 * s.db[201][5]), (p.p87 * s.db[201][6]), (p.p87 * s.db[201][7]), (p.p87 * s.db[201][8]), (p.p87 * s.db[201][9]), (p.p87 * s.db[201][10]), (p.p87 * s.db[201][11]), (p.p87 * s.db[201][12]),)
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
        let eq23_e1149: f64 = (p.p87 * s.v[202]);
        (eq23_e1149, (p.p87 * s.dn[202][0]), (p.p87 * s.dn[202][1]), (p.p87 * s.dn[202][2]), (p.p87 * s.dn[202][3]), (p.p87 * s.dn[202][4]), (p.p87 * s.dn[202][5]), (p.p87 * s.dn[202][6]), (p.p87 * s.dn[202][7]), (p.p87 * s.dn[202][8]), (p.p87 * s.dn[202][9]), (p.p87 * s.dn[202][10]), (p.p87 * s.dn[202][11]), (p.p87 * s.dn[202][12]), (p.p87 * s.dn[202][13]), (p.p87 * s.dn[202][14]), (p.p87 * s.dn[202][15]), (p.p87 * s.dn[202][16]), (p.p87 * s.dn[202][17]), (p.p87 * s.dn[202][18]), (p.p87 * s.db[202][0]), (p.p87 * s.db[202][1]), (p.p87 * s.db[202][2]), (p.p87 * s.db[202][3]), (p.p87 * s.db[202][4]), (p.p87 * s.db[202][5]), (p.p87 * s.db[202][6]), (p.p87 * s.db[202][7]), (p.p87 * s.db[202][8]), (p.p87 * s.db[202][9]), (p.p87 * s.db[202][10]), (p.p87 * s.db[202][11]), (p.p87 * s.db[202][12]),)
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
        let eq28_e1177: f64 = (s.v[18] + s.v[753]);let eq28_e1177_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);let eq28_e1177_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);let eq28_e1177_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);let eq28_e1177_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);let eq28_e1177_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);let eq28_e1177_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);let eq28_e1177_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);let eq28_e1177_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);let eq28_e1177_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);let eq28_e1177_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);let eq28_e1177_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);let eq28_e1177_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);let eq28_e1177_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);let eq28_e1177_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);let eq28_e1177_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);let eq28_e1177_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);let eq28_e1177_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);let eq28_e1177_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);let eq28_e1177_d_n18: f64 = (s.dn[18][18] + s.dn[753][18]);let eq28_e1177_d_b0: f64 = (s.db[18][0] + s.db[753][0]);let eq28_e1177_d_b1: f64 = (s.db[18][1] + s.db[753][1]);let eq28_e1177_d_b2: f64 = (s.db[18][2] + s.db[753][2]);let eq28_e1177_d_b3: f64 = (s.db[18][3] + s.db[753][3]);let eq28_e1177_d_b4: f64 = (s.db[18][4] + s.db[753][4]);let eq28_e1177_d_b5: f64 = (s.db[18][5] + s.db[753][5]);let eq28_e1177_d_b6: f64 = (s.db[18][6] + s.db[753][6]);let eq28_e1177_d_b7: f64 = (s.db[18][7] + s.db[753][7]);let eq28_e1177_d_b8: f64 = (s.db[18][8] + s.db[753][8]);let eq28_e1177_d_b9: f64 = (s.db[18][9] + s.db[753][9]);let eq28_e1177_d_b10: f64 = (s.db[18][10] + s.db[753][10]);let eq28_e1177_d_b11: f64 = (s.db[18][11] + s.db[753][11]);let eq28_e1177_d_b12: f64 = (s.db[18][12] + s.db[753][12]);let eq28_e1178: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1177);let eq28_e1179: f64 = (p.p87 * eq28_e1178);let eq28_e1179_d_n0: f64 = (p.p87 * (eq28_e1177_d_n0 * ddt_scale));let eq28_e1179_d_n1: f64 = (p.p87 * (eq28_e1177_d_n1 * ddt_scale));let eq28_e1179_d_n2: f64 = (p.p87 * (eq28_e1177_d_n2 * ddt_scale));let eq28_e1179_d_n3: f64 = (p.p87 * (eq28_e1177_d_n3 * ddt_scale));let eq28_e1179_d_n4: f64 = (p.p87 * (eq28_e1177_d_n4 * ddt_scale));let eq28_e1179_d_n5: f64 = (p.p87 * (eq28_e1177_d_n5 * ddt_scale));let eq28_e1179_d_n6: f64 = (p.p87 * (eq28_e1177_d_n6 * ddt_scale));let eq28_e1179_d_n7: f64 = (p.p87 * (eq28_e1177_d_n7 * ddt_scale));let eq28_e1179_d_n8: f64 = (p.p87 * (eq28_e1177_d_n8 * ddt_scale));let eq28_e1179_d_n9: f64 = (p.p87 * (eq28_e1177_d_n9 * ddt_scale));let eq28_e1179_d_n10: f64 = (p.p87 * (eq28_e1177_d_n10 * ddt_scale));let eq28_e1179_d_n11: f64 = (p.p87 * (eq28_e1177_d_n11 * ddt_scale));let eq28_e1179_d_n12: f64 = (p.p87 * (eq28_e1177_d_n12 * ddt_scale));let eq28_e1179_d_n13: f64 = (p.p87 * (eq28_e1177_d_n13 * ddt_scale));let eq28_e1179_d_n14: f64 = (p.p87 * (eq28_e1177_d_n14 * ddt_scale));let eq28_e1179_d_n15: f64 = (p.p87 * (eq28_e1177_d_n15 * ddt_scale));let eq28_e1179_d_n16: f64 = (p.p87 * (eq28_e1177_d_n16 * ddt_scale));let eq28_e1179_d_n17: f64 = (p.p87 * (eq28_e1177_d_n17 * ddt_scale));let eq28_e1179_d_n18: f64 = (p.p87 * (eq28_e1177_d_n18 * ddt_scale));let eq28_e1179_d_b0: f64 = (p.p87 * (eq28_e1177_d_b0 * ddt_scale));let eq28_e1179_d_b1: f64 = (p.p87 * (eq28_e1177_d_b1 * ddt_scale));let eq28_e1179_d_b2: f64 = (p.p87 * (eq28_e1177_d_b2 * ddt_scale));let eq28_e1179_d_b3: f64 = (p.p87 * (eq28_e1177_d_b3 * ddt_scale));let eq28_e1179_d_b4: f64 = (p.p87 * (eq28_e1177_d_b4 * ddt_scale));let eq28_e1179_d_b5: f64 = (p.p87 * (eq28_e1177_d_b5 * ddt_scale));let eq28_e1179_d_b6: f64 = (p.p87 * (eq28_e1177_d_b6 * ddt_scale));let eq28_e1179_d_b7: f64 = (p.p87 * (eq28_e1177_d_b7 * ddt_scale));let eq28_e1179_d_b8: f64 = (p.p87 * (eq28_e1177_d_b8 * ddt_scale));
        let eq28_e1179_d_b9: f64 = (p.p87 * (eq28_e1177_d_b9 * ddt_scale));let eq28_e1179_d_b10: f64 = (p.p87 * (eq28_e1177_d_b10 * ddt_scale));let eq28_e1179_d_b11: f64 = (p.p87 * (eq28_e1177_d_b11 * ddt_scale));let eq28_e1179_d_b12: f64 = (p.p87 * (eq28_e1177_d_b12 * ddt_scale));let eq28_value: f64 = eq28_e1179;let eq28_node_derivatives: [f64; 19] = [eq28_e1179_d_n0, eq28_e1179_d_n1, eq28_e1179_d_n2, eq28_e1179_d_n3, eq28_e1179_d_n4, eq28_e1179_d_n5, eq28_e1179_d_n6, eq28_e1179_d_n7, eq28_e1179_d_n8, eq28_e1179_d_n9, eq28_e1179_d_n10, eq28_e1179_d_n11, eq28_e1179_d_n12, eq28_e1179_d_n13, eq28_e1179_d_n14, eq28_e1179_d_n15, eq28_e1179_d_n16, eq28_e1179_d_n17, eq28_e1179_d_n18];let eq28_branch_derivatives: [f64; 13] = [eq28_e1179_d_b0, eq28_e1179_d_b1, eq28_e1179_d_b2, eq28_e1179_d_b3, eq28_e1179_d_b4, eq28_e1179_d_b5, eq28_e1179_d_b6, eq28_e1179_d_b7, eq28_e1179_d_b8, eq28_e1179_d_b9, eq28_e1179_d_b10, eq28_e1179_d_b11, eq28_e1179_d_b12];
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
        let eq29_e1183: f64 = (s.v[19] + s.v[751]);let eq29_e1183_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);let eq29_e1183_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);let eq29_e1183_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);let eq29_e1183_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);let eq29_e1183_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);let eq29_e1183_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);let eq29_e1183_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);let eq29_e1183_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);let eq29_e1183_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);let eq29_e1183_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);let eq29_e1183_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);let eq29_e1183_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);let eq29_e1183_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);let eq29_e1183_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);let eq29_e1183_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);let eq29_e1183_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);let eq29_e1183_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);let eq29_e1183_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);let eq29_e1183_d_n18: f64 = (s.dn[19][18] + s.dn[751][18]);let eq29_e1183_d_b0: f64 = (s.db[19][0] + s.db[751][0]);let eq29_e1183_d_b1: f64 = (s.db[19][1] + s.db[751][1]);let eq29_e1183_d_b2: f64 = (s.db[19][2] + s.db[751][2]);let eq29_e1183_d_b3: f64 = (s.db[19][3] + s.db[751][3]);let eq29_e1183_d_b4: f64 = (s.db[19][4] + s.db[751][4]);let eq29_e1183_d_b5: f64 = (s.db[19][5] + s.db[751][5]);let eq29_e1183_d_b6: f64 = (s.db[19][6] + s.db[751][6]);let eq29_e1183_d_b7: f64 = (s.db[19][7] + s.db[751][7]);let eq29_e1183_d_b8: f64 = (s.db[19][8] + s.db[751][8]);let eq29_e1183_d_b9: f64 = (s.db[19][9] + s.db[751][9]);let eq29_e1183_d_b10: f64 = (s.db[19][10] + s.db[751][10]);let eq29_e1183_d_b11: f64 = (s.db[19][11] + s.db[751][11]);let eq29_e1183_d_b12: f64 = (s.db[19][12] + s.db[751][12]);let eq29_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq29_e1183);let eq29_e1185: f64 = (p.p87 * eq29_e1184);let eq29_e1185_d_n0: f64 = (p.p87 * (eq29_e1183_d_n0 * ddt_scale));let eq29_e1185_d_n1: f64 = (p.p87 * (eq29_e1183_d_n1 * ddt_scale));let eq29_e1185_d_n2: f64 = (p.p87 * (eq29_e1183_d_n2 * ddt_scale));let eq29_e1185_d_n3: f64 = (p.p87 * (eq29_e1183_d_n3 * ddt_scale));let eq29_e1185_d_n4: f64 = (p.p87 * (eq29_e1183_d_n4 * ddt_scale));let eq29_e1185_d_n5: f64 = (p.p87 * (eq29_e1183_d_n5 * ddt_scale));let eq29_e1185_d_n6: f64 = (p.p87 * (eq29_e1183_d_n6 * ddt_scale));let eq29_e1185_d_n7: f64 = (p.p87 * (eq29_e1183_d_n7 * ddt_scale));let eq29_e1185_d_n8: f64 = (p.p87 * (eq29_e1183_d_n8 * ddt_scale));let eq29_e1185_d_n9: f64 = (p.p87 * (eq29_e1183_d_n9 * ddt_scale));let eq29_e1185_d_n10: f64 = (p.p87 * (eq29_e1183_d_n10 * ddt_scale));let eq29_e1185_d_n11: f64 = (p.p87 * (eq29_e1183_d_n11 * ddt_scale));let eq29_e1185_d_n12: f64 = (p.p87 * (eq29_e1183_d_n12 * ddt_scale));let eq29_e1185_d_n13: f64 = (p.p87 * (eq29_e1183_d_n13 * ddt_scale));let eq29_e1185_d_n14: f64 = (p.p87 * (eq29_e1183_d_n14 * ddt_scale));let eq29_e1185_d_n15: f64 = (p.p87 * (eq29_e1183_d_n15 * ddt_scale));let eq29_e1185_d_n16: f64 = (p.p87 * (eq29_e1183_d_n16 * ddt_scale));let eq29_e1185_d_n17: f64 = (p.p87 * (eq29_e1183_d_n17 * ddt_scale));let eq29_e1185_d_n18: f64 = (p.p87 * (eq29_e1183_d_n18 * ddt_scale));let eq29_e1185_d_b0: f64 = (p.p87 * (eq29_e1183_d_b0 * ddt_scale));let eq29_e1185_d_b1: f64 = (p.p87 * (eq29_e1183_d_b1 * ddt_scale));let eq29_e1185_d_b2: f64 = (p.p87 * (eq29_e1183_d_b2 * ddt_scale));let eq29_e1185_d_b3: f64 = (p.p87 * (eq29_e1183_d_b3 * ddt_scale));let eq29_e1185_d_b4: f64 = (p.p87 * (eq29_e1183_d_b4 * ddt_scale));let eq29_e1185_d_b5: f64 = (p.p87 * (eq29_e1183_d_b5 * ddt_scale));let eq29_e1185_d_b6: f64 = (p.p87 * (eq29_e1183_d_b6 * ddt_scale));let eq29_e1185_d_b7: f64 = (p.p87 * (eq29_e1183_d_b7 * ddt_scale));let eq29_e1185_d_b8: f64 = (p.p87 * (eq29_e1183_d_b8 * ddt_scale));
        let eq29_e1185_d_b9: f64 = (p.p87 * (eq29_e1183_d_b9 * ddt_scale));let eq29_e1185_d_b10: f64 = (p.p87 * (eq29_e1183_d_b10 * ddt_scale));let eq29_e1185_d_b11: f64 = (p.p87 * (eq29_e1183_d_b11 * ddt_scale));let eq29_e1185_d_b12: f64 = (p.p87 * (eq29_e1183_d_b12 * ddt_scale));let eq29_value: f64 = eq29_e1185;let eq29_node_derivatives: [f64; 19] = [eq29_e1185_d_n0, eq29_e1185_d_n1, eq29_e1185_d_n2, eq29_e1185_d_n3, eq29_e1185_d_n4, eq29_e1185_d_n5, eq29_e1185_d_n6, eq29_e1185_d_n7, eq29_e1185_d_n8, eq29_e1185_d_n9, eq29_e1185_d_n10, eq29_e1185_d_n11, eq29_e1185_d_n12, eq29_e1185_d_n13, eq29_e1185_d_n14, eq29_e1185_d_n15, eq29_e1185_d_n16, eq29_e1185_d_n17, eq29_e1185_d_n18];let eq29_branch_derivatives: [f64; 13] = [eq29_e1185_d_b0, eq29_e1185_d_b1, eq29_e1185_d_b2, eq29_e1185_d_b3, eq29_e1185_d_b4, eq29_e1185_d_b5, eq29_e1185_d_b6, eq29_e1185_d_b7, eq29_e1185_d_b8, eq29_e1185_d_b9, eq29_e1185_d_b10, eq29_e1185_d_b11, eq29_e1185_d_b12];
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
        let eq30_e1193_d_n2: f64 = (s.dn[20][2] - eq30_e1192_d_n2);let eq30_e1193_d_n3: f64 = (s.dn[20][3] - eq30_e1192_d_n3);let eq30_e1193_d_n4: f64 = (s.dn[20][4] - eq30_e1192_d_n4);let eq30_e1193_d_n5: f64 = (s.dn[20][5] - eq30_e1192_d_n5);let eq30_e1193_d_n6: f64 = (s.dn[20][6] - eq30_e1192_d_n6);let eq30_e1193_d_n7: f64 = (s.dn[20][7] - eq30_e1192_d_n7);let eq30_e1193_d_n8: f64 = (s.dn[20][8] - eq30_e1192_d_n8);let eq30_e1193_d_n9: f64 = (s.dn[20][9] - eq30_e1192_d_n9);let eq30_e1193_d_n10: f64 = (s.dn[20][10] - eq30_e1192_d_n10);let eq30_e1193_d_n11: f64 = (s.dn[20][11] - eq30_e1192_d_n11);let eq30_e1193_d_n12: f64 = (s.dn[20][12] - eq30_e1192_d_n12);let eq30_e1193_d_n13: f64 = (s.dn[20][13] - eq30_e1192_d_n13);let eq30_e1193_d_n14: f64 = (s.dn[20][14] - eq30_e1192_d_n14);let eq30_e1193_d_n15: f64 = (s.dn[20][15] - eq30_e1192_d_n15);let eq30_e1193_d_n16: f64 = (s.dn[20][16] - eq30_e1192_d_n16);let eq30_e1193_d_n17: f64 = (s.dn[20][17] - eq30_e1192_d_n17);let eq30_e1193_d_n18: f64 = (s.dn[20][18] - eq30_e1192_d_n18);let eq30_e1193_d_b0: f64 = (s.db[20][0] - eq30_e1192_d_b0);let eq30_e1193_d_b1: f64 = (s.db[20][1] - eq30_e1192_d_b1);let eq30_e1193_d_b2: f64 = (s.db[20][2] - eq30_e1192_d_b2);let eq30_e1193_d_b3: f64 = (s.db[20][3] - eq30_e1192_d_b3);let eq30_e1193_d_b4: f64 = (s.db[20][4] - eq30_e1192_d_b4);let eq30_e1193_d_b5: f64 = (s.db[20][5] - eq30_e1192_d_b5);let eq30_e1193_d_b6: f64 = (s.db[20][6] - eq30_e1192_d_b6);let eq30_e1193_d_b7: f64 = (s.db[20][7] - eq30_e1192_d_b7);let eq30_e1193_d_b8: f64 = (s.db[20][8] - eq30_e1192_d_b8);let eq30_e1193_d_b9: f64 = (s.db[20][9] - eq30_e1192_d_b9);let eq30_e1193_d_b10: f64 = (s.db[20][10] - eq30_e1192_d_b10);let eq30_e1193_d_b11: f64 = (s.db[20][11] - eq30_e1192_d_b11);let eq30_e1193_d_b12: f64 = (s.db[20][12] - eq30_e1192_d_b12);let eq30_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq30_e1193);let eq30_e1195: f64 = (p.p87 * eq30_e1194);let eq30_e1195_d_n0: f64 = (p.p87 * (eq30_e1193_d_n0 * ddt_scale));let eq30_e1195_d_n1: f64 = (p.p87 * (eq30_e1193_d_n1 * ddt_scale));let eq30_e1195_d_n2: f64 = (p.p87 * (eq30_e1193_d_n2 * ddt_scale));let eq30_e1195_d_n3: f64 = (p.p87 * (eq30_e1193_d_n3 * ddt_scale));let eq30_e1195_d_n4: f64 = (p.p87 * (eq30_e1193_d_n4 * ddt_scale));let eq30_e1195_d_n5: f64 = (p.p87 * (eq30_e1193_d_n5 * ddt_scale));let eq30_e1195_d_n6: f64 = (p.p87 * (eq30_e1193_d_n6 * ddt_scale));let eq30_e1195_d_n7: f64 = (p.p87 * (eq30_e1193_d_n7 * ddt_scale));let eq30_e1195_d_n8: f64 = (p.p87 * (eq30_e1193_d_n8 * ddt_scale));let eq30_e1195_d_n9: f64 = (p.p87 * (eq30_e1193_d_n9 * ddt_scale));let eq30_e1195_d_n10: f64 = (p.p87 * (eq30_e1193_d_n10 * ddt_scale));let eq30_e1195_d_n11: f64 = (p.p87 * (eq30_e1193_d_n11 * ddt_scale));let eq30_e1195_d_n12: f64 = (p.p87 * (eq30_e1193_d_n12 * ddt_scale));let eq30_e1195_d_n13: f64 = (p.p87 * (eq30_e1193_d_n13 * ddt_scale));let eq30_e1195_d_n14: f64 = (p.p87 * (eq30_e1193_d_n14 * ddt_scale));let eq30_e1195_d_n15: f64 = (p.p87 * (eq30_e1193_d_n15 * ddt_scale));let eq30_e1195_d_n16: f64 = (p.p87 * (eq30_e1193_d_n16 * ddt_scale));let eq30_e1195_d_n17: f64 = (p.p87 * (eq30_e1193_d_n17 * ddt_scale));let eq30_e1195_d_n18: f64 = (p.p87 * (eq30_e1193_d_n18 * ddt_scale));let eq30_e1195_d_b0: f64 = (p.p87 * (eq30_e1193_d_b0 * ddt_scale));let eq30_e1195_d_b1: f64 = (p.p87 * (eq30_e1193_d_b1 * ddt_scale));let eq30_e1195_d_b2: f64 = (p.p87 * (eq30_e1193_d_b2 * ddt_scale));let eq30_e1195_d_b3: f64 = (p.p87 * (eq30_e1193_d_b3 * ddt_scale));let eq30_e1195_d_b4: f64 = (p.p87 * (eq30_e1193_d_b4 * ddt_scale));let eq30_e1195_d_b5: f64 = (p.p87 * (eq30_e1193_d_b5 * ddt_scale));let eq30_e1195_d_b6: f64 = (p.p87 * (eq30_e1193_d_b6 * ddt_scale));let eq30_e1195_d_b7: f64 = (p.p87 * (eq30_e1193_d_b7 * ddt_scale));let eq30_e1195_d_b8: f64 = (p.p87 * (eq30_e1193_d_b8 * ddt_scale));let eq30_e1195_d_b9: f64 = (p.p87 * (eq30_e1193_d_b9 * ddt_scale));
        let eq30_e1195_d_b10: f64 = (p.p87 * (eq30_e1193_d_b10 * ddt_scale));let eq30_e1195_d_b11: f64 = (p.p87 * (eq30_e1193_d_b11 * ddt_scale));let eq30_e1195_d_b12: f64 = (p.p87 * (eq30_e1193_d_b12 * ddt_scale));let eq30_value: f64 = eq30_e1195;let eq30_node_derivatives: [f64; 19] = [eq30_e1195_d_n0, eq30_e1195_d_n1, eq30_e1195_d_n2, eq30_e1195_d_n3, eq30_e1195_d_n4, eq30_e1195_d_n5, eq30_e1195_d_n6, eq30_e1195_d_n7, eq30_e1195_d_n8, eq30_e1195_d_n9, eq30_e1195_d_n10, eq30_e1195_d_n11, eq30_e1195_d_n12, eq30_e1195_d_n13, eq30_e1195_d_n14, eq30_e1195_d_n15, eq30_e1195_d_n16, eq30_e1195_d_n17, eq30_e1195_d_n18];let eq30_branch_derivatives: [f64; 13] = [eq30_e1195_d_b0, eq30_e1195_d_b1, eq30_e1195_d_b2, eq30_e1195_d_b3, eq30_e1195_d_b4, eq30_e1195_d_b5, eq30_e1195_d_b6, eq30_e1195_d_b7, eq30_e1195_d_b8, eq30_e1195_d_b9, eq30_e1195_d_b10, eq30_e1195_d_b11, eq30_e1195_d_b12];
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
        let eq31_e1198: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[743]);let eq31_e1199: f64 = (p.p87 * eq31_e1198);let eq31_e1199_d_n0: f64 = (p.p87 * (s.dn[743][0] * ddt_scale));let eq31_e1199_d_n1: f64 = (p.p87 * (s.dn[743][1] * ddt_scale));let eq31_e1199_d_n2: f64 = (p.p87 * (s.dn[743][2] * ddt_scale));let eq31_e1199_d_n3: f64 = (p.p87 * (s.dn[743][3] * ddt_scale));let eq31_e1199_d_n4: f64 = (p.p87 * (s.dn[743][4] * ddt_scale));let eq31_e1199_d_n5: f64 = (p.p87 * (s.dn[743][5] * ddt_scale));let eq31_e1199_d_n6: f64 = (p.p87 * (s.dn[743][6] * ddt_scale));let eq31_e1199_d_n7: f64 = (p.p87 * (s.dn[743][7] * ddt_scale));let eq31_e1199_d_n8: f64 = (p.p87 * (s.dn[743][8] * ddt_scale));let eq31_e1199_d_n9: f64 = (p.p87 * (s.dn[743][9] * ddt_scale));let eq31_e1199_d_n10: f64 = (p.p87 * (s.dn[743][10] * ddt_scale));let eq31_e1199_d_n11: f64 = (p.p87 * (s.dn[743][11] * ddt_scale));let eq31_e1199_d_n12: f64 = (p.p87 * (s.dn[743][12] * ddt_scale));let eq31_e1199_d_n13: f64 = (p.p87 * (s.dn[743][13] * ddt_scale));let eq31_e1199_d_n14: f64 = (p.p87 * (s.dn[743][14] * ddt_scale));let eq31_e1199_d_n15: f64 = (p.p87 * (s.dn[743][15] * ddt_scale));let eq31_e1199_d_n16: f64 = (p.p87 * (s.dn[743][16] * ddt_scale));let eq31_e1199_d_n17: f64 = (p.p87 * (s.dn[743][17] * ddt_scale));let eq31_e1199_d_n18: f64 = (p.p87 * (s.dn[743][18] * ddt_scale));let eq31_e1199_d_b0: f64 = (p.p87 * (s.db[743][0] * ddt_scale));let eq31_e1199_d_b1: f64 = (p.p87 * (s.db[743][1] * ddt_scale));let eq31_e1199_d_b2: f64 = (p.p87 * (s.db[743][2] * ddt_scale));let eq31_e1199_d_b3: f64 = (p.p87 * (s.db[743][3] * ddt_scale));let eq31_e1199_d_b4: f64 = (p.p87 * (s.db[743][4] * ddt_scale));let eq31_e1199_d_b5: f64 = (p.p87 * (s.db[743][5] * ddt_scale));let eq31_e1199_d_b6: f64 = (p.p87 * (s.db[743][6] * ddt_scale));let eq31_e1199_d_b7: f64 = (p.p87 * (s.db[743][7] * ddt_scale));let eq31_e1199_d_b8: f64 = (p.p87 * (s.db[743][8] * ddt_scale));let eq31_e1199_d_b9: f64 = (p.p87 * (s.db[743][9] * ddt_scale));let eq31_e1199_d_b10: f64 = (p.p87 * (s.db[743][10] * ddt_scale));let eq31_e1199_d_b11: f64 = (p.p87 * (s.db[743][11] * ddt_scale));let eq31_e1199_d_b12: f64 = (p.p87 * (s.db[743][12] * ddt_scale));let eq31_value: f64 = eq31_e1199;let eq31_node_derivatives: [f64; 19] = [eq31_e1199_d_n0, eq31_e1199_d_n1, eq31_e1199_d_n2, eq31_e1199_d_n3, eq31_e1199_d_n4, eq31_e1199_d_n5, eq31_e1199_d_n6, eq31_e1199_d_n7, eq31_e1199_d_n8, eq31_e1199_d_n9, eq31_e1199_d_n10, eq31_e1199_d_n11, eq31_e1199_d_n12, eq31_e1199_d_n13, eq31_e1199_d_n14, eq31_e1199_d_n15, eq31_e1199_d_n16, eq31_e1199_d_n17, eq31_e1199_d_n18];let eq31_branch_derivatives: [f64; 13] = [eq31_e1199_d_b0, eq31_e1199_d_b1, eq31_e1199_d_b2, eq31_e1199_d_b3, eq31_e1199_d_b4, eq31_e1199_d_b5, eq31_e1199_d_b6, eq31_e1199_d_b7, eq31_e1199_d_b8, eq31_e1199_d_b9, eq31_e1199_d_b10, eq31_e1199_d_b11, eq31_e1199_d_b12];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );let eq32_e1202: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[742]);let eq32_e1203: f64 = (p.p87 * eq32_e1202);let eq32_e1203_d_n0: f64 = (p.p87 * (s.dn[742][0] * ddt_scale));let eq32_e1203_d_n1: f64 = (p.p87 * (s.dn[742][1] * ddt_scale));let eq32_e1203_d_n2: f64 = (p.p87 * (s.dn[742][2] * ddt_scale));let eq32_e1203_d_n3: f64 = (p.p87 * (s.dn[742][3] * ddt_scale));let eq32_e1203_d_n4: f64 = (p.p87 * (s.dn[742][4] * ddt_scale));let eq32_e1203_d_n5: f64 = (p.p87 * (s.dn[742][5] * ddt_scale));let eq32_e1203_d_n6: f64 = (p.p87 * (s.dn[742][6] * ddt_scale));let eq32_e1203_d_n7: f64 = (p.p87 * (s.dn[742][7] * ddt_scale));let eq32_e1203_d_n8: f64 = (p.p87 * (s.dn[742][8] * ddt_scale));let eq32_e1203_d_n9: f64 = (p.p87 * (s.dn[742][9] * ddt_scale));let eq32_e1203_d_n10: f64 = (p.p87 * (s.dn[742][10] * ddt_scale));let eq32_e1203_d_n11: f64 = (p.p87 * (s.dn[742][11] * ddt_scale));let eq32_e1203_d_n12: f64 = (p.p87 * (s.dn[742][12] * ddt_scale));let eq32_e1203_d_n13: f64 = (p.p87 * (s.dn[742][13] * ddt_scale));let eq32_e1203_d_n14: f64 = (p.p87 * (s.dn[742][14] * ddt_scale));let eq32_e1203_d_n15: f64 = (p.p87 * (s.dn[742][15] * ddt_scale));let eq32_e1203_d_n16: f64 = (p.p87 * (s.dn[742][16] * ddt_scale));let eq32_e1203_d_n17: f64 = (p.p87 * (s.dn[742][17] * ddt_scale));let eq32_e1203_d_n18: f64 = (p.p87 * (s.dn[742][18] * ddt_scale));let eq32_e1203_d_b0: f64 = (p.p87 * (s.db[742][0] * ddt_scale));let eq32_e1203_d_b1: f64 = (p.p87 * (s.db[742][1] * ddt_scale));let eq32_e1203_d_b2: f64 = (p.p87 * (s.db[742][2] * ddt_scale));let eq32_e1203_d_b3: f64 = (p.p87 * (s.db[742][3] * ddt_scale));let eq32_e1203_d_b4: f64 = (p.p87 * (s.db[742][4] * ddt_scale));let eq32_e1203_d_b5: f64 = (p.p87 * (s.db[742][5] * ddt_scale));let eq32_e1203_d_b6: f64 = (p.p87 * (s.db[742][6] * ddt_scale));let eq32_e1203_d_b7: f64 = (p.p87 * (s.db[742][7] * ddt_scale));let eq32_e1203_d_b8: f64 = (p.p87 * (s.db[742][8] * ddt_scale));let eq32_e1203_d_b9: f64 = (p.p87 * (s.db[742][9] * ddt_scale));let eq32_e1203_d_b10: f64 = (p.p87 * (s.db[742][10] * ddt_scale));let eq32_e1203_d_b11: f64 = (p.p87 * (s.db[742][11] * ddt_scale));let eq32_e1203_d_b12: f64 = (p.p87 * (s.db[742][12] * ddt_scale));let eq32_value: f64 = eq32_e1203;let eq32_node_derivatives: [f64; 19] = [eq32_e1203_d_n0, eq32_e1203_d_n1, eq32_e1203_d_n2, eq32_e1203_d_n3, eq32_e1203_d_n4, eq32_e1203_d_n5, eq32_e1203_d_n6, eq32_e1203_d_n7, eq32_e1203_d_n8, eq32_e1203_d_n9, eq32_e1203_d_n10, eq32_e1203_d_n11, eq32_e1203_d_n12, eq32_e1203_d_n13, eq32_e1203_d_n14, eq32_e1203_d_n15, eq32_e1203_d_n16, eq32_e1203_d_n17, eq32_e1203_d_n18];let eq32_branch_derivatives: [f64; 13] = [eq32_e1203_d_b0, eq32_e1203_d_b1, eq32_e1203_d_b2, eq32_e1203_d_b3, eq32_e1203_d_b4, eq32_e1203_d_b5, eq32_e1203_d_b6, eq32_e1203_d_b7, eq32_e1203_d_b8, eq32_e1203_d_b9, eq32_e1203_d_b10, eq32_e1203_d_b11, eq32_e1203_d_b12];
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
        let eq33_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[744]);let eq33_e1207: f64 = (p.p87 * eq33_e1206);let eq33_e1207_d_n0: f64 = (p.p87 * (s.dn[744][0] * ddt_scale));let eq33_e1207_d_n1: f64 = (p.p87 * (s.dn[744][1] * ddt_scale));let eq33_e1207_d_n2: f64 = (p.p87 * (s.dn[744][2] * ddt_scale));let eq33_e1207_d_n3: f64 = (p.p87 * (s.dn[744][3] * ddt_scale));let eq33_e1207_d_n4: f64 = (p.p87 * (s.dn[744][4] * ddt_scale));let eq33_e1207_d_n5: f64 = (p.p87 * (s.dn[744][5] * ddt_scale));let eq33_e1207_d_n6: f64 = (p.p87 * (s.dn[744][6] * ddt_scale));let eq33_e1207_d_n7: f64 = (p.p87 * (s.dn[744][7] * ddt_scale));let eq33_e1207_d_n8: f64 = (p.p87 * (s.dn[744][8] * ddt_scale));let eq33_e1207_d_n9: f64 = (p.p87 * (s.dn[744][9] * ddt_scale));let eq33_e1207_d_n10: f64 = (p.p87 * (s.dn[744][10] * ddt_scale));let eq33_e1207_d_n11: f64 = (p.p87 * (s.dn[744][11] * ddt_scale));let eq33_e1207_d_n12: f64 = (p.p87 * (s.dn[744][12] * ddt_scale));let eq33_e1207_d_n13: f64 = (p.p87 * (s.dn[744][13] * ddt_scale));let eq33_e1207_d_n14: f64 = (p.p87 * (s.dn[744][14] * ddt_scale));let eq33_e1207_d_n15: f64 = (p.p87 * (s.dn[744][15] * ddt_scale));let eq33_e1207_d_n16: f64 = (p.p87 * (s.dn[744][16] * ddt_scale));let eq33_e1207_d_n17: f64 = (p.p87 * (s.dn[744][17] * ddt_scale));let eq33_e1207_d_n18: f64 = (p.p87 * (s.dn[744][18] * ddt_scale));let eq33_e1207_d_b0: f64 = (p.p87 * (s.db[744][0] * ddt_scale));let eq33_e1207_d_b1: f64 = (p.p87 * (s.db[744][1] * ddt_scale));let eq33_e1207_d_b2: f64 = (p.p87 * (s.db[744][2] * ddt_scale));let eq33_e1207_d_b3: f64 = (p.p87 * (s.db[744][3] * ddt_scale));let eq33_e1207_d_b4: f64 = (p.p87 * (s.db[744][4] * ddt_scale));let eq33_e1207_d_b5: f64 = (p.p87 * (s.db[744][5] * ddt_scale));let eq33_e1207_d_b6: f64 = (p.p87 * (s.db[744][6] * ddt_scale));let eq33_e1207_d_b7: f64 = (p.p87 * (s.db[744][7] * ddt_scale));let eq33_e1207_d_b8: f64 = (p.p87 * (s.db[744][8] * ddt_scale));let eq33_e1207_d_b9: f64 = (p.p87 * (s.db[744][9] * ddt_scale));let eq33_e1207_d_b10: f64 = (p.p87 * (s.db[744][10] * ddt_scale));let eq33_e1207_d_b11: f64 = (p.p87 * (s.db[744][11] * ddt_scale));let eq33_e1207_d_b12: f64 = (p.p87 * (s.db[744][12] * ddt_scale));let eq33_value: f64 = eq33_e1207;let eq33_node_derivatives: [f64; 19] = [eq33_e1207_d_n0, eq33_e1207_d_n1, eq33_e1207_d_n2, eq33_e1207_d_n3, eq33_e1207_d_n4, eq33_e1207_d_n5, eq33_e1207_d_n6, eq33_e1207_d_n7, eq33_e1207_d_n8, eq33_e1207_d_n9, eq33_e1207_d_n10, eq33_e1207_d_n11, eq33_e1207_d_n12, eq33_e1207_d_n13, eq33_e1207_d_n14, eq33_e1207_d_n15, eq33_e1207_d_n16, eq33_e1207_d_n17, eq33_e1207_d_n18];let eq33_branch_derivatives: [f64; 13] = [eq33_e1207_d_b0, eq33_e1207_d_b1, eq33_e1207_d_b2, eq33_e1207_d_b3, eq33_e1207_d_b4, eq33_e1207_d_b5, eq33_e1207_d_b6, eq33_e1207_d_b7, eq33_e1207_d_b8, eq33_e1207_d_b9, eq33_e1207_d_b10, eq33_e1207_d_b11, eq33_e1207_d_b12];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );let eq34_e1209: f64 = (-p.p87);let eq34_e1211: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[299]);let eq34_e1212: f64 = (eq34_e1209 * eq34_e1211);let eq34_e1212_d_n0: f64 = (eq34_e1209 * (s.dn[299][0] * ddt_scale));let eq34_e1212_d_n1: f64 = (eq34_e1209 * (s.dn[299][1] * ddt_scale));let eq34_e1212_d_n2: f64 = (eq34_e1209 * (s.dn[299][2] * ddt_scale));let eq34_e1212_d_n3: f64 = (eq34_e1209 * (s.dn[299][3] * ddt_scale));let eq34_e1212_d_n4: f64 = (eq34_e1209 * (s.dn[299][4] * ddt_scale));let eq34_e1212_d_n5: f64 = (eq34_e1209 * (s.dn[299][5] * ddt_scale));let eq34_e1212_d_n6: f64 = (eq34_e1209 * (s.dn[299][6] * ddt_scale));let eq34_e1212_d_n7: f64 = (eq34_e1209 * (s.dn[299][7] * ddt_scale));let eq34_e1212_d_n8: f64 = (eq34_e1209 * (s.dn[299][8] * ddt_scale));let eq34_e1212_d_n9: f64 = (eq34_e1209 * (s.dn[299][9] * ddt_scale));let eq34_e1212_d_n10: f64 = (eq34_e1209 * (s.dn[299][10] * ddt_scale));let eq34_e1212_d_n11: f64 = (eq34_e1209 * (s.dn[299][11] * ddt_scale));let eq34_e1212_d_n12: f64 = (eq34_e1209 * (s.dn[299][12] * ddt_scale));let eq34_e1212_d_n13: f64 = (eq34_e1209 * (s.dn[299][13] * ddt_scale));let eq34_e1212_d_n14: f64 = (eq34_e1209 * (s.dn[299][14] * ddt_scale));let eq34_e1212_d_n15: f64 = (eq34_e1209 * (s.dn[299][15] * ddt_scale));let eq34_e1212_d_n16: f64 = (eq34_e1209 * (s.dn[299][16] * ddt_scale));let eq34_e1212_d_n17: f64 = (eq34_e1209 * (s.dn[299][17] * ddt_scale));let eq34_e1212_d_n18: f64 = (eq34_e1209 * (s.dn[299][18] * ddt_scale));let eq34_e1212_d_b0: f64 = (eq34_e1209 * (s.db[299][0] * ddt_scale));let eq34_e1212_d_b1: f64 = (eq34_e1209 * (s.db[299][1] * ddt_scale));let eq34_e1212_d_b2: f64 = (eq34_e1209 * (s.db[299][2] * ddt_scale));let eq34_e1212_d_b3: f64 = (eq34_e1209 * (s.db[299][3] * ddt_scale));let eq34_e1212_d_b4: f64 = (eq34_e1209 * (s.db[299][4] * ddt_scale));let eq34_e1212_d_b5: f64 = (eq34_e1209 * (s.db[299][5] * ddt_scale));let eq34_e1212_d_b6: f64 = (eq34_e1209 * (s.db[299][6] * ddt_scale));let eq34_e1212_d_b7: f64 = (eq34_e1209 * (s.db[299][7] * ddt_scale));let eq34_e1212_d_b8: f64 = (eq34_e1209 * (s.db[299][8] * ddt_scale));let eq34_e1212_d_b9: f64 = (eq34_e1209 * (s.db[299][9] * ddt_scale));let eq34_e1212_d_b10: f64 = (eq34_e1209 * (s.db[299][10] * ddt_scale));let eq34_e1212_d_b11: f64 = (eq34_e1209 * (s.db[299][11] * ddt_scale));let eq34_e1212_d_b12: f64 = (eq34_e1209 * (s.db[299][12] * ddt_scale));let eq34_value: f64 = eq34_e1212;let eq34_node_derivatives: [f64; 19] = [eq34_e1212_d_n0, eq34_e1212_d_n1, eq34_e1212_d_n2, eq34_e1212_d_n3, eq34_e1212_d_n4, eq34_e1212_d_n5, eq34_e1212_d_n6, eq34_e1212_d_n7, eq34_e1212_d_n8, eq34_e1212_d_n9, eq34_e1212_d_n10, eq34_e1212_d_n11, eq34_e1212_d_n12, eq34_e1212_d_n13, eq34_e1212_d_n14, eq34_e1212_d_n15, eq34_e1212_d_n16, eq34_e1212_d_n17, eq34_e1212_d_n18];let eq34_branch_derivatives: [f64; 13] = [eq34_e1212_d_b0, eq34_e1212_d_b1, eq34_e1212_d_b2, eq34_e1212_d_b3, eq34_e1212_d_b4, eq34_e1212_d_b5, eq34_e1212_d_b6, eq34_e1212_d_b7, eq34_e1212_d_b8, eq34_e1212_d_b9, eq34_e1212_d_b10, eq34_e1212_d_b11, eq34_e1212_d_b12];
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
        let nv15 = ctx.node_voltage(nodes[15]);let eq35_e1214: f64 = (-p.p87);let eq35_e1216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[301]);let eq35_e1217: f64 = (eq35_e1214 * eq35_e1216);let eq35_e1217_d_n0: f64 = (eq35_e1214 * (s.dn[301][0] * ddt_scale));let eq35_e1217_d_n1: f64 = (eq35_e1214 * (s.dn[301][1] * ddt_scale));let eq35_e1217_d_n2: f64 = (eq35_e1214 * (s.dn[301][2] * ddt_scale));let eq35_e1217_d_n3: f64 = (eq35_e1214 * (s.dn[301][3] * ddt_scale));let eq35_e1217_d_n4: f64 = (eq35_e1214 * (s.dn[301][4] * ddt_scale));let eq35_e1217_d_n5: f64 = (eq35_e1214 * (s.dn[301][5] * ddt_scale));let eq35_e1217_d_n6: f64 = (eq35_e1214 * (s.dn[301][6] * ddt_scale));let eq35_e1217_d_n7: f64 = (eq35_e1214 * (s.dn[301][7] * ddt_scale));let eq35_e1217_d_n8: f64 = (eq35_e1214 * (s.dn[301][8] * ddt_scale));let eq35_e1217_d_n9: f64 = (eq35_e1214 * (s.dn[301][9] * ddt_scale));let eq35_e1217_d_n10: f64 = (eq35_e1214 * (s.dn[301][10] * ddt_scale));let eq35_e1217_d_n11: f64 = (eq35_e1214 * (s.dn[301][11] * ddt_scale));let eq35_e1217_d_n12: f64 = (eq35_e1214 * (s.dn[301][12] * ddt_scale));let eq35_e1217_d_n13: f64 = (eq35_e1214 * (s.dn[301][13] * ddt_scale));let eq35_e1217_d_n14: f64 = (eq35_e1214 * (s.dn[301][14] * ddt_scale));let eq35_e1217_d_n15: f64 = (eq35_e1214 * (s.dn[301][15] * ddt_scale));let eq35_e1217_d_n16: f64 = (eq35_e1214 * (s.dn[301][16] * ddt_scale));let eq35_e1217_d_n17: f64 = (eq35_e1214 * (s.dn[301][17] * ddt_scale));let eq35_e1217_d_n18: f64 = (eq35_e1214 * (s.dn[301][18] * ddt_scale));let eq35_e1217_d_b0: f64 = (eq35_e1214 * (s.db[301][0] * ddt_scale));let eq35_e1217_d_b1: f64 = (eq35_e1214 * (s.db[301][1] * ddt_scale));let eq35_e1217_d_b2: f64 = (eq35_e1214 * (s.db[301][2] * ddt_scale));let eq35_e1217_d_b3: f64 = (eq35_e1214 * (s.db[301][3] * ddt_scale));let eq35_e1217_d_b4: f64 = (eq35_e1214 * (s.db[301][4] * ddt_scale));let eq35_e1217_d_b5: f64 = (eq35_e1214 * (s.db[301][5] * ddt_scale));let eq35_e1217_d_b6: f64 = (eq35_e1214 * (s.db[301][6] * ddt_scale));let eq35_e1217_d_b7: f64 = (eq35_e1214 * (s.db[301][7] * ddt_scale));let eq35_e1217_d_b8: f64 = (eq35_e1214 * (s.db[301][8] * ddt_scale));let eq35_e1217_d_b9: f64 = (eq35_e1214 * (s.db[301][9] * ddt_scale));let eq35_e1217_d_b10: f64 = (eq35_e1214 * (s.db[301][10] * ddt_scale));let eq35_e1217_d_b11: f64 = (eq35_e1214 * (s.db[301][11] * ddt_scale));let eq35_e1217_d_b12: f64 = (eq35_e1214 * (s.db[301][12] * ddt_scale));let eq35_value: f64 = eq35_e1217;let eq35_node_derivatives: [f64; 19] = [eq35_e1217_d_n0, eq35_e1217_d_n1, eq35_e1217_d_n2, eq35_e1217_d_n3, eq35_e1217_d_n4, eq35_e1217_d_n5, eq35_e1217_d_n6, eq35_e1217_d_n7, eq35_e1217_d_n8, eq35_e1217_d_n9, eq35_e1217_d_n10, eq35_e1217_d_n11, eq35_e1217_d_n12, eq35_e1217_d_n13, eq35_e1217_d_n14, eq35_e1217_d_n15, eq35_e1217_d_n16, eq35_e1217_d_n17, eq35_e1217_d_n18];let eq35_branch_derivatives: [f64; 13] = [eq35_e1217_d_b0, eq35_e1217_d_b1, eq35_e1217_d_b2, eq35_e1217_d_b3, eq35_e1217_d_b4, eq35_e1217_d_b5, eq35_e1217_d_b6, eq35_e1217_d_b7, eq35_e1217_d_b8, eq35_e1217_d_b9, eq35_e1217_d_b10, eq35_e1217_d_b11, eq35_e1217_d_b12];
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
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_17(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let nv11 = ctx.node_voltage(nodes[11]);
        let (eq48_e1297, eq48_e1297_d_n0, eq48_e1297_d_n1, eq48_e1297_d_n2, eq48_e1297_d_n3, eq48_e1297_d_n4, eq48_e1297_d_n5, eq48_e1297_d_n6, eq48_e1297_d_n7, eq48_e1297_d_n8, eq48_e1297_d_n9, eq48_e1297_d_n10, eq48_e1297_d_n11, eq48_e1297_d_n12, eq48_e1297_d_n13, eq48_e1297_d_n14, eq48_e1297_d_n15, eq48_e1297_d_n16, eq48_e1297_d_n17, eq48_e1297_d_n18, eq48_e1297_d_b0, eq48_e1297_d_b1, eq48_e1297_d_b2, eq48_e1297_d_b3, eq48_e1297_d_b4, eq48_e1297_d_b5, eq48_e1297_d_b6, eq48_e1297_d_b7, eq48_e1297_d_b8, eq48_e1297_d_b9, eq48_e1297_d_b10, eq48_e1297_d_b11, eq48_e1297_d_b12,) = {
    if s.b[3412] {
        let eq48_e1295: f64 = (s.v[643] * (nv1 - nv7));let eq48_e1295_d_n0: f64 = (s.dn[643][0] * (nv1 - nv7));let eq48_e1295_d_n1: f64 = ((s.dn[643][1] * (nv1 - nv7)) + s.v[643]);let eq48_e1295_d_n2: f64 = (s.dn[643][2] * (nv1 - nv7));let eq48_e1295_d_n3: f64 = (s.dn[643][3] * (nv1 - nv7));let eq48_e1295_d_n4: f64 = (s.dn[643][4] * (nv1 - nv7));let eq48_e1295_d_n5: f64 = (s.dn[643][5] * (nv1 - nv7));let eq48_e1295_d_n6: f64 = (s.dn[643][6] * (nv1 - nv7));let eq48_e1295_d_n7: f64 = ((s.dn[643][7] * (nv1 - nv7)) + (-s.v[643]));let eq48_e1295_d_n8: f64 = (s.dn[643][8] * (nv1 - nv7));let eq48_e1295_d_n9: f64 = (s.dn[643][9] * (nv1 - nv7));let eq48_e1295_d_n10: f64 = (s.dn[643][10] * (nv1 - nv7));let eq48_e1295_d_n11: f64 = (s.dn[643][11] * (nv1 - nv7));let eq48_e1295_d_n12: f64 = (s.dn[643][12] * (nv1 - nv7));let eq48_e1295_d_n13: f64 = (s.dn[643][13] * (nv1 - nv7));let eq48_e1295_d_n14: f64 = (s.dn[643][14] * (nv1 - nv7));let eq48_e1295_d_n15: f64 = (s.dn[643][15] * (nv1 - nv7));let eq48_e1295_d_n16: f64 = (s.dn[643][16] * (nv1 - nv7));let eq48_e1295_d_n17: f64 = (s.dn[643][17] * (nv1 - nv7));let eq48_e1295_d_n18: f64 = (s.dn[643][18] * (nv1 - nv7));let eq48_e1295_d_b0: f64 = (s.db[643][0] * (nv1 - nv7));let eq48_e1295_d_b1: f64 = (s.db[643][1] * (nv1 - nv7));let eq48_e1295_d_b2: f64 = (s.db[643][2] * (nv1 - nv7));let eq48_e1295_d_b3: f64 = (s.db[643][3] * (nv1 - nv7));let eq48_e1295_d_b4: f64 = (s.db[643][4] * (nv1 - nv7));let eq48_e1295_d_b5: f64 = (s.db[643][5] * (nv1 - nv7));let eq48_e1295_d_b6: f64 = (s.db[643][6] * (nv1 - nv7));let eq48_e1295_d_b7: f64 = (s.db[643][7] * (nv1 - nv7));let eq48_e1295_d_b8: f64 = (s.db[643][8] * (nv1 - nv7));let eq48_e1295_d_b9: f64 = (s.db[643][9] * (nv1 - nv7));let eq48_e1295_d_b10: f64 = (s.db[643][10] * (nv1 - nv7));let eq48_e1295_d_b11: f64 = (s.db[643][11] * (nv1 - nv7));let eq48_e1295_d_b12: f64 = (s.db[643][12] * (nv1 - nv7));
        (eq48_e1295, eq48_e1295_d_n0, eq48_e1295_d_n1, eq48_e1295_d_n2, eq48_e1295_d_n3, eq48_e1295_d_n4, eq48_e1295_d_n5, eq48_e1295_d_n6, eq48_e1295_d_n7, eq48_e1295_d_n8, eq48_e1295_d_n9, eq48_e1295_d_n10, eq48_e1295_d_n11, eq48_e1295_d_n12, eq48_e1295_d_n13, eq48_e1295_d_n14, eq48_e1295_d_n15, eq48_e1295_d_n16, eq48_e1295_d_n17, eq48_e1295_d_n18, eq48_e1295_d_b0, eq48_e1295_d_b1, eq48_e1295_d_b2, eq48_e1295_d_b3, eq48_e1295_d_b4, eq48_e1295_d_b5, eq48_e1295_d_b6, eq48_e1295_d_b7, eq48_e1295_d_b8, eq48_e1295_d_b9, eq48_e1295_d_b10, eq48_e1295_d_b11, eq48_e1295_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e1297;let eq48_node_derivatives: [f64; 19] = [eq48_e1297_d_n0, eq48_e1297_d_n1, eq48_e1297_d_n2, eq48_e1297_d_n3, eq48_e1297_d_n4, eq48_e1297_d_n5, eq48_e1297_d_n6, eq48_e1297_d_n7, eq48_e1297_d_n8, eq48_e1297_d_n9, eq48_e1297_d_n10, eq48_e1297_d_n11, eq48_e1297_d_n12, eq48_e1297_d_n13, eq48_e1297_d_n14, eq48_e1297_d_n15, eq48_e1297_d_n16, eq48_e1297_d_n17, eq48_e1297_d_n18];let eq48_branch_derivatives: [f64; 13] = [eq48_e1297_d_b0, eq48_e1297_d_b1, eq48_e1297_d_b2, eq48_e1297_d_b3, eq48_e1297_d_b4, eq48_e1297_d_b5, eq48_e1297_d_b6, eq48_e1297_d_b7, eq48_e1297_d_b8, eq48_e1297_d_b9, eq48_e1297_d_b10, eq48_e1297_d_b11, eq48_e1297_d_b12];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e1302,) = {
    if (!s.b[3412]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1302;
        stamper.stamp_potential_const_local(
            6,
            eq49_value,
        );
        let (eq50_e1308, eq50_e1308_d_n0, eq50_e1308_d_n1, eq50_e1308_d_n2, eq50_e1308_d_n3, eq50_e1308_d_n4, eq50_e1308_d_n5, eq50_e1308_d_n6, eq50_e1308_d_n7, eq50_e1308_d_n8, eq50_e1308_d_n9, eq50_e1308_d_n10, eq50_e1308_d_n11, eq50_e1308_d_n12, eq50_e1308_d_n13, eq50_e1308_d_n14, eq50_e1308_d_n15, eq50_e1308_d_n16, eq50_e1308_d_n17, eq50_e1308_d_n18, eq50_e1308_d_b0, eq50_e1308_d_b1, eq50_e1308_d_b2, eq50_e1308_d_b3, eq50_e1308_d_b4, eq50_e1308_d_b5, eq50_e1308_d_b6, eq50_e1308_d_b7, eq50_e1308_d_b8, eq50_e1308_d_b9, eq50_e1308_d_b10, eq50_e1308_d_b11, eq50_e1308_d_b12,) = {
    if (p.p52 != 0.0) {
        let eq50_e1306: f64 = (s.v[656] * (nv11 - nv9));let eq50_e1306_d_n0: f64 = (s.dn[656][0] * (nv11 - nv9));let eq50_e1306_d_n1: f64 = (s.dn[656][1] * (nv11 - nv9));let eq50_e1306_d_n2: f64 = (s.dn[656][2] * (nv11 - nv9));let eq50_e1306_d_n3: f64 = (s.dn[656][3] * (nv11 - nv9));let eq50_e1306_d_n4: f64 = (s.dn[656][4] * (nv11 - nv9));let eq50_e1306_d_n5: f64 = (s.dn[656][5] * (nv11 - nv9));let eq50_e1306_d_n6: f64 = (s.dn[656][6] * (nv11 - nv9));let eq50_e1306_d_n7: f64 = (s.dn[656][7] * (nv11 - nv9));let eq50_e1306_d_n8: f64 = (s.dn[656][8] * (nv11 - nv9));let eq50_e1306_d_n9: f64 = ((s.dn[656][9] * (nv11 - nv9)) + (-s.v[656]));let eq50_e1306_d_n10: f64 = (s.dn[656][10] * (nv11 - nv9));let eq50_e1306_d_n11: f64 = ((s.dn[656][11] * (nv11 - nv9)) + s.v[656]);let eq50_e1306_d_n12: f64 = (s.dn[656][12] * (nv11 - nv9));let eq50_e1306_d_n13: f64 = (s.dn[656][13] * (nv11 - nv9));let eq50_e1306_d_n14: f64 = (s.dn[656][14] * (nv11 - nv9));let eq50_e1306_d_n15: f64 = (s.dn[656][15] * (nv11 - nv9));let eq50_e1306_d_n16: f64 = (s.dn[656][16] * (nv11 - nv9));let eq50_e1306_d_n17: f64 = (s.dn[656][17] * (nv11 - nv9));let eq50_e1306_d_n18: f64 = (s.dn[656][18] * (nv11 - nv9));let eq50_e1306_d_b0: f64 = (s.db[656][0] * (nv11 - nv9));let eq50_e1306_d_b1: f64 = (s.db[656][1] * (nv11 - nv9));let eq50_e1306_d_b2: f64 = (s.db[656][2] * (nv11 - nv9));let eq50_e1306_d_b3: f64 = (s.db[656][3] * (nv11 - nv9));let eq50_e1306_d_b4: f64 = (s.db[656][4] * (nv11 - nv9));let eq50_e1306_d_b5: f64 = (s.db[656][5] * (nv11 - nv9));let eq50_e1306_d_b6: f64 = (s.db[656][6] * (nv11 - nv9));let eq50_e1306_d_b7: f64 = (s.db[656][7] * (nv11 - nv9));let eq50_e1306_d_b8: f64 = (s.db[656][8] * (nv11 - nv9));let eq50_e1306_d_b9: f64 = (s.db[656][9] * (nv11 - nv9));let eq50_e1306_d_b10: f64 = (s.db[656][10] * (nv11 - nv9));let eq50_e1306_d_b11: f64 = (s.db[656][11] * (nv11 - nv9));let eq50_e1306_d_b12: f64 = (s.db[656][12] * (nv11 - nv9));
        (eq50_e1306, eq50_e1306_d_n0, eq50_e1306_d_n1, eq50_e1306_d_n2, eq50_e1306_d_n3, eq50_e1306_d_n4, eq50_e1306_d_n5, eq50_e1306_d_n6, eq50_e1306_d_n7, eq50_e1306_d_n8, eq50_e1306_d_n9, eq50_e1306_d_n10, eq50_e1306_d_n11, eq50_e1306_d_n12, eq50_e1306_d_n13, eq50_e1306_d_n14, eq50_e1306_d_n15, eq50_e1306_d_n16, eq50_e1306_d_n17, eq50_e1306_d_n18, eq50_e1306_d_b0, eq50_e1306_d_b1, eq50_e1306_d_b2, eq50_e1306_d_b3, eq50_e1306_d_b4, eq50_e1306_d_b5, eq50_e1306_d_b6, eq50_e1306_d_b7, eq50_e1306_d_b8, eq50_e1306_d_b9, eq50_e1306_d_b10, eq50_e1306_d_b11, eq50_e1306_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1308;let eq50_node_derivatives: [f64; 19] = [eq50_e1308_d_n0, eq50_e1308_d_n1, eq50_e1308_d_n2, eq50_e1308_d_n3, eq50_e1308_d_n4, eq50_e1308_d_n5, eq50_e1308_d_n6, eq50_e1308_d_n7, eq50_e1308_d_n8, eq50_e1308_d_n9, eq50_e1308_d_n10, eq50_e1308_d_n11, eq50_e1308_d_n12, eq50_e1308_d_n13, eq50_e1308_d_n14, eq50_e1308_d_n15, eq50_e1308_d_n16, eq50_e1308_d_n17, eq50_e1308_d_n18];let eq50_branch_derivatives: [f64; 13] = [eq50_e1308_d_b0, eq50_e1308_d_b1, eq50_e1308_d_b2, eq50_e1308_d_b3, eq50_e1308_d_b4, eq50_e1308_d_b5, eq50_e1308_d_b6, eq50_e1308_d_b7, eq50_e1308_d_b8, eq50_e1308_d_b9, eq50_e1308_d_b10, eq50_e1308_d_b11, eq50_e1308_d_b12];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(9),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
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
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq51_e1314, eq51_e1314_d_n0, eq51_e1314_d_n1, eq51_e1314_d_n2, eq51_e1314_d_n3, eq51_e1314_d_n4, eq51_e1314_d_n5, eq51_e1314_d_n6, eq51_e1314_d_n7, eq51_e1314_d_n8, eq51_e1314_d_n9, eq51_e1314_d_n10, eq51_e1314_d_n11, eq51_e1314_d_n12, eq51_e1314_d_n13, eq51_e1314_d_n14, eq51_e1314_d_n15, eq51_e1314_d_n16, eq51_e1314_d_n17, eq51_e1314_d_n18, eq51_e1314_d_b0, eq51_e1314_d_b1, eq51_e1314_d_b2, eq51_e1314_d_b3, eq51_e1314_d_b4, eq51_e1314_d_b5, eq51_e1314_d_b6, eq51_e1314_d_b7, eq51_e1314_d_b8, eq51_e1314_d_b9, eq51_e1314_d_b10, eq51_e1314_d_b11, eq51_e1314_d_b12,) = {
    if (p.p52 != 0.0) {
        let eq51_e1312: f64 = (s.v[657] * (nv10 - nv9));let eq51_e1312_d_n0: f64 = (s.dn[657][0] * (nv10 - nv9));let eq51_e1312_d_n1: f64 = (s.dn[657][1] * (nv10 - nv9));let eq51_e1312_d_n2: f64 = (s.dn[657][2] * (nv10 - nv9));let eq51_e1312_d_n3: f64 = (s.dn[657][3] * (nv10 - nv9));let eq51_e1312_d_n4: f64 = (s.dn[657][4] * (nv10 - nv9));let eq51_e1312_d_n5: f64 = (s.dn[657][5] * (nv10 - nv9));let eq51_e1312_d_n6: f64 = (s.dn[657][6] * (nv10 - nv9));let eq51_e1312_d_n7: f64 = (s.dn[657][7] * (nv10 - nv9));let eq51_e1312_d_n8: f64 = (s.dn[657][8] * (nv10 - nv9));let eq51_e1312_d_n9: f64 = ((s.dn[657][9] * (nv10 - nv9)) + (-s.v[657]));let eq51_e1312_d_n10: f64 = ((s.dn[657][10] * (nv10 - nv9)) + s.v[657]);let eq51_e1312_d_n11: f64 = (s.dn[657][11] * (nv10 - nv9));let eq51_e1312_d_n12: f64 = (s.dn[657][12] * (nv10 - nv9));let eq51_e1312_d_n13: f64 = (s.dn[657][13] * (nv10 - nv9));let eq51_e1312_d_n14: f64 = (s.dn[657][14] * (nv10 - nv9));let eq51_e1312_d_n15: f64 = (s.dn[657][15] * (nv10 - nv9));let eq51_e1312_d_n16: f64 = (s.dn[657][16] * (nv10 - nv9));let eq51_e1312_d_n17: f64 = (s.dn[657][17] * (nv10 - nv9));let eq51_e1312_d_n18: f64 = (s.dn[657][18] * (nv10 - nv9));let eq51_e1312_d_b0: f64 = (s.db[657][0] * (nv10 - nv9));let eq51_e1312_d_b1: f64 = (s.db[657][1] * (nv10 - nv9));let eq51_e1312_d_b2: f64 = (s.db[657][2] * (nv10 - nv9));let eq51_e1312_d_b3: f64 = (s.db[657][3] * (nv10 - nv9));let eq51_e1312_d_b4: f64 = (s.db[657][4] * (nv10 - nv9));let eq51_e1312_d_b5: f64 = (s.db[657][5] * (nv10 - nv9));let eq51_e1312_d_b6: f64 = (s.db[657][6] * (nv10 - nv9));let eq51_e1312_d_b7: f64 = (s.db[657][7] * (nv10 - nv9));let eq51_e1312_d_b8: f64 = (s.db[657][8] * (nv10 - nv9));let eq51_e1312_d_b9: f64 = (s.db[657][9] * (nv10 - nv9));let eq51_e1312_d_b10: f64 = (s.db[657][10] * (nv10 - nv9));let eq51_e1312_d_b11: f64 = (s.db[657][11] * (nv10 - nv9));let eq51_e1312_d_b12: f64 = (s.db[657][12] * (nv10 - nv9));
        (eq51_e1312, eq51_e1312_d_n0, eq51_e1312_d_n1, eq51_e1312_d_n2, eq51_e1312_d_n3, eq51_e1312_d_n4, eq51_e1312_d_n5, eq51_e1312_d_n6, eq51_e1312_d_n7, eq51_e1312_d_n8, eq51_e1312_d_n9, eq51_e1312_d_n10, eq51_e1312_d_n11, eq51_e1312_d_n12, eq51_e1312_d_n13, eq51_e1312_d_n14, eq51_e1312_d_n15, eq51_e1312_d_n16, eq51_e1312_d_n17, eq51_e1312_d_n18, eq51_e1312_d_b0, eq51_e1312_d_b1, eq51_e1312_d_b2, eq51_e1312_d_b3, eq51_e1312_d_b4, eq51_e1312_d_b5, eq51_e1312_d_b6, eq51_e1312_d_b7, eq51_e1312_d_b8, eq51_e1312_d_b9, eq51_e1312_d_b10, eq51_e1312_d_b11, eq51_e1312_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1314;let eq51_node_derivatives: [f64; 19] = [eq51_e1314_d_n0, eq51_e1314_d_n1, eq51_e1314_d_n2, eq51_e1314_d_n3, eq51_e1314_d_n4, eq51_e1314_d_n5, eq51_e1314_d_n6, eq51_e1314_d_n7, eq51_e1314_d_n8, eq51_e1314_d_n9, eq51_e1314_d_n10, eq51_e1314_d_n11, eq51_e1314_d_n12, eq51_e1314_d_n13, eq51_e1314_d_n14, eq51_e1314_d_n15, eq51_e1314_d_n16, eq51_e1314_d_n17, eq51_e1314_d_n18];let eq51_branch_derivatives: [f64; 13] = [eq51_e1314_d_b0, eq51_e1314_d_b1, eq51_e1314_d_b2, eq51_e1314_d_b3, eq51_e1314_d_b4, eq51_e1314_d_b5, eq51_e1314_d_b6, eq51_e1314_d_b7, eq51_e1314_d_b8, eq51_e1314_d_b9, eq51_e1314_d_b10, eq51_e1314_d_b11, eq51_e1314_d_b12];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e1320, eq52_e1320_d_n0, eq52_e1320_d_n1, eq52_e1320_d_n2, eq52_e1320_d_n3, eq52_e1320_d_n4, eq52_e1320_d_n5, eq52_e1320_d_n6, eq52_e1320_d_n7, eq52_e1320_d_n8, eq52_e1320_d_n9, eq52_e1320_d_n10, eq52_e1320_d_n11, eq52_e1320_d_n12, eq52_e1320_d_n13, eq52_e1320_d_n14, eq52_e1320_d_n15, eq52_e1320_d_n16, eq52_e1320_d_n17, eq52_e1320_d_n18, eq52_e1320_d_b0, eq52_e1320_d_b1, eq52_e1320_d_b2, eq52_e1320_d_b3, eq52_e1320_d_b4, eq52_e1320_d_b5, eq52_e1320_d_b6, eq52_e1320_d_b7, eq52_e1320_d_b8, eq52_e1320_d_b9, eq52_e1320_d_b10, eq52_e1320_d_b11, eq52_e1320_d_b12,) = {
    if (p.p52 != 0.0) {
        let eq52_e1318: f64 = (s.v[655] * (nv3 - nv9));let eq52_e1318_d_n0: f64 = (s.dn[655][0] * (nv3 - nv9));let eq52_e1318_d_n1: f64 = (s.dn[655][1] * (nv3 - nv9));let eq52_e1318_d_n2: f64 = (s.dn[655][2] * (nv3 - nv9));let eq52_e1318_d_n3: f64 = ((s.dn[655][3] * (nv3 - nv9)) + s.v[655]);let eq52_e1318_d_n4: f64 = (s.dn[655][4] * (nv3 - nv9));let eq52_e1318_d_n5: f64 = (s.dn[655][5] * (nv3 - nv9));let eq52_e1318_d_n6: f64 = (s.dn[655][6] * (nv3 - nv9));let eq52_e1318_d_n7: f64 = (s.dn[655][7] * (nv3 - nv9));let eq52_e1318_d_n8: f64 = (s.dn[655][8] * (nv3 - nv9));let eq52_e1318_d_n9: f64 = ((s.dn[655][9] * (nv3 - nv9)) + (-s.v[655]));let eq52_e1318_d_n10: f64 = (s.dn[655][10] * (nv3 - nv9));let eq52_e1318_d_n11: f64 = (s.dn[655][11] * (nv3 - nv9));let eq52_e1318_d_n12: f64 = (s.dn[655][12] * (nv3 - nv9));let eq52_e1318_d_n13: f64 = (s.dn[655][13] * (nv3 - nv9));let eq52_e1318_d_n14: f64 = (s.dn[655][14] * (nv3 - nv9));let eq52_e1318_d_n15: f64 = (s.dn[655][15] * (nv3 - nv9));let eq52_e1318_d_n16: f64 = (s.dn[655][16] * (nv3 - nv9));let eq52_e1318_d_n17: f64 = (s.dn[655][17] * (nv3 - nv9));let eq52_e1318_d_n18: f64 = (s.dn[655][18] * (nv3 - nv9));let eq52_e1318_d_b0: f64 = (s.db[655][0] * (nv3 - nv9));let eq52_e1318_d_b1: f64 = (s.db[655][1] * (nv3 - nv9));let eq52_e1318_d_b2: f64 = (s.db[655][2] * (nv3 - nv9));let eq52_e1318_d_b3: f64 = (s.db[655][3] * (nv3 - nv9));let eq52_e1318_d_b4: f64 = (s.db[655][4] * (nv3 - nv9));let eq52_e1318_d_b5: f64 = (s.db[655][5] * (nv3 - nv9));let eq52_e1318_d_b6: f64 = (s.db[655][6] * (nv3 - nv9));let eq52_e1318_d_b7: f64 = (s.db[655][7] * (nv3 - nv9));let eq52_e1318_d_b8: f64 = (s.db[655][8] * (nv3 - nv9));let eq52_e1318_d_b9: f64 = (s.db[655][9] * (nv3 - nv9));let eq52_e1318_d_b10: f64 = (s.db[655][10] * (nv3 - nv9));let eq52_e1318_d_b11: f64 = (s.db[655][11] * (nv3 - nv9));let eq52_e1318_d_b12: f64 = (s.db[655][12] * (nv3 - nv9));
        (eq52_e1318, eq52_e1318_d_n0, eq52_e1318_d_n1, eq52_e1318_d_n2, eq52_e1318_d_n3, eq52_e1318_d_n4, eq52_e1318_d_n5, eq52_e1318_d_n6, eq52_e1318_d_n7, eq52_e1318_d_n8, eq52_e1318_d_n9, eq52_e1318_d_n10, eq52_e1318_d_n11, eq52_e1318_d_n12, eq52_e1318_d_n13, eq52_e1318_d_n14, eq52_e1318_d_n15, eq52_e1318_d_n16, eq52_e1318_d_n17, eq52_e1318_d_n18, eq52_e1318_d_b0, eq52_e1318_d_b1, eq52_e1318_d_b2, eq52_e1318_d_b3, eq52_e1318_d_b4, eq52_e1318_d_b5, eq52_e1318_d_b6, eq52_e1318_d_b7, eq52_e1318_d_b8, eq52_e1318_d_b9, eq52_e1318_d_b10, eq52_e1318_d_b11, eq52_e1318_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e1320;let eq52_node_derivatives: [f64; 19] = [eq52_e1320_d_n0, eq52_e1320_d_n1, eq52_e1320_d_n2, eq52_e1320_d_n3, eq52_e1320_d_n4, eq52_e1320_d_n5, eq52_e1320_d_n6, eq52_e1320_d_n7, eq52_e1320_d_n8, eq52_e1320_d_n9, eq52_e1320_d_n10, eq52_e1320_d_n11, eq52_e1320_d_n12, eq52_e1320_d_n13, eq52_e1320_d_n14, eq52_e1320_d_n15, eq52_e1320_d_n16, eq52_e1320_d_n17, eq52_e1320_d_n18];let eq52_branch_derivatives: [f64; 13] = [eq52_e1320_d_b0, eq52_e1320_d_b1, eq52_e1320_d_b2, eq52_e1320_d_b3, eq52_e1320_d_b4, eq52_e1320_d_b5, eq52_e1320_d_b6, eq52_e1320_d_b7, eq52_e1320_d_b8, eq52_e1320_d_b9, eq52_e1320_d_b10, eq52_e1320_d_b11, eq52_e1320_d_b12];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(9),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1325,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e1325;
        stamper.stamp_potential_const_local(
            7,
            eq53_value,
        );
        let (eq54_e1330,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1330;
        stamper.stamp_potential_const_local(
            8,
            eq54_value,
        );
        let (eq55_e1335,) = {
    if (p.p52 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e1335;
        stamper.stamp_potential_const_local(
            9,
            eq55_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_19(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq56_e1341, eq56_e1341_d_n0, eq56_e1341_d_n1, eq56_e1341_d_n2, eq56_e1341_d_n3, eq56_e1341_d_n4, eq56_e1341_d_n5, eq56_e1341_d_n6, eq56_e1341_d_n7, eq56_e1341_d_n8, eq56_e1341_d_n9, eq56_e1341_d_n10, eq56_e1341_d_n11, eq56_e1341_d_n12, eq56_e1341_d_n13, eq56_e1341_d_n14, eq56_e1341_d_n15, eq56_e1341_d_n16, eq56_e1341_d_n17, eq56_e1341_d_n18, eq56_e1341_d_b0, eq56_e1341_d_b1, eq56_e1341_d_b2, eq56_e1341_d_b3, eq56_e1341_d_b4, eq56_e1341_d_b5, eq56_e1341_d_b6, eq56_e1341_d_b7, eq56_e1341_d_b8, eq56_e1341_d_b9, eq56_e1341_d_b10, eq56_e1341_d_b11, eq56_e1341_d_b12,) = {
    if s.b[3413] {
        let eq56_e1339: f64 = ((nv5 - 0.0) * s.v[740]);let eq56_e1339_d_n0: f64 = ((nv5 - 0.0) * s.dn[740][0]);let eq56_e1339_d_n1: f64 = ((nv5 - 0.0) * s.dn[740][1]);let eq56_e1339_d_n2: f64 = ((nv5 - 0.0) * s.dn[740][2]);let eq56_e1339_d_n3: f64 = ((nv5 - 0.0) * s.dn[740][3]);let eq56_e1339_d_n4: f64 = ((nv5 - 0.0) * s.dn[740][4]);let eq56_e1339_d_n5: f64 = (s.v[740] + ((nv5 - 0.0) * s.dn[740][5]));let eq56_e1339_d_n6: f64 = ((nv5 - 0.0) * s.dn[740][6]);let eq56_e1339_d_n7: f64 = ((nv5 - 0.0) * s.dn[740][7]);let eq56_e1339_d_n8: f64 = ((nv5 - 0.0) * s.dn[740][8]);let eq56_e1339_d_n9: f64 = ((nv5 - 0.0) * s.dn[740][9]);let eq56_e1339_d_n10: f64 = ((nv5 - 0.0) * s.dn[740][10]);let eq56_e1339_d_n11: f64 = ((nv5 - 0.0) * s.dn[740][11]);let eq56_e1339_d_n12: f64 = ((nv5 - 0.0) * s.dn[740][12]);let eq56_e1339_d_n13: f64 = ((nv5 - 0.0) * s.dn[740][13]);let eq56_e1339_d_n14: f64 = ((nv5 - 0.0) * s.dn[740][14]);let eq56_e1339_d_n15: f64 = ((nv5 - 0.0) * s.dn[740][15]);let eq56_e1339_d_n16: f64 = ((nv5 - 0.0) * s.dn[740][16]);let eq56_e1339_d_n17: f64 = ((nv5 - 0.0) * s.dn[740][17]);let eq56_e1339_d_n18: f64 = ((nv5 - 0.0) * s.dn[740][18]);let eq56_e1339_d_b0: f64 = ((nv5 - 0.0) * s.db[740][0]);let eq56_e1339_d_b1: f64 = ((nv5 - 0.0) * s.db[740][1]);let eq56_e1339_d_b2: f64 = ((nv5 - 0.0) * s.db[740][2]);let eq56_e1339_d_b3: f64 = ((nv5 - 0.0) * s.db[740][3]);let eq56_e1339_d_b4: f64 = ((nv5 - 0.0) * s.db[740][4]);let eq56_e1339_d_b5: f64 = ((nv5 - 0.0) * s.db[740][5]);let eq56_e1339_d_b6: f64 = ((nv5 - 0.0) * s.db[740][6]);let eq56_e1339_d_b7: f64 = ((nv5 - 0.0) * s.db[740][7]);let eq56_e1339_d_b8: f64 = ((nv5 - 0.0) * s.db[740][8]);let eq56_e1339_d_b9: f64 = ((nv5 - 0.0) * s.db[740][9]);let eq56_e1339_d_b10: f64 = ((nv5 - 0.0) * s.db[740][10]);let eq56_e1339_d_b11: f64 = ((nv5 - 0.0) * s.db[740][11]);let eq56_e1339_d_b12: f64 = ((nv5 - 0.0) * s.db[740][12]);
        (eq56_e1339, eq56_e1339_d_n0, eq56_e1339_d_n1, eq56_e1339_d_n2, eq56_e1339_d_n3, eq56_e1339_d_n4, eq56_e1339_d_n5, eq56_e1339_d_n6, eq56_e1339_d_n7, eq56_e1339_d_n8, eq56_e1339_d_n9, eq56_e1339_d_n10, eq56_e1339_d_n11, eq56_e1339_d_n12, eq56_e1339_d_n13, eq56_e1339_d_n14, eq56_e1339_d_n15, eq56_e1339_d_n16, eq56_e1339_d_n17, eq56_e1339_d_n18, eq56_e1339_d_b0, eq56_e1339_d_b1, eq56_e1339_d_b2, eq56_e1339_d_b3, eq56_e1339_d_b4, eq56_e1339_d_b5, eq56_e1339_d_b6, eq56_e1339_d_b7, eq56_e1339_d_b8, eq56_e1339_d_b9, eq56_e1339_d_b10, eq56_e1339_d_b11, eq56_e1339_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e1341;let eq56_node_derivatives: [f64; 19] = [eq56_e1341_d_n0, eq56_e1341_d_n1, eq56_e1341_d_n2, eq56_e1341_d_n3, eq56_e1341_d_n4, eq56_e1341_d_n5, eq56_e1341_d_n6, eq56_e1341_d_n7, eq56_e1341_d_n8, eq56_e1341_d_n9, eq56_e1341_d_n10, eq56_e1341_d_n11, eq56_e1341_d_n12, eq56_e1341_d_n13, eq56_e1341_d_n14, eq56_e1341_d_n15, eq56_e1341_d_n16, eq56_e1341_d_n17, eq56_e1341_d_n18];let eq56_branch_derivatives: [f64; 13] = [eq56_e1341_d_b0, eq56_e1341_d_b1, eq56_e1341_d_b2, eq56_e1341_d_b3, eq56_e1341_d_b4, eq56_e1341_d_b5, eq56_e1341_d_b6, eq56_e1341_d_b7, eq56_e1341_d_b8, eq56_e1341_d_b9, eq56_e1341_d_b10, eq56_e1341_d_b11, eq56_e1341_d_b12];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1346, eq57_e1346_d_n0, eq57_e1346_d_n1, eq57_e1346_d_n2, eq57_e1346_d_n3, eq57_e1346_d_n4, eq57_e1346_d_n5, eq57_e1346_d_n6, eq57_e1346_d_n7, eq57_e1346_d_n8, eq57_e1346_d_n9, eq57_e1346_d_n10, eq57_e1346_d_n11, eq57_e1346_d_n12, eq57_e1346_d_n13, eq57_e1346_d_n14, eq57_e1346_d_n15, eq57_e1346_d_n16, eq57_e1346_d_n17, eq57_e1346_d_n18, eq57_e1346_d_b0, eq57_e1346_d_b1, eq57_e1346_d_b2, eq57_e1346_d_b3, eq57_e1346_d_b4, eq57_e1346_d_b5, eq57_e1346_d_b6, eq57_e1346_d_b7, eq57_e1346_d_b8, eq57_e1346_d_b9, eq57_e1346_d_b10, eq57_e1346_d_b11, eq57_e1346_d_b12,) = {
    if s.b[3413] {
        let eq57_e1344: f64 = (-s.v[802]);
        (eq57_e1344, (-s.dn[802][0]), (-s.dn[802][1]), (-s.dn[802][2]), (-s.dn[802][3]), (-s.dn[802][4]), (-s.dn[802][5]), (-s.dn[802][6]), (-s.dn[802][7]), (-s.dn[802][8]), (-s.dn[802][9]), (-s.dn[802][10]), (-s.dn[802][11]), (-s.dn[802][12]), (-s.dn[802][13]), (-s.dn[802][14]), (-s.dn[802][15]), (-s.dn[802][16]), (-s.dn[802][17]), (-s.dn[802][18]), (-s.db[802][0]), (-s.db[802][1]), (-s.db[802][2]), (-s.db[802][3]), (-s.db[802][4]), (-s.db[802][5]), (-s.db[802][6]), (-s.db[802][7]), (-s.db[802][8]), (-s.db[802][9]), (-s.db[802][10]), (-s.db[802][11]), (-s.db[802][12]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1346;let eq57_node_derivatives: [f64; 19] = [eq57_e1346_d_n0, eq57_e1346_d_n1, eq57_e1346_d_n2, eq57_e1346_d_n3, eq57_e1346_d_n4, eq57_e1346_d_n5, eq57_e1346_d_n6, eq57_e1346_d_n7, eq57_e1346_d_n8, eq57_e1346_d_n9, eq57_e1346_d_n10, eq57_e1346_d_n11, eq57_e1346_d_n12, eq57_e1346_d_n13, eq57_e1346_d_n14, eq57_e1346_d_n15, eq57_e1346_d_n16, eq57_e1346_d_n17, eq57_e1346_d_n18];let eq57_branch_derivatives: [f64; 13] = [eq57_e1346_d_b0, eq57_e1346_d_b1, eq57_e1346_d_b2, eq57_e1346_d_b3, eq57_e1346_d_b4, eq57_e1346_d_b5, eq57_e1346_d_b6, eq57_e1346_d_b7, eq57_e1346_d_b8, eq57_e1346_d_b9, eq57_e1346_d_b10, eq57_e1346_d_b11, eq57_e1346_d_b12];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1353, eq58_e1353_d_n5,) = {
    if (!s.b[3413]) {
        let eq58_e1351: f64 = ((nv5 - 0.0) * 10000.0);
        (eq58_e1351, 10000.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1353;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq58_value),
            5,
            multiplicity * (eq58_e1353_d_n5),
        );let eq59_e1356: f64 = (s.v[767] * (nv5 - 0.0));let eq59_e1356_d_n0: f64 = (s.dn[767][0] * (nv5 - 0.0));let eq59_e1356_d_n1: f64 = (s.dn[767][1] * (nv5 - 0.0));let eq59_e1356_d_n2: f64 = (s.dn[767][2] * (nv5 - 0.0));let eq59_e1356_d_n3: f64 = (s.dn[767][3] * (nv5 - 0.0));let eq59_e1356_d_n4: f64 = (s.dn[767][4] * (nv5 - 0.0));let eq59_e1356_d_n5: f64 = ((s.dn[767][5] * (nv5 - 0.0)) + s.v[767]);let eq59_e1356_d_n6: f64 = (s.dn[767][6] * (nv5 - 0.0));let eq59_e1356_d_n7: f64 = (s.dn[767][7] * (nv5 - 0.0));let eq59_e1356_d_n8: f64 = (s.dn[767][8] * (nv5 - 0.0));let eq59_e1356_d_n9: f64 = (s.dn[767][9] * (nv5 - 0.0));let eq59_e1356_d_n10: f64 = (s.dn[767][10] * (nv5 - 0.0));let eq59_e1356_d_n11: f64 = (s.dn[767][11] * (nv5 - 0.0));let eq59_e1356_d_n12: f64 = (s.dn[767][12] * (nv5 - 0.0));let eq59_e1356_d_n13: f64 = (s.dn[767][13] * (nv5 - 0.0));let eq59_e1356_d_n14: f64 = (s.dn[767][14] * (nv5 - 0.0));let eq59_e1356_d_n15: f64 = (s.dn[767][15] * (nv5 - 0.0));let eq59_e1356_d_n16: f64 = (s.dn[767][16] * (nv5 - 0.0));let eq59_e1356_d_n17: f64 = (s.dn[767][17] * (nv5 - 0.0));let eq59_e1356_d_n18: f64 = (s.dn[767][18] * (nv5 - 0.0));let eq59_e1356_d_b0: f64 = (s.db[767][0] * (nv5 - 0.0));let eq59_e1356_d_b1: f64 = (s.db[767][1] * (nv5 - 0.0));let eq59_e1356_d_b2: f64 = (s.db[767][2] * (nv5 - 0.0));let eq59_e1356_d_b3: f64 = (s.db[767][3] * (nv5 - 0.0));let eq59_e1356_d_b4: f64 = (s.db[767][4] * (nv5 - 0.0));let eq59_e1356_d_b5: f64 = (s.db[767][5] * (nv5 - 0.0));let eq59_e1356_d_b6: f64 = (s.db[767][6] * (nv5 - 0.0));let eq59_e1356_d_b7: f64 = (s.db[767][7] * (nv5 - 0.0));let eq59_e1356_d_b8: f64 = (s.db[767][8] * (nv5 - 0.0));let eq59_e1356_d_b9: f64 = (s.db[767][9] * (nv5 - 0.0));let eq59_e1356_d_b10: f64 = (s.db[767][10] * (nv5 - 0.0));let eq59_e1356_d_b11: f64 = (s.db[767][11] * (nv5 - 0.0));let eq59_e1356_d_b12: f64 = (s.db[767][12] * (nv5 - 0.0));let eq59_e1357: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq59_e1356);let eq59_value: f64 = eq59_e1357;let eq59_node_derivatives: [f64; 19] = [(eq59_e1356_d_n0 * ddt_scale), (eq59_e1356_d_n1 * ddt_scale), (eq59_e1356_d_n2 * ddt_scale), (eq59_e1356_d_n3 * ddt_scale), (eq59_e1356_d_n4 * ddt_scale), (eq59_e1356_d_n5 * ddt_scale), (eq59_e1356_d_n6 * ddt_scale), (eq59_e1356_d_n7 * ddt_scale), (eq59_e1356_d_n8 * ddt_scale), (eq59_e1356_d_n9 * ddt_scale), (eq59_e1356_d_n10 * ddt_scale), (eq59_e1356_d_n11 * ddt_scale), (eq59_e1356_d_n12 * ddt_scale), (eq59_e1356_d_n13 * ddt_scale), (eq59_e1356_d_n14 * ddt_scale), (eq59_e1356_d_n15 * ddt_scale), (eq59_e1356_d_n16 * ddt_scale), (eq59_e1356_d_n17 * ddt_scale), (eq59_e1356_d_n18 * ddt_scale)];let eq59_branch_derivatives: [f64; 13] = [(eq59_e1356_d_b0 * ddt_scale), (eq59_e1356_d_b1 * ddt_scale), (eq59_e1356_d_b2 * ddt_scale), (eq59_e1356_d_b3 * ddt_scale), (eq59_e1356_d_b4 * ddt_scale), (eq59_e1356_d_b5 * ddt_scale), (eq59_e1356_d_b6 * ddt_scale), (eq59_e1356_d_b7 * ddt_scale), (eq59_e1356_d_b8 * ddt_scale), (eq59_e1356_d_b9 * ddt_scale), (eq59_e1356_d_b10 * ddt_scale), (eq59_e1356_d_b11 * ddt_scale), (eq59_e1356_d_b12 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
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
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq60_e1361, eq60_e1361_d_n0, eq60_e1361_d_n1, eq60_e1361_d_n2, eq60_e1361_d_n3, eq60_e1361_d_n4, eq60_e1361_d_n5, eq60_e1361_d_n6, eq60_e1361_d_n7, eq60_e1361_d_n8, eq60_e1361_d_n9, eq60_e1361_d_n10, eq60_e1361_d_n11, eq60_e1361_d_n12, eq60_e1361_d_n13, eq60_e1361_d_n14, eq60_e1361_d_n15, eq60_e1361_d_n16, eq60_e1361_d_n17, eq60_e1361_d_n18, eq60_e1361_d_b0, eq60_e1361_d_b1, eq60_e1361_d_b2, eq60_e1361_d_b3, eq60_e1361_d_b4, eq60_e1361_d_b5, eq60_e1361_d_b6, eq60_e1361_d_b7, eq60_e1361_d_b8, eq60_e1361_d_b9, eq60_e1361_d_b10, eq60_e1361_d_b11, eq60_e1361_d_b12,) = {
    if (p.p28 != 0.0) {
        (s.v[749], s.dn[749][0], s.dn[749][1], s.dn[749][2], s.dn[749][3], s.dn[749][4], s.dn[749][5], s.dn[749][6], s.dn[749][7], s.dn[749][8], s.dn[749][9], s.dn[749][10], s.dn[749][11], s.dn[749][12], s.dn[749][13], s.dn[749][14], s.dn[749][15], s.dn[749][16], s.dn[749][17], s.dn[749][18], s.db[749][0], s.db[749][1], s.db[749][2], s.db[749][3], s.db[749][4], s.db[749][5], s.db[749][6], s.db[749][7], s.db[749][8], s.db[749][9], s.db[749][10], s.db[749][11], s.db[749][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1361;let eq60_node_derivatives: [f64; 19] = [eq60_e1361_d_n0, eq60_e1361_d_n1, eq60_e1361_d_n2, eq60_e1361_d_n3, eq60_e1361_d_n4, eq60_e1361_d_n5, eq60_e1361_d_n6, eq60_e1361_d_n7, eq60_e1361_d_n8, eq60_e1361_d_n9, eq60_e1361_d_n10, eq60_e1361_d_n11, eq60_e1361_d_n12, eq60_e1361_d_n13, eq60_e1361_d_n14, eq60_e1361_d_n15, eq60_e1361_d_n16, eq60_e1361_d_n17, eq60_e1361_d_n18];let eq60_branch_derivatives: [f64; 13] = [eq60_e1361_d_b0, eq60_e1361_d_b1, eq60_e1361_d_b2, eq60_e1361_d_b3, eq60_e1361_d_b4, eq60_e1361_d_b5, eq60_e1361_d_b6, eq60_e1361_d_b7, eq60_e1361_d_b8, eq60_e1361_d_b9, eq60_e1361_d_b10, eq60_e1361_d_b11, eq60_e1361_d_b12];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1365, eq61_e1365_d_n0, eq61_e1365_d_n1, eq61_e1365_d_n2, eq61_e1365_d_n3, eq61_e1365_d_n4, eq61_e1365_d_n5, eq61_e1365_d_n6, eq61_e1365_d_n7, eq61_e1365_d_n8, eq61_e1365_d_n9, eq61_e1365_d_n10, eq61_e1365_d_n11, eq61_e1365_d_n12, eq61_e1365_d_n13, eq61_e1365_d_n14, eq61_e1365_d_n15, eq61_e1365_d_n16, eq61_e1365_d_n17, eq61_e1365_d_n18, eq61_e1365_d_b0, eq61_e1365_d_b1, eq61_e1365_d_b2, eq61_e1365_d_b3, eq61_e1365_d_b4, eq61_e1365_d_b5, eq61_e1365_d_b6, eq61_e1365_d_b7, eq61_e1365_d_b8, eq61_e1365_d_b9, eq61_e1365_d_b10, eq61_e1365_d_b11, eq61_e1365_d_b12,) = {
    if (p.p28 != 0.0) {
        (s.v[750], s.dn[750][0], s.dn[750][1], s.dn[750][2], s.dn[750][3], s.dn[750][4], s.dn[750][5], s.dn[750][6], s.dn[750][7], s.dn[750][8], s.dn[750][9], s.dn[750][10], s.dn[750][11], s.dn[750][12], s.dn[750][13], s.dn[750][14], s.dn[750][15], s.dn[750][16], s.dn[750][17], s.dn[750][18], s.db[750][0], s.db[750][1], s.db[750][2], s.db[750][3], s.db[750][4], s.db[750][5], s.db[750][6], s.db[750][7], s.db[750][8], s.db[750][9], s.db[750][10], s.db[750][11], s.db[750][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1365;let eq61_node_derivatives: [f64; 19] = [eq61_e1365_d_n0, eq61_e1365_d_n1, eq61_e1365_d_n2, eq61_e1365_d_n3, eq61_e1365_d_n4, eq61_e1365_d_n5, eq61_e1365_d_n6, eq61_e1365_d_n7, eq61_e1365_d_n8, eq61_e1365_d_n9, eq61_e1365_d_n10, eq61_e1365_d_n11, eq61_e1365_d_n12, eq61_e1365_d_n13, eq61_e1365_d_n14, eq61_e1365_d_n15, eq61_e1365_d_n16, eq61_e1365_d_n17, eq61_e1365_d_n18];let eq61_branch_derivatives: [f64; 13] = [eq61_e1365_d_b0, eq61_e1365_d_b1, eq61_e1365_d_b2, eq61_e1365_d_b3, eq61_e1365_d_b4, eq61_e1365_d_b5, eq61_e1365_d_b6, eq61_e1365_d_b7, eq61_e1365_d_b8, eq61_e1365_d_b9, eq61_e1365_d_b10, eq61_e1365_d_b11, eq61_e1365_d_b12];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1372, eq62_e1372_d_n0, eq62_e1372_d_n1, eq62_e1372_d_n2, eq62_e1372_d_n3, eq62_e1372_d_n4, eq62_e1372_d_n5, eq62_e1372_d_n6, eq62_e1372_d_n7, eq62_e1372_d_n8, eq62_e1372_d_n9, eq62_e1372_d_n10, eq62_e1372_d_n11, eq62_e1372_d_n12, eq62_e1372_d_n13, eq62_e1372_d_n14, eq62_e1372_d_n15, eq62_e1372_d_n16, eq62_e1372_d_n17, eq62_e1372_d_n18, eq62_e1372_d_b0, eq62_e1372_d_b1, eq62_e1372_d_b2, eq62_e1372_d_b3, eq62_e1372_d_b4, eq62_e1372_d_b5, eq62_e1372_d_b6, eq62_e1372_d_b7, eq62_e1372_d_b8, eq62_e1372_d_b9, eq62_e1372_d_b10, eq62_e1372_d_b11, eq62_e1372_d_b12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1369: f64 = (s.v[800] * (nv12 - 0.0));let eq62_e1369_d_n0: f64 = (s.dn[800][0] * (nv12 - 0.0));let eq62_e1369_d_n1: f64 = (s.dn[800][1] * (nv12 - 0.0));let eq62_e1369_d_n2: f64 = (s.dn[800][2] * (nv12 - 0.0));let eq62_e1369_d_n3: f64 = (s.dn[800][3] * (nv12 - 0.0));let eq62_e1369_d_n4: f64 = (s.dn[800][4] * (nv12 - 0.0));let eq62_e1369_d_n5: f64 = (s.dn[800][5] * (nv12 - 0.0));let eq62_e1369_d_n6: f64 = (s.dn[800][6] * (nv12 - 0.0));let eq62_e1369_d_n7: f64 = (s.dn[800][7] * (nv12 - 0.0));let eq62_e1369_d_n8: f64 = (s.dn[800][8] * (nv12 - 0.0));let eq62_e1369_d_n9: f64 = (s.dn[800][9] * (nv12 - 0.0));let eq62_e1369_d_n10: f64 = (s.dn[800][10] * (nv12 - 0.0));let eq62_e1369_d_n11: f64 = (s.dn[800][11] * (nv12 - 0.0));let eq62_e1369_d_n12: f64 = ((s.dn[800][12] * (nv12 - 0.0)) + s.v[800]);let eq62_e1369_d_n13: f64 = (s.dn[800][13] * (nv12 - 0.0));let eq62_e1369_d_n14: f64 = (s.dn[800][14] * (nv12 - 0.0));let eq62_e1369_d_n15: f64 = (s.dn[800][15] * (nv12 - 0.0));let eq62_e1369_d_n16: f64 = (s.dn[800][16] * (nv12 - 0.0));let eq62_e1369_d_n17: f64 = (s.dn[800][17] * (nv12 - 0.0));let eq62_e1369_d_n18: f64 = (s.dn[800][18] * (nv12 - 0.0));let eq62_e1369_d_b0: f64 = (s.db[800][0] * (nv12 - 0.0));let eq62_e1369_d_b1: f64 = (s.db[800][1] * (nv12 - 0.0));let eq62_e1369_d_b2: f64 = (s.db[800][2] * (nv12 - 0.0));let eq62_e1369_d_b3: f64 = (s.db[800][3] * (nv12 - 0.0));let eq62_e1369_d_b4: f64 = (s.db[800][4] * (nv12 - 0.0));let eq62_e1369_d_b5: f64 = (s.db[800][5] * (nv12 - 0.0));let eq62_e1369_d_b6: f64 = (s.db[800][6] * (nv12 - 0.0));let eq62_e1369_d_b7: f64 = (s.db[800][7] * (nv12 - 0.0));let eq62_e1369_d_b8: f64 = (s.db[800][8] * (nv12 - 0.0));let eq62_e1369_d_b9: f64 = (s.db[800][9] * (nv12 - 0.0));let eq62_e1369_d_b10: f64 = (s.db[800][10] * (nv12 - 0.0));let eq62_e1369_d_b11: f64 = (s.db[800][11] * (nv12 - 0.0));let eq62_e1369_d_b12: f64 = (s.db[800][12] * (nv12 - 0.0));let eq62_e1370: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq62_e1369);
        (eq62_e1370, (eq62_e1369_d_n0 * ddt_scale), (eq62_e1369_d_n1 * ddt_scale), (eq62_e1369_d_n2 * ddt_scale), (eq62_e1369_d_n3 * ddt_scale), (eq62_e1369_d_n4 * ddt_scale), (eq62_e1369_d_n5 * ddt_scale), (eq62_e1369_d_n6 * ddt_scale), (eq62_e1369_d_n7 * ddt_scale), (eq62_e1369_d_n8 * ddt_scale), (eq62_e1369_d_n9 * ddt_scale), (eq62_e1369_d_n10 * ddt_scale), (eq62_e1369_d_n11 * ddt_scale), (eq62_e1369_d_n12 * ddt_scale), (eq62_e1369_d_n13 * ddt_scale), (eq62_e1369_d_n14 * ddt_scale), (eq62_e1369_d_n15 * ddt_scale), (eq62_e1369_d_n16 * ddt_scale), (eq62_e1369_d_n17 * ddt_scale), (eq62_e1369_d_n18 * ddt_scale), (eq62_e1369_d_b0 * ddt_scale), (eq62_e1369_d_b1 * ddt_scale), (eq62_e1369_d_b2 * ddt_scale), (eq62_e1369_d_b3 * ddt_scale), (eq62_e1369_d_b4 * ddt_scale), (eq62_e1369_d_b5 * ddt_scale), (eq62_e1369_d_b6 * ddt_scale), (eq62_e1369_d_b7 * ddt_scale), (eq62_e1369_d_b8 * ddt_scale), (eq62_e1369_d_b9 * ddt_scale), (eq62_e1369_d_b10 * ddt_scale), (eq62_e1369_d_b11 * ddt_scale), (eq62_e1369_d_b12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1372;let eq62_node_derivatives: [f64; 19] = [eq62_e1372_d_n0, eq62_e1372_d_n1, eq62_e1372_d_n2, eq62_e1372_d_n3, eq62_e1372_d_n4, eq62_e1372_d_n5, eq62_e1372_d_n6, eq62_e1372_d_n7, eq62_e1372_d_n8, eq62_e1372_d_n9, eq62_e1372_d_n10, eq62_e1372_d_n11, eq62_e1372_d_n12, eq62_e1372_d_n13, eq62_e1372_d_n14, eq62_e1372_d_n15, eq62_e1372_d_n16, eq62_e1372_d_n17, eq62_e1372_d_n18];let eq62_branch_derivatives: [f64; 13] = [eq62_e1372_d_b0, eq62_e1372_d_b1, eq62_e1372_d_b2, eq62_e1372_d_b3, eq62_e1372_d_b4, eq62_e1372_d_b5, eq62_e1372_d_b6, eq62_e1372_d_b7, eq62_e1372_d_b8, eq62_e1372_d_b9, eq62_e1372_d_b10, eq62_e1372_d_b11, eq62_e1372_d_b12];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e1379, eq63_e1379_d_n0, eq63_e1379_d_n1, eq63_e1379_d_n2, eq63_e1379_d_n3, eq63_e1379_d_n4, eq63_e1379_d_n5, eq63_e1379_d_n6, eq63_e1379_d_n7, eq63_e1379_d_n8, eq63_e1379_d_n9, eq63_e1379_d_n10, eq63_e1379_d_n11, eq63_e1379_d_n12, eq63_e1379_d_n13, eq63_e1379_d_n14, eq63_e1379_d_n15, eq63_e1379_d_n16, eq63_e1379_d_n17, eq63_e1379_d_n18, eq63_e1379_d_b0, eq63_e1379_d_b1, eq63_e1379_d_b2, eq63_e1379_d_b3, eq63_e1379_d_b4, eq63_e1379_d_b5, eq63_e1379_d_b6, eq63_e1379_d_b7, eq63_e1379_d_b8, eq63_e1379_d_b9, eq63_e1379_d_b10, eq63_e1379_d_b11, eq63_e1379_d_b12,) = {
    if (p.p28 != 0.0) {
        let eq63_e1376: f64 = (s.v[801] * (nv13 - 0.0));let eq63_e1376_d_n0: f64 = (s.dn[801][0] * (nv13 - 0.0));let eq63_e1376_d_n1: f64 = (s.dn[801][1] * (nv13 - 0.0));let eq63_e1376_d_n2: f64 = (s.dn[801][2] * (nv13 - 0.0));let eq63_e1376_d_n3: f64 = (s.dn[801][3] * (nv13 - 0.0));let eq63_e1376_d_n4: f64 = (s.dn[801][4] * (nv13 - 0.0));let eq63_e1376_d_n5: f64 = (s.dn[801][5] * (nv13 - 0.0));let eq63_e1376_d_n6: f64 = (s.dn[801][6] * (nv13 - 0.0));let eq63_e1376_d_n7: f64 = (s.dn[801][7] * (nv13 - 0.0));let eq63_e1376_d_n8: f64 = (s.dn[801][8] * (nv13 - 0.0));let eq63_e1376_d_n9: f64 = (s.dn[801][9] * (nv13 - 0.0));let eq63_e1376_d_n10: f64 = (s.dn[801][10] * (nv13 - 0.0));let eq63_e1376_d_n11: f64 = (s.dn[801][11] * (nv13 - 0.0));let eq63_e1376_d_n12: f64 = (s.dn[801][12] * (nv13 - 0.0));let eq63_e1376_d_n13: f64 = ((s.dn[801][13] * (nv13 - 0.0)) + s.v[801]);let eq63_e1376_d_n14: f64 = (s.dn[801][14] * (nv13 - 0.0));let eq63_e1376_d_n15: f64 = (s.dn[801][15] * (nv13 - 0.0));let eq63_e1376_d_n16: f64 = (s.dn[801][16] * (nv13 - 0.0));let eq63_e1376_d_n17: f64 = (s.dn[801][17] * (nv13 - 0.0));let eq63_e1376_d_n18: f64 = (s.dn[801][18] * (nv13 - 0.0));let eq63_e1376_d_b0: f64 = (s.db[801][0] * (nv13 - 0.0));let eq63_e1376_d_b1: f64 = (s.db[801][1] * (nv13 - 0.0));let eq63_e1376_d_b2: f64 = (s.db[801][2] * (nv13 - 0.0));let eq63_e1376_d_b3: f64 = (s.db[801][3] * (nv13 - 0.0));let eq63_e1376_d_b4: f64 = (s.db[801][4] * (nv13 - 0.0));let eq63_e1376_d_b5: f64 = (s.db[801][5] * (nv13 - 0.0));let eq63_e1376_d_b6: f64 = (s.db[801][6] * (nv13 - 0.0));let eq63_e1376_d_b7: f64 = (s.db[801][7] * (nv13 - 0.0));let eq63_e1376_d_b8: f64 = (s.db[801][8] * (nv13 - 0.0));let eq63_e1376_d_b9: f64 = (s.db[801][9] * (nv13 - 0.0));let eq63_e1376_d_b10: f64 = (s.db[801][10] * (nv13 - 0.0));let eq63_e1376_d_b11: f64 = (s.db[801][11] * (nv13 - 0.0));let eq63_e1376_d_b12: f64 = (s.db[801][12] * (nv13 - 0.0));let eq63_e1377: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq63_e1376);
        (eq63_e1377, (eq63_e1376_d_n0 * ddt_scale), (eq63_e1376_d_n1 * ddt_scale), (eq63_e1376_d_n2 * ddt_scale), (eq63_e1376_d_n3 * ddt_scale), (eq63_e1376_d_n4 * ddt_scale), (eq63_e1376_d_n5 * ddt_scale), (eq63_e1376_d_n6 * ddt_scale), (eq63_e1376_d_n7 * ddt_scale), (eq63_e1376_d_n8 * ddt_scale), (eq63_e1376_d_n9 * ddt_scale), (eq63_e1376_d_n10 * ddt_scale), (eq63_e1376_d_n11 * ddt_scale), (eq63_e1376_d_n12 * ddt_scale), (eq63_e1376_d_n13 * ddt_scale), (eq63_e1376_d_n14 * ddt_scale), (eq63_e1376_d_n15 * ddt_scale), (eq63_e1376_d_n16 * ddt_scale), (eq63_e1376_d_n17 * ddt_scale), (eq63_e1376_d_n18 * ddt_scale), (eq63_e1376_d_b0 * ddt_scale), (eq63_e1376_d_b1 * ddt_scale), (eq63_e1376_d_b2 * ddt_scale), (eq63_e1376_d_b3 * ddt_scale), (eq63_e1376_d_b4 * ddt_scale), (eq63_e1376_d_b5 * ddt_scale), (eq63_e1376_d_b6 * ddt_scale), (eq63_e1376_d_b7 * ddt_scale), (eq63_e1376_d_b8 * ddt_scale), (eq63_e1376_d_b9 * ddt_scale), (eq63_e1376_d_b10 * ddt_scale), (eq63_e1376_d_b11 * ddt_scale), (eq63_e1376_d_b12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1379;let eq63_node_derivatives: [f64; 19] = [eq63_e1379_d_n0, eq63_e1379_d_n1, eq63_e1379_d_n2, eq63_e1379_d_n3, eq63_e1379_d_n4, eq63_e1379_d_n5, eq63_e1379_d_n6, eq63_e1379_d_n7, eq63_e1379_d_n8, eq63_e1379_d_n9, eq63_e1379_d_n10, eq63_e1379_d_n11, eq63_e1379_d_n12, eq63_e1379_d_n13, eq63_e1379_d_n14, eq63_e1379_d_n15, eq63_e1379_d_n16, eq63_e1379_d_n17, eq63_e1379_d_n18];let eq63_branch_derivatives: [f64; 13] = [eq63_e1379_d_b0, eq63_e1379_d_b1, eq63_e1379_d_b2, eq63_e1379_d_b3, eq63_e1379_d_b4, eq63_e1379_d_b5, eq63_e1379_d_b6, eq63_e1379_d_b7, eq63_e1379_d_b8, eq63_e1379_d_b9, eq63_e1379_d_b10, eq63_e1379_d_b11, eq63_e1379_d_b12];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_21(
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq64_e1384,) = {
    if (p.p28 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1384;
        stamper.stamp_potential_const_local(
            10,
            eq64_value,
        );
        let (eq65_e1389,) = {
    if (p.p28 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1389;
        stamper.stamp_potential_const_local(
            11,
            eq65_value,
        );
        let (eq66_e1393, eq66_e1393_d_n0, eq66_e1393_d_n1, eq66_e1393_d_n2, eq66_e1393_d_n3, eq66_e1393_d_n4, eq66_e1393_d_n5, eq66_e1393_d_n6, eq66_e1393_d_n7, eq66_e1393_d_n8, eq66_e1393_d_n9, eq66_e1393_d_n10, eq66_e1393_d_n11, eq66_e1393_d_n12, eq66_e1393_d_n13, eq66_e1393_d_n14, eq66_e1393_d_n15, eq66_e1393_d_n16, eq66_e1393_d_n17, eq66_e1393_d_n18, eq66_e1393_d_b0, eq66_e1393_d_b1, eq66_e1393_d_b2, eq66_e1393_d_b3, eq66_e1393_d_b4, eq66_e1393_d_b5, eq66_e1393_d_b6, eq66_e1393_d_b7, eq66_e1393_d_b8, eq66_e1393_d_b9, eq66_e1393_d_b10, eq66_e1393_d_b11, eq66_e1393_d_b12,) = {
    if (p.p29 != 0.0) {
        (s.v[815], s.dn[815][0], s.dn[815][1], s.dn[815][2], s.dn[815][3], s.dn[815][4], s.dn[815][5], s.dn[815][6], s.dn[815][7], s.dn[815][8], s.dn[815][9], s.dn[815][10], s.dn[815][11], s.dn[815][12], s.dn[815][13], s.dn[815][14], s.dn[815][15], s.dn[815][16], s.dn[815][17], s.dn[815][18], s.db[815][0], s.db[815][1], s.db[815][2], s.db[815][3], s.db[815][4], s.db[815][5], s.db[815][6], s.db[815][7], s.db[815][8], s.db[815][9], s.db[815][10], s.db[815][11], s.db[815][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1393;let eq66_node_derivatives: [f64; 19] = [eq66_e1393_d_n0, eq66_e1393_d_n1, eq66_e1393_d_n2, eq66_e1393_d_n3, eq66_e1393_d_n4, eq66_e1393_d_n5, eq66_e1393_d_n6, eq66_e1393_d_n7, eq66_e1393_d_n8, eq66_e1393_d_n9, eq66_e1393_d_n10, eq66_e1393_d_n11, eq66_e1393_d_n12, eq66_e1393_d_n13, eq66_e1393_d_n14, eq66_e1393_d_n15, eq66_e1393_d_n16, eq66_e1393_d_n17, eq66_e1393_d_n18];let eq66_branch_derivatives: [f64; 13] = [eq66_e1393_d_b0, eq66_e1393_d_b1, eq66_e1393_d_b2, eq66_e1393_d_b3, eq66_e1393_d_b4, eq66_e1393_d_b5, eq66_e1393_d_b6, eq66_e1393_d_b7, eq66_e1393_d_b8, eq66_e1393_d_b9, eq66_e1393_d_b10, eq66_e1393_d_b11, eq66_e1393_d_b12];
        stamper.stamp_current_dense_local(
            Some(14),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivatives,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1398, eq67_e1398_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1396: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv14 - 0.0));
        (eq67_e1396, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e1398;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
            14,
            multiplicity * (eq67_e1398_d_n14),
        );
        let (eq68_e1403,) = {
    if (p.p29 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1403;
        stamper.stamp_potential_const_local(
            12,
            eq68_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        multiplicity: f64,
    ) {
        let (eq1_e1032, eq1_e1032_d_n0, eq1_e1032_d_n1, eq1_e1032_d_n2, eq1_e1032_d_n3, eq1_e1032_d_n4, eq1_e1032_d_n5, eq1_e1032_d_n6, eq1_e1032_d_n7, eq1_e1032_d_n8, eq1_e1032_d_n9, eq1_e1032_d_n10, eq1_e1032_d_n11, eq1_e1032_d_n12, eq1_e1032_d_n13, eq1_e1032_d_n14, eq1_e1032_d_n15, eq1_e1032_d_n16, eq1_e1032_d_n17, eq1_e1032_d_n18, eq1_e1032_d_b0, eq1_e1032_d_b1, eq1_e1032_d_b2, eq1_e1032_d_b3, eq1_e1032_d_b4, eq1_e1032_d_b5, eq1_e1032_d_b6, eq1_e1032_d_b7, eq1_e1032_d_b8, eq1_e1032_d_b9, eq1_e1032_d_b10, eq1_e1032_d_b11, eq1_e1032_d_b12, eq1_e1032_q, eq1_e1032_q_d_n0, eq1_e1032_q_d_n1, eq1_e1032_q_d_n2, eq1_e1032_q_d_n3, eq1_e1032_q_d_n4, eq1_e1032_q_d_n5, eq1_e1032_q_d_n6, eq1_e1032_q_d_n7, eq1_e1032_q_d_n8, eq1_e1032_q_d_n9, eq1_e1032_q_d_n10, eq1_e1032_q_d_n11, eq1_e1032_q_d_n12, eq1_e1032_q_d_n13, eq1_e1032_q_d_n14, eq1_e1032_q_d_n15, eq1_e1032_q_d_n16, eq1_e1032_q_d_n17, eq1_e1032_q_d_n18, eq1_e1032_q_d_b0, eq1_e1032_q_d_b1, eq1_e1032_q_d_b2, eq1_e1032_q_d_b3, eq1_e1032_q_d_b4, eq1_e1032_q_d_b5, eq1_e1032_q_d_b6, eq1_e1032_q_d_b7, eq1_e1032_q_d_b8, eq1_e1032_q_d_b9, eq1_e1032_q_d_b10, eq1_e1032_q_d_b11, eq1_e1032_q_d_b12,) = {
    if s.b[3309] {
        let eq1_e1029_q: f64 = s.v[924];let eq1_e1030: f64 = (s.v[926] + s.v[924]);let eq1_e1030_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);let eq1_e1030_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);let eq1_e1030_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);let eq1_e1030_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);let eq1_e1030_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);let eq1_e1030_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);let eq1_e1030_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);let eq1_e1030_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);let eq1_e1030_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);let eq1_e1030_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);let eq1_e1030_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);let eq1_e1030_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);let eq1_e1030_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);let eq1_e1030_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);let eq1_e1030_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);let eq1_e1030_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);let eq1_e1030_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);let eq1_e1030_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);let eq1_e1030_d_n18: f64 = (s.dn[926][18] + s.dn[924][18]);let eq1_e1030_d_b0: f64 = (s.db[926][0] + s.db[924][0]);let eq1_e1030_d_b1: f64 = (s.db[926][1] + s.db[924][1]);let eq1_e1030_d_b2: f64 = (s.db[926][2] + s.db[924][2]);let eq1_e1030_d_b3: f64 = (s.db[926][3] + s.db[924][3]);let eq1_e1030_d_b4: f64 = (s.db[926][4] + s.db[924][4]);let eq1_e1030_d_b5: f64 = (s.db[926][5] + s.db[924][5]);let eq1_e1030_d_b6: f64 = (s.db[926][6] + s.db[924][6]);let eq1_e1030_d_b7: f64 = (s.db[926][7] + s.db[924][7]);let eq1_e1030_d_b8: f64 = (s.db[926][8] + s.db[924][8]);let eq1_e1030_d_b9: f64 = (s.db[926][9] + s.db[924][9]);let eq1_e1030_d_b10: f64 = (s.db[926][10] + s.db[924][10]);let eq1_e1030_d_b11: f64 = (s.db[926][11] + s.db[924][11]);let eq1_e1030_d_b12: f64 = (s.db[926][12] + s.db[924][12]);let eq1_e1030_q: f64 = eq1_e1029_q;
        (eq1_e1030, eq1_e1030_d_n0, eq1_e1030_d_n1, eq1_e1030_d_n2, eq1_e1030_d_n3, eq1_e1030_d_n4, eq1_e1030_d_n5, eq1_e1030_d_n6, eq1_e1030_d_n7, eq1_e1030_d_n8, eq1_e1030_d_n9, eq1_e1030_d_n10, eq1_e1030_d_n11, eq1_e1030_d_n12, eq1_e1030_d_n13, eq1_e1030_d_n14, eq1_e1030_d_n15, eq1_e1030_d_n16, eq1_e1030_d_n17, eq1_e1030_d_n18, eq1_e1030_d_b0, eq1_e1030_d_b1, eq1_e1030_d_b2, eq1_e1030_d_b3, eq1_e1030_d_b4, eq1_e1030_d_b5, eq1_e1030_d_b6, eq1_e1030_d_b7, eq1_e1030_d_b8, eq1_e1030_d_b9, eq1_e1030_d_b10, eq1_e1030_d_b11, eq1_e1030_d_b12, eq1_e1030_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.dn[924][18], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8], s.db[924][9], s.db[924][10], s.db[924][11], s.db[924][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 19] = [eq1_e1032_q_d_n0, eq1_e1032_q_d_n1, eq1_e1032_q_d_n2, eq1_e1032_q_d_n3, eq1_e1032_q_d_n4, eq1_e1032_q_d_n5, eq1_e1032_q_d_n6, eq1_e1032_q_d_n7, eq1_e1032_q_d_n8, eq1_e1032_q_d_n9, eq1_e1032_q_d_n10, eq1_e1032_q_d_n11, eq1_e1032_q_d_n12, eq1_e1032_q_d_n13, eq1_e1032_q_d_n14, eq1_e1032_q_d_n15, eq1_e1032_q_d_n16, eq1_e1032_q_d_n17, eq1_e1032_q_d_n18];let eq1_reactive_branch_derivatives: [f64; 13] = [eq1_e1032_q_d_b0, eq1_e1032_q_d_b1, eq1_e1032_q_d_b2, eq1_e1032_q_d_b3, eq1_e1032_q_d_b4, eq1_e1032_q_d_b5, eq1_e1032_q_d_b6, eq1_e1032_q_d_b7, eq1_e1032_q_d_b8, eq1_e1032_q_d_b9, eq1_e1032_q_d_b10, eq1_e1032_q_d_b11, eq1_e1032_q_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(16),
            None,
            &eq1_reactive_node_derivatives,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1039, eq2_e1039_d_n0, eq2_e1039_d_n1, eq2_e1039_d_n2, eq2_e1039_d_n3, eq2_e1039_d_n4, eq2_e1039_d_n5, eq2_e1039_d_n6, eq2_e1039_d_n7, eq2_e1039_d_n8, eq2_e1039_d_n9, eq2_e1039_d_n10, eq2_e1039_d_n11, eq2_e1039_d_n12, eq2_e1039_d_n13, eq2_e1039_d_n14, eq2_e1039_d_n15, eq2_e1039_d_n16, eq2_e1039_d_n17, eq2_e1039_d_n18, eq2_e1039_d_b0, eq2_e1039_d_b1, eq2_e1039_d_b2, eq2_e1039_d_b3, eq2_e1039_d_b4, eq2_e1039_d_b5, eq2_e1039_d_b6, eq2_e1039_d_b7, eq2_e1039_d_b8, eq2_e1039_d_b9, eq2_e1039_d_b10, eq2_e1039_d_b11, eq2_e1039_d_b12, eq2_e1039_q, eq2_e1039_q_d_n0, eq2_e1039_q_d_n1, eq2_e1039_q_d_n2, eq2_e1039_q_d_n3, eq2_e1039_q_d_n4, eq2_e1039_q_d_n5, eq2_e1039_q_d_n6, eq2_e1039_q_d_n7, eq2_e1039_q_d_n8, eq2_e1039_q_d_n9, eq2_e1039_q_d_n10, eq2_e1039_q_d_n11, eq2_e1039_q_d_n12, eq2_e1039_q_d_n13, eq2_e1039_q_d_n14, eq2_e1039_q_d_n15, eq2_e1039_q_d_n16, eq2_e1039_q_d_n17, eq2_e1039_q_d_n18, eq2_e1039_q_d_b0, eq2_e1039_q_d_b1, eq2_e1039_q_d_b2, eq2_e1039_q_d_b3, eq2_e1039_q_d_b4, eq2_e1039_q_d_b5, eq2_e1039_q_d_b6, eq2_e1039_q_d_b7, eq2_e1039_q_d_b8, eq2_e1039_q_d_b9, eq2_e1039_q_d_b10, eq2_e1039_q_d_b11, eq2_e1039_q_d_b12,) = {
    if s.b[3309] {
        let eq2_e1036_q: f64 = s.v[925];let eq2_e1037: f64 = (s.v[927] + s.v[925]);let eq2_e1037_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);let eq2_e1037_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);let eq2_e1037_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);let eq2_e1037_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);let eq2_e1037_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);let eq2_e1037_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);let eq2_e1037_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);let eq2_e1037_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);let eq2_e1037_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);let eq2_e1037_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);let eq2_e1037_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);let eq2_e1037_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);let eq2_e1037_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);let eq2_e1037_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);let eq2_e1037_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);let eq2_e1037_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);let eq2_e1037_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);let eq2_e1037_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);let eq2_e1037_d_n18: f64 = (s.dn[927][18] + s.dn[925][18]);let eq2_e1037_d_b0: f64 = (s.db[927][0] + s.db[925][0]);let eq2_e1037_d_b1: f64 = (s.db[927][1] + s.db[925][1]);let eq2_e1037_d_b2: f64 = (s.db[927][2] + s.db[925][2]);let eq2_e1037_d_b3: f64 = (s.db[927][3] + s.db[925][3]);let eq2_e1037_d_b4: f64 = (s.db[927][4] + s.db[925][4]);let eq2_e1037_d_b5: f64 = (s.db[927][5] + s.db[925][5]);let eq2_e1037_d_b6: f64 = (s.db[927][6] + s.db[925][6]);let eq2_e1037_d_b7: f64 = (s.db[927][7] + s.db[925][7]);let eq2_e1037_d_b8: f64 = (s.db[927][8] + s.db[925][8]);let eq2_e1037_d_b9: f64 = (s.db[927][9] + s.db[925][9]);let eq2_e1037_d_b10: f64 = (s.db[927][10] + s.db[925][10]);let eq2_e1037_d_b11: f64 = (s.db[927][11] + s.db[925][11]);let eq2_e1037_d_b12: f64 = (s.db[927][12] + s.db[925][12]);let eq2_e1037_q: f64 = eq2_e1036_q;
        (eq2_e1037, eq2_e1037_d_n0, eq2_e1037_d_n1, eq2_e1037_d_n2, eq2_e1037_d_n3, eq2_e1037_d_n4, eq2_e1037_d_n5, eq2_e1037_d_n6, eq2_e1037_d_n7, eq2_e1037_d_n8, eq2_e1037_d_n9, eq2_e1037_d_n10, eq2_e1037_d_n11, eq2_e1037_d_n12, eq2_e1037_d_n13, eq2_e1037_d_n14, eq2_e1037_d_n15, eq2_e1037_d_n16, eq2_e1037_d_n17, eq2_e1037_d_n18, eq2_e1037_d_b0, eq2_e1037_d_b1, eq2_e1037_d_b2, eq2_e1037_d_b3, eq2_e1037_d_b4, eq2_e1037_d_b5, eq2_e1037_d_b6, eq2_e1037_d_b7, eq2_e1037_d_b8, eq2_e1037_d_b9, eq2_e1037_d_b10, eq2_e1037_d_b11, eq2_e1037_d_b12, eq2_e1037_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.dn[925][18], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8], s.db[925][9], s.db[925][10], s.db[925][11], s.db[925][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_reactive_node_derivatives: [f64; 19] = [eq2_e1039_q_d_n0, eq2_e1039_q_d_n1, eq2_e1039_q_d_n2, eq2_e1039_q_d_n3, eq2_e1039_q_d_n4, eq2_e1039_q_d_n5, eq2_e1039_q_d_n6, eq2_e1039_q_d_n7, eq2_e1039_q_d_n8, eq2_e1039_q_d_n9, eq2_e1039_q_d_n10, eq2_e1039_q_d_n11, eq2_e1039_q_d_n12, eq2_e1039_q_d_n13, eq2_e1039_q_d_n14, eq2_e1039_q_d_n15, eq2_e1039_q_d_n16, eq2_e1039_q_d_n17, eq2_e1039_q_d_n18];let eq2_reactive_branch_derivatives: [f64; 13] = [eq2_e1039_q_d_b0, eq2_e1039_q_d_b1, eq2_e1039_q_d_b2, eq2_e1039_q_d_b3, eq2_e1039_q_d_b4, eq2_e1039_q_d_b5, eq2_e1039_q_d_b6, eq2_e1039_q_d_b7, eq2_e1039_q_d_b8, eq2_e1039_q_d_b9, eq2_e1039_q_d_b10, eq2_e1039_q_d_b11, eq2_e1039_q_d_b12];
        stamper.stamp_current_reactive_dense_local(
            Some(17),
            None,
            &eq2_reactive_node_derivatives,
            &eq2_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
