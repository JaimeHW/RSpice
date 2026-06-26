#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_22(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq196_e2466, eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22,) = {
    if ((s.b[600] && s.b[601]) && (!s.b[602])) {
        let eq196_e2461: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 95, s.v[300]);
        let eq196_e2461_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq196_e2461_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq196_e2461_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq196_e2461_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq196_e2461_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq196_e2461_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq196_e2461_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq196_e2461_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq196_e2461_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq196_e2461_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq196_e2461_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq196_e2461_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq196_e2461_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq196_e2461_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq196_e2461_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq196_e2461_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq196_e2461_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq196_e2461_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq196_e2461_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq196_e2461_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq196_e2461_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq196_e2461_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq196_e2461_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq196_e2462: f64 = (p.p7 * eq196_e2461);
        let eq196_e2462_d_n0: f64 = (p.p7 * eq196_e2461_d_n0);
        let eq196_e2462_d_n1: f64 = (p.p7 * eq196_e2461_d_n1);
        let eq196_e2462_d_n2: f64 = (p.p7 * eq196_e2461_d_n2);
        let eq196_e2462_d_n3: f64 = (p.p7 * eq196_e2461_d_n3);
        let eq196_e2462_d_n4: f64 = (p.p7 * eq196_e2461_d_n4);
        let eq196_e2462_d_n5: f64 = (p.p7 * eq196_e2461_d_n5);
        let eq196_e2462_d_n6: f64 = (p.p7 * eq196_e2461_d_n6);
        let eq196_e2462_d_n7: f64 = (p.p7 * eq196_e2461_d_n7);
        let eq196_e2462_d_n8: f64 = (p.p7 * eq196_e2461_d_n8);
        let eq196_e2462_d_n9: f64 = (p.p7 * eq196_e2461_d_n9);
        let eq196_e2462_d_n10: f64 = (p.p7 * eq196_e2461_d_n10);
        let eq196_e2462_d_n11: f64 = (p.p7 * eq196_e2461_d_n11);
        let eq196_e2462_d_n12: f64 = (p.p7 * eq196_e2461_d_n12);
        let eq196_e2462_d_n13: f64 = (p.p7 * eq196_e2461_d_n13);
        let eq196_e2462_d_n14: f64 = (p.p7 * eq196_e2461_d_n14);
        let eq196_e2462_d_n15: f64 = (p.p7 * eq196_e2461_d_n15);
        let eq196_e2462_d_n16: f64 = (p.p7 * eq196_e2461_d_n16);
        let eq196_e2462_d_n17: f64 = (p.p7 * eq196_e2461_d_n17);
        let eq196_e2462_d_n18: f64 = (p.p7 * eq196_e2461_d_n18);
        let eq196_e2462_d_n19: f64 = (p.p7 * eq196_e2461_d_n19);
        let eq196_e2462_d_n20: f64 = (p.p7 * eq196_e2461_d_n20);
        let eq196_e2462_d_n21: f64 = (p.p7 * eq196_e2461_d_n21);
        let eq196_e2462_d_n22: f64 = (p.p7 * eq196_e2461_d_n22);
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
        let eq196_e2464_d_n10: f64 = (eq196_e2462_d_n10 * p.p249);
        let eq196_e2464_d_n11: f64 = (eq196_e2462_d_n11 * p.p249);
        let eq196_e2464_d_n12: f64 = (eq196_e2462_d_n12 * p.p249);
        let eq196_e2464_d_n13: f64 = (eq196_e2462_d_n13 * p.p249);
        let eq196_e2464_d_n14: f64 = (eq196_e2462_d_n14 * p.p249);
        let eq196_e2464_d_n15: f64 = (eq196_e2462_d_n15 * p.p249);
        let eq196_e2464_d_n16: f64 = (eq196_e2462_d_n16 * p.p249);
        let eq196_e2464_d_n17: f64 = (eq196_e2462_d_n17 * p.p249);
        let eq196_e2464_d_n18: f64 = (eq196_e2462_d_n18 * p.p249);
        let eq196_e2464_d_n19: f64 = (eq196_e2462_d_n19 * p.p249);
        let eq196_e2464_d_n20: f64 = (eq196_e2462_d_n20 * p.p249);
        let eq196_e2464_d_n21: f64 = (eq196_e2462_d_n21 * p.p249);
        let eq196_e2464_d_n22: f64 = (eq196_e2462_d_n22 * p.p249);
        (eq196_e2464, eq196_e2464_d_n0, eq196_e2464_d_n1, eq196_e2464_d_n2, eq196_e2464_d_n3, eq196_e2464_d_n4, eq196_e2464_d_n5, eq196_e2464_d_n6, eq196_e2464_d_n7, eq196_e2464_d_n8, eq196_e2464_d_n9, eq196_e2464_d_n10, eq196_e2464_d_n11, eq196_e2464_d_n12, eq196_e2464_d_n13, eq196_e2464_d_n14, eq196_e2464_d_n15, eq196_e2464_d_n16, eq196_e2464_d_n17, eq196_e2464_d_n18, eq196_e2464_d_n19, eq196_e2464_d_n20, eq196_e2464_d_n21, eq196_e2464_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2466;
        let eq196_node_derivatives: [f64; 23] = [eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22];
        let eq196_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            multiplicity * (eq196_value),
            nodes,
            &eq196_node_derivatives,
            branches,
            &eq196_branch_derivatives,
            multiplicity,
        );
        let (eq197_e2477, eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22,) = {
    if (s.b[600] && s.b[601]) {
        let eq197_e2473: f64 = (p.p254 * s.v[300]);
        let eq197_e2473_d_n0: f64 = (p.p254 * s.dn[300][0]);
        let eq197_e2473_d_n1: f64 = (p.p254 * s.dn[300][1]);
        let eq197_e2473_d_n2: f64 = (p.p254 * s.dn[300][2]);
        let eq197_e2473_d_n3: f64 = (p.p254 * s.dn[300][3]);
        let eq197_e2473_d_n4: f64 = (p.p254 * s.dn[300][4]);
        let eq197_e2473_d_n5: f64 = (p.p254 * s.dn[300][5]);
        let eq197_e2473_d_n6: f64 = (p.p254 * s.dn[300][6]);
        let eq197_e2473_d_n7: f64 = (p.p254 * s.dn[300][7]);
        let eq197_e2473_d_n8: f64 = (p.p254 * s.dn[300][8]);
        let eq197_e2473_d_n9: f64 = (p.p254 * s.dn[300][9]);
        let eq197_e2473_d_n10: f64 = (p.p254 * s.dn[300][10]);
        let eq197_e2473_d_n11: f64 = (p.p254 * s.dn[300][11]);
        let eq197_e2473_d_n12: f64 = (p.p254 * s.dn[300][12]);
        let eq197_e2473_d_n13: f64 = (p.p254 * s.dn[300][13]);
        let eq197_e2473_d_n14: f64 = (p.p254 * s.dn[300][14]);
        let eq197_e2473_d_n15: f64 = (p.p254 * s.dn[300][15]);
        let eq197_e2473_d_n16: f64 = (p.p254 * s.dn[300][16]);
        let eq197_e2473_d_n17: f64 = (p.p254 * s.dn[300][17]);
        let eq197_e2473_d_n18: f64 = (p.p254 * s.dn[300][18]);
        let eq197_e2473_d_n19: f64 = (p.p254 * s.dn[300][19]);
        let eq197_e2473_d_n20: f64 = (p.p254 * s.dn[300][20]);
        let eq197_e2473_d_n21: f64 = (p.p254 * s.dn[300][21]);
        let eq197_e2473_d_n22: f64 = (p.p254 * s.dn[300][22]);
        let eq197_e2474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 96, eq197_e2473);
        let eq197_e2474_d_n0: f64 = (eq197_e2473_d_n0 * ddt_scale);
        let eq197_e2474_d_n1: f64 = (eq197_e2473_d_n1 * ddt_scale);
        let eq197_e2474_d_n2: f64 = (eq197_e2473_d_n2 * ddt_scale);
        let eq197_e2474_d_n3: f64 = (eq197_e2473_d_n3 * ddt_scale);
        let eq197_e2474_d_n4: f64 = (eq197_e2473_d_n4 * ddt_scale);
        let eq197_e2474_d_n5: f64 = (eq197_e2473_d_n5 * ddt_scale);
        let eq197_e2474_d_n6: f64 = (eq197_e2473_d_n6 * ddt_scale);
        let eq197_e2474_d_n7: f64 = (eq197_e2473_d_n7 * ddt_scale);
        let eq197_e2474_d_n8: f64 = (eq197_e2473_d_n8 * ddt_scale);
        let eq197_e2474_d_n9: f64 = (eq197_e2473_d_n9 * ddt_scale);
        let eq197_e2474_d_n10: f64 = (eq197_e2473_d_n10 * ddt_scale);
        let eq197_e2474_d_n11: f64 = (eq197_e2473_d_n11 * ddt_scale);
        let eq197_e2474_d_n12: f64 = (eq197_e2473_d_n12 * ddt_scale);
        let eq197_e2474_d_n13: f64 = (eq197_e2473_d_n13 * ddt_scale);
        let eq197_e2474_d_n14: f64 = (eq197_e2473_d_n14 * ddt_scale);
        let eq197_e2474_d_n15: f64 = (eq197_e2473_d_n15 * ddt_scale);
        let eq197_e2474_d_n16: f64 = (eq197_e2473_d_n16 * ddt_scale);
        let eq197_e2474_d_n17: f64 = (eq197_e2473_d_n17 * ddt_scale);
        let eq197_e2474_d_n18: f64 = (eq197_e2473_d_n18 * ddt_scale);
        let eq197_e2474_d_n19: f64 = (eq197_e2473_d_n19 * ddt_scale);
        let eq197_e2474_d_n20: f64 = (eq197_e2473_d_n20 * ddt_scale);
        let eq197_e2474_d_n21: f64 = (eq197_e2473_d_n21 * ddt_scale);
        let eq197_e2474_d_n22: f64 = (eq197_e2473_d_n22 * ddt_scale);
        let eq197_e2475: f64 = (p.p7 * eq197_e2474);
        let eq197_e2475_d_n0: f64 = (p.p7 * eq197_e2474_d_n0);
        let eq197_e2475_d_n1: f64 = (p.p7 * eq197_e2474_d_n1);
        let eq197_e2475_d_n2: f64 = (p.p7 * eq197_e2474_d_n2);
        let eq197_e2475_d_n3: f64 = (p.p7 * eq197_e2474_d_n3);
        let eq197_e2475_d_n4: f64 = (p.p7 * eq197_e2474_d_n4);
        let eq197_e2475_d_n5: f64 = (p.p7 * eq197_e2474_d_n5);
        let eq197_e2475_d_n6: f64 = (p.p7 * eq197_e2474_d_n6);
        let eq197_e2475_d_n7: f64 = (p.p7 * eq197_e2474_d_n7);
        let eq197_e2475_d_n8: f64 = (p.p7 * eq197_e2474_d_n8);
        let eq197_e2475_d_n9: f64 = (p.p7 * eq197_e2474_d_n9);
        let eq197_e2475_d_n10: f64 = (p.p7 * eq197_e2474_d_n10);
        let eq197_e2475_d_n11: f64 = (p.p7 * eq197_e2474_d_n11);
        let eq197_e2475_d_n12: f64 = (p.p7 * eq197_e2474_d_n12);
        let eq197_e2475_d_n13: f64 = (p.p7 * eq197_e2474_d_n13);
        let eq197_e2475_d_n14: f64 = (p.p7 * eq197_e2474_d_n14);
        let eq197_e2475_d_n15: f64 = (p.p7 * eq197_e2474_d_n15);
        let eq197_e2475_d_n16: f64 = (p.p7 * eq197_e2474_d_n16);
        let eq197_e2475_d_n17: f64 = (p.p7 * eq197_e2474_d_n17);
        let eq197_e2475_d_n18: f64 = (p.p7 * eq197_e2474_d_n18);
        let eq197_e2475_d_n19: f64 = (p.p7 * eq197_e2474_d_n19);
        let eq197_e2475_d_n20: f64 = (p.p7 * eq197_e2474_d_n20);
        let eq197_e2475_d_n21: f64 = (p.p7 * eq197_e2474_d_n21);
        let eq197_e2475_d_n22: f64 = (p.p7 * eq197_e2474_d_n22);
        (eq197_e2475, eq197_e2475_d_n0, eq197_e2475_d_n1, eq197_e2475_d_n2, eq197_e2475_d_n3, eq197_e2475_d_n4, eq197_e2475_d_n5, eq197_e2475_d_n6, eq197_e2475_d_n7, eq197_e2475_d_n8, eq197_e2475_d_n9, eq197_e2475_d_n10, eq197_e2475_d_n11, eq197_e2475_d_n12, eq197_e2475_d_n13, eq197_e2475_d_n14, eq197_e2475_d_n15, eq197_e2475_d_n16, eq197_e2475_d_n17, eq197_e2475_d_n18, eq197_e2475_d_n19, eq197_e2475_d_n20, eq197_e2475_d_n21, eq197_e2475_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq197_value: f64 = eq197_e2477;
        let eq197_node_derivatives: [f64; 23] = [eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22];
        let eq197_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[17]),
            multiplicity * (eq197_value),
            nodes,
            &eq197_node_derivatives,
            branches,
            &eq197_branch_derivatives,
            multiplicity,
        );
        let (eq198_e2487, eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22,) = {
    if ((!s.b[600]) && s.b[603]) {
        let eq198_e2484: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 97, s.v[301]);
        let eq198_e2484_d_n0: f64 = (s.dn[301][0] * ddt_scale);
        let eq198_e2484_d_n1: f64 = (s.dn[301][1] * ddt_scale);
        let eq198_e2484_d_n2: f64 = (s.dn[301][2] * ddt_scale);
        let eq198_e2484_d_n3: f64 = (s.dn[301][3] * ddt_scale);
        let eq198_e2484_d_n4: f64 = (s.dn[301][4] * ddt_scale);
        let eq198_e2484_d_n5: f64 = (s.dn[301][5] * ddt_scale);
        let eq198_e2484_d_n6: f64 = (s.dn[301][6] * ddt_scale);
        let eq198_e2484_d_n7: f64 = (s.dn[301][7] * ddt_scale);
        let eq198_e2484_d_n8: f64 = (s.dn[301][8] * ddt_scale);
        let eq198_e2484_d_n9: f64 = (s.dn[301][9] * ddt_scale);
        let eq198_e2484_d_n10: f64 = (s.dn[301][10] * ddt_scale);
        let eq198_e2484_d_n11: f64 = (s.dn[301][11] * ddt_scale);
        let eq198_e2484_d_n12: f64 = (s.dn[301][12] * ddt_scale);
        let eq198_e2484_d_n13: f64 = (s.dn[301][13] * ddt_scale);
        let eq198_e2484_d_n14: f64 = (s.dn[301][14] * ddt_scale);
        let eq198_e2484_d_n15: f64 = (s.dn[301][15] * ddt_scale);
        let eq198_e2484_d_n16: f64 = (s.dn[301][16] * ddt_scale);
        let eq198_e2484_d_n17: f64 = (s.dn[301][17] * ddt_scale);
        let eq198_e2484_d_n18: f64 = (s.dn[301][18] * ddt_scale);
        let eq198_e2484_d_n19: f64 = (s.dn[301][19] * ddt_scale);
        let eq198_e2484_d_n20: f64 = (s.dn[301][20] * ddt_scale);
        let eq198_e2484_d_n21: f64 = (s.dn[301][21] * ddt_scale);
        let eq198_e2484_d_n22: f64 = (s.dn[301][22] * ddt_scale);
        let eq198_e2485: f64 = (p.p7 * eq198_e2484);
        let eq198_e2485_d_n0: f64 = (p.p7 * eq198_e2484_d_n0);
        let eq198_e2485_d_n1: f64 = (p.p7 * eq198_e2484_d_n1);
        let eq198_e2485_d_n2: f64 = (p.p7 * eq198_e2484_d_n2);
        let eq198_e2485_d_n3: f64 = (p.p7 * eq198_e2484_d_n3);
        let eq198_e2485_d_n4: f64 = (p.p7 * eq198_e2484_d_n4);
        let eq198_e2485_d_n5: f64 = (p.p7 * eq198_e2484_d_n5);
        let eq198_e2485_d_n6: f64 = (p.p7 * eq198_e2484_d_n6);
        let eq198_e2485_d_n7: f64 = (p.p7 * eq198_e2484_d_n7);
        let eq198_e2485_d_n8: f64 = (p.p7 * eq198_e2484_d_n8);
        let eq198_e2485_d_n9: f64 = (p.p7 * eq198_e2484_d_n9);
        let eq198_e2485_d_n10: f64 = (p.p7 * eq198_e2484_d_n10);
        let eq198_e2485_d_n11: f64 = (p.p7 * eq198_e2484_d_n11);
        let eq198_e2485_d_n12: f64 = (p.p7 * eq198_e2484_d_n12);
        let eq198_e2485_d_n13: f64 = (p.p7 * eq198_e2484_d_n13);
        let eq198_e2485_d_n14: f64 = (p.p7 * eq198_e2484_d_n14);
        let eq198_e2485_d_n15: f64 = (p.p7 * eq198_e2484_d_n15);
        let eq198_e2485_d_n16: f64 = (p.p7 * eq198_e2484_d_n16);
        let eq198_e2485_d_n17: f64 = (p.p7 * eq198_e2484_d_n17);
        let eq198_e2485_d_n18: f64 = (p.p7 * eq198_e2484_d_n18);
        let eq198_e2485_d_n19: f64 = (p.p7 * eq198_e2484_d_n19);
        let eq198_e2485_d_n20: f64 = (p.p7 * eq198_e2484_d_n20);
        let eq198_e2485_d_n21: f64 = (p.p7 * eq198_e2484_d_n21);
        let eq198_e2485_d_n22: f64 = (p.p7 * eq198_e2484_d_n22);
        (eq198_e2485, eq198_e2485_d_n0, eq198_e2485_d_n1, eq198_e2485_d_n2, eq198_e2485_d_n3, eq198_e2485_d_n4, eq198_e2485_d_n5, eq198_e2485_d_n6, eq198_e2485_d_n7, eq198_e2485_d_n8, eq198_e2485_d_n9, eq198_e2485_d_n10, eq198_e2485_d_n11, eq198_e2485_d_n12, eq198_e2485_d_n13, eq198_e2485_d_n14, eq198_e2485_d_n15, eq198_e2485_d_n16, eq198_e2485_d_n17, eq198_e2485_d_n18, eq198_e2485_d_n19, eq198_e2485_d_n20, eq198_e2485_d_n21, eq198_e2485_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq198_value: f64 = eq198_e2487;
        let eq198_node_derivatives: [f64; 23] = [eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22];
        let eq198_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            multiplicity * (eq198_value),
            nodes,
            &eq198_node_derivatives,
            branches,
            &eq198_branch_derivatives,
            multiplicity,
        );
        let (eq199_e2499, eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && s.b[604]) {
        let eq199_e2496: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 98, s.v[300]);
        let eq199_e2496_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq199_e2496_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq199_e2496_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq199_e2496_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq199_e2496_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq199_e2496_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq199_e2496_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq199_e2496_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq199_e2496_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq199_e2496_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq199_e2496_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq199_e2496_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq199_e2496_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq199_e2496_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq199_e2496_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq199_e2496_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq199_e2496_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq199_e2496_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq199_e2496_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq199_e2496_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq199_e2496_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq199_e2496_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq199_e2496_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq199_e2497: f64 = (p.p7 * eq199_e2496);
        let eq199_e2497_d_n0: f64 = (p.p7 * eq199_e2496_d_n0);
        let eq199_e2497_d_n1: f64 = (p.p7 * eq199_e2496_d_n1);
        let eq199_e2497_d_n2: f64 = (p.p7 * eq199_e2496_d_n2);
        let eq199_e2497_d_n3: f64 = (p.p7 * eq199_e2496_d_n3);
        let eq199_e2497_d_n4: f64 = (p.p7 * eq199_e2496_d_n4);
        let eq199_e2497_d_n5: f64 = (p.p7 * eq199_e2496_d_n5);
        let eq199_e2497_d_n6: f64 = (p.p7 * eq199_e2496_d_n6);
        let eq199_e2497_d_n7: f64 = (p.p7 * eq199_e2496_d_n7);
        let eq199_e2497_d_n8: f64 = (p.p7 * eq199_e2496_d_n8);
        let eq199_e2497_d_n9: f64 = (p.p7 * eq199_e2496_d_n9);
        let eq199_e2497_d_n10: f64 = (p.p7 * eq199_e2496_d_n10);
        let eq199_e2497_d_n11: f64 = (p.p7 * eq199_e2496_d_n11);
        let eq199_e2497_d_n12: f64 = (p.p7 * eq199_e2496_d_n12);
        let eq199_e2497_d_n13: f64 = (p.p7 * eq199_e2496_d_n13);
        let eq199_e2497_d_n14: f64 = (p.p7 * eq199_e2496_d_n14);
        let eq199_e2497_d_n15: f64 = (p.p7 * eq199_e2496_d_n15);
        let eq199_e2497_d_n16: f64 = (p.p7 * eq199_e2496_d_n16);
        let eq199_e2497_d_n17: f64 = (p.p7 * eq199_e2496_d_n17);
        let eq199_e2497_d_n18: f64 = (p.p7 * eq199_e2496_d_n18);
        let eq199_e2497_d_n19: f64 = (p.p7 * eq199_e2496_d_n19);
        let eq199_e2497_d_n20: f64 = (p.p7 * eq199_e2496_d_n20);
        let eq199_e2497_d_n21: f64 = (p.p7 * eq199_e2496_d_n21);
        let eq199_e2497_d_n22: f64 = (p.p7 * eq199_e2496_d_n22);
        (eq199_e2497, eq199_e2497_d_n0, eq199_e2497_d_n1, eq199_e2497_d_n2, eq199_e2497_d_n3, eq199_e2497_d_n4, eq199_e2497_d_n5, eq199_e2497_d_n6, eq199_e2497_d_n7, eq199_e2497_d_n8, eq199_e2497_d_n9, eq199_e2497_d_n10, eq199_e2497_d_n11, eq199_e2497_d_n12, eq199_e2497_d_n13, eq199_e2497_d_n14, eq199_e2497_d_n15, eq199_e2497_d_n16, eq199_e2497_d_n17, eq199_e2497_d_n18, eq199_e2497_d_n19, eq199_e2497_d_n20, eq199_e2497_d_n21, eq199_e2497_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq199_value: f64 = eq199_e2499;
        let eq199_node_derivatives: [f64; 23] = [eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22];
        let eq199_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq199_value),
            nodes,
            &eq199_node_derivatives,
            branches,
            &eq199_branch_derivatives,
            multiplicity,
        );
        let (eq200_e2513, eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && s.b[604]) {
        let eq200_e2508: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 99, s.v[300]);
        let eq200_e2508_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq200_e2508_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq200_e2508_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq200_e2508_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq200_e2508_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq200_e2508_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq200_e2508_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq200_e2508_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq200_e2508_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq200_e2508_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq200_e2508_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq200_e2508_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq200_e2508_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq200_e2508_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq200_e2508_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq200_e2508_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq200_e2508_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq200_e2508_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq200_e2508_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq200_e2508_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq200_e2508_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq200_e2508_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq200_e2508_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq200_e2509: f64 = (p.p7 * eq200_e2508);
        let eq200_e2509_d_n0: f64 = (p.p7 * eq200_e2508_d_n0);
        let eq200_e2509_d_n1: f64 = (p.p7 * eq200_e2508_d_n1);
        let eq200_e2509_d_n2: f64 = (p.p7 * eq200_e2508_d_n2);
        let eq200_e2509_d_n3: f64 = (p.p7 * eq200_e2508_d_n3);
        let eq200_e2509_d_n4: f64 = (p.p7 * eq200_e2508_d_n4);
        let eq200_e2509_d_n5: f64 = (p.p7 * eq200_e2508_d_n5);
        let eq200_e2509_d_n6: f64 = (p.p7 * eq200_e2508_d_n6);
        let eq200_e2509_d_n7: f64 = (p.p7 * eq200_e2508_d_n7);
        let eq200_e2509_d_n8: f64 = (p.p7 * eq200_e2508_d_n8);
        let eq200_e2509_d_n9: f64 = (p.p7 * eq200_e2508_d_n9);
        let eq200_e2509_d_n10: f64 = (p.p7 * eq200_e2508_d_n10);
        let eq200_e2509_d_n11: f64 = (p.p7 * eq200_e2508_d_n11);
        let eq200_e2509_d_n12: f64 = (p.p7 * eq200_e2508_d_n12);
        let eq200_e2509_d_n13: f64 = (p.p7 * eq200_e2508_d_n13);
        let eq200_e2509_d_n14: f64 = (p.p7 * eq200_e2508_d_n14);
        let eq200_e2509_d_n15: f64 = (p.p7 * eq200_e2508_d_n15);
        let eq200_e2509_d_n16: f64 = (p.p7 * eq200_e2508_d_n16);
        let eq200_e2509_d_n17: f64 = (p.p7 * eq200_e2508_d_n17);
        let eq200_e2509_d_n18: f64 = (p.p7 * eq200_e2508_d_n18);
        let eq200_e2509_d_n19: f64 = (p.p7 * eq200_e2508_d_n19);
        let eq200_e2509_d_n20: f64 = (p.p7 * eq200_e2508_d_n20);
        let eq200_e2509_d_n21: f64 = (p.p7 * eq200_e2508_d_n21);
        let eq200_e2509_d_n22: f64 = (p.p7 * eq200_e2508_d_n22);
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
        let eq200_e2511_d_n10: f64 = (eq200_e2509_d_n10 * p.p249);
        let eq200_e2511_d_n11: f64 = (eq200_e2509_d_n11 * p.p249);
        let eq200_e2511_d_n12: f64 = (eq200_e2509_d_n12 * p.p249);
        let eq200_e2511_d_n13: f64 = (eq200_e2509_d_n13 * p.p249);
        let eq200_e2511_d_n14: f64 = (eq200_e2509_d_n14 * p.p249);
        let eq200_e2511_d_n15: f64 = (eq200_e2509_d_n15 * p.p249);
        let eq200_e2511_d_n16: f64 = (eq200_e2509_d_n16 * p.p249);
        let eq200_e2511_d_n17: f64 = (eq200_e2509_d_n17 * p.p249);
        let eq200_e2511_d_n18: f64 = (eq200_e2509_d_n18 * p.p249);
        let eq200_e2511_d_n19: f64 = (eq200_e2509_d_n19 * p.p249);
        let eq200_e2511_d_n20: f64 = (eq200_e2509_d_n20 * p.p249);
        let eq200_e2511_d_n21: f64 = (eq200_e2509_d_n21 * p.p249);
        let eq200_e2511_d_n22: f64 = (eq200_e2509_d_n22 * p.p249);
        (eq200_e2511, eq200_e2511_d_n0, eq200_e2511_d_n1, eq200_e2511_d_n2, eq200_e2511_d_n3, eq200_e2511_d_n4, eq200_e2511_d_n5, eq200_e2511_d_n6, eq200_e2511_d_n7, eq200_e2511_d_n8, eq200_e2511_d_n9, eq200_e2511_d_n10, eq200_e2511_d_n11, eq200_e2511_d_n12, eq200_e2511_d_n13, eq200_e2511_d_n14, eq200_e2511_d_n15, eq200_e2511_d_n16, eq200_e2511_d_n17, eq200_e2511_d_n18, eq200_e2511_d_n19, eq200_e2511_d_n20, eq200_e2511_d_n21, eq200_e2511_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq200_value: f64 = eq200_e2513;
        let eq200_node_derivatives: [f64; 23] = [eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22];
        let eq200_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq200_value),
            nodes,
            &eq200_node_derivatives,
            branches,
            &eq200_branch_derivatives,
            multiplicity,
        );
        let (eq201_e2526, eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && (!s.b[604])) {
        let eq201_e2523: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 100, s.v[300]);
        let eq201_e2523_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq201_e2523_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq201_e2523_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq201_e2523_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq201_e2523_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq201_e2523_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq201_e2523_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq201_e2523_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq201_e2523_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq201_e2523_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq201_e2523_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq201_e2523_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq201_e2523_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq201_e2523_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq201_e2523_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq201_e2523_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq201_e2523_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq201_e2523_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq201_e2523_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq201_e2523_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq201_e2523_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq201_e2523_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq201_e2523_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq201_e2524: f64 = (p.p7 * eq201_e2523);
        let eq201_e2524_d_n0: f64 = (p.p7 * eq201_e2523_d_n0);
        let eq201_e2524_d_n1: f64 = (p.p7 * eq201_e2523_d_n1);
        let eq201_e2524_d_n2: f64 = (p.p7 * eq201_e2523_d_n2);
        let eq201_e2524_d_n3: f64 = (p.p7 * eq201_e2523_d_n3);
        let eq201_e2524_d_n4: f64 = (p.p7 * eq201_e2523_d_n4);
        let eq201_e2524_d_n5: f64 = (p.p7 * eq201_e2523_d_n5);
        let eq201_e2524_d_n6: f64 = (p.p7 * eq201_e2523_d_n6);
        let eq201_e2524_d_n7: f64 = (p.p7 * eq201_e2523_d_n7);
        let eq201_e2524_d_n8: f64 = (p.p7 * eq201_e2523_d_n8);
        let eq201_e2524_d_n9: f64 = (p.p7 * eq201_e2523_d_n9);
        let eq201_e2524_d_n10: f64 = (p.p7 * eq201_e2523_d_n10);
        let eq201_e2524_d_n11: f64 = (p.p7 * eq201_e2523_d_n11);
        let eq201_e2524_d_n12: f64 = (p.p7 * eq201_e2523_d_n12);
        let eq201_e2524_d_n13: f64 = (p.p7 * eq201_e2523_d_n13);
        let eq201_e2524_d_n14: f64 = (p.p7 * eq201_e2523_d_n14);
        let eq201_e2524_d_n15: f64 = (p.p7 * eq201_e2523_d_n15);
        let eq201_e2524_d_n16: f64 = (p.p7 * eq201_e2523_d_n16);
        let eq201_e2524_d_n17: f64 = (p.p7 * eq201_e2523_d_n17);
        let eq201_e2524_d_n18: f64 = (p.p7 * eq201_e2523_d_n18);
        let eq201_e2524_d_n19: f64 = (p.p7 * eq201_e2523_d_n19);
        let eq201_e2524_d_n20: f64 = (p.p7 * eq201_e2523_d_n20);
        let eq201_e2524_d_n21: f64 = (p.p7 * eq201_e2523_d_n21);
        let eq201_e2524_d_n22: f64 = (p.p7 * eq201_e2523_d_n22);
        (eq201_e2524, eq201_e2524_d_n0, eq201_e2524_d_n1, eq201_e2524_d_n2, eq201_e2524_d_n3, eq201_e2524_d_n4, eq201_e2524_d_n5, eq201_e2524_d_n6, eq201_e2524_d_n7, eq201_e2524_d_n8, eq201_e2524_d_n9, eq201_e2524_d_n10, eq201_e2524_d_n11, eq201_e2524_d_n12, eq201_e2524_d_n13, eq201_e2524_d_n14, eq201_e2524_d_n15, eq201_e2524_d_n16, eq201_e2524_d_n17, eq201_e2524_d_n18, eq201_e2524_d_n19, eq201_e2524_d_n20, eq201_e2524_d_n21, eq201_e2524_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq201_value: f64 = eq201_e2526;
        let eq201_node_derivatives: [f64; 23] = [eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22];
        let eq201_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq201_value),
            nodes,
            &eq201_node_derivatives,
            branches,
            &eq201_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_23(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq202_e2541, eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && (!s.b[604])) {
        let eq202_e2536: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 101, s.v[300]);
        let eq202_e2536_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq202_e2536_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq202_e2536_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq202_e2536_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq202_e2536_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq202_e2536_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq202_e2536_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq202_e2536_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq202_e2536_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq202_e2536_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq202_e2536_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq202_e2536_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq202_e2536_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq202_e2536_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq202_e2536_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq202_e2536_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq202_e2536_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq202_e2536_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq202_e2536_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq202_e2536_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq202_e2536_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq202_e2536_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq202_e2536_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq202_e2537: f64 = (p.p7 * eq202_e2536);
        let eq202_e2537_d_n0: f64 = (p.p7 * eq202_e2536_d_n0);
        let eq202_e2537_d_n1: f64 = (p.p7 * eq202_e2536_d_n1);
        let eq202_e2537_d_n2: f64 = (p.p7 * eq202_e2536_d_n2);
        let eq202_e2537_d_n3: f64 = (p.p7 * eq202_e2536_d_n3);
        let eq202_e2537_d_n4: f64 = (p.p7 * eq202_e2536_d_n4);
        let eq202_e2537_d_n5: f64 = (p.p7 * eq202_e2536_d_n5);
        let eq202_e2537_d_n6: f64 = (p.p7 * eq202_e2536_d_n6);
        let eq202_e2537_d_n7: f64 = (p.p7 * eq202_e2536_d_n7);
        let eq202_e2537_d_n8: f64 = (p.p7 * eq202_e2536_d_n8);
        let eq202_e2537_d_n9: f64 = (p.p7 * eq202_e2536_d_n9);
        let eq202_e2537_d_n10: f64 = (p.p7 * eq202_e2536_d_n10);
        let eq202_e2537_d_n11: f64 = (p.p7 * eq202_e2536_d_n11);
        let eq202_e2537_d_n12: f64 = (p.p7 * eq202_e2536_d_n12);
        let eq202_e2537_d_n13: f64 = (p.p7 * eq202_e2536_d_n13);
        let eq202_e2537_d_n14: f64 = (p.p7 * eq202_e2536_d_n14);
        let eq202_e2537_d_n15: f64 = (p.p7 * eq202_e2536_d_n15);
        let eq202_e2537_d_n16: f64 = (p.p7 * eq202_e2536_d_n16);
        let eq202_e2537_d_n17: f64 = (p.p7 * eq202_e2536_d_n17);
        let eq202_e2537_d_n18: f64 = (p.p7 * eq202_e2536_d_n18);
        let eq202_e2537_d_n19: f64 = (p.p7 * eq202_e2536_d_n19);
        let eq202_e2537_d_n20: f64 = (p.p7 * eq202_e2536_d_n20);
        let eq202_e2537_d_n21: f64 = (p.p7 * eq202_e2536_d_n21);
        let eq202_e2537_d_n22: f64 = (p.p7 * eq202_e2536_d_n22);
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
        let eq202_e2539_d_n10: f64 = (eq202_e2537_d_n10 * p.p249);
        let eq202_e2539_d_n11: f64 = (eq202_e2537_d_n11 * p.p249);
        let eq202_e2539_d_n12: f64 = (eq202_e2537_d_n12 * p.p249);
        let eq202_e2539_d_n13: f64 = (eq202_e2537_d_n13 * p.p249);
        let eq202_e2539_d_n14: f64 = (eq202_e2537_d_n14 * p.p249);
        let eq202_e2539_d_n15: f64 = (eq202_e2537_d_n15 * p.p249);
        let eq202_e2539_d_n16: f64 = (eq202_e2537_d_n16 * p.p249);
        let eq202_e2539_d_n17: f64 = (eq202_e2537_d_n17 * p.p249);
        let eq202_e2539_d_n18: f64 = (eq202_e2537_d_n18 * p.p249);
        let eq202_e2539_d_n19: f64 = (eq202_e2537_d_n19 * p.p249);
        let eq202_e2539_d_n20: f64 = (eq202_e2537_d_n20 * p.p249);
        let eq202_e2539_d_n21: f64 = (eq202_e2537_d_n21 * p.p249);
        let eq202_e2539_d_n22: f64 = (eq202_e2537_d_n22 * p.p249);
        (eq202_e2539, eq202_e2539_d_n0, eq202_e2539_d_n1, eq202_e2539_d_n2, eq202_e2539_d_n3, eq202_e2539_d_n4, eq202_e2539_d_n5, eq202_e2539_d_n6, eq202_e2539_d_n7, eq202_e2539_d_n8, eq202_e2539_d_n9, eq202_e2539_d_n10, eq202_e2539_d_n11, eq202_e2539_d_n12, eq202_e2539_d_n13, eq202_e2539_d_n14, eq202_e2539_d_n15, eq202_e2539_d_n16, eq202_e2539_d_n17, eq202_e2539_d_n18, eq202_e2539_d_n19, eq202_e2539_d_n20, eq202_e2539_d_n21, eq202_e2539_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq202_value: f64 = eq202_e2541;
        let eq202_node_derivatives: [f64; 23] = [eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22];
        let eq202_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq202_value),
            nodes,
            &eq202_node_derivatives,
            branches,
            &eq202_branch_derivatives,
            multiplicity,
        );
        let (eq203_e2553, eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22,) = {
    if ((!s.b[600]) && s.b[603]) {
        let eq203_e2549: f64 = (p.p254 * s.v[300]);
        let eq203_e2549_d_n0: f64 = (p.p254 * s.dn[300][0]);
        let eq203_e2549_d_n1: f64 = (p.p254 * s.dn[300][1]);
        let eq203_e2549_d_n2: f64 = (p.p254 * s.dn[300][2]);
        let eq203_e2549_d_n3: f64 = (p.p254 * s.dn[300][3]);
        let eq203_e2549_d_n4: f64 = (p.p254 * s.dn[300][4]);
        let eq203_e2549_d_n5: f64 = (p.p254 * s.dn[300][5]);
        let eq203_e2549_d_n6: f64 = (p.p254 * s.dn[300][6]);
        let eq203_e2549_d_n7: f64 = (p.p254 * s.dn[300][7]);
        let eq203_e2549_d_n8: f64 = (p.p254 * s.dn[300][8]);
        let eq203_e2549_d_n9: f64 = (p.p254 * s.dn[300][9]);
        let eq203_e2549_d_n10: f64 = (p.p254 * s.dn[300][10]);
        let eq203_e2549_d_n11: f64 = (p.p254 * s.dn[300][11]);
        let eq203_e2549_d_n12: f64 = (p.p254 * s.dn[300][12]);
        let eq203_e2549_d_n13: f64 = (p.p254 * s.dn[300][13]);
        let eq203_e2549_d_n14: f64 = (p.p254 * s.dn[300][14]);
        let eq203_e2549_d_n15: f64 = (p.p254 * s.dn[300][15]);
        let eq203_e2549_d_n16: f64 = (p.p254 * s.dn[300][16]);
        let eq203_e2549_d_n17: f64 = (p.p254 * s.dn[300][17]);
        let eq203_e2549_d_n18: f64 = (p.p254 * s.dn[300][18]);
        let eq203_e2549_d_n19: f64 = (p.p254 * s.dn[300][19]);
        let eq203_e2549_d_n20: f64 = (p.p254 * s.dn[300][20]);
        let eq203_e2549_d_n21: f64 = (p.p254 * s.dn[300][21]);
        let eq203_e2549_d_n22: f64 = (p.p254 * s.dn[300][22]);
        let eq203_e2550: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 102, eq203_e2549);
        let eq203_e2550_d_n0: f64 = (eq203_e2549_d_n0 * ddt_scale);
        let eq203_e2550_d_n1: f64 = (eq203_e2549_d_n1 * ddt_scale);
        let eq203_e2550_d_n2: f64 = (eq203_e2549_d_n2 * ddt_scale);
        let eq203_e2550_d_n3: f64 = (eq203_e2549_d_n3 * ddt_scale);
        let eq203_e2550_d_n4: f64 = (eq203_e2549_d_n4 * ddt_scale);
        let eq203_e2550_d_n5: f64 = (eq203_e2549_d_n5 * ddt_scale);
        let eq203_e2550_d_n6: f64 = (eq203_e2549_d_n6 * ddt_scale);
        let eq203_e2550_d_n7: f64 = (eq203_e2549_d_n7 * ddt_scale);
        let eq203_e2550_d_n8: f64 = (eq203_e2549_d_n8 * ddt_scale);
        let eq203_e2550_d_n9: f64 = (eq203_e2549_d_n9 * ddt_scale);
        let eq203_e2550_d_n10: f64 = (eq203_e2549_d_n10 * ddt_scale);
        let eq203_e2550_d_n11: f64 = (eq203_e2549_d_n11 * ddt_scale);
        let eq203_e2550_d_n12: f64 = (eq203_e2549_d_n12 * ddt_scale);
        let eq203_e2550_d_n13: f64 = (eq203_e2549_d_n13 * ddt_scale);
        let eq203_e2550_d_n14: f64 = (eq203_e2549_d_n14 * ddt_scale);
        let eq203_e2550_d_n15: f64 = (eq203_e2549_d_n15 * ddt_scale);
        let eq203_e2550_d_n16: f64 = (eq203_e2549_d_n16 * ddt_scale);
        let eq203_e2550_d_n17: f64 = (eq203_e2549_d_n17 * ddt_scale);
        let eq203_e2550_d_n18: f64 = (eq203_e2549_d_n18 * ddt_scale);
        let eq203_e2550_d_n19: f64 = (eq203_e2549_d_n19 * ddt_scale);
        let eq203_e2550_d_n20: f64 = (eq203_e2549_d_n20 * ddt_scale);
        let eq203_e2550_d_n21: f64 = (eq203_e2549_d_n21 * ddt_scale);
        let eq203_e2550_d_n22: f64 = (eq203_e2549_d_n22 * ddt_scale);
        let eq203_e2551: f64 = (p.p7 * eq203_e2550);
        let eq203_e2551_d_n0: f64 = (p.p7 * eq203_e2550_d_n0);
        let eq203_e2551_d_n1: f64 = (p.p7 * eq203_e2550_d_n1);
        let eq203_e2551_d_n2: f64 = (p.p7 * eq203_e2550_d_n2);
        let eq203_e2551_d_n3: f64 = (p.p7 * eq203_e2550_d_n3);
        let eq203_e2551_d_n4: f64 = (p.p7 * eq203_e2550_d_n4);
        let eq203_e2551_d_n5: f64 = (p.p7 * eq203_e2550_d_n5);
        let eq203_e2551_d_n6: f64 = (p.p7 * eq203_e2550_d_n6);
        let eq203_e2551_d_n7: f64 = (p.p7 * eq203_e2550_d_n7);
        let eq203_e2551_d_n8: f64 = (p.p7 * eq203_e2550_d_n8);
        let eq203_e2551_d_n9: f64 = (p.p7 * eq203_e2550_d_n9);
        let eq203_e2551_d_n10: f64 = (p.p7 * eq203_e2550_d_n10);
        let eq203_e2551_d_n11: f64 = (p.p7 * eq203_e2550_d_n11);
        let eq203_e2551_d_n12: f64 = (p.p7 * eq203_e2550_d_n12);
        let eq203_e2551_d_n13: f64 = (p.p7 * eq203_e2550_d_n13);
        let eq203_e2551_d_n14: f64 = (p.p7 * eq203_e2550_d_n14);
        let eq203_e2551_d_n15: f64 = (p.p7 * eq203_e2550_d_n15);
        let eq203_e2551_d_n16: f64 = (p.p7 * eq203_e2550_d_n16);
        let eq203_e2551_d_n17: f64 = (p.p7 * eq203_e2550_d_n17);
        let eq203_e2551_d_n18: f64 = (p.p7 * eq203_e2550_d_n18);
        let eq203_e2551_d_n19: f64 = (p.p7 * eq203_e2550_d_n19);
        let eq203_e2551_d_n20: f64 = (p.p7 * eq203_e2550_d_n20);
        let eq203_e2551_d_n21: f64 = (p.p7 * eq203_e2550_d_n21);
        let eq203_e2551_d_n22: f64 = (p.p7 * eq203_e2550_d_n22);
        (eq203_e2551, eq203_e2551_d_n0, eq203_e2551_d_n1, eq203_e2551_d_n2, eq203_e2551_d_n3, eq203_e2551_d_n4, eq203_e2551_d_n5, eq203_e2551_d_n6, eq203_e2551_d_n7, eq203_e2551_d_n8, eq203_e2551_d_n9, eq203_e2551_d_n10, eq203_e2551_d_n11, eq203_e2551_d_n12, eq203_e2551_d_n13, eq203_e2551_d_n14, eq203_e2551_d_n15, eq203_e2551_d_n16, eq203_e2551_d_n17, eq203_e2551_d_n18, eq203_e2551_d_n19, eq203_e2551_d_n20, eq203_e2551_d_n21, eq203_e2551_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq203_value: f64 = eq203_e2553;
        let eq203_node_derivatives: [f64; 23] = [eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22];
        let eq203_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            multiplicity * (eq203_value),
            nodes,
            &eq203_node_derivatives,
            branches,
            &eq203_branch_derivatives,
            multiplicity,
        );
        let (eq204_e2562, eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22,) = {
    if (s.b[605] && s.b[606]) {
        let eq204_e2559: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 103, s.v[313]);
        let eq204_e2559_d_n0: f64 = (s.dn[313][0] * ddt_scale);
        let eq204_e2559_d_n1: f64 = (s.dn[313][1] * ddt_scale);
        let eq204_e2559_d_n2: f64 = (s.dn[313][2] * ddt_scale);
        let eq204_e2559_d_n3: f64 = (s.dn[313][3] * ddt_scale);
        let eq204_e2559_d_n4: f64 = (s.dn[313][4] * ddt_scale);
        let eq204_e2559_d_n5: f64 = (s.dn[313][5] * ddt_scale);
        let eq204_e2559_d_n6: f64 = (s.dn[313][6] * ddt_scale);
        let eq204_e2559_d_n7: f64 = (s.dn[313][7] * ddt_scale);
        let eq204_e2559_d_n8: f64 = (s.dn[313][8] * ddt_scale);
        let eq204_e2559_d_n9: f64 = (s.dn[313][9] * ddt_scale);
        let eq204_e2559_d_n10: f64 = (s.dn[313][10] * ddt_scale);
        let eq204_e2559_d_n11: f64 = (s.dn[313][11] * ddt_scale);
        let eq204_e2559_d_n12: f64 = (s.dn[313][12] * ddt_scale);
        let eq204_e2559_d_n13: f64 = (s.dn[313][13] * ddt_scale);
        let eq204_e2559_d_n14: f64 = (s.dn[313][14] * ddt_scale);
        let eq204_e2559_d_n15: f64 = (s.dn[313][15] * ddt_scale);
        let eq204_e2559_d_n16: f64 = (s.dn[313][16] * ddt_scale);
        let eq204_e2559_d_n17: f64 = (s.dn[313][17] * ddt_scale);
        let eq204_e2559_d_n18: f64 = (s.dn[313][18] * ddt_scale);
        let eq204_e2559_d_n19: f64 = (s.dn[313][19] * ddt_scale);
        let eq204_e2559_d_n20: f64 = (s.dn[313][20] * ddt_scale);
        let eq204_e2559_d_n21: f64 = (s.dn[313][21] * ddt_scale);
        let eq204_e2559_d_n22: f64 = (s.dn[313][22] * ddt_scale);
        let eq204_e2560: f64 = (p.p7 * eq204_e2559);
        let eq204_e2560_d_n0: f64 = (p.p7 * eq204_e2559_d_n0);
        let eq204_e2560_d_n1: f64 = (p.p7 * eq204_e2559_d_n1);
        let eq204_e2560_d_n2: f64 = (p.p7 * eq204_e2559_d_n2);
        let eq204_e2560_d_n3: f64 = (p.p7 * eq204_e2559_d_n3);
        let eq204_e2560_d_n4: f64 = (p.p7 * eq204_e2559_d_n4);
        let eq204_e2560_d_n5: f64 = (p.p7 * eq204_e2559_d_n5);
        let eq204_e2560_d_n6: f64 = (p.p7 * eq204_e2559_d_n6);
        let eq204_e2560_d_n7: f64 = (p.p7 * eq204_e2559_d_n7);
        let eq204_e2560_d_n8: f64 = (p.p7 * eq204_e2559_d_n8);
        let eq204_e2560_d_n9: f64 = (p.p7 * eq204_e2559_d_n9);
        let eq204_e2560_d_n10: f64 = (p.p7 * eq204_e2559_d_n10);
        let eq204_e2560_d_n11: f64 = (p.p7 * eq204_e2559_d_n11);
        let eq204_e2560_d_n12: f64 = (p.p7 * eq204_e2559_d_n12);
        let eq204_e2560_d_n13: f64 = (p.p7 * eq204_e2559_d_n13);
        let eq204_e2560_d_n14: f64 = (p.p7 * eq204_e2559_d_n14);
        let eq204_e2560_d_n15: f64 = (p.p7 * eq204_e2559_d_n15);
        let eq204_e2560_d_n16: f64 = (p.p7 * eq204_e2559_d_n16);
        let eq204_e2560_d_n17: f64 = (p.p7 * eq204_e2559_d_n17);
        let eq204_e2560_d_n18: f64 = (p.p7 * eq204_e2559_d_n18);
        let eq204_e2560_d_n19: f64 = (p.p7 * eq204_e2559_d_n19);
        let eq204_e2560_d_n20: f64 = (p.p7 * eq204_e2559_d_n20);
        let eq204_e2560_d_n21: f64 = (p.p7 * eq204_e2559_d_n21);
        let eq204_e2560_d_n22: f64 = (p.p7 * eq204_e2559_d_n22);
        (eq204_e2560, eq204_e2560_d_n0, eq204_e2560_d_n1, eq204_e2560_d_n2, eq204_e2560_d_n3, eq204_e2560_d_n4, eq204_e2560_d_n5, eq204_e2560_d_n6, eq204_e2560_d_n7, eq204_e2560_d_n8, eq204_e2560_d_n9, eq204_e2560_d_n10, eq204_e2560_d_n11, eq204_e2560_d_n12, eq204_e2560_d_n13, eq204_e2560_d_n14, eq204_e2560_d_n15, eq204_e2560_d_n16, eq204_e2560_d_n17, eq204_e2560_d_n18, eq204_e2560_d_n19, eq204_e2560_d_n20, eq204_e2560_d_n21, eq204_e2560_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq204_value: f64 = eq204_e2562;
        let eq204_node_derivatives: [f64; 23] = [eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22];
        let eq204_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[22]),
            multiplicity * (eq204_value),
            nodes,
            &eq204_node_derivatives,
            branches,
            &eq204_branch_derivatives,
            multiplicity,
        );
        let (eq205_e2573, eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22,) = {
    if ((s.b[605] && s.b[606]) && s.b[607]) {
        let eq205_e2570: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 104, s.v[312]);
        let eq205_e2570_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq205_e2570_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq205_e2570_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq205_e2570_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq205_e2570_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq205_e2570_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq205_e2570_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq205_e2570_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq205_e2570_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq205_e2570_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq205_e2570_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq205_e2570_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq205_e2570_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq205_e2570_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq205_e2570_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq205_e2570_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq205_e2570_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq205_e2570_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq205_e2570_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq205_e2570_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq205_e2570_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq205_e2570_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq205_e2570_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq205_e2571: f64 = (p.p7 * eq205_e2570);
        let eq205_e2571_d_n0: f64 = (p.p7 * eq205_e2570_d_n0);
        let eq205_e2571_d_n1: f64 = (p.p7 * eq205_e2570_d_n1);
        let eq205_e2571_d_n2: f64 = (p.p7 * eq205_e2570_d_n2);
        let eq205_e2571_d_n3: f64 = (p.p7 * eq205_e2570_d_n3);
        let eq205_e2571_d_n4: f64 = (p.p7 * eq205_e2570_d_n4);
        let eq205_e2571_d_n5: f64 = (p.p7 * eq205_e2570_d_n5);
        let eq205_e2571_d_n6: f64 = (p.p7 * eq205_e2570_d_n6);
        let eq205_e2571_d_n7: f64 = (p.p7 * eq205_e2570_d_n7);
        let eq205_e2571_d_n8: f64 = (p.p7 * eq205_e2570_d_n8);
        let eq205_e2571_d_n9: f64 = (p.p7 * eq205_e2570_d_n9);
        let eq205_e2571_d_n10: f64 = (p.p7 * eq205_e2570_d_n10);
        let eq205_e2571_d_n11: f64 = (p.p7 * eq205_e2570_d_n11);
        let eq205_e2571_d_n12: f64 = (p.p7 * eq205_e2570_d_n12);
        let eq205_e2571_d_n13: f64 = (p.p7 * eq205_e2570_d_n13);
        let eq205_e2571_d_n14: f64 = (p.p7 * eq205_e2570_d_n14);
        let eq205_e2571_d_n15: f64 = (p.p7 * eq205_e2570_d_n15);
        let eq205_e2571_d_n16: f64 = (p.p7 * eq205_e2570_d_n16);
        let eq205_e2571_d_n17: f64 = (p.p7 * eq205_e2570_d_n17);
        let eq205_e2571_d_n18: f64 = (p.p7 * eq205_e2570_d_n18);
        let eq205_e2571_d_n19: f64 = (p.p7 * eq205_e2570_d_n19);
        let eq205_e2571_d_n20: f64 = (p.p7 * eq205_e2570_d_n20);
        let eq205_e2571_d_n21: f64 = (p.p7 * eq205_e2570_d_n21);
        let eq205_e2571_d_n22: f64 = (p.p7 * eq205_e2570_d_n22);
        (eq205_e2571, eq205_e2571_d_n0, eq205_e2571_d_n1, eq205_e2571_d_n2, eq205_e2571_d_n3, eq205_e2571_d_n4, eq205_e2571_d_n5, eq205_e2571_d_n6, eq205_e2571_d_n7, eq205_e2571_d_n8, eq205_e2571_d_n9, eq205_e2571_d_n10, eq205_e2571_d_n11, eq205_e2571_d_n12, eq205_e2571_d_n13, eq205_e2571_d_n14, eq205_e2571_d_n15, eq205_e2571_d_n16, eq205_e2571_d_n17, eq205_e2571_d_n18, eq205_e2571_d_n19, eq205_e2571_d_n20, eq205_e2571_d_n21, eq205_e2571_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq205_value: f64 = eq205_e2573;
        let eq205_node_derivatives: [f64; 23] = [eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22];
        let eq205_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            multiplicity * (eq205_value),
            nodes,
            &eq205_node_derivatives,
            branches,
            &eq205_branch_derivatives,
            multiplicity,
        );
        let (eq206_e2586, eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22,) = {
    if ((s.b[605] && s.b[606]) && s.b[607]) {
        let eq206_e2581: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 105, s.v[312]);
        let eq206_e2581_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq206_e2581_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq206_e2581_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq206_e2581_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq206_e2581_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq206_e2581_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq206_e2581_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq206_e2581_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq206_e2581_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq206_e2581_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq206_e2581_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq206_e2581_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq206_e2581_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq206_e2581_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq206_e2581_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq206_e2581_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq206_e2581_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq206_e2581_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq206_e2581_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq206_e2581_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq206_e2581_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq206_e2581_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq206_e2581_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq206_e2582: f64 = (p.p7 * eq206_e2581);
        let eq206_e2582_d_n0: f64 = (p.p7 * eq206_e2581_d_n0);
        let eq206_e2582_d_n1: f64 = (p.p7 * eq206_e2581_d_n1);
        let eq206_e2582_d_n2: f64 = (p.p7 * eq206_e2581_d_n2);
        let eq206_e2582_d_n3: f64 = (p.p7 * eq206_e2581_d_n3);
        let eq206_e2582_d_n4: f64 = (p.p7 * eq206_e2581_d_n4);
        let eq206_e2582_d_n5: f64 = (p.p7 * eq206_e2581_d_n5);
        let eq206_e2582_d_n6: f64 = (p.p7 * eq206_e2581_d_n6);
        let eq206_e2582_d_n7: f64 = (p.p7 * eq206_e2581_d_n7);
        let eq206_e2582_d_n8: f64 = (p.p7 * eq206_e2581_d_n8);
        let eq206_e2582_d_n9: f64 = (p.p7 * eq206_e2581_d_n9);
        let eq206_e2582_d_n10: f64 = (p.p7 * eq206_e2581_d_n10);
        let eq206_e2582_d_n11: f64 = (p.p7 * eq206_e2581_d_n11);
        let eq206_e2582_d_n12: f64 = (p.p7 * eq206_e2581_d_n12);
        let eq206_e2582_d_n13: f64 = (p.p7 * eq206_e2581_d_n13);
        let eq206_e2582_d_n14: f64 = (p.p7 * eq206_e2581_d_n14);
        let eq206_e2582_d_n15: f64 = (p.p7 * eq206_e2581_d_n15);
        let eq206_e2582_d_n16: f64 = (p.p7 * eq206_e2581_d_n16);
        let eq206_e2582_d_n17: f64 = (p.p7 * eq206_e2581_d_n17);
        let eq206_e2582_d_n18: f64 = (p.p7 * eq206_e2581_d_n18);
        let eq206_e2582_d_n19: f64 = (p.p7 * eq206_e2581_d_n19);
        let eq206_e2582_d_n20: f64 = (p.p7 * eq206_e2581_d_n20);
        let eq206_e2582_d_n21: f64 = (p.p7 * eq206_e2581_d_n21);
        let eq206_e2582_d_n22: f64 = (p.p7 * eq206_e2581_d_n22);
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
        let eq206_e2584_d_n10: f64 = (eq206_e2582_d_n10 * p.p249);
        let eq206_e2584_d_n11: f64 = (eq206_e2582_d_n11 * p.p249);
        let eq206_e2584_d_n12: f64 = (eq206_e2582_d_n12 * p.p249);
        let eq206_e2584_d_n13: f64 = (eq206_e2582_d_n13 * p.p249);
        let eq206_e2584_d_n14: f64 = (eq206_e2582_d_n14 * p.p249);
        let eq206_e2584_d_n15: f64 = (eq206_e2582_d_n15 * p.p249);
        let eq206_e2584_d_n16: f64 = (eq206_e2582_d_n16 * p.p249);
        let eq206_e2584_d_n17: f64 = (eq206_e2582_d_n17 * p.p249);
        let eq206_e2584_d_n18: f64 = (eq206_e2582_d_n18 * p.p249);
        let eq206_e2584_d_n19: f64 = (eq206_e2582_d_n19 * p.p249);
        let eq206_e2584_d_n20: f64 = (eq206_e2582_d_n20 * p.p249);
        let eq206_e2584_d_n21: f64 = (eq206_e2582_d_n21 * p.p249);
        let eq206_e2584_d_n22: f64 = (eq206_e2582_d_n22 * p.p249);
        (eq206_e2584, eq206_e2584_d_n0, eq206_e2584_d_n1, eq206_e2584_d_n2, eq206_e2584_d_n3, eq206_e2584_d_n4, eq206_e2584_d_n5, eq206_e2584_d_n6, eq206_e2584_d_n7, eq206_e2584_d_n8, eq206_e2584_d_n9, eq206_e2584_d_n10, eq206_e2584_d_n11, eq206_e2584_d_n12, eq206_e2584_d_n13, eq206_e2584_d_n14, eq206_e2584_d_n15, eq206_e2584_d_n16, eq206_e2584_d_n17, eq206_e2584_d_n18, eq206_e2584_d_n19, eq206_e2584_d_n20, eq206_e2584_d_n21, eq206_e2584_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq206_value: f64 = eq206_e2586;
        let eq206_node_derivatives: [f64; 23] = [eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22];
        let eq206_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            multiplicity * (eq206_value),
            nodes,
            &eq206_node_derivatives,
            branches,
            &eq206_branch_derivatives,
            multiplicity,
        );
        let (eq207_e2598, eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22,) = {
    if ((s.b[605] && s.b[606]) && (!s.b[607])) {
        let eq207_e2595: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 106, s.v[312]);
        let eq207_e2595_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq207_e2595_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq207_e2595_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq207_e2595_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq207_e2595_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq207_e2595_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq207_e2595_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq207_e2595_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq207_e2595_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq207_e2595_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq207_e2595_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq207_e2595_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq207_e2595_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq207_e2595_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq207_e2595_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq207_e2595_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq207_e2595_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq207_e2595_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq207_e2595_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq207_e2595_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq207_e2595_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq207_e2595_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq207_e2595_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq207_e2596: f64 = (p.p7 * eq207_e2595);
        let eq207_e2596_d_n0: f64 = (p.p7 * eq207_e2595_d_n0);
        let eq207_e2596_d_n1: f64 = (p.p7 * eq207_e2595_d_n1);
        let eq207_e2596_d_n2: f64 = (p.p7 * eq207_e2595_d_n2);
        let eq207_e2596_d_n3: f64 = (p.p7 * eq207_e2595_d_n3);
        let eq207_e2596_d_n4: f64 = (p.p7 * eq207_e2595_d_n4);
        let eq207_e2596_d_n5: f64 = (p.p7 * eq207_e2595_d_n5);
        let eq207_e2596_d_n6: f64 = (p.p7 * eq207_e2595_d_n6);
        let eq207_e2596_d_n7: f64 = (p.p7 * eq207_e2595_d_n7);
        let eq207_e2596_d_n8: f64 = (p.p7 * eq207_e2595_d_n8);
        let eq207_e2596_d_n9: f64 = (p.p7 * eq207_e2595_d_n9);
        let eq207_e2596_d_n10: f64 = (p.p7 * eq207_e2595_d_n10);
        let eq207_e2596_d_n11: f64 = (p.p7 * eq207_e2595_d_n11);
        let eq207_e2596_d_n12: f64 = (p.p7 * eq207_e2595_d_n12);
        let eq207_e2596_d_n13: f64 = (p.p7 * eq207_e2595_d_n13);
        let eq207_e2596_d_n14: f64 = (p.p7 * eq207_e2595_d_n14);
        let eq207_e2596_d_n15: f64 = (p.p7 * eq207_e2595_d_n15);
        let eq207_e2596_d_n16: f64 = (p.p7 * eq207_e2595_d_n16);
        let eq207_e2596_d_n17: f64 = (p.p7 * eq207_e2595_d_n17);
        let eq207_e2596_d_n18: f64 = (p.p7 * eq207_e2595_d_n18);
        let eq207_e2596_d_n19: f64 = (p.p7 * eq207_e2595_d_n19);
        let eq207_e2596_d_n20: f64 = (p.p7 * eq207_e2595_d_n20);
        let eq207_e2596_d_n21: f64 = (p.p7 * eq207_e2595_d_n21);
        let eq207_e2596_d_n22: f64 = (p.p7 * eq207_e2595_d_n22);
        (eq207_e2596, eq207_e2596_d_n0, eq207_e2596_d_n1, eq207_e2596_d_n2, eq207_e2596_d_n3, eq207_e2596_d_n4, eq207_e2596_d_n5, eq207_e2596_d_n6, eq207_e2596_d_n7, eq207_e2596_d_n8, eq207_e2596_d_n9, eq207_e2596_d_n10, eq207_e2596_d_n11, eq207_e2596_d_n12, eq207_e2596_d_n13, eq207_e2596_d_n14, eq207_e2596_d_n15, eq207_e2596_d_n16, eq207_e2596_d_n17, eq207_e2596_d_n18, eq207_e2596_d_n19, eq207_e2596_d_n20, eq207_e2596_d_n21, eq207_e2596_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq207_value: f64 = eq207_e2598;
        let eq207_node_derivatives: [f64; 23] = [eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22];
        let eq207_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            multiplicity * (eq207_value),
            nodes,
            &eq207_node_derivatives,
            branches,
            &eq207_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_24(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq208_e2612, eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22,) = {
    if ((s.b[605] && s.b[606]) && (!s.b[607])) {
        let eq208_e2607: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 107, s.v[312]);
        let eq208_e2607_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq208_e2607_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq208_e2607_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq208_e2607_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq208_e2607_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq208_e2607_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq208_e2607_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq208_e2607_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq208_e2607_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq208_e2607_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq208_e2607_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq208_e2607_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq208_e2607_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq208_e2607_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq208_e2607_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq208_e2607_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq208_e2607_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq208_e2607_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq208_e2607_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq208_e2607_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq208_e2607_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq208_e2607_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq208_e2607_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq208_e2608: f64 = (p.p7 * eq208_e2607);
        let eq208_e2608_d_n0: f64 = (p.p7 * eq208_e2607_d_n0);
        let eq208_e2608_d_n1: f64 = (p.p7 * eq208_e2607_d_n1);
        let eq208_e2608_d_n2: f64 = (p.p7 * eq208_e2607_d_n2);
        let eq208_e2608_d_n3: f64 = (p.p7 * eq208_e2607_d_n3);
        let eq208_e2608_d_n4: f64 = (p.p7 * eq208_e2607_d_n4);
        let eq208_e2608_d_n5: f64 = (p.p7 * eq208_e2607_d_n5);
        let eq208_e2608_d_n6: f64 = (p.p7 * eq208_e2607_d_n6);
        let eq208_e2608_d_n7: f64 = (p.p7 * eq208_e2607_d_n7);
        let eq208_e2608_d_n8: f64 = (p.p7 * eq208_e2607_d_n8);
        let eq208_e2608_d_n9: f64 = (p.p7 * eq208_e2607_d_n9);
        let eq208_e2608_d_n10: f64 = (p.p7 * eq208_e2607_d_n10);
        let eq208_e2608_d_n11: f64 = (p.p7 * eq208_e2607_d_n11);
        let eq208_e2608_d_n12: f64 = (p.p7 * eq208_e2607_d_n12);
        let eq208_e2608_d_n13: f64 = (p.p7 * eq208_e2607_d_n13);
        let eq208_e2608_d_n14: f64 = (p.p7 * eq208_e2607_d_n14);
        let eq208_e2608_d_n15: f64 = (p.p7 * eq208_e2607_d_n15);
        let eq208_e2608_d_n16: f64 = (p.p7 * eq208_e2607_d_n16);
        let eq208_e2608_d_n17: f64 = (p.p7 * eq208_e2607_d_n17);
        let eq208_e2608_d_n18: f64 = (p.p7 * eq208_e2607_d_n18);
        let eq208_e2608_d_n19: f64 = (p.p7 * eq208_e2607_d_n19);
        let eq208_e2608_d_n20: f64 = (p.p7 * eq208_e2607_d_n20);
        let eq208_e2608_d_n21: f64 = (p.p7 * eq208_e2607_d_n21);
        let eq208_e2608_d_n22: f64 = (p.p7 * eq208_e2607_d_n22);
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
        let eq208_e2610_d_n10: f64 = (eq208_e2608_d_n10 * p.p249);
        let eq208_e2610_d_n11: f64 = (eq208_e2608_d_n11 * p.p249);
        let eq208_e2610_d_n12: f64 = (eq208_e2608_d_n12 * p.p249);
        let eq208_e2610_d_n13: f64 = (eq208_e2608_d_n13 * p.p249);
        let eq208_e2610_d_n14: f64 = (eq208_e2608_d_n14 * p.p249);
        let eq208_e2610_d_n15: f64 = (eq208_e2608_d_n15 * p.p249);
        let eq208_e2610_d_n16: f64 = (eq208_e2608_d_n16 * p.p249);
        let eq208_e2610_d_n17: f64 = (eq208_e2608_d_n17 * p.p249);
        let eq208_e2610_d_n18: f64 = (eq208_e2608_d_n18 * p.p249);
        let eq208_e2610_d_n19: f64 = (eq208_e2608_d_n19 * p.p249);
        let eq208_e2610_d_n20: f64 = (eq208_e2608_d_n20 * p.p249);
        let eq208_e2610_d_n21: f64 = (eq208_e2608_d_n21 * p.p249);
        let eq208_e2610_d_n22: f64 = (eq208_e2608_d_n22 * p.p249);
        (eq208_e2610, eq208_e2610_d_n0, eq208_e2610_d_n1, eq208_e2610_d_n2, eq208_e2610_d_n3, eq208_e2610_d_n4, eq208_e2610_d_n5, eq208_e2610_d_n6, eq208_e2610_d_n7, eq208_e2610_d_n8, eq208_e2610_d_n9, eq208_e2610_d_n10, eq208_e2610_d_n11, eq208_e2610_d_n12, eq208_e2610_d_n13, eq208_e2610_d_n14, eq208_e2610_d_n15, eq208_e2610_d_n16, eq208_e2610_d_n17, eq208_e2610_d_n18, eq208_e2610_d_n19, eq208_e2610_d_n20, eq208_e2610_d_n21, eq208_e2610_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq208_value: f64 = eq208_e2612;
        let eq208_node_derivatives: [f64; 23] = [eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22];
        let eq208_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            multiplicity * (eq208_value),
            nodes,
            &eq208_node_derivatives,
            branches,
            &eq208_branch_derivatives,
            multiplicity,
        );
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22,) = {
    if (s.b[605] && s.b[606]) {
        let eq209_e2619: f64 = (p.p254 * s.v[312]);
        let eq209_e2619_d_n0: f64 = (p.p254 * s.dn[312][0]);
        let eq209_e2619_d_n1: f64 = (p.p254 * s.dn[312][1]);
        let eq209_e2619_d_n2: f64 = (p.p254 * s.dn[312][2]);
        let eq209_e2619_d_n3: f64 = (p.p254 * s.dn[312][3]);
        let eq209_e2619_d_n4: f64 = (p.p254 * s.dn[312][4]);
        let eq209_e2619_d_n5: f64 = (p.p254 * s.dn[312][5]);
        let eq209_e2619_d_n6: f64 = (p.p254 * s.dn[312][6]);
        let eq209_e2619_d_n7: f64 = (p.p254 * s.dn[312][7]);
        let eq209_e2619_d_n8: f64 = (p.p254 * s.dn[312][8]);
        let eq209_e2619_d_n9: f64 = (p.p254 * s.dn[312][9]);
        let eq209_e2619_d_n10: f64 = (p.p254 * s.dn[312][10]);
        let eq209_e2619_d_n11: f64 = (p.p254 * s.dn[312][11]);
        let eq209_e2619_d_n12: f64 = (p.p254 * s.dn[312][12]);
        let eq209_e2619_d_n13: f64 = (p.p254 * s.dn[312][13]);
        let eq209_e2619_d_n14: f64 = (p.p254 * s.dn[312][14]);
        let eq209_e2619_d_n15: f64 = (p.p254 * s.dn[312][15]);
        let eq209_e2619_d_n16: f64 = (p.p254 * s.dn[312][16]);
        let eq209_e2619_d_n17: f64 = (p.p254 * s.dn[312][17]);
        let eq209_e2619_d_n18: f64 = (p.p254 * s.dn[312][18]);
        let eq209_e2619_d_n19: f64 = (p.p254 * s.dn[312][19]);
        let eq209_e2619_d_n20: f64 = (p.p254 * s.dn[312][20]);
        let eq209_e2619_d_n21: f64 = (p.p254 * s.dn[312][21]);
        let eq209_e2619_d_n22: f64 = (p.p254 * s.dn[312][22]);
        let eq209_e2620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 108, eq209_e2619);
        let eq209_e2620_d_n0: f64 = (eq209_e2619_d_n0 * ddt_scale);
        let eq209_e2620_d_n1: f64 = (eq209_e2619_d_n1 * ddt_scale);
        let eq209_e2620_d_n2: f64 = (eq209_e2619_d_n2 * ddt_scale);
        let eq209_e2620_d_n3: f64 = (eq209_e2619_d_n3 * ddt_scale);
        let eq209_e2620_d_n4: f64 = (eq209_e2619_d_n4 * ddt_scale);
        let eq209_e2620_d_n5: f64 = (eq209_e2619_d_n5 * ddt_scale);
        let eq209_e2620_d_n6: f64 = (eq209_e2619_d_n6 * ddt_scale);
        let eq209_e2620_d_n7: f64 = (eq209_e2619_d_n7 * ddt_scale);
        let eq209_e2620_d_n8: f64 = (eq209_e2619_d_n8 * ddt_scale);
        let eq209_e2620_d_n9: f64 = (eq209_e2619_d_n9 * ddt_scale);
        let eq209_e2620_d_n10: f64 = (eq209_e2619_d_n10 * ddt_scale);
        let eq209_e2620_d_n11: f64 = (eq209_e2619_d_n11 * ddt_scale);
        let eq209_e2620_d_n12: f64 = (eq209_e2619_d_n12 * ddt_scale);
        let eq209_e2620_d_n13: f64 = (eq209_e2619_d_n13 * ddt_scale);
        let eq209_e2620_d_n14: f64 = (eq209_e2619_d_n14 * ddt_scale);
        let eq209_e2620_d_n15: f64 = (eq209_e2619_d_n15 * ddt_scale);
        let eq209_e2620_d_n16: f64 = (eq209_e2619_d_n16 * ddt_scale);
        let eq209_e2620_d_n17: f64 = (eq209_e2619_d_n17 * ddt_scale);
        let eq209_e2620_d_n18: f64 = (eq209_e2619_d_n18 * ddt_scale);
        let eq209_e2620_d_n19: f64 = (eq209_e2619_d_n19 * ddt_scale);
        let eq209_e2620_d_n20: f64 = (eq209_e2619_d_n20 * ddt_scale);
        let eq209_e2620_d_n21: f64 = (eq209_e2619_d_n21 * ddt_scale);
        let eq209_e2620_d_n22: f64 = (eq209_e2619_d_n22 * ddt_scale);
        let eq209_e2621: f64 = (p.p7 * eq209_e2620);
        let eq209_e2621_d_n0: f64 = (p.p7 * eq209_e2620_d_n0);
        let eq209_e2621_d_n1: f64 = (p.p7 * eq209_e2620_d_n1);
        let eq209_e2621_d_n2: f64 = (p.p7 * eq209_e2620_d_n2);
        let eq209_e2621_d_n3: f64 = (p.p7 * eq209_e2620_d_n3);
        let eq209_e2621_d_n4: f64 = (p.p7 * eq209_e2620_d_n4);
        let eq209_e2621_d_n5: f64 = (p.p7 * eq209_e2620_d_n5);
        let eq209_e2621_d_n6: f64 = (p.p7 * eq209_e2620_d_n6);
        let eq209_e2621_d_n7: f64 = (p.p7 * eq209_e2620_d_n7);
        let eq209_e2621_d_n8: f64 = (p.p7 * eq209_e2620_d_n8);
        let eq209_e2621_d_n9: f64 = (p.p7 * eq209_e2620_d_n9);
        let eq209_e2621_d_n10: f64 = (p.p7 * eq209_e2620_d_n10);
        let eq209_e2621_d_n11: f64 = (p.p7 * eq209_e2620_d_n11);
        let eq209_e2621_d_n12: f64 = (p.p7 * eq209_e2620_d_n12);
        let eq209_e2621_d_n13: f64 = (p.p7 * eq209_e2620_d_n13);
        let eq209_e2621_d_n14: f64 = (p.p7 * eq209_e2620_d_n14);
        let eq209_e2621_d_n15: f64 = (p.p7 * eq209_e2620_d_n15);
        let eq209_e2621_d_n16: f64 = (p.p7 * eq209_e2620_d_n16);
        let eq209_e2621_d_n17: f64 = (p.p7 * eq209_e2620_d_n17);
        let eq209_e2621_d_n18: f64 = (p.p7 * eq209_e2620_d_n18);
        let eq209_e2621_d_n19: f64 = (p.p7 * eq209_e2620_d_n19);
        let eq209_e2621_d_n20: f64 = (p.p7 * eq209_e2620_d_n20);
        let eq209_e2621_d_n21: f64 = (p.p7 * eq209_e2620_d_n21);
        let eq209_e2621_d_n22: f64 = (p.p7 * eq209_e2620_d_n22);
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n10, eq209_e2621_d_n11, eq209_e2621_d_n12, eq209_e2621_d_n13, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_value: f64 = eq209_e2623;
        let eq209_node_derivatives: [f64; 23] = [eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22];
        let eq209_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[22]),
            multiplicity * (eq209_value),
            nodes,
            &eq209_node_derivatives,
            branches,
            &eq209_branch_derivatives,
            multiplicity,
        );
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22,) = {
    if ((!s.b[605]) && s.b[608]) {
        let eq210_e2630: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 109, s.v[313]);
        let eq210_e2630_d_n0: f64 = (s.dn[313][0] * ddt_scale);
        let eq210_e2630_d_n1: f64 = (s.dn[313][1] * ddt_scale);
        let eq210_e2630_d_n2: f64 = (s.dn[313][2] * ddt_scale);
        let eq210_e2630_d_n3: f64 = (s.dn[313][3] * ddt_scale);
        let eq210_e2630_d_n4: f64 = (s.dn[313][4] * ddt_scale);
        let eq210_e2630_d_n5: f64 = (s.dn[313][5] * ddt_scale);
        let eq210_e2630_d_n6: f64 = (s.dn[313][6] * ddt_scale);
        let eq210_e2630_d_n7: f64 = (s.dn[313][7] * ddt_scale);
        let eq210_e2630_d_n8: f64 = (s.dn[313][8] * ddt_scale);
        let eq210_e2630_d_n9: f64 = (s.dn[313][9] * ddt_scale);
        let eq210_e2630_d_n10: f64 = (s.dn[313][10] * ddt_scale);
        let eq210_e2630_d_n11: f64 = (s.dn[313][11] * ddt_scale);
        let eq210_e2630_d_n12: f64 = (s.dn[313][12] * ddt_scale);
        let eq210_e2630_d_n13: f64 = (s.dn[313][13] * ddt_scale);
        let eq210_e2630_d_n14: f64 = (s.dn[313][14] * ddt_scale);
        let eq210_e2630_d_n15: f64 = (s.dn[313][15] * ddt_scale);
        let eq210_e2630_d_n16: f64 = (s.dn[313][16] * ddt_scale);
        let eq210_e2630_d_n17: f64 = (s.dn[313][17] * ddt_scale);
        let eq210_e2630_d_n18: f64 = (s.dn[313][18] * ddt_scale);
        let eq210_e2630_d_n19: f64 = (s.dn[313][19] * ddt_scale);
        let eq210_e2630_d_n20: f64 = (s.dn[313][20] * ddt_scale);
        let eq210_e2630_d_n21: f64 = (s.dn[313][21] * ddt_scale);
        let eq210_e2630_d_n22: f64 = (s.dn[313][22] * ddt_scale);
        let eq210_e2631: f64 = (p.p7 * eq210_e2630);
        let eq210_e2631_d_n0: f64 = (p.p7 * eq210_e2630_d_n0);
        let eq210_e2631_d_n1: f64 = (p.p7 * eq210_e2630_d_n1);
        let eq210_e2631_d_n2: f64 = (p.p7 * eq210_e2630_d_n2);
        let eq210_e2631_d_n3: f64 = (p.p7 * eq210_e2630_d_n3);
        let eq210_e2631_d_n4: f64 = (p.p7 * eq210_e2630_d_n4);
        let eq210_e2631_d_n5: f64 = (p.p7 * eq210_e2630_d_n5);
        let eq210_e2631_d_n6: f64 = (p.p7 * eq210_e2630_d_n6);
        let eq210_e2631_d_n7: f64 = (p.p7 * eq210_e2630_d_n7);
        let eq210_e2631_d_n8: f64 = (p.p7 * eq210_e2630_d_n8);
        let eq210_e2631_d_n9: f64 = (p.p7 * eq210_e2630_d_n9);
        let eq210_e2631_d_n10: f64 = (p.p7 * eq210_e2630_d_n10);
        let eq210_e2631_d_n11: f64 = (p.p7 * eq210_e2630_d_n11);
        let eq210_e2631_d_n12: f64 = (p.p7 * eq210_e2630_d_n12);
        let eq210_e2631_d_n13: f64 = (p.p7 * eq210_e2630_d_n13);
        let eq210_e2631_d_n14: f64 = (p.p7 * eq210_e2630_d_n14);
        let eq210_e2631_d_n15: f64 = (p.p7 * eq210_e2630_d_n15);
        let eq210_e2631_d_n16: f64 = (p.p7 * eq210_e2630_d_n16);
        let eq210_e2631_d_n17: f64 = (p.p7 * eq210_e2630_d_n17);
        let eq210_e2631_d_n18: f64 = (p.p7 * eq210_e2630_d_n18);
        let eq210_e2631_d_n19: f64 = (p.p7 * eq210_e2630_d_n19);
        let eq210_e2631_d_n20: f64 = (p.p7 * eq210_e2630_d_n20);
        let eq210_e2631_d_n21: f64 = (p.p7 * eq210_e2630_d_n21);
        let eq210_e2631_d_n22: f64 = (p.p7 * eq210_e2630_d_n22);
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n10, eq210_e2631_d_n11, eq210_e2631_d_n12, eq210_e2631_d_n13, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_value: f64 = eq210_e2633;
        let eq210_node_derivatives: [f64; 23] = [eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22];
        let eq210_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            multiplicity * (eq210_value),
            nodes,
            &eq210_node_derivatives,
            branches,
            &eq210_branch_derivatives,
            multiplicity,
        );
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && s.b[609]) {
        let eq211_e2642: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 110, s.v[312]);
        let eq211_e2642_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq211_e2642_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq211_e2642_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq211_e2642_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq211_e2642_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq211_e2642_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq211_e2642_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq211_e2642_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq211_e2642_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq211_e2642_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq211_e2642_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq211_e2642_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq211_e2642_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq211_e2642_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq211_e2642_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq211_e2642_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq211_e2642_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq211_e2642_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq211_e2642_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq211_e2642_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq211_e2642_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq211_e2642_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq211_e2642_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq211_e2643: f64 = (p.p7 * eq211_e2642);
        let eq211_e2643_d_n0: f64 = (p.p7 * eq211_e2642_d_n0);
        let eq211_e2643_d_n1: f64 = (p.p7 * eq211_e2642_d_n1);
        let eq211_e2643_d_n2: f64 = (p.p7 * eq211_e2642_d_n2);
        let eq211_e2643_d_n3: f64 = (p.p7 * eq211_e2642_d_n3);
        let eq211_e2643_d_n4: f64 = (p.p7 * eq211_e2642_d_n4);
        let eq211_e2643_d_n5: f64 = (p.p7 * eq211_e2642_d_n5);
        let eq211_e2643_d_n6: f64 = (p.p7 * eq211_e2642_d_n6);
        let eq211_e2643_d_n7: f64 = (p.p7 * eq211_e2642_d_n7);
        let eq211_e2643_d_n8: f64 = (p.p7 * eq211_e2642_d_n8);
        let eq211_e2643_d_n9: f64 = (p.p7 * eq211_e2642_d_n9);
        let eq211_e2643_d_n10: f64 = (p.p7 * eq211_e2642_d_n10);
        let eq211_e2643_d_n11: f64 = (p.p7 * eq211_e2642_d_n11);
        let eq211_e2643_d_n12: f64 = (p.p7 * eq211_e2642_d_n12);
        let eq211_e2643_d_n13: f64 = (p.p7 * eq211_e2642_d_n13);
        let eq211_e2643_d_n14: f64 = (p.p7 * eq211_e2642_d_n14);
        let eq211_e2643_d_n15: f64 = (p.p7 * eq211_e2642_d_n15);
        let eq211_e2643_d_n16: f64 = (p.p7 * eq211_e2642_d_n16);
        let eq211_e2643_d_n17: f64 = (p.p7 * eq211_e2642_d_n17);
        let eq211_e2643_d_n18: f64 = (p.p7 * eq211_e2642_d_n18);
        let eq211_e2643_d_n19: f64 = (p.p7 * eq211_e2642_d_n19);
        let eq211_e2643_d_n20: f64 = (p.p7 * eq211_e2642_d_n20);
        let eq211_e2643_d_n21: f64 = (p.p7 * eq211_e2642_d_n21);
        let eq211_e2643_d_n22: f64 = (p.p7 * eq211_e2642_d_n22);
        (eq211_e2643, eq211_e2643_d_n0, eq211_e2643_d_n1, eq211_e2643_d_n2, eq211_e2643_d_n3, eq211_e2643_d_n4, eq211_e2643_d_n5, eq211_e2643_d_n6, eq211_e2643_d_n7, eq211_e2643_d_n8, eq211_e2643_d_n9, eq211_e2643_d_n10, eq211_e2643_d_n11, eq211_e2643_d_n12, eq211_e2643_d_n13, eq211_e2643_d_n14, eq211_e2643_d_n15, eq211_e2643_d_n16, eq211_e2643_d_n17, eq211_e2643_d_n18, eq211_e2643_d_n19, eq211_e2643_d_n20, eq211_e2643_d_n21, eq211_e2643_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_value: f64 = eq211_e2645;
        let eq211_node_derivatives: [f64; 23] = [eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22];
        let eq211_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            multiplicity * (eq211_value),
            nodes,
            &eq211_node_derivatives,
            branches,
            &eq211_branch_derivatives,
            multiplicity,
        );
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && s.b[609]) {
        let eq212_e2654: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 111, s.v[312]);
        let eq212_e2654_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq212_e2654_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq212_e2654_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq212_e2654_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq212_e2654_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq212_e2654_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq212_e2654_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq212_e2654_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq212_e2654_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq212_e2654_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq212_e2654_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq212_e2654_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq212_e2654_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq212_e2654_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq212_e2654_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq212_e2654_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq212_e2654_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq212_e2654_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq212_e2654_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq212_e2654_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq212_e2654_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq212_e2654_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq212_e2654_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq212_e2655: f64 = (p.p7 * eq212_e2654);
        let eq212_e2655_d_n0: f64 = (p.p7 * eq212_e2654_d_n0);
        let eq212_e2655_d_n1: f64 = (p.p7 * eq212_e2654_d_n1);
        let eq212_e2655_d_n2: f64 = (p.p7 * eq212_e2654_d_n2);
        let eq212_e2655_d_n3: f64 = (p.p7 * eq212_e2654_d_n3);
        let eq212_e2655_d_n4: f64 = (p.p7 * eq212_e2654_d_n4);
        let eq212_e2655_d_n5: f64 = (p.p7 * eq212_e2654_d_n5);
        let eq212_e2655_d_n6: f64 = (p.p7 * eq212_e2654_d_n6);
        let eq212_e2655_d_n7: f64 = (p.p7 * eq212_e2654_d_n7);
        let eq212_e2655_d_n8: f64 = (p.p7 * eq212_e2654_d_n8);
        let eq212_e2655_d_n9: f64 = (p.p7 * eq212_e2654_d_n9);
        let eq212_e2655_d_n10: f64 = (p.p7 * eq212_e2654_d_n10);
        let eq212_e2655_d_n11: f64 = (p.p7 * eq212_e2654_d_n11);
        let eq212_e2655_d_n12: f64 = (p.p7 * eq212_e2654_d_n12);
        let eq212_e2655_d_n13: f64 = (p.p7 * eq212_e2654_d_n13);
        let eq212_e2655_d_n14: f64 = (p.p7 * eq212_e2654_d_n14);
        let eq212_e2655_d_n15: f64 = (p.p7 * eq212_e2654_d_n15);
        let eq212_e2655_d_n16: f64 = (p.p7 * eq212_e2654_d_n16);
        let eq212_e2655_d_n17: f64 = (p.p7 * eq212_e2654_d_n17);
        let eq212_e2655_d_n18: f64 = (p.p7 * eq212_e2654_d_n18);
        let eq212_e2655_d_n19: f64 = (p.p7 * eq212_e2654_d_n19);
        let eq212_e2655_d_n20: f64 = (p.p7 * eq212_e2654_d_n20);
        let eq212_e2655_d_n21: f64 = (p.p7 * eq212_e2654_d_n21);
        let eq212_e2655_d_n22: f64 = (p.p7 * eq212_e2654_d_n22);
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
        let eq212_e2657_d_n10: f64 = (eq212_e2655_d_n10 * p.p249);
        let eq212_e2657_d_n11: f64 = (eq212_e2655_d_n11 * p.p249);
        let eq212_e2657_d_n12: f64 = (eq212_e2655_d_n12 * p.p249);
        let eq212_e2657_d_n13: f64 = (eq212_e2655_d_n13 * p.p249);
        let eq212_e2657_d_n14: f64 = (eq212_e2655_d_n14 * p.p249);
        let eq212_e2657_d_n15: f64 = (eq212_e2655_d_n15 * p.p249);
        let eq212_e2657_d_n16: f64 = (eq212_e2655_d_n16 * p.p249);
        let eq212_e2657_d_n17: f64 = (eq212_e2655_d_n17 * p.p249);
        let eq212_e2657_d_n18: f64 = (eq212_e2655_d_n18 * p.p249);
        let eq212_e2657_d_n19: f64 = (eq212_e2655_d_n19 * p.p249);
        let eq212_e2657_d_n20: f64 = (eq212_e2655_d_n20 * p.p249);
        let eq212_e2657_d_n21: f64 = (eq212_e2655_d_n21 * p.p249);
        let eq212_e2657_d_n22: f64 = (eq212_e2655_d_n22 * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n10, eq212_e2657_d_n11, eq212_e2657_d_n12, eq212_e2657_d_n13, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_value: f64 = eq212_e2659;
        let eq212_node_derivatives: [f64; 23] = [eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22];
        let eq212_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq212_value),
            nodes,
            &eq212_node_derivatives,
            branches,
            &eq212_branch_derivatives,
            multiplicity,
        );
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq213_e2669: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 112, s.v[312]);
        let eq213_e2669_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq213_e2669_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq213_e2669_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq213_e2669_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq213_e2669_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq213_e2669_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq213_e2669_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq213_e2669_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq213_e2669_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq213_e2669_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq213_e2669_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq213_e2669_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq213_e2669_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq213_e2669_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq213_e2669_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq213_e2669_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq213_e2669_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq213_e2669_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq213_e2669_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq213_e2669_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq213_e2669_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq213_e2669_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq213_e2669_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq213_e2670: f64 = (p.p7 * eq213_e2669);
        let eq213_e2670_d_n0: f64 = (p.p7 * eq213_e2669_d_n0);
        let eq213_e2670_d_n1: f64 = (p.p7 * eq213_e2669_d_n1);
        let eq213_e2670_d_n2: f64 = (p.p7 * eq213_e2669_d_n2);
        let eq213_e2670_d_n3: f64 = (p.p7 * eq213_e2669_d_n3);
        let eq213_e2670_d_n4: f64 = (p.p7 * eq213_e2669_d_n4);
        let eq213_e2670_d_n5: f64 = (p.p7 * eq213_e2669_d_n5);
        let eq213_e2670_d_n6: f64 = (p.p7 * eq213_e2669_d_n6);
        let eq213_e2670_d_n7: f64 = (p.p7 * eq213_e2669_d_n7);
        let eq213_e2670_d_n8: f64 = (p.p7 * eq213_e2669_d_n8);
        let eq213_e2670_d_n9: f64 = (p.p7 * eq213_e2669_d_n9);
        let eq213_e2670_d_n10: f64 = (p.p7 * eq213_e2669_d_n10);
        let eq213_e2670_d_n11: f64 = (p.p7 * eq213_e2669_d_n11);
        let eq213_e2670_d_n12: f64 = (p.p7 * eq213_e2669_d_n12);
        let eq213_e2670_d_n13: f64 = (p.p7 * eq213_e2669_d_n13);
        let eq213_e2670_d_n14: f64 = (p.p7 * eq213_e2669_d_n14);
        let eq213_e2670_d_n15: f64 = (p.p7 * eq213_e2669_d_n15);
        let eq213_e2670_d_n16: f64 = (p.p7 * eq213_e2669_d_n16);
        let eq213_e2670_d_n17: f64 = (p.p7 * eq213_e2669_d_n17);
        let eq213_e2670_d_n18: f64 = (p.p7 * eq213_e2669_d_n18);
        let eq213_e2670_d_n19: f64 = (p.p7 * eq213_e2669_d_n19);
        let eq213_e2670_d_n20: f64 = (p.p7 * eq213_e2669_d_n20);
        let eq213_e2670_d_n21: f64 = (p.p7 * eq213_e2669_d_n21);
        let eq213_e2670_d_n22: f64 = (p.p7 * eq213_e2669_d_n22);
        (eq213_e2670, eq213_e2670_d_n0, eq213_e2670_d_n1, eq213_e2670_d_n2, eq213_e2670_d_n3, eq213_e2670_d_n4, eq213_e2670_d_n5, eq213_e2670_d_n6, eq213_e2670_d_n7, eq213_e2670_d_n8, eq213_e2670_d_n9, eq213_e2670_d_n10, eq213_e2670_d_n11, eq213_e2670_d_n12, eq213_e2670_d_n13, eq213_e2670_d_n14, eq213_e2670_d_n15, eq213_e2670_d_n16, eq213_e2670_d_n17, eq213_e2670_d_n18, eq213_e2670_d_n19, eq213_e2670_d_n20, eq213_e2670_d_n21, eq213_e2670_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_value: f64 = eq213_e2672;
        let eq213_node_derivatives: [f64; 23] = [eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22];
        let eq213_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq213_value),
            nodes,
            &eq213_node_derivatives,
            branches,
            &eq213_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_25(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq214_e2682: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 113, s.v[312]);
        let eq214_e2682_d_n0: f64 = (s.dn[312][0] * ddt_scale);
        let eq214_e2682_d_n1: f64 = (s.dn[312][1] * ddt_scale);
        let eq214_e2682_d_n2: f64 = (s.dn[312][2] * ddt_scale);
        let eq214_e2682_d_n3: f64 = (s.dn[312][3] * ddt_scale);
        let eq214_e2682_d_n4: f64 = (s.dn[312][4] * ddt_scale);
        let eq214_e2682_d_n5: f64 = (s.dn[312][5] * ddt_scale);
        let eq214_e2682_d_n6: f64 = (s.dn[312][6] * ddt_scale);
        let eq214_e2682_d_n7: f64 = (s.dn[312][7] * ddt_scale);
        let eq214_e2682_d_n8: f64 = (s.dn[312][8] * ddt_scale);
        let eq214_e2682_d_n9: f64 = (s.dn[312][9] * ddt_scale);
        let eq214_e2682_d_n10: f64 = (s.dn[312][10] * ddt_scale);
        let eq214_e2682_d_n11: f64 = (s.dn[312][11] * ddt_scale);
        let eq214_e2682_d_n12: f64 = (s.dn[312][12] * ddt_scale);
        let eq214_e2682_d_n13: f64 = (s.dn[312][13] * ddt_scale);
        let eq214_e2682_d_n14: f64 = (s.dn[312][14] * ddt_scale);
        let eq214_e2682_d_n15: f64 = (s.dn[312][15] * ddt_scale);
        let eq214_e2682_d_n16: f64 = (s.dn[312][16] * ddt_scale);
        let eq214_e2682_d_n17: f64 = (s.dn[312][17] * ddt_scale);
        let eq214_e2682_d_n18: f64 = (s.dn[312][18] * ddt_scale);
        let eq214_e2682_d_n19: f64 = (s.dn[312][19] * ddt_scale);
        let eq214_e2682_d_n20: f64 = (s.dn[312][20] * ddt_scale);
        let eq214_e2682_d_n21: f64 = (s.dn[312][21] * ddt_scale);
        let eq214_e2682_d_n22: f64 = (s.dn[312][22] * ddt_scale);
        let eq214_e2683: f64 = (p.p7 * eq214_e2682);
        let eq214_e2683_d_n0: f64 = (p.p7 * eq214_e2682_d_n0);
        let eq214_e2683_d_n1: f64 = (p.p7 * eq214_e2682_d_n1);
        let eq214_e2683_d_n2: f64 = (p.p7 * eq214_e2682_d_n2);
        let eq214_e2683_d_n3: f64 = (p.p7 * eq214_e2682_d_n3);
        let eq214_e2683_d_n4: f64 = (p.p7 * eq214_e2682_d_n4);
        let eq214_e2683_d_n5: f64 = (p.p7 * eq214_e2682_d_n5);
        let eq214_e2683_d_n6: f64 = (p.p7 * eq214_e2682_d_n6);
        let eq214_e2683_d_n7: f64 = (p.p7 * eq214_e2682_d_n7);
        let eq214_e2683_d_n8: f64 = (p.p7 * eq214_e2682_d_n8);
        let eq214_e2683_d_n9: f64 = (p.p7 * eq214_e2682_d_n9);
        let eq214_e2683_d_n10: f64 = (p.p7 * eq214_e2682_d_n10);
        let eq214_e2683_d_n11: f64 = (p.p7 * eq214_e2682_d_n11);
        let eq214_e2683_d_n12: f64 = (p.p7 * eq214_e2682_d_n12);
        let eq214_e2683_d_n13: f64 = (p.p7 * eq214_e2682_d_n13);
        let eq214_e2683_d_n14: f64 = (p.p7 * eq214_e2682_d_n14);
        let eq214_e2683_d_n15: f64 = (p.p7 * eq214_e2682_d_n15);
        let eq214_e2683_d_n16: f64 = (p.p7 * eq214_e2682_d_n16);
        let eq214_e2683_d_n17: f64 = (p.p7 * eq214_e2682_d_n17);
        let eq214_e2683_d_n18: f64 = (p.p7 * eq214_e2682_d_n18);
        let eq214_e2683_d_n19: f64 = (p.p7 * eq214_e2682_d_n19);
        let eq214_e2683_d_n20: f64 = (p.p7 * eq214_e2682_d_n20);
        let eq214_e2683_d_n21: f64 = (p.p7 * eq214_e2682_d_n21);
        let eq214_e2683_d_n22: f64 = (p.p7 * eq214_e2682_d_n22);
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
        let eq214_e2685_d_n10: f64 = (eq214_e2683_d_n10 * p.p249);
        let eq214_e2685_d_n11: f64 = (eq214_e2683_d_n11 * p.p249);
        let eq214_e2685_d_n12: f64 = (eq214_e2683_d_n12 * p.p249);
        let eq214_e2685_d_n13: f64 = (eq214_e2683_d_n13 * p.p249);
        let eq214_e2685_d_n14: f64 = (eq214_e2683_d_n14 * p.p249);
        let eq214_e2685_d_n15: f64 = (eq214_e2683_d_n15 * p.p249);
        let eq214_e2685_d_n16: f64 = (eq214_e2683_d_n16 * p.p249);
        let eq214_e2685_d_n17: f64 = (eq214_e2683_d_n17 * p.p249);
        let eq214_e2685_d_n18: f64 = (eq214_e2683_d_n18 * p.p249);
        let eq214_e2685_d_n19: f64 = (eq214_e2683_d_n19 * p.p249);
        let eq214_e2685_d_n20: f64 = (eq214_e2683_d_n20 * p.p249);
        let eq214_e2685_d_n21: f64 = (eq214_e2683_d_n21 * p.p249);
        let eq214_e2685_d_n22: f64 = (eq214_e2683_d_n22 * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n10, eq214_e2685_d_n11, eq214_e2685_d_n12, eq214_e2685_d_n13, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_value: f64 = eq214_e2687;
        let eq214_node_derivatives: [f64; 23] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq214_value),
            nodes,
            &eq214_node_derivatives,
            branches,
            &eq214_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22,) = {
    if ((!s.b[605]) && s.b[608]) {
        let eq215_e2695: f64 = (p.p254 * s.v[312]);
        let eq215_e2695_d_n0: f64 = (p.p254 * s.dn[312][0]);
        let eq215_e2695_d_n1: f64 = (p.p254 * s.dn[312][1]);
        let eq215_e2695_d_n2: f64 = (p.p254 * s.dn[312][2]);
        let eq215_e2695_d_n3: f64 = (p.p254 * s.dn[312][3]);
        let eq215_e2695_d_n4: f64 = (p.p254 * s.dn[312][4]);
        let eq215_e2695_d_n5: f64 = (p.p254 * s.dn[312][5]);
        let eq215_e2695_d_n6: f64 = (p.p254 * s.dn[312][6]);
        let eq215_e2695_d_n7: f64 = (p.p254 * s.dn[312][7]);
        let eq215_e2695_d_n8: f64 = (p.p254 * s.dn[312][8]);
        let eq215_e2695_d_n9: f64 = (p.p254 * s.dn[312][9]);
        let eq215_e2695_d_n10: f64 = (p.p254 * s.dn[312][10]);
        let eq215_e2695_d_n11: f64 = (p.p254 * s.dn[312][11]);
        let eq215_e2695_d_n12: f64 = (p.p254 * s.dn[312][12]);
        let eq215_e2695_d_n13: f64 = (p.p254 * s.dn[312][13]);
        let eq215_e2695_d_n14: f64 = (p.p254 * s.dn[312][14]);
        let eq215_e2695_d_n15: f64 = (p.p254 * s.dn[312][15]);
        let eq215_e2695_d_n16: f64 = (p.p254 * s.dn[312][16]);
        let eq215_e2695_d_n17: f64 = (p.p254 * s.dn[312][17]);
        let eq215_e2695_d_n18: f64 = (p.p254 * s.dn[312][18]);
        let eq215_e2695_d_n19: f64 = (p.p254 * s.dn[312][19]);
        let eq215_e2695_d_n20: f64 = (p.p254 * s.dn[312][20]);
        let eq215_e2695_d_n21: f64 = (p.p254 * s.dn[312][21]);
        let eq215_e2695_d_n22: f64 = (p.p254 * s.dn[312][22]);
        let eq215_e2696: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 114, eq215_e2695);
        let eq215_e2696_d_n0: f64 = (eq215_e2695_d_n0 * ddt_scale);
        let eq215_e2696_d_n1: f64 = (eq215_e2695_d_n1 * ddt_scale);
        let eq215_e2696_d_n2: f64 = (eq215_e2695_d_n2 * ddt_scale);
        let eq215_e2696_d_n3: f64 = (eq215_e2695_d_n3 * ddt_scale);
        let eq215_e2696_d_n4: f64 = (eq215_e2695_d_n4 * ddt_scale);
        let eq215_e2696_d_n5: f64 = (eq215_e2695_d_n5 * ddt_scale);
        let eq215_e2696_d_n6: f64 = (eq215_e2695_d_n6 * ddt_scale);
        let eq215_e2696_d_n7: f64 = (eq215_e2695_d_n7 * ddt_scale);
        let eq215_e2696_d_n8: f64 = (eq215_e2695_d_n8 * ddt_scale);
        let eq215_e2696_d_n9: f64 = (eq215_e2695_d_n9 * ddt_scale);
        let eq215_e2696_d_n10: f64 = (eq215_e2695_d_n10 * ddt_scale);
        let eq215_e2696_d_n11: f64 = (eq215_e2695_d_n11 * ddt_scale);
        let eq215_e2696_d_n12: f64 = (eq215_e2695_d_n12 * ddt_scale);
        let eq215_e2696_d_n13: f64 = (eq215_e2695_d_n13 * ddt_scale);
        let eq215_e2696_d_n14: f64 = (eq215_e2695_d_n14 * ddt_scale);
        let eq215_e2696_d_n15: f64 = (eq215_e2695_d_n15 * ddt_scale);
        let eq215_e2696_d_n16: f64 = (eq215_e2695_d_n16 * ddt_scale);
        let eq215_e2696_d_n17: f64 = (eq215_e2695_d_n17 * ddt_scale);
        let eq215_e2696_d_n18: f64 = (eq215_e2695_d_n18 * ddt_scale);
        let eq215_e2696_d_n19: f64 = (eq215_e2695_d_n19 * ddt_scale);
        let eq215_e2696_d_n20: f64 = (eq215_e2695_d_n20 * ddt_scale);
        let eq215_e2696_d_n21: f64 = (eq215_e2695_d_n21 * ddt_scale);
        let eq215_e2696_d_n22: f64 = (eq215_e2695_d_n22 * ddt_scale);
        let eq215_e2697: f64 = (p.p7 * eq215_e2696);
        let eq215_e2697_d_n0: f64 = (p.p7 * eq215_e2696_d_n0);
        let eq215_e2697_d_n1: f64 = (p.p7 * eq215_e2696_d_n1);
        let eq215_e2697_d_n2: f64 = (p.p7 * eq215_e2696_d_n2);
        let eq215_e2697_d_n3: f64 = (p.p7 * eq215_e2696_d_n3);
        let eq215_e2697_d_n4: f64 = (p.p7 * eq215_e2696_d_n4);
        let eq215_e2697_d_n5: f64 = (p.p7 * eq215_e2696_d_n5);
        let eq215_e2697_d_n6: f64 = (p.p7 * eq215_e2696_d_n6);
        let eq215_e2697_d_n7: f64 = (p.p7 * eq215_e2696_d_n7);
        let eq215_e2697_d_n8: f64 = (p.p7 * eq215_e2696_d_n8);
        let eq215_e2697_d_n9: f64 = (p.p7 * eq215_e2696_d_n9);
        let eq215_e2697_d_n10: f64 = (p.p7 * eq215_e2696_d_n10);
        let eq215_e2697_d_n11: f64 = (p.p7 * eq215_e2696_d_n11);
        let eq215_e2697_d_n12: f64 = (p.p7 * eq215_e2696_d_n12);
        let eq215_e2697_d_n13: f64 = (p.p7 * eq215_e2696_d_n13);
        let eq215_e2697_d_n14: f64 = (p.p7 * eq215_e2696_d_n14);
        let eq215_e2697_d_n15: f64 = (p.p7 * eq215_e2696_d_n15);
        let eq215_e2697_d_n16: f64 = (p.p7 * eq215_e2696_d_n16);
        let eq215_e2697_d_n17: f64 = (p.p7 * eq215_e2696_d_n17);
        let eq215_e2697_d_n18: f64 = (p.p7 * eq215_e2696_d_n18);
        let eq215_e2697_d_n19: f64 = (p.p7 * eq215_e2696_d_n19);
        let eq215_e2697_d_n20: f64 = (p.p7 * eq215_e2696_d_n20);
        let eq215_e2697_d_n21: f64 = (p.p7 * eq215_e2696_d_n21);
        let eq215_e2697_d_n22: f64 = (p.p7 * eq215_e2696_d_n22);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n10, eq215_e2697_d_n11, eq215_e2697_d_n12, eq215_e2697_d_n13, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_value: f64 = eq215_e2699;
        let eq215_node_derivatives: [f64; 23] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            multiplicity * (eq215_value),
            nodes,
            &eq215_node_derivatives,
            branches,
            &eq215_branch_derivatives,
            multiplicity,
        );
        let eq216_e2702: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 115, s.v[195]);
        let eq216_e2702_d_n0: f64 = (s.dn[195][0] * ddt_scale);
        let eq216_e2702_d_n1: f64 = (s.dn[195][1] * ddt_scale);
        let eq216_e2702_d_n2: f64 = (s.dn[195][2] * ddt_scale);
        let eq216_e2702_d_n3: f64 = (s.dn[195][3] * ddt_scale);
        let eq216_e2702_d_n4: f64 = (s.dn[195][4] * ddt_scale);
        let eq216_e2702_d_n5: f64 = (s.dn[195][5] * ddt_scale);
        let eq216_e2702_d_n6: f64 = (s.dn[195][6] * ddt_scale);
        let eq216_e2702_d_n7: f64 = (s.dn[195][7] * ddt_scale);
        let eq216_e2702_d_n8: f64 = (s.dn[195][8] * ddt_scale);
        let eq216_e2702_d_n9: f64 = (s.dn[195][9] * ddt_scale);
        let eq216_e2702_d_n10: f64 = (s.dn[195][10] * ddt_scale);
        let eq216_e2702_d_n11: f64 = (s.dn[195][11] * ddt_scale);
        let eq216_e2702_d_n12: f64 = (s.dn[195][12] * ddt_scale);
        let eq216_e2702_d_n13: f64 = (s.dn[195][13] * ddt_scale);
        let eq216_e2702_d_n14: f64 = (s.dn[195][14] * ddt_scale);
        let eq216_e2702_d_n15: f64 = (s.dn[195][15] * ddt_scale);
        let eq216_e2702_d_n16: f64 = (s.dn[195][16] * ddt_scale);
        let eq216_e2702_d_n17: f64 = (s.dn[195][17] * ddt_scale);
        let eq216_e2702_d_n18: f64 = (s.dn[195][18] * ddt_scale);
        let eq216_e2702_d_n19: f64 = (s.dn[195][19] * ddt_scale);
        let eq216_e2702_d_n20: f64 = (s.dn[195][20] * ddt_scale);
        let eq216_e2702_d_n21: f64 = (s.dn[195][21] * ddt_scale);
        let eq216_e2702_d_n22: f64 = (s.dn[195][22] * ddt_scale);
        let eq216_e2703: f64 = (p.p7 * eq216_e2702);
        let eq216_e2703_d_n0: f64 = (p.p7 * eq216_e2702_d_n0);
        let eq216_e2703_d_n1: f64 = (p.p7 * eq216_e2702_d_n1);
        let eq216_e2703_d_n2: f64 = (p.p7 * eq216_e2702_d_n2);
        let eq216_e2703_d_n3: f64 = (p.p7 * eq216_e2702_d_n3);
        let eq216_e2703_d_n4: f64 = (p.p7 * eq216_e2702_d_n4);
        let eq216_e2703_d_n5: f64 = (p.p7 * eq216_e2702_d_n5);
        let eq216_e2703_d_n6: f64 = (p.p7 * eq216_e2702_d_n6);
        let eq216_e2703_d_n7: f64 = (p.p7 * eq216_e2702_d_n7);
        let eq216_e2703_d_n8: f64 = (p.p7 * eq216_e2702_d_n8);
        let eq216_e2703_d_n9: f64 = (p.p7 * eq216_e2702_d_n9);
        let eq216_e2703_d_n10: f64 = (p.p7 * eq216_e2702_d_n10);
        let eq216_e2703_d_n11: f64 = (p.p7 * eq216_e2702_d_n11);
        let eq216_e2703_d_n12: f64 = (p.p7 * eq216_e2702_d_n12);
        let eq216_e2703_d_n13: f64 = (p.p7 * eq216_e2702_d_n13);
        let eq216_e2703_d_n14: f64 = (p.p7 * eq216_e2702_d_n14);
        let eq216_e2703_d_n15: f64 = (p.p7 * eq216_e2702_d_n15);
        let eq216_e2703_d_n16: f64 = (p.p7 * eq216_e2702_d_n16);
        let eq216_e2703_d_n17: f64 = (p.p7 * eq216_e2702_d_n17);
        let eq216_e2703_d_n18: f64 = (p.p7 * eq216_e2702_d_n18);
        let eq216_e2703_d_n19: f64 = (p.p7 * eq216_e2702_d_n19);
        let eq216_e2703_d_n20: f64 = (p.p7 * eq216_e2702_d_n20);
        let eq216_e2703_d_n21: f64 = (p.p7 * eq216_e2702_d_n21);
        let eq216_e2703_d_n22: f64 = (p.p7 * eq216_e2702_d_n22);
        let eq216_value: f64 = eq216_e2703;
        let eq216_node_derivatives: [f64; 23] = [eq216_e2703_d_n0, eq216_e2703_d_n1, eq216_e2703_d_n2, eq216_e2703_d_n3, eq216_e2703_d_n4, eq216_e2703_d_n5, eq216_e2703_d_n6, eq216_e2703_d_n7, eq216_e2703_d_n8, eq216_e2703_d_n9, eq216_e2703_d_n10, eq216_e2703_d_n11, eq216_e2703_d_n12, eq216_e2703_d_n13, eq216_e2703_d_n14, eq216_e2703_d_n15, eq216_e2703_d_n16, eq216_e2703_d_n17, eq216_e2703_d_n18, eq216_e2703_d_n19, eq216_e2703_d_n20, eq216_e2703_d_n21, eq216_e2703_d_n22];
        let eq216_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            multiplicity * (eq216_value),
            nodes,
            &eq216_node_derivatives,
            branches,
            &eq216_branch_derivatives,
            multiplicity,
        );
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2711_d_n2: f64 = (-eq217_e2709);
        let eq217_e2712: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 116, eq217_e2711);
        let eq217_e2712_d_n1: f64 = (eq217_e2709 * ddt_scale);
        let eq217_e2712_d_n2: f64 = (eq217_e2711_d_n2 * ddt_scale);
        let eq217_e2713: f64 = (p.p7 * eq217_e2712);
        let eq217_e2713_d_n1: f64 = (p.p7 * eq217_e2712_d_n1);
        let eq217_e2713_d_n2: f64 = (p.p7 * eq217_e2712_d_n2);
        let eq217_value: f64 = eq217_e2713;
        stamper.stamp_current_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            multiplicity * (eq217_value),
            nodes[1],
            multiplicity * (eq217_e2713_d_n1),
            nodes[2],
            multiplicity * (eq217_e2713_d_n2),
        );
        let eq218_e2716: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 117, s.v[196]);
        let eq218_e2716_d_n0: f64 = (s.dn[196][0] * ddt_scale);
        let eq218_e2716_d_n1: f64 = (s.dn[196][1] * ddt_scale);
        let eq218_e2716_d_n2: f64 = (s.dn[196][2] * ddt_scale);
        let eq218_e2716_d_n3: f64 = (s.dn[196][3] * ddt_scale);
        let eq218_e2716_d_n4: f64 = (s.dn[196][4] * ddt_scale);
        let eq218_e2716_d_n5: f64 = (s.dn[196][5] * ddt_scale);
        let eq218_e2716_d_n6: f64 = (s.dn[196][6] * ddt_scale);
        let eq218_e2716_d_n7: f64 = (s.dn[196][7] * ddt_scale);
        let eq218_e2716_d_n8: f64 = (s.dn[196][8] * ddt_scale);
        let eq218_e2716_d_n9: f64 = (s.dn[196][9] * ddt_scale);
        let eq218_e2716_d_n10: f64 = (s.dn[196][10] * ddt_scale);
        let eq218_e2716_d_n11: f64 = (s.dn[196][11] * ddt_scale);
        let eq218_e2716_d_n12: f64 = (s.dn[196][12] * ddt_scale);
        let eq218_e2716_d_n13: f64 = (s.dn[196][13] * ddt_scale);
        let eq218_e2716_d_n14: f64 = (s.dn[196][14] * ddt_scale);
        let eq218_e2716_d_n15: f64 = (s.dn[196][15] * ddt_scale);
        let eq218_e2716_d_n16: f64 = (s.dn[196][16] * ddt_scale);
        let eq218_e2716_d_n17: f64 = (s.dn[196][17] * ddt_scale);
        let eq218_e2716_d_n18: f64 = (s.dn[196][18] * ddt_scale);
        let eq218_e2716_d_n19: f64 = (s.dn[196][19] * ddt_scale);
        let eq218_e2716_d_n20: f64 = (s.dn[196][20] * ddt_scale);
        let eq218_e2716_d_n21: f64 = (s.dn[196][21] * ddt_scale);
        let eq218_e2716_d_n22: f64 = (s.dn[196][22] * ddt_scale);
        let eq218_e2717: f64 = (p.p7 * eq218_e2716);
        let eq218_e2717_d_n0: f64 = (p.p7 * eq218_e2716_d_n0);
        let eq218_e2717_d_n1: f64 = (p.p7 * eq218_e2716_d_n1);
        let eq218_e2717_d_n2: f64 = (p.p7 * eq218_e2716_d_n2);
        let eq218_e2717_d_n3: f64 = (p.p7 * eq218_e2716_d_n3);
        let eq218_e2717_d_n4: f64 = (p.p7 * eq218_e2716_d_n4);
        let eq218_e2717_d_n5: f64 = (p.p7 * eq218_e2716_d_n5);
        let eq218_e2717_d_n6: f64 = (p.p7 * eq218_e2716_d_n6);
        let eq218_e2717_d_n7: f64 = (p.p7 * eq218_e2716_d_n7);
        let eq218_e2717_d_n8: f64 = (p.p7 * eq218_e2716_d_n8);
        let eq218_e2717_d_n9: f64 = (p.p7 * eq218_e2716_d_n9);
        let eq218_e2717_d_n10: f64 = (p.p7 * eq218_e2716_d_n10);
        let eq218_e2717_d_n11: f64 = (p.p7 * eq218_e2716_d_n11);
        let eq218_e2717_d_n12: f64 = (p.p7 * eq218_e2716_d_n12);
        let eq218_e2717_d_n13: f64 = (p.p7 * eq218_e2716_d_n13);
        let eq218_e2717_d_n14: f64 = (p.p7 * eq218_e2716_d_n14);
        let eq218_e2717_d_n15: f64 = (p.p7 * eq218_e2716_d_n15);
        let eq218_e2717_d_n16: f64 = (p.p7 * eq218_e2716_d_n16);
        let eq218_e2717_d_n17: f64 = (p.p7 * eq218_e2716_d_n17);
        let eq218_e2717_d_n18: f64 = (p.p7 * eq218_e2716_d_n18);
        let eq218_e2717_d_n19: f64 = (p.p7 * eq218_e2716_d_n19);
        let eq218_e2717_d_n20: f64 = (p.p7 * eq218_e2716_d_n20);
        let eq218_e2717_d_n21: f64 = (p.p7 * eq218_e2716_d_n21);
        let eq218_e2717_d_n22: f64 = (p.p7 * eq218_e2716_d_n22);
        let eq218_value: f64 = eq218_e2717;
        let eq218_node_derivatives: [f64; 23] = [eq218_e2717_d_n0, eq218_e2717_d_n1, eq218_e2717_d_n2, eq218_e2717_d_n3, eq218_e2717_d_n4, eq218_e2717_d_n5, eq218_e2717_d_n6, eq218_e2717_d_n7, eq218_e2717_d_n8, eq218_e2717_d_n9, eq218_e2717_d_n10, eq218_e2717_d_n11, eq218_e2717_d_n12, eq218_e2717_d_n13, eq218_e2717_d_n14, eq218_e2717_d_n15, eq218_e2717_d_n16, eq218_e2717_d_n17, eq218_e2717_d_n18, eq218_e2717_d_n19, eq218_e2717_d_n20, eq218_e2717_d_n21, eq218_e2717_d_n22];
        let eq218_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            multiplicity * (eq218_value),
            nodes,
            &eq218_node_derivatives,
            branches,
            &eq218_branch_derivatives,
            multiplicity,
        );
        let eq219_e2720: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 118, s.v[197]);
        let eq219_e2720_d_n0: f64 = (s.dn[197][0] * ddt_scale);
        let eq219_e2720_d_n1: f64 = (s.dn[197][1] * ddt_scale);
        let eq219_e2720_d_n2: f64 = (s.dn[197][2] * ddt_scale);
        let eq219_e2720_d_n3: f64 = (s.dn[197][3] * ddt_scale);
        let eq219_e2720_d_n4: f64 = (s.dn[197][4] * ddt_scale);
        let eq219_e2720_d_n5: f64 = (s.dn[197][5] * ddt_scale);
        let eq219_e2720_d_n6: f64 = (s.dn[197][6] * ddt_scale);
        let eq219_e2720_d_n7: f64 = (s.dn[197][7] * ddt_scale);
        let eq219_e2720_d_n8: f64 = (s.dn[197][8] * ddt_scale);
        let eq219_e2720_d_n9: f64 = (s.dn[197][9] * ddt_scale);
        let eq219_e2720_d_n10: f64 = (s.dn[197][10] * ddt_scale);
        let eq219_e2720_d_n11: f64 = (s.dn[197][11] * ddt_scale);
        let eq219_e2720_d_n12: f64 = (s.dn[197][12] * ddt_scale);
        let eq219_e2720_d_n13: f64 = (s.dn[197][13] * ddt_scale);
        let eq219_e2720_d_n14: f64 = (s.dn[197][14] * ddt_scale);
        let eq219_e2720_d_n15: f64 = (s.dn[197][15] * ddt_scale);
        let eq219_e2720_d_n16: f64 = (s.dn[197][16] * ddt_scale);
        let eq219_e2720_d_n17: f64 = (s.dn[197][17] * ddt_scale);
        let eq219_e2720_d_n18: f64 = (s.dn[197][18] * ddt_scale);
        let eq219_e2720_d_n19: f64 = (s.dn[197][19] * ddt_scale);
        let eq219_e2720_d_n20: f64 = (s.dn[197][20] * ddt_scale);
        let eq219_e2720_d_n21: f64 = (s.dn[197][21] * ddt_scale);
        let eq219_e2720_d_n22: f64 = (s.dn[197][22] * ddt_scale);
        let eq219_e2721: f64 = (p.p7 * eq219_e2720);
        let eq219_e2721_d_n0: f64 = (p.p7 * eq219_e2720_d_n0);
        let eq219_e2721_d_n1: f64 = (p.p7 * eq219_e2720_d_n1);
        let eq219_e2721_d_n2: f64 = (p.p7 * eq219_e2720_d_n2);
        let eq219_e2721_d_n3: f64 = (p.p7 * eq219_e2720_d_n3);
        let eq219_e2721_d_n4: f64 = (p.p7 * eq219_e2720_d_n4);
        let eq219_e2721_d_n5: f64 = (p.p7 * eq219_e2720_d_n5);
        let eq219_e2721_d_n6: f64 = (p.p7 * eq219_e2720_d_n6);
        let eq219_e2721_d_n7: f64 = (p.p7 * eq219_e2720_d_n7);
        let eq219_e2721_d_n8: f64 = (p.p7 * eq219_e2720_d_n8);
        let eq219_e2721_d_n9: f64 = (p.p7 * eq219_e2720_d_n9);
        let eq219_e2721_d_n10: f64 = (p.p7 * eq219_e2720_d_n10);
        let eq219_e2721_d_n11: f64 = (p.p7 * eq219_e2720_d_n11);
        let eq219_e2721_d_n12: f64 = (p.p7 * eq219_e2720_d_n12);
        let eq219_e2721_d_n13: f64 = (p.p7 * eq219_e2720_d_n13);
        let eq219_e2721_d_n14: f64 = (p.p7 * eq219_e2720_d_n14);
        let eq219_e2721_d_n15: f64 = (p.p7 * eq219_e2720_d_n15);
        let eq219_e2721_d_n16: f64 = (p.p7 * eq219_e2720_d_n16);
        let eq219_e2721_d_n17: f64 = (p.p7 * eq219_e2720_d_n17);
        let eq219_e2721_d_n18: f64 = (p.p7 * eq219_e2720_d_n18);
        let eq219_e2721_d_n19: f64 = (p.p7 * eq219_e2720_d_n19);
        let eq219_e2721_d_n20: f64 = (p.p7 * eq219_e2720_d_n20);
        let eq219_e2721_d_n21: f64 = (p.p7 * eq219_e2720_d_n21);
        let eq219_e2721_d_n22: f64 = (p.p7 * eq219_e2720_d_n22);
        let eq219_value: f64 = eq219_e2721;
        let eq219_node_derivatives: [f64; 23] = [eq219_e2721_d_n0, eq219_e2721_d_n1, eq219_e2721_d_n2, eq219_e2721_d_n3, eq219_e2721_d_n4, eq219_e2721_d_n5, eq219_e2721_d_n6, eq219_e2721_d_n7, eq219_e2721_d_n8, eq219_e2721_d_n9, eq219_e2721_d_n10, eq219_e2721_d_n11, eq219_e2721_d_n12, eq219_e2721_d_n13, eq219_e2721_d_n14, eq219_e2721_d_n15, eq219_e2721_d_n16, eq219_e2721_d_n17, eq219_e2721_d_n18, eq219_e2721_d_n19, eq219_e2721_d_n20, eq219_e2721_d_n21, eq219_e2721_d_n22];
        let eq219_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            multiplicity * (eq219_value),
            nodes,
            &eq219_node_derivatives,
            branches,
            &eq219_branch_derivatives,
            multiplicity,
        );
        let eq220_e2724: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 119, s.v[194]);
        let eq220_e2724_d_n0: f64 = (s.dn[194][0] * ddt_scale);
        let eq220_e2724_d_n1: f64 = (s.dn[194][1] * ddt_scale);
        let eq220_e2724_d_n2: f64 = (s.dn[194][2] * ddt_scale);
        let eq220_e2724_d_n3: f64 = (s.dn[194][3] * ddt_scale);
        let eq220_e2724_d_n4: f64 = (s.dn[194][4] * ddt_scale);
        let eq220_e2724_d_n5: f64 = (s.dn[194][5] * ddt_scale);
        let eq220_e2724_d_n6: f64 = (s.dn[194][6] * ddt_scale);
        let eq220_e2724_d_n7: f64 = (s.dn[194][7] * ddt_scale);
        let eq220_e2724_d_n8: f64 = (s.dn[194][8] * ddt_scale);
        let eq220_e2724_d_n9: f64 = (s.dn[194][9] * ddt_scale);
        let eq220_e2724_d_n10: f64 = (s.dn[194][10] * ddt_scale);
        let eq220_e2724_d_n11: f64 = (s.dn[194][11] * ddt_scale);
        let eq220_e2724_d_n12: f64 = (s.dn[194][12] * ddt_scale);
        let eq220_e2724_d_n13: f64 = (s.dn[194][13] * ddt_scale);
        let eq220_e2724_d_n14: f64 = (s.dn[194][14] * ddt_scale);
        let eq220_e2724_d_n15: f64 = (s.dn[194][15] * ddt_scale);
        let eq220_e2724_d_n16: f64 = (s.dn[194][16] * ddt_scale);
        let eq220_e2724_d_n17: f64 = (s.dn[194][17] * ddt_scale);
        let eq220_e2724_d_n18: f64 = (s.dn[194][18] * ddt_scale);
        let eq220_e2724_d_n19: f64 = (s.dn[194][19] * ddt_scale);
        let eq220_e2724_d_n20: f64 = (s.dn[194][20] * ddt_scale);
        let eq220_e2724_d_n21: f64 = (s.dn[194][21] * ddt_scale);
        let eq220_e2724_d_n22: f64 = (s.dn[194][22] * ddt_scale);
        let eq220_e2725: f64 = (p.p7 * eq220_e2724);
        let eq220_e2725_d_n0: f64 = (p.p7 * eq220_e2724_d_n0);
        let eq220_e2725_d_n1: f64 = (p.p7 * eq220_e2724_d_n1);
        let eq220_e2725_d_n2: f64 = (p.p7 * eq220_e2724_d_n2);
        let eq220_e2725_d_n3: f64 = (p.p7 * eq220_e2724_d_n3);
        let eq220_e2725_d_n4: f64 = (p.p7 * eq220_e2724_d_n4);
        let eq220_e2725_d_n5: f64 = (p.p7 * eq220_e2724_d_n5);
        let eq220_e2725_d_n6: f64 = (p.p7 * eq220_e2724_d_n6);
        let eq220_e2725_d_n7: f64 = (p.p7 * eq220_e2724_d_n7);
        let eq220_e2725_d_n8: f64 = (p.p7 * eq220_e2724_d_n8);
        let eq220_e2725_d_n9: f64 = (p.p7 * eq220_e2724_d_n9);
        let eq220_e2725_d_n10: f64 = (p.p7 * eq220_e2724_d_n10);
        let eq220_e2725_d_n11: f64 = (p.p7 * eq220_e2724_d_n11);
        let eq220_e2725_d_n12: f64 = (p.p7 * eq220_e2724_d_n12);
        let eq220_e2725_d_n13: f64 = (p.p7 * eq220_e2724_d_n13);
        let eq220_e2725_d_n14: f64 = (p.p7 * eq220_e2724_d_n14);
        let eq220_e2725_d_n15: f64 = (p.p7 * eq220_e2724_d_n15);
        let eq220_e2725_d_n16: f64 = (p.p7 * eq220_e2724_d_n16);
        let eq220_e2725_d_n17: f64 = (p.p7 * eq220_e2724_d_n17);
        let eq220_e2725_d_n18: f64 = (p.p7 * eq220_e2724_d_n18);
        let eq220_e2725_d_n19: f64 = (p.p7 * eq220_e2724_d_n19);
        let eq220_e2725_d_n20: f64 = (p.p7 * eq220_e2724_d_n20);
        let eq220_e2725_d_n21: f64 = (p.p7 * eq220_e2724_d_n21);
        let eq220_e2725_d_n22: f64 = (p.p7 * eq220_e2724_d_n22);
        let eq220_value: f64 = eq220_e2725;
        let eq220_node_derivatives: [f64; 23] = [eq220_e2725_d_n0, eq220_e2725_d_n1, eq220_e2725_d_n2, eq220_e2725_d_n3, eq220_e2725_d_n4, eq220_e2725_d_n5, eq220_e2725_d_n6, eq220_e2725_d_n7, eq220_e2725_d_n8, eq220_e2725_d_n9, eq220_e2725_d_n10, eq220_e2725_d_n11, eq220_e2725_d_n12, eq220_e2725_d_n13, eq220_e2725_d_n14, eq220_e2725_d_n15, eq220_e2725_d_n16, eq220_e2725_d_n17, eq220_e2725_d_n18, eq220_e2725_d_n19, eq220_e2725_d_n20, eq220_e2725_d_n21, eq220_e2725_d_n22];
        let eq220_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            multiplicity * (eq220_value),
            nodes,
            &eq220_node_derivatives,
            branches,
            &eq220_branch_derivatives,
            multiplicity,
        );
        let eq221_ad: A = {
    if s.b[610] {
        A::sub(A::sub(A::sub(A::sub(A::mul(A::scale(s.ad_value(94), (-1.0)), s.ad_value(38)), A::mul(s.ad_value(233), s.ad_value(231))), A::mul(s.ad_value(257), s.ad_value(255))), A::mul(s.ad_value(281), s.ad_value(279))), A::mul(s.ad_value(305), s.ad_value(303)))
    } else {
        A::constant(0.0)
    }
};
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * eq221_ad.value,
            nodes,
            &eq221_ad.dn,
            branches,
            &eq221_ad.db,
            multiplicity,
        );
        let (eq222_e2764, eq222_e2764_d_n4,) = {
    if s.b[610] {
        let eq222_e2762: f64 = ((nv4 - 0.0) / p.p32);
        let eq222_e2762_d_n4: f64 = (1.0 / p.p32);
        (eq222_e2762, eq222_e2762_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq222_value: f64 = eq222_e2764;
        stamper.stamp_current_node1(
            Some(nodes[4]),
            None,
            multiplicity * (eq222_value),
            nodes[4],
            multiplicity * (eq222_e2764_d_n4),
        );
        let (eq223_e2771, eq223_e2771_d_n4,) = {
    if s.b[610] {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2768_d_n4: f64 = p.p33;
        let eq223_e2769: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 120, eq223_e2768);
        let eq223_e2769_d_n4: f64 = (eq223_e2768_d_n4 * ddt_scale);
        (eq223_e2769, eq223_e2769_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq223_value: f64 = eq223_e2771;
        stamper.stamp_current_node1(
            Some(nodes[4]),
            None,
            multiplicity * (eq223_value),
            nodes[4],
            multiplicity * (eq223_e2771_d_n4),
        );
    }

    pub(super) fn stamp_transient_equations_block_26(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        branches: &[usize; Instance::BRANCH_COUNT],
    ) {
        let (eq224_e2776,) = {
    if (!s.b[610]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq224_value: f64 = eq224_e2776;
        stamper.stamp_potential_const(
            branches[56],
            eq224_value,
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq9_e355, eq9_e355_d_n5, eq9_e355_q, eq9_e355_q_d_n5,) = {
    if (s.b[388] && (!s.b[387])) {
        let eq9_e352_q: f64 = (nv5 - 0.0);
        let eq9_e353: f64 = (p.p97 * (nv5 - 0.0));
        let eq9_e353_d_n5: f64 = p.p97;
        let eq9_e353_q: f64 = (p.p97 * eq9_e352_q);
        let eq9_e353_q_d_n5: f64 = p.p97;
        (eq9_e353, eq9_e353_d_n5, eq9_e353_q, eq9_e353_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq9_e355_q_d_n5),
        );
        let (eq17_e427, eq17_e427_d_n5, eq17_e427_q, eq17_e427_q_d_n5,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq17_e424_q: f64 = (nv5 - 0.0);
        let eq17_e425: f64 = (p.p110 * (nv5 - 0.0));
        let eq17_e425_d_n5: f64 = p.p110;
        let eq17_e425_q: f64 = (p.p110 * eq17_e424_q);
        let eq17_e425_q_d_n5: f64 = p.p110;
        (eq17_e425, eq17_e425_d_n5, eq17_e425_q, eq17_e425_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq17_e427_q_d_n5),
        );
        let (eq20_e462, eq20_e462_d_n6, eq20_e462_q, eq20_e462_q_d_n6,) = {
    if (s.b[389] && (!(s.b[387] || s.b[388]))) {
        let eq20_e459_q: f64 = (nv6 - 0.0);
        let eq20_e460: f64 = (p.p111 * (nv6 - 0.0));
        let eq20_e460_d_n6: f64 = p.p111;
        let eq20_e460_q: f64 = (p.p111 * eq20_e459_q);
        let eq20_e460_q_d_n6: f64 = p.p111;
        (eq20_e460, eq20_e460_d_n6, eq20_e460_q, eq20_e460_q_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (eq20_e462_q_d_n6),
        );
        let (eq27_e539, eq27_e539_d_n0, eq27_e539_d_n1, eq27_e539_d_n2, eq27_e539_d_n3, eq27_e539_d_n4, eq27_e539_d_n5, eq27_e539_d_n6, eq27_e539_d_n7, eq27_e539_d_n8, eq27_e539_d_n9, eq27_e539_d_n10, eq27_e539_d_n11, eq27_e539_d_n12, eq27_e539_d_n13, eq27_e539_d_n14, eq27_e539_d_n15, eq27_e539_d_n16, eq27_e539_d_n17, eq27_e539_d_n18, eq27_e539_d_n19, eq27_e539_d_n20, eq27_e539_d_n21, eq27_e539_d_n22, eq27_e539_q, eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n10, eq27_e539_q_d_n11, eq27_e539_q_d_n12, eq27_e539_q_d_n13, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22,) = {
    if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
        let eq27_e536_q: f64 = (nv5 - 0.0);
        let eq27_e537: f64 = (s.v[149] * (nv5 - 0.0));
        let eq27_e537_d_n0: f64 = (s.dn[149][0] * (nv5 - 0.0));
        let eq27_e537_d_n1: f64 = (s.dn[149][1] * (nv5 - 0.0));
        let eq27_e537_d_n2: f64 = (s.dn[149][2] * (nv5 - 0.0));
        let eq27_e537_d_n3: f64 = (s.dn[149][3] * (nv5 - 0.0));
        let eq27_e537_d_n4: f64 = (s.dn[149][4] * (nv5 - 0.0));
        let eq27_e537_d_n5: f64 = ((s.dn[149][5] * (nv5 - 0.0)) + s.v[149]);
        let eq27_e537_d_n6: f64 = (s.dn[149][6] * (nv5 - 0.0));
        let eq27_e537_d_n7: f64 = (s.dn[149][7] * (nv5 - 0.0));
        let eq27_e537_d_n8: f64 = (s.dn[149][8] * (nv5 - 0.0));
        let eq27_e537_d_n9: f64 = (s.dn[149][9] * (nv5 - 0.0));
        let eq27_e537_d_n10: f64 = (s.dn[149][10] * (nv5 - 0.0));
        let eq27_e537_d_n11: f64 = (s.dn[149][11] * (nv5 - 0.0));
        let eq27_e537_d_n12: f64 = (s.dn[149][12] * (nv5 - 0.0));
        let eq27_e537_d_n13: f64 = (s.dn[149][13] * (nv5 - 0.0));
        let eq27_e537_d_n14: f64 = (s.dn[149][14] * (nv5 - 0.0));
        let eq27_e537_d_n15: f64 = (s.dn[149][15] * (nv5 - 0.0));
        let eq27_e537_d_n16: f64 = (s.dn[149][16] * (nv5 - 0.0));
        let eq27_e537_d_n17: f64 = (s.dn[149][17] * (nv5 - 0.0));
        let eq27_e537_d_n18: f64 = (s.dn[149][18] * (nv5 - 0.0));
        let eq27_e537_d_n19: f64 = (s.dn[149][19] * (nv5 - 0.0));
        let eq27_e537_d_n20: f64 = (s.dn[149][20] * (nv5 - 0.0));
        let eq27_e537_d_n21: f64 = (s.dn[149][21] * (nv5 - 0.0));
        let eq27_e537_d_n22: f64 = (s.dn[149][22] * (nv5 - 0.0));
        let eq27_e537_q: f64 = (s.v[149] * eq27_e536_q);
        let eq27_e537_q_d_n0: f64 = (s.dn[149][0] * eq27_e536_q);
        let eq27_e537_q_d_n1: f64 = (s.dn[149][1] * eq27_e536_q);
        let eq27_e537_q_d_n2: f64 = (s.dn[149][2] * eq27_e536_q);
        let eq27_e537_q_d_n3: f64 = (s.dn[149][3] * eq27_e536_q);
        let eq27_e537_q_d_n4: f64 = (s.dn[149][4] * eq27_e536_q);
        let eq27_e537_q_d_n5: f64 = ((s.dn[149][5] * eq27_e536_q) + s.v[149]);
        let eq27_e537_q_d_n6: f64 = (s.dn[149][6] * eq27_e536_q);
        let eq27_e537_q_d_n7: f64 = (s.dn[149][7] * eq27_e536_q);
        let eq27_e537_q_d_n8: f64 = (s.dn[149][8] * eq27_e536_q);
        let eq27_e537_q_d_n9: f64 = (s.dn[149][9] * eq27_e536_q);
        let eq27_e537_q_d_n10: f64 = (s.dn[149][10] * eq27_e536_q);
        let eq27_e537_q_d_n11: f64 = (s.dn[149][11] * eq27_e536_q);
        let eq27_e537_q_d_n12: f64 = (s.dn[149][12] * eq27_e536_q);
        let eq27_e537_q_d_n13: f64 = (s.dn[149][13] * eq27_e536_q);
        let eq27_e537_q_d_n14: f64 = (s.dn[149][14] * eq27_e536_q);
        let eq27_e537_q_d_n15: f64 = (s.dn[149][15] * eq27_e536_q);
        let eq27_e537_q_d_n16: f64 = (s.dn[149][16] * eq27_e536_q);
        let eq27_e537_q_d_n17: f64 = (s.dn[149][17] * eq27_e536_q);
        let eq27_e537_q_d_n18: f64 = (s.dn[149][18] * eq27_e536_q);
        let eq27_e537_q_d_n19: f64 = (s.dn[149][19] * eq27_e536_q);
        let eq27_e537_q_d_n20: f64 = (s.dn[149][20] * eq27_e536_q);
        let eq27_e537_q_d_n21: f64 = (s.dn[149][21] * eq27_e536_q);
        let eq27_e537_q_d_n22: f64 = (s.dn[149][22] * eq27_e536_q);
        (eq27_e537, eq27_e537_d_n0, eq27_e537_d_n1, eq27_e537_d_n2, eq27_e537_d_n3, eq27_e537_d_n4, eq27_e537_d_n5, eq27_e537_d_n6, eq27_e537_d_n7, eq27_e537_d_n8, eq27_e537_d_n9, eq27_e537_d_n10, eq27_e537_d_n11, eq27_e537_d_n12, eq27_e537_d_n13, eq27_e537_d_n14, eq27_e537_d_n15, eq27_e537_d_n16, eq27_e537_d_n17, eq27_e537_d_n18, eq27_e537_d_n19, eq27_e537_d_n20, eq27_e537_d_n21, eq27_e537_d_n22, eq27_e537_q, eq27_e537_q_d_n0, eq27_e537_q_d_n1, eq27_e537_q_d_n2, eq27_e537_q_d_n3, eq27_e537_q_d_n4, eq27_e537_q_d_n5, eq27_e537_q_d_n6, eq27_e537_q_d_n7, eq27_e537_q_d_n8, eq27_e537_q_d_n9, eq27_e537_q_d_n10, eq27_e537_q_d_n11, eq27_e537_q_d_n12, eq27_e537_q_d_n13, eq27_e537_q_d_n14, eq27_e537_q_d_n15, eq27_e537_q_d_n16, eq27_e537_q_d_n17, eq27_e537_q_d_n18, eq27_e537_q_d_n19, eq27_e537_q_d_n20, eq27_e537_q_d_n21, eq27_e537_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 23] = [eq27_e539_q_d_n0, eq27_e539_q_d_n1, eq27_e539_q_d_n2, eq27_e539_q_d_n3, eq27_e539_q_d_n4, eq27_e539_q_d_n5, eq27_e539_q_d_n6, eq27_e539_q_d_n7, eq27_e539_q_d_n8, eq27_e539_q_d_n9, eq27_e539_q_d_n10, eq27_e539_q_d_n11, eq27_e539_q_d_n12, eq27_e539_q_d_n13, eq27_e539_q_d_n14, eq27_e539_q_d_n15, eq27_e539_q_d_n16, eq27_e539_q_d_n17, eq27_e539_q_d_n18, eq27_e539_q_d_n19, eq27_e539_q_d_n20, eq27_e539_q_d_n21, eq27_e539_q_d_n22];
        let eq27_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq37_e668, eq37_e668_d_n12, eq37_e668_q, eq37_e668_q_d_n12,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq37_e661_q: f64 = (nv12 - 0.0);
        let eq37_e662: f64 = (p.p97 * (nv12 - 0.0));
        let eq37_e662_d_n12: f64 = p.p97;
        let eq37_e662_q: f64 = (p.p97 * eq37_e661_q);
        let eq37_e662_q_d_n12: f64 = p.p97;
        let eq37_e665: f64 = (1e-12 * (nv12 - 0.0));
        let eq37_e665_d_n12: f64 = 1e-12;
        let eq37_e666: f64 = (eq37_e662 + eq37_e665);
        let eq37_e666_d_n12: f64 = (eq37_e662_d_n12 + eq37_e665_d_n12);
        let eq37_e666_q: f64 = eq37_e662_q;
        (eq37_e666, eq37_e666_d_n12, eq37_e666_q, eq37_e662_q_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq37_e668_q_d_n12),
        );
        let (eq40_e716, eq40_e716_d_n14, eq40_e716_q, eq40_e716_q_d_n14,) = {
    if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
        let eq40_e709_q: f64 = (nv14 - 0.0);
        let eq40_e710: f64 = (p.p83 * (nv14 - 0.0));
        let eq40_e710_d_n14: f64 = p.p83;
        let eq40_e710_q: f64 = (p.p83 * eq40_e709_q);
        let eq40_e710_q_d_n14: f64 = p.p83;
        let eq40_e713: f64 = (1e-12 * (nv14 - 0.0));
        let eq40_e713_d_n14: f64 = 1e-12;
        let eq40_e714: f64 = (eq40_e710 + eq40_e713);
        let eq40_e714_d_n14: f64 = (eq40_e710_d_n14 + eq40_e713_d_n14);
        let eq40_e714_q: f64 = eq40_e710_q;
        (eq40_e714, eq40_e714_d_n14, eq40_e714_q, eq40_e710_q_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq40_e716_q_d_n14),
        );
        let (eq43_e784, eq43_e784_d_n5, eq43_e784_q, eq43_e784_q_d_n5,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq43_e781_q: f64 = (nv5 - 0.0);
        let eq43_e782: f64 = (p.p135 * (nv5 - 0.0));
        let eq43_e782_d_n5: f64 = p.p135;
        let eq43_e782_q: f64 = (p.p135 * eq43_e781_q);
        let eq43_e782_q_d_n5: f64 = p.p135;
        (eq43_e782, eq43_e782_d_n5, eq43_e782_q, eq43_e782_q_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[5]),
            None,
            nodes[5],
            multiplicity * (eq43_e784_q_d_n5),
        );
        let (eq46_e852, eq46_e852_d_n6, eq46_e852_q, eq46_e852_q_d_n6,) = {
    if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
        let eq46_e849_q: f64 = (nv6 - 0.0);
        let eq46_e850: f64 = (p.p144 * (nv6 - 0.0));
        let eq46_e850_d_n6: f64 = p.p144;
        let eq46_e850_q: f64 = (p.p144 * eq46_e849_q);
        let eq46_e850_q_d_n6: f64 = p.p144;
        (eq46_e850, eq46_e850_d_n6, eq46_e850_q, eq46_e850_q_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[6]),
            None,
            nodes[6],
            multiplicity * (eq46_e852_q_d_n6),
        );
        let eq109_e1474_q: f64 = s.v[165];
        let eq109_e1475: f64 = (p.p7 * s.v[165]);
        let eq109_e1475_d_n0: f64 = (p.p7 * s.dn[165][0]);
        let eq109_e1475_d_n1: f64 = (p.p7 * s.dn[165][1]);
        let eq109_e1475_d_n2: f64 = (p.p7 * s.dn[165][2]);
        let eq109_e1475_d_n3: f64 = (p.p7 * s.dn[165][3]);
        let eq109_e1475_d_n4: f64 = (p.p7 * s.dn[165][4]);
        let eq109_e1475_d_n5: f64 = (p.p7 * s.dn[165][5]);
        let eq109_e1475_d_n6: f64 = (p.p7 * s.dn[165][6]);
        let eq109_e1475_d_n7: f64 = (p.p7 * s.dn[165][7]);
        let eq109_e1475_d_n8: f64 = (p.p7 * s.dn[165][8]);
        let eq109_e1475_d_n9: f64 = (p.p7 * s.dn[165][9]);
        let eq109_e1475_d_n10: f64 = (p.p7 * s.dn[165][10]);
        let eq109_e1475_d_n11: f64 = (p.p7 * s.dn[165][11]);
        let eq109_e1475_d_n12: f64 = (p.p7 * s.dn[165][12]);
        let eq109_e1475_d_n13: f64 = (p.p7 * s.dn[165][13]);
        let eq109_e1475_d_n14: f64 = (p.p7 * s.dn[165][14]);
        let eq109_e1475_d_n15: f64 = (p.p7 * s.dn[165][15]);
        let eq109_e1475_d_n16: f64 = (p.p7 * s.dn[165][16]);
        let eq109_e1475_d_n17: f64 = (p.p7 * s.dn[165][17]);
        let eq109_e1475_d_n18: f64 = (p.p7 * s.dn[165][18]);
        let eq109_e1475_d_n19: f64 = (p.p7 * s.dn[165][19]);
        let eq109_e1475_d_n20: f64 = (p.p7 * s.dn[165][20]);
        let eq109_e1475_d_n21: f64 = (p.p7 * s.dn[165][21]);
        let eq109_e1475_d_n22: f64 = (p.p7 * s.dn[165][22]);
        let eq109_e1475_q: f64 = (p.p7 * eq109_e1474_q);
        let eq109_e1475_q_d_n0: f64 = (p.p7 * s.dn[165][0]);
        let eq109_e1475_q_d_n1: f64 = (p.p7 * s.dn[165][1]);
        let eq109_e1475_q_d_n2: f64 = (p.p7 * s.dn[165][2]);
        let eq109_e1475_q_d_n3: f64 = (p.p7 * s.dn[165][3]);
        let eq109_e1475_q_d_n4: f64 = (p.p7 * s.dn[165][4]);
        let eq109_e1475_q_d_n5: f64 = (p.p7 * s.dn[165][5]);
        let eq109_e1475_q_d_n6: f64 = (p.p7 * s.dn[165][6]);
        let eq109_e1475_q_d_n7: f64 = (p.p7 * s.dn[165][7]);
        let eq109_e1475_q_d_n8: f64 = (p.p7 * s.dn[165][8]);
        let eq109_e1475_q_d_n9: f64 = (p.p7 * s.dn[165][9]);
        let eq109_e1475_q_d_n10: f64 = (p.p7 * s.dn[165][10]);
        let eq109_e1475_q_d_n11: f64 = (p.p7 * s.dn[165][11]);
        let eq109_e1475_q_d_n12: f64 = (p.p7 * s.dn[165][12]);
        let eq109_e1475_q_d_n13: f64 = (p.p7 * s.dn[165][13]);
        let eq109_e1475_q_d_n14: f64 = (p.p7 * s.dn[165][14]);
        let eq109_e1475_q_d_n15: f64 = (p.p7 * s.dn[165][15]);
        let eq109_e1475_q_d_n16: f64 = (p.p7 * s.dn[165][16]);
        let eq109_e1475_q_d_n17: f64 = (p.p7 * s.dn[165][17]);
        let eq109_e1475_q_d_n18: f64 = (p.p7 * s.dn[165][18]);
        let eq109_e1475_q_d_n19: f64 = (p.p7 * s.dn[165][19]);
        let eq109_e1475_q_d_n20: f64 = (p.p7 * s.dn[165][20]);
        let eq109_e1475_q_d_n21: f64 = (p.p7 * s.dn[165][21]);
        let eq109_e1475_q_d_n22: f64 = (p.p7 * s.dn[165][22]);
        let eq109_reactive_node_derivatives: [f64; 23] = [eq109_e1475_q_d_n0, eq109_e1475_q_d_n1, eq109_e1475_q_d_n2, eq109_e1475_q_d_n3, eq109_e1475_q_d_n4, eq109_e1475_q_d_n5, eq109_e1475_q_d_n6, eq109_e1475_q_d_n7, eq109_e1475_q_d_n8, eq109_e1475_q_d_n9, eq109_e1475_q_d_n10, eq109_e1475_q_d_n11, eq109_e1475_q_d_n12, eq109_e1475_q_d_n13, eq109_e1475_q_d_n14, eq109_e1475_q_d_n15, eq109_e1475_q_d_n16, eq109_e1475_q_d_n17, eq109_e1475_q_d_n18, eq109_e1475_q_d_n19, eq109_e1475_q_d_n20, eq109_e1475_q_d_n21, eq109_e1475_q_d_n22];
        let eq109_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq109_reactive_node_derivatives,
            branches,
            &eq109_reactive_branch_derivatives,
            multiplicity,
        );
        let eq110_e1478_q: f64 = s.v[161];
        let eq110_e1479: f64 = (p.p7 * s.v[161]);
        let eq110_e1479_d_n0: f64 = (p.p7 * s.dn[161][0]);
        let eq110_e1479_d_n1: f64 = (p.p7 * s.dn[161][1]);
        let eq110_e1479_d_n2: f64 = (p.p7 * s.dn[161][2]);
        let eq110_e1479_d_n3: f64 = (p.p7 * s.dn[161][3]);
        let eq110_e1479_d_n4: f64 = (p.p7 * s.dn[161][4]);
        let eq110_e1479_d_n5: f64 = (p.p7 * s.dn[161][5]);
        let eq110_e1479_d_n6: f64 = (p.p7 * s.dn[161][6]);
        let eq110_e1479_d_n7: f64 = (p.p7 * s.dn[161][7]);
        let eq110_e1479_d_n8: f64 = (p.p7 * s.dn[161][8]);
        let eq110_e1479_d_n9: f64 = (p.p7 * s.dn[161][9]);
        let eq110_e1479_d_n10: f64 = (p.p7 * s.dn[161][10]);
        let eq110_e1479_d_n11: f64 = (p.p7 * s.dn[161][11]);
        let eq110_e1479_d_n12: f64 = (p.p7 * s.dn[161][12]);
        let eq110_e1479_d_n13: f64 = (p.p7 * s.dn[161][13]);
        let eq110_e1479_d_n14: f64 = (p.p7 * s.dn[161][14]);
        let eq110_e1479_d_n15: f64 = (p.p7 * s.dn[161][15]);
        let eq110_e1479_d_n16: f64 = (p.p7 * s.dn[161][16]);
        let eq110_e1479_d_n17: f64 = (p.p7 * s.dn[161][17]);
        let eq110_e1479_d_n18: f64 = (p.p7 * s.dn[161][18]);
        let eq110_e1479_d_n19: f64 = (p.p7 * s.dn[161][19]);
        let eq110_e1479_d_n20: f64 = (p.p7 * s.dn[161][20]);
        let eq110_e1479_d_n21: f64 = (p.p7 * s.dn[161][21]);
        let eq110_e1479_d_n22: f64 = (p.p7 * s.dn[161][22]);
        let eq110_e1479_q: f64 = (p.p7 * eq110_e1478_q);
        let eq110_e1479_q_d_n0: f64 = (p.p7 * s.dn[161][0]);
        let eq110_e1479_q_d_n1: f64 = (p.p7 * s.dn[161][1]);
        let eq110_e1479_q_d_n2: f64 = (p.p7 * s.dn[161][2]);
        let eq110_e1479_q_d_n3: f64 = (p.p7 * s.dn[161][3]);
        let eq110_e1479_q_d_n4: f64 = (p.p7 * s.dn[161][4]);
        let eq110_e1479_q_d_n5: f64 = (p.p7 * s.dn[161][5]);
        let eq110_e1479_q_d_n6: f64 = (p.p7 * s.dn[161][6]);
        let eq110_e1479_q_d_n7: f64 = (p.p7 * s.dn[161][7]);
        let eq110_e1479_q_d_n8: f64 = (p.p7 * s.dn[161][8]);
        let eq110_e1479_q_d_n9: f64 = (p.p7 * s.dn[161][9]);
        let eq110_e1479_q_d_n10: f64 = (p.p7 * s.dn[161][10]);
        let eq110_e1479_q_d_n11: f64 = (p.p7 * s.dn[161][11]);
        let eq110_e1479_q_d_n12: f64 = (p.p7 * s.dn[161][12]);
        let eq110_e1479_q_d_n13: f64 = (p.p7 * s.dn[161][13]);
        let eq110_e1479_q_d_n14: f64 = (p.p7 * s.dn[161][14]);
        let eq110_e1479_q_d_n15: f64 = (p.p7 * s.dn[161][15]);
        let eq110_e1479_q_d_n16: f64 = (p.p7 * s.dn[161][16]);
        let eq110_e1479_q_d_n17: f64 = (p.p7 * s.dn[161][17]);
        let eq110_e1479_q_d_n18: f64 = (p.p7 * s.dn[161][18]);
        let eq110_e1479_q_d_n19: f64 = (p.p7 * s.dn[161][19]);
        let eq110_e1479_q_d_n20: f64 = (p.p7 * s.dn[161][20]);
        let eq110_e1479_q_d_n21: f64 = (p.p7 * s.dn[161][21]);
        let eq110_e1479_q_d_n22: f64 = (p.p7 * s.dn[161][22]);
        let eq110_reactive_node_derivatives: [f64; 23] = [eq110_e1479_q_d_n0, eq110_e1479_q_d_n1, eq110_e1479_q_d_n2, eq110_e1479_q_d_n3, eq110_e1479_q_d_n4, eq110_e1479_q_d_n5, eq110_e1479_q_d_n6, eq110_e1479_q_d_n7, eq110_e1479_q_d_n8, eq110_e1479_q_d_n9, eq110_e1479_q_d_n10, eq110_e1479_q_d_n11, eq110_e1479_q_d_n12, eq110_e1479_q_d_n13, eq110_e1479_q_d_n14, eq110_e1479_q_d_n15, eq110_e1479_q_d_n16, eq110_e1479_q_d_n17, eq110_e1479_q_d_n18, eq110_e1479_q_d_n19, eq110_e1479_q_d_n20, eq110_e1479_q_d_n21, eq110_e1479_q_d_n22];
        let eq110_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq110_reactive_node_derivatives,
            branches,
            &eq110_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1486, eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22, eq111_e1486_q, eq111_e1486_q_d_n0, eq111_e1486_q_d_n1, eq111_e1486_q_d_n2, eq111_e1486_q_d_n3, eq111_e1486_q_d_n4, eq111_e1486_q_d_n5, eq111_e1486_q_d_n6, eq111_e1486_q_d_n7, eq111_e1486_q_d_n8, eq111_e1486_q_d_n9, eq111_e1486_q_d_n10, eq111_e1486_q_d_n11, eq111_e1486_q_d_n12, eq111_e1486_q_d_n13, eq111_e1486_q_d_n14, eq111_e1486_q_d_n15, eq111_e1486_q_d_n16, eq111_e1486_q_d_n17, eq111_e1486_q_d_n18, eq111_e1486_q_d_n19, eq111_e1486_q_d_n20, eq111_e1486_q_d_n21, eq111_e1486_q_d_n22,) = {
    if s.b[569] {
        let eq111_e1483_q: f64 = s.v[162];
        let eq111_e1484: f64 = (p.p7 * s.v[162]);
        let eq111_e1484_d_n0: f64 = (p.p7 * s.dn[162][0]);
        let eq111_e1484_d_n1: f64 = (p.p7 * s.dn[162][1]);
        let eq111_e1484_d_n2: f64 = (p.p7 * s.dn[162][2]);
        let eq111_e1484_d_n3: f64 = (p.p7 * s.dn[162][3]);
        let eq111_e1484_d_n4: f64 = (p.p7 * s.dn[162][4]);
        let eq111_e1484_d_n5: f64 = (p.p7 * s.dn[162][5]);
        let eq111_e1484_d_n6: f64 = (p.p7 * s.dn[162][6]);
        let eq111_e1484_d_n7: f64 = (p.p7 * s.dn[162][7]);
        let eq111_e1484_d_n8: f64 = (p.p7 * s.dn[162][8]);
        let eq111_e1484_d_n9: f64 = (p.p7 * s.dn[162][9]);
        let eq111_e1484_d_n10: f64 = (p.p7 * s.dn[162][10]);
        let eq111_e1484_d_n11: f64 = (p.p7 * s.dn[162][11]);
        let eq111_e1484_d_n12: f64 = (p.p7 * s.dn[162][12]);
        let eq111_e1484_d_n13: f64 = (p.p7 * s.dn[162][13]);
        let eq111_e1484_d_n14: f64 = (p.p7 * s.dn[162][14]);
        let eq111_e1484_d_n15: f64 = (p.p7 * s.dn[162][15]);
        let eq111_e1484_d_n16: f64 = (p.p7 * s.dn[162][16]);
        let eq111_e1484_d_n17: f64 = (p.p7 * s.dn[162][17]);
        let eq111_e1484_d_n18: f64 = (p.p7 * s.dn[162][18]);
        let eq111_e1484_d_n19: f64 = (p.p7 * s.dn[162][19]);
        let eq111_e1484_d_n20: f64 = (p.p7 * s.dn[162][20]);
        let eq111_e1484_d_n21: f64 = (p.p7 * s.dn[162][21]);
        let eq111_e1484_d_n22: f64 = (p.p7 * s.dn[162][22]);
        let eq111_e1484_q: f64 = (p.p7 * eq111_e1483_q);
        let eq111_e1484_q_d_n0: f64 = (p.p7 * s.dn[162][0]);
        let eq111_e1484_q_d_n1: f64 = (p.p7 * s.dn[162][1]);
        let eq111_e1484_q_d_n2: f64 = (p.p7 * s.dn[162][2]);
        let eq111_e1484_q_d_n3: f64 = (p.p7 * s.dn[162][3]);
        let eq111_e1484_q_d_n4: f64 = (p.p7 * s.dn[162][4]);
        let eq111_e1484_q_d_n5: f64 = (p.p7 * s.dn[162][5]);
        let eq111_e1484_q_d_n6: f64 = (p.p7 * s.dn[162][6]);
        let eq111_e1484_q_d_n7: f64 = (p.p7 * s.dn[162][7]);
        let eq111_e1484_q_d_n8: f64 = (p.p7 * s.dn[162][8]);
        let eq111_e1484_q_d_n9: f64 = (p.p7 * s.dn[162][9]);
        let eq111_e1484_q_d_n10: f64 = (p.p7 * s.dn[162][10]);
        let eq111_e1484_q_d_n11: f64 = (p.p7 * s.dn[162][11]);
        let eq111_e1484_q_d_n12: f64 = (p.p7 * s.dn[162][12]);
        let eq111_e1484_q_d_n13: f64 = (p.p7 * s.dn[162][13]);
        let eq111_e1484_q_d_n14: f64 = (p.p7 * s.dn[162][14]);
        let eq111_e1484_q_d_n15: f64 = (p.p7 * s.dn[162][15]);
        let eq111_e1484_q_d_n16: f64 = (p.p7 * s.dn[162][16]);
        let eq111_e1484_q_d_n17: f64 = (p.p7 * s.dn[162][17]);
        let eq111_e1484_q_d_n18: f64 = (p.p7 * s.dn[162][18]);
        let eq111_e1484_q_d_n19: f64 = (p.p7 * s.dn[162][19]);
        let eq111_e1484_q_d_n20: f64 = (p.p7 * s.dn[162][20]);
        let eq111_e1484_q_d_n21: f64 = (p.p7 * s.dn[162][21]);
        let eq111_e1484_q_d_n22: f64 = (p.p7 * s.dn[162][22]);
        (eq111_e1484, eq111_e1484_d_n0, eq111_e1484_d_n1, eq111_e1484_d_n2, eq111_e1484_d_n3, eq111_e1484_d_n4, eq111_e1484_d_n5, eq111_e1484_d_n6, eq111_e1484_d_n7, eq111_e1484_d_n8, eq111_e1484_d_n9, eq111_e1484_d_n10, eq111_e1484_d_n11, eq111_e1484_d_n12, eq111_e1484_d_n13, eq111_e1484_d_n14, eq111_e1484_d_n15, eq111_e1484_d_n16, eq111_e1484_d_n17, eq111_e1484_d_n18, eq111_e1484_d_n19, eq111_e1484_d_n20, eq111_e1484_d_n21, eq111_e1484_d_n22, eq111_e1484_q, eq111_e1484_q_d_n0, eq111_e1484_q_d_n1, eq111_e1484_q_d_n2, eq111_e1484_q_d_n3, eq111_e1484_q_d_n4, eq111_e1484_q_d_n5, eq111_e1484_q_d_n6, eq111_e1484_q_d_n7, eq111_e1484_q_d_n8, eq111_e1484_q_d_n9, eq111_e1484_q_d_n10, eq111_e1484_q_d_n11, eq111_e1484_q_d_n12, eq111_e1484_q_d_n13, eq111_e1484_q_d_n14, eq111_e1484_q_d_n15, eq111_e1484_q_d_n16, eq111_e1484_q_d_n17, eq111_e1484_q_d_n18, eq111_e1484_q_d_n19, eq111_e1484_q_d_n20, eq111_e1484_q_d_n21, eq111_e1484_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 23] = [eq111_e1486_q_d_n0, eq111_e1486_q_d_n1, eq111_e1486_q_d_n2, eq111_e1486_q_d_n3, eq111_e1486_q_d_n4, eq111_e1486_q_d_n5, eq111_e1486_q_d_n6, eq111_e1486_q_d_n7, eq111_e1486_q_d_n8, eq111_e1486_q_d_n9, eq111_e1486_q_d_n10, eq111_e1486_q_d_n11, eq111_e1486_q_d_n12, eq111_e1486_q_d_n13, eq111_e1486_q_d_n14, eq111_e1486_q_d_n15, eq111_e1486_q_d_n16, eq111_e1486_q_d_n17, eq111_e1486_q_d_n18, eq111_e1486_q_d_n19, eq111_e1486_q_d_n20, eq111_e1486_q_d_n21, eq111_e1486_q_d_n22];
        let eq111_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22, eq112_e1493_q, eq112_e1493_q_d_n0, eq112_e1493_q_d_n1, eq112_e1493_q_d_n2, eq112_e1493_q_d_n3, eq112_e1493_q_d_n4, eq112_e1493_q_d_n5, eq112_e1493_q_d_n6, eq112_e1493_q_d_n7, eq112_e1493_q_d_n8, eq112_e1493_q_d_n9, eq112_e1493_q_d_n10, eq112_e1493_q_d_n11, eq112_e1493_q_d_n12, eq112_e1493_q_d_n13, eq112_e1493_q_d_n14, eq112_e1493_q_d_n15, eq112_e1493_q_d_n16, eq112_e1493_q_d_n17, eq112_e1493_q_d_n18, eq112_e1493_q_d_n19, eq112_e1493_q_d_n20, eq112_e1493_q_d_n21, eq112_e1493_q_d_n22,) = {
    if s.b[569] {
        let eq112_e1490_q: f64 = s.v[163];
        let eq112_e1491: f64 = (p.p7 * s.v[163]);
        let eq112_e1491_d_n0: f64 = (p.p7 * s.dn[163][0]);
        let eq112_e1491_d_n1: f64 = (p.p7 * s.dn[163][1]);
        let eq112_e1491_d_n2: f64 = (p.p7 * s.dn[163][2]);
        let eq112_e1491_d_n3: f64 = (p.p7 * s.dn[163][3]);
        let eq112_e1491_d_n4: f64 = (p.p7 * s.dn[163][4]);
        let eq112_e1491_d_n5: f64 = (p.p7 * s.dn[163][5]);
        let eq112_e1491_d_n6: f64 = (p.p7 * s.dn[163][6]);
        let eq112_e1491_d_n7: f64 = (p.p7 * s.dn[163][7]);
        let eq112_e1491_d_n8: f64 = (p.p7 * s.dn[163][8]);
        let eq112_e1491_d_n9: f64 = (p.p7 * s.dn[163][9]);
        let eq112_e1491_d_n10: f64 = (p.p7 * s.dn[163][10]);
        let eq112_e1491_d_n11: f64 = (p.p7 * s.dn[163][11]);
        let eq112_e1491_d_n12: f64 = (p.p7 * s.dn[163][12]);
        let eq112_e1491_d_n13: f64 = (p.p7 * s.dn[163][13]);
        let eq112_e1491_d_n14: f64 = (p.p7 * s.dn[163][14]);
        let eq112_e1491_d_n15: f64 = (p.p7 * s.dn[163][15]);
        let eq112_e1491_d_n16: f64 = (p.p7 * s.dn[163][16]);
        let eq112_e1491_d_n17: f64 = (p.p7 * s.dn[163][17]);
        let eq112_e1491_d_n18: f64 = (p.p7 * s.dn[163][18]);
        let eq112_e1491_d_n19: f64 = (p.p7 * s.dn[163][19]);
        let eq112_e1491_d_n20: f64 = (p.p7 * s.dn[163][20]);
        let eq112_e1491_d_n21: f64 = (p.p7 * s.dn[163][21]);
        let eq112_e1491_d_n22: f64 = (p.p7 * s.dn[163][22]);
        let eq112_e1491_q: f64 = (p.p7 * eq112_e1490_q);
        let eq112_e1491_q_d_n0: f64 = (p.p7 * s.dn[163][0]);
        let eq112_e1491_q_d_n1: f64 = (p.p7 * s.dn[163][1]);
        let eq112_e1491_q_d_n2: f64 = (p.p7 * s.dn[163][2]);
        let eq112_e1491_q_d_n3: f64 = (p.p7 * s.dn[163][3]);
        let eq112_e1491_q_d_n4: f64 = (p.p7 * s.dn[163][4]);
        let eq112_e1491_q_d_n5: f64 = (p.p7 * s.dn[163][5]);
        let eq112_e1491_q_d_n6: f64 = (p.p7 * s.dn[163][6]);
        let eq112_e1491_q_d_n7: f64 = (p.p7 * s.dn[163][7]);
        let eq112_e1491_q_d_n8: f64 = (p.p7 * s.dn[163][8]);
        let eq112_e1491_q_d_n9: f64 = (p.p7 * s.dn[163][9]);
        let eq112_e1491_q_d_n10: f64 = (p.p7 * s.dn[163][10]);
        let eq112_e1491_q_d_n11: f64 = (p.p7 * s.dn[163][11]);
        let eq112_e1491_q_d_n12: f64 = (p.p7 * s.dn[163][12]);
        let eq112_e1491_q_d_n13: f64 = (p.p7 * s.dn[163][13]);
        let eq112_e1491_q_d_n14: f64 = (p.p7 * s.dn[163][14]);
        let eq112_e1491_q_d_n15: f64 = (p.p7 * s.dn[163][15]);
        let eq112_e1491_q_d_n16: f64 = (p.p7 * s.dn[163][16]);
        let eq112_e1491_q_d_n17: f64 = (p.p7 * s.dn[163][17]);
        let eq112_e1491_q_d_n18: f64 = (p.p7 * s.dn[163][18]);
        let eq112_e1491_q_d_n19: f64 = (p.p7 * s.dn[163][19]);
        let eq112_e1491_q_d_n20: f64 = (p.p7 * s.dn[163][20]);
        let eq112_e1491_q_d_n21: f64 = (p.p7 * s.dn[163][21]);
        let eq112_e1491_q_d_n22: f64 = (p.p7 * s.dn[163][22]);
        (eq112_e1491, eq112_e1491_d_n0, eq112_e1491_d_n1, eq112_e1491_d_n2, eq112_e1491_d_n3, eq112_e1491_d_n4, eq112_e1491_d_n5, eq112_e1491_d_n6, eq112_e1491_d_n7, eq112_e1491_d_n8, eq112_e1491_d_n9, eq112_e1491_d_n10, eq112_e1491_d_n11, eq112_e1491_d_n12, eq112_e1491_d_n13, eq112_e1491_d_n14, eq112_e1491_d_n15, eq112_e1491_d_n16, eq112_e1491_d_n17, eq112_e1491_d_n18, eq112_e1491_d_n19, eq112_e1491_d_n20, eq112_e1491_d_n21, eq112_e1491_d_n22, eq112_e1491_q, eq112_e1491_q_d_n0, eq112_e1491_q_d_n1, eq112_e1491_q_d_n2, eq112_e1491_q_d_n3, eq112_e1491_q_d_n4, eq112_e1491_q_d_n5, eq112_e1491_q_d_n6, eq112_e1491_q_d_n7, eq112_e1491_q_d_n8, eq112_e1491_q_d_n9, eq112_e1491_q_d_n10, eq112_e1491_q_d_n11, eq112_e1491_q_d_n12, eq112_e1491_q_d_n13, eq112_e1491_q_d_n14, eq112_e1491_q_d_n15, eq112_e1491_q_d_n16, eq112_e1491_q_d_n17, eq112_e1491_q_d_n18, eq112_e1491_q_d_n19, eq112_e1491_q_d_n20, eq112_e1491_q_d_n21, eq112_e1491_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_reactive_node_derivatives: [f64; 23] = [eq112_e1493_q_d_n0, eq112_e1493_q_d_n1, eq112_e1493_q_d_n2, eq112_e1493_q_d_n3, eq112_e1493_q_d_n4, eq112_e1493_q_d_n5, eq112_e1493_q_d_n6, eq112_e1493_q_d_n7, eq112_e1493_q_d_n8, eq112_e1493_q_d_n9, eq112_e1493_q_d_n10, eq112_e1493_q_d_n11, eq112_e1493_q_d_n12, eq112_e1493_q_d_n13, eq112_e1493_q_d_n14, eq112_e1493_q_d_n15, eq112_e1493_q_d_n16, eq112_e1493_q_d_n17, eq112_e1493_q_d_n18, eq112_e1493_q_d_n19, eq112_e1493_q_d_n20, eq112_e1493_q_d_n21, eq112_e1493_q_d_n22];
        let eq112_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq112_reactive_node_derivatives,
            branches,
            &eq112_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq113_e1501, eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22, eq113_e1501_q, eq113_e1501_q_d_n0, eq113_e1501_q_d_n1, eq113_e1501_q_d_n2, eq113_e1501_q_d_n3, eq113_e1501_q_d_n4, eq113_e1501_q_d_n5, eq113_e1501_q_d_n6, eq113_e1501_q_d_n7, eq113_e1501_q_d_n8, eq113_e1501_q_d_n9, eq113_e1501_q_d_n10, eq113_e1501_q_d_n11, eq113_e1501_q_d_n12, eq113_e1501_q_d_n13, eq113_e1501_q_d_n14, eq113_e1501_q_d_n15, eq113_e1501_q_d_n16, eq113_e1501_q_d_n17, eq113_e1501_q_d_n18, eq113_e1501_q_d_n19, eq113_e1501_q_d_n20, eq113_e1501_q_d_n21, eq113_e1501_q_d_n22,) = {
    if (!s.b[569]) {
        let eq113_e1498_q: f64 = s.v[162];
        let eq113_e1499: f64 = (p.p7 * s.v[162]);
        let eq113_e1499_d_n0: f64 = (p.p7 * s.dn[162][0]);
        let eq113_e1499_d_n1: f64 = (p.p7 * s.dn[162][1]);
        let eq113_e1499_d_n2: f64 = (p.p7 * s.dn[162][2]);
        let eq113_e1499_d_n3: f64 = (p.p7 * s.dn[162][3]);
        let eq113_e1499_d_n4: f64 = (p.p7 * s.dn[162][4]);
        let eq113_e1499_d_n5: f64 = (p.p7 * s.dn[162][5]);
        let eq113_e1499_d_n6: f64 = (p.p7 * s.dn[162][6]);
        let eq113_e1499_d_n7: f64 = (p.p7 * s.dn[162][7]);
        let eq113_e1499_d_n8: f64 = (p.p7 * s.dn[162][8]);
        let eq113_e1499_d_n9: f64 = (p.p7 * s.dn[162][9]);
        let eq113_e1499_d_n10: f64 = (p.p7 * s.dn[162][10]);
        let eq113_e1499_d_n11: f64 = (p.p7 * s.dn[162][11]);
        let eq113_e1499_d_n12: f64 = (p.p7 * s.dn[162][12]);
        let eq113_e1499_d_n13: f64 = (p.p7 * s.dn[162][13]);
        let eq113_e1499_d_n14: f64 = (p.p7 * s.dn[162][14]);
        let eq113_e1499_d_n15: f64 = (p.p7 * s.dn[162][15]);
        let eq113_e1499_d_n16: f64 = (p.p7 * s.dn[162][16]);
        let eq113_e1499_d_n17: f64 = (p.p7 * s.dn[162][17]);
        let eq113_e1499_d_n18: f64 = (p.p7 * s.dn[162][18]);
        let eq113_e1499_d_n19: f64 = (p.p7 * s.dn[162][19]);
        let eq113_e1499_d_n20: f64 = (p.p7 * s.dn[162][20]);
        let eq113_e1499_d_n21: f64 = (p.p7 * s.dn[162][21]);
        let eq113_e1499_d_n22: f64 = (p.p7 * s.dn[162][22]);
        let eq113_e1499_q: f64 = (p.p7 * eq113_e1498_q);
        let eq113_e1499_q_d_n0: f64 = (p.p7 * s.dn[162][0]);
        let eq113_e1499_q_d_n1: f64 = (p.p7 * s.dn[162][1]);
        let eq113_e1499_q_d_n2: f64 = (p.p7 * s.dn[162][2]);
        let eq113_e1499_q_d_n3: f64 = (p.p7 * s.dn[162][3]);
        let eq113_e1499_q_d_n4: f64 = (p.p7 * s.dn[162][4]);
        let eq113_e1499_q_d_n5: f64 = (p.p7 * s.dn[162][5]);
        let eq113_e1499_q_d_n6: f64 = (p.p7 * s.dn[162][6]);
        let eq113_e1499_q_d_n7: f64 = (p.p7 * s.dn[162][7]);
        let eq113_e1499_q_d_n8: f64 = (p.p7 * s.dn[162][8]);
        let eq113_e1499_q_d_n9: f64 = (p.p7 * s.dn[162][9]);
        let eq113_e1499_q_d_n10: f64 = (p.p7 * s.dn[162][10]);
        let eq113_e1499_q_d_n11: f64 = (p.p7 * s.dn[162][11]);
        let eq113_e1499_q_d_n12: f64 = (p.p7 * s.dn[162][12]);
        let eq113_e1499_q_d_n13: f64 = (p.p7 * s.dn[162][13]);
        let eq113_e1499_q_d_n14: f64 = (p.p7 * s.dn[162][14]);
        let eq113_e1499_q_d_n15: f64 = (p.p7 * s.dn[162][15]);
        let eq113_e1499_q_d_n16: f64 = (p.p7 * s.dn[162][16]);
        let eq113_e1499_q_d_n17: f64 = (p.p7 * s.dn[162][17]);
        let eq113_e1499_q_d_n18: f64 = (p.p7 * s.dn[162][18]);
        let eq113_e1499_q_d_n19: f64 = (p.p7 * s.dn[162][19]);
        let eq113_e1499_q_d_n20: f64 = (p.p7 * s.dn[162][20]);
        let eq113_e1499_q_d_n21: f64 = (p.p7 * s.dn[162][21]);
        let eq113_e1499_q_d_n22: f64 = (p.p7 * s.dn[162][22]);
        (eq113_e1499, eq113_e1499_d_n0, eq113_e1499_d_n1, eq113_e1499_d_n2, eq113_e1499_d_n3, eq113_e1499_d_n4, eq113_e1499_d_n5, eq113_e1499_d_n6, eq113_e1499_d_n7, eq113_e1499_d_n8, eq113_e1499_d_n9, eq113_e1499_d_n10, eq113_e1499_d_n11, eq113_e1499_d_n12, eq113_e1499_d_n13, eq113_e1499_d_n14, eq113_e1499_d_n15, eq113_e1499_d_n16, eq113_e1499_d_n17, eq113_e1499_d_n18, eq113_e1499_d_n19, eq113_e1499_d_n20, eq113_e1499_d_n21, eq113_e1499_d_n22, eq113_e1499_q, eq113_e1499_q_d_n0, eq113_e1499_q_d_n1, eq113_e1499_q_d_n2, eq113_e1499_q_d_n3, eq113_e1499_q_d_n4, eq113_e1499_q_d_n5, eq113_e1499_q_d_n6, eq113_e1499_q_d_n7, eq113_e1499_q_d_n8, eq113_e1499_q_d_n9, eq113_e1499_q_d_n10, eq113_e1499_q_d_n11, eq113_e1499_q_d_n12, eq113_e1499_q_d_n13, eq113_e1499_q_d_n14, eq113_e1499_q_d_n15, eq113_e1499_q_d_n16, eq113_e1499_q_d_n17, eq113_e1499_q_d_n18, eq113_e1499_q_d_n19, eq113_e1499_q_d_n20, eq113_e1499_q_d_n21, eq113_e1499_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_reactive_node_derivatives: [f64; 23] = [eq113_e1501_q_d_n0, eq113_e1501_q_d_n1, eq113_e1501_q_d_n2, eq113_e1501_q_d_n3, eq113_e1501_q_d_n4, eq113_e1501_q_d_n5, eq113_e1501_q_d_n6, eq113_e1501_q_d_n7, eq113_e1501_q_d_n8, eq113_e1501_q_d_n9, eq113_e1501_q_d_n10, eq113_e1501_q_d_n11, eq113_e1501_q_d_n12, eq113_e1501_q_d_n13, eq113_e1501_q_d_n14, eq113_e1501_q_d_n15, eq113_e1501_q_d_n16, eq113_e1501_q_d_n17, eq113_e1501_q_d_n18, eq113_e1501_q_d_n19, eq113_e1501_q_d_n20, eq113_e1501_q_d_n21, eq113_e1501_q_d_n22];
        let eq113_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq113_reactive_node_derivatives,
            branches,
            &eq113_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22, eq114_e1509_q, eq114_e1509_q_d_n0, eq114_e1509_q_d_n1, eq114_e1509_q_d_n2, eq114_e1509_q_d_n3, eq114_e1509_q_d_n4, eq114_e1509_q_d_n5, eq114_e1509_q_d_n6, eq114_e1509_q_d_n7, eq114_e1509_q_d_n8, eq114_e1509_q_d_n9, eq114_e1509_q_d_n10, eq114_e1509_q_d_n11, eq114_e1509_q_d_n12, eq114_e1509_q_d_n13, eq114_e1509_q_d_n14, eq114_e1509_q_d_n15, eq114_e1509_q_d_n16, eq114_e1509_q_d_n17, eq114_e1509_q_d_n18, eq114_e1509_q_d_n19, eq114_e1509_q_d_n20, eq114_e1509_q_d_n21, eq114_e1509_q_d_n22,) = {
    if (!s.b[569]) {
        let eq114_e1506_q: f64 = s.v[163];
        let eq114_e1507: f64 = (p.p7 * s.v[163]);
        let eq114_e1507_d_n0: f64 = (p.p7 * s.dn[163][0]);
        let eq114_e1507_d_n1: f64 = (p.p7 * s.dn[163][1]);
        let eq114_e1507_d_n2: f64 = (p.p7 * s.dn[163][2]);
        let eq114_e1507_d_n3: f64 = (p.p7 * s.dn[163][3]);
        let eq114_e1507_d_n4: f64 = (p.p7 * s.dn[163][4]);
        let eq114_e1507_d_n5: f64 = (p.p7 * s.dn[163][5]);
        let eq114_e1507_d_n6: f64 = (p.p7 * s.dn[163][6]);
        let eq114_e1507_d_n7: f64 = (p.p7 * s.dn[163][7]);
        let eq114_e1507_d_n8: f64 = (p.p7 * s.dn[163][8]);
        let eq114_e1507_d_n9: f64 = (p.p7 * s.dn[163][9]);
        let eq114_e1507_d_n10: f64 = (p.p7 * s.dn[163][10]);
        let eq114_e1507_d_n11: f64 = (p.p7 * s.dn[163][11]);
        let eq114_e1507_d_n12: f64 = (p.p7 * s.dn[163][12]);
        let eq114_e1507_d_n13: f64 = (p.p7 * s.dn[163][13]);
        let eq114_e1507_d_n14: f64 = (p.p7 * s.dn[163][14]);
        let eq114_e1507_d_n15: f64 = (p.p7 * s.dn[163][15]);
        let eq114_e1507_d_n16: f64 = (p.p7 * s.dn[163][16]);
        let eq114_e1507_d_n17: f64 = (p.p7 * s.dn[163][17]);
        let eq114_e1507_d_n18: f64 = (p.p7 * s.dn[163][18]);
        let eq114_e1507_d_n19: f64 = (p.p7 * s.dn[163][19]);
        let eq114_e1507_d_n20: f64 = (p.p7 * s.dn[163][20]);
        let eq114_e1507_d_n21: f64 = (p.p7 * s.dn[163][21]);
        let eq114_e1507_d_n22: f64 = (p.p7 * s.dn[163][22]);
        let eq114_e1507_q: f64 = (p.p7 * eq114_e1506_q);
        let eq114_e1507_q_d_n0: f64 = (p.p7 * s.dn[163][0]);
        let eq114_e1507_q_d_n1: f64 = (p.p7 * s.dn[163][1]);
        let eq114_e1507_q_d_n2: f64 = (p.p7 * s.dn[163][2]);
        let eq114_e1507_q_d_n3: f64 = (p.p7 * s.dn[163][3]);
        let eq114_e1507_q_d_n4: f64 = (p.p7 * s.dn[163][4]);
        let eq114_e1507_q_d_n5: f64 = (p.p7 * s.dn[163][5]);
        let eq114_e1507_q_d_n6: f64 = (p.p7 * s.dn[163][6]);
        let eq114_e1507_q_d_n7: f64 = (p.p7 * s.dn[163][7]);
        let eq114_e1507_q_d_n8: f64 = (p.p7 * s.dn[163][8]);
        let eq114_e1507_q_d_n9: f64 = (p.p7 * s.dn[163][9]);
        let eq114_e1507_q_d_n10: f64 = (p.p7 * s.dn[163][10]);
        let eq114_e1507_q_d_n11: f64 = (p.p7 * s.dn[163][11]);
        let eq114_e1507_q_d_n12: f64 = (p.p7 * s.dn[163][12]);
        let eq114_e1507_q_d_n13: f64 = (p.p7 * s.dn[163][13]);
        let eq114_e1507_q_d_n14: f64 = (p.p7 * s.dn[163][14]);
        let eq114_e1507_q_d_n15: f64 = (p.p7 * s.dn[163][15]);
        let eq114_e1507_q_d_n16: f64 = (p.p7 * s.dn[163][16]);
        let eq114_e1507_q_d_n17: f64 = (p.p7 * s.dn[163][17]);
        let eq114_e1507_q_d_n18: f64 = (p.p7 * s.dn[163][18]);
        let eq114_e1507_q_d_n19: f64 = (p.p7 * s.dn[163][19]);
        let eq114_e1507_q_d_n20: f64 = (p.p7 * s.dn[163][20]);
        let eq114_e1507_q_d_n21: f64 = (p.p7 * s.dn[163][21]);
        let eq114_e1507_q_d_n22: f64 = (p.p7 * s.dn[163][22]);
        (eq114_e1507, eq114_e1507_d_n0, eq114_e1507_d_n1, eq114_e1507_d_n2, eq114_e1507_d_n3, eq114_e1507_d_n4, eq114_e1507_d_n5, eq114_e1507_d_n6, eq114_e1507_d_n7, eq114_e1507_d_n8, eq114_e1507_d_n9, eq114_e1507_d_n10, eq114_e1507_d_n11, eq114_e1507_d_n12, eq114_e1507_d_n13, eq114_e1507_d_n14, eq114_e1507_d_n15, eq114_e1507_d_n16, eq114_e1507_d_n17, eq114_e1507_d_n18, eq114_e1507_d_n19, eq114_e1507_d_n20, eq114_e1507_d_n21, eq114_e1507_d_n22, eq114_e1507_q, eq114_e1507_q_d_n0, eq114_e1507_q_d_n1, eq114_e1507_q_d_n2, eq114_e1507_q_d_n3, eq114_e1507_q_d_n4, eq114_e1507_q_d_n5, eq114_e1507_q_d_n6, eq114_e1507_q_d_n7, eq114_e1507_q_d_n8, eq114_e1507_q_d_n9, eq114_e1507_q_d_n10, eq114_e1507_q_d_n11, eq114_e1507_q_d_n12, eq114_e1507_q_d_n13, eq114_e1507_q_d_n14, eq114_e1507_q_d_n15, eq114_e1507_q_d_n16, eq114_e1507_q_d_n17, eq114_e1507_q_d_n18, eq114_e1507_q_d_n19, eq114_e1507_q_d_n20, eq114_e1507_q_d_n21, eq114_e1507_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_reactive_node_derivatives: [f64; 23] = [eq114_e1509_q_d_n0, eq114_e1509_q_d_n1, eq114_e1509_q_d_n2, eq114_e1509_q_d_n3, eq114_e1509_q_d_n4, eq114_e1509_q_d_n5, eq114_e1509_q_d_n6, eq114_e1509_q_d_n7, eq114_e1509_q_d_n8, eq114_e1509_q_d_n9, eq114_e1509_q_d_n10, eq114_e1509_q_d_n11, eq114_e1509_q_d_n12, eq114_e1509_q_d_n13, eq114_e1509_q_d_n14, eq114_e1509_q_d_n15, eq114_e1509_q_d_n16, eq114_e1509_q_d_n17, eq114_e1509_q_d_n18, eq114_e1509_q_d_n19, eq114_e1509_q_d_n20, eq114_e1509_q_d_n21, eq114_e1509_q_d_n22];
        let eq114_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq114_reactive_node_derivatives,
            branches,
            &eq114_reactive_branch_derivatives,
            multiplicity,
        );
        let eq115_e1512_q: f64 = s.v[164];
        let eq115_e1513: f64 = (p.p7 * s.v[164]);
        let eq115_e1513_d_n0: f64 = (p.p7 * s.dn[164][0]);
        let eq115_e1513_d_n1: f64 = (p.p7 * s.dn[164][1]);
        let eq115_e1513_d_n2: f64 = (p.p7 * s.dn[164][2]);
        let eq115_e1513_d_n3: f64 = (p.p7 * s.dn[164][3]);
        let eq115_e1513_d_n4: f64 = (p.p7 * s.dn[164][4]);
        let eq115_e1513_d_n5: f64 = (p.p7 * s.dn[164][5]);
        let eq115_e1513_d_n6: f64 = (p.p7 * s.dn[164][6]);
        let eq115_e1513_d_n7: f64 = (p.p7 * s.dn[164][7]);
        let eq115_e1513_d_n8: f64 = (p.p7 * s.dn[164][8]);
        let eq115_e1513_d_n9: f64 = (p.p7 * s.dn[164][9]);
        let eq115_e1513_d_n10: f64 = (p.p7 * s.dn[164][10]);
        let eq115_e1513_d_n11: f64 = (p.p7 * s.dn[164][11]);
        let eq115_e1513_d_n12: f64 = (p.p7 * s.dn[164][12]);
        let eq115_e1513_d_n13: f64 = (p.p7 * s.dn[164][13]);
        let eq115_e1513_d_n14: f64 = (p.p7 * s.dn[164][14]);
        let eq115_e1513_d_n15: f64 = (p.p7 * s.dn[164][15]);
        let eq115_e1513_d_n16: f64 = (p.p7 * s.dn[164][16]);
        let eq115_e1513_d_n17: f64 = (p.p7 * s.dn[164][17]);
        let eq115_e1513_d_n18: f64 = (p.p7 * s.dn[164][18]);
        let eq115_e1513_d_n19: f64 = (p.p7 * s.dn[164][19]);
        let eq115_e1513_d_n20: f64 = (p.p7 * s.dn[164][20]);
        let eq115_e1513_d_n21: f64 = (p.p7 * s.dn[164][21]);
        let eq115_e1513_d_n22: f64 = (p.p7 * s.dn[164][22]);
        let eq115_e1513_q: f64 = (p.p7 * eq115_e1512_q);
        let eq115_e1513_q_d_n0: f64 = (p.p7 * s.dn[164][0]);
        let eq115_e1513_q_d_n1: f64 = (p.p7 * s.dn[164][1]);
        let eq115_e1513_q_d_n2: f64 = (p.p7 * s.dn[164][2]);
        let eq115_e1513_q_d_n3: f64 = (p.p7 * s.dn[164][3]);
        let eq115_e1513_q_d_n4: f64 = (p.p7 * s.dn[164][4]);
        let eq115_e1513_q_d_n5: f64 = (p.p7 * s.dn[164][5]);
        let eq115_e1513_q_d_n6: f64 = (p.p7 * s.dn[164][6]);
        let eq115_e1513_q_d_n7: f64 = (p.p7 * s.dn[164][7]);
        let eq115_e1513_q_d_n8: f64 = (p.p7 * s.dn[164][8]);
        let eq115_e1513_q_d_n9: f64 = (p.p7 * s.dn[164][9]);
        let eq115_e1513_q_d_n10: f64 = (p.p7 * s.dn[164][10]);
        let eq115_e1513_q_d_n11: f64 = (p.p7 * s.dn[164][11]);
        let eq115_e1513_q_d_n12: f64 = (p.p7 * s.dn[164][12]);
        let eq115_e1513_q_d_n13: f64 = (p.p7 * s.dn[164][13]);
        let eq115_e1513_q_d_n14: f64 = (p.p7 * s.dn[164][14]);
        let eq115_e1513_q_d_n15: f64 = (p.p7 * s.dn[164][15]);
        let eq115_e1513_q_d_n16: f64 = (p.p7 * s.dn[164][16]);
        let eq115_e1513_q_d_n17: f64 = (p.p7 * s.dn[164][17]);
        let eq115_e1513_q_d_n18: f64 = (p.p7 * s.dn[164][18]);
        let eq115_e1513_q_d_n19: f64 = (p.p7 * s.dn[164][19]);
        let eq115_e1513_q_d_n20: f64 = (p.p7 * s.dn[164][20]);
        let eq115_e1513_q_d_n21: f64 = (p.p7 * s.dn[164][21]);
        let eq115_e1513_q_d_n22: f64 = (p.p7 * s.dn[164][22]);
        let eq115_reactive_node_derivatives: [f64; 23] = [eq115_e1513_q_d_n0, eq115_e1513_q_d_n1, eq115_e1513_q_d_n2, eq115_e1513_q_d_n3, eq115_e1513_q_d_n4, eq115_e1513_q_d_n5, eq115_e1513_q_d_n6, eq115_e1513_q_d_n7, eq115_e1513_q_d_n8, eq115_e1513_q_d_n9, eq115_e1513_q_d_n10, eq115_e1513_q_d_n11, eq115_e1513_q_d_n12, eq115_e1513_q_d_n13, eq115_e1513_q_d_n14, eq115_e1513_q_d_n15, eq115_e1513_q_d_n16, eq115_e1513_q_d_n17, eq115_e1513_q_d_n18, eq115_e1513_q_d_n19, eq115_e1513_q_d_n20, eq115_e1513_q_d_n21, eq115_e1513_q_d_n22];
        let eq115_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq115_reactive_node_derivatives,
            branches,
            &eq115_reactive_branch_derivatives,
            multiplicity,
        );
        let eq116_e1516_q: f64 = s.v[219];
        let eq116_e1517: f64 = (p.p7 * s.v[219]);
        let eq116_e1517_d_n0: f64 = (p.p7 * s.dn[219][0]);
        let eq116_e1517_d_n1: f64 = (p.p7 * s.dn[219][1]);
        let eq116_e1517_d_n2: f64 = (p.p7 * s.dn[219][2]);
        let eq116_e1517_d_n3: f64 = (p.p7 * s.dn[219][3]);
        let eq116_e1517_d_n4: f64 = (p.p7 * s.dn[219][4]);
        let eq116_e1517_d_n5: f64 = (p.p7 * s.dn[219][5]);
        let eq116_e1517_d_n6: f64 = (p.p7 * s.dn[219][6]);
        let eq116_e1517_d_n7: f64 = (p.p7 * s.dn[219][7]);
        let eq116_e1517_d_n8: f64 = (p.p7 * s.dn[219][8]);
        let eq116_e1517_d_n9: f64 = (p.p7 * s.dn[219][9]);
        let eq116_e1517_d_n10: f64 = (p.p7 * s.dn[219][10]);
        let eq116_e1517_d_n11: f64 = (p.p7 * s.dn[219][11]);
        let eq116_e1517_d_n12: f64 = (p.p7 * s.dn[219][12]);
        let eq116_e1517_d_n13: f64 = (p.p7 * s.dn[219][13]);
        let eq116_e1517_d_n14: f64 = (p.p7 * s.dn[219][14]);
        let eq116_e1517_d_n15: f64 = (p.p7 * s.dn[219][15]);
        let eq116_e1517_d_n16: f64 = (p.p7 * s.dn[219][16]);
        let eq116_e1517_d_n17: f64 = (p.p7 * s.dn[219][17]);
        let eq116_e1517_d_n18: f64 = (p.p7 * s.dn[219][18]);
        let eq116_e1517_d_n19: f64 = (p.p7 * s.dn[219][19]);
        let eq116_e1517_d_n20: f64 = (p.p7 * s.dn[219][20]);
        let eq116_e1517_d_n21: f64 = (p.p7 * s.dn[219][21]);
        let eq116_e1517_d_n22: f64 = (p.p7 * s.dn[219][22]);
        let eq116_e1517_q: f64 = (p.p7 * eq116_e1516_q);
        let eq116_e1517_q_d_n0: f64 = (p.p7 * s.dn[219][0]);
        let eq116_e1517_q_d_n1: f64 = (p.p7 * s.dn[219][1]);
        let eq116_e1517_q_d_n2: f64 = (p.p7 * s.dn[219][2]);
        let eq116_e1517_q_d_n3: f64 = (p.p7 * s.dn[219][3]);
        let eq116_e1517_q_d_n4: f64 = (p.p7 * s.dn[219][4]);
        let eq116_e1517_q_d_n5: f64 = (p.p7 * s.dn[219][5]);
        let eq116_e1517_q_d_n6: f64 = (p.p7 * s.dn[219][6]);
        let eq116_e1517_q_d_n7: f64 = (p.p7 * s.dn[219][7]);
        let eq116_e1517_q_d_n8: f64 = (p.p7 * s.dn[219][8]);
        let eq116_e1517_q_d_n9: f64 = (p.p7 * s.dn[219][9]);
        let eq116_e1517_q_d_n10: f64 = (p.p7 * s.dn[219][10]);
        let eq116_e1517_q_d_n11: f64 = (p.p7 * s.dn[219][11]);
        let eq116_e1517_q_d_n12: f64 = (p.p7 * s.dn[219][12]);
        let eq116_e1517_q_d_n13: f64 = (p.p7 * s.dn[219][13]);
        let eq116_e1517_q_d_n14: f64 = (p.p7 * s.dn[219][14]);
        let eq116_e1517_q_d_n15: f64 = (p.p7 * s.dn[219][15]);
        let eq116_e1517_q_d_n16: f64 = (p.p7 * s.dn[219][16]);
        let eq116_e1517_q_d_n17: f64 = (p.p7 * s.dn[219][17]);
        let eq116_e1517_q_d_n18: f64 = (p.p7 * s.dn[219][18]);
        let eq116_e1517_q_d_n19: f64 = (p.p7 * s.dn[219][19]);
        let eq116_e1517_q_d_n20: f64 = (p.p7 * s.dn[219][20]);
        let eq116_e1517_q_d_n21: f64 = (p.p7 * s.dn[219][21]);
        let eq116_e1517_q_d_n22: f64 = (p.p7 * s.dn[219][22]);
        let eq116_reactive_node_derivatives: [f64; 23] = [eq116_e1517_q_d_n0, eq116_e1517_q_d_n1, eq116_e1517_q_d_n2, eq116_e1517_q_d_n3, eq116_e1517_q_d_n4, eq116_e1517_q_d_n5, eq116_e1517_q_d_n6, eq116_e1517_q_d_n7, eq116_e1517_q_d_n8, eq116_e1517_q_d_n9, eq116_e1517_q_d_n10, eq116_e1517_q_d_n11, eq116_e1517_q_d_n12, eq116_e1517_q_d_n13, eq116_e1517_q_d_n14, eq116_e1517_q_d_n15, eq116_e1517_q_d_n16, eq116_e1517_q_d_n17, eq116_e1517_q_d_n18, eq116_e1517_q_d_n19, eq116_e1517_q_d_n20, eq116_e1517_q_d_n21, eq116_e1517_q_d_n22];
        let eq116_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            nodes,
            &eq116_reactive_node_derivatives,
            branches,
            &eq116_reactive_branch_derivatives,
            multiplicity,
        );
        let eq117_e1520_q: f64 = s.v[220];
        let eq117_e1521: f64 = (p.p7 * s.v[220]);
        let eq117_e1521_d_n0: f64 = (p.p7 * s.dn[220][0]);
        let eq117_e1521_d_n1: f64 = (p.p7 * s.dn[220][1]);
        let eq117_e1521_d_n2: f64 = (p.p7 * s.dn[220][2]);
        let eq117_e1521_d_n3: f64 = (p.p7 * s.dn[220][3]);
        let eq117_e1521_d_n4: f64 = (p.p7 * s.dn[220][4]);
        let eq117_e1521_d_n5: f64 = (p.p7 * s.dn[220][5]);
        let eq117_e1521_d_n6: f64 = (p.p7 * s.dn[220][6]);
        let eq117_e1521_d_n7: f64 = (p.p7 * s.dn[220][7]);
        let eq117_e1521_d_n8: f64 = (p.p7 * s.dn[220][8]);
        let eq117_e1521_d_n9: f64 = (p.p7 * s.dn[220][9]);
        let eq117_e1521_d_n10: f64 = (p.p7 * s.dn[220][10]);
        let eq117_e1521_d_n11: f64 = (p.p7 * s.dn[220][11]);
        let eq117_e1521_d_n12: f64 = (p.p7 * s.dn[220][12]);
        let eq117_e1521_d_n13: f64 = (p.p7 * s.dn[220][13]);
        let eq117_e1521_d_n14: f64 = (p.p7 * s.dn[220][14]);
        let eq117_e1521_d_n15: f64 = (p.p7 * s.dn[220][15]);
        let eq117_e1521_d_n16: f64 = (p.p7 * s.dn[220][16]);
        let eq117_e1521_d_n17: f64 = (p.p7 * s.dn[220][17]);
        let eq117_e1521_d_n18: f64 = (p.p7 * s.dn[220][18]);
        let eq117_e1521_d_n19: f64 = (p.p7 * s.dn[220][19]);
        let eq117_e1521_d_n20: f64 = (p.p7 * s.dn[220][20]);
        let eq117_e1521_d_n21: f64 = (p.p7 * s.dn[220][21]);
        let eq117_e1521_d_n22: f64 = (p.p7 * s.dn[220][22]);
        let eq117_e1521_q: f64 = (p.p7 * eq117_e1520_q);
        let eq117_e1521_q_d_n0: f64 = (p.p7 * s.dn[220][0]);
        let eq117_e1521_q_d_n1: f64 = (p.p7 * s.dn[220][1]);
        let eq117_e1521_q_d_n2: f64 = (p.p7 * s.dn[220][2]);
        let eq117_e1521_q_d_n3: f64 = (p.p7 * s.dn[220][3]);
        let eq117_e1521_q_d_n4: f64 = (p.p7 * s.dn[220][4]);
        let eq117_e1521_q_d_n5: f64 = (p.p7 * s.dn[220][5]);
        let eq117_e1521_q_d_n6: f64 = (p.p7 * s.dn[220][6]);
        let eq117_e1521_q_d_n7: f64 = (p.p7 * s.dn[220][7]);
        let eq117_e1521_q_d_n8: f64 = (p.p7 * s.dn[220][8]);
        let eq117_e1521_q_d_n9: f64 = (p.p7 * s.dn[220][9]);
        let eq117_e1521_q_d_n10: f64 = (p.p7 * s.dn[220][10]);
        let eq117_e1521_q_d_n11: f64 = (p.p7 * s.dn[220][11]);
        let eq117_e1521_q_d_n12: f64 = (p.p7 * s.dn[220][12]);
        let eq117_e1521_q_d_n13: f64 = (p.p7 * s.dn[220][13]);
        let eq117_e1521_q_d_n14: f64 = (p.p7 * s.dn[220][14]);
        let eq117_e1521_q_d_n15: f64 = (p.p7 * s.dn[220][15]);
        let eq117_e1521_q_d_n16: f64 = (p.p7 * s.dn[220][16]);
        let eq117_e1521_q_d_n17: f64 = (p.p7 * s.dn[220][17]);
        let eq117_e1521_q_d_n18: f64 = (p.p7 * s.dn[220][18]);
        let eq117_e1521_q_d_n19: f64 = (p.p7 * s.dn[220][19]);
        let eq117_e1521_q_d_n20: f64 = (p.p7 * s.dn[220][20]);
        let eq117_e1521_q_d_n21: f64 = (p.p7 * s.dn[220][21]);
        let eq117_e1521_q_d_n22: f64 = (p.p7 * s.dn[220][22]);
        let eq117_reactive_node_derivatives: [f64; 23] = [eq117_e1521_q_d_n0, eq117_e1521_q_d_n1, eq117_e1521_q_d_n2, eq117_e1521_q_d_n3, eq117_e1521_q_d_n4, eq117_e1521_q_d_n5, eq117_e1521_q_d_n6, eq117_e1521_q_d_n7, eq117_e1521_q_d_n8, eq117_e1521_q_d_n9, eq117_e1521_q_d_n10, eq117_e1521_q_d_n11, eq117_e1521_q_d_n12, eq117_e1521_q_d_n13, eq117_e1521_q_d_n14, eq117_e1521_q_d_n15, eq117_e1521_q_d_n16, eq117_e1521_q_d_n17, eq117_e1521_q_d_n18, eq117_e1521_q_d_n19, eq117_e1521_q_d_n20, eq117_e1521_q_d_n21, eq117_e1521_q_d_n22];
        let eq117_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            nodes,
            &eq117_reactive_node_derivatives,
            branches,
            &eq117_reactive_branch_derivatives,
            multiplicity,
        );
        let eq118_e1524_q: f64 = s.v[221];
        let eq118_e1525: f64 = (p.p7 * s.v[221]);
        let eq118_e1525_d_n0: f64 = (p.p7 * s.dn[221][0]);
        let eq118_e1525_d_n1: f64 = (p.p7 * s.dn[221][1]);
        let eq118_e1525_d_n2: f64 = (p.p7 * s.dn[221][2]);
        let eq118_e1525_d_n3: f64 = (p.p7 * s.dn[221][3]);
        let eq118_e1525_d_n4: f64 = (p.p7 * s.dn[221][4]);
        let eq118_e1525_d_n5: f64 = (p.p7 * s.dn[221][5]);
        let eq118_e1525_d_n6: f64 = (p.p7 * s.dn[221][6]);
        let eq118_e1525_d_n7: f64 = (p.p7 * s.dn[221][7]);
        let eq118_e1525_d_n8: f64 = (p.p7 * s.dn[221][8]);
        let eq118_e1525_d_n9: f64 = (p.p7 * s.dn[221][9]);
        let eq118_e1525_d_n10: f64 = (p.p7 * s.dn[221][10]);
        let eq118_e1525_d_n11: f64 = (p.p7 * s.dn[221][11]);
        let eq118_e1525_d_n12: f64 = (p.p7 * s.dn[221][12]);
        let eq118_e1525_d_n13: f64 = (p.p7 * s.dn[221][13]);
        let eq118_e1525_d_n14: f64 = (p.p7 * s.dn[221][14]);
        let eq118_e1525_d_n15: f64 = (p.p7 * s.dn[221][15]);
        let eq118_e1525_d_n16: f64 = (p.p7 * s.dn[221][16]);
        let eq118_e1525_d_n17: f64 = (p.p7 * s.dn[221][17]);
        let eq118_e1525_d_n18: f64 = (p.p7 * s.dn[221][18]);
        let eq118_e1525_d_n19: f64 = (p.p7 * s.dn[221][19]);
        let eq118_e1525_d_n20: f64 = (p.p7 * s.dn[221][20]);
        let eq118_e1525_d_n21: f64 = (p.p7 * s.dn[221][21]);
        let eq118_e1525_d_n22: f64 = (p.p7 * s.dn[221][22]);
        let eq118_e1525_q: f64 = (p.p7 * eq118_e1524_q);
        let eq118_e1525_q_d_n0: f64 = (p.p7 * s.dn[221][0]);
        let eq118_e1525_q_d_n1: f64 = (p.p7 * s.dn[221][1]);
        let eq118_e1525_q_d_n2: f64 = (p.p7 * s.dn[221][2]);
        let eq118_e1525_q_d_n3: f64 = (p.p7 * s.dn[221][3]);
        let eq118_e1525_q_d_n4: f64 = (p.p7 * s.dn[221][4]);
        let eq118_e1525_q_d_n5: f64 = (p.p7 * s.dn[221][5]);
        let eq118_e1525_q_d_n6: f64 = (p.p7 * s.dn[221][6]);
        let eq118_e1525_q_d_n7: f64 = (p.p7 * s.dn[221][7]);
        let eq118_e1525_q_d_n8: f64 = (p.p7 * s.dn[221][8]);
        let eq118_e1525_q_d_n9: f64 = (p.p7 * s.dn[221][9]);
        let eq118_e1525_q_d_n10: f64 = (p.p7 * s.dn[221][10]);
        let eq118_e1525_q_d_n11: f64 = (p.p7 * s.dn[221][11]);
        let eq118_e1525_q_d_n12: f64 = (p.p7 * s.dn[221][12]);
        let eq118_e1525_q_d_n13: f64 = (p.p7 * s.dn[221][13]);
        let eq118_e1525_q_d_n14: f64 = (p.p7 * s.dn[221][14]);
        let eq118_e1525_q_d_n15: f64 = (p.p7 * s.dn[221][15]);
        let eq118_e1525_q_d_n16: f64 = (p.p7 * s.dn[221][16]);
        let eq118_e1525_q_d_n17: f64 = (p.p7 * s.dn[221][17]);
        let eq118_e1525_q_d_n18: f64 = (p.p7 * s.dn[221][18]);
        let eq118_e1525_q_d_n19: f64 = (p.p7 * s.dn[221][19]);
        let eq118_e1525_q_d_n20: f64 = (p.p7 * s.dn[221][20]);
        let eq118_e1525_q_d_n21: f64 = (p.p7 * s.dn[221][21]);
        let eq118_e1525_q_d_n22: f64 = (p.p7 * s.dn[221][22]);
        let eq118_reactive_node_derivatives: [f64; 23] = [eq118_e1525_q_d_n0, eq118_e1525_q_d_n1, eq118_e1525_q_d_n2, eq118_e1525_q_d_n3, eq118_e1525_q_d_n4, eq118_e1525_q_d_n5, eq118_e1525_q_d_n6, eq118_e1525_q_d_n7, eq118_e1525_q_d_n8, eq118_e1525_q_d_n9, eq118_e1525_q_d_n10, eq118_e1525_q_d_n11, eq118_e1525_q_d_n12, eq118_e1525_q_d_n13, eq118_e1525_q_d_n14, eq118_e1525_q_d_n15, eq118_e1525_q_d_n16, eq118_e1525_q_d_n17, eq118_e1525_q_d_n18, eq118_e1525_q_d_n19, eq118_e1525_q_d_n20, eq118_e1525_q_d_n21, eq118_e1525_q_d_n22];
        let eq118_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            nodes,
            &eq118_reactive_node_derivatives,
            branches,
            &eq118_reactive_branch_derivatives,
            multiplicity,
        );
        let eq119_e1529: f64 = (p.p250 * s.v[161]);
        let eq119_e1529_d_n0: f64 = (p.p250 * s.dn[161][0]);
        let eq119_e1529_d_n1: f64 = (p.p250 * s.dn[161][1]);
        let eq119_e1529_d_n2: f64 = (p.p250 * s.dn[161][2]);
        let eq119_e1529_d_n3: f64 = (p.p250 * s.dn[161][3]);
        let eq119_e1529_d_n4: f64 = (p.p250 * s.dn[161][4]);
        let eq119_e1529_d_n5: f64 = (p.p250 * s.dn[161][5]);
        let eq119_e1529_d_n6: f64 = (p.p250 * s.dn[161][6]);
        let eq119_e1529_d_n7: f64 = (p.p250 * s.dn[161][7]);
        let eq119_e1529_d_n8: f64 = (p.p250 * s.dn[161][8]);
        let eq119_e1529_d_n9: f64 = (p.p250 * s.dn[161][9]);
        let eq119_e1529_d_n10: f64 = (p.p250 * s.dn[161][10]);
        let eq119_e1529_d_n11: f64 = (p.p250 * s.dn[161][11]);
        let eq119_e1529_d_n12: f64 = (p.p250 * s.dn[161][12]);
        let eq119_e1529_d_n13: f64 = (p.p250 * s.dn[161][13]);
        let eq119_e1529_d_n14: f64 = (p.p250 * s.dn[161][14]);
        let eq119_e1529_d_n15: f64 = (p.p250 * s.dn[161][15]);
        let eq119_e1529_d_n16: f64 = (p.p250 * s.dn[161][16]);
        let eq119_e1529_d_n17: f64 = (p.p250 * s.dn[161][17]);
        let eq119_e1529_d_n18: f64 = (p.p250 * s.dn[161][18]);
        let eq119_e1529_d_n19: f64 = (p.p250 * s.dn[161][19]);
        let eq119_e1529_d_n20: f64 = (p.p250 * s.dn[161][20]);
        let eq119_e1529_d_n21: f64 = (p.p250 * s.dn[161][21]);
        let eq119_e1529_d_n22: f64 = (p.p250 * s.dn[161][22]);
        let eq119_e1530_q: f64 = eq119_e1529;
        let eq119_e1531: f64 = (p.p7 * eq119_e1529);
        let eq119_e1531_d_n0: f64 = (p.p7 * eq119_e1529_d_n0);
        let eq119_e1531_d_n1: f64 = (p.p7 * eq119_e1529_d_n1);
        let eq119_e1531_d_n2: f64 = (p.p7 * eq119_e1529_d_n2);
        let eq119_e1531_d_n3: f64 = (p.p7 * eq119_e1529_d_n3);
        let eq119_e1531_d_n4: f64 = (p.p7 * eq119_e1529_d_n4);
        let eq119_e1531_d_n5: f64 = (p.p7 * eq119_e1529_d_n5);
        let eq119_e1531_d_n6: f64 = (p.p7 * eq119_e1529_d_n6);
        let eq119_e1531_d_n7: f64 = (p.p7 * eq119_e1529_d_n7);
        let eq119_e1531_d_n8: f64 = (p.p7 * eq119_e1529_d_n8);
        let eq119_e1531_d_n9: f64 = (p.p7 * eq119_e1529_d_n9);
        let eq119_e1531_d_n10: f64 = (p.p7 * eq119_e1529_d_n10);
        let eq119_e1531_d_n11: f64 = (p.p7 * eq119_e1529_d_n11);
        let eq119_e1531_d_n12: f64 = (p.p7 * eq119_e1529_d_n12);
        let eq119_e1531_d_n13: f64 = (p.p7 * eq119_e1529_d_n13);
        let eq119_e1531_d_n14: f64 = (p.p7 * eq119_e1529_d_n14);
        let eq119_e1531_d_n15: f64 = (p.p7 * eq119_e1529_d_n15);
        let eq119_e1531_d_n16: f64 = (p.p7 * eq119_e1529_d_n16);
        let eq119_e1531_d_n17: f64 = (p.p7 * eq119_e1529_d_n17);
        let eq119_e1531_d_n18: f64 = (p.p7 * eq119_e1529_d_n18);
        let eq119_e1531_d_n19: f64 = (p.p7 * eq119_e1529_d_n19);
        let eq119_e1531_d_n20: f64 = (p.p7 * eq119_e1529_d_n20);
        let eq119_e1531_d_n21: f64 = (p.p7 * eq119_e1529_d_n21);
        let eq119_e1531_d_n22: f64 = (p.p7 * eq119_e1529_d_n22);
        let eq119_e1531_q: f64 = (p.p7 * eq119_e1530_q);
        let eq119_e1531_q_d_n0: f64 = (p.p7 * eq119_e1529_d_n0);
        let eq119_e1531_q_d_n1: f64 = (p.p7 * eq119_e1529_d_n1);
        let eq119_e1531_q_d_n2: f64 = (p.p7 * eq119_e1529_d_n2);
        let eq119_e1531_q_d_n3: f64 = (p.p7 * eq119_e1529_d_n3);
        let eq119_e1531_q_d_n4: f64 = (p.p7 * eq119_e1529_d_n4);
        let eq119_e1531_q_d_n5: f64 = (p.p7 * eq119_e1529_d_n5);
        let eq119_e1531_q_d_n6: f64 = (p.p7 * eq119_e1529_d_n6);
        let eq119_e1531_q_d_n7: f64 = (p.p7 * eq119_e1529_d_n7);
        let eq119_e1531_q_d_n8: f64 = (p.p7 * eq119_e1529_d_n8);
        let eq119_e1531_q_d_n9: f64 = (p.p7 * eq119_e1529_d_n9);
        let eq119_e1531_q_d_n10: f64 = (p.p7 * eq119_e1529_d_n10);
        let eq119_e1531_q_d_n11: f64 = (p.p7 * eq119_e1529_d_n11);
        let eq119_e1531_q_d_n12: f64 = (p.p7 * eq119_e1529_d_n12);
        let eq119_e1531_q_d_n13: f64 = (p.p7 * eq119_e1529_d_n13);
        let eq119_e1531_q_d_n14: f64 = (p.p7 * eq119_e1529_d_n14);
        let eq119_e1531_q_d_n15: f64 = (p.p7 * eq119_e1529_d_n15);
        let eq119_e1531_q_d_n16: f64 = (p.p7 * eq119_e1529_d_n16);
        let eq119_e1531_q_d_n17: f64 = (p.p7 * eq119_e1529_d_n17);
        let eq119_e1531_q_d_n18: f64 = (p.p7 * eq119_e1529_d_n18);
        let eq119_e1531_q_d_n19: f64 = (p.p7 * eq119_e1529_d_n19);
        let eq119_e1531_q_d_n20: f64 = (p.p7 * eq119_e1529_d_n20);
        let eq119_e1531_q_d_n21: f64 = (p.p7 * eq119_e1529_d_n21);
        let eq119_e1531_q_d_n22: f64 = (p.p7 * eq119_e1529_d_n22);
        let eq119_reactive_node_derivatives: [f64; 23] = [eq119_e1531_q_d_n0, eq119_e1531_q_d_n1, eq119_e1531_q_d_n2, eq119_e1531_q_d_n3, eq119_e1531_q_d_n4, eq119_e1531_q_d_n5, eq119_e1531_q_d_n6, eq119_e1531_q_d_n7, eq119_e1531_q_d_n8, eq119_e1531_q_d_n9, eq119_e1531_q_d_n10, eq119_e1531_q_d_n11, eq119_e1531_q_d_n12, eq119_e1531_q_d_n13, eq119_e1531_q_d_n14, eq119_e1531_q_d_n15, eq119_e1531_q_d_n16, eq119_e1531_q_d_n17, eq119_e1531_q_d_n18, eq119_e1531_q_d_n19, eq119_e1531_q_d_n20, eq119_e1531_q_d_n21, eq119_e1531_q_d_n22];
        let eq119_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq119_reactive_node_derivatives,
            branches,
            &eq119_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22, eq120_e1540_q, eq120_e1540_q_d_n0, eq120_e1540_q_d_n1, eq120_e1540_q_d_n2, eq120_e1540_q_d_n3, eq120_e1540_q_d_n4, eq120_e1540_q_d_n5, eq120_e1540_q_d_n6, eq120_e1540_q_d_n7, eq120_e1540_q_d_n8, eq120_e1540_q_d_n9, eq120_e1540_q_d_n10, eq120_e1540_q_d_n11, eq120_e1540_q_d_n12, eq120_e1540_q_d_n13, eq120_e1540_q_d_n14, eq120_e1540_q_d_n15, eq120_e1540_q_d_n16, eq120_e1540_q_d_n17, eq120_e1540_q_d_n18, eq120_e1540_q_d_n19, eq120_e1540_q_d_n20, eq120_e1540_q_d_n21, eq120_e1540_q_d_n22,) = {
    if (s.b[570] && s.b[571]) {
        let eq120_e1537_q: f64 = s.v[229];
        let eq120_e1538: f64 = (p.p7 * s.v[229]);
        let eq120_e1538_d_n0: f64 = (p.p7 * s.dn[229][0]);
        let eq120_e1538_d_n1: f64 = (p.p7 * s.dn[229][1]);
        let eq120_e1538_d_n2: f64 = (p.p7 * s.dn[229][2]);
        let eq120_e1538_d_n3: f64 = (p.p7 * s.dn[229][3]);
        let eq120_e1538_d_n4: f64 = (p.p7 * s.dn[229][4]);
        let eq120_e1538_d_n5: f64 = (p.p7 * s.dn[229][5]);
        let eq120_e1538_d_n6: f64 = (p.p7 * s.dn[229][6]);
        let eq120_e1538_d_n7: f64 = (p.p7 * s.dn[229][7]);
        let eq120_e1538_d_n8: f64 = (p.p7 * s.dn[229][8]);
        let eq120_e1538_d_n9: f64 = (p.p7 * s.dn[229][9]);
        let eq120_e1538_d_n10: f64 = (p.p7 * s.dn[229][10]);
        let eq120_e1538_d_n11: f64 = (p.p7 * s.dn[229][11]);
        let eq120_e1538_d_n12: f64 = (p.p7 * s.dn[229][12]);
        let eq120_e1538_d_n13: f64 = (p.p7 * s.dn[229][13]);
        let eq120_e1538_d_n14: f64 = (p.p7 * s.dn[229][14]);
        let eq120_e1538_d_n15: f64 = (p.p7 * s.dn[229][15]);
        let eq120_e1538_d_n16: f64 = (p.p7 * s.dn[229][16]);
        let eq120_e1538_d_n17: f64 = (p.p7 * s.dn[229][17]);
        let eq120_e1538_d_n18: f64 = (p.p7 * s.dn[229][18]);
        let eq120_e1538_d_n19: f64 = (p.p7 * s.dn[229][19]);
        let eq120_e1538_d_n20: f64 = (p.p7 * s.dn[229][20]);
        let eq120_e1538_d_n21: f64 = (p.p7 * s.dn[229][21]);
        let eq120_e1538_d_n22: f64 = (p.p7 * s.dn[229][22]);
        let eq120_e1538_q: f64 = (p.p7 * eq120_e1537_q);
        let eq120_e1538_q_d_n0: f64 = (p.p7 * s.dn[229][0]);
        let eq120_e1538_q_d_n1: f64 = (p.p7 * s.dn[229][1]);
        let eq120_e1538_q_d_n2: f64 = (p.p7 * s.dn[229][2]);
        let eq120_e1538_q_d_n3: f64 = (p.p7 * s.dn[229][3]);
        let eq120_e1538_q_d_n4: f64 = (p.p7 * s.dn[229][4]);
        let eq120_e1538_q_d_n5: f64 = (p.p7 * s.dn[229][5]);
        let eq120_e1538_q_d_n6: f64 = (p.p7 * s.dn[229][6]);
        let eq120_e1538_q_d_n7: f64 = (p.p7 * s.dn[229][7]);
        let eq120_e1538_q_d_n8: f64 = (p.p7 * s.dn[229][8]);
        let eq120_e1538_q_d_n9: f64 = (p.p7 * s.dn[229][9]);
        let eq120_e1538_q_d_n10: f64 = (p.p7 * s.dn[229][10]);
        let eq120_e1538_q_d_n11: f64 = (p.p7 * s.dn[229][11]);
        let eq120_e1538_q_d_n12: f64 = (p.p7 * s.dn[229][12]);
        let eq120_e1538_q_d_n13: f64 = (p.p7 * s.dn[229][13]);
        let eq120_e1538_q_d_n14: f64 = (p.p7 * s.dn[229][14]);
        let eq120_e1538_q_d_n15: f64 = (p.p7 * s.dn[229][15]);
        let eq120_e1538_q_d_n16: f64 = (p.p7 * s.dn[229][16]);
        let eq120_e1538_q_d_n17: f64 = (p.p7 * s.dn[229][17]);
        let eq120_e1538_q_d_n18: f64 = (p.p7 * s.dn[229][18]);
        let eq120_e1538_q_d_n19: f64 = (p.p7 * s.dn[229][19]);
        let eq120_e1538_q_d_n20: f64 = (p.p7 * s.dn[229][20]);
        let eq120_e1538_q_d_n21: f64 = (p.p7 * s.dn[229][21]);
        let eq120_e1538_q_d_n22: f64 = (p.p7 * s.dn[229][22]);
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n10, eq120_e1538_d_n11, eq120_e1538_d_n12, eq120_e1538_d_n13, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22, eq120_e1538_q, eq120_e1538_q_d_n0, eq120_e1538_q_d_n1, eq120_e1538_q_d_n2, eq120_e1538_q_d_n3, eq120_e1538_q_d_n4, eq120_e1538_q_d_n5, eq120_e1538_q_d_n6, eq120_e1538_q_d_n7, eq120_e1538_q_d_n8, eq120_e1538_q_d_n9, eq120_e1538_q_d_n10, eq120_e1538_q_d_n11, eq120_e1538_q_d_n12, eq120_e1538_q_d_n13, eq120_e1538_q_d_n14, eq120_e1538_q_d_n15, eq120_e1538_q_d_n16, eq120_e1538_q_d_n17, eq120_e1538_q_d_n18, eq120_e1538_q_d_n19, eq120_e1538_q_d_n20, eq120_e1538_q_d_n21, eq120_e1538_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_reactive_node_derivatives: [f64; 23] = [eq120_e1540_q_d_n0, eq120_e1540_q_d_n1, eq120_e1540_q_d_n2, eq120_e1540_q_d_n3, eq120_e1540_q_d_n4, eq120_e1540_q_d_n5, eq120_e1540_q_d_n6, eq120_e1540_q_d_n7, eq120_e1540_q_d_n8, eq120_e1540_q_d_n9, eq120_e1540_q_d_n10, eq120_e1540_q_d_n11, eq120_e1540_q_d_n12, eq120_e1540_q_d_n13, eq120_e1540_q_d_n14, eq120_e1540_q_d_n15, eq120_e1540_q_d_n16, eq120_e1540_q_d_n17, eq120_e1540_q_d_n18, eq120_e1540_q_d_n19, eq120_e1540_q_d_n20, eq120_e1540_q_d_n21, eq120_e1540_q_d_n22];
        let eq120_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            nodes,
            &eq120_reactive_node_derivatives,
            branches,
            &eq120_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22, eq121_e1551_q, eq121_e1551_q_d_n0, eq121_e1551_q_d_n1, eq121_e1551_q_d_n2, eq121_e1551_q_d_n3, eq121_e1551_q_d_n4, eq121_e1551_q_d_n5, eq121_e1551_q_d_n6, eq121_e1551_q_d_n7, eq121_e1551_q_d_n8, eq121_e1551_q_d_n9, eq121_e1551_q_d_n10, eq121_e1551_q_d_n11, eq121_e1551_q_d_n12, eq121_e1551_q_d_n13, eq121_e1551_q_d_n14, eq121_e1551_q_d_n15, eq121_e1551_q_d_n16, eq121_e1551_q_d_n17, eq121_e1551_q_d_n18, eq121_e1551_q_d_n19, eq121_e1551_q_d_n20, eq121_e1551_q_d_n21, eq121_e1551_q_d_n22,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq121_e1548_q: f64 = s.v[228];
        let eq121_e1549: f64 = (p.p7 * s.v[228]);
        let eq121_e1549_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq121_e1549_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq121_e1549_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq121_e1549_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq121_e1549_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq121_e1549_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq121_e1549_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq121_e1549_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq121_e1549_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq121_e1549_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq121_e1549_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq121_e1549_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq121_e1549_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq121_e1549_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq121_e1549_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq121_e1549_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq121_e1549_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq121_e1549_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq121_e1549_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq121_e1549_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq121_e1549_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq121_e1549_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq121_e1549_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq121_e1549_q: f64 = (p.p7 * eq121_e1548_q);
        let eq121_e1549_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq121_e1549_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq121_e1549_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq121_e1549_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq121_e1549_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq121_e1549_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq121_e1549_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq121_e1549_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq121_e1549_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq121_e1549_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq121_e1549_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq121_e1549_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq121_e1549_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq121_e1549_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq121_e1549_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq121_e1549_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq121_e1549_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq121_e1549_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq121_e1549_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq121_e1549_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq121_e1549_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq121_e1549_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq121_e1549_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq121_e1549, eq121_e1549_d_n0, eq121_e1549_d_n1, eq121_e1549_d_n2, eq121_e1549_d_n3, eq121_e1549_d_n4, eq121_e1549_d_n5, eq121_e1549_d_n6, eq121_e1549_d_n7, eq121_e1549_d_n8, eq121_e1549_d_n9, eq121_e1549_d_n10, eq121_e1549_d_n11, eq121_e1549_d_n12, eq121_e1549_d_n13, eq121_e1549_d_n14, eq121_e1549_d_n15, eq121_e1549_d_n16, eq121_e1549_d_n17, eq121_e1549_d_n18, eq121_e1549_d_n19, eq121_e1549_d_n20, eq121_e1549_d_n21, eq121_e1549_d_n22, eq121_e1549_q, eq121_e1549_q_d_n0, eq121_e1549_q_d_n1, eq121_e1549_q_d_n2, eq121_e1549_q_d_n3, eq121_e1549_q_d_n4, eq121_e1549_q_d_n5, eq121_e1549_q_d_n6, eq121_e1549_q_d_n7, eq121_e1549_q_d_n8, eq121_e1549_q_d_n9, eq121_e1549_q_d_n10, eq121_e1549_q_d_n11, eq121_e1549_q_d_n12, eq121_e1549_q_d_n13, eq121_e1549_q_d_n14, eq121_e1549_q_d_n15, eq121_e1549_q_d_n16, eq121_e1549_q_d_n17, eq121_e1549_q_d_n18, eq121_e1549_q_d_n19, eq121_e1549_q_d_n20, eq121_e1549_q_d_n21, eq121_e1549_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_reactive_node_derivatives: [f64; 23] = [eq121_e1551_q_d_n0, eq121_e1551_q_d_n1, eq121_e1551_q_d_n2, eq121_e1551_q_d_n3, eq121_e1551_q_d_n4, eq121_e1551_q_d_n5, eq121_e1551_q_d_n6, eq121_e1551_q_d_n7, eq121_e1551_q_d_n8, eq121_e1551_q_d_n9, eq121_e1551_q_d_n10, eq121_e1551_q_d_n11, eq121_e1551_q_d_n12, eq121_e1551_q_d_n13, eq121_e1551_q_d_n14, eq121_e1551_q_d_n15, eq121_e1551_q_d_n16, eq121_e1551_q_d_n17, eq121_e1551_q_d_n18, eq121_e1551_q_d_n19, eq121_e1551_q_d_n20, eq121_e1551_q_d_n21, eq121_e1551_q_d_n22];
        let eq121_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22, eq122_e1564_q, eq122_e1564_q_d_n0, eq122_e1564_q_d_n1, eq122_e1564_q_d_n2, eq122_e1564_q_d_n3, eq122_e1564_q_d_n4, eq122_e1564_q_d_n5, eq122_e1564_q_d_n6, eq122_e1564_q_d_n7, eq122_e1564_q_d_n8, eq122_e1564_q_d_n9, eq122_e1564_q_d_n10, eq122_e1564_q_d_n11, eq122_e1564_q_d_n12, eq122_e1564_q_d_n13, eq122_e1564_q_d_n14, eq122_e1564_q_d_n15, eq122_e1564_q_d_n16, eq122_e1564_q_d_n17, eq122_e1564_q_d_n18, eq122_e1564_q_d_n19, eq122_e1564_q_d_n20, eq122_e1564_q_d_n21, eq122_e1564_q_d_n22,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq122_e1559_q: f64 = s.v[228];
        let eq122_e1560: f64 = (p.p7 * s.v[228]);
        let eq122_e1560_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq122_e1560_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq122_e1560_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq122_e1560_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq122_e1560_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq122_e1560_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq122_e1560_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq122_e1560_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq122_e1560_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq122_e1560_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq122_e1560_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq122_e1560_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq122_e1560_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq122_e1560_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq122_e1560_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq122_e1560_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq122_e1560_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq122_e1560_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq122_e1560_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq122_e1560_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq122_e1560_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq122_e1560_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq122_e1560_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq122_e1560_q: f64 = (p.p7 * eq122_e1559_q);
        let eq122_e1560_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq122_e1560_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq122_e1560_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq122_e1560_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq122_e1560_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq122_e1560_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq122_e1560_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq122_e1560_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq122_e1560_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq122_e1560_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq122_e1560_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq122_e1560_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq122_e1560_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq122_e1560_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq122_e1560_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq122_e1560_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq122_e1560_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq122_e1560_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq122_e1560_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq122_e1560_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq122_e1560_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq122_e1560_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq122_e1560_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_d_n0: f64 = (eq122_e1560_d_n0 * p.p246);
        let eq122_e1562_d_n1: f64 = (eq122_e1560_d_n1 * p.p246);
        let eq122_e1562_d_n2: f64 = (eq122_e1560_d_n2 * p.p246);
        let eq122_e1562_d_n3: f64 = (eq122_e1560_d_n3 * p.p246);
        let eq122_e1562_d_n4: f64 = (eq122_e1560_d_n4 * p.p246);
        let eq122_e1562_d_n5: f64 = (eq122_e1560_d_n5 * p.p246);
        let eq122_e1562_d_n6: f64 = (eq122_e1560_d_n6 * p.p246);
        let eq122_e1562_d_n7: f64 = (eq122_e1560_d_n7 * p.p246);
        let eq122_e1562_d_n8: f64 = (eq122_e1560_d_n8 * p.p246);
        let eq122_e1562_d_n9: f64 = (eq122_e1560_d_n9 * p.p246);
        let eq122_e1562_d_n10: f64 = (eq122_e1560_d_n10 * p.p246);
        let eq122_e1562_d_n11: f64 = (eq122_e1560_d_n11 * p.p246);
        let eq122_e1562_d_n12: f64 = (eq122_e1560_d_n12 * p.p246);
        let eq122_e1562_d_n13: f64 = (eq122_e1560_d_n13 * p.p246);
        let eq122_e1562_d_n14: f64 = (eq122_e1560_d_n14 * p.p246);
        let eq122_e1562_d_n15: f64 = (eq122_e1560_d_n15 * p.p246);
        let eq122_e1562_d_n16: f64 = (eq122_e1560_d_n16 * p.p246);
        let eq122_e1562_d_n17: f64 = (eq122_e1560_d_n17 * p.p246);
        let eq122_e1562_d_n18: f64 = (eq122_e1560_d_n18 * p.p246);
        let eq122_e1562_d_n19: f64 = (eq122_e1560_d_n19 * p.p246);
        let eq122_e1562_d_n20: f64 = (eq122_e1560_d_n20 * p.p246);
        let eq122_e1562_d_n21: f64 = (eq122_e1560_d_n21 * p.p246);
        let eq122_e1562_d_n22: f64 = (eq122_e1560_d_n22 * p.p246);
        let eq122_e1562_q: f64 = (eq122_e1560_q * p.p246);
        let eq122_e1562_q_d_n0: f64 = (eq122_e1560_q_d_n0 * p.p246);
        let eq122_e1562_q_d_n1: f64 = (eq122_e1560_q_d_n1 * p.p246);
        let eq122_e1562_q_d_n2: f64 = (eq122_e1560_q_d_n2 * p.p246);
        let eq122_e1562_q_d_n3: f64 = (eq122_e1560_q_d_n3 * p.p246);
        let eq122_e1562_q_d_n4: f64 = (eq122_e1560_q_d_n4 * p.p246);
        let eq122_e1562_q_d_n5: f64 = (eq122_e1560_q_d_n5 * p.p246);
        let eq122_e1562_q_d_n6: f64 = (eq122_e1560_q_d_n6 * p.p246);
        let eq122_e1562_q_d_n7: f64 = (eq122_e1560_q_d_n7 * p.p246);
        let eq122_e1562_q_d_n8: f64 = (eq122_e1560_q_d_n8 * p.p246);
        let eq122_e1562_q_d_n9: f64 = (eq122_e1560_q_d_n9 * p.p246);
        let eq122_e1562_q_d_n10: f64 = (eq122_e1560_q_d_n10 * p.p246);
        let eq122_e1562_q_d_n11: f64 = (eq122_e1560_q_d_n11 * p.p246);
        let eq122_e1562_q_d_n12: f64 = (eq122_e1560_q_d_n12 * p.p246);
        let eq122_e1562_q_d_n13: f64 = (eq122_e1560_q_d_n13 * p.p246);
        let eq122_e1562_q_d_n14: f64 = (eq122_e1560_q_d_n14 * p.p246);
        let eq122_e1562_q_d_n15: f64 = (eq122_e1560_q_d_n15 * p.p246);
        let eq122_e1562_q_d_n16: f64 = (eq122_e1560_q_d_n16 * p.p246);
        let eq122_e1562_q_d_n17: f64 = (eq122_e1560_q_d_n17 * p.p246);
        let eq122_e1562_q_d_n18: f64 = (eq122_e1560_q_d_n18 * p.p246);
        let eq122_e1562_q_d_n19: f64 = (eq122_e1560_q_d_n19 * p.p246);
        let eq122_e1562_q_d_n20: f64 = (eq122_e1560_q_d_n20 * p.p246);
        let eq122_e1562_q_d_n21: f64 = (eq122_e1560_q_d_n21 * p.p246);
        let eq122_e1562_q_d_n22: f64 = (eq122_e1560_q_d_n22 * p.p246);
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n10, eq122_e1562_d_n11, eq122_e1562_d_n12, eq122_e1562_d_n13, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22, eq122_e1562_q, eq122_e1562_q_d_n0, eq122_e1562_q_d_n1, eq122_e1562_q_d_n2, eq122_e1562_q_d_n3, eq122_e1562_q_d_n4, eq122_e1562_q_d_n5, eq122_e1562_q_d_n6, eq122_e1562_q_d_n7, eq122_e1562_q_d_n8, eq122_e1562_q_d_n9, eq122_e1562_q_d_n10, eq122_e1562_q_d_n11, eq122_e1562_q_d_n12, eq122_e1562_q_d_n13, eq122_e1562_q_d_n14, eq122_e1562_q_d_n15, eq122_e1562_q_d_n16, eq122_e1562_q_d_n17, eq122_e1562_q_d_n18, eq122_e1562_q_d_n19, eq122_e1562_q_d_n20, eq122_e1562_q_d_n21, eq122_e1562_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_reactive_node_derivatives: [f64; 23] = [eq122_e1564_q_d_n0, eq122_e1564_q_d_n1, eq122_e1564_q_d_n2, eq122_e1564_q_d_n3, eq122_e1564_q_d_n4, eq122_e1564_q_d_n5, eq122_e1564_q_d_n6, eq122_e1564_q_d_n7, eq122_e1564_q_d_n8, eq122_e1564_q_d_n9, eq122_e1564_q_d_n10, eq122_e1564_q_d_n11, eq122_e1564_q_d_n12, eq122_e1564_q_d_n13, eq122_e1564_q_d_n14, eq122_e1564_q_d_n15, eq122_e1564_q_d_n16, eq122_e1564_q_d_n17, eq122_e1564_q_d_n18, eq122_e1564_q_d_n19, eq122_e1564_q_d_n20, eq122_e1564_q_d_n21, eq122_e1564_q_d_n22];
        let eq122_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq122_reactive_node_derivatives,
            branches,
            &eq122_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_q, eq123_e1576_q_d_n0, eq123_e1576_q_d_n1, eq123_e1576_q_d_n2, eq123_e1576_q_d_n3, eq123_e1576_q_d_n4, eq123_e1576_q_d_n5, eq123_e1576_q_d_n6, eq123_e1576_q_d_n7, eq123_e1576_q_d_n8, eq123_e1576_q_d_n9, eq123_e1576_q_d_n10, eq123_e1576_q_d_n11, eq123_e1576_q_d_n12, eq123_e1576_q_d_n13, eq123_e1576_q_d_n14, eq123_e1576_q_d_n15, eq123_e1576_q_d_n16, eq123_e1576_q_d_n17, eq123_e1576_q_d_n18, eq123_e1576_q_d_n19, eq123_e1576_q_d_n20, eq123_e1576_q_d_n21, eq123_e1576_q_d_n22,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq123_e1573_q: f64 = s.v[228];
        let eq123_e1574: f64 = (p.p7 * s.v[228]);
        let eq123_e1574_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq123_e1574_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq123_e1574_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq123_e1574_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq123_e1574_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq123_e1574_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq123_e1574_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq123_e1574_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq123_e1574_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq123_e1574_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq123_e1574_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq123_e1574_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq123_e1574_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq123_e1574_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq123_e1574_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq123_e1574_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq123_e1574_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq123_e1574_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq123_e1574_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq123_e1574_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq123_e1574_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq123_e1574_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq123_e1574_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq123_e1574_q: f64 = (p.p7 * eq123_e1573_q);
        let eq123_e1574_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq123_e1574_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq123_e1574_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq123_e1574_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq123_e1574_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq123_e1574_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq123_e1574_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq123_e1574_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq123_e1574_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq123_e1574_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq123_e1574_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq123_e1574_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq123_e1574_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq123_e1574_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq123_e1574_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq123_e1574_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq123_e1574_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq123_e1574_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq123_e1574_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq123_e1574_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq123_e1574_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq123_e1574_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq123_e1574_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq123_e1574, eq123_e1574_d_n0, eq123_e1574_d_n1, eq123_e1574_d_n2, eq123_e1574_d_n3, eq123_e1574_d_n4, eq123_e1574_d_n5, eq123_e1574_d_n6, eq123_e1574_d_n7, eq123_e1574_d_n8, eq123_e1574_d_n9, eq123_e1574_d_n10, eq123_e1574_d_n11, eq123_e1574_d_n12, eq123_e1574_d_n13, eq123_e1574_d_n14, eq123_e1574_d_n15, eq123_e1574_d_n16, eq123_e1574_d_n17, eq123_e1574_d_n18, eq123_e1574_d_n19, eq123_e1574_d_n20, eq123_e1574_d_n21, eq123_e1574_d_n22, eq123_e1574_q, eq123_e1574_q_d_n0, eq123_e1574_q_d_n1, eq123_e1574_q_d_n2, eq123_e1574_q_d_n3, eq123_e1574_q_d_n4, eq123_e1574_q_d_n5, eq123_e1574_q_d_n6, eq123_e1574_q_d_n7, eq123_e1574_q_d_n8, eq123_e1574_q_d_n9, eq123_e1574_q_d_n10, eq123_e1574_q_d_n11, eq123_e1574_q_d_n12, eq123_e1574_q_d_n13, eq123_e1574_q_d_n14, eq123_e1574_q_d_n15, eq123_e1574_q_d_n16, eq123_e1574_q_d_n17, eq123_e1574_q_d_n18, eq123_e1574_q_d_n19, eq123_e1574_q_d_n20, eq123_e1574_q_d_n21, eq123_e1574_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_reactive_node_derivatives: [f64; 23] = [eq123_e1576_q_d_n0, eq123_e1576_q_d_n1, eq123_e1576_q_d_n2, eq123_e1576_q_d_n3, eq123_e1576_q_d_n4, eq123_e1576_q_d_n5, eq123_e1576_q_d_n6, eq123_e1576_q_d_n7, eq123_e1576_q_d_n8, eq123_e1576_q_d_n9, eq123_e1576_q_d_n10, eq123_e1576_q_d_n11, eq123_e1576_q_d_n12, eq123_e1576_q_d_n13, eq123_e1576_q_d_n14, eq123_e1576_q_d_n15, eq123_e1576_q_d_n16, eq123_e1576_q_d_n17, eq123_e1576_q_d_n18, eq123_e1576_q_d_n19, eq123_e1576_q_d_n20, eq123_e1576_q_d_n21, eq123_e1576_q_d_n22];
        let eq123_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq123_reactive_node_derivatives,
            branches,
            &eq123_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_q, eq124_e1590_q_d_n0, eq124_e1590_q_d_n1, eq124_e1590_q_d_n2, eq124_e1590_q_d_n3, eq124_e1590_q_d_n4, eq124_e1590_q_d_n5, eq124_e1590_q_d_n6, eq124_e1590_q_d_n7, eq124_e1590_q_d_n8, eq124_e1590_q_d_n9, eq124_e1590_q_d_n10, eq124_e1590_q_d_n11, eq124_e1590_q_d_n12, eq124_e1590_q_d_n13, eq124_e1590_q_d_n14, eq124_e1590_q_d_n15, eq124_e1590_q_d_n16, eq124_e1590_q_d_n17, eq124_e1590_q_d_n18, eq124_e1590_q_d_n19, eq124_e1590_q_d_n20, eq124_e1590_q_d_n21, eq124_e1590_q_d_n22,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq124_e1585_q: f64 = s.v[228];
        let eq124_e1586: f64 = (p.p7 * s.v[228]);
        let eq124_e1586_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq124_e1586_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq124_e1586_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq124_e1586_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq124_e1586_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq124_e1586_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq124_e1586_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq124_e1586_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq124_e1586_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq124_e1586_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq124_e1586_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq124_e1586_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq124_e1586_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq124_e1586_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq124_e1586_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq124_e1586_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq124_e1586_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq124_e1586_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq124_e1586_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq124_e1586_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq124_e1586_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq124_e1586_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq124_e1586_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq124_e1586_q: f64 = (p.p7 * eq124_e1585_q);
        let eq124_e1586_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq124_e1586_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq124_e1586_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq124_e1586_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq124_e1586_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq124_e1586_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq124_e1586_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq124_e1586_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq124_e1586_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq124_e1586_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq124_e1586_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq124_e1586_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq124_e1586_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq124_e1586_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq124_e1586_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq124_e1586_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq124_e1586_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq124_e1586_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq124_e1586_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq124_e1586_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq124_e1586_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq124_e1586_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq124_e1586_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_d_n0: f64 = (eq124_e1586_d_n0 * p.p246);
        let eq124_e1588_d_n1: f64 = (eq124_e1586_d_n1 * p.p246);
        let eq124_e1588_d_n2: f64 = (eq124_e1586_d_n2 * p.p246);
        let eq124_e1588_d_n3: f64 = (eq124_e1586_d_n3 * p.p246);
        let eq124_e1588_d_n4: f64 = (eq124_e1586_d_n4 * p.p246);
        let eq124_e1588_d_n5: f64 = (eq124_e1586_d_n5 * p.p246);
        let eq124_e1588_d_n6: f64 = (eq124_e1586_d_n6 * p.p246);
        let eq124_e1588_d_n7: f64 = (eq124_e1586_d_n7 * p.p246);
        let eq124_e1588_d_n8: f64 = (eq124_e1586_d_n8 * p.p246);
        let eq124_e1588_d_n9: f64 = (eq124_e1586_d_n9 * p.p246);
        let eq124_e1588_d_n10: f64 = (eq124_e1586_d_n10 * p.p246);
        let eq124_e1588_d_n11: f64 = (eq124_e1586_d_n11 * p.p246);
        let eq124_e1588_d_n12: f64 = (eq124_e1586_d_n12 * p.p246);
        let eq124_e1588_d_n13: f64 = (eq124_e1586_d_n13 * p.p246);
        let eq124_e1588_d_n14: f64 = (eq124_e1586_d_n14 * p.p246);
        let eq124_e1588_d_n15: f64 = (eq124_e1586_d_n15 * p.p246);
        let eq124_e1588_d_n16: f64 = (eq124_e1586_d_n16 * p.p246);
        let eq124_e1588_d_n17: f64 = (eq124_e1586_d_n17 * p.p246);
        let eq124_e1588_d_n18: f64 = (eq124_e1586_d_n18 * p.p246);
        let eq124_e1588_d_n19: f64 = (eq124_e1586_d_n19 * p.p246);
        let eq124_e1588_d_n20: f64 = (eq124_e1586_d_n20 * p.p246);
        let eq124_e1588_d_n21: f64 = (eq124_e1586_d_n21 * p.p246);
        let eq124_e1588_d_n22: f64 = (eq124_e1586_d_n22 * p.p246);
        let eq124_e1588_q: f64 = (eq124_e1586_q * p.p246);
        let eq124_e1588_q_d_n0: f64 = (eq124_e1586_q_d_n0 * p.p246);
        let eq124_e1588_q_d_n1: f64 = (eq124_e1586_q_d_n1 * p.p246);
        let eq124_e1588_q_d_n2: f64 = (eq124_e1586_q_d_n2 * p.p246);
        let eq124_e1588_q_d_n3: f64 = (eq124_e1586_q_d_n3 * p.p246);
        let eq124_e1588_q_d_n4: f64 = (eq124_e1586_q_d_n4 * p.p246);
        let eq124_e1588_q_d_n5: f64 = (eq124_e1586_q_d_n5 * p.p246);
        let eq124_e1588_q_d_n6: f64 = (eq124_e1586_q_d_n6 * p.p246);
        let eq124_e1588_q_d_n7: f64 = (eq124_e1586_q_d_n7 * p.p246);
        let eq124_e1588_q_d_n8: f64 = (eq124_e1586_q_d_n8 * p.p246);
        let eq124_e1588_q_d_n9: f64 = (eq124_e1586_q_d_n9 * p.p246);
        let eq124_e1588_q_d_n10: f64 = (eq124_e1586_q_d_n10 * p.p246);
        let eq124_e1588_q_d_n11: f64 = (eq124_e1586_q_d_n11 * p.p246);
        let eq124_e1588_q_d_n12: f64 = (eq124_e1586_q_d_n12 * p.p246);
        let eq124_e1588_q_d_n13: f64 = (eq124_e1586_q_d_n13 * p.p246);
        let eq124_e1588_q_d_n14: f64 = (eq124_e1586_q_d_n14 * p.p246);
        let eq124_e1588_q_d_n15: f64 = (eq124_e1586_q_d_n15 * p.p246);
        let eq124_e1588_q_d_n16: f64 = (eq124_e1586_q_d_n16 * p.p246);
        let eq124_e1588_q_d_n17: f64 = (eq124_e1586_q_d_n17 * p.p246);
        let eq124_e1588_q_d_n18: f64 = (eq124_e1586_q_d_n18 * p.p246);
        let eq124_e1588_q_d_n19: f64 = (eq124_e1586_q_d_n19 * p.p246);
        let eq124_e1588_q_d_n20: f64 = (eq124_e1586_q_d_n20 * p.p246);
        let eq124_e1588_q_d_n21: f64 = (eq124_e1586_q_d_n21 * p.p246);
        let eq124_e1588_q_d_n22: f64 = (eq124_e1586_q_d_n22 * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22, eq124_e1588_q, eq124_e1588_q_d_n0, eq124_e1588_q_d_n1, eq124_e1588_q_d_n2, eq124_e1588_q_d_n3, eq124_e1588_q_d_n4, eq124_e1588_q_d_n5, eq124_e1588_q_d_n6, eq124_e1588_q_d_n7, eq124_e1588_q_d_n8, eq124_e1588_q_d_n9, eq124_e1588_q_d_n10, eq124_e1588_q_d_n11, eq124_e1588_q_d_n12, eq124_e1588_q_d_n13, eq124_e1588_q_d_n14, eq124_e1588_q_d_n15, eq124_e1588_q_d_n16, eq124_e1588_q_d_n17, eq124_e1588_q_d_n18, eq124_e1588_q_d_n19, eq124_e1588_q_d_n20, eq124_e1588_q_d_n21, eq124_e1588_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 23] = [eq124_e1590_q_d_n0, eq124_e1590_q_d_n1, eq124_e1590_q_d_n2, eq124_e1590_q_d_n3, eq124_e1590_q_d_n4, eq124_e1590_q_d_n5, eq124_e1590_q_d_n6, eq124_e1590_q_d_n7, eq124_e1590_q_d_n8, eq124_e1590_q_d_n9, eq124_e1590_q_d_n10, eq124_e1590_q_d_n11, eq124_e1590_q_d_n12, eq124_e1590_q_d_n13, eq124_e1590_q_d_n14, eq124_e1590_q_d_n15, eq124_e1590_q_d_n16, eq124_e1590_q_d_n17, eq124_e1590_q_d_n18, eq124_e1590_q_d_n19, eq124_e1590_q_d_n20, eq124_e1590_q_d_n21, eq124_e1590_q_d_n22];
        let eq124_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_q, eq125_e1601_q_d_n0, eq125_e1601_q_d_n1, eq125_e1601_q_d_n2, eq125_e1601_q_d_n3, eq125_e1601_q_d_n4, eq125_e1601_q_d_n5, eq125_e1601_q_d_n6, eq125_e1601_q_d_n7, eq125_e1601_q_d_n8, eq125_e1601_q_d_n9, eq125_e1601_q_d_n10, eq125_e1601_q_d_n11, eq125_e1601_q_d_n12, eq125_e1601_q_d_n13, eq125_e1601_q_d_n14, eq125_e1601_q_d_n15, eq125_e1601_q_d_n16, eq125_e1601_q_d_n17, eq125_e1601_q_d_n18, eq125_e1601_q_d_n19, eq125_e1601_q_d_n20, eq125_e1601_q_d_n21, eq125_e1601_q_d_n22,) = {
    if (s.b[570] && s.b[571]) {
        let eq125_e1597: f64 = (p.p251 * s.v[228]);
        let eq125_e1597_d_n0: f64 = (p.p251 * s.dn[228][0]);
        let eq125_e1597_d_n1: f64 = (p.p251 * s.dn[228][1]);
        let eq125_e1597_d_n2: f64 = (p.p251 * s.dn[228][2]);
        let eq125_e1597_d_n3: f64 = (p.p251 * s.dn[228][3]);
        let eq125_e1597_d_n4: f64 = (p.p251 * s.dn[228][4]);
        let eq125_e1597_d_n5: f64 = (p.p251 * s.dn[228][5]);
        let eq125_e1597_d_n6: f64 = (p.p251 * s.dn[228][6]);
        let eq125_e1597_d_n7: f64 = (p.p251 * s.dn[228][7]);
        let eq125_e1597_d_n8: f64 = (p.p251 * s.dn[228][8]);
        let eq125_e1597_d_n9: f64 = (p.p251 * s.dn[228][9]);
        let eq125_e1597_d_n10: f64 = (p.p251 * s.dn[228][10]);
        let eq125_e1597_d_n11: f64 = (p.p251 * s.dn[228][11]);
        let eq125_e1597_d_n12: f64 = (p.p251 * s.dn[228][12]);
        let eq125_e1597_d_n13: f64 = (p.p251 * s.dn[228][13]);
        let eq125_e1597_d_n14: f64 = (p.p251 * s.dn[228][14]);
        let eq125_e1597_d_n15: f64 = (p.p251 * s.dn[228][15]);
        let eq125_e1597_d_n16: f64 = (p.p251 * s.dn[228][16]);
        let eq125_e1597_d_n17: f64 = (p.p251 * s.dn[228][17]);
        let eq125_e1597_d_n18: f64 = (p.p251 * s.dn[228][18]);
        let eq125_e1597_d_n19: f64 = (p.p251 * s.dn[228][19]);
        let eq125_e1597_d_n20: f64 = (p.p251 * s.dn[228][20]);
        let eq125_e1597_d_n21: f64 = (p.p251 * s.dn[228][21]);
        let eq125_e1597_d_n22: f64 = (p.p251 * s.dn[228][22]);
        let eq125_e1598_q: f64 = eq125_e1597;
        let eq125_e1599: f64 = (p.p7 * eq125_e1597);
        let eq125_e1599_d_n0: f64 = (p.p7 * eq125_e1597_d_n0);
        let eq125_e1599_d_n1: f64 = (p.p7 * eq125_e1597_d_n1);
        let eq125_e1599_d_n2: f64 = (p.p7 * eq125_e1597_d_n2);
        let eq125_e1599_d_n3: f64 = (p.p7 * eq125_e1597_d_n3);
        let eq125_e1599_d_n4: f64 = (p.p7 * eq125_e1597_d_n4);
        let eq125_e1599_d_n5: f64 = (p.p7 * eq125_e1597_d_n5);
        let eq125_e1599_d_n6: f64 = (p.p7 * eq125_e1597_d_n6);
        let eq125_e1599_d_n7: f64 = (p.p7 * eq125_e1597_d_n7);
        let eq125_e1599_d_n8: f64 = (p.p7 * eq125_e1597_d_n8);
        let eq125_e1599_d_n9: f64 = (p.p7 * eq125_e1597_d_n9);
        let eq125_e1599_d_n10: f64 = (p.p7 * eq125_e1597_d_n10);
        let eq125_e1599_d_n11: f64 = (p.p7 * eq125_e1597_d_n11);
        let eq125_e1599_d_n12: f64 = (p.p7 * eq125_e1597_d_n12);
        let eq125_e1599_d_n13: f64 = (p.p7 * eq125_e1597_d_n13);
        let eq125_e1599_d_n14: f64 = (p.p7 * eq125_e1597_d_n14);
        let eq125_e1599_d_n15: f64 = (p.p7 * eq125_e1597_d_n15);
        let eq125_e1599_d_n16: f64 = (p.p7 * eq125_e1597_d_n16);
        let eq125_e1599_d_n17: f64 = (p.p7 * eq125_e1597_d_n17);
        let eq125_e1599_d_n18: f64 = (p.p7 * eq125_e1597_d_n18);
        let eq125_e1599_d_n19: f64 = (p.p7 * eq125_e1597_d_n19);
        let eq125_e1599_d_n20: f64 = (p.p7 * eq125_e1597_d_n20);
        let eq125_e1599_d_n21: f64 = (p.p7 * eq125_e1597_d_n21);
        let eq125_e1599_d_n22: f64 = (p.p7 * eq125_e1597_d_n22);
        let eq125_e1599_q: f64 = (p.p7 * eq125_e1598_q);
        let eq125_e1599_q_d_n0: f64 = (p.p7 * eq125_e1597_d_n0);
        let eq125_e1599_q_d_n1: f64 = (p.p7 * eq125_e1597_d_n1);
        let eq125_e1599_q_d_n2: f64 = (p.p7 * eq125_e1597_d_n2);
        let eq125_e1599_q_d_n3: f64 = (p.p7 * eq125_e1597_d_n3);
        let eq125_e1599_q_d_n4: f64 = (p.p7 * eq125_e1597_d_n4);
        let eq125_e1599_q_d_n5: f64 = (p.p7 * eq125_e1597_d_n5);
        let eq125_e1599_q_d_n6: f64 = (p.p7 * eq125_e1597_d_n6);
        let eq125_e1599_q_d_n7: f64 = (p.p7 * eq125_e1597_d_n7);
        let eq125_e1599_q_d_n8: f64 = (p.p7 * eq125_e1597_d_n8);
        let eq125_e1599_q_d_n9: f64 = (p.p7 * eq125_e1597_d_n9);
        let eq125_e1599_q_d_n10: f64 = (p.p7 * eq125_e1597_d_n10);
        let eq125_e1599_q_d_n11: f64 = (p.p7 * eq125_e1597_d_n11);
        let eq125_e1599_q_d_n12: f64 = (p.p7 * eq125_e1597_d_n12);
        let eq125_e1599_q_d_n13: f64 = (p.p7 * eq125_e1597_d_n13);
        let eq125_e1599_q_d_n14: f64 = (p.p7 * eq125_e1597_d_n14);
        let eq125_e1599_q_d_n15: f64 = (p.p7 * eq125_e1597_d_n15);
        let eq125_e1599_q_d_n16: f64 = (p.p7 * eq125_e1597_d_n16);
        let eq125_e1599_q_d_n17: f64 = (p.p7 * eq125_e1597_d_n17);
        let eq125_e1599_q_d_n18: f64 = (p.p7 * eq125_e1597_d_n18);
        let eq125_e1599_q_d_n19: f64 = (p.p7 * eq125_e1597_d_n19);
        let eq125_e1599_q_d_n20: f64 = (p.p7 * eq125_e1597_d_n20);
        let eq125_e1599_q_d_n21: f64 = (p.p7 * eq125_e1597_d_n21);
        let eq125_e1599_q_d_n22: f64 = (p.p7 * eq125_e1597_d_n22);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_q, eq125_e1599_q_d_n0, eq125_e1599_q_d_n1, eq125_e1599_q_d_n2, eq125_e1599_q_d_n3, eq125_e1599_q_d_n4, eq125_e1599_q_d_n5, eq125_e1599_q_d_n6, eq125_e1599_q_d_n7, eq125_e1599_q_d_n8, eq125_e1599_q_d_n9, eq125_e1599_q_d_n10, eq125_e1599_q_d_n11, eq125_e1599_q_d_n12, eq125_e1599_q_d_n13, eq125_e1599_q_d_n14, eq125_e1599_q_d_n15, eq125_e1599_q_d_n16, eq125_e1599_q_d_n17, eq125_e1599_q_d_n18, eq125_e1599_q_d_n19, eq125_e1599_q_d_n20, eq125_e1599_q_d_n21, eq125_e1599_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 23] = [eq125_e1601_q_d_n0, eq125_e1601_q_d_n1, eq125_e1601_q_d_n2, eq125_e1601_q_d_n3, eq125_e1601_q_d_n4, eq125_e1601_q_d_n5, eq125_e1601_q_d_n6, eq125_e1601_q_d_n7, eq125_e1601_q_d_n8, eq125_e1601_q_d_n9, eq125_e1601_q_d_n10, eq125_e1601_q_d_n11, eq125_e1601_q_d_n12, eq125_e1601_q_d_n13, eq125_e1601_q_d_n14, eq125_e1601_q_d_n15, eq125_e1601_q_d_n16, eq125_e1601_q_d_n17, eq125_e1601_q_d_n18, eq125_e1601_q_d_n19, eq125_e1601_q_d_n20, eq125_e1601_q_d_n21, eq125_e1601_q_d_n22];
        let eq125_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_q, eq126_e1611_q_d_n0, eq126_e1611_q_d_n1, eq126_e1611_q_d_n2, eq126_e1611_q_d_n3, eq126_e1611_q_d_n4, eq126_e1611_q_d_n5, eq126_e1611_q_d_n6, eq126_e1611_q_d_n7, eq126_e1611_q_d_n8, eq126_e1611_q_d_n9, eq126_e1611_q_d_n10, eq126_e1611_q_d_n11, eq126_e1611_q_d_n12, eq126_e1611_q_d_n13, eq126_e1611_q_d_n14, eq126_e1611_q_d_n15, eq126_e1611_q_d_n16, eq126_e1611_q_d_n17, eq126_e1611_q_d_n18, eq126_e1611_q_d_n19, eq126_e1611_q_d_n20, eq126_e1611_q_d_n21, eq126_e1611_q_d_n22,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq126_e1608_q: f64 = s.v[229];
        let eq126_e1609: f64 = (p.p7 * s.v[229]);
        let eq126_e1609_d_n0: f64 = (p.p7 * s.dn[229][0]);
        let eq126_e1609_d_n1: f64 = (p.p7 * s.dn[229][1]);
        let eq126_e1609_d_n2: f64 = (p.p7 * s.dn[229][2]);
        let eq126_e1609_d_n3: f64 = (p.p7 * s.dn[229][3]);
        let eq126_e1609_d_n4: f64 = (p.p7 * s.dn[229][4]);
        let eq126_e1609_d_n5: f64 = (p.p7 * s.dn[229][5]);
        let eq126_e1609_d_n6: f64 = (p.p7 * s.dn[229][6]);
        let eq126_e1609_d_n7: f64 = (p.p7 * s.dn[229][7]);
        let eq126_e1609_d_n8: f64 = (p.p7 * s.dn[229][8]);
        let eq126_e1609_d_n9: f64 = (p.p7 * s.dn[229][9]);
        let eq126_e1609_d_n10: f64 = (p.p7 * s.dn[229][10]);
        let eq126_e1609_d_n11: f64 = (p.p7 * s.dn[229][11]);
        let eq126_e1609_d_n12: f64 = (p.p7 * s.dn[229][12]);
        let eq126_e1609_d_n13: f64 = (p.p7 * s.dn[229][13]);
        let eq126_e1609_d_n14: f64 = (p.p7 * s.dn[229][14]);
        let eq126_e1609_d_n15: f64 = (p.p7 * s.dn[229][15]);
        let eq126_e1609_d_n16: f64 = (p.p7 * s.dn[229][16]);
        let eq126_e1609_d_n17: f64 = (p.p7 * s.dn[229][17]);
        let eq126_e1609_d_n18: f64 = (p.p7 * s.dn[229][18]);
        let eq126_e1609_d_n19: f64 = (p.p7 * s.dn[229][19]);
        let eq126_e1609_d_n20: f64 = (p.p7 * s.dn[229][20]);
        let eq126_e1609_d_n21: f64 = (p.p7 * s.dn[229][21]);
        let eq126_e1609_d_n22: f64 = (p.p7 * s.dn[229][22]);
        let eq126_e1609_q: f64 = (p.p7 * eq126_e1608_q);
        let eq126_e1609_q_d_n0: f64 = (p.p7 * s.dn[229][0]);
        let eq126_e1609_q_d_n1: f64 = (p.p7 * s.dn[229][1]);
        let eq126_e1609_q_d_n2: f64 = (p.p7 * s.dn[229][2]);
        let eq126_e1609_q_d_n3: f64 = (p.p7 * s.dn[229][3]);
        let eq126_e1609_q_d_n4: f64 = (p.p7 * s.dn[229][4]);
        let eq126_e1609_q_d_n5: f64 = (p.p7 * s.dn[229][5]);
        let eq126_e1609_q_d_n6: f64 = (p.p7 * s.dn[229][6]);
        let eq126_e1609_q_d_n7: f64 = (p.p7 * s.dn[229][7]);
        let eq126_e1609_q_d_n8: f64 = (p.p7 * s.dn[229][8]);
        let eq126_e1609_q_d_n9: f64 = (p.p7 * s.dn[229][9]);
        let eq126_e1609_q_d_n10: f64 = (p.p7 * s.dn[229][10]);
        let eq126_e1609_q_d_n11: f64 = (p.p7 * s.dn[229][11]);
        let eq126_e1609_q_d_n12: f64 = (p.p7 * s.dn[229][12]);
        let eq126_e1609_q_d_n13: f64 = (p.p7 * s.dn[229][13]);
        let eq126_e1609_q_d_n14: f64 = (p.p7 * s.dn[229][14]);
        let eq126_e1609_q_d_n15: f64 = (p.p7 * s.dn[229][15]);
        let eq126_e1609_q_d_n16: f64 = (p.p7 * s.dn[229][16]);
        let eq126_e1609_q_d_n17: f64 = (p.p7 * s.dn[229][17]);
        let eq126_e1609_q_d_n18: f64 = (p.p7 * s.dn[229][18]);
        let eq126_e1609_q_d_n19: f64 = (p.p7 * s.dn[229][19]);
        let eq126_e1609_q_d_n20: f64 = (p.p7 * s.dn[229][20]);
        let eq126_e1609_q_d_n21: f64 = (p.p7 * s.dn[229][21]);
        let eq126_e1609_q_d_n22: f64 = (p.p7 * s.dn[229][22]);
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22, eq126_e1609_q, eq126_e1609_q_d_n0, eq126_e1609_q_d_n1, eq126_e1609_q_d_n2, eq126_e1609_q_d_n3, eq126_e1609_q_d_n4, eq126_e1609_q_d_n5, eq126_e1609_q_d_n6, eq126_e1609_q_d_n7, eq126_e1609_q_d_n8, eq126_e1609_q_d_n9, eq126_e1609_q_d_n10, eq126_e1609_q_d_n11, eq126_e1609_q_d_n12, eq126_e1609_q_d_n13, eq126_e1609_q_d_n14, eq126_e1609_q_d_n15, eq126_e1609_q_d_n16, eq126_e1609_q_d_n17, eq126_e1609_q_d_n18, eq126_e1609_q_d_n19, eq126_e1609_q_d_n20, eq126_e1609_q_d_n21, eq126_e1609_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 23] = [eq126_e1611_q_d_n0, eq126_e1611_q_d_n1, eq126_e1611_q_d_n2, eq126_e1611_q_d_n3, eq126_e1611_q_d_n4, eq126_e1611_q_d_n5, eq126_e1611_q_d_n6, eq126_e1611_q_d_n7, eq126_e1611_q_d_n8, eq126_e1611_q_d_n9, eq126_e1611_q_d_n10, eq126_e1611_q_d_n11, eq126_e1611_q_d_n12, eq126_e1611_q_d_n13, eq126_e1611_q_d_n14, eq126_e1611_q_d_n15, eq126_e1611_q_d_n16, eq126_e1611_q_d_n17, eq126_e1611_q_d_n18, eq126_e1611_q_d_n19, eq126_e1611_q_d_n20, eq126_e1611_q_d_n21, eq126_e1611_q_d_n22];
        let eq126_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_q, eq127_e1623_q_d_n0, eq127_e1623_q_d_n1, eq127_e1623_q_d_n2, eq127_e1623_q_d_n3, eq127_e1623_q_d_n4, eq127_e1623_q_d_n5, eq127_e1623_q_d_n6, eq127_e1623_q_d_n7, eq127_e1623_q_d_n8, eq127_e1623_q_d_n9, eq127_e1623_q_d_n10, eq127_e1623_q_d_n11, eq127_e1623_q_d_n12, eq127_e1623_q_d_n13, eq127_e1623_q_d_n14, eq127_e1623_q_d_n15, eq127_e1623_q_d_n16, eq127_e1623_q_d_n17, eq127_e1623_q_d_n18, eq127_e1623_q_d_n19, eq127_e1623_q_d_n20, eq127_e1623_q_d_n21, eq127_e1623_q_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq127_e1620_q: f64 = s.v[228];
        let eq127_e1621: f64 = (p.p7 * s.v[228]);
        let eq127_e1621_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq127_e1621_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq127_e1621_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq127_e1621_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq127_e1621_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq127_e1621_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq127_e1621_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq127_e1621_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq127_e1621_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq127_e1621_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq127_e1621_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq127_e1621_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq127_e1621_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq127_e1621_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq127_e1621_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq127_e1621_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq127_e1621_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq127_e1621_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq127_e1621_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq127_e1621_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq127_e1621_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq127_e1621_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq127_e1621_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq127_e1621_q: f64 = (p.p7 * eq127_e1620_q);
        let eq127_e1621_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq127_e1621_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq127_e1621_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq127_e1621_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq127_e1621_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq127_e1621_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq127_e1621_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq127_e1621_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq127_e1621_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq127_e1621_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq127_e1621_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq127_e1621_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq127_e1621_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq127_e1621_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq127_e1621_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq127_e1621_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq127_e1621_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq127_e1621_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq127_e1621_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq127_e1621_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq127_e1621_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq127_e1621_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq127_e1621_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq127_e1621, eq127_e1621_d_n0, eq127_e1621_d_n1, eq127_e1621_d_n2, eq127_e1621_d_n3, eq127_e1621_d_n4, eq127_e1621_d_n5, eq127_e1621_d_n6, eq127_e1621_d_n7, eq127_e1621_d_n8, eq127_e1621_d_n9, eq127_e1621_d_n10, eq127_e1621_d_n11, eq127_e1621_d_n12, eq127_e1621_d_n13, eq127_e1621_d_n14, eq127_e1621_d_n15, eq127_e1621_d_n16, eq127_e1621_d_n17, eq127_e1621_d_n18, eq127_e1621_d_n19, eq127_e1621_d_n20, eq127_e1621_d_n21, eq127_e1621_d_n22, eq127_e1621_q, eq127_e1621_q_d_n0, eq127_e1621_q_d_n1, eq127_e1621_q_d_n2, eq127_e1621_q_d_n3, eq127_e1621_q_d_n4, eq127_e1621_q_d_n5, eq127_e1621_q_d_n6, eq127_e1621_q_d_n7, eq127_e1621_q_d_n8, eq127_e1621_q_d_n9, eq127_e1621_q_d_n10, eq127_e1621_q_d_n11, eq127_e1621_q_d_n12, eq127_e1621_q_d_n13, eq127_e1621_q_d_n14, eq127_e1621_q_d_n15, eq127_e1621_q_d_n16, eq127_e1621_q_d_n17, eq127_e1621_q_d_n18, eq127_e1621_q_d_n19, eq127_e1621_q_d_n20, eq127_e1621_q_d_n21, eq127_e1621_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_reactive_node_derivatives: [f64; 23] = [eq127_e1623_q_d_n0, eq127_e1623_q_d_n1, eq127_e1623_q_d_n2, eq127_e1623_q_d_n3, eq127_e1623_q_d_n4, eq127_e1623_q_d_n5, eq127_e1623_q_d_n6, eq127_e1623_q_d_n7, eq127_e1623_q_d_n8, eq127_e1623_q_d_n9, eq127_e1623_q_d_n10, eq127_e1623_q_d_n11, eq127_e1623_q_d_n12, eq127_e1623_q_d_n13, eq127_e1623_q_d_n14, eq127_e1623_q_d_n15, eq127_e1623_q_d_n16, eq127_e1623_q_d_n17, eq127_e1623_q_d_n18, eq127_e1623_q_d_n19, eq127_e1623_q_d_n20, eq127_e1623_q_d_n21, eq127_e1623_q_d_n22];
        let eq127_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq127_reactive_node_derivatives,
            branches,
            &eq127_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_q, eq128_e1637_q_d_n0, eq128_e1637_q_d_n1, eq128_e1637_q_d_n2, eq128_e1637_q_d_n3, eq128_e1637_q_d_n4, eq128_e1637_q_d_n5, eq128_e1637_q_d_n6, eq128_e1637_q_d_n7, eq128_e1637_q_d_n8, eq128_e1637_q_d_n9, eq128_e1637_q_d_n10, eq128_e1637_q_d_n11, eq128_e1637_q_d_n12, eq128_e1637_q_d_n13, eq128_e1637_q_d_n14, eq128_e1637_q_d_n15, eq128_e1637_q_d_n16, eq128_e1637_q_d_n17, eq128_e1637_q_d_n18, eq128_e1637_q_d_n19, eq128_e1637_q_d_n20, eq128_e1637_q_d_n21, eq128_e1637_q_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq128_e1632_q: f64 = s.v[228];
        let eq128_e1633: f64 = (p.p7 * s.v[228]);
        let eq128_e1633_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq128_e1633_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq128_e1633_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq128_e1633_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq128_e1633_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq128_e1633_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq128_e1633_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq128_e1633_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq128_e1633_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq128_e1633_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq128_e1633_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq128_e1633_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq128_e1633_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq128_e1633_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq128_e1633_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq128_e1633_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq128_e1633_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq128_e1633_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq128_e1633_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq128_e1633_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq128_e1633_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq128_e1633_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq128_e1633_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq128_e1633_q: f64 = (p.p7 * eq128_e1632_q);
        let eq128_e1633_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq128_e1633_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq128_e1633_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq128_e1633_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq128_e1633_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq128_e1633_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq128_e1633_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq128_e1633_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq128_e1633_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq128_e1633_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq128_e1633_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq128_e1633_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq128_e1633_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq128_e1633_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq128_e1633_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq128_e1633_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq128_e1633_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq128_e1633_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq128_e1633_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq128_e1633_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq128_e1633_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq128_e1633_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq128_e1633_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (eq128_e1633_d_n0 * p.p246);
        let eq128_e1635_d_n1: f64 = (eq128_e1633_d_n1 * p.p246);
        let eq128_e1635_d_n2: f64 = (eq128_e1633_d_n2 * p.p246);
        let eq128_e1635_d_n3: f64 = (eq128_e1633_d_n3 * p.p246);
        let eq128_e1635_d_n4: f64 = (eq128_e1633_d_n4 * p.p246);
        let eq128_e1635_d_n5: f64 = (eq128_e1633_d_n5 * p.p246);
        let eq128_e1635_d_n6: f64 = (eq128_e1633_d_n6 * p.p246);
        let eq128_e1635_d_n7: f64 = (eq128_e1633_d_n7 * p.p246);
        let eq128_e1635_d_n8: f64 = (eq128_e1633_d_n8 * p.p246);
        let eq128_e1635_d_n9: f64 = (eq128_e1633_d_n9 * p.p246);
        let eq128_e1635_d_n10: f64 = (eq128_e1633_d_n10 * p.p246);
        let eq128_e1635_d_n11: f64 = (eq128_e1633_d_n11 * p.p246);
        let eq128_e1635_d_n12: f64 = (eq128_e1633_d_n12 * p.p246);
        let eq128_e1635_d_n13: f64 = (eq128_e1633_d_n13 * p.p246);
        let eq128_e1635_d_n14: f64 = (eq128_e1633_d_n14 * p.p246);
        let eq128_e1635_d_n15: f64 = (eq128_e1633_d_n15 * p.p246);
        let eq128_e1635_d_n16: f64 = (eq128_e1633_d_n16 * p.p246);
        let eq128_e1635_d_n17: f64 = (eq128_e1633_d_n17 * p.p246);
        let eq128_e1635_d_n18: f64 = (eq128_e1633_d_n18 * p.p246);
        let eq128_e1635_d_n19: f64 = (eq128_e1633_d_n19 * p.p246);
        let eq128_e1635_d_n20: f64 = (eq128_e1633_d_n20 * p.p246);
        let eq128_e1635_d_n21: f64 = (eq128_e1633_d_n21 * p.p246);
        let eq128_e1635_d_n22: f64 = (eq128_e1633_d_n22 * p.p246);
        let eq128_e1635_q: f64 = (eq128_e1633_q * p.p246);
        let eq128_e1635_q_d_n0: f64 = (eq128_e1633_q_d_n0 * p.p246);
        let eq128_e1635_q_d_n1: f64 = (eq128_e1633_q_d_n1 * p.p246);
        let eq128_e1635_q_d_n2: f64 = (eq128_e1633_q_d_n2 * p.p246);
        let eq128_e1635_q_d_n3: f64 = (eq128_e1633_q_d_n3 * p.p246);
        let eq128_e1635_q_d_n4: f64 = (eq128_e1633_q_d_n4 * p.p246);
        let eq128_e1635_q_d_n5: f64 = (eq128_e1633_q_d_n5 * p.p246);
        let eq128_e1635_q_d_n6: f64 = (eq128_e1633_q_d_n6 * p.p246);
        let eq128_e1635_q_d_n7: f64 = (eq128_e1633_q_d_n7 * p.p246);
        let eq128_e1635_q_d_n8: f64 = (eq128_e1633_q_d_n8 * p.p246);
        let eq128_e1635_q_d_n9: f64 = (eq128_e1633_q_d_n9 * p.p246);
        let eq128_e1635_q_d_n10: f64 = (eq128_e1633_q_d_n10 * p.p246);
        let eq128_e1635_q_d_n11: f64 = (eq128_e1633_q_d_n11 * p.p246);
        let eq128_e1635_q_d_n12: f64 = (eq128_e1633_q_d_n12 * p.p246);
        let eq128_e1635_q_d_n13: f64 = (eq128_e1633_q_d_n13 * p.p246);
        let eq128_e1635_q_d_n14: f64 = (eq128_e1633_q_d_n14 * p.p246);
        let eq128_e1635_q_d_n15: f64 = (eq128_e1633_q_d_n15 * p.p246);
        let eq128_e1635_q_d_n16: f64 = (eq128_e1633_q_d_n16 * p.p246);
        let eq128_e1635_q_d_n17: f64 = (eq128_e1633_q_d_n17 * p.p246);
        let eq128_e1635_q_d_n18: f64 = (eq128_e1633_q_d_n18 * p.p246);
        let eq128_e1635_q_d_n19: f64 = (eq128_e1633_q_d_n19 * p.p246);
        let eq128_e1635_q_d_n20: f64 = (eq128_e1633_q_d_n20 * p.p246);
        let eq128_e1635_q_d_n21: f64 = (eq128_e1633_q_d_n21 * p.p246);
        let eq128_e1635_q_d_n22: f64 = (eq128_e1633_q_d_n22 * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22, eq128_e1635_q, eq128_e1635_q_d_n0, eq128_e1635_q_d_n1, eq128_e1635_q_d_n2, eq128_e1635_q_d_n3, eq128_e1635_q_d_n4, eq128_e1635_q_d_n5, eq128_e1635_q_d_n6, eq128_e1635_q_d_n7, eq128_e1635_q_d_n8, eq128_e1635_q_d_n9, eq128_e1635_q_d_n10, eq128_e1635_q_d_n11, eq128_e1635_q_d_n12, eq128_e1635_q_d_n13, eq128_e1635_q_d_n14, eq128_e1635_q_d_n15, eq128_e1635_q_d_n16, eq128_e1635_q_d_n17, eq128_e1635_q_d_n18, eq128_e1635_q_d_n19, eq128_e1635_q_d_n20, eq128_e1635_q_d_n21, eq128_e1635_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 23] = [eq128_e1637_q_d_n0, eq128_e1637_q_d_n1, eq128_e1637_q_d_n2, eq128_e1637_q_d_n3, eq128_e1637_q_d_n4, eq128_e1637_q_d_n5, eq128_e1637_q_d_n6, eq128_e1637_q_d_n7, eq128_e1637_q_d_n8, eq128_e1637_q_d_n9, eq128_e1637_q_d_n10, eq128_e1637_q_d_n11, eq128_e1637_q_d_n12, eq128_e1637_q_d_n13, eq128_e1637_q_d_n14, eq128_e1637_q_d_n15, eq128_e1637_q_d_n16, eq128_e1637_q_d_n17, eq128_e1637_q_d_n18, eq128_e1637_q_d_n19, eq128_e1637_q_d_n20, eq128_e1637_q_d_n21, eq128_e1637_q_d_n22];
        let eq128_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_q, eq129_e1650_q_d_n0, eq129_e1650_q_d_n1, eq129_e1650_q_d_n2, eq129_e1650_q_d_n3, eq129_e1650_q_d_n4, eq129_e1650_q_d_n5, eq129_e1650_q_d_n6, eq129_e1650_q_d_n7, eq129_e1650_q_d_n8, eq129_e1650_q_d_n9, eq129_e1650_q_d_n10, eq129_e1650_q_d_n11, eq129_e1650_q_d_n12, eq129_e1650_q_d_n13, eq129_e1650_q_d_n14, eq129_e1650_q_d_n15, eq129_e1650_q_d_n16, eq129_e1650_q_d_n17, eq129_e1650_q_d_n18, eq129_e1650_q_d_n19, eq129_e1650_q_d_n20, eq129_e1650_q_d_n21, eq129_e1650_q_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq129_e1647_q: f64 = s.v[228];
        let eq129_e1648: f64 = (p.p7 * s.v[228]);
        let eq129_e1648_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq129_e1648_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq129_e1648_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq129_e1648_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq129_e1648_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq129_e1648_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq129_e1648_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq129_e1648_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq129_e1648_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq129_e1648_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq129_e1648_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq129_e1648_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq129_e1648_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq129_e1648_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq129_e1648_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq129_e1648_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq129_e1648_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq129_e1648_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq129_e1648_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq129_e1648_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq129_e1648_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq129_e1648_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq129_e1648_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq129_e1648_q: f64 = (p.p7 * eq129_e1647_q);
        let eq129_e1648_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq129_e1648_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq129_e1648_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq129_e1648_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq129_e1648_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq129_e1648_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq129_e1648_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq129_e1648_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq129_e1648_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq129_e1648_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq129_e1648_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq129_e1648_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq129_e1648_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq129_e1648_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq129_e1648_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq129_e1648_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq129_e1648_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq129_e1648_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq129_e1648_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq129_e1648_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq129_e1648_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq129_e1648_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq129_e1648_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        (eq129_e1648, eq129_e1648_d_n0, eq129_e1648_d_n1, eq129_e1648_d_n2, eq129_e1648_d_n3, eq129_e1648_d_n4, eq129_e1648_d_n5, eq129_e1648_d_n6, eq129_e1648_d_n7, eq129_e1648_d_n8, eq129_e1648_d_n9, eq129_e1648_d_n10, eq129_e1648_d_n11, eq129_e1648_d_n12, eq129_e1648_d_n13, eq129_e1648_d_n14, eq129_e1648_d_n15, eq129_e1648_d_n16, eq129_e1648_d_n17, eq129_e1648_d_n18, eq129_e1648_d_n19, eq129_e1648_d_n20, eq129_e1648_d_n21, eq129_e1648_d_n22, eq129_e1648_q, eq129_e1648_q_d_n0, eq129_e1648_q_d_n1, eq129_e1648_q_d_n2, eq129_e1648_q_d_n3, eq129_e1648_q_d_n4, eq129_e1648_q_d_n5, eq129_e1648_q_d_n6, eq129_e1648_q_d_n7, eq129_e1648_q_d_n8, eq129_e1648_q_d_n9, eq129_e1648_q_d_n10, eq129_e1648_q_d_n11, eq129_e1648_q_d_n12, eq129_e1648_q_d_n13, eq129_e1648_q_d_n14, eq129_e1648_q_d_n15, eq129_e1648_q_d_n16, eq129_e1648_q_d_n17, eq129_e1648_q_d_n18, eq129_e1648_q_d_n19, eq129_e1648_q_d_n20, eq129_e1648_q_d_n21, eq129_e1648_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 23] = [eq129_e1650_q_d_n0, eq129_e1650_q_d_n1, eq129_e1650_q_d_n2, eq129_e1650_q_d_n3, eq129_e1650_q_d_n4, eq129_e1650_q_d_n5, eq129_e1650_q_d_n6, eq129_e1650_q_d_n7, eq129_e1650_q_d_n8, eq129_e1650_q_d_n9, eq129_e1650_q_d_n10, eq129_e1650_q_d_n11, eq129_e1650_q_d_n12, eq129_e1650_q_d_n13, eq129_e1650_q_d_n14, eq129_e1650_q_d_n15, eq129_e1650_q_d_n16, eq129_e1650_q_d_n17, eq129_e1650_q_d_n18, eq129_e1650_q_d_n19, eq129_e1650_q_d_n20, eq129_e1650_q_d_n21, eq129_e1650_q_d_n22];
        let eq129_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq129_reactive_node_derivatives,
            branches,
            &eq129_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_4(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_q, eq130_e1665_q_d_n0, eq130_e1665_q_d_n1, eq130_e1665_q_d_n2, eq130_e1665_q_d_n3, eq130_e1665_q_d_n4, eq130_e1665_q_d_n5, eq130_e1665_q_d_n6, eq130_e1665_q_d_n7, eq130_e1665_q_d_n8, eq130_e1665_q_d_n9, eq130_e1665_q_d_n10, eq130_e1665_q_d_n11, eq130_e1665_q_d_n12, eq130_e1665_q_d_n13, eq130_e1665_q_d_n14, eq130_e1665_q_d_n15, eq130_e1665_q_d_n16, eq130_e1665_q_d_n17, eq130_e1665_q_d_n18, eq130_e1665_q_d_n19, eq130_e1665_q_d_n20, eq130_e1665_q_d_n21, eq130_e1665_q_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq130_e1660_q: f64 = s.v[228];
        let eq130_e1661: f64 = (p.p7 * s.v[228]);
        let eq130_e1661_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq130_e1661_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq130_e1661_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq130_e1661_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq130_e1661_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq130_e1661_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq130_e1661_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq130_e1661_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq130_e1661_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq130_e1661_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq130_e1661_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq130_e1661_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq130_e1661_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq130_e1661_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq130_e1661_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq130_e1661_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq130_e1661_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq130_e1661_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq130_e1661_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq130_e1661_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq130_e1661_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq130_e1661_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq130_e1661_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq130_e1661_q: f64 = (p.p7 * eq130_e1660_q);
        let eq130_e1661_q_d_n0: f64 = (p.p7 * s.dn[228][0]);
        let eq130_e1661_q_d_n1: f64 = (p.p7 * s.dn[228][1]);
        let eq130_e1661_q_d_n2: f64 = (p.p7 * s.dn[228][2]);
        let eq130_e1661_q_d_n3: f64 = (p.p7 * s.dn[228][3]);
        let eq130_e1661_q_d_n4: f64 = (p.p7 * s.dn[228][4]);
        let eq130_e1661_q_d_n5: f64 = (p.p7 * s.dn[228][5]);
        let eq130_e1661_q_d_n6: f64 = (p.p7 * s.dn[228][6]);
        let eq130_e1661_q_d_n7: f64 = (p.p7 * s.dn[228][7]);
        let eq130_e1661_q_d_n8: f64 = (p.p7 * s.dn[228][8]);
        let eq130_e1661_q_d_n9: f64 = (p.p7 * s.dn[228][9]);
        let eq130_e1661_q_d_n10: f64 = (p.p7 * s.dn[228][10]);
        let eq130_e1661_q_d_n11: f64 = (p.p7 * s.dn[228][11]);
        let eq130_e1661_q_d_n12: f64 = (p.p7 * s.dn[228][12]);
        let eq130_e1661_q_d_n13: f64 = (p.p7 * s.dn[228][13]);
        let eq130_e1661_q_d_n14: f64 = (p.p7 * s.dn[228][14]);
        let eq130_e1661_q_d_n15: f64 = (p.p7 * s.dn[228][15]);
        let eq130_e1661_q_d_n16: f64 = (p.p7 * s.dn[228][16]);
        let eq130_e1661_q_d_n17: f64 = (p.p7 * s.dn[228][17]);
        let eq130_e1661_q_d_n18: f64 = (p.p7 * s.dn[228][18]);
        let eq130_e1661_q_d_n19: f64 = (p.p7 * s.dn[228][19]);
        let eq130_e1661_q_d_n20: f64 = (p.p7 * s.dn[228][20]);
        let eq130_e1661_q_d_n21: f64 = (p.p7 * s.dn[228][21]);
        let eq130_e1661_q_d_n22: f64 = (p.p7 * s.dn[228][22]);
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = (eq130_e1661_d_n0 * p.p246);
        let eq130_e1663_d_n1: f64 = (eq130_e1661_d_n1 * p.p246);
        let eq130_e1663_d_n2: f64 = (eq130_e1661_d_n2 * p.p246);
        let eq130_e1663_d_n3: f64 = (eq130_e1661_d_n3 * p.p246);
        let eq130_e1663_d_n4: f64 = (eq130_e1661_d_n4 * p.p246);
        let eq130_e1663_d_n5: f64 = (eq130_e1661_d_n5 * p.p246);
        let eq130_e1663_d_n6: f64 = (eq130_e1661_d_n6 * p.p246);
        let eq130_e1663_d_n7: f64 = (eq130_e1661_d_n7 * p.p246);
        let eq130_e1663_d_n8: f64 = (eq130_e1661_d_n8 * p.p246);
        let eq130_e1663_d_n9: f64 = (eq130_e1661_d_n9 * p.p246);
        let eq130_e1663_d_n10: f64 = (eq130_e1661_d_n10 * p.p246);
        let eq130_e1663_d_n11: f64 = (eq130_e1661_d_n11 * p.p246);
        let eq130_e1663_d_n12: f64 = (eq130_e1661_d_n12 * p.p246);
        let eq130_e1663_d_n13: f64 = (eq130_e1661_d_n13 * p.p246);
        let eq130_e1663_d_n14: f64 = (eq130_e1661_d_n14 * p.p246);
        let eq130_e1663_d_n15: f64 = (eq130_e1661_d_n15 * p.p246);
        let eq130_e1663_d_n16: f64 = (eq130_e1661_d_n16 * p.p246);
        let eq130_e1663_d_n17: f64 = (eq130_e1661_d_n17 * p.p246);
        let eq130_e1663_d_n18: f64 = (eq130_e1661_d_n18 * p.p246);
        let eq130_e1663_d_n19: f64 = (eq130_e1661_d_n19 * p.p246);
        let eq130_e1663_d_n20: f64 = (eq130_e1661_d_n20 * p.p246);
        let eq130_e1663_d_n21: f64 = (eq130_e1661_d_n21 * p.p246);
        let eq130_e1663_d_n22: f64 = (eq130_e1661_d_n22 * p.p246);
        let eq130_e1663_q: f64 = (eq130_e1661_q * p.p246);
        let eq130_e1663_q_d_n0: f64 = (eq130_e1661_q_d_n0 * p.p246);
        let eq130_e1663_q_d_n1: f64 = (eq130_e1661_q_d_n1 * p.p246);
        let eq130_e1663_q_d_n2: f64 = (eq130_e1661_q_d_n2 * p.p246);
        let eq130_e1663_q_d_n3: f64 = (eq130_e1661_q_d_n3 * p.p246);
        let eq130_e1663_q_d_n4: f64 = (eq130_e1661_q_d_n4 * p.p246);
        let eq130_e1663_q_d_n5: f64 = (eq130_e1661_q_d_n5 * p.p246);
        let eq130_e1663_q_d_n6: f64 = (eq130_e1661_q_d_n6 * p.p246);
        let eq130_e1663_q_d_n7: f64 = (eq130_e1661_q_d_n7 * p.p246);
        let eq130_e1663_q_d_n8: f64 = (eq130_e1661_q_d_n8 * p.p246);
        let eq130_e1663_q_d_n9: f64 = (eq130_e1661_q_d_n9 * p.p246);
        let eq130_e1663_q_d_n10: f64 = (eq130_e1661_q_d_n10 * p.p246);
        let eq130_e1663_q_d_n11: f64 = (eq130_e1661_q_d_n11 * p.p246);
        let eq130_e1663_q_d_n12: f64 = (eq130_e1661_q_d_n12 * p.p246);
        let eq130_e1663_q_d_n13: f64 = (eq130_e1661_q_d_n13 * p.p246);
        let eq130_e1663_q_d_n14: f64 = (eq130_e1661_q_d_n14 * p.p246);
        let eq130_e1663_q_d_n15: f64 = (eq130_e1661_q_d_n15 * p.p246);
        let eq130_e1663_q_d_n16: f64 = (eq130_e1661_q_d_n16 * p.p246);
        let eq130_e1663_q_d_n17: f64 = (eq130_e1661_q_d_n17 * p.p246);
        let eq130_e1663_q_d_n18: f64 = (eq130_e1661_q_d_n18 * p.p246);
        let eq130_e1663_q_d_n19: f64 = (eq130_e1661_q_d_n19 * p.p246);
        let eq130_e1663_q_d_n20: f64 = (eq130_e1661_q_d_n20 * p.p246);
        let eq130_e1663_q_d_n21: f64 = (eq130_e1661_q_d_n21 * p.p246);
        let eq130_e1663_q_d_n22: f64 = (eq130_e1661_q_d_n22 * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_q, eq130_e1663_q_d_n0, eq130_e1663_q_d_n1, eq130_e1663_q_d_n2, eq130_e1663_q_d_n3, eq130_e1663_q_d_n4, eq130_e1663_q_d_n5, eq130_e1663_q_d_n6, eq130_e1663_q_d_n7, eq130_e1663_q_d_n8, eq130_e1663_q_d_n9, eq130_e1663_q_d_n10, eq130_e1663_q_d_n11, eq130_e1663_q_d_n12, eq130_e1663_q_d_n13, eq130_e1663_q_d_n14, eq130_e1663_q_d_n15, eq130_e1663_q_d_n16, eq130_e1663_q_d_n17, eq130_e1663_q_d_n18, eq130_e1663_q_d_n19, eq130_e1663_q_d_n20, eq130_e1663_q_d_n21, eq130_e1663_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 23] = [eq130_e1665_q_d_n0, eq130_e1665_q_d_n1, eq130_e1665_q_d_n2, eq130_e1665_q_d_n3, eq130_e1665_q_d_n4, eq130_e1665_q_d_n5, eq130_e1665_q_d_n6, eq130_e1665_q_d_n7, eq130_e1665_q_d_n8, eq130_e1665_q_d_n9, eq130_e1665_q_d_n10, eq130_e1665_q_d_n11, eq130_e1665_q_d_n12, eq130_e1665_q_d_n13, eq130_e1665_q_d_n14, eq130_e1665_q_d_n15, eq130_e1665_q_d_n16, eq130_e1665_q_d_n17, eq130_e1665_q_d_n18, eq130_e1665_q_d_n19, eq130_e1665_q_d_n20, eq130_e1665_q_d_n21, eq130_e1665_q_d_n22];
        let eq130_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_q, eq131_e1677_q_d_n0, eq131_e1677_q_d_n1, eq131_e1677_q_d_n2, eq131_e1677_q_d_n3, eq131_e1677_q_d_n4, eq131_e1677_q_d_n5, eq131_e1677_q_d_n6, eq131_e1677_q_d_n7, eq131_e1677_q_d_n8, eq131_e1677_q_d_n9, eq131_e1677_q_d_n10, eq131_e1677_q_d_n11, eq131_e1677_q_d_n12, eq131_e1677_q_d_n13, eq131_e1677_q_d_n14, eq131_e1677_q_d_n15, eq131_e1677_q_d_n16, eq131_e1677_q_d_n17, eq131_e1677_q_d_n18, eq131_e1677_q_d_n19, eq131_e1677_q_d_n20, eq131_e1677_q_d_n21, eq131_e1677_q_d_n22,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq131_e1673: f64 = (p.p251 * s.v[228]);
        let eq131_e1673_d_n0: f64 = (p.p251 * s.dn[228][0]);
        let eq131_e1673_d_n1: f64 = (p.p251 * s.dn[228][1]);
        let eq131_e1673_d_n2: f64 = (p.p251 * s.dn[228][2]);
        let eq131_e1673_d_n3: f64 = (p.p251 * s.dn[228][3]);
        let eq131_e1673_d_n4: f64 = (p.p251 * s.dn[228][4]);
        let eq131_e1673_d_n5: f64 = (p.p251 * s.dn[228][5]);
        let eq131_e1673_d_n6: f64 = (p.p251 * s.dn[228][6]);
        let eq131_e1673_d_n7: f64 = (p.p251 * s.dn[228][7]);
        let eq131_e1673_d_n8: f64 = (p.p251 * s.dn[228][8]);
        let eq131_e1673_d_n9: f64 = (p.p251 * s.dn[228][9]);
        let eq131_e1673_d_n10: f64 = (p.p251 * s.dn[228][10]);
        let eq131_e1673_d_n11: f64 = (p.p251 * s.dn[228][11]);
        let eq131_e1673_d_n12: f64 = (p.p251 * s.dn[228][12]);
        let eq131_e1673_d_n13: f64 = (p.p251 * s.dn[228][13]);
        let eq131_e1673_d_n14: f64 = (p.p251 * s.dn[228][14]);
        let eq131_e1673_d_n15: f64 = (p.p251 * s.dn[228][15]);
        let eq131_e1673_d_n16: f64 = (p.p251 * s.dn[228][16]);
        let eq131_e1673_d_n17: f64 = (p.p251 * s.dn[228][17]);
        let eq131_e1673_d_n18: f64 = (p.p251 * s.dn[228][18]);
        let eq131_e1673_d_n19: f64 = (p.p251 * s.dn[228][19]);
        let eq131_e1673_d_n20: f64 = (p.p251 * s.dn[228][20]);
        let eq131_e1673_d_n21: f64 = (p.p251 * s.dn[228][21]);
        let eq131_e1673_d_n22: f64 = (p.p251 * s.dn[228][22]);
        let eq131_e1674_q: f64 = eq131_e1673;
        let eq131_e1675: f64 = (p.p7 * eq131_e1673);
        let eq131_e1675_d_n0: f64 = (p.p7 * eq131_e1673_d_n0);
        let eq131_e1675_d_n1: f64 = (p.p7 * eq131_e1673_d_n1);
        let eq131_e1675_d_n2: f64 = (p.p7 * eq131_e1673_d_n2);
        let eq131_e1675_d_n3: f64 = (p.p7 * eq131_e1673_d_n3);
        let eq131_e1675_d_n4: f64 = (p.p7 * eq131_e1673_d_n4);
        let eq131_e1675_d_n5: f64 = (p.p7 * eq131_e1673_d_n5);
        let eq131_e1675_d_n6: f64 = (p.p7 * eq131_e1673_d_n6);
        let eq131_e1675_d_n7: f64 = (p.p7 * eq131_e1673_d_n7);
        let eq131_e1675_d_n8: f64 = (p.p7 * eq131_e1673_d_n8);
        let eq131_e1675_d_n9: f64 = (p.p7 * eq131_e1673_d_n9);
        let eq131_e1675_d_n10: f64 = (p.p7 * eq131_e1673_d_n10);
        let eq131_e1675_d_n11: f64 = (p.p7 * eq131_e1673_d_n11);
        let eq131_e1675_d_n12: f64 = (p.p7 * eq131_e1673_d_n12);
        let eq131_e1675_d_n13: f64 = (p.p7 * eq131_e1673_d_n13);
        let eq131_e1675_d_n14: f64 = (p.p7 * eq131_e1673_d_n14);
        let eq131_e1675_d_n15: f64 = (p.p7 * eq131_e1673_d_n15);
        let eq131_e1675_d_n16: f64 = (p.p7 * eq131_e1673_d_n16);
        let eq131_e1675_d_n17: f64 = (p.p7 * eq131_e1673_d_n17);
        let eq131_e1675_d_n18: f64 = (p.p7 * eq131_e1673_d_n18);
        let eq131_e1675_d_n19: f64 = (p.p7 * eq131_e1673_d_n19);
        let eq131_e1675_d_n20: f64 = (p.p7 * eq131_e1673_d_n20);
        let eq131_e1675_d_n21: f64 = (p.p7 * eq131_e1673_d_n21);
        let eq131_e1675_d_n22: f64 = (p.p7 * eq131_e1673_d_n22);
        let eq131_e1675_q: f64 = (p.p7 * eq131_e1674_q);
        let eq131_e1675_q_d_n0: f64 = (p.p7 * eq131_e1673_d_n0);
        let eq131_e1675_q_d_n1: f64 = (p.p7 * eq131_e1673_d_n1);
        let eq131_e1675_q_d_n2: f64 = (p.p7 * eq131_e1673_d_n2);
        let eq131_e1675_q_d_n3: f64 = (p.p7 * eq131_e1673_d_n3);
        let eq131_e1675_q_d_n4: f64 = (p.p7 * eq131_e1673_d_n4);
        let eq131_e1675_q_d_n5: f64 = (p.p7 * eq131_e1673_d_n5);
        let eq131_e1675_q_d_n6: f64 = (p.p7 * eq131_e1673_d_n6);
        let eq131_e1675_q_d_n7: f64 = (p.p7 * eq131_e1673_d_n7);
        let eq131_e1675_q_d_n8: f64 = (p.p7 * eq131_e1673_d_n8);
        let eq131_e1675_q_d_n9: f64 = (p.p7 * eq131_e1673_d_n9);
        let eq131_e1675_q_d_n10: f64 = (p.p7 * eq131_e1673_d_n10);
        let eq131_e1675_q_d_n11: f64 = (p.p7 * eq131_e1673_d_n11);
        let eq131_e1675_q_d_n12: f64 = (p.p7 * eq131_e1673_d_n12);
        let eq131_e1675_q_d_n13: f64 = (p.p7 * eq131_e1673_d_n13);
        let eq131_e1675_q_d_n14: f64 = (p.p7 * eq131_e1673_d_n14);
        let eq131_e1675_q_d_n15: f64 = (p.p7 * eq131_e1673_d_n15);
        let eq131_e1675_q_d_n16: f64 = (p.p7 * eq131_e1673_d_n16);
        let eq131_e1675_q_d_n17: f64 = (p.p7 * eq131_e1673_d_n17);
        let eq131_e1675_q_d_n18: f64 = (p.p7 * eq131_e1673_d_n18);
        let eq131_e1675_q_d_n19: f64 = (p.p7 * eq131_e1673_d_n19);
        let eq131_e1675_q_d_n20: f64 = (p.p7 * eq131_e1673_d_n20);
        let eq131_e1675_q_d_n21: f64 = (p.p7 * eq131_e1673_d_n21);
        let eq131_e1675_q_d_n22: f64 = (p.p7 * eq131_e1673_d_n22);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_q, eq131_e1675_q_d_n0, eq131_e1675_q_d_n1, eq131_e1675_q_d_n2, eq131_e1675_q_d_n3, eq131_e1675_q_d_n4, eq131_e1675_q_d_n5, eq131_e1675_q_d_n6, eq131_e1675_q_d_n7, eq131_e1675_q_d_n8, eq131_e1675_q_d_n9, eq131_e1675_q_d_n10, eq131_e1675_q_d_n11, eq131_e1675_q_d_n12, eq131_e1675_q_d_n13, eq131_e1675_q_d_n14, eq131_e1675_q_d_n15, eq131_e1675_q_d_n16, eq131_e1675_q_d_n17, eq131_e1675_q_d_n18, eq131_e1675_q_d_n19, eq131_e1675_q_d_n20, eq131_e1675_q_d_n21, eq131_e1675_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 23] = [eq131_e1677_q_d_n0, eq131_e1677_q_d_n1, eq131_e1677_q_d_n2, eq131_e1677_q_d_n3, eq131_e1677_q_d_n4, eq131_e1677_q_d_n5, eq131_e1677_q_d_n6, eq131_e1677_q_d_n7, eq131_e1677_q_d_n8, eq131_e1677_q_d_n9, eq131_e1677_q_d_n10, eq131_e1677_q_d_n11, eq131_e1677_q_d_n12, eq131_e1677_q_d_n13, eq131_e1677_q_d_n14, eq131_e1677_q_d_n15, eq131_e1677_q_d_n16, eq131_e1677_q_d_n17, eq131_e1677_q_d_n18, eq131_e1677_q_d_n19, eq131_e1677_q_d_n20, eq131_e1677_q_d_n21, eq131_e1677_q_d_n22];
        let eq131_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_q, eq132_e1686_q_d_n0, eq132_e1686_q_d_n1, eq132_e1686_q_d_n2, eq132_e1686_q_d_n3, eq132_e1686_q_d_n4, eq132_e1686_q_d_n5, eq132_e1686_q_d_n6, eq132_e1686_q_d_n7, eq132_e1686_q_d_n8, eq132_e1686_q_d_n9, eq132_e1686_q_d_n10, eq132_e1686_q_d_n11, eq132_e1686_q_d_n12, eq132_e1686_q_d_n13, eq132_e1686_q_d_n14, eq132_e1686_q_d_n15, eq132_e1686_q_d_n16, eq132_e1686_q_d_n17, eq132_e1686_q_d_n18, eq132_e1686_q_d_n19, eq132_e1686_q_d_n20, eq132_e1686_q_d_n21, eq132_e1686_q_d_n22,) = {
    if (s.b[575] && s.b[576]) {
        let eq132_e1683_q: f64 = s.v[241];
        let eq132_e1684: f64 = (p.p7 * s.v[241]);
        let eq132_e1684_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq132_e1684_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq132_e1684_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq132_e1684_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq132_e1684_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq132_e1684_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq132_e1684_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq132_e1684_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq132_e1684_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq132_e1684_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq132_e1684_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq132_e1684_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq132_e1684_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq132_e1684_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq132_e1684_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq132_e1684_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq132_e1684_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq132_e1684_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq132_e1684_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq132_e1684_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq132_e1684_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq132_e1684_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq132_e1684_d_n22: f64 = (p.p7 * s.dn[241][22]);
        let eq132_e1684_q: f64 = (p.p7 * eq132_e1683_q);
        let eq132_e1684_q_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq132_e1684_q_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq132_e1684_q_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq132_e1684_q_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq132_e1684_q_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq132_e1684_q_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq132_e1684_q_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq132_e1684_q_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq132_e1684_q_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq132_e1684_q_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq132_e1684_q_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq132_e1684_q_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq132_e1684_q_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq132_e1684_q_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq132_e1684_q_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq132_e1684_q_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq132_e1684_q_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq132_e1684_q_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq132_e1684_q_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq132_e1684_q_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq132_e1684_q_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq132_e1684_q_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq132_e1684_q_d_n22: f64 = (p.p7 * s.dn[241][22]);
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22, eq132_e1684_q, eq132_e1684_q_d_n0, eq132_e1684_q_d_n1, eq132_e1684_q_d_n2, eq132_e1684_q_d_n3, eq132_e1684_q_d_n4, eq132_e1684_q_d_n5, eq132_e1684_q_d_n6, eq132_e1684_q_d_n7, eq132_e1684_q_d_n8, eq132_e1684_q_d_n9, eq132_e1684_q_d_n10, eq132_e1684_q_d_n11, eq132_e1684_q_d_n12, eq132_e1684_q_d_n13, eq132_e1684_q_d_n14, eq132_e1684_q_d_n15, eq132_e1684_q_d_n16, eq132_e1684_q_d_n17, eq132_e1684_q_d_n18, eq132_e1684_q_d_n19, eq132_e1684_q_d_n20, eq132_e1684_q_d_n21, eq132_e1684_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_reactive_node_derivatives: [f64; 23] = [eq132_e1686_q_d_n0, eq132_e1686_q_d_n1, eq132_e1686_q_d_n2, eq132_e1686_q_d_n3, eq132_e1686_q_d_n4, eq132_e1686_q_d_n5, eq132_e1686_q_d_n6, eq132_e1686_q_d_n7, eq132_e1686_q_d_n8, eq132_e1686_q_d_n9, eq132_e1686_q_d_n10, eq132_e1686_q_d_n11, eq132_e1686_q_d_n12, eq132_e1686_q_d_n13, eq132_e1686_q_d_n14, eq132_e1686_q_d_n15, eq132_e1686_q_d_n16, eq132_e1686_q_d_n17, eq132_e1686_q_d_n18, eq132_e1686_q_d_n19, eq132_e1686_q_d_n20, eq132_e1686_q_d_n21, eq132_e1686_q_d_n22];
        let eq132_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            nodes,
            &eq132_reactive_node_derivatives,
            branches,
            &eq132_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_q, eq133_e1697_q_d_n0, eq133_e1697_q_d_n1, eq133_e1697_q_d_n2, eq133_e1697_q_d_n3, eq133_e1697_q_d_n4, eq133_e1697_q_d_n5, eq133_e1697_q_d_n6, eq133_e1697_q_d_n7, eq133_e1697_q_d_n8, eq133_e1697_q_d_n9, eq133_e1697_q_d_n10, eq133_e1697_q_d_n11, eq133_e1697_q_d_n12, eq133_e1697_q_d_n13, eq133_e1697_q_d_n14, eq133_e1697_q_d_n15, eq133_e1697_q_d_n16, eq133_e1697_q_d_n17, eq133_e1697_q_d_n18, eq133_e1697_q_d_n19, eq133_e1697_q_d_n20, eq133_e1697_q_d_n21, eq133_e1697_q_d_n22,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq133_e1694_q: f64 = s.v[240];
        let eq133_e1695: f64 = (p.p7 * s.v[240]);
        let eq133_e1695_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq133_e1695_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq133_e1695_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq133_e1695_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq133_e1695_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq133_e1695_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq133_e1695_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq133_e1695_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq133_e1695_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq133_e1695_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq133_e1695_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq133_e1695_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq133_e1695_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq133_e1695_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq133_e1695_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq133_e1695_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq133_e1695_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq133_e1695_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq133_e1695_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq133_e1695_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq133_e1695_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq133_e1695_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq133_e1695_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq133_e1695_q: f64 = (p.p7 * eq133_e1694_q);
        let eq133_e1695_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq133_e1695_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq133_e1695_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq133_e1695_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq133_e1695_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq133_e1695_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq133_e1695_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq133_e1695_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq133_e1695_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq133_e1695_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq133_e1695_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq133_e1695_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq133_e1695_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq133_e1695_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq133_e1695_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq133_e1695_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq133_e1695_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq133_e1695_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq133_e1695_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq133_e1695_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq133_e1695_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq133_e1695_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq133_e1695_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq133_e1695, eq133_e1695_d_n0, eq133_e1695_d_n1, eq133_e1695_d_n2, eq133_e1695_d_n3, eq133_e1695_d_n4, eq133_e1695_d_n5, eq133_e1695_d_n6, eq133_e1695_d_n7, eq133_e1695_d_n8, eq133_e1695_d_n9, eq133_e1695_d_n10, eq133_e1695_d_n11, eq133_e1695_d_n12, eq133_e1695_d_n13, eq133_e1695_d_n14, eq133_e1695_d_n15, eq133_e1695_d_n16, eq133_e1695_d_n17, eq133_e1695_d_n18, eq133_e1695_d_n19, eq133_e1695_d_n20, eq133_e1695_d_n21, eq133_e1695_d_n22, eq133_e1695_q, eq133_e1695_q_d_n0, eq133_e1695_q_d_n1, eq133_e1695_q_d_n2, eq133_e1695_q_d_n3, eq133_e1695_q_d_n4, eq133_e1695_q_d_n5, eq133_e1695_q_d_n6, eq133_e1695_q_d_n7, eq133_e1695_q_d_n8, eq133_e1695_q_d_n9, eq133_e1695_q_d_n10, eq133_e1695_q_d_n11, eq133_e1695_q_d_n12, eq133_e1695_q_d_n13, eq133_e1695_q_d_n14, eq133_e1695_q_d_n15, eq133_e1695_q_d_n16, eq133_e1695_q_d_n17, eq133_e1695_q_d_n18, eq133_e1695_q_d_n19, eq133_e1695_q_d_n20, eq133_e1695_q_d_n21, eq133_e1695_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_reactive_node_derivatives: [f64; 23] = [eq133_e1697_q_d_n0, eq133_e1697_q_d_n1, eq133_e1697_q_d_n2, eq133_e1697_q_d_n3, eq133_e1697_q_d_n4, eq133_e1697_q_d_n5, eq133_e1697_q_d_n6, eq133_e1697_q_d_n7, eq133_e1697_q_d_n8, eq133_e1697_q_d_n9, eq133_e1697_q_d_n10, eq133_e1697_q_d_n11, eq133_e1697_q_d_n12, eq133_e1697_q_d_n13, eq133_e1697_q_d_n14, eq133_e1697_q_d_n15, eq133_e1697_q_d_n16, eq133_e1697_q_d_n17, eq133_e1697_q_d_n18, eq133_e1697_q_d_n19, eq133_e1697_q_d_n20, eq133_e1697_q_d_n21, eq133_e1697_q_d_n22];
        let eq133_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq133_reactive_node_derivatives,
            branches,
            &eq133_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_q, eq134_e1710_q_d_n0, eq134_e1710_q_d_n1, eq134_e1710_q_d_n2, eq134_e1710_q_d_n3, eq134_e1710_q_d_n4, eq134_e1710_q_d_n5, eq134_e1710_q_d_n6, eq134_e1710_q_d_n7, eq134_e1710_q_d_n8, eq134_e1710_q_d_n9, eq134_e1710_q_d_n10, eq134_e1710_q_d_n11, eq134_e1710_q_d_n12, eq134_e1710_q_d_n13, eq134_e1710_q_d_n14, eq134_e1710_q_d_n15, eq134_e1710_q_d_n16, eq134_e1710_q_d_n17, eq134_e1710_q_d_n18, eq134_e1710_q_d_n19, eq134_e1710_q_d_n20, eq134_e1710_q_d_n21, eq134_e1710_q_d_n22,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq134_e1705_q: f64 = s.v[240];
        let eq134_e1706: f64 = (p.p7 * s.v[240]);
        let eq134_e1706_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq134_e1706_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq134_e1706_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq134_e1706_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq134_e1706_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq134_e1706_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq134_e1706_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq134_e1706_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq134_e1706_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq134_e1706_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq134_e1706_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq134_e1706_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq134_e1706_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq134_e1706_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq134_e1706_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq134_e1706_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq134_e1706_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq134_e1706_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq134_e1706_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq134_e1706_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq134_e1706_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq134_e1706_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq134_e1706_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq134_e1706_q: f64 = (p.p7 * eq134_e1705_q);
        let eq134_e1706_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq134_e1706_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq134_e1706_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq134_e1706_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq134_e1706_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq134_e1706_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq134_e1706_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq134_e1706_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq134_e1706_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq134_e1706_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq134_e1706_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq134_e1706_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq134_e1706_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq134_e1706_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq134_e1706_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq134_e1706_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq134_e1706_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq134_e1706_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq134_e1706_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq134_e1706_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq134_e1706_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq134_e1706_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq134_e1706_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (eq134_e1706_d_n0 * p.p246);
        let eq134_e1708_d_n1: f64 = (eq134_e1706_d_n1 * p.p246);
        let eq134_e1708_d_n2: f64 = (eq134_e1706_d_n2 * p.p246);
        let eq134_e1708_d_n3: f64 = (eq134_e1706_d_n3 * p.p246);
        let eq134_e1708_d_n4: f64 = (eq134_e1706_d_n4 * p.p246);
        let eq134_e1708_d_n5: f64 = (eq134_e1706_d_n5 * p.p246);
        let eq134_e1708_d_n6: f64 = (eq134_e1706_d_n6 * p.p246);
        let eq134_e1708_d_n7: f64 = (eq134_e1706_d_n7 * p.p246);
        let eq134_e1708_d_n8: f64 = (eq134_e1706_d_n8 * p.p246);
        let eq134_e1708_d_n9: f64 = (eq134_e1706_d_n9 * p.p246);
        let eq134_e1708_d_n10: f64 = (eq134_e1706_d_n10 * p.p246);
        let eq134_e1708_d_n11: f64 = (eq134_e1706_d_n11 * p.p246);
        let eq134_e1708_d_n12: f64 = (eq134_e1706_d_n12 * p.p246);
        let eq134_e1708_d_n13: f64 = (eq134_e1706_d_n13 * p.p246);
        let eq134_e1708_d_n14: f64 = (eq134_e1706_d_n14 * p.p246);
        let eq134_e1708_d_n15: f64 = (eq134_e1706_d_n15 * p.p246);
        let eq134_e1708_d_n16: f64 = (eq134_e1706_d_n16 * p.p246);
        let eq134_e1708_d_n17: f64 = (eq134_e1706_d_n17 * p.p246);
        let eq134_e1708_d_n18: f64 = (eq134_e1706_d_n18 * p.p246);
        let eq134_e1708_d_n19: f64 = (eq134_e1706_d_n19 * p.p246);
        let eq134_e1708_d_n20: f64 = (eq134_e1706_d_n20 * p.p246);
        let eq134_e1708_d_n21: f64 = (eq134_e1706_d_n21 * p.p246);
        let eq134_e1708_d_n22: f64 = (eq134_e1706_d_n22 * p.p246);
        let eq134_e1708_q: f64 = (eq134_e1706_q * p.p246);
        let eq134_e1708_q_d_n0: f64 = (eq134_e1706_q_d_n0 * p.p246);
        let eq134_e1708_q_d_n1: f64 = (eq134_e1706_q_d_n1 * p.p246);
        let eq134_e1708_q_d_n2: f64 = (eq134_e1706_q_d_n2 * p.p246);
        let eq134_e1708_q_d_n3: f64 = (eq134_e1706_q_d_n3 * p.p246);
        let eq134_e1708_q_d_n4: f64 = (eq134_e1706_q_d_n4 * p.p246);
        let eq134_e1708_q_d_n5: f64 = (eq134_e1706_q_d_n5 * p.p246);
        let eq134_e1708_q_d_n6: f64 = (eq134_e1706_q_d_n6 * p.p246);
        let eq134_e1708_q_d_n7: f64 = (eq134_e1706_q_d_n7 * p.p246);
        let eq134_e1708_q_d_n8: f64 = (eq134_e1706_q_d_n8 * p.p246);
        let eq134_e1708_q_d_n9: f64 = (eq134_e1706_q_d_n9 * p.p246);
        let eq134_e1708_q_d_n10: f64 = (eq134_e1706_q_d_n10 * p.p246);
        let eq134_e1708_q_d_n11: f64 = (eq134_e1706_q_d_n11 * p.p246);
        let eq134_e1708_q_d_n12: f64 = (eq134_e1706_q_d_n12 * p.p246);
        let eq134_e1708_q_d_n13: f64 = (eq134_e1706_q_d_n13 * p.p246);
        let eq134_e1708_q_d_n14: f64 = (eq134_e1706_q_d_n14 * p.p246);
        let eq134_e1708_q_d_n15: f64 = (eq134_e1706_q_d_n15 * p.p246);
        let eq134_e1708_q_d_n16: f64 = (eq134_e1706_q_d_n16 * p.p246);
        let eq134_e1708_q_d_n17: f64 = (eq134_e1706_q_d_n17 * p.p246);
        let eq134_e1708_q_d_n18: f64 = (eq134_e1706_q_d_n18 * p.p246);
        let eq134_e1708_q_d_n19: f64 = (eq134_e1706_q_d_n19 * p.p246);
        let eq134_e1708_q_d_n20: f64 = (eq134_e1706_q_d_n20 * p.p246);
        let eq134_e1708_q_d_n21: f64 = (eq134_e1706_q_d_n21 * p.p246);
        let eq134_e1708_q_d_n22: f64 = (eq134_e1706_q_d_n22 * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22, eq134_e1708_q, eq134_e1708_q_d_n0, eq134_e1708_q_d_n1, eq134_e1708_q_d_n2, eq134_e1708_q_d_n3, eq134_e1708_q_d_n4, eq134_e1708_q_d_n5, eq134_e1708_q_d_n6, eq134_e1708_q_d_n7, eq134_e1708_q_d_n8, eq134_e1708_q_d_n9, eq134_e1708_q_d_n10, eq134_e1708_q_d_n11, eq134_e1708_q_d_n12, eq134_e1708_q_d_n13, eq134_e1708_q_d_n14, eq134_e1708_q_d_n15, eq134_e1708_q_d_n16, eq134_e1708_q_d_n17, eq134_e1708_q_d_n18, eq134_e1708_q_d_n19, eq134_e1708_q_d_n20, eq134_e1708_q_d_n21, eq134_e1708_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_reactive_node_derivatives: [f64; 23] = [eq134_e1710_q_d_n0, eq134_e1710_q_d_n1, eq134_e1710_q_d_n2, eq134_e1710_q_d_n3, eq134_e1710_q_d_n4, eq134_e1710_q_d_n5, eq134_e1710_q_d_n6, eq134_e1710_q_d_n7, eq134_e1710_q_d_n8, eq134_e1710_q_d_n9, eq134_e1710_q_d_n10, eq134_e1710_q_d_n11, eq134_e1710_q_d_n12, eq134_e1710_q_d_n13, eq134_e1710_q_d_n14, eq134_e1710_q_d_n15, eq134_e1710_q_d_n16, eq134_e1710_q_d_n17, eq134_e1710_q_d_n18, eq134_e1710_q_d_n19, eq134_e1710_q_d_n20, eq134_e1710_q_d_n21, eq134_e1710_q_d_n22];
        let eq134_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_5(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_q, eq135_e1722_q_d_n0, eq135_e1722_q_d_n1, eq135_e1722_q_d_n2, eq135_e1722_q_d_n3, eq135_e1722_q_d_n4, eq135_e1722_q_d_n5, eq135_e1722_q_d_n6, eq135_e1722_q_d_n7, eq135_e1722_q_d_n8, eq135_e1722_q_d_n9, eq135_e1722_q_d_n10, eq135_e1722_q_d_n11, eq135_e1722_q_d_n12, eq135_e1722_q_d_n13, eq135_e1722_q_d_n14, eq135_e1722_q_d_n15, eq135_e1722_q_d_n16, eq135_e1722_q_d_n17, eq135_e1722_q_d_n18, eq135_e1722_q_d_n19, eq135_e1722_q_d_n20, eq135_e1722_q_d_n21, eq135_e1722_q_d_n22,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq135_e1719_q: f64 = s.v[240];
        let eq135_e1720: f64 = (p.p7 * s.v[240]);
        let eq135_e1720_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq135_e1720_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq135_e1720_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq135_e1720_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq135_e1720_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq135_e1720_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq135_e1720_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq135_e1720_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq135_e1720_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq135_e1720_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq135_e1720_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq135_e1720_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq135_e1720_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq135_e1720_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq135_e1720_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq135_e1720_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq135_e1720_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq135_e1720_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq135_e1720_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq135_e1720_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq135_e1720_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq135_e1720_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq135_e1720_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq135_e1720_q: f64 = (p.p7 * eq135_e1719_q);
        let eq135_e1720_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq135_e1720_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq135_e1720_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq135_e1720_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq135_e1720_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq135_e1720_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq135_e1720_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq135_e1720_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq135_e1720_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq135_e1720_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq135_e1720_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq135_e1720_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq135_e1720_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq135_e1720_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq135_e1720_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq135_e1720_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq135_e1720_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq135_e1720_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq135_e1720_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq135_e1720_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq135_e1720_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq135_e1720_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq135_e1720_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq135_e1720, eq135_e1720_d_n0, eq135_e1720_d_n1, eq135_e1720_d_n2, eq135_e1720_d_n3, eq135_e1720_d_n4, eq135_e1720_d_n5, eq135_e1720_d_n6, eq135_e1720_d_n7, eq135_e1720_d_n8, eq135_e1720_d_n9, eq135_e1720_d_n10, eq135_e1720_d_n11, eq135_e1720_d_n12, eq135_e1720_d_n13, eq135_e1720_d_n14, eq135_e1720_d_n15, eq135_e1720_d_n16, eq135_e1720_d_n17, eq135_e1720_d_n18, eq135_e1720_d_n19, eq135_e1720_d_n20, eq135_e1720_d_n21, eq135_e1720_d_n22, eq135_e1720_q, eq135_e1720_q_d_n0, eq135_e1720_q_d_n1, eq135_e1720_q_d_n2, eq135_e1720_q_d_n3, eq135_e1720_q_d_n4, eq135_e1720_q_d_n5, eq135_e1720_q_d_n6, eq135_e1720_q_d_n7, eq135_e1720_q_d_n8, eq135_e1720_q_d_n9, eq135_e1720_q_d_n10, eq135_e1720_q_d_n11, eq135_e1720_q_d_n12, eq135_e1720_q_d_n13, eq135_e1720_q_d_n14, eq135_e1720_q_d_n15, eq135_e1720_q_d_n16, eq135_e1720_q_d_n17, eq135_e1720_q_d_n18, eq135_e1720_q_d_n19, eq135_e1720_q_d_n20, eq135_e1720_q_d_n21, eq135_e1720_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_reactive_node_derivatives: [f64; 23] = [eq135_e1722_q_d_n0, eq135_e1722_q_d_n1, eq135_e1722_q_d_n2, eq135_e1722_q_d_n3, eq135_e1722_q_d_n4, eq135_e1722_q_d_n5, eq135_e1722_q_d_n6, eq135_e1722_q_d_n7, eq135_e1722_q_d_n8, eq135_e1722_q_d_n9, eq135_e1722_q_d_n10, eq135_e1722_q_d_n11, eq135_e1722_q_d_n12, eq135_e1722_q_d_n13, eq135_e1722_q_d_n14, eq135_e1722_q_d_n15, eq135_e1722_q_d_n16, eq135_e1722_q_d_n17, eq135_e1722_q_d_n18, eq135_e1722_q_d_n19, eq135_e1722_q_d_n20, eq135_e1722_q_d_n21, eq135_e1722_q_d_n22];
        let eq135_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq135_reactive_node_derivatives,
            branches,
            &eq135_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_q, eq136_e1736_q_d_n0, eq136_e1736_q_d_n1, eq136_e1736_q_d_n2, eq136_e1736_q_d_n3, eq136_e1736_q_d_n4, eq136_e1736_q_d_n5, eq136_e1736_q_d_n6, eq136_e1736_q_d_n7, eq136_e1736_q_d_n8, eq136_e1736_q_d_n9, eq136_e1736_q_d_n10, eq136_e1736_q_d_n11, eq136_e1736_q_d_n12, eq136_e1736_q_d_n13, eq136_e1736_q_d_n14, eq136_e1736_q_d_n15, eq136_e1736_q_d_n16, eq136_e1736_q_d_n17, eq136_e1736_q_d_n18, eq136_e1736_q_d_n19, eq136_e1736_q_d_n20, eq136_e1736_q_d_n21, eq136_e1736_q_d_n22,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq136_e1731_q: f64 = s.v[240];
        let eq136_e1732: f64 = (p.p7 * s.v[240]);
        let eq136_e1732_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq136_e1732_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq136_e1732_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq136_e1732_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq136_e1732_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq136_e1732_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq136_e1732_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq136_e1732_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq136_e1732_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq136_e1732_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq136_e1732_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq136_e1732_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq136_e1732_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq136_e1732_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq136_e1732_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq136_e1732_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq136_e1732_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq136_e1732_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq136_e1732_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq136_e1732_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq136_e1732_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq136_e1732_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq136_e1732_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq136_e1732_q: f64 = (p.p7 * eq136_e1731_q);
        let eq136_e1732_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq136_e1732_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq136_e1732_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq136_e1732_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq136_e1732_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq136_e1732_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq136_e1732_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq136_e1732_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq136_e1732_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq136_e1732_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq136_e1732_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq136_e1732_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq136_e1732_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq136_e1732_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq136_e1732_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq136_e1732_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq136_e1732_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq136_e1732_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq136_e1732_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq136_e1732_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq136_e1732_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq136_e1732_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq136_e1732_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_d_n0: f64 = (eq136_e1732_d_n0 * p.p246);
        let eq136_e1734_d_n1: f64 = (eq136_e1732_d_n1 * p.p246);
        let eq136_e1734_d_n2: f64 = (eq136_e1732_d_n2 * p.p246);
        let eq136_e1734_d_n3: f64 = (eq136_e1732_d_n3 * p.p246);
        let eq136_e1734_d_n4: f64 = (eq136_e1732_d_n4 * p.p246);
        let eq136_e1734_d_n5: f64 = (eq136_e1732_d_n5 * p.p246);
        let eq136_e1734_d_n6: f64 = (eq136_e1732_d_n6 * p.p246);
        let eq136_e1734_d_n7: f64 = (eq136_e1732_d_n7 * p.p246);
        let eq136_e1734_d_n8: f64 = (eq136_e1732_d_n8 * p.p246);
        let eq136_e1734_d_n9: f64 = (eq136_e1732_d_n9 * p.p246);
        let eq136_e1734_d_n10: f64 = (eq136_e1732_d_n10 * p.p246);
        let eq136_e1734_d_n11: f64 = (eq136_e1732_d_n11 * p.p246);
        let eq136_e1734_d_n12: f64 = (eq136_e1732_d_n12 * p.p246);
        let eq136_e1734_d_n13: f64 = (eq136_e1732_d_n13 * p.p246);
        let eq136_e1734_d_n14: f64 = (eq136_e1732_d_n14 * p.p246);
        let eq136_e1734_d_n15: f64 = (eq136_e1732_d_n15 * p.p246);
        let eq136_e1734_d_n16: f64 = (eq136_e1732_d_n16 * p.p246);
        let eq136_e1734_d_n17: f64 = (eq136_e1732_d_n17 * p.p246);
        let eq136_e1734_d_n18: f64 = (eq136_e1732_d_n18 * p.p246);
        let eq136_e1734_d_n19: f64 = (eq136_e1732_d_n19 * p.p246);
        let eq136_e1734_d_n20: f64 = (eq136_e1732_d_n20 * p.p246);
        let eq136_e1734_d_n21: f64 = (eq136_e1732_d_n21 * p.p246);
        let eq136_e1734_d_n22: f64 = (eq136_e1732_d_n22 * p.p246);
        let eq136_e1734_q: f64 = (eq136_e1732_q * p.p246);
        let eq136_e1734_q_d_n0: f64 = (eq136_e1732_q_d_n0 * p.p246);
        let eq136_e1734_q_d_n1: f64 = (eq136_e1732_q_d_n1 * p.p246);
        let eq136_e1734_q_d_n2: f64 = (eq136_e1732_q_d_n2 * p.p246);
        let eq136_e1734_q_d_n3: f64 = (eq136_e1732_q_d_n3 * p.p246);
        let eq136_e1734_q_d_n4: f64 = (eq136_e1732_q_d_n4 * p.p246);
        let eq136_e1734_q_d_n5: f64 = (eq136_e1732_q_d_n5 * p.p246);
        let eq136_e1734_q_d_n6: f64 = (eq136_e1732_q_d_n6 * p.p246);
        let eq136_e1734_q_d_n7: f64 = (eq136_e1732_q_d_n7 * p.p246);
        let eq136_e1734_q_d_n8: f64 = (eq136_e1732_q_d_n8 * p.p246);
        let eq136_e1734_q_d_n9: f64 = (eq136_e1732_q_d_n9 * p.p246);
        let eq136_e1734_q_d_n10: f64 = (eq136_e1732_q_d_n10 * p.p246);
        let eq136_e1734_q_d_n11: f64 = (eq136_e1732_q_d_n11 * p.p246);
        let eq136_e1734_q_d_n12: f64 = (eq136_e1732_q_d_n12 * p.p246);
        let eq136_e1734_q_d_n13: f64 = (eq136_e1732_q_d_n13 * p.p246);
        let eq136_e1734_q_d_n14: f64 = (eq136_e1732_q_d_n14 * p.p246);
        let eq136_e1734_q_d_n15: f64 = (eq136_e1732_q_d_n15 * p.p246);
        let eq136_e1734_q_d_n16: f64 = (eq136_e1732_q_d_n16 * p.p246);
        let eq136_e1734_q_d_n17: f64 = (eq136_e1732_q_d_n17 * p.p246);
        let eq136_e1734_q_d_n18: f64 = (eq136_e1732_q_d_n18 * p.p246);
        let eq136_e1734_q_d_n19: f64 = (eq136_e1732_q_d_n19 * p.p246);
        let eq136_e1734_q_d_n20: f64 = (eq136_e1732_q_d_n20 * p.p246);
        let eq136_e1734_q_d_n21: f64 = (eq136_e1732_q_d_n21 * p.p246);
        let eq136_e1734_q_d_n22: f64 = (eq136_e1732_q_d_n22 * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22, eq136_e1734_q, eq136_e1734_q_d_n0, eq136_e1734_q_d_n1, eq136_e1734_q_d_n2, eq136_e1734_q_d_n3, eq136_e1734_q_d_n4, eq136_e1734_q_d_n5, eq136_e1734_q_d_n6, eq136_e1734_q_d_n7, eq136_e1734_q_d_n8, eq136_e1734_q_d_n9, eq136_e1734_q_d_n10, eq136_e1734_q_d_n11, eq136_e1734_q_d_n12, eq136_e1734_q_d_n13, eq136_e1734_q_d_n14, eq136_e1734_q_d_n15, eq136_e1734_q_d_n16, eq136_e1734_q_d_n17, eq136_e1734_q_d_n18, eq136_e1734_q_d_n19, eq136_e1734_q_d_n20, eq136_e1734_q_d_n21, eq136_e1734_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_reactive_node_derivatives: [f64; 23] = [eq136_e1736_q_d_n0, eq136_e1736_q_d_n1, eq136_e1736_q_d_n2, eq136_e1736_q_d_n3, eq136_e1736_q_d_n4, eq136_e1736_q_d_n5, eq136_e1736_q_d_n6, eq136_e1736_q_d_n7, eq136_e1736_q_d_n8, eq136_e1736_q_d_n9, eq136_e1736_q_d_n10, eq136_e1736_q_d_n11, eq136_e1736_q_d_n12, eq136_e1736_q_d_n13, eq136_e1736_q_d_n14, eq136_e1736_q_d_n15, eq136_e1736_q_d_n16, eq136_e1736_q_d_n17, eq136_e1736_q_d_n18, eq136_e1736_q_d_n19, eq136_e1736_q_d_n20, eq136_e1736_q_d_n21, eq136_e1736_q_d_n22];
        let eq136_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq136_reactive_node_derivatives,
            branches,
            &eq136_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22, eq137_e1747_q, eq137_e1747_q_d_n0, eq137_e1747_q_d_n1, eq137_e1747_q_d_n2, eq137_e1747_q_d_n3, eq137_e1747_q_d_n4, eq137_e1747_q_d_n5, eq137_e1747_q_d_n6, eq137_e1747_q_d_n7, eq137_e1747_q_d_n8, eq137_e1747_q_d_n9, eq137_e1747_q_d_n10, eq137_e1747_q_d_n11, eq137_e1747_q_d_n12, eq137_e1747_q_d_n13, eq137_e1747_q_d_n14, eq137_e1747_q_d_n15, eq137_e1747_q_d_n16, eq137_e1747_q_d_n17, eq137_e1747_q_d_n18, eq137_e1747_q_d_n19, eq137_e1747_q_d_n20, eq137_e1747_q_d_n21, eq137_e1747_q_d_n22,) = {
    if (s.b[575] && s.b[576]) {
        let eq137_e1743: f64 = (p.p251 * s.v[240]);
        let eq137_e1743_d_n0: f64 = (p.p251 * s.dn[240][0]);
        let eq137_e1743_d_n1: f64 = (p.p251 * s.dn[240][1]);
        let eq137_e1743_d_n2: f64 = (p.p251 * s.dn[240][2]);
        let eq137_e1743_d_n3: f64 = (p.p251 * s.dn[240][3]);
        let eq137_e1743_d_n4: f64 = (p.p251 * s.dn[240][4]);
        let eq137_e1743_d_n5: f64 = (p.p251 * s.dn[240][5]);
        let eq137_e1743_d_n6: f64 = (p.p251 * s.dn[240][6]);
        let eq137_e1743_d_n7: f64 = (p.p251 * s.dn[240][7]);
        let eq137_e1743_d_n8: f64 = (p.p251 * s.dn[240][8]);
        let eq137_e1743_d_n9: f64 = (p.p251 * s.dn[240][9]);
        let eq137_e1743_d_n10: f64 = (p.p251 * s.dn[240][10]);
        let eq137_e1743_d_n11: f64 = (p.p251 * s.dn[240][11]);
        let eq137_e1743_d_n12: f64 = (p.p251 * s.dn[240][12]);
        let eq137_e1743_d_n13: f64 = (p.p251 * s.dn[240][13]);
        let eq137_e1743_d_n14: f64 = (p.p251 * s.dn[240][14]);
        let eq137_e1743_d_n15: f64 = (p.p251 * s.dn[240][15]);
        let eq137_e1743_d_n16: f64 = (p.p251 * s.dn[240][16]);
        let eq137_e1743_d_n17: f64 = (p.p251 * s.dn[240][17]);
        let eq137_e1743_d_n18: f64 = (p.p251 * s.dn[240][18]);
        let eq137_e1743_d_n19: f64 = (p.p251 * s.dn[240][19]);
        let eq137_e1743_d_n20: f64 = (p.p251 * s.dn[240][20]);
        let eq137_e1743_d_n21: f64 = (p.p251 * s.dn[240][21]);
        let eq137_e1743_d_n22: f64 = (p.p251 * s.dn[240][22]);
        let eq137_e1744_q: f64 = eq137_e1743;
        let eq137_e1745: f64 = (p.p7 * eq137_e1743);
        let eq137_e1745_d_n0: f64 = (p.p7 * eq137_e1743_d_n0);
        let eq137_e1745_d_n1: f64 = (p.p7 * eq137_e1743_d_n1);
        let eq137_e1745_d_n2: f64 = (p.p7 * eq137_e1743_d_n2);
        let eq137_e1745_d_n3: f64 = (p.p7 * eq137_e1743_d_n3);
        let eq137_e1745_d_n4: f64 = (p.p7 * eq137_e1743_d_n4);
        let eq137_e1745_d_n5: f64 = (p.p7 * eq137_e1743_d_n5);
        let eq137_e1745_d_n6: f64 = (p.p7 * eq137_e1743_d_n6);
        let eq137_e1745_d_n7: f64 = (p.p7 * eq137_e1743_d_n7);
        let eq137_e1745_d_n8: f64 = (p.p7 * eq137_e1743_d_n8);
        let eq137_e1745_d_n9: f64 = (p.p7 * eq137_e1743_d_n9);
        let eq137_e1745_d_n10: f64 = (p.p7 * eq137_e1743_d_n10);
        let eq137_e1745_d_n11: f64 = (p.p7 * eq137_e1743_d_n11);
        let eq137_e1745_d_n12: f64 = (p.p7 * eq137_e1743_d_n12);
        let eq137_e1745_d_n13: f64 = (p.p7 * eq137_e1743_d_n13);
        let eq137_e1745_d_n14: f64 = (p.p7 * eq137_e1743_d_n14);
        let eq137_e1745_d_n15: f64 = (p.p7 * eq137_e1743_d_n15);
        let eq137_e1745_d_n16: f64 = (p.p7 * eq137_e1743_d_n16);
        let eq137_e1745_d_n17: f64 = (p.p7 * eq137_e1743_d_n17);
        let eq137_e1745_d_n18: f64 = (p.p7 * eq137_e1743_d_n18);
        let eq137_e1745_d_n19: f64 = (p.p7 * eq137_e1743_d_n19);
        let eq137_e1745_d_n20: f64 = (p.p7 * eq137_e1743_d_n20);
        let eq137_e1745_d_n21: f64 = (p.p7 * eq137_e1743_d_n21);
        let eq137_e1745_d_n22: f64 = (p.p7 * eq137_e1743_d_n22);
        let eq137_e1745_q: f64 = (p.p7 * eq137_e1744_q);
        let eq137_e1745_q_d_n0: f64 = (p.p7 * eq137_e1743_d_n0);
        let eq137_e1745_q_d_n1: f64 = (p.p7 * eq137_e1743_d_n1);
        let eq137_e1745_q_d_n2: f64 = (p.p7 * eq137_e1743_d_n2);
        let eq137_e1745_q_d_n3: f64 = (p.p7 * eq137_e1743_d_n3);
        let eq137_e1745_q_d_n4: f64 = (p.p7 * eq137_e1743_d_n4);
        let eq137_e1745_q_d_n5: f64 = (p.p7 * eq137_e1743_d_n5);
        let eq137_e1745_q_d_n6: f64 = (p.p7 * eq137_e1743_d_n6);
        let eq137_e1745_q_d_n7: f64 = (p.p7 * eq137_e1743_d_n7);
        let eq137_e1745_q_d_n8: f64 = (p.p7 * eq137_e1743_d_n8);
        let eq137_e1745_q_d_n9: f64 = (p.p7 * eq137_e1743_d_n9);
        let eq137_e1745_q_d_n10: f64 = (p.p7 * eq137_e1743_d_n10);
        let eq137_e1745_q_d_n11: f64 = (p.p7 * eq137_e1743_d_n11);
        let eq137_e1745_q_d_n12: f64 = (p.p7 * eq137_e1743_d_n12);
        let eq137_e1745_q_d_n13: f64 = (p.p7 * eq137_e1743_d_n13);
        let eq137_e1745_q_d_n14: f64 = (p.p7 * eq137_e1743_d_n14);
        let eq137_e1745_q_d_n15: f64 = (p.p7 * eq137_e1743_d_n15);
        let eq137_e1745_q_d_n16: f64 = (p.p7 * eq137_e1743_d_n16);
        let eq137_e1745_q_d_n17: f64 = (p.p7 * eq137_e1743_d_n17);
        let eq137_e1745_q_d_n18: f64 = (p.p7 * eq137_e1743_d_n18);
        let eq137_e1745_q_d_n19: f64 = (p.p7 * eq137_e1743_d_n19);
        let eq137_e1745_q_d_n20: f64 = (p.p7 * eq137_e1743_d_n20);
        let eq137_e1745_q_d_n21: f64 = (p.p7 * eq137_e1743_d_n21);
        let eq137_e1745_q_d_n22: f64 = (p.p7 * eq137_e1743_d_n22);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22, eq137_e1745_q, eq137_e1745_q_d_n0, eq137_e1745_q_d_n1, eq137_e1745_q_d_n2, eq137_e1745_q_d_n3, eq137_e1745_q_d_n4, eq137_e1745_q_d_n5, eq137_e1745_q_d_n6, eq137_e1745_q_d_n7, eq137_e1745_q_d_n8, eq137_e1745_q_d_n9, eq137_e1745_q_d_n10, eq137_e1745_q_d_n11, eq137_e1745_q_d_n12, eq137_e1745_q_d_n13, eq137_e1745_q_d_n14, eq137_e1745_q_d_n15, eq137_e1745_q_d_n16, eq137_e1745_q_d_n17, eq137_e1745_q_d_n18, eq137_e1745_q_d_n19, eq137_e1745_q_d_n20, eq137_e1745_q_d_n21, eq137_e1745_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_reactive_node_derivatives: [f64; 23] = [eq137_e1747_q_d_n0, eq137_e1747_q_d_n1, eq137_e1747_q_d_n2, eq137_e1747_q_d_n3, eq137_e1747_q_d_n4, eq137_e1747_q_d_n5, eq137_e1747_q_d_n6, eq137_e1747_q_d_n7, eq137_e1747_q_d_n8, eq137_e1747_q_d_n9, eq137_e1747_q_d_n10, eq137_e1747_q_d_n11, eq137_e1747_q_d_n12, eq137_e1747_q_d_n13, eq137_e1747_q_d_n14, eq137_e1747_q_d_n15, eq137_e1747_q_d_n16, eq137_e1747_q_d_n17, eq137_e1747_q_d_n18, eq137_e1747_q_d_n19, eq137_e1747_q_d_n20, eq137_e1747_q_d_n21, eq137_e1747_q_d_n22];
        let eq137_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[19]),
            nodes,
            &eq137_reactive_node_derivatives,
            branches,
            &eq137_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22, eq138_e1757_q, eq138_e1757_q_d_n0, eq138_e1757_q_d_n1, eq138_e1757_q_d_n2, eq138_e1757_q_d_n3, eq138_e1757_q_d_n4, eq138_e1757_q_d_n5, eq138_e1757_q_d_n6, eq138_e1757_q_d_n7, eq138_e1757_q_d_n8, eq138_e1757_q_d_n9, eq138_e1757_q_d_n10, eq138_e1757_q_d_n11, eq138_e1757_q_d_n12, eq138_e1757_q_d_n13, eq138_e1757_q_d_n14, eq138_e1757_q_d_n15, eq138_e1757_q_d_n16, eq138_e1757_q_d_n17, eq138_e1757_q_d_n18, eq138_e1757_q_d_n19, eq138_e1757_q_d_n20, eq138_e1757_q_d_n21, eq138_e1757_q_d_n22,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq138_e1754_q: f64 = s.v[241];
        let eq138_e1755: f64 = (p.p7 * s.v[241]);
        let eq138_e1755_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq138_e1755_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq138_e1755_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq138_e1755_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq138_e1755_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq138_e1755_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq138_e1755_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq138_e1755_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq138_e1755_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq138_e1755_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq138_e1755_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq138_e1755_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq138_e1755_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq138_e1755_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq138_e1755_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq138_e1755_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq138_e1755_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq138_e1755_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq138_e1755_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq138_e1755_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq138_e1755_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq138_e1755_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq138_e1755_d_n22: f64 = (p.p7 * s.dn[241][22]);
        let eq138_e1755_q: f64 = (p.p7 * eq138_e1754_q);
        let eq138_e1755_q_d_n0: f64 = (p.p7 * s.dn[241][0]);
        let eq138_e1755_q_d_n1: f64 = (p.p7 * s.dn[241][1]);
        let eq138_e1755_q_d_n2: f64 = (p.p7 * s.dn[241][2]);
        let eq138_e1755_q_d_n3: f64 = (p.p7 * s.dn[241][3]);
        let eq138_e1755_q_d_n4: f64 = (p.p7 * s.dn[241][4]);
        let eq138_e1755_q_d_n5: f64 = (p.p7 * s.dn[241][5]);
        let eq138_e1755_q_d_n6: f64 = (p.p7 * s.dn[241][6]);
        let eq138_e1755_q_d_n7: f64 = (p.p7 * s.dn[241][7]);
        let eq138_e1755_q_d_n8: f64 = (p.p7 * s.dn[241][8]);
        let eq138_e1755_q_d_n9: f64 = (p.p7 * s.dn[241][9]);
        let eq138_e1755_q_d_n10: f64 = (p.p7 * s.dn[241][10]);
        let eq138_e1755_q_d_n11: f64 = (p.p7 * s.dn[241][11]);
        let eq138_e1755_q_d_n12: f64 = (p.p7 * s.dn[241][12]);
        let eq138_e1755_q_d_n13: f64 = (p.p7 * s.dn[241][13]);
        let eq138_e1755_q_d_n14: f64 = (p.p7 * s.dn[241][14]);
        let eq138_e1755_q_d_n15: f64 = (p.p7 * s.dn[241][15]);
        let eq138_e1755_q_d_n16: f64 = (p.p7 * s.dn[241][16]);
        let eq138_e1755_q_d_n17: f64 = (p.p7 * s.dn[241][17]);
        let eq138_e1755_q_d_n18: f64 = (p.p7 * s.dn[241][18]);
        let eq138_e1755_q_d_n19: f64 = (p.p7 * s.dn[241][19]);
        let eq138_e1755_q_d_n20: f64 = (p.p7 * s.dn[241][20]);
        let eq138_e1755_q_d_n21: f64 = (p.p7 * s.dn[241][21]);
        let eq138_e1755_q_d_n22: f64 = (p.p7 * s.dn[241][22]);
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22, eq138_e1755_q, eq138_e1755_q_d_n0, eq138_e1755_q_d_n1, eq138_e1755_q_d_n2, eq138_e1755_q_d_n3, eq138_e1755_q_d_n4, eq138_e1755_q_d_n5, eq138_e1755_q_d_n6, eq138_e1755_q_d_n7, eq138_e1755_q_d_n8, eq138_e1755_q_d_n9, eq138_e1755_q_d_n10, eq138_e1755_q_d_n11, eq138_e1755_q_d_n12, eq138_e1755_q_d_n13, eq138_e1755_q_d_n14, eq138_e1755_q_d_n15, eq138_e1755_q_d_n16, eq138_e1755_q_d_n17, eq138_e1755_q_d_n18, eq138_e1755_q_d_n19, eq138_e1755_q_d_n20, eq138_e1755_q_d_n21, eq138_e1755_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_reactive_node_derivatives: [f64; 23] = [eq138_e1757_q_d_n0, eq138_e1757_q_d_n1, eq138_e1757_q_d_n2, eq138_e1757_q_d_n3, eq138_e1757_q_d_n4, eq138_e1757_q_d_n5, eq138_e1757_q_d_n6, eq138_e1757_q_d_n7, eq138_e1757_q_d_n8, eq138_e1757_q_d_n9, eq138_e1757_q_d_n10, eq138_e1757_q_d_n11, eq138_e1757_q_d_n12, eq138_e1757_q_d_n13, eq138_e1757_q_d_n14, eq138_e1757_q_d_n15, eq138_e1757_q_d_n16, eq138_e1757_q_d_n17, eq138_e1757_q_d_n18, eq138_e1757_q_d_n19, eq138_e1757_q_d_n20, eq138_e1757_q_d_n21, eq138_e1757_q_d_n22];
        let eq138_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq138_reactive_node_derivatives,
            branches,
            &eq138_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22, eq139_e1769_q, eq139_e1769_q_d_n0, eq139_e1769_q_d_n1, eq139_e1769_q_d_n2, eq139_e1769_q_d_n3, eq139_e1769_q_d_n4, eq139_e1769_q_d_n5, eq139_e1769_q_d_n6, eq139_e1769_q_d_n7, eq139_e1769_q_d_n8, eq139_e1769_q_d_n9, eq139_e1769_q_d_n10, eq139_e1769_q_d_n11, eq139_e1769_q_d_n12, eq139_e1769_q_d_n13, eq139_e1769_q_d_n14, eq139_e1769_q_d_n15, eq139_e1769_q_d_n16, eq139_e1769_q_d_n17, eq139_e1769_q_d_n18, eq139_e1769_q_d_n19, eq139_e1769_q_d_n20, eq139_e1769_q_d_n21, eq139_e1769_q_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq139_e1766_q: f64 = s.v[240];
        let eq139_e1767: f64 = (p.p7 * s.v[240]);
        let eq139_e1767_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq139_e1767_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq139_e1767_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq139_e1767_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq139_e1767_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq139_e1767_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq139_e1767_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq139_e1767_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq139_e1767_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq139_e1767_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq139_e1767_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq139_e1767_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq139_e1767_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq139_e1767_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq139_e1767_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq139_e1767_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq139_e1767_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq139_e1767_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq139_e1767_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq139_e1767_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq139_e1767_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq139_e1767_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq139_e1767_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq139_e1767_q: f64 = (p.p7 * eq139_e1766_q);
        let eq139_e1767_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq139_e1767_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq139_e1767_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq139_e1767_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq139_e1767_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq139_e1767_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq139_e1767_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq139_e1767_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq139_e1767_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq139_e1767_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq139_e1767_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq139_e1767_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq139_e1767_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq139_e1767_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq139_e1767_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq139_e1767_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq139_e1767_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq139_e1767_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq139_e1767_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq139_e1767_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq139_e1767_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq139_e1767_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq139_e1767_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq139_e1767, eq139_e1767_d_n0, eq139_e1767_d_n1, eq139_e1767_d_n2, eq139_e1767_d_n3, eq139_e1767_d_n4, eq139_e1767_d_n5, eq139_e1767_d_n6, eq139_e1767_d_n7, eq139_e1767_d_n8, eq139_e1767_d_n9, eq139_e1767_d_n10, eq139_e1767_d_n11, eq139_e1767_d_n12, eq139_e1767_d_n13, eq139_e1767_d_n14, eq139_e1767_d_n15, eq139_e1767_d_n16, eq139_e1767_d_n17, eq139_e1767_d_n18, eq139_e1767_d_n19, eq139_e1767_d_n20, eq139_e1767_d_n21, eq139_e1767_d_n22, eq139_e1767_q, eq139_e1767_q_d_n0, eq139_e1767_q_d_n1, eq139_e1767_q_d_n2, eq139_e1767_q_d_n3, eq139_e1767_q_d_n4, eq139_e1767_q_d_n5, eq139_e1767_q_d_n6, eq139_e1767_q_d_n7, eq139_e1767_q_d_n8, eq139_e1767_q_d_n9, eq139_e1767_q_d_n10, eq139_e1767_q_d_n11, eq139_e1767_q_d_n12, eq139_e1767_q_d_n13, eq139_e1767_q_d_n14, eq139_e1767_q_d_n15, eq139_e1767_q_d_n16, eq139_e1767_q_d_n17, eq139_e1767_q_d_n18, eq139_e1767_q_d_n19, eq139_e1767_q_d_n20, eq139_e1767_q_d_n21, eq139_e1767_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_reactive_node_derivatives: [f64; 23] = [eq139_e1769_q_d_n0, eq139_e1769_q_d_n1, eq139_e1769_q_d_n2, eq139_e1769_q_d_n3, eq139_e1769_q_d_n4, eq139_e1769_q_d_n5, eq139_e1769_q_d_n6, eq139_e1769_q_d_n7, eq139_e1769_q_d_n8, eq139_e1769_q_d_n9, eq139_e1769_q_d_n10, eq139_e1769_q_d_n11, eq139_e1769_q_d_n12, eq139_e1769_q_d_n13, eq139_e1769_q_d_n14, eq139_e1769_q_d_n15, eq139_e1769_q_d_n16, eq139_e1769_q_d_n17, eq139_e1769_q_d_n18, eq139_e1769_q_d_n19, eq139_e1769_q_d_n20, eq139_e1769_q_d_n21, eq139_e1769_q_d_n22];
        let eq139_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq139_reactive_node_derivatives,
            branches,
            &eq139_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_6(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22, eq140_e1783_q, eq140_e1783_q_d_n0, eq140_e1783_q_d_n1, eq140_e1783_q_d_n2, eq140_e1783_q_d_n3, eq140_e1783_q_d_n4, eq140_e1783_q_d_n5, eq140_e1783_q_d_n6, eq140_e1783_q_d_n7, eq140_e1783_q_d_n8, eq140_e1783_q_d_n9, eq140_e1783_q_d_n10, eq140_e1783_q_d_n11, eq140_e1783_q_d_n12, eq140_e1783_q_d_n13, eq140_e1783_q_d_n14, eq140_e1783_q_d_n15, eq140_e1783_q_d_n16, eq140_e1783_q_d_n17, eq140_e1783_q_d_n18, eq140_e1783_q_d_n19, eq140_e1783_q_d_n20, eq140_e1783_q_d_n21, eq140_e1783_q_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq140_e1778_q: f64 = s.v[240];
        let eq140_e1779: f64 = (p.p7 * s.v[240]);
        let eq140_e1779_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq140_e1779_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq140_e1779_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq140_e1779_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq140_e1779_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq140_e1779_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq140_e1779_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq140_e1779_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq140_e1779_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq140_e1779_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq140_e1779_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq140_e1779_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq140_e1779_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq140_e1779_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq140_e1779_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq140_e1779_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq140_e1779_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq140_e1779_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq140_e1779_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq140_e1779_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq140_e1779_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq140_e1779_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq140_e1779_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq140_e1779_q: f64 = (p.p7 * eq140_e1778_q);
        let eq140_e1779_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq140_e1779_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq140_e1779_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq140_e1779_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq140_e1779_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq140_e1779_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq140_e1779_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq140_e1779_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq140_e1779_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq140_e1779_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq140_e1779_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq140_e1779_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq140_e1779_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq140_e1779_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq140_e1779_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq140_e1779_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq140_e1779_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq140_e1779_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq140_e1779_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq140_e1779_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq140_e1779_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq140_e1779_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq140_e1779_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (eq140_e1779_d_n0 * p.p246);
        let eq140_e1781_d_n1: f64 = (eq140_e1779_d_n1 * p.p246);
        let eq140_e1781_d_n2: f64 = (eq140_e1779_d_n2 * p.p246);
        let eq140_e1781_d_n3: f64 = (eq140_e1779_d_n3 * p.p246);
        let eq140_e1781_d_n4: f64 = (eq140_e1779_d_n4 * p.p246);
        let eq140_e1781_d_n5: f64 = (eq140_e1779_d_n5 * p.p246);
        let eq140_e1781_d_n6: f64 = (eq140_e1779_d_n6 * p.p246);
        let eq140_e1781_d_n7: f64 = (eq140_e1779_d_n7 * p.p246);
        let eq140_e1781_d_n8: f64 = (eq140_e1779_d_n8 * p.p246);
        let eq140_e1781_d_n9: f64 = (eq140_e1779_d_n9 * p.p246);
        let eq140_e1781_d_n10: f64 = (eq140_e1779_d_n10 * p.p246);
        let eq140_e1781_d_n11: f64 = (eq140_e1779_d_n11 * p.p246);
        let eq140_e1781_d_n12: f64 = (eq140_e1779_d_n12 * p.p246);
        let eq140_e1781_d_n13: f64 = (eq140_e1779_d_n13 * p.p246);
        let eq140_e1781_d_n14: f64 = (eq140_e1779_d_n14 * p.p246);
        let eq140_e1781_d_n15: f64 = (eq140_e1779_d_n15 * p.p246);
        let eq140_e1781_d_n16: f64 = (eq140_e1779_d_n16 * p.p246);
        let eq140_e1781_d_n17: f64 = (eq140_e1779_d_n17 * p.p246);
        let eq140_e1781_d_n18: f64 = (eq140_e1779_d_n18 * p.p246);
        let eq140_e1781_d_n19: f64 = (eq140_e1779_d_n19 * p.p246);
        let eq140_e1781_d_n20: f64 = (eq140_e1779_d_n20 * p.p246);
        let eq140_e1781_d_n21: f64 = (eq140_e1779_d_n21 * p.p246);
        let eq140_e1781_d_n22: f64 = (eq140_e1779_d_n22 * p.p246);
        let eq140_e1781_q: f64 = (eq140_e1779_q * p.p246);
        let eq140_e1781_q_d_n0: f64 = (eq140_e1779_q_d_n0 * p.p246);
        let eq140_e1781_q_d_n1: f64 = (eq140_e1779_q_d_n1 * p.p246);
        let eq140_e1781_q_d_n2: f64 = (eq140_e1779_q_d_n2 * p.p246);
        let eq140_e1781_q_d_n3: f64 = (eq140_e1779_q_d_n3 * p.p246);
        let eq140_e1781_q_d_n4: f64 = (eq140_e1779_q_d_n4 * p.p246);
        let eq140_e1781_q_d_n5: f64 = (eq140_e1779_q_d_n5 * p.p246);
        let eq140_e1781_q_d_n6: f64 = (eq140_e1779_q_d_n6 * p.p246);
        let eq140_e1781_q_d_n7: f64 = (eq140_e1779_q_d_n7 * p.p246);
        let eq140_e1781_q_d_n8: f64 = (eq140_e1779_q_d_n8 * p.p246);
        let eq140_e1781_q_d_n9: f64 = (eq140_e1779_q_d_n9 * p.p246);
        let eq140_e1781_q_d_n10: f64 = (eq140_e1779_q_d_n10 * p.p246);
        let eq140_e1781_q_d_n11: f64 = (eq140_e1779_q_d_n11 * p.p246);
        let eq140_e1781_q_d_n12: f64 = (eq140_e1779_q_d_n12 * p.p246);
        let eq140_e1781_q_d_n13: f64 = (eq140_e1779_q_d_n13 * p.p246);
        let eq140_e1781_q_d_n14: f64 = (eq140_e1779_q_d_n14 * p.p246);
        let eq140_e1781_q_d_n15: f64 = (eq140_e1779_q_d_n15 * p.p246);
        let eq140_e1781_q_d_n16: f64 = (eq140_e1779_q_d_n16 * p.p246);
        let eq140_e1781_q_d_n17: f64 = (eq140_e1779_q_d_n17 * p.p246);
        let eq140_e1781_q_d_n18: f64 = (eq140_e1779_q_d_n18 * p.p246);
        let eq140_e1781_q_d_n19: f64 = (eq140_e1779_q_d_n19 * p.p246);
        let eq140_e1781_q_d_n20: f64 = (eq140_e1779_q_d_n20 * p.p246);
        let eq140_e1781_q_d_n21: f64 = (eq140_e1779_q_d_n21 * p.p246);
        let eq140_e1781_q_d_n22: f64 = (eq140_e1779_q_d_n22 * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22, eq140_e1781_q, eq140_e1781_q_d_n0, eq140_e1781_q_d_n1, eq140_e1781_q_d_n2, eq140_e1781_q_d_n3, eq140_e1781_q_d_n4, eq140_e1781_q_d_n5, eq140_e1781_q_d_n6, eq140_e1781_q_d_n7, eq140_e1781_q_d_n8, eq140_e1781_q_d_n9, eq140_e1781_q_d_n10, eq140_e1781_q_d_n11, eq140_e1781_q_d_n12, eq140_e1781_q_d_n13, eq140_e1781_q_d_n14, eq140_e1781_q_d_n15, eq140_e1781_q_d_n16, eq140_e1781_q_d_n17, eq140_e1781_q_d_n18, eq140_e1781_q_d_n19, eq140_e1781_q_d_n20, eq140_e1781_q_d_n21, eq140_e1781_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_reactive_node_derivatives: [f64; 23] = [eq140_e1783_q_d_n0, eq140_e1783_q_d_n1, eq140_e1783_q_d_n2, eq140_e1783_q_d_n3, eq140_e1783_q_d_n4, eq140_e1783_q_d_n5, eq140_e1783_q_d_n6, eq140_e1783_q_d_n7, eq140_e1783_q_d_n8, eq140_e1783_q_d_n9, eq140_e1783_q_d_n10, eq140_e1783_q_d_n11, eq140_e1783_q_d_n12, eq140_e1783_q_d_n13, eq140_e1783_q_d_n14, eq140_e1783_q_d_n15, eq140_e1783_q_d_n16, eq140_e1783_q_d_n17, eq140_e1783_q_d_n18, eq140_e1783_q_d_n19, eq140_e1783_q_d_n20, eq140_e1783_q_d_n21, eq140_e1783_q_d_n22];
        let eq140_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq140_reactive_node_derivatives,
            branches,
            &eq140_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22, eq141_e1796_q, eq141_e1796_q_d_n0, eq141_e1796_q_d_n1, eq141_e1796_q_d_n2, eq141_e1796_q_d_n3, eq141_e1796_q_d_n4, eq141_e1796_q_d_n5, eq141_e1796_q_d_n6, eq141_e1796_q_d_n7, eq141_e1796_q_d_n8, eq141_e1796_q_d_n9, eq141_e1796_q_d_n10, eq141_e1796_q_d_n11, eq141_e1796_q_d_n12, eq141_e1796_q_d_n13, eq141_e1796_q_d_n14, eq141_e1796_q_d_n15, eq141_e1796_q_d_n16, eq141_e1796_q_d_n17, eq141_e1796_q_d_n18, eq141_e1796_q_d_n19, eq141_e1796_q_d_n20, eq141_e1796_q_d_n21, eq141_e1796_q_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq141_e1793_q: f64 = s.v[240];
        let eq141_e1794: f64 = (p.p7 * s.v[240]);
        let eq141_e1794_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq141_e1794_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq141_e1794_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq141_e1794_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq141_e1794_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq141_e1794_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq141_e1794_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq141_e1794_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq141_e1794_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq141_e1794_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq141_e1794_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq141_e1794_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq141_e1794_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq141_e1794_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq141_e1794_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq141_e1794_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq141_e1794_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq141_e1794_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq141_e1794_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq141_e1794_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq141_e1794_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq141_e1794_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq141_e1794_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq141_e1794_q: f64 = (p.p7 * eq141_e1793_q);
        let eq141_e1794_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq141_e1794_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq141_e1794_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq141_e1794_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq141_e1794_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq141_e1794_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq141_e1794_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq141_e1794_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq141_e1794_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq141_e1794_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq141_e1794_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq141_e1794_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq141_e1794_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq141_e1794_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq141_e1794_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq141_e1794_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq141_e1794_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq141_e1794_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq141_e1794_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq141_e1794_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq141_e1794_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq141_e1794_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq141_e1794_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        (eq141_e1794, eq141_e1794_d_n0, eq141_e1794_d_n1, eq141_e1794_d_n2, eq141_e1794_d_n3, eq141_e1794_d_n4, eq141_e1794_d_n5, eq141_e1794_d_n6, eq141_e1794_d_n7, eq141_e1794_d_n8, eq141_e1794_d_n9, eq141_e1794_d_n10, eq141_e1794_d_n11, eq141_e1794_d_n12, eq141_e1794_d_n13, eq141_e1794_d_n14, eq141_e1794_d_n15, eq141_e1794_d_n16, eq141_e1794_d_n17, eq141_e1794_d_n18, eq141_e1794_d_n19, eq141_e1794_d_n20, eq141_e1794_d_n21, eq141_e1794_d_n22, eq141_e1794_q, eq141_e1794_q_d_n0, eq141_e1794_q_d_n1, eq141_e1794_q_d_n2, eq141_e1794_q_d_n3, eq141_e1794_q_d_n4, eq141_e1794_q_d_n5, eq141_e1794_q_d_n6, eq141_e1794_q_d_n7, eq141_e1794_q_d_n8, eq141_e1794_q_d_n9, eq141_e1794_q_d_n10, eq141_e1794_q_d_n11, eq141_e1794_q_d_n12, eq141_e1794_q_d_n13, eq141_e1794_q_d_n14, eq141_e1794_q_d_n15, eq141_e1794_q_d_n16, eq141_e1794_q_d_n17, eq141_e1794_q_d_n18, eq141_e1794_q_d_n19, eq141_e1794_q_d_n20, eq141_e1794_q_d_n21, eq141_e1794_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_reactive_node_derivatives: [f64; 23] = [eq141_e1796_q_d_n0, eq141_e1796_q_d_n1, eq141_e1796_q_d_n2, eq141_e1796_q_d_n3, eq141_e1796_q_d_n4, eq141_e1796_q_d_n5, eq141_e1796_q_d_n6, eq141_e1796_q_d_n7, eq141_e1796_q_d_n8, eq141_e1796_q_d_n9, eq141_e1796_q_d_n10, eq141_e1796_q_d_n11, eq141_e1796_q_d_n12, eq141_e1796_q_d_n13, eq141_e1796_q_d_n14, eq141_e1796_q_d_n15, eq141_e1796_q_d_n16, eq141_e1796_q_d_n17, eq141_e1796_q_d_n18, eq141_e1796_q_d_n19, eq141_e1796_q_d_n20, eq141_e1796_q_d_n21, eq141_e1796_q_d_n22];
        let eq141_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq141_reactive_node_derivatives,
            branches,
            &eq141_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22, eq142_e1811_q, eq142_e1811_q_d_n0, eq142_e1811_q_d_n1, eq142_e1811_q_d_n2, eq142_e1811_q_d_n3, eq142_e1811_q_d_n4, eq142_e1811_q_d_n5, eq142_e1811_q_d_n6, eq142_e1811_q_d_n7, eq142_e1811_q_d_n8, eq142_e1811_q_d_n9, eq142_e1811_q_d_n10, eq142_e1811_q_d_n11, eq142_e1811_q_d_n12, eq142_e1811_q_d_n13, eq142_e1811_q_d_n14, eq142_e1811_q_d_n15, eq142_e1811_q_d_n16, eq142_e1811_q_d_n17, eq142_e1811_q_d_n18, eq142_e1811_q_d_n19, eq142_e1811_q_d_n20, eq142_e1811_q_d_n21, eq142_e1811_q_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq142_e1806_q: f64 = s.v[240];
        let eq142_e1807: f64 = (p.p7 * s.v[240]);
        let eq142_e1807_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq142_e1807_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq142_e1807_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq142_e1807_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq142_e1807_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq142_e1807_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq142_e1807_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq142_e1807_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq142_e1807_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq142_e1807_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq142_e1807_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq142_e1807_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq142_e1807_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq142_e1807_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq142_e1807_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq142_e1807_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq142_e1807_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq142_e1807_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq142_e1807_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq142_e1807_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq142_e1807_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq142_e1807_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq142_e1807_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq142_e1807_q: f64 = (p.p7 * eq142_e1806_q);
        let eq142_e1807_q_d_n0: f64 = (p.p7 * s.dn[240][0]);
        let eq142_e1807_q_d_n1: f64 = (p.p7 * s.dn[240][1]);
        let eq142_e1807_q_d_n2: f64 = (p.p7 * s.dn[240][2]);
        let eq142_e1807_q_d_n3: f64 = (p.p7 * s.dn[240][3]);
        let eq142_e1807_q_d_n4: f64 = (p.p7 * s.dn[240][4]);
        let eq142_e1807_q_d_n5: f64 = (p.p7 * s.dn[240][5]);
        let eq142_e1807_q_d_n6: f64 = (p.p7 * s.dn[240][6]);
        let eq142_e1807_q_d_n7: f64 = (p.p7 * s.dn[240][7]);
        let eq142_e1807_q_d_n8: f64 = (p.p7 * s.dn[240][8]);
        let eq142_e1807_q_d_n9: f64 = (p.p7 * s.dn[240][9]);
        let eq142_e1807_q_d_n10: f64 = (p.p7 * s.dn[240][10]);
        let eq142_e1807_q_d_n11: f64 = (p.p7 * s.dn[240][11]);
        let eq142_e1807_q_d_n12: f64 = (p.p7 * s.dn[240][12]);
        let eq142_e1807_q_d_n13: f64 = (p.p7 * s.dn[240][13]);
        let eq142_e1807_q_d_n14: f64 = (p.p7 * s.dn[240][14]);
        let eq142_e1807_q_d_n15: f64 = (p.p7 * s.dn[240][15]);
        let eq142_e1807_q_d_n16: f64 = (p.p7 * s.dn[240][16]);
        let eq142_e1807_q_d_n17: f64 = (p.p7 * s.dn[240][17]);
        let eq142_e1807_q_d_n18: f64 = (p.p7 * s.dn[240][18]);
        let eq142_e1807_q_d_n19: f64 = (p.p7 * s.dn[240][19]);
        let eq142_e1807_q_d_n20: f64 = (p.p7 * s.dn[240][20]);
        let eq142_e1807_q_d_n21: f64 = (p.p7 * s.dn[240][21]);
        let eq142_e1807_q_d_n22: f64 = (p.p7 * s.dn[240][22]);
        let eq142_e1809: f64 = (eq142_e1807 * p.p246);
        let eq142_e1809_d_n0: f64 = (eq142_e1807_d_n0 * p.p246);
        let eq142_e1809_d_n1: f64 = (eq142_e1807_d_n1 * p.p246);
        let eq142_e1809_d_n2: f64 = (eq142_e1807_d_n2 * p.p246);
        let eq142_e1809_d_n3: f64 = (eq142_e1807_d_n3 * p.p246);
        let eq142_e1809_d_n4: f64 = (eq142_e1807_d_n4 * p.p246);
        let eq142_e1809_d_n5: f64 = (eq142_e1807_d_n5 * p.p246);
        let eq142_e1809_d_n6: f64 = (eq142_e1807_d_n6 * p.p246);
        let eq142_e1809_d_n7: f64 = (eq142_e1807_d_n7 * p.p246);
        let eq142_e1809_d_n8: f64 = (eq142_e1807_d_n8 * p.p246);
        let eq142_e1809_d_n9: f64 = (eq142_e1807_d_n9 * p.p246);
        let eq142_e1809_d_n10: f64 = (eq142_e1807_d_n10 * p.p246);
        let eq142_e1809_d_n11: f64 = (eq142_e1807_d_n11 * p.p246);
        let eq142_e1809_d_n12: f64 = (eq142_e1807_d_n12 * p.p246);
        let eq142_e1809_d_n13: f64 = (eq142_e1807_d_n13 * p.p246);
        let eq142_e1809_d_n14: f64 = (eq142_e1807_d_n14 * p.p246);
        let eq142_e1809_d_n15: f64 = (eq142_e1807_d_n15 * p.p246);
        let eq142_e1809_d_n16: f64 = (eq142_e1807_d_n16 * p.p246);
        let eq142_e1809_d_n17: f64 = (eq142_e1807_d_n17 * p.p246);
        let eq142_e1809_d_n18: f64 = (eq142_e1807_d_n18 * p.p246);
        let eq142_e1809_d_n19: f64 = (eq142_e1807_d_n19 * p.p246);
        let eq142_e1809_d_n20: f64 = (eq142_e1807_d_n20 * p.p246);
        let eq142_e1809_d_n21: f64 = (eq142_e1807_d_n21 * p.p246);
        let eq142_e1809_d_n22: f64 = (eq142_e1807_d_n22 * p.p246);
        let eq142_e1809_q: f64 = (eq142_e1807_q * p.p246);
        let eq142_e1809_q_d_n0: f64 = (eq142_e1807_q_d_n0 * p.p246);
        let eq142_e1809_q_d_n1: f64 = (eq142_e1807_q_d_n1 * p.p246);
        let eq142_e1809_q_d_n2: f64 = (eq142_e1807_q_d_n2 * p.p246);
        let eq142_e1809_q_d_n3: f64 = (eq142_e1807_q_d_n3 * p.p246);
        let eq142_e1809_q_d_n4: f64 = (eq142_e1807_q_d_n4 * p.p246);
        let eq142_e1809_q_d_n5: f64 = (eq142_e1807_q_d_n5 * p.p246);
        let eq142_e1809_q_d_n6: f64 = (eq142_e1807_q_d_n6 * p.p246);
        let eq142_e1809_q_d_n7: f64 = (eq142_e1807_q_d_n7 * p.p246);
        let eq142_e1809_q_d_n8: f64 = (eq142_e1807_q_d_n8 * p.p246);
        let eq142_e1809_q_d_n9: f64 = (eq142_e1807_q_d_n9 * p.p246);
        let eq142_e1809_q_d_n10: f64 = (eq142_e1807_q_d_n10 * p.p246);
        let eq142_e1809_q_d_n11: f64 = (eq142_e1807_q_d_n11 * p.p246);
        let eq142_e1809_q_d_n12: f64 = (eq142_e1807_q_d_n12 * p.p246);
        let eq142_e1809_q_d_n13: f64 = (eq142_e1807_q_d_n13 * p.p246);
        let eq142_e1809_q_d_n14: f64 = (eq142_e1807_q_d_n14 * p.p246);
        let eq142_e1809_q_d_n15: f64 = (eq142_e1807_q_d_n15 * p.p246);
        let eq142_e1809_q_d_n16: f64 = (eq142_e1807_q_d_n16 * p.p246);
        let eq142_e1809_q_d_n17: f64 = (eq142_e1807_q_d_n17 * p.p246);
        let eq142_e1809_q_d_n18: f64 = (eq142_e1807_q_d_n18 * p.p246);
        let eq142_e1809_q_d_n19: f64 = (eq142_e1807_q_d_n19 * p.p246);
        let eq142_e1809_q_d_n20: f64 = (eq142_e1807_q_d_n20 * p.p246);
        let eq142_e1809_q_d_n21: f64 = (eq142_e1807_q_d_n21 * p.p246);
        let eq142_e1809_q_d_n22: f64 = (eq142_e1807_q_d_n22 * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22, eq142_e1809_q, eq142_e1809_q_d_n0, eq142_e1809_q_d_n1, eq142_e1809_q_d_n2, eq142_e1809_q_d_n3, eq142_e1809_q_d_n4, eq142_e1809_q_d_n5, eq142_e1809_q_d_n6, eq142_e1809_q_d_n7, eq142_e1809_q_d_n8, eq142_e1809_q_d_n9, eq142_e1809_q_d_n10, eq142_e1809_q_d_n11, eq142_e1809_q_d_n12, eq142_e1809_q_d_n13, eq142_e1809_q_d_n14, eq142_e1809_q_d_n15, eq142_e1809_q_d_n16, eq142_e1809_q_d_n17, eq142_e1809_q_d_n18, eq142_e1809_q_d_n19, eq142_e1809_q_d_n20, eq142_e1809_q_d_n21, eq142_e1809_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_reactive_node_derivatives: [f64; 23] = [eq142_e1811_q_d_n0, eq142_e1811_q_d_n1, eq142_e1811_q_d_n2, eq142_e1811_q_d_n3, eq142_e1811_q_d_n4, eq142_e1811_q_d_n5, eq142_e1811_q_d_n6, eq142_e1811_q_d_n7, eq142_e1811_q_d_n8, eq142_e1811_q_d_n9, eq142_e1811_q_d_n10, eq142_e1811_q_d_n11, eq142_e1811_q_d_n12, eq142_e1811_q_d_n13, eq142_e1811_q_d_n14, eq142_e1811_q_d_n15, eq142_e1811_q_d_n16, eq142_e1811_q_d_n17, eq142_e1811_q_d_n18, eq142_e1811_q_d_n19, eq142_e1811_q_d_n20, eq142_e1811_q_d_n21, eq142_e1811_q_d_n22];
        let eq142_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq142_reactive_node_derivatives,
            branches,
            &eq142_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22, eq143_e1823_q, eq143_e1823_q_d_n0, eq143_e1823_q_d_n1, eq143_e1823_q_d_n2, eq143_e1823_q_d_n3, eq143_e1823_q_d_n4, eq143_e1823_q_d_n5, eq143_e1823_q_d_n6, eq143_e1823_q_d_n7, eq143_e1823_q_d_n8, eq143_e1823_q_d_n9, eq143_e1823_q_d_n10, eq143_e1823_q_d_n11, eq143_e1823_q_d_n12, eq143_e1823_q_d_n13, eq143_e1823_q_d_n14, eq143_e1823_q_d_n15, eq143_e1823_q_d_n16, eq143_e1823_q_d_n17, eq143_e1823_q_d_n18, eq143_e1823_q_d_n19, eq143_e1823_q_d_n20, eq143_e1823_q_d_n21, eq143_e1823_q_d_n22,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq143_e1819: f64 = (p.p251 * s.v[240]);
        let eq143_e1819_d_n0: f64 = (p.p251 * s.dn[240][0]);
        let eq143_e1819_d_n1: f64 = (p.p251 * s.dn[240][1]);
        let eq143_e1819_d_n2: f64 = (p.p251 * s.dn[240][2]);
        let eq143_e1819_d_n3: f64 = (p.p251 * s.dn[240][3]);
        let eq143_e1819_d_n4: f64 = (p.p251 * s.dn[240][4]);
        let eq143_e1819_d_n5: f64 = (p.p251 * s.dn[240][5]);
        let eq143_e1819_d_n6: f64 = (p.p251 * s.dn[240][6]);
        let eq143_e1819_d_n7: f64 = (p.p251 * s.dn[240][7]);
        let eq143_e1819_d_n8: f64 = (p.p251 * s.dn[240][8]);
        let eq143_e1819_d_n9: f64 = (p.p251 * s.dn[240][9]);
        let eq143_e1819_d_n10: f64 = (p.p251 * s.dn[240][10]);
        let eq143_e1819_d_n11: f64 = (p.p251 * s.dn[240][11]);
        let eq143_e1819_d_n12: f64 = (p.p251 * s.dn[240][12]);
        let eq143_e1819_d_n13: f64 = (p.p251 * s.dn[240][13]);
        let eq143_e1819_d_n14: f64 = (p.p251 * s.dn[240][14]);
        let eq143_e1819_d_n15: f64 = (p.p251 * s.dn[240][15]);
        let eq143_e1819_d_n16: f64 = (p.p251 * s.dn[240][16]);
        let eq143_e1819_d_n17: f64 = (p.p251 * s.dn[240][17]);
        let eq143_e1819_d_n18: f64 = (p.p251 * s.dn[240][18]);
        let eq143_e1819_d_n19: f64 = (p.p251 * s.dn[240][19]);
        let eq143_e1819_d_n20: f64 = (p.p251 * s.dn[240][20]);
        let eq143_e1819_d_n21: f64 = (p.p251 * s.dn[240][21]);
        let eq143_e1819_d_n22: f64 = (p.p251 * s.dn[240][22]);
        let eq143_e1820_q: f64 = eq143_e1819;
        let eq143_e1821: f64 = (p.p7 * eq143_e1819);
        let eq143_e1821_d_n0: f64 = (p.p7 * eq143_e1819_d_n0);
        let eq143_e1821_d_n1: f64 = (p.p7 * eq143_e1819_d_n1);
        let eq143_e1821_d_n2: f64 = (p.p7 * eq143_e1819_d_n2);
        let eq143_e1821_d_n3: f64 = (p.p7 * eq143_e1819_d_n3);
        let eq143_e1821_d_n4: f64 = (p.p7 * eq143_e1819_d_n4);
        let eq143_e1821_d_n5: f64 = (p.p7 * eq143_e1819_d_n5);
        let eq143_e1821_d_n6: f64 = (p.p7 * eq143_e1819_d_n6);
        let eq143_e1821_d_n7: f64 = (p.p7 * eq143_e1819_d_n7);
        let eq143_e1821_d_n8: f64 = (p.p7 * eq143_e1819_d_n8);
        let eq143_e1821_d_n9: f64 = (p.p7 * eq143_e1819_d_n9);
        let eq143_e1821_d_n10: f64 = (p.p7 * eq143_e1819_d_n10);
        let eq143_e1821_d_n11: f64 = (p.p7 * eq143_e1819_d_n11);
        let eq143_e1821_d_n12: f64 = (p.p7 * eq143_e1819_d_n12);
        let eq143_e1821_d_n13: f64 = (p.p7 * eq143_e1819_d_n13);
        let eq143_e1821_d_n14: f64 = (p.p7 * eq143_e1819_d_n14);
        let eq143_e1821_d_n15: f64 = (p.p7 * eq143_e1819_d_n15);
        let eq143_e1821_d_n16: f64 = (p.p7 * eq143_e1819_d_n16);
        let eq143_e1821_d_n17: f64 = (p.p7 * eq143_e1819_d_n17);
        let eq143_e1821_d_n18: f64 = (p.p7 * eq143_e1819_d_n18);
        let eq143_e1821_d_n19: f64 = (p.p7 * eq143_e1819_d_n19);
        let eq143_e1821_d_n20: f64 = (p.p7 * eq143_e1819_d_n20);
        let eq143_e1821_d_n21: f64 = (p.p7 * eq143_e1819_d_n21);
        let eq143_e1821_d_n22: f64 = (p.p7 * eq143_e1819_d_n22);
        let eq143_e1821_q: f64 = (p.p7 * eq143_e1820_q);
        let eq143_e1821_q_d_n0: f64 = (p.p7 * eq143_e1819_d_n0);
        let eq143_e1821_q_d_n1: f64 = (p.p7 * eq143_e1819_d_n1);
        let eq143_e1821_q_d_n2: f64 = (p.p7 * eq143_e1819_d_n2);
        let eq143_e1821_q_d_n3: f64 = (p.p7 * eq143_e1819_d_n3);
        let eq143_e1821_q_d_n4: f64 = (p.p7 * eq143_e1819_d_n4);
        let eq143_e1821_q_d_n5: f64 = (p.p7 * eq143_e1819_d_n5);
        let eq143_e1821_q_d_n6: f64 = (p.p7 * eq143_e1819_d_n6);
        let eq143_e1821_q_d_n7: f64 = (p.p7 * eq143_e1819_d_n7);
        let eq143_e1821_q_d_n8: f64 = (p.p7 * eq143_e1819_d_n8);
        let eq143_e1821_q_d_n9: f64 = (p.p7 * eq143_e1819_d_n9);
        let eq143_e1821_q_d_n10: f64 = (p.p7 * eq143_e1819_d_n10);
        let eq143_e1821_q_d_n11: f64 = (p.p7 * eq143_e1819_d_n11);
        let eq143_e1821_q_d_n12: f64 = (p.p7 * eq143_e1819_d_n12);
        let eq143_e1821_q_d_n13: f64 = (p.p7 * eq143_e1819_d_n13);
        let eq143_e1821_q_d_n14: f64 = (p.p7 * eq143_e1819_d_n14);
        let eq143_e1821_q_d_n15: f64 = (p.p7 * eq143_e1819_d_n15);
        let eq143_e1821_q_d_n16: f64 = (p.p7 * eq143_e1819_d_n16);
        let eq143_e1821_q_d_n17: f64 = (p.p7 * eq143_e1819_d_n17);
        let eq143_e1821_q_d_n18: f64 = (p.p7 * eq143_e1819_d_n18);
        let eq143_e1821_q_d_n19: f64 = (p.p7 * eq143_e1819_d_n19);
        let eq143_e1821_q_d_n20: f64 = (p.p7 * eq143_e1819_d_n20);
        let eq143_e1821_q_d_n21: f64 = (p.p7 * eq143_e1819_d_n21);
        let eq143_e1821_q_d_n22: f64 = (p.p7 * eq143_e1819_d_n22);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22, eq143_e1821_q, eq143_e1821_q_d_n0, eq143_e1821_q_d_n1, eq143_e1821_q_d_n2, eq143_e1821_q_d_n3, eq143_e1821_q_d_n4, eq143_e1821_q_d_n5, eq143_e1821_q_d_n6, eq143_e1821_q_d_n7, eq143_e1821_q_d_n8, eq143_e1821_q_d_n9, eq143_e1821_q_d_n10, eq143_e1821_q_d_n11, eq143_e1821_q_d_n12, eq143_e1821_q_d_n13, eq143_e1821_q_d_n14, eq143_e1821_q_d_n15, eq143_e1821_q_d_n16, eq143_e1821_q_d_n17, eq143_e1821_q_d_n18, eq143_e1821_q_d_n19, eq143_e1821_q_d_n20, eq143_e1821_q_d_n21, eq143_e1821_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_reactive_node_derivatives: [f64; 23] = [eq143_e1823_q_d_n0, eq143_e1823_q_d_n1, eq143_e1823_q_d_n2, eq143_e1823_q_d_n3, eq143_e1823_q_d_n4, eq143_e1823_q_d_n5, eq143_e1823_q_d_n6, eq143_e1823_q_d_n7, eq143_e1823_q_d_n8, eq143_e1823_q_d_n9, eq143_e1823_q_d_n10, eq143_e1823_q_d_n11, eq143_e1823_q_d_n12, eq143_e1823_q_d_n13, eq143_e1823_q_d_n14, eq143_e1823_q_d_n15, eq143_e1823_q_d_n16, eq143_e1823_q_d_n17, eq143_e1823_q_d_n18, eq143_e1823_q_d_n19, eq143_e1823_q_d_n20, eq143_e1823_q_d_n21, eq143_e1823_q_d_n22];
        let eq143_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq143_reactive_node_derivatives,
            branches,
            &eq143_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22, eq144_e1832_q, eq144_e1832_q_d_n0, eq144_e1832_q_d_n1, eq144_e1832_q_d_n2, eq144_e1832_q_d_n3, eq144_e1832_q_d_n4, eq144_e1832_q_d_n5, eq144_e1832_q_d_n6, eq144_e1832_q_d_n7, eq144_e1832_q_d_n8, eq144_e1832_q_d_n9, eq144_e1832_q_d_n10, eq144_e1832_q_d_n11, eq144_e1832_q_d_n12, eq144_e1832_q_d_n13, eq144_e1832_q_d_n14, eq144_e1832_q_d_n15, eq144_e1832_q_d_n16, eq144_e1832_q_d_n17, eq144_e1832_q_d_n18, eq144_e1832_q_d_n19, eq144_e1832_q_d_n20, eq144_e1832_q_d_n21, eq144_e1832_q_d_n22,) = {
    if (s.b[580] && s.b[581]) {
        let eq144_e1829_q: f64 = s.v[253];
        let eq144_e1830: f64 = (p.p7 * s.v[253]);
        let eq144_e1830_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq144_e1830_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq144_e1830_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq144_e1830_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq144_e1830_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq144_e1830_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq144_e1830_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq144_e1830_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq144_e1830_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq144_e1830_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq144_e1830_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq144_e1830_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq144_e1830_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq144_e1830_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq144_e1830_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq144_e1830_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq144_e1830_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq144_e1830_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq144_e1830_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq144_e1830_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq144_e1830_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq144_e1830_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq144_e1830_d_n22: f64 = (p.p7 * s.dn[253][22]);
        let eq144_e1830_q: f64 = (p.p7 * eq144_e1829_q);
        let eq144_e1830_q_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq144_e1830_q_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq144_e1830_q_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq144_e1830_q_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq144_e1830_q_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq144_e1830_q_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq144_e1830_q_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq144_e1830_q_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq144_e1830_q_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq144_e1830_q_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq144_e1830_q_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq144_e1830_q_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq144_e1830_q_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq144_e1830_q_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq144_e1830_q_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq144_e1830_q_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq144_e1830_q_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq144_e1830_q_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq144_e1830_q_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq144_e1830_q_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq144_e1830_q_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq144_e1830_q_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq144_e1830_q_d_n22: f64 = (p.p7 * s.dn[253][22]);
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22, eq144_e1830_q, eq144_e1830_q_d_n0, eq144_e1830_q_d_n1, eq144_e1830_q_d_n2, eq144_e1830_q_d_n3, eq144_e1830_q_d_n4, eq144_e1830_q_d_n5, eq144_e1830_q_d_n6, eq144_e1830_q_d_n7, eq144_e1830_q_d_n8, eq144_e1830_q_d_n9, eq144_e1830_q_d_n10, eq144_e1830_q_d_n11, eq144_e1830_q_d_n12, eq144_e1830_q_d_n13, eq144_e1830_q_d_n14, eq144_e1830_q_d_n15, eq144_e1830_q_d_n16, eq144_e1830_q_d_n17, eq144_e1830_q_d_n18, eq144_e1830_q_d_n19, eq144_e1830_q_d_n20, eq144_e1830_q_d_n21, eq144_e1830_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_reactive_node_derivatives: [f64; 23] = [eq144_e1832_q_d_n0, eq144_e1832_q_d_n1, eq144_e1832_q_d_n2, eq144_e1832_q_d_n3, eq144_e1832_q_d_n4, eq144_e1832_q_d_n5, eq144_e1832_q_d_n6, eq144_e1832_q_d_n7, eq144_e1832_q_d_n8, eq144_e1832_q_d_n9, eq144_e1832_q_d_n10, eq144_e1832_q_d_n11, eq144_e1832_q_d_n12, eq144_e1832_q_d_n13, eq144_e1832_q_d_n14, eq144_e1832_q_d_n15, eq144_e1832_q_d_n16, eq144_e1832_q_d_n17, eq144_e1832_q_d_n18, eq144_e1832_q_d_n19, eq144_e1832_q_d_n20, eq144_e1832_q_d_n21, eq144_e1832_q_d_n22];
        let eq144_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            nodes,
            &eq144_reactive_node_derivatives,
            branches,
            &eq144_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_7(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22, eq145_e1843_q, eq145_e1843_q_d_n0, eq145_e1843_q_d_n1, eq145_e1843_q_d_n2, eq145_e1843_q_d_n3, eq145_e1843_q_d_n4, eq145_e1843_q_d_n5, eq145_e1843_q_d_n6, eq145_e1843_q_d_n7, eq145_e1843_q_d_n8, eq145_e1843_q_d_n9, eq145_e1843_q_d_n10, eq145_e1843_q_d_n11, eq145_e1843_q_d_n12, eq145_e1843_q_d_n13, eq145_e1843_q_d_n14, eq145_e1843_q_d_n15, eq145_e1843_q_d_n16, eq145_e1843_q_d_n17, eq145_e1843_q_d_n18, eq145_e1843_q_d_n19, eq145_e1843_q_d_n20, eq145_e1843_q_d_n21, eq145_e1843_q_d_n22,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq145_e1840_q: f64 = s.v[252];
        let eq145_e1841: f64 = (p.p7 * s.v[252]);
        let eq145_e1841_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq145_e1841_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq145_e1841_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq145_e1841_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq145_e1841_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq145_e1841_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq145_e1841_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq145_e1841_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq145_e1841_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq145_e1841_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq145_e1841_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq145_e1841_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq145_e1841_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq145_e1841_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq145_e1841_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq145_e1841_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq145_e1841_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq145_e1841_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq145_e1841_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq145_e1841_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq145_e1841_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq145_e1841_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq145_e1841_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq145_e1841_q: f64 = (p.p7 * eq145_e1840_q);
        let eq145_e1841_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq145_e1841_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq145_e1841_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq145_e1841_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq145_e1841_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq145_e1841_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq145_e1841_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq145_e1841_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq145_e1841_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq145_e1841_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq145_e1841_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq145_e1841_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq145_e1841_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq145_e1841_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq145_e1841_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq145_e1841_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq145_e1841_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq145_e1841_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq145_e1841_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq145_e1841_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq145_e1841_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq145_e1841_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq145_e1841_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq145_e1841, eq145_e1841_d_n0, eq145_e1841_d_n1, eq145_e1841_d_n2, eq145_e1841_d_n3, eq145_e1841_d_n4, eq145_e1841_d_n5, eq145_e1841_d_n6, eq145_e1841_d_n7, eq145_e1841_d_n8, eq145_e1841_d_n9, eq145_e1841_d_n10, eq145_e1841_d_n11, eq145_e1841_d_n12, eq145_e1841_d_n13, eq145_e1841_d_n14, eq145_e1841_d_n15, eq145_e1841_d_n16, eq145_e1841_d_n17, eq145_e1841_d_n18, eq145_e1841_d_n19, eq145_e1841_d_n20, eq145_e1841_d_n21, eq145_e1841_d_n22, eq145_e1841_q, eq145_e1841_q_d_n0, eq145_e1841_q_d_n1, eq145_e1841_q_d_n2, eq145_e1841_q_d_n3, eq145_e1841_q_d_n4, eq145_e1841_q_d_n5, eq145_e1841_q_d_n6, eq145_e1841_q_d_n7, eq145_e1841_q_d_n8, eq145_e1841_q_d_n9, eq145_e1841_q_d_n10, eq145_e1841_q_d_n11, eq145_e1841_q_d_n12, eq145_e1841_q_d_n13, eq145_e1841_q_d_n14, eq145_e1841_q_d_n15, eq145_e1841_q_d_n16, eq145_e1841_q_d_n17, eq145_e1841_q_d_n18, eq145_e1841_q_d_n19, eq145_e1841_q_d_n20, eq145_e1841_q_d_n21, eq145_e1841_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_reactive_node_derivatives: [f64; 23] = [eq145_e1843_q_d_n0, eq145_e1843_q_d_n1, eq145_e1843_q_d_n2, eq145_e1843_q_d_n3, eq145_e1843_q_d_n4, eq145_e1843_q_d_n5, eq145_e1843_q_d_n6, eq145_e1843_q_d_n7, eq145_e1843_q_d_n8, eq145_e1843_q_d_n9, eq145_e1843_q_d_n10, eq145_e1843_q_d_n11, eq145_e1843_q_d_n12, eq145_e1843_q_d_n13, eq145_e1843_q_d_n14, eq145_e1843_q_d_n15, eq145_e1843_q_d_n16, eq145_e1843_q_d_n17, eq145_e1843_q_d_n18, eq145_e1843_q_d_n19, eq145_e1843_q_d_n20, eq145_e1843_q_d_n21, eq145_e1843_q_d_n22];
        let eq145_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            nodes,
            &eq145_reactive_node_derivatives,
            branches,
            &eq145_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22, eq146_e1856_q, eq146_e1856_q_d_n0, eq146_e1856_q_d_n1, eq146_e1856_q_d_n2, eq146_e1856_q_d_n3, eq146_e1856_q_d_n4, eq146_e1856_q_d_n5, eq146_e1856_q_d_n6, eq146_e1856_q_d_n7, eq146_e1856_q_d_n8, eq146_e1856_q_d_n9, eq146_e1856_q_d_n10, eq146_e1856_q_d_n11, eq146_e1856_q_d_n12, eq146_e1856_q_d_n13, eq146_e1856_q_d_n14, eq146_e1856_q_d_n15, eq146_e1856_q_d_n16, eq146_e1856_q_d_n17, eq146_e1856_q_d_n18, eq146_e1856_q_d_n19, eq146_e1856_q_d_n20, eq146_e1856_q_d_n21, eq146_e1856_q_d_n22,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853_q: f64 = s.v[252];
        let eq146_e1854: f64 = (eq146_e1851 * s.v[252]);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * s.dn[252][0]);
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * s.dn[252][1]);
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * s.dn[252][2]);
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * s.dn[252][3]);
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * s.dn[252][4]);
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * s.dn[252][5]);
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * s.dn[252][6]);
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * s.dn[252][7]);
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * s.dn[252][8]);
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * s.dn[252][9]);
        let eq146_e1854_d_n10: f64 = (eq146_e1851 * s.dn[252][10]);
        let eq146_e1854_d_n11: f64 = (eq146_e1851 * s.dn[252][11]);
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * s.dn[252][12]);
        let eq146_e1854_d_n13: f64 = (eq146_e1851 * s.dn[252][13]);
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * s.dn[252][14]);
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * s.dn[252][15]);
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * s.dn[252][16]);
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * s.dn[252][17]);
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * s.dn[252][18]);
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * s.dn[252][19]);
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * s.dn[252][20]);
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * s.dn[252][21]);
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * s.dn[252][22]);
        let eq146_e1854_q: f64 = (eq146_e1851 * eq146_e1853_q);
        let eq146_e1854_q_d_n0: f64 = (eq146_e1851 * s.dn[252][0]);
        let eq146_e1854_q_d_n1: f64 = (eq146_e1851 * s.dn[252][1]);
        let eq146_e1854_q_d_n2: f64 = (eq146_e1851 * s.dn[252][2]);
        let eq146_e1854_q_d_n3: f64 = (eq146_e1851 * s.dn[252][3]);
        let eq146_e1854_q_d_n4: f64 = (eq146_e1851 * s.dn[252][4]);
        let eq146_e1854_q_d_n5: f64 = (eq146_e1851 * s.dn[252][5]);
        let eq146_e1854_q_d_n6: f64 = (eq146_e1851 * s.dn[252][6]);
        let eq146_e1854_q_d_n7: f64 = (eq146_e1851 * s.dn[252][7]);
        let eq146_e1854_q_d_n8: f64 = (eq146_e1851 * s.dn[252][8]);
        let eq146_e1854_q_d_n9: f64 = (eq146_e1851 * s.dn[252][9]);
        let eq146_e1854_q_d_n10: f64 = (eq146_e1851 * s.dn[252][10]);
        let eq146_e1854_q_d_n11: f64 = (eq146_e1851 * s.dn[252][11]);
        let eq146_e1854_q_d_n12: f64 = (eq146_e1851 * s.dn[252][12]);
        let eq146_e1854_q_d_n13: f64 = (eq146_e1851 * s.dn[252][13]);
        let eq146_e1854_q_d_n14: f64 = (eq146_e1851 * s.dn[252][14]);
        let eq146_e1854_q_d_n15: f64 = (eq146_e1851 * s.dn[252][15]);
        let eq146_e1854_q_d_n16: f64 = (eq146_e1851 * s.dn[252][16]);
        let eq146_e1854_q_d_n17: f64 = (eq146_e1851 * s.dn[252][17]);
        let eq146_e1854_q_d_n18: f64 = (eq146_e1851 * s.dn[252][18]);
        let eq146_e1854_q_d_n19: f64 = (eq146_e1851 * s.dn[252][19]);
        let eq146_e1854_q_d_n20: f64 = (eq146_e1851 * s.dn[252][20]);
        let eq146_e1854_q_d_n21: f64 = (eq146_e1851 * s.dn[252][21]);
        let eq146_e1854_q_d_n22: f64 = (eq146_e1851 * s.dn[252][22]);
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n10, eq146_e1854_d_n11, eq146_e1854_d_n12, eq146_e1854_d_n13, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22, eq146_e1854_q, eq146_e1854_q_d_n0, eq146_e1854_q_d_n1, eq146_e1854_q_d_n2, eq146_e1854_q_d_n3, eq146_e1854_q_d_n4, eq146_e1854_q_d_n5, eq146_e1854_q_d_n6, eq146_e1854_q_d_n7, eq146_e1854_q_d_n8, eq146_e1854_q_d_n9, eq146_e1854_q_d_n10, eq146_e1854_q_d_n11, eq146_e1854_q_d_n12, eq146_e1854_q_d_n13, eq146_e1854_q_d_n14, eq146_e1854_q_d_n15, eq146_e1854_q_d_n16, eq146_e1854_q_d_n17, eq146_e1854_q_d_n18, eq146_e1854_q_d_n19, eq146_e1854_q_d_n20, eq146_e1854_q_d_n21, eq146_e1854_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_reactive_node_derivatives: [f64; 23] = [eq146_e1856_q_d_n0, eq146_e1856_q_d_n1, eq146_e1856_q_d_n2, eq146_e1856_q_d_n3, eq146_e1856_q_d_n4, eq146_e1856_q_d_n5, eq146_e1856_q_d_n6, eq146_e1856_q_d_n7, eq146_e1856_q_d_n8, eq146_e1856_q_d_n9, eq146_e1856_q_d_n10, eq146_e1856_q_d_n11, eq146_e1856_q_d_n12, eq146_e1856_q_d_n13, eq146_e1856_q_d_n14, eq146_e1856_q_d_n15, eq146_e1856_q_d_n16, eq146_e1856_q_d_n17, eq146_e1856_q_d_n18, eq146_e1856_q_d_n19, eq146_e1856_q_d_n20, eq146_e1856_q_d_n21, eq146_e1856_q_d_n22];
        let eq146_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq146_reactive_node_derivatives,
            branches,
            &eq146_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22, eq147_e1868_q, eq147_e1868_q_d_n0, eq147_e1868_q_d_n1, eq147_e1868_q_d_n2, eq147_e1868_q_d_n3, eq147_e1868_q_d_n4, eq147_e1868_q_d_n5, eq147_e1868_q_d_n6, eq147_e1868_q_d_n7, eq147_e1868_q_d_n8, eq147_e1868_q_d_n9, eq147_e1868_q_d_n10, eq147_e1868_q_d_n11, eq147_e1868_q_d_n12, eq147_e1868_q_d_n13, eq147_e1868_q_d_n14, eq147_e1868_q_d_n15, eq147_e1868_q_d_n16, eq147_e1868_q_d_n17, eq147_e1868_q_d_n18, eq147_e1868_q_d_n19, eq147_e1868_q_d_n20, eq147_e1868_q_d_n21, eq147_e1868_q_d_n22,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq147_e1865_q: f64 = s.v[252];
        let eq147_e1866: f64 = (p.p7 * s.v[252]);
        let eq147_e1866_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq147_e1866_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq147_e1866_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq147_e1866_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq147_e1866_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq147_e1866_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq147_e1866_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq147_e1866_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq147_e1866_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq147_e1866_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq147_e1866_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq147_e1866_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq147_e1866_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq147_e1866_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq147_e1866_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq147_e1866_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq147_e1866_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq147_e1866_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq147_e1866_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq147_e1866_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq147_e1866_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq147_e1866_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq147_e1866_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq147_e1866_q: f64 = (p.p7 * eq147_e1865_q);
        let eq147_e1866_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq147_e1866_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq147_e1866_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq147_e1866_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq147_e1866_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq147_e1866_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq147_e1866_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq147_e1866_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq147_e1866_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq147_e1866_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq147_e1866_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq147_e1866_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq147_e1866_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq147_e1866_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq147_e1866_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq147_e1866_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq147_e1866_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq147_e1866_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq147_e1866_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq147_e1866_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq147_e1866_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq147_e1866_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq147_e1866_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq147_e1866, eq147_e1866_d_n0, eq147_e1866_d_n1, eq147_e1866_d_n2, eq147_e1866_d_n3, eq147_e1866_d_n4, eq147_e1866_d_n5, eq147_e1866_d_n6, eq147_e1866_d_n7, eq147_e1866_d_n8, eq147_e1866_d_n9, eq147_e1866_d_n10, eq147_e1866_d_n11, eq147_e1866_d_n12, eq147_e1866_d_n13, eq147_e1866_d_n14, eq147_e1866_d_n15, eq147_e1866_d_n16, eq147_e1866_d_n17, eq147_e1866_d_n18, eq147_e1866_d_n19, eq147_e1866_d_n20, eq147_e1866_d_n21, eq147_e1866_d_n22, eq147_e1866_q, eq147_e1866_q_d_n0, eq147_e1866_q_d_n1, eq147_e1866_q_d_n2, eq147_e1866_q_d_n3, eq147_e1866_q_d_n4, eq147_e1866_q_d_n5, eq147_e1866_q_d_n6, eq147_e1866_q_d_n7, eq147_e1866_q_d_n8, eq147_e1866_q_d_n9, eq147_e1866_q_d_n10, eq147_e1866_q_d_n11, eq147_e1866_q_d_n12, eq147_e1866_q_d_n13, eq147_e1866_q_d_n14, eq147_e1866_q_d_n15, eq147_e1866_q_d_n16, eq147_e1866_q_d_n17, eq147_e1866_q_d_n18, eq147_e1866_q_d_n19, eq147_e1866_q_d_n20, eq147_e1866_q_d_n21, eq147_e1866_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_reactive_node_derivatives: [f64; 23] = [eq147_e1868_q_d_n0, eq147_e1868_q_d_n1, eq147_e1868_q_d_n2, eq147_e1868_q_d_n3, eq147_e1868_q_d_n4, eq147_e1868_q_d_n5, eq147_e1868_q_d_n6, eq147_e1868_q_d_n7, eq147_e1868_q_d_n8, eq147_e1868_q_d_n9, eq147_e1868_q_d_n10, eq147_e1868_q_d_n11, eq147_e1868_q_d_n12, eq147_e1868_q_d_n13, eq147_e1868_q_d_n14, eq147_e1868_q_d_n15, eq147_e1868_q_d_n16, eq147_e1868_q_d_n17, eq147_e1868_q_d_n18, eq147_e1868_q_d_n19, eq147_e1868_q_d_n20, eq147_e1868_q_d_n21, eq147_e1868_q_d_n22];
        let eq147_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq147_reactive_node_derivatives,
            branches,
            &eq147_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22, eq148_e1882_q, eq148_e1882_q_d_n0, eq148_e1882_q_d_n1, eq148_e1882_q_d_n2, eq148_e1882_q_d_n3, eq148_e1882_q_d_n4, eq148_e1882_q_d_n5, eq148_e1882_q_d_n6, eq148_e1882_q_d_n7, eq148_e1882_q_d_n8, eq148_e1882_q_d_n9, eq148_e1882_q_d_n10, eq148_e1882_q_d_n11, eq148_e1882_q_d_n12, eq148_e1882_q_d_n13, eq148_e1882_q_d_n14, eq148_e1882_q_d_n15, eq148_e1882_q_d_n16, eq148_e1882_q_d_n17, eq148_e1882_q_d_n18, eq148_e1882_q_d_n19, eq148_e1882_q_d_n20, eq148_e1882_q_d_n21, eq148_e1882_q_d_n22,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879_q: f64 = s.v[252];
        let eq148_e1880: f64 = (eq148_e1877 * s.v[252]);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * s.dn[252][0]);
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * s.dn[252][1]);
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * s.dn[252][2]);
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * s.dn[252][3]);
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * s.dn[252][4]);
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * s.dn[252][5]);
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * s.dn[252][6]);
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * s.dn[252][7]);
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * s.dn[252][8]);
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * s.dn[252][9]);
        let eq148_e1880_d_n10: f64 = (eq148_e1877 * s.dn[252][10]);
        let eq148_e1880_d_n11: f64 = (eq148_e1877 * s.dn[252][11]);
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * s.dn[252][12]);
        let eq148_e1880_d_n13: f64 = (eq148_e1877 * s.dn[252][13]);
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * s.dn[252][14]);
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * s.dn[252][15]);
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * s.dn[252][16]);
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * s.dn[252][17]);
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * s.dn[252][18]);
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * s.dn[252][19]);
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * s.dn[252][20]);
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * s.dn[252][21]);
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * s.dn[252][22]);
        let eq148_e1880_q: f64 = (eq148_e1877 * eq148_e1879_q);
        let eq148_e1880_q_d_n0: f64 = (eq148_e1877 * s.dn[252][0]);
        let eq148_e1880_q_d_n1: f64 = (eq148_e1877 * s.dn[252][1]);
        let eq148_e1880_q_d_n2: f64 = (eq148_e1877 * s.dn[252][2]);
        let eq148_e1880_q_d_n3: f64 = (eq148_e1877 * s.dn[252][3]);
        let eq148_e1880_q_d_n4: f64 = (eq148_e1877 * s.dn[252][4]);
        let eq148_e1880_q_d_n5: f64 = (eq148_e1877 * s.dn[252][5]);
        let eq148_e1880_q_d_n6: f64 = (eq148_e1877 * s.dn[252][6]);
        let eq148_e1880_q_d_n7: f64 = (eq148_e1877 * s.dn[252][7]);
        let eq148_e1880_q_d_n8: f64 = (eq148_e1877 * s.dn[252][8]);
        let eq148_e1880_q_d_n9: f64 = (eq148_e1877 * s.dn[252][9]);
        let eq148_e1880_q_d_n10: f64 = (eq148_e1877 * s.dn[252][10]);
        let eq148_e1880_q_d_n11: f64 = (eq148_e1877 * s.dn[252][11]);
        let eq148_e1880_q_d_n12: f64 = (eq148_e1877 * s.dn[252][12]);
        let eq148_e1880_q_d_n13: f64 = (eq148_e1877 * s.dn[252][13]);
        let eq148_e1880_q_d_n14: f64 = (eq148_e1877 * s.dn[252][14]);
        let eq148_e1880_q_d_n15: f64 = (eq148_e1877 * s.dn[252][15]);
        let eq148_e1880_q_d_n16: f64 = (eq148_e1877 * s.dn[252][16]);
        let eq148_e1880_q_d_n17: f64 = (eq148_e1877 * s.dn[252][17]);
        let eq148_e1880_q_d_n18: f64 = (eq148_e1877 * s.dn[252][18]);
        let eq148_e1880_q_d_n19: f64 = (eq148_e1877 * s.dn[252][19]);
        let eq148_e1880_q_d_n20: f64 = (eq148_e1877 * s.dn[252][20]);
        let eq148_e1880_q_d_n21: f64 = (eq148_e1877 * s.dn[252][21]);
        let eq148_e1880_q_d_n22: f64 = (eq148_e1877 * s.dn[252][22]);
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n10, eq148_e1880_d_n11, eq148_e1880_d_n12, eq148_e1880_d_n13, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22, eq148_e1880_q, eq148_e1880_q_d_n0, eq148_e1880_q_d_n1, eq148_e1880_q_d_n2, eq148_e1880_q_d_n3, eq148_e1880_q_d_n4, eq148_e1880_q_d_n5, eq148_e1880_q_d_n6, eq148_e1880_q_d_n7, eq148_e1880_q_d_n8, eq148_e1880_q_d_n9, eq148_e1880_q_d_n10, eq148_e1880_q_d_n11, eq148_e1880_q_d_n12, eq148_e1880_q_d_n13, eq148_e1880_q_d_n14, eq148_e1880_q_d_n15, eq148_e1880_q_d_n16, eq148_e1880_q_d_n17, eq148_e1880_q_d_n18, eq148_e1880_q_d_n19, eq148_e1880_q_d_n20, eq148_e1880_q_d_n21, eq148_e1880_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_reactive_node_derivatives: [f64; 23] = [eq148_e1882_q_d_n0, eq148_e1882_q_d_n1, eq148_e1882_q_d_n2, eq148_e1882_q_d_n3, eq148_e1882_q_d_n4, eq148_e1882_q_d_n5, eq148_e1882_q_d_n6, eq148_e1882_q_d_n7, eq148_e1882_q_d_n8, eq148_e1882_q_d_n9, eq148_e1882_q_d_n10, eq148_e1882_q_d_n11, eq148_e1882_q_d_n12, eq148_e1882_q_d_n13, eq148_e1882_q_d_n14, eq148_e1882_q_d_n15, eq148_e1882_q_d_n16, eq148_e1882_q_d_n17, eq148_e1882_q_d_n18, eq148_e1882_q_d_n19, eq148_e1882_q_d_n20, eq148_e1882_q_d_n21, eq148_e1882_q_d_n22];
        let eq148_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            nodes,
            &eq148_reactive_node_derivatives,
            branches,
            &eq148_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22, eq149_e1893_q, eq149_e1893_q_d_n0, eq149_e1893_q_d_n1, eq149_e1893_q_d_n2, eq149_e1893_q_d_n3, eq149_e1893_q_d_n4, eq149_e1893_q_d_n5, eq149_e1893_q_d_n6, eq149_e1893_q_d_n7, eq149_e1893_q_d_n8, eq149_e1893_q_d_n9, eq149_e1893_q_d_n10, eq149_e1893_q_d_n11, eq149_e1893_q_d_n12, eq149_e1893_q_d_n13, eq149_e1893_q_d_n14, eq149_e1893_q_d_n15, eq149_e1893_q_d_n16, eq149_e1893_q_d_n17, eq149_e1893_q_d_n18, eq149_e1893_q_d_n19, eq149_e1893_q_d_n20, eq149_e1893_q_d_n21, eq149_e1893_q_d_n22,) = {
    if (s.b[580] && s.b[581]) {
        let eq149_e1889: f64 = (p.p252 * s.v[252]);
        let eq149_e1889_d_n0: f64 = (p.p252 * s.dn[252][0]);
        let eq149_e1889_d_n1: f64 = (p.p252 * s.dn[252][1]);
        let eq149_e1889_d_n2: f64 = (p.p252 * s.dn[252][2]);
        let eq149_e1889_d_n3: f64 = (p.p252 * s.dn[252][3]);
        let eq149_e1889_d_n4: f64 = (p.p252 * s.dn[252][4]);
        let eq149_e1889_d_n5: f64 = (p.p252 * s.dn[252][5]);
        let eq149_e1889_d_n6: f64 = (p.p252 * s.dn[252][6]);
        let eq149_e1889_d_n7: f64 = (p.p252 * s.dn[252][7]);
        let eq149_e1889_d_n8: f64 = (p.p252 * s.dn[252][8]);
        let eq149_e1889_d_n9: f64 = (p.p252 * s.dn[252][9]);
        let eq149_e1889_d_n10: f64 = (p.p252 * s.dn[252][10]);
        let eq149_e1889_d_n11: f64 = (p.p252 * s.dn[252][11]);
        let eq149_e1889_d_n12: f64 = (p.p252 * s.dn[252][12]);
        let eq149_e1889_d_n13: f64 = (p.p252 * s.dn[252][13]);
        let eq149_e1889_d_n14: f64 = (p.p252 * s.dn[252][14]);
        let eq149_e1889_d_n15: f64 = (p.p252 * s.dn[252][15]);
        let eq149_e1889_d_n16: f64 = (p.p252 * s.dn[252][16]);
        let eq149_e1889_d_n17: f64 = (p.p252 * s.dn[252][17]);
        let eq149_e1889_d_n18: f64 = (p.p252 * s.dn[252][18]);
        let eq149_e1889_d_n19: f64 = (p.p252 * s.dn[252][19]);
        let eq149_e1889_d_n20: f64 = (p.p252 * s.dn[252][20]);
        let eq149_e1889_d_n21: f64 = (p.p252 * s.dn[252][21]);
        let eq149_e1889_d_n22: f64 = (p.p252 * s.dn[252][22]);
        let eq149_e1890_q: f64 = eq149_e1889;
        let eq149_e1891: f64 = (p.p7 * eq149_e1889);
        let eq149_e1891_d_n0: f64 = (p.p7 * eq149_e1889_d_n0);
        let eq149_e1891_d_n1: f64 = (p.p7 * eq149_e1889_d_n1);
        let eq149_e1891_d_n2: f64 = (p.p7 * eq149_e1889_d_n2);
        let eq149_e1891_d_n3: f64 = (p.p7 * eq149_e1889_d_n3);
        let eq149_e1891_d_n4: f64 = (p.p7 * eq149_e1889_d_n4);
        let eq149_e1891_d_n5: f64 = (p.p7 * eq149_e1889_d_n5);
        let eq149_e1891_d_n6: f64 = (p.p7 * eq149_e1889_d_n6);
        let eq149_e1891_d_n7: f64 = (p.p7 * eq149_e1889_d_n7);
        let eq149_e1891_d_n8: f64 = (p.p7 * eq149_e1889_d_n8);
        let eq149_e1891_d_n9: f64 = (p.p7 * eq149_e1889_d_n9);
        let eq149_e1891_d_n10: f64 = (p.p7 * eq149_e1889_d_n10);
        let eq149_e1891_d_n11: f64 = (p.p7 * eq149_e1889_d_n11);
        let eq149_e1891_d_n12: f64 = (p.p7 * eq149_e1889_d_n12);
        let eq149_e1891_d_n13: f64 = (p.p7 * eq149_e1889_d_n13);
        let eq149_e1891_d_n14: f64 = (p.p7 * eq149_e1889_d_n14);
        let eq149_e1891_d_n15: f64 = (p.p7 * eq149_e1889_d_n15);
        let eq149_e1891_d_n16: f64 = (p.p7 * eq149_e1889_d_n16);
        let eq149_e1891_d_n17: f64 = (p.p7 * eq149_e1889_d_n17);
        let eq149_e1891_d_n18: f64 = (p.p7 * eq149_e1889_d_n18);
        let eq149_e1891_d_n19: f64 = (p.p7 * eq149_e1889_d_n19);
        let eq149_e1891_d_n20: f64 = (p.p7 * eq149_e1889_d_n20);
        let eq149_e1891_d_n21: f64 = (p.p7 * eq149_e1889_d_n21);
        let eq149_e1891_d_n22: f64 = (p.p7 * eq149_e1889_d_n22);
        let eq149_e1891_q: f64 = (p.p7 * eq149_e1890_q);
        let eq149_e1891_q_d_n0: f64 = (p.p7 * eq149_e1889_d_n0);
        let eq149_e1891_q_d_n1: f64 = (p.p7 * eq149_e1889_d_n1);
        let eq149_e1891_q_d_n2: f64 = (p.p7 * eq149_e1889_d_n2);
        let eq149_e1891_q_d_n3: f64 = (p.p7 * eq149_e1889_d_n3);
        let eq149_e1891_q_d_n4: f64 = (p.p7 * eq149_e1889_d_n4);
        let eq149_e1891_q_d_n5: f64 = (p.p7 * eq149_e1889_d_n5);
        let eq149_e1891_q_d_n6: f64 = (p.p7 * eq149_e1889_d_n6);
        let eq149_e1891_q_d_n7: f64 = (p.p7 * eq149_e1889_d_n7);
        let eq149_e1891_q_d_n8: f64 = (p.p7 * eq149_e1889_d_n8);
        let eq149_e1891_q_d_n9: f64 = (p.p7 * eq149_e1889_d_n9);
        let eq149_e1891_q_d_n10: f64 = (p.p7 * eq149_e1889_d_n10);
        let eq149_e1891_q_d_n11: f64 = (p.p7 * eq149_e1889_d_n11);
        let eq149_e1891_q_d_n12: f64 = (p.p7 * eq149_e1889_d_n12);
        let eq149_e1891_q_d_n13: f64 = (p.p7 * eq149_e1889_d_n13);
        let eq149_e1891_q_d_n14: f64 = (p.p7 * eq149_e1889_d_n14);
        let eq149_e1891_q_d_n15: f64 = (p.p7 * eq149_e1889_d_n15);
        let eq149_e1891_q_d_n16: f64 = (p.p7 * eq149_e1889_d_n16);
        let eq149_e1891_q_d_n17: f64 = (p.p7 * eq149_e1889_d_n17);
        let eq149_e1891_q_d_n18: f64 = (p.p7 * eq149_e1889_d_n18);
        let eq149_e1891_q_d_n19: f64 = (p.p7 * eq149_e1889_d_n19);
        let eq149_e1891_q_d_n20: f64 = (p.p7 * eq149_e1889_d_n20);
        let eq149_e1891_q_d_n21: f64 = (p.p7 * eq149_e1889_d_n21);
        let eq149_e1891_q_d_n22: f64 = (p.p7 * eq149_e1889_d_n22);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n10, eq149_e1891_d_n11, eq149_e1891_d_n12, eq149_e1891_d_n13, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22, eq149_e1891_q, eq149_e1891_q_d_n0, eq149_e1891_q_d_n1, eq149_e1891_q_d_n2, eq149_e1891_q_d_n3, eq149_e1891_q_d_n4, eq149_e1891_q_d_n5, eq149_e1891_q_d_n6, eq149_e1891_q_d_n7, eq149_e1891_q_d_n8, eq149_e1891_q_d_n9, eq149_e1891_q_d_n10, eq149_e1891_q_d_n11, eq149_e1891_q_d_n12, eq149_e1891_q_d_n13, eq149_e1891_q_d_n14, eq149_e1891_q_d_n15, eq149_e1891_q_d_n16, eq149_e1891_q_d_n17, eq149_e1891_q_d_n18, eq149_e1891_q_d_n19, eq149_e1891_q_d_n20, eq149_e1891_q_d_n21, eq149_e1891_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_reactive_node_derivatives: [f64; 23] = [eq149_e1893_q_d_n0, eq149_e1893_q_d_n1, eq149_e1893_q_d_n2, eq149_e1893_q_d_n3, eq149_e1893_q_d_n4, eq149_e1893_q_d_n5, eq149_e1893_q_d_n6, eq149_e1893_q_d_n7, eq149_e1893_q_d_n8, eq149_e1893_q_d_n9, eq149_e1893_q_d_n10, eq149_e1893_q_d_n11, eq149_e1893_q_d_n12, eq149_e1893_q_d_n13, eq149_e1893_q_d_n14, eq149_e1893_q_d_n15, eq149_e1893_q_d_n16, eq149_e1893_q_d_n17, eq149_e1893_q_d_n18, eq149_e1893_q_d_n19, eq149_e1893_q_d_n20, eq149_e1893_q_d_n21, eq149_e1893_q_d_n22];
        let eq149_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            nodes,
            &eq149_reactive_node_derivatives,
            branches,
            &eq149_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22, eq150_e1903_q, eq150_e1903_q_d_n0, eq150_e1903_q_d_n1, eq150_e1903_q_d_n2, eq150_e1903_q_d_n3, eq150_e1903_q_d_n4, eq150_e1903_q_d_n5, eq150_e1903_q_d_n6, eq150_e1903_q_d_n7, eq150_e1903_q_d_n8, eq150_e1903_q_d_n9, eq150_e1903_q_d_n10, eq150_e1903_q_d_n11, eq150_e1903_q_d_n12, eq150_e1903_q_d_n13, eq150_e1903_q_d_n14, eq150_e1903_q_d_n15, eq150_e1903_q_d_n16, eq150_e1903_q_d_n17, eq150_e1903_q_d_n18, eq150_e1903_q_d_n19, eq150_e1903_q_d_n20, eq150_e1903_q_d_n21, eq150_e1903_q_d_n22,) = {
    if ((!s.b[580]) && s.b[583]) {
        let eq150_e1900_q: f64 = s.v[253];
        let eq150_e1901: f64 = (p.p7 * s.v[253]);
        let eq150_e1901_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq150_e1901_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq150_e1901_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq150_e1901_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq150_e1901_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq150_e1901_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq150_e1901_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq150_e1901_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq150_e1901_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq150_e1901_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq150_e1901_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq150_e1901_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq150_e1901_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq150_e1901_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq150_e1901_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq150_e1901_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq150_e1901_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq150_e1901_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq150_e1901_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq150_e1901_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq150_e1901_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq150_e1901_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq150_e1901_d_n22: f64 = (p.p7 * s.dn[253][22]);
        let eq150_e1901_q: f64 = (p.p7 * eq150_e1900_q);
        let eq150_e1901_q_d_n0: f64 = (p.p7 * s.dn[253][0]);
        let eq150_e1901_q_d_n1: f64 = (p.p7 * s.dn[253][1]);
        let eq150_e1901_q_d_n2: f64 = (p.p7 * s.dn[253][2]);
        let eq150_e1901_q_d_n3: f64 = (p.p7 * s.dn[253][3]);
        let eq150_e1901_q_d_n4: f64 = (p.p7 * s.dn[253][4]);
        let eq150_e1901_q_d_n5: f64 = (p.p7 * s.dn[253][5]);
        let eq150_e1901_q_d_n6: f64 = (p.p7 * s.dn[253][6]);
        let eq150_e1901_q_d_n7: f64 = (p.p7 * s.dn[253][7]);
        let eq150_e1901_q_d_n8: f64 = (p.p7 * s.dn[253][8]);
        let eq150_e1901_q_d_n9: f64 = (p.p7 * s.dn[253][9]);
        let eq150_e1901_q_d_n10: f64 = (p.p7 * s.dn[253][10]);
        let eq150_e1901_q_d_n11: f64 = (p.p7 * s.dn[253][11]);
        let eq150_e1901_q_d_n12: f64 = (p.p7 * s.dn[253][12]);
        let eq150_e1901_q_d_n13: f64 = (p.p7 * s.dn[253][13]);
        let eq150_e1901_q_d_n14: f64 = (p.p7 * s.dn[253][14]);
        let eq150_e1901_q_d_n15: f64 = (p.p7 * s.dn[253][15]);
        let eq150_e1901_q_d_n16: f64 = (p.p7 * s.dn[253][16]);
        let eq150_e1901_q_d_n17: f64 = (p.p7 * s.dn[253][17]);
        let eq150_e1901_q_d_n18: f64 = (p.p7 * s.dn[253][18]);
        let eq150_e1901_q_d_n19: f64 = (p.p7 * s.dn[253][19]);
        let eq150_e1901_q_d_n20: f64 = (p.p7 * s.dn[253][20]);
        let eq150_e1901_q_d_n21: f64 = (p.p7 * s.dn[253][21]);
        let eq150_e1901_q_d_n22: f64 = (p.p7 * s.dn[253][22]);
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n10, eq150_e1901_d_n11, eq150_e1901_d_n12, eq150_e1901_d_n13, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22, eq150_e1901_q, eq150_e1901_q_d_n0, eq150_e1901_q_d_n1, eq150_e1901_q_d_n2, eq150_e1901_q_d_n3, eq150_e1901_q_d_n4, eq150_e1901_q_d_n5, eq150_e1901_q_d_n6, eq150_e1901_q_d_n7, eq150_e1901_q_d_n8, eq150_e1901_q_d_n9, eq150_e1901_q_d_n10, eq150_e1901_q_d_n11, eq150_e1901_q_d_n12, eq150_e1901_q_d_n13, eq150_e1901_q_d_n14, eq150_e1901_q_d_n15, eq150_e1901_q_d_n16, eq150_e1901_q_d_n17, eq150_e1901_q_d_n18, eq150_e1901_q_d_n19, eq150_e1901_q_d_n20, eq150_e1901_q_d_n21, eq150_e1901_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_reactive_node_derivatives: [f64; 23] = [eq150_e1903_q_d_n0, eq150_e1903_q_d_n1, eq150_e1903_q_d_n2, eq150_e1903_q_d_n3, eq150_e1903_q_d_n4, eq150_e1903_q_d_n5, eq150_e1903_q_d_n6, eq150_e1903_q_d_n7, eq150_e1903_q_d_n8, eq150_e1903_q_d_n9, eq150_e1903_q_d_n10, eq150_e1903_q_d_n11, eq150_e1903_q_d_n12, eq150_e1903_q_d_n13, eq150_e1903_q_d_n14, eq150_e1903_q_d_n15, eq150_e1903_q_d_n16, eq150_e1903_q_d_n17, eq150_e1903_q_d_n18, eq150_e1903_q_d_n19, eq150_e1903_q_d_n20, eq150_e1903_q_d_n21, eq150_e1903_q_d_n22];
        let eq150_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq150_reactive_node_derivatives,
            branches,
            &eq150_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22, eq151_e1915_q, eq151_e1915_q_d_n0, eq151_e1915_q_d_n1, eq151_e1915_q_d_n2, eq151_e1915_q_d_n3, eq151_e1915_q_d_n4, eq151_e1915_q_d_n5, eq151_e1915_q_d_n6, eq151_e1915_q_d_n7, eq151_e1915_q_d_n8, eq151_e1915_q_d_n9, eq151_e1915_q_d_n10, eq151_e1915_q_d_n11, eq151_e1915_q_d_n12, eq151_e1915_q_d_n13, eq151_e1915_q_d_n14, eq151_e1915_q_d_n15, eq151_e1915_q_d_n16, eq151_e1915_q_d_n17, eq151_e1915_q_d_n18, eq151_e1915_q_d_n19, eq151_e1915_q_d_n20, eq151_e1915_q_d_n21, eq151_e1915_q_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && s.b[584]) {
        let eq151_e1912_q: f64 = s.v[252];
        let eq151_e1913: f64 = (p.p7 * s.v[252]);
        let eq151_e1913_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq151_e1913_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq151_e1913_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq151_e1913_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq151_e1913_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq151_e1913_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq151_e1913_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq151_e1913_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq151_e1913_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq151_e1913_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq151_e1913_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq151_e1913_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq151_e1913_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq151_e1913_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq151_e1913_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq151_e1913_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq151_e1913_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq151_e1913_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq151_e1913_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq151_e1913_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq151_e1913_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq151_e1913_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq151_e1913_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq151_e1913_q: f64 = (p.p7 * eq151_e1912_q);
        let eq151_e1913_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq151_e1913_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq151_e1913_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq151_e1913_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq151_e1913_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq151_e1913_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq151_e1913_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq151_e1913_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq151_e1913_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq151_e1913_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq151_e1913_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq151_e1913_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq151_e1913_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq151_e1913_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq151_e1913_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq151_e1913_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq151_e1913_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq151_e1913_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq151_e1913_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq151_e1913_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq151_e1913_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq151_e1913_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq151_e1913_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n10, eq151_e1913_d_n11, eq151_e1913_d_n12, eq151_e1913_d_n13, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22, eq151_e1913_q, eq151_e1913_q_d_n0, eq151_e1913_q_d_n1, eq151_e1913_q_d_n2, eq151_e1913_q_d_n3, eq151_e1913_q_d_n4, eq151_e1913_q_d_n5, eq151_e1913_q_d_n6, eq151_e1913_q_d_n7, eq151_e1913_q_d_n8, eq151_e1913_q_d_n9, eq151_e1913_q_d_n10, eq151_e1913_q_d_n11, eq151_e1913_q_d_n12, eq151_e1913_q_d_n13, eq151_e1913_q_d_n14, eq151_e1913_q_d_n15, eq151_e1913_q_d_n16, eq151_e1913_q_d_n17, eq151_e1913_q_d_n18, eq151_e1913_q_d_n19, eq151_e1913_q_d_n20, eq151_e1913_q_d_n21, eq151_e1913_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_reactive_node_derivatives: [f64; 23] = [eq151_e1915_q_d_n0, eq151_e1915_q_d_n1, eq151_e1915_q_d_n2, eq151_e1915_q_d_n3, eq151_e1915_q_d_n4, eq151_e1915_q_d_n5, eq151_e1915_q_d_n6, eq151_e1915_q_d_n7, eq151_e1915_q_d_n8, eq151_e1915_q_d_n9, eq151_e1915_q_d_n10, eq151_e1915_q_d_n11, eq151_e1915_q_d_n12, eq151_e1915_q_d_n13, eq151_e1915_q_d_n14, eq151_e1915_q_d_n15, eq151_e1915_q_d_n16, eq151_e1915_q_d_n17, eq151_e1915_q_d_n18, eq151_e1915_q_d_n19, eq151_e1915_q_d_n20, eq151_e1915_q_d_n21, eq151_e1915_q_d_n22];
        let eq151_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq151_reactive_node_derivatives,
            branches,
            &eq151_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_8(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22, eq152_e1929_q, eq152_e1929_q_d_n0, eq152_e1929_q_d_n1, eq152_e1929_q_d_n2, eq152_e1929_q_d_n3, eq152_e1929_q_d_n4, eq152_e1929_q_d_n5, eq152_e1929_q_d_n6, eq152_e1929_q_d_n7, eq152_e1929_q_d_n8, eq152_e1929_q_d_n9, eq152_e1929_q_d_n10, eq152_e1929_q_d_n11, eq152_e1929_q_d_n12, eq152_e1929_q_d_n13, eq152_e1929_q_d_n14, eq152_e1929_q_d_n15, eq152_e1929_q_d_n16, eq152_e1929_q_d_n17, eq152_e1929_q_d_n18, eq152_e1929_q_d_n19, eq152_e1929_q_d_n20, eq152_e1929_q_d_n21, eq152_e1929_q_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && s.b[584]) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926_q: f64 = s.v[252];
        let eq152_e1927: f64 = (eq152_e1924 * s.v[252]);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * s.dn[252][0]);
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * s.dn[252][1]);
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * s.dn[252][2]);
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * s.dn[252][3]);
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * s.dn[252][4]);
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * s.dn[252][5]);
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * s.dn[252][6]);
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * s.dn[252][7]);
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * s.dn[252][8]);
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * s.dn[252][9]);
        let eq152_e1927_d_n10: f64 = (eq152_e1924 * s.dn[252][10]);
        let eq152_e1927_d_n11: f64 = (eq152_e1924 * s.dn[252][11]);
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * s.dn[252][12]);
        let eq152_e1927_d_n13: f64 = (eq152_e1924 * s.dn[252][13]);
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * s.dn[252][14]);
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * s.dn[252][15]);
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * s.dn[252][16]);
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * s.dn[252][17]);
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * s.dn[252][18]);
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * s.dn[252][19]);
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * s.dn[252][20]);
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * s.dn[252][21]);
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * s.dn[252][22]);
        let eq152_e1927_q: f64 = (eq152_e1924 * eq152_e1926_q);
        let eq152_e1927_q_d_n0: f64 = (eq152_e1924 * s.dn[252][0]);
        let eq152_e1927_q_d_n1: f64 = (eq152_e1924 * s.dn[252][1]);
        let eq152_e1927_q_d_n2: f64 = (eq152_e1924 * s.dn[252][2]);
        let eq152_e1927_q_d_n3: f64 = (eq152_e1924 * s.dn[252][3]);
        let eq152_e1927_q_d_n4: f64 = (eq152_e1924 * s.dn[252][4]);
        let eq152_e1927_q_d_n5: f64 = (eq152_e1924 * s.dn[252][5]);
        let eq152_e1927_q_d_n6: f64 = (eq152_e1924 * s.dn[252][6]);
        let eq152_e1927_q_d_n7: f64 = (eq152_e1924 * s.dn[252][7]);
        let eq152_e1927_q_d_n8: f64 = (eq152_e1924 * s.dn[252][8]);
        let eq152_e1927_q_d_n9: f64 = (eq152_e1924 * s.dn[252][9]);
        let eq152_e1927_q_d_n10: f64 = (eq152_e1924 * s.dn[252][10]);
        let eq152_e1927_q_d_n11: f64 = (eq152_e1924 * s.dn[252][11]);
        let eq152_e1927_q_d_n12: f64 = (eq152_e1924 * s.dn[252][12]);
        let eq152_e1927_q_d_n13: f64 = (eq152_e1924 * s.dn[252][13]);
        let eq152_e1927_q_d_n14: f64 = (eq152_e1924 * s.dn[252][14]);
        let eq152_e1927_q_d_n15: f64 = (eq152_e1924 * s.dn[252][15]);
        let eq152_e1927_q_d_n16: f64 = (eq152_e1924 * s.dn[252][16]);
        let eq152_e1927_q_d_n17: f64 = (eq152_e1924 * s.dn[252][17]);
        let eq152_e1927_q_d_n18: f64 = (eq152_e1924 * s.dn[252][18]);
        let eq152_e1927_q_d_n19: f64 = (eq152_e1924 * s.dn[252][19]);
        let eq152_e1927_q_d_n20: f64 = (eq152_e1924 * s.dn[252][20]);
        let eq152_e1927_q_d_n21: f64 = (eq152_e1924 * s.dn[252][21]);
        let eq152_e1927_q_d_n22: f64 = (eq152_e1924 * s.dn[252][22]);
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n10, eq152_e1927_d_n11, eq152_e1927_d_n12, eq152_e1927_d_n13, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22, eq152_e1927_q, eq152_e1927_q_d_n0, eq152_e1927_q_d_n1, eq152_e1927_q_d_n2, eq152_e1927_q_d_n3, eq152_e1927_q_d_n4, eq152_e1927_q_d_n5, eq152_e1927_q_d_n6, eq152_e1927_q_d_n7, eq152_e1927_q_d_n8, eq152_e1927_q_d_n9, eq152_e1927_q_d_n10, eq152_e1927_q_d_n11, eq152_e1927_q_d_n12, eq152_e1927_q_d_n13, eq152_e1927_q_d_n14, eq152_e1927_q_d_n15, eq152_e1927_q_d_n16, eq152_e1927_q_d_n17, eq152_e1927_q_d_n18, eq152_e1927_q_d_n19, eq152_e1927_q_d_n20, eq152_e1927_q_d_n21, eq152_e1927_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_reactive_node_derivatives: [f64; 23] = [eq152_e1929_q_d_n0, eq152_e1929_q_d_n1, eq152_e1929_q_d_n2, eq152_e1929_q_d_n3, eq152_e1929_q_d_n4, eq152_e1929_q_d_n5, eq152_e1929_q_d_n6, eq152_e1929_q_d_n7, eq152_e1929_q_d_n8, eq152_e1929_q_d_n9, eq152_e1929_q_d_n10, eq152_e1929_q_d_n11, eq152_e1929_q_d_n12, eq152_e1929_q_d_n13, eq152_e1929_q_d_n14, eq152_e1929_q_d_n15, eq152_e1929_q_d_n16, eq152_e1929_q_d_n17, eq152_e1929_q_d_n18, eq152_e1929_q_d_n19, eq152_e1929_q_d_n20, eq152_e1929_q_d_n21, eq152_e1929_q_d_n22];
        let eq152_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq152_reactive_node_derivatives,
            branches,
            &eq152_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22, eq153_e1942_q, eq153_e1942_q_d_n0, eq153_e1942_q_d_n1, eq153_e1942_q_d_n2, eq153_e1942_q_d_n3, eq153_e1942_q_d_n4, eq153_e1942_q_d_n5, eq153_e1942_q_d_n6, eq153_e1942_q_d_n7, eq153_e1942_q_d_n8, eq153_e1942_q_d_n9, eq153_e1942_q_d_n10, eq153_e1942_q_d_n11, eq153_e1942_q_d_n12, eq153_e1942_q_d_n13, eq153_e1942_q_d_n14, eq153_e1942_q_d_n15, eq153_e1942_q_d_n16, eq153_e1942_q_d_n17, eq153_e1942_q_d_n18, eq153_e1942_q_d_n19, eq153_e1942_q_d_n20, eq153_e1942_q_d_n21, eq153_e1942_q_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && (!s.b[584])) {
        let eq153_e1939_q: f64 = s.v[252];
        let eq153_e1940: f64 = (p.p7 * s.v[252]);
        let eq153_e1940_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq153_e1940_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq153_e1940_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq153_e1940_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq153_e1940_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq153_e1940_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq153_e1940_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq153_e1940_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq153_e1940_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq153_e1940_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq153_e1940_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq153_e1940_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq153_e1940_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq153_e1940_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq153_e1940_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq153_e1940_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq153_e1940_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq153_e1940_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq153_e1940_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq153_e1940_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq153_e1940_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq153_e1940_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq153_e1940_d_n22: f64 = (p.p7 * s.dn[252][22]);
        let eq153_e1940_q: f64 = (p.p7 * eq153_e1939_q);
        let eq153_e1940_q_d_n0: f64 = (p.p7 * s.dn[252][0]);
        let eq153_e1940_q_d_n1: f64 = (p.p7 * s.dn[252][1]);
        let eq153_e1940_q_d_n2: f64 = (p.p7 * s.dn[252][2]);
        let eq153_e1940_q_d_n3: f64 = (p.p7 * s.dn[252][3]);
        let eq153_e1940_q_d_n4: f64 = (p.p7 * s.dn[252][4]);
        let eq153_e1940_q_d_n5: f64 = (p.p7 * s.dn[252][5]);
        let eq153_e1940_q_d_n6: f64 = (p.p7 * s.dn[252][6]);
        let eq153_e1940_q_d_n7: f64 = (p.p7 * s.dn[252][7]);
        let eq153_e1940_q_d_n8: f64 = (p.p7 * s.dn[252][8]);
        let eq153_e1940_q_d_n9: f64 = (p.p7 * s.dn[252][9]);
        let eq153_e1940_q_d_n10: f64 = (p.p7 * s.dn[252][10]);
        let eq153_e1940_q_d_n11: f64 = (p.p7 * s.dn[252][11]);
        let eq153_e1940_q_d_n12: f64 = (p.p7 * s.dn[252][12]);
        let eq153_e1940_q_d_n13: f64 = (p.p7 * s.dn[252][13]);
        let eq153_e1940_q_d_n14: f64 = (p.p7 * s.dn[252][14]);
        let eq153_e1940_q_d_n15: f64 = (p.p7 * s.dn[252][15]);
        let eq153_e1940_q_d_n16: f64 = (p.p7 * s.dn[252][16]);
        let eq153_e1940_q_d_n17: f64 = (p.p7 * s.dn[252][17]);
        let eq153_e1940_q_d_n18: f64 = (p.p7 * s.dn[252][18]);
        let eq153_e1940_q_d_n19: f64 = (p.p7 * s.dn[252][19]);
        let eq153_e1940_q_d_n20: f64 = (p.p7 * s.dn[252][20]);
        let eq153_e1940_q_d_n21: f64 = (p.p7 * s.dn[252][21]);
        let eq153_e1940_q_d_n22: f64 = (p.p7 * s.dn[252][22]);
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n10, eq153_e1940_d_n11, eq153_e1940_d_n12, eq153_e1940_d_n13, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22, eq153_e1940_q, eq153_e1940_q_d_n0, eq153_e1940_q_d_n1, eq153_e1940_q_d_n2, eq153_e1940_q_d_n3, eq153_e1940_q_d_n4, eq153_e1940_q_d_n5, eq153_e1940_q_d_n6, eq153_e1940_q_d_n7, eq153_e1940_q_d_n8, eq153_e1940_q_d_n9, eq153_e1940_q_d_n10, eq153_e1940_q_d_n11, eq153_e1940_q_d_n12, eq153_e1940_q_d_n13, eq153_e1940_q_d_n14, eq153_e1940_q_d_n15, eq153_e1940_q_d_n16, eq153_e1940_q_d_n17, eq153_e1940_q_d_n18, eq153_e1940_q_d_n19, eq153_e1940_q_d_n20, eq153_e1940_q_d_n21, eq153_e1940_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_reactive_node_derivatives: [f64; 23] = [eq153_e1942_q_d_n0, eq153_e1942_q_d_n1, eq153_e1942_q_d_n2, eq153_e1942_q_d_n3, eq153_e1942_q_d_n4, eq153_e1942_q_d_n5, eq153_e1942_q_d_n6, eq153_e1942_q_d_n7, eq153_e1942_q_d_n8, eq153_e1942_q_d_n9, eq153_e1942_q_d_n10, eq153_e1942_q_d_n11, eq153_e1942_q_d_n12, eq153_e1942_q_d_n13, eq153_e1942_q_d_n14, eq153_e1942_q_d_n15, eq153_e1942_q_d_n16, eq153_e1942_q_d_n17, eq153_e1942_q_d_n18, eq153_e1942_q_d_n19, eq153_e1942_q_d_n20, eq153_e1942_q_d_n21, eq153_e1942_q_d_n22];
        let eq153_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq153_reactive_node_derivatives,
            branches,
            &eq153_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22, eq154_e1957_q, eq154_e1957_q_d_n0, eq154_e1957_q_d_n1, eq154_e1957_q_d_n2, eq154_e1957_q_d_n3, eq154_e1957_q_d_n4, eq154_e1957_q_d_n5, eq154_e1957_q_d_n6, eq154_e1957_q_d_n7, eq154_e1957_q_d_n8, eq154_e1957_q_d_n9, eq154_e1957_q_d_n10, eq154_e1957_q_d_n11, eq154_e1957_q_d_n12, eq154_e1957_q_d_n13, eq154_e1957_q_d_n14, eq154_e1957_q_d_n15, eq154_e1957_q_d_n16, eq154_e1957_q_d_n17, eq154_e1957_q_d_n18, eq154_e1957_q_d_n19, eq154_e1957_q_d_n20, eq154_e1957_q_d_n21, eq154_e1957_q_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && (!s.b[584])) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954_q: f64 = s.v[252];
        let eq154_e1955: f64 = (eq154_e1952 * s.v[252]);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * s.dn[252][0]);
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * s.dn[252][1]);
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * s.dn[252][2]);
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * s.dn[252][3]);
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * s.dn[252][4]);
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * s.dn[252][5]);
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * s.dn[252][6]);
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * s.dn[252][7]);
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * s.dn[252][8]);
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * s.dn[252][9]);
        let eq154_e1955_d_n10: f64 = (eq154_e1952 * s.dn[252][10]);
        let eq154_e1955_d_n11: f64 = (eq154_e1952 * s.dn[252][11]);
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * s.dn[252][12]);
        let eq154_e1955_d_n13: f64 = (eq154_e1952 * s.dn[252][13]);
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * s.dn[252][14]);
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * s.dn[252][15]);
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * s.dn[252][16]);
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * s.dn[252][17]);
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * s.dn[252][18]);
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * s.dn[252][19]);
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * s.dn[252][20]);
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * s.dn[252][21]);
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * s.dn[252][22]);
        let eq154_e1955_q: f64 = (eq154_e1952 * eq154_e1954_q);
        let eq154_e1955_q_d_n0: f64 = (eq154_e1952 * s.dn[252][0]);
        let eq154_e1955_q_d_n1: f64 = (eq154_e1952 * s.dn[252][1]);
        let eq154_e1955_q_d_n2: f64 = (eq154_e1952 * s.dn[252][2]);
        let eq154_e1955_q_d_n3: f64 = (eq154_e1952 * s.dn[252][3]);
        let eq154_e1955_q_d_n4: f64 = (eq154_e1952 * s.dn[252][4]);
        let eq154_e1955_q_d_n5: f64 = (eq154_e1952 * s.dn[252][5]);
        let eq154_e1955_q_d_n6: f64 = (eq154_e1952 * s.dn[252][6]);
        let eq154_e1955_q_d_n7: f64 = (eq154_e1952 * s.dn[252][7]);
        let eq154_e1955_q_d_n8: f64 = (eq154_e1952 * s.dn[252][8]);
        let eq154_e1955_q_d_n9: f64 = (eq154_e1952 * s.dn[252][9]);
        let eq154_e1955_q_d_n10: f64 = (eq154_e1952 * s.dn[252][10]);
        let eq154_e1955_q_d_n11: f64 = (eq154_e1952 * s.dn[252][11]);
        let eq154_e1955_q_d_n12: f64 = (eq154_e1952 * s.dn[252][12]);
        let eq154_e1955_q_d_n13: f64 = (eq154_e1952 * s.dn[252][13]);
        let eq154_e1955_q_d_n14: f64 = (eq154_e1952 * s.dn[252][14]);
        let eq154_e1955_q_d_n15: f64 = (eq154_e1952 * s.dn[252][15]);
        let eq154_e1955_q_d_n16: f64 = (eq154_e1952 * s.dn[252][16]);
        let eq154_e1955_q_d_n17: f64 = (eq154_e1952 * s.dn[252][17]);
        let eq154_e1955_q_d_n18: f64 = (eq154_e1952 * s.dn[252][18]);
        let eq154_e1955_q_d_n19: f64 = (eq154_e1952 * s.dn[252][19]);
        let eq154_e1955_q_d_n20: f64 = (eq154_e1952 * s.dn[252][20]);
        let eq154_e1955_q_d_n21: f64 = (eq154_e1952 * s.dn[252][21]);
        let eq154_e1955_q_d_n22: f64 = (eq154_e1952 * s.dn[252][22]);
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n10, eq154_e1955_d_n11, eq154_e1955_d_n12, eq154_e1955_d_n13, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22, eq154_e1955_q, eq154_e1955_q_d_n0, eq154_e1955_q_d_n1, eq154_e1955_q_d_n2, eq154_e1955_q_d_n3, eq154_e1955_q_d_n4, eq154_e1955_q_d_n5, eq154_e1955_q_d_n6, eq154_e1955_q_d_n7, eq154_e1955_q_d_n8, eq154_e1955_q_d_n9, eq154_e1955_q_d_n10, eq154_e1955_q_d_n11, eq154_e1955_q_d_n12, eq154_e1955_q_d_n13, eq154_e1955_q_d_n14, eq154_e1955_q_d_n15, eq154_e1955_q_d_n16, eq154_e1955_q_d_n17, eq154_e1955_q_d_n18, eq154_e1955_q_d_n19, eq154_e1955_q_d_n20, eq154_e1955_q_d_n21, eq154_e1955_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_reactive_node_derivatives: [f64; 23] = [eq154_e1957_q_d_n0, eq154_e1957_q_d_n1, eq154_e1957_q_d_n2, eq154_e1957_q_d_n3, eq154_e1957_q_d_n4, eq154_e1957_q_d_n5, eq154_e1957_q_d_n6, eq154_e1957_q_d_n7, eq154_e1957_q_d_n8, eq154_e1957_q_d_n9, eq154_e1957_q_d_n10, eq154_e1957_q_d_n11, eq154_e1957_q_d_n12, eq154_e1957_q_d_n13, eq154_e1957_q_d_n14, eq154_e1957_q_d_n15, eq154_e1957_q_d_n16, eq154_e1957_q_d_n17, eq154_e1957_q_d_n18, eq154_e1957_q_d_n19, eq154_e1957_q_d_n20, eq154_e1957_q_d_n21, eq154_e1957_q_d_n22];
        let eq154_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq154_reactive_node_derivatives,
            branches,
            &eq154_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22, eq155_e1969_q, eq155_e1969_q_d_n0, eq155_e1969_q_d_n1, eq155_e1969_q_d_n2, eq155_e1969_q_d_n3, eq155_e1969_q_d_n4, eq155_e1969_q_d_n5, eq155_e1969_q_d_n6, eq155_e1969_q_d_n7, eq155_e1969_q_d_n8, eq155_e1969_q_d_n9, eq155_e1969_q_d_n10, eq155_e1969_q_d_n11, eq155_e1969_q_d_n12, eq155_e1969_q_d_n13, eq155_e1969_q_d_n14, eq155_e1969_q_d_n15, eq155_e1969_q_d_n16, eq155_e1969_q_d_n17, eq155_e1969_q_d_n18, eq155_e1969_q_d_n19, eq155_e1969_q_d_n20, eq155_e1969_q_d_n21, eq155_e1969_q_d_n22,) = {
    if ((!s.b[580]) && s.b[583]) {
        let eq155_e1965: f64 = (p.p252 * s.v[252]);
        let eq155_e1965_d_n0: f64 = (p.p252 * s.dn[252][0]);
        let eq155_e1965_d_n1: f64 = (p.p252 * s.dn[252][1]);
        let eq155_e1965_d_n2: f64 = (p.p252 * s.dn[252][2]);
        let eq155_e1965_d_n3: f64 = (p.p252 * s.dn[252][3]);
        let eq155_e1965_d_n4: f64 = (p.p252 * s.dn[252][4]);
        let eq155_e1965_d_n5: f64 = (p.p252 * s.dn[252][5]);
        let eq155_e1965_d_n6: f64 = (p.p252 * s.dn[252][6]);
        let eq155_e1965_d_n7: f64 = (p.p252 * s.dn[252][7]);
        let eq155_e1965_d_n8: f64 = (p.p252 * s.dn[252][8]);
        let eq155_e1965_d_n9: f64 = (p.p252 * s.dn[252][9]);
        let eq155_e1965_d_n10: f64 = (p.p252 * s.dn[252][10]);
        let eq155_e1965_d_n11: f64 = (p.p252 * s.dn[252][11]);
        let eq155_e1965_d_n12: f64 = (p.p252 * s.dn[252][12]);
        let eq155_e1965_d_n13: f64 = (p.p252 * s.dn[252][13]);
        let eq155_e1965_d_n14: f64 = (p.p252 * s.dn[252][14]);
        let eq155_e1965_d_n15: f64 = (p.p252 * s.dn[252][15]);
        let eq155_e1965_d_n16: f64 = (p.p252 * s.dn[252][16]);
        let eq155_e1965_d_n17: f64 = (p.p252 * s.dn[252][17]);
        let eq155_e1965_d_n18: f64 = (p.p252 * s.dn[252][18]);
        let eq155_e1965_d_n19: f64 = (p.p252 * s.dn[252][19]);
        let eq155_e1965_d_n20: f64 = (p.p252 * s.dn[252][20]);
        let eq155_e1965_d_n21: f64 = (p.p252 * s.dn[252][21]);
        let eq155_e1965_d_n22: f64 = (p.p252 * s.dn[252][22]);
        let eq155_e1966_q: f64 = eq155_e1965;
        let eq155_e1967: f64 = (p.p7 * eq155_e1965);
        let eq155_e1967_d_n0: f64 = (p.p7 * eq155_e1965_d_n0);
        let eq155_e1967_d_n1: f64 = (p.p7 * eq155_e1965_d_n1);
        let eq155_e1967_d_n2: f64 = (p.p7 * eq155_e1965_d_n2);
        let eq155_e1967_d_n3: f64 = (p.p7 * eq155_e1965_d_n3);
        let eq155_e1967_d_n4: f64 = (p.p7 * eq155_e1965_d_n4);
        let eq155_e1967_d_n5: f64 = (p.p7 * eq155_e1965_d_n5);
        let eq155_e1967_d_n6: f64 = (p.p7 * eq155_e1965_d_n6);
        let eq155_e1967_d_n7: f64 = (p.p7 * eq155_e1965_d_n7);
        let eq155_e1967_d_n8: f64 = (p.p7 * eq155_e1965_d_n8);
        let eq155_e1967_d_n9: f64 = (p.p7 * eq155_e1965_d_n9);
        let eq155_e1967_d_n10: f64 = (p.p7 * eq155_e1965_d_n10);
        let eq155_e1967_d_n11: f64 = (p.p7 * eq155_e1965_d_n11);
        let eq155_e1967_d_n12: f64 = (p.p7 * eq155_e1965_d_n12);
        let eq155_e1967_d_n13: f64 = (p.p7 * eq155_e1965_d_n13);
        let eq155_e1967_d_n14: f64 = (p.p7 * eq155_e1965_d_n14);
        let eq155_e1967_d_n15: f64 = (p.p7 * eq155_e1965_d_n15);
        let eq155_e1967_d_n16: f64 = (p.p7 * eq155_e1965_d_n16);
        let eq155_e1967_d_n17: f64 = (p.p7 * eq155_e1965_d_n17);
        let eq155_e1967_d_n18: f64 = (p.p7 * eq155_e1965_d_n18);
        let eq155_e1967_d_n19: f64 = (p.p7 * eq155_e1965_d_n19);
        let eq155_e1967_d_n20: f64 = (p.p7 * eq155_e1965_d_n20);
        let eq155_e1967_d_n21: f64 = (p.p7 * eq155_e1965_d_n21);
        let eq155_e1967_d_n22: f64 = (p.p7 * eq155_e1965_d_n22);
        let eq155_e1967_q: f64 = (p.p7 * eq155_e1966_q);
        let eq155_e1967_q_d_n0: f64 = (p.p7 * eq155_e1965_d_n0);
        let eq155_e1967_q_d_n1: f64 = (p.p7 * eq155_e1965_d_n1);
        let eq155_e1967_q_d_n2: f64 = (p.p7 * eq155_e1965_d_n2);
        let eq155_e1967_q_d_n3: f64 = (p.p7 * eq155_e1965_d_n3);
        let eq155_e1967_q_d_n4: f64 = (p.p7 * eq155_e1965_d_n4);
        let eq155_e1967_q_d_n5: f64 = (p.p7 * eq155_e1965_d_n5);
        let eq155_e1967_q_d_n6: f64 = (p.p7 * eq155_e1965_d_n6);
        let eq155_e1967_q_d_n7: f64 = (p.p7 * eq155_e1965_d_n7);
        let eq155_e1967_q_d_n8: f64 = (p.p7 * eq155_e1965_d_n8);
        let eq155_e1967_q_d_n9: f64 = (p.p7 * eq155_e1965_d_n9);
        let eq155_e1967_q_d_n10: f64 = (p.p7 * eq155_e1965_d_n10);
        let eq155_e1967_q_d_n11: f64 = (p.p7 * eq155_e1965_d_n11);
        let eq155_e1967_q_d_n12: f64 = (p.p7 * eq155_e1965_d_n12);
        let eq155_e1967_q_d_n13: f64 = (p.p7 * eq155_e1965_d_n13);
        let eq155_e1967_q_d_n14: f64 = (p.p7 * eq155_e1965_d_n14);
        let eq155_e1967_q_d_n15: f64 = (p.p7 * eq155_e1965_d_n15);
        let eq155_e1967_q_d_n16: f64 = (p.p7 * eq155_e1965_d_n16);
        let eq155_e1967_q_d_n17: f64 = (p.p7 * eq155_e1965_d_n17);
        let eq155_e1967_q_d_n18: f64 = (p.p7 * eq155_e1965_d_n18);
        let eq155_e1967_q_d_n19: f64 = (p.p7 * eq155_e1965_d_n19);
        let eq155_e1967_q_d_n20: f64 = (p.p7 * eq155_e1965_d_n20);
        let eq155_e1967_q_d_n21: f64 = (p.p7 * eq155_e1965_d_n21);
        let eq155_e1967_q_d_n22: f64 = (p.p7 * eq155_e1965_d_n22);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n10, eq155_e1967_d_n11, eq155_e1967_d_n12, eq155_e1967_d_n13, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22, eq155_e1967_q, eq155_e1967_q_d_n0, eq155_e1967_q_d_n1, eq155_e1967_q_d_n2, eq155_e1967_q_d_n3, eq155_e1967_q_d_n4, eq155_e1967_q_d_n5, eq155_e1967_q_d_n6, eq155_e1967_q_d_n7, eq155_e1967_q_d_n8, eq155_e1967_q_d_n9, eq155_e1967_q_d_n10, eq155_e1967_q_d_n11, eq155_e1967_q_d_n12, eq155_e1967_q_d_n13, eq155_e1967_q_d_n14, eq155_e1967_q_d_n15, eq155_e1967_q_d_n16, eq155_e1967_q_d_n17, eq155_e1967_q_d_n18, eq155_e1967_q_d_n19, eq155_e1967_q_d_n20, eq155_e1967_q_d_n21, eq155_e1967_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_reactive_node_derivatives: [f64; 23] = [eq155_e1969_q_d_n0, eq155_e1969_q_d_n1, eq155_e1969_q_d_n2, eq155_e1969_q_d_n3, eq155_e1969_q_d_n4, eq155_e1969_q_d_n5, eq155_e1969_q_d_n6, eq155_e1969_q_d_n7, eq155_e1969_q_d_n8, eq155_e1969_q_d_n9, eq155_e1969_q_d_n10, eq155_e1969_q_d_n11, eq155_e1969_q_d_n12, eq155_e1969_q_d_n13, eq155_e1969_q_d_n14, eq155_e1969_q_d_n15, eq155_e1969_q_d_n16, eq155_e1969_q_d_n17, eq155_e1969_q_d_n18, eq155_e1969_q_d_n19, eq155_e1969_q_d_n20, eq155_e1969_q_d_n21, eq155_e1969_q_d_n22];
        let eq155_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq155_reactive_node_derivatives,
            branches,
            &eq155_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22, eq156_e1978_q, eq156_e1978_q_d_n0, eq156_e1978_q_d_n1, eq156_e1978_q_d_n2, eq156_e1978_q_d_n3, eq156_e1978_q_d_n4, eq156_e1978_q_d_n5, eq156_e1978_q_d_n6, eq156_e1978_q_d_n7, eq156_e1978_q_d_n8, eq156_e1978_q_d_n9, eq156_e1978_q_d_n10, eq156_e1978_q_d_n11, eq156_e1978_q_d_n12, eq156_e1978_q_d_n13, eq156_e1978_q_d_n14, eq156_e1978_q_d_n15, eq156_e1978_q_d_n16, eq156_e1978_q_d_n17, eq156_e1978_q_d_n18, eq156_e1978_q_d_n19, eq156_e1978_q_d_n20, eq156_e1978_q_d_n21, eq156_e1978_q_d_n22,) = {
    if (s.b[585] && s.b[586]) {
        let eq156_e1975_q: f64 = s.v[265];
        let eq156_e1976: f64 = (p.p7 * s.v[265]);
        let eq156_e1976_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq156_e1976_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq156_e1976_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq156_e1976_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq156_e1976_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq156_e1976_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq156_e1976_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq156_e1976_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq156_e1976_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq156_e1976_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq156_e1976_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq156_e1976_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq156_e1976_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq156_e1976_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq156_e1976_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq156_e1976_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq156_e1976_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq156_e1976_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq156_e1976_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq156_e1976_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq156_e1976_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq156_e1976_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq156_e1976_d_n22: f64 = (p.p7 * s.dn[265][22]);
        let eq156_e1976_q: f64 = (p.p7 * eq156_e1975_q);
        let eq156_e1976_q_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq156_e1976_q_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq156_e1976_q_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq156_e1976_q_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq156_e1976_q_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq156_e1976_q_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq156_e1976_q_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq156_e1976_q_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq156_e1976_q_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq156_e1976_q_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq156_e1976_q_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq156_e1976_q_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq156_e1976_q_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq156_e1976_q_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq156_e1976_q_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq156_e1976_q_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq156_e1976_q_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq156_e1976_q_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq156_e1976_q_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq156_e1976_q_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq156_e1976_q_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq156_e1976_q_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq156_e1976_q_d_n22: f64 = (p.p7 * s.dn[265][22]);
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n10, eq156_e1976_d_n11, eq156_e1976_d_n12, eq156_e1976_d_n13, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22, eq156_e1976_q, eq156_e1976_q_d_n0, eq156_e1976_q_d_n1, eq156_e1976_q_d_n2, eq156_e1976_q_d_n3, eq156_e1976_q_d_n4, eq156_e1976_q_d_n5, eq156_e1976_q_d_n6, eq156_e1976_q_d_n7, eq156_e1976_q_d_n8, eq156_e1976_q_d_n9, eq156_e1976_q_d_n10, eq156_e1976_q_d_n11, eq156_e1976_q_d_n12, eq156_e1976_q_d_n13, eq156_e1976_q_d_n14, eq156_e1976_q_d_n15, eq156_e1976_q_d_n16, eq156_e1976_q_d_n17, eq156_e1976_q_d_n18, eq156_e1976_q_d_n19, eq156_e1976_q_d_n20, eq156_e1976_q_d_n21, eq156_e1976_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_reactive_node_derivatives: [f64; 23] = [eq156_e1978_q_d_n0, eq156_e1978_q_d_n1, eq156_e1978_q_d_n2, eq156_e1978_q_d_n3, eq156_e1978_q_d_n4, eq156_e1978_q_d_n5, eq156_e1978_q_d_n6, eq156_e1978_q_d_n7, eq156_e1978_q_d_n8, eq156_e1978_q_d_n9, eq156_e1978_q_d_n10, eq156_e1978_q_d_n11, eq156_e1978_q_d_n12, eq156_e1978_q_d_n13, eq156_e1978_q_d_n14, eq156_e1978_q_d_n15, eq156_e1978_q_d_n16, eq156_e1978_q_d_n17, eq156_e1978_q_d_n18, eq156_e1978_q_d_n19, eq156_e1978_q_d_n20, eq156_e1978_q_d_n21, eq156_e1978_q_d_n22];
        let eq156_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[20]),
            nodes,
            &eq156_reactive_node_derivatives,
            branches,
            &eq156_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22, eq157_e1989_q, eq157_e1989_q_d_n0, eq157_e1989_q_d_n1, eq157_e1989_q_d_n2, eq157_e1989_q_d_n3, eq157_e1989_q_d_n4, eq157_e1989_q_d_n5, eq157_e1989_q_d_n6, eq157_e1989_q_d_n7, eq157_e1989_q_d_n8, eq157_e1989_q_d_n9, eq157_e1989_q_d_n10, eq157_e1989_q_d_n11, eq157_e1989_q_d_n12, eq157_e1989_q_d_n13, eq157_e1989_q_d_n14, eq157_e1989_q_d_n15, eq157_e1989_q_d_n16, eq157_e1989_q_d_n17, eq157_e1989_q_d_n18, eq157_e1989_q_d_n19, eq157_e1989_q_d_n20, eq157_e1989_q_d_n21, eq157_e1989_q_d_n22,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq157_e1986_q: f64 = s.v[264];
        let eq157_e1987: f64 = (p.p7 * s.v[264]);
        let eq157_e1987_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq157_e1987_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq157_e1987_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq157_e1987_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq157_e1987_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq157_e1987_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq157_e1987_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq157_e1987_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq157_e1987_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq157_e1987_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq157_e1987_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq157_e1987_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq157_e1987_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq157_e1987_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq157_e1987_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq157_e1987_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq157_e1987_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq157_e1987_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq157_e1987_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq157_e1987_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq157_e1987_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq157_e1987_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq157_e1987_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq157_e1987_q: f64 = (p.p7 * eq157_e1986_q);
        let eq157_e1987_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq157_e1987_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq157_e1987_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq157_e1987_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq157_e1987_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq157_e1987_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq157_e1987_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq157_e1987_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq157_e1987_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq157_e1987_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq157_e1987_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq157_e1987_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq157_e1987_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq157_e1987_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq157_e1987_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq157_e1987_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq157_e1987_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq157_e1987_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq157_e1987_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq157_e1987_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq157_e1987_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq157_e1987_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq157_e1987_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq157_e1987, eq157_e1987_d_n0, eq157_e1987_d_n1, eq157_e1987_d_n2, eq157_e1987_d_n3, eq157_e1987_d_n4, eq157_e1987_d_n5, eq157_e1987_d_n6, eq157_e1987_d_n7, eq157_e1987_d_n8, eq157_e1987_d_n9, eq157_e1987_d_n10, eq157_e1987_d_n11, eq157_e1987_d_n12, eq157_e1987_d_n13, eq157_e1987_d_n14, eq157_e1987_d_n15, eq157_e1987_d_n16, eq157_e1987_d_n17, eq157_e1987_d_n18, eq157_e1987_d_n19, eq157_e1987_d_n20, eq157_e1987_d_n21, eq157_e1987_d_n22, eq157_e1987_q, eq157_e1987_q_d_n0, eq157_e1987_q_d_n1, eq157_e1987_q_d_n2, eq157_e1987_q_d_n3, eq157_e1987_q_d_n4, eq157_e1987_q_d_n5, eq157_e1987_q_d_n6, eq157_e1987_q_d_n7, eq157_e1987_q_d_n8, eq157_e1987_q_d_n9, eq157_e1987_q_d_n10, eq157_e1987_q_d_n11, eq157_e1987_q_d_n12, eq157_e1987_q_d_n13, eq157_e1987_q_d_n14, eq157_e1987_q_d_n15, eq157_e1987_q_d_n16, eq157_e1987_q_d_n17, eq157_e1987_q_d_n18, eq157_e1987_q_d_n19, eq157_e1987_q_d_n20, eq157_e1987_q_d_n21, eq157_e1987_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_reactive_node_derivatives: [f64; 23] = [eq157_e1989_q_d_n0, eq157_e1989_q_d_n1, eq157_e1989_q_d_n2, eq157_e1989_q_d_n3, eq157_e1989_q_d_n4, eq157_e1989_q_d_n5, eq157_e1989_q_d_n6, eq157_e1989_q_d_n7, eq157_e1989_q_d_n8, eq157_e1989_q_d_n9, eq157_e1989_q_d_n10, eq157_e1989_q_d_n11, eq157_e1989_q_d_n12, eq157_e1989_q_d_n13, eq157_e1989_q_d_n14, eq157_e1989_q_d_n15, eq157_e1989_q_d_n16, eq157_e1989_q_d_n17, eq157_e1989_q_d_n18, eq157_e1989_q_d_n19, eq157_e1989_q_d_n20, eq157_e1989_q_d_n21, eq157_e1989_q_d_n22];
        let eq157_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            nodes,
            &eq157_reactive_node_derivatives,
            branches,
            &eq157_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_9(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22, eq158_e2002_q, eq158_e2002_q_d_n0, eq158_e2002_q_d_n1, eq158_e2002_q_d_n2, eq158_e2002_q_d_n3, eq158_e2002_q_d_n4, eq158_e2002_q_d_n5, eq158_e2002_q_d_n6, eq158_e2002_q_d_n7, eq158_e2002_q_d_n8, eq158_e2002_q_d_n9, eq158_e2002_q_d_n10, eq158_e2002_q_d_n11, eq158_e2002_q_d_n12, eq158_e2002_q_d_n13, eq158_e2002_q_d_n14, eq158_e2002_q_d_n15, eq158_e2002_q_d_n16, eq158_e2002_q_d_n17, eq158_e2002_q_d_n18, eq158_e2002_q_d_n19, eq158_e2002_q_d_n20, eq158_e2002_q_d_n21, eq158_e2002_q_d_n22,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq158_e1997_q: f64 = s.v[264];
        let eq158_e1998: f64 = (p.p7 * s.v[264]);
        let eq158_e1998_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq158_e1998_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq158_e1998_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq158_e1998_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq158_e1998_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq158_e1998_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq158_e1998_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq158_e1998_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq158_e1998_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq158_e1998_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq158_e1998_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq158_e1998_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq158_e1998_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq158_e1998_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq158_e1998_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq158_e1998_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq158_e1998_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq158_e1998_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq158_e1998_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq158_e1998_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq158_e1998_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq158_e1998_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq158_e1998_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq158_e1998_q: f64 = (p.p7 * eq158_e1997_q);
        let eq158_e1998_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq158_e1998_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq158_e1998_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq158_e1998_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq158_e1998_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq158_e1998_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq158_e1998_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq158_e1998_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq158_e1998_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq158_e1998_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq158_e1998_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq158_e1998_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq158_e1998_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq158_e1998_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq158_e1998_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq158_e1998_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq158_e1998_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq158_e1998_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq158_e1998_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq158_e1998_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq158_e1998_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq158_e1998_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq158_e1998_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (eq158_e1998_d_n0 * p.p247);
        let eq158_e2000_d_n1: f64 = (eq158_e1998_d_n1 * p.p247);
        let eq158_e2000_d_n2: f64 = (eq158_e1998_d_n2 * p.p247);
        let eq158_e2000_d_n3: f64 = (eq158_e1998_d_n3 * p.p247);
        let eq158_e2000_d_n4: f64 = (eq158_e1998_d_n4 * p.p247);
        let eq158_e2000_d_n5: f64 = (eq158_e1998_d_n5 * p.p247);
        let eq158_e2000_d_n6: f64 = (eq158_e1998_d_n6 * p.p247);
        let eq158_e2000_d_n7: f64 = (eq158_e1998_d_n7 * p.p247);
        let eq158_e2000_d_n8: f64 = (eq158_e1998_d_n8 * p.p247);
        let eq158_e2000_d_n9: f64 = (eq158_e1998_d_n9 * p.p247);
        let eq158_e2000_d_n10: f64 = (eq158_e1998_d_n10 * p.p247);
        let eq158_e2000_d_n11: f64 = (eq158_e1998_d_n11 * p.p247);
        let eq158_e2000_d_n12: f64 = (eq158_e1998_d_n12 * p.p247);
        let eq158_e2000_d_n13: f64 = (eq158_e1998_d_n13 * p.p247);
        let eq158_e2000_d_n14: f64 = (eq158_e1998_d_n14 * p.p247);
        let eq158_e2000_d_n15: f64 = (eq158_e1998_d_n15 * p.p247);
        let eq158_e2000_d_n16: f64 = (eq158_e1998_d_n16 * p.p247);
        let eq158_e2000_d_n17: f64 = (eq158_e1998_d_n17 * p.p247);
        let eq158_e2000_d_n18: f64 = (eq158_e1998_d_n18 * p.p247);
        let eq158_e2000_d_n19: f64 = (eq158_e1998_d_n19 * p.p247);
        let eq158_e2000_d_n20: f64 = (eq158_e1998_d_n20 * p.p247);
        let eq158_e2000_d_n21: f64 = (eq158_e1998_d_n21 * p.p247);
        let eq158_e2000_d_n22: f64 = (eq158_e1998_d_n22 * p.p247);
        let eq158_e2000_q: f64 = (eq158_e1998_q * p.p247);
        let eq158_e2000_q_d_n0: f64 = (eq158_e1998_q_d_n0 * p.p247);
        let eq158_e2000_q_d_n1: f64 = (eq158_e1998_q_d_n1 * p.p247);
        let eq158_e2000_q_d_n2: f64 = (eq158_e1998_q_d_n2 * p.p247);
        let eq158_e2000_q_d_n3: f64 = (eq158_e1998_q_d_n3 * p.p247);
        let eq158_e2000_q_d_n4: f64 = (eq158_e1998_q_d_n4 * p.p247);
        let eq158_e2000_q_d_n5: f64 = (eq158_e1998_q_d_n5 * p.p247);
        let eq158_e2000_q_d_n6: f64 = (eq158_e1998_q_d_n6 * p.p247);
        let eq158_e2000_q_d_n7: f64 = (eq158_e1998_q_d_n7 * p.p247);
        let eq158_e2000_q_d_n8: f64 = (eq158_e1998_q_d_n8 * p.p247);
        let eq158_e2000_q_d_n9: f64 = (eq158_e1998_q_d_n9 * p.p247);
        let eq158_e2000_q_d_n10: f64 = (eq158_e1998_q_d_n10 * p.p247);
        let eq158_e2000_q_d_n11: f64 = (eq158_e1998_q_d_n11 * p.p247);
        let eq158_e2000_q_d_n12: f64 = (eq158_e1998_q_d_n12 * p.p247);
        let eq158_e2000_q_d_n13: f64 = (eq158_e1998_q_d_n13 * p.p247);
        let eq158_e2000_q_d_n14: f64 = (eq158_e1998_q_d_n14 * p.p247);
        let eq158_e2000_q_d_n15: f64 = (eq158_e1998_q_d_n15 * p.p247);
        let eq158_e2000_q_d_n16: f64 = (eq158_e1998_q_d_n16 * p.p247);
        let eq158_e2000_q_d_n17: f64 = (eq158_e1998_q_d_n17 * p.p247);
        let eq158_e2000_q_d_n18: f64 = (eq158_e1998_q_d_n18 * p.p247);
        let eq158_e2000_q_d_n19: f64 = (eq158_e1998_q_d_n19 * p.p247);
        let eq158_e2000_q_d_n20: f64 = (eq158_e1998_q_d_n20 * p.p247);
        let eq158_e2000_q_d_n21: f64 = (eq158_e1998_q_d_n21 * p.p247);
        let eq158_e2000_q_d_n22: f64 = (eq158_e1998_q_d_n22 * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n10, eq158_e2000_d_n11, eq158_e2000_d_n12, eq158_e2000_d_n13, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22, eq158_e2000_q, eq158_e2000_q_d_n0, eq158_e2000_q_d_n1, eq158_e2000_q_d_n2, eq158_e2000_q_d_n3, eq158_e2000_q_d_n4, eq158_e2000_q_d_n5, eq158_e2000_q_d_n6, eq158_e2000_q_d_n7, eq158_e2000_q_d_n8, eq158_e2000_q_d_n9, eq158_e2000_q_d_n10, eq158_e2000_q_d_n11, eq158_e2000_q_d_n12, eq158_e2000_q_d_n13, eq158_e2000_q_d_n14, eq158_e2000_q_d_n15, eq158_e2000_q_d_n16, eq158_e2000_q_d_n17, eq158_e2000_q_d_n18, eq158_e2000_q_d_n19, eq158_e2000_q_d_n20, eq158_e2000_q_d_n21, eq158_e2000_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_reactive_node_derivatives: [f64; 23] = [eq158_e2002_q_d_n0, eq158_e2002_q_d_n1, eq158_e2002_q_d_n2, eq158_e2002_q_d_n3, eq158_e2002_q_d_n4, eq158_e2002_q_d_n5, eq158_e2002_q_d_n6, eq158_e2002_q_d_n7, eq158_e2002_q_d_n8, eq158_e2002_q_d_n9, eq158_e2002_q_d_n10, eq158_e2002_q_d_n11, eq158_e2002_q_d_n12, eq158_e2002_q_d_n13, eq158_e2002_q_d_n14, eq158_e2002_q_d_n15, eq158_e2002_q_d_n16, eq158_e2002_q_d_n17, eq158_e2002_q_d_n18, eq158_e2002_q_d_n19, eq158_e2002_q_d_n20, eq158_e2002_q_d_n21, eq158_e2002_q_d_n22];
        let eq158_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            nodes,
            &eq158_reactive_node_derivatives,
            branches,
            &eq158_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22, eq159_e2014_q, eq159_e2014_q_d_n0, eq159_e2014_q_d_n1, eq159_e2014_q_d_n2, eq159_e2014_q_d_n3, eq159_e2014_q_d_n4, eq159_e2014_q_d_n5, eq159_e2014_q_d_n6, eq159_e2014_q_d_n7, eq159_e2014_q_d_n8, eq159_e2014_q_d_n9, eq159_e2014_q_d_n10, eq159_e2014_q_d_n11, eq159_e2014_q_d_n12, eq159_e2014_q_d_n13, eq159_e2014_q_d_n14, eq159_e2014_q_d_n15, eq159_e2014_q_d_n16, eq159_e2014_q_d_n17, eq159_e2014_q_d_n18, eq159_e2014_q_d_n19, eq159_e2014_q_d_n20, eq159_e2014_q_d_n21, eq159_e2014_q_d_n22,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq159_e2011_q: f64 = s.v[264];
        let eq159_e2012: f64 = (p.p7 * s.v[264]);
        let eq159_e2012_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq159_e2012_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq159_e2012_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq159_e2012_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq159_e2012_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq159_e2012_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq159_e2012_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq159_e2012_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq159_e2012_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq159_e2012_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq159_e2012_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq159_e2012_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq159_e2012_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq159_e2012_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq159_e2012_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq159_e2012_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq159_e2012_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq159_e2012_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq159_e2012_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq159_e2012_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq159_e2012_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq159_e2012_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq159_e2012_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq159_e2012_q: f64 = (p.p7 * eq159_e2011_q);
        let eq159_e2012_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq159_e2012_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq159_e2012_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq159_e2012_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq159_e2012_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq159_e2012_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq159_e2012_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq159_e2012_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq159_e2012_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq159_e2012_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq159_e2012_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq159_e2012_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq159_e2012_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq159_e2012_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq159_e2012_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq159_e2012_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq159_e2012_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq159_e2012_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq159_e2012_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq159_e2012_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq159_e2012_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq159_e2012_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq159_e2012_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq159_e2012, eq159_e2012_d_n0, eq159_e2012_d_n1, eq159_e2012_d_n2, eq159_e2012_d_n3, eq159_e2012_d_n4, eq159_e2012_d_n5, eq159_e2012_d_n6, eq159_e2012_d_n7, eq159_e2012_d_n8, eq159_e2012_d_n9, eq159_e2012_d_n10, eq159_e2012_d_n11, eq159_e2012_d_n12, eq159_e2012_d_n13, eq159_e2012_d_n14, eq159_e2012_d_n15, eq159_e2012_d_n16, eq159_e2012_d_n17, eq159_e2012_d_n18, eq159_e2012_d_n19, eq159_e2012_d_n20, eq159_e2012_d_n21, eq159_e2012_d_n22, eq159_e2012_q, eq159_e2012_q_d_n0, eq159_e2012_q_d_n1, eq159_e2012_q_d_n2, eq159_e2012_q_d_n3, eq159_e2012_q_d_n4, eq159_e2012_q_d_n5, eq159_e2012_q_d_n6, eq159_e2012_q_d_n7, eq159_e2012_q_d_n8, eq159_e2012_q_d_n9, eq159_e2012_q_d_n10, eq159_e2012_q_d_n11, eq159_e2012_q_d_n12, eq159_e2012_q_d_n13, eq159_e2012_q_d_n14, eq159_e2012_q_d_n15, eq159_e2012_q_d_n16, eq159_e2012_q_d_n17, eq159_e2012_q_d_n18, eq159_e2012_q_d_n19, eq159_e2012_q_d_n20, eq159_e2012_q_d_n21, eq159_e2012_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_reactive_node_derivatives: [f64; 23] = [eq159_e2014_q_d_n0, eq159_e2014_q_d_n1, eq159_e2014_q_d_n2, eq159_e2014_q_d_n3, eq159_e2014_q_d_n4, eq159_e2014_q_d_n5, eq159_e2014_q_d_n6, eq159_e2014_q_d_n7, eq159_e2014_q_d_n8, eq159_e2014_q_d_n9, eq159_e2014_q_d_n10, eq159_e2014_q_d_n11, eq159_e2014_q_d_n12, eq159_e2014_q_d_n13, eq159_e2014_q_d_n14, eq159_e2014_q_d_n15, eq159_e2014_q_d_n16, eq159_e2014_q_d_n17, eq159_e2014_q_d_n18, eq159_e2014_q_d_n19, eq159_e2014_q_d_n20, eq159_e2014_q_d_n21, eq159_e2014_q_d_n22];
        let eq159_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            nodes,
            &eq159_reactive_node_derivatives,
            branches,
            &eq159_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22, eq160_e2028_q, eq160_e2028_q_d_n0, eq160_e2028_q_d_n1, eq160_e2028_q_d_n2, eq160_e2028_q_d_n3, eq160_e2028_q_d_n4, eq160_e2028_q_d_n5, eq160_e2028_q_d_n6, eq160_e2028_q_d_n7, eq160_e2028_q_d_n8, eq160_e2028_q_d_n9, eq160_e2028_q_d_n10, eq160_e2028_q_d_n11, eq160_e2028_q_d_n12, eq160_e2028_q_d_n13, eq160_e2028_q_d_n14, eq160_e2028_q_d_n15, eq160_e2028_q_d_n16, eq160_e2028_q_d_n17, eq160_e2028_q_d_n18, eq160_e2028_q_d_n19, eq160_e2028_q_d_n20, eq160_e2028_q_d_n21, eq160_e2028_q_d_n22,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq160_e2023_q: f64 = s.v[264];
        let eq160_e2024: f64 = (p.p7 * s.v[264]);
        let eq160_e2024_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq160_e2024_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq160_e2024_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq160_e2024_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq160_e2024_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq160_e2024_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq160_e2024_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq160_e2024_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq160_e2024_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq160_e2024_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq160_e2024_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq160_e2024_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq160_e2024_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq160_e2024_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq160_e2024_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq160_e2024_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq160_e2024_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq160_e2024_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq160_e2024_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq160_e2024_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq160_e2024_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq160_e2024_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq160_e2024_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq160_e2024_q: f64 = (p.p7 * eq160_e2023_q);
        let eq160_e2024_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq160_e2024_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq160_e2024_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq160_e2024_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq160_e2024_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq160_e2024_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq160_e2024_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq160_e2024_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq160_e2024_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq160_e2024_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq160_e2024_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq160_e2024_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq160_e2024_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq160_e2024_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq160_e2024_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq160_e2024_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq160_e2024_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq160_e2024_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq160_e2024_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq160_e2024_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq160_e2024_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq160_e2024_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq160_e2024_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq160_e2026: f64 = (eq160_e2024 * p.p247);
        let eq160_e2026_d_n0: f64 = (eq160_e2024_d_n0 * p.p247);
        let eq160_e2026_d_n1: f64 = (eq160_e2024_d_n1 * p.p247);
        let eq160_e2026_d_n2: f64 = (eq160_e2024_d_n2 * p.p247);
        let eq160_e2026_d_n3: f64 = (eq160_e2024_d_n3 * p.p247);
        let eq160_e2026_d_n4: f64 = (eq160_e2024_d_n4 * p.p247);
        let eq160_e2026_d_n5: f64 = (eq160_e2024_d_n5 * p.p247);
        let eq160_e2026_d_n6: f64 = (eq160_e2024_d_n6 * p.p247);
        let eq160_e2026_d_n7: f64 = (eq160_e2024_d_n7 * p.p247);
        let eq160_e2026_d_n8: f64 = (eq160_e2024_d_n8 * p.p247);
        let eq160_e2026_d_n9: f64 = (eq160_e2024_d_n9 * p.p247);
        let eq160_e2026_d_n10: f64 = (eq160_e2024_d_n10 * p.p247);
        let eq160_e2026_d_n11: f64 = (eq160_e2024_d_n11 * p.p247);
        let eq160_e2026_d_n12: f64 = (eq160_e2024_d_n12 * p.p247);
        let eq160_e2026_d_n13: f64 = (eq160_e2024_d_n13 * p.p247);
        let eq160_e2026_d_n14: f64 = (eq160_e2024_d_n14 * p.p247);
        let eq160_e2026_d_n15: f64 = (eq160_e2024_d_n15 * p.p247);
        let eq160_e2026_d_n16: f64 = (eq160_e2024_d_n16 * p.p247);
        let eq160_e2026_d_n17: f64 = (eq160_e2024_d_n17 * p.p247);
        let eq160_e2026_d_n18: f64 = (eq160_e2024_d_n18 * p.p247);
        let eq160_e2026_d_n19: f64 = (eq160_e2024_d_n19 * p.p247);
        let eq160_e2026_d_n20: f64 = (eq160_e2024_d_n20 * p.p247);
        let eq160_e2026_d_n21: f64 = (eq160_e2024_d_n21 * p.p247);
        let eq160_e2026_d_n22: f64 = (eq160_e2024_d_n22 * p.p247);
        let eq160_e2026_q: f64 = (eq160_e2024_q * p.p247);
        let eq160_e2026_q_d_n0: f64 = (eq160_e2024_q_d_n0 * p.p247);
        let eq160_e2026_q_d_n1: f64 = (eq160_e2024_q_d_n1 * p.p247);
        let eq160_e2026_q_d_n2: f64 = (eq160_e2024_q_d_n2 * p.p247);
        let eq160_e2026_q_d_n3: f64 = (eq160_e2024_q_d_n3 * p.p247);
        let eq160_e2026_q_d_n4: f64 = (eq160_e2024_q_d_n4 * p.p247);
        let eq160_e2026_q_d_n5: f64 = (eq160_e2024_q_d_n5 * p.p247);
        let eq160_e2026_q_d_n6: f64 = (eq160_e2024_q_d_n6 * p.p247);
        let eq160_e2026_q_d_n7: f64 = (eq160_e2024_q_d_n7 * p.p247);
        let eq160_e2026_q_d_n8: f64 = (eq160_e2024_q_d_n8 * p.p247);
        let eq160_e2026_q_d_n9: f64 = (eq160_e2024_q_d_n9 * p.p247);
        let eq160_e2026_q_d_n10: f64 = (eq160_e2024_q_d_n10 * p.p247);
        let eq160_e2026_q_d_n11: f64 = (eq160_e2024_q_d_n11 * p.p247);
        let eq160_e2026_q_d_n12: f64 = (eq160_e2024_q_d_n12 * p.p247);
        let eq160_e2026_q_d_n13: f64 = (eq160_e2024_q_d_n13 * p.p247);
        let eq160_e2026_q_d_n14: f64 = (eq160_e2024_q_d_n14 * p.p247);
        let eq160_e2026_q_d_n15: f64 = (eq160_e2024_q_d_n15 * p.p247);
        let eq160_e2026_q_d_n16: f64 = (eq160_e2024_q_d_n16 * p.p247);
        let eq160_e2026_q_d_n17: f64 = (eq160_e2024_q_d_n17 * p.p247);
        let eq160_e2026_q_d_n18: f64 = (eq160_e2024_q_d_n18 * p.p247);
        let eq160_e2026_q_d_n19: f64 = (eq160_e2024_q_d_n19 * p.p247);
        let eq160_e2026_q_d_n20: f64 = (eq160_e2024_q_d_n20 * p.p247);
        let eq160_e2026_q_d_n21: f64 = (eq160_e2024_q_d_n21 * p.p247);
        let eq160_e2026_q_d_n22: f64 = (eq160_e2024_q_d_n22 * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n10, eq160_e2026_d_n11, eq160_e2026_d_n12, eq160_e2026_d_n13, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22, eq160_e2026_q, eq160_e2026_q_d_n0, eq160_e2026_q_d_n1, eq160_e2026_q_d_n2, eq160_e2026_q_d_n3, eq160_e2026_q_d_n4, eq160_e2026_q_d_n5, eq160_e2026_q_d_n6, eq160_e2026_q_d_n7, eq160_e2026_q_d_n8, eq160_e2026_q_d_n9, eq160_e2026_q_d_n10, eq160_e2026_q_d_n11, eq160_e2026_q_d_n12, eq160_e2026_q_d_n13, eq160_e2026_q_d_n14, eq160_e2026_q_d_n15, eq160_e2026_q_d_n16, eq160_e2026_q_d_n17, eq160_e2026_q_d_n18, eq160_e2026_q_d_n19, eq160_e2026_q_d_n20, eq160_e2026_q_d_n21, eq160_e2026_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_reactive_node_derivatives: [f64; 23] = [eq160_e2028_q_d_n0, eq160_e2028_q_d_n1, eq160_e2028_q_d_n2, eq160_e2028_q_d_n3, eq160_e2028_q_d_n4, eq160_e2028_q_d_n5, eq160_e2028_q_d_n6, eq160_e2028_q_d_n7, eq160_e2028_q_d_n8, eq160_e2028_q_d_n9, eq160_e2028_q_d_n10, eq160_e2028_q_d_n11, eq160_e2028_q_d_n12, eq160_e2028_q_d_n13, eq160_e2028_q_d_n14, eq160_e2028_q_d_n15, eq160_e2028_q_d_n16, eq160_e2028_q_d_n17, eq160_e2028_q_d_n18, eq160_e2028_q_d_n19, eq160_e2028_q_d_n20, eq160_e2028_q_d_n21, eq160_e2028_q_d_n22];
        let eq160_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            nodes,
            &eq160_reactive_node_derivatives,
            branches,
            &eq160_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22, eq161_e2039_q, eq161_e2039_q_d_n0, eq161_e2039_q_d_n1, eq161_e2039_q_d_n2, eq161_e2039_q_d_n3, eq161_e2039_q_d_n4, eq161_e2039_q_d_n5, eq161_e2039_q_d_n6, eq161_e2039_q_d_n7, eq161_e2039_q_d_n8, eq161_e2039_q_d_n9, eq161_e2039_q_d_n10, eq161_e2039_q_d_n11, eq161_e2039_q_d_n12, eq161_e2039_q_d_n13, eq161_e2039_q_d_n14, eq161_e2039_q_d_n15, eq161_e2039_q_d_n16, eq161_e2039_q_d_n17, eq161_e2039_q_d_n18, eq161_e2039_q_d_n19, eq161_e2039_q_d_n20, eq161_e2039_q_d_n21, eq161_e2039_q_d_n22,) = {
    if (s.b[585] && s.b[586]) {
        let eq161_e2035: f64 = (p.p252 * s.v[264]);
        let eq161_e2035_d_n0: f64 = (p.p252 * s.dn[264][0]);
        let eq161_e2035_d_n1: f64 = (p.p252 * s.dn[264][1]);
        let eq161_e2035_d_n2: f64 = (p.p252 * s.dn[264][2]);
        let eq161_e2035_d_n3: f64 = (p.p252 * s.dn[264][3]);
        let eq161_e2035_d_n4: f64 = (p.p252 * s.dn[264][4]);
        let eq161_e2035_d_n5: f64 = (p.p252 * s.dn[264][5]);
        let eq161_e2035_d_n6: f64 = (p.p252 * s.dn[264][6]);
        let eq161_e2035_d_n7: f64 = (p.p252 * s.dn[264][7]);
        let eq161_e2035_d_n8: f64 = (p.p252 * s.dn[264][8]);
        let eq161_e2035_d_n9: f64 = (p.p252 * s.dn[264][9]);
        let eq161_e2035_d_n10: f64 = (p.p252 * s.dn[264][10]);
        let eq161_e2035_d_n11: f64 = (p.p252 * s.dn[264][11]);
        let eq161_e2035_d_n12: f64 = (p.p252 * s.dn[264][12]);
        let eq161_e2035_d_n13: f64 = (p.p252 * s.dn[264][13]);
        let eq161_e2035_d_n14: f64 = (p.p252 * s.dn[264][14]);
        let eq161_e2035_d_n15: f64 = (p.p252 * s.dn[264][15]);
        let eq161_e2035_d_n16: f64 = (p.p252 * s.dn[264][16]);
        let eq161_e2035_d_n17: f64 = (p.p252 * s.dn[264][17]);
        let eq161_e2035_d_n18: f64 = (p.p252 * s.dn[264][18]);
        let eq161_e2035_d_n19: f64 = (p.p252 * s.dn[264][19]);
        let eq161_e2035_d_n20: f64 = (p.p252 * s.dn[264][20]);
        let eq161_e2035_d_n21: f64 = (p.p252 * s.dn[264][21]);
        let eq161_e2035_d_n22: f64 = (p.p252 * s.dn[264][22]);
        let eq161_e2036_q: f64 = eq161_e2035;
        let eq161_e2037: f64 = (p.p7 * eq161_e2035);
        let eq161_e2037_d_n0: f64 = (p.p7 * eq161_e2035_d_n0);
        let eq161_e2037_d_n1: f64 = (p.p7 * eq161_e2035_d_n1);
        let eq161_e2037_d_n2: f64 = (p.p7 * eq161_e2035_d_n2);
        let eq161_e2037_d_n3: f64 = (p.p7 * eq161_e2035_d_n3);
        let eq161_e2037_d_n4: f64 = (p.p7 * eq161_e2035_d_n4);
        let eq161_e2037_d_n5: f64 = (p.p7 * eq161_e2035_d_n5);
        let eq161_e2037_d_n6: f64 = (p.p7 * eq161_e2035_d_n6);
        let eq161_e2037_d_n7: f64 = (p.p7 * eq161_e2035_d_n7);
        let eq161_e2037_d_n8: f64 = (p.p7 * eq161_e2035_d_n8);
        let eq161_e2037_d_n9: f64 = (p.p7 * eq161_e2035_d_n9);
        let eq161_e2037_d_n10: f64 = (p.p7 * eq161_e2035_d_n10);
        let eq161_e2037_d_n11: f64 = (p.p7 * eq161_e2035_d_n11);
        let eq161_e2037_d_n12: f64 = (p.p7 * eq161_e2035_d_n12);
        let eq161_e2037_d_n13: f64 = (p.p7 * eq161_e2035_d_n13);
        let eq161_e2037_d_n14: f64 = (p.p7 * eq161_e2035_d_n14);
        let eq161_e2037_d_n15: f64 = (p.p7 * eq161_e2035_d_n15);
        let eq161_e2037_d_n16: f64 = (p.p7 * eq161_e2035_d_n16);
        let eq161_e2037_d_n17: f64 = (p.p7 * eq161_e2035_d_n17);
        let eq161_e2037_d_n18: f64 = (p.p7 * eq161_e2035_d_n18);
        let eq161_e2037_d_n19: f64 = (p.p7 * eq161_e2035_d_n19);
        let eq161_e2037_d_n20: f64 = (p.p7 * eq161_e2035_d_n20);
        let eq161_e2037_d_n21: f64 = (p.p7 * eq161_e2035_d_n21);
        let eq161_e2037_d_n22: f64 = (p.p7 * eq161_e2035_d_n22);
        let eq161_e2037_q: f64 = (p.p7 * eq161_e2036_q);
        let eq161_e2037_q_d_n0: f64 = (p.p7 * eq161_e2035_d_n0);
        let eq161_e2037_q_d_n1: f64 = (p.p7 * eq161_e2035_d_n1);
        let eq161_e2037_q_d_n2: f64 = (p.p7 * eq161_e2035_d_n2);
        let eq161_e2037_q_d_n3: f64 = (p.p7 * eq161_e2035_d_n3);
        let eq161_e2037_q_d_n4: f64 = (p.p7 * eq161_e2035_d_n4);
        let eq161_e2037_q_d_n5: f64 = (p.p7 * eq161_e2035_d_n5);
        let eq161_e2037_q_d_n6: f64 = (p.p7 * eq161_e2035_d_n6);
        let eq161_e2037_q_d_n7: f64 = (p.p7 * eq161_e2035_d_n7);
        let eq161_e2037_q_d_n8: f64 = (p.p7 * eq161_e2035_d_n8);
        let eq161_e2037_q_d_n9: f64 = (p.p7 * eq161_e2035_d_n9);
        let eq161_e2037_q_d_n10: f64 = (p.p7 * eq161_e2035_d_n10);
        let eq161_e2037_q_d_n11: f64 = (p.p7 * eq161_e2035_d_n11);
        let eq161_e2037_q_d_n12: f64 = (p.p7 * eq161_e2035_d_n12);
        let eq161_e2037_q_d_n13: f64 = (p.p7 * eq161_e2035_d_n13);
        let eq161_e2037_q_d_n14: f64 = (p.p7 * eq161_e2035_d_n14);
        let eq161_e2037_q_d_n15: f64 = (p.p7 * eq161_e2035_d_n15);
        let eq161_e2037_q_d_n16: f64 = (p.p7 * eq161_e2035_d_n16);
        let eq161_e2037_q_d_n17: f64 = (p.p7 * eq161_e2035_d_n17);
        let eq161_e2037_q_d_n18: f64 = (p.p7 * eq161_e2035_d_n18);
        let eq161_e2037_q_d_n19: f64 = (p.p7 * eq161_e2035_d_n19);
        let eq161_e2037_q_d_n20: f64 = (p.p7 * eq161_e2035_d_n20);
        let eq161_e2037_q_d_n21: f64 = (p.p7 * eq161_e2035_d_n21);
        let eq161_e2037_q_d_n22: f64 = (p.p7 * eq161_e2035_d_n22);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n10, eq161_e2037_d_n11, eq161_e2037_d_n12, eq161_e2037_d_n13, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22, eq161_e2037_q, eq161_e2037_q_d_n0, eq161_e2037_q_d_n1, eq161_e2037_q_d_n2, eq161_e2037_q_d_n3, eq161_e2037_q_d_n4, eq161_e2037_q_d_n5, eq161_e2037_q_d_n6, eq161_e2037_q_d_n7, eq161_e2037_q_d_n8, eq161_e2037_q_d_n9, eq161_e2037_q_d_n10, eq161_e2037_q_d_n11, eq161_e2037_q_d_n12, eq161_e2037_q_d_n13, eq161_e2037_q_d_n14, eq161_e2037_q_d_n15, eq161_e2037_q_d_n16, eq161_e2037_q_d_n17, eq161_e2037_q_d_n18, eq161_e2037_q_d_n19, eq161_e2037_q_d_n20, eq161_e2037_q_d_n21, eq161_e2037_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_reactive_node_derivatives: [f64; 23] = [eq161_e2039_q_d_n0, eq161_e2039_q_d_n1, eq161_e2039_q_d_n2, eq161_e2039_q_d_n3, eq161_e2039_q_d_n4, eq161_e2039_q_d_n5, eq161_e2039_q_d_n6, eq161_e2039_q_d_n7, eq161_e2039_q_d_n8, eq161_e2039_q_d_n9, eq161_e2039_q_d_n10, eq161_e2039_q_d_n11, eq161_e2039_q_d_n12, eq161_e2039_q_d_n13, eq161_e2039_q_d_n14, eq161_e2039_q_d_n15, eq161_e2039_q_d_n16, eq161_e2039_q_d_n17, eq161_e2039_q_d_n18, eq161_e2039_q_d_n19, eq161_e2039_q_d_n20, eq161_e2039_q_d_n21, eq161_e2039_q_d_n22];
        let eq161_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[20]),
            nodes,
            &eq161_reactive_node_derivatives,
            branches,
            &eq161_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22, eq162_e2049_q, eq162_e2049_q_d_n0, eq162_e2049_q_d_n1, eq162_e2049_q_d_n2, eq162_e2049_q_d_n3, eq162_e2049_q_d_n4, eq162_e2049_q_d_n5, eq162_e2049_q_d_n6, eq162_e2049_q_d_n7, eq162_e2049_q_d_n8, eq162_e2049_q_d_n9, eq162_e2049_q_d_n10, eq162_e2049_q_d_n11, eq162_e2049_q_d_n12, eq162_e2049_q_d_n13, eq162_e2049_q_d_n14, eq162_e2049_q_d_n15, eq162_e2049_q_d_n16, eq162_e2049_q_d_n17, eq162_e2049_q_d_n18, eq162_e2049_q_d_n19, eq162_e2049_q_d_n20, eq162_e2049_q_d_n21, eq162_e2049_q_d_n22,) = {
    if ((!s.b[585]) && s.b[588]) {
        let eq162_e2046_q: f64 = s.v[265];
        let eq162_e2047: f64 = (p.p7 * s.v[265]);
        let eq162_e2047_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq162_e2047_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq162_e2047_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq162_e2047_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq162_e2047_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq162_e2047_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq162_e2047_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq162_e2047_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq162_e2047_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq162_e2047_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq162_e2047_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq162_e2047_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq162_e2047_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq162_e2047_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq162_e2047_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq162_e2047_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq162_e2047_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq162_e2047_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq162_e2047_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq162_e2047_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq162_e2047_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq162_e2047_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq162_e2047_d_n22: f64 = (p.p7 * s.dn[265][22]);
        let eq162_e2047_q: f64 = (p.p7 * eq162_e2046_q);
        let eq162_e2047_q_d_n0: f64 = (p.p7 * s.dn[265][0]);
        let eq162_e2047_q_d_n1: f64 = (p.p7 * s.dn[265][1]);
        let eq162_e2047_q_d_n2: f64 = (p.p7 * s.dn[265][2]);
        let eq162_e2047_q_d_n3: f64 = (p.p7 * s.dn[265][3]);
        let eq162_e2047_q_d_n4: f64 = (p.p7 * s.dn[265][4]);
        let eq162_e2047_q_d_n5: f64 = (p.p7 * s.dn[265][5]);
        let eq162_e2047_q_d_n6: f64 = (p.p7 * s.dn[265][6]);
        let eq162_e2047_q_d_n7: f64 = (p.p7 * s.dn[265][7]);
        let eq162_e2047_q_d_n8: f64 = (p.p7 * s.dn[265][8]);
        let eq162_e2047_q_d_n9: f64 = (p.p7 * s.dn[265][9]);
        let eq162_e2047_q_d_n10: f64 = (p.p7 * s.dn[265][10]);
        let eq162_e2047_q_d_n11: f64 = (p.p7 * s.dn[265][11]);
        let eq162_e2047_q_d_n12: f64 = (p.p7 * s.dn[265][12]);
        let eq162_e2047_q_d_n13: f64 = (p.p7 * s.dn[265][13]);
        let eq162_e2047_q_d_n14: f64 = (p.p7 * s.dn[265][14]);
        let eq162_e2047_q_d_n15: f64 = (p.p7 * s.dn[265][15]);
        let eq162_e2047_q_d_n16: f64 = (p.p7 * s.dn[265][16]);
        let eq162_e2047_q_d_n17: f64 = (p.p7 * s.dn[265][17]);
        let eq162_e2047_q_d_n18: f64 = (p.p7 * s.dn[265][18]);
        let eq162_e2047_q_d_n19: f64 = (p.p7 * s.dn[265][19]);
        let eq162_e2047_q_d_n20: f64 = (p.p7 * s.dn[265][20]);
        let eq162_e2047_q_d_n21: f64 = (p.p7 * s.dn[265][21]);
        let eq162_e2047_q_d_n22: f64 = (p.p7 * s.dn[265][22]);
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n10, eq162_e2047_d_n11, eq162_e2047_d_n12, eq162_e2047_d_n13, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22, eq162_e2047_q, eq162_e2047_q_d_n0, eq162_e2047_q_d_n1, eq162_e2047_q_d_n2, eq162_e2047_q_d_n3, eq162_e2047_q_d_n4, eq162_e2047_q_d_n5, eq162_e2047_q_d_n6, eq162_e2047_q_d_n7, eq162_e2047_q_d_n8, eq162_e2047_q_d_n9, eq162_e2047_q_d_n10, eq162_e2047_q_d_n11, eq162_e2047_q_d_n12, eq162_e2047_q_d_n13, eq162_e2047_q_d_n14, eq162_e2047_q_d_n15, eq162_e2047_q_d_n16, eq162_e2047_q_d_n17, eq162_e2047_q_d_n18, eq162_e2047_q_d_n19, eq162_e2047_q_d_n20, eq162_e2047_q_d_n21, eq162_e2047_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_reactive_node_derivatives: [f64; 23] = [eq162_e2049_q_d_n0, eq162_e2049_q_d_n1, eq162_e2049_q_d_n2, eq162_e2049_q_d_n3, eq162_e2049_q_d_n4, eq162_e2049_q_d_n5, eq162_e2049_q_d_n6, eq162_e2049_q_d_n7, eq162_e2049_q_d_n8, eq162_e2049_q_d_n9, eq162_e2049_q_d_n10, eq162_e2049_q_d_n11, eq162_e2049_q_d_n12, eq162_e2049_q_d_n13, eq162_e2049_q_d_n14, eq162_e2049_q_d_n15, eq162_e2049_q_d_n16, eq162_e2049_q_d_n17, eq162_e2049_q_d_n18, eq162_e2049_q_d_n19, eq162_e2049_q_d_n20, eq162_e2049_q_d_n21, eq162_e2049_q_d_n22];
        let eq162_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq162_reactive_node_derivatives,
            branches,
            &eq162_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_10(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22, eq163_e2061_q, eq163_e2061_q_d_n0, eq163_e2061_q_d_n1, eq163_e2061_q_d_n2, eq163_e2061_q_d_n3, eq163_e2061_q_d_n4, eq163_e2061_q_d_n5, eq163_e2061_q_d_n6, eq163_e2061_q_d_n7, eq163_e2061_q_d_n8, eq163_e2061_q_d_n9, eq163_e2061_q_d_n10, eq163_e2061_q_d_n11, eq163_e2061_q_d_n12, eq163_e2061_q_d_n13, eq163_e2061_q_d_n14, eq163_e2061_q_d_n15, eq163_e2061_q_d_n16, eq163_e2061_q_d_n17, eq163_e2061_q_d_n18, eq163_e2061_q_d_n19, eq163_e2061_q_d_n20, eq163_e2061_q_d_n21, eq163_e2061_q_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq163_e2058_q: f64 = s.v[264];
        let eq163_e2059: f64 = (p.p7 * s.v[264]);
        let eq163_e2059_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq163_e2059_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq163_e2059_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq163_e2059_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq163_e2059_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq163_e2059_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq163_e2059_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq163_e2059_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq163_e2059_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq163_e2059_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq163_e2059_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq163_e2059_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq163_e2059_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq163_e2059_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq163_e2059_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq163_e2059_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq163_e2059_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq163_e2059_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq163_e2059_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq163_e2059_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq163_e2059_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq163_e2059_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq163_e2059_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq163_e2059_q: f64 = (p.p7 * eq163_e2058_q);
        let eq163_e2059_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq163_e2059_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq163_e2059_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq163_e2059_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq163_e2059_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq163_e2059_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq163_e2059_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq163_e2059_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq163_e2059_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq163_e2059_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq163_e2059_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq163_e2059_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq163_e2059_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq163_e2059_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq163_e2059_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq163_e2059_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq163_e2059_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq163_e2059_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq163_e2059_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq163_e2059_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq163_e2059_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq163_e2059_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq163_e2059_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq163_e2059, eq163_e2059_d_n0, eq163_e2059_d_n1, eq163_e2059_d_n2, eq163_e2059_d_n3, eq163_e2059_d_n4, eq163_e2059_d_n5, eq163_e2059_d_n6, eq163_e2059_d_n7, eq163_e2059_d_n8, eq163_e2059_d_n9, eq163_e2059_d_n10, eq163_e2059_d_n11, eq163_e2059_d_n12, eq163_e2059_d_n13, eq163_e2059_d_n14, eq163_e2059_d_n15, eq163_e2059_d_n16, eq163_e2059_d_n17, eq163_e2059_d_n18, eq163_e2059_d_n19, eq163_e2059_d_n20, eq163_e2059_d_n21, eq163_e2059_d_n22, eq163_e2059_q, eq163_e2059_q_d_n0, eq163_e2059_q_d_n1, eq163_e2059_q_d_n2, eq163_e2059_q_d_n3, eq163_e2059_q_d_n4, eq163_e2059_q_d_n5, eq163_e2059_q_d_n6, eq163_e2059_q_d_n7, eq163_e2059_q_d_n8, eq163_e2059_q_d_n9, eq163_e2059_q_d_n10, eq163_e2059_q_d_n11, eq163_e2059_q_d_n12, eq163_e2059_q_d_n13, eq163_e2059_q_d_n14, eq163_e2059_q_d_n15, eq163_e2059_q_d_n16, eq163_e2059_q_d_n17, eq163_e2059_q_d_n18, eq163_e2059_q_d_n19, eq163_e2059_q_d_n20, eq163_e2059_q_d_n21, eq163_e2059_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_reactive_node_derivatives: [f64; 23] = [eq163_e2061_q_d_n0, eq163_e2061_q_d_n1, eq163_e2061_q_d_n2, eq163_e2061_q_d_n3, eq163_e2061_q_d_n4, eq163_e2061_q_d_n5, eq163_e2061_q_d_n6, eq163_e2061_q_d_n7, eq163_e2061_q_d_n8, eq163_e2061_q_d_n9, eq163_e2061_q_d_n10, eq163_e2061_q_d_n11, eq163_e2061_q_d_n12, eq163_e2061_q_d_n13, eq163_e2061_q_d_n14, eq163_e2061_q_d_n15, eq163_e2061_q_d_n16, eq163_e2061_q_d_n17, eq163_e2061_q_d_n18, eq163_e2061_q_d_n19, eq163_e2061_q_d_n20, eq163_e2061_q_d_n21, eq163_e2061_q_d_n22];
        let eq163_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq163_reactive_node_derivatives,
            branches,
            &eq163_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22, eq164_e2075_q, eq164_e2075_q_d_n0, eq164_e2075_q_d_n1, eq164_e2075_q_d_n2, eq164_e2075_q_d_n3, eq164_e2075_q_d_n4, eq164_e2075_q_d_n5, eq164_e2075_q_d_n6, eq164_e2075_q_d_n7, eq164_e2075_q_d_n8, eq164_e2075_q_d_n9, eq164_e2075_q_d_n10, eq164_e2075_q_d_n11, eq164_e2075_q_d_n12, eq164_e2075_q_d_n13, eq164_e2075_q_d_n14, eq164_e2075_q_d_n15, eq164_e2075_q_d_n16, eq164_e2075_q_d_n17, eq164_e2075_q_d_n18, eq164_e2075_q_d_n19, eq164_e2075_q_d_n20, eq164_e2075_q_d_n21, eq164_e2075_q_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq164_e2070_q: f64 = s.v[264];
        let eq164_e2071: f64 = (p.p7 * s.v[264]);
        let eq164_e2071_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq164_e2071_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq164_e2071_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq164_e2071_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq164_e2071_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq164_e2071_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq164_e2071_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq164_e2071_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq164_e2071_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq164_e2071_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq164_e2071_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq164_e2071_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq164_e2071_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq164_e2071_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq164_e2071_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq164_e2071_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq164_e2071_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq164_e2071_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq164_e2071_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq164_e2071_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq164_e2071_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq164_e2071_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq164_e2071_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq164_e2071_q: f64 = (p.p7 * eq164_e2070_q);
        let eq164_e2071_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq164_e2071_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq164_e2071_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq164_e2071_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq164_e2071_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq164_e2071_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq164_e2071_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq164_e2071_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq164_e2071_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq164_e2071_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq164_e2071_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq164_e2071_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq164_e2071_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq164_e2071_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq164_e2071_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq164_e2071_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq164_e2071_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq164_e2071_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq164_e2071_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq164_e2071_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq164_e2071_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq164_e2071_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq164_e2071_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (eq164_e2071_d_n0 * p.p247);
        let eq164_e2073_d_n1: f64 = (eq164_e2071_d_n1 * p.p247);
        let eq164_e2073_d_n2: f64 = (eq164_e2071_d_n2 * p.p247);
        let eq164_e2073_d_n3: f64 = (eq164_e2071_d_n3 * p.p247);
        let eq164_e2073_d_n4: f64 = (eq164_e2071_d_n4 * p.p247);
        let eq164_e2073_d_n5: f64 = (eq164_e2071_d_n5 * p.p247);
        let eq164_e2073_d_n6: f64 = (eq164_e2071_d_n6 * p.p247);
        let eq164_e2073_d_n7: f64 = (eq164_e2071_d_n7 * p.p247);
        let eq164_e2073_d_n8: f64 = (eq164_e2071_d_n8 * p.p247);
        let eq164_e2073_d_n9: f64 = (eq164_e2071_d_n9 * p.p247);
        let eq164_e2073_d_n10: f64 = (eq164_e2071_d_n10 * p.p247);
        let eq164_e2073_d_n11: f64 = (eq164_e2071_d_n11 * p.p247);
        let eq164_e2073_d_n12: f64 = (eq164_e2071_d_n12 * p.p247);
        let eq164_e2073_d_n13: f64 = (eq164_e2071_d_n13 * p.p247);
        let eq164_e2073_d_n14: f64 = (eq164_e2071_d_n14 * p.p247);
        let eq164_e2073_d_n15: f64 = (eq164_e2071_d_n15 * p.p247);
        let eq164_e2073_d_n16: f64 = (eq164_e2071_d_n16 * p.p247);
        let eq164_e2073_d_n17: f64 = (eq164_e2071_d_n17 * p.p247);
        let eq164_e2073_d_n18: f64 = (eq164_e2071_d_n18 * p.p247);
        let eq164_e2073_d_n19: f64 = (eq164_e2071_d_n19 * p.p247);
        let eq164_e2073_d_n20: f64 = (eq164_e2071_d_n20 * p.p247);
        let eq164_e2073_d_n21: f64 = (eq164_e2071_d_n21 * p.p247);
        let eq164_e2073_d_n22: f64 = (eq164_e2071_d_n22 * p.p247);
        let eq164_e2073_q: f64 = (eq164_e2071_q * p.p247);
        let eq164_e2073_q_d_n0: f64 = (eq164_e2071_q_d_n0 * p.p247);
        let eq164_e2073_q_d_n1: f64 = (eq164_e2071_q_d_n1 * p.p247);
        let eq164_e2073_q_d_n2: f64 = (eq164_e2071_q_d_n2 * p.p247);
        let eq164_e2073_q_d_n3: f64 = (eq164_e2071_q_d_n3 * p.p247);
        let eq164_e2073_q_d_n4: f64 = (eq164_e2071_q_d_n4 * p.p247);
        let eq164_e2073_q_d_n5: f64 = (eq164_e2071_q_d_n5 * p.p247);
        let eq164_e2073_q_d_n6: f64 = (eq164_e2071_q_d_n6 * p.p247);
        let eq164_e2073_q_d_n7: f64 = (eq164_e2071_q_d_n7 * p.p247);
        let eq164_e2073_q_d_n8: f64 = (eq164_e2071_q_d_n8 * p.p247);
        let eq164_e2073_q_d_n9: f64 = (eq164_e2071_q_d_n9 * p.p247);
        let eq164_e2073_q_d_n10: f64 = (eq164_e2071_q_d_n10 * p.p247);
        let eq164_e2073_q_d_n11: f64 = (eq164_e2071_q_d_n11 * p.p247);
        let eq164_e2073_q_d_n12: f64 = (eq164_e2071_q_d_n12 * p.p247);
        let eq164_e2073_q_d_n13: f64 = (eq164_e2071_q_d_n13 * p.p247);
        let eq164_e2073_q_d_n14: f64 = (eq164_e2071_q_d_n14 * p.p247);
        let eq164_e2073_q_d_n15: f64 = (eq164_e2071_q_d_n15 * p.p247);
        let eq164_e2073_q_d_n16: f64 = (eq164_e2071_q_d_n16 * p.p247);
        let eq164_e2073_q_d_n17: f64 = (eq164_e2071_q_d_n17 * p.p247);
        let eq164_e2073_q_d_n18: f64 = (eq164_e2071_q_d_n18 * p.p247);
        let eq164_e2073_q_d_n19: f64 = (eq164_e2071_q_d_n19 * p.p247);
        let eq164_e2073_q_d_n20: f64 = (eq164_e2071_q_d_n20 * p.p247);
        let eq164_e2073_q_d_n21: f64 = (eq164_e2071_q_d_n21 * p.p247);
        let eq164_e2073_q_d_n22: f64 = (eq164_e2071_q_d_n22 * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n10, eq164_e2073_d_n11, eq164_e2073_d_n12, eq164_e2073_d_n13, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22, eq164_e2073_q, eq164_e2073_q_d_n0, eq164_e2073_q_d_n1, eq164_e2073_q_d_n2, eq164_e2073_q_d_n3, eq164_e2073_q_d_n4, eq164_e2073_q_d_n5, eq164_e2073_q_d_n6, eq164_e2073_q_d_n7, eq164_e2073_q_d_n8, eq164_e2073_q_d_n9, eq164_e2073_q_d_n10, eq164_e2073_q_d_n11, eq164_e2073_q_d_n12, eq164_e2073_q_d_n13, eq164_e2073_q_d_n14, eq164_e2073_q_d_n15, eq164_e2073_q_d_n16, eq164_e2073_q_d_n17, eq164_e2073_q_d_n18, eq164_e2073_q_d_n19, eq164_e2073_q_d_n20, eq164_e2073_q_d_n21, eq164_e2073_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_reactive_node_derivatives: [f64; 23] = [eq164_e2075_q_d_n0, eq164_e2075_q_d_n1, eq164_e2075_q_d_n2, eq164_e2075_q_d_n3, eq164_e2075_q_d_n4, eq164_e2075_q_d_n5, eq164_e2075_q_d_n6, eq164_e2075_q_d_n7, eq164_e2075_q_d_n8, eq164_e2075_q_d_n9, eq164_e2075_q_d_n10, eq164_e2075_q_d_n11, eq164_e2075_q_d_n12, eq164_e2075_q_d_n13, eq164_e2075_q_d_n14, eq164_e2075_q_d_n15, eq164_e2075_q_d_n16, eq164_e2075_q_d_n17, eq164_e2075_q_d_n18, eq164_e2075_q_d_n19, eq164_e2075_q_d_n20, eq164_e2075_q_d_n21, eq164_e2075_q_d_n22];
        let eq164_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq164_reactive_node_derivatives,
            branches,
            &eq164_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22, eq165_e2088_q, eq165_e2088_q_d_n0, eq165_e2088_q_d_n1, eq165_e2088_q_d_n2, eq165_e2088_q_d_n3, eq165_e2088_q_d_n4, eq165_e2088_q_d_n5, eq165_e2088_q_d_n6, eq165_e2088_q_d_n7, eq165_e2088_q_d_n8, eq165_e2088_q_d_n9, eq165_e2088_q_d_n10, eq165_e2088_q_d_n11, eq165_e2088_q_d_n12, eq165_e2088_q_d_n13, eq165_e2088_q_d_n14, eq165_e2088_q_d_n15, eq165_e2088_q_d_n16, eq165_e2088_q_d_n17, eq165_e2088_q_d_n18, eq165_e2088_q_d_n19, eq165_e2088_q_d_n20, eq165_e2088_q_d_n21, eq165_e2088_q_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq165_e2085_q: f64 = s.v[264];
        let eq165_e2086: f64 = (p.p7 * s.v[264]);
        let eq165_e2086_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq165_e2086_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq165_e2086_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq165_e2086_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq165_e2086_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq165_e2086_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq165_e2086_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq165_e2086_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq165_e2086_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq165_e2086_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq165_e2086_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq165_e2086_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq165_e2086_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq165_e2086_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq165_e2086_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq165_e2086_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq165_e2086_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq165_e2086_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq165_e2086_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq165_e2086_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq165_e2086_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq165_e2086_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq165_e2086_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq165_e2086_q: f64 = (p.p7 * eq165_e2085_q);
        let eq165_e2086_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq165_e2086_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq165_e2086_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq165_e2086_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq165_e2086_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq165_e2086_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq165_e2086_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq165_e2086_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq165_e2086_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq165_e2086_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq165_e2086_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq165_e2086_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq165_e2086_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq165_e2086_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq165_e2086_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq165_e2086_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq165_e2086_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq165_e2086_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq165_e2086_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq165_e2086_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq165_e2086_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq165_e2086_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq165_e2086_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        (eq165_e2086, eq165_e2086_d_n0, eq165_e2086_d_n1, eq165_e2086_d_n2, eq165_e2086_d_n3, eq165_e2086_d_n4, eq165_e2086_d_n5, eq165_e2086_d_n6, eq165_e2086_d_n7, eq165_e2086_d_n8, eq165_e2086_d_n9, eq165_e2086_d_n10, eq165_e2086_d_n11, eq165_e2086_d_n12, eq165_e2086_d_n13, eq165_e2086_d_n14, eq165_e2086_d_n15, eq165_e2086_d_n16, eq165_e2086_d_n17, eq165_e2086_d_n18, eq165_e2086_d_n19, eq165_e2086_d_n20, eq165_e2086_d_n21, eq165_e2086_d_n22, eq165_e2086_q, eq165_e2086_q_d_n0, eq165_e2086_q_d_n1, eq165_e2086_q_d_n2, eq165_e2086_q_d_n3, eq165_e2086_q_d_n4, eq165_e2086_q_d_n5, eq165_e2086_q_d_n6, eq165_e2086_q_d_n7, eq165_e2086_q_d_n8, eq165_e2086_q_d_n9, eq165_e2086_q_d_n10, eq165_e2086_q_d_n11, eq165_e2086_q_d_n12, eq165_e2086_q_d_n13, eq165_e2086_q_d_n14, eq165_e2086_q_d_n15, eq165_e2086_q_d_n16, eq165_e2086_q_d_n17, eq165_e2086_q_d_n18, eq165_e2086_q_d_n19, eq165_e2086_q_d_n20, eq165_e2086_q_d_n21, eq165_e2086_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_reactive_node_derivatives: [f64; 23] = [eq165_e2088_q_d_n0, eq165_e2088_q_d_n1, eq165_e2088_q_d_n2, eq165_e2088_q_d_n3, eq165_e2088_q_d_n4, eq165_e2088_q_d_n5, eq165_e2088_q_d_n6, eq165_e2088_q_d_n7, eq165_e2088_q_d_n8, eq165_e2088_q_d_n9, eq165_e2088_q_d_n10, eq165_e2088_q_d_n11, eq165_e2088_q_d_n12, eq165_e2088_q_d_n13, eq165_e2088_q_d_n14, eq165_e2088_q_d_n15, eq165_e2088_q_d_n16, eq165_e2088_q_d_n17, eq165_e2088_q_d_n18, eq165_e2088_q_d_n19, eq165_e2088_q_d_n20, eq165_e2088_q_d_n21, eq165_e2088_q_d_n22];
        let eq165_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq165_reactive_node_derivatives,
            branches,
            &eq165_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22, eq166_e2103_q, eq166_e2103_q_d_n0, eq166_e2103_q_d_n1, eq166_e2103_q_d_n2, eq166_e2103_q_d_n3, eq166_e2103_q_d_n4, eq166_e2103_q_d_n5, eq166_e2103_q_d_n6, eq166_e2103_q_d_n7, eq166_e2103_q_d_n8, eq166_e2103_q_d_n9, eq166_e2103_q_d_n10, eq166_e2103_q_d_n11, eq166_e2103_q_d_n12, eq166_e2103_q_d_n13, eq166_e2103_q_d_n14, eq166_e2103_q_d_n15, eq166_e2103_q_d_n16, eq166_e2103_q_d_n17, eq166_e2103_q_d_n18, eq166_e2103_q_d_n19, eq166_e2103_q_d_n20, eq166_e2103_q_d_n21, eq166_e2103_q_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq166_e2098_q: f64 = s.v[264];
        let eq166_e2099: f64 = (p.p7 * s.v[264]);
        let eq166_e2099_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq166_e2099_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq166_e2099_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq166_e2099_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq166_e2099_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq166_e2099_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq166_e2099_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq166_e2099_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq166_e2099_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq166_e2099_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq166_e2099_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq166_e2099_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq166_e2099_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq166_e2099_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq166_e2099_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq166_e2099_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq166_e2099_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq166_e2099_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq166_e2099_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq166_e2099_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq166_e2099_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq166_e2099_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq166_e2099_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq166_e2099_q: f64 = (p.p7 * eq166_e2098_q);
        let eq166_e2099_q_d_n0: f64 = (p.p7 * s.dn[264][0]);
        let eq166_e2099_q_d_n1: f64 = (p.p7 * s.dn[264][1]);
        let eq166_e2099_q_d_n2: f64 = (p.p7 * s.dn[264][2]);
        let eq166_e2099_q_d_n3: f64 = (p.p7 * s.dn[264][3]);
        let eq166_e2099_q_d_n4: f64 = (p.p7 * s.dn[264][4]);
        let eq166_e2099_q_d_n5: f64 = (p.p7 * s.dn[264][5]);
        let eq166_e2099_q_d_n6: f64 = (p.p7 * s.dn[264][6]);
        let eq166_e2099_q_d_n7: f64 = (p.p7 * s.dn[264][7]);
        let eq166_e2099_q_d_n8: f64 = (p.p7 * s.dn[264][8]);
        let eq166_e2099_q_d_n9: f64 = (p.p7 * s.dn[264][9]);
        let eq166_e2099_q_d_n10: f64 = (p.p7 * s.dn[264][10]);
        let eq166_e2099_q_d_n11: f64 = (p.p7 * s.dn[264][11]);
        let eq166_e2099_q_d_n12: f64 = (p.p7 * s.dn[264][12]);
        let eq166_e2099_q_d_n13: f64 = (p.p7 * s.dn[264][13]);
        let eq166_e2099_q_d_n14: f64 = (p.p7 * s.dn[264][14]);
        let eq166_e2099_q_d_n15: f64 = (p.p7 * s.dn[264][15]);
        let eq166_e2099_q_d_n16: f64 = (p.p7 * s.dn[264][16]);
        let eq166_e2099_q_d_n17: f64 = (p.p7 * s.dn[264][17]);
        let eq166_e2099_q_d_n18: f64 = (p.p7 * s.dn[264][18]);
        let eq166_e2099_q_d_n19: f64 = (p.p7 * s.dn[264][19]);
        let eq166_e2099_q_d_n20: f64 = (p.p7 * s.dn[264][20]);
        let eq166_e2099_q_d_n21: f64 = (p.p7 * s.dn[264][21]);
        let eq166_e2099_q_d_n22: f64 = (p.p7 * s.dn[264][22]);
        let eq166_e2101: f64 = (eq166_e2099 * p.p247);
        let eq166_e2101_d_n0: f64 = (eq166_e2099_d_n0 * p.p247);
        let eq166_e2101_d_n1: f64 = (eq166_e2099_d_n1 * p.p247);
        let eq166_e2101_d_n2: f64 = (eq166_e2099_d_n2 * p.p247);
        let eq166_e2101_d_n3: f64 = (eq166_e2099_d_n3 * p.p247);
        let eq166_e2101_d_n4: f64 = (eq166_e2099_d_n4 * p.p247);
        let eq166_e2101_d_n5: f64 = (eq166_e2099_d_n5 * p.p247);
        let eq166_e2101_d_n6: f64 = (eq166_e2099_d_n6 * p.p247);
        let eq166_e2101_d_n7: f64 = (eq166_e2099_d_n7 * p.p247);
        let eq166_e2101_d_n8: f64 = (eq166_e2099_d_n8 * p.p247);
        let eq166_e2101_d_n9: f64 = (eq166_e2099_d_n9 * p.p247);
        let eq166_e2101_d_n10: f64 = (eq166_e2099_d_n10 * p.p247);
        let eq166_e2101_d_n11: f64 = (eq166_e2099_d_n11 * p.p247);
        let eq166_e2101_d_n12: f64 = (eq166_e2099_d_n12 * p.p247);
        let eq166_e2101_d_n13: f64 = (eq166_e2099_d_n13 * p.p247);
        let eq166_e2101_d_n14: f64 = (eq166_e2099_d_n14 * p.p247);
        let eq166_e2101_d_n15: f64 = (eq166_e2099_d_n15 * p.p247);
        let eq166_e2101_d_n16: f64 = (eq166_e2099_d_n16 * p.p247);
        let eq166_e2101_d_n17: f64 = (eq166_e2099_d_n17 * p.p247);
        let eq166_e2101_d_n18: f64 = (eq166_e2099_d_n18 * p.p247);
        let eq166_e2101_d_n19: f64 = (eq166_e2099_d_n19 * p.p247);
        let eq166_e2101_d_n20: f64 = (eq166_e2099_d_n20 * p.p247);
        let eq166_e2101_d_n21: f64 = (eq166_e2099_d_n21 * p.p247);
        let eq166_e2101_d_n22: f64 = (eq166_e2099_d_n22 * p.p247);
        let eq166_e2101_q: f64 = (eq166_e2099_q * p.p247);
        let eq166_e2101_q_d_n0: f64 = (eq166_e2099_q_d_n0 * p.p247);
        let eq166_e2101_q_d_n1: f64 = (eq166_e2099_q_d_n1 * p.p247);
        let eq166_e2101_q_d_n2: f64 = (eq166_e2099_q_d_n2 * p.p247);
        let eq166_e2101_q_d_n3: f64 = (eq166_e2099_q_d_n3 * p.p247);
        let eq166_e2101_q_d_n4: f64 = (eq166_e2099_q_d_n4 * p.p247);
        let eq166_e2101_q_d_n5: f64 = (eq166_e2099_q_d_n5 * p.p247);
        let eq166_e2101_q_d_n6: f64 = (eq166_e2099_q_d_n6 * p.p247);
        let eq166_e2101_q_d_n7: f64 = (eq166_e2099_q_d_n7 * p.p247);
        let eq166_e2101_q_d_n8: f64 = (eq166_e2099_q_d_n8 * p.p247);
        let eq166_e2101_q_d_n9: f64 = (eq166_e2099_q_d_n9 * p.p247);
        let eq166_e2101_q_d_n10: f64 = (eq166_e2099_q_d_n10 * p.p247);
        let eq166_e2101_q_d_n11: f64 = (eq166_e2099_q_d_n11 * p.p247);
        let eq166_e2101_q_d_n12: f64 = (eq166_e2099_q_d_n12 * p.p247);
        let eq166_e2101_q_d_n13: f64 = (eq166_e2099_q_d_n13 * p.p247);
        let eq166_e2101_q_d_n14: f64 = (eq166_e2099_q_d_n14 * p.p247);
        let eq166_e2101_q_d_n15: f64 = (eq166_e2099_q_d_n15 * p.p247);
        let eq166_e2101_q_d_n16: f64 = (eq166_e2099_q_d_n16 * p.p247);
        let eq166_e2101_q_d_n17: f64 = (eq166_e2099_q_d_n17 * p.p247);
        let eq166_e2101_q_d_n18: f64 = (eq166_e2099_q_d_n18 * p.p247);
        let eq166_e2101_q_d_n19: f64 = (eq166_e2099_q_d_n19 * p.p247);
        let eq166_e2101_q_d_n20: f64 = (eq166_e2099_q_d_n20 * p.p247);
        let eq166_e2101_q_d_n21: f64 = (eq166_e2099_q_d_n21 * p.p247);
        let eq166_e2101_q_d_n22: f64 = (eq166_e2099_q_d_n22 * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n10, eq166_e2101_d_n11, eq166_e2101_d_n12, eq166_e2101_d_n13, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22, eq166_e2101_q, eq166_e2101_q_d_n0, eq166_e2101_q_d_n1, eq166_e2101_q_d_n2, eq166_e2101_q_d_n3, eq166_e2101_q_d_n4, eq166_e2101_q_d_n5, eq166_e2101_q_d_n6, eq166_e2101_q_d_n7, eq166_e2101_q_d_n8, eq166_e2101_q_d_n9, eq166_e2101_q_d_n10, eq166_e2101_q_d_n11, eq166_e2101_q_d_n12, eq166_e2101_q_d_n13, eq166_e2101_q_d_n14, eq166_e2101_q_d_n15, eq166_e2101_q_d_n16, eq166_e2101_q_d_n17, eq166_e2101_q_d_n18, eq166_e2101_q_d_n19, eq166_e2101_q_d_n20, eq166_e2101_q_d_n21, eq166_e2101_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_reactive_node_derivatives: [f64; 23] = [eq166_e2103_q_d_n0, eq166_e2103_q_d_n1, eq166_e2103_q_d_n2, eq166_e2103_q_d_n3, eq166_e2103_q_d_n4, eq166_e2103_q_d_n5, eq166_e2103_q_d_n6, eq166_e2103_q_d_n7, eq166_e2103_q_d_n8, eq166_e2103_q_d_n9, eq166_e2103_q_d_n10, eq166_e2103_q_d_n11, eq166_e2103_q_d_n12, eq166_e2103_q_d_n13, eq166_e2103_q_d_n14, eq166_e2103_q_d_n15, eq166_e2103_q_d_n16, eq166_e2103_q_d_n17, eq166_e2103_q_d_n18, eq166_e2103_q_d_n19, eq166_e2103_q_d_n20, eq166_e2103_q_d_n21, eq166_e2103_q_d_n22];
        let eq166_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq166_reactive_node_derivatives,
            branches,
            &eq166_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22, eq167_e2115_q, eq167_e2115_q_d_n0, eq167_e2115_q_d_n1, eq167_e2115_q_d_n2, eq167_e2115_q_d_n3, eq167_e2115_q_d_n4, eq167_e2115_q_d_n5, eq167_e2115_q_d_n6, eq167_e2115_q_d_n7, eq167_e2115_q_d_n8, eq167_e2115_q_d_n9, eq167_e2115_q_d_n10, eq167_e2115_q_d_n11, eq167_e2115_q_d_n12, eq167_e2115_q_d_n13, eq167_e2115_q_d_n14, eq167_e2115_q_d_n15, eq167_e2115_q_d_n16, eq167_e2115_q_d_n17, eq167_e2115_q_d_n18, eq167_e2115_q_d_n19, eq167_e2115_q_d_n20, eq167_e2115_q_d_n21, eq167_e2115_q_d_n22,) = {
    if ((!s.b[585]) && s.b[588]) {
        let eq167_e2111: f64 = (p.p252 * s.v[264]);
        let eq167_e2111_d_n0: f64 = (p.p252 * s.dn[264][0]);
        let eq167_e2111_d_n1: f64 = (p.p252 * s.dn[264][1]);
        let eq167_e2111_d_n2: f64 = (p.p252 * s.dn[264][2]);
        let eq167_e2111_d_n3: f64 = (p.p252 * s.dn[264][3]);
        let eq167_e2111_d_n4: f64 = (p.p252 * s.dn[264][4]);
        let eq167_e2111_d_n5: f64 = (p.p252 * s.dn[264][5]);
        let eq167_e2111_d_n6: f64 = (p.p252 * s.dn[264][6]);
        let eq167_e2111_d_n7: f64 = (p.p252 * s.dn[264][7]);
        let eq167_e2111_d_n8: f64 = (p.p252 * s.dn[264][8]);
        let eq167_e2111_d_n9: f64 = (p.p252 * s.dn[264][9]);
        let eq167_e2111_d_n10: f64 = (p.p252 * s.dn[264][10]);
        let eq167_e2111_d_n11: f64 = (p.p252 * s.dn[264][11]);
        let eq167_e2111_d_n12: f64 = (p.p252 * s.dn[264][12]);
        let eq167_e2111_d_n13: f64 = (p.p252 * s.dn[264][13]);
        let eq167_e2111_d_n14: f64 = (p.p252 * s.dn[264][14]);
        let eq167_e2111_d_n15: f64 = (p.p252 * s.dn[264][15]);
        let eq167_e2111_d_n16: f64 = (p.p252 * s.dn[264][16]);
        let eq167_e2111_d_n17: f64 = (p.p252 * s.dn[264][17]);
        let eq167_e2111_d_n18: f64 = (p.p252 * s.dn[264][18]);
        let eq167_e2111_d_n19: f64 = (p.p252 * s.dn[264][19]);
        let eq167_e2111_d_n20: f64 = (p.p252 * s.dn[264][20]);
        let eq167_e2111_d_n21: f64 = (p.p252 * s.dn[264][21]);
        let eq167_e2111_d_n22: f64 = (p.p252 * s.dn[264][22]);
        let eq167_e2112_q: f64 = eq167_e2111;
        let eq167_e2113: f64 = (p.p7 * eq167_e2111);
        let eq167_e2113_d_n0: f64 = (p.p7 * eq167_e2111_d_n0);
        let eq167_e2113_d_n1: f64 = (p.p7 * eq167_e2111_d_n1);
        let eq167_e2113_d_n2: f64 = (p.p7 * eq167_e2111_d_n2);
        let eq167_e2113_d_n3: f64 = (p.p7 * eq167_e2111_d_n3);
        let eq167_e2113_d_n4: f64 = (p.p7 * eq167_e2111_d_n4);
        let eq167_e2113_d_n5: f64 = (p.p7 * eq167_e2111_d_n5);
        let eq167_e2113_d_n6: f64 = (p.p7 * eq167_e2111_d_n6);
        let eq167_e2113_d_n7: f64 = (p.p7 * eq167_e2111_d_n7);
        let eq167_e2113_d_n8: f64 = (p.p7 * eq167_e2111_d_n8);
        let eq167_e2113_d_n9: f64 = (p.p7 * eq167_e2111_d_n9);
        let eq167_e2113_d_n10: f64 = (p.p7 * eq167_e2111_d_n10);
        let eq167_e2113_d_n11: f64 = (p.p7 * eq167_e2111_d_n11);
        let eq167_e2113_d_n12: f64 = (p.p7 * eq167_e2111_d_n12);
        let eq167_e2113_d_n13: f64 = (p.p7 * eq167_e2111_d_n13);
        let eq167_e2113_d_n14: f64 = (p.p7 * eq167_e2111_d_n14);
        let eq167_e2113_d_n15: f64 = (p.p7 * eq167_e2111_d_n15);
        let eq167_e2113_d_n16: f64 = (p.p7 * eq167_e2111_d_n16);
        let eq167_e2113_d_n17: f64 = (p.p7 * eq167_e2111_d_n17);
        let eq167_e2113_d_n18: f64 = (p.p7 * eq167_e2111_d_n18);
        let eq167_e2113_d_n19: f64 = (p.p7 * eq167_e2111_d_n19);
        let eq167_e2113_d_n20: f64 = (p.p7 * eq167_e2111_d_n20);
        let eq167_e2113_d_n21: f64 = (p.p7 * eq167_e2111_d_n21);
        let eq167_e2113_d_n22: f64 = (p.p7 * eq167_e2111_d_n22);
        let eq167_e2113_q: f64 = (p.p7 * eq167_e2112_q);
        let eq167_e2113_q_d_n0: f64 = (p.p7 * eq167_e2111_d_n0);
        let eq167_e2113_q_d_n1: f64 = (p.p7 * eq167_e2111_d_n1);
        let eq167_e2113_q_d_n2: f64 = (p.p7 * eq167_e2111_d_n2);
        let eq167_e2113_q_d_n3: f64 = (p.p7 * eq167_e2111_d_n3);
        let eq167_e2113_q_d_n4: f64 = (p.p7 * eq167_e2111_d_n4);
        let eq167_e2113_q_d_n5: f64 = (p.p7 * eq167_e2111_d_n5);
        let eq167_e2113_q_d_n6: f64 = (p.p7 * eq167_e2111_d_n6);
        let eq167_e2113_q_d_n7: f64 = (p.p7 * eq167_e2111_d_n7);
        let eq167_e2113_q_d_n8: f64 = (p.p7 * eq167_e2111_d_n8);
        let eq167_e2113_q_d_n9: f64 = (p.p7 * eq167_e2111_d_n9);
        let eq167_e2113_q_d_n10: f64 = (p.p7 * eq167_e2111_d_n10);
        let eq167_e2113_q_d_n11: f64 = (p.p7 * eq167_e2111_d_n11);
        let eq167_e2113_q_d_n12: f64 = (p.p7 * eq167_e2111_d_n12);
        let eq167_e2113_q_d_n13: f64 = (p.p7 * eq167_e2111_d_n13);
        let eq167_e2113_q_d_n14: f64 = (p.p7 * eq167_e2111_d_n14);
        let eq167_e2113_q_d_n15: f64 = (p.p7 * eq167_e2111_d_n15);
        let eq167_e2113_q_d_n16: f64 = (p.p7 * eq167_e2111_d_n16);
        let eq167_e2113_q_d_n17: f64 = (p.p7 * eq167_e2111_d_n17);
        let eq167_e2113_q_d_n18: f64 = (p.p7 * eq167_e2111_d_n18);
        let eq167_e2113_q_d_n19: f64 = (p.p7 * eq167_e2111_d_n19);
        let eq167_e2113_q_d_n20: f64 = (p.p7 * eq167_e2111_d_n20);
        let eq167_e2113_q_d_n21: f64 = (p.p7 * eq167_e2111_d_n21);
        let eq167_e2113_q_d_n22: f64 = (p.p7 * eq167_e2111_d_n22);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n10, eq167_e2113_d_n11, eq167_e2113_d_n12, eq167_e2113_d_n13, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22, eq167_e2113_q, eq167_e2113_q_d_n0, eq167_e2113_q_d_n1, eq167_e2113_q_d_n2, eq167_e2113_q_d_n3, eq167_e2113_q_d_n4, eq167_e2113_q_d_n5, eq167_e2113_q_d_n6, eq167_e2113_q_d_n7, eq167_e2113_q_d_n8, eq167_e2113_q_d_n9, eq167_e2113_q_d_n10, eq167_e2113_q_d_n11, eq167_e2113_q_d_n12, eq167_e2113_q_d_n13, eq167_e2113_q_d_n14, eq167_e2113_q_d_n15, eq167_e2113_q_d_n16, eq167_e2113_q_d_n17, eq167_e2113_q_d_n18, eq167_e2113_q_d_n19, eq167_e2113_q_d_n20, eq167_e2113_q_d_n21, eq167_e2113_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_reactive_node_derivatives: [f64; 23] = [eq167_e2115_q_d_n0, eq167_e2115_q_d_n1, eq167_e2115_q_d_n2, eq167_e2115_q_d_n3, eq167_e2115_q_d_n4, eq167_e2115_q_d_n5, eq167_e2115_q_d_n6, eq167_e2115_q_d_n7, eq167_e2115_q_d_n8, eq167_e2115_q_d_n9, eq167_e2115_q_d_n10, eq167_e2115_q_d_n11, eq167_e2115_q_d_n12, eq167_e2115_q_d_n13, eq167_e2115_q_d_n14, eq167_e2115_q_d_n15, eq167_e2115_q_d_n16, eq167_e2115_q_d_n17, eq167_e2115_q_d_n18, eq167_e2115_q_d_n19, eq167_e2115_q_d_n20, eq167_e2115_q_d_n21, eq167_e2115_q_d_n22];
        let eq167_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq167_reactive_node_derivatives,
            branches,
            &eq167_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
