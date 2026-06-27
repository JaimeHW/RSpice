#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq9_e163,) = {
    if (s.v[356] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e163;
        stamper.stamp_potential_const_local(
            0,
            eq9_value,
        );
        let (eq11_e174,) = {
    if (s.v[357] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e174;
        stamper.stamp_potential_const_local(
            1,
            eq11_value,
        );
        let (eq13_e185,) = {
    if (s.v[358] == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e185;
        stamper.stamp_potential_const_local(
            2,
            eq13_value,
        );
        let (eq19_e196,) = {
    if s.b[360] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e196;
        stamper.stamp_potential_const_local(
            3,
            eq19_value,
        );
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3,) = {
    if (!s.b[360]) {
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e210;
        let eq21_node_derivatives: [f64; 10] = [eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9];
        let eq21_branch_derivatives: [f64; 4] = [eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3, eq21_e210_q, eq21_e210_q_d_n4,) = {
    if (!s.b[360]) {
        let eq21_e208_q: f64 = s.rv[167];
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3], eq21_e208_q, s.rdn[167][4],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
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
