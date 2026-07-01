#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        var_guard115: f64,
        var_i_cth: f64,
        var_i_cth_dn4: f64,
    ) {
        let (eq21_e210, eq21_e210_d_n4,) = {
    if (var_guard115 == 0.0) {
        (var_i_cth, var_i_cth_dn4,)
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
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard115: f64,
        var_i_cth: f64,
        var_i_cth_db0: f64,
        var_i_cth_db1: f64,
        var_i_cth_db2: f64,
        var_i_cth_db3: f64,
        var_i_cth_dn0: f64,
        var_i_cth_dn1: f64,
        var_i_cth_dn2: f64,
        var_i_cth_dn3: f64,
        var_i_cth_dn4: f64,
        var_i_cth_dn5: f64,
        var_i_cth_dn6: f64,
        var_i_cth_dn7: f64,
        var_i_cth_dn8: f64,
        var_i_cth_dn9: f64,
        var_i_cth_rdb0: f64,
        var_i_cth_rdb1: f64,
        var_i_cth_rdb2: f64,
        var_i_cth_rdb3: f64,
        var_i_cth_rdn0: f64,
        var_i_cth_rdn1: f64,
        var_i_cth_rdn2: f64,
        var_i_cth_rdn3: f64,
        var_i_cth_rdn4: f64,
        var_i_cth_rdn5: f64,
        var_i_cth_rdn6: f64,
        var_i_cth_rdn7: f64,
        var_i_cth_rdn8: f64,
        var_i_cth_rdn9: f64,
        var_i_cth_rv: f64,
    ) {
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3, eq21_e210_q, eq21_e210_q_d_n0, eq21_e210_q_d_n1, eq21_e210_q_d_n2, eq21_e210_q_d_n3, eq21_e210_q_d_n4, eq21_e210_q_d_n5, eq21_e210_q_d_n6, eq21_e210_q_d_n7, eq21_e210_q_d_n8, eq21_e210_q_d_n9, eq21_e210_q_d_b0, eq21_e210_q_d_b1, eq21_e210_q_d_b2, eq21_e210_q_d_b3,) = {
    if (var_guard115 == 0.0) {
        let eq21_e208_q: f64 = var_i_cth_rv;
        (var_i_cth, var_i_cth_dn0, var_i_cth_dn1, var_i_cth_dn2, var_i_cth_dn3, var_i_cth_dn4, var_i_cth_dn5, var_i_cth_dn6, var_i_cth_dn7, var_i_cth_dn8, var_i_cth_dn9, var_i_cth_db0, var_i_cth_db1, var_i_cth_db2, var_i_cth_db3, eq21_e208_q, var_i_cth_rdn0, var_i_cth_rdn1, var_i_cth_rdn2, var_i_cth_rdn3, var_i_cth_rdn4, var_i_cth_rdn5, var_i_cth_rdn6, var_i_cth_rdn7, var_i_cth_rdn8, var_i_cth_rdn9, var_i_cth_rdb0, var_i_cth_rdb1, var_i_cth_rdb2, var_i_cth_rdb3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_reactive_node_derivatives: [f64; 10] = [eq21_e210_q_d_n0, eq21_e210_q_d_n1, eq21_e210_q_d_n2, eq21_e210_q_d_n3, eq21_e210_q_d_n4, eq21_e210_q_d_n5, eq21_e210_q_d_n6, eq21_e210_q_d_n7, eq21_e210_q_d_n8, eq21_e210_q_d_n9];
        let eq21_reactive_branch_derivatives: [f64; 4] = [eq21_e210_q_d_b0, eq21_e210_q_d_b1, eq21_e210_q_d_b2, eq21_e210_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
