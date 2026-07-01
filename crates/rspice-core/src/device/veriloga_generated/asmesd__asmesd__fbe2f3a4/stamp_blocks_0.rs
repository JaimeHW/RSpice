#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv9 - 0.0));
        let eq2_e99: f64 = (p.p83 * eq2_e98);
        let eq2_value: f64 = eq2_e99;
        stamper.stamp_current_node1_local(
            Some(9),
            None,
            multiplicity * (eq2_value),
            9,
            multiplicity * ((p.p83 * ddt_scale)),
        );
        let (eq5_e121, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n8,) = {
    if (locals.var_guard13 != 0.0) {
        let eq5_e118: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (nv8 - 0.0));
        let eq5_e119: f64 = (locals.var_tff * eq5_e118);
        let eq5_e119_d_n1: f64 = (locals.var_tff_dn1 * eq5_e118);
        let eq5_e119_d_n2: f64 = (locals.var_tff_dn2 * eq5_e118);
        (eq5_e119, eq5_e119_d_n1, eq5_e119_d_n2, (locals.var_tff * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e121;
        stamper.stamp_current_node3_local(
            Some(8),
            None,
            multiplicity * (eq5_value),
            1,
            multiplicity * (eq5_e121_d_n1),
            2,
            multiplicity * (eq5_e121_d_n2),
            8,
            multiplicity * (eq5_e121_d_n8),
        );
        let (eq9_e154, eq9_e154_d_n3,) = {
    if (locals.var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq9_e151);
        (eq9_e152, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e154;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq13_e195, eq13_e195_d_n3,) = {
    if ((locals.var_guard20 == 0.0) && (locals.var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e192);
        (eq13_e193, (p.p34 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e195;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq13_value),
            3,
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq15_e214, eq15_e214_d_n7,) = {
    if ((locals.var_guard20 == 0.0) && (locals.var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e211);
        (eq15_e212, (p.p36 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e214;
        stamper.stamp_current_node1_local(
            Some(7),
            None,
            multiplicity * (eq15_value),
            7,
            multiplicity * (eq15_e214_d_n7),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq2_e98_q: f64 = (nv9 - 0.0);
        let eq2_e99: f64 = (p.p83 * (nv9 - 0.0));
        let eq2_e99_q: f64 = (p.p83 * eq2_e98_q);
        stamper.stamp_current_reactive_node1(
            Some(nodes[9]),
            None,
            nodes[9],
            multiplicity * (p.p83),
        );
        let (eq5_e121, eq5_e121_d_n1, eq5_e121_d_n2, eq5_e121_d_n8, eq5_e121_q, eq5_e121_q_d_n1, eq5_e121_q_d_n2,) = {
    if (locals.var_guard13 != 0.0) {
        let eq5_e118_q: f64 = (nv8 - 0.0);
        let eq5_e119: f64 = (locals.var_tff * (nv8 - 0.0));
        let eq5_e119_d_n1: f64 = (locals.var_tff_dn1 * (nv8 - 0.0));
        let eq5_e119_d_n2: f64 = (locals.var_tff_dn2 * (nv8 - 0.0));
        let eq5_e119_q: f64 = (locals.var_tff * eq5_e118_q);
        let eq5_e119_q_d_n1: f64 = (locals.var_tff_dn1 * eq5_e118_q);
        let eq5_e119_q_d_n2: f64 = (locals.var_tff_dn2 * eq5_e118_q);
        (eq5_e119, eq5_e119_d_n1, eq5_e119_d_n2, locals.var_tff, eq5_e119_q, eq5_e119_q_d_n1, eq5_e119_q_d_n2,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[8]),
            None,
            nodes[1],
            multiplicity * (eq5_e121_q_d_n1),
            nodes[2],
            multiplicity * (eq5_e121_q_d_n2),
            nodes[8],
            multiplicity * (eq5_e121_d_n8),
        );
        let (eq9_e154, eq9_e154_d_n3, eq9_e154_q,) = {
    if (locals.var_guard20 != 0.0) {
        let eq9_e151: f64 = ((nv3 - 0.0) * p.p34);
        let eq9_e152_q: f64 = eq9_e151;
        (eq9_e151, p.p34, eq9_e152_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq9_e154_d_n3),
        );
        let (eq13_e195, eq13_e195_d_n3, eq13_e195_q,) = {
    if ((locals.var_guard20 == 0.0) && (locals.var_guard21 != 0.0)) {
        let eq13_e192: f64 = (p.p34 * (nv3 - 0.0));
        let eq13_e193_q: f64 = eq13_e192;
        (eq13_e192, p.p34, eq13_e193_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq13_e195_d_n3),
        );
        let (eq15_e214, eq15_e214_d_n7, eq15_e214_q,) = {
    if ((locals.var_guard20 == 0.0) && (locals.var_guard21 != 0.0)) {
        let eq15_e211: f64 = (p.p36 * (nv7 - 0.0));
        let eq15_e212_q: f64 = eq15_e211;
        (eq15_e211, p.p36, eq15_e212_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[7]),
            None,
            nodes[7],
            multiplicity * (eq15_e214_d_n7),
        );
    }
}
