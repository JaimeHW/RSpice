#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let __rspice_deriv_cse_1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let __rspice_deriv_cse_2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let __rspice_deriv_cse_3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let __rspice_deriv_cse_4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let __rspice_deriv_cse_5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let __rspice_deriv_cse_6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let __rspice_deriv_cse_7: f64 = (s.db[24][0] * (nv0 - nv1));
        let __rspice_deriv_cse_8: f64 = (s.db[24][1] * (nv0 - nv1));
        let __rspice_deriv_cse_9: f64 = (s.db[24][2] * (nv0 - nv1));
        let __rspice_deriv_cse_10: f64 = (s.db[24][3] * (nv0 - nv1));
        let __rspice_deriv_cse_11: f64 = (s.db[24][4] * (nv0 - nv1));
        let __rspice_deriv_cse_12: f64 = (s.db[24][5] * (nv0 - nv1));
        let __rspice_deriv_cse_13: f64 = (s.db[24][6] * (nv0 - nv1));
        let (eq0_e60, eq0_e60_d_n0, eq0_e60_d_n1, eq0_e60_d_n2, eq0_e60_d_n3, eq0_e60_d_n4, eq0_e60_d_n5, eq0_e60_d_n6, eq0_e60_d_b0, eq0_e60_d_b1, eq0_e60_d_b2, eq0_e60_d_b3, eq0_e60_d_b4, eq0_e60_d_b5, eq0_e60_d_b6,) = {
    if s.b[68] {
        let eq0_e56: f64 = (-s.v[23]);
        let eq0_e58: f64 = (eq0_e56 * s.v[31]);
        let eq0_e58_d_n0: f64 = (((-s.dn[23][0]) * s.v[31]) + (eq0_e56 * s.dn[31][0]));
        let eq0_e58_d_n1: f64 = (((-s.dn[23][1]) * s.v[31]) + (eq0_e56 * s.dn[31][1]));
        let eq0_e58_d_n2: f64 = (((-s.dn[23][2]) * s.v[31]) + (eq0_e56 * s.dn[31][2]));
        let eq0_e58_d_n3: f64 = (((-s.dn[23][3]) * s.v[31]) + (eq0_e56 * s.dn[31][3]));
        let eq0_e58_d_n4: f64 = (((-s.dn[23][4]) * s.v[31]) + (eq0_e56 * s.dn[31][4]));
        let eq0_e58_d_n5: f64 = (((-s.dn[23][5]) * s.v[31]) + (eq0_e56 * s.dn[31][5]));
        let eq0_e58_d_n6: f64 = (((-s.dn[23][6]) * s.v[31]) + (eq0_e56 * s.dn[31][6]));
        let eq0_e58_d_b0: f64 = (((-s.db[23][0]) * s.v[31]) + (eq0_e56 * s.db[31][0]));
        let eq0_e58_d_b1: f64 = (((-s.db[23][1]) * s.v[31]) + (eq0_e56 * s.db[31][1]));
        let eq0_e58_d_b2: f64 = (((-s.db[23][2]) * s.v[31]) + (eq0_e56 * s.db[31][2]));
        let eq0_e58_d_b3: f64 = (((-s.db[23][3]) * s.v[31]) + (eq0_e56 * s.db[31][3]));
        let eq0_e58_d_b4: f64 = (((-s.db[23][4]) * s.v[31]) + (eq0_e56 * s.db[31][4]));
        let eq0_e58_d_b5: f64 = (((-s.db[23][5]) * s.v[31]) + (eq0_e56 * s.db[31][5]));
        let eq0_e58_d_b6: f64 = (((-s.db[23][6]) * s.v[31]) + (eq0_e56 * s.db[31][6]));
        (eq0_e58, eq0_e58_d_n0, eq0_e58_d_n1, eq0_e58_d_n2, eq0_e58_d_n3, eq0_e58_d_n4, eq0_e58_d_n5, eq0_e58_d_n6, eq0_e58_d_b0, eq0_e58_d_b1, eq0_e58_d_b2, eq0_e58_d_b3, eq0_e58_d_b4, eq0_e58_d_b5, eq0_e58_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e60;
        let eq0_node_derivatives: [f64; 7] = [eq0_e60_d_n0, eq0_e60_d_n1, eq0_e60_d_n2, eq0_e60_d_n3, eq0_e60_d_n4, eq0_e60_d_n5, eq0_e60_d_n6];
        let eq0_branch_derivatives: [f64; 7] = [eq0_e60_d_b0, eq0_e60_d_b1, eq0_e60_d_b2, eq0_e60_d_b3, eq0_e60_d_b4, eq0_e60_d_b5, eq0_e60_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6, eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6,) = {
    if s.b[68] {
        let eq2_e70: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, (nv6 - 0.0));
        let eq2_e71: f64 = (s.v[31] * eq2_e70);
        let eq2_e71_d_n0: f64 = (s.dn[31][0] * eq2_e70);
        let eq2_e71_d_n1: f64 = (s.dn[31][1] * eq2_e70);
        let eq2_e71_d_n2: f64 = (s.dn[31][2] * eq2_e70);
        let eq2_e71_d_n3: f64 = (s.dn[31][3] * eq2_e70);
        let eq2_e71_d_n4: f64 = (s.dn[31][4] * eq2_e70);
        let eq2_e71_d_n5: f64 = (s.dn[31][5] * eq2_e70);
        let eq2_e71_d_n6: f64 = ((s.dn[31][6] * eq2_e70) + (s.v[31] * ddt_scale));
        let eq2_e71_d_b0: f64 = (s.db[31][0] * eq2_e70);
        let eq2_e71_d_b1: f64 = (s.db[31][1] * eq2_e70);
        let eq2_e71_d_b2: f64 = (s.db[31][2] * eq2_e70);
        let eq2_e71_d_b3: f64 = (s.db[31][3] * eq2_e70);
        let eq2_e71_d_b4: f64 = (s.db[31][4] * eq2_e70);
        let eq2_e71_d_b5: f64 = (s.db[31][5] * eq2_e70);
        let eq2_e71_d_b6: f64 = (s.db[31][6] * eq2_e70);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, eq2_e71_d_n2, eq2_e71_d_n3, eq2_e71_d_n4, eq2_e71_d_n5, eq2_e71_d_n6, eq2_e71_d_b0, eq2_e71_d_b1, eq2_e71_d_b2, eq2_e71_d_b3, eq2_e71_d_b4, eq2_e71_d_b5, eq2_e71_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e73;
        let eq2_node_derivatives: [f64; 7] = [eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6];
        let eq2_branch_derivatives: [f64; 7] = [eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq4_e88, eq4_e88_d_n0, eq4_e88_d_n1, eq4_e88_d_n2, eq4_e88_d_n3, eq4_e88_d_n4, eq4_e88_d_n5, eq4_e88_d_n6, eq4_e88_d_b0, eq4_e88_d_b1, eq4_e88_d_b2, eq4_e88_d_b3, eq4_e88_d_b4, eq4_e88_d_b5, eq4_e88_d_b6,) = {
    if s.b[70] {
        let eq4_e81: f64 = (-1.0);
        let eq4_e84: f64 = (s.v[24] * (nv0 - nv1));
        let eq4_e85: f64 = (eq4_e84).abs();
        let eq4_e85_d_n0: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_0 } else { (-__rspice_deriv_cse_0) };
        let eq4_e85_d_n1: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_1 } else { (-__rspice_deriv_cse_1) };
        let eq4_e85_d_n2: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_2 } else { (-__rspice_deriv_cse_2) };
        let eq4_e85_d_n3: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_3 } else { (-__rspice_deriv_cse_3) };
        let eq4_e85_d_n4: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_4 } else { (-__rspice_deriv_cse_4) };
        let eq4_e85_d_n5: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_5 } else { (-__rspice_deriv_cse_5) };
        let eq4_e85_d_n6: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_6 } else { (-__rspice_deriv_cse_6) };
        let eq4_e85_d_b0: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_7 } else { (-__rspice_deriv_cse_7) };
        let eq4_e85_d_b1: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_8 } else { (-__rspice_deriv_cse_8) };
        let eq4_e85_d_b2: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_9 } else { (-__rspice_deriv_cse_9) };
        let eq4_e85_d_b3: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_10 } else { (-__rspice_deriv_cse_10) };
        let eq4_e85_d_b4: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_11 } else { (-__rspice_deriv_cse_11) };
        let eq4_e85_d_b5: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_12 } else { (-__rspice_deriv_cse_12) };
        let eq4_e85_d_b6: f64 = if eq4_e84 >= 0.0 { __rspice_deriv_cse_13 } else { (-__rspice_deriv_cse_13) };
        let eq4_e86: f64 = (eq4_e81 * eq4_e85);
        let eq4_e86_d_n0: f64 = (eq4_e81 * eq4_e85_d_n0);
        let eq4_e86_d_n1: f64 = (eq4_e81 * eq4_e85_d_n1);
        let eq4_e86_d_n2: f64 = (eq4_e81 * eq4_e85_d_n2);
        let eq4_e86_d_n3: f64 = (eq4_e81 * eq4_e85_d_n3);
        let eq4_e86_d_n4: f64 = (eq4_e81 * eq4_e85_d_n4);
        let eq4_e86_d_n5: f64 = (eq4_e81 * eq4_e85_d_n5);
        let eq4_e86_d_n6: f64 = (eq4_e81 * eq4_e85_d_n6);
        let eq4_e86_d_b0: f64 = (eq4_e81 * eq4_e85_d_b0);
        let eq4_e86_d_b1: f64 = (eq4_e81 * eq4_e85_d_b1);
        let eq4_e86_d_b2: f64 = (eq4_e81 * eq4_e85_d_b2);
        let eq4_e86_d_b3: f64 = (eq4_e81 * eq4_e85_d_b3);
        let eq4_e86_d_b4: f64 = (eq4_e81 * eq4_e85_d_b4);
        let eq4_e86_d_b5: f64 = (eq4_e81 * eq4_e85_d_b5);
        let eq4_e86_d_b6: f64 = (eq4_e81 * eq4_e85_d_b6);
        (eq4_e86, eq4_e86_d_n0, eq4_e86_d_n1, eq4_e86_d_n2, eq4_e86_d_n3, eq4_e86_d_n4, eq4_e86_d_n5, eq4_e86_d_n6, eq4_e86_d_b0, eq4_e86_d_b1, eq4_e86_d_b2, eq4_e86_d_b3, eq4_e86_d_b4, eq4_e86_d_b5, eq4_e86_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e88;
        let eq4_node_derivatives: [f64; 7] = [eq4_e88_d_n0, eq4_e88_d_n1, eq4_e88_d_n2, eq4_e88_d_n3, eq4_e88_d_n4, eq4_e88_d_n5, eq4_e88_d_n6];
        let eq4_branch_derivatives: [f64; 7] = [eq4_e88_d_b0, eq4_e88_d_b1, eq4_e88_d_b2, eq4_e88_d_b3, eq4_e88_d_b4, eq4_e88_d_b5, eq4_e88_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e94, eq5_e94_d_n2,) = {
    if s.b[70] {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p33;
        let eq5_e92: f64 = ((nv2 - 0.0) * __rspice_inv_cse_0);
        let eq5_e92_d_n2: f64 = (1.0 * __rspice_inv_cse_0);
        (eq5_e92, eq5_e92_d_n2,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e94;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq5_value),
            2,
            multiplicity * (eq5_e94_d_n2),
        );
        let (eq6_e101, eq6_e101_d_n2,) = {
    if s.b[70] {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e99: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq6_e98);
        (eq6_e99, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e101;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq6_value),
            2,
            multiplicity * (eq6_e101_d_n2),
        );
        let (eq8_e118, eq8_e118_d_n0, eq8_e118_d_n1, eq8_e118_d_n2, eq8_e118_d_n3, eq8_e118_d_n4, eq8_e118_d_n5, eq8_e118_d_n6, eq8_e118_d_b0, eq8_e118_d_b1, eq8_e118_d_b2, eq8_e118_d_b3, eq8_e118_d_b4, eq8_e118_d_b5, eq8_e118_d_b6,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq8_e111: f64 = (-1.0);
        let eq8_e114: f64 = (s.v[24] * (nv0 - nv1));
        let eq8_e115: f64 = (eq8_e114).abs();
        let eq8_e115_d_n0: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_0 } else { (-__rspice_deriv_cse_0) };
        let eq8_e115_d_n1: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_1 } else { (-__rspice_deriv_cse_1) };
        let eq8_e115_d_n2: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_2 } else { (-__rspice_deriv_cse_2) };
        let eq8_e115_d_n3: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_3 } else { (-__rspice_deriv_cse_3) };
        let eq8_e115_d_n4: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_4 } else { (-__rspice_deriv_cse_4) };
        let eq8_e115_d_n5: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_5 } else { (-__rspice_deriv_cse_5) };
        let eq8_e115_d_n6: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_6 } else { (-__rspice_deriv_cse_6) };
        let eq8_e115_d_b0: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_7 } else { (-__rspice_deriv_cse_7) };
        let eq8_e115_d_b1: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_8 } else { (-__rspice_deriv_cse_8) };
        let eq8_e115_d_b2: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_9 } else { (-__rspice_deriv_cse_9) };
        let eq8_e115_d_b3: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_10 } else { (-__rspice_deriv_cse_10) };
        let eq8_e115_d_b4: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_11 } else { (-__rspice_deriv_cse_11) };
        let eq8_e115_d_b5: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_12 } else { (-__rspice_deriv_cse_12) };
        let eq8_e115_d_b6: f64 = if eq8_e114 >= 0.0 { __rspice_deriv_cse_13 } else { (-__rspice_deriv_cse_13) };
        let eq8_e116: f64 = (eq8_e111 * eq8_e115);
        let eq8_e116_d_n0: f64 = (eq8_e111 * eq8_e115_d_n0);
        let eq8_e116_d_n1: f64 = (eq8_e111 * eq8_e115_d_n1);
        let eq8_e116_d_n2: f64 = (eq8_e111 * eq8_e115_d_n2);
        let eq8_e116_d_n3: f64 = (eq8_e111 * eq8_e115_d_n3);
        let eq8_e116_d_n4: f64 = (eq8_e111 * eq8_e115_d_n4);
        let eq8_e116_d_n5: f64 = (eq8_e111 * eq8_e115_d_n5);
        let eq8_e116_d_n6: f64 = (eq8_e111 * eq8_e115_d_n6);
        let eq8_e116_d_b0: f64 = (eq8_e111 * eq8_e115_d_b0);
        let eq8_e116_d_b1: f64 = (eq8_e111 * eq8_e115_d_b1);
        let eq8_e116_d_b2: f64 = (eq8_e111 * eq8_e115_d_b2);
        let eq8_e116_d_b3: f64 = (eq8_e111 * eq8_e115_d_b3);
        let eq8_e116_d_b4: f64 = (eq8_e111 * eq8_e115_d_b4);
        let eq8_e116_d_b5: f64 = (eq8_e111 * eq8_e115_d_b5);
        let eq8_e116_d_b6: f64 = (eq8_e111 * eq8_e115_d_b6);
        (eq8_e116, eq8_e116_d_n0, eq8_e116_d_n1, eq8_e116_d_n2, eq8_e116_d_n3, eq8_e116_d_n4, eq8_e116_d_n5, eq8_e116_d_n6, eq8_e116_d_b0, eq8_e116_d_b1, eq8_e116_d_b2, eq8_e116_d_b3, eq8_e116_d_b4, eq8_e116_d_b5, eq8_e116_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e118;
        let eq8_node_derivatives: [f64; 7] = [eq8_e118_d_n0, eq8_e118_d_n1, eq8_e118_d_n2, eq8_e118_d_n3, eq8_e118_d_n4, eq8_e118_d_n5, eq8_e118_d_n6];
        let eq8_branch_derivatives: [f64; 7] = [eq8_e118_d_b0, eq8_e118_d_b1, eq8_e118_d_b2, eq8_e118_d_b3, eq8_e118_d_b4, eq8_e118_d_b5, eq8_e118_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e127, eq9_e127_d_n2, eq9_e127_d_n5,) = {
    if ((!s.b[70]) && s.b[71]) {
        let __rspice_inv_cse_1: f64 = 1.0 / p.p33;
        let eq9_e125: f64 = ((nv2 - nv5) * __rspice_inv_cse_1);
        let eq9_e125_d_n2: f64 = (1.0 * __rspice_inv_cse_1);
        let eq9_e125_d_n5: f64 = ((-1.0) * __rspice_inv_cse_1);
        (eq9_e125, eq9_e125_d_n2, eq9_e125_d_n5,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e127;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(5),
            multiplicity * (eq9_value),
            2,
            multiplicity * (eq9_e127_d_n2),
            5,
            multiplicity * (eq9_e127_d_n5),
        );
        let (eq10_e137, eq10_e137_d_n2,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e135: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq10_e134);
        (eq10_e135, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e137;
        stamper.stamp_current_node1_local(
            Some(2),
            None,
            multiplicity * (eq10_value),
            2,
            multiplicity * (eq10_e137_d_n2),
        );
        let (eq11_e146, eq11_e146_d_n5,) = {
    if ((!s.b[70]) && s.b[71]) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p35;
        let eq11_e144: f64 = ((nv5 - 0.0) * __rspice_inv_cse_2);
        let eq11_e144_d_n5: f64 = (1.0 * __rspice_inv_cse_2);
        (eq11_e144, eq11_e144_d_n5,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e146;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq11_value),
            5,
            multiplicity * (eq11_e146_d_n5),
        );
        let (eq12_e156, eq12_e156_d_n5,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq12_e153);
        (eq12_e154, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e156;
        stamper.stamp_current_node1_local(
            Some(5),
            None,
            multiplicity * (eq12_value),
            5,
            multiplicity * (eq12_e156_d_n5),
        );
        let (eq13_e172, eq13_e172_d_n0, eq13_e172_d_n1, eq13_e172_d_n2, eq13_e172_d_n3, eq13_e172_d_n4, eq13_e172_d_n5, eq13_e172_d_n6, eq13_e172_d_b0, eq13_e172_d_b1, eq13_e172_d_b2, eq13_e172_d_b3, eq13_e172_d_b4, eq13_e172_d_b5, eq13_e172_d_b6,) = {
    if (((!s.b[70]) && (!s.b[71])) && s.b[72]) {
        let eq13_e165: f64 = (-1.0);
        let eq13_e168: f64 = (s.v[24] * (nv0 - nv1));
        let eq13_e169: f64 = (eq13_e168).abs();
        let eq13_e169_d_n0: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_0 } else { (-__rspice_deriv_cse_0) };
        let eq13_e169_d_n1: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_1 } else { (-__rspice_deriv_cse_1) };
        let eq13_e169_d_n2: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_2 } else { (-__rspice_deriv_cse_2) };
        let eq13_e169_d_n3: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_3 } else { (-__rspice_deriv_cse_3) };
        let eq13_e169_d_n4: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_4 } else { (-__rspice_deriv_cse_4) };
        let eq13_e169_d_n5: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_5 } else { (-__rspice_deriv_cse_5) };
        let eq13_e169_d_n6: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_6 } else { (-__rspice_deriv_cse_6) };
        let eq13_e169_d_b0: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_7 } else { (-__rspice_deriv_cse_7) };
        let eq13_e169_d_b1: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_8 } else { (-__rspice_deriv_cse_8) };
        let eq13_e169_d_b2: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_9 } else { (-__rspice_deriv_cse_9) };
        let eq13_e169_d_b3: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_10 } else { (-__rspice_deriv_cse_10) };
        let eq13_e169_d_b4: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_11 } else { (-__rspice_deriv_cse_11) };
        let eq13_e169_d_b5: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_12 } else { (-__rspice_deriv_cse_12) };
        let eq13_e169_d_b6: f64 = if eq13_e168 >= 0.0 { __rspice_deriv_cse_13 } else { (-__rspice_deriv_cse_13) };
        let eq13_e170: f64 = (eq13_e165 * eq13_e169);
        let eq13_e170_d_n0: f64 = (eq13_e165 * eq13_e169_d_n0);
        let eq13_e170_d_n1: f64 = (eq13_e165 * eq13_e169_d_n1);
        let eq13_e170_d_n2: f64 = (eq13_e165 * eq13_e169_d_n2);
        let eq13_e170_d_n3: f64 = (eq13_e165 * eq13_e169_d_n3);
        let eq13_e170_d_n4: f64 = (eq13_e165 * eq13_e169_d_n4);
        let eq13_e170_d_n5: f64 = (eq13_e165 * eq13_e169_d_n5);
        let eq13_e170_d_n6: f64 = (eq13_e165 * eq13_e169_d_n6);
        let eq13_e170_d_b0: f64 = (eq13_e165 * eq13_e169_d_b0);
        let eq13_e170_d_b1: f64 = (eq13_e165 * eq13_e169_d_b1);
        let eq13_e170_d_b2: f64 = (eq13_e165 * eq13_e169_d_b2);
        let eq13_e170_d_b3: f64 = (eq13_e165 * eq13_e169_d_b3);
        let eq13_e170_d_b4: f64 = (eq13_e165 * eq13_e169_d_b4);
        let eq13_e170_d_b5: f64 = (eq13_e165 * eq13_e169_d_b5);
        let eq13_e170_d_b6: f64 = (eq13_e165 * eq13_e169_d_b6);
        (eq13_e170, eq13_e170_d_n0, eq13_e170_d_n1, eq13_e170_d_n2, eq13_e170_d_n3, eq13_e170_d_n4, eq13_e170_d_n5, eq13_e170_d_n6, eq13_e170_d_b0, eq13_e170_d_b1, eq13_e170_d_b2, eq13_e170_d_b3, eq13_e170_d_b4, eq13_e170_d_b5, eq13_e170_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e172;
        let eq13_node_derivatives: [f64; 7] = [eq13_e172_d_n0, eq13_e172_d_n1, eq13_e172_d_n2, eq13_e172_d_n3, eq13_e172_d_n4, eq13_e172_d_n5, eq13_e172_d_n6];
        let eq13_branch_derivatives: [f64; 7] = [eq13_e172_d_b0, eq13_e172_d_b1, eq13_e172_d_b2, eq13_e172_d_b3, eq13_e172_d_b4, eq13_e172_d_b5, eq13_e172_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            None,
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq18_e224, eq18_e224_d_n0, eq18_e224_d_n1, eq18_e224_d_n2, eq18_e224_d_n3, eq18_e224_d_n4, eq18_e224_d_n5, eq18_e224_d_n6, eq18_e224_d_b0, eq18_e224_d_b1, eq18_e224_d_b2, eq18_e224_d_b3, eq18_e224_d_b4, eq18_e224_d_b5, eq18_e224_d_b6,) = {
    if s.b[73] {
        let __rspice_inv_cse_3: f64 = 1.0 / s.v[3];
        let eq18_e214: f64 = (s.v[29] * __rspice_inv_cse_3);
        let eq18_e214_d_n0: f64 = (s.dn[29][0] * __rspice_inv_cse_3);
        let eq18_e214_d_n1: f64 = (s.dn[29][1] * __rspice_inv_cse_3);
        let eq18_e214_d_n2: f64 = (s.dn[29][2] * __rspice_inv_cse_3);
        let eq18_e214_d_n3: f64 = (s.dn[29][3] * __rspice_inv_cse_3);
        let eq18_e214_d_n4: f64 = (s.dn[29][4] * __rspice_inv_cse_3);
        let eq18_e214_d_n5: f64 = (s.dn[29][5] * __rspice_inv_cse_3);
        let eq18_e214_d_n6: f64 = (s.dn[29][6] * __rspice_inv_cse_3);
        let eq18_e214_d_b0: f64 = (s.db[29][0] * __rspice_inv_cse_3);
        let eq18_e214_d_b1: f64 = (s.db[29][1] * __rspice_inv_cse_3);
        let eq18_e214_d_b2: f64 = (s.db[29][2] * __rspice_inv_cse_3);
        let eq18_e214_d_b3: f64 = (s.db[29][3] * __rspice_inv_cse_3);
        let eq18_e214_d_b4: f64 = (s.db[29][4] * __rspice_inv_cse_3);
        let eq18_e214_d_b5: f64 = (s.db[29][5] * __rspice_inv_cse_3);
        let eq18_e214_d_b6: f64 = (s.db[29][6] * __rspice_inv_cse_3);
        let (eq18_e221, eq18_e221_d_n0, eq18_e221_d_n1, eq18_e221_d_n2, eq18_e221_d_n3, eq18_e221_d_n4, eq18_e221_d_n5, eq18_e221_d_n6, eq18_e221_d_b0, eq18_e221_d_b1, eq18_e221_d_b2, eq18_e221_d_b3, eq18_e221_d_b4, eq18_e221_d_b5, eq18_e221_d_b6,) = {
            if (eq18_e214 > p.p46) {
                let __rspice_inv_cse_4: f64 = 1.0 / s.v[3];
                let eq18_e219: f64 = (s.v[29] * __rspice_inv_cse_4);
                let eq18_e219_d_n0: f64 = (s.dn[29][0] * __rspice_inv_cse_4);
                let eq18_e219_d_n1: f64 = (s.dn[29][1] * __rspice_inv_cse_4);
                let eq18_e219_d_n2: f64 = (s.dn[29][2] * __rspice_inv_cse_4);
                let eq18_e219_d_n3: f64 = (s.dn[29][3] * __rspice_inv_cse_4);
                let eq18_e219_d_n4: f64 = (s.dn[29][4] * __rspice_inv_cse_4);
                let eq18_e219_d_n5: f64 = (s.dn[29][5] * __rspice_inv_cse_4);
                let eq18_e219_d_n6: f64 = (s.dn[29][6] * __rspice_inv_cse_4);
                let eq18_e219_d_b0: f64 = (s.db[29][0] * __rspice_inv_cse_4);
                let eq18_e219_d_b1: f64 = (s.db[29][1] * __rspice_inv_cse_4);
                let eq18_e219_d_b2: f64 = (s.db[29][2] * __rspice_inv_cse_4);
                let eq18_e219_d_b3: f64 = (s.db[29][3] * __rspice_inv_cse_4);
                let eq18_e219_d_b4: f64 = (s.db[29][4] * __rspice_inv_cse_4);
                let eq18_e219_d_b5: f64 = (s.db[29][5] * __rspice_inv_cse_4);
                let eq18_e219_d_b6: f64 = (s.db[29][6] * __rspice_inv_cse_4);
                (eq18_e219, eq18_e219_d_n0, eq18_e219_d_n1, eq18_e219_d_n2, eq18_e219_d_n3, eq18_e219_d_n4, eq18_e219_d_n5, eq18_e219_d_n6, eq18_e219_d_b0, eq18_e219_d_b1, eq18_e219_d_b2, eq18_e219_d_b3, eq18_e219_d_b4, eq18_e219_d_b5, eq18_e219_d_b6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq18_e222: f64 = ((nv0 - nv3) / eq18_e221);
        let eq18_e222_d_n0: f64 = ((eq18_e221 - ((nv0 - nv3) * eq18_e221_d_n0)) / (eq18_e221 * eq18_e221));
        let eq18_e222_d_n1: f64 = (-(((nv0 - nv3) * eq18_e221_d_n1) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n2: f64 = (-(((nv0 - nv3) * eq18_e221_d_n2) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n3: f64 = (((-eq18_e221) - ((nv0 - nv3) * eq18_e221_d_n3)) / (eq18_e221 * eq18_e221));
        let eq18_e222_d_n4: f64 = (-(((nv0 - nv3) * eq18_e221_d_n4) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n5: f64 = (-(((nv0 - nv3) * eq18_e221_d_n5) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_n6: f64 = (-(((nv0 - nv3) * eq18_e221_d_n6) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b0: f64 = (-(((nv0 - nv3) * eq18_e221_d_b0) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b1: f64 = (-(((nv0 - nv3) * eq18_e221_d_b1) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b2: f64 = (-(((nv0 - nv3) * eq18_e221_d_b2) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b3: f64 = (-(((nv0 - nv3) * eq18_e221_d_b3) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b4: f64 = (-(((nv0 - nv3) * eq18_e221_d_b4) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b5: f64 = (-(((nv0 - nv3) * eq18_e221_d_b5) / (eq18_e221 * eq18_e221)));
        let eq18_e222_d_b6: f64 = (-(((nv0 - nv3) * eq18_e221_d_b6) / (eq18_e221 * eq18_e221)));
        (eq18_e222, eq18_e222_d_n0, eq18_e222_d_n1, eq18_e222_d_n2, eq18_e222_d_n3, eq18_e222_d_n4, eq18_e222_d_n5, eq18_e222_d_n6, eq18_e222_d_b0, eq18_e222_d_b1, eq18_e222_d_b2, eq18_e222_d_b3, eq18_e222_d_b4, eq18_e222_d_b5, eq18_e222_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e224;
        let eq18_node_derivatives: [f64; 7] = [eq18_e224_d_n0, eq18_e224_d_n1, eq18_e224_d_n2, eq18_e224_d_n3, eq18_e224_d_n4, eq18_e224_d_n5, eq18_e224_d_n6];
        let eq18_branch_derivatives: [f64; 7] = [eq18_e224_d_b0, eq18_e224_d_b1, eq18_e224_d_b2, eq18_e224_d_b3, eq18_e224_d_b4, eq18_e224_d_b5, eq18_e224_d_b6];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq21_e250, eq21_e250_d_n0, eq21_e250_d_n1, eq21_e250_d_n2, eq21_e250_d_n3, eq21_e250_d_n4, eq21_e250_d_n5, eq21_e250_d_n6, eq21_e250_d_b0, eq21_e250_d_b1, eq21_e250_d_b2, eq21_e250_d_b3, eq21_e250_d_b4, eq21_e250_d_b5, eq21_e250_d_b6,) = {
    if s.b[74] {
        let __rspice_inv_cse_5: f64 = 1.0 / s.v[3];
        let eq21_e240: f64 = (s.v[30] * __rspice_inv_cse_5);
        let eq21_e240_d_n0: f64 = (s.dn[30][0] * __rspice_inv_cse_5);
        let eq21_e240_d_n1: f64 = (s.dn[30][1] * __rspice_inv_cse_5);
        let eq21_e240_d_n2: f64 = (s.dn[30][2] * __rspice_inv_cse_5);
        let eq21_e240_d_n3: f64 = (s.dn[30][3] * __rspice_inv_cse_5);
        let eq21_e240_d_n4: f64 = (s.dn[30][4] * __rspice_inv_cse_5);
        let eq21_e240_d_n5: f64 = (s.dn[30][5] * __rspice_inv_cse_5);
        let eq21_e240_d_n6: f64 = (s.dn[30][6] * __rspice_inv_cse_5);
        let eq21_e240_d_b0: f64 = (s.db[30][0] * __rspice_inv_cse_5);
        let eq21_e240_d_b1: f64 = (s.db[30][1] * __rspice_inv_cse_5);
        let eq21_e240_d_b2: f64 = (s.db[30][2] * __rspice_inv_cse_5);
        let eq21_e240_d_b3: f64 = (s.db[30][3] * __rspice_inv_cse_5);
        let eq21_e240_d_b4: f64 = (s.db[30][4] * __rspice_inv_cse_5);
        let eq21_e240_d_b5: f64 = (s.db[30][5] * __rspice_inv_cse_5);
        let eq21_e240_d_b6: f64 = (s.db[30][6] * __rspice_inv_cse_5);
        let (eq21_e247, eq21_e247_d_n0, eq21_e247_d_n1, eq21_e247_d_n2, eq21_e247_d_n3, eq21_e247_d_n4, eq21_e247_d_n5, eq21_e247_d_n6, eq21_e247_d_b0, eq21_e247_d_b1, eq21_e247_d_b2, eq21_e247_d_b3, eq21_e247_d_b4, eq21_e247_d_b5, eq21_e247_d_b6,) = {
            if (eq21_e240 > p.p46) {
                let __rspice_inv_cse_6: f64 = 1.0 / s.v[3];
                let eq21_e245: f64 = (s.v[30] * __rspice_inv_cse_6);
                let eq21_e245_d_n0: f64 = (s.dn[30][0] * __rspice_inv_cse_6);
                let eq21_e245_d_n1: f64 = (s.dn[30][1] * __rspice_inv_cse_6);
                let eq21_e245_d_n2: f64 = (s.dn[30][2] * __rspice_inv_cse_6);
                let eq21_e245_d_n3: f64 = (s.dn[30][3] * __rspice_inv_cse_6);
                let eq21_e245_d_n4: f64 = (s.dn[30][4] * __rspice_inv_cse_6);
                let eq21_e245_d_n5: f64 = (s.dn[30][5] * __rspice_inv_cse_6);
                let eq21_e245_d_n6: f64 = (s.dn[30][6] * __rspice_inv_cse_6);
                let eq21_e245_d_b0: f64 = (s.db[30][0] * __rspice_inv_cse_6);
                let eq21_e245_d_b1: f64 = (s.db[30][1] * __rspice_inv_cse_6);
                let eq21_e245_d_b2: f64 = (s.db[30][2] * __rspice_inv_cse_6);
                let eq21_e245_d_b3: f64 = (s.db[30][3] * __rspice_inv_cse_6);
                let eq21_e245_d_b4: f64 = (s.db[30][4] * __rspice_inv_cse_6);
                let eq21_e245_d_b5: f64 = (s.db[30][5] * __rspice_inv_cse_6);
                let eq21_e245_d_b6: f64 = (s.db[30][6] * __rspice_inv_cse_6);
                (eq21_e245, eq21_e245_d_n0, eq21_e245_d_n1, eq21_e245_d_n2, eq21_e245_d_n3, eq21_e245_d_n4, eq21_e245_d_n5, eq21_e245_d_n6, eq21_e245_d_b0, eq21_e245_d_b1, eq21_e245_d_b2, eq21_e245_d_b3, eq21_e245_d_b4, eq21_e245_d_b5, eq21_e245_d_b6,)
            } else {
                (p.p46, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let eq21_e248: f64 = ((nv1 - nv4) / eq21_e247);
        let eq21_e248_d_n0: f64 = (-(((nv1 - nv4) * eq21_e247_d_n0) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n1: f64 = ((eq21_e247 - ((nv1 - nv4) * eq21_e247_d_n1)) / (eq21_e247 * eq21_e247));
        let eq21_e248_d_n2: f64 = (-(((nv1 - nv4) * eq21_e247_d_n2) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n3: f64 = (-(((nv1 - nv4) * eq21_e247_d_n3) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n4: f64 = (((-eq21_e247) - ((nv1 - nv4) * eq21_e247_d_n4)) / (eq21_e247 * eq21_e247));
        let eq21_e248_d_n5: f64 = (-(((nv1 - nv4) * eq21_e247_d_n5) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_n6: f64 = (-(((nv1 - nv4) * eq21_e247_d_n6) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b0: f64 = (-(((nv1 - nv4) * eq21_e247_d_b0) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b1: f64 = (-(((nv1 - nv4) * eq21_e247_d_b1) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b2: f64 = (-(((nv1 - nv4) * eq21_e247_d_b2) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b3: f64 = (-(((nv1 - nv4) * eq21_e247_d_b3) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b4: f64 = (-(((nv1 - nv4) * eq21_e247_d_b4) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b5: f64 = (-(((nv1 - nv4) * eq21_e247_d_b5) / (eq21_e247 * eq21_e247)));
        let eq21_e248_d_b6: f64 = (-(((nv1 - nv4) * eq21_e247_d_b6) / (eq21_e247 * eq21_e247)));
        (eq21_e248, eq21_e248_d_n0, eq21_e248_d_n1, eq21_e248_d_n2, eq21_e248_d_n3, eq21_e248_d_n4, eq21_e248_d_n5, eq21_e248_d_n6, eq21_e248_d_b0, eq21_e248_d_b1, eq21_e248_d_b2, eq21_e248_d_b3, eq21_e248_d_b4, eq21_e248_d_b5, eq21_e248_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e250;
        let eq21_node_derivatives: [f64; 7] = [eq21_e250_d_n0, eq21_e250_d_n1, eq21_e250_d_n2, eq21_e250_d_n3, eq21_e250_d_n4, eq21_e250_d_n5, eq21_e250_d_n6];
        let eq21_branch_derivatives: [f64; 7] = [eq21_e250_d_b0, eq21_e250_d_b1, eq21_e250_d_b2, eq21_e250_d_b3, eq21_e250_d_b4, eq21_e250_d_b5, eq21_e250_d_b6];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq24_e264: f64 = (s.v[9] * s.v[24]);
        let eq24_e264_d_n0: f64 = (s.v[9] * s.dn[24][0]);
        let eq24_e264_d_n1: f64 = (s.v[9] * s.dn[24][1]);
        let eq24_e264_d_n2: f64 = (s.v[9] * s.dn[24][2]);
        let eq24_e264_d_n3: f64 = (s.v[9] * s.dn[24][3]);
        let eq24_e264_d_n4: f64 = (s.v[9] * s.dn[24][4]);
        let eq24_e264_d_n5: f64 = (s.v[9] * s.dn[24][5]);
        let eq24_e264_d_n6: f64 = (s.v[9] * s.dn[24][6]);
        let eq24_e264_d_b0: f64 = (s.v[9] * s.db[24][0]);
        let eq24_e264_d_b1: f64 = (s.v[9] * s.db[24][1]);
        let eq24_e264_d_b2: f64 = (s.v[9] * s.db[24][2]);
        let eq24_e264_d_b3: f64 = (s.v[9] * s.db[24][3]);
        let eq24_e264_d_b4: f64 = (s.v[9] * s.db[24][4]);
        let eq24_e264_d_b5: f64 = (s.v[9] * s.db[24][5]);
        let eq24_e264_d_b6: f64 = (s.v[9] * s.db[24][6]);
        let eq24_e266: f64 = (eq24_e264 * s.v[3]);
        let eq24_e266_d_n0: f64 = (eq24_e264_d_n0 * s.v[3]);
        let eq24_e266_d_n1: f64 = (eq24_e264_d_n1 * s.v[3]);
        let eq24_e266_d_n2: f64 = (eq24_e264_d_n2 * s.v[3]);
        let eq24_e266_d_n3: f64 = (eq24_e264_d_n3 * s.v[3]);
        let eq24_e266_d_n4: f64 = (eq24_e264_d_n4 * s.v[3]);
        let eq24_e266_d_n5: f64 = (eq24_e264_d_n5 * s.v[3]);
        let eq24_e266_d_n6: f64 = (eq24_e264_d_n6 * s.v[3]);
        let eq24_e266_d_b0: f64 = (eq24_e264_d_b0 * s.v[3]);
        let eq24_e266_d_b1: f64 = (eq24_e264_d_b1 * s.v[3]);
        let eq24_e266_d_b2: f64 = (eq24_e264_d_b2 * s.v[3]);
        let eq24_e266_d_b3: f64 = (eq24_e264_d_b3 * s.v[3]);
        let eq24_e266_d_b4: f64 = (eq24_e264_d_b4 * s.v[3]);
        let eq24_e266_d_b5: f64 = (eq24_e264_d_b5 * s.v[3]);
        let eq24_e266_d_b6: f64 = (eq24_e264_d_b6 * s.v[3]);
        let eq24_value: f64 = eq24_e266;
        let eq24_node_derivatives: [f64; 7] = [eq24_e266_d_n0, eq24_e266_d_n1, eq24_e266_d_n2, eq24_e266_d_n3, eq24_e266_d_n4, eq24_e266_d_n5, eq24_e266_d_n6];
        let eq24_branch_derivatives: [f64; 7] = [eq24_e266_d_b0, eq24_e266_d_b1, eq24_e266_d_b2, eq24_e266_d_b3, eq24_e266_d_b4, eq24_e266_d_b5, eq24_e266_d_b6];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(4),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e269: f64 = (s.v[9] * s.v[33]);
        let eq25_e269_d_n0: f64 = (s.v[9] * s.dn[33][0]);
        let eq25_e269_d_n1: f64 = (s.v[9] * s.dn[33][1]);
        let eq25_e269_d_n2: f64 = (s.v[9] * s.dn[33][2]);
        let eq25_e269_d_n3: f64 = (s.v[9] * s.dn[33][3]);
        let eq25_e269_d_n4: f64 = (s.v[9] * s.dn[33][4]);
        let eq25_e269_d_n5: f64 = (s.v[9] * s.dn[33][5]);
        let eq25_e269_d_n6: f64 = (s.v[9] * s.dn[33][6]);
        let eq25_e269_d_b0: f64 = (s.v[9] * s.db[33][0]);
        let eq25_e269_d_b1: f64 = (s.v[9] * s.db[33][1]);
        let eq25_e269_d_b2: f64 = (s.v[9] * s.db[33][2]);
        let eq25_e269_d_b3: f64 = (s.v[9] * s.db[33][3]);
        let eq25_e269_d_b4: f64 = (s.v[9] * s.db[33][4]);
        let eq25_e269_d_b5: f64 = (s.v[9] * s.db[33][5]);
        let eq25_e269_d_b6: f64 = (s.v[9] * s.db[33][6]);
        let eq25_e271: f64 = (eq25_e269 * s.v[3]);
        let eq25_e271_d_n0: f64 = (eq25_e269_d_n0 * s.v[3]);
        let eq25_e271_d_n1: f64 = (eq25_e269_d_n1 * s.v[3]);
        let eq25_e271_d_n2: f64 = (eq25_e269_d_n2 * s.v[3]);
        let eq25_e271_d_n3: f64 = (eq25_e269_d_n3 * s.v[3]);
        let eq25_e271_d_n4: f64 = (eq25_e269_d_n4 * s.v[3]);
        let eq25_e271_d_n5: f64 = (eq25_e269_d_n5 * s.v[3]);
        let eq25_e271_d_n6: f64 = (eq25_e269_d_n6 * s.v[3]);
        let eq25_e271_d_b0: f64 = (eq25_e269_d_b0 * s.v[3]);
        let eq25_e271_d_b1: f64 = (eq25_e269_d_b1 * s.v[3]);
        let eq25_e271_d_b2: f64 = (eq25_e269_d_b2 * s.v[3]);
        let eq25_e271_d_b3: f64 = (eq25_e269_d_b3 * s.v[3]);
        let eq25_e271_d_b4: f64 = (eq25_e269_d_b4 * s.v[3]);
        let eq25_e271_d_b5: f64 = (eq25_e269_d_b5 * s.v[3]);
        let eq25_e271_d_b6: f64 = (eq25_e269_d_b6 * s.v[3]);
        let eq25_e272: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq25_e271);
        let eq25_value: f64 = eq25_e272;
        let eq25_node_derivatives: [f64; 7] = [(eq25_e271_d_n0 * ddt_scale), (eq25_e271_d_n1 * ddt_scale), (eq25_e271_d_n2 * ddt_scale), (eq25_e271_d_n3 * ddt_scale), (eq25_e271_d_n4 * ddt_scale), (eq25_e271_d_n5 * ddt_scale), (eq25_e271_d_n6 * ddt_scale)];
        let eq25_branch_derivatives: [f64; 7] = [(eq25_e271_d_b0 * ddt_scale), (eq25_e271_d_b1 * ddt_scale), (eq25_e271_d_b2 * ddt_scale), (eq25_e271_d_b3 * ddt_scale), (eq25_e271_d_b4 * ddt_scale), (eq25_e271_d_b5 * ddt_scale), (eq25_e271_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(4),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let eq26_e275: f64 = (s.v[9] * s.v[32]);
        let eq26_e275_d_n0: f64 = (s.v[9] * s.dn[32][0]);
        let eq26_e275_d_n1: f64 = (s.v[9] * s.dn[32][1]);
        let eq26_e275_d_n2: f64 = (s.v[9] * s.dn[32][2]);
        let eq26_e275_d_n3: f64 = (s.v[9] * s.dn[32][3]);
        let eq26_e275_d_n4: f64 = (s.v[9] * s.dn[32][4]);
        let eq26_e275_d_n5: f64 = (s.v[9] * s.dn[32][5]);
        let eq26_e275_d_n6: f64 = (s.v[9] * s.dn[32][6]);
        let eq26_e275_d_b0: f64 = (s.v[9] * s.db[32][0]);
        let eq26_e275_d_b1: f64 = (s.v[9] * s.db[32][1]);
        let eq26_e275_d_b2: f64 = (s.v[9] * s.db[32][2]);
        let eq26_e275_d_b3: f64 = (s.v[9] * s.db[32][3]);
        let eq26_e275_d_b4: f64 = (s.v[9] * s.db[32][4]);
        let eq26_e275_d_b5: f64 = (s.v[9] * s.db[32][5]);
        let eq26_e275_d_b6: f64 = (s.v[9] * s.db[32][6]);
        let eq26_e277: f64 = (eq26_e275 * s.v[3]);
        let eq26_e277_d_n0: f64 = (eq26_e275_d_n0 * s.v[3]);
        let eq26_e277_d_n1: f64 = (eq26_e275_d_n1 * s.v[3]);
        let eq26_e277_d_n2: f64 = (eq26_e275_d_n2 * s.v[3]);
        let eq26_e277_d_n3: f64 = (eq26_e275_d_n3 * s.v[3]);
        let eq26_e277_d_n4: f64 = (eq26_e275_d_n4 * s.v[3]);
        let eq26_e277_d_n5: f64 = (eq26_e275_d_n5 * s.v[3]);
        let eq26_e277_d_n6: f64 = (eq26_e275_d_n6 * s.v[3]);
        let eq26_e277_d_b0: f64 = (eq26_e275_d_b0 * s.v[3]);
        let eq26_e277_d_b1: f64 = (eq26_e275_d_b1 * s.v[3]);
        let eq26_e277_d_b2: f64 = (eq26_e275_d_b2 * s.v[3]);
        let eq26_e277_d_b3: f64 = (eq26_e275_d_b3 * s.v[3]);
        let eq26_e277_d_b4: f64 = (eq26_e275_d_b4 * s.v[3]);
        let eq26_e277_d_b5: f64 = (eq26_e275_d_b5 * s.v[3]);
        let eq26_e277_d_b6: f64 = (eq26_e275_d_b6 * s.v[3]);
        let eq26_e278: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq26_e277);
        let eq26_value: f64 = eq26_e278;
        let eq26_node_derivatives: [f64; 7] = [(eq26_e277_d_n0 * ddt_scale), (eq26_e277_d_n1 * ddt_scale), (eq26_e277_d_n2 * ddt_scale), (eq26_e277_d_n3 * ddt_scale), (eq26_e277_d_n4 * ddt_scale), (eq26_e277_d_n5 * ddt_scale), (eq26_e277_d_n6 * ddt_scale)];
        let eq26_branch_derivatives: [f64; 7] = [(eq26_e277_d_b0 * ddt_scale), (eq26_e277_d_b1 * ddt_scale), (eq26_e277_d_b2 * ddt_scale), (eq26_e277_d_b3 * ddt_scale), (eq26_e277_d_b4 * ddt_scale), (eq26_e277_d_b5 * ddt_scale), (eq26_e277_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(4),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n2, eq2_e73_d_n3, eq2_e73_d_n4, eq2_e73_d_n5, eq2_e73_d_n6, eq2_e73_d_b0, eq2_e73_d_b1, eq2_e73_d_b2, eq2_e73_d_b3, eq2_e73_d_b4, eq2_e73_d_b5, eq2_e73_d_b6, eq2_e73_q, eq2_e73_q_d_n0, eq2_e73_q_d_n1, eq2_e73_q_d_n2, eq2_e73_q_d_n3, eq2_e73_q_d_n4, eq2_e73_q_d_n5, eq2_e73_q_d_n6, eq2_e73_q_d_b0, eq2_e73_q_d_b1, eq2_e73_q_d_b2, eq2_e73_q_d_b3, eq2_e73_q_d_b4, eq2_e73_q_d_b5, eq2_e73_q_d_b6,) = {
    if s.b[68] {
        let eq2_e70_q: f64 = (nv6 - 0.0);
        let eq2_e71: f64 = (s.v[31] * (nv6 - 0.0));
        let eq2_e71_d_n0: f64 = (s.dn[31][0] * (nv6 - 0.0));
        let eq2_e71_d_n1: f64 = (s.dn[31][1] * (nv6 - 0.0));
        let eq2_e71_d_n2: f64 = (s.dn[31][2] * (nv6 - 0.0));
        let eq2_e71_d_n3: f64 = (s.dn[31][3] * (nv6 - 0.0));
        let eq2_e71_d_n4: f64 = (s.dn[31][4] * (nv6 - 0.0));
        let eq2_e71_d_n5: f64 = (s.dn[31][5] * (nv6 - 0.0));
        let eq2_e71_d_n6: f64 = ((s.dn[31][6] * (nv6 - 0.0)) + s.v[31]);
        let eq2_e71_d_b0: f64 = (s.db[31][0] * (nv6 - 0.0));
        let eq2_e71_d_b1: f64 = (s.db[31][1] * (nv6 - 0.0));
        let eq2_e71_d_b2: f64 = (s.db[31][2] * (nv6 - 0.0));
        let eq2_e71_d_b3: f64 = (s.db[31][3] * (nv6 - 0.0));
        let eq2_e71_d_b4: f64 = (s.db[31][4] * (nv6 - 0.0));
        let eq2_e71_d_b5: f64 = (s.db[31][5] * (nv6 - 0.0));
        let eq2_e71_d_b6: f64 = (s.db[31][6] * (nv6 - 0.0));
        let eq2_e71_q: f64 = (s.v[31] * eq2_e70_q);
        let eq2_e71_q_d_n0: f64 = (s.dn[31][0] * eq2_e70_q);
        let eq2_e71_q_d_n1: f64 = (s.dn[31][1] * eq2_e70_q);
        let eq2_e71_q_d_n2: f64 = (s.dn[31][2] * eq2_e70_q);
        let eq2_e71_q_d_n3: f64 = (s.dn[31][3] * eq2_e70_q);
        let eq2_e71_q_d_n4: f64 = (s.dn[31][4] * eq2_e70_q);
        let eq2_e71_q_d_n5: f64 = (s.dn[31][5] * eq2_e70_q);
        let eq2_e71_q_d_n6: f64 = ((s.dn[31][6] * eq2_e70_q) + s.v[31]);
        let eq2_e71_q_d_b0: f64 = (s.db[31][0] * eq2_e70_q);
        let eq2_e71_q_d_b1: f64 = (s.db[31][1] * eq2_e70_q);
        let eq2_e71_q_d_b2: f64 = (s.db[31][2] * eq2_e70_q);
        let eq2_e71_q_d_b3: f64 = (s.db[31][3] * eq2_e70_q);
        let eq2_e71_q_d_b4: f64 = (s.db[31][4] * eq2_e70_q);
        let eq2_e71_q_d_b5: f64 = (s.db[31][5] * eq2_e70_q);
        let eq2_e71_q_d_b6: f64 = (s.db[31][6] * eq2_e70_q);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, eq2_e71_d_n2, eq2_e71_d_n3, eq2_e71_d_n4, eq2_e71_d_n5, eq2_e71_d_n6, eq2_e71_d_b0, eq2_e71_d_b1, eq2_e71_d_b2, eq2_e71_d_b3, eq2_e71_d_b4, eq2_e71_d_b5, eq2_e71_d_b6, eq2_e71_q, eq2_e71_q_d_n0, eq2_e71_q_d_n1, eq2_e71_q_d_n2, eq2_e71_q_d_n3, eq2_e71_q_d_n4, eq2_e71_q_d_n5, eq2_e71_q_d_n6, eq2_e71_q_d_b0, eq2_e71_q_d_b1, eq2_e71_q_d_b2, eq2_e71_q_d_b3, eq2_e71_q_d_b4, eq2_e71_q_d_b5, eq2_e71_q_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_reactive_node_derivatives: [f64; 7] = [eq2_e73_q_d_n0, eq2_e73_q_d_n1, eq2_e73_q_d_n2, eq2_e73_q_d_n3, eq2_e73_q_d_n4, eq2_e73_q_d_n5, eq2_e73_q_d_n6];
        let eq2_reactive_branch_derivatives: [f64; 7] = [eq2_e73_q_d_b0, eq2_e73_q_d_b1, eq2_e73_q_d_b2, eq2_e73_q_d_b3, eq2_e73_q_d_b4, eq2_e73_q_d_b5, eq2_e73_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq2_reactive_node_derivatives,
            branches,
            &eq2_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq6_e101, eq6_e101_d_n2, eq6_e101_q,) = {
    if s.b[70] {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e99_q: f64 = eq6_e98;
        (eq6_e98, p.p34, eq6_e99_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq6_e101_d_n2),
        );
        let (eq10_e137, eq10_e137_d_n2, eq10_e137_q,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e135_q: f64 = eq10_e134;
        (eq10_e134, p.p34, eq10_e135_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq10_e137_d_n2),
        );
        let (eq12_e156, eq12_e156_d_n5, eq12_e156_q,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e154_q: f64 = eq12_e153;
        (eq12_e153, p.p36, eq12_e154_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq12_e156_d_n5),
        );
        let eq25_e269: f64 = (s.v[9] * s.v[33]);
        let eq25_e269_d_n0: f64 = (s.v[9] * s.dn[33][0]);
        let eq25_e269_d_n1: f64 = (s.v[9] * s.dn[33][1]);
        let eq25_e269_d_n2: f64 = (s.v[9] * s.dn[33][2]);
        let eq25_e269_d_n3: f64 = (s.v[9] * s.dn[33][3]);
        let eq25_e269_d_n4: f64 = (s.v[9] * s.dn[33][4]);
        let eq25_e269_d_n5: f64 = (s.v[9] * s.dn[33][5]);
        let eq25_e269_d_n6: f64 = (s.v[9] * s.dn[33][6]);
        let eq25_e269_d_b0: f64 = (s.v[9] * s.db[33][0]);
        let eq25_e269_d_b1: f64 = (s.v[9] * s.db[33][1]);
        let eq25_e269_d_b2: f64 = (s.v[9] * s.db[33][2]);
        let eq25_e269_d_b3: f64 = (s.v[9] * s.db[33][3]);
        let eq25_e269_d_b4: f64 = (s.v[9] * s.db[33][4]);
        let eq25_e269_d_b5: f64 = (s.v[9] * s.db[33][5]);
        let eq25_e269_d_b6: f64 = (s.v[9] * s.db[33][6]);
        let eq25_e271: f64 = (eq25_e269 * s.v[3]);
        let eq25_e271_d_n0: f64 = (eq25_e269_d_n0 * s.v[3]);
        let eq25_e271_d_n1: f64 = (eq25_e269_d_n1 * s.v[3]);
        let eq25_e271_d_n2: f64 = (eq25_e269_d_n2 * s.v[3]);
        let eq25_e271_d_n3: f64 = (eq25_e269_d_n3 * s.v[3]);
        let eq25_e271_d_n4: f64 = (eq25_e269_d_n4 * s.v[3]);
        let eq25_e271_d_n5: f64 = (eq25_e269_d_n5 * s.v[3]);
        let eq25_e271_d_n6: f64 = (eq25_e269_d_n6 * s.v[3]);
        let eq25_e271_d_b0: f64 = (eq25_e269_d_b0 * s.v[3]);
        let eq25_e271_d_b1: f64 = (eq25_e269_d_b1 * s.v[3]);
        let eq25_e271_d_b2: f64 = (eq25_e269_d_b2 * s.v[3]);
        let eq25_e271_d_b3: f64 = (eq25_e269_d_b3 * s.v[3]);
        let eq25_e271_d_b4: f64 = (eq25_e269_d_b4 * s.v[3]);
        let eq25_e271_d_b5: f64 = (eq25_e269_d_b5 * s.v[3]);
        let eq25_e271_d_b6: f64 = (eq25_e269_d_b6 * s.v[3]);
        let eq25_e272_q: f64 = eq25_e271;
        let eq25_reactive_node_derivatives: [f64; 7] = [eq25_e271_d_n0, eq25_e271_d_n1, eq25_e271_d_n2, eq25_e271_d_n3, eq25_e271_d_n4, eq25_e271_d_n5, eq25_e271_d_n6];
        let eq25_reactive_branch_derivatives: [f64; 7] = [eq25_e271_d_b0, eq25_e271_d_b1, eq25_e271_d_b2, eq25_e271_d_b3, eq25_e271_d_b4, eq25_e271_d_b5, eq25_e271_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e275: f64 = (s.v[9] * s.v[32]);
        let eq26_e275_d_n0: f64 = (s.v[9] * s.dn[32][0]);
        let eq26_e275_d_n1: f64 = (s.v[9] * s.dn[32][1]);
        let eq26_e275_d_n2: f64 = (s.v[9] * s.dn[32][2]);
        let eq26_e275_d_n3: f64 = (s.v[9] * s.dn[32][3]);
        let eq26_e275_d_n4: f64 = (s.v[9] * s.dn[32][4]);
        let eq26_e275_d_n5: f64 = (s.v[9] * s.dn[32][5]);
        let eq26_e275_d_n6: f64 = (s.v[9] * s.dn[32][6]);
        let eq26_e275_d_b0: f64 = (s.v[9] * s.db[32][0]);
        let eq26_e275_d_b1: f64 = (s.v[9] * s.db[32][1]);
        let eq26_e275_d_b2: f64 = (s.v[9] * s.db[32][2]);
        let eq26_e275_d_b3: f64 = (s.v[9] * s.db[32][3]);
        let eq26_e275_d_b4: f64 = (s.v[9] * s.db[32][4]);
        let eq26_e275_d_b5: f64 = (s.v[9] * s.db[32][5]);
        let eq26_e275_d_b6: f64 = (s.v[9] * s.db[32][6]);
        let eq26_e277: f64 = (eq26_e275 * s.v[3]);
        let eq26_e277_d_n0: f64 = (eq26_e275_d_n0 * s.v[3]);
        let eq26_e277_d_n1: f64 = (eq26_e275_d_n1 * s.v[3]);
        let eq26_e277_d_n2: f64 = (eq26_e275_d_n2 * s.v[3]);
        let eq26_e277_d_n3: f64 = (eq26_e275_d_n3 * s.v[3]);
        let eq26_e277_d_n4: f64 = (eq26_e275_d_n4 * s.v[3]);
        let eq26_e277_d_n5: f64 = (eq26_e275_d_n5 * s.v[3]);
        let eq26_e277_d_n6: f64 = (eq26_e275_d_n6 * s.v[3]);
        let eq26_e277_d_b0: f64 = (eq26_e275_d_b0 * s.v[3]);
        let eq26_e277_d_b1: f64 = (eq26_e275_d_b1 * s.v[3]);
        let eq26_e277_d_b2: f64 = (eq26_e275_d_b2 * s.v[3]);
        let eq26_e277_d_b3: f64 = (eq26_e275_d_b3 * s.v[3]);
        let eq26_e277_d_b4: f64 = (eq26_e275_d_b4 * s.v[3]);
        let eq26_e277_d_b5: f64 = (eq26_e275_d_b5 * s.v[3]);
        let eq26_e277_d_b6: f64 = (eq26_e275_d_b6 * s.v[3]);
        let eq26_e278_q: f64 = eq26_e277;
        let eq26_reactive_node_derivatives: [f64; 7] = [eq26_e277_d_n0, eq26_e277_d_n1, eq26_e277_d_n2, eq26_e277_d_n3, eq26_e277_d_n4, eq26_e277_d_n5, eq26_e277_d_n6];
        let eq26_reactive_branch_derivatives: [f64; 7] = [eq26_e277_d_b0, eq26_e277_d_b1, eq26_e277_d_b2, eq26_e277_d_b3, eq26_e277_d_b4, eq26_e277_d_b5, eq26_e277_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[4]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
