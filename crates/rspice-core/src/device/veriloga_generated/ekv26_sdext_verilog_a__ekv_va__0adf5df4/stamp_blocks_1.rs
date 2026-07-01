#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_ddt_qd: f64,
        var_ddt_qd_dn0: f64,
        var_ddt_qd_dn1: f64,
        var_ddt_qd_dn2: f64,
        var_ddt_qd_dn3: f64,
        var_ddt_qd_rdn0: f64,
        var_ddt_qd_rdn1: f64,
        var_ddt_qd_rdn2: f64,
        var_ddt_qd_rdn3: f64,
        var_ddt_qd_rv: f64,
        var_ddt_qs: f64,
        var_ddt_qs_dn0: f64,
        var_ddt_qs_dn1: f64,
        var_ddt_qs_dn2: f64,
        var_ddt_qs_dn3: f64,
        var_ddt_qs_rdn0: f64,
        var_ddt_qs_rdn1: f64,
        var_ddt_qs_rdn2: f64,
        var_ddt_qs_rdn3: f64,
        var_ddt_qs_rv: f64,
        var_guard21: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn1: f64,
        var_qg_dn2: f64,
        var_qg_dn3: f64,
        var_qjd: f64,
        var_qjd_dn0: f64,
        var_qjd_dn1: f64,
        var_qjd_dn2: f64,
        var_qjd_dn3: f64,
        var_qjs: f64,
        var_qjs_dn0: f64,
        var_qjs_dn1: f64,
        var_qjs_dn2: f64,
        var_qjs_dn3: f64,
    ) {
        let (eq1_e92, eq1_e92_d_n0, eq1_e92_d_n1, eq1_e92_d_n2, eq1_e92_d_n3, eq1_e92_q, eq1_e92_q_d_n0, eq1_e92_q_d_n1, eq1_e92_q_d_n2, eq1_e92_q_d_n3,) = {
    if (var_guard21 != 0.0) {
        let eq1_e89_q: f64 = var_ddt_qd_rv;
        let eq1_e90: f64 = (p.p0 * var_ddt_qd);
        let eq1_e90_d_n0: f64 = (p.p0 * var_ddt_qd_dn0);
        let eq1_e90_d_n1: f64 = (p.p0 * var_ddt_qd_dn1);
        let eq1_e90_d_n2: f64 = (p.p0 * var_ddt_qd_dn2);
        let eq1_e90_d_n3: f64 = (p.p0 * var_ddt_qd_dn3);
        let eq1_e90_q: f64 = (p.p0 * eq1_e89_q);
        let eq1_e90_q_d_n0: f64 = (p.p0 * var_ddt_qd_rdn0);
        let eq1_e90_q_d_n1: f64 = (p.p0 * var_ddt_qd_rdn1);
        let eq1_e90_q_d_n2: f64 = (p.p0 * var_ddt_qd_rdn2);
        let eq1_e90_q_d_n3: f64 = (p.p0 * var_ddt_qd_rdn3);
        (eq1_e90, eq1_e90_d_n0, eq1_e90_d_n1, eq1_e90_d_n2, eq1_e90_d_n3, eq1_e90_q, eq1_e90_q_d_n0, eq1_e90_q_d_n1, eq1_e90_q_d_n2, eq1_e90_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq1_e92_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq1_e92_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq1_e92_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq1_e92_q_d_n3)),
            ],
        );
        let (eq2_e98, eq2_e98_d_n0, eq2_e98_d_n1, eq2_e98_d_n2, eq2_e98_d_n3, eq2_e98_q, eq2_e98_q_d_n0, eq2_e98_q_d_n1, eq2_e98_q_d_n2, eq2_e98_q_d_n3,) = {
    if (var_guard21 != 0.0) {
        let eq2_e95_q: f64 = var_ddt_qs_rv;
        let eq2_e96: f64 = (p.p0 * var_ddt_qs);
        let eq2_e96_d_n0: f64 = (p.p0 * var_ddt_qs_dn0);
        let eq2_e96_d_n1: f64 = (p.p0 * var_ddt_qs_dn1);
        let eq2_e96_d_n2: f64 = (p.p0 * var_ddt_qs_dn2);
        let eq2_e96_d_n3: f64 = (p.p0 * var_ddt_qs_dn3);
        let eq2_e96_q: f64 = (p.p0 * eq2_e95_q);
        let eq2_e96_q_d_n0: f64 = (p.p0 * var_ddt_qs_rdn0);
        let eq2_e96_q_d_n1: f64 = (p.p0 * var_ddt_qs_rdn1);
        let eq2_e96_q_d_n2: f64 = (p.p0 * var_ddt_qs_rdn2);
        let eq2_e96_q_d_n3: f64 = (p.p0 * var_ddt_qs_rdn3);
        (eq2_e96, eq2_e96_d_n0, eq2_e96_d_n1, eq2_e96_d_n2, eq2_e96_d_n3, eq2_e96_q, eq2_e96_q_d_n0, eq2_e96_q_d_n1, eq2_e96_q_d_n2, eq2_e96_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq2_e98_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq2_e98_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq2_e98_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq2_e98_q_d_n3)),
            ],
        );
        let (eq4_e111, eq4_e111_d_n0, eq4_e111_d_n1, eq4_e111_d_n2, eq4_e111_d_n3, eq4_e111_q, eq4_e111_q_d_n0, eq4_e111_q_d_n1, eq4_e111_q_d_n2, eq4_e111_q_d_n3,) = {
    if (var_guard21 == 0.0) {
        let eq4_e108_q: f64 = var_ddt_qd_rv;
        let eq4_e109: f64 = (p.p0 * var_ddt_qd);
        let eq4_e109_d_n0: f64 = (p.p0 * var_ddt_qd_dn0);
        let eq4_e109_d_n1: f64 = (p.p0 * var_ddt_qd_dn1);
        let eq4_e109_d_n2: f64 = (p.p0 * var_ddt_qd_dn2);
        let eq4_e109_d_n3: f64 = (p.p0 * var_ddt_qd_dn3);
        let eq4_e109_q: f64 = (p.p0 * eq4_e108_q);
        let eq4_e109_q_d_n0: f64 = (p.p0 * var_ddt_qd_rdn0);
        let eq4_e109_q_d_n1: f64 = (p.p0 * var_ddt_qd_rdn1);
        let eq4_e109_q_d_n2: f64 = (p.p0 * var_ddt_qd_rdn2);
        let eq4_e109_q_d_n3: f64 = (p.p0 * var_ddt_qd_rdn3);
        (eq4_e109, eq4_e109_d_n0, eq4_e109_d_n1, eq4_e109_d_n2, eq4_e109_d_n3, eq4_e109_q, eq4_e109_q_d_n0, eq4_e109_q_d_n1, eq4_e109_q_d_n2, eq4_e109_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq4_e111_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq4_e111_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq4_e111_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq4_e111_q_d_n3)),
            ],
        );
        let (eq5_e118, eq5_e118_d_n0, eq5_e118_d_n1, eq5_e118_d_n2, eq5_e118_d_n3, eq5_e118_q, eq5_e118_q_d_n0, eq5_e118_q_d_n1, eq5_e118_q_d_n2, eq5_e118_q_d_n3,) = {
    if (var_guard21 == 0.0) {
        let eq5_e115_q: f64 = var_ddt_qs_rv;
        let eq5_e116: f64 = (p.p0 * var_ddt_qs);
        let eq5_e116_d_n0: f64 = (p.p0 * var_ddt_qs_dn0);
        let eq5_e116_d_n1: f64 = (p.p0 * var_ddt_qs_dn1);
        let eq5_e116_d_n2: f64 = (p.p0 * var_ddt_qs_dn2);
        let eq5_e116_d_n3: f64 = (p.p0 * var_ddt_qs_dn3);
        let eq5_e116_q: f64 = (p.p0 * eq5_e115_q);
        let eq5_e116_q_d_n0: f64 = (p.p0 * var_ddt_qs_rdn0);
        let eq5_e116_q_d_n1: f64 = (p.p0 * var_ddt_qs_rdn1);
        let eq5_e116_q_d_n2: f64 = (p.p0 * var_ddt_qs_rdn2);
        let eq5_e116_q_d_n3: f64 = (p.p0 * var_ddt_qs_rdn3);
        (eq5_e116, eq5_e116_d_n0, eq5_e116_d_n1, eq5_e116_d_n2, eq5_e116_d_n3, eq5_e116_q, eq5_e116_q_d_n0, eq5_e116_q_d_n1, eq5_e116_q_d_n2, eq5_e116_q_d_n3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq5_e118_q_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq5_e118_q_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq5_e118_q_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq5_e118_q_d_n3)),
            ],
        );
        let eq7_e128_q: f64 = var_qg;
        let eq7_e129: f64 = (p.p0 * var_qg);
        let eq7_e129_d_n0: f64 = (p.p0 * var_qg_dn0);
        let eq7_e129_d_n1: f64 = (p.p0 * var_qg_dn1);
        let eq7_e129_d_n2: f64 = (p.p0 * var_qg_dn2);
        let eq7_e129_d_n3: f64 = (p.p0 * var_qg_dn3);
        let eq7_e129_q: f64 = (p.p0 * eq7_e128_q);
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq7_e129_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq7_e129_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq7_e129_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq7_e129_d_n3)),
            ],
        );
        let eq11_e178_q: f64 = var_qjd;
        let eq11_e180: f64 = (var_qjd * p.p0);
        let eq11_e180_d_n0: f64 = (var_qjd_dn0 * p.p0);
        let eq11_e180_d_n1: f64 = (var_qjd_dn1 * p.p0);
        let eq11_e180_d_n2: f64 = (var_qjd_dn2 * p.p0);
        let eq11_e180_d_n3: f64 = (var_qjd_dn3 * p.p0);
        let eq11_e180_q: f64 = (eq11_e178_q * p.p0);
        let eq11_e182: f64 = (eq11_e180 * p.p7);
        let eq11_e182_d_n0: f64 = (eq11_e180_d_n0 * p.p7);
        let eq11_e182_d_n1: f64 = (eq11_e180_d_n1 * p.p7);
        let eq11_e182_d_n2: f64 = (eq11_e180_d_n2 * p.p7);
        let eq11_e182_d_n3: f64 = (eq11_e180_d_n3 * p.p7);
        let eq11_e182_q: f64 = (eq11_e180_q * p.p7);
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq11_e182_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq11_e182_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq11_e182_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq11_e182_d_n3)),
            ],
        );
        let eq12_e184_q: f64 = var_qjs;
        let eq12_e186: f64 = (var_qjs * p.p0);
        let eq12_e186_d_n0: f64 = (var_qjs_dn0 * p.p0);
        let eq12_e186_d_n1: f64 = (var_qjs_dn1 * p.p0);
        let eq12_e186_d_n2: f64 = (var_qjs_dn2 * p.p0);
        let eq12_e186_d_n3: f64 = (var_qjs_dn3 * p.p0);
        let eq12_e186_q: f64 = (eq12_e184_q * p.p0);
        let eq12_e188: f64 = (eq12_e186 * p.p7);
        let eq12_e188_d_n0: f64 = (eq12_e186_d_n0 * p.p7);
        let eq12_e188_d_n1: f64 = (eq12_e186_d_n1 * p.p7);
        let eq12_e188_d_n2: f64 = (eq12_e186_d_n2 * p.p7);
        let eq12_e188_d_n3: f64 = (eq12_e186_d_n3 * p.p7);
        let eq12_e188_q: f64 = (eq12_e186_q * p.p7);
        stamper.stamp_current_reactive(
            Some(nodes[2]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[0], multiplicity * (eq12_e188_d_n0)),
                GeneratedDerivative::node(nodes[1], multiplicity * (eq12_e188_d_n1)),
                GeneratedDerivative::node(nodes[2], multiplicity * (eq12_e188_d_n2)),
                GeneratedDerivative::node(nodes[3], multiplicity * (eq12_e188_d_n3)),
            ],
        );
    }
}
