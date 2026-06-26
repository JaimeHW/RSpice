#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_11(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22, eq168_e2124_q, eq168_e2124_q_d_n0, eq168_e2124_q_d_n1, eq168_e2124_q_d_n2, eq168_e2124_q_d_n3, eq168_e2124_q_d_n4, eq168_e2124_q_d_n5, eq168_e2124_q_d_n6, eq168_e2124_q_d_n7, eq168_e2124_q_d_n8, eq168_e2124_q_d_n9, eq168_e2124_q_d_n10, eq168_e2124_q_d_n11, eq168_e2124_q_d_n12, eq168_e2124_q_d_n13, eq168_e2124_q_d_n14, eq168_e2124_q_d_n15, eq168_e2124_q_d_n16, eq168_e2124_q_d_n17, eq168_e2124_q_d_n18, eq168_e2124_q_d_n19, eq168_e2124_q_d_n20, eq168_e2124_q_d_n21, eq168_e2124_q_d_n22,) = {
    if (s.b[590] && s.b[591]) {
        let eq168_e2121_q: f64 = s.v[277];
        let eq168_e2122: f64 = (p.p7 * s.v[277]);
        let eq168_e2122_d_n0: f64 = (p.p7 * s.dn[277][0]);
        let eq168_e2122_d_n1: f64 = (p.p7 * s.dn[277][1]);
        let eq168_e2122_d_n2: f64 = (p.p7 * s.dn[277][2]);
        let eq168_e2122_d_n3: f64 = (p.p7 * s.dn[277][3]);
        let eq168_e2122_d_n4: f64 = (p.p7 * s.dn[277][4]);
        let eq168_e2122_d_n5: f64 = (p.p7 * s.dn[277][5]);
        let eq168_e2122_d_n6: f64 = (p.p7 * s.dn[277][6]);
        let eq168_e2122_d_n7: f64 = (p.p7 * s.dn[277][7]);
        let eq168_e2122_d_n8: f64 = (p.p7 * s.dn[277][8]);
        let eq168_e2122_d_n9: f64 = (p.p7 * s.dn[277][9]);
        let eq168_e2122_d_n10: f64 = (p.p7 * s.dn[277][10]);
        let eq168_e2122_d_n11: f64 = (p.p7 * s.dn[277][11]);
        let eq168_e2122_d_n12: f64 = (p.p7 * s.dn[277][12]);
        let eq168_e2122_d_n13: f64 = (p.p7 * s.dn[277][13]);
        let eq168_e2122_d_n14: f64 = (p.p7 * s.dn[277][14]);
        let eq168_e2122_d_n15: f64 = (p.p7 * s.dn[277][15]);
        let eq168_e2122_d_n16: f64 = (p.p7 * s.dn[277][16]);
        let eq168_e2122_d_n17: f64 = (p.p7 * s.dn[277][17]);
        let eq168_e2122_d_n18: f64 = (p.p7 * s.dn[277][18]);
        let eq168_e2122_d_n19: f64 = (p.p7 * s.dn[277][19]);
        let eq168_e2122_d_n20: f64 = (p.p7 * s.dn[277][20]);
        let eq168_e2122_d_n21: f64 = (p.p7 * s.dn[277][21]);
        let eq168_e2122_d_n22: f64 = (p.p7 * s.dn[277][22]);
        let eq168_e2122_q: f64 = (p.p7 * eq168_e2121_q);
        let eq168_e2122_q_d_n0: f64 = (p.p7 * s.dn[277][0]);
        let eq168_e2122_q_d_n1: f64 = (p.p7 * s.dn[277][1]);
        let eq168_e2122_q_d_n2: f64 = (p.p7 * s.dn[277][2]);
        let eq168_e2122_q_d_n3: f64 = (p.p7 * s.dn[277][3]);
        let eq168_e2122_q_d_n4: f64 = (p.p7 * s.dn[277][4]);
        let eq168_e2122_q_d_n5: f64 = (p.p7 * s.dn[277][5]);
        let eq168_e2122_q_d_n6: f64 = (p.p7 * s.dn[277][6]);
        let eq168_e2122_q_d_n7: f64 = (p.p7 * s.dn[277][7]);
        let eq168_e2122_q_d_n8: f64 = (p.p7 * s.dn[277][8]);
        let eq168_e2122_q_d_n9: f64 = (p.p7 * s.dn[277][9]);
        let eq168_e2122_q_d_n10: f64 = (p.p7 * s.dn[277][10]);
        let eq168_e2122_q_d_n11: f64 = (p.p7 * s.dn[277][11]);
        let eq168_e2122_q_d_n12: f64 = (p.p7 * s.dn[277][12]);
        let eq168_e2122_q_d_n13: f64 = (p.p7 * s.dn[277][13]);
        let eq168_e2122_q_d_n14: f64 = (p.p7 * s.dn[277][14]);
        let eq168_e2122_q_d_n15: f64 = (p.p7 * s.dn[277][15]);
        let eq168_e2122_q_d_n16: f64 = (p.p7 * s.dn[277][16]);
        let eq168_e2122_q_d_n17: f64 = (p.p7 * s.dn[277][17]);
        let eq168_e2122_q_d_n18: f64 = (p.p7 * s.dn[277][18]);
        let eq168_e2122_q_d_n19: f64 = (p.p7 * s.dn[277][19]);
        let eq168_e2122_q_d_n20: f64 = (p.p7 * s.dn[277][20]);
        let eq168_e2122_q_d_n21: f64 = (p.p7 * s.dn[277][21]);
        let eq168_e2122_q_d_n22: f64 = (p.p7 * s.dn[277][22]);
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n10, eq168_e2122_d_n11, eq168_e2122_d_n12, eq168_e2122_d_n13, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22, eq168_e2122_q, eq168_e2122_q_d_n0, eq168_e2122_q_d_n1, eq168_e2122_q_d_n2, eq168_e2122_q_d_n3, eq168_e2122_q_d_n4, eq168_e2122_q_d_n5, eq168_e2122_q_d_n6, eq168_e2122_q_d_n7, eq168_e2122_q_d_n8, eq168_e2122_q_d_n9, eq168_e2122_q_d_n10, eq168_e2122_q_d_n11, eq168_e2122_q_d_n12, eq168_e2122_q_d_n13, eq168_e2122_q_d_n14, eq168_e2122_q_d_n15, eq168_e2122_q_d_n16, eq168_e2122_q_d_n17, eq168_e2122_q_d_n18, eq168_e2122_q_d_n19, eq168_e2122_q_d_n20, eq168_e2122_q_d_n21, eq168_e2122_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_reactive_node_derivatives: [f64; 23] = [eq168_e2124_q_d_n0, eq168_e2124_q_d_n1, eq168_e2124_q_d_n2, eq168_e2124_q_d_n3, eq168_e2124_q_d_n4, eq168_e2124_q_d_n5, eq168_e2124_q_d_n6, eq168_e2124_q_d_n7, eq168_e2124_q_d_n8, eq168_e2124_q_d_n9, eq168_e2124_q_d_n10, eq168_e2124_q_d_n11, eq168_e2124_q_d_n12, eq168_e2124_q_d_n13, eq168_e2124_q_d_n14, eq168_e2124_q_d_n15, eq168_e2124_q_d_n16, eq168_e2124_q_d_n17, eq168_e2124_q_d_n18, eq168_e2124_q_d_n19, eq168_e2124_q_d_n20, eq168_e2124_q_d_n21, eq168_e2124_q_d_n22];
        let eq168_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            nodes,
            &eq168_reactive_node_derivatives,
            branches,
            &eq168_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22, eq169_e2135_q, eq169_e2135_q_d_n0, eq169_e2135_q_d_n1, eq169_e2135_q_d_n2, eq169_e2135_q_d_n3, eq169_e2135_q_d_n4, eq169_e2135_q_d_n5, eq169_e2135_q_d_n6, eq169_e2135_q_d_n7, eq169_e2135_q_d_n8, eq169_e2135_q_d_n9, eq169_e2135_q_d_n10, eq169_e2135_q_d_n11, eq169_e2135_q_d_n12, eq169_e2135_q_d_n13, eq169_e2135_q_d_n14, eq169_e2135_q_d_n15, eq169_e2135_q_d_n16, eq169_e2135_q_d_n17, eq169_e2135_q_d_n18, eq169_e2135_q_d_n19, eq169_e2135_q_d_n20, eq169_e2135_q_d_n21, eq169_e2135_q_d_n22,) = {
    if ((s.b[590] && s.b[591]) && s.b[592]) {
        let eq169_e2132_q: f64 = s.v[276];
        let eq169_e2133: f64 = (p.p7 * s.v[276]);
        let eq169_e2133_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq169_e2133_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq169_e2133_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq169_e2133_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq169_e2133_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq169_e2133_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq169_e2133_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq169_e2133_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq169_e2133_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq169_e2133_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq169_e2133_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq169_e2133_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq169_e2133_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq169_e2133_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq169_e2133_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq169_e2133_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq169_e2133_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq169_e2133_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq169_e2133_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq169_e2133_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq169_e2133_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq169_e2133_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq169_e2133_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq169_e2133_q: f64 = (p.p7 * eq169_e2132_q);
        let eq169_e2133_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq169_e2133_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq169_e2133_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq169_e2133_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq169_e2133_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq169_e2133_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq169_e2133_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq169_e2133_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq169_e2133_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq169_e2133_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq169_e2133_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq169_e2133_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq169_e2133_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq169_e2133_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq169_e2133_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq169_e2133_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq169_e2133_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq169_e2133_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq169_e2133_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq169_e2133_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq169_e2133_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq169_e2133_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq169_e2133_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        (eq169_e2133, eq169_e2133_d_n0, eq169_e2133_d_n1, eq169_e2133_d_n2, eq169_e2133_d_n3, eq169_e2133_d_n4, eq169_e2133_d_n5, eq169_e2133_d_n6, eq169_e2133_d_n7, eq169_e2133_d_n8, eq169_e2133_d_n9, eq169_e2133_d_n10, eq169_e2133_d_n11, eq169_e2133_d_n12, eq169_e2133_d_n13, eq169_e2133_d_n14, eq169_e2133_d_n15, eq169_e2133_d_n16, eq169_e2133_d_n17, eq169_e2133_d_n18, eq169_e2133_d_n19, eq169_e2133_d_n20, eq169_e2133_d_n21, eq169_e2133_d_n22, eq169_e2133_q, eq169_e2133_q_d_n0, eq169_e2133_q_d_n1, eq169_e2133_q_d_n2, eq169_e2133_q_d_n3, eq169_e2133_q_d_n4, eq169_e2133_q_d_n5, eq169_e2133_q_d_n6, eq169_e2133_q_d_n7, eq169_e2133_q_d_n8, eq169_e2133_q_d_n9, eq169_e2133_q_d_n10, eq169_e2133_q_d_n11, eq169_e2133_q_d_n12, eq169_e2133_q_d_n13, eq169_e2133_q_d_n14, eq169_e2133_q_d_n15, eq169_e2133_q_d_n16, eq169_e2133_q_d_n17, eq169_e2133_q_d_n18, eq169_e2133_q_d_n19, eq169_e2133_q_d_n20, eq169_e2133_q_d_n21, eq169_e2133_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_reactive_node_derivatives: [f64; 23] = [eq169_e2135_q_d_n0, eq169_e2135_q_d_n1, eq169_e2135_q_d_n2, eq169_e2135_q_d_n3, eq169_e2135_q_d_n4, eq169_e2135_q_d_n5, eq169_e2135_q_d_n6, eq169_e2135_q_d_n7, eq169_e2135_q_d_n8, eq169_e2135_q_d_n9, eq169_e2135_q_d_n10, eq169_e2135_q_d_n11, eq169_e2135_q_d_n12, eq169_e2135_q_d_n13, eq169_e2135_q_d_n14, eq169_e2135_q_d_n15, eq169_e2135_q_d_n16, eq169_e2135_q_d_n17, eq169_e2135_q_d_n18, eq169_e2135_q_d_n19, eq169_e2135_q_d_n20, eq169_e2135_q_d_n21, eq169_e2135_q_d_n22];
        let eq169_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            nodes,
            &eq169_reactive_node_derivatives,
            branches,
            &eq169_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22, eq170_e2148_q, eq170_e2148_q_d_n0, eq170_e2148_q_d_n1, eq170_e2148_q_d_n2, eq170_e2148_q_d_n3, eq170_e2148_q_d_n4, eq170_e2148_q_d_n5, eq170_e2148_q_d_n6, eq170_e2148_q_d_n7, eq170_e2148_q_d_n8, eq170_e2148_q_d_n9, eq170_e2148_q_d_n10, eq170_e2148_q_d_n11, eq170_e2148_q_d_n12, eq170_e2148_q_d_n13, eq170_e2148_q_d_n14, eq170_e2148_q_d_n15, eq170_e2148_q_d_n16, eq170_e2148_q_d_n17, eq170_e2148_q_d_n18, eq170_e2148_q_d_n19, eq170_e2148_q_d_n20, eq170_e2148_q_d_n21, eq170_e2148_q_d_n22,) = {
    if ((s.b[590] && s.b[591]) && s.b[592]) {
        let eq170_e2143_q: f64 = s.v[276];
        let eq170_e2144: f64 = (p.p7 * s.v[276]);
        let eq170_e2144_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq170_e2144_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq170_e2144_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq170_e2144_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq170_e2144_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq170_e2144_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq170_e2144_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq170_e2144_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq170_e2144_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq170_e2144_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq170_e2144_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq170_e2144_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq170_e2144_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq170_e2144_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq170_e2144_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq170_e2144_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq170_e2144_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq170_e2144_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq170_e2144_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq170_e2144_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq170_e2144_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq170_e2144_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq170_e2144_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq170_e2144_q: f64 = (p.p7 * eq170_e2143_q);
        let eq170_e2144_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq170_e2144_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq170_e2144_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq170_e2144_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq170_e2144_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq170_e2144_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq170_e2144_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq170_e2144_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq170_e2144_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq170_e2144_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq170_e2144_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq170_e2144_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq170_e2144_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq170_e2144_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq170_e2144_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq170_e2144_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq170_e2144_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq170_e2144_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq170_e2144_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq170_e2144_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq170_e2144_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq170_e2144_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq170_e2144_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq170_e2146: f64 = (eq170_e2144 * p.p248);
        let eq170_e2146_d_n0: f64 = (eq170_e2144_d_n0 * p.p248);
        let eq170_e2146_d_n1: f64 = (eq170_e2144_d_n1 * p.p248);
        let eq170_e2146_d_n2: f64 = (eq170_e2144_d_n2 * p.p248);
        let eq170_e2146_d_n3: f64 = (eq170_e2144_d_n3 * p.p248);
        let eq170_e2146_d_n4: f64 = (eq170_e2144_d_n4 * p.p248);
        let eq170_e2146_d_n5: f64 = (eq170_e2144_d_n5 * p.p248);
        let eq170_e2146_d_n6: f64 = (eq170_e2144_d_n6 * p.p248);
        let eq170_e2146_d_n7: f64 = (eq170_e2144_d_n7 * p.p248);
        let eq170_e2146_d_n8: f64 = (eq170_e2144_d_n8 * p.p248);
        let eq170_e2146_d_n9: f64 = (eq170_e2144_d_n9 * p.p248);
        let eq170_e2146_d_n10: f64 = (eq170_e2144_d_n10 * p.p248);
        let eq170_e2146_d_n11: f64 = (eq170_e2144_d_n11 * p.p248);
        let eq170_e2146_d_n12: f64 = (eq170_e2144_d_n12 * p.p248);
        let eq170_e2146_d_n13: f64 = (eq170_e2144_d_n13 * p.p248);
        let eq170_e2146_d_n14: f64 = (eq170_e2144_d_n14 * p.p248);
        let eq170_e2146_d_n15: f64 = (eq170_e2144_d_n15 * p.p248);
        let eq170_e2146_d_n16: f64 = (eq170_e2144_d_n16 * p.p248);
        let eq170_e2146_d_n17: f64 = (eq170_e2144_d_n17 * p.p248);
        let eq170_e2146_d_n18: f64 = (eq170_e2144_d_n18 * p.p248);
        let eq170_e2146_d_n19: f64 = (eq170_e2144_d_n19 * p.p248);
        let eq170_e2146_d_n20: f64 = (eq170_e2144_d_n20 * p.p248);
        let eq170_e2146_d_n21: f64 = (eq170_e2144_d_n21 * p.p248);
        let eq170_e2146_d_n22: f64 = (eq170_e2144_d_n22 * p.p248);
        let eq170_e2146_q: f64 = (eq170_e2144_q * p.p248);
        let eq170_e2146_q_d_n0: f64 = (eq170_e2144_q_d_n0 * p.p248);
        let eq170_e2146_q_d_n1: f64 = (eq170_e2144_q_d_n1 * p.p248);
        let eq170_e2146_q_d_n2: f64 = (eq170_e2144_q_d_n2 * p.p248);
        let eq170_e2146_q_d_n3: f64 = (eq170_e2144_q_d_n3 * p.p248);
        let eq170_e2146_q_d_n4: f64 = (eq170_e2144_q_d_n4 * p.p248);
        let eq170_e2146_q_d_n5: f64 = (eq170_e2144_q_d_n5 * p.p248);
        let eq170_e2146_q_d_n6: f64 = (eq170_e2144_q_d_n6 * p.p248);
        let eq170_e2146_q_d_n7: f64 = (eq170_e2144_q_d_n7 * p.p248);
        let eq170_e2146_q_d_n8: f64 = (eq170_e2144_q_d_n8 * p.p248);
        let eq170_e2146_q_d_n9: f64 = (eq170_e2144_q_d_n9 * p.p248);
        let eq170_e2146_q_d_n10: f64 = (eq170_e2144_q_d_n10 * p.p248);
        let eq170_e2146_q_d_n11: f64 = (eq170_e2144_q_d_n11 * p.p248);
        let eq170_e2146_q_d_n12: f64 = (eq170_e2144_q_d_n12 * p.p248);
        let eq170_e2146_q_d_n13: f64 = (eq170_e2144_q_d_n13 * p.p248);
        let eq170_e2146_q_d_n14: f64 = (eq170_e2144_q_d_n14 * p.p248);
        let eq170_e2146_q_d_n15: f64 = (eq170_e2144_q_d_n15 * p.p248);
        let eq170_e2146_q_d_n16: f64 = (eq170_e2144_q_d_n16 * p.p248);
        let eq170_e2146_q_d_n17: f64 = (eq170_e2144_q_d_n17 * p.p248);
        let eq170_e2146_q_d_n18: f64 = (eq170_e2144_q_d_n18 * p.p248);
        let eq170_e2146_q_d_n19: f64 = (eq170_e2144_q_d_n19 * p.p248);
        let eq170_e2146_q_d_n20: f64 = (eq170_e2144_q_d_n20 * p.p248);
        let eq170_e2146_q_d_n21: f64 = (eq170_e2144_q_d_n21 * p.p248);
        let eq170_e2146_q_d_n22: f64 = (eq170_e2144_q_d_n22 * p.p248);
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n10, eq170_e2146_d_n11, eq170_e2146_d_n12, eq170_e2146_d_n13, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22, eq170_e2146_q, eq170_e2146_q_d_n0, eq170_e2146_q_d_n1, eq170_e2146_q_d_n2, eq170_e2146_q_d_n3, eq170_e2146_q_d_n4, eq170_e2146_q_d_n5, eq170_e2146_q_d_n6, eq170_e2146_q_d_n7, eq170_e2146_q_d_n8, eq170_e2146_q_d_n9, eq170_e2146_q_d_n10, eq170_e2146_q_d_n11, eq170_e2146_q_d_n12, eq170_e2146_q_d_n13, eq170_e2146_q_d_n14, eq170_e2146_q_d_n15, eq170_e2146_q_d_n16, eq170_e2146_q_d_n17, eq170_e2146_q_d_n18, eq170_e2146_q_d_n19, eq170_e2146_q_d_n20, eq170_e2146_q_d_n21, eq170_e2146_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_reactive_node_derivatives: [f64; 23] = [eq170_e2148_q_d_n0, eq170_e2148_q_d_n1, eq170_e2148_q_d_n2, eq170_e2148_q_d_n3, eq170_e2148_q_d_n4, eq170_e2148_q_d_n5, eq170_e2148_q_d_n6, eq170_e2148_q_d_n7, eq170_e2148_q_d_n8, eq170_e2148_q_d_n9, eq170_e2148_q_d_n10, eq170_e2148_q_d_n11, eq170_e2148_q_d_n12, eq170_e2148_q_d_n13, eq170_e2148_q_d_n14, eq170_e2148_q_d_n15, eq170_e2148_q_d_n16, eq170_e2148_q_d_n17, eq170_e2148_q_d_n18, eq170_e2148_q_d_n19, eq170_e2148_q_d_n20, eq170_e2148_q_d_n21, eq170_e2148_q_d_n22];
        let eq170_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq170_reactive_node_derivatives,
            branches,
            &eq170_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq171_e2160, eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22, eq171_e2160_q, eq171_e2160_q_d_n0, eq171_e2160_q_d_n1, eq171_e2160_q_d_n2, eq171_e2160_q_d_n3, eq171_e2160_q_d_n4, eq171_e2160_q_d_n5, eq171_e2160_q_d_n6, eq171_e2160_q_d_n7, eq171_e2160_q_d_n8, eq171_e2160_q_d_n9, eq171_e2160_q_d_n10, eq171_e2160_q_d_n11, eq171_e2160_q_d_n12, eq171_e2160_q_d_n13, eq171_e2160_q_d_n14, eq171_e2160_q_d_n15, eq171_e2160_q_d_n16, eq171_e2160_q_d_n17, eq171_e2160_q_d_n18, eq171_e2160_q_d_n19, eq171_e2160_q_d_n20, eq171_e2160_q_d_n21, eq171_e2160_q_d_n22,) = {
    if ((s.b[590] && s.b[591]) && (!s.b[592])) {
        let eq171_e2157_q: f64 = s.v[276];
        let eq171_e2158: f64 = (p.p7 * s.v[276]);
        let eq171_e2158_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq171_e2158_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq171_e2158_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq171_e2158_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq171_e2158_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq171_e2158_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq171_e2158_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq171_e2158_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq171_e2158_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq171_e2158_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq171_e2158_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq171_e2158_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq171_e2158_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq171_e2158_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq171_e2158_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq171_e2158_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq171_e2158_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq171_e2158_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq171_e2158_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq171_e2158_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq171_e2158_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq171_e2158_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq171_e2158_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq171_e2158_q: f64 = (p.p7 * eq171_e2157_q);
        let eq171_e2158_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq171_e2158_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq171_e2158_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq171_e2158_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq171_e2158_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq171_e2158_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq171_e2158_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq171_e2158_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq171_e2158_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq171_e2158_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq171_e2158_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq171_e2158_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq171_e2158_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq171_e2158_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq171_e2158_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq171_e2158_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq171_e2158_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq171_e2158_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq171_e2158_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq171_e2158_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq171_e2158_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq171_e2158_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq171_e2158_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        (eq171_e2158, eq171_e2158_d_n0, eq171_e2158_d_n1, eq171_e2158_d_n2, eq171_e2158_d_n3, eq171_e2158_d_n4, eq171_e2158_d_n5, eq171_e2158_d_n6, eq171_e2158_d_n7, eq171_e2158_d_n8, eq171_e2158_d_n9, eq171_e2158_d_n10, eq171_e2158_d_n11, eq171_e2158_d_n12, eq171_e2158_d_n13, eq171_e2158_d_n14, eq171_e2158_d_n15, eq171_e2158_d_n16, eq171_e2158_d_n17, eq171_e2158_d_n18, eq171_e2158_d_n19, eq171_e2158_d_n20, eq171_e2158_d_n21, eq171_e2158_d_n22, eq171_e2158_q, eq171_e2158_q_d_n0, eq171_e2158_q_d_n1, eq171_e2158_q_d_n2, eq171_e2158_q_d_n3, eq171_e2158_q_d_n4, eq171_e2158_q_d_n5, eq171_e2158_q_d_n6, eq171_e2158_q_d_n7, eq171_e2158_q_d_n8, eq171_e2158_q_d_n9, eq171_e2158_q_d_n10, eq171_e2158_q_d_n11, eq171_e2158_q_d_n12, eq171_e2158_q_d_n13, eq171_e2158_q_d_n14, eq171_e2158_q_d_n15, eq171_e2158_q_d_n16, eq171_e2158_q_d_n17, eq171_e2158_q_d_n18, eq171_e2158_q_d_n19, eq171_e2158_q_d_n20, eq171_e2158_q_d_n21, eq171_e2158_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq171_reactive_node_derivatives: [f64; 23] = [eq171_e2160_q_d_n0, eq171_e2160_q_d_n1, eq171_e2160_q_d_n2, eq171_e2160_q_d_n3, eq171_e2160_q_d_n4, eq171_e2160_q_d_n5, eq171_e2160_q_d_n6, eq171_e2160_q_d_n7, eq171_e2160_q_d_n8, eq171_e2160_q_d_n9, eq171_e2160_q_d_n10, eq171_e2160_q_d_n11, eq171_e2160_q_d_n12, eq171_e2160_q_d_n13, eq171_e2160_q_d_n14, eq171_e2160_q_d_n15, eq171_e2160_q_d_n16, eq171_e2160_q_d_n17, eq171_e2160_q_d_n18, eq171_e2160_q_d_n19, eq171_e2160_q_d_n20, eq171_e2160_q_d_n21, eq171_e2160_q_d_n22];
        let eq171_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            nodes,
            &eq171_reactive_node_derivatives,
            branches,
            &eq171_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq172_e2174, eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22, eq172_e2174_q, eq172_e2174_q_d_n0, eq172_e2174_q_d_n1, eq172_e2174_q_d_n2, eq172_e2174_q_d_n3, eq172_e2174_q_d_n4, eq172_e2174_q_d_n5, eq172_e2174_q_d_n6, eq172_e2174_q_d_n7, eq172_e2174_q_d_n8, eq172_e2174_q_d_n9, eq172_e2174_q_d_n10, eq172_e2174_q_d_n11, eq172_e2174_q_d_n12, eq172_e2174_q_d_n13, eq172_e2174_q_d_n14, eq172_e2174_q_d_n15, eq172_e2174_q_d_n16, eq172_e2174_q_d_n17, eq172_e2174_q_d_n18, eq172_e2174_q_d_n19, eq172_e2174_q_d_n20, eq172_e2174_q_d_n21, eq172_e2174_q_d_n22,) = {
    if ((s.b[590] && s.b[591]) && (!s.b[592])) {
        let eq172_e2169_q: f64 = s.v[276];
        let eq172_e2170: f64 = (p.p7 * s.v[276]);
        let eq172_e2170_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq172_e2170_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq172_e2170_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq172_e2170_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq172_e2170_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq172_e2170_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq172_e2170_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq172_e2170_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq172_e2170_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq172_e2170_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq172_e2170_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq172_e2170_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq172_e2170_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq172_e2170_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq172_e2170_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq172_e2170_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq172_e2170_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq172_e2170_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq172_e2170_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq172_e2170_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq172_e2170_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq172_e2170_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq172_e2170_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq172_e2170_q: f64 = (p.p7 * eq172_e2169_q);
        let eq172_e2170_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq172_e2170_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq172_e2170_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq172_e2170_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq172_e2170_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq172_e2170_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq172_e2170_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq172_e2170_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq172_e2170_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq172_e2170_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq172_e2170_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq172_e2170_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq172_e2170_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq172_e2170_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq172_e2170_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq172_e2170_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq172_e2170_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq172_e2170_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq172_e2170_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq172_e2170_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq172_e2170_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq172_e2170_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq172_e2170_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq172_e2172: f64 = (eq172_e2170 * p.p248);
        let eq172_e2172_d_n0: f64 = (eq172_e2170_d_n0 * p.p248);
        let eq172_e2172_d_n1: f64 = (eq172_e2170_d_n1 * p.p248);
        let eq172_e2172_d_n2: f64 = (eq172_e2170_d_n2 * p.p248);
        let eq172_e2172_d_n3: f64 = (eq172_e2170_d_n3 * p.p248);
        let eq172_e2172_d_n4: f64 = (eq172_e2170_d_n4 * p.p248);
        let eq172_e2172_d_n5: f64 = (eq172_e2170_d_n5 * p.p248);
        let eq172_e2172_d_n6: f64 = (eq172_e2170_d_n6 * p.p248);
        let eq172_e2172_d_n7: f64 = (eq172_e2170_d_n7 * p.p248);
        let eq172_e2172_d_n8: f64 = (eq172_e2170_d_n8 * p.p248);
        let eq172_e2172_d_n9: f64 = (eq172_e2170_d_n9 * p.p248);
        let eq172_e2172_d_n10: f64 = (eq172_e2170_d_n10 * p.p248);
        let eq172_e2172_d_n11: f64 = (eq172_e2170_d_n11 * p.p248);
        let eq172_e2172_d_n12: f64 = (eq172_e2170_d_n12 * p.p248);
        let eq172_e2172_d_n13: f64 = (eq172_e2170_d_n13 * p.p248);
        let eq172_e2172_d_n14: f64 = (eq172_e2170_d_n14 * p.p248);
        let eq172_e2172_d_n15: f64 = (eq172_e2170_d_n15 * p.p248);
        let eq172_e2172_d_n16: f64 = (eq172_e2170_d_n16 * p.p248);
        let eq172_e2172_d_n17: f64 = (eq172_e2170_d_n17 * p.p248);
        let eq172_e2172_d_n18: f64 = (eq172_e2170_d_n18 * p.p248);
        let eq172_e2172_d_n19: f64 = (eq172_e2170_d_n19 * p.p248);
        let eq172_e2172_d_n20: f64 = (eq172_e2170_d_n20 * p.p248);
        let eq172_e2172_d_n21: f64 = (eq172_e2170_d_n21 * p.p248);
        let eq172_e2172_d_n22: f64 = (eq172_e2170_d_n22 * p.p248);
        let eq172_e2172_q: f64 = (eq172_e2170_q * p.p248);
        let eq172_e2172_q_d_n0: f64 = (eq172_e2170_q_d_n0 * p.p248);
        let eq172_e2172_q_d_n1: f64 = (eq172_e2170_q_d_n1 * p.p248);
        let eq172_e2172_q_d_n2: f64 = (eq172_e2170_q_d_n2 * p.p248);
        let eq172_e2172_q_d_n3: f64 = (eq172_e2170_q_d_n3 * p.p248);
        let eq172_e2172_q_d_n4: f64 = (eq172_e2170_q_d_n4 * p.p248);
        let eq172_e2172_q_d_n5: f64 = (eq172_e2170_q_d_n5 * p.p248);
        let eq172_e2172_q_d_n6: f64 = (eq172_e2170_q_d_n6 * p.p248);
        let eq172_e2172_q_d_n7: f64 = (eq172_e2170_q_d_n7 * p.p248);
        let eq172_e2172_q_d_n8: f64 = (eq172_e2170_q_d_n8 * p.p248);
        let eq172_e2172_q_d_n9: f64 = (eq172_e2170_q_d_n9 * p.p248);
        let eq172_e2172_q_d_n10: f64 = (eq172_e2170_q_d_n10 * p.p248);
        let eq172_e2172_q_d_n11: f64 = (eq172_e2170_q_d_n11 * p.p248);
        let eq172_e2172_q_d_n12: f64 = (eq172_e2170_q_d_n12 * p.p248);
        let eq172_e2172_q_d_n13: f64 = (eq172_e2170_q_d_n13 * p.p248);
        let eq172_e2172_q_d_n14: f64 = (eq172_e2170_q_d_n14 * p.p248);
        let eq172_e2172_q_d_n15: f64 = (eq172_e2170_q_d_n15 * p.p248);
        let eq172_e2172_q_d_n16: f64 = (eq172_e2170_q_d_n16 * p.p248);
        let eq172_e2172_q_d_n17: f64 = (eq172_e2170_q_d_n17 * p.p248);
        let eq172_e2172_q_d_n18: f64 = (eq172_e2170_q_d_n18 * p.p248);
        let eq172_e2172_q_d_n19: f64 = (eq172_e2170_q_d_n19 * p.p248);
        let eq172_e2172_q_d_n20: f64 = (eq172_e2170_q_d_n20 * p.p248);
        let eq172_e2172_q_d_n21: f64 = (eq172_e2170_q_d_n21 * p.p248);
        let eq172_e2172_q_d_n22: f64 = (eq172_e2170_q_d_n22 * p.p248);
        (eq172_e2172, eq172_e2172_d_n0, eq172_e2172_d_n1, eq172_e2172_d_n2, eq172_e2172_d_n3, eq172_e2172_d_n4, eq172_e2172_d_n5, eq172_e2172_d_n6, eq172_e2172_d_n7, eq172_e2172_d_n8, eq172_e2172_d_n9, eq172_e2172_d_n10, eq172_e2172_d_n11, eq172_e2172_d_n12, eq172_e2172_d_n13, eq172_e2172_d_n14, eq172_e2172_d_n15, eq172_e2172_d_n16, eq172_e2172_d_n17, eq172_e2172_d_n18, eq172_e2172_d_n19, eq172_e2172_d_n20, eq172_e2172_d_n21, eq172_e2172_d_n22, eq172_e2172_q, eq172_e2172_q_d_n0, eq172_e2172_q_d_n1, eq172_e2172_q_d_n2, eq172_e2172_q_d_n3, eq172_e2172_q_d_n4, eq172_e2172_q_d_n5, eq172_e2172_q_d_n6, eq172_e2172_q_d_n7, eq172_e2172_q_d_n8, eq172_e2172_q_d_n9, eq172_e2172_q_d_n10, eq172_e2172_q_d_n11, eq172_e2172_q_d_n12, eq172_e2172_q_d_n13, eq172_e2172_q_d_n14, eq172_e2172_q_d_n15, eq172_e2172_q_d_n16, eq172_e2172_q_d_n17, eq172_e2172_q_d_n18, eq172_e2172_q_d_n19, eq172_e2172_q_d_n20, eq172_e2172_q_d_n21, eq172_e2172_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq172_reactive_node_derivatives: [f64; 23] = [eq172_e2174_q_d_n0, eq172_e2174_q_d_n1, eq172_e2174_q_d_n2, eq172_e2174_q_d_n3, eq172_e2174_q_d_n4, eq172_e2174_q_d_n5, eq172_e2174_q_d_n6, eq172_e2174_q_d_n7, eq172_e2174_q_d_n8, eq172_e2174_q_d_n9, eq172_e2174_q_d_n10, eq172_e2174_q_d_n11, eq172_e2174_q_d_n12, eq172_e2174_q_d_n13, eq172_e2174_q_d_n14, eq172_e2174_q_d_n15, eq172_e2174_q_d_n16, eq172_e2174_q_d_n17, eq172_e2174_q_d_n18, eq172_e2174_q_d_n19, eq172_e2174_q_d_n20, eq172_e2174_q_d_n21, eq172_e2174_q_d_n22];
        let eq172_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            nodes,
            &eq172_reactive_node_derivatives,
            branches,
            &eq172_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_12(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq173_e2185, eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22, eq173_e2185_q, eq173_e2185_q_d_n0, eq173_e2185_q_d_n1, eq173_e2185_q_d_n2, eq173_e2185_q_d_n3, eq173_e2185_q_d_n4, eq173_e2185_q_d_n5, eq173_e2185_q_d_n6, eq173_e2185_q_d_n7, eq173_e2185_q_d_n8, eq173_e2185_q_d_n9, eq173_e2185_q_d_n10, eq173_e2185_q_d_n11, eq173_e2185_q_d_n12, eq173_e2185_q_d_n13, eq173_e2185_q_d_n14, eq173_e2185_q_d_n15, eq173_e2185_q_d_n16, eq173_e2185_q_d_n17, eq173_e2185_q_d_n18, eq173_e2185_q_d_n19, eq173_e2185_q_d_n20, eq173_e2185_q_d_n21, eq173_e2185_q_d_n22,) = {
    if (s.b[590] && s.b[591]) {
        let eq173_e2181: f64 = (p.p253 * s.v[276]);
        let eq173_e2181_d_n0: f64 = (p.p253 * s.dn[276][0]);
        let eq173_e2181_d_n1: f64 = (p.p253 * s.dn[276][1]);
        let eq173_e2181_d_n2: f64 = (p.p253 * s.dn[276][2]);
        let eq173_e2181_d_n3: f64 = (p.p253 * s.dn[276][3]);
        let eq173_e2181_d_n4: f64 = (p.p253 * s.dn[276][4]);
        let eq173_e2181_d_n5: f64 = (p.p253 * s.dn[276][5]);
        let eq173_e2181_d_n6: f64 = (p.p253 * s.dn[276][6]);
        let eq173_e2181_d_n7: f64 = (p.p253 * s.dn[276][7]);
        let eq173_e2181_d_n8: f64 = (p.p253 * s.dn[276][8]);
        let eq173_e2181_d_n9: f64 = (p.p253 * s.dn[276][9]);
        let eq173_e2181_d_n10: f64 = (p.p253 * s.dn[276][10]);
        let eq173_e2181_d_n11: f64 = (p.p253 * s.dn[276][11]);
        let eq173_e2181_d_n12: f64 = (p.p253 * s.dn[276][12]);
        let eq173_e2181_d_n13: f64 = (p.p253 * s.dn[276][13]);
        let eq173_e2181_d_n14: f64 = (p.p253 * s.dn[276][14]);
        let eq173_e2181_d_n15: f64 = (p.p253 * s.dn[276][15]);
        let eq173_e2181_d_n16: f64 = (p.p253 * s.dn[276][16]);
        let eq173_e2181_d_n17: f64 = (p.p253 * s.dn[276][17]);
        let eq173_e2181_d_n18: f64 = (p.p253 * s.dn[276][18]);
        let eq173_e2181_d_n19: f64 = (p.p253 * s.dn[276][19]);
        let eq173_e2181_d_n20: f64 = (p.p253 * s.dn[276][20]);
        let eq173_e2181_d_n21: f64 = (p.p253 * s.dn[276][21]);
        let eq173_e2181_d_n22: f64 = (p.p253 * s.dn[276][22]);
        let eq173_e2182_q: f64 = eq173_e2181;
        let eq173_e2183: f64 = (p.p7 * eq173_e2181);
        let eq173_e2183_d_n0: f64 = (p.p7 * eq173_e2181_d_n0);
        let eq173_e2183_d_n1: f64 = (p.p7 * eq173_e2181_d_n1);
        let eq173_e2183_d_n2: f64 = (p.p7 * eq173_e2181_d_n2);
        let eq173_e2183_d_n3: f64 = (p.p7 * eq173_e2181_d_n3);
        let eq173_e2183_d_n4: f64 = (p.p7 * eq173_e2181_d_n4);
        let eq173_e2183_d_n5: f64 = (p.p7 * eq173_e2181_d_n5);
        let eq173_e2183_d_n6: f64 = (p.p7 * eq173_e2181_d_n6);
        let eq173_e2183_d_n7: f64 = (p.p7 * eq173_e2181_d_n7);
        let eq173_e2183_d_n8: f64 = (p.p7 * eq173_e2181_d_n8);
        let eq173_e2183_d_n9: f64 = (p.p7 * eq173_e2181_d_n9);
        let eq173_e2183_d_n10: f64 = (p.p7 * eq173_e2181_d_n10);
        let eq173_e2183_d_n11: f64 = (p.p7 * eq173_e2181_d_n11);
        let eq173_e2183_d_n12: f64 = (p.p7 * eq173_e2181_d_n12);
        let eq173_e2183_d_n13: f64 = (p.p7 * eq173_e2181_d_n13);
        let eq173_e2183_d_n14: f64 = (p.p7 * eq173_e2181_d_n14);
        let eq173_e2183_d_n15: f64 = (p.p7 * eq173_e2181_d_n15);
        let eq173_e2183_d_n16: f64 = (p.p7 * eq173_e2181_d_n16);
        let eq173_e2183_d_n17: f64 = (p.p7 * eq173_e2181_d_n17);
        let eq173_e2183_d_n18: f64 = (p.p7 * eq173_e2181_d_n18);
        let eq173_e2183_d_n19: f64 = (p.p7 * eq173_e2181_d_n19);
        let eq173_e2183_d_n20: f64 = (p.p7 * eq173_e2181_d_n20);
        let eq173_e2183_d_n21: f64 = (p.p7 * eq173_e2181_d_n21);
        let eq173_e2183_d_n22: f64 = (p.p7 * eq173_e2181_d_n22);
        let eq173_e2183_q: f64 = (p.p7 * eq173_e2182_q);
        let eq173_e2183_q_d_n0: f64 = (p.p7 * eq173_e2181_d_n0);
        let eq173_e2183_q_d_n1: f64 = (p.p7 * eq173_e2181_d_n1);
        let eq173_e2183_q_d_n2: f64 = (p.p7 * eq173_e2181_d_n2);
        let eq173_e2183_q_d_n3: f64 = (p.p7 * eq173_e2181_d_n3);
        let eq173_e2183_q_d_n4: f64 = (p.p7 * eq173_e2181_d_n4);
        let eq173_e2183_q_d_n5: f64 = (p.p7 * eq173_e2181_d_n5);
        let eq173_e2183_q_d_n6: f64 = (p.p7 * eq173_e2181_d_n6);
        let eq173_e2183_q_d_n7: f64 = (p.p7 * eq173_e2181_d_n7);
        let eq173_e2183_q_d_n8: f64 = (p.p7 * eq173_e2181_d_n8);
        let eq173_e2183_q_d_n9: f64 = (p.p7 * eq173_e2181_d_n9);
        let eq173_e2183_q_d_n10: f64 = (p.p7 * eq173_e2181_d_n10);
        let eq173_e2183_q_d_n11: f64 = (p.p7 * eq173_e2181_d_n11);
        let eq173_e2183_q_d_n12: f64 = (p.p7 * eq173_e2181_d_n12);
        let eq173_e2183_q_d_n13: f64 = (p.p7 * eq173_e2181_d_n13);
        let eq173_e2183_q_d_n14: f64 = (p.p7 * eq173_e2181_d_n14);
        let eq173_e2183_q_d_n15: f64 = (p.p7 * eq173_e2181_d_n15);
        let eq173_e2183_q_d_n16: f64 = (p.p7 * eq173_e2181_d_n16);
        let eq173_e2183_q_d_n17: f64 = (p.p7 * eq173_e2181_d_n17);
        let eq173_e2183_q_d_n18: f64 = (p.p7 * eq173_e2181_d_n18);
        let eq173_e2183_q_d_n19: f64 = (p.p7 * eq173_e2181_d_n19);
        let eq173_e2183_q_d_n20: f64 = (p.p7 * eq173_e2181_d_n20);
        let eq173_e2183_q_d_n21: f64 = (p.p7 * eq173_e2181_d_n21);
        let eq173_e2183_q_d_n22: f64 = (p.p7 * eq173_e2181_d_n22);
        (eq173_e2183, eq173_e2183_d_n0, eq173_e2183_d_n1, eq173_e2183_d_n2, eq173_e2183_d_n3, eq173_e2183_d_n4, eq173_e2183_d_n5, eq173_e2183_d_n6, eq173_e2183_d_n7, eq173_e2183_d_n8, eq173_e2183_d_n9, eq173_e2183_d_n10, eq173_e2183_d_n11, eq173_e2183_d_n12, eq173_e2183_d_n13, eq173_e2183_d_n14, eq173_e2183_d_n15, eq173_e2183_d_n16, eq173_e2183_d_n17, eq173_e2183_d_n18, eq173_e2183_d_n19, eq173_e2183_d_n20, eq173_e2183_d_n21, eq173_e2183_d_n22, eq173_e2183_q, eq173_e2183_q_d_n0, eq173_e2183_q_d_n1, eq173_e2183_q_d_n2, eq173_e2183_q_d_n3, eq173_e2183_q_d_n4, eq173_e2183_q_d_n5, eq173_e2183_q_d_n6, eq173_e2183_q_d_n7, eq173_e2183_q_d_n8, eq173_e2183_q_d_n9, eq173_e2183_q_d_n10, eq173_e2183_q_d_n11, eq173_e2183_q_d_n12, eq173_e2183_q_d_n13, eq173_e2183_q_d_n14, eq173_e2183_q_d_n15, eq173_e2183_q_d_n16, eq173_e2183_q_d_n17, eq173_e2183_q_d_n18, eq173_e2183_q_d_n19, eq173_e2183_q_d_n20, eq173_e2183_q_d_n21, eq173_e2183_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq173_reactive_node_derivatives: [f64; 23] = [eq173_e2185_q_d_n0, eq173_e2185_q_d_n1, eq173_e2185_q_d_n2, eq173_e2185_q_d_n3, eq173_e2185_q_d_n4, eq173_e2185_q_d_n5, eq173_e2185_q_d_n6, eq173_e2185_q_d_n7, eq173_e2185_q_d_n8, eq173_e2185_q_d_n9, eq173_e2185_q_d_n10, eq173_e2185_q_d_n11, eq173_e2185_q_d_n12, eq173_e2185_q_d_n13, eq173_e2185_q_d_n14, eq173_e2185_q_d_n15, eq173_e2185_q_d_n16, eq173_e2185_q_d_n17, eq173_e2185_q_d_n18, eq173_e2185_q_d_n19, eq173_e2185_q_d_n20, eq173_e2185_q_d_n21, eq173_e2185_q_d_n22];
        let eq173_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            nodes,
            &eq173_reactive_node_derivatives,
            branches,
            &eq173_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq174_e2195, eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22, eq174_e2195_q, eq174_e2195_q_d_n0, eq174_e2195_q_d_n1, eq174_e2195_q_d_n2, eq174_e2195_q_d_n3, eq174_e2195_q_d_n4, eq174_e2195_q_d_n5, eq174_e2195_q_d_n6, eq174_e2195_q_d_n7, eq174_e2195_q_d_n8, eq174_e2195_q_d_n9, eq174_e2195_q_d_n10, eq174_e2195_q_d_n11, eq174_e2195_q_d_n12, eq174_e2195_q_d_n13, eq174_e2195_q_d_n14, eq174_e2195_q_d_n15, eq174_e2195_q_d_n16, eq174_e2195_q_d_n17, eq174_e2195_q_d_n18, eq174_e2195_q_d_n19, eq174_e2195_q_d_n20, eq174_e2195_q_d_n21, eq174_e2195_q_d_n22,) = {
    if ((!s.b[590]) && s.b[593]) {
        let eq174_e2192_q: f64 = s.v[277];
        let eq174_e2193: f64 = (p.p7 * s.v[277]);
        let eq174_e2193_d_n0: f64 = (p.p7 * s.dn[277][0]);
        let eq174_e2193_d_n1: f64 = (p.p7 * s.dn[277][1]);
        let eq174_e2193_d_n2: f64 = (p.p7 * s.dn[277][2]);
        let eq174_e2193_d_n3: f64 = (p.p7 * s.dn[277][3]);
        let eq174_e2193_d_n4: f64 = (p.p7 * s.dn[277][4]);
        let eq174_e2193_d_n5: f64 = (p.p7 * s.dn[277][5]);
        let eq174_e2193_d_n6: f64 = (p.p7 * s.dn[277][6]);
        let eq174_e2193_d_n7: f64 = (p.p7 * s.dn[277][7]);
        let eq174_e2193_d_n8: f64 = (p.p7 * s.dn[277][8]);
        let eq174_e2193_d_n9: f64 = (p.p7 * s.dn[277][9]);
        let eq174_e2193_d_n10: f64 = (p.p7 * s.dn[277][10]);
        let eq174_e2193_d_n11: f64 = (p.p7 * s.dn[277][11]);
        let eq174_e2193_d_n12: f64 = (p.p7 * s.dn[277][12]);
        let eq174_e2193_d_n13: f64 = (p.p7 * s.dn[277][13]);
        let eq174_e2193_d_n14: f64 = (p.p7 * s.dn[277][14]);
        let eq174_e2193_d_n15: f64 = (p.p7 * s.dn[277][15]);
        let eq174_e2193_d_n16: f64 = (p.p7 * s.dn[277][16]);
        let eq174_e2193_d_n17: f64 = (p.p7 * s.dn[277][17]);
        let eq174_e2193_d_n18: f64 = (p.p7 * s.dn[277][18]);
        let eq174_e2193_d_n19: f64 = (p.p7 * s.dn[277][19]);
        let eq174_e2193_d_n20: f64 = (p.p7 * s.dn[277][20]);
        let eq174_e2193_d_n21: f64 = (p.p7 * s.dn[277][21]);
        let eq174_e2193_d_n22: f64 = (p.p7 * s.dn[277][22]);
        let eq174_e2193_q: f64 = (p.p7 * eq174_e2192_q);
        let eq174_e2193_q_d_n0: f64 = (p.p7 * s.dn[277][0]);
        let eq174_e2193_q_d_n1: f64 = (p.p7 * s.dn[277][1]);
        let eq174_e2193_q_d_n2: f64 = (p.p7 * s.dn[277][2]);
        let eq174_e2193_q_d_n3: f64 = (p.p7 * s.dn[277][3]);
        let eq174_e2193_q_d_n4: f64 = (p.p7 * s.dn[277][4]);
        let eq174_e2193_q_d_n5: f64 = (p.p7 * s.dn[277][5]);
        let eq174_e2193_q_d_n6: f64 = (p.p7 * s.dn[277][6]);
        let eq174_e2193_q_d_n7: f64 = (p.p7 * s.dn[277][7]);
        let eq174_e2193_q_d_n8: f64 = (p.p7 * s.dn[277][8]);
        let eq174_e2193_q_d_n9: f64 = (p.p7 * s.dn[277][9]);
        let eq174_e2193_q_d_n10: f64 = (p.p7 * s.dn[277][10]);
        let eq174_e2193_q_d_n11: f64 = (p.p7 * s.dn[277][11]);
        let eq174_e2193_q_d_n12: f64 = (p.p7 * s.dn[277][12]);
        let eq174_e2193_q_d_n13: f64 = (p.p7 * s.dn[277][13]);
        let eq174_e2193_q_d_n14: f64 = (p.p7 * s.dn[277][14]);
        let eq174_e2193_q_d_n15: f64 = (p.p7 * s.dn[277][15]);
        let eq174_e2193_q_d_n16: f64 = (p.p7 * s.dn[277][16]);
        let eq174_e2193_q_d_n17: f64 = (p.p7 * s.dn[277][17]);
        let eq174_e2193_q_d_n18: f64 = (p.p7 * s.dn[277][18]);
        let eq174_e2193_q_d_n19: f64 = (p.p7 * s.dn[277][19]);
        let eq174_e2193_q_d_n20: f64 = (p.p7 * s.dn[277][20]);
        let eq174_e2193_q_d_n21: f64 = (p.p7 * s.dn[277][21]);
        let eq174_e2193_q_d_n22: f64 = (p.p7 * s.dn[277][22]);
        (eq174_e2193, eq174_e2193_d_n0, eq174_e2193_d_n1, eq174_e2193_d_n2, eq174_e2193_d_n3, eq174_e2193_d_n4, eq174_e2193_d_n5, eq174_e2193_d_n6, eq174_e2193_d_n7, eq174_e2193_d_n8, eq174_e2193_d_n9, eq174_e2193_d_n10, eq174_e2193_d_n11, eq174_e2193_d_n12, eq174_e2193_d_n13, eq174_e2193_d_n14, eq174_e2193_d_n15, eq174_e2193_d_n16, eq174_e2193_d_n17, eq174_e2193_d_n18, eq174_e2193_d_n19, eq174_e2193_d_n20, eq174_e2193_d_n21, eq174_e2193_d_n22, eq174_e2193_q, eq174_e2193_q_d_n0, eq174_e2193_q_d_n1, eq174_e2193_q_d_n2, eq174_e2193_q_d_n3, eq174_e2193_q_d_n4, eq174_e2193_q_d_n5, eq174_e2193_q_d_n6, eq174_e2193_q_d_n7, eq174_e2193_q_d_n8, eq174_e2193_q_d_n9, eq174_e2193_q_d_n10, eq174_e2193_q_d_n11, eq174_e2193_q_d_n12, eq174_e2193_q_d_n13, eq174_e2193_q_d_n14, eq174_e2193_q_d_n15, eq174_e2193_q_d_n16, eq174_e2193_q_d_n17, eq174_e2193_q_d_n18, eq174_e2193_q_d_n19, eq174_e2193_q_d_n20, eq174_e2193_q_d_n21, eq174_e2193_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq174_reactive_node_derivatives: [f64; 23] = [eq174_e2195_q_d_n0, eq174_e2195_q_d_n1, eq174_e2195_q_d_n2, eq174_e2195_q_d_n3, eq174_e2195_q_d_n4, eq174_e2195_q_d_n5, eq174_e2195_q_d_n6, eq174_e2195_q_d_n7, eq174_e2195_q_d_n8, eq174_e2195_q_d_n9, eq174_e2195_q_d_n10, eq174_e2195_q_d_n11, eq174_e2195_q_d_n12, eq174_e2195_q_d_n13, eq174_e2195_q_d_n14, eq174_e2195_q_d_n15, eq174_e2195_q_d_n16, eq174_e2195_q_d_n17, eq174_e2195_q_d_n18, eq174_e2195_q_d_n19, eq174_e2195_q_d_n20, eq174_e2195_q_d_n21, eq174_e2195_q_d_n22];
        let eq174_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq174_reactive_node_derivatives,
            branches,
            &eq174_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq175_e2207, eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22, eq175_e2207_q, eq175_e2207_q_d_n0, eq175_e2207_q_d_n1, eq175_e2207_q_d_n2, eq175_e2207_q_d_n3, eq175_e2207_q_d_n4, eq175_e2207_q_d_n5, eq175_e2207_q_d_n6, eq175_e2207_q_d_n7, eq175_e2207_q_d_n8, eq175_e2207_q_d_n9, eq175_e2207_q_d_n10, eq175_e2207_q_d_n11, eq175_e2207_q_d_n12, eq175_e2207_q_d_n13, eq175_e2207_q_d_n14, eq175_e2207_q_d_n15, eq175_e2207_q_d_n16, eq175_e2207_q_d_n17, eq175_e2207_q_d_n18, eq175_e2207_q_d_n19, eq175_e2207_q_d_n20, eq175_e2207_q_d_n21, eq175_e2207_q_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && s.b[594]) {
        let eq175_e2204_q: f64 = s.v[276];
        let eq175_e2205: f64 = (p.p7 * s.v[276]);
        let eq175_e2205_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq175_e2205_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq175_e2205_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq175_e2205_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq175_e2205_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq175_e2205_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq175_e2205_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq175_e2205_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq175_e2205_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq175_e2205_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq175_e2205_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq175_e2205_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq175_e2205_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq175_e2205_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq175_e2205_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq175_e2205_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq175_e2205_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq175_e2205_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq175_e2205_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq175_e2205_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq175_e2205_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq175_e2205_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq175_e2205_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq175_e2205_q: f64 = (p.p7 * eq175_e2204_q);
        let eq175_e2205_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq175_e2205_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq175_e2205_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq175_e2205_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq175_e2205_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq175_e2205_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq175_e2205_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq175_e2205_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq175_e2205_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq175_e2205_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq175_e2205_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq175_e2205_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq175_e2205_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq175_e2205_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq175_e2205_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq175_e2205_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq175_e2205_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq175_e2205_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq175_e2205_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq175_e2205_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq175_e2205_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq175_e2205_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq175_e2205_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        (eq175_e2205, eq175_e2205_d_n0, eq175_e2205_d_n1, eq175_e2205_d_n2, eq175_e2205_d_n3, eq175_e2205_d_n4, eq175_e2205_d_n5, eq175_e2205_d_n6, eq175_e2205_d_n7, eq175_e2205_d_n8, eq175_e2205_d_n9, eq175_e2205_d_n10, eq175_e2205_d_n11, eq175_e2205_d_n12, eq175_e2205_d_n13, eq175_e2205_d_n14, eq175_e2205_d_n15, eq175_e2205_d_n16, eq175_e2205_d_n17, eq175_e2205_d_n18, eq175_e2205_d_n19, eq175_e2205_d_n20, eq175_e2205_d_n21, eq175_e2205_d_n22, eq175_e2205_q, eq175_e2205_q_d_n0, eq175_e2205_q_d_n1, eq175_e2205_q_d_n2, eq175_e2205_q_d_n3, eq175_e2205_q_d_n4, eq175_e2205_q_d_n5, eq175_e2205_q_d_n6, eq175_e2205_q_d_n7, eq175_e2205_q_d_n8, eq175_e2205_q_d_n9, eq175_e2205_q_d_n10, eq175_e2205_q_d_n11, eq175_e2205_q_d_n12, eq175_e2205_q_d_n13, eq175_e2205_q_d_n14, eq175_e2205_q_d_n15, eq175_e2205_q_d_n16, eq175_e2205_q_d_n17, eq175_e2205_q_d_n18, eq175_e2205_q_d_n19, eq175_e2205_q_d_n20, eq175_e2205_q_d_n21, eq175_e2205_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq175_reactive_node_derivatives: [f64; 23] = [eq175_e2207_q_d_n0, eq175_e2207_q_d_n1, eq175_e2207_q_d_n2, eq175_e2207_q_d_n3, eq175_e2207_q_d_n4, eq175_e2207_q_d_n5, eq175_e2207_q_d_n6, eq175_e2207_q_d_n7, eq175_e2207_q_d_n8, eq175_e2207_q_d_n9, eq175_e2207_q_d_n10, eq175_e2207_q_d_n11, eq175_e2207_q_d_n12, eq175_e2207_q_d_n13, eq175_e2207_q_d_n14, eq175_e2207_q_d_n15, eq175_e2207_q_d_n16, eq175_e2207_q_d_n17, eq175_e2207_q_d_n18, eq175_e2207_q_d_n19, eq175_e2207_q_d_n20, eq175_e2207_q_d_n21, eq175_e2207_q_d_n22];
        let eq175_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq175_reactive_node_derivatives,
            branches,
            &eq175_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq176_e2221, eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22, eq176_e2221_q, eq176_e2221_q_d_n0, eq176_e2221_q_d_n1, eq176_e2221_q_d_n2, eq176_e2221_q_d_n3, eq176_e2221_q_d_n4, eq176_e2221_q_d_n5, eq176_e2221_q_d_n6, eq176_e2221_q_d_n7, eq176_e2221_q_d_n8, eq176_e2221_q_d_n9, eq176_e2221_q_d_n10, eq176_e2221_q_d_n11, eq176_e2221_q_d_n12, eq176_e2221_q_d_n13, eq176_e2221_q_d_n14, eq176_e2221_q_d_n15, eq176_e2221_q_d_n16, eq176_e2221_q_d_n17, eq176_e2221_q_d_n18, eq176_e2221_q_d_n19, eq176_e2221_q_d_n20, eq176_e2221_q_d_n21, eq176_e2221_q_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && s.b[594]) {
        let eq176_e2216_q: f64 = s.v[276];
        let eq176_e2217: f64 = (p.p7 * s.v[276]);
        let eq176_e2217_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq176_e2217_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq176_e2217_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq176_e2217_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq176_e2217_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq176_e2217_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq176_e2217_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq176_e2217_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq176_e2217_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq176_e2217_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq176_e2217_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq176_e2217_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq176_e2217_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq176_e2217_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq176_e2217_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq176_e2217_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq176_e2217_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq176_e2217_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq176_e2217_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq176_e2217_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq176_e2217_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq176_e2217_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq176_e2217_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq176_e2217_q: f64 = (p.p7 * eq176_e2216_q);
        let eq176_e2217_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq176_e2217_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq176_e2217_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq176_e2217_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq176_e2217_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq176_e2217_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq176_e2217_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq176_e2217_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq176_e2217_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq176_e2217_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq176_e2217_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq176_e2217_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq176_e2217_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq176_e2217_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq176_e2217_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq176_e2217_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq176_e2217_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq176_e2217_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq176_e2217_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq176_e2217_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq176_e2217_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq176_e2217_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq176_e2217_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq176_e2219: f64 = (eq176_e2217 * p.p248);
        let eq176_e2219_d_n0: f64 = (eq176_e2217_d_n0 * p.p248);
        let eq176_e2219_d_n1: f64 = (eq176_e2217_d_n1 * p.p248);
        let eq176_e2219_d_n2: f64 = (eq176_e2217_d_n2 * p.p248);
        let eq176_e2219_d_n3: f64 = (eq176_e2217_d_n3 * p.p248);
        let eq176_e2219_d_n4: f64 = (eq176_e2217_d_n4 * p.p248);
        let eq176_e2219_d_n5: f64 = (eq176_e2217_d_n5 * p.p248);
        let eq176_e2219_d_n6: f64 = (eq176_e2217_d_n6 * p.p248);
        let eq176_e2219_d_n7: f64 = (eq176_e2217_d_n7 * p.p248);
        let eq176_e2219_d_n8: f64 = (eq176_e2217_d_n8 * p.p248);
        let eq176_e2219_d_n9: f64 = (eq176_e2217_d_n9 * p.p248);
        let eq176_e2219_d_n10: f64 = (eq176_e2217_d_n10 * p.p248);
        let eq176_e2219_d_n11: f64 = (eq176_e2217_d_n11 * p.p248);
        let eq176_e2219_d_n12: f64 = (eq176_e2217_d_n12 * p.p248);
        let eq176_e2219_d_n13: f64 = (eq176_e2217_d_n13 * p.p248);
        let eq176_e2219_d_n14: f64 = (eq176_e2217_d_n14 * p.p248);
        let eq176_e2219_d_n15: f64 = (eq176_e2217_d_n15 * p.p248);
        let eq176_e2219_d_n16: f64 = (eq176_e2217_d_n16 * p.p248);
        let eq176_e2219_d_n17: f64 = (eq176_e2217_d_n17 * p.p248);
        let eq176_e2219_d_n18: f64 = (eq176_e2217_d_n18 * p.p248);
        let eq176_e2219_d_n19: f64 = (eq176_e2217_d_n19 * p.p248);
        let eq176_e2219_d_n20: f64 = (eq176_e2217_d_n20 * p.p248);
        let eq176_e2219_d_n21: f64 = (eq176_e2217_d_n21 * p.p248);
        let eq176_e2219_d_n22: f64 = (eq176_e2217_d_n22 * p.p248);
        let eq176_e2219_q: f64 = (eq176_e2217_q * p.p248);
        let eq176_e2219_q_d_n0: f64 = (eq176_e2217_q_d_n0 * p.p248);
        let eq176_e2219_q_d_n1: f64 = (eq176_e2217_q_d_n1 * p.p248);
        let eq176_e2219_q_d_n2: f64 = (eq176_e2217_q_d_n2 * p.p248);
        let eq176_e2219_q_d_n3: f64 = (eq176_e2217_q_d_n3 * p.p248);
        let eq176_e2219_q_d_n4: f64 = (eq176_e2217_q_d_n4 * p.p248);
        let eq176_e2219_q_d_n5: f64 = (eq176_e2217_q_d_n5 * p.p248);
        let eq176_e2219_q_d_n6: f64 = (eq176_e2217_q_d_n6 * p.p248);
        let eq176_e2219_q_d_n7: f64 = (eq176_e2217_q_d_n7 * p.p248);
        let eq176_e2219_q_d_n8: f64 = (eq176_e2217_q_d_n8 * p.p248);
        let eq176_e2219_q_d_n9: f64 = (eq176_e2217_q_d_n9 * p.p248);
        let eq176_e2219_q_d_n10: f64 = (eq176_e2217_q_d_n10 * p.p248);
        let eq176_e2219_q_d_n11: f64 = (eq176_e2217_q_d_n11 * p.p248);
        let eq176_e2219_q_d_n12: f64 = (eq176_e2217_q_d_n12 * p.p248);
        let eq176_e2219_q_d_n13: f64 = (eq176_e2217_q_d_n13 * p.p248);
        let eq176_e2219_q_d_n14: f64 = (eq176_e2217_q_d_n14 * p.p248);
        let eq176_e2219_q_d_n15: f64 = (eq176_e2217_q_d_n15 * p.p248);
        let eq176_e2219_q_d_n16: f64 = (eq176_e2217_q_d_n16 * p.p248);
        let eq176_e2219_q_d_n17: f64 = (eq176_e2217_q_d_n17 * p.p248);
        let eq176_e2219_q_d_n18: f64 = (eq176_e2217_q_d_n18 * p.p248);
        let eq176_e2219_q_d_n19: f64 = (eq176_e2217_q_d_n19 * p.p248);
        let eq176_e2219_q_d_n20: f64 = (eq176_e2217_q_d_n20 * p.p248);
        let eq176_e2219_q_d_n21: f64 = (eq176_e2217_q_d_n21 * p.p248);
        let eq176_e2219_q_d_n22: f64 = (eq176_e2217_q_d_n22 * p.p248);
        (eq176_e2219, eq176_e2219_d_n0, eq176_e2219_d_n1, eq176_e2219_d_n2, eq176_e2219_d_n3, eq176_e2219_d_n4, eq176_e2219_d_n5, eq176_e2219_d_n6, eq176_e2219_d_n7, eq176_e2219_d_n8, eq176_e2219_d_n9, eq176_e2219_d_n10, eq176_e2219_d_n11, eq176_e2219_d_n12, eq176_e2219_d_n13, eq176_e2219_d_n14, eq176_e2219_d_n15, eq176_e2219_d_n16, eq176_e2219_d_n17, eq176_e2219_d_n18, eq176_e2219_d_n19, eq176_e2219_d_n20, eq176_e2219_d_n21, eq176_e2219_d_n22, eq176_e2219_q, eq176_e2219_q_d_n0, eq176_e2219_q_d_n1, eq176_e2219_q_d_n2, eq176_e2219_q_d_n3, eq176_e2219_q_d_n4, eq176_e2219_q_d_n5, eq176_e2219_q_d_n6, eq176_e2219_q_d_n7, eq176_e2219_q_d_n8, eq176_e2219_q_d_n9, eq176_e2219_q_d_n10, eq176_e2219_q_d_n11, eq176_e2219_q_d_n12, eq176_e2219_q_d_n13, eq176_e2219_q_d_n14, eq176_e2219_q_d_n15, eq176_e2219_q_d_n16, eq176_e2219_q_d_n17, eq176_e2219_q_d_n18, eq176_e2219_q_d_n19, eq176_e2219_q_d_n20, eq176_e2219_q_d_n21, eq176_e2219_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq176_reactive_node_derivatives: [f64; 23] = [eq176_e2221_q_d_n0, eq176_e2221_q_d_n1, eq176_e2221_q_d_n2, eq176_e2221_q_d_n3, eq176_e2221_q_d_n4, eq176_e2221_q_d_n5, eq176_e2221_q_d_n6, eq176_e2221_q_d_n7, eq176_e2221_q_d_n8, eq176_e2221_q_d_n9, eq176_e2221_q_d_n10, eq176_e2221_q_d_n11, eq176_e2221_q_d_n12, eq176_e2221_q_d_n13, eq176_e2221_q_d_n14, eq176_e2221_q_d_n15, eq176_e2221_q_d_n16, eq176_e2221_q_d_n17, eq176_e2221_q_d_n18, eq176_e2221_q_d_n19, eq176_e2221_q_d_n20, eq176_e2221_q_d_n21, eq176_e2221_q_d_n22];
        let eq176_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq176_reactive_node_derivatives,
            branches,
            &eq176_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq177_e2234, eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22, eq177_e2234_q, eq177_e2234_q_d_n0, eq177_e2234_q_d_n1, eq177_e2234_q_d_n2, eq177_e2234_q_d_n3, eq177_e2234_q_d_n4, eq177_e2234_q_d_n5, eq177_e2234_q_d_n6, eq177_e2234_q_d_n7, eq177_e2234_q_d_n8, eq177_e2234_q_d_n9, eq177_e2234_q_d_n10, eq177_e2234_q_d_n11, eq177_e2234_q_d_n12, eq177_e2234_q_d_n13, eq177_e2234_q_d_n14, eq177_e2234_q_d_n15, eq177_e2234_q_d_n16, eq177_e2234_q_d_n17, eq177_e2234_q_d_n18, eq177_e2234_q_d_n19, eq177_e2234_q_d_n20, eq177_e2234_q_d_n21, eq177_e2234_q_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && (!s.b[594])) {
        let eq177_e2231_q: f64 = s.v[276];
        let eq177_e2232: f64 = (p.p7 * s.v[276]);
        let eq177_e2232_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq177_e2232_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq177_e2232_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq177_e2232_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq177_e2232_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq177_e2232_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq177_e2232_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq177_e2232_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq177_e2232_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq177_e2232_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq177_e2232_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq177_e2232_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq177_e2232_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq177_e2232_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq177_e2232_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq177_e2232_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq177_e2232_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq177_e2232_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq177_e2232_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq177_e2232_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq177_e2232_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq177_e2232_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq177_e2232_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq177_e2232_q: f64 = (p.p7 * eq177_e2231_q);
        let eq177_e2232_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq177_e2232_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq177_e2232_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq177_e2232_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq177_e2232_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq177_e2232_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq177_e2232_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq177_e2232_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq177_e2232_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq177_e2232_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq177_e2232_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq177_e2232_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq177_e2232_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq177_e2232_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq177_e2232_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq177_e2232_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq177_e2232_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq177_e2232_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq177_e2232_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq177_e2232_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq177_e2232_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq177_e2232_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq177_e2232_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        (eq177_e2232, eq177_e2232_d_n0, eq177_e2232_d_n1, eq177_e2232_d_n2, eq177_e2232_d_n3, eq177_e2232_d_n4, eq177_e2232_d_n5, eq177_e2232_d_n6, eq177_e2232_d_n7, eq177_e2232_d_n8, eq177_e2232_d_n9, eq177_e2232_d_n10, eq177_e2232_d_n11, eq177_e2232_d_n12, eq177_e2232_d_n13, eq177_e2232_d_n14, eq177_e2232_d_n15, eq177_e2232_d_n16, eq177_e2232_d_n17, eq177_e2232_d_n18, eq177_e2232_d_n19, eq177_e2232_d_n20, eq177_e2232_d_n21, eq177_e2232_d_n22, eq177_e2232_q, eq177_e2232_q_d_n0, eq177_e2232_q_d_n1, eq177_e2232_q_d_n2, eq177_e2232_q_d_n3, eq177_e2232_q_d_n4, eq177_e2232_q_d_n5, eq177_e2232_q_d_n6, eq177_e2232_q_d_n7, eq177_e2232_q_d_n8, eq177_e2232_q_d_n9, eq177_e2232_q_d_n10, eq177_e2232_q_d_n11, eq177_e2232_q_d_n12, eq177_e2232_q_d_n13, eq177_e2232_q_d_n14, eq177_e2232_q_d_n15, eq177_e2232_q_d_n16, eq177_e2232_q_d_n17, eq177_e2232_q_d_n18, eq177_e2232_q_d_n19, eq177_e2232_q_d_n20, eq177_e2232_q_d_n21, eq177_e2232_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq177_reactive_node_derivatives: [f64; 23] = [eq177_e2234_q_d_n0, eq177_e2234_q_d_n1, eq177_e2234_q_d_n2, eq177_e2234_q_d_n3, eq177_e2234_q_d_n4, eq177_e2234_q_d_n5, eq177_e2234_q_d_n6, eq177_e2234_q_d_n7, eq177_e2234_q_d_n8, eq177_e2234_q_d_n9, eq177_e2234_q_d_n10, eq177_e2234_q_d_n11, eq177_e2234_q_d_n12, eq177_e2234_q_d_n13, eq177_e2234_q_d_n14, eq177_e2234_q_d_n15, eq177_e2234_q_d_n16, eq177_e2234_q_d_n17, eq177_e2234_q_d_n18, eq177_e2234_q_d_n19, eq177_e2234_q_d_n20, eq177_e2234_q_d_n21, eq177_e2234_q_d_n22];
        let eq177_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq177_reactive_node_derivatives,
            branches,
            &eq177_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_13(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq178_e2249, eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22, eq178_e2249_q, eq178_e2249_q_d_n0, eq178_e2249_q_d_n1, eq178_e2249_q_d_n2, eq178_e2249_q_d_n3, eq178_e2249_q_d_n4, eq178_e2249_q_d_n5, eq178_e2249_q_d_n6, eq178_e2249_q_d_n7, eq178_e2249_q_d_n8, eq178_e2249_q_d_n9, eq178_e2249_q_d_n10, eq178_e2249_q_d_n11, eq178_e2249_q_d_n12, eq178_e2249_q_d_n13, eq178_e2249_q_d_n14, eq178_e2249_q_d_n15, eq178_e2249_q_d_n16, eq178_e2249_q_d_n17, eq178_e2249_q_d_n18, eq178_e2249_q_d_n19, eq178_e2249_q_d_n20, eq178_e2249_q_d_n21, eq178_e2249_q_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && (!s.b[594])) {
        let eq178_e2244_q: f64 = s.v[276];
        let eq178_e2245: f64 = (p.p7 * s.v[276]);
        let eq178_e2245_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq178_e2245_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq178_e2245_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq178_e2245_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq178_e2245_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq178_e2245_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq178_e2245_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq178_e2245_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq178_e2245_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq178_e2245_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq178_e2245_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq178_e2245_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq178_e2245_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq178_e2245_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq178_e2245_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq178_e2245_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq178_e2245_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq178_e2245_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq178_e2245_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq178_e2245_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq178_e2245_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq178_e2245_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq178_e2245_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq178_e2245_q: f64 = (p.p7 * eq178_e2244_q);
        let eq178_e2245_q_d_n0: f64 = (p.p7 * s.dn[276][0]);
        let eq178_e2245_q_d_n1: f64 = (p.p7 * s.dn[276][1]);
        let eq178_e2245_q_d_n2: f64 = (p.p7 * s.dn[276][2]);
        let eq178_e2245_q_d_n3: f64 = (p.p7 * s.dn[276][3]);
        let eq178_e2245_q_d_n4: f64 = (p.p7 * s.dn[276][4]);
        let eq178_e2245_q_d_n5: f64 = (p.p7 * s.dn[276][5]);
        let eq178_e2245_q_d_n6: f64 = (p.p7 * s.dn[276][6]);
        let eq178_e2245_q_d_n7: f64 = (p.p7 * s.dn[276][7]);
        let eq178_e2245_q_d_n8: f64 = (p.p7 * s.dn[276][8]);
        let eq178_e2245_q_d_n9: f64 = (p.p7 * s.dn[276][9]);
        let eq178_e2245_q_d_n10: f64 = (p.p7 * s.dn[276][10]);
        let eq178_e2245_q_d_n11: f64 = (p.p7 * s.dn[276][11]);
        let eq178_e2245_q_d_n12: f64 = (p.p7 * s.dn[276][12]);
        let eq178_e2245_q_d_n13: f64 = (p.p7 * s.dn[276][13]);
        let eq178_e2245_q_d_n14: f64 = (p.p7 * s.dn[276][14]);
        let eq178_e2245_q_d_n15: f64 = (p.p7 * s.dn[276][15]);
        let eq178_e2245_q_d_n16: f64 = (p.p7 * s.dn[276][16]);
        let eq178_e2245_q_d_n17: f64 = (p.p7 * s.dn[276][17]);
        let eq178_e2245_q_d_n18: f64 = (p.p7 * s.dn[276][18]);
        let eq178_e2245_q_d_n19: f64 = (p.p7 * s.dn[276][19]);
        let eq178_e2245_q_d_n20: f64 = (p.p7 * s.dn[276][20]);
        let eq178_e2245_q_d_n21: f64 = (p.p7 * s.dn[276][21]);
        let eq178_e2245_q_d_n22: f64 = (p.p7 * s.dn[276][22]);
        let eq178_e2247: f64 = (eq178_e2245 * p.p248);
        let eq178_e2247_d_n0: f64 = (eq178_e2245_d_n0 * p.p248);
        let eq178_e2247_d_n1: f64 = (eq178_e2245_d_n1 * p.p248);
        let eq178_e2247_d_n2: f64 = (eq178_e2245_d_n2 * p.p248);
        let eq178_e2247_d_n3: f64 = (eq178_e2245_d_n3 * p.p248);
        let eq178_e2247_d_n4: f64 = (eq178_e2245_d_n4 * p.p248);
        let eq178_e2247_d_n5: f64 = (eq178_e2245_d_n5 * p.p248);
        let eq178_e2247_d_n6: f64 = (eq178_e2245_d_n6 * p.p248);
        let eq178_e2247_d_n7: f64 = (eq178_e2245_d_n7 * p.p248);
        let eq178_e2247_d_n8: f64 = (eq178_e2245_d_n8 * p.p248);
        let eq178_e2247_d_n9: f64 = (eq178_e2245_d_n9 * p.p248);
        let eq178_e2247_d_n10: f64 = (eq178_e2245_d_n10 * p.p248);
        let eq178_e2247_d_n11: f64 = (eq178_e2245_d_n11 * p.p248);
        let eq178_e2247_d_n12: f64 = (eq178_e2245_d_n12 * p.p248);
        let eq178_e2247_d_n13: f64 = (eq178_e2245_d_n13 * p.p248);
        let eq178_e2247_d_n14: f64 = (eq178_e2245_d_n14 * p.p248);
        let eq178_e2247_d_n15: f64 = (eq178_e2245_d_n15 * p.p248);
        let eq178_e2247_d_n16: f64 = (eq178_e2245_d_n16 * p.p248);
        let eq178_e2247_d_n17: f64 = (eq178_e2245_d_n17 * p.p248);
        let eq178_e2247_d_n18: f64 = (eq178_e2245_d_n18 * p.p248);
        let eq178_e2247_d_n19: f64 = (eq178_e2245_d_n19 * p.p248);
        let eq178_e2247_d_n20: f64 = (eq178_e2245_d_n20 * p.p248);
        let eq178_e2247_d_n21: f64 = (eq178_e2245_d_n21 * p.p248);
        let eq178_e2247_d_n22: f64 = (eq178_e2245_d_n22 * p.p248);
        let eq178_e2247_q: f64 = (eq178_e2245_q * p.p248);
        let eq178_e2247_q_d_n0: f64 = (eq178_e2245_q_d_n0 * p.p248);
        let eq178_e2247_q_d_n1: f64 = (eq178_e2245_q_d_n1 * p.p248);
        let eq178_e2247_q_d_n2: f64 = (eq178_e2245_q_d_n2 * p.p248);
        let eq178_e2247_q_d_n3: f64 = (eq178_e2245_q_d_n3 * p.p248);
        let eq178_e2247_q_d_n4: f64 = (eq178_e2245_q_d_n4 * p.p248);
        let eq178_e2247_q_d_n5: f64 = (eq178_e2245_q_d_n5 * p.p248);
        let eq178_e2247_q_d_n6: f64 = (eq178_e2245_q_d_n6 * p.p248);
        let eq178_e2247_q_d_n7: f64 = (eq178_e2245_q_d_n7 * p.p248);
        let eq178_e2247_q_d_n8: f64 = (eq178_e2245_q_d_n8 * p.p248);
        let eq178_e2247_q_d_n9: f64 = (eq178_e2245_q_d_n9 * p.p248);
        let eq178_e2247_q_d_n10: f64 = (eq178_e2245_q_d_n10 * p.p248);
        let eq178_e2247_q_d_n11: f64 = (eq178_e2245_q_d_n11 * p.p248);
        let eq178_e2247_q_d_n12: f64 = (eq178_e2245_q_d_n12 * p.p248);
        let eq178_e2247_q_d_n13: f64 = (eq178_e2245_q_d_n13 * p.p248);
        let eq178_e2247_q_d_n14: f64 = (eq178_e2245_q_d_n14 * p.p248);
        let eq178_e2247_q_d_n15: f64 = (eq178_e2245_q_d_n15 * p.p248);
        let eq178_e2247_q_d_n16: f64 = (eq178_e2245_q_d_n16 * p.p248);
        let eq178_e2247_q_d_n17: f64 = (eq178_e2245_q_d_n17 * p.p248);
        let eq178_e2247_q_d_n18: f64 = (eq178_e2245_q_d_n18 * p.p248);
        let eq178_e2247_q_d_n19: f64 = (eq178_e2245_q_d_n19 * p.p248);
        let eq178_e2247_q_d_n20: f64 = (eq178_e2245_q_d_n20 * p.p248);
        let eq178_e2247_q_d_n21: f64 = (eq178_e2245_q_d_n21 * p.p248);
        let eq178_e2247_q_d_n22: f64 = (eq178_e2245_q_d_n22 * p.p248);
        (eq178_e2247, eq178_e2247_d_n0, eq178_e2247_d_n1, eq178_e2247_d_n2, eq178_e2247_d_n3, eq178_e2247_d_n4, eq178_e2247_d_n5, eq178_e2247_d_n6, eq178_e2247_d_n7, eq178_e2247_d_n8, eq178_e2247_d_n9, eq178_e2247_d_n10, eq178_e2247_d_n11, eq178_e2247_d_n12, eq178_e2247_d_n13, eq178_e2247_d_n14, eq178_e2247_d_n15, eq178_e2247_d_n16, eq178_e2247_d_n17, eq178_e2247_d_n18, eq178_e2247_d_n19, eq178_e2247_d_n20, eq178_e2247_d_n21, eq178_e2247_d_n22, eq178_e2247_q, eq178_e2247_q_d_n0, eq178_e2247_q_d_n1, eq178_e2247_q_d_n2, eq178_e2247_q_d_n3, eq178_e2247_q_d_n4, eq178_e2247_q_d_n5, eq178_e2247_q_d_n6, eq178_e2247_q_d_n7, eq178_e2247_q_d_n8, eq178_e2247_q_d_n9, eq178_e2247_q_d_n10, eq178_e2247_q_d_n11, eq178_e2247_q_d_n12, eq178_e2247_q_d_n13, eq178_e2247_q_d_n14, eq178_e2247_q_d_n15, eq178_e2247_q_d_n16, eq178_e2247_q_d_n17, eq178_e2247_q_d_n18, eq178_e2247_q_d_n19, eq178_e2247_q_d_n20, eq178_e2247_q_d_n21, eq178_e2247_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq178_reactive_node_derivatives: [f64; 23] = [eq178_e2249_q_d_n0, eq178_e2249_q_d_n1, eq178_e2249_q_d_n2, eq178_e2249_q_d_n3, eq178_e2249_q_d_n4, eq178_e2249_q_d_n5, eq178_e2249_q_d_n6, eq178_e2249_q_d_n7, eq178_e2249_q_d_n8, eq178_e2249_q_d_n9, eq178_e2249_q_d_n10, eq178_e2249_q_d_n11, eq178_e2249_q_d_n12, eq178_e2249_q_d_n13, eq178_e2249_q_d_n14, eq178_e2249_q_d_n15, eq178_e2249_q_d_n16, eq178_e2249_q_d_n17, eq178_e2249_q_d_n18, eq178_e2249_q_d_n19, eq178_e2249_q_d_n20, eq178_e2249_q_d_n21, eq178_e2249_q_d_n22];
        let eq178_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq178_reactive_node_derivatives,
            branches,
            &eq178_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq179_e2261, eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22, eq179_e2261_q, eq179_e2261_q_d_n0, eq179_e2261_q_d_n1, eq179_e2261_q_d_n2, eq179_e2261_q_d_n3, eq179_e2261_q_d_n4, eq179_e2261_q_d_n5, eq179_e2261_q_d_n6, eq179_e2261_q_d_n7, eq179_e2261_q_d_n8, eq179_e2261_q_d_n9, eq179_e2261_q_d_n10, eq179_e2261_q_d_n11, eq179_e2261_q_d_n12, eq179_e2261_q_d_n13, eq179_e2261_q_d_n14, eq179_e2261_q_d_n15, eq179_e2261_q_d_n16, eq179_e2261_q_d_n17, eq179_e2261_q_d_n18, eq179_e2261_q_d_n19, eq179_e2261_q_d_n20, eq179_e2261_q_d_n21, eq179_e2261_q_d_n22,) = {
    if ((!s.b[590]) && s.b[593]) {
        let eq179_e2257: f64 = (p.p253 * s.v[276]);
        let eq179_e2257_d_n0: f64 = (p.p253 * s.dn[276][0]);
        let eq179_e2257_d_n1: f64 = (p.p253 * s.dn[276][1]);
        let eq179_e2257_d_n2: f64 = (p.p253 * s.dn[276][2]);
        let eq179_e2257_d_n3: f64 = (p.p253 * s.dn[276][3]);
        let eq179_e2257_d_n4: f64 = (p.p253 * s.dn[276][4]);
        let eq179_e2257_d_n5: f64 = (p.p253 * s.dn[276][5]);
        let eq179_e2257_d_n6: f64 = (p.p253 * s.dn[276][6]);
        let eq179_e2257_d_n7: f64 = (p.p253 * s.dn[276][7]);
        let eq179_e2257_d_n8: f64 = (p.p253 * s.dn[276][8]);
        let eq179_e2257_d_n9: f64 = (p.p253 * s.dn[276][9]);
        let eq179_e2257_d_n10: f64 = (p.p253 * s.dn[276][10]);
        let eq179_e2257_d_n11: f64 = (p.p253 * s.dn[276][11]);
        let eq179_e2257_d_n12: f64 = (p.p253 * s.dn[276][12]);
        let eq179_e2257_d_n13: f64 = (p.p253 * s.dn[276][13]);
        let eq179_e2257_d_n14: f64 = (p.p253 * s.dn[276][14]);
        let eq179_e2257_d_n15: f64 = (p.p253 * s.dn[276][15]);
        let eq179_e2257_d_n16: f64 = (p.p253 * s.dn[276][16]);
        let eq179_e2257_d_n17: f64 = (p.p253 * s.dn[276][17]);
        let eq179_e2257_d_n18: f64 = (p.p253 * s.dn[276][18]);
        let eq179_e2257_d_n19: f64 = (p.p253 * s.dn[276][19]);
        let eq179_e2257_d_n20: f64 = (p.p253 * s.dn[276][20]);
        let eq179_e2257_d_n21: f64 = (p.p253 * s.dn[276][21]);
        let eq179_e2257_d_n22: f64 = (p.p253 * s.dn[276][22]);
        let eq179_e2258_q: f64 = eq179_e2257;
        let eq179_e2259: f64 = (p.p7 * eq179_e2257);
        let eq179_e2259_d_n0: f64 = (p.p7 * eq179_e2257_d_n0);
        let eq179_e2259_d_n1: f64 = (p.p7 * eq179_e2257_d_n1);
        let eq179_e2259_d_n2: f64 = (p.p7 * eq179_e2257_d_n2);
        let eq179_e2259_d_n3: f64 = (p.p7 * eq179_e2257_d_n3);
        let eq179_e2259_d_n4: f64 = (p.p7 * eq179_e2257_d_n4);
        let eq179_e2259_d_n5: f64 = (p.p7 * eq179_e2257_d_n5);
        let eq179_e2259_d_n6: f64 = (p.p7 * eq179_e2257_d_n6);
        let eq179_e2259_d_n7: f64 = (p.p7 * eq179_e2257_d_n7);
        let eq179_e2259_d_n8: f64 = (p.p7 * eq179_e2257_d_n8);
        let eq179_e2259_d_n9: f64 = (p.p7 * eq179_e2257_d_n9);
        let eq179_e2259_d_n10: f64 = (p.p7 * eq179_e2257_d_n10);
        let eq179_e2259_d_n11: f64 = (p.p7 * eq179_e2257_d_n11);
        let eq179_e2259_d_n12: f64 = (p.p7 * eq179_e2257_d_n12);
        let eq179_e2259_d_n13: f64 = (p.p7 * eq179_e2257_d_n13);
        let eq179_e2259_d_n14: f64 = (p.p7 * eq179_e2257_d_n14);
        let eq179_e2259_d_n15: f64 = (p.p7 * eq179_e2257_d_n15);
        let eq179_e2259_d_n16: f64 = (p.p7 * eq179_e2257_d_n16);
        let eq179_e2259_d_n17: f64 = (p.p7 * eq179_e2257_d_n17);
        let eq179_e2259_d_n18: f64 = (p.p7 * eq179_e2257_d_n18);
        let eq179_e2259_d_n19: f64 = (p.p7 * eq179_e2257_d_n19);
        let eq179_e2259_d_n20: f64 = (p.p7 * eq179_e2257_d_n20);
        let eq179_e2259_d_n21: f64 = (p.p7 * eq179_e2257_d_n21);
        let eq179_e2259_d_n22: f64 = (p.p7 * eq179_e2257_d_n22);
        let eq179_e2259_q: f64 = (p.p7 * eq179_e2258_q);
        let eq179_e2259_q_d_n0: f64 = (p.p7 * eq179_e2257_d_n0);
        let eq179_e2259_q_d_n1: f64 = (p.p7 * eq179_e2257_d_n1);
        let eq179_e2259_q_d_n2: f64 = (p.p7 * eq179_e2257_d_n2);
        let eq179_e2259_q_d_n3: f64 = (p.p7 * eq179_e2257_d_n3);
        let eq179_e2259_q_d_n4: f64 = (p.p7 * eq179_e2257_d_n4);
        let eq179_e2259_q_d_n5: f64 = (p.p7 * eq179_e2257_d_n5);
        let eq179_e2259_q_d_n6: f64 = (p.p7 * eq179_e2257_d_n6);
        let eq179_e2259_q_d_n7: f64 = (p.p7 * eq179_e2257_d_n7);
        let eq179_e2259_q_d_n8: f64 = (p.p7 * eq179_e2257_d_n8);
        let eq179_e2259_q_d_n9: f64 = (p.p7 * eq179_e2257_d_n9);
        let eq179_e2259_q_d_n10: f64 = (p.p7 * eq179_e2257_d_n10);
        let eq179_e2259_q_d_n11: f64 = (p.p7 * eq179_e2257_d_n11);
        let eq179_e2259_q_d_n12: f64 = (p.p7 * eq179_e2257_d_n12);
        let eq179_e2259_q_d_n13: f64 = (p.p7 * eq179_e2257_d_n13);
        let eq179_e2259_q_d_n14: f64 = (p.p7 * eq179_e2257_d_n14);
        let eq179_e2259_q_d_n15: f64 = (p.p7 * eq179_e2257_d_n15);
        let eq179_e2259_q_d_n16: f64 = (p.p7 * eq179_e2257_d_n16);
        let eq179_e2259_q_d_n17: f64 = (p.p7 * eq179_e2257_d_n17);
        let eq179_e2259_q_d_n18: f64 = (p.p7 * eq179_e2257_d_n18);
        let eq179_e2259_q_d_n19: f64 = (p.p7 * eq179_e2257_d_n19);
        let eq179_e2259_q_d_n20: f64 = (p.p7 * eq179_e2257_d_n20);
        let eq179_e2259_q_d_n21: f64 = (p.p7 * eq179_e2257_d_n21);
        let eq179_e2259_q_d_n22: f64 = (p.p7 * eq179_e2257_d_n22);
        (eq179_e2259, eq179_e2259_d_n0, eq179_e2259_d_n1, eq179_e2259_d_n2, eq179_e2259_d_n3, eq179_e2259_d_n4, eq179_e2259_d_n5, eq179_e2259_d_n6, eq179_e2259_d_n7, eq179_e2259_d_n8, eq179_e2259_d_n9, eq179_e2259_d_n10, eq179_e2259_d_n11, eq179_e2259_d_n12, eq179_e2259_d_n13, eq179_e2259_d_n14, eq179_e2259_d_n15, eq179_e2259_d_n16, eq179_e2259_d_n17, eq179_e2259_d_n18, eq179_e2259_d_n19, eq179_e2259_d_n20, eq179_e2259_d_n21, eq179_e2259_d_n22, eq179_e2259_q, eq179_e2259_q_d_n0, eq179_e2259_q_d_n1, eq179_e2259_q_d_n2, eq179_e2259_q_d_n3, eq179_e2259_q_d_n4, eq179_e2259_q_d_n5, eq179_e2259_q_d_n6, eq179_e2259_q_d_n7, eq179_e2259_q_d_n8, eq179_e2259_q_d_n9, eq179_e2259_q_d_n10, eq179_e2259_q_d_n11, eq179_e2259_q_d_n12, eq179_e2259_q_d_n13, eq179_e2259_q_d_n14, eq179_e2259_q_d_n15, eq179_e2259_q_d_n16, eq179_e2259_q_d_n17, eq179_e2259_q_d_n18, eq179_e2259_q_d_n19, eq179_e2259_q_d_n20, eq179_e2259_q_d_n21, eq179_e2259_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq179_reactive_node_derivatives: [f64; 23] = [eq179_e2261_q_d_n0, eq179_e2261_q_d_n1, eq179_e2261_q_d_n2, eq179_e2261_q_d_n3, eq179_e2261_q_d_n4, eq179_e2261_q_d_n5, eq179_e2261_q_d_n6, eq179_e2261_q_d_n7, eq179_e2261_q_d_n8, eq179_e2261_q_d_n9, eq179_e2261_q_d_n10, eq179_e2261_q_d_n11, eq179_e2261_q_d_n12, eq179_e2261_q_d_n13, eq179_e2261_q_d_n14, eq179_e2261_q_d_n15, eq179_e2261_q_d_n16, eq179_e2261_q_d_n17, eq179_e2261_q_d_n18, eq179_e2261_q_d_n19, eq179_e2261_q_d_n20, eq179_e2261_q_d_n21, eq179_e2261_q_d_n22];
        let eq179_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq179_reactive_node_derivatives,
            branches,
            &eq179_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq180_e2270, eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22, eq180_e2270_q, eq180_e2270_q_d_n0, eq180_e2270_q_d_n1, eq180_e2270_q_d_n2, eq180_e2270_q_d_n3, eq180_e2270_q_d_n4, eq180_e2270_q_d_n5, eq180_e2270_q_d_n6, eq180_e2270_q_d_n7, eq180_e2270_q_d_n8, eq180_e2270_q_d_n9, eq180_e2270_q_d_n10, eq180_e2270_q_d_n11, eq180_e2270_q_d_n12, eq180_e2270_q_d_n13, eq180_e2270_q_d_n14, eq180_e2270_q_d_n15, eq180_e2270_q_d_n16, eq180_e2270_q_d_n17, eq180_e2270_q_d_n18, eq180_e2270_q_d_n19, eq180_e2270_q_d_n20, eq180_e2270_q_d_n21, eq180_e2270_q_d_n22,) = {
    if (s.b[595] && s.b[596]) {
        let eq180_e2267_q: f64 = s.v[289];
        let eq180_e2268: f64 = (p.p7 * s.v[289]);
        let eq180_e2268_d_n0: f64 = (p.p7 * s.dn[289][0]);
        let eq180_e2268_d_n1: f64 = (p.p7 * s.dn[289][1]);
        let eq180_e2268_d_n2: f64 = (p.p7 * s.dn[289][2]);
        let eq180_e2268_d_n3: f64 = (p.p7 * s.dn[289][3]);
        let eq180_e2268_d_n4: f64 = (p.p7 * s.dn[289][4]);
        let eq180_e2268_d_n5: f64 = (p.p7 * s.dn[289][5]);
        let eq180_e2268_d_n6: f64 = (p.p7 * s.dn[289][6]);
        let eq180_e2268_d_n7: f64 = (p.p7 * s.dn[289][7]);
        let eq180_e2268_d_n8: f64 = (p.p7 * s.dn[289][8]);
        let eq180_e2268_d_n9: f64 = (p.p7 * s.dn[289][9]);
        let eq180_e2268_d_n10: f64 = (p.p7 * s.dn[289][10]);
        let eq180_e2268_d_n11: f64 = (p.p7 * s.dn[289][11]);
        let eq180_e2268_d_n12: f64 = (p.p7 * s.dn[289][12]);
        let eq180_e2268_d_n13: f64 = (p.p7 * s.dn[289][13]);
        let eq180_e2268_d_n14: f64 = (p.p7 * s.dn[289][14]);
        let eq180_e2268_d_n15: f64 = (p.p7 * s.dn[289][15]);
        let eq180_e2268_d_n16: f64 = (p.p7 * s.dn[289][16]);
        let eq180_e2268_d_n17: f64 = (p.p7 * s.dn[289][17]);
        let eq180_e2268_d_n18: f64 = (p.p7 * s.dn[289][18]);
        let eq180_e2268_d_n19: f64 = (p.p7 * s.dn[289][19]);
        let eq180_e2268_d_n20: f64 = (p.p7 * s.dn[289][20]);
        let eq180_e2268_d_n21: f64 = (p.p7 * s.dn[289][21]);
        let eq180_e2268_d_n22: f64 = (p.p7 * s.dn[289][22]);
        let eq180_e2268_q: f64 = (p.p7 * eq180_e2267_q);
        let eq180_e2268_q_d_n0: f64 = (p.p7 * s.dn[289][0]);
        let eq180_e2268_q_d_n1: f64 = (p.p7 * s.dn[289][1]);
        let eq180_e2268_q_d_n2: f64 = (p.p7 * s.dn[289][2]);
        let eq180_e2268_q_d_n3: f64 = (p.p7 * s.dn[289][3]);
        let eq180_e2268_q_d_n4: f64 = (p.p7 * s.dn[289][4]);
        let eq180_e2268_q_d_n5: f64 = (p.p7 * s.dn[289][5]);
        let eq180_e2268_q_d_n6: f64 = (p.p7 * s.dn[289][6]);
        let eq180_e2268_q_d_n7: f64 = (p.p7 * s.dn[289][7]);
        let eq180_e2268_q_d_n8: f64 = (p.p7 * s.dn[289][8]);
        let eq180_e2268_q_d_n9: f64 = (p.p7 * s.dn[289][9]);
        let eq180_e2268_q_d_n10: f64 = (p.p7 * s.dn[289][10]);
        let eq180_e2268_q_d_n11: f64 = (p.p7 * s.dn[289][11]);
        let eq180_e2268_q_d_n12: f64 = (p.p7 * s.dn[289][12]);
        let eq180_e2268_q_d_n13: f64 = (p.p7 * s.dn[289][13]);
        let eq180_e2268_q_d_n14: f64 = (p.p7 * s.dn[289][14]);
        let eq180_e2268_q_d_n15: f64 = (p.p7 * s.dn[289][15]);
        let eq180_e2268_q_d_n16: f64 = (p.p7 * s.dn[289][16]);
        let eq180_e2268_q_d_n17: f64 = (p.p7 * s.dn[289][17]);
        let eq180_e2268_q_d_n18: f64 = (p.p7 * s.dn[289][18]);
        let eq180_e2268_q_d_n19: f64 = (p.p7 * s.dn[289][19]);
        let eq180_e2268_q_d_n20: f64 = (p.p7 * s.dn[289][20]);
        let eq180_e2268_q_d_n21: f64 = (p.p7 * s.dn[289][21]);
        let eq180_e2268_q_d_n22: f64 = (p.p7 * s.dn[289][22]);
        (eq180_e2268, eq180_e2268_d_n0, eq180_e2268_d_n1, eq180_e2268_d_n2, eq180_e2268_d_n3, eq180_e2268_d_n4, eq180_e2268_d_n5, eq180_e2268_d_n6, eq180_e2268_d_n7, eq180_e2268_d_n8, eq180_e2268_d_n9, eq180_e2268_d_n10, eq180_e2268_d_n11, eq180_e2268_d_n12, eq180_e2268_d_n13, eq180_e2268_d_n14, eq180_e2268_d_n15, eq180_e2268_d_n16, eq180_e2268_d_n17, eq180_e2268_d_n18, eq180_e2268_d_n19, eq180_e2268_d_n20, eq180_e2268_d_n21, eq180_e2268_d_n22, eq180_e2268_q, eq180_e2268_q_d_n0, eq180_e2268_q_d_n1, eq180_e2268_q_d_n2, eq180_e2268_q_d_n3, eq180_e2268_q_d_n4, eq180_e2268_q_d_n5, eq180_e2268_q_d_n6, eq180_e2268_q_d_n7, eq180_e2268_q_d_n8, eq180_e2268_q_d_n9, eq180_e2268_q_d_n10, eq180_e2268_q_d_n11, eq180_e2268_q_d_n12, eq180_e2268_q_d_n13, eq180_e2268_q_d_n14, eq180_e2268_q_d_n15, eq180_e2268_q_d_n16, eq180_e2268_q_d_n17, eq180_e2268_q_d_n18, eq180_e2268_q_d_n19, eq180_e2268_q_d_n20, eq180_e2268_q_d_n21, eq180_e2268_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq180_reactive_node_derivatives: [f64; 23] = [eq180_e2270_q_d_n0, eq180_e2270_q_d_n1, eq180_e2270_q_d_n2, eq180_e2270_q_d_n3, eq180_e2270_q_d_n4, eq180_e2270_q_d_n5, eq180_e2270_q_d_n6, eq180_e2270_q_d_n7, eq180_e2270_q_d_n8, eq180_e2270_q_d_n9, eq180_e2270_q_d_n10, eq180_e2270_q_d_n11, eq180_e2270_q_d_n12, eq180_e2270_q_d_n13, eq180_e2270_q_d_n14, eq180_e2270_q_d_n15, eq180_e2270_q_d_n16, eq180_e2270_q_d_n17, eq180_e2270_q_d_n18, eq180_e2270_q_d_n19, eq180_e2270_q_d_n20, eq180_e2270_q_d_n21, eq180_e2270_q_d_n22];
        let eq180_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[21]),
            nodes,
            &eq180_reactive_node_derivatives,
            branches,
            &eq180_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq181_e2281, eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22, eq181_e2281_q, eq181_e2281_q_d_n0, eq181_e2281_q_d_n1, eq181_e2281_q_d_n2, eq181_e2281_q_d_n3, eq181_e2281_q_d_n4, eq181_e2281_q_d_n5, eq181_e2281_q_d_n6, eq181_e2281_q_d_n7, eq181_e2281_q_d_n8, eq181_e2281_q_d_n9, eq181_e2281_q_d_n10, eq181_e2281_q_d_n11, eq181_e2281_q_d_n12, eq181_e2281_q_d_n13, eq181_e2281_q_d_n14, eq181_e2281_q_d_n15, eq181_e2281_q_d_n16, eq181_e2281_q_d_n17, eq181_e2281_q_d_n18, eq181_e2281_q_d_n19, eq181_e2281_q_d_n20, eq181_e2281_q_d_n21, eq181_e2281_q_d_n22,) = {
    if ((s.b[595] && s.b[596]) && s.b[597]) {
        let eq181_e2278_q: f64 = s.v[288];
        let eq181_e2279: f64 = (p.p7 * s.v[288]);
        let eq181_e2279_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq181_e2279_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq181_e2279_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq181_e2279_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq181_e2279_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq181_e2279_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq181_e2279_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq181_e2279_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq181_e2279_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq181_e2279_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq181_e2279_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq181_e2279_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq181_e2279_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq181_e2279_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq181_e2279_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq181_e2279_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq181_e2279_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq181_e2279_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq181_e2279_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq181_e2279_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq181_e2279_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq181_e2279_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq181_e2279_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq181_e2279_q: f64 = (p.p7 * eq181_e2278_q);
        let eq181_e2279_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq181_e2279_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq181_e2279_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq181_e2279_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq181_e2279_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq181_e2279_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq181_e2279_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq181_e2279_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq181_e2279_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq181_e2279_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq181_e2279_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq181_e2279_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq181_e2279_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq181_e2279_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq181_e2279_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq181_e2279_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq181_e2279_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq181_e2279_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq181_e2279_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq181_e2279_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq181_e2279_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq181_e2279_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq181_e2279_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        (eq181_e2279, eq181_e2279_d_n0, eq181_e2279_d_n1, eq181_e2279_d_n2, eq181_e2279_d_n3, eq181_e2279_d_n4, eq181_e2279_d_n5, eq181_e2279_d_n6, eq181_e2279_d_n7, eq181_e2279_d_n8, eq181_e2279_d_n9, eq181_e2279_d_n10, eq181_e2279_d_n11, eq181_e2279_d_n12, eq181_e2279_d_n13, eq181_e2279_d_n14, eq181_e2279_d_n15, eq181_e2279_d_n16, eq181_e2279_d_n17, eq181_e2279_d_n18, eq181_e2279_d_n19, eq181_e2279_d_n20, eq181_e2279_d_n21, eq181_e2279_d_n22, eq181_e2279_q, eq181_e2279_q_d_n0, eq181_e2279_q_d_n1, eq181_e2279_q_d_n2, eq181_e2279_q_d_n3, eq181_e2279_q_d_n4, eq181_e2279_q_d_n5, eq181_e2279_q_d_n6, eq181_e2279_q_d_n7, eq181_e2279_q_d_n8, eq181_e2279_q_d_n9, eq181_e2279_q_d_n10, eq181_e2279_q_d_n11, eq181_e2279_q_d_n12, eq181_e2279_q_d_n13, eq181_e2279_q_d_n14, eq181_e2279_q_d_n15, eq181_e2279_q_d_n16, eq181_e2279_q_d_n17, eq181_e2279_q_d_n18, eq181_e2279_q_d_n19, eq181_e2279_q_d_n20, eq181_e2279_q_d_n21, eq181_e2279_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq181_reactive_node_derivatives: [f64; 23] = [eq181_e2281_q_d_n0, eq181_e2281_q_d_n1, eq181_e2281_q_d_n2, eq181_e2281_q_d_n3, eq181_e2281_q_d_n4, eq181_e2281_q_d_n5, eq181_e2281_q_d_n6, eq181_e2281_q_d_n7, eq181_e2281_q_d_n8, eq181_e2281_q_d_n9, eq181_e2281_q_d_n10, eq181_e2281_q_d_n11, eq181_e2281_q_d_n12, eq181_e2281_q_d_n13, eq181_e2281_q_d_n14, eq181_e2281_q_d_n15, eq181_e2281_q_d_n16, eq181_e2281_q_d_n17, eq181_e2281_q_d_n18, eq181_e2281_q_d_n19, eq181_e2281_q_d_n20, eq181_e2281_q_d_n21, eq181_e2281_q_d_n22];
        let eq181_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            nodes,
            &eq181_reactive_node_derivatives,
            branches,
            &eq181_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq182_e2294, eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22, eq182_e2294_q, eq182_e2294_q_d_n0, eq182_e2294_q_d_n1, eq182_e2294_q_d_n2, eq182_e2294_q_d_n3, eq182_e2294_q_d_n4, eq182_e2294_q_d_n5, eq182_e2294_q_d_n6, eq182_e2294_q_d_n7, eq182_e2294_q_d_n8, eq182_e2294_q_d_n9, eq182_e2294_q_d_n10, eq182_e2294_q_d_n11, eq182_e2294_q_d_n12, eq182_e2294_q_d_n13, eq182_e2294_q_d_n14, eq182_e2294_q_d_n15, eq182_e2294_q_d_n16, eq182_e2294_q_d_n17, eq182_e2294_q_d_n18, eq182_e2294_q_d_n19, eq182_e2294_q_d_n20, eq182_e2294_q_d_n21, eq182_e2294_q_d_n22,) = {
    if ((s.b[595] && s.b[596]) && s.b[597]) {
        let eq182_e2289_q: f64 = s.v[288];
        let eq182_e2290: f64 = (p.p7 * s.v[288]);
        let eq182_e2290_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq182_e2290_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq182_e2290_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq182_e2290_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq182_e2290_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq182_e2290_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq182_e2290_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq182_e2290_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq182_e2290_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq182_e2290_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq182_e2290_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq182_e2290_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq182_e2290_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq182_e2290_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq182_e2290_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq182_e2290_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq182_e2290_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq182_e2290_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq182_e2290_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq182_e2290_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq182_e2290_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq182_e2290_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq182_e2290_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq182_e2290_q: f64 = (p.p7 * eq182_e2289_q);
        let eq182_e2290_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq182_e2290_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq182_e2290_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq182_e2290_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq182_e2290_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq182_e2290_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq182_e2290_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq182_e2290_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq182_e2290_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq182_e2290_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq182_e2290_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq182_e2290_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq182_e2290_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq182_e2290_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq182_e2290_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq182_e2290_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq182_e2290_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq182_e2290_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq182_e2290_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq182_e2290_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq182_e2290_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq182_e2290_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq182_e2290_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq182_e2292: f64 = (eq182_e2290 * p.p248);
        let eq182_e2292_d_n0: f64 = (eq182_e2290_d_n0 * p.p248);
        let eq182_e2292_d_n1: f64 = (eq182_e2290_d_n1 * p.p248);
        let eq182_e2292_d_n2: f64 = (eq182_e2290_d_n2 * p.p248);
        let eq182_e2292_d_n3: f64 = (eq182_e2290_d_n3 * p.p248);
        let eq182_e2292_d_n4: f64 = (eq182_e2290_d_n4 * p.p248);
        let eq182_e2292_d_n5: f64 = (eq182_e2290_d_n5 * p.p248);
        let eq182_e2292_d_n6: f64 = (eq182_e2290_d_n6 * p.p248);
        let eq182_e2292_d_n7: f64 = (eq182_e2290_d_n7 * p.p248);
        let eq182_e2292_d_n8: f64 = (eq182_e2290_d_n8 * p.p248);
        let eq182_e2292_d_n9: f64 = (eq182_e2290_d_n9 * p.p248);
        let eq182_e2292_d_n10: f64 = (eq182_e2290_d_n10 * p.p248);
        let eq182_e2292_d_n11: f64 = (eq182_e2290_d_n11 * p.p248);
        let eq182_e2292_d_n12: f64 = (eq182_e2290_d_n12 * p.p248);
        let eq182_e2292_d_n13: f64 = (eq182_e2290_d_n13 * p.p248);
        let eq182_e2292_d_n14: f64 = (eq182_e2290_d_n14 * p.p248);
        let eq182_e2292_d_n15: f64 = (eq182_e2290_d_n15 * p.p248);
        let eq182_e2292_d_n16: f64 = (eq182_e2290_d_n16 * p.p248);
        let eq182_e2292_d_n17: f64 = (eq182_e2290_d_n17 * p.p248);
        let eq182_e2292_d_n18: f64 = (eq182_e2290_d_n18 * p.p248);
        let eq182_e2292_d_n19: f64 = (eq182_e2290_d_n19 * p.p248);
        let eq182_e2292_d_n20: f64 = (eq182_e2290_d_n20 * p.p248);
        let eq182_e2292_d_n21: f64 = (eq182_e2290_d_n21 * p.p248);
        let eq182_e2292_d_n22: f64 = (eq182_e2290_d_n22 * p.p248);
        let eq182_e2292_q: f64 = (eq182_e2290_q * p.p248);
        let eq182_e2292_q_d_n0: f64 = (eq182_e2290_q_d_n0 * p.p248);
        let eq182_e2292_q_d_n1: f64 = (eq182_e2290_q_d_n1 * p.p248);
        let eq182_e2292_q_d_n2: f64 = (eq182_e2290_q_d_n2 * p.p248);
        let eq182_e2292_q_d_n3: f64 = (eq182_e2290_q_d_n3 * p.p248);
        let eq182_e2292_q_d_n4: f64 = (eq182_e2290_q_d_n4 * p.p248);
        let eq182_e2292_q_d_n5: f64 = (eq182_e2290_q_d_n5 * p.p248);
        let eq182_e2292_q_d_n6: f64 = (eq182_e2290_q_d_n6 * p.p248);
        let eq182_e2292_q_d_n7: f64 = (eq182_e2290_q_d_n7 * p.p248);
        let eq182_e2292_q_d_n8: f64 = (eq182_e2290_q_d_n8 * p.p248);
        let eq182_e2292_q_d_n9: f64 = (eq182_e2290_q_d_n9 * p.p248);
        let eq182_e2292_q_d_n10: f64 = (eq182_e2290_q_d_n10 * p.p248);
        let eq182_e2292_q_d_n11: f64 = (eq182_e2290_q_d_n11 * p.p248);
        let eq182_e2292_q_d_n12: f64 = (eq182_e2290_q_d_n12 * p.p248);
        let eq182_e2292_q_d_n13: f64 = (eq182_e2290_q_d_n13 * p.p248);
        let eq182_e2292_q_d_n14: f64 = (eq182_e2290_q_d_n14 * p.p248);
        let eq182_e2292_q_d_n15: f64 = (eq182_e2290_q_d_n15 * p.p248);
        let eq182_e2292_q_d_n16: f64 = (eq182_e2290_q_d_n16 * p.p248);
        let eq182_e2292_q_d_n17: f64 = (eq182_e2290_q_d_n17 * p.p248);
        let eq182_e2292_q_d_n18: f64 = (eq182_e2290_q_d_n18 * p.p248);
        let eq182_e2292_q_d_n19: f64 = (eq182_e2290_q_d_n19 * p.p248);
        let eq182_e2292_q_d_n20: f64 = (eq182_e2290_q_d_n20 * p.p248);
        let eq182_e2292_q_d_n21: f64 = (eq182_e2290_q_d_n21 * p.p248);
        let eq182_e2292_q_d_n22: f64 = (eq182_e2290_q_d_n22 * p.p248);
        (eq182_e2292, eq182_e2292_d_n0, eq182_e2292_d_n1, eq182_e2292_d_n2, eq182_e2292_d_n3, eq182_e2292_d_n4, eq182_e2292_d_n5, eq182_e2292_d_n6, eq182_e2292_d_n7, eq182_e2292_d_n8, eq182_e2292_d_n9, eq182_e2292_d_n10, eq182_e2292_d_n11, eq182_e2292_d_n12, eq182_e2292_d_n13, eq182_e2292_d_n14, eq182_e2292_d_n15, eq182_e2292_d_n16, eq182_e2292_d_n17, eq182_e2292_d_n18, eq182_e2292_d_n19, eq182_e2292_d_n20, eq182_e2292_d_n21, eq182_e2292_d_n22, eq182_e2292_q, eq182_e2292_q_d_n0, eq182_e2292_q_d_n1, eq182_e2292_q_d_n2, eq182_e2292_q_d_n3, eq182_e2292_q_d_n4, eq182_e2292_q_d_n5, eq182_e2292_q_d_n6, eq182_e2292_q_d_n7, eq182_e2292_q_d_n8, eq182_e2292_q_d_n9, eq182_e2292_q_d_n10, eq182_e2292_q_d_n11, eq182_e2292_q_d_n12, eq182_e2292_q_d_n13, eq182_e2292_q_d_n14, eq182_e2292_q_d_n15, eq182_e2292_q_d_n16, eq182_e2292_q_d_n17, eq182_e2292_q_d_n18, eq182_e2292_q_d_n19, eq182_e2292_q_d_n20, eq182_e2292_q_d_n21, eq182_e2292_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq182_reactive_node_derivatives: [f64; 23] = [eq182_e2294_q_d_n0, eq182_e2294_q_d_n1, eq182_e2294_q_d_n2, eq182_e2294_q_d_n3, eq182_e2294_q_d_n4, eq182_e2294_q_d_n5, eq182_e2294_q_d_n6, eq182_e2294_q_d_n7, eq182_e2294_q_d_n8, eq182_e2294_q_d_n9, eq182_e2294_q_d_n10, eq182_e2294_q_d_n11, eq182_e2294_q_d_n12, eq182_e2294_q_d_n13, eq182_e2294_q_d_n14, eq182_e2294_q_d_n15, eq182_e2294_q_d_n16, eq182_e2294_q_d_n17, eq182_e2294_q_d_n18, eq182_e2294_q_d_n19, eq182_e2294_q_d_n20, eq182_e2294_q_d_n21, eq182_e2294_q_d_n22];
        let eq182_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            nodes,
            &eq182_reactive_node_derivatives,
            branches,
            &eq182_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_14(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq183_e2306, eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22, eq183_e2306_q, eq183_e2306_q_d_n0, eq183_e2306_q_d_n1, eq183_e2306_q_d_n2, eq183_e2306_q_d_n3, eq183_e2306_q_d_n4, eq183_e2306_q_d_n5, eq183_e2306_q_d_n6, eq183_e2306_q_d_n7, eq183_e2306_q_d_n8, eq183_e2306_q_d_n9, eq183_e2306_q_d_n10, eq183_e2306_q_d_n11, eq183_e2306_q_d_n12, eq183_e2306_q_d_n13, eq183_e2306_q_d_n14, eq183_e2306_q_d_n15, eq183_e2306_q_d_n16, eq183_e2306_q_d_n17, eq183_e2306_q_d_n18, eq183_e2306_q_d_n19, eq183_e2306_q_d_n20, eq183_e2306_q_d_n21, eq183_e2306_q_d_n22,) = {
    if ((s.b[595] && s.b[596]) && (!s.b[597])) {
        let eq183_e2303_q: f64 = s.v[288];
        let eq183_e2304: f64 = (p.p7 * s.v[288]);
        let eq183_e2304_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq183_e2304_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq183_e2304_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq183_e2304_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq183_e2304_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq183_e2304_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq183_e2304_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq183_e2304_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq183_e2304_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq183_e2304_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq183_e2304_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq183_e2304_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq183_e2304_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq183_e2304_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq183_e2304_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq183_e2304_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq183_e2304_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq183_e2304_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq183_e2304_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq183_e2304_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq183_e2304_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq183_e2304_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq183_e2304_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq183_e2304_q: f64 = (p.p7 * eq183_e2303_q);
        let eq183_e2304_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq183_e2304_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq183_e2304_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq183_e2304_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq183_e2304_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq183_e2304_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq183_e2304_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq183_e2304_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq183_e2304_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq183_e2304_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq183_e2304_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq183_e2304_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq183_e2304_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq183_e2304_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq183_e2304_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq183_e2304_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq183_e2304_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq183_e2304_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq183_e2304_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq183_e2304_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq183_e2304_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq183_e2304_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq183_e2304_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        (eq183_e2304, eq183_e2304_d_n0, eq183_e2304_d_n1, eq183_e2304_d_n2, eq183_e2304_d_n3, eq183_e2304_d_n4, eq183_e2304_d_n5, eq183_e2304_d_n6, eq183_e2304_d_n7, eq183_e2304_d_n8, eq183_e2304_d_n9, eq183_e2304_d_n10, eq183_e2304_d_n11, eq183_e2304_d_n12, eq183_e2304_d_n13, eq183_e2304_d_n14, eq183_e2304_d_n15, eq183_e2304_d_n16, eq183_e2304_d_n17, eq183_e2304_d_n18, eq183_e2304_d_n19, eq183_e2304_d_n20, eq183_e2304_d_n21, eq183_e2304_d_n22, eq183_e2304_q, eq183_e2304_q_d_n0, eq183_e2304_q_d_n1, eq183_e2304_q_d_n2, eq183_e2304_q_d_n3, eq183_e2304_q_d_n4, eq183_e2304_q_d_n5, eq183_e2304_q_d_n6, eq183_e2304_q_d_n7, eq183_e2304_q_d_n8, eq183_e2304_q_d_n9, eq183_e2304_q_d_n10, eq183_e2304_q_d_n11, eq183_e2304_q_d_n12, eq183_e2304_q_d_n13, eq183_e2304_q_d_n14, eq183_e2304_q_d_n15, eq183_e2304_q_d_n16, eq183_e2304_q_d_n17, eq183_e2304_q_d_n18, eq183_e2304_q_d_n19, eq183_e2304_q_d_n20, eq183_e2304_q_d_n21, eq183_e2304_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq183_reactive_node_derivatives: [f64; 23] = [eq183_e2306_q_d_n0, eq183_e2306_q_d_n1, eq183_e2306_q_d_n2, eq183_e2306_q_d_n3, eq183_e2306_q_d_n4, eq183_e2306_q_d_n5, eq183_e2306_q_d_n6, eq183_e2306_q_d_n7, eq183_e2306_q_d_n8, eq183_e2306_q_d_n9, eq183_e2306_q_d_n10, eq183_e2306_q_d_n11, eq183_e2306_q_d_n12, eq183_e2306_q_d_n13, eq183_e2306_q_d_n14, eq183_e2306_q_d_n15, eq183_e2306_q_d_n16, eq183_e2306_q_d_n17, eq183_e2306_q_d_n18, eq183_e2306_q_d_n19, eq183_e2306_q_d_n20, eq183_e2306_q_d_n21, eq183_e2306_q_d_n22];
        let eq183_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            nodes,
            &eq183_reactive_node_derivatives,
            branches,
            &eq183_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq184_e2320, eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22, eq184_e2320_q, eq184_e2320_q_d_n0, eq184_e2320_q_d_n1, eq184_e2320_q_d_n2, eq184_e2320_q_d_n3, eq184_e2320_q_d_n4, eq184_e2320_q_d_n5, eq184_e2320_q_d_n6, eq184_e2320_q_d_n7, eq184_e2320_q_d_n8, eq184_e2320_q_d_n9, eq184_e2320_q_d_n10, eq184_e2320_q_d_n11, eq184_e2320_q_d_n12, eq184_e2320_q_d_n13, eq184_e2320_q_d_n14, eq184_e2320_q_d_n15, eq184_e2320_q_d_n16, eq184_e2320_q_d_n17, eq184_e2320_q_d_n18, eq184_e2320_q_d_n19, eq184_e2320_q_d_n20, eq184_e2320_q_d_n21, eq184_e2320_q_d_n22,) = {
    if ((s.b[595] && s.b[596]) && (!s.b[597])) {
        let eq184_e2315_q: f64 = s.v[288];
        let eq184_e2316: f64 = (p.p7 * s.v[288]);
        let eq184_e2316_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq184_e2316_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq184_e2316_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq184_e2316_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq184_e2316_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq184_e2316_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq184_e2316_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq184_e2316_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq184_e2316_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq184_e2316_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq184_e2316_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq184_e2316_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq184_e2316_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq184_e2316_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq184_e2316_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq184_e2316_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq184_e2316_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq184_e2316_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq184_e2316_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq184_e2316_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq184_e2316_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq184_e2316_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq184_e2316_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq184_e2316_q: f64 = (p.p7 * eq184_e2315_q);
        let eq184_e2316_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq184_e2316_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq184_e2316_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq184_e2316_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq184_e2316_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq184_e2316_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq184_e2316_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq184_e2316_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq184_e2316_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq184_e2316_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq184_e2316_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq184_e2316_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq184_e2316_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq184_e2316_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq184_e2316_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq184_e2316_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq184_e2316_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq184_e2316_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq184_e2316_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq184_e2316_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq184_e2316_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq184_e2316_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq184_e2316_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq184_e2318: f64 = (eq184_e2316 * p.p248);
        let eq184_e2318_d_n0: f64 = (eq184_e2316_d_n0 * p.p248);
        let eq184_e2318_d_n1: f64 = (eq184_e2316_d_n1 * p.p248);
        let eq184_e2318_d_n2: f64 = (eq184_e2316_d_n2 * p.p248);
        let eq184_e2318_d_n3: f64 = (eq184_e2316_d_n3 * p.p248);
        let eq184_e2318_d_n4: f64 = (eq184_e2316_d_n4 * p.p248);
        let eq184_e2318_d_n5: f64 = (eq184_e2316_d_n5 * p.p248);
        let eq184_e2318_d_n6: f64 = (eq184_e2316_d_n6 * p.p248);
        let eq184_e2318_d_n7: f64 = (eq184_e2316_d_n7 * p.p248);
        let eq184_e2318_d_n8: f64 = (eq184_e2316_d_n8 * p.p248);
        let eq184_e2318_d_n9: f64 = (eq184_e2316_d_n9 * p.p248);
        let eq184_e2318_d_n10: f64 = (eq184_e2316_d_n10 * p.p248);
        let eq184_e2318_d_n11: f64 = (eq184_e2316_d_n11 * p.p248);
        let eq184_e2318_d_n12: f64 = (eq184_e2316_d_n12 * p.p248);
        let eq184_e2318_d_n13: f64 = (eq184_e2316_d_n13 * p.p248);
        let eq184_e2318_d_n14: f64 = (eq184_e2316_d_n14 * p.p248);
        let eq184_e2318_d_n15: f64 = (eq184_e2316_d_n15 * p.p248);
        let eq184_e2318_d_n16: f64 = (eq184_e2316_d_n16 * p.p248);
        let eq184_e2318_d_n17: f64 = (eq184_e2316_d_n17 * p.p248);
        let eq184_e2318_d_n18: f64 = (eq184_e2316_d_n18 * p.p248);
        let eq184_e2318_d_n19: f64 = (eq184_e2316_d_n19 * p.p248);
        let eq184_e2318_d_n20: f64 = (eq184_e2316_d_n20 * p.p248);
        let eq184_e2318_d_n21: f64 = (eq184_e2316_d_n21 * p.p248);
        let eq184_e2318_d_n22: f64 = (eq184_e2316_d_n22 * p.p248);
        let eq184_e2318_q: f64 = (eq184_e2316_q * p.p248);
        let eq184_e2318_q_d_n0: f64 = (eq184_e2316_q_d_n0 * p.p248);
        let eq184_e2318_q_d_n1: f64 = (eq184_e2316_q_d_n1 * p.p248);
        let eq184_e2318_q_d_n2: f64 = (eq184_e2316_q_d_n2 * p.p248);
        let eq184_e2318_q_d_n3: f64 = (eq184_e2316_q_d_n3 * p.p248);
        let eq184_e2318_q_d_n4: f64 = (eq184_e2316_q_d_n4 * p.p248);
        let eq184_e2318_q_d_n5: f64 = (eq184_e2316_q_d_n5 * p.p248);
        let eq184_e2318_q_d_n6: f64 = (eq184_e2316_q_d_n6 * p.p248);
        let eq184_e2318_q_d_n7: f64 = (eq184_e2316_q_d_n7 * p.p248);
        let eq184_e2318_q_d_n8: f64 = (eq184_e2316_q_d_n8 * p.p248);
        let eq184_e2318_q_d_n9: f64 = (eq184_e2316_q_d_n9 * p.p248);
        let eq184_e2318_q_d_n10: f64 = (eq184_e2316_q_d_n10 * p.p248);
        let eq184_e2318_q_d_n11: f64 = (eq184_e2316_q_d_n11 * p.p248);
        let eq184_e2318_q_d_n12: f64 = (eq184_e2316_q_d_n12 * p.p248);
        let eq184_e2318_q_d_n13: f64 = (eq184_e2316_q_d_n13 * p.p248);
        let eq184_e2318_q_d_n14: f64 = (eq184_e2316_q_d_n14 * p.p248);
        let eq184_e2318_q_d_n15: f64 = (eq184_e2316_q_d_n15 * p.p248);
        let eq184_e2318_q_d_n16: f64 = (eq184_e2316_q_d_n16 * p.p248);
        let eq184_e2318_q_d_n17: f64 = (eq184_e2316_q_d_n17 * p.p248);
        let eq184_e2318_q_d_n18: f64 = (eq184_e2316_q_d_n18 * p.p248);
        let eq184_e2318_q_d_n19: f64 = (eq184_e2316_q_d_n19 * p.p248);
        let eq184_e2318_q_d_n20: f64 = (eq184_e2316_q_d_n20 * p.p248);
        let eq184_e2318_q_d_n21: f64 = (eq184_e2316_q_d_n21 * p.p248);
        let eq184_e2318_q_d_n22: f64 = (eq184_e2316_q_d_n22 * p.p248);
        (eq184_e2318, eq184_e2318_d_n0, eq184_e2318_d_n1, eq184_e2318_d_n2, eq184_e2318_d_n3, eq184_e2318_d_n4, eq184_e2318_d_n5, eq184_e2318_d_n6, eq184_e2318_d_n7, eq184_e2318_d_n8, eq184_e2318_d_n9, eq184_e2318_d_n10, eq184_e2318_d_n11, eq184_e2318_d_n12, eq184_e2318_d_n13, eq184_e2318_d_n14, eq184_e2318_d_n15, eq184_e2318_d_n16, eq184_e2318_d_n17, eq184_e2318_d_n18, eq184_e2318_d_n19, eq184_e2318_d_n20, eq184_e2318_d_n21, eq184_e2318_d_n22, eq184_e2318_q, eq184_e2318_q_d_n0, eq184_e2318_q_d_n1, eq184_e2318_q_d_n2, eq184_e2318_q_d_n3, eq184_e2318_q_d_n4, eq184_e2318_q_d_n5, eq184_e2318_q_d_n6, eq184_e2318_q_d_n7, eq184_e2318_q_d_n8, eq184_e2318_q_d_n9, eq184_e2318_q_d_n10, eq184_e2318_q_d_n11, eq184_e2318_q_d_n12, eq184_e2318_q_d_n13, eq184_e2318_q_d_n14, eq184_e2318_q_d_n15, eq184_e2318_q_d_n16, eq184_e2318_q_d_n17, eq184_e2318_q_d_n18, eq184_e2318_q_d_n19, eq184_e2318_q_d_n20, eq184_e2318_q_d_n21, eq184_e2318_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq184_reactive_node_derivatives: [f64; 23] = [eq184_e2320_q_d_n0, eq184_e2320_q_d_n1, eq184_e2320_q_d_n2, eq184_e2320_q_d_n3, eq184_e2320_q_d_n4, eq184_e2320_q_d_n5, eq184_e2320_q_d_n6, eq184_e2320_q_d_n7, eq184_e2320_q_d_n8, eq184_e2320_q_d_n9, eq184_e2320_q_d_n10, eq184_e2320_q_d_n11, eq184_e2320_q_d_n12, eq184_e2320_q_d_n13, eq184_e2320_q_d_n14, eq184_e2320_q_d_n15, eq184_e2320_q_d_n16, eq184_e2320_q_d_n17, eq184_e2320_q_d_n18, eq184_e2320_q_d_n19, eq184_e2320_q_d_n20, eq184_e2320_q_d_n21, eq184_e2320_q_d_n22];
        let eq184_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            nodes,
            &eq184_reactive_node_derivatives,
            branches,
            &eq184_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq185_e2331, eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22, eq185_e2331_q, eq185_e2331_q_d_n0, eq185_e2331_q_d_n1, eq185_e2331_q_d_n2, eq185_e2331_q_d_n3, eq185_e2331_q_d_n4, eq185_e2331_q_d_n5, eq185_e2331_q_d_n6, eq185_e2331_q_d_n7, eq185_e2331_q_d_n8, eq185_e2331_q_d_n9, eq185_e2331_q_d_n10, eq185_e2331_q_d_n11, eq185_e2331_q_d_n12, eq185_e2331_q_d_n13, eq185_e2331_q_d_n14, eq185_e2331_q_d_n15, eq185_e2331_q_d_n16, eq185_e2331_q_d_n17, eq185_e2331_q_d_n18, eq185_e2331_q_d_n19, eq185_e2331_q_d_n20, eq185_e2331_q_d_n21, eq185_e2331_q_d_n22,) = {
    if (s.b[595] && s.b[596]) {
        let eq185_e2327: f64 = (p.p253 * s.v[288]);
        let eq185_e2327_d_n0: f64 = (p.p253 * s.dn[288][0]);
        let eq185_e2327_d_n1: f64 = (p.p253 * s.dn[288][1]);
        let eq185_e2327_d_n2: f64 = (p.p253 * s.dn[288][2]);
        let eq185_e2327_d_n3: f64 = (p.p253 * s.dn[288][3]);
        let eq185_e2327_d_n4: f64 = (p.p253 * s.dn[288][4]);
        let eq185_e2327_d_n5: f64 = (p.p253 * s.dn[288][5]);
        let eq185_e2327_d_n6: f64 = (p.p253 * s.dn[288][6]);
        let eq185_e2327_d_n7: f64 = (p.p253 * s.dn[288][7]);
        let eq185_e2327_d_n8: f64 = (p.p253 * s.dn[288][8]);
        let eq185_e2327_d_n9: f64 = (p.p253 * s.dn[288][9]);
        let eq185_e2327_d_n10: f64 = (p.p253 * s.dn[288][10]);
        let eq185_e2327_d_n11: f64 = (p.p253 * s.dn[288][11]);
        let eq185_e2327_d_n12: f64 = (p.p253 * s.dn[288][12]);
        let eq185_e2327_d_n13: f64 = (p.p253 * s.dn[288][13]);
        let eq185_e2327_d_n14: f64 = (p.p253 * s.dn[288][14]);
        let eq185_e2327_d_n15: f64 = (p.p253 * s.dn[288][15]);
        let eq185_e2327_d_n16: f64 = (p.p253 * s.dn[288][16]);
        let eq185_e2327_d_n17: f64 = (p.p253 * s.dn[288][17]);
        let eq185_e2327_d_n18: f64 = (p.p253 * s.dn[288][18]);
        let eq185_e2327_d_n19: f64 = (p.p253 * s.dn[288][19]);
        let eq185_e2327_d_n20: f64 = (p.p253 * s.dn[288][20]);
        let eq185_e2327_d_n21: f64 = (p.p253 * s.dn[288][21]);
        let eq185_e2327_d_n22: f64 = (p.p253 * s.dn[288][22]);
        let eq185_e2328_q: f64 = eq185_e2327;
        let eq185_e2329: f64 = (p.p7 * eq185_e2327);
        let eq185_e2329_d_n0: f64 = (p.p7 * eq185_e2327_d_n0);
        let eq185_e2329_d_n1: f64 = (p.p7 * eq185_e2327_d_n1);
        let eq185_e2329_d_n2: f64 = (p.p7 * eq185_e2327_d_n2);
        let eq185_e2329_d_n3: f64 = (p.p7 * eq185_e2327_d_n3);
        let eq185_e2329_d_n4: f64 = (p.p7 * eq185_e2327_d_n4);
        let eq185_e2329_d_n5: f64 = (p.p7 * eq185_e2327_d_n5);
        let eq185_e2329_d_n6: f64 = (p.p7 * eq185_e2327_d_n6);
        let eq185_e2329_d_n7: f64 = (p.p7 * eq185_e2327_d_n7);
        let eq185_e2329_d_n8: f64 = (p.p7 * eq185_e2327_d_n8);
        let eq185_e2329_d_n9: f64 = (p.p7 * eq185_e2327_d_n9);
        let eq185_e2329_d_n10: f64 = (p.p7 * eq185_e2327_d_n10);
        let eq185_e2329_d_n11: f64 = (p.p7 * eq185_e2327_d_n11);
        let eq185_e2329_d_n12: f64 = (p.p7 * eq185_e2327_d_n12);
        let eq185_e2329_d_n13: f64 = (p.p7 * eq185_e2327_d_n13);
        let eq185_e2329_d_n14: f64 = (p.p7 * eq185_e2327_d_n14);
        let eq185_e2329_d_n15: f64 = (p.p7 * eq185_e2327_d_n15);
        let eq185_e2329_d_n16: f64 = (p.p7 * eq185_e2327_d_n16);
        let eq185_e2329_d_n17: f64 = (p.p7 * eq185_e2327_d_n17);
        let eq185_e2329_d_n18: f64 = (p.p7 * eq185_e2327_d_n18);
        let eq185_e2329_d_n19: f64 = (p.p7 * eq185_e2327_d_n19);
        let eq185_e2329_d_n20: f64 = (p.p7 * eq185_e2327_d_n20);
        let eq185_e2329_d_n21: f64 = (p.p7 * eq185_e2327_d_n21);
        let eq185_e2329_d_n22: f64 = (p.p7 * eq185_e2327_d_n22);
        let eq185_e2329_q: f64 = (p.p7 * eq185_e2328_q);
        let eq185_e2329_q_d_n0: f64 = (p.p7 * eq185_e2327_d_n0);
        let eq185_e2329_q_d_n1: f64 = (p.p7 * eq185_e2327_d_n1);
        let eq185_e2329_q_d_n2: f64 = (p.p7 * eq185_e2327_d_n2);
        let eq185_e2329_q_d_n3: f64 = (p.p7 * eq185_e2327_d_n3);
        let eq185_e2329_q_d_n4: f64 = (p.p7 * eq185_e2327_d_n4);
        let eq185_e2329_q_d_n5: f64 = (p.p7 * eq185_e2327_d_n5);
        let eq185_e2329_q_d_n6: f64 = (p.p7 * eq185_e2327_d_n6);
        let eq185_e2329_q_d_n7: f64 = (p.p7 * eq185_e2327_d_n7);
        let eq185_e2329_q_d_n8: f64 = (p.p7 * eq185_e2327_d_n8);
        let eq185_e2329_q_d_n9: f64 = (p.p7 * eq185_e2327_d_n9);
        let eq185_e2329_q_d_n10: f64 = (p.p7 * eq185_e2327_d_n10);
        let eq185_e2329_q_d_n11: f64 = (p.p7 * eq185_e2327_d_n11);
        let eq185_e2329_q_d_n12: f64 = (p.p7 * eq185_e2327_d_n12);
        let eq185_e2329_q_d_n13: f64 = (p.p7 * eq185_e2327_d_n13);
        let eq185_e2329_q_d_n14: f64 = (p.p7 * eq185_e2327_d_n14);
        let eq185_e2329_q_d_n15: f64 = (p.p7 * eq185_e2327_d_n15);
        let eq185_e2329_q_d_n16: f64 = (p.p7 * eq185_e2327_d_n16);
        let eq185_e2329_q_d_n17: f64 = (p.p7 * eq185_e2327_d_n17);
        let eq185_e2329_q_d_n18: f64 = (p.p7 * eq185_e2327_d_n18);
        let eq185_e2329_q_d_n19: f64 = (p.p7 * eq185_e2327_d_n19);
        let eq185_e2329_q_d_n20: f64 = (p.p7 * eq185_e2327_d_n20);
        let eq185_e2329_q_d_n21: f64 = (p.p7 * eq185_e2327_d_n21);
        let eq185_e2329_q_d_n22: f64 = (p.p7 * eq185_e2327_d_n22);
        (eq185_e2329, eq185_e2329_d_n0, eq185_e2329_d_n1, eq185_e2329_d_n2, eq185_e2329_d_n3, eq185_e2329_d_n4, eq185_e2329_d_n5, eq185_e2329_d_n6, eq185_e2329_d_n7, eq185_e2329_d_n8, eq185_e2329_d_n9, eq185_e2329_d_n10, eq185_e2329_d_n11, eq185_e2329_d_n12, eq185_e2329_d_n13, eq185_e2329_d_n14, eq185_e2329_d_n15, eq185_e2329_d_n16, eq185_e2329_d_n17, eq185_e2329_d_n18, eq185_e2329_d_n19, eq185_e2329_d_n20, eq185_e2329_d_n21, eq185_e2329_d_n22, eq185_e2329_q, eq185_e2329_q_d_n0, eq185_e2329_q_d_n1, eq185_e2329_q_d_n2, eq185_e2329_q_d_n3, eq185_e2329_q_d_n4, eq185_e2329_q_d_n5, eq185_e2329_q_d_n6, eq185_e2329_q_d_n7, eq185_e2329_q_d_n8, eq185_e2329_q_d_n9, eq185_e2329_q_d_n10, eq185_e2329_q_d_n11, eq185_e2329_q_d_n12, eq185_e2329_q_d_n13, eq185_e2329_q_d_n14, eq185_e2329_q_d_n15, eq185_e2329_q_d_n16, eq185_e2329_q_d_n17, eq185_e2329_q_d_n18, eq185_e2329_q_d_n19, eq185_e2329_q_d_n20, eq185_e2329_q_d_n21, eq185_e2329_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq185_reactive_node_derivatives: [f64; 23] = [eq185_e2331_q_d_n0, eq185_e2331_q_d_n1, eq185_e2331_q_d_n2, eq185_e2331_q_d_n3, eq185_e2331_q_d_n4, eq185_e2331_q_d_n5, eq185_e2331_q_d_n6, eq185_e2331_q_d_n7, eq185_e2331_q_d_n8, eq185_e2331_q_d_n9, eq185_e2331_q_d_n10, eq185_e2331_q_d_n11, eq185_e2331_q_d_n12, eq185_e2331_q_d_n13, eq185_e2331_q_d_n14, eq185_e2331_q_d_n15, eq185_e2331_q_d_n16, eq185_e2331_q_d_n17, eq185_e2331_q_d_n18, eq185_e2331_q_d_n19, eq185_e2331_q_d_n20, eq185_e2331_q_d_n21, eq185_e2331_q_d_n22];
        let eq185_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[21]),
            nodes,
            &eq185_reactive_node_derivatives,
            branches,
            &eq185_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq186_e2341, eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22, eq186_e2341_q, eq186_e2341_q_d_n0, eq186_e2341_q_d_n1, eq186_e2341_q_d_n2, eq186_e2341_q_d_n3, eq186_e2341_q_d_n4, eq186_e2341_q_d_n5, eq186_e2341_q_d_n6, eq186_e2341_q_d_n7, eq186_e2341_q_d_n8, eq186_e2341_q_d_n9, eq186_e2341_q_d_n10, eq186_e2341_q_d_n11, eq186_e2341_q_d_n12, eq186_e2341_q_d_n13, eq186_e2341_q_d_n14, eq186_e2341_q_d_n15, eq186_e2341_q_d_n16, eq186_e2341_q_d_n17, eq186_e2341_q_d_n18, eq186_e2341_q_d_n19, eq186_e2341_q_d_n20, eq186_e2341_q_d_n21, eq186_e2341_q_d_n22,) = {
    if ((!s.b[595]) && s.b[598]) {
        let eq186_e2338_q: f64 = s.v[289];
        let eq186_e2339: f64 = (p.p7 * s.v[289]);
        let eq186_e2339_d_n0: f64 = (p.p7 * s.dn[289][0]);
        let eq186_e2339_d_n1: f64 = (p.p7 * s.dn[289][1]);
        let eq186_e2339_d_n2: f64 = (p.p7 * s.dn[289][2]);
        let eq186_e2339_d_n3: f64 = (p.p7 * s.dn[289][3]);
        let eq186_e2339_d_n4: f64 = (p.p7 * s.dn[289][4]);
        let eq186_e2339_d_n5: f64 = (p.p7 * s.dn[289][5]);
        let eq186_e2339_d_n6: f64 = (p.p7 * s.dn[289][6]);
        let eq186_e2339_d_n7: f64 = (p.p7 * s.dn[289][7]);
        let eq186_e2339_d_n8: f64 = (p.p7 * s.dn[289][8]);
        let eq186_e2339_d_n9: f64 = (p.p7 * s.dn[289][9]);
        let eq186_e2339_d_n10: f64 = (p.p7 * s.dn[289][10]);
        let eq186_e2339_d_n11: f64 = (p.p7 * s.dn[289][11]);
        let eq186_e2339_d_n12: f64 = (p.p7 * s.dn[289][12]);
        let eq186_e2339_d_n13: f64 = (p.p7 * s.dn[289][13]);
        let eq186_e2339_d_n14: f64 = (p.p7 * s.dn[289][14]);
        let eq186_e2339_d_n15: f64 = (p.p7 * s.dn[289][15]);
        let eq186_e2339_d_n16: f64 = (p.p7 * s.dn[289][16]);
        let eq186_e2339_d_n17: f64 = (p.p7 * s.dn[289][17]);
        let eq186_e2339_d_n18: f64 = (p.p7 * s.dn[289][18]);
        let eq186_e2339_d_n19: f64 = (p.p7 * s.dn[289][19]);
        let eq186_e2339_d_n20: f64 = (p.p7 * s.dn[289][20]);
        let eq186_e2339_d_n21: f64 = (p.p7 * s.dn[289][21]);
        let eq186_e2339_d_n22: f64 = (p.p7 * s.dn[289][22]);
        let eq186_e2339_q: f64 = (p.p7 * eq186_e2338_q);
        let eq186_e2339_q_d_n0: f64 = (p.p7 * s.dn[289][0]);
        let eq186_e2339_q_d_n1: f64 = (p.p7 * s.dn[289][1]);
        let eq186_e2339_q_d_n2: f64 = (p.p7 * s.dn[289][2]);
        let eq186_e2339_q_d_n3: f64 = (p.p7 * s.dn[289][3]);
        let eq186_e2339_q_d_n4: f64 = (p.p7 * s.dn[289][4]);
        let eq186_e2339_q_d_n5: f64 = (p.p7 * s.dn[289][5]);
        let eq186_e2339_q_d_n6: f64 = (p.p7 * s.dn[289][6]);
        let eq186_e2339_q_d_n7: f64 = (p.p7 * s.dn[289][7]);
        let eq186_e2339_q_d_n8: f64 = (p.p7 * s.dn[289][8]);
        let eq186_e2339_q_d_n9: f64 = (p.p7 * s.dn[289][9]);
        let eq186_e2339_q_d_n10: f64 = (p.p7 * s.dn[289][10]);
        let eq186_e2339_q_d_n11: f64 = (p.p7 * s.dn[289][11]);
        let eq186_e2339_q_d_n12: f64 = (p.p7 * s.dn[289][12]);
        let eq186_e2339_q_d_n13: f64 = (p.p7 * s.dn[289][13]);
        let eq186_e2339_q_d_n14: f64 = (p.p7 * s.dn[289][14]);
        let eq186_e2339_q_d_n15: f64 = (p.p7 * s.dn[289][15]);
        let eq186_e2339_q_d_n16: f64 = (p.p7 * s.dn[289][16]);
        let eq186_e2339_q_d_n17: f64 = (p.p7 * s.dn[289][17]);
        let eq186_e2339_q_d_n18: f64 = (p.p7 * s.dn[289][18]);
        let eq186_e2339_q_d_n19: f64 = (p.p7 * s.dn[289][19]);
        let eq186_e2339_q_d_n20: f64 = (p.p7 * s.dn[289][20]);
        let eq186_e2339_q_d_n21: f64 = (p.p7 * s.dn[289][21]);
        let eq186_e2339_q_d_n22: f64 = (p.p7 * s.dn[289][22]);
        (eq186_e2339, eq186_e2339_d_n0, eq186_e2339_d_n1, eq186_e2339_d_n2, eq186_e2339_d_n3, eq186_e2339_d_n4, eq186_e2339_d_n5, eq186_e2339_d_n6, eq186_e2339_d_n7, eq186_e2339_d_n8, eq186_e2339_d_n9, eq186_e2339_d_n10, eq186_e2339_d_n11, eq186_e2339_d_n12, eq186_e2339_d_n13, eq186_e2339_d_n14, eq186_e2339_d_n15, eq186_e2339_d_n16, eq186_e2339_d_n17, eq186_e2339_d_n18, eq186_e2339_d_n19, eq186_e2339_d_n20, eq186_e2339_d_n21, eq186_e2339_d_n22, eq186_e2339_q, eq186_e2339_q_d_n0, eq186_e2339_q_d_n1, eq186_e2339_q_d_n2, eq186_e2339_q_d_n3, eq186_e2339_q_d_n4, eq186_e2339_q_d_n5, eq186_e2339_q_d_n6, eq186_e2339_q_d_n7, eq186_e2339_q_d_n8, eq186_e2339_q_d_n9, eq186_e2339_q_d_n10, eq186_e2339_q_d_n11, eq186_e2339_q_d_n12, eq186_e2339_q_d_n13, eq186_e2339_q_d_n14, eq186_e2339_q_d_n15, eq186_e2339_q_d_n16, eq186_e2339_q_d_n17, eq186_e2339_q_d_n18, eq186_e2339_q_d_n19, eq186_e2339_q_d_n20, eq186_e2339_q_d_n21, eq186_e2339_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq186_reactive_node_derivatives: [f64; 23] = [eq186_e2341_q_d_n0, eq186_e2341_q_d_n1, eq186_e2341_q_d_n2, eq186_e2341_q_d_n3, eq186_e2341_q_d_n4, eq186_e2341_q_d_n5, eq186_e2341_q_d_n6, eq186_e2341_q_d_n7, eq186_e2341_q_d_n8, eq186_e2341_q_d_n9, eq186_e2341_q_d_n10, eq186_e2341_q_d_n11, eq186_e2341_q_d_n12, eq186_e2341_q_d_n13, eq186_e2341_q_d_n14, eq186_e2341_q_d_n15, eq186_e2341_q_d_n16, eq186_e2341_q_d_n17, eq186_e2341_q_d_n18, eq186_e2341_q_d_n19, eq186_e2341_q_d_n20, eq186_e2341_q_d_n21, eq186_e2341_q_d_n22];
        let eq186_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq186_reactive_node_derivatives,
            branches,
            &eq186_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq187_e2353, eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22, eq187_e2353_q, eq187_e2353_q_d_n0, eq187_e2353_q_d_n1, eq187_e2353_q_d_n2, eq187_e2353_q_d_n3, eq187_e2353_q_d_n4, eq187_e2353_q_d_n5, eq187_e2353_q_d_n6, eq187_e2353_q_d_n7, eq187_e2353_q_d_n8, eq187_e2353_q_d_n9, eq187_e2353_q_d_n10, eq187_e2353_q_d_n11, eq187_e2353_q_d_n12, eq187_e2353_q_d_n13, eq187_e2353_q_d_n14, eq187_e2353_q_d_n15, eq187_e2353_q_d_n16, eq187_e2353_q_d_n17, eq187_e2353_q_d_n18, eq187_e2353_q_d_n19, eq187_e2353_q_d_n20, eq187_e2353_q_d_n21, eq187_e2353_q_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && s.b[599]) {
        let eq187_e2350_q: f64 = s.v[288];
        let eq187_e2351: f64 = (p.p7 * s.v[288]);
        let eq187_e2351_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq187_e2351_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq187_e2351_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq187_e2351_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq187_e2351_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq187_e2351_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq187_e2351_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq187_e2351_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq187_e2351_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq187_e2351_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq187_e2351_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq187_e2351_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq187_e2351_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq187_e2351_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq187_e2351_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq187_e2351_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq187_e2351_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq187_e2351_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq187_e2351_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq187_e2351_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq187_e2351_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq187_e2351_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq187_e2351_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq187_e2351_q: f64 = (p.p7 * eq187_e2350_q);
        let eq187_e2351_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq187_e2351_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq187_e2351_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq187_e2351_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq187_e2351_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq187_e2351_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq187_e2351_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq187_e2351_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq187_e2351_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq187_e2351_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq187_e2351_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq187_e2351_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq187_e2351_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq187_e2351_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq187_e2351_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq187_e2351_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq187_e2351_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq187_e2351_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq187_e2351_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq187_e2351_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq187_e2351_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq187_e2351_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq187_e2351_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        (eq187_e2351, eq187_e2351_d_n0, eq187_e2351_d_n1, eq187_e2351_d_n2, eq187_e2351_d_n3, eq187_e2351_d_n4, eq187_e2351_d_n5, eq187_e2351_d_n6, eq187_e2351_d_n7, eq187_e2351_d_n8, eq187_e2351_d_n9, eq187_e2351_d_n10, eq187_e2351_d_n11, eq187_e2351_d_n12, eq187_e2351_d_n13, eq187_e2351_d_n14, eq187_e2351_d_n15, eq187_e2351_d_n16, eq187_e2351_d_n17, eq187_e2351_d_n18, eq187_e2351_d_n19, eq187_e2351_d_n20, eq187_e2351_d_n21, eq187_e2351_d_n22, eq187_e2351_q, eq187_e2351_q_d_n0, eq187_e2351_q_d_n1, eq187_e2351_q_d_n2, eq187_e2351_q_d_n3, eq187_e2351_q_d_n4, eq187_e2351_q_d_n5, eq187_e2351_q_d_n6, eq187_e2351_q_d_n7, eq187_e2351_q_d_n8, eq187_e2351_q_d_n9, eq187_e2351_q_d_n10, eq187_e2351_q_d_n11, eq187_e2351_q_d_n12, eq187_e2351_q_d_n13, eq187_e2351_q_d_n14, eq187_e2351_q_d_n15, eq187_e2351_q_d_n16, eq187_e2351_q_d_n17, eq187_e2351_q_d_n18, eq187_e2351_q_d_n19, eq187_e2351_q_d_n20, eq187_e2351_q_d_n21, eq187_e2351_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq187_reactive_node_derivatives: [f64; 23] = [eq187_e2353_q_d_n0, eq187_e2353_q_d_n1, eq187_e2353_q_d_n2, eq187_e2353_q_d_n3, eq187_e2353_q_d_n4, eq187_e2353_q_d_n5, eq187_e2353_q_d_n6, eq187_e2353_q_d_n7, eq187_e2353_q_d_n8, eq187_e2353_q_d_n9, eq187_e2353_q_d_n10, eq187_e2353_q_d_n11, eq187_e2353_q_d_n12, eq187_e2353_q_d_n13, eq187_e2353_q_d_n14, eq187_e2353_q_d_n15, eq187_e2353_q_d_n16, eq187_e2353_q_d_n17, eq187_e2353_q_d_n18, eq187_e2353_q_d_n19, eq187_e2353_q_d_n20, eq187_e2353_q_d_n21, eq187_e2353_q_d_n22];
        let eq187_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq187_reactive_node_derivatives,
            branches,
            &eq187_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_15(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq188_e2367, eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22, eq188_e2367_q, eq188_e2367_q_d_n0, eq188_e2367_q_d_n1, eq188_e2367_q_d_n2, eq188_e2367_q_d_n3, eq188_e2367_q_d_n4, eq188_e2367_q_d_n5, eq188_e2367_q_d_n6, eq188_e2367_q_d_n7, eq188_e2367_q_d_n8, eq188_e2367_q_d_n9, eq188_e2367_q_d_n10, eq188_e2367_q_d_n11, eq188_e2367_q_d_n12, eq188_e2367_q_d_n13, eq188_e2367_q_d_n14, eq188_e2367_q_d_n15, eq188_e2367_q_d_n16, eq188_e2367_q_d_n17, eq188_e2367_q_d_n18, eq188_e2367_q_d_n19, eq188_e2367_q_d_n20, eq188_e2367_q_d_n21, eq188_e2367_q_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && s.b[599]) {
        let eq188_e2362_q: f64 = s.v[288];
        let eq188_e2363: f64 = (p.p7 * s.v[288]);
        let eq188_e2363_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq188_e2363_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq188_e2363_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq188_e2363_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq188_e2363_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq188_e2363_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq188_e2363_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq188_e2363_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq188_e2363_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq188_e2363_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq188_e2363_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq188_e2363_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq188_e2363_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq188_e2363_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq188_e2363_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq188_e2363_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq188_e2363_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq188_e2363_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq188_e2363_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq188_e2363_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq188_e2363_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq188_e2363_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq188_e2363_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq188_e2363_q: f64 = (p.p7 * eq188_e2362_q);
        let eq188_e2363_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq188_e2363_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq188_e2363_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq188_e2363_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq188_e2363_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq188_e2363_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq188_e2363_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq188_e2363_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq188_e2363_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq188_e2363_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq188_e2363_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq188_e2363_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq188_e2363_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq188_e2363_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq188_e2363_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq188_e2363_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq188_e2363_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq188_e2363_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq188_e2363_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq188_e2363_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq188_e2363_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq188_e2363_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq188_e2363_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq188_e2365: f64 = (eq188_e2363 * p.p248);
        let eq188_e2365_d_n0: f64 = (eq188_e2363_d_n0 * p.p248);
        let eq188_e2365_d_n1: f64 = (eq188_e2363_d_n1 * p.p248);
        let eq188_e2365_d_n2: f64 = (eq188_e2363_d_n2 * p.p248);
        let eq188_e2365_d_n3: f64 = (eq188_e2363_d_n3 * p.p248);
        let eq188_e2365_d_n4: f64 = (eq188_e2363_d_n4 * p.p248);
        let eq188_e2365_d_n5: f64 = (eq188_e2363_d_n5 * p.p248);
        let eq188_e2365_d_n6: f64 = (eq188_e2363_d_n6 * p.p248);
        let eq188_e2365_d_n7: f64 = (eq188_e2363_d_n7 * p.p248);
        let eq188_e2365_d_n8: f64 = (eq188_e2363_d_n8 * p.p248);
        let eq188_e2365_d_n9: f64 = (eq188_e2363_d_n9 * p.p248);
        let eq188_e2365_d_n10: f64 = (eq188_e2363_d_n10 * p.p248);
        let eq188_e2365_d_n11: f64 = (eq188_e2363_d_n11 * p.p248);
        let eq188_e2365_d_n12: f64 = (eq188_e2363_d_n12 * p.p248);
        let eq188_e2365_d_n13: f64 = (eq188_e2363_d_n13 * p.p248);
        let eq188_e2365_d_n14: f64 = (eq188_e2363_d_n14 * p.p248);
        let eq188_e2365_d_n15: f64 = (eq188_e2363_d_n15 * p.p248);
        let eq188_e2365_d_n16: f64 = (eq188_e2363_d_n16 * p.p248);
        let eq188_e2365_d_n17: f64 = (eq188_e2363_d_n17 * p.p248);
        let eq188_e2365_d_n18: f64 = (eq188_e2363_d_n18 * p.p248);
        let eq188_e2365_d_n19: f64 = (eq188_e2363_d_n19 * p.p248);
        let eq188_e2365_d_n20: f64 = (eq188_e2363_d_n20 * p.p248);
        let eq188_e2365_d_n21: f64 = (eq188_e2363_d_n21 * p.p248);
        let eq188_e2365_d_n22: f64 = (eq188_e2363_d_n22 * p.p248);
        let eq188_e2365_q: f64 = (eq188_e2363_q * p.p248);
        let eq188_e2365_q_d_n0: f64 = (eq188_e2363_q_d_n0 * p.p248);
        let eq188_e2365_q_d_n1: f64 = (eq188_e2363_q_d_n1 * p.p248);
        let eq188_e2365_q_d_n2: f64 = (eq188_e2363_q_d_n2 * p.p248);
        let eq188_e2365_q_d_n3: f64 = (eq188_e2363_q_d_n3 * p.p248);
        let eq188_e2365_q_d_n4: f64 = (eq188_e2363_q_d_n4 * p.p248);
        let eq188_e2365_q_d_n5: f64 = (eq188_e2363_q_d_n5 * p.p248);
        let eq188_e2365_q_d_n6: f64 = (eq188_e2363_q_d_n6 * p.p248);
        let eq188_e2365_q_d_n7: f64 = (eq188_e2363_q_d_n7 * p.p248);
        let eq188_e2365_q_d_n8: f64 = (eq188_e2363_q_d_n8 * p.p248);
        let eq188_e2365_q_d_n9: f64 = (eq188_e2363_q_d_n9 * p.p248);
        let eq188_e2365_q_d_n10: f64 = (eq188_e2363_q_d_n10 * p.p248);
        let eq188_e2365_q_d_n11: f64 = (eq188_e2363_q_d_n11 * p.p248);
        let eq188_e2365_q_d_n12: f64 = (eq188_e2363_q_d_n12 * p.p248);
        let eq188_e2365_q_d_n13: f64 = (eq188_e2363_q_d_n13 * p.p248);
        let eq188_e2365_q_d_n14: f64 = (eq188_e2363_q_d_n14 * p.p248);
        let eq188_e2365_q_d_n15: f64 = (eq188_e2363_q_d_n15 * p.p248);
        let eq188_e2365_q_d_n16: f64 = (eq188_e2363_q_d_n16 * p.p248);
        let eq188_e2365_q_d_n17: f64 = (eq188_e2363_q_d_n17 * p.p248);
        let eq188_e2365_q_d_n18: f64 = (eq188_e2363_q_d_n18 * p.p248);
        let eq188_e2365_q_d_n19: f64 = (eq188_e2363_q_d_n19 * p.p248);
        let eq188_e2365_q_d_n20: f64 = (eq188_e2363_q_d_n20 * p.p248);
        let eq188_e2365_q_d_n21: f64 = (eq188_e2363_q_d_n21 * p.p248);
        let eq188_e2365_q_d_n22: f64 = (eq188_e2363_q_d_n22 * p.p248);
        (eq188_e2365, eq188_e2365_d_n0, eq188_e2365_d_n1, eq188_e2365_d_n2, eq188_e2365_d_n3, eq188_e2365_d_n4, eq188_e2365_d_n5, eq188_e2365_d_n6, eq188_e2365_d_n7, eq188_e2365_d_n8, eq188_e2365_d_n9, eq188_e2365_d_n10, eq188_e2365_d_n11, eq188_e2365_d_n12, eq188_e2365_d_n13, eq188_e2365_d_n14, eq188_e2365_d_n15, eq188_e2365_d_n16, eq188_e2365_d_n17, eq188_e2365_d_n18, eq188_e2365_d_n19, eq188_e2365_d_n20, eq188_e2365_d_n21, eq188_e2365_d_n22, eq188_e2365_q, eq188_e2365_q_d_n0, eq188_e2365_q_d_n1, eq188_e2365_q_d_n2, eq188_e2365_q_d_n3, eq188_e2365_q_d_n4, eq188_e2365_q_d_n5, eq188_e2365_q_d_n6, eq188_e2365_q_d_n7, eq188_e2365_q_d_n8, eq188_e2365_q_d_n9, eq188_e2365_q_d_n10, eq188_e2365_q_d_n11, eq188_e2365_q_d_n12, eq188_e2365_q_d_n13, eq188_e2365_q_d_n14, eq188_e2365_q_d_n15, eq188_e2365_q_d_n16, eq188_e2365_q_d_n17, eq188_e2365_q_d_n18, eq188_e2365_q_d_n19, eq188_e2365_q_d_n20, eq188_e2365_q_d_n21, eq188_e2365_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq188_reactive_node_derivatives: [f64; 23] = [eq188_e2367_q_d_n0, eq188_e2367_q_d_n1, eq188_e2367_q_d_n2, eq188_e2367_q_d_n3, eq188_e2367_q_d_n4, eq188_e2367_q_d_n5, eq188_e2367_q_d_n6, eq188_e2367_q_d_n7, eq188_e2367_q_d_n8, eq188_e2367_q_d_n9, eq188_e2367_q_d_n10, eq188_e2367_q_d_n11, eq188_e2367_q_d_n12, eq188_e2367_q_d_n13, eq188_e2367_q_d_n14, eq188_e2367_q_d_n15, eq188_e2367_q_d_n16, eq188_e2367_q_d_n17, eq188_e2367_q_d_n18, eq188_e2367_q_d_n19, eq188_e2367_q_d_n20, eq188_e2367_q_d_n21, eq188_e2367_q_d_n22];
        let eq188_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq188_reactive_node_derivatives,
            branches,
            &eq188_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq189_e2380, eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22, eq189_e2380_q, eq189_e2380_q_d_n0, eq189_e2380_q_d_n1, eq189_e2380_q_d_n2, eq189_e2380_q_d_n3, eq189_e2380_q_d_n4, eq189_e2380_q_d_n5, eq189_e2380_q_d_n6, eq189_e2380_q_d_n7, eq189_e2380_q_d_n8, eq189_e2380_q_d_n9, eq189_e2380_q_d_n10, eq189_e2380_q_d_n11, eq189_e2380_q_d_n12, eq189_e2380_q_d_n13, eq189_e2380_q_d_n14, eq189_e2380_q_d_n15, eq189_e2380_q_d_n16, eq189_e2380_q_d_n17, eq189_e2380_q_d_n18, eq189_e2380_q_d_n19, eq189_e2380_q_d_n20, eq189_e2380_q_d_n21, eq189_e2380_q_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && (!s.b[599])) {
        let eq189_e2377_q: f64 = s.v[288];
        let eq189_e2378: f64 = (p.p7 * s.v[288]);
        let eq189_e2378_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq189_e2378_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq189_e2378_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq189_e2378_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq189_e2378_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq189_e2378_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq189_e2378_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq189_e2378_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq189_e2378_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq189_e2378_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq189_e2378_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq189_e2378_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq189_e2378_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq189_e2378_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq189_e2378_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq189_e2378_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq189_e2378_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq189_e2378_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq189_e2378_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq189_e2378_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq189_e2378_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq189_e2378_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq189_e2378_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq189_e2378_q: f64 = (p.p7 * eq189_e2377_q);
        let eq189_e2378_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq189_e2378_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq189_e2378_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq189_e2378_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq189_e2378_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq189_e2378_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq189_e2378_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq189_e2378_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq189_e2378_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq189_e2378_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq189_e2378_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq189_e2378_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq189_e2378_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq189_e2378_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq189_e2378_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq189_e2378_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq189_e2378_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq189_e2378_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq189_e2378_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq189_e2378_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq189_e2378_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq189_e2378_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq189_e2378_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        (eq189_e2378, eq189_e2378_d_n0, eq189_e2378_d_n1, eq189_e2378_d_n2, eq189_e2378_d_n3, eq189_e2378_d_n4, eq189_e2378_d_n5, eq189_e2378_d_n6, eq189_e2378_d_n7, eq189_e2378_d_n8, eq189_e2378_d_n9, eq189_e2378_d_n10, eq189_e2378_d_n11, eq189_e2378_d_n12, eq189_e2378_d_n13, eq189_e2378_d_n14, eq189_e2378_d_n15, eq189_e2378_d_n16, eq189_e2378_d_n17, eq189_e2378_d_n18, eq189_e2378_d_n19, eq189_e2378_d_n20, eq189_e2378_d_n21, eq189_e2378_d_n22, eq189_e2378_q, eq189_e2378_q_d_n0, eq189_e2378_q_d_n1, eq189_e2378_q_d_n2, eq189_e2378_q_d_n3, eq189_e2378_q_d_n4, eq189_e2378_q_d_n5, eq189_e2378_q_d_n6, eq189_e2378_q_d_n7, eq189_e2378_q_d_n8, eq189_e2378_q_d_n9, eq189_e2378_q_d_n10, eq189_e2378_q_d_n11, eq189_e2378_q_d_n12, eq189_e2378_q_d_n13, eq189_e2378_q_d_n14, eq189_e2378_q_d_n15, eq189_e2378_q_d_n16, eq189_e2378_q_d_n17, eq189_e2378_q_d_n18, eq189_e2378_q_d_n19, eq189_e2378_q_d_n20, eq189_e2378_q_d_n21, eq189_e2378_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq189_reactive_node_derivatives: [f64; 23] = [eq189_e2380_q_d_n0, eq189_e2380_q_d_n1, eq189_e2380_q_d_n2, eq189_e2380_q_d_n3, eq189_e2380_q_d_n4, eq189_e2380_q_d_n5, eq189_e2380_q_d_n6, eq189_e2380_q_d_n7, eq189_e2380_q_d_n8, eq189_e2380_q_d_n9, eq189_e2380_q_d_n10, eq189_e2380_q_d_n11, eq189_e2380_q_d_n12, eq189_e2380_q_d_n13, eq189_e2380_q_d_n14, eq189_e2380_q_d_n15, eq189_e2380_q_d_n16, eq189_e2380_q_d_n17, eq189_e2380_q_d_n18, eq189_e2380_q_d_n19, eq189_e2380_q_d_n20, eq189_e2380_q_d_n21, eq189_e2380_q_d_n22];
        let eq189_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq189_reactive_node_derivatives,
            branches,
            &eq189_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq190_e2395, eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22, eq190_e2395_q, eq190_e2395_q_d_n0, eq190_e2395_q_d_n1, eq190_e2395_q_d_n2, eq190_e2395_q_d_n3, eq190_e2395_q_d_n4, eq190_e2395_q_d_n5, eq190_e2395_q_d_n6, eq190_e2395_q_d_n7, eq190_e2395_q_d_n8, eq190_e2395_q_d_n9, eq190_e2395_q_d_n10, eq190_e2395_q_d_n11, eq190_e2395_q_d_n12, eq190_e2395_q_d_n13, eq190_e2395_q_d_n14, eq190_e2395_q_d_n15, eq190_e2395_q_d_n16, eq190_e2395_q_d_n17, eq190_e2395_q_d_n18, eq190_e2395_q_d_n19, eq190_e2395_q_d_n20, eq190_e2395_q_d_n21, eq190_e2395_q_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && (!s.b[599])) {
        let eq190_e2390_q: f64 = s.v[288];
        let eq190_e2391: f64 = (p.p7 * s.v[288]);
        let eq190_e2391_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq190_e2391_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq190_e2391_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq190_e2391_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq190_e2391_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq190_e2391_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq190_e2391_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq190_e2391_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq190_e2391_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq190_e2391_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq190_e2391_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq190_e2391_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq190_e2391_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq190_e2391_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq190_e2391_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq190_e2391_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq190_e2391_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq190_e2391_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq190_e2391_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq190_e2391_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq190_e2391_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq190_e2391_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq190_e2391_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq190_e2391_q: f64 = (p.p7 * eq190_e2390_q);
        let eq190_e2391_q_d_n0: f64 = (p.p7 * s.dn[288][0]);
        let eq190_e2391_q_d_n1: f64 = (p.p7 * s.dn[288][1]);
        let eq190_e2391_q_d_n2: f64 = (p.p7 * s.dn[288][2]);
        let eq190_e2391_q_d_n3: f64 = (p.p7 * s.dn[288][3]);
        let eq190_e2391_q_d_n4: f64 = (p.p7 * s.dn[288][4]);
        let eq190_e2391_q_d_n5: f64 = (p.p7 * s.dn[288][5]);
        let eq190_e2391_q_d_n6: f64 = (p.p7 * s.dn[288][6]);
        let eq190_e2391_q_d_n7: f64 = (p.p7 * s.dn[288][7]);
        let eq190_e2391_q_d_n8: f64 = (p.p7 * s.dn[288][8]);
        let eq190_e2391_q_d_n9: f64 = (p.p7 * s.dn[288][9]);
        let eq190_e2391_q_d_n10: f64 = (p.p7 * s.dn[288][10]);
        let eq190_e2391_q_d_n11: f64 = (p.p7 * s.dn[288][11]);
        let eq190_e2391_q_d_n12: f64 = (p.p7 * s.dn[288][12]);
        let eq190_e2391_q_d_n13: f64 = (p.p7 * s.dn[288][13]);
        let eq190_e2391_q_d_n14: f64 = (p.p7 * s.dn[288][14]);
        let eq190_e2391_q_d_n15: f64 = (p.p7 * s.dn[288][15]);
        let eq190_e2391_q_d_n16: f64 = (p.p7 * s.dn[288][16]);
        let eq190_e2391_q_d_n17: f64 = (p.p7 * s.dn[288][17]);
        let eq190_e2391_q_d_n18: f64 = (p.p7 * s.dn[288][18]);
        let eq190_e2391_q_d_n19: f64 = (p.p7 * s.dn[288][19]);
        let eq190_e2391_q_d_n20: f64 = (p.p7 * s.dn[288][20]);
        let eq190_e2391_q_d_n21: f64 = (p.p7 * s.dn[288][21]);
        let eq190_e2391_q_d_n22: f64 = (p.p7 * s.dn[288][22]);
        let eq190_e2393: f64 = (eq190_e2391 * p.p248);
        let eq190_e2393_d_n0: f64 = (eq190_e2391_d_n0 * p.p248);
        let eq190_e2393_d_n1: f64 = (eq190_e2391_d_n1 * p.p248);
        let eq190_e2393_d_n2: f64 = (eq190_e2391_d_n2 * p.p248);
        let eq190_e2393_d_n3: f64 = (eq190_e2391_d_n3 * p.p248);
        let eq190_e2393_d_n4: f64 = (eq190_e2391_d_n4 * p.p248);
        let eq190_e2393_d_n5: f64 = (eq190_e2391_d_n5 * p.p248);
        let eq190_e2393_d_n6: f64 = (eq190_e2391_d_n6 * p.p248);
        let eq190_e2393_d_n7: f64 = (eq190_e2391_d_n7 * p.p248);
        let eq190_e2393_d_n8: f64 = (eq190_e2391_d_n8 * p.p248);
        let eq190_e2393_d_n9: f64 = (eq190_e2391_d_n9 * p.p248);
        let eq190_e2393_d_n10: f64 = (eq190_e2391_d_n10 * p.p248);
        let eq190_e2393_d_n11: f64 = (eq190_e2391_d_n11 * p.p248);
        let eq190_e2393_d_n12: f64 = (eq190_e2391_d_n12 * p.p248);
        let eq190_e2393_d_n13: f64 = (eq190_e2391_d_n13 * p.p248);
        let eq190_e2393_d_n14: f64 = (eq190_e2391_d_n14 * p.p248);
        let eq190_e2393_d_n15: f64 = (eq190_e2391_d_n15 * p.p248);
        let eq190_e2393_d_n16: f64 = (eq190_e2391_d_n16 * p.p248);
        let eq190_e2393_d_n17: f64 = (eq190_e2391_d_n17 * p.p248);
        let eq190_e2393_d_n18: f64 = (eq190_e2391_d_n18 * p.p248);
        let eq190_e2393_d_n19: f64 = (eq190_e2391_d_n19 * p.p248);
        let eq190_e2393_d_n20: f64 = (eq190_e2391_d_n20 * p.p248);
        let eq190_e2393_d_n21: f64 = (eq190_e2391_d_n21 * p.p248);
        let eq190_e2393_d_n22: f64 = (eq190_e2391_d_n22 * p.p248);
        let eq190_e2393_q: f64 = (eq190_e2391_q * p.p248);
        let eq190_e2393_q_d_n0: f64 = (eq190_e2391_q_d_n0 * p.p248);
        let eq190_e2393_q_d_n1: f64 = (eq190_e2391_q_d_n1 * p.p248);
        let eq190_e2393_q_d_n2: f64 = (eq190_e2391_q_d_n2 * p.p248);
        let eq190_e2393_q_d_n3: f64 = (eq190_e2391_q_d_n3 * p.p248);
        let eq190_e2393_q_d_n4: f64 = (eq190_e2391_q_d_n4 * p.p248);
        let eq190_e2393_q_d_n5: f64 = (eq190_e2391_q_d_n5 * p.p248);
        let eq190_e2393_q_d_n6: f64 = (eq190_e2391_q_d_n6 * p.p248);
        let eq190_e2393_q_d_n7: f64 = (eq190_e2391_q_d_n7 * p.p248);
        let eq190_e2393_q_d_n8: f64 = (eq190_e2391_q_d_n8 * p.p248);
        let eq190_e2393_q_d_n9: f64 = (eq190_e2391_q_d_n9 * p.p248);
        let eq190_e2393_q_d_n10: f64 = (eq190_e2391_q_d_n10 * p.p248);
        let eq190_e2393_q_d_n11: f64 = (eq190_e2391_q_d_n11 * p.p248);
        let eq190_e2393_q_d_n12: f64 = (eq190_e2391_q_d_n12 * p.p248);
        let eq190_e2393_q_d_n13: f64 = (eq190_e2391_q_d_n13 * p.p248);
        let eq190_e2393_q_d_n14: f64 = (eq190_e2391_q_d_n14 * p.p248);
        let eq190_e2393_q_d_n15: f64 = (eq190_e2391_q_d_n15 * p.p248);
        let eq190_e2393_q_d_n16: f64 = (eq190_e2391_q_d_n16 * p.p248);
        let eq190_e2393_q_d_n17: f64 = (eq190_e2391_q_d_n17 * p.p248);
        let eq190_e2393_q_d_n18: f64 = (eq190_e2391_q_d_n18 * p.p248);
        let eq190_e2393_q_d_n19: f64 = (eq190_e2391_q_d_n19 * p.p248);
        let eq190_e2393_q_d_n20: f64 = (eq190_e2391_q_d_n20 * p.p248);
        let eq190_e2393_q_d_n21: f64 = (eq190_e2391_q_d_n21 * p.p248);
        let eq190_e2393_q_d_n22: f64 = (eq190_e2391_q_d_n22 * p.p248);
        (eq190_e2393, eq190_e2393_d_n0, eq190_e2393_d_n1, eq190_e2393_d_n2, eq190_e2393_d_n3, eq190_e2393_d_n4, eq190_e2393_d_n5, eq190_e2393_d_n6, eq190_e2393_d_n7, eq190_e2393_d_n8, eq190_e2393_d_n9, eq190_e2393_d_n10, eq190_e2393_d_n11, eq190_e2393_d_n12, eq190_e2393_d_n13, eq190_e2393_d_n14, eq190_e2393_d_n15, eq190_e2393_d_n16, eq190_e2393_d_n17, eq190_e2393_d_n18, eq190_e2393_d_n19, eq190_e2393_d_n20, eq190_e2393_d_n21, eq190_e2393_d_n22, eq190_e2393_q, eq190_e2393_q_d_n0, eq190_e2393_q_d_n1, eq190_e2393_q_d_n2, eq190_e2393_q_d_n3, eq190_e2393_q_d_n4, eq190_e2393_q_d_n5, eq190_e2393_q_d_n6, eq190_e2393_q_d_n7, eq190_e2393_q_d_n8, eq190_e2393_q_d_n9, eq190_e2393_q_d_n10, eq190_e2393_q_d_n11, eq190_e2393_q_d_n12, eq190_e2393_q_d_n13, eq190_e2393_q_d_n14, eq190_e2393_q_d_n15, eq190_e2393_q_d_n16, eq190_e2393_q_d_n17, eq190_e2393_q_d_n18, eq190_e2393_q_d_n19, eq190_e2393_q_d_n20, eq190_e2393_q_d_n21, eq190_e2393_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq190_reactive_node_derivatives: [f64; 23] = [eq190_e2395_q_d_n0, eq190_e2395_q_d_n1, eq190_e2395_q_d_n2, eq190_e2395_q_d_n3, eq190_e2395_q_d_n4, eq190_e2395_q_d_n5, eq190_e2395_q_d_n6, eq190_e2395_q_d_n7, eq190_e2395_q_d_n8, eq190_e2395_q_d_n9, eq190_e2395_q_d_n10, eq190_e2395_q_d_n11, eq190_e2395_q_d_n12, eq190_e2395_q_d_n13, eq190_e2395_q_d_n14, eq190_e2395_q_d_n15, eq190_e2395_q_d_n16, eq190_e2395_q_d_n17, eq190_e2395_q_d_n18, eq190_e2395_q_d_n19, eq190_e2395_q_d_n20, eq190_e2395_q_d_n21, eq190_e2395_q_d_n22];
        let eq190_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq190_reactive_node_derivatives,
            branches,
            &eq190_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq191_e2407, eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22, eq191_e2407_q, eq191_e2407_q_d_n0, eq191_e2407_q_d_n1, eq191_e2407_q_d_n2, eq191_e2407_q_d_n3, eq191_e2407_q_d_n4, eq191_e2407_q_d_n5, eq191_e2407_q_d_n6, eq191_e2407_q_d_n7, eq191_e2407_q_d_n8, eq191_e2407_q_d_n9, eq191_e2407_q_d_n10, eq191_e2407_q_d_n11, eq191_e2407_q_d_n12, eq191_e2407_q_d_n13, eq191_e2407_q_d_n14, eq191_e2407_q_d_n15, eq191_e2407_q_d_n16, eq191_e2407_q_d_n17, eq191_e2407_q_d_n18, eq191_e2407_q_d_n19, eq191_e2407_q_d_n20, eq191_e2407_q_d_n21, eq191_e2407_q_d_n22,) = {
    if ((!s.b[595]) && s.b[598]) {
        let eq191_e2403: f64 = (p.p253 * s.v[288]);
        let eq191_e2403_d_n0: f64 = (p.p253 * s.dn[288][0]);
        let eq191_e2403_d_n1: f64 = (p.p253 * s.dn[288][1]);
        let eq191_e2403_d_n2: f64 = (p.p253 * s.dn[288][2]);
        let eq191_e2403_d_n3: f64 = (p.p253 * s.dn[288][3]);
        let eq191_e2403_d_n4: f64 = (p.p253 * s.dn[288][4]);
        let eq191_e2403_d_n5: f64 = (p.p253 * s.dn[288][5]);
        let eq191_e2403_d_n6: f64 = (p.p253 * s.dn[288][6]);
        let eq191_e2403_d_n7: f64 = (p.p253 * s.dn[288][7]);
        let eq191_e2403_d_n8: f64 = (p.p253 * s.dn[288][8]);
        let eq191_e2403_d_n9: f64 = (p.p253 * s.dn[288][9]);
        let eq191_e2403_d_n10: f64 = (p.p253 * s.dn[288][10]);
        let eq191_e2403_d_n11: f64 = (p.p253 * s.dn[288][11]);
        let eq191_e2403_d_n12: f64 = (p.p253 * s.dn[288][12]);
        let eq191_e2403_d_n13: f64 = (p.p253 * s.dn[288][13]);
        let eq191_e2403_d_n14: f64 = (p.p253 * s.dn[288][14]);
        let eq191_e2403_d_n15: f64 = (p.p253 * s.dn[288][15]);
        let eq191_e2403_d_n16: f64 = (p.p253 * s.dn[288][16]);
        let eq191_e2403_d_n17: f64 = (p.p253 * s.dn[288][17]);
        let eq191_e2403_d_n18: f64 = (p.p253 * s.dn[288][18]);
        let eq191_e2403_d_n19: f64 = (p.p253 * s.dn[288][19]);
        let eq191_e2403_d_n20: f64 = (p.p253 * s.dn[288][20]);
        let eq191_e2403_d_n21: f64 = (p.p253 * s.dn[288][21]);
        let eq191_e2403_d_n22: f64 = (p.p253 * s.dn[288][22]);
        let eq191_e2404_q: f64 = eq191_e2403;
        let eq191_e2405: f64 = (p.p7 * eq191_e2403);
        let eq191_e2405_d_n0: f64 = (p.p7 * eq191_e2403_d_n0);
        let eq191_e2405_d_n1: f64 = (p.p7 * eq191_e2403_d_n1);
        let eq191_e2405_d_n2: f64 = (p.p7 * eq191_e2403_d_n2);
        let eq191_e2405_d_n3: f64 = (p.p7 * eq191_e2403_d_n3);
        let eq191_e2405_d_n4: f64 = (p.p7 * eq191_e2403_d_n4);
        let eq191_e2405_d_n5: f64 = (p.p7 * eq191_e2403_d_n5);
        let eq191_e2405_d_n6: f64 = (p.p7 * eq191_e2403_d_n6);
        let eq191_e2405_d_n7: f64 = (p.p7 * eq191_e2403_d_n7);
        let eq191_e2405_d_n8: f64 = (p.p7 * eq191_e2403_d_n8);
        let eq191_e2405_d_n9: f64 = (p.p7 * eq191_e2403_d_n9);
        let eq191_e2405_d_n10: f64 = (p.p7 * eq191_e2403_d_n10);
        let eq191_e2405_d_n11: f64 = (p.p7 * eq191_e2403_d_n11);
        let eq191_e2405_d_n12: f64 = (p.p7 * eq191_e2403_d_n12);
        let eq191_e2405_d_n13: f64 = (p.p7 * eq191_e2403_d_n13);
        let eq191_e2405_d_n14: f64 = (p.p7 * eq191_e2403_d_n14);
        let eq191_e2405_d_n15: f64 = (p.p7 * eq191_e2403_d_n15);
        let eq191_e2405_d_n16: f64 = (p.p7 * eq191_e2403_d_n16);
        let eq191_e2405_d_n17: f64 = (p.p7 * eq191_e2403_d_n17);
        let eq191_e2405_d_n18: f64 = (p.p7 * eq191_e2403_d_n18);
        let eq191_e2405_d_n19: f64 = (p.p7 * eq191_e2403_d_n19);
        let eq191_e2405_d_n20: f64 = (p.p7 * eq191_e2403_d_n20);
        let eq191_e2405_d_n21: f64 = (p.p7 * eq191_e2403_d_n21);
        let eq191_e2405_d_n22: f64 = (p.p7 * eq191_e2403_d_n22);
        let eq191_e2405_q: f64 = (p.p7 * eq191_e2404_q);
        let eq191_e2405_q_d_n0: f64 = (p.p7 * eq191_e2403_d_n0);
        let eq191_e2405_q_d_n1: f64 = (p.p7 * eq191_e2403_d_n1);
        let eq191_e2405_q_d_n2: f64 = (p.p7 * eq191_e2403_d_n2);
        let eq191_e2405_q_d_n3: f64 = (p.p7 * eq191_e2403_d_n3);
        let eq191_e2405_q_d_n4: f64 = (p.p7 * eq191_e2403_d_n4);
        let eq191_e2405_q_d_n5: f64 = (p.p7 * eq191_e2403_d_n5);
        let eq191_e2405_q_d_n6: f64 = (p.p7 * eq191_e2403_d_n6);
        let eq191_e2405_q_d_n7: f64 = (p.p7 * eq191_e2403_d_n7);
        let eq191_e2405_q_d_n8: f64 = (p.p7 * eq191_e2403_d_n8);
        let eq191_e2405_q_d_n9: f64 = (p.p7 * eq191_e2403_d_n9);
        let eq191_e2405_q_d_n10: f64 = (p.p7 * eq191_e2403_d_n10);
        let eq191_e2405_q_d_n11: f64 = (p.p7 * eq191_e2403_d_n11);
        let eq191_e2405_q_d_n12: f64 = (p.p7 * eq191_e2403_d_n12);
        let eq191_e2405_q_d_n13: f64 = (p.p7 * eq191_e2403_d_n13);
        let eq191_e2405_q_d_n14: f64 = (p.p7 * eq191_e2403_d_n14);
        let eq191_e2405_q_d_n15: f64 = (p.p7 * eq191_e2403_d_n15);
        let eq191_e2405_q_d_n16: f64 = (p.p7 * eq191_e2403_d_n16);
        let eq191_e2405_q_d_n17: f64 = (p.p7 * eq191_e2403_d_n17);
        let eq191_e2405_q_d_n18: f64 = (p.p7 * eq191_e2403_d_n18);
        let eq191_e2405_q_d_n19: f64 = (p.p7 * eq191_e2403_d_n19);
        let eq191_e2405_q_d_n20: f64 = (p.p7 * eq191_e2403_d_n20);
        let eq191_e2405_q_d_n21: f64 = (p.p7 * eq191_e2403_d_n21);
        let eq191_e2405_q_d_n22: f64 = (p.p7 * eq191_e2403_d_n22);
        (eq191_e2405, eq191_e2405_d_n0, eq191_e2405_d_n1, eq191_e2405_d_n2, eq191_e2405_d_n3, eq191_e2405_d_n4, eq191_e2405_d_n5, eq191_e2405_d_n6, eq191_e2405_d_n7, eq191_e2405_d_n8, eq191_e2405_d_n9, eq191_e2405_d_n10, eq191_e2405_d_n11, eq191_e2405_d_n12, eq191_e2405_d_n13, eq191_e2405_d_n14, eq191_e2405_d_n15, eq191_e2405_d_n16, eq191_e2405_d_n17, eq191_e2405_d_n18, eq191_e2405_d_n19, eq191_e2405_d_n20, eq191_e2405_d_n21, eq191_e2405_d_n22, eq191_e2405_q, eq191_e2405_q_d_n0, eq191_e2405_q_d_n1, eq191_e2405_q_d_n2, eq191_e2405_q_d_n3, eq191_e2405_q_d_n4, eq191_e2405_q_d_n5, eq191_e2405_q_d_n6, eq191_e2405_q_d_n7, eq191_e2405_q_d_n8, eq191_e2405_q_d_n9, eq191_e2405_q_d_n10, eq191_e2405_q_d_n11, eq191_e2405_q_d_n12, eq191_e2405_q_d_n13, eq191_e2405_q_d_n14, eq191_e2405_q_d_n15, eq191_e2405_q_d_n16, eq191_e2405_q_d_n17, eq191_e2405_q_d_n18, eq191_e2405_q_d_n19, eq191_e2405_q_d_n20, eq191_e2405_q_d_n21, eq191_e2405_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq191_reactive_node_derivatives: [f64; 23] = [eq191_e2407_q_d_n0, eq191_e2407_q_d_n1, eq191_e2407_q_d_n2, eq191_e2407_q_d_n3, eq191_e2407_q_d_n4, eq191_e2407_q_d_n5, eq191_e2407_q_d_n6, eq191_e2407_q_d_n7, eq191_e2407_q_d_n8, eq191_e2407_q_d_n9, eq191_e2407_q_d_n10, eq191_e2407_q_d_n11, eq191_e2407_q_d_n12, eq191_e2407_q_d_n13, eq191_e2407_q_d_n14, eq191_e2407_q_d_n15, eq191_e2407_q_d_n16, eq191_e2407_q_d_n17, eq191_e2407_q_d_n18, eq191_e2407_q_d_n19, eq191_e2407_q_d_n20, eq191_e2407_q_d_n21, eq191_e2407_q_d_n22];
        let eq191_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq191_reactive_node_derivatives,
            branches,
            &eq191_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq192_e2416, eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22, eq192_e2416_q, eq192_e2416_q_d_n0, eq192_e2416_q_d_n1, eq192_e2416_q_d_n2, eq192_e2416_q_d_n3, eq192_e2416_q_d_n4, eq192_e2416_q_d_n5, eq192_e2416_q_d_n6, eq192_e2416_q_d_n7, eq192_e2416_q_d_n8, eq192_e2416_q_d_n9, eq192_e2416_q_d_n10, eq192_e2416_q_d_n11, eq192_e2416_q_d_n12, eq192_e2416_q_d_n13, eq192_e2416_q_d_n14, eq192_e2416_q_d_n15, eq192_e2416_q_d_n16, eq192_e2416_q_d_n17, eq192_e2416_q_d_n18, eq192_e2416_q_d_n19, eq192_e2416_q_d_n20, eq192_e2416_q_d_n21, eq192_e2416_q_d_n22,) = {
    if (s.b[600] && s.b[601]) {
        let eq192_e2413_q: f64 = s.v[301];
        let eq192_e2414: f64 = (p.p7 * s.v[301]);
        let eq192_e2414_d_n0: f64 = (p.p7 * s.dn[301][0]);
        let eq192_e2414_d_n1: f64 = (p.p7 * s.dn[301][1]);
        let eq192_e2414_d_n2: f64 = (p.p7 * s.dn[301][2]);
        let eq192_e2414_d_n3: f64 = (p.p7 * s.dn[301][3]);
        let eq192_e2414_d_n4: f64 = (p.p7 * s.dn[301][4]);
        let eq192_e2414_d_n5: f64 = (p.p7 * s.dn[301][5]);
        let eq192_e2414_d_n6: f64 = (p.p7 * s.dn[301][6]);
        let eq192_e2414_d_n7: f64 = (p.p7 * s.dn[301][7]);
        let eq192_e2414_d_n8: f64 = (p.p7 * s.dn[301][8]);
        let eq192_e2414_d_n9: f64 = (p.p7 * s.dn[301][9]);
        let eq192_e2414_d_n10: f64 = (p.p7 * s.dn[301][10]);
        let eq192_e2414_d_n11: f64 = (p.p7 * s.dn[301][11]);
        let eq192_e2414_d_n12: f64 = (p.p7 * s.dn[301][12]);
        let eq192_e2414_d_n13: f64 = (p.p7 * s.dn[301][13]);
        let eq192_e2414_d_n14: f64 = (p.p7 * s.dn[301][14]);
        let eq192_e2414_d_n15: f64 = (p.p7 * s.dn[301][15]);
        let eq192_e2414_d_n16: f64 = (p.p7 * s.dn[301][16]);
        let eq192_e2414_d_n17: f64 = (p.p7 * s.dn[301][17]);
        let eq192_e2414_d_n18: f64 = (p.p7 * s.dn[301][18]);
        let eq192_e2414_d_n19: f64 = (p.p7 * s.dn[301][19]);
        let eq192_e2414_d_n20: f64 = (p.p7 * s.dn[301][20]);
        let eq192_e2414_d_n21: f64 = (p.p7 * s.dn[301][21]);
        let eq192_e2414_d_n22: f64 = (p.p7 * s.dn[301][22]);
        let eq192_e2414_q: f64 = (p.p7 * eq192_e2413_q);
        let eq192_e2414_q_d_n0: f64 = (p.p7 * s.dn[301][0]);
        let eq192_e2414_q_d_n1: f64 = (p.p7 * s.dn[301][1]);
        let eq192_e2414_q_d_n2: f64 = (p.p7 * s.dn[301][2]);
        let eq192_e2414_q_d_n3: f64 = (p.p7 * s.dn[301][3]);
        let eq192_e2414_q_d_n4: f64 = (p.p7 * s.dn[301][4]);
        let eq192_e2414_q_d_n5: f64 = (p.p7 * s.dn[301][5]);
        let eq192_e2414_q_d_n6: f64 = (p.p7 * s.dn[301][6]);
        let eq192_e2414_q_d_n7: f64 = (p.p7 * s.dn[301][7]);
        let eq192_e2414_q_d_n8: f64 = (p.p7 * s.dn[301][8]);
        let eq192_e2414_q_d_n9: f64 = (p.p7 * s.dn[301][9]);
        let eq192_e2414_q_d_n10: f64 = (p.p7 * s.dn[301][10]);
        let eq192_e2414_q_d_n11: f64 = (p.p7 * s.dn[301][11]);
        let eq192_e2414_q_d_n12: f64 = (p.p7 * s.dn[301][12]);
        let eq192_e2414_q_d_n13: f64 = (p.p7 * s.dn[301][13]);
        let eq192_e2414_q_d_n14: f64 = (p.p7 * s.dn[301][14]);
        let eq192_e2414_q_d_n15: f64 = (p.p7 * s.dn[301][15]);
        let eq192_e2414_q_d_n16: f64 = (p.p7 * s.dn[301][16]);
        let eq192_e2414_q_d_n17: f64 = (p.p7 * s.dn[301][17]);
        let eq192_e2414_q_d_n18: f64 = (p.p7 * s.dn[301][18]);
        let eq192_e2414_q_d_n19: f64 = (p.p7 * s.dn[301][19]);
        let eq192_e2414_q_d_n20: f64 = (p.p7 * s.dn[301][20]);
        let eq192_e2414_q_d_n21: f64 = (p.p7 * s.dn[301][21]);
        let eq192_e2414_q_d_n22: f64 = (p.p7 * s.dn[301][22]);
        (eq192_e2414, eq192_e2414_d_n0, eq192_e2414_d_n1, eq192_e2414_d_n2, eq192_e2414_d_n3, eq192_e2414_d_n4, eq192_e2414_d_n5, eq192_e2414_d_n6, eq192_e2414_d_n7, eq192_e2414_d_n8, eq192_e2414_d_n9, eq192_e2414_d_n10, eq192_e2414_d_n11, eq192_e2414_d_n12, eq192_e2414_d_n13, eq192_e2414_d_n14, eq192_e2414_d_n15, eq192_e2414_d_n16, eq192_e2414_d_n17, eq192_e2414_d_n18, eq192_e2414_d_n19, eq192_e2414_d_n20, eq192_e2414_d_n21, eq192_e2414_d_n22, eq192_e2414_q, eq192_e2414_q_d_n0, eq192_e2414_q_d_n1, eq192_e2414_q_d_n2, eq192_e2414_q_d_n3, eq192_e2414_q_d_n4, eq192_e2414_q_d_n5, eq192_e2414_q_d_n6, eq192_e2414_q_d_n7, eq192_e2414_q_d_n8, eq192_e2414_q_d_n9, eq192_e2414_q_d_n10, eq192_e2414_q_d_n11, eq192_e2414_q_d_n12, eq192_e2414_q_d_n13, eq192_e2414_q_d_n14, eq192_e2414_q_d_n15, eq192_e2414_q_d_n16, eq192_e2414_q_d_n17, eq192_e2414_q_d_n18, eq192_e2414_q_d_n19, eq192_e2414_q_d_n20, eq192_e2414_q_d_n21, eq192_e2414_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq192_reactive_node_derivatives: [f64; 23] = [eq192_e2416_q_d_n0, eq192_e2416_q_d_n1, eq192_e2416_q_d_n2, eq192_e2416_q_d_n3, eq192_e2416_q_d_n4, eq192_e2416_q_d_n5, eq192_e2416_q_d_n6, eq192_e2416_q_d_n7, eq192_e2416_q_d_n8, eq192_e2416_q_d_n9, eq192_e2416_q_d_n10, eq192_e2416_q_d_n11, eq192_e2416_q_d_n12, eq192_e2416_q_d_n13, eq192_e2416_q_d_n14, eq192_e2416_q_d_n15, eq192_e2416_q_d_n16, eq192_e2416_q_d_n17, eq192_e2416_q_d_n18, eq192_e2416_q_d_n19, eq192_e2416_q_d_n20, eq192_e2416_q_d_n21, eq192_e2416_q_d_n22];
        let eq192_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            nodes,
            &eq192_reactive_node_derivatives,
            branches,
            &eq192_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_16(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq193_e2427, eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22, eq193_e2427_q, eq193_e2427_q_d_n0, eq193_e2427_q_d_n1, eq193_e2427_q_d_n2, eq193_e2427_q_d_n3, eq193_e2427_q_d_n4, eq193_e2427_q_d_n5, eq193_e2427_q_d_n6, eq193_e2427_q_d_n7, eq193_e2427_q_d_n8, eq193_e2427_q_d_n9, eq193_e2427_q_d_n10, eq193_e2427_q_d_n11, eq193_e2427_q_d_n12, eq193_e2427_q_d_n13, eq193_e2427_q_d_n14, eq193_e2427_q_d_n15, eq193_e2427_q_d_n16, eq193_e2427_q_d_n17, eq193_e2427_q_d_n18, eq193_e2427_q_d_n19, eq193_e2427_q_d_n20, eq193_e2427_q_d_n21, eq193_e2427_q_d_n22,) = {
    if ((s.b[600] && s.b[601]) && s.b[602]) {
        let eq193_e2424_q: f64 = s.v[300];
        let eq193_e2425: f64 = (p.p7 * s.v[300]);
        let eq193_e2425_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq193_e2425_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq193_e2425_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq193_e2425_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq193_e2425_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq193_e2425_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq193_e2425_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq193_e2425_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq193_e2425_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq193_e2425_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq193_e2425_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq193_e2425_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq193_e2425_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq193_e2425_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq193_e2425_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq193_e2425_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq193_e2425_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq193_e2425_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq193_e2425_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq193_e2425_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq193_e2425_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq193_e2425_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq193_e2425_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq193_e2425_q: f64 = (p.p7 * eq193_e2424_q);
        let eq193_e2425_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq193_e2425_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq193_e2425_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq193_e2425_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq193_e2425_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq193_e2425_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq193_e2425_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq193_e2425_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq193_e2425_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq193_e2425_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq193_e2425_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq193_e2425_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq193_e2425_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq193_e2425_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq193_e2425_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq193_e2425_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq193_e2425_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq193_e2425_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq193_e2425_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq193_e2425_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq193_e2425_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq193_e2425_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq193_e2425_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
        (eq193_e2425, eq193_e2425_d_n0, eq193_e2425_d_n1, eq193_e2425_d_n2, eq193_e2425_d_n3, eq193_e2425_d_n4, eq193_e2425_d_n5, eq193_e2425_d_n6, eq193_e2425_d_n7, eq193_e2425_d_n8, eq193_e2425_d_n9, eq193_e2425_d_n10, eq193_e2425_d_n11, eq193_e2425_d_n12, eq193_e2425_d_n13, eq193_e2425_d_n14, eq193_e2425_d_n15, eq193_e2425_d_n16, eq193_e2425_d_n17, eq193_e2425_d_n18, eq193_e2425_d_n19, eq193_e2425_d_n20, eq193_e2425_d_n21, eq193_e2425_d_n22, eq193_e2425_q, eq193_e2425_q_d_n0, eq193_e2425_q_d_n1, eq193_e2425_q_d_n2, eq193_e2425_q_d_n3, eq193_e2425_q_d_n4, eq193_e2425_q_d_n5, eq193_e2425_q_d_n6, eq193_e2425_q_d_n7, eq193_e2425_q_d_n8, eq193_e2425_q_d_n9, eq193_e2425_q_d_n10, eq193_e2425_q_d_n11, eq193_e2425_q_d_n12, eq193_e2425_q_d_n13, eq193_e2425_q_d_n14, eq193_e2425_q_d_n15, eq193_e2425_q_d_n16, eq193_e2425_q_d_n17, eq193_e2425_q_d_n18, eq193_e2425_q_d_n19, eq193_e2425_q_d_n20, eq193_e2425_q_d_n21, eq193_e2425_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq193_reactive_node_derivatives: [f64; 23] = [eq193_e2427_q_d_n0, eq193_e2427_q_d_n1, eq193_e2427_q_d_n2, eq193_e2427_q_d_n3, eq193_e2427_q_d_n4, eq193_e2427_q_d_n5, eq193_e2427_q_d_n6, eq193_e2427_q_d_n7, eq193_e2427_q_d_n8, eq193_e2427_q_d_n9, eq193_e2427_q_d_n10, eq193_e2427_q_d_n11, eq193_e2427_q_d_n12, eq193_e2427_q_d_n13, eq193_e2427_q_d_n14, eq193_e2427_q_d_n15, eq193_e2427_q_d_n16, eq193_e2427_q_d_n17, eq193_e2427_q_d_n18, eq193_e2427_q_d_n19, eq193_e2427_q_d_n20, eq193_e2427_q_d_n21, eq193_e2427_q_d_n22];
        let eq193_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            nodes,
            &eq193_reactive_node_derivatives,
            branches,
            &eq193_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq194_e2440, eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22, eq194_e2440_q, eq194_e2440_q_d_n0, eq194_e2440_q_d_n1, eq194_e2440_q_d_n2, eq194_e2440_q_d_n3, eq194_e2440_q_d_n4, eq194_e2440_q_d_n5, eq194_e2440_q_d_n6, eq194_e2440_q_d_n7, eq194_e2440_q_d_n8, eq194_e2440_q_d_n9, eq194_e2440_q_d_n10, eq194_e2440_q_d_n11, eq194_e2440_q_d_n12, eq194_e2440_q_d_n13, eq194_e2440_q_d_n14, eq194_e2440_q_d_n15, eq194_e2440_q_d_n16, eq194_e2440_q_d_n17, eq194_e2440_q_d_n18, eq194_e2440_q_d_n19, eq194_e2440_q_d_n20, eq194_e2440_q_d_n21, eq194_e2440_q_d_n22,) = {
    if ((s.b[600] && s.b[601]) && s.b[602]) {
        let eq194_e2435_q: f64 = s.v[300];
        let eq194_e2436: f64 = (p.p7 * s.v[300]);
        let eq194_e2436_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq194_e2436_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq194_e2436_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq194_e2436_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq194_e2436_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq194_e2436_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq194_e2436_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq194_e2436_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq194_e2436_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq194_e2436_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq194_e2436_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq194_e2436_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq194_e2436_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq194_e2436_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq194_e2436_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq194_e2436_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq194_e2436_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq194_e2436_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq194_e2436_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq194_e2436_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq194_e2436_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq194_e2436_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq194_e2436_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq194_e2436_q: f64 = (p.p7 * eq194_e2435_q);
        let eq194_e2436_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq194_e2436_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq194_e2436_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq194_e2436_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq194_e2436_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq194_e2436_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq194_e2436_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq194_e2436_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq194_e2436_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq194_e2436_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq194_e2436_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq194_e2436_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq194_e2436_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq194_e2436_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq194_e2436_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq194_e2436_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq194_e2436_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq194_e2436_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq194_e2436_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq194_e2436_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq194_e2436_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq194_e2436_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq194_e2436_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq194_e2438: f64 = (eq194_e2436 * p.p249);
        let eq194_e2438_d_n0: f64 = (eq194_e2436_d_n0 * p.p249);
        let eq194_e2438_d_n1: f64 = (eq194_e2436_d_n1 * p.p249);
        let eq194_e2438_d_n2: f64 = (eq194_e2436_d_n2 * p.p249);
        let eq194_e2438_d_n3: f64 = (eq194_e2436_d_n3 * p.p249);
        let eq194_e2438_d_n4: f64 = (eq194_e2436_d_n4 * p.p249);
        let eq194_e2438_d_n5: f64 = (eq194_e2436_d_n5 * p.p249);
        let eq194_e2438_d_n6: f64 = (eq194_e2436_d_n6 * p.p249);
        let eq194_e2438_d_n7: f64 = (eq194_e2436_d_n7 * p.p249);
        let eq194_e2438_d_n8: f64 = (eq194_e2436_d_n8 * p.p249);
        let eq194_e2438_d_n9: f64 = (eq194_e2436_d_n9 * p.p249);
        let eq194_e2438_d_n10: f64 = (eq194_e2436_d_n10 * p.p249);
        let eq194_e2438_d_n11: f64 = (eq194_e2436_d_n11 * p.p249);
        let eq194_e2438_d_n12: f64 = (eq194_e2436_d_n12 * p.p249);
        let eq194_e2438_d_n13: f64 = (eq194_e2436_d_n13 * p.p249);
        let eq194_e2438_d_n14: f64 = (eq194_e2436_d_n14 * p.p249);
        let eq194_e2438_d_n15: f64 = (eq194_e2436_d_n15 * p.p249);
        let eq194_e2438_d_n16: f64 = (eq194_e2436_d_n16 * p.p249);
        let eq194_e2438_d_n17: f64 = (eq194_e2436_d_n17 * p.p249);
        let eq194_e2438_d_n18: f64 = (eq194_e2436_d_n18 * p.p249);
        let eq194_e2438_d_n19: f64 = (eq194_e2436_d_n19 * p.p249);
        let eq194_e2438_d_n20: f64 = (eq194_e2436_d_n20 * p.p249);
        let eq194_e2438_d_n21: f64 = (eq194_e2436_d_n21 * p.p249);
        let eq194_e2438_d_n22: f64 = (eq194_e2436_d_n22 * p.p249);
        let eq194_e2438_q: f64 = (eq194_e2436_q * p.p249);
        let eq194_e2438_q_d_n0: f64 = (eq194_e2436_q_d_n0 * p.p249);
        let eq194_e2438_q_d_n1: f64 = (eq194_e2436_q_d_n1 * p.p249);
        let eq194_e2438_q_d_n2: f64 = (eq194_e2436_q_d_n2 * p.p249);
        let eq194_e2438_q_d_n3: f64 = (eq194_e2436_q_d_n3 * p.p249);
        let eq194_e2438_q_d_n4: f64 = (eq194_e2436_q_d_n4 * p.p249);
        let eq194_e2438_q_d_n5: f64 = (eq194_e2436_q_d_n5 * p.p249);
        let eq194_e2438_q_d_n6: f64 = (eq194_e2436_q_d_n6 * p.p249);
        let eq194_e2438_q_d_n7: f64 = (eq194_e2436_q_d_n7 * p.p249);
        let eq194_e2438_q_d_n8: f64 = (eq194_e2436_q_d_n8 * p.p249);
        let eq194_e2438_q_d_n9: f64 = (eq194_e2436_q_d_n9 * p.p249);
        let eq194_e2438_q_d_n10: f64 = (eq194_e2436_q_d_n10 * p.p249);
        let eq194_e2438_q_d_n11: f64 = (eq194_e2436_q_d_n11 * p.p249);
        let eq194_e2438_q_d_n12: f64 = (eq194_e2436_q_d_n12 * p.p249);
        let eq194_e2438_q_d_n13: f64 = (eq194_e2436_q_d_n13 * p.p249);
        let eq194_e2438_q_d_n14: f64 = (eq194_e2436_q_d_n14 * p.p249);
        let eq194_e2438_q_d_n15: f64 = (eq194_e2436_q_d_n15 * p.p249);
        let eq194_e2438_q_d_n16: f64 = (eq194_e2436_q_d_n16 * p.p249);
        let eq194_e2438_q_d_n17: f64 = (eq194_e2436_q_d_n17 * p.p249);
        let eq194_e2438_q_d_n18: f64 = (eq194_e2436_q_d_n18 * p.p249);
        let eq194_e2438_q_d_n19: f64 = (eq194_e2436_q_d_n19 * p.p249);
        let eq194_e2438_q_d_n20: f64 = (eq194_e2436_q_d_n20 * p.p249);
        let eq194_e2438_q_d_n21: f64 = (eq194_e2436_q_d_n21 * p.p249);
        let eq194_e2438_q_d_n22: f64 = (eq194_e2436_q_d_n22 * p.p249);
        (eq194_e2438, eq194_e2438_d_n0, eq194_e2438_d_n1, eq194_e2438_d_n2, eq194_e2438_d_n3, eq194_e2438_d_n4, eq194_e2438_d_n5, eq194_e2438_d_n6, eq194_e2438_d_n7, eq194_e2438_d_n8, eq194_e2438_d_n9, eq194_e2438_d_n10, eq194_e2438_d_n11, eq194_e2438_d_n12, eq194_e2438_d_n13, eq194_e2438_d_n14, eq194_e2438_d_n15, eq194_e2438_d_n16, eq194_e2438_d_n17, eq194_e2438_d_n18, eq194_e2438_d_n19, eq194_e2438_d_n20, eq194_e2438_d_n21, eq194_e2438_d_n22, eq194_e2438_q, eq194_e2438_q_d_n0, eq194_e2438_q_d_n1, eq194_e2438_q_d_n2, eq194_e2438_q_d_n3, eq194_e2438_q_d_n4, eq194_e2438_q_d_n5, eq194_e2438_q_d_n6, eq194_e2438_q_d_n7, eq194_e2438_q_d_n8, eq194_e2438_q_d_n9, eq194_e2438_q_d_n10, eq194_e2438_q_d_n11, eq194_e2438_q_d_n12, eq194_e2438_q_d_n13, eq194_e2438_q_d_n14, eq194_e2438_q_d_n15, eq194_e2438_q_d_n16, eq194_e2438_q_d_n17, eq194_e2438_q_d_n18, eq194_e2438_q_d_n19, eq194_e2438_q_d_n20, eq194_e2438_q_d_n21, eq194_e2438_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq194_reactive_node_derivatives: [f64; 23] = [eq194_e2440_q_d_n0, eq194_e2440_q_d_n1, eq194_e2440_q_d_n2, eq194_e2440_q_d_n3, eq194_e2440_q_d_n4, eq194_e2440_q_d_n5, eq194_e2440_q_d_n6, eq194_e2440_q_d_n7, eq194_e2440_q_d_n8, eq194_e2440_q_d_n9, eq194_e2440_q_d_n10, eq194_e2440_q_d_n11, eq194_e2440_q_d_n12, eq194_e2440_q_d_n13, eq194_e2440_q_d_n14, eq194_e2440_q_d_n15, eq194_e2440_q_d_n16, eq194_e2440_q_d_n17, eq194_e2440_q_d_n18, eq194_e2440_q_d_n19, eq194_e2440_q_d_n20, eq194_e2440_q_d_n21, eq194_e2440_q_d_n22];
        let eq194_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            nodes,
            &eq194_reactive_node_derivatives,
            branches,
            &eq194_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq195_e2452, eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22, eq195_e2452_q, eq195_e2452_q_d_n0, eq195_e2452_q_d_n1, eq195_e2452_q_d_n2, eq195_e2452_q_d_n3, eq195_e2452_q_d_n4, eq195_e2452_q_d_n5, eq195_e2452_q_d_n6, eq195_e2452_q_d_n7, eq195_e2452_q_d_n8, eq195_e2452_q_d_n9, eq195_e2452_q_d_n10, eq195_e2452_q_d_n11, eq195_e2452_q_d_n12, eq195_e2452_q_d_n13, eq195_e2452_q_d_n14, eq195_e2452_q_d_n15, eq195_e2452_q_d_n16, eq195_e2452_q_d_n17, eq195_e2452_q_d_n18, eq195_e2452_q_d_n19, eq195_e2452_q_d_n20, eq195_e2452_q_d_n21, eq195_e2452_q_d_n22,) = {
    if ((s.b[600] && s.b[601]) && (!s.b[602])) {
        let eq195_e2449_q: f64 = s.v[300];
        let eq195_e2450: f64 = (p.p7 * s.v[300]);
        let eq195_e2450_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq195_e2450_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq195_e2450_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq195_e2450_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq195_e2450_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq195_e2450_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq195_e2450_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq195_e2450_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq195_e2450_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq195_e2450_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq195_e2450_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq195_e2450_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq195_e2450_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq195_e2450_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq195_e2450_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq195_e2450_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq195_e2450_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq195_e2450_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq195_e2450_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq195_e2450_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq195_e2450_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq195_e2450_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq195_e2450_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq195_e2450_q: f64 = (p.p7 * eq195_e2449_q);
        let eq195_e2450_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq195_e2450_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq195_e2450_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq195_e2450_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq195_e2450_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq195_e2450_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq195_e2450_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq195_e2450_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq195_e2450_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq195_e2450_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq195_e2450_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq195_e2450_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq195_e2450_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq195_e2450_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq195_e2450_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq195_e2450_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq195_e2450_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq195_e2450_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq195_e2450_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq195_e2450_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq195_e2450_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq195_e2450_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq195_e2450_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
        (eq195_e2450, eq195_e2450_d_n0, eq195_e2450_d_n1, eq195_e2450_d_n2, eq195_e2450_d_n3, eq195_e2450_d_n4, eq195_e2450_d_n5, eq195_e2450_d_n6, eq195_e2450_d_n7, eq195_e2450_d_n8, eq195_e2450_d_n9, eq195_e2450_d_n10, eq195_e2450_d_n11, eq195_e2450_d_n12, eq195_e2450_d_n13, eq195_e2450_d_n14, eq195_e2450_d_n15, eq195_e2450_d_n16, eq195_e2450_d_n17, eq195_e2450_d_n18, eq195_e2450_d_n19, eq195_e2450_d_n20, eq195_e2450_d_n21, eq195_e2450_d_n22, eq195_e2450_q, eq195_e2450_q_d_n0, eq195_e2450_q_d_n1, eq195_e2450_q_d_n2, eq195_e2450_q_d_n3, eq195_e2450_q_d_n4, eq195_e2450_q_d_n5, eq195_e2450_q_d_n6, eq195_e2450_q_d_n7, eq195_e2450_q_d_n8, eq195_e2450_q_d_n9, eq195_e2450_q_d_n10, eq195_e2450_q_d_n11, eq195_e2450_q_d_n12, eq195_e2450_q_d_n13, eq195_e2450_q_d_n14, eq195_e2450_q_d_n15, eq195_e2450_q_d_n16, eq195_e2450_q_d_n17, eq195_e2450_q_d_n18, eq195_e2450_q_d_n19, eq195_e2450_q_d_n20, eq195_e2450_q_d_n21, eq195_e2450_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_reactive_node_derivatives: [f64; 23] = [eq195_e2452_q_d_n0, eq195_e2452_q_d_n1, eq195_e2452_q_d_n2, eq195_e2452_q_d_n3, eq195_e2452_q_d_n4, eq195_e2452_q_d_n5, eq195_e2452_q_d_n6, eq195_e2452_q_d_n7, eq195_e2452_q_d_n8, eq195_e2452_q_d_n9, eq195_e2452_q_d_n10, eq195_e2452_q_d_n11, eq195_e2452_q_d_n12, eq195_e2452_q_d_n13, eq195_e2452_q_d_n14, eq195_e2452_q_d_n15, eq195_e2452_q_d_n16, eq195_e2452_q_d_n17, eq195_e2452_q_d_n18, eq195_e2452_q_d_n19, eq195_e2452_q_d_n20, eq195_e2452_q_d_n21, eq195_e2452_q_d_n22];
        let eq195_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            nodes,
            &eq195_reactive_node_derivatives,
            branches,
            &eq195_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq196_e2466, eq196_e2466_d_n0, eq196_e2466_d_n1, eq196_e2466_d_n2, eq196_e2466_d_n3, eq196_e2466_d_n4, eq196_e2466_d_n5, eq196_e2466_d_n6, eq196_e2466_d_n7, eq196_e2466_d_n8, eq196_e2466_d_n9, eq196_e2466_d_n10, eq196_e2466_d_n11, eq196_e2466_d_n12, eq196_e2466_d_n13, eq196_e2466_d_n14, eq196_e2466_d_n15, eq196_e2466_d_n16, eq196_e2466_d_n17, eq196_e2466_d_n18, eq196_e2466_d_n19, eq196_e2466_d_n20, eq196_e2466_d_n21, eq196_e2466_d_n22, eq196_e2466_q, eq196_e2466_q_d_n0, eq196_e2466_q_d_n1, eq196_e2466_q_d_n2, eq196_e2466_q_d_n3, eq196_e2466_q_d_n4, eq196_e2466_q_d_n5, eq196_e2466_q_d_n6, eq196_e2466_q_d_n7, eq196_e2466_q_d_n8, eq196_e2466_q_d_n9, eq196_e2466_q_d_n10, eq196_e2466_q_d_n11, eq196_e2466_q_d_n12, eq196_e2466_q_d_n13, eq196_e2466_q_d_n14, eq196_e2466_q_d_n15, eq196_e2466_q_d_n16, eq196_e2466_q_d_n17, eq196_e2466_q_d_n18, eq196_e2466_q_d_n19, eq196_e2466_q_d_n20, eq196_e2466_q_d_n21, eq196_e2466_q_d_n22,) = {
    if ((s.b[600] && s.b[601]) && (!s.b[602])) {
        let eq196_e2461_q: f64 = s.v[300];
        let eq196_e2462: f64 = (p.p7 * s.v[300]);
        let eq196_e2462_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq196_e2462_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq196_e2462_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq196_e2462_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq196_e2462_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq196_e2462_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq196_e2462_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq196_e2462_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq196_e2462_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq196_e2462_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq196_e2462_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq196_e2462_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq196_e2462_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq196_e2462_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq196_e2462_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq196_e2462_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq196_e2462_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq196_e2462_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq196_e2462_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq196_e2462_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq196_e2462_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq196_e2462_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq196_e2462_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq196_e2462_q: f64 = (p.p7 * eq196_e2461_q);
        let eq196_e2462_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq196_e2462_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq196_e2462_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq196_e2462_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq196_e2462_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq196_e2462_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq196_e2462_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq196_e2462_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq196_e2462_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq196_e2462_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq196_e2462_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq196_e2462_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq196_e2462_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq196_e2462_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq196_e2462_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq196_e2462_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq196_e2462_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq196_e2462_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq196_e2462_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq196_e2462_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq196_e2462_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq196_e2462_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq196_e2462_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
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
        let eq196_e2464_q: f64 = (eq196_e2462_q * p.p249);
        let eq196_e2464_q_d_n0: f64 = (eq196_e2462_q_d_n0 * p.p249);
        let eq196_e2464_q_d_n1: f64 = (eq196_e2462_q_d_n1 * p.p249);
        let eq196_e2464_q_d_n2: f64 = (eq196_e2462_q_d_n2 * p.p249);
        let eq196_e2464_q_d_n3: f64 = (eq196_e2462_q_d_n3 * p.p249);
        let eq196_e2464_q_d_n4: f64 = (eq196_e2462_q_d_n4 * p.p249);
        let eq196_e2464_q_d_n5: f64 = (eq196_e2462_q_d_n5 * p.p249);
        let eq196_e2464_q_d_n6: f64 = (eq196_e2462_q_d_n6 * p.p249);
        let eq196_e2464_q_d_n7: f64 = (eq196_e2462_q_d_n7 * p.p249);
        let eq196_e2464_q_d_n8: f64 = (eq196_e2462_q_d_n8 * p.p249);
        let eq196_e2464_q_d_n9: f64 = (eq196_e2462_q_d_n9 * p.p249);
        let eq196_e2464_q_d_n10: f64 = (eq196_e2462_q_d_n10 * p.p249);
        let eq196_e2464_q_d_n11: f64 = (eq196_e2462_q_d_n11 * p.p249);
        let eq196_e2464_q_d_n12: f64 = (eq196_e2462_q_d_n12 * p.p249);
        let eq196_e2464_q_d_n13: f64 = (eq196_e2462_q_d_n13 * p.p249);
        let eq196_e2464_q_d_n14: f64 = (eq196_e2462_q_d_n14 * p.p249);
        let eq196_e2464_q_d_n15: f64 = (eq196_e2462_q_d_n15 * p.p249);
        let eq196_e2464_q_d_n16: f64 = (eq196_e2462_q_d_n16 * p.p249);
        let eq196_e2464_q_d_n17: f64 = (eq196_e2462_q_d_n17 * p.p249);
        let eq196_e2464_q_d_n18: f64 = (eq196_e2462_q_d_n18 * p.p249);
        let eq196_e2464_q_d_n19: f64 = (eq196_e2462_q_d_n19 * p.p249);
        let eq196_e2464_q_d_n20: f64 = (eq196_e2462_q_d_n20 * p.p249);
        let eq196_e2464_q_d_n21: f64 = (eq196_e2462_q_d_n21 * p.p249);
        let eq196_e2464_q_d_n22: f64 = (eq196_e2462_q_d_n22 * p.p249);
        (eq196_e2464, eq196_e2464_d_n0, eq196_e2464_d_n1, eq196_e2464_d_n2, eq196_e2464_d_n3, eq196_e2464_d_n4, eq196_e2464_d_n5, eq196_e2464_d_n6, eq196_e2464_d_n7, eq196_e2464_d_n8, eq196_e2464_d_n9, eq196_e2464_d_n10, eq196_e2464_d_n11, eq196_e2464_d_n12, eq196_e2464_d_n13, eq196_e2464_d_n14, eq196_e2464_d_n15, eq196_e2464_d_n16, eq196_e2464_d_n17, eq196_e2464_d_n18, eq196_e2464_d_n19, eq196_e2464_d_n20, eq196_e2464_d_n21, eq196_e2464_d_n22, eq196_e2464_q, eq196_e2464_q_d_n0, eq196_e2464_q_d_n1, eq196_e2464_q_d_n2, eq196_e2464_q_d_n3, eq196_e2464_q_d_n4, eq196_e2464_q_d_n5, eq196_e2464_q_d_n6, eq196_e2464_q_d_n7, eq196_e2464_q_d_n8, eq196_e2464_q_d_n9, eq196_e2464_q_d_n10, eq196_e2464_q_d_n11, eq196_e2464_q_d_n12, eq196_e2464_q_d_n13, eq196_e2464_q_d_n14, eq196_e2464_q_d_n15, eq196_e2464_q_d_n16, eq196_e2464_q_d_n17, eq196_e2464_q_d_n18, eq196_e2464_q_d_n19, eq196_e2464_q_d_n20, eq196_e2464_q_d_n21, eq196_e2464_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq196_reactive_node_derivatives: [f64; 23] = [eq196_e2466_q_d_n0, eq196_e2466_q_d_n1, eq196_e2466_q_d_n2, eq196_e2466_q_d_n3, eq196_e2466_q_d_n4, eq196_e2466_q_d_n5, eq196_e2466_q_d_n6, eq196_e2466_q_d_n7, eq196_e2466_q_d_n8, eq196_e2466_q_d_n9, eq196_e2466_q_d_n10, eq196_e2466_q_d_n11, eq196_e2466_q_d_n12, eq196_e2466_q_d_n13, eq196_e2466_q_d_n14, eq196_e2466_q_d_n15, eq196_e2466_q_d_n16, eq196_e2466_q_d_n17, eq196_e2466_q_d_n18, eq196_e2466_q_d_n19, eq196_e2466_q_d_n20, eq196_e2466_q_d_n21, eq196_e2466_q_d_n22];
        let eq196_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            nodes,
            &eq196_reactive_node_derivatives,
            branches,
            &eq196_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq197_e2477, eq197_e2477_d_n0, eq197_e2477_d_n1, eq197_e2477_d_n2, eq197_e2477_d_n3, eq197_e2477_d_n4, eq197_e2477_d_n5, eq197_e2477_d_n6, eq197_e2477_d_n7, eq197_e2477_d_n8, eq197_e2477_d_n9, eq197_e2477_d_n10, eq197_e2477_d_n11, eq197_e2477_d_n12, eq197_e2477_d_n13, eq197_e2477_d_n14, eq197_e2477_d_n15, eq197_e2477_d_n16, eq197_e2477_d_n17, eq197_e2477_d_n18, eq197_e2477_d_n19, eq197_e2477_d_n20, eq197_e2477_d_n21, eq197_e2477_d_n22, eq197_e2477_q, eq197_e2477_q_d_n0, eq197_e2477_q_d_n1, eq197_e2477_q_d_n2, eq197_e2477_q_d_n3, eq197_e2477_q_d_n4, eq197_e2477_q_d_n5, eq197_e2477_q_d_n6, eq197_e2477_q_d_n7, eq197_e2477_q_d_n8, eq197_e2477_q_d_n9, eq197_e2477_q_d_n10, eq197_e2477_q_d_n11, eq197_e2477_q_d_n12, eq197_e2477_q_d_n13, eq197_e2477_q_d_n14, eq197_e2477_q_d_n15, eq197_e2477_q_d_n16, eq197_e2477_q_d_n17, eq197_e2477_q_d_n18, eq197_e2477_q_d_n19, eq197_e2477_q_d_n20, eq197_e2477_q_d_n21, eq197_e2477_q_d_n22,) = {
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
        let eq197_e2475_d_n10: f64 = (p.p7 * eq197_e2473_d_n10);
        let eq197_e2475_d_n11: f64 = (p.p7 * eq197_e2473_d_n11);
        let eq197_e2475_d_n12: f64 = (p.p7 * eq197_e2473_d_n12);
        let eq197_e2475_d_n13: f64 = (p.p7 * eq197_e2473_d_n13);
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
        let eq197_e2475_q_d_n0: f64 = (p.p7 * eq197_e2473_d_n0);
        let eq197_e2475_q_d_n1: f64 = (p.p7 * eq197_e2473_d_n1);
        let eq197_e2475_q_d_n2: f64 = (p.p7 * eq197_e2473_d_n2);
        let eq197_e2475_q_d_n3: f64 = (p.p7 * eq197_e2473_d_n3);
        let eq197_e2475_q_d_n4: f64 = (p.p7 * eq197_e2473_d_n4);
        let eq197_e2475_q_d_n5: f64 = (p.p7 * eq197_e2473_d_n5);
        let eq197_e2475_q_d_n6: f64 = (p.p7 * eq197_e2473_d_n6);
        let eq197_e2475_q_d_n7: f64 = (p.p7 * eq197_e2473_d_n7);
        let eq197_e2475_q_d_n8: f64 = (p.p7 * eq197_e2473_d_n8);
        let eq197_e2475_q_d_n9: f64 = (p.p7 * eq197_e2473_d_n9);
        let eq197_e2475_q_d_n10: f64 = (p.p7 * eq197_e2473_d_n10);
        let eq197_e2475_q_d_n11: f64 = (p.p7 * eq197_e2473_d_n11);
        let eq197_e2475_q_d_n12: f64 = (p.p7 * eq197_e2473_d_n12);
        let eq197_e2475_q_d_n13: f64 = (p.p7 * eq197_e2473_d_n13);
        let eq197_e2475_q_d_n14: f64 = (p.p7 * eq197_e2473_d_n14);
        let eq197_e2475_q_d_n15: f64 = (p.p7 * eq197_e2473_d_n15);
        let eq197_e2475_q_d_n16: f64 = (p.p7 * eq197_e2473_d_n16);
        let eq197_e2475_q_d_n17: f64 = (p.p7 * eq197_e2473_d_n17);
        let eq197_e2475_q_d_n18: f64 = (p.p7 * eq197_e2473_d_n18);
        let eq197_e2475_q_d_n19: f64 = (p.p7 * eq197_e2473_d_n19);
        let eq197_e2475_q_d_n20: f64 = (p.p7 * eq197_e2473_d_n20);
        let eq197_e2475_q_d_n21: f64 = (p.p7 * eq197_e2473_d_n21);
        let eq197_e2475_q_d_n22: f64 = (p.p7 * eq197_e2473_d_n22);
        (eq197_e2475, eq197_e2475_d_n0, eq197_e2475_d_n1, eq197_e2475_d_n2, eq197_e2475_d_n3, eq197_e2475_d_n4, eq197_e2475_d_n5, eq197_e2475_d_n6, eq197_e2475_d_n7, eq197_e2475_d_n8, eq197_e2475_d_n9, eq197_e2475_d_n10, eq197_e2475_d_n11, eq197_e2475_d_n12, eq197_e2475_d_n13, eq197_e2475_d_n14, eq197_e2475_d_n15, eq197_e2475_d_n16, eq197_e2475_d_n17, eq197_e2475_d_n18, eq197_e2475_d_n19, eq197_e2475_d_n20, eq197_e2475_d_n21, eq197_e2475_d_n22, eq197_e2475_q, eq197_e2475_q_d_n0, eq197_e2475_q_d_n1, eq197_e2475_q_d_n2, eq197_e2475_q_d_n3, eq197_e2475_q_d_n4, eq197_e2475_q_d_n5, eq197_e2475_q_d_n6, eq197_e2475_q_d_n7, eq197_e2475_q_d_n8, eq197_e2475_q_d_n9, eq197_e2475_q_d_n10, eq197_e2475_q_d_n11, eq197_e2475_q_d_n12, eq197_e2475_q_d_n13, eq197_e2475_q_d_n14, eq197_e2475_q_d_n15, eq197_e2475_q_d_n16, eq197_e2475_q_d_n17, eq197_e2475_q_d_n18, eq197_e2475_q_d_n19, eq197_e2475_q_d_n20, eq197_e2475_q_d_n21, eq197_e2475_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq197_reactive_node_derivatives: [f64; 23] = [eq197_e2477_q_d_n0, eq197_e2477_q_d_n1, eq197_e2477_q_d_n2, eq197_e2477_q_d_n3, eq197_e2477_q_d_n4, eq197_e2477_q_d_n5, eq197_e2477_q_d_n6, eq197_e2477_q_d_n7, eq197_e2477_q_d_n8, eq197_e2477_q_d_n9, eq197_e2477_q_d_n10, eq197_e2477_q_d_n11, eq197_e2477_q_d_n12, eq197_e2477_q_d_n13, eq197_e2477_q_d_n14, eq197_e2477_q_d_n15, eq197_e2477_q_d_n16, eq197_e2477_q_d_n17, eq197_e2477_q_d_n18, eq197_e2477_q_d_n19, eq197_e2477_q_d_n20, eq197_e2477_q_d_n21, eq197_e2477_q_d_n22];
        let eq197_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[17]),
            nodes,
            &eq197_reactive_node_derivatives,
            branches,
            &eq197_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_17(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq198_e2487, eq198_e2487_d_n0, eq198_e2487_d_n1, eq198_e2487_d_n2, eq198_e2487_d_n3, eq198_e2487_d_n4, eq198_e2487_d_n5, eq198_e2487_d_n6, eq198_e2487_d_n7, eq198_e2487_d_n8, eq198_e2487_d_n9, eq198_e2487_d_n10, eq198_e2487_d_n11, eq198_e2487_d_n12, eq198_e2487_d_n13, eq198_e2487_d_n14, eq198_e2487_d_n15, eq198_e2487_d_n16, eq198_e2487_d_n17, eq198_e2487_d_n18, eq198_e2487_d_n19, eq198_e2487_d_n20, eq198_e2487_d_n21, eq198_e2487_d_n22, eq198_e2487_q, eq198_e2487_q_d_n0, eq198_e2487_q_d_n1, eq198_e2487_q_d_n2, eq198_e2487_q_d_n3, eq198_e2487_q_d_n4, eq198_e2487_q_d_n5, eq198_e2487_q_d_n6, eq198_e2487_q_d_n7, eq198_e2487_q_d_n8, eq198_e2487_q_d_n9, eq198_e2487_q_d_n10, eq198_e2487_q_d_n11, eq198_e2487_q_d_n12, eq198_e2487_q_d_n13, eq198_e2487_q_d_n14, eq198_e2487_q_d_n15, eq198_e2487_q_d_n16, eq198_e2487_q_d_n17, eq198_e2487_q_d_n18, eq198_e2487_q_d_n19, eq198_e2487_q_d_n20, eq198_e2487_q_d_n21, eq198_e2487_q_d_n22,) = {
    if ((!s.b[600]) && s.b[603]) {
        let eq198_e2484_q: f64 = s.v[301];
        let eq198_e2485: f64 = (p.p7 * s.v[301]);
        let eq198_e2485_d_n0: f64 = (p.p7 * s.dn[301][0]);
        let eq198_e2485_d_n1: f64 = (p.p7 * s.dn[301][1]);
        let eq198_e2485_d_n2: f64 = (p.p7 * s.dn[301][2]);
        let eq198_e2485_d_n3: f64 = (p.p7 * s.dn[301][3]);
        let eq198_e2485_d_n4: f64 = (p.p7 * s.dn[301][4]);
        let eq198_e2485_d_n5: f64 = (p.p7 * s.dn[301][5]);
        let eq198_e2485_d_n6: f64 = (p.p7 * s.dn[301][6]);
        let eq198_e2485_d_n7: f64 = (p.p7 * s.dn[301][7]);
        let eq198_e2485_d_n8: f64 = (p.p7 * s.dn[301][8]);
        let eq198_e2485_d_n9: f64 = (p.p7 * s.dn[301][9]);
        let eq198_e2485_d_n10: f64 = (p.p7 * s.dn[301][10]);
        let eq198_e2485_d_n11: f64 = (p.p7 * s.dn[301][11]);
        let eq198_e2485_d_n12: f64 = (p.p7 * s.dn[301][12]);
        let eq198_e2485_d_n13: f64 = (p.p7 * s.dn[301][13]);
        let eq198_e2485_d_n14: f64 = (p.p7 * s.dn[301][14]);
        let eq198_e2485_d_n15: f64 = (p.p7 * s.dn[301][15]);
        let eq198_e2485_d_n16: f64 = (p.p7 * s.dn[301][16]);
        let eq198_e2485_d_n17: f64 = (p.p7 * s.dn[301][17]);
        let eq198_e2485_d_n18: f64 = (p.p7 * s.dn[301][18]);
        let eq198_e2485_d_n19: f64 = (p.p7 * s.dn[301][19]);
        let eq198_e2485_d_n20: f64 = (p.p7 * s.dn[301][20]);
        let eq198_e2485_d_n21: f64 = (p.p7 * s.dn[301][21]);
        let eq198_e2485_d_n22: f64 = (p.p7 * s.dn[301][22]);
        let eq198_e2485_q: f64 = (p.p7 * eq198_e2484_q);
        let eq198_e2485_q_d_n0: f64 = (p.p7 * s.dn[301][0]);
        let eq198_e2485_q_d_n1: f64 = (p.p7 * s.dn[301][1]);
        let eq198_e2485_q_d_n2: f64 = (p.p7 * s.dn[301][2]);
        let eq198_e2485_q_d_n3: f64 = (p.p7 * s.dn[301][3]);
        let eq198_e2485_q_d_n4: f64 = (p.p7 * s.dn[301][4]);
        let eq198_e2485_q_d_n5: f64 = (p.p7 * s.dn[301][5]);
        let eq198_e2485_q_d_n6: f64 = (p.p7 * s.dn[301][6]);
        let eq198_e2485_q_d_n7: f64 = (p.p7 * s.dn[301][7]);
        let eq198_e2485_q_d_n8: f64 = (p.p7 * s.dn[301][8]);
        let eq198_e2485_q_d_n9: f64 = (p.p7 * s.dn[301][9]);
        let eq198_e2485_q_d_n10: f64 = (p.p7 * s.dn[301][10]);
        let eq198_e2485_q_d_n11: f64 = (p.p7 * s.dn[301][11]);
        let eq198_e2485_q_d_n12: f64 = (p.p7 * s.dn[301][12]);
        let eq198_e2485_q_d_n13: f64 = (p.p7 * s.dn[301][13]);
        let eq198_e2485_q_d_n14: f64 = (p.p7 * s.dn[301][14]);
        let eq198_e2485_q_d_n15: f64 = (p.p7 * s.dn[301][15]);
        let eq198_e2485_q_d_n16: f64 = (p.p7 * s.dn[301][16]);
        let eq198_e2485_q_d_n17: f64 = (p.p7 * s.dn[301][17]);
        let eq198_e2485_q_d_n18: f64 = (p.p7 * s.dn[301][18]);
        let eq198_e2485_q_d_n19: f64 = (p.p7 * s.dn[301][19]);
        let eq198_e2485_q_d_n20: f64 = (p.p7 * s.dn[301][20]);
        let eq198_e2485_q_d_n21: f64 = (p.p7 * s.dn[301][21]);
        let eq198_e2485_q_d_n22: f64 = (p.p7 * s.dn[301][22]);
        (eq198_e2485, eq198_e2485_d_n0, eq198_e2485_d_n1, eq198_e2485_d_n2, eq198_e2485_d_n3, eq198_e2485_d_n4, eq198_e2485_d_n5, eq198_e2485_d_n6, eq198_e2485_d_n7, eq198_e2485_d_n8, eq198_e2485_d_n9, eq198_e2485_d_n10, eq198_e2485_d_n11, eq198_e2485_d_n12, eq198_e2485_d_n13, eq198_e2485_d_n14, eq198_e2485_d_n15, eq198_e2485_d_n16, eq198_e2485_d_n17, eq198_e2485_d_n18, eq198_e2485_d_n19, eq198_e2485_d_n20, eq198_e2485_d_n21, eq198_e2485_d_n22, eq198_e2485_q, eq198_e2485_q_d_n0, eq198_e2485_q_d_n1, eq198_e2485_q_d_n2, eq198_e2485_q_d_n3, eq198_e2485_q_d_n4, eq198_e2485_q_d_n5, eq198_e2485_q_d_n6, eq198_e2485_q_d_n7, eq198_e2485_q_d_n8, eq198_e2485_q_d_n9, eq198_e2485_q_d_n10, eq198_e2485_q_d_n11, eq198_e2485_q_d_n12, eq198_e2485_q_d_n13, eq198_e2485_q_d_n14, eq198_e2485_q_d_n15, eq198_e2485_q_d_n16, eq198_e2485_q_d_n17, eq198_e2485_q_d_n18, eq198_e2485_q_d_n19, eq198_e2485_q_d_n20, eq198_e2485_q_d_n21, eq198_e2485_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq198_reactive_node_derivatives: [f64; 23] = [eq198_e2487_q_d_n0, eq198_e2487_q_d_n1, eq198_e2487_q_d_n2, eq198_e2487_q_d_n3, eq198_e2487_q_d_n4, eq198_e2487_q_d_n5, eq198_e2487_q_d_n6, eq198_e2487_q_d_n7, eq198_e2487_q_d_n8, eq198_e2487_q_d_n9, eq198_e2487_q_d_n10, eq198_e2487_q_d_n11, eq198_e2487_q_d_n12, eq198_e2487_q_d_n13, eq198_e2487_q_d_n14, eq198_e2487_q_d_n15, eq198_e2487_q_d_n16, eq198_e2487_q_d_n17, eq198_e2487_q_d_n18, eq198_e2487_q_d_n19, eq198_e2487_q_d_n20, eq198_e2487_q_d_n21, eq198_e2487_q_d_n22];
        let eq198_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq198_reactive_node_derivatives,
            branches,
            &eq198_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq199_e2499, eq199_e2499_d_n0, eq199_e2499_d_n1, eq199_e2499_d_n2, eq199_e2499_d_n3, eq199_e2499_d_n4, eq199_e2499_d_n5, eq199_e2499_d_n6, eq199_e2499_d_n7, eq199_e2499_d_n8, eq199_e2499_d_n9, eq199_e2499_d_n10, eq199_e2499_d_n11, eq199_e2499_d_n12, eq199_e2499_d_n13, eq199_e2499_d_n14, eq199_e2499_d_n15, eq199_e2499_d_n16, eq199_e2499_d_n17, eq199_e2499_d_n18, eq199_e2499_d_n19, eq199_e2499_d_n20, eq199_e2499_d_n21, eq199_e2499_d_n22, eq199_e2499_q, eq199_e2499_q_d_n0, eq199_e2499_q_d_n1, eq199_e2499_q_d_n2, eq199_e2499_q_d_n3, eq199_e2499_q_d_n4, eq199_e2499_q_d_n5, eq199_e2499_q_d_n6, eq199_e2499_q_d_n7, eq199_e2499_q_d_n8, eq199_e2499_q_d_n9, eq199_e2499_q_d_n10, eq199_e2499_q_d_n11, eq199_e2499_q_d_n12, eq199_e2499_q_d_n13, eq199_e2499_q_d_n14, eq199_e2499_q_d_n15, eq199_e2499_q_d_n16, eq199_e2499_q_d_n17, eq199_e2499_q_d_n18, eq199_e2499_q_d_n19, eq199_e2499_q_d_n20, eq199_e2499_q_d_n21, eq199_e2499_q_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && s.b[604]) {
        let eq199_e2496_q: f64 = s.v[300];
        let eq199_e2497: f64 = (p.p7 * s.v[300]);
        let eq199_e2497_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq199_e2497_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq199_e2497_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq199_e2497_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq199_e2497_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq199_e2497_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq199_e2497_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq199_e2497_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq199_e2497_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq199_e2497_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq199_e2497_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq199_e2497_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq199_e2497_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq199_e2497_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq199_e2497_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq199_e2497_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq199_e2497_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq199_e2497_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq199_e2497_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq199_e2497_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq199_e2497_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq199_e2497_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq199_e2497_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq199_e2497_q: f64 = (p.p7 * eq199_e2496_q);
        let eq199_e2497_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq199_e2497_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq199_e2497_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq199_e2497_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq199_e2497_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq199_e2497_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq199_e2497_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq199_e2497_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq199_e2497_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq199_e2497_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq199_e2497_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq199_e2497_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq199_e2497_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq199_e2497_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq199_e2497_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq199_e2497_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq199_e2497_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq199_e2497_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq199_e2497_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq199_e2497_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq199_e2497_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq199_e2497_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq199_e2497_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
        (eq199_e2497, eq199_e2497_d_n0, eq199_e2497_d_n1, eq199_e2497_d_n2, eq199_e2497_d_n3, eq199_e2497_d_n4, eq199_e2497_d_n5, eq199_e2497_d_n6, eq199_e2497_d_n7, eq199_e2497_d_n8, eq199_e2497_d_n9, eq199_e2497_d_n10, eq199_e2497_d_n11, eq199_e2497_d_n12, eq199_e2497_d_n13, eq199_e2497_d_n14, eq199_e2497_d_n15, eq199_e2497_d_n16, eq199_e2497_d_n17, eq199_e2497_d_n18, eq199_e2497_d_n19, eq199_e2497_d_n20, eq199_e2497_d_n21, eq199_e2497_d_n22, eq199_e2497_q, eq199_e2497_q_d_n0, eq199_e2497_q_d_n1, eq199_e2497_q_d_n2, eq199_e2497_q_d_n3, eq199_e2497_q_d_n4, eq199_e2497_q_d_n5, eq199_e2497_q_d_n6, eq199_e2497_q_d_n7, eq199_e2497_q_d_n8, eq199_e2497_q_d_n9, eq199_e2497_q_d_n10, eq199_e2497_q_d_n11, eq199_e2497_q_d_n12, eq199_e2497_q_d_n13, eq199_e2497_q_d_n14, eq199_e2497_q_d_n15, eq199_e2497_q_d_n16, eq199_e2497_q_d_n17, eq199_e2497_q_d_n18, eq199_e2497_q_d_n19, eq199_e2497_q_d_n20, eq199_e2497_q_d_n21, eq199_e2497_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq199_reactive_node_derivatives: [f64; 23] = [eq199_e2499_q_d_n0, eq199_e2499_q_d_n1, eq199_e2499_q_d_n2, eq199_e2499_q_d_n3, eq199_e2499_q_d_n4, eq199_e2499_q_d_n5, eq199_e2499_q_d_n6, eq199_e2499_q_d_n7, eq199_e2499_q_d_n8, eq199_e2499_q_d_n9, eq199_e2499_q_d_n10, eq199_e2499_q_d_n11, eq199_e2499_q_d_n12, eq199_e2499_q_d_n13, eq199_e2499_q_d_n14, eq199_e2499_q_d_n15, eq199_e2499_q_d_n16, eq199_e2499_q_d_n17, eq199_e2499_q_d_n18, eq199_e2499_q_d_n19, eq199_e2499_q_d_n20, eq199_e2499_q_d_n21, eq199_e2499_q_d_n22];
        let eq199_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq199_reactive_node_derivatives,
            branches,
            &eq199_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq200_e2513, eq200_e2513_d_n0, eq200_e2513_d_n1, eq200_e2513_d_n2, eq200_e2513_d_n3, eq200_e2513_d_n4, eq200_e2513_d_n5, eq200_e2513_d_n6, eq200_e2513_d_n7, eq200_e2513_d_n8, eq200_e2513_d_n9, eq200_e2513_d_n10, eq200_e2513_d_n11, eq200_e2513_d_n12, eq200_e2513_d_n13, eq200_e2513_d_n14, eq200_e2513_d_n15, eq200_e2513_d_n16, eq200_e2513_d_n17, eq200_e2513_d_n18, eq200_e2513_d_n19, eq200_e2513_d_n20, eq200_e2513_d_n21, eq200_e2513_d_n22, eq200_e2513_q, eq200_e2513_q_d_n0, eq200_e2513_q_d_n1, eq200_e2513_q_d_n2, eq200_e2513_q_d_n3, eq200_e2513_q_d_n4, eq200_e2513_q_d_n5, eq200_e2513_q_d_n6, eq200_e2513_q_d_n7, eq200_e2513_q_d_n8, eq200_e2513_q_d_n9, eq200_e2513_q_d_n10, eq200_e2513_q_d_n11, eq200_e2513_q_d_n12, eq200_e2513_q_d_n13, eq200_e2513_q_d_n14, eq200_e2513_q_d_n15, eq200_e2513_q_d_n16, eq200_e2513_q_d_n17, eq200_e2513_q_d_n18, eq200_e2513_q_d_n19, eq200_e2513_q_d_n20, eq200_e2513_q_d_n21, eq200_e2513_q_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && s.b[604]) {
        let eq200_e2508_q: f64 = s.v[300];
        let eq200_e2509: f64 = (p.p7 * s.v[300]);
        let eq200_e2509_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq200_e2509_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq200_e2509_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq200_e2509_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq200_e2509_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq200_e2509_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq200_e2509_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq200_e2509_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq200_e2509_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq200_e2509_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq200_e2509_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq200_e2509_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq200_e2509_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq200_e2509_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq200_e2509_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq200_e2509_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq200_e2509_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq200_e2509_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq200_e2509_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq200_e2509_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq200_e2509_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq200_e2509_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq200_e2509_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq200_e2509_q: f64 = (p.p7 * eq200_e2508_q);
        let eq200_e2509_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq200_e2509_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq200_e2509_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq200_e2509_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq200_e2509_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq200_e2509_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq200_e2509_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq200_e2509_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq200_e2509_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq200_e2509_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq200_e2509_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq200_e2509_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq200_e2509_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq200_e2509_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq200_e2509_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq200_e2509_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq200_e2509_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq200_e2509_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq200_e2509_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq200_e2509_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq200_e2509_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq200_e2509_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq200_e2509_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
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
        let eq200_e2511_q: f64 = (eq200_e2509_q * p.p249);
        let eq200_e2511_q_d_n0: f64 = (eq200_e2509_q_d_n0 * p.p249);
        let eq200_e2511_q_d_n1: f64 = (eq200_e2509_q_d_n1 * p.p249);
        let eq200_e2511_q_d_n2: f64 = (eq200_e2509_q_d_n2 * p.p249);
        let eq200_e2511_q_d_n3: f64 = (eq200_e2509_q_d_n3 * p.p249);
        let eq200_e2511_q_d_n4: f64 = (eq200_e2509_q_d_n4 * p.p249);
        let eq200_e2511_q_d_n5: f64 = (eq200_e2509_q_d_n5 * p.p249);
        let eq200_e2511_q_d_n6: f64 = (eq200_e2509_q_d_n6 * p.p249);
        let eq200_e2511_q_d_n7: f64 = (eq200_e2509_q_d_n7 * p.p249);
        let eq200_e2511_q_d_n8: f64 = (eq200_e2509_q_d_n8 * p.p249);
        let eq200_e2511_q_d_n9: f64 = (eq200_e2509_q_d_n9 * p.p249);
        let eq200_e2511_q_d_n10: f64 = (eq200_e2509_q_d_n10 * p.p249);
        let eq200_e2511_q_d_n11: f64 = (eq200_e2509_q_d_n11 * p.p249);
        let eq200_e2511_q_d_n12: f64 = (eq200_e2509_q_d_n12 * p.p249);
        let eq200_e2511_q_d_n13: f64 = (eq200_e2509_q_d_n13 * p.p249);
        let eq200_e2511_q_d_n14: f64 = (eq200_e2509_q_d_n14 * p.p249);
        let eq200_e2511_q_d_n15: f64 = (eq200_e2509_q_d_n15 * p.p249);
        let eq200_e2511_q_d_n16: f64 = (eq200_e2509_q_d_n16 * p.p249);
        let eq200_e2511_q_d_n17: f64 = (eq200_e2509_q_d_n17 * p.p249);
        let eq200_e2511_q_d_n18: f64 = (eq200_e2509_q_d_n18 * p.p249);
        let eq200_e2511_q_d_n19: f64 = (eq200_e2509_q_d_n19 * p.p249);
        let eq200_e2511_q_d_n20: f64 = (eq200_e2509_q_d_n20 * p.p249);
        let eq200_e2511_q_d_n21: f64 = (eq200_e2509_q_d_n21 * p.p249);
        let eq200_e2511_q_d_n22: f64 = (eq200_e2509_q_d_n22 * p.p249);
        (eq200_e2511, eq200_e2511_d_n0, eq200_e2511_d_n1, eq200_e2511_d_n2, eq200_e2511_d_n3, eq200_e2511_d_n4, eq200_e2511_d_n5, eq200_e2511_d_n6, eq200_e2511_d_n7, eq200_e2511_d_n8, eq200_e2511_d_n9, eq200_e2511_d_n10, eq200_e2511_d_n11, eq200_e2511_d_n12, eq200_e2511_d_n13, eq200_e2511_d_n14, eq200_e2511_d_n15, eq200_e2511_d_n16, eq200_e2511_d_n17, eq200_e2511_d_n18, eq200_e2511_d_n19, eq200_e2511_d_n20, eq200_e2511_d_n21, eq200_e2511_d_n22, eq200_e2511_q, eq200_e2511_q_d_n0, eq200_e2511_q_d_n1, eq200_e2511_q_d_n2, eq200_e2511_q_d_n3, eq200_e2511_q_d_n4, eq200_e2511_q_d_n5, eq200_e2511_q_d_n6, eq200_e2511_q_d_n7, eq200_e2511_q_d_n8, eq200_e2511_q_d_n9, eq200_e2511_q_d_n10, eq200_e2511_q_d_n11, eq200_e2511_q_d_n12, eq200_e2511_q_d_n13, eq200_e2511_q_d_n14, eq200_e2511_q_d_n15, eq200_e2511_q_d_n16, eq200_e2511_q_d_n17, eq200_e2511_q_d_n18, eq200_e2511_q_d_n19, eq200_e2511_q_d_n20, eq200_e2511_q_d_n21, eq200_e2511_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq200_reactive_node_derivatives: [f64; 23] = [eq200_e2513_q_d_n0, eq200_e2513_q_d_n1, eq200_e2513_q_d_n2, eq200_e2513_q_d_n3, eq200_e2513_q_d_n4, eq200_e2513_q_d_n5, eq200_e2513_q_d_n6, eq200_e2513_q_d_n7, eq200_e2513_q_d_n8, eq200_e2513_q_d_n9, eq200_e2513_q_d_n10, eq200_e2513_q_d_n11, eq200_e2513_q_d_n12, eq200_e2513_q_d_n13, eq200_e2513_q_d_n14, eq200_e2513_q_d_n15, eq200_e2513_q_d_n16, eq200_e2513_q_d_n17, eq200_e2513_q_d_n18, eq200_e2513_q_d_n19, eq200_e2513_q_d_n20, eq200_e2513_q_d_n21, eq200_e2513_q_d_n22];
        let eq200_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq200_reactive_node_derivatives,
            branches,
            &eq200_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq201_e2526, eq201_e2526_d_n0, eq201_e2526_d_n1, eq201_e2526_d_n2, eq201_e2526_d_n3, eq201_e2526_d_n4, eq201_e2526_d_n5, eq201_e2526_d_n6, eq201_e2526_d_n7, eq201_e2526_d_n8, eq201_e2526_d_n9, eq201_e2526_d_n10, eq201_e2526_d_n11, eq201_e2526_d_n12, eq201_e2526_d_n13, eq201_e2526_d_n14, eq201_e2526_d_n15, eq201_e2526_d_n16, eq201_e2526_d_n17, eq201_e2526_d_n18, eq201_e2526_d_n19, eq201_e2526_d_n20, eq201_e2526_d_n21, eq201_e2526_d_n22, eq201_e2526_q, eq201_e2526_q_d_n0, eq201_e2526_q_d_n1, eq201_e2526_q_d_n2, eq201_e2526_q_d_n3, eq201_e2526_q_d_n4, eq201_e2526_q_d_n5, eq201_e2526_q_d_n6, eq201_e2526_q_d_n7, eq201_e2526_q_d_n8, eq201_e2526_q_d_n9, eq201_e2526_q_d_n10, eq201_e2526_q_d_n11, eq201_e2526_q_d_n12, eq201_e2526_q_d_n13, eq201_e2526_q_d_n14, eq201_e2526_q_d_n15, eq201_e2526_q_d_n16, eq201_e2526_q_d_n17, eq201_e2526_q_d_n18, eq201_e2526_q_d_n19, eq201_e2526_q_d_n20, eq201_e2526_q_d_n21, eq201_e2526_q_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && (!s.b[604])) {
        let eq201_e2523_q: f64 = s.v[300];
        let eq201_e2524: f64 = (p.p7 * s.v[300]);
        let eq201_e2524_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq201_e2524_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq201_e2524_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq201_e2524_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq201_e2524_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq201_e2524_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq201_e2524_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq201_e2524_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq201_e2524_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq201_e2524_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq201_e2524_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq201_e2524_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq201_e2524_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq201_e2524_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq201_e2524_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq201_e2524_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq201_e2524_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq201_e2524_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq201_e2524_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq201_e2524_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq201_e2524_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq201_e2524_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq201_e2524_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq201_e2524_q: f64 = (p.p7 * eq201_e2523_q);
        let eq201_e2524_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq201_e2524_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq201_e2524_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq201_e2524_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq201_e2524_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq201_e2524_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq201_e2524_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq201_e2524_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq201_e2524_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq201_e2524_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq201_e2524_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq201_e2524_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq201_e2524_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq201_e2524_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq201_e2524_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq201_e2524_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq201_e2524_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq201_e2524_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq201_e2524_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq201_e2524_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq201_e2524_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq201_e2524_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq201_e2524_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
        (eq201_e2524, eq201_e2524_d_n0, eq201_e2524_d_n1, eq201_e2524_d_n2, eq201_e2524_d_n3, eq201_e2524_d_n4, eq201_e2524_d_n5, eq201_e2524_d_n6, eq201_e2524_d_n7, eq201_e2524_d_n8, eq201_e2524_d_n9, eq201_e2524_d_n10, eq201_e2524_d_n11, eq201_e2524_d_n12, eq201_e2524_d_n13, eq201_e2524_d_n14, eq201_e2524_d_n15, eq201_e2524_d_n16, eq201_e2524_d_n17, eq201_e2524_d_n18, eq201_e2524_d_n19, eq201_e2524_d_n20, eq201_e2524_d_n21, eq201_e2524_d_n22, eq201_e2524_q, eq201_e2524_q_d_n0, eq201_e2524_q_d_n1, eq201_e2524_q_d_n2, eq201_e2524_q_d_n3, eq201_e2524_q_d_n4, eq201_e2524_q_d_n5, eq201_e2524_q_d_n6, eq201_e2524_q_d_n7, eq201_e2524_q_d_n8, eq201_e2524_q_d_n9, eq201_e2524_q_d_n10, eq201_e2524_q_d_n11, eq201_e2524_q_d_n12, eq201_e2524_q_d_n13, eq201_e2524_q_d_n14, eq201_e2524_q_d_n15, eq201_e2524_q_d_n16, eq201_e2524_q_d_n17, eq201_e2524_q_d_n18, eq201_e2524_q_d_n19, eq201_e2524_q_d_n20, eq201_e2524_q_d_n21, eq201_e2524_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq201_reactive_node_derivatives: [f64; 23] = [eq201_e2526_q_d_n0, eq201_e2526_q_d_n1, eq201_e2526_q_d_n2, eq201_e2526_q_d_n3, eq201_e2526_q_d_n4, eq201_e2526_q_d_n5, eq201_e2526_q_d_n6, eq201_e2526_q_d_n7, eq201_e2526_q_d_n8, eq201_e2526_q_d_n9, eq201_e2526_q_d_n10, eq201_e2526_q_d_n11, eq201_e2526_q_d_n12, eq201_e2526_q_d_n13, eq201_e2526_q_d_n14, eq201_e2526_q_d_n15, eq201_e2526_q_d_n16, eq201_e2526_q_d_n17, eq201_e2526_q_d_n18, eq201_e2526_q_d_n19, eq201_e2526_q_d_n20, eq201_e2526_q_d_n21, eq201_e2526_q_d_n22];
        let eq201_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq201_reactive_node_derivatives,
            branches,
            &eq201_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq202_e2541, eq202_e2541_d_n0, eq202_e2541_d_n1, eq202_e2541_d_n2, eq202_e2541_d_n3, eq202_e2541_d_n4, eq202_e2541_d_n5, eq202_e2541_d_n6, eq202_e2541_d_n7, eq202_e2541_d_n8, eq202_e2541_d_n9, eq202_e2541_d_n10, eq202_e2541_d_n11, eq202_e2541_d_n12, eq202_e2541_d_n13, eq202_e2541_d_n14, eq202_e2541_d_n15, eq202_e2541_d_n16, eq202_e2541_d_n17, eq202_e2541_d_n18, eq202_e2541_d_n19, eq202_e2541_d_n20, eq202_e2541_d_n21, eq202_e2541_d_n22, eq202_e2541_q, eq202_e2541_q_d_n0, eq202_e2541_q_d_n1, eq202_e2541_q_d_n2, eq202_e2541_q_d_n3, eq202_e2541_q_d_n4, eq202_e2541_q_d_n5, eq202_e2541_q_d_n6, eq202_e2541_q_d_n7, eq202_e2541_q_d_n8, eq202_e2541_q_d_n9, eq202_e2541_q_d_n10, eq202_e2541_q_d_n11, eq202_e2541_q_d_n12, eq202_e2541_q_d_n13, eq202_e2541_q_d_n14, eq202_e2541_q_d_n15, eq202_e2541_q_d_n16, eq202_e2541_q_d_n17, eq202_e2541_q_d_n18, eq202_e2541_q_d_n19, eq202_e2541_q_d_n20, eq202_e2541_q_d_n21, eq202_e2541_q_d_n22,) = {
    if (((!s.b[600]) && s.b[603]) && (!s.b[604])) {
        let eq202_e2536_q: f64 = s.v[300];
        let eq202_e2537: f64 = (p.p7 * s.v[300]);
        let eq202_e2537_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq202_e2537_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq202_e2537_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq202_e2537_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq202_e2537_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq202_e2537_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq202_e2537_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq202_e2537_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq202_e2537_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq202_e2537_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq202_e2537_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq202_e2537_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq202_e2537_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq202_e2537_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq202_e2537_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq202_e2537_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq202_e2537_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq202_e2537_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq202_e2537_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq202_e2537_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq202_e2537_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq202_e2537_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq202_e2537_d_n22: f64 = (p.p7 * s.dn[300][22]);
        let eq202_e2537_q: f64 = (p.p7 * eq202_e2536_q);
        let eq202_e2537_q_d_n0: f64 = (p.p7 * s.dn[300][0]);
        let eq202_e2537_q_d_n1: f64 = (p.p7 * s.dn[300][1]);
        let eq202_e2537_q_d_n2: f64 = (p.p7 * s.dn[300][2]);
        let eq202_e2537_q_d_n3: f64 = (p.p7 * s.dn[300][3]);
        let eq202_e2537_q_d_n4: f64 = (p.p7 * s.dn[300][4]);
        let eq202_e2537_q_d_n5: f64 = (p.p7 * s.dn[300][5]);
        let eq202_e2537_q_d_n6: f64 = (p.p7 * s.dn[300][6]);
        let eq202_e2537_q_d_n7: f64 = (p.p7 * s.dn[300][7]);
        let eq202_e2537_q_d_n8: f64 = (p.p7 * s.dn[300][8]);
        let eq202_e2537_q_d_n9: f64 = (p.p7 * s.dn[300][9]);
        let eq202_e2537_q_d_n10: f64 = (p.p7 * s.dn[300][10]);
        let eq202_e2537_q_d_n11: f64 = (p.p7 * s.dn[300][11]);
        let eq202_e2537_q_d_n12: f64 = (p.p7 * s.dn[300][12]);
        let eq202_e2537_q_d_n13: f64 = (p.p7 * s.dn[300][13]);
        let eq202_e2537_q_d_n14: f64 = (p.p7 * s.dn[300][14]);
        let eq202_e2537_q_d_n15: f64 = (p.p7 * s.dn[300][15]);
        let eq202_e2537_q_d_n16: f64 = (p.p7 * s.dn[300][16]);
        let eq202_e2537_q_d_n17: f64 = (p.p7 * s.dn[300][17]);
        let eq202_e2537_q_d_n18: f64 = (p.p7 * s.dn[300][18]);
        let eq202_e2537_q_d_n19: f64 = (p.p7 * s.dn[300][19]);
        let eq202_e2537_q_d_n20: f64 = (p.p7 * s.dn[300][20]);
        let eq202_e2537_q_d_n21: f64 = (p.p7 * s.dn[300][21]);
        let eq202_e2537_q_d_n22: f64 = (p.p7 * s.dn[300][22]);
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
        let eq202_e2539_q: f64 = (eq202_e2537_q * p.p249);
        let eq202_e2539_q_d_n0: f64 = (eq202_e2537_q_d_n0 * p.p249);
        let eq202_e2539_q_d_n1: f64 = (eq202_e2537_q_d_n1 * p.p249);
        let eq202_e2539_q_d_n2: f64 = (eq202_e2537_q_d_n2 * p.p249);
        let eq202_e2539_q_d_n3: f64 = (eq202_e2537_q_d_n3 * p.p249);
        let eq202_e2539_q_d_n4: f64 = (eq202_e2537_q_d_n4 * p.p249);
        let eq202_e2539_q_d_n5: f64 = (eq202_e2537_q_d_n5 * p.p249);
        let eq202_e2539_q_d_n6: f64 = (eq202_e2537_q_d_n6 * p.p249);
        let eq202_e2539_q_d_n7: f64 = (eq202_e2537_q_d_n7 * p.p249);
        let eq202_e2539_q_d_n8: f64 = (eq202_e2537_q_d_n8 * p.p249);
        let eq202_e2539_q_d_n9: f64 = (eq202_e2537_q_d_n9 * p.p249);
        let eq202_e2539_q_d_n10: f64 = (eq202_e2537_q_d_n10 * p.p249);
        let eq202_e2539_q_d_n11: f64 = (eq202_e2537_q_d_n11 * p.p249);
        let eq202_e2539_q_d_n12: f64 = (eq202_e2537_q_d_n12 * p.p249);
        let eq202_e2539_q_d_n13: f64 = (eq202_e2537_q_d_n13 * p.p249);
        let eq202_e2539_q_d_n14: f64 = (eq202_e2537_q_d_n14 * p.p249);
        let eq202_e2539_q_d_n15: f64 = (eq202_e2537_q_d_n15 * p.p249);
        let eq202_e2539_q_d_n16: f64 = (eq202_e2537_q_d_n16 * p.p249);
        let eq202_e2539_q_d_n17: f64 = (eq202_e2537_q_d_n17 * p.p249);
        let eq202_e2539_q_d_n18: f64 = (eq202_e2537_q_d_n18 * p.p249);
        let eq202_e2539_q_d_n19: f64 = (eq202_e2537_q_d_n19 * p.p249);
        let eq202_e2539_q_d_n20: f64 = (eq202_e2537_q_d_n20 * p.p249);
        let eq202_e2539_q_d_n21: f64 = (eq202_e2537_q_d_n21 * p.p249);
        let eq202_e2539_q_d_n22: f64 = (eq202_e2537_q_d_n22 * p.p249);
        (eq202_e2539, eq202_e2539_d_n0, eq202_e2539_d_n1, eq202_e2539_d_n2, eq202_e2539_d_n3, eq202_e2539_d_n4, eq202_e2539_d_n5, eq202_e2539_d_n6, eq202_e2539_d_n7, eq202_e2539_d_n8, eq202_e2539_d_n9, eq202_e2539_d_n10, eq202_e2539_d_n11, eq202_e2539_d_n12, eq202_e2539_d_n13, eq202_e2539_d_n14, eq202_e2539_d_n15, eq202_e2539_d_n16, eq202_e2539_d_n17, eq202_e2539_d_n18, eq202_e2539_d_n19, eq202_e2539_d_n20, eq202_e2539_d_n21, eq202_e2539_d_n22, eq202_e2539_q, eq202_e2539_q_d_n0, eq202_e2539_q_d_n1, eq202_e2539_q_d_n2, eq202_e2539_q_d_n3, eq202_e2539_q_d_n4, eq202_e2539_q_d_n5, eq202_e2539_q_d_n6, eq202_e2539_q_d_n7, eq202_e2539_q_d_n8, eq202_e2539_q_d_n9, eq202_e2539_q_d_n10, eq202_e2539_q_d_n11, eq202_e2539_q_d_n12, eq202_e2539_q_d_n13, eq202_e2539_q_d_n14, eq202_e2539_q_d_n15, eq202_e2539_q_d_n16, eq202_e2539_q_d_n17, eq202_e2539_q_d_n18, eq202_e2539_q_d_n19, eq202_e2539_q_d_n20, eq202_e2539_q_d_n21, eq202_e2539_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq202_reactive_node_derivatives: [f64; 23] = [eq202_e2541_q_d_n0, eq202_e2541_q_d_n1, eq202_e2541_q_d_n2, eq202_e2541_q_d_n3, eq202_e2541_q_d_n4, eq202_e2541_q_d_n5, eq202_e2541_q_d_n6, eq202_e2541_q_d_n7, eq202_e2541_q_d_n8, eq202_e2541_q_d_n9, eq202_e2541_q_d_n10, eq202_e2541_q_d_n11, eq202_e2541_q_d_n12, eq202_e2541_q_d_n13, eq202_e2541_q_d_n14, eq202_e2541_q_d_n15, eq202_e2541_q_d_n16, eq202_e2541_q_d_n17, eq202_e2541_q_d_n18, eq202_e2541_q_d_n19, eq202_e2541_q_d_n20, eq202_e2541_q_d_n21, eq202_e2541_q_d_n22];
        let eq202_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq202_reactive_node_derivatives,
            branches,
            &eq202_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_18(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq203_e2553, eq203_e2553_d_n0, eq203_e2553_d_n1, eq203_e2553_d_n2, eq203_e2553_d_n3, eq203_e2553_d_n4, eq203_e2553_d_n5, eq203_e2553_d_n6, eq203_e2553_d_n7, eq203_e2553_d_n8, eq203_e2553_d_n9, eq203_e2553_d_n10, eq203_e2553_d_n11, eq203_e2553_d_n12, eq203_e2553_d_n13, eq203_e2553_d_n14, eq203_e2553_d_n15, eq203_e2553_d_n16, eq203_e2553_d_n17, eq203_e2553_d_n18, eq203_e2553_d_n19, eq203_e2553_d_n20, eq203_e2553_d_n21, eq203_e2553_d_n22, eq203_e2553_q, eq203_e2553_q_d_n0, eq203_e2553_q_d_n1, eq203_e2553_q_d_n2, eq203_e2553_q_d_n3, eq203_e2553_q_d_n4, eq203_e2553_q_d_n5, eq203_e2553_q_d_n6, eq203_e2553_q_d_n7, eq203_e2553_q_d_n8, eq203_e2553_q_d_n9, eq203_e2553_q_d_n10, eq203_e2553_q_d_n11, eq203_e2553_q_d_n12, eq203_e2553_q_d_n13, eq203_e2553_q_d_n14, eq203_e2553_q_d_n15, eq203_e2553_q_d_n16, eq203_e2553_q_d_n17, eq203_e2553_q_d_n18, eq203_e2553_q_d_n19, eq203_e2553_q_d_n20, eq203_e2553_q_d_n21, eq203_e2553_q_d_n22,) = {
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
        let eq203_e2551_d_n10: f64 = (p.p7 * eq203_e2549_d_n10);
        let eq203_e2551_d_n11: f64 = (p.p7 * eq203_e2549_d_n11);
        let eq203_e2551_d_n12: f64 = (p.p7 * eq203_e2549_d_n12);
        let eq203_e2551_d_n13: f64 = (p.p7 * eq203_e2549_d_n13);
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
        let eq203_e2551_q_d_n0: f64 = (p.p7 * eq203_e2549_d_n0);
        let eq203_e2551_q_d_n1: f64 = (p.p7 * eq203_e2549_d_n1);
        let eq203_e2551_q_d_n2: f64 = (p.p7 * eq203_e2549_d_n2);
        let eq203_e2551_q_d_n3: f64 = (p.p7 * eq203_e2549_d_n3);
        let eq203_e2551_q_d_n4: f64 = (p.p7 * eq203_e2549_d_n4);
        let eq203_e2551_q_d_n5: f64 = (p.p7 * eq203_e2549_d_n5);
        let eq203_e2551_q_d_n6: f64 = (p.p7 * eq203_e2549_d_n6);
        let eq203_e2551_q_d_n7: f64 = (p.p7 * eq203_e2549_d_n7);
        let eq203_e2551_q_d_n8: f64 = (p.p7 * eq203_e2549_d_n8);
        let eq203_e2551_q_d_n9: f64 = (p.p7 * eq203_e2549_d_n9);
        let eq203_e2551_q_d_n10: f64 = (p.p7 * eq203_e2549_d_n10);
        let eq203_e2551_q_d_n11: f64 = (p.p7 * eq203_e2549_d_n11);
        let eq203_e2551_q_d_n12: f64 = (p.p7 * eq203_e2549_d_n12);
        let eq203_e2551_q_d_n13: f64 = (p.p7 * eq203_e2549_d_n13);
        let eq203_e2551_q_d_n14: f64 = (p.p7 * eq203_e2549_d_n14);
        let eq203_e2551_q_d_n15: f64 = (p.p7 * eq203_e2549_d_n15);
        let eq203_e2551_q_d_n16: f64 = (p.p7 * eq203_e2549_d_n16);
        let eq203_e2551_q_d_n17: f64 = (p.p7 * eq203_e2549_d_n17);
        let eq203_e2551_q_d_n18: f64 = (p.p7 * eq203_e2549_d_n18);
        let eq203_e2551_q_d_n19: f64 = (p.p7 * eq203_e2549_d_n19);
        let eq203_e2551_q_d_n20: f64 = (p.p7 * eq203_e2549_d_n20);
        let eq203_e2551_q_d_n21: f64 = (p.p7 * eq203_e2549_d_n21);
        let eq203_e2551_q_d_n22: f64 = (p.p7 * eq203_e2549_d_n22);
        (eq203_e2551, eq203_e2551_d_n0, eq203_e2551_d_n1, eq203_e2551_d_n2, eq203_e2551_d_n3, eq203_e2551_d_n4, eq203_e2551_d_n5, eq203_e2551_d_n6, eq203_e2551_d_n7, eq203_e2551_d_n8, eq203_e2551_d_n9, eq203_e2551_d_n10, eq203_e2551_d_n11, eq203_e2551_d_n12, eq203_e2551_d_n13, eq203_e2551_d_n14, eq203_e2551_d_n15, eq203_e2551_d_n16, eq203_e2551_d_n17, eq203_e2551_d_n18, eq203_e2551_d_n19, eq203_e2551_d_n20, eq203_e2551_d_n21, eq203_e2551_d_n22, eq203_e2551_q, eq203_e2551_q_d_n0, eq203_e2551_q_d_n1, eq203_e2551_q_d_n2, eq203_e2551_q_d_n3, eq203_e2551_q_d_n4, eq203_e2551_q_d_n5, eq203_e2551_q_d_n6, eq203_e2551_q_d_n7, eq203_e2551_q_d_n8, eq203_e2551_q_d_n9, eq203_e2551_q_d_n10, eq203_e2551_q_d_n11, eq203_e2551_q_d_n12, eq203_e2551_q_d_n13, eq203_e2551_q_d_n14, eq203_e2551_q_d_n15, eq203_e2551_q_d_n16, eq203_e2551_q_d_n17, eq203_e2551_q_d_n18, eq203_e2551_q_d_n19, eq203_e2551_q_d_n20, eq203_e2551_q_d_n21, eq203_e2551_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq203_reactive_node_derivatives: [f64; 23] = [eq203_e2553_q_d_n0, eq203_e2553_q_d_n1, eq203_e2553_q_d_n2, eq203_e2553_q_d_n3, eq203_e2553_q_d_n4, eq203_e2553_q_d_n5, eq203_e2553_q_d_n6, eq203_e2553_q_d_n7, eq203_e2553_q_d_n8, eq203_e2553_q_d_n9, eq203_e2553_q_d_n10, eq203_e2553_q_d_n11, eq203_e2553_q_d_n12, eq203_e2553_q_d_n13, eq203_e2553_q_d_n14, eq203_e2553_q_d_n15, eq203_e2553_q_d_n16, eq203_e2553_q_d_n17, eq203_e2553_q_d_n18, eq203_e2553_q_d_n19, eq203_e2553_q_d_n20, eq203_e2553_q_d_n21, eq203_e2553_q_d_n22];
        let eq203_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq203_reactive_node_derivatives,
            branches,
            &eq203_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq204_e2562, eq204_e2562_d_n0, eq204_e2562_d_n1, eq204_e2562_d_n2, eq204_e2562_d_n3, eq204_e2562_d_n4, eq204_e2562_d_n5, eq204_e2562_d_n6, eq204_e2562_d_n7, eq204_e2562_d_n8, eq204_e2562_d_n9, eq204_e2562_d_n10, eq204_e2562_d_n11, eq204_e2562_d_n12, eq204_e2562_d_n13, eq204_e2562_d_n14, eq204_e2562_d_n15, eq204_e2562_d_n16, eq204_e2562_d_n17, eq204_e2562_d_n18, eq204_e2562_d_n19, eq204_e2562_d_n20, eq204_e2562_d_n21, eq204_e2562_d_n22, eq204_e2562_q, eq204_e2562_q_d_n0, eq204_e2562_q_d_n1, eq204_e2562_q_d_n2, eq204_e2562_q_d_n3, eq204_e2562_q_d_n4, eq204_e2562_q_d_n5, eq204_e2562_q_d_n6, eq204_e2562_q_d_n7, eq204_e2562_q_d_n8, eq204_e2562_q_d_n9, eq204_e2562_q_d_n10, eq204_e2562_q_d_n11, eq204_e2562_q_d_n12, eq204_e2562_q_d_n13, eq204_e2562_q_d_n14, eq204_e2562_q_d_n15, eq204_e2562_q_d_n16, eq204_e2562_q_d_n17, eq204_e2562_q_d_n18, eq204_e2562_q_d_n19, eq204_e2562_q_d_n20, eq204_e2562_q_d_n21, eq204_e2562_q_d_n22,) = {
    if (s.b[605] && s.b[606]) {
        let eq204_e2559_q: f64 = s.v[313];
        let eq204_e2560: f64 = (p.p7 * s.v[313]);
        let eq204_e2560_d_n0: f64 = (p.p7 * s.dn[313][0]);
        let eq204_e2560_d_n1: f64 = (p.p7 * s.dn[313][1]);
        let eq204_e2560_d_n2: f64 = (p.p7 * s.dn[313][2]);
        let eq204_e2560_d_n3: f64 = (p.p7 * s.dn[313][3]);
        let eq204_e2560_d_n4: f64 = (p.p7 * s.dn[313][4]);
        let eq204_e2560_d_n5: f64 = (p.p7 * s.dn[313][5]);
        let eq204_e2560_d_n6: f64 = (p.p7 * s.dn[313][6]);
        let eq204_e2560_d_n7: f64 = (p.p7 * s.dn[313][7]);
        let eq204_e2560_d_n8: f64 = (p.p7 * s.dn[313][8]);
        let eq204_e2560_d_n9: f64 = (p.p7 * s.dn[313][9]);
        let eq204_e2560_d_n10: f64 = (p.p7 * s.dn[313][10]);
        let eq204_e2560_d_n11: f64 = (p.p7 * s.dn[313][11]);
        let eq204_e2560_d_n12: f64 = (p.p7 * s.dn[313][12]);
        let eq204_e2560_d_n13: f64 = (p.p7 * s.dn[313][13]);
        let eq204_e2560_d_n14: f64 = (p.p7 * s.dn[313][14]);
        let eq204_e2560_d_n15: f64 = (p.p7 * s.dn[313][15]);
        let eq204_e2560_d_n16: f64 = (p.p7 * s.dn[313][16]);
        let eq204_e2560_d_n17: f64 = (p.p7 * s.dn[313][17]);
        let eq204_e2560_d_n18: f64 = (p.p7 * s.dn[313][18]);
        let eq204_e2560_d_n19: f64 = (p.p7 * s.dn[313][19]);
        let eq204_e2560_d_n20: f64 = (p.p7 * s.dn[313][20]);
        let eq204_e2560_d_n21: f64 = (p.p7 * s.dn[313][21]);
        let eq204_e2560_d_n22: f64 = (p.p7 * s.dn[313][22]);
        let eq204_e2560_q: f64 = (p.p7 * eq204_e2559_q);
        let eq204_e2560_q_d_n0: f64 = (p.p7 * s.dn[313][0]);
        let eq204_e2560_q_d_n1: f64 = (p.p7 * s.dn[313][1]);
        let eq204_e2560_q_d_n2: f64 = (p.p7 * s.dn[313][2]);
        let eq204_e2560_q_d_n3: f64 = (p.p7 * s.dn[313][3]);
        let eq204_e2560_q_d_n4: f64 = (p.p7 * s.dn[313][4]);
        let eq204_e2560_q_d_n5: f64 = (p.p7 * s.dn[313][5]);
        let eq204_e2560_q_d_n6: f64 = (p.p7 * s.dn[313][6]);
        let eq204_e2560_q_d_n7: f64 = (p.p7 * s.dn[313][7]);
        let eq204_e2560_q_d_n8: f64 = (p.p7 * s.dn[313][8]);
        let eq204_e2560_q_d_n9: f64 = (p.p7 * s.dn[313][9]);
        let eq204_e2560_q_d_n10: f64 = (p.p7 * s.dn[313][10]);
        let eq204_e2560_q_d_n11: f64 = (p.p7 * s.dn[313][11]);
        let eq204_e2560_q_d_n12: f64 = (p.p7 * s.dn[313][12]);
        let eq204_e2560_q_d_n13: f64 = (p.p7 * s.dn[313][13]);
        let eq204_e2560_q_d_n14: f64 = (p.p7 * s.dn[313][14]);
        let eq204_e2560_q_d_n15: f64 = (p.p7 * s.dn[313][15]);
        let eq204_e2560_q_d_n16: f64 = (p.p7 * s.dn[313][16]);
        let eq204_e2560_q_d_n17: f64 = (p.p7 * s.dn[313][17]);
        let eq204_e2560_q_d_n18: f64 = (p.p7 * s.dn[313][18]);
        let eq204_e2560_q_d_n19: f64 = (p.p7 * s.dn[313][19]);
        let eq204_e2560_q_d_n20: f64 = (p.p7 * s.dn[313][20]);
        let eq204_e2560_q_d_n21: f64 = (p.p7 * s.dn[313][21]);
        let eq204_e2560_q_d_n22: f64 = (p.p7 * s.dn[313][22]);
        (eq204_e2560, eq204_e2560_d_n0, eq204_e2560_d_n1, eq204_e2560_d_n2, eq204_e2560_d_n3, eq204_e2560_d_n4, eq204_e2560_d_n5, eq204_e2560_d_n6, eq204_e2560_d_n7, eq204_e2560_d_n8, eq204_e2560_d_n9, eq204_e2560_d_n10, eq204_e2560_d_n11, eq204_e2560_d_n12, eq204_e2560_d_n13, eq204_e2560_d_n14, eq204_e2560_d_n15, eq204_e2560_d_n16, eq204_e2560_d_n17, eq204_e2560_d_n18, eq204_e2560_d_n19, eq204_e2560_d_n20, eq204_e2560_d_n21, eq204_e2560_d_n22, eq204_e2560_q, eq204_e2560_q_d_n0, eq204_e2560_q_d_n1, eq204_e2560_q_d_n2, eq204_e2560_q_d_n3, eq204_e2560_q_d_n4, eq204_e2560_q_d_n5, eq204_e2560_q_d_n6, eq204_e2560_q_d_n7, eq204_e2560_q_d_n8, eq204_e2560_q_d_n9, eq204_e2560_q_d_n10, eq204_e2560_q_d_n11, eq204_e2560_q_d_n12, eq204_e2560_q_d_n13, eq204_e2560_q_d_n14, eq204_e2560_q_d_n15, eq204_e2560_q_d_n16, eq204_e2560_q_d_n17, eq204_e2560_q_d_n18, eq204_e2560_q_d_n19, eq204_e2560_q_d_n20, eq204_e2560_q_d_n21, eq204_e2560_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq204_reactive_node_derivatives: [f64; 23] = [eq204_e2562_q_d_n0, eq204_e2562_q_d_n1, eq204_e2562_q_d_n2, eq204_e2562_q_d_n3, eq204_e2562_q_d_n4, eq204_e2562_q_d_n5, eq204_e2562_q_d_n6, eq204_e2562_q_d_n7, eq204_e2562_q_d_n8, eq204_e2562_q_d_n9, eq204_e2562_q_d_n10, eq204_e2562_q_d_n11, eq204_e2562_q_d_n12, eq204_e2562_q_d_n13, eq204_e2562_q_d_n14, eq204_e2562_q_d_n15, eq204_e2562_q_d_n16, eq204_e2562_q_d_n17, eq204_e2562_q_d_n18, eq204_e2562_q_d_n19, eq204_e2562_q_d_n20, eq204_e2562_q_d_n21, eq204_e2562_q_d_n22];
        let eq204_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[22]),
            nodes,
            &eq204_reactive_node_derivatives,
            branches,
            &eq204_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq205_e2573, eq205_e2573_d_n0, eq205_e2573_d_n1, eq205_e2573_d_n2, eq205_e2573_d_n3, eq205_e2573_d_n4, eq205_e2573_d_n5, eq205_e2573_d_n6, eq205_e2573_d_n7, eq205_e2573_d_n8, eq205_e2573_d_n9, eq205_e2573_d_n10, eq205_e2573_d_n11, eq205_e2573_d_n12, eq205_e2573_d_n13, eq205_e2573_d_n14, eq205_e2573_d_n15, eq205_e2573_d_n16, eq205_e2573_d_n17, eq205_e2573_d_n18, eq205_e2573_d_n19, eq205_e2573_d_n20, eq205_e2573_d_n21, eq205_e2573_d_n22, eq205_e2573_q, eq205_e2573_q_d_n0, eq205_e2573_q_d_n1, eq205_e2573_q_d_n2, eq205_e2573_q_d_n3, eq205_e2573_q_d_n4, eq205_e2573_q_d_n5, eq205_e2573_q_d_n6, eq205_e2573_q_d_n7, eq205_e2573_q_d_n8, eq205_e2573_q_d_n9, eq205_e2573_q_d_n10, eq205_e2573_q_d_n11, eq205_e2573_q_d_n12, eq205_e2573_q_d_n13, eq205_e2573_q_d_n14, eq205_e2573_q_d_n15, eq205_e2573_q_d_n16, eq205_e2573_q_d_n17, eq205_e2573_q_d_n18, eq205_e2573_q_d_n19, eq205_e2573_q_d_n20, eq205_e2573_q_d_n21, eq205_e2573_q_d_n22,) = {
    if ((s.b[605] && s.b[606]) && s.b[607]) {
        let eq205_e2570_q: f64 = s.v[312];
        let eq205_e2571: f64 = (p.p7 * s.v[312]);
        let eq205_e2571_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq205_e2571_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq205_e2571_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq205_e2571_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq205_e2571_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq205_e2571_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq205_e2571_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq205_e2571_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq205_e2571_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq205_e2571_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq205_e2571_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq205_e2571_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq205_e2571_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq205_e2571_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq205_e2571_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq205_e2571_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq205_e2571_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq205_e2571_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq205_e2571_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq205_e2571_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq205_e2571_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq205_e2571_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq205_e2571_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq205_e2571_q: f64 = (p.p7 * eq205_e2570_q);
        let eq205_e2571_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq205_e2571_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq205_e2571_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq205_e2571_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq205_e2571_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq205_e2571_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq205_e2571_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq205_e2571_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq205_e2571_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq205_e2571_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq205_e2571_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq205_e2571_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq205_e2571_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq205_e2571_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq205_e2571_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq205_e2571_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq205_e2571_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq205_e2571_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq205_e2571_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq205_e2571_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq205_e2571_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq205_e2571_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq205_e2571_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
        (eq205_e2571, eq205_e2571_d_n0, eq205_e2571_d_n1, eq205_e2571_d_n2, eq205_e2571_d_n3, eq205_e2571_d_n4, eq205_e2571_d_n5, eq205_e2571_d_n6, eq205_e2571_d_n7, eq205_e2571_d_n8, eq205_e2571_d_n9, eq205_e2571_d_n10, eq205_e2571_d_n11, eq205_e2571_d_n12, eq205_e2571_d_n13, eq205_e2571_d_n14, eq205_e2571_d_n15, eq205_e2571_d_n16, eq205_e2571_d_n17, eq205_e2571_d_n18, eq205_e2571_d_n19, eq205_e2571_d_n20, eq205_e2571_d_n21, eq205_e2571_d_n22, eq205_e2571_q, eq205_e2571_q_d_n0, eq205_e2571_q_d_n1, eq205_e2571_q_d_n2, eq205_e2571_q_d_n3, eq205_e2571_q_d_n4, eq205_e2571_q_d_n5, eq205_e2571_q_d_n6, eq205_e2571_q_d_n7, eq205_e2571_q_d_n8, eq205_e2571_q_d_n9, eq205_e2571_q_d_n10, eq205_e2571_q_d_n11, eq205_e2571_q_d_n12, eq205_e2571_q_d_n13, eq205_e2571_q_d_n14, eq205_e2571_q_d_n15, eq205_e2571_q_d_n16, eq205_e2571_q_d_n17, eq205_e2571_q_d_n18, eq205_e2571_q_d_n19, eq205_e2571_q_d_n20, eq205_e2571_q_d_n21, eq205_e2571_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq205_reactive_node_derivatives: [f64; 23] = [eq205_e2573_q_d_n0, eq205_e2573_q_d_n1, eq205_e2573_q_d_n2, eq205_e2573_q_d_n3, eq205_e2573_q_d_n4, eq205_e2573_q_d_n5, eq205_e2573_q_d_n6, eq205_e2573_q_d_n7, eq205_e2573_q_d_n8, eq205_e2573_q_d_n9, eq205_e2573_q_d_n10, eq205_e2573_q_d_n11, eq205_e2573_q_d_n12, eq205_e2573_q_d_n13, eq205_e2573_q_d_n14, eq205_e2573_q_d_n15, eq205_e2573_q_d_n16, eq205_e2573_q_d_n17, eq205_e2573_q_d_n18, eq205_e2573_q_d_n19, eq205_e2573_q_d_n20, eq205_e2573_q_d_n21, eq205_e2573_q_d_n22];
        let eq205_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            nodes,
            &eq205_reactive_node_derivatives,
            branches,
            &eq205_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq206_e2586, eq206_e2586_d_n0, eq206_e2586_d_n1, eq206_e2586_d_n2, eq206_e2586_d_n3, eq206_e2586_d_n4, eq206_e2586_d_n5, eq206_e2586_d_n6, eq206_e2586_d_n7, eq206_e2586_d_n8, eq206_e2586_d_n9, eq206_e2586_d_n10, eq206_e2586_d_n11, eq206_e2586_d_n12, eq206_e2586_d_n13, eq206_e2586_d_n14, eq206_e2586_d_n15, eq206_e2586_d_n16, eq206_e2586_d_n17, eq206_e2586_d_n18, eq206_e2586_d_n19, eq206_e2586_d_n20, eq206_e2586_d_n21, eq206_e2586_d_n22, eq206_e2586_q, eq206_e2586_q_d_n0, eq206_e2586_q_d_n1, eq206_e2586_q_d_n2, eq206_e2586_q_d_n3, eq206_e2586_q_d_n4, eq206_e2586_q_d_n5, eq206_e2586_q_d_n6, eq206_e2586_q_d_n7, eq206_e2586_q_d_n8, eq206_e2586_q_d_n9, eq206_e2586_q_d_n10, eq206_e2586_q_d_n11, eq206_e2586_q_d_n12, eq206_e2586_q_d_n13, eq206_e2586_q_d_n14, eq206_e2586_q_d_n15, eq206_e2586_q_d_n16, eq206_e2586_q_d_n17, eq206_e2586_q_d_n18, eq206_e2586_q_d_n19, eq206_e2586_q_d_n20, eq206_e2586_q_d_n21, eq206_e2586_q_d_n22,) = {
    if ((s.b[605] && s.b[606]) && s.b[607]) {
        let eq206_e2581_q: f64 = s.v[312];
        let eq206_e2582: f64 = (p.p7 * s.v[312]);
        let eq206_e2582_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq206_e2582_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq206_e2582_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq206_e2582_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq206_e2582_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq206_e2582_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq206_e2582_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq206_e2582_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq206_e2582_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq206_e2582_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq206_e2582_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq206_e2582_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq206_e2582_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq206_e2582_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq206_e2582_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq206_e2582_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq206_e2582_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq206_e2582_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq206_e2582_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq206_e2582_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq206_e2582_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq206_e2582_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq206_e2582_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq206_e2582_q: f64 = (p.p7 * eq206_e2581_q);
        let eq206_e2582_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq206_e2582_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq206_e2582_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq206_e2582_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq206_e2582_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq206_e2582_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq206_e2582_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq206_e2582_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq206_e2582_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq206_e2582_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq206_e2582_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq206_e2582_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq206_e2582_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq206_e2582_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq206_e2582_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq206_e2582_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq206_e2582_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq206_e2582_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq206_e2582_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq206_e2582_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq206_e2582_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq206_e2582_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq206_e2582_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
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
        let eq206_e2584_q: f64 = (eq206_e2582_q * p.p249);
        let eq206_e2584_q_d_n0: f64 = (eq206_e2582_q_d_n0 * p.p249);
        let eq206_e2584_q_d_n1: f64 = (eq206_e2582_q_d_n1 * p.p249);
        let eq206_e2584_q_d_n2: f64 = (eq206_e2582_q_d_n2 * p.p249);
        let eq206_e2584_q_d_n3: f64 = (eq206_e2582_q_d_n3 * p.p249);
        let eq206_e2584_q_d_n4: f64 = (eq206_e2582_q_d_n4 * p.p249);
        let eq206_e2584_q_d_n5: f64 = (eq206_e2582_q_d_n5 * p.p249);
        let eq206_e2584_q_d_n6: f64 = (eq206_e2582_q_d_n6 * p.p249);
        let eq206_e2584_q_d_n7: f64 = (eq206_e2582_q_d_n7 * p.p249);
        let eq206_e2584_q_d_n8: f64 = (eq206_e2582_q_d_n8 * p.p249);
        let eq206_e2584_q_d_n9: f64 = (eq206_e2582_q_d_n9 * p.p249);
        let eq206_e2584_q_d_n10: f64 = (eq206_e2582_q_d_n10 * p.p249);
        let eq206_e2584_q_d_n11: f64 = (eq206_e2582_q_d_n11 * p.p249);
        let eq206_e2584_q_d_n12: f64 = (eq206_e2582_q_d_n12 * p.p249);
        let eq206_e2584_q_d_n13: f64 = (eq206_e2582_q_d_n13 * p.p249);
        let eq206_e2584_q_d_n14: f64 = (eq206_e2582_q_d_n14 * p.p249);
        let eq206_e2584_q_d_n15: f64 = (eq206_e2582_q_d_n15 * p.p249);
        let eq206_e2584_q_d_n16: f64 = (eq206_e2582_q_d_n16 * p.p249);
        let eq206_e2584_q_d_n17: f64 = (eq206_e2582_q_d_n17 * p.p249);
        let eq206_e2584_q_d_n18: f64 = (eq206_e2582_q_d_n18 * p.p249);
        let eq206_e2584_q_d_n19: f64 = (eq206_e2582_q_d_n19 * p.p249);
        let eq206_e2584_q_d_n20: f64 = (eq206_e2582_q_d_n20 * p.p249);
        let eq206_e2584_q_d_n21: f64 = (eq206_e2582_q_d_n21 * p.p249);
        let eq206_e2584_q_d_n22: f64 = (eq206_e2582_q_d_n22 * p.p249);
        (eq206_e2584, eq206_e2584_d_n0, eq206_e2584_d_n1, eq206_e2584_d_n2, eq206_e2584_d_n3, eq206_e2584_d_n4, eq206_e2584_d_n5, eq206_e2584_d_n6, eq206_e2584_d_n7, eq206_e2584_d_n8, eq206_e2584_d_n9, eq206_e2584_d_n10, eq206_e2584_d_n11, eq206_e2584_d_n12, eq206_e2584_d_n13, eq206_e2584_d_n14, eq206_e2584_d_n15, eq206_e2584_d_n16, eq206_e2584_d_n17, eq206_e2584_d_n18, eq206_e2584_d_n19, eq206_e2584_d_n20, eq206_e2584_d_n21, eq206_e2584_d_n22, eq206_e2584_q, eq206_e2584_q_d_n0, eq206_e2584_q_d_n1, eq206_e2584_q_d_n2, eq206_e2584_q_d_n3, eq206_e2584_q_d_n4, eq206_e2584_q_d_n5, eq206_e2584_q_d_n6, eq206_e2584_q_d_n7, eq206_e2584_q_d_n8, eq206_e2584_q_d_n9, eq206_e2584_q_d_n10, eq206_e2584_q_d_n11, eq206_e2584_q_d_n12, eq206_e2584_q_d_n13, eq206_e2584_q_d_n14, eq206_e2584_q_d_n15, eq206_e2584_q_d_n16, eq206_e2584_q_d_n17, eq206_e2584_q_d_n18, eq206_e2584_q_d_n19, eq206_e2584_q_d_n20, eq206_e2584_q_d_n21, eq206_e2584_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq206_reactive_node_derivatives: [f64; 23] = [eq206_e2586_q_d_n0, eq206_e2586_q_d_n1, eq206_e2586_q_d_n2, eq206_e2586_q_d_n3, eq206_e2586_q_d_n4, eq206_e2586_q_d_n5, eq206_e2586_q_d_n6, eq206_e2586_q_d_n7, eq206_e2586_q_d_n8, eq206_e2586_q_d_n9, eq206_e2586_q_d_n10, eq206_e2586_q_d_n11, eq206_e2586_q_d_n12, eq206_e2586_q_d_n13, eq206_e2586_q_d_n14, eq206_e2586_q_d_n15, eq206_e2586_q_d_n16, eq206_e2586_q_d_n17, eq206_e2586_q_d_n18, eq206_e2586_q_d_n19, eq206_e2586_q_d_n20, eq206_e2586_q_d_n21, eq206_e2586_q_d_n22];
        let eq206_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            nodes,
            &eq206_reactive_node_derivatives,
            branches,
            &eq206_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq207_e2598, eq207_e2598_d_n0, eq207_e2598_d_n1, eq207_e2598_d_n2, eq207_e2598_d_n3, eq207_e2598_d_n4, eq207_e2598_d_n5, eq207_e2598_d_n6, eq207_e2598_d_n7, eq207_e2598_d_n8, eq207_e2598_d_n9, eq207_e2598_d_n10, eq207_e2598_d_n11, eq207_e2598_d_n12, eq207_e2598_d_n13, eq207_e2598_d_n14, eq207_e2598_d_n15, eq207_e2598_d_n16, eq207_e2598_d_n17, eq207_e2598_d_n18, eq207_e2598_d_n19, eq207_e2598_d_n20, eq207_e2598_d_n21, eq207_e2598_d_n22, eq207_e2598_q, eq207_e2598_q_d_n0, eq207_e2598_q_d_n1, eq207_e2598_q_d_n2, eq207_e2598_q_d_n3, eq207_e2598_q_d_n4, eq207_e2598_q_d_n5, eq207_e2598_q_d_n6, eq207_e2598_q_d_n7, eq207_e2598_q_d_n8, eq207_e2598_q_d_n9, eq207_e2598_q_d_n10, eq207_e2598_q_d_n11, eq207_e2598_q_d_n12, eq207_e2598_q_d_n13, eq207_e2598_q_d_n14, eq207_e2598_q_d_n15, eq207_e2598_q_d_n16, eq207_e2598_q_d_n17, eq207_e2598_q_d_n18, eq207_e2598_q_d_n19, eq207_e2598_q_d_n20, eq207_e2598_q_d_n21, eq207_e2598_q_d_n22,) = {
    if ((s.b[605] && s.b[606]) && (!s.b[607])) {
        let eq207_e2595_q: f64 = s.v[312];
        let eq207_e2596: f64 = (p.p7 * s.v[312]);
        let eq207_e2596_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq207_e2596_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq207_e2596_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq207_e2596_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq207_e2596_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq207_e2596_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq207_e2596_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq207_e2596_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq207_e2596_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq207_e2596_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq207_e2596_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq207_e2596_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq207_e2596_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq207_e2596_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq207_e2596_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq207_e2596_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq207_e2596_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq207_e2596_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq207_e2596_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq207_e2596_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq207_e2596_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq207_e2596_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq207_e2596_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq207_e2596_q: f64 = (p.p7 * eq207_e2595_q);
        let eq207_e2596_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq207_e2596_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq207_e2596_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq207_e2596_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq207_e2596_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq207_e2596_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq207_e2596_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq207_e2596_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq207_e2596_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq207_e2596_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq207_e2596_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq207_e2596_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq207_e2596_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq207_e2596_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq207_e2596_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq207_e2596_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq207_e2596_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq207_e2596_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq207_e2596_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq207_e2596_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq207_e2596_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq207_e2596_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq207_e2596_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
        (eq207_e2596, eq207_e2596_d_n0, eq207_e2596_d_n1, eq207_e2596_d_n2, eq207_e2596_d_n3, eq207_e2596_d_n4, eq207_e2596_d_n5, eq207_e2596_d_n6, eq207_e2596_d_n7, eq207_e2596_d_n8, eq207_e2596_d_n9, eq207_e2596_d_n10, eq207_e2596_d_n11, eq207_e2596_d_n12, eq207_e2596_d_n13, eq207_e2596_d_n14, eq207_e2596_d_n15, eq207_e2596_d_n16, eq207_e2596_d_n17, eq207_e2596_d_n18, eq207_e2596_d_n19, eq207_e2596_d_n20, eq207_e2596_d_n21, eq207_e2596_d_n22, eq207_e2596_q, eq207_e2596_q_d_n0, eq207_e2596_q_d_n1, eq207_e2596_q_d_n2, eq207_e2596_q_d_n3, eq207_e2596_q_d_n4, eq207_e2596_q_d_n5, eq207_e2596_q_d_n6, eq207_e2596_q_d_n7, eq207_e2596_q_d_n8, eq207_e2596_q_d_n9, eq207_e2596_q_d_n10, eq207_e2596_q_d_n11, eq207_e2596_q_d_n12, eq207_e2596_q_d_n13, eq207_e2596_q_d_n14, eq207_e2596_q_d_n15, eq207_e2596_q_d_n16, eq207_e2596_q_d_n17, eq207_e2596_q_d_n18, eq207_e2596_q_d_n19, eq207_e2596_q_d_n20, eq207_e2596_q_d_n21, eq207_e2596_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq207_reactive_node_derivatives: [f64; 23] = [eq207_e2598_q_d_n0, eq207_e2598_q_d_n1, eq207_e2598_q_d_n2, eq207_e2598_q_d_n3, eq207_e2598_q_d_n4, eq207_e2598_q_d_n5, eq207_e2598_q_d_n6, eq207_e2598_q_d_n7, eq207_e2598_q_d_n8, eq207_e2598_q_d_n9, eq207_e2598_q_d_n10, eq207_e2598_q_d_n11, eq207_e2598_q_d_n12, eq207_e2598_q_d_n13, eq207_e2598_q_d_n14, eq207_e2598_q_d_n15, eq207_e2598_q_d_n16, eq207_e2598_q_d_n17, eq207_e2598_q_d_n18, eq207_e2598_q_d_n19, eq207_e2598_q_d_n20, eq207_e2598_q_d_n21, eq207_e2598_q_d_n22];
        let eq207_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[22]),
            nodes,
            &eq207_reactive_node_derivatives,
            branches,
            &eq207_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_19(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq208_e2612, eq208_e2612_d_n0, eq208_e2612_d_n1, eq208_e2612_d_n2, eq208_e2612_d_n3, eq208_e2612_d_n4, eq208_e2612_d_n5, eq208_e2612_d_n6, eq208_e2612_d_n7, eq208_e2612_d_n8, eq208_e2612_d_n9, eq208_e2612_d_n10, eq208_e2612_d_n11, eq208_e2612_d_n12, eq208_e2612_d_n13, eq208_e2612_d_n14, eq208_e2612_d_n15, eq208_e2612_d_n16, eq208_e2612_d_n17, eq208_e2612_d_n18, eq208_e2612_d_n19, eq208_e2612_d_n20, eq208_e2612_d_n21, eq208_e2612_d_n22, eq208_e2612_q, eq208_e2612_q_d_n0, eq208_e2612_q_d_n1, eq208_e2612_q_d_n2, eq208_e2612_q_d_n3, eq208_e2612_q_d_n4, eq208_e2612_q_d_n5, eq208_e2612_q_d_n6, eq208_e2612_q_d_n7, eq208_e2612_q_d_n8, eq208_e2612_q_d_n9, eq208_e2612_q_d_n10, eq208_e2612_q_d_n11, eq208_e2612_q_d_n12, eq208_e2612_q_d_n13, eq208_e2612_q_d_n14, eq208_e2612_q_d_n15, eq208_e2612_q_d_n16, eq208_e2612_q_d_n17, eq208_e2612_q_d_n18, eq208_e2612_q_d_n19, eq208_e2612_q_d_n20, eq208_e2612_q_d_n21, eq208_e2612_q_d_n22,) = {
    if ((s.b[605] && s.b[606]) && (!s.b[607])) {
        let eq208_e2607_q: f64 = s.v[312];
        let eq208_e2608: f64 = (p.p7 * s.v[312]);
        let eq208_e2608_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq208_e2608_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq208_e2608_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq208_e2608_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq208_e2608_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq208_e2608_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq208_e2608_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq208_e2608_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq208_e2608_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq208_e2608_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq208_e2608_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq208_e2608_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq208_e2608_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq208_e2608_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq208_e2608_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq208_e2608_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq208_e2608_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq208_e2608_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq208_e2608_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq208_e2608_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq208_e2608_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq208_e2608_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq208_e2608_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq208_e2608_q: f64 = (p.p7 * eq208_e2607_q);
        let eq208_e2608_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq208_e2608_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq208_e2608_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq208_e2608_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq208_e2608_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq208_e2608_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq208_e2608_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq208_e2608_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq208_e2608_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq208_e2608_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq208_e2608_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq208_e2608_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq208_e2608_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq208_e2608_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq208_e2608_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq208_e2608_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq208_e2608_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq208_e2608_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq208_e2608_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq208_e2608_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq208_e2608_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq208_e2608_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq208_e2608_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
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
        let eq208_e2610_q: f64 = (eq208_e2608_q * p.p249);
        let eq208_e2610_q_d_n0: f64 = (eq208_e2608_q_d_n0 * p.p249);
        let eq208_e2610_q_d_n1: f64 = (eq208_e2608_q_d_n1 * p.p249);
        let eq208_e2610_q_d_n2: f64 = (eq208_e2608_q_d_n2 * p.p249);
        let eq208_e2610_q_d_n3: f64 = (eq208_e2608_q_d_n3 * p.p249);
        let eq208_e2610_q_d_n4: f64 = (eq208_e2608_q_d_n4 * p.p249);
        let eq208_e2610_q_d_n5: f64 = (eq208_e2608_q_d_n5 * p.p249);
        let eq208_e2610_q_d_n6: f64 = (eq208_e2608_q_d_n6 * p.p249);
        let eq208_e2610_q_d_n7: f64 = (eq208_e2608_q_d_n7 * p.p249);
        let eq208_e2610_q_d_n8: f64 = (eq208_e2608_q_d_n8 * p.p249);
        let eq208_e2610_q_d_n9: f64 = (eq208_e2608_q_d_n9 * p.p249);
        let eq208_e2610_q_d_n10: f64 = (eq208_e2608_q_d_n10 * p.p249);
        let eq208_e2610_q_d_n11: f64 = (eq208_e2608_q_d_n11 * p.p249);
        let eq208_e2610_q_d_n12: f64 = (eq208_e2608_q_d_n12 * p.p249);
        let eq208_e2610_q_d_n13: f64 = (eq208_e2608_q_d_n13 * p.p249);
        let eq208_e2610_q_d_n14: f64 = (eq208_e2608_q_d_n14 * p.p249);
        let eq208_e2610_q_d_n15: f64 = (eq208_e2608_q_d_n15 * p.p249);
        let eq208_e2610_q_d_n16: f64 = (eq208_e2608_q_d_n16 * p.p249);
        let eq208_e2610_q_d_n17: f64 = (eq208_e2608_q_d_n17 * p.p249);
        let eq208_e2610_q_d_n18: f64 = (eq208_e2608_q_d_n18 * p.p249);
        let eq208_e2610_q_d_n19: f64 = (eq208_e2608_q_d_n19 * p.p249);
        let eq208_e2610_q_d_n20: f64 = (eq208_e2608_q_d_n20 * p.p249);
        let eq208_e2610_q_d_n21: f64 = (eq208_e2608_q_d_n21 * p.p249);
        let eq208_e2610_q_d_n22: f64 = (eq208_e2608_q_d_n22 * p.p249);
        (eq208_e2610, eq208_e2610_d_n0, eq208_e2610_d_n1, eq208_e2610_d_n2, eq208_e2610_d_n3, eq208_e2610_d_n4, eq208_e2610_d_n5, eq208_e2610_d_n6, eq208_e2610_d_n7, eq208_e2610_d_n8, eq208_e2610_d_n9, eq208_e2610_d_n10, eq208_e2610_d_n11, eq208_e2610_d_n12, eq208_e2610_d_n13, eq208_e2610_d_n14, eq208_e2610_d_n15, eq208_e2610_d_n16, eq208_e2610_d_n17, eq208_e2610_d_n18, eq208_e2610_d_n19, eq208_e2610_d_n20, eq208_e2610_d_n21, eq208_e2610_d_n22, eq208_e2610_q, eq208_e2610_q_d_n0, eq208_e2610_q_d_n1, eq208_e2610_q_d_n2, eq208_e2610_q_d_n3, eq208_e2610_q_d_n4, eq208_e2610_q_d_n5, eq208_e2610_q_d_n6, eq208_e2610_q_d_n7, eq208_e2610_q_d_n8, eq208_e2610_q_d_n9, eq208_e2610_q_d_n10, eq208_e2610_q_d_n11, eq208_e2610_q_d_n12, eq208_e2610_q_d_n13, eq208_e2610_q_d_n14, eq208_e2610_q_d_n15, eq208_e2610_q_d_n16, eq208_e2610_q_d_n17, eq208_e2610_q_d_n18, eq208_e2610_q_d_n19, eq208_e2610_q_d_n20, eq208_e2610_q_d_n21, eq208_e2610_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq208_reactive_node_derivatives: [f64; 23] = [eq208_e2612_q_d_n0, eq208_e2612_q_d_n1, eq208_e2612_q_d_n2, eq208_e2612_q_d_n3, eq208_e2612_q_d_n4, eq208_e2612_q_d_n5, eq208_e2612_q_d_n6, eq208_e2612_q_d_n7, eq208_e2612_q_d_n8, eq208_e2612_q_d_n9, eq208_e2612_q_d_n10, eq208_e2612_q_d_n11, eq208_e2612_q_d_n12, eq208_e2612_q_d_n13, eq208_e2612_q_d_n14, eq208_e2612_q_d_n15, eq208_e2612_q_d_n16, eq208_e2612_q_d_n17, eq208_e2612_q_d_n18, eq208_e2612_q_d_n19, eq208_e2612_q_d_n20, eq208_e2612_q_d_n21, eq208_e2612_q_d_n22];
        let eq208_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[22]),
            nodes,
            &eq208_reactive_node_derivatives,
            branches,
            &eq208_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq209_e2623, eq209_e2623_d_n0, eq209_e2623_d_n1, eq209_e2623_d_n2, eq209_e2623_d_n3, eq209_e2623_d_n4, eq209_e2623_d_n5, eq209_e2623_d_n6, eq209_e2623_d_n7, eq209_e2623_d_n8, eq209_e2623_d_n9, eq209_e2623_d_n10, eq209_e2623_d_n11, eq209_e2623_d_n12, eq209_e2623_d_n13, eq209_e2623_d_n14, eq209_e2623_d_n15, eq209_e2623_d_n16, eq209_e2623_d_n17, eq209_e2623_d_n18, eq209_e2623_d_n19, eq209_e2623_d_n20, eq209_e2623_d_n21, eq209_e2623_d_n22, eq209_e2623_q, eq209_e2623_q_d_n0, eq209_e2623_q_d_n1, eq209_e2623_q_d_n2, eq209_e2623_q_d_n3, eq209_e2623_q_d_n4, eq209_e2623_q_d_n5, eq209_e2623_q_d_n6, eq209_e2623_q_d_n7, eq209_e2623_q_d_n8, eq209_e2623_q_d_n9, eq209_e2623_q_d_n10, eq209_e2623_q_d_n11, eq209_e2623_q_d_n12, eq209_e2623_q_d_n13, eq209_e2623_q_d_n14, eq209_e2623_q_d_n15, eq209_e2623_q_d_n16, eq209_e2623_q_d_n17, eq209_e2623_q_d_n18, eq209_e2623_q_d_n19, eq209_e2623_q_d_n20, eq209_e2623_q_d_n21, eq209_e2623_q_d_n22,) = {
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
        let eq209_e2621_d_n10: f64 = (p.p7 * eq209_e2619_d_n10);
        let eq209_e2621_d_n11: f64 = (p.p7 * eq209_e2619_d_n11);
        let eq209_e2621_d_n12: f64 = (p.p7 * eq209_e2619_d_n12);
        let eq209_e2621_d_n13: f64 = (p.p7 * eq209_e2619_d_n13);
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
        let eq209_e2621_q_d_n0: f64 = (p.p7 * eq209_e2619_d_n0);
        let eq209_e2621_q_d_n1: f64 = (p.p7 * eq209_e2619_d_n1);
        let eq209_e2621_q_d_n2: f64 = (p.p7 * eq209_e2619_d_n2);
        let eq209_e2621_q_d_n3: f64 = (p.p7 * eq209_e2619_d_n3);
        let eq209_e2621_q_d_n4: f64 = (p.p7 * eq209_e2619_d_n4);
        let eq209_e2621_q_d_n5: f64 = (p.p7 * eq209_e2619_d_n5);
        let eq209_e2621_q_d_n6: f64 = (p.p7 * eq209_e2619_d_n6);
        let eq209_e2621_q_d_n7: f64 = (p.p7 * eq209_e2619_d_n7);
        let eq209_e2621_q_d_n8: f64 = (p.p7 * eq209_e2619_d_n8);
        let eq209_e2621_q_d_n9: f64 = (p.p7 * eq209_e2619_d_n9);
        let eq209_e2621_q_d_n10: f64 = (p.p7 * eq209_e2619_d_n10);
        let eq209_e2621_q_d_n11: f64 = (p.p7 * eq209_e2619_d_n11);
        let eq209_e2621_q_d_n12: f64 = (p.p7 * eq209_e2619_d_n12);
        let eq209_e2621_q_d_n13: f64 = (p.p7 * eq209_e2619_d_n13);
        let eq209_e2621_q_d_n14: f64 = (p.p7 * eq209_e2619_d_n14);
        let eq209_e2621_q_d_n15: f64 = (p.p7 * eq209_e2619_d_n15);
        let eq209_e2621_q_d_n16: f64 = (p.p7 * eq209_e2619_d_n16);
        let eq209_e2621_q_d_n17: f64 = (p.p7 * eq209_e2619_d_n17);
        let eq209_e2621_q_d_n18: f64 = (p.p7 * eq209_e2619_d_n18);
        let eq209_e2621_q_d_n19: f64 = (p.p7 * eq209_e2619_d_n19);
        let eq209_e2621_q_d_n20: f64 = (p.p7 * eq209_e2619_d_n20);
        let eq209_e2621_q_d_n21: f64 = (p.p7 * eq209_e2619_d_n21);
        let eq209_e2621_q_d_n22: f64 = (p.p7 * eq209_e2619_d_n22);
        (eq209_e2621, eq209_e2621_d_n0, eq209_e2621_d_n1, eq209_e2621_d_n2, eq209_e2621_d_n3, eq209_e2621_d_n4, eq209_e2621_d_n5, eq209_e2621_d_n6, eq209_e2621_d_n7, eq209_e2621_d_n8, eq209_e2621_d_n9, eq209_e2621_d_n10, eq209_e2621_d_n11, eq209_e2621_d_n12, eq209_e2621_d_n13, eq209_e2621_d_n14, eq209_e2621_d_n15, eq209_e2621_d_n16, eq209_e2621_d_n17, eq209_e2621_d_n18, eq209_e2621_d_n19, eq209_e2621_d_n20, eq209_e2621_d_n21, eq209_e2621_d_n22, eq209_e2621_q, eq209_e2621_q_d_n0, eq209_e2621_q_d_n1, eq209_e2621_q_d_n2, eq209_e2621_q_d_n3, eq209_e2621_q_d_n4, eq209_e2621_q_d_n5, eq209_e2621_q_d_n6, eq209_e2621_q_d_n7, eq209_e2621_q_d_n8, eq209_e2621_q_d_n9, eq209_e2621_q_d_n10, eq209_e2621_q_d_n11, eq209_e2621_q_d_n12, eq209_e2621_q_d_n13, eq209_e2621_q_d_n14, eq209_e2621_q_d_n15, eq209_e2621_q_d_n16, eq209_e2621_q_d_n17, eq209_e2621_q_d_n18, eq209_e2621_q_d_n19, eq209_e2621_q_d_n20, eq209_e2621_q_d_n21, eq209_e2621_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq209_reactive_node_derivatives: [f64; 23] = [eq209_e2623_q_d_n0, eq209_e2623_q_d_n1, eq209_e2623_q_d_n2, eq209_e2623_q_d_n3, eq209_e2623_q_d_n4, eq209_e2623_q_d_n5, eq209_e2623_q_d_n6, eq209_e2623_q_d_n7, eq209_e2623_q_d_n8, eq209_e2623_q_d_n9, eq209_e2623_q_d_n10, eq209_e2623_q_d_n11, eq209_e2623_q_d_n12, eq209_e2623_q_d_n13, eq209_e2623_q_d_n14, eq209_e2623_q_d_n15, eq209_e2623_q_d_n16, eq209_e2623_q_d_n17, eq209_e2623_q_d_n18, eq209_e2623_q_d_n19, eq209_e2623_q_d_n20, eq209_e2623_q_d_n21, eq209_e2623_q_d_n22];
        let eq209_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[22]),
            nodes,
            &eq209_reactive_node_derivatives,
            branches,
            &eq209_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq210_e2633, eq210_e2633_d_n0, eq210_e2633_d_n1, eq210_e2633_d_n2, eq210_e2633_d_n3, eq210_e2633_d_n4, eq210_e2633_d_n5, eq210_e2633_d_n6, eq210_e2633_d_n7, eq210_e2633_d_n8, eq210_e2633_d_n9, eq210_e2633_d_n10, eq210_e2633_d_n11, eq210_e2633_d_n12, eq210_e2633_d_n13, eq210_e2633_d_n14, eq210_e2633_d_n15, eq210_e2633_d_n16, eq210_e2633_d_n17, eq210_e2633_d_n18, eq210_e2633_d_n19, eq210_e2633_d_n20, eq210_e2633_d_n21, eq210_e2633_d_n22, eq210_e2633_q, eq210_e2633_q_d_n0, eq210_e2633_q_d_n1, eq210_e2633_q_d_n2, eq210_e2633_q_d_n3, eq210_e2633_q_d_n4, eq210_e2633_q_d_n5, eq210_e2633_q_d_n6, eq210_e2633_q_d_n7, eq210_e2633_q_d_n8, eq210_e2633_q_d_n9, eq210_e2633_q_d_n10, eq210_e2633_q_d_n11, eq210_e2633_q_d_n12, eq210_e2633_q_d_n13, eq210_e2633_q_d_n14, eq210_e2633_q_d_n15, eq210_e2633_q_d_n16, eq210_e2633_q_d_n17, eq210_e2633_q_d_n18, eq210_e2633_q_d_n19, eq210_e2633_q_d_n20, eq210_e2633_q_d_n21, eq210_e2633_q_d_n22,) = {
    if ((!s.b[605]) && s.b[608]) {
        let eq210_e2630_q: f64 = s.v[313];
        let eq210_e2631: f64 = (p.p7 * s.v[313]);
        let eq210_e2631_d_n0: f64 = (p.p7 * s.dn[313][0]);
        let eq210_e2631_d_n1: f64 = (p.p7 * s.dn[313][1]);
        let eq210_e2631_d_n2: f64 = (p.p7 * s.dn[313][2]);
        let eq210_e2631_d_n3: f64 = (p.p7 * s.dn[313][3]);
        let eq210_e2631_d_n4: f64 = (p.p7 * s.dn[313][4]);
        let eq210_e2631_d_n5: f64 = (p.p7 * s.dn[313][5]);
        let eq210_e2631_d_n6: f64 = (p.p7 * s.dn[313][6]);
        let eq210_e2631_d_n7: f64 = (p.p7 * s.dn[313][7]);
        let eq210_e2631_d_n8: f64 = (p.p7 * s.dn[313][8]);
        let eq210_e2631_d_n9: f64 = (p.p7 * s.dn[313][9]);
        let eq210_e2631_d_n10: f64 = (p.p7 * s.dn[313][10]);
        let eq210_e2631_d_n11: f64 = (p.p7 * s.dn[313][11]);
        let eq210_e2631_d_n12: f64 = (p.p7 * s.dn[313][12]);
        let eq210_e2631_d_n13: f64 = (p.p7 * s.dn[313][13]);
        let eq210_e2631_d_n14: f64 = (p.p7 * s.dn[313][14]);
        let eq210_e2631_d_n15: f64 = (p.p7 * s.dn[313][15]);
        let eq210_e2631_d_n16: f64 = (p.p7 * s.dn[313][16]);
        let eq210_e2631_d_n17: f64 = (p.p7 * s.dn[313][17]);
        let eq210_e2631_d_n18: f64 = (p.p7 * s.dn[313][18]);
        let eq210_e2631_d_n19: f64 = (p.p7 * s.dn[313][19]);
        let eq210_e2631_d_n20: f64 = (p.p7 * s.dn[313][20]);
        let eq210_e2631_d_n21: f64 = (p.p7 * s.dn[313][21]);
        let eq210_e2631_d_n22: f64 = (p.p7 * s.dn[313][22]);
        let eq210_e2631_q: f64 = (p.p7 * eq210_e2630_q);
        let eq210_e2631_q_d_n0: f64 = (p.p7 * s.dn[313][0]);
        let eq210_e2631_q_d_n1: f64 = (p.p7 * s.dn[313][1]);
        let eq210_e2631_q_d_n2: f64 = (p.p7 * s.dn[313][2]);
        let eq210_e2631_q_d_n3: f64 = (p.p7 * s.dn[313][3]);
        let eq210_e2631_q_d_n4: f64 = (p.p7 * s.dn[313][4]);
        let eq210_e2631_q_d_n5: f64 = (p.p7 * s.dn[313][5]);
        let eq210_e2631_q_d_n6: f64 = (p.p7 * s.dn[313][6]);
        let eq210_e2631_q_d_n7: f64 = (p.p7 * s.dn[313][7]);
        let eq210_e2631_q_d_n8: f64 = (p.p7 * s.dn[313][8]);
        let eq210_e2631_q_d_n9: f64 = (p.p7 * s.dn[313][9]);
        let eq210_e2631_q_d_n10: f64 = (p.p7 * s.dn[313][10]);
        let eq210_e2631_q_d_n11: f64 = (p.p7 * s.dn[313][11]);
        let eq210_e2631_q_d_n12: f64 = (p.p7 * s.dn[313][12]);
        let eq210_e2631_q_d_n13: f64 = (p.p7 * s.dn[313][13]);
        let eq210_e2631_q_d_n14: f64 = (p.p7 * s.dn[313][14]);
        let eq210_e2631_q_d_n15: f64 = (p.p7 * s.dn[313][15]);
        let eq210_e2631_q_d_n16: f64 = (p.p7 * s.dn[313][16]);
        let eq210_e2631_q_d_n17: f64 = (p.p7 * s.dn[313][17]);
        let eq210_e2631_q_d_n18: f64 = (p.p7 * s.dn[313][18]);
        let eq210_e2631_q_d_n19: f64 = (p.p7 * s.dn[313][19]);
        let eq210_e2631_q_d_n20: f64 = (p.p7 * s.dn[313][20]);
        let eq210_e2631_q_d_n21: f64 = (p.p7 * s.dn[313][21]);
        let eq210_e2631_q_d_n22: f64 = (p.p7 * s.dn[313][22]);
        (eq210_e2631, eq210_e2631_d_n0, eq210_e2631_d_n1, eq210_e2631_d_n2, eq210_e2631_d_n3, eq210_e2631_d_n4, eq210_e2631_d_n5, eq210_e2631_d_n6, eq210_e2631_d_n7, eq210_e2631_d_n8, eq210_e2631_d_n9, eq210_e2631_d_n10, eq210_e2631_d_n11, eq210_e2631_d_n12, eq210_e2631_d_n13, eq210_e2631_d_n14, eq210_e2631_d_n15, eq210_e2631_d_n16, eq210_e2631_d_n17, eq210_e2631_d_n18, eq210_e2631_d_n19, eq210_e2631_d_n20, eq210_e2631_d_n21, eq210_e2631_d_n22, eq210_e2631_q, eq210_e2631_q_d_n0, eq210_e2631_q_d_n1, eq210_e2631_q_d_n2, eq210_e2631_q_d_n3, eq210_e2631_q_d_n4, eq210_e2631_q_d_n5, eq210_e2631_q_d_n6, eq210_e2631_q_d_n7, eq210_e2631_q_d_n8, eq210_e2631_q_d_n9, eq210_e2631_q_d_n10, eq210_e2631_q_d_n11, eq210_e2631_q_d_n12, eq210_e2631_q_d_n13, eq210_e2631_q_d_n14, eq210_e2631_q_d_n15, eq210_e2631_q_d_n16, eq210_e2631_q_d_n17, eq210_e2631_q_d_n18, eq210_e2631_q_d_n19, eq210_e2631_q_d_n20, eq210_e2631_q_d_n21, eq210_e2631_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq210_reactive_node_derivatives: [f64; 23] = [eq210_e2633_q_d_n0, eq210_e2633_q_d_n1, eq210_e2633_q_d_n2, eq210_e2633_q_d_n3, eq210_e2633_q_d_n4, eq210_e2633_q_d_n5, eq210_e2633_q_d_n6, eq210_e2633_q_d_n7, eq210_e2633_q_d_n8, eq210_e2633_q_d_n9, eq210_e2633_q_d_n10, eq210_e2633_q_d_n11, eq210_e2633_q_d_n12, eq210_e2633_q_d_n13, eq210_e2633_q_d_n14, eq210_e2633_q_d_n15, eq210_e2633_q_d_n16, eq210_e2633_q_d_n17, eq210_e2633_q_d_n18, eq210_e2633_q_d_n19, eq210_e2633_q_d_n20, eq210_e2633_q_d_n21, eq210_e2633_q_d_n22];
        let eq210_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq210_reactive_node_derivatives,
            branches,
            &eq210_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq211_e2645, eq211_e2645_d_n0, eq211_e2645_d_n1, eq211_e2645_d_n2, eq211_e2645_d_n3, eq211_e2645_d_n4, eq211_e2645_d_n5, eq211_e2645_d_n6, eq211_e2645_d_n7, eq211_e2645_d_n8, eq211_e2645_d_n9, eq211_e2645_d_n10, eq211_e2645_d_n11, eq211_e2645_d_n12, eq211_e2645_d_n13, eq211_e2645_d_n14, eq211_e2645_d_n15, eq211_e2645_d_n16, eq211_e2645_d_n17, eq211_e2645_d_n18, eq211_e2645_d_n19, eq211_e2645_d_n20, eq211_e2645_d_n21, eq211_e2645_d_n22, eq211_e2645_q, eq211_e2645_q_d_n0, eq211_e2645_q_d_n1, eq211_e2645_q_d_n2, eq211_e2645_q_d_n3, eq211_e2645_q_d_n4, eq211_e2645_q_d_n5, eq211_e2645_q_d_n6, eq211_e2645_q_d_n7, eq211_e2645_q_d_n8, eq211_e2645_q_d_n9, eq211_e2645_q_d_n10, eq211_e2645_q_d_n11, eq211_e2645_q_d_n12, eq211_e2645_q_d_n13, eq211_e2645_q_d_n14, eq211_e2645_q_d_n15, eq211_e2645_q_d_n16, eq211_e2645_q_d_n17, eq211_e2645_q_d_n18, eq211_e2645_q_d_n19, eq211_e2645_q_d_n20, eq211_e2645_q_d_n21, eq211_e2645_q_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && s.b[609]) {
        let eq211_e2642_q: f64 = s.v[312];
        let eq211_e2643: f64 = (p.p7 * s.v[312]);
        let eq211_e2643_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq211_e2643_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq211_e2643_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq211_e2643_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq211_e2643_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq211_e2643_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq211_e2643_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq211_e2643_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq211_e2643_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq211_e2643_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq211_e2643_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq211_e2643_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq211_e2643_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq211_e2643_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq211_e2643_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq211_e2643_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq211_e2643_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq211_e2643_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq211_e2643_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq211_e2643_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq211_e2643_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq211_e2643_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq211_e2643_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq211_e2643_q: f64 = (p.p7 * eq211_e2642_q);
        let eq211_e2643_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq211_e2643_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq211_e2643_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq211_e2643_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq211_e2643_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq211_e2643_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq211_e2643_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq211_e2643_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq211_e2643_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq211_e2643_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq211_e2643_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq211_e2643_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq211_e2643_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq211_e2643_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq211_e2643_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq211_e2643_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq211_e2643_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq211_e2643_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq211_e2643_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq211_e2643_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq211_e2643_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq211_e2643_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq211_e2643_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
        (eq211_e2643, eq211_e2643_d_n0, eq211_e2643_d_n1, eq211_e2643_d_n2, eq211_e2643_d_n3, eq211_e2643_d_n4, eq211_e2643_d_n5, eq211_e2643_d_n6, eq211_e2643_d_n7, eq211_e2643_d_n8, eq211_e2643_d_n9, eq211_e2643_d_n10, eq211_e2643_d_n11, eq211_e2643_d_n12, eq211_e2643_d_n13, eq211_e2643_d_n14, eq211_e2643_d_n15, eq211_e2643_d_n16, eq211_e2643_d_n17, eq211_e2643_d_n18, eq211_e2643_d_n19, eq211_e2643_d_n20, eq211_e2643_d_n21, eq211_e2643_d_n22, eq211_e2643_q, eq211_e2643_q_d_n0, eq211_e2643_q_d_n1, eq211_e2643_q_d_n2, eq211_e2643_q_d_n3, eq211_e2643_q_d_n4, eq211_e2643_q_d_n5, eq211_e2643_q_d_n6, eq211_e2643_q_d_n7, eq211_e2643_q_d_n8, eq211_e2643_q_d_n9, eq211_e2643_q_d_n10, eq211_e2643_q_d_n11, eq211_e2643_q_d_n12, eq211_e2643_q_d_n13, eq211_e2643_q_d_n14, eq211_e2643_q_d_n15, eq211_e2643_q_d_n16, eq211_e2643_q_d_n17, eq211_e2643_q_d_n18, eq211_e2643_q_d_n19, eq211_e2643_q_d_n20, eq211_e2643_q_d_n21, eq211_e2643_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq211_reactive_node_derivatives: [f64; 23] = [eq211_e2645_q_d_n0, eq211_e2645_q_d_n1, eq211_e2645_q_d_n2, eq211_e2645_q_d_n3, eq211_e2645_q_d_n4, eq211_e2645_q_d_n5, eq211_e2645_q_d_n6, eq211_e2645_q_d_n7, eq211_e2645_q_d_n8, eq211_e2645_q_d_n9, eq211_e2645_q_d_n10, eq211_e2645_q_d_n11, eq211_e2645_q_d_n12, eq211_e2645_q_d_n13, eq211_e2645_q_d_n14, eq211_e2645_q_d_n15, eq211_e2645_q_d_n16, eq211_e2645_q_d_n17, eq211_e2645_q_d_n18, eq211_e2645_q_d_n19, eq211_e2645_q_d_n20, eq211_e2645_q_d_n21, eq211_e2645_q_d_n22];
        let eq211_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq211_reactive_node_derivatives,
            branches,
            &eq211_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq212_e2659, eq212_e2659_d_n0, eq212_e2659_d_n1, eq212_e2659_d_n2, eq212_e2659_d_n3, eq212_e2659_d_n4, eq212_e2659_d_n5, eq212_e2659_d_n6, eq212_e2659_d_n7, eq212_e2659_d_n8, eq212_e2659_d_n9, eq212_e2659_d_n10, eq212_e2659_d_n11, eq212_e2659_d_n12, eq212_e2659_d_n13, eq212_e2659_d_n14, eq212_e2659_d_n15, eq212_e2659_d_n16, eq212_e2659_d_n17, eq212_e2659_d_n18, eq212_e2659_d_n19, eq212_e2659_d_n20, eq212_e2659_d_n21, eq212_e2659_d_n22, eq212_e2659_q, eq212_e2659_q_d_n0, eq212_e2659_q_d_n1, eq212_e2659_q_d_n2, eq212_e2659_q_d_n3, eq212_e2659_q_d_n4, eq212_e2659_q_d_n5, eq212_e2659_q_d_n6, eq212_e2659_q_d_n7, eq212_e2659_q_d_n8, eq212_e2659_q_d_n9, eq212_e2659_q_d_n10, eq212_e2659_q_d_n11, eq212_e2659_q_d_n12, eq212_e2659_q_d_n13, eq212_e2659_q_d_n14, eq212_e2659_q_d_n15, eq212_e2659_q_d_n16, eq212_e2659_q_d_n17, eq212_e2659_q_d_n18, eq212_e2659_q_d_n19, eq212_e2659_q_d_n20, eq212_e2659_q_d_n21, eq212_e2659_q_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && s.b[609]) {
        let eq212_e2654_q: f64 = s.v[312];
        let eq212_e2655: f64 = (p.p7 * s.v[312]);
        let eq212_e2655_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq212_e2655_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq212_e2655_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq212_e2655_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq212_e2655_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq212_e2655_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq212_e2655_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq212_e2655_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq212_e2655_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq212_e2655_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq212_e2655_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq212_e2655_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq212_e2655_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq212_e2655_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq212_e2655_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq212_e2655_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq212_e2655_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq212_e2655_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq212_e2655_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq212_e2655_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq212_e2655_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq212_e2655_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq212_e2655_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq212_e2655_q: f64 = (p.p7 * eq212_e2654_q);
        let eq212_e2655_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq212_e2655_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq212_e2655_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq212_e2655_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq212_e2655_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq212_e2655_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq212_e2655_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq212_e2655_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq212_e2655_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq212_e2655_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq212_e2655_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq212_e2655_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq212_e2655_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq212_e2655_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq212_e2655_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq212_e2655_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq212_e2655_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq212_e2655_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq212_e2655_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq212_e2655_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq212_e2655_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq212_e2655_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq212_e2655_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
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
        let eq212_e2657_q: f64 = (eq212_e2655_q * p.p249);
        let eq212_e2657_q_d_n0: f64 = (eq212_e2655_q_d_n0 * p.p249);
        let eq212_e2657_q_d_n1: f64 = (eq212_e2655_q_d_n1 * p.p249);
        let eq212_e2657_q_d_n2: f64 = (eq212_e2655_q_d_n2 * p.p249);
        let eq212_e2657_q_d_n3: f64 = (eq212_e2655_q_d_n3 * p.p249);
        let eq212_e2657_q_d_n4: f64 = (eq212_e2655_q_d_n4 * p.p249);
        let eq212_e2657_q_d_n5: f64 = (eq212_e2655_q_d_n5 * p.p249);
        let eq212_e2657_q_d_n6: f64 = (eq212_e2655_q_d_n6 * p.p249);
        let eq212_e2657_q_d_n7: f64 = (eq212_e2655_q_d_n7 * p.p249);
        let eq212_e2657_q_d_n8: f64 = (eq212_e2655_q_d_n8 * p.p249);
        let eq212_e2657_q_d_n9: f64 = (eq212_e2655_q_d_n9 * p.p249);
        let eq212_e2657_q_d_n10: f64 = (eq212_e2655_q_d_n10 * p.p249);
        let eq212_e2657_q_d_n11: f64 = (eq212_e2655_q_d_n11 * p.p249);
        let eq212_e2657_q_d_n12: f64 = (eq212_e2655_q_d_n12 * p.p249);
        let eq212_e2657_q_d_n13: f64 = (eq212_e2655_q_d_n13 * p.p249);
        let eq212_e2657_q_d_n14: f64 = (eq212_e2655_q_d_n14 * p.p249);
        let eq212_e2657_q_d_n15: f64 = (eq212_e2655_q_d_n15 * p.p249);
        let eq212_e2657_q_d_n16: f64 = (eq212_e2655_q_d_n16 * p.p249);
        let eq212_e2657_q_d_n17: f64 = (eq212_e2655_q_d_n17 * p.p249);
        let eq212_e2657_q_d_n18: f64 = (eq212_e2655_q_d_n18 * p.p249);
        let eq212_e2657_q_d_n19: f64 = (eq212_e2655_q_d_n19 * p.p249);
        let eq212_e2657_q_d_n20: f64 = (eq212_e2655_q_d_n20 * p.p249);
        let eq212_e2657_q_d_n21: f64 = (eq212_e2655_q_d_n21 * p.p249);
        let eq212_e2657_q_d_n22: f64 = (eq212_e2655_q_d_n22 * p.p249);
        (eq212_e2657, eq212_e2657_d_n0, eq212_e2657_d_n1, eq212_e2657_d_n2, eq212_e2657_d_n3, eq212_e2657_d_n4, eq212_e2657_d_n5, eq212_e2657_d_n6, eq212_e2657_d_n7, eq212_e2657_d_n8, eq212_e2657_d_n9, eq212_e2657_d_n10, eq212_e2657_d_n11, eq212_e2657_d_n12, eq212_e2657_d_n13, eq212_e2657_d_n14, eq212_e2657_d_n15, eq212_e2657_d_n16, eq212_e2657_d_n17, eq212_e2657_d_n18, eq212_e2657_d_n19, eq212_e2657_d_n20, eq212_e2657_d_n21, eq212_e2657_d_n22, eq212_e2657_q, eq212_e2657_q_d_n0, eq212_e2657_q_d_n1, eq212_e2657_q_d_n2, eq212_e2657_q_d_n3, eq212_e2657_q_d_n4, eq212_e2657_q_d_n5, eq212_e2657_q_d_n6, eq212_e2657_q_d_n7, eq212_e2657_q_d_n8, eq212_e2657_q_d_n9, eq212_e2657_q_d_n10, eq212_e2657_q_d_n11, eq212_e2657_q_d_n12, eq212_e2657_q_d_n13, eq212_e2657_q_d_n14, eq212_e2657_q_d_n15, eq212_e2657_q_d_n16, eq212_e2657_q_d_n17, eq212_e2657_q_d_n18, eq212_e2657_q_d_n19, eq212_e2657_q_d_n20, eq212_e2657_q_d_n21, eq212_e2657_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq212_reactive_node_derivatives: [f64; 23] = [eq212_e2659_q_d_n0, eq212_e2659_q_d_n1, eq212_e2659_q_d_n2, eq212_e2659_q_d_n3, eq212_e2659_q_d_n4, eq212_e2659_q_d_n5, eq212_e2659_q_d_n6, eq212_e2659_q_d_n7, eq212_e2659_q_d_n8, eq212_e2659_q_d_n9, eq212_e2659_q_d_n10, eq212_e2659_q_d_n11, eq212_e2659_q_d_n12, eq212_e2659_q_d_n13, eq212_e2659_q_d_n14, eq212_e2659_q_d_n15, eq212_e2659_q_d_n16, eq212_e2659_q_d_n17, eq212_e2659_q_d_n18, eq212_e2659_q_d_n19, eq212_e2659_q_d_n20, eq212_e2659_q_d_n21, eq212_e2659_q_d_n22];
        let eq212_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq212_reactive_node_derivatives,
            branches,
            &eq212_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_20(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq213_e2672, eq213_e2672_d_n0, eq213_e2672_d_n1, eq213_e2672_d_n2, eq213_e2672_d_n3, eq213_e2672_d_n4, eq213_e2672_d_n5, eq213_e2672_d_n6, eq213_e2672_d_n7, eq213_e2672_d_n8, eq213_e2672_d_n9, eq213_e2672_d_n10, eq213_e2672_d_n11, eq213_e2672_d_n12, eq213_e2672_d_n13, eq213_e2672_d_n14, eq213_e2672_d_n15, eq213_e2672_d_n16, eq213_e2672_d_n17, eq213_e2672_d_n18, eq213_e2672_d_n19, eq213_e2672_d_n20, eq213_e2672_d_n21, eq213_e2672_d_n22, eq213_e2672_q, eq213_e2672_q_d_n0, eq213_e2672_q_d_n1, eq213_e2672_q_d_n2, eq213_e2672_q_d_n3, eq213_e2672_q_d_n4, eq213_e2672_q_d_n5, eq213_e2672_q_d_n6, eq213_e2672_q_d_n7, eq213_e2672_q_d_n8, eq213_e2672_q_d_n9, eq213_e2672_q_d_n10, eq213_e2672_q_d_n11, eq213_e2672_q_d_n12, eq213_e2672_q_d_n13, eq213_e2672_q_d_n14, eq213_e2672_q_d_n15, eq213_e2672_q_d_n16, eq213_e2672_q_d_n17, eq213_e2672_q_d_n18, eq213_e2672_q_d_n19, eq213_e2672_q_d_n20, eq213_e2672_q_d_n21, eq213_e2672_q_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq213_e2669_q: f64 = s.v[312];
        let eq213_e2670: f64 = (p.p7 * s.v[312]);
        let eq213_e2670_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq213_e2670_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq213_e2670_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq213_e2670_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq213_e2670_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq213_e2670_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq213_e2670_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq213_e2670_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq213_e2670_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq213_e2670_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq213_e2670_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq213_e2670_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq213_e2670_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq213_e2670_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq213_e2670_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq213_e2670_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq213_e2670_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq213_e2670_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq213_e2670_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq213_e2670_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq213_e2670_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq213_e2670_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq213_e2670_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq213_e2670_q: f64 = (p.p7 * eq213_e2669_q);
        let eq213_e2670_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq213_e2670_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq213_e2670_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq213_e2670_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq213_e2670_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq213_e2670_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq213_e2670_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq213_e2670_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq213_e2670_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq213_e2670_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq213_e2670_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq213_e2670_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq213_e2670_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq213_e2670_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq213_e2670_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq213_e2670_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq213_e2670_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq213_e2670_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq213_e2670_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq213_e2670_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq213_e2670_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq213_e2670_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq213_e2670_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
        (eq213_e2670, eq213_e2670_d_n0, eq213_e2670_d_n1, eq213_e2670_d_n2, eq213_e2670_d_n3, eq213_e2670_d_n4, eq213_e2670_d_n5, eq213_e2670_d_n6, eq213_e2670_d_n7, eq213_e2670_d_n8, eq213_e2670_d_n9, eq213_e2670_d_n10, eq213_e2670_d_n11, eq213_e2670_d_n12, eq213_e2670_d_n13, eq213_e2670_d_n14, eq213_e2670_d_n15, eq213_e2670_d_n16, eq213_e2670_d_n17, eq213_e2670_d_n18, eq213_e2670_d_n19, eq213_e2670_d_n20, eq213_e2670_d_n21, eq213_e2670_d_n22, eq213_e2670_q, eq213_e2670_q_d_n0, eq213_e2670_q_d_n1, eq213_e2670_q_d_n2, eq213_e2670_q_d_n3, eq213_e2670_q_d_n4, eq213_e2670_q_d_n5, eq213_e2670_q_d_n6, eq213_e2670_q_d_n7, eq213_e2670_q_d_n8, eq213_e2670_q_d_n9, eq213_e2670_q_d_n10, eq213_e2670_q_d_n11, eq213_e2670_q_d_n12, eq213_e2670_q_d_n13, eq213_e2670_q_d_n14, eq213_e2670_q_d_n15, eq213_e2670_q_d_n16, eq213_e2670_q_d_n17, eq213_e2670_q_d_n18, eq213_e2670_q_d_n19, eq213_e2670_q_d_n20, eq213_e2670_q_d_n21, eq213_e2670_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq213_reactive_node_derivatives: [f64; 23] = [eq213_e2672_q_d_n0, eq213_e2672_q_d_n1, eq213_e2672_q_d_n2, eq213_e2672_q_d_n3, eq213_e2672_q_d_n4, eq213_e2672_q_d_n5, eq213_e2672_q_d_n6, eq213_e2672_q_d_n7, eq213_e2672_q_d_n8, eq213_e2672_q_d_n9, eq213_e2672_q_d_n10, eq213_e2672_q_d_n11, eq213_e2672_q_d_n12, eq213_e2672_q_d_n13, eq213_e2672_q_d_n14, eq213_e2672_q_d_n15, eq213_e2672_q_d_n16, eq213_e2672_q_d_n17, eq213_e2672_q_d_n18, eq213_e2672_q_d_n19, eq213_e2672_q_d_n20, eq213_e2672_q_d_n21, eq213_e2672_q_d_n22];
        let eq213_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq213_reactive_node_derivatives,
            branches,
            &eq213_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22, eq214_e2687_q, eq214_e2687_q_d_n0, eq214_e2687_q_d_n1, eq214_e2687_q_d_n2, eq214_e2687_q_d_n3, eq214_e2687_q_d_n4, eq214_e2687_q_d_n5, eq214_e2687_q_d_n6, eq214_e2687_q_d_n7, eq214_e2687_q_d_n8, eq214_e2687_q_d_n9, eq214_e2687_q_d_n10, eq214_e2687_q_d_n11, eq214_e2687_q_d_n12, eq214_e2687_q_d_n13, eq214_e2687_q_d_n14, eq214_e2687_q_d_n15, eq214_e2687_q_d_n16, eq214_e2687_q_d_n17, eq214_e2687_q_d_n18, eq214_e2687_q_d_n19, eq214_e2687_q_d_n20, eq214_e2687_q_d_n21, eq214_e2687_q_d_n22,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq214_e2682_q: f64 = s.v[312];
        let eq214_e2683: f64 = (p.p7 * s.v[312]);
        let eq214_e2683_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq214_e2683_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq214_e2683_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq214_e2683_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq214_e2683_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq214_e2683_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq214_e2683_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq214_e2683_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq214_e2683_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq214_e2683_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq214_e2683_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq214_e2683_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq214_e2683_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq214_e2683_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq214_e2683_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq214_e2683_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq214_e2683_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq214_e2683_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq214_e2683_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq214_e2683_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq214_e2683_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq214_e2683_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq214_e2683_d_n22: f64 = (p.p7 * s.dn[312][22]);
        let eq214_e2683_q: f64 = (p.p7 * eq214_e2682_q);
        let eq214_e2683_q_d_n0: f64 = (p.p7 * s.dn[312][0]);
        let eq214_e2683_q_d_n1: f64 = (p.p7 * s.dn[312][1]);
        let eq214_e2683_q_d_n2: f64 = (p.p7 * s.dn[312][2]);
        let eq214_e2683_q_d_n3: f64 = (p.p7 * s.dn[312][3]);
        let eq214_e2683_q_d_n4: f64 = (p.p7 * s.dn[312][4]);
        let eq214_e2683_q_d_n5: f64 = (p.p7 * s.dn[312][5]);
        let eq214_e2683_q_d_n6: f64 = (p.p7 * s.dn[312][6]);
        let eq214_e2683_q_d_n7: f64 = (p.p7 * s.dn[312][7]);
        let eq214_e2683_q_d_n8: f64 = (p.p7 * s.dn[312][8]);
        let eq214_e2683_q_d_n9: f64 = (p.p7 * s.dn[312][9]);
        let eq214_e2683_q_d_n10: f64 = (p.p7 * s.dn[312][10]);
        let eq214_e2683_q_d_n11: f64 = (p.p7 * s.dn[312][11]);
        let eq214_e2683_q_d_n12: f64 = (p.p7 * s.dn[312][12]);
        let eq214_e2683_q_d_n13: f64 = (p.p7 * s.dn[312][13]);
        let eq214_e2683_q_d_n14: f64 = (p.p7 * s.dn[312][14]);
        let eq214_e2683_q_d_n15: f64 = (p.p7 * s.dn[312][15]);
        let eq214_e2683_q_d_n16: f64 = (p.p7 * s.dn[312][16]);
        let eq214_e2683_q_d_n17: f64 = (p.p7 * s.dn[312][17]);
        let eq214_e2683_q_d_n18: f64 = (p.p7 * s.dn[312][18]);
        let eq214_e2683_q_d_n19: f64 = (p.p7 * s.dn[312][19]);
        let eq214_e2683_q_d_n20: f64 = (p.p7 * s.dn[312][20]);
        let eq214_e2683_q_d_n21: f64 = (p.p7 * s.dn[312][21]);
        let eq214_e2683_q_d_n22: f64 = (p.p7 * s.dn[312][22]);
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
        let eq214_e2685_q: f64 = (eq214_e2683_q * p.p249);
        let eq214_e2685_q_d_n0: f64 = (eq214_e2683_q_d_n0 * p.p249);
        let eq214_e2685_q_d_n1: f64 = (eq214_e2683_q_d_n1 * p.p249);
        let eq214_e2685_q_d_n2: f64 = (eq214_e2683_q_d_n2 * p.p249);
        let eq214_e2685_q_d_n3: f64 = (eq214_e2683_q_d_n3 * p.p249);
        let eq214_e2685_q_d_n4: f64 = (eq214_e2683_q_d_n4 * p.p249);
        let eq214_e2685_q_d_n5: f64 = (eq214_e2683_q_d_n5 * p.p249);
        let eq214_e2685_q_d_n6: f64 = (eq214_e2683_q_d_n6 * p.p249);
        let eq214_e2685_q_d_n7: f64 = (eq214_e2683_q_d_n7 * p.p249);
        let eq214_e2685_q_d_n8: f64 = (eq214_e2683_q_d_n8 * p.p249);
        let eq214_e2685_q_d_n9: f64 = (eq214_e2683_q_d_n9 * p.p249);
        let eq214_e2685_q_d_n10: f64 = (eq214_e2683_q_d_n10 * p.p249);
        let eq214_e2685_q_d_n11: f64 = (eq214_e2683_q_d_n11 * p.p249);
        let eq214_e2685_q_d_n12: f64 = (eq214_e2683_q_d_n12 * p.p249);
        let eq214_e2685_q_d_n13: f64 = (eq214_e2683_q_d_n13 * p.p249);
        let eq214_e2685_q_d_n14: f64 = (eq214_e2683_q_d_n14 * p.p249);
        let eq214_e2685_q_d_n15: f64 = (eq214_e2683_q_d_n15 * p.p249);
        let eq214_e2685_q_d_n16: f64 = (eq214_e2683_q_d_n16 * p.p249);
        let eq214_e2685_q_d_n17: f64 = (eq214_e2683_q_d_n17 * p.p249);
        let eq214_e2685_q_d_n18: f64 = (eq214_e2683_q_d_n18 * p.p249);
        let eq214_e2685_q_d_n19: f64 = (eq214_e2683_q_d_n19 * p.p249);
        let eq214_e2685_q_d_n20: f64 = (eq214_e2683_q_d_n20 * p.p249);
        let eq214_e2685_q_d_n21: f64 = (eq214_e2683_q_d_n21 * p.p249);
        let eq214_e2685_q_d_n22: f64 = (eq214_e2683_q_d_n22 * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n10, eq214_e2685_d_n11, eq214_e2685_d_n12, eq214_e2685_d_n13, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22, eq214_e2685_q, eq214_e2685_q_d_n0, eq214_e2685_q_d_n1, eq214_e2685_q_d_n2, eq214_e2685_q_d_n3, eq214_e2685_q_d_n4, eq214_e2685_q_d_n5, eq214_e2685_q_d_n6, eq214_e2685_q_d_n7, eq214_e2685_q_d_n8, eq214_e2685_q_d_n9, eq214_e2685_q_d_n10, eq214_e2685_q_d_n11, eq214_e2685_q_d_n12, eq214_e2685_q_d_n13, eq214_e2685_q_d_n14, eq214_e2685_q_d_n15, eq214_e2685_q_d_n16, eq214_e2685_q_d_n17, eq214_e2685_q_d_n18, eq214_e2685_q_d_n19, eq214_e2685_q_d_n20, eq214_e2685_q_d_n21, eq214_e2685_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_reactive_node_derivatives: [f64; 23] = [eq214_e2687_q_d_n0, eq214_e2687_q_d_n1, eq214_e2687_q_d_n2, eq214_e2687_q_d_n3, eq214_e2687_q_d_n4, eq214_e2687_q_d_n5, eq214_e2687_q_d_n6, eq214_e2687_q_d_n7, eq214_e2687_q_d_n8, eq214_e2687_q_d_n9, eq214_e2687_q_d_n10, eq214_e2687_q_d_n11, eq214_e2687_q_d_n12, eq214_e2687_q_d_n13, eq214_e2687_q_d_n14, eq214_e2687_q_d_n15, eq214_e2687_q_d_n16, eq214_e2687_q_d_n17, eq214_e2687_q_d_n18, eq214_e2687_q_d_n19, eq214_e2687_q_d_n20, eq214_e2687_q_d_n21, eq214_e2687_q_d_n22];
        let eq214_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq214_reactive_node_derivatives,
            branches,
            &eq214_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22, eq215_e2699_q, eq215_e2699_q_d_n0, eq215_e2699_q_d_n1, eq215_e2699_q_d_n2, eq215_e2699_q_d_n3, eq215_e2699_q_d_n4, eq215_e2699_q_d_n5, eq215_e2699_q_d_n6, eq215_e2699_q_d_n7, eq215_e2699_q_d_n8, eq215_e2699_q_d_n9, eq215_e2699_q_d_n10, eq215_e2699_q_d_n11, eq215_e2699_q_d_n12, eq215_e2699_q_d_n13, eq215_e2699_q_d_n14, eq215_e2699_q_d_n15, eq215_e2699_q_d_n16, eq215_e2699_q_d_n17, eq215_e2699_q_d_n18, eq215_e2699_q_d_n19, eq215_e2699_q_d_n20, eq215_e2699_q_d_n21, eq215_e2699_q_d_n22,) = {
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
        let eq215_e2697_d_n10: f64 = (p.p7 * eq215_e2695_d_n10);
        let eq215_e2697_d_n11: f64 = (p.p7 * eq215_e2695_d_n11);
        let eq215_e2697_d_n12: f64 = (p.p7 * eq215_e2695_d_n12);
        let eq215_e2697_d_n13: f64 = (p.p7 * eq215_e2695_d_n13);
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
        let eq215_e2697_q_d_n0: f64 = (p.p7 * eq215_e2695_d_n0);
        let eq215_e2697_q_d_n1: f64 = (p.p7 * eq215_e2695_d_n1);
        let eq215_e2697_q_d_n2: f64 = (p.p7 * eq215_e2695_d_n2);
        let eq215_e2697_q_d_n3: f64 = (p.p7 * eq215_e2695_d_n3);
        let eq215_e2697_q_d_n4: f64 = (p.p7 * eq215_e2695_d_n4);
        let eq215_e2697_q_d_n5: f64 = (p.p7 * eq215_e2695_d_n5);
        let eq215_e2697_q_d_n6: f64 = (p.p7 * eq215_e2695_d_n6);
        let eq215_e2697_q_d_n7: f64 = (p.p7 * eq215_e2695_d_n7);
        let eq215_e2697_q_d_n8: f64 = (p.p7 * eq215_e2695_d_n8);
        let eq215_e2697_q_d_n9: f64 = (p.p7 * eq215_e2695_d_n9);
        let eq215_e2697_q_d_n10: f64 = (p.p7 * eq215_e2695_d_n10);
        let eq215_e2697_q_d_n11: f64 = (p.p7 * eq215_e2695_d_n11);
        let eq215_e2697_q_d_n12: f64 = (p.p7 * eq215_e2695_d_n12);
        let eq215_e2697_q_d_n13: f64 = (p.p7 * eq215_e2695_d_n13);
        let eq215_e2697_q_d_n14: f64 = (p.p7 * eq215_e2695_d_n14);
        let eq215_e2697_q_d_n15: f64 = (p.p7 * eq215_e2695_d_n15);
        let eq215_e2697_q_d_n16: f64 = (p.p7 * eq215_e2695_d_n16);
        let eq215_e2697_q_d_n17: f64 = (p.p7 * eq215_e2695_d_n17);
        let eq215_e2697_q_d_n18: f64 = (p.p7 * eq215_e2695_d_n18);
        let eq215_e2697_q_d_n19: f64 = (p.p7 * eq215_e2695_d_n19);
        let eq215_e2697_q_d_n20: f64 = (p.p7 * eq215_e2695_d_n20);
        let eq215_e2697_q_d_n21: f64 = (p.p7 * eq215_e2695_d_n21);
        let eq215_e2697_q_d_n22: f64 = (p.p7 * eq215_e2695_d_n22);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n10, eq215_e2697_d_n11, eq215_e2697_d_n12, eq215_e2697_d_n13, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22, eq215_e2697_q, eq215_e2697_q_d_n0, eq215_e2697_q_d_n1, eq215_e2697_q_d_n2, eq215_e2697_q_d_n3, eq215_e2697_q_d_n4, eq215_e2697_q_d_n5, eq215_e2697_q_d_n6, eq215_e2697_q_d_n7, eq215_e2697_q_d_n8, eq215_e2697_q_d_n9, eq215_e2697_q_d_n10, eq215_e2697_q_d_n11, eq215_e2697_q_d_n12, eq215_e2697_q_d_n13, eq215_e2697_q_d_n14, eq215_e2697_q_d_n15, eq215_e2697_q_d_n16, eq215_e2697_q_d_n17, eq215_e2697_q_d_n18, eq215_e2697_q_d_n19, eq215_e2697_q_d_n20, eq215_e2697_q_d_n21, eq215_e2697_q_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_reactive_node_derivatives: [f64; 23] = [eq215_e2699_q_d_n0, eq215_e2699_q_d_n1, eq215_e2699_q_d_n2, eq215_e2699_q_d_n3, eq215_e2699_q_d_n4, eq215_e2699_q_d_n5, eq215_e2699_q_d_n6, eq215_e2699_q_d_n7, eq215_e2699_q_d_n8, eq215_e2699_q_d_n9, eq215_e2699_q_d_n10, eq215_e2699_q_d_n11, eq215_e2699_q_d_n12, eq215_e2699_q_d_n13, eq215_e2699_q_d_n14, eq215_e2699_q_d_n15, eq215_e2699_q_d_n16, eq215_e2699_q_d_n17, eq215_e2699_q_d_n18, eq215_e2699_q_d_n19, eq215_e2699_q_d_n20, eq215_e2699_q_d_n21, eq215_e2699_q_d_n22];
        let eq215_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq215_reactive_node_derivatives,
            branches,
            &eq215_reactive_branch_derivatives,
            multiplicity,
        );
        let eq216_e2702_q: f64 = s.v[195];
        let eq216_e2703: f64 = (p.p7 * s.v[195]);
        let eq216_e2703_d_n0: f64 = (p.p7 * s.dn[195][0]);
        let eq216_e2703_d_n1: f64 = (p.p7 * s.dn[195][1]);
        let eq216_e2703_d_n2: f64 = (p.p7 * s.dn[195][2]);
        let eq216_e2703_d_n3: f64 = (p.p7 * s.dn[195][3]);
        let eq216_e2703_d_n4: f64 = (p.p7 * s.dn[195][4]);
        let eq216_e2703_d_n5: f64 = (p.p7 * s.dn[195][5]);
        let eq216_e2703_d_n6: f64 = (p.p7 * s.dn[195][6]);
        let eq216_e2703_d_n7: f64 = (p.p7 * s.dn[195][7]);
        let eq216_e2703_d_n8: f64 = (p.p7 * s.dn[195][8]);
        let eq216_e2703_d_n9: f64 = (p.p7 * s.dn[195][9]);
        let eq216_e2703_d_n10: f64 = (p.p7 * s.dn[195][10]);
        let eq216_e2703_d_n11: f64 = (p.p7 * s.dn[195][11]);
        let eq216_e2703_d_n12: f64 = (p.p7 * s.dn[195][12]);
        let eq216_e2703_d_n13: f64 = (p.p7 * s.dn[195][13]);
        let eq216_e2703_d_n14: f64 = (p.p7 * s.dn[195][14]);
        let eq216_e2703_d_n15: f64 = (p.p7 * s.dn[195][15]);
        let eq216_e2703_d_n16: f64 = (p.p7 * s.dn[195][16]);
        let eq216_e2703_d_n17: f64 = (p.p7 * s.dn[195][17]);
        let eq216_e2703_d_n18: f64 = (p.p7 * s.dn[195][18]);
        let eq216_e2703_d_n19: f64 = (p.p7 * s.dn[195][19]);
        let eq216_e2703_d_n20: f64 = (p.p7 * s.dn[195][20]);
        let eq216_e2703_d_n21: f64 = (p.p7 * s.dn[195][21]);
        let eq216_e2703_d_n22: f64 = (p.p7 * s.dn[195][22]);
        let eq216_e2703_q: f64 = (p.p7 * eq216_e2702_q);
        let eq216_e2703_q_d_n0: f64 = (p.p7 * s.dn[195][0]);
        let eq216_e2703_q_d_n1: f64 = (p.p7 * s.dn[195][1]);
        let eq216_e2703_q_d_n2: f64 = (p.p7 * s.dn[195][2]);
        let eq216_e2703_q_d_n3: f64 = (p.p7 * s.dn[195][3]);
        let eq216_e2703_q_d_n4: f64 = (p.p7 * s.dn[195][4]);
        let eq216_e2703_q_d_n5: f64 = (p.p7 * s.dn[195][5]);
        let eq216_e2703_q_d_n6: f64 = (p.p7 * s.dn[195][6]);
        let eq216_e2703_q_d_n7: f64 = (p.p7 * s.dn[195][7]);
        let eq216_e2703_q_d_n8: f64 = (p.p7 * s.dn[195][8]);
        let eq216_e2703_q_d_n9: f64 = (p.p7 * s.dn[195][9]);
        let eq216_e2703_q_d_n10: f64 = (p.p7 * s.dn[195][10]);
        let eq216_e2703_q_d_n11: f64 = (p.p7 * s.dn[195][11]);
        let eq216_e2703_q_d_n12: f64 = (p.p7 * s.dn[195][12]);
        let eq216_e2703_q_d_n13: f64 = (p.p7 * s.dn[195][13]);
        let eq216_e2703_q_d_n14: f64 = (p.p7 * s.dn[195][14]);
        let eq216_e2703_q_d_n15: f64 = (p.p7 * s.dn[195][15]);
        let eq216_e2703_q_d_n16: f64 = (p.p7 * s.dn[195][16]);
        let eq216_e2703_q_d_n17: f64 = (p.p7 * s.dn[195][17]);
        let eq216_e2703_q_d_n18: f64 = (p.p7 * s.dn[195][18]);
        let eq216_e2703_q_d_n19: f64 = (p.p7 * s.dn[195][19]);
        let eq216_e2703_q_d_n20: f64 = (p.p7 * s.dn[195][20]);
        let eq216_e2703_q_d_n21: f64 = (p.p7 * s.dn[195][21]);
        let eq216_e2703_q_d_n22: f64 = (p.p7 * s.dn[195][22]);
        let eq216_reactive_node_derivatives: [f64; 23] = [eq216_e2703_q_d_n0, eq216_e2703_q_d_n1, eq216_e2703_q_d_n2, eq216_e2703_q_d_n3, eq216_e2703_q_d_n4, eq216_e2703_q_d_n5, eq216_e2703_q_d_n6, eq216_e2703_q_d_n7, eq216_e2703_q_d_n8, eq216_e2703_q_d_n9, eq216_e2703_q_d_n10, eq216_e2703_q_d_n11, eq216_e2703_q_d_n12, eq216_e2703_q_d_n13, eq216_e2703_q_d_n14, eq216_e2703_q_d_n15, eq216_e2703_q_d_n16, eq216_e2703_q_d_n17, eq216_e2703_q_d_n18, eq216_e2703_q_d_n19, eq216_e2703_q_d_n20, eq216_e2703_q_d_n21, eq216_e2703_q_d_n22];
        let eq216_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq216_reactive_node_derivatives,
            branches,
            &eq216_reactive_branch_derivatives,
            multiplicity,
        );
        let eq217_e2707: f64 = (p.p4 * p.p5);
        let eq217_e2709: f64 = (eq217_e2707 * p.p220);
        let eq217_e2711: f64 = (eq217_e2709 * (nv1 - nv2));
        let eq217_e2711_d_n2: f64 = (-eq217_e2709);
        let eq217_e2712_q: f64 = eq217_e2711;
        let eq217_e2713: f64 = (p.p7 * eq217_e2711);
        let eq217_e2713_d_n1: f64 = (p.p7 * eq217_e2709);
        let eq217_e2713_d_n2: f64 = (p.p7 * eq217_e2711_d_n2);
        let eq217_e2713_q: f64 = (p.p7 * eq217_e2712_q);
        let eq217_e2713_q_d_n1: f64 = (p.p7 * eq217_e2709);
        let eq217_e2713_q_d_n2: f64 = (p.p7 * eq217_e2711_d_n2);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq217_e2713_q_d_n1),
            nodes[2],
            multiplicity * (eq217_e2713_q_d_n2),
        );
        let eq218_e2716_q: f64 = s.v[196];
        let eq218_e2717: f64 = (p.p7 * s.v[196]);
        let eq218_e2717_d_n0: f64 = (p.p7 * s.dn[196][0]);
        let eq218_e2717_d_n1: f64 = (p.p7 * s.dn[196][1]);
        let eq218_e2717_d_n2: f64 = (p.p7 * s.dn[196][2]);
        let eq218_e2717_d_n3: f64 = (p.p7 * s.dn[196][3]);
        let eq218_e2717_d_n4: f64 = (p.p7 * s.dn[196][4]);
        let eq218_e2717_d_n5: f64 = (p.p7 * s.dn[196][5]);
        let eq218_e2717_d_n6: f64 = (p.p7 * s.dn[196][6]);
        let eq218_e2717_d_n7: f64 = (p.p7 * s.dn[196][7]);
        let eq218_e2717_d_n8: f64 = (p.p7 * s.dn[196][8]);
        let eq218_e2717_d_n9: f64 = (p.p7 * s.dn[196][9]);
        let eq218_e2717_d_n10: f64 = (p.p7 * s.dn[196][10]);
        let eq218_e2717_d_n11: f64 = (p.p7 * s.dn[196][11]);
        let eq218_e2717_d_n12: f64 = (p.p7 * s.dn[196][12]);
        let eq218_e2717_d_n13: f64 = (p.p7 * s.dn[196][13]);
        let eq218_e2717_d_n14: f64 = (p.p7 * s.dn[196][14]);
        let eq218_e2717_d_n15: f64 = (p.p7 * s.dn[196][15]);
        let eq218_e2717_d_n16: f64 = (p.p7 * s.dn[196][16]);
        let eq218_e2717_d_n17: f64 = (p.p7 * s.dn[196][17]);
        let eq218_e2717_d_n18: f64 = (p.p7 * s.dn[196][18]);
        let eq218_e2717_d_n19: f64 = (p.p7 * s.dn[196][19]);
        let eq218_e2717_d_n20: f64 = (p.p7 * s.dn[196][20]);
        let eq218_e2717_d_n21: f64 = (p.p7 * s.dn[196][21]);
        let eq218_e2717_d_n22: f64 = (p.p7 * s.dn[196][22]);
        let eq218_e2717_q: f64 = (p.p7 * eq218_e2716_q);
        let eq218_e2717_q_d_n0: f64 = (p.p7 * s.dn[196][0]);
        let eq218_e2717_q_d_n1: f64 = (p.p7 * s.dn[196][1]);
        let eq218_e2717_q_d_n2: f64 = (p.p7 * s.dn[196][2]);
        let eq218_e2717_q_d_n3: f64 = (p.p7 * s.dn[196][3]);
        let eq218_e2717_q_d_n4: f64 = (p.p7 * s.dn[196][4]);
        let eq218_e2717_q_d_n5: f64 = (p.p7 * s.dn[196][5]);
        let eq218_e2717_q_d_n6: f64 = (p.p7 * s.dn[196][6]);
        let eq218_e2717_q_d_n7: f64 = (p.p7 * s.dn[196][7]);
        let eq218_e2717_q_d_n8: f64 = (p.p7 * s.dn[196][8]);
        let eq218_e2717_q_d_n9: f64 = (p.p7 * s.dn[196][9]);
        let eq218_e2717_q_d_n10: f64 = (p.p7 * s.dn[196][10]);
        let eq218_e2717_q_d_n11: f64 = (p.p7 * s.dn[196][11]);
        let eq218_e2717_q_d_n12: f64 = (p.p7 * s.dn[196][12]);
        let eq218_e2717_q_d_n13: f64 = (p.p7 * s.dn[196][13]);
        let eq218_e2717_q_d_n14: f64 = (p.p7 * s.dn[196][14]);
        let eq218_e2717_q_d_n15: f64 = (p.p7 * s.dn[196][15]);
        let eq218_e2717_q_d_n16: f64 = (p.p7 * s.dn[196][16]);
        let eq218_e2717_q_d_n17: f64 = (p.p7 * s.dn[196][17]);
        let eq218_e2717_q_d_n18: f64 = (p.p7 * s.dn[196][18]);
        let eq218_e2717_q_d_n19: f64 = (p.p7 * s.dn[196][19]);
        let eq218_e2717_q_d_n20: f64 = (p.p7 * s.dn[196][20]);
        let eq218_e2717_q_d_n21: f64 = (p.p7 * s.dn[196][21]);
        let eq218_e2717_q_d_n22: f64 = (p.p7 * s.dn[196][22]);
        let eq218_reactive_node_derivatives: [f64; 23] = [eq218_e2717_q_d_n0, eq218_e2717_q_d_n1, eq218_e2717_q_d_n2, eq218_e2717_q_d_n3, eq218_e2717_q_d_n4, eq218_e2717_q_d_n5, eq218_e2717_q_d_n6, eq218_e2717_q_d_n7, eq218_e2717_q_d_n8, eq218_e2717_q_d_n9, eq218_e2717_q_d_n10, eq218_e2717_q_d_n11, eq218_e2717_q_d_n12, eq218_e2717_q_d_n13, eq218_e2717_q_d_n14, eq218_e2717_q_d_n15, eq218_e2717_q_d_n16, eq218_e2717_q_d_n17, eq218_e2717_q_d_n18, eq218_e2717_q_d_n19, eq218_e2717_q_d_n20, eq218_e2717_q_d_n21, eq218_e2717_q_d_n22];
        let eq218_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq218_reactive_node_derivatives,
            branches,
            &eq218_reactive_branch_derivatives,
            multiplicity,
        );
        let eq219_e2720_q: f64 = s.v[197];
        let eq219_e2721: f64 = (p.p7 * s.v[197]);
        let eq219_e2721_d_n0: f64 = (p.p7 * s.dn[197][0]);
        let eq219_e2721_d_n1: f64 = (p.p7 * s.dn[197][1]);
        let eq219_e2721_d_n2: f64 = (p.p7 * s.dn[197][2]);
        let eq219_e2721_d_n3: f64 = (p.p7 * s.dn[197][3]);
        let eq219_e2721_d_n4: f64 = (p.p7 * s.dn[197][4]);
        let eq219_e2721_d_n5: f64 = (p.p7 * s.dn[197][5]);
        let eq219_e2721_d_n6: f64 = (p.p7 * s.dn[197][6]);
        let eq219_e2721_d_n7: f64 = (p.p7 * s.dn[197][7]);
        let eq219_e2721_d_n8: f64 = (p.p7 * s.dn[197][8]);
        let eq219_e2721_d_n9: f64 = (p.p7 * s.dn[197][9]);
        let eq219_e2721_d_n10: f64 = (p.p7 * s.dn[197][10]);
        let eq219_e2721_d_n11: f64 = (p.p7 * s.dn[197][11]);
        let eq219_e2721_d_n12: f64 = (p.p7 * s.dn[197][12]);
        let eq219_e2721_d_n13: f64 = (p.p7 * s.dn[197][13]);
        let eq219_e2721_d_n14: f64 = (p.p7 * s.dn[197][14]);
        let eq219_e2721_d_n15: f64 = (p.p7 * s.dn[197][15]);
        let eq219_e2721_d_n16: f64 = (p.p7 * s.dn[197][16]);
        let eq219_e2721_d_n17: f64 = (p.p7 * s.dn[197][17]);
        let eq219_e2721_d_n18: f64 = (p.p7 * s.dn[197][18]);
        let eq219_e2721_d_n19: f64 = (p.p7 * s.dn[197][19]);
        let eq219_e2721_d_n20: f64 = (p.p7 * s.dn[197][20]);
        let eq219_e2721_d_n21: f64 = (p.p7 * s.dn[197][21]);
        let eq219_e2721_d_n22: f64 = (p.p7 * s.dn[197][22]);
        let eq219_e2721_q: f64 = (p.p7 * eq219_e2720_q);
        let eq219_e2721_q_d_n0: f64 = (p.p7 * s.dn[197][0]);
        let eq219_e2721_q_d_n1: f64 = (p.p7 * s.dn[197][1]);
        let eq219_e2721_q_d_n2: f64 = (p.p7 * s.dn[197][2]);
        let eq219_e2721_q_d_n3: f64 = (p.p7 * s.dn[197][3]);
        let eq219_e2721_q_d_n4: f64 = (p.p7 * s.dn[197][4]);
        let eq219_e2721_q_d_n5: f64 = (p.p7 * s.dn[197][5]);
        let eq219_e2721_q_d_n6: f64 = (p.p7 * s.dn[197][6]);
        let eq219_e2721_q_d_n7: f64 = (p.p7 * s.dn[197][7]);
        let eq219_e2721_q_d_n8: f64 = (p.p7 * s.dn[197][8]);
        let eq219_e2721_q_d_n9: f64 = (p.p7 * s.dn[197][9]);
        let eq219_e2721_q_d_n10: f64 = (p.p7 * s.dn[197][10]);
        let eq219_e2721_q_d_n11: f64 = (p.p7 * s.dn[197][11]);
        let eq219_e2721_q_d_n12: f64 = (p.p7 * s.dn[197][12]);
        let eq219_e2721_q_d_n13: f64 = (p.p7 * s.dn[197][13]);
        let eq219_e2721_q_d_n14: f64 = (p.p7 * s.dn[197][14]);
        let eq219_e2721_q_d_n15: f64 = (p.p7 * s.dn[197][15]);
        let eq219_e2721_q_d_n16: f64 = (p.p7 * s.dn[197][16]);
        let eq219_e2721_q_d_n17: f64 = (p.p7 * s.dn[197][17]);
        let eq219_e2721_q_d_n18: f64 = (p.p7 * s.dn[197][18]);
        let eq219_e2721_q_d_n19: f64 = (p.p7 * s.dn[197][19]);
        let eq219_e2721_q_d_n20: f64 = (p.p7 * s.dn[197][20]);
        let eq219_e2721_q_d_n21: f64 = (p.p7 * s.dn[197][21]);
        let eq219_e2721_q_d_n22: f64 = (p.p7 * s.dn[197][22]);
        let eq219_reactive_node_derivatives: [f64; 23] = [eq219_e2721_q_d_n0, eq219_e2721_q_d_n1, eq219_e2721_q_d_n2, eq219_e2721_q_d_n3, eq219_e2721_q_d_n4, eq219_e2721_q_d_n5, eq219_e2721_q_d_n6, eq219_e2721_q_d_n7, eq219_e2721_q_d_n8, eq219_e2721_q_d_n9, eq219_e2721_q_d_n10, eq219_e2721_q_d_n11, eq219_e2721_q_d_n12, eq219_e2721_q_d_n13, eq219_e2721_q_d_n14, eq219_e2721_q_d_n15, eq219_e2721_q_d_n16, eq219_e2721_q_d_n17, eq219_e2721_q_d_n18, eq219_e2721_q_d_n19, eq219_e2721_q_d_n20, eq219_e2721_q_d_n21, eq219_e2721_q_d_n22];
        let eq219_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &eq219_reactive_node_derivatives,
            branches,
            &eq219_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_21(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq220_e2724_q: f64 = s.v[194];
        let eq220_e2725: f64 = (p.p7 * s.v[194]);
        let eq220_e2725_d_n0: f64 = (p.p7 * s.dn[194][0]);
        let eq220_e2725_d_n1: f64 = (p.p7 * s.dn[194][1]);
        let eq220_e2725_d_n2: f64 = (p.p7 * s.dn[194][2]);
        let eq220_e2725_d_n3: f64 = (p.p7 * s.dn[194][3]);
        let eq220_e2725_d_n4: f64 = (p.p7 * s.dn[194][4]);
        let eq220_e2725_d_n5: f64 = (p.p7 * s.dn[194][5]);
        let eq220_e2725_d_n6: f64 = (p.p7 * s.dn[194][6]);
        let eq220_e2725_d_n7: f64 = (p.p7 * s.dn[194][7]);
        let eq220_e2725_d_n8: f64 = (p.p7 * s.dn[194][8]);
        let eq220_e2725_d_n9: f64 = (p.p7 * s.dn[194][9]);
        let eq220_e2725_d_n10: f64 = (p.p7 * s.dn[194][10]);
        let eq220_e2725_d_n11: f64 = (p.p7 * s.dn[194][11]);
        let eq220_e2725_d_n12: f64 = (p.p7 * s.dn[194][12]);
        let eq220_e2725_d_n13: f64 = (p.p7 * s.dn[194][13]);
        let eq220_e2725_d_n14: f64 = (p.p7 * s.dn[194][14]);
        let eq220_e2725_d_n15: f64 = (p.p7 * s.dn[194][15]);
        let eq220_e2725_d_n16: f64 = (p.p7 * s.dn[194][16]);
        let eq220_e2725_d_n17: f64 = (p.p7 * s.dn[194][17]);
        let eq220_e2725_d_n18: f64 = (p.p7 * s.dn[194][18]);
        let eq220_e2725_d_n19: f64 = (p.p7 * s.dn[194][19]);
        let eq220_e2725_d_n20: f64 = (p.p7 * s.dn[194][20]);
        let eq220_e2725_d_n21: f64 = (p.p7 * s.dn[194][21]);
        let eq220_e2725_d_n22: f64 = (p.p7 * s.dn[194][22]);
        let eq220_e2725_q: f64 = (p.p7 * eq220_e2724_q);
        let eq220_e2725_q_d_n0: f64 = (p.p7 * s.dn[194][0]);
        let eq220_e2725_q_d_n1: f64 = (p.p7 * s.dn[194][1]);
        let eq220_e2725_q_d_n2: f64 = (p.p7 * s.dn[194][2]);
        let eq220_e2725_q_d_n3: f64 = (p.p7 * s.dn[194][3]);
        let eq220_e2725_q_d_n4: f64 = (p.p7 * s.dn[194][4]);
        let eq220_e2725_q_d_n5: f64 = (p.p7 * s.dn[194][5]);
        let eq220_e2725_q_d_n6: f64 = (p.p7 * s.dn[194][6]);
        let eq220_e2725_q_d_n7: f64 = (p.p7 * s.dn[194][7]);
        let eq220_e2725_q_d_n8: f64 = (p.p7 * s.dn[194][8]);
        let eq220_e2725_q_d_n9: f64 = (p.p7 * s.dn[194][9]);
        let eq220_e2725_q_d_n10: f64 = (p.p7 * s.dn[194][10]);
        let eq220_e2725_q_d_n11: f64 = (p.p7 * s.dn[194][11]);
        let eq220_e2725_q_d_n12: f64 = (p.p7 * s.dn[194][12]);
        let eq220_e2725_q_d_n13: f64 = (p.p7 * s.dn[194][13]);
        let eq220_e2725_q_d_n14: f64 = (p.p7 * s.dn[194][14]);
        let eq220_e2725_q_d_n15: f64 = (p.p7 * s.dn[194][15]);
        let eq220_e2725_q_d_n16: f64 = (p.p7 * s.dn[194][16]);
        let eq220_e2725_q_d_n17: f64 = (p.p7 * s.dn[194][17]);
        let eq220_e2725_q_d_n18: f64 = (p.p7 * s.dn[194][18]);
        let eq220_e2725_q_d_n19: f64 = (p.p7 * s.dn[194][19]);
        let eq220_e2725_q_d_n20: f64 = (p.p7 * s.dn[194][20]);
        let eq220_e2725_q_d_n21: f64 = (p.p7 * s.dn[194][21]);
        let eq220_e2725_q_d_n22: f64 = (p.p7 * s.dn[194][22]);
        let eq220_reactive_node_derivatives: [f64; 23] = [eq220_e2725_q_d_n0, eq220_e2725_q_d_n1, eq220_e2725_q_d_n2, eq220_e2725_q_d_n3, eq220_e2725_q_d_n4, eq220_e2725_q_d_n5, eq220_e2725_q_d_n6, eq220_e2725_q_d_n7, eq220_e2725_q_d_n8, eq220_e2725_q_d_n9, eq220_e2725_q_d_n10, eq220_e2725_q_d_n11, eq220_e2725_q_d_n12, eq220_e2725_q_d_n13, eq220_e2725_q_d_n14, eq220_e2725_q_d_n15, eq220_e2725_q_d_n16, eq220_e2725_q_d_n17, eq220_e2725_q_d_n18, eq220_e2725_q_d_n19, eq220_e2725_q_d_n20, eq220_e2725_q_d_n21, eq220_e2725_q_d_n22];
        let eq220_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &eq220_reactive_node_derivatives,
            branches,
            &eq220_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq223_e2771, eq223_e2771_d_n4, eq223_e2771_q, eq223_e2771_q_d_n4,) = {
    if s.b[610] {
        let eq223_e2768: f64 = ((nv4 - 0.0) * p.p33);
        let eq223_e2768_d_n4: f64 = p.p33;
        let eq223_e2769_q: f64 = eq223_e2768;
        (eq223_e2768, eq223_e2768_d_n4, eq223_e2769_q, eq223_e2768_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq223_e2771_q_d_n4),
        );
    }
}
