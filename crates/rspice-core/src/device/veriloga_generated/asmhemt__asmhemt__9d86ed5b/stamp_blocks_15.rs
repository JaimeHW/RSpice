#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_12(
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq214_e2687, eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22, eq214_e2687_d_b0, eq214_e2687_d_b1, eq214_e2687_d_b2, eq214_e2687_d_b3, eq214_e2687_d_b4, eq214_e2687_d_b5, eq214_e2687_d_b6, eq214_e2687_d_b7, eq214_e2687_d_b8, eq214_e2687_d_b9, eq214_e2687_d_b10, eq214_e2687_d_b11, eq214_e2687_d_b12, eq214_e2687_d_b13, eq214_e2687_d_b14, eq214_e2687_d_b15, eq214_e2687_d_b16, eq214_e2687_d_b17, eq214_e2687_d_b18, eq214_e2687_d_b19, eq214_e2687_d_b20, eq214_e2687_d_b21, eq214_e2687_d_b22, eq214_e2687_d_b23, eq214_e2687_d_b24, eq214_e2687_d_b25, eq214_e2687_d_b26, eq214_e2687_d_b27, eq214_e2687_d_b28, eq214_e2687_d_b29, eq214_e2687_d_b30, eq214_e2687_d_b31, eq214_e2687_d_b32, eq214_e2687_d_b33, eq214_e2687_d_b34, eq214_e2687_d_b35, eq214_e2687_d_b36, eq214_e2687_d_b37, eq214_e2687_d_b38, eq214_e2687_d_b39, eq214_e2687_d_b40, eq214_e2687_d_b41, eq214_e2687_d_b42, eq214_e2687_d_b43, eq214_e2687_d_b44, eq214_e2687_d_b45, eq214_e2687_d_b46, eq214_e2687_d_b47, eq214_e2687_d_b48, eq214_e2687_d_b49, eq214_e2687_d_b50, eq214_e2687_d_b51, eq214_e2687_d_b52, eq214_e2687_d_b53, eq214_e2687_d_b54, eq214_e2687_q,) = {
    if (((!s.b[605]) && s.b[608]) && (!s.b[609])) {
        let eq214_e2682_q: f64 = s.v[312];
        let eq214_e2683: f64 = (p.p7 * s.v[312]);
        let eq214_e2683_q: f64 = (p.p7 * eq214_e2682_q);
        let eq214_e2685: f64 = (eq214_e2683 * p.p249);
        let eq214_e2685_d_n0: f64 = ((p.p7 * s.dn[312][0]) * p.p249);
        let eq214_e2685_d_n1: f64 = ((p.p7 * s.dn[312][1]) * p.p249);
        let eq214_e2685_d_n2: f64 = ((p.p7 * s.dn[312][2]) * p.p249);
        let eq214_e2685_d_n3: f64 = ((p.p7 * s.dn[312][3]) * p.p249);
        let eq214_e2685_d_n4: f64 = ((p.p7 * s.dn[312][4]) * p.p249);
        let eq214_e2685_d_n5: f64 = ((p.p7 * s.dn[312][5]) * p.p249);
        let eq214_e2685_d_n6: f64 = ((p.p7 * s.dn[312][6]) * p.p249);
        let eq214_e2685_d_n7: f64 = ((p.p7 * s.dn[312][7]) * p.p249);
        let eq214_e2685_d_n8: f64 = ((p.p7 * s.dn[312][8]) * p.p249);
        let eq214_e2685_d_n9: f64 = ((p.p7 * s.dn[312][9]) * p.p249);
        let eq214_e2685_d_n10: f64 = ((p.p7 * s.dn[312][10]) * p.p249);
        let eq214_e2685_d_n11: f64 = ((p.p7 * s.dn[312][11]) * p.p249);
        let eq214_e2685_d_n12: f64 = ((p.p7 * s.dn[312][12]) * p.p249);
        let eq214_e2685_d_n13: f64 = ((p.p7 * s.dn[312][13]) * p.p249);
        let eq214_e2685_d_n14: f64 = ((p.p7 * s.dn[312][14]) * p.p249);
        let eq214_e2685_d_n15: f64 = ((p.p7 * s.dn[312][15]) * p.p249);
        let eq214_e2685_d_n16: f64 = ((p.p7 * s.dn[312][16]) * p.p249);
        let eq214_e2685_d_n17: f64 = ((p.p7 * s.dn[312][17]) * p.p249);
        let eq214_e2685_d_n18: f64 = ((p.p7 * s.dn[312][18]) * p.p249);
        let eq214_e2685_d_n19: f64 = ((p.p7 * s.dn[312][19]) * p.p249);
        let eq214_e2685_d_n20: f64 = ((p.p7 * s.dn[312][20]) * p.p249);
        let eq214_e2685_d_n21: f64 = ((p.p7 * s.dn[312][21]) * p.p249);
        let eq214_e2685_d_n22: f64 = ((p.p7 * s.dn[312][22]) * p.p249);
        let eq214_e2685_d_b0: f64 = ((p.p7 * s.db[312][0]) * p.p249);
        let eq214_e2685_d_b1: f64 = ((p.p7 * s.db[312][1]) * p.p249);
        let eq214_e2685_d_b2: f64 = ((p.p7 * s.db[312][2]) * p.p249);
        let eq214_e2685_d_b3: f64 = ((p.p7 * s.db[312][3]) * p.p249);
        let eq214_e2685_d_b4: f64 = ((p.p7 * s.db[312][4]) * p.p249);
        let eq214_e2685_d_b5: f64 = ((p.p7 * s.db[312][5]) * p.p249);
        let eq214_e2685_d_b6: f64 = ((p.p7 * s.db[312][6]) * p.p249);
        let eq214_e2685_d_b7: f64 = ((p.p7 * s.db[312][7]) * p.p249);
        let eq214_e2685_d_b8: f64 = ((p.p7 * s.db[312][8]) * p.p249);
        let eq214_e2685_d_b9: f64 = ((p.p7 * s.db[312][9]) * p.p249);
        let eq214_e2685_d_b10: f64 = ((p.p7 * s.db[312][10]) * p.p249);
        let eq214_e2685_d_b11: f64 = ((p.p7 * s.db[312][11]) * p.p249);
        let eq214_e2685_d_b12: f64 = ((p.p7 * s.db[312][12]) * p.p249);
        let eq214_e2685_d_b13: f64 = ((p.p7 * s.db[312][13]) * p.p249);
        let eq214_e2685_d_b14: f64 = ((p.p7 * s.db[312][14]) * p.p249);
        let eq214_e2685_d_b15: f64 = ((p.p7 * s.db[312][15]) * p.p249);
        let eq214_e2685_d_b16: f64 = ((p.p7 * s.db[312][16]) * p.p249);
        let eq214_e2685_d_b17: f64 = ((p.p7 * s.db[312][17]) * p.p249);
        let eq214_e2685_d_b18: f64 = ((p.p7 * s.db[312][18]) * p.p249);
        let eq214_e2685_d_b19: f64 = ((p.p7 * s.db[312][19]) * p.p249);
        let eq214_e2685_d_b20: f64 = ((p.p7 * s.db[312][20]) * p.p249);
        let eq214_e2685_d_b21: f64 = ((p.p7 * s.db[312][21]) * p.p249);
        let eq214_e2685_d_b22: f64 = ((p.p7 * s.db[312][22]) * p.p249);
        let eq214_e2685_d_b23: f64 = ((p.p7 * s.db[312][23]) * p.p249);
        let eq214_e2685_d_b24: f64 = ((p.p7 * s.db[312][24]) * p.p249);
        let eq214_e2685_d_b25: f64 = ((p.p7 * s.db[312][25]) * p.p249);
        let eq214_e2685_d_b26: f64 = ((p.p7 * s.db[312][26]) * p.p249);
        let eq214_e2685_d_b27: f64 = ((p.p7 * s.db[312][27]) * p.p249);
        let eq214_e2685_d_b28: f64 = ((p.p7 * s.db[312][28]) * p.p249);
        let eq214_e2685_d_b29: f64 = ((p.p7 * s.db[312][29]) * p.p249);
        let eq214_e2685_d_b30: f64 = ((p.p7 * s.db[312][30]) * p.p249);
        let eq214_e2685_d_b31: f64 = ((p.p7 * s.db[312][31]) * p.p249);
        let eq214_e2685_d_b32: f64 = ((p.p7 * s.db[312][32]) * p.p249);
        let eq214_e2685_d_b33: f64 = ((p.p7 * s.db[312][33]) * p.p249);
        let eq214_e2685_d_b34: f64 = ((p.p7 * s.db[312][34]) * p.p249);
        let eq214_e2685_d_b35: f64 = ((p.p7 * s.db[312][35]) * p.p249);
        let eq214_e2685_d_b36: f64 = ((p.p7 * s.db[312][36]) * p.p249);
        let eq214_e2685_d_b37: f64 = ((p.p7 * s.db[312][37]) * p.p249);
        let eq214_e2685_d_b38: f64 = ((p.p7 * s.db[312][38]) * p.p249);
        let eq214_e2685_d_b39: f64 = ((p.p7 * s.db[312][39]) * p.p249);
        let eq214_e2685_d_b40: f64 = ((p.p7 * s.db[312][40]) * p.p249);
        let eq214_e2685_d_b41: f64 = ((p.p7 * s.db[312][41]) * p.p249);
        let eq214_e2685_d_b42: f64 = ((p.p7 * s.db[312][42]) * p.p249);
        let eq214_e2685_d_b43: f64 = ((p.p7 * s.db[312][43]) * p.p249);
        let eq214_e2685_d_b44: f64 = ((p.p7 * s.db[312][44]) * p.p249);
        let eq214_e2685_d_b45: f64 = ((p.p7 * s.db[312][45]) * p.p249);
        let eq214_e2685_d_b46: f64 = ((p.p7 * s.db[312][46]) * p.p249);
        let eq214_e2685_d_b47: f64 = ((p.p7 * s.db[312][47]) * p.p249);
        let eq214_e2685_d_b48: f64 = ((p.p7 * s.db[312][48]) * p.p249);
        let eq214_e2685_d_b49: f64 = ((p.p7 * s.db[312][49]) * p.p249);
        let eq214_e2685_d_b50: f64 = ((p.p7 * s.db[312][50]) * p.p249);
        let eq214_e2685_d_b51: f64 = ((p.p7 * s.db[312][51]) * p.p249);
        let eq214_e2685_d_b52: f64 = ((p.p7 * s.db[312][52]) * p.p249);
        let eq214_e2685_d_b53: f64 = ((p.p7 * s.db[312][53]) * p.p249);
        let eq214_e2685_d_b54: f64 = ((p.p7 * s.db[312][54]) * p.p249);
        let eq214_e2685_q: f64 = (eq214_e2683_q * p.p249);
        (eq214_e2685, eq214_e2685_d_n0, eq214_e2685_d_n1, eq214_e2685_d_n2, eq214_e2685_d_n3, eq214_e2685_d_n4, eq214_e2685_d_n5, eq214_e2685_d_n6, eq214_e2685_d_n7, eq214_e2685_d_n8, eq214_e2685_d_n9, eq214_e2685_d_n10, eq214_e2685_d_n11, eq214_e2685_d_n12, eq214_e2685_d_n13, eq214_e2685_d_n14, eq214_e2685_d_n15, eq214_e2685_d_n16, eq214_e2685_d_n17, eq214_e2685_d_n18, eq214_e2685_d_n19, eq214_e2685_d_n20, eq214_e2685_d_n21, eq214_e2685_d_n22, eq214_e2685_d_b0, eq214_e2685_d_b1, eq214_e2685_d_b2, eq214_e2685_d_b3, eq214_e2685_d_b4, eq214_e2685_d_b5, eq214_e2685_d_b6, eq214_e2685_d_b7, eq214_e2685_d_b8, eq214_e2685_d_b9, eq214_e2685_d_b10, eq214_e2685_d_b11, eq214_e2685_d_b12, eq214_e2685_d_b13, eq214_e2685_d_b14, eq214_e2685_d_b15, eq214_e2685_d_b16, eq214_e2685_d_b17, eq214_e2685_d_b18, eq214_e2685_d_b19, eq214_e2685_d_b20, eq214_e2685_d_b21, eq214_e2685_d_b22, eq214_e2685_d_b23, eq214_e2685_d_b24, eq214_e2685_d_b25, eq214_e2685_d_b26, eq214_e2685_d_b27, eq214_e2685_d_b28, eq214_e2685_d_b29, eq214_e2685_d_b30, eq214_e2685_d_b31, eq214_e2685_d_b32, eq214_e2685_d_b33, eq214_e2685_d_b34, eq214_e2685_d_b35, eq214_e2685_d_b36, eq214_e2685_d_b37, eq214_e2685_d_b38, eq214_e2685_d_b39, eq214_e2685_d_b40, eq214_e2685_d_b41, eq214_e2685_d_b42, eq214_e2685_d_b43, eq214_e2685_d_b44, eq214_e2685_d_b45, eq214_e2685_d_b46, eq214_e2685_d_b47, eq214_e2685_d_b48, eq214_e2685_d_b49, eq214_e2685_d_b50, eq214_e2685_d_b51, eq214_e2685_d_b52, eq214_e2685_d_b53, eq214_e2685_d_b54, eq214_e2685_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq214_reactive_node_derivatives: [f64; 23] = [eq214_e2687_d_n0, eq214_e2687_d_n1, eq214_e2687_d_n2, eq214_e2687_d_n3, eq214_e2687_d_n4, eq214_e2687_d_n5, eq214_e2687_d_n6, eq214_e2687_d_n7, eq214_e2687_d_n8, eq214_e2687_d_n9, eq214_e2687_d_n10, eq214_e2687_d_n11, eq214_e2687_d_n12, eq214_e2687_d_n13, eq214_e2687_d_n14, eq214_e2687_d_n15, eq214_e2687_d_n16, eq214_e2687_d_n17, eq214_e2687_d_n18, eq214_e2687_d_n19, eq214_e2687_d_n20, eq214_e2687_d_n21, eq214_e2687_d_n22];
        let eq214_reactive_branch_derivatives: [f64; 55] = [eq214_e2687_d_b0, eq214_e2687_d_b1, eq214_e2687_d_b2, eq214_e2687_d_b3, eq214_e2687_d_b4, eq214_e2687_d_b5, eq214_e2687_d_b6, eq214_e2687_d_b7, eq214_e2687_d_b8, eq214_e2687_d_b9, eq214_e2687_d_b10, eq214_e2687_d_b11, eq214_e2687_d_b12, eq214_e2687_d_b13, eq214_e2687_d_b14, eq214_e2687_d_b15, eq214_e2687_d_b16, eq214_e2687_d_b17, eq214_e2687_d_b18, eq214_e2687_d_b19, eq214_e2687_d_b20, eq214_e2687_d_b21, eq214_e2687_d_b22, eq214_e2687_d_b23, eq214_e2687_d_b24, eq214_e2687_d_b25, eq214_e2687_d_b26, eq214_e2687_d_b27, eq214_e2687_d_b28, eq214_e2687_d_b29, eq214_e2687_d_b30, eq214_e2687_d_b31, eq214_e2687_d_b32, eq214_e2687_d_b33, eq214_e2687_d_b34, eq214_e2687_d_b35, eq214_e2687_d_b36, eq214_e2687_d_b37, eq214_e2687_d_b38, eq214_e2687_d_b39, eq214_e2687_d_b40, eq214_e2687_d_b41, eq214_e2687_d_b42, eq214_e2687_d_b43, eq214_e2687_d_b44, eq214_e2687_d_b45, eq214_e2687_d_b46, eq214_e2687_d_b47, eq214_e2687_d_b48, eq214_e2687_d_b49, eq214_e2687_d_b50, eq214_e2687_d_b51, eq214_e2687_d_b52, eq214_e2687_d_b53, eq214_e2687_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq214_reactive_node_derivatives,
            branches,
            &eq214_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq215_e2699, eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22, eq215_e2699_d_b0, eq215_e2699_d_b1, eq215_e2699_d_b2, eq215_e2699_d_b3, eq215_e2699_d_b4, eq215_e2699_d_b5, eq215_e2699_d_b6, eq215_e2699_d_b7, eq215_e2699_d_b8, eq215_e2699_d_b9, eq215_e2699_d_b10, eq215_e2699_d_b11, eq215_e2699_d_b12, eq215_e2699_d_b13, eq215_e2699_d_b14, eq215_e2699_d_b15, eq215_e2699_d_b16, eq215_e2699_d_b17, eq215_e2699_d_b18, eq215_e2699_d_b19, eq215_e2699_d_b20, eq215_e2699_d_b21, eq215_e2699_d_b22, eq215_e2699_d_b23, eq215_e2699_d_b24, eq215_e2699_d_b25, eq215_e2699_d_b26, eq215_e2699_d_b27, eq215_e2699_d_b28, eq215_e2699_d_b29, eq215_e2699_d_b30, eq215_e2699_d_b31, eq215_e2699_d_b32, eq215_e2699_d_b33, eq215_e2699_d_b34, eq215_e2699_d_b35, eq215_e2699_d_b36, eq215_e2699_d_b37, eq215_e2699_d_b38, eq215_e2699_d_b39, eq215_e2699_d_b40, eq215_e2699_d_b41, eq215_e2699_d_b42, eq215_e2699_d_b43, eq215_e2699_d_b44, eq215_e2699_d_b45, eq215_e2699_d_b46, eq215_e2699_d_b47, eq215_e2699_d_b48, eq215_e2699_d_b49, eq215_e2699_d_b50, eq215_e2699_d_b51, eq215_e2699_d_b52, eq215_e2699_d_b53, eq215_e2699_d_b54, eq215_e2699_q,) = {
    if ((!s.b[605]) && s.b[608]) {
        let eq215_e2695: f64 = (p.p254 * s.v[312]);
        let eq215_e2696_q: f64 = eq215_e2695;
        let eq215_e2697: f64 = (p.p7 * eq215_e2695);
        let eq215_e2697_d_n0: f64 = (p.p7 * (p.p254 * s.dn[312][0]));
        let eq215_e2697_d_n1: f64 = (p.p7 * (p.p254 * s.dn[312][1]));
        let eq215_e2697_d_n2: f64 = (p.p7 * (p.p254 * s.dn[312][2]));
        let eq215_e2697_d_n3: f64 = (p.p7 * (p.p254 * s.dn[312][3]));
        let eq215_e2697_d_n4: f64 = (p.p7 * (p.p254 * s.dn[312][4]));
        let eq215_e2697_d_n5: f64 = (p.p7 * (p.p254 * s.dn[312][5]));
        let eq215_e2697_d_n6: f64 = (p.p7 * (p.p254 * s.dn[312][6]));
        let eq215_e2697_d_n7: f64 = (p.p7 * (p.p254 * s.dn[312][7]));
        let eq215_e2697_d_n8: f64 = (p.p7 * (p.p254 * s.dn[312][8]));
        let eq215_e2697_d_n9: f64 = (p.p7 * (p.p254 * s.dn[312][9]));
        let eq215_e2697_d_n10: f64 = (p.p7 * (p.p254 * s.dn[312][10]));
        let eq215_e2697_d_n11: f64 = (p.p7 * (p.p254 * s.dn[312][11]));
        let eq215_e2697_d_n12: f64 = (p.p7 * (p.p254 * s.dn[312][12]));
        let eq215_e2697_d_n13: f64 = (p.p7 * (p.p254 * s.dn[312][13]));
        let eq215_e2697_d_n14: f64 = (p.p7 * (p.p254 * s.dn[312][14]));
        let eq215_e2697_d_n15: f64 = (p.p7 * (p.p254 * s.dn[312][15]));
        let eq215_e2697_d_n16: f64 = (p.p7 * (p.p254 * s.dn[312][16]));
        let eq215_e2697_d_n17: f64 = (p.p7 * (p.p254 * s.dn[312][17]));
        let eq215_e2697_d_n18: f64 = (p.p7 * (p.p254 * s.dn[312][18]));
        let eq215_e2697_d_n19: f64 = (p.p7 * (p.p254 * s.dn[312][19]));
        let eq215_e2697_d_n20: f64 = (p.p7 * (p.p254 * s.dn[312][20]));
        let eq215_e2697_d_n21: f64 = (p.p7 * (p.p254 * s.dn[312][21]));
        let eq215_e2697_d_n22: f64 = (p.p7 * (p.p254 * s.dn[312][22]));
        let eq215_e2697_d_b0: f64 = (p.p7 * (p.p254 * s.db[312][0]));
        let eq215_e2697_d_b1: f64 = (p.p7 * (p.p254 * s.db[312][1]));
        let eq215_e2697_d_b2: f64 = (p.p7 * (p.p254 * s.db[312][2]));
        let eq215_e2697_d_b3: f64 = (p.p7 * (p.p254 * s.db[312][3]));
        let eq215_e2697_d_b4: f64 = (p.p7 * (p.p254 * s.db[312][4]));
        let eq215_e2697_d_b5: f64 = (p.p7 * (p.p254 * s.db[312][5]));
        let eq215_e2697_d_b6: f64 = (p.p7 * (p.p254 * s.db[312][6]));
        let eq215_e2697_d_b7: f64 = (p.p7 * (p.p254 * s.db[312][7]));
        let eq215_e2697_d_b8: f64 = (p.p7 * (p.p254 * s.db[312][8]));
        let eq215_e2697_d_b9: f64 = (p.p7 * (p.p254 * s.db[312][9]));
        let eq215_e2697_d_b10: f64 = (p.p7 * (p.p254 * s.db[312][10]));
        let eq215_e2697_d_b11: f64 = (p.p7 * (p.p254 * s.db[312][11]));
        let eq215_e2697_d_b12: f64 = (p.p7 * (p.p254 * s.db[312][12]));
        let eq215_e2697_d_b13: f64 = (p.p7 * (p.p254 * s.db[312][13]));
        let eq215_e2697_d_b14: f64 = (p.p7 * (p.p254 * s.db[312][14]));
        let eq215_e2697_d_b15: f64 = (p.p7 * (p.p254 * s.db[312][15]));
        let eq215_e2697_d_b16: f64 = (p.p7 * (p.p254 * s.db[312][16]));
        let eq215_e2697_d_b17: f64 = (p.p7 * (p.p254 * s.db[312][17]));
        let eq215_e2697_d_b18: f64 = (p.p7 * (p.p254 * s.db[312][18]));
        let eq215_e2697_d_b19: f64 = (p.p7 * (p.p254 * s.db[312][19]));
        let eq215_e2697_d_b20: f64 = (p.p7 * (p.p254 * s.db[312][20]));
        let eq215_e2697_d_b21: f64 = (p.p7 * (p.p254 * s.db[312][21]));
        let eq215_e2697_d_b22: f64 = (p.p7 * (p.p254 * s.db[312][22]));
        let eq215_e2697_d_b23: f64 = (p.p7 * (p.p254 * s.db[312][23]));
        let eq215_e2697_d_b24: f64 = (p.p7 * (p.p254 * s.db[312][24]));
        let eq215_e2697_d_b25: f64 = (p.p7 * (p.p254 * s.db[312][25]));
        let eq215_e2697_d_b26: f64 = (p.p7 * (p.p254 * s.db[312][26]));
        let eq215_e2697_d_b27: f64 = (p.p7 * (p.p254 * s.db[312][27]));
        let eq215_e2697_d_b28: f64 = (p.p7 * (p.p254 * s.db[312][28]));
        let eq215_e2697_d_b29: f64 = (p.p7 * (p.p254 * s.db[312][29]));
        let eq215_e2697_d_b30: f64 = (p.p7 * (p.p254 * s.db[312][30]));
        let eq215_e2697_d_b31: f64 = (p.p7 * (p.p254 * s.db[312][31]));
        let eq215_e2697_d_b32: f64 = (p.p7 * (p.p254 * s.db[312][32]));
        let eq215_e2697_d_b33: f64 = (p.p7 * (p.p254 * s.db[312][33]));
        let eq215_e2697_d_b34: f64 = (p.p7 * (p.p254 * s.db[312][34]));
        let eq215_e2697_d_b35: f64 = (p.p7 * (p.p254 * s.db[312][35]));
        let eq215_e2697_d_b36: f64 = (p.p7 * (p.p254 * s.db[312][36]));
        let eq215_e2697_d_b37: f64 = (p.p7 * (p.p254 * s.db[312][37]));
        let eq215_e2697_d_b38: f64 = (p.p7 * (p.p254 * s.db[312][38]));
        let eq215_e2697_d_b39: f64 = (p.p7 * (p.p254 * s.db[312][39]));
        let eq215_e2697_d_b40: f64 = (p.p7 * (p.p254 * s.db[312][40]));
        let eq215_e2697_d_b41: f64 = (p.p7 * (p.p254 * s.db[312][41]));
        let eq215_e2697_d_b42: f64 = (p.p7 * (p.p254 * s.db[312][42]));
        let eq215_e2697_d_b43: f64 = (p.p7 * (p.p254 * s.db[312][43]));
        let eq215_e2697_d_b44: f64 = (p.p7 * (p.p254 * s.db[312][44]));
        let eq215_e2697_d_b45: f64 = (p.p7 * (p.p254 * s.db[312][45]));
        let eq215_e2697_d_b46: f64 = (p.p7 * (p.p254 * s.db[312][46]));
        let eq215_e2697_d_b47: f64 = (p.p7 * (p.p254 * s.db[312][47]));
        let eq215_e2697_d_b48: f64 = (p.p7 * (p.p254 * s.db[312][48]));
        let eq215_e2697_d_b49: f64 = (p.p7 * (p.p254 * s.db[312][49]));
        let eq215_e2697_d_b50: f64 = (p.p7 * (p.p254 * s.db[312][50]));
        let eq215_e2697_d_b51: f64 = (p.p7 * (p.p254 * s.db[312][51]));
        let eq215_e2697_d_b52: f64 = (p.p7 * (p.p254 * s.db[312][52]));
        let eq215_e2697_d_b53: f64 = (p.p7 * (p.p254 * s.db[312][53]));
        let eq215_e2697_d_b54: f64 = (p.p7 * (p.p254 * s.db[312][54]));
        let eq215_e2697_q: f64 = (p.p7 * eq215_e2696_q);
        (eq215_e2697, eq215_e2697_d_n0, eq215_e2697_d_n1, eq215_e2697_d_n2, eq215_e2697_d_n3, eq215_e2697_d_n4, eq215_e2697_d_n5, eq215_e2697_d_n6, eq215_e2697_d_n7, eq215_e2697_d_n8, eq215_e2697_d_n9, eq215_e2697_d_n10, eq215_e2697_d_n11, eq215_e2697_d_n12, eq215_e2697_d_n13, eq215_e2697_d_n14, eq215_e2697_d_n15, eq215_e2697_d_n16, eq215_e2697_d_n17, eq215_e2697_d_n18, eq215_e2697_d_n19, eq215_e2697_d_n20, eq215_e2697_d_n21, eq215_e2697_d_n22, eq215_e2697_d_b0, eq215_e2697_d_b1, eq215_e2697_d_b2, eq215_e2697_d_b3, eq215_e2697_d_b4, eq215_e2697_d_b5, eq215_e2697_d_b6, eq215_e2697_d_b7, eq215_e2697_d_b8, eq215_e2697_d_b9, eq215_e2697_d_b10, eq215_e2697_d_b11, eq215_e2697_d_b12, eq215_e2697_d_b13, eq215_e2697_d_b14, eq215_e2697_d_b15, eq215_e2697_d_b16, eq215_e2697_d_b17, eq215_e2697_d_b18, eq215_e2697_d_b19, eq215_e2697_d_b20, eq215_e2697_d_b21, eq215_e2697_d_b22, eq215_e2697_d_b23, eq215_e2697_d_b24, eq215_e2697_d_b25, eq215_e2697_d_b26, eq215_e2697_d_b27, eq215_e2697_d_b28, eq215_e2697_d_b29, eq215_e2697_d_b30, eq215_e2697_d_b31, eq215_e2697_d_b32, eq215_e2697_d_b33, eq215_e2697_d_b34, eq215_e2697_d_b35, eq215_e2697_d_b36, eq215_e2697_d_b37, eq215_e2697_d_b38, eq215_e2697_d_b39, eq215_e2697_d_b40, eq215_e2697_d_b41, eq215_e2697_d_b42, eq215_e2697_d_b43, eq215_e2697_d_b44, eq215_e2697_d_b45, eq215_e2697_d_b46, eq215_e2697_d_b47, eq215_e2697_d_b48, eq215_e2697_d_b49, eq215_e2697_d_b50, eq215_e2697_d_b51, eq215_e2697_d_b52, eq215_e2697_d_b53, eq215_e2697_d_b54, eq215_e2697_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq215_reactive_node_derivatives: [f64; 23] = [eq215_e2699_d_n0, eq215_e2699_d_n1, eq215_e2699_d_n2, eq215_e2699_d_n3, eq215_e2699_d_n4, eq215_e2699_d_n5, eq215_e2699_d_n6, eq215_e2699_d_n7, eq215_e2699_d_n8, eq215_e2699_d_n9, eq215_e2699_d_n10, eq215_e2699_d_n11, eq215_e2699_d_n12, eq215_e2699_d_n13, eq215_e2699_d_n14, eq215_e2699_d_n15, eq215_e2699_d_n16, eq215_e2699_d_n17, eq215_e2699_d_n18, eq215_e2699_d_n19, eq215_e2699_d_n20, eq215_e2699_d_n21, eq215_e2699_d_n22];
        let eq215_reactive_branch_derivatives: [f64; 55] = [eq215_e2699_d_b0, eq215_e2699_d_b1, eq215_e2699_d_b2, eq215_e2699_d_b3, eq215_e2699_d_b4, eq215_e2699_d_b5, eq215_e2699_d_b6, eq215_e2699_d_b7, eq215_e2699_d_b8, eq215_e2699_d_b9, eq215_e2699_d_b10, eq215_e2699_d_b11, eq215_e2699_d_b12, eq215_e2699_d_b13, eq215_e2699_d_b14, eq215_e2699_d_b15, eq215_e2699_d_b16, eq215_e2699_d_b17, eq215_e2699_d_b18, eq215_e2699_d_b19, eq215_e2699_d_b20, eq215_e2699_d_b21, eq215_e2699_d_b22, eq215_e2699_d_b23, eq215_e2699_d_b24, eq215_e2699_d_b25, eq215_e2699_d_b26, eq215_e2699_d_b27, eq215_e2699_d_b28, eq215_e2699_d_b29, eq215_e2699_d_b30, eq215_e2699_d_b31, eq215_e2699_d_b32, eq215_e2699_d_b33, eq215_e2699_d_b34, eq215_e2699_d_b35, eq215_e2699_d_b36, eq215_e2699_d_b37, eq215_e2699_d_b38, eq215_e2699_d_b39, eq215_e2699_d_b40, eq215_e2699_d_b41, eq215_e2699_d_b42, eq215_e2699_d_b43, eq215_e2699_d_b44, eq215_e2699_d_b45, eq215_e2699_d_b46, eq215_e2699_d_b47, eq215_e2699_d_b48, eq215_e2699_d_b49, eq215_e2699_d_b50, eq215_e2699_d_b51, eq215_e2699_d_b52, eq215_e2699_d_b53, eq215_e2699_d_b54];
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
        let eq216_e2703_q: f64 = (p.p7 * eq216_e2702_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &s.dn[195],
            branches,
            &s.db[195],
            (multiplicity) * (p.p7),
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
        let eq218_e2716_q: f64 = s.v[196];
        let eq218_e2717: f64 = (p.p7 * s.v[196]);
        let eq218_e2717_q: f64 = (p.p7 * eq218_e2716_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &s.dn[196],
            branches,
            &s.db[196],
            (multiplicity) * (p.p7),
        );
        let eq219_e2720_q: f64 = s.v[197];
        let eq219_e2721: f64 = (p.p7 * s.v[197]);
        let eq219_e2721_q: f64 = (p.p7 * eq219_e2720_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &s.dn[197],
            branches,
            &s.db[197],
            (multiplicity) * (p.p7),
        );
        let eq220_e2724_q: f64 = s.v[194];
        let eq220_e2725: f64 = (p.p7 * s.v[194]);
        let eq220_e2725_q: f64 = (p.p7 * eq220_e2724_q);
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[0]),
            nodes,
            &s.dn[194],
            branches,
            &s.db[194],
            (multiplicity) * (p.p7),
        );
        let (eq223_e2771, eq223_e2771_d_n4, eq223_e2771_q,) = {
    if s.b[610] {
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
