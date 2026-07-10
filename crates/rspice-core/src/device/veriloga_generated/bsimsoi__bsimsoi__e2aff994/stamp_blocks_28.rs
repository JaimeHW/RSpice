#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq12_e1635, eq12_e1635_d_n13,) = {
    if s.b[1620] {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1635;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq12_value),
            13,
            multiplicity * (eq12_e1635_d_n13),
        );
        let (eq13_e1639, eq13_e1639_d_n12,) = {
    if s.b[1620] {
        ((nv12 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1639;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq13_value),
            12,
            multiplicity * (eq13_e1639_d_n12),
        );
        let (eq17_e1691, eq17_e1691_d_n0, eq17_e1691_d_n1, eq17_e1691_d_n2, eq17_e1691_d_n3, eq17_e1691_d_n4, eq17_e1691_d_n5, eq17_e1691_d_n6, eq17_e1691_d_n7, eq17_e1691_d_n8, eq17_e1691_d_n9, eq17_e1691_d_n10, eq17_e1691_d_n11, eq17_e1691_d_n12, eq17_e1691_d_n13, eq17_e1691_d_b0, eq17_e1691_d_b1, eq17_e1691_d_b2, eq17_e1691_d_b3, eq17_e1691_d_b4, eq17_e1691_d_b5, eq17_e1691_d_b6, eq17_e1691_d_b7, eq17_e1691_d_b8, eq17_e1691_d_b9, eq17_e1691_d_b10, eq17_e1691_d_b11,) = {
    if ((!s.b[1620]) && s.b[1947]) {
        let eq17_e1689: f64 = (s.v[379] * s.v[974]);let eq17_e1689_d_n0: f64 = ((s.dn[379][0] * s.v[974]) + (s.v[379] * s.dn[974][0]));let eq17_e1689_d_n1: f64 = ((s.dn[379][1] * s.v[974]) + (s.v[379] * s.dn[974][1]));let eq17_e1689_d_n2: f64 = ((s.dn[379][2] * s.v[974]) + (s.v[379] * s.dn[974][2]));let eq17_e1689_d_n3: f64 = ((s.dn[379][3] * s.v[974]) + (s.v[379] * s.dn[974][3]));let eq17_e1689_d_n4: f64 = ((s.dn[379][4] * s.v[974]) + (s.v[379] * s.dn[974][4]));let eq17_e1689_d_n5: f64 = ((s.dn[379][5] * s.v[974]) + (s.v[379] * s.dn[974][5]));let eq17_e1689_d_n6: f64 = ((s.dn[379][6] * s.v[974]) + (s.v[379] * s.dn[974][6]));let eq17_e1689_d_n7: f64 = ((s.dn[379][7] * s.v[974]) + (s.v[379] * s.dn[974][7]));let eq17_e1689_d_n8: f64 = ((s.dn[379][8] * s.v[974]) + (s.v[379] * s.dn[974][8]));let eq17_e1689_d_n9: f64 = ((s.dn[379][9] * s.v[974]) + (s.v[379] * s.dn[974][9]));let eq17_e1689_d_n10: f64 = ((s.dn[379][10] * s.v[974]) + (s.v[379] * s.dn[974][10]));let eq17_e1689_d_n11: f64 = ((s.dn[379][11] * s.v[974]) + (s.v[379] * s.dn[974][11]));let eq17_e1689_d_n12: f64 = ((s.dn[379][12] * s.v[974]) + (s.v[379] * s.dn[974][12]));let eq17_e1689_d_n13: f64 = ((s.dn[379][13] * s.v[974]) + (s.v[379] * s.dn[974][13]));let eq17_e1689_d_b0: f64 = ((s.db[379][0] * s.v[974]) + (s.v[379] * s.db[974][0]));let eq17_e1689_d_b1: f64 = ((s.db[379][1] * s.v[974]) + (s.v[379] * s.db[974][1]));let eq17_e1689_d_b2: f64 = ((s.db[379][2] * s.v[974]) + (s.v[379] * s.db[974][2]));let eq17_e1689_d_b3: f64 = ((s.db[379][3] * s.v[974]) + (s.v[379] * s.db[974][3]));let eq17_e1689_d_b4: f64 = ((s.db[379][4] * s.v[974]) + (s.v[379] * s.db[974][4]));let eq17_e1689_d_b5: f64 = ((s.db[379][5] * s.v[974]) + (s.v[379] * s.db[974][5]));let eq17_e1689_d_b6: f64 = ((s.db[379][6] * s.v[974]) + (s.v[379] * s.db[974][6]));let eq17_e1689_d_b7: f64 = ((s.db[379][7] * s.v[974]) + (s.v[379] * s.db[974][7]));let eq17_e1689_d_b8: f64 = ((s.db[379][8] * s.v[974]) + (s.v[379] * s.db[974][8]));let eq17_e1689_d_b9: f64 = ((s.db[379][9] * s.v[974]) + (s.v[379] * s.db[974][9]));let eq17_e1689_d_b10: f64 = ((s.db[379][10] * s.v[974]) + (s.v[379] * s.db[974][10]));let eq17_e1689_d_b11: f64 = ((s.db[379][11] * s.v[974]) + (s.v[379] * s.db[974][11]));
        (eq17_e1689, eq17_e1689_d_n0, eq17_e1689_d_n1, eq17_e1689_d_n2, eq17_e1689_d_n3, eq17_e1689_d_n4, eq17_e1689_d_n5, eq17_e1689_d_n6, eq17_e1689_d_n7, eq17_e1689_d_n8, eq17_e1689_d_n9, eq17_e1689_d_n10, eq17_e1689_d_n11, eq17_e1689_d_n12, eq17_e1689_d_n13, eq17_e1689_d_b0, eq17_e1689_d_b1, eq17_e1689_d_b2, eq17_e1689_d_b3, eq17_e1689_d_b4, eq17_e1689_d_b5, eq17_e1689_d_b6, eq17_e1689_d_b7, eq17_e1689_d_b8, eq17_e1689_d_b9, eq17_e1689_d_b10, eq17_e1689_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1691;let eq17_node_derivatives: [f64; 14] = [eq17_e1691_d_n0, eq17_e1691_d_n1, eq17_e1691_d_n2, eq17_e1691_d_n3, eq17_e1691_d_n4, eq17_e1691_d_n5, eq17_e1691_d_n6, eq17_e1691_d_n7, eq17_e1691_d_n8, eq17_e1691_d_n9, eq17_e1691_d_n10, eq17_e1691_d_n11, eq17_e1691_d_n12, eq17_e1691_d_n13];let eq17_branch_derivatives: [f64; 12] = [eq17_e1691_d_b0, eq17_e1691_d_b1, eq17_e1691_d_b2, eq17_e1691_d_b3, eq17_e1691_d_b4, eq17_e1691_d_b5, eq17_e1691_d_b6, eq17_e1691_d_b7, eq17_e1691_d_b8, eq17_e1691_d_b9, eq17_e1691_d_b10, eq17_e1691_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1768, eq23_e1768_d_n0, eq23_e1768_d_n1, eq23_e1768_d_n2, eq23_e1768_d_n3, eq23_e1768_d_n4, eq23_e1768_d_n5, eq23_e1768_d_n6, eq23_e1768_d_n7, eq23_e1768_d_n8, eq23_e1768_d_n9, eq23_e1768_d_n10, eq23_e1768_d_n11, eq23_e1768_d_n12, eq23_e1768_d_n13, eq23_e1768_d_b0, eq23_e1768_d_b1, eq23_e1768_d_b2, eq23_e1768_d_b3, eq23_e1768_d_b4, eq23_e1768_d_b5, eq23_e1768_d_b6, eq23_e1768_d_b7, eq23_e1768_d_b8, eq23_e1768_d_b9, eq23_e1768_d_b10, eq23_e1768_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq23_e1764: f64 = (-s.v[629]);let eq23_e1766: f64 = (eq23_e1764 * (nv13 - 0.0));let eq23_e1766_d_n0: f64 = ((-s.dn[629][0]) * (nv13 - 0.0));let eq23_e1766_d_n1: f64 = ((-s.dn[629][1]) * (nv13 - 0.0));let eq23_e1766_d_n2: f64 = ((-s.dn[629][2]) * (nv13 - 0.0));let eq23_e1766_d_n3: f64 = ((-s.dn[629][3]) * (nv13 - 0.0));let eq23_e1766_d_n4: f64 = ((-s.dn[629][4]) * (nv13 - 0.0));let eq23_e1766_d_n5: f64 = ((-s.dn[629][5]) * (nv13 - 0.0));let eq23_e1766_d_n6: f64 = ((-s.dn[629][6]) * (nv13 - 0.0));let eq23_e1766_d_n7: f64 = ((-s.dn[629][7]) * (nv13 - 0.0));let eq23_e1766_d_n8: f64 = ((-s.dn[629][8]) * (nv13 - 0.0));let eq23_e1766_d_n9: f64 = ((-s.dn[629][9]) * (nv13 - 0.0));let eq23_e1766_d_n10: f64 = ((-s.dn[629][10]) * (nv13 - 0.0));let eq23_e1766_d_n11: f64 = ((-s.dn[629][11]) * (nv13 - 0.0));let eq23_e1766_d_n12: f64 = ((-s.dn[629][12]) * (nv13 - 0.0));let eq23_e1766_d_n13: f64 = (((-s.dn[629][13]) * (nv13 - 0.0)) + eq23_e1764);let eq23_e1766_d_b0: f64 = ((-s.db[629][0]) * (nv13 - 0.0));let eq23_e1766_d_b1: f64 = ((-s.db[629][1]) * (nv13 - 0.0));let eq23_e1766_d_b2: f64 = ((-s.db[629][2]) * (nv13 - 0.0));let eq23_e1766_d_b3: f64 = ((-s.db[629][3]) * (nv13 - 0.0));let eq23_e1766_d_b4: f64 = ((-s.db[629][4]) * (nv13 - 0.0));let eq23_e1766_d_b5: f64 = ((-s.db[629][5]) * (nv13 - 0.0));let eq23_e1766_d_b6: f64 = ((-s.db[629][6]) * (nv13 - 0.0));let eq23_e1766_d_b7: f64 = ((-s.db[629][7]) * (nv13 - 0.0));let eq23_e1766_d_b8: f64 = ((-s.db[629][8]) * (nv13 - 0.0));let eq23_e1766_d_b9: f64 = ((-s.db[629][9]) * (nv13 - 0.0));let eq23_e1766_d_b10: f64 = ((-s.db[629][10]) * (nv13 - 0.0));let eq23_e1766_d_b11: f64 = ((-s.db[629][11]) * (nv13 - 0.0));
        (eq23_e1766, eq23_e1766_d_n0, eq23_e1766_d_n1, eq23_e1766_d_n2, eq23_e1766_d_n3, eq23_e1766_d_n4, eq23_e1766_d_n5, eq23_e1766_d_n6, eq23_e1766_d_n7, eq23_e1766_d_n8, eq23_e1766_d_n9, eq23_e1766_d_n10, eq23_e1766_d_n11, eq23_e1766_d_n12, eq23_e1766_d_n13, eq23_e1766_d_b0, eq23_e1766_d_b1, eq23_e1766_d_b2, eq23_e1766_d_b3, eq23_e1766_d_b4, eq23_e1766_d_b5, eq23_e1766_d_b6, eq23_e1766_d_b7, eq23_e1766_d_b8, eq23_e1766_d_b9, eq23_e1766_d_b10, eq23_e1766_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1768;let eq23_node_derivatives: [f64; 14] = [eq23_e1768_d_n0, eq23_e1768_d_n1, eq23_e1768_d_n2, eq23_e1768_d_n3, eq23_e1768_d_n4, eq23_e1768_d_n5, eq23_e1768_d_n6, eq23_e1768_d_n7, eq23_e1768_d_n8, eq23_e1768_d_n9, eq23_e1768_d_n10, eq23_e1768_d_n11, eq23_e1768_d_n12, eq23_e1768_d_n13];let eq23_branch_derivatives: [f64; 12] = [eq23_e1768_d_b0, eq23_e1768_d_b1, eq23_e1768_d_b2, eq23_e1768_d_b3, eq23_e1768_d_b4, eq23_e1768_d_b5, eq23_e1768_d_b6, eq23_e1768_d_b7, eq23_e1768_d_b8, eq23_e1768_d_b9, eq23_e1768_d_b10, eq23_e1768_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq24_e1789, eq24_e1789_d_n0, eq24_e1789_d_n1, eq24_e1789_d_n2, eq24_e1789_d_n3, eq24_e1789_d_n4, eq24_e1789_d_n5, eq24_e1789_d_n6, eq24_e1789_d_n7, eq24_e1789_d_n8, eq24_e1789_d_n9, eq24_e1789_d_n10, eq24_e1789_d_n11, eq24_e1789_d_n12, eq24_e1789_d_n13, eq24_e1789_d_b0, eq24_e1789_d_b1, eq24_e1789_d_b2, eq24_e1789_d_b3, eq24_e1789_d_b4, eq24_e1789_d_b5, eq24_e1789_d_b6, eq24_e1789_d_b7, eq24_e1789_d_b8, eq24_e1789_d_b9, eq24_e1789_d_b10, eq24_e1789_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq24_e1778: f64 = (s.v[622] * s.v[199]);let eq24_e1780: f64 = (eq24_e1778 * s.v[183]);let eq24_e1780_d_n0: f64 = ((s.dn[622][0] * s.v[199]) * s.v[183]);let eq24_e1780_d_n1: f64 = ((s.dn[622][1] * s.v[199]) * s.v[183]);let eq24_e1780_d_n2: f64 = ((s.dn[622][2] * s.v[199]) * s.v[183]);let eq24_e1780_d_n3: f64 = ((s.dn[622][3] * s.v[199]) * s.v[183]);let eq24_e1780_d_n4: f64 = ((s.dn[622][4] * s.v[199]) * s.v[183]);let eq24_e1780_d_n5: f64 = ((s.dn[622][5] * s.v[199]) * s.v[183]);let eq24_e1780_d_n6: f64 = ((s.dn[622][6] * s.v[199]) * s.v[183]);let eq24_e1780_d_n7: f64 = ((s.dn[622][7] * s.v[199]) * s.v[183]);let eq24_e1780_d_n8: f64 = ((s.dn[622][8] * s.v[199]) * s.v[183]);let eq24_e1780_d_n9: f64 = ((s.dn[622][9] * s.v[199]) * s.v[183]);let eq24_e1780_d_n10: f64 = ((s.dn[622][10] * s.v[199]) * s.v[183]);let eq24_e1780_d_n11: f64 = ((s.dn[622][11] * s.v[199]) * s.v[183]);let eq24_e1780_d_n12: f64 = ((s.dn[622][12] * s.v[199]) * s.v[183]);let eq24_e1780_d_n13: f64 = ((s.dn[622][13] * s.v[199]) * s.v[183]);let eq24_e1780_d_b0: f64 = ((s.db[622][0] * s.v[199]) * s.v[183]);let eq24_e1780_d_b1: f64 = ((s.db[622][1] * s.v[199]) * s.v[183]);let eq24_e1780_d_b2: f64 = ((s.db[622][2] * s.v[199]) * s.v[183]);let eq24_e1780_d_b3: f64 = ((s.db[622][3] * s.v[199]) * s.v[183]);let eq24_e1780_d_b4: f64 = ((s.db[622][4] * s.v[199]) * s.v[183]);let eq24_e1780_d_b5: f64 = ((s.db[622][5] * s.v[199]) * s.v[183]);let eq24_e1780_d_b6: f64 = ((s.db[622][6] * s.v[199]) * s.v[183]);let eq24_e1780_d_b7: f64 = ((s.db[622][7] * s.v[199]) * s.v[183]);let eq24_e1780_d_b8: f64 = ((s.db[622][8] * s.v[199]) * s.v[183]);let eq24_e1780_d_b9: f64 = ((s.db[622][9] * s.v[199]) * s.v[183]);let eq24_e1780_d_b10: f64 = ((s.db[622][10] * s.v[199]) * s.v[183]);let eq24_e1780_d_b11: f64 = ((s.db[622][11] * s.v[199]) * s.v[183]);let eq24_e1782: f64 = (eq24_e1780 * p.p2);let eq24_e1782_d_n0: f64 = (eq24_e1780_d_n0 * p.p2);let eq24_e1782_d_n1: f64 = (eq24_e1780_d_n1 * p.p2);let eq24_e1782_d_n2: f64 = (eq24_e1780_d_n2 * p.p2);let eq24_e1782_d_n3: f64 = (eq24_e1780_d_n3 * p.p2);let eq24_e1782_d_n4: f64 = (eq24_e1780_d_n4 * p.p2);let eq24_e1782_d_n5: f64 = (eq24_e1780_d_n5 * p.p2);let eq24_e1782_d_n6: f64 = (eq24_e1780_d_n6 * p.p2);let eq24_e1782_d_n7: f64 = (eq24_e1780_d_n7 * p.p2);let eq24_e1782_d_n8: f64 = (eq24_e1780_d_n8 * p.p2);let eq24_e1782_d_n9: f64 = (eq24_e1780_d_n9 * p.p2);let eq24_e1782_d_n10: f64 = (eq24_e1780_d_n10 * p.p2);let eq24_e1782_d_n11: f64 = (eq24_e1780_d_n11 * p.p2);let eq24_e1782_d_n12: f64 = (eq24_e1780_d_n12 * p.p2);let eq24_e1782_d_n13: f64 = (eq24_e1780_d_n13 * p.p2);let eq24_e1782_d_b0: f64 = (eq24_e1780_d_b0 * p.p2);let eq24_e1782_d_b1: f64 = (eq24_e1780_d_b1 * p.p2);let eq24_e1782_d_b2: f64 = (eq24_e1780_d_b2 * p.p2);let eq24_e1782_d_b3: f64 = (eq24_e1780_d_b3 * p.p2);let eq24_e1782_d_b4: f64 = (eq24_e1780_d_b4 * p.p2);let eq24_e1782_d_b5: f64 = (eq24_e1780_d_b5 * p.p2);let eq24_e1782_d_b6: f64 = (eq24_e1780_d_b6 * p.p2);let eq24_e1782_d_b7: f64 = (eq24_e1780_d_b7 * p.p2);let eq24_e1782_d_b8: f64 = (eq24_e1780_d_b8 * p.p2);let eq24_e1782_d_b9: f64 = (eq24_e1780_d_b9 * p.p2);let eq24_e1782_d_b10: f64 = (eq24_e1780_d_b10 * p.p2);let eq24_e1782_d_b11: f64 = (eq24_e1780_d_b11 * p.p2);let eq24_e1784: f64 = (eq24_e1782 * s.v[184]);let eq24_e1784_d_n0: f64 = (eq24_e1782_d_n0 * s.v[184]);let eq24_e1784_d_n1: f64 = (eq24_e1782_d_n1 * s.v[184]);let eq24_e1784_d_n2: f64 = (eq24_e1782_d_n2 * s.v[184]);let eq24_e1784_d_n3: f64 = (eq24_e1782_d_n3 * s.v[184]);let eq24_e1784_d_n4: f64 = (eq24_e1782_d_n4 * s.v[184]);let eq24_e1784_d_n5: f64 = (eq24_e1782_d_n5 * s.v[184]);let eq24_e1784_d_n6: f64 = (eq24_e1782_d_n6 * s.v[184]);let eq24_e1784_d_n7: f64 = (eq24_e1782_d_n7 * s.v[184]);let eq24_e1784_d_n8: f64 = (eq24_e1782_d_n8 * s.v[184]);let eq24_e1784_d_n9: f64 = (eq24_e1782_d_n9 * s.v[184]);let eq24_e1784_d_n10: f64 = (eq24_e1782_d_n10 * s.v[184]);let eq24_e1784_d_n11: f64 = (eq24_e1782_d_n11 * s.v[184]);let eq24_e1784_d_n12: f64 = (eq24_e1782_d_n12 * s.v[184]);let eq24_e1784_d_n13: f64 = (eq24_e1782_d_n13 * s.v[184]);
        let eq24_e1784_d_b0: f64 = (eq24_e1782_d_b0 * s.v[184]);let eq24_e1784_d_b1: f64 = (eq24_e1782_d_b1 * s.v[184]);let eq24_e1784_d_b2: f64 = (eq24_e1782_d_b2 * s.v[184]);let eq24_e1784_d_b3: f64 = (eq24_e1782_d_b3 * s.v[184]);let eq24_e1784_d_b4: f64 = (eq24_e1782_d_b4 * s.v[184]);let eq24_e1784_d_b5: f64 = (eq24_e1782_d_b5 * s.v[184]);let eq24_e1784_d_b6: f64 = (eq24_e1782_d_b6 * s.v[184]);let eq24_e1784_d_b7: f64 = (eq24_e1782_d_b7 * s.v[184]);let eq24_e1784_d_b8: f64 = (eq24_e1782_d_b8 * s.v[184]);let eq24_e1784_d_b9: f64 = (eq24_e1782_d_b9 * s.v[184]);let eq24_e1784_d_b10: f64 = (eq24_e1782_d_b10 * s.v[184]);let eq24_e1784_d_b11: f64 = (eq24_e1782_d_b11 * s.v[184]);let eq24_e1786: f64 = (eq24_e1784 * (nv12 - 0.0));let eq24_e1786_d_n0: f64 = (eq24_e1784_d_n0 * (nv12 - 0.0));let eq24_e1786_d_n1: f64 = (eq24_e1784_d_n1 * (nv12 - 0.0));let eq24_e1786_d_n2: f64 = (eq24_e1784_d_n2 * (nv12 - 0.0));let eq24_e1786_d_n3: f64 = (eq24_e1784_d_n3 * (nv12 - 0.0));let eq24_e1786_d_n4: f64 = (eq24_e1784_d_n4 * (nv12 - 0.0));let eq24_e1786_d_n5: f64 = (eq24_e1784_d_n5 * (nv12 - 0.0));let eq24_e1786_d_n6: f64 = (eq24_e1784_d_n6 * (nv12 - 0.0));let eq24_e1786_d_n7: f64 = (eq24_e1784_d_n7 * (nv12 - 0.0));let eq24_e1786_d_n8: f64 = (eq24_e1784_d_n8 * (nv12 - 0.0));let eq24_e1786_d_n9: f64 = (eq24_e1784_d_n9 * (nv12 - 0.0));let eq24_e1786_d_n10: f64 = (eq24_e1784_d_n10 * (nv12 - 0.0));let eq24_e1786_d_n11: f64 = (eq24_e1784_d_n11 * (nv12 - 0.0));let eq24_e1786_d_n12: f64 = ((eq24_e1784_d_n12 * (nv12 - 0.0)) + eq24_e1784);let eq24_e1786_d_n13: f64 = (eq24_e1784_d_n13 * (nv12 - 0.0));let eq24_e1786_d_b0: f64 = (eq24_e1784_d_b0 * (nv12 - 0.0));let eq24_e1786_d_b1: f64 = (eq24_e1784_d_b1 * (nv12 - 0.0));let eq24_e1786_d_b2: f64 = (eq24_e1784_d_b2 * (nv12 - 0.0));let eq24_e1786_d_b3: f64 = (eq24_e1784_d_b3 * (nv12 - 0.0));let eq24_e1786_d_b4: f64 = (eq24_e1784_d_b4 * (nv12 - 0.0));let eq24_e1786_d_b5: f64 = (eq24_e1784_d_b5 * (nv12 - 0.0));let eq24_e1786_d_b6: f64 = (eq24_e1784_d_b6 * (nv12 - 0.0));let eq24_e1786_d_b7: f64 = (eq24_e1784_d_b7 * (nv12 - 0.0));let eq24_e1786_d_b8: f64 = (eq24_e1784_d_b8 * (nv12 - 0.0));let eq24_e1786_d_b9: f64 = (eq24_e1784_d_b9 * (nv12 - 0.0));let eq24_e1786_d_b10: f64 = (eq24_e1784_d_b10 * (nv12 - 0.0));let eq24_e1786_d_b11: f64 = (eq24_e1784_d_b11 * (nv12 - 0.0));let eq24_e1787: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq24_e1786);
        (eq24_e1787, (eq24_e1786_d_n0 * ddt_scale), (eq24_e1786_d_n1 * ddt_scale), (eq24_e1786_d_n2 * ddt_scale), (eq24_e1786_d_n3 * ddt_scale), (eq24_e1786_d_n4 * ddt_scale), (eq24_e1786_d_n5 * ddt_scale), (eq24_e1786_d_n6 * ddt_scale), (eq24_e1786_d_n7 * ddt_scale), (eq24_e1786_d_n8 * ddt_scale), (eq24_e1786_d_n9 * ddt_scale), (eq24_e1786_d_n10 * ddt_scale), (eq24_e1786_d_n11 * ddt_scale), (eq24_e1786_d_n12 * ddt_scale), (eq24_e1786_d_n13 * ddt_scale), (eq24_e1786_d_b0 * ddt_scale), (eq24_e1786_d_b1 * ddt_scale), (eq24_e1786_d_b2 * ddt_scale), (eq24_e1786_d_b3 * ddt_scale), (eq24_e1786_d_b4 * ddt_scale), (eq24_e1786_d_b5 * ddt_scale), (eq24_e1786_d_b6 * ddt_scale), (eq24_e1786_d_b7 * ddt_scale), (eq24_e1786_d_b8 * ddt_scale), (eq24_e1786_d_b9 * ddt_scale), (eq24_e1786_d_b10 * ddt_scale), (eq24_e1786_d_b11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1789;let eq24_node_derivatives: [f64; 14] = [eq24_e1789_d_n0, eq24_e1789_d_n1, eq24_e1789_d_n2, eq24_e1789_d_n3, eq24_e1789_d_n4, eq24_e1789_d_n5, eq24_e1789_d_n6, eq24_e1789_d_n7, eq24_e1789_d_n8, eq24_e1789_d_n9, eq24_e1789_d_n10, eq24_e1789_d_n11, eq24_e1789_d_n12, eq24_e1789_d_n13];let eq24_branch_derivatives: [f64; 12] = [eq24_e1789_d_b0, eq24_e1789_d_b1, eq24_e1789_d_b2, eq24_e1789_d_b3, eq24_e1789_d_b4, eq24_e1789_d_b5, eq24_e1789_d_b6, eq24_e1789_d_b7, eq24_e1789_d_b8, eq24_e1789_d_b9, eq24_e1789_d_b10, eq24_e1789_d_b11];
        stamper.stamp_current_dense_local(
            Some(12),
            None,
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq26_e1819, eq26_e1819_d_n0, eq26_e1819_d_n1, eq26_e1819_d_n2, eq26_e1819_d_n3, eq26_e1819_d_n4, eq26_e1819_d_n5, eq26_e1819_d_n6, eq26_e1819_d_n7, eq26_e1819_d_n8, eq26_e1819_d_n9, eq26_e1819_d_n10, eq26_e1819_d_n11, eq26_e1819_d_n12, eq26_e1819_d_n13, eq26_e1819_d_b0, eq26_e1819_d_b1, eq26_e1819_d_b2, eq26_e1819_d_b3, eq26_e1819_d_b4, eq26_e1819_d_b5, eq26_e1819_d_b6, eq26_e1819_d_b7, eq26_e1819_d_b8, eq26_e1819_d_b9, eq26_e1819_d_b10, eq26_e1819_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq26_e1817: f64 = (s.v[628] * (nv13 - 0.0));let eq26_e1817_d_n0: f64 = (s.dn[628][0] * (nv13 - 0.0));let eq26_e1817_d_n1: f64 = (s.dn[628][1] * (nv13 - 0.0));let eq26_e1817_d_n2: f64 = (s.dn[628][2] * (nv13 - 0.0));let eq26_e1817_d_n3: f64 = (s.dn[628][3] * (nv13 - 0.0));let eq26_e1817_d_n4: f64 = (s.dn[628][4] * (nv13 - 0.0));let eq26_e1817_d_n5: f64 = (s.dn[628][5] * (nv13 - 0.0));let eq26_e1817_d_n6: f64 = (s.dn[628][6] * (nv13 - 0.0));let eq26_e1817_d_n7: f64 = (s.dn[628][7] * (nv13 - 0.0));let eq26_e1817_d_n8: f64 = (s.dn[628][8] * (nv13 - 0.0));let eq26_e1817_d_n9: f64 = (s.dn[628][9] * (nv13 - 0.0));let eq26_e1817_d_n10: f64 = (s.dn[628][10] * (nv13 - 0.0));let eq26_e1817_d_n11: f64 = (s.dn[628][11] * (nv13 - 0.0));let eq26_e1817_d_n12: f64 = (s.dn[628][12] * (nv13 - 0.0));let eq26_e1817_d_n13: f64 = ((s.dn[628][13] * (nv13 - 0.0)) + s.v[628]);let eq26_e1817_d_b0: f64 = (s.db[628][0] * (nv13 - 0.0));let eq26_e1817_d_b1: f64 = (s.db[628][1] * (nv13 - 0.0));let eq26_e1817_d_b2: f64 = (s.db[628][2] * (nv13 - 0.0));let eq26_e1817_d_b3: f64 = (s.db[628][3] * (nv13 - 0.0));let eq26_e1817_d_b4: f64 = (s.db[628][4] * (nv13 - 0.0));let eq26_e1817_d_b5: f64 = (s.db[628][5] * (nv13 - 0.0));let eq26_e1817_d_b6: f64 = (s.db[628][6] * (nv13 - 0.0));let eq26_e1817_d_b7: f64 = (s.db[628][7] * (nv13 - 0.0));let eq26_e1817_d_b8: f64 = (s.db[628][8] * (nv13 - 0.0));let eq26_e1817_d_b9: f64 = (s.db[628][9] * (nv13 - 0.0));let eq26_e1817_d_b10: f64 = (s.db[628][10] * (nv13 - 0.0));let eq26_e1817_d_b11: f64 = (s.db[628][11] * (nv13 - 0.0));
        (eq26_e1817, eq26_e1817_d_n0, eq26_e1817_d_n1, eq26_e1817_d_n2, eq26_e1817_d_n3, eq26_e1817_d_n4, eq26_e1817_d_n5, eq26_e1817_d_n6, eq26_e1817_d_n7, eq26_e1817_d_n8, eq26_e1817_d_n9, eq26_e1817_d_n10, eq26_e1817_d_n11, eq26_e1817_d_n12, eq26_e1817_d_n13, eq26_e1817_d_b0, eq26_e1817_d_b1, eq26_e1817_d_b2, eq26_e1817_d_b3, eq26_e1817_d_b4, eq26_e1817_d_b5, eq26_e1817_d_b6, eq26_e1817_d_b7, eq26_e1817_d_b8, eq26_e1817_d_b9, eq26_e1817_d_b10, eq26_e1817_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1819;let eq26_node_derivatives: [f64; 14] = [eq26_e1819_d_n0, eq26_e1819_d_n1, eq26_e1819_d_n2, eq26_e1819_d_n3, eq26_e1819_d_n4, eq26_e1819_d_n5, eq26_e1819_d_n6, eq26_e1819_d_n7, eq26_e1819_d_n8, eq26_e1819_d_n9, eq26_e1819_d_n10, eq26_e1819_d_n11, eq26_e1819_d_n12, eq26_e1819_d_n13];let eq26_branch_derivatives: [f64; 12] = [eq26_e1819_d_b0, eq26_e1819_d_b1, eq26_e1819_d_b2, eq26_e1819_d_b3, eq26_e1819_d_b4, eq26_e1819_d_b5, eq26_e1819_d_b6, eq26_e1819_d_b7, eq26_e1819_d_b8, eq26_e1819_d_b9, eq26_e1819_d_b10, eq26_e1819_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1846, eq27_e1846_d_n0, eq27_e1846_d_n1, eq27_e1846_d_n2, eq27_e1846_d_n3, eq27_e1846_d_n4, eq27_e1846_d_n5, eq27_e1846_d_n6, eq27_e1846_d_n7, eq27_e1846_d_n8, eq27_e1846_d_n9, eq27_e1846_d_n10, eq27_e1846_d_n11, eq27_e1846_d_n12, eq27_e1846_d_n13, eq27_e1846_d_b0, eq27_e1846_d_b1, eq27_e1846_d_b2, eq27_e1846_d_b3, eq27_e1846_d_b4, eq27_e1846_d_b5, eq27_e1846_d_b6, eq27_e1846_d_b7, eq27_e1846_d_b8, eq27_e1846_d_b9, eq27_e1846_d_b10, eq27_e1846_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq27_e1830: f64 = (1.0 + s.v[211]);let eq27_e1832: f64 = (eq27_e1830 * s.v[622]);let eq27_e1832_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq27_e1830 * s.dn[622][0]));let eq27_e1832_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq27_e1830 * s.dn[622][1]));let eq27_e1832_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq27_e1830 * s.dn[622][2]));let eq27_e1832_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq27_e1830 * s.dn[622][3]));let eq27_e1832_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq27_e1830 * s.dn[622][4]));let eq27_e1832_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq27_e1830 * s.dn[622][5]));let eq27_e1832_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq27_e1830 * s.dn[622][6]));let eq27_e1832_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq27_e1830 * s.dn[622][7]));let eq27_e1832_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq27_e1830 * s.dn[622][8]));let eq27_e1832_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq27_e1830 * s.dn[622][9]));let eq27_e1832_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq27_e1830 * s.dn[622][10]));let eq27_e1832_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq27_e1830 * s.dn[622][11]));let eq27_e1832_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq27_e1830 * s.dn[622][12]));let eq27_e1832_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq27_e1830 * s.dn[622][13]));let eq27_e1832_d_b0: f64 = ((s.db[211][0] * s.v[622]) + (eq27_e1830 * s.db[622][0]));let eq27_e1832_d_b1: f64 = ((s.db[211][1] * s.v[622]) + (eq27_e1830 * s.db[622][1]));let eq27_e1832_d_b2: f64 = ((s.db[211][2] * s.v[622]) + (eq27_e1830 * s.db[622][2]));let eq27_e1832_d_b3: f64 = ((s.db[211][3] * s.v[622]) + (eq27_e1830 * s.db[622][3]));let eq27_e1832_d_b4: f64 = ((s.db[211][4] * s.v[622]) + (eq27_e1830 * s.db[622][4]));let eq27_e1832_d_b5: f64 = ((s.db[211][5] * s.v[622]) + (eq27_e1830 * s.db[622][5]));let eq27_e1832_d_b6: f64 = ((s.db[211][6] * s.v[622]) + (eq27_e1830 * s.db[622][6]));let eq27_e1832_d_b7: f64 = ((s.db[211][7] * s.v[622]) + (eq27_e1830 * s.db[622][7]));let eq27_e1832_d_b8: f64 = ((s.db[211][8] * s.v[622]) + (eq27_e1830 * s.db[622][8]));let eq27_e1832_d_b9: f64 = ((s.db[211][9] * s.v[622]) + (eq27_e1830 * s.db[622][9]));let eq27_e1832_d_b10: f64 = ((s.db[211][10] * s.v[622]) + (eq27_e1830 * s.db[622][10]));let eq27_e1832_d_b11: f64 = ((s.db[211][11] * s.v[622]) + (eq27_e1830 * s.db[622][11]));let eq27_e1834: f64 = (eq27_e1832 * s.v[199]);let eq27_e1834_d_n0: f64 = (eq27_e1832_d_n0 * s.v[199]);let eq27_e1834_d_n1: f64 = (eq27_e1832_d_n1 * s.v[199]);let eq27_e1834_d_n2: f64 = (eq27_e1832_d_n2 * s.v[199]);let eq27_e1834_d_n3: f64 = (eq27_e1832_d_n3 * s.v[199]);let eq27_e1834_d_n4: f64 = (eq27_e1832_d_n4 * s.v[199]);let eq27_e1834_d_n5: f64 = (eq27_e1832_d_n5 * s.v[199]);let eq27_e1834_d_n6: f64 = (eq27_e1832_d_n6 * s.v[199]);let eq27_e1834_d_n7: f64 = (eq27_e1832_d_n7 * s.v[199]);let eq27_e1834_d_n8: f64 = (eq27_e1832_d_n8 * s.v[199]);let eq27_e1834_d_n9: f64 = (eq27_e1832_d_n9 * s.v[199]);let eq27_e1834_d_n10: f64 = (eq27_e1832_d_n10 * s.v[199]);let eq27_e1834_d_n11: f64 = (eq27_e1832_d_n11 * s.v[199]);let eq27_e1834_d_n12: f64 = (eq27_e1832_d_n12 * s.v[199]);let eq27_e1834_d_n13: f64 = (eq27_e1832_d_n13 * s.v[199]);let eq27_e1834_d_b0: f64 = (eq27_e1832_d_b0 * s.v[199]);let eq27_e1834_d_b1: f64 = (eq27_e1832_d_b1 * s.v[199]);let eq27_e1834_d_b2: f64 = (eq27_e1832_d_b2 * s.v[199]);let eq27_e1834_d_b3: f64 = (eq27_e1832_d_b3 * s.v[199]);let eq27_e1834_d_b4: f64 = (eq27_e1832_d_b4 * s.v[199]);let eq27_e1834_d_b5: f64 = (eq27_e1832_d_b5 * s.v[199]);let eq27_e1834_d_b6: f64 = (eq27_e1832_d_b6 * s.v[199]);let eq27_e1834_d_b7: f64 = (eq27_e1832_d_b7 * s.v[199]);let eq27_e1834_d_b8: f64 = (eq27_e1832_d_b8 * s.v[199]);let eq27_e1834_d_b9: f64 = (eq27_e1832_d_b9 * s.v[199]);let eq27_e1834_d_b10: f64 = (eq27_e1832_d_b10 * s.v[199]);let eq27_e1834_d_b11: f64 = (eq27_e1832_d_b11 * s.v[199]);let eq27_e1836: f64 = (eq27_e1834 * s.v[183]);let eq27_e1836_d_n0: f64 = (eq27_e1834_d_n0 * s.v[183]);let eq27_e1836_d_n1: f64 = (eq27_e1834_d_n1 * s.v[183]);let eq27_e1836_d_n2: f64 = (eq27_e1834_d_n2 * s.v[183]);
        let eq27_e1836_d_n3: f64 = (eq27_e1834_d_n3 * s.v[183]);let eq27_e1836_d_n4: f64 = (eq27_e1834_d_n4 * s.v[183]);let eq27_e1836_d_n5: f64 = (eq27_e1834_d_n5 * s.v[183]);let eq27_e1836_d_n6: f64 = (eq27_e1834_d_n6 * s.v[183]);let eq27_e1836_d_n7: f64 = (eq27_e1834_d_n7 * s.v[183]);let eq27_e1836_d_n8: f64 = (eq27_e1834_d_n8 * s.v[183]);let eq27_e1836_d_n9: f64 = (eq27_e1834_d_n9 * s.v[183]);let eq27_e1836_d_n10: f64 = (eq27_e1834_d_n10 * s.v[183]);let eq27_e1836_d_n11: f64 = (eq27_e1834_d_n11 * s.v[183]);let eq27_e1836_d_n12: f64 = (eq27_e1834_d_n12 * s.v[183]);let eq27_e1836_d_n13: f64 = (eq27_e1834_d_n13 * s.v[183]);let eq27_e1836_d_b0: f64 = (eq27_e1834_d_b0 * s.v[183]);let eq27_e1836_d_b1: f64 = (eq27_e1834_d_b1 * s.v[183]);let eq27_e1836_d_b2: f64 = (eq27_e1834_d_b2 * s.v[183]);let eq27_e1836_d_b3: f64 = (eq27_e1834_d_b3 * s.v[183]);let eq27_e1836_d_b4: f64 = (eq27_e1834_d_b4 * s.v[183]);let eq27_e1836_d_b5: f64 = (eq27_e1834_d_b5 * s.v[183]);let eq27_e1836_d_b6: f64 = (eq27_e1834_d_b6 * s.v[183]);let eq27_e1836_d_b7: f64 = (eq27_e1834_d_b7 * s.v[183]);let eq27_e1836_d_b8: f64 = (eq27_e1834_d_b8 * s.v[183]);let eq27_e1836_d_b9: f64 = (eq27_e1834_d_b9 * s.v[183]);let eq27_e1836_d_b10: f64 = (eq27_e1834_d_b10 * s.v[183]);let eq27_e1836_d_b11: f64 = (eq27_e1834_d_b11 * s.v[183]);let eq27_e1838: f64 = (eq27_e1836 * p.p2);let eq27_e1838_d_n0: f64 = (eq27_e1836_d_n0 * p.p2);let eq27_e1838_d_n1: f64 = (eq27_e1836_d_n1 * p.p2);let eq27_e1838_d_n2: f64 = (eq27_e1836_d_n2 * p.p2);let eq27_e1838_d_n3: f64 = (eq27_e1836_d_n3 * p.p2);let eq27_e1838_d_n4: f64 = (eq27_e1836_d_n4 * p.p2);let eq27_e1838_d_n5: f64 = (eq27_e1836_d_n5 * p.p2);let eq27_e1838_d_n6: f64 = (eq27_e1836_d_n6 * p.p2);let eq27_e1838_d_n7: f64 = (eq27_e1836_d_n7 * p.p2);let eq27_e1838_d_n8: f64 = (eq27_e1836_d_n8 * p.p2);let eq27_e1838_d_n9: f64 = (eq27_e1836_d_n9 * p.p2);let eq27_e1838_d_n10: f64 = (eq27_e1836_d_n10 * p.p2);let eq27_e1838_d_n11: f64 = (eq27_e1836_d_n11 * p.p2);let eq27_e1838_d_n12: f64 = (eq27_e1836_d_n12 * p.p2);let eq27_e1838_d_n13: f64 = (eq27_e1836_d_n13 * p.p2);let eq27_e1838_d_b0: f64 = (eq27_e1836_d_b0 * p.p2);let eq27_e1838_d_b1: f64 = (eq27_e1836_d_b1 * p.p2);let eq27_e1838_d_b2: f64 = (eq27_e1836_d_b2 * p.p2);let eq27_e1838_d_b3: f64 = (eq27_e1836_d_b3 * p.p2);let eq27_e1838_d_b4: f64 = (eq27_e1836_d_b4 * p.p2);let eq27_e1838_d_b5: f64 = (eq27_e1836_d_b5 * p.p2);let eq27_e1838_d_b6: f64 = (eq27_e1836_d_b6 * p.p2);let eq27_e1838_d_b7: f64 = (eq27_e1836_d_b7 * p.p2);let eq27_e1838_d_b8: f64 = (eq27_e1836_d_b8 * p.p2);let eq27_e1838_d_b9: f64 = (eq27_e1836_d_b9 * p.p2);let eq27_e1838_d_b10: f64 = (eq27_e1836_d_b10 * p.p2);let eq27_e1838_d_b11: f64 = (eq27_e1836_d_b11 * p.p2);let eq27_e1840: f64 = (eq27_e1838 * s.v[184]);let eq27_e1840_d_n0: f64 = (eq27_e1838_d_n0 * s.v[184]);let eq27_e1840_d_n1: f64 = (eq27_e1838_d_n1 * s.v[184]);let eq27_e1840_d_n2: f64 = (eq27_e1838_d_n2 * s.v[184]);let eq27_e1840_d_n3: f64 = (eq27_e1838_d_n3 * s.v[184]);let eq27_e1840_d_n4: f64 = (eq27_e1838_d_n4 * s.v[184]);let eq27_e1840_d_n5: f64 = (eq27_e1838_d_n5 * s.v[184]);let eq27_e1840_d_n6: f64 = (eq27_e1838_d_n6 * s.v[184]);let eq27_e1840_d_n7: f64 = (eq27_e1838_d_n7 * s.v[184]);let eq27_e1840_d_n8: f64 = (eq27_e1838_d_n8 * s.v[184]);let eq27_e1840_d_n9: f64 = (eq27_e1838_d_n9 * s.v[184]);let eq27_e1840_d_n10: f64 = (eq27_e1838_d_n10 * s.v[184]);let eq27_e1840_d_n11: f64 = (eq27_e1838_d_n11 * s.v[184]);let eq27_e1840_d_n12: f64 = (eq27_e1838_d_n12 * s.v[184]);let eq27_e1840_d_n13: f64 = (eq27_e1838_d_n13 * s.v[184]);let eq27_e1840_d_b0: f64 = (eq27_e1838_d_b0 * s.v[184]);let eq27_e1840_d_b1: f64 = (eq27_e1838_d_b1 * s.v[184]);let eq27_e1840_d_b2: f64 = (eq27_e1838_d_b2 * s.v[184]);let eq27_e1840_d_b3: f64 = (eq27_e1838_d_b3 * s.v[184]);let eq27_e1840_d_b4: f64 = (eq27_e1838_d_b4 * s.v[184]);let eq27_e1840_d_b5: f64 = (eq27_e1838_d_b5 * s.v[184]);let eq27_e1840_d_b6: f64 = (eq27_e1838_d_b6 * s.v[184]);let eq27_e1840_d_b7: f64 = (eq27_e1838_d_b7 * s.v[184]);let eq27_e1840_d_b8: f64 = (eq27_e1838_d_b8 * s.v[184]);
        let eq27_e1840_d_b9: f64 = (eq27_e1838_d_b9 * s.v[184]);let eq27_e1840_d_b10: f64 = (eq27_e1838_d_b10 * s.v[184]);let eq27_e1840_d_b11: f64 = (eq27_e1838_d_b11 * s.v[184]);let eq27_e1842: f64 = (eq27_e1840 * (nv12 - 0.0));let eq27_e1842_d_n0: f64 = (eq27_e1840_d_n0 * (nv12 - 0.0));let eq27_e1842_d_n1: f64 = (eq27_e1840_d_n1 * (nv12 - 0.0));let eq27_e1842_d_n2: f64 = (eq27_e1840_d_n2 * (nv12 - 0.0));let eq27_e1842_d_n3: f64 = (eq27_e1840_d_n3 * (nv12 - 0.0));let eq27_e1842_d_n4: f64 = (eq27_e1840_d_n4 * (nv12 - 0.0));let eq27_e1842_d_n5: f64 = (eq27_e1840_d_n5 * (nv12 - 0.0));let eq27_e1842_d_n6: f64 = (eq27_e1840_d_n6 * (nv12 - 0.0));let eq27_e1842_d_n7: f64 = (eq27_e1840_d_n7 * (nv12 - 0.0));let eq27_e1842_d_n8: f64 = (eq27_e1840_d_n8 * (nv12 - 0.0));let eq27_e1842_d_n9: f64 = (eq27_e1840_d_n9 * (nv12 - 0.0));let eq27_e1842_d_n10: f64 = (eq27_e1840_d_n10 * (nv12 - 0.0));let eq27_e1842_d_n11: f64 = (eq27_e1840_d_n11 * (nv12 - 0.0));let eq27_e1842_d_n12: f64 = ((eq27_e1840_d_n12 * (nv12 - 0.0)) + eq27_e1840);let eq27_e1842_d_n13: f64 = (eq27_e1840_d_n13 * (nv12 - 0.0));let eq27_e1842_d_b0: f64 = (eq27_e1840_d_b0 * (nv12 - 0.0));let eq27_e1842_d_b1: f64 = (eq27_e1840_d_b1 * (nv12 - 0.0));let eq27_e1842_d_b2: f64 = (eq27_e1840_d_b2 * (nv12 - 0.0));let eq27_e1842_d_b3: f64 = (eq27_e1840_d_b3 * (nv12 - 0.0));let eq27_e1842_d_b4: f64 = (eq27_e1840_d_b4 * (nv12 - 0.0));let eq27_e1842_d_b5: f64 = (eq27_e1840_d_b5 * (nv12 - 0.0));let eq27_e1842_d_b6: f64 = (eq27_e1840_d_b6 * (nv12 - 0.0));let eq27_e1842_d_b7: f64 = (eq27_e1840_d_b7 * (nv12 - 0.0));let eq27_e1842_d_b8: f64 = (eq27_e1840_d_b8 * (nv12 - 0.0));let eq27_e1842_d_b9: f64 = (eq27_e1840_d_b9 * (nv12 - 0.0));let eq27_e1842_d_b10: f64 = (eq27_e1840_d_b10 * (nv12 - 0.0));let eq27_e1842_d_b11: f64 = (eq27_e1840_d_b11 * (nv12 - 0.0));let eq27_e1843: f64 = (0.5 * eq27_e1842);let eq27_e1843_d_n0: f64 = (0.5 * eq27_e1842_d_n0);let eq27_e1843_d_n1: f64 = (0.5 * eq27_e1842_d_n1);let eq27_e1843_d_n2: f64 = (0.5 * eq27_e1842_d_n2);let eq27_e1843_d_n3: f64 = (0.5 * eq27_e1842_d_n3);let eq27_e1843_d_n4: f64 = (0.5 * eq27_e1842_d_n4);let eq27_e1843_d_n5: f64 = (0.5 * eq27_e1842_d_n5);let eq27_e1843_d_n6: f64 = (0.5 * eq27_e1842_d_n6);let eq27_e1843_d_n7: f64 = (0.5 * eq27_e1842_d_n7);let eq27_e1843_d_n8: f64 = (0.5 * eq27_e1842_d_n8);let eq27_e1843_d_n9: f64 = (0.5 * eq27_e1842_d_n9);let eq27_e1843_d_n10: f64 = (0.5 * eq27_e1842_d_n10);let eq27_e1843_d_n11: f64 = (0.5 * eq27_e1842_d_n11);let eq27_e1843_d_n12: f64 = (0.5 * eq27_e1842_d_n12);let eq27_e1843_d_n13: f64 = (0.5 * eq27_e1842_d_n13);let eq27_e1843_d_b0: f64 = (0.5 * eq27_e1842_d_b0);let eq27_e1843_d_b1: f64 = (0.5 * eq27_e1842_d_b1);let eq27_e1843_d_b2: f64 = (0.5 * eq27_e1842_d_b2);let eq27_e1843_d_b3: f64 = (0.5 * eq27_e1842_d_b3);let eq27_e1843_d_b4: f64 = (0.5 * eq27_e1842_d_b4);let eq27_e1843_d_b5: f64 = (0.5 * eq27_e1842_d_b5);let eq27_e1843_d_b6: f64 = (0.5 * eq27_e1842_d_b6);let eq27_e1843_d_b7: f64 = (0.5 * eq27_e1842_d_b7);let eq27_e1843_d_b8: f64 = (0.5 * eq27_e1842_d_b8);let eq27_e1843_d_b9: f64 = (0.5 * eq27_e1842_d_b9);let eq27_e1843_d_b10: f64 = (0.5 * eq27_e1842_d_b10);let eq27_e1843_d_b11: f64 = (0.5 * eq27_e1842_d_b11);let eq27_e1844: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq27_e1843);
        (eq27_e1844, (eq27_e1843_d_n0 * ddt_scale), (eq27_e1843_d_n1 * ddt_scale), (eq27_e1843_d_n2 * ddt_scale), (eq27_e1843_d_n3 * ddt_scale), (eq27_e1843_d_n4 * ddt_scale), (eq27_e1843_d_n5 * ddt_scale), (eq27_e1843_d_n6 * ddt_scale), (eq27_e1843_d_n7 * ddt_scale), (eq27_e1843_d_n8 * ddt_scale), (eq27_e1843_d_n9 * ddt_scale), (eq27_e1843_d_n10 * ddt_scale), (eq27_e1843_d_n11 * ddt_scale), (eq27_e1843_d_n12 * ddt_scale), (eq27_e1843_d_n13 * ddt_scale), (eq27_e1843_d_b0 * ddt_scale), (eq27_e1843_d_b1 * ddt_scale), (eq27_e1843_d_b2 * ddt_scale), (eq27_e1843_d_b3 * ddt_scale), (eq27_e1843_d_b4 * ddt_scale), (eq27_e1843_d_b5 * ddt_scale), (eq27_e1843_d_b6 * ddt_scale), (eq27_e1843_d_b7 * ddt_scale), (eq27_e1843_d_b8 * ddt_scale), (eq27_e1843_d_b9 * ddt_scale), (eq27_e1843_d_b10 * ddt_scale), (eq27_e1843_d_b11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1846;let eq27_node_derivatives: [f64; 14] = [eq27_e1846_d_n0, eq27_e1846_d_n1, eq27_e1846_d_n2, eq27_e1846_d_n3, eq27_e1846_d_n4, eq27_e1846_d_n5, eq27_e1846_d_n6, eq27_e1846_d_n7, eq27_e1846_d_n8, eq27_e1846_d_n9, eq27_e1846_d_n10, eq27_e1846_d_n11, eq27_e1846_d_n12, eq27_e1846_d_n13];let eq27_branch_derivatives: [f64; 12] = [eq27_e1846_d_b0, eq27_e1846_d_b1, eq27_e1846_d_b2, eq27_e1846_d_b3, eq27_e1846_d_b4, eq27_e1846_d_b5, eq27_e1846_d_b6, eq27_e1846_d_b7, eq27_e1846_d_b8, eq27_e1846_d_b9, eq27_e1846_d_b10, eq27_e1846_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq28_e1873, eq28_e1873_d_n0, eq28_e1873_d_n1, eq28_e1873_d_n2, eq28_e1873_d_n3, eq28_e1873_d_n4, eq28_e1873_d_n5, eq28_e1873_d_n6, eq28_e1873_d_n7, eq28_e1873_d_n8, eq28_e1873_d_n9, eq28_e1873_d_n10, eq28_e1873_d_n11, eq28_e1873_d_n12, eq28_e1873_d_n13, eq28_e1873_d_b0, eq28_e1873_d_b1, eq28_e1873_d_b2, eq28_e1873_d_b3, eq28_e1873_d_b4, eq28_e1873_d_b5, eq28_e1873_d_b6, eq28_e1873_d_b7, eq28_e1873_d_b8, eq28_e1873_d_b9, eq28_e1873_d_b10, eq28_e1873_d_b11,) = {
    if ((!s.b[1620]) && (s.b[1965] && (!s.b[1964]))) {
        let eq28_e1857: f64 = (1.0 - s.v[211]);let eq28_e1859: f64 = (eq28_e1857 * s.v[622]);let eq28_e1859_d_n0: f64 = (((-s.dn[211][0]) * s.v[622]) + (eq28_e1857 * s.dn[622][0]));let eq28_e1859_d_n1: f64 = (((-s.dn[211][1]) * s.v[622]) + (eq28_e1857 * s.dn[622][1]));let eq28_e1859_d_n2: f64 = (((-s.dn[211][2]) * s.v[622]) + (eq28_e1857 * s.dn[622][2]));let eq28_e1859_d_n3: f64 = (((-s.dn[211][3]) * s.v[622]) + (eq28_e1857 * s.dn[622][3]));let eq28_e1859_d_n4: f64 = (((-s.dn[211][4]) * s.v[622]) + (eq28_e1857 * s.dn[622][4]));let eq28_e1859_d_n5: f64 = (((-s.dn[211][5]) * s.v[622]) + (eq28_e1857 * s.dn[622][5]));let eq28_e1859_d_n6: f64 = (((-s.dn[211][6]) * s.v[622]) + (eq28_e1857 * s.dn[622][6]));let eq28_e1859_d_n7: f64 = (((-s.dn[211][7]) * s.v[622]) + (eq28_e1857 * s.dn[622][7]));let eq28_e1859_d_n8: f64 = (((-s.dn[211][8]) * s.v[622]) + (eq28_e1857 * s.dn[622][8]));let eq28_e1859_d_n9: f64 = (((-s.dn[211][9]) * s.v[622]) + (eq28_e1857 * s.dn[622][9]));let eq28_e1859_d_n10: f64 = (((-s.dn[211][10]) * s.v[622]) + (eq28_e1857 * s.dn[622][10]));let eq28_e1859_d_n11: f64 = (((-s.dn[211][11]) * s.v[622]) + (eq28_e1857 * s.dn[622][11]));let eq28_e1859_d_n12: f64 = (((-s.dn[211][12]) * s.v[622]) + (eq28_e1857 * s.dn[622][12]));let eq28_e1859_d_n13: f64 = (((-s.dn[211][13]) * s.v[622]) + (eq28_e1857 * s.dn[622][13]));let eq28_e1859_d_b0: f64 = (((-s.db[211][0]) * s.v[622]) + (eq28_e1857 * s.db[622][0]));let eq28_e1859_d_b1: f64 = (((-s.db[211][1]) * s.v[622]) + (eq28_e1857 * s.db[622][1]));let eq28_e1859_d_b2: f64 = (((-s.db[211][2]) * s.v[622]) + (eq28_e1857 * s.db[622][2]));let eq28_e1859_d_b3: f64 = (((-s.db[211][3]) * s.v[622]) + (eq28_e1857 * s.db[622][3]));let eq28_e1859_d_b4: f64 = (((-s.db[211][4]) * s.v[622]) + (eq28_e1857 * s.db[622][4]));let eq28_e1859_d_b5: f64 = (((-s.db[211][5]) * s.v[622]) + (eq28_e1857 * s.db[622][5]));let eq28_e1859_d_b6: f64 = (((-s.db[211][6]) * s.v[622]) + (eq28_e1857 * s.db[622][6]));let eq28_e1859_d_b7: f64 = (((-s.db[211][7]) * s.v[622]) + (eq28_e1857 * s.db[622][7]));let eq28_e1859_d_b8: f64 = (((-s.db[211][8]) * s.v[622]) + (eq28_e1857 * s.db[622][8]));let eq28_e1859_d_b9: f64 = (((-s.db[211][9]) * s.v[622]) + (eq28_e1857 * s.db[622][9]));let eq28_e1859_d_b10: f64 = (((-s.db[211][10]) * s.v[622]) + (eq28_e1857 * s.db[622][10]));let eq28_e1859_d_b11: f64 = (((-s.db[211][11]) * s.v[622]) + (eq28_e1857 * s.db[622][11]));let eq28_e1861: f64 = (eq28_e1859 * s.v[199]);let eq28_e1861_d_n0: f64 = (eq28_e1859_d_n0 * s.v[199]);let eq28_e1861_d_n1: f64 = (eq28_e1859_d_n1 * s.v[199]);let eq28_e1861_d_n2: f64 = (eq28_e1859_d_n2 * s.v[199]);let eq28_e1861_d_n3: f64 = (eq28_e1859_d_n3 * s.v[199]);let eq28_e1861_d_n4: f64 = (eq28_e1859_d_n4 * s.v[199]);let eq28_e1861_d_n5: f64 = (eq28_e1859_d_n5 * s.v[199]);let eq28_e1861_d_n6: f64 = (eq28_e1859_d_n6 * s.v[199]);let eq28_e1861_d_n7: f64 = (eq28_e1859_d_n7 * s.v[199]);let eq28_e1861_d_n8: f64 = (eq28_e1859_d_n8 * s.v[199]);let eq28_e1861_d_n9: f64 = (eq28_e1859_d_n9 * s.v[199]);let eq28_e1861_d_n10: f64 = (eq28_e1859_d_n10 * s.v[199]);let eq28_e1861_d_n11: f64 = (eq28_e1859_d_n11 * s.v[199]);let eq28_e1861_d_n12: f64 = (eq28_e1859_d_n12 * s.v[199]);let eq28_e1861_d_n13: f64 = (eq28_e1859_d_n13 * s.v[199]);let eq28_e1861_d_b0: f64 = (eq28_e1859_d_b0 * s.v[199]);let eq28_e1861_d_b1: f64 = (eq28_e1859_d_b1 * s.v[199]);let eq28_e1861_d_b2: f64 = (eq28_e1859_d_b2 * s.v[199]);let eq28_e1861_d_b3: f64 = (eq28_e1859_d_b3 * s.v[199]);let eq28_e1861_d_b4: f64 = (eq28_e1859_d_b4 * s.v[199]);let eq28_e1861_d_b5: f64 = (eq28_e1859_d_b5 * s.v[199]);let eq28_e1861_d_b6: f64 = (eq28_e1859_d_b6 * s.v[199]);let eq28_e1861_d_b7: f64 = (eq28_e1859_d_b7 * s.v[199]);let eq28_e1861_d_b8: f64 = (eq28_e1859_d_b8 * s.v[199]);let eq28_e1861_d_b9: f64 = (eq28_e1859_d_b9 * s.v[199]);let eq28_e1861_d_b10: f64 = (eq28_e1859_d_b10 * s.v[199]);let eq28_e1861_d_b11: f64 = (eq28_e1859_d_b11 * s.v[199]);let eq28_e1863: f64 = (eq28_e1861 * s.v[183]);let eq28_e1863_d_n0: f64 = (eq28_e1861_d_n0 * s.v[183]);let eq28_e1863_d_n1: f64 = (eq28_e1861_d_n1 * s.v[183]);
        let eq28_e1863_d_n2: f64 = (eq28_e1861_d_n2 * s.v[183]);let eq28_e1863_d_n3: f64 = (eq28_e1861_d_n3 * s.v[183]);let eq28_e1863_d_n4: f64 = (eq28_e1861_d_n4 * s.v[183]);let eq28_e1863_d_n5: f64 = (eq28_e1861_d_n5 * s.v[183]);let eq28_e1863_d_n6: f64 = (eq28_e1861_d_n6 * s.v[183]);let eq28_e1863_d_n7: f64 = (eq28_e1861_d_n7 * s.v[183]);let eq28_e1863_d_n8: f64 = (eq28_e1861_d_n8 * s.v[183]);let eq28_e1863_d_n9: f64 = (eq28_e1861_d_n9 * s.v[183]);let eq28_e1863_d_n10: f64 = (eq28_e1861_d_n10 * s.v[183]);let eq28_e1863_d_n11: f64 = (eq28_e1861_d_n11 * s.v[183]);let eq28_e1863_d_n12: f64 = (eq28_e1861_d_n12 * s.v[183]);let eq28_e1863_d_n13: f64 = (eq28_e1861_d_n13 * s.v[183]);let eq28_e1863_d_b0: f64 = (eq28_e1861_d_b0 * s.v[183]);let eq28_e1863_d_b1: f64 = (eq28_e1861_d_b1 * s.v[183]);let eq28_e1863_d_b2: f64 = (eq28_e1861_d_b2 * s.v[183]);let eq28_e1863_d_b3: f64 = (eq28_e1861_d_b3 * s.v[183]);let eq28_e1863_d_b4: f64 = (eq28_e1861_d_b4 * s.v[183]);let eq28_e1863_d_b5: f64 = (eq28_e1861_d_b5 * s.v[183]);let eq28_e1863_d_b6: f64 = (eq28_e1861_d_b6 * s.v[183]);let eq28_e1863_d_b7: f64 = (eq28_e1861_d_b7 * s.v[183]);let eq28_e1863_d_b8: f64 = (eq28_e1861_d_b8 * s.v[183]);let eq28_e1863_d_b9: f64 = (eq28_e1861_d_b9 * s.v[183]);let eq28_e1863_d_b10: f64 = (eq28_e1861_d_b10 * s.v[183]);let eq28_e1863_d_b11: f64 = (eq28_e1861_d_b11 * s.v[183]);let eq28_e1865: f64 = (eq28_e1863 * p.p2);let eq28_e1865_d_n0: f64 = (eq28_e1863_d_n0 * p.p2);let eq28_e1865_d_n1: f64 = (eq28_e1863_d_n1 * p.p2);let eq28_e1865_d_n2: f64 = (eq28_e1863_d_n2 * p.p2);let eq28_e1865_d_n3: f64 = (eq28_e1863_d_n3 * p.p2);let eq28_e1865_d_n4: f64 = (eq28_e1863_d_n4 * p.p2);let eq28_e1865_d_n5: f64 = (eq28_e1863_d_n5 * p.p2);let eq28_e1865_d_n6: f64 = (eq28_e1863_d_n6 * p.p2);let eq28_e1865_d_n7: f64 = (eq28_e1863_d_n7 * p.p2);let eq28_e1865_d_n8: f64 = (eq28_e1863_d_n8 * p.p2);let eq28_e1865_d_n9: f64 = (eq28_e1863_d_n9 * p.p2);let eq28_e1865_d_n10: f64 = (eq28_e1863_d_n10 * p.p2);let eq28_e1865_d_n11: f64 = (eq28_e1863_d_n11 * p.p2);let eq28_e1865_d_n12: f64 = (eq28_e1863_d_n12 * p.p2);let eq28_e1865_d_n13: f64 = (eq28_e1863_d_n13 * p.p2);let eq28_e1865_d_b0: f64 = (eq28_e1863_d_b0 * p.p2);let eq28_e1865_d_b1: f64 = (eq28_e1863_d_b1 * p.p2);let eq28_e1865_d_b2: f64 = (eq28_e1863_d_b2 * p.p2);let eq28_e1865_d_b3: f64 = (eq28_e1863_d_b3 * p.p2);let eq28_e1865_d_b4: f64 = (eq28_e1863_d_b4 * p.p2);let eq28_e1865_d_b5: f64 = (eq28_e1863_d_b5 * p.p2);let eq28_e1865_d_b6: f64 = (eq28_e1863_d_b6 * p.p2);let eq28_e1865_d_b7: f64 = (eq28_e1863_d_b7 * p.p2);let eq28_e1865_d_b8: f64 = (eq28_e1863_d_b8 * p.p2);let eq28_e1865_d_b9: f64 = (eq28_e1863_d_b9 * p.p2);let eq28_e1865_d_b10: f64 = (eq28_e1863_d_b10 * p.p2);let eq28_e1865_d_b11: f64 = (eq28_e1863_d_b11 * p.p2);let eq28_e1867: f64 = (eq28_e1865 * s.v[184]);let eq28_e1867_d_n0: f64 = (eq28_e1865_d_n0 * s.v[184]);let eq28_e1867_d_n1: f64 = (eq28_e1865_d_n1 * s.v[184]);let eq28_e1867_d_n2: f64 = (eq28_e1865_d_n2 * s.v[184]);let eq28_e1867_d_n3: f64 = (eq28_e1865_d_n3 * s.v[184]);let eq28_e1867_d_n4: f64 = (eq28_e1865_d_n4 * s.v[184]);let eq28_e1867_d_n5: f64 = (eq28_e1865_d_n5 * s.v[184]);let eq28_e1867_d_n6: f64 = (eq28_e1865_d_n6 * s.v[184]);let eq28_e1867_d_n7: f64 = (eq28_e1865_d_n7 * s.v[184]);let eq28_e1867_d_n8: f64 = (eq28_e1865_d_n8 * s.v[184]);let eq28_e1867_d_n9: f64 = (eq28_e1865_d_n9 * s.v[184]);let eq28_e1867_d_n10: f64 = (eq28_e1865_d_n10 * s.v[184]);let eq28_e1867_d_n11: f64 = (eq28_e1865_d_n11 * s.v[184]);let eq28_e1867_d_n12: f64 = (eq28_e1865_d_n12 * s.v[184]);let eq28_e1867_d_n13: f64 = (eq28_e1865_d_n13 * s.v[184]);let eq28_e1867_d_b0: f64 = (eq28_e1865_d_b0 * s.v[184]);let eq28_e1867_d_b1: f64 = (eq28_e1865_d_b1 * s.v[184]);let eq28_e1867_d_b2: f64 = (eq28_e1865_d_b2 * s.v[184]);let eq28_e1867_d_b3: f64 = (eq28_e1865_d_b3 * s.v[184]);let eq28_e1867_d_b4: f64 = (eq28_e1865_d_b4 * s.v[184]);let eq28_e1867_d_b5: f64 = (eq28_e1865_d_b5 * s.v[184]);let eq28_e1867_d_b6: f64 = (eq28_e1865_d_b6 * s.v[184]);let eq28_e1867_d_b7: f64 = (eq28_e1865_d_b7 * s.v[184]);
        let eq28_e1867_d_b8: f64 = (eq28_e1865_d_b8 * s.v[184]);let eq28_e1867_d_b9: f64 = (eq28_e1865_d_b9 * s.v[184]);let eq28_e1867_d_b10: f64 = (eq28_e1865_d_b10 * s.v[184]);let eq28_e1867_d_b11: f64 = (eq28_e1865_d_b11 * s.v[184]);let eq28_e1869: f64 = (eq28_e1867 * (nv12 - 0.0));let eq28_e1869_d_n0: f64 = (eq28_e1867_d_n0 * (nv12 - 0.0));let eq28_e1869_d_n1: f64 = (eq28_e1867_d_n1 * (nv12 - 0.0));let eq28_e1869_d_n2: f64 = (eq28_e1867_d_n2 * (nv12 - 0.0));let eq28_e1869_d_n3: f64 = (eq28_e1867_d_n3 * (nv12 - 0.0));let eq28_e1869_d_n4: f64 = (eq28_e1867_d_n4 * (nv12 - 0.0));let eq28_e1869_d_n5: f64 = (eq28_e1867_d_n5 * (nv12 - 0.0));let eq28_e1869_d_n6: f64 = (eq28_e1867_d_n6 * (nv12 - 0.0));let eq28_e1869_d_n7: f64 = (eq28_e1867_d_n7 * (nv12 - 0.0));let eq28_e1869_d_n8: f64 = (eq28_e1867_d_n8 * (nv12 - 0.0));let eq28_e1869_d_n9: f64 = (eq28_e1867_d_n9 * (nv12 - 0.0));let eq28_e1869_d_n10: f64 = (eq28_e1867_d_n10 * (nv12 - 0.0));let eq28_e1869_d_n11: f64 = (eq28_e1867_d_n11 * (nv12 - 0.0));let eq28_e1869_d_n12: f64 = ((eq28_e1867_d_n12 * (nv12 - 0.0)) + eq28_e1867);let eq28_e1869_d_n13: f64 = (eq28_e1867_d_n13 * (nv12 - 0.0));let eq28_e1869_d_b0: f64 = (eq28_e1867_d_b0 * (nv12 - 0.0));let eq28_e1869_d_b1: f64 = (eq28_e1867_d_b1 * (nv12 - 0.0));let eq28_e1869_d_b2: f64 = (eq28_e1867_d_b2 * (nv12 - 0.0));let eq28_e1869_d_b3: f64 = (eq28_e1867_d_b3 * (nv12 - 0.0));let eq28_e1869_d_b4: f64 = (eq28_e1867_d_b4 * (nv12 - 0.0));let eq28_e1869_d_b5: f64 = (eq28_e1867_d_b5 * (nv12 - 0.0));let eq28_e1869_d_b6: f64 = (eq28_e1867_d_b6 * (nv12 - 0.0));let eq28_e1869_d_b7: f64 = (eq28_e1867_d_b7 * (nv12 - 0.0));let eq28_e1869_d_b8: f64 = (eq28_e1867_d_b8 * (nv12 - 0.0));let eq28_e1869_d_b9: f64 = (eq28_e1867_d_b9 * (nv12 - 0.0));let eq28_e1869_d_b10: f64 = (eq28_e1867_d_b10 * (nv12 - 0.0));let eq28_e1869_d_b11: f64 = (eq28_e1867_d_b11 * (nv12 - 0.0));let eq28_e1870: f64 = (0.5 * eq28_e1869);let eq28_e1870_d_n0: f64 = (0.5 * eq28_e1869_d_n0);let eq28_e1870_d_n1: f64 = (0.5 * eq28_e1869_d_n1);let eq28_e1870_d_n2: f64 = (0.5 * eq28_e1869_d_n2);let eq28_e1870_d_n3: f64 = (0.5 * eq28_e1869_d_n3);let eq28_e1870_d_n4: f64 = (0.5 * eq28_e1869_d_n4);let eq28_e1870_d_n5: f64 = (0.5 * eq28_e1869_d_n5);let eq28_e1870_d_n6: f64 = (0.5 * eq28_e1869_d_n6);let eq28_e1870_d_n7: f64 = (0.5 * eq28_e1869_d_n7);let eq28_e1870_d_n8: f64 = (0.5 * eq28_e1869_d_n8);let eq28_e1870_d_n9: f64 = (0.5 * eq28_e1869_d_n9);let eq28_e1870_d_n10: f64 = (0.5 * eq28_e1869_d_n10);let eq28_e1870_d_n11: f64 = (0.5 * eq28_e1869_d_n11);let eq28_e1870_d_n12: f64 = (0.5 * eq28_e1869_d_n12);let eq28_e1870_d_n13: f64 = (0.5 * eq28_e1869_d_n13);let eq28_e1870_d_b0: f64 = (0.5 * eq28_e1869_d_b0);let eq28_e1870_d_b1: f64 = (0.5 * eq28_e1869_d_b1);let eq28_e1870_d_b2: f64 = (0.5 * eq28_e1869_d_b2);let eq28_e1870_d_b3: f64 = (0.5 * eq28_e1869_d_b3);let eq28_e1870_d_b4: f64 = (0.5 * eq28_e1869_d_b4);let eq28_e1870_d_b5: f64 = (0.5 * eq28_e1869_d_b5);let eq28_e1870_d_b6: f64 = (0.5 * eq28_e1869_d_b6);let eq28_e1870_d_b7: f64 = (0.5 * eq28_e1869_d_b7);let eq28_e1870_d_b8: f64 = (0.5 * eq28_e1869_d_b8);let eq28_e1870_d_b9: f64 = (0.5 * eq28_e1869_d_b9);let eq28_e1870_d_b10: f64 = (0.5 * eq28_e1869_d_b10);let eq28_e1870_d_b11: f64 = (0.5 * eq28_e1869_d_b11);let eq28_e1871: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e1870);
        (eq28_e1871, (eq28_e1870_d_n0 * ddt_scale), (eq28_e1870_d_n1 * ddt_scale), (eq28_e1870_d_n2 * ddt_scale), (eq28_e1870_d_n3 * ddt_scale), (eq28_e1870_d_n4 * ddt_scale), (eq28_e1870_d_n5 * ddt_scale), (eq28_e1870_d_n6 * ddt_scale), (eq28_e1870_d_n7 * ddt_scale), (eq28_e1870_d_n8 * ddt_scale), (eq28_e1870_d_n9 * ddt_scale), (eq28_e1870_d_n10 * ddt_scale), (eq28_e1870_d_n11 * ddt_scale), (eq28_e1870_d_n12 * ddt_scale), (eq28_e1870_d_n13 * ddt_scale), (eq28_e1870_d_b0 * ddt_scale), (eq28_e1870_d_b1 * ddt_scale), (eq28_e1870_d_b2 * ddt_scale), (eq28_e1870_d_b3 * ddt_scale), (eq28_e1870_d_b4 * ddt_scale), (eq28_e1870_d_b5 * ddt_scale), (eq28_e1870_d_b6 * ddt_scale), (eq28_e1870_d_b7 * ddt_scale), (eq28_e1870_d_b8 * ddt_scale), (eq28_e1870_d_b9 * ddt_scale), (eq28_e1870_d_b10 * ddt_scale), (eq28_e1870_d_b11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1873;let eq28_node_derivatives: [f64; 14] = [eq28_e1873_d_n0, eq28_e1873_d_n1, eq28_e1873_d_n2, eq28_e1873_d_n3, eq28_e1873_d_n4, eq28_e1873_d_n5, eq28_e1873_d_n6, eq28_e1873_d_n7, eq28_e1873_d_n8, eq28_e1873_d_n9, eq28_e1873_d_n10, eq28_e1873_d_n11, eq28_e1873_d_n12, eq28_e1873_d_n13];let eq28_branch_derivatives: [f64; 12] = [eq28_e1873_d_b0, eq28_e1873_d_b1, eq28_e1873_d_b2, eq28_e1873_d_b3, eq28_e1873_d_b4, eq28_e1873_d_b5, eq28_e1873_d_b6, eq28_e1873_d_b7, eq28_e1873_d_b8, eq28_e1873_d_b9, eq28_e1873_d_b10, eq28_e1873_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_10(
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
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq29_e1878, eq29_e1878_d_n13,) = {
    if (!s.b[1620]) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1878;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq29_value),
            13,
            multiplicity * (eq29_e1878_d_n13),
        );
        let (eq30_e1883, eq30_e1883_d_n12,) = {
    if (!s.b[1620]) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1883;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq30_value),
            12,
            multiplicity * (eq30_e1883_d_n12),
        );let eq35_e1943: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, s.v[1057]);let eq35_value: f64 = eq35_e1943;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(10),
            multiplicity * (eq35_value),
            &s.dn[1057],
            &s.db[1057],
            (multiplicity) * (ddt_scale),
        );let eq36_e1945: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, s.v[1058]);let eq36_value: f64 = eq36_e1945;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(11),
            multiplicity * (eq36_value),
            &s.dn[1058],
            &s.db[1058],
            (multiplicity) * (ddt_scale),
        );let eq37_e1947: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, s.v[1051]);let eq37_value: f64 = eq37_e1947;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq37_value),
            &s.dn[1051],
            &s.db[1051],
            (multiplicity) * (ddt_scale),
        );let eq38_e1949: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[1052]);let eq38_value: f64 = eq38_e1949;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq38_value),
            &s.dn[1052],
            &s.db[1052],
            (multiplicity) * (ddt_scale),
        );let eq39_e1951: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[1054]);let eq39_value: f64 = eq39_e1951;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq39_value),
            &s.dn[1054],
            &s.db[1054],
            (multiplicity) * (ddt_scale),
        );let eq40_e1953: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[1055]);let eq40_value: f64 = eq40_e1953;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq40_value),
            &s.dn[1055],
            &s.db[1055],
            (multiplicity) * (ddt_scale),
        );let eq41_e1955: f64 = (-s.v[379]);let eq41_e1957: f64 = (eq41_e1955 * s.v[423]);let eq41_e1957_d_n0: f64 = (((-s.dn[379][0]) * s.v[423]) + (eq41_e1955 * s.dn[423][0]));let eq41_e1957_d_n1: f64 = (((-s.dn[379][1]) * s.v[423]) + (eq41_e1955 * s.dn[423][1]));let eq41_e1957_d_n2: f64 = (((-s.dn[379][2]) * s.v[423]) + (eq41_e1955 * s.dn[423][2]));let eq41_e1957_d_n3: f64 = (((-s.dn[379][3]) * s.v[423]) + (eq41_e1955 * s.dn[423][3]));let eq41_e1957_d_n4: f64 = (((-s.dn[379][4]) * s.v[423]) + (eq41_e1955 * s.dn[423][4]));let eq41_e1957_d_n5: f64 = (((-s.dn[379][5]) * s.v[423]) + (eq41_e1955 * s.dn[423][5]));let eq41_e1957_d_n6: f64 = (((-s.dn[379][6]) * s.v[423]) + (eq41_e1955 * s.dn[423][6]));let eq41_e1957_d_n7: f64 = (((-s.dn[379][7]) * s.v[423]) + (eq41_e1955 * s.dn[423][7]));let eq41_e1957_d_n8: f64 = (((-s.dn[379][8]) * s.v[423]) + (eq41_e1955 * s.dn[423][8]));let eq41_e1957_d_n9: f64 = (((-s.dn[379][9]) * s.v[423]) + (eq41_e1955 * s.dn[423][9]));let eq41_e1957_d_n10: f64 = (((-s.dn[379][10]) * s.v[423]) + (eq41_e1955 * s.dn[423][10]));let eq41_e1957_d_n11: f64 = (((-s.dn[379][11]) * s.v[423]) + (eq41_e1955 * s.dn[423][11]));let eq41_e1957_d_n12: f64 = (((-s.dn[379][12]) * s.v[423]) + (eq41_e1955 * s.dn[423][12]));let eq41_e1957_d_n13: f64 = (((-s.dn[379][13]) * s.v[423]) + (eq41_e1955 * s.dn[423][13]));let eq41_e1957_d_b0: f64 = (((-s.db[379][0]) * s.v[423]) + (eq41_e1955 * s.db[423][0]));let eq41_e1957_d_b1: f64 = (((-s.db[379][1]) * s.v[423]) + (eq41_e1955 * s.db[423][1]));let eq41_e1957_d_b2: f64 = (((-s.db[379][2]) * s.v[423]) + (eq41_e1955 * s.db[423][2]));let eq41_e1957_d_b3: f64 = (((-s.db[379][3]) * s.v[423]) + (eq41_e1955 * s.db[423][3]));let eq41_e1957_d_b4: f64 = (((-s.db[379][4]) * s.v[423]) + (eq41_e1955 * s.db[423][4]));let eq41_e1957_d_b5: f64 = (((-s.db[379][5]) * s.v[423]) + (eq41_e1955 * s.db[423][5]));let eq41_e1957_d_b6: f64 = (((-s.db[379][6]) * s.v[423]) + (eq41_e1955 * s.db[423][6]));let eq41_e1957_d_b7: f64 = (((-s.db[379][7]) * s.v[423]) + (eq41_e1955 * s.db[423][7]));let eq41_e1957_d_b8: f64 = (((-s.db[379][8]) * s.v[423]) + (eq41_e1955 * s.db[423][8]));let eq41_e1957_d_b9: f64 = (((-s.db[379][9]) * s.v[423]) + (eq41_e1955 * s.db[423][9]));let eq41_e1957_d_b10: f64 = (((-s.db[379][10]) * s.v[423]) + (eq41_e1955 * s.db[423][10]));let eq41_e1957_d_b11: f64 = (((-s.db[379][11]) * s.v[423]) + (eq41_e1955 * s.db[423][11]));let eq41_e1958: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq41_e1957);let eq41_value: f64 = eq41_e1958;let eq41_node_derivatives: [f64; 14] = [(eq41_e1957_d_n0 * ddt_scale), (eq41_e1957_d_n1 * ddt_scale), (eq41_e1957_d_n2 * ddt_scale), (eq41_e1957_d_n3 * ddt_scale), (eq41_e1957_d_n4 * ddt_scale), (eq41_e1957_d_n5 * ddt_scale), (eq41_e1957_d_n6 * ddt_scale), (eq41_e1957_d_n7 * ddt_scale), (eq41_e1957_d_n8 * ddt_scale), (eq41_e1957_d_n9 * ddt_scale), (eq41_e1957_d_n10 * ddt_scale), (eq41_e1957_d_n11 * ddt_scale), (eq41_e1957_d_n12 * ddt_scale), (eq41_e1957_d_n13 * ddt_scale)];let eq41_branch_derivatives: [f64; 12] = [(eq41_e1957_d_b0 * ddt_scale), (eq41_e1957_d_b1 * ddt_scale), (eq41_e1957_d_b2 * ddt_scale), (eq41_e1957_d_b3 * ddt_scale), (eq41_e1957_d_b4 * ddt_scale), (eq41_e1957_d_b5 * ddt_scale), (eq41_e1957_d_b6 * ddt_scale), (eq41_e1957_d_b7 * ddt_scale), (eq41_e1957_d_b8 * ddt_scale), (eq41_e1957_d_b9 * ddt_scale), (eq41_e1957_d_b10 * ddt_scale), (eq41_e1957_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );let eq42_e1960: f64 = (-s.v[379]);let eq42_e1962: f64 = (eq42_e1960 * s.v[424]);let eq42_e1962_d_n0: f64 = (((-s.dn[379][0]) * s.v[424]) + (eq42_e1960 * s.dn[424][0]));let eq42_e1962_d_n1: f64 = (((-s.dn[379][1]) * s.v[424]) + (eq42_e1960 * s.dn[424][1]));let eq42_e1962_d_n2: f64 = (((-s.dn[379][2]) * s.v[424]) + (eq42_e1960 * s.dn[424][2]));let eq42_e1962_d_n3: f64 = (((-s.dn[379][3]) * s.v[424]) + (eq42_e1960 * s.dn[424][3]));let eq42_e1962_d_n4: f64 = (((-s.dn[379][4]) * s.v[424]) + (eq42_e1960 * s.dn[424][4]));let eq42_e1962_d_n5: f64 = (((-s.dn[379][5]) * s.v[424]) + (eq42_e1960 * s.dn[424][5]));let eq42_e1962_d_n6: f64 = (((-s.dn[379][6]) * s.v[424]) + (eq42_e1960 * s.dn[424][6]));let eq42_e1962_d_n7: f64 = (((-s.dn[379][7]) * s.v[424]) + (eq42_e1960 * s.dn[424][7]));let eq42_e1962_d_n8: f64 = (((-s.dn[379][8]) * s.v[424]) + (eq42_e1960 * s.dn[424][8]));let eq42_e1962_d_n9: f64 = (((-s.dn[379][9]) * s.v[424]) + (eq42_e1960 * s.dn[424][9]));let eq42_e1962_d_n10: f64 = (((-s.dn[379][10]) * s.v[424]) + (eq42_e1960 * s.dn[424][10]));let eq42_e1962_d_n11: f64 = (((-s.dn[379][11]) * s.v[424]) + (eq42_e1960 * s.dn[424][11]));let eq42_e1962_d_n12: f64 = (((-s.dn[379][12]) * s.v[424]) + (eq42_e1960 * s.dn[424][12]));let eq42_e1962_d_n13: f64 = (((-s.dn[379][13]) * s.v[424]) + (eq42_e1960 * s.dn[424][13]));let eq42_e1962_d_b0: f64 = (((-s.db[379][0]) * s.v[424]) + (eq42_e1960 * s.db[424][0]));let eq42_e1962_d_b1: f64 = (((-s.db[379][1]) * s.v[424]) + (eq42_e1960 * s.db[424][1]));let eq42_e1962_d_b2: f64 = (((-s.db[379][2]) * s.v[424]) + (eq42_e1960 * s.db[424][2]));let eq42_e1962_d_b3: f64 = (((-s.db[379][3]) * s.v[424]) + (eq42_e1960 * s.db[424][3]));let eq42_e1962_d_b4: f64 = (((-s.db[379][4]) * s.v[424]) + (eq42_e1960 * s.db[424][4]));let eq42_e1962_d_b5: f64 = (((-s.db[379][5]) * s.v[424]) + (eq42_e1960 * s.db[424][5]));let eq42_e1962_d_b6: f64 = (((-s.db[379][6]) * s.v[424]) + (eq42_e1960 * s.db[424][6]));let eq42_e1962_d_b7: f64 = (((-s.db[379][7]) * s.v[424]) + (eq42_e1960 * s.db[424][7]));let eq42_e1962_d_b8: f64 = (((-s.db[379][8]) * s.v[424]) + (eq42_e1960 * s.db[424][8]));let eq42_e1962_d_b9: f64 = (((-s.db[379][9]) * s.v[424]) + (eq42_e1960 * s.db[424][9]));let eq42_e1962_d_b10: f64 = (((-s.db[379][10]) * s.v[424]) + (eq42_e1960 * s.db[424][10]));let eq42_e1962_d_b11: f64 = (((-s.db[379][11]) * s.v[424]) + (eq42_e1960 * s.db[424][11]));let eq42_e1963: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq42_e1962);let eq42_value: f64 = eq42_e1963;let eq42_node_derivatives: [f64; 14] = [(eq42_e1962_d_n0 * ddt_scale), (eq42_e1962_d_n1 * ddt_scale), (eq42_e1962_d_n2 * ddt_scale), (eq42_e1962_d_n3 * ddt_scale), (eq42_e1962_d_n4 * ddt_scale), (eq42_e1962_d_n5 * ddt_scale), (eq42_e1962_d_n6 * ddt_scale), (eq42_e1962_d_n7 * ddt_scale), (eq42_e1962_d_n8 * ddt_scale), (eq42_e1962_d_n9 * ddt_scale), (eq42_e1962_d_n10 * ddt_scale), (eq42_e1962_d_n11 * ddt_scale), (eq42_e1962_d_n12 * ddt_scale), (eq42_e1962_d_n13 * ddt_scale)];let eq42_branch_derivatives: [f64; 12] = [(eq42_e1962_d_b0 * ddt_scale), (eq42_e1962_d_b1 * ddt_scale), (eq42_e1962_d_b2 * ddt_scale), (eq42_e1962_d_b3 * ddt_scale), (eq42_e1962_d_b4 * ddt_scale), (eq42_e1962_d_b5 * ddt_scale), (eq42_e1962_d_b6 * ddt_scale), (eq42_e1962_d_b7 * ddt_scale), (eq42_e1962_d_b8 * ddt_scale), (eq42_e1962_d_b9 * ddt_scale), (eq42_e1962_d_b10 * ddt_scale), (eq42_e1962_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_11(
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
        let eq43_e1965: f64 = (-s.v[379]);let eq43_e1967: f64 = (eq43_e1965 * s.v[421]);let eq43_e1967_d_n0: f64 = (((-s.dn[379][0]) * s.v[421]) + (eq43_e1965 * s.dn[421][0]));let eq43_e1967_d_n1: f64 = (((-s.dn[379][1]) * s.v[421]) + (eq43_e1965 * s.dn[421][1]));let eq43_e1967_d_n2: f64 = (((-s.dn[379][2]) * s.v[421]) + (eq43_e1965 * s.dn[421][2]));let eq43_e1967_d_n3: f64 = (((-s.dn[379][3]) * s.v[421]) + (eq43_e1965 * s.dn[421][3]));let eq43_e1967_d_n4: f64 = (((-s.dn[379][4]) * s.v[421]) + (eq43_e1965 * s.dn[421][4]));let eq43_e1967_d_n5: f64 = (((-s.dn[379][5]) * s.v[421]) + (eq43_e1965 * s.dn[421][5]));let eq43_e1967_d_n6: f64 = (((-s.dn[379][6]) * s.v[421]) + (eq43_e1965 * s.dn[421][6]));let eq43_e1967_d_n7: f64 = (((-s.dn[379][7]) * s.v[421]) + (eq43_e1965 * s.dn[421][7]));let eq43_e1967_d_n8: f64 = (((-s.dn[379][8]) * s.v[421]) + (eq43_e1965 * s.dn[421][8]));let eq43_e1967_d_n9: f64 = (((-s.dn[379][9]) * s.v[421]) + (eq43_e1965 * s.dn[421][9]));let eq43_e1967_d_n10: f64 = (((-s.dn[379][10]) * s.v[421]) + (eq43_e1965 * s.dn[421][10]));let eq43_e1967_d_n11: f64 = (((-s.dn[379][11]) * s.v[421]) + (eq43_e1965 * s.dn[421][11]));let eq43_e1967_d_n12: f64 = (((-s.dn[379][12]) * s.v[421]) + (eq43_e1965 * s.dn[421][12]));let eq43_e1967_d_n13: f64 = (((-s.dn[379][13]) * s.v[421]) + (eq43_e1965 * s.dn[421][13]));let eq43_e1967_d_b0: f64 = (((-s.db[379][0]) * s.v[421]) + (eq43_e1965 * s.db[421][0]));let eq43_e1967_d_b1: f64 = (((-s.db[379][1]) * s.v[421]) + (eq43_e1965 * s.db[421][1]));let eq43_e1967_d_b2: f64 = (((-s.db[379][2]) * s.v[421]) + (eq43_e1965 * s.db[421][2]));let eq43_e1967_d_b3: f64 = (((-s.db[379][3]) * s.v[421]) + (eq43_e1965 * s.db[421][3]));let eq43_e1967_d_b4: f64 = (((-s.db[379][4]) * s.v[421]) + (eq43_e1965 * s.db[421][4]));let eq43_e1967_d_b5: f64 = (((-s.db[379][5]) * s.v[421]) + (eq43_e1965 * s.db[421][5]));let eq43_e1967_d_b6: f64 = (((-s.db[379][6]) * s.v[421]) + (eq43_e1965 * s.db[421][6]));let eq43_e1967_d_b7: f64 = (((-s.db[379][7]) * s.v[421]) + (eq43_e1965 * s.db[421][7]));let eq43_e1967_d_b8: f64 = (((-s.db[379][8]) * s.v[421]) + (eq43_e1965 * s.db[421][8]));let eq43_e1967_d_b9: f64 = (((-s.db[379][9]) * s.v[421]) + (eq43_e1965 * s.db[421][9]));let eq43_e1967_d_b10: f64 = (((-s.db[379][10]) * s.v[421]) + (eq43_e1965 * s.db[421][10]));let eq43_e1967_d_b11: f64 = (((-s.db[379][11]) * s.v[421]) + (eq43_e1965 * s.db[421][11]));let eq43_e1968: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq43_e1967);let eq43_value: f64 = eq43_e1968;let eq43_node_derivatives: [f64; 14] = [(eq43_e1967_d_n0 * ddt_scale), (eq43_e1967_d_n1 * ddt_scale), (eq43_e1967_d_n2 * ddt_scale), (eq43_e1967_d_n3 * ddt_scale), (eq43_e1967_d_n4 * ddt_scale), (eq43_e1967_d_n5 * ddt_scale), (eq43_e1967_d_n6 * ddt_scale), (eq43_e1967_d_n7 * ddt_scale), (eq43_e1967_d_n8 * ddt_scale), (eq43_e1967_d_n9 * ddt_scale), (eq43_e1967_d_n10 * ddt_scale), (eq43_e1967_d_n11 * ddt_scale), (eq43_e1967_d_n12 * ddt_scale), (eq43_e1967_d_n13 * ddt_scale)];let eq43_branch_derivatives: [f64; 12] = [(eq43_e1967_d_b0 * ddt_scale), (eq43_e1967_d_b1 * ddt_scale), (eq43_e1967_d_b2 * ddt_scale), (eq43_e1967_d_b3 * ddt_scale), (eq43_e1967_d_b4 * ddt_scale), (eq43_e1967_d_b5 * ddt_scale), (eq43_e1967_d_b6 * ddt_scale), (eq43_e1967_d_b7 * ddt_scale), (eq43_e1967_d_b8 * ddt_scale), (eq43_e1967_d_b9 * ddt_scale), (eq43_e1967_d_b10 * ddt_scale), (eq43_e1967_d_b11 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );let eq44_e1971: f64 = (s.v[379] * s.v[210]);let eq44_e1971_d_n0: f64 = ((s.dn[379][0] * s.v[210]) + (s.v[379] * s.dn[210][0]));let eq44_e1971_d_n1: f64 = ((s.dn[379][1] * s.v[210]) + (s.v[379] * s.dn[210][1]));let eq44_e1971_d_n2: f64 = ((s.dn[379][2] * s.v[210]) + (s.v[379] * s.dn[210][2]));let eq44_e1971_d_n3: f64 = ((s.dn[379][3] * s.v[210]) + (s.v[379] * s.dn[210][3]));let eq44_e1971_d_n4: f64 = ((s.dn[379][4] * s.v[210]) + (s.v[379] * s.dn[210][4]));let eq44_e1971_d_n5: f64 = ((s.dn[379][5] * s.v[210]) + (s.v[379] * s.dn[210][5]));let eq44_e1971_d_n6: f64 = ((s.dn[379][6] * s.v[210]) + (s.v[379] * s.dn[210][6]));let eq44_e1971_d_n7: f64 = ((s.dn[379][7] * s.v[210]) + (s.v[379] * s.dn[210][7]));let eq44_e1971_d_n8: f64 = ((s.dn[379][8] * s.v[210]) + (s.v[379] * s.dn[210][8]));let eq44_e1971_d_n9: f64 = ((s.dn[379][9] * s.v[210]) + (s.v[379] * s.dn[210][9]));let eq44_e1971_d_n10: f64 = ((s.dn[379][10] * s.v[210]) + (s.v[379] * s.dn[210][10]));let eq44_e1971_d_n11: f64 = ((s.dn[379][11] * s.v[210]) + (s.v[379] * s.dn[210][11]));let eq44_e1971_d_n12: f64 = ((s.dn[379][12] * s.v[210]) + (s.v[379] * s.dn[210][12]));let eq44_e1971_d_n13: f64 = ((s.dn[379][13] * s.v[210]) + (s.v[379] * s.dn[210][13]));let eq44_e1971_d_b0: f64 = ((s.db[379][0] * s.v[210]) + (s.v[379] * s.db[210][0]));let eq44_e1971_d_b1: f64 = ((s.db[379][1] * s.v[210]) + (s.v[379] * s.db[210][1]));let eq44_e1971_d_b2: f64 = ((s.db[379][2] * s.v[210]) + (s.v[379] * s.db[210][2]));let eq44_e1971_d_b3: f64 = ((s.db[379][3] * s.v[210]) + (s.v[379] * s.db[210][3]));let eq44_e1971_d_b4: f64 = ((s.db[379][4] * s.v[210]) + (s.v[379] * s.db[210][4]));let eq44_e1971_d_b5: f64 = ((s.db[379][5] * s.v[210]) + (s.v[379] * s.db[210][5]));let eq44_e1971_d_b6: f64 = ((s.db[379][6] * s.v[210]) + (s.v[379] * s.db[210][6]));let eq44_e1971_d_b7: f64 = ((s.db[379][7] * s.v[210]) + (s.v[379] * s.db[210][7]));let eq44_e1971_d_b8: f64 = ((s.db[379][8] * s.v[210]) + (s.v[379] * s.db[210][8]));let eq44_e1971_d_b9: f64 = ((s.db[379][9] * s.v[210]) + (s.v[379] * s.db[210][9]));let eq44_e1971_d_b10: f64 = ((s.db[379][10] * s.v[210]) + (s.v[379] * s.db[210][10]));let eq44_e1971_d_b11: f64 = ((s.db[379][11] * s.v[210]) + (s.v[379] * s.db[210][11]));let eq44_value: f64 = eq44_e1971;let eq44_node_derivatives: [f64; 14] = [eq44_e1971_d_n0, eq44_e1971_d_n1, eq44_e1971_d_n2, eq44_e1971_d_n3, eq44_e1971_d_n4, eq44_e1971_d_n5, eq44_e1971_d_n6, eq44_e1971_d_n7, eq44_e1971_d_n8, eq44_e1971_d_n9, eq44_e1971_d_n10, eq44_e1971_d_n11, eq44_e1971_d_n12, eq44_e1971_d_n13];let eq44_branch_derivatives: [f64; 12] = [eq44_e1971_d_b0, eq44_e1971_d_b1, eq44_e1971_d_b2, eq44_e1971_d_b3, eq44_e1971_d_b4, eq44_e1971_d_b5, eq44_e1971_d_b6, eq44_e1971_d_b7, eq44_e1971_d_b8, eq44_e1971_d_b9, eq44_e1971_d_b10, eq44_e1971_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(11),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_12(
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
        let eq45_e1974: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[1039]);let eq45_e1975: f64 = (s.v[379] * eq45_e1974);let eq45_e1975_d_n0: f64 = ((s.dn[379][0] * eq45_e1974) + (s.v[379] * (s.dn[1039][0] * ddt_scale)));let eq45_e1975_d_n1: f64 = ((s.dn[379][1] * eq45_e1974) + (s.v[379] * (s.dn[1039][1] * ddt_scale)));let eq45_e1975_d_n2: f64 = ((s.dn[379][2] * eq45_e1974) + (s.v[379] * (s.dn[1039][2] * ddt_scale)));let eq45_e1975_d_n3: f64 = ((s.dn[379][3] * eq45_e1974) + (s.v[379] * (s.dn[1039][3] * ddt_scale)));let eq45_e1975_d_n4: f64 = ((s.dn[379][4] * eq45_e1974) + (s.v[379] * (s.dn[1039][4] * ddt_scale)));let eq45_e1975_d_n5: f64 = ((s.dn[379][5] * eq45_e1974) + (s.v[379] * (s.dn[1039][5] * ddt_scale)));let eq45_e1975_d_n6: f64 = ((s.dn[379][6] * eq45_e1974) + (s.v[379] * (s.dn[1039][6] * ddt_scale)));let eq45_e1975_d_n7: f64 = ((s.dn[379][7] * eq45_e1974) + (s.v[379] * (s.dn[1039][7] * ddt_scale)));let eq45_e1975_d_n8: f64 = ((s.dn[379][8] * eq45_e1974) + (s.v[379] * (s.dn[1039][8] * ddt_scale)));let eq45_e1975_d_n9: f64 = ((s.dn[379][9] * eq45_e1974) + (s.v[379] * (s.dn[1039][9] * ddt_scale)));let eq45_e1975_d_n10: f64 = ((s.dn[379][10] * eq45_e1974) + (s.v[379] * (s.dn[1039][10] * ddt_scale)));let eq45_e1975_d_n11: f64 = ((s.dn[379][11] * eq45_e1974) + (s.v[379] * (s.dn[1039][11] * ddt_scale)));let eq45_e1975_d_n12: f64 = ((s.dn[379][12] * eq45_e1974) + (s.v[379] * (s.dn[1039][12] * ddt_scale)));let eq45_e1975_d_n13: f64 = ((s.dn[379][13] * eq45_e1974) + (s.v[379] * (s.dn[1039][13] * ddt_scale)));let eq45_e1975_d_b0: f64 = ((s.db[379][0] * eq45_e1974) + (s.v[379] * (s.db[1039][0] * ddt_scale)));let eq45_e1975_d_b1: f64 = ((s.db[379][1] * eq45_e1974) + (s.v[379] * (s.db[1039][1] * ddt_scale)));let eq45_e1975_d_b2: f64 = ((s.db[379][2] * eq45_e1974) + (s.v[379] * (s.db[1039][2] * ddt_scale)));let eq45_e1975_d_b3: f64 = ((s.db[379][3] * eq45_e1974) + (s.v[379] * (s.db[1039][3] * ddt_scale)));let eq45_e1975_d_b4: f64 = ((s.db[379][4] * eq45_e1974) + (s.v[379] * (s.db[1039][4] * ddt_scale)));let eq45_e1975_d_b5: f64 = ((s.db[379][5] * eq45_e1974) + (s.v[379] * (s.db[1039][5] * ddt_scale)));let eq45_e1975_d_b6: f64 = ((s.db[379][6] * eq45_e1974) + (s.v[379] * (s.db[1039][6] * ddt_scale)));let eq45_e1975_d_b7: f64 = ((s.db[379][7] * eq45_e1974) + (s.v[379] * (s.db[1039][7] * ddt_scale)));let eq45_e1975_d_b8: f64 = ((s.db[379][8] * eq45_e1974) + (s.v[379] * (s.db[1039][8] * ddt_scale)));let eq45_e1975_d_b9: f64 = ((s.db[379][9] * eq45_e1974) + (s.v[379] * (s.db[1039][9] * ddt_scale)));let eq45_e1975_d_b10: f64 = ((s.db[379][10] * eq45_e1974) + (s.v[379] * (s.db[1039][10] * ddt_scale)));let eq45_e1975_d_b11: f64 = ((s.db[379][11] * eq45_e1974) + (s.v[379] * (s.db[1039][11] * ddt_scale)));let eq45_value: f64 = eq45_e1975;let eq45_node_derivatives: [f64; 14] = [eq45_e1975_d_n0, eq45_e1975_d_n1, eq45_e1975_d_n2, eq45_e1975_d_n3, eq45_e1975_d_n4, eq45_e1975_d_n5, eq45_e1975_d_n6, eq45_e1975_d_n7, eq45_e1975_d_n8, eq45_e1975_d_n9, eq45_e1975_d_n10, eq45_e1975_d_n11, eq45_e1975_d_n12, eq45_e1975_d_n13];let eq45_branch_derivatives: [f64; 12] = [eq45_e1975_d_b0, eq45_e1975_d_b1, eq45_e1975_d_b2, eq45_e1975_d_b3, eq45_e1975_d_b4, eq45_e1975_d_b5, eq45_e1975_d_b6, eq45_e1975_d_b7, eq45_e1975_d_b8, eq45_e1975_d_b9, eq45_e1975_d_b10, eq45_e1975_d_b11];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );let eq46_e1977: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, s.v[1047]);let eq46_value: f64 = eq46_e1977;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq46_value),
            &s.dn[1047],
            &s.db[1047],
            (multiplicity) * (ddt_scale),
        );let eq47_e1979: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, s.v[1046]);let eq47_value: f64 = eq47_e1979;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq47_value),
            &s.dn[1046],
            &s.db[1046],
            (multiplicity) * (ddt_scale),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_13(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let eq48_e1982: f64 = (s.v[379] * s.v[211]);let eq48_e1982_d_n0: f64 = ((s.dn[379][0] * s.v[211]) + (s.v[379] * s.dn[211][0]));let eq48_e1982_d_n1: f64 = ((s.dn[379][1] * s.v[211]) + (s.v[379] * s.dn[211][1]));let eq48_e1982_d_n2: f64 = ((s.dn[379][2] * s.v[211]) + (s.v[379] * s.dn[211][2]));let eq48_e1982_d_n3: f64 = ((s.dn[379][3] * s.v[211]) + (s.v[379] * s.dn[211][3]));let eq48_e1982_d_n4: f64 = ((s.dn[379][4] * s.v[211]) + (s.v[379] * s.dn[211][4]));let eq48_e1982_d_n5: f64 = ((s.dn[379][5] * s.v[211]) + (s.v[379] * s.dn[211][5]));let eq48_e1982_d_n6: f64 = ((s.dn[379][6] * s.v[211]) + (s.v[379] * s.dn[211][6]));let eq48_e1982_d_n7: f64 = ((s.dn[379][7] * s.v[211]) + (s.v[379] * s.dn[211][7]));let eq48_e1982_d_n8: f64 = ((s.dn[379][8] * s.v[211]) + (s.v[379] * s.dn[211][8]));let eq48_e1982_d_n9: f64 = ((s.dn[379][9] * s.v[211]) + (s.v[379] * s.dn[211][9]));let eq48_e1982_d_n10: f64 = ((s.dn[379][10] * s.v[211]) + (s.v[379] * s.dn[211][10]));let eq48_e1982_d_n11: f64 = ((s.dn[379][11] * s.v[211]) + (s.v[379] * s.dn[211][11]));let eq48_e1982_d_n12: f64 = ((s.dn[379][12] * s.v[211]) + (s.v[379] * s.dn[211][12]));let eq48_e1982_d_n13: f64 = ((s.dn[379][13] * s.v[211]) + (s.v[379] * s.dn[211][13]));let eq48_e1982_d_b0: f64 = ((s.db[379][0] * s.v[211]) + (s.v[379] * s.db[211][0]));let eq48_e1982_d_b1: f64 = ((s.db[379][1] * s.v[211]) + (s.v[379] * s.db[211][1]));let eq48_e1982_d_b2: f64 = ((s.db[379][2] * s.v[211]) + (s.v[379] * s.db[211][2]));let eq48_e1982_d_b3: f64 = ((s.db[379][3] * s.v[211]) + (s.v[379] * s.db[211][3]));let eq48_e1982_d_b4: f64 = ((s.db[379][4] * s.v[211]) + (s.v[379] * s.db[211][4]));let eq48_e1982_d_b5: f64 = ((s.db[379][5] * s.v[211]) + (s.v[379] * s.db[211][5]));let eq48_e1982_d_b6: f64 = ((s.db[379][6] * s.v[211]) + (s.v[379] * s.db[211][6]));let eq48_e1982_d_b7: f64 = ((s.db[379][7] * s.v[211]) + (s.v[379] * s.db[211][7]));let eq48_e1982_d_b8: f64 = ((s.db[379][8] * s.v[211]) + (s.v[379] * s.db[211][8]));let eq48_e1982_d_b9: f64 = ((s.db[379][9] * s.v[211]) + (s.v[379] * s.db[211][9]));let eq48_e1982_d_b10: f64 = ((s.db[379][10] * s.v[211]) + (s.v[379] * s.db[211][10]));let eq48_e1982_d_b11: f64 = ((s.db[379][11] * s.v[211]) + (s.v[379] * s.db[211][11]));let eq48_e1984: f64 = (eq48_e1982 * s.v[380]);let eq48_e1984_d_n0: f64 = ((eq48_e1982_d_n0 * s.v[380]) + (eq48_e1982 * s.dn[380][0]));let eq48_e1984_d_n1: f64 = ((eq48_e1982_d_n1 * s.v[380]) + (eq48_e1982 * s.dn[380][1]));let eq48_e1984_d_n2: f64 = ((eq48_e1982_d_n2 * s.v[380]) + (eq48_e1982 * s.dn[380][2]));let eq48_e1984_d_n3: f64 = ((eq48_e1982_d_n3 * s.v[380]) + (eq48_e1982 * s.dn[380][3]));let eq48_e1984_d_n4: f64 = ((eq48_e1982_d_n4 * s.v[380]) + (eq48_e1982 * s.dn[380][4]));let eq48_e1984_d_n5: f64 = ((eq48_e1982_d_n5 * s.v[380]) + (eq48_e1982 * s.dn[380][5]));let eq48_e1984_d_n6: f64 = ((eq48_e1982_d_n6 * s.v[380]) + (eq48_e1982 * s.dn[380][6]));let eq48_e1984_d_n7: f64 = ((eq48_e1982_d_n7 * s.v[380]) + (eq48_e1982 * s.dn[380][7]));let eq48_e1984_d_n8: f64 = ((eq48_e1982_d_n8 * s.v[380]) + (eq48_e1982 * s.dn[380][8]));let eq48_e1984_d_n9: f64 = ((eq48_e1982_d_n9 * s.v[380]) + (eq48_e1982 * s.dn[380][9]));let eq48_e1984_d_n10: f64 = ((eq48_e1982_d_n10 * s.v[380]) + (eq48_e1982 * s.dn[380][10]));let eq48_e1984_d_n11: f64 = ((eq48_e1982_d_n11 * s.v[380]) + (eq48_e1982 * s.dn[380][11]));let eq48_e1984_d_n12: f64 = ((eq48_e1982_d_n12 * s.v[380]) + (eq48_e1982 * s.dn[380][12]));let eq48_e1984_d_n13: f64 = ((eq48_e1982_d_n13 * s.v[380]) + (eq48_e1982 * s.dn[380][13]));let eq48_e1984_d_b0: f64 = ((eq48_e1982_d_b0 * s.v[380]) + (eq48_e1982 * s.db[380][0]));let eq48_e1984_d_b1: f64 = ((eq48_e1982_d_b1 * s.v[380]) + (eq48_e1982 * s.db[380][1]));let eq48_e1984_d_b2: f64 = ((eq48_e1982_d_b2 * s.v[380]) + (eq48_e1982 * s.db[380][2]));let eq48_e1984_d_b3: f64 = ((eq48_e1982_d_b3 * s.v[380]) + (eq48_e1982 * s.db[380][3]));let eq48_e1984_d_b4: f64 = ((eq48_e1982_d_b4 * s.v[380]) + (eq48_e1982 * s.db[380][4]));let eq48_e1984_d_b5: f64 = ((eq48_e1982_d_b5 * s.v[380]) + (eq48_e1982 * s.db[380][5]));
        let eq48_e1984_d_b6: f64 = ((eq48_e1982_d_b6 * s.v[380]) + (eq48_e1982 * s.db[380][6]));let eq48_e1984_d_b7: f64 = ((eq48_e1982_d_b7 * s.v[380]) + (eq48_e1982 * s.db[380][7]));let eq48_e1984_d_b8: f64 = ((eq48_e1982_d_b8 * s.v[380]) + (eq48_e1982 * s.db[380][8]));let eq48_e1984_d_b9: f64 = ((eq48_e1982_d_b9 * s.v[380]) + (eq48_e1982 * s.db[380][9]));let eq48_e1984_d_b10: f64 = ((eq48_e1982_d_b10 * s.v[380]) + (eq48_e1982 * s.db[380][10]));let eq48_e1984_d_b11: f64 = ((eq48_e1982_d_b11 * s.v[380]) + (eq48_e1982 * s.db[380][11]));let eq48_value: f64 = eq48_e1984;let eq48_node_derivatives: [f64; 14] = [eq48_e1984_d_n0, eq48_e1984_d_n1, eq48_e1984_d_n2, eq48_e1984_d_n3, eq48_e1984_d_n4, eq48_e1984_d_n5, eq48_e1984_d_n6, eq48_e1984_d_n7, eq48_e1984_d_n8, eq48_e1984_d_n9, eq48_e1984_d_n10, eq48_e1984_d_n11, eq48_e1984_d_n12, eq48_e1984_d_n13];let eq48_branch_derivatives: [f64; 12] = [eq48_e1984_d_b0, eq48_e1984_d_b1, eq48_e1984_d_b2, eq48_e1984_d_b3, eq48_e1984_d_b4, eq48_e1984_d_b5, eq48_e1984_d_b6, eq48_e1984_d_b7, eq48_e1984_d_b8, eq48_e1984_d_b9, eq48_e1984_d_b10, eq48_e1984_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e1988, eq49_e1988_d_n0, eq49_e1988_d_n1, eq49_e1988_d_n2, eq49_e1988_d_n3, eq49_e1988_d_n4, eq49_e1988_d_n5, eq49_e1988_d_n6, eq49_e1988_d_n7, eq49_e1988_d_n8, eq49_e1988_d_n9, eq49_e1988_d_n10, eq49_e1988_d_n11, eq49_e1988_d_n12, eq49_e1988_d_n13, eq49_e1988_d_b0, eq49_e1988_d_b1, eq49_e1988_d_b2, eq49_e1988_d_b3, eq49_e1988_d_b4, eq49_e1988_d_b5, eq49_e1988_d_b6, eq49_e1988_d_b7, eq49_e1988_d_b8, eq49_e1988_d_b9, eq49_e1988_d_b10, eq49_e1988_d_b11,) = {
    if s.b[2009] {
        (s.v[1102], s.dn[1102][0], s.dn[1102][1], s.dn[1102][2], s.dn[1102][3], s.dn[1102][4], s.dn[1102][5], s.dn[1102][6], s.dn[1102][7], s.dn[1102][8], s.dn[1102][9], s.dn[1102][10], s.dn[1102][11], s.dn[1102][12], s.dn[1102][13], s.db[1102][0], s.db[1102][1], s.db[1102][2], s.db[1102][3], s.db[1102][4], s.db[1102][5], s.db[1102][6], s.db[1102][7], s.db[1102][8], s.db[1102][9], s.db[1102][10], s.db[1102][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1988;let eq49_node_derivatives: [f64; 14] = [eq49_e1988_d_n0, eq49_e1988_d_n1, eq49_e1988_d_n2, eq49_e1988_d_n3, eq49_e1988_d_n4, eq49_e1988_d_n5, eq49_e1988_d_n6, eq49_e1988_d_n7, eq49_e1988_d_n8, eq49_e1988_d_n9, eq49_e1988_d_n10, eq49_e1988_d_n11, eq49_e1988_d_n12, eq49_e1988_d_n13];let eq49_branch_derivatives: [f64; 12] = [eq49_e1988_d_b0, eq49_e1988_d_b1, eq49_e1988_d_b2, eq49_e1988_d_b3, eq49_e1988_d_b4, eq49_e1988_d_b5, eq49_e1988_d_b6, eq49_e1988_d_b7, eq49_e1988_d_b8, eq49_e1988_d_b9, eq49_e1988_d_b10, eq49_e1988_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(10),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_14(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq50_e1994, eq50_e1994_d_n0, eq50_e1994_d_n1, eq50_e1994_d_n2, eq50_e1994_d_n3, eq50_e1994_d_n4, eq50_e1994_d_n5, eq50_e1994_d_n6, eq50_e1994_d_n7, eq50_e1994_d_n8, eq50_e1994_d_n9, eq50_e1994_d_n10, eq50_e1994_d_n11, eq50_e1994_d_n12, eq50_e1994_d_n13, eq50_e1994_d_b0, eq50_e1994_d_b1, eq50_e1994_d_b2, eq50_e1994_d_b3, eq50_e1994_d_b4, eq50_e1994_d_b5, eq50_e1994_d_b6, eq50_e1994_d_b7, eq50_e1994_d_b8, eq50_e1994_d_b9, eq50_e1994_d_b10, eq50_e1994_d_b11,) = {
    if s.b[2010] {
        let eq50_e1992: f64 = (s.v[1098] + s.v[1100]);let eq50_e1992_d_n0: f64 = (s.dn[1098][0] + s.dn[1100][0]);let eq50_e1992_d_n1: f64 = (s.dn[1098][1] + s.dn[1100][1]);let eq50_e1992_d_n2: f64 = (s.dn[1098][2] + s.dn[1100][2]);let eq50_e1992_d_n3: f64 = (s.dn[1098][3] + s.dn[1100][3]);let eq50_e1992_d_n4: f64 = (s.dn[1098][4] + s.dn[1100][4]);let eq50_e1992_d_n5: f64 = (s.dn[1098][5] + s.dn[1100][5]);let eq50_e1992_d_n6: f64 = (s.dn[1098][6] + s.dn[1100][6]);let eq50_e1992_d_n7: f64 = (s.dn[1098][7] + s.dn[1100][7]);let eq50_e1992_d_n8: f64 = (s.dn[1098][8] + s.dn[1100][8]);let eq50_e1992_d_n9: f64 = (s.dn[1098][9] + s.dn[1100][9]);let eq50_e1992_d_n10: f64 = (s.dn[1098][10] + s.dn[1100][10]);let eq50_e1992_d_n11: f64 = (s.dn[1098][11] + s.dn[1100][11]);let eq50_e1992_d_n12: f64 = (s.dn[1098][12] + s.dn[1100][12]);let eq50_e1992_d_n13: f64 = (s.dn[1098][13] + s.dn[1100][13]);let eq50_e1992_d_b0: f64 = (s.db[1098][0] + s.db[1100][0]);let eq50_e1992_d_b1: f64 = (s.db[1098][1] + s.db[1100][1]);let eq50_e1992_d_b2: f64 = (s.db[1098][2] + s.db[1100][2]);let eq50_e1992_d_b3: f64 = (s.db[1098][3] + s.db[1100][3]);let eq50_e1992_d_b4: f64 = (s.db[1098][4] + s.db[1100][4]);let eq50_e1992_d_b5: f64 = (s.db[1098][5] + s.db[1100][5]);let eq50_e1992_d_b6: f64 = (s.db[1098][6] + s.db[1100][6]);let eq50_e1992_d_b7: f64 = (s.db[1098][7] + s.db[1100][7]);let eq50_e1992_d_b8: f64 = (s.db[1098][8] + s.db[1100][8]);let eq50_e1992_d_b9: f64 = (s.db[1098][9] + s.db[1100][9]);let eq50_e1992_d_b10: f64 = (s.db[1098][10] + s.db[1100][10]);let eq50_e1992_d_b11: f64 = (s.db[1098][11] + s.db[1100][11]);
        (eq50_e1992, eq50_e1992_d_n0, eq50_e1992_d_n1, eq50_e1992_d_n2, eq50_e1992_d_n3, eq50_e1992_d_n4, eq50_e1992_d_n5, eq50_e1992_d_n6, eq50_e1992_d_n7, eq50_e1992_d_n8, eq50_e1992_d_n9, eq50_e1992_d_n10, eq50_e1992_d_n11, eq50_e1992_d_n12, eq50_e1992_d_n13, eq50_e1992_d_b0, eq50_e1992_d_b1, eq50_e1992_d_b2, eq50_e1992_d_b3, eq50_e1992_d_b4, eq50_e1992_d_b5, eq50_e1992_d_b6, eq50_e1992_d_b7, eq50_e1992_d_b8, eq50_e1992_d_b9, eq50_e1992_d_b10, eq50_e1992_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1994;let eq50_node_derivatives: [f64; 14] = [eq50_e1994_d_n0, eq50_e1994_d_n1, eq50_e1994_d_n2, eq50_e1994_d_n3, eq50_e1994_d_n4, eq50_e1994_d_n5, eq50_e1994_d_n6, eq50_e1994_d_n7, eq50_e1994_d_n8, eq50_e1994_d_n9, eq50_e1994_d_n10, eq50_e1994_d_n11, eq50_e1994_d_n12, eq50_e1994_d_n13];let eq50_branch_derivatives: [f64; 12] = [eq50_e1994_d_b0, eq50_e1994_d_b1, eq50_e1994_d_b2, eq50_e1994_d_b3, eq50_e1994_d_b4, eq50_e1994_d_b5, eq50_e1994_d_b6, eq50_e1994_d_b7, eq50_e1994_d_b8, eq50_e1994_d_b9, eq50_e1994_d_b10, eq50_e1994_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2000, eq51_e2000_d_n0, eq51_e2000_d_n1, eq51_e2000_d_n2, eq51_e2000_d_n3, eq51_e2000_d_n4, eq51_e2000_d_n5, eq51_e2000_d_n6, eq51_e2000_d_n7, eq51_e2000_d_n8, eq51_e2000_d_n9, eq51_e2000_d_n10, eq51_e2000_d_n11, eq51_e2000_d_n12, eq51_e2000_d_n13, eq51_e2000_d_b0, eq51_e2000_d_b1, eq51_e2000_d_b2, eq51_e2000_d_b3, eq51_e2000_d_b4, eq51_e2000_d_b5, eq51_e2000_d_b6, eq51_e2000_d_b7, eq51_e2000_d_b8, eq51_e2000_d_b9, eq51_e2000_d_b10, eq51_e2000_d_b11,) = {
    if s.b[2010] {
        let eq51_e1998: f64 = (s.v[1099] + s.v[1101]);let eq51_e1998_d_n0: f64 = (s.dn[1099][0] + s.dn[1101][0]);let eq51_e1998_d_n1: f64 = (s.dn[1099][1] + s.dn[1101][1]);let eq51_e1998_d_n2: f64 = (s.dn[1099][2] + s.dn[1101][2]);let eq51_e1998_d_n3: f64 = (s.dn[1099][3] + s.dn[1101][3]);let eq51_e1998_d_n4: f64 = (s.dn[1099][4] + s.dn[1101][4]);let eq51_e1998_d_n5: f64 = (s.dn[1099][5] + s.dn[1101][5]);let eq51_e1998_d_n6: f64 = (s.dn[1099][6] + s.dn[1101][6]);let eq51_e1998_d_n7: f64 = (s.dn[1099][7] + s.dn[1101][7]);let eq51_e1998_d_n8: f64 = (s.dn[1099][8] + s.dn[1101][8]);let eq51_e1998_d_n9: f64 = (s.dn[1099][9] + s.dn[1101][9]);let eq51_e1998_d_n10: f64 = (s.dn[1099][10] + s.dn[1101][10]);let eq51_e1998_d_n11: f64 = (s.dn[1099][11] + s.dn[1101][11]);let eq51_e1998_d_n12: f64 = (s.dn[1099][12] + s.dn[1101][12]);let eq51_e1998_d_n13: f64 = (s.dn[1099][13] + s.dn[1101][13]);let eq51_e1998_d_b0: f64 = (s.db[1099][0] + s.db[1101][0]);let eq51_e1998_d_b1: f64 = (s.db[1099][1] + s.db[1101][1]);let eq51_e1998_d_b2: f64 = (s.db[1099][2] + s.db[1101][2]);let eq51_e1998_d_b3: f64 = (s.db[1099][3] + s.db[1101][3]);let eq51_e1998_d_b4: f64 = (s.db[1099][4] + s.db[1101][4]);let eq51_e1998_d_b5: f64 = (s.db[1099][5] + s.db[1101][5]);let eq51_e1998_d_b6: f64 = (s.db[1099][6] + s.db[1101][6]);let eq51_e1998_d_b7: f64 = (s.db[1099][7] + s.db[1101][7]);let eq51_e1998_d_b8: f64 = (s.db[1099][8] + s.db[1101][8]);let eq51_e1998_d_b9: f64 = (s.db[1099][9] + s.db[1101][9]);let eq51_e1998_d_b10: f64 = (s.db[1099][10] + s.db[1101][10]);let eq51_e1998_d_b11: f64 = (s.db[1099][11] + s.db[1101][11]);
        (eq51_e1998, eq51_e1998_d_n0, eq51_e1998_d_n1, eq51_e1998_d_n2, eq51_e1998_d_n3, eq51_e1998_d_n4, eq51_e1998_d_n5, eq51_e1998_d_n6, eq51_e1998_d_n7, eq51_e1998_d_n8, eq51_e1998_d_n9, eq51_e1998_d_n10, eq51_e1998_d_n11, eq51_e1998_d_n12, eq51_e1998_d_n13, eq51_e1998_d_b0, eq51_e1998_d_b1, eq51_e1998_d_b2, eq51_e1998_d_b3, eq51_e1998_d_b4, eq51_e1998_d_b5, eq51_e1998_d_b6, eq51_e1998_d_b7, eq51_e1998_d_b8, eq51_e1998_d_b9, eq51_e1998_d_b10, eq51_e1998_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2000;let eq51_node_derivatives: [f64; 14] = [eq51_e2000_d_n0, eq51_e2000_d_n1, eq51_e2000_d_n2, eq51_e2000_d_n3, eq51_e2000_d_n4, eq51_e2000_d_n5, eq51_e2000_d_n6, eq51_e2000_d_n7, eq51_e2000_d_n8, eq51_e2000_d_n9, eq51_e2000_d_n10, eq51_e2000_d_n11, eq51_e2000_d_n12, eq51_e2000_d_n13];let eq51_branch_derivatives: [f64; 12] = [eq51_e2000_d_b0, eq51_e2000_d_b1, eq51_e2000_d_b2, eq51_e2000_d_b3, eq51_e2000_d_b4, eq51_e2000_d_b5, eq51_e2000_d_b6, eq51_e2000_d_b7, eq51_e2000_d_b8, eq51_e2000_d_b9, eq51_e2000_d_b10, eq51_e2000_d_b11];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_15(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq52_e2006, eq52_e2006_d_n0, eq52_e2006_d_n1, eq52_e2006_d_n2, eq52_e2006_d_n3, eq52_e2006_d_n4, eq52_e2006_d_n5, eq52_e2006_d_n6, eq52_e2006_d_n7, eq52_e2006_d_n8, eq52_e2006_d_n9, eq52_e2006_d_n10, eq52_e2006_d_n11, eq52_e2006_d_n12, eq52_e2006_d_n13, eq52_e2006_d_b0, eq52_e2006_d_b1, eq52_e2006_d_b2, eq52_e2006_d_b3, eq52_e2006_d_b4, eq52_e2006_d_b5, eq52_e2006_d_b6, eq52_e2006_d_b7, eq52_e2006_d_b8, eq52_e2006_d_b9, eq52_e2006_d_b10, eq52_e2006_d_b11,) = {
    if s.b[2011] {
        let eq52_e2004: f64 = (s.v[1095] + s.v[1096]);let eq52_e2004_d_n0: f64 = (s.dn[1095][0] + s.dn[1096][0]);let eq52_e2004_d_n1: f64 = (s.dn[1095][1] + s.dn[1096][1]);let eq52_e2004_d_n2: f64 = (s.dn[1095][2] + s.dn[1096][2]);let eq52_e2004_d_n3: f64 = (s.dn[1095][3] + s.dn[1096][3]);let eq52_e2004_d_n4: f64 = (s.dn[1095][4] + s.dn[1096][4]);let eq52_e2004_d_n5: f64 = (s.dn[1095][5] + s.dn[1096][5]);let eq52_e2004_d_n6: f64 = (s.dn[1095][6] + s.dn[1096][6]);let eq52_e2004_d_n7: f64 = (s.dn[1095][7] + s.dn[1096][7]);let eq52_e2004_d_n8: f64 = (s.dn[1095][8] + s.dn[1096][8]);let eq52_e2004_d_n9: f64 = (s.dn[1095][9] + s.dn[1096][9]);let eq52_e2004_d_n10: f64 = (s.dn[1095][10] + s.dn[1096][10]);let eq52_e2004_d_n11: f64 = (s.dn[1095][11] + s.dn[1096][11]);let eq52_e2004_d_n12: f64 = (s.dn[1095][12] + s.dn[1096][12]);let eq52_e2004_d_n13: f64 = (s.dn[1095][13] + s.dn[1096][13]);let eq52_e2004_d_b0: f64 = (s.db[1095][0] + s.db[1096][0]);let eq52_e2004_d_b1: f64 = (s.db[1095][1] + s.db[1096][1]);let eq52_e2004_d_b2: f64 = (s.db[1095][2] + s.db[1096][2]);let eq52_e2004_d_b3: f64 = (s.db[1095][3] + s.db[1096][3]);let eq52_e2004_d_b4: f64 = (s.db[1095][4] + s.db[1096][4]);let eq52_e2004_d_b5: f64 = (s.db[1095][5] + s.db[1096][5]);let eq52_e2004_d_b6: f64 = (s.db[1095][6] + s.db[1096][6]);let eq52_e2004_d_b7: f64 = (s.db[1095][7] + s.db[1096][7]);let eq52_e2004_d_b8: f64 = (s.db[1095][8] + s.db[1096][8]);let eq52_e2004_d_b9: f64 = (s.db[1095][9] + s.db[1096][9]);let eq52_e2004_d_b10: f64 = (s.db[1095][10] + s.db[1096][10]);let eq52_e2004_d_b11: f64 = (s.db[1095][11] + s.db[1096][11]);
        (eq52_e2004, eq52_e2004_d_n0, eq52_e2004_d_n1, eq52_e2004_d_n2, eq52_e2004_d_n3, eq52_e2004_d_n4, eq52_e2004_d_n5, eq52_e2004_d_n6, eq52_e2004_d_n7, eq52_e2004_d_n8, eq52_e2004_d_n9, eq52_e2004_d_n10, eq52_e2004_d_n11, eq52_e2004_d_n12, eq52_e2004_d_n13, eq52_e2004_d_b0, eq52_e2004_d_b1, eq52_e2004_d_b2, eq52_e2004_d_b3, eq52_e2004_d_b4, eq52_e2004_d_b5, eq52_e2004_d_b6, eq52_e2004_d_b7, eq52_e2004_d_b8, eq52_e2004_d_b9, eq52_e2004_d_b10, eq52_e2004_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2006;let eq52_node_derivatives: [f64; 14] = [eq52_e2006_d_n0, eq52_e2006_d_n1, eq52_e2006_d_n2, eq52_e2006_d_n3, eq52_e2006_d_n4, eq52_e2006_d_n5, eq52_e2006_d_n6, eq52_e2006_d_n7, eq52_e2006_d_n8, eq52_e2006_d_n9, eq52_e2006_d_n10, eq52_e2006_d_n11, eq52_e2006_d_n12, eq52_e2006_d_n13];let eq52_branch_derivatives: [f64; 12] = [eq52_e2006_d_b0, eq52_e2006_d_b1, eq52_e2006_d_b2, eq52_e2006_d_b3, eq52_e2006_d_b4, eq52_e2006_d_b5, eq52_e2006_d_b6, eq52_e2006_d_b7, eq52_e2006_d_b8, eq52_e2006_d_b9, eq52_e2006_d_b10, eq52_e2006_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e2010, eq53_e2010_d_n0, eq53_e2010_d_n1, eq53_e2010_d_n2, eq53_e2010_d_n3, eq53_e2010_d_n4, eq53_e2010_d_n5, eq53_e2010_d_n6, eq53_e2010_d_n7, eq53_e2010_d_n8, eq53_e2010_d_n9, eq53_e2010_d_n10, eq53_e2010_d_n11, eq53_e2010_d_n12, eq53_e2010_d_n13, eq53_e2010_d_b0, eq53_e2010_d_b1, eq53_e2010_d_b2, eq53_e2010_d_b3, eq53_e2010_d_b4, eq53_e2010_d_b5, eq53_e2010_d_b6, eq53_e2010_d_b7, eq53_e2010_d_b8, eq53_e2010_d_b9, eq53_e2010_d_b10, eq53_e2010_d_b11,) = {
    if s.b[2011] {
        (s.v[1097], s.dn[1097][0], s.dn[1097][1], s.dn[1097][2], s.dn[1097][3], s.dn[1097][4], s.dn[1097][5], s.dn[1097][6], s.dn[1097][7], s.dn[1097][8], s.dn[1097][9], s.dn[1097][10], s.dn[1097][11], s.dn[1097][12], s.dn[1097][13], s.db[1097][0], s.db[1097][1], s.db[1097][2], s.db[1097][3], s.db[1097][4], s.db[1097][5], s.db[1097][6], s.db[1097][7], s.db[1097][8], s.db[1097][9], s.db[1097][10], s.db[1097][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2010;let eq53_node_derivatives: [f64; 14] = [eq53_e2010_d_n0, eq53_e2010_d_n1, eq53_e2010_d_n2, eq53_e2010_d_n3, eq53_e2010_d_n4, eq53_e2010_d_n5, eq53_e2010_d_n6, eq53_e2010_d_n7, eq53_e2010_d_n8, eq53_e2010_d_n9, eq53_e2010_d_n10, eq53_e2010_d_n11, eq53_e2010_d_n12, eq53_e2010_d_n13];let eq53_branch_derivatives: [f64; 12] = [eq53_e2010_d_b0, eq53_e2010_d_b1, eq53_e2010_d_b2, eq53_e2010_d_b3, eq53_e2010_d_b4, eq53_e2010_d_b5, eq53_e2010_d_b6, eq53_e2010_d_b7, eq53_e2010_d_b8, eq53_e2010_d_b9, eq53_e2010_d_b10, eq53_e2010_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2015, eq54_e2015_d_n0, eq54_e2015_d_n1, eq54_e2015_d_n2, eq54_e2015_d_n3, eq54_e2015_d_n4, eq54_e2015_d_n5, eq54_e2015_d_n6, eq54_e2015_d_n7, eq54_e2015_d_n8, eq54_e2015_d_n9, eq54_e2015_d_n10, eq54_e2015_d_n11, eq54_e2015_d_n12, eq54_e2015_d_n13, eq54_e2015_d_b0, eq54_e2015_d_b1, eq54_e2015_d_b2, eq54_e2015_d_b3, eq54_e2015_d_b4, eq54_e2015_d_b5, eq54_e2015_d_b6, eq54_e2015_d_b7, eq54_e2015_d_b8, eq54_e2015_d_b9, eq54_e2015_d_b10, eq54_e2015_d_b11,) = {
    if (!s.b[2011]) {
        (s.v[1096], s.dn[1096][0], s.dn[1096][1], s.dn[1096][2], s.dn[1096][3], s.dn[1096][4], s.dn[1096][5], s.dn[1096][6], s.dn[1096][7], s.dn[1096][8], s.dn[1096][9], s.dn[1096][10], s.dn[1096][11], s.dn[1096][12], s.dn[1096][13], s.db[1096][0], s.db[1096][1], s.db[1096][2], s.db[1096][3], s.db[1096][4], s.db[1096][5], s.db[1096][6], s.db[1096][7], s.db[1096][8], s.db[1096][9], s.db[1096][10], s.db[1096][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2015;let eq54_node_derivatives: [f64; 14] = [eq54_e2015_d_n0, eq54_e2015_d_n1, eq54_e2015_d_n2, eq54_e2015_d_n3, eq54_e2015_d_n4, eq54_e2015_d_n5, eq54_e2015_d_n6, eq54_e2015_d_n7, eq54_e2015_d_n8, eq54_e2015_d_n9, eq54_e2015_d_n10, eq54_e2015_d_n11, eq54_e2015_d_n12, eq54_e2015_d_n13];let eq54_branch_derivatives: [f64; 12] = [eq54_e2015_d_b0, eq54_e2015_d_b1, eq54_e2015_d_b2, eq54_e2015_d_b3, eq54_e2015_d_b4, eq54_e2015_d_b5, eq54_e2015_d_b6, eq54_e2015_d_b7, eq54_e2015_d_b8, eq54_e2015_d_b9, eq54_e2015_d_b10, eq54_e2015_d_b11];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(10),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2022, eq55_e2022_d_n0, eq55_e2022_d_n1, eq55_e2022_d_n2, eq55_e2022_d_n3, eq55_e2022_d_n4, eq55_e2022_d_n5, eq55_e2022_d_n6, eq55_e2022_d_n7, eq55_e2022_d_n8, eq55_e2022_d_n9, eq55_e2022_d_n10, eq55_e2022_d_n11, eq55_e2022_d_n12, eq55_e2022_d_n13, eq55_e2022_d_b0, eq55_e2022_d_b1, eq55_e2022_d_b2, eq55_e2022_d_b3, eq55_e2022_d_b4, eq55_e2022_d_b5, eq55_e2022_d_b6, eq55_e2022_d_b7, eq55_e2022_d_b8, eq55_e2022_d_b9, eq55_e2022_d_b10, eq55_e2022_d_b11,) = {
    if (!s.b[2011]) {
        let eq55_e2020: f64 = (s.v[1095] + s.v[1097]);let eq55_e2020_d_n0: f64 = (s.dn[1095][0] + s.dn[1097][0]);let eq55_e2020_d_n1: f64 = (s.dn[1095][1] + s.dn[1097][1]);let eq55_e2020_d_n2: f64 = (s.dn[1095][2] + s.dn[1097][2]);let eq55_e2020_d_n3: f64 = (s.dn[1095][3] + s.dn[1097][3]);let eq55_e2020_d_n4: f64 = (s.dn[1095][4] + s.dn[1097][4]);let eq55_e2020_d_n5: f64 = (s.dn[1095][5] + s.dn[1097][5]);let eq55_e2020_d_n6: f64 = (s.dn[1095][6] + s.dn[1097][6]);let eq55_e2020_d_n7: f64 = (s.dn[1095][7] + s.dn[1097][7]);let eq55_e2020_d_n8: f64 = (s.dn[1095][8] + s.dn[1097][8]);let eq55_e2020_d_n9: f64 = (s.dn[1095][9] + s.dn[1097][9]);let eq55_e2020_d_n10: f64 = (s.dn[1095][10] + s.dn[1097][10]);let eq55_e2020_d_n11: f64 = (s.dn[1095][11] + s.dn[1097][11]);let eq55_e2020_d_n12: f64 = (s.dn[1095][12] + s.dn[1097][12]);let eq55_e2020_d_n13: f64 = (s.dn[1095][13] + s.dn[1097][13]);let eq55_e2020_d_b0: f64 = (s.db[1095][0] + s.db[1097][0]);let eq55_e2020_d_b1: f64 = (s.db[1095][1] + s.db[1097][1]);let eq55_e2020_d_b2: f64 = (s.db[1095][2] + s.db[1097][2]);let eq55_e2020_d_b3: f64 = (s.db[1095][3] + s.db[1097][3]);let eq55_e2020_d_b4: f64 = (s.db[1095][4] + s.db[1097][4]);let eq55_e2020_d_b5: f64 = (s.db[1095][5] + s.db[1097][5]);let eq55_e2020_d_b6: f64 = (s.db[1095][6] + s.db[1097][6]);let eq55_e2020_d_b7: f64 = (s.db[1095][7] + s.db[1097][7]);let eq55_e2020_d_b8: f64 = (s.db[1095][8] + s.db[1097][8]);let eq55_e2020_d_b9: f64 = (s.db[1095][9] + s.db[1097][9]);let eq55_e2020_d_b10: f64 = (s.db[1095][10] + s.db[1097][10]);let eq55_e2020_d_b11: f64 = (s.db[1095][11] + s.db[1097][11]);
        (eq55_e2020, eq55_e2020_d_n0, eq55_e2020_d_n1, eq55_e2020_d_n2, eq55_e2020_d_n3, eq55_e2020_d_n4, eq55_e2020_d_n5, eq55_e2020_d_n6, eq55_e2020_d_n7, eq55_e2020_d_n8, eq55_e2020_d_n9, eq55_e2020_d_n10, eq55_e2020_d_n11, eq55_e2020_d_n12, eq55_e2020_d_n13, eq55_e2020_d_b0, eq55_e2020_d_b1, eq55_e2020_d_b2, eq55_e2020_d_b3, eq55_e2020_d_b4, eq55_e2020_d_b5, eq55_e2020_d_b6, eq55_e2020_d_b7, eq55_e2020_d_b8, eq55_e2020_d_b9, eq55_e2020_d_b10, eq55_e2020_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2022;let eq55_node_derivatives: [f64; 14] = [eq55_e2022_d_n0, eq55_e2022_d_n1, eq55_e2022_d_n2, eq55_e2022_d_n3, eq55_e2022_d_n4, eq55_e2022_d_n5, eq55_e2022_d_n6, eq55_e2022_d_n7, eq55_e2022_d_n8, eq55_e2022_d_n9, eq55_e2022_d_n10, eq55_e2022_d_n11, eq55_e2022_d_n12, eq55_e2022_d_n13];let eq55_branch_derivatives: [f64; 12] = [eq55_e2022_d_b0, eq55_e2022_d_b1, eq55_e2022_d_b2, eq55_e2022_d_b3, eq55_e2022_d_b4, eq55_e2022_d_b5, eq55_e2022_d_b6, eq55_e2022_d_b7, eq55_e2022_d_b8, eq55_e2022_d_b9, eq55_e2022_d_b10, eq55_e2022_d_b11];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(10),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2026,) = {
    if s.b[2012] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e2026;
        stamper.stamp_potential_const_local(
            2,
            eq56_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_16(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv1 = ctx.node_voltage(nodes[1]);let nv6 = ctx.node_voltage(nodes[6]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq57_e2033, eq57_e2033_d_n0, eq57_e2033_d_n1, eq57_e2033_d_n2, eq57_e2033_d_n3, eq57_e2033_d_n4, eq57_e2033_d_n5, eq57_e2033_d_n6, eq57_e2033_d_n7, eq57_e2033_d_n8, eq57_e2033_d_n9, eq57_e2033_d_n10, eq57_e2033_d_n11, eq57_e2033_d_n12, eq57_e2033_d_n13, eq57_e2033_d_b0, eq57_e2033_d_b1, eq57_e2033_d_b2, eq57_e2033_d_b3, eq57_e2033_d_b4, eq57_e2033_d_b5, eq57_e2033_d_b6, eq57_e2033_d_b7, eq57_e2033_d_b8, eq57_e2033_d_b9, eq57_e2033_d_b10, eq57_e2033_d_b11,) = {
    if (!s.b[2012]) {
        let eq57_e2031: f64 = ((nv1 - nv9) * s.v[2013]);let eq57_e2031_d_n0: f64 = ((nv1 - nv9) * s.dn[2013][0]);let eq57_e2031_d_n1: f64 = (s.v[2013] + ((nv1 - nv9) * s.dn[2013][1]));let eq57_e2031_d_n2: f64 = ((nv1 - nv9) * s.dn[2013][2]);let eq57_e2031_d_n3: f64 = ((nv1 - nv9) * s.dn[2013][3]);let eq57_e2031_d_n4: f64 = ((nv1 - nv9) * s.dn[2013][4]);let eq57_e2031_d_n5: f64 = ((nv1 - nv9) * s.dn[2013][5]);let eq57_e2031_d_n6: f64 = ((nv1 - nv9) * s.dn[2013][6]);let eq57_e2031_d_n7: f64 = ((nv1 - nv9) * s.dn[2013][7]);let eq57_e2031_d_n8: f64 = ((nv1 - nv9) * s.dn[2013][8]);let eq57_e2031_d_n9: f64 = ((-s.v[2013]) + ((nv1 - nv9) * s.dn[2013][9]));let eq57_e2031_d_n10: f64 = ((nv1 - nv9) * s.dn[2013][10]);let eq57_e2031_d_n11: f64 = ((nv1 - nv9) * s.dn[2013][11]);let eq57_e2031_d_n12: f64 = ((nv1 - nv9) * s.dn[2013][12]);let eq57_e2031_d_n13: f64 = ((nv1 - nv9) * s.dn[2013][13]);let eq57_e2031_d_b0: f64 = ((nv1 - nv9) * s.db[2013][0]);let eq57_e2031_d_b1: f64 = ((nv1 - nv9) * s.db[2013][1]);let eq57_e2031_d_b2: f64 = ((nv1 - nv9) * s.db[2013][2]);let eq57_e2031_d_b3: f64 = ((nv1 - nv9) * s.db[2013][3]);let eq57_e2031_d_b4: f64 = ((nv1 - nv9) * s.db[2013][4]);let eq57_e2031_d_b5: f64 = ((nv1 - nv9) * s.db[2013][5]);let eq57_e2031_d_b6: f64 = ((nv1 - nv9) * s.db[2013][6]);let eq57_e2031_d_b7: f64 = ((nv1 - nv9) * s.db[2013][7]);let eq57_e2031_d_b8: f64 = ((nv1 - nv9) * s.db[2013][8]);let eq57_e2031_d_b9: f64 = ((nv1 - nv9) * s.db[2013][9]);let eq57_e2031_d_b10: f64 = ((nv1 - nv9) * s.db[2013][10]);let eq57_e2031_d_b11: f64 = ((nv1 - nv9) * s.db[2013][11]);
        (eq57_e2031, eq57_e2031_d_n0, eq57_e2031_d_n1, eq57_e2031_d_n2, eq57_e2031_d_n3, eq57_e2031_d_n4, eq57_e2031_d_n5, eq57_e2031_d_n6, eq57_e2031_d_n7, eq57_e2031_d_n8, eq57_e2031_d_n9, eq57_e2031_d_n10, eq57_e2031_d_n11, eq57_e2031_d_n12, eq57_e2031_d_n13, eq57_e2031_d_b0, eq57_e2031_d_b1, eq57_e2031_d_b2, eq57_e2031_d_b3, eq57_e2031_d_b4, eq57_e2031_d_b5, eq57_e2031_d_b6, eq57_e2031_d_b7, eq57_e2031_d_b8, eq57_e2031_d_b9, eq57_e2031_d_b10, eq57_e2031_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2033;let eq57_node_derivatives: [f64; 14] = [eq57_e2033_d_n0, eq57_e2033_d_n1, eq57_e2033_d_n2, eq57_e2033_d_n3, eq57_e2033_d_n4, eq57_e2033_d_n5, eq57_e2033_d_n6, eq57_e2033_d_n7, eq57_e2033_d_n8, eq57_e2033_d_n9, eq57_e2033_d_n10, eq57_e2033_d_n11, eq57_e2033_d_n12, eq57_e2033_d_n13];let eq57_branch_derivatives: [f64; 12] = [eq57_e2033_d_b0, eq57_e2033_d_b1, eq57_e2033_d_b2, eq57_e2033_d_b3, eq57_e2033_d_b4, eq57_e2033_d_b5, eq57_e2033_d_b6, eq57_e2033_d_b7, eq57_e2033_d_b8, eq57_e2033_d_b9, eq57_e2033_d_b10, eq57_e2033_d_b11];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq59_e2048, eq59_e2048_d_n0, eq59_e2048_d_n1, eq59_e2048_d_n2, eq59_e2048_d_n3, eq59_e2048_d_n4, eq59_e2048_d_n5, eq59_e2048_d_n6, eq59_e2048_d_n7, eq59_e2048_d_n8, eq59_e2048_d_n9, eq59_e2048_d_n10, eq59_e2048_d_n11, eq59_e2048_d_n12, eq59_e2048_d_n13, eq59_e2048_d_b0, eq59_e2048_d_b1, eq59_e2048_d_b2, eq59_e2048_d_b3, eq59_e2048_d_b4, eq59_e2048_d_b5, eq59_e2048_d_b6, eq59_e2048_d_b7, eq59_e2048_d_b8, eq59_e2048_d_b9, eq59_e2048_d_b10, eq59_e2048_d_b11,) = {
    if s.b[2016] {
        let eq59_e2046: f64 = ((nv0 - nv6) * s.v[618]);let eq59_e2046_d_n0: f64 = (s.v[618] + ((nv0 - nv6) * s.dn[618][0]));let eq59_e2046_d_n1: f64 = ((nv0 - nv6) * s.dn[618][1]);let eq59_e2046_d_n2: f64 = ((nv0 - nv6) * s.dn[618][2]);let eq59_e2046_d_n3: f64 = ((nv0 - nv6) * s.dn[618][3]);let eq59_e2046_d_n4: f64 = ((nv0 - nv6) * s.dn[618][4]);let eq59_e2046_d_n5: f64 = ((nv0 - nv6) * s.dn[618][5]);let eq59_e2046_d_n6: f64 = ((-s.v[618]) + ((nv0 - nv6) * s.dn[618][6]));let eq59_e2046_d_n7: f64 = ((nv0 - nv6) * s.dn[618][7]);let eq59_e2046_d_n8: f64 = ((nv0 - nv6) * s.dn[618][8]);let eq59_e2046_d_n9: f64 = ((nv0 - nv6) * s.dn[618][9]);let eq59_e2046_d_n10: f64 = ((nv0 - nv6) * s.dn[618][10]);let eq59_e2046_d_n11: f64 = ((nv0 - nv6) * s.dn[618][11]);let eq59_e2046_d_n12: f64 = ((nv0 - nv6) * s.dn[618][12]);let eq59_e2046_d_n13: f64 = ((nv0 - nv6) * s.dn[618][13]);let eq59_e2046_d_b0: f64 = ((nv0 - nv6) * s.db[618][0]);let eq59_e2046_d_b1: f64 = ((nv0 - nv6) * s.db[618][1]);let eq59_e2046_d_b2: f64 = ((nv0 - nv6) * s.db[618][2]);let eq59_e2046_d_b3: f64 = ((nv0 - nv6) * s.db[618][3]);let eq59_e2046_d_b4: f64 = ((nv0 - nv6) * s.db[618][4]);let eq59_e2046_d_b5: f64 = ((nv0 - nv6) * s.db[618][5]);let eq59_e2046_d_b6: f64 = ((nv0 - nv6) * s.db[618][6]);let eq59_e2046_d_b7: f64 = ((nv0 - nv6) * s.db[618][7]);let eq59_e2046_d_b8: f64 = ((nv0 - nv6) * s.db[618][8]);let eq59_e2046_d_b9: f64 = ((nv0 - nv6) * s.db[618][9]);let eq59_e2046_d_b10: f64 = ((nv0 - nv6) * s.db[618][10]);let eq59_e2046_d_b11: f64 = ((nv0 - nv6) * s.db[618][11]);
        (eq59_e2046, eq59_e2046_d_n0, eq59_e2046_d_n1, eq59_e2046_d_n2, eq59_e2046_d_n3, eq59_e2046_d_n4, eq59_e2046_d_n5, eq59_e2046_d_n6, eq59_e2046_d_n7, eq59_e2046_d_n8, eq59_e2046_d_n9, eq59_e2046_d_n10, eq59_e2046_d_n11, eq59_e2046_d_n12, eq59_e2046_d_n13, eq59_e2046_d_b0, eq59_e2046_d_b1, eq59_e2046_d_b2, eq59_e2046_d_b3, eq59_e2046_d_b4, eq59_e2046_d_b5, eq59_e2046_d_b6, eq59_e2046_d_b7, eq59_e2046_d_b8, eq59_e2046_d_b9, eq59_e2046_d_b10, eq59_e2046_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2048;let eq59_node_derivatives: [f64; 14] = [eq59_e2048_d_n0, eq59_e2048_d_n1, eq59_e2048_d_n2, eq59_e2048_d_n3, eq59_e2048_d_n4, eq59_e2048_d_n5, eq59_e2048_d_n6, eq59_e2048_d_n7, eq59_e2048_d_n8, eq59_e2048_d_n9, eq59_e2048_d_n10, eq59_e2048_d_n11, eq59_e2048_d_n12, eq59_e2048_d_n13];let eq59_branch_derivatives: [f64; 12] = [eq59_e2048_d_b0, eq59_e2048_d_b1, eq59_e2048_d_b2, eq59_e2048_d_b3, eq59_e2048_d_b4, eq59_e2048_d_b5, eq59_e2048_d_b6, eq59_e2048_d_b7, eq59_e2048_d_b8, eq59_e2048_d_b9, eq59_e2048_d_b10, eq59_e2048_d_b11];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq59_value),
            &eq59_node_derivatives,
            &eq59_branch_derivatives,
            multiplicity,
        );
        let (eq60_e2053,) = {
    if (!s.b[2016]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2053;
        stamper.stamp_potential_const_local(
            3,
            eq60_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_17(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        let (eq62_e2067, eq62_e2067_d_n0, eq62_e2067_d_n1, eq62_e2067_d_n2, eq62_e2067_d_n3, eq62_e2067_d_n4, eq62_e2067_d_n5, eq62_e2067_d_n6, eq62_e2067_d_n7, eq62_e2067_d_n8, eq62_e2067_d_n9, eq62_e2067_d_n10, eq62_e2067_d_n11, eq62_e2067_d_n12, eq62_e2067_d_n13, eq62_e2067_d_b0, eq62_e2067_d_b1, eq62_e2067_d_b2, eq62_e2067_d_b3, eq62_e2067_d_b4, eq62_e2067_d_b5, eq62_e2067_d_b6, eq62_e2067_d_b7, eq62_e2067_d_b8, eq62_e2067_d_b9, eq62_e2067_d_b10, eq62_e2067_d_b11,) = {
    if s.b[2018] {
        let eq62_e2065: f64 = ((nv2 - nv7) * s.v[617]);let eq62_e2065_d_n0: f64 = ((nv2 - nv7) * s.dn[617][0]);let eq62_e2065_d_n1: f64 = ((nv2 - nv7) * s.dn[617][1]);let eq62_e2065_d_n2: f64 = (s.v[617] + ((nv2 - nv7) * s.dn[617][2]));let eq62_e2065_d_n3: f64 = ((nv2 - nv7) * s.dn[617][3]);let eq62_e2065_d_n4: f64 = ((nv2 - nv7) * s.dn[617][4]);let eq62_e2065_d_n5: f64 = ((nv2 - nv7) * s.dn[617][5]);let eq62_e2065_d_n6: f64 = ((nv2 - nv7) * s.dn[617][6]);let eq62_e2065_d_n7: f64 = ((-s.v[617]) + ((nv2 - nv7) * s.dn[617][7]));let eq62_e2065_d_n8: f64 = ((nv2 - nv7) * s.dn[617][8]);let eq62_e2065_d_n9: f64 = ((nv2 - nv7) * s.dn[617][9]);let eq62_e2065_d_n10: f64 = ((nv2 - nv7) * s.dn[617][10]);let eq62_e2065_d_n11: f64 = ((nv2 - nv7) * s.dn[617][11]);let eq62_e2065_d_n12: f64 = ((nv2 - nv7) * s.dn[617][12]);let eq62_e2065_d_n13: f64 = ((nv2 - nv7) * s.dn[617][13]);let eq62_e2065_d_b0: f64 = ((nv2 - nv7) * s.db[617][0]);let eq62_e2065_d_b1: f64 = ((nv2 - nv7) * s.db[617][1]);let eq62_e2065_d_b2: f64 = ((nv2 - nv7) * s.db[617][2]);let eq62_e2065_d_b3: f64 = ((nv2 - nv7) * s.db[617][3]);let eq62_e2065_d_b4: f64 = ((nv2 - nv7) * s.db[617][4]);let eq62_e2065_d_b5: f64 = ((nv2 - nv7) * s.db[617][5]);let eq62_e2065_d_b6: f64 = ((nv2 - nv7) * s.db[617][6]);let eq62_e2065_d_b7: f64 = ((nv2 - nv7) * s.db[617][7]);let eq62_e2065_d_b8: f64 = ((nv2 - nv7) * s.db[617][8]);let eq62_e2065_d_b9: f64 = ((nv2 - nv7) * s.db[617][9]);let eq62_e2065_d_b10: f64 = ((nv2 - nv7) * s.db[617][10]);let eq62_e2065_d_b11: f64 = ((nv2 - nv7) * s.db[617][11]);
        (eq62_e2065, eq62_e2065_d_n0, eq62_e2065_d_n1, eq62_e2065_d_n2, eq62_e2065_d_n3, eq62_e2065_d_n4, eq62_e2065_d_n5, eq62_e2065_d_n6, eq62_e2065_d_n7, eq62_e2065_d_n8, eq62_e2065_d_n9, eq62_e2065_d_n10, eq62_e2065_d_n11, eq62_e2065_d_n12, eq62_e2065_d_n13, eq62_e2065_d_b0, eq62_e2065_d_b1, eq62_e2065_d_b2, eq62_e2065_d_b3, eq62_e2065_d_b4, eq62_e2065_d_b5, eq62_e2065_d_b6, eq62_e2065_d_b7, eq62_e2065_d_b8, eq62_e2065_d_b9, eq62_e2065_d_b10, eq62_e2065_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2067;let eq62_node_derivatives: [f64; 14] = [eq62_e2067_d_n0, eq62_e2067_d_n1, eq62_e2067_d_n2, eq62_e2067_d_n3, eq62_e2067_d_n4, eq62_e2067_d_n5, eq62_e2067_d_n6, eq62_e2067_d_n7, eq62_e2067_d_n8, eq62_e2067_d_n9, eq62_e2067_d_n10, eq62_e2067_d_n11, eq62_e2067_d_n12, eq62_e2067_d_n13];let eq62_branch_derivatives: [f64; 12] = [eq62_e2067_d_b0, eq62_e2067_d_b1, eq62_e2067_d_b2, eq62_e2067_d_b3, eq62_e2067_d_b4, eq62_e2067_d_b5, eq62_e2067_d_b6, eq62_e2067_d_b7, eq62_e2067_d_b8, eq62_e2067_d_b9, eq62_e2067_d_b10, eq62_e2067_d_b11];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e2072,) = {
    if (!s.b[2018]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2072;
        stamper.stamp_potential_const_local(
            4,
            eq63_value,
        );
        let (eq65_e2086, eq65_e2086_d_n0, eq65_e2086_d_n1, eq65_e2086_d_n2, eq65_e2086_d_n3, eq65_e2086_d_n4, eq65_e2086_d_n5, eq65_e2086_d_n6, eq65_e2086_d_n7, eq65_e2086_d_n8, eq65_e2086_d_n9, eq65_e2086_d_n10, eq65_e2086_d_n11, eq65_e2086_d_n12, eq65_e2086_d_n13, eq65_e2086_d_b0, eq65_e2086_d_b1, eq65_e2086_d_b2, eq65_e2086_d_b3, eq65_e2086_d_b4, eq65_e2086_d_b5, eq65_e2086_d_b6, eq65_e2086_d_b7, eq65_e2086_d_b8, eq65_e2086_d_b9, eq65_e2086_d_b10, eq65_e2086_d_b11,) = {
    if s.b[2020] {
        let eq65_e2084: f64 = ((nv9 - nv8) * s.v[467]);let eq65_e2084_d_n0: f64 = ((nv9 - nv8) * s.dn[467][0]);let eq65_e2084_d_n1: f64 = ((nv9 - nv8) * s.dn[467][1]);let eq65_e2084_d_n2: f64 = ((nv9 - nv8) * s.dn[467][2]);let eq65_e2084_d_n3: f64 = ((nv9 - nv8) * s.dn[467][3]);let eq65_e2084_d_n4: f64 = ((nv9 - nv8) * s.dn[467][4]);let eq65_e2084_d_n5: f64 = ((nv9 - nv8) * s.dn[467][5]);let eq65_e2084_d_n6: f64 = ((nv9 - nv8) * s.dn[467][6]);let eq65_e2084_d_n7: f64 = ((nv9 - nv8) * s.dn[467][7]);let eq65_e2084_d_n8: f64 = ((-s.v[467]) + ((nv9 - nv8) * s.dn[467][8]));let eq65_e2084_d_n9: f64 = (s.v[467] + ((nv9 - nv8) * s.dn[467][9]));let eq65_e2084_d_n10: f64 = ((nv9 - nv8) * s.dn[467][10]);let eq65_e2084_d_n11: f64 = ((nv9 - nv8) * s.dn[467][11]);let eq65_e2084_d_n12: f64 = ((nv9 - nv8) * s.dn[467][12]);let eq65_e2084_d_n13: f64 = ((nv9 - nv8) * s.dn[467][13]);let eq65_e2084_d_b0: f64 = ((nv9 - nv8) * s.db[467][0]);let eq65_e2084_d_b1: f64 = ((nv9 - nv8) * s.db[467][1]);let eq65_e2084_d_b2: f64 = ((nv9 - nv8) * s.db[467][2]);let eq65_e2084_d_b3: f64 = ((nv9 - nv8) * s.db[467][3]);let eq65_e2084_d_b4: f64 = ((nv9 - nv8) * s.db[467][4]);let eq65_e2084_d_b5: f64 = ((nv9 - nv8) * s.db[467][5]);let eq65_e2084_d_b6: f64 = ((nv9 - nv8) * s.db[467][6]);let eq65_e2084_d_b7: f64 = ((nv9 - nv8) * s.db[467][7]);let eq65_e2084_d_b8: f64 = ((nv9 - nv8) * s.db[467][8]);let eq65_e2084_d_b9: f64 = ((nv9 - nv8) * s.db[467][9]);let eq65_e2084_d_b10: f64 = ((nv9 - nv8) * s.db[467][10]);let eq65_e2084_d_b11: f64 = ((nv9 - nv8) * s.db[467][11]);
        (eq65_e2084, eq65_e2084_d_n0, eq65_e2084_d_n1, eq65_e2084_d_n2, eq65_e2084_d_n3, eq65_e2084_d_n4, eq65_e2084_d_n5, eq65_e2084_d_n6, eq65_e2084_d_n7, eq65_e2084_d_n8, eq65_e2084_d_n9, eq65_e2084_d_n10, eq65_e2084_d_n11, eq65_e2084_d_n12, eq65_e2084_d_n13, eq65_e2084_d_b0, eq65_e2084_d_b1, eq65_e2084_d_b2, eq65_e2084_d_b3, eq65_e2084_d_b4, eq65_e2084_d_b5, eq65_e2084_d_b6, eq65_e2084_d_b7, eq65_e2084_d_b8, eq65_e2084_d_b9, eq65_e2084_d_b10, eq65_e2084_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e2086;let eq65_node_derivatives: [f64; 14] = [eq65_e2086_d_n0, eq65_e2086_d_n1, eq65_e2086_d_n2, eq65_e2086_d_n3, eq65_e2086_d_n4, eq65_e2086_d_n5, eq65_e2086_d_n6, eq65_e2086_d_n7, eq65_e2086_d_n8, eq65_e2086_d_n9, eq65_e2086_d_n10, eq65_e2086_d_n11, eq65_e2086_d_n12, eq65_e2086_d_n13];let eq65_branch_derivatives: [f64; 12] = [eq65_e2086_d_b0, eq65_e2086_d_b1, eq65_e2086_d_b2, eq65_e2086_d_b3, eq65_e2086_d_b4, eq65_e2086_d_b5, eq65_e2086_d_b6, eq65_e2086_d_b7, eq65_e2086_d_b8, eq65_e2086_d_b9, eq65_e2086_d_b10, eq65_e2086_d_b11];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq65_value),
            &eq65_node_derivatives,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let (eq66_e2091,) = {
    if (!s.b[2020]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2091;
        stamper.stamp_potential_const_local(
            5,
            eq66_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_18(
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
        let (eq67_e2108, eq67_e2108_d_n0, eq67_e2108_d_n1, eq67_e2108_d_n2, eq67_e2108_d_n3, eq67_e2108_d_n4, eq67_e2108_d_n5, eq67_e2108_d_n6, eq67_e2108_d_n7, eq67_e2108_d_n8, eq67_e2108_d_n9, eq67_e2108_d_n10, eq67_e2108_d_n11, eq67_e2108_d_n12, eq67_e2108_d_n13, eq67_e2108_d_b0, eq67_e2108_d_b1, eq67_e2108_d_b2, eq67_e2108_d_b3, eq67_e2108_d_b4, eq67_e2108_d_b5, eq67_e2108_d_b6, eq67_e2108_d_b7, eq67_e2108_d_b8, eq67_e2108_d_b9, eq67_e2108_d_b10, eq67_e2108_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && s.b[2025]) {
        let eq67_e2099: f64 = (s.v[634] * s.v[1015]);let eq67_e2099_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));let eq67_e2099_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));let eq67_e2099_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));let eq67_e2099_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));let eq67_e2099_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));let eq67_e2099_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));let eq67_e2099_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));let eq67_e2099_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));let eq67_e2099_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));let eq67_e2099_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));let eq67_e2099_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));let eq67_e2099_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));let eq67_e2099_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));let eq67_e2099_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));let eq67_e2099_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));let eq67_e2099_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));let eq67_e2099_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));let eq67_e2099_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));let eq67_e2099_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));let eq67_e2099_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));let eq67_e2099_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));let eq67_e2099_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));let eq67_e2099_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));let eq67_e2099_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));let eq67_e2099_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));let eq67_e2099_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));let eq67_e2102: f64 = (s.v[634] * s.v[1016]);let eq67_e2102_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));let eq67_e2102_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));let eq67_e2102_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));let eq67_e2102_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));let eq67_e2102_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));let eq67_e2102_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));let eq67_e2102_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));let eq67_e2102_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));let eq67_e2102_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));let eq67_e2102_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));let eq67_e2102_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));let eq67_e2102_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));let eq67_e2102_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));let eq67_e2102_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));let eq67_e2102_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));let eq67_e2102_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));let eq67_e2102_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));let eq67_e2102_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));let eq67_e2102_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));let eq67_e2102_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq67_e2102_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));let eq67_e2102_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));let eq67_e2102_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));let eq67_e2102_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));let eq67_e2102_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));let eq67_e2102_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));let eq67_e2103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq67_e2102);let eq67_e2104: f64 = (eq67_e2099 + eq67_e2103);let eq67_e2104_d_n0: f64 = (eq67_e2099_d_n0 + (eq67_e2102_d_n0 * ddt_scale));let eq67_e2104_d_n1: f64 = (eq67_e2099_d_n1 + (eq67_e2102_d_n1 * ddt_scale));let eq67_e2104_d_n2: f64 = (eq67_e2099_d_n2 + (eq67_e2102_d_n2 * ddt_scale));let eq67_e2104_d_n3: f64 = (eq67_e2099_d_n3 + (eq67_e2102_d_n3 * ddt_scale));let eq67_e2104_d_n4: f64 = (eq67_e2099_d_n4 + (eq67_e2102_d_n4 * ddt_scale));let eq67_e2104_d_n5: f64 = (eq67_e2099_d_n5 + (eq67_e2102_d_n5 * ddt_scale));let eq67_e2104_d_n6: f64 = (eq67_e2099_d_n6 + (eq67_e2102_d_n6 * ddt_scale));let eq67_e2104_d_n7: f64 = (eq67_e2099_d_n7 + (eq67_e2102_d_n7 * ddt_scale));let eq67_e2104_d_n8: f64 = (eq67_e2099_d_n8 + (eq67_e2102_d_n8 * ddt_scale));let eq67_e2104_d_n9: f64 = (eq67_e2099_d_n9 + (eq67_e2102_d_n9 * ddt_scale));let eq67_e2104_d_n10: f64 = (eq67_e2099_d_n10 + (eq67_e2102_d_n10 * ddt_scale));let eq67_e2104_d_n11: f64 = (eq67_e2099_d_n11 + (eq67_e2102_d_n11 * ddt_scale));let eq67_e2104_d_n12: f64 = (eq67_e2099_d_n12 + (eq67_e2102_d_n12 * ddt_scale));let eq67_e2104_d_n13: f64 = (eq67_e2099_d_n13 + (eq67_e2102_d_n13 * ddt_scale));let eq67_e2104_d_b0: f64 = (eq67_e2099_d_b0 + (eq67_e2102_d_b0 * ddt_scale));let eq67_e2104_d_b1: f64 = (eq67_e2099_d_b1 + (eq67_e2102_d_b1 * ddt_scale));let eq67_e2104_d_b2: f64 = (eq67_e2099_d_b2 + (eq67_e2102_d_b2 * ddt_scale));let eq67_e2104_d_b3: f64 = (eq67_e2099_d_b3 + (eq67_e2102_d_b3 * ddt_scale));let eq67_e2104_d_b4: f64 = (eq67_e2099_d_b4 + (eq67_e2102_d_b4 * ddt_scale));let eq67_e2104_d_b5: f64 = (eq67_e2099_d_b5 + (eq67_e2102_d_b5 * ddt_scale));let eq67_e2104_d_b6: f64 = (eq67_e2099_d_b6 + (eq67_e2102_d_b6 * ddt_scale));let eq67_e2104_d_b7: f64 = (eq67_e2099_d_b7 + (eq67_e2102_d_b7 * ddt_scale));let eq67_e2104_d_b8: f64 = (eq67_e2099_d_b8 + (eq67_e2102_d_b8 * ddt_scale));let eq67_e2104_d_b9: f64 = (eq67_e2099_d_b9 + (eq67_e2102_d_b9 * ddt_scale));let eq67_e2104_d_b10: f64 = (eq67_e2099_d_b10 + (eq67_e2102_d_b10 * ddt_scale));let eq67_e2104_d_b11: f64 = (eq67_e2099_d_b11 + (eq67_e2102_d_b11 * ddt_scale));let eq67_e2106: f64 = (eq67_e2104 - s.v[1017]);let eq67_e2106_d_n0: f64 = (eq67_e2104_d_n0 - s.dn[1017][0]);let eq67_e2106_d_n1: f64 = (eq67_e2104_d_n1 - s.dn[1017][1]);let eq67_e2106_d_n2: f64 = (eq67_e2104_d_n2 - s.dn[1017][2]);let eq67_e2106_d_n3: f64 = (eq67_e2104_d_n3 - s.dn[1017][3]);let eq67_e2106_d_n4: f64 = (eq67_e2104_d_n4 - s.dn[1017][4]);let eq67_e2106_d_n5: f64 = (eq67_e2104_d_n5 - s.dn[1017][5]);let eq67_e2106_d_n6: f64 = (eq67_e2104_d_n6 - s.dn[1017][6]);let eq67_e2106_d_n7: f64 = (eq67_e2104_d_n7 - s.dn[1017][7]);let eq67_e2106_d_n8: f64 = (eq67_e2104_d_n8 - s.dn[1017][8]);let eq67_e2106_d_n9: f64 = (eq67_e2104_d_n9 - s.dn[1017][9]);let eq67_e2106_d_n10: f64 = (eq67_e2104_d_n10 - s.dn[1017][10]);let eq67_e2106_d_n11: f64 = (eq67_e2104_d_n11 - s.dn[1017][11]);let eq67_e2106_d_n12: f64 = (eq67_e2104_d_n12 - s.dn[1017][12]);let eq67_e2106_d_n13: f64 = (eq67_e2104_d_n13 - s.dn[1017][13]);let eq67_e2106_d_b0: f64 = (eq67_e2104_d_b0 - s.db[1017][0]);let eq67_e2106_d_b1: f64 = (eq67_e2104_d_b1 - s.db[1017][1]);let eq67_e2106_d_b2: f64 = (eq67_e2104_d_b2 - s.db[1017][2]);let eq67_e2106_d_b3: f64 = (eq67_e2104_d_b3 - s.db[1017][3]);let eq67_e2106_d_b4: f64 = (eq67_e2104_d_b4 - s.db[1017][4]);
        let eq67_e2106_d_b5: f64 = (eq67_e2104_d_b5 - s.db[1017][5]);let eq67_e2106_d_b6: f64 = (eq67_e2104_d_b6 - s.db[1017][6]);let eq67_e2106_d_b7: f64 = (eq67_e2104_d_b7 - s.db[1017][7]);let eq67_e2106_d_b8: f64 = (eq67_e2104_d_b8 - s.db[1017][8]);let eq67_e2106_d_b9: f64 = (eq67_e2104_d_b9 - s.db[1017][9]);let eq67_e2106_d_b10: f64 = (eq67_e2104_d_b10 - s.db[1017][10]);let eq67_e2106_d_b11: f64 = (eq67_e2104_d_b11 - s.db[1017][11]);
        (eq67_e2106, eq67_e2106_d_n0, eq67_e2106_d_n1, eq67_e2106_d_n2, eq67_e2106_d_n3, eq67_e2106_d_n4, eq67_e2106_d_n5, eq67_e2106_d_n6, eq67_e2106_d_n7, eq67_e2106_d_n8, eq67_e2106_d_n9, eq67_e2106_d_n10, eq67_e2106_d_n11, eq67_e2106_d_n12, eq67_e2106_d_n13, eq67_e2106_d_b0, eq67_e2106_d_b1, eq67_e2106_d_b2, eq67_e2106_d_b3, eq67_e2106_d_b4, eq67_e2106_d_b5, eq67_e2106_d_b6, eq67_e2106_d_b7, eq67_e2106_d_b8, eq67_e2106_d_b9, eq67_e2106_d_b10, eq67_e2106_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2108;let eq67_node_derivatives: [f64; 14] = [eq67_e2108_d_n0, eq67_e2108_d_n1, eq67_e2108_d_n2, eq67_e2108_d_n3, eq67_e2108_d_n4, eq67_e2108_d_n5, eq67_e2108_d_n6, eq67_e2108_d_n7, eq67_e2108_d_n8, eq67_e2108_d_n9, eq67_e2108_d_n10, eq67_e2108_d_n11, eq67_e2108_d_n12, eq67_e2108_d_n13];let eq67_branch_derivatives: [f64; 12] = [eq67_e2108_d_b0, eq67_e2108_d_b1, eq67_e2108_d_b2, eq67_e2108_d_b3, eq67_e2108_d_b4, eq67_e2108_d_b5, eq67_e2108_d_b6, eq67_e2108_d_b7, eq67_e2108_d_b8, eq67_e2108_d_b9, eq67_e2108_d_b10, eq67_e2108_d_b11];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq67_value),
            &eq67_node_derivatives,
            &eq67_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_19(
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
        let (eq68_e2126, eq68_e2126_d_n0, eq68_e2126_d_n1, eq68_e2126_d_n2, eq68_e2126_d_n3, eq68_e2126_d_n4, eq68_e2126_d_n5, eq68_e2126_d_n6, eq68_e2126_d_n7, eq68_e2126_d_n8, eq68_e2126_d_n9, eq68_e2126_d_n10, eq68_e2126_d_n11, eq68_e2126_d_n12, eq68_e2126_d_n13, eq68_e2126_d_b0, eq68_e2126_d_b1, eq68_e2126_d_b2, eq68_e2126_d_b3, eq68_e2126_d_b4, eq68_e2126_d_b5, eq68_e2126_d_b6, eq68_e2126_d_b7, eq68_e2126_d_b8, eq68_e2126_d_b9, eq68_e2126_d_b10, eq68_e2126_d_b11,) = {
    if ((s.b[2021] && s.b[2024]) && (!s.b[2025])) {
        let eq68_e2117: f64 = (s.v[634] * s.v[1015]);let eq68_e2117_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));let eq68_e2117_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));let eq68_e2117_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));let eq68_e2117_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));let eq68_e2117_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));let eq68_e2117_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));let eq68_e2117_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));let eq68_e2117_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));let eq68_e2117_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));let eq68_e2117_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));let eq68_e2117_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));let eq68_e2117_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));let eq68_e2117_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));let eq68_e2117_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));let eq68_e2117_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));let eq68_e2117_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));let eq68_e2117_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));let eq68_e2117_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));let eq68_e2117_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));let eq68_e2117_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));let eq68_e2117_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));let eq68_e2117_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));let eq68_e2117_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));let eq68_e2117_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));let eq68_e2117_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));let eq68_e2117_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));let eq68_e2120: f64 = (s.v[634] * s.v[1016]);let eq68_e2120_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));let eq68_e2120_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));let eq68_e2120_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));let eq68_e2120_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));let eq68_e2120_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));let eq68_e2120_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));let eq68_e2120_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));let eq68_e2120_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));let eq68_e2120_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));let eq68_e2120_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));let eq68_e2120_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));let eq68_e2120_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));let eq68_e2120_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));let eq68_e2120_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));let eq68_e2120_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));let eq68_e2120_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));let eq68_e2120_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));let eq68_e2120_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));let eq68_e2120_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));let eq68_e2120_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq68_e2120_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));let eq68_e2120_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));let eq68_e2120_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));let eq68_e2120_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));let eq68_e2120_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));let eq68_e2120_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));let eq68_e2121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq68_e2120);let eq68_e2122: f64 = (eq68_e2117 + eq68_e2121);let eq68_e2122_d_n0: f64 = (eq68_e2117_d_n0 + (eq68_e2120_d_n0 * ddt_scale));let eq68_e2122_d_n1: f64 = (eq68_e2117_d_n1 + (eq68_e2120_d_n1 * ddt_scale));let eq68_e2122_d_n2: f64 = (eq68_e2117_d_n2 + (eq68_e2120_d_n2 * ddt_scale));let eq68_e2122_d_n3: f64 = (eq68_e2117_d_n3 + (eq68_e2120_d_n3 * ddt_scale));let eq68_e2122_d_n4: f64 = (eq68_e2117_d_n4 + (eq68_e2120_d_n4 * ddt_scale));let eq68_e2122_d_n5: f64 = (eq68_e2117_d_n5 + (eq68_e2120_d_n5 * ddt_scale));let eq68_e2122_d_n6: f64 = (eq68_e2117_d_n6 + (eq68_e2120_d_n6 * ddt_scale));let eq68_e2122_d_n7: f64 = (eq68_e2117_d_n7 + (eq68_e2120_d_n7 * ddt_scale));let eq68_e2122_d_n8: f64 = (eq68_e2117_d_n8 + (eq68_e2120_d_n8 * ddt_scale));let eq68_e2122_d_n9: f64 = (eq68_e2117_d_n9 + (eq68_e2120_d_n9 * ddt_scale));let eq68_e2122_d_n10: f64 = (eq68_e2117_d_n10 + (eq68_e2120_d_n10 * ddt_scale));let eq68_e2122_d_n11: f64 = (eq68_e2117_d_n11 + (eq68_e2120_d_n11 * ddt_scale));let eq68_e2122_d_n12: f64 = (eq68_e2117_d_n12 + (eq68_e2120_d_n12 * ddt_scale));let eq68_e2122_d_n13: f64 = (eq68_e2117_d_n13 + (eq68_e2120_d_n13 * ddt_scale));let eq68_e2122_d_b0: f64 = (eq68_e2117_d_b0 + (eq68_e2120_d_b0 * ddt_scale));let eq68_e2122_d_b1: f64 = (eq68_e2117_d_b1 + (eq68_e2120_d_b1 * ddt_scale));let eq68_e2122_d_b2: f64 = (eq68_e2117_d_b2 + (eq68_e2120_d_b2 * ddt_scale));let eq68_e2122_d_b3: f64 = (eq68_e2117_d_b3 + (eq68_e2120_d_b3 * ddt_scale));let eq68_e2122_d_b4: f64 = (eq68_e2117_d_b4 + (eq68_e2120_d_b4 * ddt_scale));let eq68_e2122_d_b5: f64 = (eq68_e2117_d_b5 + (eq68_e2120_d_b5 * ddt_scale));let eq68_e2122_d_b6: f64 = (eq68_e2117_d_b6 + (eq68_e2120_d_b6 * ddt_scale));let eq68_e2122_d_b7: f64 = (eq68_e2117_d_b7 + (eq68_e2120_d_b7 * ddt_scale));let eq68_e2122_d_b8: f64 = (eq68_e2117_d_b8 + (eq68_e2120_d_b8 * ddt_scale));let eq68_e2122_d_b9: f64 = (eq68_e2117_d_b9 + (eq68_e2120_d_b9 * ddt_scale));let eq68_e2122_d_b10: f64 = (eq68_e2117_d_b10 + (eq68_e2120_d_b10 * ddt_scale));let eq68_e2122_d_b11: f64 = (eq68_e2117_d_b11 + (eq68_e2120_d_b11 * ddt_scale));let eq68_e2124: f64 = (eq68_e2122 - s.v[1017]);let eq68_e2124_d_n0: f64 = (eq68_e2122_d_n0 - s.dn[1017][0]);let eq68_e2124_d_n1: f64 = (eq68_e2122_d_n1 - s.dn[1017][1]);let eq68_e2124_d_n2: f64 = (eq68_e2122_d_n2 - s.dn[1017][2]);let eq68_e2124_d_n3: f64 = (eq68_e2122_d_n3 - s.dn[1017][3]);let eq68_e2124_d_n4: f64 = (eq68_e2122_d_n4 - s.dn[1017][4]);let eq68_e2124_d_n5: f64 = (eq68_e2122_d_n5 - s.dn[1017][5]);let eq68_e2124_d_n6: f64 = (eq68_e2122_d_n6 - s.dn[1017][6]);let eq68_e2124_d_n7: f64 = (eq68_e2122_d_n7 - s.dn[1017][7]);let eq68_e2124_d_n8: f64 = (eq68_e2122_d_n8 - s.dn[1017][8]);let eq68_e2124_d_n9: f64 = (eq68_e2122_d_n9 - s.dn[1017][9]);let eq68_e2124_d_n10: f64 = (eq68_e2122_d_n10 - s.dn[1017][10]);let eq68_e2124_d_n11: f64 = (eq68_e2122_d_n11 - s.dn[1017][11]);let eq68_e2124_d_n12: f64 = (eq68_e2122_d_n12 - s.dn[1017][12]);let eq68_e2124_d_n13: f64 = (eq68_e2122_d_n13 - s.dn[1017][13]);let eq68_e2124_d_b0: f64 = (eq68_e2122_d_b0 - s.db[1017][0]);let eq68_e2124_d_b1: f64 = (eq68_e2122_d_b1 - s.db[1017][1]);let eq68_e2124_d_b2: f64 = (eq68_e2122_d_b2 - s.db[1017][2]);let eq68_e2124_d_b3: f64 = (eq68_e2122_d_b3 - s.db[1017][3]);let eq68_e2124_d_b4: f64 = (eq68_e2122_d_b4 - s.db[1017][4]);
        let eq68_e2124_d_b5: f64 = (eq68_e2122_d_b5 - s.db[1017][5]);let eq68_e2124_d_b6: f64 = (eq68_e2122_d_b6 - s.db[1017][6]);let eq68_e2124_d_b7: f64 = (eq68_e2122_d_b7 - s.db[1017][7]);let eq68_e2124_d_b8: f64 = (eq68_e2122_d_b8 - s.db[1017][8]);let eq68_e2124_d_b9: f64 = (eq68_e2122_d_b9 - s.db[1017][9]);let eq68_e2124_d_b10: f64 = (eq68_e2122_d_b10 - s.db[1017][10]);let eq68_e2124_d_b11: f64 = (eq68_e2122_d_b11 - s.db[1017][11]);
        (eq68_e2124, eq68_e2124_d_n0, eq68_e2124_d_n1, eq68_e2124_d_n2, eq68_e2124_d_n3, eq68_e2124_d_n4, eq68_e2124_d_n5, eq68_e2124_d_n6, eq68_e2124_d_n7, eq68_e2124_d_n8, eq68_e2124_d_n9, eq68_e2124_d_n10, eq68_e2124_d_n11, eq68_e2124_d_n12, eq68_e2124_d_n13, eq68_e2124_d_b0, eq68_e2124_d_b1, eq68_e2124_d_b2, eq68_e2124_d_b3, eq68_e2124_d_b4, eq68_e2124_d_b5, eq68_e2124_d_b6, eq68_e2124_d_b7, eq68_e2124_d_b8, eq68_e2124_d_b9, eq68_e2124_d_b10, eq68_e2124_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2126;let eq68_node_derivatives: [f64; 14] = [eq68_e2126_d_n0, eq68_e2126_d_n1, eq68_e2126_d_n2, eq68_e2126_d_n3, eq68_e2126_d_n4, eq68_e2126_d_n5, eq68_e2126_d_n6, eq68_e2126_d_n7, eq68_e2126_d_n8, eq68_e2126_d_n9, eq68_e2126_d_n10, eq68_e2126_d_n11, eq68_e2126_d_n12, eq68_e2126_d_n13];let eq68_branch_derivatives: [f64; 12] = [eq68_e2126_d_b0, eq68_e2126_d_b1, eq68_e2126_d_b2, eq68_e2126_d_b3, eq68_e2126_d_b4, eq68_e2126_d_b5, eq68_e2126_d_b6, eq68_e2126_d_b7, eq68_e2126_d_b8, eq68_e2126_d_b9, eq68_e2126_d_b10, eq68_e2126_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq68_value),
            &eq68_node_derivatives,
            &eq68_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_20(
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
        let (eq69_e2142, eq69_e2142_d_n0, eq69_e2142_d_n1, eq69_e2142_d_n2, eq69_e2142_d_n3, eq69_e2142_d_n4, eq69_e2142_d_n5, eq69_e2142_d_n6, eq69_e2142_d_n7, eq69_e2142_d_n8, eq69_e2142_d_n9, eq69_e2142_d_n10, eq69_e2142_d_n11, eq69_e2142_d_n12, eq69_e2142_d_n13, eq69_e2142_d_b0, eq69_e2142_d_b1, eq69_e2142_d_b2, eq69_e2142_d_b3, eq69_e2142_d_b4, eq69_e2142_d_b5, eq69_e2142_d_b6, eq69_e2142_d_b7, eq69_e2142_d_b8, eq69_e2142_d_b9, eq69_e2142_d_b10, eq69_e2142_d_b11,) = {
    if (s.b[2021] && (!s.b[2024])) {
        let eq69_e2133: f64 = (s.v[634] * s.v[1015]);let eq69_e2133_d_n0: f64 = ((s.dn[634][0] * s.v[1015]) + (s.v[634] * s.dn[1015][0]));let eq69_e2133_d_n1: f64 = ((s.dn[634][1] * s.v[1015]) + (s.v[634] * s.dn[1015][1]));let eq69_e2133_d_n2: f64 = ((s.dn[634][2] * s.v[1015]) + (s.v[634] * s.dn[1015][2]));let eq69_e2133_d_n3: f64 = ((s.dn[634][3] * s.v[1015]) + (s.v[634] * s.dn[1015][3]));let eq69_e2133_d_n4: f64 = ((s.dn[634][4] * s.v[1015]) + (s.v[634] * s.dn[1015][4]));let eq69_e2133_d_n5: f64 = ((s.dn[634][5] * s.v[1015]) + (s.v[634] * s.dn[1015][5]));let eq69_e2133_d_n6: f64 = ((s.dn[634][6] * s.v[1015]) + (s.v[634] * s.dn[1015][6]));let eq69_e2133_d_n7: f64 = ((s.dn[634][7] * s.v[1015]) + (s.v[634] * s.dn[1015][7]));let eq69_e2133_d_n8: f64 = ((s.dn[634][8] * s.v[1015]) + (s.v[634] * s.dn[1015][8]));let eq69_e2133_d_n9: f64 = ((s.dn[634][9] * s.v[1015]) + (s.v[634] * s.dn[1015][9]));let eq69_e2133_d_n10: f64 = ((s.dn[634][10] * s.v[1015]) + (s.v[634] * s.dn[1015][10]));let eq69_e2133_d_n11: f64 = ((s.dn[634][11] * s.v[1015]) + (s.v[634] * s.dn[1015][11]));let eq69_e2133_d_n12: f64 = ((s.dn[634][12] * s.v[1015]) + (s.v[634] * s.dn[1015][12]));let eq69_e2133_d_n13: f64 = ((s.dn[634][13] * s.v[1015]) + (s.v[634] * s.dn[1015][13]));let eq69_e2133_d_b0: f64 = ((s.db[634][0] * s.v[1015]) + (s.v[634] * s.db[1015][0]));let eq69_e2133_d_b1: f64 = ((s.db[634][1] * s.v[1015]) + (s.v[634] * s.db[1015][1]));let eq69_e2133_d_b2: f64 = ((s.db[634][2] * s.v[1015]) + (s.v[634] * s.db[1015][2]));let eq69_e2133_d_b3: f64 = ((s.db[634][3] * s.v[1015]) + (s.v[634] * s.db[1015][3]));let eq69_e2133_d_b4: f64 = ((s.db[634][4] * s.v[1015]) + (s.v[634] * s.db[1015][4]));let eq69_e2133_d_b5: f64 = ((s.db[634][5] * s.v[1015]) + (s.v[634] * s.db[1015][5]));let eq69_e2133_d_b6: f64 = ((s.db[634][6] * s.v[1015]) + (s.v[634] * s.db[1015][6]));let eq69_e2133_d_b7: f64 = ((s.db[634][7] * s.v[1015]) + (s.v[634] * s.db[1015][7]));let eq69_e2133_d_b8: f64 = ((s.db[634][8] * s.v[1015]) + (s.v[634] * s.db[1015][8]));let eq69_e2133_d_b9: f64 = ((s.db[634][9] * s.v[1015]) + (s.v[634] * s.db[1015][9]));let eq69_e2133_d_b10: f64 = ((s.db[634][10] * s.v[1015]) + (s.v[634] * s.db[1015][10]));let eq69_e2133_d_b11: f64 = ((s.db[634][11] * s.v[1015]) + (s.v[634] * s.db[1015][11]));let eq69_e2136: f64 = (s.v[634] * s.v[1016]);let eq69_e2136_d_n0: f64 = ((s.dn[634][0] * s.v[1016]) + (s.v[634] * s.dn[1016][0]));let eq69_e2136_d_n1: f64 = ((s.dn[634][1] * s.v[1016]) + (s.v[634] * s.dn[1016][1]));let eq69_e2136_d_n2: f64 = ((s.dn[634][2] * s.v[1016]) + (s.v[634] * s.dn[1016][2]));let eq69_e2136_d_n3: f64 = ((s.dn[634][3] * s.v[1016]) + (s.v[634] * s.dn[1016][3]));let eq69_e2136_d_n4: f64 = ((s.dn[634][4] * s.v[1016]) + (s.v[634] * s.dn[1016][4]));let eq69_e2136_d_n5: f64 = ((s.dn[634][5] * s.v[1016]) + (s.v[634] * s.dn[1016][5]));let eq69_e2136_d_n6: f64 = ((s.dn[634][6] * s.v[1016]) + (s.v[634] * s.dn[1016][6]));let eq69_e2136_d_n7: f64 = ((s.dn[634][7] * s.v[1016]) + (s.v[634] * s.dn[1016][7]));let eq69_e2136_d_n8: f64 = ((s.dn[634][8] * s.v[1016]) + (s.v[634] * s.dn[1016][8]));let eq69_e2136_d_n9: f64 = ((s.dn[634][9] * s.v[1016]) + (s.v[634] * s.dn[1016][9]));let eq69_e2136_d_n10: f64 = ((s.dn[634][10] * s.v[1016]) + (s.v[634] * s.dn[1016][10]));let eq69_e2136_d_n11: f64 = ((s.dn[634][11] * s.v[1016]) + (s.v[634] * s.dn[1016][11]));let eq69_e2136_d_n12: f64 = ((s.dn[634][12] * s.v[1016]) + (s.v[634] * s.dn[1016][12]));let eq69_e2136_d_n13: f64 = ((s.dn[634][13] * s.v[1016]) + (s.v[634] * s.dn[1016][13]));let eq69_e2136_d_b0: f64 = ((s.db[634][0] * s.v[1016]) + (s.v[634] * s.db[1016][0]));let eq69_e2136_d_b1: f64 = ((s.db[634][1] * s.v[1016]) + (s.v[634] * s.db[1016][1]));let eq69_e2136_d_b2: f64 = ((s.db[634][2] * s.v[1016]) + (s.v[634] * s.db[1016][2]));let eq69_e2136_d_b3: f64 = ((s.db[634][3] * s.v[1016]) + (s.v[634] * s.db[1016][3]));let eq69_e2136_d_b4: f64 = ((s.db[634][4] * s.v[1016]) + (s.v[634] * s.db[1016][4]));let eq69_e2136_d_b5: f64 = ((s.db[634][5] * s.v[1016]) + (s.v[634] * s.db[1016][5]));
        let eq69_e2136_d_b6: f64 = ((s.db[634][6] * s.v[1016]) + (s.v[634] * s.db[1016][6]));let eq69_e2136_d_b7: f64 = ((s.db[634][7] * s.v[1016]) + (s.v[634] * s.db[1016][7]));let eq69_e2136_d_b8: f64 = ((s.db[634][8] * s.v[1016]) + (s.v[634] * s.db[1016][8]));let eq69_e2136_d_b9: f64 = ((s.db[634][9] * s.v[1016]) + (s.v[634] * s.db[1016][9]));let eq69_e2136_d_b10: f64 = ((s.db[634][10] * s.v[1016]) + (s.v[634] * s.db[1016][10]));let eq69_e2136_d_b11: f64 = ((s.db[634][11] * s.v[1016]) + (s.v[634] * s.db[1016][11]));let eq69_e2137: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq69_e2136);let eq69_e2138: f64 = (eq69_e2133 + eq69_e2137);let eq69_e2138_d_n0: f64 = (eq69_e2133_d_n0 + (eq69_e2136_d_n0 * ddt_scale));let eq69_e2138_d_n1: f64 = (eq69_e2133_d_n1 + (eq69_e2136_d_n1 * ddt_scale));let eq69_e2138_d_n2: f64 = (eq69_e2133_d_n2 + (eq69_e2136_d_n2 * ddt_scale));let eq69_e2138_d_n3: f64 = (eq69_e2133_d_n3 + (eq69_e2136_d_n3 * ddt_scale));let eq69_e2138_d_n4: f64 = (eq69_e2133_d_n4 + (eq69_e2136_d_n4 * ddt_scale));let eq69_e2138_d_n5: f64 = (eq69_e2133_d_n5 + (eq69_e2136_d_n5 * ddt_scale));let eq69_e2138_d_n6: f64 = (eq69_e2133_d_n6 + (eq69_e2136_d_n6 * ddt_scale));let eq69_e2138_d_n7: f64 = (eq69_e2133_d_n7 + (eq69_e2136_d_n7 * ddt_scale));let eq69_e2138_d_n8: f64 = (eq69_e2133_d_n8 + (eq69_e2136_d_n8 * ddt_scale));let eq69_e2138_d_n9: f64 = (eq69_e2133_d_n9 + (eq69_e2136_d_n9 * ddt_scale));let eq69_e2138_d_n10: f64 = (eq69_e2133_d_n10 + (eq69_e2136_d_n10 * ddt_scale));let eq69_e2138_d_n11: f64 = (eq69_e2133_d_n11 + (eq69_e2136_d_n11 * ddt_scale));let eq69_e2138_d_n12: f64 = (eq69_e2133_d_n12 + (eq69_e2136_d_n12 * ddt_scale));let eq69_e2138_d_n13: f64 = (eq69_e2133_d_n13 + (eq69_e2136_d_n13 * ddt_scale));let eq69_e2138_d_b0: f64 = (eq69_e2133_d_b0 + (eq69_e2136_d_b0 * ddt_scale));let eq69_e2138_d_b1: f64 = (eq69_e2133_d_b1 + (eq69_e2136_d_b1 * ddt_scale));let eq69_e2138_d_b2: f64 = (eq69_e2133_d_b2 + (eq69_e2136_d_b2 * ddt_scale));let eq69_e2138_d_b3: f64 = (eq69_e2133_d_b3 + (eq69_e2136_d_b3 * ddt_scale));let eq69_e2138_d_b4: f64 = (eq69_e2133_d_b4 + (eq69_e2136_d_b4 * ddt_scale));let eq69_e2138_d_b5: f64 = (eq69_e2133_d_b5 + (eq69_e2136_d_b5 * ddt_scale));let eq69_e2138_d_b6: f64 = (eq69_e2133_d_b6 + (eq69_e2136_d_b6 * ddt_scale));let eq69_e2138_d_b7: f64 = (eq69_e2133_d_b7 + (eq69_e2136_d_b7 * ddt_scale));let eq69_e2138_d_b8: f64 = (eq69_e2133_d_b8 + (eq69_e2136_d_b8 * ddt_scale));let eq69_e2138_d_b9: f64 = (eq69_e2133_d_b9 + (eq69_e2136_d_b9 * ddt_scale));let eq69_e2138_d_b10: f64 = (eq69_e2133_d_b10 + (eq69_e2136_d_b10 * ddt_scale));let eq69_e2138_d_b11: f64 = (eq69_e2133_d_b11 + (eq69_e2136_d_b11 * ddt_scale));let eq69_e2140: f64 = (eq69_e2138 - s.v[1017]);let eq69_e2140_d_n0: f64 = (eq69_e2138_d_n0 - s.dn[1017][0]);let eq69_e2140_d_n1: f64 = (eq69_e2138_d_n1 - s.dn[1017][1]);let eq69_e2140_d_n2: f64 = (eq69_e2138_d_n2 - s.dn[1017][2]);let eq69_e2140_d_n3: f64 = (eq69_e2138_d_n3 - s.dn[1017][3]);let eq69_e2140_d_n4: f64 = (eq69_e2138_d_n4 - s.dn[1017][4]);let eq69_e2140_d_n5: f64 = (eq69_e2138_d_n5 - s.dn[1017][5]);let eq69_e2140_d_n6: f64 = (eq69_e2138_d_n6 - s.dn[1017][6]);let eq69_e2140_d_n7: f64 = (eq69_e2138_d_n7 - s.dn[1017][7]);let eq69_e2140_d_n8: f64 = (eq69_e2138_d_n8 - s.dn[1017][8]);let eq69_e2140_d_n9: f64 = (eq69_e2138_d_n9 - s.dn[1017][9]);let eq69_e2140_d_n10: f64 = (eq69_e2138_d_n10 - s.dn[1017][10]);let eq69_e2140_d_n11: f64 = (eq69_e2138_d_n11 - s.dn[1017][11]);let eq69_e2140_d_n12: f64 = (eq69_e2138_d_n12 - s.dn[1017][12]);let eq69_e2140_d_n13: f64 = (eq69_e2138_d_n13 - s.dn[1017][13]);let eq69_e2140_d_b0: f64 = (eq69_e2138_d_b0 - s.db[1017][0]);let eq69_e2140_d_b1: f64 = (eq69_e2138_d_b1 - s.db[1017][1]);let eq69_e2140_d_b2: f64 = (eq69_e2138_d_b2 - s.db[1017][2]);let eq69_e2140_d_b3: f64 = (eq69_e2138_d_b3 - s.db[1017][3]);let eq69_e2140_d_b4: f64 = (eq69_e2138_d_b4 - s.db[1017][4]);
        let eq69_e2140_d_b5: f64 = (eq69_e2138_d_b5 - s.db[1017][5]);let eq69_e2140_d_b6: f64 = (eq69_e2138_d_b6 - s.db[1017][6]);let eq69_e2140_d_b7: f64 = (eq69_e2138_d_b7 - s.db[1017][7]);let eq69_e2140_d_b8: f64 = (eq69_e2138_d_b8 - s.db[1017][8]);let eq69_e2140_d_b9: f64 = (eq69_e2138_d_b9 - s.db[1017][9]);let eq69_e2140_d_b10: f64 = (eq69_e2138_d_b10 - s.db[1017][10]);let eq69_e2140_d_b11: f64 = (eq69_e2138_d_b11 - s.db[1017][11]);
        (eq69_e2140, eq69_e2140_d_n0, eq69_e2140_d_n1, eq69_e2140_d_n2, eq69_e2140_d_n3, eq69_e2140_d_n4, eq69_e2140_d_n5, eq69_e2140_d_n6, eq69_e2140_d_n7, eq69_e2140_d_n8, eq69_e2140_d_n9, eq69_e2140_d_n10, eq69_e2140_d_n11, eq69_e2140_d_n12, eq69_e2140_d_n13, eq69_e2140_d_b0, eq69_e2140_d_b1, eq69_e2140_d_b2, eq69_e2140_d_b3, eq69_e2140_d_b4, eq69_e2140_d_b5, eq69_e2140_d_b6, eq69_e2140_d_b7, eq69_e2140_d_b8, eq69_e2140_d_b9, eq69_e2140_d_b10, eq69_e2140_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2142;let eq69_node_derivatives: [f64; 14] = [eq69_e2142_d_n0, eq69_e2142_d_n1, eq69_e2142_d_n2, eq69_e2142_d_n3, eq69_e2142_d_n4, eq69_e2142_d_n5, eq69_e2142_d_n6, eq69_e2142_d_n7, eq69_e2142_d_n8, eq69_e2142_d_n9, eq69_e2142_d_n10, eq69_e2142_d_n11, eq69_e2142_d_n12, eq69_e2142_d_n13];let eq69_branch_derivatives: [f64; 12] = [eq69_e2142_d_b0, eq69_e2142_d_b1, eq69_e2142_d_b2, eq69_e2142_d_b3, eq69_e2142_d_b4, eq69_e2142_d_b5, eq69_e2142_d_b6, eq69_e2142_d_b7, eq69_e2142_d_b8, eq69_e2142_d_b9, eq69_e2142_d_b10, eq69_e2142_d_b11];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivatives,
            &eq69_branch_derivatives,
            multiplicity,
        );
    }
}
