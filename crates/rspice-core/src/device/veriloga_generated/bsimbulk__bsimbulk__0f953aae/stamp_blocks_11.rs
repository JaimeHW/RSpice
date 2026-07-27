#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq0_e1204,) = {
    if (s.b[896] && s.b[897]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1204;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e1212, eq1_e1212_d_n0, eq1_e1212_d_n1, eq1_e1212_d_n2, eq1_e1212_d_n3, eq1_e1212_d_n4, eq1_e1212_d_n5, eq1_e1212_d_n6, eq1_e1212_d_n7, eq1_e1212_d_n8, eq1_e1212_d_n9, eq1_e1212_d_n10, eq1_e1212_d_n11, eq1_e1212_d_n12, eq1_e1212_d_n13, eq1_e1212_d_n14, eq1_e1212_d_n15, eq1_e1212_d_n16, eq1_e1212_d_b0, eq1_e1212_d_b1, eq1_e1212_d_b2, eq1_e1212_d_b3, eq1_e1212_d_b4, eq1_e1212_d_b5, eq1_e1212_d_b6, eq1_e1212_d_b7, eq1_e1212_d_b8, eq1_e1212_d_b9, eq1_e1212_d_b10, eq1_e1212_d_b11, eq1_e1212_d_b12, eq1_e1212_d_b13,) = {
    if s.b[1538] {
        let eq1_e1208: f64 = (s.v[187] * p[28]);let eq1_e1210: f64 = (eq1_e1208 * s.v[706]);let eq1_e1210_d_n0: f64 = (((s.dn[187][0] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][0]));let eq1_e1210_d_n1: f64 = (((s.dn[187][1] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][1]));let eq1_e1210_d_n2: f64 = (((s.dn[187][2] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][2]));let eq1_e1210_d_n3: f64 = (((s.dn[187][3] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][3]));let eq1_e1210_d_n4: f64 = (((s.dn[187][4] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][4]));let eq1_e1210_d_n5: f64 = (((s.dn[187][5] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][5]));let eq1_e1210_d_n6: f64 = (((s.dn[187][6] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][6]));let eq1_e1210_d_n7: f64 = (((s.dn[187][7] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][7]));let eq1_e1210_d_n8: f64 = (((s.dn[187][8] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][8]));let eq1_e1210_d_n9: f64 = (((s.dn[187][9] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][9]));let eq1_e1210_d_n10: f64 = (((s.dn[187][10] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][10]));let eq1_e1210_d_n11: f64 = (((s.dn[187][11] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][11]));let eq1_e1210_d_n12: f64 = (((s.dn[187][12] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][12]));let eq1_e1210_d_n13: f64 = (((s.dn[187][13] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][13]));let eq1_e1210_d_n14: f64 = (((s.dn[187][14] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][14]));let eq1_e1210_d_n15: f64 = (((s.dn[187][15] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][15]));let eq1_e1210_d_n16: f64 = (((s.dn[187][16] * p[28]) * s.v[706]) + (eq1_e1208 * s.dn[706][16]));let eq1_e1210_d_b0: f64 = (((s.db[187][0] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][0]));let eq1_e1210_d_b1: f64 = (((s.db[187][1] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][1]));let eq1_e1210_d_b2: f64 = (((s.db[187][2] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][2]));let eq1_e1210_d_b3: f64 = (((s.db[187][3] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][3]));let eq1_e1210_d_b4: f64 = (((s.db[187][4] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][4]));let eq1_e1210_d_b5: f64 = (((s.db[187][5] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][5]));let eq1_e1210_d_b6: f64 = (((s.db[187][6] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][6]));let eq1_e1210_d_b7: f64 = (((s.db[187][7] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][7]));let eq1_e1210_d_b8: f64 = (((s.db[187][8] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][8]));let eq1_e1210_d_b9: f64 = (((s.db[187][9] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][9]));let eq1_e1210_d_b10: f64 = (((s.db[187][10] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][10]));let eq1_e1210_d_b11: f64 = (((s.db[187][11] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][11]));let eq1_e1210_d_b12: f64 = (((s.db[187][12] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][12]));let eq1_e1210_d_b13: f64 = (((s.db[187][13] * p[28]) * s.v[706]) + (eq1_e1208 * s.db[706][13]));
        (eq1_e1210, eq1_e1210_d_n0, eq1_e1210_d_n1, eq1_e1210_d_n2, eq1_e1210_d_n3, eq1_e1210_d_n4, eq1_e1210_d_n5, eq1_e1210_d_n6, eq1_e1210_d_n7, eq1_e1210_d_n8, eq1_e1210_d_n9, eq1_e1210_d_n10, eq1_e1210_d_n11, eq1_e1210_d_n12, eq1_e1210_d_n13, eq1_e1210_d_n14, eq1_e1210_d_n15, eq1_e1210_d_n16, eq1_e1210_d_b0, eq1_e1210_d_b1, eq1_e1210_d_b2, eq1_e1210_d_b3, eq1_e1210_d_b4, eq1_e1210_d_b5, eq1_e1210_d_b6, eq1_e1210_d_b7, eq1_e1210_d_b8, eq1_e1210_d_b9, eq1_e1210_d_b10, eq1_e1210_d_b11, eq1_e1210_d_b12, eq1_e1210_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1212;let eq1_node_derivatives: [f64; 17] = [eq1_e1212_d_n0, eq1_e1212_d_n1, eq1_e1212_d_n2, eq1_e1212_d_n3, eq1_e1212_d_n4, eq1_e1212_d_n5, eq1_e1212_d_n6, eq1_e1212_d_n7, eq1_e1212_d_n8, eq1_e1212_d_n9, eq1_e1212_d_n10, eq1_e1212_d_n11, eq1_e1212_d_n12, eq1_e1212_d_n13, eq1_e1212_d_n14, eq1_e1212_d_n15, eq1_e1212_d_n16];let eq1_branch_derivatives: [f64; 14] = [eq1_e1212_d_b0, eq1_e1212_d_b1, eq1_e1212_d_b2, eq1_e1212_d_b3, eq1_e1212_d_b4, eq1_e1212_d_b5, eq1_e1212_d_b6, eq1_e1212_d_b7, eq1_e1212_d_b8, eq1_e1212_d_b9, eq1_e1212_d_b10, eq1_e1212_d_b11, eq1_e1212_d_b12, eq1_e1212_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1277, eq7_e1277_d_n0, eq7_e1277_d_n1, eq7_e1277_d_n2, eq7_e1277_d_n3, eq7_e1277_d_n4, eq7_e1277_d_n5, eq7_e1277_d_n6, eq7_e1277_d_n7, eq7_e1277_d_n8, eq7_e1277_d_n9, eq7_e1277_d_n10, eq7_e1277_d_n11, eq7_e1277_d_n12, eq7_e1277_d_n13, eq7_e1277_d_n14, eq7_e1277_d_n15, eq7_e1277_d_n16, eq7_e1277_d_b0, eq7_e1277_d_b1, eq7_e1277_d_b2, eq7_e1277_d_b3, eq7_e1277_d_b4, eq7_e1277_d_b5, eq7_e1277_d_b6, eq7_e1277_d_b7, eq7_e1277_d_b8, eq7_e1277_d_b9, eq7_e1277_d_b10, eq7_e1277_d_b11, eq7_e1277_d_b12, eq7_e1277_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq7_e1273: f64 = (-s.v[385]);let eq7_e1275: f64 = (eq7_e1273 * (nv16 - 0.0));let eq7_e1275_d_n0: f64 = ((-s.dn[385][0]) * (nv16 - 0.0));let eq7_e1275_d_n1: f64 = ((-s.dn[385][1]) * (nv16 - 0.0));let eq7_e1275_d_n2: f64 = ((-s.dn[385][2]) * (nv16 - 0.0));let eq7_e1275_d_n3: f64 = ((-s.dn[385][3]) * (nv16 - 0.0));let eq7_e1275_d_n4: f64 = ((-s.dn[385][4]) * (nv16 - 0.0));let eq7_e1275_d_n5: f64 = ((-s.dn[385][5]) * (nv16 - 0.0));let eq7_e1275_d_n6: f64 = ((-s.dn[385][6]) * (nv16 - 0.0));let eq7_e1275_d_n7: f64 = ((-s.dn[385][7]) * (nv16 - 0.0));let eq7_e1275_d_n8: f64 = ((-s.dn[385][8]) * (nv16 - 0.0));let eq7_e1275_d_n9: f64 = ((-s.dn[385][9]) * (nv16 - 0.0));let eq7_e1275_d_n10: f64 = ((-s.dn[385][10]) * (nv16 - 0.0));let eq7_e1275_d_n11: f64 = ((-s.dn[385][11]) * (nv16 - 0.0));let eq7_e1275_d_n12: f64 = ((-s.dn[385][12]) * (nv16 - 0.0));let eq7_e1275_d_n13: f64 = ((-s.dn[385][13]) * (nv16 - 0.0));let eq7_e1275_d_n14: f64 = ((-s.dn[385][14]) * (nv16 - 0.0));let eq7_e1275_d_n15: f64 = ((-s.dn[385][15]) * (nv16 - 0.0));let eq7_e1275_d_n16: f64 = (((-s.dn[385][16]) * (nv16 - 0.0)) + eq7_e1273);let eq7_e1275_d_b0: f64 = ((-s.db[385][0]) * (nv16 - 0.0));let eq7_e1275_d_b1: f64 = ((-s.db[385][1]) * (nv16 - 0.0));let eq7_e1275_d_b2: f64 = ((-s.db[385][2]) * (nv16 - 0.0));let eq7_e1275_d_b3: f64 = ((-s.db[385][3]) * (nv16 - 0.0));let eq7_e1275_d_b4: f64 = ((-s.db[385][4]) * (nv16 - 0.0));let eq7_e1275_d_b5: f64 = ((-s.db[385][5]) * (nv16 - 0.0));let eq7_e1275_d_b6: f64 = ((-s.db[385][6]) * (nv16 - 0.0));let eq7_e1275_d_b7: f64 = ((-s.db[385][7]) * (nv16 - 0.0));let eq7_e1275_d_b8: f64 = ((-s.db[385][8]) * (nv16 - 0.0));let eq7_e1275_d_b9: f64 = ((-s.db[385][9]) * (nv16 - 0.0));let eq7_e1275_d_b10: f64 = ((-s.db[385][10]) * (nv16 - 0.0));let eq7_e1275_d_b11: f64 = ((-s.db[385][11]) * (nv16 - 0.0));let eq7_e1275_d_b12: f64 = ((-s.db[385][12]) * (nv16 - 0.0));let eq7_e1275_d_b13: f64 = ((-s.db[385][13]) * (nv16 - 0.0));
        (eq7_e1275, eq7_e1275_d_n0, eq7_e1275_d_n1, eq7_e1275_d_n2, eq7_e1275_d_n3, eq7_e1275_d_n4, eq7_e1275_d_n5, eq7_e1275_d_n6, eq7_e1275_d_n7, eq7_e1275_d_n8, eq7_e1275_d_n9, eq7_e1275_d_n10, eq7_e1275_d_n11, eq7_e1275_d_n12, eq7_e1275_d_n13, eq7_e1275_d_n14, eq7_e1275_d_n15, eq7_e1275_d_n16, eq7_e1275_d_b0, eq7_e1275_d_b1, eq7_e1275_d_b2, eq7_e1275_d_b3, eq7_e1275_d_b4, eq7_e1275_d_b5, eq7_e1275_d_b6, eq7_e1275_d_b7, eq7_e1275_d_b8, eq7_e1275_d_b9, eq7_e1275_d_b10, eq7_e1275_d_b11, eq7_e1275_d_b12, eq7_e1275_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1277;let eq7_node_derivatives: [f64; 17] = [eq7_e1277_d_n0, eq7_e1277_d_n1, eq7_e1277_d_n2, eq7_e1277_d_n3, eq7_e1277_d_n4, eq7_e1277_d_n5, eq7_e1277_d_n6, eq7_e1277_d_n7, eq7_e1277_d_n8, eq7_e1277_d_n9, eq7_e1277_d_n10, eq7_e1277_d_n11, eq7_e1277_d_n12, eq7_e1277_d_n13, eq7_e1277_d_n14, eq7_e1277_d_n15, eq7_e1277_d_n16];let eq7_branch_derivatives: [f64; 14] = [eq7_e1277_d_b0, eq7_e1277_d_b1, eq7_e1277_d_b2, eq7_e1277_d_b3, eq7_e1277_d_b4, eq7_e1277_d_b5, eq7_e1277_d_b6, eq7_e1277_d_b7, eq7_e1277_d_b8, eq7_e1277_d_b9, eq7_e1277_d_b10, eq7_e1277_d_b11, eq7_e1277_d_b12, eq7_e1277_d_b13];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1295, eq8_e1295_d_n0, eq8_e1295_d_n1, eq8_e1295_d_n2, eq8_e1295_d_n3, eq8_e1295_d_n4, eq8_e1295_d_n5, eq8_e1295_d_n6, eq8_e1295_d_n7, eq8_e1295_d_n8, eq8_e1295_d_n9, eq8_e1295_d_n10, eq8_e1295_d_n11, eq8_e1295_d_n12, eq8_e1295_d_n13, eq8_e1295_d_n14, eq8_e1295_d_n15, eq8_e1295_d_n16, eq8_e1295_d_b0, eq8_e1295_d_b1, eq8_e1295_d_b2, eq8_e1295_d_b3, eq8_e1295_d_b4, eq8_e1295_d_b5, eq8_e1295_d_b6, eq8_e1295_d_b7, eq8_e1295_d_b8, eq8_e1295_d_b9, eq8_e1295_d_b10, eq8_e1295_d_b11, eq8_e1295_d_b12, eq8_e1295_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq8_e1284: f64 = (s.v[378] * s.v[46]);let eq8_e1286: f64 = (eq8_e1284 * s.v[29]);let eq8_e1286_d_n0: f64 = ((s.dn[378][0] * s.v[46]) * s.v[29]);let eq8_e1286_d_n1: f64 = ((s.dn[378][1] * s.v[46]) * s.v[29]);let eq8_e1286_d_n2: f64 = ((s.dn[378][2] * s.v[46]) * s.v[29]);let eq8_e1286_d_n3: f64 = ((s.dn[378][3] * s.v[46]) * s.v[29]);let eq8_e1286_d_n4: f64 = ((s.dn[378][4] * s.v[46]) * s.v[29]);let eq8_e1286_d_n5: f64 = ((s.dn[378][5] * s.v[46]) * s.v[29]);let eq8_e1286_d_n6: f64 = ((s.dn[378][6] * s.v[46]) * s.v[29]);let eq8_e1286_d_n7: f64 = ((s.dn[378][7] * s.v[46]) * s.v[29]);let eq8_e1286_d_n8: f64 = ((s.dn[378][8] * s.v[46]) * s.v[29]);let eq8_e1286_d_n9: f64 = ((s.dn[378][9] * s.v[46]) * s.v[29]);let eq8_e1286_d_n10: f64 = ((s.dn[378][10] * s.v[46]) * s.v[29]);let eq8_e1286_d_n11: f64 = ((s.dn[378][11] * s.v[46]) * s.v[29]);let eq8_e1286_d_n12: f64 = ((s.dn[378][12] * s.v[46]) * s.v[29]);let eq8_e1286_d_n13: f64 = ((s.dn[378][13] * s.v[46]) * s.v[29]);let eq8_e1286_d_n14: f64 = ((s.dn[378][14] * s.v[46]) * s.v[29]);let eq8_e1286_d_n15: f64 = ((s.dn[378][15] * s.v[46]) * s.v[29]);let eq8_e1286_d_n16: f64 = ((s.dn[378][16] * s.v[46]) * s.v[29]);let eq8_e1286_d_b0: f64 = ((s.db[378][0] * s.v[46]) * s.v[29]);let eq8_e1286_d_b1: f64 = ((s.db[378][1] * s.v[46]) * s.v[29]);let eq8_e1286_d_b2: f64 = ((s.db[378][2] * s.v[46]) * s.v[29]);let eq8_e1286_d_b3: f64 = ((s.db[378][3] * s.v[46]) * s.v[29]);let eq8_e1286_d_b4: f64 = ((s.db[378][4] * s.v[46]) * s.v[29]);let eq8_e1286_d_b5: f64 = ((s.db[378][5] * s.v[46]) * s.v[29]);let eq8_e1286_d_b6: f64 = ((s.db[378][6] * s.v[46]) * s.v[29]);let eq8_e1286_d_b7: f64 = ((s.db[378][7] * s.v[46]) * s.v[29]);let eq8_e1286_d_b8: f64 = ((s.db[378][8] * s.v[46]) * s.v[29]);let eq8_e1286_d_b9: f64 = ((s.db[378][9] * s.v[46]) * s.v[29]);let eq8_e1286_d_b10: f64 = ((s.db[378][10] * s.v[46]) * s.v[29]);let eq8_e1286_d_b11: f64 = ((s.db[378][11] * s.v[46]) * s.v[29]);let eq8_e1286_d_b12: f64 = ((s.db[378][12] * s.v[46]) * s.v[29]);let eq8_e1286_d_b13: f64 = ((s.db[378][13] * s.v[46]) * s.v[29]);let eq8_e1288: f64 = (eq8_e1286 * p[2]);let eq8_e1288_d_n0: f64 = (eq8_e1286_d_n0 * p[2]);let eq8_e1288_d_n1: f64 = (eq8_e1286_d_n1 * p[2]);let eq8_e1288_d_n2: f64 = (eq8_e1286_d_n2 * p[2]);let eq8_e1288_d_n3: f64 = (eq8_e1286_d_n3 * p[2]);let eq8_e1288_d_n4: f64 = (eq8_e1286_d_n4 * p[2]);let eq8_e1288_d_n5: f64 = (eq8_e1286_d_n5 * p[2]);let eq8_e1288_d_n6: f64 = (eq8_e1286_d_n6 * p[2]);let eq8_e1288_d_n7: f64 = (eq8_e1286_d_n7 * p[2]);let eq8_e1288_d_n8: f64 = (eq8_e1286_d_n8 * p[2]);let eq8_e1288_d_n9: f64 = (eq8_e1286_d_n9 * p[2]);let eq8_e1288_d_n10: f64 = (eq8_e1286_d_n10 * p[2]);let eq8_e1288_d_n11: f64 = (eq8_e1286_d_n11 * p[2]);let eq8_e1288_d_n12: f64 = (eq8_e1286_d_n12 * p[2]);let eq8_e1288_d_n13: f64 = (eq8_e1286_d_n13 * p[2]);let eq8_e1288_d_n14: f64 = (eq8_e1286_d_n14 * p[2]);let eq8_e1288_d_n15: f64 = (eq8_e1286_d_n15 * p[2]);let eq8_e1288_d_n16: f64 = (eq8_e1286_d_n16 * p[2]);let eq8_e1288_d_b0: f64 = (eq8_e1286_d_b0 * p[2]);let eq8_e1288_d_b1: f64 = (eq8_e1286_d_b1 * p[2]);let eq8_e1288_d_b2: f64 = (eq8_e1286_d_b2 * p[2]);let eq8_e1288_d_b3: f64 = (eq8_e1286_d_b3 * p[2]);let eq8_e1288_d_b4: f64 = (eq8_e1286_d_b4 * p[2]);let eq8_e1288_d_b5: f64 = (eq8_e1286_d_b5 * p[2]);let eq8_e1288_d_b6: f64 = (eq8_e1286_d_b6 * p[2]);let eq8_e1288_d_b7: f64 = (eq8_e1286_d_b7 * p[2]);let eq8_e1288_d_b8: f64 = (eq8_e1286_d_b8 * p[2]);let eq8_e1288_d_b9: f64 = (eq8_e1286_d_b9 * p[2]);let eq8_e1288_d_b10: f64 = (eq8_e1286_d_b10 * p[2]);let eq8_e1288_d_b11: f64 = (eq8_e1286_d_b11 * p[2]);let eq8_e1288_d_b12: f64 = (eq8_e1286_d_b12 * p[2]);let eq8_e1288_d_b13: f64 = (eq8_e1286_d_b13 * p[2]);let eq8_e1290: f64 = (eq8_e1288 * s.v[30]);let eq8_e1290_d_n0: f64 = (eq8_e1288_d_n0 * s.v[30]);let eq8_e1290_d_n1: f64 = (eq8_e1288_d_n1 * s.v[30]);let eq8_e1290_d_n2: f64 = (eq8_e1288_d_n2 * s.v[30]);let eq8_e1290_d_n3: f64 = (eq8_e1288_d_n3 * s.v[30]);let eq8_e1290_d_n4: f64 = (eq8_e1288_d_n4 * s.v[30]);let eq8_e1290_d_n5: f64 = (eq8_e1288_d_n5 * s.v[30]);let eq8_e1290_d_n6: f64 = (eq8_e1288_d_n6 * s.v[30]);
        let eq8_e1290_d_n7: f64 = (eq8_e1288_d_n7 * s.v[30]);let eq8_e1290_d_n8: f64 = (eq8_e1288_d_n8 * s.v[30]);let eq8_e1290_d_n9: f64 = (eq8_e1288_d_n9 * s.v[30]);let eq8_e1290_d_n10: f64 = (eq8_e1288_d_n10 * s.v[30]);let eq8_e1290_d_n11: f64 = (eq8_e1288_d_n11 * s.v[30]);let eq8_e1290_d_n12: f64 = (eq8_e1288_d_n12 * s.v[30]);let eq8_e1290_d_n13: f64 = (eq8_e1288_d_n13 * s.v[30]);let eq8_e1290_d_n14: f64 = (eq8_e1288_d_n14 * s.v[30]);let eq8_e1290_d_n15: f64 = (eq8_e1288_d_n15 * s.v[30]);let eq8_e1290_d_n16: f64 = (eq8_e1288_d_n16 * s.v[30]);let eq8_e1290_d_b0: f64 = (eq8_e1288_d_b0 * s.v[30]);let eq8_e1290_d_b1: f64 = (eq8_e1288_d_b1 * s.v[30]);let eq8_e1290_d_b2: f64 = (eq8_e1288_d_b2 * s.v[30]);let eq8_e1290_d_b3: f64 = (eq8_e1288_d_b3 * s.v[30]);let eq8_e1290_d_b4: f64 = (eq8_e1288_d_b4 * s.v[30]);let eq8_e1290_d_b5: f64 = (eq8_e1288_d_b5 * s.v[30]);let eq8_e1290_d_b6: f64 = (eq8_e1288_d_b6 * s.v[30]);let eq8_e1290_d_b7: f64 = (eq8_e1288_d_b7 * s.v[30]);let eq8_e1290_d_b8: f64 = (eq8_e1288_d_b8 * s.v[30]);let eq8_e1290_d_b9: f64 = (eq8_e1288_d_b9 * s.v[30]);let eq8_e1290_d_b10: f64 = (eq8_e1288_d_b10 * s.v[30]);let eq8_e1290_d_b11: f64 = (eq8_e1288_d_b11 * s.v[30]);let eq8_e1290_d_b12: f64 = (eq8_e1288_d_b12 * s.v[30]);let eq8_e1290_d_b13: f64 = (eq8_e1288_d_b13 * s.v[30]);let eq8_e1292: f64 = (eq8_e1290 * (nv15 - 0.0));let eq8_e1292_d_n0: f64 = (eq8_e1290_d_n0 * (nv15 - 0.0));let eq8_e1292_d_n1: f64 = (eq8_e1290_d_n1 * (nv15 - 0.0));let eq8_e1292_d_n2: f64 = (eq8_e1290_d_n2 * (nv15 - 0.0));let eq8_e1292_d_n3: f64 = (eq8_e1290_d_n3 * (nv15 - 0.0));let eq8_e1292_d_n4: f64 = (eq8_e1290_d_n4 * (nv15 - 0.0));let eq8_e1292_d_n5: f64 = (eq8_e1290_d_n5 * (nv15 - 0.0));let eq8_e1292_d_n6: f64 = (eq8_e1290_d_n6 * (nv15 - 0.0));let eq8_e1292_d_n7: f64 = (eq8_e1290_d_n7 * (nv15 - 0.0));let eq8_e1292_d_n8: f64 = (eq8_e1290_d_n8 * (nv15 - 0.0));let eq8_e1292_d_n9: f64 = (eq8_e1290_d_n9 * (nv15 - 0.0));let eq8_e1292_d_n10: f64 = (eq8_e1290_d_n10 * (nv15 - 0.0));let eq8_e1292_d_n11: f64 = (eq8_e1290_d_n11 * (nv15 - 0.0));let eq8_e1292_d_n12: f64 = (eq8_e1290_d_n12 * (nv15 - 0.0));let eq8_e1292_d_n13: f64 = (eq8_e1290_d_n13 * (nv15 - 0.0));let eq8_e1292_d_n14: f64 = (eq8_e1290_d_n14 * (nv15 - 0.0));let eq8_e1292_d_n15: f64 = ((eq8_e1290_d_n15 * (nv15 - 0.0)) + eq8_e1290);let eq8_e1292_d_n16: f64 = (eq8_e1290_d_n16 * (nv15 - 0.0));let eq8_e1292_d_b0: f64 = (eq8_e1290_d_b0 * (nv15 - 0.0));let eq8_e1292_d_b1: f64 = (eq8_e1290_d_b1 * (nv15 - 0.0));let eq8_e1292_d_b2: f64 = (eq8_e1290_d_b2 * (nv15 - 0.0));let eq8_e1292_d_b3: f64 = (eq8_e1290_d_b3 * (nv15 - 0.0));let eq8_e1292_d_b4: f64 = (eq8_e1290_d_b4 * (nv15 - 0.0));let eq8_e1292_d_b5: f64 = (eq8_e1290_d_b5 * (nv15 - 0.0));let eq8_e1292_d_b6: f64 = (eq8_e1290_d_b6 * (nv15 - 0.0));let eq8_e1292_d_b7: f64 = (eq8_e1290_d_b7 * (nv15 - 0.0));let eq8_e1292_d_b8: f64 = (eq8_e1290_d_b8 * (nv15 - 0.0));let eq8_e1292_d_b9: f64 = (eq8_e1290_d_b9 * (nv15 - 0.0));let eq8_e1292_d_b10: f64 = (eq8_e1290_d_b10 * (nv15 - 0.0));let eq8_e1292_d_b11: f64 = (eq8_e1290_d_b11 * (nv15 - 0.0));let eq8_e1292_d_b12: f64 = (eq8_e1290_d_b12 * (nv15 - 0.0));let eq8_e1292_d_b13: f64 = (eq8_e1290_d_b13 * (nv15 - 0.0));let eq8_e1293: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq8_e1292);
        (eq8_e1293, (eq8_e1292_d_n0 * ddt_scale), (eq8_e1292_d_n1 * ddt_scale), (eq8_e1292_d_n2 * ddt_scale), (eq8_e1292_d_n3 * ddt_scale), (eq8_e1292_d_n4 * ddt_scale), (eq8_e1292_d_n5 * ddt_scale), (eq8_e1292_d_n6 * ddt_scale), (eq8_e1292_d_n7 * ddt_scale), (eq8_e1292_d_n8 * ddt_scale), (eq8_e1292_d_n9 * ddt_scale), (eq8_e1292_d_n10 * ddt_scale), (eq8_e1292_d_n11 * ddt_scale), (eq8_e1292_d_n12 * ddt_scale), (eq8_e1292_d_n13 * ddt_scale), (eq8_e1292_d_n14 * ddt_scale), (eq8_e1292_d_n15 * ddt_scale), (eq8_e1292_d_n16 * ddt_scale), (eq8_e1292_d_b0 * ddt_scale), (eq8_e1292_d_b1 * ddt_scale), (eq8_e1292_d_b2 * ddt_scale), (eq8_e1292_d_b3 * ddt_scale), (eq8_e1292_d_b4 * ddt_scale), (eq8_e1292_d_b5 * ddt_scale), (eq8_e1292_d_b6 * ddt_scale), (eq8_e1292_d_b7 * ddt_scale), (eq8_e1292_d_b8 * ddt_scale), (eq8_e1292_d_b9 * ddt_scale), (eq8_e1292_d_b10 * ddt_scale), (eq8_e1292_d_b11 * ddt_scale), (eq8_e1292_d_b12 * ddt_scale), (eq8_e1292_d_b13 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e1295;let eq8_node_derivatives: [f64; 17] = [eq8_e1295_d_n0, eq8_e1295_d_n1, eq8_e1295_d_n2, eq8_e1295_d_n3, eq8_e1295_d_n4, eq8_e1295_d_n5, eq8_e1295_d_n6, eq8_e1295_d_n7, eq8_e1295_d_n8, eq8_e1295_d_n9, eq8_e1295_d_n10, eq8_e1295_d_n11, eq8_e1295_d_n12, eq8_e1295_d_n13, eq8_e1295_d_n14, eq8_e1295_d_n15, eq8_e1295_d_n16];let eq8_branch_derivatives: [f64; 14] = [eq8_e1295_d_b0, eq8_e1295_d_b1, eq8_e1295_d_b2, eq8_e1295_d_b3, eq8_e1295_d_b4, eq8_e1295_d_b5, eq8_e1295_d_b6, eq8_e1295_d_b7, eq8_e1295_d_b8, eq8_e1295_d_b9, eq8_e1295_d_b10, eq8_e1295_d_b11, eq8_e1295_d_b12, eq8_e1295_d_b13];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq10_e1323, eq10_e1323_d_n0, eq10_e1323_d_n1, eq10_e1323_d_n2, eq10_e1323_d_n3, eq10_e1323_d_n4, eq10_e1323_d_n5, eq10_e1323_d_n6, eq10_e1323_d_n7, eq10_e1323_d_n8, eq10_e1323_d_n9, eq10_e1323_d_n10, eq10_e1323_d_n11, eq10_e1323_d_n12, eq10_e1323_d_n13, eq10_e1323_d_n14, eq10_e1323_d_n15, eq10_e1323_d_n16, eq10_e1323_d_b0, eq10_e1323_d_b1, eq10_e1323_d_b2, eq10_e1323_d_b3, eq10_e1323_d_b4, eq10_e1323_d_b5, eq10_e1323_d_b6, eq10_e1323_d_b7, eq10_e1323_d_b8, eq10_e1323_d_b9, eq10_e1323_d_b10, eq10_e1323_d_b11, eq10_e1323_d_b12, eq10_e1323_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq10_e1319: f64 = (s.v[384] * p[28]);let eq10_e1321: f64 = (eq10_e1319 * (nv16 - 0.0));let eq10_e1321_d_n0: f64 = ((s.dn[384][0] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n1: f64 = ((s.dn[384][1] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n2: f64 = ((s.dn[384][2] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n3: f64 = ((s.dn[384][3] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n4: f64 = ((s.dn[384][4] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n5: f64 = ((s.dn[384][5] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n6: f64 = ((s.dn[384][6] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n7: f64 = ((s.dn[384][7] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n8: f64 = ((s.dn[384][8] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n9: f64 = ((s.dn[384][9] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n10: f64 = ((s.dn[384][10] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n11: f64 = ((s.dn[384][11] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n12: f64 = ((s.dn[384][12] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n13: f64 = ((s.dn[384][13] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n14: f64 = ((s.dn[384][14] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n15: f64 = ((s.dn[384][15] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_n16: f64 = (((s.dn[384][16] * p[28]) * (nv16 - 0.0)) + eq10_e1319);let eq10_e1321_d_b0: f64 = ((s.db[384][0] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b1: f64 = ((s.db[384][1] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b2: f64 = ((s.db[384][2] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b3: f64 = ((s.db[384][3] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b4: f64 = ((s.db[384][4] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b5: f64 = ((s.db[384][5] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b6: f64 = ((s.db[384][6] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b7: f64 = ((s.db[384][7] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b8: f64 = ((s.db[384][8] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b9: f64 = ((s.db[384][9] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b10: f64 = ((s.db[384][10] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b11: f64 = ((s.db[384][11] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b12: f64 = ((s.db[384][12] * p[28]) * (nv16 - 0.0));let eq10_e1321_d_b13: f64 = ((s.db[384][13] * p[28]) * (nv16 - 0.0));
        (eq10_e1321, eq10_e1321_d_n0, eq10_e1321_d_n1, eq10_e1321_d_n2, eq10_e1321_d_n3, eq10_e1321_d_n4, eq10_e1321_d_n5, eq10_e1321_d_n6, eq10_e1321_d_n7, eq10_e1321_d_n8, eq10_e1321_d_n9, eq10_e1321_d_n10, eq10_e1321_d_n11, eq10_e1321_d_n12, eq10_e1321_d_n13, eq10_e1321_d_n14, eq10_e1321_d_n15, eq10_e1321_d_n16, eq10_e1321_d_b0, eq10_e1321_d_b1, eq10_e1321_d_b2, eq10_e1321_d_b3, eq10_e1321_d_b4, eq10_e1321_d_b5, eq10_e1321_d_b6, eq10_e1321_d_b7, eq10_e1321_d_b8, eq10_e1321_d_b9, eq10_e1321_d_b10, eq10_e1321_d_b11, eq10_e1321_d_b12, eq10_e1321_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1323;let eq10_node_derivatives: [f64; 17] = [eq10_e1323_d_n0, eq10_e1323_d_n1, eq10_e1323_d_n2, eq10_e1323_d_n3, eq10_e1323_d_n4, eq10_e1323_d_n5, eq10_e1323_d_n6, eq10_e1323_d_n7, eq10_e1323_d_n8, eq10_e1323_d_n9, eq10_e1323_d_n10, eq10_e1323_d_n11, eq10_e1323_d_n12, eq10_e1323_d_n13, eq10_e1323_d_n14, eq10_e1323_d_n15, eq10_e1323_d_n16];let eq10_branch_derivatives: [f64; 14] = [eq10_e1323_d_b0, eq10_e1323_d_b1, eq10_e1323_d_b2, eq10_e1323_d_b3, eq10_e1323_d_b4, eq10_e1323_d_b5, eq10_e1323_d_b6, eq10_e1323_d_b7, eq10_e1323_d_b8, eq10_e1323_d_b9, eq10_e1323_d_b10, eq10_e1323_d_b11, eq10_e1323_d_b12, eq10_e1323_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq11_e1349, eq11_e1349_d_n0, eq11_e1349_d_n1, eq11_e1349_d_n2, eq11_e1349_d_n3, eq11_e1349_d_n4, eq11_e1349_d_n5, eq11_e1349_d_n6, eq11_e1349_d_n7, eq11_e1349_d_n8, eq11_e1349_d_n9, eq11_e1349_d_n10, eq11_e1349_d_n11, eq11_e1349_d_n12, eq11_e1349_d_n13, eq11_e1349_d_n14, eq11_e1349_d_n15, eq11_e1349_d_n16, eq11_e1349_d_b0, eq11_e1349_d_b1, eq11_e1349_d_b2, eq11_e1349_d_b3, eq11_e1349_d_b4, eq11_e1349_d_b5, eq11_e1349_d_b6, eq11_e1349_d_b7, eq11_e1349_d_b8, eq11_e1349_d_b9, eq11_e1349_d_b10, eq11_e1349_d_b11, eq11_e1349_d_b12, eq11_e1349_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq11_e1332: f64 = (1.0 + s.v[57]);let eq11_e1334: f64 = (eq11_e1332 * s.v[378]);let eq11_e1334_d_n0: f64 = ((s.dn[57][0] * s.v[378]) + (eq11_e1332 * s.dn[378][0]));let eq11_e1334_d_n1: f64 = ((s.dn[57][1] * s.v[378]) + (eq11_e1332 * s.dn[378][1]));let eq11_e1334_d_n2: f64 = ((s.dn[57][2] * s.v[378]) + (eq11_e1332 * s.dn[378][2]));let eq11_e1334_d_n3: f64 = ((s.dn[57][3] * s.v[378]) + (eq11_e1332 * s.dn[378][3]));let eq11_e1334_d_n4: f64 = ((s.dn[57][4] * s.v[378]) + (eq11_e1332 * s.dn[378][4]));let eq11_e1334_d_n5: f64 = ((s.dn[57][5] * s.v[378]) + (eq11_e1332 * s.dn[378][5]));let eq11_e1334_d_n6: f64 = ((s.dn[57][6] * s.v[378]) + (eq11_e1332 * s.dn[378][6]));let eq11_e1334_d_n7: f64 = ((s.dn[57][7] * s.v[378]) + (eq11_e1332 * s.dn[378][7]));let eq11_e1334_d_n8: f64 = ((s.dn[57][8] * s.v[378]) + (eq11_e1332 * s.dn[378][8]));let eq11_e1334_d_n9: f64 = ((s.dn[57][9] * s.v[378]) + (eq11_e1332 * s.dn[378][9]));let eq11_e1334_d_n10: f64 = ((s.dn[57][10] * s.v[378]) + (eq11_e1332 * s.dn[378][10]));let eq11_e1334_d_n11: f64 = ((s.dn[57][11] * s.v[378]) + (eq11_e1332 * s.dn[378][11]));let eq11_e1334_d_n12: f64 = ((s.dn[57][12] * s.v[378]) + (eq11_e1332 * s.dn[378][12]));let eq11_e1334_d_n13: f64 = ((s.dn[57][13] * s.v[378]) + (eq11_e1332 * s.dn[378][13]));let eq11_e1334_d_n14: f64 = ((s.dn[57][14] * s.v[378]) + (eq11_e1332 * s.dn[378][14]));let eq11_e1334_d_n15: f64 = ((s.dn[57][15] * s.v[378]) + (eq11_e1332 * s.dn[378][15]));let eq11_e1334_d_n16: f64 = ((s.dn[57][16] * s.v[378]) + (eq11_e1332 * s.dn[378][16]));let eq11_e1334_d_b0: f64 = ((s.db[57][0] * s.v[378]) + (eq11_e1332 * s.db[378][0]));let eq11_e1334_d_b1: f64 = ((s.db[57][1] * s.v[378]) + (eq11_e1332 * s.db[378][1]));let eq11_e1334_d_b2: f64 = ((s.db[57][2] * s.v[378]) + (eq11_e1332 * s.db[378][2]));let eq11_e1334_d_b3: f64 = ((s.db[57][3] * s.v[378]) + (eq11_e1332 * s.db[378][3]));let eq11_e1334_d_b4: f64 = ((s.db[57][4] * s.v[378]) + (eq11_e1332 * s.db[378][4]));let eq11_e1334_d_b5: f64 = ((s.db[57][5] * s.v[378]) + (eq11_e1332 * s.db[378][5]));let eq11_e1334_d_b6: f64 = ((s.db[57][6] * s.v[378]) + (eq11_e1332 * s.db[378][6]));let eq11_e1334_d_b7: f64 = ((s.db[57][7] * s.v[378]) + (eq11_e1332 * s.db[378][7]));let eq11_e1334_d_b8: f64 = ((s.db[57][8] * s.v[378]) + (eq11_e1332 * s.db[378][8]));let eq11_e1334_d_b9: f64 = ((s.db[57][9] * s.v[378]) + (eq11_e1332 * s.db[378][9]));let eq11_e1334_d_b10: f64 = ((s.db[57][10] * s.v[378]) + (eq11_e1332 * s.db[378][10]));let eq11_e1334_d_b11: f64 = ((s.db[57][11] * s.v[378]) + (eq11_e1332 * s.db[378][11]));let eq11_e1334_d_b12: f64 = ((s.db[57][12] * s.v[378]) + (eq11_e1332 * s.db[378][12]));let eq11_e1334_d_b13: f64 = ((s.db[57][13] * s.v[378]) + (eq11_e1332 * s.db[378][13]));let eq11_e1336: f64 = (eq11_e1334 * s.v[46]);let eq11_e1336_d_n0: f64 = (eq11_e1334_d_n0 * s.v[46]);let eq11_e1336_d_n1: f64 = (eq11_e1334_d_n1 * s.v[46]);let eq11_e1336_d_n2: f64 = (eq11_e1334_d_n2 * s.v[46]);let eq11_e1336_d_n3: f64 = (eq11_e1334_d_n3 * s.v[46]);let eq11_e1336_d_n4: f64 = (eq11_e1334_d_n4 * s.v[46]);let eq11_e1336_d_n5: f64 = (eq11_e1334_d_n5 * s.v[46]);let eq11_e1336_d_n6: f64 = (eq11_e1334_d_n6 * s.v[46]);let eq11_e1336_d_n7: f64 = (eq11_e1334_d_n7 * s.v[46]);let eq11_e1336_d_n8: f64 = (eq11_e1334_d_n8 * s.v[46]);let eq11_e1336_d_n9: f64 = (eq11_e1334_d_n9 * s.v[46]);let eq11_e1336_d_n10: f64 = (eq11_e1334_d_n10 * s.v[46]);let eq11_e1336_d_n11: f64 = (eq11_e1334_d_n11 * s.v[46]);let eq11_e1336_d_n12: f64 = (eq11_e1334_d_n12 * s.v[46]);let eq11_e1336_d_n13: f64 = (eq11_e1334_d_n13 * s.v[46]);let eq11_e1336_d_n14: f64 = (eq11_e1334_d_n14 * s.v[46]);let eq11_e1336_d_n15: f64 = (eq11_e1334_d_n15 * s.v[46]);let eq11_e1336_d_n16: f64 = (eq11_e1334_d_n16 * s.v[46]);let eq11_e1336_d_b0: f64 = (eq11_e1334_d_b0 * s.v[46]);let eq11_e1336_d_b1: f64 = (eq11_e1334_d_b1 * s.v[46]);let eq11_e1336_d_b2: f64 = (eq11_e1334_d_b2 * s.v[46]);let eq11_e1336_d_b3: f64 = (eq11_e1334_d_b3 * s.v[46]);let eq11_e1336_d_b4: f64 = (eq11_e1334_d_b4 * s.v[46]);let eq11_e1336_d_b5: f64 = (eq11_e1334_d_b5 * s.v[46]);
        let eq11_e1336_d_b6: f64 = (eq11_e1334_d_b6 * s.v[46]);let eq11_e1336_d_b7: f64 = (eq11_e1334_d_b7 * s.v[46]);let eq11_e1336_d_b8: f64 = (eq11_e1334_d_b8 * s.v[46]);let eq11_e1336_d_b9: f64 = (eq11_e1334_d_b9 * s.v[46]);let eq11_e1336_d_b10: f64 = (eq11_e1334_d_b10 * s.v[46]);let eq11_e1336_d_b11: f64 = (eq11_e1334_d_b11 * s.v[46]);let eq11_e1336_d_b12: f64 = (eq11_e1334_d_b12 * s.v[46]);let eq11_e1336_d_b13: f64 = (eq11_e1334_d_b13 * s.v[46]);let eq11_e1338: f64 = (eq11_e1336 * s.v[29]);let eq11_e1338_d_n0: f64 = (eq11_e1336_d_n0 * s.v[29]);let eq11_e1338_d_n1: f64 = (eq11_e1336_d_n1 * s.v[29]);let eq11_e1338_d_n2: f64 = (eq11_e1336_d_n2 * s.v[29]);let eq11_e1338_d_n3: f64 = (eq11_e1336_d_n3 * s.v[29]);let eq11_e1338_d_n4: f64 = (eq11_e1336_d_n4 * s.v[29]);let eq11_e1338_d_n5: f64 = (eq11_e1336_d_n5 * s.v[29]);let eq11_e1338_d_n6: f64 = (eq11_e1336_d_n6 * s.v[29]);let eq11_e1338_d_n7: f64 = (eq11_e1336_d_n7 * s.v[29]);let eq11_e1338_d_n8: f64 = (eq11_e1336_d_n8 * s.v[29]);let eq11_e1338_d_n9: f64 = (eq11_e1336_d_n9 * s.v[29]);let eq11_e1338_d_n10: f64 = (eq11_e1336_d_n10 * s.v[29]);let eq11_e1338_d_n11: f64 = (eq11_e1336_d_n11 * s.v[29]);let eq11_e1338_d_n12: f64 = (eq11_e1336_d_n12 * s.v[29]);let eq11_e1338_d_n13: f64 = (eq11_e1336_d_n13 * s.v[29]);let eq11_e1338_d_n14: f64 = (eq11_e1336_d_n14 * s.v[29]);let eq11_e1338_d_n15: f64 = (eq11_e1336_d_n15 * s.v[29]);let eq11_e1338_d_n16: f64 = (eq11_e1336_d_n16 * s.v[29]);let eq11_e1338_d_b0: f64 = (eq11_e1336_d_b0 * s.v[29]);let eq11_e1338_d_b1: f64 = (eq11_e1336_d_b1 * s.v[29]);let eq11_e1338_d_b2: f64 = (eq11_e1336_d_b2 * s.v[29]);let eq11_e1338_d_b3: f64 = (eq11_e1336_d_b3 * s.v[29]);let eq11_e1338_d_b4: f64 = (eq11_e1336_d_b4 * s.v[29]);let eq11_e1338_d_b5: f64 = (eq11_e1336_d_b5 * s.v[29]);let eq11_e1338_d_b6: f64 = (eq11_e1336_d_b6 * s.v[29]);let eq11_e1338_d_b7: f64 = (eq11_e1336_d_b7 * s.v[29]);let eq11_e1338_d_b8: f64 = (eq11_e1336_d_b8 * s.v[29]);let eq11_e1338_d_b9: f64 = (eq11_e1336_d_b9 * s.v[29]);let eq11_e1338_d_b10: f64 = (eq11_e1336_d_b10 * s.v[29]);let eq11_e1338_d_b11: f64 = (eq11_e1336_d_b11 * s.v[29]);let eq11_e1338_d_b12: f64 = (eq11_e1336_d_b12 * s.v[29]);let eq11_e1338_d_b13: f64 = (eq11_e1336_d_b13 * s.v[29]);let eq11_e1340: f64 = (eq11_e1338 * p[2]);let eq11_e1340_d_n0: f64 = (eq11_e1338_d_n0 * p[2]);let eq11_e1340_d_n1: f64 = (eq11_e1338_d_n1 * p[2]);let eq11_e1340_d_n2: f64 = (eq11_e1338_d_n2 * p[2]);let eq11_e1340_d_n3: f64 = (eq11_e1338_d_n3 * p[2]);let eq11_e1340_d_n4: f64 = (eq11_e1338_d_n4 * p[2]);let eq11_e1340_d_n5: f64 = (eq11_e1338_d_n5 * p[2]);let eq11_e1340_d_n6: f64 = (eq11_e1338_d_n6 * p[2]);let eq11_e1340_d_n7: f64 = (eq11_e1338_d_n7 * p[2]);let eq11_e1340_d_n8: f64 = (eq11_e1338_d_n8 * p[2]);let eq11_e1340_d_n9: f64 = (eq11_e1338_d_n9 * p[2]);let eq11_e1340_d_n10: f64 = (eq11_e1338_d_n10 * p[2]);let eq11_e1340_d_n11: f64 = (eq11_e1338_d_n11 * p[2]);let eq11_e1340_d_n12: f64 = (eq11_e1338_d_n12 * p[2]);let eq11_e1340_d_n13: f64 = (eq11_e1338_d_n13 * p[2]);let eq11_e1340_d_n14: f64 = (eq11_e1338_d_n14 * p[2]);let eq11_e1340_d_n15: f64 = (eq11_e1338_d_n15 * p[2]);let eq11_e1340_d_n16: f64 = (eq11_e1338_d_n16 * p[2]);let eq11_e1340_d_b0: f64 = (eq11_e1338_d_b0 * p[2]);let eq11_e1340_d_b1: f64 = (eq11_e1338_d_b1 * p[2]);let eq11_e1340_d_b2: f64 = (eq11_e1338_d_b2 * p[2]);let eq11_e1340_d_b3: f64 = (eq11_e1338_d_b3 * p[2]);let eq11_e1340_d_b4: f64 = (eq11_e1338_d_b4 * p[2]);let eq11_e1340_d_b5: f64 = (eq11_e1338_d_b5 * p[2]);let eq11_e1340_d_b6: f64 = (eq11_e1338_d_b6 * p[2]);let eq11_e1340_d_b7: f64 = (eq11_e1338_d_b7 * p[2]);let eq11_e1340_d_b8: f64 = (eq11_e1338_d_b8 * p[2]);let eq11_e1340_d_b9: f64 = (eq11_e1338_d_b9 * p[2]);let eq11_e1340_d_b10: f64 = (eq11_e1338_d_b10 * p[2]);let eq11_e1340_d_b11: f64 = (eq11_e1338_d_b11 * p[2]);let eq11_e1340_d_b12: f64 = (eq11_e1338_d_b12 * p[2]);let eq11_e1340_d_b13: f64 = (eq11_e1338_d_b13 * p[2]);let eq11_e1342: f64 = (eq11_e1340 * s.v[30]);let eq11_e1342_d_n0: f64 = (eq11_e1340_d_n0 * s.v[30]);let eq11_e1342_d_n1: f64 = (eq11_e1340_d_n1 * s.v[30]);
        let eq11_e1342_d_n2: f64 = (eq11_e1340_d_n2 * s.v[30]);let eq11_e1342_d_n3: f64 = (eq11_e1340_d_n3 * s.v[30]);let eq11_e1342_d_n4: f64 = (eq11_e1340_d_n4 * s.v[30]);let eq11_e1342_d_n5: f64 = (eq11_e1340_d_n5 * s.v[30]);let eq11_e1342_d_n6: f64 = (eq11_e1340_d_n6 * s.v[30]);let eq11_e1342_d_n7: f64 = (eq11_e1340_d_n7 * s.v[30]);let eq11_e1342_d_n8: f64 = (eq11_e1340_d_n8 * s.v[30]);let eq11_e1342_d_n9: f64 = (eq11_e1340_d_n9 * s.v[30]);let eq11_e1342_d_n10: f64 = (eq11_e1340_d_n10 * s.v[30]);let eq11_e1342_d_n11: f64 = (eq11_e1340_d_n11 * s.v[30]);let eq11_e1342_d_n12: f64 = (eq11_e1340_d_n12 * s.v[30]);let eq11_e1342_d_n13: f64 = (eq11_e1340_d_n13 * s.v[30]);let eq11_e1342_d_n14: f64 = (eq11_e1340_d_n14 * s.v[30]);let eq11_e1342_d_n15: f64 = (eq11_e1340_d_n15 * s.v[30]);let eq11_e1342_d_n16: f64 = (eq11_e1340_d_n16 * s.v[30]);let eq11_e1342_d_b0: f64 = (eq11_e1340_d_b0 * s.v[30]);let eq11_e1342_d_b1: f64 = (eq11_e1340_d_b1 * s.v[30]);let eq11_e1342_d_b2: f64 = (eq11_e1340_d_b2 * s.v[30]);let eq11_e1342_d_b3: f64 = (eq11_e1340_d_b3 * s.v[30]);let eq11_e1342_d_b4: f64 = (eq11_e1340_d_b4 * s.v[30]);let eq11_e1342_d_b5: f64 = (eq11_e1340_d_b5 * s.v[30]);let eq11_e1342_d_b6: f64 = (eq11_e1340_d_b6 * s.v[30]);let eq11_e1342_d_b7: f64 = (eq11_e1340_d_b7 * s.v[30]);let eq11_e1342_d_b8: f64 = (eq11_e1340_d_b8 * s.v[30]);let eq11_e1342_d_b9: f64 = (eq11_e1340_d_b9 * s.v[30]);let eq11_e1342_d_b10: f64 = (eq11_e1340_d_b10 * s.v[30]);let eq11_e1342_d_b11: f64 = (eq11_e1340_d_b11 * s.v[30]);let eq11_e1342_d_b12: f64 = (eq11_e1340_d_b12 * s.v[30]);let eq11_e1342_d_b13: f64 = (eq11_e1340_d_b13 * s.v[30]);let eq11_e1344: f64 = (eq11_e1342 * (nv15 - 0.0));let eq11_e1344_d_n0: f64 = (eq11_e1342_d_n0 * (nv15 - 0.0));let eq11_e1344_d_n1: f64 = (eq11_e1342_d_n1 * (nv15 - 0.0));let eq11_e1344_d_n2: f64 = (eq11_e1342_d_n2 * (nv15 - 0.0));let eq11_e1344_d_n3: f64 = (eq11_e1342_d_n3 * (nv15 - 0.0));let eq11_e1344_d_n4: f64 = (eq11_e1342_d_n4 * (nv15 - 0.0));let eq11_e1344_d_n5: f64 = (eq11_e1342_d_n5 * (nv15 - 0.0));let eq11_e1344_d_n6: f64 = (eq11_e1342_d_n6 * (nv15 - 0.0));let eq11_e1344_d_n7: f64 = (eq11_e1342_d_n7 * (nv15 - 0.0));let eq11_e1344_d_n8: f64 = (eq11_e1342_d_n8 * (nv15 - 0.0));let eq11_e1344_d_n9: f64 = (eq11_e1342_d_n9 * (nv15 - 0.0));let eq11_e1344_d_n10: f64 = (eq11_e1342_d_n10 * (nv15 - 0.0));let eq11_e1344_d_n11: f64 = (eq11_e1342_d_n11 * (nv15 - 0.0));let eq11_e1344_d_n12: f64 = (eq11_e1342_d_n12 * (nv15 - 0.0));let eq11_e1344_d_n13: f64 = (eq11_e1342_d_n13 * (nv15 - 0.0));let eq11_e1344_d_n14: f64 = (eq11_e1342_d_n14 * (nv15 - 0.0));let eq11_e1344_d_n15: f64 = ((eq11_e1342_d_n15 * (nv15 - 0.0)) + eq11_e1342);let eq11_e1344_d_n16: f64 = (eq11_e1342_d_n16 * (nv15 - 0.0));let eq11_e1344_d_b0: f64 = (eq11_e1342_d_b0 * (nv15 - 0.0));let eq11_e1344_d_b1: f64 = (eq11_e1342_d_b1 * (nv15 - 0.0));let eq11_e1344_d_b2: f64 = (eq11_e1342_d_b2 * (nv15 - 0.0));let eq11_e1344_d_b3: f64 = (eq11_e1342_d_b3 * (nv15 - 0.0));let eq11_e1344_d_b4: f64 = (eq11_e1342_d_b4 * (nv15 - 0.0));let eq11_e1344_d_b5: f64 = (eq11_e1342_d_b5 * (nv15 - 0.0));let eq11_e1344_d_b6: f64 = (eq11_e1342_d_b6 * (nv15 - 0.0));let eq11_e1344_d_b7: f64 = (eq11_e1342_d_b7 * (nv15 - 0.0));let eq11_e1344_d_b8: f64 = (eq11_e1342_d_b8 * (nv15 - 0.0));let eq11_e1344_d_b9: f64 = (eq11_e1342_d_b9 * (nv15 - 0.0));let eq11_e1344_d_b10: f64 = (eq11_e1342_d_b10 * (nv15 - 0.0));let eq11_e1344_d_b11: f64 = (eq11_e1342_d_b11 * (nv15 - 0.0));let eq11_e1344_d_b12: f64 = (eq11_e1342_d_b12 * (nv15 - 0.0));let eq11_e1344_d_b13: f64 = (eq11_e1342_d_b13 * (nv15 - 0.0));let eq11_e1345: f64 = (0.5 * eq11_e1344);let eq11_e1345_d_n0: f64 = (0.5 * eq11_e1344_d_n0);let eq11_e1345_d_n1: f64 = (0.5 * eq11_e1344_d_n1);let eq11_e1345_d_n2: f64 = (0.5 * eq11_e1344_d_n2);let eq11_e1345_d_n3: f64 = (0.5 * eq11_e1344_d_n3);let eq11_e1345_d_n4: f64 = (0.5 * eq11_e1344_d_n4);let eq11_e1345_d_n5: f64 = (0.5 * eq11_e1344_d_n5);let eq11_e1345_d_n6: f64 = (0.5 * eq11_e1344_d_n6);let eq11_e1345_d_n7: f64 = (0.5 * eq11_e1344_d_n7);let eq11_e1345_d_n8: f64 = (0.5 * eq11_e1344_d_n8);
        let eq11_e1345_d_n9: f64 = (0.5 * eq11_e1344_d_n9);let eq11_e1345_d_n10: f64 = (0.5 * eq11_e1344_d_n10);let eq11_e1345_d_n11: f64 = (0.5 * eq11_e1344_d_n11);let eq11_e1345_d_n12: f64 = (0.5 * eq11_e1344_d_n12);let eq11_e1345_d_n13: f64 = (0.5 * eq11_e1344_d_n13);let eq11_e1345_d_n14: f64 = (0.5 * eq11_e1344_d_n14);let eq11_e1345_d_n15: f64 = (0.5 * eq11_e1344_d_n15);let eq11_e1345_d_n16: f64 = (0.5 * eq11_e1344_d_n16);let eq11_e1345_d_b0: f64 = (0.5 * eq11_e1344_d_b0);let eq11_e1345_d_b1: f64 = (0.5 * eq11_e1344_d_b1);let eq11_e1345_d_b2: f64 = (0.5 * eq11_e1344_d_b2);let eq11_e1345_d_b3: f64 = (0.5 * eq11_e1344_d_b3);let eq11_e1345_d_b4: f64 = (0.5 * eq11_e1344_d_b4);let eq11_e1345_d_b5: f64 = (0.5 * eq11_e1344_d_b5);let eq11_e1345_d_b6: f64 = (0.5 * eq11_e1344_d_b6);let eq11_e1345_d_b7: f64 = (0.5 * eq11_e1344_d_b7);let eq11_e1345_d_b8: f64 = (0.5 * eq11_e1344_d_b8);let eq11_e1345_d_b9: f64 = (0.5 * eq11_e1344_d_b9);let eq11_e1345_d_b10: f64 = (0.5 * eq11_e1344_d_b10);let eq11_e1345_d_b11: f64 = (0.5 * eq11_e1344_d_b11);let eq11_e1345_d_b12: f64 = (0.5 * eq11_e1344_d_b12);let eq11_e1345_d_b13: f64 = (0.5 * eq11_e1344_d_b13);let eq11_e1346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e1345);let eq11_e1347: f64 = (p[29] * eq11_e1346);let eq11_e1347_d_n0: f64 = (p[29] * (eq11_e1345_d_n0 * ddt_scale));let eq11_e1347_d_n1: f64 = (p[29] * (eq11_e1345_d_n1 * ddt_scale));let eq11_e1347_d_n2: f64 = (p[29] * (eq11_e1345_d_n2 * ddt_scale));let eq11_e1347_d_n3: f64 = (p[29] * (eq11_e1345_d_n3 * ddt_scale));let eq11_e1347_d_n4: f64 = (p[29] * (eq11_e1345_d_n4 * ddt_scale));let eq11_e1347_d_n5: f64 = (p[29] * (eq11_e1345_d_n5 * ddt_scale));let eq11_e1347_d_n6: f64 = (p[29] * (eq11_e1345_d_n6 * ddt_scale));let eq11_e1347_d_n7: f64 = (p[29] * (eq11_e1345_d_n7 * ddt_scale));let eq11_e1347_d_n8: f64 = (p[29] * (eq11_e1345_d_n8 * ddt_scale));let eq11_e1347_d_n9: f64 = (p[29] * (eq11_e1345_d_n9 * ddt_scale));let eq11_e1347_d_n10: f64 = (p[29] * (eq11_e1345_d_n10 * ddt_scale));let eq11_e1347_d_n11: f64 = (p[29] * (eq11_e1345_d_n11 * ddt_scale));let eq11_e1347_d_n12: f64 = (p[29] * (eq11_e1345_d_n12 * ddt_scale));let eq11_e1347_d_n13: f64 = (p[29] * (eq11_e1345_d_n13 * ddt_scale));let eq11_e1347_d_n14: f64 = (p[29] * (eq11_e1345_d_n14 * ddt_scale));let eq11_e1347_d_n15: f64 = (p[29] * (eq11_e1345_d_n15 * ddt_scale));let eq11_e1347_d_n16: f64 = (p[29] * (eq11_e1345_d_n16 * ddt_scale));let eq11_e1347_d_b0: f64 = (p[29] * (eq11_e1345_d_b0 * ddt_scale));let eq11_e1347_d_b1: f64 = (p[29] * (eq11_e1345_d_b1 * ddt_scale));let eq11_e1347_d_b2: f64 = (p[29] * (eq11_e1345_d_b2 * ddt_scale));let eq11_e1347_d_b3: f64 = (p[29] * (eq11_e1345_d_b3 * ddt_scale));let eq11_e1347_d_b4: f64 = (p[29] * (eq11_e1345_d_b4 * ddt_scale));let eq11_e1347_d_b5: f64 = (p[29] * (eq11_e1345_d_b5 * ddt_scale));let eq11_e1347_d_b6: f64 = (p[29] * (eq11_e1345_d_b6 * ddt_scale));let eq11_e1347_d_b7: f64 = (p[29] * (eq11_e1345_d_b7 * ddt_scale));let eq11_e1347_d_b8: f64 = (p[29] * (eq11_e1345_d_b8 * ddt_scale));let eq11_e1347_d_b9: f64 = (p[29] * (eq11_e1345_d_b9 * ddt_scale));let eq11_e1347_d_b10: f64 = (p[29] * (eq11_e1345_d_b10 * ddt_scale));let eq11_e1347_d_b11: f64 = (p[29] * (eq11_e1345_d_b11 * ddt_scale));let eq11_e1347_d_b12: f64 = (p[29] * (eq11_e1345_d_b12 * ddt_scale));let eq11_e1347_d_b13: f64 = (p[29] * (eq11_e1345_d_b13 * ddt_scale));
        (eq11_e1347, eq11_e1347_d_n0, eq11_e1347_d_n1, eq11_e1347_d_n2, eq11_e1347_d_n3, eq11_e1347_d_n4, eq11_e1347_d_n5, eq11_e1347_d_n6, eq11_e1347_d_n7, eq11_e1347_d_n8, eq11_e1347_d_n9, eq11_e1347_d_n10, eq11_e1347_d_n11, eq11_e1347_d_n12, eq11_e1347_d_n13, eq11_e1347_d_n14, eq11_e1347_d_n15, eq11_e1347_d_n16, eq11_e1347_d_b0, eq11_e1347_d_b1, eq11_e1347_d_b2, eq11_e1347_d_b3, eq11_e1347_d_b4, eq11_e1347_d_b5, eq11_e1347_d_b6, eq11_e1347_d_b7, eq11_e1347_d_b8, eq11_e1347_d_b9, eq11_e1347_d_b10, eq11_e1347_d_b11, eq11_e1347_d_b12, eq11_e1347_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1349;let eq11_node_derivatives: [f64; 17] = [eq11_e1349_d_n0, eq11_e1349_d_n1, eq11_e1349_d_n2, eq11_e1349_d_n3, eq11_e1349_d_n4, eq11_e1349_d_n5, eq11_e1349_d_n6, eq11_e1349_d_n7, eq11_e1349_d_n8, eq11_e1349_d_n9, eq11_e1349_d_n10, eq11_e1349_d_n11, eq11_e1349_d_n12, eq11_e1349_d_n13, eq11_e1349_d_n14, eq11_e1349_d_n15, eq11_e1349_d_n16];let eq11_branch_derivatives: [f64; 14] = [eq11_e1349_d_b0, eq11_e1349_d_b1, eq11_e1349_d_b2, eq11_e1349_d_b3, eq11_e1349_d_b4, eq11_e1349_d_b5, eq11_e1349_d_b6, eq11_e1349_d_b7, eq11_e1349_d_b8, eq11_e1349_d_b9, eq11_e1349_d_b10, eq11_e1349_d_b11, eq11_e1349_d_b12, eq11_e1349_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1375, eq12_e1375_d_n0, eq12_e1375_d_n1, eq12_e1375_d_n2, eq12_e1375_d_n3, eq12_e1375_d_n4, eq12_e1375_d_n5, eq12_e1375_d_n6, eq12_e1375_d_n7, eq12_e1375_d_n8, eq12_e1375_d_n9, eq12_e1375_d_n10, eq12_e1375_d_n11, eq12_e1375_d_n12, eq12_e1375_d_n13, eq12_e1375_d_n14, eq12_e1375_d_n15, eq12_e1375_d_n16, eq12_e1375_d_b0, eq12_e1375_d_b1, eq12_e1375_d_b2, eq12_e1375_d_b3, eq12_e1375_d_b4, eq12_e1375_d_b5, eq12_e1375_d_b6, eq12_e1375_d_b7, eq12_e1375_d_b8, eq12_e1375_d_b9, eq12_e1375_d_b10, eq12_e1375_d_b11, eq12_e1375_d_b12, eq12_e1375_d_b13,) = {
    if (s.b[1556] && (!s.b[1555])) {
        let eq12_e1358: f64 = (1.0 - s.v[57]);let eq12_e1360: f64 = (eq12_e1358 * s.v[378]);let eq12_e1360_d_n0: f64 = (((-s.dn[57][0]) * s.v[378]) + (eq12_e1358 * s.dn[378][0]));let eq12_e1360_d_n1: f64 = (((-s.dn[57][1]) * s.v[378]) + (eq12_e1358 * s.dn[378][1]));let eq12_e1360_d_n2: f64 = (((-s.dn[57][2]) * s.v[378]) + (eq12_e1358 * s.dn[378][2]));let eq12_e1360_d_n3: f64 = (((-s.dn[57][3]) * s.v[378]) + (eq12_e1358 * s.dn[378][3]));let eq12_e1360_d_n4: f64 = (((-s.dn[57][4]) * s.v[378]) + (eq12_e1358 * s.dn[378][4]));let eq12_e1360_d_n5: f64 = (((-s.dn[57][5]) * s.v[378]) + (eq12_e1358 * s.dn[378][5]));let eq12_e1360_d_n6: f64 = (((-s.dn[57][6]) * s.v[378]) + (eq12_e1358 * s.dn[378][6]));let eq12_e1360_d_n7: f64 = (((-s.dn[57][7]) * s.v[378]) + (eq12_e1358 * s.dn[378][7]));let eq12_e1360_d_n8: f64 = (((-s.dn[57][8]) * s.v[378]) + (eq12_e1358 * s.dn[378][8]));let eq12_e1360_d_n9: f64 = (((-s.dn[57][9]) * s.v[378]) + (eq12_e1358 * s.dn[378][9]));let eq12_e1360_d_n10: f64 = (((-s.dn[57][10]) * s.v[378]) + (eq12_e1358 * s.dn[378][10]));let eq12_e1360_d_n11: f64 = (((-s.dn[57][11]) * s.v[378]) + (eq12_e1358 * s.dn[378][11]));let eq12_e1360_d_n12: f64 = (((-s.dn[57][12]) * s.v[378]) + (eq12_e1358 * s.dn[378][12]));let eq12_e1360_d_n13: f64 = (((-s.dn[57][13]) * s.v[378]) + (eq12_e1358 * s.dn[378][13]));let eq12_e1360_d_n14: f64 = (((-s.dn[57][14]) * s.v[378]) + (eq12_e1358 * s.dn[378][14]));let eq12_e1360_d_n15: f64 = (((-s.dn[57][15]) * s.v[378]) + (eq12_e1358 * s.dn[378][15]));let eq12_e1360_d_n16: f64 = (((-s.dn[57][16]) * s.v[378]) + (eq12_e1358 * s.dn[378][16]));let eq12_e1360_d_b0: f64 = (((-s.db[57][0]) * s.v[378]) + (eq12_e1358 * s.db[378][0]));let eq12_e1360_d_b1: f64 = (((-s.db[57][1]) * s.v[378]) + (eq12_e1358 * s.db[378][1]));let eq12_e1360_d_b2: f64 = (((-s.db[57][2]) * s.v[378]) + (eq12_e1358 * s.db[378][2]));let eq12_e1360_d_b3: f64 = (((-s.db[57][3]) * s.v[378]) + (eq12_e1358 * s.db[378][3]));let eq12_e1360_d_b4: f64 = (((-s.db[57][4]) * s.v[378]) + (eq12_e1358 * s.db[378][4]));let eq12_e1360_d_b5: f64 = (((-s.db[57][5]) * s.v[378]) + (eq12_e1358 * s.db[378][5]));let eq12_e1360_d_b6: f64 = (((-s.db[57][6]) * s.v[378]) + (eq12_e1358 * s.db[378][6]));let eq12_e1360_d_b7: f64 = (((-s.db[57][7]) * s.v[378]) + (eq12_e1358 * s.db[378][7]));let eq12_e1360_d_b8: f64 = (((-s.db[57][8]) * s.v[378]) + (eq12_e1358 * s.db[378][8]));let eq12_e1360_d_b9: f64 = (((-s.db[57][9]) * s.v[378]) + (eq12_e1358 * s.db[378][9]));let eq12_e1360_d_b10: f64 = (((-s.db[57][10]) * s.v[378]) + (eq12_e1358 * s.db[378][10]));let eq12_e1360_d_b11: f64 = (((-s.db[57][11]) * s.v[378]) + (eq12_e1358 * s.db[378][11]));let eq12_e1360_d_b12: f64 = (((-s.db[57][12]) * s.v[378]) + (eq12_e1358 * s.db[378][12]));let eq12_e1360_d_b13: f64 = (((-s.db[57][13]) * s.v[378]) + (eq12_e1358 * s.db[378][13]));let eq12_e1362: f64 = (eq12_e1360 * s.v[46]);let eq12_e1362_d_n0: f64 = (eq12_e1360_d_n0 * s.v[46]);let eq12_e1362_d_n1: f64 = (eq12_e1360_d_n1 * s.v[46]);let eq12_e1362_d_n2: f64 = (eq12_e1360_d_n2 * s.v[46]);let eq12_e1362_d_n3: f64 = (eq12_e1360_d_n3 * s.v[46]);let eq12_e1362_d_n4: f64 = (eq12_e1360_d_n4 * s.v[46]);let eq12_e1362_d_n5: f64 = (eq12_e1360_d_n5 * s.v[46]);let eq12_e1362_d_n6: f64 = (eq12_e1360_d_n6 * s.v[46]);let eq12_e1362_d_n7: f64 = (eq12_e1360_d_n7 * s.v[46]);let eq12_e1362_d_n8: f64 = (eq12_e1360_d_n8 * s.v[46]);let eq12_e1362_d_n9: f64 = (eq12_e1360_d_n9 * s.v[46]);let eq12_e1362_d_n10: f64 = (eq12_e1360_d_n10 * s.v[46]);let eq12_e1362_d_n11: f64 = (eq12_e1360_d_n11 * s.v[46]);let eq12_e1362_d_n12: f64 = (eq12_e1360_d_n12 * s.v[46]);let eq12_e1362_d_n13: f64 = (eq12_e1360_d_n13 * s.v[46]);let eq12_e1362_d_n14: f64 = (eq12_e1360_d_n14 * s.v[46]);let eq12_e1362_d_n15: f64 = (eq12_e1360_d_n15 * s.v[46]);let eq12_e1362_d_n16: f64 = (eq12_e1360_d_n16 * s.v[46]);let eq12_e1362_d_b0: f64 = (eq12_e1360_d_b0 * s.v[46]);let eq12_e1362_d_b1: f64 = (eq12_e1360_d_b1 * s.v[46]);let eq12_e1362_d_b2: f64 = (eq12_e1360_d_b2 * s.v[46]);let eq12_e1362_d_b3: f64 = (eq12_e1360_d_b3 * s.v[46]);let eq12_e1362_d_b4: f64 = (eq12_e1360_d_b4 * s.v[46]);
        let eq12_e1362_d_b5: f64 = (eq12_e1360_d_b5 * s.v[46]);let eq12_e1362_d_b6: f64 = (eq12_e1360_d_b6 * s.v[46]);let eq12_e1362_d_b7: f64 = (eq12_e1360_d_b7 * s.v[46]);let eq12_e1362_d_b8: f64 = (eq12_e1360_d_b8 * s.v[46]);let eq12_e1362_d_b9: f64 = (eq12_e1360_d_b9 * s.v[46]);let eq12_e1362_d_b10: f64 = (eq12_e1360_d_b10 * s.v[46]);let eq12_e1362_d_b11: f64 = (eq12_e1360_d_b11 * s.v[46]);let eq12_e1362_d_b12: f64 = (eq12_e1360_d_b12 * s.v[46]);let eq12_e1362_d_b13: f64 = (eq12_e1360_d_b13 * s.v[46]);let eq12_e1364: f64 = (eq12_e1362 * s.v[29]);let eq12_e1364_d_n0: f64 = (eq12_e1362_d_n0 * s.v[29]);let eq12_e1364_d_n1: f64 = (eq12_e1362_d_n1 * s.v[29]);let eq12_e1364_d_n2: f64 = (eq12_e1362_d_n2 * s.v[29]);let eq12_e1364_d_n3: f64 = (eq12_e1362_d_n3 * s.v[29]);let eq12_e1364_d_n4: f64 = (eq12_e1362_d_n4 * s.v[29]);let eq12_e1364_d_n5: f64 = (eq12_e1362_d_n5 * s.v[29]);let eq12_e1364_d_n6: f64 = (eq12_e1362_d_n6 * s.v[29]);let eq12_e1364_d_n7: f64 = (eq12_e1362_d_n7 * s.v[29]);let eq12_e1364_d_n8: f64 = (eq12_e1362_d_n8 * s.v[29]);let eq12_e1364_d_n9: f64 = (eq12_e1362_d_n9 * s.v[29]);let eq12_e1364_d_n10: f64 = (eq12_e1362_d_n10 * s.v[29]);let eq12_e1364_d_n11: f64 = (eq12_e1362_d_n11 * s.v[29]);let eq12_e1364_d_n12: f64 = (eq12_e1362_d_n12 * s.v[29]);let eq12_e1364_d_n13: f64 = (eq12_e1362_d_n13 * s.v[29]);let eq12_e1364_d_n14: f64 = (eq12_e1362_d_n14 * s.v[29]);let eq12_e1364_d_n15: f64 = (eq12_e1362_d_n15 * s.v[29]);let eq12_e1364_d_n16: f64 = (eq12_e1362_d_n16 * s.v[29]);let eq12_e1364_d_b0: f64 = (eq12_e1362_d_b0 * s.v[29]);let eq12_e1364_d_b1: f64 = (eq12_e1362_d_b1 * s.v[29]);let eq12_e1364_d_b2: f64 = (eq12_e1362_d_b2 * s.v[29]);let eq12_e1364_d_b3: f64 = (eq12_e1362_d_b3 * s.v[29]);let eq12_e1364_d_b4: f64 = (eq12_e1362_d_b4 * s.v[29]);let eq12_e1364_d_b5: f64 = (eq12_e1362_d_b5 * s.v[29]);let eq12_e1364_d_b6: f64 = (eq12_e1362_d_b6 * s.v[29]);let eq12_e1364_d_b7: f64 = (eq12_e1362_d_b7 * s.v[29]);let eq12_e1364_d_b8: f64 = (eq12_e1362_d_b8 * s.v[29]);let eq12_e1364_d_b9: f64 = (eq12_e1362_d_b9 * s.v[29]);let eq12_e1364_d_b10: f64 = (eq12_e1362_d_b10 * s.v[29]);let eq12_e1364_d_b11: f64 = (eq12_e1362_d_b11 * s.v[29]);let eq12_e1364_d_b12: f64 = (eq12_e1362_d_b12 * s.v[29]);let eq12_e1364_d_b13: f64 = (eq12_e1362_d_b13 * s.v[29]);let eq12_e1366: f64 = (eq12_e1364 * p[2]);let eq12_e1366_d_n0: f64 = (eq12_e1364_d_n0 * p[2]);let eq12_e1366_d_n1: f64 = (eq12_e1364_d_n1 * p[2]);let eq12_e1366_d_n2: f64 = (eq12_e1364_d_n2 * p[2]);let eq12_e1366_d_n3: f64 = (eq12_e1364_d_n3 * p[2]);let eq12_e1366_d_n4: f64 = (eq12_e1364_d_n4 * p[2]);let eq12_e1366_d_n5: f64 = (eq12_e1364_d_n5 * p[2]);let eq12_e1366_d_n6: f64 = (eq12_e1364_d_n6 * p[2]);let eq12_e1366_d_n7: f64 = (eq12_e1364_d_n7 * p[2]);let eq12_e1366_d_n8: f64 = (eq12_e1364_d_n8 * p[2]);let eq12_e1366_d_n9: f64 = (eq12_e1364_d_n9 * p[2]);let eq12_e1366_d_n10: f64 = (eq12_e1364_d_n10 * p[2]);let eq12_e1366_d_n11: f64 = (eq12_e1364_d_n11 * p[2]);let eq12_e1366_d_n12: f64 = (eq12_e1364_d_n12 * p[2]);let eq12_e1366_d_n13: f64 = (eq12_e1364_d_n13 * p[2]);let eq12_e1366_d_n14: f64 = (eq12_e1364_d_n14 * p[2]);let eq12_e1366_d_n15: f64 = (eq12_e1364_d_n15 * p[2]);let eq12_e1366_d_n16: f64 = (eq12_e1364_d_n16 * p[2]);let eq12_e1366_d_b0: f64 = (eq12_e1364_d_b0 * p[2]);let eq12_e1366_d_b1: f64 = (eq12_e1364_d_b1 * p[2]);let eq12_e1366_d_b2: f64 = (eq12_e1364_d_b2 * p[2]);let eq12_e1366_d_b3: f64 = (eq12_e1364_d_b3 * p[2]);let eq12_e1366_d_b4: f64 = (eq12_e1364_d_b4 * p[2]);let eq12_e1366_d_b5: f64 = (eq12_e1364_d_b5 * p[2]);let eq12_e1366_d_b6: f64 = (eq12_e1364_d_b6 * p[2]);let eq12_e1366_d_b7: f64 = (eq12_e1364_d_b7 * p[2]);let eq12_e1366_d_b8: f64 = (eq12_e1364_d_b8 * p[2]);let eq12_e1366_d_b9: f64 = (eq12_e1364_d_b9 * p[2]);let eq12_e1366_d_b10: f64 = (eq12_e1364_d_b10 * p[2]);let eq12_e1366_d_b11: f64 = (eq12_e1364_d_b11 * p[2]);let eq12_e1366_d_b12: f64 = (eq12_e1364_d_b12 * p[2]);let eq12_e1366_d_b13: f64 = (eq12_e1364_d_b13 * p[2]);let eq12_e1368: f64 = (eq12_e1366 * s.v[30]);let eq12_e1368_d_n0: f64 = (eq12_e1366_d_n0 * s.v[30]);
        let eq12_e1368_d_n1: f64 = (eq12_e1366_d_n1 * s.v[30]);let eq12_e1368_d_n2: f64 = (eq12_e1366_d_n2 * s.v[30]);let eq12_e1368_d_n3: f64 = (eq12_e1366_d_n3 * s.v[30]);let eq12_e1368_d_n4: f64 = (eq12_e1366_d_n4 * s.v[30]);let eq12_e1368_d_n5: f64 = (eq12_e1366_d_n5 * s.v[30]);let eq12_e1368_d_n6: f64 = (eq12_e1366_d_n6 * s.v[30]);let eq12_e1368_d_n7: f64 = (eq12_e1366_d_n7 * s.v[30]);let eq12_e1368_d_n8: f64 = (eq12_e1366_d_n8 * s.v[30]);let eq12_e1368_d_n9: f64 = (eq12_e1366_d_n9 * s.v[30]);let eq12_e1368_d_n10: f64 = (eq12_e1366_d_n10 * s.v[30]);let eq12_e1368_d_n11: f64 = (eq12_e1366_d_n11 * s.v[30]);let eq12_e1368_d_n12: f64 = (eq12_e1366_d_n12 * s.v[30]);let eq12_e1368_d_n13: f64 = (eq12_e1366_d_n13 * s.v[30]);let eq12_e1368_d_n14: f64 = (eq12_e1366_d_n14 * s.v[30]);let eq12_e1368_d_n15: f64 = (eq12_e1366_d_n15 * s.v[30]);let eq12_e1368_d_n16: f64 = (eq12_e1366_d_n16 * s.v[30]);let eq12_e1368_d_b0: f64 = (eq12_e1366_d_b0 * s.v[30]);let eq12_e1368_d_b1: f64 = (eq12_e1366_d_b1 * s.v[30]);let eq12_e1368_d_b2: f64 = (eq12_e1366_d_b2 * s.v[30]);let eq12_e1368_d_b3: f64 = (eq12_e1366_d_b3 * s.v[30]);let eq12_e1368_d_b4: f64 = (eq12_e1366_d_b4 * s.v[30]);let eq12_e1368_d_b5: f64 = (eq12_e1366_d_b5 * s.v[30]);let eq12_e1368_d_b6: f64 = (eq12_e1366_d_b6 * s.v[30]);let eq12_e1368_d_b7: f64 = (eq12_e1366_d_b7 * s.v[30]);let eq12_e1368_d_b8: f64 = (eq12_e1366_d_b8 * s.v[30]);let eq12_e1368_d_b9: f64 = (eq12_e1366_d_b9 * s.v[30]);let eq12_e1368_d_b10: f64 = (eq12_e1366_d_b10 * s.v[30]);let eq12_e1368_d_b11: f64 = (eq12_e1366_d_b11 * s.v[30]);let eq12_e1368_d_b12: f64 = (eq12_e1366_d_b12 * s.v[30]);let eq12_e1368_d_b13: f64 = (eq12_e1366_d_b13 * s.v[30]);let eq12_e1370: f64 = (eq12_e1368 * (nv15 - 0.0));let eq12_e1370_d_n0: f64 = (eq12_e1368_d_n0 * (nv15 - 0.0));let eq12_e1370_d_n1: f64 = (eq12_e1368_d_n1 * (nv15 - 0.0));let eq12_e1370_d_n2: f64 = (eq12_e1368_d_n2 * (nv15 - 0.0));let eq12_e1370_d_n3: f64 = (eq12_e1368_d_n3 * (nv15 - 0.0));let eq12_e1370_d_n4: f64 = (eq12_e1368_d_n4 * (nv15 - 0.0));let eq12_e1370_d_n5: f64 = (eq12_e1368_d_n5 * (nv15 - 0.0));let eq12_e1370_d_n6: f64 = (eq12_e1368_d_n6 * (nv15 - 0.0));let eq12_e1370_d_n7: f64 = (eq12_e1368_d_n7 * (nv15 - 0.0));let eq12_e1370_d_n8: f64 = (eq12_e1368_d_n8 * (nv15 - 0.0));let eq12_e1370_d_n9: f64 = (eq12_e1368_d_n9 * (nv15 - 0.0));let eq12_e1370_d_n10: f64 = (eq12_e1368_d_n10 * (nv15 - 0.0));let eq12_e1370_d_n11: f64 = (eq12_e1368_d_n11 * (nv15 - 0.0));let eq12_e1370_d_n12: f64 = (eq12_e1368_d_n12 * (nv15 - 0.0));let eq12_e1370_d_n13: f64 = (eq12_e1368_d_n13 * (nv15 - 0.0));let eq12_e1370_d_n14: f64 = (eq12_e1368_d_n14 * (nv15 - 0.0));let eq12_e1370_d_n15: f64 = ((eq12_e1368_d_n15 * (nv15 - 0.0)) + eq12_e1368);let eq12_e1370_d_n16: f64 = (eq12_e1368_d_n16 * (nv15 - 0.0));let eq12_e1370_d_b0: f64 = (eq12_e1368_d_b0 * (nv15 - 0.0));let eq12_e1370_d_b1: f64 = (eq12_e1368_d_b1 * (nv15 - 0.0));let eq12_e1370_d_b2: f64 = (eq12_e1368_d_b2 * (nv15 - 0.0));let eq12_e1370_d_b3: f64 = (eq12_e1368_d_b3 * (nv15 - 0.0));let eq12_e1370_d_b4: f64 = (eq12_e1368_d_b4 * (nv15 - 0.0));let eq12_e1370_d_b5: f64 = (eq12_e1368_d_b5 * (nv15 - 0.0));let eq12_e1370_d_b6: f64 = (eq12_e1368_d_b6 * (nv15 - 0.0));let eq12_e1370_d_b7: f64 = (eq12_e1368_d_b7 * (nv15 - 0.0));let eq12_e1370_d_b8: f64 = (eq12_e1368_d_b8 * (nv15 - 0.0));let eq12_e1370_d_b9: f64 = (eq12_e1368_d_b9 * (nv15 - 0.0));let eq12_e1370_d_b10: f64 = (eq12_e1368_d_b10 * (nv15 - 0.0));let eq12_e1370_d_b11: f64 = (eq12_e1368_d_b11 * (nv15 - 0.0));let eq12_e1370_d_b12: f64 = (eq12_e1368_d_b12 * (nv15 - 0.0));let eq12_e1370_d_b13: f64 = (eq12_e1368_d_b13 * (nv15 - 0.0));let eq12_e1371: f64 = (0.5 * eq12_e1370);let eq12_e1371_d_n0: f64 = (0.5 * eq12_e1370_d_n0);let eq12_e1371_d_n1: f64 = (0.5 * eq12_e1370_d_n1);let eq12_e1371_d_n2: f64 = (0.5 * eq12_e1370_d_n2);let eq12_e1371_d_n3: f64 = (0.5 * eq12_e1370_d_n3);let eq12_e1371_d_n4: f64 = (0.5 * eq12_e1370_d_n4);let eq12_e1371_d_n5: f64 = (0.5 * eq12_e1370_d_n5);let eq12_e1371_d_n6: f64 = (0.5 * eq12_e1370_d_n6);let eq12_e1371_d_n7: f64 = (0.5 * eq12_e1370_d_n7);
        let eq12_e1371_d_n8: f64 = (0.5 * eq12_e1370_d_n8);let eq12_e1371_d_n9: f64 = (0.5 * eq12_e1370_d_n9);let eq12_e1371_d_n10: f64 = (0.5 * eq12_e1370_d_n10);let eq12_e1371_d_n11: f64 = (0.5 * eq12_e1370_d_n11);let eq12_e1371_d_n12: f64 = (0.5 * eq12_e1370_d_n12);let eq12_e1371_d_n13: f64 = (0.5 * eq12_e1370_d_n13);let eq12_e1371_d_n14: f64 = (0.5 * eq12_e1370_d_n14);let eq12_e1371_d_n15: f64 = (0.5 * eq12_e1370_d_n15);let eq12_e1371_d_n16: f64 = (0.5 * eq12_e1370_d_n16);let eq12_e1371_d_b0: f64 = (0.5 * eq12_e1370_d_b0);let eq12_e1371_d_b1: f64 = (0.5 * eq12_e1370_d_b1);let eq12_e1371_d_b2: f64 = (0.5 * eq12_e1370_d_b2);let eq12_e1371_d_b3: f64 = (0.5 * eq12_e1370_d_b3);let eq12_e1371_d_b4: f64 = (0.5 * eq12_e1370_d_b4);let eq12_e1371_d_b5: f64 = (0.5 * eq12_e1370_d_b5);let eq12_e1371_d_b6: f64 = (0.5 * eq12_e1370_d_b6);let eq12_e1371_d_b7: f64 = (0.5 * eq12_e1370_d_b7);let eq12_e1371_d_b8: f64 = (0.5 * eq12_e1370_d_b8);let eq12_e1371_d_b9: f64 = (0.5 * eq12_e1370_d_b9);let eq12_e1371_d_b10: f64 = (0.5 * eq12_e1370_d_b10);let eq12_e1371_d_b11: f64 = (0.5 * eq12_e1370_d_b11);let eq12_e1371_d_b12: f64 = (0.5 * eq12_e1370_d_b12);let eq12_e1371_d_b13: f64 = (0.5 * eq12_e1370_d_b13);let eq12_e1372: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e1371);let eq12_e1373: f64 = (p[29] * eq12_e1372);let eq12_e1373_d_n0: f64 = (p[29] * (eq12_e1371_d_n0 * ddt_scale));let eq12_e1373_d_n1: f64 = (p[29] * (eq12_e1371_d_n1 * ddt_scale));let eq12_e1373_d_n2: f64 = (p[29] * (eq12_e1371_d_n2 * ddt_scale));let eq12_e1373_d_n3: f64 = (p[29] * (eq12_e1371_d_n3 * ddt_scale));let eq12_e1373_d_n4: f64 = (p[29] * (eq12_e1371_d_n4 * ddt_scale));let eq12_e1373_d_n5: f64 = (p[29] * (eq12_e1371_d_n5 * ddt_scale));let eq12_e1373_d_n6: f64 = (p[29] * (eq12_e1371_d_n6 * ddt_scale));let eq12_e1373_d_n7: f64 = (p[29] * (eq12_e1371_d_n7 * ddt_scale));let eq12_e1373_d_n8: f64 = (p[29] * (eq12_e1371_d_n8 * ddt_scale));let eq12_e1373_d_n9: f64 = (p[29] * (eq12_e1371_d_n9 * ddt_scale));let eq12_e1373_d_n10: f64 = (p[29] * (eq12_e1371_d_n10 * ddt_scale));let eq12_e1373_d_n11: f64 = (p[29] * (eq12_e1371_d_n11 * ddt_scale));let eq12_e1373_d_n12: f64 = (p[29] * (eq12_e1371_d_n12 * ddt_scale));let eq12_e1373_d_n13: f64 = (p[29] * (eq12_e1371_d_n13 * ddt_scale));let eq12_e1373_d_n14: f64 = (p[29] * (eq12_e1371_d_n14 * ddt_scale));let eq12_e1373_d_n15: f64 = (p[29] * (eq12_e1371_d_n15 * ddt_scale));let eq12_e1373_d_n16: f64 = (p[29] * (eq12_e1371_d_n16 * ddt_scale));let eq12_e1373_d_b0: f64 = (p[29] * (eq12_e1371_d_b0 * ddt_scale));let eq12_e1373_d_b1: f64 = (p[29] * (eq12_e1371_d_b1 * ddt_scale));let eq12_e1373_d_b2: f64 = (p[29] * (eq12_e1371_d_b2 * ddt_scale));let eq12_e1373_d_b3: f64 = (p[29] * (eq12_e1371_d_b3 * ddt_scale));let eq12_e1373_d_b4: f64 = (p[29] * (eq12_e1371_d_b4 * ddt_scale));let eq12_e1373_d_b5: f64 = (p[29] * (eq12_e1371_d_b5 * ddt_scale));let eq12_e1373_d_b6: f64 = (p[29] * (eq12_e1371_d_b6 * ddt_scale));let eq12_e1373_d_b7: f64 = (p[29] * (eq12_e1371_d_b7 * ddt_scale));let eq12_e1373_d_b8: f64 = (p[29] * (eq12_e1371_d_b8 * ddt_scale));let eq12_e1373_d_b9: f64 = (p[29] * (eq12_e1371_d_b9 * ddt_scale));let eq12_e1373_d_b10: f64 = (p[29] * (eq12_e1371_d_b10 * ddt_scale));let eq12_e1373_d_b11: f64 = (p[29] * (eq12_e1371_d_b11 * ddt_scale));let eq12_e1373_d_b12: f64 = (p[29] * (eq12_e1371_d_b12 * ddt_scale));let eq12_e1373_d_b13: f64 = (p[29] * (eq12_e1371_d_b13 * ddt_scale));
        (eq12_e1373, eq12_e1373_d_n0, eq12_e1373_d_n1, eq12_e1373_d_n2, eq12_e1373_d_n3, eq12_e1373_d_n4, eq12_e1373_d_n5, eq12_e1373_d_n6, eq12_e1373_d_n7, eq12_e1373_d_n8, eq12_e1373_d_n9, eq12_e1373_d_n10, eq12_e1373_d_n11, eq12_e1373_d_n12, eq12_e1373_d_n13, eq12_e1373_d_n14, eq12_e1373_d_n15, eq12_e1373_d_n16, eq12_e1373_d_b0, eq12_e1373_d_b1, eq12_e1373_d_b2, eq12_e1373_d_b3, eq12_e1373_d_b4, eq12_e1373_d_b5, eq12_e1373_d_b6, eq12_e1373_d_b7, eq12_e1373_d_b8, eq12_e1373_d_b9, eq12_e1373_d_b10, eq12_e1373_d_b11, eq12_e1373_d_b12, eq12_e1373_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1375;let eq12_node_derivatives: [f64; 17] = [eq12_e1375_d_n0, eq12_e1375_d_n1, eq12_e1375_d_n2, eq12_e1375_d_n3, eq12_e1375_d_n4, eq12_e1375_d_n5, eq12_e1375_d_n6, eq12_e1375_d_n7, eq12_e1375_d_n8, eq12_e1375_d_n9, eq12_e1375_d_n10, eq12_e1375_d_n11, eq12_e1375_d_n12, eq12_e1375_d_n13, eq12_e1375_d_n14, eq12_e1375_d_n15, eq12_e1375_d_n16];let eq12_branch_derivatives: [f64; 14] = [eq12_e1375_d_b0, eq12_e1375_d_b1, eq12_e1375_d_b2, eq12_e1375_d_b3, eq12_e1375_d_b4, eq12_e1375_d_b5, eq12_e1375_d_b6, eq12_e1375_d_b7, eq12_e1375_d_b8, eq12_e1375_d_b9, eq12_e1375_d_b10, eq12_e1375_d_b11, eq12_e1375_d_b12, eq12_e1375_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
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
        let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let eq13_value: f64 = (nv16 - 0.0);
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq13_value),
            16,
            multiplicity * (1.0),
        );let eq14_value: f64 = (nv15 - 0.0);
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq14_value),
            15,
            multiplicity * (1.0),
        );let eq19_e1433: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, s.v[787]);let eq19_value: f64 = eq19_e1433;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq19_value),
            &s.dn[787],
            &s.db[787],
            (multiplicity) * (ddt_scale),
        );let eq20_e1435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, s.v[785]);let eq20_value: f64 = eq20_e1435;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq20_value),
            &s.dn[785],
            &s.db[785],
            (multiplicity) * (ddt_scale),
        );let eq21_e1437: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, s.v[786]);let eq21_value: f64 = eq21_e1437;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq21_value),
            &s.dn[786],
            &s.db[786],
            (multiplicity) * (ddt_scale),
        );let eq22_e1440: f64 = (-s.v[187]);let eq22_e1442: f64 = (eq22_e1440 * s.v[223]);let eq22_e1442_d_n0: f64 = (((-s.dn[187][0]) * s.v[223]) + (eq22_e1440 * s.dn[223][0]));let eq22_e1442_d_n1: f64 = (((-s.dn[187][1]) * s.v[223]) + (eq22_e1440 * s.dn[223][1]));let eq22_e1442_d_n2: f64 = (((-s.dn[187][2]) * s.v[223]) + (eq22_e1440 * s.dn[223][2]));let eq22_e1442_d_n3: f64 = (((-s.dn[187][3]) * s.v[223]) + (eq22_e1440 * s.dn[223][3]));let eq22_e1442_d_n4: f64 = (((-s.dn[187][4]) * s.v[223]) + (eq22_e1440 * s.dn[223][4]));let eq22_e1442_d_n5: f64 = (((-s.dn[187][5]) * s.v[223]) + (eq22_e1440 * s.dn[223][5]));let eq22_e1442_d_n6: f64 = (((-s.dn[187][6]) * s.v[223]) + (eq22_e1440 * s.dn[223][6]));let eq22_e1442_d_n7: f64 = (((-s.dn[187][7]) * s.v[223]) + (eq22_e1440 * s.dn[223][7]));let eq22_e1442_d_n8: f64 = (((-s.dn[187][8]) * s.v[223]) + (eq22_e1440 * s.dn[223][8]));let eq22_e1442_d_n9: f64 = (((-s.dn[187][9]) * s.v[223]) + (eq22_e1440 * s.dn[223][9]));let eq22_e1442_d_n10: f64 = (((-s.dn[187][10]) * s.v[223]) + (eq22_e1440 * s.dn[223][10]));let eq22_e1442_d_n11: f64 = (((-s.dn[187][11]) * s.v[223]) + (eq22_e1440 * s.dn[223][11]));let eq22_e1442_d_n12: f64 = (((-s.dn[187][12]) * s.v[223]) + (eq22_e1440 * s.dn[223][12]));let eq22_e1442_d_n13: f64 = (((-s.dn[187][13]) * s.v[223]) + (eq22_e1440 * s.dn[223][13]));let eq22_e1442_d_n14: f64 = (((-s.dn[187][14]) * s.v[223]) + (eq22_e1440 * s.dn[223][14]));let eq22_e1442_d_n15: f64 = (((-s.dn[187][15]) * s.v[223]) + (eq22_e1440 * s.dn[223][15]));let eq22_e1442_d_n16: f64 = (((-s.dn[187][16]) * s.v[223]) + (eq22_e1440 * s.dn[223][16]));let eq22_e1442_d_b0: f64 = (((-s.db[187][0]) * s.v[223]) + (eq22_e1440 * s.db[223][0]));let eq22_e1442_d_b1: f64 = (((-s.db[187][1]) * s.v[223]) + (eq22_e1440 * s.db[223][1]));let eq22_e1442_d_b2: f64 = (((-s.db[187][2]) * s.v[223]) + (eq22_e1440 * s.db[223][2]));let eq22_e1442_d_b3: f64 = (((-s.db[187][3]) * s.v[223]) + (eq22_e1440 * s.db[223][3]));let eq22_e1442_d_b4: f64 = (((-s.db[187][4]) * s.v[223]) + (eq22_e1440 * s.db[223][4]));let eq22_e1442_d_b5: f64 = (((-s.db[187][5]) * s.v[223]) + (eq22_e1440 * s.db[223][5]));let eq22_e1442_d_b6: f64 = (((-s.db[187][6]) * s.v[223]) + (eq22_e1440 * s.db[223][6]));let eq22_e1442_d_b7: f64 = (((-s.db[187][7]) * s.v[223]) + (eq22_e1440 * s.db[223][7]));let eq22_e1442_d_b8: f64 = (((-s.db[187][8]) * s.v[223]) + (eq22_e1440 * s.db[223][8]));let eq22_e1442_d_b9: f64 = (((-s.db[187][9]) * s.v[223]) + (eq22_e1440 * s.db[223][9]));let eq22_e1442_d_b10: f64 = (((-s.db[187][10]) * s.v[223]) + (eq22_e1440 * s.db[223][10]));let eq22_e1442_d_b11: f64 = (((-s.db[187][11]) * s.v[223]) + (eq22_e1440 * s.db[223][11]));let eq22_e1442_d_b12: f64 = (((-s.db[187][12]) * s.v[223]) + (eq22_e1440 * s.db[223][12]));let eq22_e1442_d_b13: f64 = (((-s.db[187][13]) * s.v[223]) + (eq22_e1440 * s.db[223][13]));let eq22_e1443: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e1442);let eq22_e1444: f64 = (p[29] * eq22_e1443);let eq22_e1444_d_n0: f64 = (p[29] * (eq22_e1442_d_n0 * ddt_scale));let eq22_e1444_d_n1: f64 = (p[29] * (eq22_e1442_d_n1 * ddt_scale));let eq22_e1444_d_n2: f64 = (p[29] * (eq22_e1442_d_n2 * ddt_scale));let eq22_e1444_d_n3: f64 = (p[29] * (eq22_e1442_d_n3 * ddt_scale));let eq22_e1444_d_n4: f64 = (p[29] * (eq22_e1442_d_n4 * ddt_scale));let eq22_e1444_d_n5: f64 = (p[29] * (eq22_e1442_d_n5 * ddt_scale));let eq22_e1444_d_n6: f64 = (p[29] * (eq22_e1442_d_n6 * ddt_scale));let eq22_e1444_d_n7: f64 = (p[29] * (eq22_e1442_d_n7 * ddt_scale));let eq22_e1444_d_n8: f64 = (p[29] * (eq22_e1442_d_n8 * ddt_scale));let eq22_e1444_d_n9: f64 = (p[29] * (eq22_e1442_d_n9 * ddt_scale));let eq22_e1444_d_n10: f64 = (p[29] * (eq22_e1442_d_n10 * ddt_scale));let eq22_e1444_d_n11: f64 = (p[29] * (eq22_e1442_d_n11 * ddt_scale));let eq22_e1444_d_n12: f64 = (p[29] * (eq22_e1442_d_n12 * ddt_scale));
        let eq22_e1444_d_n13: f64 = (p[29] * (eq22_e1442_d_n13 * ddt_scale));let eq22_e1444_d_n14: f64 = (p[29] * (eq22_e1442_d_n14 * ddt_scale));let eq22_e1444_d_n15: f64 = (p[29] * (eq22_e1442_d_n15 * ddt_scale));let eq22_e1444_d_n16: f64 = (p[29] * (eq22_e1442_d_n16 * ddt_scale));let eq22_e1444_d_b0: f64 = (p[29] * (eq22_e1442_d_b0 * ddt_scale));let eq22_e1444_d_b1: f64 = (p[29] * (eq22_e1442_d_b1 * ddt_scale));let eq22_e1444_d_b2: f64 = (p[29] * (eq22_e1442_d_b2 * ddt_scale));let eq22_e1444_d_b3: f64 = (p[29] * (eq22_e1442_d_b3 * ddt_scale));let eq22_e1444_d_b4: f64 = (p[29] * (eq22_e1442_d_b4 * ddt_scale));let eq22_e1444_d_b5: f64 = (p[29] * (eq22_e1442_d_b5 * ddt_scale));let eq22_e1444_d_b6: f64 = (p[29] * (eq22_e1442_d_b6 * ddt_scale));let eq22_e1444_d_b7: f64 = (p[29] * (eq22_e1442_d_b7 * ddt_scale));let eq22_e1444_d_b8: f64 = (p[29] * (eq22_e1442_d_b8 * ddt_scale));let eq22_e1444_d_b9: f64 = (p[29] * (eq22_e1442_d_b9 * ddt_scale));let eq22_e1444_d_b10: f64 = (p[29] * (eq22_e1442_d_b10 * ddt_scale));let eq22_e1444_d_b11: f64 = (p[29] * (eq22_e1442_d_b11 * ddt_scale));let eq22_e1444_d_b12: f64 = (p[29] * (eq22_e1442_d_b12 * ddt_scale));let eq22_e1444_d_b13: f64 = (p[29] * (eq22_e1442_d_b13 * ddt_scale));let eq22_value: f64 = eq22_e1444;let eq22_node_derivatives: [f64; 17] = [eq22_e1444_d_n0, eq22_e1444_d_n1, eq22_e1444_d_n2, eq22_e1444_d_n3, eq22_e1444_d_n4, eq22_e1444_d_n5, eq22_e1444_d_n6, eq22_e1444_d_n7, eq22_e1444_d_n8, eq22_e1444_d_n9, eq22_e1444_d_n10, eq22_e1444_d_n11, eq22_e1444_d_n12, eq22_e1444_d_n13, eq22_e1444_d_n14, eq22_e1444_d_n15, eq22_e1444_d_n16];let eq22_branch_derivatives: [f64; 14] = [eq22_e1444_d_b0, eq22_e1444_d_b1, eq22_e1444_d_b2, eq22_e1444_d_b3, eq22_e1444_d_b4, eq22_e1444_d_b5, eq22_e1444_d_b6, eq22_e1444_d_b7, eq22_e1444_d_b8, eq22_e1444_d_b9, eq22_e1444_d_b10, eq22_e1444_d_b11, eq22_e1444_d_b12, eq22_e1444_d_b13];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
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
        let eq23_e1447: f64 = (-s.v[187]);let eq23_e1449: f64 = (eq23_e1447 * s.v[224]);let eq23_e1449_d_n0: f64 = (((-s.dn[187][0]) * s.v[224]) + (eq23_e1447 * s.dn[224][0]));let eq23_e1449_d_n1: f64 = (((-s.dn[187][1]) * s.v[224]) + (eq23_e1447 * s.dn[224][1]));let eq23_e1449_d_n2: f64 = (((-s.dn[187][2]) * s.v[224]) + (eq23_e1447 * s.dn[224][2]));let eq23_e1449_d_n3: f64 = (((-s.dn[187][3]) * s.v[224]) + (eq23_e1447 * s.dn[224][3]));let eq23_e1449_d_n4: f64 = (((-s.dn[187][4]) * s.v[224]) + (eq23_e1447 * s.dn[224][4]));let eq23_e1449_d_n5: f64 = (((-s.dn[187][5]) * s.v[224]) + (eq23_e1447 * s.dn[224][5]));let eq23_e1449_d_n6: f64 = (((-s.dn[187][6]) * s.v[224]) + (eq23_e1447 * s.dn[224][6]));let eq23_e1449_d_n7: f64 = (((-s.dn[187][7]) * s.v[224]) + (eq23_e1447 * s.dn[224][7]));let eq23_e1449_d_n8: f64 = (((-s.dn[187][8]) * s.v[224]) + (eq23_e1447 * s.dn[224][8]));let eq23_e1449_d_n9: f64 = (((-s.dn[187][9]) * s.v[224]) + (eq23_e1447 * s.dn[224][9]));let eq23_e1449_d_n10: f64 = (((-s.dn[187][10]) * s.v[224]) + (eq23_e1447 * s.dn[224][10]));let eq23_e1449_d_n11: f64 = (((-s.dn[187][11]) * s.v[224]) + (eq23_e1447 * s.dn[224][11]));let eq23_e1449_d_n12: f64 = (((-s.dn[187][12]) * s.v[224]) + (eq23_e1447 * s.dn[224][12]));let eq23_e1449_d_n13: f64 = (((-s.dn[187][13]) * s.v[224]) + (eq23_e1447 * s.dn[224][13]));let eq23_e1449_d_n14: f64 = (((-s.dn[187][14]) * s.v[224]) + (eq23_e1447 * s.dn[224][14]));let eq23_e1449_d_n15: f64 = (((-s.dn[187][15]) * s.v[224]) + (eq23_e1447 * s.dn[224][15]));let eq23_e1449_d_n16: f64 = (((-s.dn[187][16]) * s.v[224]) + (eq23_e1447 * s.dn[224][16]));let eq23_e1449_d_b0: f64 = (((-s.db[187][0]) * s.v[224]) + (eq23_e1447 * s.db[224][0]));let eq23_e1449_d_b1: f64 = (((-s.db[187][1]) * s.v[224]) + (eq23_e1447 * s.db[224][1]));let eq23_e1449_d_b2: f64 = (((-s.db[187][2]) * s.v[224]) + (eq23_e1447 * s.db[224][2]));let eq23_e1449_d_b3: f64 = (((-s.db[187][3]) * s.v[224]) + (eq23_e1447 * s.db[224][3]));let eq23_e1449_d_b4: f64 = (((-s.db[187][4]) * s.v[224]) + (eq23_e1447 * s.db[224][4]));let eq23_e1449_d_b5: f64 = (((-s.db[187][5]) * s.v[224]) + (eq23_e1447 * s.db[224][5]));let eq23_e1449_d_b6: f64 = (((-s.db[187][6]) * s.v[224]) + (eq23_e1447 * s.db[224][6]));let eq23_e1449_d_b7: f64 = (((-s.db[187][7]) * s.v[224]) + (eq23_e1447 * s.db[224][7]));let eq23_e1449_d_b8: f64 = (((-s.db[187][8]) * s.v[224]) + (eq23_e1447 * s.db[224][8]));let eq23_e1449_d_b9: f64 = (((-s.db[187][9]) * s.v[224]) + (eq23_e1447 * s.db[224][9]));let eq23_e1449_d_b10: f64 = (((-s.db[187][10]) * s.v[224]) + (eq23_e1447 * s.db[224][10]));let eq23_e1449_d_b11: f64 = (((-s.db[187][11]) * s.v[224]) + (eq23_e1447 * s.db[224][11]));let eq23_e1449_d_b12: f64 = (((-s.db[187][12]) * s.v[224]) + (eq23_e1447 * s.db[224][12]));let eq23_e1449_d_b13: f64 = (((-s.db[187][13]) * s.v[224]) + (eq23_e1447 * s.db[224][13]));let eq23_e1450: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e1449);let eq23_e1451: f64 = (p[29] * eq23_e1450);let eq23_e1451_d_n0: f64 = (p[29] * (eq23_e1449_d_n0 * ddt_scale));let eq23_e1451_d_n1: f64 = (p[29] * (eq23_e1449_d_n1 * ddt_scale));let eq23_e1451_d_n2: f64 = (p[29] * (eq23_e1449_d_n2 * ddt_scale));let eq23_e1451_d_n3: f64 = (p[29] * (eq23_e1449_d_n3 * ddt_scale));let eq23_e1451_d_n4: f64 = (p[29] * (eq23_e1449_d_n4 * ddt_scale));let eq23_e1451_d_n5: f64 = (p[29] * (eq23_e1449_d_n5 * ddt_scale));let eq23_e1451_d_n6: f64 = (p[29] * (eq23_e1449_d_n6 * ddt_scale));let eq23_e1451_d_n7: f64 = (p[29] * (eq23_e1449_d_n7 * ddt_scale));let eq23_e1451_d_n8: f64 = (p[29] * (eq23_e1449_d_n8 * ddt_scale));let eq23_e1451_d_n9: f64 = (p[29] * (eq23_e1449_d_n9 * ddt_scale));let eq23_e1451_d_n10: f64 = (p[29] * (eq23_e1449_d_n10 * ddt_scale));let eq23_e1451_d_n11: f64 = (p[29] * (eq23_e1449_d_n11 * ddt_scale));let eq23_e1451_d_n12: f64 = (p[29] * (eq23_e1449_d_n12 * ddt_scale));
        let eq23_e1451_d_n13: f64 = (p[29] * (eq23_e1449_d_n13 * ddt_scale));let eq23_e1451_d_n14: f64 = (p[29] * (eq23_e1449_d_n14 * ddt_scale));let eq23_e1451_d_n15: f64 = (p[29] * (eq23_e1449_d_n15 * ddt_scale));let eq23_e1451_d_n16: f64 = (p[29] * (eq23_e1449_d_n16 * ddt_scale));let eq23_e1451_d_b0: f64 = (p[29] * (eq23_e1449_d_b0 * ddt_scale));let eq23_e1451_d_b1: f64 = (p[29] * (eq23_e1449_d_b1 * ddt_scale));let eq23_e1451_d_b2: f64 = (p[29] * (eq23_e1449_d_b2 * ddt_scale));let eq23_e1451_d_b3: f64 = (p[29] * (eq23_e1449_d_b3 * ddt_scale));let eq23_e1451_d_b4: f64 = (p[29] * (eq23_e1449_d_b4 * ddt_scale));let eq23_e1451_d_b5: f64 = (p[29] * (eq23_e1449_d_b5 * ddt_scale));let eq23_e1451_d_b6: f64 = (p[29] * (eq23_e1449_d_b6 * ddt_scale));let eq23_e1451_d_b7: f64 = (p[29] * (eq23_e1449_d_b7 * ddt_scale));let eq23_e1451_d_b8: f64 = (p[29] * (eq23_e1449_d_b8 * ddt_scale));let eq23_e1451_d_b9: f64 = (p[29] * (eq23_e1449_d_b9 * ddt_scale));let eq23_e1451_d_b10: f64 = (p[29] * (eq23_e1449_d_b10 * ddt_scale));let eq23_e1451_d_b11: f64 = (p[29] * (eq23_e1449_d_b11 * ddt_scale));let eq23_e1451_d_b12: f64 = (p[29] * (eq23_e1449_d_b12 * ddt_scale));let eq23_e1451_d_b13: f64 = (p[29] * (eq23_e1449_d_b13 * ddt_scale));let eq23_value: f64 = eq23_e1451;let eq23_node_derivatives: [f64; 17] = [eq23_e1451_d_n0, eq23_e1451_d_n1, eq23_e1451_d_n2, eq23_e1451_d_n3, eq23_e1451_d_n4, eq23_e1451_d_n5, eq23_e1451_d_n6, eq23_e1451_d_n7, eq23_e1451_d_n8, eq23_e1451_d_n9, eq23_e1451_d_n10, eq23_e1451_d_n11, eq23_e1451_d_n12, eq23_e1451_d_n13, eq23_e1451_d_n14, eq23_e1451_d_n15, eq23_e1451_d_n16];let eq23_branch_derivatives: [f64; 14] = [eq23_e1451_d_b0, eq23_e1451_d_b1, eq23_e1451_d_b2, eq23_e1451_d_b3, eq23_e1451_d_b4, eq23_e1451_d_b5, eq23_e1451_d_b6, eq23_e1451_d_b7, eq23_e1451_d_b8, eq23_e1451_d_b9, eq23_e1451_d_b10, eq23_e1451_d_b11, eq23_e1451_d_b12, eq23_e1451_d_b13];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
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
        let eq24_e1454: f64 = (-s.v[187]);let eq24_e1456: f64 = (eq24_e1454 * s.v[221]);let eq24_e1456_d_n0: f64 = (((-s.dn[187][0]) * s.v[221]) + (eq24_e1454 * s.dn[221][0]));let eq24_e1456_d_n1: f64 = (((-s.dn[187][1]) * s.v[221]) + (eq24_e1454 * s.dn[221][1]));let eq24_e1456_d_n2: f64 = (((-s.dn[187][2]) * s.v[221]) + (eq24_e1454 * s.dn[221][2]));let eq24_e1456_d_n3: f64 = (((-s.dn[187][3]) * s.v[221]) + (eq24_e1454 * s.dn[221][3]));let eq24_e1456_d_n4: f64 = (((-s.dn[187][4]) * s.v[221]) + (eq24_e1454 * s.dn[221][4]));let eq24_e1456_d_n5: f64 = (((-s.dn[187][5]) * s.v[221]) + (eq24_e1454 * s.dn[221][5]));let eq24_e1456_d_n6: f64 = (((-s.dn[187][6]) * s.v[221]) + (eq24_e1454 * s.dn[221][6]));let eq24_e1456_d_n7: f64 = (((-s.dn[187][7]) * s.v[221]) + (eq24_e1454 * s.dn[221][7]));let eq24_e1456_d_n8: f64 = (((-s.dn[187][8]) * s.v[221]) + (eq24_e1454 * s.dn[221][8]));let eq24_e1456_d_n9: f64 = (((-s.dn[187][9]) * s.v[221]) + (eq24_e1454 * s.dn[221][9]));let eq24_e1456_d_n10: f64 = (((-s.dn[187][10]) * s.v[221]) + (eq24_e1454 * s.dn[221][10]));let eq24_e1456_d_n11: f64 = (((-s.dn[187][11]) * s.v[221]) + (eq24_e1454 * s.dn[221][11]));let eq24_e1456_d_n12: f64 = (((-s.dn[187][12]) * s.v[221]) + (eq24_e1454 * s.dn[221][12]));let eq24_e1456_d_n13: f64 = (((-s.dn[187][13]) * s.v[221]) + (eq24_e1454 * s.dn[221][13]));let eq24_e1456_d_n14: f64 = (((-s.dn[187][14]) * s.v[221]) + (eq24_e1454 * s.dn[221][14]));let eq24_e1456_d_n15: f64 = (((-s.dn[187][15]) * s.v[221]) + (eq24_e1454 * s.dn[221][15]));let eq24_e1456_d_n16: f64 = (((-s.dn[187][16]) * s.v[221]) + (eq24_e1454 * s.dn[221][16]));let eq24_e1456_d_b0: f64 = (((-s.db[187][0]) * s.v[221]) + (eq24_e1454 * s.db[221][0]));let eq24_e1456_d_b1: f64 = (((-s.db[187][1]) * s.v[221]) + (eq24_e1454 * s.db[221][1]));let eq24_e1456_d_b2: f64 = (((-s.db[187][2]) * s.v[221]) + (eq24_e1454 * s.db[221][2]));let eq24_e1456_d_b3: f64 = (((-s.db[187][3]) * s.v[221]) + (eq24_e1454 * s.db[221][3]));let eq24_e1456_d_b4: f64 = (((-s.db[187][4]) * s.v[221]) + (eq24_e1454 * s.db[221][4]));let eq24_e1456_d_b5: f64 = (((-s.db[187][5]) * s.v[221]) + (eq24_e1454 * s.db[221][5]));let eq24_e1456_d_b6: f64 = (((-s.db[187][6]) * s.v[221]) + (eq24_e1454 * s.db[221][6]));let eq24_e1456_d_b7: f64 = (((-s.db[187][7]) * s.v[221]) + (eq24_e1454 * s.db[221][7]));let eq24_e1456_d_b8: f64 = (((-s.db[187][8]) * s.v[221]) + (eq24_e1454 * s.db[221][8]));let eq24_e1456_d_b9: f64 = (((-s.db[187][9]) * s.v[221]) + (eq24_e1454 * s.db[221][9]));let eq24_e1456_d_b10: f64 = (((-s.db[187][10]) * s.v[221]) + (eq24_e1454 * s.db[221][10]));let eq24_e1456_d_b11: f64 = (((-s.db[187][11]) * s.v[221]) + (eq24_e1454 * s.db[221][11]));let eq24_e1456_d_b12: f64 = (((-s.db[187][12]) * s.v[221]) + (eq24_e1454 * s.db[221][12]));let eq24_e1456_d_b13: f64 = (((-s.db[187][13]) * s.v[221]) + (eq24_e1454 * s.db[221][13]));let eq24_e1457: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq24_e1456);let eq24_e1458: f64 = (p[29] * eq24_e1457);let eq24_e1458_d_n0: f64 = (p[29] * (eq24_e1456_d_n0 * ddt_scale));let eq24_e1458_d_n1: f64 = (p[29] * (eq24_e1456_d_n1 * ddt_scale));let eq24_e1458_d_n2: f64 = (p[29] * (eq24_e1456_d_n2 * ddt_scale));let eq24_e1458_d_n3: f64 = (p[29] * (eq24_e1456_d_n3 * ddt_scale));let eq24_e1458_d_n4: f64 = (p[29] * (eq24_e1456_d_n4 * ddt_scale));let eq24_e1458_d_n5: f64 = (p[29] * (eq24_e1456_d_n5 * ddt_scale));let eq24_e1458_d_n6: f64 = (p[29] * (eq24_e1456_d_n6 * ddt_scale));let eq24_e1458_d_n7: f64 = (p[29] * (eq24_e1456_d_n7 * ddt_scale));let eq24_e1458_d_n8: f64 = (p[29] * (eq24_e1456_d_n8 * ddt_scale));let eq24_e1458_d_n9: f64 = (p[29] * (eq24_e1456_d_n9 * ddt_scale));let eq24_e1458_d_n10: f64 = (p[29] * (eq24_e1456_d_n10 * ddt_scale));let eq24_e1458_d_n11: f64 = (p[29] * (eq24_e1456_d_n11 * ddt_scale));let eq24_e1458_d_n12: f64 = (p[29] * (eq24_e1456_d_n12 * ddt_scale));
        let eq24_e1458_d_n13: f64 = (p[29] * (eq24_e1456_d_n13 * ddt_scale));let eq24_e1458_d_n14: f64 = (p[29] * (eq24_e1456_d_n14 * ddt_scale));let eq24_e1458_d_n15: f64 = (p[29] * (eq24_e1456_d_n15 * ddt_scale));let eq24_e1458_d_n16: f64 = (p[29] * (eq24_e1456_d_n16 * ddt_scale));let eq24_e1458_d_b0: f64 = (p[29] * (eq24_e1456_d_b0 * ddt_scale));let eq24_e1458_d_b1: f64 = (p[29] * (eq24_e1456_d_b1 * ddt_scale));let eq24_e1458_d_b2: f64 = (p[29] * (eq24_e1456_d_b2 * ddt_scale));let eq24_e1458_d_b3: f64 = (p[29] * (eq24_e1456_d_b3 * ddt_scale));let eq24_e1458_d_b4: f64 = (p[29] * (eq24_e1456_d_b4 * ddt_scale));let eq24_e1458_d_b5: f64 = (p[29] * (eq24_e1456_d_b5 * ddt_scale));let eq24_e1458_d_b6: f64 = (p[29] * (eq24_e1456_d_b6 * ddt_scale));let eq24_e1458_d_b7: f64 = (p[29] * (eq24_e1456_d_b7 * ddt_scale));let eq24_e1458_d_b8: f64 = (p[29] * (eq24_e1456_d_b8 * ddt_scale));let eq24_e1458_d_b9: f64 = (p[29] * (eq24_e1456_d_b9 * ddt_scale));let eq24_e1458_d_b10: f64 = (p[29] * (eq24_e1456_d_b10 * ddt_scale));let eq24_e1458_d_b11: f64 = (p[29] * (eq24_e1456_d_b11 * ddt_scale));let eq24_e1458_d_b12: f64 = (p[29] * (eq24_e1456_d_b12 * ddt_scale));let eq24_e1458_d_b13: f64 = (p[29] * (eq24_e1456_d_b13 * ddt_scale));let eq24_value: f64 = eq24_e1458;let eq24_node_derivatives: [f64; 17] = [eq24_e1458_d_n0, eq24_e1458_d_n1, eq24_e1458_d_n2, eq24_e1458_d_n3, eq24_e1458_d_n4, eq24_e1458_d_n5, eq24_e1458_d_n6, eq24_e1458_d_n7, eq24_e1458_d_n8, eq24_e1458_d_n9, eq24_e1458_d_n10, eq24_e1458_d_n11, eq24_e1458_d_n12, eq24_e1458_d_n13, eq24_e1458_d_n14, eq24_e1458_d_n15, eq24_e1458_d_n16];let eq24_branch_derivatives: [f64; 14] = [eq24_e1458_d_b0, eq24_e1458_d_b1, eq24_e1458_d_b2, eq24_e1458_d_b3, eq24_e1458_d_b4, eq24_e1458_d_b5, eq24_e1458_d_b6, eq24_e1458_d_b7, eq24_e1458_d_b8, eq24_e1458_d_b9, eq24_e1458_d_b10, eq24_e1458_d_b11, eq24_e1458_d_b12, eq24_e1458_d_b13];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq25_e1461: f64 = (s.v[187] * p[28]);let eq25_e1463: f64 = (eq25_e1461 * s.v[57]);let eq25_e1463_d_n0: f64 = (((s.dn[187][0] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][0]));let eq25_e1463_d_n1: f64 = (((s.dn[187][1] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][1]));let eq25_e1463_d_n2: f64 = (((s.dn[187][2] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][2]));let eq25_e1463_d_n3: f64 = (((s.dn[187][3] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][3]));let eq25_e1463_d_n4: f64 = (((s.dn[187][4] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][4]));let eq25_e1463_d_n5: f64 = (((s.dn[187][5] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][5]));let eq25_e1463_d_n6: f64 = (((s.dn[187][6] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][6]));let eq25_e1463_d_n7: f64 = (((s.dn[187][7] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][7]));let eq25_e1463_d_n8: f64 = (((s.dn[187][8] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][8]));let eq25_e1463_d_n9: f64 = (((s.dn[187][9] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][9]));let eq25_e1463_d_n10: f64 = (((s.dn[187][10] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][10]));let eq25_e1463_d_n11: f64 = (((s.dn[187][11] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][11]));let eq25_e1463_d_n12: f64 = (((s.dn[187][12] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][12]));let eq25_e1463_d_n13: f64 = (((s.dn[187][13] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][13]));let eq25_e1463_d_n14: f64 = (((s.dn[187][14] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][14]));let eq25_e1463_d_n15: f64 = (((s.dn[187][15] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][15]));let eq25_e1463_d_n16: f64 = (((s.dn[187][16] * p[28]) * s.v[57]) + (eq25_e1461 * s.dn[57][16]));let eq25_e1463_d_b0: f64 = (((s.db[187][0] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][0]));let eq25_e1463_d_b1: f64 = (((s.db[187][1] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][1]));let eq25_e1463_d_b2: f64 = (((s.db[187][2] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][2]));let eq25_e1463_d_b3: f64 = (((s.db[187][3] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][3]));let eq25_e1463_d_b4: f64 = (((s.db[187][4] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][4]));let eq25_e1463_d_b5: f64 = (((s.db[187][5] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][5]));let eq25_e1463_d_b6: f64 = (((s.db[187][6] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][6]));let eq25_e1463_d_b7: f64 = (((s.db[187][7] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][7]));let eq25_e1463_d_b8: f64 = (((s.db[187][8] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][8]));let eq25_e1463_d_b9: f64 = (((s.db[187][9] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][9]));let eq25_e1463_d_b10: f64 = (((s.db[187][10] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][10]));let eq25_e1463_d_b11: f64 = (((s.db[187][11] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][11]));let eq25_e1463_d_b12: f64 = (((s.db[187][12] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][12]));let eq25_e1463_d_b13: f64 = (((s.db[187][13] * p[28]) * s.v[57]) + (eq25_e1461 * s.db[57][13]));let eq25_e1465: f64 = (eq25_e1463 * s.v[188]);let eq25_e1465_d_n0: f64 = ((eq25_e1463_d_n0 * s.v[188]) + (eq25_e1463 * s.dn[188][0]));let eq25_e1465_d_n1: f64 = ((eq25_e1463_d_n1 * s.v[188]) + (eq25_e1463 * s.dn[188][1]));let eq25_e1465_d_n2: f64 = ((eq25_e1463_d_n2 * s.v[188]) + (eq25_e1463 * s.dn[188][2]));let eq25_e1465_d_n3: f64 = ((eq25_e1463_d_n3 * s.v[188]) + (eq25_e1463 * s.dn[188][3]));let eq25_e1465_d_n4: f64 = ((eq25_e1463_d_n4 * s.v[188]) + (eq25_e1463 * s.dn[188][4]));let eq25_e1465_d_n5: f64 = ((eq25_e1463_d_n5 * s.v[188]) + (eq25_e1463 * s.dn[188][5]));let eq25_e1465_d_n6: f64 = ((eq25_e1463_d_n6 * s.v[188]) + (eq25_e1463 * s.dn[188][6]));let eq25_e1465_d_n7: f64 = ((eq25_e1463_d_n7 * s.v[188]) + (eq25_e1463 * s.dn[188][7]));let eq25_e1465_d_n8: f64 = ((eq25_e1463_d_n8 * s.v[188]) + (eq25_e1463 * s.dn[188][8]));let eq25_e1465_d_n9: f64 = ((eq25_e1463_d_n9 * s.v[188]) + (eq25_e1463 * s.dn[188][9]));let eq25_e1465_d_n10: f64 = ((eq25_e1463_d_n10 * s.v[188]) + (eq25_e1463 * s.dn[188][10]));
        let eq25_e1465_d_n11: f64 = ((eq25_e1463_d_n11 * s.v[188]) + (eq25_e1463 * s.dn[188][11]));let eq25_e1465_d_n12: f64 = ((eq25_e1463_d_n12 * s.v[188]) + (eq25_e1463 * s.dn[188][12]));let eq25_e1465_d_n13: f64 = ((eq25_e1463_d_n13 * s.v[188]) + (eq25_e1463 * s.dn[188][13]));let eq25_e1465_d_n14: f64 = ((eq25_e1463_d_n14 * s.v[188]) + (eq25_e1463 * s.dn[188][14]));let eq25_e1465_d_n15: f64 = ((eq25_e1463_d_n15 * s.v[188]) + (eq25_e1463 * s.dn[188][15]));let eq25_e1465_d_n16: f64 = ((eq25_e1463_d_n16 * s.v[188]) + (eq25_e1463 * s.dn[188][16]));let eq25_e1465_d_b0: f64 = ((eq25_e1463_d_b0 * s.v[188]) + (eq25_e1463 * s.db[188][0]));let eq25_e1465_d_b1: f64 = ((eq25_e1463_d_b1 * s.v[188]) + (eq25_e1463 * s.db[188][1]));let eq25_e1465_d_b2: f64 = ((eq25_e1463_d_b2 * s.v[188]) + (eq25_e1463 * s.db[188][2]));let eq25_e1465_d_b3: f64 = ((eq25_e1463_d_b3 * s.v[188]) + (eq25_e1463 * s.db[188][3]));let eq25_e1465_d_b4: f64 = ((eq25_e1463_d_b4 * s.v[188]) + (eq25_e1463 * s.db[188][4]));let eq25_e1465_d_b5: f64 = ((eq25_e1463_d_b5 * s.v[188]) + (eq25_e1463 * s.db[188][5]));let eq25_e1465_d_b6: f64 = ((eq25_e1463_d_b6 * s.v[188]) + (eq25_e1463 * s.db[188][6]));let eq25_e1465_d_b7: f64 = ((eq25_e1463_d_b7 * s.v[188]) + (eq25_e1463 * s.db[188][7]));let eq25_e1465_d_b8: f64 = ((eq25_e1463_d_b8 * s.v[188]) + (eq25_e1463 * s.db[188][8]));let eq25_e1465_d_b9: f64 = ((eq25_e1463_d_b9 * s.v[188]) + (eq25_e1463 * s.db[188][9]));let eq25_e1465_d_b10: f64 = ((eq25_e1463_d_b10 * s.v[188]) + (eq25_e1463 * s.db[188][10]));let eq25_e1465_d_b11: f64 = ((eq25_e1463_d_b11 * s.v[188]) + (eq25_e1463 * s.db[188][11]));let eq25_e1465_d_b12: f64 = ((eq25_e1463_d_b12 * s.v[188]) + (eq25_e1463 * s.db[188][12]));let eq25_e1465_d_b13: f64 = ((eq25_e1463_d_b13 * s.v[188]) + (eq25_e1463 * s.db[188][13]));let eq25_value: f64 = eq25_e1465;let eq25_node_derivatives: [f64; 17] = [eq25_e1465_d_n0, eq25_e1465_d_n1, eq25_e1465_d_n2, eq25_e1465_d_n3, eq25_e1465_d_n4, eq25_e1465_d_n5, eq25_e1465_d_n6, eq25_e1465_d_n7, eq25_e1465_d_n8, eq25_e1465_d_n9, eq25_e1465_d_n10, eq25_e1465_d_n11, eq25_e1465_d_n12, eq25_e1465_d_n13, eq25_e1465_d_n14, eq25_e1465_d_n15, eq25_e1465_d_n16];let eq25_branch_derivatives: [f64; 14] = [eq25_e1465_d_b0, eq25_e1465_d_b1, eq25_e1465_d_b2, eq25_e1465_d_b3, eq25_e1465_d_b4, eq25_e1465_d_b5, eq25_e1465_d_b6, eq25_e1465_d_b7, eq25_e1465_d_b8, eq25_e1465_d_b9, eq25_e1465_d_b10, eq25_e1465_d_b11, eq25_e1465_d_b12, eq25_e1465_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1469, eq26_e1469_d_n0, eq26_e1469_d_n1, eq26_e1469_d_n2, eq26_e1469_d_n3, eq26_e1469_d_n4, eq26_e1469_d_n5, eq26_e1469_d_n6, eq26_e1469_d_n7, eq26_e1469_d_n8, eq26_e1469_d_n9, eq26_e1469_d_n10, eq26_e1469_d_n11, eq26_e1469_d_n12, eq26_e1469_d_n13, eq26_e1469_d_n14, eq26_e1469_d_n15, eq26_e1469_d_n16, eq26_e1469_d_b0, eq26_e1469_d_b1, eq26_e1469_d_b2, eq26_e1469_d_b3, eq26_e1469_d_b4, eq26_e1469_d_b5, eq26_e1469_d_b6, eq26_e1469_d_b7, eq26_e1469_d_b8, eq26_e1469_d_b9, eq26_e1469_d_b10, eq26_e1469_d_b11, eq26_e1469_d_b12, eq26_e1469_d_b13,) = {
    if s.b[1609] {
        (s.v[831], s.dn[831][0], s.dn[831][1], s.dn[831][2], s.dn[831][3], s.dn[831][4], s.dn[831][5], s.dn[831][6], s.dn[831][7], s.dn[831][8], s.dn[831][9], s.dn[831][10], s.dn[831][11], s.dn[831][12], s.dn[831][13], s.dn[831][14], s.dn[831][15], s.dn[831][16], s.db[831][0], s.db[831][1], s.db[831][2], s.db[831][3], s.db[831][4], s.db[831][5], s.db[831][6], s.db[831][7], s.db[831][8], s.db[831][9], s.db[831][10], s.db[831][11], s.db[831][12], s.db[831][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1469;let eq26_node_derivatives: [f64; 17] = [eq26_e1469_d_n0, eq26_e1469_d_n1, eq26_e1469_d_n2, eq26_e1469_d_n3, eq26_e1469_d_n4, eq26_e1469_d_n5, eq26_e1469_d_n6, eq26_e1469_d_n7, eq26_e1469_d_n8, eq26_e1469_d_n9, eq26_e1469_d_n10, eq26_e1469_d_n11, eq26_e1469_d_n12, eq26_e1469_d_n13, eq26_e1469_d_n14, eq26_e1469_d_n15, eq26_e1469_d_n16];let eq26_branch_derivatives: [f64; 14] = [eq26_e1469_d_b0, eq26_e1469_d_b1, eq26_e1469_d_b2, eq26_e1469_d_b3, eq26_e1469_d_b4, eq26_e1469_d_b5, eq26_e1469_d_b6, eq26_e1469_d_b7, eq26_e1469_d_b8, eq26_e1469_d_b9, eq26_e1469_d_b10, eq26_e1469_d_b11, eq26_e1469_d_b12, eq26_e1469_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq27_e1475, eq27_e1475_d_n0, eq27_e1475_d_n1, eq27_e1475_d_n2, eq27_e1475_d_n3, eq27_e1475_d_n4, eq27_e1475_d_n5, eq27_e1475_d_n6, eq27_e1475_d_n7, eq27_e1475_d_n8, eq27_e1475_d_n9, eq27_e1475_d_n10, eq27_e1475_d_n11, eq27_e1475_d_n12, eq27_e1475_d_n13, eq27_e1475_d_n14, eq27_e1475_d_n15, eq27_e1475_d_n16, eq27_e1475_d_b0, eq27_e1475_d_b1, eq27_e1475_d_b2, eq27_e1475_d_b3, eq27_e1475_d_b4, eq27_e1475_d_b5, eq27_e1475_d_b6, eq27_e1475_d_b7, eq27_e1475_d_b8, eq27_e1475_d_b9, eq27_e1475_d_b10, eq27_e1475_d_b11, eq27_e1475_d_b12, eq27_e1475_d_b13,) = {
    if s.b[1610] {
        let eq27_e1473: f64 = (s.v[827] + s.v[829]);let eq27_e1473_d_n0: f64 = (s.dn[827][0] + s.dn[829][0]);let eq27_e1473_d_n1: f64 = (s.dn[827][1] + s.dn[829][1]);let eq27_e1473_d_n2: f64 = (s.dn[827][2] + s.dn[829][2]);let eq27_e1473_d_n3: f64 = (s.dn[827][3] + s.dn[829][3]);let eq27_e1473_d_n4: f64 = (s.dn[827][4] + s.dn[829][4]);let eq27_e1473_d_n5: f64 = (s.dn[827][5] + s.dn[829][5]);let eq27_e1473_d_n6: f64 = (s.dn[827][6] + s.dn[829][6]);let eq27_e1473_d_n7: f64 = (s.dn[827][7] + s.dn[829][7]);let eq27_e1473_d_n8: f64 = (s.dn[827][8] + s.dn[829][8]);let eq27_e1473_d_n9: f64 = (s.dn[827][9] + s.dn[829][9]);let eq27_e1473_d_n10: f64 = (s.dn[827][10] + s.dn[829][10]);let eq27_e1473_d_n11: f64 = (s.dn[827][11] + s.dn[829][11]);let eq27_e1473_d_n12: f64 = (s.dn[827][12] + s.dn[829][12]);let eq27_e1473_d_n13: f64 = (s.dn[827][13] + s.dn[829][13]);let eq27_e1473_d_n14: f64 = (s.dn[827][14] + s.dn[829][14]);let eq27_e1473_d_n15: f64 = (s.dn[827][15] + s.dn[829][15]);let eq27_e1473_d_n16: f64 = (s.dn[827][16] + s.dn[829][16]);let eq27_e1473_d_b0: f64 = (s.db[827][0] + s.db[829][0]);let eq27_e1473_d_b1: f64 = (s.db[827][1] + s.db[829][1]);let eq27_e1473_d_b2: f64 = (s.db[827][2] + s.db[829][2]);let eq27_e1473_d_b3: f64 = (s.db[827][3] + s.db[829][3]);let eq27_e1473_d_b4: f64 = (s.db[827][4] + s.db[829][4]);let eq27_e1473_d_b5: f64 = (s.db[827][5] + s.db[829][5]);let eq27_e1473_d_b6: f64 = (s.db[827][6] + s.db[829][6]);let eq27_e1473_d_b7: f64 = (s.db[827][7] + s.db[829][7]);let eq27_e1473_d_b8: f64 = (s.db[827][8] + s.db[829][8]);let eq27_e1473_d_b9: f64 = (s.db[827][9] + s.db[829][9]);let eq27_e1473_d_b10: f64 = (s.db[827][10] + s.db[829][10]);let eq27_e1473_d_b11: f64 = (s.db[827][11] + s.db[829][11]);let eq27_e1473_d_b12: f64 = (s.db[827][12] + s.db[829][12]);let eq27_e1473_d_b13: f64 = (s.db[827][13] + s.db[829][13]);
        (eq27_e1473, eq27_e1473_d_n0, eq27_e1473_d_n1, eq27_e1473_d_n2, eq27_e1473_d_n3, eq27_e1473_d_n4, eq27_e1473_d_n5, eq27_e1473_d_n6, eq27_e1473_d_n7, eq27_e1473_d_n8, eq27_e1473_d_n9, eq27_e1473_d_n10, eq27_e1473_d_n11, eq27_e1473_d_n12, eq27_e1473_d_n13, eq27_e1473_d_n14, eq27_e1473_d_n15, eq27_e1473_d_n16, eq27_e1473_d_b0, eq27_e1473_d_b1, eq27_e1473_d_b2, eq27_e1473_d_b3, eq27_e1473_d_b4, eq27_e1473_d_b5, eq27_e1473_d_b6, eq27_e1473_d_b7, eq27_e1473_d_b8, eq27_e1473_d_b9, eq27_e1473_d_b10, eq27_e1473_d_b11, eq27_e1473_d_b12, eq27_e1473_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1475;let eq27_node_derivatives: [f64; 17] = [eq27_e1475_d_n0, eq27_e1475_d_n1, eq27_e1475_d_n2, eq27_e1475_d_n3, eq27_e1475_d_n4, eq27_e1475_d_n5, eq27_e1475_d_n6, eq27_e1475_d_n7, eq27_e1475_d_n8, eq27_e1475_d_n9, eq27_e1475_d_n10, eq27_e1475_d_n11, eq27_e1475_d_n12, eq27_e1475_d_n13, eq27_e1475_d_n14, eq27_e1475_d_n15, eq27_e1475_d_n16];let eq27_branch_derivatives: [f64; 14] = [eq27_e1475_d_b0, eq27_e1475_d_b1, eq27_e1475_d_b2, eq27_e1475_d_b3, eq27_e1475_d_b4, eq27_e1475_d_b5, eq27_e1475_d_b6, eq27_e1475_d_b7, eq27_e1475_d_b8, eq27_e1475_d_b9, eq27_e1475_d_b10, eq27_e1475_d_b11, eq27_e1475_d_b12, eq27_e1475_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1481, eq28_e1481_d_n0, eq28_e1481_d_n1, eq28_e1481_d_n2, eq28_e1481_d_n3, eq28_e1481_d_n4, eq28_e1481_d_n5, eq28_e1481_d_n6, eq28_e1481_d_n7, eq28_e1481_d_n8, eq28_e1481_d_n9, eq28_e1481_d_n10, eq28_e1481_d_n11, eq28_e1481_d_n12, eq28_e1481_d_n13, eq28_e1481_d_n14, eq28_e1481_d_n15, eq28_e1481_d_n16, eq28_e1481_d_b0, eq28_e1481_d_b1, eq28_e1481_d_b2, eq28_e1481_d_b3, eq28_e1481_d_b4, eq28_e1481_d_b5, eq28_e1481_d_b6, eq28_e1481_d_b7, eq28_e1481_d_b8, eq28_e1481_d_b9, eq28_e1481_d_b10, eq28_e1481_d_b11, eq28_e1481_d_b12, eq28_e1481_d_b13,) = {
    if s.b[1610] {
        let eq28_e1479: f64 = (s.v[828] + s.v[830]);let eq28_e1479_d_n0: f64 = (s.dn[828][0] + s.dn[830][0]);let eq28_e1479_d_n1: f64 = (s.dn[828][1] + s.dn[830][1]);let eq28_e1479_d_n2: f64 = (s.dn[828][2] + s.dn[830][2]);let eq28_e1479_d_n3: f64 = (s.dn[828][3] + s.dn[830][3]);let eq28_e1479_d_n4: f64 = (s.dn[828][4] + s.dn[830][4]);let eq28_e1479_d_n5: f64 = (s.dn[828][5] + s.dn[830][5]);let eq28_e1479_d_n6: f64 = (s.dn[828][6] + s.dn[830][6]);let eq28_e1479_d_n7: f64 = (s.dn[828][7] + s.dn[830][7]);let eq28_e1479_d_n8: f64 = (s.dn[828][8] + s.dn[830][8]);let eq28_e1479_d_n9: f64 = (s.dn[828][9] + s.dn[830][9]);let eq28_e1479_d_n10: f64 = (s.dn[828][10] + s.dn[830][10]);let eq28_e1479_d_n11: f64 = (s.dn[828][11] + s.dn[830][11]);let eq28_e1479_d_n12: f64 = (s.dn[828][12] + s.dn[830][12]);let eq28_e1479_d_n13: f64 = (s.dn[828][13] + s.dn[830][13]);let eq28_e1479_d_n14: f64 = (s.dn[828][14] + s.dn[830][14]);let eq28_e1479_d_n15: f64 = (s.dn[828][15] + s.dn[830][15]);let eq28_e1479_d_n16: f64 = (s.dn[828][16] + s.dn[830][16]);let eq28_e1479_d_b0: f64 = (s.db[828][0] + s.db[830][0]);let eq28_e1479_d_b1: f64 = (s.db[828][1] + s.db[830][1]);let eq28_e1479_d_b2: f64 = (s.db[828][2] + s.db[830][2]);let eq28_e1479_d_b3: f64 = (s.db[828][3] + s.db[830][3]);let eq28_e1479_d_b4: f64 = (s.db[828][4] + s.db[830][4]);let eq28_e1479_d_b5: f64 = (s.db[828][5] + s.db[830][5]);let eq28_e1479_d_b6: f64 = (s.db[828][6] + s.db[830][6]);let eq28_e1479_d_b7: f64 = (s.db[828][7] + s.db[830][7]);let eq28_e1479_d_b8: f64 = (s.db[828][8] + s.db[830][8]);let eq28_e1479_d_b9: f64 = (s.db[828][9] + s.db[830][9]);let eq28_e1479_d_b10: f64 = (s.db[828][10] + s.db[830][10]);let eq28_e1479_d_b11: f64 = (s.db[828][11] + s.db[830][11]);let eq28_e1479_d_b12: f64 = (s.db[828][12] + s.db[830][12]);let eq28_e1479_d_b13: f64 = (s.db[828][13] + s.db[830][13]);
        (eq28_e1479, eq28_e1479_d_n0, eq28_e1479_d_n1, eq28_e1479_d_n2, eq28_e1479_d_n3, eq28_e1479_d_n4, eq28_e1479_d_n5, eq28_e1479_d_n6, eq28_e1479_d_n7, eq28_e1479_d_n8, eq28_e1479_d_n9, eq28_e1479_d_n10, eq28_e1479_d_n11, eq28_e1479_d_n12, eq28_e1479_d_n13, eq28_e1479_d_n14, eq28_e1479_d_n15, eq28_e1479_d_n16, eq28_e1479_d_b0, eq28_e1479_d_b1, eq28_e1479_d_b2, eq28_e1479_d_b3, eq28_e1479_d_b4, eq28_e1479_d_b5, eq28_e1479_d_b6, eq28_e1479_d_b7, eq28_e1479_d_b8, eq28_e1479_d_b9, eq28_e1479_d_b10, eq28_e1479_d_b11, eq28_e1479_d_b12, eq28_e1479_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1481;let eq28_node_derivatives: [f64; 17] = [eq28_e1481_d_n0, eq28_e1481_d_n1, eq28_e1481_d_n2, eq28_e1481_d_n3, eq28_e1481_d_n4, eq28_e1481_d_n5, eq28_e1481_d_n6, eq28_e1481_d_n7, eq28_e1481_d_n8, eq28_e1481_d_n9, eq28_e1481_d_n10, eq28_e1481_d_n11, eq28_e1481_d_n12, eq28_e1481_d_n13, eq28_e1481_d_n14, eq28_e1481_d_n15, eq28_e1481_d_n16];let eq28_branch_derivatives: [f64; 14] = [eq28_e1481_d_b0, eq28_e1481_d_b1, eq28_e1481_d_b2, eq28_e1481_d_b3, eq28_e1481_d_b4, eq28_e1481_d_b5, eq28_e1481_d_b6, eq28_e1481_d_b7, eq28_e1481_d_b8, eq28_e1481_d_b9, eq28_e1481_d_b10, eq28_e1481_d_b11, eq28_e1481_d_b12, eq28_e1481_d_b13];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq29_e1487, eq29_e1487_d_n0, eq29_e1487_d_n1, eq29_e1487_d_n2, eq29_e1487_d_n3, eq29_e1487_d_n4, eq29_e1487_d_n5, eq29_e1487_d_n6, eq29_e1487_d_n7, eq29_e1487_d_n8, eq29_e1487_d_n9, eq29_e1487_d_n10, eq29_e1487_d_n11, eq29_e1487_d_n12, eq29_e1487_d_n13, eq29_e1487_d_n14, eq29_e1487_d_n15, eq29_e1487_d_n16, eq29_e1487_d_b0, eq29_e1487_d_b1, eq29_e1487_d_b2, eq29_e1487_d_b3, eq29_e1487_d_b4, eq29_e1487_d_b5, eq29_e1487_d_b6, eq29_e1487_d_b7, eq29_e1487_d_b8, eq29_e1487_d_b9, eq29_e1487_d_b10, eq29_e1487_d_b11, eq29_e1487_d_b12, eq29_e1487_d_b13,) = {
    if s.b[1611] {
        let eq29_e1485: f64 = (s.v[824] + s.v[825]);let eq29_e1485_d_n0: f64 = (s.dn[824][0] + s.dn[825][0]);let eq29_e1485_d_n1: f64 = (s.dn[824][1] + s.dn[825][1]);let eq29_e1485_d_n2: f64 = (s.dn[824][2] + s.dn[825][2]);let eq29_e1485_d_n3: f64 = (s.dn[824][3] + s.dn[825][3]);let eq29_e1485_d_n4: f64 = (s.dn[824][4] + s.dn[825][4]);let eq29_e1485_d_n5: f64 = (s.dn[824][5] + s.dn[825][5]);let eq29_e1485_d_n6: f64 = (s.dn[824][6] + s.dn[825][6]);let eq29_e1485_d_n7: f64 = (s.dn[824][7] + s.dn[825][7]);let eq29_e1485_d_n8: f64 = (s.dn[824][8] + s.dn[825][8]);let eq29_e1485_d_n9: f64 = (s.dn[824][9] + s.dn[825][9]);let eq29_e1485_d_n10: f64 = (s.dn[824][10] + s.dn[825][10]);let eq29_e1485_d_n11: f64 = (s.dn[824][11] + s.dn[825][11]);let eq29_e1485_d_n12: f64 = (s.dn[824][12] + s.dn[825][12]);let eq29_e1485_d_n13: f64 = (s.dn[824][13] + s.dn[825][13]);let eq29_e1485_d_n14: f64 = (s.dn[824][14] + s.dn[825][14]);let eq29_e1485_d_n15: f64 = (s.dn[824][15] + s.dn[825][15]);let eq29_e1485_d_n16: f64 = (s.dn[824][16] + s.dn[825][16]);let eq29_e1485_d_b0: f64 = (s.db[824][0] + s.db[825][0]);let eq29_e1485_d_b1: f64 = (s.db[824][1] + s.db[825][1]);let eq29_e1485_d_b2: f64 = (s.db[824][2] + s.db[825][2]);let eq29_e1485_d_b3: f64 = (s.db[824][3] + s.db[825][3]);let eq29_e1485_d_b4: f64 = (s.db[824][4] + s.db[825][4]);let eq29_e1485_d_b5: f64 = (s.db[824][5] + s.db[825][5]);let eq29_e1485_d_b6: f64 = (s.db[824][6] + s.db[825][6]);let eq29_e1485_d_b7: f64 = (s.db[824][7] + s.db[825][7]);let eq29_e1485_d_b8: f64 = (s.db[824][8] + s.db[825][8]);let eq29_e1485_d_b9: f64 = (s.db[824][9] + s.db[825][9]);let eq29_e1485_d_b10: f64 = (s.db[824][10] + s.db[825][10]);let eq29_e1485_d_b11: f64 = (s.db[824][11] + s.db[825][11]);let eq29_e1485_d_b12: f64 = (s.db[824][12] + s.db[825][12]);let eq29_e1485_d_b13: f64 = (s.db[824][13] + s.db[825][13]);
        (eq29_e1485, eq29_e1485_d_n0, eq29_e1485_d_n1, eq29_e1485_d_n2, eq29_e1485_d_n3, eq29_e1485_d_n4, eq29_e1485_d_n5, eq29_e1485_d_n6, eq29_e1485_d_n7, eq29_e1485_d_n8, eq29_e1485_d_n9, eq29_e1485_d_n10, eq29_e1485_d_n11, eq29_e1485_d_n12, eq29_e1485_d_n13, eq29_e1485_d_n14, eq29_e1485_d_n15, eq29_e1485_d_n16, eq29_e1485_d_b0, eq29_e1485_d_b1, eq29_e1485_d_b2, eq29_e1485_d_b3, eq29_e1485_d_b4, eq29_e1485_d_b5, eq29_e1485_d_b6, eq29_e1485_d_b7, eq29_e1485_d_b8, eq29_e1485_d_b9, eq29_e1485_d_b10, eq29_e1485_d_b11, eq29_e1485_d_b12, eq29_e1485_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1487;let eq29_node_derivatives: [f64; 17] = [eq29_e1487_d_n0, eq29_e1487_d_n1, eq29_e1487_d_n2, eq29_e1487_d_n3, eq29_e1487_d_n4, eq29_e1487_d_n5, eq29_e1487_d_n6, eq29_e1487_d_n7, eq29_e1487_d_n8, eq29_e1487_d_n9, eq29_e1487_d_n10, eq29_e1487_d_n11, eq29_e1487_d_n12, eq29_e1487_d_n13, eq29_e1487_d_n14, eq29_e1487_d_n15, eq29_e1487_d_n16];let eq29_branch_derivatives: [f64; 14] = [eq29_e1487_d_b0, eq29_e1487_d_b1, eq29_e1487_d_b2, eq29_e1487_d_b3, eq29_e1487_d_b4, eq29_e1487_d_b5, eq29_e1487_d_b6, eq29_e1487_d_b7, eq29_e1487_d_b8, eq29_e1487_d_b9, eq29_e1487_d_b10, eq29_e1487_d_b11, eq29_e1487_d_b12, eq29_e1487_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e1495, eq30_e1495_d_n0, eq30_e1495_d_n1, eq30_e1495_d_n2, eq30_e1495_d_n3, eq30_e1495_d_n4, eq30_e1495_d_n5, eq30_e1495_d_n6, eq30_e1495_d_n7, eq30_e1495_d_n8, eq30_e1495_d_n9, eq30_e1495_d_n10, eq30_e1495_d_n11, eq30_e1495_d_n12, eq30_e1495_d_n13, eq30_e1495_d_n14, eq30_e1495_d_n15, eq30_e1495_d_n16, eq30_e1495_d_b0, eq30_e1495_d_b1, eq30_e1495_d_b2, eq30_e1495_d_b3, eq30_e1495_d_b4, eq30_e1495_d_b5, eq30_e1495_d_b6, eq30_e1495_d_b7, eq30_e1495_d_b8, eq30_e1495_d_b9, eq30_e1495_d_b10, eq30_e1495_d_b11, eq30_e1495_d_b12, eq30_e1495_d_b13,) = {
    if s.b[1611] {
        let eq30_e1491: f64 = (p[28] * s.v[187]);let eq30_e1493: f64 = (eq30_e1491 * s.v[780]);let eq30_e1493_d_n0: f64 = (((p[28] * s.dn[187][0]) * s.v[780]) + (eq30_e1491 * s.dn[780][0]));let eq30_e1493_d_n1: f64 = (((p[28] * s.dn[187][1]) * s.v[780]) + (eq30_e1491 * s.dn[780][1]));let eq30_e1493_d_n2: f64 = (((p[28] * s.dn[187][2]) * s.v[780]) + (eq30_e1491 * s.dn[780][2]));let eq30_e1493_d_n3: f64 = (((p[28] * s.dn[187][3]) * s.v[780]) + (eq30_e1491 * s.dn[780][3]));let eq30_e1493_d_n4: f64 = (((p[28] * s.dn[187][4]) * s.v[780]) + (eq30_e1491 * s.dn[780][4]));let eq30_e1493_d_n5: f64 = (((p[28] * s.dn[187][5]) * s.v[780]) + (eq30_e1491 * s.dn[780][5]));let eq30_e1493_d_n6: f64 = (((p[28] * s.dn[187][6]) * s.v[780]) + (eq30_e1491 * s.dn[780][6]));let eq30_e1493_d_n7: f64 = (((p[28] * s.dn[187][7]) * s.v[780]) + (eq30_e1491 * s.dn[780][7]));let eq30_e1493_d_n8: f64 = (((p[28] * s.dn[187][8]) * s.v[780]) + (eq30_e1491 * s.dn[780][8]));let eq30_e1493_d_n9: f64 = (((p[28] * s.dn[187][9]) * s.v[780]) + (eq30_e1491 * s.dn[780][9]));let eq30_e1493_d_n10: f64 = (((p[28] * s.dn[187][10]) * s.v[780]) + (eq30_e1491 * s.dn[780][10]));let eq30_e1493_d_n11: f64 = (((p[28] * s.dn[187][11]) * s.v[780]) + (eq30_e1491 * s.dn[780][11]));let eq30_e1493_d_n12: f64 = (((p[28] * s.dn[187][12]) * s.v[780]) + (eq30_e1491 * s.dn[780][12]));let eq30_e1493_d_n13: f64 = (((p[28] * s.dn[187][13]) * s.v[780]) + (eq30_e1491 * s.dn[780][13]));let eq30_e1493_d_n14: f64 = (((p[28] * s.dn[187][14]) * s.v[780]) + (eq30_e1491 * s.dn[780][14]));let eq30_e1493_d_n15: f64 = (((p[28] * s.dn[187][15]) * s.v[780]) + (eq30_e1491 * s.dn[780][15]));let eq30_e1493_d_n16: f64 = (((p[28] * s.dn[187][16]) * s.v[780]) + (eq30_e1491 * s.dn[780][16]));let eq30_e1493_d_b0: f64 = (((p[28] * s.db[187][0]) * s.v[780]) + (eq30_e1491 * s.db[780][0]));let eq30_e1493_d_b1: f64 = (((p[28] * s.db[187][1]) * s.v[780]) + (eq30_e1491 * s.db[780][1]));let eq30_e1493_d_b2: f64 = (((p[28] * s.db[187][2]) * s.v[780]) + (eq30_e1491 * s.db[780][2]));let eq30_e1493_d_b3: f64 = (((p[28] * s.db[187][3]) * s.v[780]) + (eq30_e1491 * s.db[780][3]));let eq30_e1493_d_b4: f64 = (((p[28] * s.db[187][4]) * s.v[780]) + (eq30_e1491 * s.db[780][4]));let eq30_e1493_d_b5: f64 = (((p[28] * s.db[187][5]) * s.v[780]) + (eq30_e1491 * s.db[780][5]));let eq30_e1493_d_b6: f64 = (((p[28] * s.db[187][6]) * s.v[780]) + (eq30_e1491 * s.db[780][6]));let eq30_e1493_d_b7: f64 = (((p[28] * s.db[187][7]) * s.v[780]) + (eq30_e1491 * s.db[780][7]));let eq30_e1493_d_b8: f64 = (((p[28] * s.db[187][8]) * s.v[780]) + (eq30_e1491 * s.db[780][8]));let eq30_e1493_d_b9: f64 = (((p[28] * s.db[187][9]) * s.v[780]) + (eq30_e1491 * s.db[780][9]));let eq30_e1493_d_b10: f64 = (((p[28] * s.db[187][10]) * s.v[780]) + (eq30_e1491 * s.db[780][10]));let eq30_e1493_d_b11: f64 = (((p[28] * s.db[187][11]) * s.v[780]) + (eq30_e1491 * s.db[780][11]));let eq30_e1493_d_b12: f64 = (((p[28] * s.db[187][12]) * s.v[780]) + (eq30_e1491 * s.db[780][12]));let eq30_e1493_d_b13: f64 = (((p[28] * s.db[187][13]) * s.v[780]) + (eq30_e1491 * s.db[780][13]));
        (eq30_e1493, eq30_e1493_d_n0, eq30_e1493_d_n1, eq30_e1493_d_n2, eq30_e1493_d_n3, eq30_e1493_d_n4, eq30_e1493_d_n5, eq30_e1493_d_n6, eq30_e1493_d_n7, eq30_e1493_d_n8, eq30_e1493_d_n9, eq30_e1493_d_n10, eq30_e1493_d_n11, eq30_e1493_d_n12, eq30_e1493_d_n13, eq30_e1493_d_n14, eq30_e1493_d_n15, eq30_e1493_d_n16, eq30_e1493_d_b0, eq30_e1493_d_b1, eq30_e1493_d_b2, eq30_e1493_d_b3, eq30_e1493_d_b4, eq30_e1493_d_b5, eq30_e1493_d_b6, eq30_e1493_d_b7, eq30_e1493_d_b8, eq30_e1493_d_b9, eq30_e1493_d_b10, eq30_e1493_d_b11, eq30_e1493_d_b12, eq30_e1493_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1495;let eq30_node_derivatives: [f64; 17] = [eq30_e1495_d_n0, eq30_e1495_d_n1, eq30_e1495_d_n2, eq30_e1495_d_n3, eq30_e1495_d_n4, eq30_e1495_d_n5, eq30_e1495_d_n6, eq30_e1495_d_n7, eq30_e1495_d_n8, eq30_e1495_d_n9, eq30_e1495_d_n10, eq30_e1495_d_n11, eq30_e1495_d_n12, eq30_e1495_d_n13, eq30_e1495_d_n14, eq30_e1495_d_n15, eq30_e1495_d_n16];let eq30_branch_derivatives: [f64; 14] = [eq30_e1495_d_b0, eq30_e1495_d_b1, eq30_e1495_d_b2, eq30_e1495_d_b3, eq30_e1495_d_b4, eq30_e1495_d_b5, eq30_e1495_d_b6, eq30_e1495_d_b7, eq30_e1495_d_b8, eq30_e1495_d_b9, eq30_e1495_d_b10, eq30_e1495_d_b11, eq30_e1495_d_b12, eq30_e1495_d_b13];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e1499, eq31_e1499_d_n0, eq31_e1499_d_n1, eq31_e1499_d_n2, eq31_e1499_d_n3, eq31_e1499_d_n4, eq31_e1499_d_n5, eq31_e1499_d_n6, eq31_e1499_d_n7, eq31_e1499_d_n8, eq31_e1499_d_n9, eq31_e1499_d_n10, eq31_e1499_d_n11, eq31_e1499_d_n12, eq31_e1499_d_n13, eq31_e1499_d_n14, eq31_e1499_d_n15, eq31_e1499_d_n16, eq31_e1499_d_b0, eq31_e1499_d_b1, eq31_e1499_d_b2, eq31_e1499_d_b3, eq31_e1499_d_b4, eq31_e1499_d_b5, eq31_e1499_d_b6, eq31_e1499_d_b7, eq31_e1499_d_b8, eq31_e1499_d_b9, eq31_e1499_d_b10, eq31_e1499_d_b11, eq31_e1499_d_b12, eq31_e1499_d_b13,) = {
    if s.b[1611] {
        (s.v[826], s.dn[826][0], s.dn[826][1], s.dn[826][2], s.dn[826][3], s.dn[826][4], s.dn[826][5], s.dn[826][6], s.dn[826][7], s.dn[826][8], s.dn[826][9], s.dn[826][10], s.dn[826][11], s.dn[826][12], s.dn[826][13], s.dn[826][14], s.dn[826][15], s.dn[826][16], s.db[826][0], s.db[826][1], s.db[826][2], s.db[826][3], s.db[826][4], s.db[826][5], s.db[826][6], s.db[826][7], s.db[826][8], s.db[826][9], s.db[826][10], s.db[826][11], s.db[826][12], s.db[826][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e1499;let eq31_node_derivatives: [f64; 17] = [eq31_e1499_d_n0, eq31_e1499_d_n1, eq31_e1499_d_n2, eq31_e1499_d_n3, eq31_e1499_d_n4, eq31_e1499_d_n5, eq31_e1499_d_n6, eq31_e1499_d_n7, eq31_e1499_d_n8, eq31_e1499_d_n9, eq31_e1499_d_n10, eq31_e1499_d_n11, eq31_e1499_d_n12, eq31_e1499_d_n13, eq31_e1499_d_n14, eq31_e1499_d_n15, eq31_e1499_d_n16];let eq31_branch_derivatives: [f64; 14] = [eq31_e1499_d_b0, eq31_e1499_d_b1, eq31_e1499_d_b2, eq31_e1499_d_b3, eq31_e1499_d_b4, eq31_e1499_d_b5, eq31_e1499_d_b6, eq31_e1499_d_b7, eq31_e1499_d_b8, eq31_e1499_d_b9, eq31_e1499_d_b10, eq31_e1499_d_b11, eq31_e1499_d_b12, eq31_e1499_d_b13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1504, eq32_e1504_d_n0, eq32_e1504_d_n1, eq32_e1504_d_n2, eq32_e1504_d_n3, eq32_e1504_d_n4, eq32_e1504_d_n5, eq32_e1504_d_n6, eq32_e1504_d_n7, eq32_e1504_d_n8, eq32_e1504_d_n9, eq32_e1504_d_n10, eq32_e1504_d_n11, eq32_e1504_d_n12, eq32_e1504_d_n13, eq32_e1504_d_n14, eq32_e1504_d_n15, eq32_e1504_d_n16, eq32_e1504_d_b0, eq32_e1504_d_b1, eq32_e1504_d_b2, eq32_e1504_d_b3, eq32_e1504_d_b4, eq32_e1504_d_b5, eq32_e1504_d_b6, eq32_e1504_d_b7, eq32_e1504_d_b8, eq32_e1504_d_b9, eq32_e1504_d_b10, eq32_e1504_d_b11, eq32_e1504_d_b12, eq32_e1504_d_b13,) = {
    if (!s.b[1611]) {
        (s.v[825], s.dn[825][0], s.dn[825][1], s.dn[825][2], s.dn[825][3], s.dn[825][4], s.dn[825][5], s.dn[825][6], s.dn[825][7], s.dn[825][8], s.dn[825][9], s.dn[825][10], s.dn[825][11], s.dn[825][12], s.dn[825][13], s.dn[825][14], s.dn[825][15], s.dn[825][16], s.db[825][0], s.db[825][1], s.db[825][2], s.db[825][3], s.db[825][4], s.db[825][5], s.db[825][6], s.db[825][7], s.db[825][8], s.db[825][9], s.db[825][10], s.db[825][11], s.db[825][12], s.db[825][13],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e1504;let eq32_node_derivatives: [f64; 17] = [eq32_e1504_d_n0, eq32_e1504_d_n1, eq32_e1504_d_n2, eq32_e1504_d_n3, eq32_e1504_d_n4, eq32_e1504_d_n5, eq32_e1504_d_n6, eq32_e1504_d_n7, eq32_e1504_d_n8, eq32_e1504_d_n9, eq32_e1504_d_n10, eq32_e1504_d_n11, eq32_e1504_d_n12, eq32_e1504_d_n13, eq32_e1504_d_n14, eq32_e1504_d_n15, eq32_e1504_d_n16];let eq32_branch_derivatives: [f64; 14] = [eq32_e1504_d_b0, eq32_e1504_d_b1, eq32_e1504_d_b2, eq32_e1504_d_b3, eq32_e1504_d_b4, eq32_e1504_d_b5, eq32_e1504_d_b6, eq32_e1504_d_b7, eq32_e1504_d_b8, eq32_e1504_d_b9, eq32_e1504_d_b10, eq32_e1504_d_b11, eq32_e1504_d_b12, eq32_e1504_d_b13];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv6 = ctx.node_voltage(nodes[6]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq33_e1511, eq33_e1511_d_n0, eq33_e1511_d_n1, eq33_e1511_d_n2, eq33_e1511_d_n3, eq33_e1511_d_n4, eq33_e1511_d_n5, eq33_e1511_d_n6, eq33_e1511_d_n7, eq33_e1511_d_n8, eq33_e1511_d_n9, eq33_e1511_d_n10, eq33_e1511_d_n11, eq33_e1511_d_n12, eq33_e1511_d_n13, eq33_e1511_d_n14, eq33_e1511_d_n15, eq33_e1511_d_n16, eq33_e1511_d_b0, eq33_e1511_d_b1, eq33_e1511_d_b2, eq33_e1511_d_b3, eq33_e1511_d_b4, eq33_e1511_d_b5, eq33_e1511_d_b6, eq33_e1511_d_b7, eq33_e1511_d_b8, eq33_e1511_d_b9, eq33_e1511_d_b10, eq33_e1511_d_b11, eq33_e1511_d_b12, eq33_e1511_d_b13,) = {
    if (!s.b[1611]) {
        let eq33_e1509: f64 = (s.v[824] + s.v[826]);let eq33_e1509_d_n0: f64 = (s.dn[824][0] + s.dn[826][0]);let eq33_e1509_d_n1: f64 = (s.dn[824][1] + s.dn[826][1]);let eq33_e1509_d_n2: f64 = (s.dn[824][2] + s.dn[826][2]);let eq33_e1509_d_n3: f64 = (s.dn[824][3] + s.dn[826][3]);let eq33_e1509_d_n4: f64 = (s.dn[824][4] + s.dn[826][4]);let eq33_e1509_d_n5: f64 = (s.dn[824][5] + s.dn[826][5]);let eq33_e1509_d_n6: f64 = (s.dn[824][6] + s.dn[826][6]);let eq33_e1509_d_n7: f64 = (s.dn[824][7] + s.dn[826][7]);let eq33_e1509_d_n8: f64 = (s.dn[824][8] + s.dn[826][8]);let eq33_e1509_d_n9: f64 = (s.dn[824][9] + s.dn[826][9]);let eq33_e1509_d_n10: f64 = (s.dn[824][10] + s.dn[826][10]);let eq33_e1509_d_n11: f64 = (s.dn[824][11] + s.dn[826][11]);let eq33_e1509_d_n12: f64 = (s.dn[824][12] + s.dn[826][12]);let eq33_e1509_d_n13: f64 = (s.dn[824][13] + s.dn[826][13]);let eq33_e1509_d_n14: f64 = (s.dn[824][14] + s.dn[826][14]);let eq33_e1509_d_n15: f64 = (s.dn[824][15] + s.dn[826][15]);let eq33_e1509_d_n16: f64 = (s.dn[824][16] + s.dn[826][16]);let eq33_e1509_d_b0: f64 = (s.db[824][0] + s.db[826][0]);let eq33_e1509_d_b1: f64 = (s.db[824][1] + s.db[826][1]);let eq33_e1509_d_b2: f64 = (s.db[824][2] + s.db[826][2]);let eq33_e1509_d_b3: f64 = (s.db[824][3] + s.db[826][3]);let eq33_e1509_d_b4: f64 = (s.db[824][4] + s.db[826][4]);let eq33_e1509_d_b5: f64 = (s.db[824][5] + s.db[826][5]);let eq33_e1509_d_b6: f64 = (s.db[824][6] + s.db[826][6]);let eq33_e1509_d_b7: f64 = (s.db[824][7] + s.db[826][7]);let eq33_e1509_d_b8: f64 = (s.db[824][8] + s.db[826][8]);let eq33_e1509_d_b9: f64 = (s.db[824][9] + s.db[826][9]);let eq33_e1509_d_b10: f64 = (s.db[824][10] + s.db[826][10]);let eq33_e1509_d_b11: f64 = (s.db[824][11] + s.db[826][11]);let eq33_e1509_d_b12: f64 = (s.db[824][12] + s.db[826][12]);let eq33_e1509_d_b13: f64 = (s.db[824][13] + s.db[826][13]);
        (eq33_e1509, eq33_e1509_d_n0, eq33_e1509_d_n1, eq33_e1509_d_n2, eq33_e1509_d_n3, eq33_e1509_d_n4, eq33_e1509_d_n5, eq33_e1509_d_n6, eq33_e1509_d_n7, eq33_e1509_d_n8, eq33_e1509_d_n9, eq33_e1509_d_n10, eq33_e1509_d_n11, eq33_e1509_d_n12, eq33_e1509_d_n13, eq33_e1509_d_n14, eq33_e1509_d_n15, eq33_e1509_d_n16, eq33_e1509_d_b0, eq33_e1509_d_b1, eq33_e1509_d_b2, eq33_e1509_d_b3, eq33_e1509_d_b4, eq33_e1509_d_b5, eq33_e1509_d_b6, eq33_e1509_d_b7, eq33_e1509_d_b8, eq33_e1509_d_b9, eq33_e1509_d_b10, eq33_e1509_d_b11, eq33_e1509_d_b12, eq33_e1509_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1511;let eq33_node_derivatives: [f64; 17] = [eq33_e1511_d_n0, eq33_e1511_d_n1, eq33_e1511_d_n2, eq33_e1511_d_n3, eq33_e1511_d_n4, eq33_e1511_d_n5, eq33_e1511_d_n6, eq33_e1511_d_n7, eq33_e1511_d_n8, eq33_e1511_d_n9, eq33_e1511_d_n10, eq33_e1511_d_n11, eq33_e1511_d_n12, eq33_e1511_d_n13, eq33_e1511_d_n14, eq33_e1511_d_n15, eq33_e1511_d_n16];let eq33_branch_derivatives: [f64; 14] = [eq33_e1511_d_b0, eq33_e1511_d_b1, eq33_e1511_d_b2, eq33_e1511_d_b3, eq33_e1511_d_b4, eq33_e1511_d_b5, eq33_e1511_d_b6, eq33_e1511_d_b7, eq33_e1511_d_b8, eq33_e1511_d_b9, eq33_e1511_d_b10, eq33_e1511_d_b11, eq33_e1511_d_b12, eq33_e1511_d_b13];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e1519, eq34_e1519_d_n0, eq34_e1519_d_n1, eq34_e1519_d_n2, eq34_e1519_d_n3, eq34_e1519_d_n4, eq34_e1519_d_n5, eq34_e1519_d_n6, eq34_e1519_d_n7, eq34_e1519_d_n8, eq34_e1519_d_n9, eq34_e1519_d_n10, eq34_e1519_d_n11, eq34_e1519_d_n12, eq34_e1519_d_n13, eq34_e1519_d_n14, eq34_e1519_d_n15, eq34_e1519_d_n16, eq34_e1519_d_b0, eq34_e1519_d_b1, eq34_e1519_d_b2, eq34_e1519_d_b3, eq34_e1519_d_b4, eq34_e1519_d_b5, eq34_e1519_d_b6, eq34_e1519_d_b7, eq34_e1519_d_b8, eq34_e1519_d_b9, eq34_e1519_d_b10, eq34_e1519_d_b11, eq34_e1519_d_b12, eq34_e1519_d_b13,) = {
    if s.b[1612] {
        let eq34_e1515: f64 = (p[28] * (nv0 - nv6));let eq34_e1517: f64 = (eq34_e1515 * s.v[372]);let eq34_e1517_d_n0: f64 = ((p[28] * s.v[372]) + (eq34_e1515 * s.dn[372][0]));let eq34_e1517_d_n6: f64 = (((-p[28]) * s.v[372]) + (eq34_e1515 * s.dn[372][6]));
        (eq34_e1517, eq34_e1517_d_n0, (eq34_e1515 * s.dn[372][1]), (eq34_e1515 * s.dn[372][2]), (eq34_e1515 * s.dn[372][3]), (eq34_e1515 * s.dn[372][4]), (eq34_e1515 * s.dn[372][5]), eq34_e1517_d_n6, (eq34_e1515 * s.dn[372][7]), (eq34_e1515 * s.dn[372][8]), (eq34_e1515 * s.dn[372][9]), (eq34_e1515 * s.dn[372][10]), (eq34_e1515 * s.dn[372][11]), (eq34_e1515 * s.dn[372][12]), (eq34_e1515 * s.dn[372][13]), (eq34_e1515 * s.dn[372][14]), (eq34_e1515 * s.dn[372][15]), (eq34_e1515 * s.dn[372][16]), (eq34_e1515 * s.db[372][0]), (eq34_e1515 * s.db[372][1]), (eq34_e1515 * s.db[372][2]), (eq34_e1515 * s.db[372][3]), (eq34_e1515 * s.db[372][4]), (eq34_e1515 * s.db[372][5]), (eq34_e1515 * s.db[372][6]), (eq34_e1515 * s.db[372][7]), (eq34_e1515 * s.db[372][8]), (eq34_e1515 * s.db[372][9]), (eq34_e1515 * s.db[372][10]), (eq34_e1515 * s.db[372][11]), (eq34_e1515 * s.db[372][12]), (eq34_e1515 * s.db[372][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e1519;let eq34_node_derivatives: [f64; 17] = [eq34_e1519_d_n0, eq34_e1519_d_n1, eq34_e1519_d_n2, eq34_e1519_d_n3, eq34_e1519_d_n4, eq34_e1519_d_n5, eq34_e1519_d_n6, eq34_e1519_d_n7, eq34_e1519_d_n8, eq34_e1519_d_n9, eq34_e1519_d_n10, eq34_e1519_d_n11, eq34_e1519_d_n12, eq34_e1519_d_n13, eq34_e1519_d_n14, eq34_e1519_d_n15, eq34_e1519_d_n16];let eq34_branch_derivatives: [f64; 14] = [eq34_e1519_d_b0, eq34_e1519_d_b1, eq34_e1519_d_b2, eq34_e1519_d_b3, eq34_e1519_d_b4, eq34_e1519_d_b5, eq34_e1519_d_b6, eq34_e1519_d_b7, eq34_e1519_d_b8, eq34_e1519_d_b9, eq34_e1519_d_b10, eq34_e1519_d_b11, eq34_e1519_d_b12, eq34_e1519_d_b13];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1539, eq36_e1539_d_n0, eq36_e1539_d_n1, eq36_e1539_d_n2, eq36_e1539_d_n3, eq36_e1539_d_n4, eq36_e1539_d_n5, eq36_e1539_d_n6, eq36_e1539_d_n7, eq36_e1539_d_n8, eq36_e1539_d_n9, eq36_e1539_d_n10, eq36_e1539_d_n11, eq36_e1539_d_n12, eq36_e1539_d_n13, eq36_e1539_d_n14, eq36_e1539_d_n15, eq36_e1539_d_n16, eq36_e1539_d_b0, eq36_e1539_d_b1, eq36_e1539_d_b2, eq36_e1539_d_b3, eq36_e1539_d_b4, eq36_e1539_d_b5, eq36_e1539_d_b6, eq36_e1539_d_b7, eq36_e1539_d_b8, eq36_e1539_d_b9, eq36_e1539_d_b10, eq36_e1539_d_b11, eq36_e1539_d_b12, eq36_e1539_d_b13,) = {
    if (s.b[1612] && s.b[1613]) {
        let eq36_e1535: f64 = (p[28] * (nv6 - nv5));let eq36_e1537: f64 = (eq36_e1535 * s.v[374]);let eq36_e1537_d_n5: f64 = (((-p[28]) * s.v[374]) + (eq36_e1535 * s.dn[374][5]));let eq36_e1537_d_n6: f64 = ((p[28] * s.v[374]) + (eq36_e1535 * s.dn[374][6]));
        (eq36_e1537, (eq36_e1535 * s.dn[374][0]), (eq36_e1535 * s.dn[374][1]), (eq36_e1535 * s.dn[374][2]), (eq36_e1535 * s.dn[374][3]), (eq36_e1535 * s.dn[374][4]), eq36_e1537_d_n5, eq36_e1537_d_n6, (eq36_e1535 * s.dn[374][7]), (eq36_e1535 * s.dn[374][8]), (eq36_e1535 * s.dn[374][9]), (eq36_e1535 * s.dn[374][10]), (eq36_e1535 * s.dn[374][11]), (eq36_e1535 * s.dn[374][12]), (eq36_e1535 * s.dn[374][13]), (eq36_e1535 * s.dn[374][14]), (eq36_e1535 * s.dn[374][15]), (eq36_e1535 * s.dn[374][16]), (eq36_e1535 * s.db[374][0]), (eq36_e1535 * s.db[374][1]), (eq36_e1535 * s.db[374][2]), (eq36_e1535 * s.db[374][3]), (eq36_e1535 * s.db[374][4]), (eq36_e1535 * s.db[374][5]), (eq36_e1535 * s.db[374][6]), (eq36_e1535 * s.db[374][7]), (eq36_e1535 * s.db[374][8]), (eq36_e1535 * s.db[374][9]), (eq36_e1535 * s.db[374][10]), (eq36_e1535 * s.db[374][11]), (eq36_e1535 * s.db[374][12]), (eq36_e1535 * s.db[374][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1539;let eq36_node_derivatives: [f64; 17] = [eq36_e1539_d_n0, eq36_e1539_d_n1, eq36_e1539_d_n2, eq36_e1539_d_n3, eq36_e1539_d_n4, eq36_e1539_d_n5, eq36_e1539_d_n6, eq36_e1539_d_n7, eq36_e1539_d_n8, eq36_e1539_d_n9, eq36_e1539_d_n10, eq36_e1539_d_n11, eq36_e1539_d_n12, eq36_e1539_d_n13, eq36_e1539_d_n14, eq36_e1539_d_n15, eq36_e1539_d_n16];let eq36_branch_derivatives: [f64; 14] = [eq36_e1539_d_b0, eq36_e1539_d_b1, eq36_e1539_d_b2, eq36_e1539_d_b3, eq36_e1539_d_b4, eq36_e1539_d_b5, eq36_e1539_d_b6, eq36_e1539_d_b7, eq36_e1539_d_b8, eq36_e1539_d_b9, eq36_e1539_d_b10, eq36_e1539_d_b11, eq36_e1539_d_b12, eq36_e1539_d_b13];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1577,) = {
    if (s.b[1612] && (!s.b[1613])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e1577;
        stamper.stamp_potential_const_local(
            1,
            eq39_value,
        );
        let (eq40_e1582,) = {
    if (!s.b[1612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e1582;
        stamper.stamp_potential_const_local(
            2,
            eq40_value,
        );
        let (eq41_e1587,) = {
    if (!s.b[1612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e1587;
        stamper.stamp_potential_const_local(
            3,
            eq41_value,
        );
        let (eq42_e1595, eq42_e1595_d_n0, eq42_e1595_d_n1, eq42_e1595_d_n2, eq42_e1595_d_n3, eq42_e1595_d_n4, eq42_e1595_d_n5, eq42_e1595_d_n6, eq42_e1595_d_n7, eq42_e1595_d_n8, eq42_e1595_d_n9, eq42_e1595_d_n10, eq42_e1595_d_n11, eq42_e1595_d_n12, eq42_e1595_d_n13, eq42_e1595_d_n14, eq42_e1595_d_n15, eq42_e1595_d_n16, eq42_e1595_d_b0, eq42_e1595_d_b1, eq42_e1595_d_b2, eq42_e1595_d_b3, eq42_e1595_d_b4, eq42_e1595_d_b5, eq42_e1595_d_b6, eq42_e1595_d_b7, eq42_e1595_d_b8, eq42_e1595_d_b9, eq42_e1595_d_b10, eq42_e1595_d_b11, eq42_e1595_d_b12, eq42_e1595_d_b13,) = {
    if s.b[1614] {
        let eq42_e1591: f64 = (p[28] * (nv2 - nv8));let eq42_e1593: f64 = (eq42_e1591 * s.v[371]);let eq42_e1593_d_n2: f64 = ((p[28] * s.v[371]) + (eq42_e1591 * s.dn[371][2]));let eq42_e1593_d_n8: f64 = (((-p[28]) * s.v[371]) + (eq42_e1591 * s.dn[371][8]));
        (eq42_e1593, (eq42_e1591 * s.dn[371][0]), (eq42_e1591 * s.dn[371][1]), eq42_e1593_d_n2, (eq42_e1591 * s.dn[371][3]), (eq42_e1591 * s.dn[371][4]), (eq42_e1591 * s.dn[371][5]), (eq42_e1591 * s.dn[371][6]), (eq42_e1591 * s.dn[371][7]), eq42_e1593_d_n8, (eq42_e1591 * s.dn[371][9]), (eq42_e1591 * s.dn[371][10]), (eq42_e1591 * s.dn[371][11]), (eq42_e1591 * s.dn[371][12]), (eq42_e1591 * s.dn[371][13]), (eq42_e1591 * s.dn[371][14]), (eq42_e1591 * s.dn[371][15]), (eq42_e1591 * s.dn[371][16]), (eq42_e1591 * s.db[371][0]), (eq42_e1591 * s.db[371][1]), (eq42_e1591 * s.db[371][2]), (eq42_e1591 * s.db[371][3]), (eq42_e1591 * s.db[371][4]), (eq42_e1591 * s.db[371][5]), (eq42_e1591 * s.db[371][6]), (eq42_e1591 * s.db[371][7]), (eq42_e1591 * s.db[371][8]), (eq42_e1591 * s.db[371][9]), (eq42_e1591 * s.db[371][10]), (eq42_e1591 * s.db[371][11]), (eq42_e1591 * s.db[371][12]), (eq42_e1591 * s.db[371][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e1595;let eq42_node_derivatives: [f64; 17] = [eq42_e1595_d_n0, eq42_e1595_d_n1, eq42_e1595_d_n2, eq42_e1595_d_n3, eq42_e1595_d_n4, eq42_e1595_d_n5, eq42_e1595_d_n6, eq42_e1595_d_n7, eq42_e1595_d_n8, eq42_e1595_d_n9, eq42_e1595_d_n10, eq42_e1595_d_n11, eq42_e1595_d_n12, eq42_e1595_d_n13, eq42_e1595_d_n14, eq42_e1595_d_n15, eq42_e1595_d_n16];let eq42_branch_derivatives: [f64; 14] = [eq42_e1595_d_b0, eq42_e1595_d_b1, eq42_e1595_d_b2, eq42_e1595_d_b3, eq42_e1595_d_b4, eq42_e1595_d_b5, eq42_e1595_d_b6, eq42_e1595_d_b7, eq42_e1595_d_b8, eq42_e1595_d_b9, eq42_e1595_d_b10, eq42_e1595_d_b11, eq42_e1595_d_b12, eq42_e1595_d_b13];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq44_e1615, eq44_e1615_d_n0, eq44_e1615_d_n1, eq44_e1615_d_n2, eq44_e1615_d_n3, eq44_e1615_d_n4, eq44_e1615_d_n5, eq44_e1615_d_n6, eq44_e1615_d_n7, eq44_e1615_d_n8, eq44_e1615_d_n9, eq44_e1615_d_n10, eq44_e1615_d_n11, eq44_e1615_d_n12, eq44_e1615_d_n13, eq44_e1615_d_n14, eq44_e1615_d_n15, eq44_e1615_d_n16, eq44_e1615_d_b0, eq44_e1615_d_b1, eq44_e1615_d_b2, eq44_e1615_d_b3, eq44_e1615_d_b4, eq44_e1615_d_b5, eq44_e1615_d_b6, eq44_e1615_d_b7, eq44_e1615_d_b8, eq44_e1615_d_b9, eq44_e1615_d_b10, eq44_e1615_d_b11, eq44_e1615_d_b12, eq44_e1615_d_b13,) = {
    if (s.b[1614] && s.b[1615]) {
        let eq44_e1611: f64 = (p[28] * (nv8 - nv7));let eq44_e1613: f64 = (eq44_e1611 * s.v[373]);let eq44_e1613_d_n7: f64 = (((-p[28]) * s.v[373]) + (eq44_e1611 * s.dn[373][7]));let eq44_e1613_d_n8: f64 = ((p[28] * s.v[373]) + (eq44_e1611 * s.dn[373][8]));
        (eq44_e1613, (eq44_e1611 * s.dn[373][0]), (eq44_e1611 * s.dn[373][1]), (eq44_e1611 * s.dn[373][2]), (eq44_e1611 * s.dn[373][3]), (eq44_e1611 * s.dn[373][4]), (eq44_e1611 * s.dn[373][5]), (eq44_e1611 * s.dn[373][6]), eq44_e1613_d_n7, eq44_e1613_d_n8, (eq44_e1611 * s.dn[373][9]), (eq44_e1611 * s.dn[373][10]), (eq44_e1611 * s.dn[373][11]), (eq44_e1611 * s.dn[373][12]), (eq44_e1611 * s.dn[373][13]), (eq44_e1611 * s.dn[373][14]), (eq44_e1611 * s.dn[373][15]), (eq44_e1611 * s.dn[373][16]), (eq44_e1611 * s.db[373][0]), (eq44_e1611 * s.db[373][1]), (eq44_e1611 * s.db[373][2]), (eq44_e1611 * s.db[373][3]), (eq44_e1611 * s.db[373][4]), (eq44_e1611 * s.db[373][5]), (eq44_e1611 * s.db[373][6]), (eq44_e1611 * s.db[373][7]), (eq44_e1611 * s.db[373][8]), (eq44_e1611 * s.db[373][9]), (eq44_e1611 * s.db[373][10]), (eq44_e1611 * s.db[373][11]), (eq44_e1611 * s.db[373][12]), (eq44_e1611 * s.db[373][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e1615;let eq44_node_derivatives: [f64; 17] = [eq44_e1615_d_n0, eq44_e1615_d_n1, eq44_e1615_d_n2, eq44_e1615_d_n3, eq44_e1615_d_n4, eq44_e1615_d_n5, eq44_e1615_d_n6, eq44_e1615_d_n7, eq44_e1615_d_n8, eq44_e1615_d_n9, eq44_e1615_d_n10, eq44_e1615_d_n11, eq44_e1615_d_n12, eq44_e1615_d_n13, eq44_e1615_d_n14, eq44_e1615_d_n15, eq44_e1615_d_n16];let eq44_branch_derivatives: [f64; 14] = [eq44_e1615_d_b0, eq44_e1615_d_b1, eq44_e1615_d_b2, eq44_e1615_d_b3, eq44_e1615_d_b4, eq44_e1615_d_b5, eq44_e1615_d_b6, eq44_e1615_d_b7, eq44_e1615_d_b8, eq44_e1615_d_b9, eq44_e1615_d_b10, eq44_e1615_d_b11, eq44_e1615_d_b12, eq44_e1615_d_b13];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e1653,) = {
    if (s.b[1614] && (!s.b[1615])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1653;
        stamper.stamp_potential_const_local(
            4,
            eq47_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq48_e1658,) = {
    if (!s.b[1614]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1658;
        stamper.stamp_potential_const_local(
            5,
            eq48_value,
        );
        let (eq49_e1663,) = {
    if (!s.b[1614]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1663;
        stamper.stamp_potential_const_local(
            6,
            eq49_value,
        );
        let (eq50_e1667,) = {
    if s.b[1616] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e1667;
        stamper.stamp_potential_const_local(
            7,
            eq50_value,
        );
        let (eq51_e1676, eq51_e1676_d_n0, eq51_e1676_d_n1, eq51_e1676_d_n2, eq51_e1676_d_n3, eq51_e1676_d_n4, eq51_e1676_d_n5, eq51_e1676_d_n6, eq51_e1676_d_n7, eq51_e1676_d_n8, eq51_e1676_d_n9, eq51_e1676_d_n10, eq51_e1676_d_n11, eq51_e1676_d_n12, eq51_e1676_d_n13, eq51_e1676_d_n14, eq51_e1676_d_n15, eq51_e1676_d_n16, eq51_e1676_d_b0, eq51_e1676_d_b1, eq51_e1676_d_b2, eq51_e1676_d_b3, eq51_e1676_d_b4, eq51_e1676_d_b5, eq51_e1676_d_b6, eq51_e1676_d_b7, eq51_e1676_d_b8, eq51_e1676_d_b9, eq51_e1676_d_b10, eq51_e1676_d_b11, eq51_e1676_d_b12, eq51_e1676_d_b13,) = {
    if (!s.b[1616]) {
        let eq51_e1672: f64 = (p[28] * (nv1 - nv10));let eq51_e1674: f64 = (eq51_e1672 * s.v[1617]);let eq51_e1674_d_n1: f64 = ((p[28] * s.v[1617]) + (eq51_e1672 * s.dn[1617][1]));let eq51_e1674_d_n10: f64 = (((-p[28]) * s.v[1617]) + (eq51_e1672 * s.dn[1617][10]));
        (eq51_e1674, (eq51_e1672 * s.dn[1617][0]), eq51_e1674_d_n1, (eq51_e1672 * s.dn[1617][2]), (eq51_e1672 * s.dn[1617][3]), (eq51_e1672 * s.dn[1617][4]), (eq51_e1672 * s.dn[1617][5]), (eq51_e1672 * s.dn[1617][6]), (eq51_e1672 * s.dn[1617][7]), (eq51_e1672 * s.dn[1617][8]), (eq51_e1672 * s.dn[1617][9]), eq51_e1674_d_n10, (eq51_e1672 * s.dn[1617][11]), (eq51_e1672 * s.dn[1617][12]), (eq51_e1672 * s.dn[1617][13]), (eq51_e1672 * s.dn[1617][14]), (eq51_e1672 * s.dn[1617][15]), (eq51_e1672 * s.dn[1617][16]), (eq51_e1672 * s.db[1617][0]), (eq51_e1672 * s.db[1617][1]), (eq51_e1672 * s.db[1617][2]), (eq51_e1672 * s.db[1617][3]), (eq51_e1672 * s.db[1617][4]), (eq51_e1672 * s.db[1617][5]), (eq51_e1672 * s.db[1617][6]), (eq51_e1672 * s.db[1617][7]), (eq51_e1672 * s.db[1617][8]), (eq51_e1672 * s.db[1617][9]), (eq51_e1672 * s.db[1617][10]), (eq51_e1672 * s.db[1617][11]), (eq51_e1672 * s.db[1617][12]), (eq51_e1672 * s.db[1617][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1676;let eq51_node_derivatives: [f64; 17] = [eq51_e1676_d_n0, eq51_e1676_d_n1, eq51_e1676_d_n2, eq51_e1676_d_n3, eq51_e1676_d_n4, eq51_e1676_d_n5, eq51_e1676_d_n6, eq51_e1676_d_n7, eq51_e1676_d_n8, eq51_e1676_d_n9, eq51_e1676_d_n10, eq51_e1676_d_n11, eq51_e1676_d_n12, eq51_e1676_d_n13, eq51_e1676_d_n14, eq51_e1676_d_n15, eq51_e1676_d_n16];let eq51_branch_derivatives: [f64; 14] = [eq51_e1676_d_b0, eq51_e1676_d_b1, eq51_e1676_d_b2, eq51_e1676_d_b3, eq51_e1676_d_b4, eq51_e1676_d_b5, eq51_e1676_d_b6, eq51_e1676_d_b7, eq51_e1676_d_b8, eq51_e1676_d_b9, eq51_e1676_d_b10, eq51_e1676_d_b11, eq51_e1676_d_b12, eq51_e1676_d_b13];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1695, eq53_e1695_d_n0, eq53_e1695_d_n1, eq53_e1695_d_n2, eq53_e1695_d_n3, eq53_e1695_d_n4, eq53_e1695_d_n5, eq53_e1695_d_n6, eq53_e1695_d_n7, eq53_e1695_d_n8, eq53_e1695_d_n9, eq53_e1695_d_n10, eq53_e1695_d_n11, eq53_e1695_d_n12, eq53_e1695_d_n13, eq53_e1695_d_n14, eq53_e1695_d_n15, eq53_e1695_d_n16, eq53_e1695_d_b0, eq53_e1695_d_b1, eq53_e1695_d_b2, eq53_e1695_d_b3, eq53_e1695_d_b4, eq53_e1695_d_b5, eq53_e1695_d_b6, eq53_e1695_d_b7, eq53_e1695_d_b8, eq53_e1695_d_b9, eq53_e1695_d_b10, eq53_e1695_d_b11, eq53_e1695_d_b12, eq53_e1695_d_b13,) = {
    if s.b[1620] {
        let eq53_e1691: f64 = ((nv10 - nv9) * p[28]);let eq53_e1693: f64 = (eq53_e1691 * s.v[254]);let eq53_e1693_d_n9: f64 = (((-p[28]) * s.v[254]) + (eq53_e1691 * s.dn[254][9]));let eq53_e1693_d_n10: f64 = ((p[28] * s.v[254]) + (eq53_e1691 * s.dn[254][10]));
        (eq53_e1693, (eq53_e1691 * s.dn[254][0]), (eq53_e1691 * s.dn[254][1]), (eq53_e1691 * s.dn[254][2]), (eq53_e1691 * s.dn[254][3]), (eq53_e1691 * s.dn[254][4]), (eq53_e1691 * s.dn[254][5]), (eq53_e1691 * s.dn[254][6]), (eq53_e1691 * s.dn[254][7]), (eq53_e1691 * s.dn[254][8]), eq53_e1693_d_n9, eq53_e1693_d_n10, (eq53_e1691 * s.dn[254][11]), (eq53_e1691 * s.dn[254][12]), (eq53_e1691 * s.dn[254][13]), (eq53_e1691 * s.dn[254][14]), (eq53_e1691 * s.dn[254][15]), (eq53_e1691 * s.dn[254][16]), (eq53_e1691 * s.db[254][0]), (eq53_e1691 * s.db[254][1]), (eq53_e1691 * s.db[254][2]), (eq53_e1691 * s.db[254][3]), (eq53_e1691 * s.db[254][4]), (eq53_e1691 * s.db[254][5]), (eq53_e1691 * s.db[254][6]), (eq53_e1691 * s.db[254][7]), (eq53_e1691 * s.db[254][8]), (eq53_e1691 * s.db[254][9]), (eq53_e1691 * s.db[254][10]), (eq53_e1691 * s.db[254][11]), (eq53_e1691 * s.db[254][12]), (eq53_e1691 * s.db[254][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1695;let eq53_node_derivatives: [f64; 17] = [eq53_e1695_d_n0, eq53_e1695_d_n1, eq53_e1695_d_n2, eq53_e1695_d_n3, eq53_e1695_d_n4, eq53_e1695_d_n5, eq53_e1695_d_n6, eq53_e1695_d_n7, eq53_e1695_d_n8, eq53_e1695_d_n9, eq53_e1695_d_n10, eq53_e1695_d_n11, eq53_e1695_d_n12, eq53_e1695_d_n13, eq53_e1695_d_n14, eq53_e1695_d_n15, eq53_e1695_d_n16];let eq53_branch_derivatives: [f64; 14] = [eq53_e1695_d_b0, eq53_e1695_d_b1, eq53_e1695_d_b2, eq53_e1695_d_b3, eq53_e1695_d_b4, eq53_e1695_d_b5, eq53_e1695_d_b6, eq53_e1695_d_b7, eq53_e1695_d_b8, eq53_e1695_d_b9, eq53_e1695_d_b10, eq53_e1695_d_b11, eq53_e1695_d_b12, eq53_e1695_d_b13];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e1700,) = {
    if (!s.b[1620]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e1700;
        stamper.stamp_potential_const_local(
            8,
            eq54_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
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
        let (eq55_e1713, eq55_e1713_d_n0, eq55_e1713_d_n1, eq55_e1713_d_n2, eq55_e1713_d_n3, eq55_e1713_d_n4, eq55_e1713_d_n5, eq55_e1713_d_n6, eq55_e1713_d_n7, eq55_e1713_d_n8, eq55_e1713_d_n9, eq55_e1713_d_n10, eq55_e1713_d_n11, eq55_e1713_d_n12, eq55_e1713_d_n13, eq55_e1713_d_n14, eq55_e1713_d_n15, eq55_e1713_d_n16, eq55_e1713_d_b0, eq55_e1713_d_b1, eq55_e1713_d_b2, eq55_e1713_d_b3, eq55_e1713_d_b4, eq55_e1713_d_b5, eq55_e1713_d_b6, eq55_e1713_d_b7, eq55_e1713_d_b8, eq55_e1713_d_b9, eq55_e1713_d_b10, eq55_e1713_d_b11, eq55_e1713_d_b12, eq55_e1713_d_b13,) = {
    if s.b[1621] {
        let eq55_e1704: f64 = (s.v[390] * s.v[747]);let eq55_e1704_d_n0: f64 = ((s.dn[390][0] * s.v[747]) + (s.v[390] * s.dn[747][0]));let eq55_e1704_d_n1: f64 = ((s.dn[390][1] * s.v[747]) + (s.v[390] * s.dn[747][1]));let eq55_e1704_d_n2: f64 = ((s.dn[390][2] * s.v[747]) + (s.v[390] * s.dn[747][2]));let eq55_e1704_d_n3: f64 = ((s.dn[390][3] * s.v[747]) + (s.v[390] * s.dn[747][3]));let eq55_e1704_d_n4: f64 = ((s.dn[390][4] * s.v[747]) + (s.v[390] * s.dn[747][4]));let eq55_e1704_d_n5: f64 = ((s.dn[390][5] * s.v[747]) + (s.v[390] * s.dn[747][5]));let eq55_e1704_d_n6: f64 = ((s.dn[390][6] * s.v[747]) + (s.v[390] * s.dn[747][6]));let eq55_e1704_d_n7: f64 = ((s.dn[390][7] * s.v[747]) + (s.v[390] * s.dn[747][7]));let eq55_e1704_d_n8: f64 = ((s.dn[390][8] * s.v[747]) + (s.v[390] * s.dn[747][8]));let eq55_e1704_d_n9: f64 = ((s.dn[390][9] * s.v[747]) + (s.v[390] * s.dn[747][9]));let eq55_e1704_d_n10: f64 = ((s.dn[390][10] * s.v[747]) + (s.v[390] * s.dn[747][10]));let eq55_e1704_d_n11: f64 = ((s.dn[390][11] * s.v[747]) + (s.v[390] * s.dn[747][11]));let eq55_e1704_d_n12: f64 = ((s.dn[390][12] * s.v[747]) + (s.v[390] * s.dn[747][12]));let eq55_e1704_d_n13: f64 = ((s.dn[390][13] * s.v[747]) + (s.v[390] * s.dn[747][13]));let eq55_e1704_d_n14: f64 = ((s.dn[390][14] * s.v[747]) + (s.v[390] * s.dn[747][14]));let eq55_e1704_d_n15: f64 = ((s.dn[390][15] * s.v[747]) + (s.v[390] * s.dn[747][15]));let eq55_e1704_d_n16: f64 = ((s.dn[390][16] * s.v[747]) + (s.v[390] * s.dn[747][16]));let eq55_e1704_d_b0: f64 = ((s.db[390][0] * s.v[747]) + (s.v[390] * s.db[747][0]));let eq55_e1704_d_b1: f64 = ((s.db[390][1] * s.v[747]) + (s.v[390] * s.db[747][1]));let eq55_e1704_d_b2: f64 = ((s.db[390][2] * s.v[747]) + (s.v[390] * s.db[747][2]));let eq55_e1704_d_b3: f64 = ((s.db[390][3] * s.v[747]) + (s.v[390] * s.db[747][3]));let eq55_e1704_d_b4: f64 = ((s.db[390][4] * s.v[747]) + (s.v[390] * s.db[747][4]));let eq55_e1704_d_b5: f64 = ((s.db[390][5] * s.v[747]) + (s.v[390] * s.db[747][5]));let eq55_e1704_d_b6: f64 = ((s.db[390][6] * s.v[747]) + (s.v[390] * s.db[747][6]));let eq55_e1704_d_b7: f64 = ((s.db[390][7] * s.v[747]) + (s.v[390] * s.db[747][7]));let eq55_e1704_d_b8: f64 = ((s.db[390][8] * s.v[747]) + (s.v[390] * s.db[747][8]));let eq55_e1704_d_b9: f64 = ((s.db[390][9] * s.v[747]) + (s.v[390] * s.db[747][9]));let eq55_e1704_d_b10: f64 = ((s.db[390][10] * s.v[747]) + (s.v[390] * s.db[747][10]));let eq55_e1704_d_b11: f64 = ((s.db[390][11] * s.v[747]) + (s.v[390] * s.db[747][11]));let eq55_e1704_d_b12: f64 = ((s.db[390][12] * s.v[747]) + (s.v[390] * s.db[747][12]));let eq55_e1704_d_b13: f64 = ((s.db[390][13] * s.v[747]) + (s.v[390] * s.db[747][13]));let eq55_e1707: f64 = (s.v[390] * s.v[748]);let eq55_e1707_d_n0: f64 = ((s.dn[390][0] * s.v[748]) + (s.v[390] * s.dn[748][0]));let eq55_e1707_d_n1: f64 = ((s.dn[390][1] * s.v[748]) + (s.v[390] * s.dn[748][1]));let eq55_e1707_d_n2: f64 = ((s.dn[390][2] * s.v[748]) + (s.v[390] * s.dn[748][2]));let eq55_e1707_d_n3: f64 = ((s.dn[390][3] * s.v[748]) + (s.v[390] * s.dn[748][3]));let eq55_e1707_d_n4: f64 = ((s.dn[390][4] * s.v[748]) + (s.v[390] * s.dn[748][4]));let eq55_e1707_d_n5: f64 = ((s.dn[390][5] * s.v[748]) + (s.v[390] * s.dn[748][5]));let eq55_e1707_d_n6: f64 = ((s.dn[390][6] * s.v[748]) + (s.v[390] * s.dn[748][6]));let eq55_e1707_d_n7: f64 = ((s.dn[390][7] * s.v[748]) + (s.v[390] * s.dn[748][7]));let eq55_e1707_d_n8: f64 = ((s.dn[390][8] * s.v[748]) + (s.v[390] * s.dn[748][8]));let eq55_e1707_d_n9: f64 = ((s.dn[390][9] * s.v[748]) + (s.v[390] * s.dn[748][9]));let eq55_e1707_d_n10: f64 = ((s.dn[390][10] * s.v[748]) + (s.v[390] * s.dn[748][10]));let eq55_e1707_d_n11: f64 = ((s.dn[390][11] * s.v[748]) + (s.v[390] * s.dn[748][11]));let eq55_e1707_d_n12: f64 = ((s.dn[390][12] * s.v[748]) + (s.v[390] * s.dn[748][12]));let eq55_e1707_d_n13: f64 = ((s.dn[390][13] * s.v[748]) + (s.v[390] * s.dn[748][13]));let eq55_e1707_d_n14: f64 = ((s.dn[390][14] * s.v[748]) + (s.v[390] * s.dn[748][14]));let eq55_e1707_d_n15: f64 = ((s.dn[390][15] * s.v[748]) + (s.v[390] * s.dn[748][15]));
        let eq55_e1707_d_n16: f64 = ((s.dn[390][16] * s.v[748]) + (s.v[390] * s.dn[748][16]));let eq55_e1707_d_b0: f64 = ((s.db[390][0] * s.v[748]) + (s.v[390] * s.db[748][0]));let eq55_e1707_d_b1: f64 = ((s.db[390][1] * s.v[748]) + (s.v[390] * s.db[748][1]));let eq55_e1707_d_b2: f64 = ((s.db[390][2] * s.v[748]) + (s.v[390] * s.db[748][2]));let eq55_e1707_d_b3: f64 = ((s.db[390][3] * s.v[748]) + (s.v[390] * s.db[748][3]));let eq55_e1707_d_b4: f64 = ((s.db[390][4] * s.v[748]) + (s.v[390] * s.db[748][4]));let eq55_e1707_d_b5: f64 = ((s.db[390][5] * s.v[748]) + (s.v[390] * s.db[748][5]));let eq55_e1707_d_b6: f64 = ((s.db[390][6] * s.v[748]) + (s.v[390] * s.db[748][6]));let eq55_e1707_d_b7: f64 = ((s.db[390][7] * s.v[748]) + (s.v[390] * s.db[748][7]));let eq55_e1707_d_b8: f64 = ((s.db[390][8] * s.v[748]) + (s.v[390] * s.db[748][8]));let eq55_e1707_d_b9: f64 = ((s.db[390][9] * s.v[748]) + (s.v[390] * s.db[748][9]));let eq55_e1707_d_b10: f64 = ((s.db[390][10] * s.v[748]) + (s.v[390] * s.db[748][10]));let eq55_e1707_d_b11: f64 = ((s.db[390][11] * s.v[748]) + (s.v[390] * s.db[748][11]));let eq55_e1707_d_b12: f64 = ((s.db[390][12] * s.v[748]) + (s.v[390] * s.db[748][12]));let eq55_e1707_d_b13: f64 = ((s.db[390][13] * s.v[748]) + (s.v[390] * s.db[748][13]));let eq55_e1708: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq55_e1707);let eq55_e1709: f64 = (eq55_e1704 + eq55_e1708);let eq55_e1709_d_n0: f64 = (eq55_e1704_d_n0 + (eq55_e1707_d_n0 * ddt_scale));let eq55_e1709_d_n1: f64 = (eq55_e1704_d_n1 + (eq55_e1707_d_n1 * ddt_scale));let eq55_e1709_d_n2: f64 = (eq55_e1704_d_n2 + (eq55_e1707_d_n2 * ddt_scale));let eq55_e1709_d_n3: f64 = (eq55_e1704_d_n3 + (eq55_e1707_d_n3 * ddt_scale));let eq55_e1709_d_n4: f64 = (eq55_e1704_d_n4 + (eq55_e1707_d_n4 * ddt_scale));let eq55_e1709_d_n5: f64 = (eq55_e1704_d_n5 + (eq55_e1707_d_n5 * ddt_scale));let eq55_e1709_d_n6: f64 = (eq55_e1704_d_n6 + (eq55_e1707_d_n6 * ddt_scale));let eq55_e1709_d_n7: f64 = (eq55_e1704_d_n7 + (eq55_e1707_d_n7 * ddt_scale));let eq55_e1709_d_n8: f64 = (eq55_e1704_d_n8 + (eq55_e1707_d_n8 * ddt_scale));let eq55_e1709_d_n9: f64 = (eq55_e1704_d_n9 + (eq55_e1707_d_n9 * ddt_scale));let eq55_e1709_d_n10: f64 = (eq55_e1704_d_n10 + (eq55_e1707_d_n10 * ddt_scale));let eq55_e1709_d_n11: f64 = (eq55_e1704_d_n11 + (eq55_e1707_d_n11 * ddt_scale));let eq55_e1709_d_n12: f64 = (eq55_e1704_d_n12 + (eq55_e1707_d_n12 * ddt_scale));let eq55_e1709_d_n13: f64 = (eq55_e1704_d_n13 + (eq55_e1707_d_n13 * ddt_scale));let eq55_e1709_d_n14: f64 = (eq55_e1704_d_n14 + (eq55_e1707_d_n14 * ddt_scale));let eq55_e1709_d_n15: f64 = (eq55_e1704_d_n15 + (eq55_e1707_d_n15 * ddt_scale));let eq55_e1709_d_n16: f64 = (eq55_e1704_d_n16 + (eq55_e1707_d_n16 * ddt_scale));let eq55_e1709_d_b0: f64 = (eq55_e1704_d_b0 + (eq55_e1707_d_b0 * ddt_scale));let eq55_e1709_d_b1: f64 = (eq55_e1704_d_b1 + (eq55_e1707_d_b1 * ddt_scale));let eq55_e1709_d_b2: f64 = (eq55_e1704_d_b2 + (eq55_e1707_d_b2 * ddt_scale));let eq55_e1709_d_b3: f64 = (eq55_e1704_d_b3 + (eq55_e1707_d_b3 * ddt_scale));let eq55_e1709_d_b4: f64 = (eq55_e1704_d_b4 + (eq55_e1707_d_b4 * ddt_scale));let eq55_e1709_d_b5: f64 = (eq55_e1704_d_b5 + (eq55_e1707_d_b5 * ddt_scale));let eq55_e1709_d_b6: f64 = (eq55_e1704_d_b6 + (eq55_e1707_d_b6 * ddt_scale));let eq55_e1709_d_b7: f64 = (eq55_e1704_d_b7 + (eq55_e1707_d_b7 * ddt_scale));let eq55_e1709_d_b8: f64 = (eq55_e1704_d_b8 + (eq55_e1707_d_b8 * ddt_scale));let eq55_e1709_d_b9: f64 = (eq55_e1704_d_b9 + (eq55_e1707_d_b9 * ddt_scale));let eq55_e1709_d_b10: f64 = (eq55_e1704_d_b10 + (eq55_e1707_d_b10 * ddt_scale));let eq55_e1709_d_b11: f64 = (eq55_e1704_d_b11 + (eq55_e1707_d_b11 * ddt_scale));let eq55_e1709_d_b12: f64 = (eq55_e1704_d_b12 + (eq55_e1707_d_b12 * ddt_scale));let eq55_e1709_d_b13: f64 = (eq55_e1704_d_b13 + (eq55_e1707_d_b13 * ddt_scale));let eq55_e1711: f64 = (eq55_e1709 - s.v[749]);
        let eq55_e1711_d_n0: f64 = (eq55_e1709_d_n0 - s.dn[749][0]);let eq55_e1711_d_n1: f64 = (eq55_e1709_d_n1 - s.dn[749][1]);let eq55_e1711_d_n2: f64 = (eq55_e1709_d_n2 - s.dn[749][2]);let eq55_e1711_d_n3: f64 = (eq55_e1709_d_n3 - s.dn[749][3]);let eq55_e1711_d_n4: f64 = (eq55_e1709_d_n4 - s.dn[749][4]);let eq55_e1711_d_n5: f64 = (eq55_e1709_d_n5 - s.dn[749][5]);let eq55_e1711_d_n6: f64 = (eq55_e1709_d_n6 - s.dn[749][6]);let eq55_e1711_d_n7: f64 = (eq55_e1709_d_n7 - s.dn[749][7]);let eq55_e1711_d_n8: f64 = (eq55_e1709_d_n8 - s.dn[749][8]);let eq55_e1711_d_n9: f64 = (eq55_e1709_d_n9 - s.dn[749][9]);let eq55_e1711_d_n10: f64 = (eq55_e1709_d_n10 - s.dn[749][10]);let eq55_e1711_d_n11: f64 = (eq55_e1709_d_n11 - s.dn[749][11]);let eq55_e1711_d_n12: f64 = (eq55_e1709_d_n12 - s.dn[749][12]);let eq55_e1711_d_n13: f64 = (eq55_e1709_d_n13 - s.dn[749][13]);let eq55_e1711_d_n14: f64 = (eq55_e1709_d_n14 - s.dn[749][14]);let eq55_e1711_d_n15: f64 = (eq55_e1709_d_n15 - s.dn[749][15]);let eq55_e1711_d_n16: f64 = (eq55_e1709_d_n16 - s.dn[749][16]);let eq55_e1711_d_b0: f64 = (eq55_e1709_d_b0 - s.db[749][0]);let eq55_e1711_d_b1: f64 = (eq55_e1709_d_b1 - s.db[749][1]);let eq55_e1711_d_b2: f64 = (eq55_e1709_d_b2 - s.db[749][2]);let eq55_e1711_d_b3: f64 = (eq55_e1709_d_b3 - s.db[749][3]);let eq55_e1711_d_b4: f64 = (eq55_e1709_d_b4 - s.db[749][4]);let eq55_e1711_d_b5: f64 = (eq55_e1709_d_b5 - s.db[749][5]);let eq55_e1711_d_b6: f64 = (eq55_e1709_d_b6 - s.db[749][6]);let eq55_e1711_d_b7: f64 = (eq55_e1709_d_b7 - s.db[749][7]);let eq55_e1711_d_b8: f64 = (eq55_e1709_d_b8 - s.db[749][8]);let eq55_e1711_d_b9: f64 = (eq55_e1709_d_b9 - s.db[749][9]);let eq55_e1711_d_b10: f64 = (eq55_e1709_d_b10 - s.db[749][10]);let eq55_e1711_d_b11: f64 = (eq55_e1709_d_b11 - s.db[749][11]);let eq55_e1711_d_b12: f64 = (eq55_e1709_d_b12 - s.db[749][12]);let eq55_e1711_d_b13: f64 = (eq55_e1709_d_b13 - s.db[749][13]);
        (eq55_e1711, eq55_e1711_d_n0, eq55_e1711_d_n1, eq55_e1711_d_n2, eq55_e1711_d_n3, eq55_e1711_d_n4, eq55_e1711_d_n5, eq55_e1711_d_n6, eq55_e1711_d_n7, eq55_e1711_d_n8, eq55_e1711_d_n9, eq55_e1711_d_n10, eq55_e1711_d_n11, eq55_e1711_d_n12, eq55_e1711_d_n13, eq55_e1711_d_n14, eq55_e1711_d_n15, eq55_e1711_d_n16, eq55_e1711_d_b0, eq55_e1711_d_b1, eq55_e1711_d_b2, eq55_e1711_d_b3, eq55_e1711_d_b4, eq55_e1711_d_b5, eq55_e1711_d_b6, eq55_e1711_d_b7, eq55_e1711_d_b8, eq55_e1711_d_b9, eq55_e1711_d_b10, eq55_e1711_d_b11, eq55_e1711_d_b12, eq55_e1711_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1713;let eq55_node_derivatives: [f64; 17] = [eq55_e1713_d_n0, eq55_e1713_d_n1, eq55_e1713_d_n2, eq55_e1713_d_n3, eq55_e1713_d_n4, eq55_e1713_d_n5, eq55_e1713_d_n6, eq55_e1713_d_n7, eq55_e1713_d_n8, eq55_e1713_d_n9, eq55_e1713_d_n10, eq55_e1713_d_n11, eq55_e1713_d_n12, eq55_e1713_d_n13, eq55_e1713_d_n14, eq55_e1713_d_n15, eq55_e1713_d_n16];let eq55_branch_derivatives: [f64; 14] = [eq55_e1713_d_b0, eq55_e1713_d_b1, eq55_e1713_d_b2, eq55_e1713_d_b3, eq55_e1713_d_b4, eq55_e1713_d_b5, eq55_e1713_d_b6, eq55_e1713_d_b7, eq55_e1713_d_b8, eq55_e1713_d_b9, eq55_e1713_d_b10, eq55_e1713_d_b11, eq55_e1713_d_b12, eq55_e1713_d_b13];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
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
        let nv3 = ctx.node_voltage(nodes[3]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq56_e1718,) = {
    if (!s.b[1621]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e1718;
        stamper.stamp_potential_const_local(
            9,
            eq56_value,
        );
        let (eq57_e1726, eq57_e1726_d_n0, eq57_e1726_d_n1, eq57_e1726_d_n2, eq57_e1726_d_n3, eq57_e1726_d_n4, eq57_e1726_d_n5, eq57_e1726_d_n6, eq57_e1726_d_n7, eq57_e1726_d_n8, eq57_e1726_d_n9, eq57_e1726_d_n10, eq57_e1726_d_n11, eq57_e1726_d_n12, eq57_e1726_d_n13, eq57_e1726_d_n14, eq57_e1726_d_n15, eq57_e1726_d_n16, eq57_e1726_d_b0, eq57_e1726_d_b1, eq57_e1726_d_b2, eq57_e1726_d_b3, eq57_e1726_d_b4, eq57_e1726_d_b5, eq57_e1726_d_b6, eq57_e1726_d_b7, eq57_e1726_d_b8, eq57_e1726_d_b9, eq57_e1726_d_b10, eq57_e1726_d_b11, eq57_e1726_d_b12, eq57_e1726_d_b13,) = {
    if s.b[1626] {
        let eq57_e1722: f64 = (p[28] * (nv11 - nv12));let eq57_e1724: f64 = (eq57_e1722 * s.v[274]);let eq57_e1724_d_n11: f64 = ((p[28] * s.v[274]) + (eq57_e1722 * s.dn[274][11]));let eq57_e1724_d_n12: f64 = (((-p[28]) * s.v[274]) + (eq57_e1722 * s.dn[274][12]));
        (eq57_e1724, (eq57_e1722 * s.dn[274][0]), (eq57_e1722 * s.dn[274][1]), (eq57_e1722 * s.dn[274][2]), (eq57_e1722 * s.dn[274][3]), (eq57_e1722 * s.dn[274][4]), (eq57_e1722 * s.dn[274][5]), (eq57_e1722 * s.dn[274][6]), (eq57_e1722 * s.dn[274][7]), (eq57_e1722 * s.dn[274][8]), (eq57_e1722 * s.dn[274][9]), (eq57_e1722 * s.dn[274][10]), eq57_e1724_d_n11, eq57_e1724_d_n12, (eq57_e1722 * s.dn[274][13]), (eq57_e1722 * s.dn[274][14]), (eq57_e1722 * s.dn[274][15]), (eq57_e1722 * s.dn[274][16]), (eq57_e1722 * s.db[274][0]), (eq57_e1722 * s.db[274][1]), (eq57_e1722 * s.db[274][2]), (eq57_e1722 * s.db[274][3]), (eq57_e1722 * s.db[274][4]), (eq57_e1722 * s.db[274][5]), (eq57_e1722 * s.db[274][6]), (eq57_e1722 * s.db[274][7]), (eq57_e1722 * s.db[274][8]), (eq57_e1722 * s.db[274][9]), (eq57_e1722 * s.db[274][10]), (eq57_e1722 * s.db[274][11]), (eq57_e1722 * s.db[274][12]), (eq57_e1722 * s.db[274][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1726;let eq57_node_derivatives: [f64; 17] = [eq57_e1726_d_n0, eq57_e1726_d_n1, eq57_e1726_d_n2, eq57_e1726_d_n3, eq57_e1726_d_n4, eq57_e1726_d_n5, eq57_e1726_d_n6, eq57_e1726_d_n7, eq57_e1726_d_n8, eq57_e1726_d_n9, eq57_e1726_d_n10, eq57_e1726_d_n11, eq57_e1726_d_n12, eq57_e1726_d_n13, eq57_e1726_d_n14, eq57_e1726_d_n15, eq57_e1726_d_n16];let eq57_branch_derivatives: [f64; 14] = [eq57_e1726_d_b0, eq57_e1726_d_b1, eq57_e1726_d_b2, eq57_e1726_d_b3, eq57_e1726_d_b4, eq57_e1726_d_b5, eq57_e1726_d_b6, eq57_e1726_d_b7, eq57_e1726_d_b8, eq57_e1726_d_b9, eq57_e1726_d_b10, eq57_e1726_d_b11, eq57_e1726_d_b12, eq57_e1726_d_b13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e1734, eq58_e1734_d_n0, eq58_e1734_d_n1, eq58_e1734_d_n2, eq58_e1734_d_n3, eq58_e1734_d_n4, eq58_e1734_d_n5, eq58_e1734_d_n6, eq58_e1734_d_n7, eq58_e1734_d_n8, eq58_e1734_d_n9, eq58_e1734_d_n10, eq58_e1734_d_n11, eq58_e1734_d_n12, eq58_e1734_d_n13, eq58_e1734_d_n14, eq58_e1734_d_n15, eq58_e1734_d_n16, eq58_e1734_d_b0, eq58_e1734_d_b1, eq58_e1734_d_b2, eq58_e1734_d_b3, eq58_e1734_d_b4, eq58_e1734_d_b5, eq58_e1734_d_b6, eq58_e1734_d_b7, eq58_e1734_d_b8, eq58_e1734_d_b9, eq58_e1734_d_b10, eq58_e1734_d_b11, eq58_e1734_d_b12, eq58_e1734_d_b13,) = {
    if s.b[1626] {
        let eq58_e1730: f64 = (p[28] * (nv3 - nv12));let eq58_e1732: f64 = (eq58_e1730 * s.v[271]);let eq58_e1732_d_n3: f64 = ((p[28] * s.v[271]) + (eq58_e1730 * s.dn[271][3]));let eq58_e1732_d_n12: f64 = (((-p[28]) * s.v[271]) + (eq58_e1730 * s.dn[271][12]));
        (eq58_e1732, (eq58_e1730 * s.dn[271][0]), (eq58_e1730 * s.dn[271][1]), (eq58_e1730 * s.dn[271][2]), eq58_e1732_d_n3, (eq58_e1730 * s.dn[271][4]), (eq58_e1730 * s.dn[271][5]), (eq58_e1730 * s.dn[271][6]), (eq58_e1730 * s.dn[271][7]), (eq58_e1730 * s.dn[271][8]), (eq58_e1730 * s.dn[271][9]), (eq58_e1730 * s.dn[271][10]), (eq58_e1730 * s.dn[271][11]), eq58_e1732_d_n12, (eq58_e1730 * s.dn[271][13]), (eq58_e1730 * s.dn[271][14]), (eq58_e1730 * s.dn[271][15]), (eq58_e1730 * s.dn[271][16]), (eq58_e1730 * s.db[271][0]), (eq58_e1730 * s.db[271][1]), (eq58_e1730 * s.db[271][2]), (eq58_e1730 * s.db[271][3]), (eq58_e1730 * s.db[271][4]), (eq58_e1730 * s.db[271][5]), (eq58_e1730 * s.db[271][6]), (eq58_e1730 * s.db[271][7]), (eq58_e1730 * s.db[271][8]), (eq58_e1730 * s.db[271][9]), (eq58_e1730 * s.db[271][10]), (eq58_e1730 * s.db[271][11]), (eq58_e1730 * s.db[271][12]), (eq58_e1730 * s.db[271][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e1734;let eq58_node_derivatives: [f64; 17] = [eq58_e1734_d_n0, eq58_e1734_d_n1, eq58_e1734_d_n2, eq58_e1734_d_n3, eq58_e1734_d_n4, eq58_e1734_d_n5, eq58_e1734_d_n6, eq58_e1734_d_n7, eq58_e1734_d_n8, eq58_e1734_d_n9, eq58_e1734_d_n10, eq58_e1734_d_n11, eq58_e1734_d_n12, eq58_e1734_d_n13, eq58_e1734_d_n14, eq58_e1734_d_n15, eq58_e1734_d_n16];let eq58_branch_derivatives: [f64; 14] = [eq58_e1734_d_b0, eq58_e1734_d_b1, eq58_e1734_d_b2, eq58_e1734_d_b3, eq58_e1734_d_b4, eq58_e1734_d_b5, eq58_e1734_d_b6, eq58_e1734_d_b7, eq58_e1734_d_b8, eq58_e1734_d_b9, eq58_e1734_d_b10, eq58_e1734_d_b11, eq58_e1734_d_b12, eq58_e1734_d_b13];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(12),
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e1742, eq59_e1742_d_n0, eq59_e1742_d_n1, eq59_e1742_d_n2, eq59_e1742_d_n3, eq59_e1742_d_n4, eq59_e1742_d_n5, eq59_e1742_d_n6, eq59_e1742_d_n7, eq59_e1742_d_n8, eq59_e1742_d_n9, eq59_e1742_d_n10, eq59_e1742_d_n11, eq59_e1742_d_n12, eq59_e1742_d_n13, eq59_e1742_d_n14, eq59_e1742_d_n15, eq59_e1742_d_n16, eq59_e1742_d_b0, eq59_e1742_d_b1, eq59_e1742_d_b2, eq59_e1742_d_b3, eq59_e1742_d_b4, eq59_e1742_d_b5, eq59_e1742_d_b6, eq59_e1742_d_b7, eq59_e1742_d_b8, eq59_e1742_d_b9, eq59_e1742_d_b10, eq59_e1742_d_b11, eq59_e1742_d_b12, eq59_e1742_d_b13,) = {
    if s.b[1626] {
        let eq59_e1738: f64 = (p[28] * (nv3 - nv11));let eq59_e1740: f64 = (eq59_e1738 * s.v[273]);let eq59_e1740_d_n3: f64 = ((p[28] * s.v[273]) + (eq59_e1738 * s.dn[273][3]));let eq59_e1740_d_n11: f64 = (((-p[28]) * s.v[273]) + (eq59_e1738 * s.dn[273][11]));
        (eq59_e1740, (eq59_e1738 * s.dn[273][0]), (eq59_e1738 * s.dn[273][1]), (eq59_e1738 * s.dn[273][2]), eq59_e1740_d_n3, (eq59_e1738 * s.dn[273][4]), (eq59_e1738 * s.dn[273][5]), (eq59_e1738 * s.dn[273][6]), (eq59_e1738 * s.dn[273][7]), (eq59_e1738 * s.dn[273][8]), (eq59_e1738 * s.dn[273][9]), (eq59_e1738 * s.dn[273][10]), eq59_e1740_d_n11, (eq59_e1738 * s.dn[273][12]), (eq59_e1738 * s.dn[273][13]), (eq59_e1738 * s.dn[273][14]), (eq59_e1738 * s.dn[273][15]), (eq59_e1738 * s.dn[273][16]), (eq59_e1738 * s.db[273][0]), (eq59_e1738 * s.db[273][1]), (eq59_e1738 * s.db[273][2]), (eq59_e1738 * s.db[273][3]), (eq59_e1738 * s.db[273][4]), (eq59_e1738 * s.db[273][5]), (eq59_e1738 * s.db[273][6]), (eq59_e1738 * s.db[273][7]), (eq59_e1738 * s.db[273][8]), (eq59_e1738 * s.db[273][9]), (eq59_e1738 * s.db[273][10]), (eq59_e1738 * s.db[273][11]), (eq59_e1738 * s.db[273][12]), (eq59_e1738 * s.db[273][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1742;let eq59_node_derivatives: [f64; 17] = [eq59_e1742_d_n0, eq59_e1742_d_n1, eq59_e1742_d_n2, eq59_e1742_d_n3, eq59_e1742_d_n4, eq59_e1742_d_n5, eq59_e1742_d_n6, eq59_e1742_d_n7, eq59_e1742_d_n8, eq59_e1742_d_n9, eq59_e1742_d_n10, eq59_e1742_d_n11, eq59_e1742_d_n12, eq59_e1742_d_n13, eq59_e1742_d_n14, eq59_e1742_d_n15, eq59_e1742_d_n16];let eq59_branch_derivatives: [f64; 14] = [eq59_e1742_d_b0, eq59_e1742_d_b1, eq59_e1742_d_b2, eq59_e1742_d_b3, eq59_e1742_d_b4, eq59_e1742_d_b5, eq59_e1742_d_b6, eq59_e1742_d_b7, eq59_e1742_d_b8, eq59_e1742_d_b9, eq59_e1742_d_b10, eq59_e1742_d_b11, eq59_e1742_d_b12, eq59_e1742_d_b13];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(11),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1750, eq60_e1750_d_n0, eq60_e1750_d_n1, eq60_e1750_d_n2, eq60_e1750_d_n3, eq60_e1750_d_n4, eq60_e1750_d_n5, eq60_e1750_d_n6, eq60_e1750_d_n7, eq60_e1750_d_n8, eq60_e1750_d_n9, eq60_e1750_d_n10, eq60_e1750_d_n11, eq60_e1750_d_n12, eq60_e1750_d_n13, eq60_e1750_d_n14, eq60_e1750_d_n15, eq60_e1750_d_n16, eq60_e1750_d_b0, eq60_e1750_d_b1, eq60_e1750_d_b2, eq60_e1750_d_b3, eq60_e1750_d_b4, eq60_e1750_d_b5, eq60_e1750_d_b6, eq60_e1750_d_b7, eq60_e1750_d_b8, eq60_e1750_d_b9, eq60_e1750_d_b10, eq60_e1750_d_b11, eq60_e1750_d_b12, eq60_e1750_d_b13,) = {
    if s.b[1626] {
        let eq60_e1746: f64 = (p[28] * (nv3 - nv13));let eq60_e1748: f64 = (eq60_e1746 * s.v[272]);let eq60_e1748_d_n3: f64 = ((p[28] * s.v[272]) + (eq60_e1746 * s.dn[272][3]));let eq60_e1748_d_n13: f64 = (((-p[28]) * s.v[272]) + (eq60_e1746 * s.dn[272][13]));
        (eq60_e1748, (eq60_e1746 * s.dn[272][0]), (eq60_e1746 * s.dn[272][1]), (eq60_e1746 * s.dn[272][2]), eq60_e1748_d_n3, (eq60_e1746 * s.dn[272][4]), (eq60_e1746 * s.dn[272][5]), (eq60_e1746 * s.dn[272][6]), (eq60_e1746 * s.dn[272][7]), (eq60_e1746 * s.dn[272][8]), (eq60_e1746 * s.dn[272][9]), (eq60_e1746 * s.dn[272][10]), (eq60_e1746 * s.dn[272][11]), (eq60_e1746 * s.dn[272][12]), eq60_e1748_d_n13, (eq60_e1746 * s.dn[272][14]), (eq60_e1746 * s.dn[272][15]), (eq60_e1746 * s.dn[272][16]), (eq60_e1746 * s.db[272][0]), (eq60_e1746 * s.db[272][1]), (eq60_e1746 * s.db[272][2]), (eq60_e1746 * s.db[272][3]), (eq60_e1746 * s.db[272][4]), (eq60_e1746 * s.db[272][5]), (eq60_e1746 * s.db[272][6]), (eq60_e1746 * s.db[272][7]), (eq60_e1746 * s.db[272][8]), (eq60_e1746 * s.db[272][9]), (eq60_e1746 * s.db[272][10]), (eq60_e1746 * s.db[272][11]), (eq60_e1746 * s.db[272][12]), (eq60_e1746 * s.db[272][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1750;let eq60_node_derivatives: [f64; 17] = [eq60_e1750_d_n0, eq60_e1750_d_n1, eq60_e1750_d_n2, eq60_e1750_d_n3, eq60_e1750_d_n4, eq60_e1750_d_n5, eq60_e1750_d_n6, eq60_e1750_d_n7, eq60_e1750_d_n8, eq60_e1750_d_n9, eq60_e1750_d_n10, eq60_e1750_d_n11, eq60_e1750_d_n12, eq60_e1750_d_n13, eq60_e1750_d_n14, eq60_e1750_d_n15, eq60_e1750_d_n16];let eq60_branch_derivatives: [f64; 14] = [eq60_e1750_d_b0, eq60_e1750_d_b1, eq60_e1750_d_b2, eq60_e1750_d_b3, eq60_e1750_d_b4, eq60_e1750_d_b5, eq60_e1750_d_b6, eq60_e1750_d_b7, eq60_e1750_d_b8, eq60_e1750_d_b9, eq60_e1750_d_b10, eq60_e1750_d_b11, eq60_e1750_d_b12, eq60_e1750_d_b13];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(13),
            multiplicity * (eq60_value),
            &eq60_node_derivatives,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1758, eq61_e1758_d_n0, eq61_e1758_d_n1, eq61_e1758_d_n2, eq61_e1758_d_n3, eq61_e1758_d_n4, eq61_e1758_d_n5, eq61_e1758_d_n6, eq61_e1758_d_n7, eq61_e1758_d_n8, eq61_e1758_d_n9, eq61_e1758_d_n10, eq61_e1758_d_n11, eq61_e1758_d_n12, eq61_e1758_d_n13, eq61_e1758_d_n14, eq61_e1758_d_n15, eq61_e1758_d_n16, eq61_e1758_d_b0, eq61_e1758_d_b1, eq61_e1758_d_b2, eq61_e1758_d_b3, eq61_e1758_d_b4, eq61_e1758_d_b5, eq61_e1758_d_b6, eq61_e1758_d_b7, eq61_e1758_d_b8, eq61_e1758_d_b9, eq61_e1758_d_b10, eq61_e1758_d_b11, eq61_e1758_d_b12, eq61_e1758_d_b13,) = {
    if s.b[1626] {
        let eq61_e1754: f64 = (p[28] * (nv11 - nv13));let eq61_e1756: f64 = (eq61_e1754 * s.v[275]);let eq61_e1756_d_n11: f64 = ((p[28] * s.v[275]) + (eq61_e1754 * s.dn[275][11]));let eq61_e1756_d_n13: f64 = (((-p[28]) * s.v[275]) + (eq61_e1754 * s.dn[275][13]));
        (eq61_e1756, (eq61_e1754 * s.dn[275][0]), (eq61_e1754 * s.dn[275][1]), (eq61_e1754 * s.dn[275][2]), (eq61_e1754 * s.dn[275][3]), (eq61_e1754 * s.dn[275][4]), (eq61_e1754 * s.dn[275][5]), (eq61_e1754 * s.dn[275][6]), (eq61_e1754 * s.dn[275][7]), (eq61_e1754 * s.dn[275][8]), (eq61_e1754 * s.dn[275][9]), (eq61_e1754 * s.dn[275][10]), eq61_e1756_d_n11, (eq61_e1754 * s.dn[275][12]), eq61_e1756_d_n13, (eq61_e1754 * s.dn[275][14]), (eq61_e1754 * s.dn[275][15]), (eq61_e1754 * s.dn[275][16]), (eq61_e1754 * s.db[275][0]), (eq61_e1754 * s.db[275][1]), (eq61_e1754 * s.db[275][2]), (eq61_e1754 * s.db[275][3]), (eq61_e1754 * s.db[275][4]), (eq61_e1754 * s.db[275][5]), (eq61_e1754 * s.db[275][6]), (eq61_e1754 * s.db[275][7]), (eq61_e1754 * s.db[275][8]), (eq61_e1754 * s.db[275][9]), (eq61_e1754 * s.db[275][10]), (eq61_e1754 * s.db[275][11]), (eq61_e1754 * s.db[275][12]), (eq61_e1754 * s.db[275][13]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1758;let eq61_node_derivatives: [f64; 17] = [eq61_e1758_d_n0, eq61_e1758_d_n1, eq61_e1758_d_n2, eq61_e1758_d_n3, eq61_e1758_d_n4, eq61_e1758_d_n5, eq61_e1758_d_n6, eq61_e1758_d_n7, eq61_e1758_d_n8, eq61_e1758_d_n9, eq61_e1758_d_n10, eq61_e1758_d_n11, eq61_e1758_d_n12, eq61_e1758_d_n13, eq61_e1758_d_n14, eq61_e1758_d_n15, eq61_e1758_d_n16];let eq61_branch_derivatives: [f64; 14] = [eq61_e1758_d_b0, eq61_e1758_d_b1, eq61_e1758_d_b2, eq61_e1758_d_b3, eq61_e1758_d_b4, eq61_e1758_d_b5, eq61_e1758_d_b6, eq61_e1758_d_b7, eq61_e1758_d_b8, eq61_e1758_d_b9, eq61_e1758_d_b10, eq61_e1758_d_b11, eq61_e1758_d_b12, eq61_e1758_d_b13];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(13),
            multiplicity * (eq61_value),
            &eq61_node_derivatives,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1813,) = {
    if (!s.b[1626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1813;
        stamper.stamp_potential_const_local(
            10,
            eq67_value,
        );
        let (eq68_e1818,) = {
    if (!s.b[1626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1818;
        stamper.stamp_potential_const_local(
            11,
            eq68_value,
        );
        let (eq69_e1823,) = {
    if (!s.b[1626]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1823;
        stamper.stamp_potential_const_local(
            12,
            eq69_value,
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
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv12 = ctx.node_voltage(nodes[12]);
        let (eq70_e1837, eq70_e1837_d_n0, eq70_e1837_d_n1, eq70_e1837_d_n2, eq70_e1837_d_n3, eq70_e1837_d_n4, eq70_e1837_d_n5, eq70_e1837_d_n6, eq70_e1837_d_n7, eq70_e1837_d_n8, eq70_e1837_d_n9, eq70_e1837_d_n10, eq70_e1837_d_n11, eq70_e1837_d_n12, eq70_e1837_d_n13, eq70_e1837_d_n14, eq70_e1837_d_n15, eq70_e1837_d_n16, eq70_e1837_d_b0, eq70_e1837_d_b1, eq70_e1837_d_b2, eq70_e1837_d_b3, eq70_e1837_d_b4, eq70_e1837_d_b5, eq70_e1837_d_b6, eq70_e1837_d_b7, eq70_e1837_d_b8, eq70_e1837_d_b9, eq70_e1837_d_b10, eq70_e1837_d_b11, eq70_e1837_d_b12, eq70_e1837_d_b13,) = {
    if s.b[1627] {
        let eq70_e1827: f64 = (s.v[187] * p[28]);let eq70_e1829: f64 = (eq70_e1827 * s.v[303]);let eq70_e1829_d_n0: f64 = (((s.dn[187][0] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][0]));let eq70_e1829_d_n1: f64 = (((s.dn[187][1] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][1]));let eq70_e1829_d_n2: f64 = (((s.dn[187][2] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][2]));let eq70_e1829_d_n3: f64 = (((s.dn[187][3] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][3]));let eq70_e1829_d_n4: f64 = (((s.dn[187][4] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][4]));let eq70_e1829_d_n5: f64 = (((s.dn[187][5] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][5]));let eq70_e1829_d_n6: f64 = (((s.dn[187][6] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][6]));let eq70_e1829_d_n7: f64 = (((s.dn[187][7] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][7]));let eq70_e1829_d_n8: f64 = (((s.dn[187][8] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][8]));let eq70_e1829_d_n9: f64 = (((s.dn[187][9] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][9]));let eq70_e1829_d_n10: f64 = (((s.dn[187][10] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][10]));let eq70_e1829_d_n11: f64 = (((s.dn[187][11] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][11]));let eq70_e1829_d_n12: f64 = (((s.dn[187][12] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][12]));let eq70_e1829_d_n13: f64 = (((s.dn[187][13] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][13]));let eq70_e1829_d_n14: f64 = (((s.dn[187][14] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][14]));let eq70_e1829_d_n15: f64 = (((s.dn[187][15] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][15]));let eq70_e1829_d_n16: f64 = (((s.dn[187][16] * p[28]) * s.v[303]) + (eq70_e1827 * s.dn[303][16]));let eq70_e1829_d_b0: f64 = (((s.db[187][0] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][0]));let eq70_e1829_d_b1: f64 = (((s.db[187][1] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][1]));let eq70_e1829_d_b2: f64 = (((s.db[187][2] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][2]));let eq70_e1829_d_b3: f64 = (((s.db[187][3] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][3]));let eq70_e1829_d_b4: f64 = (((s.db[187][4] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][4]));let eq70_e1829_d_b5: f64 = (((s.db[187][5] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][5]));let eq70_e1829_d_b6: f64 = (((s.db[187][6] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][6]));let eq70_e1829_d_b7: f64 = (((s.db[187][7] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][7]));let eq70_e1829_d_b8: f64 = (((s.db[187][8] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][8]));let eq70_e1829_d_b9: f64 = (((s.db[187][9] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][9]));let eq70_e1829_d_b10: f64 = (((s.db[187][10] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][10]));let eq70_e1829_d_b11: f64 = (((s.db[187][11] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][11]));let eq70_e1829_d_b12: f64 = (((s.db[187][12] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][12]));let eq70_e1829_d_b13: f64 = (((s.db[187][13] * p[28]) * s.v[303]) + (eq70_e1827 * s.db[303][13]));let eq70_e1832: f64 = ((nv12 - nv7) * p[28]);let eq70_e1834: f64 = (eq70_e1832 * s.v[781]);let eq70_e1834_d_n7: f64 = ((-p[28]) * s.v[781]);let eq70_e1834_d_n12: f64 = (p[28] * s.v[781]);let eq70_e1835: f64 = (eq70_e1829 + eq70_e1834);let eq70_e1835_d_n7: f64 = (eq70_e1829_d_n7 + eq70_e1834_d_n7);let eq70_e1835_d_n12: f64 = (eq70_e1829_d_n12 + eq70_e1834_d_n12);
        (eq70_e1835, eq70_e1829_d_n0, eq70_e1829_d_n1, eq70_e1829_d_n2, eq70_e1829_d_n3, eq70_e1829_d_n4, eq70_e1829_d_n5, eq70_e1829_d_n6, eq70_e1835_d_n7, eq70_e1829_d_n8, eq70_e1829_d_n9, eq70_e1829_d_n10, eq70_e1829_d_n11, eq70_e1835_d_n12, eq70_e1829_d_n13, eq70_e1829_d_n14, eq70_e1829_d_n15, eq70_e1829_d_n16, eq70_e1829_d_b0, eq70_e1829_d_b1, eq70_e1829_d_b2, eq70_e1829_d_b3, eq70_e1829_d_b4, eq70_e1829_d_b5, eq70_e1829_d_b6, eq70_e1829_d_b7, eq70_e1829_d_b8, eq70_e1829_d_b9, eq70_e1829_d_b10, eq70_e1829_d_b11, eq70_e1829_d_b12, eq70_e1829_d_b13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1837;let eq70_node_derivatives: [f64; 17] = [eq70_e1837_d_n0, eq70_e1837_d_n1, eq70_e1837_d_n2, eq70_e1837_d_n3, eq70_e1837_d_n4, eq70_e1837_d_n5, eq70_e1837_d_n6, eq70_e1837_d_n7, eq70_e1837_d_n8, eq70_e1837_d_n9, eq70_e1837_d_n10, eq70_e1837_d_n11, eq70_e1837_d_n12, eq70_e1837_d_n13, eq70_e1837_d_n14, eq70_e1837_d_n15, eq70_e1837_d_n16];let eq70_branch_derivatives: [f64; 14] = [eq70_e1837_d_b0, eq70_e1837_d_b1, eq70_e1837_d_b2, eq70_e1837_d_b3, eq70_e1837_d_b4, eq70_e1837_d_b5, eq70_e1837_d_b6, eq70_e1837_d_b7, eq70_e1837_d_b8, eq70_e1837_d_b9, eq70_e1837_d_b10, eq70_e1837_d_b11, eq70_e1837_d_b12, eq70_e1837_d_b13];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq70_value),
            &eq70_node_derivatives,
            &eq70_branch_derivatives,
            multiplicity,
        );
    }
}
