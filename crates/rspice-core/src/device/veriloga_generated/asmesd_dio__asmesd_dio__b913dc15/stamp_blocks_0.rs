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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n6,) = {
    if (locals.var_guard8 != 0.0) {
        let eq2_e70: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (nv6 - 0.0));
        let eq2_e71: f64 = (locals.var_tff * eq2_e70);
        let eq2_e71_d_n0: f64 = (locals.var_tff_dn0 * eq2_e70);
        let eq2_e71_d_n1: f64 = (locals.var_tff_dn1 * eq2_e70);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, (locals.var_tff * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e73;
        stamper.stamp_current_node3_local(
            Some(6),
            None,
            multiplicity * (eq2_value),
            0,
            multiplicity * (eq2_e73_d_n0),
            1,
            multiplicity * (eq2_e73_d_n1),
            6,
            multiplicity * (eq2_e73_d_n6),
        );
        let (eq6_e101, eq6_e101_d_n2,) = {
    if (locals.var_guard10 != 0.0) {
        let eq6_e98: f64 = ((nv2 - 0.0) * p.p34);
        let eq6_e99: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq6_e98);
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
        let (eq10_e137, eq10_e137_d_n2,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let eq10_e134: f64 = (p.p34 * (nv2 - 0.0));
        let eq10_e135: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq10_e134);
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
        let (eq12_e156, eq12_e156_d_n5,) = {
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
        let eq12_e153: f64 = (p.p36 * (nv5 - 0.0));
        let eq12_e154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq12_e153);
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
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq2_e73, eq2_e73_d_n0, eq2_e73_d_n1, eq2_e73_d_n6, eq2_e73_q, eq2_e73_q_d_n0, eq2_e73_q_d_n1,) = {
    if (locals.var_guard8 != 0.0) {
        let eq2_e70_q: f64 = (nv6 - 0.0);
        let eq2_e71: f64 = (locals.var_tff * (nv6 - 0.0));
        let eq2_e71_d_n0: f64 = (locals.var_tff_dn0 * (nv6 - 0.0));
        let eq2_e71_d_n1: f64 = (locals.var_tff_dn1 * (nv6 - 0.0));
        let eq2_e71_q: f64 = (locals.var_tff * eq2_e70_q);
        let eq2_e71_q_d_n0: f64 = (locals.var_tff_dn0 * eq2_e70_q);
        let eq2_e71_q_d_n1: f64 = (locals.var_tff_dn1 * eq2_e70_q);
        (eq2_e71, eq2_e71_d_n0, eq2_e71_d_n1, locals.var_tff, eq2_e71_q, eq2_e71_q_d_n0, eq2_e71_q_d_n1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[6]),
            None,
            nodes[0],
            multiplicity * (eq2_e73_q_d_n0),
            nodes[1],
            multiplicity * (eq2_e73_q_d_n1),
            nodes[6],
            multiplicity * (eq2_e73_d_n6),
        );
        let (eq6_e101, eq6_e101_d_n2, eq6_e101_q,) = {
    if (locals.var_guard10 != 0.0) {
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
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
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
    if ((locals.var_guard10 == 0.0) && (locals.var_guard11 != 0.0)) {
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
    }
}
