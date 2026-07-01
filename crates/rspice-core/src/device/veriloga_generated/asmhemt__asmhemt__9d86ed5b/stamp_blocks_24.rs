#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_9(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq196_e2466, eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n12, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22, eq196_e2466_q,) = {
    if (((locals.var_guard566 != 0.0) && (locals.var_guard567 != 0.0)) && (locals.var_guard568 == 0.0)) {
        let eq196_e2461_q: f64 = locals.var_qg_fp4;
        let eq196_e2462: f64 = (p.p7 * locals.var_qg_fp4);
        let eq196_e2462_d_n0: f64 = (p.p7 * locals.var_qg_fp4_dn0);
        let eq196_e2462_d_n1: f64 = (p.p7 * locals.var_qg_fp4_dn1);
        let eq196_e2462_d_n2: f64 = (p.p7 * locals.var_qg_fp4_dn2);
        let eq196_e2462_d_n3: f64 = (p.p7 * locals.var_qg_fp4_dn3);
        let eq196_e2462_d_n4: f64 = (p.p7 * locals.var_qg_fp4_dn4);
        let eq196_e2462_d_n5: f64 = (p.p7 * locals.var_qg_fp4_dn5);
        let eq196_e2462_d_n6: f64 = (p.p7 * locals.var_qg_fp4_dn6);
        let eq196_e2462_d_n7: f64 = (p.p7 * locals.var_qg_fp4_dn7);
        let eq196_e2462_d_n8: f64 = (p.p7 * locals.var_qg_fp4_dn8);
        let eq196_e2462_d_n9: f64 = (p.p7 * locals.var_qg_fp4_dn9);
        let eq196_e2462_d_n12: f64 = (p.p7 * locals.var_qg_fp4_dn12);
        let eq196_e2462_d_n14: f64 = (p.p7 * locals.var_qg_fp4_dn14);
        let eq196_e2462_d_n15: f64 = (p.p7 * locals.var_qg_fp4_dn15);
        let eq196_e2462_d_n16: f64 = (p.p7 * locals.var_qg_fp4_dn16);
        let eq196_e2462_d_n17: f64 = (p.p7 * locals.var_qg_fp4_dn17);
        let eq196_e2462_d_n18: f64 = (p.p7 * locals.var_qg_fp4_dn18);
        let eq196_e2462_d_n19: f64 = (p.p7 * locals.var_qg_fp4_dn19);
        let eq196_e2462_d_n20: f64 = (p.p7 * locals.var_qg_fp4_dn20);
        let eq196_e2462_d_n21: f64 = (p.p7 * locals.var_qg_fp4_dn21);
        let eq196_e2462_d_n22: f64 = (p.p7 * locals.var_qg_fp4_dn22);
        let eq196_e2462_q: f64 = (p.p7 * eq196_e2461_q);
        let eq196_e2464: f64 = (eq196_e2462 * p.p249);
        let eq196_e2464_d_n0: f64 = (eq196_e2462_d_n0 * p.p249);
        let eq196_e2464_d_n1: f64 = (eq196_e2462_d_n1 * p.p249);
        let eq196_e2464_d_n2: f64 = (eq196_e2462_d_n2 * p.p249);
        let eq196_e2464_d_n3: f64 = (eq196_e2462_d_n3 * p.p249);
        let eq196_e2464_d_n4: f64 = (eq196_e2462_d_n4 * p.p249);
        let eq196_e2464_d_n5: f64 = (eq196_e2462_d_n5 * p.p249);
        let eq196_e2464_d_n6: f64 = (eq196_e2462_d_n6 * p.p249);
        let eq196_e2464_d_n7: f64 = (eq196_e2462_d_n7 * p.p249);
        let eq196_e2464_d_n8: f64 = (eq196_e2462_d_n8 * p.p249);
        let eq196_e2464_d_n9: f64 = (eq196_e2462_d_n9 * p.p249);
        let eq196_e2464_d_n12: f64 = (eq196_e2462_d_n12 * p.p249);
        let eq196_e2464_d_n14: f64 = (eq196_e2462_d_n14 * p.p249);
        let eq196_e2464_d_n15: f64 = (eq196_e2462_d_n15 * p.p249);
        let eq196_e2464_d_n16: f64 = (eq196_e2462_d_n16 * p.p249);
        let eq196_e2464_d_n17: f64 = (eq196_e2462_d_n17 * p.p249);
        let eq196_e2464_d_n18: f64 = (eq196_e2462_d_n18 * p.p249);
        let eq196_e2464_d_n19: f64 = (eq196_e2462_d_n19 * p.p249);
        let eq196_e2464_d_n20: f64 = (eq196_e2462_d_n20 * p.p249);
        let eq196_e2464_d_n21: f64 = (eq196_e2462_d_n21 * p.p249);
        let eq196_e2464_d_n22: f64 = (eq196_e2462_d_n22 * p.p249);
        let eq196_e2464_q: f64 = (eq196_e2462_q * p.p249);
        (eq196_e2464, eq196_e2464_d_n0, eq196_e2464_d_n1, eq196_e2464_d_n2, eq196_e2464_d_n3, eq196_e2464_d_n4, eq196_e2464_d_n5, eq196_e2464_d_n6, eq196_e2464_d_n7, eq196_e2464_d_n8, eq196_e2464_d_n9, eq196_e2464_d_n12, eq196_e2464_d_n14, eq196_e2464_d_n15, eq196_e2464_d_n16, eq196_e2464_d_n17, eq196_e2464_d_n18, eq196_e2464_d_n19, eq196_e2464_d_n20, eq196_e2464_d_n21, eq196_e2464_d_n22, eq196_e2464_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq196_reactive_node_derivatives: [f64; 23] = [eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, 0.0, 0.0, eq196_e2466_d_n12, 0.0, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22];
        let eq196_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            nodes,
            &eq196_reactive_node_derivatives,
            branches,
            &eq196_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq197_e2477, eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n12, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22, eq197_e2477_q,) = {
    if ((locals.var_guard566 != 0.0) && (locals.var_guard567 != 0.0)) {
        let eq197_e2473: f64 = (p.p254 * locals.var_qg_fp4);
        let eq197_e2473_d_n0: f64 = (p.p254 * locals.var_qg_fp4_dn0);
        let eq197_e2473_d_n1: f64 = (p.p254 * locals.var_qg_fp4_dn1);
        let eq197_e2473_d_n2: f64 = (p.p254 * locals.var_qg_fp4_dn2);
        let eq197_e2473_d_n3: f64 = (p.p254 * locals.var_qg_fp4_dn3);
        let eq197_e2473_d_n4: f64 = (p.p254 * locals.var_qg_fp4_dn4);
        let eq197_e2473_d_n5: f64 = (p.p254 * locals.var_qg_fp4_dn5);
        let eq197_e2473_d_n6: f64 = (p.p254 * locals.var_qg_fp4_dn6);
        let eq197_e2473_d_n7: f64 = (p.p254 * locals.var_qg_fp4_dn7);
        let eq197_e2473_d_n8: f64 = (p.p254 * locals.var_qg_fp4_dn8);
        let eq197_e2473_d_n9: f64 = (p.p254 * locals.var_qg_fp4_dn9);
        let eq197_e2473_d_n12: f64 = (p.p254 * locals.var_qg_fp4_dn12);
        let eq197_e2473_d_n14: f64 = (p.p254 * locals.var_qg_fp4_dn14);
        let eq197_e2473_d_n15: f64 = (p.p254 * locals.var_qg_fp4_dn15);
        let eq197_e2473_d_n16: f64 = (p.p254 * locals.var_qg_fp4_dn16);
        let eq197_e2473_d_n17: f64 = (p.p254 * locals.var_qg_fp4_dn17);
        let eq197_e2473_d_n18: f64 = (p.p254 * locals.var_qg_fp4_dn18);
        let eq197_e2473_d_n19: f64 = (p.p254 * locals.var_qg_fp4_dn19);
        let eq197_e2473_d_n20: f64 = (p.p254 * locals.var_qg_fp4_dn20);
        let eq197_e2473_d_n21: f64 = (p.p254 * locals.var_qg_fp4_dn21);
        let eq197_e2473_d_n22: f64 = (p.p254 * locals.var_qg_fp4_dn22);
        let eq197_e2474_q: f64 = eq197_e2473;
        let eq197_e2475: f64 = (p.p7 * eq197_e2473);
        let eq197_e2475_d_n0: f64 = (p.p7 * eq197_e2473_d_n0);
        let eq197_e2475_d_n1: f64 = (p.p7 * eq197_e2473_d_n1);
        let eq197_e2475_d_n2: f64 = (p.p7 * eq197_e2473_d_n2);
        let eq197_e2475_d_n3: f64 = (p.p7 * eq197_e2473_d_n3);
        let eq197_e2475_d_n4: f64 = (p.p7 * eq197_e2473_d_n4);
        let eq197_e2475_d_n5: f64 = (p.p7 * eq197_e2473_d_n5);
        let eq197_e2475_d_n6: f64 = (p.p7 * eq197_e2473_d_n6);
        let eq197_e2475_d_n7: f64 = (p.p7 * eq197_e2473_d_n7);
        let eq197_e2475_d_n8: f64 = (p.p7 * eq197_e2473_d_n8);
        let eq197_e2475_d_n9: f64 = (p.p7 * eq197_e2473_d_n9);
        let eq197_e2475_d_n12: f64 = (p.p7 * eq197_e2473_d_n12);
        let eq197_e2475_d_n14: f64 = (p.p7 * eq197_e2473_d_n14);
        let eq197_e2475_d_n15: f64 = (p.p7 * eq197_e2473_d_n15);
        let eq197_e2475_d_n16: f64 = (p.p7 * eq197_e2473_d_n16);
        let eq197_e2475_d_n17: f64 = (p.p7 * eq197_e2473_d_n17);
        let eq197_e2475_d_n18: f64 = (p.p7 * eq197_e2473_d_n18);
        let eq197_e2475_d_n19: f64 = (p.p7 * eq197_e2473_d_n19);
        let eq197_e2475_d_n20: f64 = (p.p7 * eq197_e2473_d_n20);
        let eq197_e2475_d_n21: f64 = (p.p7 * eq197_e2473_d_n21);
        let eq197_e2475_d_n22: f64 = (p.p7 * eq197_e2473_d_n22);
        let eq197_e2475_q: f64 = (p.p7 * eq197_e2474_q);
        (eq197_e2475, eq197_e2475_d_n0, eq197_e2475_d_n1, eq197_e2475_d_n2, eq197_e2475_d_n3, eq197_e2475_d_n4, eq197_e2475_d_n5, eq197_e2475_d_n6, eq197_e2475_d_n7, eq197_e2475_d_n8, eq197_e2475_d_n9, eq197_e2475_d_n12, eq197_e2475_d_n14, eq197_e2475_d_n15, eq197_e2475_d_n16, eq197_e2475_d_n17, eq197_e2475_d_n18, eq197_e2475_d_n19, eq197_e2475_d_n20, eq197_e2475_d_n21, eq197_e2475_d_n22, eq197_e2475_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq197_reactive_node_derivatives: [f64; 23] = [eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, 0.0, 0.0, eq197_e2477_d_n12, 0.0, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22];
        let eq197_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[17]),
            nodes,
            &eq197_reactive_node_derivatives,
            branches,
            &eq197_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq198_e2487, eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n12, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22, eq198_e2487_q,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard569 != 0.0)) {
        let eq198_e2484_q: f64 = locals.var_qd_fp4;
        let eq198_e2485: f64 = (p.p7 * locals.var_qd_fp4);
        let eq198_e2485_d_n0: f64 = (p.p7 * locals.var_qd_fp4_dn0);
        let eq198_e2485_d_n1: f64 = (p.p7 * locals.var_qd_fp4_dn1);
        let eq198_e2485_d_n2: f64 = (p.p7 * locals.var_qd_fp4_dn2);
        let eq198_e2485_d_n3: f64 = (p.p7 * locals.var_qd_fp4_dn3);
        let eq198_e2485_d_n4: f64 = (p.p7 * locals.var_qd_fp4_dn4);
        let eq198_e2485_d_n5: f64 = (p.p7 * locals.var_qd_fp4_dn5);
        let eq198_e2485_d_n6: f64 = (p.p7 * locals.var_qd_fp4_dn6);
        let eq198_e2485_d_n7: f64 = (p.p7 * locals.var_qd_fp4_dn7);
        let eq198_e2485_d_n8: f64 = (p.p7 * locals.var_qd_fp4_dn8);
        let eq198_e2485_d_n9: f64 = (p.p7 * locals.var_qd_fp4_dn9);
        let eq198_e2485_d_n12: f64 = (p.p7 * locals.var_qd_fp4_dn12);
        let eq198_e2485_d_n14: f64 = (p.p7 * locals.var_qd_fp4_dn14);
        let eq198_e2485_d_n15: f64 = (p.p7 * locals.var_qd_fp4_dn15);
        let eq198_e2485_d_n16: f64 = (p.p7 * locals.var_qd_fp4_dn16);
        let eq198_e2485_d_n17: f64 = (p.p7 * locals.var_qd_fp4_dn17);
        let eq198_e2485_d_n18: f64 = (p.p7 * locals.var_qd_fp4_dn18);
        let eq198_e2485_d_n19: f64 = (p.p7 * locals.var_qd_fp4_dn19);
        let eq198_e2485_d_n20: f64 = (p.p7 * locals.var_qd_fp4_dn20);
        let eq198_e2485_d_n21: f64 = (p.p7 * locals.var_qd_fp4_dn21);
        let eq198_e2485_d_n22: f64 = (p.p7 * locals.var_qd_fp4_dn22);
        let eq198_e2485_q: f64 = (p.p7 * eq198_e2484_q);
        (eq198_e2485, eq198_e2485_d_n0, eq198_e2485_d_n1, eq198_e2485_d_n2, eq198_e2485_d_n3, eq198_e2485_d_n4, eq198_e2485_d_n5, eq198_e2485_d_n6, eq198_e2485_d_n7, eq198_e2485_d_n8, eq198_e2485_d_n9, eq198_e2485_d_n12, eq198_e2485_d_n14, eq198_e2485_d_n15, eq198_e2485_d_n16, eq198_e2485_d_n17, eq198_e2485_d_n18, eq198_e2485_d_n19, eq198_e2485_d_n20, eq198_e2485_d_n21, eq198_e2485_d_n22, eq198_e2485_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq198_reactive_node_derivatives: [f64; 23] = [eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, 0.0, 0.0, eq198_e2487_d_n12, 0.0, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22];
        let eq198_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq198_reactive_node_derivatives,
            branches,
            &eq198_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq199_e2499, eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n12, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22, eq199_e2499_q,) = {
    if (((locals.var_guard566 == 0.0) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let eq199_e2496_q: f64 = locals.var_qg_fp4;
        let eq199_e2497: f64 = (p.p7 * locals.var_qg_fp4);
        let eq199_e2497_d_n0: f64 = (p.p7 * locals.var_qg_fp4_dn0);
        let eq199_e2497_d_n1: f64 = (p.p7 * locals.var_qg_fp4_dn1);
        let eq199_e2497_d_n2: f64 = (p.p7 * locals.var_qg_fp4_dn2);
        let eq199_e2497_d_n3: f64 = (p.p7 * locals.var_qg_fp4_dn3);
        let eq199_e2497_d_n4: f64 = (p.p7 * locals.var_qg_fp4_dn4);
        let eq199_e2497_d_n5: f64 = (p.p7 * locals.var_qg_fp4_dn5);
        let eq199_e2497_d_n6: f64 = (p.p7 * locals.var_qg_fp4_dn6);
        let eq199_e2497_d_n7: f64 = (p.p7 * locals.var_qg_fp4_dn7);
        let eq199_e2497_d_n8: f64 = (p.p7 * locals.var_qg_fp4_dn8);
        let eq199_e2497_d_n9: f64 = (p.p7 * locals.var_qg_fp4_dn9);
        let eq199_e2497_d_n12: f64 = (p.p7 * locals.var_qg_fp4_dn12);
        let eq199_e2497_d_n14: f64 = (p.p7 * locals.var_qg_fp4_dn14);
        let eq199_e2497_d_n15: f64 = (p.p7 * locals.var_qg_fp4_dn15);
        let eq199_e2497_d_n16: f64 = (p.p7 * locals.var_qg_fp4_dn16);
        let eq199_e2497_d_n17: f64 = (p.p7 * locals.var_qg_fp4_dn17);
        let eq199_e2497_d_n18: f64 = (p.p7 * locals.var_qg_fp4_dn18);
        let eq199_e2497_d_n19: f64 = (p.p7 * locals.var_qg_fp4_dn19);
        let eq199_e2497_d_n20: f64 = (p.p7 * locals.var_qg_fp4_dn20);
        let eq199_e2497_d_n21: f64 = (p.p7 * locals.var_qg_fp4_dn21);
        let eq199_e2497_d_n22: f64 = (p.p7 * locals.var_qg_fp4_dn22);
        let eq199_e2497_q: f64 = (p.p7 * eq199_e2496_q);
        (eq199_e2497, eq199_e2497_d_n0, eq199_e2497_d_n1, eq199_e2497_d_n2, eq199_e2497_d_n3, eq199_e2497_d_n4, eq199_e2497_d_n5, eq199_e2497_d_n6, eq199_e2497_d_n7, eq199_e2497_d_n8, eq199_e2497_d_n9, eq199_e2497_d_n12, eq199_e2497_d_n14, eq199_e2497_d_n15, eq199_e2497_d_n16, eq199_e2497_d_n17, eq199_e2497_d_n18, eq199_e2497_d_n19, eq199_e2497_d_n20, eq199_e2497_d_n21, eq199_e2497_d_n22, eq199_e2497_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq199_reactive_node_derivatives: [f64; 23] = [eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, 0.0, 0.0, eq199_e2499_d_n12, 0.0, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22];
        let eq199_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq199_reactive_node_derivatives,
            branches,
            &eq199_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq200_e2513, eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n12, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22, eq200_e2513_q,) = {
    if (((locals.var_guard566 == 0.0) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 != 0.0)) {
        let eq200_e2508_q: f64 = locals.var_qg_fp4;
        let eq200_e2509: f64 = (p.p7 * locals.var_qg_fp4);
        let eq200_e2509_d_n0: f64 = (p.p7 * locals.var_qg_fp4_dn0);
        let eq200_e2509_d_n1: f64 = (p.p7 * locals.var_qg_fp4_dn1);
        let eq200_e2509_d_n2: f64 = (p.p7 * locals.var_qg_fp4_dn2);
        let eq200_e2509_d_n3: f64 = (p.p7 * locals.var_qg_fp4_dn3);
        let eq200_e2509_d_n4: f64 = (p.p7 * locals.var_qg_fp4_dn4);
        let eq200_e2509_d_n5: f64 = (p.p7 * locals.var_qg_fp4_dn5);
        let eq200_e2509_d_n6: f64 = (p.p7 * locals.var_qg_fp4_dn6);
        let eq200_e2509_d_n7: f64 = (p.p7 * locals.var_qg_fp4_dn7);
        let eq200_e2509_d_n8: f64 = (p.p7 * locals.var_qg_fp4_dn8);
        let eq200_e2509_d_n9: f64 = (p.p7 * locals.var_qg_fp4_dn9);
        let eq200_e2509_d_n12: f64 = (p.p7 * locals.var_qg_fp4_dn12);
        let eq200_e2509_d_n14: f64 = (p.p7 * locals.var_qg_fp4_dn14);
        let eq200_e2509_d_n15: f64 = (p.p7 * locals.var_qg_fp4_dn15);
        let eq200_e2509_d_n16: f64 = (p.p7 * locals.var_qg_fp4_dn16);
        let eq200_e2509_d_n17: f64 = (p.p7 * locals.var_qg_fp4_dn17);
        let eq200_e2509_d_n18: f64 = (p.p7 * locals.var_qg_fp4_dn18);
        let eq200_e2509_d_n19: f64 = (p.p7 * locals.var_qg_fp4_dn19);
        let eq200_e2509_d_n20: f64 = (p.p7 * locals.var_qg_fp4_dn20);
        let eq200_e2509_d_n21: f64 = (p.p7 * locals.var_qg_fp4_dn21);
        let eq200_e2509_d_n22: f64 = (p.p7 * locals.var_qg_fp4_dn22);
        let eq200_e2509_q: f64 = (p.p7 * eq200_e2508_q);
        let eq200_e2511: f64 = (eq200_e2509 * p.p249);
        let eq200_e2511_d_n0: f64 = (eq200_e2509_d_n0 * p.p249);
        let eq200_e2511_d_n1: f64 = (eq200_e2509_d_n1 * p.p249);
        let eq200_e2511_d_n2: f64 = (eq200_e2509_d_n2 * p.p249);
        let eq200_e2511_d_n3: f64 = (eq200_e2509_d_n3 * p.p249);
        let eq200_e2511_d_n4: f64 = (eq200_e2509_d_n4 * p.p249);
        let eq200_e2511_d_n5: f64 = (eq200_e2509_d_n5 * p.p249);
        let eq200_e2511_d_n6: f64 = (eq200_e2509_d_n6 * p.p249);
        let eq200_e2511_d_n7: f64 = (eq200_e2509_d_n7 * p.p249);
        let eq200_e2511_d_n8: f64 = (eq200_e2509_d_n8 * p.p249);
        let eq200_e2511_d_n9: f64 = (eq200_e2509_d_n9 * p.p249);
        let eq200_e2511_d_n12: f64 = (eq200_e2509_d_n12 * p.p249);
        let eq200_e2511_d_n14: f64 = (eq200_e2509_d_n14 * p.p249);
        let eq200_e2511_d_n15: f64 = (eq200_e2509_d_n15 * p.p249);
        let eq200_e2511_d_n16: f64 = (eq200_e2509_d_n16 * p.p249);
        let eq200_e2511_d_n17: f64 = (eq200_e2509_d_n17 * p.p249);
        let eq200_e2511_d_n18: f64 = (eq200_e2509_d_n18 * p.p249);
        let eq200_e2511_d_n19: f64 = (eq200_e2509_d_n19 * p.p249);
        let eq200_e2511_d_n20: f64 = (eq200_e2509_d_n20 * p.p249);
        let eq200_e2511_d_n21: f64 = (eq200_e2509_d_n21 * p.p249);
        let eq200_e2511_d_n22: f64 = (eq200_e2509_d_n22 * p.p249);
        let eq200_e2511_q: f64 = (eq200_e2509_q * p.p249);
        (eq200_e2511, eq200_e2511_d_n0, eq200_e2511_d_n1, eq200_e2511_d_n2, eq200_e2511_d_n3, eq200_e2511_d_n4, eq200_e2511_d_n5, eq200_e2511_d_n6, eq200_e2511_d_n7, eq200_e2511_d_n8, eq200_e2511_d_n9, eq200_e2511_d_n12, eq200_e2511_d_n14, eq200_e2511_d_n15, eq200_e2511_d_n16, eq200_e2511_d_n17, eq200_e2511_d_n18, eq200_e2511_d_n19, eq200_e2511_d_n20, eq200_e2511_d_n21, eq200_e2511_d_n22, eq200_e2511_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq200_reactive_node_derivatives: [f64; 23] = [eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, 0.0, 0.0, eq200_e2513_d_n12, 0.0, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22];
        let eq200_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq200_reactive_node_derivatives,
            branches,
            &eq200_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq201_e2526, eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n12, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22, eq201_e2526_q,) = {
    if (((locals.var_guard566 == 0.0) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let eq201_e2523_q: f64 = locals.var_qg_fp4;
        let eq201_e2524: f64 = (p.p7 * locals.var_qg_fp4);
        let eq201_e2524_d_n0: f64 = (p.p7 * locals.var_qg_fp4_dn0);
        let eq201_e2524_d_n1: f64 = (p.p7 * locals.var_qg_fp4_dn1);
        let eq201_e2524_d_n2: f64 = (p.p7 * locals.var_qg_fp4_dn2);
        let eq201_e2524_d_n3: f64 = (p.p7 * locals.var_qg_fp4_dn3);
        let eq201_e2524_d_n4: f64 = (p.p7 * locals.var_qg_fp4_dn4);
        let eq201_e2524_d_n5: f64 = (p.p7 * locals.var_qg_fp4_dn5);
        let eq201_e2524_d_n6: f64 = (p.p7 * locals.var_qg_fp4_dn6);
        let eq201_e2524_d_n7: f64 = (p.p7 * locals.var_qg_fp4_dn7);
        let eq201_e2524_d_n8: f64 = (p.p7 * locals.var_qg_fp4_dn8);
        let eq201_e2524_d_n9: f64 = (p.p7 * locals.var_qg_fp4_dn9);
        let eq201_e2524_d_n12: f64 = (p.p7 * locals.var_qg_fp4_dn12);
        let eq201_e2524_d_n14: f64 = (p.p7 * locals.var_qg_fp4_dn14);
        let eq201_e2524_d_n15: f64 = (p.p7 * locals.var_qg_fp4_dn15);
        let eq201_e2524_d_n16: f64 = (p.p7 * locals.var_qg_fp4_dn16);
        let eq201_e2524_d_n17: f64 = (p.p7 * locals.var_qg_fp4_dn17);
        let eq201_e2524_d_n18: f64 = (p.p7 * locals.var_qg_fp4_dn18);
        let eq201_e2524_d_n19: f64 = (p.p7 * locals.var_qg_fp4_dn19);
        let eq201_e2524_d_n20: f64 = (p.p7 * locals.var_qg_fp4_dn20);
        let eq201_e2524_d_n21: f64 = (p.p7 * locals.var_qg_fp4_dn21);
        let eq201_e2524_d_n22: f64 = (p.p7 * locals.var_qg_fp4_dn22);
        let eq201_e2524_q: f64 = (p.p7 * eq201_e2523_q);
        (eq201_e2524, eq201_e2524_d_n0, eq201_e2524_d_n1, eq201_e2524_d_n2, eq201_e2524_d_n3, eq201_e2524_d_n4, eq201_e2524_d_n5, eq201_e2524_d_n6, eq201_e2524_d_n7, eq201_e2524_d_n8, eq201_e2524_d_n9, eq201_e2524_d_n12, eq201_e2524_d_n14, eq201_e2524_d_n15, eq201_e2524_d_n16, eq201_e2524_d_n17, eq201_e2524_d_n18, eq201_e2524_d_n19, eq201_e2524_d_n20, eq201_e2524_d_n21, eq201_e2524_d_n22, eq201_e2524_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq201_reactive_node_derivatives: [f64; 23] = [eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, 0.0, 0.0, eq201_e2526_d_n12, 0.0, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22];
        let eq201_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq201_reactive_node_derivatives,
            branches,
            &eq201_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq202_e2541, eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n12, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22, eq202_e2541_q,) = {
    if (((locals.var_guard566 == 0.0) && (locals.var_guard569 != 0.0)) && (locals.var_guard570 == 0.0)) {
        let eq202_e2536_q: f64 = locals.var_qg_fp4;
        let eq202_e2537: f64 = (p.p7 * locals.var_qg_fp4);
        let eq202_e2537_d_n0: f64 = (p.p7 * locals.var_qg_fp4_dn0);
        let eq202_e2537_d_n1: f64 = (p.p7 * locals.var_qg_fp4_dn1);
        let eq202_e2537_d_n2: f64 = (p.p7 * locals.var_qg_fp4_dn2);
        let eq202_e2537_d_n3: f64 = (p.p7 * locals.var_qg_fp4_dn3);
        let eq202_e2537_d_n4: f64 = (p.p7 * locals.var_qg_fp4_dn4);
        let eq202_e2537_d_n5: f64 = (p.p7 * locals.var_qg_fp4_dn5);
        let eq202_e2537_d_n6: f64 = (p.p7 * locals.var_qg_fp4_dn6);
        let eq202_e2537_d_n7: f64 = (p.p7 * locals.var_qg_fp4_dn7);
        let eq202_e2537_d_n8: f64 = (p.p7 * locals.var_qg_fp4_dn8);
        let eq202_e2537_d_n9: f64 = (p.p7 * locals.var_qg_fp4_dn9);
        let eq202_e2537_d_n12: f64 = (p.p7 * locals.var_qg_fp4_dn12);
        let eq202_e2537_d_n14: f64 = (p.p7 * locals.var_qg_fp4_dn14);
        let eq202_e2537_d_n15: f64 = (p.p7 * locals.var_qg_fp4_dn15);
        let eq202_e2537_d_n16: f64 = (p.p7 * locals.var_qg_fp4_dn16);
        let eq202_e2537_d_n17: f64 = (p.p7 * locals.var_qg_fp4_dn17);
        let eq202_e2537_d_n18: f64 = (p.p7 * locals.var_qg_fp4_dn18);
        let eq202_e2537_d_n19: f64 = (p.p7 * locals.var_qg_fp4_dn19);
        let eq202_e2537_d_n20: f64 = (p.p7 * locals.var_qg_fp4_dn20);
        let eq202_e2537_d_n21: f64 = (p.p7 * locals.var_qg_fp4_dn21);
        let eq202_e2537_d_n22: f64 = (p.p7 * locals.var_qg_fp4_dn22);
        let eq202_e2537_q: f64 = (p.p7 * eq202_e2536_q);
        let eq202_e2539: f64 = (eq202_e2537 * p.p249);
        let eq202_e2539_d_n0: f64 = (eq202_e2537_d_n0 * p.p249);
        let eq202_e2539_d_n1: f64 = (eq202_e2537_d_n1 * p.p249);
        let eq202_e2539_d_n2: f64 = (eq202_e2537_d_n2 * p.p249);
        let eq202_e2539_d_n3: f64 = (eq202_e2537_d_n3 * p.p249);
        let eq202_e2539_d_n4: f64 = (eq202_e2537_d_n4 * p.p249);
        let eq202_e2539_d_n5: f64 = (eq202_e2537_d_n5 * p.p249);
        let eq202_e2539_d_n6: f64 = (eq202_e2537_d_n6 * p.p249);
        let eq202_e2539_d_n7: f64 = (eq202_e2537_d_n7 * p.p249);
        let eq202_e2539_d_n8: f64 = (eq202_e2537_d_n8 * p.p249);
        let eq202_e2539_d_n9: f64 = (eq202_e2537_d_n9 * p.p249);
        let eq202_e2539_d_n12: f64 = (eq202_e2537_d_n12 * p.p249);
        let eq202_e2539_d_n14: f64 = (eq202_e2537_d_n14 * p.p249);
        let eq202_e2539_d_n15: f64 = (eq202_e2537_d_n15 * p.p249);
        let eq202_e2539_d_n16: f64 = (eq202_e2537_d_n16 * p.p249);
        let eq202_e2539_d_n17: f64 = (eq202_e2537_d_n17 * p.p249);
        let eq202_e2539_d_n18: f64 = (eq202_e2537_d_n18 * p.p249);
        let eq202_e2539_d_n19: f64 = (eq202_e2537_d_n19 * p.p249);
        let eq202_e2539_d_n20: f64 = (eq202_e2537_d_n20 * p.p249);
        let eq202_e2539_d_n21: f64 = (eq202_e2537_d_n21 * p.p249);
        let eq202_e2539_d_n22: f64 = (eq202_e2537_d_n22 * p.p249);
        let eq202_e2539_q: f64 = (eq202_e2537_q * p.p249);
        (eq202_e2539, eq202_e2539_d_n0, eq202_e2539_d_n1, eq202_e2539_d_n2, eq202_e2539_d_n3, eq202_e2539_d_n4, eq202_e2539_d_n5, eq202_e2539_d_n6, eq202_e2539_d_n7, eq202_e2539_d_n8, eq202_e2539_d_n9, eq202_e2539_d_n12, eq202_e2539_d_n14, eq202_e2539_d_n15, eq202_e2539_d_n16, eq202_e2539_d_n17, eq202_e2539_d_n18, eq202_e2539_d_n19, eq202_e2539_d_n20, eq202_e2539_d_n21, eq202_e2539_d_n22, eq202_e2539_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq202_reactive_node_derivatives: [f64; 23] = [eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, 0.0, 0.0, eq202_e2541_d_n12, 0.0, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22];
        let eq202_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq202_reactive_node_derivatives,
            branches,
            &eq202_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq203_e2553, eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n12, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22, eq203_e2553_q,) = {
    if ((locals.var_guard566 == 0.0) && (locals.var_guard569 != 0.0)) {
        let eq203_e2549: f64 = (p.p254 * locals.var_qg_fp4);
        let eq203_e2549_d_n0: f64 = (p.p254 * locals.var_qg_fp4_dn0);
        let eq203_e2549_d_n1: f64 = (p.p254 * locals.var_qg_fp4_dn1);
        let eq203_e2549_d_n2: f64 = (p.p254 * locals.var_qg_fp4_dn2);
        let eq203_e2549_d_n3: f64 = (p.p254 * locals.var_qg_fp4_dn3);
        let eq203_e2549_d_n4: f64 = (p.p254 * locals.var_qg_fp4_dn4);
        let eq203_e2549_d_n5: f64 = (p.p254 * locals.var_qg_fp4_dn5);
        let eq203_e2549_d_n6: f64 = (p.p254 * locals.var_qg_fp4_dn6);
        let eq203_e2549_d_n7: f64 = (p.p254 * locals.var_qg_fp4_dn7);
        let eq203_e2549_d_n8: f64 = (p.p254 * locals.var_qg_fp4_dn8);
        let eq203_e2549_d_n9: f64 = (p.p254 * locals.var_qg_fp4_dn9);
        let eq203_e2549_d_n12: f64 = (p.p254 * locals.var_qg_fp4_dn12);
        let eq203_e2549_d_n14: f64 = (p.p254 * locals.var_qg_fp4_dn14);
        let eq203_e2549_d_n15: f64 = (p.p254 * locals.var_qg_fp4_dn15);
        let eq203_e2549_d_n16: f64 = (p.p254 * locals.var_qg_fp4_dn16);
        let eq203_e2549_d_n17: f64 = (p.p254 * locals.var_qg_fp4_dn17);
        let eq203_e2549_d_n18: f64 = (p.p254 * locals.var_qg_fp4_dn18);
        let eq203_e2549_d_n19: f64 = (p.p254 * locals.var_qg_fp4_dn19);
        let eq203_e2549_d_n20: f64 = (p.p254 * locals.var_qg_fp4_dn20);
        let eq203_e2549_d_n21: f64 = (p.p254 * locals.var_qg_fp4_dn21);
        let eq203_e2549_d_n22: f64 = (p.p254 * locals.var_qg_fp4_dn22);
        let eq203_e2550_q: f64 = eq203_e2549;
        let eq203_e2551: f64 = (p.p7 * eq203_e2549);
        let eq203_e2551_d_n0: f64 = (p.p7 * eq203_e2549_d_n0);
        let eq203_e2551_d_n1: f64 = (p.p7 * eq203_e2549_d_n1);
        let eq203_e2551_d_n2: f64 = (p.p7 * eq203_e2549_d_n2);
        let eq203_e2551_d_n3: f64 = (p.p7 * eq203_e2549_d_n3);
        let eq203_e2551_d_n4: f64 = (p.p7 * eq203_e2549_d_n4);
        let eq203_e2551_d_n5: f64 = (p.p7 * eq203_e2549_d_n5);
        let eq203_e2551_d_n6: f64 = (p.p7 * eq203_e2549_d_n6);
        let eq203_e2551_d_n7: f64 = (p.p7 * eq203_e2549_d_n7);
        let eq203_e2551_d_n8: f64 = (p.p7 * eq203_e2549_d_n8);
        let eq203_e2551_d_n9: f64 = (p.p7 * eq203_e2549_d_n9);
        let eq203_e2551_d_n12: f64 = (p.p7 * eq203_e2549_d_n12);
        let eq203_e2551_d_n14: f64 = (p.p7 * eq203_e2549_d_n14);
        let eq203_e2551_d_n15: f64 = (p.p7 * eq203_e2549_d_n15);
        let eq203_e2551_d_n16: f64 = (p.p7 * eq203_e2549_d_n16);
        let eq203_e2551_d_n17: f64 = (p.p7 * eq203_e2549_d_n17);
        let eq203_e2551_d_n18: f64 = (p.p7 * eq203_e2549_d_n18);
        let eq203_e2551_d_n19: f64 = (p.p7 * eq203_e2549_d_n19);
        let eq203_e2551_d_n20: f64 = (p.p7 * eq203_e2549_d_n20);
        let eq203_e2551_d_n21: f64 = (p.p7 * eq203_e2549_d_n21);
        let eq203_e2551_d_n22: f64 = (p.p7 * eq203_e2549_d_n22);
        let eq203_e2551_q: f64 = (p.p7 * eq203_e2550_q);
        (eq203_e2551, eq203_e2551_d_n0, eq203_e2551_d_n1, eq203_e2551_d_n2, eq203_e2551_d_n3, eq203_e2551_d_n4, eq203_e2551_d_n5, eq203_e2551_d_n6, eq203_e2551_d_n7, eq203_e2551_d_n8, eq203_e2551_d_n9, eq203_e2551_d_n12, eq203_e2551_d_n14, eq203_e2551_d_n15, eq203_e2551_d_n16, eq203_e2551_d_n17, eq203_e2551_d_n18, eq203_e2551_d_n19, eq203_e2551_d_n20, eq203_e2551_d_n21, eq203_e2551_d_n22, eq203_e2551_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq203_reactive_node_derivatives: [f64; 23] = [eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, 0.0, 0.0, eq203_e2553_d_n12, 0.0, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22];
        let eq203_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq203_reactive_node_derivatives,
            branches,
            &eq203_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq204_e2562, eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n12, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22, eq204_e2562_q,) = {
    if ((locals.var_guard571 != 0.0) && (locals.var_guard572 != 0.0)) {
        let eq204_e2559_q: f64 = locals.var_qd_fp4s;
        let eq204_e2560: f64 = (p.p7 * locals.var_qd_fp4s);
        let eq204_e2560_d_n0: f64 = (p.p7 * locals.var_qd_fp4s_dn0);
        let eq204_e2560_d_n1: f64 = (p.p7 * locals.var_qd_fp4s_dn1);
        let eq204_e2560_d_n2: f64 = (p.p7 * locals.var_qd_fp4s_dn2);
        let eq204_e2560_d_n3: f64 = (p.p7 * locals.var_qd_fp4s_dn3);
        let eq204_e2560_d_n4: f64 = (p.p7 * locals.var_qd_fp4s_dn4);
        let eq204_e2560_d_n5: f64 = (p.p7 * locals.var_qd_fp4s_dn5);
        let eq204_e2560_d_n6: f64 = (p.p7 * locals.var_qd_fp4s_dn6);
        let eq204_e2560_d_n7: f64 = (p.p7 * locals.var_qd_fp4s_dn7);
        let eq204_e2560_d_n8: f64 = (p.p7 * locals.var_qd_fp4s_dn8);
        let eq204_e2560_d_n9: f64 = (p.p7 * locals.var_qd_fp4s_dn9);
        let eq204_e2560_d_n12: f64 = (p.p7 * locals.var_qd_fp4s_dn12);
        let eq204_e2560_d_n14: f64 = (p.p7 * locals.var_qd_fp4s_dn14);
        let eq204_e2560_d_n15: f64 = (p.p7 * locals.var_qd_fp4s_dn15);
        let eq204_e2560_d_n16: f64 = (p.p7 * locals.var_qd_fp4s_dn16);
        let eq204_e2560_d_n17: f64 = (p.p7 * locals.var_qd_fp4s_dn17);
        let eq204_e2560_d_n18: f64 = (p.p7 * locals.var_qd_fp4s_dn18);
        let eq204_e2560_d_n19: f64 = (p.p7 * locals.var_qd_fp4s_dn19);
        let eq204_e2560_d_n20: f64 = (p.p7 * locals.var_qd_fp4s_dn20);
        let eq204_e2560_d_n21: f64 = (p.p7 * locals.var_qd_fp4s_dn21);
        let eq204_e2560_d_n22: f64 = (p.p7 * locals.var_qd_fp4s_dn22);
        let eq204_e2560_q: f64 = (p.p7 * eq204_e2559_q);
        (eq204_e2560, eq204_e2560_d_n0, eq204_e2560_d_n1, eq204_e2560_d_n2, eq204_e2560_d_n3, eq204_e2560_d_n4, eq204_e2560_d_n5, eq204_e2560_d_n6, eq204_e2560_d_n7, eq204_e2560_d_n8, eq204_e2560_d_n9, eq204_e2560_d_n12, eq204_e2560_d_n14, eq204_e2560_d_n15, eq204_e2560_d_n16, eq204_e2560_d_n17, eq204_e2560_d_n18, eq204_e2560_d_n19, eq204_e2560_d_n20, eq204_e2560_d_n21, eq204_e2560_d_n22, eq204_e2560_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq204_reactive_node_derivatives: [f64; 23] = [eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, 0.0, 0.0, eq204_e2562_d_n12, 0.0, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22];
        let eq204_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[22]),
            nodes,
            &eq204_reactive_node_derivatives,
            branches,
            &eq204_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_10(
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let (eq205_e2573, eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n12, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22, eq205_e2573_q,) = {
    if (((locals.var_guard571 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 != 0.0)) {
        let eq205_e2570_q: f64 = locals.var_qg_fp4s;
        let eq205_e2571: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq205_e2571_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq205_e2571_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq205_e2571_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq205_e2571_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq205_e2571_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq205_e2571_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq205_e2571_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq205_e2571_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq205_e2571_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq205_e2571_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq205_e2571_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq205_e2571_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq205_e2571_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq205_e2571_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq205_e2571_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq205_e2571_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq205_e2571_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq205_e2571_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq205_e2571_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq205_e2571_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq205_e2571_q: f64 = (p.p7 * eq205_e2570_q);
        (eq205_e2571, eq205_e2571_d_n0, eq205_e2571_d_n1, eq205_e2571_d_n2, eq205_e2571_d_n3, eq205_e2571_d_n4, eq205_e2571_d_n5, eq205_e2571_d_n6, eq205_e2571_d_n7, eq205_e2571_d_n8, eq205_e2571_d_n9, eq205_e2571_d_n12, eq205_e2571_d_n14, eq205_e2571_d_n15, eq205_e2571_d_n16, eq205_e2571_d_n17, eq205_e2571_d_n18, eq205_e2571_d_n19, eq205_e2571_d_n20, eq205_e2571_d_n21, eq205_e2571_d_n22, eq205_e2571_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq205_reactive_node_derivatives: [f64; 23] = [eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, 0.0, 0.0, eq205_e2573_d_n12, 0.0, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22];
        let eq205_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            nodes,
            &eq205_reactive_node_derivatives,
            branches,
            &eq205_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq206_e2586, eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n12, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22, eq206_e2586_q,) = {
    if (((locals.var_guard571 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 != 0.0)) {
        let eq206_e2581_q: f64 = locals.var_qg_fp4s;
        let eq206_e2582: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq206_e2582_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq206_e2582_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq206_e2582_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq206_e2582_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq206_e2582_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq206_e2582_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq206_e2582_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq206_e2582_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq206_e2582_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq206_e2582_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq206_e2582_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq206_e2582_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq206_e2582_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq206_e2582_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq206_e2582_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq206_e2582_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq206_e2582_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq206_e2582_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq206_e2582_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq206_e2582_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq206_e2582_q: f64 = (p.p7 * eq206_e2581_q);
        let eq206_e2584: f64 = (eq206_e2582 * p.p249);
        let eq206_e2584_d_n0: f64 = (eq206_e2582_d_n0 * p.p249);
        let eq206_e2584_d_n1: f64 = (eq206_e2582_d_n1 * p.p249);
        let eq206_e2584_d_n2: f64 = (eq206_e2582_d_n2 * p.p249);
        let eq206_e2584_d_n3: f64 = (eq206_e2582_d_n3 * p.p249);
        let eq206_e2584_d_n4: f64 = (eq206_e2582_d_n4 * p.p249);
        let eq206_e2584_d_n5: f64 = (eq206_e2582_d_n5 * p.p249);
        let eq206_e2584_d_n6: f64 = (eq206_e2582_d_n6 * p.p249);
        let eq206_e2584_d_n7: f64 = (eq206_e2582_d_n7 * p.p249);
        let eq206_e2584_d_n8: f64 = (eq206_e2582_d_n8 * p.p249);
        let eq206_e2584_d_n9: f64 = (eq206_e2582_d_n9 * p.p249);
        let eq206_e2584_d_n12: f64 = (eq206_e2582_d_n12 * p.p249);
        let eq206_e2584_d_n14: f64 = (eq206_e2582_d_n14 * p.p249);
        let eq206_e2584_d_n15: f64 = (eq206_e2582_d_n15 * p.p249);
        let eq206_e2584_d_n16: f64 = (eq206_e2582_d_n16 * p.p249);
        let eq206_e2584_d_n17: f64 = (eq206_e2582_d_n17 * p.p249);
        let eq206_e2584_d_n18: f64 = (eq206_e2582_d_n18 * p.p249);
        let eq206_e2584_d_n19: f64 = (eq206_e2582_d_n19 * p.p249);
        let eq206_e2584_d_n20: f64 = (eq206_e2582_d_n20 * p.p249);
        let eq206_e2584_d_n21: f64 = (eq206_e2582_d_n21 * p.p249);
        let eq206_e2584_d_n22: f64 = (eq206_e2582_d_n22 * p.p249);
        let eq206_e2584_q: f64 = (eq206_e2582_q * p.p249);
        (eq206_e2584, eq206_e2584_d_n0, eq206_e2584_d_n1, eq206_e2584_d_n2, eq206_e2584_d_n3, eq206_e2584_d_n4, eq206_e2584_d_n5, eq206_e2584_d_n6, eq206_e2584_d_n7, eq206_e2584_d_n8, eq206_e2584_d_n9, eq206_e2584_d_n12, eq206_e2584_d_n14, eq206_e2584_d_n15, eq206_e2584_d_n16, eq206_e2584_d_n17, eq206_e2584_d_n18, eq206_e2584_d_n19, eq206_e2584_d_n20, eq206_e2584_d_n21, eq206_e2584_d_n22, eq206_e2584_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq206_reactive_node_derivatives: [f64; 23] = [eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, 0.0, 0.0, eq206_e2586_d_n12, 0.0, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22];
        let eq206_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            nodes,
            &eq206_reactive_node_derivatives,
            branches,
            &eq206_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq207_e2598, eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n12, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22, eq207_e2598_q,) = {
    if (((locals.var_guard571 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 == 0.0)) {
        let eq207_e2595_q: f64 = locals.var_qg_fp4s;
        let eq207_e2596: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq207_e2596_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq207_e2596_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq207_e2596_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq207_e2596_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq207_e2596_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq207_e2596_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq207_e2596_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq207_e2596_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq207_e2596_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq207_e2596_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq207_e2596_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq207_e2596_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq207_e2596_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq207_e2596_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq207_e2596_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq207_e2596_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq207_e2596_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq207_e2596_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq207_e2596_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq207_e2596_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq207_e2596_q: f64 = (p.p7 * eq207_e2595_q);
        (eq207_e2596, eq207_e2596_d_n0, eq207_e2596_d_n1, eq207_e2596_d_n2, eq207_e2596_d_n3, eq207_e2596_d_n4, eq207_e2596_d_n5, eq207_e2596_d_n6, eq207_e2596_d_n7, eq207_e2596_d_n8, eq207_e2596_d_n9, eq207_e2596_d_n12, eq207_e2596_d_n14, eq207_e2596_d_n15, eq207_e2596_d_n16, eq207_e2596_d_n17, eq207_e2596_d_n18, eq207_e2596_d_n19, eq207_e2596_d_n20, eq207_e2596_d_n21, eq207_e2596_d_n22, eq207_e2596_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq207_reactive_node_derivatives: [f64; 23] = [eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, 0.0, 0.0, eq207_e2598_d_n12, 0.0, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22];
        let eq207_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            nodes,
            &eq207_reactive_node_derivatives,
            branches,
            &eq207_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq208_e2612, eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n12, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22, eq208_e2612_q,) = {
    if (((locals.var_guard571 != 0.0) && (locals.var_guard572 != 0.0)) && (locals.var_guard573 == 0.0)) {
        let eq208_e2607_q: f64 = locals.var_qg_fp4s;
        let eq208_e2608: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq208_e2608_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq208_e2608_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq208_e2608_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq208_e2608_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq208_e2608_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq208_e2608_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq208_e2608_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq208_e2608_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq208_e2608_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq208_e2608_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq208_e2608_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq208_e2608_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq208_e2608_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq208_e2608_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq208_e2608_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq208_e2608_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq208_e2608_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq208_e2608_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq208_e2608_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq208_e2608_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq208_e2608_q: f64 = (p.p7 * eq208_e2607_q);
        let eq208_e2610: f64 = (eq208_e2608 * p.p249);
        let eq208_e2610_d_n0: f64 = (eq208_e2608_d_n0 * p.p249);
        let eq208_e2610_d_n1: f64 = (eq208_e2608_d_n1 * p.p249);
        let eq208_e2610_d_n2: f64 = (eq208_e2608_d_n2 * p.p249);
        let eq208_e2610_d_n3: f64 = (eq208_e2608_d_n3 * p.p249);
        let eq208_e2610_d_n4: f64 = (eq208_e2608_d_n4 * p.p249);
        let eq208_e2610_d_n5: f64 = (eq208_e2608_d_n5 * p.p249);
        let eq208_e2610_d_n6: f64 = (eq208_e2608_d_n6 * p.p249);
        let eq208_e2610_d_n7: f64 = (eq208_e2608_d_n7 * p.p249);
        let eq208_e2610_d_n8: f64 = (eq208_e2608_d_n8 * p.p249);
        let eq208_e2610_d_n9: f64 = (eq208_e2608_d_n9 * p.p249);
        let eq208_e2610_d_n12: f64 = (eq208_e2608_d_n12 * p.p249);
        let eq208_e2610_d_n14: f64 = (eq208_e2608_d_n14 * p.p249);
        let eq208_e2610_d_n15: f64 = (eq208_e2608_d_n15 * p.p249);
        let eq208_e2610_d_n16: f64 = (eq208_e2608_d_n16 * p.p249);
        let eq208_e2610_d_n17: f64 = (eq208_e2608_d_n17 * p.p249);
        let eq208_e2610_d_n18: f64 = (eq208_e2608_d_n18 * p.p249);
        let eq208_e2610_d_n19: f64 = (eq208_e2608_d_n19 * p.p249);
        let eq208_e2610_d_n20: f64 = (eq208_e2608_d_n20 * p.p249);
        let eq208_e2610_d_n21: f64 = (eq208_e2608_d_n21 * p.p249);
        let eq208_e2610_d_n22: f64 = (eq208_e2608_d_n22 * p.p249);
        let eq208_e2610_q: f64 = (eq208_e2608_q * p.p249);
        (eq208_e2610, eq208_e2610_d_n0, eq208_e2610_d_n1, eq208_e2610_d_n2, eq208_e2610_d_n3, eq208_e2610_d_n4, eq208_e2610_d_n5, eq208_e2610_d_n6, eq208_e2610_d_n7, eq208_e2610_d_n8, eq208_e2610_d_n9, eq208_e2610_d_n12, eq208_e2610_d_n14, eq208_e2610_d_n15, eq208_e2610_d_n16, eq208_e2610_d_n17, eq208_e2610_d_n18, eq208_e2610_d_n19, eq208_e2610_d_n20, eq208_e2610_d_n21, eq208_e2610_d_n22, eq208_e2610_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq208_reactive_node_derivatives: [f64; 23] = [eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, 0.0, 0.0, eq208_e2612_d_n12, 0.0, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22];
        let eq208_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            nodes,
            &eq208_reactive_node_derivatives,
            branches,
            &eq208_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n12, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22, eq209_e2623_q,) = {
    if ((locals.var_guard571 != 0.0) && (locals.var_guard572 != 0.0)) {
        let eq209_e2619: f64 = (p.p254 * locals.var_qg_fp4s);
        let eq209_e2619_d_n0: f64 = (p.p254 * locals.var_qg_fp4s_dn0);
        let eq209_e2619_d_n1: f64 = (p.p254 * locals.var_qg_fp4s_dn1);
        let eq209_e2619_d_n2: f64 = (p.p254 * locals.var_qg_fp4s_dn2);
        let eq209_e2619_d_n3: f64 = (p.p254 * locals.var_qg_fp4s_dn3);
        let eq209_e2619_d_n4: f64 = (p.p254 * locals.var_qg_fp4s_dn4);
        let eq209_e2619_d_n5: f64 = (p.p254 * locals.var_qg_fp4s_dn5);
        let eq209_e2619_d_n6: f64 = (p.p254 * locals.var_qg_fp4s_dn6);
        let eq209_e2619_d_n7: f64 = (p.p254 * locals.var_qg_fp4s_dn7);
        let eq209_e2619_d_n8: f64 = (p.p254 * locals.var_qg_fp4s_dn8);
        let eq209_e2619_d_n9: f64 = (p.p254 * locals.var_qg_fp4s_dn9);
        let eq209_e2619_d_n12: f64 = (p.p254 * locals.var_qg_fp4s_dn12);
        let eq209_e2619_d_n14: f64 = (p.p254 * locals.var_qg_fp4s_dn14);
        let eq209_e2619_d_n15: f64 = (p.p254 * locals.var_qg_fp4s_dn15);
        let eq209_e2619_d_n16: f64 = (p.p254 * locals.var_qg_fp4s_dn16);
        let eq209_e2619_d_n17: f64 = (p.p254 * locals.var_qg_fp4s_dn17);
        let eq209_e2619_d_n18: f64 = (p.p254 * locals.var_qg_fp4s_dn18);
        let eq209_e2619_d_n19: f64 = (p.p254 * locals.var_qg_fp4s_dn19);
        let eq209_e2619_d_n20: f64 = (p.p254 * locals.var_qg_fp4s_dn20);
        let eq209_e2619_d_n21: f64 = (p.p254 * locals.var_qg_fp4s_dn21);
        let eq209_e2619_d_n22: f64 = (p.p254 * locals.var_qg_fp4s_dn22);
        let eq209_e2620_q: f64 = eq209_e2619;
        let eq209_e2621: f64 = (p.p7 * eq209_e2619);
        let eq209_e2621_d_n0: f64 = (p.p7 * eq209_e2619_d_n0);
        let eq209_e2621_d_n1: f64 = (p.p7 * eq209_e2619_d_n1);
        let eq209_e2621_d_n2: f64 = (p.p7 * eq209_e2619_d_n2);
        let eq209_e2621_d_n3: f64 = (p.p7 * eq209_e2619_d_n3);
        let eq209_e2621_d_n4: f64 = (p.p7 * eq209_e2619_d_n4);
        let eq209_e2621_d_n5: f64 = (p.p7 * eq209_e2619_d_n5);
        let eq209_e2621_d_n6: f64 = (p.p7 * eq209_e2619_d_n6);
        let eq209_e2621_d_n7: f64 = (p.p7 * eq209_e2619_d_n7);
        let eq209_e2621_d_n8: f64 = (p.p7 * eq209_e2619_d_n8);
        let eq209_e2621_d_n9: f64 = (p.p7 * eq209_e2619_d_n9);
        let eq209_e2621_d_n12: f64 = (p.p7 * eq209_e2619_d_n12);
        let eq209_e2621_d_n14: f64 = (p.p7 * eq209_e2619_d_n14);
        let eq209_e2621_d_n15: f64 = (p.p7 * eq209_e2619_d_n15);
        let eq209_e2621_d_n16: f64 = (p.p7 * eq209_e2619_d_n16);
        let eq209_e2621_d_n17: f64 = (p.p7 * eq209_e2619_d_n17);
        let eq209_e2621_d_n18: f64 = (p.p7 * eq209_e2619_d_n18);
        let eq209_e2621_d_n19: f64 = (p.p7 * eq209_e2619_d_n19);
        let eq209_e2621_d_n20: f64 = (p.p7 * eq209_e2619_d_n20);
        let eq209_e2621_d_n21: f64 = (p.p7 * eq209_e2619_d_n21);
        let eq209_e2621_d_n22: f64 = (p.p7 * eq209_e2619_d_n22);
        let eq209_e2621_q: f64 = (p.p7 * eq209_e2620_q);
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n12, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22, eq209_e2621_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_reactive_node_derivatives: [f64; 23] = [eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, 0.0, 0.0, eq209_e2623_d_n12, 0.0, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22];
        let eq209_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[22]),
            nodes,
            &eq209_reactive_node_derivatives,
            branches,
            &eq209_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n12, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22, eq210_e2633_q,) = {
    if ((locals.var_guard571 == 0.0) && (locals.var_guard574 != 0.0)) {
        let eq210_e2630_q: f64 = locals.var_qd_fp4s;
        let eq210_e2631: f64 = (p.p7 * locals.var_qd_fp4s);
        let eq210_e2631_d_n0: f64 = (p.p7 * locals.var_qd_fp4s_dn0);
        let eq210_e2631_d_n1: f64 = (p.p7 * locals.var_qd_fp4s_dn1);
        let eq210_e2631_d_n2: f64 = (p.p7 * locals.var_qd_fp4s_dn2);
        let eq210_e2631_d_n3: f64 = (p.p7 * locals.var_qd_fp4s_dn3);
        let eq210_e2631_d_n4: f64 = (p.p7 * locals.var_qd_fp4s_dn4);
        let eq210_e2631_d_n5: f64 = (p.p7 * locals.var_qd_fp4s_dn5);
        let eq210_e2631_d_n6: f64 = (p.p7 * locals.var_qd_fp4s_dn6);
        let eq210_e2631_d_n7: f64 = (p.p7 * locals.var_qd_fp4s_dn7);
        let eq210_e2631_d_n8: f64 = (p.p7 * locals.var_qd_fp4s_dn8);
        let eq210_e2631_d_n9: f64 = (p.p7 * locals.var_qd_fp4s_dn9);
        let eq210_e2631_d_n12: f64 = (p.p7 * locals.var_qd_fp4s_dn12);
        let eq210_e2631_d_n14: f64 = (p.p7 * locals.var_qd_fp4s_dn14);
        let eq210_e2631_d_n15: f64 = (p.p7 * locals.var_qd_fp4s_dn15);
        let eq210_e2631_d_n16: f64 = (p.p7 * locals.var_qd_fp4s_dn16);
        let eq210_e2631_d_n17: f64 = (p.p7 * locals.var_qd_fp4s_dn17);
        let eq210_e2631_d_n18: f64 = (p.p7 * locals.var_qd_fp4s_dn18);
        let eq210_e2631_d_n19: f64 = (p.p7 * locals.var_qd_fp4s_dn19);
        let eq210_e2631_d_n20: f64 = (p.p7 * locals.var_qd_fp4s_dn20);
        let eq210_e2631_d_n21: f64 = (p.p7 * locals.var_qd_fp4s_dn21);
        let eq210_e2631_d_n22: f64 = (p.p7 * locals.var_qd_fp4s_dn22);
        let eq210_e2631_q: f64 = (p.p7 * eq210_e2630_q);
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n12, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22, eq210_e2631_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_reactive_node_derivatives: [f64; 23] = [eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, 0.0, 0.0, eq210_e2633_d_n12, 0.0, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22];
        let eq210_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq210_reactive_node_derivatives,
            branches,
            &eq210_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n12, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22, eq211_e2645_q,) = {
    if (((locals.var_guard571 == 0.0) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 != 0.0)) {
        let eq211_e2642_q: f64 = locals.var_qg_fp4s;
        let eq211_e2643: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq211_e2643_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq211_e2643_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq211_e2643_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq211_e2643_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq211_e2643_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq211_e2643_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq211_e2643_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq211_e2643_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq211_e2643_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq211_e2643_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq211_e2643_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq211_e2643_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq211_e2643_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq211_e2643_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq211_e2643_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq211_e2643_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq211_e2643_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq211_e2643_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq211_e2643_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq211_e2643_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq211_e2643_q: f64 = (p.p7 * eq211_e2642_q);
        (eq211_e2643, eq211_e2643_d_n0, eq211_e2643_d_n1, eq211_e2643_d_n2, eq211_e2643_d_n3, eq211_e2643_d_n4, eq211_e2643_d_n5, eq211_e2643_d_n6, eq211_e2643_d_n7, eq211_e2643_d_n8, eq211_e2643_d_n9, eq211_e2643_d_n12, eq211_e2643_d_n14, eq211_e2643_d_n15, eq211_e2643_d_n16, eq211_e2643_d_n17, eq211_e2643_d_n18, eq211_e2643_d_n19, eq211_e2643_d_n20, eq211_e2643_d_n21, eq211_e2643_d_n22, eq211_e2643_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_reactive_node_derivatives: [f64; 23] = [eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, 0.0, 0.0, eq211_e2645_d_n12, 0.0, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22];
        let eq211_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq211_reactive_node_derivatives,
            branches,
            &eq211_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n12, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22, eq212_e2659_q,) = {
    if (((locals.var_guard571 == 0.0) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 != 0.0)) {
        let eq212_e2654_q: f64 = locals.var_qg_fp4s;
        let eq212_e2655: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq212_e2655_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq212_e2655_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq212_e2655_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq212_e2655_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq212_e2655_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq212_e2655_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq212_e2655_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq212_e2655_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq212_e2655_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq212_e2655_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq212_e2655_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq212_e2655_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq212_e2655_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq212_e2655_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq212_e2655_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq212_e2655_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq212_e2655_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq212_e2655_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq212_e2655_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq212_e2655_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq212_e2655_q: f64 = (p.p7 * eq212_e2654_q);
        let eq212_e2657: f64 = (eq212_e2655 * p.p249);
        let eq212_e2657_d_n0: f64 = (eq212_e2655_d_n0 * p.p249);
        let eq212_e2657_d_n1: f64 = (eq212_e2655_d_n1 * p.p249);
        let eq212_e2657_d_n2: f64 = (eq212_e2655_d_n2 * p.p249);
        let eq212_e2657_d_n3: f64 = (eq212_e2655_d_n3 * p.p249);
        let eq212_e2657_d_n4: f64 = (eq212_e2655_d_n4 * p.p249);
        let eq212_e2657_d_n5: f64 = (eq212_e2655_d_n5 * p.p249);
        let eq212_e2657_d_n6: f64 = (eq212_e2655_d_n6 * p.p249);
        let eq212_e2657_d_n7: f64 = (eq212_e2655_d_n7 * p.p249);
        let eq212_e2657_d_n8: f64 = (eq212_e2655_d_n8 * p.p249);
        let eq212_e2657_d_n9: f64 = (eq212_e2655_d_n9 * p.p249);
        let eq212_e2657_d_n12: f64 = (eq212_e2655_d_n12 * p.p249);
        let eq212_e2657_d_n14: f64 = (eq212_e2655_d_n14 * p.p249);
        let eq212_e2657_d_n15: f64 = (eq212_e2655_d_n15 * p.p249);
        let eq212_e2657_d_n16: f64 = (eq212_e2655_d_n16 * p.p249);
        let eq212_e2657_d_n17: f64 = (eq212_e2655_d_n17 * p.p249);
        let eq212_e2657_d_n18: f64 = (eq212_e2655_d_n18 * p.p249);
        let eq212_e2657_d_n19: f64 = (eq212_e2655_d_n19 * p.p249);
        let eq212_e2657_d_n20: f64 = (eq212_e2655_d_n20 * p.p249);
        let eq212_e2657_d_n21: f64 = (eq212_e2655_d_n21 * p.p249);
        let eq212_e2657_d_n22: f64 = (eq212_e2655_d_n22 * p.p249);
        let eq212_e2657_q: f64 = (eq212_e2655_q * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n12, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22, eq212_e2657_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_reactive_node_derivatives: [f64; 23] = [eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, 0.0, 0.0, eq212_e2659_d_n12, 0.0, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22];
        let eq212_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq212_reactive_node_derivatives,
            branches,
            &eq212_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n12, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22, eq213_e2672_q,) = {
    if (((locals.var_guard571 == 0.0) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 == 0.0)) {
        let eq213_e2669_q: f64 = locals.var_qg_fp4s;
        let eq213_e2670: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq213_e2670_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq213_e2670_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq213_e2670_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq213_e2670_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq213_e2670_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq213_e2670_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq213_e2670_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq213_e2670_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq213_e2670_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq213_e2670_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq213_e2670_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq213_e2670_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq213_e2670_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq213_e2670_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq213_e2670_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq213_e2670_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq213_e2670_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq213_e2670_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq213_e2670_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq213_e2670_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq213_e2670_q: f64 = (p.p7 * eq213_e2669_q);
        (eq213_e2670, eq213_e2670_d_n0, eq213_e2670_d_n1, eq213_e2670_d_n2, eq213_e2670_d_n3, eq213_e2670_d_n4, eq213_e2670_d_n5, eq213_e2670_d_n6, eq213_e2670_d_n7, eq213_e2670_d_n8, eq213_e2670_d_n9, eq213_e2670_d_n12, eq213_e2670_d_n14, eq213_e2670_d_n15, eq213_e2670_d_n16, eq213_e2670_d_n17, eq213_e2670_d_n18, eq213_e2670_d_n19, eq213_e2670_d_n20, eq213_e2670_d_n21, eq213_e2670_d_n22, eq213_e2670_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_reactive_node_derivatives: [f64; 23] = [eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, 0.0, 0.0, eq213_e2672_d_n12, 0.0, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22];
        let eq213_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq213_reactive_node_derivatives,
            branches,
            &eq213_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n12, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22, eq214_e2687_q,) = {
    if (((locals.var_guard571 == 0.0) && (locals.var_guard574 != 0.0)) && (locals.var_guard575 == 0.0)) {
        let eq214_e2682_q: f64 = locals.var_qg_fp4s;
        let eq214_e2683: f64 = (p.p7 * locals.var_qg_fp4s);
        let eq214_e2683_d_n0: f64 = (p.p7 * locals.var_qg_fp4s_dn0);
        let eq214_e2683_d_n1: f64 = (p.p7 * locals.var_qg_fp4s_dn1);
        let eq214_e2683_d_n2: f64 = (p.p7 * locals.var_qg_fp4s_dn2);
        let eq214_e2683_d_n3: f64 = (p.p7 * locals.var_qg_fp4s_dn3);
        let eq214_e2683_d_n4: f64 = (p.p7 * locals.var_qg_fp4s_dn4);
        let eq214_e2683_d_n5: f64 = (p.p7 * locals.var_qg_fp4s_dn5);
        let eq214_e2683_d_n6: f64 = (p.p7 * locals.var_qg_fp4s_dn6);
        let eq214_e2683_d_n7: f64 = (p.p7 * locals.var_qg_fp4s_dn7);
        let eq214_e2683_d_n8: f64 = (p.p7 * locals.var_qg_fp4s_dn8);
        let eq214_e2683_d_n9: f64 = (p.p7 * locals.var_qg_fp4s_dn9);
        let eq214_e2683_d_n12: f64 = (p.p7 * locals.var_qg_fp4s_dn12);
        let eq214_e2683_d_n14: f64 = (p.p7 * locals.var_qg_fp4s_dn14);
        let eq214_e2683_d_n15: f64 = (p.p7 * locals.var_qg_fp4s_dn15);
        let eq214_e2683_d_n16: f64 = (p.p7 * locals.var_qg_fp4s_dn16);
        let eq214_e2683_d_n17: f64 = (p.p7 * locals.var_qg_fp4s_dn17);
        let eq214_e2683_d_n18: f64 = (p.p7 * locals.var_qg_fp4s_dn18);
        let eq214_e2683_d_n19: f64 = (p.p7 * locals.var_qg_fp4s_dn19);
        let eq214_e2683_d_n20: f64 = (p.p7 * locals.var_qg_fp4s_dn20);
        let eq214_e2683_d_n21: f64 = (p.p7 * locals.var_qg_fp4s_dn21);
        let eq214_e2683_d_n22: f64 = (p.p7 * locals.var_qg_fp4s_dn22);
        let eq214_e2683_q: f64 = (p.p7 * eq214_e2682_q);
        let eq214_e2685: f64 = (eq214_e2683 * p.p249);
        let eq214_e2685_d_n0: f64 = (eq214_e2683_d_n0 * p.p249);
        let eq214_e2685_d_n1: f64 = (eq214_e2683_d_n1 * p.p249);
        let eq214_e2685_d_n2: f64 = (eq214_e2683_d_n2 * p.p249);
        let eq214_e2685_d_n3: f64 = (eq214_e2683_d_n3 * p.p249);
        let eq214_e2685_d_n4: f64 = (eq214_e2683_d_n4 * p.p249);
        let eq214_e2685_d_n5: f64 = (eq214_e2683_d_n5 * p.p249);
        let eq214_e2685_d_n6: f64 = (eq214_e2683_d_n6 * p.p249);
        let eq214_e2685_d_n7: f64 = (eq214_e2683_d_n7 * p.p249);
        let eq214_e2685_d_n8: f64 = (eq214_e2683_d_n8 * p.p249);
        let eq214_e2685_d_n9: f64 = (eq214_e2683_d_n9 * p.p249);
        let eq214_e2685_d_n12: f64 = (eq214_e2683_d_n12 * p.p249);
        let eq214_e2685_d_n14: f64 = (eq214_e2683_d_n14 * p.p249);
        let eq214_e2685_d_n15: f64 = (eq214_e2683_d_n15 * p.p249);
        let eq214_e2685_d_n16: f64 = (eq214_e2683_d_n16 * p.p249);
        let eq214_e2685_d_n17: f64 = (eq214_e2683_d_n17 * p.p249);
        let eq214_e2685_d_n18: f64 = (eq214_e2683_d_n18 * p.p249);
        let eq214_e2685_d_n19: f64 = (eq214_e2683_d_n19 * p.p249);
        let eq214_e2685_d_n20: f64 = (eq214_e2683_d_n20 * p.p249);
        let eq214_e2685_d_n21: f64 = (eq214_e2683_d_n21 * p.p249);
        let eq214_e2685_d_n22: f64 = (eq214_e2683_d_n22 * p.p249);
        let eq214_e2685_q: f64 = (eq214_e2683_q * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n12, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22, eq214_e2685_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_reactive_node_derivatives: [f64; 23] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, 0.0, 0.0, eq214_e2687_d_n12, 0.0, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq214_reactive_node_derivatives,
            branches,
            &eq214_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n12, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22, eq215_e2699_q,) = {
    if ((locals.var_guard571 == 0.0) && (locals.var_guard574 != 0.0)) {
        let eq215_e2695: f64 = (p.p254 * locals.var_qg_fp4s);
        let eq215_e2695_d_n0: f64 = (p.p254 * locals.var_qg_fp4s_dn0);
        let eq215_e2695_d_n1: f64 = (p.p254 * locals.var_qg_fp4s_dn1);
        let eq215_e2695_d_n2: f64 = (p.p254 * locals.var_qg_fp4s_dn2);
        let eq215_e2695_d_n3: f64 = (p.p254 * locals.var_qg_fp4s_dn3);
        let eq215_e2695_d_n4: f64 = (p.p254 * locals.var_qg_fp4s_dn4);
        let eq215_e2695_d_n5: f64 = (p.p254 * locals.var_qg_fp4s_dn5);
        let eq215_e2695_d_n6: f64 = (p.p254 * locals.var_qg_fp4s_dn6);
        let eq215_e2695_d_n7: f64 = (p.p254 * locals.var_qg_fp4s_dn7);
        let eq215_e2695_d_n8: f64 = (p.p254 * locals.var_qg_fp4s_dn8);
        let eq215_e2695_d_n9: f64 = (p.p254 * locals.var_qg_fp4s_dn9);
        let eq215_e2695_d_n12: f64 = (p.p254 * locals.var_qg_fp4s_dn12);
        let eq215_e2695_d_n14: f64 = (p.p254 * locals.var_qg_fp4s_dn14);
        let eq215_e2695_d_n15: f64 = (p.p254 * locals.var_qg_fp4s_dn15);
        let eq215_e2695_d_n16: f64 = (p.p254 * locals.var_qg_fp4s_dn16);
        let eq215_e2695_d_n17: f64 = (p.p254 * locals.var_qg_fp4s_dn17);
        let eq215_e2695_d_n18: f64 = (p.p254 * locals.var_qg_fp4s_dn18);
        let eq215_e2695_d_n19: f64 = (p.p254 * locals.var_qg_fp4s_dn19);
        let eq215_e2695_d_n20: f64 = (p.p254 * locals.var_qg_fp4s_dn20);
        let eq215_e2695_d_n21: f64 = (p.p254 * locals.var_qg_fp4s_dn21);
        let eq215_e2695_d_n22: f64 = (p.p254 * locals.var_qg_fp4s_dn22);
        let eq215_e2696_q: f64 = eq215_e2695;
        let eq215_e2697: f64 = (p.p7 * eq215_e2695);
        let eq215_e2697_d_n0: f64 = (p.p7 * eq215_e2695_d_n0);
        let eq215_e2697_d_n1: f64 = (p.p7 * eq215_e2695_d_n1);
        let eq215_e2697_d_n2: f64 = (p.p7 * eq215_e2695_d_n2);
        let eq215_e2697_d_n3: f64 = (p.p7 * eq215_e2695_d_n3);
        let eq215_e2697_d_n4: f64 = (p.p7 * eq215_e2695_d_n4);
        let eq215_e2697_d_n5: f64 = (p.p7 * eq215_e2695_d_n5);
        let eq215_e2697_d_n6: f64 = (p.p7 * eq215_e2695_d_n6);
        let eq215_e2697_d_n7: f64 = (p.p7 * eq215_e2695_d_n7);
        let eq215_e2697_d_n8: f64 = (p.p7 * eq215_e2695_d_n8);
        let eq215_e2697_d_n9: f64 = (p.p7 * eq215_e2695_d_n9);
        let eq215_e2697_d_n12: f64 = (p.p7 * eq215_e2695_d_n12);
        let eq215_e2697_d_n14: f64 = (p.p7 * eq215_e2695_d_n14);
        let eq215_e2697_d_n15: f64 = (p.p7 * eq215_e2695_d_n15);
        let eq215_e2697_d_n16: f64 = (p.p7 * eq215_e2695_d_n16);
        let eq215_e2697_d_n17: f64 = (p.p7 * eq215_e2695_d_n17);
        let eq215_e2697_d_n18: f64 = (p.p7 * eq215_e2695_d_n18);
        let eq215_e2697_d_n19: f64 = (p.p7 * eq215_e2695_d_n19);
        let eq215_e2697_d_n20: f64 = (p.p7 * eq215_e2695_d_n20);
        let eq215_e2697_d_n21: f64 = (p.p7 * eq215_e2695_d_n21);
        let eq215_e2697_d_n22: f64 = (p.p7 * eq215_e2695_d_n22);
        let eq215_e2697_q: f64 = (p.p7 * eq215_e2696_q);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n12, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22, eq215_e2697_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_reactive_node_derivatives: [f64; 23] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, 0.0, 0.0, eq215_e2699_d_n12, 0.0, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq215_reactive_node_derivatives,
            branches,
            &eq215_reactive_branch_derivatives,
            multiplicity,
        );
        let eq216_e2702_q: f64 = locals.var_qfr;
        let eq216_e2703: f64 = (p.p7 * locals.var_qfr);
        let eq216_e2703_d_n0: f64 = (p.p7 * locals.var_qfr_dn0);
        let eq216_e2703_d_n2: f64 = (p.p7 * locals.var_qfr_dn2);
        let eq216_e2703_d_n4: f64 = (p.p7 * locals.var_qfr_dn4);
        let eq216_e2703_q: f64 = (p.p7 * eq216_e2702_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq216_e2703_d_n0),
            nodes[2],
            multiplicity * (eq216_e2703_d_n2),
            nodes[4],
            multiplicity * (eq216_e2703_d_n4),
        );
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2712_q: f64 = eq217_e2711;
        let eq217_e2713: f64 = (p.p7 * eq217_e2711);
        let eq217_e2713_d_n1: f64 = (p.p7 * eq217_e2709);
        let eq217_e2713_d_n2: f64 = (p.p7 * (-eq217_e2709));
        let eq217_e2713_q: f64 = (p.p7 * eq217_e2712_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq217_e2713_d_n1),
            nodes[2],
            multiplicity * (eq217_e2713_d_n2),
        );
        let eq218_e2716_q: f64 = locals.var_qfr2;
        let eq218_e2717: f64 = (p.p7 * locals.var_qfr2);
        let eq218_e2717_d_n0: f64 = (p.p7 * locals.var_qfr2_dn0);
        let eq218_e2717_d_n1: f64 = (p.p7 * locals.var_qfr2_dn1);
        let eq218_e2717_d_n2: f64 = (p.p7 * locals.var_qfr2_dn2);
        let eq218_e2717_d_n3: f64 = (p.p7 * locals.var_qfr2_dn3);
        let eq218_e2717_d_n4: f64 = (p.p7 * locals.var_qfr2_dn4);
        let eq218_e2717_d_n5: f64 = (p.p7 * locals.var_qfr2_dn5);
        let eq218_e2717_d_n6: f64 = (p.p7 * locals.var_qfr2_dn6);
        let eq218_e2717_d_n7: f64 = (p.p7 * locals.var_qfr2_dn7);
        let eq218_e2717_d_n8: f64 = (p.p7 * locals.var_qfr2_dn8);
        let eq218_e2717_d_n9: f64 = (p.p7 * locals.var_qfr2_dn9);
        let eq218_e2717_d_n12: f64 = (p.p7 * locals.var_qfr2_dn12);
        let eq218_e2717_d_n14: f64 = (p.p7 * locals.var_qfr2_dn14);
        let eq218_e2717_d_n15: f64 = (p.p7 * locals.var_qfr2_dn15);
        let eq218_e2717_d_n16: f64 = (p.p7 * locals.var_qfr2_dn16);
        let eq218_e2717_d_n17: f64 = (p.p7 * locals.var_qfr2_dn17);
        let eq218_e2717_d_n18: f64 = (p.p7 * locals.var_qfr2_dn18);
        let eq218_e2717_d_n19: f64 = (p.p7 * locals.var_qfr2_dn19);
        let eq218_e2717_d_n20: f64 = (p.p7 * locals.var_qfr2_dn20);
        let eq218_e2717_d_n21: f64 = (p.p7 * locals.var_qfr2_dn21);
        let eq218_e2717_d_n22: f64 = (p.p7 * locals.var_qfr2_dn22);
        let eq218_e2717_q: f64 = (p.p7 * eq218_e2716_q);
        let eq218_reactive_node_derivatives: [f64; 23] = [eq218_e2717_d_n0, eq218_e2717_d_n1, eq218_e2717_d_n2, eq218_e2717_d_n3, eq218_e2717_d_n4, eq218_e2717_d_n5, eq218_e2717_d_n6, eq218_e2717_d_n7, eq218_e2717_d_n8, eq218_e2717_d_n9, 0.0, 0.0, eq218_e2717_d_n12, 0.0, eq218_e2717_d_n14, eq218_e2717_d_n15, eq218_e2717_d_n16, eq218_e2717_d_n17, eq218_e2717_d_n18, eq218_e2717_d_n19, eq218_e2717_d_n20, eq218_e2717_d_n21, eq218_e2717_d_n22];
        let eq218_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq218_reactive_node_derivatives,
            branches,
            &eq218_reactive_branch_derivatives,
            multiplicity,
        );
        let eq219_e2720_q: f64 = locals.var_qfr3;
        let eq219_e2721: f64 = (p.p7 * locals.var_qfr3);
        let eq219_e2721_d_n0: f64 = (p.p7 * locals.var_qfr3_dn0);
        let eq219_e2721_d_n2: f64 = (p.p7 * locals.var_qfr3_dn2);
        let eq219_e2721_q: f64 = (p.p7 * eq219_e2720_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq219_e2721_d_n0),
            nodes[2],
            multiplicity * (eq219_e2721_d_n2),
        );
        let eq220_e2724_q: f64 = locals.var_qdep;
        let eq220_e2725: f64 = (p.p7 * locals.var_qdep);
        let eq220_e2725_d_n0: f64 = (p.p7 * locals.var_qdep_dn0);
        let eq220_e2725_d_n1: f64 = (p.p7 * locals.var_qdep_dn1);
        let eq220_e2725_d_n2: f64 = (p.p7 * locals.var_qdep_dn2);
        let eq220_e2725_d_n3: f64 = (p.p7 * locals.var_qdep_dn3);
        let eq220_e2725_d_n4: f64 = (p.p7 * locals.var_qdep_dn4);
        let eq220_e2725_d_n5: f64 = (p.p7 * locals.var_qdep_dn5);
        let eq220_e2725_d_n6: f64 = (p.p7 * locals.var_qdep_dn6);
        let eq220_e2725_d_n7: f64 = (p.p7 * locals.var_qdep_dn7);
        let eq220_e2725_d_n8: f64 = (p.p7 * locals.var_qdep_dn8);
        let eq220_e2725_d_n9: f64 = (p.p7 * locals.var_qdep_dn9);
        let eq220_e2725_d_n12: f64 = (p.p7 * locals.var_qdep_dn12);
        let eq220_e2725_d_n14: f64 = (p.p7 * locals.var_qdep_dn14);
        let eq220_e2725_d_n15: f64 = (p.p7 * locals.var_qdep_dn15);
        let eq220_e2725_d_n16: f64 = (p.p7 * locals.var_qdep_dn16);
        let eq220_e2725_d_n17: f64 = (p.p7 * locals.var_qdep_dn17);
        let eq220_e2725_d_n18: f64 = (p.p7 * locals.var_qdep_dn18);
        let eq220_e2725_d_n19: f64 = (p.p7 * locals.var_qdep_dn19);
        let eq220_e2725_d_n20: f64 = (p.p7 * locals.var_qdep_dn20);
        let eq220_e2725_d_n21: f64 = (p.p7 * locals.var_qdep_dn21);
        let eq220_e2725_d_n22: f64 = (p.p7 * locals.var_qdep_dn22);
        let eq220_e2725_q: f64 = (p.p7 * eq220_e2724_q);
        let eq220_reactive_node_derivatives: [f64; 23] = [eq220_e2725_d_n0, eq220_e2725_d_n1, eq220_e2725_d_n2, eq220_e2725_d_n3, eq220_e2725_d_n4, eq220_e2725_d_n5, eq220_e2725_d_n6, eq220_e2725_d_n7, eq220_e2725_d_n8, eq220_e2725_d_n9, 0.0, 0.0, eq220_e2725_d_n12, 0.0, eq220_e2725_d_n14, eq220_e2725_d_n15, eq220_e2725_d_n16, eq220_e2725_d_n17, eq220_e2725_d_n18, eq220_e2725_d_n19, eq220_e2725_d_n20, eq220_e2725_d_n21, eq220_e2725_d_n22];
        let eq220_reactive_branch_derivatives: [f64; 55] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &eq220_reactive_node_derivatives,
            branches,
            &eq220_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq223_e2771, eq223_e2771_d_n4, eq223_e2771_q,) = {
    if (locals.var_guard576 != 0.0) {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2769_q: f64 = eq223_e2768;
        (eq223_e2768, p.p33, eq223_e2769_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq223_e2771_d_n4),
        );
    }
}
