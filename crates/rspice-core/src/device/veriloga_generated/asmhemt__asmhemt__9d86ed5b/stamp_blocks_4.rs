#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq85_e1278,) = {
    if ((!s.b[478]) && (!s.b[487])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq85_value: f64 = eq85_e1278;
        stamper.stamp_potential_const(
            branches[42],
            eq85_value,
        );
        let (eq86_e1294, eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22,) = {
    if (s.b[493] && s.b[494]) {
        let eq86_e1284: f64 = (p.p6 * s.v[64]);
        let eq86_e1284_d_n0: f64 = (p.p6 * s.dn[64][0]);
        let eq86_e1284_d_n1: f64 = (p.p6 * s.dn[64][1]);
        let eq86_e1284_d_n2: f64 = (p.p6 * s.dn[64][2]);
        let eq86_e1284_d_n3: f64 = (p.p6 * s.dn[64][3]);
        let eq86_e1284_d_n4: f64 = (p.p6 * s.dn[64][4]);
        let eq86_e1284_d_n5: f64 = (p.p6 * s.dn[64][5]);
        let eq86_e1284_d_n6: f64 = (p.p6 * s.dn[64][6]);
        let eq86_e1284_d_n7: f64 = (p.p6 * s.dn[64][7]);
        let eq86_e1284_d_n8: f64 = (p.p6 * s.dn[64][8]);
        let eq86_e1284_d_n9: f64 = (p.p6 * s.dn[64][9]);
        let eq86_e1284_d_n10: f64 = (p.p6 * s.dn[64][10]);
        let eq86_e1284_d_n11: f64 = (p.p6 * s.dn[64][11]);
        let eq86_e1284_d_n12: f64 = (p.p6 * s.dn[64][12]);
        let eq86_e1284_d_n13: f64 = (p.p6 * s.dn[64][13]);
        let eq86_e1284_d_n14: f64 = (p.p6 * s.dn[64][14]);
        let eq86_e1284_d_n15: f64 = (p.p6 * s.dn[64][15]);
        let eq86_e1284_d_n16: f64 = (p.p6 * s.dn[64][16]);
        let eq86_e1284_d_n17: f64 = (p.p6 * s.dn[64][17]);
        let eq86_e1284_d_n18: f64 = (p.p6 * s.dn[64][18]);
        let eq86_e1284_d_n19: f64 = (p.p6 * s.dn[64][19]);
        let eq86_e1284_d_n20: f64 = (p.p6 * s.dn[64][20]);
        let eq86_e1284_d_n21: f64 = (p.p6 * s.dn[64][21]);
        let eq86_e1284_d_n22: f64 = (p.p6 * s.dn[64][22]);
        let eq86_e1286: f64 = (eq86_e1284 * s.v[281]);
        let eq86_e1286_d_n0: f64 = ((eq86_e1284_d_n0 * s.v[281]) + (eq86_e1284 * s.dn[281][0]));
        let eq86_e1286_d_n1: f64 = ((eq86_e1284_d_n1 * s.v[281]) + (eq86_e1284 * s.dn[281][1]));
        let eq86_e1286_d_n2: f64 = ((eq86_e1284_d_n2 * s.v[281]) + (eq86_e1284 * s.dn[281][2]));
        let eq86_e1286_d_n3: f64 = ((eq86_e1284_d_n3 * s.v[281]) + (eq86_e1284 * s.dn[281][3]));
        let eq86_e1286_d_n4: f64 = ((eq86_e1284_d_n4 * s.v[281]) + (eq86_e1284 * s.dn[281][4]));
        let eq86_e1286_d_n5: f64 = ((eq86_e1284_d_n5 * s.v[281]) + (eq86_e1284 * s.dn[281][5]));
        let eq86_e1286_d_n6: f64 = ((eq86_e1284_d_n6 * s.v[281]) + (eq86_e1284 * s.dn[281][6]));
        let eq86_e1286_d_n7: f64 = ((eq86_e1284_d_n7 * s.v[281]) + (eq86_e1284 * s.dn[281][7]));
        let eq86_e1286_d_n8: f64 = ((eq86_e1284_d_n8 * s.v[281]) + (eq86_e1284 * s.dn[281][8]));
        let eq86_e1286_d_n9: f64 = ((eq86_e1284_d_n9 * s.v[281]) + (eq86_e1284 * s.dn[281][9]));
        let eq86_e1286_d_n10: f64 = ((eq86_e1284_d_n10 * s.v[281]) + (eq86_e1284 * s.dn[281][10]));
        let eq86_e1286_d_n11: f64 = ((eq86_e1284_d_n11 * s.v[281]) + (eq86_e1284 * s.dn[281][11]));
        let eq86_e1286_d_n12: f64 = ((eq86_e1284_d_n12 * s.v[281]) + (eq86_e1284 * s.dn[281][12]));
        let eq86_e1286_d_n13: f64 = ((eq86_e1284_d_n13 * s.v[281]) + (eq86_e1284 * s.dn[281][13]));
        let eq86_e1286_d_n14: f64 = ((eq86_e1284_d_n14 * s.v[281]) + (eq86_e1284 * s.dn[281][14]));
        let eq86_e1286_d_n15: f64 = ((eq86_e1284_d_n15 * s.v[281]) + (eq86_e1284 * s.dn[281][15]));
        let eq86_e1286_d_n16: f64 = ((eq86_e1284_d_n16 * s.v[281]) + (eq86_e1284 * s.dn[281][16]));
        let eq86_e1286_d_n17: f64 = ((eq86_e1284_d_n17 * s.v[281]) + (eq86_e1284 * s.dn[281][17]));
        let eq86_e1286_d_n18: f64 = ((eq86_e1284_d_n18 * s.v[281]) + (eq86_e1284 * s.dn[281][18]));
        let eq86_e1286_d_n19: f64 = ((eq86_e1284_d_n19 * s.v[281]) + (eq86_e1284 * s.dn[281][19]));
        let eq86_e1286_d_n20: f64 = ((eq86_e1284_d_n20 * s.v[281]) + (eq86_e1284 * s.dn[281][20]));
        let eq86_e1286_d_n21: f64 = ((eq86_e1284_d_n21 * s.v[281]) + (eq86_e1284 * s.dn[281][21]));
        let eq86_e1286_d_n22: f64 = ((eq86_e1284_d_n22 * s.v[281]) + (eq86_e1284 * s.dn[281][22]));
        let eq86_e1289: f64 = (p.p6 * s.v[379]);
        let eq86_e1289_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq86_e1289_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq86_e1289_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq86_e1289_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq86_e1289_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq86_e1289_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq86_e1289_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq86_e1289_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq86_e1289_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq86_e1289_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq86_e1289_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq86_e1289_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq86_e1289_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq86_e1289_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq86_e1289_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq86_e1289_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq86_e1289_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq86_e1289_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq86_e1289_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq86_e1289_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq86_e1289_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq86_e1289_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq86_e1289_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq86_e1291: f64 = (eq86_e1289 * (nv17 - nv16));
        let eq86_e1291_d_n0: f64 = (eq86_e1289_d_n0 * (nv17 - nv16));
        let eq86_e1291_d_n1: f64 = (eq86_e1289_d_n1 * (nv17 - nv16));
        let eq86_e1291_d_n2: f64 = (eq86_e1289_d_n2 * (nv17 - nv16));
        let eq86_e1291_d_n3: f64 = (eq86_e1289_d_n3 * (nv17 - nv16));
        let eq86_e1291_d_n4: f64 = (eq86_e1289_d_n4 * (nv17 - nv16));
        let eq86_e1291_d_n5: f64 = (eq86_e1289_d_n5 * (nv17 - nv16));
        let eq86_e1291_d_n6: f64 = (eq86_e1289_d_n6 * (nv17 - nv16));
        let eq86_e1291_d_n7: f64 = (eq86_e1289_d_n7 * (nv17 - nv16));
        let eq86_e1291_d_n8: f64 = (eq86_e1289_d_n8 * (nv17 - nv16));
        let eq86_e1291_d_n9: f64 = (eq86_e1289_d_n9 * (nv17 - nv16));
        let eq86_e1291_d_n10: f64 = (eq86_e1289_d_n10 * (nv17 - nv16));
        let eq86_e1291_d_n11: f64 = (eq86_e1289_d_n11 * (nv17 - nv16));
        let eq86_e1291_d_n12: f64 = (eq86_e1289_d_n12 * (nv17 - nv16));
        let eq86_e1291_d_n13: f64 = (eq86_e1289_d_n13 * (nv17 - nv16));
        let eq86_e1291_d_n14: f64 = (eq86_e1289_d_n14 * (nv17 - nv16));
        let eq86_e1291_d_n15: f64 = (eq86_e1289_d_n15 * (nv17 - nv16));
        let eq86_e1291_d_n16: f64 = ((eq86_e1289_d_n16 * (nv17 - nv16)) + (-eq86_e1289));
        let eq86_e1291_d_n17: f64 = ((eq86_e1289_d_n17 * (nv17 - nv16)) + eq86_e1289);
        let eq86_e1291_d_n18: f64 = (eq86_e1289_d_n18 * (nv17 - nv16));
        let eq86_e1291_d_n19: f64 = (eq86_e1289_d_n19 * (nv17 - nv16));
        let eq86_e1291_d_n20: f64 = (eq86_e1289_d_n20 * (nv17 - nv16));
        let eq86_e1291_d_n21: f64 = (eq86_e1289_d_n21 * (nv17 - nv16));
        let eq86_e1291_d_n22: f64 = (eq86_e1289_d_n22 * (nv17 - nv16));
        let eq86_e1292: f64 = (eq86_e1286 + eq86_e1291);
        let eq86_e1292_d_n0: f64 = (eq86_e1286_d_n0 + eq86_e1291_d_n0);
        let eq86_e1292_d_n1: f64 = (eq86_e1286_d_n1 + eq86_e1291_d_n1);
        let eq86_e1292_d_n2: f64 = (eq86_e1286_d_n2 + eq86_e1291_d_n2);
        let eq86_e1292_d_n3: f64 = (eq86_e1286_d_n3 + eq86_e1291_d_n3);
        let eq86_e1292_d_n4: f64 = (eq86_e1286_d_n4 + eq86_e1291_d_n4);
        let eq86_e1292_d_n5: f64 = (eq86_e1286_d_n5 + eq86_e1291_d_n5);
        let eq86_e1292_d_n6: f64 = (eq86_e1286_d_n6 + eq86_e1291_d_n6);
        let eq86_e1292_d_n7: f64 = (eq86_e1286_d_n7 + eq86_e1291_d_n7);
        let eq86_e1292_d_n8: f64 = (eq86_e1286_d_n8 + eq86_e1291_d_n8);
        let eq86_e1292_d_n9: f64 = (eq86_e1286_d_n9 + eq86_e1291_d_n9);
        let eq86_e1292_d_n10: f64 = (eq86_e1286_d_n10 + eq86_e1291_d_n10);
        let eq86_e1292_d_n11: f64 = (eq86_e1286_d_n11 + eq86_e1291_d_n11);
        let eq86_e1292_d_n12: f64 = (eq86_e1286_d_n12 + eq86_e1291_d_n12);
        let eq86_e1292_d_n13: f64 = (eq86_e1286_d_n13 + eq86_e1291_d_n13);
        let eq86_e1292_d_n14: f64 = (eq86_e1286_d_n14 + eq86_e1291_d_n14);
        let eq86_e1292_d_n15: f64 = (eq86_e1286_d_n15 + eq86_e1291_d_n15);
        let eq86_e1292_d_n16: f64 = (eq86_e1286_d_n16 + eq86_e1291_d_n16);
        let eq86_e1292_d_n17: f64 = (eq86_e1286_d_n17 + eq86_e1291_d_n17);
        let eq86_e1292_d_n18: f64 = (eq86_e1286_d_n18 + eq86_e1291_d_n18);
        let eq86_e1292_d_n19: f64 = (eq86_e1286_d_n19 + eq86_e1291_d_n19);
        let eq86_e1292_d_n20: f64 = (eq86_e1286_d_n20 + eq86_e1291_d_n20);
        let eq86_e1292_d_n21: f64 = (eq86_e1286_d_n21 + eq86_e1291_d_n21);
        let eq86_e1292_d_n22: f64 = (eq86_e1286_d_n22 + eq86_e1291_d_n22);
        (eq86_e1292, eq86_e1292_d_n0, eq86_e1292_d_n1, eq86_e1292_d_n2, eq86_e1292_d_n3, eq86_e1292_d_n4, eq86_e1292_d_n5, eq86_e1292_d_n6, eq86_e1292_d_n7, eq86_e1292_d_n8, eq86_e1292_d_n9, eq86_e1292_d_n10, eq86_e1292_d_n11, eq86_e1292_d_n12, eq86_e1292_d_n13, eq86_e1292_d_n14, eq86_e1292_d_n15, eq86_e1292_d_n16, eq86_e1292_d_n17, eq86_e1292_d_n18, eq86_e1292_d_n19, eq86_e1292_d_n20, eq86_e1292_d_n21, eq86_e1292_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1294;
        let eq86_node_derivatives: [f64; 23] = [eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22];
        let eq86_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            multiplicity * (eq86_value),
            nodes,
            &eq86_node_derivatives,
            branches,
            &eq86_branch_derivatives,
            multiplicity,
        );
        let (eq87_e1301,) = {
    if (s.b[493] && (!s.b[494])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq87_value: f64 = eq87_e1301;
        stamper.stamp_potential_const(
            branches[43],
            eq87_value,
        );
        let (eq88_e1306,) = {
    if (!s.b[493]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e1306;
        stamper.stamp_potential_const(
            branches[44],
            eq88_value,
        );
        let (eq89_e1322, eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22,) = {
    if (s.b[508] && s.b[509]) {
        let eq89_e1312: f64 = (p.p6 * s.v[68]);
        let eq89_e1312_d_n0: f64 = (p.p6 * s.dn[68][0]);
        let eq89_e1312_d_n1: f64 = (p.p6 * s.dn[68][1]);
        let eq89_e1312_d_n2: f64 = (p.p6 * s.dn[68][2]);
        let eq89_e1312_d_n3: f64 = (p.p6 * s.dn[68][3]);
        let eq89_e1312_d_n4: f64 = (p.p6 * s.dn[68][4]);
        let eq89_e1312_d_n5: f64 = (p.p6 * s.dn[68][5]);
        let eq89_e1312_d_n6: f64 = (p.p6 * s.dn[68][6]);
        let eq89_e1312_d_n7: f64 = (p.p6 * s.dn[68][7]);
        let eq89_e1312_d_n8: f64 = (p.p6 * s.dn[68][8]);
        let eq89_e1312_d_n9: f64 = (p.p6 * s.dn[68][9]);
        let eq89_e1312_d_n10: f64 = (p.p6 * s.dn[68][10]);
        let eq89_e1312_d_n11: f64 = (p.p6 * s.dn[68][11]);
        let eq89_e1312_d_n12: f64 = (p.p6 * s.dn[68][12]);
        let eq89_e1312_d_n13: f64 = (p.p6 * s.dn[68][13]);
        let eq89_e1312_d_n14: f64 = (p.p6 * s.dn[68][14]);
        let eq89_e1312_d_n15: f64 = (p.p6 * s.dn[68][15]);
        let eq89_e1312_d_n16: f64 = (p.p6 * s.dn[68][16]);
        let eq89_e1312_d_n17: f64 = (p.p6 * s.dn[68][17]);
        let eq89_e1312_d_n18: f64 = (p.p6 * s.dn[68][18]);
        let eq89_e1312_d_n19: f64 = (p.p6 * s.dn[68][19]);
        let eq89_e1312_d_n20: f64 = (p.p6 * s.dn[68][20]);
        let eq89_e1312_d_n21: f64 = (p.p6 * s.dn[68][21]);
        let eq89_e1312_d_n22: f64 = (p.p6 * s.dn[68][22]);
        let eq89_e1314: f64 = (eq89_e1312 * s.v[293]);
        let eq89_e1314_d_n0: f64 = ((eq89_e1312_d_n0 * s.v[293]) + (eq89_e1312 * s.dn[293][0]));
        let eq89_e1314_d_n1: f64 = ((eq89_e1312_d_n1 * s.v[293]) + (eq89_e1312 * s.dn[293][1]));
        let eq89_e1314_d_n2: f64 = ((eq89_e1312_d_n2 * s.v[293]) + (eq89_e1312 * s.dn[293][2]));
        let eq89_e1314_d_n3: f64 = ((eq89_e1312_d_n3 * s.v[293]) + (eq89_e1312 * s.dn[293][3]));
        let eq89_e1314_d_n4: f64 = ((eq89_e1312_d_n4 * s.v[293]) + (eq89_e1312 * s.dn[293][4]));
        let eq89_e1314_d_n5: f64 = ((eq89_e1312_d_n5 * s.v[293]) + (eq89_e1312 * s.dn[293][5]));
        let eq89_e1314_d_n6: f64 = ((eq89_e1312_d_n6 * s.v[293]) + (eq89_e1312 * s.dn[293][6]));
        let eq89_e1314_d_n7: f64 = ((eq89_e1312_d_n7 * s.v[293]) + (eq89_e1312 * s.dn[293][7]));
        let eq89_e1314_d_n8: f64 = ((eq89_e1312_d_n8 * s.v[293]) + (eq89_e1312 * s.dn[293][8]));
        let eq89_e1314_d_n9: f64 = ((eq89_e1312_d_n9 * s.v[293]) + (eq89_e1312 * s.dn[293][9]));
        let eq89_e1314_d_n10: f64 = ((eq89_e1312_d_n10 * s.v[293]) + (eq89_e1312 * s.dn[293][10]));
        let eq89_e1314_d_n11: f64 = ((eq89_e1312_d_n11 * s.v[293]) + (eq89_e1312 * s.dn[293][11]));
        let eq89_e1314_d_n12: f64 = ((eq89_e1312_d_n12 * s.v[293]) + (eq89_e1312 * s.dn[293][12]));
        let eq89_e1314_d_n13: f64 = ((eq89_e1312_d_n13 * s.v[293]) + (eq89_e1312 * s.dn[293][13]));
        let eq89_e1314_d_n14: f64 = ((eq89_e1312_d_n14 * s.v[293]) + (eq89_e1312 * s.dn[293][14]));
        let eq89_e1314_d_n15: f64 = ((eq89_e1312_d_n15 * s.v[293]) + (eq89_e1312 * s.dn[293][15]));
        let eq89_e1314_d_n16: f64 = ((eq89_e1312_d_n16 * s.v[293]) + (eq89_e1312 * s.dn[293][16]));
        let eq89_e1314_d_n17: f64 = ((eq89_e1312_d_n17 * s.v[293]) + (eq89_e1312 * s.dn[293][17]));
        let eq89_e1314_d_n18: f64 = ((eq89_e1312_d_n18 * s.v[293]) + (eq89_e1312 * s.dn[293][18]));
        let eq89_e1314_d_n19: f64 = ((eq89_e1312_d_n19 * s.v[293]) + (eq89_e1312 * s.dn[293][19]));
        let eq89_e1314_d_n20: f64 = ((eq89_e1312_d_n20 * s.v[293]) + (eq89_e1312 * s.dn[293][20]));
        let eq89_e1314_d_n21: f64 = ((eq89_e1312_d_n21 * s.v[293]) + (eq89_e1312 * s.dn[293][21]));
        let eq89_e1314_d_n22: f64 = ((eq89_e1312_d_n22 * s.v[293]) + (eq89_e1312 * s.dn[293][22]));
        let eq89_e1317: f64 = (p.p6 * s.v[379]);
        let eq89_e1317_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq89_e1317_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq89_e1317_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq89_e1317_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq89_e1317_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq89_e1317_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq89_e1317_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq89_e1317_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq89_e1317_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq89_e1317_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq89_e1317_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq89_e1317_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq89_e1317_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq89_e1317_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq89_e1317_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq89_e1317_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq89_e1317_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq89_e1317_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq89_e1317_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq89_e1317_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq89_e1317_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq89_e1317_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq89_e1317_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq89_e1319: f64 = (eq89_e1317 * (nv20 - nv21));
        let eq89_e1319_d_n0: f64 = (eq89_e1317_d_n0 * (nv20 - nv21));
        let eq89_e1319_d_n1: f64 = (eq89_e1317_d_n1 * (nv20 - nv21));
        let eq89_e1319_d_n2: f64 = (eq89_e1317_d_n2 * (nv20 - nv21));
        let eq89_e1319_d_n3: f64 = (eq89_e1317_d_n3 * (nv20 - nv21));
        let eq89_e1319_d_n4: f64 = (eq89_e1317_d_n4 * (nv20 - nv21));
        let eq89_e1319_d_n5: f64 = (eq89_e1317_d_n5 * (nv20 - nv21));
        let eq89_e1319_d_n6: f64 = (eq89_e1317_d_n6 * (nv20 - nv21));
        let eq89_e1319_d_n7: f64 = (eq89_e1317_d_n7 * (nv20 - nv21));
        let eq89_e1319_d_n8: f64 = (eq89_e1317_d_n8 * (nv20 - nv21));
        let eq89_e1319_d_n9: f64 = (eq89_e1317_d_n9 * (nv20 - nv21));
        let eq89_e1319_d_n10: f64 = (eq89_e1317_d_n10 * (nv20 - nv21));
        let eq89_e1319_d_n11: f64 = (eq89_e1317_d_n11 * (nv20 - nv21));
        let eq89_e1319_d_n12: f64 = (eq89_e1317_d_n12 * (nv20 - nv21));
        let eq89_e1319_d_n13: f64 = (eq89_e1317_d_n13 * (nv20 - nv21));
        let eq89_e1319_d_n14: f64 = (eq89_e1317_d_n14 * (nv20 - nv21));
        let eq89_e1319_d_n15: f64 = (eq89_e1317_d_n15 * (nv20 - nv21));
        let eq89_e1319_d_n16: f64 = (eq89_e1317_d_n16 * (nv20 - nv21));
        let eq89_e1319_d_n17: f64 = (eq89_e1317_d_n17 * (nv20 - nv21));
        let eq89_e1319_d_n18: f64 = (eq89_e1317_d_n18 * (nv20 - nv21));
        let eq89_e1319_d_n19: f64 = (eq89_e1317_d_n19 * (nv20 - nv21));
        let eq89_e1319_d_n20: f64 = ((eq89_e1317_d_n20 * (nv20 - nv21)) + eq89_e1317);
        let eq89_e1319_d_n21: f64 = ((eq89_e1317_d_n21 * (nv20 - nv21)) + (-eq89_e1317));
        let eq89_e1319_d_n22: f64 = (eq89_e1317_d_n22 * (nv20 - nv21));
        let eq89_e1320: f64 = (eq89_e1314 + eq89_e1319);
        let eq89_e1320_d_n0: f64 = (eq89_e1314_d_n0 + eq89_e1319_d_n0);
        let eq89_e1320_d_n1: f64 = (eq89_e1314_d_n1 + eq89_e1319_d_n1);
        let eq89_e1320_d_n2: f64 = (eq89_e1314_d_n2 + eq89_e1319_d_n2);
        let eq89_e1320_d_n3: f64 = (eq89_e1314_d_n3 + eq89_e1319_d_n3);
        let eq89_e1320_d_n4: f64 = (eq89_e1314_d_n4 + eq89_e1319_d_n4);
        let eq89_e1320_d_n5: f64 = (eq89_e1314_d_n5 + eq89_e1319_d_n5);
        let eq89_e1320_d_n6: f64 = (eq89_e1314_d_n6 + eq89_e1319_d_n6);
        let eq89_e1320_d_n7: f64 = (eq89_e1314_d_n7 + eq89_e1319_d_n7);
        let eq89_e1320_d_n8: f64 = (eq89_e1314_d_n8 + eq89_e1319_d_n8);
        let eq89_e1320_d_n9: f64 = (eq89_e1314_d_n9 + eq89_e1319_d_n9);
        let eq89_e1320_d_n10: f64 = (eq89_e1314_d_n10 + eq89_e1319_d_n10);
        let eq89_e1320_d_n11: f64 = (eq89_e1314_d_n11 + eq89_e1319_d_n11);
        let eq89_e1320_d_n12: f64 = (eq89_e1314_d_n12 + eq89_e1319_d_n12);
        let eq89_e1320_d_n13: f64 = (eq89_e1314_d_n13 + eq89_e1319_d_n13);
        let eq89_e1320_d_n14: f64 = (eq89_e1314_d_n14 + eq89_e1319_d_n14);
        let eq89_e1320_d_n15: f64 = (eq89_e1314_d_n15 + eq89_e1319_d_n15);
        let eq89_e1320_d_n16: f64 = (eq89_e1314_d_n16 + eq89_e1319_d_n16);
        let eq89_e1320_d_n17: f64 = (eq89_e1314_d_n17 + eq89_e1319_d_n17);
        let eq89_e1320_d_n18: f64 = (eq89_e1314_d_n18 + eq89_e1319_d_n18);
        let eq89_e1320_d_n19: f64 = (eq89_e1314_d_n19 + eq89_e1319_d_n19);
        let eq89_e1320_d_n20: f64 = (eq89_e1314_d_n20 + eq89_e1319_d_n20);
        let eq89_e1320_d_n21: f64 = (eq89_e1314_d_n21 + eq89_e1319_d_n21);
        let eq89_e1320_d_n22: f64 = (eq89_e1314_d_n22 + eq89_e1319_d_n22);
        (eq89_e1320, eq89_e1320_d_n0, eq89_e1320_d_n1, eq89_e1320_d_n2, eq89_e1320_d_n3, eq89_e1320_d_n4, eq89_e1320_d_n5, eq89_e1320_d_n6, eq89_e1320_d_n7, eq89_e1320_d_n8, eq89_e1320_d_n9, eq89_e1320_d_n10, eq89_e1320_d_n11, eq89_e1320_d_n12, eq89_e1320_d_n13, eq89_e1320_d_n14, eq89_e1320_d_n15, eq89_e1320_d_n16, eq89_e1320_d_n17, eq89_e1320_d_n18, eq89_e1320_d_n19, eq89_e1320_d_n20, eq89_e1320_d_n21, eq89_e1320_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1322;
        let eq89_node_derivatives: [f64; 23] = [eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22];
        let eq89_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[20]),
            Some(nodes[21]),
            multiplicity * (eq89_value),
            nodes,
            &eq89_node_derivatives,
            branches,
            &eq89_branch_derivatives,
            multiplicity,
        );
        let (eq90_e1329,) = {
    if (s.b[508] && (!s.b[509])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq90_value: f64 = eq90_e1329;
        stamper.stamp_potential_const(
            branches[45],
            eq90_value,
        );
        let (eq91_e1334,) = {
    if (!s.b[508]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq91_value: f64 = eq91_e1334;
        stamper.stamp_potential_const(
            branches[46],
            eq91_value,
        );
        let (eq92_e1342,) = {
    if ((!s.b[508]) && (!s.b[517])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq92_value: f64 = eq92_e1342;
        stamper.stamp_potential_const(
            branches[47],
            eq92_value,
        );
        let (eq93_e1358, eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22,) = {
    if (s.b[523] && s.b[524]) {
        let eq93_e1348: f64 = (p.p6 * s.v[72]);
        let eq93_e1348_d_n0: f64 = (p.p6 * s.dn[72][0]);
        let eq93_e1348_d_n1: f64 = (p.p6 * s.dn[72][1]);
        let eq93_e1348_d_n2: f64 = (p.p6 * s.dn[72][2]);
        let eq93_e1348_d_n3: f64 = (p.p6 * s.dn[72][3]);
        let eq93_e1348_d_n4: f64 = (p.p6 * s.dn[72][4]);
        let eq93_e1348_d_n5: f64 = (p.p6 * s.dn[72][5]);
        let eq93_e1348_d_n6: f64 = (p.p6 * s.dn[72][6]);
        let eq93_e1348_d_n7: f64 = (p.p6 * s.dn[72][7]);
        let eq93_e1348_d_n8: f64 = (p.p6 * s.dn[72][8]);
        let eq93_e1348_d_n9: f64 = (p.p6 * s.dn[72][9]);
        let eq93_e1348_d_n10: f64 = (p.p6 * s.dn[72][10]);
        let eq93_e1348_d_n11: f64 = (p.p6 * s.dn[72][11]);
        let eq93_e1348_d_n12: f64 = (p.p6 * s.dn[72][12]);
        let eq93_e1348_d_n13: f64 = (p.p6 * s.dn[72][13]);
        let eq93_e1348_d_n14: f64 = (p.p6 * s.dn[72][14]);
        let eq93_e1348_d_n15: f64 = (p.p6 * s.dn[72][15]);
        let eq93_e1348_d_n16: f64 = (p.p6 * s.dn[72][16]);
        let eq93_e1348_d_n17: f64 = (p.p6 * s.dn[72][17]);
        let eq93_e1348_d_n18: f64 = (p.p6 * s.dn[72][18]);
        let eq93_e1348_d_n19: f64 = (p.p6 * s.dn[72][19]);
        let eq93_e1348_d_n20: f64 = (p.p6 * s.dn[72][20]);
        let eq93_e1348_d_n21: f64 = (p.p6 * s.dn[72][21]);
        let eq93_e1348_d_n22: f64 = (p.p6 * s.dn[72][22]);
        let eq93_e1350: f64 = (eq93_e1348 * s.v[305]);
        let eq93_e1350_d_n0: f64 = ((eq93_e1348_d_n0 * s.v[305]) + (eq93_e1348 * s.dn[305][0]));
        let eq93_e1350_d_n1: f64 = ((eq93_e1348_d_n1 * s.v[305]) + (eq93_e1348 * s.dn[305][1]));
        let eq93_e1350_d_n2: f64 = ((eq93_e1348_d_n2 * s.v[305]) + (eq93_e1348 * s.dn[305][2]));
        let eq93_e1350_d_n3: f64 = ((eq93_e1348_d_n3 * s.v[305]) + (eq93_e1348 * s.dn[305][3]));
        let eq93_e1350_d_n4: f64 = ((eq93_e1348_d_n4 * s.v[305]) + (eq93_e1348 * s.dn[305][4]));
        let eq93_e1350_d_n5: f64 = ((eq93_e1348_d_n5 * s.v[305]) + (eq93_e1348 * s.dn[305][5]));
        let eq93_e1350_d_n6: f64 = ((eq93_e1348_d_n6 * s.v[305]) + (eq93_e1348 * s.dn[305][6]));
        let eq93_e1350_d_n7: f64 = ((eq93_e1348_d_n7 * s.v[305]) + (eq93_e1348 * s.dn[305][7]));
        let eq93_e1350_d_n8: f64 = ((eq93_e1348_d_n8 * s.v[305]) + (eq93_e1348 * s.dn[305][8]));
        let eq93_e1350_d_n9: f64 = ((eq93_e1348_d_n9 * s.v[305]) + (eq93_e1348 * s.dn[305][9]));
        let eq93_e1350_d_n10: f64 = ((eq93_e1348_d_n10 * s.v[305]) + (eq93_e1348 * s.dn[305][10]));
        let eq93_e1350_d_n11: f64 = ((eq93_e1348_d_n11 * s.v[305]) + (eq93_e1348 * s.dn[305][11]));
        let eq93_e1350_d_n12: f64 = ((eq93_e1348_d_n12 * s.v[305]) + (eq93_e1348 * s.dn[305][12]));
        let eq93_e1350_d_n13: f64 = ((eq93_e1348_d_n13 * s.v[305]) + (eq93_e1348 * s.dn[305][13]));
        let eq93_e1350_d_n14: f64 = ((eq93_e1348_d_n14 * s.v[305]) + (eq93_e1348 * s.dn[305][14]));
        let eq93_e1350_d_n15: f64 = ((eq93_e1348_d_n15 * s.v[305]) + (eq93_e1348 * s.dn[305][15]));
        let eq93_e1350_d_n16: f64 = ((eq93_e1348_d_n16 * s.v[305]) + (eq93_e1348 * s.dn[305][16]));
        let eq93_e1350_d_n17: f64 = ((eq93_e1348_d_n17 * s.v[305]) + (eq93_e1348 * s.dn[305][17]));
        let eq93_e1350_d_n18: f64 = ((eq93_e1348_d_n18 * s.v[305]) + (eq93_e1348 * s.dn[305][18]));
        let eq93_e1350_d_n19: f64 = ((eq93_e1348_d_n19 * s.v[305]) + (eq93_e1348 * s.dn[305][19]));
        let eq93_e1350_d_n20: f64 = ((eq93_e1348_d_n20 * s.v[305]) + (eq93_e1348 * s.dn[305][20]));
        let eq93_e1350_d_n21: f64 = ((eq93_e1348_d_n21 * s.v[305]) + (eq93_e1348 * s.dn[305][21]));
        let eq93_e1350_d_n22: f64 = ((eq93_e1348_d_n22 * s.v[305]) + (eq93_e1348 * s.dn[305][22]));
        let eq93_e1353: f64 = (p.p6 * s.v[379]);
        let eq93_e1353_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq93_e1353_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq93_e1353_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq93_e1353_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq93_e1353_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq93_e1353_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq93_e1353_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq93_e1353_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq93_e1353_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq93_e1353_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq93_e1353_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq93_e1353_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq93_e1353_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq93_e1353_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq93_e1353_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq93_e1353_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq93_e1353_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq93_e1353_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq93_e1353_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq93_e1353_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq93_e1353_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq93_e1353_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq93_e1353_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq93_e1355: f64 = (eq93_e1353 * (nv18 - nv17));
        let eq93_e1355_d_n0: f64 = (eq93_e1353_d_n0 * (nv18 - nv17));
        let eq93_e1355_d_n1: f64 = (eq93_e1353_d_n1 * (nv18 - nv17));
        let eq93_e1355_d_n2: f64 = (eq93_e1353_d_n2 * (nv18 - nv17));
        let eq93_e1355_d_n3: f64 = (eq93_e1353_d_n3 * (nv18 - nv17));
        let eq93_e1355_d_n4: f64 = (eq93_e1353_d_n4 * (nv18 - nv17));
        let eq93_e1355_d_n5: f64 = (eq93_e1353_d_n5 * (nv18 - nv17));
        let eq93_e1355_d_n6: f64 = (eq93_e1353_d_n6 * (nv18 - nv17));
        let eq93_e1355_d_n7: f64 = (eq93_e1353_d_n7 * (nv18 - nv17));
        let eq93_e1355_d_n8: f64 = (eq93_e1353_d_n8 * (nv18 - nv17));
        let eq93_e1355_d_n9: f64 = (eq93_e1353_d_n9 * (nv18 - nv17));
        let eq93_e1355_d_n10: f64 = (eq93_e1353_d_n10 * (nv18 - nv17));
        let eq93_e1355_d_n11: f64 = (eq93_e1353_d_n11 * (nv18 - nv17));
        let eq93_e1355_d_n12: f64 = (eq93_e1353_d_n12 * (nv18 - nv17));
        let eq93_e1355_d_n13: f64 = (eq93_e1353_d_n13 * (nv18 - nv17));
        let eq93_e1355_d_n14: f64 = (eq93_e1353_d_n14 * (nv18 - nv17));
        let eq93_e1355_d_n15: f64 = (eq93_e1353_d_n15 * (nv18 - nv17));
        let eq93_e1355_d_n16: f64 = (eq93_e1353_d_n16 * (nv18 - nv17));
        let eq93_e1355_d_n17: f64 = ((eq93_e1353_d_n17 * (nv18 - nv17)) + (-eq93_e1353));
        let eq93_e1355_d_n18: f64 = ((eq93_e1353_d_n18 * (nv18 - nv17)) + eq93_e1353);
        let eq93_e1355_d_n19: f64 = (eq93_e1353_d_n19 * (nv18 - nv17));
        let eq93_e1355_d_n20: f64 = (eq93_e1353_d_n20 * (nv18 - nv17));
        let eq93_e1355_d_n21: f64 = (eq93_e1353_d_n21 * (nv18 - nv17));
        let eq93_e1355_d_n22: f64 = (eq93_e1353_d_n22 * (nv18 - nv17));
        let eq93_e1356: f64 = (eq93_e1350 + eq93_e1355);
        let eq93_e1356_d_n0: f64 = (eq93_e1350_d_n0 + eq93_e1355_d_n0);
        let eq93_e1356_d_n1: f64 = (eq93_e1350_d_n1 + eq93_e1355_d_n1);
        let eq93_e1356_d_n2: f64 = (eq93_e1350_d_n2 + eq93_e1355_d_n2);
        let eq93_e1356_d_n3: f64 = (eq93_e1350_d_n3 + eq93_e1355_d_n3);
        let eq93_e1356_d_n4: f64 = (eq93_e1350_d_n4 + eq93_e1355_d_n4);
        let eq93_e1356_d_n5: f64 = (eq93_e1350_d_n5 + eq93_e1355_d_n5);
        let eq93_e1356_d_n6: f64 = (eq93_e1350_d_n6 + eq93_e1355_d_n6);
        let eq93_e1356_d_n7: f64 = (eq93_e1350_d_n7 + eq93_e1355_d_n7);
        let eq93_e1356_d_n8: f64 = (eq93_e1350_d_n8 + eq93_e1355_d_n8);
        let eq93_e1356_d_n9: f64 = (eq93_e1350_d_n9 + eq93_e1355_d_n9);
        let eq93_e1356_d_n10: f64 = (eq93_e1350_d_n10 + eq93_e1355_d_n10);
        let eq93_e1356_d_n11: f64 = (eq93_e1350_d_n11 + eq93_e1355_d_n11);
        let eq93_e1356_d_n12: f64 = (eq93_e1350_d_n12 + eq93_e1355_d_n12);
        let eq93_e1356_d_n13: f64 = (eq93_e1350_d_n13 + eq93_e1355_d_n13);
        let eq93_e1356_d_n14: f64 = (eq93_e1350_d_n14 + eq93_e1355_d_n14);
        let eq93_e1356_d_n15: f64 = (eq93_e1350_d_n15 + eq93_e1355_d_n15);
        let eq93_e1356_d_n16: f64 = (eq93_e1350_d_n16 + eq93_e1355_d_n16);
        let eq93_e1356_d_n17: f64 = (eq93_e1350_d_n17 + eq93_e1355_d_n17);
        let eq93_e1356_d_n18: f64 = (eq93_e1350_d_n18 + eq93_e1355_d_n18);
        let eq93_e1356_d_n19: f64 = (eq93_e1350_d_n19 + eq93_e1355_d_n19);
        let eq93_e1356_d_n20: f64 = (eq93_e1350_d_n20 + eq93_e1355_d_n20);
        let eq93_e1356_d_n21: f64 = (eq93_e1350_d_n21 + eq93_e1355_d_n21);
        let eq93_e1356_d_n22: f64 = (eq93_e1350_d_n22 + eq93_e1355_d_n22);
        (eq93_e1356, eq93_e1356_d_n0, eq93_e1356_d_n1, eq93_e1356_d_n2, eq93_e1356_d_n3, eq93_e1356_d_n4, eq93_e1356_d_n5, eq93_e1356_d_n6, eq93_e1356_d_n7, eq93_e1356_d_n8, eq93_e1356_d_n9, eq93_e1356_d_n10, eq93_e1356_d_n11, eq93_e1356_d_n12, eq93_e1356_d_n13, eq93_e1356_d_n14, eq93_e1356_d_n15, eq93_e1356_d_n16, eq93_e1356_d_n17, eq93_e1356_d_n18, eq93_e1356_d_n19, eq93_e1356_d_n20, eq93_e1356_d_n21, eq93_e1356_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq93_value: f64 = eq93_e1358;
        let eq93_node_derivatives: [f64; 23] = [eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22];
        let eq93_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            multiplicity * (eq93_value),
            nodes,
            &eq93_node_derivatives,
            branches,
            &eq93_branch_derivatives,
            multiplicity,
        );
        let (eq94_e1365,) = {
    if (s.b[523] && (!s.b[524])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e1365;
        stamper.stamp_potential_const(
            branches[48],
            eq94_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq95_e1370,) = {
    if (!s.b[523]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq95_value: f64 = eq95_e1370;
        stamper.stamp_potential_const(
            branches[49],
            eq95_value,
        );
        let (eq96_e1386, eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22,) = {
    if (s.b[538] && s.b[539]) {
        let eq96_e1376: f64 = (p.p6 * s.v[76]);
        let eq96_e1376_d_n0: f64 = (p.p6 * s.dn[76][0]);
        let eq96_e1376_d_n1: f64 = (p.p6 * s.dn[76][1]);
        let eq96_e1376_d_n2: f64 = (p.p6 * s.dn[76][2]);
        let eq96_e1376_d_n3: f64 = (p.p6 * s.dn[76][3]);
        let eq96_e1376_d_n4: f64 = (p.p6 * s.dn[76][4]);
        let eq96_e1376_d_n5: f64 = (p.p6 * s.dn[76][5]);
        let eq96_e1376_d_n6: f64 = (p.p6 * s.dn[76][6]);
        let eq96_e1376_d_n7: f64 = (p.p6 * s.dn[76][7]);
        let eq96_e1376_d_n8: f64 = (p.p6 * s.dn[76][8]);
        let eq96_e1376_d_n9: f64 = (p.p6 * s.dn[76][9]);
        let eq96_e1376_d_n10: f64 = (p.p6 * s.dn[76][10]);
        let eq96_e1376_d_n11: f64 = (p.p6 * s.dn[76][11]);
        let eq96_e1376_d_n12: f64 = (p.p6 * s.dn[76][12]);
        let eq96_e1376_d_n13: f64 = (p.p6 * s.dn[76][13]);
        let eq96_e1376_d_n14: f64 = (p.p6 * s.dn[76][14]);
        let eq96_e1376_d_n15: f64 = (p.p6 * s.dn[76][15]);
        let eq96_e1376_d_n16: f64 = (p.p6 * s.dn[76][16]);
        let eq96_e1376_d_n17: f64 = (p.p6 * s.dn[76][17]);
        let eq96_e1376_d_n18: f64 = (p.p6 * s.dn[76][18]);
        let eq96_e1376_d_n19: f64 = (p.p6 * s.dn[76][19]);
        let eq96_e1376_d_n20: f64 = (p.p6 * s.dn[76][20]);
        let eq96_e1376_d_n21: f64 = (p.p6 * s.dn[76][21]);
        let eq96_e1376_d_n22: f64 = (p.p6 * s.dn[76][22]);
        let eq96_e1378: f64 = (eq96_e1376 * s.v[317]);
        let eq96_e1378_d_n0: f64 = ((eq96_e1376_d_n0 * s.v[317]) + (eq96_e1376 * s.dn[317][0]));
        let eq96_e1378_d_n1: f64 = ((eq96_e1376_d_n1 * s.v[317]) + (eq96_e1376 * s.dn[317][1]));
        let eq96_e1378_d_n2: f64 = ((eq96_e1376_d_n2 * s.v[317]) + (eq96_e1376 * s.dn[317][2]));
        let eq96_e1378_d_n3: f64 = ((eq96_e1376_d_n3 * s.v[317]) + (eq96_e1376 * s.dn[317][3]));
        let eq96_e1378_d_n4: f64 = ((eq96_e1376_d_n4 * s.v[317]) + (eq96_e1376 * s.dn[317][4]));
        let eq96_e1378_d_n5: f64 = ((eq96_e1376_d_n5 * s.v[317]) + (eq96_e1376 * s.dn[317][5]));
        let eq96_e1378_d_n6: f64 = ((eq96_e1376_d_n6 * s.v[317]) + (eq96_e1376 * s.dn[317][6]));
        let eq96_e1378_d_n7: f64 = ((eq96_e1376_d_n7 * s.v[317]) + (eq96_e1376 * s.dn[317][7]));
        let eq96_e1378_d_n8: f64 = ((eq96_e1376_d_n8 * s.v[317]) + (eq96_e1376 * s.dn[317][8]));
        let eq96_e1378_d_n9: f64 = ((eq96_e1376_d_n9 * s.v[317]) + (eq96_e1376 * s.dn[317][9]));
        let eq96_e1378_d_n10: f64 = ((eq96_e1376_d_n10 * s.v[317]) + (eq96_e1376 * s.dn[317][10]));
        let eq96_e1378_d_n11: f64 = ((eq96_e1376_d_n11 * s.v[317]) + (eq96_e1376 * s.dn[317][11]));
        let eq96_e1378_d_n12: f64 = ((eq96_e1376_d_n12 * s.v[317]) + (eq96_e1376 * s.dn[317][12]));
        let eq96_e1378_d_n13: f64 = ((eq96_e1376_d_n13 * s.v[317]) + (eq96_e1376 * s.dn[317][13]));
        let eq96_e1378_d_n14: f64 = ((eq96_e1376_d_n14 * s.v[317]) + (eq96_e1376 * s.dn[317][14]));
        let eq96_e1378_d_n15: f64 = ((eq96_e1376_d_n15 * s.v[317]) + (eq96_e1376 * s.dn[317][15]));
        let eq96_e1378_d_n16: f64 = ((eq96_e1376_d_n16 * s.v[317]) + (eq96_e1376 * s.dn[317][16]));
        let eq96_e1378_d_n17: f64 = ((eq96_e1376_d_n17 * s.v[317]) + (eq96_e1376 * s.dn[317][17]));
        let eq96_e1378_d_n18: f64 = ((eq96_e1376_d_n18 * s.v[317]) + (eq96_e1376 * s.dn[317][18]));
        let eq96_e1378_d_n19: f64 = ((eq96_e1376_d_n19 * s.v[317]) + (eq96_e1376 * s.dn[317][19]));
        let eq96_e1378_d_n20: f64 = ((eq96_e1376_d_n20 * s.v[317]) + (eq96_e1376 * s.dn[317][20]));
        let eq96_e1378_d_n21: f64 = ((eq96_e1376_d_n21 * s.v[317]) + (eq96_e1376 * s.dn[317][21]));
        let eq96_e1378_d_n22: f64 = ((eq96_e1376_d_n22 * s.v[317]) + (eq96_e1376 * s.dn[317][22]));
        let eq96_e1381: f64 = (p.p6 * s.v[379]);
        let eq96_e1381_d_n0: f64 = (p.p6 * s.dn[379][0]);
        let eq96_e1381_d_n1: f64 = (p.p6 * s.dn[379][1]);
        let eq96_e1381_d_n2: f64 = (p.p6 * s.dn[379][2]);
        let eq96_e1381_d_n3: f64 = (p.p6 * s.dn[379][3]);
        let eq96_e1381_d_n4: f64 = (p.p6 * s.dn[379][4]);
        let eq96_e1381_d_n5: f64 = (p.p6 * s.dn[379][5]);
        let eq96_e1381_d_n6: f64 = (p.p6 * s.dn[379][6]);
        let eq96_e1381_d_n7: f64 = (p.p6 * s.dn[379][7]);
        let eq96_e1381_d_n8: f64 = (p.p6 * s.dn[379][8]);
        let eq96_e1381_d_n9: f64 = (p.p6 * s.dn[379][9]);
        let eq96_e1381_d_n10: f64 = (p.p6 * s.dn[379][10]);
        let eq96_e1381_d_n11: f64 = (p.p6 * s.dn[379][11]);
        let eq96_e1381_d_n12: f64 = (p.p6 * s.dn[379][12]);
        let eq96_e1381_d_n13: f64 = (p.p6 * s.dn[379][13]);
        let eq96_e1381_d_n14: f64 = (p.p6 * s.dn[379][14]);
        let eq96_e1381_d_n15: f64 = (p.p6 * s.dn[379][15]);
        let eq96_e1381_d_n16: f64 = (p.p6 * s.dn[379][16]);
        let eq96_e1381_d_n17: f64 = (p.p6 * s.dn[379][17]);
        let eq96_e1381_d_n18: f64 = (p.p6 * s.dn[379][18]);
        let eq96_e1381_d_n19: f64 = (p.p6 * s.dn[379][19]);
        let eq96_e1381_d_n20: f64 = (p.p6 * s.dn[379][20]);
        let eq96_e1381_d_n21: f64 = (p.p6 * s.dn[379][21]);
        let eq96_e1381_d_n22: f64 = (p.p6 * s.dn[379][22]);
        let eq96_e1383: f64 = (eq96_e1381 * (nv21 - nv22));
        let eq96_e1383_d_n0: f64 = (eq96_e1381_d_n0 * (nv21 - nv22));
        let eq96_e1383_d_n1: f64 = (eq96_e1381_d_n1 * (nv21 - nv22));
        let eq96_e1383_d_n2: f64 = (eq96_e1381_d_n2 * (nv21 - nv22));
        let eq96_e1383_d_n3: f64 = (eq96_e1381_d_n3 * (nv21 - nv22));
        let eq96_e1383_d_n4: f64 = (eq96_e1381_d_n4 * (nv21 - nv22));
        let eq96_e1383_d_n5: f64 = (eq96_e1381_d_n5 * (nv21 - nv22));
        let eq96_e1383_d_n6: f64 = (eq96_e1381_d_n6 * (nv21 - nv22));
        let eq96_e1383_d_n7: f64 = (eq96_e1381_d_n7 * (nv21 - nv22));
        let eq96_e1383_d_n8: f64 = (eq96_e1381_d_n8 * (nv21 - nv22));
        let eq96_e1383_d_n9: f64 = (eq96_e1381_d_n9 * (nv21 - nv22));
        let eq96_e1383_d_n10: f64 = (eq96_e1381_d_n10 * (nv21 - nv22));
        let eq96_e1383_d_n11: f64 = (eq96_e1381_d_n11 * (nv21 - nv22));
        let eq96_e1383_d_n12: f64 = (eq96_e1381_d_n12 * (nv21 - nv22));
        let eq96_e1383_d_n13: f64 = (eq96_e1381_d_n13 * (nv21 - nv22));
        let eq96_e1383_d_n14: f64 = (eq96_e1381_d_n14 * (nv21 - nv22));
        let eq96_e1383_d_n15: f64 = (eq96_e1381_d_n15 * (nv21 - nv22));
        let eq96_e1383_d_n16: f64 = (eq96_e1381_d_n16 * (nv21 - nv22));
        let eq96_e1383_d_n17: f64 = (eq96_e1381_d_n17 * (nv21 - nv22));
        let eq96_e1383_d_n18: f64 = (eq96_e1381_d_n18 * (nv21 - nv22));
        let eq96_e1383_d_n19: f64 = (eq96_e1381_d_n19 * (nv21 - nv22));
        let eq96_e1383_d_n20: f64 = (eq96_e1381_d_n20 * (nv21 - nv22));
        let eq96_e1383_d_n21: f64 = ((eq96_e1381_d_n21 * (nv21 - nv22)) + eq96_e1381);
        let eq96_e1383_d_n22: f64 = ((eq96_e1381_d_n22 * (nv21 - nv22)) + (-eq96_e1381));
        let eq96_e1384: f64 = (eq96_e1378 + eq96_e1383);
        let eq96_e1384_d_n0: f64 = (eq96_e1378_d_n0 + eq96_e1383_d_n0);
        let eq96_e1384_d_n1: f64 = (eq96_e1378_d_n1 + eq96_e1383_d_n1);
        let eq96_e1384_d_n2: f64 = (eq96_e1378_d_n2 + eq96_e1383_d_n2);
        let eq96_e1384_d_n3: f64 = (eq96_e1378_d_n3 + eq96_e1383_d_n3);
        let eq96_e1384_d_n4: f64 = (eq96_e1378_d_n4 + eq96_e1383_d_n4);
        let eq96_e1384_d_n5: f64 = (eq96_e1378_d_n5 + eq96_e1383_d_n5);
        let eq96_e1384_d_n6: f64 = (eq96_e1378_d_n6 + eq96_e1383_d_n6);
        let eq96_e1384_d_n7: f64 = (eq96_e1378_d_n7 + eq96_e1383_d_n7);
        let eq96_e1384_d_n8: f64 = (eq96_e1378_d_n8 + eq96_e1383_d_n8);
        let eq96_e1384_d_n9: f64 = (eq96_e1378_d_n9 + eq96_e1383_d_n9);
        let eq96_e1384_d_n10: f64 = (eq96_e1378_d_n10 + eq96_e1383_d_n10);
        let eq96_e1384_d_n11: f64 = (eq96_e1378_d_n11 + eq96_e1383_d_n11);
        let eq96_e1384_d_n12: f64 = (eq96_e1378_d_n12 + eq96_e1383_d_n12);
        let eq96_e1384_d_n13: f64 = (eq96_e1378_d_n13 + eq96_e1383_d_n13);
        let eq96_e1384_d_n14: f64 = (eq96_e1378_d_n14 + eq96_e1383_d_n14);
        let eq96_e1384_d_n15: f64 = (eq96_e1378_d_n15 + eq96_e1383_d_n15);
        let eq96_e1384_d_n16: f64 = (eq96_e1378_d_n16 + eq96_e1383_d_n16);
        let eq96_e1384_d_n17: f64 = (eq96_e1378_d_n17 + eq96_e1383_d_n17);
        let eq96_e1384_d_n18: f64 = (eq96_e1378_d_n18 + eq96_e1383_d_n18);
        let eq96_e1384_d_n19: f64 = (eq96_e1378_d_n19 + eq96_e1383_d_n19);
        let eq96_e1384_d_n20: f64 = (eq96_e1378_d_n20 + eq96_e1383_d_n20);
        let eq96_e1384_d_n21: f64 = (eq96_e1378_d_n21 + eq96_e1383_d_n21);
        let eq96_e1384_d_n22: f64 = (eq96_e1378_d_n22 + eq96_e1383_d_n22);
        (eq96_e1384, eq96_e1384_d_n0, eq96_e1384_d_n1, eq96_e1384_d_n2, eq96_e1384_d_n3, eq96_e1384_d_n4, eq96_e1384_d_n5, eq96_e1384_d_n6, eq96_e1384_d_n7, eq96_e1384_d_n8, eq96_e1384_d_n9, eq96_e1384_d_n10, eq96_e1384_d_n11, eq96_e1384_d_n12, eq96_e1384_d_n13, eq96_e1384_d_n14, eq96_e1384_d_n15, eq96_e1384_d_n16, eq96_e1384_d_n17, eq96_e1384_d_n18, eq96_e1384_d_n19, eq96_e1384_d_n20, eq96_e1384_d_n21, eq96_e1384_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1386;
        let eq96_node_derivatives: [f64; 23] = [eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[21]),
            Some(nodes[22]),
            multiplicity * (eq96_value),
            nodes,
            &eq96_node_derivatives,
            branches,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let (eq97_e1393,) = {
    if (s.b[538] && (!s.b[539])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq97_value: f64 = eq97_e1393;
        stamper.stamp_potential_const(
            branches[50],
            eq97_value,
        );
        let (eq98_e1398,) = {
    if (!s.b[538]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq98_value: f64 = eq98_e1398;
        stamper.stamp_potential_const(
            branches[51],
            eq98_value,
        );
        let (eq99_e1406,) = {
    if ((!s.b[538]) && (!s.b[547])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq99_value: f64 = eq99_e1406;
        stamper.stamp_potential_const(
            branches[52],
            eq99_value,
        );
        let (eq100_e1414, eq100_e1414_d_n0, eq100_e1414_d_n1, eq100_e1414_d_n2, eq100_e1414_d_n3, eq100_e1414_d_n4, eq100_e1414_d_n5, eq100_e1414_d_n6, eq100_e1414_d_n7, eq100_e1414_d_n8, eq100_e1414_d_n9, eq100_e1414_d_n10, eq100_e1414_d_n11, eq100_e1414_d_n12, eq100_e1414_d_n13, eq100_e1414_d_n14, eq100_e1414_d_n15, eq100_e1414_d_n16, eq100_e1414_d_n17, eq100_e1414_d_n18, eq100_e1414_d_n19, eq100_e1414_d_n20, eq100_e1414_d_n21, eq100_e1414_d_n22,) = {
    if s.b[553] {
        let eq100_e1410: f64 = (p.p6 * s.v[318]);
        let eq100_e1410_d_n0: f64 = (p.p6 * s.dn[318][0]);
        let eq100_e1410_d_n1: f64 = (p.p6 * s.dn[318][1]);
        let eq100_e1410_d_n2: f64 = (p.p6 * s.dn[318][2]);
        let eq100_e1410_d_n3: f64 = (p.p6 * s.dn[318][3]);
        let eq100_e1410_d_n4: f64 = (p.p6 * s.dn[318][4]);
        let eq100_e1410_d_n5: f64 = (p.p6 * s.dn[318][5]);
        let eq100_e1410_d_n6: f64 = (p.p6 * s.dn[318][6]);
        let eq100_e1410_d_n7: f64 = (p.p6 * s.dn[318][7]);
        let eq100_e1410_d_n8: f64 = (p.p6 * s.dn[318][8]);
        let eq100_e1410_d_n9: f64 = (p.p6 * s.dn[318][9]);
        let eq100_e1410_d_n10: f64 = (p.p6 * s.dn[318][10]);
        let eq100_e1410_d_n11: f64 = (p.p6 * s.dn[318][11]);
        let eq100_e1410_d_n12: f64 = (p.p6 * s.dn[318][12]);
        let eq100_e1410_d_n13: f64 = (p.p6 * s.dn[318][13]);
        let eq100_e1410_d_n14: f64 = (p.p6 * s.dn[318][14]);
        let eq100_e1410_d_n15: f64 = (p.p6 * s.dn[318][15]);
        let eq100_e1410_d_n16: f64 = (p.p6 * s.dn[318][16]);
        let eq100_e1410_d_n17: f64 = (p.p6 * s.dn[318][17]);
        let eq100_e1410_d_n18: f64 = (p.p6 * s.dn[318][18]);
        let eq100_e1410_d_n19: f64 = (p.p6 * s.dn[318][19]);
        let eq100_e1410_d_n20: f64 = (p.p6 * s.dn[318][20]);
        let eq100_e1410_d_n21: f64 = (p.p6 * s.dn[318][21]);
        let eq100_e1410_d_n22: f64 = (p.p6 * s.dn[318][22]);
        let eq100_e1412: f64 = (eq100_e1410 * (nv1 - nv9));
        let eq100_e1412_d_n0: f64 = (eq100_e1410_d_n0 * (nv1 - nv9));
        let eq100_e1412_d_n1: f64 = ((eq100_e1410_d_n1 * (nv1 - nv9)) + eq100_e1410);
        let eq100_e1412_d_n2: f64 = (eq100_e1410_d_n2 * (nv1 - nv9));
        let eq100_e1412_d_n3: f64 = (eq100_e1410_d_n3 * (nv1 - nv9));
        let eq100_e1412_d_n4: f64 = (eq100_e1410_d_n4 * (nv1 - nv9));
        let eq100_e1412_d_n5: f64 = (eq100_e1410_d_n5 * (nv1 - nv9));
        let eq100_e1412_d_n6: f64 = (eq100_e1410_d_n6 * (nv1 - nv9));
        let eq100_e1412_d_n7: f64 = (eq100_e1410_d_n7 * (nv1 - nv9));
        let eq100_e1412_d_n8: f64 = (eq100_e1410_d_n8 * (nv1 - nv9));
        let eq100_e1412_d_n9: f64 = ((eq100_e1410_d_n9 * (nv1 - nv9)) + (-eq100_e1410));
        let eq100_e1412_d_n10: f64 = (eq100_e1410_d_n10 * (nv1 - nv9));
        let eq100_e1412_d_n11: f64 = (eq100_e1410_d_n11 * (nv1 - nv9));
        let eq100_e1412_d_n12: f64 = (eq100_e1410_d_n12 * (nv1 - nv9));
        let eq100_e1412_d_n13: f64 = (eq100_e1410_d_n13 * (nv1 - nv9));
        let eq100_e1412_d_n14: f64 = (eq100_e1410_d_n14 * (nv1 - nv9));
        let eq100_e1412_d_n15: f64 = (eq100_e1410_d_n15 * (nv1 - nv9));
        let eq100_e1412_d_n16: f64 = (eq100_e1410_d_n16 * (nv1 - nv9));
        let eq100_e1412_d_n17: f64 = (eq100_e1410_d_n17 * (nv1 - nv9));
        let eq100_e1412_d_n18: f64 = (eq100_e1410_d_n18 * (nv1 - nv9));
        let eq100_e1412_d_n19: f64 = (eq100_e1410_d_n19 * (nv1 - nv9));
        let eq100_e1412_d_n20: f64 = (eq100_e1410_d_n20 * (nv1 - nv9));
        let eq100_e1412_d_n21: f64 = (eq100_e1410_d_n21 * (nv1 - nv9));
        let eq100_e1412_d_n22: f64 = (eq100_e1410_d_n22 * (nv1 - nv9));
        (eq100_e1412, eq100_e1412_d_n0, eq100_e1412_d_n1, eq100_e1412_d_n2, eq100_e1412_d_n3, eq100_e1412_d_n4, eq100_e1412_d_n5, eq100_e1412_d_n6, eq100_e1412_d_n7, eq100_e1412_d_n8, eq100_e1412_d_n9, eq100_e1412_d_n10, eq100_e1412_d_n11, eq100_e1412_d_n12, eq100_e1412_d_n13, eq100_e1412_d_n14, eq100_e1412_d_n15, eq100_e1412_d_n16, eq100_e1412_d_n17, eq100_e1412_d_n18, eq100_e1412_d_n19, eq100_e1412_d_n20, eq100_e1412_d_n21, eq100_e1412_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1414;
        let eq100_node_derivatives: [f64; 23] = [eq100_e1414_d_n0, eq100_e1414_d_n1, eq100_e1414_d_n2, eq100_e1414_d_n3, eq100_e1414_d_n4, eq100_e1414_d_n5, eq100_e1414_d_n6, eq100_e1414_d_n7, eq100_e1414_d_n8, eq100_e1414_d_n9, eq100_e1414_d_n10, eq100_e1414_d_n11, eq100_e1414_d_n12, eq100_e1414_d_n13, eq100_e1414_d_n14, eq100_e1414_d_n15, eq100_e1414_d_n16, eq100_e1414_d_n17, eq100_e1414_d_n18, eq100_e1414_d_n19, eq100_e1414_d_n20, eq100_e1414_d_n21, eq100_e1414_d_n22];
        let eq100_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            multiplicity * (eq100_value),
            nodes,
            &eq100_node_derivatives,
            branches,
            &eq100_branch_derivatives,
            multiplicity,
        );
        let (eq101_e1418,) = {
    if s.b[553] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e1418;
        stamper.stamp_potential_const(
            branches[53],
            eq101_value,
        );
        let (eq102_e1429, eq102_e1429_d_n0, eq102_e1429_d_n1, eq102_e1429_d_n2, eq102_e1429_d_n3, eq102_e1429_d_n4, eq102_e1429_d_n5, eq102_e1429_d_n6, eq102_e1429_d_n7, eq102_e1429_d_n8, eq102_e1429_d_n9, eq102_e1429_d_n10, eq102_e1429_d_n11, eq102_e1429_d_n12, eq102_e1429_d_n13, eq102_e1429_d_n14, eq102_e1429_d_n15, eq102_e1429_d_n16, eq102_e1429_d_n17, eq102_e1429_d_n18, eq102_e1429_d_n19, eq102_e1429_d_n20, eq102_e1429_d_n21, eq102_e1429_d_n22,) = {
    if ((!s.b[553]) && s.b[555]) {
        let eq102_e1425: f64 = (p.p6 * s.v[319]);
        let eq102_e1425_d_n0: f64 = (p.p6 * s.dn[319][0]);
        let eq102_e1425_d_n1: f64 = (p.p6 * s.dn[319][1]);
        let eq102_e1425_d_n2: f64 = (p.p6 * s.dn[319][2]);
        let eq102_e1425_d_n3: f64 = (p.p6 * s.dn[319][3]);
        let eq102_e1425_d_n4: f64 = (p.p6 * s.dn[319][4]);
        let eq102_e1425_d_n5: f64 = (p.p6 * s.dn[319][5]);
        let eq102_e1425_d_n6: f64 = (p.p6 * s.dn[319][6]);
        let eq102_e1425_d_n7: f64 = (p.p6 * s.dn[319][7]);
        let eq102_e1425_d_n8: f64 = (p.p6 * s.dn[319][8]);
        let eq102_e1425_d_n9: f64 = (p.p6 * s.dn[319][9]);
        let eq102_e1425_d_n10: f64 = (p.p6 * s.dn[319][10]);
        let eq102_e1425_d_n11: f64 = (p.p6 * s.dn[319][11]);
        let eq102_e1425_d_n12: f64 = (p.p6 * s.dn[319][12]);
        let eq102_e1425_d_n13: f64 = (p.p6 * s.dn[319][13]);
        let eq102_e1425_d_n14: f64 = (p.p6 * s.dn[319][14]);
        let eq102_e1425_d_n15: f64 = (p.p6 * s.dn[319][15]);
        let eq102_e1425_d_n16: f64 = (p.p6 * s.dn[319][16]);
        let eq102_e1425_d_n17: f64 = (p.p6 * s.dn[319][17]);
        let eq102_e1425_d_n18: f64 = (p.p6 * s.dn[319][18]);
        let eq102_e1425_d_n19: f64 = (p.p6 * s.dn[319][19]);
        let eq102_e1425_d_n20: f64 = (p.p6 * s.dn[319][20]);
        let eq102_e1425_d_n21: f64 = (p.p6 * s.dn[319][21]);
        let eq102_e1425_d_n22: f64 = (p.p6 * s.dn[319][22]);
        let eq102_e1427: f64 = (eq102_e1425 * (nv1 - nv10));
        let eq102_e1427_d_n0: f64 = (eq102_e1425_d_n0 * (nv1 - nv10));
        let eq102_e1427_d_n1: f64 = ((eq102_e1425_d_n1 * (nv1 - nv10)) + eq102_e1425);
        let eq102_e1427_d_n2: f64 = (eq102_e1425_d_n2 * (nv1 - nv10));
        let eq102_e1427_d_n3: f64 = (eq102_e1425_d_n3 * (nv1 - nv10));
        let eq102_e1427_d_n4: f64 = (eq102_e1425_d_n4 * (nv1 - nv10));
        let eq102_e1427_d_n5: f64 = (eq102_e1425_d_n5 * (nv1 - nv10));
        let eq102_e1427_d_n6: f64 = (eq102_e1425_d_n6 * (nv1 - nv10));
        let eq102_e1427_d_n7: f64 = (eq102_e1425_d_n7 * (nv1 - nv10));
        let eq102_e1427_d_n8: f64 = (eq102_e1425_d_n8 * (nv1 - nv10));
        let eq102_e1427_d_n9: f64 = (eq102_e1425_d_n9 * (nv1 - nv10));
        let eq102_e1427_d_n10: f64 = ((eq102_e1425_d_n10 * (nv1 - nv10)) + (-eq102_e1425));
        let eq102_e1427_d_n11: f64 = (eq102_e1425_d_n11 * (nv1 - nv10));
        let eq102_e1427_d_n12: f64 = (eq102_e1425_d_n12 * (nv1 - nv10));
        let eq102_e1427_d_n13: f64 = (eq102_e1425_d_n13 * (nv1 - nv10));
        let eq102_e1427_d_n14: f64 = (eq102_e1425_d_n14 * (nv1 - nv10));
        let eq102_e1427_d_n15: f64 = (eq102_e1425_d_n15 * (nv1 - nv10));
        let eq102_e1427_d_n16: f64 = (eq102_e1425_d_n16 * (nv1 - nv10));
        let eq102_e1427_d_n17: f64 = (eq102_e1425_d_n17 * (nv1 - nv10));
        let eq102_e1427_d_n18: f64 = (eq102_e1425_d_n18 * (nv1 - nv10));
        let eq102_e1427_d_n19: f64 = (eq102_e1425_d_n19 * (nv1 - nv10));
        let eq102_e1427_d_n20: f64 = (eq102_e1425_d_n20 * (nv1 - nv10));
        let eq102_e1427_d_n21: f64 = (eq102_e1425_d_n21 * (nv1 - nv10));
        let eq102_e1427_d_n22: f64 = (eq102_e1425_d_n22 * (nv1 - nv10));
        (eq102_e1427, eq102_e1427_d_n0, eq102_e1427_d_n1, eq102_e1427_d_n2, eq102_e1427_d_n3, eq102_e1427_d_n4, eq102_e1427_d_n5, eq102_e1427_d_n6, eq102_e1427_d_n7, eq102_e1427_d_n8, eq102_e1427_d_n9, eq102_e1427_d_n10, eq102_e1427_d_n11, eq102_e1427_d_n12, eq102_e1427_d_n13, eq102_e1427_d_n14, eq102_e1427_d_n15, eq102_e1427_d_n16, eq102_e1427_d_n17, eq102_e1427_d_n18, eq102_e1427_d_n19, eq102_e1427_d_n20, eq102_e1427_d_n21, eq102_e1427_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1429;
        let eq102_node_derivatives: [f64; 23] = [eq102_e1429_d_n0, eq102_e1429_d_n1, eq102_e1429_d_n2, eq102_e1429_d_n3, eq102_e1429_d_n4, eq102_e1429_d_n5, eq102_e1429_d_n6, eq102_e1429_d_n7, eq102_e1429_d_n8, eq102_e1429_d_n9, eq102_e1429_d_n10, eq102_e1429_d_n11, eq102_e1429_d_n12, eq102_e1429_d_n13, eq102_e1429_d_n14, eq102_e1429_d_n15, eq102_e1429_d_n16, eq102_e1429_d_n17, eq102_e1429_d_n18, eq102_e1429_d_n19, eq102_e1429_d_n20, eq102_e1429_d_n21, eq102_e1429_d_n22];
        let eq102_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            multiplicity * (eq102_value),
            nodes,
            &eq102_node_derivatives,
            branches,
            &eq102_branch_derivatives,
            multiplicity,
        );
        let (eq103_e1440, eq103_e1440_d_n0, eq103_e1440_d_n1, eq103_e1440_d_n2, eq103_e1440_d_n3, eq103_e1440_d_n4, eq103_e1440_d_n5, eq103_e1440_d_n6, eq103_e1440_d_n7, eq103_e1440_d_n8, eq103_e1440_d_n9, eq103_e1440_d_n10, eq103_e1440_d_n11, eq103_e1440_d_n12, eq103_e1440_d_n13, eq103_e1440_d_n14, eq103_e1440_d_n15, eq103_e1440_d_n16, eq103_e1440_d_n17, eq103_e1440_d_n18, eq103_e1440_d_n19, eq103_e1440_d_n20, eq103_e1440_d_n21, eq103_e1440_d_n22,) = {
    if ((!s.b[553]) && s.b[555]) {
        let eq103_e1436: f64 = (p.p6 * s.v[320]);
        let eq103_e1436_d_n0: f64 = (p.p6 * s.dn[320][0]);
        let eq103_e1436_d_n1: f64 = (p.p6 * s.dn[320][1]);
        let eq103_e1436_d_n2: f64 = (p.p6 * s.dn[320][2]);
        let eq103_e1436_d_n3: f64 = (p.p6 * s.dn[320][3]);
        let eq103_e1436_d_n4: f64 = (p.p6 * s.dn[320][4]);
        let eq103_e1436_d_n5: f64 = (p.p6 * s.dn[320][5]);
        let eq103_e1436_d_n6: f64 = (p.p6 * s.dn[320][6]);
        let eq103_e1436_d_n7: f64 = (p.p6 * s.dn[320][7]);
        let eq103_e1436_d_n8: f64 = (p.p6 * s.dn[320][8]);
        let eq103_e1436_d_n9: f64 = (p.p6 * s.dn[320][9]);
        let eq103_e1436_d_n10: f64 = (p.p6 * s.dn[320][10]);
        let eq103_e1436_d_n11: f64 = (p.p6 * s.dn[320][11]);
        let eq103_e1436_d_n12: f64 = (p.p6 * s.dn[320][12]);
        let eq103_e1436_d_n13: f64 = (p.p6 * s.dn[320][13]);
        let eq103_e1436_d_n14: f64 = (p.p6 * s.dn[320][14]);
        let eq103_e1436_d_n15: f64 = (p.p6 * s.dn[320][15]);
        let eq103_e1436_d_n16: f64 = (p.p6 * s.dn[320][16]);
        let eq103_e1436_d_n17: f64 = (p.p6 * s.dn[320][17]);
        let eq103_e1436_d_n18: f64 = (p.p6 * s.dn[320][18]);
        let eq103_e1436_d_n19: f64 = (p.p6 * s.dn[320][19]);
        let eq103_e1436_d_n20: f64 = (p.p6 * s.dn[320][20]);
        let eq103_e1436_d_n21: f64 = (p.p6 * s.dn[320][21]);
        let eq103_e1436_d_n22: f64 = (p.p6 * s.dn[320][22]);
        let eq103_e1438: f64 = (eq103_e1436 * (nv10 - nv9));
        let eq103_e1438_d_n0: f64 = (eq103_e1436_d_n0 * (nv10 - nv9));
        let eq103_e1438_d_n1: f64 = (eq103_e1436_d_n1 * (nv10 - nv9));
        let eq103_e1438_d_n2: f64 = (eq103_e1436_d_n2 * (nv10 - nv9));
        let eq103_e1438_d_n3: f64 = (eq103_e1436_d_n3 * (nv10 - nv9));
        let eq103_e1438_d_n4: f64 = (eq103_e1436_d_n4 * (nv10 - nv9));
        let eq103_e1438_d_n5: f64 = (eq103_e1436_d_n5 * (nv10 - nv9));
        let eq103_e1438_d_n6: f64 = (eq103_e1436_d_n6 * (nv10 - nv9));
        let eq103_e1438_d_n7: f64 = (eq103_e1436_d_n7 * (nv10 - nv9));
        let eq103_e1438_d_n8: f64 = (eq103_e1436_d_n8 * (nv10 - nv9));
        let eq103_e1438_d_n9: f64 = ((eq103_e1436_d_n9 * (nv10 - nv9)) + (-eq103_e1436));
        let eq103_e1438_d_n10: f64 = ((eq103_e1436_d_n10 * (nv10 - nv9)) + eq103_e1436);
        let eq103_e1438_d_n11: f64 = (eq103_e1436_d_n11 * (nv10 - nv9));
        let eq103_e1438_d_n12: f64 = (eq103_e1436_d_n12 * (nv10 - nv9));
        let eq103_e1438_d_n13: f64 = (eq103_e1436_d_n13 * (nv10 - nv9));
        let eq103_e1438_d_n14: f64 = (eq103_e1436_d_n14 * (nv10 - nv9));
        let eq103_e1438_d_n15: f64 = (eq103_e1436_d_n15 * (nv10 - nv9));
        let eq103_e1438_d_n16: f64 = (eq103_e1436_d_n16 * (nv10 - nv9));
        let eq103_e1438_d_n17: f64 = (eq103_e1436_d_n17 * (nv10 - nv9));
        let eq103_e1438_d_n18: f64 = (eq103_e1436_d_n18 * (nv10 - nv9));
        let eq103_e1438_d_n19: f64 = (eq103_e1436_d_n19 * (nv10 - nv9));
        let eq103_e1438_d_n20: f64 = (eq103_e1436_d_n20 * (nv10 - nv9));
        let eq103_e1438_d_n21: f64 = (eq103_e1436_d_n21 * (nv10 - nv9));
        let eq103_e1438_d_n22: f64 = (eq103_e1436_d_n22 * (nv10 - nv9));
        (eq103_e1438, eq103_e1438_d_n0, eq103_e1438_d_n1, eq103_e1438_d_n2, eq103_e1438_d_n3, eq103_e1438_d_n4, eq103_e1438_d_n5, eq103_e1438_d_n6, eq103_e1438_d_n7, eq103_e1438_d_n8, eq103_e1438_d_n9, eq103_e1438_d_n10, eq103_e1438_d_n11, eq103_e1438_d_n12, eq103_e1438_d_n13, eq103_e1438_d_n14, eq103_e1438_d_n15, eq103_e1438_d_n16, eq103_e1438_d_n17, eq103_e1438_d_n18, eq103_e1438_d_n19, eq103_e1438_d_n20, eq103_e1438_d_n21, eq103_e1438_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1440;
        let eq103_node_derivatives: [f64; 23] = [eq103_e1440_d_n0, eq103_e1440_d_n1, eq103_e1440_d_n2, eq103_e1440_d_n3, eq103_e1440_d_n4, eq103_e1440_d_n5, eq103_e1440_d_n6, eq103_e1440_d_n7, eq103_e1440_d_n8, eq103_e1440_d_n9, eq103_e1440_d_n10, eq103_e1440_d_n11, eq103_e1440_d_n12, eq103_e1440_d_n13, eq103_e1440_d_n14, eq103_e1440_d_n15, eq103_e1440_d_n16, eq103_e1440_d_n17, eq103_e1440_d_n18, eq103_e1440_d_n19, eq103_e1440_d_n20, eq103_e1440_d_n21, eq103_e1440_d_n22];
        let eq103_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
            multiplicity * (eq103_value),
            nodes,
            &eq103_node_derivatives,
            branches,
            &eq103_branch_derivatives,
            multiplicity,
        );
        let (eq104_e1448,) = {
    if ((!s.b[553]) && (!s.b[555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq104_value: f64 = eq104_e1448;
        stamper.stamp_potential_const(
            branches[54],
            eq104_value,
        );
        let (eq105_e1456,) = {
    if ((!s.b[553]) && (!s.b[555])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq105_value: f64 = eq105_e1456;
        stamper.stamp_potential_const(
            branches[55],
            eq105_value,
        );
        let eq106_e1459: f64 = (p.p6 * s.v[369]);
        let eq106_e1459_d_n0: f64 = (p.p6 * s.dn[369][0]);
        let eq106_e1459_d_n1: f64 = (p.p6 * s.dn[369][1]);
        let eq106_e1459_d_n2: f64 = (p.p6 * s.dn[369][2]);
        let eq106_e1459_d_n3: f64 = (p.p6 * s.dn[369][3]);
        let eq106_e1459_d_n4: f64 = (p.p6 * s.dn[369][4]);
        let eq106_e1459_d_n5: f64 = (p.p6 * s.dn[369][5]);
        let eq106_e1459_d_n6: f64 = (p.p6 * s.dn[369][6]);
        let eq106_e1459_d_n7: f64 = (p.p6 * s.dn[369][7]);
        let eq106_e1459_d_n8: f64 = (p.p6 * s.dn[369][8]);
        let eq106_e1459_d_n9: f64 = (p.p6 * s.dn[369][9]);
        let eq106_e1459_d_n10: f64 = (p.p6 * s.dn[369][10]);
        let eq106_e1459_d_n11: f64 = (p.p6 * s.dn[369][11]);
        let eq106_e1459_d_n12: f64 = (p.p6 * s.dn[369][12]);
        let eq106_e1459_d_n13: f64 = (p.p6 * s.dn[369][13]);
        let eq106_e1459_d_n14: f64 = (p.p6 * s.dn[369][14]);
        let eq106_e1459_d_n15: f64 = (p.p6 * s.dn[369][15]);
        let eq106_e1459_d_n16: f64 = (p.p6 * s.dn[369][16]);
        let eq106_e1459_d_n17: f64 = (p.p6 * s.dn[369][17]);
        let eq106_e1459_d_n18: f64 = (p.p6 * s.dn[369][18]);
        let eq106_e1459_d_n19: f64 = (p.p6 * s.dn[369][19]);
        let eq106_e1459_d_n20: f64 = (p.p6 * s.dn[369][20]);
        let eq106_e1459_d_n21: f64 = (p.p6 * s.dn[369][21]);
        let eq106_e1459_d_n22: f64 = (p.p6 * s.dn[369][22]);
        let eq106_value: f64 = eq106_e1459;
        let eq106_node_derivatives: [f64; 23] = [eq106_e1459_d_n0, eq106_e1459_d_n1, eq106_e1459_d_n2, eq106_e1459_d_n3, eq106_e1459_d_n4, eq106_e1459_d_n5, eq106_e1459_d_n6, eq106_e1459_d_n7, eq106_e1459_d_n8, eq106_e1459_d_n9, eq106_e1459_d_n10, eq106_e1459_d_n11, eq106_e1459_d_n12, eq106_e1459_d_n13, eq106_e1459_d_n14, eq106_e1459_d_n15, eq106_e1459_d_n16, eq106_e1459_d_n17, eq106_e1459_d_n18, eq106_e1459_d_n19, eq106_e1459_d_n20, eq106_e1459_d_n21, eq106_e1459_d_n22];
        let eq106_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            multiplicity * (eq106_value),
            nodes,
            &eq106_node_derivatives,
            branches,
            &eq106_branch_derivatives,
            multiplicity,
        );
        let eq107_e1462: f64 = (p.p6 * s.v[370]);
        let eq107_e1462_d_n0: f64 = (p.p6 * s.dn[370][0]);
        let eq107_e1462_d_n1: f64 = (p.p6 * s.dn[370][1]);
        let eq107_e1462_d_n2: f64 = (p.p6 * s.dn[370][2]);
        let eq107_e1462_d_n3: f64 = (p.p6 * s.dn[370][3]);
        let eq107_e1462_d_n4: f64 = (p.p6 * s.dn[370][4]);
        let eq107_e1462_d_n5: f64 = (p.p6 * s.dn[370][5]);
        let eq107_e1462_d_n6: f64 = (p.p6 * s.dn[370][6]);
        let eq107_e1462_d_n7: f64 = (p.p6 * s.dn[370][7]);
        let eq107_e1462_d_n8: f64 = (p.p6 * s.dn[370][8]);
        let eq107_e1462_d_n9: f64 = (p.p6 * s.dn[370][9]);
        let eq107_e1462_d_n10: f64 = (p.p6 * s.dn[370][10]);
        let eq107_e1462_d_n11: f64 = (p.p6 * s.dn[370][11]);
        let eq107_e1462_d_n12: f64 = (p.p6 * s.dn[370][12]);
        let eq107_e1462_d_n13: f64 = (p.p6 * s.dn[370][13]);
        let eq107_e1462_d_n14: f64 = (p.p6 * s.dn[370][14]);
        let eq107_e1462_d_n15: f64 = (p.p6 * s.dn[370][15]);
        let eq107_e1462_d_n16: f64 = (p.p6 * s.dn[370][16]);
        let eq107_e1462_d_n17: f64 = (p.p6 * s.dn[370][17]);
        let eq107_e1462_d_n18: f64 = (p.p6 * s.dn[370][18]);
        let eq107_e1462_d_n19: f64 = (p.p6 * s.dn[370][19]);
        let eq107_e1462_d_n20: f64 = (p.p6 * s.dn[370][20]);
        let eq107_e1462_d_n21: f64 = (p.p6 * s.dn[370][21]);
        let eq107_e1462_d_n22: f64 = (p.p6 * s.dn[370][22]);
        let eq107_value: f64 = eq107_e1462;
        let eq107_node_derivatives: [f64; 23] = [eq107_e1462_d_n0, eq107_e1462_d_n1, eq107_e1462_d_n2, eq107_e1462_d_n3, eq107_e1462_d_n4, eq107_e1462_d_n5, eq107_e1462_d_n6, eq107_e1462_d_n7, eq107_e1462_d_n8, eq107_e1462_d_n9, eq107_e1462_d_n10, eq107_e1462_d_n11, eq107_e1462_d_n12, eq107_e1462_d_n13, eq107_e1462_d_n14, eq107_e1462_d_n15, eq107_e1462_d_n16, eq107_e1462_d_n17, eq107_e1462_d_n18, eq107_e1462_d_n19, eq107_e1462_d_n20, eq107_e1462_d_n21, eq107_e1462_d_n22];
        let eq107_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            multiplicity * (eq107_value),
            nodes,
            &eq107_node_derivatives,
            branches,
            &eq107_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
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
        let (eq108_e1471,) = {
    if s.b[567] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq108_value: f64 = eq108_e1471;
        stamper.stamp_current_const(
            Some(nodes[7]),
            Some(nodes[8]),
            multiplicity * (eq108_value),
        );
        let eq109_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[165]);
        let eq109_e1474_d_n0: f64 = (s.dn[165][0] * ddt_scale);
        let eq109_e1474_d_n1: f64 = (s.dn[165][1] * ddt_scale);
        let eq109_e1474_d_n2: f64 = (s.dn[165][2] * ddt_scale);
        let eq109_e1474_d_n3: f64 = (s.dn[165][3] * ddt_scale);
        let eq109_e1474_d_n4: f64 = (s.dn[165][4] * ddt_scale);
        let eq109_e1474_d_n5: f64 = (s.dn[165][5] * ddt_scale);
        let eq109_e1474_d_n6: f64 = (s.dn[165][6] * ddt_scale);
        let eq109_e1474_d_n7: f64 = (s.dn[165][7] * ddt_scale);
        let eq109_e1474_d_n8: f64 = (s.dn[165][8] * ddt_scale);
        let eq109_e1474_d_n9: f64 = (s.dn[165][9] * ddt_scale);
        let eq109_e1474_d_n10: f64 = (s.dn[165][10] * ddt_scale);
        let eq109_e1474_d_n11: f64 = (s.dn[165][11] * ddt_scale);
        let eq109_e1474_d_n12: f64 = (s.dn[165][12] * ddt_scale);
        let eq109_e1474_d_n13: f64 = (s.dn[165][13] * ddt_scale);
        let eq109_e1474_d_n14: f64 = (s.dn[165][14] * ddt_scale);
        let eq109_e1474_d_n15: f64 = (s.dn[165][15] * ddt_scale);
        let eq109_e1474_d_n16: f64 = (s.dn[165][16] * ddt_scale);
        let eq109_e1474_d_n17: f64 = (s.dn[165][17] * ddt_scale);
        let eq109_e1474_d_n18: f64 = (s.dn[165][18] * ddt_scale);
        let eq109_e1474_d_n19: f64 = (s.dn[165][19] * ddt_scale);
        let eq109_e1474_d_n20: f64 = (s.dn[165][20] * ddt_scale);
        let eq109_e1474_d_n21: f64 = (s.dn[165][21] * ddt_scale);
        let eq109_e1474_d_n22: f64 = (s.dn[165][22] * ddt_scale);
        let eq109_e1475: f64 = (p.p7 * eq109_e1474);
        let eq109_e1475_d_n0: f64 = (p.p7 * eq109_e1474_d_n0);
        let eq109_e1475_d_n1: f64 = (p.p7 * eq109_e1474_d_n1);
        let eq109_e1475_d_n2: f64 = (p.p7 * eq109_e1474_d_n2);
        let eq109_e1475_d_n3: f64 = (p.p7 * eq109_e1474_d_n3);
        let eq109_e1475_d_n4: f64 = (p.p7 * eq109_e1474_d_n4);
        let eq109_e1475_d_n5: f64 = (p.p7 * eq109_e1474_d_n5);
        let eq109_e1475_d_n6: f64 = (p.p7 * eq109_e1474_d_n6);
        let eq109_e1475_d_n7: f64 = (p.p7 * eq109_e1474_d_n7);
        let eq109_e1475_d_n8: f64 = (p.p7 * eq109_e1474_d_n8);
        let eq109_e1475_d_n9: f64 = (p.p7 * eq109_e1474_d_n9);
        let eq109_e1475_d_n10: f64 = (p.p7 * eq109_e1474_d_n10);
        let eq109_e1475_d_n11: f64 = (p.p7 * eq109_e1474_d_n11);
        let eq109_e1475_d_n12: f64 = (p.p7 * eq109_e1474_d_n12);
        let eq109_e1475_d_n13: f64 = (p.p7 * eq109_e1474_d_n13);
        let eq109_e1475_d_n14: f64 = (p.p7 * eq109_e1474_d_n14);
        let eq109_e1475_d_n15: f64 = (p.p7 * eq109_e1474_d_n15);
        let eq109_e1475_d_n16: f64 = (p.p7 * eq109_e1474_d_n16);
        let eq109_e1475_d_n17: f64 = (p.p7 * eq109_e1474_d_n17);
        let eq109_e1475_d_n18: f64 = (p.p7 * eq109_e1474_d_n18);
        let eq109_e1475_d_n19: f64 = (p.p7 * eq109_e1474_d_n19);
        let eq109_e1475_d_n20: f64 = (p.p7 * eq109_e1474_d_n20);
        let eq109_e1475_d_n21: f64 = (p.p7 * eq109_e1474_d_n21);
        let eq109_e1475_d_n22: f64 = (p.p7 * eq109_e1474_d_n22);
        let eq109_value: f64 = eq109_e1475;
        let eq109_node_derivatives: [f64; 23] = [eq109_e1475_d_n0, eq109_e1475_d_n1, eq109_e1475_d_n2, eq109_e1475_d_n3, eq109_e1475_d_n4, eq109_e1475_d_n5, eq109_e1475_d_n6, eq109_e1475_d_n7, eq109_e1475_d_n8, eq109_e1475_d_n9, eq109_e1475_d_n10, eq109_e1475_d_n11, eq109_e1475_d_n12, eq109_e1475_d_n13, eq109_e1475_d_n14, eq109_e1475_d_n15, eq109_e1475_d_n16, eq109_e1475_d_n17, eq109_e1475_d_n18, eq109_e1475_d_n19, eq109_e1475_d_n20, eq109_e1475_d_n21, eq109_e1475_d_n22];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            multiplicity * (eq109_value),
            nodes,
            &eq109_node_derivatives,
            branches,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let eq110_e1478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, s.v[161]);
        let eq110_e1478_d_n0: f64 = (s.dn[161][0] * ddt_scale);
        let eq110_e1478_d_n1: f64 = (s.dn[161][1] * ddt_scale);
        let eq110_e1478_d_n2: f64 = (s.dn[161][2] * ddt_scale);
        let eq110_e1478_d_n3: f64 = (s.dn[161][3] * ddt_scale);
        let eq110_e1478_d_n4: f64 = (s.dn[161][4] * ddt_scale);
        let eq110_e1478_d_n5: f64 = (s.dn[161][5] * ddt_scale);
        let eq110_e1478_d_n6: f64 = (s.dn[161][6] * ddt_scale);
        let eq110_e1478_d_n7: f64 = (s.dn[161][7] * ddt_scale);
        let eq110_e1478_d_n8: f64 = (s.dn[161][8] * ddt_scale);
        let eq110_e1478_d_n9: f64 = (s.dn[161][9] * ddt_scale);
        let eq110_e1478_d_n10: f64 = (s.dn[161][10] * ddt_scale);
        let eq110_e1478_d_n11: f64 = (s.dn[161][11] * ddt_scale);
        let eq110_e1478_d_n12: f64 = (s.dn[161][12] * ddt_scale);
        let eq110_e1478_d_n13: f64 = (s.dn[161][13] * ddt_scale);
        let eq110_e1478_d_n14: f64 = (s.dn[161][14] * ddt_scale);
        let eq110_e1478_d_n15: f64 = (s.dn[161][15] * ddt_scale);
        let eq110_e1478_d_n16: f64 = (s.dn[161][16] * ddt_scale);
        let eq110_e1478_d_n17: f64 = (s.dn[161][17] * ddt_scale);
        let eq110_e1478_d_n18: f64 = (s.dn[161][18] * ddt_scale);
        let eq110_e1478_d_n19: f64 = (s.dn[161][19] * ddt_scale);
        let eq110_e1478_d_n20: f64 = (s.dn[161][20] * ddt_scale);
        let eq110_e1478_d_n21: f64 = (s.dn[161][21] * ddt_scale);
        let eq110_e1478_d_n22: f64 = (s.dn[161][22] * ddt_scale);
        let eq110_e1479: f64 = (p.p7 * eq110_e1478);
        let eq110_e1479_d_n0: f64 = (p.p7 * eq110_e1478_d_n0);
        let eq110_e1479_d_n1: f64 = (p.p7 * eq110_e1478_d_n1);
        let eq110_e1479_d_n2: f64 = (p.p7 * eq110_e1478_d_n2);
        let eq110_e1479_d_n3: f64 = (p.p7 * eq110_e1478_d_n3);
        let eq110_e1479_d_n4: f64 = (p.p7 * eq110_e1478_d_n4);
        let eq110_e1479_d_n5: f64 = (p.p7 * eq110_e1478_d_n5);
        let eq110_e1479_d_n6: f64 = (p.p7 * eq110_e1478_d_n6);
        let eq110_e1479_d_n7: f64 = (p.p7 * eq110_e1478_d_n7);
        let eq110_e1479_d_n8: f64 = (p.p7 * eq110_e1478_d_n8);
        let eq110_e1479_d_n9: f64 = (p.p7 * eq110_e1478_d_n9);
        let eq110_e1479_d_n10: f64 = (p.p7 * eq110_e1478_d_n10);
        let eq110_e1479_d_n11: f64 = (p.p7 * eq110_e1478_d_n11);
        let eq110_e1479_d_n12: f64 = (p.p7 * eq110_e1478_d_n12);
        let eq110_e1479_d_n13: f64 = (p.p7 * eq110_e1478_d_n13);
        let eq110_e1479_d_n14: f64 = (p.p7 * eq110_e1478_d_n14);
        let eq110_e1479_d_n15: f64 = (p.p7 * eq110_e1478_d_n15);
        let eq110_e1479_d_n16: f64 = (p.p7 * eq110_e1478_d_n16);
        let eq110_e1479_d_n17: f64 = (p.p7 * eq110_e1478_d_n17);
        let eq110_e1479_d_n18: f64 = (p.p7 * eq110_e1478_d_n18);
        let eq110_e1479_d_n19: f64 = (p.p7 * eq110_e1478_d_n19);
        let eq110_e1479_d_n20: f64 = (p.p7 * eq110_e1478_d_n20);
        let eq110_e1479_d_n21: f64 = (p.p7 * eq110_e1478_d_n21);
        let eq110_e1479_d_n22: f64 = (p.p7 * eq110_e1478_d_n22);
        let eq110_value: f64 = eq110_e1479;
        let eq110_node_derivatives: [f64; 23] = [eq110_e1479_d_n0, eq110_e1479_d_n1, eq110_e1479_d_n2, eq110_e1479_d_n3, eq110_e1479_d_n4, eq110_e1479_d_n5, eq110_e1479_d_n6, eq110_e1479_d_n7, eq110_e1479_d_n8, eq110_e1479_d_n9, eq110_e1479_d_n10, eq110_e1479_d_n11, eq110_e1479_d_n12, eq110_e1479_d_n13, eq110_e1479_d_n14, eq110_e1479_d_n15, eq110_e1479_d_n16, eq110_e1479_d_n17, eq110_e1479_d_n18, eq110_e1479_d_n19, eq110_e1479_d_n20, eq110_e1479_d_n21, eq110_e1479_d_n22];
        let eq110_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq110_value),
            nodes,
            &eq110_node_derivatives,
            branches,
            &eq110_branch_derivatives,
            multiplicity,
        );
        let (eq111_e1486, eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22,) = {
    if s.b[569] {
        let eq111_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[162]);
        let eq111_e1483_d_n0: f64 = (s.dn[162][0] * ddt_scale);
        let eq111_e1483_d_n1: f64 = (s.dn[162][1] * ddt_scale);
        let eq111_e1483_d_n2: f64 = (s.dn[162][2] * ddt_scale);
        let eq111_e1483_d_n3: f64 = (s.dn[162][3] * ddt_scale);
        let eq111_e1483_d_n4: f64 = (s.dn[162][4] * ddt_scale);
        let eq111_e1483_d_n5: f64 = (s.dn[162][5] * ddt_scale);
        let eq111_e1483_d_n6: f64 = (s.dn[162][6] * ddt_scale);
        let eq111_e1483_d_n7: f64 = (s.dn[162][7] * ddt_scale);
        let eq111_e1483_d_n8: f64 = (s.dn[162][8] * ddt_scale);
        let eq111_e1483_d_n9: f64 = (s.dn[162][9] * ddt_scale);
        let eq111_e1483_d_n10: f64 = (s.dn[162][10] * ddt_scale);
        let eq111_e1483_d_n11: f64 = (s.dn[162][11] * ddt_scale);
        let eq111_e1483_d_n12: f64 = (s.dn[162][12] * ddt_scale);
        let eq111_e1483_d_n13: f64 = (s.dn[162][13] * ddt_scale);
        let eq111_e1483_d_n14: f64 = (s.dn[162][14] * ddt_scale);
        let eq111_e1483_d_n15: f64 = (s.dn[162][15] * ddt_scale);
        let eq111_e1483_d_n16: f64 = (s.dn[162][16] * ddt_scale);
        let eq111_e1483_d_n17: f64 = (s.dn[162][17] * ddt_scale);
        let eq111_e1483_d_n18: f64 = (s.dn[162][18] * ddt_scale);
        let eq111_e1483_d_n19: f64 = (s.dn[162][19] * ddt_scale);
        let eq111_e1483_d_n20: f64 = (s.dn[162][20] * ddt_scale);
        let eq111_e1483_d_n21: f64 = (s.dn[162][21] * ddt_scale);
        let eq111_e1483_d_n22: f64 = (s.dn[162][22] * ddt_scale);
        let eq111_e1484: f64 = (p.p7 * eq111_e1483);
        let eq111_e1484_d_n0: f64 = (p.p7 * eq111_e1483_d_n0);
        let eq111_e1484_d_n1: f64 = (p.p7 * eq111_e1483_d_n1);
        let eq111_e1484_d_n2: f64 = (p.p7 * eq111_e1483_d_n2);
        let eq111_e1484_d_n3: f64 = (p.p7 * eq111_e1483_d_n3);
        let eq111_e1484_d_n4: f64 = (p.p7 * eq111_e1483_d_n4);
        let eq111_e1484_d_n5: f64 = (p.p7 * eq111_e1483_d_n5);
        let eq111_e1484_d_n6: f64 = (p.p7 * eq111_e1483_d_n6);
        let eq111_e1484_d_n7: f64 = (p.p7 * eq111_e1483_d_n7);
        let eq111_e1484_d_n8: f64 = (p.p7 * eq111_e1483_d_n8);
        let eq111_e1484_d_n9: f64 = (p.p7 * eq111_e1483_d_n9);
        let eq111_e1484_d_n10: f64 = (p.p7 * eq111_e1483_d_n10);
        let eq111_e1484_d_n11: f64 = (p.p7 * eq111_e1483_d_n11);
        let eq111_e1484_d_n12: f64 = (p.p7 * eq111_e1483_d_n12);
        let eq111_e1484_d_n13: f64 = (p.p7 * eq111_e1483_d_n13);
        let eq111_e1484_d_n14: f64 = (p.p7 * eq111_e1483_d_n14);
        let eq111_e1484_d_n15: f64 = (p.p7 * eq111_e1483_d_n15);
        let eq111_e1484_d_n16: f64 = (p.p7 * eq111_e1483_d_n16);
        let eq111_e1484_d_n17: f64 = (p.p7 * eq111_e1483_d_n17);
        let eq111_e1484_d_n18: f64 = (p.p7 * eq111_e1483_d_n18);
        let eq111_e1484_d_n19: f64 = (p.p7 * eq111_e1483_d_n19);
        let eq111_e1484_d_n20: f64 = (p.p7 * eq111_e1483_d_n20);
        let eq111_e1484_d_n21: f64 = (p.p7 * eq111_e1483_d_n21);
        let eq111_e1484_d_n22: f64 = (p.p7 * eq111_e1483_d_n22);
        (eq111_e1484, eq111_e1484_d_n0, eq111_e1484_d_n1, eq111_e1484_d_n2, eq111_e1484_d_n3, eq111_e1484_d_n4, eq111_e1484_d_n5, eq111_e1484_d_n6, eq111_e1484_d_n7, eq111_e1484_d_n8, eq111_e1484_d_n9, eq111_e1484_d_n10, eq111_e1484_d_n11, eq111_e1484_d_n12, eq111_e1484_d_n13, eq111_e1484_d_n14, eq111_e1484_d_n15, eq111_e1484_d_n16, eq111_e1484_d_n17, eq111_e1484_d_n18, eq111_e1484_d_n19, eq111_e1484_d_n20, eq111_e1484_d_n21, eq111_e1484_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1486;
        let eq111_node_derivatives: [f64; 23] = [eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            multiplicity * (eq111_value),
            nodes,
            &eq111_node_derivatives,
            branches,
            &eq111_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22,) = {
    if s.b[569] {
        let eq112_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[163]);
        let eq112_e1490_d_n0: f64 = (s.dn[163][0] * ddt_scale);
        let eq112_e1490_d_n1: f64 = (s.dn[163][1] * ddt_scale);
        let eq112_e1490_d_n2: f64 = (s.dn[163][2] * ddt_scale);
        let eq112_e1490_d_n3: f64 = (s.dn[163][3] * ddt_scale);
        let eq112_e1490_d_n4: f64 = (s.dn[163][4] * ddt_scale);
        let eq112_e1490_d_n5: f64 = (s.dn[163][5] * ddt_scale);
        let eq112_e1490_d_n6: f64 = (s.dn[163][6] * ddt_scale);
        let eq112_e1490_d_n7: f64 = (s.dn[163][7] * ddt_scale);
        let eq112_e1490_d_n8: f64 = (s.dn[163][8] * ddt_scale);
        let eq112_e1490_d_n9: f64 = (s.dn[163][9] * ddt_scale);
        let eq112_e1490_d_n10: f64 = (s.dn[163][10] * ddt_scale);
        let eq112_e1490_d_n11: f64 = (s.dn[163][11] * ddt_scale);
        let eq112_e1490_d_n12: f64 = (s.dn[163][12] * ddt_scale);
        let eq112_e1490_d_n13: f64 = (s.dn[163][13] * ddt_scale);
        let eq112_e1490_d_n14: f64 = (s.dn[163][14] * ddt_scale);
        let eq112_e1490_d_n15: f64 = (s.dn[163][15] * ddt_scale);
        let eq112_e1490_d_n16: f64 = (s.dn[163][16] * ddt_scale);
        let eq112_e1490_d_n17: f64 = (s.dn[163][17] * ddt_scale);
        let eq112_e1490_d_n18: f64 = (s.dn[163][18] * ddt_scale);
        let eq112_e1490_d_n19: f64 = (s.dn[163][19] * ddt_scale);
        let eq112_e1490_d_n20: f64 = (s.dn[163][20] * ddt_scale);
        let eq112_e1490_d_n21: f64 = (s.dn[163][21] * ddt_scale);
        let eq112_e1490_d_n22: f64 = (s.dn[163][22] * ddt_scale);
        let eq112_e1491: f64 = (p.p7 * eq112_e1490);
        let eq112_e1491_d_n0: f64 = (p.p7 * eq112_e1490_d_n0);
        let eq112_e1491_d_n1: f64 = (p.p7 * eq112_e1490_d_n1);
        let eq112_e1491_d_n2: f64 = (p.p7 * eq112_e1490_d_n2);
        let eq112_e1491_d_n3: f64 = (p.p7 * eq112_e1490_d_n3);
        let eq112_e1491_d_n4: f64 = (p.p7 * eq112_e1490_d_n4);
        let eq112_e1491_d_n5: f64 = (p.p7 * eq112_e1490_d_n5);
        let eq112_e1491_d_n6: f64 = (p.p7 * eq112_e1490_d_n6);
        let eq112_e1491_d_n7: f64 = (p.p7 * eq112_e1490_d_n7);
        let eq112_e1491_d_n8: f64 = (p.p7 * eq112_e1490_d_n8);
        let eq112_e1491_d_n9: f64 = (p.p7 * eq112_e1490_d_n9);
        let eq112_e1491_d_n10: f64 = (p.p7 * eq112_e1490_d_n10);
        let eq112_e1491_d_n11: f64 = (p.p7 * eq112_e1490_d_n11);
        let eq112_e1491_d_n12: f64 = (p.p7 * eq112_e1490_d_n12);
        let eq112_e1491_d_n13: f64 = (p.p7 * eq112_e1490_d_n13);
        let eq112_e1491_d_n14: f64 = (p.p7 * eq112_e1490_d_n14);
        let eq112_e1491_d_n15: f64 = (p.p7 * eq112_e1490_d_n15);
        let eq112_e1491_d_n16: f64 = (p.p7 * eq112_e1490_d_n16);
        let eq112_e1491_d_n17: f64 = (p.p7 * eq112_e1490_d_n17);
        let eq112_e1491_d_n18: f64 = (p.p7 * eq112_e1490_d_n18);
        let eq112_e1491_d_n19: f64 = (p.p7 * eq112_e1490_d_n19);
        let eq112_e1491_d_n20: f64 = (p.p7 * eq112_e1490_d_n20);
        let eq112_e1491_d_n21: f64 = (p.p7 * eq112_e1490_d_n21);
        let eq112_e1491_d_n22: f64 = (p.p7 * eq112_e1490_d_n22);
        (eq112_e1491, eq112_e1491_d_n0, eq112_e1491_d_n1, eq112_e1491_d_n2, eq112_e1491_d_n3, eq112_e1491_d_n4, eq112_e1491_d_n5, eq112_e1491_d_n6, eq112_e1491_d_n7, eq112_e1491_d_n8, eq112_e1491_d_n9, eq112_e1491_d_n10, eq112_e1491_d_n11, eq112_e1491_d_n12, eq112_e1491_d_n13, eq112_e1491_d_n14, eq112_e1491_d_n15, eq112_e1491_d_n16, eq112_e1491_d_n17, eq112_e1491_d_n18, eq112_e1491_d_n19, eq112_e1491_d_n20, eq112_e1491_d_n21, eq112_e1491_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1493;
        let eq112_node_derivatives: [f64; 23] = [eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22];
        let eq112_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            multiplicity * (eq112_value),
            nodes,
            &eq112_node_derivatives,
            branches,
            &eq112_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1501, eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22,) = {
    if (!s.b[569]) {
        let eq113_e1498: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[162]);
        let eq113_e1498_d_n0: f64 = (s.dn[162][0] * ddt_scale);
        let eq113_e1498_d_n1: f64 = (s.dn[162][1] * ddt_scale);
        let eq113_e1498_d_n2: f64 = (s.dn[162][2] * ddt_scale);
        let eq113_e1498_d_n3: f64 = (s.dn[162][3] * ddt_scale);
        let eq113_e1498_d_n4: f64 = (s.dn[162][4] * ddt_scale);
        let eq113_e1498_d_n5: f64 = (s.dn[162][5] * ddt_scale);
        let eq113_e1498_d_n6: f64 = (s.dn[162][6] * ddt_scale);
        let eq113_e1498_d_n7: f64 = (s.dn[162][7] * ddt_scale);
        let eq113_e1498_d_n8: f64 = (s.dn[162][8] * ddt_scale);
        let eq113_e1498_d_n9: f64 = (s.dn[162][9] * ddt_scale);
        let eq113_e1498_d_n10: f64 = (s.dn[162][10] * ddt_scale);
        let eq113_e1498_d_n11: f64 = (s.dn[162][11] * ddt_scale);
        let eq113_e1498_d_n12: f64 = (s.dn[162][12] * ddt_scale);
        let eq113_e1498_d_n13: f64 = (s.dn[162][13] * ddt_scale);
        let eq113_e1498_d_n14: f64 = (s.dn[162][14] * ddt_scale);
        let eq113_e1498_d_n15: f64 = (s.dn[162][15] * ddt_scale);
        let eq113_e1498_d_n16: f64 = (s.dn[162][16] * ddt_scale);
        let eq113_e1498_d_n17: f64 = (s.dn[162][17] * ddt_scale);
        let eq113_e1498_d_n18: f64 = (s.dn[162][18] * ddt_scale);
        let eq113_e1498_d_n19: f64 = (s.dn[162][19] * ddt_scale);
        let eq113_e1498_d_n20: f64 = (s.dn[162][20] * ddt_scale);
        let eq113_e1498_d_n21: f64 = (s.dn[162][21] * ddt_scale);
        let eq113_e1498_d_n22: f64 = (s.dn[162][22] * ddt_scale);
        let eq113_e1499: f64 = (p.p7 * eq113_e1498);
        let eq113_e1499_d_n0: f64 = (p.p7 * eq113_e1498_d_n0);
        let eq113_e1499_d_n1: f64 = (p.p7 * eq113_e1498_d_n1);
        let eq113_e1499_d_n2: f64 = (p.p7 * eq113_e1498_d_n2);
        let eq113_e1499_d_n3: f64 = (p.p7 * eq113_e1498_d_n3);
        let eq113_e1499_d_n4: f64 = (p.p7 * eq113_e1498_d_n4);
        let eq113_e1499_d_n5: f64 = (p.p7 * eq113_e1498_d_n5);
        let eq113_e1499_d_n6: f64 = (p.p7 * eq113_e1498_d_n6);
        let eq113_e1499_d_n7: f64 = (p.p7 * eq113_e1498_d_n7);
        let eq113_e1499_d_n8: f64 = (p.p7 * eq113_e1498_d_n8);
        let eq113_e1499_d_n9: f64 = (p.p7 * eq113_e1498_d_n9);
        let eq113_e1499_d_n10: f64 = (p.p7 * eq113_e1498_d_n10);
        let eq113_e1499_d_n11: f64 = (p.p7 * eq113_e1498_d_n11);
        let eq113_e1499_d_n12: f64 = (p.p7 * eq113_e1498_d_n12);
        let eq113_e1499_d_n13: f64 = (p.p7 * eq113_e1498_d_n13);
        let eq113_e1499_d_n14: f64 = (p.p7 * eq113_e1498_d_n14);
        let eq113_e1499_d_n15: f64 = (p.p7 * eq113_e1498_d_n15);
        let eq113_e1499_d_n16: f64 = (p.p7 * eq113_e1498_d_n16);
        let eq113_e1499_d_n17: f64 = (p.p7 * eq113_e1498_d_n17);
        let eq113_e1499_d_n18: f64 = (p.p7 * eq113_e1498_d_n18);
        let eq113_e1499_d_n19: f64 = (p.p7 * eq113_e1498_d_n19);
        let eq113_e1499_d_n20: f64 = (p.p7 * eq113_e1498_d_n20);
        let eq113_e1499_d_n21: f64 = (p.p7 * eq113_e1498_d_n21);
        let eq113_e1499_d_n22: f64 = (p.p7 * eq113_e1498_d_n22);
        (eq113_e1499, eq113_e1499_d_n0, eq113_e1499_d_n1, eq113_e1499_d_n2, eq113_e1499_d_n3, eq113_e1499_d_n4, eq113_e1499_d_n5, eq113_e1499_d_n6, eq113_e1499_d_n7, eq113_e1499_d_n8, eq113_e1499_d_n9, eq113_e1499_d_n10, eq113_e1499_d_n11, eq113_e1499_d_n12, eq113_e1499_d_n13, eq113_e1499_d_n14, eq113_e1499_d_n15, eq113_e1499_d_n16, eq113_e1499_d_n17, eq113_e1499_d_n18, eq113_e1499_d_n19, eq113_e1499_d_n20, eq113_e1499_d_n21, eq113_e1499_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1501;
        let eq113_node_derivatives: [f64; 23] = [eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22];
        let eq113_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            multiplicity * (eq113_value),
            nodes,
            &eq113_node_derivatives,
            branches,
            &eq113_branch_derivatives,
            multiplicity,
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22,) = {
    if (!s.b[569]) {
        let eq114_e1506: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[163]);
        let eq114_e1506_d_n0: f64 = (s.dn[163][0] * ddt_scale);
        let eq114_e1506_d_n1: f64 = (s.dn[163][1] * ddt_scale);
        let eq114_e1506_d_n2: f64 = (s.dn[163][2] * ddt_scale);
        let eq114_e1506_d_n3: f64 = (s.dn[163][3] * ddt_scale);
        let eq114_e1506_d_n4: f64 = (s.dn[163][4] * ddt_scale);
        let eq114_e1506_d_n5: f64 = (s.dn[163][5] * ddt_scale);
        let eq114_e1506_d_n6: f64 = (s.dn[163][6] * ddt_scale);
        let eq114_e1506_d_n7: f64 = (s.dn[163][7] * ddt_scale);
        let eq114_e1506_d_n8: f64 = (s.dn[163][8] * ddt_scale);
        let eq114_e1506_d_n9: f64 = (s.dn[163][9] * ddt_scale);
        let eq114_e1506_d_n10: f64 = (s.dn[163][10] * ddt_scale);
        let eq114_e1506_d_n11: f64 = (s.dn[163][11] * ddt_scale);
        let eq114_e1506_d_n12: f64 = (s.dn[163][12] * ddt_scale);
        let eq114_e1506_d_n13: f64 = (s.dn[163][13] * ddt_scale);
        let eq114_e1506_d_n14: f64 = (s.dn[163][14] * ddt_scale);
        let eq114_e1506_d_n15: f64 = (s.dn[163][15] * ddt_scale);
        let eq114_e1506_d_n16: f64 = (s.dn[163][16] * ddt_scale);
        let eq114_e1506_d_n17: f64 = (s.dn[163][17] * ddt_scale);
        let eq114_e1506_d_n18: f64 = (s.dn[163][18] * ddt_scale);
        let eq114_e1506_d_n19: f64 = (s.dn[163][19] * ddt_scale);
        let eq114_e1506_d_n20: f64 = (s.dn[163][20] * ddt_scale);
        let eq114_e1506_d_n21: f64 = (s.dn[163][21] * ddt_scale);
        let eq114_e1506_d_n22: f64 = (s.dn[163][22] * ddt_scale);
        let eq114_e1507: f64 = (p.p7 * eq114_e1506);
        let eq114_e1507_d_n0: f64 = (p.p7 * eq114_e1506_d_n0);
        let eq114_e1507_d_n1: f64 = (p.p7 * eq114_e1506_d_n1);
        let eq114_e1507_d_n2: f64 = (p.p7 * eq114_e1506_d_n2);
        let eq114_e1507_d_n3: f64 = (p.p7 * eq114_e1506_d_n3);
        let eq114_e1507_d_n4: f64 = (p.p7 * eq114_e1506_d_n4);
        let eq114_e1507_d_n5: f64 = (p.p7 * eq114_e1506_d_n5);
        let eq114_e1507_d_n6: f64 = (p.p7 * eq114_e1506_d_n6);
        let eq114_e1507_d_n7: f64 = (p.p7 * eq114_e1506_d_n7);
        let eq114_e1507_d_n8: f64 = (p.p7 * eq114_e1506_d_n8);
        let eq114_e1507_d_n9: f64 = (p.p7 * eq114_e1506_d_n9);
        let eq114_e1507_d_n10: f64 = (p.p7 * eq114_e1506_d_n10);
        let eq114_e1507_d_n11: f64 = (p.p7 * eq114_e1506_d_n11);
        let eq114_e1507_d_n12: f64 = (p.p7 * eq114_e1506_d_n12);
        let eq114_e1507_d_n13: f64 = (p.p7 * eq114_e1506_d_n13);
        let eq114_e1507_d_n14: f64 = (p.p7 * eq114_e1506_d_n14);
        let eq114_e1507_d_n15: f64 = (p.p7 * eq114_e1506_d_n15);
        let eq114_e1507_d_n16: f64 = (p.p7 * eq114_e1506_d_n16);
        let eq114_e1507_d_n17: f64 = (p.p7 * eq114_e1506_d_n17);
        let eq114_e1507_d_n18: f64 = (p.p7 * eq114_e1506_d_n18);
        let eq114_e1507_d_n19: f64 = (p.p7 * eq114_e1506_d_n19);
        let eq114_e1507_d_n20: f64 = (p.p7 * eq114_e1506_d_n20);
        let eq114_e1507_d_n21: f64 = (p.p7 * eq114_e1506_d_n21);
        let eq114_e1507_d_n22: f64 = (p.p7 * eq114_e1506_d_n22);
        (eq114_e1507, eq114_e1507_d_n0, eq114_e1507_d_n1, eq114_e1507_d_n2, eq114_e1507_d_n3, eq114_e1507_d_n4, eq114_e1507_d_n5, eq114_e1507_d_n6, eq114_e1507_d_n7, eq114_e1507_d_n8, eq114_e1507_d_n9, eq114_e1507_d_n10, eq114_e1507_d_n11, eq114_e1507_d_n12, eq114_e1507_d_n13, eq114_e1507_d_n14, eq114_e1507_d_n15, eq114_e1507_d_n16, eq114_e1507_d_n17, eq114_e1507_d_n18, eq114_e1507_d_n19, eq114_e1507_d_n20, eq114_e1507_d_n21, eq114_e1507_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_value: f64 = eq114_e1509;
        let eq114_node_derivatives: [f64; 23] = [eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22];
        let eq114_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            multiplicity * (eq114_value),
            nodes,
            &eq114_node_derivatives,
            branches,
            &eq114_branch_derivatives,
            multiplicity,
        );
        let eq115_e1512: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[164]);
        let eq115_e1512_d_n0: f64 = (s.dn[164][0] * ddt_scale);
        let eq115_e1512_d_n1: f64 = (s.dn[164][1] * ddt_scale);
        let eq115_e1512_d_n2: f64 = (s.dn[164][2] * ddt_scale);
        let eq115_e1512_d_n3: f64 = (s.dn[164][3] * ddt_scale);
        let eq115_e1512_d_n4: f64 = (s.dn[164][4] * ddt_scale);
        let eq115_e1512_d_n5: f64 = (s.dn[164][5] * ddt_scale);
        let eq115_e1512_d_n6: f64 = (s.dn[164][6] * ddt_scale);
        let eq115_e1512_d_n7: f64 = (s.dn[164][7] * ddt_scale);
        let eq115_e1512_d_n8: f64 = (s.dn[164][8] * ddt_scale);
        let eq115_e1512_d_n9: f64 = (s.dn[164][9] * ddt_scale);
        let eq115_e1512_d_n10: f64 = (s.dn[164][10] * ddt_scale);
        let eq115_e1512_d_n11: f64 = (s.dn[164][11] * ddt_scale);
        let eq115_e1512_d_n12: f64 = (s.dn[164][12] * ddt_scale);
        let eq115_e1512_d_n13: f64 = (s.dn[164][13] * ddt_scale);
        let eq115_e1512_d_n14: f64 = (s.dn[164][14] * ddt_scale);
        let eq115_e1512_d_n15: f64 = (s.dn[164][15] * ddt_scale);
        let eq115_e1512_d_n16: f64 = (s.dn[164][16] * ddt_scale);
        let eq115_e1512_d_n17: f64 = (s.dn[164][17] * ddt_scale);
        let eq115_e1512_d_n18: f64 = (s.dn[164][18] * ddt_scale);
        let eq115_e1512_d_n19: f64 = (s.dn[164][19] * ddt_scale);
        let eq115_e1512_d_n20: f64 = (s.dn[164][20] * ddt_scale);
        let eq115_e1512_d_n21: f64 = (s.dn[164][21] * ddt_scale);
        let eq115_e1512_d_n22: f64 = (s.dn[164][22] * ddt_scale);
        let eq115_e1513: f64 = (p.p7 * eq115_e1512);
        let eq115_e1513_d_n0: f64 = (p.p7 * eq115_e1512_d_n0);
        let eq115_e1513_d_n1: f64 = (p.p7 * eq115_e1512_d_n1);
        let eq115_e1513_d_n2: f64 = (p.p7 * eq115_e1512_d_n2);
        let eq115_e1513_d_n3: f64 = (p.p7 * eq115_e1512_d_n3);
        let eq115_e1513_d_n4: f64 = (p.p7 * eq115_e1512_d_n4);
        let eq115_e1513_d_n5: f64 = (p.p7 * eq115_e1512_d_n5);
        let eq115_e1513_d_n6: f64 = (p.p7 * eq115_e1512_d_n6);
        let eq115_e1513_d_n7: f64 = (p.p7 * eq115_e1512_d_n7);
        let eq115_e1513_d_n8: f64 = (p.p7 * eq115_e1512_d_n8);
        let eq115_e1513_d_n9: f64 = (p.p7 * eq115_e1512_d_n9);
        let eq115_e1513_d_n10: f64 = (p.p7 * eq115_e1512_d_n10);
        let eq115_e1513_d_n11: f64 = (p.p7 * eq115_e1512_d_n11);
        let eq115_e1513_d_n12: f64 = (p.p7 * eq115_e1512_d_n12);
        let eq115_e1513_d_n13: f64 = (p.p7 * eq115_e1512_d_n13);
        let eq115_e1513_d_n14: f64 = (p.p7 * eq115_e1512_d_n14);
        let eq115_e1513_d_n15: f64 = (p.p7 * eq115_e1512_d_n15);
        let eq115_e1513_d_n16: f64 = (p.p7 * eq115_e1512_d_n16);
        let eq115_e1513_d_n17: f64 = (p.p7 * eq115_e1512_d_n17);
        let eq115_e1513_d_n18: f64 = (p.p7 * eq115_e1512_d_n18);
        let eq115_e1513_d_n19: f64 = (p.p7 * eq115_e1512_d_n19);
        let eq115_e1513_d_n20: f64 = (p.p7 * eq115_e1512_d_n20);
        let eq115_e1513_d_n21: f64 = (p.p7 * eq115_e1512_d_n21);
        let eq115_e1513_d_n22: f64 = (p.p7 * eq115_e1512_d_n22);
        let eq115_value: f64 = eq115_e1513;
        let eq115_node_derivatives: [f64; 23] = [eq115_e1513_d_n0, eq115_e1513_d_n1, eq115_e1513_d_n2, eq115_e1513_d_n3, eq115_e1513_d_n4, eq115_e1513_d_n5, eq115_e1513_d_n6, eq115_e1513_d_n7, eq115_e1513_d_n8, eq115_e1513_d_n9, eq115_e1513_d_n10, eq115_e1513_d_n11, eq115_e1513_d_n12, eq115_e1513_d_n13, eq115_e1513_d_n14, eq115_e1513_d_n15, eq115_e1513_d_n16, eq115_e1513_d_n17, eq115_e1513_d_n18, eq115_e1513_d_n19, eq115_e1513_d_n20, eq115_e1513_d_n21, eq115_e1513_d_n22];
        let eq115_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            multiplicity * (eq115_value),
            nodes,
            &eq115_node_derivatives,
            branches,
            &eq115_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
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
        let eq116_e1516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, s.v[219]);
        let eq116_e1516_d_n0: f64 = (s.dn[219][0] * ddt_scale);
        let eq116_e1516_d_n1: f64 = (s.dn[219][1] * ddt_scale);
        let eq116_e1516_d_n2: f64 = (s.dn[219][2] * ddt_scale);
        let eq116_e1516_d_n3: f64 = (s.dn[219][3] * ddt_scale);
        let eq116_e1516_d_n4: f64 = (s.dn[219][4] * ddt_scale);
        let eq116_e1516_d_n5: f64 = (s.dn[219][5] * ddt_scale);
        let eq116_e1516_d_n6: f64 = (s.dn[219][6] * ddt_scale);
        let eq116_e1516_d_n7: f64 = (s.dn[219][7] * ddt_scale);
        let eq116_e1516_d_n8: f64 = (s.dn[219][8] * ddt_scale);
        let eq116_e1516_d_n9: f64 = (s.dn[219][9] * ddt_scale);
        let eq116_e1516_d_n10: f64 = (s.dn[219][10] * ddt_scale);
        let eq116_e1516_d_n11: f64 = (s.dn[219][11] * ddt_scale);
        let eq116_e1516_d_n12: f64 = (s.dn[219][12] * ddt_scale);
        let eq116_e1516_d_n13: f64 = (s.dn[219][13] * ddt_scale);
        let eq116_e1516_d_n14: f64 = (s.dn[219][14] * ddt_scale);
        let eq116_e1516_d_n15: f64 = (s.dn[219][15] * ddt_scale);
        let eq116_e1516_d_n16: f64 = (s.dn[219][16] * ddt_scale);
        let eq116_e1516_d_n17: f64 = (s.dn[219][17] * ddt_scale);
        let eq116_e1516_d_n18: f64 = (s.dn[219][18] * ddt_scale);
        let eq116_e1516_d_n19: f64 = (s.dn[219][19] * ddt_scale);
        let eq116_e1516_d_n20: f64 = (s.dn[219][20] * ddt_scale);
        let eq116_e1516_d_n21: f64 = (s.dn[219][21] * ddt_scale);
        let eq116_e1516_d_n22: f64 = (s.dn[219][22] * ddt_scale);
        let eq116_e1517: f64 = (p.p7 * eq116_e1516);
        let eq116_e1517_d_n0: f64 = (p.p7 * eq116_e1516_d_n0);
        let eq116_e1517_d_n1: f64 = (p.p7 * eq116_e1516_d_n1);
        let eq116_e1517_d_n2: f64 = (p.p7 * eq116_e1516_d_n2);
        let eq116_e1517_d_n3: f64 = (p.p7 * eq116_e1516_d_n3);
        let eq116_e1517_d_n4: f64 = (p.p7 * eq116_e1516_d_n4);
        let eq116_e1517_d_n5: f64 = (p.p7 * eq116_e1516_d_n5);
        let eq116_e1517_d_n6: f64 = (p.p7 * eq116_e1516_d_n6);
        let eq116_e1517_d_n7: f64 = (p.p7 * eq116_e1516_d_n7);
        let eq116_e1517_d_n8: f64 = (p.p7 * eq116_e1516_d_n8);
        let eq116_e1517_d_n9: f64 = (p.p7 * eq116_e1516_d_n9);
        let eq116_e1517_d_n10: f64 = (p.p7 * eq116_e1516_d_n10);
        let eq116_e1517_d_n11: f64 = (p.p7 * eq116_e1516_d_n11);
        let eq116_e1517_d_n12: f64 = (p.p7 * eq116_e1516_d_n12);
        let eq116_e1517_d_n13: f64 = (p.p7 * eq116_e1516_d_n13);
        let eq116_e1517_d_n14: f64 = (p.p7 * eq116_e1516_d_n14);
        let eq116_e1517_d_n15: f64 = (p.p7 * eq116_e1516_d_n15);
        let eq116_e1517_d_n16: f64 = (p.p7 * eq116_e1516_d_n16);
        let eq116_e1517_d_n17: f64 = (p.p7 * eq116_e1516_d_n17);
        let eq116_e1517_d_n18: f64 = (p.p7 * eq116_e1516_d_n18);
        let eq116_e1517_d_n19: f64 = (p.p7 * eq116_e1516_d_n19);
        let eq116_e1517_d_n20: f64 = (p.p7 * eq116_e1516_d_n20);
        let eq116_e1517_d_n21: f64 = (p.p7 * eq116_e1516_d_n21);
        let eq116_e1517_d_n22: f64 = (p.p7 * eq116_e1516_d_n22);
        let eq116_value: f64 = eq116_e1517;
        let eq116_node_derivatives: [f64; 23] = [eq116_e1517_d_n0, eq116_e1517_d_n1, eq116_e1517_d_n2, eq116_e1517_d_n3, eq116_e1517_d_n4, eq116_e1517_d_n5, eq116_e1517_d_n6, eq116_e1517_d_n7, eq116_e1517_d_n8, eq116_e1517_d_n9, eq116_e1517_d_n10, eq116_e1517_d_n11, eq116_e1517_d_n12, eq116_e1517_d_n13, eq116_e1517_d_n14, eq116_e1517_d_n15, eq116_e1517_d_n16, eq116_e1517_d_n17, eq116_e1517_d_n18, eq116_e1517_d_n19, eq116_e1517_d_n20, eq116_e1517_d_n21, eq116_e1517_d_n22];
        let eq116_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            multiplicity * (eq116_value),
            nodes,
            &eq116_node_derivatives,
            branches,
            &eq116_branch_derivatives,
            multiplicity,
        );
        let eq117_e1520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[220]);
        let eq117_e1520_d_n0: f64 = (s.dn[220][0] * ddt_scale);
        let eq117_e1520_d_n1: f64 = (s.dn[220][1] * ddt_scale);
        let eq117_e1520_d_n2: f64 = (s.dn[220][2] * ddt_scale);
        let eq117_e1520_d_n3: f64 = (s.dn[220][3] * ddt_scale);
        let eq117_e1520_d_n4: f64 = (s.dn[220][4] * ddt_scale);
        let eq117_e1520_d_n5: f64 = (s.dn[220][5] * ddt_scale);
        let eq117_e1520_d_n6: f64 = (s.dn[220][6] * ddt_scale);
        let eq117_e1520_d_n7: f64 = (s.dn[220][7] * ddt_scale);
        let eq117_e1520_d_n8: f64 = (s.dn[220][8] * ddt_scale);
        let eq117_e1520_d_n9: f64 = (s.dn[220][9] * ddt_scale);
        let eq117_e1520_d_n10: f64 = (s.dn[220][10] * ddt_scale);
        let eq117_e1520_d_n11: f64 = (s.dn[220][11] * ddt_scale);
        let eq117_e1520_d_n12: f64 = (s.dn[220][12] * ddt_scale);
        let eq117_e1520_d_n13: f64 = (s.dn[220][13] * ddt_scale);
        let eq117_e1520_d_n14: f64 = (s.dn[220][14] * ddt_scale);
        let eq117_e1520_d_n15: f64 = (s.dn[220][15] * ddt_scale);
        let eq117_e1520_d_n16: f64 = (s.dn[220][16] * ddt_scale);
        let eq117_e1520_d_n17: f64 = (s.dn[220][17] * ddt_scale);
        let eq117_e1520_d_n18: f64 = (s.dn[220][18] * ddt_scale);
        let eq117_e1520_d_n19: f64 = (s.dn[220][19] * ddt_scale);
        let eq117_e1520_d_n20: f64 = (s.dn[220][20] * ddt_scale);
        let eq117_e1520_d_n21: f64 = (s.dn[220][21] * ddt_scale);
        let eq117_e1520_d_n22: f64 = (s.dn[220][22] * ddt_scale);
        let eq117_e1521: f64 = (p.p7 * eq117_e1520);
        let eq117_e1521_d_n0: f64 = (p.p7 * eq117_e1520_d_n0);
        let eq117_e1521_d_n1: f64 = (p.p7 * eq117_e1520_d_n1);
        let eq117_e1521_d_n2: f64 = (p.p7 * eq117_e1520_d_n2);
        let eq117_e1521_d_n3: f64 = (p.p7 * eq117_e1520_d_n3);
        let eq117_e1521_d_n4: f64 = (p.p7 * eq117_e1520_d_n4);
        let eq117_e1521_d_n5: f64 = (p.p7 * eq117_e1520_d_n5);
        let eq117_e1521_d_n6: f64 = (p.p7 * eq117_e1520_d_n6);
        let eq117_e1521_d_n7: f64 = (p.p7 * eq117_e1520_d_n7);
        let eq117_e1521_d_n8: f64 = (p.p7 * eq117_e1520_d_n8);
        let eq117_e1521_d_n9: f64 = (p.p7 * eq117_e1520_d_n9);
        let eq117_e1521_d_n10: f64 = (p.p7 * eq117_e1520_d_n10);
        let eq117_e1521_d_n11: f64 = (p.p7 * eq117_e1520_d_n11);
        let eq117_e1521_d_n12: f64 = (p.p7 * eq117_e1520_d_n12);
        let eq117_e1521_d_n13: f64 = (p.p7 * eq117_e1520_d_n13);
        let eq117_e1521_d_n14: f64 = (p.p7 * eq117_e1520_d_n14);
        let eq117_e1521_d_n15: f64 = (p.p7 * eq117_e1520_d_n15);
        let eq117_e1521_d_n16: f64 = (p.p7 * eq117_e1520_d_n16);
        let eq117_e1521_d_n17: f64 = (p.p7 * eq117_e1520_d_n17);
        let eq117_e1521_d_n18: f64 = (p.p7 * eq117_e1520_d_n18);
        let eq117_e1521_d_n19: f64 = (p.p7 * eq117_e1520_d_n19);
        let eq117_e1521_d_n20: f64 = (p.p7 * eq117_e1520_d_n20);
        let eq117_e1521_d_n21: f64 = (p.p7 * eq117_e1520_d_n21);
        let eq117_e1521_d_n22: f64 = (p.p7 * eq117_e1520_d_n22);
        let eq117_value: f64 = eq117_e1521;
        let eq117_node_derivatives: [f64; 23] = [eq117_e1521_d_n0, eq117_e1521_d_n1, eq117_e1521_d_n2, eq117_e1521_d_n3, eq117_e1521_d_n4, eq117_e1521_d_n5, eq117_e1521_d_n6, eq117_e1521_d_n7, eq117_e1521_d_n8, eq117_e1521_d_n9, eq117_e1521_d_n10, eq117_e1521_d_n11, eq117_e1521_d_n12, eq117_e1521_d_n13, eq117_e1521_d_n14, eq117_e1521_d_n15, eq117_e1521_d_n16, eq117_e1521_d_n17, eq117_e1521_d_n18, eq117_e1521_d_n19, eq117_e1521_d_n20, eq117_e1521_d_n21, eq117_e1521_d_n22];
        let eq117_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            multiplicity * (eq117_value),
            nodes,
            &eq117_node_derivatives,
            branches,
            &eq117_branch_derivatives,
            multiplicity,
        );
        let eq118_e1524: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, s.v[221]);
        let eq118_e1524_d_n0: f64 = (s.dn[221][0] * ddt_scale);
        let eq118_e1524_d_n1: f64 = (s.dn[221][1] * ddt_scale);
        let eq118_e1524_d_n2: f64 = (s.dn[221][2] * ddt_scale);
        let eq118_e1524_d_n3: f64 = (s.dn[221][3] * ddt_scale);
        let eq118_e1524_d_n4: f64 = (s.dn[221][4] * ddt_scale);
        let eq118_e1524_d_n5: f64 = (s.dn[221][5] * ddt_scale);
        let eq118_e1524_d_n6: f64 = (s.dn[221][6] * ddt_scale);
        let eq118_e1524_d_n7: f64 = (s.dn[221][7] * ddt_scale);
        let eq118_e1524_d_n8: f64 = (s.dn[221][8] * ddt_scale);
        let eq118_e1524_d_n9: f64 = (s.dn[221][9] * ddt_scale);
        let eq118_e1524_d_n10: f64 = (s.dn[221][10] * ddt_scale);
        let eq118_e1524_d_n11: f64 = (s.dn[221][11] * ddt_scale);
        let eq118_e1524_d_n12: f64 = (s.dn[221][12] * ddt_scale);
        let eq118_e1524_d_n13: f64 = (s.dn[221][13] * ddt_scale);
        let eq118_e1524_d_n14: f64 = (s.dn[221][14] * ddt_scale);
        let eq118_e1524_d_n15: f64 = (s.dn[221][15] * ddt_scale);
        let eq118_e1524_d_n16: f64 = (s.dn[221][16] * ddt_scale);
        let eq118_e1524_d_n17: f64 = (s.dn[221][17] * ddt_scale);
        let eq118_e1524_d_n18: f64 = (s.dn[221][18] * ddt_scale);
        let eq118_e1524_d_n19: f64 = (s.dn[221][19] * ddt_scale);
        let eq118_e1524_d_n20: f64 = (s.dn[221][20] * ddt_scale);
        let eq118_e1524_d_n21: f64 = (s.dn[221][21] * ddt_scale);
        let eq118_e1524_d_n22: f64 = (s.dn[221][22] * ddt_scale);
        let eq118_e1525: f64 = (p.p7 * eq118_e1524);
        let eq118_e1525_d_n0: f64 = (p.p7 * eq118_e1524_d_n0);
        let eq118_e1525_d_n1: f64 = (p.p7 * eq118_e1524_d_n1);
        let eq118_e1525_d_n2: f64 = (p.p7 * eq118_e1524_d_n2);
        let eq118_e1525_d_n3: f64 = (p.p7 * eq118_e1524_d_n3);
        let eq118_e1525_d_n4: f64 = (p.p7 * eq118_e1524_d_n4);
        let eq118_e1525_d_n5: f64 = (p.p7 * eq118_e1524_d_n5);
        let eq118_e1525_d_n6: f64 = (p.p7 * eq118_e1524_d_n6);
        let eq118_e1525_d_n7: f64 = (p.p7 * eq118_e1524_d_n7);
        let eq118_e1525_d_n8: f64 = (p.p7 * eq118_e1524_d_n8);
        let eq118_e1525_d_n9: f64 = (p.p7 * eq118_e1524_d_n9);
        let eq118_e1525_d_n10: f64 = (p.p7 * eq118_e1524_d_n10);
        let eq118_e1525_d_n11: f64 = (p.p7 * eq118_e1524_d_n11);
        let eq118_e1525_d_n12: f64 = (p.p7 * eq118_e1524_d_n12);
        let eq118_e1525_d_n13: f64 = (p.p7 * eq118_e1524_d_n13);
        let eq118_e1525_d_n14: f64 = (p.p7 * eq118_e1524_d_n14);
        let eq118_e1525_d_n15: f64 = (p.p7 * eq118_e1524_d_n15);
        let eq118_e1525_d_n16: f64 = (p.p7 * eq118_e1524_d_n16);
        let eq118_e1525_d_n17: f64 = (p.p7 * eq118_e1524_d_n17);
        let eq118_e1525_d_n18: f64 = (p.p7 * eq118_e1524_d_n18);
        let eq118_e1525_d_n19: f64 = (p.p7 * eq118_e1524_d_n19);
        let eq118_e1525_d_n20: f64 = (p.p7 * eq118_e1524_d_n20);
        let eq118_e1525_d_n21: f64 = (p.p7 * eq118_e1524_d_n21);
        let eq118_e1525_d_n22: f64 = (p.p7 * eq118_e1524_d_n22);
        let eq118_value: f64 = eq118_e1525;
        let eq118_node_derivatives: [f64; 23] = [eq118_e1525_d_n0, eq118_e1525_d_n1, eq118_e1525_d_n2, eq118_e1525_d_n3, eq118_e1525_d_n4, eq118_e1525_d_n5, eq118_e1525_d_n6, eq118_e1525_d_n7, eq118_e1525_d_n8, eq118_e1525_d_n9, eq118_e1525_d_n10, eq118_e1525_d_n11, eq118_e1525_d_n12, eq118_e1525_d_n13, eq118_e1525_d_n14, eq118_e1525_d_n15, eq118_e1525_d_n16, eq118_e1525_d_n17, eq118_e1525_d_n18, eq118_e1525_d_n19, eq118_e1525_d_n20, eq118_e1525_d_n21, eq118_e1525_d_n22];
        let eq118_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            multiplicity * (eq118_value),
            nodes,
            &eq118_node_derivatives,
            branches,
            &eq118_branch_derivatives,
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
        let eq119_e1530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq119_e1529);
        let eq119_e1530_d_n0: f64 = (eq119_e1529_d_n0 * ddt_scale);
        let eq119_e1530_d_n1: f64 = (eq119_e1529_d_n1 * ddt_scale);
        let eq119_e1530_d_n2: f64 = (eq119_e1529_d_n2 * ddt_scale);
        let eq119_e1530_d_n3: f64 = (eq119_e1529_d_n3 * ddt_scale);
        let eq119_e1530_d_n4: f64 = (eq119_e1529_d_n4 * ddt_scale);
        let eq119_e1530_d_n5: f64 = (eq119_e1529_d_n5 * ddt_scale);
        let eq119_e1530_d_n6: f64 = (eq119_e1529_d_n6 * ddt_scale);
        let eq119_e1530_d_n7: f64 = (eq119_e1529_d_n7 * ddt_scale);
        let eq119_e1530_d_n8: f64 = (eq119_e1529_d_n8 * ddt_scale);
        let eq119_e1530_d_n9: f64 = (eq119_e1529_d_n9 * ddt_scale);
        let eq119_e1530_d_n10: f64 = (eq119_e1529_d_n10 * ddt_scale);
        let eq119_e1530_d_n11: f64 = (eq119_e1529_d_n11 * ddt_scale);
        let eq119_e1530_d_n12: f64 = (eq119_e1529_d_n12 * ddt_scale);
        let eq119_e1530_d_n13: f64 = (eq119_e1529_d_n13 * ddt_scale);
        let eq119_e1530_d_n14: f64 = (eq119_e1529_d_n14 * ddt_scale);
        let eq119_e1530_d_n15: f64 = (eq119_e1529_d_n15 * ddt_scale);
        let eq119_e1530_d_n16: f64 = (eq119_e1529_d_n16 * ddt_scale);
        let eq119_e1530_d_n17: f64 = (eq119_e1529_d_n17 * ddt_scale);
        let eq119_e1530_d_n18: f64 = (eq119_e1529_d_n18 * ddt_scale);
        let eq119_e1530_d_n19: f64 = (eq119_e1529_d_n19 * ddt_scale);
        let eq119_e1530_d_n20: f64 = (eq119_e1529_d_n20 * ddt_scale);
        let eq119_e1530_d_n21: f64 = (eq119_e1529_d_n21 * ddt_scale);
        let eq119_e1530_d_n22: f64 = (eq119_e1529_d_n22 * ddt_scale);
        let eq119_e1531: f64 = (p.p7 * eq119_e1530);
        let eq119_e1531_d_n0: f64 = (p.p7 * eq119_e1530_d_n0);
        let eq119_e1531_d_n1: f64 = (p.p7 * eq119_e1530_d_n1);
        let eq119_e1531_d_n2: f64 = (p.p7 * eq119_e1530_d_n2);
        let eq119_e1531_d_n3: f64 = (p.p7 * eq119_e1530_d_n3);
        let eq119_e1531_d_n4: f64 = (p.p7 * eq119_e1530_d_n4);
        let eq119_e1531_d_n5: f64 = (p.p7 * eq119_e1530_d_n5);
        let eq119_e1531_d_n6: f64 = (p.p7 * eq119_e1530_d_n6);
        let eq119_e1531_d_n7: f64 = (p.p7 * eq119_e1530_d_n7);
        let eq119_e1531_d_n8: f64 = (p.p7 * eq119_e1530_d_n8);
        let eq119_e1531_d_n9: f64 = (p.p7 * eq119_e1530_d_n9);
        let eq119_e1531_d_n10: f64 = (p.p7 * eq119_e1530_d_n10);
        let eq119_e1531_d_n11: f64 = (p.p7 * eq119_e1530_d_n11);
        let eq119_e1531_d_n12: f64 = (p.p7 * eq119_e1530_d_n12);
        let eq119_e1531_d_n13: f64 = (p.p7 * eq119_e1530_d_n13);
        let eq119_e1531_d_n14: f64 = (p.p7 * eq119_e1530_d_n14);
        let eq119_e1531_d_n15: f64 = (p.p7 * eq119_e1530_d_n15);
        let eq119_e1531_d_n16: f64 = (p.p7 * eq119_e1530_d_n16);
        let eq119_e1531_d_n17: f64 = (p.p7 * eq119_e1530_d_n17);
        let eq119_e1531_d_n18: f64 = (p.p7 * eq119_e1530_d_n18);
        let eq119_e1531_d_n19: f64 = (p.p7 * eq119_e1530_d_n19);
        let eq119_e1531_d_n20: f64 = (p.p7 * eq119_e1530_d_n20);
        let eq119_e1531_d_n21: f64 = (p.p7 * eq119_e1530_d_n21);
        let eq119_e1531_d_n22: f64 = (p.p7 * eq119_e1530_d_n22);
        let eq119_value: f64 = eq119_e1531;
        let eq119_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n10, eq119_e1531_d_n11, eq119_e1531_d_n12, eq119_e1531_d_n13, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            multiplicity * (eq119_value),
            nodes,
            &eq119_node_derivatives,
            branches,
            &eq119_branch_derivatives,
            multiplicity,
        );
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22,) = {
    if (s.b[570] && s.b[571]) {
        let eq120_e1537: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, s.v[229]);
        let eq120_e1537_d_n0: f64 = (s.dn[229][0] * ddt_scale);
        let eq120_e1537_d_n1: f64 = (s.dn[229][1] * ddt_scale);
        let eq120_e1537_d_n2: f64 = (s.dn[229][2] * ddt_scale);
        let eq120_e1537_d_n3: f64 = (s.dn[229][3] * ddt_scale);
        let eq120_e1537_d_n4: f64 = (s.dn[229][4] * ddt_scale);
        let eq120_e1537_d_n5: f64 = (s.dn[229][5] * ddt_scale);
        let eq120_e1537_d_n6: f64 = (s.dn[229][6] * ddt_scale);
        let eq120_e1537_d_n7: f64 = (s.dn[229][7] * ddt_scale);
        let eq120_e1537_d_n8: f64 = (s.dn[229][8] * ddt_scale);
        let eq120_e1537_d_n9: f64 = (s.dn[229][9] * ddt_scale);
        let eq120_e1537_d_n10: f64 = (s.dn[229][10] * ddt_scale);
        let eq120_e1537_d_n11: f64 = (s.dn[229][11] * ddt_scale);
        let eq120_e1537_d_n12: f64 = (s.dn[229][12] * ddt_scale);
        let eq120_e1537_d_n13: f64 = (s.dn[229][13] * ddt_scale);
        let eq120_e1537_d_n14: f64 = (s.dn[229][14] * ddt_scale);
        let eq120_e1537_d_n15: f64 = (s.dn[229][15] * ddt_scale);
        let eq120_e1537_d_n16: f64 = (s.dn[229][16] * ddt_scale);
        let eq120_e1537_d_n17: f64 = (s.dn[229][17] * ddt_scale);
        let eq120_e1537_d_n18: f64 = (s.dn[229][18] * ddt_scale);
        let eq120_e1537_d_n19: f64 = (s.dn[229][19] * ddt_scale);
        let eq120_e1537_d_n20: f64 = (s.dn[229][20] * ddt_scale);
        let eq120_e1537_d_n21: f64 = (s.dn[229][21] * ddt_scale);
        let eq120_e1537_d_n22: f64 = (s.dn[229][22] * ddt_scale);
        let eq120_e1538: f64 = (p.p7 * eq120_e1537);
        let eq120_e1538_d_n0: f64 = (p.p7 * eq120_e1537_d_n0);
        let eq120_e1538_d_n1: f64 = (p.p7 * eq120_e1537_d_n1);
        let eq120_e1538_d_n2: f64 = (p.p7 * eq120_e1537_d_n2);
        let eq120_e1538_d_n3: f64 = (p.p7 * eq120_e1537_d_n3);
        let eq120_e1538_d_n4: f64 = (p.p7 * eq120_e1537_d_n4);
        let eq120_e1538_d_n5: f64 = (p.p7 * eq120_e1537_d_n5);
        let eq120_e1538_d_n6: f64 = (p.p7 * eq120_e1537_d_n6);
        let eq120_e1538_d_n7: f64 = (p.p7 * eq120_e1537_d_n7);
        let eq120_e1538_d_n8: f64 = (p.p7 * eq120_e1537_d_n8);
        let eq120_e1538_d_n9: f64 = (p.p7 * eq120_e1537_d_n9);
        let eq120_e1538_d_n10: f64 = (p.p7 * eq120_e1537_d_n10);
        let eq120_e1538_d_n11: f64 = (p.p7 * eq120_e1537_d_n11);
        let eq120_e1538_d_n12: f64 = (p.p7 * eq120_e1537_d_n12);
        let eq120_e1538_d_n13: f64 = (p.p7 * eq120_e1537_d_n13);
        let eq120_e1538_d_n14: f64 = (p.p7 * eq120_e1537_d_n14);
        let eq120_e1538_d_n15: f64 = (p.p7 * eq120_e1537_d_n15);
        let eq120_e1538_d_n16: f64 = (p.p7 * eq120_e1537_d_n16);
        let eq120_e1538_d_n17: f64 = (p.p7 * eq120_e1537_d_n17);
        let eq120_e1538_d_n18: f64 = (p.p7 * eq120_e1537_d_n18);
        let eq120_e1538_d_n19: f64 = (p.p7 * eq120_e1537_d_n19);
        let eq120_e1538_d_n20: f64 = (p.p7 * eq120_e1537_d_n20);
        let eq120_e1538_d_n21: f64 = (p.p7 * eq120_e1537_d_n21);
        let eq120_e1538_d_n22: f64 = (p.p7 * eq120_e1537_d_n22);
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n10, eq120_e1538_d_n11, eq120_e1538_d_n12, eq120_e1538_d_n13, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_value: f64 = eq120_e1540;
        let eq120_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            multiplicity * (eq120_value),
            nodes,
            &eq120_node_derivatives,
            branches,
            &eq120_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq121_e1548: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, s.v[228]);
        let eq121_e1548_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq121_e1548_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq121_e1548_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq121_e1548_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq121_e1548_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq121_e1548_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq121_e1548_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq121_e1548_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq121_e1548_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq121_e1548_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq121_e1548_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq121_e1548_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq121_e1548_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq121_e1548_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq121_e1548_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq121_e1548_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq121_e1548_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq121_e1548_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq121_e1548_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq121_e1548_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq121_e1548_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq121_e1548_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq121_e1548_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq121_e1549: f64 = (p.p7 * eq121_e1548);
        let eq121_e1549_d_n0: f64 = (p.p7 * eq121_e1548_d_n0);
        let eq121_e1549_d_n1: f64 = (p.p7 * eq121_e1548_d_n1);
        let eq121_e1549_d_n2: f64 = (p.p7 * eq121_e1548_d_n2);
        let eq121_e1549_d_n3: f64 = (p.p7 * eq121_e1548_d_n3);
        let eq121_e1549_d_n4: f64 = (p.p7 * eq121_e1548_d_n4);
        let eq121_e1549_d_n5: f64 = (p.p7 * eq121_e1548_d_n5);
        let eq121_e1549_d_n6: f64 = (p.p7 * eq121_e1548_d_n6);
        let eq121_e1549_d_n7: f64 = (p.p7 * eq121_e1548_d_n7);
        let eq121_e1549_d_n8: f64 = (p.p7 * eq121_e1548_d_n8);
        let eq121_e1549_d_n9: f64 = (p.p7 * eq121_e1548_d_n9);
        let eq121_e1549_d_n10: f64 = (p.p7 * eq121_e1548_d_n10);
        let eq121_e1549_d_n11: f64 = (p.p7 * eq121_e1548_d_n11);
        let eq121_e1549_d_n12: f64 = (p.p7 * eq121_e1548_d_n12);
        let eq121_e1549_d_n13: f64 = (p.p7 * eq121_e1548_d_n13);
        let eq121_e1549_d_n14: f64 = (p.p7 * eq121_e1548_d_n14);
        let eq121_e1549_d_n15: f64 = (p.p7 * eq121_e1548_d_n15);
        let eq121_e1549_d_n16: f64 = (p.p7 * eq121_e1548_d_n16);
        let eq121_e1549_d_n17: f64 = (p.p7 * eq121_e1548_d_n17);
        let eq121_e1549_d_n18: f64 = (p.p7 * eq121_e1548_d_n18);
        let eq121_e1549_d_n19: f64 = (p.p7 * eq121_e1548_d_n19);
        let eq121_e1549_d_n20: f64 = (p.p7 * eq121_e1548_d_n20);
        let eq121_e1549_d_n21: f64 = (p.p7 * eq121_e1548_d_n21);
        let eq121_e1549_d_n22: f64 = (p.p7 * eq121_e1548_d_n22);
        (eq121_e1549, eq121_e1549_d_n0, eq121_e1549_d_n1, eq121_e1549_d_n2, eq121_e1549_d_n3, eq121_e1549_d_n4, eq121_e1549_d_n5, eq121_e1549_d_n6, eq121_e1549_d_n7, eq121_e1549_d_n8, eq121_e1549_d_n9, eq121_e1549_d_n10, eq121_e1549_d_n11, eq121_e1549_d_n12, eq121_e1549_d_n13, eq121_e1549_d_n14, eq121_e1549_d_n15, eq121_e1549_d_n16, eq121_e1549_d_n17, eq121_e1549_d_n18, eq121_e1549_d_n19, eq121_e1549_d_n20, eq121_e1549_d_n21, eq121_e1549_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_value: f64 = eq121_e1551;
        let eq121_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq121_value),
            nodes,
            &eq121_node_derivatives,
            branches,
            &eq121_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq122_e1559: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, s.v[228]);
        let eq122_e1559_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq122_e1559_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq122_e1559_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq122_e1559_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq122_e1559_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq122_e1559_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq122_e1559_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq122_e1559_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq122_e1559_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq122_e1559_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq122_e1559_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq122_e1559_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq122_e1559_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq122_e1559_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq122_e1559_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq122_e1559_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq122_e1559_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq122_e1559_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq122_e1559_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq122_e1559_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq122_e1559_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq122_e1559_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq122_e1559_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq122_e1560: f64 = (p.p7 * eq122_e1559);
        let eq122_e1560_d_n0: f64 = (p.p7 * eq122_e1559_d_n0);
        let eq122_e1560_d_n1: f64 = (p.p7 * eq122_e1559_d_n1);
        let eq122_e1560_d_n2: f64 = (p.p7 * eq122_e1559_d_n2);
        let eq122_e1560_d_n3: f64 = (p.p7 * eq122_e1559_d_n3);
        let eq122_e1560_d_n4: f64 = (p.p7 * eq122_e1559_d_n4);
        let eq122_e1560_d_n5: f64 = (p.p7 * eq122_e1559_d_n5);
        let eq122_e1560_d_n6: f64 = (p.p7 * eq122_e1559_d_n6);
        let eq122_e1560_d_n7: f64 = (p.p7 * eq122_e1559_d_n7);
        let eq122_e1560_d_n8: f64 = (p.p7 * eq122_e1559_d_n8);
        let eq122_e1560_d_n9: f64 = (p.p7 * eq122_e1559_d_n9);
        let eq122_e1560_d_n10: f64 = (p.p7 * eq122_e1559_d_n10);
        let eq122_e1560_d_n11: f64 = (p.p7 * eq122_e1559_d_n11);
        let eq122_e1560_d_n12: f64 = (p.p7 * eq122_e1559_d_n12);
        let eq122_e1560_d_n13: f64 = (p.p7 * eq122_e1559_d_n13);
        let eq122_e1560_d_n14: f64 = (p.p7 * eq122_e1559_d_n14);
        let eq122_e1560_d_n15: f64 = (p.p7 * eq122_e1559_d_n15);
        let eq122_e1560_d_n16: f64 = (p.p7 * eq122_e1559_d_n16);
        let eq122_e1560_d_n17: f64 = (p.p7 * eq122_e1559_d_n17);
        let eq122_e1560_d_n18: f64 = (p.p7 * eq122_e1559_d_n18);
        let eq122_e1560_d_n19: f64 = (p.p7 * eq122_e1559_d_n19);
        let eq122_e1560_d_n20: f64 = (p.p7 * eq122_e1559_d_n20);
        let eq122_e1560_d_n21: f64 = (p.p7 * eq122_e1559_d_n21);
        let eq122_e1560_d_n22: f64 = (p.p7 * eq122_e1559_d_n22);
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
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n10, eq122_e1562_d_n11, eq122_e1562_d_n12, eq122_e1562_d_n13, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1564;
        let eq122_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq122_value),
            nodes,
            &eq122_node_derivatives,
            branches,
            &eq122_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
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
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq123_e1573: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, s.v[228]);
        let eq123_e1573_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq123_e1573_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq123_e1573_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq123_e1573_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq123_e1573_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq123_e1573_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq123_e1573_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq123_e1573_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq123_e1573_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq123_e1573_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq123_e1573_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq123_e1573_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq123_e1573_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq123_e1573_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq123_e1573_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq123_e1573_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq123_e1573_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq123_e1573_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq123_e1573_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq123_e1573_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq123_e1573_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq123_e1573_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq123_e1573_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq123_e1574: f64 = (p.p7 * eq123_e1573);
        let eq123_e1574_d_n0: f64 = (p.p7 * eq123_e1573_d_n0);
        let eq123_e1574_d_n1: f64 = (p.p7 * eq123_e1573_d_n1);
        let eq123_e1574_d_n2: f64 = (p.p7 * eq123_e1573_d_n2);
        let eq123_e1574_d_n3: f64 = (p.p7 * eq123_e1573_d_n3);
        let eq123_e1574_d_n4: f64 = (p.p7 * eq123_e1573_d_n4);
        let eq123_e1574_d_n5: f64 = (p.p7 * eq123_e1573_d_n5);
        let eq123_e1574_d_n6: f64 = (p.p7 * eq123_e1573_d_n6);
        let eq123_e1574_d_n7: f64 = (p.p7 * eq123_e1573_d_n7);
        let eq123_e1574_d_n8: f64 = (p.p7 * eq123_e1573_d_n8);
        let eq123_e1574_d_n9: f64 = (p.p7 * eq123_e1573_d_n9);
        let eq123_e1574_d_n10: f64 = (p.p7 * eq123_e1573_d_n10);
        let eq123_e1574_d_n11: f64 = (p.p7 * eq123_e1573_d_n11);
        let eq123_e1574_d_n12: f64 = (p.p7 * eq123_e1573_d_n12);
        let eq123_e1574_d_n13: f64 = (p.p7 * eq123_e1573_d_n13);
        let eq123_e1574_d_n14: f64 = (p.p7 * eq123_e1573_d_n14);
        let eq123_e1574_d_n15: f64 = (p.p7 * eq123_e1573_d_n15);
        let eq123_e1574_d_n16: f64 = (p.p7 * eq123_e1573_d_n16);
        let eq123_e1574_d_n17: f64 = (p.p7 * eq123_e1573_d_n17);
        let eq123_e1574_d_n18: f64 = (p.p7 * eq123_e1573_d_n18);
        let eq123_e1574_d_n19: f64 = (p.p7 * eq123_e1573_d_n19);
        let eq123_e1574_d_n20: f64 = (p.p7 * eq123_e1573_d_n20);
        let eq123_e1574_d_n21: f64 = (p.p7 * eq123_e1573_d_n21);
        let eq123_e1574_d_n22: f64 = (p.p7 * eq123_e1573_d_n22);
        (eq123_e1574, eq123_e1574_d_n0, eq123_e1574_d_n1, eq123_e1574_d_n2, eq123_e1574_d_n3, eq123_e1574_d_n4, eq123_e1574_d_n5, eq123_e1574_d_n6, eq123_e1574_d_n7, eq123_e1574_d_n8, eq123_e1574_d_n9, eq123_e1574_d_n10, eq123_e1574_d_n11, eq123_e1574_d_n12, eq123_e1574_d_n13, eq123_e1574_d_n14, eq123_e1574_d_n15, eq123_e1574_d_n16, eq123_e1574_d_n17, eq123_e1574_d_n18, eq123_e1574_d_n19, eq123_e1574_d_n20, eq123_e1574_d_n21, eq123_e1574_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_value: f64 = eq123_e1576;
        let eq123_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq123_value),
            nodes,
            &eq123_node_derivatives,
            branches,
            &eq123_branch_derivatives,
            multiplicity,
        );
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq124_e1585: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 23, s.v[228]);
        let eq124_e1585_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq124_e1585_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq124_e1585_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq124_e1585_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq124_e1585_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq124_e1585_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq124_e1585_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq124_e1585_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq124_e1585_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq124_e1585_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq124_e1585_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq124_e1585_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq124_e1585_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq124_e1585_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq124_e1585_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq124_e1585_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq124_e1585_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq124_e1585_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq124_e1585_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq124_e1585_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq124_e1585_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq124_e1585_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq124_e1585_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq124_e1586: f64 = (p.p7 * eq124_e1585);
        let eq124_e1586_d_n0: f64 = (p.p7 * eq124_e1585_d_n0);
        let eq124_e1586_d_n1: f64 = (p.p7 * eq124_e1585_d_n1);
        let eq124_e1586_d_n2: f64 = (p.p7 * eq124_e1585_d_n2);
        let eq124_e1586_d_n3: f64 = (p.p7 * eq124_e1585_d_n3);
        let eq124_e1586_d_n4: f64 = (p.p7 * eq124_e1585_d_n4);
        let eq124_e1586_d_n5: f64 = (p.p7 * eq124_e1585_d_n5);
        let eq124_e1586_d_n6: f64 = (p.p7 * eq124_e1585_d_n6);
        let eq124_e1586_d_n7: f64 = (p.p7 * eq124_e1585_d_n7);
        let eq124_e1586_d_n8: f64 = (p.p7 * eq124_e1585_d_n8);
        let eq124_e1586_d_n9: f64 = (p.p7 * eq124_e1585_d_n9);
        let eq124_e1586_d_n10: f64 = (p.p7 * eq124_e1585_d_n10);
        let eq124_e1586_d_n11: f64 = (p.p7 * eq124_e1585_d_n11);
        let eq124_e1586_d_n12: f64 = (p.p7 * eq124_e1585_d_n12);
        let eq124_e1586_d_n13: f64 = (p.p7 * eq124_e1585_d_n13);
        let eq124_e1586_d_n14: f64 = (p.p7 * eq124_e1585_d_n14);
        let eq124_e1586_d_n15: f64 = (p.p7 * eq124_e1585_d_n15);
        let eq124_e1586_d_n16: f64 = (p.p7 * eq124_e1585_d_n16);
        let eq124_e1586_d_n17: f64 = (p.p7 * eq124_e1585_d_n17);
        let eq124_e1586_d_n18: f64 = (p.p7 * eq124_e1585_d_n18);
        let eq124_e1586_d_n19: f64 = (p.p7 * eq124_e1585_d_n19);
        let eq124_e1586_d_n20: f64 = (p.p7 * eq124_e1585_d_n20);
        let eq124_e1586_d_n21: f64 = (p.p7 * eq124_e1585_d_n21);
        let eq124_e1586_d_n22: f64 = (p.p7 * eq124_e1585_d_n22);
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
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1590;
        let eq124_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq124_value),
            nodes,
            &eq124_node_derivatives,
            branches,
            &eq124_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22,) = {
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
        let eq125_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 24, eq125_e1597);
        let eq125_e1598_d_n0: f64 = (eq125_e1597_d_n0 * ddt_scale);
        let eq125_e1598_d_n1: f64 = (eq125_e1597_d_n1 * ddt_scale);
        let eq125_e1598_d_n2: f64 = (eq125_e1597_d_n2 * ddt_scale);
        let eq125_e1598_d_n3: f64 = (eq125_e1597_d_n3 * ddt_scale);
        let eq125_e1598_d_n4: f64 = (eq125_e1597_d_n4 * ddt_scale);
        let eq125_e1598_d_n5: f64 = (eq125_e1597_d_n5 * ddt_scale);
        let eq125_e1598_d_n6: f64 = (eq125_e1597_d_n6 * ddt_scale);
        let eq125_e1598_d_n7: f64 = (eq125_e1597_d_n7 * ddt_scale);
        let eq125_e1598_d_n8: f64 = (eq125_e1597_d_n8 * ddt_scale);
        let eq125_e1598_d_n9: f64 = (eq125_e1597_d_n9 * ddt_scale);
        let eq125_e1598_d_n10: f64 = (eq125_e1597_d_n10 * ddt_scale);
        let eq125_e1598_d_n11: f64 = (eq125_e1597_d_n11 * ddt_scale);
        let eq125_e1598_d_n12: f64 = (eq125_e1597_d_n12 * ddt_scale);
        let eq125_e1598_d_n13: f64 = (eq125_e1597_d_n13 * ddt_scale);
        let eq125_e1598_d_n14: f64 = (eq125_e1597_d_n14 * ddt_scale);
        let eq125_e1598_d_n15: f64 = (eq125_e1597_d_n15 * ddt_scale);
        let eq125_e1598_d_n16: f64 = (eq125_e1597_d_n16 * ddt_scale);
        let eq125_e1598_d_n17: f64 = (eq125_e1597_d_n17 * ddt_scale);
        let eq125_e1598_d_n18: f64 = (eq125_e1597_d_n18 * ddt_scale);
        let eq125_e1598_d_n19: f64 = (eq125_e1597_d_n19 * ddt_scale);
        let eq125_e1598_d_n20: f64 = (eq125_e1597_d_n20 * ddt_scale);
        let eq125_e1598_d_n21: f64 = (eq125_e1597_d_n21 * ddt_scale);
        let eq125_e1598_d_n22: f64 = (eq125_e1597_d_n22 * ddt_scale);
        let eq125_e1599: f64 = (p.p7 * eq125_e1598);
        let eq125_e1599_d_n0: f64 = (p.p7 * eq125_e1598_d_n0);
        let eq125_e1599_d_n1: f64 = (p.p7 * eq125_e1598_d_n1);
        let eq125_e1599_d_n2: f64 = (p.p7 * eq125_e1598_d_n2);
        let eq125_e1599_d_n3: f64 = (p.p7 * eq125_e1598_d_n3);
        let eq125_e1599_d_n4: f64 = (p.p7 * eq125_e1598_d_n4);
        let eq125_e1599_d_n5: f64 = (p.p7 * eq125_e1598_d_n5);
        let eq125_e1599_d_n6: f64 = (p.p7 * eq125_e1598_d_n6);
        let eq125_e1599_d_n7: f64 = (p.p7 * eq125_e1598_d_n7);
        let eq125_e1599_d_n8: f64 = (p.p7 * eq125_e1598_d_n8);
        let eq125_e1599_d_n9: f64 = (p.p7 * eq125_e1598_d_n9);
        let eq125_e1599_d_n10: f64 = (p.p7 * eq125_e1598_d_n10);
        let eq125_e1599_d_n11: f64 = (p.p7 * eq125_e1598_d_n11);
        let eq125_e1599_d_n12: f64 = (p.p7 * eq125_e1598_d_n12);
        let eq125_e1599_d_n13: f64 = (p.p7 * eq125_e1598_d_n13);
        let eq125_e1599_d_n14: f64 = (p.p7 * eq125_e1598_d_n14);
        let eq125_e1599_d_n15: f64 = (p.p7 * eq125_e1598_d_n15);
        let eq125_e1599_d_n16: f64 = (p.p7 * eq125_e1598_d_n16);
        let eq125_e1599_d_n17: f64 = (p.p7 * eq125_e1598_d_n17);
        let eq125_e1599_d_n18: f64 = (p.p7 * eq125_e1598_d_n18);
        let eq125_e1599_d_n19: f64 = (p.p7 * eq125_e1598_d_n19);
        let eq125_e1599_d_n20: f64 = (p.p7 * eq125_e1598_d_n20);
        let eq125_e1599_d_n21: f64 = (p.p7 * eq125_e1598_d_n21);
        let eq125_e1599_d_n22: f64 = (p.p7 * eq125_e1598_d_n22);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1601;
        let eq125_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            multiplicity * (eq125_value),
            nodes,
            &eq125_node_derivatives,
            branches,
            &eq125_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq126_e1608: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 25, s.v[229]);
        let eq126_e1608_d_n0: f64 = (s.dn[229][0] * ddt_scale);
        let eq126_e1608_d_n1: f64 = (s.dn[229][1] * ddt_scale);
        let eq126_e1608_d_n2: f64 = (s.dn[229][2] * ddt_scale);
        let eq126_e1608_d_n3: f64 = (s.dn[229][3] * ddt_scale);
        let eq126_e1608_d_n4: f64 = (s.dn[229][4] * ddt_scale);
        let eq126_e1608_d_n5: f64 = (s.dn[229][5] * ddt_scale);
        let eq126_e1608_d_n6: f64 = (s.dn[229][6] * ddt_scale);
        let eq126_e1608_d_n7: f64 = (s.dn[229][7] * ddt_scale);
        let eq126_e1608_d_n8: f64 = (s.dn[229][8] * ddt_scale);
        let eq126_e1608_d_n9: f64 = (s.dn[229][9] * ddt_scale);
        let eq126_e1608_d_n10: f64 = (s.dn[229][10] * ddt_scale);
        let eq126_e1608_d_n11: f64 = (s.dn[229][11] * ddt_scale);
        let eq126_e1608_d_n12: f64 = (s.dn[229][12] * ddt_scale);
        let eq126_e1608_d_n13: f64 = (s.dn[229][13] * ddt_scale);
        let eq126_e1608_d_n14: f64 = (s.dn[229][14] * ddt_scale);
        let eq126_e1608_d_n15: f64 = (s.dn[229][15] * ddt_scale);
        let eq126_e1608_d_n16: f64 = (s.dn[229][16] * ddt_scale);
        let eq126_e1608_d_n17: f64 = (s.dn[229][17] * ddt_scale);
        let eq126_e1608_d_n18: f64 = (s.dn[229][18] * ddt_scale);
        let eq126_e1608_d_n19: f64 = (s.dn[229][19] * ddt_scale);
        let eq126_e1608_d_n20: f64 = (s.dn[229][20] * ddt_scale);
        let eq126_e1608_d_n21: f64 = (s.dn[229][21] * ddt_scale);
        let eq126_e1608_d_n22: f64 = (s.dn[229][22] * ddt_scale);
        let eq126_e1609: f64 = (p.p7 * eq126_e1608);
        let eq126_e1609_d_n0: f64 = (p.p7 * eq126_e1608_d_n0);
        let eq126_e1609_d_n1: f64 = (p.p7 * eq126_e1608_d_n1);
        let eq126_e1609_d_n2: f64 = (p.p7 * eq126_e1608_d_n2);
        let eq126_e1609_d_n3: f64 = (p.p7 * eq126_e1608_d_n3);
        let eq126_e1609_d_n4: f64 = (p.p7 * eq126_e1608_d_n4);
        let eq126_e1609_d_n5: f64 = (p.p7 * eq126_e1608_d_n5);
        let eq126_e1609_d_n6: f64 = (p.p7 * eq126_e1608_d_n6);
        let eq126_e1609_d_n7: f64 = (p.p7 * eq126_e1608_d_n7);
        let eq126_e1609_d_n8: f64 = (p.p7 * eq126_e1608_d_n8);
        let eq126_e1609_d_n9: f64 = (p.p7 * eq126_e1608_d_n9);
        let eq126_e1609_d_n10: f64 = (p.p7 * eq126_e1608_d_n10);
        let eq126_e1609_d_n11: f64 = (p.p7 * eq126_e1608_d_n11);
        let eq126_e1609_d_n12: f64 = (p.p7 * eq126_e1608_d_n12);
        let eq126_e1609_d_n13: f64 = (p.p7 * eq126_e1608_d_n13);
        let eq126_e1609_d_n14: f64 = (p.p7 * eq126_e1608_d_n14);
        let eq126_e1609_d_n15: f64 = (p.p7 * eq126_e1608_d_n15);
        let eq126_e1609_d_n16: f64 = (p.p7 * eq126_e1608_d_n16);
        let eq126_e1609_d_n17: f64 = (p.p7 * eq126_e1608_d_n17);
        let eq126_e1609_d_n18: f64 = (p.p7 * eq126_e1608_d_n18);
        let eq126_e1609_d_n19: f64 = (p.p7 * eq126_e1608_d_n19);
        let eq126_e1609_d_n20: f64 = (p.p7 * eq126_e1608_d_n20);
        let eq126_e1609_d_n21: f64 = (p.p7 * eq126_e1608_d_n21);
        let eq126_e1609_d_n22: f64 = (p.p7 * eq126_e1608_d_n22);
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1611;
        let eq126_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            multiplicity * (eq126_value),
            nodes,
            &eq126_node_derivatives,
            branches,
            &eq126_branch_derivatives,
            multiplicity,
        );
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq127_e1620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 26, s.v[228]);
        let eq127_e1620_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq127_e1620_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq127_e1620_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq127_e1620_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq127_e1620_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq127_e1620_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq127_e1620_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq127_e1620_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq127_e1620_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq127_e1620_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq127_e1620_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq127_e1620_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq127_e1620_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq127_e1620_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq127_e1620_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq127_e1620_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq127_e1620_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq127_e1620_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq127_e1620_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq127_e1620_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq127_e1620_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq127_e1620_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq127_e1620_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq127_e1621: f64 = (p.p7 * eq127_e1620);
        let eq127_e1621_d_n0: f64 = (p.p7 * eq127_e1620_d_n0);
        let eq127_e1621_d_n1: f64 = (p.p7 * eq127_e1620_d_n1);
        let eq127_e1621_d_n2: f64 = (p.p7 * eq127_e1620_d_n2);
        let eq127_e1621_d_n3: f64 = (p.p7 * eq127_e1620_d_n3);
        let eq127_e1621_d_n4: f64 = (p.p7 * eq127_e1620_d_n4);
        let eq127_e1621_d_n5: f64 = (p.p7 * eq127_e1620_d_n5);
        let eq127_e1621_d_n6: f64 = (p.p7 * eq127_e1620_d_n6);
        let eq127_e1621_d_n7: f64 = (p.p7 * eq127_e1620_d_n7);
        let eq127_e1621_d_n8: f64 = (p.p7 * eq127_e1620_d_n8);
        let eq127_e1621_d_n9: f64 = (p.p7 * eq127_e1620_d_n9);
        let eq127_e1621_d_n10: f64 = (p.p7 * eq127_e1620_d_n10);
        let eq127_e1621_d_n11: f64 = (p.p7 * eq127_e1620_d_n11);
        let eq127_e1621_d_n12: f64 = (p.p7 * eq127_e1620_d_n12);
        let eq127_e1621_d_n13: f64 = (p.p7 * eq127_e1620_d_n13);
        let eq127_e1621_d_n14: f64 = (p.p7 * eq127_e1620_d_n14);
        let eq127_e1621_d_n15: f64 = (p.p7 * eq127_e1620_d_n15);
        let eq127_e1621_d_n16: f64 = (p.p7 * eq127_e1620_d_n16);
        let eq127_e1621_d_n17: f64 = (p.p7 * eq127_e1620_d_n17);
        let eq127_e1621_d_n18: f64 = (p.p7 * eq127_e1620_d_n18);
        let eq127_e1621_d_n19: f64 = (p.p7 * eq127_e1620_d_n19);
        let eq127_e1621_d_n20: f64 = (p.p7 * eq127_e1620_d_n20);
        let eq127_e1621_d_n21: f64 = (p.p7 * eq127_e1620_d_n21);
        let eq127_e1621_d_n22: f64 = (p.p7 * eq127_e1620_d_n22);
        (eq127_e1621, eq127_e1621_d_n0, eq127_e1621_d_n1, eq127_e1621_d_n2, eq127_e1621_d_n3, eq127_e1621_d_n4, eq127_e1621_d_n5, eq127_e1621_d_n6, eq127_e1621_d_n7, eq127_e1621_d_n8, eq127_e1621_d_n9, eq127_e1621_d_n10, eq127_e1621_d_n11, eq127_e1621_d_n12, eq127_e1621_d_n13, eq127_e1621_d_n14, eq127_e1621_d_n15, eq127_e1621_d_n16, eq127_e1621_d_n17, eq127_e1621_d_n18, eq127_e1621_d_n19, eq127_e1621_d_n20, eq127_e1621_d_n21, eq127_e1621_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_value: f64 = eq127_e1623;
        let eq127_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq127_value),
            nodes,
            &eq127_node_derivatives,
            branches,
            &eq127_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq128_e1632: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 27, s.v[228]);
        let eq128_e1632_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq128_e1632_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq128_e1632_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq128_e1632_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq128_e1632_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq128_e1632_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq128_e1632_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq128_e1632_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq128_e1632_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq128_e1632_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq128_e1632_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq128_e1632_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq128_e1632_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq128_e1632_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq128_e1632_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq128_e1632_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq128_e1632_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq128_e1632_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq128_e1632_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq128_e1632_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq128_e1632_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq128_e1632_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq128_e1632_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq128_e1633: f64 = (p.p7 * eq128_e1632);
        let eq128_e1633_d_n0: f64 = (p.p7 * eq128_e1632_d_n0);
        let eq128_e1633_d_n1: f64 = (p.p7 * eq128_e1632_d_n1);
        let eq128_e1633_d_n2: f64 = (p.p7 * eq128_e1632_d_n2);
        let eq128_e1633_d_n3: f64 = (p.p7 * eq128_e1632_d_n3);
        let eq128_e1633_d_n4: f64 = (p.p7 * eq128_e1632_d_n4);
        let eq128_e1633_d_n5: f64 = (p.p7 * eq128_e1632_d_n5);
        let eq128_e1633_d_n6: f64 = (p.p7 * eq128_e1632_d_n6);
        let eq128_e1633_d_n7: f64 = (p.p7 * eq128_e1632_d_n7);
        let eq128_e1633_d_n8: f64 = (p.p7 * eq128_e1632_d_n8);
        let eq128_e1633_d_n9: f64 = (p.p7 * eq128_e1632_d_n9);
        let eq128_e1633_d_n10: f64 = (p.p7 * eq128_e1632_d_n10);
        let eq128_e1633_d_n11: f64 = (p.p7 * eq128_e1632_d_n11);
        let eq128_e1633_d_n12: f64 = (p.p7 * eq128_e1632_d_n12);
        let eq128_e1633_d_n13: f64 = (p.p7 * eq128_e1632_d_n13);
        let eq128_e1633_d_n14: f64 = (p.p7 * eq128_e1632_d_n14);
        let eq128_e1633_d_n15: f64 = (p.p7 * eq128_e1632_d_n15);
        let eq128_e1633_d_n16: f64 = (p.p7 * eq128_e1632_d_n16);
        let eq128_e1633_d_n17: f64 = (p.p7 * eq128_e1632_d_n17);
        let eq128_e1633_d_n18: f64 = (p.p7 * eq128_e1632_d_n18);
        let eq128_e1633_d_n19: f64 = (p.p7 * eq128_e1632_d_n19);
        let eq128_e1633_d_n20: f64 = (p.p7 * eq128_e1632_d_n20);
        let eq128_e1633_d_n21: f64 = (p.p7 * eq128_e1632_d_n21);
        let eq128_e1633_d_n22: f64 = (p.p7 * eq128_e1632_d_n22);
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
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1637;
        let eq128_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq128_value),
            nodes,
            &eq128_node_derivatives,
            branches,
            &eq128_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
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
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq129_e1647: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 28, s.v[228]);
        let eq129_e1647_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq129_e1647_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq129_e1647_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq129_e1647_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq129_e1647_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq129_e1647_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq129_e1647_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq129_e1647_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq129_e1647_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq129_e1647_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq129_e1647_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq129_e1647_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq129_e1647_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq129_e1647_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq129_e1647_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq129_e1647_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq129_e1647_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq129_e1647_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq129_e1647_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq129_e1647_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq129_e1647_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq129_e1647_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq129_e1647_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq129_e1648: f64 = (p.p7 * eq129_e1647);
        let eq129_e1648_d_n0: f64 = (p.p7 * eq129_e1647_d_n0);
        let eq129_e1648_d_n1: f64 = (p.p7 * eq129_e1647_d_n1);
        let eq129_e1648_d_n2: f64 = (p.p7 * eq129_e1647_d_n2);
        let eq129_e1648_d_n3: f64 = (p.p7 * eq129_e1647_d_n3);
        let eq129_e1648_d_n4: f64 = (p.p7 * eq129_e1647_d_n4);
        let eq129_e1648_d_n5: f64 = (p.p7 * eq129_e1647_d_n5);
        let eq129_e1648_d_n6: f64 = (p.p7 * eq129_e1647_d_n6);
        let eq129_e1648_d_n7: f64 = (p.p7 * eq129_e1647_d_n7);
        let eq129_e1648_d_n8: f64 = (p.p7 * eq129_e1647_d_n8);
        let eq129_e1648_d_n9: f64 = (p.p7 * eq129_e1647_d_n9);
        let eq129_e1648_d_n10: f64 = (p.p7 * eq129_e1647_d_n10);
        let eq129_e1648_d_n11: f64 = (p.p7 * eq129_e1647_d_n11);
        let eq129_e1648_d_n12: f64 = (p.p7 * eq129_e1647_d_n12);
        let eq129_e1648_d_n13: f64 = (p.p7 * eq129_e1647_d_n13);
        let eq129_e1648_d_n14: f64 = (p.p7 * eq129_e1647_d_n14);
        let eq129_e1648_d_n15: f64 = (p.p7 * eq129_e1647_d_n15);
        let eq129_e1648_d_n16: f64 = (p.p7 * eq129_e1647_d_n16);
        let eq129_e1648_d_n17: f64 = (p.p7 * eq129_e1647_d_n17);
        let eq129_e1648_d_n18: f64 = (p.p7 * eq129_e1647_d_n18);
        let eq129_e1648_d_n19: f64 = (p.p7 * eq129_e1647_d_n19);
        let eq129_e1648_d_n20: f64 = (p.p7 * eq129_e1647_d_n20);
        let eq129_e1648_d_n21: f64 = (p.p7 * eq129_e1647_d_n21);
        let eq129_e1648_d_n22: f64 = (p.p7 * eq129_e1647_d_n22);
        (eq129_e1648, eq129_e1648_d_n0, eq129_e1648_d_n1, eq129_e1648_d_n2, eq129_e1648_d_n3, eq129_e1648_d_n4, eq129_e1648_d_n5, eq129_e1648_d_n6, eq129_e1648_d_n7, eq129_e1648_d_n8, eq129_e1648_d_n9, eq129_e1648_d_n10, eq129_e1648_d_n11, eq129_e1648_d_n12, eq129_e1648_d_n13, eq129_e1648_d_n14, eq129_e1648_d_n15, eq129_e1648_d_n16, eq129_e1648_d_n17, eq129_e1648_d_n18, eq129_e1648_d_n19, eq129_e1648_d_n20, eq129_e1648_d_n21, eq129_e1648_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1650;
        let eq129_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq129_value),
            nodes,
            &eq129_node_derivatives,
            branches,
            &eq129_branch_derivatives,
            multiplicity,
        );
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq130_e1660: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 29, s.v[228]);
        let eq130_e1660_d_n0: f64 = (s.dn[228][0] * ddt_scale);
        let eq130_e1660_d_n1: f64 = (s.dn[228][1] * ddt_scale);
        let eq130_e1660_d_n2: f64 = (s.dn[228][2] * ddt_scale);
        let eq130_e1660_d_n3: f64 = (s.dn[228][3] * ddt_scale);
        let eq130_e1660_d_n4: f64 = (s.dn[228][4] * ddt_scale);
        let eq130_e1660_d_n5: f64 = (s.dn[228][5] * ddt_scale);
        let eq130_e1660_d_n6: f64 = (s.dn[228][6] * ddt_scale);
        let eq130_e1660_d_n7: f64 = (s.dn[228][7] * ddt_scale);
        let eq130_e1660_d_n8: f64 = (s.dn[228][8] * ddt_scale);
        let eq130_e1660_d_n9: f64 = (s.dn[228][9] * ddt_scale);
        let eq130_e1660_d_n10: f64 = (s.dn[228][10] * ddt_scale);
        let eq130_e1660_d_n11: f64 = (s.dn[228][11] * ddt_scale);
        let eq130_e1660_d_n12: f64 = (s.dn[228][12] * ddt_scale);
        let eq130_e1660_d_n13: f64 = (s.dn[228][13] * ddt_scale);
        let eq130_e1660_d_n14: f64 = (s.dn[228][14] * ddt_scale);
        let eq130_e1660_d_n15: f64 = (s.dn[228][15] * ddt_scale);
        let eq130_e1660_d_n16: f64 = (s.dn[228][16] * ddt_scale);
        let eq130_e1660_d_n17: f64 = (s.dn[228][17] * ddt_scale);
        let eq130_e1660_d_n18: f64 = (s.dn[228][18] * ddt_scale);
        let eq130_e1660_d_n19: f64 = (s.dn[228][19] * ddt_scale);
        let eq130_e1660_d_n20: f64 = (s.dn[228][20] * ddt_scale);
        let eq130_e1660_d_n21: f64 = (s.dn[228][21] * ddt_scale);
        let eq130_e1660_d_n22: f64 = (s.dn[228][22] * ddt_scale);
        let eq130_e1661: f64 = (p.p7 * eq130_e1660);
        let eq130_e1661_d_n0: f64 = (p.p7 * eq130_e1660_d_n0);
        let eq130_e1661_d_n1: f64 = (p.p7 * eq130_e1660_d_n1);
        let eq130_e1661_d_n2: f64 = (p.p7 * eq130_e1660_d_n2);
        let eq130_e1661_d_n3: f64 = (p.p7 * eq130_e1660_d_n3);
        let eq130_e1661_d_n4: f64 = (p.p7 * eq130_e1660_d_n4);
        let eq130_e1661_d_n5: f64 = (p.p7 * eq130_e1660_d_n5);
        let eq130_e1661_d_n6: f64 = (p.p7 * eq130_e1660_d_n6);
        let eq130_e1661_d_n7: f64 = (p.p7 * eq130_e1660_d_n7);
        let eq130_e1661_d_n8: f64 = (p.p7 * eq130_e1660_d_n8);
        let eq130_e1661_d_n9: f64 = (p.p7 * eq130_e1660_d_n9);
        let eq130_e1661_d_n10: f64 = (p.p7 * eq130_e1660_d_n10);
        let eq130_e1661_d_n11: f64 = (p.p7 * eq130_e1660_d_n11);
        let eq130_e1661_d_n12: f64 = (p.p7 * eq130_e1660_d_n12);
        let eq130_e1661_d_n13: f64 = (p.p7 * eq130_e1660_d_n13);
        let eq130_e1661_d_n14: f64 = (p.p7 * eq130_e1660_d_n14);
        let eq130_e1661_d_n15: f64 = (p.p7 * eq130_e1660_d_n15);
        let eq130_e1661_d_n16: f64 = (p.p7 * eq130_e1660_d_n16);
        let eq130_e1661_d_n17: f64 = (p.p7 * eq130_e1660_d_n17);
        let eq130_e1661_d_n18: f64 = (p.p7 * eq130_e1660_d_n18);
        let eq130_e1661_d_n19: f64 = (p.p7 * eq130_e1660_d_n19);
        let eq130_e1661_d_n20: f64 = (p.p7 * eq130_e1660_d_n20);
        let eq130_e1661_d_n21: f64 = (p.p7 * eq130_e1660_d_n21);
        let eq130_e1661_d_n22: f64 = (p.p7 * eq130_e1660_d_n22);
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
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1665;
        let eq130_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq130_value),
            nodes,
            &eq130_node_derivatives,
            branches,
            &eq130_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22,) = {
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
        let eq131_e1674: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 30, eq131_e1673);
        let eq131_e1674_d_n0: f64 = (eq131_e1673_d_n0 * ddt_scale);
        let eq131_e1674_d_n1: f64 = (eq131_e1673_d_n1 * ddt_scale);
        let eq131_e1674_d_n2: f64 = (eq131_e1673_d_n2 * ddt_scale);
        let eq131_e1674_d_n3: f64 = (eq131_e1673_d_n3 * ddt_scale);
        let eq131_e1674_d_n4: f64 = (eq131_e1673_d_n4 * ddt_scale);
        let eq131_e1674_d_n5: f64 = (eq131_e1673_d_n5 * ddt_scale);
        let eq131_e1674_d_n6: f64 = (eq131_e1673_d_n6 * ddt_scale);
        let eq131_e1674_d_n7: f64 = (eq131_e1673_d_n7 * ddt_scale);
        let eq131_e1674_d_n8: f64 = (eq131_e1673_d_n8 * ddt_scale);
        let eq131_e1674_d_n9: f64 = (eq131_e1673_d_n9 * ddt_scale);
        let eq131_e1674_d_n10: f64 = (eq131_e1673_d_n10 * ddt_scale);
        let eq131_e1674_d_n11: f64 = (eq131_e1673_d_n11 * ddt_scale);
        let eq131_e1674_d_n12: f64 = (eq131_e1673_d_n12 * ddt_scale);
        let eq131_e1674_d_n13: f64 = (eq131_e1673_d_n13 * ddt_scale);
        let eq131_e1674_d_n14: f64 = (eq131_e1673_d_n14 * ddt_scale);
        let eq131_e1674_d_n15: f64 = (eq131_e1673_d_n15 * ddt_scale);
        let eq131_e1674_d_n16: f64 = (eq131_e1673_d_n16 * ddt_scale);
        let eq131_e1674_d_n17: f64 = (eq131_e1673_d_n17 * ddt_scale);
        let eq131_e1674_d_n18: f64 = (eq131_e1673_d_n18 * ddt_scale);
        let eq131_e1674_d_n19: f64 = (eq131_e1673_d_n19 * ddt_scale);
        let eq131_e1674_d_n20: f64 = (eq131_e1673_d_n20 * ddt_scale);
        let eq131_e1674_d_n21: f64 = (eq131_e1673_d_n21 * ddt_scale);
        let eq131_e1674_d_n22: f64 = (eq131_e1673_d_n22 * ddt_scale);
        let eq131_e1675: f64 = (p.p7 * eq131_e1674);
        let eq131_e1675_d_n0: f64 = (p.p7 * eq131_e1674_d_n0);
        let eq131_e1675_d_n1: f64 = (p.p7 * eq131_e1674_d_n1);
        let eq131_e1675_d_n2: f64 = (p.p7 * eq131_e1674_d_n2);
        let eq131_e1675_d_n3: f64 = (p.p7 * eq131_e1674_d_n3);
        let eq131_e1675_d_n4: f64 = (p.p7 * eq131_e1674_d_n4);
        let eq131_e1675_d_n5: f64 = (p.p7 * eq131_e1674_d_n5);
        let eq131_e1675_d_n6: f64 = (p.p7 * eq131_e1674_d_n6);
        let eq131_e1675_d_n7: f64 = (p.p7 * eq131_e1674_d_n7);
        let eq131_e1675_d_n8: f64 = (p.p7 * eq131_e1674_d_n8);
        let eq131_e1675_d_n9: f64 = (p.p7 * eq131_e1674_d_n9);
        let eq131_e1675_d_n10: f64 = (p.p7 * eq131_e1674_d_n10);
        let eq131_e1675_d_n11: f64 = (p.p7 * eq131_e1674_d_n11);
        let eq131_e1675_d_n12: f64 = (p.p7 * eq131_e1674_d_n12);
        let eq131_e1675_d_n13: f64 = (p.p7 * eq131_e1674_d_n13);
        let eq131_e1675_d_n14: f64 = (p.p7 * eq131_e1674_d_n14);
        let eq131_e1675_d_n15: f64 = (p.p7 * eq131_e1674_d_n15);
        let eq131_e1675_d_n16: f64 = (p.p7 * eq131_e1674_d_n16);
        let eq131_e1675_d_n17: f64 = (p.p7 * eq131_e1674_d_n17);
        let eq131_e1675_d_n18: f64 = (p.p7 * eq131_e1674_d_n18);
        let eq131_e1675_d_n19: f64 = (p.p7 * eq131_e1674_d_n19);
        let eq131_e1675_d_n20: f64 = (p.p7 * eq131_e1674_d_n20);
        let eq131_e1675_d_n21: f64 = (p.p7 * eq131_e1674_d_n21);
        let eq131_e1675_d_n22: f64 = (p.p7 * eq131_e1674_d_n22);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1677;
        let eq131_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            multiplicity * (eq131_value),
            nodes,
            &eq131_node_derivatives,
            branches,
            &eq131_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22,) = {
    if (s.b[575] && s.b[576]) {
        let eq132_e1683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 31, s.v[241]);
        let eq132_e1683_d_n0: f64 = (s.dn[241][0] * ddt_scale);
        let eq132_e1683_d_n1: f64 = (s.dn[241][1] * ddt_scale);
        let eq132_e1683_d_n2: f64 = (s.dn[241][2] * ddt_scale);
        let eq132_e1683_d_n3: f64 = (s.dn[241][3] * ddt_scale);
        let eq132_e1683_d_n4: f64 = (s.dn[241][4] * ddt_scale);
        let eq132_e1683_d_n5: f64 = (s.dn[241][5] * ddt_scale);
        let eq132_e1683_d_n6: f64 = (s.dn[241][6] * ddt_scale);
        let eq132_e1683_d_n7: f64 = (s.dn[241][7] * ddt_scale);
        let eq132_e1683_d_n8: f64 = (s.dn[241][8] * ddt_scale);
        let eq132_e1683_d_n9: f64 = (s.dn[241][9] * ddt_scale);
        let eq132_e1683_d_n10: f64 = (s.dn[241][10] * ddt_scale);
        let eq132_e1683_d_n11: f64 = (s.dn[241][11] * ddt_scale);
        let eq132_e1683_d_n12: f64 = (s.dn[241][12] * ddt_scale);
        let eq132_e1683_d_n13: f64 = (s.dn[241][13] * ddt_scale);
        let eq132_e1683_d_n14: f64 = (s.dn[241][14] * ddt_scale);
        let eq132_e1683_d_n15: f64 = (s.dn[241][15] * ddt_scale);
        let eq132_e1683_d_n16: f64 = (s.dn[241][16] * ddt_scale);
        let eq132_e1683_d_n17: f64 = (s.dn[241][17] * ddt_scale);
        let eq132_e1683_d_n18: f64 = (s.dn[241][18] * ddt_scale);
        let eq132_e1683_d_n19: f64 = (s.dn[241][19] * ddt_scale);
        let eq132_e1683_d_n20: f64 = (s.dn[241][20] * ddt_scale);
        let eq132_e1683_d_n21: f64 = (s.dn[241][21] * ddt_scale);
        let eq132_e1683_d_n22: f64 = (s.dn[241][22] * ddt_scale);
        let eq132_e1684: f64 = (p.p7 * eq132_e1683);
        let eq132_e1684_d_n0: f64 = (p.p7 * eq132_e1683_d_n0);
        let eq132_e1684_d_n1: f64 = (p.p7 * eq132_e1683_d_n1);
        let eq132_e1684_d_n2: f64 = (p.p7 * eq132_e1683_d_n2);
        let eq132_e1684_d_n3: f64 = (p.p7 * eq132_e1683_d_n3);
        let eq132_e1684_d_n4: f64 = (p.p7 * eq132_e1683_d_n4);
        let eq132_e1684_d_n5: f64 = (p.p7 * eq132_e1683_d_n5);
        let eq132_e1684_d_n6: f64 = (p.p7 * eq132_e1683_d_n6);
        let eq132_e1684_d_n7: f64 = (p.p7 * eq132_e1683_d_n7);
        let eq132_e1684_d_n8: f64 = (p.p7 * eq132_e1683_d_n8);
        let eq132_e1684_d_n9: f64 = (p.p7 * eq132_e1683_d_n9);
        let eq132_e1684_d_n10: f64 = (p.p7 * eq132_e1683_d_n10);
        let eq132_e1684_d_n11: f64 = (p.p7 * eq132_e1683_d_n11);
        let eq132_e1684_d_n12: f64 = (p.p7 * eq132_e1683_d_n12);
        let eq132_e1684_d_n13: f64 = (p.p7 * eq132_e1683_d_n13);
        let eq132_e1684_d_n14: f64 = (p.p7 * eq132_e1683_d_n14);
        let eq132_e1684_d_n15: f64 = (p.p7 * eq132_e1683_d_n15);
        let eq132_e1684_d_n16: f64 = (p.p7 * eq132_e1683_d_n16);
        let eq132_e1684_d_n17: f64 = (p.p7 * eq132_e1683_d_n17);
        let eq132_e1684_d_n18: f64 = (p.p7 * eq132_e1683_d_n18);
        let eq132_e1684_d_n19: f64 = (p.p7 * eq132_e1683_d_n19);
        let eq132_e1684_d_n20: f64 = (p.p7 * eq132_e1683_d_n20);
        let eq132_e1684_d_n21: f64 = (p.p7 * eq132_e1683_d_n21);
        let eq132_e1684_d_n22: f64 = (p.p7 * eq132_e1683_d_n22);
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_value: f64 = eq132_e1686;
        let eq132_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            multiplicity * (eq132_value),
            nodes,
            &eq132_node_derivatives,
            branches,
            &eq132_branch_derivatives,
            multiplicity,
        );
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq133_e1694: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 32, s.v[240]);
        let eq133_e1694_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq133_e1694_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq133_e1694_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq133_e1694_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq133_e1694_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq133_e1694_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq133_e1694_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq133_e1694_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq133_e1694_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq133_e1694_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq133_e1694_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq133_e1694_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq133_e1694_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq133_e1694_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq133_e1694_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq133_e1694_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq133_e1694_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq133_e1694_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq133_e1694_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq133_e1694_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq133_e1694_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq133_e1694_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq133_e1694_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq133_e1695: f64 = (p.p7 * eq133_e1694);
        let eq133_e1695_d_n0: f64 = (p.p7 * eq133_e1694_d_n0);
        let eq133_e1695_d_n1: f64 = (p.p7 * eq133_e1694_d_n1);
        let eq133_e1695_d_n2: f64 = (p.p7 * eq133_e1694_d_n2);
        let eq133_e1695_d_n3: f64 = (p.p7 * eq133_e1694_d_n3);
        let eq133_e1695_d_n4: f64 = (p.p7 * eq133_e1694_d_n4);
        let eq133_e1695_d_n5: f64 = (p.p7 * eq133_e1694_d_n5);
        let eq133_e1695_d_n6: f64 = (p.p7 * eq133_e1694_d_n6);
        let eq133_e1695_d_n7: f64 = (p.p7 * eq133_e1694_d_n7);
        let eq133_e1695_d_n8: f64 = (p.p7 * eq133_e1694_d_n8);
        let eq133_e1695_d_n9: f64 = (p.p7 * eq133_e1694_d_n9);
        let eq133_e1695_d_n10: f64 = (p.p7 * eq133_e1694_d_n10);
        let eq133_e1695_d_n11: f64 = (p.p7 * eq133_e1694_d_n11);
        let eq133_e1695_d_n12: f64 = (p.p7 * eq133_e1694_d_n12);
        let eq133_e1695_d_n13: f64 = (p.p7 * eq133_e1694_d_n13);
        let eq133_e1695_d_n14: f64 = (p.p7 * eq133_e1694_d_n14);
        let eq133_e1695_d_n15: f64 = (p.p7 * eq133_e1694_d_n15);
        let eq133_e1695_d_n16: f64 = (p.p7 * eq133_e1694_d_n16);
        let eq133_e1695_d_n17: f64 = (p.p7 * eq133_e1694_d_n17);
        let eq133_e1695_d_n18: f64 = (p.p7 * eq133_e1694_d_n18);
        let eq133_e1695_d_n19: f64 = (p.p7 * eq133_e1694_d_n19);
        let eq133_e1695_d_n20: f64 = (p.p7 * eq133_e1694_d_n20);
        let eq133_e1695_d_n21: f64 = (p.p7 * eq133_e1694_d_n21);
        let eq133_e1695_d_n22: f64 = (p.p7 * eq133_e1694_d_n22);
        (eq133_e1695, eq133_e1695_d_n0, eq133_e1695_d_n1, eq133_e1695_d_n2, eq133_e1695_d_n3, eq133_e1695_d_n4, eq133_e1695_d_n5, eq133_e1695_d_n6, eq133_e1695_d_n7, eq133_e1695_d_n8, eq133_e1695_d_n9, eq133_e1695_d_n10, eq133_e1695_d_n11, eq133_e1695_d_n12, eq133_e1695_d_n13, eq133_e1695_d_n14, eq133_e1695_d_n15, eq133_e1695_d_n16, eq133_e1695_d_n17, eq133_e1695_d_n18, eq133_e1695_d_n19, eq133_e1695_d_n20, eq133_e1695_d_n21, eq133_e1695_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_value: f64 = eq133_e1697;
        let eq133_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            multiplicity * (eq133_value),
            nodes,
            &eq133_node_derivatives,
            branches,
            &eq133_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq134_e1705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 33, s.v[240]);
        let eq134_e1705_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq134_e1705_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq134_e1705_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq134_e1705_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq134_e1705_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq134_e1705_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq134_e1705_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq134_e1705_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq134_e1705_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq134_e1705_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq134_e1705_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq134_e1705_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq134_e1705_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq134_e1705_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq134_e1705_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq134_e1705_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq134_e1705_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq134_e1705_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq134_e1705_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq134_e1705_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq134_e1705_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq134_e1705_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq134_e1705_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq134_e1706: f64 = (p.p7 * eq134_e1705);
        let eq134_e1706_d_n0: f64 = (p.p7 * eq134_e1705_d_n0);
        let eq134_e1706_d_n1: f64 = (p.p7 * eq134_e1705_d_n1);
        let eq134_e1706_d_n2: f64 = (p.p7 * eq134_e1705_d_n2);
        let eq134_e1706_d_n3: f64 = (p.p7 * eq134_e1705_d_n3);
        let eq134_e1706_d_n4: f64 = (p.p7 * eq134_e1705_d_n4);
        let eq134_e1706_d_n5: f64 = (p.p7 * eq134_e1705_d_n5);
        let eq134_e1706_d_n6: f64 = (p.p7 * eq134_e1705_d_n6);
        let eq134_e1706_d_n7: f64 = (p.p7 * eq134_e1705_d_n7);
        let eq134_e1706_d_n8: f64 = (p.p7 * eq134_e1705_d_n8);
        let eq134_e1706_d_n9: f64 = (p.p7 * eq134_e1705_d_n9);
        let eq134_e1706_d_n10: f64 = (p.p7 * eq134_e1705_d_n10);
        let eq134_e1706_d_n11: f64 = (p.p7 * eq134_e1705_d_n11);
        let eq134_e1706_d_n12: f64 = (p.p7 * eq134_e1705_d_n12);
        let eq134_e1706_d_n13: f64 = (p.p7 * eq134_e1705_d_n13);
        let eq134_e1706_d_n14: f64 = (p.p7 * eq134_e1705_d_n14);
        let eq134_e1706_d_n15: f64 = (p.p7 * eq134_e1705_d_n15);
        let eq134_e1706_d_n16: f64 = (p.p7 * eq134_e1705_d_n16);
        let eq134_e1706_d_n17: f64 = (p.p7 * eq134_e1705_d_n17);
        let eq134_e1706_d_n18: f64 = (p.p7 * eq134_e1705_d_n18);
        let eq134_e1706_d_n19: f64 = (p.p7 * eq134_e1705_d_n19);
        let eq134_e1706_d_n20: f64 = (p.p7 * eq134_e1705_d_n20);
        let eq134_e1706_d_n21: f64 = (p.p7 * eq134_e1705_d_n21);
        let eq134_e1706_d_n22: f64 = (p.p7 * eq134_e1705_d_n22);
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
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_value: f64 = eq134_e1710;
        let eq134_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            multiplicity * (eq134_value),
            nodes,
            &eq134_node_derivatives,
            branches,
            &eq134_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
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
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq135_e1719: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 34, s.v[240]);
        let eq135_e1719_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq135_e1719_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq135_e1719_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq135_e1719_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq135_e1719_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq135_e1719_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq135_e1719_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq135_e1719_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq135_e1719_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq135_e1719_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq135_e1719_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq135_e1719_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq135_e1719_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq135_e1719_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq135_e1719_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq135_e1719_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq135_e1719_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq135_e1719_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq135_e1719_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq135_e1719_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq135_e1719_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq135_e1719_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq135_e1719_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq135_e1720: f64 = (p.p7 * eq135_e1719);
        let eq135_e1720_d_n0: f64 = (p.p7 * eq135_e1719_d_n0);
        let eq135_e1720_d_n1: f64 = (p.p7 * eq135_e1719_d_n1);
        let eq135_e1720_d_n2: f64 = (p.p7 * eq135_e1719_d_n2);
        let eq135_e1720_d_n3: f64 = (p.p7 * eq135_e1719_d_n3);
        let eq135_e1720_d_n4: f64 = (p.p7 * eq135_e1719_d_n4);
        let eq135_e1720_d_n5: f64 = (p.p7 * eq135_e1719_d_n5);
        let eq135_e1720_d_n6: f64 = (p.p7 * eq135_e1719_d_n6);
        let eq135_e1720_d_n7: f64 = (p.p7 * eq135_e1719_d_n7);
        let eq135_e1720_d_n8: f64 = (p.p7 * eq135_e1719_d_n8);
        let eq135_e1720_d_n9: f64 = (p.p7 * eq135_e1719_d_n9);
        let eq135_e1720_d_n10: f64 = (p.p7 * eq135_e1719_d_n10);
        let eq135_e1720_d_n11: f64 = (p.p7 * eq135_e1719_d_n11);
        let eq135_e1720_d_n12: f64 = (p.p7 * eq135_e1719_d_n12);
        let eq135_e1720_d_n13: f64 = (p.p7 * eq135_e1719_d_n13);
        let eq135_e1720_d_n14: f64 = (p.p7 * eq135_e1719_d_n14);
        let eq135_e1720_d_n15: f64 = (p.p7 * eq135_e1719_d_n15);
        let eq135_e1720_d_n16: f64 = (p.p7 * eq135_e1719_d_n16);
        let eq135_e1720_d_n17: f64 = (p.p7 * eq135_e1719_d_n17);
        let eq135_e1720_d_n18: f64 = (p.p7 * eq135_e1719_d_n18);
        let eq135_e1720_d_n19: f64 = (p.p7 * eq135_e1719_d_n19);
        let eq135_e1720_d_n20: f64 = (p.p7 * eq135_e1719_d_n20);
        let eq135_e1720_d_n21: f64 = (p.p7 * eq135_e1719_d_n21);
        let eq135_e1720_d_n22: f64 = (p.p7 * eq135_e1719_d_n22);
        (eq135_e1720, eq135_e1720_d_n0, eq135_e1720_d_n1, eq135_e1720_d_n2, eq135_e1720_d_n3, eq135_e1720_d_n4, eq135_e1720_d_n5, eq135_e1720_d_n6, eq135_e1720_d_n7, eq135_e1720_d_n8, eq135_e1720_d_n9, eq135_e1720_d_n10, eq135_e1720_d_n11, eq135_e1720_d_n12, eq135_e1720_d_n13, eq135_e1720_d_n14, eq135_e1720_d_n15, eq135_e1720_d_n16, eq135_e1720_d_n17, eq135_e1720_d_n18, eq135_e1720_d_n19, eq135_e1720_d_n20, eq135_e1720_d_n21, eq135_e1720_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1722;
        let eq135_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            multiplicity * (eq135_value),
            nodes,
            &eq135_node_derivatives,
            branches,
            &eq135_branch_derivatives,
            multiplicity,
        );
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq136_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 35, s.v[240]);
        let eq136_e1731_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq136_e1731_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq136_e1731_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq136_e1731_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq136_e1731_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq136_e1731_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq136_e1731_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq136_e1731_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq136_e1731_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq136_e1731_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq136_e1731_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq136_e1731_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq136_e1731_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq136_e1731_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq136_e1731_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq136_e1731_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq136_e1731_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq136_e1731_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq136_e1731_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq136_e1731_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq136_e1731_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq136_e1731_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq136_e1731_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq136_e1732: f64 = (p.p7 * eq136_e1731);
        let eq136_e1732_d_n0: f64 = (p.p7 * eq136_e1731_d_n0);
        let eq136_e1732_d_n1: f64 = (p.p7 * eq136_e1731_d_n1);
        let eq136_e1732_d_n2: f64 = (p.p7 * eq136_e1731_d_n2);
        let eq136_e1732_d_n3: f64 = (p.p7 * eq136_e1731_d_n3);
        let eq136_e1732_d_n4: f64 = (p.p7 * eq136_e1731_d_n4);
        let eq136_e1732_d_n5: f64 = (p.p7 * eq136_e1731_d_n5);
        let eq136_e1732_d_n6: f64 = (p.p7 * eq136_e1731_d_n6);
        let eq136_e1732_d_n7: f64 = (p.p7 * eq136_e1731_d_n7);
        let eq136_e1732_d_n8: f64 = (p.p7 * eq136_e1731_d_n8);
        let eq136_e1732_d_n9: f64 = (p.p7 * eq136_e1731_d_n9);
        let eq136_e1732_d_n10: f64 = (p.p7 * eq136_e1731_d_n10);
        let eq136_e1732_d_n11: f64 = (p.p7 * eq136_e1731_d_n11);
        let eq136_e1732_d_n12: f64 = (p.p7 * eq136_e1731_d_n12);
        let eq136_e1732_d_n13: f64 = (p.p7 * eq136_e1731_d_n13);
        let eq136_e1732_d_n14: f64 = (p.p7 * eq136_e1731_d_n14);
        let eq136_e1732_d_n15: f64 = (p.p7 * eq136_e1731_d_n15);
        let eq136_e1732_d_n16: f64 = (p.p7 * eq136_e1731_d_n16);
        let eq136_e1732_d_n17: f64 = (p.p7 * eq136_e1731_d_n17);
        let eq136_e1732_d_n18: f64 = (p.p7 * eq136_e1731_d_n18);
        let eq136_e1732_d_n19: f64 = (p.p7 * eq136_e1731_d_n19);
        let eq136_e1732_d_n20: f64 = (p.p7 * eq136_e1731_d_n20);
        let eq136_e1732_d_n21: f64 = (p.p7 * eq136_e1731_d_n21);
        let eq136_e1732_d_n22: f64 = (p.p7 * eq136_e1731_d_n22);
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
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_value: f64 = eq136_e1736;
        let eq136_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            multiplicity * (eq136_value),
            nodes,
            &eq136_node_derivatives,
            branches,
            &eq136_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22,) = {
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
        let eq137_e1744: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 36, eq137_e1743);
        let eq137_e1744_d_n0: f64 = (eq137_e1743_d_n0 * ddt_scale);
        let eq137_e1744_d_n1: f64 = (eq137_e1743_d_n1 * ddt_scale);
        let eq137_e1744_d_n2: f64 = (eq137_e1743_d_n2 * ddt_scale);
        let eq137_e1744_d_n3: f64 = (eq137_e1743_d_n3 * ddt_scale);
        let eq137_e1744_d_n4: f64 = (eq137_e1743_d_n4 * ddt_scale);
        let eq137_e1744_d_n5: f64 = (eq137_e1743_d_n5 * ddt_scale);
        let eq137_e1744_d_n6: f64 = (eq137_e1743_d_n6 * ddt_scale);
        let eq137_e1744_d_n7: f64 = (eq137_e1743_d_n7 * ddt_scale);
        let eq137_e1744_d_n8: f64 = (eq137_e1743_d_n8 * ddt_scale);
        let eq137_e1744_d_n9: f64 = (eq137_e1743_d_n9 * ddt_scale);
        let eq137_e1744_d_n10: f64 = (eq137_e1743_d_n10 * ddt_scale);
        let eq137_e1744_d_n11: f64 = (eq137_e1743_d_n11 * ddt_scale);
        let eq137_e1744_d_n12: f64 = (eq137_e1743_d_n12 * ddt_scale);
        let eq137_e1744_d_n13: f64 = (eq137_e1743_d_n13 * ddt_scale);
        let eq137_e1744_d_n14: f64 = (eq137_e1743_d_n14 * ddt_scale);
        let eq137_e1744_d_n15: f64 = (eq137_e1743_d_n15 * ddt_scale);
        let eq137_e1744_d_n16: f64 = (eq137_e1743_d_n16 * ddt_scale);
        let eq137_e1744_d_n17: f64 = (eq137_e1743_d_n17 * ddt_scale);
        let eq137_e1744_d_n18: f64 = (eq137_e1743_d_n18 * ddt_scale);
        let eq137_e1744_d_n19: f64 = (eq137_e1743_d_n19 * ddt_scale);
        let eq137_e1744_d_n20: f64 = (eq137_e1743_d_n20 * ddt_scale);
        let eq137_e1744_d_n21: f64 = (eq137_e1743_d_n21 * ddt_scale);
        let eq137_e1744_d_n22: f64 = (eq137_e1743_d_n22 * ddt_scale);
        let eq137_e1745: f64 = (p.p7 * eq137_e1744);
        let eq137_e1745_d_n0: f64 = (p.p7 * eq137_e1744_d_n0);
        let eq137_e1745_d_n1: f64 = (p.p7 * eq137_e1744_d_n1);
        let eq137_e1745_d_n2: f64 = (p.p7 * eq137_e1744_d_n2);
        let eq137_e1745_d_n3: f64 = (p.p7 * eq137_e1744_d_n3);
        let eq137_e1745_d_n4: f64 = (p.p7 * eq137_e1744_d_n4);
        let eq137_e1745_d_n5: f64 = (p.p7 * eq137_e1744_d_n5);
        let eq137_e1745_d_n6: f64 = (p.p7 * eq137_e1744_d_n6);
        let eq137_e1745_d_n7: f64 = (p.p7 * eq137_e1744_d_n7);
        let eq137_e1745_d_n8: f64 = (p.p7 * eq137_e1744_d_n8);
        let eq137_e1745_d_n9: f64 = (p.p7 * eq137_e1744_d_n9);
        let eq137_e1745_d_n10: f64 = (p.p7 * eq137_e1744_d_n10);
        let eq137_e1745_d_n11: f64 = (p.p7 * eq137_e1744_d_n11);
        let eq137_e1745_d_n12: f64 = (p.p7 * eq137_e1744_d_n12);
        let eq137_e1745_d_n13: f64 = (p.p7 * eq137_e1744_d_n13);
        let eq137_e1745_d_n14: f64 = (p.p7 * eq137_e1744_d_n14);
        let eq137_e1745_d_n15: f64 = (p.p7 * eq137_e1744_d_n15);
        let eq137_e1745_d_n16: f64 = (p.p7 * eq137_e1744_d_n16);
        let eq137_e1745_d_n17: f64 = (p.p7 * eq137_e1744_d_n17);
        let eq137_e1745_d_n18: f64 = (p.p7 * eq137_e1744_d_n18);
        let eq137_e1745_d_n19: f64 = (p.p7 * eq137_e1744_d_n19);
        let eq137_e1745_d_n20: f64 = (p.p7 * eq137_e1744_d_n20);
        let eq137_e1745_d_n21: f64 = (p.p7 * eq137_e1744_d_n21);
        let eq137_e1745_d_n22: f64 = (p.p7 * eq137_e1744_d_n22);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1747;
        let eq137_node_derivatives: [f64; 23] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[19]),
            multiplicity * (eq137_value),
            nodes,
            &eq137_node_derivatives,
            branches,
            &eq137_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq138_e1754: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 37, s.v[241]);
        let eq138_e1754_d_n0: f64 = (s.dn[241][0] * ddt_scale);
        let eq138_e1754_d_n1: f64 = (s.dn[241][1] * ddt_scale);
        let eq138_e1754_d_n2: f64 = (s.dn[241][2] * ddt_scale);
        let eq138_e1754_d_n3: f64 = (s.dn[241][3] * ddt_scale);
        let eq138_e1754_d_n4: f64 = (s.dn[241][4] * ddt_scale);
        let eq138_e1754_d_n5: f64 = (s.dn[241][5] * ddt_scale);
        let eq138_e1754_d_n6: f64 = (s.dn[241][6] * ddt_scale);
        let eq138_e1754_d_n7: f64 = (s.dn[241][7] * ddt_scale);
        let eq138_e1754_d_n8: f64 = (s.dn[241][8] * ddt_scale);
        let eq138_e1754_d_n9: f64 = (s.dn[241][9] * ddt_scale);
        let eq138_e1754_d_n10: f64 = (s.dn[241][10] * ddt_scale);
        let eq138_e1754_d_n11: f64 = (s.dn[241][11] * ddt_scale);
        let eq138_e1754_d_n12: f64 = (s.dn[241][12] * ddt_scale);
        let eq138_e1754_d_n13: f64 = (s.dn[241][13] * ddt_scale);
        let eq138_e1754_d_n14: f64 = (s.dn[241][14] * ddt_scale);
        let eq138_e1754_d_n15: f64 = (s.dn[241][15] * ddt_scale);
        let eq138_e1754_d_n16: f64 = (s.dn[241][16] * ddt_scale);
        let eq138_e1754_d_n17: f64 = (s.dn[241][17] * ddt_scale);
        let eq138_e1754_d_n18: f64 = (s.dn[241][18] * ddt_scale);
        let eq138_e1754_d_n19: f64 = (s.dn[241][19] * ddt_scale);
        let eq138_e1754_d_n20: f64 = (s.dn[241][20] * ddt_scale);
        let eq138_e1754_d_n21: f64 = (s.dn[241][21] * ddt_scale);
        let eq138_e1754_d_n22: f64 = (s.dn[241][22] * ddt_scale);
        let eq138_e1755: f64 = (p.p7 * eq138_e1754);
        let eq138_e1755_d_n0: f64 = (p.p7 * eq138_e1754_d_n0);
        let eq138_e1755_d_n1: f64 = (p.p7 * eq138_e1754_d_n1);
        let eq138_e1755_d_n2: f64 = (p.p7 * eq138_e1754_d_n2);
        let eq138_e1755_d_n3: f64 = (p.p7 * eq138_e1754_d_n3);
        let eq138_e1755_d_n4: f64 = (p.p7 * eq138_e1754_d_n4);
        let eq138_e1755_d_n5: f64 = (p.p7 * eq138_e1754_d_n5);
        let eq138_e1755_d_n6: f64 = (p.p7 * eq138_e1754_d_n6);
        let eq138_e1755_d_n7: f64 = (p.p7 * eq138_e1754_d_n7);
        let eq138_e1755_d_n8: f64 = (p.p7 * eq138_e1754_d_n8);
        let eq138_e1755_d_n9: f64 = (p.p7 * eq138_e1754_d_n9);
        let eq138_e1755_d_n10: f64 = (p.p7 * eq138_e1754_d_n10);
        let eq138_e1755_d_n11: f64 = (p.p7 * eq138_e1754_d_n11);
        let eq138_e1755_d_n12: f64 = (p.p7 * eq138_e1754_d_n12);
        let eq138_e1755_d_n13: f64 = (p.p7 * eq138_e1754_d_n13);
        let eq138_e1755_d_n14: f64 = (p.p7 * eq138_e1754_d_n14);
        let eq138_e1755_d_n15: f64 = (p.p7 * eq138_e1754_d_n15);
        let eq138_e1755_d_n16: f64 = (p.p7 * eq138_e1754_d_n16);
        let eq138_e1755_d_n17: f64 = (p.p7 * eq138_e1754_d_n17);
        let eq138_e1755_d_n18: f64 = (p.p7 * eq138_e1754_d_n18);
        let eq138_e1755_d_n19: f64 = (p.p7 * eq138_e1754_d_n19);
        let eq138_e1755_d_n20: f64 = (p.p7 * eq138_e1754_d_n20);
        let eq138_e1755_d_n21: f64 = (p.p7 * eq138_e1754_d_n21);
        let eq138_e1755_d_n22: f64 = (p.p7 * eq138_e1754_d_n22);
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_value: f64 = eq138_e1757;
        let eq138_node_derivatives: [f64; 23] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            multiplicity * (eq138_value),
            nodes,
            &eq138_node_derivatives,
            branches,
            &eq138_branch_derivatives,
            multiplicity,
        );
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq139_e1766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 38, s.v[240]);
        let eq139_e1766_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq139_e1766_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq139_e1766_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq139_e1766_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq139_e1766_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq139_e1766_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq139_e1766_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq139_e1766_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq139_e1766_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq139_e1766_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq139_e1766_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq139_e1766_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq139_e1766_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq139_e1766_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq139_e1766_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq139_e1766_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq139_e1766_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq139_e1766_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq139_e1766_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq139_e1766_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq139_e1766_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq139_e1766_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq139_e1766_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq139_e1767: f64 = (p.p7 * eq139_e1766);
        let eq139_e1767_d_n0: f64 = (p.p7 * eq139_e1766_d_n0);
        let eq139_e1767_d_n1: f64 = (p.p7 * eq139_e1766_d_n1);
        let eq139_e1767_d_n2: f64 = (p.p7 * eq139_e1766_d_n2);
        let eq139_e1767_d_n3: f64 = (p.p7 * eq139_e1766_d_n3);
        let eq139_e1767_d_n4: f64 = (p.p7 * eq139_e1766_d_n4);
        let eq139_e1767_d_n5: f64 = (p.p7 * eq139_e1766_d_n5);
        let eq139_e1767_d_n6: f64 = (p.p7 * eq139_e1766_d_n6);
        let eq139_e1767_d_n7: f64 = (p.p7 * eq139_e1766_d_n7);
        let eq139_e1767_d_n8: f64 = (p.p7 * eq139_e1766_d_n8);
        let eq139_e1767_d_n9: f64 = (p.p7 * eq139_e1766_d_n9);
        let eq139_e1767_d_n10: f64 = (p.p7 * eq139_e1766_d_n10);
        let eq139_e1767_d_n11: f64 = (p.p7 * eq139_e1766_d_n11);
        let eq139_e1767_d_n12: f64 = (p.p7 * eq139_e1766_d_n12);
        let eq139_e1767_d_n13: f64 = (p.p7 * eq139_e1766_d_n13);
        let eq139_e1767_d_n14: f64 = (p.p7 * eq139_e1766_d_n14);
        let eq139_e1767_d_n15: f64 = (p.p7 * eq139_e1766_d_n15);
        let eq139_e1767_d_n16: f64 = (p.p7 * eq139_e1766_d_n16);
        let eq139_e1767_d_n17: f64 = (p.p7 * eq139_e1766_d_n17);
        let eq139_e1767_d_n18: f64 = (p.p7 * eq139_e1766_d_n18);
        let eq139_e1767_d_n19: f64 = (p.p7 * eq139_e1766_d_n19);
        let eq139_e1767_d_n20: f64 = (p.p7 * eq139_e1766_d_n20);
        let eq139_e1767_d_n21: f64 = (p.p7 * eq139_e1766_d_n21);
        let eq139_e1767_d_n22: f64 = (p.p7 * eq139_e1766_d_n22);
        (eq139_e1767, eq139_e1767_d_n0, eq139_e1767_d_n1, eq139_e1767_d_n2, eq139_e1767_d_n3, eq139_e1767_d_n4, eq139_e1767_d_n5, eq139_e1767_d_n6, eq139_e1767_d_n7, eq139_e1767_d_n8, eq139_e1767_d_n9, eq139_e1767_d_n10, eq139_e1767_d_n11, eq139_e1767_d_n12, eq139_e1767_d_n13, eq139_e1767_d_n14, eq139_e1767_d_n15, eq139_e1767_d_n16, eq139_e1767_d_n17, eq139_e1767_d_n18, eq139_e1767_d_n19, eq139_e1767_d_n20, eq139_e1767_d_n21, eq139_e1767_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_value: f64 = eq139_e1769;
        let eq139_node_derivatives: [f64; 23] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq139_value),
            nodes,
            &eq139_node_derivatives,
            branches,
            &eq139_branch_derivatives,
            multiplicity,
        );
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq140_e1778: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 39, s.v[240]);
        let eq140_e1778_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq140_e1778_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq140_e1778_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq140_e1778_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq140_e1778_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq140_e1778_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq140_e1778_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq140_e1778_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq140_e1778_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq140_e1778_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq140_e1778_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq140_e1778_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq140_e1778_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq140_e1778_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq140_e1778_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq140_e1778_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq140_e1778_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq140_e1778_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq140_e1778_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq140_e1778_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq140_e1778_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq140_e1778_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq140_e1778_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq140_e1779: f64 = (p.p7 * eq140_e1778);
        let eq140_e1779_d_n0: f64 = (p.p7 * eq140_e1778_d_n0);
        let eq140_e1779_d_n1: f64 = (p.p7 * eq140_e1778_d_n1);
        let eq140_e1779_d_n2: f64 = (p.p7 * eq140_e1778_d_n2);
        let eq140_e1779_d_n3: f64 = (p.p7 * eq140_e1778_d_n3);
        let eq140_e1779_d_n4: f64 = (p.p7 * eq140_e1778_d_n4);
        let eq140_e1779_d_n5: f64 = (p.p7 * eq140_e1778_d_n5);
        let eq140_e1779_d_n6: f64 = (p.p7 * eq140_e1778_d_n6);
        let eq140_e1779_d_n7: f64 = (p.p7 * eq140_e1778_d_n7);
        let eq140_e1779_d_n8: f64 = (p.p7 * eq140_e1778_d_n8);
        let eq140_e1779_d_n9: f64 = (p.p7 * eq140_e1778_d_n9);
        let eq140_e1779_d_n10: f64 = (p.p7 * eq140_e1778_d_n10);
        let eq140_e1779_d_n11: f64 = (p.p7 * eq140_e1778_d_n11);
        let eq140_e1779_d_n12: f64 = (p.p7 * eq140_e1778_d_n12);
        let eq140_e1779_d_n13: f64 = (p.p7 * eq140_e1778_d_n13);
        let eq140_e1779_d_n14: f64 = (p.p7 * eq140_e1778_d_n14);
        let eq140_e1779_d_n15: f64 = (p.p7 * eq140_e1778_d_n15);
        let eq140_e1779_d_n16: f64 = (p.p7 * eq140_e1778_d_n16);
        let eq140_e1779_d_n17: f64 = (p.p7 * eq140_e1778_d_n17);
        let eq140_e1779_d_n18: f64 = (p.p7 * eq140_e1778_d_n18);
        let eq140_e1779_d_n19: f64 = (p.p7 * eq140_e1778_d_n19);
        let eq140_e1779_d_n20: f64 = (p.p7 * eq140_e1778_d_n20);
        let eq140_e1779_d_n21: f64 = (p.p7 * eq140_e1778_d_n21);
        let eq140_e1779_d_n22: f64 = (p.p7 * eq140_e1778_d_n22);
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
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_value: f64 = eq140_e1783;
        let eq140_node_derivatives: [f64; 23] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq140_value),
            nodes,
            &eq140_node_derivatives,
            branches,
            &eq140_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_13(
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
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq141_e1793: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 40, s.v[240]);
        let eq141_e1793_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq141_e1793_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq141_e1793_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq141_e1793_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq141_e1793_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq141_e1793_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq141_e1793_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq141_e1793_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq141_e1793_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq141_e1793_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq141_e1793_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq141_e1793_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq141_e1793_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq141_e1793_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq141_e1793_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq141_e1793_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq141_e1793_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq141_e1793_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq141_e1793_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq141_e1793_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq141_e1793_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq141_e1793_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq141_e1793_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq141_e1794: f64 = (p.p7 * eq141_e1793);
        let eq141_e1794_d_n0: f64 = (p.p7 * eq141_e1793_d_n0);
        let eq141_e1794_d_n1: f64 = (p.p7 * eq141_e1793_d_n1);
        let eq141_e1794_d_n2: f64 = (p.p7 * eq141_e1793_d_n2);
        let eq141_e1794_d_n3: f64 = (p.p7 * eq141_e1793_d_n3);
        let eq141_e1794_d_n4: f64 = (p.p7 * eq141_e1793_d_n4);
        let eq141_e1794_d_n5: f64 = (p.p7 * eq141_e1793_d_n5);
        let eq141_e1794_d_n6: f64 = (p.p7 * eq141_e1793_d_n6);
        let eq141_e1794_d_n7: f64 = (p.p7 * eq141_e1793_d_n7);
        let eq141_e1794_d_n8: f64 = (p.p7 * eq141_e1793_d_n8);
        let eq141_e1794_d_n9: f64 = (p.p7 * eq141_e1793_d_n9);
        let eq141_e1794_d_n10: f64 = (p.p7 * eq141_e1793_d_n10);
        let eq141_e1794_d_n11: f64 = (p.p7 * eq141_e1793_d_n11);
        let eq141_e1794_d_n12: f64 = (p.p7 * eq141_e1793_d_n12);
        let eq141_e1794_d_n13: f64 = (p.p7 * eq141_e1793_d_n13);
        let eq141_e1794_d_n14: f64 = (p.p7 * eq141_e1793_d_n14);
        let eq141_e1794_d_n15: f64 = (p.p7 * eq141_e1793_d_n15);
        let eq141_e1794_d_n16: f64 = (p.p7 * eq141_e1793_d_n16);
        let eq141_e1794_d_n17: f64 = (p.p7 * eq141_e1793_d_n17);
        let eq141_e1794_d_n18: f64 = (p.p7 * eq141_e1793_d_n18);
        let eq141_e1794_d_n19: f64 = (p.p7 * eq141_e1793_d_n19);
        let eq141_e1794_d_n20: f64 = (p.p7 * eq141_e1793_d_n20);
        let eq141_e1794_d_n21: f64 = (p.p7 * eq141_e1793_d_n21);
        let eq141_e1794_d_n22: f64 = (p.p7 * eq141_e1793_d_n22);
        (eq141_e1794, eq141_e1794_d_n0, eq141_e1794_d_n1, eq141_e1794_d_n2, eq141_e1794_d_n3, eq141_e1794_d_n4, eq141_e1794_d_n5, eq141_e1794_d_n6, eq141_e1794_d_n7, eq141_e1794_d_n8, eq141_e1794_d_n9, eq141_e1794_d_n10, eq141_e1794_d_n11, eq141_e1794_d_n12, eq141_e1794_d_n13, eq141_e1794_d_n14, eq141_e1794_d_n15, eq141_e1794_d_n16, eq141_e1794_d_n17, eq141_e1794_d_n18, eq141_e1794_d_n19, eq141_e1794_d_n20, eq141_e1794_d_n21, eq141_e1794_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1796;
        let eq141_node_derivatives: [f64; 23] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq141_value),
            nodes,
            &eq141_node_derivatives,
            branches,
            &eq141_branch_derivatives,
            multiplicity,
        );
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq142_e1806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 41, s.v[240]);
        let eq142_e1806_d_n0: f64 = (s.dn[240][0] * ddt_scale);
        let eq142_e1806_d_n1: f64 = (s.dn[240][1] * ddt_scale);
        let eq142_e1806_d_n2: f64 = (s.dn[240][2] * ddt_scale);
        let eq142_e1806_d_n3: f64 = (s.dn[240][3] * ddt_scale);
        let eq142_e1806_d_n4: f64 = (s.dn[240][4] * ddt_scale);
        let eq142_e1806_d_n5: f64 = (s.dn[240][5] * ddt_scale);
        let eq142_e1806_d_n6: f64 = (s.dn[240][6] * ddt_scale);
        let eq142_e1806_d_n7: f64 = (s.dn[240][7] * ddt_scale);
        let eq142_e1806_d_n8: f64 = (s.dn[240][8] * ddt_scale);
        let eq142_e1806_d_n9: f64 = (s.dn[240][9] * ddt_scale);
        let eq142_e1806_d_n10: f64 = (s.dn[240][10] * ddt_scale);
        let eq142_e1806_d_n11: f64 = (s.dn[240][11] * ddt_scale);
        let eq142_e1806_d_n12: f64 = (s.dn[240][12] * ddt_scale);
        let eq142_e1806_d_n13: f64 = (s.dn[240][13] * ddt_scale);
        let eq142_e1806_d_n14: f64 = (s.dn[240][14] * ddt_scale);
        let eq142_e1806_d_n15: f64 = (s.dn[240][15] * ddt_scale);
        let eq142_e1806_d_n16: f64 = (s.dn[240][16] * ddt_scale);
        let eq142_e1806_d_n17: f64 = (s.dn[240][17] * ddt_scale);
        let eq142_e1806_d_n18: f64 = (s.dn[240][18] * ddt_scale);
        let eq142_e1806_d_n19: f64 = (s.dn[240][19] * ddt_scale);
        let eq142_e1806_d_n20: f64 = (s.dn[240][20] * ddt_scale);
        let eq142_e1806_d_n21: f64 = (s.dn[240][21] * ddt_scale);
        let eq142_e1806_d_n22: f64 = (s.dn[240][22] * ddt_scale);
        let eq142_e1807: f64 = (p.p7 * eq142_e1806);
        let eq142_e1807_d_n0: f64 = (p.p7 * eq142_e1806_d_n0);
        let eq142_e1807_d_n1: f64 = (p.p7 * eq142_e1806_d_n1);
        let eq142_e1807_d_n2: f64 = (p.p7 * eq142_e1806_d_n2);
        let eq142_e1807_d_n3: f64 = (p.p7 * eq142_e1806_d_n3);
        let eq142_e1807_d_n4: f64 = (p.p7 * eq142_e1806_d_n4);
        let eq142_e1807_d_n5: f64 = (p.p7 * eq142_e1806_d_n5);
        let eq142_e1807_d_n6: f64 = (p.p7 * eq142_e1806_d_n6);
        let eq142_e1807_d_n7: f64 = (p.p7 * eq142_e1806_d_n7);
        let eq142_e1807_d_n8: f64 = (p.p7 * eq142_e1806_d_n8);
        let eq142_e1807_d_n9: f64 = (p.p7 * eq142_e1806_d_n9);
        let eq142_e1807_d_n10: f64 = (p.p7 * eq142_e1806_d_n10);
        let eq142_e1807_d_n11: f64 = (p.p7 * eq142_e1806_d_n11);
        let eq142_e1807_d_n12: f64 = (p.p7 * eq142_e1806_d_n12);
        let eq142_e1807_d_n13: f64 = (p.p7 * eq142_e1806_d_n13);
        let eq142_e1807_d_n14: f64 = (p.p7 * eq142_e1806_d_n14);
        let eq142_e1807_d_n15: f64 = (p.p7 * eq142_e1806_d_n15);
        let eq142_e1807_d_n16: f64 = (p.p7 * eq142_e1806_d_n16);
        let eq142_e1807_d_n17: f64 = (p.p7 * eq142_e1806_d_n17);
        let eq142_e1807_d_n18: f64 = (p.p7 * eq142_e1806_d_n18);
        let eq142_e1807_d_n19: f64 = (p.p7 * eq142_e1806_d_n19);
        let eq142_e1807_d_n20: f64 = (p.p7 * eq142_e1806_d_n20);
        let eq142_e1807_d_n21: f64 = (p.p7 * eq142_e1806_d_n21);
        let eq142_e1807_d_n22: f64 = (p.p7 * eq142_e1806_d_n22);
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
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1811;
        let eq142_node_derivatives: [f64; 23] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq142_value),
            nodes,
            &eq142_node_derivatives,
            branches,
            &eq142_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22,) = {
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
        let eq143_e1820: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 42, eq143_e1819);
        let eq143_e1820_d_n0: f64 = (eq143_e1819_d_n0 * ddt_scale);
        let eq143_e1820_d_n1: f64 = (eq143_e1819_d_n1 * ddt_scale);
        let eq143_e1820_d_n2: f64 = (eq143_e1819_d_n2 * ddt_scale);
        let eq143_e1820_d_n3: f64 = (eq143_e1819_d_n3 * ddt_scale);
        let eq143_e1820_d_n4: f64 = (eq143_e1819_d_n4 * ddt_scale);
        let eq143_e1820_d_n5: f64 = (eq143_e1819_d_n5 * ddt_scale);
        let eq143_e1820_d_n6: f64 = (eq143_e1819_d_n6 * ddt_scale);
        let eq143_e1820_d_n7: f64 = (eq143_e1819_d_n7 * ddt_scale);
        let eq143_e1820_d_n8: f64 = (eq143_e1819_d_n8 * ddt_scale);
        let eq143_e1820_d_n9: f64 = (eq143_e1819_d_n9 * ddt_scale);
        let eq143_e1820_d_n10: f64 = (eq143_e1819_d_n10 * ddt_scale);
        let eq143_e1820_d_n11: f64 = (eq143_e1819_d_n11 * ddt_scale);
        let eq143_e1820_d_n12: f64 = (eq143_e1819_d_n12 * ddt_scale);
        let eq143_e1820_d_n13: f64 = (eq143_e1819_d_n13 * ddt_scale);
        let eq143_e1820_d_n14: f64 = (eq143_e1819_d_n14 * ddt_scale);
        let eq143_e1820_d_n15: f64 = (eq143_e1819_d_n15 * ddt_scale);
        let eq143_e1820_d_n16: f64 = (eq143_e1819_d_n16 * ddt_scale);
        let eq143_e1820_d_n17: f64 = (eq143_e1819_d_n17 * ddt_scale);
        let eq143_e1820_d_n18: f64 = (eq143_e1819_d_n18 * ddt_scale);
        let eq143_e1820_d_n19: f64 = (eq143_e1819_d_n19 * ddt_scale);
        let eq143_e1820_d_n20: f64 = (eq143_e1819_d_n20 * ddt_scale);
        let eq143_e1820_d_n21: f64 = (eq143_e1819_d_n21 * ddt_scale);
        let eq143_e1820_d_n22: f64 = (eq143_e1819_d_n22 * ddt_scale);
        let eq143_e1821: f64 = (p.p7 * eq143_e1820);
        let eq143_e1821_d_n0: f64 = (p.p7 * eq143_e1820_d_n0);
        let eq143_e1821_d_n1: f64 = (p.p7 * eq143_e1820_d_n1);
        let eq143_e1821_d_n2: f64 = (p.p7 * eq143_e1820_d_n2);
        let eq143_e1821_d_n3: f64 = (p.p7 * eq143_e1820_d_n3);
        let eq143_e1821_d_n4: f64 = (p.p7 * eq143_e1820_d_n4);
        let eq143_e1821_d_n5: f64 = (p.p7 * eq143_e1820_d_n5);
        let eq143_e1821_d_n6: f64 = (p.p7 * eq143_e1820_d_n6);
        let eq143_e1821_d_n7: f64 = (p.p7 * eq143_e1820_d_n7);
        let eq143_e1821_d_n8: f64 = (p.p7 * eq143_e1820_d_n8);
        let eq143_e1821_d_n9: f64 = (p.p7 * eq143_e1820_d_n9);
        let eq143_e1821_d_n10: f64 = (p.p7 * eq143_e1820_d_n10);
        let eq143_e1821_d_n11: f64 = (p.p7 * eq143_e1820_d_n11);
        let eq143_e1821_d_n12: f64 = (p.p7 * eq143_e1820_d_n12);
        let eq143_e1821_d_n13: f64 = (p.p7 * eq143_e1820_d_n13);
        let eq143_e1821_d_n14: f64 = (p.p7 * eq143_e1820_d_n14);
        let eq143_e1821_d_n15: f64 = (p.p7 * eq143_e1820_d_n15);
        let eq143_e1821_d_n16: f64 = (p.p7 * eq143_e1820_d_n16);
        let eq143_e1821_d_n17: f64 = (p.p7 * eq143_e1820_d_n17);
        let eq143_e1821_d_n18: f64 = (p.p7 * eq143_e1820_d_n18);
        let eq143_e1821_d_n19: f64 = (p.p7 * eq143_e1820_d_n19);
        let eq143_e1821_d_n20: f64 = (p.p7 * eq143_e1820_d_n20);
        let eq143_e1821_d_n21: f64 = (p.p7 * eq143_e1820_d_n21);
        let eq143_e1821_d_n22: f64 = (p.p7 * eq143_e1820_d_n22);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1823;
        let eq143_node_derivatives: [f64; 23] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            multiplicity * (eq143_value),
            nodes,
            &eq143_node_derivatives,
            branches,
            &eq143_branch_derivatives,
            multiplicity,
        );
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22,) = {
    if (s.b[580] && s.b[581]) {
        let eq144_e1829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 43, s.v[253]);
        let eq144_e1829_d_n0: f64 = (s.dn[253][0] * ddt_scale);
        let eq144_e1829_d_n1: f64 = (s.dn[253][1] * ddt_scale);
        let eq144_e1829_d_n2: f64 = (s.dn[253][2] * ddt_scale);
        let eq144_e1829_d_n3: f64 = (s.dn[253][3] * ddt_scale);
        let eq144_e1829_d_n4: f64 = (s.dn[253][4] * ddt_scale);
        let eq144_e1829_d_n5: f64 = (s.dn[253][5] * ddt_scale);
        let eq144_e1829_d_n6: f64 = (s.dn[253][6] * ddt_scale);
        let eq144_e1829_d_n7: f64 = (s.dn[253][7] * ddt_scale);
        let eq144_e1829_d_n8: f64 = (s.dn[253][8] * ddt_scale);
        let eq144_e1829_d_n9: f64 = (s.dn[253][9] * ddt_scale);
        let eq144_e1829_d_n10: f64 = (s.dn[253][10] * ddt_scale);
        let eq144_e1829_d_n11: f64 = (s.dn[253][11] * ddt_scale);
        let eq144_e1829_d_n12: f64 = (s.dn[253][12] * ddt_scale);
        let eq144_e1829_d_n13: f64 = (s.dn[253][13] * ddt_scale);
        let eq144_e1829_d_n14: f64 = (s.dn[253][14] * ddt_scale);
        let eq144_e1829_d_n15: f64 = (s.dn[253][15] * ddt_scale);
        let eq144_e1829_d_n16: f64 = (s.dn[253][16] * ddt_scale);
        let eq144_e1829_d_n17: f64 = (s.dn[253][17] * ddt_scale);
        let eq144_e1829_d_n18: f64 = (s.dn[253][18] * ddt_scale);
        let eq144_e1829_d_n19: f64 = (s.dn[253][19] * ddt_scale);
        let eq144_e1829_d_n20: f64 = (s.dn[253][20] * ddt_scale);
        let eq144_e1829_d_n21: f64 = (s.dn[253][21] * ddt_scale);
        let eq144_e1829_d_n22: f64 = (s.dn[253][22] * ddt_scale);
        let eq144_e1830: f64 = (p.p7 * eq144_e1829);
        let eq144_e1830_d_n0: f64 = (p.p7 * eq144_e1829_d_n0);
        let eq144_e1830_d_n1: f64 = (p.p7 * eq144_e1829_d_n1);
        let eq144_e1830_d_n2: f64 = (p.p7 * eq144_e1829_d_n2);
        let eq144_e1830_d_n3: f64 = (p.p7 * eq144_e1829_d_n3);
        let eq144_e1830_d_n4: f64 = (p.p7 * eq144_e1829_d_n4);
        let eq144_e1830_d_n5: f64 = (p.p7 * eq144_e1829_d_n5);
        let eq144_e1830_d_n6: f64 = (p.p7 * eq144_e1829_d_n6);
        let eq144_e1830_d_n7: f64 = (p.p7 * eq144_e1829_d_n7);
        let eq144_e1830_d_n8: f64 = (p.p7 * eq144_e1829_d_n8);
        let eq144_e1830_d_n9: f64 = (p.p7 * eq144_e1829_d_n9);
        let eq144_e1830_d_n10: f64 = (p.p7 * eq144_e1829_d_n10);
        let eq144_e1830_d_n11: f64 = (p.p7 * eq144_e1829_d_n11);
        let eq144_e1830_d_n12: f64 = (p.p7 * eq144_e1829_d_n12);
        let eq144_e1830_d_n13: f64 = (p.p7 * eq144_e1829_d_n13);
        let eq144_e1830_d_n14: f64 = (p.p7 * eq144_e1829_d_n14);
        let eq144_e1830_d_n15: f64 = (p.p7 * eq144_e1829_d_n15);
        let eq144_e1830_d_n16: f64 = (p.p7 * eq144_e1829_d_n16);
        let eq144_e1830_d_n17: f64 = (p.p7 * eq144_e1829_d_n17);
        let eq144_e1830_d_n18: f64 = (p.p7 * eq144_e1829_d_n18);
        let eq144_e1830_d_n19: f64 = (p.p7 * eq144_e1829_d_n19);
        let eq144_e1830_d_n20: f64 = (p.p7 * eq144_e1829_d_n20);
        let eq144_e1830_d_n21: f64 = (p.p7 * eq144_e1829_d_n21);
        let eq144_e1830_d_n22: f64 = (p.p7 * eq144_e1829_d_n22);
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1832;
        let eq144_node_derivatives: [f64; 23] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            Some(nodes[15]),
            multiplicity * (eq144_value),
            nodes,
            &eq144_node_derivatives,
            branches,
            &eq144_branch_derivatives,
            multiplicity,
        );
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq145_e1840: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 44, s.v[252]);
        let eq145_e1840_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq145_e1840_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq145_e1840_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq145_e1840_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq145_e1840_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq145_e1840_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq145_e1840_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq145_e1840_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq145_e1840_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq145_e1840_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq145_e1840_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq145_e1840_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq145_e1840_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq145_e1840_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq145_e1840_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq145_e1840_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq145_e1840_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq145_e1840_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq145_e1840_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq145_e1840_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq145_e1840_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq145_e1840_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq145_e1840_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq145_e1841: f64 = (p.p7 * eq145_e1840);
        let eq145_e1841_d_n0: f64 = (p.p7 * eq145_e1840_d_n0);
        let eq145_e1841_d_n1: f64 = (p.p7 * eq145_e1840_d_n1);
        let eq145_e1841_d_n2: f64 = (p.p7 * eq145_e1840_d_n2);
        let eq145_e1841_d_n3: f64 = (p.p7 * eq145_e1840_d_n3);
        let eq145_e1841_d_n4: f64 = (p.p7 * eq145_e1840_d_n4);
        let eq145_e1841_d_n5: f64 = (p.p7 * eq145_e1840_d_n5);
        let eq145_e1841_d_n6: f64 = (p.p7 * eq145_e1840_d_n6);
        let eq145_e1841_d_n7: f64 = (p.p7 * eq145_e1840_d_n7);
        let eq145_e1841_d_n8: f64 = (p.p7 * eq145_e1840_d_n8);
        let eq145_e1841_d_n9: f64 = (p.p7 * eq145_e1840_d_n9);
        let eq145_e1841_d_n10: f64 = (p.p7 * eq145_e1840_d_n10);
        let eq145_e1841_d_n11: f64 = (p.p7 * eq145_e1840_d_n11);
        let eq145_e1841_d_n12: f64 = (p.p7 * eq145_e1840_d_n12);
        let eq145_e1841_d_n13: f64 = (p.p7 * eq145_e1840_d_n13);
        let eq145_e1841_d_n14: f64 = (p.p7 * eq145_e1840_d_n14);
        let eq145_e1841_d_n15: f64 = (p.p7 * eq145_e1840_d_n15);
        let eq145_e1841_d_n16: f64 = (p.p7 * eq145_e1840_d_n16);
        let eq145_e1841_d_n17: f64 = (p.p7 * eq145_e1840_d_n17);
        let eq145_e1841_d_n18: f64 = (p.p7 * eq145_e1840_d_n18);
        let eq145_e1841_d_n19: f64 = (p.p7 * eq145_e1840_d_n19);
        let eq145_e1841_d_n20: f64 = (p.p7 * eq145_e1840_d_n20);
        let eq145_e1841_d_n21: f64 = (p.p7 * eq145_e1840_d_n21);
        let eq145_e1841_d_n22: f64 = (p.p7 * eq145_e1840_d_n22);
        (eq145_e1841, eq145_e1841_d_n0, eq145_e1841_d_n1, eq145_e1841_d_n2, eq145_e1841_d_n3, eq145_e1841_d_n4, eq145_e1841_d_n5, eq145_e1841_d_n6, eq145_e1841_d_n7, eq145_e1841_d_n8, eq145_e1841_d_n9, eq145_e1841_d_n10, eq145_e1841_d_n11, eq145_e1841_d_n12, eq145_e1841_d_n13, eq145_e1841_d_n14, eq145_e1841_d_n15, eq145_e1841_d_n16, eq145_e1841_d_n17, eq145_e1841_d_n18, eq145_e1841_d_n19, eq145_e1841_d_n20, eq145_e1841_d_n21, eq145_e1841_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_value: f64 = eq145_e1843;
        let eq145_node_derivatives: [f64; 23] = [eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22];
        let eq145_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            multiplicity * (eq145_value),
            nodes,
            &eq145_node_derivatives,
            branches,
            &eq145_branch_derivatives,
            multiplicity,
        );
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 45, s.v[252]);
        let eq146_e1853_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq146_e1853_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq146_e1853_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq146_e1853_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq146_e1853_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq146_e1853_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq146_e1853_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq146_e1853_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq146_e1853_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq146_e1853_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq146_e1853_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq146_e1853_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq146_e1853_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq146_e1853_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq146_e1853_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq146_e1853_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq146_e1853_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq146_e1853_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq146_e1853_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq146_e1853_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq146_e1853_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq146_e1853_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq146_e1853_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq146_e1854: f64 = (eq146_e1851 * eq146_e1853);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * eq146_e1853_d_n0);
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * eq146_e1853_d_n1);
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * eq146_e1853_d_n2);
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * eq146_e1853_d_n3);
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * eq146_e1853_d_n4);
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * eq146_e1853_d_n5);
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * eq146_e1853_d_n6);
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * eq146_e1853_d_n7);
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * eq146_e1853_d_n8);
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * eq146_e1853_d_n9);
        let eq146_e1854_d_n10: f64 = (eq146_e1851 * eq146_e1853_d_n10);
        let eq146_e1854_d_n11: f64 = (eq146_e1851 * eq146_e1853_d_n11);
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * eq146_e1853_d_n12);
        let eq146_e1854_d_n13: f64 = (eq146_e1851 * eq146_e1853_d_n13);
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * eq146_e1853_d_n14);
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * eq146_e1853_d_n15);
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * eq146_e1853_d_n16);
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * eq146_e1853_d_n17);
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * eq146_e1853_d_n18);
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * eq146_e1853_d_n19);
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * eq146_e1853_d_n20);
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * eq146_e1853_d_n21);
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * eq146_e1853_d_n22);
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n10, eq146_e1854_d_n11, eq146_e1854_d_n12, eq146_e1854_d_n13, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_value: f64 = eq146_e1856;
        let eq146_node_derivatives: [f64; 23] = [eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22];
        let eq146_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            multiplicity * (eq146_value),
            nodes,
            &eq146_node_derivatives,
            branches,
            &eq146_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_14(
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
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq147_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 46, s.v[252]);
        let eq147_e1865_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq147_e1865_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq147_e1865_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq147_e1865_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq147_e1865_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq147_e1865_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq147_e1865_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq147_e1865_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq147_e1865_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq147_e1865_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq147_e1865_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq147_e1865_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq147_e1865_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq147_e1865_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq147_e1865_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq147_e1865_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq147_e1865_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq147_e1865_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq147_e1865_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq147_e1865_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq147_e1865_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq147_e1865_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq147_e1865_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq147_e1866: f64 = (p.p7 * eq147_e1865);
        let eq147_e1866_d_n0: f64 = (p.p7 * eq147_e1865_d_n0);
        let eq147_e1866_d_n1: f64 = (p.p7 * eq147_e1865_d_n1);
        let eq147_e1866_d_n2: f64 = (p.p7 * eq147_e1865_d_n2);
        let eq147_e1866_d_n3: f64 = (p.p7 * eq147_e1865_d_n3);
        let eq147_e1866_d_n4: f64 = (p.p7 * eq147_e1865_d_n4);
        let eq147_e1866_d_n5: f64 = (p.p7 * eq147_e1865_d_n5);
        let eq147_e1866_d_n6: f64 = (p.p7 * eq147_e1865_d_n6);
        let eq147_e1866_d_n7: f64 = (p.p7 * eq147_e1865_d_n7);
        let eq147_e1866_d_n8: f64 = (p.p7 * eq147_e1865_d_n8);
        let eq147_e1866_d_n9: f64 = (p.p7 * eq147_e1865_d_n9);
        let eq147_e1866_d_n10: f64 = (p.p7 * eq147_e1865_d_n10);
        let eq147_e1866_d_n11: f64 = (p.p7 * eq147_e1865_d_n11);
        let eq147_e1866_d_n12: f64 = (p.p7 * eq147_e1865_d_n12);
        let eq147_e1866_d_n13: f64 = (p.p7 * eq147_e1865_d_n13);
        let eq147_e1866_d_n14: f64 = (p.p7 * eq147_e1865_d_n14);
        let eq147_e1866_d_n15: f64 = (p.p7 * eq147_e1865_d_n15);
        let eq147_e1866_d_n16: f64 = (p.p7 * eq147_e1865_d_n16);
        let eq147_e1866_d_n17: f64 = (p.p7 * eq147_e1865_d_n17);
        let eq147_e1866_d_n18: f64 = (p.p7 * eq147_e1865_d_n18);
        let eq147_e1866_d_n19: f64 = (p.p7 * eq147_e1865_d_n19);
        let eq147_e1866_d_n20: f64 = (p.p7 * eq147_e1865_d_n20);
        let eq147_e1866_d_n21: f64 = (p.p7 * eq147_e1865_d_n21);
        let eq147_e1866_d_n22: f64 = (p.p7 * eq147_e1865_d_n22);
        (eq147_e1866, eq147_e1866_d_n0, eq147_e1866_d_n1, eq147_e1866_d_n2, eq147_e1866_d_n3, eq147_e1866_d_n4, eq147_e1866_d_n5, eq147_e1866_d_n6, eq147_e1866_d_n7, eq147_e1866_d_n8, eq147_e1866_d_n9, eq147_e1866_d_n10, eq147_e1866_d_n11, eq147_e1866_d_n12, eq147_e1866_d_n13, eq147_e1866_d_n14, eq147_e1866_d_n15, eq147_e1866_d_n16, eq147_e1866_d_n17, eq147_e1866_d_n18, eq147_e1866_d_n19, eq147_e1866_d_n20, eq147_e1866_d_n21, eq147_e1866_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1868;
        let eq147_node_derivatives: [f64; 23] = [eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22];
        let eq147_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            multiplicity * (eq147_value),
            nodes,
            &eq147_node_derivatives,
            branches,
            &eq147_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 47, s.v[252]);
        let eq148_e1879_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq148_e1879_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq148_e1879_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq148_e1879_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq148_e1879_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq148_e1879_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq148_e1879_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq148_e1879_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq148_e1879_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq148_e1879_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq148_e1879_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq148_e1879_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq148_e1879_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq148_e1879_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq148_e1879_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq148_e1879_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq148_e1879_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq148_e1879_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq148_e1879_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq148_e1879_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq148_e1879_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq148_e1879_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq148_e1879_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq148_e1880: f64 = (eq148_e1877 * eq148_e1879);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * eq148_e1879_d_n0);
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * eq148_e1879_d_n1);
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * eq148_e1879_d_n2);
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * eq148_e1879_d_n3);
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * eq148_e1879_d_n4);
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * eq148_e1879_d_n5);
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * eq148_e1879_d_n6);
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * eq148_e1879_d_n7);
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * eq148_e1879_d_n8);
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * eq148_e1879_d_n9);
        let eq148_e1880_d_n10: f64 = (eq148_e1877 * eq148_e1879_d_n10);
        let eq148_e1880_d_n11: f64 = (eq148_e1877 * eq148_e1879_d_n11);
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * eq148_e1879_d_n12);
        let eq148_e1880_d_n13: f64 = (eq148_e1877 * eq148_e1879_d_n13);
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * eq148_e1879_d_n14);
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * eq148_e1879_d_n15);
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * eq148_e1879_d_n16);
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * eq148_e1879_d_n17);
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * eq148_e1879_d_n18);
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * eq148_e1879_d_n19);
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * eq148_e1879_d_n20);
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * eq148_e1879_d_n21);
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * eq148_e1879_d_n22);
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n10, eq148_e1880_d_n11, eq148_e1880_d_n12, eq148_e1880_d_n13, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1882;
        let eq148_node_derivatives: [f64; 23] = [eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22];
        let eq148_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            multiplicity * (eq148_value),
            nodes,
            &eq148_node_derivatives,
            branches,
            &eq148_branch_derivatives,
            multiplicity,
        );
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22,) = {
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
        let eq149_e1890: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 48, eq149_e1889);
        let eq149_e1890_d_n0: f64 = (eq149_e1889_d_n0 * ddt_scale);
        let eq149_e1890_d_n1: f64 = (eq149_e1889_d_n1 * ddt_scale);
        let eq149_e1890_d_n2: f64 = (eq149_e1889_d_n2 * ddt_scale);
        let eq149_e1890_d_n3: f64 = (eq149_e1889_d_n3 * ddt_scale);
        let eq149_e1890_d_n4: f64 = (eq149_e1889_d_n4 * ddt_scale);
        let eq149_e1890_d_n5: f64 = (eq149_e1889_d_n5 * ddt_scale);
        let eq149_e1890_d_n6: f64 = (eq149_e1889_d_n6 * ddt_scale);
        let eq149_e1890_d_n7: f64 = (eq149_e1889_d_n7 * ddt_scale);
        let eq149_e1890_d_n8: f64 = (eq149_e1889_d_n8 * ddt_scale);
        let eq149_e1890_d_n9: f64 = (eq149_e1889_d_n9 * ddt_scale);
        let eq149_e1890_d_n10: f64 = (eq149_e1889_d_n10 * ddt_scale);
        let eq149_e1890_d_n11: f64 = (eq149_e1889_d_n11 * ddt_scale);
        let eq149_e1890_d_n12: f64 = (eq149_e1889_d_n12 * ddt_scale);
        let eq149_e1890_d_n13: f64 = (eq149_e1889_d_n13 * ddt_scale);
        let eq149_e1890_d_n14: f64 = (eq149_e1889_d_n14 * ddt_scale);
        let eq149_e1890_d_n15: f64 = (eq149_e1889_d_n15 * ddt_scale);
        let eq149_e1890_d_n16: f64 = (eq149_e1889_d_n16 * ddt_scale);
        let eq149_e1890_d_n17: f64 = (eq149_e1889_d_n17 * ddt_scale);
        let eq149_e1890_d_n18: f64 = (eq149_e1889_d_n18 * ddt_scale);
        let eq149_e1890_d_n19: f64 = (eq149_e1889_d_n19 * ddt_scale);
        let eq149_e1890_d_n20: f64 = (eq149_e1889_d_n20 * ddt_scale);
        let eq149_e1890_d_n21: f64 = (eq149_e1889_d_n21 * ddt_scale);
        let eq149_e1890_d_n22: f64 = (eq149_e1889_d_n22 * ddt_scale);
        let eq149_e1891: f64 = (p.p7 * eq149_e1890);
        let eq149_e1891_d_n0: f64 = (p.p7 * eq149_e1890_d_n0);
        let eq149_e1891_d_n1: f64 = (p.p7 * eq149_e1890_d_n1);
        let eq149_e1891_d_n2: f64 = (p.p7 * eq149_e1890_d_n2);
        let eq149_e1891_d_n3: f64 = (p.p7 * eq149_e1890_d_n3);
        let eq149_e1891_d_n4: f64 = (p.p7 * eq149_e1890_d_n4);
        let eq149_e1891_d_n5: f64 = (p.p7 * eq149_e1890_d_n5);
        let eq149_e1891_d_n6: f64 = (p.p7 * eq149_e1890_d_n6);
        let eq149_e1891_d_n7: f64 = (p.p7 * eq149_e1890_d_n7);
        let eq149_e1891_d_n8: f64 = (p.p7 * eq149_e1890_d_n8);
        let eq149_e1891_d_n9: f64 = (p.p7 * eq149_e1890_d_n9);
        let eq149_e1891_d_n10: f64 = (p.p7 * eq149_e1890_d_n10);
        let eq149_e1891_d_n11: f64 = (p.p7 * eq149_e1890_d_n11);
        let eq149_e1891_d_n12: f64 = (p.p7 * eq149_e1890_d_n12);
        let eq149_e1891_d_n13: f64 = (p.p7 * eq149_e1890_d_n13);
        let eq149_e1891_d_n14: f64 = (p.p7 * eq149_e1890_d_n14);
        let eq149_e1891_d_n15: f64 = (p.p7 * eq149_e1890_d_n15);
        let eq149_e1891_d_n16: f64 = (p.p7 * eq149_e1890_d_n16);
        let eq149_e1891_d_n17: f64 = (p.p7 * eq149_e1890_d_n17);
        let eq149_e1891_d_n18: f64 = (p.p7 * eq149_e1890_d_n18);
        let eq149_e1891_d_n19: f64 = (p.p7 * eq149_e1890_d_n19);
        let eq149_e1891_d_n20: f64 = (p.p7 * eq149_e1890_d_n20);
        let eq149_e1891_d_n21: f64 = (p.p7 * eq149_e1890_d_n21);
        let eq149_e1891_d_n22: f64 = (p.p7 * eq149_e1890_d_n22);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n10, eq149_e1891_d_n11, eq149_e1891_d_n12, eq149_e1891_d_n13, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1893;
        let eq149_node_derivatives: [f64; 23] = [eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22];
        let eq149_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            multiplicity * (eq149_value),
            nodes,
            &eq149_node_derivatives,
            branches,
            &eq149_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22,) = {
    if ((!s.b[580]) && s.b[583]) {
        let eq150_e1900: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 49, s.v[253]);
        let eq150_e1900_d_n0: f64 = (s.dn[253][0] * ddt_scale);
        let eq150_e1900_d_n1: f64 = (s.dn[253][1] * ddt_scale);
        let eq150_e1900_d_n2: f64 = (s.dn[253][2] * ddt_scale);
        let eq150_e1900_d_n3: f64 = (s.dn[253][3] * ddt_scale);
        let eq150_e1900_d_n4: f64 = (s.dn[253][4] * ddt_scale);
        let eq150_e1900_d_n5: f64 = (s.dn[253][5] * ddt_scale);
        let eq150_e1900_d_n6: f64 = (s.dn[253][6] * ddt_scale);
        let eq150_e1900_d_n7: f64 = (s.dn[253][7] * ddt_scale);
        let eq150_e1900_d_n8: f64 = (s.dn[253][8] * ddt_scale);
        let eq150_e1900_d_n9: f64 = (s.dn[253][9] * ddt_scale);
        let eq150_e1900_d_n10: f64 = (s.dn[253][10] * ddt_scale);
        let eq150_e1900_d_n11: f64 = (s.dn[253][11] * ddt_scale);
        let eq150_e1900_d_n12: f64 = (s.dn[253][12] * ddt_scale);
        let eq150_e1900_d_n13: f64 = (s.dn[253][13] * ddt_scale);
        let eq150_e1900_d_n14: f64 = (s.dn[253][14] * ddt_scale);
        let eq150_e1900_d_n15: f64 = (s.dn[253][15] * ddt_scale);
        let eq150_e1900_d_n16: f64 = (s.dn[253][16] * ddt_scale);
        let eq150_e1900_d_n17: f64 = (s.dn[253][17] * ddt_scale);
        let eq150_e1900_d_n18: f64 = (s.dn[253][18] * ddt_scale);
        let eq150_e1900_d_n19: f64 = (s.dn[253][19] * ddt_scale);
        let eq150_e1900_d_n20: f64 = (s.dn[253][20] * ddt_scale);
        let eq150_e1900_d_n21: f64 = (s.dn[253][21] * ddt_scale);
        let eq150_e1900_d_n22: f64 = (s.dn[253][22] * ddt_scale);
        let eq150_e1901: f64 = (p.p7 * eq150_e1900);
        let eq150_e1901_d_n0: f64 = (p.p7 * eq150_e1900_d_n0);
        let eq150_e1901_d_n1: f64 = (p.p7 * eq150_e1900_d_n1);
        let eq150_e1901_d_n2: f64 = (p.p7 * eq150_e1900_d_n2);
        let eq150_e1901_d_n3: f64 = (p.p7 * eq150_e1900_d_n3);
        let eq150_e1901_d_n4: f64 = (p.p7 * eq150_e1900_d_n4);
        let eq150_e1901_d_n5: f64 = (p.p7 * eq150_e1900_d_n5);
        let eq150_e1901_d_n6: f64 = (p.p7 * eq150_e1900_d_n6);
        let eq150_e1901_d_n7: f64 = (p.p7 * eq150_e1900_d_n7);
        let eq150_e1901_d_n8: f64 = (p.p7 * eq150_e1900_d_n8);
        let eq150_e1901_d_n9: f64 = (p.p7 * eq150_e1900_d_n9);
        let eq150_e1901_d_n10: f64 = (p.p7 * eq150_e1900_d_n10);
        let eq150_e1901_d_n11: f64 = (p.p7 * eq150_e1900_d_n11);
        let eq150_e1901_d_n12: f64 = (p.p7 * eq150_e1900_d_n12);
        let eq150_e1901_d_n13: f64 = (p.p7 * eq150_e1900_d_n13);
        let eq150_e1901_d_n14: f64 = (p.p7 * eq150_e1900_d_n14);
        let eq150_e1901_d_n15: f64 = (p.p7 * eq150_e1900_d_n15);
        let eq150_e1901_d_n16: f64 = (p.p7 * eq150_e1900_d_n16);
        let eq150_e1901_d_n17: f64 = (p.p7 * eq150_e1900_d_n17);
        let eq150_e1901_d_n18: f64 = (p.p7 * eq150_e1900_d_n18);
        let eq150_e1901_d_n19: f64 = (p.p7 * eq150_e1900_d_n19);
        let eq150_e1901_d_n20: f64 = (p.p7 * eq150_e1900_d_n20);
        let eq150_e1901_d_n21: f64 = (p.p7 * eq150_e1900_d_n21);
        let eq150_e1901_d_n22: f64 = (p.p7 * eq150_e1900_d_n22);
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n10, eq150_e1901_d_n11, eq150_e1901_d_n12, eq150_e1901_d_n13, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1903;
        let eq150_node_derivatives: [f64; 23] = [eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22];
        let eq150_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            multiplicity * (eq150_value),
            nodes,
            &eq150_node_derivatives,
            branches,
            &eq150_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && s.b[584]) {
        let eq151_e1912: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 50, s.v[252]);
        let eq151_e1912_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq151_e1912_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq151_e1912_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq151_e1912_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq151_e1912_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq151_e1912_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq151_e1912_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq151_e1912_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq151_e1912_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq151_e1912_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq151_e1912_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq151_e1912_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq151_e1912_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq151_e1912_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq151_e1912_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq151_e1912_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq151_e1912_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq151_e1912_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq151_e1912_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq151_e1912_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq151_e1912_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq151_e1912_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq151_e1912_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq151_e1913: f64 = (p.p7 * eq151_e1912);
        let eq151_e1913_d_n0: f64 = (p.p7 * eq151_e1912_d_n0);
        let eq151_e1913_d_n1: f64 = (p.p7 * eq151_e1912_d_n1);
        let eq151_e1913_d_n2: f64 = (p.p7 * eq151_e1912_d_n2);
        let eq151_e1913_d_n3: f64 = (p.p7 * eq151_e1912_d_n3);
        let eq151_e1913_d_n4: f64 = (p.p7 * eq151_e1912_d_n4);
        let eq151_e1913_d_n5: f64 = (p.p7 * eq151_e1912_d_n5);
        let eq151_e1913_d_n6: f64 = (p.p7 * eq151_e1912_d_n6);
        let eq151_e1913_d_n7: f64 = (p.p7 * eq151_e1912_d_n7);
        let eq151_e1913_d_n8: f64 = (p.p7 * eq151_e1912_d_n8);
        let eq151_e1913_d_n9: f64 = (p.p7 * eq151_e1912_d_n9);
        let eq151_e1913_d_n10: f64 = (p.p7 * eq151_e1912_d_n10);
        let eq151_e1913_d_n11: f64 = (p.p7 * eq151_e1912_d_n11);
        let eq151_e1913_d_n12: f64 = (p.p7 * eq151_e1912_d_n12);
        let eq151_e1913_d_n13: f64 = (p.p7 * eq151_e1912_d_n13);
        let eq151_e1913_d_n14: f64 = (p.p7 * eq151_e1912_d_n14);
        let eq151_e1913_d_n15: f64 = (p.p7 * eq151_e1912_d_n15);
        let eq151_e1913_d_n16: f64 = (p.p7 * eq151_e1912_d_n16);
        let eq151_e1913_d_n17: f64 = (p.p7 * eq151_e1912_d_n17);
        let eq151_e1913_d_n18: f64 = (p.p7 * eq151_e1912_d_n18);
        let eq151_e1913_d_n19: f64 = (p.p7 * eq151_e1912_d_n19);
        let eq151_e1913_d_n20: f64 = (p.p7 * eq151_e1912_d_n20);
        let eq151_e1913_d_n21: f64 = (p.p7 * eq151_e1912_d_n21);
        let eq151_e1913_d_n22: f64 = (p.p7 * eq151_e1912_d_n22);
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n10, eq151_e1913_d_n11, eq151_e1913_d_n12, eq151_e1913_d_n13, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1915;
        let eq151_node_derivatives: [f64; 23] = [eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22];
        let eq151_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq151_value),
            nodes,
            &eq151_node_derivatives,
            branches,
            &eq151_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && s.b[584]) {
        let eq152_e1924: f64 = (p.p7 * p.p247);
        let eq152_e1926: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 51, s.v[252]);
        let eq152_e1926_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq152_e1926_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq152_e1926_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq152_e1926_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq152_e1926_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq152_e1926_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq152_e1926_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq152_e1926_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq152_e1926_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq152_e1926_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq152_e1926_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq152_e1926_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq152_e1926_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq152_e1926_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq152_e1926_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq152_e1926_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq152_e1926_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq152_e1926_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq152_e1926_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq152_e1926_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq152_e1926_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq152_e1926_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq152_e1926_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq152_e1927: f64 = (eq152_e1924 * eq152_e1926);
        let eq152_e1927_d_n0: f64 = (eq152_e1924 * eq152_e1926_d_n0);
        let eq152_e1927_d_n1: f64 = (eq152_e1924 * eq152_e1926_d_n1);
        let eq152_e1927_d_n2: f64 = (eq152_e1924 * eq152_e1926_d_n2);
        let eq152_e1927_d_n3: f64 = (eq152_e1924 * eq152_e1926_d_n3);
        let eq152_e1927_d_n4: f64 = (eq152_e1924 * eq152_e1926_d_n4);
        let eq152_e1927_d_n5: f64 = (eq152_e1924 * eq152_e1926_d_n5);
        let eq152_e1927_d_n6: f64 = (eq152_e1924 * eq152_e1926_d_n6);
        let eq152_e1927_d_n7: f64 = (eq152_e1924 * eq152_e1926_d_n7);
        let eq152_e1927_d_n8: f64 = (eq152_e1924 * eq152_e1926_d_n8);
        let eq152_e1927_d_n9: f64 = (eq152_e1924 * eq152_e1926_d_n9);
        let eq152_e1927_d_n10: f64 = (eq152_e1924 * eq152_e1926_d_n10);
        let eq152_e1927_d_n11: f64 = (eq152_e1924 * eq152_e1926_d_n11);
        let eq152_e1927_d_n12: f64 = (eq152_e1924 * eq152_e1926_d_n12);
        let eq152_e1927_d_n13: f64 = (eq152_e1924 * eq152_e1926_d_n13);
        let eq152_e1927_d_n14: f64 = (eq152_e1924 * eq152_e1926_d_n14);
        let eq152_e1927_d_n15: f64 = (eq152_e1924 * eq152_e1926_d_n15);
        let eq152_e1927_d_n16: f64 = (eq152_e1924 * eq152_e1926_d_n16);
        let eq152_e1927_d_n17: f64 = (eq152_e1924 * eq152_e1926_d_n17);
        let eq152_e1927_d_n18: f64 = (eq152_e1924 * eq152_e1926_d_n18);
        let eq152_e1927_d_n19: f64 = (eq152_e1924 * eq152_e1926_d_n19);
        let eq152_e1927_d_n20: f64 = (eq152_e1924 * eq152_e1926_d_n20);
        let eq152_e1927_d_n21: f64 = (eq152_e1924 * eq152_e1926_d_n21);
        let eq152_e1927_d_n22: f64 = (eq152_e1924 * eq152_e1926_d_n22);
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n10, eq152_e1927_d_n11, eq152_e1927_d_n12, eq152_e1927_d_n13, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1929;
        let eq152_node_derivatives: [f64; 23] = [eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22];
        let eq152_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq152_value),
            nodes,
            &eq152_node_derivatives,
            branches,
            &eq152_branch_derivatives,
            multiplicity,
        );
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && (!s.b[584])) {
        let eq153_e1939: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 52, s.v[252]);
        let eq153_e1939_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq153_e1939_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq153_e1939_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq153_e1939_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq153_e1939_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq153_e1939_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq153_e1939_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq153_e1939_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq153_e1939_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq153_e1939_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq153_e1939_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq153_e1939_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq153_e1939_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq153_e1939_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq153_e1939_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq153_e1939_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq153_e1939_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq153_e1939_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq153_e1939_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq153_e1939_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq153_e1939_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq153_e1939_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq153_e1939_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq153_e1940: f64 = (p.p7 * eq153_e1939);
        let eq153_e1940_d_n0: f64 = (p.p7 * eq153_e1939_d_n0);
        let eq153_e1940_d_n1: f64 = (p.p7 * eq153_e1939_d_n1);
        let eq153_e1940_d_n2: f64 = (p.p7 * eq153_e1939_d_n2);
        let eq153_e1940_d_n3: f64 = (p.p7 * eq153_e1939_d_n3);
        let eq153_e1940_d_n4: f64 = (p.p7 * eq153_e1939_d_n4);
        let eq153_e1940_d_n5: f64 = (p.p7 * eq153_e1939_d_n5);
        let eq153_e1940_d_n6: f64 = (p.p7 * eq153_e1939_d_n6);
        let eq153_e1940_d_n7: f64 = (p.p7 * eq153_e1939_d_n7);
        let eq153_e1940_d_n8: f64 = (p.p7 * eq153_e1939_d_n8);
        let eq153_e1940_d_n9: f64 = (p.p7 * eq153_e1939_d_n9);
        let eq153_e1940_d_n10: f64 = (p.p7 * eq153_e1939_d_n10);
        let eq153_e1940_d_n11: f64 = (p.p7 * eq153_e1939_d_n11);
        let eq153_e1940_d_n12: f64 = (p.p7 * eq153_e1939_d_n12);
        let eq153_e1940_d_n13: f64 = (p.p7 * eq153_e1939_d_n13);
        let eq153_e1940_d_n14: f64 = (p.p7 * eq153_e1939_d_n14);
        let eq153_e1940_d_n15: f64 = (p.p7 * eq153_e1939_d_n15);
        let eq153_e1940_d_n16: f64 = (p.p7 * eq153_e1939_d_n16);
        let eq153_e1940_d_n17: f64 = (p.p7 * eq153_e1939_d_n17);
        let eq153_e1940_d_n18: f64 = (p.p7 * eq153_e1939_d_n18);
        let eq153_e1940_d_n19: f64 = (p.p7 * eq153_e1939_d_n19);
        let eq153_e1940_d_n20: f64 = (p.p7 * eq153_e1939_d_n20);
        let eq153_e1940_d_n21: f64 = (p.p7 * eq153_e1939_d_n21);
        let eq153_e1940_d_n22: f64 = (p.p7 * eq153_e1939_d_n22);
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n10, eq153_e1940_d_n11, eq153_e1940_d_n12, eq153_e1940_d_n13, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1942;
        let eq153_node_derivatives: [f64; 23] = [eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22];
        let eq153_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq153_value),
            nodes,
            &eq153_node_derivatives,
            branches,
            &eq153_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_15(
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
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22,) = {
    if (((!s.b[580]) && s.b[583]) && (!s.b[584])) {
        let eq154_e1952: f64 = (p.p7 * p.p247);
        let eq154_e1954: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 53, s.v[252]);
        let eq154_e1954_d_n0: f64 = (s.dn[252][0] * ddt_scale);
        let eq154_e1954_d_n1: f64 = (s.dn[252][1] * ddt_scale);
        let eq154_e1954_d_n2: f64 = (s.dn[252][2] * ddt_scale);
        let eq154_e1954_d_n3: f64 = (s.dn[252][3] * ddt_scale);
        let eq154_e1954_d_n4: f64 = (s.dn[252][4] * ddt_scale);
        let eq154_e1954_d_n5: f64 = (s.dn[252][5] * ddt_scale);
        let eq154_e1954_d_n6: f64 = (s.dn[252][6] * ddt_scale);
        let eq154_e1954_d_n7: f64 = (s.dn[252][7] * ddt_scale);
        let eq154_e1954_d_n8: f64 = (s.dn[252][8] * ddt_scale);
        let eq154_e1954_d_n9: f64 = (s.dn[252][9] * ddt_scale);
        let eq154_e1954_d_n10: f64 = (s.dn[252][10] * ddt_scale);
        let eq154_e1954_d_n11: f64 = (s.dn[252][11] * ddt_scale);
        let eq154_e1954_d_n12: f64 = (s.dn[252][12] * ddt_scale);
        let eq154_e1954_d_n13: f64 = (s.dn[252][13] * ddt_scale);
        let eq154_e1954_d_n14: f64 = (s.dn[252][14] * ddt_scale);
        let eq154_e1954_d_n15: f64 = (s.dn[252][15] * ddt_scale);
        let eq154_e1954_d_n16: f64 = (s.dn[252][16] * ddt_scale);
        let eq154_e1954_d_n17: f64 = (s.dn[252][17] * ddt_scale);
        let eq154_e1954_d_n18: f64 = (s.dn[252][18] * ddt_scale);
        let eq154_e1954_d_n19: f64 = (s.dn[252][19] * ddt_scale);
        let eq154_e1954_d_n20: f64 = (s.dn[252][20] * ddt_scale);
        let eq154_e1954_d_n21: f64 = (s.dn[252][21] * ddt_scale);
        let eq154_e1954_d_n22: f64 = (s.dn[252][22] * ddt_scale);
        let eq154_e1955: f64 = (eq154_e1952 * eq154_e1954);
        let eq154_e1955_d_n0: f64 = (eq154_e1952 * eq154_e1954_d_n0);
        let eq154_e1955_d_n1: f64 = (eq154_e1952 * eq154_e1954_d_n1);
        let eq154_e1955_d_n2: f64 = (eq154_e1952 * eq154_e1954_d_n2);
        let eq154_e1955_d_n3: f64 = (eq154_e1952 * eq154_e1954_d_n3);
        let eq154_e1955_d_n4: f64 = (eq154_e1952 * eq154_e1954_d_n4);
        let eq154_e1955_d_n5: f64 = (eq154_e1952 * eq154_e1954_d_n5);
        let eq154_e1955_d_n6: f64 = (eq154_e1952 * eq154_e1954_d_n6);
        let eq154_e1955_d_n7: f64 = (eq154_e1952 * eq154_e1954_d_n7);
        let eq154_e1955_d_n8: f64 = (eq154_e1952 * eq154_e1954_d_n8);
        let eq154_e1955_d_n9: f64 = (eq154_e1952 * eq154_e1954_d_n9);
        let eq154_e1955_d_n10: f64 = (eq154_e1952 * eq154_e1954_d_n10);
        let eq154_e1955_d_n11: f64 = (eq154_e1952 * eq154_e1954_d_n11);
        let eq154_e1955_d_n12: f64 = (eq154_e1952 * eq154_e1954_d_n12);
        let eq154_e1955_d_n13: f64 = (eq154_e1952 * eq154_e1954_d_n13);
        let eq154_e1955_d_n14: f64 = (eq154_e1952 * eq154_e1954_d_n14);
        let eq154_e1955_d_n15: f64 = (eq154_e1952 * eq154_e1954_d_n15);
        let eq154_e1955_d_n16: f64 = (eq154_e1952 * eq154_e1954_d_n16);
        let eq154_e1955_d_n17: f64 = (eq154_e1952 * eq154_e1954_d_n17);
        let eq154_e1955_d_n18: f64 = (eq154_e1952 * eq154_e1954_d_n18);
        let eq154_e1955_d_n19: f64 = (eq154_e1952 * eq154_e1954_d_n19);
        let eq154_e1955_d_n20: f64 = (eq154_e1952 * eq154_e1954_d_n20);
        let eq154_e1955_d_n21: f64 = (eq154_e1952 * eq154_e1954_d_n21);
        let eq154_e1955_d_n22: f64 = (eq154_e1952 * eq154_e1954_d_n22);
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n10, eq154_e1955_d_n11, eq154_e1955_d_n12, eq154_e1955_d_n13, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1957;
        let eq154_node_derivatives: [f64; 23] = [eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22];
        let eq154_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq154_value),
            nodes,
            &eq154_node_derivatives,
            branches,
            &eq154_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22,) = {
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
        let eq155_e1966: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 54, eq155_e1965);
        let eq155_e1966_d_n0: f64 = (eq155_e1965_d_n0 * ddt_scale);
        let eq155_e1966_d_n1: f64 = (eq155_e1965_d_n1 * ddt_scale);
        let eq155_e1966_d_n2: f64 = (eq155_e1965_d_n2 * ddt_scale);
        let eq155_e1966_d_n3: f64 = (eq155_e1965_d_n3 * ddt_scale);
        let eq155_e1966_d_n4: f64 = (eq155_e1965_d_n4 * ddt_scale);
        let eq155_e1966_d_n5: f64 = (eq155_e1965_d_n5 * ddt_scale);
        let eq155_e1966_d_n6: f64 = (eq155_e1965_d_n6 * ddt_scale);
        let eq155_e1966_d_n7: f64 = (eq155_e1965_d_n7 * ddt_scale);
        let eq155_e1966_d_n8: f64 = (eq155_e1965_d_n8 * ddt_scale);
        let eq155_e1966_d_n9: f64 = (eq155_e1965_d_n9 * ddt_scale);
        let eq155_e1966_d_n10: f64 = (eq155_e1965_d_n10 * ddt_scale);
        let eq155_e1966_d_n11: f64 = (eq155_e1965_d_n11 * ddt_scale);
        let eq155_e1966_d_n12: f64 = (eq155_e1965_d_n12 * ddt_scale);
        let eq155_e1966_d_n13: f64 = (eq155_e1965_d_n13 * ddt_scale);
        let eq155_e1966_d_n14: f64 = (eq155_e1965_d_n14 * ddt_scale);
        let eq155_e1966_d_n15: f64 = (eq155_e1965_d_n15 * ddt_scale);
        let eq155_e1966_d_n16: f64 = (eq155_e1965_d_n16 * ddt_scale);
        let eq155_e1966_d_n17: f64 = (eq155_e1965_d_n17 * ddt_scale);
        let eq155_e1966_d_n18: f64 = (eq155_e1965_d_n18 * ddt_scale);
        let eq155_e1966_d_n19: f64 = (eq155_e1965_d_n19 * ddt_scale);
        let eq155_e1966_d_n20: f64 = (eq155_e1965_d_n20 * ddt_scale);
        let eq155_e1966_d_n21: f64 = (eq155_e1965_d_n21 * ddt_scale);
        let eq155_e1966_d_n22: f64 = (eq155_e1965_d_n22 * ddt_scale);
        let eq155_e1967: f64 = (p.p7 * eq155_e1966);
        let eq155_e1967_d_n0: f64 = (p.p7 * eq155_e1966_d_n0);
        let eq155_e1967_d_n1: f64 = (p.p7 * eq155_e1966_d_n1);
        let eq155_e1967_d_n2: f64 = (p.p7 * eq155_e1966_d_n2);
        let eq155_e1967_d_n3: f64 = (p.p7 * eq155_e1966_d_n3);
        let eq155_e1967_d_n4: f64 = (p.p7 * eq155_e1966_d_n4);
        let eq155_e1967_d_n5: f64 = (p.p7 * eq155_e1966_d_n5);
        let eq155_e1967_d_n6: f64 = (p.p7 * eq155_e1966_d_n6);
        let eq155_e1967_d_n7: f64 = (p.p7 * eq155_e1966_d_n7);
        let eq155_e1967_d_n8: f64 = (p.p7 * eq155_e1966_d_n8);
        let eq155_e1967_d_n9: f64 = (p.p7 * eq155_e1966_d_n9);
        let eq155_e1967_d_n10: f64 = (p.p7 * eq155_e1966_d_n10);
        let eq155_e1967_d_n11: f64 = (p.p7 * eq155_e1966_d_n11);
        let eq155_e1967_d_n12: f64 = (p.p7 * eq155_e1966_d_n12);
        let eq155_e1967_d_n13: f64 = (p.p7 * eq155_e1966_d_n13);
        let eq155_e1967_d_n14: f64 = (p.p7 * eq155_e1966_d_n14);
        let eq155_e1967_d_n15: f64 = (p.p7 * eq155_e1966_d_n15);
        let eq155_e1967_d_n16: f64 = (p.p7 * eq155_e1966_d_n16);
        let eq155_e1967_d_n17: f64 = (p.p7 * eq155_e1966_d_n17);
        let eq155_e1967_d_n18: f64 = (p.p7 * eq155_e1966_d_n18);
        let eq155_e1967_d_n19: f64 = (p.p7 * eq155_e1966_d_n19);
        let eq155_e1967_d_n20: f64 = (p.p7 * eq155_e1966_d_n20);
        let eq155_e1967_d_n21: f64 = (p.p7 * eq155_e1966_d_n21);
        let eq155_e1967_d_n22: f64 = (p.p7 * eq155_e1966_d_n22);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n10, eq155_e1967_d_n11, eq155_e1967_d_n12, eq155_e1967_d_n13, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1969;
        let eq155_node_derivatives: [f64; 23] = [eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22];
        let eq155_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            multiplicity * (eq155_value),
            nodes,
            &eq155_node_derivatives,
            branches,
            &eq155_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22,) = {
    if (s.b[585] && s.b[586]) {
        let eq156_e1975: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 55, s.v[265]);
        let eq156_e1975_d_n0: f64 = (s.dn[265][0] * ddt_scale);
        let eq156_e1975_d_n1: f64 = (s.dn[265][1] * ddt_scale);
        let eq156_e1975_d_n2: f64 = (s.dn[265][2] * ddt_scale);
        let eq156_e1975_d_n3: f64 = (s.dn[265][3] * ddt_scale);
        let eq156_e1975_d_n4: f64 = (s.dn[265][4] * ddt_scale);
        let eq156_e1975_d_n5: f64 = (s.dn[265][5] * ddt_scale);
        let eq156_e1975_d_n6: f64 = (s.dn[265][6] * ddt_scale);
        let eq156_e1975_d_n7: f64 = (s.dn[265][7] * ddt_scale);
        let eq156_e1975_d_n8: f64 = (s.dn[265][8] * ddt_scale);
        let eq156_e1975_d_n9: f64 = (s.dn[265][9] * ddt_scale);
        let eq156_e1975_d_n10: f64 = (s.dn[265][10] * ddt_scale);
        let eq156_e1975_d_n11: f64 = (s.dn[265][11] * ddt_scale);
        let eq156_e1975_d_n12: f64 = (s.dn[265][12] * ddt_scale);
        let eq156_e1975_d_n13: f64 = (s.dn[265][13] * ddt_scale);
        let eq156_e1975_d_n14: f64 = (s.dn[265][14] * ddt_scale);
        let eq156_e1975_d_n15: f64 = (s.dn[265][15] * ddt_scale);
        let eq156_e1975_d_n16: f64 = (s.dn[265][16] * ddt_scale);
        let eq156_e1975_d_n17: f64 = (s.dn[265][17] * ddt_scale);
        let eq156_e1975_d_n18: f64 = (s.dn[265][18] * ddt_scale);
        let eq156_e1975_d_n19: f64 = (s.dn[265][19] * ddt_scale);
        let eq156_e1975_d_n20: f64 = (s.dn[265][20] * ddt_scale);
        let eq156_e1975_d_n21: f64 = (s.dn[265][21] * ddt_scale);
        let eq156_e1975_d_n22: f64 = (s.dn[265][22] * ddt_scale);
        let eq156_e1976: f64 = (p.p7 * eq156_e1975);
        let eq156_e1976_d_n0: f64 = (p.p7 * eq156_e1975_d_n0);
        let eq156_e1976_d_n1: f64 = (p.p7 * eq156_e1975_d_n1);
        let eq156_e1976_d_n2: f64 = (p.p7 * eq156_e1975_d_n2);
        let eq156_e1976_d_n3: f64 = (p.p7 * eq156_e1975_d_n3);
        let eq156_e1976_d_n4: f64 = (p.p7 * eq156_e1975_d_n4);
        let eq156_e1976_d_n5: f64 = (p.p7 * eq156_e1975_d_n5);
        let eq156_e1976_d_n6: f64 = (p.p7 * eq156_e1975_d_n6);
        let eq156_e1976_d_n7: f64 = (p.p7 * eq156_e1975_d_n7);
        let eq156_e1976_d_n8: f64 = (p.p7 * eq156_e1975_d_n8);
        let eq156_e1976_d_n9: f64 = (p.p7 * eq156_e1975_d_n9);
        let eq156_e1976_d_n10: f64 = (p.p7 * eq156_e1975_d_n10);
        let eq156_e1976_d_n11: f64 = (p.p7 * eq156_e1975_d_n11);
        let eq156_e1976_d_n12: f64 = (p.p7 * eq156_e1975_d_n12);
        let eq156_e1976_d_n13: f64 = (p.p7 * eq156_e1975_d_n13);
        let eq156_e1976_d_n14: f64 = (p.p7 * eq156_e1975_d_n14);
        let eq156_e1976_d_n15: f64 = (p.p7 * eq156_e1975_d_n15);
        let eq156_e1976_d_n16: f64 = (p.p7 * eq156_e1975_d_n16);
        let eq156_e1976_d_n17: f64 = (p.p7 * eq156_e1975_d_n17);
        let eq156_e1976_d_n18: f64 = (p.p7 * eq156_e1975_d_n18);
        let eq156_e1976_d_n19: f64 = (p.p7 * eq156_e1975_d_n19);
        let eq156_e1976_d_n20: f64 = (p.p7 * eq156_e1975_d_n20);
        let eq156_e1976_d_n21: f64 = (p.p7 * eq156_e1975_d_n21);
        let eq156_e1976_d_n22: f64 = (p.p7 * eq156_e1975_d_n22);
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n10, eq156_e1976_d_n11, eq156_e1976_d_n12, eq156_e1976_d_n13, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1978;
        let eq156_node_derivatives: [f64; 23] = [eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22];
        let eq156_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[20]),
            multiplicity * (eq156_value),
            nodes,
            &eq156_node_derivatives,
            branches,
            &eq156_branch_derivatives,
            multiplicity,
        );
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq157_e1986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 56, s.v[264]);
        let eq157_e1986_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq157_e1986_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq157_e1986_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq157_e1986_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq157_e1986_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq157_e1986_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq157_e1986_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq157_e1986_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq157_e1986_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq157_e1986_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq157_e1986_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq157_e1986_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq157_e1986_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq157_e1986_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq157_e1986_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq157_e1986_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq157_e1986_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq157_e1986_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq157_e1986_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq157_e1986_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq157_e1986_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq157_e1986_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq157_e1986_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq157_e1987: f64 = (p.p7 * eq157_e1986);
        let eq157_e1987_d_n0: f64 = (p.p7 * eq157_e1986_d_n0);
        let eq157_e1987_d_n1: f64 = (p.p7 * eq157_e1986_d_n1);
        let eq157_e1987_d_n2: f64 = (p.p7 * eq157_e1986_d_n2);
        let eq157_e1987_d_n3: f64 = (p.p7 * eq157_e1986_d_n3);
        let eq157_e1987_d_n4: f64 = (p.p7 * eq157_e1986_d_n4);
        let eq157_e1987_d_n5: f64 = (p.p7 * eq157_e1986_d_n5);
        let eq157_e1987_d_n6: f64 = (p.p7 * eq157_e1986_d_n6);
        let eq157_e1987_d_n7: f64 = (p.p7 * eq157_e1986_d_n7);
        let eq157_e1987_d_n8: f64 = (p.p7 * eq157_e1986_d_n8);
        let eq157_e1987_d_n9: f64 = (p.p7 * eq157_e1986_d_n9);
        let eq157_e1987_d_n10: f64 = (p.p7 * eq157_e1986_d_n10);
        let eq157_e1987_d_n11: f64 = (p.p7 * eq157_e1986_d_n11);
        let eq157_e1987_d_n12: f64 = (p.p7 * eq157_e1986_d_n12);
        let eq157_e1987_d_n13: f64 = (p.p7 * eq157_e1986_d_n13);
        let eq157_e1987_d_n14: f64 = (p.p7 * eq157_e1986_d_n14);
        let eq157_e1987_d_n15: f64 = (p.p7 * eq157_e1986_d_n15);
        let eq157_e1987_d_n16: f64 = (p.p7 * eq157_e1986_d_n16);
        let eq157_e1987_d_n17: f64 = (p.p7 * eq157_e1986_d_n17);
        let eq157_e1987_d_n18: f64 = (p.p7 * eq157_e1986_d_n18);
        let eq157_e1987_d_n19: f64 = (p.p7 * eq157_e1986_d_n19);
        let eq157_e1987_d_n20: f64 = (p.p7 * eq157_e1986_d_n20);
        let eq157_e1987_d_n21: f64 = (p.p7 * eq157_e1986_d_n21);
        let eq157_e1987_d_n22: f64 = (p.p7 * eq157_e1986_d_n22);
        (eq157_e1987, eq157_e1987_d_n0, eq157_e1987_d_n1, eq157_e1987_d_n2, eq157_e1987_d_n3, eq157_e1987_d_n4, eq157_e1987_d_n5, eq157_e1987_d_n6, eq157_e1987_d_n7, eq157_e1987_d_n8, eq157_e1987_d_n9, eq157_e1987_d_n10, eq157_e1987_d_n11, eq157_e1987_d_n12, eq157_e1987_d_n13, eq157_e1987_d_n14, eq157_e1987_d_n15, eq157_e1987_d_n16, eq157_e1987_d_n17, eq157_e1987_d_n18, eq157_e1987_d_n19, eq157_e1987_d_n20, eq157_e1987_d_n21, eq157_e1987_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1989;
        let eq157_node_derivatives: [f64; 23] = [eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22];
        let eq157_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            multiplicity * (eq157_value),
            nodes,
            &eq157_node_derivatives,
            branches,
            &eq157_branch_derivatives,
            multiplicity,
        );
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq158_e1997: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 57, s.v[264]);
        let eq158_e1997_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq158_e1997_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq158_e1997_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq158_e1997_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq158_e1997_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq158_e1997_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq158_e1997_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq158_e1997_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq158_e1997_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq158_e1997_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq158_e1997_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq158_e1997_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq158_e1997_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq158_e1997_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq158_e1997_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq158_e1997_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq158_e1997_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq158_e1997_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq158_e1997_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq158_e1997_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq158_e1997_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq158_e1997_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq158_e1997_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq158_e1998: f64 = (p.p7 * eq158_e1997);
        let eq158_e1998_d_n0: f64 = (p.p7 * eq158_e1997_d_n0);
        let eq158_e1998_d_n1: f64 = (p.p7 * eq158_e1997_d_n1);
        let eq158_e1998_d_n2: f64 = (p.p7 * eq158_e1997_d_n2);
        let eq158_e1998_d_n3: f64 = (p.p7 * eq158_e1997_d_n3);
        let eq158_e1998_d_n4: f64 = (p.p7 * eq158_e1997_d_n4);
        let eq158_e1998_d_n5: f64 = (p.p7 * eq158_e1997_d_n5);
        let eq158_e1998_d_n6: f64 = (p.p7 * eq158_e1997_d_n6);
        let eq158_e1998_d_n7: f64 = (p.p7 * eq158_e1997_d_n7);
        let eq158_e1998_d_n8: f64 = (p.p7 * eq158_e1997_d_n8);
        let eq158_e1998_d_n9: f64 = (p.p7 * eq158_e1997_d_n9);
        let eq158_e1998_d_n10: f64 = (p.p7 * eq158_e1997_d_n10);
        let eq158_e1998_d_n11: f64 = (p.p7 * eq158_e1997_d_n11);
        let eq158_e1998_d_n12: f64 = (p.p7 * eq158_e1997_d_n12);
        let eq158_e1998_d_n13: f64 = (p.p7 * eq158_e1997_d_n13);
        let eq158_e1998_d_n14: f64 = (p.p7 * eq158_e1997_d_n14);
        let eq158_e1998_d_n15: f64 = (p.p7 * eq158_e1997_d_n15);
        let eq158_e1998_d_n16: f64 = (p.p7 * eq158_e1997_d_n16);
        let eq158_e1998_d_n17: f64 = (p.p7 * eq158_e1997_d_n17);
        let eq158_e1998_d_n18: f64 = (p.p7 * eq158_e1997_d_n18);
        let eq158_e1998_d_n19: f64 = (p.p7 * eq158_e1997_d_n19);
        let eq158_e1998_d_n20: f64 = (p.p7 * eq158_e1997_d_n20);
        let eq158_e1998_d_n21: f64 = (p.p7 * eq158_e1997_d_n21);
        let eq158_e1998_d_n22: f64 = (p.p7 * eq158_e1997_d_n22);
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
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n10, eq158_e2000_d_n11, eq158_e2000_d_n12, eq158_e2000_d_n13, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_value: f64 = eq158_e2002;
        let eq158_node_derivatives: [f64; 23] = [eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22];
        let eq158_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            multiplicity * (eq158_value),
            nodes,
            &eq158_node_derivatives,
            branches,
            &eq158_branch_derivatives,
            multiplicity,
        );
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq159_e2011: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 58, s.v[264]);
        let eq159_e2011_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq159_e2011_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq159_e2011_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq159_e2011_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq159_e2011_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq159_e2011_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq159_e2011_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq159_e2011_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq159_e2011_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq159_e2011_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq159_e2011_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq159_e2011_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq159_e2011_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq159_e2011_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq159_e2011_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq159_e2011_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq159_e2011_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq159_e2011_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq159_e2011_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq159_e2011_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq159_e2011_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq159_e2011_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq159_e2011_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq159_e2012: f64 = (p.p7 * eq159_e2011);
        let eq159_e2012_d_n0: f64 = (p.p7 * eq159_e2011_d_n0);
        let eq159_e2012_d_n1: f64 = (p.p7 * eq159_e2011_d_n1);
        let eq159_e2012_d_n2: f64 = (p.p7 * eq159_e2011_d_n2);
        let eq159_e2012_d_n3: f64 = (p.p7 * eq159_e2011_d_n3);
        let eq159_e2012_d_n4: f64 = (p.p7 * eq159_e2011_d_n4);
        let eq159_e2012_d_n5: f64 = (p.p7 * eq159_e2011_d_n5);
        let eq159_e2012_d_n6: f64 = (p.p7 * eq159_e2011_d_n6);
        let eq159_e2012_d_n7: f64 = (p.p7 * eq159_e2011_d_n7);
        let eq159_e2012_d_n8: f64 = (p.p7 * eq159_e2011_d_n8);
        let eq159_e2012_d_n9: f64 = (p.p7 * eq159_e2011_d_n9);
        let eq159_e2012_d_n10: f64 = (p.p7 * eq159_e2011_d_n10);
        let eq159_e2012_d_n11: f64 = (p.p7 * eq159_e2011_d_n11);
        let eq159_e2012_d_n12: f64 = (p.p7 * eq159_e2011_d_n12);
        let eq159_e2012_d_n13: f64 = (p.p7 * eq159_e2011_d_n13);
        let eq159_e2012_d_n14: f64 = (p.p7 * eq159_e2011_d_n14);
        let eq159_e2012_d_n15: f64 = (p.p7 * eq159_e2011_d_n15);
        let eq159_e2012_d_n16: f64 = (p.p7 * eq159_e2011_d_n16);
        let eq159_e2012_d_n17: f64 = (p.p7 * eq159_e2011_d_n17);
        let eq159_e2012_d_n18: f64 = (p.p7 * eq159_e2011_d_n18);
        let eq159_e2012_d_n19: f64 = (p.p7 * eq159_e2011_d_n19);
        let eq159_e2012_d_n20: f64 = (p.p7 * eq159_e2011_d_n20);
        let eq159_e2012_d_n21: f64 = (p.p7 * eq159_e2011_d_n21);
        let eq159_e2012_d_n22: f64 = (p.p7 * eq159_e2011_d_n22);
        (eq159_e2012, eq159_e2012_d_n0, eq159_e2012_d_n1, eq159_e2012_d_n2, eq159_e2012_d_n3, eq159_e2012_d_n4, eq159_e2012_d_n5, eq159_e2012_d_n6, eq159_e2012_d_n7, eq159_e2012_d_n8, eq159_e2012_d_n9, eq159_e2012_d_n10, eq159_e2012_d_n11, eq159_e2012_d_n12, eq159_e2012_d_n13, eq159_e2012_d_n14, eq159_e2012_d_n15, eq159_e2012_d_n16, eq159_e2012_d_n17, eq159_e2012_d_n18, eq159_e2012_d_n19, eq159_e2012_d_n20, eq159_e2012_d_n21, eq159_e2012_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_value: f64 = eq159_e2014;
        let eq159_node_derivatives: [f64; 23] = [eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22];
        let eq159_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            multiplicity * (eq159_value),
            nodes,
            &eq159_node_derivatives,
            branches,
            &eq159_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_16(
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
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq160_e2023: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 59, s.v[264]);
        let eq160_e2023_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq160_e2023_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq160_e2023_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq160_e2023_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq160_e2023_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq160_e2023_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq160_e2023_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq160_e2023_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq160_e2023_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq160_e2023_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq160_e2023_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq160_e2023_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq160_e2023_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq160_e2023_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq160_e2023_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq160_e2023_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq160_e2023_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq160_e2023_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq160_e2023_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq160_e2023_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq160_e2023_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq160_e2023_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq160_e2023_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq160_e2024: f64 = (p.p7 * eq160_e2023);
        let eq160_e2024_d_n0: f64 = (p.p7 * eq160_e2023_d_n0);
        let eq160_e2024_d_n1: f64 = (p.p7 * eq160_e2023_d_n1);
        let eq160_e2024_d_n2: f64 = (p.p7 * eq160_e2023_d_n2);
        let eq160_e2024_d_n3: f64 = (p.p7 * eq160_e2023_d_n3);
        let eq160_e2024_d_n4: f64 = (p.p7 * eq160_e2023_d_n4);
        let eq160_e2024_d_n5: f64 = (p.p7 * eq160_e2023_d_n5);
        let eq160_e2024_d_n6: f64 = (p.p7 * eq160_e2023_d_n6);
        let eq160_e2024_d_n7: f64 = (p.p7 * eq160_e2023_d_n7);
        let eq160_e2024_d_n8: f64 = (p.p7 * eq160_e2023_d_n8);
        let eq160_e2024_d_n9: f64 = (p.p7 * eq160_e2023_d_n9);
        let eq160_e2024_d_n10: f64 = (p.p7 * eq160_e2023_d_n10);
        let eq160_e2024_d_n11: f64 = (p.p7 * eq160_e2023_d_n11);
        let eq160_e2024_d_n12: f64 = (p.p7 * eq160_e2023_d_n12);
        let eq160_e2024_d_n13: f64 = (p.p7 * eq160_e2023_d_n13);
        let eq160_e2024_d_n14: f64 = (p.p7 * eq160_e2023_d_n14);
        let eq160_e2024_d_n15: f64 = (p.p7 * eq160_e2023_d_n15);
        let eq160_e2024_d_n16: f64 = (p.p7 * eq160_e2023_d_n16);
        let eq160_e2024_d_n17: f64 = (p.p7 * eq160_e2023_d_n17);
        let eq160_e2024_d_n18: f64 = (p.p7 * eq160_e2023_d_n18);
        let eq160_e2024_d_n19: f64 = (p.p7 * eq160_e2023_d_n19);
        let eq160_e2024_d_n20: f64 = (p.p7 * eq160_e2023_d_n20);
        let eq160_e2024_d_n21: f64 = (p.p7 * eq160_e2023_d_n21);
        let eq160_e2024_d_n22: f64 = (p.p7 * eq160_e2023_d_n22);
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
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n10, eq160_e2026_d_n11, eq160_e2026_d_n12, eq160_e2026_d_n13, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e2028;
        let eq160_node_derivatives: [f64; 23] = [eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22];
        let eq160_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            multiplicity * (eq160_value),
            nodes,
            &eq160_node_derivatives,
            branches,
            &eq160_branch_derivatives,
            multiplicity,
        );
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22,) = {
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
        let eq161_e2036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 60, eq161_e2035);
        let eq161_e2036_d_n0: f64 = (eq161_e2035_d_n0 * ddt_scale);
        let eq161_e2036_d_n1: f64 = (eq161_e2035_d_n1 * ddt_scale);
        let eq161_e2036_d_n2: f64 = (eq161_e2035_d_n2 * ddt_scale);
        let eq161_e2036_d_n3: f64 = (eq161_e2035_d_n3 * ddt_scale);
        let eq161_e2036_d_n4: f64 = (eq161_e2035_d_n4 * ddt_scale);
        let eq161_e2036_d_n5: f64 = (eq161_e2035_d_n5 * ddt_scale);
        let eq161_e2036_d_n6: f64 = (eq161_e2035_d_n6 * ddt_scale);
        let eq161_e2036_d_n7: f64 = (eq161_e2035_d_n7 * ddt_scale);
        let eq161_e2036_d_n8: f64 = (eq161_e2035_d_n8 * ddt_scale);
        let eq161_e2036_d_n9: f64 = (eq161_e2035_d_n9 * ddt_scale);
        let eq161_e2036_d_n10: f64 = (eq161_e2035_d_n10 * ddt_scale);
        let eq161_e2036_d_n11: f64 = (eq161_e2035_d_n11 * ddt_scale);
        let eq161_e2036_d_n12: f64 = (eq161_e2035_d_n12 * ddt_scale);
        let eq161_e2036_d_n13: f64 = (eq161_e2035_d_n13 * ddt_scale);
        let eq161_e2036_d_n14: f64 = (eq161_e2035_d_n14 * ddt_scale);
        let eq161_e2036_d_n15: f64 = (eq161_e2035_d_n15 * ddt_scale);
        let eq161_e2036_d_n16: f64 = (eq161_e2035_d_n16 * ddt_scale);
        let eq161_e2036_d_n17: f64 = (eq161_e2035_d_n17 * ddt_scale);
        let eq161_e2036_d_n18: f64 = (eq161_e2035_d_n18 * ddt_scale);
        let eq161_e2036_d_n19: f64 = (eq161_e2035_d_n19 * ddt_scale);
        let eq161_e2036_d_n20: f64 = (eq161_e2035_d_n20 * ddt_scale);
        let eq161_e2036_d_n21: f64 = (eq161_e2035_d_n21 * ddt_scale);
        let eq161_e2036_d_n22: f64 = (eq161_e2035_d_n22 * ddt_scale);
        let eq161_e2037: f64 = (p.p7 * eq161_e2036);
        let eq161_e2037_d_n0: f64 = (p.p7 * eq161_e2036_d_n0);
        let eq161_e2037_d_n1: f64 = (p.p7 * eq161_e2036_d_n1);
        let eq161_e2037_d_n2: f64 = (p.p7 * eq161_e2036_d_n2);
        let eq161_e2037_d_n3: f64 = (p.p7 * eq161_e2036_d_n3);
        let eq161_e2037_d_n4: f64 = (p.p7 * eq161_e2036_d_n4);
        let eq161_e2037_d_n5: f64 = (p.p7 * eq161_e2036_d_n5);
        let eq161_e2037_d_n6: f64 = (p.p7 * eq161_e2036_d_n6);
        let eq161_e2037_d_n7: f64 = (p.p7 * eq161_e2036_d_n7);
        let eq161_e2037_d_n8: f64 = (p.p7 * eq161_e2036_d_n8);
        let eq161_e2037_d_n9: f64 = (p.p7 * eq161_e2036_d_n9);
        let eq161_e2037_d_n10: f64 = (p.p7 * eq161_e2036_d_n10);
        let eq161_e2037_d_n11: f64 = (p.p7 * eq161_e2036_d_n11);
        let eq161_e2037_d_n12: f64 = (p.p7 * eq161_e2036_d_n12);
        let eq161_e2037_d_n13: f64 = (p.p7 * eq161_e2036_d_n13);
        let eq161_e2037_d_n14: f64 = (p.p7 * eq161_e2036_d_n14);
        let eq161_e2037_d_n15: f64 = (p.p7 * eq161_e2036_d_n15);
        let eq161_e2037_d_n16: f64 = (p.p7 * eq161_e2036_d_n16);
        let eq161_e2037_d_n17: f64 = (p.p7 * eq161_e2036_d_n17);
        let eq161_e2037_d_n18: f64 = (p.p7 * eq161_e2036_d_n18);
        let eq161_e2037_d_n19: f64 = (p.p7 * eq161_e2036_d_n19);
        let eq161_e2037_d_n20: f64 = (p.p7 * eq161_e2036_d_n20);
        let eq161_e2037_d_n21: f64 = (p.p7 * eq161_e2036_d_n21);
        let eq161_e2037_d_n22: f64 = (p.p7 * eq161_e2036_d_n22);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n10, eq161_e2037_d_n11, eq161_e2037_d_n12, eq161_e2037_d_n13, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e2039;
        let eq161_node_derivatives: [f64; 23] = [eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22];
        let eq161_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[20]),
            multiplicity * (eq161_value),
            nodes,
            &eq161_node_derivatives,
            branches,
            &eq161_branch_derivatives,
            multiplicity,
        );
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22,) = {
    if ((!s.b[585]) && s.b[588]) {
        let eq162_e2046: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 61, s.v[265]);
        let eq162_e2046_d_n0: f64 = (s.dn[265][0] * ddt_scale);
        let eq162_e2046_d_n1: f64 = (s.dn[265][1] * ddt_scale);
        let eq162_e2046_d_n2: f64 = (s.dn[265][2] * ddt_scale);
        let eq162_e2046_d_n3: f64 = (s.dn[265][3] * ddt_scale);
        let eq162_e2046_d_n4: f64 = (s.dn[265][4] * ddt_scale);
        let eq162_e2046_d_n5: f64 = (s.dn[265][5] * ddt_scale);
        let eq162_e2046_d_n6: f64 = (s.dn[265][6] * ddt_scale);
        let eq162_e2046_d_n7: f64 = (s.dn[265][7] * ddt_scale);
        let eq162_e2046_d_n8: f64 = (s.dn[265][8] * ddt_scale);
        let eq162_e2046_d_n9: f64 = (s.dn[265][9] * ddt_scale);
        let eq162_e2046_d_n10: f64 = (s.dn[265][10] * ddt_scale);
        let eq162_e2046_d_n11: f64 = (s.dn[265][11] * ddt_scale);
        let eq162_e2046_d_n12: f64 = (s.dn[265][12] * ddt_scale);
        let eq162_e2046_d_n13: f64 = (s.dn[265][13] * ddt_scale);
        let eq162_e2046_d_n14: f64 = (s.dn[265][14] * ddt_scale);
        let eq162_e2046_d_n15: f64 = (s.dn[265][15] * ddt_scale);
        let eq162_e2046_d_n16: f64 = (s.dn[265][16] * ddt_scale);
        let eq162_e2046_d_n17: f64 = (s.dn[265][17] * ddt_scale);
        let eq162_e2046_d_n18: f64 = (s.dn[265][18] * ddt_scale);
        let eq162_e2046_d_n19: f64 = (s.dn[265][19] * ddt_scale);
        let eq162_e2046_d_n20: f64 = (s.dn[265][20] * ddt_scale);
        let eq162_e2046_d_n21: f64 = (s.dn[265][21] * ddt_scale);
        let eq162_e2046_d_n22: f64 = (s.dn[265][22] * ddt_scale);
        let eq162_e2047: f64 = (p.p7 * eq162_e2046);
        let eq162_e2047_d_n0: f64 = (p.p7 * eq162_e2046_d_n0);
        let eq162_e2047_d_n1: f64 = (p.p7 * eq162_e2046_d_n1);
        let eq162_e2047_d_n2: f64 = (p.p7 * eq162_e2046_d_n2);
        let eq162_e2047_d_n3: f64 = (p.p7 * eq162_e2046_d_n3);
        let eq162_e2047_d_n4: f64 = (p.p7 * eq162_e2046_d_n4);
        let eq162_e2047_d_n5: f64 = (p.p7 * eq162_e2046_d_n5);
        let eq162_e2047_d_n6: f64 = (p.p7 * eq162_e2046_d_n6);
        let eq162_e2047_d_n7: f64 = (p.p7 * eq162_e2046_d_n7);
        let eq162_e2047_d_n8: f64 = (p.p7 * eq162_e2046_d_n8);
        let eq162_e2047_d_n9: f64 = (p.p7 * eq162_e2046_d_n9);
        let eq162_e2047_d_n10: f64 = (p.p7 * eq162_e2046_d_n10);
        let eq162_e2047_d_n11: f64 = (p.p7 * eq162_e2046_d_n11);
        let eq162_e2047_d_n12: f64 = (p.p7 * eq162_e2046_d_n12);
        let eq162_e2047_d_n13: f64 = (p.p7 * eq162_e2046_d_n13);
        let eq162_e2047_d_n14: f64 = (p.p7 * eq162_e2046_d_n14);
        let eq162_e2047_d_n15: f64 = (p.p7 * eq162_e2046_d_n15);
        let eq162_e2047_d_n16: f64 = (p.p7 * eq162_e2046_d_n16);
        let eq162_e2047_d_n17: f64 = (p.p7 * eq162_e2046_d_n17);
        let eq162_e2047_d_n18: f64 = (p.p7 * eq162_e2046_d_n18);
        let eq162_e2047_d_n19: f64 = (p.p7 * eq162_e2046_d_n19);
        let eq162_e2047_d_n20: f64 = (p.p7 * eq162_e2046_d_n20);
        let eq162_e2047_d_n21: f64 = (p.p7 * eq162_e2046_d_n21);
        let eq162_e2047_d_n22: f64 = (p.p7 * eq162_e2046_d_n22);
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n10, eq162_e2047_d_n11, eq162_e2047_d_n12, eq162_e2047_d_n13, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e2049;
        let eq162_node_derivatives: [f64; 23] = [eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22];
        let eq162_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            multiplicity * (eq162_value),
            nodes,
            &eq162_node_derivatives,
            branches,
            &eq162_branch_derivatives,
            multiplicity,
        );
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq163_e2058: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 62, s.v[264]);
        let eq163_e2058_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq163_e2058_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq163_e2058_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq163_e2058_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq163_e2058_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq163_e2058_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq163_e2058_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq163_e2058_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq163_e2058_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq163_e2058_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq163_e2058_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq163_e2058_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq163_e2058_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq163_e2058_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq163_e2058_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq163_e2058_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq163_e2058_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq163_e2058_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq163_e2058_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq163_e2058_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq163_e2058_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq163_e2058_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq163_e2058_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq163_e2059: f64 = (p.p7 * eq163_e2058);
        let eq163_e2059_d_n0: f64 = (p.p7 * eq163_e2058_d_n0);
        let eq163_e2059_d_n1: f64 = (p.p7 * eq163_e2058_d_n1);
        let eq163_e2059_d_n2: f64 = (p.p7 * eq163_e2058_d_n2);
        let eq163_e2059_d_n3: f64 = (p.p7 * eq163_e2058_d_n3);
        let eq163_e2059_d_n4: f64 = (p.p7 * eq163_e2058_d_n4);
        let eq163_e2059_d_n5: f64 = (p.p7 * eq163_e2058_d_n5);
        let eq163_e2059_d_n6: f64 = (p.p7 * eq163_e2058_d_n6);
        let eq163_e2059_d_n7: f64 = (p.p7 * eq163_e2058_d_n7);
        let eq163_e2059_d_n8: f64 = (p.p7 * eq163_e2058_d_n8);
        let eq163_e2059_d_n9: f64 = (p.p7 * eq163_e2058_d_n9);
        let eq163_e2059_d_n10: f64 = (p.p7 * eq163_e2058_d_n10);
        let eq163_e2059_d_n11: f64 = (p.p7 * eq163_e2058_d_n11);
        let eq163_e2059_d_n12: f64 = (p.p7 * eq163_e2058_d_n12);
        let eq163_e2059_d_n13: f64 = (p.p7 * eq163_e2058_d_n13);
        let eq163_e2059_d_n14: f64 = (p.p7 * eq163_e2058_d_n14);
        let eq163_e2059_d_n15: f64 = (p.p7 * eq163_e2058_d_n15);
        let eq163_e2059_d_n16: f64 = (p.p7 * eq163_e2058_d_n16);
        let eq163_e2059_d_n17: f64 = (p.p7 * eq163_e2058_d_n17);
        let eq163_e2059_d_n18: f64 = (p.p7 * eq163_e2058_d_n18);
        let eq163_e2059_d_n19: f64 = (p.p7 * eq163_e2058_d_n19);
        let eq163_e2059_d_n20: f64 = (p.p7 * eq163_e2058_d_n20);
        let eq163_e2059_d_n21: f64 = (p.p7 * eq163_e2058_d_n21);
        let eq163_e2059_d_n22: f64 = (p.p7 * eq163_e2058_d_n22);
        (eq163_e2059, eq163_e2059_d_n0, eq163_e2059_d_n1, eq163_e2059_d_n2, eq163_e2059_d_n3, eq163_e2059_d_n4, eq163_e2059_d_n5, eq163_e2059_d_n6, eq163_e2059_d_n7, eq163_e2059_d_n8, eq163_e2059_d_n9, eq163_e2059_d_n10, eq163_e2059_d_n11, eq163_e2059_d_n12, eq163_e2059_d_n13, eq163_e2059_d_n14, eq163_e2059_d_n15, eq163_e2059_d_n16, eq163_e2059_d_n17, eq163_e2059_d_n18, eq163_e2059_d_n19, eq163_e2059_d_n20, eq163_e2059_d_n21, eq163_e2059_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e2061;
        let eq163_node_derivatives: [f64; 23] = [eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22];
        let eq163_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq163_value),
            nodes,
            &eq163_node_derivatives,
            branches,
            &eq163_branch_derivatives,
            multiplicity,
        );
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq164_e2070: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 63, s.v[264]);
        let eq164_e2070_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq164_e2070_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq164_e2070_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq164_e2070_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq164_e2070_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq164_e2070_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq164_e2070_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq164_e2070_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq164_e2070_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq164_e2070_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq164_e2070_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq164_e2070_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq164_e2070_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq164_e2070_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq164_e2070_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq164_e2070_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq164_e2070_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq164_e2070_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq164_e2070_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq164_e2070_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq164_e2070_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq164_e2070_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq164_e2070_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq164_e2071: f64 = (p.p7 * eq164_e2070);
        let eq164_e2071_d_n0: f64 = (p.p7 * eq164_e2070_d_n0);
        let eq164_e2071_d_n1: f64 = (p.p7 * eq164_e2070_d_n1);
        let eq164_e2071_d_n2: f64 = (p.p7 * eq164_e2070_d_n2);
        let eq164_e2071_d_n3: f64 = (p.p7 * eq164_e2070_d_n3);
        let eq164_e2071_d_n4: f64 = (p.p7 * eq164_e2070_d_n4);
        let eq164_e2071_d_n5: f64 = (p.p7 * eq164_e2070_d_n5);
        let eq164_e2071_d_n6: f64 = (p.p7 * eq164_e2070_d_n6);
        let eq164_e2071_d_n7: f64 = (p.p7 * eq164_e2070_d_n7);
        let eq164_e2071_d_n8: f64 = (p.p7 * eq164_e2070_d_n8);
        let eq164_e2071_d_n9: f64 = (p.p7 * eq164_e2070_d_n9);
        let eq164_e2071_d_n10: f64 = (p.p7 * eq164_e2070_d_n10);
        let eq164_e2071_d_n11: f64 = (p.p7 * eq164_e2070_d_n11);
        let eq164_e2071_d_n12: f64 = (p.p7 * eq164_e2070_d_n12);
        let eq164_e2071_d_n13: f64 = (p.p7 * eq164_e2070_d_n13);
        let eq164_e2071_d_n14: f64 = (p.p7 * eq164_e2070_d_n14);
        let eq164_e2071_d_n15: f64 = (p.p7 * eq164_e2070_d_n15);
        let eq164_e2071_d_n16: f64 = (p.p7 * eq164_e2070_d_n16);
        let eq164_e2071_d_n17: f64 = (p.p7 * eq164_e2070_d_n17);
        let eq164_e2071_d_n18: f64 = (p.p7 * eq164_e2070_d_n18);
        let eq164_e2071_d_n19: f64 = (p.p7 * eq164_e2070_d_n19);
        let eq164_e2071_d_n20: f64 = (p.p7 * eq164_e2070_d_n20);
        let eq164_e2071_d_n21: f64 = (p.p7 * eq164_e2070_d_n21);
        let eq164_e2071_d_n22: f64 = (p.p7 * eq164_e2070_d_n22);
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
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n10, eq164_e2073_d_n11, eq164_e2073_d_n12, eq164_e2073_d_n13, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e2075;
        let eq164_node_derivatives: [f64; 23] = [eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22];
        let eq164_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq164_value),
            nodes,
            &eq164_node_derivatives,
            branches,
            &eq164_branch_derivatives,
            multiplicity,
        );
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq165_e2085: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 64, s.v[264]);
        let eq165_e2085_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq165_e2085_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq165_e2085_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq165_e2085_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq165_e2085_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq165_e2085_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq165_e2085_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq165_e2085_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq165_e2085_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq165_e2085_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq165_e2085_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq165_e2085_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq165_e2085_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq165_e2085_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq165_e2085_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq165_e2085_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq165_e2085_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq165_e2085_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq165_e2085_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq165_e2085_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq165_e2085_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq165_e2085_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq165_e2085_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq165_e2086: f64 = (p.p7 * eq165_e2085);
        let eq165_e2086_d_n0: f64 = (p.p7 * eq165_e2085_d_n0);
        let eq165_e2086_d_n1: f64 = (p.p7 * eq165_e2085_d_n1);
        let eq165_e2086_d_n2: f64 = (p.p7 * eq165_e2085_d_n2);
        let eq165_e2086_d_n3: f64 = (p.p7 * eq165_e2085_d_n3);
        let eq165_e2086_d_n4: f64 = (p.p7 * eq165_e2085_d_n4);
        let eq165_e2086_d_n5: f64 = (p.p7 * eq165_e2085_d_n5);
        let eq165_e2086_d_n6: f64 = (p.p7 * eq165_e2085_d_n6);
        let eq165_e2086_d_n7: f64 = (p.p7 * eq165_e2085_d_n7);
        let eq165_e2086_d_n8: f64 = (p.p7 * eq165_e2085_d_n8);
        let eq165_e2086_d_n9: f64 = (p.p7 * eq165_e2085_d_n9);
        let eq165_e2086_d_n10: f64 = (p.p7 * eq165_e2085_d_n10);
        let eq165_e2086_d_n11: f64 = (p.p7 * eq165_e2085_d_n11);
        let eq165_e2086_d_n12: f64 = (p.p7 * eq165_e2085_d_n12);
        let eq165_e2086_d_n13: f64 = (p.p7 * eq165_e2085_d_n13);
        let eq165_e2086_d_n14: f64 = (p.p7 * eq165_e2085_d_n14);
        let eq165_e2086_d_n15: f64 = (p.p7 * eq165_e2085_d_n15);
        let eq165_e2086_d_n16: f64 = (p.p7 * eq165_e2085_d_n16);
        let eq165_e2086_d_n17: f64 = (p.p7 * eq165_e2085_d_n17);
        let eq165_e2086_d_n18: f64 = (p.p7 * eq165_e2085_d_n18);
        let eq165_e2086_d_n19: f64 = (p.p7 * eq165_e2085_d_n19);
        let eq165_e2086_d_n20: f64 = (p.p7 * eq165_e2085_d_n20);
        let eq165_e2086_d_n21: f64 = (p.p7 * eq165_e2085_d_n21);
        let eq165_e2086_d_n22: f64 = (p.p7 * eq165_e2085_d_n22);
        (eq165_e2086, eq165_e2086_d_n0, eq165_e2086_d_n1, eq165_e2086_d_n2, eq165_e2086_d_n3, eq165_e2086_d_n4, eq165_e2086_d_n5, eq165_e2086_d_n6, eq165_e2086_d_n7, eq165_e2086_d_n8, eq165_e2086_d_n9, eq165_e2086_d_n10, eq165_e2086_d_n11, eq165_e2086_d_n12, eq165_e2086_d_n13, eq165_e2086_d_n14, eq165_e2086_d_n15, eq165_e2086_d_n16, eq165_e2086_d_n17, eq165_e2086_d_n18, eq165_e2086_d_n19, eq165_e2086_d_n20, eq165_e2086_d_n21, eq165_e2086_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_value: f64 = eq165_e2088;
        let eq165_node_derivatives: [f64; 23] = [eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22];
        let eq165_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq165_value),
            nodes,
            &eq165_node_derivatives,
            branches,
            &eq165_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_17(
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
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq166_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 65, s.v[264]);
        let eq166_e2098_d_n0: f64 = (s.dn[264][0] * ddt_scale);
        let eq166_e2098_d_n1: f64 = (s.dn[264][1] * ddt_scale);
        let eq166_e2098_d_n2: f64 = (s.dn[264][2] * ddt_scale);
        let eq166_e2098_d_n3: f64 = (s.dn[264][3] * ddt_scale);
        let eq166_e2098_d_n4: f64 = (s.dn[264][4] * ddt_scale);
        let eq166_e2098_d_n5: f64 = (s.dn[264][5] * ddt_scale);
        let eq166_e2098_d_n6: f64 = (s.dn[264][6] * ddt_scale);
        let eq166_e2098_d_n7: f64 = (s.dn[264][7] * ddt_scale);
        let eq166_e2098_d_n8: f64 = (s.dn[264][8] * ddt_scale);
        let eq166_e2098_d_n9: f64 = (s.dn[264][9] * ddt_scale);
        let eq166_e2098_d_n10: f64 = (s.dn[264][10] * ddt_scale);
        let eq166_e2098_d_n11: f64 = (s.dn[264][11] * ddt_scale);
        let eq166_e2098_d_n12: f64 = (s.dn[264][12] * ddt_scale);
        let eq166_e2098_d_n13: f64 = (s.dn[264][13] * ddt_scale);
        let eq166_e2098_d_n14: f64 = (s.dn[264][14] * ddt_scale);
        let eq166_e2098_d_n15: f64 = (s.dn[264][15] * ddt_scale);
        let eq166_e2098_d_n16: f64 = (s.dn[264][16] * ddt_scale);
        let eq166_e2098_d_n17: f64 = (s.dn[264][17] * ddt_scale);
        let eq166_e2098_d_n18: f64 = (s.dn[264][18] * ddt_scale);
        let eq166_e2098_d_n19: f64 = (s.dn[264][19] * ddt_scale);
        let eq166_e2098_d_n20: f64 = (s.dn[264][20] * ddt_scale);
        let eq166_e2098_d_n21: f64 = (s.dn[264][21] * ddt_scale);
        let eq166_e2098_d_n22: f64 = (s.dn[264][22] * ddt_scale);
        let eq166_e2099: f64 = (p.p7 * eq166_e2098);
        let eq166_e2099_d_n0: f64 = (p.p7 * eq166_e2098_d_n0);
        let eq166_e2099_d_n1: f64 = (p.p7 * eq166_e2098_d_n1);
        let eq166_e2099_d_n2: f64 = (p.p7 * eq166_e2098_d_n2);
        let eq166_e2099_d_n3: f64 = (p.p7 * eq166_e2098_d_n3);
        let eq166_e2099_d_n4: f64 = (p.p7 * eq166_e2098_d_n4);
        let eq166_e2099_d_n5: f64 = (p.p7 * eq166_e2098_d_n5);
        let eq166_e2099_d_n6: f64 = (p.p7 * eq166_e2098_d_n6);
        let eq166_e2099_d_n7: f64 = (p.p7 * eq166_e2098_d_n7);
        let eq166_e2099_d_n8: f64 = (p.p7 * eq166_e2098_d_n8);
        let eq166_e2099_d_n9: f64 = (p.p7 * eq166_e2098_d_n9);
        let eq166_e2099_d_n10: f64 = (p.p7 * eq166_e2098_d_n10);
        let eq166_e2099_d_n11: f64 = (p.p7 * eq166_e2098_d_n11);
        let eq166_e2099_d_n12: f64 = (p.p7 * eq166_e2098_d_n12);
        let eq166_e2099_d_n13: f64 = (p.p7 * eq166_e2098_d_n13);
        let eq166_e2099_d_n14: f64 = (p.p7 * eq166_e2098_d_n14);
        let eq166_e2099_d_n15: f64 = (p.p7 * eq166_e2098_d_n15);
        let eq166_e2099_d_n16: f64 = (p.p7 * eq166_e2098_d_n16);
        let eq166_e2099_d_n17: f64 = (p.p7 * eq166_e2098_d_n17);
        let eq166_e2099_d_n18: f64 = (p.p7 * eq166_e2098_d_n18);
        let eq166_e2099_d_n19: f64 = (p.p7 * eq166_e2098_d_n19);
        let eq166_e2099_d_n20: f64 = (p.p7 * eq166_e2098_d_n20);
        let eq166_e2099_d_n21: f64 = (p.p7 * eq166_e2098_d_n21);
        let eq166_e2099_d_n22: f64 = (p.p7 * eq166_e2098_d_n22);
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
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n10, eq166_e2101_d_n11, eq166_e2101_d_n12, eq166_e2101_d_n13, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e2103;
        let eq166_node_derivatives: [f64; 23] = [eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22];
        let eq166_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq166_value),
            nodes,
            &eq166_node_derivatives,
            branches,
            &eq166_branch_derivatives,
            multiplicity,
        );
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22,) = {
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
        let eq167_e2112: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 66, eq167_e2111);
        let eq167_e2112_d_n0: f64 = (eq167_e2111_d_n0 * ddt_scale);
        let eq167_e2112_d_n1: f64 = (eq167_e2111_d_n1 * ddt_scale);
        let eq167_e2112_d_n2: f64 = (eq167_e2111_d_n2 * ddt_scale);
        let eq167_e2112_d_n3: f64 = (eq167_e2111_d_n3 * ddt_scale);
        let eq167_e2112_d_n4: f64 = (eq167_e2111_d_n4 * ddt_scale);
        let eq167_e2112_d_n5: f64 = (eq167_e2111_d_n5 * ddt_scale);
        let eq167_e2112_d_n6: f64 = (eq167_e2111_d_n6 * ddt_scale);
        let eq167_e2112_d_n7: f64 = (eq167_e2111_d_n7 * ddt_scale);
        let eq167_e2112_d_n8: f64 = (eq167_e2111_d_n8 * ddt_scale);
        let eq167_e2112_d_n9: f64 = (eq167_e2111_d_n9 * ddt_scale);
        let eq167_e2112_d_n10: f64 = (eq167_e2111_d_n10 * ddt_scale);
        let eq167_e2112_d_n11: f64 = (eq167_e2111_d_n11 * ddt_scale);
        let eq167_e2112_d_n12: f64 = (eq167_e2111_d_n12 * ddt_scale);
        let eq167_e2112_d_n13: f64 = (eq167_e2111_d_n13 * ddt_scale);
        let eq167_e2112_d_n14: f64 = (eq167_e2111_d_n14 * ddt_scale);
        let eq167_e2112_d_n15: f64 = (eq167_e2111_d_n15 * ddt_scale);
        let eq167_e2112_d_n16: f64 = (eq167_e2111_d_n16 * ddt_scale);
        let eq167_e2112_d_n17: f64 = (eq167_e2111_d_n17 * ddt_scale);
        let eq167_e2112_d_n18: f64 = (eq167_e2111_d_n18 * ddt_scale);
        let eq167_e2112_d_n19: f64 = (eq167_e2111_d_n19 * ddt_scale);
        let eq167_e2112_d_n20: f64 = (eq167_e2111_d_n20 * ddt_scale);
        let eq167_e2112_d_n21: f64 = (eq167_e2111_d_n21 * ddt_scale);
        let eq167_e2112_d_n22: f64 = (eq167_e2111_d_n22 * ddt_scale);
        let eq167_e2113: f64 = (p.p7 * eq167_e2112);
        let eq167_e2113_d_n0: f64 = (p.p7 * eq167_e2112_d_n0);
        let eq167_e2113_d_n1: f64 = (p.p7 * eq167_e2112_d_n1);
        let eq167_e2113_d_n2: f64 = (p.p7 * eq167_e2112_d_n2);
        let eq167_e2113_d_n3: f64 = (p.p7 * eq167_e2112_d_n3);
        let eq167_e2113_d_n4: f64 = (p.p7 * eq167_e2112_d_n4);
        let eq167_e2113_d_n5: f64 = (p.p7 * eq167_e2112_d_n5);
        let eq167_e2113_d_n6: f64 = (p.p7 * eq167_e2112_d_n6);
        let eq167_e2113_d_n7: f64 = (p.p7 * eq167_e2112_d_n7);
        let eq167_e2113_d_n8: f64 = (p.p7 * eq167_e2112_d_n8);
        let eq167_e2113_d_n9: f64 = (p.p7 * eq167_e2112_d_n9);
        let eq167_e2113_d_n10: f64 = (p.p7 * eq167_e2112_d_n10);
        let eq167_e2113_d_n11: f64 = (p.p7 * eq167_e2112_d_n11);
        let eq167_e2113_d_n12: f64 = (p.p7 * eq167_e2112_d_n12);
        let eq167_e2113_d_n13: f64 = (p.p7 * eq167_e2112_d_n13);
        let eq167_e2113_d_n14: f64 = (p.p7 * eq167_e2112_d_n14);
        let eq167_e2113_d_n15: f64 = (p.p7 * eq167_e2112_d_n15);
        let eq167_e2113_d_n16: f64 = (p.p7 * eq167_e2112_d_n16);
        let eq167_e2113_d_n17: f64 = (p.p7 * eq167_e2112_d_n17);
        let eq167_e2113_d_n18: f64 = (p.p7 * eq167_e2112_d_n18);
        let eq167_e2113_d_n19: f64 = (p.p7 * eq167_e2112_d_n19);
        let eq167_e2113_d_n20: f64 = (p.p7 * eq167_e2112_d_n20);
        let eq167_e2113_d_n21: f64 = (p.p7 * eq167_e2112_d_n21);
        let eq167_e2113_d_n22: f64 = (p.p7 * eq167_e2112_d_n22);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n10, eq167_e2113_d_n11, eq167_e2113_d_n12, eq167_e2113_d_n13, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_value: f64 = eq167_e2115;
        let eq167_node_derivatives: [f64; 23] = [eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22];
        let eq167_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            multiplicity * (eq167_value),
            nodes,
            &eq167_node_derivatives,
            branches,
            &eq167_branch_derivatives,
            multiplicity,
        );
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22,) = {
    if (s.b[590] && s.b[591]) {
        let eq168_e2121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 67, s.v[277]);
        let eq168_e2121_d_n0: f64 = (s.dn[277][0] * ddt_scale);
        let eq168_e2121_d_n1: f64 = (s.dn[277][1] * ddt_scale);
        let eq168_e2121_d_n2: f64 = (s.dn[277][2] * ddt_scale);
        let eq168_e2121_d_n3: f64 = (s.dn[277][3] * ddt_scale);
        let eq168_e2121_d_n4: f64 = (s.dn[277][4] * ddt_scale);
        let eq168_e2121_d_n5: f64 = (s.dn[277][5] * ddt_scale);
        let eq168_e2121_d_n6: f64 = (s.dn[277][6] * ddt_scale);
        let eq168_e2121_d_n7: f64 = (s.dn[277][7] * ddt_scale);
        let eq168_e2121_d_n8: f64 = (s.dn[277][8] * ddt_scale);
        let eq168_e2121_d_n9: f64 = (s.dn[277][9] * ddt_scale);
        let eq168_e2121_d_n10: f64 = (s.dn[277][10] * ddt_scale);
        let eq168_e2121_d_n11: f64 = (s.dn[277][11] * ddt_scale);
        let eq168_e2121_d_n12: f64 = (s.dn[277][12] * ddt_scale);
        let eq168_e2121_d_n13: f64 = (s.dn[277][13] * ddt_scale);
        let eq168_e2121_d_n14: f64 = (s.dn[277][14] * ddt_scale);
        let eq168_e2121_d_n15: f64 = (s.dn[277][15] * ddt_scale);
        let eq168_e2121_d_n16: f64 = (s.dn[277][16] * ddt_scale);
        let eq168_e2121_d_n17: f64 = (s.dn[277][17] * ddt_scale);
        let eq168_e2121_d_n18: f64 = (s.dn[277][18] * ddt_scale);
        let eq168_e2121_d_n19: f64 = (s.dn[277][19] * ddt_scale);
        let eq168_e2121_d_n20: f64 = (s.dn[277][20] * ddt_scale);
        let eq168_e2121_d_n21: f64 = (s.dn[277][21] * ddt_scale);
        let eq168_e2121_d_n22: f64 = (s.dn[277][22] * ddt_scale);
        let eq168_e2122: f64 = (p.p7 * eq168_e2121);
        let eq168_e2122_d_n0: f64 = (p.p7 * eq168_e2121_d_n0);
        let eq168_e2122_d_n1: f64 = (p.p7 * eq168_e2121_d_n1);
        let eq168_e2122_d_n2: f64 = (p.p7 * eq168_e2121_d_n2);
        let eq168_e2122_d_n3: f64 = (p.p7 * eq168_e2121_d_n3);
        let eq168_e2122_d_n4: f64 = (p.p7 * eq168_e2121_d_n4);
        let eq168_e2122_d_n5: f64 = (p.p7 * eq168_e2121_d_n5);
        let eq168_e2122_d_n6: f64 = (p.p7 * eq168_e2121_d_n6);
        let eq168_e2122_d_n7: f64 = (p.p7 * eq168_e2121_d_n7);
        let eq168_e2122_d_n8: f64 = (p.p7 * eq168_e2121_d_n8);
        let eq168_e2122_d_n9: f64 = (p.p7 * eq168_e2121_d_n9);
        let eq168_e2122_d_n10: f64 = (p.p7 * eq168_e2121_d_n10);
        let eq168_e2122_d_n11: f64 = (p.p7 * eq168_e2121_d_n11);
        let eq168_e2122_d_n12: f64 = (p.p7 * eq168_e2121_d_n12);
        let eq168_e2122_d_n13: f64 = (p.p7 * eq168_e2121_d_n13);
        let eq168_e2122_d_n14: f64 = (p.p7 * eq168_e2121_d_n14);
        let eq168_e2122_d_n15: f64 = (p.p7 * eq168_e2121_d_n15);
        let eq168_e2122_d_n16: f64 = (p.p7 * eq168_e2121_d_n16);
        let eq168_e2122_d_n17: f64 = (p.p7 * eq168_e2121_d_n17);
        let eq168_e2122_d_n18: f64 = (p.p7 * eq168_e2121_d_n18);
        let eq168_e2122_d_n19: f64 = (p.p7 * eq168_e2121_d_n19);
        let eq168_e2122_d_n20: f64 = (p.p7 * eq168_e2121_d_n20);
        let eq168_e2122_d_n21: f64 = (p.p7 * eq168_e2121_d_n21);
        let eq168_e2122_d_n22: f64 = (p.p7 * eq168_e2121_d_n22);
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n10, eq168_e2122_d_n11, eq168_e2122_d_n12, eq168_e2122_d_n13, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_value: f64 = eq168_e2124;
        let eq168_node_derivatives: [f64; 23] = [eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22];
        let eq168_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            multiplicity * (eq168_value),
            nodes,
            &eq168_node_derivatives,
            branches,
            &eq168_branch_derivatives,
            multiplicity,
        );
        let (eq169_e2135, eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22,) = {
    if ((s.b[590] && s.b[591]) && s.b[592]) {
        let eq169_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 68, s.v[276]);
        let eq169_e2132_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq169_e2132_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq169_e2132_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq169_e2132_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq169_e2132_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq169_e2132_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq169_e2132_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq169_e2132_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq169_e2132_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq169_e2132_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq169_e2132_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq169_e2132_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq169_e2132_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq169_e2132_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq169_e2132_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq169_e2132_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq169_e2132_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq169_e2132_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq169_e2132_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq169_e2132_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq169_e2132_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq169_e2132_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq169_e2132_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq169_e2133: f64 = (p.p7 * eq169_e2132);
        let eq169_e2133_d_n0: f64 = (p.p7 * eq169_e2132_d_n0);
        let eq169_e2133_d_n1: f64 = (p.p7 * eq169_e2132_d_n1);
        let eq169_e2133_d_n2: f64 = (p.p7 * eq169_e2132_d_n2);
        let eq169_e2133_d_n3: f64 = (p.p7 * eq169_e2132_d_n3);
        let eq169_e2133_d_n4: f64 = (p.p7 * eq169_e2132_d_n4);
        let eq169_e2133_d_n5: f64 = (p.p7 * eq169_e2132_d_n5);
        let eq169_e2133_d_n6: f64 = (p.p7 * eq169_e2132_d_n6);
        let eq169_e2133_d_n7: f64 = (p.p7 * eq169_e2132_d_n7);
        let eq169_e2133_d_n8: f64 = (p.p7 * eq169_e2132_d_n8);
        let eq169_e2133_d_n9: f64 = (p.p7 * eq169_e2132_d_n9);
        let eq169_e2133_d_n10: f64 = (p.p7 * eq169_e2132_d_n10);
        let eq169_e2133_d_n11: f64 = (p.p7 * eq169_e2132_d_n11);
        let eq169_e2133_d_n12: f64 = (p.p7 * eq169_e2132_d_n12);
        let eq169_e2133_d_n13: f64 = (p.p7 * eq169_e2132_d_n13);
        let eq169_e2133_d_n14: f64 = (p.p7 * eq169_e2132_d_n14);
        let eq169_e2133_d_n15: f64 = (p.p7 * eq169_e2132_d_n15);
        let eq169_e2133_d_n16: f64 = (p.p7 * eq169_e2132_d_n16);
        let eq169_e2133_d_n17: f64 = (p.p7 * eq169_e2132_d_n17);
        let eq169_e2133_d_n18: f64 = (p.p7 * eq169_e2132_d_n18);
        let eq169_e2133_d_n19: f64 = (p.p7 * eq169_e2132_d_n19);
        let eq169_e2133_d_n20: f64 = (p.p7 * eq169_e2132_d_n20);
        let eq169_e2133_d_n21: f64 = (p.p7 * eq169_e2132_d_n21);
        let eq169_e2133_d_n22: f64 = (p.p7 * eq169_e2132_d_n22);
        (eq169_e2133, eq169_e2133_d_n0, eq169_e2133_d_n1, eq169_e2133_d_n2, eq169_e2133_d_n3, eq169_e2133_d_n4, eq169_e2133_d_n5, eq169_e2133_d_n6, eq169_e2133_d_n7, eq169_e2133_d_n8, eq169_e2133_d_n9, eq169_e2133_d_n10, eq169_e2133_d_n11, eq169_e2133_d_n12, eq169_e2133_d_n13, eq169_e2133_d_n14, eq169_e2133_d_n15, eq169_e2133_d_n16, eq169_e2133_d_n17, eq169_e2133_d_n18, eq169_e2133_d_n19, eq169_e2133_d_n20, eq169_e2133_d_n21, eq169_e2133_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq169_value: f64 = eq169_e2135;
        let eq169_node_derivatives: [f64; 23] = [eq169_e2135_d_n0, eq169_e2135_d_n1, eq169_e2135_d_n2, eq169_e2135_d_n3, eq169_e2135_d_n4, eq169_e2135_d_n5, eq169_e2135_d_n6, eq169_e2135_d_n7, eq169_e2135_d_n8, eq169_e2135_d_n9, eq169_e2135_d_n10, eq169_e2135_d_n11, eq169_e2135_d_n12, eq169_e2135_d_n13, eq169_e2135_d_n14, eq169_e2135_d_n15, eq169_e2135_d_n16, eq169_e2135_d_n17, eq169_e2135_d_n18, eq169_e2135_d_n19, eq169_e2135_d_n20, eq169_e2135_d_n21, eq169_e2135_d_n22];
        let eq169_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            multiplicity * (eq169_value),
            nodes,
            &eq169_node_derivatives,
            branches,
            &eq169_branch_derivatives,
            multiplicity,
        );
        let (eq170_e2148, eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22,) = {
    if ((s.b[590] && s.b[591]) && s.b[592]) {
        let eq170_e2143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 69, s.v[276]);
        let eq170_e2143_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq170_e2143_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq170_e2143_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq170_e2143_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq170_e2143_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq170_e2143_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq170_e2143_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq170_e2143_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq170_e2143_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq170_e2143_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq170_e2143_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq170_e2143_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq170_e2143_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq170_e2143_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq170_e2143_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq170_e2143_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq170_e2143_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq170_e2143_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq170_e2143_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq170_e2143_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq170_e2143_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq170_e2143_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq170_e2143_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq170_e2144: f64 = (p.p7 * eq170_e2143);
        let eq170_e2144_d_n0: f64 = (p.p7 * eq170_e2143_d_n0);
        let eq170_e2144_d_n1: f64 = (p.p7 * eq170_e2143_d_n1);
        let eq170_e2144_d_n2: f64 = (p.p7 * eq170_e2143_d_n2);
        let eq170_e2144_d_n3: f64 = (p.p7 * eq170_e2143_d_n3);
        let eq170_e2144_d_n4: f64 = (p.p7 * eq170_e2143_d_n4);
        let eq170_e2144_d_n5: f64 = (p.p7 * eq170_e2143_d_n5);
        let eq170_e2144_d_n6: f64 = (p.p7 * eq170_e2143_d_n6);
        let eq170_e2144_d_n7: f64 = (p.p7 * eq170_e2143_d_n7);
        let eq170_e2144_d_n8: f64 = (p.p7 * eq170_e2143_d_n8);
        let eq170_e2144_d_n9: f64 = (p.p7 * eq170_e2143_d_n9);
        let eq170_e2144_d_n10: f64 = (p.p7 * eq170_e2143_d_n10);
        let eq170_e2144_d_n11: f64 = (p.p7 * eq170_e2143_d_n11);
        let eq170_e2144_d_n12: f64 = (p.p7 * eq170_e2143_d_n12);
        let eq170_e2144_d_n13: f64 = (p.p7 * eq170_e2143_d_n13);
        let eq170_e2144_d_n14: f64 = (p.p7 * eq170_e2143_d_n14);
        let eq170_e2144_d_n15: f64 = (p.p7 * eq170_e2143_d_n15);
        let eq170_e2144_d_n16: f64 = (p.p7 * eq170_e2143_d_n16);
        let eq170_e2144_d_n17: f64 = (p.p7 * eq170_e2143_d_n17);
        let eq170_e2144_d_n18: f64 = (p.p7 * eq170_e2143_d_n18);
        let eq170_e2144_d_n19: f64 = (p.p7 * eq170_e2143_d_n19);
        let eq170_e2144_d_n20: f64 = (p.p7 * eq170_e2143_d_n20);
        let eq170_e2144_d_n21: f64 = (p.p7 * eq170_e2143_d_n21);
        let eq170_e2144_d_n22: f64 = (p.p7 * eq170_e2143_d_n22);
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
        (eq170_e2146, eq170_e2146_d_n0, eq170_e2146_d_n1, eq170_e2146_d_n2, eq170_e2146_d_n3, eq170_e2146_d_n4, eq170_e2146_d_n5, eq170_e2146_d_n6, eq170_e2146_d_n7, eq170_e2146_d_n8, eq170_e2146_d_n9, eq170_e2146_d_n10, eq170_e2146_d_n11, eq170_e2146_d_n12, eq170_e2146_d_n13, eq170_e2146_d_n14, eq170_e2146_d_n15, eq170_e2146_d_n16, eq170_e2146_d_n17, eq170_e2146_d_n18, eq170_e2146_d_n19, eq170_e2146_d_n20, eq170_e2146_d_n21, eq170_e2146_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq170_value: f64 = eq170_e2148;
        let eq170_node_derivatives: [f64; 23] = [eq170_e2148_d_n0, eq170_e2148_d_n1, eq170_e2148_d_n2, eq170_e2148_d_n3, eq170_e2148_d_n4, eq170_e2148_d_n5, eq170_e2148_d_n6, eq170_e2148_d_n7, eq170_e2148_d_n8, eq170_e2148_d_n9, eq170_e2148_d_n10, eq170_e2148_d_n11, eq170_e2148_d_n12, eq170_e2148_d_n13, eq170_e2148_d_n14, eq170_e2148_d_n15, eq170_e2148_d_n16, eq170_e2148_d_n17, eq170_e2148_d_n18, eq170_e2148_d_n19, eq170_e2148_d_n20, eq170_e2148_d_n21, eq170_e2148_d_n22];
        let eq170_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            multiplicity * (eq170_value),
            nodes,
            &eq170_node_derivatives,
            branches,
            &eq170_branch_derivatives,
            multiplicity,
        );
        let (eq171_e2160, eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22,) = {
    if ((s.b[590] && s.b[591]) && (!s.b[592])) {
        let eq171_e2157: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 70, s.v[276]);
        let eq171_e2157_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq171_e2157_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq171_e2157_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq171_e2157_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq171_e2157_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq171_e2157_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq171_e2157_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq171_e2157_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq171_e2157_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq171_e2157_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq171_e2157_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq171_e2157_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq171_e2157_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq171_e2157_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq171_e2157_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq171_e2157_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq171_e2157_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq171_e2157_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq171_e2157_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq171_e2157_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq171_e2157_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq171_e2157_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq171_e2157_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq171_e2158: f64 = (p.p7 * eq171_e2157);
        let eq171_e2158_d_n0: f64 = (p.p7 * eq171_e2157_d_n0);
        let eq171_e2158_d_n1: f64 = (p.p7 * eq171_e2157_d_n1);
        let eq171_e2158_d_n2: f64 = (p.p7 * eq171_e2157_d_n2);
        let eq171_e2158_d_n3: f64 = (p.p7 * eq171_e2157_d_n3);
        let eq171_e2158_d_n4: f64 = (p.p7 * eq171_e2157_d_n4);
        let eq171_e2158_d_n5: f64 = (p.p7 * eq171_e2157_d_n5);
        let eq171_e2158_d_n6: f64 = (p.p7 * eq171_e2157_d_n6);
        let eq171_e2158_d_n7: f64 = (p.p7 * eq171_e2157_d_n7);
        let eq171_e2158_d_n8: f64 = (p.p7 * eq171_e2157_d_n8);
        let eq171_e2158_d_n9: f64 = (p.p7 * eq171_e2157_d_n9);
        let eq171_e2158_d_n10: f64 = (p.p7 * eq171_e2157_d_n10);
        let eq171_e2158_d_n11: f64 = (p.p7 * eq171_e2157_d_n11);
        let eq171_e2158_d_n12: f64 = (p.p7 * eq171_e2157_d_n12);
        let eq171_e2158_d_n13: f64 = (p.p7 * eq171_e2157_d_n13);
        let eq171_e2158_d_n14: f64 = (p.p7 * eq171_e2157_d_n14);
        let eq171_e2158_d_n15: f64 = (p.p7 * eq171_e2157_d_n15);
        let eq171_e2158_d_n16: f64 = (p.p7 * eq171_e2157_d_n16);
        let eq171_e2158_d_n17: f64 = (p.p7 * eq171_e2157_d_n17);
        let eq171_e2158_d_n18: f64 = (p.p7 * eq171_e2157_d_n18);
        let eq171_e2158_d_n19: f64 = (p.p7 * eq171_e2157_d_n19);
        let eq171_e2158_d_n20: f64 = (p.p7 * eq171_e2157_d_n20);
        let eq171_e2158_d_n21: f64 = (p.p7 * eq171_e2157_d_n21);
        let eq171_e2158_d_n22: f64 = (p.p7 * eq171_e2157_d_n22);
        (eq171_e2158, eq171_e2158_d_n0, eq171_e2158_d_n1, eq171_e2158_d_n2, eq171_e2158_d_n3, eq171_e2158_d_n4, eq171_e2158_d_n5, eq171_e2158_d_n6, eq171_e2158_d_n7, eq171_e2158_d_n8, eq171_e2158_d_n9, eq171_e2158_d_n10, eq171_e2158_d_n11, eq171_e2158_d_n12, eq171_e2158_d_n13, eq171_e2158_d_n14, eq171_e2158_d_n15, eq171_e2158_d_n16, eq171_e2158_d_n17, eq171_e2158_d_n18, eq171_e2158_d_n19, eq171_e2158_d_n20, eq171_e2158_d_n21, eq171_e2158_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq171_value: f64 = eq171_e2160;
        let eq171_node_derivatives: [f64; 23] = [eq171_e2160_d_n0, eq171_e2160_d_n1, eq171_e2160_d_n2, eq171_e2160_d_n3, eq171_e2160_d_n4, eq171_e2160_d_n5, eq171_e2160_d_n6, eq171_e2160_d_n7, eq171_e2160_d_n8, eq171_e2160_d_n9, eq171_e2160_d_n10, eq171_e2160_d_n11, eq171_e2160_d_n12, eq171_e2160_d_n13, eq171_e2160_d_n14, eq171_e2160_d_n15, eq171_e2160_d_n16, eq171_e2160_d_n17, eq171_e2160_d_n18, eq171_e2160_d_n19, eq171_e2160_d_n20, eq171_e2160_d_n21, eq171_e2160_d_n22];
        let eq171_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[16]),
            multiplicity * (eq171_value),
            nodes,
            &eq171_node_derivatives,
            branches,
            &eq171_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_18(
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
        let (eq172_e2174, eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22,) = {
    if ((s.b[590] && s.b[591]) && (!s.b[592])) {
        let eq172_e2169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 71, s.v[276]);
        let eq172_e2169_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq172_e2169_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq172_e2169_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq172_e2169_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq172_e2169_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq172_e2169_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq172_e2169_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq172_e2169_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq172_e2169_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq172_e2169_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq172_e2169_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq172_e2169_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq172_e2169_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq172_e2169_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq172_e2169_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq172_e2169_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq172_e2169_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq172_e2169_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq172_e2169_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq172_e2169_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq172_e2169_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq172_e2169_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq172_e2169_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq172_e2170: f64 = (p.p7 * eq172_e2169);
        let eq172_e2170_d_n0: f64 = (p.p7 * eq172_e2169_d_n0);
        let eq172_e2170_d_n1: f64 = (p.p7 * eq172_e2169_d_n1);
        let eq172_e2170_d_n2: f64 = (p.p7 * eq172_e2169_d_n2);
        let eq172_e2170_d_n3: f64 = (p.p7 * eq172_e2169_d_n3);
        let eq172_e2170_d_n4: f64 = (p.p7 * eq172_e2169_d_n4);
        let eq172_e2170_d_n5: f64 = (p.p7 * eq172_e2169_d_n5);
        let eq172_e2170_d_n6: f64 = (p.p7 * eq172_e2169_d_n6);
        let eq172_e2170_d_n7: f64 = (p.p7 * eq172_e2169_d_n7);
        let eq172_e2170_d_n8: f64 = (p.p7 * eq172_e2169_d_n8);
        let eq172_e2170_d_n9: f64 = (p.p7 * eq172_e2169_d_n9);
        let eq172_e2170_d_n10: f64 = (p.p7 * eq172_e2169_d_n10);
        let eq172_e2170_d_n11: f64 = (p.p7 * eq172_e2169_d_n11);
        let eq172_e2170_d_n12: f64 = (p.p7 * eq172_e2169_d_n12);
        let eq172_e2170_d_n13: f64 = (p.p7 * eq172_e2169_d_n13);
        let eq172_e2170_d_n14: f64 = (p.p7 * eq172_e2169_d_n14);
        let eq172_e2170_d_n15: f64 = (p.p7 * eq172_e2169_d_n15);
        let eq172_e2170_d_n16: f64 = (p.p7 * eq172_e2169_d_n16);
        let eq172_e2170_d_n17: f64 = (p.p7 * eq172_e2169_d_n17);
        let eq172_e2170_d_n18: f64 = (p.p7 * eq172_e2169_d_n18);
        let eq172_e2170_d_n19: f64 = (p.p7 * eq172_e2169_d_n19);
        let eq172_e2170_d_n20: f64 = (p.p7 * eq172_e2169_d_n20);
        let eq172_e2170_d_n21: f64 = (p.p7 * eq172_e2169_d_n21);
        let eq172_e2170_d_n22: f64 = (p.p7 * eq172_e2169_d_n22);
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
        (eq172_e2172, eq172_e2172_d_n0, eq172_e2172_d_n1, eq172_e2172_d_n2, eq172_e2172_d_n3, eq172_e2172_d_n4, eq172_e2172_d_n5, eq172_e2172_d_n6, eq172_e2172_d_n7, eq172_e2172_d_n8, eq172_e2172_d_n9, eq172_e2172_d_n10, eq172_e2172_d_n11, eq172_e2172_d_n12, eq172_e2172_d_n13, eq172_e2172_d_n14, eq172_e2172_d_n15, eq172_e2172_d_n16, eq172_e2172_d_n17, eq172_e2172_d_n18, eq172_e2172_d_n19, eq172_e2172_d_n20, eq172_e2172_d_n21, eq172_e2172_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq172_value: f64 = eq172_e2174;
        let eq172_node_derivatives: [f64; 23] = [eq172_e2174_d_n0, eq172_e2174_d_n1, eq172_e2174_d_n2, eq172_e2174_d_n3, eq172_e2174_d_n4, eq172_e2174_d_n5, eq172_e2174_d_n6, eq172_e2174_d_n7, eq172_e2174_d_n8, eq172_e2174_d_n9, eq172_e2174_d_n10, eq172_e2174_d_n11, eq172_e2174_d_n12, eq172_e2174_d_n13, eq172_e2174_d_n14, eq172_e2174_d_n15, eq172_e2174_d_n16, eq172_e2174_d_n17, eq172_e2174_d_n18, eq172_e2174_d_n19, eq172_e2174_d_n20, eq172_e2174_d_n21, eq172_e2174_d_n22];
        let eq172_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[16]),
            multiplicity * (eq172_value),
            nodes,
            &eq172_node_derivatives,
            branches,
            &eq172_branch_derivatives,
            multiplicity,
        );
        let (eq173_e2185, eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22,) = {
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
        let eq173_e2182: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 72, eq173_e2181);
        let eq173_e2182_d_n0: f64 = (eq173_e2181_d_n0 * ddt_scale);
        let eq173_e2182_d_n1: f64 = (eq173_e2181_d_n1 * ddt_scale);
        let eq173_e2182_d_n2: f64 = (eq173_e2181_d_n2 * ddt_scale);
        let eq173_e2182_d_n3: f64 = (eq173_e2181_d_n3 * ddt_scale);
        let eq173_e2182_d_n4: f64 = (eq173_e2181_d_n4 * ddt_scale);
        let eq173_e2182_d_n5: f64 = (eq173_e2181_d_n5 * ddt_scale);
        let eq173_e2182_d_n6: f64 = (eq173_e2181_d_n6 * ddt_scale);
        let eq173_e2182_d_n7: f64 = (eq173_e2181_d_n7 * ddt_scale);
        let eq173_e2182_d_n8: f64 = (eq173_e2181_d_n8 * ddt_scale);
        let eq173_e2182_d_n9: f64 = (eq173_e2181_d_n9 * ddt_scale);
        let eq173_e2182_d_n10: f64 = (eq173_e2181_d_n10 * ddt_scale);
        let eq173_e2182_d_n11: f64 = (eq173_e2181_d_n11 * ddt_scale);
        let eq173_e2182_d_n12: f64 = (eq173_e2181_d_n12 * ddt_scale);
        let eq173_e2182_d_n13: f64 = (eq173_e2181_d_n13 * ddt_scale);
        let eq173_e2182_d_n14: f64 = (eq173_e2181_d_n14 * ddt_scale);
        let eq173_e2182_d_n15: f64 = (eq173_e2181_d_n15 * ddt_scale);
        let eq173_e2182_d_n16: f64 = (eq173_e2181_d_n16 * ddt_scale);
        let eq173_e2182_d_n17: f64 = (eq173_e2181_d_n17 * ddt_scale);
        let eq173_e2182_d_n18: f64 = (eq173_e2181_d_n18 * ddt_scale);
        let eq173_e2182_d_n19: f64 = (eq173_e2181_d_n19 * ddt_scale);
        let eq173_e2182_d_n20: f64 = (eq173_e2181_d_n20 * ddt_scale);
        let eq173_e2182_d_n21: f64 = (eq173_e2181_d_n21 * ddt_scale);
        let eq173_e2182_d_n22: f64 = (eq173_e2181_d_n22 * ddt_scale);
        let eq173_e2183: f64 = (p.p7 * eq173_e2182);
        let eq173_e2183_d_n0: f64 = (p.p7 * eq173_e2182_d_n0);
        let eq173_e2183_d_n1: f64 = (p.p7 * eq173_e2182_d_n1);
        let eq173_e2183_d_n2: f64 = (p.p7 * eq173_e2182_d_n2);
        let eq173_e2183_d_n3: f64 = (p.p7 * eq173_e2182_d_n3);
        let eq173_e2183_d_n4: f64 = (p.p7 * eq173_e2182_d_n4);
        let eq173_e2183_d_n5: f64 = (p.p7 * eq173_e2182_d_n5);
        let eq173_e2183_d_n6: f64 = (p.p7 * eq173_e2182_d_n6);
        let eq173_e2183_d_n7: f64 = (p.p7 * eq173_e2182_d_n7);
        let eq173_e2183_d_n8: f64 = (p.p7 * eq173_e2182_d_n8);
        let eq173_e2183_d_n9: f64 = (p.p7 * eq173_e2182_d_n9);
        let eq173_e2183_d_n10: f64 = (p.p7 * eq173_e2182_d_n10);
        let eq173_e2183_d_n11: f64 = (p.p7 * eq173_e2182_d_n11);
        let eq173_e2183_d_n12: f64 = (p.p7 * eq173_e2182_d_n12);
        let eq173_e2183_d_n13: f64 = (p.p7 * eq173_e2182_d_n13);
        let eq173_e2183_d_n14: f64 = (p.p7 * eq173_e2182_d_n14);
        let eq173_e2183_d_n15: f64 = (p.p7 * eq173_e2182_d_n15);
        let eq173_e2183_d_n16: f64 = (p.p7 * eq173_e2182_d_n16);
        let eq173_e2183_d_n17: f64 = (p.p7 * eq173_e2182_d_n17);
        let eq173_e2183_d_n18: f64 = (p.p7 * eq173_e2182_d_n18);
        let eq173_e2183_d_n19: f64 = (p.p7 * eq173_e2182_d_n19);
        let eq173_e2183_d_n20: f64 = (p.p7 * eq173_e2182_d_n20);
        let eq173_e2183_d_n21: f64 = (p.p7 * eq173_e2182_d_n21);
        let eq173_e2183_d_n22: f64 = (p.p7 * eq173_e2182_d_n22);
        (eq173_e2183, eq173_e2183_d_n0, eq173_e2183_d_n1, eq173_e2183_d_n2, eq173_e2183_d_n3, eq173_e2183_d_n4, eq173_e2183_d_n5, eq173_e2183_d_n6, eq173_e2183_d_n7, eq173_e2183_d_n8, eq173_e2183_d_n9, eq173_e2183_d_n10, eq173_e2183_d_n11, eq173_e2183_d_n12, eq173_e2183_d_n13, eq173_e2183_d_n14, eq173_e2183_d_n15, eq173_e2183_d_n16, eq173_e2183_d_n17, eq173_e2183_d_n18, eq173_e2183_d_n19, eq173_e2183_d_n20, eq173_e2183_d_n21, eq173_e2183_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq173_value: f64 = eq173_e2185;
        let eq173_node_derivatives: [f64; 23] = [eq173_e2185_d_n0, eq173_e2185_d_n1, eq173_e2185_d_n2, eq173_e2185_d_n3, eq173_e2185_d_n4, eq173_e2185_d_n5, eq173_e2185_d_n6, eq173_e2185_d_n7, eq173_e2185_d_n8, eq173_e2185_d_n9, eq173_e2185_d_n10, eq173_e2185_d_n11, eq173_e2185_d_n12, eq173_e2185_d_n13, eq173_e2185_d_n14, eq173_e2185_d_n15, eq173_e2185_d_n16, eq173_e2185_d_n17, eq173_e2185_d_n18, eq173_e2185_d_n19, eq173_e2185_d_n20, eq173_e2185_d_n21, eq173_e2185_d_n22];
        let eq173_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[16]),
            multiplicity * (eq173_value),
            nodes,
            &eq173_node_derivatives,
            branches,
            &eq173_branch_derivatives,
            multiplicity,
        );
        let (eq174_e2195, eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22,) = {
    if ((!s.b[590]) && s.b[593]) {
        let eq174_e2192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 73, s.v[277]);
        let eq174_e2192_d_n0: f64 = (s.dn[277][0] * ddt_scale);
        let eq174_e2192_d_n1: f64 = (s.dn[277][1] * ddt_scale);
        let eq174_e2192_d_n2: f64 = (s.dn[277][2] * ddt_scale);
        let eq174_e2192_d_n3: f64 = (s.dn[277][3] * ddt_scale);
        let eq174_e2192_d_n4: f64 = (s.dn[277][4] * ddt_scale);
        let eq174_e2192_d_n5: f64 = (s.dn[277][5] * ddt_scale);
        let eq174_e2192_d_n6: f64 = (s.dn[277][6] * ddt_scale);
        let eq174_e2192_d_n7: f64 = (s.dn[277][7] * ddt_scale);
        let eq174_e2192_d_n8: f64 = (s.dn[277][8] * ddt_scale);
        let eq174_e2192_d_n9: f64 = (s.dn[277][9] * ddt_scale);
        let eq174_e2192_d_n10: f64 = (s.dn[277][10] * ddt_scale);
        let eq174_e2192_d_n11: f64 = (s.dn[277][11] * ddt_scale);
        let eq174_e2192_d_n12: f64 = (s.dn[277][12] * ddt_scale);
        let eq174_e2192_d_n13: f64 = (s.dn[277][13] * ddt_scale);
        let eq174_e2192_d_n14: f64 = (s.dn[277][14] * ddt_scale);
        let eq174_e2192_d_n15: f64 = (s.dn[277][15] * ddt_scale);
        let eq174_e2192_d_n16: f64 = (s.dn[277][16] * ddt_scale);
        let eq174_e2192_d_n17: f64 = (s.dn[277][17] * ddt_scale);
        let eq174_e2192_d_n18: f64 = (s.dn[277][18] * ddt_scale);
        let eq174_e2192_d_n19: f64 = (s.dn[277][19] * ddt_scale);
        let eq174_e2192_d_n20: f64 = (s.dn[277][20] * ddt_scale);
        let eq174_e2192_d_n21: f64 = (s.dn[277][21] * ddt_scale);
        let eq174_e2192_d_n22: f64 = (s.dn[277][22] * ddt_scale);
        let eq174_e2193: f64 = (p.p7 * eq174_e2192);
        let eq174_e2193_d_n0: f64 = (p.p7 * eq174_e2192_d_n0);
        let eq174_e2193_d_n1: f64 = (p.p7 * eq174_e2192_d_n1);
        let eq174_e2193_d_n2: f64 = (p.p7 * eq174_e2192_d_n2);
        let eq174_e2193_d_n3: f64 = (p.p7 * eq174_e2192_d_n3);
        let eq174_e2193_d_n4: f64 = (p.p7 * eq174_e2192_d_n4);
        let eq174_e2193_d_n5: f64 = (p.p7 * eq174_e2192_d_n5);
        let eq174_e2193_d_n6: f64 = (p.p7 * eq174_e2192_d_n6);
        let eq174_e2193_d_n7: f64 = (p.p7 * eq174_e2192_d_n7);
        let eq174_e2193_d_n8: f64 = (p.p7 * eq174_e2192_d_n8);
        let eq174_e2193_d_n9: f64 = (p.p7 * eq174_e2192_d_n9);
        let eq174_e2193_d_n10: f64 = (p.p7 * eq174_e2192_d_n10);
        let eq174_e2193_d_n11: f64 = (p.p7 * eq174_e2192_d_n11);
        let eq174_e2193_d_n12: f64 = (p.p7 * eq174_e2192_d_n12);
        let eq174_e2193_d_n13: f64 = (p.p7 * eq174_e2192_d_n13);
        let eq174_e2193_d_n14: f64 = (p.p7 * eq174_e2192_d_n14);
        let eq174_e2193_d_n15: f64 = (p.p7 * eq174_e2192_d_n15);
        let eq174_e2193_d_n16: f64 = (p.p7 * eq174_e2192_d_n16);
        let eq174_e2193_d_n17: f64 = (p.p7 * eq174_e2192_d_n17);
        let eq174_e2193_d_n18: f64 = (p.p7 * eq174_e2192_d_n18);
        let eq174_e2193_d_n19: f64 = (p.p7 * eq174_e2192_d_n19);
        let eq174_e2193_d_n20: f64 = (p.p7 * eq174_e2192_d_n20);
        let eq174_e2193_d_n21: f64 = (p.p7 * eq174_e2192_d_n21);
        let eq174_e2193_d_n22: f64 = (p.p7 * eq174_e2192_d_n22);
        (eq174_e2193, eq174_e2193_d_n0, eq174_e2193_d_n1, eq174_e2193_d_n2, eq174_e2193_d_n3, eq174_e2193_d_n4, eq174_e2193_d_n5, eq174_e2193_d_n6, eq174_e2193_d_n7, eq174_e2193_d_n8, eq174_e2193_d_n9, eq174_e2193_d_n10, eq174_e2193_d_n11, eq174_e2193_d_n12, eq174_e2193_d_n13, eq174_e2193_d_n14, eq174_e2193_d_n15, eq174_e2193_d_n16, eq174_e2193_d_n17, eq174_e2193_d_n18, eq174_e2193_d_n19, eq174_e2193_d_n20, eq174_e2193_d_n21, eq174_e2193_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq174_value: f64 = eq174_e2195;
        let eq174_node_derivatives: [f64; 23] = [eq174_e2195_d_n0, eq174_e2195_d_n1, eq174_e2195_d_n2, eq174_e2195_d_n3, eq174_e2195_d_n4, eq174_e2195_d_n5, eq174_e2195_d_n6, eq174_e2195_d_n7, eq174_e2195_d_n8, eq174_e2195_d_n9, eq174_e2195_d_n10, eq174_e2195_d_n11, eq174_e2195_d_n12, eq174_e2195_d_n13, eq174_e2195_d_n14, eq174_e2195_d_n15, eq174_e2195_d_n16, eq174_e2195_d_n17, eq174_e2195_d_n18, eq174_e2195_d_n19, eq174_e2195_d_n20, eq174_e2195_d_n21, eq174_e2195_d_n22];
        let eq174_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            multiplicity * (eq174_value),
            nodes,
            &eq174_node_derivatives,
            branches,
            &eq174_branch_derivatives,
            multiplicity,
        );
        let (eq175_e2207, eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && s.b[594]) {
        let eq175_e2204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 74, s.v[276]);
        let eq175_e2204_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq175_e2204_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq175_e2204_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq175_e2204_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq175_e2204_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq175_e2204_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq175_e2204_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq175_e2204_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq175_e2204_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq175_e2204_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq175_e2204_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq175_e2204_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq175_e2204_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq175_e2204_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq175_e2204_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq175_e2204_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq175_e2204_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq175_e2204_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq175_e2204_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq175_e2204_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq175_e2204_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq175_e2204_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq175_e2204_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq175_e2205: f64 = (p.p7 * eq175_e2204);
        let eq175_e2205_d_n0: f64 = (p.p7 * eq175_e2204_d_n0);
        let eq175_e2205_d_n1: f64 = (p.p7 * eq175_e2204_d_n1);
        let eq175_e2205_d_n2: f64 = (p.p7 * eq175_e2204_d_n2);
        let eq175_e2205_d_n3: f64 = (p.p7 * eq175_e2204_d_n3);
        let eq175_e2205_d_n4: f64 = (p.p7 * eq175_e2204_d_n4);
        let eq175_e2205_d_n5: f64 = (p.p7 * eq175_e2204_d_n5);
        let eq175_e2205_d_n6: f64 = (p.p7 * eq175_e2204_d_n6);
        let eq175_e2205_d_n7: f64 = (p.p7 * eq175_e2204_d_n7);
        let eq175_e2205_d_n8: f64 = (p.p7 * eq175_e2204_d_n8);
        let eq175_e2205_d_n9: f64 = (p.p7 * eq175_e2204_d_n9);
        let eq175_e2205_d_n10: f64 = (p.p7 * eq175_e2204_d_n10);
        let eq175_e2205_d_n11: f64 = (p.p7 * eq175_e2204_d_n11);
        let eq175_e2205_d_n12: f64 = (p.p7 * eq175_e2204_d_n12);
        let eq175_e2205_d_n13: f64 = (p.p7 * eq175_e2204_d_n13);
        let eq175_e2205_d_n14: f64 = (p.p7 * eq175_e2204_d_n14);
        let eq175_e2205_d_n15: f64 = (p.p7 * eq175_e2204_d_n15);
        let eq175_e2205_d_n16: f64 = (p.p7 * eq175_e2204_d_n16);
        let eq175_e2205_d_n17: f64 = (p.p7 * eq175_e2204_d_n17);
        let eq175_e2205_d_n18: f64 = (p.p7 * eq175_e2204_d_n18);
        let eq175_e2205_d_n19: f64 = (p.p7 * eq175_e2204_d_n19);
        let eq175_e2205_d_n20: f64 = (p.p7 * eq175_e2204_d_n20);
        let eq175_e2205_d_n21: f64 = (p.p7 * eq175_e2204_d_n21);
        let eq175_e2205_d_n22: f64 = (p.p7 * eq175_e2204_d_n22);
        (eq175_e2205, eq175_e2205_d_n0, eq175_e2205_d_n1, eq175_e2205_d_n2, eq175_e2205_d_n3, eq175_e2205_d_n4, eq175_e2205_d_n5, eq175_e2205_d_n6, eq175_e2205_d_n7, eq175_e2205_d_n8, eq175_e2205_d_n9, eq175_e2205_d_n10, eq175_e2205_d_n11, eq175_e2205_d_n12, eq175_e2205_d_n13, eq175_e2205_d_n14, eq175_e2205_d_n15, eq175_e2205_d_n16, eq175_e2205_d_n17, eq175_e2205_d_n18, eq175_e2205_d_n19, eq175_e2205_d_n20, eq175_e2205_d_n21, eq175_e2205_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq175_value: f64 = eq175_e2207;
        let eq175_node_derivatives: [f64; 23] = [eq175_e2207_d_n0, eq175_e2207_d_n1, eq175_e2207_d_n2, eq175_e2207_d_n3, eq175_e2207_d_n4, eq175_e2207_d_n5, eq175_e2207_d_n6, eq175_e2207_d_n7, eq175_e2207_d_n8, eq175_e2207_d_n9, eq175_e2207_d_n10, eq175_e2207_d_n11, eq175_e2207_d_n12, eq175_e2207_d_n13, eq175_e2207_d_n14, eq175_e2207_d_n15, eq175_e2207_d_n16, eq175_e2207_d_n17, eq175_e2207_d_n18, eq175_e2207_d_n19, eq175_e2207_d_n20, eq175_e2207_d_n21, eq175_e2207_d_n22];
        let eq175_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq175_value),
            nodes,
            &eq175_node_derivatives,
            branches,
            &eq175_branch_derivatives,
            multiplicity,
        );
        let (eq176_e2221, eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && s.b[594]) {
        let eq176_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 75, s.v[276]);
        let eq176_e2216_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq176_e2216_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq176_e2216_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq176_e2216_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq176_e2216_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq176_e2216_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq176_e2216_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq176_e2216_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq176_e2216_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq176_e2216_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq176_e2216_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq176_e2216_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq176_e2216_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq176_e2216_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq176_e2216_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq176_e2216_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq176_e2216_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq176_e2216_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq176_e2216_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq176_e2216_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq176_e2216_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq176_e2216_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq176_e2216_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq176_e2217: f64 = (p.p7 * eq176_e2216);
        let eq176_e2217_d_n0: f64 = (p.p7 * eq176_e2216_d_n0);
        let eq176_e2217_d_n1: f64 = (p.p7 * eq176_e2216_d_n1);
        let eq176_e2217_d_n2: f64 = (p.p7 * eq176_e2216_d_n2);
        let eq176_e2217_d_n3: f64 = (p.p7 * eq176_e2216_d_n3);
        let eq176_e2217_d_n4: f64 = (p.p7 * eq176_e2216_d_n4);
        let eq176_e2217_d_n5: f64 = (p.p7 * eq176_e2216_d_n5);
        let eq176_e2217_d_n6: f64 = (p.p7 * eq176_e2216_d_n6);
        let eq176_e2217_d_n7: f64 = (p.p7 * eq176_e2216_d_n7);
        let eq176_e2217_d_n8: f64 = (p.p7 * eq176_e2216_d_n8);
        let eq176_e2217_d_n9: f64 = (p.p7 * eq176_e2216_d_n9);
        let eq176_e2217_d_n10: f64 = (p.p7 * eq176_e2216_d_n10);
        let eq176_e2217_d_n11: f64 = (p.p7 * eq176_e2216_d_n11);
        let eq176_e2217_d_n12: f64 = (p.p7 * eq176_e2216_d_n12);
        let eq176_e2217_d_n13: f64 = (p.p7 * eq176_e2216_d_n13);
        let eq176_e2217_d_n14: f64 = (p.p7 * eq176_e2216_d_n14);
        let eq176_e2217_d_n15: f64 = (p.p7 * eq176_e2216_d_n15);
        let eq176_e2217_d_n16: f64 = (p.p7 * eq176_e2216_d_n16);
        let eq176_e2217_d_n17: f64 = (p.p7 * eq176_e2216_d_n17);
        let eq176_e2217_d_n18: f64 = (p.p7 * eq176_e2216_d_n18);
        let eq176_e2217_d_n19: f64 = (p.p7 * eq176_e2216_d_n19);
        let eq176_e2217_d_n20: f64 = (p.p7 * eq176_e2216_d_n20);
        let eq176_e2217_d_n21: f64 = (p.p7 * eq176_e2216_d_n21);
        let eq176_e2217_d_n22: f64 = (p.p7 * eq176_e2216_d_n22);
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
        (eq176_e2219, eq176_e2219_d_n0, eq176_e2219_d_n1, eq176_e2219_d_n2, eq176_e2219_d_n3, eq176_e2219_d_n4, eq176_e2219_d_n5, eq176_e2219_d_n6, eq176_e2219_d_n7, eq176_e2219_d_n8, eq176_e2219_d_n9, eq176_e2219_d_n10, eq176_e2219_d_n11, eq176_e2219_d_n12, eq176_e2219_d_n13, eq176_e2219_d_n14, eq176_e2219_d_n15, eq176_e2219_d_n16, eq176_e2219_d_n17, eq176_e2219_d_n18, eq176_e2219_d_n19, eq176_e2219_d_n20, eq176_e2219_d_n21, eq176_e2219_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq176_value: f64 = eq176_e2221;
        let eq176_node_derivatives: [f64; 23] = [eq176_e2221_d_n0, eq176_e2221_d_n1, eq176_e2221_d_n2, eq176_e2221_d_n3, eq176_e2221_d_n4, eq176_e2221_d_n5, eq176_e2221_d_n6, eq176_e2221_d_n7, eq176_e2221_d_n8, eq176_e2221_d_n9, eq176_e2221_d_n10, eq176_e2221_d_n11, eq176_e2221_d_n12, eq176_e2221_d_n13, eq176_e2221_d_n14, eq176_e2221_d_n15, eq176_e2221_d_n16, eq176_e2221_d_n17, eq176_e2221_d_n18, eq176_e2221_d_n19, eq176_e2221_d_n20, eq176_e2221_d_n21, eq176_e2221_d_n22];
        let eq176_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq176_value),
            nodes,
            &eq176_node_derivatives,
            branches,
            &eq176_branch_derivatives,
            multiplicity,
        );
        let (eq177_e2234, eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && (!s.b[594])) {
        let eq177_e2231: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 76, s.v[276]);
        let eq177_e2231_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq177_e2231_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq177_e2231_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq177_e2231_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq177_e2231_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq177_e2231_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq177_e2231_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq177_e2231_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq177_e2231_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq177_e2231_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq177_e2231_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq177_e2231_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq177_e2231_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq177_e2231_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq177_e2231_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq177_e2231_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq177_e2231_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq177_e2231_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq177_e2231_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq177_e2231_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq177_e2231_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq177_e2231_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq177_e2231_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq177_e2232: f64 = (p.p7 * eq177_e2231);
        let eq177_e2232_d_n0: f64 = (p.p7 * eq177_e2231_d_n0);
        let eq177_e2232_d_n1: f64 = (p.p7 * eq177_e2231_d_n1);
        let eq177_e2232_d_n2: f64 = (p.p7 * eq177_e2231_d_n2);
        let eq177_e2232_d_n3: f64 = (p.p7 * eq177_e2231_d_n3);
        let eq177_e2232_d_n4: f64 = (p.p7 * eq177_e2231_d_n4);
        let eq177_e2232_d_n5: f64 = (p.p7 * eq177_e2231_d_n5);
        let eq177_e2232_d_n6: f64 = (p.p7 * eq177_e2231_d_n6);
        let eq177_e2232_d_n7: f64 = (p.p7 * eq177_e2231_d_n7);
        let eq177_e2232_d_n8: f64 = (p.p7 * eq177_e2231_d_n8);
        let eq177_e2232_d_n9: f64 = (p.p7 * eq177_e2231_d_n9);
        let eq177_e2232_d_n10: f64 = (p.p7 * eq177_e2231_d_n10);
        let eq177_e2232_d_n11: f64 = (p.p7 * eq177_e2231_d_n11);
        let eq177_e2232_d_n12: f64 = (p.p7 * eq177_e2231_d_n12);
        let eq177_e2232_d_n13: f64 = (p.p7 * eq177_e2231_d_n13);
        let eq177_e2232_d_n14: f64 = (p.p7 * eq177_e2231_d_n14);
        let eq177_e2232_d_n15: f64 = (p.p7 * eq177_e2231_d_n15);
        let eq177_e2232_d_n16: f64 = (p.p7 * eq177_e2231_d_n16);
        let eq177_e2232_d_n17: f64 = (p.p7 * eq177_e2231_d_n17);
        let eq177_e2232_d_n18: f64 = (p.p7 * eq177_e2231_d_n18);
        let eq177_e2232_d_n19: f64 = (p.p7 * eq177_e2231_d_n19);
        let eq177_e2232_d_n20: f64 = (p.p7 * eq177_e2231_d_n20);
        let eq177_e2232_d_n21: f64 = (p.p7 * eq177_e2231_d_n21);
        let eq177_e2232_d_n22: f64 = (p.p7 * eq177_e2231_d_n22);
        (eq177_e2232, eq177_e2232_d_n0, eq177_e2232_d_n1, eq177_e2232_d_n2, eq177_e2232_d_n3, eq177_e2232_d_n4, eq177_e2232_d_n5, eq177_e2232_d_n6, eq177_e2232_d_n7, eq177_e2232_d_n8, eq177_e2232_d_n9, eq177_e2232_d_n10, eq177_e2232_d_n11, eq177_e2232_d_n12, eq177_e2232_d_n13, eq177_e2232_d_n14, eq177_e2232_d_n15, eq177_e2232_d_n16, eq177_e2232_d_n17, eq177_e2232_d_n18, eq177_e2232_d_n19, eq177_e2232_d_n20, eq177_e2232_d_n21, eq177_e2232_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq177_value: f64 = eq177_e2234;
        let eq177_node_derivatives: [f64; 23] = [eq177_e2234_d_n0, eq177_e2234_d_n1, eq177_e2234_d_n2, eq177_e2234_d_n3, eq177_e2234_d_n4, eq177_e2234_d_n5, eq177_e2234_d_n6, eq177_e2234_d_n7, eq177_e2234_d_n8, eq177_e2234_d_n9, eq177_e2234_d_n10, eq177_e2234_d_n11, eq177_e2234_d_n12, eq177_e2234_d_n13, eq177_e2234_d_n14, eq177_e2234_d_n15, eq177_e2234_d_n16, eq177_e2234_d_n17, eq177_e2234_d_n18, eq177_e2234_d_n19, eq177_e2234_d_n20, eq177_e2234_d_n21, eq177_e2234_d_n22];
        let eq177_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            multiplicity * (eq177_value),
            nodes,
            &eq177_node_derivatives,
            branches,
            &eq177_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_19(
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
        let (eq178_e2249, eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22,) = {
    if (((!s.b[590]) && s.b[593]) && (!s.b[594])) {
        let eq178_e2244: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 77, s.v[276]);
        let eq178_e2244_d_n0: f64 = (s.dn[276][0] * ddt_scale);
        let eq178_e2244_d_n1: f64 = (s.dn[276][1] * ddt_scale);
        let eq178_e2244_d_n2: f64 = (s.dn[276][2] * ddt_scale);
        let eq178_e2244_d_n3: f64 = (s.dn[276][3] * ddt_scale);
        let eq178_e2244_d_n4: f64 = (s.dn[276][4] * ddt_scale);
        let eq178_e2244_d_n5: f64 = (s.dn[276][5] * ddt_scale);
        let eq178_e2244_d_n6: f64 = (s.dn[276][6] * ddt_scale);
        let eq178_e2244_d_n7: f64 = (s.dn[276][7] * ddt_scale);
        let eq178_e2244_d_n8: f64 = (s.dn[276][8] * ddt_scale);
        let eq178_e2244_d_n9: f64 = (s.dn[276][9] * ddt_scale);
        let eq178_e2244_d_n10: f64 = (s.dn[276][10] * ddt_scale);
        let eq178_e2244_d_n11: f64 = (s.dn[276][11] * ddt_scale);
        let eq178_e2244_d_n12: f64 = (s.dn[276][12] * ddt_scale);
        let eq178_e2244_d_n13: f64 = (s.dn[276][13] * ddt_scale);
        let eq178_e2244_d_n14: f64 = (s.dn[276][14] * ddt_scale);
        let eq178_e2244_d_n15: f64 = (s.dn[276][15] * ddt_scale);
        let eq178_e2244_d_n16: f64 = (s.dn[276][16] * ddt_scale);
        let eq178_e2244_d_n17: f64 = (s.dn[276][17] * ddt_scale);
        let eq178_e2244_d_n18: f64 = (s.dn[276][18] * ddt_scale);
        let eq178_e2244_d_n19: f64 = (s.dn[276][19] * ddt_scale);
        let eq178_e2244_d_n20: f64 = (s.dn[276][20] * ddt_scale);
        let eq178_e2244_d_n21: f64 = (s.dn[276][21] * ddt_scale);
        let eq178_e2244_d_n22: f64 = (s.dn[276][22] * ddt_scale);
        let eq178_e2245: f64 = (p.p7 * eq178_e2244);
        let eq178_e2245_d_n0: f64 = (p.p7 * eq178_e2244_d_n0);
        let eq178_e2245_d_n1: f64 = (p.p7 * eq178_e2244_d_n1);
        let eq178_e2245_d_n2: f64 = (p.p7 * eq178_e2244_d_n2);
        let eq178_e2245_d_n3: f64 = (p.p7 * eq178_e2244_d_n3);
        let eq178_e2245_d_n4: f64 = (p.p7 * eq178_e2244_d_n4);
        let eq178_e2245_d_n5: f64 = (p.p7 * eq178_e2244_d_n5);
        let eq178_e2245_d_n6: f64 = (p.p7 * eq178_e2244_d_n6);
        let eq178_e2245_d_n7: f64 = (p.p7 * eq178_e2244_d_n7);
        let eq178_e2245_d_n8: f64 = (p.p7 * eq178_e2244_d_n8);
        let eq178_e2245_d_n9: f64 = (p.p7 * eq178_e2244_d_n9);
        let eq178_e2245_d_n10: f64 = (p.p7 * eq178_e2244_d_n10);
        let eq178_e2245_d_n11: f64 = (p.p7 * eq178_e2244_d_n11);
        let eq178_e2245_d_n12: f64 = (p.p7 * eq178_e2244_d_n12);
        let eq178_e2245_d_n13: f64 = (p.p7 * eq178_e2244_d_n13);
        let eq178_e2245_d_n14: f64 = (p.p7 * eq178_e2244_d_n14);
        let eq178_e2245_d_n15: f64 = (p.p7 * eq178_e2244_d_n15);
        let eq178_e2245_d_n16: f64 = (p.p7 * eq178_e2244_d_n16);
        let eq178_e2245_d_n17: f64 = (p.p7 * eq178_e2244_d_n17);
        let eq178_e2245_d_n18: f64 = (p.p7 * eq178_e2244_d_n18);
        let eq178_e2245_d_n19: f64 = (p.p7 * eq178_e2244_d_n19);
        let eq178_e2245_d_n20: f64 = (p.p7 * eq178_e2244_d_n20);
        let eq178_e2245_d_n21: f64 = (p.p7 * eq178_e2244_d_n21);
        let eq178_e2245_d_n22: f64 = (p.p7 * eq178_e2244_d_n22);
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
        (eq178_e2247, eq178_e2247_d_n0, eq178_e2247_d_n1, eq178_e2247_d_n2, eq178_e2247_d_n3, eq178_e2247_d_n4, eq178_e2247_d_n5, eq178_e2247_d_n6, eq178_e2247_d_n7, eq178_e2247_d_n8, eq178_e2247_d_n9, eq178_e2247_d_n10, eq178_e2247_d_n11, eq178_e2247_d_n12, eq178_e2247_d_n13, eq178_e2247_d_n14, eq178_e2247_d_n15, eq178_e2247_d_n16, eq178_e2247_d_n17, eq178_e2247_d_n18, eq178_e2247_d_n19, eq178_e2247_d_n20, eq178_e2247_d_n21, eq178_e2247_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq178_value: f64 = eq178_e2249;
        let eq178_node_derivatives: [f64; 23] = [eq178_e2249_d_n0, eq178_e2249_d_n1, eq178_e2249_d_n2, eq178_e2249_d_n3, eq178_e2249_d_n4, eq178_e2249_d_n5, eq178_e2249_d_n6, eq178_e2249_d_n7, eq178_e2249_d_n8, eq178_e2249_d_n9, eq178_e2249_d_n10, eq178_e2249_d_n11, eq178_e2249_d_n12, eq178_e2249_d_n13, eq178_e2249_d_n14, eq178_e2249_d_n15, eq178_e2249_d_n16, eq178_e2249_d_n17, eq178_e2249_d_n18, eq178_e2249_d_n19, eq178_e2249_d_n20, eq178_e2249_d_n21, eq178_e2249_d_n22];
        let eq178_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq178_value),
            nodes,
            &eq178_node_derivatives,
            branches,
            &eq178_branch_derivatives,
            multiplicity,
        );
        let (eq179_e2261, eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22,) = {
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
        let eq179_e2258: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 78, eq179_e2257);
        let eq179_e2258_d_n0: f64 = (eq179_e2257_d_n0 * ddt_scale);
        let eq179_e2258_d_n1: f64 = (eq179_e2257_d_n1 * ddt_scale);
        let eq179_e2258_d_n2: f64 = (eq179_e2257_d_n2 * ddt_scale);
        let eq179_e2258_d_n3: f64 = (eq179_e2257_d_n3 * ddt_scale);
        let eq179_e2258_d_n4: f64 = (eq179_e2257_d_n4 * ddt_scale);
        let eq179_e2258_d_n5: f64 = (eq179_e2257_d_n5 * ddt_scale);
        let eq179_e2258_d_n6: f64 = (eq179_e2257_d_n6 * ddt_scale);
        let eq179_e2258_d_n7: f64 = (eq179_e2257_d_n7 * ddt_scale);
        let eq179_e2258_d_n8: f64 = (eq179_e2257_d_n8 * ddt_scale);
        let eq179_e2258_d_n9: f64 = (eq179_e2257_d_n9 * ddt_scale);
        let eq179_e2258_d_n10: f64 = (eq179_e2257_d_n10 * ddt_scale);
        let eq179_e2258_d_n11: f64 = (eq179_e2257_d_n11 * ddt_scale);
        let eq179_e2258_d_n12: f64 = (eq179_e2257_d_n12 * ddt_scale);
        let eq179_e2258_d_n13: f64 = (eq179_e2257_d_n13 * ddt_scale);
        let eq179_e2258_d_n14: f64 = (eq179_e2257_d_n14 * ddt_scale);
        let eq179_e2258_d_n15: f64 = (eq179_e2257_d_n15 * ddt_scale);
        let eq179_e2258_d_n16: f64 = (eq179_e2257_d_n16 * ddt_scale);
        let eq179_e2258_d_n17: f64 = (eq179_e2257_d_n17 * ddt_scale);
        let eq179_e2258_d_n18: f64 = (eq179_e2257_d_n18 * ddt_scale);
        let eq179_e2258_d_n19: f64 = (eq179_e2257_d_n19 * ddt_scale);
        let eq179_e2258_d_n20: f64 = (eq179_e2257_d_n20 * ddt_scale);
        let eq179_e2258_d_n21: f64 = (eq179_e2257_d_n21 * ddt_scale);
        let eq179_e2258_d_n22: f64 = (eq179_e2257_d_n22 * ddt_scale);
        let eq179_e2259: f64 = (p.p7 * eq179_e2258);
        let eq179_e2259_d_n0: f64 = (p.p7 * eq179_e2258_d_n0);
        let eq179_e2259_d_n1: f64 = (p.p7 * eq179_e2258_d_n1);
        let eq179_e2259_d_n2: f64 = (p.p7 * eq179_e2258_d_n2);
        let eq179_e2259_d_n3: f64 = (p.p7 * eq179_e2258_d_n3);
        let eq179_e2259_d_n4: f64 = (p.p7 * eq179_e2258_d_n4);
        let eq179_e2259_d_n5: f64 = (p.p7 * eq179_e2258_d_n5);
        let eq179_e2259_d_n6: f64 = (p.p7 * eq179_e2258_d_n6);
        let eq179_e2259_d_n7: f64 = (p.p7 * eq179_e2258_d_n7);
        let eq179_e2259_d_n8: f64 = (p.p7 * eq179_e2258_d_n8);
        let eq179_e2259_d_n9: f64 = (p.p7 * eq179_e2258_d_n9);
        let eq179_e2259_d_n10: f64 = (p.p7 * eq179_e2258_d_n10);
        let eq179_e2259_d_n11: f64 = (p.p7 * eq179_e2258_d_n11);
        let eq179_e2259_d_n12: f64 = (p.p7 * eq179_e2258_d_n12);
        let eq179_e2259_d_n13: f64 = (p.p7 * eq179_e2258_d_n13);
        let eq179_e2259_d_n14: f64 = (p.p7 * eq179_e2258_d_n14);
        let eq179_e2259_d_n15: f64 = (p.p7 * eq179_e2258_d_n15);
        let eq179_e2259_d_n16: f64 = (p.p7 * eq179_e2258_d_n16);
        let eq179_e2259_d_n17: f64 = (p.p7 * eq179_e2258_d_n17);
        let eq179_e2259_d_n18: f64 = (p.p7 * eq179_e2258_d_n18);
        let eq179_e2259_d_n19: f64 = (p.p7 * eq179_e2258_d_n19);
        let eq179_e2259_d_n20: f64 = (p.p7 * eq179_e2258_d_n20);
        let eq179_e2259_d_n21: f64 = (p.p7 * eq179_e2258_d_n21);
        let eq179_e2259_d_n22: f64 = (p.p7 * eq179_e2258_d_n22);
        (eq179_e2259, eq179_e2259_d_n0, eq179_e2259_d_n1, eq179_e2259_d_n2, eq179_e2259_d_n3, eq179_e2259_d_n4, eq179_e2259_d_n5, eq179_e2259_d_n6, eq179_e2259_d_n7, eq179_e2259_d_n8, eq179_e2259_d_n9, eq179_e2259_d_n10, eq179_e2259_d_n11, eq179_e2259_d_n12, eq179_e2259_d_n13, eq179_e2259_d_n14, eq179_e2259_d_n15, eq179_e2259_d_n16, eq179_e2259_d_n17, eq179_e2259_d_n18, eq179_e2259_d_n19, eq179_e2259_d_n20, eq179_e2259_d_n21, eq179_e2259_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq179_value: f64 = eq179_e2261;
        let eq179_node_derivatives: [f64; 23] = [eq179_e2261_d_n0, eq179_e2261_d_n1, eq179_e2261_d_n2, eq179_e2261_d_n3, eq179_e2261_d_n4, eq179_e2261_d_n5, eq179_e2261_d_n6, eq179_e2261_d_n7, eq179_e2261_d_n8, eq179_e2261_d_n9, eq179_e2261_d_n10, eq179_e2261_d_n11, eq179_e2261_d_n12, eq179_e2261_d_n13, eq179_e2261_d_n14, eq179_e2261_d_n15, eq179_e2261_d_n16, eq179_e2261_d_n17, eq179_e2261_d_n18, eq179_e2261_d_n19, eq179_e2261_d_n20, eq179_e2261_d_n21, eq179_e2261_d_n22];
        let eq179_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            multiplicity * (eq179_value),
            nodes,
            &eq179_node_derivatives,
            branches,
            &eq179_branch_derivatives,
            multiplicity,
        );
        let (eq180_e2270, eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22,) = {
    if (s.b[595] && s.b[596]) {
        let eq180_e2267: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 79, s.v[289]);
        let eq180_e2267_d_n0: f64 = (s.dn[289][0] * ddt_scale);
        let eq180_e2267_d_n1: f64 = (s.dn[289][1] * ddt_scale);
        let eq180_e2267_d_n2: f64 = (s.dn[289][2] * ddt_scale);
        let eq180_e2267_d_n3: f64 = (s.dn[289][3] * ddt_scale);
        let eq180_e2267_d_n4: f64 = (s.dn[289][4] * ddt_scale);
        let eq180_e2267_d_n5: f64 = (s.dn[289][5] * ddt_scale);
        let eq180_e2267_d_n6: f64 = (s.dn[289][6] * ddt_scale);
        let eq180_e2267_d_n7: f64 = (s.dn[289][7] * ddt_scale);
        let eq180_e2267_d_n8: f64 = (s.dn[289][8] * ddt_scale);
        let eq180_e2267_d_n9: f64 = (s.dn[289][9] * ddt_scale);
        let eq180_e2267_d_n10: f64 = (s.dn[289][10] * ddt_scale);
        let eq180_e2267_d_n11: f64 = (s.dn[289][11] * ddt_scale);
        let eq180_e2267_d_n12: f64 = (s.dn[289][12] * ddt_scale);
        let eq180_e2267_d_n13: f64 = (s.dn[289][13] * ddt_scale);
        let eq180_e2267_d_n14: f64 = (s.dn[289][14] * ddt_scale);
        let eq180_e2267_d_n15: f64 = (s.dn[289][15] * ddt_scale);
        let eq180_e2267_d_n16: f64 = (s.dn[289][16] * ddt_scale);
        let eq180_e2267_d_n17: f64 = (s.dn[289][17] * ddt_scale);
        let eq180_e2267_d_n18: f64 = (s.dn[289][18] * ddt_scale);
        let eq180_e2267_d_n19: f64 = (s.dn[289][19] * ddt_scale);
        let eq180_e2267_d_n20: f64 = (s.dn[289][20] * ddt_scale);
        let eq180_e2267_d_n21: f64 = (s.dn[289][21] * ddt_scale);
        let eq180_e2267_d_n22: f64 = (s.dn[289][22] * ddt_scale);
        let eq180_e2268: f64 = (p.p7 * eq180_e2267);
        let eq180_e2268_d_n0: f64 = (p.p7 * eq180_e2267_d_n0);
        let eq180_e2268_d_n1: f64 = (p.p7 * eq180_e2267_d_n1);
        let eq180_e2268_d_n2: f64 = (p.p7 * eq180_e2267_d_n2);
        let eq180_e2268_d_n3: f64 = (p.p7 * eq180_e2267_d_n3);
        let eq180_e2268_d_n4: f64 = (p.p7 * eq180_e2267_d_n4);
        let eq180_e2268_d_n5: f64 = (p.p7 * eq180_e2267_d_n5);
        let eq180_e2268_d_n6: f64 = (p.p7 * eq180_e2267_d_n6);
        let eq180_e2268_d_n7: f64 = (p.p7 * eq180_e2267_d_n7);
        let eq180_e2268_d_n8: f64 = (p.p7 * eq180_e2267_d_n8);
        let eq180_e2268_d_n9: f64 = (p.p7 * eq180_e2267_d_n9);
        let eq180_e2268_d_n10: f64 = (p.p7 * eq180_e2267_d_n10);
        let eq180_e2268_d_n11: f64 = (p.p7 * eq180_e2267_d_n11);
        let eq180_e2268_d_n12: f64 = (p.p7 * eq180_e2267_d_n12);
        let eq180_e2268_d_n13: f64 = (p.p7 * eq180_e2267_d_n13);
        let eq180_e2268_d_n14: f64 = (p.p7 * eq180_e2267_d_n14);
        let eq180_e2268_d_n15: f64 = (p.p7 * eq180_e2267_d_n15);
        let eq180_e2268_d_n16: f64 = (p.p7 * eq180_e2267_d_n16);
        let eq180_e2268_d_n17: f64 = (p.p7 * eq180_e2267_d_n17);
        let eq180_e2268_d_n18: f64 = (p.p7 * eq180_e2267_d_n18);
        let eq180_e2268_d_n19: f64 = (p.p7 * eq180_e2267_d_n19);
        let eq180_e2268_d_n20: f64 = (p.p7 * eq180_e2267_d_n20);
        let eq180_e2268_d_n21: f64 = (p.p7 * eq180_e2267_d_n21);
        let eq180_e2268_d_n22: f64 = (p.p7 * eq180_e2267_d_n22);
        (eq180_e2268, eq180_e2268_d_n0, eq180_e2268_d_n1, eq180_e2268_d_n2, eq180_e2268_d_n3, eq180_e2268_d_n4, eq180_e2268_d_n5, eq180_e2268_d_n6, eq180_e2268_d_n7, eq180_e2268_d_n8, eq180_e2268_d_n9, eq180_e2268_d_n10, eq180_e2268_d_n11, eq180_e2268_d_n12, eq180_e2268_d_n13, eq180_e2268_d_n14, eq180_e2268_d_n15, eq180_e2268_d_n16, eq180_e2268_d_n17, eq180_e2268_d_n18, eq180_e2268_d_n19, eq180_e2268_d_n20, eq180_e2268_d_n21, eq180_e2268_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq180_value: f64 = eq180_e2270;
        let eq180_node_derivatives: [f64; 23] = [eq180_e2270_d_n0, eq180_e2270_d_n1, eq180_e2270_d_n2, eq180_e2270_d_n3, eq180_e2270_d_n4, eq180_e2270_d_n5, eq180_e2270_d_n6, eq180_e2270_d_n7, eq180_e2270_d_n8, eq180_e2270_d_n9, eq180_e2270_d_n10, eq180_e2270_d_n11, eq180_e2270_d_n12, eq180_e2270_d_n13, eq180_e2270_d_n14, eq180_e2270_d_n15, eq180_e2270_d_n16, eq180_e2270_d_n17, eq180_e2270_d_n18, eq180_e2270_d_n19, eq180_e2270_d_n20, eq180_e2270_d_n21, eq180_e2270_d_n22];
        let eq180_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[21]),
            multiplicity * (eq180_value),
            nodes,
            &eq180_node_derivatives,
            branches,
            &eq180_branch_derivatives,
            multiplicity,
        );
        let (eq181_e2281, eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22,) = {
    if ((s.b[595] && s.b[596]) && s.b[597]) {
        let eq181_e2278: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 80, s.v[288]);
        let eq181_e2278_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq181_e2278_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq181_e2278_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq181_e2278_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq181_e2278_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq181_e2278_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq181_e2278_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq181_e2278_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq181_e2278_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq181_e2278_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq181_e2278_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq181_e2278_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq181_e2278_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq181_e2278_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq181_e2278_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq181_e2278_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq181_e2278_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq181_e2278_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq181_e2278_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq181_e2278_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq181_e2278_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq181_e2278_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq181_e2278_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq181_e2279: f64 = (p.p7 * eq181_e2278);
        let eq181_e2279_d_n0: f64 = (p.p7 * eq181_e2278_d_n0);
        let eq181_e2279_d_n1: f64 = (p.p7 * eq181_e2278_d_n1);
        let eq181_e2279_d_n2: f64 = (p.p7 * eq181_e2278_d_n2);
        let eq181_e2279_d_n3: f64 = (p.p7 * eq181_e2278_d_n3);
        let eq181_e2279_d_n4: f64 = (p.p7 * eq181_e2278_d_n4);
        let eq181_e2279_d_n5: f64 = (p.p7 * eq181_e2278_d_n5);
        let eq181_e2279_d_n6: f64 = (p.p7 * eq181_e2278_d_n6);
        let eq181_e2279_d_n7: f64 = (p.p7 * eq181_e2278_d_n7);
        let eq181_e2279_d_n8: f64 = (p.p7 * eq181_e2278_d_n8);
        let eq181_e2279_d_n9: f64 = (p.p7 * eq181_e2278_d_n9);
        let eq181_e2279_d_n10: f64 = (p.p7 * eq181_e2278_d_n10);
        let eq181_e2279_d_n11: f64 = (p.p7 * eq181_e2278_d_n11);
        let eq181_e2279_d_n12: f64 = (p.p7 * eq181_e2278_d_n12);
        let eq181_e2279_d_n13: f64 = (p.p7 * eq181_e2278_d_n13);
        let eq181_e2279_d_n14: f64 = (p.p7 * eq181_e2278_d_n14);
        let eq181_e2279_d_n15: f64 = (p.p7 * eq181_e2278_d_n15);
        let eq181_e2279_d_n16: f64 = (p.p7 * eq181_e2278_d_n16);
        let eq181_e2279_d_n17: f64 = (p.p7 * eq181_e2278_d_n17);
        let eq181_e2279_d_n18: f64 = (p.p7 * eq181_e2278_d_n18);
        let eq181_e2279_d_n19: f64 = (p.p7 * eq181_e2278_d_n19);
        let eq181_e2279_d_n20: f64 = (p.p7 * eq181_e2278_d_n20);
        let eq181_e2279_d_n21: f64 = (p.p7 * eq181_e2278_d_n21);
        let eq181_e2279_d_n22: f64 = (p.p7 * eq181_e2278_d_n22);
        (eq181_e2279, eq181_e2279_d_n0, eq181_e2279_d_n1, eq181_e2279_d_n2, eq181_e2279_d_n3, eq181_e2279_d_n4, eq181_e2279_d_n5, eq181_e2279_d_n6, eq181_e2279_d_n7, eq181_e2279_d_n8, eq181_e2279_d_n9, eq181_e2279_d_n10, eq181_e2279_d_n11, eq181_e2279_d_n12, eq181_e2279_d_n13, eq181_e2279_d_n14, eq181_e2279_d_n15, eq181_e2279_d_n16, eq181_e2279_d_n17, eq181_e2279_d_n18, eq181_e2279_d_n19, eq181_e2279_d_n20, eq181_e2279_d_n21, eq181_e2279_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq181_value: f64 = eq181_e2281;
        let eq181_node_derivatives: [f64; 23] = [eq181_e2281_d_n0, eq181_e2281_d_n1, eq181_e2281_d_n2, eq181_e2281_d_n3, eq181_e2281_d_n4, eq181_e2281_d_n5, eq181_e2281_d_n6, eq181_e2281_d_n7, eq181_e2281_d_n8, eq181_e2281_d_n9, eq181_e2281_d_n10, eq181_e2281_d_n11, eq181_e2281_d_n12, eq181_e2281_d_n13, eq181_e2281_d_n14, eq181_e2281_d_n15, eq181_e2281_d_n16, eq181_e2281_d_n17, eq181_e2281_d_n18, eq181_e2281_d_n19, eq181_e2281_d_n20, eq181_e2281_d_n21, eq181_e2281_d_n22];
        let eq181_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            multiplicity * (eq181_value),
            nodes,
            &eq181_node_derivatives,
            branches,
            &eq181_branch_derivatives,
            multiplicity,
        );
        let (eq182_e2294, eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22,) = {
    if ((s.b[595] && s.b[596]) && s.b[597]) {
        let eq182_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 81, s.v[288]);
        let eq182_e2289_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq182_e2289_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq182_e2289_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq182_e2289_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq182_e2289_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq182_e2289_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq182_e2289_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq182_e2289_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq182_e2289_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq182_e2289_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq182_e2289_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq182_e2289_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq182_e2289_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq182_e2289_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq182_e2289_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq182_e2289_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq182_e2289_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq182_e2289_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq182_e2289_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq182_e2289_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq182_e2289_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq182_e2289_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq182_e2289_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq182_e2290: f64 = (p.p7 * eq182_e2289);
        let eq182_e2290_d_n0: f64 = (p.p7 * eq182_e2289_d_n0);
        let eq182_e2290_d_n1: f64 = (p.p7 * eq182_e2289_d_n1);
        let eq182_e2290_d_n2: f64 = (p.p7 * eq182_e2289_d_n2);
        let eq182_e2290_d_n3: f64 = (p.p7 * eq182_e2289_d_n3);
        let eq182_e2290_d_n4: f64 = (p.p7 * eq182_e2289_d_n4);
        let eq182_e2290_d_n5: f64 = (p.p7 * eq182_e2289_d_n5);
        let eq182_e2290_d_n6: f64 = (p.p7 * eq182_e2289_d_n6);
        let eq182_e2290_d_n7: f64 = (p.p7 * eq182_e2289_d_n7);
        let eq182_e2290_d_n8: f64 = (p.p7 * eq182_e2289_d_n8);
        let eq182_e2290_d_n9: f64 = (p.p7 * eq182_e2289_d_n9);
        let eq182_e2290_d_n10: f64 = (p.p7 * eq182_e2289_d_n10);
        let eq182_e2290_d_n11: f64 = (p.p7 * eq182_e2289_d_n11);
        let eq182_e2290_d_n12: f64 = (p.p7 * eq182_e2289_d_n12);
        let eq182_e2290_d_n13: f64 = (p.p7 * eq182_e2289_d_n13);
        let eq182_e2290_d_n14: f64 = (p.p7 * eq182_e2289_d_n14);
        let eq182_e2290_d_n15: f64 = (p.p7 * eq182_e2289_d_n15);
        let eq182_e2290_d_n16: f64 = (p.p7 * eq182_e2289_d_n16);
        let eq182_e2290_d_n17: f64 = (p.p7 * eq182_e2289_d_n17);
        let eq182_e2290_d_n18: f64 = (p.p7 * eq182_e2289_d_n18);
        let eq182_e2290_d_n19: f64 = (p.p7 * eq182_e2289_d_n19);
        let eq182_e2290_d_n20: f64 = (p.p7 * eq182_e2289_d_n20);
        let eq182_e2290_d_n21: f64 = (p.p7 * eq182_e2289_d_n21);
        let eq182_e2290_d_n22: f64 = (p.p7 * eq182_e2289_d_n22);
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
        (eq182_e2292, eq182_e2292_d_n0, eq182_e2292_d_n1, eq182_e2292_d_n2, eq182_e2292_d_n3, eq182_e2292_d_n4, eq182_e2292_d_n5, eq182_e2292_d_n6, eq182_e2292_d_n7, eq182_e2292_d_n8, eq182_e2292_d_n9, eq182_e2292_d_n10, eq182_e2292_d_n11, eq182_e2292_d_n12, eq182_e2292_d_n13, eq182_e2292_d_n14, eq182_e2292_d_n15, eq182_e2292_d_n16, eq182_e2292_d_n17, eq182_e2292_d_n18, eq182_e2292_d_n19, eq182_e2292_d_n20, eq182_e2292_d_n21, eq182_e2292_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq182_value: f64 = eq182_e2294;
        let eq182_node_derivatives: [f64; 23] = [eq182_e2294_d_n0, eq182_e2294_d_n1, eq182_e2294_d_n2, eq182_e2294_d_n3, eq182_e2294_d_n4, eq182_e2294_d_n5, eq182_e2294_d_n6, eq182_e2294_d_n7, eq182_e2294_d_n8, eq182_e2294_d_n9, eq182_e2294_d_n10, eq182_e2294_d_n11, eq182_e2294_d_n12, eq182_e2294_d_n13, eq182_e2294_d_n14, eq182_e2294_d_n15, eq182_e2294_d_n16, eq182_e2294_d_n17, eq182_e2294_d_n18, eq182_e2294_d_n19, eq182_e2294_d_n20, eq182_e2294_d_n21, eq182_e2294_d_n22];
        let eq182_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            multiplicity * (eq182_value),
            nodes,
            &eq182_node_derivatives,
            branches,
            &eq182_branch_derivatives,
            multiplicity,
        );
        let (eq183_e2306, eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22,) = {
    if ((s.b[595] && s.b[596]) && (!s.b[597])) {
        let eq183_e2303: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 82, s.v[288]);
        let eq183_e2303_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq183_e2303_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq183_e2303_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq183_e2303_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq183_e2303_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq183_e2303_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq183_e2303_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq183_e2303_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq183_e2303_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq183_e2303_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq183_e2303_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq183_e2303_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq183_e2303_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq183_e2303_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq183_e2303_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq183_e2303_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq183_e2303_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq183_e2303_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq183_e2303_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq183_e2303_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq183_e2303_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq183_e2303_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq183_e2303_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq183_e2304: f64 = (p.p7 * eq183_e2303);
        let eq183_e2304_d_n0: f64 = (p.p7 * eq183_e2303_d_n0);
        let eq183_e2304_d_n1: f64 = (p.p7 * eq183_e2303_d_n1);
        let eq183_e2304_d_n2: f64 = (p.p7 * eq183_e2303_d_n2);
        let eq183_e2304_d_n3: f64 = (p.p7 * eq183_e2303_d_n3);
        let eq183_e2304_d_n4: f64 = (p.p7 * eq183_e2303_d_n4);
        let eq183_e2304_d_n5: f64 = (p.p7 * eq183_e2303_d_n5);
        let eq183_e2304_d_n6: f64 = (p.p7 * eq183_e2303_d_n6);
        let eq183_e2304_d_n7: f64 = (p.p7 * eq183_e2303_d_n7);
        let eq183_e2304_d_n8: f64 = (p.p7 * eq183_e2303_d_n8);
        let eq183_e2304_d_n9: f64 = (p.p7 * eq183_e2303_d_n9);
        let eq183_e2304_d_n10: f64 = (p.p7 * eq183_e2303_d_n10);
        let eq183_e2304_d_n11: f64 = (p.p7 * eq183_e2303_d_n11);
        let eq183_e2304_d_n12: f64 = (p.p7 * eq183_e2303_d_n12);
        let eq183_e2304_d_n13: f64 = (p.p7 * eq183_e2303_d_n13);
        let eq183_e2304_d_n14: f64 = (p.p7 * eq183_e2303_d_n14);
        let eq183_e2304_d_n15: f64 = (p.p7 * eq183_e2303_d_n15);
        let eq183_e2304_d_n16: f64 = (p.p7 * eq183_e2303_d_n16);
        let eq183_e2304_d_n17: f64 = (p.p7 * eq183_e2303_d_n17);
        let eq183_e2304_d_n18: f64 = (p.p7 * eq183_e2303_d_n18);
        let eq183_e2304_d_n19: f64 = (p.p7 * eq183_e2303_d_n19);
        let eq183_e2304_d_n20: f64 = (p.p7 * eq183_e2303_d_n20);
        let eq183_e2304_d_n21: f64 = (p.p7 * eq183_e2303_d_n21);
        let eq183_e2304_d_n22: f64 = (p.p7 * eq183_e2303_d_n22);
        (eq183_e2304, eq183_e2304_d_n0, eq183_e2304_d_n1, eq183_e2304_d_n2, eq183_e2304_d_n3, eq183_e2304_d_n4, eq183_e2304_d_n5, eq183_e2304_d_n6, eq183_e2304_d_n7, eq183_e2304_d_n8, eq183_e2304_d_n9, eq183_e2304_d_n10, eq183_e2304_d_n11, eq183_e2304_d_n12, eq183_e2304_d_n13, eq183_e2304_d_n14, eq183_e2304_d_n15, eq183_e2304_d_n16, eq183_e2304_d_n17, eq183_e2304_d_n18, eq183_e2304_d_n19, eq183_e2304_d_n20, eq183_e2304_d_n21, eq183_e2304_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq183_value: f64 = eq183_e2306;
        let eq183_node_derivatives: [f64; 23] = [eq183_e2306_d_n0, eq183_e2306_d_n1, eq183_e2306_d_n2, eq183_e2306_d_n3, eq183_e2306_d_n4, eq183_e2306_d_n5, eq183_e2306_d_n6, eq183_e2306_d_n7, eq183_e2306_d_n8, eq183_e2306_d_n9, eq183_e2306_d_n10, eq183_e2306_d_n11, eq183_e2306_d_n12, eq183_e2306_d_n13, eq183_e2306_d_n14, eq183_e2306_d_n15, eq183_e2306_d_n16, eq183_e2306_d_n17, eq183_e2306_d_n18, eq183_e2306_d_n19, eq183_e2306_d_n20, eq183_e2306_d_n21, eq183_e2306_d_n22];
        let eq183_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[21]),
            multiplicity * (eq183_value),
            nodes,
            &eq183_node_derivatives,
            branches,
            &eq183_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_20(
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
        let (eq184_e2320, eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22,) = {
    if ((s.b[595] && s.b[596]) && (!s.b[597])) {
        let eq184_e2315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 83, s.v[288]);
        let eq184_e2315_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq184_e2315_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq184_e2315_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq184_e2315_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq184_e2315_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq184_e2315_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq184_e2315_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq184_e2315_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq184_e2315_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq184_e2315_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq184_e2315_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq184_e2315_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq184_e2315_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq184_e2315_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq184_e2315_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq184_e2315_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq184_e2315_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq184_e2315_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq184_e2315_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq184_e2315_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq184_e2315_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq184_e2315_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq184_e2315_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq184_e2316: f64 = (p.p7 * eq184_e2315);
        let eq184_e2316_d_n0: f64 = (p.p7 * eq184_e2315_d_n0);
        let eq184_e2316_d_n1: f64 = (p.p7 * eq184_e2315_d_n1);
        let eq184_e2316_d_n2: f64 = (p.p7 * eq184_e2315_d_n2);
        let eq184_e2316_d_n3: f64 = (p.p7 * eq184_e2315_d_n3);
        let eq184_e2316_d_n4: f64 = (p.p7 * eq184_e2315_d_n4);
        let eq184_e2316_d_n5: f64 = (p.p7 * eq184_e2315_d_n5);
        let eq184_e2316_d_n6: f64 = (p.p7 * eq184_e2315_d_n6);
        let eq184_e2316_d_n7: f64 = (p.p7 * eq184_e2315_d_n7);
        let eq184_e2316_d_n8: f64 = (p.p7 * eq184_e2315_d_n8);
        let eq184_e2316_d_n9: f64 = (p.p7 * eq184_e2315_d_n9);
        let eq184_e2316_d_n10: f64 = (p.p7 * eq184_e2315_d_n10);
        let eq184_e2316_d_n11: f64 = (p.p7 * eq184_e2315_d_n11);
        let eq184_e2316_d_n12: f64 = (p.p7 * eq184_e2315_d_n12);
        let eq184_e2316_d_n13: f64 = (p.p7 * eq184_e2315_d_n13);
        let eq184_e2316_d_n14: f64 = (p.p7 * eq184_e2315_d_n14);
        let eq184_e2316_d_n15: f64 = (p.p7 * eq184_e2315_d_n15);
        let eq184_e2316_d_n16: f64 = (p.p7 * eq184_e2315_d_n16);
        let eq184_e2316_d_n17: f64 = (p.p7 * eq184_e2315_d_n17);
        let eq184_e2316_d_n18: f64 = (p.p7 * eq184_e2315_d_n18);
        let eq184_e2316_d_n19: f64 = (p.p7 * eq184_e2315_d_n19);
        let eq184_e2316_d_n20: f64 = (p.p7 * eq184_e2315_d_n20);
        let eq184_e2316_d_n21: f64 = (p.p7 * eq184_e2315_d_n21);
        let eq184_e2316_d_n22: f64 = (p.p7 * eq184_e2315_d_n22);
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
        (eq184_e2318, eq184_e2318_d_n0, eq184_e2318_d_n1, eq184_e2318_d_n2, eq184_e2318_d_n3, eq184_e2318_d_n4, eq184_e2318_d_n5, eq184_e2318_d_n6, eq184_e2318_d_n7, eq184_e2318_d_n8, eq184_e2318_d_n9, eq184_e2318_d_n10, eq184_e2318_d_n11, eq184_e2318_d_n12, eq184_e2318_d_n13, eq184_e2318_d_n14, eq184_e2318_d_n15, eq184_e2318_d_n16, eq184_e2318_d_n17, eq184_e2318_d_n18, eq184_e2318_d_n19, eq184_e2318_d_n20, eq184_e2318_d_n21, eq184_e2318_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq184_value: f64 = eq184_e2320;
        let eq184_node_derivatives: [f64; 23] = [eq184_e2320_d_n0, eq184_e2320_d_n1, eq184_e2320_d_n2, eq184_e2320_d_n3, eq184_e2320_d_n4, eq184_e2320_d_n5, eq184_e2320_d_n6, eq184_e2320_d_n7, eq184_e2320_d_n8, eq184_e2320_d_n9, eq184_e2320_d_n10, eq184_e2320_d_n11, eq184_e2320_d_n12, eq184_e2320_d_n13, eq184_e2320_d_n14, eq184_e2320_d_n15, eq184_e2320_d_n16, eq184_e2320_d_n17, eq184_e2320_d_n18, eq184_e2320_d_n19, eq184_e2320_d_n20, eq184_e2320_d_n21, eq184_e2320_d_n22];
        let eq184_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[21]),
            multiplicity * (eq184_value),
            nodes,
            &eq184_node_derivatives,
            branches,
            &eq184_branch_derivatives,
            multiplicity,
        );
        let (eq185_e2331, eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22,) = {
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
        let eq185_e2328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 84, eq185_e2327);
        let eq185_e2328_d_n0: f64 = (eq185_e2327_d_n0 * ddt_scale);
        let eq185_e2328_d_n1: f64 = (eq185_e2327_d_n1 * ddt_scale);
        let eq185_e2328_d_n2: f64 = (eq185_e2327_d_n2 * ddt_scale);
        let eq185_e2328_d_n3: f64 = (eq185_e2327_d_n3 * ddt_scale);
        let eq185_e2328_d_n4: f64 = (eq185_e2327_d_n4 * ddt_scale);
        let eq185_e2328_d_n5: f64 = (eq185_e2327_d_n5 * ddt_scale);
        let eq185_e2328_d_n6: f64 = (eq185_e2327_d_n6 * ddt_scale);
        let eq185_e2328_d_n7: f64 = (eq185_e2327_d_n7 * ddt_scale);
        let eq185_e2328_d_n8: f64 = (eq185_e2327_d_n8 * ddt_scale);
        let eq185_e2328_d_n9: f64 = (eq185_e2327_d_n9 * ddt_scale);
        let eq185_e2328_d_n10: f64 = (eq185_e2327_d_n10 * ddt_scale);
        let eq185_e2328_d_n11: f64 = (eq185_e2327_d_n11 * ddt_scale);
        let eq185_e2328_d_n12: f64 = (eq185_e2327_d_n12 * ddt_scale);
        let eq185_e2328_d_n13: f64 = (eq185_e2327_d_n13 * ddt_scale);
        let eq185_e2328_d_n14: f64 = (eq185_e2327_d_n14 * ddt_scale);
        let eq185_e2328_d_n15: f64 = (eq185_e2327_d_n15 * ddt_scale);
        let eq185_e2328_d_n16: f64 = (eq185_e2327_d_n16 * ddt_scale);
        let eq185_e2328_d_n17: f64 = (eq185_e2327_d_n17 * ddt_scale);
        let eq185_e2328_d_n18: f64 = (eq185_e2327_d_n18 * ddt_scale);
        let eq185_e2328_d_n19: f64 = (eq185_e2327_d_n19 * ddt_scale);
        let eq185_e2328_d_n20: f64 = (eq185_e2327_d_n20 * ddt_scale);
        let eq185_e2328_d_n21: f64 = (eq185_e2327_d_n21 * ddt_scale);
        let eq185_e2328_d_n22: f64 = (eq185_e2327_d_n22 * ddt_scale);
        let eq185_e2329: f64 = (p.p7 * eq185_e2328);
        let eq185_e2329_d_n0: f64 = (p.p7 * eq185_e2328_d_n0);
        let eq185_e2329_d_n1: f64 = (p.p7 * eq185_e2328_d_n1);
        let eq185_e2329_d_n2: f64 = (p.p7 * eq185_e2328_d_n2);
        let eq185_e2329_d_n3: f64 = (p.p7 * eq185_e2328_d_n3);
        let eq185_e2329_d_n4: f64 = (p.p7 * eq185_e2328_d_n4);
        let eq185_e2329_d_n5: f64 = (p.p7 * eq185_e2328_d_n5);
        let eq185_e2329_d_n6: f64 = (p.p7 * eq185_e2328_d_n6);
        let eq185_e2329_d_n7: f64 = (p.p7 * eq185_e2328_d_n7);
        let eq185_e2329_d_n8: f64 = (p.p7 * eq185_e2328_d_n8);
        let eq185_e2329_d_n9: f64 = (p.p7 * eq185_e2328_d_n9);
        let eq185_e2329_d_n10: f64 = (p.p7 * eq185_e2328_d_n10);
        let eq185_e2329_d_n11: f64 = (p.p7 * eq185_e2328_d_n11);
        let eq185_e2329_d_n12: f64 = (p.p7 * eq185_e2328_d_n12);
        let eq185_e2329_d_n13: f64 = (p.p7 * eq185_e2328_d_n13);
        let eq185_e2329_d_n14: f64 = (p.p7 * eq185_e2328_d_n14);
        let eq185_e2329_d_n15: f64 = (p.p7 * eq185_e2328_d_n15);
        let eq185_e2329_d_n16: f64 = (p.p7 * eq185_e2328_d_n16);
        let eq185_e2329_d_n17: f64 = (p.p7 * eq185_e2328_d_n17);
        let eq185_e2329_d_n18: f64 = (p.p7 * eq185_e2328_d_n18);
        let eq185_e2329_d_n19: f64 = (p.p7 * eq185_e2328_d_n19);
        let eq185_e2329_d_n20: f64 = (p.p7 * eq185_e2328_d_n20);
        let eq185_e2329_d_n21: f64 = (p.p7 * eq185_e2328_d_n21);
        let eq185_e2329_d_n22: f64 = (p.p7 * eq185_e2328_d_n22);
        (eq185_e2329, eq185_e2329_d_n0, eq185_e2329_d_n1, eq185_e2329_d_n2, eq185_e2329_d_n3, eq185_e2329_d_n4, eq185_e2329_d_n5, eq185_e2329_d_n6, eq185_e2329_d_n7, eq185_e2329_d_n8, eq185_e2329_d_n9, eq185_e2329_d_n10, eq185_e2329_d_n11, eq185_e2329_d_n12, eq185_e2329_d_n13, eq185_e2329_d_n14, eq185_e2329_d_n15, eq185_e2329_d_n16, eq185_e2329_d_n17, eq185_e2329_d_n18, eq185_e2329_d_n19, eq185_e2329_d_n20, eq185_e2329_d_n21, eq185_e2329_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq185_value: f64 = eq185_e2331;
        let eq185_node_derivatives: [f64; 23] = [eq185_e2331_d_n0, eq185_e2331_d_n1, eq185_e2331_d_n2, eq185_e2331_d_n3, eq185_e2331_d_n4, eq185_e2331_d_n5, eq185_e2331_d_n6, eq185_e2331_d_n7, eq185_e2331_d_n8, eq185_e2331_d_n9, eq185_e2331_d_n10, eq185_e2331_d_n11, eq185_e2331_d_n12, eq185_e2331_d_n13, eq185_e2331_d_n14, eq185_e2331_d_n15, eq185_e2331_d_n16, eq185_e2331_d_n17, eq185_e2331_d_n18, eq185_e2331_d_n19, eq185_e2331_d_n20, eq185_e2331_d_n21, eq185_e2331_d_n22];
        let eq185_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[21]),
            multiplicity * (eq185_value),
            nodes,
            &eq185_node_derivatives,
            branches,
            &eq185_branch_derivatives,
            multiplicity,
        );
        let (eq186_e2341, eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22,) = {
    if ((!s.b[595]) && s.b[598]) {
        let eq186_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 85, s.v[289]);
        let eq186_e2338_d_n0: f64 = (s.dn[289][0] * ddt_scale);
        let eq186_e2338_d_n1: f64 = (s.dn[289][1] * ddt_scale);
        let eq186_e2338_d_n2: f64 = (s.dn[289][2] * ddt_scale);
        let eq186_e2338_d_n3: f64 = (s.dn[289][3] * ddt_scale);
        let eq186_e2338_d_n4: f64 = (s.dn[289][4] * ddt_scale);
        let eq186_e2338_d_n5: f64 = (s.dn[289][5] * ddt_scale);
        let eq186_e2338_d_n6: f64 = (s.dn[289][6] * ddt_scale);
        let eq186_e2338_d_n7: f64 = (s.dn[289][7] * ddt_scale);
        let eq186_e2338_d_n8: f64 = (s.dn[289][8] * ddt_scale);
        let eq186_e2338_d_n9: f64 = (s.dn[289][9] * ddt_scale);
        let eq186_e2338_d_n10: f64 = (s.dn[289][10] * ddt_scale);
        let eq186_e2338_d_n11: f64 = (s.dn[289][11] * ddt_scale);
        let eq186_e2338_d_n12: f64 = (s.dn[289][12] * ddt_scale);
        let eq186_e2338_d_n13: f64 = (s.dn[289][13] * ddt_scale);
        let eq186_e2338_d_n14: f64 = (s.dn[289][14] * ddt_scale);
        let eq186_e2338_d_n15: f64 = (s.dn[289][15] * ddt_scale);
        let eq186_e2338_d_n16: f64 = (s.dn[289][16] * ddt_scale);
        let eq186_e2338_d_n17: f64 = (s.dn[289][17] * ddt_scale);
        let eq186_e2338_d_n18: f64 = (s.dn[289][18] * ddt_scale);
        let eq186_e2338_d_n19: f64 = (s.dn[289][19] * ddt_scale);
        let eq186_e2338_d_n20: f64 = (s.dn[289][20] * ddt_scale);
        let eq186_e2338_d_n21: f64 = (s.dn[289][21] * ddt_scale);
        let eq186_e2338_d_n22: f64 = (s.dn[289][22] * ddt_scale);
        let eq186_e2339: f64 = (p.p7 * eq186_e2338);
        let eq186_e2339_d_n0: f64 = (p.p7 * eq186_e2338_d_n0);
        let eq186_e2339_d_n1: f64 = (p.p7 * eq186_e2338_d_n1);
        let eq186_e2339_d_n2: f64 = (p.p7 * eq186_e2338_d_n2);
        let eq186_e2339_d_n3: f64 = (p.p7 * eq186_e2338_d_n3);
        let eq186_e2339_d_n4: f64 = (p.p7 * eq186_e2338_d_n4);
        let eq186_e2339_d_n5: f64 = (p.p7 * eq186_e2338_d_n5);
        let eq186_e2339_d_n6: f64 = (p.p7 * eq186_e2338_d_n6);
        let eq186_e2339_d_n7: f64 = (p.p7 * eq186_e2338_d_n7);
        let eq186_e2339_d_n8: f64 = (p.p7 * eq186_e2338_d_n8);
        let eq186_e2339_d_n9: f64 = (p.p7 * eq186_e2338_d_n9);
        let eq186_e2339_d_n10: f64 = (p.p7 * eq186_e2338_d_n10);
        let eq186_e2339_d_n11: f64 = (p.p7 * eq186_e2338_d_n11);
        let eq186_e2339_d_n12: f64 = (p.p7 * eq186_e2338_d_n12);
        let eq186_e2339_d_n13: f64 = (p.p7 * eq186_e2338_d_n13);
        let eq186_e2339_d_n14: f64 = (p.p7 * eq186_e2338_d_n14);
        let eq186_e2339_d_n15: f64 = (p.p7 * eq186_e2338_d_n15);
        let eq186_e2339_d_n16: f64 = (p.p7 * eq186_e2338_d_n16);
        let eq186_e2339_d_n17: f64 = (p.p7 * eq186_e2338_d_n17);
        let eq186_e2339_d_n18: f64 = (p.p7 * eq186_e2338_d_n18);
        let eq186_e2339_d_n19: f64 = (p.p7 * eq186_e2338_d_n19);
        let eq186_e2339_d_n20: f64 = (p.p7 * eq186_e2338_d_n20);
        let eq186_e2339_d_n21: f64 = (p.p7 * eq186_e2338_d_n21);
        let eq186_e2339_d_n22: f64 = (p.p7 * eq186_e2338_d_n22);
        (eq186_e2339, eq186_e2339_d_n0, eq186_e2339_d_n1, eq186_e2339_d_n2, eq186_e2339_d_n3, eq186_e2339_d_n4, eq186_e2339_d_n5, eq186_e2339_d_n6, eq186_e2339_d_n7, eq186_e2339_d_n8, eq186_e2339_d_n9, eq186_e2339_d_n10, eq186_e2339_d_n11, eq186_e2339_d_n12, eq186_e2339_d_n13, eq186_e2339_d_n14, eq186_e2339_d_n15, eq186_e2339_d_n16, eq186_e2339_d_n17, eq186_e2339_d_n18, eq186_e2339_d_n19, eq186_e2339_d_n20, eq186_e2339_d_n21, eq186_e2339_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq186_value: f64 = eq186_e2341;
        let eq186_node_derivatives: [f64; 23] = [eq186_e2341_d_n0, eq186_e2341_d_n1, eq186_e2341_d_n2, eq186_e2341_d_n3, eq186_e2341_d_n4, eq186_e2341_d_n5, eq186_e2341_d_n6, eq186_e2341_d_n7, eq186_e2341_d_n8, eq186_e2341_d_n9, eq186_e2341_d_n10, eq186_e2341_d_n11, eq186_e2341_d_n12, eq186_e2341_d_n13, eq186_e2341_d_n14, eq186_e2341_d_n15, eq186_e2341_d_n16, eq186_e2341_d_n17, eq186_e2341_d_n18, eq186_e2341_d_n19, eq186_e2341_d_n20, eq186_e2341_d_n21, eq186_e2341_d_n22];
        let eq186_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            multiplicity * (eq186_value),
            nodes,
            &eq186_node_derivatives,
            branches,
            &eq186_branch_derivatives,
            multiplicity,
        );
        let (eq187_e2353, eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && s.b[599]) {
        let eq187_e2350: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 86, s.v[288]);
        let eq187_e2350_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq187_e2350_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq187_e2350_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq187_e2350_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq187_e2350_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq187_e2350_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq187_e2350_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq187_e2350_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq187_e2350_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq187_e2350_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq187_e2350_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq187_e2350_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq187_e2350_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq187_e2350_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq187_e2350_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq187_e2350_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq187_e2350_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq187_e2350_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq187_e2350_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq187_e2350_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq187_e2350_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq187_e2350_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq187_e2350_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq187_e2351: f64 = (p.p7 * eq187_e2350);
        let eq187_e2351_d_n0: f64 = (p.p7 * eq187_e2350_d_n0);
        let eq187_e2351_d_n1: f64 = (p.p7 * eq187_e2350_d_n1);
        let eq187_e2351_d_n2: f64 = (p.p7 * eq187_e2350_d_n2);
        let eq187_e2351_d_n3: f64 = (p.p7 * eq187_e2350_d_n3);
        let eq187_e2351_d_n4: f64 = (p.p7 * eq187_e2350_d_n4);
        let eq187_e2351_d_n5: f64 = (p.p7 * eq187_e2350_d_n5);
        let eq187_e2351_d_n6: f64 = (p.p7 * eq187_e2350_d_n6);
        let eq187_e2351_d_n7: f64 = (p.p7 * eq187_e2350_d_n7);
        let eq187_e2351_d_n8: f64 = (p.p7 * eq187_e2350_d_n8);
        let eq187_e2351_d_n9: f64 = (p.p7 * eq187_e2350_d_n9);
        let eq187_e2351_d_n10: f64 = (p.p7 * eq187_e2350_d_n10);
        let eq187_e2351_d_n11: f64 = (p.p7 * eq187_e2350_d_n11);
        let eq187_e2351_d_n12: f64 = (p.p7 * eq187_e2350_d_n12);
        let eq187_e2351_d_n13: f64 = (p.p7 * eq187_e2350_d_n13);
        let eq187_e2351_d_n14: f64 = (p.p7 * eq187_e2350_d_n14);
        let eq187_e2351_d_n15: f64 = (p.p7 * eq187_e2350_d_n15);
        let eq187_e2351_d_n16: f64 = (p.p7 * eq187_e2350_d_n16);
        let eq187_e2351_d_n17: f64 = (p.p7 * eq187_e2350_d_n17);
        let eq187_e2351_d_n18: f64 = (p.p7 * eq187_e2350_d_n18);
        let eq187_e2351_d_n19: f64 = (p.p7 * eq187_e2350_d_n19);
        let eq187_e2351_d_n20: f64 = (p.p7 * eq187_e2350_d_n20);
        let eq187_e2351_d_n21: f64 = (p.p7 * eq187_e2350_d_n21);
        let eq187_e2351_d_n22: f64 = (p.p7 * eq187_e2350_d_n22);
        (eq187_e2351, eq187_e2351_d_n0, eq187_e2351_d_n1, eq187_e2351_d_n2, eq187_e2351_d_n3, eq187_e2351_d_n4, eq187_e2351_d_n5, eq187_e2351_d_n6, eq187_e2351_d_n7, eq187_e2351_d_n8, eq187_e2351_d_n9, eq187_e2351_d_n10, eq187_e2351_d_n11, eq187_e2351_d_n12, eq187_e2351_d_n13, eq187_e2351_d_n14, eq187_e2351_d_n15, eq187_e2351_d_n16, eq187_e2351_d_n17, eq187_e2351_d_n18, eq187_e2351_d_n19, eq187_e2351_d_n20, eq187_e2351_d_n21, eq187_e2351_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq187_value: f64 = eq187_e2353;
        let eq187_node_derivatives: [f64; 23] = [eq187_e2353_d_n0, eq187_e2353_d_n1, eq187_e2353_d_n2, eq187_e2353_d_n3, eq187_e2353_d_n4, eq187_e2353_d_n5, eq187_e2353_d_n6, eq187_e2353_d_n7, eq187_e2353_d_n8, eq187_e2353_d_n9, eq187_e2353_d_n10, eq187_e2353_d_n11, eq187_e2353_d_n12, eq187_e2353_d_n13, eq187_e2353_d_n14, eq187_e2353_d_n15, eq187_e2353_d_n16, eq187_e2353_d_n17, eq187_e2353_d_n18, eq187_e2353_d_n19, eq187_e2353_d_n20, eq187_e2353_d_n21, eq187_e2353_d_n22];
        let eq187_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            multiplicity * (eq187_value),
            nodes,
            &eq187_node_derivatives,
            branches,
            &eq187_branch_derivatives,
            multiplicity,
        );
        let (eq188_e2367, eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && s.b[599]) {
        let eq188_e2362: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 87, s.v[288]);
        let eq188_e2362_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq188_e2362_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq188_e2362_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq188_e2362_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq188_e2362_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq188_e2362_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq188_e2362_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq188_e2362_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq188_e2362_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq188_e2362_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq188_e2362_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq188_e2362_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq188_e2362_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq188_e2362_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq188_e2362_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq188_e2362_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq188_e2362_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq188_e2362_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq188_e2362_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq188_e2362_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq188_e2362_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq188_e2362_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq188_e2362_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq188_e2363: f64 = (p.p7 * eq188_e2362);
        let eq188_e2363_d_n0: f64 = (p.p7 * eq188_e2362_d_n0);
        let eq188_e2363_d_n1: f64 = (p.p7 * eq188_e2362_d_n1);
        let eq188_e2363_d_n2: f64 = (p.p7 * eq188_e2362_d_n2);
        let eq188_e2363_d_n3: f64 = (p.p7 * eq188_e2362_d_n3);
        let eq188_e2363_d_n4: f64 = (p.p7 * eq188_e2362_d_n4);
        let eq188_e2363_d_n5: f64 = (p.p7 * eq188_e2362_d_n5);
        let eq188_e2363_d_n6: f64 = (p.p7 * eq188_e2362_d_n6);
        let eq188_e2363_d_n7: f64 = (p.p7 * eq188_e2362_d_n7);
        let eq188_e2363_d_n8: f64 = (p.p7 * eq188_e2362_d_n8);
        let eq188_e2363_d_n9: f64 = (p.p7 * eq188_e2362_d_n9);
        let eq188_e2363_d_n10: f64 = (p.p7 * eq188_e2362_d_n10);
        let eq188_e2363_d_n11: f64 = (p.p7 * eq188_e2362_d_n11);
        let eq188_e2363_d_n12: f64 = (p.p7 * eq188_e2362_d_n12);
        let eq188_e2363_d_n13: f64 = (p.p7 * eq188_e2362_d_n13);
        let eq188_e2363_d_n14: f64 = (p.p7 * eq188_e2362_d_n14);
        let eq188_e2363_d_n15: f64 = (p.p7 * eq188_e2362_d_n15);
        let eq188_e2363_d_n16: f64 = (p.p7 * eq188_e2362_d_n16);
        let eq188_e2363_d_n17: f64 = (p.p7 * eq188_e2362_d_n17);
        let eq188_e2363_d_n18: f64 = (p.p7 * eq188_e2362_d_n18);
        let eq188_e2363_d_n19: f64 = (p.p7 * eq188_e2362_d_n19);
        let eq188_e2363_d_n20: f64 = (p.p7 * eq188_e2362_d_n20);
        let eq188_e2363_d_n21: f64 = (p.p7 * eq188_e2362_d_n21);
        let eq188_e2363_d_n22: f64 = (p.p7 * eq188_e2362_d_n22);
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
        (eq188_e2365, eq188_e2365_d_n0, eq188_e2365_d_n1, eq188_e2365_d_n2, eq188_e2365_d_n3, eq188_e2365_d_n4, eq188_e2365_d_n5, eq188_e2365_d_n6, eq188_e2365_d_n7, eq188_e2365_d_n8, eq188_e2365_d_n9, eq188_e2365_d_n10, eq188_e2365_d_n11, eq188_e2365_d_n12, eq188_e2365_d_n13, eq188_e2365_d_n14, eq188_e2365_d_n15, eq188_e2365_d_n16, eq188_e2365_d_n17, eq188_e2365_d_n18, eq188_e2365_d_n19, eq188_e2365_d_n20, eq188_e2365_d_n21, eq188_e2365_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq188_value: f64 = eq188_e2367;
        let eq188_node_derivatives: [f64; 23] = [eq188_e2367_d_n0, eq188_e2367_d_n1, eq188_e2367_d_n2, eq188_e2367_d_n3, eq188_e2367_d_n4, eq188_e2367_d_n5, eq188_e2367_d_n6, eq188_e2367_d_n7, eq188_e2367_d_n8, eq188_e2367_d_n9, eq188_e2367_d_n10, eq188_e2367_d_n11, eq188_e2367_d_n12, eq188_e2367_d_n13, eq188_e2367_d_n14, eq188_e2367_d_n15, eq188_e2367_d_n16, eq188_e2367_d_n17, eq188_e2367_d_n18, eq188_e2367_d_n19, eq188_e2367_d_n20, eq188_e2367_d_n21, eq188_e2367_d_n22];
        let eq188_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq188_value),
            nodes,
            &eq188_node_derivatives,
            branches,
            &eq188_branch_derivatives,
            multiplicity,
        );
        let (eq189_e2380, eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && (!s.b[599])) {
        let eq189_e2377: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 88, s.v[288]);
        let eq189_e2377_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq189_e2377_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq189_e2377_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq189_e2377_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq189_e2377_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq189_e2377_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq189_e2377_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq189_e2377_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq189_e2377_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq189_e2377_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq189_e2377_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq189_e2377_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq189_e2377_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq189_e2377_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq189_e2377_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq189_e2377_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq189_e2377_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq189_e2377_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq189_e2377_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq189_e2377_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq189_e2377_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq189_e2377_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq189_e2377_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq189_e2378: f64 = (p.p7 * eq189_e2377);
        let eq189_e2378_d_n0: f64 = (p.p7 * eq189_e2377_d_n0);
        let eq189_e2378_d_n1: f64 = (p.p7 * eq189_e2377_d_n1);
        let eq189_e2378_d_n2: f64 = (p.p7 * eq189_e2377_d_n2);
        let eq189_e2378_d_n3: f64 = (p.p7 * eq189_e2377_d_n3);
        let eq189_e2378_d_n4: f64 = (p.p7 * eq189_e2377_d_n4);
        let eq189_e2378_d_n5: f64 = (p.p7 * eq189_e2377_d_n5);
        let eq189_e2378_d_n6: f64 = (p.p7 * eq189_e2377_d_n6);
        let eq189_e2378_d_n7: f64 = (p.p7 * eq189_e2377_d_n7);
        let eq189_e2378_d_n8: f64 = (p.p7 * eq189_e2377_d_n8);
        let eq189_e2378_d_n9: f64 = (p.p7 * eq189_e2377_d_n9);
        let eq189_e2378_d_n10: f64 = (p.p7 * eq189_e2377_d_n10);
        let eq189_e2378_d_n11: f64 = (p.p7 * eq189_e2377_d_n11);
        let eq189_e2378_d_n12: f64 = (p.p7 * eq189_e2377_d_n12);
        let eq189_e2378_d_n13: f64 = (p.p7 * eq189_e2377_d_n13);
        let eq189_e2378_d_n14: f64 = (p.p7 * eq189_e2377_d_n14);
        let eq189_e2378_d_n15: f64 = (p.p7 * eq189_e2377_d_n15);
        let eq189_e2378_d_n16: f64 = (p.p7 * eq189_e2377_d_n16);
        let eq189_e2378_d_n17: f64 = (p.p7 * eq189_e2377_d_n17);
        let eq189_e2378_d_n18: f64 = (p.p7 * eq189_e2377_d_n18);
        let eq189_e2378_d_n19: f64 = (p.p7 * eq189_e2377_d_n19);
        let eq189_e2378_d_n20: f64 = (p.p7 * eq189_e2377_d_n20);
        let eq189_e2378_d_n21: f64 = (p.p7 * eq189_e2377_d_n21);
        let eq189_e2378_d_n22: f64 = (p.p7 * eq189_e2377_d_n22);
        (eq189_e2378, eq189_e2378_d_n0, eq189_e2378_d_n1, eq189_e2378_d_n2, eq189_e2378_d_n3, eq189_e2378_d_n4, eq189_e2378_d_n5, eq189_e2378_d_n6, eq189_e2378_d_n7, eq189_e2378_d_n8, eq189_e2378_d_n9, eq189_e2378_d_n10, eq189_e2378_d_n11, eq189_e2378_d_n12, eq189_e2378_d_n13, eq189_e2378_d_n14, eq189_e2378_d_n15, eq189_e2378_d_n16, eq189_e2378_d_n17, eq189_e2378_d_n18, eq189_e2378_d_n19, eq189_e2378_d_n20, eq189_e2378_d_n21, eq189_e2378_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq189_value: f64 = eq189_e2380;
        let eq189_node_derivatives: [f64; 23] = [eq189_e2380_d_n0, eq189_e2380_d_n1, eq189_e2380_d_n2, eq189_e2380_d_n3, eq189_e2380_d_n4, eq189_e2380_d_n5, eq189_e2380_d_n6, eq189_e2380_d_n7, eq189_e2380_d_n8, eq189_e2380_d_n9, eq189_e2380_d_n10, eq189_e2380_d_n11, eq189_e2380_d_n12, eq189_e2380_d_n13, eq189_e2380_d_n14, eq189_e2380_d_n15, eq189_e2380_d_n16, eq189_e2380_d_n17, eq189_e2380_d_n18, eq189_e2380_d_n19, eq189_e2380_d_n20, eq189_e2380_d_n21, eq189_e2380_d_n22];
        let eq189_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq189_value),
            nodes,
            &eq189_node_derivatives,
            branches,
            &eq189_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_21(
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
        let (eq190_e2395, eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22,) = {
    if (((!s.b[595]) && s.b[598]) && (!s.b[599])) {
        let eq190_e2390: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 89, s.v[288]);
        let eq190_e2390_d_n0: f64 = (s.dn[288][0] * ddt_scale);
        let eq190_e2390_d_n1: f64 = (s.dn[288][1] * ddt_scale);
        let eq190_e2390_d_n2: f64 = (s.dn[288][2] * ddt_scale);
        let eq190_e2390_d_n3: f64 = (s.dn[288][3] * ddt_scale);
        let eq190_e2390_d_n4: f64 = (s.dn[288][4] * ddt_scale);
        let eq190_e2390_d_n5: f64 = (s.dn[288][5] * ddt_scale);
        let eq190_e2390_d_n6: f64 = (s.dn[288][6] * ddt_scale);
        let eq190_e2390_d_n7: f64 = (s.dn[288][7] * ddt_scale);
        let eq190_e2390_d_n8: f64 = (s.dn[288][8] * ddt_scale);
        let eq190_e2390_d_n9: f64 = (s.dn[288][9] * ddt_scale);
        let eq190_e2390_d_n10: f64 = (s.dn[288][10] * ddt_scale);
        let eq190_e2390_d_n11: f64 = (s.dn[288][11] * ddt_scale);
        let eq190_e2390_d_n12: f64 = (s.dn[288][12] * ddt_scale);
        let eq190_e2390_d_n13: f64 = (s.dn[288][13] * ddt_scale);
        let eq190_e2390_d_n14: f64 = (s.dn[288][14] * ddt_scale);
        let eq190_e2390_d_n15: f64 = (s.dn[288][15] * ddt_scale);
        let eq190_e2390_d_n16: f64 = (s.dn[288][16] * ddt_scale);
        let eq190_e2390_d_n17: f64 = (s.dn[288][17] * ddt_scale);
        let eq190_e2390_d_n18: f64 = (s.dn[288][18] * ddt_scale);
        let eq190_e2390_d_n19: f64 = (s.dn[288][19] * ddt_scale);
        let eq190_e2390_d_n20: f64 = (s.dn[288][20] * ddt_scale);
        let eq190_e2390_d_n21: f64 = (s.dn[288][21] * ddt_scale);
        let eq190_e2390_d_n22: f64 = (s.dn[288][22] * ddt_scale);
        let eq190_e2391: f64 = (p.p7 * eq190_e2390);
        let eq190_e2391_d_n0: f64 = (p.p7 * eq190_e2390_d_n0);
        let eq190_e2391_d_n1: f64 = (p.p7 * eq190_e2390_d_n1);
        let eq190_e2391_d_n2: f64 = (p.p7 * eq190_e2390_d_n2);
        let eq190_e2391_d_n3: f64 = (p.p7 * eq190_e2390_d_n3);
        let eq190_e2391_d_n4: f64 = (p.p7 * eq190_e2390_d_n4);
        let eq190_e2391_d_n5: f64 = (p.p7 * eq190_e2390_d_n5);
        let eq190_e2391_d_n6: f64 = (p.p7 * eq190_e2390_d_n6);
        let eq190_e2391_d_n7: f64 = (p.p7 * eq190_e2390_d_n7);
        let eq190_e2391_d_n8: f64 = (p.p7 * eq190_e2390_d_n8);
        let eq190_e2391_d_n9: f64 = (p.p7 * eq190_e2390_d_n9);
        let eq190_e2391_d_n10: f64 = (p.p7 * eq190_e2390_d_n10);
        let eq190_e2391_d_n11: f64 = (p.p7 * eq190_e2390_d_n11);
        let eq190_e2391_d_n12: f64 = (p.p7 * eq190_e2390_d_n12);
        let eq190_e2391_d_n13: f64 = (p.p7 * eq190_e2390_d_n13);
        let eq190_e2391_d_n14: f64 = (p.p7 * eq190_e2390_d_n14);
        let eq190_e2391_d_n15: f64 = (p.p7 * eq190_e2390_d_n15);
        let eq190_e2391_d_n16: f64 = (p.p7 * eq190_e2390_d_n16);
        let eq190_e2391_d_n17: f64 = (p.p7 * eq190_e2390_d_n17);
        let eq190_e2391_d_n18: f64 = (p.p7 * eq190_e2390_d_n18);
        let eq190_e2391_d_n19: f64 = (p.p7 * eq190_e2390_d_n19);
        let eq190_e2391_d_n20: f64 = (p.p7 * eq190_e2390_d_n20);
        let eq190_e2391_d_n21: f64 = (p.p7 * eq190_e2390_d_n21);
        let eq190_e2391_d_n22: f64 = (p.p7 * eq190_e2390_d_n22);
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
        (eq190_e2393, eq190_e2393_d_n0, eq190_e2393_d_n1, eq190_e2393_d_n2, eq190_e2393_d_n3, eq190_e2393_d_n4, eq190_e2393_d_n5, eq190_e2393_d_n6, eq190_e2393_d_n7, eq190_e2393_d_n8, eq190_e2393_d_n9, eq190_e2393_d_n10, eq190_e2393_d_n11, eq190_e2393_d_n12, eq190_e2393_d_n13, eq190_e2393_d_n14, eq190_e2393_d_n15, eq190_e2393_d_n16, eq190_e2393_d_n17, eq190_e2393_d_n18, eq190_e2393_d_n19, eq190_e2393_d_n20, eq190_e2393_d_n21, eq190_e2393_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq190_value: f64 = eq190_e2395;
        let eq190_node_derivatives: [f64; 23] = [eq190_e2395_d_n0, eq190_e2395_d_n1, eq190_e2395_d_n2, eq190_e2395_d_n3, eq190_e2395_d_n4, eq190_e2395_d_n5, eq190_e2395_d_n6, eq190_e2395_d_n7, eq190_e2395_d_n8, eq190_e2395_d_n9, eq190_e2395_d_n10, eq190_e2395_d_n11, eq190_e2395_d_n12, eq190_e2395_d_n13, eq190_e2395_d_n14, eq190_e2395_d_n15, eq190_e2395_d_n16, eq190_e2395_d_n17, eq190_e2395_d_n18, eq190_e2395_d_n19, eq190_e2395_d_n20, eq190_e2395_d_n21, eq190_e2395_d_n22];
        let eq190_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            multiplicity * (eq190_value),
            nodes,
            &eq190_node_derivatives,
            branches,
            &eq190_branch_derivatives,
            multiplicity,
        );
        let (eq191_e2407, eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22,) = {
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
        let eq191_e2404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 90, eq191_e2403);
        let eq191_e2404_d_n0: f64 = (eq191_e2403_d_n0 * ddt_scale);
        let eq191_e2404_d_n1: f64 = (eq191_e2403_d_n1 * ddt_scale);
        let eq191_e2404_d_n2: f64 = (eq191_e2403_d_n2 * ddt_scale);
        let eq191_e2404_d_n3: f64 = (eq191_e2403_d_n3 * ddt_scale);
        let eq191_e2404_d_n4: f64 = (eq191_e2403_d_n4 * ddt_scale);
        let eq191_e2404_d_n5: f64 = (eq191_e2403_d_n5 * ddt_scale);
        let eq191_e2404_d_n6: f64 = (eq191_e2403_d_n6 * ddt_scale);
        let eq191_e2404_d_n7: f64 = (eq191_e2403_d_n7 * ddt_scale);
        let eq191_e2404_d_n8: f64 = (eq191_e2403_d_n8 * ddt_scale);
        let eq191_e2404_d_n9: f64 = (eq191_e2403_d_n9 * ddt_scale);
        let eq191_e2404_d_n10: f64 = (eq191_e2403_d_n10 * ddt_scale);
        let eq191_e2404_d_n11: f64 = (eq191_e2403_d_n11 * ddt_scale);
        let eq191_e2404_d_n12: f64 = (eq191_e2403_d_n12 * ddt_scale);
        let eq191_e2404_d_n13: f64 = (eq191_e2403_d_n13 * ddt_scale);
        let eq191_e2404_d_n14: f64 = (eq191_e2403_d_n14 * ddt_scale);
        let eq191_e2404_d_n15: f64 = (eq191_e2403_d_n15 * ddt_scale);
        let eq191_e2404_d_n16: f64 = (eq191_e2403_d_n16 * ddt_scale);
        let eq191_e2404_d_n17: f64 = (eq191_e2403_d_n17 * ddt_scale);
        let eq191_e2404_d_n18: f64 = (eq191_e2403_d_n18 * ddt_scale);
        let eq191_e2404_d_n19: f64 = (eq191_e2403_d_n19 * ddt_scale);
        let eq191_e2404_d_n20: f64 = (eq191_e2403_d_n20 * ddt_scale);
        let eq191_e2404_d_n21: f64 = (eq191_e2403_d_n21 * ddt_scale);
        let eq191_e2404_d_n22: f64 = (eq191_e2403_d_n22 * ddt_scale);
        let eq191_e2405: f64 = (p.p7 * eq191_e2404);
        let eq191_e2405_d_n0: f64 = (p.p7 * eq191_e2404_d_n0);
        let eq191_e2405_d_n1: f64 = (p.p7 * eq191_e2404_d_n1);
        let eq191_e2405_d_n2: f64 = (p.p7 * eq191_e2404_d_n2);
        let eq191_e2405_d_n3: f64 = (p.p7 * eq191_e2404_d_n3);
        let eq191_e2405_d_n4: f64 = (p.p7 * eq191_e2404_d_n4);
        let eq191_e2405_d_n5: f64 = (p.p7 * eq191_e2404_d_n5);
        let eq191_e2405_d_n6: f64 = (p.p7 * eq191_e2404_d_n6);
        let eq191_e2405_d_n7: f64 = (p.p7 * eq191_e2404_d_n7);
        let eq191_e2405_d_n8: f64 = (p.p7 * eq191_e2404_d_n8);
        let eq191_e2405_d_n9: f64 = (p.p7 * eq191_e2404_d_n9);
        let eq191_e2405_d_n10: f64 = (p.p7 * eq191_e2404_d_n10);
        let eq191_e2405_d_n11: f64 = (p.p7 * eq191_e2404_d_n11);
        let eq191_e2405_d_n12: f64 = (p.p7 * eq191_e2404_d_n12);
        let eq191_e2405_d_n13: f64 = (p.p7 * eq191_e2404_d_n13);
        let eq191_e2405_d_n14: f64 = (p.p7 * eq191_e2404_d_n14);
        let eq191_e2405_d_n15: f64 = (p.p7 * eq191_e2404_d_n15);
        let eq191_e2405_d_n16: f64 = (p.p7 * eq191_e2404_d_n16);
        let eq191_e2405_d_n17: f64 = (p.p7 * eq191_e2404_d_n17);
        let eq191_e2405_d_n18: f64 = (p.p7 * eq191_e2404_d_n18);
        let eq191_e2405_d_n19: f64 = (p.p7 * eq191_e2404_d_n19);
        let eq191_e2405_d_n20: f64 = (p.p7 * eq191_e2404_d_n20);
        let eq191_e2405_d_n21: f64 = (p.p7 * eq191_e2404_d_n21);
        let eq191_e2405_d_n22: f64 = (p.p7 * eq191_e2404_d_n22);
        (eq191_e2405, eq191_e2405_d_n0, eq191_e2405_d_n1, eq191_e2405_d_n2, eq191_e2405_d_n3, eq191_e2405_d_n4, eq191_e2405_d_n5, eq191_e2405_d_n6, eq191_e2405_d_n7, eq191_e2405_d_n8, eq191_e2405_d_n9, eq191_e2405_d_n10, eq191_e2405_d_n11, eq191_e2405_d_n12, eq191_e2405_d_n13, eq191_e2405_d_n14, eq191_e2405_d_n15, eq191_e2405_d_n16, eq191_e2405_d_n17, eq191_e2405_d_n18, eq191_e2405_d_n19, eq191_e2405_d_n20, eq191_e2405_d_n21, eq191_e2405_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq191_value: f64 = eq191_e2407;
        let eq191_node_derivatives: [f64; 23] = [eq191_e2407_d_n0, eq191_e2407_d_n1, eq191_e2407_d_n2, eq191_e2407_d_n3, eq191_e2407_d_n4, eq191_e2407_d_n5, eq191_e2407_d_n6, eq191_e2407_d_n7, eq191_e2407_d_n8, eq191_e2407_d_n9, eq191_e2407_d_n10, eq191_e2407_d_n11, eq191_e2407_d_n12, eq191_e2407_d_n13, eq191_e2407_d_n14, eq191_e2407_d_n15, eq191_e2407_d_n16, eq191_e2407_d_n17, eq191_e2407_d_n18, eq191_e2407_d_n19, eq191_e2407_d_n20, eq191_e2407_d_n21, eq191_e2407_d_n22];
        let eq191_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            multiplicity * (eq191_value),
            nodes,
            &eq191_node_derivatives,
            branches,
            &eq191_branch_derivatives,
            multiplicity,
        );
        let (eq192_e2416, eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22,) = {
    if (s.b[600] && s.b[601]) {
        let eq192_e2413: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 91, s.v[301]);
        let eq192_e2413_d_n0: f64 = (s.dn[301][0] * ddt_scale);
        let eq192_e2413_d_n1: f64 = (s.dn[301][1] * ddt_scale);
        let eq192_e2413_d_n2: f64 = (s.dn[301][2] * ddt_scale);
        let eq192_e2413_d_n3: f64 = (s.dn[301][3] * ddt_scale);
        let eq192_e2413_d_n4: f64 = (s.dn[301][4] * ddt_scale);
        let eq192_e2413_d_n5: f64 = (s.dn[301][5] * ddt_scale);
        let eq192_e2413_d_n6: f64 = (s.dn[301][6] * ddt_scale);
        let eq192_e2413_d_n7: f64 = (s.dn[301][7] * ddt_scale);
        let eq192_e2413_d_n8: f64 = (s.dn[301][8] * ddt_scale);
        let eq192_e2413_d_n9: f64 = (s.dn[301][9] * ddt_scale);
        let eq192_e2413_d_n10: f64 = (s.dn[301][10] * ddt_scale);
        let eq192_e2413_d_n11: f64 = (s.dn[301][11] * ddt_scale);
        let eq192_e2413_d_n12: f64 = (s.dn[301][12] * ddt_scale);
        let eq192_e2413_d_n13: f64 = (s.dn[301][13] * ddt_scale);
        let eq192_e2413_d_n14: f64 = (s.dn[301][14] * ddt_scale);
        let eq192_e2413_d_n15: f64 = (s.dn[301][15] * ddt_scale);
        let eq192_e2413_d_n16: f64 = (s.dn[301][16] * ddt_scale);
        let eq192_e2413_d_n17: f64 = (s.dn[301][17] * ddt_scale);
        let eq192_e2413_d_n18: f64 = (s.dn[301][18] * ddt_scale);
        let eq192_e2413_d_n19: f64 = (s.dn[301][19] * ddt_scale);
        let eq192_e2413_d_n20: f64 = (s.dn[301][20] * ddt_scale);
        let eq192_e2413_d_n21: f64 = (s.dn[301][21] * ddt_scale);
        let eq192_e2413_d_n22: f64 = (s.dn[301][22] * ddt_scale);
        let eq192_e2414: f64 = (p.p7 * eq192_e2413);
        let eq192_e2414_d_n0: f64 = (p.p7 * eq192_e2413_d_n0);
        let eq192_e2414_d_n1: f64 = (p.p7 * eq192_e2413_d_n1);
        let eq192_e2414_d_n2: f64 = (p.p7 * eq192_e2413_d_n2);
        let eq192_e2414_d_n3: f64 = (p.p7 * eq192_e2413_d_n3);
        let eq192_e2414_d_n4: f64 = (p.p7 * eq192_e2413_d_n4);
        let eq192_e2414_d_n5: f64 = (p.p7 * eq192_e2413_d_n5);
        let eq192_e2414_d_n6: f64 = (p.p7 * eq192_e2413_d_n6);
        let eq192_e2414_d_n7: f64 = (p.p7 * eq192_e2413_d_n7);
        let eq192_e2414_d_n8: f64 = (p.p7 * eq192_e2413_d_n8);
        let eq192_e2414_d_n9: f64 = (p.p7 * eq192_e2413_d_n9);
        let eq192_e2414_d_n10: f64 = (p.p7 * eq192_e2413_d_n10);
        let eq192_e2414_d_n11: f64 = (p.p7 * eq192_e2413_d_n11);
        let eq192_e2414_d_n12: f64 = (p.p7 * eq192_e2413_d_n12);
        let eq192_e2414_d_n13: f64 = (p.p7 * eq192_e2413_d_n13);
        let eq192_e2414_d_n14: f64 = (p.p7 * eq192_e2413_d_n14);
        let eq192_e2414_d_n15: f64 = (p.p7 * eq192_e2413_d_n15);
        let eq192_e2414_d_n16: f64 = (p.p7 * eq192_e2413_d_n16);
        let eq192_e2414_d_n17: f64 = (p.p7 * eq192_e2413_d_n17);
        let eq192_e2414_d_n18: f64 = (p.p7 * eq192_e2413_d_n18);
        let eq192_e2414_d_n19: f64 = (p.p7 * eq192_e2413_d_n19);
        let eq192_e2414_d_n20: f64 = (p.p7 * eq192_e2413_d_n20);
        let eq192_e2414_d_n21: f64 = (p.p7 * eq192_e2413_d_n21);
        let eq192_e2414_d_n22: f64 = (p.p7 * eq192_e2413_d_n22);
        (eq192_e2414, eq192_e2414_d_n0, eq192_e2414_d_n1, eq192_e2414_d_n2, eq192_e2414_d_n3, eq192_e2414_d_n4, eq192_e2414_d_n5, eq192_e2414_d_n6, eq192_e2414_d_n7, eq192_e2414_d_n8, eq192_e2414_d_n9, eq192_e2414_d_n10, eq192_e2414_d_n11, eq192_e2414_d_n12, eq192_e2414_d_n13, eq192_e2414_d_n14, eq192_e2414_d_n15, eq192_e2414_d_n16, eq192_e2414_d_n17, eq192_e2414_d_n18, eq192_e2414_d_n19, eq192_e2414_d_n20, eq192_e2414_d_n21, eq192_e2414_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq192_value: f64 = eq192_e2416;
        let eq192_node_derivatives: [f64; 23] = [eq192_e2416_d_n0, eq192_e2416_d_n1, eq192_e2416_d_n2, eq192_e2416_d_n3, eq192_e2416_d_n4, eq192_e2416_d_n5, eq192_e2416_d_n6, eq192_e2416_d_n7, eq192_e2416_d_n8, eq192_e2416_d_n9, eq192_e2416_d_n10, eq192_e2416_d_n11, eq192_e2416_d_n12, eq192_e2416_d_n13, eq192_e2416_d_n14, eq192_e2416_d_n15, eq192_e2416_d_n16, eq192_e2416_d_n17, eq192_e2416_d_n18, eq192_e2416_d_n19, eq192_e2416_d_n20, eq192_e2416_d_n21, eq192_e2416_d_n22];
        let eq192_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            multiplicity * (eq192_value),
            nodes,
            &eq192_node_derivatives,
            branches,
            &eq192_branch_derivatives,
            multiplicity,
        );
        let (eq193_e2427, eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22,) = {
    if ((s.b[600] && s.b[601]) && s.b[602]) {
        let eq193_e2424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 92, s.v[300]);
        let eq193_e2424_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq193_e2424_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq193_e2424_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq193_e2424_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq193_e2424_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq193_e2424_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq193_e2424_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq193_e2424_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq193_e2424_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq193_e2424_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq193_e2424_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq193_e2424_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq193_e2424_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq193_e2424_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq193_e2424_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq193_e2424_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq193_e2424_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq193_e2424_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq193_e2424_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq193_e2424_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq193_e2424_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq193_e2424_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq193_e2424_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq193_e2425: f64 = (p.p7 * eq193_e2424);
        let eq193_e2425_d_n0: f64 = (p.p7 * eq193_e2424_d_n0);
        let eq193_e2425_d_n1: f64 = (p.p7 * eq193_e2424_d_n1);
        let eq193_e2425_d_n2: f64 = (p.p7 * eq193_e2424_d_n2);
        let eq193_e2425_d_n3: f64 = (p.p7 * eq193_e2424_d_n3);
        let eq193_e2425_d_n4: f64 = (p.p7 * eq193_e2424_d_n4);
        let eq193_e2425_d_n5: f64 = (p.p7 * eq193_e2424_d_n5);
        let eq193_e2425_d_n6: f64 = (p.p7 * eq193_e2424_d_n6);
        let eq193_e2425_d_n7: f64 = (p.p7 * eq193_e2424_d_n7);
        let eq193_e2425_d_n8: f64 = (p.p7 * eq193_e2424_d_n8);
        let eq193_e2425_d_n9: f64 = (p.p7 * eq193_e2424_d_n9);
        let eq193_e2425_d_n10: f64 = (p.p7 * eq193_e2424_d_n10);
        let eq193_e2425_d_n11: f64 = (p.p7 * eq193_e2424_d_n11);
        let eq193_e2425_d_n12: f64 = (p.p7 * eq193_e2424_d_n12);
        let eq193_e2425_d_n13: f64 = (p.p7 * eq193_e2424_d_n13);
        let eq193_e2425_d_n14: f64 = (p.p7 * eq193_e2424_d_n14);
        let eq193_e2425_d_n15: f64 = (p.p7 * eq193_e2424_d_n15);
        let eq193_e2425_d_n16: f64 = (p.p7 * eq193_e2424_d_n16);
        let eq193_e2425_d_n17: f64 = (p.p7 * eq193_e2424_d_n17);
        let eq193_e2425_d_n18: f64 = (p.p7 * eq193_e2424_d_n18);
        let eq193_e2425_d_n19: f64 = (p.p7 * eq193_e2424_d_n19);
        let eq193_e2425_d_n20: f64 = (p.p7 * eq193_e2424_d_n20);
        let eq193_e2425_d_n21: f64 = (p.p7 * eq193_e2424_d_n21);
        let eq193_e2425_d_n22: f64 = (p.p7 * eq193_e2424_d_n22);
        (eq193_e2425, eq193_e2425_d_n0, eq193_e2425_d_n1, eq193_e2425_d_n2, eq193_e2425_d_n3, eq193_e2425_d_n4, eq193_e2425_d_n5, eq193_e2425_d_n6, eq193_e2425_d_n7, eq193_e2425_d_n8, eq193_e2425_d_n9, eq193_e2425_d_n10, eq193_e2425_d_n11, eq193_e2425_d_n12, eq193_e2425_d_n13, eq193_e2425_d_n14, eq193_e2425_d_n15, eq193_e2425_d_n16, eq193_e2425_d_n17, eq193_e2425_d_n18, eq193_e2425_d_n19, eq193_e2425_d_n20, eq193_e2425_d_n21, eq193_e2425_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq193_value: f64 = eq193_e2427;
        let eq193_node_derivatives: [f64; 23] = [eq193_e2427_d_n0, eq193_e2427_d_n1, eq193_e2427_d_n2, eq193_e2427_d_n3, eq193_e2427_d_n4, eq193_e2427_d_n5, eq193_e2427_d_n6, eq193_e2427_d_n7, eq193_e2427_d_n8, eq193_e2427_d_n9, eq193_e2427_d_n10, eq193_e2427_d_n11, eq193_e2427_d_n12, eq193_e2427_d_n13, eq193_e2427_d_n14, eq193_e2427_d_n15, eq193_e2427_d_n16, eq193_e2427_d_n17, eq193_e2427_d_n18, eq193_e2427_d_n19, eq193_e2427_d_n20, eq193_e2427_d_n21, eq193_e2427_d_n22];
        let eq193_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[17]),
            multiplicity * (eq193_value),
            nodes,
            &eq193_node_derivatives,
            branches,
            &eq193_branch_derivatives,
            multiplicity,
        );
        let (eq194_e2440, eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22,) = {
    if ((s.b[600] && s.b[601]) && s.b[602]) {
        let eq194_e2435: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 93, s.v[300]);
        let eq194_e2435_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq194_e2435_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq194_e2435_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq194_e2435_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq194_e2435_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq194_e2435_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq194_e2435_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq194_e2435_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq194_e2435_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq194_e2435_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq194_e2435_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq194_e2435_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq194_e2435_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq194_e2435_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq194_e2435_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq194_e2435_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq194_e2435_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq194_e2435_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq194_e2435_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq194_e2435_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq194_e2435_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq194_e2435_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq194_e2435_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq194_e2436: f64 = (p.p7 * eq194_e2435);
        let eq194_e2436_d_n0: f64 = (p.p7 * eq194_e2435_d_n0);
        let eq194_e2436_d_n1: f64 = (p.p7 * eq194_e2435_d_n1);
        let eq194_e2436_d_n2: f64 = (p.p7 * eq194_e2435_d_n2);
        let eq194_e2436_d_n3: f64 = (p.p7 * eq194_e2435_d_n3);
        let eq194_e2436_d_n4: f64 = (p.p7 * eq194_e2435_d_n4);
        let eq194_e2436_d_n5: f64 = (p.p7 * eq194_e2435_d_n5);
        let eq194_e2436_d_n6: f64 = (p.p7 * eq194_e2435_d_n6);
        let eq194_e2436_d_n7: f64 = (p.p7 * eq194_e2435_d_n7);
        let eq194_e2436_d_n8: f64 = (p.p7 * eq194_e2435_d_n8);
        let eq194_e2436_d_n9: f64 = (p.p7 * eq194_e2435_d_n9);
        let eq194_e2436_d_n10: f64 = (p.p7 * eq194_e2435_d_n10);
        let eq194_e2436_d_n11: f64 = (p.p7 * eq194_e2435_d_n11);
        let eq194_e2436_d_n12: f64 = (p.p7 * eq194_e2435_d_n12);
        let eq194_e2436_d_n13: f64 = (p.p7 * eq194_e2435_d_n13);
        let eq194_e2436_d_n14: f64 = (p.p7 * eq194_e2435_d_n14);
        let eq194_e2436_d_n15: f64 = (p.p7 * eq194_e2435_d_n15);
        let eq194_e2436_d_n16: f64 = (p.p7 * eq194_e2435_d_n16);
        let eq194_e2436_d_n17: f64 = (p.p7 * eq194_e2435_d_n17);
        let eq194_e2436_d_n18: f64 = (p.p7 * eq194_e2435_d_n18);
        let eq194_e2436_d_n19: f64 = (p.p7 * eq194_e2435_d_n19);
        let eq194_e2436_d_n20: f64 = (p.p7 * eq194_e2435_d_n20);
        let eq194_e2436_d_n21: f64 = (p.p7 * eq194_e2435_d_n21);
        let eq194_e2436_d_n22: f64 = (p.p7 * eq194_e2435_d_n22);
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
        (eq194_e2438, eq194_e2438_d_n0, eq194_e2438_d_n1, eq194_e2438_d_n2, eq194_e2438_d_n3, eq194_e2438_d_n4, eq194_e2438_d_n5, eq194_e2438_d_n6, eq194_e2438_d_n7, eq194_e2438_d_n8, eq194_e2438_d_n9, eq194_e2438_d_n10, eq194_e2438_d_n11, eq194_e2438_d_n12, eq194_e2438_d_n13, eq194_e2438_d_n14, eq194_e2438_d_n15, eq194_e2438_d_n16, eq194_e2438_d_n17, eq194_e2438_d_n18, eq194_e2438_d_n19, eq194_e2438_d_n20, eq194_e2438_d_n21, eq194_e2438_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2440;
        let eq194_node_derivatives: [f64; 23] = [eq194_e2440_d_n0, eq194_e2440_d_n1, eq194_e2440_d_n2, eq194_e2440_d_n3, eq194_e2440_d_n4, eq194_e2440_d_n5, eq194_e2440_d_n6, eq194_e2440_d_n7, eq194_e2440_d_n8, eq194_e2440_d_n9, eq194_e2440_d_n10, eq194_e2440_d_n11, eq194_e2440_d_n12, eq194_e2440_d_n13, eq194_e2440_d_n14, eq194_e2440_d_n15, eq194_e2440_d_n16, eq194_e2440_d_n17, eq194_e2440_d_n18, eq194_e2440_d_n19, eq194_e2440_d_n20, eq194_e2440_d_n21, eq194_e2440_d_n22];
        let eq194_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            multiplicity * (eq194_value),
            nodes,
            &eq194_node_derivatives,
            branches,
            &eq194_branch_derivatives,
            multiplicity,
        );
        let (eq195_e2452, eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22,) = {
    if ((s.b[600] && s.b[601]) && (!s.b[602])) {
        let eq195_e2449: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 94, s.v[300]);
        let eq195_e2449_d_n0: f64 = (s.dn[300][0] * ddt_scale);
        let eq195_e2449_d_n1: f64 = (s.dn[300][1] * ddt_scale);
        let eq195_e2449_d_n2: f64 = (s.dn[300][2] * ddt_scale);
        let eq195_e2449_d_n3: f64 = (s.dn[300][3] * ddt_scale);
        let eq195_e2449_d_n4: f64 = (s.dn[300][4] * ddt_scale);
        let eq195_e2449_d_n5: f64 = (s.dn[300][5] * ddt_scale);
        let eq195_e2449_d_n6: f64 = (s.dn[300][6] * ddt_scale);
        let eq195_e2449_d_n7: f64 = (s.dn[300][7] * ddt_scale);
        let eq195_e2449_d_n8: f64 = (s.dn[300][8] * ddt_scale);
        let eq195_e2449_d_n9: f64 = (s.dn[300][9] * ddt_scale);
        let eq195_e2449_d_n10: f64 = (s.dn[300][10] * ddt_scale);
        let eq195_e2449_d_n11: f64 = (s.dn[300][11] * ddt_scale);
        let eq195_e2449_d_n12: f64 = (s.dn[300][12] * ddt_scale);
        let eq195_e2449_d_n13: f64 = (s.dn[300][13] * ddt_scale);
        let eq195_e2449_d_n14: f64 = (s.dn[300][14] * ddt_scale);
        let eq195_e2449_d_n15: f64 = (s.dn[300][15] * ddt_scale);
        let eq195_e2449_d_n16: f64 = (s.dn[300][16] * ddt_scale);
        let eq195_e2449_d_n17: f64 = (s.dn[300][17] * ddt_scale);
        let eq195_e2449_d_n18: f64 = (s.dn[300][18] * ddt_scale);
        let eq195_e2449_d_n19: f64 = (s.dn[300][19] * ddt_scale);
        let eq195_e2449_d_n20: f64 = (s.dn[300][20] * ddt_scale);
        let eq195_e2449_d_n21: f64 = (s.dn[300][21] * ddt_scale);
        let eq195_e2449_d_n22: f64 = (s.dn[300][22] * ddt_scale);
        let eq195_e2450: f64 = (p.p7 * eq195_e2449);
        let eq195_e2450_d_n0: f64 = (p.p7 * eq195_e2449_d_n0);
        let eq195_e2450_d_n1: f64 = (p.p7 * eq195_e2449_d_n1);
        let eq195_e2450_d_n2: f64 = (p.p7 * eq195_e2449_d_n2);
        let eq195_e2450_d_n3: f64 = (p.p7 * eq195_e2449_d_n3);
        let eq195_e2450_d_n4: f64 = (p.p7 * eq195_e2449_d_n4);
        let eq195_e2450_d_n5: f64 = (p.p7 * eq195_e2449_d_n5);
        let eq195_e2450_d_n6: f64 = (p.p7 * eq195_e2449_d_n6);
        let eq195_e2450_d_n7: f64 = (p.p7 * eq195_e2449_d_n7);
        let eq195_e2450_d_n8: f64 = (p.p7 * eq195_e2449_d_n8);
        let eq195_e2450_d_n9: f64 = (p.p7 * eq195_e2449_d_n9);
        let eq195_e2450_d_n10: f64 = (p.p7 * eq195_e2449_d_n10);
        let eq195_e2450_d_n11: f64 = (p.p7 * eq195_e2449_d_n11);
        let eq195_e2450_d_n12: f64 = (p.p7 * eq195_e2449_d_n12);
        let eq195_e2450_d_n13: f64 = (p.p7 * eq195_e2449_d_n13);
        let eq195_e2450_d_n14: f64 = (p.p7 * eq195_e2449_d_n14);
        let eq195_e2450_d_n15: f64 = (p.p7 * eq195_e2449_d_n15);
        let eq195_e2450_d_n16: f64 = (p.p7 * eq195_e2449_d_n16);
        let eq195_e2450_d_n17: f64 = (p.p7 * eq195_e2449_d_n17);
        let eq195_e2450_d_n18: f64 = (p.p7 * eq195_e2449_d_n18);
        let eq195_e2450_d_n19: f64 = (p.p7 * eq195_e2449_d_n19);
        let eq195_e2450_d_n20: f64 = (p.p7 * eq195_e2449_d_n20);
        let eq195_e2450_d_n21: f64 = (p.p7 * eq195_e2449_d_n21);
        let eq195_e2450_d_n22: f64 = (p.p7 * eq195_e2449_d_n22);
        (eq195_e2450, eq195_e2450_d_n0, eq195_e2450_d_n1, eq195_e2450_d_n2, eq195_e2450_d_n3, eq195_e2450_d_n4, eq195_e2450_d_n5, eq195_e2450_d_n6, eq195_e2450_d_n7, eq195_e2450_d_n8, eq195_e2450_d_n9, eq195_e2450_d_n10, eq195_e2450_d_n11, eq195_e2450_d_n12, eq195_e2450_d_n13, eq195_e2450_d_n14, eq195_e2450_d_n15, eq195_e2450_d_n16, eq195_e2450_d_n17, eq195_e2450_d_n18, eq195_e2450_d_n19, eq195_e2450_d_n20, eq195_e2450_d_n21, eq195_e2450_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2452;
        let eq195_node_derivatives: [f64; 23] = [eq195_e2452_d_n0, eq195_e2452_d_n1, eq195_e2452_d_n2, eq195_e2452_d_n3, eq195_e2452_d_n4, eq195_e2452_d_n5, eq195_e2452_d_n6, eq195_e2452_d_n7, eq195_e2452_d_n8, eq195_e2452_d_n9, eq195_e2452_d_n10, eq195_e2452_d_n11, eq195_e2452_d_n12, eq195_e2452_d_n13, eq195_e2452_d_n14, eq195_e2452_d_n15, eq195_e2452_d_n16, eq195_e2452_d_n17, eq195_e2452_d_n18, eq195_e2452_d_n19, eq195_e2452_d_n20, eq195_e2452_d_n21, eq195_e2452_d_n22];
        let eq195_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[17]),
            multiplicity * (eq195_value),
            nodes,
            &eq195_node_derivatives,
            branches,
            &eq195_branch_derivatives,
            multiplicity,
        );
    }
}
