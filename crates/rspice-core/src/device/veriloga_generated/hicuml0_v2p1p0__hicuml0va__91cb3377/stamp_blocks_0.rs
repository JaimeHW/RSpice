#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq21_e210, eq21_e210_d_n4,) = {
    if (locals.var_guard115 == 0.0) {
        (locals.var_i_cth, locals.var_i_cth_dn4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e210;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq21_value),
            4,
            multiplicity * (eq21_e210_d_n4),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq21_e210, eq21_e210_d_n4, eq21_e210_q, eq21_e210_q_d_n4,) = {
    if (locals.var_guard115 == 0.0) {
        let eq21_e208_q: f64 = locals.var_i_cth_rv;
        (locals.var_i_cth, locals.var_i_cth_dn4, eq21_e208_q, locals.var_i_cth_rdn4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq21_e210_q_d_n4),
        );
    }
}
