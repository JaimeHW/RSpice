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
        let (eq0_e60, eq0_e60_d_n0, eq0_e60_d_n1, eq0_e60_d_n2, eq0_e60_d_n3, eq0_e60_d_n4, eq0_e60_d_n5, eq0_e60_d_n6, eq0_e60_d_b0, eq0_e60_d_b1, eq0_e60_d_b2, eq0_e60_d_b3, eq0_e60_d_b4, eq0_e60_d_b5, eq0_e60_d_b6,) = {
    if s.b[68] {
        let eq0_e56: f64 = (-s.v[23]);
        let eq0_e56_d_n0: f64 = (-s.dn[23][0]);
        let eq0_e56_d_n1: f64 = (-s.dn[23][1]);
        let eq0_e56_d_n2: f64 = (-s.dn[23][2]);
        let eq0_e56_d_n3: f64 = (-s.dn[23][3]);
        let eq0_e56_d_n4: f64 = (-s.dn[23][4]);
        let eq0_e56_d_n5: f64 = (-s.dn[23][5]);
        let eq0_e56_d_n6: f64 = (-s.dn[23][6]);
        let eq0_e56_d_b0: f64 = (-s.db[23][0]);
        let eq0_e56_d_b1: f64 = (-s.db[23][1]);
        let eq0_e56_d_b2: f64 = (-s.db[23][2]);
        let eq0_e56_d_b3: f64 = (-s.db[23][3]);
        let eq0_e56_d_b4: f64 = (-s.db[23][4]);
        let eq0_e56_d_b5: f64 = (-s.db[23][5]);
        let eq0_e56_d_b6: f64 = (-s.db[23][6]);
        let eq0_e58: f64 = (eq0_e56 * s.v[31]);
        let eq0_e58_d_n0: f64 = ((eq0_e56_d_n0 * s.v[31]) + (eq0_e56 * s.dn[31][0]));
        let eq0_e58_d_n1: f64 = ((eq0_e56_d_n1 * s.v[31]) + (eq0_e56 * s.dn[31][1]));
        let eq0_e58_d_n2: f64 = ((eq0_e56_d_n2 * s.v[31]) + (eq0_e56 * s.dn[31][2]));
        let eq0_e58_d_n3: f64 = ((eq0_e56_d_n3 * s.v[31]) + (eq0_e56 * s.dn[31][3]));
        let eq0_e58_d_n4: f64 = ((eq0_e56_d_n4 * s.v[31]) + (eq0_e56 * s.dn[31][4]));
        let eq0_e58_d_n5: f64 = ((eq0_e56_d_n5 * s.v[31]) + (eq0_e56 * s.dn[31][5]));
        let eq0_e58_d_n6: f64 = ((eq0_e56_d_n6 * s.v[31]) + (eq0_e56 * s.dn[31][6]));
        let eq0_e58_d_b0: f64 = ((eq0_e56_d_b0 * s.v[31]) + (eq0_e56 * s.db[31][0]));
        let eq0_e58_d_b1: f64 = ((eq0_e56_d_b1 * s.v[31]) + (eq0_e56 * s.db[31][1]));
        let eq0_e58_d_b2: f64 = ((eq0_e56_d_b2 * s.v[31]) + (eq0_e56 * s.db[31][2]));
        let eq0_e58_d_b3: f64 = ((eq0_e56_d_b3 * s.v[31]) + (eq0_e56 * s.db[31][3]));
        let eq0_e58_d_b4: f64 = ((eq0_e56_d_b4 * s.v[31]) + (eq0_e56 * s.db[31][4]));
        let eq0_e58_d_b5: f64 = ((eq0_e56_d_b5 * s.v[31]) + (eq0_e56 * s.db[31][5]));
        let eq0_e58_d_b6: f64 = ((eq0_e56_d_b6 * s.v[31]) + (eq0_e56 * s.db[31][6]));
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
        let (eq3_e78,) = {
    if (!s.b[68]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e78;
        stamper.stamp_potential_const_local(
            0,
            eq3_value,
        );
        let (eq4_e88, eq4_e88_d_n0, eq4_e88_d_n1, eq4_e88_d_n2, eq4_e88_d_n3, eq4_e88_d_n4, eq4_e88_d_n5, eq4_e88_d_n6, eq4_e88_d_b0, eq4_e88_d_b1, eq4_e88_d_b2, eq4_e88_d_b3, eq4_e88_d_b4, eq4_e88_d_b5, eq4_e88_d_b6,) = {
    if s.b[70] {
        let eq4_e81: f64 = (-1.0);
        let eq4_e84: f64 = (s.v[24] * (nv0 - nv1));
        let eq4_e84_d_n0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let eq4_e84_d_n1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let eq4_e84_d_n2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let eq4_e84_d_n3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let eq4_e84_d_n4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let eq4_e84_d_n5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let eq4_e84_d_n6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let eq4_e84_d_b0: f64 = (s.db[24][0] * (nv0 - nv1));
        let eq4_e84_d_b1: f64 = (s.db[24][1] * (nv0 - nv1));
        let eq4_e84_d_b2: f64 = (s.db[24][2] * (nv0 - nv1));
        let eq4_e84_d_b3: f64 = (s.db[24][3] * (nv0 - nv1));
        let eq4_e84_d_b4: f64 = (s.db[24][4] * (nv0 - nv1));
        let eq4_e84_d_b5: f64 = (s.db[24][5] * (nv0 - nv1));
        let eq4_e84_d_b6: f64 = (s.db[24][6] * (nv0 - nv1));
        let eq4_e85: f64 = (eq4_e84).abs();
        let eq4_e85_d_n0: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n0 } else { (-eq4_e84_d_n0) };
        let eq4_e85_d_n1: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n1 } else { (-eq4_e84_d_n1) };
        let eq4_e85_d_n2: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n2 } else { (-eq4_e84_d_n2) };
        let eq4_e85_d_n3: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n3 } else { (-eq4_e84_d_n3) };
        let eq4_e85_d_n4: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n4 } else { (-eq4_e84_d_n4) };
        let eq4_e85_d_n5: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n5 } else { (-eq4_e84_d_n5) };
        let eq4_e85_d_n6: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_n6 } else { (-eq4_e84_d_n6) };
        let eq4_e85_d_b0: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b0 } else { (-eq4_e84_d_b0) };
        let eq4_e85_d_b1: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b1 } else { (-eq4_e84_d_b1) };
        let eq4_e85_d_b2: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b2 } else { (-eq4_e84_d_b2) };
        let eq4_e85_d_b3: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b3 } else { (-eq4_e84_d_b3) };
        let eq4_e85_d_b4: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b4 } else { (-eq4_e84_d_b4) };
        let eq4_e85_d_b5: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b5 } else { (-eq4_e84_d_b5) };
        let eq4_e85_d_b6: f64 = if eq4_e84 >= 0.0 { eq4_e84_d_b6 } else { (-eq4_e84_d_b6) };
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
        let eq5_e92: f64 = ((nv2 - 0.0) / p.p33);
        let eq5_e92_d_n2: f64 = (1.0 / p.p33);
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
        let eq6_e98_d_n2: f64 = p.p34;
        let eq6_e99: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq6_e98);
        let eq6_e99_d_n2: f64 = (eq6_e98_d_n2 * ddt_scale);
        (eq6_e99, eq6_e99_d_n2,)
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
        let (eq7_e105,) = {
    if s.b[70] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e105;
        stamper.stamp_potential_const_local(
            1,
            eq7_value,
        );
        let (eq8_e118, eq8_e118_d_n0, eq8_e118_d_n1, eq8_e118_d_n2, eq8_e118_d_n3, eq8_e118_d_n4, eq8_e118_d_n5, eq8_e118_d_n6, eq8_e118_d_b0, eq8_e118_d_b1, eq8_e118_d_b2, eq8_e118_d_b3, eq8_e118_d_b4, eq8_e118_d_b5, eq8_e118_d_b6,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq8_e111: f64 = (-1.0);
        let eq8_e114: f64 = (s.v[24] * (nv0 - nv1));
        let eq8_e114_d_n0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let eq8_e114_d_n1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let eq8_e114_d_n2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let eq8_e114_d_n3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let eq8_e114_d_n4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let eq8_e114_d_n5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let eq8_e114_d_n6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let eq8_e114_d_b0: f64 = (s.db[24][0] * (nv0 - nv1));
        let eq8_e114_d_b1: f64 = (s.db[24][1] * (nv0 - nv1));
        let eq8_e114_d_b2: f64 = (s.db[24][2] * (nv0 - nv1));
        let eq8_e114_d_b3: f64 = (s.db[24][3] * (nv0 - nv1));
        let eq8_e114_d_b4: f64 = (s.db[24][4] * (nv0 - nv1));
        let eq8_e114_d_b5: f64 = (s.db[24][5] * (nv0 - nv1));
        let eq8_e114_d_b6: f64 = (s.db[24][6] * (nv0 - nv1));
        let eq8_e115: f64 = (eq8_e114).abs();
        let eq8_e115_d_n0: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n0 } else { (-eq8_e114_d_n0) };
        let eq8_e115_d_n1: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n1 } else { (-eq8_e114_d_n1) };
        let eq8_e115_d_n2: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n2 } else { (-eq8_e114_d_n2) };
        let eq8_e115_d_n3: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n3 } else { (-eq8_e114_d_n3) };
        let eq8_e115_d_n4: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n4 } else { (-eq8_e114_d_n4) };
        let eq8_e115_d_n5: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n5 } else { (-eq8_e114_d_n5) };
        let eq8_e115_d_n6: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_n6 } else { (-eq8_e114_d_n6) };
        let eq8_e115_d_b0: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b0 } else { (-eq8_e114_d_b0) };
        let eq8_e115_d_b1: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b1 } else { (-eq8_e114_d_b1) };
        let eq8_e115_d_b2: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b2 } else { (-eq8_e114_d_b2) };
        let eq8_e115_d_b3: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b3 } else { (-eq8_e114_d_b3) };
        let eq8_e115_d_b4: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b4 } else { (-eq8_e114_d_b4) };
        let eq8_e115_d_b5: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b5 } else { (-eq8_e114_d_b5) };
        let eq8_e115_d_b6: f64 = if eq8_e114 >= 0.0 { eq8_e114_d_b6 } else { (-eq8_e114_d_b6) };
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
        let eq9_e125: f64 = ((nv2 - nv5) / p.p33);
        let eq9_e125_d_n2: f64 = (1.0 / p.p33);
        let eq9_e125_d_n5: f64 = (-1.0 / p.p33);
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
        let eq10_e134_d_n2: f64 = p.p34;
        let eq10_e135: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq10_e134);
        let eq10_e135_d_n2: f64 = (eq10_e134_d_n2 * ddt_scale);
        (eq10_e135, eq10_e135_d_n2,)
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
        let eq11_e144: f64 = ((nv5 - 0.0) / p.p35);
        let eq11_e144_d_n5: f64 = (1.0 / p.p35);
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
        let eq12_e153_d_n5: f64 = p.p36;
        let eq12_e154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq12_e153);
        let eq12_e154_d_n5: f64 = (eq12_e153_d_n5 * ddt_scale);
        (eq12_e154, eq12_e154_d_n5,)
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
        let eq13_e168_d_n0: f64 = ((s.dn[24][0] * (nv0 - nv1)) + s.v[24]);
        let eq13_e168_d_n1: f64 = ((s.dn[24][1] * (nv0 - nv1)) + (-s.v[24]));
        let eq13_e168_d_n2: f64 = (s.dn[24][2] * (nv0 - nv1));
        let eq13_e168_d_n3: f64 = (s.dn[24][3] * (nv0 - nv1));
        let eq13_e168_d_n4: f64 = (s.dn[24][4] * (nv0 - nv1));
        let eq13_e168_d_n5: f64 = (s.dn[24][5] * (nv0 - nv1));
        let eq13_e168_d_n6: f64 = (s.dn[24][6] * (nv0 - nv1));
        let eq13_e168_d_b0: f64 = (s.db[24][0] * (nv0 - nv1));
        let eq13_e168_d_b1: f64 = (s.db[24][1] * (nv0 - nv1));
        let eq13_e168_d_b2: f64 = (s.db[24][2] * (nv0 - nv1));
        let eq13_e168_d_b3: f64 = (s.db[24][3] * (nv0 - nv1));
        let eq13_e168_d_b4: f64 = (s.db[24][4] * (nv0 - nv1));
        let eq13_e168_d_b5: f64 = (s.db[24][5] * (nv0 - nv1));
        let eq13_e168_d_b6: f64 = (s.db[24][6] * (nv0 - nv1));
        let eq13_e169: f64 = (eq13_e168).abs();
        let eq13_e169_d_n0: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n0 } else { (-eq13_e168_d_n0) };
        let eq13_e169_d_n1: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n1 } else { (-eq13_e168_d_n1) };
        let eq13_e169_d_n2: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n2 } else { (-eq13_e168_d_n2) };
        let eq13_e169_d_n3: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n3 } else { (-eq13_e168_d_n3) };
        let eq13_e169_d_n4: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n4 } else { (-eq13_e168_d_n4) };
        let eq13_e169_d_n5: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n5 } else { (-eq13_e168_d_n5) };
        let eq13_e169_d_n6: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_n6 } else { (-eq13_e168_d_n6) };
        let eq13_e169_d_b0: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b0 } else { (-eq13_e168_d_b0) };
        let eq13_e169_d_b1: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b1 } else { (-eq13_e168_d_b1) };
        let eq13_e169_d_b2: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b2 } else { (-eq13_e168_d_b2) };
        let eq13_e169_d_b3: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b3 } else { (-eq13_e168_d_b3) };
        let eq13_e169_d_b4: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b4 } else { (-eq13_e168_d_b4) };
        let eq13_e169_d_b5: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b5 } else { (-eq13_e168_d_b5) };
        let eq13_e169_d_b6: f64 = if eq13_e168 >= 0.0 { eq13_e168_d_b6 } else { (-eq13_e168_d_b6) };
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
        let (eq14_e182,) = {
    if (((!s.b[70]) && (!s.b[71])) && s.b[72]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e182;
        stamper.stamp_potential_const_local(
            2,
            eq14_value,
        );
        let (eq15_e193,) = {
    if (((!s.b[70]) && (!s.b[71])) && (!s.b[72])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e193;
        stamper.stamp_potential_const_local(
            3,
            eq15_value,
        );
        let (eq16_e204,) = {
    if (((!s.b[70]) && (!s.b[71])) && (!s.b[72])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e204;
        stamper.stamp_potential_const_local(
            4,
            eq16_value,
        );
        let eq17_e207: f64 = 0.0;
        let eq17_e209: f64 = (eq17_e207 * (nv3 - nv4));
        let eq17_e209_d_n4: f64 = (-eq17_e207);
        let eq17_value: f64 = eq17_e209;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(4),
            multiplicity * (eq17_value),
            3,
            multiplicity * (eq17_e207),
            4,
            multiplicity * (eq17_e209_d_n4),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq18_e224, eq18_e224_d_n0, eq18_e224_d_n1, eq18_e224_d_n2, eq18_e224_d_n3, eq18_e224_d_n4, eq18_e224_d_n5, eq18_e224_d_n6, eq18_e224_d_b0, eq18_e224_d_b1, eq18_e224_d_b2, eq18_e224_d_b3, eq18_e224_d_b4, eq18_e224_d_b5, eq18_e224_d_b6,) = {
    if s.b[73] {
        let eq18_e214: f64 = (s.v[29] / s.v[3]);
        let eq18_e214_d_n0: f64 = (s.dn[29][0] / s.v[3]);
        let eq18_e214_d_n1: f64 = (s.dn[29][1] / s.v[3]);
        let eq18_e214_d_n2: f64 = (s.dn[29][2] / s.v[3]);
        let eq18_e214_d_n3: f64 = (s.dn[29][3] / s.v[3]);
        let eq18_e214_d_n4: f64 = (s.dn[29][4] / s.v[3]);
        let eq18_e214_d_n5: f64 = (s.dn[29][5] / s.v[3]);
        let eq18_e214_d_n6: f64 = (s.dn[29][6] / s.v[3]);
        let eq18_e214_d_b0: f64 = (s.db[29][0] / s.v[3]);
        let eq18_e214_d_b1: f64 = (s.db[29][1] / s.v[3]);
        let eq18_e214_d_b2: f64 = (s.db[29][2] / s.v[3]);
        let eq18_e214_d_b3: f64 = (s.db[29][3] / s.v[3]);
        let eq18_e214_d_b4: f64 = (s.db[29][4] / s.v[3]);
        let eq18_e214_d_b5: f64 = (s.db[29][5] / s.v[3]);
        let eq18_e214_d_b6: f64 = (s.db[29][6] / s.v[3]);
        let (eq18_e221, eq18_e221_d_n0, eq18_e221_d_n1, eq18_e221_d_n2, eq18_e221_d_n3, eq18_e221_d_n4, eq18_e221_d_n5, eq18_e221_d_n6, eq18_e221_d_b0, eq18_e221_d_b1, eq18_e221_d_b2, eq18_e221_d_b3, eq18_e221_d_b4, eq18_e221_d_b5, eq18_e221_d_b6,) = {
            if (eq18_e214 > p.p46) {
                let eq18_e219: f64 = (s.v[29] / s.v[3]);
                let eq18_e219_d_n0: f64 = (s.dn[29][0] / s.v[3]);
                let eq18_e219_d_n1: f64 = (s.dn[29][1] / s.v[3]);
                let eq18_e219_d_n2: f64 = (s.dn[29][2] / s.v[3]);
                let eq18_e219_d_n3: f64 = (s.dn[29][3] / s.v[3]);
                let eq18_e219_d_n4: f64 = (s.dn[29][4] / s.v[3]);
                let eq18_e219_d_n5: f64 = (s.dn[29][5] / s.v[3]);
                let eq18_e219_d_n6: f64 = (s.dn[29][6] / s.v[3]);
                let eq18_e219_d_b0: f64 = (s.db[29][0] / s.v[3]);
                let eq18_e219_d_b1: f64 = (s.db[29][1] / s.v[3]);
                let eq18_e219_d_b2: f64 = (s.db[29][2] / s.v[3]);
                let eq18_e219_d_b3: f64 = (s.db[29][3] / s.v[3]);
                let eq18_e219_d_b4: f64 = (s.db[29][4] / s.v[3]);
                let eq18_e219_d_b5: f64 = (s.db[29][5] / s.v[3]);
                let eq18_e219_d_b6: f64 = (s.db[29][6] / s.v[3]);
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
        let (eq19_e230,) = {
    if s.b[73] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e230;
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (eq19_value),
        );
        let (eq20_e235,) = {
    if (!s.b[73]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e235;
        stamper.stamp_potential_const_local(
            5,
            eq20_value,
        );
        let (eq21_e250, eq21_e250_d_n0, eq21_e250_d_n1, eq21_e250_d_n2, eq21_e250_d_n3, eq21_e250_d_n4, eq21_e250_d_n5, eq21_e250_d_n6, eq21_e250_d_b0, eq21_e250_d_b1, eq21_e250_d_b2, eq21_e250_d_b3, eq21_e250_d_b4, eq21_e250_d_b5, eq21_e250_d_b6,) = {
    if s.b[74] {
        let eq21_e240: f64 = (s.v[30] / s.v[3]);
        let eq21_e240_d_n0: f64 = (s.dn[30][0] / s.v[3]);
        let eq21_e240_d_n1: f64 = (s.dn[30][1] / s.v[3]);
        let eq21_e240_d_n2: f64 = (s.dn[30][2] / s.v[3]);
        let eq21_e240_d_n3: f64 = (s.dn[30][3] / s.v[3]);
        let eq21_e240_d_n4: f64 = (s.dn[30][4] / s.v[3]);
        let eq21_e240_d_n5: f64 = (s.dn[30][5] / s.v[3]);
        let eq21_e240_d_n6: f64 = (s.dn[30][6] / s.v[3]);
        let eq21_e240_d_b0: f64 = (s.db[30][0] / s.v[3]);
        let eq21_e240_d_b1: f64 = (s.db[30][1] / s.v[3]);
        let eq21_e240_d_b2: f64 = (s.db[30][2] / s.v[3]);
        let eq21_e240_d_b3: f64 = (s.db[30][3] / s.v[3]);
        let eq21_e240_d_b4: f64 = (s.db[30][4] / s.v[3]);
        let eq21_e240_d_b5: f64 = (s.db[30][5] / s.v[3]);
        let eq21_e240_d_b6: f64 = (s.db[30][6] / s.v[3]);
        let (eq21_e247, eq21_e247_d_n0, eq21_e247_d_n1, eq21_e247_d_n2, eq21_e247_d_n3, eq21_e247_d_n4, eq21_e247_d_n5, eq21_e247_d_n6, eq21_e247_d_b0, eq21_e247_d_b1, eq21_e247_d_b2, eq21_e247_d_b3, eq21_e247_d_b4, eq21_e247_d_b5, eq21_e247_d_b6,) = {
            if (eq21_e240 > p.p46) {
                let eq21_e245: f64 = (s.v[30] / s.v[3]);
                let eq21_e245_d_n0: f64 = (s.dn[30][0] / s.v[3]);
                let eq21_e245_d_n1: f64 = (s.dn[30][1] / s.v[3]);
                let eq21_e245_d_n2: f64 = (s.dn[30][2] / s.v[3]);
                let eq21_e245_d_n3: f64 = (s.dn[30][3] / s.v[3]);
                let eq21_e245_d_n4: f64 = (s.dn[30][4] / s.v[3]);
                let eq21_e245_d_n5: f64 = (s.dn[30][5] / s.v[3]);
                let eq21_e245_d_n6: f64 = (s.dn[30][6] / s.v[3]);
                let eq21_e245_d_b0: f64 = (s.db[30][0] / s.v[3]);
                let eq21_e245_d_b1: f64 = (s.db[30][1] / s.v[3]);
                let eq21_e245_d_b2: f64 = (s.db[30][2] / s.v[3]);
                let eq21_e245_d_b3: f64 = (s.db[30][3] / s.v[3]);
                let eq21_e245_d_b4: f64 = (s.db[30][4] / s.v[3]);
                let eq21_e245_d_b5: f64 = (s.db[30][5] / s.v[3]);
                let eq21_e245_d_b6: f64 = (s.db[30][6] / s.v[3]);
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
        let (eq22_e256,) = {
    if s.b[74] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e256;
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (eq22_value),
        );
        let (eq23_e261,) = {
    if (!s.b[74]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e261;
        stamper.stamp_potential_const_local(
            6,
            eq23_value,
        );
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
        let eq25_e272_d_n0: f64 = (eq25_e271_d_n0 * ddt_scale);
        let eq25_e272_d_n1: f64 = (eq25_e271_d_n1 * ddt_scale);
        let eq25_e272_d_n2: f64 = (eq25_e271_d_n2 * ddt_scale);
        let eq25_e272_d_n3: f64 = (eq25_e271_d_n3 * ddt_scale);
        let eq25_e272_d_n4: f64 = (eq25_e271_d_n4 * ddt_scale);
        let eq25_e272_d_n5: f64 = (eq25_e271_d_n5 * ddt_scale);
        let eq25_e272_d_n6: f64 = (eq25_e271_d_n6 * ddt_scale);
        let eq25_e272_d_b0: f64 = (eq25_e271_d_b0 * ddt_scale);
        let eq25_e272_d_b1: f64 = (eq25_e271_d_b1 * ddt_scale);
        let eq25_e272_d_b2: f64 = (eq25_e271_d_b2 * ddt_scale);
        let eq25_e272_d_b3: f64 = (eq25_e271_d_b3 * ddt_scale);
        let eq25_e272_d_b4: f64 = (eq25_e271_d_b4 * ddt_scale);
        let eq25_e272_d_b5: f64 = (eq25_e271_d_b5 * ddt_scale);
        let eq25_e272_d_b6: f64 = (eq25_e271_d_b6 * ddt_scale);
        let eq25_value: f64 = eq25_e272;
        let eq25_node_derivatives: [f64; 7] = [eq25_e272_d_n0, eq25_e272_d_n1, eq25_e272_d_n2, eq25_e272_d_n3, eq25_e272_d_n4, eq25_e272_d_n5, eq25_e272_d_n6];
        let eq25_branch_derivatives: [f64; 7] = [eq25_e272_d_b0, eq25_e272_d_b1, eq25_e272_d_b2, eq25_e272_d_b3, eq25_e272_d_b4, eq25_e272_d_b5, eq25_e272_d_b6];
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
        let eq26_e278_d_n0: f64 = (eq26_e277_d_n0 * ddt_scale);
        let eq26_e278_d_n1: f64 = (eq26_e277_d_n1 * ddt_scale);
        let eq26_e278_d_n2: f64 = (eq26_e277_d_n2 * ddt_scale);
        let eq26_e278_d_n3: f64 = (eq26_e277_d_n3 * ddt_scale);
        let eq26_e278_d_n4: f64 = (eq26_e277_d_n4 * ddt_scale);
        let eq26_e278_d_n5: f64 = (eq26_e277_d_n5 * ddt_scale);
        let eq26_e278_d_n6: f64 = (eq26_e277_d_n6 * ddt_scale);
        let eq26_e278_d_b0: f64 = (eq26_e277_d_b0 * ddt_scale);
        let eq26_e278_d_b1: f64 = (eq26_e277_d_b1 * ddt_scale);
        let eq26_e278_d_b2: f64 = (eq26_e277_d_b2 * ddt_scale);
        let eq26_e278_d_b3: f64 = (eq26_e277_d_b3 * ddt_scale);
        let eq26_e278_d_b4: f64 = (eq26_e277_d_b4 * ddt_scale);
        let eq26_e278_d_b5: f64 = (eq26_e277_d_b5 * ddt_scale);
        let eq26_e278_d_b6: f64 = (eq26_e277_d_b6 * ddt_scale);
        let eq26_value: f64 = eq26_e278;
        let eq26_node_derivatives: [f64; 7] = [eq26_e278_d_n0, eq26_e278_d_n1, eq26_e278_d_n2, eq26_e278_d_n3, eq26_e278_d_n4, eq26_e278_d_n5, eq26_e278_d_n6];
        let eq26_branch_derivatives: [f64; 7] = [eq26_e278_d_b0, eq26_e278_d_b1, eq26_e278_d_b2, eq26_e278_d_b3, eq26_e278_d_b4, eq26_e278_d_b5, eq26_e278_d_b6];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(4),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq27_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (eq27_value),
        );
        let eq28_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (eq28_value),
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
        let (eq6_e101, eq6_e101_d_n2, eq6_e101_q, eq6_e101_q_d_n2,) = {
    if s.b[70] {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e98_d_n2: f64 = p.p34;
        let eq6_e99_q: f64 = eq6_e98;
        (eq6_e98, eq6_e98_d_n2, eq6_e99_q, eq6_e98_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq6_e101_q_d_n2),
        );
        let (eq10_e137, eq10_e137_d_n2, eq10_e137_q, eq10_e137_q_d_n2,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e134_d_n2: f64 = p.p34;
        let eq10_e135_q: f64 = eq10_e134;
        (eq10_e134, eq10_e134_d_n2, eq10_e135_q, eq10_e134_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[2]),
            None,
            nodes[2],
            multiplicity * (eq10_e137_q_d_n2),
        );
        let (eq12_e156, eq12_e156_d_n5, eq12_e156_q, eq12_e156_q_d_n5,) = {
    if ((!s.b[70]) && s.b[71]) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e153_d_n5: f64 = p.p36;
        let eq12_e154_q: f64 = eq12_e153;
        (eq12_e153, eq12_e153_d_n5, eq12_e154_q, eq12_e153_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq12_e156_q_d_n5),
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
