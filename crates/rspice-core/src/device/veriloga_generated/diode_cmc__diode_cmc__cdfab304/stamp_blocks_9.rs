#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_guard571: f64,
        var_guard572: f64,
        var_inqs0_a: f64,
        var_inqs0_a_db0: f64,
        var_inqs0_a_db1: f64,
        var_inqs0_a_db2: f64,
        var_inqs0_a_db3: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn1: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn3: f64,
        var_inqs0_a_dn4: f64,
        var_inqs0_a_dn5: f64,
        var_inqs0_k: f64,
        var_inqs0_k_db0: f64,
        var_inqs0_k_db1: f64,
        var_inqs0_k_db2: f64,
        var_inqs0_k_db3: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn1: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn3: f64,
        var_inqs0_k_dn4: f64,
        var_inqs0_k_dn5: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_db0: f64,
        var_iwnqs0_a_db1: f64,
        var_iwnqs0_a_db2: f64,
        var_iwnqs0_a_db3: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn1: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn3: f64,
        var_iwnqs0_a_dn4: f64,
        var_iwnqs0_a_dn5: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_db0: f64,
        var_q_nqs_a_db1: f64,
        var_q_nqs_a_db2: f64,
        var_q_nqs_a_db3: f64,
        var_q_nqs_a_dn0: f64,
        var_q_nqs_a_dn1: f64,
        var_q_nqs_a_dn2: f64,
        var_q_nqs_a_dn3: f64,
        var_q_nqs_a_dn4: f64,
        var_q_nqs_a_dn5: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_db0: f64,
        var_q_nqs_k_db1: f64,
        var_q_nqs_k_db2: f64,
        var_q_nqs_k_db3: f64,
        var_q_nqs_k_dn0: f64,
        var_q_nqs_k_dn1: f64,
        var_q_nqs_k_dn2: f64,
        var_q_nqs_k_dn3: f64,
        var_q_nqs_k_dn4: f64,
        var_q_nqs_k_dn5: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_db0: f64,
        var_w_nqs_a_db1: f64,
        var_w_nqs_a_db2: f64,
        var_w_nqs_a_db3: f64,
        var_w_nqs_a_dn0: f64,
        var_w_nqs_a_dn1: f64,
        var_w_nqs_a_dn2: f64,
        var_w_nqs_a_dn3: f64,
        var_w_nqs_a_dn4: f64,
        var_w_nqs_a_dn5: f64,
    ) {
        let (eq7_e144, eq7_e144_d_n0, eq7_e144_d_n1, eq7_e144_d_n2, eq7_e144_d_n3, eq7_e144_d_n4, eq7_e144_d_n5, eq7_e144_d_b0, eq7_e144_d_b1, eq7_e144_d_b2, eq7_e144_d_b3, eq7_e144_q, eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5, eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3,) = {
    if (var_guard571 != 0.0) {
        let eq7_e140_q: f64 = var_q_nqs_a;
        let eq7_e141: f64 = (var_inqs0_a + var_q_nqs_a);
        let eq7_e141_d_n0: f64 = (var_inqs0_a_dn0 + var_q_nqs_a_dn0);
        let eq7_e141_d_n1: f64 = (var_inqs0_a_dn1 + var_q_nqs_a_dn1);
        let eq7_e141_d_n2: f64 = (var_inqs0_a_dn2 + var_q_nqs_a_dn2);
        let eq7_e141_d_n3: f64 = (var_inqs0_a_dn3 + var_q_nqs_a_dn3);
        let eq7_e141_d_n4: f64 = (var_inqs0_a_dn4 + var_q_nqs_a_dn4);
        let eq7_e141_d_n5: f64 = (var_inqs0_a_dn5 + var_q_nqs_a_dn5);
        let eq7_e141_d_b0: f64 = (var_inqs0_a_db0 + var_q_nqs_a_db0);
        let eq7_e141_d_b1: f64 = (var_inqs0_a_db1 + var_q_nqs_a_db1);
        let eq7_e141_d_b2: f64 = (var_inqs0_a_db2 + var_q_nqs_a_db2);
        let eq7_e141_d_b3: f64 = (var_inqs0_a_db3 + var_q_nqs_a_db3);
        let eq7_e141_q: f64 = eq7_e140_q;
        let eq7_e142: f64 = (1e-12 * eq7_e141);
        let eq7_e142_d_n0: f64 = (1e-12 * eq7_e141_d_n0);
        let eq7_e142_d_n1: f64 = (1e-12 * eq7_e141_d_n1);
        let eq7_e142_d_n2: f64 = (1e-12 * eq7_e141_d_n2);
        let eq7_e142_d_n3: f64 = (1e-12 * eq7_e141_d_n3);
        let eq7_e142_d_n4: f64 = (1e-12 * eq7_e141_d_n4);
        let eq7_e142_d_n5: f64 = (1e-12 * eq7_e141_d_n5);
        let eq7_e142_d_b0: f64 = (1e-12 * eq7_e141_d_b0);
        let eq7_e142_d_b1: f64 = (1e-12 * eq7_e141_d_b1);
        let eq7_e142_d_b2: f64 = (1e-12 * eq7_e141_d_b2);
        let eq7_e142_d_b3: f64 = (1e-12 * eq7_e141_d_b3);
        let eq7_e142_q: f64 = (1e-12 * eq7_e141_q);
        let eq7_e142_q_d_n0: f64 = (1e-12 * var_q_nqs_a_dn0);
        let eq7_e142_q_d_n1: f64 = (1e-12 * var_q_nqs_a_dn1);
        let eq7_e142_q_d_n2: f64 = (1e-12 * var_q_nqs_a_dn2);
        let eq7_e142_q_d_n3: f64 = (1e-12 * var_q_nqs_a_dn3);
        let eq7_e142_q_d_n4: f64 = (1e-12 * var_q_nqs_a_dn4);
        let eq7_e142_q_d_n5: f64 = (1e-12 * var_q_nqs_a_dn5);
        let eq7_e142_q_d_b0: f64 = (1e-12 * var_q_nqs_a_db0);
        let eq7_e142_q_d_b1: f64 = (1e-12 * var_q_nqs_a_db1);
        let eq7_e142_q_d_b2: f64 = (1e-12 * var_q_nqs_a_db2);
        let eq7_e142_q_d_b3: f64 = (1e-12 * var_q_nqs_a_db3);
        (eq7_e142, eq7_e142_d_n0, eq7_e142_d_n1, eq7_e142_d_n2, eq7_e142_d_n3, eq7_e142_d_n4, eq7_e142_d_n5, eq7_e142_d_b0, eq7_e142_d_b1, eq7_e142_d_b2, eq7_e142_d_b3, eq7_e142_q, eq7_e142_q_d_n0, eq7_e142_q_d_n1, eq7_e142_q_d_n2, eq7_e142_q_d_n3, eq7_e142_q_d_n4, eq7_e142_q_d_n5, eq7_e142_q_d_b0, eq7_e142_q_d_b1, eq7_e142_q_d_b2, eq7_e142_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 6] = [eq7_e144_q_d_n0, eq7_e144_q_d_n1, eq7_e144_q_d_n2, eq7_e144_q_d_n3, eq7_e144_q_d_n4, eq7_e144_q_d_n5];
        let eq7_reactive_branch_derivatives: [f64; 4] = [eq7_e144_q_d_b0, eq7_e144_q_d_b1, eq7_e144_q_d_b2, eq7_e144_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e153, eq8_e153_d_n0, eq8_e153_d_n1, eq8_e153_d_n2, eq8_e153_d_n3, eq8_e153_d_n4, eq8_e153_d_n5, eq8_e153_d_b0, eq8_e153_d_b1, eq8_e153_d_b2, eq8_e153_d_b3, eq8_e153_q, eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5, eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3,) = {
    if (var_guard571 != 0.0) {
        let eq8_e149_q: f64 = var_q_nqs_k;
        let eq8_e150: f64 = (var_inqs0_k + var_q_nqs_k);
        let eq8_e150_d_n0: f64 = (var_inqs0_k_dn0 + var_q_nqs_k_dn0);
        let eq8_e150_d_n1: f64 = (var_inqs0_k_dn1 + var_q_nqs_k_dn1);
        let eq8_e150_d_n2: f64 = (var_inqs0_k_dn2 + var_q_nqs_k_dn2);
        let eq8_e150_d_n3: f64 = (var_inqs0_k_dn3 + var_q_nqs_k_dn3);
        let eq8_e150_d_n4: f64 = (var_inqs0_k_dn4 + var_q_nqs_k_dn4);
        let eq8_e150_d_n5: f64 = (var_inqs0_k_dn5 + var_q_nqs_k_dn5);
        let eq8_e150_d_b0: f64 = (var_inqs0_k_db0 + var_q_nqs_k_db0);
        let eq8_e150_d_b1: f64 = (var_inqs0_k_db1 + var_q_nqs_k_db1);
        let eq8_e150_d_b2: f64 = (var_inqs0_k_db2 + var_q_nqs_k_db2);
        let eq8_e150_d_b3: f64 = (var_inqs0_k_db3 + var_q_nqs_k_db3);
        let eq8_e150_q: f64 = eq8_e149_q;
        let eq8_e151: f64 = (1e-12 * eq8_e150);
        let eq8_e151_d_n0: f64 = (1e-12 * eq8_e150_d_n0);
        let eq8_e151_d_n1: f64 = (1e-12 * eq8_e150_d_n1);
        let eq8_e151_d_n2: f64 = (1e-12 * eq8_e150_d_n2);
        let eq8_e151_d_n3: f64 = (1e-12 * eq8_e150_d_n3);
        let eq8_e151_d_n4: f64 = (1e-12 * eq8_e150_d_n4);
        let eq8_e151_d_n5: f64 = (1e-12 * eq8_e150_d_n5);
        let eq8_e151_d_b0: f64 = (1e-12 * eq8_e150_d_b0);
        let eq8_e151_d_b1: f64 = (1e-12 * eq8_e150_d_b1);
        let eq8_e151_d_b2: f64 = (1e-12 * eq8_e150_d_b2);
        let eq8_e151_d_b3: f64 = (1e-12 * eq8_e150_d_b3);
        let eq8_e151_q: f64 = (1e-12 * eq8_e150_q);
        let eq8_e151_q_d_n0: f64 = (1e-12 * var_q_nqs_k_dn0);
        let eq8_e151_q_d_n1: f64 = (1e-12 * var_q_nqs_k_dn1);
        let eq8_e151_q_d_n2: f64 = (1e-12 * var_q_nqs_k_dn2);
        let eq8_e151_q_d_n3: f64 = (1e-12 * var_q_nqs_k_dn3);
        let eq8_e151_q_d_n4: f64 = (1e-12 * var_q_nqs_k_dn4);
        let eq8_e151_q_d_n5: f64 = (1e-12 * var_q_nqs_k_dn5);
        let eq8_e151_q_d_b0: f64 = (1e-12 * var_q_nqs_k_db0);
        let eq8_e151_q_d_b1: f64 = (1e-12 * var_q_nqs_k_db1);
        let eq8_e151_q_d_b2: f64 = (1e-12 * var_q_nqs_k_db2);
        let eq8_e151_q_d_b3: f64 = (1e-12 * var_q_nqs_k_db3);
        (eq8_e151, eq8_e151_d_n0, eq8_e151_d_n1, eq8_e151_d_n2, eq8_e151_d_n3, eq8_e151_d_n4, eq8_e151_d_n5, eq8_e151_d_b0, eq8_e151_d_b1, eq8_e151_d_b2, eq8_e151_d_b3, eq8_e151_q, eq8_e151_q_d_n0, eq8_e151_q_d_n1, eq8_e151_q_d_n2, eq8_e151_q_d_n3, eq8_e151_q_d_n4, eq8_e151_q_d_n5, eq8_e151_q_d_b0, eq8_e151_q_d_b1, eq8_e151_q_d_b2, eq8_e151_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 6] = [eq8_e153_q_d_n0, eq8_e153_q_d_n1, eq8_e153_q_d_n2, eq8_e153_q_d_n3, eq8_e153_q_d_n4, eq8_e153_q_d_n5];
        let eq8_reactive_branch_derivatives: [f64; 4] = [eq8_e153_q_d_b0, eq8_e153_q_d_b1, eq8_e153_q_d_b2, eq8_e153_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e172, eq11_e172_d_n0, eq11_e172_d_n1, eq11_e172_d_n2, eq11_e172_d_n3, eq11_e172_d_n4, eq11_e172_d_n5, eq11_e172_d_b0, eq11_e172_d_b1, eq11_e172_d_b2, eq11_e172_d_b3, eq11_e172_q, eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5, eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3,) = {
    if (var_guard572 != 0.0) {
        let eq11_e168_q: f64 = var_w_nqs_a;
        let eq11_e169: f64 = (var_iwnqs0_a + var_w_nqs_a);
        let eq11_e169_d_n0: f64 = (var_iwnqs0_a_dn0 + var_w_nqs_a_dn0);
        let eq11_e169_d_n1: f64 = (var_iwnqs0_a_dn1 + var_w_nqs_a_dn1);
        let eq11_e169_d_n2: f64 = (var_iwnqs0_a_dn2 + var_w_nqs_a_dn2);
        let eq11_e169_d_n3: f64 = (var_iwnqs0_a_dn3 + var_w_nqs_a_dn3);
        let eq11_e169_d_n4: f64 = (var_iwnqs0_a_dn4 + var_w_nqs_a_dn4);
        let eq11_e169_d_n5: f64 = (var_iwnqs0_a_dn5 + var_w_nqs_a_dn5);
        let eq11_e169_d_b0: f64 = (var_iwnqs0_a_db0 + var_w_nqs_a_db0);
        let eq11_e169_d_b1: f64 = (var_iwnqs0_a_db1 + var_w_nqs_a_db1);
        let eq11_e169_d_b2: f64 = (var_iwnqs0_a_db2 + var_w_nqs_a_db2);
        let eq11_e169_d_b3: f64 = (var_iwnqs0_a_db3 + var_w_nqs_a_db3);
        let eq11_e169_q: f64 = eq11_e168_q;
        let eq11_e170: f64 = (1e-13 * eq11_e169);
        let eq11_e170_d_n0: f64 = (1e-13 * eq11_e169_d_n0);
        let eq11_e170_d_n1: f64 = (1e-13 * eq11_e169_d_n1);
        let eq11_e170_d_n2: f64 = (1e-13 * eq11_e169_d_n2);
        let eq11_e170_d_n3: f64 = (1e-13 * eq11_e169_d_n3);
        let eq11_e170_d_n4: f64 = (1e-13 * eq11_e169_d_n4);
        let eq11_e170_d_n5: f64 = (1e-13 * eq11_e169_d_n5);
        let eq11_e170_d_b0: f64 = (1e-13 * eq11_e169_d_b0);
        let eq11_e170_d_b1: f64 = (1e-13 * eq11_e169_d_b1);
        let eq11_e170_d_b2: f64 = (1e-13 * eq11_e169_d_b2);
        let eq11_e170_d_b3: f64 = (1e-13 * eq11_e169_d_b3);
        let eq11_e170_q: f64 = (1e-13 * eq11_e169_q);
        let eq11_e170_q_d_n0: f64 = (1e-13 * var_w_nqs_a_dn0);
        let eq11_e170_q_d_n1: f64 = (1e-13 * var_w_nqs_a_dn1);
        let eq11_e170_q_d_n2: f64 = (1e-13 * var_w_nqs_a_dn2);
        let eq11_e170_q_d_n3: f64 = (1e-13 * var_w_nqs_a_dn3);
        let eq11_e170_q_d_n4: f64 = (1e-13 * var_w_nqs_a_dn4);
        let eq11_e170_q_d_n5: f64 = (1e-13 * var_w_nqs_a_dn5);
        let eq11_e170_q_d_b0: f64 = (1e-13 * var_w_nqs_a_db0);
        let eq11_e170_q_d_b1: f64 = (1e-13 * var_w_nqs_a_db1);
        let eq11_e170_q_d_b2: f64 = (1e-13 * var_w_nqs_a_db2);
        let eq11_e170_q_d_b3: f64 = (1e-13 * var_w_nqs_a_db3);
        (eq11_e170, eq11_e170_d_n0, eq11_e170_d_n1, eq11_e170_d_n2, eq11_e170_d_n3, eq11_e170_d_n4, eq11_e170_d_n5, eq11_e170_d_b0, eq11_e170_d_b1, eq11_e170_d_b2, eq11_e170_d_b3, eq11_e170_q, eq11_e170_q_d_n0, eq11_e170_q_d_n1, eq11_e170_q_d_n2, eq11_e170_q_d_n3, eq11_e170_q_d_n4, eq11_e170_q_d_n5, eq11_e170_q_d_b0, eq11_e170_q_d_b1, eq11_e170_q_d_b2, eq11_e170_q_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 6] = [eq11_e172_q_d_n0, eq11_e172_q_d_n1, eq11_e172_q_d_n2, eq11_e172_q_d_n3, eq11_e172_q_d_n4, eq11_e172_q_d_n5];
        let eq11_reactive_branch_derivatives: [f64; 4] = [eq11_e172_q_d_b0, eq11_e172_q_d_b1, eq11_e172_q_d_b2, eq11_e172_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
