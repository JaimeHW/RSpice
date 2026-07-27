#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq31_e494, eq31_e494_d_n0, eq31_e494_d_n1, eq31_e494_d_n2, eq31_e494_d_n3, eq31_e494_d_n4, eq31_e494_d_n5, eq31_e494_d_n6, eq31_e494_d_n7, eq31_e494_d_n8, eq31_e494_d_n9, eq31_e494_d_n10, eq31_e494_d_n11, eq31_e494_d_n12, eq31_e494_d_n13, eq31_e494_d_n14, eq31_e494_d_n15, eq31_e494_d_n16, eq31_e494_d_n17, eq31_e494_d_n18, eq31_e494_d_b0, eq31_e494_d_b1, eq31_e494_d_b2, eq31_e494_d_b3, eq31_e494_d_b4, eq31_e494_d_b5, eq31_e494_d_b6, eq31_e494_d_b7, eq31_e494_d_b8, eq31_e494_d_b9, eq31_e494_d_b10, eq31_e494_d_b11, eq31_e494_d_b12, eq31_e494_d_b13, eq31_e494_d_b14, eq31_e494_d_b15, eq31_e494_q,) = {
    if s.b[1850] {
        let eq31_e491: f64 = (s.v[563] * (nv10 - 0.0));let eq31_e491_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));let eq31_e491_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));let eq31_e491_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));let eq31_e491_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));let eq31_e491_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));let eq31_e491_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));let eq31_e491_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));let eq31_e491_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));let eq31_e491_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));let eq31_e491_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));let eq31_e491_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);let eq31_e491_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));let eq31_e491_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));let eq31_e491_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));let eq31_e491_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));let eq31_e491_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));let eq31_e491_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));let eq31_e491_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));let eq31_e491_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));let eq31_e491_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));let eq31_e491_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));let eq31_e491_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));let eq31_e491_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));let eq31_e491_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));let eq31_e491_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));let eq31_e491_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));let eq31_e491_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));let eq31_e491_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));let eq31_e491_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));let eq31_e491_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));let eq31_e491_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));let eq31_e491_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));let eq31_e491_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));let eq31_e491_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));let eq31_e491_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));let eq31_e492_q: f64 = eq31_e491;
        (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15, eq31_e492_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e494_d_n0, eq31_e494_d_n1, eq31_e494_d_n2, eq31_e494_d_n3, eq31_e494_d_n4, eq31_e494_d_n5, eq31_e494_d_n6, eq31_e494_d_n7, eq31_e494_d_n8, eq31_e494_d_n9, eq31_e494_d_n10, eq31_e494_d_n11, eq31_e494_d_n12, eq31_e494_d_n13, eq31_e494_d_n14, eq31_e494_d_n15, eq31_e494_d_n16, eq31_e494_d_n17, eq31_e494_d_n18];let eq31_reactive_branch_derivatives: [f64; 16] = [eq31_e494_d_b0, eq31_e494_d_b1, eq31_e494_d_b2, eq31_e494_d_b3, eq31_e494_d_b4, eq31_e494_d_b5, eq31_e494_d_b6, eq31_e494_d_b7, eq31_e494_d_b8, eq31_e494_d_b9, eq31_e494_d_b10, eq31_e494_d_b11, eq31_e494_d_b12, eq31_e494_d_b13, eq31_e494_d_b14, eq31_e494_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(10),
            None,
            &eq31_reactive_node_derivatives,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq35_e526, eq35_e526_d_n0, eq35_e526_d_n1, eq35_e526_d_n2, eq35_e526_d_n3, eq35_e526_d_n4, eq35_e526_d_n5, eq35_e526_d_n6, eq35_e526_d_n7, eq35_e526_d_n8, eq35_e526_d_n9, eq35_e526_d_n10, eq35_e526_d_n11, eq35_e526_d_n12, eq35_e526_d_n13, eq35_e526_d_n14, eq35_e526_d_n15, eq35_e526_d_n16, eq35_e526_d_n17, eq35_e526_d_n18, eq35_e526_d_b0, eq35_e526_d_b1, eq35_e526_d_b2, eq35_e526_d_b3, eq35_e526_d_b4, eq35_e526_d_b5, eq35_e526_d_b6, eq35_e526_d_b7, eq35_e526_d_b8, eq35_e526_d_b9, eq35_e526_d_b10, eq35_e526_d_b11, eq35_e526_d_b12, eq35_e526_d_b13, eq35_e526_d_b14, eq35_e526_d_b15, eq35_e526_q, eq35_e526_q_d_n0, eq35_e526_q_d_n1, eq35_e526_q_d_n2, eq35_e526_q_d_n3, eq35_e526_q_d_n4, eq35_e526_q_d_n5, eq35_e526_q_d_n6, eq35_e526_q_d_n7, eq35_e526_q_d_n8, eq35_e526_q_d_n9, eq35_e526_q_d_n10, eq35_e526_q_d_n11, eq35_e526_q_d_n12, eq35_e526_q_d_n13, eq35_e526_q_d_n14, eq35_e526_q_d_n15, eq35_e526_q_d_n16, eq35_e526_q_d_n17, eq35_e526_q_d_n18, eq35_e526_q_d_b0, eq35_e526_q_d_b1, eq35_e526_q_d_b2, eq35_e526_q_d_b3, eq35_e526_q_d_b4, eq35_e526_q_d_b5, eq35_e526_q_d_b6, eq35_e526_q_d_b7, eq35_e526_q_d_b8, eq35_e526_q_d_b9, eq35_e526_q_d_b10, eq35_e526_q_d_b11, eq35_e526_q_d_b12, eq35_e526_q_d_b13, eq35_e526_q_d_b14, eq35_e526_q_d_b15,) = {
    if s.b[1851] {
        let eq35_e522_q: f64 = s.v[283];let eq35_e523: f64 = (s.v[281] + s.v[283]);let eq35_e523_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);let eq35_e523_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);let eq35_e523_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);let eq35_e523_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);let eq35_e523_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);let eq35_e523_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);let eq35_e523_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);let eq35_e523_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);let eq35_e523_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);let eq35_e523_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);let eq35_e523_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);let eq35_e523_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);let eq35_e523_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);let eq35_e523_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);let eq35_e523_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);let eq35_e523_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);let eq35_e523_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);let eq35_e523_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);let eq35_e523_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);let eq35_e523_d_b0: f64 = (s.db[281][0] + s.db[283][0]);let eq35_e523_d_b1: f64 = (s.db[281][1] + s.db[283][1]);let eq35_e523_d_b2: f64 = (s.db[281][2] + s.db[283][2]);let eq35_e523_d_b3: f64 = (s.db[281][3] + s.db[283][3]);let eq35_e523_d_b4: f64 = (s.db[281][4] + s.db[283][4]);let eq35_e523_d_b5: f64 = (s.db[281][5] + s.db[283][5]);let eq35_e523_d_b6: f64 = (s.db[281][6] + s.db[283][6]);let eq35_e523_d_b7: f64 = (s.db[281][7] + s.db[283][7]);let eq35_e523_d_b8: f64 = (s.db[281][8] + s.db[283][8]);let eq35_e523_d_b9: f64 = (s.db[281][9] + s.db[283][9]);let eq35_e523_d_b10: f64 = (s.db[281][10] + s.db[283][10]);let eq35_e523_d_b11: f64 = (s.db[281][11] + s.db[283][11]);let eq35_e523_d_b12: f64 = (s.db[281][12] + s.db[283][12]);let eq35_e523_d_b13: f64 = (s.db[281][13] + s.db[283][13]);let eq35_e523_d_b14: f64 = (s.db[281][14] + s.db[283][14]);let eq35_e523_d_b15: f64 = (s.db[281][15] + s.db[283][15]);let eq35_e523_q: f64 = eq35_e522_q;let eq35_e524: f64 = (p[50] * eq35_e523);let eq35_e524_d_n0: f64 = (p[50] * eq35_e523_d_n0);let eq35_e524_d_n1: f64 = (p[50] * eq35_e523_d_n1);let eq35_e524_d_n2: f64 = (p[50] * eq35_e523_d_n2);let eq35_e524_d_n3: f64 = (p[50] * eq35_e523_d_n3);let eq35_e524_d_n4: f64 = (p[50] * eq35_e523_d_n4);let eq35_e524_d_n5: f64 = (p[50] * eq35_e523_d_n5);let eq35_e524_d_n6: f64 = (p[50] * eq35_e523_d_n6);let eq35_e524_d_n7: f64 = (p[50] * eq35_e523_d_n7);let eq35_e524_d_n8: f64 = (p[50] * eq35_e523_d_n8);let eq35_e524_d_n9: f64 = (p[50] * eq35_e523_d_n9);let eq35_e524_d_n10: f64 = (p[50] * eq35_e523_d_n10);let eq35_e524_d_n11: f64 = (p[50] * eq35_e523_d_n11);let eq35_e524_d_n12: f64 = (p[50] * eq35_e523_d_n12);let eq35_e524_d_n13: f64 = (p[50] * eq35_e523_d_n13);let eq35_e524_d_n14: f64 = (p[50] * eq35_e523_d_n14);let eq35_e524_d_n15: f64 = (p[50] * eq35_e523_d_n15);let eq35_e524_d_n16: f64 = (p[50] * eq35_e523_d_n16);let eq35_e524_d_n17: f64 = (p[50] * eq35_e523_d_n17);let eq35_e524_d_n18: f64 = (p[50] * eq35_e523_d_n18);let eq35_e524_d_b0: f64 = (p[50] * eq35_e523_d_b0);let eq35_e524_d_b1: f64 = (p[50] * eq35_e523_d_b1);let eq35_e524_d_b2: f64 = (p[50] * eq35_e523_d_b2);let eq35_e524_d_b3: f64 = (p[50] * eq35_e523_d_b3);let eq35_e524_d_b4: f64 = (p[50] * eq35_e523_d_b4);let eq35_e524_d_b5: f64 = (p[50] * eq35_e523_d_b5);let eq35_e524_d_b6: f64 = (p[50] * eq35_e523_d_b6);let eq35_e524_d_b7: f64 = (p[50] * eq35_e523_d_b7);let eq35_e524_d_b8: f64 = (p[50] * eq35_e523_d_b8);let eq35_e524_d_b9: f64 = (p[50] * eq35_e523_d_b9);let eq35_e524_d_b10: f64 = (p[50] * eq35_e523_d_b10);let eq35_e524_d_b11: f64 = (p[50] * eq35_e523_d_b11);let eq35_e524_d_b12: f64 = (p[50] * eq35_e523_d_b12);let eq35_e524_d_b13: f64 = (p[50] * eq35_e523_d_b13);let eq35_e524_d_b14: f64 = (p[50] * eq35_e523_d_b14);let eq35_e524_d_b15: f64 = (p[50] * eq35_e523_d_b15);let eq35_e524_q: f64 = (p[50] * eq35_e523_q);
        (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_d_b12, eq35_e524_d_b13, eq35_e524_d_b14, eq35_e524_d_b15, eq35_e524_q, (p[50] * s.dn[283][0]), (p[50] * s.dn[283][1]), (p[50] * s.dn[283][2]), (p[50] * s.dn[283][3]), (p[50] * s.dn[283][4]), (p[50] * s.dn[283][5]), (p[50] * s.dn[283][6]), (p[50] * s.dn[283][7]), (p[50] * s.dn[283][8]), (p[50] * s.dn[283][9]), (p[50] * s.dn[283][10]), (p[50] * s.dn[283][11]), (p[50] * s.dn[283][12]), (p[50] * s.dn[283][13]), (p[50] * s.dn[283][14]), (p[50] * s.dn[283][15]), (p[50] * s.dn[283][16]), (p[50] * s.dn[283][17]), (p[50] * s.dn[283][18]), (p[50] * s.db[283][0]), (p[50] * s.db[283][1]), (p[50] * s.db[283][2]), (p[50] * s.db[283][3]), (p[50] * s.db[283][4]), (p[50] * s.db[283][5]), (p[50] * s.db[283][6]), (p[50] * s.db[283][7]), (p[50] * s.db[283][8]), (p[50] * s.db[283][9]), (p[50] * s.db[283][10]), (p[50] * s.db[283][11]), (p[50] * s.db[283][12]), (p[50] * s.db[283][13]), (p[50] * s.db[283][14]), (p[50] * s.db[283][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e526_q_d_n0, eq35_e526_q_d_n1, eq35_e526_q_d_n2, eq35_e526_q_d_n3, eq35_e526_q_d_n4, eq35_e526_q_d_n5, eq35_e526_q_d_n6, eq35_e526_q_d_n7, eq35_e526_q_d_n8, eq35_e526_q_d_n9, eq35_e526_q_d_n10, eq35_e526_q_d_n11, eq35_e526_q_d_n12, eq35_e526_q_d_n13, eq35_e526_q_d_n14, eq35_e526_q_d_n15, eq35_e526_q_d_n16, eq35_e526_q_d_n17, eq35_e526_q_d_n18];let eq35_reactive_branch_derivatives: [f64; 16] = [eq35_e526_q_d_b0, eq35_e526_q_d_b1, eq35_e526_q_d_b2, eq35_e526_q_d_b3, eq35_e526_q_d_b4, eq35_e526_q_d_b5, eq35_e526_q_d_b6, eq35_e526_q_d_b7, eq35_e526_q_d_b8, eq35_e526_q_d_b9, eq35_e526_q_d_b10, eq35_e526_q_d_b11, eq35_e526_q_d_b12, eq35_e526_q_d_b13, eq35_e526_q_d_b14, eq35_e526_q_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(9),
            Some(7),
            &eq35_reactive_node_derivatives,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);let nv17 = ctx.node_voltage(nodes[17]);let nv18 = ctx.node_voltage(nodes[18]);
        let (eq36_e535, eq36_e535_d_n0, eq36_e535_d_n1, eq36_e535_d_n2, eq36_e535_d_n3, eq36_e535_d_n4, eq36_e535_d_n5, eq36_e535_d_n6, eq36_e535_d_n7, eq36_e535_d_n8, eq36_e535_d_n9, eq36_e535_d_n10, eq36_e535_d_n11, eq36_e535_d_n12, eq36_e535_d_n13, eq36_e535_d_n14, eq36_e535_d_n15, eq36_e535_d_n16, eq36_e535_d_n17, eq36_e535_d_n18, eq36_e535_d_b0, eq36_e535_d_b1, eq36_e535_d_b2, eq36_e535_d_b3, eq36_e535_d_b4, eq36_e535_d_b5, eq36_e535_d_b6, eq36_e535_d_b7, eq36_e535_d_b8, eq36_e535_d_b9, eq36_e535_d_b10, eq36_e535_d_b11, eq36_e535_d_b12, eq36_e535_d_b13, eq36_e535_d_b14, eq36_e535_d_b15, eq36_e535_q, eq36_e535_q_d_n0, eq36_e535_q_d_n1, eq36_e535_q_d_n2, eq36_e535_q_d_n3, eq36_e535_q_d_n4, eq36_e535_q_d_n5, eq36_e535_q_d_n6, eq36_e535_q_d_n7, eq36_e535_q_d_n8, eq36_e535_q_d_n9, eq36_e535_q_d_n10, eq36_e535_q_d_n11, eq36_e535_q_d_n12, eq36_e535_q_d_n13, eq36_e535_q_d_n14, eq36_e535_q_d_n15, eq36_e535_q_d_n16, eq36_e535_q_d_n17, eq36_e535_q_d_n18, eq36_e535_q_d_b0, eq36_e535_q_d_b1, eq36_e535_q_d_b2, eq36_e535_q_d_b3, eq36_e535_q_d_b4, eq36_e535_q_d_b5, eq36_e535_q_d_b6, eq36_e535_q_d_b7, eq36_e535_q_d_b8, eq36_e535_q_d_b9, eq36_e535_q_d_b10, eq36_e535_q_d_b11, eq36_e535_q_d_b12, eq36_e535_q_d_b13, eq36_e535_q_d_b14, eq36_e535_q_d_b15,) = {
    if s.b[1851] {
        let eq36_e531_q: f64 = s.v[284];let eq36_e532: f64 = (s.v[282] + s.v[284]);let eq36_e532_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);let eq36_e532_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);let eq36_e532_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);let eq36_e532_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);let eq36_e532_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);let eq36_e532_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);let eq36_e532_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);let eq36_e532_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);let eq36_e532_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);let eq36_e532_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);let eq36_e532_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);let eq36_e532_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);let eq36_e532_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);let eq36_e532_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);let eq36_e532_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);let eq36_e532_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);let eq36_e532_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);let eq36_e532_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);let eq36_e532_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);let eq36_e532_d_b0: f64 = (s.db[282][0] + s.db[284][0]);let eq36_e532_d_b1: f64 = (s.db[282][1] + s.db[284][1]);let eq36_e532_d_b2: f64 = (s.db[282][2] + s.db[284][2]);let eq36_e532_d_b3: f64 = (s.db[282][3] + s.db[284][3]);let eq36_e532_d_b4: f64 = (s.db[282][4] + s.db[284][4]);let eq36_e532_d_b5: f64 = (s.db[282][5] + s.db[284][5]);let eq36_e532_d_b6: f64 = (s.db[282][6] + s.db[284][6]);let eq36_e532_d_b7: f64 = (s.db[282][7] + s.db[284][7]);let eq36_e532_d_b8: f64 = (s.db[282][8] + s.db[284][8]);let eq36_e532_d_b9: f64 = (s.db[282][9] + s.db[284][9]);let eq36_e532_d_b10: f64 = (s.db[282][10] + s.db[284][10]);let eq36_e532_d_b11: f64 = (s.db[282][11] + s.db[284][11]);let eq36_e532_d_b12: f64 = (s.db[282][12] + s.db[284][12]);let eq36_e532_d_b13: f64 = (s.db[282][13] + s.db[284][13]);let eq36_e532_d_b14: f64 = (s.db[282][14] + s.db[284][14]);let eq36_e532_d_b15: f64 = (s.db[282][15] + s.db[284][15]);let eq36_e532_q: f64 = eq36_e531_q;let eq36_e533: f64 = (p[50] * eq36_e532);let eq36_e533_d_n0: f64 = (p[50] * eq36_e532_d_n0);let eq36_e533_d_n1: f64 = (p[50] * eq36_e532_d_n1);let eq36_e533_d_n2: f64 = (p[50] * eq36_e532_d_n2);let eq36_e533_d_n3: f64 = (p[50] * eq36_e532_d_n3);let eq36_e533_d_n4: f64 = (p[50] * eq36_e532_d_n4);let eq36_e533_d_n5: f64 = (p[50] * eq36_e532_d_n5);let eq36_e533_d_n6: f64 = (p[50] * eq36_e532_d_n6);let eq36_e533_d_n7: f64 = (p[50] * eq36_e532_d_n7);let eq36_e533_d_n8: f64 = (p[50] * eq36_e532_d_n8);let eq36_e533_d_n9: f64 = (p[50] * eq36_e532_d_n9);let eq36_e533_d_n10: f64 = (p[50] * eq36_e532_d_n10);let eq36_e533_d_n11: f64 = (p[50] * eq36_e532_d_n11);let eq36_e533_d_n12: f64 = (p[50] * eq36_e532_d_n12);let eq36_e533_d_n13: f64 = (p[50] * eq36_e532_d_n13);let eq36_e533_d_n14: f64 = (p[50] * eq36_e532_d_n14);let eq36_e533_d_n15: f64 = (p[50] * eq36_e532_d_n15);let eq36_e533_d_n16: f64 = (p[50] * eq36_e532_d_n16);let eq36_e533_d_n17: f64 = (p[50] * eq36_e532_d_n17);let eq36_e533_d_n18: f64 = (p[50] * eq36_e532_d_n18);let eq36_e533_d_b0: f64 = (p[50] * eq36_e532_d_b0);let eq36_e533_d_b1: f64 = (p[50] * eq36_e532_d_b1);let eq36_e533_d_b2: f64 = (p[50] * eq36_e532_d_b2);let eq36_e533_d_b3: f64 = (p[50] * eq36_e532_d_b3);let eq36_e533_d_b4: f64 = (p[50] * eq36_e532_d_b4);let eq36_e533_d_b5: f64 = (p[50] * eq36_e532_d_b5);let eq36_e533_d_b6: f64 = (p[50] * eq36_e532_d_b6);let eq36_e533_d_b7: f64 = (p[50] * eq36_e532_d_b7);let eq36_e533_d_b8: f64 = (p[50] * eq36_e532_d_b8);let eq36_e533_d_b9: f64 = (p[50] * eq36_e532_d_b9);let eq36_e533_d_b10: f64 = (p[50] * eq36_e532_d_b10);let eq36_e533_d_b11: f64 = (p[50] * eq36_e532_d_b11);let eq36_e533_d_b12: f64 = (p[50] * eq36_e532_d_b12);let eq36_e533_d_b13: f64 = (p[50] * eq36_e532_d_b13);let eq36_e533_d_b14: f64 = (p[50] * eq36_e532_d_b14);let eq36_e533_d_b15: f64 = (p[50] * eq36_e532_d_b15);let eq36_e533_q: f64 = (p[50] * eq36_e532_q);
        (eq36_e533, eq36_e533_d_n0, eq36_e533_d_n1, eq36_e533_d_n2, eq36_e533_d_n3, eq36_e533_d_n4, eq36_e533_d_n5, eq36_e533_d_n6, eq36_e533_d_n7, eq36_e533_d_n8, eq36_e533_d_n9, eq36_e533_d_n10, eq36_e533_d_n11, eq36_e533_d_n12, eq36_e533_d_n13, eq36_e533_d_n14, eq36_e533_d_n15, eq36_e533_d_n16, eq36_e533_d_n17, eq36_e533_d_n18, eq36_e533_d_b0, eq36_e533_d_b1, eq36_e533_d_b2, eq36_e533_d_b3, eq36_e533_d_b4, eq36_e533_d_b5, eq36_e533_d_b6, eq36_e533_d_b7, eq36_e533_d_b8, eq36_e533_d_b9, eq36_e533_d_b10, eq36_e533_d_b11, eq36_e533_d_b12, eq36_e533_d_b13, eq36_e533_d_b14, eq36_e533_d_b15, eq36_e533_q, (p[50] * s.dn[284][0]), (p[50] * s.dn[284][1]), (p[50] * s.dn[284][2]), (p[50] * s.dn[284][3]), (p[50] * s.dn[284][4]), (p[50] * s.dn[284][5]), (p[50] * s.dn[284][6]), (p[50] * s.dn[284][7]), (p[50] * s.dn[284][8]), (p[50] * s.dn[284][9]), (p[50] * s.dn[284][10]), (p[50] * s.dn[284][11]), (p[50] * s.dn[284][12]), (p[50] * s.dn[284][13]), (p[50] * s.dn[284][14]), (p[50] * s.dn[284][15]), (p[50] * s.dn[284][16]), (p[50] * s.dn[284][17]), (p[50] * s.dn[284][18]), (p[50] * s.db[284][0]), (p[50] * s.db[284][1]), (p[50] * s.db[284][2]), (p[50] * s.db[284][3]), (p[50] * s.db[284][4]), (p[50] * s.db[284][5]), (p[50] * s.db[284][6]), (p[50] * s.db[284][7]), (p[50] * s.db[284][8]), (p[50] * s.db[284][9]), (p[50] * s.db[284][10]), (p[50] * s.db[284][11]), (p[50] * s.db[284][12]), (p[50] * s.db[284][13]), (p[50] * s.db[284][14]), (p[50] * s.db[284][15]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e535_q_d_n0, eq36_e535_q_d_n1, eq36_e535_q_d_n2, eq36_e535_q_d_n3, eq36_e535_q_d_n4, eq36_e535_q_d_n5, eq36_e535_q_d_n6, eq36_e535_q_d_n7, eq36_e535_q_d_n8, eq36_e535_q_d_n9, eq36_e535_q_d_n10, eq36_e535_q_d_n11, eq36_e535_q_d_n12, eq36_e535_q_d_n13, eq36_e535_q_d_n14, eq36_e535_q_d_n15, eq36_e535_q_d_n16, eq36_e535_q_d_n17, eq36_e535_q_d_n18];let eq36_reactive_branch_derivatives: [f64; 16] = [eq36_e535_q_d_b0, eq36_e535_q_d_b1, eq36_e535_q_d_b2, eq36_e535_q_d_b3, eq36_e535_q_d_b4, eq36_e535_q_d_b5, eq36_e535_q_d_b6, eq36_e535_q_d_b7, eq36_e535_q_d_b8, eq36_e535_q_d_b9, eq36_e535_q_d_b10, eq36_e535_q_d_b11, eq36_e535_q_d_b12, eq36_e535_q_d_b13, eq36_e535_q_d_b14, eq36_e535_q_d_b15];
        stamper.stamp_current_reactive_dense_local(
            Some(8),
            Some(6),
            &eq36_reactive_node_derivatives,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e619, eq47_e619_d_n18, eq47_e619_q,) = {
    if (s.b[1851] && (p[34] != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);let eq47_e616: f64 = (eq47_e614 * (nv18 - 0.0));let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(18),
            None,
            18,
            multiplicity * (eq47_e619_d_n18),
        );
        let (eq48_e630, eq48_e630_d_n13, eq48_e630_q,) = {
    if (s.b[1851] && (p[34] != 0.0)) {
        let eq48_e625: f64 = (1e-9 / 0.0001);let eq48_e627: f64 = (eq48_e625 * (nv13 - 0.0));let eq48_e628_q: f64 = eq48_e627;
        (eq48_e627, eq48_e625, eq48_e628_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq48_e630_d_n13),
        );
        let (eq53_e669, eq53_e669_d_n17, eq53_e669_q,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e664: f64 = (1e-9 / 0.0001);let eq53_e666: f64 = (eq53_e664 * (nv17 - 0.0));let eq53_e667_q: f64 = eq53_e666;
        (eq53_e666, eq53_e664, eq53_e667_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(17),
            None,
            17,
            multiplicity * (eq53_e669_d_n17),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);
        let (eq60_e727, eq60_e727_d_n17, eq60_e727_q,) = {
    if ((!s.b[1851]) && (p[37] != 0.0)) {
        let eq60_e722: f64 = (1e-9 / 0.0001);let eq60_e724: f64 = (eq60_e722 * (nv17 - 0.0));let eq60_e725_q: f64 = eq60_e724;
        (eq60_e724, eq60_e722, eq60_e725_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(17),
            None,
            17,
            multiplicity * (eq60_e727_d_n17),
        );
        let (eq68_e795, eq68_e795_d_n15, eq68_e795_q,) = {
    if ((!s.b[1851]) && (p[34] != 0.0)) {
        let eq68_e790: f64 = (1e-9 / 0.0001);let eq68_e792: f64 = (eq68_e790 * (nv15 - 0.0));let eq68_e793_q: f64 = eq68_e792;
        (eq68_e792, eq68_e790, eq68_e793_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(15),
            None,
            15,
            multiplicity * (eq68_e795_d_n15),
        );
        let (eq69_e807, eq69_e807_d_n16, eq69_e807_q,) = {
    if ((!s.b[1851]) && (p[34] != 0.0)) {
        let eq69_e802: f64 = (1e-9 / 0.0001);let eq69_e804: f64 = (eq69_e802 * (nv16 - 0.0));let eq69_e805_q: f64 = eq69_e804;
        (eq69_e804, eq69_e802, eq69_e805_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(16),
            None,
            16,
            multiplicity * (eq69_e807_d_n16),
        );
        let (eq70_e819, eq70_e819_d_n13, eq70_e819_q,) = {
    if ((!s.b[1851]) && (p[34] != 0.0)) {
        let eq70_e814: f64 = (1e-9 / 0.0001);let eq70_e816: f64 = (eq70_e814 * (nv13 - 0.0));let eq70_e817_q: f64 = eq70_e816;
        (eq70_e816, eq70_e814, eq70_e817_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq70_e819_d_n13),
        );
    }
}
