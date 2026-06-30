#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq0_e1018, eq0_e1018_d_n0, eq0_e1018_d_n1, eq0_e1018_d_n2, eq0_e1018_d_n3, eq0_e1018_d_n4, eq0_e1018_d_n5, eq0_e1018_d_n6, eq0_e1018_d_n7, eq0_e1018_d_n8, eq0_e1018_d_n9, eq0_e1018_d_n10, eq0_e1018_d_n11, eq0_e1018_d_n12, eq0_e1018_d_n13, eq0_e1018_d_n14, eq0_e1018_d_n15, eq0_e1018_d_n16, eq0_e1018_d_n17, eq0_e1018_d_b0, eq0_e1018_d_b1, eq0_e1018_d_b2, eq0_e1018_d_b3, eq0_e1018_d_b4, eq0_e1018_d_b5, eq0_e1018_d_b6, eq0_e1018_d_b7, eq0_e1018_d_b8, eq0_e1018_d_b9, eq0_e1018_d_b10, eq0_e1018_d_b11, eq0_e1018_q, eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17, eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8, eq0_e1018_q_d_b9, eq0_e1018_q_d_b10, eq0_e1018_q_d_b11,) = {
    if s.b[3305] {
        let eq0_e1015_q: f64 = s.v[924];
        let eq0_e1016: f64 = (s.v[926] + s.v[924]);
        let eq0_e1016_d_n0: f64 = (s.dn[926][0] + s.dn[924][0]);
        let eq0_e1016_d_n1: f64 = (s.dn[926][1] + s.dn[924][1]);
        let eq0_e1016_d_n2: f64 = (s.dn[926][2] + s.dn[924][2]);
        let eq0_e1016_d_n3: f64 = (s.dn[926][3] + s.dn[924][3]);
        let eq0_e1016_d_n4: f64 = (s.dn[926][4] + s.dn[924][4]);
        let eq0_e1016_d_n5: f64 = (s.dn[926][5] + s.dn[924][5]);
        let eq0_e1016_d_n6: f64 = (s.dn[926][6] + s.dn[924][6]);
        let eq0_e1016_d_n7: f64 = (s.dn[926][7] + s.dn[924][7]);
        let eq0_e1016_d_n8: f64 = (s.dn[926][8] + s.dn[924][8]);
        let eq0_e1016_d_n9: f64 = (s.dn[926][9] + s.dn[924][9]);
        let eq0_e1016_d_n10: f64 = (s.dn[926][10] + s.dn[924][10]);
        let eq0_e1016_d_n11: f64 = (s.dn[926][11] + s.dn[924][11]);
        let eq0_e1016_d_n12: f64 = (s.dn[926][12] + s.dn[924][12]);
        let eq0_e1016_d_n13: f64 = (s.dn[926][13] + s.dn[924][13]);
        let eq0_e1016_d_n14: f64 = (s.dn[926][14] + s.dn[924][14]);
        let eq0_e1016_d_n15: f64 = (s.dn[926][15] + s.dn[924][15]);
        let eq0_e1016_d_n16: f64 = (s.dn[926][16] + s.dn[924][16]);
        let eq0_e1016_d_n17: f64 = (s.dn[926][17] + s.dn[924][17]);
        let eq0_e1016_d_b0: f64 = (s.db[926][0] + s.db[924][0]);
        let eq0_e1016_d_b1: f64 = (s.db[926][1] + s.db[924][1]);
        let eq0_e1016_d_b2: f64 = (s.db[926][2] + s.db[924][2]);
        let eq0_e1016_d_b3: f64 = (s.db[926][3] + s.db[924][3]);
        let eq0_e1016_d_b4: f64 = (s.db[926][4] + s.db[924][4]);
        let eq0_e1016_d_b5: f64 = (s.db[926][5] + s.db[924][5]);
        let eq0_e1016_d_b6: f64 = (s.db[926][6] + s.db[924][6]);
        let eq0_e1016_d_b7: f64 = (s.db[926][7] + s.db[924][7]);
        let eq0_e1016_d_b8: f64 = (s.db[926][8] + s.db[924][8]);
        let eq0_e1016_d_b9: f64 = (s.db[926][9] + s.db[924][9]);
        let eq0_e1016_d_b10: f64 = (s.db[926][10] + s.db[924][10]);
        let eq0_e1016_d_b11: f64 = (s.db[926][11] + s.db[924][11]);
        let eq0_e1016_q: f64 = eq0_e1015_q;
        (eq0_e1016, eq0_e1016_d_n0, eq0_e1016_d_n1, eq0_e1016_d_n2, eq0_e1016_d_n3, eq0_e1016_d_n4, eq0_e1016_d_n5, eq0_e1016_d_n6, eq0_e1016_d_n7, eq0_e1016_d_n8, eq0_e1016_d_n9, eq0_e1016_d_n10, eq0_e1016_d_n11, eq0_e1016_d_n12, eq0_e1016_d_n13, eq0_e1016_d_n14, eq0_e1016_d_n15, eq0_e1016_d_n16, eq0_e1016_d_n17, eq0_e1016_d_b0, eq0_e1016_d_b1, eq0_e1016_d_b2, eq0_e1016_d_b3, eq0_e1016_d_b4, eq0_e1016_d_b5, eq0_e1016_d_b6, eq0_e1016_d_b7, eq0_e1016_d_b8, eq0_e1016_d_b9, eq0_e1016_d_b10, eq0_e1016_d_b11, eq0_e1016_q, s.dn[924][0], s.dn[924][1], s.dn[924][2], s.dn[924][3], s.dn[924][4], s.dn[924][5], s.dn[924][6], s.dn[924][7], s.dn[924][8], s.dn[924][9], s.dn[924][10], s.dn[924][11], s.dn[924][12], s.dn[924][13], s.dn[924][14], s.dn[924][15], s.dn[924][16], s.dn[924][17], s.db[924][0], s.db[924][1], s.db[924][2], s.db[924][3], s.db[924][4], s.db[924][5], s.db[924][6], s.db[924][7], s.db[924][8], s.db[924][9], s.db[924][10], s.db[924][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_reactive_node_derivatives: [f64; 18] = [eq0_e1018_q_d_n0, eq0_e1018_q_d_n1, eq0_e1018_q_d_n2, eq0_e1018_q_d_n3, eq0_e1018_q_d_n4, eq0_e1018_q_d_n5, eq0_e1018_q_d_n6, eq0_e1018_q_d_n7, eq0_e1018_q_d_n8, eq0_e1018_q_d_n9, eq0_e1018_q_d_n10, eq0_e1018_q_d_n11, eq0_e1018_q_d_n12, eq0_e1018_q_d_n13, eq0_e1018_q_d_n14, eq0_e1018_q_d_n15, eq0_e1018_q_d_n16, eq0_e1018_q_d_n17];
        let eq0_reactive_branch_derivatives: [f64; 12] = [eq0_e1018_q_d_b0, eq0_e1018_q_d_b1, eq0_e1018_q_d_b2, eq0_e1018_q_d_b3, eq0_e1018_q_d_b4, eq0_e1018_q_d_b5, eq0_e1018_q_d_b6, eq0_e1018_q_d_b7, eq0_e1018_q_d_b8, eq0_e1018_q_d_b9, eq0_e1018_q_d_b10, eq0_e1018_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq0_reactive_node_derivatives,
            branches,
            &eq0_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq1_e1025, eq1_e1025_d_n0, eq1_e1025_d_n1, eq1_e1025_d_n2, eq1_e1025_d_n3, eq1_e1025_d_n4, eq1_e1025_d_n5, eq1_e1025_d_n6, eq1_e1025_d_n7, eq1_e1025_d_n8, eq1_e1025_d_n9, eq1_e1025_d_n10, eq1_e1025_d_n11, eq1_e1025_d_n12, eq1_e1025_d_n13, eq1_e1025_d_n14, eq1_e1025_d_n15, eq1_e1025_d_n16, eq1_e1025_d_n17, eq1_e1025_d_b0, eq1_e1025_d_b1, eq1_e1025_d_b2, eq1_e1025_d_b3, eq1_e1025_d_b4, eq1_e1025_d_b5, eq1_e1025_d_b6, eq1_e1025_d_b7, eq1_e1025_d_b8, eq1_e1025_d_b9, eq1_e1025_d_b10, eq1_e1025_d_b11, eq1_e1025_q, eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17, eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8, eq1_e1025_q_d_b9, eq1_e1025_q_d_b10, eq1_e1025_q_d_b11,) = {
    if s.b[3305] {
        let eq1_e1022_q: f64 = s.v[925];
        let eq1_e1023: f64 = (s.v[927] + s.v[925]);
        let eq1_e1023_d_n0: f64 = (s.dn[927][0] + s.dn[925][0]);
        let eq1_e1023_d_n1: f64 = (s.dn[927][1] + s.dn[925][1]);
        let eq1_e1023_d_n2: f64 = (s.dn[927][2] + s.dn[925][2]);
        let eq1_e1023_d_n3: f64 = (s.dn[927][3] + s.dn[925][3]);
        let eq1_e1023_d_n4: f64 = (s.dn[927][4] + s.dn[925][4]);
        let eq1_e1023_d_n5: f64 = (s.dn[927][5] + s.dn[925][5]);
        let eq1_e1023_d_n6: f64 = (s.dn[927][6] + s.dn[925][6]);
        let eq1_e1023_d_n7: f64 = (s.dn[927][7] + s.dn[925][7]);
        let eq1_e1023_d_n8: f64 = (s.dn[927][8] + s.dn[925][8]);
        let eq1_e1023_d_n9: f64 = (s.dn[927][9] + s.dn[925][9]);
        let eq1_e1023_d_n10: f64 = (s.dn[927][10] + s.dn[925][10]);
        let eq1_e1023_d_n11: f64 = (s.dn[927][11] + s.dn[925][11]);
        let eq1_e1023_d_n12: f64 = (s.dn[927][12] + s.dn[925][12]);
        let eq1_e1023_d_n13: f64 = (s.dn[927][13] + s.dn[925][13]);
        let eq1_e1023_d_n14: f64 = (s.dn[927][14] + s.dn[925][14]);
        let eq1_e1023_d_n15: f64 = (s.dn[927][15] + s.dn[925][15]);
        let eq1_e1023_d_n16: f64 = (s.dn[927][16] + s.dn[925][16]);
        let eq1_e1023_d_n17: f64 = (s.dn[927][17] + s.dn[925][17]);
        let eq1_e1023_d_b0: f64 = (s.db[927][0] + s.db[925][0]);
        let eq1_e1023_d_b1: f64 = (s.db[927][1] + s.db[925][1]);
        let eq1_e1023_d_b2: f64 = (s.db[927][2] + s.db[925][2]);
        let eq1_e1023_d_b3: f64 = (s.db[927][3] + s.db[925][3]);
        let eq1_e1023_d_b4: f64 = (s.db[927][4] + s.db[925][4]);
        let eq1_e1023_d_b5: f64 = (s.db[927][5] + s.db[925][5]);
        let eq1_e1023_d_b6: f64 = (s.db[927][6] + s.db[925][6]);
        let eq1_e1023_d_b7: f64 = (s.db[927][7] + s.db[925][7]);
        let eq1_e1023_d_b8: f64 = (s.db[927][8] + s.db[925][8]);
        let eq1_e1023_d_b9: f64 = (s.db[927][9] + s.db[925][9]);
        let eq1_e1023_d_b10: f64 = (s.db[927][10] + s.db[925][10]);
        let eq1_e1023_d_b11: f64 = (s.db[927][11] + s.db[925][11]);
        let eq1_e1023_q: f64 = eq1_e1022_q;
        (eq1_e1023, eq1_e1023_d_n0, eq1_e1023_d_n1, eq1_e1023_d_n2, eq1_e1023_d_n3, eq1_e1023_d_n4, eq1_e1023_d_n5, eq1_e1023_d_n6, eq1_e1023_d_n7, eq1_e1023_d_n8, eq1_e1023_d_n9, eq1_e1023_d_n10, eq1_e1023_d_n11, eq1_e1023_d_n12, eq1_e1023_d_n13, eq1_e1023_d_n14, eq1_e1023_d_n15, eq1_e1023_d_n16, eq1_e1023_d_n17, eq1_e1023_d_b0, eq1_e1023_d_b1, eq1_e1023_d_b2, eq1_e1023_d_b3, eq1_e1023_d_b4, eq1_e1023_d_b5, eq1_e1023_d_b6, eq1_e1023_d_b7, eq1_e1023_d_b8, eq1_e1023_d_b9, eq1_e1023_d_b10, eq1_e1023_d_b11, eq1_e1023_q, s.dn[925][0], s.dn[925][1], s.dn[925][2], s.dn[925][3], s.dn[925][4], s.dn[925][5], s.dn[925][6], s.dn[925][7], s.dn[925][8], s.dn[925][9], s.dn[925][10], s.dn[925][11], s.dn[925][12], s.dn[925][13], s.dn[925][14], s.dn[925][15], s.dn[925][16], s.dn[925][17], s.db[925][0], s.db[925][1], s.db[925][2], s.db[925][3], s.db[925][4], s.db[925][5], s.db[925][6], s.db[925][7], s.db[925][8], s.db[925][9], s.db[925][10], s.db[925][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_reactive_node_derivatives: [f64; 18] = [eq1_e1025_q_d_n0, eq1_e1025_q_d_n1, eq1_e1025_q_d_n2, eq1_e1025_q_d_n3, eq1_e1025_q_d_n4, eq1_e1025_q_d_n5, eq1_e1025_q_d_n6, eq1_e1025_q_d_n7, eq1_e1025_q_d_n8, eq1_e1025_q_d_n9, eq1_e1025_q_d_n10, eq1_e1025_q_d_n11, eq1_e1025_q_d_n12, eq1_e1025_q_d_n13, eq1_e1025_q_d_n14, eq1_e1025_q_d_n15, eq1_e1025_q_d_n16, eq1_e1025_q_d_n17];
        let eq1_reactive_branch_derivatives: [f64; 12] = [eq1_e1025_q_d_b0, eq1_e1025_q_d_b1, eq1_e1025_q_d_b2, eq1_e1025_q_d_b3, eq1_e1025_q_d_b4, eq1_e1025_q_d_b5, eq1_e1025_q_d_b6, eq1_e1025_q_d_b7, eq1_e1025_q_d_b8, eq1_e1025_q_d_b9, eq1_e1025_q_d_b10, eq1_e1025_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[16]),
            None,
            nodes,
            &eq1_reactive_node_derivatives,
            branches,
            &eq1_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8, eq4_e1042_d_b9, eq4_e1042_d_b10, eq4_e1042_d_b11, eq4_e1042_q, eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17, eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8, eq4_e1042_q_d_b9, eq4_e1042_q_d_b10, eq4_e1042_q_d_b11,) = {
    if s.b[3306] {
        let eq4_e1039_q: f64 = s.v[931];
        let eq4_e1040: f64 = (s.v[932] + s.v[931]);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + s.dn[931][0]);
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + s.dn[931][1]);
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + s.dn[931][2]);
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + s.dn[931][3]);
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + s.dn[931][4]);
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + s.dn[931][5]);
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + s.dn[931][6]);
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + s.dn[931][7]);
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + s.dn[931][8]);
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + s.dn[931][9]);
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + s.dn[931][10]);
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + s.dn[931][11]);
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + s.dn[931][12]);
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + s.dn[931][13]);
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + s.dn[931][14]);
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + s.dn[931][15]);
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + s.dn[931][16]);
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + s.dn[931][17]);
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + s.db[931][0]);
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + s.db[931][1]);
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + s.db[931][2]);
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + s.db[931][3]);
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + s.db[931][4]);
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + s.db[931][5]);
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + s.db[931][6]);
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + s.db[931][7]);
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + s.db[931][8]);
        let eq4_e1040_d_b9: f64 = (s.db[932][9] + s.db[931][9]);
        let eq4_e1040_d_b10: f64 = (s.db[932][10] + s.db[931][10]);
        let eq4_e1040_d_b11: f64 = (s.db[932][11] + s.db[931][11]);
        let eq4_e1040_q: f64 = eq4_e1039_q;
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8, eq4_e1040_d_b9, eq4_e1040_d_b10, eq4_e1040_d_b11, eq4_e1040_q, s.dn[931][0], s.dn[931][1], s.dn[931][2], s.dn[931][3], s.dn[931][4], s.dn[931][5], s.dn[931][6], s.dn[931][7], s.dn[931][8], s.dn[931][9], s.dn[931][10], s.dn[931][11], s.dn[931][12], s.dn[931][13], s.dn[931][14], s.dn[931][15], s.dn[931][16], s.dn[931][17], s.db[931][0], s.db[931][1], s.db[931][2], s.db[931][3], s.db[931][4], s.db[931][5], s.db[931][6], s.db[931][7], s.db[931][8], s.db[931][9], s.db[931][10], s.db[931][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 18] = [eq4_e1042_q_d_n0, eq4_e1042_q_d_n1, eq4_e1042_q_d_n2, eq4_e1042_q_d_n3, eq4_e1042_q_d_n4, eq4_e1042_q_d_n5, eq4_e1042_q_d_n6, eq4_e1042_q_d_n7, eq4_e1042_q_d_n8, eq4_e1042_q_d_n9, eq4_e1042_q_d_n10, eq4_e1042_q_d_n11, eq4_e1042_q_d_n12, eq4_e1042_q_d_n13, eq4_e1042_q_d_n14, eq4_e1042_q_d_n15, eq4_e1042_q_d_n16, eq4_e1042_q_d_n17];
        let eq4_reactive_branch_derivatives: [f64; 12] = [eq4_e1042_q_d_b0, eq4_e1042_q_d_b1, eq4_e1042_q_d_b2, eq4_e1042_q_d_b3, eq4_e1042_q_d_b4, eq4_e1042_q_d_b5, eq4_e1042_q_d_b6, eq4_e1042_q_d_b7, eq4_e1042_q_d_b8, eq4_e1042_q_d_b9, eq4_e1042_q_d_b10, eq4_e1042_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            None,
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e1088_q: f64 = s.v[66];
        let eq14_e1089: f64 = (p.p87 * s.v[66]);
        let eq14_e1089_q: f64 = (p.p87 * eq14_e1088_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &s.dn[66],
            branches,
            &s.db[66],
            (multiplicity) * (p.p87),
        );
        let eq15_e1092_q: f64 = s.v[65];
        let eq15_e1093: f64 = (p.p87 * s.v[65]);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[0]),
            nodes,
            &s.dn[65],
            branches,
            &s.db[65],
            (multiplicity) * (p.p87),
        );
        let (eq18_e1112, eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17, eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11, eq18_e1112_q,) = {
    if s.b[3405] {
        let eq18_e1109_q: f64 = s.v[68];
        let eq18_e1110: f64 = (p.p87 * s.v[68]);
        let eq18_e1110_q: f64 = (p.p87 * eq18_e1109_q);
        (eq18_e1110, (p.p87 * s.dn[68][0]), (p.p87 * s.dn[68][1]), (p.p87 * s.dn[68][2]), (p.p87 * s.dn[68][3]), (p.p87 * s.dn[68][4]), (p.p87 * s.dn[68][5]), (p.p87 * s.dn[68][6]), (p.p87 * s.dn[68][7]), (p.p87 * s.dn[68][8]), (p.p87 * s.dn[68][9]), (p.p87 * s.dn[68][10]), (p.p87 * s.dn[68][11]), (p.p87 * s.dn[68][12]), (p.p87 * s.dn[68][13]), (p.p87 * s.dn[68][14]), (p.p87 * s.dn[68][15]), (p.p87 * s.dn[68][16]), (p.p87 * s.dn[68][17]), (p.p87 * s.db[68][0]), (p.p87 * s.db[68][1]), (p.p87 * s.db[68][2]), (p.p87 * s.db[68][3]), (p.p87 * s.db[68][4]), (p.p87 * s.db[68][5]), (p.p87 * s.db[68][6]), (p.p87 * s.db[68][7]), (p.p87 * s.db[68][8]), (p.p87 * s.db[68][9]), (p.p87 * s.db[68][10]), (p.p87 * s.db[68][11]), eq18_e1110_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 18] = [eq18_e1112_d_n0, eq18_e1112_d_n1, eq18_e1112_d_n2, eq18_e1112_d_n3, eq18_e1112_d_n4, eq18_e1112_d_n5, eq18_e1112_d_n6, eq18_e1112_d_n7, eq18_e1112_d_n8, eq18_e1112_d_n9, eq18_e1112_d_n10, eq18_e1112_d_n11, eq18_e1112_d_n12, eq18_e1112_d_n13, eq18_e1112_d_n14, eq18_e1112_d_n15, eq18_e1112_d_n16, eq18_e1112_d_n17];
        let eq18_reactive_branch_derivatives: [f64; 12] = [eq18_e1112_d_b0, eq18_e1112_d_b1, eq18_e1112_d_b2, eq18_e1112_d_b3, eq18_e1112_d_b4, eq18_e1112_d_b5, eq18_e1112_d_b6, eq18_e1112_d_b7, eq18_e1112_d_b8, eq18_e1112_d_b9, eq18_e1112_d_b10, eq18_e1112_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1119, eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17, eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11, eq19_e1119_q,) = {
    if s.b[3405] {
        let eq19_e1116_q: f64 = s.v[67];
        let eq19_e1117: f64 = (p.p87 * s.v[67]);
        let eq19_e1117_q: f64 = (p.p87 * eq19_e1116_q);
        (eq19_e1117, (p.p87 * s.dn[67][0]), (p.p87 * s.dn[67][1]), (p.p87 * s.dn[67][2]), (p.p87 * s.dn[67][3]), (p.p87 * s.dn[67][4]), (p.p87 * s.dn[67][5]), (p.p87 * s.dn[67][6]), (p.p87 * s.dn[67][7]), (p.p87 * s.dn[67][8]), (p.p87 * s.dn[67][9]), (p.p87 * s.dn[67][10]), (p.p87 * s.dn[67][11]), (p.p87 * s.dn[67][12]), (p.p87 * s.dn[67][13]), (p.p87 * s.dn[67][14]), (p.p87 * s.dn[67][15]), (p.p87 * s.dn[67][16]), (p.p87 * s.dn[67][17]), (p.p87 * s.db[67][0]), (p.p87 * s.db[67][1]), (p.p87 * s.db[67][2]), (p.p87 * s.db[67][3]), (p.p87 * s.db[67][4]), (p.p87 * s.db[67][5]), (p.p87 * s.db[67][6]), (p.p87 * s.db[67][7]), (p.p87 * s.db[67][8]), (p.p87 * s.db[67][9]), (p.p87 * s.db[67][10]), (p.p87 * s.db[67][11]), eq19_e1117_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 18] = [eq19_e1119_d_n0, eq19_e1119_d_n1, eq19_e1119_d_n2, eq19_e1119_d_n3, eq19_e1119_d_n4, eq19_e1119_d_n5, eq19_e1119_d_n6, eq19_e1119_d_n7, eq19_e1119_d_n8, eq19_e1119_d_n9, eq19_e1119_d_n10, eq19_e1119_d_n11, eq19_e1119_d_n12, eq19_e1119_d_n13, eq19_e1119_d_n14, eq19_e1119_d_n15, eq19_e1119_d_n16, eq19_e1119_d_n17];
        let eq19_reactive_branch_derivatives: [f64; 12] = [eq19_e1119_d_b0, eq19_e1119_d_b1, eq19_e1119_d_b2, eq19_e1119_d_b3, eq19_e1119_d_b4, eq19_e1119_d_b5, eq19_e1119_d_b6, eq19_e1119_d_b7, eq19_e1119_d_b8, eq19_e1119_d_b9, eq19_e1119_d_b10, eq19_e1119_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e1163: f64 = (s.v[18] + s.v[753]);
        let eq27_e1163_d_n0: f64 = (s.dn[18][0] + s.dn[753][0]);
        let eq27_e1163_d_n1: f64 = (s.dn[18][1] + s.dn[753][1]);
        let eq27_e1163_d_n2: f64 = (s.dn[18][2] + s.dn[753][2]);
        let eq27_e1163_d_n3: f64 = (s.dn[18][3] + s.dn[753][3]);
        let eq27_e1163_d_n4: f64 = (s.dn[18][4] + s.dn[753][4]);
        let eq27_e1163_d_n5: f64 = (s.dn[18][5] + s.dn[753][5]);
        let eq27_e1163_d_n6: f64 = (s.dn[18][6] + s.dn[753][6]);
        let eq27_e1163_d_n7: f64 = (s.dn[18][7] + s.dn[753][7]);
        let eq27_e1163_d_n8: f64 = (s.dn[18][8] + s.dn[753][8]);
        let eq27_e1163_d_n9: f64 = (s.dn[18][9] + s.dn[753][9]);
        let eq27_e1163_d_n10: f64 = (s.dn[18][10] + s.dn[753][10]);
        let eq27_e1163_d_n11: f64 = (s.dn[18][11] + s.dn[753][11]);
        let eq27_e1163_d_n12: f64 = (s.dn[18][12] + s.dn[753][12]);
        let eq27_e1163_d_n13: f64 = (s.dn[18][13] + s.dn[753][13]);
        let eq27_e1163_d_n14: f64 = (s.dn[18][14] + s.dn[753][14]);
        let eq27_e1163_d_n15: f64 = (s.dn[18][15] + s.dn[753][15]);
        let eq27_e1163_d_n16: f64 = (s.dn[18][16] + s.dn[753][16]);
        let eq27_e1163_d_n17: f64 = (s.dn[18][17] + s.dn[753][17]);
        let eq27_e1163_d_b0: f64 = (s.db[18][0] + s.db[753][0]);
        let eq27_e1163_d_b1: f64 = (s.db[18][1] + s.db[753][1]);
        let eq27_e1163_d_b2: f64 = (s.db[18][2] + s.db[753][2]);
        let eq27_e1163_d_b3: f64 = (s.db[18][3] + s.db[753][3]);
        let eq27_e1163_d_b4: f64 = (s.db[18][4] + s.db[753][4]);
        let eq27_e1163_d_b5: f64 = (s.db[18][5] + s.db[753][5]);
        let eq27_e1163_d_b6: f64 = (s.db[18][6] + s.db[753][6]);
        let eq27_e1163_d_b7: f64 = (s.db[18][7] + s.db[753][7]);
        let eq27_e1163_d_b8: f64 = (s.db[18][8] + s.db[753][8]);
        let eq27_e1163_d_b9: f64 = (s.db[18][9] + s.db[753][9]);
        let eq27_e1163_d_b10: f64 = (s.db[18][10] + s.db[753][10]);
        let eq27_e1163_d_b11: f64 = (s.db[18][11] + s.db[753][11]);
        let eq27_e1164_q: f64 = eq27_e1163;
        let eq27_e1165: f64 = (p.p87 * eq27_e1163);
        let eq27_e1165_d_n0: f64 = (p.p87 * eq27_e1163_d_n0);
        let eq27_e1165_d_n1: f64 = (p.p87 * eq27_e1163_d_n1);
        let eq27_e1165_d_n2: f64 = (p.p87 * eq27_e1163_d_n2);
        let eq27_e1165_d_n3: f64 = (p.p87 * eq27_e1163_d_n3);
        let eq27_e1165_d_n4: f64 = (p.p87 * eq27_e1163_d_n4);
        let eq27_e1165_d_n5: f64 = (p.p87 * eq27_e1163_d_n5);
        let eq27_e1165_d_n6: f64 = (p.p87 * eq27_e1163_d_n6);
        let eq27_e1165_d_n7: f64 = (p.p87 * eq27_e1163_d_n7);
        let eq27_e1165_d_n8: f64 = (p.p87 * eq27_e1163_d_n8);
        let eq27_e1165_d_n9: f64 = (p.p87 * eq27_e1163_d_n9);
        let eq27_e1165_d_n10: f64 = (p.p87 * eq27_e1163_d_n10);
        let eq27_e1165_d_n11: f64 = (p.p87 * eq27_e1163_d_n11);
        let eq27_e1165_d_n12: f64 = (p.p87 * eq27_e1163_d_n12);
        let eq27_e1165_d_n13: f64 = (p.p87 * eq27_e1163_d_n13);
        let eq27_e1165_d_n14: f64 = (p.p87 * eq27_e1163_d_n14);
        let eq27_e1165_d_n15: f64 = (p.p87 * eq27_e1163_d_n15);
        let eq27_e1165_d_n16: f64 = (p.p87 * eq27_e1163_d_n16);
        let eq27_e1165_d_n17: f64 = (p.p87 * eq27_e1163_d_n17);
        let eq27_e1165_d_b0: f64 = (p.p87 * eq27_e1163_d_b0);
        let eq27_e1165_d_b1: f64 = (p.p87 * eq27_e1163_d_b1);
        let eq27_e1165_d_b2: f64 = (p.p87 * eq27_e1163_d_b2);
        let eq27_e1165_d_b3: f64 = (p.p87 * eq27_e1163_d_b3);
        let eq27_e1165_d_b4: f64 = (p.p87 * eq27_e1163_d_b4);
        let eq27_e1165_d_b5: f64 = (p.p87 * eq27_e1163_d_b5);
        let eq27_e1165_d_b6: f64 = (p.p87 * eq27_e1163_d_b6);
        let eq27_e1165_d_b7: f64 = (p.p87 * eq27_e1163_d_b7);
        let eq27_e1165_d_b8: f64 = (p.p87 * eq27_e1163_d_b8);
        let eq27_e1165_d_b9: f64 = (p.p87 * eq27_e1163_d_b9);
        let eq27_e1165_d_b10: f64 = (p.p87 * eq27_e1163_d_b10);
        let eq27_e1165_d_b11: f64 = (p.p87 * eq27_e1163_d_b11);
        let eq27_e1165_q: f64 = (p.p87 * eq27_e1164_q);
        let eq27_reactive_node_derivatives: [f64; 18] = [eq27_e1165_d_n0, eq27_e1165_d_n1, eq27_e1165_d_n2, eq27_e1165_d_n3, eq27_e1165_d_n4, eq27_e1165_d_n5, eq27_e1165_d_n6, eq27_e1165_d_n7, eq27_e1165_d_n8, eq27_e1165_d_n9, eq27_e1165_d_n10, eq27_e1165_d_n11, eq27_e1165_d_n12, eq27_e1165_d_n13, eq27_e1165_d_n14, eq27_e1165_d_n15, eq27_e1165_d_n16, eq27_e1165_d_n17];
        let eq27_reactive_branch_derivatives: [f64; 12] = [eq27_e1165_d_b0, eq27_e1165_d_b1, eq27_e1165_d_b2, eq27_e1165_d_b3, eq27_e1165_d_b4, eq27_e1165_d_b5, eq27_e1165_d_b6, eq27_e1165_d_b7, eq27_e1165_d_b8, eq27_e1165_d_b9, eq27_e1165_d_b10, eq27_e1165_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1169: f64 = (s.v[19] + s.v[751]);
        let eq28_e1169_d_n0: f64 = (s.dn[19][0] + s.dn[751][0]);
        let eq28_e1169_d_n1: f64 = (s.dn[19][1] + s.dn[751][1]);
        let eq28_e1169_d_n2: f64 = (s.dn[19][2] + s.dn[751][2]);
        let eq28_e1169_d_n3: f64 = (s.dn[19][3] + s.dn[751][3]);
        let eq28_e1169_d_n4: f64 = (s.dn[19][4] + s.dn[751][4]);
        let eq28_e1169_d_n5: f64 = (s.dn[19][5] + s.dn[751][5]);
        let eq28_e1169_d_n6: f64 = (s.dn[19][6] + s.dn[751][6]);
        let eq28_e1169_d_n7: f64 = (s.dn[19][7] + s.dn[751][7]);
        let eq28_e1169_d_n8: f64 = (s.dn[19][8] + s.dn[751][8]);
        let eq28_e1169_d_n9: f64 = (s.dn[19][9] + s.dn[751][9]);
        let eq28_e1169_d_n10: f64 = (s.dn[19][10] + s.dn[751][10]);
        let eq28_e1169_d_n11: f64 = (s.dn[19][11] + s.dn[751][11]);
        let eq28_e1169_d_n12: f64 = (s.dn[19][12] + s.dn[751][12]);
        let eq28_e1169_d_n13: f64 = (s.dn[19][13] + s.dn[751][13]);
        let eq28_e1169_d_n14: f64 = (s.dn[19][14] + s.dn[751][14]);
        let eq28_e1169_d_n15: f64 = (s.dn[19][15] + s.dn[751][15]);
        let eq28_e1169_d_n16: f64 = (s.dn[19][16] + s.dn[751][16]);
        let eq28_e1169_d_n17: f64 = (s.dn[19][17] + s.dn[751][17]);
        let eq28_e1169_d_b0: f64 = (s.db[19][0] + s.db[751][0]);
        let eq28_e1169_d_b1: f64 = (s.db[19][1] + s.db[751][1]);
        let eq28_e1169_d_b2: f64 = (s.db[19][2] + s.db[751][2]);
        let eq28_e1169_d_b3: f64 = (s.db[19][3] + s.db[751][3]);
        let eq28_e1169_d_b4: f64 = (s.db[19][4] + s.db[751][4]);
        let eq28_e1169_d_b5: f64 = (s.db[19][5] + s.db[751][5]);
        let eq28_e1169_d_b6: f64 = (s.db[19][6] + s.db[751][6]);
        let eq28_e1169_d_b7: f64 = (s.db[19][7] + s.db[751][7]);
        let eq28_e1169_d_b8: f64 = (s.db[19][8] + s.db[751][8]);
        let eq28_e1169_d_b9: f64 = (s.db[19][9] + s.db[751][9]);
        let eq28_e1169_d_b10: f64 = (s.db[19][10] + s.db[751][10]);
        let eq28_e1169_d_b11: f64 = (s.db[19][11] + s.db[751][11]);
        let eq28_e1170_q: f64 = eq28_e1169;
        let eq28_e1171: f64 = (p.p87 * eq28_e1169);
        let eq28_e1171_d_n0: f64 = (p.p87 * eq28_e1169_d_n0);
        let eq28_e1171_d_n1: f64 = (p.p87 * eq28_e1169_d_n1);
        let eq28_e1171_d_n2: f64 = (p.p87 * eq28_e1169_d_n2);
        let eq28_e1171_d_n3: f64 = (p.p87 * eq28_e1169_d_n3);
        let eq28_e1171_d_n4: f64 = (p.p87 * eq28_e1169_d_n4);
        let eq28_e1171_d_n5: f64 = (p.p87 * eq28_e1169_d_n5);
        let eq28_e1171_d_n6: f64 = (p.p87 * eq28_e1169_d_n6);
        let eq28_e1171_d_n7: f64 = (p.p87 * eq28_e1169_d_n7);
        let eq28_e1171_d_n8: f64 = (p.p87 * eq28_e1169_d_n8);
        let eq28_e1171_d_n9: f64 = (p.p87 * eq28_e1169_d_n9);
        let eq28_e1171_d_n10: f64 = (p.p87 * eq28_e1169_d_n10);
        let eq28_e1171_d_n11: f64 = (p.p87 * eq28_e1169_d_n11);
        let eq28_e1171_d_n12: f64 = (p.p87 * eq28_e1169_d_n12);
        let eq28_e1171_d_n13: f64 = (p.p87 * eq28_e1169_d_n13);
        let eq28_e1171_d_n14: f64 = (p.p87 * eq28_e1169_d_n14);
        let eq28_e1171_d_n15: f64 = (p.p87 * eq28_e1169_d_n15);
        let eq28_e1171_d_n16: f64 = (p.p87 * eq28_e1169_d_n16);
        let eq28_e1171_d_n17: f64 = (p.p87 * eq28_e1169_d_n17);
        let eq28_e1171_d_b0: f64 = (p.p87 * eq28_e1169_d_b0);
        let eq28_e1171_d_b1: f64 = (p.p87 * eq28_e1169_d_b1);
        let eq28_e1171_d_b2: f64 = (p.p87 * eq28_e1169_d_b2);
        let eq28_e1171_d_b3: f64 = (p.p87 * eq28_e1169_d_b3);
        let eq28_e1171_d_b4: f64 = (p.p87 * eq28_e1169_d_b4);
        let eq28_e1171_d_b5: f64 = (p.p87 * eq28_e1169_d_b5);
        let eq28_e1171_d_b6: f64 = (p.p87 * eq28_e1169_d_b6);
        let eq28_e1171_d_b7: f64 = (p.p87 * eq28_e1169_d_b7);
        let eq28_e1171_d_b8: f64 = (p.p87 * eq28_e1169_d_b8);
        let eq28_e1171_d_b9: f64 = (p.p87 * eq28_e1169_d_b9);
        let eq28_e1171_d_b10: f64 = (p.p87 * eq28_e1169_d_b10);
        let eq28_e1171_d_b11: f64 = (p.p87 * eq28_e1169_d_b11);
        let eq28_e1171_q: f64 = (p.p87 * eq28_e1170_q);
        let eq28_reactive_node_derivatives: [f64; 18] = [eq28_e1171_d_n0, eq28_e1171_d_n1, eq28_e1171_d_n2, eq28_e1171_d_n3, eq28_e1171_d_n4, eq28_e1171_d_n5, eq28_e1171_d_n6, eq28_e1171_d_n7, eq28_e1171_d_n8, eq28_e1171_d_n9, eq28_e1171_d_n10, eq28_e1171_d_n11, eq28_e1171_d_n12, eq28_e1171_d_n13, eq28_e1171_d_n14, eq28_e1171_d_n15, eq28_e1171_d_n16, eq28_e1171_d_n17];
        let eq28_reactive_branch_derivatives: [f64; 12] = [eq28_e1171_d_b0, eq28_e1171_d_b1, eq28_e1171_d_b2, eq28_e1171_d_b3, eq28_e1171_d_b4, eq28_e1171_d_b5, eq28_e1171_d_b6, eq28_e1171_d_b7, eq28_e1171_d_b8, eq28_e1171_d_b9, eq28_e1171_d_b10, eq28_e1171_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1176: f64 = (s.v[753] + s.v[751]);
        let eq29_e1176_d_n0: f64 = (s.dn[753][0] + s.dn[751][0]);
        let eq29_e1176_d_n1: f64 = (s.dn[753][1] + s.dn[751][1]);
        let eq29_e1176_d_n2: f64 = (s.dn[753][2] + s.dn[751][2]);
        let eq29_e1176_d_n3: f64 = (s.dn[753][3] + s.dn[751][3]);
        let eq29_e1176_d_n4: f64 = (s.dn[753][4] + s.dn[751][4]);
        let eq29_e1176_d_n5: f64 = (s.dn[753][5] + s.dn[751][5]);
        let eq29_e1176_d_n6: f64 = (s.dn[753][6] + s.dn[751][6]);
        let eq29_e1176_d_n7: f64 = (s.dn[753][7] + s.dn[751][7]);
        let eq29_e1176_d_n8: f64 = (s.dn[753][8] + s.dn[751][8]);
        let eq29_e1176_d_n9: f64 = (s.dn[753][9] + s.dn[751][9]);
        let eq29_e1176_d_n10: f64 = (s.dn[753][10] + s.dn[751][10]);
        let eq29_e1176_d_n11: f64 = (s.dn[753][11] + s.dn[751][11]);
        let eq29_e1176_d_n12: f64 = (s.dn[753][12] + s.dn[751][12]);
        let eq29_e1176_d_n13: f64 = (s.dn[753][13] + s.dn[751][13]);
        let eq29_e1176_d_n14: f64 = (s.dn[753][14] + s.dn[751][14]);
        let eq29_e1176_d_n15: f64 = (s.dn[753][15] + s.dn[751][15]);
        let eq29_e1176_d_n16: f64 = (s.dn[753][16] + s.dn[751][16]);
        let eq29_e1176_d_n17: f64 = (s.dn[753][17] + s.dn[751][17]);
        let eq29_e1176_d_b0: f64 = (s.db[753][0] + s.db[751][0]);
        let eq29_e1176_d_b1: f64 = (s.db[753][1] + s.db[751][1]);
        let eq29_e1176_d_b2: f64 = (s.db[753][2] + s.db[751][2]);
        let eq29_e1176_d_b3: f64 = (s.db[753][3] + s.db[751][3]);
        let eq29_e1176_d_b4: f64 = (s.db[753][4] + s.db[751][4]);
        let eq29_e1176_d_b5: f64 = (s.db[753][5] + s.db[751][5]);
        let eq29_e1176_d_b6: f64 = (s.db[753][6] + s.db[751][6]);
        let eq29_e1176_d_b7: f64 = (s.db[753][7] + s.db[751][7]);
        let eq29_e1176_d_b8: f64 = (s.db[753][8] + s.db[751][8]);
        let eq29_e1176_d_b9: f64 = (s.db[753][9] + s.db[751][9]);
        let eq29_e1176_d_b10: f64 = (s.db[753][10] + s.db[751][10]);
        let eq29_e1176_d_b11: f64 = (s.db[753][11] + s.db[751][11]);
        let eq29_e1178: f64 = (eq29_e1176 + s.v[752]);
        let eq29_e1178_d_n0: f64 = (eq29_e1176_d_n0 + s.dn[752][0]);
        let eq29_e1178_d_n1: f64 = (eq29_e1176_d_n1 + s.dn[752][1]);
        let eq29_e1178_d_n2: f64 = (eq29_e1176_d_n2 + s.dn[752][2]);
        let eq29_e1178_d_n3: f64 = (eq29_e1176_d_n3 + s.dn[752][3]);
        let eq29_e1178_d_n4: f64 = (eq29_e1176_d_n4 + s.dn[752][4]);
        let eq29_e1178_d_n5: f64 = (eq29_e1176_d_n5 + s.dn[752][5]);
        let eq29_e1178_d_n6: f64 = (eq29_e1176_d_n6 + s.dn[752][6]);
        let eq29_e1178_d_n7: f64 = (eq29_e1176_d_n7 + s.dn[752][7]);
        let eq29_e1178_d_n8: f64 = (eq29_e1176_d_n8 + s.dn[752][8]);
        let eq29_e1178_d_n9: f64 = (eq29_e1176_d_n9 + s.dn[752][9]);
        let eq29_e1178_d_n10: f64 = (eq29_e1176_d_n10 + s.dn[752][10]);
        let eq29_e1178_d_n11: f64 = (eq29_e1176_d_n11 + s.dn[752][11]);
        let eq29_e1178_d_n12: f64 = (eq29_e1176_d_n12 + s.dn[752][12]);
        let eq29_e1178_d_n13: f64 = (eq29_e1176_d_n13 + s.dn[752][13]);
        let eq29_e1178_d_n14: f64 = (eq29_e1176_d_n14 + s.dn[752][14]);
        let eq29_e1178_d_n15: f64 = (eq29_e1176_d_n15 + s.dn[752][15]);
        let eq29_e1178_d_n16: f64 = (eq29_e1176_d_n16 + s.dn[752][16]);
        let eq29_e1178_d_n17: f64 = (eq29_e1176_d_n17 + s.dn[752][17]);
        let eq29_e1178_d_b0: f64 = (eq29_e1176_d_b0 + s.db[752][0]);
        let eq29_e1178_d_b1: f64 = (eq29_e1176_d_b1 + s.db[752][1]);
        let eq29_e1178_d_b2: f64 = (eq29_e1176_d_b2 + s.db[752][2]);
        let eq29_e1178_d_b3: f64 = (eq29_e1176_d_b3 + s.db[752][3]);
        let eq29_e1178_d_b4: f64 = (eq29_e1176_d_b4 + s.db[752][4]);
        let eq29_e1178_d_b5: f64 = (eq29_e1176_d_b5 + s.db[752][5]);
        let eq29_e1178_d_b6: f64 = (eq29_e1176_d_b6 + s.db[752][6]);
        let eq29_e1178_d_b7: f64 = (eq29_e1176_d_b7 + s.db[752][7]);
        let eq29_e1178_d_b8: f64 = (eq29_e1176_d_b8 + s.db[752][8]);
        let eq29_e1178_d_b9: f64 = (eq29_e1176_d_b9 + s.db[752][9]);
        let eq29_e1178_d_b10: f64 = (eq29_e1176_d_b10 + s.db[752][10]);
        let eq29_e1178_d_b11: f64 = (eq29_e1176_d_b11 + s.db[752][11]);
        let eq29_e1179: f64 = (s.v[20] - eq29_e1178);
        let eq29_e1179_d_n0: f64 = (s.dn[20][0] - eq29_e1178_d_n0);
        let eq29_e1179_d_n1: f64 = (s.dn[20][1] - eq29_e1178_d_n1);
        let eq29_e1179_d_n2: f64 = (s.dn[20][2] - eq29_e1178_d_n2);
        let eq29_e1179_d_n3: f64 = (s.dn[20][3] - eq29_e1178_d_n3);
        let eq29_e1179_d_n4: f64 = (s.dn[20][4] - eq29_e1178_d_n4);
        let eq29_e1179_d_n5: f64 = (s.dn[20][5] - eq29_e1178_d_n5);
        let eq29_e1179_d_n6: f64 = (s.dn[20][6] - eq29_e1178_d_n6);
        let eq29_e1179_d_n7: f64 = (s.dn[20][7] - eq29_e1178_d_n7);
        let eq29_e1179_d_n8: f64 = (s.dn[20][8] - eq29_e1178_d_n8);
        let eq29_e1179_d_n9: f64 = (s.dn[20][9] - eq29_e1178_d_n9);
        let eq29_e1179_d_n10: f64 = (s.dn[20][10] - eq29_e1178_d_n10);
        let eq29_e1179_d_n11: f64 = (s.dn[20][11] - eq29_e1178_d_n11);
        let eq29_e1179_d_n12: f64 = (s.dn[20][12] - eq29_e1178_d_n12);
        let eq29_e1179_d_n13: f64 = (s.dn[20][13] - eq29_e1178_d_n13);
        let eq29_e1179_d_n14: f64 = (s.dn[20][14] - eq29_e1178_d_n14);
        let eq29_e1179_d_n15: f64 = (s.dn[20][15] - eq29_e1178_d_n15);
        let eq29_e1179_d_n16: f64 = (s.dn[20][16] - eq29_e1178_d_n16);
        let eq29_e1179_d_n17: f64 = (s.dn[20][17] - eq29_e1178_d_n17);
        let eq29_e1179_d_b0: f64 = (s.db[20][0] - eq29_e1178_d_b0);
        let eq29_e1179_d_b1: f64 = (s.db[20][1] - eq29_e1178_d_b1);
        let eq29_e1179_d_b2: f64 = (s.db[20][2] - eq29_e1178_d_b2);
        let eq29_e1179_d_b3: f64 = (s.db[20][3] - eq29_e1178_d_b3);
        let eq29_e1179_d_b4: f64 = (s.db[20][4] - eq29_e1178_d_b4);
        let eq29_e1179_d_b5: f64 = (s.db[20][5] - eq29_e1178_d_b5);
        let eq29_e1179_d_b6: f64 = (s.db[20][6] - eq29_e1178_d_b6);
        let eq29_e1179_d_b7: f64 = (s.db[20][7] - eq29_e1178_d_b7);
        let eq29_e1179_d_b8: f64 = (s.db[20][8] - eq29_e1178_d_b8);
        let eq29_e1179_d_b9: f64 = (s.db[20][9] - eq29_e1178_d_b9);
        let eq29_e1179_d_b10: f64 = (s.db[20][10] - eq29_e1178_d_b10);
        let eq29_e1179_d_b11: f64 = (s.db[20][11] - eq29_e1178_d_b11);
        let eq29_e1180_q: f64 = eq29_e1179;
        let eq29_e1181: f64 = (p.p87 * eq29_e1179);
        let eq29_e1181_d_n0: f64 = (p.p87 * eq29_e1179_d_n0);
        let eq29_e1181_d_n1: f64 = (p.p87 * eq29_e1179_d_n1);
        let eq29_e1181_d_n2: f64 = (p.p87 * eq29_e1179_d_n2);
        let eq29_e1181_d_n3: f64 = (p.p87 * eq29_e1179_d_n3);
        let eq29_e1181_d_n4: f64 = (p.p87 * eq29_e1179_d_n4);
        let eq29_e1181_d_n5: f64 = (p.p87 * eq29_e1179_d_n5);
        let eq29_e1181_d_n6: f64 = (p.p87 * eq29_e1179_d_n6);
        let eq29_e1181_d_n7: f64 = (p.p87 * eq29_e1179_d_n7);
        let eq29_e1181_d_n8: f64 = (p.p87 * eq29_e1179_d_n8);
        let eq29_e1181_d_n9: f64 = (p.p87 * eq29_e1179_d_n9);
        let eq29_e1181_d_n10: f64 = (p.p87 * eq29_e1179_d_n10);
        let eq29_e1181_d_n11: f64 = (p.p87 * eq29_e1179_d_n11);
        let eq29_e1181_d_n12: f64 = (p.p87 * eq29_e1179_d_n12);
        let eq29_e1181_d_n13: f64 = (p.p87 * eq29_e1179_d_n13);
        let eq29_e1181_d_n14: f64 = (p.p87 * eq29_e1179_d_n14);
        let eq29_e1181_d_n15: f64 = (p.p87 * eq29_e1179_d_n15);
        let eq29_e1181_d_n16: f64 = (p.p87 * eq29_e1179_d_n16);
        let eq29_e1181_d_n17: f64 = (p.p87 * eq29_e1179_d_n17);
        let eq29_e1181_d_b0: f64 = (p.p87 * eq29_e1179_d_b0);
        let eq29_e1181_d_b1: f64 = (p.p87 * eq29_e1179_d_b1);
        let eq29_e1181_d_b2: f64 = (p.p87 * eq29_e1179_d_b2);
        let eq29_e1181_d_b3: f64 = (p.p87 * eq29_e1179_d_b3);
        let eq29_e1181_d_b4: f64 = (p.p87 * eq29_e1179_d_b4);
        let eq29_e1181_d_b5: f64 = (p.p87 * eq29_e1179_d_b5);
        let eq29_e1181_d_b6: f64 = (p.p87 * eq29_e1179_d_b6);
        let eq29_e1181_d_b7: f64 = (p.p87 * eq29_e1179_d_b7);
        let eq29_e1181_d_b8: f64 = (p.p87 * eq29_e1179_d_b8);
        let eq29_e1181_d_b9: f64 = (p.p87 * eq29_e1179_d_b9);
        let eq29_e1181_d_b10: f64 = (p.p87 * eq29_e1179_d_b10);
        let eq29_e1181_d_b11: f64 = (p.p87 * eq29_e1179_d_b11);
        let eq29_e1181_q: f64 = (p.p87 * eq29_e1180_q);
        let eq29_reactive_node_derivatives: [f64; 18] = [eq29_e1181_d_n0, eq29_e1181_d_n1, eq29_e1181_d_n2, eq29_e1181_d_n3, eq29_e1181_d_n4, eq29_e1181_d_n5, eq29_e1181_d_n6, eq29_e1181_d_n7, eq29_e1181_d_n8, eq29_e1181_d_n9, eq29_e1181_d_n10, eq29_e1181_d_n11, eq29_e1181_d_n12, eq29_e1181_d_n13, eq29_e1181_d_n14, eq29_e1181_d_n15, eq29_e1181_d_n16, eq29_e1181_d_n17];
        let eq29_reactive_branch_derivatives: [f64; 12] = [eq29_e1181_d_b0, eq29_e1181_d_b1, eq29_e1181_d_b2, eq29_e1181_d_b3, eq29_e1181_d_b4, eq29_e1181_d_b5, eq29_e1181_d_b6, eq29_e1181_d_b7, eq29_e1181_d_b8, eq29_e1181_d_b9, eq29_e1181_d_b10, eq29_e1181_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq30_e1184_q: f64 = s.v[743];
        let eq30_e1185: f64 = (p.p87 * s.v[743]);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &s.dn[743],
            branches,
            &s.db[743],
            (multiplicity) * (p.p87),
        );
        let eq31_e1188_q: f64 = s.v[742];
        let eq31_e1189: f64 = (p.p87 * s.v[742]);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &s.dn[742],
            branches,
            &s.db[742],
            (multiplicity) * (p.p87),
        );
        let eq32_e1192_q: f64 = s.v[744];
        let eq32_e1193: f64 = (p.p87 * s.v[744]);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &s.dn[744],
            branches,
            &s.db[744],
            (multiplicity) * (p.p87),
        );
        let eq33_e1195: f64 = (-p.p87);
        let eq33_e1197_q: f64 = s.v[299];
        let eq33_e1198: f64 = (eq33_e1195 * s.v[299]);
        let eq33_e1198_q: f64 = (eq33_e1195 * eq33_e1197_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[0]),
            nodes,
            &s.dn[299],
            branches,
            &s.db[299],
            (multiplicity) * (eq33_e1195),
        );
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202_q: f64 = s.v[301];
        let eq34_e1203: f64 = (eq34_e1200 * s.v[301]);
        let eq34_e1203_q: f64 = (eq34_e1200 * eq34_e1202_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            nodes,
            &s.dn[301],
            branches,
            &s.db[301],
            (multiplicity) * (eq34_e1200),
        );
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1232_d_b9: f64 = ((nv14 - 0.0) * s.db[954][9]);
        let eq40_e1232_d_b10: f64 = ((nv14 - 0.0) * s.db[954][10]);
        let eq40_e1232_d_b11: f64 = ((nv14 - 0.0) * s.db[954][11]);
        let eq40_e1233_q: f64 = eq40_e1232;
        let eq40_reactive_node_derivatives: [f64; 18] = [eq40_e1232_d_n0, eq40_e1232_d_n1, eq40_e1232_d_n2, eq40_e1232_d_n3, eq40_e1232_d_n4, eq40_e1232_d_n5, eq40_e1232_d_n6, eq40_e1232_d_n7, eq40_e1232_d_n8, eq40_e1232_d_n9, eq40_e1232_d_n10, eq40_e1232_d_n11, eq40_e1232_d_n12, eq40_e1232_d_n13, eq40_e1232_d_n14, eq40_e1232_d_n15, eq40_e1232_d_n16, eq40_e1232_d_n17];
        let eq40_reactive_branch_derivatives: [f64; 12] = [eq40_e1232_d_b0, eq40_e1232_d_b1, eq40_e1232_d_b2, eq40_e1232_d_b3, eq40_e1232_d_b4, eq40_e1232_d_b5, eq40_e1232_d_b6, eq40_e1232_d_b7, eq40_e1232_d_b8, eq40_e1232_d_b9, eq40_e1232_d_b10, eq40_e1232_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1236_d_b9: f64 = ((nv14 - 0.0) * s.db[955][9]);
        let eq41_e1236_d_b10: f64 = ((nv14 - 0.0) * s.db[955][10]);
        let eq41_e1236_d_b11: f64 = ((nv14 - 0.0) * s.db[955][11]);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 18] = [eq41_e1236_d_n0, eq41_e1236_d_n1, eq41_e1236_d_n2, eq41_e1236_d_n3, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, eq41_e1236_d_n12, eq41_e1236_d_n13, eq41_e1236_d_n14, eq41_e1236_d_n15, eq41_e1236_d_n16, eq41_e1236_d_n17];
        let eq41_reactive_branch_derivatives: [f64; 12] = [eq41_e1236_d_b0, eq41_e1236_d_b1, eq41_e1236_d_b2, eq41_e1236_d_b3, eq41_e1236_d_b4, eq41_e1236_d_b5, eq41_e1236_d_b6, eq41_e1236_d_b7, eq41_e1236_d_b8, eq41_e1236_d_b9, eq41_e1236_d_b10, eq41_e1236_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1358, eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17, eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11, eq61_e1358_q,) = {
    if (p.p28 != 0.0) {
        let eq61_e1355: f64 = (s.v[800] * (nv11 - 0.0));
        let eq61_e1355_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);
        let eq61_e1355_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));
        let eq61_e1355_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));
        let eq61_e1355_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));
        let eq61_e1355_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));
        let eq61_e1355_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));
        let eq61_e1355_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));
        let eq61_e1355_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));
        let eq61_e1355_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));
        let eq61_e1355_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));
        let eq61_e1355_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));
        let eq61_e1355_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));
        let eq61_e1355_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));
        let eq61_e1355_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));
        let eq61_e1355_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));
        let eq61_e1355_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));
        let eq61_e1355_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));
        let eq61_e1355_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));
        let eq61_e1355_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));
        let eq61_e1356_q: f64 = eq61_e1355;
        (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n1, eq61_e1355_d_n2, eq61_e1355_d_n3, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n12, eq61_e1355_d_n13, eq61_e1355_d_n14, eq61_e1355_d_n15, eq61_e1355_d_n16, eq61_e1355_d_n17, eq61_e1355_d_b0, eq61_e1355_d_b1, eq61_e1355_d_b2, eq61_e1355_d_b3, eq61_e1355_d_b4, eq61_e1355_d_b5, eq61_e1355_d_b6, eq61_e1355_d_b7, eq61_e1355_d_b8, eq61_e1355_d_b9, eq61_e1355_d_b10, eq61_e1355_d_b11, eq61_e1356_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 18] = [eq61_e1358_d_n0, eq61_e1358_d_n1, eq61_e1358_d_n2, eq61_e1358_d_n3, eq61_e1358_d_n4, eq61_e1358_d_n5, eq61_e1358_d_n6, eq61_e1358_d_n7, eq61_e1358_d_n8, eq61_e1358_d_n9, eq61_e1358_d_n10, eq61_e1358_d_n11, eq61_e1358_d_n12, eq61_e1358_d_n13, eq61_e1358_d_n14, eq61_e1358_d_n15, eq61_e1358_d_n16, eq61_e1358_d_n17];
        let eq61_reactive_branch_derivatives: [f64; 12] = [eq61_e1358_d_b0, eq61_e1358_d_b1, eq61_e1358_d_b2, eq61_e1358_d_b3, eq61_e1358_d_b4, eq61_e1358_d_b5, eq61_e1358_d_b6, eq61_e1358_d_b7, eq61_e1358_d_b8, eq61_e1358_d_b9, eq61_e1358_d_b10, eq61_e1358_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            None,
            nodes,
            &eq61_reactive_node_derivatives,
            branches,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1365, eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17, eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11, eq62_e1365_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1362: f64 = (s.v[801] * (nv12 - 0.0));
        let eq62_e1362_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));
        let eq62_e1362_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);
        let eq62_e1362_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));
        let eq62_e1362_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));
        let eq62_e1362_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));
        let eq62_e1362_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));
        let eq62_e1362_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));
        let eq62_e1362_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));
        let eq62_e1362_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));
        let eq62_e1362_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));
        let eq62_e1362_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));
        let eq62_e1362_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));
        let eq62_e1362_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));
        let eq62_e1362_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));
        let eq62_e1362_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));
        let eq62_e1362_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));
        let eq62_e1362_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));
        let eq62_e1362_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));
        let eq62_e1362_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));
        let eq62_e1363_q: f64 = eq62_e1362;
        (eq62_e1362, eq62_e1362_d_n0, eq62_e1362_d_n1, eq62_e1362_d_n2, eq62_e1362_d_n3, eq62_e1362_d_n4, eq62_e1362_d_n5, eq62_e1362_d_n6, eq62_e1362_d_n7, eq62_e1362_d_n8, eq62_e1362_d_n9, eq62_e1362_d_n10, eq62_e1362_d_n11, eq62_e1362_d_n12, eq62_e1362_d_n13, eq62_e1362_d_n14, eq62_e1362_d_n15, eq62_e1362_d_n16, eq62_e1362_d_n17, eq62_e1362_d_b0, eq62_e1362_d_b1, eq62_e1362_d_b2, eq62_e1362_d_b3, eq62_e1362_d_b4, eq62_e1362_d_b5, eq62_e1362_d_b6, eq62_e1362_d_b7, eq62_e1362_d_b8, eq62_e1362_d_b9, eq62_e1362_d_b10, eq62_e1362_d_b11, eq62_e1363_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 18] = [eq62_e1365_d_n0, eq62_e1365_d_n1, eq62_e1365_d_n2, eq62_e1365_d_n3, eq62_e1365_d_n4, eq62_e1365_d_n5, eq62_e1365_d_n6, eq62_e1365_d_n7, eq62_e1365_d_n8, eq62_e1365_d_n9, eq62_e1365_d_n10, eq62_e1365_d_n11, eq62_e1365_d_n12, eq62_e1365_d_n13, eq62_e1365_d_n14, eq62_e1365_d_n15, eq62_e1365_d_n16, eq62_e1365_d_n17];
        let eq62_reactive_branch_derivatives: [f64; 12] = [eq62_e1365_d_b0, eq62_e1365_d_b1, eq62_e1365_d_b2, eq62_e1365_d_b3, eq62_e1365_d_b4, eq62_e1365_d_b5, eq62_e1365_d_b6, eq62_e1365_d_b7, eq62_e1365_d_b8, eq62_e1365_d_b9, eq62_e1365_d_b10, eq62_e1365_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq62_reactive_node_derivatives,
            branches,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1384, eq66_e1384_d_n13, eq66_e1384_q,) = {
    if (p.p29 != 0.0) {
        let eq66_e1382_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1382_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq66_e1384_d_n13),
        );
    }
}
