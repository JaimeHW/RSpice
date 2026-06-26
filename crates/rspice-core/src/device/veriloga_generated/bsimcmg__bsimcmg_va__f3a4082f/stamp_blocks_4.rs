#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq27_e2201, eq27_e2201_d_n0, eq27_e2201_d_n1, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n12, eq27_e2201_d_n13, eq27_e2201_d_n14, eq27_e2201_d_n15, eq27_e2201_d_n16, eq27_e2201_d_b0, eq27_e2201_d_b1, eq27_e2201_d_b2, eq27_e2201_d_b3, eq27_e2201_d_b4, eq27_e2201_d_b5, eq27_e2201_d_b6, eq27_e2201_d_b7, eq27_e2201_d_b8, eq27_e2201_d_b9, eq27_e2201_d_b10, eq27_e2201_d_b11, eq27_e2201_d_b12, eq27_e2201_d_b13, eq27_e2201_d_b14, eq27_e2201_d_b15, eq27_e2201_d_b16, eq27_e2201_d_b17,) = {
    if (((!s.b[1698]) && s.b[1701]) && (!s.b[1702])) {
        let eq27_e2198: f64 = (s.v[476] + s.v[488]);
        let eq27_e2198_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq27_e2198_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq27_e2198_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq27_e2198_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq27_e2198_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq27_e2198_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq27_e2198_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq27_e2198_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq27_e2198_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq27_e2198_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq27_e2198_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq27_e2198_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq27_e2198_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq27_e2198_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq27_e2198_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq27_e2198_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq27_e2198_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq27_e2198_d_b0: f64 = (s.db[476][0] + s.db[488][0]);
        let eq27_e2198_d_b1: f64 = (s.db[476][1] + s.db[488][1]);
        let eq27_e2198_d_b2: f64 = (s.db[476][2] + s.db[488][2]);
        let eq27_e2198_d_b3: f64 = (s.db[476][3] + s.db[488][3]);
        let eq27_e2198_d_b4: f64 = (s.db[476][4] + s.db[488][4]);
        let eq27_e2198_d_b5: f64 = (s.db[476][5] + s.db[488][5]);
        let eq27_e2198_d_b6: f64 = (s.db[476][6] + s.db[488][6]);
        let eq27_e2198_d_b7: f64 = (s.db[476][7] + s.db[488][7]);
        let eq27_e2198_d_b8: f64 = (s.db[476][8] + s.db[488][8]);
        let eq27_e2198_d_b9: f64 = (s.db[476][9] + s.db[488][9]);
        let eq27_e2198_d_b10: f64 = (s.db[476][10] + s.db[488][10]);
        let eq27_e2198_d_b11: f64 = (s.db[476][11] + s.db[488][11]);
        let eq27_e2198_d_b12: f64 = (s.db[476][12] + s.db[488][12]);
        let eq27_e2198_d_b13: f64 = (s.db[476][13] + s.db[488][13]);
        let eq27_e2198_d_b14: f64 = (s.db[476][14] + s.db[488][14]);
        let eq27_e2198_d_b15: f64 = (s.db[476][15] + s.db[488][15]);
        let eq27_e2198_d_b16: f64 = (s.db[476][16] + s.db[488][16]);
        let eq27_e2198_d_b17: f64 = (s.db[476][17] + s.db[488][17]);
        let eq27_e2199: f64 = (s.v[114] * eq27_e2198);
        let eq27_e2199_d_n0: f64 = ((s.dn[114][0] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n0));
        let eq27_e2199_d_n1: f64 = ((s.dn[114][1] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n1));
        let eq27_e2199_d_n2: f64 = ((s.dn[114][2] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n2));
        let eq27_e2199_d_n3: f64 = ((s.dn[114][3] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n3));
        let eq27_e2199_d_n4: f64 = ((s.dn[114][4] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n4));
        let eq27_e2199_d_n5: f64 = ((s.dn[114][5] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n5));
        let eq27_e2199_d_n6: f64 = ((s.dn[114][6] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n6));
        let eq27_e2199_d_n7: f64 = ((s.dn[114][7] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n7));
        let eq27_e2199_d_n8: f64 = ((s.dn[114][8] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n8));
        let eq27_e2199_d_n9: f64 = ((s.dn[114][9] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n9));
        let eq27_e2199_d_n10: f64 = ((s.dn[114][10] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n10));
        let eq27_e2199_d_n11: f64 = ((s.dn[114][11] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n11));
        let eq27_e2199_d_n12: f64 = ((s.dn[114][12] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n12));
        let eq27_e2199_d_n13: f64 = ((s.dn[114][13] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n13));
        let eq27_e2199_d_n14: f64 = ((s.dn[114][14] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n14));
        let eq27_e2199_d_n15: f64 = ((s.dn[114][15] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n15));
        let eq27_e2199_d_n16: f64 = ((s.dn[114][16] * eq27_e2198) + (s.v[114] * eq27_e2198_d_n16));
        let eq27_e2199_d_b0: f64 = ((s.db[114][0] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b0));
        let eq27_e2199_d_b1: f64 = ((s.db[114][1] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b1));
        let eq27_e2199_d_b2: f64 = ((s.db[114][2] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b2));
        let eq27_e2199_d_b3: f64 = ((s.db[114][3] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b3));
        let eq27_e2199_d_b4: f64 = ((s.db[114][4] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b4));
        let eq27_e2199_d_b5: f64 = ((s.db[114][5] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b5));
        let eq27_e2199_d_b6: f64 = ((s.db[114][6] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b6));
        let eq27_e2199_d_b7: f64 = ((s.db[114][7] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b7));
        let eq27_e2199_d_b8: f64 = ((s.db[114][8] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b8));
        let eq27_e2199_d_b9: f64 = ((s.db[114][9] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b9));
        let eq27_e2199_d_b10: f64 = ((s.db[114][10] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b10));
        let eq27_e2199_d_b11: f64 = ((s.db[114][11] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b11));
        let eq27_e2199_d_b12: f64 = ((s.db[114][12] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b12));
        let eq27_e2199_d_b13: f64 = ((s.db[114][13] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b13));
        let eq27_e2199_d_b14: f64 = ((s.db[114][14] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b14));
        let eq27_e2199_d_b15: f64 = ((s.db[114][15] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b15));
        let eq27_e2199_d_b16: f64 = ((s.db[114][16] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b16));
        let eq27_e2199_d_b17: f64 = ((s.db[114][17] * eq27_e2198) + (s.v[114] * eq27_e2198_d_b17));
        (eq27_e2199, eq27_e2199_d_n0, eq27_e2199_d_n1, eq27_e2199_d_n2, eq27_e2199_d_n3, eq27_e2199_d_n4, eq27_e2199_d_n5, eq27_e2199_d_n6, eq27_e2199_d_n7, eq27_e2199_d_n8, eq27_e2199_d_n9, eq27_e2199_d_n10, eq27_e2199_d_n11, eq27_e2199_d_n12, eq27_e2199_d_n13, eq27_e2199_d_n14, eq27_e2199_d_n15, eq27_e2199_d_n16, eq27_e2199_d_b0, eq27_e2199_d_b1, eq27_e2199_d_b2, eq27_e2199_d_b3, eq27_e2199_d_b4, eq27_e2199_d_b5, eq27_e2199_d_b6, eq27_e2199_d_b7, eq27_e2199_d_b8, eq27_e2199_d_b9, eq27_e2199_d_b10, eq27_e2199_d_b11, eq27_e2199_d_b12, eq27_e2199_d_b13, eq27_e2199_d_b14, eq27_e2199_d_b15, eq27_e2199_d_b16, eq27_e2199_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e2201;
        let eq27_node_derivatives: [f64; 17] = [eq27_e2201_d_n0, eq27_e2201_d_n1, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n12, eq27_e2201_d_n13, eq27_e2201_d_n14, eq27_e2201_d_n15, eq27_e2201_d_n16];
        let eq27_branch_derivatives: [f64; 18] = [eq27_e2201_d_b0, eq27_e2201_d_b1, eq27_e2201_d_b2, eq27_e2201_d_b3, eq27_e2201_d_b4, eq27_e2201_d_b5, eq27_e2201_d_b6, eq27_e2201_d_b7, eq27_e2201_d_b8, eq27_e2201_d_b9, eq27_e2201_d_b10, eq27_e2201_d_b11, eq27_e2201_d_b12, eq27_e2201_d_b13, eq27_e2201_d_b14, eq27_e2201_d_b15, eq27_e2201_d_b16, eq27_e2201_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            multiplicity * (eq27_value),
            nodes,
            &eq27_node_derivatives,
            branches,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e2213, eq28_e2213_d_n0, eq28_e2213_d_n1, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n12, eq28_e2213_d_n13, eq28_e2213_d_n14, eq28_e2213_d_n15, eq28_e2213_d_n16, eq28_e2213_d_b0, eq28_e2213_d_b1, eq28_e2213_d_b2, eq28_e2213_d_b3, eq28_e2213_d_b4, eq28_e2213_d_b5, eq28_e2213_d_b6, eq28_e2213_d_b7, eq28_e2213_d_b8, eq28_e2213_d_b9, eq28_e2213_d_b10, eq28_e2213_d_b11, eq28_e2213_d_b12, eq28_e2213_d_b13, eq28_e2213_d_b14, eq28_e2213_d_b15, eq28_e2213_d_b16, eq28_e2213_d_b17,) = {
    if (((!s.b[1698]) && s.b[1701]) && (!s.b[1702])) {
        let eq28_e2211: f64 = (s.v[114] * s.v[475]);
        let eq28_e2211_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq28_e2211_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq28_e2211_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq28_e2211_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq28_e2211_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq28_e2211_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq28_e2211_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq28_e2211_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq28_e2211_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq28_e2211_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq28_e2211_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq28_e2211_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq28_e2211_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq28_e2211_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq28_e2211_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq28_e2211_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq28_e2211_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        let eq28_e2211_d_b0: f64 = ((s.db[114][0] * s.v[475]) + (s.v[114] * s.db[475][0]));
        let eq28_e2211_d_b1: f64 = ((s.db[114][1] * s.v[475]) + (s.v[114] * s.db[475][1]));
        let eq28_e2211_d_b2: f64 = ((s.db[114][2] * s.v[475]) + (s.v[114] * s.db[475][2]));
        let eq28_e2211_d_b3: f64 = ((s.db[114][3] * s.v[475]) + (s.v[114] * s.db[475][3]));
        let eq28_e2211_d_b4: f64 = ((s.db[114][4] * s.v[475]) + (s.v[114] * s.db[475][4]));
        let eq28_e2211_d_b5: f64 = ((s.db[114][5] * s.v[475]) + (s.v[114] * s.db[475][5]));
        let eq28_e2211_d_b6: f64 = ((s.db[114][6] * s.v[475]) + (s.v[114] * s.db[475][6]));
        let eq28_e2211_d_b7: f64 = ((s.db[114][7] * s.v[475]) + (s.v[114] * s.db[475][7]));
        let eq28_e2211_d_b8: f64 = ((s.db[114][8] * s.v[475]) + (s.v[114] * s.db[475][8]));
        let eq28_e2211_d_b9: f64 = ((s.db[114][9] * s.v[475]) + (s.v[114] * s.db[475][9]));
        let eq28_e2211_d_b10: f64 = ((s.db[114][10] * s.v[475]) + (s.v[114] * s.db[475][10]));
        let eq28_e2211_d_b11: f64 = ((s.db[114][11] * s.v[475]) + (s.v[114] * s.db[475][11]));
        let eq28_e2211_d_b12: f64 = ((s.db[114][12] * s.v[475]) + (s.v[114] * s.db[475][12]));
        let eq28_e2211_d_b13: f64 = ((s.db[114][13] * s.v[475]) + (s.v[114] * s.db[475][13]));
        let eq28_e2211_d_b14: f64 = ((s.db[114][14] * s.v[475]) + (s.v[114] * s.db[475][14]));
        let eq28_e2211_d_b15: f64 = ((s.db[114][15] * s.v[475]) + (s.v[114] * s.db[475][15]));
        let eq28_e2211_d_b16: f64 = ((s.db[114][16] * s.v[475]) + (s.v[114] * s.db[475][16]));
        let eq28_e2211_d_b17: f64 = ((s.db[114][17] * s.v[475]) + (s.v[114] * s.db[475][17]));
        (eq28_e2211, eq28_e2211_d_n0, eq28_e2211_d_n1, eq28_e2211_d_n2, eq28_e2211_d_n3, eq28_e2211_d_n4, eq28_e2211_d_n5, eq28_e2211_d_n6, eq28_e2211_d_n7, eq28_e2211_d_n8, eq28_e2211_d_n9, eq28_e2211_d_n10, eq28_e2211_d_n11, eq28_e2211_d_n12, eq28_e2211_d_n13, eq28_e2211_d_n14, eq28_e2211_d_n15, eq28_e2211_d_n16, eq28_e2211_d_b0, eq28_e2211_d_b1, eq28_e2211_d_b2, eq28_e2211_d_b3, eq28_e2211_d_b4, eq28_e2211_d_b5, eq28_e2211_d_b6, eq28_e2211_d_b7, eq28_e2211_d_b8, eq28_e2211_d_b9, eq28_e2211_d_b10, eq28_e2211_d_b11, eq28_e2211_d_b12, eq28_e2211_d_b13, eq28_e2211_d_b14, eq28_e2211_d_b15, eq28_e2211_d_b16, eq28_e2211_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e2213;
        let eq28_node_derivatives: [f64; 17] = [eq28_e2213_d_n0, eq28_e2213_d_n1, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n12, eq28_e2213_d_n13, eq28_e2213_d_n14, eq28_e2213_d_n15, eq28_e2213_d_n16];
        let eq28_branch_derivatives: [f64; 18] = [eq28_e2213_d_b0, eq28_e2213_d_b1, eq28_e2213_d_b2, eq28_e2213_d_b3, eq28_e2213_d_b4, eq28_e2213_d_b5, eq28_e2213_d_b6, eq28_e2213_d_b7, eq28_e2213_d_b8, eq28_e2213_d_b9, eq28_e2213_d_b10, eq28_e2213_d_b11, eq28_e2213_d_b12, eq28_e2213_d_b13, eq28_e2213_d_b14, eq28_e2213_d_b15, eq28_e2213_d_b16, eq28_e2213_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            multiplicity * (eq28_value),
            nodes,
            &eq28_node_derivatives,
            branches,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e2224, eq29_e2224_d_n0, eq29_e2224_d_n1, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n12, eq29_e2224_d_n13, eq29_e2224_d_n14, eq29_e2224_d_n15, eq29_e2224_d_n16, eq29_e2224_d_b0, eq29_e2224_d_b1, eq29_e2224_d_b2, eq29_e2224_d_b3, eq29_e2224_d_b4, eq29_e2224_d_b5, eq29_e2224_d_b6, eq29_e2224_d_b7, eq29_e2224_d_b8, eq29_e2224_d_b9, eq29_e2224_d_b10, eq29_e2224_d_b11, eq29_e2224_d_b12, eq29_e2224_d_b13, eq29_e2224_d_b14, eq29_e2224_d_b15, eq29_e2224_d_b16, eq29_e2224_d_b17,) = {
    if ((!s.b[1698]) && s.b[1701]) {
        let eq29_e2221: f64 = (s.v[461] + s.v[469]);
        let eq29_e2221_d_n0: f64 = (s.dn[461][0] + s.dn[469][0]);
        let eq29_e2221_d_n1: f64 = (s.dn[461][1] + s.dn[469][1]);
        let eq29_e2221_d_n2: f64 = (s.dn[461][2] + s.dn[469][2]);
        let eq29_e2221_d_n3: f64 = (s.dn[461][3] + s.dn[469][3]);
        let eq29_e2221_d_n4: f64 = (s.dn[461][4] + s.dn[469][4]);
        let eq29_e2221_d_n5: f64 = (s.dn[461][5] + s.dn[469][5]);
        let eq29_e2221_d_n6: f64 = (s.dn[461][6] + s.dn[469][6]);
        let eq29_e2221_d_n7: f64 = (s.dn[461][7] + s.dn[469][7]);
        let eq29_e2221_d_n8: f64 = (s.dn[461][8] + s.dn[469][8]);
        let eq29_e2221_d_n9: f64 = (s.dn[461][9] + s.dn[469][9]);
        let eq29_e2221_d_n10: f64 = (s.dn[461][10] + s.dn[469][10]);
        let eq29_e2221_d_n11: f64 = (s.dn[461][11] + s.dn[469][11]);
        let eq29_e2221_d_n12: f64 = (s.dn[461][12] + s.dn[469][12]);
        let eq29_e2221_d_n13: f64 = (s.dn[461][13] + s.dn[469][13]);
        let eq29_e2221_d_n14: f64 = (s.dn[461][14] + s.dn[469][14]);
        let eq29_e2221_d_n15: f64 = (s.dn[461][15] + s.dn[469][15]);
        let eq29_e2221_d_n16: f64 = (s.dn[461][16] + s.dn[469][16]);
        let eq29_e2221_d_b0: f64 = (s.db[461][0] + s.db[469][0]);
        let eq29_e2221_d_b1: f64 = (s.db[461][1] + s.db[469][1]);
        let eq29_e2221_d_b2: f64 = (s.db[461][2] + s.db[469][2]);
        let eq29_e2221_d_b3: f64 = (s.db[461][3] + s.db[469][3]);
        let eq29_e2221_d_b4: f64 = (s.db[461][4] + s.db[469][4]);
        let eq29_e2221_d_b5: f64 = (s.db[461][5] + s.db[469][5]);
        let eq29_e2221_d_b6: f64 = (s.db[461][6] + s.db[469][6]);
        let eq29_e2221_d_b7: f64 = (s.db[461][7] + s.db[469][7]);
        let eq29_e2221_d_b8: f64 = (s.db[461][8] + s.db[469][8]);
        let eq29_e2221_d_b9: f64 = (s.db[461][9] + s.db[469][9]);
        let eq29_e2221_d_b10: f64 = (s.db[461][10] + s.db[469][10]);
        let eq29_e2221_d_b11: f64 = (s.db[461][11] + s.db[469][11]);
        let eq29_e2221_d_b12: f64 = (s.db[461][12] + s.db[469][12]);
        let eq29_e2221_d_b13: f64 = (s.db[461][13] + s.db[469][13]);
        let eq29_e2221_d_b14: f64 = (s.db[461][14] + s.db[469][14]);
        let eq29_e2221_d_b15: f64 = (s.db[461][15] + s.db[469][15]);
        let eq29_e2221_d_b16: f64 = (s.db[461][16] + s.db[469][16]);
        let eq29_e2221_d_b17: f64 = (s.db[461][17] + s.db[469][17]);
        let eq29_e2222: f64 = (s.v[114] * eq29_e2221);
        let eq29_e2222_d_n0: f64 = ((s.dn[114][0] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n0));
        let eq29_e2222_d_n1: f64 = ((s.dn[114][1] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n1));
        let eq29_e2222_d_n2: f64 = ((s.dn[114][2] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n2));
        let eq29_e2222_d_n3: f64 = ((s.dn[114][3] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n3));
        let eq29_e2222_d_n4: f64 = ((s.dn[114][4] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n4));
        let eq29_e2222_d_n5: f64 = ((s.dn[114][5] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n5));
        let eq29_e2222_d_n6: f64 = ((s.dn[114][6] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n6));
        let eq29_e2222_d_n7: f64 = ((s.dn[114][7] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n7));
        let eq29_e2222_d_n8: f64 = ((s.dn[114][8] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n8));
        let eq29_e2222_d_n9: f64 = ((s.dn[114][9] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n9));
        let eq29_e2222_d_n10: f64 = ((s.dn[114][10] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n10));
        let eq29_e2222_d_n11: f64 = ((s.dn[114][11] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n11));
        let eq29_e2222_d_n12: f64 = ((s.dn[114][12] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n12));
        let eq29_e2222_d_n13: f64 = ((s.dn[114][13] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n13));
        let eq29_e2222_d_n14: f64 = ((s.dn[114][14] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n14));
        let eq29_e2222_d_n15: f64 = ((s.dn[114][15] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n15));
        let eq29_e2222_d_n16: f64 = ((s.dn[114][16] * eq29_e2221) + (s.v[114] * eq29_e2221_d_n16));
        let eq29_e2222_d_b0: f64 = ((s.db[114][0] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b0));
        let eq29_e2222_d_b1: f64 = ((s.db[114][1] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b1));
        let eq29_e2222_d_b2: f64 = ((s.db[114][2] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b2));
        let eq29_e2222_d_b3: f64 = ((s.db[114][3] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b3));
        let eq29_e2222_d_b4: f64 = ((s.db[114][4] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b4));
        let eq29_e2222_d_b5: f64 = ((s.db[114][5] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b5));
        let eq29_e2222_d_b6: f64 = ((s.db[114][6] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b6));
        let eq29_e2222_d_b7: f64 = ((s.db[114][7] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b7));
        let eq29_e2222_d_b8: f64 = ((s.db[114][8] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b8));
        let eq29_e2222_d_b9: f64 = ((s.db[114][9] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b9));
        let eq29_e2222_d_b10: f64 = ((s.db[114][10] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b10));
        let eq29_e2222_d_b11: f64 = ((s.db[114][11] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b11));
        let eq29_e2222_d_b12: f64 = ((s.db[114][12] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b12));
        let eq29_e2222_d_b13: f64 = ((s.db[114][13] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b13));
        let eq29_e2222_d_b14: f64 = ((s.db[114][14] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b14));
        let eq29_e2222_d_b15: f64 = ((s.db[114][15] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b15));
        let eq29_e2222_d_b16: f64 = ((s.db[114][16] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b16));
        let eq29_e2222_d_b17: f64 = ((s.db[114][17] * eq29_e2221) + (s.v[114] * eq29_e2221_d_b17));
        (eq29_e2222, eq29_e2222_d_n0, eq29_e2222_d_n1, eq29_e2222_d_n2, eq29_e2222_d_n3, eq29_e2222_d_n4, eq29_e2222_d_n5, eq29_e2222_d_n6, eq29_e2222_d_n7, eq29_e2222_d_n8, eq29_e2222_d_n9, eq29_e2222_d_n10, eq29_e2222_d_n11, eq29_e2222_d_n12, eq29_e2222_d_n13, eq29_e2222_d_n14, eq29_e2222_d_n15, eq29_e2222_d_n16, eq29_e2222_d_b0, eq29_e2222_d_b1, eq29_e2222_d_b2, eq29_e2222_d_b3, eq29_e2222_d_b4, eq29_e2222_d_b5, eq29_e2222_d_b6, eq29_e2222_d_b7, eq29_e2222_d_b8, eq29_e2222_d_b9, eq29_e2222_d_b10, eq29_e2222_d_b11, eq29_e2222_d_b12, eq29_e2222_d_b13, eq29_e2222_d_b14, eq29_e2222_d_b15, eq29_e2222_d_b16, eq29_e2222_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e2224;
        let eq29_node_derivatives: [f64; 17] = [eq29_e2224_d_n0, eq29_e2224_d_n1, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n12, eq29_e2224_d_n13, eq29_e2224_d_n14, eq29_e2224_d_n15, eq29_e2224_d_n16];
        let eq29_branch_derivatives: [f64; 18] = [eq29_e2224_d_b0, eq29_e2224_d_b1, eq29_e2224_d_b2, eq29_e2224_d_b3, eq29_e2224_d_b4, eq29_e2224_d_b5, eq29_e2224_d_b6, eq29_e2224_d_b7, eq29_e2224_d_b8, eq29_e2224_d_b9, eq29_e2224_d_b10, eq29_e2224_d_b11, eq29_e2224_d_b12, eq29_e2224_d_b13, eq29_e2224_d_b14, eq29_e2224_d_b15, eq29_e2224_d_b16, eq29_e2224_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[3]),
            multiplicity * (eq29_value),
            nodes,
            &eq29_node_derivatives,
            branches,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e2236, eq30_e2236_d_n0, eq30_e2236_d_n1, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n12, eq30_e2236_d_n13, eq30_e2236_d_n14, eq30_e2236_d_n15, eq30_e2236_d_n16, eq30_e2236_d_b0, eq30_e2236_d_b1, eq30_e2236_d_b2, eq30_e2236_d_b3, eq30_e2236_d_b4, eq30_e2236_d_b5, eq30_e2236_d_b6, eq30_e2236_d_b7, eq30_e2236_d_b8, eq30_e2236_d_b9, eq30_e2236_d_b10, eq30_e2236_d_b11, eq30_e2236_d_b12, eq30_e2236_d_b13, eq30_e2236_d_b14, eq30_e2236_d_b15, eq30_e2236_d_b16, eq30_e2236_d_b17,) = {
    if ((!s.b[1698]) && (!s.b[1701])) {
        let eq30_e2233: f64 = (s.v[476] + s.v[488]);
        let eq30_e2233_d_n0: f64 = (s.dn[476][0] + s.dn[488][0]);
        let eq30_e2233_d_n1: f64 = (s.dn[476][1] + s.dn[488][1]);
        let eq30_e2233_d_n2: f64 = (s.dn[476][2] + s.dn[488][2]);
        let eq30_e2233_d_n3: f64 = (s.dn[476][3] + s.dn[488][3]);
        let eq30_e2233_d_n4: f64 = (s.dn[476][4] + s.dn[488][4]);
        let eq30_e2233_d_n5: f64 = (s.dn[476][5] + s.dn[488][5]);
        let eq30_e2233_d_n6: f64 = (s.dn[476][6] + s.dn[488][6]);
        let eq30_e2233_d_n7: f64 = (s.dn[476][7] + s.dn[488][7]);
        let eq30_e2233_d_n8: f64 = (s.dn[476][8] + s.dn[488][8]);
        let eq30_e2233_d_n9: f64 = (s.dn[476][9] + s.dn[488][9]);
        let eq30_e2233_d_n10: f64 = (s.dn[476][10] + s.dn[488][10]);
        let eq30_e2233_d_n11: f64 = (s.dn[476][11] + s.dn[488][11]);
        let eq30_e2233_d_n12: f64 = (s.dn[476][12] + s.dn[488][12]);
        let eq30_e2233_d_n13: f64 = (s.dn[476][13] + s.dn[488][13]);
        let eq30_e2233_d_n14: f64 = (s.dn[476][14] + s.dn[488][14]);
        let eq30_e2233_d_n15: f64 = (s.dn[476][15] + s.dn[488][15]);
        let eq30_e2233_d_n16: f64 = (s.dn[476][16] + s.dn[488][16]);
        let eq30_e2233_d_b0: f64 = (s.db[476][0] + s.db[488][0]);
        let eq30_e2233_d_b1: f64 = (s.db[476][1] + s.db[488][1]);
        let eq30_e2233_d_b2: f64 = (s.db[476][2] + s.db[488][2]);
        let eq30_e2233_d_b3: f64 = (s.db[476][3] + s.db[488][3]);
        let eq30_e2233_d_b4: f64 = (s.db[476][4] + s.db[488][4]);
        let eq30_e2233_d_b5: f64 = (s.db[476][5] + s.db[488][5]);
        let eq30_e2233_d_b6: f64 = (s.db[476][6] + s.db[488][6]);
        let eq30_e2233_d_b7: f64 = (s.db[476][7] + s.db[488][7]);
        let eq30_e2233_d_b8: f64 = (s.db[476][8] + s.db[488][8]);
        let eq30_e2233_d_b9: f64 = (s.db[476][9] + s.db[488][9]);
        let eq30_e2233_d_b10: f64 = (s.db[476][10] + s.db[488][10]);
        let eq30_e2233_d_b11: f64 = (s.db[476][11] + s.db[488][11]);
        let eq30_e2233_d_b12: f64 = (s.db[476][12] + s.db[488][12]);
        let eq30_e2233_d_b13: f64 = (s.db[476][13] + s.db[488][13]);
        let eq30_e2233_d_b14: f64 = (s.db[476][14] + s.db[488][14]);
        let eq30_e2233_d_b15: f64 = (s.db[476][15] + s.db[488][15]);
        let eq30_e2233_d_b16: f64 = (s.db[476][16] + s.db[488][16]);
        let eq30_e2233_d_b17: f64 = (s.db[476][17] + s.db[488][17]);
        let eq30_e2234: f64 = (s.v[114] * eq30_e2233);
        let eq30_e2234_d_n0: f64 = ((s.dn[114][0] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n0));
        let eq30_e2234_d_n1: f64 = ((s.dn[114][1] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n1));
        let eq30_e2234_d_n2: f64 = ((s.dn[114][2] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n2));
        let eq30_e2234_d_n3: f64 = ((s.dn[114][3] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n3));
        let eq30_e2234_d_n4: f64 = ((s.dn[114][4] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n4));
        let eq30_e2234_d_n5: f64 = ((s.dn[114][5] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n5));
        let eq30_e2234_d_n6: f64 = ((s.dn[114][6] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n6));
        let eq30_e2234_d_n7: f64 = ((s.dn[114][7] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n7));
        let eq30_e2234_d_n8: f64 = ((s.dn[114][8] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n8));
        let eq30_e2234_d_n9: f64 = ((s.dn[114][9] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n9));
        let eq30_e2234_d_n10: f64 = ((s.dn[114][10] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n10));
        let eq30_e2234_d_n11: f64 = ((s.dn[114][11] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n11));
        let eq30_e2234_d_n12: f64 = ((s.dn[114][12] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n12));
        let eq30_e2234_d_n13: f64 = ((s.dn[114][13] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n13));
        let eq30_e2234_d_n14: f64 = ((s.dn[114][14] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n14));
        let eq30_e2234_d_n15: f64 = ((s.dn[114][15] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n15));
        let eq30_e2234_d_n16: f64 = ((s.dn[114][16] * eq30_e2233) + (s.v[114] * eq30_e2233_d_n16));
        let eq30_e2234_d_b0: f64 = ((s.db[114][0] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b0));
        let eq30_e2234_d_b1: f64 = ((s.db[114][1] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b1));
        let eq30_e2234_d_b2: f64 = ((s.db[114][2] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b2));
        let eq30_e2234_d_b3: f64 = ((s.db[114][3] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b3));
        let eq30_e2234_d_b4: f64 = ((s.db[114][4] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b4));
        let eq30_e2234_d_b5: f64 = ((s.db[114][5] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b5));
        let eq30_e2234_d_b6: f64 = ((s.db[114][6] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b6));
        let eq30_e2234_d_b7: f64 = ((s.db[114][7] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b7));
        let eq30_e2234_d_b8: f64 = ((s.db[114][8] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b8));
        let eq30_e2234_d_b9: f64 = ((s.db[114][9] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b9));
        let eq30_e2234_d_b10: f64 = ((s.db[114][10] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b10));
        let eq30_e2234_d_b11: f64 = ((s.db[114][11] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b11));
        let eq30_e2234_d_b12: f64 = ((s.db[114][12] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b12));
        let eq30_e2234_d_b13: f64 = ((s.db[114][13] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b13));
        let eq30_e2234_d_b14: f64 = ((s.db[114][14] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b14));
        let eq30_e2234_d_b15: f64 = ((s.db[114][15] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b15));
        let eq30_e2234_d_b16: f64 = ((s.db[114][16] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b16));
        let eq30_e2234_d_b17: f64 = ((s.db[114][17] * eq30_e2233) + (s.v[114] * eq30_e2233_d_b17));
        (eq30_e2234, eq30_e2234_d_n0, eq30_e2234_d_n1, eq30_e2234_d_n2, eq30_e2234_d_n3, eq30_e2234_d_n4, eq30_e2234_d_n5, eq30_e2234_d_n6, eq30_e2234_d_n7, eq30_e2234_d_n8, eq30_e2234_d_n9, eq30_e2234_d_n10, eq30_e2234_d_n11, eq30_e2234_d_n12, eq30_e2234_d_n13, eq30_e2234_d_n14, eq30_e2234_d_n15, eq30_e2234_d_n16, eq30_e2234_d_b0, eq30_e2234_d_b1, eq30_e2234_d_b2, eq30_e2234_d_b3, eq30_e2234_d_b4, eq30_e2234_d_b5, eq30_e2234_d_b6, eq30_e2234_d_b7, eq30_e2234_d_b8, eq30_e2234_d_b9, eq30_e2234_d_b10, eq30_e2234_d_b11, eq30_e2234_d_b12, eq30_e2234_d_b13, eq30_e2234_d_b14, eq30_e2234_d_b15, eq30_e2234_d_b16, eq30_e2234_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e2236;
        let eq30_node_derivatives: [f64; 17] = [eq30_e2236_d_n0, eq30_e2236_d_n1, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n12, eq30_e2236_d_n13, eq30_e2236_d_n14, eq30_e2236_d_n15, eq30_e2236_d_n16];
        let eq30_branch_derivatives: [f64; 18] = [eq30_e2236_d_b0, eq30_e2236_d_b1, eq30_e2236_d_b2, eq30_e2236_d_b3, eq30_e2236_d_b4, eq30_e2236_d_b5, eq30_e2236_d_b6, eq30_e2236_d_b7, eq30_e2236_d_b8, eq30_e2236_d_b9, eq30_e2236_d_b10, eq30_e2236_d_b11, eq30_e2236_d_b12, eq30_e2236_d_b13, eq30_e2236_d_b14, eq30_e2236_d_b15, eq30_e2236_d_b16, eq30_e2236_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            multiplicity * (eq30_value),
            nodes,
            &eq30_node_derivatives,
            branches,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e2246, eq31_e2246_d_n0, eq31_e2246_d_n1, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n12, eq31_e2246_d_n13, eq31_e2246_d_n14, eq31_e2246_d_n15, eq31_e2246_d_n16, eq31_e2246_d_b0, eq31_e2246_d_b1, eq31_e2246_d_b2, eq31_e2246_d_b3, eq31_e2246_d_b4, eq31_e2246_d_b5, eq31_e2246_d_b6, eq31_e2246_d_b7, eq31_e2246_d_b8, eq31_e2246_d_b9, eq31_e2246_d_b10, eq31_e2246_d_b11, eq31_e2246_d_b12, eq31_e2246_d_b13, eq31_e2246_d_b14, eq31_e2246_d_b15, eq31_e2246_d_b16, eq31_e2246_d_b17,) = {
    if ((!s.b[1698]) && (!s.b[1701])) {
        let eq31_e2244: f64 = (s.v[114] * s.v[475]);
        let eq31_e2244_d_n0: f64 = ((s.dn[114][0] * s.v[475]) + (s.v[114] * s.dn[475][0]));
        let eq31_e2244_d_n1: f64 = ((s.dn[114][1] * s.v[475]) + (s.v[114] * s.dn[475][1]));
        let eq31_e2244_d_n2: f64 = ((s.dn[114][2] * s.v[475]) + (s.v[114] * s.dn[475][2]));
        let eq31_e2244_d_n3: f64 = ((s.dn[114][3] * s.v[475]) + (s.v[114] * s.dn[475][3]));
        let eq31_e2244_d_n4: f64 = ((s.dn[114][4] * s.v[475]) + (s.v[114] * s.dn[475][4]));
        let eq31_e2244_d_n5: f64 = ((s.dn[114][5] * s.v[475]) + (s.v[114] * s.dn[475][5]));
        let eq31_e2244_d_n6: f64 = ((s.dn[114][6] * s.v[475]) + (s.v[114] * s.dn[475][6]));
        let eq31_e2244_d_n7: f64 = ((s.dn[114][7] * s.v[475]) + (s.v[114] * s.dn[475][7]));
        let eq31_e2244_d_n8: f64 = ((s.dn[114][8] * s.v[475]) + (s.v[114] * s.dn[475][8]));
        let eq31_e2244_d_n9: f64 = ((s.dn[114][9] * s.v[475]) + (s.v[114] * s.dn[475][9]));
        let eq31_e2244_d_n10: f64 = ((s.dn[114][10] * s.v[475]) + (s.v[114] * s.dn[475][10]));
        let eq31_e2244_d_n11: f64 = ((s.dn[114][11] * s.v[475]) + (s.v[114] * s.dn[475][11]));
        let eq31_e2244_d_n12: f64 = ((s.dn[114][12] * s.v[475]) + (s.v[114] * s.dn[475][12]));
        let eq31_e2244_d_n13: f64 = ((s.dn[114][13] * s.v[475]) + (s.v[114] * s.dn[475][13]));
        let eq31_e2244_d_n14: f64 = ((s.dn[114][14] * s.v[475]) + (s.v[114] * s.dn[475][14]));
        let eq31_e2244_d_n15: f64 = ((s.dn[114][15] * s.v[475]) + (s.v[114] * s.dn[475][15]));
        let eq31_e2244_d_n16: f64 = ((s.dn[114][16] * s.v[475]) + (s.v[114] * s.dn[475][16]));
        let eq31_e2244_d_b0: f64 = ((s.db[114][0] * s.v[475]) + (s.v[114] * s.db[475][0]));
        let eq31_e2244_d_b1: f64 = ((s.db[114][1] * s.v[475]) + (s.v[114] * s.db[475][1]));
        let eq31_e2244_d_b2: f64 = ((s.db[114][2] * s.v[475]) + (s.v[114] * s.db[475][2]));
        let eq31_e2244_d_b3: f64 = ((s.db[114][3] * s.v[475]) + (s.v[114] * s.db[475][3]));
        let eq31_e2244_d_b4: f64 = ((s.db[114][4] * s.v[475]) + (s.v[114] * s.db[475][4]));
        let eq31_e2244_d_b5: f64 = ((s.db[114][5] * s.v[475]) + (s.v[114] * s.db[475][5]));
        let eq31_e2244_d_b6: f64 = ((s.db[114][6] * s.v[475]) + (s.v[114] * s.db[475][6]));
        let eq31_e2244_d_b7: f64 = ((s.db[114][7] * s.v[475]) + (s.v[114] * s.db[475][7]));
        let eq31_e2244_d_b8: f64 = ((s.db[114][8] * s.v[475]) + (s.v[114] * s.db[475][8]));
        let eq31_e2244_d_b9: f64 = ((s.db[114][9] * s.v[475]) + (s.v[114] * s.db[475][9]));
        let eq31_e2244_d_b10: f64 = ((s.db[114][10] * s.v[475]) + (s.v[114] * s.db[475][10]));
        let eq31_e2244_d_b11: f64 = ((s.db[114][11] * s.v[475]) + (s.v[114] * s.db[475][11]));
        let eq31_e2244_d_b12: f64 = ((s.db[114][12] * s.v[475]) + (s.v[114] * s.db[475][12]));
        let eq31_e2244_d_b13: f64 = ((s.db[114][13] * s.v[475]) + (s.v[114] * s.db[475][13]));
        let eq31_e2244_d_b14: f64 = ((s.db[114][14] * s.v[475]) + (s.v[114] * s.db[475][14]));
        let eq31_e2244_d_b15: f64 = ((s.db[114][15] * s.v[475]) + (s.v[114] * s.db[475][15]));
        let eq31_e2244_d_b16: f64 = ((s.db[114][16] * s.v[475]) + (s.v[114] * s.db[475][16]));
        let eq31_e2244_d_b17: f64 = ((s.db[114][17] * s.v[475]) + (s.v[114] * s.db[475][17]));
        (eq31_e2244, eq31_e2244_d_n0, eq31_e2244_d_n1, eq31_e2244_d_n2, eq31_e2244_d_n3, eq31_e2244_d_n4, eq31_e2244_d_n5, eq31_e2244_d_n6, eq31_e2244_d_n7, eq31_e2244_d_n8, eq31_e2244_d_n9, eq31_e2244_d_n10, eq31_e2244_d_n11, eq31_e2244_d_n12, eq31_e2244_d_n13, eq31_e2244_d_n14, eq31_e2244_d_n15, eq31_e2244_d_n16, eq31_e2244_d_b0, eq31_e2244_d_b1, eq31_e2244_d_b2, eq31_e2244_d_b3, eq31_e2244_d_b4, eq31_e2244_d_b5, eq31_e2244_d_b6, eq31_e2244_d_b7, eq31_e2244_d_b8, eq31_e2244_d_b9, eq31_e2244_d_b10, eq31_e2244_d_b11, eq31_e2244_d_b12, eq31_e2244_d_b13, eq31_e2244_d_b14, eq31_e2244_d_b15, eq31_e2244_d_b16, eq31_e2244_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e2246;
        let eq31_node_derivatives: [f64; 17] = [eq31_e2246_d_n0, eq31_e2246_d_n1, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n12, eq31_e2246_d_n13, eq31_e2246_d_n14, eq31_e2246_d_n15, eq31_e2246_d_n16];
        let eq31_branch_derivatives: [f64; 18] = [eq31_e2246_d_b0, eq31_e2246_d_b1, eq31_e2246_d_b2, eq31_e2246_d_b3, eq31_e2246_d_b4, eq31_e2246_d_b5, eq31_e2246_d_b6, eq31_e2246_d_b7, eq31_e2246_d_b8, eq31_e2246_d_b9, eq31_e2246_d_b10, eq31_e2246_d_b11, eq31_e2246_d_b12, eq31_e2246_d_b13, eq31_e2246_d_b14, eq31_e2246_d_b15, eq31_e2246_d_b16, eq31_e2246_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            multiplicity * (eq31_value),
            nodes,
            &eq31_node_derivatives,
            branches,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e2252, eq32_e2252_d_n0, eq32_e2252_d_n1, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n12, eq32_e2252_d_n13, eq32_e2252_d_n14, eq32_e2252_d_n15, eq32_e2252_d_n16, eq32_e2252_d_b0, eq32_e2252_d_b1, eq32_e2252_d_b2, eq32_e2252_d_b3, eq32_e2252_d_b4, eq32_e2252_d_b5, eq32_e2252_d_b6, eq32_e2252_d_b7, eq32_e2252_d_b8, eq32_e2252_d_b9, eq32_e2252_d_b10, eq32_e2252_d_b11, eq32_e2252_d_b12, eq32_e2252_d_b13, eq32_e2252_d_b14, eq32_e2252_d_b15, eq32_e2252_d_b16, eq32_e2252_d_b17,) = {
    if s.b[1703] {
        let eq32_e2250: f64 = (s.v[114] * s.v[464]);
        let eq32_e2250_d_n0: f64 = ((s.dn[114][0] * s.v[464]) + (s.v[114] * s.dn[464][0]));
        let eq32_e2250_d_n1: f64 = ((s.dn[114][1] * s.v[464]) + (s.v[114] * s.dn[464][1]));
        let eq32_e2250_d_n2: f64 = ((s.dn[114][2] * s.v[464]) + (s.v[114] * s.dn[464][2]));
        let eq32_e2250_d_n3: f64 = ((s.dn[114][3] * s.v[464]) + (s.v[114] * s.dn[464][3]));
        let eq32_e2250_d_n4: f64 = ((s.dn[114][4] * s.v[464]) + (s.v[114] * s.dn[464][4]));
        let eq32_e2250_d_n5: f64 = ((s.dn[114][5] * s.v[464]) + (s.v[114] * s.dn[464][5]));
        let eq32_e2250_d_n6: f64 = ((s.dn[114][6] * s.v[464]) + (s.v[114] * s.dn[464][6]));
        let eq32_e2250_d_n7: f64 = ((s.dn[114][7] * s.v[464]) + (s.v[114] * s.dn[464][7]));
        let eq32_e2250_d_n8: f64 = ((s.dn[114][8] * s.v[464]) + (s.v[114] * s.dn[464][8]));
        let eq32_e2250_d_n9: f64 = ((s.dn[114][9] * s.v[464]) + (s.v[114] * s.dn[464][9]));
        let eq32_e2250_d_n10: f64 = ((s.dn[114][10] * s.v[464]) + (s.v[114] * s.dn[464][10]));
        let eq32_e2250_d_n11: f64 = ((s.dn[114][11] * s.v[464]) + (s.v[114] * s.dn[464][11]));
        let eq32_e2250_d_n12: f64 = ((s.dn[114][12] * s.v[464]) + (s.v[114] * s.dn[464][12]));
        let eq32_e2250_d_n13: f64 = ((s.dn[114][13] * s.v[464]) + (s.v[114] * s.dn[464][13]));
        let eq32_e2250_d_n14: f64 = ((s.dn[114][14] * s.v[464]) + (s.v[114] * s.dn[464][14]));
        let eq32_e2250_d_n15: f64 = ((s.dn[114][15] * s.v[464]) + (s.v[114] * s.dn[464][15]));
        let eq32_e2250_d_n16: f64 = ((s.dn[114][16] * s.v[464]) + (s.v[114] * s.dn[464][16]));
        let eq32_e2250_d_b0: f64 = ((s.db[114][0] * s.v[464]) + (s.v[114] * s.db[464][0]));
        let eq32_e2250_d_b1: f64 = ((s.db[114][1] * s.v[464]) + (s.v[114] * s.db[464][1]));
        let eq32_e2250_d_b2: f64 = ((s.db[114][2] * s.v[464]) + (s.v[114] * s.db[464][2]));
        let eq32_e2250_d_b3: f64 = ((s.db[114][3] * s.v[464]) + (s.v[114] * s.db[464][3]));
        let eq32_e2250_d_b4: f64 = ((s.db[114][4] * s.v[464]) + (s.v[114] * s.db[464][4]));
        let eq32_e2250_d_b5: f64 = ((s.db[114][5] * s.v[464]) + (s.v[114] * s.db[464][5]));
        let eq32_e2250_d_b6: f64 = ((s.db[114][6] * s.v[464]) + (s.v[114] * s.db[464][6]));
        let eq32_e2250_d_b7: f64 = ((s.db[114][7] * s.v[464]) + (s.v[114] * s.db[464][7]));
        let eq32_e2250_d_b8: f64 = ((s.db[114][8] * s.v[464]) + (s.v[114] * s.db[464][8]));
        let eq32_e2250_d_b9: f64 = ((s.db[114][9] * s.v[464]) + (s.v[114] * s.db[464][9]));
        let eq32_e2250_d_b10: f64 = ((s.db[114][10] * s.v[464]) + (s.v[114] * s.db[464][10]));
        let eq32_e2250_d_b11: f64 = ((s.db[114][11] * s.v[464]) + (s.v[114] * s.db[464][11]));
        let eq32_e2250_d_b12: f64 = ((s.db[114][12] * s.v[464]) + (s.v[114] * s.db[464][12]));
        let eq32_e2250_d_b13: f64 = ((s.db[114][13] * s.v[464]) + (s.v[114] * s.db[464][13]));
        let eq32_e2250_d_b14: f64 = ((s.db[114][14] * s.v[464]) + (s.v[114] * s.db[464][14]));
        let eq32_e2250_d_b15: f64 = ((s.db[114][15] * s.v[464]) + (s.v[114] * s.db[464][15]));
        let eq32_e2250_d_b16: f64 = ((s.db[114][16] * s.v[464]) + (s.v[114] * s.db[464][16]));
        let eq32_e2250_d_b17: f64 = ((s.db[114][17] * s.v[464]) + (s.v[114] * s.db[464][17]));
        (eq32_e2250, eq32_e2250_d_n0, eq32_e2250_d_n1, eq32_e2250_d_n2, eq32_e2250_d_n3, eq32_e2250_d_n4, eq32_e2250_d_n5, eq32_e2250_d_n6, eq32_e2250_d_n7, eq32_e2250_d_n8, eq32_e2250_d_n9, eq32_e2250_d_n10, eq32_e2250_d_n11, eq32_e2250_d_n12, eq32_e2250_d_n13, eq32_e2250_d_n14, eq32_e2250_d_n15, eq32_e2250_d_n16, eq32_e2250_d_b0, eq32_e2250_d_b1, eq32_e2250_d_b2, eq32_e2250_d_b3, eq32_e2250_d_b4, eq32_e2250_d_b5, eq32_e2250_d_b6, eq32_e2250_d_b7, eq32_e2250_d_b8, eq32_e2250_d_b9, eq32_e2250_d_b10, eq32_e2250_d_b11, eq32_e2250_d_b12, eq32_e2250_d_b13, eq32_e2250_d_b14, eq32_e2250_d_b15, eq32_e2250_d_b16, eq32_e2250_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e2252;
        let eq32_node_derivatives: [f64; 17] = [eq32_e2252_d_n0, eq32_e2252_d_n1, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n12, eq32_e2252_d_n13, eq32_e2252_d_n14, eq32_e2252_d_n15, eq32_e2252_d_n16];
        let eq32_branch_derivatives: [f64; 18] = [eq32_e2252_d_b0, eq32_e2252_d_b1, eq32_e2252_d_b2, eq32_e2252_d_b3, eq32_e2252_d_b4, eq32_e2252_d_b5, eq32_e2252_d_b6, eq32_e2252_d_b7, eq32_e2252_d_b8, eq32_e2252_d_b9, eq32_e2252_d_b10, eq32_e2252_d_b11, eq32_e2252_d_b12, eq32_e2252_d_b13, eq32_e2252_d_b14, eq32_e2252_d_b15, eq32_e2252_d_b16, eq32_e2252_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            multiplicity * (eq32_value),
            nodes,
            &eq32_node_derivatives,
            branches,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e2258, eq33_e2258_d_n0, eq33_e2258_d_n1, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n12, eq33_e2258_d_n13, eq33_e2258_d_n14, eq33_e2258_d_n15, eq33_e2258_d_n16, eq33_e2258_d_b0, eq33_e2258_d_b1, eq33_e2258_d_b2, eq33_e2258_d_b3, eq33_e2258_d_b4, eq33_e2258_d_b5, eq33_e2258_d_b6, eq33_e2258_d_b7, eq33_e2258_d_b8, eq33_e2258_d_b9, eq33_e2258_d_b10, eq33_e2258_d_b11, eq33_e2258_d_b12, eq33_e2258_d_b13, eq33_e2258_d_b14, eq33_e2258_d_b15, eq33_e2258_d_b16, eq33_e2258_d_b17,) = {
    if s.b[1703] {
        let eq33_e2256: f64 = (s.v[114] * s.v[465]);
        let eq33_e2256_d_n0: f64 = ((s.dn[114][0] * s.v[465]) + (s.v[114] * s.dn[465][0]));
        let eq33_e2256_d_n1: f64 = ((s.dn[114][1] * s.v[465]) + (s.v[114] * s.dn[465][1]));
        let eq33_e2256_d_n2: f64 = ((s.dn[114][2] * s.v[465]) + (s.v[114] * s.dn[465][2]));
        let eq33_e2256_d_n3: f64 = ((s.dn[114][3] * s.v[465]) + (s.v[114] * s.dn[465][3]));
        let eq33_e2256_d_n4: f64 = ((s.dn[114][4] * s.v[465]) + (s.v[114] * s.dn[465][4]));
        let eq33_e2256_d_n5: f64 = ((s.dn[114][5] * s.v[465]) + (s.v[114] * s.dn[465][5]));
        let eq33_e2256_d_n6: f64 = ((s.dn[114][6] * s.v[465]) + (s.v[114] * s.dn[465][6]));
        let eq33_e2256_d_n7: f64 = ((s.dn[114][7] * s.v[465]) + (s.v[114] * s.dn[465][7]));
        let eq33_e2256_d_n8: f64 = ((s.dn[114][8] * s.v[465]) + (s.v[114] * s.dn[465][8]));
        let eq33_e2256_d_n9: f64 = ((s.dn[114][9] * s.v[465]) + (s.v[114] * s.dn[465][9]));
        let eq33_e2256_d_n10: f64 = ((s.dn[114][10] * s.v[465]) + (s.v[114] * s.dn[465][10]));
        let eq33_e2256_d_n11: f64 = ((s.dn[114][11] * s.v[465]) + (s.v[114] * s.dn[465][11]));
        let eq33_e2256_d_n12: f64 = ((s.dn[114][12] * s.v[465]) + (s.v[114] * s.dn[465][12]));
        let eq33_e2256_d_n13: f64 = ((s.dn[114][13] * s.v[465]) + (s.v[114] * s.dn[465][13]));
        let eq33_e2256_d_n14: f64 = ((s.dn[114][14] * s.v[465]) + (s.v[114] * s.dn[465][14]));
        let eq33_e2256_d_n15: f64 = ((s.dn[114][15] * s.v[465]) + (s.v[114] * s.dn[465][15]));
        let eq33_e2256_d_n16: f64 = ((s.dn[114][16] * s.v[465]) + (s.v[114] * s.dn[465][16]));
        let eq33_e2256_d_b0: f64 = ((s.db[114][0] * s.v[465]) + (s.v[114] * s.db[465][0]));
        let eq33_e2256_d_b1: f64 = ((s.db[114][1] * s.v[465]) + (s.v[114] * s.db[465][1]));
        let eq33_e2256_d_b2: f64 = ((s.db[114][2] * s.v[465]) + (s.v[114] * s.db[465][2]));
        let eq33_e2256_d_b3: f64 = ((s.db[114][3] * s.v[465]) + (s.v[114] * s.db[465][3]));
        let eq33_e2256_d_b4: f64 = ((s.db[114][4] * s.v[465]) + (s.v[114] * s.db[465][4]));
        let eq33_e2256_d_b5: f64 = ((s.db[114][5] * s.v[465]) + (s.v[114] * s.db[465][5]));
        let eq33_e2256_d_b6: f64 = ((s.db[114][6] * s.v[465]) + (s.v[114] * s.db[465][6]));
        let eq33_e2256_d_b7: f64 = ((s.db[114][7] * s.v[465]) + (s.v[114] * s.db[465][7]));
        let eq33_e2256_d_b8: f64 = ((s.db[114][8] * s.v[465]) + (s.v[114] * s.db[465][8]));
        let eq33_e2256_d_b9: f64 = ((s.db[114][9] * s.v[465]) + (s.v[114] * s.db[465][9]));
        let eq33_e2256_d_b10: f64 = ((s.db[114][10] * s.v[465]) + (s.v[114] * s.db[465][10]));
        let eq33_e2256_d_b11: f64 = ((s.db[114][11] * s.v[465]) + (s.v[114] * s.db[465][11]));
        let eq33_e2256_d_b12: f64 = ((s.db[114][12] * s.v[465]) + (s.v[114] * s.db[465][12]));
        let eq33_e2256_d_b13: f64 = ((s.db[114][13] * s.v[465]) + (s.v[114] * s.db[465][13]));
        let eq33_e2256_d_b14: f64 = ((s.db[114][14] * s.v[465]) + (s.v[114] * s.db[465][14]));
        let eq33_e2256_d_b15: f64 = ((s.db[114][15] * s.v[465]) + (s.v[114] * s.db[465][15]));
        let eq33_e2256_d_b16: f64 = ((s.db[114][16] * s.v[465]) + (s.v[114] * s.db[465][16]));
        let eq33_e2256_d_b17: f64 = ((s.db[114][17] * s.v[465]) + (s.v[114] * s.db[465][17]));
        (eq33_e2256, eq33_e2256_d_n0, eq33_e2256_d_n1, eq33_e2256_d_n2, eq33_e2256_d_n3, eq33_e2256_d_n4, eq33_e2256_d_n5, eq33_e2256_d_n6, eq33_e2256_d_n7, eq33_e2256_d_n8, eq33_e2256_d_n9, eq33_e2256_d_n10, eq33_e2256_d_n11, eq33_e2256_d_n12, eq33_e2256_d_n13, eq33_e2256_d_n14, eq33_e2256_d_n15, eq33_e2256_d_n16, eq33_e2256_d_b0, eq33_e2256_d_b1, eq33_e2256_d_b2, eq33_e2256_d_b3, eq33_e2256_d_b4, eq33_e2256_d_b5, eq33_e2256_d_b6, eq33_e2256_d_b7, eq33_e2256_d_b8, eq33_e2256_d_b9, eq33_e2256_d_b10, eq33_e2256_d_b11, eq33_e2256_d_b12, eq33_e2256_d_b13, eq33_e2256_d_b14, eq33_e2256_d_b15, eq33_e2256_d_b16, eq33_e2256_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e2258;
        let eq33_node_derivatives: [f64; 17] = [eq33_e2258_d_n0, eq33_e2258_d_n1, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n12, eq33_e2258_d_n13, eq33_e2258_d_n14, eq33_e2258_d_n15, eq33_e2258_d_n16];
        let eq33_branch_derivatives: [f64; 18] = [eq33_e2258_d_b0, eq33_e2258_d_b1, eq33_e2258_d_b2, eq33_e2258_d_b3, eq33_e2258_d_b4, eq33_e2258_d_b5, eq33_e2258_d_b6, eq33_e2258_d_b7, eq33_e2258_d_b8, eq33_e2258_d_b9, eq33_e2258_d_b10, eq33_e2258_d_b11, eq33_e2258_d_b12, eq33_e2258_d_b13, eq33_e2258_d_b14, eq33_e2258_d_b15, eq33_e2258_d_b16, eq33_e2258_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq33_value),
            nodes,
            &eq33_node_derivatives,
            branches,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq34_e2268, eq34_e2268_d_n0, eq34_e2268_d_n1, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n12, eq34_e2268_d_n13, eq34_e2268_d_n14, eq34_e2268_d_n15, eq34_e2268_d_n16, eq34_e2268_d_b0, eq34_e2268_d_b1, eq34_e2268_d_b2, eq34_e2268_d_b3, eq34_e2268_d_b4, eq34_e2268_d_b5, eq34_e2268_d_b6, eq34_e2268_d_b7, eq34_e2268_d_b8, eq34_e2268_d_b9, eq34_e2268_d_b10, eq34_e2268_d_b11, eq34_e2268_d_b12, eq34_e2268_d_b13, eq34_e2268_d_b14, eq34_e2268_d_b15, eq34_e2268_d_b16, eq34_e2268_d_b17,) = {
    if s.b[1704] {
        let eq34_e2262: f64 = (s.v[114] * s.v[519]);
        let eq34_e2262_d_n0: f64 = ((s.dn[114][0] * s.v[519]) + (s.v[114] * s.dn[519][0]));
        let eq34_e2262_d_n1: f64 = ((s.dn[114][1] * s.v[519]) + (s.v[114] * s.dn[519][1]));
        let eq34_e2262_d_n2: f64 = ((s.dn[114][2] * s.v[519]) + (s.v[114] * s.dn[519][2]));
        let eq34_e2262_d_n3: f64 = ((s.dn[114][3] * s.v[519]) + (s.v[114] * s.dn[519][3]));
        let eq34_e2262_d_n4: f64 = ((s.dn[114][4] * s.v[519]) + (s.v[114] * s.dn[519][4]));
        let eq34_e2262_d_n5: f64 = ((s.dn[114][5] * s.v[519]) + (s.v[114] * s.dn[519][5]));
        let eq34_e2262_d_n6: f64 = ((s.dn[114][6] * s.v[519]) + (s.v[114] * s.dn[519][6]));
        let eq34_e2262_d_n7: f64 = ((s.dn[114][7] * s.v[519]) + (s.v[114] * s.dn[519][7]));
        let eq34_e2262_d_n8: f64 = ((s.dn[114][8] * s.v[519]) + (s.v[114] * s.dn[519][8]));
        let eq34_e2262_d_n9: f64 = ((s.dn[114][9] * s.v[519]) + (s.v[114] * s.dn[519][9]));
        let eq34_e2262_d_n10: f64 = ((s.dn[114][10] * s.v[519]) + (s.v[114] * s.dn[519][10]));
        let eq34_e2262_d_n11: f64 = ((s.dn[114][11] * s.v[519]) + (s.v[114] * s.dn[519][11]));
        let eq34_e2262_d_n12: f64 = ((s.dn[114][12] * s.v[519]) + (s.v[114] * s.dn[519][12]));
        let eq34_e2262_d_n13: f64 = ((s.dn[114][13] * s.v[519]) + (s.v[114] * s.dn[519][13]));
        let eq34_e2262_d_n14: f64 = ((s.dn[114][14] * s.v[519]) + (s.v[114] * s.dn[519][14]));
        let eq34_e2262_d_n15: f64 = ((s.dn[114][15] * s.v[519]) + (s.v[114] * s.dn[519][15]));
        let eq34_e2262_d_n16: f64 = ((s.dn[114][16] * s.v[519]) + (s.v[114] * s.dn[519][16]));
        let eq34_e2262_d_b0: f64 = ((s.db[114][0] * s.v[519]) + (s.v[114] * s.db[519][0]));
        let eq34_e2262_d_b1: f64 = ((s.db[114][1] * s.v[519]) + (s.v[114] * s.db[519][1]));
        let eq34_e2262_d_b2: f64 = ((s.db[114][2] * s.v[519]) + (s.v[114] * s.db[519][2]));
        let eq34_e2262_d_b3: f64 = ((s.db[114][3] * s.v[519]) + (s.v[114] * s.db[519][3]));
        let eq34_e2262_d_b4: f64 = ((s.db[114][4] * s.v[519]) + (s.v[114] * s.db[519][4]));
        let eq34_e2262_d_b5: f64 = ((s.db[114][5] * s.v[519]) + (s.v[114] * s.db[519][5]));
        let eq34_e2262_d_b6: f64 = ((s.db[114][6] * s.v[519]) + (s.v[114] * s.db[519][6]));
        let eq34_e2262_d_b7: f64 = ((s.db[114][7] * s.v[519]) + (s.v[114] * s.db[519][7]));
        let eq34_e2262_d_b8: f64 = ((s.db[114][8] * s.v[519]) + (s.v[114] * s.db[519][8]));
        let eq34_e2262_d_b9: f64 = ((s.db[114][9] * s.v[519]) + (s.v[114] * s.db[519][9]));
        let eq34_e2262_d_b10: f64 = ((s.db[114][10] * s.v[519]) + (s.v[114] * s.db[519][10]));
        let eq34_e2262_d_b11: f64 = ((s.db[114][11] * s.v[519]) + (s.v[114] * s.db[519][11]));
        let eq34_e2262_d_b12: f64 = ((s.db[114][12] * s.v[519]) + (s.v[114] * s.db[519][12]));
        let eq34_e2262_d_b13: f64 = ((s.db[114][13] * s.v[519]) + (s.v[114] * s.db[519][13]));
        let eq34_e2262_d_b14: f64 = ((s.db[114][14] * s.v[519]) + (s.v[114] * s.db[519][14]));
        let eq34_e2262_d_b15: f64 = ((s.db[114][15] * s.v[519]) + (s.v[114] * s.db[519][15]));
        let eq34_e2262_d_b16: f64 = ((s.db[114][16] * s.v[519]) + (s.v[114] * s.db[519][16]));
        let eq34_e2262_d_b17: f64 = ((s.db[114][17] * s.v[519]) + (s.v[114] * s.db[519][17]));
        let eq34_e2265: f64 = ((nv3 - nv6) * s.v[1052]);
        let eq34_e2265_d_n3: f64 = s.v[1052];
        let eq34_e2265_d_n6: f64 = (-s.v[1052]);
        let eq34_e2266: f64 = (eq34_e2262 + eq34_e2265);
        let eq34_e2266_d_n3: f64 = (eq34_e2262_d_n3 + eq34_e2265_d_n3);
        let eq34_e2266_d_n6: f64 = (eq34_e2262_d_n6 + eq34_e2265_d_n6);
        (eq34_e2266, eq34_e2262_d_n0, eq34_e2262_d_n1, eq34_e2262_d_n2, eq34_e2266_d_n3, eq34_e2262_d_n4, eq34_e2262_d_n5, eq34_e2266_d_n6, eq34_e2262_d_n7, eq34_e2262_d_n8, eq34_e2262_d_n9, eq34_e2262_d_n10, eq34_e2262_d_n11, eq34_e2262_d_n12, eq34_e2262_d_n13, eq34_e2262_d_n14, eq34_e2262_d_n15, eq34_e2262_d_n16, eq34_e2262_d_b0, eq34_e2262_d_b1, eq34_e2262_d_b2, eq34_e2262_d_b3, eq34_e2262_d_b4, eq34_e2262_d_b5, eq34_e2262_d_b6, eq34_e2262_d_b7, eq34_e2262_d_b8, eq34_e2262_d_b9, eq34_e2262_d_b10, eq34_e2262_d_b11, eq34_e2262_d_b12, eq34_e2262_d_b13, eq34_e2262_d_b14, eq34_e2262_d_b15, eq34_e2262_d_b16, eq34_e2262_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e2268;
        let eq34_node_derivatives: [f64; 17] = [eq34_e2268_d_n0, eq34_e2268_d_n1, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n12, eq34_e2268_d_n13, eq34_e2268_d_n14, eq34_e2268_d_n15, eq34_e2268_d_n16];
        let eq34_branch_derivatives: [f64; 18] = [eq34_e2268_d_b0, eq34_e2268_d_b1, eq34_e2268_d_b2, eq34_e2268_d_b3, eq34_e2268_d_b4, eq34_e2268_d_b5, eq34_e2268_d_b6, eq34_e2268_d_b7, eq34_e2268_d_b8, eq34_e2268_d_b9, eq34_e2268_d_b10, eq34_e2268_d_b11, eq34_e2268_d_b12, eq34_e2268_d_b13, eq34_e2268_d_b14, eq34_e2268_d_b15, eq34_e2268_d_b16, eq34_e2268_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            multiplicity * (eq34_value),
            nodes,
            &eq34_node_derivatives,
            branches,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e2278, eq35_e2278_d_n0, eq35_e2278_d_n1, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n12, eq35_e2278_d_n13, eq35_e2278_d_n14, eq35_e2278_d_n15, eq35_e2278_d_n16, eq35_e2278_d_b0, eq35_e2278_d_b1, eq35_e2278_d_b2, eq35_e2278_d_b3, eq35_e2278_d_b4, eq35_e2278_d_b5, eq35_e2278_d_b6, eq35_e2278_d_b7, eq35_e2278_d_b8, eq35_e2278_d_b9, eq35_e2278_d_b10, eq35_e2278_d_b11, eq35_e2278_d_b12, eq35_e2278_d_b13, eq35_e2278_d_b14, eq35_e2278_d_b15, eq35_e2278_d_b16, eq35_e2278_d_b17,) = {
    if s.b[1704] {
        let eq35_e2272: f64 = (s.v[114] * s.v[520]);
        let eq35_e2272_d_n0: f64 = ((s.dn[114][0] * s.v[520]) + (s.v[114] * s.dn[520][0]));
        let eq35_e2272_d_n1: f64 = ((s.dn[114][1] * s.v[520]) + (s.v[114] * s.dn[520][1]));
        let eq35_e2272_d_n2: f64 = ((s.dn[114][2] * s.v[520]) + (s.v[114] * s.dn[520][2]));
        let eq35_e2272_d_n3: f64 = ((s.dn[114][3] * s.v[520]) + (s.v[114] * s.dn[520][3]));
        let eq35_e2272_d_n4: f64 = ((s.dn[114][4] * s.v[520]) + (s.v[114] * s.dn[520][4]));
        let eq35_e2272_d_n5: f64 = ((s.dn[114][5] * s.v[520]) + (s.v[114] * s.dn[520][5]));
        let eq35_e2272_d_n6: f64 = ((s.dn[114][6] * s.v[520]) + (s.v[114] * s.dn[520][6]));
        let eq35_e2272_d_n7: f64 = ((s.dn[114][7] * s.v[520]) + (s.v[114] * s.dn[520][7]));
        let eq35_e2272_d_n8: f64 = ((s.dn[114][8] * s.v[520]) + (s.v[114] * s.dn[520][8]));
        let eq35_e2272_d_n9: f64 = ((s.dn[114][9] * s.v[520]) + (s.v[114] * s.dn[520][9]));
        let eq35_e2272_d_n10: f64 = ((s.dn[114][10] * s.v[520]) + (s.v[114] * s.dn[520][10]));
        let eq35_e2272_d_n11: f64 = ((s.dn[114][11] * s.v[520]) + (s.v[114] * s.dn[520][11]));
        let eq35_e2272_d_n12: f64 = ((s.dn[114][12] * s.v[520]) + (s.v[114] * s.dn[520][12]));
        let eq35_e2272_d_n13: f64 = ((s.dn[114][13] * s.v[520]) + (s.v[114] * s.dn[520][13]));
        let eq35_e2272_d_n14: f64 = ((s.dn[114][14] * s.v[520]) + (s.v[114] * s.dn[520][14]));
        let eq35_e2272_d_n15: f64 = ((s.dn[114][15] * s.v[520]) + (s.v[114] * s.dn[520][15]));
        let eq35_e2272_d_n16: f64 = ((s.dn[114][16] * s.v[520]) + (s.v[114] * s.dn[520][16]));
        let eq35_e2272_d_b0: f64 = ((s.db[114][0] * s.v[520]) + (s.v[114] * s.db[520][0]));
        let eq35_e2272_d_b1: f64 = ((s.db[114][1] * s.v[520]) + (s.v[114] * s.db[520][1]));
        let eq35_e2272_d_b2: f64 = ((s.db[114][2] * s.v[520]) + (s.v[114] * s.db[520][2]));
        let eq35_e2272_d_b3: f64 = ((s.db[114][3] * s.v[520]) + (s.v[114] * s.db[520][3]));
        let eq35_e2272_d_b4: f64 = ((s.db[114][4] * s.v[520]) + (s.v[114] * s.db[520][4]));
        let eq35_e2272_d_b5: f64 = ((s.db[114][5] * s.v[520]) + (s.v[114] * s.db[520][5]));
        let eq35_e2272_d_b6: f64 = ((s.db[114][6] * s.v[520]) + (s.v[114] * s.db[520][6]));
        let eq35_e2272_d_b7: f64 = ((s.db[114][7] * s.v[520]) + (s.v[114] * s.db[520][7]));
        let eq35_e2272_d_b8: f64 = ((s.db[114][8] * s.v[520]) + (s.v[114] * s.db[520][8]));
        let eq35_e2272_d_b9: f64 = ((s.db[114][9] * s.v[520]) + (s.v[114] * s.db[520][9]));
        let eq35_e2272_d_b10: f64 = ((s.db[114][10] * s.v[520]) + (s.v[114] * s.db[520][10]));
        let eq35_e2272_d_b11: f64 = ((s.db[114][11] * s.v[520]) + (s.v[114] * s.db[520][11]));
        let eq35_e2272_d_b12: f64 = ((s.db[114][12] * s.v[520]) + (s.v[114] * s.db[520][12]));
        let eq35_e2272_d_b13: f64 = ((s.db[114][13] * s.v[520]) + (s.v[114] * s.db[520][13]));
        let eq35_e2272_d_b14: f64 = ((s.db[114][14] * s.v[520]) + (s.v[114] * s.db[520][14]));
        let eq35_e2272_d_b15: f64 = ((s.db[114][15] * s.v[520]) + (s.v[114] * s.db[520][15]));
        let eq35_e2272_d_b16: f64 = ((s.db[114][16] * s.v[520]) + (s.v[114] * s.db[520][16]));
        let eq35_e2272_d_b17: f64 = ((s.db[114][17] * s.v[520]) + (s.v[114] * s.db[520][17]));
        let eq35_e2275: f64 = ((nv3 - nv5) * s.v[1052]);
        let eq35_e2275_d_n3: f64 = s.v[1052];
        let eq35_e2275_d_n5: f64 = (-s.v[1052]);
        let eq35_e2276: f64 = (eq35_e2272 + eq35_e2275);
        let eq35_e2276_d_n3: f64 = (eq35_e2272_d_n3 + eq35_e2275_d_n3);
        let eq35_e2276_d_n5: f64 = (eq35_e2272_d_n5 + eq35_e2275_d_n5);
        (eq35_e2276, eq35_e2272_d_n0, eq35_e2272_d_n1, eq35_e2272_d_n2, eq35_e2276_d_n3, eq35_e2272_d_n4, eq35_e2276_d_n5, eq35_e2272_d_n6, eq35_e2272_d_n7, eq35_e2272_d_n8, eq35_e2272_d_n9, eq35_e2272_d_n10, eq35_e2272_d_n11, eq35_e2272_d_n12, eq35_e2272_d_n13, eq35_e2272_d_n14, eq35_e2272_d_n15, eq35_e2272_d_n16, eq35_e2272_d_b0, eq35_e2272_d_b1, eq35_e2272_d_b2, eq35_e2272_d_b3, eq35_e2272_d_b4, eq35_e2272_d_b5, eq35_e2272_d_b6, eq35_e2272_d_b7, eq35_e2272_d_b8, eq35_e2272_d_b9, eq35_e2272_d_b10, eq35_e2272_d_b11, eq35_e2272_d_b12, eq35_e2272_d_b13, eq35_e2272_d_b14, eq35_e2272_d_b15, eq35_e2272_d_b16, eq35_e2272_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e2278;
        let eq35_node_derivatives: [f64; 17] = [eq35_e2278_d_n0, eq35_e2278_d_n1, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n12, eq35_e2278_d_n13, eq35_e2278_d_n14, eq35_e2278_d_n15, eq35_e2278_d_n16];
        let eq35_branch_derivatives: [f64; 18] = [eq35_e2278_d_b0, eq35_e2278_d_b1, eq35_e2278_d_b2, eq35_e2278_d_b3, eq35_e2278_d_b4, eq35_e2278_d_b5, eq35_e2278_d_b6, eq35_e2278_d_b7, eq35_e2278_d_b8, eq35_e2278_d_b9, eq35_e2278_d_b10, eq35_e2278_d_b11, eq35_e2278_d_b12, eq35_e2278_d_b13, eq35_e2278_d_b14, eq35_e2278_d_b15, eq35_e2278_d_b16, eq35_e2278_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            multiplicity * (eq35_value),
            nodes,
            &eq35_node_derivatives,
            branches,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[507]);
        let eq36_e2281_d_n0: f64 = (s.dn[507][0] * ddt_scale);
        let eq36_e2281_d_n1: f64 = (s.dn[507][1] * ddt_scale);
        let eq36_e2281_d_n2: f64 = (s.dn[507][2] * ddt_scale);
        let eq36_e2281_d_n3: f64 = (s.dn[507][3] * ddt_scale);
        let eq36_e2281_d_n4: f64 = (s.dn[507][4] * ddt_scale);
        let eq36_e2281_d_n5: f64 = (s.dn[507][5] * ddt_scale);
        let eq36_e2281_d_n6: f64 = (s.dn[507][6] * ddt_scale);
        let eq36_e2281_d_n7: f64 = (s.dn[507][7] * ddt_scale);
        let eq36_e2281_d_n8: f64 = (s.dn[507][8] * ddt_scale);
        let eq36_e2281_d_n9: f64 = (s.dn[507][9] * ddt_scale);
        let eq36_e2281_d_n10: f64 = (s.dn[507][10] * ddt_scale);
        let eq36_e2281_d_n11: f64 = (s.dn[507][11] * ddt_scale);
        let eq36_e2281_d_n12: f64 = (s.dn[507][12] * ddt_scale);
        let eq36_e2281_d_n13: f64 = (s.dn[507][13] * ddt_scale);
        let eq36_e2281_d_n14: f64 = (s.dn[507][14] * ddt_scale);
        let eq36_e2281_d_n15: f64 = (s.dn[507][15] * ddt_scale);
        let eq36_e2281_d_n16: f64 = (s.dn[507][16] * ddt_scale);
        let eq36_e2281_d_b0: f64 = (s.db[507][0] * ddt_scale);
        let eq36_e2281_d_b1: f64 = (s.db[507][1] * ddt_scale);
        let eq36_e2281_d_b2: f64 = (s.db[507][2] * ddt_scale);
        let eq36_e2281_d_b3: f64 = (s.db[507][3] * ddt_scale);
        let eq36_e2281_d_b4: f64 = (s.db[507][4] * ddt_scale);
        let eq36_e2281_d_b5: f64 = (s.db[507][5] * ddt_scale);
        let eq36_e2281_d_b6: f64 = (s.db[507][6] * ddt_scale);
        let eq36_e2281_d_b7: f64 = (s.db[507][7] * ddt_scale);
        let eq36_e2281_d_b8: f64 = (s.db[507][8] * ddt_scale);
        let eq36_e2281_d_b9: f64 = (s.db[507][9] * ddt_scale);
        let eq36_e2281_d_b10: f64 = (s.db[507][10] * ddt_scale);
        let eq36_e2281_d_b11: f64 = (s.db[507][11] * ddt_scale);
        let eq36_e2281_d_b12: f64 = (s.db[507][12] * ddt_scale);
        let eq36_e2281_d_b13: f64 = (s.db[507][13] * ddt_scale);
        let eq36_e2281_d_b14: f64 = (s.db[507][14] * ddt_scale);
        let eq36_e2281_d_b15: f64 = (s.db[507][15] * ddt_scale);
        let eq36_e2281_d_b16: f64 = (s.db[507][16] * ddt_scale);
        let eq36_e2281_d_b17: f64 = (s.db[507][17] * ddt_scale);
        let eq36_e2282: f64 = (s.v[114] * eq36_e2281);
        let eq36_e2282_d_n0: f64 = ((s.dn[114][0] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n0));
        let eq36_e2282_d_n1: f64 = ((s.dn[114][1] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n1));
        let eq36_e2282_d_n2: f64 = ((s.dn[114][2] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n2));
        let eq36_e2282_d_n3: f64 = ((s.dn[114][3] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n3));
        let eq36_e2282_d_n4: f64 = ((s.dn[114][4] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n4));
        let eq36_e2282_d_n5: f64 = ((s.dn[114][5] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n5));
        let eq36_e2282_d_n6: f64 = ((s.dn[114][6] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n6));
        let eq36_e2282_d_n7: f64 = ((s.dn[114][7] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n7));
        let eq36_e2282_d_n8: f64 = ((s.dn[114][8] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n8));
        let eq36_e2282_d_n9: f64 = ((s.dn[114][9] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n9));
        let eq36_e2282_d_n10: f64 = ((s.dn[114][10] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n10));
        let eq36_e2282_d_n11: f64 = ((s.dn[114][11] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n11));
        let eq36_e2282_d_n12: f64 = ((s.dn[114][12] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n12));
        let eq36_e2282_d_n13: f64 = ((s.dn[114][13] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n13));
        let eq36_e2282_d_n14: f64 = ((s.dn[114][14] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n14));
        let eq36_e2282_d_n15: f64 = ((s.dn[114][15] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n15));
        let eq36_e2282_d_n16: f64 = ((s.dn[114][16] * eq36_e2281) + (s.v[114] * eq36_e2281_d_n16));
        let eq36_e2282_d_b0: f64 = ((s.db[114][0] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b0));
        let eq36_e2282_d_b1: f64 = ((s.db[114][1] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b1));
        let eq36_e2282_d_b2: f64 = ((s.db[114][2] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b2));
        let eq36_e2282_d_b3: f64 = ((s.db[114][3] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b3));
        let eq36_e2282_d_b4: f64 = ((s.db[114][4] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b4));
        let eq36_e2282_d_b5: f64 = ((s.db[114][5] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b5));
        let eq36_e2282_d_b6: f64 = ((s.db[114][6] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b6));
        let eq36_e2282_d_b7: f64 = ((s.db[114][7] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b7));
        let eq36_e2282_d_b8: f64 = ((s.db[114][8] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b8));
        let eq36_e2282_d_b9: f64 = ((s.db[114][9] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b9));
        let eq36_e2282_d_b10: f64 = ((s.db[114][10] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b10));
        let eq36_e2282_d_b11: f64 = ((s.db[114][11] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b11));
        let eq36_e2282_d_b12: f64 = ((s.db[114][12] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b12));
        let eq36_e2282_d_b13: f64 = ((s.db[114][13] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b13));
        let eq36_e2282_d_b14: f64 = ((s.db[114][14] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b14));
        let eq36_e2282_d_b15: f64 = ((s.db[114][15] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b15));
        let eq36_e2282_d_b16: f64 = ((s.db[114][16] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b16));
        let eq36_e2282_d_b17: f64 = ((s.db[114][17] * eq36_e2281) + (s.v[114] * eq36_e2281_d_b17));
        let eq36_value: f64 = eq36_e2282;
        let eq36_node_derivatives: [f64; 17] = [eq36_e2282_d_n0, eq36_e2282_d_n1, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, eq36_e2282_d_n12, eq36_e2282_d_n13, eq36_e2282_d_n14, eq36_e2282_d_n15, eq36_e2282_d_n16];
        let eq36_branch_derivatives: [f64; 18] = [eq36_e2282_d_b0, eq36_e2282_d_b1, eq36_e2282_d_b2, eq36_e2282_d_b3, eq36_e2282_d_b4, eq36_e2282_d_b5, eq36_e2282_d_b6, eq36_e2282_d_b7, eq36_e2282_d_b8, eq36_e2282_d_b9, eq36_e2282_d_b10, eq36_e2282_d_b11, eq36_e2282_d_b12, eq36_e2282_d_b13, eq36_e2282_d_b14, eq36_e2282_d_b15, eq36_e2282_d_b16, eq36_e2282_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            multiplicity * (eq36_value),
            nodes,
            &eq36_node_derivatives,
            branches,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let eq37_e2285: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, s.v[508]);
        let eq37_e2285_d_n0: f64 = (s.dn[508][0] * ddt_scale);
        let eq37_e2285_d_n1: f64 = (s.dn[508][1] * ddt_scale);
        let eq37_e2285_d_n2: f64 = (s.dn[508][2] * ddt_scale);
        let eq37_e2285_d_n3: f64 = (s.dn[508][3] * ddt_scale);
        let eq37_e2285_d_n4: f64 = (s.dn[508][4] * ddt_scale);
        let eq37_e2285_d_n5: f64 = (s.dn[508][5] * ddt_scale);
        let eq37_e2285_d_n6: f64 = (s.dn[508][6] * ddt_scale);
        let eq37_e2285_d_n7: f64 = (s.dn[508][7] * ddt_scale);
        let eq37_e2285_d_n8: f64 = (s.dn[508][8] * ddt_scale);
        let eq37_e2285_d_n9: f64 = (s.dn[508][9] * ddt_scale);
        let eq37_e2285_d_n10: f64 = (s.dn[508][10] * ddt_scale);
        let eq37_e2285_d_n11: f64 = (s.dn[508][11] * ddt_scale);
        let eq37_e2285_d_n12: f64 = (s.dn[508][12] * ddt_scale);
        let eq37_e2285_d_n13: f64 = (s.dn[508][13] * ddt_scale);
        let eq37_e2285_d_n14: f64 = (s.dn[508][14] * ddt_scale);
        let eq37_e2285_d_n15: f64 = (s.dn[508][15] * ddt_scale);
        let eq37_e2285_d_n16: f64 = (s.dn[508][16] * ddt_scale);
        let eq37_e2285_d_b0: f64 = (s.db[508][0] * ddt_scale);
        let eq37_e2285_d_b1: f64 = (s.db[508][1] * ddt_scale);
        let eq37_e2285_d_b2: f64 = (s.db[508][2] * ddt_scale);
        let eq37_e2285_d_b3: f64 = (s.db[508][3] * ddt_scale);
        let eq37_e2285_d_b4: f64 = (s.db[508][4] * ddt_scale);
        let eq37_e2285_d_b5: f64 = (s.db[508][5] * ddt_scale);
        let eq37_e2285_d_b6: f64 = (s.db[508][6] * ddt_scale);
        let eq37_e2285_d_b7: f64 = (s.db[508][7] * ddt_scale);
        let eq37_e2285_d_b8: f64 = (s.db[508][8] * ddt_scale);
        let eq37_e2285_d_b9: f64 = (s.db[508][9] * ddt_scale);
        let eq37_e2285_d_b10: f64 = (s.db[508][10] * ddt_scale);
        let eq37_e2285_d_b11: f64 = (s.db[508][11] * ddt_scale);
        let eq37_e2285_d_b12: f64 = (s.db[508][12] * ddt_scale);
        let eq37_e2285_d_b13: f64 = (s.db[508][13] * ddt_scale);
        let eq37_e2285_d_b14: f64 = (s.db[508][14] * ddt_scale);
        let eq37_e2285_d_b15: f64 = (s.db[508][15] * ddt_scale);
        let eq37_e2285_d_b16: f64 = (s.db[508][16] * ddt_scale);
        let eq37_e2285_d_b17: f64 = (s.db[508][17] * ddt_scale);
        let eq37_e2286: f64 = (s.v[114] * eq37_e2285);
        let eq37_e2286_d_n0: f64 = ((s.dn[114][0] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n0));
        let eq37_e2286_d_n1: f64 = ((s.dn[114][1] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n1));
        let eq37_e2286_d_n2: f64 = ((s.dn[114][2] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n2));
        let eq37_e2286_d_n3: f64 = ((s.dn[114][3] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n3));
        let eq37_e2286_d_n4: f64 = ((s.dn[114][4] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n4));
        let eq37_e2286_d_n5: f64 = ((s.dn[114][5] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n5));
        let eq37_e2286_d_n6: f64 = ((s.dn[114][6] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n6));
        let eq37_e2286_d_n7: f64 = ((s.dn[114][7] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n7));
        let eq37_e2286_d_n8: f64 = ((s.dn[114][8] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n8));
        let eq37_e2286_d_n9: f64 = ((s.dn[114][9] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n9));
        let eq37_e2286_d_n10: f64 = ((s.dn[114][10] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n10));
        let eq37_e2286_d_n11: f64 = ((s.dn[114][11] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n11));
        let eq37_e2286_d_n12: f64 = ((s.dn[114][12] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n12));
        let eq37_e2286_d_n13: f64 = ((s.dn[114][13] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n13));
        let eq37_e2286_d_n14: f64 = ((s.dn[114][14] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n14));
        let eq37_e2286_d_n15: f64 = ((s.dn[114][15] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n15));
        let eq37_e2286_d_n16: f64 = ((s.dn[114][16] * eq37_e2285) + (s.v[114] * eq37_e2285_d_n16));
        let eq37_e2286_d_b0: f64 = ((s.db[114][0] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b0));
        let eq37_e2286_d_b1: f64 = ((s.db[114][1] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b1));
        let eq37_e2286_d_b2: f64 = ((s.db[114][2] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b2));
        let eq37_e2286_d_b3: f64 = ((s.db[114][3] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b3));
        let eq37_e2286_d_b4: f64 = ((s.db[114][4] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b4));
        let eq37_e2286_d_b5: f64 = ((s.db[114][5] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b5));
        let eq37_e2286_d_b6: f64 = ((s.db[114][6] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b6));
        let eq37_e2286_d_b7: f64 = ((s.db[114][7] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b7));
        let eq37_e2286_d_b8: f64 = ((s.db[114][8] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b8));
        let eq37_e2286_d_b9: f64 = ((s.db[114][9] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b9));
        let eq37_e2286_d_b10: f64 = ((s.db[114][10] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b10));
        let eq37_e2286_d_b11: f64 = ((s.db[114][11] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b11));
        let eq37_e2286_d_b12: f64 = ((s.db[114][12] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b12));
        let eq37_e2286_d_b13: f64 = ((s.db[114][13] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b13));
        let eq37_e2286_d_b14: f64 = ((s.db[114][14] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b14));
        let eq37_e2286_d_b15: f64 = ((s.db[114][15] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b15));
        let eq37_e2286_d_b16: f64 = ((s.db[114][16] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b16));
        let eq37_e2286_d_b17: f64 = ((s.db[114][17] * eq37_e2285) + (s.v[114] * eq37_e2285_d_b17));
        let eq37_value: f64 = eq37_e2286;
        let eq37_node_derivatives: [f64; 17] = [eq37_e2286_d_n0, eq37_e2286_d_n1, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, eq37_e2286_d_n12, eq37_e2286_d_n13, eq37_e2286_d_n14, eq37_e2286_d_n15, eq37_e2286_d_n16];
        let eq37_branch_derivatives: [f64; 18] = [eq37_e2286_d_b0, eq37_e2286_d_b1, eq37_e2286_d_b2, eq37_e2286_d_b3, eq37_e2286_d_b4, eq37_e2286_d_b5, eq37_e2286_d_b6, eq37_e2286_d_b7, eq37_e2286_d_b8, eq37_e2286_d_b9, eq37_e2286_d_b10, eq37_e2286_d_b11, eq37_e2286_d_b12, eq37_e2286_d_b13, eq37_e2286_d_b14, eq37_e2286_d_b15, eq37_e2286_d_b16, eq37_e2286_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            multiplicity * (eq37_value),
            nodes,
            &eq37_node_derivatives,
            branches,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, s.v[509]);
        let eq38_e2289_d_n0: f64 = (s.dn[509][0] * ddt_scale);
        let eq38_e2289_d_n1: f64 = (s.dn[509][1] * ddt_scale);
        let eq38_e2289_d_n2: f64 = (s.dn[509][2] * ddt_scale);
        let eq38_e2289_d_n3: f64 = (s.dn[509][3] * ddt_scale);
        let eq38_e2289_d_n4: f64 = (s.dn[509][4] * ddt_scale);
        let eq38_e2289_d_n5: f64 = (s.dn[509][5] * ddt_scale);
        let eq38_e2289_d_n6: f64 = (s.dn[509][6] * ddt_scale);
        let eq38_e2289_d_n7: f64 = (s.dn[509][7] * ddt_scale);
        let eq38_e2289_d_n8: f64 = (s.dn[509][8] * ddt_scale);
        let eq38_e2289_d_n9: f64 = (s.dn[509][9] * ddt_scale);
        let eq38_e2289_d_n10: f64 = (s.dn[509][10] * ddt_scale);
        let eq38_e2289_d_n11: f64 = (s.dn[509][11] * ddt_scale);
        let eq38_e2289_d_n12: f64 = (s.dn[509][12] * ddt_scale);
        let eq38_e2289_d_n13: f64 = (s.dn[509][13] * ddt_scale);
        let eq38_e2289_d_n14: f64 = (s.dn[509][14] * ddt_scale);
        let eq38_e2289_d_n15: f64 = (s.dn[509][15] * ddt_scale);
        let eq38_e2289_d_n16: f64 = (s.dn[509][16] * ddt_scale);
        let eq38_e2289_d_b0: f64 = (s.db[509][0] * ddt_scale);
        let eq38_e2289_d_b1: f64 = (s.db[509][1] * ddt_scale);
        let eq38_e2289_d_b2: f64 = (s.db[509][2] * ddt_scale);
        let eq38_e2289_d_b3: f64 = (s.db[509][3] * ddt_scale);
        let eq38_e2289_d_b4: f64 = (s.db[509][4] * ddt_scale);
        let eq38_e2289_d_b5: f64 = (s.db[509][5] * ddt_scale);
        let eq38_e2289_d_b6: f64 = (s.db[509][6] * ddt_scale);
        let eq38_e2289_d_b7: f64 = (s.db[509][7] * ddt_scale);
        let eq38_e2289_d_b8: f64 = (s.db[509][8] * ddt_scale);
        let eq38_e2289_d_b9: f64 = (s.db[509][9] * ddt_scale);
        let eq38_e2289_d_b10: f64 = (s.db[509][10] * ddt_scale);
        let eq38_e2289_d_b11: f64 = (s.db[509][11] * ddt_scale);
        let eq38_e2289_d_b12: f64 = (s.db[509][12] * ddt_scale);
        let eq38_e2289_d_b13: f64 = (s.db[509][13] * ddt_scale);
        let eq38_e2289_d_b14: f64 = (s.db[509][14] * ddt_scale);
        let eq38_e2289_d_b15: f64 = (s.db[509][15] * ddt_scale);
        let eq38_e2289_d_b16: f64 = (s.db[509][16] * ddt_scale);
        let eq38_e2289_d_b17: f64 = (s.db[509][17] * ddt_scale);
        let eq38_e2290: f64 = (s.v[114] * eq38_e2289);
        let eq38_e2290_d_n0: f64 = ((s.dn[114][0] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n0));
        let eq38_e2290_d_n1: f64 = ((s.dn[114][1] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n1));
        let eq38_e2290_d_n2: f64 = ((s.dn[114][2] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n2));
        let eq38_e2290_d_n3: f64 = ((s.dn[114][3] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n3));
        let eq38_e2290_d_n4: f64 = ((s.dn[114][4] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n4));
        let eq38_e2290_d_n5: f64 = ((s.dn[114][5] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n5));
        let eq38_e2290_d_n6: f64 = ((s.dn[114][6] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n6));
        let eq38_e2290_d_n7: f64 = ((s.dn[114][7] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n7));
        let eq38_e2290_d_n8: f64 = ((s.dn[114][8] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n8));
        let eq38_e2290_d_n9: f64 = ((s.dn[114][9] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n9));
        let eq38_e2290_d_n10: f64 = ((s.dn[114][10] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n10));
        let eq38_e2290_d_n11: f64 = ((s.dn[114][11] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n11));
        let eq38_e2290_d_n12: f64 = ((s.dn[114][12] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n12));
        let eq38_e2290_d_n13: f64 = ((s.dn[114][13] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n13));
        let eq38_e2290_d_n14: f64 = ((s.dn[114][14] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n14));
        let eq38_e2290_d_n15: f64 = ((s.dn[114][15] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n15));
        let eq38_e2290_d_n16: f64 = ((s.dn[114][16] * eq38_e2289) + (s.v[114] * eq38_e2289_d_n16));
        let eq38_e2290_d_b0: f64 = ((s.db[114][0] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b0));
        let eq38_e2290_d_b1: f64 = ((s.db[114][1] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b1));
        let eq38_e2290_d_b2: f64 = ((s.db[114][2] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b2));
        let eq38_e2290_d_b3: f64 = ((s.db[114][3] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b3));
        let eq38_e2290_d_b4: f64 = ((s.db[114][4] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b4));
        let eq38_e2290_d_b5: f64 = ((s.db[114][5] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b5));
        let eq38_e2290_d_b6: f64 = ((s.db[114][6] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b6));
        let eq38_e2290_d_b7: f64 = ((s.db[114][7] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b7));
        let eq38_e2290_d_b8: f64 = ((s.db[114][8] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b8));
        let eq38_e2290_d_b9: f64 = ((s.db[114][9] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b9));
        let eq38_e2290_d_b10: f64 = ((s.db[114][10] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b10));
        let eq38_e2290_d_b11: f64 = ((s.db[114][11] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b11));
        let eq38_e2290_d_b12: f64 = ((s.db[114][12] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b12));
        let eq38_e2290_d_b13: f64 = ((s.db[114][13] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b13));
        let eq38_e2290_d_b14: f64 = ((s.db[114][14] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b14));
        let eq38_e2290_d_b15: f64 = ((s.db[114][15] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b15));
        let eq38_e2290_d_b16: f64 = ((s.db[114][16] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b16));
        let eq38_e2290_d_b17: f64 = ((s.db[114][17] * eq38_e2289) + (s.v[114] * eq38_e2289_d_b17));
        let eq38_value: f64 = eq38_e2290;
        let eq38_node_derivatives: [f64; 17] = [eq38_e2290_d_n0, eq38_e2290_d_n1, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, eq38_e2290_d_n12, eq38_e2290_d_n13, eq38_e2290_d_n14, eq38_e2290_d_n15, eq38_e2290_d_n16];
        let eq38_branch_derivatives: [f64; 18] = [eq38_e2290_d_b0, eq38_e2290_d_b1, eq38_e2290_d_b2, eq38_e2290_d_b3, eq38_e2290_d_b4, eq38_e2290_d_b5, eq38_e2290_d_b6, eq38_e2290_d_b7, eq38_e2290_d_b8, eq38_e2290_d_b9, eq38_e2290_d_b10, eq38_e2290_d_b11, eq38_e2290_d_b12, eq38_e2290_d_b13, eq38_e2290_d_b14, eq38_e2290_d_b15, eq38_e2290_d_b16, eq38_e2290_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            multiplicity * (eq38_value),
            nodes,
            &eq38_node_derivatives,
            branches,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16, eq39_e2295_d_b0, eq39_e2295_d_b1, eq39_e2295_d_b2, eq39_e2295_d_b3, eq39_e2295_d_b4, eq39_e2295_d_b5, eq39_e2295_d_b6, eq39_e2295_d_b7, eq39_e2295_d_b8, eq39_e2295_d_b9, eq39_e2295_d_b10, eq39_e2295_d_b11, eq39_e2295_d_b12, eq39_e2295_d_b13, eq39_e2295_d_b14, eq39_e2295_d_b15, eq39_e2295_d_b16, eq39_e2295_d_b17,) = {
    if s.b[1705] {
        let eq39_e2293: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, s.v[505]);
        let eq39_e2293_d_n0: f64 = (s.dn[505][0] * ddt_scale);
        let eq39_e2293_d_n1: f64 = (s.dn[505][1] * ddt_scale);
        let eq39_e2293_d_n2: f64 = (s.dn[505][2] * ddt_scale);
        let eq39_e2293_d_n3: f64 = (s.dn[505][3] * ddt_scale);
        let eq39_e2293_d_n4: f64 = (s.dn[505][4] * ddt_scale);
        let eq39_e2293_d_n5: f64 = (s.dn[505][5] * ddt_scale);
        let eq39_e2293_d_n6: f64 = (s.dn[505][6] * ddt_scale);
        let eq39_e2293_d_n7: f64 = (s.dn[505][7] * ddt_scale);
        let eq39_e2293_d_n8: f64 = (s.dn[505][8] * ddt_scale);
        let eq39_e2293_d_n9: f64 = (s.dn[505][9] * ddt_scale);
        let eq39_e2293_d_n10: f64 = (s.dn[505][10] * ddt_scale);
        let eq39_e2293_d_n11: f64 = (s.dn[505][11] * ddt_scale);
        let eq39_e2293_d_n12: f64 = (s.dn[505][12] * ddt_scale);
        let eq39_e2293_d_n13: f64 = (s.dn[505][13] * ddt_scale);
        let eq39_e2293_d_n14: f64 = (s.dn[505][14] * ddt_scale);
        let eq39_e2293_d_n15: f64 = (s.dn[505][15] * ddt_scale);
        let eq39_e2293_d_n16: f64 = (s.dn[505][16] * ddt_scale);
        let eq39_e2293_d_b0: f64 = (s.db[505][0] * ddt_scale);
        let eq39_e2293_d_b1: f64 = (s.db[505][1] * ddt_scale);
        let eq39_e2293_d_b2: f64 = (s.db[505][2] * ddt_scale);
        let eq39_e2293_d_b3: f64 = (s.db[505][3] * ddt_scale);
        let eq39_e2293_d_b4: f64 = (s.db[505][4] * ddt_scale);
        let eq39_e2293_d_b5: f64 = (s.db[505][5] * ddt_scale);
        let eq39_e2293_d_b6: f64 = (s.db[505][6] * ddt_scale);
        let eq39_e2293_d_b7: f64 = (s.db[505][7] * ddt_scale);
        let eq39_e2293_d_b8: f64 = (s.db[505][8] * ddt_scale);
        let eq39_e2293_d_b9: f64 = (s.db[505][9] * ddt_scale);
        let eq39_e2293_d_b10: f64 = (s.db[505][10] * ddt_scale);
        let eq39_e2293_d_b11: f64 = (s.db[505][11] * ddt_scale);
        let eq39_e2293_d_b12: f64 = (s.db[505][12] * ddt_scale);
        let eq39_e2293_d_b13: f64 = (s.db[505][13] * ddt_scale);
        let eq39_e2293_d_b14: f64 = (s.db[505][14] * ddt_scale);
        let eq39_e2293_d_b15: f64 = (s.db[505][15] * ddt_scale);
        let eq39_e2293_d_b16: f64 = (s.db[505][16] * ddt_scale);
        let eq39_e2293_d_b17: f64 = (s.db[505][17] * ddt_scale);
        (eq39_e2293, eq39_e2293_d_n0, eq39_e2293_d_n1, eq39_e2293_d_n2, eq39_e2293_d_n3, eq39_e2293_d_n4, eq39_e2293_d_n5, eq39_e2293_d_n6, eq39_e2293_d_n7, eq39_e2293_d_n8, eq39_e2293_d_n9, eq39_e2293_d_n10, eq39_e2293_d_n11, eq39_e2293_d_n12, eq39_e2293_d_n13, eq39_e2293_d_n14, eq39_e2293_d_n15, eq39_e2293_d_n16, eq39_e2293_d_b0, eq39_e2293_d_b1, eq39_e2293_d_b2, eq39_e2293_d_b3, eq39_e2293_d_b4, eq39_e2293_d_b5, eq39_e2293_d_b6, eq39_e2293_d_b7, eq39_e2293_d_b8, eq39_e2293_d_b9, eq39_e2293_d_b10, eq39_e2293_d_b11, eq39_e2293_d_b12, eq39_e2293_d_b13, eq39_e2293_d_b14, eq39_e2293_d_b15, eq39_e2293_d_b16, eq39_e2293_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e2295;
        let eq39_node_derivatives: [f64; 17] = [eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16];
        let eq39_branch_derivatives: [f64; 18] = [eq39_e2295_d_b0, eq39_e2295_d_b1, eq39_e2295_d_b2, eq39_e2295_d_b3, eq39_e2295_d_b4, eq39_e2295_d_b5, eq39_e2295_d_b6, eq39_e2295_d_b7, eq39_e2295_d_b8, eq39_e2295_d_b9, eq39_e2295_d_b10, eq39_e2295_d_b11, eq39_e2295_d_b12, eq39_e2295_d_b13, eq39_e2295_d_b14, eq39_e2295_d_b15, eq39_e2295_d_b16, eq39_e2295_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            multiplicity * (eq39_value),
            nodes,
            &eq39_node_derivatives,
            branches,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16, eq40_e2302_d_b0, eq40_e2302_d_b1, eq40_e2302_d_b2, eq40_e2302_d_b3, eq40_e2302_d_b4, eq40_e2302_d_b5, eq40_e2302_d_b6, eq40_e2302_d_b7, eq40_e2302_d_b8, eq40_e2302_d_b9, eq40_e2302_d_b10, eq40_e2302_d_b11, eq40_e2302_d_b12, eq40_e2302_d_b13, eq40_e2302_d_b14, eq40_e2302_d_b15, eq40_e2302_d_b16, eq40_e2302_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq40_e2300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[506]);
        let eq40_e2300_d_n0: f64 = (s.dn[506][0] * ddt_scale);
        let eq40_e2300_d_n1: f64 = (s.dn[506][1] * ddt_scale);
        let eq40_e2300_d_n2: f64 = (s.dn[506][2] * ddt_scale);
        let eq40_e2300_d_n3: f64 = (s.dn[506][3] * ddt_scale);
        let eq40_e2300_d_n4: f64 = (s.dn[506][4] * ddt_scale);
        let eq40_e2300_d_n5: f64 = (s.dn[506][5] * ddt_scale);
        let eq40_e2300_d_n6: f64 = (s.dn[506][6] * ddt_scale);
        let eq40_e2300_d_n7: f64 = (s.dn[506][7] * ddt_scale);
        let eq40_e2300_d_n8: f64 = (s.dn[506][8] * ddt_scale);
        let eq40_e2300_d_n9: f64 = (s.dn[506][9] * ddt_scale);
        let eq40_e2300_d_n10: f64 = (s.dn[506][10] * ddt_scale);
        let eq40_e2300_d_n11: f64 = (s.dn[506][11] * ddt_scale);
        let eq40_e2300_d_n12: f64 = (s.dn[506][12] * ddt_scale);
        let eq40_e2300_d_n13: f64 = (s.dn[506][13] * ddt_scale);
        let eq40_e2300_d_n14: f64 = (s.dn[506][14] * ddt_scale);
        let eq40_e2300_d_n15: f64 = (s.dn[506][15] * ddt_scale);
        let eq40_e2300_d_n16: f64 = (s.dn[506][16] * ddt_scale);
        let eq40_e2300_d_b0: f64 = (s.db[506][0] * ddt_scale);
        let eq40_e2300_d_b1: f64 = (s.db[506][1] * ddt_scale);
        let eq40_e2300_d_b2: f64 = (s.db[506][2] * ddt_scale);
        let eq40_e2300_d_b3: f64 = (s.db[506][3] * ddt_scale);
        let eq40_e2300_d_b4: f64 = (s.db[506][4] * ddt_scale);
        let eq40_e2300_d_b5: f64 = (s.db[506][5] * ddt_scale);
        let eq40_e2300_d_b6: f64 = (s.db[506][6] * ddt_scale);
        let eq40_e2300_d_b7: f64 = (s.db[506][7] * ddt_scale);
        let eq40_e2300_d_b8: f64 = (s.db[506][8] * ddt_scale);
        let eq40_e2300_d_b9: f64 = (s.db[506][9] * ddt_scale);
        let eq40_e2300_d_b10: f64 = (s.db[506][10] * ddt_scale);
        let eq40_e2300_d_b11: f64 = (s.db[506][11] * ddt_scale);
        let eq40_e2300_d_b12: f64 = (s.db[506][12] * ddt_scale);
        let eq40_e2300_d_b13: f64 = (s.db[506][13] * ddt_scale);
        let eq40_e2300_d_b14: f64 = (s.db[506][14] * ddt_scale);
        let eq40_e2300_d_b15: f64 = (s.db[506][15] * ddt_scale);
        let eq40_e2300_d_b16: f64 = (s.db[506][16] * ddt_scale);
        let eq40_e2300_d_b17: f64 = (s.db[506][17] * ddt_scale);
        (eq40_e2300, eq40_e2300_d_n0, eq40_e2300_d_n1, eq40_e2300_d_n2, eq40_e2300_d_n3, eq40_e2300_d_n4, eq40_e2300_d_n5, eq40_e2300_d_n6, eq40_e2300_d_n7, eq40_e2300_d_n8, eq40_e2300_d_n9, eq40_e2300_d_n10, eq40_e2300_d_n11, eq40_e2300_d_n12, eq40_e2300_d_n13, eq40_e2300_d_n14, eq40_e2300_d_n15, eq40_e2300_d_n16, eq40_e2300_d_b0, eq40_e2300_d_b1, eq40_e2300_d_b2, eq40_e2300_d_b3, eq40_e2300_d_b4, eq40_e2300_d_b5, eq40_e2300_d_b6, eq40_e2300_d_b7, eq40_e2300_d_b8, eq40_e2300_d_b9, eq40_e2300_d_b10, eq40_e2300_d_b11, eq40_e2300_d_b12, eq40_e2300_d_b13, eq40_e2300_d_b14, eq40_e2300_d_b15, eq40_e2300_d_b16, eq40_e2300_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e2302;
        let eq40_node_derivatives: [f64; 17] = [eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16];
        let eq40_branch_derivatives: [f64; 18] = [eq40_e2302_d_b0, eq40_e2302_d_b1, eq40_e2302_d_b2, eq40_e2302_d_b3, eq40_e2302_d_b4, eq40_e2302_d_b5, eq40_e2302_d_b6, eq40_e2302_d_b7, eq40_e2302_d_b8, eq40_e2302_d_b9, eq40_e2302_d_b10, eq40_e2302_d_b11, eq40_e2302_d_b12, eq40_e2302_d_b13, eq40_e2302_d_b14, eq40_e2302_d_b15, eq40_e2302_d_b16, eq40_e2302_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            multiplicity * (eq40_value),
            nodes,
            &eq40_node_derivatives,
            branches,
            &eq40_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16, eq41_e2311_d_b0, eq41_e2311_d_b1, eq41_e2311_d_b2, eq41_e2311_d_b3, eq41_e2311_d_b4, eq41_e2311_d_b5, eq41_e2311_d_b6, eq41_e2311_d_b7, eq41_e2311_d_b8, eq41_e2311_d_b9, eq41_e2311_d_b10, eq41_e2311_d_b11, eq41_e2311_d_b12, eq41_e2311_d_b13, eq41_e2311_d_b14, eq41_e2311_d_b15, eq41_e2311_d_b16, eq41_e2311_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq41_e2308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[503]);
        let eq41_e2308_d_n0: f64 = (s.dn[503][0] * ddt_scale);
        let eq41_e2308_d_n1: f64 = (s.dn[503][1] * ddt_scale);
        let eq41_e2308_d_n2: f64 = (s.dn[503][2] * ddt_scale);
        let eq41_e2308_d_n3: f64 = (s.dn[503][3] * ddt_scale);
        let eq41_e2308_d_n4: f64 = (s.dn[503][4] * ddt_scale);
        let eq41_e2308_d_n5: f64 = (s.dn[503][5] * ddt_scale);
        let eq41_e2308_d_n6: f64 = (s.dn[503][6] * ddt_scale);
        let eq41_e2308_d_n7: f64 = (s.dn[503][7] * ddt_scale);
        let eq41_e2308_d_n8: f64 = (s.dn[503][8] * ddt_scale);
        let eq41_e2308_d_n9: f64 = (s.dn[503][9] * ddt_scale);
        let eq41_e2308_d_n10: f64 = (s.dn[503][10] * ddt_scale);
        let eq41_e2308_d_n11: f64 = (s.dn[503][11] * ddt_scale);
        let eq41_e2308_d_n12: f64 = (s.dn[503][12] * ddt_scale);
        let eq41_e2308_d_n13: f64 = (s.dn[503][13] * ddt_scale);
        let eq41_e2308_d_n14: f64 = (s.dn[503][14] * ddt_scale);
        let eq41_e2308_d_n15: f64 = (s.dn[503][15] * ddt_scale);
        let eq41_e2308_d_n16: f64 = (s.dn[503][16] * ddt_scale);
        let eq41_e2308_d_b0: f64 = (s.db[503][0] * ddt_scale);
        let eq41_e2308_d_b1: f64 = (s.db[503][1] * ddt_scale);
        let eq41_e2308_d_b2: f64 = (s.db[503][2] * ddt_scale);
        let eq41_e2308_d_b3: f64 = (s.db[503][3] * ddt_scale);
        let eq41_e2308_d_b4: f64 = (s.db[503][4] * ddt_scale);
        let eq41_e2308_d_b5: f64 = (s.db[503][5] * ddt_scale);
        let eq41_e2308_d_b6: f64 = (s.db[503][6] * ddt_scale);
        let eq41_e2308_d_b7: f64 = (s.db[503][7] * ddt_scale);
        let eq41_e2308_d_b8: f64 = (s.db[503][8] * ddt_scale);
        let eq41_e2308_d_b9: f64 = (s.db[503][9] * ddt_scale);
        let eq41_e2308_d_b10: f64 = (s.db[503][10] * ddt_scale);
        let eq41_e2308_d_b11: f64 = (s.db[503][11] * ddt_scale);
        let eq41_e2308_d_b12: f64 = (s.db[503][12] * ddt_scale);
        let eq41_e2308_d_b13: f64 = (s.db[503][13] * ddt_scale);
        let eq41_e2308_d_b14: f64 = (s.db[503][14] * ddt_scale);
        let eq41_e2308_d_b15: f64 = (s.db[503][15] * ddt_scale);
        let eq41_e2308_d_b16: f64 = (s.db[503][16] * ddt_scale);
        let eq41_e2308_d_b17: f64 = (s.db[503][17] * ddt_scale);
        let eq41_e2309: f64 = (s.v[114] * eq41_e2308);
        let eq41_e2309_d_n0: f64 = ((s.dn[114][0] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n0));
        let eq41_e2309_d_n1: f64 = ((s.dn[114][1] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n1));
        let eq41_e2309_d_n2: f64 = ((s.dn[114][2] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n2));
        let eq41_e2309_d_n3: f64 = ((s.dn[114][3] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n3));
        let eq41_e2309_d_n4: f64 = ((s.dn[114][4] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n4));
        let eq41_e2309_d_n5: f64 = ((s.dn[114][5] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n5));
        let eq41_e2309_d_n6: f64 = ((s.dn[114][6] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n6));
        let eq41_e2309_d_n7: f64 = ((s.dn[114][7] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n7));
        let eq41_e2309_d_n8: f64 = ((s.dn[114][8] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n8));
        let eq41_e2309_d_n9: f64 = ((s.dn[114][9] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n9));
        let eq41_e2309_d_n10: f64 = ((s.dn[114][10] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n10));
        let eq41_e2309_d_n11: f64 = ((s.dn[114][11] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n11));
        let eq41_e2309_d_n12: f64 = ((s.dn[114][12] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n12));
        let eq41_e2309_d_n13: f64 = ((s.dn[114][13] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n13));
        let eq41_e2309_d_n14: f64 = ((s.dn[114][14] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n14));
        let eq41_e2309_d_n15: f64 = ((s.dn[114][15] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n15));
        let eq41_e2309_d_n16: f64 = ((s.dn[114][16] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n16));
        let eq41_e2309_d_b0: f64 = ((s.db[114][0] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b0));
        let eq41_e2309_d_b1: f64 = ((s.db[114][1] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b1));
        let eq41_e2309_d_b2: f64 = ((s.db[114][2] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b2));
        let eq41_e2309_d_b3: f64 = ((s.db[114][3] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b3));
        let eq41_e2309_d_b4: f64 = ((s.db[114][4] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b4));
        let eq41_e2309_d_b5: f64 = ((s.db[114][5] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b5));
        let eq41_e2309_d_b6: f64 = ((s.db[114][6] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b6));
        let eq41_e2309_d_b7: f64 = ((s.db[114][7] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b7));
        let eq41_e2309_d_b8: f64 = ((s.db[114][8] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b8));
        let eq41_e2309_d_b9: f64 = ((s.db[114][9] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b9));
        let eq41_e2309_d_b10: f64 = ((s.db[114][10] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b10));
        let eq41_e2309_d_b11: f64 = ((s.db[114][11] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b11));
        let eq41_e2309_d_b12: f64 = ((s.db[114][12] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b12));
        let eq41_e2309_d_b13: f64 = ((s.db[114][13] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b13));
        let eq41_e2309_d_b14: f64 = ((s.db[114][14] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b14));
        let eq41_e2309_d_b15: f64 = ((s.db[114][15] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b15));
        let eq41_e2309_d_b16: f64 = ((s.db[114][16] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b16));
        let eq41_e2309_d_b17: f64 = ((s.db[114][17] * eq41_e2308) + (s.v[114] * eq41_e2308_d_b17));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n1, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n12, eq41_e2309_d_n13, eq41_e2309_d_n14, eq41_e2309_d_n15, eq41_e2309_d_n16, eq41_e2309_d_b0, eq41_e2309_d_b1, eq41_e2309_d_b2, eq41_e2309_d_b3, eq41_e2309_d_b4, eq41_e2309_d_b5, eq41_e2309_d_b6, eq41_e2309_d_b7, eq41_e2309_d_b8, eq41_e2309_d_b9, eq41_e2309_d_b10, eq41_e2309_d_b11, eq41_e2309_d_b12, eq41_e2309_d_b13, eq41_e2309_d_b14, eq41_e2309_d_b15, eq41_e2309_d_b16, eq41_e2309_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e2311;
        let eq41_node_derivatives: [f64; 17] = [eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16];
        let eq41_branch_derivatives: [f64; 18] = [eq41_e2311_d_b0, eq41_e2311_d_b1, eq41_e2311_d_b2, eq41_e2311_d_b3, eq41_e2311_d_b4, eq41_e2311_d_b5, eq41_e2311_d_b6, eq41_e2311_d_b7, eq41_e2311_d_b8, eq41_e2311_d_b9, eq41_e2311_d_b10, eq41_e2311_d_b11, eq41_e2311_d_b12, eq41_e2311_d_b13, eq41_e2311_d_b14, eq41_e2311_d_b15, eq41_e2311_d_b16, eq41_e2311_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            multiplicity * (eq41_value),
            nodes,
            &eq41_node_derivatives,
            branches,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16, eq42_e2320_d_b0, eq42_e2320_d_b1, eq42_e2320_d_b2, eq42_e2320_d_b3, eq42_e2320_d_b4, eq42_e2320_d_b5, eq42_e2320_d_b6, eq42_e2320_d_b7, eq42_e2320_d_b8, eq42_e2320_d_b9, eq42_e2320_d_b10, eq42_e2320_d_b11, eq42_e2320_d_b12, eq42_e2320_d_b13, eq42_e2320_d_b14, eq42_e2320_d_b15, eq42_e2320_d_b16, eq42_e2320_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq42_e2317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, s.v[504]);
        let eq42_e2317_d_n0: f64 = (s.dn[504][0] * ddt_scale);
        let eq42_e2317_d_n1: f64 = (s.dn[504][1] * ddt_scale);
        let eq42_e2317_d_n2: f64 = (s.dn[504][2] * ddt_scale);
        let eq42_e2317_d_n3: f64 = (s.dn[504][3] * ddt_scale);
        let eq42_e2317_d_n4: f64 = (s.dn[504][4] * ddt_scale);
        let eq42_e2317_d_n5: f64 = (s.dn[504][5] * ddt_scale);
        let eq42_e2317_d_n6: f64 = (s.dn[504][6] * ddt_scale);
        let eq42_e2317_d_n7: f64 = (s.dn[504][7] * ddt_scale);
        let eq42_e2317_d_n8: f64 = (s.dn[504][8] * ddt_scale);
        let eq42_e2317_d_n9: f64 = (s.dn[504][9] * ddt_scale);
        let eq42_e2317_d_n10: f64 = (s.dn[504][10] * ddt_scale);
        let eq42_e2317_d_n11: f64 = (s.dn[504][11] * ddt_scale);
        let eq42_e2317_d_n12: f64 = (s.dn[504][12] * ddt_scale);
        let eq42_e2317_d_n13: f64 = (s.dn[504][13] * ddt_scale);
        let eq42_e2317_d_n14: f64 = (s.dn[504][14] * ddt_scale);
        let eq42_e2317_d_n15: f64 = (s.dn[504][15] * ddt_scale);
        let eq42_e2317_d_n16: f64 = (s.dn[504][16] * ddt_scale);
        let eq42_e2317_d_b0: f64 = (s.db[504][0] * ddt_scale);
        let eq42_e2317_d_b1: f64 = (s.db[504][1] * ddt_scale);
        let eq42_e2317_d_b2: f64 = (s.db[504][2] * ddt_scale);
        let eq42_e2317_d_b3: f64 = (s.db[504][3] * ddt_scale);
        let eq42_e2317_d_b4: f64 = (s.db[504][4] * ddt_scale);
        let eq42_e2317_d_b5: f64 = (s.db[504][5] * ddt_scale);
        let eq42_e2317_d_b6: f64 = (s.db[504][6] * ddt_scale);
        let eq42_e2317_d_b7: f64 = (s.db[504][7] * ddt_scale);
        let eq42_e2317_d_b8: f64 = (s.db[504][8] * ddt_scale);
        let eq42_e2317_d_b9: f64 = (s.db[504][9] * ddt_scale);
        let eq42_e2317_d_b10: f64 = (s.db[504][10] * ddt_scale);
        let eq42_e2317_d_b11: f64 = (s.db[504][11] * ddt_scale);
        let eq42_e2317_d_b12: f64 = (s.db[504][12] * ddt_scale);
        let eq42_e2317_d_b13: f64 = (s.db[504][13] * ddt_scale);
        let eq42_e2317_d_b14: f64 = (s.db[504][14] * ddt_scale);
        let eq42_e2317_d_b15: f64 = (s.db[504][15] * ddt_scale);
        let eq42_e2317_d_b16: f64 = (s.db[504][16] * ddt_scale);
        let eq42_e2317_d_b17: f64 = (s.db[504][17] * ddt_scale);
        let eq42_e2318: f64 = (s.v[114] * eq42_e2317);
        let eq42_e2318_d_n0: f64 = ((s.dn[114][0] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n0));
        let eq42_e2318_d_n1: f64 = ((s.dn[114][1] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n1));
        let eq42_e2318_d_n2: f64 = ((s.dn[114][2] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n2));
        let eq42_e2318_d_n3: f64 = ((s.dn[114][3] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n3));
        let eq42_e2318_d_n4: f64 = ((s.dn[114][4] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n4));
        let eq42_e2318_d_n5: f64 = ((s.dn[114][5] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n5));
        let eq42_e2318_d_n6: f64 = ((s.dn[114][6] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n6));
        let eq42_e2318_d_n7: f64 = ((s.dn[114][7] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n7));
        let eq42_e2318_d_n8: f64 = ((s.dn[114][8] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n8));
        let eq42_e2318_d_n9: f64 = ((s.dn[114][9] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n9));
        let eq42_e2318_d_n10: f64 = ((s.dn[114][10] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n10));
        let eq42_e2318_d_n11: f64 = ((s.dn[114][11] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n11));
        let eq42_e2318_d_n12: f64 = ((s.dn[114][12] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n12));
        let eq42_e2318_d_n13: f64 = ((s.dn[114][13] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n13));
        let eq42_e2318_d_n14: f64 = ((s.dn[114][14] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n14));
        let eq42_e2318_d_n15: f64 = ((s.dn[114][15] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n15));
        let eq42_e2318_d_n16: f64 = ((s.dn[114][16] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n16));
        let eq42_e2318_d_b0: f64 = ((s.db[114][0] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b0));
        let eq42_e2318_d_b1: f64 = ((s.db[114][1] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b1));
        let eq42_e2318_d_b2: f64 = ((s.db[114][2] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b2));
        let eq42_e2318_d_b3: f64 = ((s.db[114][3] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b3));
        let eq42_e2318_d_b4: f64 = ((s.db[114][4] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b4));
        let eq42_e2318_d_b5: f64 = ((s.db[114][5] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b5));
        let eq42_e2318_d_b6: f64 = ((s.db[114][6] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b6));
        let eq42_e2318_d_b7: f64 = ((s.db[114][7] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b7));
        let eq42_e2318_d_b8: f64 = ((s.db[114][8] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b8));
        let eq42_e2318_d_b9: f64 = ((s.db[114][9] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b9));
        let eq42_e2318_d_b10: f64 = ((s.db[114][10] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b10));
        let eq42_e2318_d_b11: f64 = ((s.db[114][11] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b11));
        let eq42_e2318_d_b12: f64 = ((s.db[114][12] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b12));
        let eq42_e2318_d_b13: f64 = ((s.db[114][13] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b13));
        let eq42_e2318_d_b14: f64 = ((s.db[114][14] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b14));
        let eq42_e2318_d_b15: f64 = ((s.db[114][15] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b15));
        let eq42_e2318_d_b16: f64 = ((s.db[114][16] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b16));
        let eq42_e2318_d_b17: f64 = ((s.db[114][17] * eq42_e2317) + (s.v[114] * eq42_e2317_d_b17));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n1, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n12, eq42_e2318_d_n13, eq42_e2318_d_n14, eq42_e2318_d_n15, eq42_e2318_d_n16, eq42_e2318_d_b0, eq42_e2318_d_b1, eq42_e2318_d_b2, eq42_e2318_d_b3, eq42_e2318_d_b4, eq42_e2318_d_b5, eq42_e2318_d_b6, eq42_e2318_d_b7, eq42_e2318_d_b8, eq42_e2318_d_b9, eq42_e2318_d_b10, eq42_e2318_d_b11, eq42_e2318_d_b12, eq42_e2318_d_b13, eq42_e2318_d_b14, eq42_e2318_d_b15, eq42_e2318_d_b16, eq42_e2318_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e2320;
        let eq42_node_derivatives: [f64; 17] = [eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16];
        let eq42_branch_derivatives: [f64; 18] = [eq42_e2320_d_b0, eq42_e2320_d_b1, eq42_e2320_d_b2, eq42_e2320_d_b3, eq42_e2320_d_b4, eq42_e2320_d_b5, eq42_e2320_d_b6, eq42_e2320_d_b7, eq42_e2320_d_b8, eq42_e2320_d_b9, eq42_e2320_d_b10, eq42_e2320_d_b11, eq42_e2320_d_b12, eq42_e2320_d_b13, eq42_e2320_d_b14, eq42_e2320_d_b15, eq42_e2320_d_b16, eq42_e2320_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            multiplicity * (eq42_value),
            nodes,
            &eq42_node_derivatives,
            branches,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16, eq43_e2328_d_b0, eq43_e2328_d_b1, eq43_e2328_d_b2, eq43_e2328_d_b3, eq43_e2328_d_b4, eq43_e2328_d_b5, eq43_e2328_d_b6, eq43_e2328_d_b7, eq43_e2328_d_b8, eq43_e2328_d_b9, eq43_e2328_d_b10, eq43_e2328_d_b11, eq43_e2328_d_b12, eq43_e2328_d_b13, eq43_e2328_d_b14, eq43_e2328_d_b15, eq43_e2328_d_b16, eq43_e2328_d_b17,) = {
    if (s.b[1705] && (!s.b[1706])) {
        let eq43_e2326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, s.v[506]);
        let eq43_e2326_d_n0: f64 = (s.dn[506][0] * ddt_scale);
        let eq43_e2326_d_n1: f64 = (s.dn[506][1] * ddt_scale);
        let eq43_e2326_d_n2: f64 = (s.dn[506][2] * ddt_scale);
        let eq43_e2326_d_n3: f64 = (s.dn[506][3] * ddt_scale);
        let eq43_e2326_d_n4: f64 = (s.dn[506][4] * ddt_scale);
        let eq43_e2326_d_n5: f64 = (s.dn[506][5] * ddt_scale);
        let eq43_e2326_d_n6: f64 = (s.dn[506][6] * ddt_scale);
        let eq43_e2326_d_n7: f64 = (s.dn[506][7] * ddt_scale);
        let eq43_e2326_d_n8: f64 = (s.dn[506][8] * ddt_scale);
        let eq43_e2326_d_n9: f64 = (s.dn[506][9] * ddt_scale);
        let eq43_e2326_d_n10: f64 = (s.dn[506][10] * ddt_scale);
        let eq43_e2326_d_n11: f64 = (s.dn[506][11] * ddt_scale);
        let eq43_e2326_d_n12: f64 = (s.dn[506][12] * ddt_scale);
        let eq43_e2326_d_n13: f64 = (s.dn[506][13] * ddt_scale);
        let eq43_e2326_d_n14: f64 = (s.dn[506][14] * ddt_scale);
        let eq43_e2326_d_n15: f64 = (s.dn[506][15] * ddt_scale);
        let eq43_e2326_d_n16: f64 = (s.dn[506][16] * ddt_scale);
        let eq43_e2326_d_b0: f64 = (s.db[506][0] * ddt_scale);
        let eq43_e2326_d_b1: f64 = (s.db[506][1] * ddt_scale);
        let eq43_e2326_d_b2: f64 = (s.db[506][2] * ddt_scale);
        let eq43_e2326_d_b3: f64 = (s.db[506][3] * ddt_scale);
        let eq43_e2326_d_b4: f64 = (s.db[506][4] * ddt_scale);
        let eq43_e2326_d_b5: f64 = (s.db[506][5] * ddt_scale);
        let eq43_e2326_d_b6: f64 = (s.db[506][6] * ddt_scale);
        let eq43_e2326_d_b7: f64 = (s.db[506][7] * ddt_scale);
        let eq43_e2326_d_b8: f64 = (s.db[506][8] * ddt_scale);
        let eq43_e2326_d_b9: f64 = (s.db[506][9] * ddt_scale);
        let eq43_e2326_d_b10: f64 = (s.db[506][10] * ddt_scale);
        let eq43_e2326_d_b11: f64 = (s.db[506][11] * ddt_scale);
        let eq43_e2326_d_b12: f64 = (s.db[506][12] * ddt_scale);
        let eq43_e2326_d_b13: f64 = (s.db[506][13] * ddt_scale);
        let eq43_e2326_d_b14: f64 = (s.db[506][14] * ddt_scale);
        let eq43_e2326_d_b15: f64 = (s.db[506][15] * ddt_scale);
        let eq43_e2326_d_b16: f64 = (s.db[506][16] * ddt_scale);
        let eq43_e2326_d_b17: f64 = (s.db[506][17] * ddt_scale);
        (eq43_e2326, eq43_e2326_d_n0, eq43_e2326_d_n1, eq43_e2326_d_n2, eq43_e2326_d_n3, eq43_e2326_d_n4, eq43_e2326_d_n5, eq43_e2326_d_n6, eq43_e2326_d_n7, eq43_e2326_d_n8, eq43_e2326_d_n9, eq43_e2326_d_n10, eq43_e2326_d_n11, eq43_e2326_d_n12, eq43_e2326_d_n13, eq43_e2326_d_n14, eq43_e2326_d_n15, eq43_e2326_d_n16, eq43_e2326_d_b0, eq43_e2326_d_b1, eq43_e2326_d_b2, eq43_e2326_d_b3, eq43_e2326_d_b4, eq43_e2326_d_b5, eq43_e2326_d_b6, eq43_e2326_d_b7, eq43_e2326_d_b8, eq43_e2326_d_b9, eq43_e2326_d_b10, eq43_e2326_d_b11, eq43_e2326_d_b12, eq43_e2326_d_b13, eq43_e2326_d_b14, eq43_e2326_d_b15, eq43_e2326_d_b16, eq43_e2326_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e2328;
        let eq43_node_derivatives: [f64; 17] = [eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16];
        let eq43_branch_derivatives: [f64; 18] = [eq43_e2328_d_b0, eq43_e2328_d_b1, eq43_e2328_d_b2, eq43_e2328_d_b3, eq43_e2328_d_b4, eq43_e2328_d_b5, eq43_e2328_d_b6, eq43_e2328_d_b7, eq43_e2328_d_b8, eq43_e2328_d_b9, eq43_e2328_d_b10, eq43_e2328_d_b11, eq43_e2328_d_b12, eq43_e2328_d_b13, eq43_e2328_d_b14, eq43_e2328_d_b15, eq43_e2328_d_b16, eq43_e2328_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            multiplicity * (eq43_value),
            nodes,
            &eq43_node_derivatives,
            branches,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16, eq44_e2333_d_b0, eq44_e2333_d_b1, eq44_e2333_d_b2, eq44_e2333_d_b3, eq44_e2333_d_b4, eq44_e2333_d_b5, eq44_e2333_d_b6, eq44_e2333_d_b7, eq44_e2333_d_b8, eq44_e2333_d_b9, eq44_e2333_d_b10, eq44_e2333_d_b11, eq44_e2333_d_b12, eq44_e2333_d_b13, eq44_e2333_d_b14, eq44_e2333_d_b15, eq44_e2333_d_b16, eq44_e2333_d_b17,) = {
    if s.b[1705] {
        let eq44_e2331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, s.v[502]);
        let eq44_e2331_d_n0: f64 = (s.dn[502][0] * ddt_scale);
        let eq44_e2331_d_n1: f64 = (s.dn[502][1] * ddt_scale);
        let eq44_e2331_d_n2: f64 = (s.dn[502][2] * ddt_scale);
        let eq44_e2331_d_n3: f64 = (s.dn[502][3] * ddt_scale);
        let eq44_e2331_d_n4: f64 = (s.dn[502][4] * ddt_scale);
        let eq44_e2331_d_n5: f64 = (s.dn[502][5] * ddt_scale);
        let eq44_e2331_d_n6: f64 = (s.dn[502][6] * ddt_scale);
        let eq44_e2331_d_n7: f64 = (s.dn[502][7] * ddt_scale);
        let eq44_e2331_d_n8: f64 = (s.dn[502][8] * ddt_scale);
        let eq44_e2331_d_n9: f64 = (s.dn[502][9] * ddt_scale);
        let eq44_e2331_d_n10: f64 = (s.dn[502][10] * ddt_scale);
        let eq44_e2331_d_n11: f64 = (s.dn[502][11] * ddt_scale);
        let eq44_e2331_d_n12: f64 = (s.dn[502][12] * ddt_scale);
        let eq44_e2331_d_n13: f64 = (s.dn[502][13] * ddt_scale);
        let eq44_e2331_d_n14: f64 = (s.dn[502][14] * ddt_scale);
        let eq44_e2331_d_n15: f64 = (s.dn[502][15] * ddt_scale);
        let eq44_e2331_d_n16: f64 = (s.dn[502][16] * ddt_scale);
        let eq44_e2331_d_b0: f64 = (s.db[502][0] * ddt_scale);
        let eq44_e2331_d_b1: f64 = (s.db[502][1] * ddt_scale);
        let eq44_e2331_d_b2: f64 = (s.db[502][2] * ddt_scale);
        let eq44_e2331_d_b3: f64 = (s.db[502][3] * ddt_scale);
        let eq44_e2331_d_b4: f64 = (s.db[502][4] * ddt_scale);
        let eq44_e2331_d_b5: f64 = (s.db[502][5] * ddt_scale);
        let eq44_e2331_d_b6: f64 = (s.db[502][6] * ddt_scale);
        let eq44_e2331_d_b7: f64 = (s.db[502][7] * ddt_scale);
        let eq44_e2331_d_b8: f64 = (s.db[502][8] * ddt_scale);
        let eq44_e2331_d_b9: f64 = (s.db[502][9] * ddt_scale);
        let eq44_e2331_d_b10: f64 = (s.db[502][10] * ddt_scale);
        let eq44_e2331_d_b11: f64 = (s.db[502][11] * ddt_scale);
        let eq44_e2331_d_b12: f64 = (s.db[502][12] * ddt_scale);
        let eq44_e2331_d_b13: f64 = (s.db[502][13] * ddt_scale);
        let eq44_e2331_d_b14: f64 = (s.db[502][14] * ddt_scale);
        let eq44_e2331_d_b15: f64 = (s.db[502][15] * ddt_scale);
        let eq44_e2331_d_b16: f64 = (s.db[502][16] * ddt_scale);
        let eq44_e2331_d_b17: f64 = (s.db[502][17] * ddt_scale);
        (eq44_e2331, eq44_e2331_d_n0, eq44_e2331_d_n1, eq44_e2331_d_n2, eq44_e2331_d_n3, eq44_e2331_d_n4, eq44_e2331_d_n5, eq44_e2331_d_n6, eq44_e2331_d_n7, eq44_e2331_d_n8, eq44_e2331_d_n9, eq44_e2331_d_n10, eq44_e2331_d_n11, eq44_e2331_d_n12, eq44_e2331_d_n13, eq44_e2331_d_n14, eq44_e2331_d_n15, eq44_e2331_d_n16, eq44_e2331_d_b0, eq44_e2331_d_b1, eq44_e2331_d_b2, eq44_e2331_d_b3, eq44_e2331_d_b4, eq44_e2331_d_b5, eq44_e2331_d_b6, eq44_e2331_d_b7, eq44_e2331_d_b8, eq44_e2331_d_b9, eq44_e2331_d_b10, eq44_e2331_d_b11, eq44_e2331_d_b12, eq44_e2331_d_b13, eq44_e2331_d_b14, eq44_e2331_d_b15, eq44_e2331_d_b16, eq44_e2331_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e2333;
        let eq44_node_derivatives: [f64; 17] = [eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16];
        let eq44_branch_derivatives: [f64; 18] = [eq44_e2333_d_b0, eq44_e2333_d_b1, eq44_e2333_d_b2, eq44_e2333_d_b3, eq44_e2333_d_b4, eq44_e2333_d_b5, eq44_e2333_d_b6, eq44_e2333_d_b7, eq44_e2333_d_b8, eq44_e2333_d_b9, eq44_e2333_d_b10, eq44_e2333_d_b11, eq44_e2333_d_b12, eq44_e2333_d_b13, eq44_e2333_d_b14, eq44_e2333_d_b15, eq44_e2333_d_b16, eq44_e2333_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            multiplicity * (eq44_value),
            nodes,
            &eq44_node_derivatives,
            branches,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16, eq45_e2340_d_b0, eq45_e2340_d_b1, eq45_e2340_d_b2, eq45_e2340_d_b3, eq45_e2340_d_b4, eq45_e2340_d_b5, eq45_e2340_d_b6, eq45_e2340_d_b7, eq45_e2340_d_b8, eq45_e2340_d_b9, eq45_e2340_d_b10, eq45_e2340_d_b11, eq45_e2340_d_b12, eq45_e2340_d_b13, eq45_e2340_d_b14, eq45_e2340_d_b15, eq45_e2340_d_b16, eq45_e2340_d_b17,) = {
    if (s.b[1705] && s.b[1707]) {
        let eq45_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, s.v[500]);
        let eq45_e2338_d_n0: f64 = (s.dn[500][0] * ddt_scale);
        let eq45_e2338_d_n1: f64 = (s.dn[500][1] * ddt_scale);
        let eq45_e2338_d_n2: f64 = (s.dn[500][2] * ddt_scale);
        let eq45_e2338_d_n3: f64 = (s.dn[500][3] * ddt_scale);
        let eq45_e2338_d_n4: f64 = (s.dn[500][4] * ddt_scale);
        let eq45_e2338_d_n5: f64 = (s.dn[500][5] * ddt_scale);
        let eq45_e2338_d_n6: f64 = (s.dn[500][6] * ddt_scale);
        let eq45_e2338_d_n7: f64 = (s.dn[500][7] * ddt_scale);
        let eq45_e2338_d_n8: f64 = (s.dn[500][8] * ddt_scale);
        let eq45_e2338_d_n9: f64 = (s.dn[500][9] * ddt_scale);
        let eq45_e2338_d_n10: f64 = (s.dn[500][10] * ddt_scale);
        let eq45_e2338_d_n11: f64 = (s.dn[500][11] * ddt_scale);
        let eq45_e2338_d_n12: f64 = (s.dn[500][12] * ddt_scale);
        let eq45_e2338_d_n13: f64 = (s.dn[500][13] * ddt_scale);
        let eq45_e2338_d_n14: f64 = (s.dn[500][14] * ddt_scale);
        let eq45_e2338_d_n15: f64 = (s.dn[500][15] * ddt_scale);
        let eq45_e2338_d_n16: f64 = (s.dn[500][16] * ddt_scale);
        let eq45_e2338_d_b0: f64 = (s.db[500][0] * ddt_scale);
        let eq45_e2338_d_b1: f64 = (s.db[500][1] * ddt_scale);
        let eq45_e2338_d_b2: f64 = (s.db[500][2] * ddt_scale);
        let eq45_e2338_d_b3: f64 = (s.db[500][3] * ddt_scale);
        let eq45_e2338_d_b4: f64 = (s.db[500][4] * ddt_scale);
        let eq45_e2338_d_b5: f64 = (s.db[500][5] * ddt_scale);
        let eq45_e2338_d_b6: f64 = (s.db[500][6] * ddt_scale);
        let eq45_e2338_d_b7: f64 = (s.db[500][7] * ddt_scale);
        let eq45_e2338_d_b8: f64 = (s.db[500][8] * ddt_scale);
        let eq45_e2338_d_b9: f64 = (s.db[500][9] * ddt_scale);
        let eq45_e2338_d_b10: f64 = (s.db[500][10] * ddt_scale);
        let eq45_e2338_d_b11: f64 = (s.db[500][11] * ddt_scale);
        let eq45_e2338_d_b12: f64 = (s.db[500][12] * ddt_scale);
        let eq45_e2338_d_b13: f64 = (s.db[500][13] * ddt_scale);
        let eq45_e2338_d_b14: f64 = (s.db[500][14] * ddt_scale);
        let eq45_e2338_d_b15: f64 = (s.db[500][15] * ddt_scale);
        let eq45_e2338_d_b16: f64 = (s.db[500][16] * ddt_scale);
        let eq45_e2338_d_b17: f64 = (s.db[500][17] * ddt_scale);
        (eq45_e2338, eq45_e2338_d_n0, eq45_e2338_d_n1, eq45_e2338_d_n2, eq45_e2338_d_n3, eq45_e2338_d_n4, eq45_e2338_d_n5, eq45_e2338_d_n6, eq45_e2338_d_n7, eq45_e2338_d_n8, eq45_e2338_d_n9, eq45_e2338_d_n10, eq45_e2338_d_n11, eq45_e2338_d_n12, eq45_e2338_d_n13, eq45_e2338_d_n14, eq45_e2338_d_n15, eq45_e2338_d_n16, eq45_e2338_d_b0, eq45_e2338_d_b1, eq45_e2338_d_b2, eq45_e2338_d_b3, eq45_e2338_d_b4, eq45_e2338_d_b5, eq45_e2338_d_b6, eq45_e2338_d_b7, eq45_e2338_d_b8, eq45_e2338_d_b9, eq45_e2338_d_b10, eq45_e2338_d_b11, eq45_e2338_d_b12, eq45_e2338_d_b13, eq45_e2338_d_b14, eq45_e2338_d_b15, eq45_e2338_d_b16, eq45_e2338_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e2340;
        let eq45_node_derivatives: [f64; 17] = [eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16];
        let eq45_branch_derivatives: [f64; 18] = [eq45_e2340_d_b0, eq45_e2340_d_b1, eq45_e2340_d_b2, eq45_e2340_d_b3, eq45_e2340_d_b4, eq45_e2340_d_b5, eq45_e2340_d_b6, eq45_e2340_d_b7, eq45_e2340_d_b8, eq45_e2340_d_b9, eq45_e2340_d_b10, eq45_e2340_d_b11, eq45_e2340_d_b12, eq45_e2340_d_b13, eq45_e2340_d_b14, eq45_e2340_d_b15, eq45_e2340_d_b16, eq45_e2340_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            multiplicity * (eq45_value),
            nodes,
            &eq45_node_derivatives,
            branches,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16, eq46_e2347_d_b0, eq46_e2347_d_b1, eq46_e2347_d_b2, eq46_e2347_d_b3, eq46_e2347_d_b4, eq46_e2347_d_b5, eq46_e2347_d_b6, eq46_e2347_d_b7, eq46_e2347_d_b8, eq46_e2347_d_b9, eq46_e2347_d_b10, eq46_e2347_d_b11, eq46_e2347_d_b12, eq46_e2347_d_b13, eq46_e2347_d_b14, eq46_e2347_d_b15, eq46_e2347_d_b16, eq46_e2347_d_b17,) = {
    if (s.b[1705] && s.b[1707]) {
        let eq46_e2345: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, s.v[501]);
        let eq46_e2345_d_n0: f64 = (s.dn[501][0] * ddt_scale);
        let eq46_e2345_d_n1: f64 = (s.dn[501][1] * ddt_scale);
        let eq46_e2345_d_n2: f64 = (s.dn[501][2] * ddt_scale);
        let eq46_e2345_d_n3: f64 = (s.dn[501][3] * ddt_scale);
        let eq46_e2345_d_n4: f64 = (s.dn[501][4] * ddt_scale);
        let eq46_e2345_d_n5: f64 = (s.dn[501][5] * ddt_scale);
        let eq46_e2345_d_n6: f64 = (s.dn[501][6] * ddt_scale);
        let eq46_e2345_d_n7: f64 = (s.dn[501][7] * ddt_scale);
        let eq46_e2345_d_n8: f64 = (s.dn[501][8] * ddt_scale);
        let eq46_e2345_d_n9: f64 = (s.dn[501][9] * ddt_scale);
        let eq46_e2345_d_n10: f64 = (s.dn[501][10] * ddt_scale);
        let eq46_e2345_d_n11: f64 = (s.dn[501][11] * ddt_scale);
        let eq46_e2345_d_n12: f64 = (s.dn[501][12] * ddt_scale);
        let eq46_e2345_d_n13: f64 = (s.dn[501][13] * ddt_scale);
        let eq46_e2345_d_n14: f64 = (s.dn[501][14] * ddt_scale);
        let eq46_e2345_d_n15: f64 = (s.dn[501][15] * ddt_scale);
        let eq46_e2345_d_n16: f64 = (s.dn[501][16] * ddt_scale);
        let eq46_e2345_d_b0: f64 = (s.db[501][0] * ddt_scale);
        let eq46_e2345_d_b1: f64 = (s.db[501][1] * ddt_scale);
        let eq46_e2345_d_b2: f64 = (s.db[501][2] * ddt_scale);
        let eq46_e2345_d_b3: f64 = (s.db[501][3] * ddt_scale);
        let eq46_e2345_d_b4: f64 = (s.db[501][4] * ddt_scale);
        let eq46_e2345_d_b5: f64 = (s.db[501][5] * ddt_scale);
        let eq46_e2345_d_b6: f64 = (s.db[501][6] * ddt_scale);
        let eq46_e2345_d_b7: f64 = (s.db[501][7] * ddt_scale);
        let eq46_e2345_d_b8: f64 = (s.db[501][8] * ddt_scale);
        let eq46_e2345_d_b9: f64 = (s.db[501][9] * ddt_scale);
        let eq46_e2345_d_b10: f64 = (s.db[501][10] * ddt_scale);
        let eq46_e2345_d_b11: f64 = (s.db[501][11] * ddt_scale);
        let eq46_e2345_d_b12: f64 = (s.db[501][12] * ddt_scale);
        let eq46_e2345_d_b13: f64 = (s.db[501][13] * ddt_scale);
        let eq46_e2345_d_b14: f64 = (s.db[501][14] * ddt_scale);
        let eq46_e2345_d_b15: f64 = (s.db[501][15] * ddt_scale);
        let eq46_e2345_d_b16: f64 = (s.db[501][16] * ddt_scale);
        let eq46_e2345_d_b17: f64 = (s.db[501][17] * ddt_scale);
        (eq46_e2345, eq46_e2345_d_n0, eq46_e2345_d_n1, eq46_e2345_d_n2, eq46_e2345_d_n3, eq46_e2345_d_n4, eq46_e2345_d_n5, eq46_e2345_d_n6, eq46_e2345_d_n7, eq46_e2345_d_n8, eq46_e2345_d_n9, eq46_e2345_d_n10, eq46_e2345_d_n11, eq46_e2345_d_n12, eq46_e2345_d_n13, eq46_e2345_d_n14, eq46_e2345_d_n15, eq46_e2345_d_n16, eq46_e2345_d_b0, eq46_e2345_d_b1, eq46_e2345_d_b2, eq46_e2345_d_b3, eq46_e2345_d_b4, eq46_e2345_d_b5, eq46_e2345_d_b6, eq46_e2345_d_b7, eq46_e2345_d_b8, eq46_e2345_d_b9, eq46_e2345_d_b10, eq46_e2345_d_b11, eq46_e2345_d_b12, eq46_e2345_d_b13, eq46_e2345_d_b14, eq46_e2345_d_b15, eq46_e2345_d_b16, eq46_e2345_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e2347;
        let eq46_node_derivatives: [f64; 17] = [eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16];
        let eq46_branch_derivatives: [f64; 18] = [eq46_e2347_d_b0, eq46_e2347_d_b1, eq46_e2347_d_b2, eq46_e2347_d_b3, eq46_e2347_d_b4, eq46_e2347_d_b5, eq46_e2347_d_b6, eq46_e2347_d_b7, eq46_e2347_d_b8, eq46_e2347_d_b9, eq46_e2347_d_b10, eq46_e2347_d_b11, eq46_e2347_d_b12, eq46_e2347_d_b13, eq46_e2347_d_b14, eq46_e2347_d_b15, eq46_e2347_d_b16, eq46_e2347_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            multiplicity * (eq46_value),
            nodes,
            &eq46_node_derivatives,
            branches,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16, eq47_e2353_d_b0, eq47_e2353_d_b1, eq47_e2353_d_b2, eq47_e2353_d_b3, eq47_e2353_d_b4, eq47_e2353_d_b5, eq47_e2353_d_b6, eq47_e2353_d_b7, eq47_e2353_d_b8, eq47_e2353_d_b9, eq47_e2353_d_b10, eq47_e2353_d_b11, eq47_e2353_d_b12, eq47_e2353_d_b13, eq47_e2353_d_b14, eq47_e2353_d_b15, eq47_e2353_d_b16, eq47_e2353_d_b17,) = {
    if (!s.b[1705]) {
        let eq47_e2351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, s.v[505]);
        let eq47_e2351_d_n0: f64 = (s.dn[505][0] * ddt_scale);
        let eq47_e2351_d_n1: f64 = (s.dn[505][1] * ddt_scale);
        let eq47_e2351_d_n2: f64 = (s.dn[505][2] * ddt_scale);
        let eq47_e2351_d_n3: f64 = (s.dn[505][3] * ddt_scale);
        let eq47_e2351_d_n4: f64 = (s.dn[505][4] * ddt_scale);
        let eq47_e2351_d_n5: f64 = (s.dn[505][5] * ddt_scale);
        let eq47_e2351_d_n6: f64 = (s.dn[505][6] * ddt_scale);
        let eq47_e2351_d_n7: f64 = (s.dn[505][7] * ddt_scale);
        let eq47_e2351_d_n8: f64 = (s.dn[505][8] * ddt_scale);
        let eq47_e2351_d_n9: f64 = (s.dn[505][9] * ddt_scale);
        let eq47_e2351_d_n10: f64 = (s.dn[505][10] * ddt_scale);
        let eq47_e2351_d_n11: f64 = (s.dn[505][11] * ddt_scale);
        let eq47_e2351_d_n12: f64 = (s.dn[505][12] * ddt_scale);
        let eq47_e2351_d_n13: f64 = (s.dn[505][13] * ddt_scale);
        let eq47_e2351_d_n14: f64 = (s.dn[505][14] * ddt_scale);
        let eq47_e2351_d_n15: f64 = (s.dn[505][15] * ddt_scale);
        let eq47_e2351_d_n16: f64 = (s.dn[505][16] * ddt_scale);
        let eq47_e2351_d_b0: f64 = (s.db[505][0] * ddt_scale);
        let eq47_e2351_d_b1: f64 = (s.db[505][1] * ddt_scale);
        let eq47_e2351_d_b2: f64 = (s.db[505][2] * ddt_scale);
        let eq47_e2351_d_b3: f64 = (s.db[505][3] * ddt_scale);
        let eq47_e2351_d_b4: f64 = (s.db[505][4] * ddt_scale);
        let eq47_e2351_d_b5: f64 = (s.db[505][5] * ddt_scale);
        let eq47_e2351_d_b6: f64 = (s.db[505][6] * ddt_scale);
        let eq47_e2351_d_b7: f64 = (s.db[505][7] * ddt_scale);
        let eq47_e2351_d_b8: f64 = (s.db[505][8] * ddt_scale);
        let eq47_e2351_d_b9: f64 = (s.db[505][9] * ddt_scale);
        let eq47_e2351_d_b10: f64 = (s.db[505][10] * ddt_scale);
        let eq47_e2351_d_b11: f64 = (s.db[505][11] * ddt_scale);
        let eq47_e2351_d_b12: f64 = (s.db[505][12] * ddt_scale);
        let eq47_e2351_d_b13: f64 = (s.db[505][13] * ddt_scale);
        let eq47_e2351_d_b14: f64 = (s.db[505][14] * ddt_scale);
        let eq47_e2351_d_b15: f64 = (s.db[505][15] * ddt_scale);
        let eq47_e2351_d_b16: f64 = (s.db[505][16] * ddt_scale);
        let eq47_e2351_d_b17: f64 = (s.db[505][17] * ddt_scale);
        (eq47_e2351, eq47_e2351_d_n0, eq47_e2351_d_n1, eq47_e2351_d_n2, eq47_e2351_d_n3, eq47_e2351_d_n4, eq47_e2351_d_n5, eq47_e2351_d_n6, eq47_e2351_d_n7, eq47_e2351_d_n8, eq47_e2351_d_n9, eq47_e2351_d_n10, eq47_e2351_d_n11, eq47_e2351_d_n12, eq47_e2351_d_n13, eq47_e2351_d_n14, eq47_e2351_d_n15, eq47_e2351_d_n16, eq47_e2351_d_b0, eq47_e2351_d_b1, eq47_e2351_d_b2, eq47_e2351_d_b3, eq47_e2351_d_b4, eq47_e2351_d_b5, eq47_e2351_d_b6, eq47_e2351_d_b7, eq47_e2351_d_b8, eq47_e2351_d_b9, eq47_e2351_d_b10, eq47_e2351_d_b11, eq47_e2351_d_b12, eq47_e2351_d_b13, eq47_e2351_d_b14, eq47_e2351_d_b15, eq47_e2351_d_b16, eq47_e2351_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e2353;
        let eq47_node_derivatives: [f64; 17] = [eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16];
        let eq47_branch_derivatives: [f64; 18] = [eq47_e2353_d_b0, eq47_e2353_d_b1, eq47_e2353_d_b2, eq47_e2353_d_b3, eq47_e2353_d_b4, eq47_e2353_d_b5, eq47_e2353_d_b6, eq47_e2353_d_b7, eq47_e2353_d_b8, eq47_e2353_d_b9, eq47_e2353_d_b10, eq47_e2353_d_b11, eq47_e2353_d_b12, eq47_e2353_d_b13, eq47_e2353_d_b14, eq47_e2353_d_b15, eq47_e2353_d_b16, eq47_e2353_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            multiplicity * (eq47_value),
            nodes,
            &eq47_node_derivatives,
            branches,
            &eq47_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16, eq48_e2361_d_b0, eq48_e2361_d_b1, eq48_e2361_d_b2, eq48_e2361_d_b3, eq48_e2361_d_b4, eq48_e2361_d_b5, eq48_e2361_d_b6, eq48_e2361_d_b7, eq48_e2361_d_b8, eq48_e2361_d_b9, eq48_e2361_d_b10, eq48_e2361_d_b11, eq48_e2361_d_b12, eq48_e2361_d_b13, eq48_e2361_d_b14, eq48_e2361_d_b15, eq48_e2361_d_b16, eq48_e2361_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq48_e2359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, s.v[506]);
        let eq48_e2359_d_n0: f64 = (s.dn[506][0] * ddt_scale);
        let eq48_e2359_d_n1: f64 = (s.dn[506][1] * ddt_scale);
        let eq48_e2359_d_n2: f64 = (s.dn[506][2] * ddt_scale);
        let eq48_e2359_d_n3: f64 = (s.dn[506][3] * ddt_scale);
        let eq48_e2359_d_n4: f64 = (s.dn[506][4] * ddt_scale);
        let eq48_e2359_d_n5: f64 = (s.dn[506][5] * ddt_scale);
        let eq48_e2359_d_n6: f64 = (s.dn[506][6] * ddt_scale);
        let eq48_e2359_d_n7: f64 = (s.dn[506][7] * ddt_scale);
        let eq48_e2359_d_n8: f64 = (s.dn[506][8] * ddt_scale);
        let eq48_e2359_d_n9: f64 = (s.dn[506][9] * ddt_scale);
        let eq48_e2359_d_n10: f64 = (s.dn[506][10] * ddt_scale);
        let eq48_e2359_d_n11: f64 = (s.dn[506][11] * ddt_scale);
        let eq48_e2359_d_n12: f64 = (s.dn[506][12] * ddt_scale);
        let eq48_e2359_d_n13: f64 = (s.dn[506][13] * ddt_scale);
        let eq48_e2359_d_n14: f64 = (s.dn[506][14] * ddt_scale);
        let eq48_e2359_d_n15: f64 = (s.dn[506][15] * ddt_scale);
        let eq48_e2359_d_n16: f64 = (s.dn[506][16] * ddt_scale);
        let eq48_e2359_d_b0: f64 = (s.db[506][0] * ddt_scale);
        let eq48_e2359_d_b1: f64 = (s.db[506][1] * ddt_scale);
        let eq48_e2359_d_b2: f64 = (s.db[506][2] * ddt_scale);
        let eq48_e2359_d_b3: f64 = (s.db[506][3] * ddt_scale);
        let eq48_e2359_d_b4: f64 = (s.db[506][4] * ddt_scale);
        let eq48_e2359_d_b5: f64 = (s.db[506][5] * ddt_scale);
        let eq48_e2359_d_b6: f64 = (s.db[506][6] * ddt_scale);
        let eq48_e2359_d_b7: f64 = (s.db[506][7] * ddt_scale);
        let eq48_e2359_d_b8: f64 = (s.db[506][8] * ddt_scale);
        let eq48_e2359_d_b9: f64 = (s.db[506][9] * ddt_scale);
        let eq48_e2359_d_b10: f64 = (s.db[506][10] * ddt_scale);
        let eq48_e2359_d_b11: f64 = (s.db[506][11] * ddt_scale);
        let eq48_e2359_d_b12: f64 = (s.db[506][12] * ddt_scale);
        let eq48_e2359_d_b13: f64 = (s.db[506][13] * ddt_scale);
        let eq48_e2359_d_b14: f64 = (s.db[506][14] * ddt_scale);
        let eq48_e2359_d_b15: f64 = (s.db[506][15] * ddt_scale);
        let eq48_e2359_d_b16: f64 = (s.db[506][16] * ddt_scale);
        let eq48_e2359_d_b17: f64 = (s.db[506][17] * ddt_scale);
        (eq48_e2359, eq48_e2359_d_n0, eq48_e2359_d_n1, eq48_e2359_d_n2, eq48_e2359_d_n3, eq48_e2359_d_n4, eq48_e2359_d_n5, eq48_e2359_d_n6, eq48_e2359_d_n7, eq48_e2359_d_n8, eq48_e2359_d_n9, eq48_e2359_d_n10, eq48_e2359_d_n11, eq48_e2359_d_n12, eq48_e2359_d_n13, eq48_e2359_d_n14, eq48_e2359_d_n15, eq48_e2359_d_n16, eq48_e2359_d_b0, eq48_e2359_d_b1, eq48_e2359_d_b2, eq48_e2359_d_b3, eq48_e2359_d_b4, eq48_e2359_d_b5, eq48_e2359_d_b6, eq48_e2359_d_b7, eq48_e2359_d_b8, eq48_e2359_d_b9, eq48_e2359_d_b10, eq48_e2359_d_b11, eq48_e2359_d_b12, eq48_e2359_d_b13, eq48_e2359_d_b14, eq48_e2359_d_b15, eq48_e2359_d_b16, eq48_e2359_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e2361;
        let eq48_node_derivatives: [f64; 17] = [eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16];
        let eq48_branch_derivatives: [f64; 18] = [eq48_e2361_d_b0, eq48_e2361_d_b1, eq48_e2361_d_b2, eq48_e2361_d_b3, eq48_e2361_d_b4, eq48_e2361_d_b5, eq48_e2361_d_b6, eq48_e2361_d_b7, eq48_e2361_d_b8, eq48_e2361_d_b9, eq48_e2361_d_b10, eq48_e2361_d_b11, eq48_e2361_d_b12, eq48_e2361_d_b13, eq48_e2361_d_b14, eq48_e2361_d_b15, eq48_e2361_d_b16, eq48_e2361_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            multiplicity * (eq48_value),
            nodes,
            &eq48_node_derivatives,
            branches,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16, eq49_e2371_d_b0, eq49_e2371_d_b1, eq49_e2371_d_b2, eq49_e2371_d_b3, eq49_e2371_d_b4, eq49_e2371_d_b5, eq49_e2371_d_b6, eq49_e2371_d_b7, eq49_e2371_d_b8, eq49_e2371_d_b9, eq49_e2371_d_b10, eq49_e2371_d_b11, eq49_e2371_d_b12, eq49_e2371_d_b13, eq49_e2371_d_b14, eq49_e2371_d_b15, eq49_e2371_d_b16, eq49_e2371_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq49_e2368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, s.v[503]);
        let eq49_e2368_d_n0: f64 = (s.dn[503][0] * ddt_scale);
        let eq49_e2368_d_n1: f64 = (s.dn[503][1] * ddt_scale);
        let eq49_e2368_d_n2: f64 = (s.dn[503][2] * ddt_scale);
        let eq49_e2368_d_n3: f64 = (s.dn[503][3] * ddt_scale);
        let eq49_e2368_d_n4: f64 = (s.dn[503][4] * ddt_scale);
        let eq49_e2368_d_n5: f64 = (s.dn[503][5] * ddt_scale);
        let eq49_e2368_d_n6: f64 = (s.dn[503][6] * ddt_scale);
        let eq49_e2368_d_n7: f64 = (s.dn[503][7] * ddt_scale);
        let eq49_e2368_d_n8: f64 = (s.dn[503][8] * ddt_scale);
        let eq49_e2368_d_n9: f64 = (s.dn[503][9] * ddt_scale);
        let eq49_e2368_d_n10: f64 = (s.dn[503][10] * ddt_scale);
        let eq49_e2368_d_n11: f64 = (s.dn[503][11] * ddt_scale);
        let eq49_e2368_d_n12: f64 = (s.dn[503][12] * ddt_scale);
        let eq49_e2368_d_n13: f64 = (s.dn[503][13] * ddt_scale);
        let eq49_e2368_d_n14: f64 = (s.dn[503][14] * ddt_scale);
        let eq49_e2368_d_n15: f64 = (s.dn[503][15] * ddt_scale);
        let eq49_e2368_d_n16: f64 = (s.dn[503][16] * ddt_scale);
        let eq49_e2368_d_b0: f64 = (s.db[503][0] * ddt_scale);
        let eq49_e2368_d_b1: f64 = (s.db[503][1] * ddt_scale);
        let eq49_e2368_d_b2: f64 = (s.db[503][2] * ddt_scale);
        let eq49_e2368_d_b3: f64 = (s.db[503][3] * ddt_scale);
        let eq49_e2368_d_b4: f64 = (s.db[503][4] * ddt_scale);
        let eq49_e2368_d_b5: f64 = (s.db[503][5] * ddt_scale);
        let eq49_e2368_d_b6: f64 = (s.db[503][6] * ddt_scale);
        let eq49_e2368_d_b7: f64 = (s.db[503][7] * ddt_scale);
        let eq49_e2368_d_b8: f64 = (s.db[503][8] * ddt_scale);
        let eq49_e2368_d_b9: f64 = (s.db[503][9] * ddt_scale);
        let eq49_e2368_d_b10: f64 = (s.db[503][10] * ddt_scale);
        let eq49_e2368_d_b11: f64 = (s.db[503][11] * ddt_scale);
        let eq49_e2368_d_b12: f64 = (s.db[503][12] * ddt_scale);
        let eq49_e2368_d_b13: f64 = (s.db[503][13] * ddt_scale);
        let eq49_e2368_d_b14: f64 = (s.db[503][14] * ddt_scale);
        let eq49_e2368_d_b15: f64 = (s.db[503][15] * ddt_scale);
        let eq49_e2368_d_b16: f64 = (s.db[503][16] * ddt_scale);
        let eq49_e2368_d_b17: f64 = (s.db[503][17] * ddt_scale);
        let eq49_e2369: f64 = (s.v[114] * eq49_e2368);
        let eq49_e2369_d_n0: f64 = ((s.dn[114][0] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n0));
        let eq49_e2369_d_n1: f64 = ((s.dn[114][1] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n1));
        let eq49_e2369_d_n2: f64 = ((s.dn[114][2] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n2));
        let eq49_e2369_d_n3: f64 = ((s.dn[114][3] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n3));
        let eq49_e2369_d_n4: f64 = ((s.dn[114][4] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n4));
        let eq49_e2369_d_n5: f64 = ((s.dn[114][5] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n5));
        let eq49_e2369_d_n6: f64 = ((s.dn[114][6] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n6));
        let eq49_e2369_d_n7: f64 = ((s.dn[114][7] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n7));
        let eq49_e2369_d_n8: f64 = ((s.dn[114][8] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n8));
        let eq49_e2369_d_n9: f64 = ((s.dn[114][9] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n9));
        let eq49_e2369_d_n10: f64 = ((s.dn[114][10] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n10));
        let eq49_e2369_d_n11: f64 = ((s.dn[114][11] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n11));
        let eq49_e2369_d_n12: f64 = ((s.dn[114][12] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n12));
        let eq49_e2369_d_n13: f64 = ((s.dn[114][13] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n13));
        let eq49_e2369_d_n14: f64 = ((s.dn[114][14] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n14));
        let eq49_e2369_d_n15: f64 = ((s.dn[114][15] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n15));
        let eq49_e2369_d_n16: f64 = ((s.dn[114][16] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n16));
        let eq49_e2369_d_b0: f64 = ((s.db[114][0] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b0));
        let eq49_e2369_d_b1: f64 = ((s.db[114][1] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b1));
        let eq49_e2369_d_b2: f64 = ((s.db[114][2] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b2));
        let eq49_e2369_d_b3: f64 = ((s.db[114][3] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b3));
        let eq49_e2369_d_b4: f64 = ((s.db[114][4] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b4));
        let eq49_e2369_d_b5: f64 = ((s.db[114][5] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b5));
        let eq49_e2369_d_b6: f64 = ((s.db[114][6] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b6));
        let eq49_e2369_d_b7: f64 = ((s.db[114][7] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b7));
        let eq49_e2369_d_b8: f64 = ((s.db[114][8] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b8));
        let eq49_e2369_d_b9: f64 = ((s.db[114][9] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b9));
        let eq49_e2369_d_b10: f64 = ((s.db[114][10] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b10));
        let eq49_e2369_d_b11: f64 = ((s.db[114][11] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b11));
        let eq49_e2369_d_b12: f64 = ((s.db[114][12] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b12));
        let eq49_e2369_d_b13: f64 = ((s.db[114][13] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b13));
        let eq49_e2369_d_b14: f64 = ((s.db[114][14] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b14));
        let eq49_e2369_d_b15: f64 = ((s.db[114][15] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b15));
        let eq49_e2369_d_b16: f64 = ((s.db[114][16] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b16));
        let eq49_e2369_d_b17: f64 = ((s.db[114][17] * eq49_e2368) + (s.v[114] * eq49_e2368_d_b17));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n1, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n12, eq49_e2369_d_n13, eq49_e2369_d_n14, eq49_e2369_d_n15, eq49_e2369_d_n16, eq49_e2369_d_b0, eq49_e2369_d_b1, eq49_e2369_d_b2, eq49_e2369_d_b3, eq49_e2369_d_b4, eq49_e2369_d_b5, eq49_e2369_d_b6, eq49_e2369_d_b7, eq49_e2369_d_b8, eq49_e2369_d_b9, eq49_e2369_d_b10, eq49_e2369_d_b11, eq49_e2369_d_b12, eq49_e2369_d_b13, eq49_e2369_d_b14, eq49_e2369_d_b15, eq49_e2369_d_b16, eq49_e2369_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e2371;
        let eq49_node_derivatives: [f64; 17] = [eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16];
        let eq49_branch_derivatives: [f64; 18] = [eq49_e2371_d_b0, eq49_e2371_d_b1, eq49_e2371_d_b2, eq49_e2371_d_b3, eq49_e2371_d_b4, eq49_e2371_d_b5, eq49_e2371_d_b6, eq49_e2371_d_b7, eq49_e2371_d_b8, eq49_e2371_d_b9, eq49_e2371_d_b10, eq49_e2371_d_b11, eq49_e2371_d_b12, eq49_e2371_d_b13, eq49_e2371_d_b14, eq49_e2371_d_b15, eq49_e2371_d_b16, eq49_e2371_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            multiplicity * (eq49_value),
            nodes,
            &eq49_node_derivatives,
            branches,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16, eq50_e2381_d_b0, eq50_e2381_d_b1, eq50_e2381_d_b2, eq50_e2381_d_b3, eq50_e2381_d_b4, eq50_e2381_d_b5, eq50_e2381_d_b6, eq50_e2381_d_b7, eq50_e2381_d_b8, eq50_e2381_d_b9, eq50_e2381_d_b10, eq50_e2381_d_b11, eq50_e2381_d_b12, eq50_e2381_d_b13, eq50_e2381_d_b14, eq50_e2381_d_b15, eq50_e2381_d_b16, eq50_e2381_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq50_e2378: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, s.v[504]);
        let eq50_e2378_d_n0: f64 = (s.dn[504][0] * ddt_scale);
        let eq50_e2378_d_n1: f64 = (s.dn[504][1] * ddt_scale);
        let eq50_e2378_d_n2: f64 = (s.dn[504][2] * ddt_scale);
        let eq50_e2378_d_n3: f64 = (s.dn[504][3] * ddt_scale);
        let eq50_e2378_d_n4: f64 = (s.dn[504][4] * ddt_scale);
        let eq50_e2378_d_n5: f64 = (s.dn[504][5] * ddt_scale);
        let eq50_e2378_d_n6: f64 = (s.dn[504][6] * ddt_scale);
        let eq50_e2378_d_n7: f64 = (s.dn[504][7] * ddt_scale);
        let eq50_e2378_d_n8: f64 = (s.dn[504][8] * ddt_scale);
        let eq50_e2378_d_n9: f64 = (s.dn[504][9] * ddt_scale);
        let eq50_e2378_d_n10: f64 = (s.dn[504][10] * ddt_scale);
        let eq50_e2378_d_n11: f64 = (s.dn[504][11] * ddt_scale);
        let eq50_e2378_d_n12: f64 = (s.dn[504][12] * ddt_scale);
        let eq50_e2378_d_n13: f64 = (s.dn[504][13] * ddt_scale);
        let eq50_e2378_d_n14: f64 = (s.dn[504][14] * ddt_scale);
        let eq50_e2378_d_n15: f64 = (s.dn[504][15] * ddt_scale);
        let eq50_e2378_d_n16: f64 = (s.dn[504][16] * ddt_scale);
        let eq50_e2378_d_b0: f64 = (s.db[504][0] * ddt_scale);
        let eq50_e2378_d_b1: f64 = (s.db[504][1] * ddt_scale);
        let eq50_e2378_d_b2: f64 = (s.db[504][2] * ddt_scale);
        let eq50_e2378_d_b3: f64 = (s.db[504][3] * ddt_scale);
        let eq50_e2378_d_b4: f64 = (s.db[504][4] * ddt_scale);
        let eq50_e2378_d_b5: f64 = (s.db[504][5] * ddt_scale);
        let eq50_e2378_d_b6: f64 = (s.db[504][6] * ddt_scale);
        let eq50_e2378_d_b7: f64 = (s.db[504][7] * ddt_scale);
        let eq50_e2378_d_b8: f64 = (s.db[504][8] * ddt_scale);
        let eq50_e2378_d_b9: f64 = (s.db[504][9] * ddt_scale);
        let eq50_e2378_d_b10: f64 = (s.db[504][10] * ddt_scale);
        let eq50_e2378_d_b11: f64 = (s.db[504][11] * ddt_scale);
        let eq50_e2378_d_b12: f64 = (s.db[504][12] * ddt_scale);
        let eq50_e2378_d_b13: f64 = (s.db[504][13] * ddt_scale);
        let eq50_e2378_d_b14: f64 = (s.db[504][14] * ddt_scale);
        let eq50_e2378_d_b15: f64 = (s.db[504][15] * ddt_scale);
        let eq50_e2378_d_b16: f64 = (s.db[504][16] * ddt_scale);
        let eq50_e2378_d_b17: f64 = (s.db[504][17] * ddt_scale);
        let eq50_e2379: f64 = (s.v[114] * eq50_e2378);
        let eq50_e2379_d_n0: f64 = ((s.dn[114][0] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n0));
        let eq50_e2379_d_n1: f64 = ((s.dn[114][1] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n1));
        let eq50_e2379_d_n2: f64 = ((s.dn[114][2] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n2));
        let eq50_e2379_d_n3: f64 = ((s.dn[114][3] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n3));
        let eq50_e2379_d_n4: f64 = ((s.dn[114][4] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n4));
        let eq50_e2379_d_n5: f64 = ((s.dn[114][5] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n5));
        let eq50_e2379_d_n6: f64 = ((s.dn[114][6] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n6));
        let eq50_e2379_d_n7: f64 = ((s.dn[114][7] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n7));
        let eq50_e2379_d_n8: f64 = ((s.dn[114][8] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n8));
        let eq50_e2379_d_n9: f64 = ((s.dn[114][9] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n9));
        let eq50_e2379_d_n10: f64 = ((s.dn[114][10] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n10));
        let eq50_e2379_d_n11: f64 = ((s.dn[114][11] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n11));
        let eq50_e2379_d_n12: f64 = ((s.dn[114][12] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n12));
        let eq50_e2379_d_n13: f64 = ((s.dn[114][13] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n13));
        let eq50_e2379_d_n14: f64 = ((s.dn[114][14] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n14));
        let eq50_e2379_d_n15: f64 = ((s.dn[114][15] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n15));
        let eq50_e2379_d_n16: f64 = ((s.dn[114][16] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n16));
        let eq50_e2379_d_b0: f64 = ((s.db[114][0] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b0));
        let eq50_e2379_d_b1: f64 = ((s.db[114][1] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b1));
        let eq50_e2379_d_b2: f64 = ((s.db[114][2] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b2));
        let eq50_e2379_d_b3: f64 = ((s.db[114][3] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b3));
        let eq50_e2379_d_b4: f64 = ((s.db[114][4] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b4));
        let eq50_e2379_d_b5: f64 = ((s.db[114][5] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b5));
        let eq50_e2379_d_b6: f64 = ((s.db[114][6] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b6));
        let eq50_e2379_d_b7: f64 = ((s.db[114][7] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b7));
        let eq50_e2379_d_b8: f64 = ((s.db[114][8] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b8));
        let eq50_e2379_d_b9: f64 = ((s.db[114][9] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b9));
        let eq50_e2379_d_b10: f64 = ((s.db[114][10] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b10));
        let eq50_e2379_d_b11: f64 = ((s.db[114][11] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b11));
        let eq50_e2379_d_b12: f64 = ((s.db[114][12] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b12));
        let eq50_e2379_d_b13: f64 = ((s.db[114][13] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b13));
        let eq50_e2379_d_b14: f64 = ((s.db[114][14] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b14));
        let eq50_e2379_d_b15: f64 = ((s.db[114][15] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b15));
        let eq50_e2379_d_b16: f64 = ((s.db[114][16] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b16));
        let eq50_e2379_d_b17: f64 = ((s.db[114][17] * eq50_e2378) + (s.v[114] * eq50_e2378_d_b17));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n1, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n12, eq50_e2379_d_n13, eq50_e2379_d_n14, eq50_e2379_d_n15, eq50_e2379_d_n16, eq50_e2379_d_b0, eq50_e2379_d_b1, eq50_e2379_d_b2, eq50_e2379_d_b3, eq50_e2379_d_b4, eq50_e2379_d_b5, eq50_e2379_d_b6, eq50_e2379_d_b7, eq50_e2379_d_b8, eq50_e2379_d_b9, eq50_e2379_d_b10, eq50_e2379_d_b11, eq50_e2379_d_b12, eq50_e2379_d_b13, eq50_e2379_d_b14, eq50_e2379_d_b15, eq50_e2379_d_b16, eq50_e2379_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e2381;
        let eq50_node_derivatives: [f64; 17] = [eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16];
        let eq50_branch_derivatives: [f64; 18] = [eq50_e2381_d_b0, eq50_e2381_d_b1, eq50_e2381_d_b2, eq50_e2381_d_b3, eq50_e2381_d_b4, eq50_e2381_d_b5, eq50_e2381_d_b6, eq50_e2381_d_b7, eq50_e2381_d_b8, eq50_e2381_d_b9, eq50_e2381_d_b10, eq50_e2381_d_b11, eq50_e2381_d_b12, eq50_e2381_d_b13, eq50_e2381_d_b14, eq50_e2381_d_b15, eq50_e2381_d_b16, eq50_e2381_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            multiplicity * (eq50_value),
            nodes,
            &eq50_node_derivatives,
            branches,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16, eq51_e2390_d_b0, eq51_e2390_d_b1, eq51_e2390_d_b2, eq51_e2390_d_b3, eq51_e2390_d_b4, eq51_e2390_d_b5, eq51_e2390_d_b6, eq51_e2390_d_b7, eq51_e2390_d_b8, eq51_e2390_d_b9, eq51_e2390_d_b10, eq51_e2390_d_b11, eq51_e2390_d_b12, eq51_e2390_d_b13, eq51_e2390_d_b14, eq51_e2390_d_b15, eq51_e2390_d_b16, eq51_e2390_d_b17,) = {
    if ((!s.b[1705]) && (!s.b[1708])) {
        let eq51_e2388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, s.v[506]);
        let eq51_e2388_d_n0: f64 = (s.dn[506][0] * ddt_scale);
        let eq51_e2388_d_n1: f64 = (s.dn[506][1] * ddt_scale);
        let eq51_e2388_d_n2: f64 = (s.dn[506][2] * ddt_scale);
        let eq51_e2388_d_n3: f64 = (s.dn[506][3] * ddt_scale);
        let eq51_e2388_d_n4: f64 = (s.dn[506][4] * ddt_scale);
        let eq51_e2388_d_n5: f64 = (s.dn[506][5] * ddt_scale);
        let eq51_e2388_d_n6: f64 = (s.dn[506][6] * ddt_scale);
        let eq51_e2388_d_n7: f64 = (s.dn[506][7] * ddt_scale);
        let eq51_e2388_d_n8: f64 = (s.dn[506][8] * ddt_scale);
        let eq51_e2388_d_n9: f64 = (s.dn[506][9] * ddt_scale);
        let eq51_e2388_d_n10: f64 = (s.dn[506][10] * ddt_scale);
        let eq51_e2388_d_n11: f64 = (s.dn[506][11] * ddt_scale);
        let eq51_e2388_d_n12: f64 = (s.dn[506][12] * ddt_scale);
        let eq51_e2388_d_n13: f64 = (s.dn[506][13] * ddt_scale);
        let eq51_e2388_d_n14: f64 = (s.dn[506][14] * ddt_scale);
        let eq51_e2388_d_n15: f64 = (s.dn[506][15] * ddt_scale);
        let eq51_e2388_d_n16: f64 = (s.dn[506][16] * ddt_scale);
        let eq51_e2388_d_b0: f64 = (s.db[506][0] * ddt_scale);
        let eq51_e2388_d_b1: f64 = (s.db[506][1] * ddt_scale);
        let eq51_e2388_d_b2: f64 = (s.db[506][2] * ddt_scale);
        let eq51_e2388_d_b3: f64 = (s.db[506][3] * ddt_scale);
        let eq51_e2388_d_b4: f64 = (s.db[506][4] * ddt_scale);
        let eq51_e2388_d_b5: f64 = (s.db[506][5] * ddt_scale);
        let eq51_e2388_d_b6: f64 = (s.db[506][6] * ddt_scale);
        let eq51_e2388_d_b7: f64 = (s.db[506][7] * ddt_scale);
        let eq51_e2388_d_b8: f64 = (s.db[506][8] * ddt_scale);
        let eq51_e2388_d_b9: f64 = (s.db[506][9] * ddt_scale);
        let eq51_e2388_d_b10: f64 = (s.db[506][10] * ddt_scale);
        let eq51_e2388_d_b11: f64 = (s.db[506][11] * ddt_scale);
        let eq51_e2388_d_b12: f64 = (s.db[506][12] * ddt_scale);
        let eq51_e2388_d_b13: f64 = (s.db[506][13] * ddt_scale);
        let eq51_e2388_d_b14: f64 = (s.db[506][14] * ddt_scale);
        let eq51_e2388_d_b15: f64 = (s.db[506][15] * ddt_scale);
        let eq51_e2388_d_b16: f64 = (s.db[506][16] * ddt_scale);
        let eq51_e2388_d_b17: f64 = (s.db[506][17] * ddt_scale);
        (eq51_e2388, eq51_e2388_d_n0, eq51_e2388_d_n1, eq51_e2388_d_n2, eq51_e2388_d_n3, eq51_e2388_d_n4, eq51_e2388_d_n5, eq51_e2388_d_n6, eq51_e2388_d_n7, eq51_e2388_d_n8, eq51_e2388_d_n9, eq51_e2388_d_n10, eq51_e2388_d_n11, eq51_e2388_d_n12, eq51_e2388_d_n13, eq51_e2388_d_n14, eq51_e2388_d_n15, eq51_e2388_d_n16, eq51_e2388_d_b0, eq51_e2388_d_b1, eq51_e2388_d_b2, eq51_e2388_d_b3, eq51_e2388_d_b4, eq51_e2388_d_b5, eq51_e2388_d_b6, eq51_e2388_d_b7, eq51_e2388_d_b8, eq51_e2388_d_b9, eq51_e2388_d_b10, eq51_e2388_d_b11, eq51_e2388_d_b12, eq51_e2388_d_b13, eq51_e2388_d_b14, eq51_e2388_d_b15, eq51_e2388_d_b16, eq51_e2388_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2390;
        let eq51_node_derivatives: [f64; 17] = [eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16];
        let eq51_branch_derivatives: [f64; 18] = [eq51_e2390_d_b0, eq51_e2390_d_b1, eq51_e2390_d_b2, eq51_e2390_d_b3, eq51_e2390_d_b4, eq51_e2390_d_b5, eq51_e2390_d_b6, eq51_e2390_d_b7, eq51_e2390_d_b8, eq51_e2390_d_b9, eq51_e2390_d_b10, eq51_e2390_d_b11, eq51_e2390_d_b12, eq51_e2390_d_b13, eq51_e2390_d_b14, eq51_e2390_d_b15, eq51_e2390_d_b16, eq51_e2390_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            multiplicity * (eq51_value),
            nodes,
            &eq51_node_derivatives,
            branches,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16, eq52_e2396_d_b0, eq52_e2396_d_b1, eq52_e2396_d_b2, eq52_e2396_d_b3, eq52_e2396_d_b4, eq52_e2396_d_b5, eq52_e2396_d_b6, eq52_e2396_d_b7, eq52_e2396_d_b8, eq52_e2396_d_b9, eq52_e2396_d_b10, eq52_e2396_d_b11, eq52_e2396_d_b12, eq52_e2396_d_b13, eq52_e2396_d_b14, eq52_e2396_d_b15, eq52_e2396_d_b16, eq52_e2396_d_b17,) = {
    if (!s.b[1705]) {
        let eq52_e2394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, s.v[502]);
        let eq52_e2394_d_n0: f64 = (s.dn[502][0] * ddt_scale);
        let eq52_e2394_d_n1: f64 = (s.dn[502][1] * ddt_scale);
        let eq52_e2394_d_n2: f64 = (s.dn[502][2] * ddt_scale);
        let eq52_e2394_d_n3: f64 = (s.dn[502][3] * ddt_scale);
        let eq52_e2394_d_n4: f64 = (s.dn[502][4] * ddt_scale);
        let eq52_e2394_d_n5: f64 = (s.dn[502][5] * ddt_scale);
        let eq52_e2394_d_n6: f64 = (s.dn[502][6] * ddt_scale);
        let eq52_e2394_d_n7: f64 = (s.dn[502][7] * ddt_scale);
        let eq52_e2394_d_n8: f64 = (s.dn[502][8] * ddt_scale);
        let eq52_e2394_d_n9: f64 = (s.dn[502][9] * ddt_scale);
        let eq52_e2394_d_n10: f64 = (s.dn[502][10] * ddt_scale);
        let eq52_e2394_d_n11: f64 = (s.dn[502][11] * ddt_scale);
        let eq52_e2394_d_n12: f64 = (s.dn[502][12] * ddt_scale);
        let eq52_e2394_d_n13: f64 = (s.dn[502][13] * ddt_scale);
        let eq52_e2394_d_n14: f64 = (s.dn[502][14] * ddt_scale);
        let eq52_e2394_d_n15: f64 = (s.dn[502][15] * ddt_scale);
        let eq52_e2394_d_n16: f64 = (s.dn[502][16] * ddt_scale);
        let eq52_e2394_d_b0: f64 = (s.db[502][0] * ddt_scale);
        let eq52_e2394_d_b1: f64 = (s.db[502][1] * ddt_scale);
        let eq52_e2394_d_b2: f64 = (s.db[502][2] * ddt_scale);
        let eq52_e2394_d_b3: f64 = (s.db[502][3] * ddt_scale);
        let eq52_e2394_d_b4: f64 = (s.db[502][4] * ddt_scale);
        let eq52_e2394_d_b5: f64 = (s.db[502][5] * ddt_scale);
        let eq52_e2394_d_b6: f64 = (s.db[502][6] * ddt_scale);
        let eq52_e2394_d_b7: f64 = (s.db[502][7] * ddt_scale);
        let eq52_e2394_d_b8: f64 = (s.db[502][8] * ddt_scale);
        let eq52_e2394_d_b9: f64 = (s.db[502][9] * ddt_scale);
        let eq52_e2394_d_b10: f64 = (s.db[502][10] * ddt_scale);
        let eq52_e2394_d_b11: f64 = (s.db[502][11] * ddt_scale);
        let eq52_e2394_d_b12: f64 = (s.db[502][12] * ddt_scale);
        let eq52_e2394_d_b13: f64 = (s.db[502][13] * ddt_scale);
        let eq52_e2394_d_b14: f64 = (s.db[502][14] * ddt_scale);
        let eq52_e2394_d_b15: f64 = (s.db[502][15] * ddt_scale);
        let eq52_e2394_d_b16: f64 = (s.db[502][16] * ddt_scale);
        let eq52_e2394_d_b17: f64 = (s.db[502][17] * ddt_scale);
        (eq52_e2394, eq52_e2394_d_n0, eq52_e2394_d_n1, eq52_e2394_d_n2, eq52_e2394_d_n3, eq52_e2394_d_n4, eq52_e2394_d_n5, eq52_e2394_d_n6, eq52_e2394_d_n7, eq52_e2394_d_n8, eq52_e2394_d_n9, eq52_e2394_d_n10, eq52_e2394_d_n11, eq52_e2394_d_n12, eq52_e2394_d_n13, eq52_e2394_d_n14, eq52_e2394_d_n15, eq52_e2394_d_n16, eq52_e2394_d_b0, eq52_e2394_d_b1, eq52_e2394_d_b2, eq52_e2394_d_b3, eq52_e2394_d_b4, eq52_e2394_d_b5, eq52_e2394_d_b6, eq52_e2394_d_b7, eq52_e2394_d_b8, eq52_e2394_d_b9, eq52_e2394_d_b10, eq52_e2394_d_b11, eq52_e2394_d_b12, eq52_e2394_d_b13, eq52_e2394_d_b14, eq52_e2394_d_b15, eq52_e2394_d_b16, eq52_e2394_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2396;
        let eq52_node_derivatives: [f64; 17] = [eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16];
        let eq52_branch_derivatives: [f64; 18] = [eq52_e2396_d_b0, eq52_e2396_d_b1, eq52_e2396_d_b2, eq52_e2396_d_b3, eq52_e2396_d_b4, eq52_e2396_d_b5, eq52_e2396_d_b6, eq52_e2396_d_b7, eq52_e2396_d_b8, eq52_e2396_d_b9, eq52_e2396_d_b10, eq52_e2396_d_b11, eq52_e2396_d_b12, eq52_e2396_d_b13, eq52_e2396_d_b14, eq52_e2396_d_b15, eq52_e2396_d_b16, eq52_e2396_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            multiplicity * (eq52_value),
            nodes,
            &eq52_node_derivatives,
            branches,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16, eq53_e2404_d_b0, eq53_e2404_d_b1, eq53_e2404_d_b2, eq53_e2404_d_b3, eq53_e2404_d_b4, eq53_e2404_d_b5, eq53_e2404_d_b6, eq53_e2404_d_b7, eq53_e2404_d_b8, eq53_e2404_d_b9, eq53_e2404_d_b10, eq53_e2404_d_b11, eq53_e2404_d_b12, eq53_e2404_d_b13, eq53_e2404_d_b14, eq53_e2404_d_b15, eq53_e2404_d_b16, eq53_e2404_d_b17,) = {
    if ((!s.b[1705]) && s.b[1709]) {
        let eq53_e2402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, s.v[500]);
        let eq53_e2402_d_n0: f64 = (s.dn[500][0] * ddt_scale);
        let eq53_e2402_d_n1: f64 = (s.dn[500][1] * ddt_scale);
        let eq53_e2402_d_n2: f64 = (s.dn[500][2] * ddt_scale);
        let eq53_e2402_d_n3: f64 = (s.dn[500][3] * ddt_scale);
        let eq53_e2402_d_n4: f64 = (s.dn[500][4] * ddt_scale);
        let eq53_e2402_d_n5: f64 = (s.dn[500][5] * ddt_scale);
        let eq53_e2402_d_n6: f64 = (s.dn[500][6] * ddt_scale);
        let eq53_e2402_d_n7: f64 = (s.dn[500][7] * ddt_scale);
        let eq53_e2402_d_n8: f64 = (s.dn[500][8] * ddt_scale);
        let eq53_e2402_d_n9: f64 = (s.dn[500][9] * ddt_scale);
        let eq53_e2402_d_n10: f64 = (s.dn[500][10] * ddt_scale);
        let eq53_e2402_d_n11: f64 = (s.dn[500][11] * ddt_scale);
        let eq53_e2402_d_n12: f64 = (s.dn[500][12] * ddt_scale);
        let eq53_e2402_d_n13: f64 = (s.dn[500][13] * ddt_scale);
        let eq53_e2402_d_n14: f64 = (s.dn[500][14] * ddt_scale);
        let eq53_e2402_d_n15: f64 = (s.dn[500][15] * ddt_scale);
        let eq53_e2402_d_n16: f64 = (s.dn[500][16] * ddt_scale);
        let eq53_e2402_d_b0: f64 = (s.db[500][0] * ddt_scale);
        let eq53_e2402_d_b1: f64 = (s.db[500][1] * ddt_scale);
        let eq53_e2402_d_b2: f64 = (s.db[500][2] * ddt_scale);
        let eq53_e2402_d_b3: f64 = (s.db[500][3] * ddt_scale);
        let eq53_e2402_d_b4: f64 = (s.db[500][4] * ddt_scale);
        let eq53_e2402_d_b5: f64 = (s.db[500][5] * ddt_scale);
        let eq53_e2402_d_b6: f64 = (s.db[500][6] * ddt_scale);
        let eq53_e2402_d_b7: f64 = (s.db[500][7] * ddt_scale);
        let eq53_e2402_d_b8: f64 = (s.db[500][8] * ddt_scale);
        let eq53_e2402_d_b9: f64 = (s.db[500][9] * ddt_scale);
        let eq53_e2402_d_b10: f64 = (s.db[500][10] * ddt_scale);
        let eq53_e2402_d_b11: f64 = (s.db[500][11] * ddt_scale);
        let eq53_e2402_d_b12: f64 = (s.db[500][12] * ddt_scale);
        let eq53_e2402_d_b13: f64 = (s.db[500][13] * ddt_scale);
        let eq53_e2402_d_b14: f64 = (s.db[500][14] * ddt_scale);
        let eq53_e2402_d_b15: f64 = (s.db[500][15] * ddt_scale);
        let eq53_e2402_d_b16: f64 = (s.db[500][16] * ddt_scale);
        let eq53_e2402_d_b17: f64 = (s.db[500][17] * ddt_scale);
        (eq53_e2402, eq53_e2402_d_n0, eq53_e2402_d_n1, eq53_e2402_d_n2, eq53_e2402_d_n3, eq53_e2402_d_n4, eq53_e2402_d_n5, eq53_e2402_d_n6, eq53_e2402_d_n7, eq53_e2402_d_n8, eq53_e2402_d_n9, eq53_e2402_d_n10, eq53_e2402_d_n11, eq53_e2402_d_n12, eq53_e2402_d_n13, eq53_e2402_d_n14, eq53_e2402_d_n15, eq53_e2402_d_n16, eq53_e2402_d_b0, eq53_e2402_d_b1, eq53_e2402_d_b2, eq53_e2402_d_b3, eq53_e2402_d_b4, eq53_e2402_d_b5, eq53_e2402_d_b6, eq53_e2402_d_b7, eq53_e2402_d_b8, eq53_e2402_d_b9, eq53_e2402_d_b10, eq53_e2402_d_b11, eq53_e2402_d_b12, eq53_e2402_d_b13, eq53_e2402_d_b14, eq53_e2402_d_b15, eq53_e2402_d_b16, eq53_e2402_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2404;
        let eq53_node_derivatives: [f64; 17] = [eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16];
        let eq53_branch_derivatives: [f64; 18] = [eq53_e2404_d_b0, eq53_e2404_d_b1, eq53_e2404_d_b2, eq53_e2404_d_b3, eq53_e2404_d_b4, eq53_e2404_d_b5, eq53_e2404_d_b6, eq53_e2404_d_b7, eq53_e2404_d_b8, eq53_e2404_d_b9, eq53_e2404_d_b10, eq53_e2404_d_b11, eq53_e2404_d_b12, eq53_e2404_d_b13, eq53_e2404_d_b14, eq53_e2404_d_b15, eq53_e2404_d_b16, eq53_e2404_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[2]),
            multiplicity * (eq53_value),
            nodes,
            &eq53_node_derivatives,
            branches,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16, eq54_e2412_d_b0, eq54_e2412_d_b1, eq54_e2412_d_b2, eq54_e2412_d_b3, eq54_e2412_d_b4, eq54_e2412_d_b5, eq54_e2412_d_b6, eq54_e2412_d_b7, eq54_e2412_d_b8, eq54_e2412_d_b9, eq54_e2412_d_b10, eq54_e2412_d_b11, eq54_e2412_d_b12, eq54_e2412_d_b13, eq54_e2412_d_b14, eq54_e2412_d_b15, eq54_e2412_d_b16, eq54_e2412_d_b17,) = {
    if ((!s.b[1705]) && s.b[1709]) {
        let eq54_e2410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, s.v[501]);
        let eq54_e2410_d_n0: f64 = (s.dn[501][0] * ddt_scale);
        let eq54_e2410_d_n1: f64 = (s.dn[501][1] * ddt_scale);
        let eq54_e2410_d_n2: f64 = (s.dn[501][2] * ddt_scale);
        let eq54_e2410_d_n3: f64 = (s.dn[501][3] * ddt_scale);
        let eq54_e2410_d_n4: f64 = (s.dn[501][4] * ddt_scale);
        let eq54_e2410_d_n5: f64 = (s.dn[501][5] * ddt_scale);
        let eq54_e2410_d_n6: f64 = (s.dn[501][6] * ddt_scale);
        let eq54_e2410_d_n7: f64 = (s.dn[501][7] * ddt_scale);
        let eq54_e2410_d_n8: f64 = (s.dn[501][8] * ddt_scale);
        let eq54_e2410_d_n9: f64 = (s.dn[501][9] * ddt_scale);
        let eq54_e2410_d_n10: f64 = (s.dn[501][10] * ddt_scale);
        let eq54_e2410_d_n11: f64 = (s.dn[501][11] * ddt_scale);
        let eq54_e2410_d_n12: f64 = (s.dn[501][12] * ddt_scale);
        let eq54_e2410_d_n13: f64 = (s.dn[501][13] * ddt_scale);
        let eq54_e2410_d_n14: f64 = (s.dn[501][14] * ddt_scale);
        let eq54_e2410_d_n15: f64 = (s.dn[501][15] * ddt_scale);
        let eq54_e2410_d_n16: f64 = (s.dn[501][16] * ddt_scale);
        let eq54_e2410_d_b0: f64 = (s.db[501][0] * ddt_scale);
        let eq54_e2410_d_b1: f64 = (s.db[501][1] * ddt_scale);
        let eq54_e2410_d_b2: f64 = (s.db[501][2] * ddt_scale);
        let eq54_e2410_d_b3: f64 = (s.db[501][3] * ddt_scale);
        let eq54_e2410_d_b4: f64 = (s.db[501][4] * ddt_scale);
        let eq54_e2410_d_b5: f64 = (s.db[501][5] * ddt_scale);
        let eq54_e2410_d_b6: f64 = (s.db[501][6] * ddt_scale);
        let eq54_e2410_d_b7: f64 = (s.db[501][7] * ddt_scale);
        let eq54_e2410_d_b8: f64 = (s.db[501][8] * ddt_scale);
        let eq54_e2410_d_b9: f64 = (s.db[501][9] * ddt_scale);
        let eq54_e2410_d_b10: f64 = (s.db[501][10] * ddt_scale);
        let eq54_e2410_d_b11: f64 = (s.db[501][11] * ddt_scale);
        let eq54_e2410_d_b12: f64 = (s.db[501][12] * ddt_scale);
        let eq54_e2410_d_b13: f64 = (s.db[501][13] * ddt_scale);
        let eq54_e2410_d_b14: f64 = (s.db[501][14] * ddt_scale);
        let eq54_e2410_d_b15: f64 = (s.db[501][15] * ddt_scale);
        let eq54_e2410_d_b16: f64 = (s.db[501][16] * ddt_scale);
        let eq54_e2410_d_b17: f64 = (s.db[501][17] * ddt_scale);
        (eq54_e2410, eq54_e2410_d_n0, eq54_e2410_d_n1, eq54_e2410_d_n2, eq54_e2410_d_n3, eq54_e2410_d_n4, eq54_e2410_d_n5, eq54_e2410_d_n6, eq54_e2410_d_n7, eq54_e2410_d_n8, eq54_e2410_d_n9, eq54_e2410_d_n10, eq54_e2410_d_n11, eq54_e2410_d_n12, eq54_e2410_d_n13, eq54_e2410_d_n14, eq54_e2410_d_n15, eq54_e2410_d_n16, eq54_e2410_d_b0, eq54_e2410_d_b1, eq54_e2410_d_b2, eq54_e2410_d_b3, eq54_e2410_d_b4, eq54_e2410_d_b5, eq54_e2410_d_b6, eq54_e2410_d_b7, eq54_e2410_d_b8, eq54_e2410_d_b9, eq54_e2410_d_b10, eq54_e2410_d_b11, eq54_e2410_d_b12, eq54_e2410_d_b13, eq54_e2410_d_b14, eq54_e2410_d_b15, eq54_e2410_d_b16, eq54_e2410_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2412;
        let eq54_node_derivatives: [f64; 17] = [eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16];
        let eq54_branch_derivatives: [f64; 18] = [eq54_e2412_d_b0, eq54_e2412_d_b1, eq54_e2412_d_b2, eq54_e2412_d_b3, eq54_e2412_d_b4, eq54_e2412_d_b5, eq54_e2412_d_b6, eq54_e2412_d_b7, eq54_e2412_d_b8, eq54_e2412_d_b9, eq54_e2412_d_b10, eq54_e2412_d_b11, eq54_e2412_d_b12, eq54_e2412_d_b13, eq54_e2412_d_b14, eq54_e2412_d_b15, eq54_e2412_d_b16, eq54_e2412_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            multiplicity * (eq54_value),
            nodes,
            &eq54_node_derivatives,
            branches,
            &eq54_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16, eq55_e2419_d_b0, eq55_e2419_d_b1, eq55_e2419_d_b2, eq55_e2419_d_b3, eq55_e2419_d_b4, eq55_e2419_d_b5, eq55_e2419_d_b6, eq55_e2419_d_b7, eq55_e2419_d_b8, eq55_e2419_d_b9, eq55_e2419_d_b10, eq55_e2419_d_b11, eq55_e2419_d_b12, eq55_e2419_d_b13, eq55_e2419_d_b14, eq55_e2419_d_b15, eq55_e2419_d_b16, eq55_e2419_d_b17,) = {
    if s.b[1710] {
        let eq55_e2416: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, s.v[495]);
        let eq55_e2416_d_n0: f64 = (s.dn[495][0] * ddt_scale);
        let eq55_e2416_d_n1: f64 = (s.dn[495][1] * ddt_scale);
        let eq55_e2416_d_n2: f64 = (s.dn[495][2] * ddt_scale);
        let eq55_e2416_d_n3: f64 = (s.dn[495][3] * ddt_scale);
        let eq55_e2416_d_n4: f64 = (s.dn[495][4] * ddt_scale);
        let eq55_e2416_d_n5: f64 = (s.dn[495][5] * ddt_scale);
        let eq55_e2416_d_n6: f64 = (s.dn[495][6] * ddt_scale);
        let eq55_e2416_d_n7: f64 = (s.dn[495][7] * ddt_scale);
        let eq55_e2416_d_n8: f64 = (s.dn[495][8] * ddt_scale);
        let eq55_e2416_d_n9: f64 = (s.dn[495][9] * ddt_scale);
        let eq55_e2416_d_n10: f64 = (s.dn[495][10] * ddt_scale);
        let eq55_e2416_d_n11: f64 = (s.dn[495][11] * ddt_scale);
        let eq55_e2416_d_n12: f64 = (s.dn[495][12] * ddt_scale);
        let eq55_e2416_d_n13: f64 = (s.dn[495][13] * ddt_scale);
        let eq55_e2416_d_n14: f64 = (s.dn[495][14] * ddt_scale);
        let eq55_e2416_d_n15: f64 = (s.dn[495][15] * ddt_scale);
        let eq55_e2416_d_n16: f64 = (s.dn[495][16] * ddt_scale);
        let eq55_e2416_d_b0: f64 = (s.db[495][0] * ddt_scale);
        let eq55_e2416_d_b1: f64 = (s.db[495][1] * ddt_scale);
        let eq55_e2416_d_b2: f64 = (s.db[495][2] * ddt_scale);
        let eq55_e2416_d_b3: f64 = (s.db[495][3] * ddt_scale);
        let eq55_e2416_d_b4: f64 = (s.db[495][4] * ddt_scale);
        let eq55_e2416_d_b5: f64 = (s.db[495][5] * ddt_scale);
        let eq55_e2416_d_b6: f64 = (s.db[495][6] * ddt_scale);
        let eq55_e2416_d_b7: f64 = (s.db[495][7] * ddt_scale);
        let eq55_e2416_d_b8: f64 = (s.db[495][8] * ddt_scale);
        let eq55_e2416_d_b9: f64 = (s.db[495][9] * ddt_scale);
        let eq55_e2416_d_b10: f64 = (s.db[495][10] * ddt_scale);
        let eq55_e2416_d_b11: f64 = (s.db[495][11] * ddt_scale);
        let eq55_e2416_d_b12: f64 = (s.db[495][12] * ddt_scale);
        let eq55_e2416_d_b13: f64 = (s.db[495][13] * ddt_scale);
        let eq55_e2416_d_b14: f64 = (s.db[495][14] * ddt_scale);
        let eq55_e2416_d_b15: f64 = (s.db[495][15] * ddt_scale);
        let eq55_e2416_d_b16: f64 = (s.db[495][16] * ddt_scale);
        let eq55_e2416_d_b17: f64 = (s.db[495][17] * ddt_scale);
        let eq55_e2417: f64 = (s.v[114] * eq55_e2416);
        let eq55_e2417_d_n0: f64 = ((s.dn[114][0] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n0));
        let eq55_e2417_d_n1: f64 = ((s.dn[114][1] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n1));
        let eq55_e2417_d_n2: f64 = ((s.dn[114][2] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n2));
        let eq55_e2417_d_n3: f64 = ((s.dn[114][3] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n3));
        let eq55_e2417_d_n4: f64 = ((s.dn[114][4] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n4));
        let eq55_e2417_d_n5: f64 = ((s.dn[114][5] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n5));
        let eq55_e2417_d_n6: f64 = ((s.dn[114][6] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n6));
        let eq55_e2417_d_n7: f64 = ((s.dn[114][7] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n7));
        let eq55_e2417_d_n8: f64 = ((s.dn[114][8] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n8));
        let eq55_e2417_d_n9: f64 = ((s.dn[114][9] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n9));
        let eq55_e2417_d_n10: f64 = ((s.dn[114][10] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n10));
        let eq55_e2417_d_n11: f64 = ((s.dn[114][11] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n11));
        let eq55_e2417_d_n12: f64 = ((s.dn[114][12] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n12));
        let eq55_e2417_d_n13: f64 = ((s.dn[114][13] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n13));
        let eq55_e2417_d_n14: f64 = ((s.dn[114][14] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n14));
        let eq55_e2417_d_n15: f64 = ((s.dn[114][15] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n15));
        let eq55_e2417_d_n16: f64 = ((s.dn[114][16] * eq55_e2416) + (s.v[114] * eq55_e2416_d_n16));
        let eq55_e2417_d_b0: f64 = ((s.db[114][0] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b0));
        let eq55_e2417_d_b1: f64 = ((s.db[114][1] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b1));
        let eq55_e2417_d_b2: f64 = ((s.db[114][2] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b2));
        let eq55_e2417_d_b3: f64 = ((s.db[114][3] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b3));
        let eq55_e2417_d_b4: f64 = ((s.db[114][4] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b4));
        let eq55_e2417_d_b5: f64 = ((s.db[114][5] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b5));
        let eq55_e2417_d_b6: f64 = ((s.db[114][6] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b6));
        let eq55_e2417_d_b7: f64 = ((s.db[114][7] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b7));
        let eq55_e2417_d_b8: f64 = ((s.db[114][8] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b8));
        let eq55_e2417_d_b9: f64 = ((s.db[114][9] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b9));
        let eq55_e2417_d_b10: f64 = ((s.db[114][10] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b10));
        let eq55_e2417_d_b11: f64 = ((s.db[114][11] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b11));
        let eq55_e2417_d_b12: f64 = ((s.db[114][12] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b12));
        let eq55_e2417_d_b13: f64 = ((s.db[114][13] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b13));
        let eq55_e2417_d_b14: f64 = ((s.db[114][14] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b14));
        let eq55_e2417_d_b15: f64 = ((s.db[114][15] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b15));
        let eq55_e2417_d_b16: f64 = ((s.db[114][16] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b16));
        let eq55_e2417_d_b17: f64 = ((s.db[114][17] * eq55_e2416) + (s.v[114] * eq55_e2416_d_b17));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n1, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n12, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_d_n15, eq55_e2417_d_n16, eq55_e2417_d_b0, eq55_e2417_d_b1, eq55_e2417_d_b2, eq55_e2417_d_b3, eq55_e2417_d_b4, eq55_e2417_d_b5, eq55_e2417_d_b6, eq55_e2417_d_b7, eq55_e2417_d_b8, eq55_e2417_d_b9, eq55_e2417_d_b10, eq55_e2417_d_b11, eq55_e2417_d_b12, eq55_e2417_d_b13, eq55_e2417_d_b14, eq55_e2417_d_b15, eq55_e2417_d_b16, eq55_e2417_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2419;
        let eq55_node_derivatives: [f64; 17] = [eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16];
        let eq55_branch_derivatives: [f64; 18] = [eq55_e2419_d_b0, eq55_e2419_d_b1, eq55_e2419_d_b2, eq55_e2419_d_b3, eq55_e2419_d_b4, eq55_e2419_d_b5, eq55_e2419_d_b6, eq55_e2419_d_b7, eq55_e2419_d_b8, eq55_e2419_d_b9, eq55_e2419_d_b10, eq55_e2419_d_b11, eq55_e2419_d_b12, eq55_e2419_d_b13, eq55_e2419_d_b14, eq55_e2419_d_b15, eq55_e2419_d_b16, eq55_e2419_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            multiplicity * (eq55_value),
            nodes,
            &eq55_node_derivatives,
            branches,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16, eq56_e2426_d_b0, eq56_e2426_d_b1, eq56_e2426_d_b2, eq56_e2426_d_b3, eq56_e2426_d_b4, eq56_e2426_d_b5, eq56_e2426_d_b6, eq56_e2426_d_b7, eq56_e2426_d_b8, eq56_e2426_d_b9, eq56_e2426_d_b10, eq56_e2426_d_b11, eq56_e2426_d_b12, eq56_e2426_d_b13, eq56_e2426_d_b14, eq56_e2426_d_b15, eq56_e2426_d_b16, eq56_e2426_d_b17,) = {
    if s.b[1710] {
        let eq56_e2423: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 22, s.v[496]);
        let eq56_e2423_d_n0: f64 = (s.dn[496][0] * ddt_scale);
        let eq56_e2423_d_n1: f64 = (s.dn[496][1] * ddt_scale);
        let eq56_e2423_d_n2: f64 = (s.dn[496][2] * ddt_scale);
        let eq56_e2423_d_n3: f64 = (s.dn[496][3] * ddt_scale);
        let eq56_e2423_d_n4: f64 = (s.dn[496][4] * ddt_scale);
        let eq56_e2423_d_n5: f64 = (s.dn[496][5] * ddt_scale);
        let eq56_e2423_d_n6: f64 = (s.dn[496][6] * ddt_scale);
        let eq56_e2423_d_n7: f64 = (s.dn[496][7] * ddt_scale);
        let eq56_e2423_d_n8: f64 = (s.dn[496][8] * ddt_scale);
        let eq56_e2423_d_n9: f64 = (s.dn[496][9] * ddt_scale);
        let eq56_e2423_d_n10: f64 = (s.dn[496][10] * ddt_scale);
        let eq56_e2423_d_n11: f64 = (s.dn[496][11] * ddt_scale);
        let eq56_e2423_d_n12: f64 = (s.dn[496][12] * ddt_scale);
        let eq56_e2423_d_n13: f64 = (s.dn[496][13] * ddt_scale);
        let eq56_e2423_d_n14: f64 = (s.dn[496][14] * ddt_scale);
        let eq56_e2423_d_n15: f64 = (s.dn[496][15] * ddt_scale);
        let eq56_e2423_d_n16: f64 = (s.dn[496][16] * ddt_scale);
        let eq56_e2423_d_b0: f64 = (s.db[496][0] * ddt_scale);
        let eq56_e2423_d_b1: f64 = (s.db[496][1] * ddt_scale);
        let eq56_e2423_d_b2: f64 = (s.db[496][2] * ddt_scale);
        let eq56_e2423_d_b3: f64 = (s.db[496][3] * ddt_scale);
        let eq56_e2423_d_b4: f64 = (s.db[496][4] * ddt_scale);
        let eq56_e2423_d_b5: f64 = (s.db[496][5] * ddt_scale);
        let eq56_e2423_d_b6: f64 = (s.db[496][6] * ddt_scale);
        let eq56_e2423_d_b7: f64 = (s.db[496][7] * ddt_scale);
        let eq56_e2423_d_b8: f64 = (s.db[496][8] * ddt_scale);
        let eq56_e2423_d_b9: f64 = (s.db[496][9] * ddt_scale);
        let eq56_e2423_d_b10: f64 = (s.db[496][10] * ddt_scale);
        let eq56_e2423_d_b11: f64 = (s.db[496][11] * ddt_scale);
        let eq56_e2423_d_b12: f64 = (s.db[496][12] * ddt_scale);
        let eq56_e2423_d_b13: f64 = (s.db[496][13] * ddt_scale);
        let eq56_e2423_d_b14: f64 = (s.db[496][14] * ddt_scale);
        let eq56_e2423_d_b15: f64 = (s.db[496][15] * ddt_scale);
        let eq56_e2423_d_b16: f64 = (s.db[496][16] * ddt_scale);
        let eq56_e2423_d_b17: f64 = (s.db[496][17] * ddt_scale);
        let eq56_e2424: f64 = (s.v[114] * eq56_e2423);
        let eq56_e2424_d_n0: f64 = ((s.dn[114][0] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n0));
        let eq56_e2424_d_n1: f64 = ((s.dn[114][1] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n1));
        let eq56_e2424_d_n2: f64 = ((s.dn[114][2] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n2));
        let eq56_e2424_d_n3: f64 = ((s.dn[114][3] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n3));
        let eq56_e2424_d_n4: f64 = ((s.dn[114][4] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n4));
        let eq56_e2424_d_n5: f64 = ((s.dn[114][5] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n5));
        let eq56_e2424_d_n6: f64 = ((s.dn[114][6] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n6));
        let eq56_e2424_d_n7: f64 = ((s.dn[114][7] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n7));
        let eq56_e2424_d_n8: f64 = ((s.dn[114][8] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n8));
        let eq56_e2424_d_n9: f64 = ((s.dn[114][9] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n9));
        let eq56_e2424_d_n10: f64 = ((s.dn[114][10] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n10));
        let eq56_e2424_d_n11: f64 = ((s.dn[114][11] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n11));
        let eq56_e2424_d_n12: f64 = ((s.dn[114][12] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n12));
        let eq56_e2424_d_n13: f64 = ((s.dn[114][13] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n13));
        let eq56_e2424_d_n14: f64 = ((s.dn[114][14] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n14));
        let eq56_e2424_d_n15: f64 = ((s.dn[114][15] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n15));
        let eq56_e2424_d_n16: f64 = ((s.dn[114][16] * eq56_e2423) + (s.v[114] * eq56_e2423_d_n16));
        let eq56_e2424_d_b0: f64 = ((s.db[114][0] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b0));
        let eq56_e2424_d_b1: f64 = ((s.db[114][1] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b1));
        let eq56_e2424_d_b2: f64 = ((s.db[114][2] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b2));
        let eq56_e2424_d_b3: f64 = ((s.db[114][3] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b3));
        let eq56_e2424_d_b4: f64 = ((s.db[114][4] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b4));
        let eq56_e2424_d_b5: f64 = ((s.db[114][5] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b5));
        let eq56_e2424_d_b6: f64 = ((s.db[114][6] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b6));
        let eq56_e2424_d_b7: f64 = ((s.db[114][7] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b7));
        let eq56_e2424_d_b8: f64 = ((s.db[114][8] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b8));
        let eq56_e2424_d_b9: f64 = ((s.db[114][9] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b9));
        let eq56_e2424_d_b10: f64 = ((s.db[114][10] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b10));
        let eq56_e2424_d_b11: f64 = ((s.db[114][11] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b11));
        let eq56_e2424_d_b12: f64 = ((s.db[114][12] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b12));
        let eq56_e2424_d_b13: f64 = ((s.db[114][13] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b13));
        let eq56_e2424_d_b14: f64 = ((s.db[114][14] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b14));
        let eq56_e2424_d_b15: f64 = ((s.db[114][15] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b15));
        let eq56_e2424_d_b16: f64 = ((s.db[114][16] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b16));
        let eq56_e2424_d_b17: f64 = ((s.db[114][17] * eq56_e2423) + (s.v[114] * eq56_e2423_d_b17));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n1, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n12, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_d_n15, eq56_e2424_d_n16, eq56_e2424_d_b0, eq56_e2424_d_b1, eq56_e2424_d_b2, eq56_e2424_d_b3, eq56_e2424_d_b4, eq56_e2424_d_b5, eq56_e2424_d_b6, eq56_e2424_d_b7, eq56_e2424_d_b8, eq56_e2424_d_b9, eq56_e2424_d_b10, eq56_e2424_d_b11, eq56_e2424_d_b12, eq56_e2424_d_b13, eq56_e2424_d_b14, eq56_e2424_d_b15, eq56_e2424_d_b16, eq56_e2424_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e2426;
        let eq56_node_derivatives: [f64; 17] = [eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16];
        let eq56_branch_derivatives: [f64; 18] = [eq56_e2426_d_b0, eq56_e2426_d_b1, eq56_e2426_d_b2, eq56_e2426_d_b3, eq56_e2426_d_b4, eq56_e2426_d_b5, eq56_e2426_d_b6, eq56_e2426_d_b7, eq56_e2426_d_b8, eq56_e2426_d_b9, eq56_e2426_d_b10, eq56_e2426_d_b11, eq56_e2426_d_b12, eq56_e2426_d_b13, eq56_e2426_d_b14, eq56_e2426_d_b15, eq56_e2426_d_b16, eq56_e2426_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            multiplicity * (eq56_value),
            nodes,
            &eq56_node_derivatives,
            branches,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e2432, eq57_e2432_d_n0, eq57_e2432_d_n1, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n12, eq57_e2432_d_n13, eq57_e2432_d_n14, eq57_e2432_d_n15, eq57_e2432_d_n16, eq57_e2432_d_b0, eq57_e2432_d_b1, eq57_e2432_d_b2, eq57_e2432_d_b3, eq57_e2432_d_b4, eq57_e2432_d_b5, eq57_e2432_d_b6, eq57_e2432_d_b7, eq57_e2432_d_b8, eq57_e2432_d_b9, eq57_e2432_d_b10, eq57_e2432_d_b11, eq57_e2432_d_b12, eq57_e2432_d_b13, eq57_e2432_d_b14, eq57_e2432_d_b15, eq57_e2432_d_b16, eq57_e2432_d_b17,) = {
    if s.b[1718] {
        let eq57_e2430: f64 = ((nv0 - nv9) * s.v[596]);
        let eq57_e2430_d_n0: f64 = (s.v[596] + ((nv0 - nv9) * s.dn[596][0]));
        let eq57_e2430_d_n1: f64 = ((nv0 - nv9) * s.dn[596][1]);
        let eq57_e2430_d_n2: f64 = ((nv0 - nv9) * s.dn[596][2]);
        let eq57_e2430_d_n3: f64 = ((nv0 - nv9) * s.dn[596][3]);
        let eq57_e2430_d_n4: f64 = ((nv0 - nv9) * s.dn[596][4]);
        let eq57_e2430_d_n5: f64 = ((nv0 - nv9) * s.dn[596][5]);
        let eq57_e2430_d_n6: f64 = ((nv0 - nv9) * s.dn[596][6]);
        let eq57_e2430_d_n7: f64 = ((nv0 - nv9) * s.dn[596][7]);
        let eq57_e2430_d_n8: f64 = ((nv0 - nv9) * s.dn[596][8]);
        let eq57_e2430_d_n9: f64 = ((-s.v[596]) + ((nv0 - nv9) * s.dn[596][9]));
        let eq57_e2430_d_n10: f64 = ((nv0 - nv9) * s.dn[596][10]);
        let eq57_e2430_d_n11: f64 = ((nv0 - nv9) * s.dn[596][11]);
        let eq57_e2430_d_n12: f64 = ((nv0 - nv9) * s.dn[596][12]);
        let eq57_e2430_d_n13: f64 = ((nv0 - nv9) * s.dn[596][13]);
        let eq57_e2430_d_n14: f64 = ((nv0 - nv9) * s.dn[596][14]);
        let eq57_e2430_d_n15: f64 = ((nv0 - nv9) * s.dn[596][15]);
        let eq57_e2430_d_n16: f64 = ((nv0 - nv9) * s.dn[596][16]);
        let eq57_e2430_d_b0: f64 = ((nv0 - nv9) * s.db[596][0]);
        let eq57_e2430_d_b1: f64 = ((nv0 - nv9) * s.db[596][1]);
        let eq57_e2430_d_b2: f64 = ((nv0 - nv9) * s.db[596][2]);
        let eq57_e2430_d_b3: f64 = ((nv0 - nv9) * s.db[596][3]);
        let eq57_e2430_d_b4: f64 = ((nv0 - nv9) * s.db[596][4]);
        let eq57_e2430_d_b5: f64 = ((nv0 - nv9) * s.db[596][5]);
        let eq57_e2430_d_b6: f64 = ((nv0 - nv9) * s.db[596][6]);
        let eq57_e2430_d_b7: f64 = ((nv0 - nv9) * s.db[596][7]);
        let eq57_e2430_d_b8: f64 = ((nv0 - nv9) * s.db[596][8]);
        let eq57_e2430_d_b9: f64 = ((nv0 - nv9) * s.db[596][9]);
        let eq57_e2430_d_b10: f64 = ((nv0 - nv9) * s.db[596][10]);
        let eq57_e2430_d_b11: f64 = ((nv0 - nv9) * s.db[596][11]);
        let eq57_e2430_d_b12: f64 = ((nv0 - nv9) * s.db[596][12]);
        let eq57_e2430_d_b13: f64 = ((nv0 - nv9) * s.db[596][13]);
        let eq57_e2430_d_b14: f64 = ((nv0 - nv9) * s.db[596][14]);
        let eq57_e2430_d_b15: f64 = ((nv0 - nv9) * s.db[596][15]);
        let eq57_e2430_d_b16: f64 = ((nv0 - nv9) * s.db[596][16]);
        let eq57_e2430_d_b17: f64 = ((nv0 - nv9) * s.db[596][17]);
        (eq57_e2430, eq57_e2430_d_n0, eq57_e2430_d_n1, eq57_e2430_d_n2, eq57_e2430_d_n3, eq57_e2430_d_n4, eq57_e2430_d_n5, eq57_e2430_d_n6, eq57_e2430_d_n7, eq57_e2430_d_n8, eq57_e2430_d_n9, eq57_e2430_d_n10, eq57_e2430_d_n11, eq57_e2430_d_n12, eq57_e2430_d_n13, eq57_e2430_d_n14, eq57_e2430_d_n15, eq57_e2430_d_n16, eq57_e2430_d_b0, eq57_e2430_d_b1, eq57_e2430_d_b2, eq57_e2430_d_b3, eq57_e2430_d_b4, eq57_e2430_d_b5, eq57_e2430_d_b6, eq57_e2430_d_b7, eq57_e2430_d_b8, eq57_e2430_d_b9, eq57_e2430_d_b10, eq57_e2430_d_b11, eq57_e2430_d_b12, eq57_e2430_d_b13, eq57_e2430_d_b14, eq57_e2430_d_b15, eq57_e2430_d_b16, eq57_e2430_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2432;
        let eq57_node_derivatives: [f64; 17] = [eq57_e2432_d_n0, eq57_e2432_d_n1, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n12, eq57_e2432_d_n13, eq57_e2432_d_n14, eq57_e2432_d_n15, eq57_e2432_d_n16];
        let eq57_branch_derivatives: [f64; 18] = [eq57_e2432_d_b0, eq57_e2432_d_b1, eq57_e2432_d_b2, eq57_e2432_d_b3, eq57_e2432_d_b4, eq57_e2432_d_b5, eq57_e2432_d_b6, eq57_e2432_d_b7, eq57_e2432_d_b8, eq57_e2432_d_b9, eq57_e2432_d_b10, eq57_e2432_d_b11, eq57_e2432_d_b12, eq57_e2432_d_b13, eq57_e2432_d_b14, eq57_e2432_d_b15, eq57_e2432_d_b16, eq57_e2432_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[9]),
            multiplicity * (eq57_value),
            nodes,
            &eq57_node_derivatives,
            branches,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e2440, eq58_e2440_d_n0, eq58_e2440_d_n1, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n12, eq58_e2440_d_n13, eq58_e2440_d_n14, eq58_e2440_d_n15, eq58_e2440_d_n16, eq58_e2440_d_b0, eq58_e2440_d_b1, eq58_e2440_d_b2, eq58_e2440_d_b3, eq58_e2440_d_b4, eq58_e2440_d_b5, eq58_e2440_d_b6, eq58_e2440_d_b7, eq58_e2440_d_b8, eq58_e2440_d_b9, eq58_e2440_d_b10, eq58_e2440_d_b11, eq58_e2440_d_b12, eq58_e2440_d_b13, eq58_e2440_d_b14, eq58_e2440_d_b15, eq58_e2440_d_b16, eq58_e2440_d_b17,) = {
    if (s.b[1718] && s.b[1719]) {
        let eq58_e2438: f64 = ((nv9 - nv7) * s.v[1042]);
        let eq58_e2438_d_n0: f64 = ((nv9 - nv7) * s.dn[1042][0]);
        let eq58_e2438_d_n1: f64 = ((nv9 - nv7) * s.dn[1042][1]);
        let eq58_e2438_d_n2: f64 = ((nv9 - nv7) * s.dn[1042][2]);
        let eq58_e2438_d_n3: f64 = ((nv9 - nv7) * s.dn[1042][3]);
        let eq58_e2438_d_n4: f64 = ((nv9 - nv7) * s.dn[1042][4]);
        let eq58_e2438_d_n5: f64 = ((nv9 - nv7) * s.dn[1042][5]);
        let eq58_e2438_d_n6: f64 = ((nv9 - nv7) * s.dn[1042][6]);
        let eq58_e2438_d_n7: f64 = ((-s.v[1042]) + ((nv9 - nv7) * s.dn[1042][7]));
        let eq58_e2438_d_n8: f64 = ((nv9 - nv7) * s.dn[1042][8]);
        let eq58_e2438_d_n9: f64 = (s.v[1042] + ((nv9 - nv7) * s.dn[1042][9]));
        let eq58_e2438_d_n10: f64 = ((nv9 - nv7) * s.dn[1042][10]);
        let eq58_e2438_d_n11: f64 = ((nv9 - nv7) * s.dn[1042][11]);
        let eq58_e2438_d_n12: f64 = ((nv9 - nv7) * s.dn[1042][12]);
        let eq58_e2438_d_n13: f64 = ((nv9 - nv7) * s.dn[1042][13]);
        let eq58_e2438_d_n14: f64 = ((nv9 - nv7) * s.dn[1042][14]);
        let eq58_e2438_d_n15: f64 = ((nv9 - nv7) * s.dn[1042][15]);
        let eq58_e2438_d_n16: f64 = ((nv9 - nv7) * s.dn[1042][16]);
        let eq58_e2438_d_b0: f64 = ((nv9 - nv7) * s.db[1042][0]);
        let eq58_e2438_d_b1: f64 = ((nv9 - nv7) * s.db[1042][1]);
        let eq58_e2438_d_b2: f64 = ((nv9 - nv7) * s.db[1042][2]);
        let eq58_e2438_d_b3: f64 = ((nv9 - nv7) * s.db[1042][3]);
        let eq58_e2438_d_b4: f64 = ((nv9 - nv7) * s.db[1042][4]);
        let eq58_e2438_d_b5: f64 = ((nv9 - nv7) * s.db[1042][5]);
        let eq58_e2438_d_b6: f64 = ((nv9 - nv7) * s.db[1042][6]);
        let eq58_e2438_d_b7: f64 = ((nv9 - nv7) * s.db[1042][7]);
        let eq58_e2438_d_b8: f64 = ((nv9 - nv7) * s.db[1042][8]);
        let eq58_e2438_d_b9: f64 = ((nv9 - nv7) * s.db[1042][9]);
        let eq58_e2438_d_b10: f64 = ((nv9 - nv7) * s.db[1042][10]);
        let eq58_e2438_d_b11: f64 = ((nv9 - nv7) * s.db[1042][11]);
        let eq58_e2438_d_b12: f64 = ((nv9 - nv7) * s.db[1042][12]);
        let eq58_e2438_d_b13: f64 = ((nv9 - nv7) * s.db[1042][13]);
        let eq58_e2438_d_b14: f64 = ((nv9 - nv7) * s.db[1042][14]);
        let eq58_e2438_d_b15: f64 = ((nv9 - nv7) * s.db[1042][15]);
        let eq58_e2438_d_b16: f64 = ((nv9 - nv7) * s.db[1042][16]);
        let eq58_e2438_d_b17: f64 = ((nv9 - nv7) * s.db[1042][17]);
        (eq58_e2438, eq58_e2438_d_n0, eq58_e2438_d_n1, eq58_e2438_d_n2, eq58_e2438_d_n3, eq58_e2438_d_n4, eq58_e2438_d_n5, eq58_e2438_d_n6, eq58_e2438_d_n7, eq58_e2438_d_n8, eq58_e2438_d_n9, eq58_e2438_d_n10, eq58_e2438_d_n11, eq58_e2438_d_n12, eq58_e2438_d_n13, eq58_e2438_d_n14, eq58_e2438_d_n15, eq58_e2438_d_n16, eq58_e2438_d_b0, eq58_e2438_d_b1, eq58_e2438_d_b2, eq58_e2438_d_b3, eq58_e2438_d_b4, eq58_e2438_d_b5, eq58_e2438_d_b6, eq58_e2438_d_b7, eq58_e2438_d_b8, eq58_e2438_d_b9, eq58_e2438_d_b10, eq58_e2438_d_b11, eq58_e2438_d_b12, eq58_e2438_d_b13, eq58_e2438_d_b14, eq58_e2438_d_b15, eq58_e2438_d_b16, eq58_e2438_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e2440;
        let eq58_node_derivatives: [f64; 17] = [eq58_e2440_d_n0, eq58_e2440_d_n1, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n12, eq58_e2440_d_n13, eq58_e2440_d_n14, eq58_e2440_d_n15, eq58_e2440_d_n16];
        let eq58_branch_derivatives: [f64; 18] = [eq58_e2440_d_b0, eq58_e2440_d_b1, eq58_e2440_d_b2, eq58_e2440_d_b3, eq58_e2440_d_b4, eq58_e2440_d_b5, eq58_e2440_d_b6, eq58_e2440_d_b7, eq58_e2440_d_b8, eq58_e2440_d_b9, eq58_e2440_d_b10, eq58_e2440_d_b11, eq58_e2440_d_b12, eq58_e2440_d_b13, eq58_e2440_d_b14, eq58_e2440_d_b15, eq58_e2440_d_b16, eq58_e2440_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq58_value),
            nodes,
            &eq58_node_derivatives,
            branches,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e2447,) = {
    if (s.b[1718] && (!s.b[1719])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e2447;
        stamper.stamp_potential_const(
            branches[1],
            eq59_value,
        );
        let (eq60_e2452,) = {
    if (!s.b[1718]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2452;
        stamper.stamp_potential_const(
            branches[2],
            eq60_value,
        );
        let (eq61_e2457,) = {
    if (!s.b[1718]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e2457;
        stamper.stamp_potential_const(
            branches[3],
            eq61_value,
        );
        let (eq62_e2463, eq62_e2463_d_n0, eq62_e2463_d_n1, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n12, eq62_e2463_d_n13, eq62_e2463_d_n14, eq62_e2463_d_n15, eq62_e2463_d_n16, eq62_e2463_d_b0, eq62_e2463_d_b1, eq62_e2463_d_b2, eq62_e2463_d_b3, eq62_e2463_d_b4, eq62_e2463_d_b5, eq62_e2463_d_b6, eq62_e2463_d_b7, eq62_e2463_d_b8, eq62_e2463_d_b9, eq62_e2463_d_b10, eq62_e2463_d_b11, eq62_e2463_d_b12, eq62_e2463_d_b13, eq62_e2463_d_b14, eq62_e2463_d_b15, eq62_e2463_d_b16, eq62_e2463_d_b17,) = {
    if s.b[1720] {
        let eq62_e2461: f64 = ((nv2 - nv8) * s.v[595]);
        let eq62_e2461_d_n0: f64 = ((nv2 - nv8) * s.dn[595][0]);
        let eq62_e2461_d_n1: f64 = ((nv2 - nv8) * s.dn[595][1]);
        let eq62_e2461_d_n2: f64 = (s.v[595] + ((nv2 - nv8) * s.dn[595][2]));
        let eq62_e2461_d_n3: f64 = ((nv2 - nv8) * s.dn[595][3]);
        let eq62_e2461_d_n4: f64 = ((nv2 - nv8) * s.dn[595][4]);
        let eq62_e2461_d_n5: f64 = ((nv2 - nv8) * s.dn[595][5]);
        let eq62_e2461_d_n6: f64 = ((nv2 - nv8) * s.dn[595][6]);
        let eq62_e2461_d_n7: f64 = ((nv2 - nv8) * s.dn[595][7]);
        let eq62_e2461_d_n8: f64 = ((-s.v[595]) + ((nv2 - nv8) * s.dn[595][8]));
        let eq62_e2461_d_n9: f64 = ((nv2 - nv8) * s.dn[595][9]);
        let eq62_e2461_d_n10: f64 = ((nv2 - nv8) * s.dn[595][10]);
        let eq62_e2461_d_n11: f64 = ((nv2 - nv8) * s.dn[595][11]);
        let eq62_e2461_d_n12: f64 = ((nv2 - nv8) * s.dn[595][12]);
        let eq62_e2461_d_n13: f64 = ((nv2 - nv8) * s.dn[595][13]);
        let eq62_e2461_d_n14: f64 = ((nv2 - nv8) * s.dn[595][14]);
        let eq62_e2461_d_n15: f64 = ((nv2 - nv8) * s.dn[595][15]);
        let eq62_e2461_d_n16: f64 = ((nv2 - nv8) * s.dn[595][16]);
        let eq62_e2461_d_b0: f64 = ((nv2 - nv8) * s.db[595][0]);
        let eq62_e2461_d_b1: f64 = ((nv2 - nv8) * s.db[595][1]);
        let eq62_e2461_d_b2: f64 = ((nv2 - nv8) * s.db[595][2]);
        let eq62_e2461_d_b3: f64 = ((nv2 - nv8) * s.db[595][3]);
        let eq62_e2461_d_b4: f64 = ((nv2 - nv8) * s.db[595][4]);
        let eq62_e2461_d_b5: f64 = ((nv2 - nv8) * s.db[595][5]);
        let eq62_e2461_d_b6: f64 = ((nv2 - nv8) * s.db[595][6]);
        let eq62_e2461_d_b7: f64 = ((nv2 - nv8) * s.db[595][7]);
        let eq62_e2461_d_b8: f64 = ((nv2 - nv8) * s.db[595][8]);
        let eq62_e2461_d_b9: f64 = ((nv2 - nv8) * s.db[595][9]);
        let eq62_e2461_d_b10: f64 = ((nv2 - nv8) * s.db[595][10]);
        let eq62_e2461_d_b11: f64 = ((nv2 - nv8) * s.db[595][11]);
        let eq62_e2461_d_b12: f64 = ((nv2 - nv8) * s.db[595][12]);
        let eq62_e2461_d_b13: f64 = ((nv2 - nv8) * s.db[595][13]);
        let eq62_e2461_d_b14: f64 = ((nv2 - nv8) * s.db[595][14]);
        let eq62_e2461_d_b15: f64 = ((nv2 - nv8) * s.db[595][15]);
        let eq62_e2461_d_b16: f64 = ((nv2 - nv8) * s.db[595][16]);
        let eq62_e2461_d_b17: f64 = ((nv2 - nv8) * s.db[595][17]);
        (eq62_e2461, eq62_e2461_d_n0, eq62_e2461_d_n1, eq62_e2461_d_n2, eq62_e2461_d_n3, eq62_e2461_d_n4, eq62_e2461_d_n5, eq62_e2461_d_n6, eq62_e2461_d_n7, eq62_e2461_d_n8, eq62_e2461_d_n9, eq62_e2461_d_n10, eq62_e2461_d_n11, eq62_e2461_d_n12, eq62_e2461_d_n13, eq62_e2461_d_n14, eq62_e2461_d_n15, eq62_e2461_d_n16, eq62_e2461_d_b0, eq62_e2461_d_b1, eq62_e2461_d_b2, eq62_e2461_d_b3, eq62_e2461_d_b4, eq62_e2461_d_b5, eq62_e2461_d_b6, eq62_e2461_d_b7, eq62_e2461_d_b8, eq62_e2461_d_b9, eq62_e2461_d_b10, eq62_e2461_d_b11, eq62_e2461_d_b12, eq62_e2461_d_b13, eq62_e2461_d_b14, eq62_e2461_d_b15, eq62_e2461_d_b16, eq62_e2461_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2463;
        let eq62_node_derivatives: [f64; 17] = [eq62_e2463_d_n0, eq62_e2463_d_n1, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n12, eq62_e2463_d_n13, eq62_e2463_d_n14, eq62_e2463_d_n15, eq62_e2463_d_n16];
        let eq62_branch_derivatives: [f64; 18] = [eq62_e2463_d_b0, eq62_e2463_d_b1, eq62_e2463_d_b2, eq62_e2463_d_b3, eq62_e2463_d_b4, eq62_e2463_d_b5, eq62_e2463_d_b6, eq62_e2463_d_b7, eq62_e2463_d_b8, eq62_e2463_d_b9, eq62_e2463_d_b10, eq62_e2463_d_b11, eq62_e2463_d_b12, eq62_e2463_d_b13, eq62_e2463_d_b14, eq62_e2463_d_b15, eq62_e2463_d_b16, eq62_e2463_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq62_value),
            nodes,
            &eq62_node_derivatives,
            branches,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e2471, eq63_e2471_d_n0, eq63_e2471_d_n1, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n12, eq63_e2471_d_n13, eq63_e2471_d_n14, eq63_e2471_d_n15, eq63_e2471_d_n16, eq63_e2471_d_b0, eq63_e2471_d_b1, eq63_e2471_d_b2, eq63_e2471_d_b3, eq63_e2471_d_b4, eq63_e2471_d_b5, eq63_e2471_d_b6, eq63_e2471_d_b7, eq63_e2471_d_b8, eq63_e2471_d_b9, eq63_e2471_d_b10, eq63_e2471_d_b11, eq63_e2471_d_b12, eq63_e2471_d_b13, eq63_e2471_d_b14, eq63_e2471_d_b15, eq63_e2471_d_b16, eq63_e2471_d_b17,) = {
    if (s.b[1720] && s.b[1721]) {
        let eq63_e2469: f64 = ((nv8 - nv6) * s.v[1043]);
        let eq63_e2469_d_n0: f64 = ((nv8 - nv6) * s.dn[1043][0]);
        let eq63_e2469_d_n1: f64 = ((nv8 - nv6) * s.dn[1043][1]);
        let eq63_e2469_d_n2: f64 = ((nv8 - nv6) * s.dn[1043][2]);
        let eq63_e2469_d_n3: f64 = ((nv8 - nv6) * s.dn[1043][3]);
        let eq63_e2469_d_n4: f64 = ((nv8 - nv6) * s.dn[1043][4]);
        let eq63_e2469_d_n5: f64 = ((nv8 - nv6) * s.dn[1043][5]);
        let eq63_e2469_d_n6: f64 = ((-s.v[1043]) + ((nv8 - nv6) * s.dn[1043][6]));
        let eq63_e2469_d_n7: f64 = ((nv8 - nv6) * s.dn[1043][7]);
        let eq63_e2469_d_n8: f64 = (s.v[1043] + ((nv8 - nv6) * s.dn[1043][8]));
        let eq63_e2469_d_n9: f64 = ((nv8 - nv6) * s.dn[1043][9]);
        let eq63_e2469_d_n10: f64 = ((nv8 - nv6) * s.dn[1043][10]);
        let eq63_e2469_d_n11: f64 = ((nv8 - nv6) * s.dn[1043][11]);
        let eq63_e2469_d_n12: f64 = ((nv8 - nv6) * s.dn[1043][12]);
        let eq63_e2469_d_n13: f64 = ((nv8 - nv6) * s.dn[1043][13]);
        let eq63_e2469_d_n14: f64 = ((nv8 - nv6) * s.dn[1043][14]);
        let eq63_e2469_d_n15: f64 = ((nv8 - nv6) * s.dn[1043][15]);
        let eq63_e2469_d_n16: f64 = ((nv8 - nv6) * s.dn[1043][16]);
        let eq63_e2469_d_b0: f64 = ((nv8 - nv6) * s.db[1043][0]);
        let eq63_e2469_d_b1: f64 = ((nv8 - nv6) * s.db[1043][1]);
        let eq63_e2469_d_b2: f64 = ((nv8 - nv6) * s.db[1043][2]);
        let eq63_e2469_d_b3: f64 = ((nv8 - nv6) * s.db[1043][3]);
        let eq63_e2469_d_b4: f64 = ((nv8 - nv6) * s.db[1043][4]);
        let eq63_e2469_d_b5: f64 = ((nv8 - nv6) * s.db[1043][5]);
        let eq63_e2469_d_b6: f64 = ((nv8 - nv6) * s.db[1043][6]);
        let eq63_e2469_d_b7: f64 = ((nv8 - nv6) * s.db[1043][7]);
        let eq63_e2469_d_b8: f64 = ((nv8 - nv6) * s.db[1043][8]);
        let eq63_e2469_d_b9: f64 = ((nv8 - nv6) * s.db[1043][9]);
        let eq63_e2469_d_b10: f64 = ((nv8 - nv6) * s.db[1043][10]);
        let eq63_e2469_d_b11: f64 = ((nv8 - nv6) * s.db[1043][11]);
        let eq63_e2469_d_b12: f64 = ((nv8 - nv6) * s.db[1043][12]);
        let eq63_e2469_d_b13: f64 = ((nv8 - nv6) * s.db[1043][13]);
        let eq63_e2469_d_b14: f64 = ((nv8 - nv6) * s.db[1043][14]);
        let eq63_e2469_d_b15: f64 = ((nv8 - nv6) * s.db[1043][15]);
        let eq63_e2469_d_b16: f64 = ((nv8 - nv6) * s.db[1043][16]);
        let eq63_e2469_d_b17: f64 = ((nv8 - nv6) * s.db[1043][17]);
        (eq63_e2469, eq63_e2469_d_n0, eq63_e2469_d_n1, eq63_e2469_d_n2, eq63_e2469_d_n3, eq63_e2469_d_n4, eq63_e2469_d_n5, eq63_e2469_d_n6, eq63_e2469_d_n7, eq63_e2469_d_n8, eq63_e2469_d_n9, eq63_e2469_d_n10, eq63_e2469_d_n11, eq63_e2469_d_n12, eq63_e2469_d_n13, eq63_e2469_d_n14, eq63_e2469_d_n15, eq63_e2469_d_n16, eq63_e2469_d_b0, eq63_e2469_d_b1, eq63_e2469_d_b2, eq63_e2469_d_b3, eq63_e2469_d_b4, eq63_e2469_d_b5, eq63_e2469_d_b6, eq63_e2469_d_b7, eq63_e2469_d_b8, eq63_e2469_d_b9, eq63_e2469_d_b10, eq63_e2469_d_b11, eq63_e2469_d_b12, eq63_e2469_d_b13, eq63_e2469_d_b14, eq63_e2469_d_b15, eq63_e2469_d_b16, eq63_e2469_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e2471;
        let eq63_node_derivatives: [f64; 17] = [eq63_e2471_d_n0, eq63_e2471_d_n1, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n12, eq63_e2471_d_n13, eq63_e2471_d_n14, eq63_e2471_d_n15, eq63_e2471_d_n16];
        let eq63_branch_derivatives: [f64; 18] = [eq63_e2471_d_b0, eq63_e2471_d_b1, eq63_e2471_d_b2, eq63_e2471_d_b3, eq63_e2471_d_b4, eq63_e2471_d_b5, eq63_e2471_d_b6, eq63_e2471_d_b7, eq63_e2471_d_b8, eq63_e2471_d_b9, eq63_e2471_d_b10, eq63_e2471_d_b11, eq63_e2471_d_b12, eq63_e2471_d_b13, eq63_e2471_d_b14, eq63_e2471_d_b15, eq63_e2471_d_b16, eq63_e2471_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            multiplicity * (eq63_value),
            nodes,
            &eq63_node_derivatives,
            branches,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e2478,) = {
    if (s.b[1720] && (!s.b[1721])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e2478;
        stamper.stamp_potential_const(
            branches[4],
            eq64_value,
        );
        let (eq65_e2483,) = {
    if (!s.b[1720]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e2483;
        stamper.stamp_potential_const(
            branches[5],
            eq65_value,
        );
        let (eq66_e2488,) = {
    if (!s.b[1720]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2488;
        stamper.stamp_potential_const(
            branches[6],
            eq66_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq67_e2494, eq67_e2494_d_n0, eq67_e2494_d_n1, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14, eq67_e2494_d_n15, eq67_e2494_d_n16, eq67_e2494_d_b0, eq67_e2494_d_b1, eq67_e2494_d_b2, eq67_e2494_d_b3, eq67_e2494_d_b4, eq67_e2494_d_b5, eq67_e2494_d_b6, eq67_e2494_d_b7, eq67_e2494_d_b8, eq67_e2494_d_b9, eq67_e2494_d_b10, eq67_e2494_d_b11, eq67_e2494_d_b12, eq67_e2494_d_b13, eq67_e2494_d_b14, eq67_e2494_d_b15, eq67_e2494_d_b16, eq67_e2494_d_b17,) = {
    if s.b[1722] {
        let eq67_e2492: f64 = ((nv12 - nv11) * s.v[569]);
        let eq67_e2492_d_n0: f64 = ((nv12 - nv11) * s.dn[569][0]);
        let eq67_e2492_d_n1: f64 = ((nv12 - nv11) * s.dn[569][1]);
        let eq67_e2492_d_n2: f64 = ((nv12 - nv11) * s.dn[569][2]);
        let eq67_e2492_d_n3: f64 = ((nv12 - nv11) * s.dn[569][3]);
        let eq67_e2492_d_n4: f64 = ((nv12 - nv11) * s.dn[569][4]);
        let eq67_e2492_d_n5: f64 = ((nv12 - nv11) * s.dn[569][5]);
        let eq67_e2492_d_n6: f64 = ((nv12 - nv11) * s.dn[569][6]);
        let eq67_e2492_d_n7: f64 = ((nv12 - nv11) * s.dn[569][7]);
        let eq67_e2492_d_n8: f64 = ((nv12 - nv11) * s.dn[569][8]);
        let eq67_e2492_d_n9: f64 = ((nv12 - nv11) * s.dn[569][9]);
        let eq67_e2492_d_n10: f64 = ((nv12 - nv11) * s.dn[569][10]);
        let eq67_e2492_d_n11: f64 = ((-s.v[569]) + ((nv12 - nv11) * s.dn[569][11]));
        let eq67_e2492_d_n12: f64 = (s.v[569] + ((nv12 - nv11) * s.dn[569][12]));
        let eq67_e2492_d_n13: f64 = ((nv12 - nv11) * s.dn[569][13]);
        let eq67_e2492_d_n14: f64 = ((nv12 - nv11) * s.dn[569][14]);
        let eq67_e2492_d_n15: f64 = ((nv12 - nv11) * s.dn[569][15]);
        let eq67_e2492_d_n16: f64 = ((nv12 - nv11) * s.dn[569][16]);
        let eq67_e2492_d_b0: f64 = ((nv12 - nv11) * s.db[569][0]);
        let eq67_e2492_d_b1: f64 = ((nv12 - nv11) * s.db[569][1]);
        let eq67_e2492_d_b2: f64 = ((nv12 - nv11) * s.db[569][2]);
        let eq67_e2492_d_b3: f64 = ((nv12 - nv11) * s.db[569][3]);
        let eq67_e2492_d_b4: f64 = ((nv12 - nv11) * s.db[569][4]);
        let eq67_e2492_d_b5: f64 = ((nv12 - nv11) * s.db[569][5]);
        let eq67_e2492_d_b6: f64 = ((nv12 - nv11) * s.db[569][6]);
        let eq67_e2492_d_b7: f64 = ((nv12 - nv11) * s.db[569][7]);
        let eq67_e2492_d_b8: f64 = ((nv12 - nv11) * s.db[569][8]);
        let eq67_e2492_d_b9: f64 = ((nv12 - nv11) * s.db[569][9]);
        let eq67_e2492_d_b10: f64 = ((nv12 - nv11) * s.db[569][10]);
        let eq67_e2492_d_b11: f64 = ((nv12 - nv11) * s.db[569][11]);
        let eq67_e2492_d_b12: f64 = ((nv12 - nv11) * s.db[569][12]);
        let eq67_e2492_d_b13: f64 = ((nv12 - nv11) * s.db[569][13]);
        let eq67_e2492_d_b14: f64 = ((nv12 - nv11) * s.db[569][14]);
        let eq67_e2492_d_b15: f64 = ((nv12 - nv11) * s.db[569][15]);
        let eq67_e2492_d_b16: f64 = ((nv12 - nv11) * s.db[569][16]);
        let eq67_e2492_d_b17: f64 = ((nv12 - nv11) * s.db[569][17]);
        (eq67_e2492, eq67_e2492_d_n0, eq67_e2492_d_n1, eq67_e2492_d_n2, eq67_e2492_d_n3, eq67_e2492_d_n4, eq67_e2492_d_n5, eq67_e2492_d_n6, eq67_e2492_d_n7, eq67_e2492_d_n8, eq67_e2492_d_n9, eq67_e2492_d_n10, eq67_e2492_d_n11, eq67_e2492_d_n12, eq67_e2492_d_n13, eq67_e2492_d_n14, eq67_e2492_d_n15, eq67_e2492_d_n16, eq67_e2492_d_b0, eq67_e2492_d_b1, eq67_e2492_d_b2, eq67_e2492_d_b3, eq67_e2492_d_b4, eq67_e2492_d_b5, eq67_e2492_d_b6, eq67_e2492_d_b7, eq67_e2492_d_b8, eq67_e2492_d_b9, eq67_e2492_d_b10, eq67_e2492_d_b11, eq67_e2492_d_b12, eq67_e2492_d_b13, eq67_e2492_d_b14, eq67_e2492_d_b15, eq67_e2492_d_b16, eq67_e2492_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2494;
        let eq67_node_derivatives: [f64; 17] = [eq67_e2494_d_n0, eq67_e2494_d_n1, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14, eq67_e2494_d_n15, eq67_e2494_d_n16];
        let eq67_branch_derivatives: [f64; 18] = [eq67_e2494_d_b0, eq67_e2494_d_b1, eq67_e2494_d_b2, eq67_e2494_d_b3, eq67_e2494_d_b4, eq67_e2494_d_b5, eq67_e2494_d_b6, eq67_e2494_d_b7, eq67_e2494_d_b8, eq67_e2494_d_b9, eq67_e2494_d_b10, eq67_e2494_d_b11, eq67_e2494_d_b12, eq67_e2494_d_b13, eq67_e2494_d_b14, eq67_e2494_d_b15, eq67_e2494_d_b16, eq67_e2494_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[11]),
            multiplicity * (eq67_value),
            nodes,
            &eq67_node_derivatives,
            branches,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let (eq68_e2499,) = {
    if (!s.b[1722]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e2499;
        stamper.stamp_potential_const(
            branches[7],
            eq68_value,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16, eq69_e2506_d_b0, eq69_e2506_d_b1, eq69_e2506_d_b2, eq69_e2506_d_b3, eq69_e2506_d_b4, eq69_e2506_d_b5, eq69_e2506_d_b6, eq69_e2506_d_b7, eq69_e2506_d_b8, eq69_e2506_d_b9, eq69_e2506_d_b10, eq69_e2506_d_b11, eq69_e2506_d_b12, eq69_e2506_d_b13, eq69_e2506_d_b14, eq69_e2506_d_b15, eq69_e2506_d_b16, eq69_e2506_d_b17,) = {
    if s.b[1723] {
        let eq69_e2503: f64 = (s.v[138] - s.v[140]);
        let eq69_e2503_d_n0: f64 = (s.dn[138][0] - s.dn[140][0]);
        let eq69_e2503_d_n1: f64 = (s.dn[138][1] - s.dn[140][1]);
        let eq69_e2503_d_n2: f64 = (s.dn[138][2] - s.dn[140][2]);
        let eq69_e2503_d_n3: f64 = (s.dn[138][3] - s.dn[140][3]);
        let eq69_e2503_d_n4: f64 = (s.dn[138][4] - s.dn[140][4]);
        let eq69_e2503_d_n5: f64 = (s.dn[138][5] - s.dn[140][5]);
        let eq69_e2503_d_n6: f64 = (s.dn[138][6] - s.dn[140][6]);
        let eq69_e2503_d_n7: f64 = (s.dn[138][7] - s.dn[140][7]);
        let eq69_e2503_d_n8: f64 = (s.dn[138][8] - s.dn[140][8]);
        let eq69_e2503_d_n9: f64 = (s.dn[138][9] - s.dn[140][9]);
        let eq69_e2503_d_n10: f64 = (s.dn[138][10] - s.dn[140][10]);
        let eq69_e2503_d_n11: f64 = (s.dn[138][11] - s.dn[140][11]);
        let eq69_e2503_d_n12: f64 = (s.dn[138][12] - s.dn[140][12]);
        let eq69_e2503_d_n13: f64 = (s.dn[138][13] - s.dn[140][13]);
        let eq69_e2503_d_n14: f64 = (s.dn[138][14] - s.dn[140][14]);
        let eq69_e2503_d_n15: f64 = (s.dn[138][15] - s.dn[140][15]);
        let eq69_e2503_d_n16: f64 = (s.dn[138][16] - s.dn[140][16]);
        let eq69_e2503_d_b0: f64 = (s.db[138][0] - s.db[140][0]);
        let eq69_e2503_d_b1: f64 = (s.db[138][1] - s.db[140][1]);
        let eq69_e2503_d_b2: f64 = (s.db[138][2] - s.db[140][2]);
        let eq69_e2503_d_b3: f64 = (s.db[138][3] - s.db[140][3]);
        let eq69_e2503_d_b4: f64 = (s.db[138][4] - s.db[140][4]);
        let eq69_e2503_d_b5: f64 = (s.db[138][5] - s.db[140][5]);
        let eq69_e2503_d_b6: f64 = (s.db[138][6] - s.db[140][6]);
        let eq69_e2503_d_b7: f64 = (s.db[138][7] - s.db[140][7]);
        let eq69_e2503_d_b8: f64 = (s.db[138][8] - s.db[140][8]);
        let eq69_e2503_d_b9: f64 = (s.db[138][9] - s.db[140][9]);
        let eq69_e2503_d_b10: f64 = (s.db[138][10] - s.db[140][10]);
        let eq69_e2503_d_b11: f64 = (s.db[138][11] - s.db[140][11]);
        let eq69_e2503_d_b12: f64 = (s.db[138][12] - s.db[140][12]);
        let eq69_e2503_d_b13: f64 = (s.db[138][13] - s.db[140][13]);
        let eq69_e2503_d_b14: f64 = (s.db[138][14] - s.db[140][14]);
        let eq69_e2503_d_b15: f64 = (s.db[138][15] - s.db[140][15]);
        let eq69_e2503_d_b16: f64 = (s.db[138][16] - s.db[140][16]);
        let eq69_e2503_d_b17: f64 = (s.db[138][17] - s.db[140][17]);
        let eq69_e2504: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 23, eq69_e2503);
        let eq69_e2504_d_n0: f64 = (eq69_e2503_d_n0 * ddt_scale);
        let eq69_e2504_d_n1: f64 = (eq69_e2503_d_n1 * ddt_scale);
        let eq69_e2504_d_n2: f64 = (eq69_e2503_d_n2 * ddt_scale);
        let eq69_e2504_d_n3: f64 = (eq69_e2503_d_n3 * ddt_scale);
        let eq69_e2504_d_n4: f64 = (eq69_e2503_d_n4 * ddt_scale);
        let eq69_e2504_d_n5: f64 = (eq69_e2503_d_n5 * ddt_scale);
        let eq69_e2504_d_n6: f64 = (eq69_e2503_d_n6 * ddt_scale);
        let eq69_e2504_d_n7: f64 = (eq69_e2503_d_n7 * ddt_scale);
        let eq69_e2504_d_n8: f64 = (eq69_e2503_d_n8 * ddt_scale);
        let eq69_e2504_d_n9: f64 = (eq69_e2503_d_n9 * ddt_scale);
        let eq69_e2504_d_n10: f64 = (eq69_e2503_d_n10 * ddt_scale);
        let eq69_e2504_d_n11: f64 = (eq69_e2503_d_n11 * ddt_scale);
        let eq69_e2504_d_n12: f64 = (eq69_e2503_d_n12 * ddt_scale);
        let eq69_e2504_d_n13: f64 = (eq69_e2503_d_n13 * ddt_scale);
        let eq69_e2504_d_n14: f64 = (eq69_e2503_d_n14 * ddt_scale);
        let eq69_e2504_d_n15: f64 = (eq69_e2503_d_n15 * ddt_scale);
        let eq69_e2504_d_n16: f64 = (eq69_e2503_d_n16 * ddt_scale);
        let eq69_e2504_d_b0: f64 = (eq69_e2503_d_b0 * ddt_scale);
        let eq69_e2504_d_b1: f64 = (eq69_e2503_d_b1 * ddt_scale);
        let eq69_e2504_d_b2: f64 = (eq69_e2503_d_b2 * ddt_scale);
        let eq69_e2504_d_b3: f64 = (eq69_e2503_d_b3 * ddt_scale);
        let eq69_e2504_d_b4: f64 = (eq69_e2503_d_b4 * ddt_scale);
        let eq69_e2504_d_b5: f64 = (eq69_e2503_d_b5 * ddt_scale);
        let eq69_e2504_d_b6: f64 = (eq69_e2503_d_b6 * ddt_scale);
        let eq69_e2504_d_b7: f64 = (eq69_e2503_d_b7 * ddt_scale);
        let eq69_e2504_d_b8: f64 = (eq69_e2503_d_b8 * ddt_scale);
        let eq69_e2504_d_b9: f64 = (eq69_e2503_d_b9 * ddt_scale);
        let eq69_e2504_d_b10: f64 = (eq69_e2503_d_b10 * ddt_scale);
        let eq69_e2504_d_b11: f64 = (eq69_e2503_d_b11 * ddt_scale);
        let eq69_e2504_d_b12: f64 = (eq69_e2503_d_b12 * ddt_scale);
        let eq69_e2504_d_b13: f64 = (eq69_e2503_d_b13 * ddt_scale);
        let eq69_e2504_d_b14: f64 = (eq69_e2503_d_b14 * ddt_scale);
        let eq69_e2504_d_b15: f64 = (eq69_e2503_d_b15 * ddt_scale);
        let eq69_e2504_d_b16: f64 = (eq69_e2503_d_b16 * ddt_scale);
        let eq69_e2504_d_b17: f64 = (eq69_e2503_d_b17 * ddt_scale);
        (eq69_e2504, eq69_e2504_d_n0, eq69_e2504_d_n1, eq69_e2504_d_n2, eq69_e2504_d_n3, eq69_e2504_d_n4, eq69_e2504_d_n5, eq69_e2504_d_n6, eq69_e2504_d_n7, eq69_e2504_d_n8, eq69_e2504_d_n9, eq69_e2504_d_n10, eq69_e2504_d_n11, eq69_e2504_d_n12, eq69_e2504_d_n13, eq69_e2504_d_n14, eq69_e2504_d_n15, eq69_e2504_d_n16, eq69_e2504_d_b0, eq69_e2504_d_b1, eq69_e2504_d_b2, eq69_e2504_d_b3, eq69_e2504_d_b4, eq69_e2504_d_b5, eq69_e2504_d_b6, eq69_e2504_d_b7, eq69_e2504_d_b8, eq69_e2504_d_b9, eq69_e2504_d_b10, eq69_e2504_d_b11, eq69_e2504_d_b12, eq69_e2504_d_b13, eq69_e2504_d_b14, eq69_e2504_d_b15, eq69_e2504_d_b16, eq69_e2504_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2506;
        let eq69_node_derivatives: [f64; 17] = [eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16];
        let eq69_branch_derivatives: [f64; 18] = [eq69_e2506_d_b0, eq69_e2506_d_b1, eq69_e2506_d_b2, eq69_e2506_d_b3, eq69_e2506_d_b4, eq69_e2506_d_b5, eq69_e2506_d_b6, eq69_e2506_d_b7, eq69_e2506_d_b8, eq69_e2506_d_b9, eq69_e2506_d_b10, eq69_e2506_d_b11, eq69_e2506_d_b12, eq69_e2506_d_b13, eq69_e2506_d_b14, eq69_e2506_d_b15, eq69_e2506_d_b16, eq69_e2506_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            multiplicity * (eq69_value),
            nodes,
            &eq69_node_derivatives,
            branches,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq70_e2512, eq70_e2512_d_n0, eq70_e2512_d_n1, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n12, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15, eq70_e2512_d_n16, eq70_e2512_d_b0, eq70_e2512_d_b1, eq70_e2512_d_b2, eq70_e2512_d_b3, eq70_e2512_d_b4, eq70_e2512_d_b5, eq70_e2512_d_b6, eq70_e2512_d_b7, eq70_e2512_d_b8, eq70_e2512_d_b9, eq70_e2512_d_b10, eq70_e2512_d_b11, eq70_e2512_d_b12, eq70_e2512_d_b13, eq70_e2512_d_b14, eq70_e2512_d_b15, eq70_e2512_d_b16, eq70_e2512_d_b17,) = {
    if s.b[1723] {
        let eq70_e2510: f64 = ((nv15 - 0.0) * s.v[570]);
        let eq70_e2510_d_n0: f64 = ((nv15 - 0.0) * s.dn[570][0]);
        let eq70_e2510_d_n1: f64 = ((nv15 - 0.0) * s.dn[570][1]);
        let eq70_e2510_d_n2: f64 = ((nv15 - 0.0) * s.dn[570][2]);
        let eq70_e2510_d_n3: f64 = ((nv15 - 0.0) * s.dn[570][3]);
        let eq70_e2510_d_n4: f64 = ((nv15 - 0.0) * s.dn[570][4]);
        let eq70_e2510_d_n5: f64 = ((nv15 - 0.0) * s.dn[570][5]);
        let eq70_e2510_d_n6: f64 = ((nv15 - 0.0) * s.dn[570][6]);
        let eq70_e2510_d_n7: f64 = ((nv15 - 0.0) * s.dn[570][7]);
        let eq70_e2510_d_n8: f64 = ((nv15 - 0.0) * s.dn[570][8]);
        let eq70_e2510_d_n9: f64 = ((nv15 - 0.0) * s.dn[570][9]);
        let eq70_e2510_d_n10: f64 = ((nv15 - 0.0) * s.dn[570][10]);
        let eq70_e2510_d_n11: f64 = ((nv15 - 0.0) * s.dn[570][11]);
        let eq70_e2510_d_n12: f64 = ((nv15 - 0.0) * s.dn[570][12]);
        let eq70_e2510_d_n13: f64 = ((nv15 - 0.0) * s.dn[570][13]);
        let eq70_e2510_d_n14: f64 = ((nv15 - 0.0) * s.dn[570][14]);
        let eq70_e2510_d_n15: f64 = (s.v[570] + ((nv15 - 0.0) * s.dn[570][15]));
        let eq70_e2510_d_n16: f64 = ((nv15 - 0.0) * s.dn[570][16]);
        let eq70_e2510_d_b0: f64 = ((nv15 - 0.0) * s.db[570][0]);
        let eq70_e2510_d_b1: f64 = ((nv15 - 0.0) * s.db[570][1]);
        let eq70_e2510_d_b2: f64 = ((nv15 - 0.0) * s.db[570][2]);
        let eq70_e2510_d_b3: f64 = ((nv15 - 0.0) * s.db[570][3]);
        let eq70_e2510_d_b4: f64 = ((nv15 - 0.0) * s.db[570][4]);
        let eq70_e2510_d_b5: f64 = ((nv15 - 0.0) * s.db[570][5]);
        let eq70_e2510_d_b6: f64 = ((nv15 - 0.0) * s.db[570][6]);
        let eq70_e2510_d_b7: f64 = ((nv15 - 0.0) * s.db[570][7]);
        let eq70_e2510_d_b8: f64 = ((nv15 - 0.0) * s.db[570][8]);
        let eq70_e2510_d_b9: f64 = ((nv15 - 0.0) * s.db[570][9]);
        let eq70_e2510_d_b10: f64 = ((nv15 - 0.0) * s.db[570][10]);
        let eq70_e2510_d_b11: f64 = ((nv15 - 0.0) * s.db[570][11]);
        let eq70_e2510_d_b12: f64 = ((nv15 - 0.0) * s.db[570][12]);
        let eq70_e2510_d_b13: f64 = ((nv15 - 0.0) * s.db[570][13]);
        let eq70_e2510_d_b14: f64 = ((nv15 - 0.0) * s.db[570][14]);
        let eq70_e2510_d_b15: f64 = ((nv15 - 0.0) * s.db[570][15]);
        let eq70_e2510_d_b16: f64 = ((nv15 - 0.0) * s.db[570][16]);
        let eq70_e2510_d_b17: f64 = ((nv15 - 0.0) * s.db[570][17]);
        (eq70_e2510, eq70_e2510_d_n0, eq70_e2510_d_n1, eq70_e2510_d_n2, eq70_e2510_d_n3, eq70_e2510_d_n4, eq70_e2510_d_n5, eq70_e2510_d_n6, eq70_e2510_d_n7, eq70_e2510_d_n8, eq70_e2510_d_n9, eq70_e2510_d_n10, eq70_e2510_d_n11, eq70_e2510_d_n12, eq70_e2510_d_n13, eq70_e2510_d_n14, eq70_e2510_d_n15, eq70_e2510_d_n16, eq70_e2510_d_b0, eq70_e2510_d_b1, eq70_e2510_d_b2, eq70_e2510_d_b3, eq70_e2510_d_b4, eq70_e2510_d_b5, eq70_e2510_d_b6, eq70_e2510_d_b7, eq70_e2510_d_b8, eq70_e2510_d_b9, eq70_e2510_d_b10, eq70_e2510_d_b11, eq70_e2510_d_b12, eq70_e2510_d_b13, eq70_e2510_d_b14, eq70_e2510_d_b15, eq70_e2510_d_b16, eq70_e2510_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e2512;
        let eq70_node_derivatives: [f64; 17] = [eq70_e2512_d_n0, eq70_e2512_d_n1, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n12, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15, eq70_e2512_d_n16];
        let eq70_branch_derivatives: [f64; 18] = [eq70_e2512_d_b0, eq70_e2512_d_b1, eq70_e2512_d_b2, eq70_e2512_d_b3, eq70_e2512_d_b4, eq70_e2512_d_b5, eq70_e2512_d_b6, eq70_e2512_d_b7, eq70_e2512_d_b8, eq70_e2512_d_b9, eq70_e2512_d_b10, eq70_e2512_d_b11, eq70_e2512_d_b12, eq70_e2512_d_b13, eq70_e2512_d_b14, eq70_e2512_d_b15, eq70_e2512_d_b16, eq70_e2512_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            multiplicity * (eq70_value),
            nodes,
            &eq70_node_derivatives,
            branches,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15,) = {
    if s.b[1723] {
        let eq71_e2516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 24, (nv15 - 0.0));
        let eq71_e2517: f64 = (1e-9 * eq71_e2516);
        let eq71_e2517_d_n15: f64 = (1e-9 * ddt_scale);
        (eq71_e2517, eq71_e2517_d_n15,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e2519;
        stamper.stamp_current_node1(
            Some(nodes[15]),
            None,
            multiplicity * (eq71_value),
            nodes[15],
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq72_e2524,) = {
    if (!s.b[1723]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e2524;
        stamper.stamp_potential_const(
            branches[8],
            eq72_value,
        );
        let (eq73_e2530, eq73_e2530_d_n0, eq73_e2530_d_n1, eq73_e2530_d_n2, eq73_e2530_d_n3, eq73_e2530_d_n4, eq73_e2530_d_n5, eq73_e2530_d_n6, eq73_e2530_d_n7, eq73_e2530_d_n8, eq73_e2530_d_n9, eq73_e2530_d_n10, eq73_e2530_d_n11, eq73_e2530_d_n12, eq73_e2530_d_n13, eq73_e2530_d_n14, eq73_e2530_d_n15, eq73_e2530_d_n16, eq73_e2530_d_b0, eq73_e2530_d_b1, eq73_e2530_d_b2, eq73_e2530_d_b3, eq73_e2530_d_b4, eq73_e2530_d_b5, eq73_e2530_d_b6, eq73_e2530_d_b7, eq73_e2530_d_b8, eq73_e2530_d_b9, eq73_e2530_d_b10, eq73_e2530_d_b11, eq73_e2530_d_b12, eq73_e2530_d_b13, eq73_e2530_d_b14, eq73_e2530_d_b15, eq73_e2530_d_b16, eq73_e2530_d_b17,) = {
    if s.b[1724] {
        let eq73_e2528: f64 = ((nv1 - nv10) * s.v[456]);
        let eq73_e2528_d_n0: f64 = ((nv1 - nv10) * s.dn[456][0]);
        let eq73_e2528_d_n1: f64 = (s.v[456] + ((nv1 - nv10) * s.dn[456][1]));
        let eq73_e2528_d_n2: f64 = ((nv1 - nv10) * s.dn[456][2]);
        let eq73_e2528_d_n3: f64 = ((nv1 - nv10) * s.dn[456][3]);
        let eq73_e2528_d_n4: f64 = ((nv1 - nv10) * s.dn[456][4]);
        let eq73_e2528_d_n5: f64 = ((nv1 - nv10) * s.dn[456][5]);
        let eq73_e2528_d_n6: f64 = ((nv1 - nv10) * s.dn[456][6]);
        let eq73_e2528_d_n7: f64 = ((nv1 - nv10) * s.dn[456][7]);
        let eq73_e2528_d_n8: f64 = ((nv1 - nv10) * s.dn[456][8]);
        let eq73_e2528_d_n9: f64 = ((nv1 - nv10) * s.dn[456][9]);
        let eq73_e2528_d_n10: f64 = ((-s.v[456]) + ((nv1 - nv10) * s.dn[456][10]));
        let eq73_e2528_d_n11: f64 = ((nv1 - nv10) * s.dn[456][11]);
        let eq73_e2528_d_n12: f64 = ((nv1 - nv10) * s.dn[456][12]);
        let eq73_e2528_d_n13: f64 = ((nv1 - nv10) * s.dn[456][13]);
        let eq73_e2528_d_n14: f64 = ((nv1 - nv10) * s.dn[456][14]);
        let eq73_e2528_d_n15: f64 = ((nv1 - nv10) * s.dn[456][15]);
        let eq73_e2528_d_n16: f64 = ((nv1 - nv10) * s.dn[456][16]);
        let eq73_e2528_d_b0: f64 = ((nv1 - nv10) * s.db[456][0]);
        let eq73_e2528_d_b1: f64 = ((nv1 - nv10) * s.db[456][1]);
        let eq73_e2528_d_b2: f64 = ((nv1 - nv10) * s.db[456][2]);
        let eq73_e2528_d_b3: f64 = ((nv1 - nv10) * s.db[456][3]);
        let eq73_e2528_d_b4: f64 = ((nv1 - nv10) * s.db[456][4]);
        let eq73_e2528_d_b5: f64 = ((nv1 - nv10) * s.db[456][5]);
        let eq73_e2528_d_b6: f64 = ((nv1 - nv10) * s.db[456][6]);
        let eq73_e2528_d_b7: f64 = ((nv1 - nv10) * s.db[456][7]);
        let eq73_e2528_d_b8: f64 = ((nv1 - nv10) * s.db[456][8]);
        let eq73_e2528_d_b9: f64 = ((nv1 - nv10) * s.db[456][9]);
        let eq73_e2528_d_b10: f64 = ((nv1 - nv10) * s.db[456][10]);
        let eq73_e2528_d_b11: f64 = ((nv1 - nv10) * s.db[456][11]);
        let eq73_e2528_d_b12: f64 = ((nv1 - nv10) * s.db[456][12]);
        let eq73_e2528_d_b13: f64 = ((nv1 - nv10) * s.db[456][13]);
        let eq73_e2528_d_b14: f64 = ((nv1 - nv10) * s.db[456][14]);
        let eq73_e2528_d_b15: f64 = ((nv1 - nv10) * s.db[456][15]);
        let eq73_e2528_d_b16: f64 = ((nv1 - nv10) * s.db[456][16]);
        let eq73_e2528_d_b17: f64 = ((nv1 - nv10) * s.db[456][17]);
        (eq73_e2528, eq73_e2528_d_n0, eq73_e2528_d_n1, eq73_e2528_d_n2, eq73_e2528_d_n3, eq73_e2528_d_n4, eq73_e2528_d_n5, eq73_e2528_d_n6, eq73_e2528_d_n7, eq73_e2528_d_n8, eq73_e2528_d_n9, eq73_e2528_d_n10, eq73_e2528_d_n11, eq73_e2528_d_n12, eq73_e2528_d_n13, eq73_e2528_d_n14, eq73_e2528_d_n15, eq73_e2528_d_n16, eq73_e2528_d_b0, eq73_e2528_d_b1, eq73_e2528_d_b2, eq73_e2528_d_b3, eq73_e2528_d_b4, eq73_e2528_d_b5, eq73_e2528_d_b6, eq73_e2528_d_b7, eq73_e2528_d_b8, eq73_e2528_d_b9, eq73_e2528_d_b10, eq73_e2528_d_b11, eq73_e2528_d_b12, eq73_e2528_d_b13, eq73_e2528_d_b14, eq73_e2528_d_b15, eq73_e2528_d_b16, eq73_e2528_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e2530;
        let eq73_node_derivatives: [f64; 17] = [eq73_e2530_d_n0, eq73_e2530_d_n1, eq73_e2530_d_n2, eq73_e2530_d_n3, eq73_e2530_d_n4, eq73_e2530_d_n5, eq73_e2530_d_n6, eq73_e2530_d_n7, eq73_e2530_d_n8, eq73_e2530_d_n9, eq73_e2530_d_n10, eq73_e2530_d_n11, eq73_e2530_d_n12, eq73_e2530_d_n13, eq73_e2530_d_n14, eq73_e2530_d_n15, eq73_e2530_d_n16];
        let eq73_branch_derivatives: [f64; 18] = [eq73_e2530_d_b0, eq73_e2530_d_b1, eq73_e2530_d_b2, eq73_e2530_d_b3, eq73_e2530_d_b4, eq73_e2530_d_b5, eq73_e2530_d_b6, eq73_e2530_d_b7, eq73_e2530_d_b8, eq73_e2530_d_b9, eq73_e2530_d_b10, eq73_e2530_d_b11, eq73_e2530_d_b12, eq73_e2530_d_b13, eq73_e2530_d_b14, eq73_e2530_d_b15, eq73_e2530_d_b16, eq73_e2530_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            multiplicity * (eq73_value),
            nodes,
            &eq73_node_derivatives,
            branches,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq74_e2538, eq74_e2538_d_n0, eq74_e2538_d_n1, eq74_e2538_d_n2, eq74_e2538_d_n3, eq74_e2538_d_n4, eq74_e2538_d_n5, eq74_e2538_d_n6, eq74_e2538_d_n7, eq74_e2538_d_n8, eq74_e2538_d_n9, eq74_e2538_d_n10, eq74_e2538_d_n11, eq74_e2538_d_n12, eq74_e2538_d_n13, eq74_e2538_d_n14, eq74_e2538_d_n15, eq74_e2538_d_n16, eq74_e2538_d_b0, eq74_e2538_d_b1, eq74_e2538_d_b2, eq74_e2538_d_b3, eq74_e2538_d_b4, eq74_e2538_d_b5, eq74_e2538_d_b6, eq74_e2538_d_b7, eq74_e2538_d_b8, eq74_e2538_d_b9, eq74_e2538_d_b10, eq74_e2538_d_b11, eq74_e2538_d_b12, eq74_e2538_d_b13, eq74_e2538_d_b14, eq74_e2538_d_b15, eq74_e2538_d_b16, eq74_e2538_d_b17,) = {
    if (s.b[1724] && s.b[1725]) {
        let eq74_e2536: f64 = ((nv10 - nv12) * s.v[458]);
        let eq74_e2536_d_n0: f64 = ((nv10 - nv12) * s.dn[458][0]);
        let eq74_e2536_d_n1: f64 = ((nv10 - nv12) * s.dn[458][1]);
        let eq74_e2536_d_n2: f64 = ((nv10 - nv12) * s.dn[458][2]);
        let eq74_e2536_d_n3: f64 = ((nv10 - nv12) * s.dn[458][3]);
        let eq74_e2536_d_n4: f64 = ((nv10 - nv12) * s.dn[458][4]);
        let eq74_e2536_d_n5: f64 = ((nv10 - nv12) * s.dn[458][5]);
        let eq74_e2536_d_n6: f64 = ((nv10 - nv12) * s.dn[458][6]);
        let eq74_e2536_d_n7: f64 = ((nv10 - nv12) * s.dn[458][7]);
        let eq74_e2536_d_n8: f64 = ((nv10 - nv12) * s.dn[458][8]);
        let eq74_e2536_d_n9: f64 = ((nv10 - nv12) * s.dn[458][9]);
        let eq74_e2536_d_n10: f64 = (s.v[458] + ((nv10 - nv12) * s.dn[458][10]));
        let eq74_e2536_d_n11: f64 = ((nv10 - nv12) * s.dn[458][11]);
        let eq74_e2536_d_n12: f64 = ((-s.v[458]) + ((nv10 - nv12) * s.dn[458][12]));
        let eq74_e2536_d_n13: f64 = ((nv10 - nv12) * s.dn[458][13]);
        let eq74_e2536_d_n14: f64 = ((nv10 - nv12) * s.dn[458][14]);
        let eq74_e2536_d_n15: f64 = ((nv10 - nv12) * s.dn[458][15]);
        let eq74_e2536_d_n16: f64 = ((nv10 - nv12) * s.dn[458][16]);
        let eq74_e2536_d_b0: f64 = ((nv10 - nv12) * s.db[458][0]);
        let eq74_e2536_d_b1: f64 = ((nv10 - nv12) * s.db[458][1]);
        let eq74_e2536_d_b2: f64 = ((nv10 - nv12) * s.db[458][2]);
        let eq74_e2536_d_b3: f64 = ((nv10 - nv12) * s.db[458][3]);
        let eq74_e2536_d_b4: f64 = ((nv10 - nv12) * s.db[458][4]);
        let eq74_e2536_d_b5: f64 = ((nv10 - nv12) * s.db[458][5]);
        let eq74_e2536_d_b6: f64 = ((nv10 - nv12) * s.db[458][6]);
        let eq74_e2536_d_b7: f64 = ((nv10 - nv12) * s.db[458][7]);
        let eq74_e2536_d_b8: f64 = ((nv10 - nv12) * s.db[458][8]);
        let eq74_e2536_d_b9: f64 = ((nv10 - nv12) * s.db[458][9]);
        let eq74_e2536_d_b10: f64 = ((nv10 - nv12) * s.db[458][10]);
        let eq74_e2536_d_b11: f64 = ((nv10 - nv12) * s.db[458][11]);
        let eq74_e2536_d_b12: f64 = ((nv10 - nv12) * s.db[458][12]);
        let eq74_e2536_d_b13: f64 = ((nv10 - nv12) * s.db[458][13]);
        let eq74_e2536_d_b14: f64 = ((nv10 - nv12) * s.db[458][14]);
        let eq74_e2536_d_b15: f64 = ((nv10 - nv12) * s.db[458][15]);
        let eq74_e2536_d_b16: f64 = ((nv10 - nv12) * s.db[458][16]);
        let eq74_e2536_d_b17: f64 = ((nv10 - nv12) * s.db[458][17]);
        (eq74_e2536, eq74_e2536_d_n0, eq74_e2536_d_n1, eq74_e2536_d_n2, eq74_e2536_d_n3, eq74_e2536_d_n4, eq74_e2536_d_n5, eq74_e2536_d_n6, eq74_e2536_d_n7, eq74_e2536_d_n8, eq74_e2536_d_n9, eq74_e2536_d_n10, eq74_e2536_d_n11, eq74_e2536_d_n12, eq74_e2536_d_n13, eq74_e2536_d_n14, eq74_e2536_d_n15, eq74_e2536_d_n16, eq74_e2536_d_b0, eq74_e2536_d_b1, eq74_e2536_d_b2, eq74_e2536_d_b3, eq74_e2536_d_b4, eq74_e2536_d_b5, eq74_e2536_d_b6, eq74_e2536_d_b7, eq74_e2536_d_b8, eq74_e2536_d_b9, eq74_e2536_d_b10, eq74_e2536_d_b11, eq74_e2536_d_b12, eq74_e2536_d_b13, eq74_e2536_d_b14, eq74_e2536_d_b15, eq74_e2536_d_b16, eq74_e2536_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e2538;
        let eq74_node_derivatives: [f64; 17] = [eq74_e2538_d_n0, eq74_e2538_d_n1, eq74_e2538_d_n2, eq74_e2538_d_n3, eq74_e2538_d_n4, eq74_e2538_d_n5, eq74_e2538_d_n6, eq74_e2538_d_n7, eq74_e2538_d_n8, eq74_e2538_d_n9, eq74_e2538_d_n10, eq74_e2538_d_n11, eq74_e2538_d_n12, eq74_e2538_d_n13, eq74_e2538_d_n14, eq74_e2538_d_n15, eq74_e2538_d_n16];
        let eq74_branch_derivatives: [f64; 18] = [eq74_e2538_d_b0, eq74_e2538_d_b1, eq74_e2538_d_b2, eq74_e2538_d_b3, eq74_e2538_d_b4, eq74_e2538_d_b5, eq74_e2538_d_b6, eq74_e2538_d_b7, eq74_e2538_d_b8, eq74_e2538_d_b9, eq74_e2538_d_b10, eq74_e2538_d_b11, eq74_e2538_d_b12, eq74_e2538_d_b13, eq74_e2538_d_b14, eq74_e2538_d_b15, eq74_e2538_d_b16, eq74_e2538_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[12]),
            multiplicity * (eq74_value),
            nodes,
            &eq74_node_derivatives,
            branches,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e2546, eq75_e2546_d_n0, eq75_e2546_d_n1, eq75_e2546_d_n2, eq75_e2546_d_n3, eq75_e2546_d_n4, eq75_e2546_d_n5, eq75_e2546_d_n6, eq75_e2546_d_n7, eq75_e2546_d_n8, eq75_e2546_d_n9, eq75_e2546_d_n10, eq75_e2546_d_n11, eq75_e2546_d_n12, eq75_e2546_d_n13, eq75_e2546_d_n14, eq75_e2546_d_n15, eq75_e2546_d_n16, eq75_e2546_d_b0, eq75_e2546_d_b1, eq75_e2546_d_b2, eq75_e2546_d_b3, eq75_e2546_d_b4, eq75_e2546_d_b5, eq75_e2546_d_b6, eq75_e2546_d_b7, eq75_e2546_d_b8, eq75_e2546_d_b9, eq75_e2546_d_b10, eq75_e2546_d_b11, eq75_e2546_d_b12, eq75_e2546_d_b13, eq75_e2546_d_b14, eq75_e2546_d_b15, eq75_e2546_d_b16, eq75_e2546_d_b17,) = {
    if (s.b[1724] && s.b[1725]) {
        let eq75_e2544: f64 = ((nv10 - nv13) * s.v[459]);
        let eq75_e2544_d_n0: f64 = ((nv10 - nv13) * s.dn[459][0]);
        let eq75_e2544_d_n1: f64 = ((nv10 - nv13) * s.dn[459][1]);
        let eq75_e2544_d_n2: f64 = ((nv10 - nv13) * s.dn[459][2]);
        let eq75_e2544_d_n3: f64 = ((nv10 - nv13) * s.dn[459][3]);
        let eq75_e2544_d_n4: f64 = ((nv10 - nv13) * s.dn[459][4]);
        let eq75_e2544_d_n5: f64 = ((nv10 - nv13) * s.dn[459][5]);
        let eq75_e2544_d_n6: f64 = ((nv10 - nv13) * s.dn[459][6]);
        let eq75_e2544_d_n7: f64 = ((nv10 - nv13) * s.dn[459][7]);
        let eq75_e2544_d_n8: f64 = ((nv10 - nv13) * s.dn[459][8]);
        let eq75_e2544_d_n9: f64 = ((nv10 - nv13) * s.dn[459][9]);
        let eq75_e2544_d_n10: f64 = (s.v[459] + ((nv10 - nv13) * s.dn[459][10]));
        let eq75_e2544_d_n11: f64 = ((nv10 - nv13) * s.dn[459][11]);
        let eq75_e2544_d_n12: f64 = ((nv10 - nv13) * s.dn[459][12]);
        let eq75_e2544_d_n13: f64 = ((-s.v[459]) + ((nv10 - nv13) * s.dn[459][13]));
        let eq75_e2544_d_n14: f64 = ((nv10 - nv13) * s.dn[459][14]);
        let eq75_e2544_d_n15: f64 = ((nv10 - nv13) * s.dn[459][15]);
        let eq75_e2544_d_n16: f64 = ((nv10 - nv13) * s.dn[459][16]);
        let eq75_e2544_d_b0: f64 = ((nv10 - nv13) * s.db[459][0]);
        let eq75_e2544_d_b1: f64 = ((nv10 - nv13) * s.db[459][1]);
        let eq75_e2544_d_b2: f64 = ((nv10 - nv13) * s.db[459][2]);
        let eq75_e2544_d_b3: f64 = ((nv10 - nv13) * s.db[459][3]);
        let eq75_e2544_d_b4: f64 = ((nv10 - nv13) * s.db[459][4]);
        let eq75_e2544_d_b5: f64 = ((nv10 - nv13) * s.db[459][5]);
        let eq75_e2544_d_b6: f64 = ((nv10 - nv13) * s.db[459][6]);
        let eq75_e2544_d_b7: f64 = ((nv10 - nv13) * s.db[459][7]);
        let eq75_e2544_d_b8: f64 = ((nv10 - nv13) * s.db[459][8]);
        let eq75_e2544_d_b9: f64 = ((nv10 - nv13) * s.db[459][9]);
        let eq75_e2544_d_b10: f64 = ((nv10 - nv13) * s.db[459][10]);
        let eq75_e2544_d_b11: f64 = ((nv10 - nv13) * s.db[459][11]);
        let eq75_e2544_d_b12: f64 = ((nv10 - nv13) * s.db[459][12]);
        let eq75_e2544_d_b13: f64 = ((nv10 - nv13) * s.db[459][13]);
        let eq75_e2544_d_b14: f64 = ((nv10 - nv13) * s.db[459][14]);
        let eq75_e2544_d_b15: f64 = ((nv10 - nv13) * s.db[459][15]);
        let eq75_e2544_d_b16: f64 = ((nv10 - nv13) * s.db[459][16]);
        let eq75_e2544_d_b17: f64 = ((nv10 - nv13) * s.db[459][17]);
        (eq75_e2544, eq75_e2544_d_n0, eq75_e2544_d_n1, eq75_e2544_d_n2, eq75_e2544_d_n3, eq75_e2544_d_n4, eq75_e2544_d_n5, eq75_e2544_d_n6, eq75_e2544_d_n7, eq75_e2544_d_n8, eq75_e2544_d_n9, eq75_e2544_d_n10, eq75_e2544_d_n11, eq75_e2544_d_n12, eq75_e2544_d_n13, eq75_e2544_d_n14, eq75_e2544_d_n15, eq75_e2544_d_n16, eq75_e2544_d_b0, eq75_e2544_d_b1, eq75_e2544_d_b2, eq75_e2544_d_b3, eq75_e2544_d_b4, eq75_e2544_d_b5, eq75_e2544_d_b6, eq75_e2544_d_b7, eq75_e2544_d_b8, eq75_e2544_d_b9, eq75_e2544_d_b10, eq75_e2544_d_b11, eq75_e2544_d_b12, eq75_e2544_d_b13, eq75_e2544_d_b14, eq75_e2544_d_b15, eq75_e2544_d_b16, eq75_e2544_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e2546;
        let eq75_node_derivatives: [f64; 17] = [eq75_e2546_d_n0, eq75_e2546_d_n1, eq75_e2546_d_n2, eq75_e2546_d_n3, eq75_e2546_d_n4, eq75_e2546_d_n5, eq75_e2546_d_n6, eq75_e2546_d_n7, eq75_e2546_d_n8, eq75_e2546_d_n9, eq75_e2546_d_n10, eq75_e2546_d_n11, eq75_e2546_d_n12, eq75_e2546_d_n13, eq75_e2546_d_n14, eq75_e2546_d_n15, eq75_e2546_d_n16];
        let eq75_branch_derivatives: [f64; 18] = [eq75_e2546_d_b0, eq75_e2546_d_b1, eq75_e2546_d_b2, eq75_e2546_d_b3, eq75_e2546_d_b4, eq75_e2546_d_b5, eq75_e2546_d_b6, eq75_e2546_d_b7, eq75_e2546_d_b8, eq75_e2546_d_b9, eq75_e2546_d_b10, eq75_e2546_d_b11, eq75_e2546_d_b12, eq75_e2546_d_b13, eq75_e2546_d_b14, eq75_e2546_d_b15, eq75_e2546_d_b16, eq75_e2546_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            multiplicity * (eq75_value),
            nodes,
            &eq75_node_derivatives,
            branches,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e2554, eq76_e2554_d_n0, eq76_e2554_d_n1, eq76_e2554_d_n2, eq76_e2554_d_n3, eq76_e2554_d_n4, eq76_e2554_d_n5, eq76_e2554_d_n6, eq76_e2554_d_n7, eq76_e2554_d_n8, eq76_e2554_d_n9, eq76_e2554_d_n10, eq76_e2554_d_n11, eq76_e2554_d_n12, eq76_e2554_d_n13, eq76_e2554_d_n14, eq76_e2554_d_n15, eq76_e2554_d_n16, eq76_e2554_d_b0, eq76_e2554_d_b1, eq76_e2554_d_b2, eq76_e2554_d_b3, eq76_e2554_d_b4, eq76_e2554_d_b5, eq76_e2554_d_b6, eq76_e2554_d_b7, eq76_e2554_d_b8, eq76_e2554_d_b9, eq76_e2554_d_b10, eq76_e2554_d_b11, eq76_e2554_d_b12, eq76_e2554_d_b13, eq76_e2554_d_b14, eq76_e2554_d_b15, eq76_e2554_d_b16, eq76_e2554_d_b17,) = {
    if (s.b[1724] && s.b[1725]) {
        let eq76_e2552: f64 = ((nv10 - nv14) * s.v[459]);
        let eq76_e2552_d_n0: f64 = ((nv10 - nv14) * s.dn[459][0]);
        let eq76_e2552_d_n1: f64 = ((nv10 - nv14) * s.dn[459][1]);
        let eq76_e2552_d_n2: f64 = ((nv10 - nv14) * s.dn[459][2]);
        let eq76_e2552_d_n3: f64 = ((nv10 - nv14) * s.dn[459][3]);
        let eq76_e2552_d_n4: f64 = ((nv10 - nv14) * s.dn[459][4]);
        let eq76_e2552_d_n5: f64 = ((nv10 - nv14) * s.dn[459][5]);
        let eq76_e2552_d_n6: f64 = ((nv10 - nv14) * s.dn[459][6]);
        let eq76_e2552_d_n7: f64 = ((nv10 - nv14) * s.dn[459][7]);
        let eq76_e2552_d_n8: f64 = ((nv10 - nv14) * s.dn[459][8]);
        let eq76_e2552_d_n9: f64 = ((nv10 - nv14) * s.dn[459][9]);
        let eq76_e2552_d_n10: f64 = (s.v[459] + ((nv10 - nv14) * s.dn[459][10]));
        let eq76_e2552_d_n11: f64 = ((nv10 - nv14) * s.dn[459][11]);
        let eq76_e2552_d_n12: f64 = ((nv10 - nv14) * s.dn[459][12]);
        let eq76_e2552_d_n13: f64 = ((nv10 - nv14) * s.dn[459][13]);
        let eq76_e2552_d_n14: f64 = ((-s.v[459]) + ((nv10 - nv14) * s.dn[459][14]));
        let eq76_e2552_d_n15: f64 = ((nv10 - nv14) * s.dn[459][15]);
        let eq76_e2552_d_n16: f64 = ((nv10 - nv14) * s.dn[459][16]);
        let eq76_e2552_d_b0: f64 = ((nv10 - nv14) * s.db[459][0]);
        let eq76_e2552_d_b1: f64 = ((nv10 - nv14) * s.db[459][1]);
        let eq76_e2552_d_b2: f64 = ((nv10 - nv14) * s.db[459][2]);
        let eq76_e2552_d_b3: f64 = ((nv10 - nv14) * s.db[459][3]);
        let eq76_e2552_d_b4: f64 = ((nv10 - nv14) * s.db[459][4]);
        let eq76_e2552_d_b5: f64 = ((nv10 - nv14) * s.db[459][5]);
        let eq76_e2552_d_b6: f64 = ((nv10 - nv14) * s.db[459][6]);
        let eq76_e2552_d_b7: f64 = ((nv10 - nv14) * s.db[459][7]);
        let eq76_e2552_d_b8: f64 = ((nv10 - nv14) * s.db[459][8]);
        let eq76_e2552_d_b9: f64 = ((nv10 - nv14) * s.db[459][9]);
        let eq76_e2552_d_b10: f64 = ((nv10 - nv14) * s.db[459][10]);
        let eq76_e2552_d_b11: f64 = ((nv10 - nv14) * s.db[459][11]);
        let eq76_e2552_d_b12: f64 = ((nv10 - nv14) * s.db[459][12]);
        let eq76_e2552_d_b13: f64 = ((nv10 - nv14) * s.db[459][13]);
        let eq76_e2552_d_b14: f64 = ((nv10 - nv14) * s.db[459][14]);
        let eq76_e2552_d_b15: f64 = ((nv10 - nv14) * s.db[459][15]);
        let eq76_e2552_d_b16: f64 = ((nv10 - nv14) * s.db[459][16]);
        let eq76_e2552_d_b17: f64 = ((nv10 - nv14) * s.db[459][17]);
        (eq76_e2552, eq76_e2552_d_n0, eq76_e2552_d_n1, eq76_e2552_d_n2, eq76_e2552_d_n3, eq76_e2552_d_n4, eq76_e2552_d_n5, eq76_e2552_d_n6, eq76_e2552_d_n7, eq76_e2552_d_n8, eq76_e2552_d_n9, eq76_e2552_d_n10, eq76_e2552_d_n11, eq76_e2552_d_n12, eq76_e2552_d_n13, eq76_e2552_d_n14, eq76_e2552_d_n15, eq76_e2552_d_n16, eq76_e2552_d_b0, eq76_e2552_d_b1, eq76_e2552_d_b2, eq76_e2552_d_b3, eq76_e2552_d_b4, eq76_e2552_d_b5, eq76_e2552_d_b6, eq76_e2552_d_b7, eq76_e2552_d_b8, eq76_e2552_d_b9, eq76_e2552_d_b10, eq76_e2552_d_b11, eq76_e2552_d_b12, eq76_e2552_d_b13, eq76_e2552_d_b14, eq76_e2552_d_b15, eq76_e2552_d_b16, eq76_e2552_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2554;
        let eq76_node_derivatives: [f64; 17] = [eq76_e2554_d_n0, eq76_e2554_d_n1, eq76_e2554_d_n2, eq76_e2554_d_n3, eq76_e2554_d_n4, eq76_e2554_d_n5, eq76_e2554_d_n6, eq76_e2554_d_n7, eq76_e2554_d_n8, eq76_e2554_d_n9, eq76_e2554_d_n10, eq76_e2554_d_n11, eq76_e2554_d_n12, eq76_e2554_d_n13, eq76_e2554_d_n14, eq76_e2554_d_n15, eq76_e2554_d_n16];
        let eq76_branch_derivatives: [f64; 18] = [eq76_e2554_d_b0, eq76_e2554_d_b1, eq76_e2554_d_b2, eq76_e2554_d_b3, eq76_e2554_d_b4, eq76_e2554_d_b5, eq76_e2554_d_b6, eq76_e2554_d_b7, eq76_e2554_d_b8, eq76_e2554_d_b9, eq76_e2554_d_b10, eq76_e2554_d_b11, eq76_e2554_d_b12, eq76_e2554_d_b13, eq76_e2554_d_b14, eq76_e2554_d_b15, eq76_e2554_d_b16, eq76_e2554_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[14]),
            multiplicity * (eq76_value),
            nodes,
            &eq76_node_derivatives,
            branches,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e2561,) = {
    if (s.b[1724] && (!s.b[1725])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq77_value: f64 = eq77_e2561;
        stamper.stamp_potential_const(
            branches[9],
            eq77_value,
        );
        let (eq78_e2568,) = {
    if (s.b[1724] && (!s.b[1725])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq78_value: f64 = eq78_e2568;
        stamper.stamp_potential_const(
            branches[10],
            eq78_value,
        );
        let (eq79_e2575,) = {
    if (s.b[1724] && (!s.b[1725])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq79_value: f64 = eq79_e2575;
        stamper.stamp_potential_const(
            branches[11],
            eq79_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq80_e2580,) = {
    if (!s.b[1724]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e2580;
        stamper.stamp_potential_const(
            branches[12],
            eq80_value,
        );
        let (eq81_e2585,) = {
    if (!s.b[1724]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e2585;
        stamper.stamp_potential_const(
            branches[13],
            eq81_value,
        );
        let (eq82_e2590,) = {
    if (!s.b[1724]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq82_value: f64 = eq82_e2590;
        stamper.stamp_potential_const(
            branches[14],
            eq82_value,
        );
        let (eq83_e2595,) = {
    if (!s.b[1724]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq83_value: f64 = eq83_e2595;
        stamper.stamp_potential_const(
            branches[15],
            eq83_value,
        );
        let eq84_value: f64 = 0.0;
        stamper.stamp_current_const(
            Some(nodes[5]),
            Some(nodes[6]),
            multiplicity * (eq84_value),
        );
        let (eq85_e2613,) = {
    if s.b[1726] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq85_value: f64 = eq85_e2613;
        stamper.stamp_current_const(
            Some(nodes[0]),
            Some(nodes[9]),
            multiplicity * (eq85_value),
        );
        let (eq86_e2627,) = {
    if (s.b[1726] && s.b[1727]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq86_value: f64 = eq86_e2627;
        stamper.stamp_current_const(
            Some(nodes[9]),
            Some(nodes[7]),
            multiplicity * (eq86_value),
        );
        let (eq87_e2639,) = {
    if s.b[1728] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq87_value: f64 = eq87_e2639;
        stamper.stamp_current_const(
            Some(nodes[2]),
            Some(nodes[8]),
            multiplicity * (eq87_value),
        );
        let (eq88_e2653,) = {
    if (s.b[1728] && s.b[1729]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq88_value: f64 = eq88_e2653;
        stamper.stamp_current_const(
            Some(nodes[8]),
            Some(nodes[6]),
            multiplicity * (eq88_value),
        );
        let (eq89_e2665,) = {
    if s.b[1730] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq89_value: f64 = eq89_e2665;
        stamper.stamp_current_const(
            Some(nodes[1]),
            Some(nodes[10]),
            multiplicity * (eq89_value),
        );
        let (eq90_e2671,) = {
    if s.b[1731] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq90_value: f64 = eq90_e2671;
        stamper.stamp_current_const(
            Some(nodes[5]),
            Some(nodes[6]),
            multiplicity * (eq90_value),
        );
        let (eq91_e2675,) = {
    if s.b[1731] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq91_value: f64 = eq91_e2675;
        stamper.stamp_potential_const(
            branches[16],
            eq91_value,
        );
        let (eq92_e2680, eq92_e2680_d_n16,) = {
    if (!s.b[1731]) {
        ((nv16 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e2680;
        stamper.stamp_current_node1(
            Some(nodes[16]),
            None,
            multiplicity * (eq92_value),
            nodes[16],
            multiplicity * (eq92_e2680_d_n16),
        );
        let (eq93_e2687,) = {
    if (!s.b[1731]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq93_value: f64 = eq93_e2687;
        stamper.stamp_current_const(
            Some(nodes[16]),
            None,
            multiplicity * (eq93_value),
        );
        let (eq94_e2700,) = {
    if (!s.b[1731]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq94_value: f64 = eq94_e2700;
        stamper.stamp_current_const(
            Some(nodes[5]),
            Some(nodes[6]),
            multiplicity * (eq94_value),
        );
        let (eq95_e2707, eq95_e2707_d_n0, eq95_e2707_d_n1, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n12, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n15, eq95_e2707_d_n16, eq95_e2707_d_b0, eq95_e2707_d_b1, eq95_e2707_d_b2, eq95_e2707_d_b3, eq95_e2707_d_b4, eq95_e2707_d_b5, eq95_e2707_d_b6, eq95_e2707_d_b7, eq95_e2707_d_b8, eq95_e2707_d_b9, eq95_e2707_d_b10, eq95_e2707_d_b11, eq95_e2707_d_b12, eq95_e2707_d_b13, eq95_e2707_d_b14, eq95_e2707_d_b15, eq95_e2707_d_b16, eq95_e2707_d_b17,) = {
    if (!s.b[1731]) {
        let eq95_e2705: f64 = (s.v[631] * (nv16 - 0.0));
        let eq95_e2705_d_n0: f64 = (s.dn[631][0] * (nv16 - 0.0));
        let eq95_e2705_d_n1: f64 = (s.dn[631][1] * (nv16 - 0.0));
        let eq95_e2705_d_n2: f64 = (s.dn[631][2] * (nv16 - 0.0));
        let eq95_e2705_d_n3: f64 = (s.dn[631][3] * (nv16 - 0.0));
        let eq95_e2705_d_n4: f64 = (s.dn[631][4] * (nv16 - 0.0));
        let eq95_e2705_d_n5: f64 = (s.dn[631][5] * (nv16 - 0.0));
        let eq95_e2705_d_n6: f64 = (s.dn[631][6] * (nv16 - 0.0));
        let eq95_e2705_d_n7: f64 = (s.dn[631][7] * (nv16 - 0.0));
        let eq95_e2705_d_n8: f64 = (s.dn[631][8] * (nv16 - 0.0));
        let eq95_e2705_d_n9: f64 = (s.dn[631][9] * (nv16 - 0.0));
        let eq95_e2705_d_n10: f64 = (s.dn[631][10] * (nv16 - 0.0));
        let eq95_e2705_d_n11: f64 = (s.dn[631][11] * (nv16 - 0.0));
        let eq95_e2705_d_n12: f64 = (s.dn[631][12] * (nv16 - 0.0));
        let eq95_e2705_d_n13: f64 = (s.dn[631][13] * (nv16 - 0.0));
        let eq95_e2705_d_n14: f64 = (s.dn[631][14] * (nv16 - 0.0));
        let eq95_e2705_d_n15: f64 = (s.dn[631][15] * (nv16 - 0.0));
        let eq95_e2705_d_n16: f64 = ((s.dn[631][16] * (nv16 - 0.0)) + s.v[631]);
        let eq95_e2705_d_b0: f64 = (s.db[631][0] * (nv16 - 0.0));
        let eq95_e2705_d_b1: f64 = (s.db[631][1] * (nv16 - 0.0));
        let eq95_e2705_d_b2: f64 = (s.db[631][2] * (nv16 - 0.0));
        let eq95_e2705_d_b3: f64 = (s.db[631][3] * (nv16 - 0.0));
        let eq95_e2705_d_b4: f64 = (s.db[631][4] * (nv16 - 0.0));
        let eq95_e2705_d_b5: f64 = (s.db[631][5] * (nv16 - 0.0));
        let eq95_e2705_d_b6: f64 = (s.db[631][6] * (nv16 - 0.0));
        let eq95_e2705_d_b7: f64 = (s.db[631][7] * (nv16 - 0.0));
        let eq95_e2705_d_b8: f64 = (s.db[631][8] * (nv16 - 0.0));
        let eq95_e2705_d_b9: f64 = (s.db[631][9] * (nv16 - 0.0));
        let eq95_e2705_d_b10: f64 = (s.db[631][10] * (nv16 - 0.0));
        let eq95_e2705_d_b11: f64 = (s.db[631][11] * (nv16 - 0.0));
        let eq95_e2705_d_b12: f64 = (s.db[631][12] * (nv16 - 0.0));
        let eq95_e2705_d_b13: f64 = (s.db[631][13] * (nv16 - 0.0));
        let eq95_e2705_d_b14: f64 = (s.db[631][14] * (nv16 - 0.0));
        let eq95_e2705_d_b15: f64 = (s.db[631][15] * (nv16 - 0.0));
        let eq95_e2705_d_b16: f64 = (s.db[631][16] * (nv16 - 0.0));
        let eq95_e2705_d_b17: f64 = (s.db[631][17] * (nv16 - 0.0));
        (eq95_e2705, eq95_e2705_d_n0, eq95_e2705_d_n1, eq95_e2705_d_n2, eq95_e2705_d_n3, eq95_e2705_d_n4, eq95_e2705_d_n5, eq95_e2705_d_n6, eq95_e2705_d_n7, eq95_e2705_d_n8, eq95_e2705_d_n9, eq95_e2705_d_n10, eq95_e2705_d_n11, eq95_e2705_d_n12, eq95_e2705_d_n13, eq95_e2705_d_n14, eq95_e2705_d_n15, eq95_e2705_d_n16, eq95_e2705_d_b0, eq95_e2705_d_b1, eq95_e2705_d_b2, eq95_e2705_d_b3, eq95_e2705_d_b4, eq95_e2705_d_b5, eq95_e2705_d_b6, eq95_e2705_d_b7, eq95_e2705_d_b8, eq95_e2705_d_b9, eq95_e2705_d_b10, eq95_e2705_d_b11, eq95_e2705_d_b12, eq95_e2705_d_b13, eq95_e2705_d_b14, eq95_e2705_d_b15, eq95_e2705_d_b16, eq95_e2705_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq95_value: f64 = eq95_e2707;
        let eq95_node_derivatives: [f64; 17] = [eq95_e2707_d_n0, eq95_e2707_d_n1, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n12, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n15, eq95_e2707_d_n16];
        let eq95_branch_derivatives: [f64; 18] = [eq95_e2707_d_b0, eq95_e2707_d_b1, eq95_e2707_d_b2, eq95_e2707_d_b3, eq95_e2707_d_b4, eq95_e2707_d_b5, eq95_e2707_d_b6, eq95_e2707_d_b7, eq95_e2707_d_b8, eq95_e2707_d_b9, eq95_e2707_d_b10, eq95_e2707_d_b11, eq95_e2707_d_b12, eq95_e2707_d_b13, eq95_e2707_d_b14, eq95_e2707_d_b15, eq95_e2707_d_b16, eq95_e2707_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            multiplicity * (eq95_value),
            nodes,
            &eq95_node_derivatives,
            branches,
            &eq95_branch_derivatives,
            multiplicity,
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16, eq96_e2717_d_b0, eq96_e2717_d_b1, eq96_e2717_d_b2, eq96_e2717_d_b3, eq96_e2717_d_b4, eq96_e2717_d_b5, eq96_e2717_d_b6, eq96_e2717_d_b7, eq96_e2717_d_b8, eq96_e2717_d_b9, eq96_e2717_d_b10, eq96_e2717_d_b11, eq96_e2717_d_b12, eq96_e2717_d_b13, eq96_e2717_d_b14, eq96_e2717_d_b15, eq96_e2717_d_b16, eq96_e2717_d_b17,) = {
    if (!s.b[1731]) {
        let eq96_e2712: f64 = (0.7071 * s.v[632]);
        let eq96_e2712_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq96_e2712_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq96_e2712_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq96_e2712_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq96_e2712_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq96_e2712_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq96_e2712_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq96_e2712_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq96_e2712_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq96_e2712_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq96_e2712_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq96_e2712_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq96_e2712_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq96_e2712_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq96_e2712_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq96_e2712_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq96_e2712_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq96_e2712_d_b0: f64 = (0.7071 * s.db[632][0]);
        let eq96_e2712_d_b1: f64 = (0.7071 * s.db[632][1]);
        let eq96_e2712_d_b2: f64 = (0.7071 * s.db[632][2]);
        let eq96_e2712_d_b3: f64 = (0.7071 * s.db[632][3]);
        let eq96_e2712_d_b4: f64 = (0.7071 * s.db[632][4]);
        let eq96_e2712_d_b5: f64 = (0.7071 * s.db[632][5]);
        let eq96_e2712_d_b6: f64 = (0.7071 * s.db[632][6]);
        let eq96_e2712_d_b7: f64 = (0.7071 * s.db[632][7]);
        let eq96_e2712_d_b8: f64 = (0.7071 * s.db[632][8]);
        let eq96_e2712_d_b9: f64 = (0.7071 * s.db[632][9]);
        let eq96_e2712_d_b10: f64 = (0.7071 * s.db[632][10]);
        let eq96_e2712_d_b11: f64 = (0.7071 * s.db[632][11]);
        let eq96_e2712_d_b12: f64 = (0.7071 * s.db[632][12]);
        let eq96_e2712_d_b13: f64 = (0.7071 * s.db[632][13]);
        let eq96_e2712_d_b14: f64 = (0.7071 * s.db[632][14]);
        let eq96_e2712_d_b15: f64 = (0.7071 * s.db[632][15]);
        let eq96_e2712_d_b16: f64 = (0.7071 * s.db[632][16]);
        let eq96_e2712_d_b17: f64 = (0.7071 * s.db[632][17]);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n1: f64 = (eq96_e2712_d_n1 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n12: f64 = (eq96_e2712_d_n12 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2714_d_n15: f64 = (eq96_e2712_d_n15 * (nv16 - 0.0));
        let eq96_e2714_d_n16: f64 = ((eq96_e2712_d_n16 * (nv16 - 0.0)) + eq96_e2712);
        let eq96_e2714_d_b0: f64 = (eq96_e2712_d_b0 * (nv16 - 0.0));
        let eq96_e2714_d_b1: f64 = (eq96_e2712_d_b1 * (nv16 - 0.0));
        let eq96_e2714_d_b2: f64 = (eq96_e2712_d_b2 * (nv16 - 0.0));
        let eq96_e2714_d_b3: f64 = (eq96_e2712_d_b3 * (nv16 - 0.0));
        let eq96_e2714_d_b4: f64 = (eq96_e2712_d_b4 * (nv16 - 0.0));
        let eq96_e2714_d_b5: f64 = (eq96_e2712_d_b5 * (nv16 - 0.0));
        let eq96_e2714_d_b6: f64 = (eq96_e2712_d_b6 * (nv16 - 0.0));
        let eq96_e2714_d_b7: f64 = (eq96_e2712_d_b7 * (nv16 - 0.0));
        let eq96_e2714_d_b8: f64 = (eq96_e2712_d_b8 * (nv16 - 0.0));
        let eq96_e2714_d_b9: f64 = (eq96_e2712_d_b9 * (nv16 - 0.0));
        let eq96_e2714_d_b10: f64 = (eq96_e2712_d_b10 * (nv16 - 0.0));
        let eq96_e2714_d_b11: f64 = (eq96_e2712_d_b11 * (nv16 - 0.0));
        let eq96_e2714_d_b12: f64 = (eq96_e2712_d_b12 * (nv16 - 0.0));
        let eq96_e2714_d_b13: f64 = (eq96_e2712_d_b13 * (nv16 - 0.0));
        let eq96_e2714_d_b14: f64 = (eq96_e2712_d_b14 * (nv16 - 0.0));
        let eq96_e2714_d_b15: f64 = (eq96_e2712_d_b15 * (nv16 - 0.0));
        let eq96_e2714_d_b16: f64 = (eq96_e2712_d_b16 * (nv16 - 0.0));
        let eq96_e2714_d_b17: f64 = (eq96_e2712_d_b17 * (nv16 - 0.0));
        let eq96_e2715: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 25, eq96_e2714);
        let eq96_e2715_d_n0: f64 = (eq96_e2714_d_n0 * ddt_scale);
        let eq96_e2715_d_n1: f64 = (eq96_e2714_d_n1 * ddt_scale);
        let eq96_e2715_d_n2: f64 = (eq96_e2714_d_n2 * ddt_scale);
        let eq96_e2715_d_n3: f64 = (eq96_e2714_d_n3 * ddt_scale);
        let eq96_e2715_d_n4: f64 = (eq96_e2714_d_n4 * ddt_scale);
        let eq96_e2715_d_n5: f64 = (eq96_e2714_d_n5 * ddt_scale);
        let eq96_e2715_d_n6: f64 = (eq96_e2714_d_n6 * ddt_scale);
        let eq96_e2715_d_n7: f64 = (eq96_e2714_d_n7 * ddt_scale);
        let eq96_e2715_d_n8: f64 = (eq96_e2714_d_n8 * ddt_scale);
        let eq96_e2715_d_n9: f64 = (eq96_e2714_d_n9 * ddt_scale);
        let eq96_e2715_d_n10: f64 = (eq96_e2714_d_n10 * ddt_scale);
        let eq96_e2715_d_n11: f64 = (eq96_e2714_d_n11 * ddt_scale);
        let eq96_e2715_d_n12: f64 = (eq96_e2714_d_n12 * ddt_scale);
        let eq96_e2715_d_n13: f64 = (eq96_e2714_d_n13 * ddt_scale);
        let eq96_e2715_d_n14: f64 = (eq96_e2714_d_n14 * ddt_scale);
        let eq96_e2715_d_n15: f64 = (eq96_e2714_d_n15 * ddt_scale);
        let eq96_e2715_d_n16: f64 = (eq96_e2714_d_n16 * ddt_scale);
        let eq96_e2715_d_b0: f64 = (eq96_e2714_d_b0 * ddt_scale);
        let eq96_e2715_d_b1: f64 = (eq96_e2714_d_b1 * ddt_scale);
        let eq96_e2715_d_b2: f64 = (eq96_e2714_d_b2 * ddt_scale);
        let eq96_e2715_d_b3: f64 = (eq96_e2714_d_b3 * ddt_scale);
        let eq96_e2715_d_b4: f64 = (eq96_e2714_d_b4 * ddt_scale);
        let eq96_e2715_d_b5: f64 = (eq96_e2714_d_b5 * ddt_scale);
        let eq96_e2715_d_b6: f64 = (eq96_e2714_d_b6 * ddt_scale);
        let eq96_e2715_d_b7: f64 = (eq96_e2714_d_b7 * ddt_scale);
        let eq96_e2715_d_b8: f64 = (eq96_e2714_d_b8 * ddt_scale);
        let eq96_e2715_d_b9: f64 = (eq96_e2714_d_b9 * ddt_scale);
        let eq96_e2715_d_b10: f64 = (eq96_e2714_d_b10 * ddt_scale);
        let eq96_e2715_d_b11: f64 = (eq96_e2714_d_b11 * ddt_scale);
        let eq96_e2715_d_b12: f64 = (eq96_e2714_d_b12 * ddt_scale);
        let eq96_e2715_d_b13: f64 = (eq96_e2714_d_b13 * ddt_scale);
        let eq96_e2715_d_b14: f64 = (eq96_e2714_d_b14 * ddt_scale);
        let eq96_e2715_d_b15: f64 = (eq96_e2714_d_b15 * ddt_scale);
        let eq96_e2715_d_b16: f64 = (eq96_e2714_d_b16 * ddt_scale);
        let eq96_e2715_d_b17: f64 = (eq96_e2714_d_b17 * ddt_scale);
        (eq96_e2715, eq96_e2715_d_n0, eq96_e2715_d_n1, eq96_e2715_d_n2, eq96_e2715_d_n3, eq96_e2715_d_n4, eq96_e2715_d_n5, eq96_e2715_d_n6, eq96_e2715_d_n7, eq96_e2715_d_n8, eq96_e2715_d_n9, eq96_e2715_d_n10, eq96_e2715_d_n11, eq96_e2715_d_n12, eq96_e2715_d_n13, eq96_e2715_d_n14, eq96_e2715_d_n15, eq96_e2715_d_n16, eq96_e2715_d_b0, eq96_e2715_d_b1, eq96_e2715_d_b2, eq96_e2715_d_b3, eq96_e2715_d_b4, eq96_e2715_d_b5, eq96_e2715_d_b6, eq96_e2715_d_b7, eq96_e2715_d_b8, eq96_e2715_d_b9, eq96_e2715_d_b10, eq96_e2715_d_b11, eq96_e2715_d_b12, eq96_e2715_d_b13, eq96_e2715_d_b14, eq96_e2715_d_b15, eq96_e2715_d_b16, eq96_e2715_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e2717;
        let eq96_node_derivatives: [f64; 17] = [eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16];
        let eq96_branch_derivatives: [f64; 18] = [eq96_e2717_d_b0, eq96_e2717_d_b1, eq96_e2717_d_b2, eq96_e2717_d_b3, eq96_e2717_d_b4, eq96_e2717_d_b5, eq96_e2717_d_b6, eq96_e2717_d_b7, eq96_e2717_d_b8, eq96_e2717_d_b9, eq96_e2717_d_b10, eq96_e2717_d_b11, eq96_e2717_d_b12, eq96_e2717_d_b13, eq96_e2717_d_b14, eq96_e2717_d_b15, eq96_e2717_d_b16, eq96_e2717_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            multiplicity * (eq96_value),
            nodes,
            &eq96_node_derivatives,
            branches,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16, eq97_e2727_d_b0, eq97_e2727_d_b1, eq97_e2727_d_b2, eq97_e2727_d_b3, eq97_e2727_d_b4, eq97_e2727_d_b5, eq97_e2727_d_b6, eq97_e2727_d_b7, eq97_e2727_d_b8, eq97_e2727_d_b9, eq97_e2727_d_b10, eq97_e2727_d_b11, eq97_e2727_d_b12, eq97_e2727_d_b13, eq97_e2727_d_b14, eq97_e2727_d_b15, eq97_e2727_d_b16, eq97_e2727_d_b17,) = {
    if (!s.b[1731]) {
        let eq97_e2722: f64 = (0.7071 * s.v[632]);
        let eq97_e2722_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq97_e2722_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq97_e2722_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq97_e2722_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq97_e2722_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq97_e2722_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq97_e2722_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq97_e2722_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq97_e2722_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq97_e2722_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq97_e2722_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq97_e2722_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq97_e2722_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq97_e2722_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq97_e2722_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq97_e2722_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq97_e2722_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq97_e2722_d_b0: f64 = (0.7071 * s.db[632][0]);
        let eq97_e2722_d_b1: f64 = (0.7071 * s.db[632][1]);
        let eq97_e2722_d_b2: f64 = (0.7071 * s.db[632][2]);
        let eq97_e2722_d_b3: f64 = (0.7071 * s.db[632][3]);
        let eq97_e2722_d_b4: f64 = (0.7071 * s.db[632][4]);
        let eq97_e2722_d_b5: f64 = (0.7071 * s.db[632][5]);
        let eq97_e2722_d_b6: f64 = (0.7071 * s.db[632][6]);
        let eq97_e2722_d_b7: f64 = (0.7071 * s.db[632][7]);
        let eq97_e2722_d_b8: f64 = (0.7071 * s.db[632][8]);
        let eq97_e2722_d_b9: f64 = (0.7071 * s.db[632][9]);
        let eq97_e2722_d_b10: f64 = (0.7071 * s.db[632][10]);
        let eq97_e2722_d_b11: f64 = (0.7071 * s.db[632][11]);
        let eq97_e2722_d_b12: f64 = (0.7071 * s.db[632][12]);
        let eq97_e2722_d_b13: f64 = (0.7071 * s.db[632][13]);
        let eq97_e2722_d_b14: f64 = (0.7071 * s.db[632][14]);
        let eq97_e2722_d_b15: f64 = (0.7071 * s.db[632][15]);
        let eq97_e2722_d_b16: f64 = (0.7071 * s.db[632][16]);
        let eq97_e2722_d_b17: f64 = (0.7071 * s.db[632][17]);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n1: f64 = (eq97_e2722_d_n1 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n12: f64 = (eq97_e2722_d_n12 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2724_d_n15: f64 = (eq97_e2722_d_n15 * (nv16 - 0.0));
        let eq97_e2724_d_n16: f64 = ((eq97_e2722_d_n16 * (nv16 - 0.0)) + eq97_e2722);
        let eq97_e2724_d_b0: f64 = (eq97_e2722_d_b0 * (nv16 - 0.0));
        let eq97_e2724_d_b1: f64 = (eq97_e2722_d_b1 * (nv16 - 0.0));
        let eq97_e2724_d_b2: f64 = (eq97_e2722_d_b2 * (nv16 - 0.0));
        let eq97_e2724_d_b3: f64 = (eq97_e2722_d_b3 * (nv16 - 0.0));
        let eq97_e2724_d_b4: f64 = (eq97_e2722_d_b4 * (nv16 - 0.0));
        let eq97_e2724_d_b5: f64 = (eq97_e2722_d_b5 * (nv16 - 0.0));
        let eq97_e2724_d_b6: f64 = (eq97_e2722_d_b6 * (nv16 - 0.0));
        let eq97_e2724_d_b7: f64 = (eq97_e2722_d_b7 * (nv16 - 0.0));
        let eq97_e2724_d_b8: f64 = (eq97_e2722_d_b8 * (nv16 - 0.0));
        let eq97_e2724_d_b9: f64 = (eq97_e2722_d_b9 * (nv16 - 0.0));
        let eq97_e2724_d_b10: f64 = (eq97_e2722_d_b10 * (nv16 - 0.0));
        let eq97_e2724_d_b11: f64 = (eq97_e2722_d_b11 * (nv16 - 0.0));
        let eq97_e2724_d_b12: f64 = (eq97_e2722_d_b12 * (nv16 - 0.0));
        let eq97_e2724_d_b13: f64 = (eq97_e2722_d_b13 * (nv16 - 0.0));
        let eq97_e2724_d_b14: f64 = (eq97_e2722_d_b14 * (nv16 - 0.0));
        let eq97_e2724_d_b15: f64 = (eq97_e2722_d_b15 * (nv16 - 0.0));
        let eq97_e2724_d_b16: f64 = (eq97_e2722_d_b16 * (nv16 - 0.0));
        let eq97_e2724_d_b17: f64 = (eq97_e2722_d_b17 * (nv16 - 0.0));
        let eq97_e2725: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 26, eq97_e2724);
        let eq97_e2725_d_n0: f64 = (eq97_e2724_d_n0 * ddt_scale);
        let eq97_e2725_d_n1: f64 = (eq97_e2724_d_n1 * ddt_scale);
        let eq97_e2725_d_n2: f64 = (eq97_e2724_d_n2 * ddt_scale);
        let eq97_e2725_d_n3: f64 = (eq97_e2724_d_n3 * ddt_scale);
        let eq97_e2725_d_n4: f64 = (eq97_e2724_d_n4 * ddt_scale);
        let eq97_e2725_d_n5: f64 = (eq97_e2724_d_n5 * ddt_scale);
        let eq97_e2725_d_n6: f64 = (eq97_e2724_d_n6 * ddt_scale);
        let eq97_e2725_d_n7: f64 = (eq97_e2724_d_n7 * ddt_scale);
        let eq97_e2725_d_n8: f64 = (eq97_e2724_d_n8 * ddt_scale);
        let eq97_e2725_d_n9: f64 = (eq97_e2724_d_n9 * ddt_scale);
        let eq97_e2725_d_n10: f64 = (eq97_e2724_d_n10 * ddt_scale);
        let eq97_e2725_d_n11: f64 = (eq97_e2724_d_n11 * ddt_scale);
        let eq97_e2725_d_n12: f64 = (eq97_e2724_d_n12 * ddt_scale);
        let eq97_e2725_d_n13: f64 = (eq97_e2724_d_n13 * ddt_scale);
        let eq97_e2725_d_n14: f64 = (eq97_e2724_d_n14 * ddt_scale);
        let eq97_e2725_d_n15: f64 = (eq97_e2724_d_n15 * ddt_scale);
        let eq97_e2725_d_n16: f64 = (eq97_e2724_d_n16 * ddt_scale);
        let eq97_e2725_d_b0: f64 = (eq97_e2724_d_b0 * ddt_scale);
        let eq97_e2725_d_b1: f64 = (eq97_e2724_d_b1 * ddt_scale);
        let eq97_e2725_d_b2: f64 = (eq97_e2724_d_b2 * ddt_scale);
        let eq97_e2725_d_b3: f64 = (eq97_e2724_d_b3 * ddt_scale);
        let eq97_e2725_d_b4: f64 = (eq97_e2724_d_b4 * ddt_scale);
        let eq97_e2725_d_b5: f64 = (eq97_e2724_d_b5 * ddt_scale);
        let eq97_e2725_d_b6: f64 = (eq97_e2724_d_b6 * ddt_scale);
        let eq97_e2725_d_b7: f64 = (eq97_e2724_d_b7 * ddt_scale);
        let eq97_e2725_d_b8: f64 = (eq97_e2724_d_b8 * ddt_scale);
        let eq97_e2725_d_b9: f64 = (eq97_e2724_d_b9 * ddt_scale);
        let eq97_e2725_d_b10: f64 = (eq97_e2724_d_b10 * ddt_scale);
        let eq97_e2725_d_b11: f64 = (eq97_e2724_d_b11 * ddt_scale);
        let eq97_e2725_d_b12: f64 = (eq97_e2724_d_b12 * ddt_scale);
        let eq97_e2725_d_b13: f64 = (eq97_e2724_d_b13 * ddt_scale);
        let eq97_e2725_d_b14: f64 = (eq97_e2724_d_b14 * ddt_scale);
        let eq97_e2725_d_b15: f64 = (eq97_e2724_d_b15 * ddt_scale);
        let eq97_e2725_d_b16: f64 = (eq97_e2724_d_b16 * ddt_scale);
        let eq97_e2725_d_b17: f64 = (eq97_e2724_d_b17 * ddt_scale);
        (eq97_e2725, eq97_e2725_d_n0, eq97_e2725_d_n1, eq97_e2725_d_n2, eq97_e2725_d_n3, eq97_e2725_d_n4, eq97_e2725_d_n5, eq97_e2725_d_n6, eq97_e2725_d_n7, eq97_e2725_d_n8, eq97_e2725_d_n9, eq97_e2725_d_n10, eq97_e2725_d_n11, eq97_e2725_d_n12, eq97_e2725_d_n13, eq97_e2725_d_n14, eq97_e2725_d_n15, eq97_e2725_d_n16, eq97_e2725_d_b0, eq97_e2725_d_b1, eq97_e2725_d_b2, eq97_e2725_d_b3, eq97_e2725_d_b4, eq97_e2725_d_b5, eq97_e2725_d_b6, eq97_e2725_d_b7, eq97_e2725_d_b8, eq97_e2725_d_b9, eq97_e2725_d_b10, eq97_e2725_d_b11, eq97_e2725_d_b12, eq97_e2725_d_b13, eq97_e2725_d_b14, eq97_e2725_d_b15, eq97_e2725_d_b16, eq97_e2725_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_value: f64 = eq97_e2727;
        let eq97_node_derivatives: [f64; 17] = [eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16];
        let eq97_branch_derivatives: [f64; 18] = [eq97_e2727_d_b0, eq97_e2727_d_b1, eq97_e2727_d_b2, eq97_e2727_d_b3, eq97_e2727_d_b4, eq97_e2727_d_b5, eq97_e2727_d_b6, eq97_e2727_d_b7, eq97_e2727_d_b8, eq97_e2727_d_b9, eq97_e2727_d_b10, eq97_e2727_d_b11, eq97_e2727_d_b12, eq97_e2727_d_b13, eq97_e2727_d_b14, eq97_e2727_d_b15, eq97_e2727_d_b16, eq97_e2727_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq97_value),
            nodes,
            &eq97_node_derivatives,
            branches,
            &eq97_branch_derivatives,
            multiplicity,
        );
        let (eq98_e2742,) = {
    if (s.b[1732] && s.b[1733]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq98_value: f64 = eq98_e2742;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[6]),
            multiplicity * (eq98_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq99_e2757,) = {
    if (s.b[1732] && s.b[1733]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq99_value: f64 = eq99_e2757;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq99_value),
        );
        let (eq100_e2773,) = {
    if (s.b[1732] && (!s.b[1733])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq100_value: f64 = eq100_e2773;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq100_value),
        );
        let (eq101_e2789,) = {
    if (s.b[1732] && (!s.b[1733])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq101_value: f64 = eq101_e2789;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[6]),
            multiplicity * (eq101_value),
        );
        let (eq102_e2804,) = {
    if (s.b[1734] && s.b[1735]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq102_value: f64 = eq102_e2804;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[3]),
            multiplicity * (eq102_value),
        );
        let (eq103_e2818,) = {
    if (s.b[1734] && (!s.b[1735])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq103_value: f64 = eq103_e2818;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[6]),
            multiplicity * (eq103_value),
        );
        let (eq104_e2832,) = {
    if (s.b[1734] && (!s.b[1735])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq104_value: f64 = eq104_e2832;
        stamper.stamp_current_const(
            Some(nodes[11]),
            Some(nodes[5]),
            multiplicity * (eq104_value),
        );
        let (eq105_e2843, eq105_e2843_d_n0, eq105_e2843_d_n1, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n12, eq105_e2843_d_n13, eq105_e2843_d_n14, eq105_e2843_d_n15, eq105_e2843_d_n16, eq105_e2843_d_b0, eq105_e2843_d_b1, eq105_e2843_d_b2, eq105_e2843_d_b3, eq105_e2843_d_b4, eq105_e2843_d_b5, eq105_e2843_d_b6, eq105_e2843_d_b7, eq105_e2843_d_b8, eq105_e2843_d_b9, eq105_e2843_d_b10, eq105_e2843_d_b11, eq105_e2843_d_b12, eq105_e2843_d_b13, eq105_e2843_d_b14, eq105_e2843_d_b15, eq105_e2843_d_b16, eq105_e2843_d_b17,) = {
    if s.b[1736] {
        let eq105_e2836: f64 = (s.v[114] * s.v[128]);
        let eq105_e2836_d_n0: f64 = ((s.dn[114][0] * s.v[128]) + (s.v[114] * s.dn[128][0]));
        let eq105_e2836_d_n1: f64 = ((s.dn[114][1] * s.v[128]) + (s.v[114] * s.dn[128][1]));
        let eq105_e2836_d_n2: f64 = ((s.dn[114][2] * s.v[128]) + (s.v[114] * s.dn[128][2]));
        let eq105_e2836_d_n3: f64 = ((s.dn[114][3] * s.v[128]) + (s.v[114] * s.dn[128][3]));
        let eq105_e2836_d_n4: f64 = ((s.dn[114][4] * s.v[128]) + (s.v[114] * s.dn[128][4]));
        let eq105_e2836_d_n5: f64 = ((s.dn[114][5] * s.v[128]) + (s.v[114] * s.dn[128][5]));
        let eq105_e2836_d_n6: f64 = ((s.dn[114][6] * s.v[128]) + (s.v[114] * s.dn[128][6]));
        let eq105_e2836_d_n7: f64 = ((s.dn[114][7] * s.v[128]) + (s.v[114] * s.dn[128][7]));
        let eq105_e2836_d_n8: f64 = ((s.dn[114][8] * s.v[128]) + (s.v[114] * s.dn[128][8]));
        let eq105_e2836_d_n9: f64 = ((s.dn[114][9] * s.v[128]) + (s.v[114] * s.dn[128][9]));
        let eq105_e2836_d_n10: f64 = ((s.dn[114][10] * s.v[128]) + (s.v[114] * s.dn[128][10]));
        let eq105_e2836_d_n11: f64 = ((s.dn[114][11] * s.v[128]) + (s.v[114] * s.dn[128][11]));
        let eq105_e2836_d_n12: f64 = ((s.dn[114][12] * s.v[128]) + (s.v[114] * s.dn[128][12]));
        let eq105_e2836_d_n13: f64 = ((s.dn[114][13] * s.v[128]) + (s.v[114] * s.dn[128][13]));
        let eq105_e2836_d_n14: f64 = ((s.dn[114][14] * s.v[128]) + (s.v[114] * s.dn[128][14]));
        let eq105_e2836_d_n15: f64 = ((s.dn[114][15] * s.v[128]) + (s.v[114] * s.dn[128][15]));
        let eq105_e2836_d_n16: f64 = ((s.dn[114][16] * s.v[128]) + (s.v[114] * s.dn[128][16]));
        let eq105_e2836_d_b0: f64 = ((s.db[114][0] * s.v[128]) + (s.v[114] * s.db[128][0]));
        let eq105_e2836_d_b1: f64 = ((s.db[114][1] * s.v[128]) + (s.v[114] * s.db[128][1]));
        let eq105_e2836_d_b2: f64 = ((s.db[114][2] * s.v[128]) + (s.v[114] * s.db[128][2]));
        let eq105_e2836_d_b3: f64 = ((s.db[114][3] * s.v[128]) + (s.v[114] * s.db[128][3]));
        let eq105_e2836_d_b4: f64 = ((s.db[114][4] * s.v[128]) + (s.v[114] * s.db[128][4]));
        let eq105_e2836_d_b5: f64 = ((s.db[114][5] * s.v[128]) + (s.v[114] * s.db[128][5]));
        let eq105_e2836_d_b6: f64 = ((s.db[114][6] * s.v[128]) + (s.v[114] * s.db[128][6]));
        let eq105_e2836_d_b7: f64 = ((s.db[114][7] * s.v[128]) + (s.v[114] * s.db[128][7]));
        let eq105_e2836_d_b8: f64 = ((s.db[114][8] * s.v[128]) + (s.v[114] * s.db[128][8]));
        let eq105_e2836_d_b9: f64 = ((s.db[114][9] * s.v[128]) + (s.v[114] * s.db[128][9]));
        let eq105_e2836_d_b10: f64 = ((s.db[114][10] * s.v[128]) + (s.v[114] * s.db[128][10]));
        let eq105_e2836_d_b11: f64 = ((s.db[114][11] * s.v[128]) + (s.v[114] * s.db[128][11]));
        let eq105_e2836_d_b12: f64 = ((s.db[114][12] * s.v[128]) + (s.v[114] * s.db[128][12]));
        let eq105_e2836_d_b13: f64 = ((s.db[114][13] * s.v[128]) + (s.v[114] * s.db[128][13]));
        let eq105_e2836_d_b14: f64 = ((s.db[114][14] * s.v[128]) + (s.v[114] * s.db[128][14]));
        let eq105_e2836_d_b15: f64 = ((s.db[114][15] * s.v[128]) + (s.v[114] * s.db[128][15]));
        let eq105_e2836_d_b16: f64 = ((s.db[114][16] * s.v[128]) + (s.v[114] * s.db[128][16]));
        let eq105_e2836_d_b17: f64 = ((s.db[114][17] * s.v[128]) + (s.v[114] * s.db[128][17]));
        let eq105_e2838: f64 = (eq105_e2836 * (nv5 - nv6));
        let eq105_e2838_d_n0: f64 = (eq105_e2836_d_n0 * (nv5 - nv6));
        let eq105_e2838_d_n1: f64 = (eq105_e2836_d_n1 * (nv5 - nv6));
        let eq105_e2838_d_n2: f64 = (eq105_e2836_d_n2 * (nv5 - nv6));
        let eq105_e2838_d_n3: f64 = (eq105_e2836_d_n3 * (nv5 - nv6));
        let eq105_e2838_d_n4: f64 = (eq105_e2836_d_n4 * (nv5 - nv6));
        let eq105_e2838_d_n5: f64 = ((eq105_e2836_d_n5 * (nv5 - nv6)) + eq105_e2836);
        let eq105_e2838_d_n6: f64 = ((eq105_e2836_d_n6 * (nv5 - nv6)) + (-eq105_e2836));
        let eq105_e2838_d_n7: f64 = (eq105_e2836_d_n7 * (nv5 - nv6));
        let eq105_e2838_d_n8: f64 = (eq105_e2836_d_n8 * (nv5 - nv6));
        let eq105_e2838_d_n9: f64 = (eq105_e2836_d_n9 * (nv5 - nv6));
        let eq105_e2838_d_n10: f64 = (eq105_e2836_d_n10 * (nv5 - nv6));
        let eq105_e2838_d_n11: f64 = (eq105_e2836_d_n11 * (nv5 - nv6));
        let eq105_e2838_d_n12: f64 = (eq105_e2836_d_n12 * (nv5 - nv6));
        let eq105_e2838_d_n13: f64 = (eq105_e2836_d_n13 * (nv5 - nv6));
        let eq105_e2838_d_n14: f64 = (eq105_e2836_d_n14 * (nv5 - nv6));
        let eq105_e2838_d_n15: f64 = (eq105_e2836_d_n15 * (nv5 - nv6));
        let eq105_e2838_d_n16: f64 = (eq105_e2836_d_n16 * (nv5 - nv6));
        let eq105_e2838_d_b0: f64 = (eq105_e2836_d_b0 * (nv5 - nv6));
        let eq105_e2838_d_b1: f64 = (eq105_e2836_d_b1 * (nv5 - nv6));
        let eq105_e2838_d_b2: f64 = (eq105_e2836_d_b2 * (nv5 - nv6));
        let eq105_e2838_d_b3: f64 = (eq105_e2836_d_b3 * (nv5 - nv6));
        let eq105_e2838_d_b4: f64 = (eq105_e2836_d_b4 * (nv5 - nv6));
        let eq105_e2838_d_b5: f64 = (eq105_e2836_d_b5 * (nv5 - nv6));
        let eq105_e2838_d_b6: f64 = (eq105_e2836_d_b6 * (nv5 - nv6));
        let eq105_e2838_d_b7: f64 = (eq105_e2836_d_b7 * (nv5 - nv6));
        let eq105_e2838_d_b8: f64 = (eq105_e2836_d_b8 * (nv5 - nv6));
        let eq105_e2838_d_b9: f64 = (eq105_e2836_d_b9 * (nv5 - nv6));
        let eq105_e2838_d_b10: f64 = (eq105_e2836_d_b10 * (nv5 - nv6));
        let eq105_e2838_d_b11: f64 = (eq105_e2836_d_b11 * (nv5 - nv6));
        let eq105_e2838_d_b12: f64 = (eq105_e2836_d_b12 * (nv5 - nv6));
        let eq105_e2838_d_b13: f64 = (eq105_e2836_d_b13 * (nv5 - nv6));
        let eq105_e2838_d_b14: f64 = (eq105_e2836_d_b14 * (nv5 - nv6));
        let eq105_e2838_d_b15: f64 = (eq105_e2836_d_b15 * (nv5 - nv6));
        let eq105_e2838_d_b16: f64 = (eq105_e2836_d_b16 * (nv5 - nv6));
        let eq105_e2838_d_b17: f64 = (eq105_e2836_d_b17 * (nv5 - nv6));
        let eq105_e2840: f64 = (eq105_e2838 * s.v[124]);
        let eq105_e2840_d_n0: f64 = ((eq105_e2838_d_n0 * s.v[124]) + (eq105_e2838 * s.dn[124][0]));
        let eq105_e2840_d_n1: f64 = ((eq105_e2838_d_n1 * s.v[124]) + (eq105_e2838 * s.dn[124][1]));
        let eq105_e2840_d_n2: f64 = ((eq105_e2838_d_n2 * s.v[124]) + (eq105_e2838 * s.dn[124][2]));
        let eq105_e2840_d_n3: f64 = ((eq105_e2838_d_n3 * s.v[124]) + (eq105_e2838 * s.dn[124][3]));
        let eq105_e2840_d_n4: f64 = ((eq105_e2838_d_n4 * s.v[124]) + (eq105_e2838 * s.dn[124][4]));
        let eq105_e2840_d_n5: f64 = ((eq105_e2838_d_n5 * s.v[124]) + (eq105_e2838 * s.dn[124][5]));
        let eq105_e2840_d_n6: f64 = ((eq105_e2838_d_n6 * s.v[124]) + (eq105_e2838 * s.dn[124][6]));
        let eq105_e2840_d_n7: f64 = ((eq105_e2838_d_n7 * s.v[124]) + (eq105_e2838 * s.dn[124][7]));
        let eq105_e2840_d_n8: f64 = ((eq105_e2838_d_n8 * s.v[124]) + (eq105_e2838 * s.dn[124][8]));
        let eq105_e2840_d_n9: f64 = ((eq105_e2838_d_n9 * s.v[124]) + (eq105_e2838 * s.dn[124][9]));
        let eq105_e2840_d_n10: f64 = ((eq105_e2838_d_n10 * s.v[124]) + (eq105_e2838 * s.dn[124][10]));
        let eq105_e2840_d_n11: f64 = ((eq105_e2838_d_n11 * s.v[124]) + (eq105_e2838 * s.dn[124][11]));
        let eq105_e2840_d_n12: f64 = ((eq105_e2838_d_n12 * s.v[124]) + (eq105_e2838 * s.dn[124][12]));
        let eq105_e2840_d_n13: f64 = ((eq105_e2838_d_n13 * s.v[124]) + (eq105_e2838 * s.dn[124][13]));
        let eq105_e2840_d_n14: f64 = ((eq105_e2838_d_n14 * s.v[124]) + (eq105_e2838 * s.dn[124][14]));
        let eq105_e2840_d_n15: f64 = ((eq105_e2838_d_n15 * s.v[124]) + (eq105_e2838 * s.dn[124][15]));
        let eq105_e2840_d_n16: f64 = ((eq105_e2838_d_n16 * s.v[124]) + (eq105_e2838 * s.dn[124][16]));
        let eq105_e2840_d_b0: f64 = ((eq105_e2838_d_b0 * s.v[124]) + (eq105_e2838 * s.db[124][0]));
        let eq105_e2840_d_b1: f64 = ((eq105_e2838_d_b1 * s.v[124]) + (eq105_e2838 * s.db[124][1]));
        let eq105_e2840_d_b2: f64 = ((eq105_e2838_d_b2 * s.v[124]) + (eq105_e2838 * s.db[124][2]));
        let eq105_e2840_d_b3: f64 = ((eq105_e2838_d_b3 * s.v[124]) + (eq105_e2838 * s.db[124][3]));
        let eq105_e2840_d_b4: f64 = ((eq105_e2838_d_b4 * s.v[124]) + (eq105_e2838 * s.db[124][4]));
        let eq105_e2840_d_b5: f64 = ((eq105_e2838_d_b5 * s.v[124]) + (eq105_e2838 * s.db[124][5]));
        let eq105_e2840_d_b6: f64 = ((eq105_e2838_d_b6 * s.v[124]) + (eq105_e2838 * s.db[124][6]));
        let eq105_e2840_d_b7: f64 = ((eq105_e2838_d_b7 * s.v[124]) + (eq105_e2838 * s.db[124][7]));
        let eq105_e2840_d_b8: f64 = ((eq105_e2838_d_b8 * s.v[124]) + (eq105_e2838 * s.db[124][8]));
        let eq105_e2840_d_b9: f64 = ((eq105_e2838_d_b9 * s.v[124]) + (eq105_e2838 * s.db[124][9]));
        let eq105_e2840_d_b10: f64 = ((eq105_e2838_d_b10 * s.v[124]) + (eq105_e2838 * s.db[124][10]));
        let eq105_e2840_d_b11: f64 = ((eq105_e2838_d_b11 * s.v[124]) + (eq105_e2838 * s.db[124][11]));
        let eq105_e2840_d_b12: f64 = ((eq105_e2838_d_b12 * s.v[124]) + (eq105_e2838 * s.db[124][12]));
        let eq105_e2840_d_b13: f64 = ((eq105_e2838_d_b13 * s.v[124]) + (eq105_e2838 * s.db[124][13]));
        let eq105_e2840_d_b14: f64 = ((eq105_e2838_d_b14 * s.v[124]) + (eq105_e2838 * s.db[124][14]));
        let eq105_e2840_d_b15: f64 = ((eq105_e2838_d_b15 * s.v[124]) + (eq105_e2838 * s.db[124][15]));
        let eq105_e2840_d_b16: f64 = ((eq105_e2838_d_b16 * s.v[124]) + (eq105_e2838 * s.db[124][16]));
        let eq105_e2840_d_b17: f64 = ((eq105_e2838_d_b17 * s.v[124]) + (eq105_e2838 * s.db[124][17]));
        let eq105_e2841: f64 = (-eq105_e2840);
        let eq105_e2841_d_n0: f64 = (-eq105_e2840_d_n0);
        let eq105_e2841_d_n1: f64 = (-eq105_e2840_d_n1);
        let eq105_e2841_d_n2: f64 = (-eq105_e2840_d_n2);
        let eq105_e2841_d_n3: f64 = (-eq105_e2840_d_n3);
        let eq105_e2841_d_n4: f64 = (-eq105_e2840_d_n4);
        let eq105_e2841_d_n5: f64 = (-eq105_e2840_d_n5);
        let eq105_e2841_d_n6: f64 = (-eq105_e2840_d_n6);
        let eq105_e2841_d_n7: f64 = (-eq105_e2840_d_n7);
        let eq105_e2841_d_n8: f64 = (-eq105_e2840_d_n8);
        let eq105_e2841_d_n9: f64 = (-eq105_e2840_d_n9);
        let eq105_e2841_d_n10: f64 = (-eq105_e2840_d_n10);
        let eq105_e2841_d_n11: f64 = (-eq105_e2840_d_n11);
        let eq105_e2841_d_n12: f64 = (-eq105_e2840_d_n12);
        let eq105_e2841_d_n13: f64 = (-eq105_e2840_d_n13);
        let eq105_e2841_d_n14: f64 = (-eq105_e2840_d_n14);
        let eq105_e2841_d_n15: f64 = (-eq105_e2840_d_n15);
        let eq105_e2841_d_n16: f64 = (-eq105_e2840_d_n16);
        let eq105_e2841_d_b0: f64 = (-eq105_e2840_d_b0);
        let eq105_e2841_d_b1: f64 = (-eq105_e2840_d_b1);
        let eq105_e2841_d_b2: f64 = (-eq105_e2840_d_b2);
        let eq105_e2841_d_b3: f64 = (-eq105_e2840_d_b3);
        let eq105_e2841_d_b4: f64 = (-eq105_e2840_d_b4);
        let eq105_e2841_d_b5: f64 = (-eq105_e2840_d_b5);
        let eq105_e2841_d_b6: f64 = (-eq105_e2840_d_b6);
        let eq105_e2841_d_b7: f64 = (-eq105_e2840_d_b7);
        let eq105_e2841_d_b8: f64 = (-eq105_e2840_d_b8);
        let eq105_e2841_d_b9: f64 = (-eq105_e2840_d_b9);
        let eq105_e2841_d_b10: f64 = (-eq105_e2840_d_b10);
        let eq105_e2841_d_b11: f64 = (-eq105_e2840_d_b11);
        let eq105_e2841_d_b12: f64 = (-eq105_e2840_d_b12);
        let eq105_e2841_d_b13: f64 = (-eq105_e2840_d_b13);
        let eq105_e2841_d_b14: f64 = (-eq105_e2840_d_b14);
        let eq105_e2841_d_b15: f64 = (-eq105_e2840_d_b15);
        let eq105_e2841_d_b16: f64 = (-eq105_e2840_d_b16);
        let eq105_e2841_d_b17: f64 = (-eq105_e2840_d_b17);
        (eq105_e2841, eq105_e2841_d_n0, eq105_e2841_d_n1, eq105_e2841_d_n2, eq105_e2841_d_n3, eq105_e2841_d_n4, eq105_e2841_d_n5, eq105_e2841_d_n6, eq105_e2841_d_n7, eq105_e2841_d_n8, eq105_e2841_d_n9, eq105_e2841_d_n10, eq105_e2841_d_n11, eq105_e2841_d_n12, eq105_e2841_d_n13, eq105_e2841_d_n14, eq105_e2841_d_n15, eq105_e2841_d_n16, eq105_e2841_d_b0, eq105_e2841_d_b1, eq105_e2841_d_b2, eq105_e2841_d_b3, eq105_e2841_d_b4, eq105_e2841_d_b5, eq105_e2841_d_b6, eq105_e2841_d_b7, eq105_e2841_d_b8, eq105_e2841_d_b9, eq105_e2841_d_b10, eq105_e2841_d_b11, eq105_e2841_d_b12, eq105_e2841_d_b13, eq105_e2841_d_b14, eq105_e2841_d_b15, eq105_e2841_d_b16, eq105_e2841_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e2843;
        let eq105_node_derivatives: [f64; 17] = [eq105_e2843_d_n0, eq105_e2843_d_n1, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n12, eq105_e2843_d_n13, eq105_e2843_d_n14, eq105_e2843_d_n15, eq105_e2843_d_n16];
        let eq105_branch_derivatives: [f64; 18] = [eq105_e2843_d_b0, eq105_e2843_d_b1, eq105_e2843_d_b2, eq105_e2843_d_b3, eq105_e2843_d_b4, eq105_e2843_d_b5, eq105_e2843_d_b6, eq105_e2843_d_b7, eq105_e2843_d_b8, eq105_e2843_d_b9, eq105_e2843_d_b10, eq105_e2843_d_b11, eq105_e2843_d_b12, eq105_e2843_d_b13, eq105_e2843_d_b14, eq105_e2843_d_b15, eq105_e2843_d_b16, eq105_e2843_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq105_value),
            nodes,
            &eq105_node_derivatives,
            branches,
            &eq105_branch_derivatives,
            multiplicity,
        );
        let (eq106_e2854, eq106_e2854_d_n0, eq106_e2854_d_n1, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n12, eq106_e2854_d_n13, eq106_e2854_d_n14, eq106_e2854_d_n15, eq106_e2854_d_n16, eq106_e2854_d_b0, eq106_e2854_d_b1, eq106_e2854_d_b2, eq106_e2854_d_b3, eq106_e2854_d_b4, eq106_e2854_d_b5, eq106_e2854_d_b6, eq106_e2854_d_b7, eq106_e2854_d_b8, eq106_e2854_d_b9, eq106_e2854_d_b10, eq106_e2854_d_b11, eq106_e2854_d_b12, eq106_e2854_d_b13, eq106_e2854_d_b14, eq106_e2854_d_b15, eq106_e2854_d_b16, eq106_e2854_d_b17,) = {
    if (s.b[1736] && s.b[1737]) {
        let eq106_e2848: f64 = (-(nv0 - nv9));
        let eq106_e2848_d_n0: f64 = (-1.0);
        let eq106_e2850: f64 = (eq106_e2848 * (nv0 - nv9));
        let eq106_e2850_d_n0: f64 = ((eq106_e2848_d_n0 * (nv0 - nv9)) + eq106_e2848);
        let eq106_e2850_d_n9: f64 = ((nv0 - nv9) + (-eq106_e2848));
        let eq106_e2852: f64 = (eq106_e2850 * s.v[596]);
        let eq106_e2852_d_n0: f64 = ((eq106_e2850_d_n0 * s.v[596]) + (eq106_e2850 * s.dn[596][0]));
        let eq106_e2852_d_n1: f64 = (eq106_e2850 * s.dn[596][1]);
        let eq106_e2852_d_n2: f64 = (eq106_e2850 * s.dn[596][2]);
        let eq106_e2852_d_n3: f64 = (eq106_e2850 * s.dn[596][3]);
        let eq106_e2852_d_n4: f64 = (eq106_e2850 * s.dn[596][4]);
        let eq106_e2852_d_n5: f64 = (eq106_e2850 * s.dn[596][5]);
        let eq106_e2852_d_n6: f64 = (eq106_e2850 * s.dn[596][6]);
        let eq106_e2852_d_n7: f64 = (eq106_e2850 * s.dn[596][7]);
        let eq106_e2852_d_n8: f64 = (eq106_e2850 * s.dn[596][8]);
        let eq106_e2852_d_n9: f64 = ((eq106_e2850_d_n9 * s.v[596]) + (eq106_e2850 * s.dn[596][9]));
        let eq106_e2852_d_n10: f64 = (eq106_e2850 * s.dn[596][10]);
        let eq106_e2852_d_n11: f64 = (eq106_e2850 * s.dn[596][11]);
        let eq106_e2852_d_n12: f64 = (eq106_e2850 * s.dn[596][12]);
        let eq106_e2852_d_n13: f64 = (eq106_e2850 * s.dn[596][13]);
        let eq106_e2852_d_n14: f64 = (eq106_e2850 * s.dn[596][14]);
        let eq106_e2852_d_n15: f64 = (eq106_e2850 * s.dn[596][15]);
        let eq106_e2852_d_n16: f64 = (eq106_e2850 * s.dn[596][16]);
        let eq106_e2852_d_b0: f64 = (eq106_e2850 * s.db[596][0]);
        let eq106_e2852_d_b1: f64 = (eq106_e2850 * s.db[596][1]);
        let eq106_e2852_d_b2: f64 = (eq106_e2850 * s.db[596][2]);
        let eq106_e2852_d_b3: f64 = (eq106_e2850 * s.db[596][3]);
        let eq106_e2852_d_b4: f64 = (eq106_e2850 * s.db[596][4]);
        let eq106_e2852_d_b5: f64 = (eq106_e2850 * s.db[596][5]);
        let eq106_e2852_d_b6: f64 = (eq106_e2850 * s.db[596][6]);
        let eq106_e2852_d_b7: f64 = (eq106_e2850 * s.db[596][7]);
        let eq106_e2852_d_b8: f64 = (eq106_e2850 * s.db[596][8]);
        let eq106_e2852_d_b9: f64 = (eq106_e2850 * s.db[596][9]);
        let eq106_e2852_d_b10: f64 = (eq106_e2850 * s.db[596][10]);
        let eq106_e2852_d_b11: f64 = (eq106_e2850 * s.db[596][11]);
        let eq106_e2852_d_b12: f64 = (eq106_e2850 * s.db[596][12]);
        let eq106_e2852_d_b13: f64 = (eq106_e2850 * s.db[596][13]);
        let eq106_e2852_d_b14: f64 = (eq106_e2850 * s.db[596][14]);
        let eq106_e2852_d_b15: f64 = (eq106_e2850 * s.db[596][15]);
        let eq106_e2852_d_b16: f64 = (eq106_e2850 * s.db[596][16]);
        let eq106_e2852_d_b17: f64 = (eq106_e2850 * s.db[596][17]);
        (eq106_e2852, eq106_e2852_d_n0, eq106_e2852_d_n1, eq106_e2852_d_n2, eq106_e2852_d_n3, eq106_e2852_d_n4, eq106_e2852_d_n5, eq106_e2852_d_n6, eq106_e2852_d_n7, eq106_e2852_d_n8, eq106_e2852_d_n9, eq106_e2852_d_n10, eq106_e2852_d_n11, eq106_e2852_d_n12, eq106_e2852_d_n13, eq106_e2852_d_n14, eq106_e2852_d_n15, eq106_e2852_d_n16, eq106_e2852_d_b0, eq106_e2852_d_b1, eq106_e2852_d_b2, eq106_e2852_d_b3, eq106_e2852_d_b4, eq106_e2852_d_b5, eq106_e2852_d_b6, eq106_e2852_d_b7, eq106_e2852_d_b8, eq106_e2852_d_b9, eq106_e2852_d_b10, eq106_e2852_d_b11, eq106_e2852_d_b12, eq106_e2852_d_b13, eq106_e2852_d_b14, eq106_e2852_d_b15, eq106_e2852_d_b16, eq106_e2852_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq106_value: f64 = eq106_e2854;
        let eq106_node_derivatives: [f64; 17] = [eq106_e2854_d_n0, eq106_e2854_d_n1, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n12, eq106_e2854_d_n13, eq106_e2854_d_n14, eq106_e2854_d_n15, eq106_e2854_d_n16];
        let eq106_branch_derivatives: [f64; 18] = [eq106_e2854_d_b0, eq106_e2854_d_b1, eq106_e2854_d_b2, eq106_e2854_d_b3, eq106_e2854_d_b4, eq106_e2854_d_b5, eq106_e2854_d_b6, eq106_e2854_d_b7, eq106_e2854_d_b8, eq106_e2854_d_b9, eq106_e2854_d_b10, eq106_e2854_d_b11, eq106_e2854_d_b12, eq106_e2854_d_b13, eq106_e2854_d_b14, eq106_e2854_d_b15, eq106_e2854_d_b16, eq106_e2854_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq106_value),
            nodes,
            &eq106_node_derivatives,
            branches,
            &eq106_branch_derivatives,
            multiplicity,
        );
        let (eq107_e2867, eq107_e2867_d_n0, eq107_e2867_d_n1, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n12, eq107_e2867_d_n13, eq107_e2867_d_n14, eq107_e2867_d_n15, eq107_e2867_d_n16, eq107_e2867_d_b0, eq107_e2867_d_b1, eq107_e2867_d_b2, eq107_e2867_d_b3, eq107_e2867_d_b4, eq107_e2867_d_b5, eq107_e2867_d_b6, eq107_e2867_d_b7, eq107_e2867_d_b8, eq107_e2867_d_b9, eq107_e2867_d_b10, eq107_e2867_d_b11, eq107_e2867_d_b12, eq107_e2867_d_b13, eq107_e2867_d_b14, eq107_e2867_d_b15, eq107_e2867_d_b16, eq107_e2867_d_b17,) = {
    if ((s.b[1736] && s.b[1737]) && s.b[1738]) {
        let eq107_e2861: f64 = (-(nv9 - nv7));
        let eq107_e2861_d_n9: f64 = (-1.0);
        let eq107_e2863: f64 = (eq107_e2861 * (nv9 - nv7));
        let eq107_e2863_d_n7: f64 = ((nv9 - nv7) + (-eq107_e2861));
        let eq107_e2863_d_n9: f64 = ((eq107_e2861_d_n9 * (nv9 - nv7)) + eq107_e2861);
        let eq107_e2865: f64 = (eq107_e2863 * s.v[1042]);
        let eq107_e2865_d_n0: f64 = (eq107_e2863 * s.dn[1042][0]);
        let eq107_e2865_d_n1: f64 = (eq107_e2863 * s.dn[1042][1]);
        let eq107_e2865_d_n2: f64 = (eq107_e2863 * s.dn[1042][2]);
        let eq107_e2865_d_n3: f64 = (eq107_e2863 * s.dn[1042][3]);
        let eq107_e2865_d_n4: f64 = (eq107_e2863 * s.dn[1042][4]);
        let eq107_e2865_d_n5: f64 = (eq107_e2863 * s.dn[1042][5]);
        let eq107_e2865_d_n6: f64 = (eq107_e2863 * s.dn[1042][6]);
        let eq107_e2865_d_n7: f64 = ((eq107_e2863_d_n7 * s.v[1042]) + (eq107_e2863 * s.dn[1042][7]));
        let eq107_e2865_d_n8: f64 = (eq107_e2863 * s.dn[1042][8]);
        let eq107_e2865_d_n9: f64 = ((eq107_e2863_d_n9 * s.v[1042]) + (eq107_e2863 * s.dn[1042][9]));
        let eq107_e2865_d_n10: f64 = (eq107_e2863 * s.dn[1042][10]);
        let eq107_e2865_d_n11: f64 = (eq107_e2863 * s.dn[1042][11]);
        let eq107_e2865_d_n12: f64 = (eq107_e2863 * s.dn[1042][12]);
        let eq107_e2865_d_n13: f64 = (eq107_e2863 * s.dn[1042][13]);
        let eq107_e2865_d_n14: f64 = (eq107_e2863 * s.dn[1042][14]);
        let eq107_e2865_d_n15: f64 = (eq107_e2863 * s.dn[1042][15]);
        let eq107_e2865_d_n16: f64 = (eq107_e2863 * s.dn[1042][16]);
        let eq107_e2865_d_b0: f64 = (eq107_e2863 * s.db[1042][0]);
        let eq107_e2865_d_b1: f64 = (eq107_e2863 * s.db[1042][1]);
        let eq107_e2865_d_b2: f64 = (eq107_e2863 * s.db[1042][2]);
        let eq107_e2865_d_b3: f64 = (eq107_e2863 * s.db[1042][3]);
        let eq107_e2865_d_b4: f64 = (eq107_e2863 * s.db[1042][4]);
        let eq107_e2865_d_b5: f64 = (eq107_e2863 * s.db[1042][5]);
        let eq107_e2865_d_b6: f64 = (eq107_e2863 * s.db[1042][6]);
        let eq107_e2865_d_b7: f64 = (eq107_e2863 * s.db[1042][7]);
        let eq107_e2865_d_b8: f64 = (eq107_e2863 * s.db[1042][8]);
        let eq107_e2865_d_b9: f64 = (eq107_e2863 * s.db[1042][9]);
        let eq107_e2865_d_b10: f64 = (eq107_e2863 * s.db[1042][10]);
        let eq107_e2865_d_b11: f64 = (eq107_e2863 * s.db[1042][11]);
        let eq107_e2865_d_b12: f64 = (eq107_e2863 * s.db[1042][12]);
        let eq107_e2865_d_b13: f64 = (eq107_e2863 * s.db[1042][13]);
        let eq107_e2865_d_b14: f64 = (eq107_e2863 * s.db[1042][14]);
        let eq107_e2865_d_b15: f64 = (eq107_e2863 * s.db[1042][15]);
        let eq107_e2865_d_b16: f64 = (eq107_e2863 * s.db[1042][16]);
        let eq107_e2865_d_b17: f64 = (eq107_e2863 * s.db[1042][17]);
        (eq107_e2865, eq107_e2865_d_n0, eq107_e2865_d_n1, eq107_e2865_d_n2, eq107_e2865_d_n3, eq107_e2865_d_n4, eq107_e2865_d_n5, eq107_e2865_d_n6, eq107_e2865_d_n7, eq107_e2865_d_n8, eq107_e2865_d_n9, eq107_e2865_d_n10, eq107_e2865_d_n11, eq107_e2865_d_n12, eq107_e2865_d_n13, eq107_e2865_d_n14, eq107_e2865_d_n15, eq107_e2865_d_n16, eq107_e2865_d_b0, eq107_e2865_d_b1, eq107_e2865_d_b2, eq107_e2865_d_b3, eq107_e2865_d_b4, eq107_e2865_d_b5, eq107_e2865_d_b6, eq107_e2865_d_b7, eq107_e2865_d_b8, eq107_e2865_d_b9, eq107_e2865_d_b10, eq107_e2865_d_b11, eq107_e2865_d_b12, eq107_e2865_d_b13, eq107_e2865_d_b14, eq107_e2865_d_b15, eq107_e2865_d_b16, eq107_e2865_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq107_value: f64 = eq107_e2867;
        let eq107_node_derivatives: [f64; 17] = [eq107_e2867_d_n0, eq107_e2867_d_n1, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n12, eq107_e2867_d_n13, eq107_e2867_d_n14, eq107_e2867_d_n15, eq107_e2867_d_n16];
        let eq107_branch_derivatives: [f64; 18] = [eq107_e2867_d_b0, eq107_e2867_d_b1, eq107_e2867_d_b2, eq107_e2867_d_b3, eq107_e2867_d_b4, eq107_e2867_d_b5, eq107_e2867_d_b6, eq107_e2867_d_b7, eq107_e2867_d_b8, eq107_e2867_d_b9, eq107_e2867_d_b10, eq107_e2867_d_b11, eq107_e2867_d_b12, eq107_e2867_d_b13, eq107_e2867_d_b14, eq107_e2867_d_b15, eq107_e2867_d_b16, eq107_e2867_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq107_value),
            nodes,
            &eq107_node_derivatives,
            branches,
            &eq107_branch_derivatives,
            multiplicity,
        );
        let (eq108_e2878, eq108_e2878_d_n0, eq108_e2878_d_n1, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n12, eq108_e2878_d_n13, eq108_e2878_d_n14, eq108_e2878_d_n15, eq108_e2878_d_n16, eq108_e2878_d_b0, eq108_e2878_d_b1, eq108_e2878_d_b2, eq108_e2878_d_b3, eq108_e2878_d_b4, eq108_e2878_d_b5, eq108_e2878_d_b6, eq108_e2878_d_b7, eq108_e2878_d_b8, eq108_e2878_d_b9, eq108_e2878_d_b10, eq108_e2878_d_b11, eq108_e2878_d_b12, eq108_e2878_d_b13, eq108_e2878_d_b14, eq108_e2878_d_b15, eq108_e2878_d_b16, eq108_e2878_d_b17,) = {
    if (s.b[1736] && s.b[1739]) {
        let eq108_e2872: f64 = (-(nv2 - nv8));
        let eq108_e2872_d_n2: f64 = (-1.0);
        let eq108_e2874: f64 = (eq108_e2872 * (nv2 - nv8));
        let eq108_e2874_d_n2: f64 = ((eq108_e2872_d_n2 * (nv2 - nv8)) + eq108_e2872);
        let eq108_e2874_d_n8: f64 = ((nv2 - nv8) + (-eq108_e2872));
        let eq108_e2876: f64 = (eq108_e2874 * s.v[595]);
        let eq108_e2876_d_n0: f64 = (eq108_e2874 * s.dn[595][0]);
        let eq108_e2876_d_n1: f64 = (eq108_e2874 * s.dn[595][1]);
        let eq108_e2876_d_n2: f64 = ((eq108_e2874_d_n2 * s.v[595]) + (eq108_e2874 * s.dn[595][2]));
        let eq108_e2876_d_n3: f64 = (eq108_e2874 * s.dn[595][3]);
        let eq108_e2876_d_n4: f64 = (eq108_e2874 * s.dn[595][4]);
        let eq108_e2876_d_n5: f64 = (eq108_e2874 * s.dn[595][5]);
        let eq108_e2876_d_n6: f64 = (eq108_e2874 * s.dn[595][6]);
        let eq108_e2876_d_n7: f64 = (eq108_e2874 * s.dn[595][7]);
        let eq108_e2876_d_n8: f64 = ((eq108_e2874_d_n8 * s.v[595]) + (eq108_e2874 * s.dn[595][8]));
        let eq108_e2876_d_n9: f64 = (eq108_e2874 * s.dn[595][9]);
        let eq108_e2876_d_n10: f64 = (eq108_e2874 * s.dn[595][10]);
        let eq108_e2876_d_n11: f64 = (eq108_e2874 * s.dn[595][11]);
        let eq108_e2876_d_n12: f64 = (eq108_e2874 * s.dn[595][12]);
        let eq108_e2876_d_n13: f64 = (eq108_e2874 * s.dn[595][13]);
        let eq108_e2876_d_n14: f64 = (eq108_e2874 * s.dn[595][14]);
        let eq108_e2876_d_n15: f64 = (eq108_e2874 * s.dn[595][15]);
        let eq108_e2876_d_n16: f64 = (eq108_e2874 * s.dn[595][16]);
        let eq108_e2876_d_b0: f64 = (eq108_e2874 * s.db[595][0]);
        let eq108_e2876_d_b1: f64 = (eq108_e2874 * s.db[595][1]);
        let eq108_e2876_d_b2: f64 = (eq108_e2874 * s.db[595][2]);
        let eq108_e2876_d_b3: f64 = (eq108_e2874 * s.db[595][3]);
        let eq108_e2876_d_b4: f64 = (eq108_e2874 * s.db[595][4]);
        let eq108_e2876_d_b5: f64 = (eq108_e2874 * s.db[595][5]);
        let eq108_e2876_d_b6: f64 = (eq108_e2874 * s.db[595][6]);
        let eq108_e2876_d_b7: f64 = (eq108_e2874 * s.db[595][7]);
        let eq108_e2876_d_b8: f64 = (eq108_e2874 * s.db[595][8]);
        let eq108_e2876_d_b9: f64 = (eq108_e2874 * s.db[595][9]);
        let eq108_e2876_d_b10: f64 = (eq108_e2874 * s.db[595][10]);
        let eq108_e2876_d_b11: f64 = (eq108_e2874 * s.db[595][11]);
        let eq108_e2876_d_b12: f64 = (eq108_e2874 * s.db[595][12]);
        let eq108_e2876_d_b13: f64 = (eq108_e2874 * s.db[595][13]);
        let eq108_e2876_d_b14: f64 = (eq108_e2874 * s.db[595][14]);
        let eq108_e2876_d_b15: f64 = (eq108_e2874 * s.db[595][15]);
        let eq108_e2876_d_b16: f64 = (eq108_e2874 * s.db[595][16]);
        let eq108_e2876_d_b17: f64 = (eq108_e2874 * s.db[595][17]);
        (eq108_e2876, eq108_e2876_d_n0, eq108_e2876_d_n1, eq108_e2876_d_n2, eq108_e2876_d_n3, eq108_e2876_d_n4, eq108_e2876_d_n5, eq108_e2876_d_n6, eq108_e2876_d_n7, eq108_e2876_d_n8, eq108_e2876_d_n9, eq108_e2876_d_n10, eq108_e2876_d_n11, eq108_e2876_d_n12, eq108_e2876_d_n13, eq108_e2876_d_n14, eq108_e2876_d_n15, eq108_e2876_d_n16, eq108_e2876_d_b0, eq108_e2876_d_b1, eq108_e2876_d_b2, eq108_e2876_d_b3, eq108_e2876_d_b4, eq108_e2876_d_b5, eq108_e2876_d_b6, eq108_e2876_d_b7, eq108_e2876_d_b8, eq108_e2876_d_b9, eq108_e2876_d_b10, eq108_e2876_d_b11, eq108_e2876_d_b12, eq108_e2876_d_b13, eq108_e2876_d_b14, eq108_e2876_d_b15, eq108_e2876_d_b16, eq108_e2876_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq108_value: f64 = eq108_e2878;
        let eq108_node_derivatives: [f64; 17] = [eq108_e2878_d_n0, eq108_e2878_d_n1, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n12, eq108_e2878_d_n13, eq108_e2878_d_n14, eq108_e2878_d_n15, eq108_e2878_d_n16];
        let eq108_branch_derivatives: [f64; 18] = [eq108_e2878_d_b0, eq108_e2878_d_b1, eq108_e2878_d_b2, eq108_e2878_d_b3, eq108_e2878_d_b4, eq108_e2878_d_b5, eq108_e2878_d_b6, eq108_e2878_d_b7, eq108_e2878_d_b8, eq108_e2878_d_b9, eq108_e2878_d_b10, eq108_e2878_d_b11, eq108_e2878_d_b12, eq108_e2878_d_b13, eq108_e2878_d_b14, eq108_e2878_d_b15, eq108_e2878_d_b16, eq108_e2878_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq108_value),
            nodes,
            &eq108_node_derivatives,
            branches,
            &eq108_branch_derivatives,
            multiplicity,
        );
        let (eq109_e2891, eq109_e2891_d_n0, eq109_e2891_d_n1, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n12, eq109_e2891_d_n13, eq109_e2891_d_n14, eq109_e2891_d_n15, eq109_e2891_d_n16, eq109_e2891_d_b0, eq109_e2891_d_b1, eq109_e2891_d_b2, eq109_e2891_d_b3, eq109_e2891_d_b4, eq109_e2891_d_b5, eq109_e2891_d_b6, eq109_e2891_d_b7, eq109_e2891_d_b8, eq109_e2891_d_b9, eq109_e2891_d_b10, eq109_e2891_d_b11, eq109_e2891_d_b12, eq109_e2891_d_b13, eq109_e2891_d_b14, eq109_e2891_d_b15, eq109_e2891_d_b16, eq109_e2891_d_b17,) = {
    if ((s.b[1736] && s.b[1739]) && s.b[1740]) {
        let eq109_e2885: f64 = (-(nv8 - nv6));
        let eq109_e2885_d_n8: f64 = (-1.0);
        let eq109_e2887: f64 = (eq109_e2885 * (nv8 - nv6));
        let eq109_e2887_d_n6: f64 = ((nv8 - nv6) + (-eq109_e2885));
        let eq109_e2887_d_n8: f64 = ((eq109_e2885_d_n8 * (nv8 - nv6)) + eq109_e2885);
        let eq109_e2889: f64 = (eq109_e2887 * s.v[1043]);
        let eq109_e2889_d_n0: f64 = (eq109_e2887 * s.dn[1043][0]);
        let eq109_e2889_d_n1: f64 = (eq109_e2887 * s.dn[1043][1]);
        let eq109_e2889_d_n2: f64 = (eq109_e2887 * s.dn[1043][2]);
        let eq109_e2889_d_n3: f64 = (eq109_e2887 * s.dn[1043][3]);
        let eq109_e2889_d_n4: f64 = (eq109_e2887 * s.dn[1043][4]);
        let eq109_e2889_d_n5: f64 = (eq109_e2887 * s.dn[1043][5]);
        let eq109_e2889_d_n6: f64 = ((eq109_e2887_d_n6 * s.v[1043]) + (eq109_e2887 * s.dn[1043][6]));
        let eq109_e2889_d_n7: f64 = (eq109_e2887 * s.dn[1043][7]);
        let eq109_e2889_d_n8: f64 = ((eq109_e2887_d_n8 * s.v[1043]) + (eq109_e2887 * s.dn[1043][8]));
        let eq109_e2889_d_n9: f64 = (eq109_e2887 * s.dn[1043][9]);
        let eq109_e2889_d_n10: f64 = (eq109_e2887 * s.dn[1043][10]);
        let eq109_e2889_d_n11: f64 = (eq109_e2887 * s.dn[1043][11]);
        let eq109_e2889_d_n12: f64 = (eq109_e2887 * s.dn[1043][12]);
        let eq109_e2889_d_n13: f64 = (eq109_e2887 * s.dn[1043][13]);
        let eq109_e2889_d_n14: f64 = (eq109_e2887 * s.dn[1043][14]);
        let eq109_e2889_d_n15: f64 = (eq109_e2887 * s.dn[1043][15]);
        let eq109_e2889_d_n16: f64 = (eq109_e2887 * s.dn[1043][16]);
        let eq109_e2889_d_b0: f64 = (eq109_e2887 * s.db[1043][0]);
        let eq109_e2889_d_b1: f64 = (eq109_e2887 * s.db[1043][1]);
        let eq109_e2889_d_b2: f64 = (eq109_e2887 * s.db[1043][2]);
        let eq109_e2889_d_b3: f64 = (eq109_e2887 * s.db[1043][3]);
        let eq109_e2889_d_b4: f64 = (eq109_e2887 * s.db[1043][4]);
        let eq109_e2889_d_b5: f64 = (eq109_e2887 * s.db[1043][5]);
        let eq109_e2889_d_b6: f64 = (eq109_e2887 * s.db[1043][6]);
        let eq109_e2889_d_b7: f64 = (eq109_e2887 * s.db[1043][7]);
        let eq109_e2889_d_b8: f64 = (eq109_e2887 * s.db[1043][8]);
        let eq109_e2889_d_b9: f64 = (eq109_e2887 * s.db[1043][9]);
        let eq109_e2889_d_b10: f64 = (eq109_e2887 * s.db[1043][10]);
        let eq109_e2889_d_b11: f64 = (eq109_e2887 * s.db[1043][11]);
        let eq109_e2889_d_b12: f64 = (eq109_e2887 * s.db[1043][12]);
        let eq109_e2889_d_b13: f64 = (eq109_e2887 * s.db[1043][13]);
        let eq109_e2889_d_b14: f64 = (eq109_e2887 * s.db[1043][14]);
        let eq109_e2889_d_b15: f64 = (eq109_e2887 * s.db[1043][15]);
        let eq109_e2889_d_b16: f64 = (eq109_e2887 * s.db[1043][16]);
        let eq109_e2889_d_b17: f64 = (eq109_e2887 * s.db[1043][17]);
        (eq109_e2889, eq109_e2889_d_n0, eq109_e2889_d_n1, eq109_e2889_d_n2, eq109_e2889_d_n3, eq109_e2889_d_n4, eq109_e2889_d_n5, eq109_e2889_d_n6, eq109_e2889_d_n7, eq109_e2889_d_n8, eq109_e2889_d_n9, eq109_e2889_d_n10, eq109_e2889_d_n11, eq109_e2889_d_n12, eq109_e2889_d_n13, eq109_e2889_d_n14, eq109_e2889_d_n15, eq109_e2889_d_n16, eq109_e2889_d_b0, eq109_e2889_d_b1, eq109_e2889_d_b2, eq109_e2889_d_b3, eq109_e2889_d_b4, eq109_e2889_d_b5, eq109_e2889_d_b6, eq109_e2889_d_b7, eq109_e2889_d_b8, eq109_e2889_d_b9, eq109_e2889_d_b10, eq109_e2889_d_b11, eq109_e2889_d_b12, eq109_e2889_d_b13, eq109_e2889_d_b14, eq109_e2889_d_b15, eq109_e2889_d_b16, eq109_e2889_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e2891;
        let eq109_node_derivatives: [f64; 17] = [eq109_e2891_d_n0, eq109_e2891_d_n1, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n12, eq109_e2891_d_n13, eq109_e2891_d_n14, eq109_e2891_d_n15, eq109_e2891_d_n16];
        let eq109_branch_derivatives: [f64; 18] = [eq109_e2891_d_b0, eq109_e2891_d_b1, eq109_e2891_d_b2, eq109_e2891_d_b3, eq109_e2891_d_b4, eq109_e2891_d_b5, eq109_e2891_d_b6, eq109_e2891_d_b7, eq109_e2891_d_b8, eq109_e2891_d_b9, eq109_e2891_d_b10, eq109_e2891_d_b11, eq109_e2891_d_b12, eq109_e2891_d_b13, eq109_e2891_d_b14, eq109_e2891_d_b15, eq109_e2891_d_b16, eq109_e2891_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq109_value),
            nodes,
            &eq109_node_derivatives,
            branches,
            &eq109_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq110_e2897, eq110_e2897_d_n0, eq110_e2897_d_n1, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n12, eq110_e2897_d_n13, eq110_e2897_d_n14, eq110_e2897_d_n15, eq110_e2897_d_n16, eq110_e2897_d_b0, eq110_e2897_d_b1, eq110_e2897_d_b2, eq110_e2897_d_b3, eq110_e2897_d_b4, eq110_e2897_d_b5, eq110_e2897_d_b6, eq110_e2897_d_b7, eq110_e2897_d_b8, eq110_e2897_d_b9, eq110_e2897_d_b10, eq110_e2897_d_b11, eq110_e2897_d_b12, eq110_e2897_d_b13, eq110_e2897_d_b14, eq110_e2897_d_b15, eq110_e2897_d_b16, eq110_e2897_d_b17,) = {
    if s.b[1736] {
        let eq110_e2895: f64 = ((nv4 - 0.0) * s.v[633]);
        let eq110_e2895_d_n0: f64 = ((nv4 - 0.0) * s.dn[633][0]);
        let eq110_e2895_d_n1: f64 = ((nv4 - 0.0) * s.dn[633][1]);
        let eq110_e2895_d_n2: f64 = ((nv4 - 0.0) * s.dn[633][2]);
        let eq110_e2895_d_n3: f64 = ((nv4 - 0.0) * s.dn[633][3]);
        let eq110_e2895_d_n4: f64 = (s.v[633] + ((nv4 - 0.0) * s.dn[633][4]));
        let eq110_e2895_d_n5: f64 = ((nv4 - 0.0) * s.dn[633][5]);
        let eq110_e2895_d_n6: f64 = ((nv4 - 0.0) * s.dn[633][6]);
        let eq110_e2895_d_n7: f64 = ((nv4 - 0.0) * s.dn[633][7]);
        let eq110_e2895_d_n8: f64 = ((nv4 - 0.0) * s.dn[633][8]);
        let eq110_e2895_d_n9: f64 = ((nv4 - 0.0) * s.dn[633][9]);
        let eq110_e2895_d_n10: f64 = ((nv4 - 0.0) * s.dn[633][10]);
        let eq110_e2895_d_n11: f64 = ((nv4 - 0.0) * s.dn[633][11]);
        let eq110_e2895_d_n12: f64 = ((nv4 - 0.0) * s.dn[633][12]);
        let eq110_e2895_d_n13: f64 = ((nv4 - 0.0) * s.dn[633][13]);
        let eq110_e2895_d_n14: f64 = ((nv4 - 0.0) * s.dn[633][14]);
        let eq110_e2895_d_n15: f64 = ((nv4 - 0.0) * s.dn[633][15]);
        let eq110_e2895_d_n16: f64 = ((nv4 - 0.0) * s.dn[633][16]);
        let eq110_e2895_d_b0: f64 = ((nv4 - 0.0) * s.db[633][0]);
        let eq110_e2895_d_b1: f64 = ((nv4 - 0.0) * s.db[633][1]);
        let eq110_e2895_d_b2: f64 = ((nv4 - 0.0) * s.db[633][2]);
        let eq110_e2895_d_b3: f64 = ((nv4 - 0.0) * s.db[633][3]);
        let eq110_e2895_d_b4: f64 = ((nv4 - 0.0) * s.db[633][4]);
        let eq110_e2895_d_b5: f64 = ((nv4 - 0.0) * s.db[633][5]);
        let eq110_e2895_d_b6: f64 = ((nv4 - 0.0) * s.db[633][6]);
        let eq110_e2895_d_b7: f64 = ((nv4 - 0.0) * s.db[633][7]);
        let eq110_e2895_d_b8: f64 = ((nv4 - 0.0) * s.db[633][8]);
        let eq110_e2895_d_b9: f64 = ((nv4 - 0.0) * s.db[633][9]);
        let eq110_e2895_d_b10: f64 = ((nv4 - 0.0) * s.db[633][10]);
        let eq110_e2895_d_b11: f64 = ((nv4 - 0.0) * s.db[633][11]);
        let eq110_e2895_d_b12: f64 = ((nv4 - 0.0) * s.db[633][12]);
        let eq110_e2895_d_b13: f64 = ((nv4 - 0.0) * s.db[633][13]);
        let eq110_e2895_d_b14: f64 = ((nv4 - 0.0) * s.db[633][14]);
        let eq110_e2895_d_b15: f64 = ((nv4 - 0.0) * s.db[633][15]);
        let eq110_e2895_d_b16: f64 = ((nv4 - 0.0) * s.db[633][16]);
        let eq110_e2895_d_b17: f64 = ((nv4 - 0.0) * s.db[633][17]);
        (eq110_e2895, eq110_e2895_d_n0, eq110_e2895_d_n1, eq110_e2895_d_n2, eq110_e2895_d_n3, eq110_e2895_d_n4, eq110_e2895_d_n5, eq110_e2895_d_n6, eq110_e2895_d_n7, eq110_e2895_d_n8, eq110_e2895_d_n9, eq110_e2895_d_n10, eq110_e2895_d_n11, eq110_e2895_d_n12, eq110_e2895_d_n13, eq110_e2895_d_n14, eq110_e2895_d_n15, eq110_e2895_d_n16, eq110_e2895_d_b0, eq110_e2895_d_b1, eq110_e2895_d_b2, eq110_e2895_d_b3, eq110_e2895_d_b4, eq110_e2895_d_b5, eq110_e2895_d_b6, eq110_e2895_d_b7, eq110_e2895_d_b8, eq110_e2895_d_b9, eq110_e2895_d_b10, eq110_e2895_d_b11, eq110_e2895_d_b12, eq110_e2895_d_b13, eq110_e2895_d_b14, eq110_e2895_d_b15, eq110_e2895_d_b16, eq110_e2895_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq110_value: f64 = eq110_e2897;
        let eq110_node_derivatives: [f64; 17] = [eq110_e2897_d_n0, eq110_e2897_d_n1, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n12, eq110_e2897_d_n13, eq110_e2897_d_n14, eq110_e2897_d_n15, eq110_e2897_d_n16];
        let eq110_branch_derivatives: [f64; 18] = [eq110_e2897_d_b0, eq110_e2897_d_b1, eq110_e2897_d_b2, eq110_e2897_d_b3, eq110_e2897_d_b4, eq110_e2897_d_b5, eq110_e2897_d_b6, eq110_e2897_d_b7, eq110_e2897_d_b8, eq110_e2897_d_b9, eq110_e2897_d_b10, eq110_e2897_d_b11, eq110_e2897_d_b12, eq110_e2897_d_b13, eq110_e2897_d_b14, eq110_e2897_d_b15, eq110_e2897_d_b16, eq110_e2897_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq110_value),
            nodes,
            &eq110_node_derivatives,
            branches,
            &eq110_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16, eq111_e2904_d_b0, eq111_e2904_d_b1, eq111_e2904_d_b2, eq111_e2904_d_b3, eq111_e2904_d_b4, eq111_e2904_d_b5, eq111_e2904_d_b6, eq111_e2904_d_b7, eq111_e2904_d_b8, eq111_e2904_d_b9, eq111_e2904_d_b10, eq111_e2904_d_b11, eq111_e2904_d_b12, eq111_e2904_d_b13, eq111_e2904_d_b14, eq111_e2904_d_b15, eq111_e2904_d_b16, eq111_e2904_d_b17,) = {
    if s.b[1736] {
        let eq111_e2901: f64 = ((nv4 - 0.0) * s.v[634]);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * s.dn[634][0]);
        let eq111_e2901_d_n1: f64 = ((nv4 - 0.0) * s.dn[634][1]);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * s.dn[634][2]);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * s.dn[634][3]);
        let eq111_e2901_d_n4: f64 = (s.v[634] + ((nv4 - 0.0) * s.dn[634][4]));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * s.dn[634][5]);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * s.dn[634][6]);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * s.dn[634][7]);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * s.dn[634][8]);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * s.dn[634][9]);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * s.dn[634][10]);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * s.dn[634][11]);
        let eq111_e2901_d_n12: f64 = ((nv4 - 0.0) * s.dn[634][12]);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * s.dn[634][13]);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * s.dn[634][14]);
        let eq111_e2901_d_n15: f64 = ((nv4 - 0.0) * s.dn[634][15]);
        let eq111_e2901_d_n16: f64 = ((nv4 - 0.0) * s.dn[634][16]);
        let eq111_e2901_d_b0: f64 = ((nv4 - 0.0) * s.db[634][0]);
        let eq111_e2901_d_b1: f64 = ((nv4 - 0.0) * s.db[634][1]);
        let eq111_e2901_d_b2: f64 = ((nv4 - 0.0) * s.db[634][2]);
        let eq111_e2901_d_b3: f64 = ((nv4 - 0.0) * s.db[634][3]);
        let eq111_e2901_d_b4: f64 = ((nv4 - 0.0) * s.db[634][4]);
        let eq111_e2901_d_b5: f64 = ((nv4 - 0.0) * s.db[634][5]);
        let eq111_e2901_d_b6: f64 = ((nv4 - 0.0) * s.db[634][6]);
        let eq111_e2901_d_b7: f64 = ((nv4 - 0.0) * s.db[634][7]);
        let eq111_e2901_d_b8: f64 = ((nv4 - 0.0) * s.db[634][8]);
        let eq111_e2901_d_b9: f64 = ((nv4 - 0.0) * s.db[634][9]);
        let eq111_e2901_d_b10: f64 = ((nv4 - 0.0) * s.db[634][10]);
        let eq111_e2901_d_b11: f64 = ((nv4 - 0.0) * s.db[634][11]);
        let eq111_e2901_d_b12: f64 = ((nv4 - 0.0) * s.db[634][12]);
        let eq111_e2901_d_b13: f64 = ((nv4 - 0.0) * s.db[634][13]);
        let eq111_e2901_d_b14: f64 = ((nv4 - 0.0) * s.db[634][14]);
        let eq111_e2901_d_b15: f64 = ((nv4 - 0.0) * s.db[634][15]);
        let eq111_e2901_d_b16: f64 = ((nv4 - 0.0) * s.db[634][16]);
        let eq111_e2901_d_b17: f64 = ((nv4 - 0.0) * s.db[634][17]);
        let eq111_e2902: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 27, eq111_e2901);
        let eq111_e2902_d_n0: f64 = (eq111_e2901_d_n0 * ddt_scale);
        let eq111_e2902_d_n1: f64 = (eq111_e2901_d_n1 * ddt_scale);
        let eq111_e2902_d_n2: f64 = (eq111_e2901_d_n2 * ddt_scale);
        let eq111_e2902_d_n3: f64 = (eq111_e2901_d_n3 * ddt_scale);
        let eq111_e2902_d_n4: f64 = (eq111_e2901_d_n4 * ddt_scale);
        let eq111_e2902_d_n5: f64 = (eq111_e2901_d_n5 * ddt_scale);
        let eq111_e2902_d_n6: f64 = (eq111_e2901_d_n6 * ddt_scale);
        let eq111_e2902_d_n7: f64 = (eq111_e2901_d_n7 * ddt_scale);
        let eq111_e2902_d_n8: f64 = (eq111_e2901_d_n8 * ddt_scale);
        let eq111_e2902_d_n9: f64 = (eq111_e2901_d_n9 * ddt_scale);
        let eq111_e2902_d_n10: f64 = (eq111_e2901_d_n10 * ddt_scale);
        let eq111_e2902_d_n11: f64 = (eq111_e2901_d_n11 * ddt_scale);
        let eq111_e2902_d_n12: f64 = (eq111_e2901_d_n12 * ddt_scale);
        let eq111_e2902_d_n13: f64 = (eq111_e2901_d_n13 * ddt_scale);
        let eq111_e2902_d_n14: f64 = (eq111_e2901_d_n14 * ddt_scale);
        let eq111_e2902_d_n15: f64 = (eq111_e2901_d_n15 * ddt_scale);
        let eq111_e2902_d_n16: f64 = (eq111_e2901_d_n16 * ddt_scale);
        let eq111_e2902_d_b0: f64 = (eq111_e2901_d_b0 * ddt_scale);
        let eq111_e2902_d_b1: f64 = (eq111_e2901_d_b1 * ddt_scale);
        let eq111_e2902_d_b2: f64 = (eq111_e2901_d_b2 * ddt_scale);
        let eq111_e2902_d_b3: f64 = (eq111_e2901_d_b3 * ddt_scale);
        let eq111_e2902_d_b4: f64 = (eq111_e2901_d_b4 * ddt_scale);
        let eq111_e2902_d_b5: f64 = (eq111_e2901_d_b5 * ddt_scale);
        let eq111_e2902_d_b6: f64 = (eq111_e2901_d_b6 * ddt_scale);
        let eq111_e2902_d_b7: f64 = (eq111_e2901_d_b7 * ddt_scale);
        let eq111_e2902_d_b8: f64 = (eq111_e2901_d_b8 * ddt_scale);
        let eq111_e2902_d_b9: f64 = (eq111_e2901_d_b9 * ddt_scale);
        let eq111_e2902_d_b10: f64 = (eq111_e2901_d_b10 * ddt_scale);
        let eq111_e2902_d_b11: f64 = (eq111_e2901_d_b11 * ddt_scale);
        let eq111_e2902_d_b12: f64 = (eq111_e2901_d_b12 * ddt_scale);
        let eq111_e2902_d_b13: f64 = (eq111_e2901_d_b13 * ddt_scale);
        let eq111_e2902_d_b14: f64 = (eq111_e2901_d_b14 * ddt_scale);
        let eq111_e2902_d_b15: f64 = (eq111_e2901_d_b15 * ddt_scale);
        let eq111_e2902_d_b16: f64 = (eq111_e2901_d_b16 * ddt_scale);
        let eq111_e2902_d_b17: f64 = (eq111_e2901_d_b17 * ddt_scale);
        (eq111_e2902, eq111_e2902_d_n0, eq111_e2902_d_n1, eq111_e2902_d_n2, eq111_e2902_d_n3, eq111_e2902_d_n4, eq111_e2902_d_n5, eq111_e2902_d_n6, eq111_e2902_d_n7, eq111_e2902_d_n8, eq111_e2902_d_n9, eq111_e2902_d_n10, eq111_e2902_d_n11, eq111_e2902_d_n12, eq111_e2902_d_n13, eq111_e2902_d_n14, eq111_e2902_d_n15, eq111_e2902_d_n16, eq111_e2902_d_b0, eq111_e2902_d_b1, eq111_e2902_d_b2, eq111_e2902_d_b3, eq111_e2902_d_b4, eq111_e2902_d_b5, eq111_e2902_d_b6, eq111_e2902_d_b7, eq111_e2902_d_b8, eq111_e2902_d_b9, eq111_e2902_d_b10, eq111_e2902_d_b11, eq111_e2902_d_b12, eq111_e2902_d_b13, eq111_e2902_d_b14, eq111_e2902_d_b15, eq111_e2902_d_b16, eq111_e2902_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e2904;
        let eq111_node_derivatives: [f64; 17] = [eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16];
        let eq111_branch_derivatives: [f64; 18] = [eq111_e2904_d_b0, eq111_e2904_d_b1, eq111_e2904_d_b2, eq111_e2904_d_b3, eq111_e2904_d_b4, eq111_e2904_d_b5, eq111_e2904_d_b6, eq111_e2904_d_b7, eq111_e2904_d_b8, eq111_e2904_d_b9, eq111_e2904_d_b10, eq111_e2904_d_b11, eq111_e2904_d_b12, eq111_e2904_d_b13, eq111_e2904_d_b14, eq111_e2904_d_b15, eq111_e2904_d_b16, eq111_e2904_d_b17];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            multiplicity * (eq111_value),
            nodes,
            &eq111_node_derivatives,
            branches,
            &eq111_branch_derivatives,
            multiplicity,
        );
        let (eq112_e2909,) = {
    if (!s.b[1736]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq112_value: f64 = eq112_e2909;
        stamper.stamp_potential_const(
            branches[17],
            eq112_value,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16, eq4_e1979_d_b0, eq4_e1979_d_b1, eq4_e1979_d_b2, eq4_e1979_d_b3, eq4_e1979_d_b4, eq4_e1979_d_b5, eq4_e1979_d_b6, eq4_e1979_d_b7, eq4_e1979_d_b8, eq4_e1979_d_b9, eq4_e1979_d_b10, eq4_e1979_d_b11, eq4_e1979_d_b12, eq4_e1979_d_b13, eq4_e1979_d_b14, eq4_e1979_d_b15, eq4_e1979_d_b16, eq4_e1979_d_b17, eq4_e1979_q, eq4_e1979_q_d_n0, eq4_e1979_q_d_n1, eq4_e1979_q_d_n2, eq4_e1979_q_d_n3, eq4_e1979_q_d_n4, eq4_e1979_q_d_n5, eq4_e1979_q_d_n6, eq4_e1979_q_d_n7, eq4_e1979_q_d_n8, eq4_e1979_q_d_n9, eq4_e1979_q_d_n10, eq4_e1979_q_d_n11, eq4_e1979_q_d_n12, eq4_e1979_q_d_n13, eq4_e1979_q_d_n14, eq4_e1979_q_d_n15, eq4_e1979_q_d_n16, eq4_e1979_q_d_b0, eq4_e1979_q_d_b1, eq4_e1979_q_d_b2, eq4_e1979_q_d_b3, eq4_e1979_q_d_b4, eq4_e1979_q_d_b5, eq4_e1979_q_d_b6, eq4_e1979_q_d_b7, eq4_e1979_q_d_b8, eq4_e1979_q_d_b9, eq4_e1979_q_d_b10, eq4_e1979_q_d_b11, eq4_e1979_q_d_b12, eq4_e1979_q_d_b13, eq4_e1979_q_d_b14, eq4_e1979_q_d_b15, eq4_e1979_q_d_b16, eq4_e1979_q_d_b17,) = {
    if (!s.b[1696]) {
        let eq4_e1976_q: f64 = s.v[137];
        let eq4_e1977: f64 = (s.v[114] * s.v[137]);
        let eq4_e1977_d_n0: f64 = ((s.dn[114][0] * s.v[137]) + (s.v[114] * s.dn[137][0]));
        let eq4_e1977_d_n1: f64 = ((s.dn[114][1] * s.v[137]) + (s.v[114] * s.dn[137][1]));
        let eq4_e1977_d_n2: f64 = ((s.dn[114][2] * s.v[137]) + (s.v[114] * s.dn[137][2]));
        let eq4_e1977_d_n3: f64 = ((s.dn[114][3] * s.v[137]) + (s.v[114] * s.dn[137][3]));
        let eq4_e1977_d_n4: f64 = ((s.dn[114][4] * s.v[137]) + (s.v[114] * s.dn[137][4]));
        let eq4_e1977_d_n5: f64 = ((s.dn[114][5] * s.v[137]) + (s.v[114] * s.dn[137][5]));
        let eq4_e1977_d_n6: f64 = ((s.dn[114][6] * s.v[137]) + (s.v[114] * s.dn[137][6]));
        let eq4_e1977_d_n7: f64 = ((s.dn[114][7] * s.v[137]) + (s.v[114] * s.dn[137][7]));
        let eq4_e1977_d_n8: f64 = ((s.dn[114][8] * s.v[137]) + (s.v[114] * s.dn[137][8]));
        let eq4_e1977_d_n9: f64 = ((s.dn[114][9] * s.v[137]) + (s.v[114] * s.dn[137][9]));
        let eq4_e1977_d_n10: f64 = ((s.dn[114][10] * s.v[137]) + (s.v[114] * s.dn[137][10]));
        let eq4_e1977_d_n11: f64 = ((s.dn[114][11] * s.v[137]) + (s.v[114] * s.dn[137][11]));
        let eq4_e1977_d_n12: f64 = ((s.dn[114][12] * s.v[137]) + (s.v[114] * s.dn[137][12]));
        let eq4_e1977_d_n13: f64 = ((s.dn[114][13] * s.v[137]) + (s.v[114] * s.dn[137][13]));
        let eq4_e1977_d_n14: f64 = ((s.dn[114][14] * s.v[137]) + (s.v[114] * s.dn[137][14]));
        let eq4_e1977_d_n15: f64 = ((s.dn[114][15] * s.v[137]) + (s.v[114] * s.dn[137][15]));
        let eq4_e1977_d_n16: f64 = ((s.dn[114][16] * s.v[137]) + (s.v[114] * s.dn[137][16]));
        let eq4_e1977_d_b0: f64 = ((s.db[114][0] * s.v[137]) + (s.v[114] * s.db[137][0]));
        let eq4_e1977_d_b1: f64 = ((s.db[114][1] * s.v[137]) + (s.v[114] * s.db[137][1]));
        let eq4_e1977_d_b2: f64 = ((s.db[114][2] * s.v[137]) + (s.v[114] * s.db[137][2]));
        let eq4_e1977_d_b3: f64 = ((s.db[114][3] * s.v[137]) + (s.v[114] * s.db[137][3]));
        let eq4_e1977_d_b4: f64 = ((s.db[114][4] * s.v[137]) + (s.v[114] * s.db[137][4]));
        let eq4_e1977_d_b5: f64 = ((s.db[114][5] * s.v[137]) + (s.v[114] * s.db[137][5]));
        let eq4_e1977_d_b6: f64 = ((s.db[114][6] * s.v[137]) + (s.v[114] * s.db[137][6]));
        let eq4_e1977_d_b7: f64 = ((s.db[114][7] * s.v[137]) + (s.v[114] * s.db[137][7]));
        let eq4_e1977_d_b8: f64 = ((s.db[114][8] * s.v[137]) + (s.v[114] * s.db[137][8]));
        let eq4_e1977_d_b9: f64 = ((s.db[114][9] * s.v[137]) + (s.v[114] * s.db[137][9]));
        let eq4_e1977_d_b10: f64 = ((s.db[114][10] * s.v[137]) + (s.v[114] * s.db[137][10]));
        let eq4_e1977_d_b11: f64 = ((s.db[114][11] * s.v[137]) + (s.v[114] * s.db[137][11]));
        let eq4_e1977_d_b12: f64 = ((s.db[114][12] * s.v[137]) + (s.v[114] * s.db[137][12]));
        let eq4_e1977_d_b13: f64 = ((s.db[114][13] * s.v[137]) + (s.v[114] * s.db[137][13]));
        let eq4_e1977_d_b14: f64 = ((s.db[114][14] * s.v[137]) + (s.v[114] * s.db[137][14]));
        let eq4_e1977_d_b15: f64 = ((s.db[114][15] * s.v[137]) + (s.v[114] * s.db[137][15]));
        let eq4_e1977_d_b16: f64 = ((s.db[114][16] * s.v[137]) + (s.v[114] * s.db[137][16]));
        let eq4_e1977_d_b17: f64 = ((s.db[114][17] * s.v[137]) + (s.v[114] * s.db[137][17]));
        let eq4_e1977_q: f64 = (s.v[114] * eq4_e1976_q);
        let eq4_e1977_q_d_n0: f64 = ((s.dn[114][0] * eq4_e1976_q) + (s.v[114] * s.dn[137][0]));
        let eq4_e1977_q_d_n1: f64 = ((s.dn[114][1] * eq4_e1976_q) + (s.v[114] * s.dn[137][1]));
        let eq4_e1977_q_d_n2: f64 = ((s.dn[114][2] * eq4_e1976_q) + (s.v[114] * s.dn[137][2]));
        let eq4_e1977_q_d_n3: f64 = ((s.dn[114][3] * eq4_e1976_q) + (s.v[114] * s.dn[137][3]));
        let eq4_e1977_q_d_n4: f64 = ((s.dn[114][4] * eq4_e1976_q) + (s.v[114] * s.dn[137][4]));
        let eq4_e1977_q_d_n5: f64 = ((s.dn[114][5] * eq4_e1976_q) + (s.v[114] * s.dn[137][5]));
        let eq4_e1977_q_d_n6: f64 = ((s.dn[114][6] * eq4_e1976_q) + (s.v[114] * s.dn[137][6]));
        let eq4_e1977_q_d_n7: f64 = ((s.dn[114][7] * eq4_e1976_q) + (s.v[114] * s.dn[137][7]));
        let eq4_e1977_q_d_n8: f64 = ((s.dn[114][8] * eq4_e1976_q) + (s.v[114] * s.dn[137][8]));
        let eq4_e1977_q_d_n9: f64 = ((s.dn[114][9] * eq4_e1976_q) + (s.v[114] * s.dn[137][9]));
        let eq4_e1977_q_d_n10: f64 = ((s.dn[114][10] * eq4_e1976_q) + (s.v[114] * s.dn[137][10]));
        let eq4_e1977_q_d_n11: f64 = ((s.dn[114][11] * eq4_e1976_q) + (s.v[114] * s.dn[137][11]));
        let eq4_e1977_q_d_n12: f64 = ((s.dn[114][12] * eq4_e1976_q) + (s.v[114] * s.dn[137][12]));
        let eq4_e1977_q_d_n13: f64 = ((s.dn[114][13] * eq4_e1976_q) + (s.v[114] * s.dn[137][13]));
        let eq4_e1977_q_d_n14: f64 = ((s.dn[114][14] * eq4_e1976_q) + (s.v[114] * s.dn[137][14]));
        let eq4_e1977_q_d_n15: f64 = ((s.dn[114][15] * eq4_e1976_q) + (s.v[114] * s.dn[137][15]));
        let eq4_e1977_q_d_n16: f64 = ((s.dn[114][16] * eq4_e1976_q) + (s.v[114] * s.dn[137][16]));
        let eq4_e1977_q_d_b0: f64 = ((s.db[114][0] * eq4_e1976_q) + (s.v[114] * s.db[137][0]));
        let eq4_e1977_q_d_b1: f64 = ((s.db[114][1] * eq4_e1976_q) + (s.v[114] * s.db[137][1]));
        let eq4_e1977_q_d_b2: f64 = ((s.db[114][2] * eq4_e1976_q) + (s.v[114] * s.db[137][2]));
        let eq4_e1977_q_d_b3: f64 = ((s.db[114][3] * eq4_e1976_q) + (s.v[114] * s.db[137][3]));
        let eq4_e1977_q_d_b4: f64 = ((s.db[114][4] * eq4_e1976_q) + (s.v[114] * s.db[137][4]));
        let eq4_e1977_q_d_b5: f64 = ((s.db[114][5] * eq4_e1976_q) + (s.v[114] * s.db[137][5]));
        let eq4_e1977_q_d_b6: f64 = ((s.db[114][6] * eq4_e1976_q) + (s.v[114] * s.db[137][6]));
        let eq4_e1977_q_d_b7: f64 = ((s.db[114][7] * eq4_e1976_q) + (s.v[114] * s.db[137][7]));
        let eq4_e1977_q_d_b8: f64 = ((s.db[114][8] * eq4_e1976_q) + (s.v[114] * s.db[137][8]));
        let eq4_e1977_q_d_b9: f64 = ((s.db[114][9] * eq4_e1976_q) + (s.v[114] * s.db[137][9]));
        let eq4_e1977_q_d_b10: f64 = ((s.db[114][10] * eq4_e1976_q) + (s.v[114] * s.db[137][10]));
        let eq4_e1977_q_d_b11: f64 = ((s.db[114][11] * eq4_e1976_q) + (s.v[114] * s.db[137][11]));
        let eq4_e1977_q_d_b12: f64 = ((s.db[114][12] * eq4_e1976_q) + (s.v[114] * s.db[137][12]));
        let eq4_e1977_q_d_b13: f64 = ((s.db[114][13] * eq4_e1976_q) + (s.v[114] * s.db[137][13]));
        let eq4_e1977_q_d_b14: f64 = ((s.db[114][14] * eq4_e1976_q) + (s.v[114] * s.db[137][14]));
        let eq4_e1977_q_d_b15: f64 = ((s.db[114][15] * eq4_e1976_q) + (s.v[114] * s.db[137][15]));
        let eq4_e1977_q_d_b16: f64 = ((s.db[114][16] * eq4_e1976_q) + (s.v[114] * s.db[137][16]));
        let eq4_e1977_q_d_b17: f64 = ((s.db[114][17] * eq4_e1976_q) + (s.v[114] * s.db[137][17]));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n1, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n12, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_d_n15, eq4_e1977_d_n16, eq4_e1977_d_b0, eq4_e1977_d_b1, eq4_e1977_d_b2, eq4_e1977_d_b3, eq4_e1977_d_b4, eq4_e1977_d_b5, eq4_e1977_d_b6, eq4_e1977_d_b7, eq4_e1977_d_b8, eq4_e1977_d_b9, eq4_e1977_d_b10, eq4_e1977_d_b11, eq4_e1977_d_b12, eq4_e1977_d_b13, eq4_e1977_d_b14, eq4_e1977_d_b15, eq4_e1977_d_b16, eq4_e1977_d_b17, eq4_e1977_q, eq4_e1977_q_d_n0, eq4_e1977_q_d_n1, eq4_e1977_q_d_n2, eq4_e1977_q_d_n3, eq4_e1977_q_d_n4, eq4_e1977_q_d_n5, eq4_e1977_q_d_n6, eq4_e1977_q_d_n7, eq4_e1977_q_d_n8, eq4_e1977_q_d_n9, eq4_e1977_q_d_n10, eq4_e1977_q_d_n11, eq4_e1977_q_d_n12, eq4_e1977_q_d_n13, eq4_e1977_q_d_n14, eq4_e1977_q_d_n15, eq4_e1977_q_d_n16, eq4_e1977_q_d_b0, eq4_e1977_q_d_b1, eq4_e1977_q_d_b2, eq4_e1977_q_d_b3, eq4_e1977_q_d_b4, eq4_e1977_q_d_b5, eq4_e1977_q_d_b6, eq4_e1977_q_d_b7, eq4_e1977_q_d_b8, eq4_e1977_q_d_b9, eq4_e1977_q_d_b10, eq4_e1977_q_d_b11, eq4_e1977_q_d_b12, eq4_e1977_q_d_b13, eq4_e1977_q_d_b14, eq4_e1977_q_d_b15, eq4_e1977_q_d_b16, eq4_e1977_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 17] = [eq4_e1979_q_d_n0, eq4_e1979_q_d_n1, eq4_e1979_q_d_n2, eq4_e1979_q_d_n3, eq4_e1979_q_d_n4, eq4_e1979_q_d_n5, eq4_e1979_q_d_n6, eq4_e1979_q_d_n7, eq4_e1979_q_d_n8, eq4_e1979_q_d_n9, eq4_e1979_q_d_n10, eq4_e1979_q_d_n11, eq4_e1979_q_d_n12, eq4_e1979_q_d_n13, eq4_e1979_q_d_n14, eq4_e1979_q_d_n15, eq4_e1979_q_d_n16];
        let eq4_reactive_branch_derivatives: [f64; 18] = [eq4_e1979_q_d_b0, eq4_e1979_q_d_b1, eq4_e1979_q_d_b2, eq4_e1979_q_d_b3, eq4_e1979_q_d_b4, eq4_e1979_q_d_b5, eq4_e1979_q_d_b6, eq4_e1979_q_d_b7, eq4_e1979_q_d_b8, eq4_e1979_q_d_b9, eq4_e1979_q_d_b10, eq4_e1979_q_d_b11, eq4_e1979_q_d_b12, eq4_e1979_q_d_b13, eq4_e1979_q_d_b14, eq4_e1979_q_d_b15, eq4_e1979_q_d_b16, eq4_e1979_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16, eq5_e1987_d_b0, eq5_e1987_d_b1, eq5_e1987_d_b2, eq5_e1987_d_b3, eq5_e1987_d_b4, eq5_e1987_d_b5, eq5_e1987_d_b6, eq5_e1987_d_b7, eq5_e1987_d_b8, eq5_e1987_d_b9, eq5_e1987_d_b10, eq5_e1987_d_b11, eq5_e1987_d_b12, eq5_e1987_d_b13, eq5_e1987_d_b14, eq5_e1987_d_b15, eq5_e1987_d_b16, eq5_e1987_d_b17, eq5_e1987_q, eq5_e1987_q_d_n0, eq5_e1987_q_d_n1, eq5_e1987_q_d_n2, eq5_e1987_q_d_n3, eq5_e1987_q_d_n4, eq5_e1987_q_d_n5, eq5_e1987_q_d_n6, eq5_e1987_q_d_n7, eq5_e1987_q_d_n8, eq5_e1987_q_d_n9, eq5_e1987_q_d_n10, eq5_e1987_q_d_n11, eq5_e1987_q_d_n12, eq5_e1987_q_d_n13, eq5_e1987_q_d_n14, eq5_e1987_q_d_n15, eq5_e1987_q_d_n16, eq5_e1987_q_d_b0, eq5_e1987_q_d_b1, eq5_e1987_q_d_b2, eq5_e1987_q_d_b3, eq5_e1987_q_d_b4, eq5_e1987_q_d_b5, eq5_e1987_q_d_b6, eq5_e1987_q_d_b7, eq5_e1987_q_d_b8, eq5_e1987_q_d_b9, eq5_e1987_q_d_b10, eq5_e1987_q_d_b11, eq5_e1987_q_d_b12, eq5_e1987_q_d_b13, eq5_e1987_q_d_b14, eq5_e1987_q_d_b15, eq5_e1987_q_d_b16, eq5_e1987_q_d_b17,) = {
    if (!s.b[1696]) {
        let eq5_e1984_q: f64 = s.v[138];
        let eq5_e1985: f64 = (s.v[114] * s.v[138]);
        let eq5_e1985_d_n0: f64 = ((s.dn[114][0] * s.v[138]) + (s.v[114] * s.dn[138][0]));
        let eq5_e1985_d_n1: f64 = ((s.dn[114][1] * s.v[138]) + (s.v[114] * s.dn[138][1]));
        let eq5_e1985_d_n2: f64 = ((s.dn[114][2] * s.v[138]) + (s.v[114] * s.dn[138][2]));
        let eq5_e1985_d_n3: f64 = ((s.dn[114][3] * s.v[138]) + (s.v[114] * s.dn[138][3]));
        let eq5_e1985_d_n4: f64 = ((s.dn[114][4] * s.v[138]) + (s.v[114] * s.dn[138][4]));
        let eq5_e1985_d_n5: f64 = ((s.dn[114][5] * s.v[138]) + (s.v[114] * s.dn[138][5]));
        let eq5_e1985_d_n6: f64 = ((s.dn[114][6] * s.v[138]) + (s.v[114] * s.dn[138][6]));
        let eq5_e1985_d_n7: f64 = ((s.dn[114][7] * s.v[138]) + (s.v[114] * s.dn[138][7]));
        let eq5_e1985_d_n8: f64 = ((s.dn[114][8] * s.v[138]) + (s.v[114] * s.dn[138][8]));
        let eq5_e1985_d_n9: f64 = ((s.dn[114][9] * s.v[138]) + (s.v[114] * s.dn[138][9]));
        let eq5_e1985_d_n10: f64 = ((s.dn[114][10] * s.v[138]) + (s.v[114] * s.dn[138][10]));
        let eq5_e1985_d_n11: f64 = ((s.dn[114][11] * s.v[138]) + (s.v[114] * s.dn[138][11]));
        let eq5_e1985_d_n12: f64 = ((s.dn[114][12] * s.v[138]) + (s.v[114] * s.dn[138][12]));
        let eq5_e1985_d_n13: f64 = ((s.dn[114][13] * s.v[138]) + (s.v[114] * s.dn[138][13]));
        let eq5_e1985_d_n14: f64 = ((s.dn[114][14] * s.v[138]) + (s.v[114] * s.dn[138][14]));
        let eq5_e1985_d_n15: f64 = ((s.dn[114][15] * s.v[138]) + (s.v[114] * s.dn[138][15]));
        let eq5_e1985_d_n16: f64 = ((s.dn[114][16] * s.v[138]) + (s.v[114] * s.dn[138][16]));
        let eq5_e1985_d_b0: f64 = ((s.db[114][0] * s.v[138]) + (s.v[114] * s.db[138][0]));
        let eq5_e1985_d_b1: f64 = ((s.db[114][1] * s.v[138]) + (s.v[114] * s.db[138][1]));
        let eq5_e1985_d_b2: f64 = ((s.db[114][2] * s.v[138]) + (s.v[114] * s.db[138][2]));
        let eq5_e1985_d_b3: f64 = ((s.db[114][3] * s.v[138]) + (s.v[114] * s.db[138][3]));
        let eq5_e1985_d_b4: f64 = ((s.db[114][4] * s.v[138]) + (s.v[114] * s.db[138][4]));
        let eq5_e1985_d_b5: f64 = ((s.db[114][5] * s.v[138]) + (s.v[114] * s.db[138][5]));
        let eq5_e1985_d_b6: f64 = ((s.db[114][6] * s.v[138]) + (s.v[114] * s.db[138][6]));
        let eq5_e1985_d_b7: f64 = ((s.db[114][7] * s.v[138]) + (s.v[114] * s.db[138][7]));
        let eq5_e1985_d_b8: f64 = ((s.db[114][8] * s.v[138]) + (s.v[114] * s.db[138][8]));
        let eq5_e1985_d_b9: f64 = ((s.db[114][9] * s.v[138]) + (s.v[114] * s.db[138][9]));
        let eq5_e1985_d_b10: f64 = ((s.db[114][10] * s.v[138]) + (s.v[114] * s.db[138][10]));
        let eq5_e1985_d_b11: f64 = ((s.db[114][11] * s.v[138]) + (s.v[114] * s.db[138][11]));
        let eq5_e1985_d_b12: f64 = ((s.db[114][12] * s.v[138]) + (s.v[114] * s.db[138][12]));
        let eq5_e1985_d_b13: f64 = ((s.db[114][13] * s.v[138]) + (s.v[114] * s.db[138][13]));
        let eq5_e1985_d_b14: f64 = ((s.db[114][14] * s.v[138]) + (s.v[114] * s.db[138][14]));
        let eq5_e1985_d_b15: f64 = ((s.db[114][15] * s.v[138]) + (s.v[114] * s.db[138][15]));
        let eq5_e1985_d_b16: f64 = ((s.db[114][16] * s.v[138]) + (s.v[114] * s.db[138][16]));
        let eq5_e1985_d_b17: f64 = ((s.db[114][17] * s.v[138]) + (s.v[114] * s.db[138][17]));
        let eq5_e1985_q: f64 = (s.v[114] * eq5_e1984_q);
        let eq5_e1985_q_d_n0: f64 = ((s.dn[114][0] * eq5_e1984_q) + (s.v[114] * s.dn[138][0]));
        let eq5_e1985_q_d_n1: f64 = ((s.dn[114][1] * eq5_e1984_q) + (s.v[114] * s.dn[138][1]));
        let eq5_e1985_q_d_n2: f64 = ((s.dn[114][2] * eq5_e1984_q) + (s.v[114] * s.dn[138][2]));
        let eq5_e1985_q_d_n3: f64 = ((s.dn[114][3] * eq5_e1984_q) + (s.v[114] * s.dn[138][3]));
        let eq5_e1985_q_d_n4: f64 = ((s.dn[114][4] * eq5_e1984_q) + (s.v[114] * s.dn[138][4]));
        let eq5_e1985_q_d_n5: f64 = ((s.dn[114][5] * eq5_e1984_q) + (s.v[114] * s.dn[138][5]));
        let eq5_e1985_q_d_n6: f64 = ((s.dn[114][6] * eq5_e1984_q) + (s.v[114] * s.dn[138][6]));
        let eq5_e1985_q_d_n7: f64 = ((s.dn[114][7] * eq5_e1984_q) + (s.v[114] * s.dn[138][7]));
        let eq5_e1985_q_d_n8: f64 = ((s.dn[114][8] * eq5_e1984_q) + (s.v[114] * s.dn[138][8]));
        let eq5_e1985_q_d_n9: f64 = ((s.dn[114][9] * eq5_e1984_q) + (s.v[114] * s.dn[138][9]));
        let eq5_e1985_q_d_n10: f64 = ((s.dn[114][10] * eq5_e1984_q) + (s.v[114] * s.dn[138][10]));
        let eq5_e1985_q_d_n11: f64 = ((s.dn[114][11] * eq5_e1984_q) + (s.v[114] * s.dn[138][11]));
        let eq5_e1985_q_d_n12: f64 = ((s.dn[114][12] * eq5_e1984_q) + (s.v[114] * s.dn[138][12]));
        let eq5_e1985_q_d_n13: f64 = ((s.dn[114][13] * eq5_e1984_q) + (s.v[114] * s.dn[138][13]));
        let eq5_e1985_q_d_n14: f64 = ((s.dn[114][14] * eq5_e1984_q) + (s.v[114] * s.dn[138][14]));
        let eq5_e1985_q_d_n15: f64 = ((s.dn[114][15] * eq5_e1984_q) + (s.v[114] * s.dn[138][15]));
        let eq5_e1985_q_d_n16: f64 = ((s.dn[114][16] * eq5_e1984_q) + (s.v[114] * s.dn[138][16]));
        let eq5_e1985_q_d_b0: f64 = ((s.db[114][0] * eq5_e1984_q) + (s.v[114] * s.db[138][0]));
        let eq5_e1985_q_d_b1: f64 = ((s.db[114][1] * eq5_e1984_q) + (s.v[114] * s.db[138][1]));
        let eq5_e1985_q_d_b2: f64 = ((s.db[114][2] * eq5_e1984_q) + (s.v[114] * s.db[138][2]));
        let eq5_e1985_q_d_b3: f64 = ((s.db[114][3] * eq5_e1984_q) + (s.v[114] * s.db[138][3]));
        let eq5_e1985_q_d_b4: f64 = ((s.db[114][4] * eq5_e1984_q) + (s.v[114] * s.db[138][4]));
        let eq5_e1985_q_d_b5: f64 = ((s.db[114][5] * eq5_e1984_q) + (s.v[114] * s.db[138][5]));
        let eq5_e1985_q_d_b6: f64 = ((s.db[114][6] * eq5_e1984_q) + (s.v[114] * s.db[138][6]));
        let eq5_e1985_q_d_b7: f64 = ((s.db[114][7] * eq5_e1984_q) + (s.v[114] * s.db[138][7]));
        let eq5_e1985_q_d_b8: f64 = ((s.db[114][8] * eq5_e1984_q) + (s.v[114] * s.db[138][8]));
        let eq5_e1985_q_d_b9: f64 = ((s.db[114][9] * eq5_e1984_q) + (s.v[114] * s.db[138][9]));
        let eq5_e1985_q_d_b10: f64 = ((s.db[114][10] * eq5_e1984_q) + (s.v[114] * s.db[138][10]));
        let eq5_e1985_q_d_b11: f64 = ((s.db[114][11] * eq5_e1984_q) + (s.v[114] * s.db[138][11]));
        let eq5_e1985_q_d_b12: f64 = ((s.db[114][12] * eq5_e1984_q) + (s.v[114] * s.db[138][12]));
        let eq5_e1985_q_d_b13: f64 = ((s.db[114][13] * eq5_e1984_q) + (s.v[114] * s.db[138][13]));
        let eq5_e1985_q_d_b14: f64 = ((s.db[114][14] * eq5_e1984_q) + (s.v[114] * s.db[138][14]));
        let eq5_e1985_q_d_b15: f64 = ((s.db[114][15] * eq5_e1984_q) + (s.v[114] * s.db[138][15]));
        let eq5_e1985_q_d_b16: f64 = ((s.db[114][16] * eq5_e1984_q) + (s.v[114] * s.db[138][16]));
        let eq5_e1985_q_d_b17: f64 = ((s.db[114][17] * eq5_e1984_q) + (s.v[114] * s.db[138][17]));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n1, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n12, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_d_n15, eq5_e1985_d_n16, eq5_e1985_d_b0, eq5_e1985_d_b1, eq5_e1985_d_b2, eq5_e1985_d_b3, eq5_e1985_d_b4, eq5_e1985_d_b5, eq5_e1985_d_b6, eq5_e1985_d_b7, eq5_e1985_d_b8, eq5_e1985_d_b9, eq5_e1985_d_b10, eq5_e1985_d_b11, eq5_e1985_d_b12, eq5_e1985_d_b13, eq5_e1985_d_b14, eq5_e1985_d_b15, eq5_e1985_d_b16, eq5_e1985_d_b17, eq5_e1985_q, eq5_e1985_q_d_n0, eq5_e1985_q_d_n1, eq5_e1985_q_d_n2, eq5_e1985_q_d_n3, eq5_e1985_q_d_n4, eq5_e1985_q_d_n5, eq5_e1985_q_d_n6, eq5_e1985_q_d_n7, eq5_e1985_q_d_n8, eq5_e1985_q_d_n9, eq5_e1985_q_d_n10, eq5_e1985_q_d_n11, eq5_e1985_q_d_n12, eq5_e1985_q_d_n13, eq5_e1985_q_d_n14, eq5_e1985_q_d_n15, eq5_e1985_q_d_n16, eq5_e1985_q_d_b0, eq5_e1985_q_d_b1, eq5_e1985_q_d_b2, eq5_e1985_q_d_b3, eq5_e1985_q_d_b4, eq5_e1985_q_d_b5, eq5_e1985_q_d_b6, eq5_e1985_q_d_b7, eq5_e1985_q_d_b8, eq5_e1985_q_d_b9, eq5_e1985_q_d_b10, eq5_e1985_q_d_b11, eq5_e1985_q_d_b12, eq5_e1985_q_d_b13, eq5_e1985_q_d_b14, eq5_e1985_q_d_b15, eq5_e1985_q_d_b16, eq5_e1985_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 17] = [eq5_e1987_q_d_n0, eq5_e1987_q_d_n1, eq5_e1987_q_d_n2, eq5_e1987_q_d_n3, eq5_e1987_q_d_n4, eq5_e1987_q_d_n5, eq5_e1987_q_d_n6, eq5_e1987_q_d_n7, eq5_e1987_q_d_n8, eq5_e1987_q_d_n9, eq5_e1987_q_d_n10, eq5_e1987_q_d_n11, eq5_e1987_q_d_n12, eq5_e1987_q_d_n13, eq5_e1987_q_d_n14, eq5_e1987_q_d_n15, eq5_e1987_q_d_n16];
        let eq5_reactive_branch_derivatives: [f64; 18] = [eq5_e1987_q_d_b0, eq5_e1987_q_d_b1, eq5_e1987_q_d_b2, eq5_e1987_q_d_b3, eq5_e1987_q_d_b4, eq5_e1987_q_d_b5, eq5_e1987_q_d_b6, eq5_e1987_q_d_b7, eq5_e1987_q_d_b8, eq5_e1987_q_d_b9, eq5_e1987_q_d_b10, eq5_e1987_q_d_b11, eq5_e1987_q_d_b12, eq5_e1987_q_d_b13, eq5_e1987_q_d_b14, eq5_e1987_q_d_b15, eq5_e1987_q_d_b16, eq5_e1987_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281_q: f64 = s.v[507];
        let eq36_e2282: f64 = (s.v[114] * s.v[507]);
        let eq36_e2282_d_n0: f64 = ((s.dn[114][0] * s.v[507]) + (s.v[114] * s.dn[507][0]));
        let eq36_e2282_d_n1: f64 = ((s.dn[114][1] * s.v[507]) + (s.v[114] * s.dn[507][1]));
        let eq36_e2282_d_n2: f64 = ((s.dn[114][2] * s.v[507]) + (s.v[114] * s.dn[507][2]));
        let eq36_e2282_d_n3: f64 = ((s.dn[114][3] * s.v[507]) + (s.v[114] * s.dn[507][3]));
        let eq36_e2282_d_n4: f64 = ((s.dn[114][4] * s.v[507]) + (s.v[114] * s.dn[507][4]));
        let eq36_e2282_d_n5: f64 = ((s.dn[114][5] * s.v[507]) + (s.v[114] * s.dn[507][5]));
        let eq36_e2282_d_n6: f64 = ((s.dn[114][6] * s.v[507]) + (s.v[114] * s.dn[507][6]));
        let eq36_e2282_d_n7: f64 = ((s.dn[114][7] * s.v[507]) + (s.v[114] * s.dn[507][7]));
        let eq36_e2282_d_n8: f64 = ((s.dn[114][8] * s.v[507]) + (s.v[114] * s.dn[507][8]));
        let eq36_e2282_d_n9: f64 = ((s.dn[114][9] * s.v[507]) + (s.v[114] * s.dn[507][9]));
        let eq36_e2282_d_n10: f64 = ((s.dn[114][10] * s.v[507]) + (s.v[114] * s.dn[507][10]));
        let eq36_e2282_d_n11: f64 = ((s.dn[114][11] * s.v[507]) + (s.v[114] * s.dn[507][11]));
        let eq36_e2282_d_n12: f64 = ((s.dn[114][12] * s.v[507]) + (s.v[114] * s.dn[507][12]));
        let eq36_e2282_d_n13: f64 = ((s.dn[114][13] * s.v[507]) + (s.v[114] * s.dn[507][13]));
        let eq36_e2282_d_n14: f64 = ((s.dn[114][14] * s.v[507]) + (s.v[114] * s.dn[507][14]));
        let eq36_e2282_d_n15: f64 = ((s.dn[114][15] * s.v[507]) + (s.v[114] * s.dn[507][15]));
        let eq36_e2282_d_n16: f64 = ((s.dn[114][16] * s.v[507]) + (s.v[114] * s.dn[507][16]));
        let eq36_e2282_d_b0: f64 = ((s.db[114][0] * s.v[507]) + (s.v[114] * s.db[507][0]));
        let eq36_e2282_d_b1: f64 = ((s.db[114][1] * s.v[507]) + (s.v[114] * s.db[507][1]));
        let eq36_e2282_d_b2: f64 = ((s.db[114][2] * s.v[507]) + (s.v[114] * s.db[507][2]));
        let eq36_e2282_d_b3: f64 = ((s.db[114][3] * s.v[507]) + (s.v[114] * s.db[507][3]));
        let eq36_e2282_d_b4: f64 = ((s.db[114][4] * s.v[507]) + (s.v[114] * s.db[507][4]));
        let eq36_e2282_d_b5: f64 = ((s.db[114][5] * s.v[507]) + (s.v[114] * s.db[507][5]));
        let eq36_e2282_d_b6: f64 = ((s.db[114][6] * s.v[507]) + (s.v[114] * s.db[507][6]));
        let eq36_e2282_d_b7: f64 = ((s.db[114][7] * s.v[507]) + (s.v[114] * s.db[507][7]));
        let eq36_e2282_d_b8: f64 = ((s.db[114][8] * s.v[507]) + (s.v[114] * s.db[507][8]));
        let eq36_e2282_d_b9: f64 = ((s.db[114][9] * s.v[507]) + (s.v[114] * s.db[507][9]));
        let eq36_e2282_d_b10: f64 = ((s.db[114][10] * s.v[507]) + (s.v[114] * s.db[507][10]));
        let eq36_e2282_d_b11: f64 = ((s.db[114][11] * s.v[507]) + (s.v[114] * s.db[507][11]));
        let eq36_e2282_d_b12: f64 = ((s.db[114][12] * s.v[507]) + (s.v[114] * s.db[507][12]));
        let eq36_e2282_d_b13: f64 = ((s.db[114][13] * s.v[507]) + (s.v[114] * s.db[507][13]));
        let eq36_e2282_d_b14: f64 = ((s.db[114][14] * s.v[507]) + (s.v[114] * s.db[507][14]));
        let eq36_e2282_d_b15: f64 = ((s.db[114][15] * s.v[507]) + (s.v[114] * s.db[507][15]));
        let eq36_e2282_d_b16: f64 = ((s.db[114][16] * s.v[507]) + (s.v[114] * s.db[507][16]));
        let eq36_e2282_d_b17: f64 = ((s.db[114][17] * s.v[507]) + (s.v[114] * s.db[507][17]));
        let eq36_e2282_q: f64 = (s.v[114] * eq36_e2281_q);
        let eq36_e2282_q_d_n0: f64 = ((s.dn[114][0] * eq36_e2281_q) + (s.v[114] * s.dn[507][0]));
        let eq36_e2282_q_d_n1: f64 = ((s.dn[114][1] * eq36_e2281_q) + (s.v[114] * s.dn[507][1]));
        let eq36_e2282_q_d_n2: f64 = ((s.dn[114][2] * eq36_e2281_q) + (s.v[114] * s.dn[507][2]));
        let eq36_e2282_q_d_n3: f64 = ((s.dn[114][3] * eq36_e2281_q) + (s.v[114] * s.dn[507][3]));
        let eq36_e2282_q_d_n4: f64 = ((s.dn[114][4] * eq36_e2281_q) + (s.v[114] * s.dn[507][4]));
        let eq36_e2282_q_d_n5: f64 = ((s.dn[114][5] * eq36_e2281_q) + (s.v[114] * s.dn[507][5]));
        let eq36_e2282_q_d_n6: f64 = ((s.dn[114][6] * eq36_e2281_q) + (s.v[114] * s.dn[507][6]));
        let eq36_e2282_q_d_n7: f64 = ((s.dn[114][7] * eq36_e2281_q) + (s.v[114] * s.dn[507][7]));
        let eq36_e2282_q_d_n8: f64 = ((s.dn[114][8] * eq36_e2281_q) + (s.v[114] * s.dn[507][8]));
        let eq36_e2282_q_d_n9: f64 = ((s.dn[114][9] * eq36_e2281_q) + (s.v[114] * s.dn[507][9]));
        let eq36_e2282_q_d_n10: f64 = ((s.dn[114][10] * eq36_e2281_q) + (s.v[114] * s.dn[507][10]));
        let eq36_e2282_q_d_n11: f64 = ((s.dn[114][11] * eq36_e2281_q) + (s.v[114] * s.dn[507][11]));
        let eq36_e2282_q_d_n12: f64 = ((s.dn[114][12] * eq36_e2281_q) + (s.v[114] * s.dn[507][12]));
        let eq36_e2282_q_d_n13: f64 = ((s.dn[114][13] * eq36_e2281_q) + (s.v[114] * s.dn[507][13]));
        let eq36_e2282_q_d_n14: f64 = ((s.dn[114][14] * eq36_e2281_q) + (s.v[114] * s.dn[507][14]));
        let eq36_e2282_q_d_n15: f64 = ((s.dn[114][15] * eq36_e2281_q) + (s.v[114] * s.dn[507][15]));
        let eq36_e2282_q_d_n16: f64 = ((s.dn[114][16] * eq36_e2281_q) + (s.v[114] * s.dn[507][16]));
        let eq36_e2282_q_d_b0: f64 = ((s.db[114][0] * eq36_e2281_q) + (s.v[114] * s.db[507][0]));
        let eq36_e2282_q_d_b1: f64 = ((s.db[114][1] * eq36_e2281_q) + (s.v[114] * s.db[507][1]));
        let eq36_e2282_q_d_b2: f64 = ((s.db[114][2] * eq36_e2281_q) + (s.v[114] * s.db[507][2]));
        let eq36_e2282_q_d_b3: f64 = ((s.db[114][3] * eq36_e2281_q) + (s.v[114] * s.db[507][3]));
        let eq36_e2282_q_d_b4: f64 = ((s.db[114][4] * eq36_e2281_q) + (s.v[114] * s.db[507][4]));
        let eq36_e2282_q_d_b5: f64 = ((s.db[114][5] * eq36_e2281_q) + (s.v[114] * s.db[507][5]));
        let eq36_e2282_q_d_b6: f64 = ((s.db[114][6] * eq36_e2281_q) + (s.v[114] * s.db[507][6]));
        let eq36_e2282_q_d_b7: f64 = ((s.db[114][7] * eq36_e2281_q) + (s.v[114] * s.db[507][7]));
        let eq36_e2282_q_d_b8: f64 = ((s.db[114][8] * eq36_e2281_q) + (s.v[114] * s.db[507][8]));
        let eq36_e2282_q_d_b9: f64 = ((s.db[114][9] * eq36_e2281_q) + (s.v[114] * s.db[507][9]));
        let eq36_e2282_q_d_b10: f64 = ((s.db[114][10] * eq36_e2281_q) + (s.v[114] * s.db[507][10]));
        let eq36_e2282_q_d_b11: f64 = ((s.db[114][11] * eq36_e2281_q) + (s.v[114] * s.db[507][11]));
        let eq36_e2282_q_d_b12: f64 = ((s.db[114][12] * eq36_e2281_q) + (s.v[114] * s.db[507][12]));
        let eq36_e2282_q_d_b13: f64 = ((s.db[114][13] * eq36_e2281_q) + (s.v[114] * s.db[507][13]));
        let eq36_e2282_q_d_b14: f64 = ((s.db[114][14] * eq36_e2281_q) + (s.v[114] * s.db[507][14]));
        let eq36_e2282_q_d_b15: f64 = ((s.db[114][15] * eq36_e2281_q) + (s.v[114] * s.db[507][15]));
        let eq36_e2282_q_d_b16: f64 = ((s.db[114][16] * eq36_e2281_q) + (s.v[114] * s.db[507][16]));
        let eq36_e2282_q_d_b17: f64 = ((s.db[114][17] * eq36_e2281_q) + (s.v[114] * s.db[507][17]));
        let eq36_reactive_node_derivatives: [f64; 17] = [eq36_e2282_q_d_n0, eq36_e2282_q_d_n1, eq36_e2282_q_d_n2, eq36_e2282_q_d_n3, eq36_e2282_q_d_n4, eq36_e2282_q_d_n5, eq36_e2282_q_d_n6, eq36_e2282_q_d_n7, eq36_e2282_q_d_n8, eq36_e2282_q_d_n9, eq36_e2282_q_d_n10, eq36_e2282_q_d_n11, eq36_e2282_q_d_n12, eq36_e2282_q_d_n13, eq36_e2282_q_d_n14, eq36_e2282_q_d_n15, eq36_e2282_q_d_n16];
        let eq36_reactive_branch_derivatives: [f64; 18] = [eq36_e2282_q_d_b0, eq36_e2282_q_d_b1, eq36_e2282_q_d_b2, eq36_e2282_q_d_b3, eq36_e2282_q_d_b4, eq36_e2282_q_d_b5, eq36_e2282_q_d_b6, eq36_e2282_q_d_b7, eq36_e2282_q_d_b8, eq36_e2282_q_d_b9, eq36_e2282_q_d_b10, eq36_e2282_q_d_b11, eq36_e2282_q_d_b12, eq36_e2282_q_d_b13, eq36_e2282_q_d_b14, eq36_e2282_q_d_b15, eq36_e2282_q_d_b16, eq36_e2282_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e2285_q: f64 = s.v[508];
        let eq37_e2286: f64 = (s.v[114] * s.v[508]);
        let eq37_e2286_d_n0: f64 = ((s.dn[114][0] * s.v[508]) + (s.v[114] * s.dn[508][0]));
        let eq37_e2286_d_n1: f64 = ((s.dn[114][1] * s.v[508]) + (s.v[114] * s.dn[508][1]));
        let eq37_e2286_d_n2: f64 = ((s.dn[114][2] * s.v[508]) + (s.v[114] * s.dn[508][2]));
        let eq37_e2286_d_n3: f64 = ((s.dn[114][3] * s.v[508]) + (s.v[114] * s.dn[508][3]));
        let eq37_e2286_d_n4: f64 = ((s.dn[114][4] * s.v[508]) + (s.v[114] * s.dn[508][4]));
        let eq37_e2286_d_n5: f64 = ((s.dn[114][5] * s.v[508]) + (s.v[114] * s.dn[508][5]));
        let eq37_e2286_d_n6: f64 = ((s.dn[114][6] * s.v[508]) + (s.v[114] * s.dn[508][6]));
        let eq37_e2286_d_n7: f64 = ((s.dn[114][7] * s.v[508]) + (s.v[114] * s.dn[508][7]));
        let eq37_e2286_d_n8: f64 = ((s.dn[114][8] * s.v[508]) + (s.v[114] * s.dn[508][8]));
        let eq37_e2286_d_n9: f64 = ((s.dn[114][9] * s.v[508]) + (s.v[114] * s.dn[508][9]));
        let eq37_e2286_d_n10: f64 = ((s.dn[114][10] * s.v[508]) + (s.v[114] * s.dn[508][10]));
        let eq37_e2286_d_n11: f64 = ((s.dn[114][11] * s.v[508]) + (s.v[114] * s.dn[508][11]));
        let eq37_e2286_d_n12: f64 = ((s.dn[114][12] * s.v[508]) + (s.v[114] * s.dn[508][12]));
        let eq37_e2286_d_n13: f64 = ((s.dn[114][13] * s.v[508]) + (s.v[114] * s.dn[508][13]));
        let eq37_e2286_d_n14: f64 = ((s.dn[114][14] * s.v[508]) + (s.v[114] * s.dn[508][14]));
        let eq37_e2286_d_n15: f64 = ((s.dn[114][15] * s.v[508]) + (s.v[114] * s.dn[508][15]));
        let eq37_e2286_d_n16: f64 = ((s.dn[114][16] * s.v[508]) + (s.v[114] * s.dn[508][16]));
        let eq37_e2286_d_b0: f64 = ((s.db[114][0] * s.v[508]) + (s.v[114] * s.db[508][0]));
        let eq37_e2286_d_b1: f64 = ((s.db[114][1] * s.v[508]) + (s.v[114] * s.db[508][1]));
        let eq37_e2286_d_b2: f64 = ((s.db[114][2] * s.v[508]) + (s.v[114] * s.db[508][2]));
        let eq37_e2286_d_b3: f64 = ((s.db[114][3] * s.v[508]) + (s.v[114] * s.db[508][3]));
        let eq37_e2286_d_b4: f64 = ((s.db[114][4] * s.v[508]) + (s.v[114] * s.db[508][4]));
        let eq37_e2286_d_b5: f64 = ((s.db[114][5] * s.v[508]) + (s.v[114] * s.db[508][5]));
        let eq37_e2286_d_b6: f64 = ((s.db[114][6] * s.v[508]) + (s.v[114] * s.db[508][6]));
        let eq37_e2286_d_b7: f64 = ((s.db[114][7] * s.v[508]) + (s.v[114] * s.db[508][7]));
        let eq37_e2286_d_b8: f64 = ((s.db[114][8] * s.v[508]) + (s.v[114] * s.db[508][8]));
        let eq37_e2286_d_b9: f64 = ((s.db[114][9] * s.v[508]) + (s.v[114] * s.db[508][9]));
        let eq37_e2286_d_b10: f64 = ((s.db[114][10] * s.v[508]) + (s.v[114] * s.db[508][10]));
        let eq37_e2286_d_b11: f64 = ((s.db[114][11] * s.v[508]) + (s.v[114] * s.db[508][11]));
        let eq37_e2286_d_b12: f64 = ((s.db[114][12] * s.v[508]) + (s.v[114] * s.db[508][12]));
        let eq37_e2286_d_b13: f64 = ((s.db[114][13] * s.v[508]) + (s.v[114] * s.db[508][13]));
        let eq37_e2286_d_b14: f64 = ((s.db[114][14] * s.v[508]) + (s.v[114] * s.db[508][14]));
        let eq37_e2286_d_b15: f64 = ((s.db[114][15] * s.v[508]) + (s.v[114] * s.db[508][15]));
        let eq37_e2286_d_b16: f64 = ((s.db[114][16] * s.v[508]) + (s.v[114] * s.db[508][16]));
        let eq37_e2286_d_b17: f64 = ((s.db[114][17] * s.v[508]) + (s.v[114] * s.db[508][17]));
        let eq37_e2286_q: f64 = (s.v[114] * eq37_e2285_q);
        let eq37_e2286_q_d_n0: f64 = ((s.dn[114][0] * eq37_e2285_q) + (s.v[114] * s.dn[508][0]));
        let eq37_e2286_q_d_n1: f64 = ((s.dn[114][1] * eq37_e2285_q) + (s.v[114] * s.dn[508][1]));
        let eq37_e2286_q_d_n2: f64 = ((s.dn[114][2] * eq37_e2285_q) + (s.v[114] * s.dn[508][2]));
        let eq37_e2286_q_d_n3: f64 = ((s.dn[114][3] * eq37_e2285_q) + (s.v[114] * s.dn[508][3]));
        let eq37_e2286_q_d_n4: f64 = ((s.dn[114][4] * eq37_e2285_q) + (s.v[114] * s.dn[508][4]));
        let eq37_e2286_q_d_n5: f64 = ((s.dn[114][5] * eq37_e2285_q) + (s.v[114] * s.dn[508][5]));
        let eq37_e2286_q_d_n6: f64 = ((s.dn[114][6] * eq37_e2285_q) + (s.v[114] * s.dn[508][6]));
        let eq37_e2286_q_d_n7: f64 = ((s.dn[114][7] * eq37_e2285_q) + (s.v[114] * s.dn[508][7]));
        let eq37_e2286_q_d_n8: f64 = ((s.dn[114][8] * eq37_e2285_q) + (s.v[114] * s.dn[508][8]));
        let eq37_e2286_q_d_n9: f64 = ((s.dn[114][9] * eq37_e2285_q) + (s.v[114] * s.dn[508][9]));
        let eq37_e2286_q_d_n10: f64 = ((s.dn[114][10] * eq37_e2285_q) + (s.v[114] * s.dn[508][10]));
        let eq37_e2286_q_d_n11: f64 = ((s.dn[114][11] * eq37_e2285_q) + (s.v[114] * s.dn[508][11]));
        let eq37_e2286_q_d_n12: f64 = ((s.dn[114][12] * eq37_e2285_q) + (s.v[114] * s.dn[508][12]));
        let eq37_e2286_q_d_n13: f64 = ((s.dn[114][13] * eq37_e2285_q) + (s.v[114] * s.dn[508][13]));
        let eq37_e2286_q_d_n14: f64 = ((s.dn[114][14] * eq37_e2285_q) + (s.v[114] * s.dn[508][14]));
        let eq37_e2286_q_d_n15: f64 = ((s.dn[114][15] * eq37_e2285_q) + (s.v[114] * s.dn[508][15]));
        let eq37_e2286_q_d_n16: f64 = ((s.dn[114][16] * eq37_e2285_q) + (s.v[114] * s.dn[508][16]));
        let eq37_e2286_q_d_b0: f64 = ((s.db[114][0] * eq37_e2285_q) + (s.v[114] * s.db[508][0]));
        let eq37_e2286_q_d_b1: f64 = ((s.db[114][1] * eq37_e2285_q) + (s.v[114] * s.db[508][1]));
        let eq37_e2286_q_d_b2: f64 = ((s.db[114][2] * eq37_e2285_q) + (s.v[114] * s.db[508][2]));
        let eq37_e2286_q_d_b3: f64 = ((s.db[114][3] * eq37_e2285_q) + (s.v[114] * s.db[508][3]));
        let eq37_e2286_q_d_b4: f64 = ((s.db[114][4] * eq37_e2285_q) + (s.v[114] * s.db[508][4]));
        let eq37_e2286_q_d_b5: f64 = ((s.db[114][5] * eq37_e2285_q) + (s.v[114] * s.db[508][5]));
        let eq37_e2286_q_d_b6: f64 = ((s.db[114][6] * eq37_e2285_q) + (s.v[114] * s.db[508][6]));
        let eq37_e2286_q_d_b7: f64 = ((s.db[114][7] * eq37_e2285_q) + (s.v[114] * s.db[508][7]));
        let eq37_e2286_q_d_b8: f64 = ((s.db[114][8] * eq37_e2285_q) + (s.v[114] * s.db[508][8]));
        let eq37_e2286_q_d_b9: f64 = ((s.db[114][9] * eq37_e2285_q) + (s.v[114] * s.db[508][9]));
        let eq37_e2286_q_d_b10: f64 = ((s.db[114][10] * eq37_e2285_q) + (s.v[114] * s.db[508][10]));
        let eq37_e2286_q_d_b11: f64 = ((s.db[114][11] * eq37_e2285_q) + (s.v[114] * s.db[508][11]));
        let eq37_e2286_q_d_b12: f64 = ((s.db[114][12] * eq37_e2285_q) + (s.v[114] * s.db[508][12]));
        let eq37_e2286_q_d_b13: f64 = ((s.db[114][13] * eq37_e2285_q) + (s.v[114] * s.db[508][13]));
        let eq37_e2286_q_d_b14: f64 = ((s.db[114][14] * eq37_e2285_q) + (s.v[114] * s.db[508][14]));
        let eq37_e2286_q_d_b15: f64 = ((s.db[114][15] * eq37_e2285_q) + (s.v[114] * s.db[508][15]));
        let eq37_e2286_q_d_b16: f64 = ((s.db[114][16] * eq37_e2285_q) + (s.v[114] * s.db[508][16]));
        let eq37_e2286_q_d_b17: f64 = ((s.db[114][17] * eq37_e2285_q) + (s.v[114] * s.db[508][17]));
        let eq37_reactive_node_derivatives: [f64; 17] = [eq37_e2286_q_d_n0, eq37_e2286_q_d_n1, eq37_e2286_q_d_n2, eq37_e2286_q_d_n3, eq37_e2286_q_d_n4, eq37_e2286_q_d_n5, eq37_e2286_q_d_n6, eq37_e2286_q_d_n7, eq37_e2286_q_d_n8, eq37_e2286_q_d_n9, eq37_e2286_q_d_n10, eq37_e2286_q_d_n11, eq37_e2286_q_d_n12, eq37_e2286_q_d_n13, eq37_e2286_q_d_n14, eq37_e2286_q_d_n15, eq37_e2286_q_d_n16];
        let eq37_reactive_branch_derivatives: [f64; 18] = [eq37_e2286_q_d_b0, eq37_e2286_q_d_b1, eq37_e2286_q_d_b2, eq37_e2286_q_d_b3, eq37_e2286_q_d_b4, eq37_e2286_q_d_b5, eq37_e2286_q_d_b6, eq37_e2286_q_d_b7, eq37_e2286_q_d_b8, eq37_e2286_q_d_b9, eq37_e2286_q_d_b10, eq37_e2286_q_d_b11, eq37_e2286_q_d_b12, eq37_e2286_q_d_b13, eq37_e2286_q_d_b14, eq37_e2286_q_d_b15, eq37_e2286_q_d_b16, eq37_e2286_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289_q: f64 = s.v[509];
        let eq38_e2290: f64 = (s.v[114] * s.v[509]);
        let eq38_e2290_d_n0: f64 = ((s.dn[114][0] * s.v[509]) + (s.v[114] * s.dn[509][0]));
        let eq38_e2290_d_n1: f64 = ((s.dn[114][1] * s.v[509]) + (s.v[114] * s.dn[509][1]));
        let eq38_e2290_d_n2: f64 = ((s.dn[114][2] * s.v[509]) + (s.v[114] * s.dn[509][2]));
        let eq38_e2290_d_n3: f64 = ((s.dn[114][3] * s.v[509]) + (s.v[114] * s.dn[509][3]));
        let eq38_e2290_d_n4: f64 = ((s.dn[114][4] * s.v[509]) + (s.v[114] * s.dn[509][4]));
        let eq38_e2290_d_n5: f64 = ((s.dn[114][5] * s.v[509]) + (s.v[114] * s.dn[509][5]));
        let eq38_e2290_d_n6: f64 = ((s.dn[114][6] * s.v[509]) + (s.v[114] * s.dn[509][6]));
        let eq38_e2290_d_n7: f64 = ((s.dn[114][7] * s.v[509]) + (s.v[114] * s.dn[509][7]));
        let eq38_e2290_d_n8: f64 = ((s.dn[114][8] * s.v[509]) + (s.v[114] * s.dn[509][8]));
        let eq38_e2290_d_n9: f64 = ((s.dn[114][9] * s.v[509]) + (s.v[114] * s.dn[509][9]));
        let eq38_e2290_d_n10: f64 = ((s.dn[114][10] * s.v[509]) + (s.v[114] * s.dn[509][10]));
        let eq38_e2290_d_n11: f64 = ((s.dn[114][11] * s.v[509]) + (s.v[114] * s.dn[509][11]));
        let eq38_e2290_d_n12: f64 = ((s.dn[114][12] * s.v[509]) + (s.v[114] * s.dn[509][12]));
        let eq38_e2290_d_n13: f64 = ((s.dn[114][13] * s.v[509]) + (s.v[114] * s.dn[509][13]));
        let eq38_e2290_d_n14: f64 = ((s.dn[114][14] * s.v[509]) + (s.v[114] * s.dn[509][14]));
        let eq38_e2290_d_n15: f64 = ((s.dn[114][15] * s.v[509]) + (s.v[114] * s.dn[509][15]));
        let eq38_e2290_d_n16: f64 = ((s.dn[114][16] * s.v[509]) + (s.v[114] * s.dn[509][16]));
        let eq38_e2290_d_b0: f64 = ((s.db[114][0] * s.v[509]) + (s.v[114] * s.db[509][0]));
        let eq38_e2290_d_b1: f64 = ((s.db[114][1] * s.v[509]) + (s.v[114] * s.db[509][1]));
        let eq38_e2290_d_b2: f64 = ((s.db[114][2] * s.v[509]) + (s.v[114] * s.db[509][2]));
        let eq38_e2290_d_b3: f64 = ((s.db[114][3] * s.v[509]) + (s.v[114] * s.db[509][3]));
        let eq38_e2290_d_b4: f64 = ((s.db[114][4] * s.v[509]) + (s.v[114] * s.db[509][4]));
        let eq38_e2290_d_b5: f64 = ((s.db[114][5] * s.v[509]) + (s.v[114] * s.db[509][5]));
        let eq38_e2290_d_b6: f64 = ((s.db[114][6] * s.v[509]) + (s.v[114] * s.db[509][6]));
        let eq38_e2290_d_b7: f64 = ((s.db[114][7] * s.v[509]) + (s.v[114] * s.db[509][7]));
        let eq38_e2290_d_b8: f64 = ((s.db[114][8] * s.v[509]) + (s.v[114] * s.db[509][8]));
        let eq38_e2290_d_b9: f64 = ((s.db[114][9] * s.v[509]) + (s.v[114] * s.db[509][9]));
        let eq38_e2290_d_b10: f64 = ((s.db[114][10] * s.v[509]) + (s.v[114] * s.db[509][10]));
        let eq38_e2290_d_b11: f64 = ((s.db[114][11] * s.v[509]) + (s.v[114] * s.db[509][11]));
        let eq38_e2290_d_b12: f64 = ((s.db[114][12] * s.v[509]) + (s.v[114] * s.db[509][12]));
        let eq38_e2290_d_b13: f64 = ((s.db[114][13] * s.v[509]) + (s.v[114] * s.db[509][13]));
        let eq38_e2290_d_b14: f64 = ((s.db[114][14] * s.v[509]) + (s.v[114] * s.db[509][14]));
        let eq38_e2290_d_b15: f64 = ((s.db[114][15] * s.v[509]) + (s.v[114] * s.db[509][15]));
        let eq38_e2290_d_b16: f64 = ((s.db[114][16] * s.v[509]) + (s.v[114] * s.db[509][16]));
        let eq38_e2290_d_b17: f64 = ((s.db[114][17] * s.v[509]) + (s.v[114] * s.db[509][17]));
        let eq38_e2290_q: f64 = (s.v[114] * eq38_e2289_q);
        let eq38_e2290_q_d_n0: f64 = ((s.dn[114][0] * eq38_e2289_q) + (s.v[114] * s.dn[509][0]));
        let eq38_e2290_q_d_n1: f64 = ((s.dn[114][1] * eq38_e2289_q) + (s.v[114] * s.dn[509][1]));
        let eq38_e2290_q_d_n2: f64 = ((s.dn[114][2] * eq38_e2289_q) + (s.v[114] * s.dn[509][2]));
        let eq38_e2290_q_d_n3: f64 = ((s.dn[114][3] * eq38_e2289_q) + (s.v[114] * s.dn[509][3]));
        let eq38_e2290_q_d_n4: f64 = ((s.dn[114][4] * eq38_e2289_q) + (s.v[114] * s.dn[509][4]));
        let eq38_e2290_q_d_n5: f64 = ((s.dn[114][5] * eq38_e2289_q) + (s.v[114] * s.dn[509][5]));
        let eq38_e2290_q_d_n6: f64 = ((s.dn[114][6] * eq38_e2289_q) + (s.v[114] * s.dn[509][6]));
        let eq38_e2290_q_d_n7: f64 = ((s.dn[114][7] * eq38_e2289_q) + (s.v[114] * s.dn[509][7]));
        let eq38_e2290_q_d_n8: f64 = ((s.dn[114][8] * eq38_e2289_q) + (s.v[114] * s.dn[509][8]));
        let eq38_e2290_q_d_n9: f64 = ((s.dn[114][9] * eq38_e2289_q) + (s.v[114] * s.dn[509][9]));
        let eq38_e2290_q_d_n10: f64 = ((s.dn[114][10] * eq38_e2289_q) + (s.v[114] * s.dn[509][10]));
        let eq38_e2290_q_d_n11: f64 = ((s.dn[114][11] * eq38_e2289_q) + (s.v[114] * s.dn[509][11]));
        let eq38_e2290_q_d_n12: f64 = ((s.dn[114][12] * eq38_e2289_q) + (s.v[114] * s.dn[509][12]));
        let eq38_e2290_q_d_n13: f64 = ((s.dn[114][13] * eq38_e2289_q) + (s.v[114] * s.dn[509][13]));
        let eq38_e2290_q_d_n14: f64 = ((s.dn[114][14] * eq38_e2289_q) + (s.v[114] * s.dn[509][14]));
        let eq38_e2290_q_d_n15: f64 = ((s.dn[114][15] * eq38_e2289_q) + (s.v[114] * s.dn[509][15]));
        let eq38_e2290_q_d_n16: f64 = ((s.dn[114][16] * eq38_e2289_q) + (s.v[114] * s.dn[509][16]));
        let eq38_e2290_q_d_b0: f64 = ((s.db[114][0] * eq38_e2289_q) + (s.v[114] * s.db[509][0]));
        let eq38_e2290_q_d_b1: f64 = ((s.db[114][1] * eq38_e2289_q) + (s.v[114] * s.db[509][1]));
        let eq38_e2290_q_d_b2: f64 = ((s.db[114][2] * eq38_e2289_q) + (s.v[114] * s.db[509][2]));
        let eq38_e2290_q_d_b3: f64 = ((s.db[114][3] * eq38_e2289_q) + (s.v[114] * s.db[509][3]));
        let eq38_e2290_q_d_b4: f64 = ((s.db[114][4] * eq38_e2289_q) + (s.v[114] * s.db[509][4]));
        let eq38_e2290_q_d_b5: f64 = ((s.db[114][5] * eq38_e2289_q) + (s.v[114] * s.db[509][5]));
        let eq38_e2290_q_d_b6: f64 = ((s.db[114][6] * eq38_e2289_q) + (s.v[114] * s.db[509][6]));
        let eq38_e2290_q_d_b7: f64 = ((s.db[114][7] * eq38_e2289_q) + (s.v[114] * s.db[509][7]));
        let eq38_e2290_q_d_b8: f64 = ((s.db[114][8] * eq38_e2289_q) + (s.v[114] * s.db[509][8]));
        let eq38_e2290_q_d_b9: f64 = ((s.db[114][9] * eq38_e2289_q) + (s.v[114] * s.db[509][9]));
        let eq38_e2290_q_d_b10: f64 = ((s.db[114][10] * eq38_e2289_q) + (s.v[114] * s.db[509][10]));
        let eq38_e2290_q_d_b11: f64 = ((s.db[114][11] * eq38_e2289_q) + (s.v[114] * s.db[509][11]));
        let eq38_e2290_q_d_b12: f64 = ((s.db[114][12] * eq38_e2289_q) + (s.v[114] * s.db[509][12]));
        let eq38_e2290_q_d_b13: f64 = ((s.db[114][13] * eq38_e2289_q) + (s.v[114] * s.db[509][13]));
        let eq38_e2290_q_d_b14: f64 = ((s.db[114][14] * eq38_e2289_q) + (s.v[114] * s.db[509][14]));
        let eq38_e2290_q_d_b15: f64 = ((s.db[114][15] * eq38_e2289_q) + (s.v[114] * s.db[509][15]));
        let eq38_e2290_q_d_b16: f64 = ((s.db[114][16] * eq38_e2289_q) + (s.v[114] * s.db[509][16]));
        let eq38_e2290_q_d_b17: f64 = ((s.db[114][17] * eq38_e2289_q) + (s.v[114] * s.db[509][17]));
        let eq38_reactive_node_derivatives: [f64; 17] = [eq38_e2290_q_d_n0, eq38_e2290_q_d_n1, eq38_e2290_q_d_n2, eq38_e2290_q_d_n3, eq38_e2290_q_d_n4, eq38_e2290_q_d_n5, eq38_e2290_q_d_n6, eq38_e2290_q_d_n7, eq38_e2290_q_d_n8, eq38_e2290_q_d_n9, eq38_e2290_q_d_n10, eq38_e2290_q_d_n11, eq38_e2290_q_d_n12, eq38_e2290_q_d_n13, eq38_e2290_q_d_n14, eq38_e2290_q_d_n15, eq38_e2290_q_d_n16];
        let eq38_reactive_branch_derivatives: [f64; 18] = [eq38_e2290_q_d_b0, eq38_e2290_q_d_b1, eq38_e2290_q_d_b2, eq38_e2290_q_d_b3, eq38_e2290_q_d_b4, eq38_e2290_q_d_b5, eq38_e2290_q_d_b6, eq38_e2290_q_d_b7, eq38_e2290_q_d_b8, eq38_e2290_q_d_b9, eq38_e2290_q_d_b10, eq38_e2290_q_d_b11, eq38_e2290_q_d_b12, eq38_e2290_q_d_b13, eq38_e2290_q_d_b14, eq38_e2290_q_d_b15, eq38_e2290_q_d_b16, eq38_e2290_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16, eq39_e2295_d_b0, eq39_e2295_d_b1, eq39_e2295_d_b2, eq39_e2295_d_b3, eq39_e2295_d_b4, eq39_e2295_d_b5, eq39_e2295_d_b6, eq39_e2295_d_b7, eq39_e2295_d_b8, eq39_e2295_d_b9, eq39_e2295_d_b10, eq39_e2295_d_b11, eq39_e2295_d_b12, eq39_e2295_d_b13, eq39_e2295_d_b14, eq39_e2295_d_b15, eq39_e2295_d_b16, eq39_e2295_d_b17, eq39_e2295_q, eq39_e2295_q_d_n0, eq39_e2295_q_d_n1, eq39_e2295_q_d_n2, eq39_e2295_q_d_n3, eq39_e2295_q_d_n4, eq39_e2295_q_d_n5, eq39_e2295_q_d_n6, eq39_e2295_q_d_n7, eq39_e2295_q_d_n8, eq39_e2295_q_d_n9, eq39_e2295_q_d_n10, eq39_e2295_q_d_n11, eq39_e2295_q_d_n12, eq39_e2295_q_d_n13, eq39_e2295_q_d_n14, eq39_e2295_q_d_n15, eq39_e2295_q_d_n16, eq39_e2295_q_d_b0, eq39_e2295_q_d_b1, eq39_e2295_q_d_b2, eq39_e2295_q_d_b3, eq39_e2295_q_d_b4, eq39_e2295_q_d_b5, eq39_e2295_q_d_b6, eq39_e2295_q_d_b7, eq39_e2295_q_d_b8, eq39_e2295_q_d_b9, eq39_e2295_q_d_b10, eq39_e2295_q_d_b11, eq39_e2295_q_d_b12, eq39_e2295_q_d_b13, eq39_e2295_q_d_b14, eq39_e2295_q_d_b15, eq39_e2295_q_d_b16, eq39_e2295_q_d_b17,) = {
    if s.b[1705] {
        let eq39_e2293_q: f64 = s.v[505];
        (s.v[505], s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], s.db[505][0], s.db[505][1], s.db[505][2], s.db[505][3], s.db[505][4], s.db[505][5], s.db[505][6], s.db[505][7], s.db[505][8], s.db[505][9], s.db[505][10], s.db[505][11], s.db[505][12], s.db[505][13], s.db[505][14], s.db[505][15], s.db[505][16], s.db[505][17], eq39_e2293_q, s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], s.db[505][0], s.db[505][1], s.db[505][2], s.db[505][3], s.db[505][4], s.db[505][5], s.db[505][6], s.db[505][7], s.db[505][8], s.db[505][9], s.db[505][10], s.db[505][11], s.db[505][12], s.db[505][13], s.db[505][14], s.db[505][15], s.db[505][16], s.db[505][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 17] = [eq39_e2295_q_d_n0, eq39_e2295_q_d_n1, eq39_e2295_q_d_n2, eq39_e2295_q_d_n3, eq39_e2295_q_d_n4, eq39_e2295_q_d_n5, eq39_e2295_q_d_n6, eq39_e2295_q_d_n7, eq39_e2295_q_d_n8, eq39_e2295_q_d_n9, eq39_e2295_q_d_n10, eq39_e2295_q_d_n11, eq39_e2295_q_d_n12, eq39_e2295_q_d_n13, eq39_e2295_q_d_n14, eq39_e2295_q_d_n15, eq39_e2295_q_d_n16];
        let eq39_reactive_branch_derivatives: [f64; 18] = [eq39_e2295_q_d_b0, eq39_e2295_q_d_b1, eq39_e2295_q_d_b2, eq39_e2295_q_d_b3, eq39_e2295_q_d_b4, eq39_e2295_q_d_b5, eq39_e2295_q_d_b6, eq39_e2295_q_d_b7, eq39_e2295_q_d_b8, eq39_e2295_q_d_b9, eq39_e2295_q_d_b10, eq39_e2295_q_d_b11, eq39_e2295_q_d_b12, eq39_e2295_q_d_b13, eq39_e2295_q_d_b14, eq39_e2295_q_d_b15, eq39_e2295_q_d_b16, eq39_e2295_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16, eq40_e2302_d_b0, eq40_e2302_d_b1, eq40_e2302_d_b2, eq40_e2302_d_b3, eq40_e2302_d_b4, eq40_e2302_d_b5, eq40_e2302_d_b6, eq40_e2302_d_b7, eq40_e2302_d_b8, eq40_e2302_d_b9, eq40_e2302_d_b10, eq40_e2302_d_b11, eq40_e2302_d_b12, eq40_e2302_d_b13, eq40_e2302_d_b14, eq40_e2302_d_b15, eq40_e2302_d_b16, eq40_e2302_d_b17, eq40_e2302_q, eq40_e2302_q_d_n0, eq40_e2302_q_d_n1, eq40_e2302_q_d_n2, eq40_e2302_q_d_n3, eq40_e2302_q_d_n4, eq40_e2302_q_d_n5, eq40_e2302_q_d_n6, eq40_e2302_q_d_n7, eq40_e2302_q_d_n8, eq40_e2302_q_d_n9, eq40_e2302_q_d_n10, eq40_e2302_q_d_n11, eq40_e2302_q_d_n12, eq40_e2302_q_d_n13, eq40_e2302_q_d_n14, eq40_e2302_q_d_n15, eq40_e2302_q_d_n16, eq40_e2302_q_d_b0, eq40_e2302_q_d_b1, eq40_e2302_q_d_b2, eq40_e2302_q_d_b3, eq40_e2302_q_d_b4, eq40_e2302_q_d_b5, eq40_e2302_q_d_b6, eq40_e2302_q_d_b7, eq40_e2302_q_d_b8, eq40_e2302_q_d_b9, eq40_e2302_q_d_b10, eq40_e2302_q_d_b11, eq40_e2302_q_d_b12, eq40_e2302_q_d_b13, eq40_e2302_q_d_b14, eq40_e2302_q_d_b15, eq40_e2302_q_d_b16, eq40_e2302_q_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq40_e2300_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq40_e2300_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 17] = [eq40_e2302_q_d_n0, eq40_e2302_q_d_n1, eq40_e2302_q_d_n2, eq40_e2302_q_d_n3, eq40_e2302_q_d_n4, eq40_e2302_q_d_n5, eq40_e2302_q_d_n6, eq40_e2302_q_d_n7, eq40_e2302_q_d_n8, eq40_e2302_q_d_n9, eq40_e2302_q_d_n10, eq40_e2302_q_d_n11, eq40_e2302_q_d_n12, eq40_e2302_q_d_n13, eq40_e2302_q_d_n14, eq40_e2302_q_d_n15, eq40_e2302_q_d_n16];
        let eq40_reactive_branch_derivatives: [f64; 18] = [eq40_e2302_q_d_b0, eq40_e2302_q_d_b1, eq40_e2302_q_d_b2, eq40_e2302_q_d_b3, eq40_e2302_q_d_b4, eq40_e2302_q_d_b5, eq40_e2302_q_d_b6, eq40_e2302_q_d_b7, eq40_e2302_q_d_b8, eq40_e2302_q_d_b9, eq40_e2302_q_d_b10, eq40_e2302_q_d_b11, eq40_e2302_q_d_b12, eq40_e2302_q_d_b13, eq40_e2302_q_d_b14, eq40_e2302_q_d_b15, eq40_e2302_q_d_b16, eq40_e2302_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16, eq41_e2311_d_b0, eq41_e2311_d_b1, eq41_e2311_d_b2, eq41_e2311_d_b3, eq41_e2311_d_b4, eq41_e2311_d_b5, eq41_e2311_d_b6, eq41_e2311_d_b7, eq41_e2311_d_b8, eq41_e2311_d_b9, eq41_e2311_d_b10, eq41_e2311_d_b11, eq41_e2311_d_b12, eq41_e2311_d_b13, eq41_e2311_d_b14, eq41_e2311_d_b15, eq41_e2311_d_b16, eq41_e2311_d_b17, eq41_e2311_q, eq41_e2311_q_d_n0, eq41_e2311_q_d_n1, eq41_e2311_q_d_n2, eq41_e2311_q_d_n3, eq41_e2311_q_d_n4, eq41_e2311_q_d_n5, eq41_e2311_q_d_n6, eq41_e2311_q_d_n7, eq41_e2311_q_d_n8, eq41_e2311_q_d_n9, eq41_e2311_q_d_n10, eq41_e2311_q_d_n11, eq41_e2311_q_d_n12, eq41_e2311_q_d_n13, eq41_e2311_q_d_n14, eq41_e2311_q_d_n15, eq41_e2311_q_d_n16, eq41_e2311_q_d_b0, eq41_e2311_q_d_b1, eq41_e2311_q_d_b2, eq41_e2311_q_d_b3, eq41_e2311_q_d_b4, eq41_e2311_q_d_b5, eq41_e2311_q_d_b6, eq41_e2311_q_d_b7, eq41_e2311_q_d_b8, eq41_e2311_q_d_b9, eq41_e2311_q_d_b10, eq41_e2311_q_d_b11, eq41_e2311_q_d_b12, eq41_e2311_q_d_b13, eq41_e2311_q_d_b14, eq41_e2311_q_d_b15, eq41_e2311_q_d_b16, eq41_e2311_q_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq41_e2308_q: f64 = s.v[503];
        let eq41_e2309: f64 = (s.v[114] * s.v[503]);
        let eq41_e2309_d_n0: f64 = ((s.dn[114][0] * s.v[503]) + (s.v[114] * s.dn[503][0]));
        let eq41_e2309_d_n1: f64 = ((s.dn[114][1] * s.v[503]) + (s.v[114] * s.dn[503][1]));
        let eq41_e2309_d_n2: f64 = ((s.dn[114][2] * s.v[503]) + (s.v[114] * s.dn[503][2]));
        let eq41_e2309_d_n3: f64 = ((s.dn[114][3] * s.v[503]) + (s.v[114] * s.dn[503][3]));
        let eq41_e2309_d_n4: f64 = ((s.dn[114][4] * s.v[503]) + (s.v[114] * s.dn[503][4]));
        let eq41_e2309_d_n5: f64 = ((s.dn[114][5] * s.v[503]) + (s.v[114] * s.dn[503][5]));
        let eq41_e2309_d_n6: f64 = ((s.dn[114][6] * s.v[503]) + (s.v[114] * s.dn[503][6]));
        let eq41_e2309_d_n7: f64 = ((s.dn[114][7] * s.v[503]) + (s.v[114] * s.dn[503][7]));
        let eq41_e2309_d_n8: f64 = ((s.dn[114][8] * s.v[503]) + (s.v[114] * s.dn[503][8]));
        let eq41_e2309_d_n9: f64 = ((s.dn[114][9] * s.v[503]) + (s.v[114] * s.dn[503][9]));
        let eq41_e2309_d_n10: f64 = ((s.dn[114][10] * s.v[503]) + (s.v[114] * s.dn[503][10]));
        let eq41_e2309_d_n11: f64 = ((s.dn[114][11] * s.v[503]) + (s.v[114] * s.dn[503][11]));
        let eq41_e2309_d_n12: f64 = ((s.dn[114][12] * s.v[503]) + (s.v[114] * s.dn[503][12]));
        let eq41_e2309_d_n13: f64 = ((s.dn[114][13] * s.v[503]) + (s.v[114] * s.dn[503][13]));
        let eq41_e2309_d_n14: f64 = ((s.dn[114][14] * s.v[503]) + (s.v[114] * s.dn[503][14]));
        let eq41_e2309_d_n15: f64 = ((s.dn[114][15] * s.v[503]) + (s.v[114] * s.dn[503][15]));
        let eq41_e2309_d_n16: f64 = ((s.dn[114][16] * s.v[503]) + (s.v[114] * s.dn[503][16]));
        let eq41_e2309_d_b0: f64 = ((s.db[114][0] * s.v[503]) + (s.v[114] * s.db[503][0]));
        let eq41_e2309_d_b1: f64 = ((s.db[114][1] * s.v[503]) + (s.v[114] * s.db[503][1]));
        let eq41_e2309_d_b2: f64 = ((s.db[114][2] * s.v[503]) + (s.v[114] * s.db[503][2]));
        let eq41_e2309_d_b3: f64 = ((s.db[114][3] * s.v[503]) + (s.v[114] * s.db[503][3]));
        let eq41_e2309_d_b4: f64 = ((s.db[114][4] * s.v[503]) + (s.v[114] * s.db[503][4]));
        let eq41_e2309_d_b5: f64 = ((s.db[114][5] * s.v[503]) + (s.v[114] * s.db[503][5]));
        let eq41_e2309_d_b6: f64 = ((s.db[114][6] * s.v[503]) + (s.v[114] * s.db[503][6]));
        let eq41_e2309_d_b7: f64 = ((s.db[114][7] * s.v[503]) + (s.v[114] * s.db[503][7]));
        let eq41_e2309_d_b8: f64 = ((s.db[114][8] * s.v[503]) + (s.v[114] * s.db[503][8]));
        let eq41_e2309_d_b9: f64 = ((s.db[114][9] * s.v[503]) + (s.v[114] * s.db[503][9]));
        let eq41_e2309_d_b10: f64 = ((s.db[114][10] * s.v[503]) + (s.v[114] * s.db[503][10]));
        let eq41_e2309_d_b11: f64 = ((s.db[114][11] * s.v[503]) + (s.v[114] * s.db[503][11]));
        let eq41_e2309_d_b12: f64 = ((s.db[114][12] * s.v[503]) + (s.v[114] * s.db[503][12]));
        let eq41_e2309_d_b13: f64 = ((s.db[114][13] * s.v[503]) + (s.v[114] * s.db[503][13]));
        let eq41_e2309_d_b14: f64 = ((s.db[114][14] * s.v[503]) + (s.v[114] * s.db[503][14]));
        let eq41_e2309_d_b15: f64 = ((s.db[114][15] * s.v[503]) + (s.v[114] * s.db[503][15]));
        let eq41_e2309_d_b16: f64 = ((s.db[114][16] * s.v[503]) + (s.v[114] * s.db[503][16]));
        let eq41_e2309_d_b17: f64 = ((s.db[114][17] * s.v[503]) + (s.v[114] * s.db[503][17]));
        let eq41_e2309_q: f64 = (s.v[114] * eq41_e2308_q);
        let eq41_e2309_q_d_n0: f64 = ((s.dn[114][0] * eq41_e2308_q) + (s.v[114] * s.dn[503][0]));
        let eq41_e2309_q_d_n1: f64 = ((s.dn[114][1] * eq41_e2308_q) + (s.v[114] * s.dn[503][1]));
        let eq41_e2309_q_d_n2: f64 = ((s.dn[114][2] * eq41_e2308_q) + (s.v[114] * s.dn[503][2]));
        let eq41_e2309_q_d_n3: f64 = ((s.dn[114][3] * eq41_e2308_q) + (s.v[114] * s.dn[503][3]));
        let eq41_e2309_q_d_n4: f64 = ((s.dn[114][4] * eq41_e2308_q) + (s.v[114] * s.dn[503][4]));
        let eq41_e2309_q_d_n5: f64 = ((s.dn[114][5] * eq41_e2308_q) + (s.v[114] * s.dn[503][5]));
        let eq41_e2309_q_d_n6: f64 = ((s.dn[114][6] * eq41_e2308_q) + (s.v[114] * s.dn[503][6]));
        let eq41_e2309_q_d_n7: f64 = ((s.dn[114][7] * eq41_e2308_q) + (s.v[114] * s.dn[503][7]));
        let eq41_e2309_q_d_n8: f64 = ((s.dn[114][8] * eq41_e2308_q) + (s.v[114] * s.dn[503][8]));
        let eq41_e2309_q_d_n9: f64 = ((s.dn[114][9] * eq41_e2308_q) + (s.v[114] * s.dn[503][9]));
        let eq41_e2309_q_d_n10: f64 = ((s.dn[114][10] * eq41_e2308_q) + (s.v[114] * s.dn[503][10]));
        let eq41_e2309_q_d_n11: f64 = ((s.dn[114][11] * eq41_e2308_q) + (s.v[114] * s.dn[503][11]));
        let eq41_e2309_q_d_n12: f64 = ((s.dn[114][12] * eq41_e2308_q) + (s.v[114] * s.dn[503][12]));
        let eq41_e2309_q_d_n13: f64 = ((s.dn[114][13] * eq41_e2308_q) + (s.v[114] * s.dn[503][13]));
        let eq41_e2309_q_d_n14: f64 = ((s.dn[114][14] * eq41_e2308_q) + (s.v[114] * s.dn[503][14]));
        let eq41_e2309_q_d_n15: f64 = ((s.dn[114][15] * eq41_e2308_q) + (s.v[114] * s.dn[503][15]));
        let eq41_e2309_q_d_n16: f64 = ((s.dn[114][16] * eq41_e2308_q) + (s.v[114] * s.dn[503][16]));
        let eq41_e2309_q_d_b0: f64 = ((s.db[114][0] * eq41_e2308_q) + (s.v[114] * s.db[503][0]));
        let eq41_e2309_q_d_b1: f64 = ((s.db[114][1] * eq41_e2308_q) + (s.v[114] * s.db[503][1]));
        let eq41_e2309_q_d_b2: f64 = ((s.db[114][2] * eq41_e2308_q) + (s.v[114] * s.db[503][2]));
        let eq41_e2309_q_d_b3: f64 = ((s.db[114][3] * eq41_e2308_q) + (s.v[114] * s.db[503][3]));
        let eq41_e2309_q_d_b4: f64 = ((s.db[114][4] * eq41_e2308_q) + (s.v[114] * s.db[503][4]));
        let eq41_e2309_q_d_b5: f64 = ((s.db[114][5] * eq41_e2308_q) + (s.v[114] * s.db[503][5]));
        let eq41_e2309_q_d_b6: f64 = ((s.db[114][6] * eq41_e2308_q) + (s.v[114] * s.db[503][6]));
        let eq41_e2309_q_d_b7: f64 = ((s.db[114][7] * eq41_e2308_q) + (s.v[114] * s.db[503][7]));
        let eq41_e2309_q_d_b8: f64 = ((s.db[114][8] * eq41_e2308_q) + (s.v[114] * s.db[503][8]));
        let eq41_e2309_q_d_b9: f64 = ((s.db[114][9] * eq41_e2308_q) + (s.v[114] * s.db[503][9]));
        let eq41_e2309_q_d_b10: f64 = ((s.db[114][10] * eq41_e2308_q) + (s.v[114] * s.db[503][10]));
        let eq41_e2309_q_d_b11: f64 = ((s.db[114][11] * eq41_e2308_q) + (s.v[114] * s.db[503][11]));
        let eq41_e2309_q_d_b12: f64 = ((s.db[114][12] * eq41_e2308_q) + (s.v[114] * s.db[503][12]));
        let eq41_e2309_q_d_b13: f64 = ((s.db[114][13] * eq41_e2308_q) + (s.v[114] * s.db[503][13]));
        let eq41_e2309_q_d_b14: f64 = ((s.db[114][14] * eq41_e2308_q) + (s.v[114] * s.db[503][14]));
        let eq41_e2309_q_d_b15: f64 = ((s.db[114][15] * eq41_e2308_q) + (s.v[114] * s.db[503][15]));
        let eq41_e2309_q_d_b16: f64 = ((s.db[114][16] * eq41_e2308_q) + (s.v[114] * s.db[503][16]));
        let eq41_e2309_q_d_b17: f64 = ((s.db[114][17] * eq41_e2308_q) + (s.v[114] * s.db[503][17]));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n1, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n12, eq41_e2309_d_n13, eq41_e2309_d_n14, eq41_e2309_d_n15, eq41_e2309_d_n16, eq41_e2309_d_b0, eq41_e2309_d_b1, eq41_e2309_d_b2, eq41_e2309_d_b3, eq41_e2309_d_b4, eq41_e2309_d_b5, eq41_e2309_d_b6, eq41_e2309_d_b7, eq41_e2309_d_b8, eq41_e2309_d_b9, eq41_e2309_d_b10, eq41_e2309_d_b11, eq41_e2309_d_b12, eq41_e2309_d_b13, eq41_e2309_d_b14, eq41_e2309_d_b15, eq41_e2309_d_b16, eq41_e2309_d_b17, eq41_e2309_q, eq41_e2309_q_d_n0, eq41_e2309_q_d_n1, eq41_e2309_q_d_n2, eq41_e2309_q_d_n3, eq41_e2309_q_d_n4, eq41_e2309_q_d_n5, eq41_e2309_q_d_n6, eq41_e2309_q_d_n7, eq41_e2309_q_d_n8, eq41_e2309_q_d_n9, eq41_e2309_q_d_n10, eq41_e2309_q_d_n11, eq41_e2309_q_d_n12, eq41_e2309_q_d_n13, eq41_e2309_q_d_n14, eq41_e2309_q_d_n15, eq41_e2309_q_d_n16, eq41_e2309_q_d_b0, eq41_e2309_q_d_b1, eq41_e2309_q_d_b2, eq41_e2309_q_d_b3, eq41_e2309_q_d_b4, eq41_e2309_q_d_b5, eq41_e2309_q_d_b6, eq41_e2309_q_d_b7, eq41_e2309_q_d_b8, eq41_e2309_q_d_b9, eq41_e2309_q_d_b10, eq41_e2309_q_d_b11, eq41_e2309_q_d_b12, eq41_e2309_q_d_b13, eq41_e2309_q_d_b14, eq41_e2309_q_d_b15, eq41_e2309_q_d_b16, eq41_e2309_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 17] = [eq41_e2311_q_d_n0, eq41_e2311_q_d_n1, eq41_e2311_q_d_n2, eq41_e2311_q_d_n3, eq41_e2311_q_d_n4, eq41_e2311_q_d_n5, eq41_e2311_q_d_n6, eq41_e2311_q_d_n7, eq41_e2311_q_d_n8, eq41_e2311_q_d_n9, eq41_e2311_q_d_n10, eq41_e2311_q_d_n11, eq41_e2311_q_d_n12, eq41_e2311_q_d_n13, eq41_e2311_q_d_n14, eq41_e2311_q_d_n15, eq41_e2311_q_d_n16];
        let eq41_reactive_branch_derivatives: [f64; 18] = [eq41_e2311_q_d_b0, eq41_e2311_q_d_b1, eq41_e2311_q_d_b2, eq41_e2311_q_d_b3, eq41_e2311_q_d_b4, eq41_e2311_q_d_b5, eq41_e2311_q_d_b6, eq41_e2311_q_d_b7, eq41_e2311_q_d_b8, eq41_e2311_q_d_b9, eq41_e2311_q_d_b10, eq41_e2311_q_d_b11, eq41_e2311_q_d_b12, eq41_e2311_q_d_b13, eq41_e2311_q_d_b14, eq41_e2311_q_d_b15, eq41_e2311_q_d_b16, eq41_e2311_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16, eq42_e2320_d_b0, eq42_e2320_d_b1, eq42_e2320_d_b2, eq42_e2320_d_b3, eq42_e2320_d_b4, eq42_e2320_d_b5, eq42_e2320_d_b6, eq42_e2320_d_b7, eq42_e2320_d_b8, eq42_e2320_d_b9, eq42_e2320_d_b10, eq42_e2320_d_b11, eq42_e2320_d_b12, eq42_e2320_d_b13, eq42_e2320_d_b14, eq42_e2320_d_b15, eq42_e2320_d_b16, eq42_e2320_d_b17, eq42_e2320_q, eq42_e2320_q_d_n0, eq42_e2320_q_d_n1, eq42_e2320_q_d_n2, eq42_e2320_q_d_n3, eq42_e2320_q_d_n4, eq42_e2320_q_d_n5, eq42_e2320_q_d_n6, eq42_e2320_q_d_n7, eq42_e2320_q_d_n8, eq42_e2320_q_d_n9, eq42_e2320_q_d_n10, eq42_e2320_q_d_n11, eq42_e2320_q_d_n12, eq42_e2320_q_d_n13, eq42_e2320_q_d_n14, eq42_e2320_q_d_n15, eq42_e2320_q_d_n16, eq42_e2320_q_d_b0, eq42_e2320_q_d_b1, eq42_e2320_q_d_b2, eq42_e2320_q_d_b3, eq42_e2320_q_d_b4, eq42_e2320_q_d_b5, eq42_e2320_q_d_b6, eq42_e2320_q_d_b7, eq42_e2320_q_d_b8, eq42_e2320_q_d_b9, eq42_e2320_q_d_b10, eq42_e2320_q_d_b11, eq42_e2320_q_d_b12, eq42_e2320_q_d_b13, eq42_e2320_q_d_b14, eq42_e2320_q_d_b15, eq42_e2320_q_d_b16, eq42_e2320_q_d_b17,) = {
    if (s.b[1705] && s.b[1706]) {
        let eq42_e2317_q: f64 = s.v[504];
        let eq42_e2318: f64 = (s.v[114] * s.v[504]);
        let eq42_e2318_d_n0: f64 = ((s.dn[114][0] * s.v[504]) + (s.v[114] * s.dn[504][0]));
        let eq42_e2318_d_n1: f64 = ((s.dn[114][1] * s.v[504]) + (s.v[114] * s.dn[504][1]));
        let eq42_e2318_d_n2: f64 = ((s.dn[114][2] * s.v[504]) + (s.v[114] * s.dn[504][2]));
        let eq42_e2318_d_n3: f64 = ((s.dn[114][3] * s.v[504]) + (s.v[114] * s.dn[504][3]));
        let eq42_e2318_d_n4: f64 = ((s.dn[114][4] * s.v[504]) + (s.v[114] * s.dn[504][4]));
        let eq42_e2318_d_n5: f64 = ((s.dn[114][5] * s.v[504]) + (s.v[114] * s.dn[504][5]));
        let eq42_e2318_d_n6: f64 = ((s.dn[114][6] * s.v[504]) + (s.v[114] * s.dn[504][6]));
        let eq42_e2318_d_n7: f64 = ((s.dn[114][7] * s.v[504]) + (s.v[114] * s.dn[504][7]));
        let eq42_e2318_d_n8: f64 = ((s.dn[114][8] * s.v[504]) + (s.v[114] * s.dn[504][8]));
        let eq42_e2318_d_n9: f64 = ((s.dn[114][9] * s.v[504]) + (s.v[114] * s.dn[504][9]));
        let eq42_e2318_d_n10: f64 = ((s.dn[114][10] * s.v[504]) + (s.v[114] * s.dn[504][10]));
        let eq42_e2318_d_n11: f64 = ((s.dn[114][11] * s.v[504]) + (s.v[114] * s.dn[504][11]));
        let eq42_e2318_d_n12: f64 = ((s.dn[114][12] * s.v[504]) + (s.v[114] * s.dn[504][12]));
        let eq42_e2318_d_n13: f64 = ((s.dn[114][13] * s.v[504]) + (s.v[114] * s.dn[504][13]));
        let eq42_e2318_d_n14: f64 = ((s.dn[114][14] * s.v[504]) + (s.v[114] * s.dn[504][14]));
        let eq42_e2318_d_n15: f64 = ((s.dn[114][15] * s.v[504]) + (s.v[114] * s.dn[504][15]));
        let eq42_e2318_d_n16: f64 = ((s.dn[114][16] * s.v[504]) + (s.v[114] * s.dn[504][16]));
        let eq42_e2318_d_b0: f64 = ((s.db[114][0] * s.v[504]) + (s.v[114] * s.db[504][0]));
        let eq42_e2318_d_b1: f64 = ((s.db[114][1] * s.v[504]) + (s.v[114] * s.db[504][1]));
        let eq42_e2318_d_b2: f64 = ((s.db[114][2] * s.v[504]) + (s.v[114] * s.db[504][2]));
        let eq42_e2318_d_b3: f64 = ((s.db[114][3] * s.v[504]) + (s.v[114] * s.db[504][3]));
        let eq42_e2318_d_b4: f64 = ((s.db[114][4] * s.v[504]) + (s.v[114] * s.db[504][4]));
        let eq42_e2318_d_b5: f64 = ((s.db[114][5] * s.v[504]) + (s.v[114] * s.db[504][5]));
        let eq42_e2318_d_b6: f64 = ((s.db[114][6] * s.v[504]) + (s.v[114] * s.db[504][6]));
        let eq42_e2318_d_b7: f64 = ((s.db[114][7] * s.v[504]) + (s.v[114] * s.db[504][7]));
        let eq42_e2318_d_b8: f64 = ((s.db[114][8] * s.v[504]) + (s.v[114] * s.db[504][8]));
        let eq42_e2318_d_b9: f64 = ((s.db[114][9] * s.v[504]) + (s.v[114] * s.db[504][9]));
        let eq42_e2318_d_b10: f64 = ((s.db[114][10] * s.v[504]) + (s.v[114] * s.db[504][10]));
        let eq42_e2318_d_b11: f64 = ((s.db[114][11] * s.v[504]) + (s.v[114] * s.db[504][11]));
        let eq42_e2318_d_b12: f64 = ((s.db[114][12] * s.v[504]) + (s.v[114] * s.db[504][12]));
        let eq42_e2318_d_b13: f64 = ((s.db[114][13] * s.v[504]) + (s.v[114] * s.db[504][13]));
        let eq42_e2318_d_b14: f64 = ((s.db[114][14] * s.v[504]) + (s.v[114] * s.db[504][14]));
        let eq42_e2318_d_b15: f64 = ((s.db[114][15] * s.v[504]) + (s.v[114] * s.db[504][15]));
        let eq42_e2318_d_b16: f64 = ((s.db[114][16] * s.v[504]) + (s.v[114] * s.db[504][16]));
        let eq42_e2318_d_b17: f64 = ((s.db[114][17] * s.v[504]) + (s.v[114] * s.db[504][17]));
        let eq42_e2318_q: f64 = (s.v[114] * eq42_e2317_q);
        let eq42_e2318_q_d_n0: f64 = ((s.dn[114][0] * eq42_e2317_q) + (s.v[114] * s.dn[504][0]));
        let eq42_e2318_q_d_n1: f64 = ((s.dn[114][1] * eq42_e2317_q) + (s.v[114] * s.dn[504][1]));
        let eq42_e2318_q_d_n2: f64 = ((s.dn[114][2] * eq42_e2317_q) + (s.v[114] * s.dn[504][2]));
        let eq42_e2318_q_d_n3: f64 = ((s.dn[114][3] * eq42_e2317_q) + (s.v[114] * s.dn[504][3]));
        let eq42_e2318_q_d_n4: f64 = ((s.dn[114][4] * eq42_e2317_q) + (s.v[114] * s.dn[504][4]));
        let eq42_e2318_q_d_n5: f64 = ((s.dn[114][5] * eq42_e2317_q) + (s.v[114] * s.dn[504][5]));
        let eq42_e2318_q_d_n6: f64 = ((s.dn[114][6] * eq42_e2317_q) + (s.v[114] * s.dn[504][6]));
        let eq42_e2318_q_d_n7: f64 = ((s.dn[114][7] * eq42_e2317_q) + (s.v[114] * s.dn[504][7]));
        let eq42_e2318_q_d_n8: f64 = ((s.dn[114][8] * eq42_e2317_q) + (s.v[114] * s.dn[504][8]));
        let eq42_e2318_q_d_n9: f64 = ((s.dn[114][9] * eq42_e2317_q) + (s.v[114] * s.dn[504][9]));
        let eq42_e2318_q_d_n10: f64 = ((s.dn[114][10] * eq42_e2317_q) + (s.v[114] * s.dn[504][10]));
        let eq42_e2318_q_d_n11: f64 = ((s.dn[114][11] * eq42_e2317_q) + (s.v[114] * s.dn[504][11]));
        let eq42_e2318_q_d_n12: f64 = ((s.dn[114][12] * eq42_e2317_q) + (s.v[114] * s.dn[504][12]));
        let eq42_e2318_q_d_n13: f64 = ((s.dn[114][13] * eq42_e2317_q) + (s.v[114] * s.dn[504][13]));
        let eq42_e2318_q_d_n14: f64 = ((s.dn[114][14] * eq42_e2317_q) + (s.v[114] * s.dn[504][14]));
        let eq42_e2318_q_d_n15: f64 = ((s.dn[114][15] * eq42_e2317_q) + (s.v[114] * s.dn[504][15]));
        let eq42_e2318_q_d_n16: f64 = ((s.dn[114][16] * eq42_e2317_q) + (s.v[114] * s.dn[504][16]));
        let eq42_e2318_q_d_b0: f64 = ((s.db[114][0] * eq42_e2317_q) + (s.v[114] * s.db[504][0]));
        let eq42_e2318_q_d_b1: f64 = ((s.db[114][1] * eq42_e2317_q) + (s.v[114] * s.db[504][1]));
        let eq42_e2318_q_d_b2: f64 = ((s.db[114][2] * eq42_e2317_q) + (s.v[114] * s.db[504][2]));
        let eq42_e2318_q_d_b3: f64 = ((s.db[114][3] * eq42_e2317_q) + (s.v[114] * s.db[504][3]));
        let eq42_e2318_q_d_b4: f64 = ((s.db[114][4] * eq42_e2317_q) + (s.v[114] * s.db[504][4]));
        let eq42_e2318_q_d_b5: f64 = ((s.db[114][5] * eq42_e2317_q) + (s.v[114] * s.db[504][5]));
        let eq42_e2318_q_d_b6: f64 = ((s.db[114][6] * eq42_e2317_q) + (s.v[114] * s.db[504][6]));
        let eq42_e2318_q_d_b7: f64 = ((s.db[114][7] * eq42_e2317_q) + (s.v[114] * s.db[504][7]));
        let eq42_e2318_q_d_b8: f64 = ((s.db[114][8] * eq42_e2317_q) + (s.v[114] * s.db[504][8]));
        let eq42_e2318_q_d_b9: f64 = ((s.db[114][9] * eq42_e2317_q) + (s.v[114] * s.db[504][9]));
        let eq42_e2318_q_d_b10: f64 = ((s.db[114][10] * eq42_e2317_q) + (s.v[114] * s.db[504][10]));
        let eq42_e2318_q_d_b11: f64 = ((s.db[114][11] * eq42_e2317_q) + (s.v[114] * s.db[504][11]));
        let eq42_e2318_q_d_b12: f64 = ((s.db[114][12] * eq42_e2317_q) + (s.v[114] * s.db[504][12]));
        let eq42_e2318_q_d_b13: f64 = ((s.db[114][13] * eq42_e2317_q) + (s.v[114] * s.db[504][13]));
        let eq42_e2318_q_d_b14: f64 = ((s.db[114][14] * eq42_e2317_q) + (s.v[114] * s.db[504][14]));
        let eq42_e2318_q_d_b15: f64 = ((s.db[114][15] * eq42_e2317_q) + (s.v[114] * s.db[504][15]));
        let eq42_e2318_q_d_b16: f64 = ((s.db[114][16] * eq42_e2317_q) + (s.v[114] * s.db[504][16]));
        let eq42_e2318_q_d_b17: f64 = ((s.db[114][17] * eq42_e2317_q) + (s.v[114] * s.db[504][17]));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n1, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n12, eq42_e2318_d_n13, eq42_e2318_d_n14, eq42_e2318_d_n15, eq42_e2318_d_n16, eq42_e2318_d_b0, eq42_e2318_d_b1, eq42_e2318_d_b2, eq42_e2318_d_b3, eq42_e2318_d_b4, eq42_e2318_d_b5, eq42_e2318_d_b6, eq42_e2318_d_b7, eq42_e2318_d_b8, eq42_e2318_d_b9, eq42_e2318_d_b10, eq42_e2318_d_b11, eq42_e2318_d_b12, eq42_e2318_d_b13, eq42_e2318_d_b14, eq42_e2318_d_b15, eq42_e2318_d_b16, eq42_e2318_d_b17, eq42_e2318_q, eq42_e2318_q_d_n0, eq42_e2318_q_d_n1, eq42_e2318_q_d_n2, eq42_e2318_q_d_n3, eq42_e2318_q_d_n4, eq42_e2318_q_d_n5, eq42_e2318_q_d_n6, eq42_e2318_q_d_n7, eq42_e2318_q_d_n8, eq42_e2318_q_d_n9, eq42_e2318_q_d_n10, eq42_e2318_q_d_n11, eq42_e2318_q_d_n12, eq42_e2318_q_d_n13, eq42_e2318_q_d_n14, eq42_e2318_q_d_n15, eq42_e2318_q_d_n16, eq42_e2318_q_d_b0, eq42_e2318_q_d_b1, eq42_e2318_q_d_b2, eq42_e2318_q_d_b3, eq42_e2318_q_d_b4, eq42_e2318_q_d_b5, eq42_e2318_q_d_b6, eq42_e2318_q_d_b7, eq42_e2318_q_d_b8, eq42_e2318_q_d_b9, eq42_e2318_q_d_b10, eq42_e2318_q_d_b11, eq42_e2318_q_d_b12, eq42_e2318_q_d_b13, eq42_e2318_q_d_b14, eq42_e2318_q_d_b15, eq42_e2318_q_d_b16, eq42_e2318_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_reactive_node_derivatives: [f64; 17] = [eq42_e2320_q_d_n0, eq42_e2320_q_d_n1, eq42_e2320_q_d_n2, eq42_e2320_q_d_n3, eq42_e2320_q_d_n4, eq42_e2320_q_d_n5, eq42_e2320_q_d_n6, eq42_e2320_q_d_n7, eq42_e2320_q_d_n8, eq42_e2320_q_d_n9, eq42_e2320_q_d_n10, eq42_e2320_q_d_n11, eq42_e2320_q_d_n12, eq42_e2320_q_d_n13, eq42_e2320_q_d_n14, eq42_e2320_q_d_n15, eq42_e2320_q_d_n16];
        let eq42_reactive_branch_derivatives: [f64; 18] = [eq42_e2320_q_d_b0, eq42_e2320_q_d_b1, eq42_e2320_q_d_b2, eq42_e2320_q_d_b3, eq42_e2320_q_d_b4, eq42_e2320_q_d_b5, eq42_e2320_q_d_b6, eq42_e2320_q_d_b7, eq42_e2320_q_d_b8, eq42_e2320_q_d_b9, eq42_e2320_q_d_b10, eq42_e2320_q_d_b11, eq42_e2320_q_d_b12, eq42_e2320_q_d_b13, eq42_e2320_q_d_b14, eq42_e2320_q_d_b15, eq42_e2320_q_d_b16, eq42_e2320_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16, eq43_e2328_d_b0, eq43_e2328_d_b1, eq43_e2328_d_b2, eq43_e2328_d_b3, eq43_e2328_d_b4, eq43_e2328_d_b5, eq43_e2328_d_b6, eq43_e2328_d_b7, eq43_e2328_d_b8, eq43_e2328_d_b9, eq43_e2328_d_b10, eq43_e2328_d_b11, eq43_e2328_d_b12, eq43_e2328_d_b13, eq43_e2328_d_b14, eq43_e2328_d_b15, eq43_e2328_d_b16, eq43_e2328_d_b17, eq43_e2328_q, eq43_e2328_q_d_n0, eq43_e2328_q_d_n1, eq43_e2328_q_d_n2, eq43_e2328_q_d_n3, eq43_e2328_q_d_n4, eq43_e2328_q_d_n5, eq43_e2328_q_d_n6, eq43_e2328_q_d_n7, eq43_e2328_q_d_n8, eq43_e2328_q_d_n9, eq43_e2328_q_d_n10, eq43_e2328_q_d_n11, eq43_e2328_q_d_n12, eq43_e2328_q_d_n13, eq43_e2328_q_d_n14, eq43_e2328_q_d_n15, eq43_e2328_q_d_n16, eq43_e2328_q_d_b0, eq43_e2328_q_d_b1, eq43_e2328_q_d_b2, eq43_e2328_q_d_b3, eq43_e2328_q_d_b4, eq43_e2328_q_d_b5, eq43_e2328_q_d_b6, eq43_e2328_q_d_b7, eq43_e2328_q_d_b8, eq43_e2328_q_d_b9, eq43_e2328_q_d_b10, eq43_e2328_q_d_b11, eq43_e2328_q_d_b12, eq43_e2328_q_d_b13, eq43_e2328_q_d_b14, eq43_e2328_q_d_b15, eq43_e2328_q_d_b16, eq43_e2328_q_d_b17,) = {
    if (s.b[1705] && (!s.b[1706])) {
        let eq43_e2326_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq43_e2326_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_reactive_node_derivatives: [f64; 17] = [eq43_e2328_q_d_n0, eq43_e2328_q_d_n1, eq43_e2328_q_d_n2, eq43_e2328_q_d_n3, eq43_e2328_q_d_n4, eq43_e2328_q_d_n5, eq43_e2328_q_d_n6, eq43_e2328_q_d_n7, eq43_e2328_q_d_n8, eq43_e2328_q_d_n9, eq43_e2328_q_d_n10, eq43_e2328_q_d_n11, eq43_e2328_q_d_n12, eq43_e2328_q_d_n13, eq43_e2328_q_d_n14, eq43_e2328_q_d_n15, eq43_e2328_q_d_n16];
        let eq43_reactive_branch_derivatives: [f64; 18] = [eq43_e2328_q_d_b0, eq43_e2328_q_d_b1, eq43_e2328_q_d_b2, eq43_e2328_q_d_b3, eq43_e2328_q_d_b4, eq43_e2328_q_d_b5, eq43_e2328_q_d_b6, eq43_e2328_q_d_b7, eq43_e2328_q_d_b8, eq43_e2328_q_d_b9, eq43_e2328_q_d_b10, eq43_e2328_q_d_b11, eq43_e2328_q_d_b12, eq43_e2328_q_d_b13, eq43_e2328_q_d_b14, eq43_e2328_q_d_b15, eq43_e2328_q_d_b16, eq43_e2328_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16, eq44_e2333_d_b0, eq44_e2333_d_b1, eq44_e2333_d_b2, eq44_e2333_d_b3, eq44_e2333_d_b4, eq44_e2333_d_b5, eq44_e2333_d_b6, eq44_e2333_d_b7, eq44_e2333_d_b8, eq44_e2333_d_b9, eq44_e2333_d_b10, eq44_e2333_d_b11, eq44_e2333_d_b12, eq44_e2333_d_b13, eq44_e2333_d_b14, eq44_e2333_d_b15, eq44_e2333_d_b16, eq44_e2333_d_b17, eq44_e2333_q, eq44_e2333_q_d_n0, eq44_e2333_q_d_n1, eq44_e2333_q_d_n2, eq44_e2333_q_d_n3, eq44_e2333_q_d_n4, eq44_e2333_q_d_n5, eq44_e2333_q_d_n6, eq44_e2333_q_d_n7, eq44_e2333_q_d_n8, eq44_e2333_q_d_n9, eq44_e2333_q_d_n10, eq44_e2333_q_d_n11, eq44_e2333_q_d_n12, eq44_e2333_q_d_n13, eq44_e2333_q_d_n14, eq44_e2333_q_d_n15, eq44_e2333_q_d_n16, eq44_e2333_q_d_b0, eq44_e2333_q_d_b1, eq44_e2333_q_d_b2, eq44_e2333_q_d_b3, eq44_e2333_q_d_b4, eq44_e2333_q_d_b5, eq44_e2333_q_d_b6, eq44_e2333_q_d_b7, eq44_e2333_q_d_b8, eq44_e2333_q_d_b9, eq44_e2333_q_d_b10, eq44_e2333_q_d_b11, eq44_e2333_q_d_b12, eq44_e2333_q_d_b13, eq44_e2333_q_d_b14, eq44_e2333_q_d_b15, eq44_e2333_q_d_b16, eq44_e2333_q_d_b17,) = {
    if s.b[1705] {
        let eq44_e2331_q: f64 = s.v[502];
        (s.v[502], s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], s.db[502][0], s.db[502][1], s.db[502][2], s.db[502][3], s.db[502][4], s.db[502][5], s.db[502][6], s.db[502][7], s.db[502][8], s.db[502][9], s.db[502][10], s.db[502][11], s.db[502][12], s.db[502][13], s.db[502][14], s.db[502][15], s.db[502][16], s.db[502][17], eq44_e2331_q, s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], s.db[502][0], s.db[502][1], s.db[502][2], s.db[502][3], s.db[502][4], s.db[502][5], s.db[502][6], s.db[502][7], s.db[502][8], s.db[502][9], s.db[502][10], s.db[502][11], s.db[502][12], s.db[502][13], s.db[502][14], s.db[502][15], s.db[502][16], s.db[502][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_reactive_node_derivatives: [f64; 17] = [eq44_e2333_q_d_n0, eq44_e2333_q_d_n1, eq44_e2333_q_d_n2, eq44_e2333_q_d_n3, eq44_e2333_q_d_n4, eq44_e2333_q_d_n5, eq44_e2333_q_d_n6, eq44_e2333_q_d_n7, eq44_e2333_q_d_n8, eq44_e2333_q_d_n9, eq44_e2333_q_d_n10, eq44_e2333_q_d_n11, eq44_e2333_q_d_n12, eq44_e2333_q_d_n13, eq44_e2333_q_d_n14, eq44_e2333_q_d_n15, eq44_e2333_q_d_n16];
        let eq44_reactive_branch_derivatives: [f64; 18] = [eq44_e2333_q_d_b0, eq44_e2333_q_d_b1, eq44_e2333_q_d_b2, eq44_e2333_q_d_b3, eq44_e2333_q_d_b4, eq44_e2333_q_d_b5, eq44_e2333_q_d_b6, eq44_e2333_q_d_b7, eq44_e2333_q_d_b8, eq44_e2333_q_d_b9, eq44_e2333_q_d_b10, eq44_e2333_q_d_b11, eq44_e2333_q_d_b12, eq44_e2333_q_d_b13, eq44_e2333_q_d_b14, eq44_e2333_q_d_b15, eq44_e2333_q_d_b16, eq44_e2333_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16, eq45_e2340_d_b0, eq45_e2340_d_b1, eq45_e2340_d_b2, eq45_e2340_d_b3, eq45_e2340_d_b4, eq45_e2340_d_b5, eq45_e2340_d_b6, eq45_e2340_d_b7, eq45_e2340_d_b8, eq45_e2340_d_b9, eq45_e2340_d_b10, eq45_e2340_d_b11, eq45_e2340_d_b12, eq45_e2340_d_b13, eq45_e2340_d_b14, eq45_e2340_d_b15, eq45_e2340_d_b16, eq45_e2340_d_b17, eq45_e2340_q, eq45_e2340_q_d_n0, eq45_e2340_q_d_n1, eq45_e2340_q_d_n2, eq45_e2340_q_d_n3, eq45_e2340_q_d_n4, eq45_e2340_q_d_n5, eq45_e2340_q_d_n6, eq45_e2340_q_d_n7, eq45_e2340_q_d_n8, eq45_e2340_q_d_n9, eq45_e2340_q_d_n10, eq45_e2340_q_d_n11, eq45_e2340_q_d_n12, eq45_e2340_q_d_n13, eq45_e2340_q_d_n14, eq45_e2340_q_d_n15, eq45_e2340_q_d_n16, eq45_e2340_q_d_b0, eq45_e2340_q_d_b1, eq45_e2340_q_d_b2, eq45_e2340_q_d_b3, eq45_e2340_q_d_b4, eq45_e2340_q_d_b5, eq45_e2340_q_d_b6, eq45_e2340_q_d_b7, eq45_e2340_q_d_b8, eq45_e2340_q_d_b9, eq45_e2340_q_d_b10, eq45_e2340_q_d_b11, eq45_e2340_q_d_b12, eq45_e2340_q_d_b13, eq45_e2340_q_d_b14, eq45_e2340_q_d_b15, eq45_e2340_q_d_b16, eq45_e2340_q_d_b17,) = {
    if (s.b[1705] && s.b[1707]) {
        let eq45_e2338_q: f64 = s.v[500];
        (s.v[500], s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], s.db[500][0], s.db[500][1], s.db[500][2], s.db[500][3], s.db[500][4], s.db[500][5], s.db[500][6], s.db[500][7], s.db[500][8], s.db[500][9], s.db[500][10], s.db[500][11], s.db[500][12], s.db[500][13], s.db[500][14], s.db[500][15], s.db[500][16], s.db[500][17], eq45_e2338_q, s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], s.db[500][0], s.db[500][1], s.db[500][2], s.db[500][3], s.db[500][4], s.db[500][5], s.db[500][6], s.db[500][7], s.db[500][8], s.db[500][9], s.db[500][10], s.db[500][11], s.db[500][12], s.db[500][13], s.db[500][14], s.db[500][15], s.db[500][16], s.db[500][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_reactive_node_derivatives: [f64; 17] = [eq45_e2340_q_d_n0, eq45_e2340_q_d_n1, eq45_e2340_q_d_n2, eq45_e2340_q_d_n3, eq45_e2340_q_d_n4, eq45_e2340_q_d_n5, eq45_e2340_q_d_n6, eq45_e2340_q_d_n7, eq45_e2340_q_d_n8, eq45_e2340_q_d_n9, eq45_e2340_q_d_n10, eq45_e2340_q_d_n11, eq45_e2340_q_d_n12, eq45_e2340_q_d_n13, eq45_e2340_q_d_n14, eq45_e2340_q_d_n15, eq45_e2340_q_d_n16];
        let eq45_reactive_branch_derivatives: [f64; 18] = [eq45_e2340_q_d_b0, eq45_e2340_q_d_b1, eq45_e2340_q_d_b2, eq45_e2340_q_d_b3, eq45_e2340_q_d_b4, eq45_e2340_q_d_b5, eq45_e2340_q_d_b6, eq45_e2340_q_d_b7, eq45_e2340_q_d_b8, eq45_e2340_q_d_b9, eq45_e2340_q_d_b10, eq45_e2340_q_d_b11, eq45_e2340_q_d_b12, eq45_e2340_q_d_b13, eq45_e2340_q_d_b14, eq45_e2340_q_d_b15, eq45_e2340_q_d_b16, eq45_e2340_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16, eq46_e2347_d_b0, eq46_e2347_d_b1, eq46_e2347_d_b2, eq46_e2347_d_b3, eq46_e2347_d_b4, eq46_e2347_d_b5, eq46_e2347_d_b6, eq46_e2347_d_b7, eq46_e2347_d_b8, eq46_e2347_d_b9, eq46_e2347_d_b10, eq46_e2347_d_b11, eq46_e2347_d_b12, eq46_e2347_d_b13, eq46_e2347_d_b14, eq46_e2347_d_b15, eq46_e2347_d_b16, eq46_e2347_d_b17, eq46_e2347_q, eq46_e2347_q_d_n0, eq46_e2347_q_d_n1, eq46_e2347_q_d_n2, eq46_e2347_q_d_n3, eq46_e2347_q_d_n4, eq46_e2347_q_d_n5, eq46_e2347_q_d_n6, eq46_e2347_q_d_n7, eq46_e2347_q_d_n8, eq46_e2347_q_d_n9, eq46_e2347_q_d_n10, eq46_e2347_q_d_n11, eq46_e2347_q_d_n12, eq46_e2347_q_d_n13, eq46_e2347_q_d_n14, eq46_e2347_q_d_n15, eq46_e2347_q_d_n16, eq46_e2347_q_d_b0, eq46_e2347_q_d_b1, eq46_e2347_q_d_b2, eq46_e2347_q_d_b3, eq46_e2347_q_d_b4, eq46_e2347_q_d_b5, eq46_e2347_q_d_b6, eq46_e2347_q_d_b7, eq46_e2347_q_d_b8, eq46_e2347_q_d_b9, eq46_e2347_q_d_b10, eq46_e2347_q_d_b11, eq46_e2347_q_d_b12, eq46_e2347_q_d_b13, eq46_e2347_q_d_b14, eq46_e2347_q_d_b15, eq46_e2347_q_d_b16, eq46_e2347_q_d_b17,) = {
    if (s.b[1705] && s.b[1707]) {
        let eq46_e2345_q: f64 = s.v[501];
        (s.v[501], s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], s.db[501][0], s.db[501][1], s.db[501][2], s.db[501][3], s.db[501][4], s.db[501][5], s.db[501][6], s.db[501][7], s.db[501][8], s.db[501][9], s.db[501][10], s.db[501][11], s.db[501][12], s.db[501][13], s.db[501][14], s.db[501][15], s.db[501][16], s.db[501][17], eq46_e2345_q, s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], s.db[501][0], s.db[501][1], s.db[501][2], s.db[501][3], s.db[501][4], s.db[501][5], s.db[501][6], s.db[501][7], s.db[501][8], s.db[501][9], s.db[501][10], s.db[501][11], s.db[501][12], s.db[501][13], s.db[501][14], s.db[501][15], s.db[501][16], s.db[501][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 17] = [eq46_e2347_q_d_n0, eq46_e2347_q_d_n1, eq46_e2347_q_d_n2, eq46_e2347_q_d_n3, eq46_e2347_q_d_n4, eq46_e2347_q_d_n5, eq46_e2347_q_d_n6, eq46_e2347_q_d_n7, eq46_e2347_q_d_n8, eq46_e2347_q_d_n9, eq46_e2347_q_d_n10, eq46_e2347_q_d_n11, eq46_e2347_q_d_n12, eq46_e2347_q_d_n13, eq46_e2347_q_d_n14, eq46_e2347_q_d_n15, eq46_e2347_q_d_n16];
        let eq46_reactive_branch_derivatives: [f64; 18] = [eq46_e2347_q_d_b0, eq46_e2347_q_d_b1, eq46_e2347_q_d_b2, eq46_e2347_q_d_b3, eq46_e2347_q_d_b4, eq46_e2347_q_d_b5, eq46_e2347_q_d_b6, eq46_e2347_q_d_b7, eq46_e2347_q_d_b8, eq46_e2347_q_d_b9, eq46_e2347_q_d_b10, eq46_e2347_q_d_b11, eq46_e2347_q_d_b12, eq46_e2347_q_d_b13, eq46_e2347_q_d_b14, eq46_e2347_q_d_b15, eq46_e2347_q_d_b16, eq46_e2347_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16, eq47_e2353_d_b0, eq47_e2353_d_b1, eq47_e2353_d_b2, eq47_e2353_d_b3, eq47_e2353_d_b4, eq47_e2353_d_b5, eq47_e2353_d_b6, eq47_e2353_d_b7, eq47_e2353_d_b8, eq47_e2353_d_b9, eq47_e2353_d_b10, eq47_e2353_d_b11, eq47_e2353_d_b12, eq47_e2353_d_b13, eq47_e2353_d_b14, eq47_e2353_d_b15, eq47_e2353_d_b16, eq47_e2353_d_b17, eq47_e2353_q, eq47_e2353_q_d_n0, eq47_e2353_q_d_n1, eq47_e2353_q_d_n2, eq47_e2353_q_d_n3, eq47_e2353_q_d_n4, eq47_e2353_q_d_n5, eq47_e2353_q_d_n6, eq47_e2353_q_d_n7, eq47_e2353_q_d_n8, eq47_e2353_q_d_n9, eq47_e2353_q_d_n10, eq47_e2353_q_d_n11, eq47_e2353_q_d_n12, eq47_e2353_q_d_n13, eq47_e2353_q_d_n14, eq47_e2353_q_d_n15, eq47_e2353_q_d_n16, eq47_e2353_q_d_b0, eq47_e2353_q_d_b1, eq47_e2353_q_d_b2, eq47_e2353_q_d_b3, eq47_e2353_q_d_b4, eq47_e2353_q_d_b5, eq47_e2353_q_d_b6, eq47_e2353_q_d_b7, eq47_e2353_q_d_b8, eq47_e2353_q_d_b9, eq47_e2353_q_d_b10, eq47_e2353_q_d_b11, eq47_e2353_q_d_b12, eq47_e2353_q_d_b13, eq47_e2353_q_d_b14, eq47_e2353_q_d_b15, eq47_e2353_q_d_b16, eq47_e2353_q_d_b17,) = {
    if (!s.b[1705]) {
        let eq47_e2351_q: f64 = s.v[505];
        (s.v[505], s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], s.db[505][0], s.db[505][1], s.db[505][2], s.db[505][3], s.db[505][4], s.db[505][5], s.db[505][6], s.db[505][7], s.db[505][8], s.db[505][9], s.db[505][10], s.db[505][11], s.db[505][12], s.db[505][13], s.db[505][14], s.db[505][15], s.db[505][16], s.db[505][17], eq47_e2351_q, s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], s.db[505][0], s.db[505][1], s.db[505][2], s.db[505][3], s.db[505][4], s.db[505][5], s.db[505][6], s.db[505][7], s.db[505][8], s.db[505][9], s.db[505][10], s.db[505][11], s.db[505][12], s.db[505][13], s.db[505][14], s.db[505][15], s.db[505][16], s.db[505][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 17] = [eq47_e2353_q_d_n0, eq47_e2353_q_d_n1, eq47_e2353_q_d_n2, eq47_e2353_q_d_n3, eq47_e2353_q_d_n4, eq47_e2353_q_d_n5, eq47_e2353_q_d_n6, eq47_e2353_q_d_n7, eq47_e2353_q_d_n8, eq47_e2353_q_d_n9, eq47_e2353_q_d_n10, eq47_e2353_q_d_n11, eq47_e2353_q_d_n12, eq47_e2353_q_d_n13, eq47_e2353_q_d_n14, eq47_e2353_q_d_n15, eq47_e2353_q_d_n16];
        let eq47_reactive_branch_derivatives: [f64; 18] = [eq47_e2353_q_d_b0, eq47_e2353_q_d_b1, eq47_e2353_q_d_b2, eq47_e2353_q_d_b3, eq47_e2353_q_d_b4, eq47_e2353_q_d_b5, eq47_e2353_q_d_b6, eq47_e2353_q_d_b7, eq47_e2353_q_d_b8, eq47_e2353_q_d_b9, eq47_e2353_q_d_b10, eq47_e2353_q_d_b11, eq47_e2353_q_d_b12, eq47_e2353_q_d_b13, eq47_e2353_q_d_b14, eq47_e2353_q_d_b15, eq47_e2353_q_d_b16, eq47_e2353_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16, eq48_e2361_d_b0, eq48_e2361_d_b1, eq48_e2361_d_b2, eq48_e2361_d_b3, eq48_e2361_d_b4, eq48_e2361_d_b5, eq48_e2361_d_b6, eq48_e2361_d_b7, eq48_e2361_d_b8, eq48_e2361_d_b9, eq48_e2361_d_b10, eq48_e2361_d_b11, eq48_e2361_d_b12, eq48_e2361_d_b13, eq48_e2361_d_b14, eq48_e2361_d_b15, eq48_e2361_d_b16, eq48_e2361_d_b17, eq48_e2361_q, eq48_e2361_q_d_n0, eq48_e2361_q_d_n1, eq48_e2361_q_d_n2, eq48_e2361_q_d_n3, eq48_e2361_q_d_n4, eq48_e2361_q_d_n5, eq48_e2361_q_d_n6, eq48_e2361_q_d_n7, eq48_e2361_q_d_n8, eq48_e2361_q_d_n9, eq48_e2361_q_d_n10, eq48_e2361_q_d_n11, eq48_e2361_q_d_n12, eq48_e2361_q_d_n13, eq48_e2361_q_d_n14, eq48_e2361_q_d_n15, eq48_e2361_q_d_n16, eq48_e2361_q_d_b0, eq48_e2361_q_d_b1, eq48_e2361_q_d_b2, eq48_e2361_q_d_b3, eq48_e2361_q_d_b4, eq48_e2361_q_d_b5, eq48_e2361_q_d_b6, eq48_e2361_q_d_b7, eq48_e2361_q_d_b8, eq48_e2361_q_d_b9, eq48_e2361_q_d_b10, eq48_e2361_q_d_b11, eq48_e2361_q_d_b12, eq48_e2361_q_d_b13, eq48_e2361_q_d_b14, eq48_e2361_q_d_b15, eq48_e2361_q_d_b16, eq48_e2361_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq48_e2359_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq48_e2359_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 17] = [eq48_e2361_q_d_n0, eq48_e2361_q_d_n1, eq48_e2361_q_d_n2, eq48_e2361_q_d_n3, eq48_e2361_q_d_n4, eq48_e2361_q_d_n5, eq48_e2361_q_d_n6, eq48_e2361_q_d_n7, eq48_e2361_q_d_n8, eq48_e2361_q_d_n9, eq48_e2361_q_d_n10, eq48_e2361_q_d_n11, eq48_e2361_q_d_n12, eq48_e2361_q_d_n13, eq48_e2361_q_d_n14, eq48_e2361_q_d_n15, eq48_e2361_q_d_n16];
        let eq48_reactive_branch_derivatives: [f64; 18] = [eq48_e2361_q_d_b0, eq48_e2361_q_d_b1, eq48_e2361_q_d_b2, eq48_e2361_q_d_b3, eq48_e2361_q_d_b4, eq48_e2361_q_d_b5, eq48_e2361_q_d_b6, eq48_e2361_q_d_b7, eq48_e2361_q_d_b8, eq48_e2361_q_d_b9, eq48_e2361_q_d_b10, eq48_e2361_q_d_b11, eq48_e2361_q_d_b12, eq48_e2361_q_d_b13, eq48_e2361_q_d_b14, eq48_e2361_q_d_b15, eq48_e2361_q_d_b16, eq48_e2361_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16, eq49_e2371_d_b0, eq49_e2371_d_b1, eq49_e2371_d_b2, eq49_e2371_d_b3, eq49_e2371_d_b4, eq49_e2371_d_b5, eq49_e2371_d_b6, eq49_e2371_d_b7, eq49_e2371_d_b8, eq49_e2371_d_b9, eq49_e2371_d_b10, eq49_e2371_d_b11, eq49_e2371_d_b12, eq49_e2371_d_b13, eq49_e2371_d_b14, eq49_e2371_d_b15, eq49_e2371_d_b16, eq49_e2371_d_b17, eq49_e2371_q, eq49_e2371_q_d_n0, eq49_e2371_q_d_n1, eq49_e2371_q_d_n2, eq49_e2371_q_d_n3, eq49_e2371_q_d_n4, eq49_e2371_q_d_n5, eq49_e2371_q_d_n6, eq49_e2371_q_d_n7, eq49_e2371_q_d_n8, eq49_e2371_q_d_n9, eq49_e2371_q_d_n10, eq49_e2371_q_d_n11, eq49_e2371_q_d_n12, eq49_e2371_q_d_n13, eq49_e2371_q_d_n14, eq49_e2371_q_d_n15, eq49_e2371_q_d_n16, eq49_e2371_q_d_b0, eq49_e2371_q_d_b1, eq49_e2371_q_d_b2, eq49_e2371_q_d_b3, eq49_e2371_q_d_b4, eq49_e2371_q_d_b5, eq49_e2371_q_d_b6, eq49_e2371_q_d_b7, eq49_e2371_q_d_b8, eq49_e2371_q_d_b9, eq49_e2371_q_d_b10, eq49_e2371_q_d_b11, eq49_e2371_q_d_b12, eq49_e2371_q_d_b13, eq49_e2371_q_d_b14, eq49_e2371_q_d_b15, eq49_e2371_q_d_b16, eq49_e2371_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq49_e2368_q: f64 = s.v[503];
        let eq49_e2369: f64 = (s.v[114] * s.v[503]);
        let eq49_e2369_d_n0: f64 = ((s.dn[114][0] * s.v[503]) + (s.v[114] * s.dn[503][0]));
        let eq49_e2369_d_n1: f64 = ((s.dn[114][1] * s.v[503]) + (s.v[114] * s.dn[503][1]));
        let eq49_e2369_d_n2: f64 = ((s.dn[114][2] * s.v[503]) + (s.v[114] * s.dn[503][2]));
        let eq49_e2369_d_n3: f64 = ((s.dn[114][3] * s.v[503]) + (s.v[114] * s.dn[503][3]));
        let eq49_e2369_d_n4: f64 = ((s.dn[114][4] * s.v[503]) + (s.v[114] * s.dn[503][4]));
        let eq49_e2369_d_n5: f64 = ((s.dn[114][5] * s.v[503]) + (s.v[114] * s.dn[503][5]));
        let eq49_e2369_d_n6: f64 = ((s.dn[114][6] * s.v[503]) + (s.v[114] * s.dn[503][6]));
        let eq49_e2369_d_n7: f64 = ((s.dn[114][7] * s.v[503]) + (s.v[114] * s.dn[503][7]));
        let eq49_e2369_d_n8: f64 = ((s.dn[114][8] * s.v[503]) + (s.v[114] * s.dn[503][8]));
        let eq49_e2369_d_n9: f64 = ((s.dn[114][9] * s.v[503]) + (s.v[114] * s.dn[503][9]));
        let eq49_e2369_d_n10: f64 = ((s.dn[114][10] * s.v[503]) + (s.v[114] * s.dn[503][10]));
        let eq49_e2369_d_n11: f64 = ((s.dn[114][11] * s.v[503]) + (s.v[114] * s.dn[503][11]));
        let eq49_e2369_d_n12: f64 = ((s.dn[114][12] * s.v[503]) + (s.v[114] * s.dn[503][12]));
        let eq49_e2369_d_n13: f64 = ((s.dn[114][13] * s.v[503]) + (s.v[114] * s.dn[503][13]));
        let eq49_e2369_d_n14: f64 = ((s.dn[114][14] * s.v[503]) + (s.v[114] * s.dn[503][14]));
        let eq49_e2369_d_n15: f64 = ((s.dn[114][15] * s.v[503]) + (s.v[114] * s.dn[503][15]));
        let eq49_e2369_d_n16: f64 = ((s.dn[114][16] * s.v[503]) + (s.v[114] * s.dn[503][16]));
        let eq49_e2369_d_b0: f64 = ((s.db[114][0] * s.v[503]) + (s.v[114] * s.db[503][0]));
        let eq49_e2369_d_b1: f64 = ((s.db[114][1] * s.v[503]) + (s.v[114] * s.db[503][1]));
        let eq49_e2369_d_b2: f64 = ((s.db[114][2] * s.v[503]) + (s.v[114] * s.db[503][2]));
        let eq49_e2369_d_b3: f64 = ((s.db[114][3] * s.v[503]) + (s.v[114] * s.db[503][3]));
        let eq49_e2369_d_b4: f64 = ((s.db[114][4] * s.v[503]) + (s.v[114] * s.db[503][4]));
        let eq49_e2369_d_b5: f64 = ((s.db[114][5] * s.v[503]) + (s.v[114] * s.db[503][5]));
        let eq49_e2369_d_b6: f64 = ((s.db[114][6] * s.v[503]) + (s.v[114] * s.db[503][6]));
        let eq49_e2369_d_b7: f64 = ((s.db[114][7] * s.v[503]) + (s.v[114] * s.db[503][7]));
        let eq49_e2369_d_b8: f64 = ((s.db[114][8] * s.v[503]) + (s.v[114] * s.db[503][8]));
        let eq49_e2369_d_b9: f64 = ((s.db[114][9] * s.v[503]) + (s.v[114] * s.db[503][9]));
        let eq49_e2369_d_b10: f64 = ((s.db[114][10] * s.v[503]) + (s.v[114] * s.db[503][10]));
        let eq49_e2369_d_b11: f64 = ((s.db[114][11] * s.v[503]) + (s.v[114] * s.db[503][11]));
        let eq49_e2369_d_b12: f64 = ((s.db[114][12] * s.v[503]) + (s.v[114] * s.db[503][12]));
        let eq49_e2369_d_b13: f64 = ((s.db[114][13] * s.v[503]) + (s.v[114] * s.db[503][13]));
        let eq49_e2369_d_b14: f64 = ((s.db[114][14] * s.v[503]) + (s.v[114] * s.db[503][14]));
        let eq49_e2369_d_b15: f64 = ((s.db[114][15] * s.v[503]) + (s.v[114] * s.db[503][15]));
        let eq49_e2369_d_b16: f64 = ((s.db[114][16] * s.v[503]) + (s.v[114] * s.db[503][16]));
        let eq49_e2369_d_b17: f64 = ((s.db[114][17] * s.v[503]) + (s.v[114] * s.db[503][17]));
        let eq49_e2369_q: f64 = (s.v[114] * eq49_e2368_q);
        let eq49_e2369_q_d_n0: f64 = ((s.dn[114][0] * eq49_e2368_q) + (s.v[114] * s.dn[503][0]));
        let eq49_e2369_q_d_n1: f64 = ((s.dn[114][1] * eq49_e2368_q) + (s.v[114] * s.dn[503][1]));
        let eq49_e2369_q_d_n2: f64 = ((s.dn[114][2] * eq49_e2368_q) + (s.v[114] * s.dn[503][2]));
        let eq49_e2369_q_d_n3: f64 = ((s.dn[114][3] * eq49_e2368_q) + (s.v[114] * s.dn[503][3]));
        let eq49_e2369_q_d_n4: f64 = ((s.dn[114][4] * eq49_e2368_q) + (s.v[114] * s.dn[503][4]));
        let eq49_e2369_q_d_n5: f64 = ((s.dn[114][5] * eq49_e2368_q) + (s.v[114] * s.dn[503][5]));
        let eq49_e2369_q_d_n6: f64 = ((s.dn[114][6] * eq49_e2368_q) + (s.v[114] * s.dn[503][6]));
        let eq49_e2369_q_d_n7: f64 = ((s.dn[114][7] * eq49_e2368_q) + (s.v[114] * s.dn[503][7]));
        let eq49_e2369_q_d_n8: f64 = ((s.dn[114][8] * eq49_e2368_q) + (s.v[114] * s.dn[503][8]));
        let eq49_e2369_q_d_n9: f64 = ((s.dn[114][9] * eq49_e2368_q) + (s.v[114] * s.dn[503][9]));
        let eq49_e2369_q_d_n10: f64 = ((s.dn[114][10] * eq49_e2368_q) + (s.v[114] * s.dn[503][10]));
        let eq49_e2369_q_d_n11: f64 = ((s.dn[114][11] * eq49_e2368_q) + (s.v[114] * s.dn[503][11]));
        let eq49_e2369_q_d_n12: f64 = ((s.dn[114][12] * eq49_e2368_q) + (s.v[114] * s.dn[503][12]));
        let eq49_e2369_q_d_n13: f64 = ((s.dn[114][13] * eq49_e2368_q) + (s.v[114] * s.dn[503][13]));
        let eq49_e2369_q_d_n14: f64 = ((s.dn[114][14] * eq49_e2368_q) + (s.v[114] * s.dn[503][14]));
        let eq49_e2369_q_d_n15: f64 = ((s.dn[114][15] * eq49_e2368_q) + (s.v[114] * s.dn[503][15]));
        let eq49_e2369_q_d_n16: f64 = ((s.dn[114][16] * eq49_e2368_q) + (s.v[114] * s.dn[503][16]));
        let eq49_e2369_q_d_b0: f64 = ((s.db[114][0] * eq49_e2368_q) + (s.v[114] * s.db[503][0]));
        let eq49_e2369_q_d_b1: f64 = ((s.db[114][1] * eq49_e2368_q) + (s.v[114] * s.db[503][1]));
        let eq49_e2369_q_d_b2: f64 = ((s.db[114][2] * eq49_e2368_q) + (s.v[114] * s.db[503][2]));
        let eq49_e2369_q_d_b3: f64 = ((s.db[114][3] * eq49_e2368_q) + (s.v[114] * s.db[503][3]));
        let eq49_e2369_q_d_b4: f64 = ((s.db[114][4] * eq49_e2368_q) + (s.v[114] * s.db[503][4]));
        let eq49_e2369_q_d_b5: f64 = ((s.db[114][5] * eq49_e2368_q) + (s.v[114] * s.db[503][5]));
        let eq49_e2369_q_d_b6: f64 = ((s.db[114][6] * eq49_e2368_q) + (s.v[114] * s.db[503][6]));
        let eq49_e2369_q_d_b7: f64 = ((s.db[114][7] * eq49_e2368_q) + (s.v[114] * s.db[503][7]));
        let eq49_e2369_q_d_b8: f64 = ((s.db[114][8] * eq49_e2368_q) + (s.v[114] * s.db[503][8]));
        let eq49_e2369_q_d_b9: f64 = ((s.db[114][9] * eq49_e2368_q) + (s.v[114] * s.db[503][9]));
        let eq49_e2369_q_d_b10: f64 = ((s.db[114][10] * eq49_e2368_q) + (s.v[114] * s.db[503][10]));
        let eq49_e2369_q_d_b11: f64 = ((s.db[114][11] * eq49_e2368_q) + (s.v[114] * s.db[503][11]));
        let eq49_e2369_q_d_b12: f64 = ((s.db[114][12] * eq49_e2368_q) + (s.v[114] * s.db[503][12]));
        let eq49_e2369_q_d_b13: f64 = ((s.db[114][13] * eq49_e2368_q) + (s.v[114] * s.db[503][13]));
        let eq49_e2369_q_d_b14: f64 = ((s.db[114][14] * eq49_e2368_q) + (s.v[114] * s.db[503][14]));
        let eq49_e2369_q_d_b15: f64 = ((s.db[114][15] * eq49_e2368_q) + (s.v[114] * s.db[503][15]));
        let eq49_e2369_q_d_b16: f64 = ((s.db[114][16] * eq49_e2368_q) + (s.v[114] * s.db[503][16]));
        let eq49_e2369_q_d_b17: f64 = ((s.db[114][17] * eq49_e2368_q) + (s.v[114] * s.db[503][17]));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n1, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n12, eq49_e2369_d_n13, eq49_e2369_d_n14, eq49_e2369_d_n15, eq49_e2369_d_n16, eq49_e2369_d_b0, eq49_e2369_d_b1, eq49_e2369_d_b2, eq49_e2369_d_b3, eq49_e2369_d_b4, eq49_e2369_d_b5, eq49_e2369_d_b6, eq49_e2369_d_b7, eq49_e2369_d_b8, eq49_e2369_d_b9, eq49_e2369_d_b10, eq49_e2369_d_b11, eq49_e2369_d_b12, eq49_e2369_d_b13, eq49_e2369_d_b14, eq49_e2369_d_b15, eq49_e2369_d_b16, eq49_e2369_d_b17, eq49_e2369_q, eq49_e2369_q_d_n0, eq49_e2369_q_d_n1, eq49_e2369_q_d_n2, eq49_e2369_q_d_n3, eq49_e2369_q_d_n4, eq49_e2369_q_d_n5, eq49_e2369_q_d_n6, eq49_e2369_q_d_n7, eq49_e2369_q_d_n8, eq49_e2369_q_d_n9, eq49_e2369_q_d_n10, eq49_e2369_q_d_n11, eq49_e2369_q_d_n12, eq49_e2369_q_d_n13, eq49_e2369_q_d_n14, eq49_e2369_q_d_n15, eq49_e2369_q_d_n16, eq49_e2369_q_d_b0, eq49_e2369_q_d_b1, eq49_e2369_q_d_b2, eq49_e2369_q_d_b3, eq49_e2369_q_d_b4, eq49_e2369_q_d_b5, eq49_e2369_q_d_b6, eq49_e2369_q_d_b7, eq49_e2369_q_d_b8, eq49_e2369_q_d_b9, eq49_e2369_q_d_b10, eq49_e2369_q_d_b11, eq49_e2369_q_d_b12, eq49_e2369_q_d_b13, eq49_e2369_q_d_b14, eq49_e2369_q_d_b15, eq49_e2369_q_d_b16, eq49_e2369_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_reactive_node_derivatives: [f64; 17] = [eq49_e2371_q_d_n0, eq49_e2371_q_d_n1, eq49_e2371_q_d_n2, eq49_e2371_q_d_n3, eq49_e2371_q_d_n4, eq49_e2371_q_d_n5, eq49_e2371_q_d_n6, eq49_e2371_q_d_n7, eq49_e2371_q_d_n8, eq49_e2371_q_d_n9, eq49_e2371_q_d_n10, eq49_e2371_q_d_n11, eq49_e2371_q_d_n12, eq49_e2371_q_d_n13, eq49_e2371_q_d_n14, eq49_e2371_q_d_n15, eq49_e2371_q_d_n16];
        let eq49_reactive_branch_derivatives: [f64; 18] = [eq49_e2371_q_d_b0, eq49_e2371_q_d_b1, eq49_e2371_q_d_b2, eq49_e2371_q_d_b3, eq49_e2371_q_d_b4, eq49_e2371_q_d_b5, eq49_e2371_q_d_b6, eq49_e2371_q_d_b7, eq49_e2371_q_d_b8, eq49_e2371_q_d_b9, eq49_e2371_q_d_b10, eq49_e2371_q_d_b11, eq49_e2371_q_d_b12, eq49_e2371_q_d_b13, eq49_e2371_q_d_b14, eq49_e2371_q_d_b15, eq49_e2371_q_d_b16, eq49_e2371_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16, eq50_e2381_d_b0, eq50_e2381_d_b1, eq50_e2381_d_b2, eq50_e2381_d_b3, eq50_e2381_d_b4, eq50_e2381_d_b5, eq50_e2381_d_b6, eq50_e2381_d_b7, eq50_e2381_d_b8, eq50_e2381_d_b9, eq50_e2381_d_b10, eq50_e2381_d_b11, eq50_e2381_d_b12, eq50_e2381_d_b13, eq50_e2381_d_b14, eq50_e2381_d_b15, eq50_e2381_d_b16, eq50_e2381_d_b17, eq50_e2381_q, eq50_e2381_q_d_n0, eq50_e2381_q_d_n1, eq50_e2381_q_d_n2, eq50_e2381_q_d_n3, eq50_e2381_q_d_n4, eq50_e2381_q_d_n5, eq50_e2381_q_d_n6, eq50_e2381_q_d_n7, eq50_e2381_q_d_n8, eq50_e2381_q_d_n9, eq50_e2381_q_d_n10, eq50_e2381_q_d_n11, eq50_e2381_q_d_n12, eq50_e2381_q_d_n13, eq50_e2381_q_d_n14, eq50_e2381_q_d_n15, eq50_e2381_q_d_n16, eq50_e2381_q_d_b0, eq50_e2381_q_d_b1, eq50_e2381_q_d_b2, eq50_e2381_q_d_b3, eq50_e2381_q_d_b4, eq50_e2381_q_d_b5, eq50_e2381_q_d_b6, eq50_e2381_q_d_b7, eq50_e2381_q_d_b8, eq50_e2381_q_d_b9, eq50_e2381_q_d_b10, eq50_e2381_q_d_b11, eq50_e2381_q_d_b12, eq50_e2381_q_d_b13, eq50_e2381_q_d_b14, eq50_e2381_q_d_b15, eq50_e2381_q_d_b16, eq50_e2381_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1708]) {
        let eq50_e2378_q: f64 = s.v[504];
        let eq50_e2379: f64 = (s.v[114] * s.v[504]);
        let eq50_e2379_d_n0: f64 = ((s.dn[114][0] * s.v[504]) + (s.v[114] * s.dn[504][0]));
        let eq50_e2379_d_n1: f64 = ((s.dn[114][1] * s.v[504]) + (s.v[114] * s.dn[504][1]));
        let eq50_e2379_d_n2: f64 = ((s.dn[114][2] * s.v[504]) + (s.v[114] * s.dn[504][2]));
        let eq50_e2379_d_n3: f64 = ((s.dn[114][3] * s.v[504]) + (s.v[114] * s.dn[504][3]));
        let eq50_e2379_d_n4: f64 = ((s.dn[114][4] * s.v[504]) + (s.v[114] * s.dn[504][4]));
        let eq50_e2379_d_n5: f64 = ((s.dn[114][5] * s.v[504]) + (s.v[114] * s.dn[504][5]));
        let eq50_e2379_d_n6: f64 = ((s.dn[114][6] * s.v[504]) + (s.v[114] * s.dn[504][6]));
        let eq50_e2379_d_n7: f64 = ((s.dn[114][7] * s.v[504]) + (s.v[114] * s.dn[504][7]));
        let eq50_e2379_d_n8: f64 = ((s.dn[114][8] * s.v[504]) + (s.v[114] * s.dn[504][8]));
        let eq50_e2379_d_n9: f64 = ((s.dn[114][9] * s.v[504]) + (s.v[114] * s.dn[504][9]));
        let eq50_e2379_d_n10: f64 = ((s.dn[114][10] * s.v[504]) + (s.v[114] * s.dn[504][10]));
        let eq50_e2379_d_n11: f64 = ((s.dn[114][11] * s.v[504]) + (s.v[114] * s.dn[504][11]));
        let eq50_e2379_d_n12: f64 = ((s.dn[114][12] * s.v[504]) + (s.v[114] * s.dn[504][12]));
        let eq50_e2379_d_n13: f64 = ((s.dn[114][13] * s.v[504]) + (s.v[114] * s.dn[504][13]));
        let eq50_e2379_d_n14: f64 = ((s.dn[114][14] * s.v[504]) + (s.v[114] * s.dn[504][14]));
        let eq50_e2379_d_n15: f64 = ((s.dn[114][15] * s.v[504]) + (s.v[114] * s.dn[504][15]));
        let eq50_e2379_d_n16: f64 = ((s.dn[114][16] * s.v[504]) + (s.v[114] * s.dn[504][16]));
        let eq50_e2379_d_b0: f64 = ((s.db[114][0] * s.v[504]) + (s.v[114] * s.db[504][0]));
        let eq50_e2379_d_b1: f64 = ((s.db[114][1] * s.v[504]) + (s.v[114] * s.db[504][1]));
        let eq50_e2379_d_b2: f64 = ((s.db[114][2] * s.v[504]) + (s.v[114] * s.db[504][2]));
        let eq50_e2379_d_b3: f64 = ((s.db[114][3] * s.v[504]) + (s.v[114] * s.db[504][3]));
        let eq50_e2379_d_b4: f64 = ((s.db[114][4] * s.v[504]) + (s.v[114] * s.db[504][4]));
        let eq50_e2379_d_b5: f64 = ((s.db[114][5] * s.v[504]) + (s.v[114] * s.db[504][5]));
        let eq50_e2379_d_b6: f64 = ((s.db[114][6] * s.v[504]) + (s.v[114] * s.db[504][6]));
        let eq50_e2379_d_b7: f64 = ((s.db[114][7] * s.v[504]) + (s.v[114] * s.db[504][7]));
        let eq50_e2379_d_b8: f64 = ((s.db[114][8] * s.v[504]) + (s.v[114] * s.db[504][8]));
        let eq50_e2379_d_b9: f64 = ((s.db[114][9] * s.v[504]) + (s.v[114] * s.db[504][9]));
        let eq50_e2379_d_b10: f64 = ((s.db[114][10] * s.v[504]) + (s.v[114] * s.db[504][10]));
        let eq50_e2379_d_b11: f64 = ((s.db[114][11] * s.v[504]) + (s.v[114] * s.db[504][11]));
        let eq50_e2379_d_b12: f64 = ((s.db[114][12] * s.v[504]) + (s.v[114] * s.db[504][12]));
        let eq50_e2379_d_b13: f64 = ((s.db[114][13] * s.v[504]) + (s.v[114] * s.db[504][13]));
        let eq50_e2379_d_b14: f64 = ((s.db[114][14] * s.v[504]) + (s.v[114] * s.db[504][14]));
        let eq50_e2379_d_b15: f64 = ((s.db[114][15] * s.v[504]) + (s.v[114] * s.db[504][15]));
        let eq50_e2379_d_b16: f64 = ((s.db[114][16] * s.v[504]) + (s.v[114] * s.db[504][16]));
        let eq50_e2379_d_b17: f64 = ((s.db[114][17] * s.v[504]) + (s.v[114] * s.db[504][17]));
        let eq50_e2379_q: f64 = (s.v[114] * eq50_e2378_q);
        let eq50_e2379_q_d_n0: f64 = ((s.dn[114][0] * eq50_e2378_q) + (s.v[114] * s.dn[504][0]));
        let eq50_e2379_q_d_n1: f64 = ((s.dn[114][1] * eq50_e2378_q) + (s.v[114] * s.dn[504][1]));
        let eq50_e2379_q_d_n2: f64 = ((s.dn[114][2] * eq50_e2378_q) + (s.v[114] * s.dn[504][2]));
        let eq50_e2379_q_d_n3: f64 = ((s.dn[114][3] * eq50_e2378_q) + (s.v[114] * s.dn[504][3]));
        let eq50_e2379_q_d_n4: f64 = ((s.dn[114][4] * eq50_e2378_q) + (s.v[114] * s.dn[504][4]));
        let eq50_e2379_q_d_n5: f64 = ((s.dn[114][5] * eq50_e2378_q) + (s.v[114] * s.dn[504][5]));
        let eq50_e2379_q_d_n6: f64 = ((s.dn[114][6] * eq50_e2378_q) + (s.v[114] * s.dn[504][6]));
        let eq50_e2379_q_d_n7: f64 = ((s.dn[114][7] * eq50_e2378_q) + (s.v[114] * s.dn[504][7]));
        let eq50_e2379_q_d_n8: f64 = ((s.dn[114][8] * eq50_e2378_q) + (s.v[114] * s.dn[504][8]));
        let eq50_e2379_q_d_n9: f64 = ((s.dn[114][9] * eq50_e2378_q) + (s.v[114] * s.dn[504][9]));
        let eq50_e2379_q_d_n10: f64 = ((s.dn[114][10] * eq50_e2378_q) + (s.v[114] * s.dn[504][10]));
        let eq50_e2379_q_d_n11: f64 = ((s.dn[114][11] * eq50_e2378_q) + (s.v[114] * s.dn[504][11]));
        let eq50_e2379_q_d_n12: f64 = ((s.dn[114][12] * eq50_e2378_q) + (s.v[114] * s.dn[504][12]));
        let eq50_e2379_q_d_n13: f64 = ((s.dn[114][13] * eq50_e2378_q) + (s.v[114] * s.dn[504][13]));
        let eq50_e2379_q_d_n14: f64 = ((s.dn[114][14] * eq50_e2378_q) + (s.v[114] * s.dn[504][14]));
        let eq50_e2379_q_d_n15: f64 = ((s.dn[114][15] * eq50_e2378_q) + (s.v[114] * s.dn[504][15]));
        let eq50_e2379_q_d_n16: f64 = ((s.dn[114][16] * eq50_e2378_q) + (s.v[114] * s.dn[504][16]));
        let eq50_e2379_q_d_b0: f64 = ((s.db[114][0] * eq50_e2378_q) + (s.v[114] * s.db[504][0]));
        let eq50_e2379_q_d_b1: f64 = ((s.db[114][1] * eq50_e2378_q) + (s.v[114] * s.db[504][1]));
        let eq50_e2379_q_d_b2: f64 = ((s.db[114][2] * eq50_e2378_q) + (s.v[114] * s.db[504][2]));
        let eq50_e2379_q_d_b3: f64 = ((s.db[114][3] * eq50_e2378_q) + (s.v[114] * s.db[504][3]));
        let eq50_e2379_q_d_b4: f64 = ((s.db[114][4] * eq50_e2378_q) + (s.v[114] * s.db[504][4]));
        let eq50_e2379_q_d_b5: f64 = ((s.db[114][5] * eq50_e2378_q) + (s.v[114] * s.db[504][5]));
        let eq50_e2379_q_d_b6: f64 = ((s.db[114][6] * eq50_e2378_q) + (s.v[114] * s.db[504][6]));
        let eq50_e2379_q_d_b7: f64 = ((s.db[114][7] * eq50_e2378_q) + (s.v[114] * s.db[504][7]));
        let eq50_e2379_q_d_b8: f64 = ((s.db[114][8] * eq50_e2378_q) + (s.v[114] * s.db[504][8]));
        let eq50_e2379_q_d_b9: f64 = ((s.db[114][9] * eq50_e2378_q) + (s.v[114] * s.db[504][9]));
        let eq50_e2379_q_d_b10: f64 = ((s.db[114][10] * eq50_e2378_q) + (s.v[114] * s.db[504][10]));
        let eq50_e2379_q_d_b11: f64 = ((s.db[114][11] * eq50_e2378_q) + (s.v[114] * s.db[504][11]));
        let eq50_e2379_q_d_b12: f64 = ((s.db[114][12] * eq50_e2378_q) + (s.v[114] * s.db[504][12]));
        let eq50_e2379_q_d_b13: f64 = ((s.db[114][13] * eq50_e2378_q) + (s.v[114] * s.db[504][13]));
        let eq50_e2379_q_d_b14: f64 = ((s.db[114][14] * eq50_e2378_q) + (s.v[114] * s.db[504][14]));
        let eq50_e2379_q_d_b15: f64 = ((s.db[114][15] * eq50_e2378_q) + (s.v[114] * s.db[504][15]));
        let eq50_e2379_q_d_b16: f64 = ((s.db[114][16] * eq50_e2378_q) + (s.v[114] * s.db[504][16]));
        let eq50_e2379_q_d_b17: f64 = ((s.db[114][17] * eq50_e2378_q) + (s.v[114] * s.db[504][17]));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n1, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n12, eq50_e2379_d_n13, eq50_e2379_d_n14, eq50_e2379_d_n15, eq50_e2379_d_n16, eq50_e2379_d_b0, eq50_e2379_d_b1, eq50_e2379_d_b2, eq50_e2379_d_b3, eq50_e2379_d_b4, eq50_e2379_d_b5, eq50_e2379_d_b6, eq50_e2379_d_b7, eq50_e2379_d_b8, eq50_e2379_d_b9, eq50_e2379_d_b10, eq50_e2379_d_b11, eq50_e2379_d_b12, eq50_e2379_d_b13, eq50_e2379_d_b14, eq50_e2379_d_b15, eq50_e2379_d_b16, eq50_e2379_d_b17, eq50_e2379_q, eq50_e2379_q_d_n0, eq50_e2379_q_d_n1, eq50_e2379_q_d_n2, eq50_e2379_q_d_n3, eq50_e2379_q_d_n4, eq50_e2379_q_d_n5, eq50_e2379_q_d_n6, eq50_e2379_q_d_n7, eq50_e2379_q_d_n8, eq50_e2379_q_d_n9, eq50_e2379_q_d_n10, eq50_e2379_q_d_n11, eq50_e2379_q_d_n12, eq50_e2379_q_d_n13, eq50_e2379_q_d_n14, eq50_e2379_q_d_n15, eq50_e2379_q_d_n16, eq50_e2379_q_d_b0, eq50_e2379_q_d_b1, eq50_e2379_q_d_b2, eq50_e2379_q_d_b3, eq50_e2379_q_d_b4, eq50_e2379_q_d_b5, eq50_e2379_q_d_b6, eq50_e2379_q_d_b7, eq50_e2379_q_d_b8, eq50_e2379_q_d_b9, eq50_e2379_q_d_b10, eq50_e2379_q_d_b11, eq50_e2379_q_d_b12, eq50_e2379_q_d_b13, eq50_e2379_q_d_b14, eq50_e2379_q_d_b15, eq50_e2379_q_d_b16, eq50_e2379_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 17] = [eq50_e2381_q_d_n0, eq50_e2381_q_d_n1, eq50_e2381_q_d_n2, eq50_e2381_q_d_n3, eq50_e2381_q_d_n4, eq50_e2381_q_d_n5, eq50_e2381_q_d_n6, eq50_e2381_q_d_n7, eq50_e2381_q_d_n8, eq50_e2381_q_d_n9, eq50_e2381_q_d_n10, eq50_e2381_q_d_n11, eq50_e2381_q_d_n12, eq50_e2381_q_d_n13, eq50_e2381_q_d_n14, eq50_e2381_q_d_n15, eq50_e2381_q_d_n16];
        let eq50_reactive_branch_derivatives: [f64; 18] = [eq50_e2381_q_d_b0, eq50_e2381_q_d_b1, eq50_e2381_q_d_b2, eq50_e2381_q_d_b3, eq50_e2381_q_d_b4, eq50_e2381_q_d_b5, eq50_e2381_q_d_b6, eq50_e2381_q_d_b7, eq50_e2381_q_d_b8, eq50_e2381_q_d_b9, eq50_e2381_q_d_b10, eq50_e2381_q_d_b11, eq50_e2381_q_d_b12, eq50_e2381_q_d_b13, eq50_e2381_q_d_b14, eq50_e2381_q_d_b15, eq50_e2381_q_d_b16, eq50_e2381_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16, eq51_e2390_d_b0, eq51_e2390_d_b1, eq51_e2390_d_b2, eq51_e2390_d_b3, eq51_e2390_d_b4, eq51_e2390_d_b5, eq51_e2390_d_b6, eq51_e2390_d_b7, eq51_e2390_d_b8, eq51_e2390_d_b9, eq51_e2390_d_b10, eq51_e2390_d_b11, eq51_e2390_d_b12, eq51_e2390_d_b13, eq51_e2390_d_b14, eq51_e2390_d_b15, eq51_e2390_d_b16, eq51_e2390_d_b17, eq51_e2390_q, eq51_e2390_q_d_n0, eq51_e2390_q_d_n1, eq51_e2390_q_d_n2, eq51_e2390_q_d_n3, eq51_e2390_q_d_n4, eq51_e2390_q_d_n5, eq51_e2390_q_d_n6, eq51_e2390_q_d_n7, eq51_e2390_q_d_n8, eq51_e2390_q_d_n9, eq51_e2390_q_d_n10, eq51_e2390_q_d_n11, eq51_e2390_q_d_n12, eq51_e2390_q_d_n13, eq51_e2390_q_d_n14, eq51_e2390_q_d_n15, eq51_e2390_q_d_n16, eq51_e2390_q_d_b0, eq51_e2390_q_d_b1, eq51_e2390_q_d_b2, eq51_e2390_q_d_b3, eq51_e2390_q_d_b4, eq51_e2390_q_d_b5, eq51_e2390_q_d_b6, eq51_e2390_q_d_b7, eq51_e2390_q_d_b8, eq51_e2390_q_d_b9, eq51_e2390_q_d_b10, eq51_e2390_q_d_b11, eq51_e2390_q_d_b12, eq51_e2390_q_d_b13, eq51_e2390_q_d_b14, eq51_e2390_q_d_b15, eq51_e2390_q_d_b16, eq51_e2390_q_d_b17,) = {
    if ((!s.b[1705]) && (!s.b[1708])) {
        let eq51_e2388_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17], eq51_e2388_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], s.db[506][0], s.db[506][1], s.db[506][2], s.db[506][3], s.db[506][4], s.db[506][5], s.db[506][6], s.db[506][7], s.db[506][8], s.db[506][9], s.db[506][10], s.db[506][11], s.db[506][12], s.db[506][13], s.db[506][14], s.db[506][15], s.db[506][16], s.db[506][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 17] = [eq51_e2390_q_d_n0, eq51_e2390_q_d_n1, eq51_e2390_q_d_n2, eq51_e2390_q_d_n3, eq51_e2390_q_d_n4, eq51_e2390_q_d_n5, eq51_e2390_q_d_n6, eq51_e2390_q_d_n7, eq51_e2390_q_d_n8, eq51_e2390_q_d_n9, eq51_e2390_q_d_n10, eq51_e2390_q_d_n11, eq51_e2390_q_d_n12, eq51_e2390_q_d_n13, eq51_e2390_q_d_n14, eq51_e2390_q_d_n15, eq51_e2390_q_d_n16];
        let eq51_reactive_branch_derivatives: [f64; 18] = [eq51_e2390_q_d_b0, eq51_e2390_q_d_b1, eq51_e2390_q_d_b2, eq51_e2390_q_d_b3, eq51_e2390_q_d_b4, eq51_e2390_q_d_b5, eq51_e2390_q_d_b6, eq51_e2390_q_d_b7, eq51_e2390_q_d_b8, eq51_e2390_q_d_b9, eq51_e2390_q_d_b10, eq51_e2390_q_d_b11, eq51_e2390_q_d_b12, eq51_e2390_q_d_b13, eq51_e2390_q_d_b14, eq51_e2390_q_d_b15, eq51_e2390_q_d_b16, eq51_e2390_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16, eq52_e2396_d_b0, eq52_e2396_d_b1, eq52_e2396_d_b2, eq52_e2396_d_b3, eq52_e2396_d_b4, eq52_e2396_d_b5, eq52_e2396_d_b6, eq52_e2396_d_b7, eq52_e2396_d_b8, eq52_e2396_d_b9, eq52_e2396_d_b10, eq52_e2396_d_b11, eq52_e2396_d_b12, eq52_e2396_d_b13, eq52_e2396_d_b14, eq52_e2396_d_b15, eq52_e2396_d_b16, eq52_e2396_d_b17, eq52_e2396_q, eq52_e2396_q_d_n0, eq52_e2396_q_d_n1, eq52_e2396_q_d_n2, eq52_e2396_q_d_n3, eq52_e2396_q_d_n4, eq52_e2396_q_d_n5, eq52_e2396_q_d_n6, eq52_e2396_q_d_n7, eq52_e2396_q_d_n8, eq52_e2396_q_d_n9, eq52_e2396_q_d_n10, eq52_e2396_q_d_n11, eq52_e2396_q_d_n12, eq52_e2396_q_d_n13, eq52_e2396_q_d_n14, eq52_e2396_q_d_n15, eq52_e2396_q_d_n16, eq52_e2396_q_d_b0, eq52_e2396_q_d_b1, eq52_e2396_q_d_b2, eq52_e2396_q_d_b3, eq52_e2396_q_d_b4, eq52_e2396_q_d_b5, eq52_e2396_q_d_b6, eq52_e2396_q_d_b7, eq52_e2396_q_d_b8, eq52_e2396_q_d_b9, eq52_e2396_q_d_b10, eq52_e2396_q_d_b11, eq52_e2396_q_d_b12, eq52_e2396_q_d_b13, eq52_e2396_q_d_b14, eq52_e2396_q_d_b15, eq52_e2396_q_d_b16, eq52_e2396_q_d_b17,) = {
    if (!s.b[1705]) {
        let eq52_e2394_q: f64 = s.v[502];
        (s.v[502], s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], s.db[502][0], s.db[502][1], s.db[502][2], s.db[502][3], s.db[502][4], s.db[502][5], s.db[502][6], s.db[502][7], s.db[502][8], s.db[502][9], s.db[502][10], s.db[502][11], s.db[502][12], s.db[502][13], s.db[502][14], s.db[502][15], s.db[502][16], s.db[502][17], eq52_e2394_q, s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], s.db[502][0], s.db[502][1], s.db[502][2], s.db[502][3], s.db[502][4], s.db[502][5], s.db[502][6], s.db[502][7], s.db[502][8], s.db[502][9], s.db[502][10], s.db[502][11], s.db[502][12], s.db[502][13], s.db[502][14], s.db[502][15], s.db[502][16], s.db[502][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 17] = [eq52_e2396_q_d_n0, eq52_e2396_q_d_n1, eq52_e2396_q_d_n2, eq52_e2396_q_d_n3, eq52_e2396_q_d_n4, eq52_e2396_q_d_n5, eq52_e2396_q_d_n6, eq52_e2396_q_d_n7, eq52_e2396_q_d_n8, eq52_e2396_q_d_n9, eq52_e2396_q_d_n10, eq52_e2396_q_d_n11, eq52_e2396_q_d_n12, eq52_e2396_q_d_n13, eq52_e2396_q_d_n14, eq52_e2396_q_d_n15, eq52_e2396_q_d_n16];
        let eq52_reactive_branch_derivatives: [f64; 18] = [eq52_e2396_q_d_b0, eq52_e2396_q_d_b1, eq52_e2396_q_d_b2, eq52_e2396_q_d_b3, eq52_e2396_q_d_b4, eq52_e2396_q_d_b5, eq52_e2396_q_d_b6, eq52_e2396_q_d_b7, eq52_e2396_q_d_b8, eq52_e2396_q_d_b9, eq52_e2396_q_d_b10, eq52_e2396_q_d_b11, eq52_e2396_q_d_b12, eq52_e2396_q_d_b13, eq52_e2396_q_d_b14, eq52_e2396_q_d_b15, eq52_e2396_q_d_b16, eq52_e2396_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16, eq53_e2404_d_b0, eq53_e2404_d_b1, eq53_e2404_d_b2, eq53_e2404_d_b3, eq53_e2404_d_b4, eq53_e2404_d_b5, eq53_e2404_d_b6, eq53_e2404_d_b7, eq53_e2404_d_b8, eq53_e2404_d_b9, eq53_e2404_d_b10, eq53_e2404_d_b11, eq53_e2404_d_b12, eq53_e2404_d_b13, eq53_e2404_d_b14, eq53_e2404_d_b15, eq53_e2404_d_b16, eq53_e2404_d_b17, eq53_e2404_q, eq53_e2404_q_d_n0, eq53_e2404_q_d_n1, eq53_e2404_q_d_n2, eq53_e2404_q_d_n3, eq53_e2404_q_d_n4, eq53_e2404_q_d_n5, eq53_e2404_q_d_n6, eq53_e2404_q_d_n7, eq53_e2404_q_d_n8, eq53_e2404_q_d_n9, eq53_e2404_q_d_n10, eq53_e2404_q_d_n11, eq53_e2404_q_d_n12, eq53_e2404_q_d_n13, eq53_e2404_q_d_n14, eq53_e2404_q_d_n15, eq53_e2404_q_d_n16, eq53_e2404_q_d_b0, eq53_e2404_q_d_b1, eq53_e2404_q_d_b2, eq53_e2404_q_d_b3, eq53_e2404_q_d_b4, eq53_e2404_q_d_b5, eq53_e2404_q_d_b6, eq53_e2404_q_d_b7, eq53_e2404_q_d_b8, eq53_e2404_q_d_b9, eq53_e2404_q_d_b10, eq53_e2404_q_d_b11, eq53_e2404_q_d_b12, eq53_e2404_q_d_b13, eq53_e2404_q_d_b14, eq53_e2404_q_d_b15, eq53_e2404_q_d_b16, eq53_e2404_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1709]) {
        let eq53_e2402_q: f64 = s.v[500];
        (s.v[500], s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], s.db[500][0], s.db[500][1], s.db[500][2], s.db[500][3], s.db[500][4], s.db[500][5], s.db[500][6], s.db[500][7], s.db[500][8], s.db[500][9], s.db[500][10], s.db[500][11], s.db[500][12], s.db[500][13], s.db[500][14], s.db[500][15], s.db[500][16], s.db[500][17], eq53_e2402_q, s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], s.db[500][0], s.db[500][1], s.db[500][2], s.db[500][3], s.db[500][4], s.db[500][5], s.db[500][6], s.db[500][7], s.db[500][8], s.db[500][9], s.db[500][10], s.db[500][11], s.db[500][12], s.db[500][13], s.db[500][14], s.db[500][15], s.db[500][16], s.db[500][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 17] = [eq53_e2404_q_d_n0, eq53_e2404_q_d_n1, eq53_e2404_q_d_n2, eq53_e2404_q_d_n3, eq53_e2404_q_d_n4, eq53_e2404_q_d_n5, eq53_e2404_q_d_n6, eq53_e2404_q_d_n7, eq53_e2404_q_d_n8, eq53_e2404_q_d_n9, eq53_e2404_q_d_n10, eq53_e2404_q_d_n11, eq53_e2404_q_d_n12, eq53_e2404_q_d_n13, eq53_e2404_q_d_n14, eq53_e2404_q_d_n15, eq53_e2404_q_d_n16];
        let eq53_reactive_branch_derivatives: [f64; 18] = [eq53_e2404_q_d_b0, eq53_e2404_q_d_b1, eq53_e2404_q_d_b2, eq53_e2404_q_d_b3, eq53_e2404_q_d_b4, eq53_e2404_q_d_b5, eq53_e2404_q_d_b6, eq53_e2404_q_d_b7, eq53_e2404_q_d_b8, eq53_e2404_q_d_b9, eq53_e2404_q_d_b10, eq53_e2404_q_d_b11, eq53_e2404_q_d_b12, eq53_e2404_q_d_b13, eq53_e2404_q_d_b14, eq53_e2404_q_d_b15, eq53_e2404_q_d_b16, eq53_e2404_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[2]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16, eq54_e2412_d_b0, eq54_e2412_d_b1, eq54_e2412_d_b2, eq54_e2412_d_b3, eq54_e2412_d_b4, eq54_e2412_d_b5, eq54_e2412_d_b6, eq54_e2412_d_b7, eq54_e2412_d_b8, eq54_e2412_d_b9, eq54_e2412_d_b10, eq54_e2412_d_b11, eq54_e2412_d_b12, eq54_e2412_d_b13, eq54_e2412_d_b14, eq54_e2412_d_b15, eq54_e2412_d_b16, eq54_e2412_d_b17, eq54_e2412_q, eq54_e2412_q_d_n0, eq54_e2412_q_d_n1, eq54_e2412_q_d_n2, eq54_e2412_q_d_n3, eq54_e2412_q_d_n4, eq54_e2412_q_d_n5, eq54_e2412_q_d_n6, eq54_e2412_q_d_n7, eq54_e2412_q_d_n8, eq54_e2412_q_d_n9, eq54_e2412_q_d_n10, eq54_e2412_q_d_n11, eq54_e2412_q_d_n12, eq54_e2412_q_d_n13, eq54_e2412_q_d_n14, eq54_e2412_q_d_n15, eq54_e2412_q_d_n16, eq54_e2412_q_d_b0, eq54_e2412_q_d_b1, eq54_e2412_q_d_b2, eq54_e2412_q_d_b3, eq54_e2412_q_d_b4, eq54_e2412_q_d_b5, eq54_e2412_q_d_b6, eq54_e2412_q_d_b7, eq54_e2412_q_d_b8, eq54_e2412_q_d_b9, eq54_e2412_q_d_b10, eq54_e2412_q_d_b11, eq54_e2412_q_d_b12, eq54_e2412_q_d_b13, eq54_e2412_q_d_b14, eq54_e2412_q_d_b15, eq54_e2412_q_d_b16, eq54_e2412_q_d_b17,) = {
    if ((!s.b[1705]) && s.b[1709]) {
        let eq54_e2410_q: f64 = s.v[501];
        (s.v[501], s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], s.db[501][0], s.db[501][1], s.db[501][2], s.db[501][3], s.db[501][4], s.db[501][5], s.db[501][6], s.db[501][7], s.db[501][8], s.db[501][9], s.db[501][10], s.db[501][11], s.db[501][12], s.db[501][13], s.db[501][14], s.db[501][15], s.db[501][16], s.db[501][17], eq54_e2410_q, s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], s.db[501][0], s.db[501][1], s.db[501][2], s.db[501][3], s.db[501][4], s.db[501][5], s.db[501][6], s.db[501][7], s.db[501][8], s.db[501][9], s.db[501][10], s.db[501][11], s.db[501][12], s.db[501][13], s.db[501][14], s.db[501][15], s.db[501][16], s.db[501][17],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 17] = [eq54_e2412_q_d_n0, eq54_e2412_q_d_n1, eq54_e2412_q_d_n2, eq54_e2412_q_d_n3, eq54_e2412_q_d_n4, eq54_e2412_q_d_n5, eq54_e2412_q_d_n6, eq54_e2412_q_d_n7, eq54_e2412_q_d_n8, eq54_e2412_q_d_n9, eq54_e2412_q_d_n10, eq54_e2412_q_d_n11, eq54_e2412_q_d_n12, eq54_e2412_q_d_n13, eq54_e2412_q_d_n14, eq54_e2412_q_d_n15, eq54_e2412_q_d_n16];
        let eq54_reactive_branch_derivatives: [f64; 18] = [eq54_e2412_q_d_b0, eq54_e2412_q_d_b1, eq54_e2412_q_d_b2, eq54_e2412_q_d_b3, eq54_e2412_q_d_b4, eq54_e2412_q_d_b5, eq54_e2412_q_d_b6, eq54_e2412_q_d_b7, eq54_e2412_q_d_b8, eq54_e2412_q_d_b9, eq54_e2412_q_d_b10, eq54_e2412_q_d_b11, eq54_e2412_q_d_b12, eq54_e2412_q_d_b13, eq54_e2412_q_d_b14, eq54_e2412_q_d_b15, eq54_e2412_q_d_b16, eq54_e2412_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16, eq55_e2419_d_b0, eq55_e2419_d_b1, eq55_e2419_d_b2, eq55_e2419_d_b3, eq55_e2419_d_b4, eq55_e2419_d_b5, eq55_e2419_d_b6, eq55_e2419_d_b7, eq55_e2419_d_b8, eq55_e2419_d_b9, eq55_e2419_d_b10, eq55_e2419_d_b11, eq55_e2419_d_b12, eq55_e2419_d_b13, eq55_e2419_d_b14, eq55_e2419_d_b15, eq55_e2419_d_b16, eq55_e2419_d_b17, eq55_e2419_q, eq55_e2419_q_d_n0, eq55_e2419_q_d_n1, eq55_e2419_q_d_n2, eq55_e2419_q_d_n3, eq55_e2419_q_d_n4, eq55_e2419_q_d_n5, eq55_e2419_q_d_n6, eq55_e2419_q_d_n7, eq55_e2419_q_d_n8, eq55_e2419_q_d_n9, eq55_e2419_q_d_n10, eq55_e2419_q_d_n11, eq55_e2419_q_d_n12, eq55_e2419_q_d_n13, eq55_e2419_q_d_n14, eq55_e2419_q_d_n15, eq55_e2419_q_d_n16, eq55_e2419_q_d_b0, eq55_e2419_q_d_b1, eq55_e2419_q_d_b2, eq55_e2419_q_d_b3, eq55_e2419_q_d_b4, eq55_e2419_q_d_b5, eq55_e2419_q_d_b6, eq55_e2419_q_d_b7, eq55_e2419_q_d_b8, eq55_e2419_q_d_b9, eq55_e2419_q_d_b10, eq55_e2419_q_d_b11, eq55_e2419_q_d_b12, eq55_e2419_q_d_b13, eq55_e2419_q_d_b14, eq55_e2419_q_d_b15, eq55_e2419_q_d_b16, eq55_e2419_q_d_b17,) = {
    if s.b[1710] {
        let eq55_e2416_q: f64 = s.v[495];
        let eq55_e2417: f64 = (s.v[114] * s.v[495]);
        let eq55_e2417_d_n0: f64 = ((s.dn[114][0] * s.v[495]) + (s.v[114] * s.dn[495][0]));
        let eq55_e2417_d_n1: f64 = ((s.dn[114][1] * s.v[495]) + (s.v[114] * s.dn[495][1]));
        let eq55_e2417_d_n2: f64 = ((s.dn[114][2] * s.v[495]) + (s.v[114] * s.dn[495][2]));
        let eq55_e2417_d_n3: f64 = ((s.dn[114][3] * s.v[495]) + (s.v[114] * s.dn[495][3]));
        let eq55_e2417_d_n4: f64 = ((s.dn[114][4] * s.v[495]) + (s.v[114] * s.dn[495][4]));
        let eq55_e2417_d_n5: f64 = ((s.dn[114][5] * s.v[495]) + (s.v[114] * s.dn[495][5]));
        let eq55_e2417_d_n6: f64 = ((s.dn[114][6] * s.v[495]) + (s.v[114] * s.dn[495][6]));
        let eq55_e2417_d_n7: f64 = ((s.dn[114][7] * s.v[495]) + (s.v[114] * s.dn[495][7]));
        let eq55_e2417_d_n8: f64 = ((s.dn[114][8] * s.v[495]) + (s.v[114] * s.dn[495][8]));
        let eq55_e2417_d_n9: f64 = ((s.dn[114][9] * s.v[495]) + (s.v[114] * s.dn[495][9]));
        let eq55_e2417_d_n10: f64 = ((s.dn[114][10] * s.v[495]) + (s.v[114] * s.dn[495][10]));
        let eq55_e2417_d_n11: f64 = ((s.dn[114][11] * s.v[495]) + (s.v[114] * s.dn[495][11]));
        let eq55_e2417_d_n12: f64 = ((s.dn[114][12] * s.v[495]) + (s.v[114] * s.dn[495][12]));
        let eq55_e2417_d_n13: f64 = ((s.dn[114][13] * s.v[495]) + (s.v[114] * s.dn[495][13]));
        let eq55_e2417_d_n14: f64 = ((s.dn[114][14] * s.v[495]) + (s.v[114] * s.dn[495][14]));
        let eq55_e2417_d_n15: f64 = ((s.dn[114][15] * s.v[495]) + (s.v[114] * s.dn[495][15]));
        let eq55_e2417_d_n16: f64 = ((s.dn[114][16] * s.v[495]) + (s.v[114] * s.dn[495][16]));
        let eq55_e2417_d_b0: f64 = ((s.db[114][0] * s.v[495]) + (s.v[114] * s.db[495][0]));
        let eq55_e2417_d_b1: f64 = ((s.db[114][1] * s.v[495]) + (s.v[114] * s.db[495][1]));
        let eq55_e2417_d_b2: f64 = ((s.db[114][2] * s.v[495]) + (s.v[114] * s.db[495][2]));
        let eq55_e2417_d_b3: f64 = ((s.db[114][3] * s.v[495]) + (s.v[114] * s.db[495][3]));
        let eq55_e2417_d_b4: f64 = ((s.db[114][4] * s.v[495]) + (s.v[114] * s.db[495][4]));
        let eq55_e2417_d_b5: f64 = ((s.db[114][5] * s.v[495]) + (s.v[114] * s.db[495][5]));
        let eq55_e2417_d_b6: f64 = ((s.db[114][6] * s.v[495]) + (s.v[114] * s.db[495][6]));
        let eq55_e2417_d_b7: f64 = ((s.db[114][7] * s.v[495]) + (s.v[114] * s.db[495][7]));
        let eq55_e2417_d_b8: f64 = ((s.db[114][8] * s.v[495]) + (s.v[114] * s.db[495][8]));
        let eq55_e2417_d_b9: f64 = ((s.db[114][9] * s.v[495]) + (s.v[114] * s.db[495][9]));
        let eq55_e2417_d_b10: f64 = ((s.db[114][10] * s.v[495]) + (s.v[114] * s.db[495][10]));
        let eq55_e2417_d_b11: f64 = ((s.db[114][11] * s.v[495]) + (s.v[114] * s.db[495][11]));
        let eq55_e2417_d_b12: f64 = ((s.db[114][12] * s.v[495]) + (s.v[114] * s.db[495][12]));
        let eq55_e2417_d_b13: f64 = ((s.db[114][13] * s.v[495]) + (s.v[114] * s.db[495][13]));
        let eq55_e2417_d_b14: f64 = ((s.db[114][14] * s.v[495]) + (s.v[114] * s.db[495][14]));
        let eq55_e2417_d_b15: f64 = ((s.db[114][15] * s.v[495]) + (s.v[114] * s.db[495][15]));
        let eq55_e2417_d_b16: f64 = ((s.db[114][16] * s.v[495]) + (s.v[114] * s.db[495][16]));
        let eq55_e2417_d_b17: f64 = ((s.db[114][17] * s.v[495]) + (s.v[114] * s.db[495][17]));
        let eq55_e2417_q: f64 = (s.v[114] * eq55_e2416_q);
        let eq55_e2417_q_d_n0: f64 = ((s.dn[114][0] * eq55_e2416_q) + (s.v[114] * s.dn[495][0]));
        let eq55_e2417_q_d_n1: f64 = ((s.dn[114][1] * eq55_e2416_q) + (s.v[114] * s.dn[495][1]));
        let eq55_e2417_q_d_n2: f64 = ((s.dn[114][2] * eq55_e2416_q) + (s.v[114] * s.dn[495][2]));
        let eq55_e2417_q_d_n3: f64 = ((s.dn[114][3] * eq55_e2416_q) + (s.v[114] * s.dn[495][3]));
        let eq55_e2417_q_d_n4: f64 = ((s.dn[114][4] * eq55_e2416_q) + (s.v[114] * s.dn[495][4]));
        let eq55_e2417_q_d_n5: f64 = ((s.dn[114][5] * eq55_e2416_q) + (s.v[114] * s.dn[495][5]));
        let eq55_e2417_q_d_n6: f64 = ((s.dn[114][6] * eq55_e2416_q) + (s.v[114] * s.dn[495][6]));
        let eq55_e2417_q_d_n7: f64 = ((s.dn[114][7] * eq55_e2416_q) + (s.v[114] * s.dn[495][7]));
        let eq55_e2417_q_d_n8: f64 = ((s.dn[114][8] * eq55_e2416_q) + (s.v[114] * s.dn[495][8]));
        let eq55_e2417_q_d_n9: f64 = ((s.dn[114][9] * eq55_e2416_q) + (s.v[114] * s.dn[495][9]));
        let eq55_e2417_q_d_n10: f64 = ((s.dn[114][10] * eq55_e2416_q) + (s.v[114] * s.dn[495][10]));
        let eq55_e2417_q_d_n11: f64 = ((s.dn[114][11] * eq55_e2416_q) + (s.v[114] * s.dn[495][11]));
        let eq55_e2417_q_d_n12: f64 = ((s.dn[114][12] * eq55_e2416_q) + (s.v[114] * s.dn[495][12]));
        let eq55_e2417_q_d_n13: f64 = ((s.dn[114][13] * eq55_e2416_q) + (s.v[114] * s.dn[495][13]));
        let eq55_e2417_q_d_n14: f64 = ((s.dn[114][14] * eq55_e2416_q) + (s.v[114] * s.dn[495][14]));
        let eq55_e2417_q_d_n15: f64 = ((s.dn[114][15] * eq55_e2416_q) + (s.v[114] * s.dn[495][15]));
        let eq55_e2417_q_d_n16: f64 = ((s.dn[114][16] * eq55_e2416_q) + (s.v[114] * s.dn[495][16]));
        let eq55_e2417_q_d_b0: f64 = ((s.db[114][0] * eq55_e2416_q) + (s.v[114] * s.db[495][0]));
        let eq55_e2417_q_d_b1: f64 = ((s.db[114][1] * eq55_e2416_q) + (s.v[114] * s.db[495][1]));
        let eq55_e2417_q_d_b2: f64 = ((s.db[114][2] * eq55_e2416_q) + (s.v[114] * s.db[495][2]));
        let eq55_e2417_q_d_b3: f64 = ((s.db[114][3] * eq55_e2416_q) + (s.v[114] * s.db[495][3]));
        let eq55_e2417_q_d_b4: f64 = ((s.db[114][4] * eq55_e2416_q) + (s.v[114] * s.db[495][4]));
        let eq55_e2417_q_d_b5: f64 = ((s.db[114][5] * eq55_e2416_q) + (s.v[114] * s.db[495][5]));
        let eq55_e2417_q_d_b6: f64 = ((s.db[114][6] * eq55_e2416_q) + (s.v[114] * s.db[495][6]));
        let eq55_e2417_q_d_b7: f64 = ((s.db[114][7] * eq55_e2416_q) + (s.v[114] * s.db[495][7]));
        let eq55_e2417_q_d_b8: f64 = ((s.db[114][8] * eq55_e2416_q) + (s.v[114] * s.db[495][8]));
        let eq55_e2417_q_d_b9: f64 = ((s.db[114][9] * eq55_e2416_q) + (s.v[114] * s.db[495][9]));
        let eq55_e2417_q_d_b10: f64 = ((s.db[114][10] * eq55_e2416_q) + (s.v[114] * s.db[495][10]));
        let eq55_e2417_q_d_b11: f64 = ((s.db[114][11] * eq55_e2416_q) + (s.v[114] * s.db[495][11]));
        let eq55_e2417_q_d_b12: f64 = ((s.db[114][12] * eq55_e2416_q) + (s.v[114] * s.db[495][12]));
        let eq55_e2417_q_d_b13: f64 = ((s.db[114][13] * eq55_e2416_q) + (s.v[114] * s.db[495][13]));
        let eq55_e2417_q_d_b14: f64 = ((s.db[114][14] * eq55_e2416_q) + (s.v[114] * s.db[495][14]));
        let eq55_e2417_q_d_b15: f64 = ((s.db[114][15] * eq55_e2416_q) + (s.v[114] * s.db[495][15]));
        let eq55_e2417_q_d_b16: f64 = ((s.db[114][16] * eq55_e2416_q) + (s.v[114] * s.db[495][16]));
        let eq55_e2417_q_d_b17: f64 = ((s.db[114][17] * eq55_e2416_q) + (s.v[114] * s.db[495][17]));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n1, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n12, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_d_n15, eq55_e2417_d_n16, eq55_e2417_d_b0, eq55_e2417_d_b1, eq55_e2417_d_b2, eq55_e2417_d_b3, eq55_e2417_d_b4, eq55_e2417_d_b5, eq55_e2417_d_b6, eq55_e2417_d_b7, eq55_e2417_d_b8, eq55_e2417_d_b9, eq55_e2417_d_b10, eq55_e2417_d_b11, eq55_e2417_d_b12, eq55_e2417_d_b13, eq55_e2417_d_b14, eq55_e2417_d_b15, eq55_e2417_d_b16, eq55_e2417_d_b17, eq55_e2417_q, eq55_e2417_q_d_n0, eq55_e2417_q_d_n1, eq55_e2417_q_d_n2, eq55_e2417_q_d_n3, eq55_e2417_q_d_n4, eq55_e2417_q_d_n5, eq55_e2417_q_d_n6, eq55_e2417_q_d_n7, eq55_e2417_q_d_n8, eq55_e2417_q_d_n9, eq55_e2417_q_d_n10, eq55_e2417_q_d_n11, eq55_e2417_q_d_n12, eq55_e2417_q_d_n13, eq55_e2417_q_d_n14, eq55_e2417_q_d_n15, eq55_e2417_q_d_n16, eq55_e2417_q_d_b0, eq55_e2417_q_d_b1, eq55_e2417_q_d_b2, eq55_e2417_q_d_b3, eq55_e2417_q_d_b4, eq55_e2417_q_d_b5, eq55_e2417_q_d_b6, eq55_e2417_q_d_b7, eq55_e2417_q_d_b8, eq55_e2417_q_d_b9, eq55_e2417_q_d_b10, eq55_e2417_q_d_b11, eq55_e2417_q_d_b12, eq55_e2417_q_d_b13, eq55_e2417_q_d_b14, eq55_e2417_q_d_b15, eq55_e2417_q_d_b16, eq55_e2417_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e2419_q_d_n0, eq55_e2419_q_d_n1, eq55_e2419_q_d_n2, eq55_e2419_q_d_n3, eq55_e2419_q_d_n4, eq55_e2419_q_d_n5, eq55_e2419_q_d_n6, eq55_e2419_q_d_n7, eq55_e2419_q_d_n8, eq55_e2419_q_d_n9, eq55_e2419_q_d_n10, eq55_e2419_q_d_n11, eq55_e2419_q_d_n12, eq55_e2419_q_d_n13, eq55_e2419_q_d_n14, eq55_e2419_q_d_n15, eq55_e2419_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 18] = [eq55_e2419_q_d_b0, eq55_e2419_q_d_b1, eq55_e2419_q_d_b2, eq55_e2419_q_d_b3, eq55_e2419_q_d_b4, eq55_e2419_q_d_b5, eq55_e2419_q_d_b6, eq55_e2419_q_d_b7, eq55_e2419_q_d_b8, eq55_e2419_q_d_b9, eq55_e2419_q_d_b10, eq55_e2419_q_d_b11, eq55_e2419_q_d_b12, eq55_e2419_q_d_b13, eq55_e2419_q_d_b14, eq55_e2419_q_d_b15, eq55_e2419_q_d_b16, eq55_e2419_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16, eq56_e2426_d_b0, eq56_e2426_d_b1, eq56_e2426_d_b2, eq56_e2426_d_b3, eq56_e2426_d_b4, eq56_e2426_d_b5, eq56_e2426_d_b6, eq56_e2426_d_b7, eq56_e2426_d_b8, eq56_e2426_d_b9, eq56_e2426_d_b10, eq56_e2426_d_b11, eq56_e2426_d_b12, eq56_e2426_d_b13, eq56_e2426_d_b14, eq56_e2426_d_b15, eq56_e2426_d_b16, eq56_e2426_d_b17, eq56_e2426_q, eq56_e2426_q_d_n0, eq56_e2426_q_d_n1, eq56_e2426_q_d_n2, eq56_e2426_q_d_n3, eq56_e2426_q_d_n4, eq56_e2426_q_d_n5, eq56_e2426_q_d_n6, eq56_e2426_q_d_n7, eq56_e2426_q_d_n8, eq56_e2426_q_d_n9, eq56_e2426_q_d_n10, eq56_e2426_q_d_n11, eq56_e2426_q_d_n12, eq56_e2426_q_d_n13, eq56_e2426_q_d_n14, eq56_e2426_q_d_n15, eq56_e2426_q_d_n16, eq56_e2426_q_d_b0, eq56_e2426_q_d_b1, eq56_e2426_q_d_b2, eq56_e2426_q_d_b3, eq56_e2426_q_d_b4, eq56_e2426_q_d_b5, eq56_e2426_q_d_b6, eq56_e2426_q_d_b7, eq56_e2426_q_d_b8, eq56_e2426_q_d_b9, eq56_e2426_q_d_b10, eq56_e2426_q_d_b11, eq56_e2426_q_d_b12, eq56_e2426_q_d_b13, eq56_e2426_q_d_b14, eq56_e2426_q_d_b15, eq56_e2426_q_d_b16, eq56_e2426_q_d_b17,) = {
    if s.b[1710] {
        let eq56_e2423_q: f64 = s.v[496];
        let eq56_e2424: f64 = (s.v[114] * s.v[496]);
        let eq56_e2424_d_n0: f64 = ((s.dn[114][0] * s.v[496]) + (s.v[114] * s.dn[496][0]));
        let eq56_e2424_d_n1: f64 = ((s.dn[114][1] * s.v[496]) + (s.v[114] * s.dn[496][1]));
        let eq56_e2424_d_n2: f64 = ((s.dn[114][2] * s.v[496]) + (s.v[114] * s.dn[496][2]));
        let eq56_e2424_d_n3: f64 = ((s.dn[114][3] * s.v[496]) + (s.v[114] * s.dn[496][3]));
        let eq56_e2424_d_n4: f64 = ((s.dn[114][4] * s.v[496]) + (s.v[114] * s.dn[496][4]));
        let eq56_e2424_d_n5: f64 = ((s.dn[114][5] * s.v[496]) + (s.v[114] * s.dn[496][5]));
        let eq56_e2424_d_n6: f64 = ((s.dn[114][6] * s.v[496]) + (s.v[114] * s.dn[496][6]));
        let eq56_e2424_d_n7: f64 = ((s.dn[114][7] * s.v[496]) + (s.v[114] * s.dn[496][7]));
        let eq56_e2424_d_n8: f64 = ((s.dn[114][8] * s.v[496]) + (s.v[114] * s.dn[496][8]));
        let eq56_e2424_d_n9: f64 = ((s.dn[114][9] * s.v[496]) + (s.v[114] * s.dn[496][9]));
        let eq56_e2424_d_n10: f64 = ((s.dn[114][10] * s.v[496]) + (s.v[114] * s.dn[496][10]));
        let eq56_e2424_d_n11: f64 = ((s.dn[114][11] * s.v[496]) + (s.v[114] * s.dn[496][11]));
        let eq56_e2424_d_n12: f64 = ((s.dn[114][12] * s.v[496]) + (s.v[114] * s.dn[496][12]));
        let eq56_e2424_d_n13: f64 = ((s.dn[114][13] * s.v[496]) + (s.v[114] * s.dn[496][13]));
        let eq56_e2424_d_n14: f64 = ((s.dn[114][14] * s.v[496]) + (s.v[114] * s.dn[496][14]));
        let eq56_e2424_d_n15: f64 = ((s.dn[114][15] * s.v[496]) + (s.v[114] * s.dn[496][15]));
        let eq56_e2424_d_n16: f64 = ((s.dn[114][16] * s.v[496]) + (s.v[114] * s.dn[496][16]));
        let eq56_e2424_d_b0: f64 = ((s.db[114][0] * s.v[496]) + (s.v[114] * s.db[496][0]));
        let eq56_e2424_d_b1: f64 = ((s.db[114][1] * s.v[496]) + (s.v[114] * s.db[496][1]));
        let eq56_e2424_d_b2: f64 = ((s.db[114][2] * s.v[496]) + (s.v[114] * s.db[496][2]));
        let eq56_e2424_d_b3: f64 = ((s.db[114][3] * s.v[496]) + (s.v[114] * s.db[496][3]));
        let eq56_e2424_d_b4: f64 = ((s.db[114][4] * s.v[496]) + (s.v[114] * s.db[496][4]));
        let eq56_e2424_d_b5: f64 = ((s.db[114][5] * s.v[496]) + (s.v[114] * s.db[496][5]));
        let eq56_e2424_d_b6: f64 = ((s.db[114][6] * s.v[496]) + (s.v[114] * s.db[496][6]));
        let eq56_e2424_d_b7: f64 = ((s.db[114][7] * s.v[496]) + (s.v[114] * s.db[496][7]));
        let eq56_e2424_d_b8: f64 = ((s.db[114][8] * s.v[496]) + (s.v[114] * s.db[496][8]));
        let eq56_e2424_d_b9: f64 = ((s.db[114][9] * s.v[496]) + (s.v[114] * s.db[496][9]));
        let eq56_e2424_d_b10: f64 = ((s.db[114][10] * s.v[496]) + (s.v[114] * s.db[496][10]));
        let eq56_e2424_d_b11: f64 = ((s.db[114][11] * s.v[496]) + (s.v[114] * s.db[496][11]));
        let eq56_e2424_d_b12: f64 = ((s.db[114][12] * s.v[496]) + (s.v[114] * s.db[496][12]));
        let eq56_e2424_d_b13: f64 = ((s.db[114][13] * s.v[496]) + (s.v[114] * s.db[496][13]));
        let eq56_e2424_d_b14: f64 = ((s.db[114][14] * s.v[496]) + (s.v[114] * s.db[496][14]));
        let eq56_e2424_d_b15: f64 = ((s.db[114][15] * s.v[496]) + (s.v[114] * s.db[496][15]));
        let eq56_e2424_d_b16: f64 = ((s.db[114][16] * s.v[496]) + (s.v[114] * s.db[496][16]));
        let eq56_e2424_d_b17: f64 = ((s.db[114][17] * s.v[496]) + (s.v[114] * s.db[496][17]));
        let eq56_e2424_q: f64 = (s.v[114] * eq56_e2423_q);
        let eq56_e2424_q_d_n0: f64 = ((s.dn[114][0] * eq56_e2423_q) + (s.v[114] * s.dn[496][0]));
        let eq56_e2424_q_d_n1: f64 = ((s.dn[114][1] * eq56_e2423_q) + (s.v[114] * s.dn[496][1]));
        let eq56_e2424_q_d_n2: f64 = ((s.dn[114][2] * eq56_e2423_q) + (s.v[114] * s.dn[496][2]));
        let eq56_e2424_q_d_n3: f64 = ((s.dn[114][3] * eq56_e2423_q) + (s.v[114] * s.dn[496][3]));
        let eq56_e2424_q_d_n4: f64 = ((s.dn[114][4] * eq56_e2423_q) + (s.v[114] * s.dn[496][4]));
        let eq56_e2424_q_d_n5: f64 = ((s.dn[114][5] * eq56_e2423_q) + (s.v[114] * s.dn[496][5]));
        let eq56_e2424_q_d_n6: f64 = ((s.dn[114][6] * eq56_e2423_q) + (s.v[114] * s.dn[496][6]));
        let eq56_e2424_q_d_n7: f64 = ((s.dn[114][7] * eq56_e2423_q) + (s.v[114] * s.dn[496][7]));
        let eq56_e2424_q_d_n8: f64 = ((s.dn[114][8] * eq56_e2423_q) + (s.v[114] * s.dn[496][8]));
        let eq56_e2424_q_d_n9: f64 = ((s.dn[114][9] * eq56_e2423_q) + (s.v[114] * s.dn[496][9]));
        let eq56_e2424_q_d_n10: f64 = ((s.dn[114][10] * eq56_e2423_q) + (s.v[114] * s.dn[496][10]));
        let eq56_e2424_q_d_n11: f64 = ((s.dn[114][11] * eq56_e2423_q) + (s.v[114] * s.dn[496][11]));
        let eq56_e2424_q_d_n12: f64 = ((s.dn[114][12] * eq56_e2423_q) + (s.v[114] * s.dn[496][12]));
        let eq56_e2424_q_d_n13: f64 = ((s.dn[114][13] * eq56_e2423_q) + (s.v[114] * s.dn[496][13]));
        let eq56_e2424_q_d_n14: f64 = ((s.dn[114][14] * eq56_e2423_q) + (s.v[114] * s.dn[496][14]));
        let eq56_e2424_q_d_n15: f64 = ((s.dn[114][15] * eq56_e2423_q) + (s.v[114] * s.dn[496][15]));
        let eq56_e2424_q_d_n16: f64 = ((s.dn[114][16] * eq56_e2423_q) + (s.v[114] * s.dn[496][16]));
        let eq56_e2424_q_d_b0: f64 = ((s.db[114][0] * eq56_e2423_q) + (s.v[114] * s.db[496][0]));
        let eq56_e2424_q_d_b1: f64 = ((s.db[114][1] * eq56_e2423_q) + (s.v[114] * s.db[496][1]));
        let eq56_e2424_q_d_b2: f64 = ((s.db[114][2] * eq56_e2423_q) + (s.v[114] * s.db[496][2]));
        let eq56_e2424_q_d_b3: f64 = ((s.db[114][3] * eq56_e2423_q) + (s.v[114] * s.db[496][3]));
        let eq56_e2424_q_d_b4: f64 = ((s.db[114][4] * eq56_e2423_q) + (s.v[114] * s.db[496][4]));
        let eq56_e2424_q_d_b5: f64 = ((s.db[114][5] * eq56_e2423_q) + (s.v[114] * s.db[496][5]));
        let eq56_e2424_q_d_b6: f64 = ((s.db[114][6] * eq56_e2423_q) + (s.v[114] * s.db[496][6]));
        let eq56_e2424_q_d_b7: f64 = ((s.db[114][7] * eq56_e2423_q) + (s.v[114] * s.db[496][7]));
        let eq56_e2424_q_d_b8: f64 = ((s.db[114][8] * eq56_e2423_q) + (s.v[114] * s.db[496][8]));
        let eq56_e2424_q_d_b9: f64 = ((s.db[114][9] * eq56_e2423_q) + (s.v[114] * s.db[496][9]));
        let eq56_e2424_q_d_b10: f64 = ((s.db[114][10] * eq56_e2423_q) + (s.v[114] * s.db[496][10]));
        let eq56_e2424_q_d_b11: f64 = ((s.db[114][11] * eq56_e2423_q) + (s.v[114] * s.db[496][11]));
        let eq56_e2424_q_d_b12: f64 = ((s.db[114][12] * eq56_e2423_q) + (s.v[114] * s.db[496][12]));
        let eq56_e2424_q_d_b13: f64 = ((s.db[114][13] * eq56_e2423_q) + (s.v[114] * s.db[496][13]));
        let eq56_e2424_q_d_b14: f64 = ((s.db[114][14] * eq56_e2423_q) + (s.v[114] * s.db[496][14]));
        let eq56_e2424_q_d_b15: f64 = ((s.db[114][15] * eq56_e2423_q) + (s.v[114] * s.db[496][15]));
        let eq56_e2424_q_d_b16: f64 = ((s.db[114][16] * eq56_e2423_q) + (s.v[114] * s.db[496][16]));
        let eq56_e2424_q_d_b17: f64 = ((s.db[114][17] * eq56_e2423_q) + (s.v[114] * s.db[496][17]));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n1, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n12, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_d_n15, eq56_e2424_d_n16, eq56_e2424_d_b0, eq56_e2424_d_b1, eq56_e2424_d_b2, eq56_e2424_d_b3, eq56_e2424_d_b4, eq56_e2424_d_b5, eq56_e2424_d_b6, eq56_e2424_d_b7, eq56_e2424_d_b8, eq56_e2424_d_b9, eq56_e2424_d_b10, eq56_e2424_d_b11, eq56_e2424_d_b12, eq56_e2424_d_b13, eq56_e2424_d_b14, eq56_e2424_d_b15, eq56_e2424_d_b16, eq56_e2424_d_b17, eq56_e2424_q, eq56_e2424_q_d_n0, eq56_e2424_q_d_n1, eq56_e2424_q_d_n2, eq56_e2424_q_d_n3, eq56_e2424_q_d_n4, eq56_e2424_q_d_n5, eq56_e2424_q_d_n6, eq56_e2424_q_d_n7, eq56_e2424_q_d_n8, eq56_e2424_q_d_n9, eq56_e2424_q_d_n10, eq56_e2424_q_d_n11, eq56_e2424_q_d_n12, eq56_e2424_q_d_n13, eq56_e2424_q_d_n14, eq56_e2424_q_d_n15, eq56_e2424_q_d_n16, eq56_e2424_q_d_b0, eq56_e2424_q_d_b1, eq56_e2424_q_d_b2, eq56_e2424_q_d_b3, eq56_e2424_q_d_b4, eq56_e2424_q_d_b5, eq56_e2424_q_d_b6, eq56_e2424_q_d_b7, eq56_e2424_q_d_b8, eq56_e2424_q_d_b9, eq56_e2424_q_d_b10, eq56_e2424_q_d_b11, eq56_e2424_q_d_b12, eq56_e2424_q_d_b13, eq56_e2424_q_d_b14, eq56_e2424_q_d_b15, eq56_e2424_q_d_b16, eq56_e2424_q_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_reactive_node_derivatives: [f64; 17] = [eq56_e2426_q_d_n0, eq56_e2426_q_d_n1, eq56_e2426_q_d_n2, eq56_e2426_q_d_n3, eq56_e2426_q_d_n4, eq56_e2426_q_d_n5, eq56_e2426_q_d_n6, eq56_e2426_q_d_n7, eq56_e2426_q_d_n8, eq56_e2426_q_d_n9, eq56_e2426_q_d_n10, eq56_e2426_q_d_n11, eq56_e2426_q_d_n12, eq56_e2426_q_d_n13, eq56_e2426_q_d_n14, eq56_e2426_q_d_n15, eq56_e2426_q_d_n16];
        let eq56_reactive_branch_derivatives: [f64; 18] = [eq56_e2426_q_d_b0, eq56_e2426_q_d_b1, eq56_e2426_q_d_b2, eq56_e2426_q_d_b3, eq56_e2426_q_d_b4, eq56_e2426_q_d_b5, eq56_e2426_q_d_b6, eq56_e2426_q_d_b7, eq56_e2426_q_d_b8, eq56_e2426_q_d_b9, eq56_e2426_q_d_b10, eq56_e2426_q_d_b11, eq56_e2426_q_d_b12, eq56_e2426_q_d_b13, eq56_e2426_q_d_b14, eq56_e2426_q_d_b15, eq56_e2426_q_d_b16, eq56_e2426_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16, eq69_e2506_d_b0, eq69_e2506_d_b1, eq69_e2506_d_b2, eq69_e2506_d_b3, eq69_e2506_d_b4, eq69_e2506_d_b5, eq69_e2506_d_b6, eq69_e2506_d_b7, eq69_e2506_d_b8, eq69_e2506_d_b9, eq69_e2506_d_b10, eq69_e2506_d_b11, eq69_e2506_d_b12, eq69_e2506_d_b13, eq69_e2506_d_b14, eq69_e2506_d_b15, eq69_e2506_d_b16, eq69_e2506_d_b17, eq69_e2506_q, eq69_e2506_q_d_n0, eq69_e2506_q_d_n1, eq69_e2506_q_d_n2, eq69_e2506_q_d_n3, eq69_e2506_q_d_n4, eq69_e2506_q_d_n5, eq69_e2506_q_d_n6, eq69_e2506_q_d_n7, eq69_e2506_q_d_n8, eq69_e2506_q_d_n9, eq69_e2506_q_d_n10, eq69_e2506_q_d_n11, eq69_e2506_q_d_n12, eq69_e2506_q_d_n13, eq69_e2506_q_d_n14, eq69_e2506_q_d_n15, eq69_e2506_q_d_n16, eq69_e2506_q_d_b0, eq69_e2506_q_d_b1, eq69_e2506_q_d_b2, eq69_e2506_q_d_b3, eq69_e2506_q_d_b4, eq69_e2506_q_d_b5, eq69_e2506_q_d_b6, eq69_e2506_q_d_b7, eq69_e2506_q_d_b8, eq69_e2506_q_d_b9, eq69_e2506_q_d_b10, eq69_e2506_q_d_b11, eq69_e2506_q_d_b12, eq69_e2506_q_d_b13, eq69_e2506_q_d_b14, eq69_e2506_q_d_b15, eq69_e2506_q_d_b16, eq69_e2506_q_d_b17,) = {
    if s.b[1723] {
        let eq69_e2503: f64 = (s.v[138] - s.v[140]);
        let eq69_e2503_d_n0: f64 = (s.dn[138][0] - s.dn[140][0]);
        let eq69_e2503_d_n1: f64 = (s.dn[138][1] - s.dn[140][1]);
        let eq69_e2503_d_n2: f64 = (s.dn[138][2] - s.dn[140][2]);
        let eq69_e2503_d_n3: f64 = (s.dn[138][3] - s.dn[140][3]);
        let eq69_e2503_d_n4: f64 = (s.dn[138][4] - s.dn[140][4]);
        let eq69_e2503_d_n5: f64 = (s.dn[138][5] - s.dn[140][5]);
        let eq69_e2503_d_n6: f64 = (s.dn[138][6] - s.dn[140][6]);
        let eq69_e2503_d_n7: f64 = (s.dn[138][7] - s.dn[140][7]);
        let eq69_e2503_d_n8: f64 = (s.dn[138][8] - s.dn[140][8]);
        let eq69_e2503_d_n9: f64 = (s.dn[138][9] - s.dn[140][9]);
        let eq69_e2503_d_n10: f64 = (s.dn[138][10] - s.dn[140][10]);
        let eq69_e2503_d_n11: f64 = (s.dn[138][11] - s.dn[140][11]);
        let eq69_e2503_d_n12: f64 = (s.dn[138][12] - s.dn[140][12]);
        let eq69_e2503_d_n13: f64 = (s.dn[138][13] - s.dn[140][13]);
        let eq69_e2503_d_n14: f64 = (s.dn[138][14] - s.dn[140][14]);
        let eq69_e2503_d_n15: f64 = (s.dn[138][15] - s.dn[140][15]);
        let eq69_e2503_d_n16: f64 = (s.dn[138][16] - s.dn[140][16]);
        let eq69_e2503_d_b0: f64 = (s.db[138][0] - s.db[140][0]);
        let eq69_e2503_d_b1: f64 = (s.db[138][1] - s.db[140][1]);
        let eq69_e2503_d_b2: f64 = (s.db[138][2] - s.db[140][2]);
        let eq69_e2503_d_b3: f64 = (s.db[138][3] - s.db[140][3]);
        let eq69_e2503_d_b4: f64 = (s.db[138][4] - s.db[140][4]);
        let eq69_e2503_d_b5: f64 = (s.db[138][5] - s.db[140][5]);
        let eq69_e2503_d_b6: f64 = (s.db[138][6] - s.db[140][6]);
        let eq69_e2503_d_b7: f64 = (s.db[138][7] - s.db[140][7]);
        let eq69_e2503_d_b8: f64 = (s.db[138][8] - s.db[140][8]);
        let eq69_e2503_d_b9: f64 = (s.db[138][9] - s.db[140][9]);
        let eq69_e2503_d_b10: f64 = (s.db[138][10] - s.db[140][10]);
        let eq69_e2503_d_b11: f64 = (s.db[138][11] - s.db[140][11]);
        let eq69_e2503_d_b12: f64 = (s.db[138][12] - s.db[140][12]);
        let eq69_e2503_d_b13: f64 = (s.db[138][13] - s.db[140][13]);
        let eq69_e2503_d_b14: f64 = (s.db[138][14] - s.db[140][14]);
        let eq69_e2503_d_b15: f64 = (s.db[138][15] - s.db[140][15]);
        let eq69_e2503_d_b16: f64 = (s.db[138][16] - s.db[140][16]);
        let eq69_e2503_d_b17: f64 = (s.db[138][17] - s.db[140][17]);
        let eq69_e2504_q: f64 = eq69_e2503;
        (eq69_e2503, eq69_e2503_d_n0, eq69_e2503_d_n1, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n12, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2503_d_n15, eq69_e2503_d_n16, eq69_e2503_d_b0, eq69_e2503_d_b1, eq69_e2503_d_b2, eq69_e2503_d_b3, eq69_e2503_d_b4, eq69_e2503_d_b5, eq69_e2503_d_b6, eq69_e2503_d_b7, eq69_e2503_d_b8, eq69_e2503_d_b9, eq69_e2503_d_b10, eq69_e2503_d_b11, eq69_e2503_d_b12, eq69_e2503_d_b13, eq69_e2503_d_b14, eq69_e2503_d_b15, eq69_e2503_d_b16, eq69_e2503_d_b17, eq69_e2504_q, eq69_e2503_d_n0, eq69_e2503_d_n1, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n12, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2503_d_n15, eq69_e2503_d_n16, eq69_e2503_d_b0, eq69_e2503_d_b1, eq69_e2503_d_b2, eq69_e2503_d_b3, eq69_e2503_d_b4, eq69_e2503_d_b5, eq69_e2503_d_b6, eq69_e2503_d_b7, eq69_e2503_d_b8, eq69_e2503_d_b9, eq69_e2503_d_b10, eq69_e2503_d_b11, eq69_e2503_d_b12, eq69_e2503_d_b13, eq69_e2503_d_b14, eq69_e2503_d_b15, eq69_e2503_d_b16, eq69_e2503_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 17] = [eq69_e2506_q_d_n0, eq69_e2506_q_d_n1, eq69_e2506_q_d_n2, eq69_e2506_q_d_n3, eq69_e2506_q_d_n4, eq69_e2506_q_d_n5, eq69_e2506_q_d_n6, eq69_e2506_q_d_n7, eq69_e2506_q_d_n8, eq69_e2506_q_d_n9, eq69_e2506_q_d_n10, eq69_e2506_q_d_n11, eq69_e2506_q_d_n12, eq69_e2506_q_d_n13, eq69_e2506_q_d_n14, eq69_e2506_q_d_n15, eq69_e2506_q_d_n16];
        let eq69_reactive_branch_derivatives: [f64; 18] = [eq69_e2506_q_d_b0, eq69_e2506_q_d_b1, eq69_e2506_q_d_b2, eq69_e2506_q_d_b3, eq69_e2506_q_d_b4, eq69_e2506_q_d_b5, eq69_e2506_q_d_b6, eq69_e2506_q_d_b7, eq69_e2506_q_d_b8, eq69_e2506_q_d_b9, eq69_e2506_q_d_b10, eq69_e2506_q_d_b11, eq69_e2506_q_d_b12, eq69_e2506_q_d_b13, eq69_e2506_q_d_b14, eq69_e2506_q_d_b15, eq69_e2506_q_d_b16, eq69_e2506_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15, eq71_e2519_q, eq71_e2519_q_d_n15,) = {
    if s.b[1723] {
        let eq71_e2516_q: f64 = (nv15 - 0.0);
        let eq71_e2517: f64 = (1e-9 * (nv15 - 0.0));
        let eq71_e2517_d_n15: f64 = 1e-9;
        let eq71_e2517_q: f64 = (1e-9 * eq71_e2516_q);
        let eq71_e2517_q_d_n15: f64 = 1e-9;
        (eq71_e2517, eq71_e2517_d_n15, eq71_e2517_q, eq71_e2517_q_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq71_e2519_q_d_n15),
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16, eq96_e2717_d_b0, eq96_e2717_d_b1, eq96_e2717_d_b2, eq96_e2717_d_b3, eq96_e2717_d_b4, eq96_e2717_d_b5, eq96_e2717_d_b6, eq96_e2717_d_b7, eq96_e2717_d_b8, eq96_e2717_d_b9, eq96_e2717_d_b10, eq96_e2717_d_b11, eq96_e2717_d_b12, eq96_e2717_d_b13, eq96_e2717_d_b14, eq96_e2717_d_b15, eq96_e2717_d_b16, eq96_e2717_d_b17, eq96_e2717_q, eq96_e2717_q_d_n0, eq96_e2717_q_d_n1, eq96_e2717_q_d_n2, eq96_e2717_q_d_n3, eq96_e2717_q_d_n4, eq96_e2717_q_d_n5, eq96_e2717_q_d_n6, eq96_e2717_q_d_n7, eq96_e2717_q_d_n8, eq96_e2717_q_d_n9, eq96_e2717_q_d_n10, eq96_e2717_q_d_n11, eq96_e2717_q_d_n12, eq96_e2717_q_d_n13, eq96_e2717_q_d_n14, eq96_e2717_q_d_n15, eq96_e2717_q_d_n16, eq96_e2717_q_d_b0, eq96_e2717_q_d_b1, eq96_e2717_q_d_b2, eq96_e2717_q_d_b3, eq96_e2717_q_d_b4, eq96_e2717_q_d_b5, eq96_e2717_q_d_b6, eq96_e2717_q_d_b7, eq96_e2717_q_d_b8, eq96_e2717_q_d_b9, eq96_e2717_q_d_b10, eq96_e2717_q_d_b11, eq96_e2717_q_d_b12, eq96_e2717_q_d_b13, eq96_e2717_q_d_b14, eq96_e2717_q_d_b15, eq96_e2717_q_d_b16, eq96_e2717_q_d_b17,) = {
    if (!s.b[1731]) {
        let eq96_e2712: f64 = (0.7071 * s.v[632]);
        let eq96_e2712_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq96_e2712_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq96_e2712_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq96_e2712_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq96_e2712_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq96_e2712_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq96_e2712_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq96_e2712_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq96_e2712_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq96_e2712_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq96_e2712_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq96_e2712_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq96_e2712_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq96_e2712_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq96_e2712_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq96_e2712_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq96_e2712_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq96_e2712_d_b0: f64 = (0.7071 * s.db[632][0]);
        let eq96_e2712_d_b1: f64 = (0.7071 * s.db[632][1]);
        let eq96_e2712_d_b2: f64 = (0.7071 * s.db[632][2]);
        let eq96_e2712_d_b3: f64 = (0.7071 * s.db[632][3]);
        let eq96_e2712_d_b4: f64 = (0.7071 * s.db[632][4]);
        let eq96_e2712_d_b5: f64 = (0.7071 * s.db[632][5]);
        let eq96_e2712_d_b6: f64 = (0.7071 * s.db[632][6]);
        let eq96_e2712_d_b7: f64 = (0.7071 * s.db[632][7]);
        let eq96_e2712_d_b8: f64 = (0.7071 * s.db[632][8]);
        let eq96_e2712_d_b9: f64 = (0.7071 * s.db[632][9]);
        let eq96_e2712_d_b10: f64 = (0.7071 * s.db[632][10]);
        let eq96_e2712_d_b11: f64 = (0.7071 * s.db[632][11]);
        let eq96_e2712_d_b12: f64 = (0.7071 * s.db[632][12]);
        let eq96_e2712_d_b13: f64 = (0.7071 * s.db[632][13]);
        let eq96_e2712_d_b14: f64 = (0.7071 * s.db[632][14]);
        let eq96_e2712_d_b15: f64 = (0.7071 * s.db[632][15]);
        let eq96_e2712_d_b16: f64 = (0.7071 * s.db[632][16]);
        let eq96_e2712_d_b17: f64 = (0.7071 * s.db[632][17]);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n1: f64 = (eq96_e2712_d_n1 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n12: f64 = (eq96_e2712_d_n12 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2714_d_n15: f64 = (eq96_e2712_d_n15 * (nv16 - 0.0));
        let eq96_e2714_d_n16: f64 = ((eq96_e2712_d_n16 * (nv16 - 0.0)) + eq96_e2712);
        let eq96_e2714_d_b0: f64 = (eq96_e2712_d_b0 * (nv16 - 0.0));
        let eq96_e2714_d_b1: f64 = (eq96_e2712_d_b1 * (nv16 - 0.0));
        let eq96_e2714_d_b2: f64 = (eq96_e2712_d_b2 * (nv16 - 0.0));
        let eq96_e2714_d_b3: f64 = (eq96_e2712_d_b3 * (nv16 - 0.0));
        let eq96_e2714_d_b4: f64 = (eq96_e2712_d_b4 * (nv16 - 0.0));
        let eq96_e2714_d_b5: f64 = (eq96_e2712_d_b5 * (nv16 - 0.0));
        let eq96_e2714_d_b6: f64 = (eq96_e2712_d_b6 * (nv16 - 0.0));
        let eq96_e2714_d_b7: f64 = (eq96_e2712_d_b7 * (nv16 - 0.0));
        let eq96_e2714_d_b8: f64 = (eq96_e2712_d_b8 * (nv16 - 0.0));
        let eq96_e2714_d_b9: f64 = (eq96_e2712_d_b9 * (nv16 - 0.0));
        let eq96_e2714_d_b10: f64 = (eq96_e2712_d_b10 * (nv16 - 0.0));
        let eq96_e2714_d_b11: f64 = (eq96_e2712_d_b11 * (nv16 - 0.0));
        let eq96_e2714_d_b12: f64 = (eq96_e2712_d_b12 * (nv16 - 0.0));
        let eq96_e2714_d_b13: f64 = (eq96_e2712_d_b13 * (nv16 - 0.0));
        let eq96_e2714_d_b14: f64 = (eq96_e2712_d_b14 * (nv16 - 0.0));
        let eq96_e2714_d_b15: f64 = (eq96_e2712_d_b15 * (nv16 - 0.0));
        let eq96_e2714_d_b16: f64 = (eq96_e2712_d_b16 * (nv16 - 0.0));
        let eq96_e2714_d_b17: f64 = (eq96_e2712_d_b17 * (nv16 - 0.0));
        let eq96_e2715_q: f64 = eq96_e2714;
        (eq96_e2714, eq96_e2714_d_n0, eq96_e2714_d_n1, eq96_e2714_d_n2, eq96_e2714_d_n3, eq96_e2714_d_n4, eq96_e2714_d_n5, eq96_e2714_d_n6, eq96_e2714_d_n7, eq96_e2714_d_n8, eq96_e2714_d_n9, eq96_e2714_d_n10, eq96_e2714_d_n11, eq96_e2714_d_n12, eq96_e2714_d_n13, eq96_e2714_d_n14, eq96_e2714_d_n15, eq96_e2714_d_n16, eq96_e2714_d_b0, eq96_e2714_d_b1, eq96_e2714_d_b2, eq96_e2714_d_b3, eq96_e2714_d_b4, eq96_e2714_d_b5, eq96_e2714_d_b6, eq96_e2714_d_b7, eq96_e2714_d_b8, eq96_e2714_d_b9, eq96_e2714_d_b10, eq96_e2714_d_b11, eq96_e2714_d_b12, eq96_e2714_d_b13, eq96_e2714_d_b14, eq96_e2714_d_b15, eq96_e2714_d_b16, eq96_e2714_d_b17, eq96_e2715_q, eq96_e2714_d_n0, eq96_e2714_d_n1, eq96_e2714_d_n2, eq96_e2714_d_n3, eq96_e2714_d_n4, eq96_e2714_d_n5, eq96_e2714_d_n6, eq96_e2714_d_n7, eq96_e2714_d_n8, eq96_e2714_d_n9, eq96_e2714_d_n10, eq96_e2714_d_n11, eq96_e2714_d_n12, eq96_e2714_d_n13, eq96_e2714_d_n14, eq96_e2714_d_n15, eq96_e2714_d_n16, eq96_e2714_d_b0, eq96_e2714_d_b1, eq96_e2714_d_b2, eq96_e2714_d_b3, eq96_e2714_d_b4, eq96_e2714_d_b5, eq96_e2714_d_b6, eq96_e2714_d_b7, eq96_e2714_d_b8, eq96_e2714_d_b9, eq96_e2714_d_b10, eq96_e2714_d_b11, eq96_e2714_d_b12, eq96_e2714_d_b13, eq96_e2714_d_b14, eq96_e2714_d_b15, eq96_e2714_d_b16, eq96_e2714_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_reactive_node_derivatives: [f64; 17] = [eq96_e2717_q_d_n0, eq96_e2717_q_d_n1, eq96_e2717_q_d_n2, eq96_e2717_q_d_n3, eq96_e2717_q_d_n4, eq96_e2717_q_d_n5, eq96_e2717_q_d_n6, eq96_e2717_q_d_n7, eq96_e2717_q_d_n8, eq96_e2717_q_d_n9, eq96_e2717_q_d_n10, eq96_e2717_q_d_n11, eq96_e2717_q_d_n12, eq96_e2717_q_d_n13, eq96_e2717_q_d_n14, eq96_e2717_q_d_n15, eq96_e2717_q_d_n16];
        let eq96_reactive_branch_derivatives: [f64; 18] = [eq96_e2717_q_d_b0, eq96_e2717_q_d_b1, eq96_e2717_q_d_b2, eq96_e2717_q_d_b3, eq96_e2717_q_d_b4, eq96_e2717_q_d_b5, eq96_e2717_q_d_b6, eq96_e2717_q_d_b7, eq96_e2717_q_d_b8, eq96_e2717_q_d_b9, eq96_e2717_q_d_b10, eq96_e2717_q_d_b11, eq96_e2717_q_d_b12, eq96_e2717_q_d_b13, eq96_e2717_q_d_b14, eq96_e2717_q_d_b15, eq96_e2717_q_d_b16, eq96_e2717_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq96_reactive_node_derivatives,
            branches,
            &eq96_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16, eq97_e2727_d_b0, eq97_e2727_d_b1, eq97_e2727_d_b2, eq97_e2727_d_b3, eq97_e2727_d_b4, eq97_e2727_d_b5, eq97_e2727_d_b6, eq97_e2727_d_b7, eq97_e2727_d_b8, eq97_e2727_d_b9, eq97_e2727_d_b10, eq97_e2727_d_b11, eq97_e2727_d_b12, eq97_e2727_d_b13, eq97_e2727_d_b14, eq97_e2727_d_b15, eq97_e2727_d_b16, eq97_e2727_d_b17, eq97_e2727_q, eq97_e2727_q_d_n0, eq97_e2727_q_d_n1, eq97_e2727_q_d_n2, eq97_e2727_q_d_n3, eq97_e2727_q_d_n4, eq97_e2727_q_d_n5, eq97_e2727_q_d_n6, eq97_e2727_q_d_n7, eq97_e2727_q_d_n8, eq97_e2727_q_d_n9, eq97_e2727_q_d_n10, eq97_e2727_q_d_n11, eq97_e2727_q_d_n12, eq97_e2727_q_d_n13, eq97_e2727_q_d_n14, eq97_e2727_q_d_n15, eq97_e2727_q_d_n16, eq97_e2727_q_d_b0, eq97_e2727_q_d_b1, eq97_e2727_q_d_b2, eq97_e2727_q_d_b3, eq97_e2727_q_d_b4, eq97_e2727_q_d_b5, eq97_e2727_q_d_b6, eq97_e2727_q_d_b7, eq97_e2727_q_d_b8, eq97_e2727_q_d_b9, eq97_e2727_q_d_b10, eq97_e2727_q_d_b11, eq97_e2727_q_d_b12, eq97_e2727_q_d_b13, eq97_e2727_q_d_b14, eq97_e2727_q_d_b15, eq97_e2727_q_d_b16, eq97_e2727_q_d_b17,) = {
    if (!s.b[1731]) {
        let eq97_e2722: f64 = (0.7071 * s.v[632]);
        let eq97_e2722_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq97_e2722_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq97_e2722_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq97_e2722_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq97_e2722_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq97_e2722_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq97_e2722_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq97_e2722_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq97_e2722_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq97_e2722_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq97_e2722_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq97_e2722_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq97_e2722_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq97_e2722_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq97_e2722_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq97_e2722_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq97_e2722_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq97_e2722_d_b0: f64 = (0.7071 * s.db[632][0]);
        let eq97_e2722_d_b1: f64 = (0.7071 * s.db[632][1]);
        let eq97_e2722_d_b2: f64 = (0.7071 * s.db[632][2]);
        let eq97_e2722_d_b3: f64 = (0.7071 * s.db[632][3]);
        let eq97_e2722_d_b4: f64 = (0.7071 * s.db[632][4]);
        let eq97_e2722_d_b5: f64 = (0.7071 * s.db[632][5]);
        let eq97_e2722_d_b6: f64 = (0.7071 * s.db[632][6]);
        let eq97_e2722_d_b7: f64 = (0.7071 * s.db[632][7]);
        let eq97_e2722_d_b8: f64 = (0.7071 * s.db[632][8]);
        let eq97_e2722_d_b9: f64 = (0.7071 * s.db[632][9]);
        let eq97_e2722_d_b10: f64 = (0.7071 * s.db[632][10]);
        let eq97_e2722_d_b11: f64 = (0.7071 * s.db[632][11]);
        let eq97_e2722_d_b12: f64 = (0.7071 * s.db[632][12]);
        let eq97_e2722_d_b13: f64 = (0.7071 * s.db[632][13]);
        let eq97_e2722_d_b14: f64 = (0.7071 * s.db[632][14]);
        let eq97_e2722_d_b15: f64 = (0.7071 * s.db[632][15]);
        let eq97_e2722_d_b16: f64 = (0.7071 * s.db[632][16]);
        let eq97_e2722_d_b17: f64 = (0.7071 * s.db[632][17]);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n1: f64 = (eq97_e2722_d_n1 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n12: f64 = (eq97_e2722_d_n12 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2724_d_n15: f64 = (eq97_e2722_d_n15 * (nv16 - 0.0));
        let eq97_e2724_d_n16: f64 = ((eq97_e2722_d_n16 * (nv16 - 0.0)) + eq97_e2722);
        let eq97_e2724_d_b0: f64 = (eq97_e2722_d_b0 * (nv16 - 0.0));
        let eq97_e2724_d_b1: f64 = (eq97_e2722_d_b1 * (nv16 - 0.0));
        let eq97_e2724_d_b2: f64 = (eq97_e2722_d_b2 * (nv16 - 0.0));
        let eq97_e2724_d_b3: f64 = (eq97_e2722_d_b3 * (nv16 - 0.0));
        let eq97_e2724_d_b4: f64 = (eq97_e2722_d_b4 * (nv16 - 0.0));
        let eq97_e2724_d_b5: f64 = (eq97_e2722_d_b5 * (nv16 - 0.0));
        let eq97_e2724_d_b6: f64 = (eq97_e2722_d_b6 * (nv16 - 0.0));
        let eq97_e2724_d_b7: f64 = (eq97_e2722_d_b7 * (nv16 - 0.0));
        let eq97_e2724_d_b8: f64 = (eq97_e2722_d_b8 * (nv16 - 0.0));
        let eq97_e2724_d_b9: f64 = (eq97_e2722_d_b9 * (nv16 - 0.0));
        let eq97_e2724_d_b10: f64 = (eq97_e2722_d_b10 * (nv16 - 0.0));
        let eq97_e2724_d_b11: f64 = (eq97_e2722_d_b11 * (nv16 - 0.0));
        let eq97_e2724_d_b12: f64 = (eq97_e2722_d_b12 * (nv16 - 0.0));
        let eq97_e2724_d_b13: f64 = (eq97_e2722_d_b13 * (nv16 - 0.0));
        let eq97_e2724_d_b14: f64 = (eq97_e2722_d_b14 * (nv16 - 0.0));
        let eq97_e2724_d_b15: f64 = (eq97_e2722_d_b15 * (nv16 - 0.0));
        let eq97_e2724_d_b16: f64 = (eq97_e2722_d_b16 * (nv16 - 0.0));
        let eq97_e2724_d_b17: f64 = (eq97_e2722_d_b17 * (nv16 - 0.0));
        let eq97_e2725_q: f64 = eq97_e2724;
        (eq97_e2724, eq97_e2724_d_n0, eq97_e2724_d_n1, eq97_e2724_d_n2, eq97_e2724_d_n3, eq97_e2724_d_n4, eq97_e2724_d_n5, eq97_e2724_d_n6, eq97_e2724_d_n7, eq97_e2724_d_n8, eq97_e2724_d_n9, eq97_e2724_d_n10, eq97_e2724_d_n11, eq97_e2724_d_n12, eq97_e2724_d_n13, eq97_e2724_d_n14, eq97_e2724_d_n15, eq97_e2724_d_n16, eq97_e2724_d_b0, eq97_e2724_d_b1, eq97_e2724_d_b2, eq97_e2724_d_b3, eq97_e2724_d_b4, eq97_e2724_d_b5, eq97_e2724_d_b6, eq97_e2724_d_b7, eq97_e2724_d_b8, eq97_e2724_d_b9, eq97_e2724_d_b10, eq97_e2724_d_b11, eq97_e2724_d_b12, eq97_e2724_d_b13, eq97_e2724_d_b14, eq97_e2724_d_b15, eq97_e2724_d_b16, eq97_e2724_d_b17, eq97_e2725_q, eq97_e2724_d_n0, eq97_e2724_d_n1, eq97_e2724_d_n2, eq97_e2724_d_n3, eq97_e2724_d_n4, eq97_e2724_d_n5, eq97_e2724_d_n6, eq97_e2724_d_n7, eq97_e2724_d_n8, eq97_e2724_d_n9, eq97_e2724_d_n10, eq97_e2724_d_n11, eq97_e2724_d_n12, eq97_e2724_d_n13, eq97_e2724_d_n14, eq97_e2724_d_n15, eq97_e2724_d_n16, eq97_e2724_d_b0, eq97_e2724_d_b1, eq97_e2724_d_b2, eq97_e2724_d_b3, eq97_e2724_d_b4, eq97_e2724_d_b5, eq97_e2724_d_b6, eq97_e2724_d_b7, eq97_e2724_d_b8, eq97_e2724_d_b9, eq97_e2724_d_b10, eq97_e2724_d_b11, eq97_e2724_d_b12, eq97_e2724_d_b13, eq97_e2724_d_b14, eq97_e2724_d_b15, eq97_e2724_d_b16, eq97_e2724_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_reactive_node_derivatives: [f64; 17] = [eq97_e2727_q_d_n0, eq97_e2727_q_d_n1, eq97_e2727_q_d_n2, eq97_e2727_q_d_n3, eq97_e2727_q_d_n4, eq97_e2727_q_d_n5, eq97_e2727_q_d_n6, eq97_e2727_q_d_n7, eq97_e2727_q_d_n8, eq97_e2727_q_d_n9, eq97_e2727_q_d_n10, eq97_e2727_q_d_n11, eq97_e2727_q_d_n12, eq97_e2727_q_d_n13, eq97_e2727_q_d_n14, eq97_e2727_q_d_n15, eq97_e2727_q_d_n16];
        let eq97_reactive_branch_derivatives: [f64; 18] = [eq97_e2727_q_d_b0, eq97_e2727_q_d_b1, eq97_e2727_q_d_b2, eq97_e2727_q_d_b3, eq97_e2727_q_d_b4, eq97_e2727_q_d_b5, eq97_e2727_q_d_b6, eq97_e2727_q_d_b7, eq97_e2727_q_d_b8, eq97_e2727_q_d_b9, eq97_e2727_q_d_b10, eq97_e2727_q_d_b11, eq97_e2727_q_d_b12, eq97_e2727_q_d_b13, eq97_e2727_q_d_b14, eq97_e2727_q_d_b15, eq97_e2727_q_d_b16, eq97_e2727_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq97_reactive_node_derivatives,
            branches,
            &eq97_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16, eq111_e2904_d_b0, eq111_e2904_d_b1, eq111_e2904_d_b2, eq111_e2904_d_b3, eq111_e2904_d_b4, eq111_e2904_d_b5, eq111_e2904_d_b6, eq111_e2904_d_b7, eq111_e2904_d_b8, eq111_e2904_d_b9, eq111_e2904_d_b10, eq111_e2904_d_b11, eq111_e2904_d_b12, eq111_e2904_d_b13, eq111_e2904_d_b14, eq111_e2904_d_b15, eq111_e2904_d_b16, eq111_e2904_d_b17, eq111_e2904_q, eq111_e2904_q_d_n0, eq111_e2904_q_d_n1, eq111_e2904_q_d_n2, eq111_e2904_q_d_n3, eq111_e2904_q_d_n4, eq111_e2904_q_d_n5, eq111_e2904_q_d_n6, eq111_e2904_q_d_n7, eq111_e2904_q_d_n8, eq111_e2904_q_d_n9, eq111_e2904_q_d_n10, eq111_e2904_q_d_n11, eq111_e2904_q_d_n12, eq111_e2904_q_d_n13, eq111_e2904_q_d_n14, eq111_e2904_q_d_n15, eq111_e2904_q_d_n16, eq111_e2904_q_d_b0, eq111_e2904_q_d_b1, eq111_e2904_q_d_b2, eq111_e2904_q_d_b3, eq111_e2904_q_d_b4, eq111_e2904_q_d_b5, eq111_e2904_q_d_b6, eq111_e2904_q_d_b7, eq111_e2904_q_d_b8, eq111_e2904_q_d_b9, eq111_e2904_q_d_b10, eq111_e2904_q_d_b11, eq111_e2904_q_d_b12, eq111_e2904_q_d_b13, eq111_e2904_q_d_b14, eq111_e2904_q_d_b15, eq111_e2904_q_d_b16, eq111_e2904_q_d_b17,) = {
    if s.b[1736] {
        let eq111_e2901: f64 = ((nv4 - 0.0) * s.v[634]);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * s.dn[634][0]);
        let eq111_e2901_d_n1: f64 = ((nv4 - 0.0) * s.dn[634][1]);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * s.dn[634][2]);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * s.dn[634][3]);
        let eq111_e2901_d_n4: f64 = (s.v[634] + ((nv4 - 0.0) * s.dn[634][4]));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * s.dn[634][5]);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * s.dn[634][6]);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * s.dn[634][7]);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * s.dn[634][8]);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * s.dn[634][9]);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * s.dn[634][10]);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * s.dn[634][11]);
        let eq111_e2901_d_n12: f64 = ((nv4 - 0.0) * s.dn[634][12]);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * s.dn[634][13]);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * s.dn[634][14]);
        let eq111_e2901_d_n15: f64 = ((nv4 - 0.0) * s.dn[634][15]);
        let eq111_e2901_d_n16: f64 = ((nv4 - 0.0) * s.dn[634][16]);
        let eq111_e2901_d_b0: f64 = ((nv4 - 0.0) * s.db[634][0]);
        let eq111_e2901_d_b1: f64 = ((nv4 - 0.0) * s.db[634][1]);
        let eq111_e2901_d_b2: f64 = ((nv4 - 0.0) * s.db[634][2]);
        let eq111_e2901_d_b3: f64 = ((nv4 - 0.0) * s.db[634][3]);
        let eq111_e2901_d_b4: f64 = ((nv4 - 0.0) * s.db[634][4]);
        let eq111_e2901_d_b5: f64 = ((nv4 - 0.0) * s.db[634][5]);
        let eq111_e2901_d_b6: f64 = ((nv4 - 0.0) * s.db[634][6]);
        let eq111_e2901_d_b7: f64 = ((nv4 - 0.0) * s.db[634][7]);
        let eq111_e2901_d_b8: f64 = ((nv4 - 0.0) * s.db[634][8]);
        let eq111_e2901_d_b9: f64 = ((nv4 - 0.0) * s.db[634][9]);
        let eq111_e2901_d_b10: f64 = ((nv4 - 0.0) * s.db[634][10]);
        let eq111_e2901_d_b11: f64 = ((nv4 - 0.0) * s.db[634][11]);
        let eq111_e2901_d_b12: f64 = ((nv4 - 0.0) * s.db[634][12]);
        let eq111_e2901_d_b13: f64 = ((nv4 - 0.0) * s.db[634][13]);
        let eq111_e2901_d_b14: f64 = ((nv4 - 0.0) * s.db[634][14]);
        let eq111_e2901_d_b15: f64 = ((nv4 - 0.0) * s.db[634][15]);
        let eq111_e2901_d_b16: f64 = ((nv4 - 0.0) * s.db[634][16]);
        let eq111_e2901_d_b17: f64 = ((nv4 - 0.0) * s.db[634][17]);
        let eq111_e2902_q: f64 = eq111_e2901;
        (eq111_e2901, eq111_e2901_d_n0, eq111_e2901_d_n1, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n12, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2901_d_n15, eq111_e2901_d_n16, eq111_e2901_d_b0, eq111_e2901_d_b1, eq111_e2901_d_b2, eq111_e2901_d_b3, eq111_e2901_d_b4, eq111_e2901_d_b5, eq111_e2901_d_b6, eq111_e2901_d_b7, eq111_e2901_d_b8, eq111_e2901_d_b9, eq111_e2901_d_b10, eq111_e2901_d_b11, eq111_e2901_d_b12, eq111_e2901_d_b13, eq111_e2901_d_b14, eq111_e2901_d_b15, eq111_e2901_d_b16, eq111_e2901_d_b17, eq111_e2902_q, eq111_e2901_d_n0, eq111_e2901_d_n1, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n12, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2901_d_n15, eq111_e2901_d_n16, eq111_e2901_d_b0, eq111_e2901_d_b1, eq111_e2901_d_b2, eq111_e2901_d_b3, eq111_e2901_d_b4, eq111_e2901_d_b5, eq111_e2901_d_b6, eq111_e2901_d_b7, eq111_e2901_d_b8, eq111_e2901_d_b9, eq111_e2901_d_b10, eq111_e2901_d_b11, eq111_e2901_d_b12, eq111_e2901_d_b13, eq111_e2901_d_b14, eq111_e2901_d_b15, eq111_e2901_d_b16, eq111_e2901_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 17] = [eq111_e2904_q_d_n0, eq111_e2904_q_d_n1, eq111_e2904_q_d_n2, eq111_e2904_q_d_n3, eq111_e2904_q_d_n4, eq111_e2904_q_d_n5, eq111_e2904_q_d_n6, eq111_e2904_q_d_n7, eq111_e2904_q_d_n8, eq111_e2904_q_d_n9, eq111_e2904_q_d_n10, eq111_e2904_q_d_n11, eq111_e2904_q_d_n12, eq111_e2904_q_d_n13, eq111_e2904_q_d_n14, eq111_e2904_q_d_n15, eq111_e2904_q_d_n16];
        let eq111_reactive_branch_derivatives: [f64; 18] = [eq111_e2904_q_d_b0, eq111_e2904_q_d_b1, eq111_e2904_q_d_b2, eq111_e2904_q_d_b3, eq111_e2904_q_d_b4, eq111_e2904_q_d_b5, eq111_e2904_q_d_b6, eq111_e2904_q_d_b7, eq111_e2904_q_d_b8, eq111_e2904_q_d_b9, eq111_e2904_q_d_b10, eq111_e2904_q_d_b11, eq111_e2904_q_d_b12, eq111_e2904_q_d_b13, eq111_e2904_q_d_b14, eq111_e2904_q_d_b15, eq111_e2904_q_d_b16, eq111_e2904_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
