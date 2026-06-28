#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
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
        let eq116_e1517_d_b0: f64 = (p.p7 * s.db[219][0]);
        let eq116_e1517_d_b1: f64 = (p.p7 * s.db[219][1]);
        let eq116_e1517_d_b2: f64 = (p.p7 * s.db[219][2]);
        let eq116_e1517_d_b3: f64 = (p.p7 * s.db[219][3]);
        let eq116_e1517_d_b4: f64 = (p.p7 * s.db[219][4]);
        let eq116_e1517_d_b5: f64 = (p.p7 * s.db[219][5]);
        let eq116_e1517_d_b6: f64 = (p.p7 * s.db[219][6]);
        let eq116_e1517_d_b7: f64 = (p.p7 * s.db[219][7]);
        let eq116_e1517_d_b8: f64 = (p.p7 * s.db[219][8]);
        let eq116_e1517_d_b9: f64 = (p.p7 * s.db[219][9]);
        let eq116_e1517_d_b10: f64 = (p.p7 * s.db[219][10]);
        let eq116_e1517_d_b11: f64 = (p.p7 * s.db[219][11]);
        let eq116_e1517_d_b12: f64 = (p.p7 * s.db[219][12]);
        let eq116_e1517_d_b13: f64 = (p.p7 * s.db[219][13]);
        let eq116_e1517_d_b14: f64 = (p.p7 * s.db[219][14]);
        let eq116_e1517_d_b15: f64 = (p.p7 * s.db[219][15]);
        let eq116_e1517_d_b16: f64 = (p.p7 * s.db[219][16]);
        let eq116_e1517_d_b17: f64 = (p.p7 * s.db[219][17]);
        let eq116_e1517_d_b18: f64 = (p.p7 * s.db[219][18]);
        let eq116_e1517_d_b19: f64 = (p.p7 * s.db[219][19]);
        let eq116_e1517_d_b20: f64 = (p.p7 * s.db[219][20]);
        let eq116_e1517_d_b21: f64 = (p.p7 * s.db[219][21]);
        let eq116_e1517_d_b22: f64 = (p.p7 * s.db[219][22]);
        let eq116_e1517_d_b23: f64 = (p.p7 * s.db[219][23]);
        let eq116_e1517_d_b24: f64 = (p.p7 * s.db[219][24]);
        let eq116_e1517_d_b25: f64 = (p.p7 * s.db[219][25]);
        let eq116_e1517_d_b26: f64 = (p.p7 * s.db[219][26]);
        let eq116_e1517_d_b27: f64 = (p.p7 * s.db[219][27]);
        let eq116_e1517_d_b28: f64 = (p.p7 * s.db[219][28]);
        let eq116_e1517_d_b29: f64 = (p.p7 * s.db[219][29]);
        let eq116_e1517_d_b30: f64 = (p.p7 * s.db[219][30]);
        let eq116_e1517_d_b31: f64 = (p.p7 * s.db[219][31]);
        let eq116_e1517_d_b32: f64 = (p.p7 * s.db[219][32]);
        let eq116_e1517_d_b33: f64 = (p.p7 * s.db[219][33]);
        let eq116_e1517_d_b34: f64 = (p.p7 * s.db[219][34]);
        let eq116_e1517_d_b35: f64 = (p.p7 * s.db[219][35]);
        let eq116_e1517_d_b36: f64 = (p.p7 * s.db[219][36]);
        let eq116_e1517_d_b37: f64 = (p.p7 * s.db[219][37]);
        let eq116_e1517_d_b38: f64 = (p.p7 * s.db[219][38]);
        let eq116_e1517_d_b39: f64 = (p.p7 * s.db[219][39]);
        let eq116_e1517_d_b40: f64 = (p.p7 * s.db[219][40]);
        let eq116_e1517_d_b41: f64 = (p.p7 * s.db[219][41]);
        let eq116_e1517_d_b42: f64 = (p.p7 * s.db[219][42]);
        let eq116_e1517_d_b43: f64 = (p.p7 * s.db[219][43]);
        let eq116_e1517_d_b44: f64 = (p.p7 * s.db[219][44]);
        let eq116_e1517_d_b45: f64 = (p.p7 * s.db[219][45]);
        let eq116_e1517_d_b46: f64 = (p.p7 * s.db[219][46]);
        let eq116_e1517_d_b47: f64 = (p.p7 * s.db[219][47]);
        let eq116_e1517_d_b48: f64 = (p.p7 * s.db[219][48]);
        let eq116_e1517_d_b49: f64 = (p.p7 * s.db[219][49]);
        let eq116_e1517_d_b50: f64 = (p.p7 * s.db[219][50]);
        let eq116_e1517_d_b51: f64 = (p.p7 * s.db[219][51]);
        let eq116_e1517_d_b52: f64 = (p.p7 * s.db[219][52]);
        let eq116_e1517_d_b53: f64 = (p.p7 * s.db[219][53]);
        let eq116_e1517_d_b54: f64 = (p.p7 * s.db[219][54]);
        let eq116_e1517_q: f64 = (p.p7 * eq116_e1516_q);
        let eq116_reactive_node_derivatives: [f64; 23] = [eq116_e1517_d_n0, eq116_e1517_d_n1, eq116_e1517_d_n2, eq116_e1517_d_n3, eq116_e1517_d_n4, eq116_e1517_d_n5, eq116_e1517_d_n6, eq116_e1517_d_n7, eq116_e1517_d_n8, eq116_e1517_d_n9, eq116_e1517_d_n10, eq116_e1517_d_n11, eq116_e1517_d_n12, eq116_e1517_d_n13, eq116_e1517_d_n14, eq116_e1517_d_n15, eq116_e1517_d_n16, eq116_e1517_d_n17, eq116_e1517_d_n18, eq116_e1517_d_n19, eq116_e1517_d_n20, eq116_e1517_d_n21, eq116_e1517_d_n22];
        let eq116_reactive_branch_derivatives: [f64; 55] = [eq116_e1517_d_b0, eq116_e1517_d_b1, eq116_e1517_d_b2, eq116_e1517_d_b3, eq116_e1517_d_b4, eq116_e1517_d_b5, eq116_e1517_d_b6, eq116_e1517_d_b7, eq116_e1517_d_b8, eq116_e1517_d_b9, eq116_e1517_d_b10, eq116_e1517_d_b11, eq116_e1517_d_b12, eq116_e1517_d_b13, eq116_e1517_d_b14, eq116_e1517_d_b15, eq116_e1517_d_b16, eq116_e1517_d_b17, eq116_e1517_d_b18, eq116_e1517_d_b19, eq116_e1517_d_b20, eq116_e1517_d_b21, eq116_e1517_d_b22, eq116_e1517_d_b23, eq116_e1517_d_b24, eq116_e1517_d_b25, eq116_e1517_d_b26, eq116_e1517_d_b27, eq116_e1517_d_b28, eq116_e1517_d_b29, eq116_e1517_d_b30, eq116_e1517_d_b31, eq116_e1517_d_b32, eq116_e1517_d_b33, eq116_e1517_d_b34, eq116_e1517_d_b35, eq116_e1517_d_b36, eq116_e1517_d_b37, eq116_e1517_d_b38, eq116_e1517_d_b39, eq116_e1517_d_b40, eq116_e1517_d_b41, eq116_e1517_d_b42, eq116_e1517_d_b43, eq116_e1517_d_b44, eq116_e1517_d_b45, eq116_e1517_d_b46, eq116_e1517_d_b47, eq116_e1517_d_b48, eq116_e1517_d_b49, eq116_e1517_d_b50, eq116_e1517_d_b51, eq116_e1517_d_b52, eq116_e1517_d_b53, eq116_e1517_d_b54];
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
        let eq117_e1521_d_b0: f64 = (p.p7 * s.db[220][0]);
        let eq117_e1521_d_b1: f64 = (p.p7 * s.db[220][1]);
        let eq117_e1521_d_b2: f64 = (p.p7 * s.db[220][2]);
        let eq117_e1521_d_b3: f64 = (p.p7 * s.db[220][3]);
        let eq117_e1521_d_b4: f64 = (p.p7 * s.db[220][4]);
        let eq117_e1521_d_b5: f64 = (p.p7 * s.db[220][5]);
        let eq117_e1521_d_b6: f64 = (p.p7 * s.db[220][6]);
        let eq117_e1521_d_b7: f64 = (p.p7 * s.db[220][7]);
        let eq117_e1521_d_b8: f64 = (p.p7 * s.db[220][8]);
        let eq117_e1521_d_b9: f64 = (p.p7 * s.db[220][9]);
        let eq117_e1521_d_b10: f64 = (p.p7 * s.db[220][10]);
        let eq117_e1521_d_b11: f64 = (p.p7 * s.db[220][11]);
        let eq117_e1521_d_b12: f64 = (p.p7 * s.db[220][12]);
        let eq117_e1521_d_b13: f64 = (p.p7 * s.db[220][13]);
        let eq117_e1521_d_b14: f64 = (p.p7 * s.db[220][14]);
        let eq117_e1521_d_b15: f64 = (p.p7 * s.db[220][15]);
        let eq117_e1521_d_b16: f64 = (p.p7 * s.db[220][16]);
        let eq117_e1521_d_b17: f64 = (p.p7 * s.db[220][17]);
        let eq117_e1521_d_b18: f64 = (p.p7 * s.db[220][18]);
        let eq117_e1521_d_b19: f64 = (p.p7 * s.db[220][19]);
        let eq117_e1521_d_b20: f64 = (p.p7 * s.db[220][20]);
        let eq117_e1521_d_b21: f64 = (p.p7 * s.db[220][21]);
        let eq117_e1521_d_b22: f64 = (p.p7 * s.db[220][22]);
        let eq117_e1521_d_b23: f64 = (p.p7 * s.db[220][23]);
        let eq117_e1521_d_b24: f64 = (p.p7 * s.db[220][24]);
        let eq117_e1521_d_b25: f64 = (p.p7 * s.db[220][25]);
        let eq117_e1521_d_b26: f64 = (p.p7 * s.db[220][26]);
        let eq117_e1521_d_b27: f64 = (p.p7 * s.db[220][27]);
        let eq117_e1521_d_b28: f64 = (p.p7 * s.db[220][28]);
        let eq117_e1521_d_b29: f64 = (p.p7 * s.db[220][29]);
        let eq117_e1521_d_b30: f64 = (p.p7 * s.db[220][30]);
        let eq117_e1521_d_b31: f64 = (p.p7 * s.db[220][31]);
        let eq117_e1521_d_b32: f64 = (p.p7 * s.db[220][32]);
        let eq117_e1521_d_b33: f64 = (p.p7 * s.db[220][33]);
        let eq117_e1521_d_b34: f64 = (p.p7 * s.db[220][34]);
        let eq117_e1521_d_b35: f64 = (p.p7 * s.db[220][35]);
        let eq117_e1521_d_b36: f64 = (p.p7 * s.db[220][36]);
        let eq117_e1521_d_b37: f64 = (p.p7 * s.db[220][37]);
        let eq117_e1521_d_b38: f64 = (p.p7 * s.db[220][38]);
        let eq117_e1521_d_b39: f64 = (p.p7 * s.db[220][39]);
        let eq117_e1521_d_b40: f64 = (p.p7 * s.db[220][40]);
        let eq117_e1521_d_b41: f64 = (p.p7 * s.db[220][41]);
        let eq117_e1521_d_b42: f64 = (p.p7 * s.db[220][42]);
        let eq117_e1521_d_b43: f64 = (p.p7 * s.db[220][43]);
        let eq117_e1521_d_b44: f64 = (p.p7 * s.db[220][44]);
        let eq117_e1521_d_b45: f64 = (p.p7 * s.db[220][45]);
        let eq117_e1521_d_b46: f64 = (p.p7 * s.db[220][46]);
        let eq117_e1521_d_b47: f64 = (p.p7 * s.db[220][47]);
        let eq117_e1521_d_b48: f64 = (p.p7 * s.db[220][48]);
        let eq117_e1521_d_b49: f64 = (p.p7 * s.db[220][49]);
        let eq117_e1521_d_b50: f64 = (p.p7 * s.db[220][50]);
        let eq117_e1521_d_b51: f64 = (p.p7 * s.db[220][51]);
        let eq117_e1521_d_b52: f64 = (p.p7 * s.db[220][52]);
        let eq117_e1521_d_b53: f64 = (p.p7 * s.db[220][53]);
        let eq117_e1521_d_b54: f64 = (p.p7 * s.db[220][54]);
        let eq117_e1521_q: f64 = (p.p7 * eq117_e1520_q);
        let eq117_reactive_node_derivatives: [f64; 23] = [eq117_e1521_d_n0, eq117_e1521_d_n1, eq117_e1521_d_n2, eq117_e1521_d_n3, eq117_e1521_d_n4, eq117_e1521_d_n5, eq117_e1521_d_n6, eq117_e1521_d_n7, eq117_e1521_d_n8, eq117_e1521_d_n9, eq117_e1521_d_n10, eq117_e1521_d_n11, eq117_e1521_d_n12, eq117_e1521_d_n13, eq117_e1521_d_n14, eq117_e1521_d_n15, eq117_e1521_d_n16, eq117_e1521_d_n17, eq117_e1521_d_n18, eq117_e1521_d_n19, eq117_e1521_d_n20, eq117_e1521_d_n21, eq117_e1521_d_n22];
        let eq117_reactive_branch_derivatives: [f64; 55] = [eq117_e1521_d_b0, eq117_e1521_d_b1, eq117_e1521_d_b2, eq117_e1521_d_b3, eq117_e1521_d_b4, eq117_e1521_d_b5, eq117_e1521_d_b6, eq117_e1521_d_b7, eq117_e1521_d_b8, eq117_e1521_d_b9, eq117_e1521_d_b10, eq117_e1521_d_b11, eq117_e1521_d_b12, eq117_e1521_d_b13, eq117_e1521_d_b14, eq117_e1521_d_b15, eq117_e1521_d_b16, eq117_e1521_d_b17, eq117_e1521_d_b18, eq117_e1521_d_b19, eq117_e1521_d_b20, eq117_e1521_d_b21, eq117_e1521_d_b22, eq117_e1521_d_b23, eq117_e1521_d_b24, eq117_e1521_d_b25, eq117_e1521_d_b26, eq117_e1521_d_b27, eq117_e1521_d_b28, eq117_e1521_d_b29, eq117_e1521_d_b30, eq117_e1521_d_b31, eq117_e1521_d_b32, eq117_e1521_d_b33, eq117_e1521_d_b34, eq117_e1521_d_b35, eq117_e1521_d_b36, eq117_e1521_d_b37, eq117_e1521_d_b38, eq117_e1521_d_b39, eq117_e1521_d_b40, eq117_e1521_d_b41, eq117_e1521_d_b42, eq117_e1521_d_b43, eq117_e1521_d_b44, eq117_e1521_d_b45, eq117_e1521_d_b46, eq117_e1521_d_b47, eq117_e1521_d_b48, eq117_e1521_d_b49, eq117_e1521_d_b50, eq117_e1521_d_b51, eq117_e1521_d_b52, eq117_e1521_d_b53, eq117_e1521_d_b54];
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
        let eq118_e1525_d_b0: f64 = (p.p7 * s.db[221][0]);
        let eq118_e1525_d_b1: f64 = (p.p7 * s.db[221][1]);
        let eq118_e1525_d_b2: f64 = (p.p7 * s.db[221][2]);
        let eq118_e1525_d_b3: f64 = (p.p7 * s.db[221][3]);
        let eq118_e1525_d_b4: f64 = (p.p7 * s.db[221][4]);
        let eq118_e1525_d_b5: f64 = (p.p7 * s.db[221][5]);
        let eq118_e1525_d_b6: f64 = (p.p7 * s.db[221][6]);
        let eq118_e1525_d_b7: f64 = (p.p7 * s.db[221][7]);
        let eq118_e1525_d_b8: f64 = (p.p7 * s.db[221][8]);
        let eq118_e1525_d_b9: f64 = (p.p7 * s.db[221][9]);
        let eq118_e1525_d_b10: f64 = (p.p7 * s.db[221][10]);
        let eq118_e1525_d_b11: f64 = (p.p7 * s.db[221][11]);
        let eq118_e1525_d_b12: f64 = (p.p7 * s.db[221][12]);
        let eq118_e1525_d_b13: f64 = (p.p7 * s.db[221][13]);
        let eq118_e1525_d_b14: f64 = (p.p7 * s.db[221][14]);
        let eq118_e1525_d_b15: f64 = (p.p7 * s.db[221][15]);
        let eq118_e1525_d_b16: f64 = (p.p7 * s.db[221][16]);
        let eq118_e1525_d_b17: f64 = (p.p7 * s.db[221][17]);
        let eq118_e1525_d_b18: f64 = (p.p7 * s.db[221][18]);
        let eq118_e1525_d_b19: f64 = (p.p7 * s.db[221][19]);
        let eq118_e1525_d_b20: f64 = (p.p7 * s.db[221][20]);
        let eq118_e1525_d_b21: f64 = (p.p7 * s.db[221][21]);
        let eq118_e1525_d_b22: f64 = (p.p7 * s.db[221][22]);
        let eq118_e1525_d_b23: f64 = (p.p7 * s.db[221][23]);
        let eq118_e1525_d_b24: f64 = (p.p7 * s.db[221][24]);
        let eq118_e1525_d_b25: f64 = (p.p7 * s.db[221][25]);
        let eq118_e1525_d_b26: f64 = (p.p7 * s.db[221][26]);
        let eq118_e1525_d_b27: f64 = (p.p7 * s.db[221][27]);
        let eq118_e1525_d_b28: f64 = (p.p7 * s.db[221][28]);
        let eq118_e1525_d_b29: f64 = (p.p7 * s.db[221][29]);
        let eq118_e1525_d_b30: f64 = (p.p7 * s.db[221][30]);
        let eq118_e1525_d_b31: f64 = (p.p7 * s.db[221][31]);
        let eq118_e1525_d_b32: f64 = (p.p7 * s.db[221][32]);
        let eq118_e1525_d_b33: f64 = (p.p7 * s.db[221][33]);
        let eq118_e1525_d_b34: f64 = (p.p7 * s.db[221][34]);
        let eq118_e1525_d_b35: f64 = (p.p7 * s.db[221][35]);
        let eq118_e1525_d_b36: f64 = (p.p7 * s.db[221][36]);
        let eq118_e1525_d_b37: f64 = (p.p7 * s.db[221][37]);
        let eq118_e1525_d_b38: f64 = (p.p7 * s.db[221][38]);
        let eq118_e1525_d_b39: f64 = (p.p7 * s.db[221][39]);
        let eq118_e1525_d_b40: f64 = (p.p7 * s.db[221][40]);
        let eq118_e1525_d_b41: f64 = (p.p7 * s.db[221][41]);
        let eq118_e1525_d_b42: f64 = (p.p7 * s.db[221][42]);
        let eq118_e1525_d_b43: f64 = (p.p7 * s.db[221][43]);
        let eq118_e1525_d_b44: f64 = (p.p7 * s.db[221][44]);
        let eq118_e1525_d_b45: f64 = (p.p7 * s.db[221][45]);
        let eq118_e1525_d_b46: f64 = (p.p7 * s.db[221][46]);
        let eq118_e1525_d_b47: f64 = (p.p7 * s.db[221][47]);
        let eq118_e1525_d_b48: f64 = (p.p7 * s.db[221][48]);
        let eq118_e1525_d_b49: f64 = (p.p7 * s.db[221][49]);
        let eq118_e1525_d_b50: f64 = (p.p7 * s.db[221][50]);
        let eq118_e1525_d_b51: f64 = (p.p7 * s.db[221][51]);
        let eq118_e1525_d_b52: f64 = (p.p7 * s.db[221][52]);
        let eq118_e1525_d_b53: f64 = (p.p7 * s.db[221][53]);
        let eq118_e1525_d_b54: f64 = (p.p7 * s.db[221][54]);
        let eq118_e1525_q: f64 = (p.p7 * eq118_e1524_q);
        let eq118_reactive_node_derivatives: [f64; 23] = [eq118_e1525_d_n0, eq118_e1525_d_n1, eq118_e1525_d_n2, eq118_e1525_d_n3, eq118_e1525_d_n4, eq118_e1525_d_n5, eq118_e1525_d_n6, eq118_e1525_d_n7, eq118_e1525_d_n8, eq118_e1525_d_n9, eq118_e1525_d_n10, eq118_e1525_d_n11, eq118_e1525_d_n12, eq118_e1525_d_n13, eq118_e1525_d_n14, eq118_e1525_d_n15, eq118_e1525_d_n16, eq118_e1525_d_n17, eq118_e1525_d_n18, eq118_e1525_d_n19, eq118_e1525_d_n20, eq118_e1525_d_n21, eq118_e1525_d_n22];
        let eq118_reactive_branch_derivatives: [f64; 55] = [eq118_e1525_d_b0, eq118_e1525_d_b1, eq118_e1525_d_b2, eq118_e1525_d_b3, eq118_e1525_d_b4, eq118_e1525_d_b5, eq118_e1525_d_b6, eq118_e1525_d_b7, eq118_e1525_d_b8, eq118_e1525_d_b9, eq118_e1525_d_b10, eq118_e1525_d_b11, eq118_e1525_d_b12, eq118_e1525_d_b13, eq118_e1525_d_b14, eq118_e1525_d_b15, eq118_e1525_d_b16, eq118_e1525_d_b17, eq118_e1525_d_b18, eq118_e1525_d_b19, eq118_e1525_d_b20, eq118_e1525_d_b21, eq118_e1525_d_b22, eq118_e1525_d_b23, eq118_e1525_d_b24, eq118_e1525_d_b25, eq118_e1525_d_b26, eq118_e1525_d_b27, eq118_e1525_d_b28, eq118_e1525_d_b29, eq118_e1525_d_b30, eq118_e1525_d_b31, eq118_e1525_d_b32, eq118_e1525_d_b33, eq118_e1525_d_b34, eq118_e1525_d_b35, eq118_e1525_d_b36, eq118_e1525_d_b37, eq118_e1525_d_b38, eq118_e1525_d_b39, eq118_e1525_d_b40, eq118_e1525_d_b41, eq118_e1525_d_b42, eq118_e1525_d_b43, eq118_e1525_d_b44, eq118_e1525_d_b45, eq118_e1525_d_b46, eq118_e1525_d_b47, eq118_e1525_d_b48, eq118_e1525_d_b49, eq118_e1525_d_b50, eq118_e1525_d_b51, eq118_e1525_d_b52, eq118_e1525_d_b53, eq118_e1525_d_b54];
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
        let eq119_e1529_d_b0: f64 = (p.p250 * s.db[161][0]);
        let eq119_e1529_d_b1: f64 = (p.p250 * s.db[161][1]);
        let eq119_e1529_d_b2: f64 = (p.p250 * s.db[161][2]);
        let eq119_e1529_d_b3: f64 = (p.p250 * s.db[161][3]);
        let eq119_e1529_d_b4: f64 = (p.p250 * s.db[161][4]);
        let eq119_e1529_d_b5: f64 = (p.p250 * s.db[161][5]);
        let eq119_e1529_d_b6: f64 = (p.p250 * s.db[161][6]);
        let eq119_e1529_d_b7: f64 = (p.p250 * s.db[161][7]);
        let eq119_e1529_d_b8: f64 = (p.p250 * s.db[161][8]);
        let eq119_e1529_d_b9: f64 = (p.p250 * s.db[161][9]);
        let eq119_e1529_d_b10: f64 = (p.p250 * s.db[161][10]);
        let eq119_e1529_d_b11: f64 = (p.p250 * s.db[161][11]);
        let eq119_e1529_d_b12: f64 = (p.p250 * s.db[161][12]);
        let eq119_e1529_d_b13: f64 = (p.p250 * s.db[161][13]);
        let eq119_e1529_d_b14: f64 = (p.p250 * s.db[161][14]);
        let eq119_e1529_d_b15: f64 = (p.p250 * s.db[161][15]);
        let eq119_e1529_d_b16: f64 = (p.p250 * s.db[161][16]);
        let eq119_e1529_d_b17: f64 = (p.p250 * s.db[161][17]);
        let eq119_e1529_d_b18: f64 = (p.p250 * s.db[161][18]);
        let eq119_e1529_d_b19: f64 = (p.p250 * s.db[161][19]);
        let eq119_e1529_d_b20: f64 = (p.p250 * s.db[161][20]);
        let eq119_e1529_d_b21: f64 = (p.p250 * s.db[161][21]);
        let eq119_e1529_d_b22: f64 = (p.p250 * s.db[161][22]);
        let eq119_e1529_d_b23: f64 = (p.p250 * s.db[161][23]);
        let eq119_e1529_d_b24: f64 = (p.p250 * s.db[161][24]);
        let eq119_e1529_d_b25: f64 = (p.p250 * s.db[161][25]);
        let eq119_e1529_d_b26: f64 = (p.p250 * s.db[161][26]);
        let eq119_e1529_d_b27: f64 = (p.p250 * s.db[161][27]);
        let eq119_e1529_d_b28: f64 = (p.p250 * s.db[161][28]);
        let eq119_e1529_d_b29: f64 = (p.p250 * s.db[161][29]);
        let eq119_e1529_d_b30: f64 = (p.p250 * s.db[161][30]);
        let eq119_e1529_d_b31: f64 = (p.p250 * s.db[161][31]);
        let eq119_e1529_d_b32: f64 = (p.p250 * s.db[161][32]);
        let eq119_e1529_d_b33: f64 = (p.p250 * s.db[161][33]);
        let eq119_e1529_d_b34: f64 = (p.p250 * s.db[161][34]);
        let eq119_e1529_d_b35: f64 = (p.p250 * s.db[161][35]);
        let eq119_e1529_d_b36: f64 = (p.p250 * s.db[161][36]);
        let eq119_e1529_d_b37: f64 = (p.p250 * s.db[161][37]);
        let eq119_e1529_d_b38: f64 = (p.p250 * s.db[161][38]);
        let eq119_e1529_d_b39: f64 = (p.p250 * s.db[161][39]);
        let eq119_e1529_d_b40: f64 = (p.p250 * s.db[161][40]);
        let eq119_e1529_d_b41: f64 = (p.p250 * s.db[161][41]);
        let eq119_e1529_d_b42: f64 = (p.p250 * s.db[161][42]);
        let eq119_e1529_d_b43: f64 = (p.p250 * s.db[161][43]);
        let eq119_e1529_d_b44: f64 = (p.p250 * s.db[161][44]);
        let eq119_e1529_d_b45: f64 = (p.p250 * s.db[161][45]);
        let eq119_e1529_d_b46: f64 = (p.p250 * s.db[161][46]);
        let eq119_e1529_d_b47: f64 = (p.p250 * s.db[161][47]);
        let eq119_e1529_d_b48: f64 = (p.p250 * s.db[161][48]);
        let eq119_e1529_d_b49: f64 = (p.p250 * s.db[161][49]);
        let eq119_e1529_d_b50: f64 = (p.p250 * s.db[161][50]);
        let eq119_e1529_d_b51: f64 = (p.p250 * s.db[161][51]);
        let eq119_e1529_d_b52: f64 = (p.p250 * s.db[161][52]);
        let eq119_e1529_d_b53: f64 = (p.p250 * s.db[161][53]);
        let eq119_e1529_d_b54: f64 = (p.p250 * s.db[161][54]);
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
        let eq119_e1531_d_b0: f64 = (p.p7 * eq119_e1529_d_b0);
        let eq119_e1531_d_b1: f64 = (p.p7 * eq119_e1529_d_b1);
        let eq119_e1531_d_b2: f64 = (p.p7 * eq119_e1529_d_b2);
        let eq119_e1531_d_b3: f64 = (p.p7 * eq119_e1529_d_b3);
        let eq119_e1531_d_b4: f64 = (p.p7 * eq119_e1529_d_b4);
        let eq119_e1531_d_b5: f64 = (p.p7 * eq119_e1529_d_b5);
        let eq119_e1531_d_b6: f64 = (p.p7 * eq119_e1529_d_b6);
        let eq119_e1531_d_b7: f64 = (p.p7 * eq119_e1529_d_b7);
        let eq119_e1531_d_b8: f64 = (p.p7 * eq119_e1529_d_b8);
        let eq119_e1531_d_b9: f64 = (p.p7 * eq119_e1529_d_b9);
        let eq119_e1531_d_b10: f64 = (p.p7 * eq119_e1529_d_b10);
        let eq119_e1531_d_b11: f64 = (p.p7 * eq119_e1529_d_b11);
        let eq119_e1531_d_b12: f64 = (p.p7 * eq119_e1529_d_b12);
        let eq119_e1531_d_b13: f64 = (p.p7 * eq119_e1529_d_b13);
        let eq119_e1531_d_b14: f64 = (p.p7 * eq119_e1529_d_b14);
        let eq119_e1531_d_b15: f64 = (p.p7 * eq119_e1529_d_b15);
        let eq119_e1531_d_b16: f64 = (p.p7 * eq119_e1529_d_b16);
        let eq119_e1531_d_b17: f64 = (p.p7 * eq119_e1529_d_b17);
        let eq119_e1531_d_b18: f64 = (p.p7 * eq119_e1529_d_b18);
        let eq119_e1531_d_b19: f64 = (p.p7 * eq119_e1529_d_b19);
        let eq119_e1531_d_b20: f64 = (p.p7 * eq119_e1529_d_b20);
        let eq119_e1531_d_b21: f64 = (p.p7 * eq119_e1529_d_b21);
        let eq119_e1531_d_b22: f64 = (p.p7 * eq119_e1529_d_b22);
        let eq119_e1531_d_b23: f64 = (p.p7 * eq119_e1529_d_b23);
        let eq119_e1531_d_b24: f64 = (p.p7 * eq119_e1529_d_b24);
        let eq119_e1531_d_b25: f64 = (p.p7 * eq119_e1529_d_b25);
        let eq119_e1531_d_b26: f64 = (p.p7 * eq119_e1529_d_b26);
        let eq119_e1531_d_b27: f64 = (p.p7 * eq119_e1529_d_b27);
        let eq119_e1531_d_b28: f64 = (p.p7 * eq119_e1529_d_b28);
        let eq119_e1531_d_b29: f64 = (p.p7 * eq119_e1529_d_b29);
        let eq119_e1531_d_b30: f64 = (p.p7 * eq119_e1529_d_b30);
        let eq119_e1531_d_b31: f64 = (p.p7 * eq119_e1529_d_b31);
        let eq119_e1531_d_b32: f64 = (p.p7 * eq119_e1529_d_b32);
        let eq119_e1531_d_b33: f64 = (p.p7 * eq119_e1529_d_b33);
        let eq119_e1531_d_b34: f64 = (p.p7 * eq119_e1529_d_b34);
        let eq119_e1531_d_b35: f64 = (p.p7 * eq119_e1529_d_b35);
        let eq119_e1531_d_b36: f64 = (p.p7 * eq119_e1529_d_b36);
        let eq119_e1531_d_b37: f64 = (p.p7 * eq119_e1529_d_b37);
        let eq119_e1531_d_b38: f64 = (p.p7 * eq119_e1529_d_b38);
        let eq119_e1531_d_b39: f64 = (p.p7 * eq119_e1529_d_b39);
        let eq119_e1531_d_b40: f64 = (p.p7 * eq119_e1529_d_b40);
        let eq119_e1531_d_b41: f64 = (p.p7 * eq119_e1529_d_b41);
        let eq119_e1531_d_b42: f64 = (p.p7 * eq119_e1529_d_b42);
        let eq119_e1531_d_b43: f64 = (p.p7 * eq119_e1529_d_b43);
        let eq119_e1531_d_b44: f64 = (p.p7 * eq119_e1529_d_b44);
        let eq119_e1531_d_b45: f64 = (p.p7 * eq119_e1529_d_b45);
        let eq119_e1531_d_b46: f64 = (p.p7 * eq119_e1529_d_b46);
        let eq119_e1531_d_b47: f64 = (p.p7 * eq119_e1529_d_b47);
        let eq119_e1531_d_b48: f64 = (p.p7 * eq119_e1529_d_b48);
        let eq119_e1531_d_b49: f64 = (p.p7 * eq119_e1529_d_b49);
        let eq119_e1531_d_b50: f64 = (p.p7 * eq119_e1529_d_b50);
        let eq119_e1531_d_b51: f64 = (p.p7 * eq119_e1529_d_b51);
        let eq119_e1531_d_b52: f64 = (p.p7 * eq119_e1529_d_b52);
        let eq119_e1531_d_b53: f64 = (p.p7 * eq119_e1529_d_b53);
        let eq119_e1531_d_b54: f64 = (p.p7 * eq119_e1529_d_b54);
        let eq119_e1531_q: f64 = (p.p7 * eq119_e1530_q);
        let eq119_reactive_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n10, eq119_e1531_d_n11, eq119_e1531_d_n12, eq119_e1531_d_n13, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_reactive_branch_derivatives: [f64; 55] = [eq119_e1531_d_b0, eq119_e1531_d_b1, eq119_e1531_d_b2, eq119_e1531_d_b3, eq119_e1531_d_b4, eq119_e1531_d_b5, eq119_e1531_d_b6, eq119_e1531_d_b7, eq119_e1531_d_b8, eq119_e1531_d_b9, eq119_e1531_d_b10, eq119_e1531_d_b11, eq119_e1531_d_b12, eq119_e1531_d_b13, eq119_e1531_d_b14, eq119_e1531_d_b15, eq119_e1531_d_b16, eq119_e1531_d_b17, eq119_e1531_d_b18, eq119_e1531_d_b19, eq119_e1531_d_b20, eq119_e1531_d_b21, eq119_e1531_d_b22, eq119_e1531_d_b23, eq119_e1531_d_b24, eq119_e1531_d_b25, eq119_e1531_d_b26, eq119_e1531_d_b27, eq119_e1531_d_b28, eq119_e1531_d_b29, eq119_e1531_d_b30, eq119_e1531_d_b31, eq119_e1531_d_b32, eq119_e1531_d_b33, eq119_e1531_d_b34, eq119_e1531_d_b35, eq119_e1531_d_b36, eq119_e1531_d_b37, eq119_e1531_d_b38, eq119_e1531_d_b39, eq119_e1531_d_b40, eq119_e1531_d_b41, eq119_e1531_d_b42, eq119_e1531_d_b43, eq119_e1531_d_b44, eq119_e1531_d_b45, eq119_e1531_d_b46, eq119_e1531_d_b47, eq119_e1531_d_b48, eq119_e1531_d_b49, eq119_e1531_d_b50, eq119_e1531_d_b51, eq119_e1531_d_b52, eq119_e1531_d_b53, eq119_e1531_d_b54];
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

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[228][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[228][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[228][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[228][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[228][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[228][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[228][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[228][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[228][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[228][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[228][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[228][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[228][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[228][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[228][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[228][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[228][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[228][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[228][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[228][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[228][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[228][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[228][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[228][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[228][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[228][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[228][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[228][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[228][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[228][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[228][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[228][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[228][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[228][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[228][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[228][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[228][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[228][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[228][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[228][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[228][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[228][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[228][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[228][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[228][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[228][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[228][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[228][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[228][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[228][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[228][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[228][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[228][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[228][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[228][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[228][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[228][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[228][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[228][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[228][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[228][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[228][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[228][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[228][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[228][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[228][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[228][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[228][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[228][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[228][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[228][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[228][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[228][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[228][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[228][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[228][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[228][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[228][54]);
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22, eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54, eq120_e1540_q,) = {
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
        let eq120_e1538_d_b0: f64 = (p.p7 * s.db[229][0]);
        let eq120_e1538_d_b1: f64 = (p.p7 * s.db[229][1]);
        let eq120_e1538_d_b2: f64 = (p.p7 * s.db[229][2]);
        let eq120_e1538_d_b3: f64 = (p.p7 * s.db[229][3]);
        let eq120_e1538_d_b4: f64 = (p.p7 * s.db[229][4]);
        let eq120_e1538_d_b5: f64 = (p.p7 * s.db[229][5]);
        let eq120_e1538_d_b6: f64 = (p.p7 * s.db[229][6]);
        let eq120_e1538_d_b7: f64 = (p.p7 * s.db[229][7]);
        let eq120_e1538_d_b8: f64 = (p.p7 * s.db[229][8]);
        let eq120_e1538_d_b9: f64 = (p.p7 * s.db[229][9]);
        let eq120_e1538_d_b10: f64 = (p.p7 * s.db[229][10]);
        let eq120_e1538_d_b11: f64 = (p.p7 * s.db[229][11]);
        let eq120_e1538_d_b12: f64 = (p.p7 * s.db[229][12]);
        let eq120_e1538_d_b13: f64 = (p.p7 * s.db[229][13]);
        let eq120_e1538_d_b14: f64 = (p.p7 * s.db[229][14]);
        let eq120_e1538_d_b15: f64 = (p.p7 * s.db[229][15]);
        let eq120_e1538_d_b16: f64 = (p.p7 * s.db[229][16]);
        let eq120_e1538_d_b17: f64 = (p.p7 * s.db[229][17]);
        let eq120_e1538_d_b18: f64 = (p.p7 * s.db[229][18]);
        let eq120_e1538_d_b19: f64 = (p.p7 * s.db[229][19]);
        let eq120_e1538_d_b20: f64 = (p.p7 * s.db[229][20]);
        let eq120_e1538_d_b21: f64 = (p.p7 * s.db[229][21]);
        let eq120_e1538_d_b22: f64 = (p.p7 * s.db[229][22]);
        let eq120_e1538_d_b23: f64 = (p.p7 * s.db[229][23]);
        let eq120_e1538_d_b24: f64 = (p.p7 * s.db[229][24]);
        let eq120_e1538_d_b25: f64 = (p.p7 * s.db[229][25]);
        let eq120_e1538_d_b26: f64 = (p.p7 * s.db[229][26]);
        let eq120_e1538_d_b27: f64 = (p.p7 * s.db[229][27]);
        let eq120_e1538_d_b28: f64 = (p.p7 * s.db[229][28]);
        let eq120_e1538_d_b29: f64 = (p.p7 * s.db[229][29]);
        let eq120_e1538_d_b30: f64 = (p.p7 * s.db[229][30]);
        let eq120_e1538_d_b31: f64 = (p.p7 * s.db[229][31]);
        let eq120_e1538_d_b32: f64 = (p.p7 * s.db[229][32]);
        let eq120_e1538_d_b33: f64 = (p.p7 * s.db[229][33]);
        let eq120_e1538_d_b34: f64 = (p.p7 * s.db[229][34]);
        let eq120_e1538_d_b35: f64 = (p.p7 * s.db[229][35]);
        let eq120_e1538_d_b36: f64 = (p.p7 * s.db[229][36]);
        let eq120_e1538_d_b37: f64 = (p.p7 * s.db[229][37]);
        let eq120_e1538_d_b38: f64 = (p.p7 * s.db[229][38]);
        let eq120_e1538_d_b39: f64 = (p.p7 * s.db[229][39]);
        let eq120_e1538_d_b40: f64 = (p.p7 * s.db[229][40]);
        let eq120_e1538_d_b41: f64 = (p.p7 * s.db[229][41]);
        let eq120_e1538_d_b42: f64 = (p.p7 * s.db[229][42]);
        let eq120_e1538_d_b43: f64 = (p.p7 * s.db[229][43]);
        let eq120_e1538_d_b44: f64 = (p.p7 * s.db[229][44]);
        let eq120_e1538_d_b45: f64 = (p.p7 * s.db[229][45]);
        let eq120_e1538_d_b46: f64 = (p.p7 * s.db[229][46]);
        let eq120_e1538_d_b47: f64 = (p.p7 * s.db[229][47]);
        let eq120_e1538_d_b48: f64 = (p.p7 * s.db[229][48]);
        let eq120_e1538_d_b49: f64 = (p.p7 * s.db[229][49]);
        let eq120_e1538_d_b50: f64 = (p.p7 * s.db[229][50]);
        let eq120_e1538_d_b51: f64 = (p.p7 * s.db[229][51]);
        let eq120_e1538_d_b52: f64 = (p.p7 * s.db[229][52]);
        let eq120_e1538_d_b53: f64 = (p.p7 * s.db[229][53]);
        let eq120_e1538_d_b54: f64 = (p.p7 * s.db[229][54]);
        let eq120_e1538_q: f64 = (p.p7 * eq120_e1537_q);
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n10, eq120_e1538_d_n11, eq120_e1538_d_n12, eq120_e1538_d_n13, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22, eq120_e1538_d_b0, eq120_e1538_d_b1, eq120_e1538_d_b2, eq120_e1538_d_b3, eq120_e1538_d_b4, eq120_e1538_d_b5, eq120_e1538_d_b6, eq120_e1538_d_b7, eq120_e1538_d_b8, eq120_e1538_d_b9, eq120_e1538_d_b10, eq120_e1538_d_b11, eq120_e1538_d_b12, eq120_e1538_d_b13, eq120_e1538_d_b14, eq120_e1538_d_b15, eq120_e1538_d_b16, eq120_e1538_d_b17, eq120_e1538_d_b18, eq120_e1538_d_b19, eq120_e1538_d_b20, eq120_e1538_d_b21, eq120_e1538_d_b22, eq120_e1538_d_b23, eq120_e1538_d_b24, eq120_e1538_d_b25, eq120_e1538_d_b26, eq120_e1538_d_b27, eq120_e1538_d_b28, eq120_e1538_d_b29, eq120_e1538_d_b30, eq120_e1538_d_b31, eq120_e1538_d_b32, eq120_e1538_d_b33, eq120_e1538_d_b34, eq120_e1538_d_b35, eq120_e1538_d_b36, eq120_e1538_d_b37, eq120_e1538_d_b38, eq120_e1538_d_b39, eq120_e1538_d_b40, eq120_e1538_d_b41, eq120_e1538_d_b42, eq120_e1538_d_b43, eq120_e1538_d_b44, eq120_e1538_d_b45, eq120_e1538_d_b46, eq120_e1538_d_b47, eq120_e1538_d_b48, eq120_e1538_d_b49, eq120_e1538_d_b50, eq120_e1538_d_b51, eq120_e1538_d_b52, eq120_e1538_d_b53, eq120_e1538_d_b54, eq120_e1538_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_reactive_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_reactive_branch_derivatives: [f64; 55] = [eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            nodes,
            &eq120_reactive_node_derivatives,
            branches,
            &eq120_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22, eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54, eq121_e1551_q,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq121_e1548_q: f64 = s.v[228];
        let eq121_e1549: f64 = (p.p7 * s.v[228]);
        let eq121_e1549_q: f64 = (p.p7 * eq121_e1548_q);
        (eq121_e1549, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq121_e1549_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_reactive_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_reactive_branch_derivatives: [f64; 55] = [eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq121_reactive_node_derivatives,
            branches,
            &eq121_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22, eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54, eq122_e1564_q,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq122_e1559_q: f64 = s.v[228];
        let eq122_e1560: f64 = (p.p7 * s.v[228]);
        let eq122_e1560_q: f64 = (p.p7 * eq122_e1559_q);
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq122_e1562_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq122_e1562_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq122_e1562_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq122_e1562_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq122_e1562_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq122_e1562_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq122_e1562_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq122_e1562_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq122_e1562_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq122_e1562_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq122_e1562_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq122_e1562_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq122_e1562_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq122_e1562_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq122_e1562_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq122_e1562_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq122_e1562_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq122_e1562_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq122_e1562_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq122_e1562_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq122_e1562_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq122_e1562_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq122_e1562_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq122_e1562_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq122_e1562_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq122_e1562_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq122_e1562_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq122_e1562_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq122_e1562_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq122_e1562_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq122_e1562_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq122_e1562_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq122_e1562_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq122_e1562_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq122_e1562_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq122_e1562_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq122_e1562_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq122_e1562_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq122_e1562_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq122_e1562_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq122_e1562_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq122_e1562_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq122_e1562_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq122_e1562_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq122_e1562_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq122_e1562_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq122_e1562_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq122_e1562_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq122_e1562_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq122_e1562_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq122_e1562_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq122_e1562_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq122_e1562_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq122_e1562_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq122_e1562_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq122_e1562_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq122_e1562_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq122_e1562_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq122_e1562_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq122_e1562_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq122_e1562_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq122_e1562_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq122_e1562_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq122_e1562_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq122_e1562_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq122_e1562_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq122_e1562_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq122_e1562_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq122_e1562_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq122_e1562_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq122_e1562_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq122_e1562_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq122_e1562_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq122_e1562_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq122_e1562_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq122_e1562_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq122_e1562_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        let eq122_e1562_q: f64 = (eq122_e1560_q * p.p246);
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n10, eq122_e1562_d_n11, eq122_e1562_d_n12, eq122_e1562_d_n13, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22, eq122_e1562_d_b0, eq122_e1562_d_b1, eq122_e1562_d_b2, eq122_e1562_d_b3, eq122_e1562_d_b4, eq122_e1562_d_b5, eq122_e1562_d_b6, eq122_e1562_d_b7, eq122_e1562_d_b8, eq122_e1562_d_b9, eq122_e1562_d_b10, eq122_e1562_d_b11, eq122_e1562_d_b12, eq122_e1562_d_b13, eq122_e1562_d_b14, eq122_e1562_d_b15, eq122_e1562_d_b16, eq122_e1562_d_b17, eq122_e1562_d_b18, eq122_e1562_d_b19, eq122_e1562_d_b20, eq122_e1562_d_b21, eq122_e1562_d_b22, eq122_e1562_d_b23, eq122_e1562_d_b24, eq122_e1562_d_b25, eq122_e1562_d_b26, eq122_e1562_d_b27, eq122_e1562_d_b28, eq122_e1562_d_b29, eq122_e1562_d_b30, eq122_e1562_d_b31, eq122_e1562_d_b32, eq122_e1562_d_b33, eq122_e1562_d_b34, eq122_e1562_d_b35, eq122_e1562_d_b36, eq122_e1562_d_b37, eq122_e1562_d_b38, eq122_e1562_d_b39, eq122_e1562_d_b40, eq122_e1562_d_b41, eq122_e1562_d_b42, eq122_e1562_d_b43, eq122_e1562_d_b44, eq122_e1562_d_b45, eq122_e1562_d_b46, eq122_e1562_d_b47, eq122_e1562_d_b48, eq122_e1562_d_b49, eq122_e1562_d_b50, eq122_e1562_d_b51, eq122_e1562_d_b52, eq122_e1562_d_b53, eq122_e1562_d_b54, eq122_e1562_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_reactive_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_reactive_branch_derivatives: [f64; 55] = [eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq122_reactive_node_derivatives,
            branches,
            &eq122_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54, eq123_e1576_q,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq123_e1573_q: f64 = s.v[228];
        let eq123_e1574: f64 = (p.p7 * s.v[228]);
        let eq123_e1574_q: f64 = (p.p7 * eq123_e1573_q);
        (eq123_e1574, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq123_e1574_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_reactive_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_reactive_branch_derivatives: [f64; 55] = [eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq123_reactive_node_derivatives,
            branches,
            &eq123_reactive_branch_derivatives,
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
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54, eq124_e1590_q,) = {
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
        let eq124_e1586_d_b0: f64 = (p.p7 * s.db[228][0]);
        let eq124_e1586_d_b1: f64 = (p.p7 * s.db[228][1]);
        let eq124_e1586_d_b2: f64 = (p.p7 * s.db[228][2]);
        let eq124_e1586_d_b3: f64 = (p.p7 * s.db[228][3]);
        let eq124_e1586_d_b4: f64 = (p.p7 * s.db[228][4]);
        let eq124_e1586_d_b5: f64 = (p.p7 * s.db[228][5]);
        let eq124_e1586_d_b6: f64 = (p.p7 * s.db[228][6]);
        let eq124_e1586_d_b7: f64 = (p.p7 * s.db[228][7]);
        let eq124_e1586_d_b8: f64 = (p.p7 * s.db[228][8]);
        let eq124_e1586_d_b9: f64 = (p.p7 * s.db[228][9]);
        let eq124_e1586_d_b10: f64 = (p.p7 * s.db[228][10]);
        let eq124_e1586_d_b11: f64 = (p.p7 * s.db[228][11]);
        let eq124_e1586_d_b12: f64 = (p.p7 * s.db[228][12]);
        let eq124_e1586_d_b13: f64 = (p.p7 * s.db[228][13]);
        let eq124_e1586_d_b14: f64 = (p.p7 * s.db[228][14]);
        let eq124_e1586_d_b15: f64 = (p.p7 * s.db[228][15]);
        let eq124_e1586_d_b16: f64 = (p.p7 * s.db[228][16]);
        let eq124_e1586_d_b17: f64 = (p.p7 * s.db[228][17]);
        let eq124_e1586_d_b18: f64 = (p.p7 * s.db[228][18]);
        let eq124_e1586_d_b19: f64 = (p.p7 * s.db[228][19]);
        let eq124_e1586_d_b20: f64 = (p.p7 * s.db[228][20]);
        let eq124_e1586_d_b21: f64 = (p.p7 * s.db[228][21]);
        let eq124_e1586_d_b22: f64 = (p.p7 * s.db[228][22]);
        let eq124_e1586_d_b23: f64 = (p.p7 * s.db[228][23]);
        let eq124_e1586_d_b24: f64 = (p.p7 * s.db[228][24]);
        let eq124_e1586_d_b25: f64 = (p.p7 * s.db[228][25]);
        let eq124_e1586_d_b26: f64 = (p.p7 * s.db[228][26]);
        let eq124_e1586_d_b27: f64 = (p.p7 * s.db[228][27]);
        let eq124_e1586_d_b28: f64 = (p.p7 * s.db[228][28]);
        let eq124_e1586_d_b29: f64 = (p.p7 * s.db[228][29]);
        let eq124_e1586_d_b30: f64 = (p.p7 * s.db[228][30]);
        let eq124_e1586_d_b31: f64 = (p.p7 * s.db[228][31]);
        let eq124_e1586_d_b32: f64 = (p.p7 * s.db[228][32]);
        let eq124_e1586_d_b33: f64 = (p.p7 * s.db[228][33]);
        let eq124_e1586_d_b34: f64 = (p.p7 * s.db[228][34]);
        let eq124_e1586_d_b35: f64 = (p.p7 * s.db[228][35]);
        let eq124_e1586_d_b36: f64 = (p.p7 * s.db[228][36]);
        let eq124_e1586_d_b37: f64 = (p.p7 * s.db[228][37]);
        let eq124_e1586_d_b38: f64 = (p.p7 * s.db[228][38]);
        let eq124_e1586_d_b39: f64 = (p.p7 * s.db[228][39]);
        let eq124_e1586_d_b40: f64 = (p.p7 * s.db[228][40]);
        let eq124_e1586_d_b41: f64 = (p.p7 * s.db[228][41]);
        let eq124_e1586_d_b42: f64 = (p.p7 * s.db[228][42]);
        let eq124_e1586_d_b43: f64 = (p.p7 * s.db[228][43]);
        let eq124_e1586_d_b44: f64 = (p.p7 * s.db[228][44]);
        let eq124_e1586_d_b45: f64 = (p.p7 * s.db[228][45]);
        let eq124_e1586_d_b46: f64 = (p.p7 * s.db[228][46]);
        let eq124_e1586_d_b47: f64 = (p.p7 * s.db[228][47]);
        let eq124_e1586_d_b48: f64 = (p.p7 * s.db[228][48]);
        let eq124_e1586_d_b49: f64 = (p.p7 * s.db[228][49]);
        let eq124_e1586_d_b50: f64 = (p.p7 * s.db[228][50]);
        let eq124_e1586_d_b51: f64 = (p.p7 * s.db[228][51]);
        let eq124_e1586_d_b52: f64 = (p.p7 * s.db[228][52]);
        let eq124_e1586_d_b53: f64 = (p.p7 * s.db[228][53]);
        let eq124_e1586_d_b54: f64 = (p.p7 * s.db[228][54]);
        let eq124_e1586_q: f64 = (p.p7 * eq124_e1585_q);
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
        let eq124_e1588_d_b0: f64 = (eq124_e1586_d_b0 * p.p246);
        let eq124_e1588_d_b1: f64 = (eq124_e1586_d_b1 * p.p246);
        let eq124_e1588_d_b2: f64 = (eq124_e1586_d_b2 * p.p246);
        let eq124_e1588_d_b3: f64 = (eq124_e1586_d_b3 * p.p246);
        let eq124_e1588_d_b4: f64 = (eq124_e1586_d_b4 * p.p246);
        let eq124_e1588_d_b5: f64 = (eq124_e1586_d_b5 * p.p246);
        let eq124_e1588_d_b6: f64 = (eq124_e1586_d_b6 * p.p246);
        let eq124_e1588_d_b7: f64 = (eq124_e1586_d_b7 * p.p246);
        let eq124_e1588_d_b8: f64 = (eq124_e1586_d_b8 * p.p246);
        let eq124_e1588_d_b9: f64 = (eq124_e1586_d_b9 * p.p246);
        let eq124_e1588_d_b10: f64 = (eq124_e1586_d_b10 * p.p246);
        let eq124_e1588_d_b11: f64 = (eq124_e1586_d_b11 * p.p246);
        let eq124_e1588_d_b12: f64 = (eq124_e1586_d_b12 * p.p246);
        let eq124_e1588_d_b13: f64 = (eq124_e1586_d_b13 * p.p246);
        let eq124_e1588_d_b14: f64 = (eq124_e1586_d_b14 * p.p246);
        let eq124_e1588_d_b15: f64 = (eq124_e1586_d_b15 * p.p246);
        let eq124_e1588_d_b16: f64 = (eq124_e1586_d_b16 * p.p246);
        let eq124_e1588_d_b17: f64 = (eq124_e1586_d_b17 * p.p246);
        let eq124_e1588_d_b18: f64 = (eq124_e1586_d_b18 * p.p246);
        let eq124_e1588_d_b19: f64 = (eq124_e1586_d_b19 * p.p246);
        let eq124_e1588_d_b20: f64 = (eq124_e1586_d_b20 * p.p246);
        let eq124_e1588_d_b21: f64 = (eq124_e1586_d_b21 * p.p246);
        let eq124_e1588_d_b22: f64 = (eq124_e1586_d_b22 * p.p246);
        let eq124_e1588_d_b23: f64 = (eq124_e1586_d_b23 * p.p246);
        let eq124_e1588_d_b24: f64 = (eq124_e1586_d_b24 * p.p246);
        let eq124_e1588_d_b25: f64 = (eq124_e1586_d_b25 * p.p246);
        let eq124_e1588_d_b26: f64 = (eq124_e1586_d_b26 * p.p246);
        let eq124_e1588_d_b27: f64 = (eq124_e1586_d_b27 * p.p246);
        let eq124_e1588_d_b28: f64 = (eq124_e1586_d_b28 * p.p246);
        let eq124_e1588_d_b29: f64 = (eq124_e1586_d_b29 * p.p246);
        let eq124_e1588_d_b30: f64 = (eq124_e1586_d_b30 * p.p246);
        let eq124_e1588_d_b31: f64 = (eq124_e1586_d_b31 * p.p246);
        let eq124_e1588_d_b32: f64 = (eq124_e1586_d_b32 * p.p246);
        let eq124_e1588_d_b33: f64 = (eq124_e1586_d_b33 * p.p246);
        let eq124_e1588_d_b34: f64 = (eq124_e1586_d_b34 * p.p246);
        let eq124_e1588_d_b35: f64 = (eq124_e1586_d_b35 * p.p246);
        let eq124_e1588_d_b36: f64 = (eq124_e1586_d_b36 * p.p246);
        let eq124_e1588_d_b37: f64 = (eq124_e1586_d_b37 * p.p246);
        let eq124_e1588_d_b38: f64 = (eq124_e1586_d_b38 * p.p246);
        let eq124_e1588_d_b39: f64 = (eq124_e1586_d_b39 * p.p246);
        let eq124_e1588_d_b40: f64 = (eq124_e1586_d_b40 * p.p246);
        let eq124_e1588_d_b41: f64 = (eq124_e1586_d_b41 * p.p246);
        let eq124_e1588_d_b42: f64 = (eq124_e1586_d_b42 * p.p246);
        let eq124_e1588_d_b43: f64 = (eq124_e1586_d_b43 * p.p246);
        let eq124_e1588_d_b44: f64 = (eq124_e1586_d_b44 * p.p246);
        let eq124_e1588_d_b45: f64 = (eq124_e1586_d_b45 * p.p246);
        let eq124_e1588_d_b46: f64 = (eq124_e1586_d_b46 * p.p246);
        let eq124_e1588_d_b47: f64 = (eq124_e1586_d_b47 * p.p246);
        let eq124_e1588_d_b48: f64 = (eq124_e1586_d_b48 * p.p246);
        let eq124_e1588_d_b49: f64 = (eq124_e1586_d_b49 * p.p246);
        let eq124_e1588_d_b50: f64 = (eq124_e1586_d_b50 * p.p246);
        let eq124_e1588_d_b51: f64 = (eq124_e1586_d_b51 * p.p246);
        let eq124_e1588_d_b52: f64 = (eq124_e1586_d_b52 * p.p246);
        let eq124_e1588_d_b53: f64 = (eq124_e1586_d_b53 * p.p246);
        let eq124_e1588_d_b54: f64 = (eq124_e1586_d_b54 * p.p246);
        let eq124_e1588_q: f64 = (eq124_e1586_q * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22, eq124_e1588_d_b0, eq124_e1588_d_b1, eq124_e1588_d_b2, eq124_e1588_d_b3, eq124_e1588_d_b4, eq124_e1588_d_b5, eq124_e1588_d_b6, eq124_e1588_d_b7, eq124_e1588_d_b8, eq124_e1588_d_b9, eq124_e1588_d_b10, eq124_e1588_d_b11, eq124_e1588_d_b12, eq124_e1588_d_b13, eq124_e1588_d_b14, eq124_e1588_d_b15, eq124_e1588_d_b16, eq124_e1588_d_b17, eq124_e1588_d_b18, eq124_e1588_d_b19, eq124_e1588_d_b20, eq124_e1588_d_b21, eq124_e1588_d_b22, eq124_e1588_d_b23, eq124_e1588_d_b24, eq124_e1588_d_b25, eq124_e1588_d_b26, eq124_e1588_d_b27, eq124_e1588_d_b28, eq124_e1588_d_b29, eq124_e1588_d_b30, eq124_e1588_d_b31, eq124_e1588_d_b32, eq124_e1588_d_b33, eq124_e1588_d_b34, eq124_e1588_d_b35, eq124_e1588_d_b36, eq124_e1588_d_b37, eq124_e1588_d_b38, eq124_e1588_d_b39, eq124_e1588_d_b40, eq124_e1588_d_b41, eq124_e1588_d_b42, eq124_e1588_d_b43, eq124_e1588_d_b44, eq124_e1588_d_b45, eq124_e1588_d_b46, eq124_e1588_d_b47, eq124_e1588_d_b48, eq124_e1588_d_b49, eq124_e1588_d_b50, eq124_e1588_d_b51, eq124_e1588_d_b52, eq124_e1588_d_b53, eq124_e1588_d_b54, eq124_e1588_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_reactive_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_reactive_branch_derivatives: [f64; 55] = [eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq124_reactive_node_derivatives,
            branches,
            &eq124_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54, eq125_e1601_q,) = {
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
        let eq125_e1597_d_b0: f64 = (p.p251 * s.db[228][0]);
        let eq125_e1597_d_b1: f64 = (p.p251 * s.db[228][1]);
        let eq125_e1597_d_b2: f64 = (p.p251 * s.db[228][2]);
        let eq125_e1597_d_b3: f64 = (p.p251 * s.db[228][3]);
        let eq125_e1597_d_b4: f64 = (p.p251 * s.db[228][4]);
        let eq125_e1597_d_b5: f64 = (p.p251 * s.db[228][5]);
        let eq125_e1597_d_b6: f64 = (p.p251 * s.db[228][6]);
        let eq125_e1597_d_b7: f64 = (p.p251 * s.db[228][7]);
        let eq125_e1597_d_b8: f64 = (p.p251 * s.db[228][8]);
        let eq125_e1597_d_b9: f64 = (p.p251 * s.db[228][9]);
        let eq125_e1597_d_b10: f64 = (p.p251 * s.db[228][10]);
        let eq125_e1597_d_b11: f64 = (p.p251 * s.db[228][11]);
        let eq125_e1597_d_b12: f64 = (p.p251 * s.db[228][12]);
        let eq125_e1597_d_b13: f64 = (p.p251 * s.db[228][13]);
        let eq125_e1597_d_b14: f64 = (p.p251 * s.db[228][14]);
        let eq125_e1597_d_b15: f64 = (p.p251 * s.db[228][15]);
        let eq125_e1597_d_b16: f64 = (p.p251 * s.db[228][16]);
        let eq125_e1597_d_b17: f64 = (p.p251 * s.db[228][17]);
        let eq125_e1597_d_b18: f64 = (p.p251 * s.db[228][18]);
        let eq125_e1597_d_b19: f64 = (p.p251 * s.db[228][19]);
        let eq125_e1597_d_b20: f64 = (p.p251 * s.db[228][20]);
        let eq125_e1597_d_b21: f64 = (p.p251 * s.db[228][21]);
        let eq125_e1597_d_b22: f64 = (p.p251 * s.db[228][22]);
        let eq125_e1597_d_b23: f64 = (p.p251 * s.db[228][23]);
        let eq125_e1597_d_b24: f64 = (p.p251 * s.db[228][24]);
        let eq125_e1597_d_b25: f64 = (p.p251 * s.db[228][25]);
        let eq125_e1597_d_b26: f64 = (p.p251 * s.db[228][26]);
        let eq125_e1597_d_b27: f64 = (p.p251 * s.db[228][27]);
        let eq125_e1597_d_b28: f64 = (p.p251 * s.db[228][28]);
        let eq125_e1597_d_b29: f64 = (p.p251 * s.db[228][29]);
        let eq125_e1597_d_b30: f64 = (p.p251 * s.db[228][30]);
        let eq125_e1597_d_b31: f64 = (p.p251 * s.db[228][31]);
        let eq125_e1597_d_b32: f64 = (p.p251 * s.db[228][32]);
        let eq125_e1597_d_b33: f64 = (p.p251 * s.db[228][33]);
        let eq125_e1597_d_b34: f64 = (p.p251 * s.db[228][34]);
        let eq125_e1597_d_b35: f64 = (p.p251 * s.db[228][35]);
        let eq125_e1597_d_b36: f64 = (p.p251 * s.db[228][36]);
        let eq125_e1597_d_b37: f64 = (p.p251 * s.db[228][37]);
        let eq125_e1597_d_b38: f64 = (p.p251 * s.db[228][38]);
        let eq125_e1597_d_b39: f64 = (p.p251 * s.db[228][39]);
        let eq125_e1597_d_b40: f64 = (p.p251 * s.db[228][40]);
        let eq125_e1597_d_b41: f64 = (p.p251 * s.db[228][41]);
        let eq125_e1597_d_b42: f64 = (p.p251 * s.db[228][42]);
        let eq125_e1597_d_b43: f64 = (p.p251 * s.db[228][43]);
        let eq125_e1597_d_b44: f64 = (p.p251 * s.db[228][44]);
        let eq125_e1597_d_b45: f64 = (p.p251 * s.db[228][45]);
        let eq125_e1597_d_b46: f64 = (p.p251 * s.db[228][46]);
        let eq125_e1597_d_b47: f64 = (p.p251 * s.db[228][47]);
        let eq125_e1597_d_b48: f64 = (p.p251 * s.db[228][48]);
        let eq125_e1597_d_b49: f64 = (p.p251 * s.db[228][49]);
        let eq125_e1597_d_b50: f64 = (p.p251 * s.db[228][50]);
        let eq125_e1597_d_b51: f64 = (p.p251 * s.db[228][51]);
        let eq125_e1597_d_b52: f64 = (p.p251 * s.db[228][52]);
        let eq125_e1597_d_b53: f64 = (p.p251 * s.db[228][53]);
        let eq125_e1597_d_b54: f64 = (p.p251 * s.db[228][54]);
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
        let eq125_e1599_d_b0: f64 = (p.p7 * eq125_e1597_d_b0);
        let eq125_e1599_d_b1: f64 = (p.p7 * eq125_e1597_d_b1);
        let eq125_e1599_d_b2: f64 = (p.p7 * eq125_e1597_d_b2);
        let eq125_e1599_d_b3: f64 = (p.p7 * eq125_e1597_d_b3);
        let eq125_e1599_d_b4: f64 = (p.p7 * eq125_e1597_d_b4);
        let eq125_e1599_d_b5: f64 = (p.p7 * eq125_e1597_d_b5);
        let eq125_e1599_d_b6: f64 = (p.p7 * eq125_e1597_d_b6);
        let eq125_e1599_d_b7: f64 = (p.p7 * eq125_e1597_d_b7);
        let eq125_e1599_d_b8: f64 = (p.p7 * eq125_e1597_d_b8);
        let eq125_e1599_d_b9: f64 = (p.p7 * eq125_e1597_d_b9);
        let eq125_e1599_d_b10: f64 = (p.p7 * eq125_e1597_d_b10);
        let eq125_e1599_d_b11: f64 = (p.p7 * eq125_e1597_d_b11);
        let eq125_e1599_d_b12: f64 = (p.p7 * eq125_e1597_d_b12);
        let eq125_e1599_d_b13: f64 = (p.p7 * eq125_e1597_d_b13);
        let eq125_e1599_d_b14: f64 = (p.p7 * eq125_e1597_d_b14);
        let eq125_e1599_d_b15: f64 = (p.p7 * eq125_e1597_d_b15);
        let eq125_e1599_d_b16: f64 = (p.p7 * eq125_e1597_d_b16);
        let eq125_e1599_d_b17: f64 = (p.p7 * eq125_e1597_d_b17);
        let eq125_e1599_d_b18: f64 = (p.p7 * eq125_e1597_d_b18);
        let eq125_e1599_d_b19: f64 = (p.p7 * eq125_e1597_d_b19);
        let eq125_e1599_d_b20: f64 = (p.p7 * eq125_e1597_d_b20);
        let eq125_e1599_d_b21: f64 = (p.p7 * eq125_e1597_d_b21);
        let eq125_e1599_d_b22: f64 = (p.p7 * eq125_e1597_d_b22);
        let eq125_e1599_d_b23: f64 = (p.p7 * eq125_e1597_d_b23);
        let eq125_e1599_d_b24: f64 = (p.p7 * eq125_e1597_d_b24);
        let eq125_e1599_d_b25: f64 = (p.p7 * eq125_e1597_d_b25);
        let eq125_e1599_d_b26: f64 = (p.p7 * eq125_e1597_d_b26);
        let eq125_e1599_d_b27: f64 = (p.p7 * eq125_e1597_d_b27);
        let eq125_e1599_d_b28: f64 = (p.p7 * eq125_e1597_d_b28);
        let eq125_e1599_d_b29: f64 = (p.p7 * eq125_e1597_d_b29);
        let eq125_e1599_d_b30: f64 = (p.p7 * eq125_e1597_d_b30);
        let eq125_e1599_d_b31: f64 = (p.p7 * eq125_e1597_d_b31);
        let eq125_e1599_d_b32: f64 = (p.p7 * eq125_e1597_d_b32);
        let eq125_e1599_d_b33: f64 = (p.p7 * eq125_e1597_d_b33);
        let eq125_e1599_d_b34: f64 = (p.p7 * eq125_e1597_d_b34);
        let eq125_e1599_d_b35: f64 = (p.p7 * eq125_e1597_d_b35);
        let eq125_e1599_d_b36: f64 = (p.p7 * eq125_e1597_d_b36);
        let eq125_e1599_d_b37: f64 = (p.p7 * eq125_e1597_d_b37);
        let eq125_e1599_d_b38: f64 = (p.p7 * eq125_e1597_d_b38);
        let eq125_e1599_d_b39: f64 = (p.p7 * eq125_e1597_d_b39);
        let eq125_e1599_d_b40: f64 = (p.p7 * eq125_e1597_d_b40);
        let eq125_e1599_d_b41: f64 = (p.p7 * eq125_e1597_d_b41);
        let eq125_e1599_d_b42: f64 = (p.p7 * eq125_e1597_d_b42);
        let eq125_e1599_d_b43: f64 = (p.p7 * eq125_e1597_d_b43);
        let eq125_e1599_d_b44: f64 = (p.p7 * eq125_e1597_d_b44);
        let eq125_e1599_d_b45: f64 = (p.p7 * eq125_e1597_d_b45);
        let eq125_e1599_d_b46: f64 = (p.p7 * eq125_e1597_d_b46);
        let eq125_e1599_d_b47: f64 = (p.p7 * eq125_e1597_d_b47);
        let eq125_e1599_d_b48: f64 = (p.p7 * eq125_e1597_d_b48);
        let eq125_e1599_d_b49: f64 = (p.p7 * eq125_e1597_d_b49);
        let eq125_e1599_d_b50: f64 = (p.p7 * eq125_e1597_d_b50);
        let eq125_e1599_d_b51: f64 = (p.p7 * eq125_e1597_d_b51);
        let eq125_e1599_d_b52: f64 = (p.p7 * eq125_e1597_d_b52);
        let eq125_e1599_d_b53: f64 = (p.p7 * eq125_e1597_d_b53);
        let eq125_e1599_d_b54: f64 = (p.p7 * eq125_e1597_d_b54);
        let eq125_e1599_q: f64 = (p.p7 * eq125_e1598_q);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_d_b0, eq125_e1599_d_b1, eq125_e1599_d_b2, eq125_e1599_d_b3, eq125_e1599_d_b4, eq125_e1599_d_b5, eq125_e1599_d_b6, eq125_e1599_d_b7, eq125_e1599_d_b8, eq125_e1599_d_b9, eq125_e1599_d_b10, eq125_e1599_d_b11, eq125_e1599_d_b12, eq125_e1599_d_b13, eq125_e1599_d_b14, eq125_e1599_d_b15, eq125_e1599_d_b16, eq125_e1599_d_b17, eq125_e1599_d_b18, eq125_e1599_d_b19, eq125_e1599_d_b20, eq125_e1599_d_b21, eq125_e1599_d_b22, eq125_e1599_d_b23, eq125_e1599_d_b24, eq125_e1599_d_b25, eq125_e1599_d_b26, eq125_e1599_d_b27, eq125_e1599_d_b28, eq125_e1599_d_b29, eq125_e1599_d_b30, eq125_e1599_d_b31, eq125_e1599_d_b32, eq125_e1599_d_b33, eq125_e1599_d_b34, eq125_e1599_d_b35, eq125_e1599_d_b36, eq125_e1599_d_b37, eq125_e1599_d_b38, eq125_e1599_d_b39, eq125_e1599_d_b40, eq125_e1599_d_b41, eq125_e1599_d_b42, eq125_e1599_d_b43, eq125_e1599_d_b44, eq125_e1599_d_b45, eq125_e1599_d_b46, eq125_e1599_d_b47, eq125_e1599_d_b48, eq125_e1599_d_b49, eq125_e1599_d_b50, eq125_e1599_d_b51, eq125_e1599_d_b52, eq125_e1599_d_b53, eq125_e1599_d_b54, eq125_e1599_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_reactive_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_reactive_branch_derivatives: [f64; 55] = [eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq125_reactive_node_derivatives,
            branches,
            &eq125_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54, eq126_e1611_q,) = {
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
        let eq126_e1609_d_b0: f64 = (p.p7 * s.db[229][0]);
        let eq126_e1609_d_b1: f64 = (p.p7 * s.db[229][1]);
        let eq126_e1609_d_b2: f64 = (p.p7 * s.db[229][2]);
        let eq126_e1609_d_b3: f64 = (p.p7 * s.db[229][3]);
        let eq126_e1609_d_b4: f64 = (p.p7 * s.db[229][4]);
        let eq126_e1609_d_b5: f64 = (p.p7 * s.db[229][5]);
        let eq126_e1609_d_b6: f64 = (p.p7 * s.db[229][6]);
        let eq126_e1609_d_b7: f64 = (p.p7 * s.db[229][7]);
        let eq126_e1609_d_b8: f64 = (p.p7 * s.db[229][8]);
        let eq126_e1609_d_b9: f64 = (p.p7 * s.db[229][9]);
        let eq126_e1609_d_b10: f64 = (p.p7 * s.db[229][10]);
        let eq126_e1609_d_b11: f64 = (p.p7 * s.db[229][11]);
        let eq126_e1609_d_b12: f64 = (p.p7 * s.db[229][12]);
        let eq126_e1609_d_b13: f64 = (p.p7 * s.db[229][13]);
        let eq126_e1609_d_b14: f64 = (p.p7 * s.db[229][14]);
        let eq126_e1609_d_b15: f64 = (p.p7 * s.db[229][15]);
        let eq126_e1609_d_b16: f64 = (p.p7 * s.db[229][16]);
        let eq126_e1609_d_b17: f64 = (p.p7 * s.db[229][17]);
        let eq126_e1609_d_b18: f64 = (p.p7 * s.db[229][18]);
        let eq126_e1609_d_b19: f64 = (p.p7 * s.db[229][19]);
        let eq126_e1609_d_b20: f64 = (p.p7 * s.db[229][20]);
        let eq126_e1609_d_b21: f64 = (p.p7 * s.db[229][21]);
        let eq126_e1609_d_b22: f64 = (p.p7 * s.db[229][22]);
        let eq126_e1609_d_b23: f64 = (p.p7 * s.db[229][23]);
        let eq126_e1609_d_b24: f64 = (p.p7 * s.db[229][24]);
        let eq126_e1609_d_b25: f64 = (p.p7 * s.db[229][25]);
        let eq126_e1609_d_b26: f64 = (p.p7 * s.db[229][26]);
        let eq126_e1609_d_b27: f64 = (p.p7 * s.db[229][27]);
        let eq126_e1609_d_b28: f64 = (p.p7 * s.db[229][28]);
        let eq126_e1609_d_b29: f64 = (p.p7 * s.db[229][29]);
        let eq126_e1609_d_b30: f64 = (p.p7 * s.db[229][30]);
        let eq126_e1609_d_b31: f64 = (p.p7 * s.db[229][31]);
        let eq126_e1609_d_b32: f64 = (p.p7 * s.db[229][32]);
        let eq126_e1609_d_b33: f64 = (p.p7 * s.db[229][33]);
        let eq126_e1609_d_b34: f64 = (p.p7 * s.db[229][34]);
        let eq126_e1609_d_b35: f64 = (p.p7 * s.db[229][35]);
        let eq126_e1609_d_b36: f64 = (p.p7 * s.db[229][36]);
        let eq126_e1609_d_b37: f64 = (p.p7 * s.db[229][37]);
        let eq126_e1609_d_b38: f64 = (p.p7 * s.db[229][38]);
        let eq126_e1609_d_b39: f64 = (p.p7 * s.db[229][39]);
        let eq126_e1609_d_b40: f64 = (p.p7 * s.db[229][40]);
        let eq126_e1609_d_b41: f64 = (p.p7 * s.db[229][41]);
        let eq126_e1609_d_b42: f64 = (p.p7 * s.db[229][42]);
        let eq126_e1609_d_b43: f64 = (p.p7 * s.db[229][43]);
        let eq126_e1609_d_b44: f64 = (p.p7 * s.db[229][44]);
        let eq126_e1609_d_b45: f64 = (p.p7 * s.db[229][45]);
        let eq126_e1609_d_b46: f64 = (p.p7 * s.db[229][46]);
        let eq126_e1609_d_b47: f64 = (p.p7 * s.db[229][47]);
        let eq126_e1609_d_b48: f64 = (p.p7 * s.db[229][48]);
        let eq126_e1609_d_b49: f64 = (p.p7 * s.db[229][49]);
        let eq126_e1609_d_b50: f64 = (p.p7 * s.db[229][50]);
        let eq126_e1609_d_b51: f64 = (p.p7 * s.db[229][51]);
        let eq126_e1609_d_b52: f64 = (p.p7 * s.db[229][52]);
        let eq126_e1609_d_b53: f64 = (p.p7 * s.db[229][53]);
        let eq126_e1609_d_b54: f64 = (p.p7 * s.db[229][54]);
        let eq126_e1609_q: f64 = (p.p7 * eq126_e1608_q);
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22, eq126_e1609_d_b0, eq126_e1609_d_b1, eq126_e1609_d_b2, eq126_e1609_d_b3, eq126_e1609_d_b4, eq126_e1609_d_b5, eq126_e1609_d_b6, eq126_e1609_d_b7, eq126_e1609_d_b8, eq126_e1609_d_b9, eq126_e1609_d_b10, eq126_e1609_d_b11, eq126_e1609_d_b12, eq126_e1609_d_b13, eq126_e1609_d_b14, eq126_e1609_d_b15, eq126_e1609_d_b16, eq126_e1609_d_b17, eq126_e1609_d_b18, eq126_e1609_d_b19, eq126_e1609_d_b20, eq126_e1609_d_b21, eq126_e1609_d_b22, eq126_e1609_d_b23, eq126_e1609_d_b24, eq126_e1609_d_b25, eq126_e1609_d_b26, eq126_e1609_d_b27, eq126_e1609_d_b28, eq126_e1609_d_b29, eq126_e1609_d_b30, eq126_e1609_d_b31, eq126_e1609_d_b32, eq126_e1609_d_b33, eq126_e1609_d_b34, eq126_e1609_d_b35, eq126_e1609_d_b36, eq126_e1609_d_b37, eq126_e1609_d_b38, eq126_e1609_d_b39, eq126_e1609_d_b40, eq126_e1609_d_b41, eq126_e1609_d_b42, eq126_e1609_d_b43, eq126_e1609_d_b44, eq126_e1609_d_b45, eq126_e1609_d_b46, eq126_e1609_d_b47, eq126_e1609_d_b48, eq126_e1609_d_b49, eq126_e1609_d_b50, eq126_e1609_d_b51, eq126_e1609_d_b52, eq126_e1609_d_b53, eq126_e1609_d_b54, eq126_e1609_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_reactive_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_reactive_branch_derivatives: [f64; 55] = [eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq126_reactive_node_derivatives,
            branches,
            &eq126_reactive_branch_derivatives,
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[228][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[228][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[228][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[228][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[228][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[228][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[228][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[228][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[228][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[228][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[228][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[228][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[228][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[228][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[228][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[228][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[228][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[228][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[228][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[228][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[228][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[228][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[228][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[228][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[228][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[228][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[228][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[228][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[228][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[228][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[228][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[228][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[228][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[228][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[228][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[228][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[228][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[228][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[228][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[228][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[228][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[228][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[228][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[228][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[228][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[228][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[228][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[228][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[228][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[228][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[228][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[228][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[228][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[228][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[228][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[228][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[228][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[228][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[228][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[228][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[228][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[228][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[228][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[228][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[228][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[228][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[228][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[228][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[228][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[228][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[228][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[228][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[228][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[228][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[228][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[228][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[228][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[228][54]);
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54, eq127_e1623_q,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq127_e1620_q: f64 = s.v[228];
        let eq127_e1621: f64 = (p.p7 * s.v[228]);
        let eq127_e1621_q: f64 = (p.p7 * eq127_e1620_q);
        (eq127_e1621, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq127_e1621_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_reactive_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_reactive_branch_derivatives: [f64; 55] = [eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq127_reactive_node_derivatives,
            branches,
            &eq127_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54, eq128_e1637_q,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq128_e1632_q: f64 = s.v[228];
        let eq128_e1633: f64 = (p.p7 * s.v[228]);
        let eq128_e1633_q: f64 = (p.p7 * eq128_e1632_q);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq128_e1635_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq128_e1635_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq128_e1635_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq128_e1635_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq128_e1635_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq128_e1635_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq128_e1635_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq128_e1635_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq128_e1635_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq128_e1635_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq128_e1635_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq128_e1635_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq128_e1635_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq128_e1635_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq128_e1635_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq128_e1635_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq128_e1635_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq128_e1635_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq128_e1635_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq128_e1635_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq128_e1635_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq128_e1635_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq128_e1635_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq128_e1635_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq128_e1635_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq128_e1635_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq128_e1635_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq128_e1635_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq128_e1635_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq128_e1635_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq128_e1635_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq128_e1635_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq128_e1635_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq128_e1635_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq128_e1635_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq128_e1635_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq128_e1635_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq128_e1635_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq128_e1635_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq128_e1635_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq128_e1635_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq128_e1635_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq128_e1635_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq128_e1635_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq128_e1635_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq128_e1635_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq128_e1635_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq128_e1635_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq128_e1635_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq128_e1635_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq128_e1635_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq128_e1635_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq128_e1635_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq128_e1635_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq128_e1635_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq128_e1635_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq128_e1635_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq128_e1635_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq128_e1635_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq128_e1635_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq128_e1635_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq128_e1635_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq128_e1635_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq128_e1635_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq128_e1635_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq128_e1635_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq128_e1635_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq128_e1635_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq128_e1635_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq128_e1635_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq128_e1635_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq128_e1635_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq128_e1635_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq128_e1635_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq128_e1635_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq128_e1635_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq128_e1635_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        let eq128_e1635_q: f64 = (eq128_e1633_q * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22, eq128_e1635_d_b0, eq128_e1635_d_b1, eq128_e1635_d_b2, eq128_e1635_d_b3, eq128_e1635_d_b4, eq128_e1635_d_b5, eq128_e1635_d_b6, eq128_e1635_d_b7, eq128_e1635_d_b8, eq128_e1635_d_b9, eq128_e1635_d_b10, eq128_e1635_d_b11, eq128_e1635_d_b12, eq128_e1635_d_b13, eq128_e1635_d_b14, eq128_e1635_d_b15, eq128_e1635_d_b16, eq128_e1635_d_b17, eq128_e1635_d_b18, eq128_e1635_d_b19, eq128_e1635_d_b20, eq128_e1635_d_b21, eq128_e1635_d_b22, eq128_e1635_d_b23, eq128_e1635_d_b24, eq128_e1635_d_b25, eq128_e1635_d_b26, eq128_e1635_d_b27, eq128_e1635_d_b28, eq128_e1635_d_b29, eq128_e1635_d_b30, eq128_e1635_d_b31, eq128_e1635_d_b32, eq128_e1635_d_b33, eq128_e1635_d_b34, eq128_e1635_d_b35, eq128_e1635_d_b36, eq128_e1635_d_b37, eq128_e1635_d_b38, eq128_e1635_d_b39, eq128_e1635_d_b40, eq128_e1635_d_b41, eq128_e1635_d_b42, eq128_e1635_d_b43, eq128_e1635_d_b44, eq128_e1635_d_b45, eq128_e1635_d_b46, eq128_e1635_d_b47, eq128_e1635_d_b48, eq128_e1635_d_b49, eq128_e1635_d_b50, eq128_e1635_d_b51, eq128_e1635_d_b52, eq128_e1635_d_b53, eq128_e1635_d_b54, eq128_e1635_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_reactive_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_reactive_branch_derivatives: [f64; 55] = [eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq128_reactive_node_derivatives,
            branches,
            &eq128_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54, eq129_e1650_q,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq129_e1647_q: f64 = s.v[228];
        let eq129_e1648: f64 = (p.p7 * s.v[228]);
        let eq129_e1648_q: f64 = (p.p7 * eq129_e1647_q);
        (eq129_e1648, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq129_e1648_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_reactive_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_reactive_branch_derivatives: [f64; 55] = [eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54];
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

    pub(super) fn stamp_reactive_equations_block_6(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54, eq130_e1665_q,) = {
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
        let eq130_e1661_d_b0: f64 = (p.p7 * s.db[228][0]);
        let eq130_e1661_d_b1: f64 = (p.p7 * s.db[228][1]);
        let eq130_e1661_d_b2: f64 = (p.p7 * s.db[228][2]);
        let eq130_e1661_d_b3: f64 = (p.p7 * s.db[228][3]);
        let eq130_e1661_d_b4: f64 = (p.p7 * s.db[228][4]);
        let eq130_e1661_d_b5: f64 = (p.p7 * s.db[228][5]);
        let eq130_e1661_d_b6: f64 = (p.p7 * s.db[228][6]);
        let eq130_e1661_d_b7: f64 = (p.p7 * s.db[228][7]);
        let eq130_e1661_d_b8: f64 = (p.p7 * s.db[228][8]);
        let eq130_e1661_d_b9: f64 = (p.p7 * s.db[228][9]);
        let eq130_e1661_d_b10: f64 = (p.p7 * s.db[228][10]);
        let eq130_e1661_d_b11: f64 = (p.p7 * s.db[228][11]);
        let eq130_e1661_d_b12: f64 = (p.p7 * s.db[228][12]);
        let eq130_e1661_d_b13: f64 = (p.p7 * s.db[228][13]);
        let eq130_e1661_d_b14: f64 = (p.p7 * s.db[228][14]);
        let eq130_e1661_d_b15: f64 = (p.p7 * s.db[228][15]);
        let eq130_e1661_d_b16: f64 = (p.p7 * s.db[228][16]);
        let eq130_e1661_d_b17: f64 = (p.p7 * s.db[228][17]);
        let eq130_e1661_d_b18: f64 = (p.p7 * s.db[228][18]);
        let eq130_e1661_d_b19: f64 = (p.p7 * s.db[228][19]);
        let eq130_e1661_d_b20: f64 = (p.p7 * s.db[228][20]);
        let eq130_e1661_d_b21: f64 = (p.p7 * s.db[228][21]);
        let eq130_e1661_d_b22: f64 = (p.p7 * s.db[228][22]);
        let eq130_e1661_d_b23: f64 = (p.p7 * s.db[228][23]);
        let eq130_e1661_d_b24: f64 = (p.p7 * s.db[228][24]);
        let eq130_e1661_d_b25: f64 = (p.p7 * s.db[228][25]);
        let eq130_e1661_d_b26: f64 = (p.p7 * s.db[228][26]);
        let eq130_e1661_d_b27: f64 = (p.p7 * s.db[228][27]);
        let eq130_e1661_d_b28: f64 = (p.p7 * s.db[228][28]);
        let eq130_e1661_d_b29: f64 = (p.p7 * s.db[228][29]);
        let eq130_e1661_d_b30: f64 = (p.p7 * s.db[228][30]);
        let eq130_e1661_d_b31: f64 = (p.p7 * s.db[228][31]);
        let eq130_e1661_d_b32: f64 = (p.p7 * s.db[228][32]);
        let eq130_e1661_d_b33: f64 = (p.p7 * s.db[228][33]);
        let eq130_e1661_d_b34: f64 = (p.p7 * s.db[228][34]);
        let eq130_e1661_d_b35: f64 = (p.p7 * s.db[228][35]);
        let eq130_e1661_d_b36: f64 = (p.p7 * s.db[228][36]);
        let eq130_e1661_d_b37: f64 = (p.p7 * s.db[228][37]);
        let eq130_e1661_d_b38: f64 = (p.p7 * s.db[228][38]);
        let eq130_e1661_d_b39: f64 = (p.p7 * s.db[228][39]);
        let eq130_e1661_d_b40: f64 = (p.p7 * s.db[228][40]);
        let eq130_e1661_d_b41: f64 = (p.p7 * s.db[228][41]);
        let eq130_e1661_d_b42: f64 = (p.p7 * s.db[228][42]);
        let eq130_e1661_d_b43: f64 = (p.p7 * s.db[228][43]);
        let eq130_e1661_d_b44: f64 = (p.p7 * s.db[228][44]);
        let eq130_e1661_d_b45: f64 = (p.p7 * s.db[228][45]);
        let eq130_e1661_d_b46: f64 = (p.p7 * s.db[228][46]);
        let eq130_e1661_d_b47: f64 = (p.p7 * s.db[228][47]);
        let eq130_e1661_d_b48: f64 = (p.p7 * s.db[228][48]);
        let eq130_e1661_d_b49: f64 = (p.p7 * s.db[228][49]);
        let eq130_e1661_d_b50: f64 = (p.p7 * s.db[228][50]);
        let eq130_e1661_d_b51: f64 = (p.p7 * s.db[228][51]);
        let eq130_e1661_d_b52: f64 = (p.p7 * s.db[228][52]);
        let eq130_e1661_d_b53: f64 = (p.p7 * s.db[228][53]);
        let eq130_e1661_d_b54: f64 = (p.p7 * s.db[228][54]);
        let eq130_e1661_q: f64 = (p.p7 * eq130_e1660_q);
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
        let eq130_e1663_d_b0: f64 = (eq130_e1661_d_b0 * p.p246);
        let eq130_e1663_d_b1: f64 = (eq130_e1661_d_b1 * p.p246);
        let eq130_e1663_d_b2: f64 = (eq130_e1661_d_b2 * p.p246);
        let eq130_e1663_d_b3: f64 = (eq130_e1661_d_b3 * p.p246);
        let eq130_e1663_d_b4: f64 = (eq130_e1661_d_b4 * p.p246);
        let eq130_e1663_d_b5: f64 = (eq130_e1661_d_b5 * p.p246);
        let eq130_e1663_d_b6: f64 = (eq130_e1661_d_b6 * p.p246);
        let eq130_e1663_d_b7: f64 = (eq130_e1661_d_b7 * p.p246);
        let eq130_e1663_d_b8: f64 = (eq130_e1661_d_b8 * p.p246);
        let eq130_e1663_d_b9: f64 = (eq130_e1661_d_b9 * p.p246);
        let eq130_e1663_d_b10: f64 = (eq130_e1661_d_b10 * p.p246);
        let eq130_e1663_d_b11: f64 = (eq130_e1661_d_b11 * p.p246);
        let eq130_e1663_d_b12: f64 = (eq130_e1661_d_b12 * p.p246);
        let eq130_e1663_d_b13: f64 = (eq130_e1661_d_b13 * p.p246);
        let eq130_e1663_d_b14: f64 = (eq130_e1661_d_b14 * p.p246);
        let eq130_e1663_d_b15: f64 = (eq130_e1661_d_b15 * p.p246);
        let eq130_e1663_d_b16: f64 = (eq130_e1661_d_b16 * p.p246);
        let eq130_e1663_d_b17: f64 = (eq130_e1661_d_b17 * p.p246);
        let eq130_e1663_d_b18: f64 = (eq130_e1661_d_b18 * p.p246);
        let eq130_e1663_d_b19: f64 = (eq130_e1661_d_b19 * p.p246);
        let eq130_e1663_d_b20: f64 = (eq130_e1661_d_b20 * p.p246);
        let eq130_e1663_d_b21: f64 = (eq130_e1661_d_b21 * p.p246);
        let eq130_e1663_d_b22: f64 = (eq130_e1661_d_b22 * p.p246);
        let eq130_e1663_d_b23: f64 = (eq130_e1661_d_b23 * p.p246);
        let eq130_e1663_d_b24: f64 = (eq130_e1661_d_b24 * p.p246);
        let eq130_e1663_d_b25: f64 = (eq130_e1661_d_b25 * p.p246);
        let eq130_e1663_d_b26: f64 = (eq130_e1661_d_b26 * p.p246);
        let eq130_e1663_d_b27: f64 = (eq130_e1661_d_b27 * p.p246);
        let eq130_e1663_d_b28: f64 = (eq130_e1661_d_b28 * p.p246);
        let eq130_e1663_d_b29: f64 = (eq130_e1661_d_b29 * p.p246);
        let eq130_e1663_d_b30: f64 = (eq130_e1661_d_b30 * p.p246);
        let eq130_e1663_d_b31: f64 = (eq130_e1661_d_b31 * p.p246);
        let eq130_e1663_d_b32: f64 = (eq130_e1661_d_b32 * p.p246);
        let eq130_e1663_d_b33: f64 = (eq130_e1661_d_b33 * p.p246);
        let eq130_e1663_d_b34: f64 = (eq130_e1661_d_b34 * p.p246);
        let eq130_e1663_d_b35: f64 = (eq130_e1661_d_b35 * p.p246);
        let eq130_e1663_d_b36: f64 = (eq130_e1661_d_b36 * p.p246);
        let eq130_e1663_d_b37: f64 = (eq130_e1661_d_b37 * p.p246);
        let eq130_e1663_d_b38: f64 = (eq130_e1661_d_b38 * p.p246);
        let eq130_e1663_d_b39: f64 = (eq130_e1661_d_b39 * p.p246);
        let eq130_e1663_d_b40: f64 = (eq130_e1661_d_b40 * p.p246);
        let eq130_e1663_d_b41: f64 = (eq130_e1661_d_b41 * p.p246);
        let eq130_e1663_d_b42: f64 = (eq130_e1661_d_b42 * p.p246);
        let eq130_e1663_d_b43: f64 = (eq130_e1661_d_b43 * p.p246);
        let eq130_e1663_d_b44: f64 = (eq130_e1661_d_b44 * p.p246);
        let eq130_e1663_d_b45: f64 = (eq130_e1661_d_b45 * p.p246);
        let eq130_e1663_d_b46: f64 = (eq130_e1661_d_b46 * p.p246);
        let eq130_e1663_d_b47: f64 = (eq130_e1661_d_b47 * p.p246);
        let eq130_e1663_d_b48: f64 = (eq130_e1661_d_b48 * p.p246);
        let eq130_e1663_d_b49: f64 = (eq130_e1661_d_b49 * p.p246);
        let eq130_e1663_d_b50: f64 = (eq130_e1661_d_b50 * p.p246);
        let eq130_e1663_d_b51: f64 = (eq130_e1661_d_b51 * p.p246);
        let eq130_e1663_d_b52: f64 = (eq130_e1661_d_b52 * p.p246);
        let eq130_e1663_d_b53: f64 = (eq130_e1661_d_b53 * p.p246);
        let eq130_e1663_d_b54: f64 = (eq130_e1661_d_b54 * p.p246);
        let eq130_e1663_q: f64 = (eq130_e1661_q * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_d_b0, eq130_e1663_d_b1, eq130_e1663_d_b2, eq130_e1663_d_b3, eq130_e1663_d_b4, eq130_e1663_d_b5, eq130_e1663_d_b6, eq130_e1663_d_b7, eq130_e1663_d_b8, eq130_e1663_d_b9, eq130_e1663_d_b10, eq130_e1663_d_b11, eq130_e1663_d_b12, eq130_e1663_d_b13, eq130_e1663_d_b14, eq130_e1663_d_b15, eq130_e1663_d_b16, eq130_e1663_d_b17, eq130_e1663_d_b18, eq130_e1663_d_b19, eq130_e1663_d_b20, eq130_e1663_d_b21, eq130_e1663_d_b22, eq130_e1663_d_b23, eq130_e1663_d_b24, eq130_e1663_d_b25, eq130_e1663_d_b26, eq130_e1663_d_b27, eq130_e1663_d_b28, eq130_e1663_d_b29, eq130_e1663_d_b30, eq130_e1663_d_b31, eq130_e1663_d_b32, eq130_e1663_d_b33, eq130_e1663_d_b34, eq130_e1663_d_b35, eq130_e1663_d_b36, eq130_e1663_d_b37, eq130_e1663_d_b38, eq130_e1663_d_b39, eq130_e1663_d_b40, eq130_e1663_d_b41, eq130_e1663_d_b42, eq130_e1663_d_b43, eq130_e1663_d_b44, eq130_e1663_d_b45, eq130_e1663_d_b46, eq130_e1663_d_b47, eq130_e1663_d_b48, eq130_e1663_d_b49, eq130_e1663_d_b50, eq130_e1663_d_b51, eq130_e1663_d_b52, eq130_e1663_d_b53, eq130_e1663_d_b54, eq130_e1663_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_reactive_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_reactive_branch_derivatives: [f64; 55] = [eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq130_reactive_node_derivatives,
            branches,
            &eq130_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54, eq131_e1677_q,) = {
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
        let eq131_e1673_d_b0: f64 = (p.p251 * s.db[228][0]);
        let eq131_e1673_d_b1: f64 = (p.p251 * s.db[228][1]);
        let eq131_e1673_d_b2: f64 = (p.p251 * s.db[228][2]);
        let eq131_e1673_d_b3: f64 = (p.p251 * s.db[228][3]);
        let eq131_e1673_d_b4: f64 = (p.p251 * s.db[228][4]);
        let eq131_e1673_d_b5: f64 = (p.p251 * s.db[228][5]);
        let eq131_e1673_d_b6: f64 = (p.p251 * s.db[228][6]);
        let eq131_e1673_d_b7: f64 = (p.p251 * s.db[228][7]);
        let eq131_e1673_d_b8: f64 = (p.p251 * s.db[228][8]);
        let eq131_e1673_d_b9: f64 = (p.p251 * s.db[228][9]);
        let eq131_e1673_d_b10: f64 = (p.p251 * s.db[228][10]);
        let eq131_e1673_d_b11: f64 = (p.p251 * s.db[228][11]);
        let eq131_e1673_d_b12: f64 = (p.p251 * s.db[228][12]);
        let eq131_e1673_d_b13: f64 = (p.p251 * s.db[228][13]);
        let eq131_e1673_d_b14: f64 = (p.p251 * s.db[228][14]);
        let eq131_e1673_d_b15: f64 = (p.p251 * s.db[228][15]);
        let eq131_e1673_d_b16: f64 = (p.p251 * s.db[228][16]);
        let eq131_e1673_d_b17: f64 = (p.p251 * s.db[228][17]);
        let eq131_e1673_d_b18: f64 = (p.p251 * s.db[228][18]);
        let eq131_e1673_d_b19: f64 = (p.p251 * s.db[228][19]);
        let eq131_e1673_d_b20: f64 = (p.p251 * s.db[228][20]);
        let eq131_e1673_d_b21: f64 = (p.p251 * s.db[228][21]);
        let eq131_e1673_d_b22: f64 = (p.p251 * s.db[228][22]);
        let eq131_e1673_d_b23: f64 = (p.p251 * s.db[228][23]);
        let eq131_e1673_d_b24: f64 = (p.p251 * s.db[228][24]);
        let eq131_e1673_d_b25: f64 = (p.p251 * s.db[228][25]);
        let eq131_e1673_d_b26: f64 = (p.p251 * s.db[228][26]);
        let eq131_e1673_d_b27: f64 = (p.p251 * s.db[228][27]);
        let eq131_e1673_d_b28: f64 = (p.p251 * s.db[228][28]);
        let eq131_e1673_d_b29: f64 = (p.p251 * s.db[228][29]);
        let eq131_e1673_d_b30: f64 = (p.p251 * s.db[228][30]);
        let eq131_e1673_d_b31: f64 = (p.p251 * s.db[228][31]);
        let eq131_e1673_d_b32: f64 = (p.p251 * s.db[228][32]);
        let eq131_e1673_d_b33: f64 = (p.p251 * s.db[228][33]);
        let eq131_e1673_d_b34: f64 = (p.p251 * s.db[228][34]);
        let eq131_e1673_d_b35: f64 = (p.p251 * s.db[228][35]);
        let eq131_e1673_d_b36: f64 = (p.p251 * s.db[228][36]);
        let eq131_e1673_d_b37: f64 = (p.p251 * s.db[228][37]);
        let eq131_e1673_d_b38: f64 = (p.p251 * s.db[228][38]);
        let eq131_e1673_d_b39: f64 = (p.p251 * s.db[228][39]);
        let eq131_e1673_d_b40: f64 = (p.p251 * s.db[228][40]);
        let eq131_e1673_d_b41: f64 = (p.p251 * s.db[228][41]);
        let eq131_e1673_d_b42: f64 = (p.p251 * s.db[228][42]);
        let eq131_e1673_d_b43: f64 = (p.p251 * s.db[228][43]);
        let eq131_e1673_d_b44: f64 = (p.p251 * s.db[228][44]);
        let eq131_e1673_d_b45: f64 = (p.p251 * s.db[228][45]);
        let eq131_e1673_d_b46: f64 = (p.p251 * s.db[228][46]);
        let eq131_e1673_d_b47: f64 = (p.p251 * s.db[228][47]);
        let eq131_e1673_d_b48: f64 = (p.p251 * s.db[228][48]);
        let eq131_e1673_d_b49: f64 = (p.p251 * s.db[228][49]);
        let eq131_e1673_d_b50: f64 = (p.p251 * s.db[228][50]);
        let eq131_e1673_d_b51: f64 = (p.p251 * s.db[228][51]);
        let eq131_e1673_d_b52: f64 = (p.p251 * s.db[228][52]);
        let eq131_e1673_d_b53: f64 = (p.p251 * s.db[228][53]);
        let eq131_e1673_d_b54: f64 = (p.p251 * s.db[228][54]);
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
        let eq131_e1675_d_b0: f64 = (p.p7 * eq131_e1673_d_b0);
        let eq131_e1675_d_b1: f64 = (p.p7 * eq131_e1673_d_b1);
        let eq131_e1675_d_b2: f64 = (p.p7 * eq131_e1673_d_b2);
        let eq131_e1675_d_b3: f64 = (p.p7 * eq131_e1673_d_b3);
        let eq131_e1675_d_b4: f64 = (p.p7 * eq131_e1673_d_b4);
        let eq131_e1675_d_b5: f64 = (p.p7 * eq131_e1673_d_b5);
        let eq131_e1675_d_b6: f64 = (p.p7 * eq131_e1673_d_b6);
        let eq131_e1675_d_b7: f64 = (p.p7 * eq131_e1673_d_b7);
        let eq131_e1675_d_b8: f64 = (p.p7 * eq131_e1673_d_b8);
        let eq131_e1675_d_b9: f64 = (p.p7 * eq131_e1673_d_b9);
        let eq131_e1675_d_b10: f64 = (p.p7 * eq131_e1673_d_b10);
        let eq131_e1675_d_b11: f64 = (p.p7 * eq131_e1673_d_b11);
        let eq131_e1675_d_b12: f64 = (p.p7 * eq131_e1673_d_b12);
        let eq131_e1675_d_b13: f64 = (p.p7 * eq131_e1673_d_b13);
        let eq131_e1675_d_b14: f64 = (p.p7 * eq131_e1673_d_b14);
        let eq131_e1675_d_b15: f64 = (p.p7 * eq131_e1673_d_b15);
        let eq131_e1675_d_b16: f64 = (p.p7 * eq131_e1673_d_b16);
        let eq131_e1675_d_b17: f64 = (p.p7 * eq131_e1673_d_b17);
        let eq131_e1675_d_b18: f64 = (p.p7 * eq131_e1673_d_b18);
        let eq131_e1675_d_b19: f64 = (p.p7 * eq131_e1673_d_b19);
        let eq131_e1675_d_b20: f64 = (p.p7 * eq131_e1673_d_b20);
        let eq131_e1675_d_b21: f64 = (p.p7 * eq131_e1673_d_b21);
        let eq131_e1675_d_b22: f64 = (p.p7 * eq131_e1673_d_b22);
        let eq131_e1675_d_b23: f64 = (p.p7 * eq131_e1673_d_b23);
        let eq131_e1675_d_b24: f64 = (p.p7 * eq131_e1673_d_b24);
        let eq131_e1675_d_b25: f64 = (p.p7 * eq131_e1673_d_b25);
        let eq131_e1675_d_b26: f64 = (p.p7 * eq131_e1673_d_b26);
        let eq131_e1675_d_b27: f64 = (p.p7 * eq131_e1673_d_b27);
        let eq131_e1675_d_b28: f64 = (p.p7 * eq131_e1673_d_b28);
        let eq131_e1675_d_b29: f64 = (p.p7 * eq131_e1673_d_b29);
        let eq131_e1675_d_b30: f64 = (p.p7 * eq131_e1673_d_b30);
        let eq131_e1675_d_b31: f64 = (p.p7 * eq131_e1673_d_b31);
        let eq131_e1675_d_b32: f64 = (p.p7 * eq131_e1673_d_b32);
        let eq131_e1675_d_b33: f64 = (p.p7 * eq131_e1673_d_b33);
        let eq131_e1675_d_b34: f64 = (p.p7 * eq131_e1673_d_b34);
        let eq131_e1675_d_b35: f64 = (p.p7 * eq131_e1673_d_b35);
        let eq131_e1675_d_b36: f64 = (p.p7 * eq131_e1673_d_b36);
        let eq131_e1675_d_b37: f64 = (p.p7 * eq131_e1673_d_b37);
        let eq131_e1675_d_b38: f64 = (p.p7 * eq131_e1673_d_b38);
        let eq131_e1675_d_b39: f64 = (p.p7 * eq131_e1673_d_b39);
        let eq131_e1675_d_b40: f64 = (p.p7 * eq131_e1673_d_b40);
        let eq131_e1675_d_b41: f64 = (p.p7 * eq131_e1673_d_b41);
        let eq131_e1675_d_b42: f64 = (p.p7 * eq131_e1673_d_b42);
        let eq131_e1675_d_b43: f64 = (p.p7 * eq131_e1673_d_b43);
        let eq131_e1675_d_b44: f64 = (p.p7 * eq131_e1673_d_b44);
        let eq131_e1675_d_b45: f64 = (p.p7 * eq131_e1673_d_b45);
        let eq131_e1675_d_b46: f64 = (p.p7 * eq131_e1673_d_b46);
        let eq131_e1675_d_b47: f64 = (p.p7 * eq131_e1673_d_b47);
        let eq131_e1675_d_b48: f64 = (p.p7 * eq131_e1673_d_b48);
        let eq131_e1675_d_b49: f64 = (p.p7 * eq131_e1673_d_b49);
        let eq131_e1675_d_b50: f64 = (p.p7 * eq131_e1673_d_b50);
        let eq131_e1675_d_b51: f64 = (p.p7 * eq131_e1673_d_b51);
        let eq131_e1675_d_b52: f64 = (p.p7 * eq131_e1673_d_b52);
        let eq131_e1675_d_b53: f64 = (p.p7 * eq131_e1673_d_b53);
        let eq131_e1675_d_b54: f64 = (p.p7 * eq131_e1673_d_b54);
        let eq131_e1675_q: f64 = (p.p7 * eq131_e1674_q);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_d_b0, eq131_e1675_d_b1, eq131_e1675_d_b2, eq131_e1675_d_b3, eq131_e1675_d_b4, eq131_e1675_d_b5, eq131_e1675_d_b6, eq131_e1675_d_b7, eq131_e1675_d_b8, eq131_e1675_d_b9, eq131_e1675_d_b10, eq131_e1675_d_b11, eq131_e1675_d_b12, eq131_e1675_d_b13, eq131_e1675_d_b14, eq131_e1675_d_b15, eq131_e1675_d_b16, eq131_e1675_d_b17, eq131_e1675_d_b18, eq131_e1675_d_b19, eq131_e1675_d_b20, eq131_e1675_d_b21, eq131_e1675_d_b22, eq131_e1675_d_b23, eq131_e1675_d_b24, eq131_e1675_d_b25, eq131_e1675_d_b26, eq131_e1675_d_b27, eq131_e1675_d_b28, eq131_e1675_d_b29, eq131_e1675_d_b30, eq131_e1675_d_b31, eq131_e1675_d_b32, eq131_e1675_d_b33, eq131_e1675_d_b34, eq131_e1675_d_b35, eq131_e1675_d_b36, eq131_e1675_d_b37, eq131_e1675_d_b38, eq131_e1675_d_b39, eq131_e1675_d_b40, eq131_e1675_d_b41, eq131_e1675_d_b42, eq131_e1675_d_b43, eq131_e1675_d_b44, eq131_e1675_d_b45, eq131_e1675_d_b46, eq131_e1675_d_b47, eq131_e1675_d_b48, eq131_e1675_d_b49, eq131_e1675_d_b50, eq131_e1675_d_b51, eq131_e1675_d_b52, eq131_e1675_d_b53, eq131_e1675_d_b54, eq131_e1675_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_reactive_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_reactive_branch_derivatives: [f64; 55] = [eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq131_reactive_node_derivatives,
            branches,
            &eq131_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54, eq132_e1686_q,) = {
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
        let eq132_e1684_d_b0: f64 = (p.p7 * s.db[241][0]);
        let eq132_e1684_d_b1: f64 = (p.p7 * s.db[241][1]);
        let eq132_e1684_d_b2: f64 = (p.p7 * s.db[241][2]);
        let eq132_e1684_d_b3: f64 = (p.p7 * s.db[241][3]);
        let eq132_e1684_d_b4: f64 = (p.p7 * s.db[241][4]);
        let eq132_e1684_d_b5: f64 = (p.p7 * s.db[241][5]);
        let eq132_e1684_d_b6: f64 = (p.p7 * s.db[241][6]);
        let eq132_e1684_d_b7: f64 = (p.p7 * s.db[241][7]);
        let eq132_e1684_d_b8: f64 = (p.p7 * s.db[241][8]);
        let eq132_e1684_d_b9: f64 = (p.p7 * s.db[241][9]);
        let eq132_e1684_d_b10: f64 = (p.p7 * s.db[241][10]);
        let eq132_e1684_d_b11: f64 = (p.p7 * s.db[241][11]);
        let eq132_e1684_d_b12: f64 = (p.p7 * s.db[241][12]);
        let eq132_e1684_d_b13: f64 = (p.p7 * s.db[241][13]);
        let eq132_e1684_d_b14: f64 = (p.p7 * s.db[241][14]);
        let eq132_e1684_d_b15: f64 = (p.p7 * s.db[241][15]);
        let eq132_e1684_d_b16: f64 = (p.p7 * s.db[241][16]);
        let eq132_e1684_d_b17: f64 = (p.p7 * s.db[241][17]);
        let eq132_e1684_d_b18: f64 = (p.p7 * s.db[241][18]);
        let eq132_e1684_d_b19: f64 = (p.p7 * s.db[241][19]);
        let eq132_e1684_d_b20: f64 = (p.p7 * s.db[241][20]);
        let eq132_e1684_d_b21: f64 = (p.p7 * s.db[241][21]);
        let eq132_e1684_d_b22: f64 = (p.p7 * s.db[241][22]);
        let eq132_e1684_d_b23: f64 = (p.p7 * s.db[241][23]);
        let eq132_e1684_d_b24: f64 = (p.p7 * s.db[241][24]);
        let eq132_e1684_d_b25: f64 = (p.p7 * s.db[241][25]);
        let eq132_e1684_d_b26: f64 = (p.p7 * s.db[241][26]);
        let eq132_e1684_d_b27: f64 = (p.p7 * s.db[241][27]);
        let eq132_e1684_d_b28: f64 = (p.p7 * s.db[241][28]);
        let eq132_e1684_d_b29: f64 = (p.p7 * s.db[241][29]);
        let eq132_e1684_d_b30: f64 = (p.p7 * s.db[241][30]);
        let eq132_e1684_d_b31: f64 = (p.p7 * s.db[241][31]);
        let eq132_e1684_d_b32: f64 = (p.p7 * s.db[241][32]);
        let eq132_e1684_d_b33: f64 = (p.p7 * s.db[241][33]);
        let eq132_e1684_d_b34: f64 = (p.p7 * s.db[241][34]);
        let eq132_e1684_d_b35: f64 = (p.p7 * s.db[241][35]);
        let eq132_e1684_d_b36: f64 = (p.p7 * s.db[241][36]);
        let eq132_e1684_d_b37: f64 = (p.p7 * s.db[241][37]);
        let eq132_e1684_d_b38: f64 = (p.p7 * s.db[241][38]);
        let eq132_e1684_d_b39: f64 = (p.p7 * s.db[241][39]);
        let eq132_e1684_d_b40: f64 = (p.p7 * s.db[241][40]);
        let eq132_e1684_d_b41: f64 = (p.p7 * s.db[241][41]);
        let eq132_e1684_d_b42: f64 = (p.p7 * s.db[241][42]);
        let eq132_e1684_d_b43: f64 = (p.p7 * s.db[241][43]);
        let eq132_e1684_d_b44: f64 = (p.p7 * s.db[241][44]);
        let eq132_e1684_d_b45: f64 = (p.p7 * s.db[241][45]);
        let eq132_e1684_d_b46: f64 = (p.p7 * s.db[241][46]);
        let eq132_e1684_d_b47: f64 = (p.p7 * s.db[241][47]);
        let eq132_e1684_d_b48: f64 = (p.p7 * s.db[241][48]);
        let eq132_e1684_d_b49: f64 = (p.p7 * s.db[241][49]);
        let eq132_e1684_d_b50: f64 = (p.p7 * s.db[241][50]);
        let eq132_e1684_d_b51: f64 = (p.p7 * s.db[241][51]);
        let eq132_e1684_d_b52: f64 = (p.p7 * s.db[241][52]);
        let eq132_e1684_d_b53: f64 = (p.p7 * s.db[241][53]);
        let eq132_e1684_d_b54: f64 = (p.p7 * s.db[241][54]);
        let eq132_e1684_q: f64 = (p.p7 * eq132_e1683_q);
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22, eq132_e1684_d_b0, eq132_e1684_d_b1, eq132_e1684_d_b2, eq132_e1684_d_b3, eq132_e1684_d_b4, eq132_e1684_d_b5, eq132_e1684_d_b6, eq132_e1684_d_b7, eq132_e1684_d_b8, eq132_e1684_d_b9, eq132_e1684_d_b10, eq132_e1684_d_b11, eq132_e1684_d_b12, eq132_e1684_d_b13, eq132_e1684_d_b14, eq132_e1684_d_b15, eq132_e1684_d_b16, eq132_e1684_d_b17, eq132_e1684_d_b18, eq132_e1684_d_b19, eq132_e1684_d_b20, eq132_e1684_d_b21, eq132_e1684_d_b22, eq132_e1684_d_b23, eq132_e1684_d_b24, eq132_e1684_d_b25, eq132_e1684_d_b26, eq132_e1684_d_b27, eq132_e1684_d_b28, eq132_e1684_d_b29, eq132_e1684_d_b30, eq132_e1684_d_b31, eq132_e1684_d_b32, eq132_e1684_d_b33, eq132_e1684_d_b34, eq132_e1684_d_b35, eq132_e1684_d_b36, eq132_e1684_d_b37, eq132_e1684_d_b38, eq132_e1684_d_b39, eq132_e1684_d_b40, eq132_e1684_d_b41, eq132_e1684_d_b42, eq132_e1684_d_b43, eq132_e1684_d_b44, eq132_e1684_d_b45, eq132_e1684_d_b46, eq132_e1684_d_b47, eq132_e1684_d_b48, eq132_e1684_d_b49, eq132_e1684_d_b50, eq132_e1684_d_b51, eq132_e1684_d_b52, eq132_e1684_d_b53, eq132_e1684_d_b54, eq132_e1684_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_reactive_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_reactive_branch_derivatives: [f64; 55] = [eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            nodes,
            &eq132_reactive_node_derivatives,
            branches,
            &eq132_reactive_branch_derivatives,
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[240][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[240][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[240][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[240][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[240][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[240][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[240][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[240][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[240][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[240][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[240][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[240][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[240][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[240][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[240][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[240][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[240][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[240][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[240][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[240][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[240][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[240][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[240][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[240][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[240][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[240][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[240][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[240][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[240][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[240][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[240][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[240][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[240][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[240][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[240][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[240][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[240][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[240][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[240][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[240][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[240][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[240][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[240][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[240][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[240][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[240][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[240][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[240][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[240][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[240][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[240][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[240][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[240][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[240][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[240][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[240][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[240][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[240][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[240][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[240][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[240][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[240][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[240][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[240][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[240][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[240][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[240][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[240][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[240][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[240][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[240][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[240][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[240][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[240][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[240][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[240][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[240][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[240][54]);
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54, eq133_e1697_q,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq133_e1694_q: f64 = s.v[240];
        let eq133_e1695: f64 = (p.p7 * s.v[240]);
        let eq133_e1695_q: f64 = (p.p7 * eq133_e1694_q);
        (eq133_e1695, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq133_e1695_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_reactive_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_reactive_branch_derivatives: [f64; 55] = [eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq133_reactive_node_derivatives,
            branches,
            &eq133_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54, eq134_e1710_q,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq134_e1705_q: f64 = s.v[240];
        let eq134_e1706: f64 = (p.p7 * s.v[240]);
        let eq134_e1706_q: f64 = (p.p7 * eq134_e1705_q);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq134_e1708_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq134_e1708_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq134_e1708_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq134_e1708_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq134_e1708_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq134_e1708_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq134_e1708_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq134_e1708_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq134_e1708_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq134_e1708_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq134_e1708_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq134_e1708_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq134_e1708_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq134_e1708_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq134_e1708_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq134_e1708_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq134_e1708_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq134_e1708_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq134_e1708_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq134_e1708_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq134_e1708_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq134_e1708_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq134_e1708_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq134_e1708_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq134_e1708_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq134_e1708_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq134_e1708_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq134_e1708_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq134_e1708_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq134_e1708_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq134_e1708_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq134_e1708_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq134_e1708_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq134_e1708_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq134_e1708_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq134_e1708_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq134_e1708_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq134_e1708_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq134_e1708_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq134_e1708_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq134_e1708_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq134_e1708_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq134_e1708_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq134_e1708_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq134_e1708_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq134_e1708_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq134_e1708_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq134_e1708_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq134_e1708_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq134_e1708_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq134_e1708_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq134_e1708_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq134_e1708_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq134_e1708_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq134_e1708_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq134_e1708_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq134_e1708_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq134_e1708_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq134_e1708_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq134_e1708_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq134_e1708_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq134_e1708_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq134_e1708_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq134_e1708_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq134_e1708_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq134_e1708_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq134_e1708_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq134_e1708_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq134_e1708_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq134_e1708_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq134_e1708_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq134_e1708_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq134_e1708_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq134_e1708_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq134_e1708_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq134_e1708_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq134_e1708_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        let eq134_e1708_q: f64 = (eq134_e1706_q * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22, eq134_e1708_d_b0, eq134_e1708_d_b1, eq134_e1708_d_b2, eq134_e1708_d_b3, eq134_e1708_d_b4, eq134_e1708_d_b5, eq134_e1708_d_b6, eq134_e1708_d_b7, eq134_e1708_d_b8, eq134_e1708_d_b9, eq134_e1708_d_b10, eq134_e1708_d_b11, eq134_e1708_d_b12, eq134_e1708_d_b13, eq134_e1708_d_b14, eq134_e1708_d_b15, eq134_e1708_d_b16, eq134_e1708_d_b17, eq134_e1708_d_b18, eq134_e1708_d_b19, eq134_e1708_d_b20, eq134_e1708_d_b21, eq134_e1708_d_b22, eq134_e1708_d_b23, eq134_e1708_d_b24, eq134_e1708_d_b25, eq134_e1708_d_b26, eq134_e1708_d_b27, eq134_e1708_d_b28, eq134_e1708_d_b29, eq134_e1708_d_b30, eq134_e1708_d_b31, eq134_e1708_d_b32, eq134_e1708_d_b33, eq134_e1708_d_b34, eq134_e1708_d_b35, eq134_e1708_d_b36, eq134_e1708_d_b37, eq134_e1708_d_b38, eq134_e1708_d_b39, eq134_e1708_d_b40, eq134_e1708_d_b41, eq134_e1708_d_b42, eq134_e1708_d_b43, eq134_e1708_d_b44, eq134_e1708_d_b45, eq134_e1708_d_b46, eq134_e1708_d_b47, eq134_e1708_d_b48, eq134_e1708_d_b49, eq134_e1708_d_b50, eq134_e1708_d_b51, eq134_e1708_d_b52, eq134_e1708_d_b53, eq134_e1708_d_b54, eq134_e1708_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_reactive_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_reactive_branch_derivatives: [f64; 55] = [eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq134_reactive_node_derivatives,
            branches,
            &eq134_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54, eq135_e1722_q,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq135_e1719_q: f64 = s.v[240];
        let eq135_e1720: f64 = (p.p7 * s.v[240]);
        let eq135_e1720_q: f64 = (p.p7 * eq135_e1719_q);
        (eq135_e1720, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq135_e1720_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_reactive_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_reactive_branch_derivatives: [f64; 55] = [eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            nodes,
            &eq135_reactive_node_derivatives,
            branches,
            &eq135_reactive_branch_derivatives,
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
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54, eq136_e1736_q,) = {
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
        let eq136_e1732_d_b0: f64 = (p.p7 * s.db[240][0]);
        let eq136_e1732_d_b1: f64 = (p.p7 * s.db[240][1]);
        let eq136_e1732_d_b2: f64 = (p.p7 * s.db[240][2]);
        let eq136_e1732_d_b3: f64 = (p.p7 * s.db[240][3]);
        let eq136_e1732_d_b4: f64 = (p.p7 * s.db[240][4]);
        let eq136_e1732_d_b5: f64 = (p.p7 * s.db[240][5]);
        let eq136_e1732_d_b6: f64 = (p.p7 * s.db[240][6]);
        let eq136_e1732_d_b7: f64 = (p.p7 * s.db[240][7]);
        let eq136_e1732_d_b8: f64 = (p.p7 * s.db[240][8]);
        let eq136_e1732_d_b9: f64 = (p.p7 * s.db[240][9]);
        let eq136_e1732_d_b10: f64 = (p.p7 * s.db[240][10]);
        let eq136_e1732_d_b11: f64 = (p.p7 * s.db[240][11]);
        let eq136_e1732_d_b12: f64 = (p.p7 * s.db[240][12]);
        let eq136_e1732_d_b13: f64 = (p.p7 * s.db[240][13]);
        let eq136_e1732_d_b14: f64 = (p.p7 * s.db[240][14]);
        let eq136_e1732_d_b15: f64 = (p.p7 * s.db[240][15]);
        let eq136_e1732_d_b16: f64 = (p.p7 * s.db[240][16]);
        let eq136_e1732_d_b17: f64 = (p.p7 * s.db[240][17]);
        let eq136_e1732_d_b18: f64 = (p.p7 * s.db[240][18]);
        let eq136_e1732_d_b19: f64 = (p.p7 * s.db[240][19]);
        let eq136_e1732_d_b20: f64 = (p.p7 * s.db[240][20]);
        let eq136_e1732_d_b21: f64 = (p.p7 * s.db[240][21]);
        let eq136_e1732_d_b22: f64 = (p.p7 * s.db[240][22]);
        let eq136_e1732_d_b23: f64 = (p.p7 * s.db[240][23]);
        let eq136_e1732_d_b24: f64 = (p.p7 * s.db[240][24]);
        let eq136_e1732_d_b25: f64 = (p.p7 * s.db[240][25]);
        let eq136_e1732_d_b26: f64 = (p.p7 * s.db[240][26]);
        let eq136_e1732_d_b27: f64 = (p.p7 * s.db[240][27]);
        let eq136_e1732_d_b28: f64 = (p.p7 * s.db[240][28]);
        let eq136_e1732_d_b29: f64 = (p.p7 * s.db[240][29]);
        let eq136_e1732_d_b30: f64 = (p.p7 * s.db[240][30]);
        let eq136_e1732_d_b31: f64 = (p.p7 * s.db[240][31]);
        let eq136_e1732_d_b32: f64 = (p.p7 * s.db[240][32]);
        let eq136_e1732_d_b33: f64 = (p.p7 * s.db[240][33]);
        let eq136_e1732_d_b34: f64 = (p.p7 * s.db[240][34]);
        let eq136_e1732_d_b35: f64 = (p.p7 * s.db[240][35]);
        let eq136_e1732_d_b36: f64 = (p.p7 * s.db[240][36]);
        let eq136_e1732_d_b37: f64 = (p.p7 * s.db[240][37]);
        let eq136_e1732_d_b38: f64 = (p.p7 * s.db[240][38]);
        let eq136_e1732_d_b39: f64 = (p.p7 * s.db[240][39]);
        let eq136_e1732_d_b40: f64 = (p.p7 * s.db[240][40]);
        let eq136_e1732_d_b41: f64 = (p.p7 * s.db[240][41]);
        let eq136_e1732_d_b42: f64 = (p.p7 * s.db[240][42]);
        let eq136_e1732_d_b43: f64 = (p.p7 * s.db[240][43]);
        let eq136_e1732_d_b44: f64 = (p.p7 * s.db[240][44]);
        let eq136_e1732_d_b45: f64 = (p.p7 * s.db[240][45]);
        let eq136_e1732_d_b46: f64 = (p.p7 * s.db[240][46]);
        let eq136_e1732_d_b47: f64 = (p.p7 * s.db[240][47]);
        let eq136_e1732_d_b48: f64 = (p.p7 * s.db[240][48]);
        let eq136_e1732_d_b49: f64 = (p.p7 * s.db[240][49]);
        let eq136_e1732_d_b50: f64 = (p.p7 * s.db[240][50]);
        let eq136_e1732_d_b51: f64 = (p.p7 * s.db[240][51]);
        let eq136_e1732_d_b52: f64 = (p.p7 * s.db[240][52]);
        let eq136_e1732_d_b53: f64 = (p.p7 * s.db[240][53]);
        let eq136_e1732_d_b54: f64 = (p.p7 * s.db[240][54]);
        let eq136_e1732_q: f64 = (p.p7 * eq136_e1731_q);
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
        let eq136_e1734_d_b0: f64 = (eq136_e1732_d_b0 * p.p246);
        let eq136_e1734_d_b1: f64 = (eq136_e1732_d_b1 * p.p246);
        let eq136_e1734_d_b2: f64 = (eq136_e1732_d_b2 * p.p246);
        let eq136_e1734_d_b3: f64 = (eq136_e1732_d_b3 * p.p246);
        let eq136_e1734_d_b4: f64 = (eq136_e1732_d_b4 * p.p246);
        let eq136_e1734_d_b5: f64 = (eq136_e1732_d_b5 * p.p246);
        let eq136_e1734_d_b6: f64 = (eq136_e1732_d_b6 * p.p246);
        let eq136_e1734_d_b7: f64 = (eq136_e1732_d_b7 * p.p246);
        let eq136_e1734_d_b8: f64 = (eq136_e1732_d_b8 * p.p246);
        let eq136_e1734_d_b9: f64 = (eq136_e1732_d_b9 * p.p246);
        let eq136_e1734_d_b10: f64 = (eq136_e1732_d_b10 * p.p246);
        let eq136_e1734_d_b11: f64 = (eq136_e1732_d_b11 * p.p246);
        let eq136_e1734_d_b12: f64 = (eq136_e1732_d_b12 * p.p246);
        let eq136_e1734_d_b13: f64 = (eq136_e1732_d_b13 * p.p246);
        let eq136_e1734_d_b14: f64 = (eq136_e1732_d_b14 * p.p246);
        let eq136_e1734_d_b15: f64 = (eq136_e1732_d_b15 * p.p246);
        let eq136_e1734_d_b16: f64 = (eq136_e1732_d_b16 * p.p246);
        let eq136_e1734_d_b17: f64 = (eq136_e1732_d_b17 * p.p246);
        let eq136_e1734_d_b18: f64 = (eq136_e1732_d_b18 * p.p246);
        let eq136_e1734_d_b19: f64 = (eq136_e1732_d_b19 * p.p246);
        let eq136_e1734_d_b20: f64 = (eq136_e1732_d_b20 * p.p246);
        let eq136_e1734_d_b21: f64 = (eq136_e1732_d_b21 * p.p246);
        let eq136_e1734_d_b22: f64 = (eq136_e1732_d_b22 * p.p246);
        let eq136_e1734_d_b23: f64 = (eq136_e1732_d_b23 * p.p246);
        let eq136_e1734_d_b24: f64 = (eq136_e1732_d_b24 * p.p246);
        let eq136_e1734_d_b25: f64 = (eq136_e1732_d_b25 * p.p246);
        let eq136_e1734_d_b26: f64 = (eq136_e1732_d_b26 * p.p246);
        let eq136_e1734_d_b27: f64 = (eq136_e1732_d_b27 * p.p246);
        let eq136_e1734_d_b28: f64 = (eq136_e1732_d_b28 * p.p246);
        let eq136_e1734_d_b29: f64 = (eq136_e1732_d_b29 * p.p246);
        let eq136_e1734_d_b30: f64 = (eq136_e1732_d_b30 * p.p246);
        let eq136_e1734_d_b31: f64 = (eq136_e1732_d_b31 * p.p246);
        let eq136_e1734_d_b32: f64 = (eq136_e1732_d_b32 * p.p246);
        let eq136_e1734_d_b33: f64 = (eq136_e1732_d_b33 * p.p246);
        let eq136_e1734_d_b34: f64 = (eq136_e1732_d_b34 * p.p246);
        let eq136_e1734_d_b35: f64 = (eq136_e1732_d_b35 * p.p246);
        let eq136_e1734_d_b36: f64 = (eq136_e1732_d_b36 * p.p246);
        let eq136_e1734_d_b37: f64 = (eq136_e1732_d_b37 * p.p246);
        let eq136_e1734_d_b38: f64 = (eq136_e1732_d_b38 * p.p246);
        let eq136_e1734_d_b39: f64 = (eq136_e1732_d_b39 * p.p246);
        let eq136_e1734_d_b40: f64 = (eq136_e1732_d_b40 * p.p246);
        let eq136_e1734_d_b41: f64 = (eq136_e1732_d_b41 * p.p246);
        let eq136_e1734_d_b42: f64 = (eq136_e1732_d_b42 * p.p246);
        let eq136_e1734_d_b43: f64 = (eq136_e1732_d_b43 * p.p246);
        let eq136_e1734_d_b44: f64 = (eq136_e1732_d_b44 * p.p246);
        let eq136_e1734_d_b45: f64 = (eq136_e1732_d_b45 * p.p246);
        let eq136_e1734_d_b46: f64 = (eq136_e1732_d_b46 * p.p246);
        let eq136_e1734_d_b47: f64 = (eq136_e1732_d_b47 * p.p246);
        let eq136_e1734_d_b48: f64 = (eq136_e1732_d_b48 * p.p246);
        let eq136_e1734_d_b49: f64 = (eq136_e1732_d_b49 * p.p246);
        let eq136_e1734_d_b50: f64 = (eq136_e1732_d_b50 * p.p246);
        let eq136_e1734_d_b51: f64 = (eq136_e1732_d_b51 * p.p246);
        let eq136_e1734_d_b52: f64 = (eq136_e1732_d_b52 * p.p246);
        let eq136_e1734_d_b53: f64 = (eq136_e1732_d_b53 * p.p246);
        let eq136_e1734_d_b54: f64 = (eq136_e1732_d_b54 * p.p246);
        let eq136_e1734_q: f64 = (eq136_e1732_q * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22, eq136_e1734_d_b0, eq136_e1734_d_b1, eq136_e1734_d_b2, eq136_e1734_d_b3, eq136_e1734_d_b4, eq136_e1734_d_b5, eq136_e1734_d_b6, eq136_e1734_d_b7, eq136_e1734_d_b8, eq136_e1734_d_b9, eq136_e1734_d_b10, eq136_e1734_d_b11, eq136_e1734_d_b12, eq136_e1734_d_b13, eq136_e1734_d_b14, eq136_e1734_d_b15, eq136_e1734_d_b16, eq136_e1734_d_b17, eq136_e1734_d_b18, eq136_e1734_d_b19, eq136_e1734_d_b20, eq136_e1734_d_b21, eq136_e1734_d_b22, eq136_e1734_d_b23, eq136_e1734_d_b24, eq136_e1734_d_b25, eq136_e1734_d_b26, eq136_e1734_d_b27, eq136_e1734_d_b28, eq136_e1734_d_b29, eq136_e1734_d_b30, eq136_e1734_d_b31, eq136_e1734_d_b32, eq136_e1734_d_b33, eq136_e1734_d_b34, eq136_e1734_d_b35, eq136_e1734_d_b36, eq136_e1734_d_b37, eq136_e1734_d_b38, eq136_e1734_d_b39, eq136_e1734_d_b40, eq136_e1734_d_b41, eq136_e1734_d_b42, eq136_e1734_d_b43, eq136_e1734_d_b44, eq136_e1734_d_b45, eq136_e1734_d_b46, eq136_e1734_d_b47, eq136_e1734_d_b48, eq136_e1734_d_b49, eq136_e1734_d_b50, eq136_e1734_d_b51, eq136_e1734_d_b52, eq136_e1734_d_b53, eq136_e1734_d_b54, eq136_e1734_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_reactive_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_reactive_branch_derivatives: [f64; 55] = [eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            nodes,
            &eq136_reactive_node_derivatives,
            branches,
            &eq136_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22, eq137_e1747_d_b0, eq137_e1747_d_b1, eq137_e1747_d_b2, eq137_e1747_d_b3, eq137_e1747_d_b4, eq137_e1747_d_b5, eq137_e1747_d_b6, eq137_e1747_d_b7, eq137_e1747_d_b8, eq137_e1747_d_b9, eq137_e1747_d_b10, eq137_e1747_d_b11, eq137_e1747_d_b12, eq137_e1747_d_b13, eq137_e1747_d_b14, eq137_e1747_d_b15, eq137_e1747_d_b16, eq137_e1747_d_b17, eq137_e1747_d_b18, eq137_e1747_d_b19, eq137_e1747_d_b20, eq137_e1747_d_b21, eq137_e1747_d_b22, eq137_e1747_d_b23, eq137_e1747_d_b24, eq137_e1747_d_b25, eq137_e1747_d_b26, eq137_e1747_d_b27, eq137_e1747_d_b28, eq137_e1747_d_b29, eq137_e1747_d_b30, eq137_e1747_d_b31, eq137_e1747_d_b32, eq137_e1747_d_b33, eq137_e1747_d_b34, eq137_e1747_d_b35, eq137_e1747_d_b36, eq137_e1747_d_b37, eq137_e1747_d_b38, eq137_e1747_d_b39, eq137_e1747_d_b40, eq137_e1747_d_b41, eq137_e1747_d_b42, eq137_e1747_d_b43, eq137_e1747_d_b44, eq137_e1747_d_b45, eq137_e1747_d_b46, eq137_e1747_d_b47, eq137_e1747_d_b48, eq137_e1747_d_b49, eq137_e1747_d_b50, eq137_e1747_d_b51, eq137_e1747_d_b52, eq137_e1747_d_b53, eq137_e1747_d_b54, eq137_e1747_q,) = {
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
        let eq137_e1743_d_b0: f64 = (p.p251 * s.db[240][0]);
        let eq137_e1743_d_b1: f64 = (p.p251 * s.db[240][1]);
        let eq137_e1743_d_b2: f64 = (p.p251 * s.db[240][2]);
        let eq137_e1743_d_b3: f64 = (p.p251 * s.db[240][3]);
        let eq137_e1743_d_b4: f64 = (p.p251 * s.db[240][4]);
        let eq137_e1743_d_b5: f64 = (p.p251 * s.db[240][5]);
        let eq137_e1743_d_b6: f64 = (p.p251 * s.db[240][6]);
        let eq137_e1743_d_b7: f64 = (p.p251 * s.db[240][7]);
        let eq137_e1743_d_b8: f64 = (p.p251 * s.db[240][8]);
        let eq137_e1743_d_b9: f64 = (p.p251 * s.db[240][9]);
        let eq137_e1743_d_b10: f64 = (p.p251 * s.db[240][10]);
        let eq137_e1743_d_b11: f64 = (p.p251 * s.db[240][11]);
        let eq137_e1743_d_b12: f64 = (p.p251 * s.db[240][12]);
        let eq137_e1743_d_b13: f64 = (p.p251 * s.db[240][13]);
        let eq137_e1743_d_b14: f64 = (p.p251 * s.db[240][14]);
        let eq137_e1743_d_b15: f64 = (p.p251 * s.db[240][15]);
        let eq137_e1743_d_b16: f64 = (p.p251 * s.db[240][16]);
        let eq137_e1743_d_b17: f64 = (p.p251 * s.db[240][17]);
        let eq137_e1743_d_b18: f64 = (p.p251 * s.db[240][18]);
        let eq137_e1743_d_b19: f64 = (p.p251 * s.db[240][19]);
        let eq137_e1743_d_b20: f64 = (p.p251 * s.db[240][20]);
        let eq137_e1743_d_b21: f64 = (p.p251 * s.db[240][21]);
        let eq137_e1743_d_b22: f64 = (p.p251 * s.db[240][22]);
        let eq137_e1743_d_b23: f64 = (p.p251 * s.db[240][23]);
        let eq137_e1743_d_b24: f64 = (p.p251 * s.db[240][24]);
        let eq137_e1743_d_b25: f64 = (p.p251 * s.db[240][25]);
        let eq137_e1743_d_b26: f64 = (p.p251 * s.db[240][26]);
        let eq137_e1743_d_b27: f64 = (p.p251 * s.db[240][27]);
        let eq137_e1743_d_b28: f64 = (p.p251 * s.db[240][28]);
        let eq137_e1743_d_b29: f64 = (p.p251 * s.db[240][29]);
        let eq137_e1743_d_b30: f64 = (p.p251 * s.db[240][30]);
        let eq137_e1743_d_b31: f64 = (p.p251 * s.db[240][31]);
        let eq137_e1743_d_b32: f64 = (p.p251 * s.db[240][32]);
        let eq137_e1743_d_b33: f64 = (p.p251 * s.db[240][33]);
        let eq137_e1743_d_b34: f64 = (p.p251 * s.db[240][34]);
        let eq137_e1743_d_b35: f64 = (p.p251 * s.db[240][35]);
        let eq137_e1743_d_b36: f64 = (p.p251 * s.db[240][36]);
        let eq137_e1743_d_b37: f64 = (p.p251 * s.db[240][37]);
        let eq137_e1743_d_b38: f64 = (p.p251 * s.db[240][38]);
        let eq137_e1743_d_b39: f64 = (p.p251 * s.db[240][39]);
        let eq137_e1743_d_b40: f64 = (p.p251 * s.db[240][40]);
        let eq137_e1743_d_b41: f64 = (p.p251 * s.db[240][41]);
        let eq137_e1743_d_b42: f64 = (p.p251 * s.db[240][42]);
        let eq137_e1743_d_b43: f64 = (p.p251 * s.db[240][43]);
        let eq137_e1743_d_b44: f64 = (p.p251 * s.db[240][44]);
        let eq137_e1743_d_b45: f64 = (p.p251 * s.db[240][45]);
        let eq137_e1743_d_b46: f64 = (p.p251 * s.db[240][46]);
        let eq137_e1743_d_b47: f64 = (p.p251 * s.db[240][47]);
        let eq137_e1743_d_b48: f64 = (p.p251 * s.db[240][48]);
        let eq137_e1743_d_b49: f64 = (p.p251 * s.db[240][49]);
        let eq137_e1743_d_b50: f64 = (p.p251 * s.db[240][50]);
        let eq137_e1743_d_b51: f64 = (p.p251 * s.db[240][51]);
        let eq137_e1743_d_b52: f64 = (p.p251 * s.db[240][52]);
        let eq137_e1743_d_b53: f64 = (p.p251 * s.db[240][53]);
        let eq137_e1743_d_b54: f64 = (p.p251 * s.db[240][54]);
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
        let eq137_e1745_d_b0: f64 = (p.p7 * eq137_e1743_d_b0);
        let eq137_e1745_d_b1: f64 = (p.p7 * eq137_e1743_d_b1);
        let eq137_e1745_d_b2: f64 = (p.p7 * eq137_e1743_d_b2);
        let eq137_e1745_d_b3: f64 = (p.p7 * eq137_e1743_d_b3);
        let eq137_e1745_d_b4: f64 = (p.p7 * eq137_e1743_d_b4);
        let eq137_e1745_d_b5: f64 = (p.p7 * eq137_e1743_d_b5);
        let eq137_e1745_d_b6: f64 = (p.p7 * eq137_e1743_d_b6);
        let eq137_e1745_d_b7: f64 = (p.p7 * eq137_e1743_d_b7);
        let eq137_e1745_d_b8: f64 = (p.p7 * eq137_e1743_d_b8);
        let eq137_e1745_d_b9: f64 = (p.p7 * eq137_e1743_d_b9);
        let eq137_e1745_d_b10: f64 = (p.p7 * eq137_e1743_d_b10);
        let eq137_e1745_d_b11: f64 = (p.p7 * eq137_e1743_d_b11);
        let eq137_e1745_d_b12: f64 = (p.p7 * eq137_e1743_d_b12);
        let eq137_e1745_d_b13: f64 = (p.p7 * eq137_e1743_d_b13);
        let eq137_e1745_d_b14: f64 = (p.p7 * eq137_e1743_d_b14);
        let eq137_e1745_d_b15: f64 = (p.p7 * eq137_e1743_d_b15);
        let eq137_e1745_d_b16: f64 = (p.p7 * eq137_e1743_d_b16);
        let eq137_e1745_d_b17: f64 = (p.p7 * eq137_e1743_d_b17);
        let eq137_e1745_d_b18: f64 = (p.p7 * eq137_e1743_d_b18);
        let eq137_e1745_d_b19: f64 = (p.p7 * eq137_e1743_d_b19);
        let eq137_e1745_d_b20: f64 = (p.p7 * eq137_e1743_d_b20);
        let eq137_e1745_d_b21: f64 = (p.p7 * eq137_e1743_d_b21);
        let eq137_e1745_d_b22: f64 = (p.p7 * eq137_e1743_d_b22);
        let eq137_e1745_d_b23: f64 = (p.p7 * eq137_e1743_d_b23);
        let eq137_e1745_d_b24: f64 = (p.p7 * eq137_e1743_d_b24);
        let eq137_e1745_d_b25: f64 = (p.p7 * eq137_e1743_d_b25);
        let eq137_e1745_d_b26: f64 = (p.p7 * eq137_e1743_d_b26);
        let eq137_e1745_d_b27: f64 = (p.p7 * eq137_e1743_d_b27);
        let eq137_e1745_d_b28: f64 = (p.p7 * eq137_e1743_d_b28);
        let eq137_e1745_d_b29: f64 = (p.p7 * eq137_e1743_d_b29);
        let eq137_e1745_d_b30: f64 = (p.p7 * eq137_e1743_d_b30);
        let eq137_e1745_d_b31: f64 = (p.p7 * eq137_e1743_d_b31);
        let eq137_e1745_d_b32: f64 = (p.p7 * eq137_e1743_d_b32);
        let eq137_e1745_d_b33: f64 = (p.p7 * eq137_e1743_d_b33);
        let eq137_e1745_d_b34: f64 = (p.p7 * eq137_e1743_d_b34);
        let eq137_e1745_d_b35: f64 = (p.p7 * eq137_e1743_d_b35);
        let eq137_e1745_d_b36: f64 = (p.p7 * eq137_e1743_d_b36);
        let eq137_e1745_d_b37: f64 = (p.p7 * eq137_e1743_d_b37);
        let eq137_e1745_d_b38: f64 = (p.p7 * eq137_e1743_d_b38);
        let eq137_e1745_d_b39: f64 = (p.p7 * eq137_e1743_d_b39);
        let eq137_e1745_d_b40: f64 = (p.p7 * eq137_e1743_d_b40);
        let eq137_e1745_d_b41: f64 = (p.p7 * eq137_e1743_d_b41);
        let eq137_e1745_d_b42: f64 = (p.p7 * eq137_e1743_d_b42);
        let eq137_e1745_d_b43: f64 = (p.p7 * eq137_e1743_d_b43);
        let eq137_e1745_d_b44: f64 = (p.p7 * eq137_e1743_d_b44);
        let eq137_e1745_d_b45: f64 = (p.p7 * eq137_e1743_d_b45);
        let eq137_e1745_d_b46: f64 = (p.p7 * eq137_e1743_d_b46);
        let eq137_e1745_d_b47: f64 = (p.p7 * eq137_e1743_d_b47);
        let eq137_e1745_d_b48: f64 = (p.p7 * eq137_e1743_d_b48);
        let eq137_e1745_d_b49: f64 = (p.p7 * eq137_e1743_d_b49);
        let eq137_e1745_d_b50: f64 = (p.p7 * eq137_e1743_d_b50);
        let eq137_e1745_d_b51: f64 = (p.p7 * eq137_e1743_d_b51);
        let eq137_e1745_d_b52: f64 = (p.p7 * eq137_e1743_d_b52);
        let eq137_e1745_d_b53: f64 = (p.p7 * eq137_e1743_d_b53);
        let eq137_e1745_d_b54: f64 = (p.p7 * eq137_e1743_d_b54);
        let eq137_e1745_q: f64 = (p.p7 * eq137_e1744_q);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22, eq137_e1745_d_b0, eq137_e1745_d_b1, eq137_e1745_d_b2, eq137_e1745_d_b3, eq137_e1745_d_b4, eq137_e1745_d_b5, eq137_e1745_d_b6, eq137_e1745_d_b7, eq137_e1745_d_b8, eq137_e1745_d_b9, eq137_e1745_d_b10, eq137_e1745_d_b11, eq137_e1745_d_b12, eq137_e1745_d_b13, eq137_e1745_d_b14, eq137_e1745_d_b15, eq137_e1745_d_b16, eq137_e1745_d_b17, eq137_e1745_d_b18, eq137_e1745_d_b19, eq137_e1745_d_b20, eq137_e1745_d_b21, eq137_e1745_d_b22, eq137_e1745_d_b23, eq137_e1745_d_b24, eq137_e1745_d_b25, eq137_e1745_d_b26, eq137_e1745_d_b27, eq137_e1745_d_b28, eq137_e1745_d_b29, eq137_e1745_d_b30, eq137_e1745_d_b31, eq137_e1745_d_b32, eq137_e1745_d_b33, eq137_e1745_d_b34, eq137_e1745_d_b35, eq137_e1745_d_b36, eq137_e1745_d_b37, eq137_e1745_d_b38, eq137_e1745_d_b39, eq137_e1745_d_b40, eq137_e1745_d_b41, eq137_e1745_d_b42, eq137_e1745_d_b43, eq137_e1745_d_b44, eq137_e1745_d_b45, eq137_e1745_d_b46, eq137_e1745_d_b47, eq137_e1745_d_b48, eq137_e1745_d_b49, eq137_e1745_d_b50, eq137_e1745_d_b51, eq137_e1745_d_b52, eq137_e1745_d_b53, eq137_e1745_d_b54, eq137_e1745_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_reactive_node_derivatives: [f64; 23] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_reactive_branch_derivatives: [f64; 55] = [eq137_e1747_d_b0, eq137_e1747_d_b1, eq137_e1747_d_b2, eq137_e1747_d_b3, eq137_e1747_d_b4, eq137_e1747_d_b5, eq137_e1747_d_b6, eq137_e1747_d_b7, eq137_e1747_d_b8, eq137_e1747_d_b9, eq137_e1747_d_b10, eq137_e1747_d_b11, eq137_e1747_d_b12, eq137_e1747_d_b13, eq137_e1747_d_b14, eq137_e1747_d_b15, eq137_e1747_d_b16, eq137_e1747_d_b17, eq137_e1747_d_b18, eq137_e1747_d_b19, eq137_e1747_d_b20, eq137_e1747_d_b21, eq137_e1747_d_b22, eq137_e1747_d_b23, eq137_e1747_d_b24, eq137_e1747_d_b25, eq137_e1747_d_b26, eq137_e1747_d_b27, eq137_e1747_d_b28, eq137_e1747_d_b29, eq137_e1747_d_b30, eq137_e1747_d_b31, eq137_e1747_d_b32, eq137_e1747_d_b33, eq137_e1747_d_b34, eq137_e1747_d_b35, eq137_e1747_d_b36, eq137_e1747_d_b37, eq137_e1747_d_b38, eq137_e1747_d_b39, eq137_e1747_d_b40, eq137_e1747_d_b41, eq137_e1747_d_b42, eq137_e1747_d_b43, eq137_e1747_d_b44, eq137_e1747_d_b45, eq137_e1747_d_b46, eq137_e1747_d_b47, eq137_e1747_d_b48, eq137_e1747_d_b49, eq137_e1747_d_b50, eq137_e1747_d_b51, eq137_e1747_d_b52, eq137_e1747_d_b53, eq137_e1747_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[19]),
            nodes,
            &eq137_reactive_node_derivatives,
            branches,
            &eq137_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22, eq138_e1757_d_b0, eq138_e1757_d_b1, eq138_e1757_d_b2, eq138_e1757_d_b3, eq138_e1757_d_b4, eq138_e1757_d_b5, eq138_e1757_d_b6, eq138_e1757_d_b7, eq138_e1757_d_b8, eq138_e1757_d_b9, eq138_e1757_d_b10, eq138_e1757_d_b11, eq138_e1757_d_b12, eq138_e1757_d_b13, eq138_e1757_d_b14, eq138_e1757_d_b15, eq138_e1757_d_b16, eq138_e1757_d_b17, eq138_e1757_d_b18, eq138_e1757_d_b19, eq138_e1757_d_b20, eq138_e1757_d_b21, eq138_e1757_d_b22, eq138_e1757_d_b23, eq138_e1757_d_b24, eq138_e1757_d_b25, eq138_e1757_d_b26, eq138_e1757_d_b27, eq138_e1757_d_b28, eq138_e1757_d_b29, eq138_e1757_d_b30, eq138_e1757_d_b31, eq138_e1757_d_b32, eq138_e1757_d_b33, eq138_e1757_d_b34, eq138_e1757_d_b35, eq138_e1757_d_b36, eq138_e1757_d_b37, eq138_e1757_d_b38, eq138_e1757_d_b39, eq138_e1757_d_b40, eq138_e1757_d_b41, eq138_e1757_d_b42, eq138_e1757_d_b43, eq138_e1757_d_b44, eq138_e1757_d_b45, eq138_e1757_d_b46, eq138_e1757_d_b47, eq138_e1757_d_b48, eq138_e1757_d_b49, eq138_e1757_d_b50, eq138_e1757_d_b51, eq138_e1757_d_b52, eq138_e1757_d_b53, eq138_e1757_d_b54, eq138_e1757_q,) = {
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
        let eq138_e1755_d_b0: f64 = (p.p7 * s.db[241][0]);
        let eq138_e1755_d_b1: f64 = (p.p7 * s.db[241][1]);
        let eq138_e1755_d_b2: f64 = (p.p7 * s.db[241][2]);
        let eq138_e1755_d_b3: f64 = (p.p7 * s.db[241][3]);
        let eq138_e1755_d_b4: f64 = (p.p7 * s.db[241][4]);
        let eq138_e1755_d_b5: f64 = (p.p7 * s.db[241][5]);
        let eq138_e1755_d_b6: f64 = (p.p7 * s.db[241][6]);
        let eq138_e1755_d_b7: f64 = (p.p7 * s.db[241][7]);
        let eq138_e1755_d_b8: f64 = (p.p7 * s.db[241][8]);
        let eq138_e1755_d_b9: f64 = (p.p7 * s.db[241][9]);
        let eq138_e1755_d_b10: f64 = (p.p7 * s.db[241][10]);
        let eq138_e1755_d_b11: f64 = (p.p7 * s.db[241][11]);
        let eq138_e1755_d_b12: f64 = (p.p7 * s.db[241][12]);
        let eq138_e1755_d_b13: f64 = (p.p7 * s.db[241][13]);
        let eq138_e1755_d_b14: f64 = (p.p7 * s.db[241][14]);
        let eq138_e1755_d_b15: f64 = (p.p7 * s.db[241][15]);
        let eq138_e1755_d_b16: f64 = (p.p7 * s.db[241][16]);
        let eq138_e1755_d_b17: f64 = (p.p7 * s.db[241][17]);
        let eq138_e1755_d_b18: f64 = (p.p7 * s.db[241][18]);
        let eq138_e1755_d_b19: f64 = (p.p7 * s.db[241][19]);
        let eq138_e1755_d_b20: f64 = (p.p7 * s.db[241][20]);
        let eq138_e1755_d_b21: f64 = (p.p7 * s.db[241][21]);
        let eq138_e1755_d_b22: f64 = (p.p7 * s.db[241][22]);
        let eq138_e1755_d_b23: f64 = (p.p7 * s.db[241][23]);
        let eq138_e1755_d_b24: f64 = (p.p7 * s.db[241][24]);
        let eq138_e1755_d_b25: f64 = (p.p7 * s.db[241][25]);
        let eq138_e1755_d_b26: f64 = (p.p7 * s.db[241][26]);
        let eq138_e1755_d_b27: f64 = (p.p7 * s.db[241][27]);
        let eq138_e1755_d_b28: f64 = (p.p7 * s.db[241][28]);
        let eq138_e1755_d_b29: f64 = (p.p7 * s.db[241][29]);
        let eq138_e1755_d_b30: f64 = (p.p7 * s.db[241][30]);
        let eq138_e1755_d_b31: f64 = (p.p7 * s.db[241][31]);
        let eq138_e1755_d_b32: f64 = (p.p7 * s.db[241][32]);
        let eq138_e1755_d_b33: f64 = (p.p7 * s.db[241][33]);
        let eq138_e1755_d_b34: f64 = (p.p7 * s.db[241][34]);
        let eq138_e1755_d_b35: f64 = (p.p7 * s.db[241][35]);
        let eq138_e1755_d_b36: f64 = (p.p7 * s.db[241][36]);
        let eq138_e1755_d_b37: f64 = (p.p7 * s.db[241][37]);
        let eq138_e1755_d_b38: f64 = (p.p7 * s.db[241][38]);
        let eq138_e1755_d_b39: f64 = (p.p7 * s.db[241][39]);
        let eq138_e1755_d_b40: f64 = (p.p7 * s.db[241][40]);
        let eq138_e1755_d_b41: f64 = (p.p7 * s.db[241][41]);
        let eq138_e1755_d_b42: f64 = (p.p7 * s.db[241][42]);
        let eq138_e1755_d_b43: f64 = (p.p7 * s.db[241][43]);
        let eq138_e1755_d_b44: f64 = (p.p7 * s.db[241][44]);
        let eq138_e1755_d_b45: f64 = (p.p7 * s.db[241][45]);
        let eq138_e1755_d_b46: f64 = (p.p7 * s.db[241][46]);
        let eq138_e1755_d_b47: f64 = (p.p7 * s.db[241][47]);
        let eq138_e1755_d_b48: f64 = (p.p7 * s.db[241][48]);
        let eq138_e1755_d_b49: f64 = (p.p7 * s.db[241][49]);
        let eq138_e1755_d_b50: f64 = (p.p7 * s.db[241][50]);
        let eq138_e1755_d_b51: f64 = (p.p7 * s.db[241][51]);
        let eq138_e1755_d_b52: f64 = (p.p7 * s.db[241][52]);
        let eq138_e1755_d_b53: f64 = (p.p7 * s.db[241][53]);
        let eq138_e1755_d_b54: f64 = (p.p7 * s.db[241][54]);
        let eq138_e1755_q: f64 = (p.p7 * eq138_e1754_q);
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22, eq138_e1755_d_b0, eq138_e1755_d_b1, eq138_e1755_d_b2, eq138_e1755_d_b3, eq138_e1755_d_b4, eq138_e1755_d_b5, eq138_e1755_d_b6, eq138_e1755_d_b7, eq138_e1755_d_b8, eq138_e1755_d_b9, eq138_e1755_d_b10, eq138_e1755_d_b11, eq138_e1755_d_b12, eq138_e1755_d_b13, eq138_e1755_d_b14, eq138_e1755_d_b15, eq138_e1755_d_b16, eq138_e1755_d_b17, eq138_e1755_d_b18, eq138_e1755_d_b19, eq138_e1755_d_b20, eq138_e1755_d_b21, eq138_e1755_d_b22, eq138_e1755_d_b23, eq138_e1755_d_b24, eq138_e1755_d_b25, eq138_e1755_d_b26, eq138_e1755_d_b27, eq138_e1755_d_b28, eq138_e1755_d_b29, eq138_e1755_d_b30, eq138_e1755_d_b31, eq138_e1755_d_b32, eq138_e1755_d_b33, eq138_e1755_d_b34, eq138_e1755_d_b35, eq138_e1755_d_b36, eq138_e1755_d_b37, eq138_e1755_d_b38, eq138_e1755_d_b39, eq138_e1755_d_b40, eq138_e1755_d_b41, eq138_e1755_d_b42, eq138_e1755_d_b43, eq138_e1755_d_b44, eq138_e1755_d_b45, eq138_e1755_d_b46, eq138_e1755_d_b47, eq138_e1755_d_b48, eq138_e1755_d_b49, eq138_e1755_d_b50, eq138_e1755_d_b51, eq138_e1755_d_b52, eq138_e1755_d_b53, eq138_e1755_d_b54, eq138_e1755_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_reactive_node_derivatives: [f64; 23] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_reactive_branch_derivatives: [f64; 55] = [eq138_e1757_d_b0, eq138_e1757_d_b1, eq138_e1757_d_b2, eq138_e1757_d_b3, eq138_e1757_d_b4, eq138_e1757_d_b5, eq138_e1757_d_b6, eq138_e1757_d_b7, eq138_e1757_d_b8, eq138_e1757_d_b9, eq138_e1757_d_b10, eq138_e1757_d_b11, eq138_e1757_d_b12, eq138_e1757_d_b13, eq138_e1757_d_b14, eq138_e1757_d_b15, eq138_e1757_d_b16, eq138_e1757_d_b17, eq138_e1757_d_b18, eq138_e1757_d_b19, eq138_e1757_d_b20, eq138_e1757_d_b21, eq138_e1757_d_b22, eq138_e1757_d_b23, eq138_e1757_d_b24, eq138_e1757_d_b25, eq138_e1757_d_b26, eq138_e1757_d_b27, eq138_e1757_d_b28, eq138_e1757_d_b29, eq138_e1757_d_b30, eq138_e1757_d_b31, eq138_e1757_d_b32, eq138_e1757_d_b33, eq138_e1757_d_b34, eq138_e1757_d_b35, eq138_e1757_d_b36, eq138_e1757_d_b37, eq138_e1757_d_b38, eq138_e1757_d_b39, eq138_e1757_d_b40, eq138_e1757_d_b41, eq138_e1757_d_b42, eq138_e1757_d_b43, eq138_e1757_d_b44, eq138_e1757_d_b45, eq138_e1757_d_b46, eq138_e1757_d_b47, eq138_e1757_d_b48, eq138_e1757_d_b49, eq138_e1757_d_b50, eq138_e1757_d_b51, eq138_e1757_d_b52, eq138_e1757_d_b53, eq138_e1757_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            nodes,
            &eq138_reactive_node_derivatives,
            branches,
            &eq138_reactive_branch_derivatives,
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[240][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[240][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[240][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[240][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[240][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[240][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[240][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[240][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[240][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[240][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[240][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[240][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[240][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[240][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[240][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[240][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[240][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[240][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[240][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[240][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[240][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[240][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[240][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[240][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[240][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[240][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[240][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[240][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[240][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[240][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[240][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[240][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[240][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[240][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[240][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[240][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[240][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[240][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[240][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[240][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[240][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[240][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[240][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[240][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[240][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[240][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[240][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[240][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[240][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[240][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[240][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[240][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[240][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[240][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[240][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[240][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[240][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[240][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[240][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[240][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[240][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[240][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[240][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[240][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[240][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[240][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[240][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[240][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[240][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[240][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[240][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[240][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[240][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[240][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[240][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[240][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[240][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[240][54]);
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22, eq139_e1769_d_b0, eq139_e1769_d_b1, eq139_e1769_d_b2, eq139_e1769_d_b3, eq139_e1769_d_b4, eq139_e1769_d_b5, eq139_e1769_d_b6, eq139_e1769_d_b7, eq139_e1769_d_b8, eq139_e1769_d_b9, eq139_e1769_d_b10, eq139_e1769_d_b11, eq139_e1769_d_b12, eq139_e1769_d_b13, eq139_e1769_d_b14, eq139_e1769_d_b15, eq139_e1769_d_b16, eq139_e1769_d_b17, eq139_e1769_d_b18, eq139_e1769_d_b19, eq139_e1769_d_b20, eq139_e1769_d_b21, eq139_e1769_d_b22, eq139_e1769_d_b23, eq139_e1769_d_b24, eq139_e1769_d_b25, eq139_e1769_d_b26, eq139_e1769_d_b27, eq139_e1769_d_b28, eq139_e1769_d_b29, eq139_e1769_d_b30, eq139_e1769_d_b31, eq139_e1769_d_b32, eq139_e1769_d_b33, eq139_e1769_d_b34, eq139_e1769_d_b35, eq139_e1769_d_b36, eq139_e1769_d_b37, eq139_e1769_d_b38, eq139_e1769_d_b39, eq139_e1769_d_b40, eq139_e1769_d_b41, eq139_e1769_d_b42, eq139_e1769_d_b43, eq139_e1769_d_b44, eq139_e1769_d_b45, eq139_e1769_d_b46, eq139_e1769_d_b47, eq139_e1769_d_b48, eq139_e1769_d_b49, eq139_e1769_d_b50, eq139_e1769_d_b51, eq139_e1769_d_b52, eq139_e1769_d_b53, eq139_e1769_d_b54, eq139_e1769_q,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq139_e1766_q: f64 = s.v[240];
        let eq139_e1767: f64 = (p.p7 * s.v[240]);
        let eq139_e1767_q: f64 = (p.p7 * eq139_e1766_q);
        (eq139_e1767, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq139_e1767_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_reactive_node_derivatives: [f64; 23] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_reactive_branch_derivatives: [f64; 55] = [eq139_e1769_d_b0, eq139_e1769_d_b1, eq139_e1769_d_b2, eq139_e1769_d_b3, eq139_e1769_d_b4, eq139_e1769_d_b5, eq139_e1769_d_b6, eq139_e1769_d_b7, eq139_e1769_d_b8, eq139_e1769_d_b9, eq139_e1769_d_b10, eq139_e1769_d_b11, eq139_e1769_d_b12, eq139_e1769_d_b13, eq139_e1769_d_b14, eq139_e1769_d_b15, eq139_e1769_d_b16, eq139_e1769_d_b17, eq139_e1769_d_b18, eq139_e1769_d_b19, eq139_e1769_d_b20, eq139_e1769_d_b21, eq139_e1769_d_b22, eq139_e1769_d_b23, eq139_e1769_d_b24, eq139_e1769_d_b25, eq139_e1769_d_b26, eq139_e1769_d_b27, eq139_e1769_d_b28, eq139_e1769_d_b29, eq139_e1769_d_b30, eq139_e1769_d_b31, eq139_e1769_d_b32, eq139_e1769_d_b33, eq139_e1769_d_b34, eq139_e1769_d_b35, eq139_e1769_d_b36, eq139_e1769_d_b37, eq139_e1769_d_b38, eq139_e1769_d_b39, eq139_e1769_d_b40, eq139_e1769_d_b41, eq139_e1769_d_b42, eq139_e1769_d_b43, eq139_e1769_d_b44, eq139_e1769_d_b45, eq139_e1769_d_b46, eq139_e1769_d_b47, eq139_e1769_d_b48, eq139_e1769_d_b49, eq139_e1769_d_b50, eq139_e1769_d_b51, eq139_e1769_d_b52, eq139_e1769_d_b53, eq139_e1769_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq139_reactive_node_derivatives,
            branches,
            &eq139_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22, eq140_e1783_d_b0, eq140_e1783_d_b1, eq140_e1783_d_b2, eq140_e1783_d_b3, eq140_e1783_d_b4, eq140_e1783_d_b5, eq140_e1783_d_b6, eq140_e1783_d_b7, eq140_e1783_d_b8, eq140_e1783_d_b9, eq140_e1783_d_b10, eq140_e1783_d_b11, eq140_e1783_d_b12, eq140_e1783_d_b13, eq140_e1783_d_b14, eq140_e1783_d_b15, eq140_e1783_d_b16, eq140_e1783_d_b17, eq140_e1783_d_b18, eq140_e1783_d_b19, eq140_e1783_d_b20, eq140_e1783_d_b21, eq140_e1783_d_b22, eq140_e1783_d_b23, eq140_e1783_d_b24, eq140_e1783_d_b25, eq140_e1783_d_b26, eq140_e1783_d_b27, eq140_e1783_d_b28, eq140_e1783_d_b29, eq140_e1783_d_b30, eq140_e1783_d_b31, eq140_e1783_d_b32, eq140_e1783_d_b33, eq140_e1783_d_b34, eq140_e1783_d_b35, eq140_e1783_d_b36, eq140_e1783_d_b37, eq140_e1783_d_b38, eq140_e1783_d_b39, eq140_e1783_d_b40, eq140_e1783_d_b41, eq140_e1783_d_b42, eq140_e1783_d_b43, eq140_e1783_d_b44, eq140_e1783_d_b45, eq140_e1783_d_b46, eq140_e1783_d_b47, eq140_e1783_d_b48, eq140_e1783_d_b49, eq140_e1783_d_b50, eq140_e1783_d_b51, eq140_e1783_d_b52, eq140_e1783_d_b53, eq140_e1783_d_b54, eq140_e1783_q,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq140_e1778_q: f64 = s.v[240];
        let eq140_e1779: f64 = (p.p7 * s.v[240]);
        let eq140_e1779_q: f64 = (p.p7 * eq140_e1778_q);
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (__rspice_deriv_cse_0 * p.p246);
        let eq140_e1781_d_n1: f64 = (__rspice_deriv_cse_1 * p.p246);
        let eq140_e1781_d_n2: f64 = (__rspice_deriv_cse_2 * p.p246);
        let eq140_e1781_d_n3: f64 = (__rspice_deriv_cse_3 * p.p246);
        let eq140_e1781_d_n4: f64 = (__rspice_deriv_cse_4 * p.p246);
        let eq140_e1781_d_n5: f64 = (__rspice_deriv_cse_5 * p.p246);
        let eq140_e1781_d_n6: f64 = (__rspice_deriv_cse_6 * p.p246);
        let eq140_e1781_d_n7: f64 = (__rspice_deriv_cse_7 * p.p246);
        let eq140_e1781_d_n8: f64 = (__rspice_deriv_cse_8 * p.p246);
        let eq140_e1781_d_n9: f64 = (__rspice_deriv_cse_9 * p.p246);
        let eq140_e1781_d_n10: f64 = (__rspice_deriv_cse_10 * p.p246);
        let eq140_e1781_d_n11: f64 = (__rspice_deriv_cse_11 * p.p246);
        let eq140_e1781_d_n12: f64 = (__rspice_deriv_cse_12 * p.p246);
        let eq140_e1781_d_n13: f64 = (__rspice_deriv_cse_13 * p.p246);
        let eq140_e1781_d_n14: f64 = (__rspice_deriv_cse_14 * p.p246);
        let eq140_e1781_d_n15: f64 = (__rspice_deriv_cse_15 * p.p246);
        let eq140_e1781_d_n16: f64 = (__rspice_deriv_cse_16 * p.p246);
        let eq140_e1781_d_n17: f64 = (__rspice_deriv_cse_17 * p.p246);
        let eq140_e1781_d_n18: f64 = (__rspice_deriv_cse_18 * p.p246);
        let eq140_e1781_d_n19: f64 = (__rspice_deriv_cse_19 * p.p246);
        let eq140_e1781_d_n20: f64 = (__rspice_deriv_cse_20 * p.p246);
        let eq140_e1781_d_n21: f64 = (__rspice_deriv_cse_21 * p.p246);
        let eq140_e1781_d_n22: f64 = (__rspice_deriv_cse_22 * p.p246);
        let eq140_e1781_d_b0: f64 = (__rspice_deriv_cse_23 * p.p246);
        let eq140_e1781_d_b1: f64 = (__rspice_deriv_cse_24 * p.p246);
        let eq140_e1781_d_b2: f64 = (__rspice_deriv_cse_25 * p.p246);
        let eq140_e1781_d_b3: f64 = (__rspice_deriv_cse_26 * p.p246);
        let eq140_e1781_d_b4: f64 = (__rspice_deriv_cse_27 * p.p246);
        let eq140_e1781_d_b5: f64 = (__rspice_deriv_cse_28 * p.p246);
        let eq140_e1781_d_b6: f64 = (__rspice_deriv_cse_29 * p.p246);
        let eq140_e1781_d_b7: f64 = (__rspice_deriv_cse_30 * p.p246);
        let eq140_e1781_d_b8: f64 = (__rspice_deriv_cse_31 * p.p246);
        let eq140_e1781_d_b9: f64 = (__rspice_deriv_cse_32 * p.p246);
        let eq140_e1781_d_b10: f64 = (__rspice_deriv_cse_33 * p.p246);
        let eq140_e1781_d_b11: f64 = (__rspice_deriv_cse_34 * p.p246);
        let eq140_e1781_d_b12: f64 = (__rspice_deriv_cse_35 * p.p246);
        let eq140_e1781_d_b13: f64 = (__rspice_deriv_cse_36 * p.p246);
        let eq140_e1781_d_b14: f64 = (__rspice_deriv_cse_37 * p.p246);
        let eq140_e1781_d_b15: f64 = (__rspice_deriv_cse_38 * p.p246);
        let eq140_e1781_d_b16: f64 = (__rspice_deriv_cse_39 * p.p246);
        let eq140_e1781_d_b17: f64 = (__rspice_deriv_cse_40 * p.p246);
        let eq140_e1781_d_b18: f64 = (__rspice_deriv_cse_41 * p.p246);
        let eq140_e1781_d_b19: f64 = (__rspice_deriv_cse_42 * p.p246);
        let eq140_e1781_d_b20: f64 = (__rspice_deriv_cse_43 * p.p246);
        let eq140_e1781_d_b21: f64 = (__rspice_deriv_cse_44 * p.p246);
        let eq140_e1781_d_b22: f64 = (__rspice_deriv_cse_45 * p.p246);
        let eq140_e1781_d_b23: f64 = (__rspice_deriv_cse_46 * p.p246);
        let eq140_e1781_d_b24: f64 = (__rspice_deriv_cse_47 * p.p246);
        let eq140_e1781_d_b25: f64 = (__rspice_deriv_cse_48 * p.p246);
        let eq140_e1781_d_b26: f64 = (__rspice_deriv_cse_49 * p.p246);
        let eq140_e1781_d_b27: f64 = (__rspice_deriv_cse_50 * p.p246);
        let eq140_e1781_d_b28: f64 = (__rspice_deriv_cse_51 * p.p246);
        let eq140_e1781_d_b29: f64 = (__rspice_deriv_cse_52 * p.p246);
        let eq140_e1781_d_b30: f64 = (__rspice_deriv_cse_53 * p.p246);
        let eq140_e1781_d_b31: f64 = (__rspice_deriv_cse_54 * p.p246);
        let eq140_e1781_d_b32: f64 = (__rspice_deriv_cse_55 * p.p246);
        let eq140_e1781_d_b33: f64 = (__rspice_deriv_cse_56 * p.p246);
        let eq140_e1781_d_b34: f64 = (__rspice_deriv_cse_57 * p.p246);
        let eq140_e1781_d_b35: f64 = (__rspice_deriv_cse_58 * p.p246);
        let eq140_e1781_d_b36: f64 = (__rspice_deriv_cse_59 * p.p246);
        let eq140_e1781_d_b37: f64 = (__rspice_deriv_cse_60 * p.p246);
        let eq140_e1781_d_b38: f64 = (__rspice_deriv_cse_61 * p.p246);
        let eq140_e1781_d_b39: f64 = (__rspice_deriv_cse_62 * p.p246);
        let eq140_e1781_d_b40: f64 = (__rspice_deriv_cse_63 * p.p246);
        let eq140_e1781_d_b41: f64 = (__rspice_deriv_cse_64 * p.p246);
        let eq140_e1781_d_b42: f64 = (__rspice_deriv_cse_65 * p.p246);
        let eq140_e1781_d_b43: f64 = (__rspice_deriv_cse_66 * p.p246);
        let eq140_e1781_d_b44: f64 = (__rspice_deriv_cse_67 * p.p246);
        let eq140_e1781_d_b45: f64 = (__rspice_deriv_cse_68 * p.p246);
        let eq140_e1781_d_b46: f64 = (__rspice_deriv_cse_69 * p.p246);
        let eq140_e1781_d_b47: f64 = (__rspice_deriv_cse_70 * p.p246);
        let eq140_e1781_d_b48: f64 = (__rspice_deriv_cse_71 * p.p246);
        let eq140_e1781_d_b49: f64 = (__rspice_deriv_cse_72 * p.p246);
        let eq140_e1781_d_b50: f64 = (__rspice_deriv_cse_73 * p.p246);
        let eq140_e1781_d_b51: f64 = (__rspice_deriv_cse_74 * p.p246);
        let eq140_e1781_d_b52: f64 = (__rspice_deriv_cse_75 * p.p246);
        let eq140_e1781_d_b53: f64 = (__rspice_deriv_cse_76 * p.p246);
        let eq140_e1781_d_b54: f64 = (__rspice_deriv_cse_77 * p.p246);
        let eq140_e1781_q: f64 = (eq140_e1779_q * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22, eq140_e1781_d_b0, eq140_e1781_d_b1, eq140_e1781_d_b2, eq140_e1781_d_b3, eq140_e1781_d_b4, eq140_e1781_d_b5, eq140_e1781_d_b6, eq140_e1781_d_b7, eq140_e1781_d_b8, eq140_e1781_d_b9, eq140_e1781_d_b10, eq140_e1781_d_b11, eq140_e1781_d_b12, eq140_e1781_d_b13, eq140_e1781_d_b14, eq140_e1781_d_b15, eq140_e1781_d_b16, eq140_e1781_d_b17, eq140_e1781_d_b18, eq140_e1781_d_b19, eq140_e1781_d_b20, eq140_e1781_d_b21, eq140_e1781_d_b22, eq140_e1781_d_b23, eq140_e1781_d_b24, eq140_e1781_d_b25, eq140_e1781_d_b26, eq140_e1781_d_b27, eq140_e1781_d_b28, eq140_e1781_d_b29, eq140_e1781_d_b30, eq140_e1781_d_b31, eq140_e1781_d_b32, eq140_e1781_d_b33, eq140_e1781_d_b34, eq140_e1781_d_b35, eq140_e1781_d_b36, eq140_e1781_d_b37, eq140_e1781_d_b38, eq140_e1781_d_b39, eq140_e1781_d_b40, eq140_e1781_d_b41, eq140_e1781_d_b42, eq140_e1781_d_b43, eq140_e1781_d_b44, eq140_e1781_d_b45, eq140_e1781_d_b46, eq140_e1781_d_b47, eq140_e1781_d_b48, eq140_e1781_d_b49, eq140_e1781_d_b50, eq140_e1781_d_b51, eq140_e1781_d_b52, eq140_e1781_d_b53, eq140_e1781_d_b54, eq140_e1781_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_reactive_node_derivatives: [f64; 23] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_reactive_branch_derivatives: [f64; 55] = [eq140_e1783_d_b0, eq140_e1783_d_b1, eq140_e1783_d_b2, eq140_e1783_d_b3, eq140_e1783_d_b4, eq140_e1783_d_b5, eq140_e1783_d_b6, eq140_e1783_d_b7, eq140_e1783_d_b8, eq140_e1783_d_b9, eq140_e1783_d_b10, eq140_e1783_d_b11, eq140_e1783_d_b12, eq140_e1783_d_b13, eq140_e1783_d_b14, eq140_e1783_d_b15, eq140_e1783_d_b16, eq140_e1783_d_b17, eq140_e1783_d_b18, eq140_e1783_d_b19, eq140_e1783_d_b20, eq140_e1783_d_b21, eq140_e1783_d_b22, eq140_e1783_d_b23, eq140_e1783_d_b24, eq140_e1783_d_b25, eq140_e1783_d_b26, eq140_e1783_d_b27, eq140_e1783_d_b28, eq140_e1783_d_b29, eq140_e1783_d_b30, eq140_e1783_d_b31, eq140_e1783_d_b32, eq140_e1783_d_b33, eq140_e1783_d_b34, eq140_e1783_d_b35, eq140_e1783_d_b36, eq140_e1783_d_b37, eq140_e1783_d_b38, eq140_e1783_d_b39, eq140_e1783_d_b40, eq140_e1783_d_b41, eq140_e1783_d_b42, eq140_e1783_d_b43, eq140_e1783_d_b44, eq140_e1783_d_b45, eq140_e1783_d_b46, eq140_e1783_d_b47, eq140_e1783_d_b48, eq140_e1783_d_b49, eq140_e1783_d_b50, eq140_e1783_d_b51, eq140_e1783_d_b52, eq140_e1783_d_b53, eq140_e1783_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq140_reactive_node_derivatives,
            branches,
            &eq140_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22, eq141_e1796_d_b0, eq141_e1796_d_b1, eq141_e1796_d_b2, eq141_e1796_d_b3, eq141_e1796_d_b4, eq141_e1796_d_b5, eq141_e1796_d_b6, eq141_e1796_d_b7, eq141_e1796_d_b8, eq141_e1796_d_b9, eq141_e1796_d_b10, eq141_e1796_d_b11, eq141_e1796_d_b12, eq141_e1796_d_b13, eq141_e1796_d_b14, eq141_e1796_d_b15, eq141_e1796_d_b16, eq141_e1796_d_b17, eq141_e1796_d_b18, eq141_e1796_d_b19, eq141_e1796_d_b20, eq141_e1796_d_b21, eq141_e1796_d_b22, eq141_e1796_d_b23, eq141_e1796_d_b24, eq141_e1796_d_b25, eq141_e1796_d_b26, eq141_e1796_d_b27, eq141_e1796_d_b28, eq141_e1796_d_b29, eq141_e1796_d_b30, eq141_e1796_d_b31, eq141_e1796_d_b32, eq141_e1796_d_b33, eq141_e1796_d_b34, eq141_e1796_d_b35, eq141_e1796_d_b36, eq141_e1796_d_b37, eq141_e1796_d_b38, eq141_e1796_d_b39, eq141_e1796_d_b40, eq141_e1796_d_b41, eq141_e1796_d_b42, eq141_e1796_d_b43, eq141_e1796_d_b44, eq141_e1796_d_b45, eq141_e1796_d_b46, eq141_e1796_d_b47, eq141_e1796_d_b48, eq141_e1796_d_b49, eq141_e1796_d_b50, eq141_e1796_d_b51, eq141_e1796_d_b52, eq141_e1796_d_b53, eq141_e1796_d_b54, eq141_e1796_q,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq141_e1793_q: f64 = s.v[240];
        let eq141_e1794: f64 = (p.p7 * s.v[240]);
        let eq141_e1794_q: f64 = (p.p7 * eq141_e1793_q);
        (eq141_e1794, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq141_e1794_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_reactive_node_derivatives: [f64; 23] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_reactive_branch_derivatives: [f64; 55] = [eq141_e1796_d_b0, eq141_e1796_d_b1, eq141_e1796_d_b2, eq141_e1796_d_b3, eq141_e1796_d_b4, eq141_e1796_d_b5, eq141_e1796_d_b6, eq141_e1796_d_b7, eq141_e1796_d_b8, eq141_e1796_d_b9, eq141_e1796_d_b10, eq141_e1796_d_b11, eq141_e1796_d_b12, eq141_e1796_d_b13, eq141_e1796_d_b14, eq141_e1796_d_b15, eq141_e1796_d_b16, eq141_e1796_d_b17, eq141_e1796_d_b18, eq141_e1796_d_b19, eq141_e1796_d_b20, eq141_e1796_d_b21, eq141_e1796_d_b22, eq141_e1796_d_b23, eq141_e1796_d_b24, eq141_e1796_d_b25, eq141_e1796_d_b26, eq141_e1796_d_b27, eq141_e1796_d_b28, eq141_e1796_d_b29, eq141_e1796_d_b30, eq141_e1796_d_b31, eq141_e1796_d_b32, eq141_e1796_d_b33, eq141_e1796_d_b34, eq141_e1796_d_b35, eq141_e1796_d_b36, eq141_e1796_d_b37, eq141_e1796_d_b38, eq141_e1796_d_b39, eq141_e1796_d_b40, eq141_e1796_d_b41, eq141_e1796_d_b42, eq141_e1796_d_b43, eq141_e1796_d_b44, eq141_e1796_d_b45, eq141_e1796_d_b46, eq141_e1796_d_b47, eq141_e1796_d_b48, eq141_e1796_d_b49, eq141_e1796_d_b50, eq141_e1796_d_b51, eq141_e1796_d_b52, eq141_e1796_d_b53, eq141_e1796_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq141_reactive_node_derivatives,
            branches,
            &eq141_reactive_branch_derivatives,
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
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22, eq142_e1811_d_b0, eq142_e1811_d_b1, eq142_e1811_d_b2, eq142_e1811_d_b3, eq142_e1811_d_b4, eq142_e1811_d_b5, eq142_e1811_d_b6, eq142_e1811_d_b7, eq142_e1811_d_b8, eq142_e1811_d_b9, eq142_e1811_d_b10, eq142_e1811_d_b11, eq142_e1811_d_b12, eq142_e1811_d_b13, eq142_e1811_d_b14, eq142_e1811_d_b15, eq142_e1811_d_b16, eq142_e1811_d_b17, eq142_e1811_d_b18, eq142_e1811_d_b19, eq142_e1811_d_b20, eq142_e1811_d_b21, eq142_e1811_d_b22, eq142_e1811_d_b23, eq142_e1811_d_b24, eq142_e1811_d_b25, eq142_e1811_d_b26, eq142_e1811_d_b27, eq142_e1811_d_b28, eq142_e1811_d_b29, eq142_e1811_d_b30, eq142_e1811_d_b31, eq142_e1811_d_b32, eq142_e1811_d_b33, eq142_e1811_d_b34, eq142_e1811_d_b35, eq142_e1811_d_b36, eq142_e1811_d_b37, eq142_e1811_d_b38, eq142_e1811_d_b39, eq142_e1811_d_b40, eq142_e1811_d_b41, eq142_e1811_d_b42, eq142_e1811_d_b43, eq142_e1811_d_b44, eq142_e1811_d_b45, eq142_e1811_d_b46, eq142_e1811_d_b47, eq142_e1811_d_b48, eq142_e1811_d_b49, eq142_e1811_d_b50, eq142_e1811_d_b51, eq142_e1811_d_b52, eq142_e1811_d_b53, eq142_e1811_d_b54, eq142_e1811_q,) = {
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
        let eq142_e1807_d_b0: f64 = (p.p7 * s.db[240][0]);
        let eq142_e1807_d_b1: f64 = (p.p7 * s.db[240][1]);
        let eq142_e1807_d_b2: f64 = (p.p7 * s.db[240][2]);
        let eq142_e1807_d_b3: f64 = (p.p7 * s.db[240][3]);
        let eq142_e1807_d_b4: f64 = (p.p7 * s.db[240][4]);
        let eq142_e1807_d_b5: f64 = (p.p7 * s.db[240][5]);
        let eq142_e1807_d_b6: f64 = (p.p7 * s.db[240][6]);
        let eq142_e1807_d_b7: f64 = (p.p7 * s.db[240][7]);
        let eq142_e1807_d_b8: f64 = (p.p7 * s.db[240][8]);
        let eq142_e1807_d_b9: f64 = (p.p7 * s.db[240][9]);
        let eq142_e1807_d_b10: f64 = (p.p7 * s.db[240][10]);
        let eq142_e1807_d_b11: f64 = (p.p7 * s.db[240][11]);
        let eq142_e1807_d_b12: f64 = (p.p7 * s.db[240][12]);
        let eq142_e1807_d_b13: f64 = (p.p7 * s.db[240][13]);
        let eq142_e1807_d_b14: f64 = (p.p7 * s.db[240][14]);
        let eq142_e1807_d_b15: f64 = (p.p7 * s.db[240][15]);
        let eq142_e1807_d_b16: f64 = (p.p7 * s.db[240][16]);
        let eq142_e1807_d_b17: f64 = (p.p7 * s.db[240][17]);
        let eq142_e1807_d_b18: f64 = (p.p7 * s.db[240][18]);
        let eq142_e1807_d_b19: f64 = (p.p7 * s.db[240][19]);
        let eq142_e1807_d_b20: f64 = (p.p7 * s.db[240][20]);
        let eq142_e1807_d_b21: f64 = (p.p7 * s.db[240][21]);
        let eq142_e1807_d_b22: f64 = (p.p7 * s.db[240][22]);
        let eq142_e1807_d_b23: f64 = (p.p7 * s.db[240][23]);
        let eq142_e1807_d_b24: f64 = (p.p7 * s.db[240][24]);
        let eq142_e1807_d_b25: f64 = (p.p7 * s.db[240][25]);
        let eq142_e1807_d_b26: f64 = (p.p7 * s.db[240][26]);
        let eq142_e1807_d_b27: f64 = (p.p7 * s.db[240][27]);
        let eq142_e1807_d_b28: f64 = (p.p7 * s.db[240][28]);
        let eq142_e1807_d_b29: f64 = (p.p7 * s.db[240][29]);
        let eq142_e1807_d_b30: f64 = (p.p7 * s.db[240][30]);
        let eq142_e1807_d_b31: f64 = (p.p7 * s.db[240][31]);
        let eq142_e1807_d_b32: f64 = (p.p7 * s.db[240][32]);
        let eq142_e1807_d_b33: f64 = (p.p7 * s.db[240][33]);
        let eq142_e1807_d_b34: f64 = (p.p7 * s.db[240][34]);
        let eq142_e1807_d_b35: f64 = (p.p7 * s.db[240][35]);
        let eq142_e1807_d_b36: f64 = (p.p7 * s.db[240][36]);
        let eq142_e1807_d_b37: f64 = (p.p7 * s.db[240][37]);
        let eq142_e1807_d_b38: f64 = (p.p7 * s.db[240][38]);
        let eq142_e1807_d_b39: f64 = (p.p7 * s.db[240][39]);
        let eq142_e1807_d_b40: f64 = (p.p7 * s.db[240][40]);
        let eq142_e1807_d_b41: f64 = (p.p7 * s.db[240][41]);
        let eq142_e1807_d_b42: f64 = (p.p7 * s.db[240][42]);
        let eq142_e1807_d_b43: f64 = (p.p7 * s.db[240][43]);
        let eq142_e1807_d_b44: f64 = (p.p7 * s.db[240][44]);
        let eq142_e1807_d_b45: f64 = (p.p7 * s.db[240][45]);
        let eq142_e1807_d_b46: f64 = (p.p7 * s.db[240][46]);
        let eq142_e1807_d_b47: f64 = (p.p7 * s.db[240][47]);
        let eq142_e1807_d_b48: f64 = (p.p7 * s.db[240][48]);
        let eq142_e1807_d_b49: f64 = (p.p7 * s.db[240][49]);
        let eq142_e1807_d_b50: f64 = (p.p7 * s.db[240][50]);
        let eq142_e1807_d_b51: f64 = (p.p7 * s.db[240][51]);
        let eq142_e1807_d_b52: f64 = (p.p7 * s.db[240][52]);
        let eq142_e1807_d_b53: f64 = (p.p7 * s.db[240][53]);
        let eq142_e1807_d_b54: f64 = (p.p7 * s.db[240][54]);
        let eq142_e1807_q: f64 = (p.p7 * eq142_e1806_q);
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
        let eq142_e1809_d_b0: f64 = (eq142_e1807_d_b0 * p.p246);
        let eq142_e1809_d_b1: f64 = (eq142_e1807_d_b1 * p.p246);
        let eq142_e1809_d_b2: f64 = (eq142_e1807_d_b2 * p.p246);
        let eq142_e1809_d_b3: f64 = (eq142_e1807_d_b3 * p.p246);
        let eq142_e1809_d_b4: f64 = (eq142_e1807_d_b4 * p.p246);
        let eq142_e1809_d_b5: f64 = (eq142_e1807_d_b5 * p.p246);
        let eq142_e1809_d_b6: f64 = (eq142_e1807_d_b6 * p.p246);
        let eq142_e1809_d_b7: f64 = (eq142_e1807_d_b7 * p.p246);
        let eq142_e1809_d_b8: f64 = (eq142_e1807_d_b8 * p.p246);
        let eq142_e1809_d_b9: f64 = (eq142_e1807_d_b9 * p.p246);
        let eq142_e1809_d_b10: f64 = (eq142_e1807_d_b10 * p.p246);
        let eq142_e1809_d_b11: f64 = (eq142_e1807_d_b11 * p.p246);
        let eq142_e1809_d_b12: f64 = (eq142_e1807_d_b12 * p.p246);
        let eq142_e1809_d_b13: f64 = (eq142_e1807_d_b13 * p.p246);
        let eq142_e1809_d_b14: f64 = (eq142_e1807_d_b14 * p.p246);
        let eq142_e1809_d_b15: f64 = (eq142_e1807_d_b15 * p.p246);
        let eq142_e1809_d_b16: f64 = (eq142_e1807_d_b16 * p.p246);
        let eq142_e1809_d_b17: f64 = (eq142_e1807_d_b17 * p.p246);
        let eq142_e1809_d_b18: f64 = (eq142_e1807_d_b18 * p.p246);
        let eq142_e1809_d_b19: f64 = (eq142_e1807_d_b19 * p.p246);
        let eq142_e1809_d_b20: f64 = (eq142_e1807_d_b20 * p.p246);
        let eq142_e1809_d_b21: f64 = (eq142_e1807_d_b21 * p.p246);
        let eq142_e1809_d_b22: f64 = (eq142_e1807_d_b22 * p.p246);
        let eq142_e1809_d_b23: f64 = (eq142_e1807_d_b23 * p.p246);
        let eq142_e1809_d_b24: f64 = (eq142_e1807_d_b24 * p.p246);
        let eq142_e1809_d_b25: f64 = (eq142_e1807_d_b25 * p.p246);
        let eq142_e1809_d_b26: f64 = (eq142_e1807_d_b26 * p.p246);
        let eq142_e1809_d_b27: f64 = (eq142_e1807_d_b27 * p.p246);
        let eq142_e1809_d_b28: f64 = (eq142_e1807_d_b28 * p.p246);
        let eq142_e1809_d_b29: f64 = (eq142_e1807_d_b29 * p.p246);
        let eq142_e1809_d_b30: f64 = (eq142_e1807_d_b30 * p.p246);
        let eq142_e1809_d_b31: f64 = (eq142_e1807_d_b31 * p.p246);
        let eq142_e1809_d_b32: f64 = (eq142_e1807_d_b32 * p.p246);
        let eq142_e1809_d_b33: f64 = (eq142_e1807_d_b33 * p.p246);
        let eq142_e1809_d_b34: f64 = (eq142_e1807_d_b34 * p.p246);
        let eq142_e1809_d_b35: f64 = (eq142_e1807_d_b35 * p.p246);
        let eq142_e1809_d_b36: f64 = (eq142_e1807_d_b36 * p.p246);
        let eq142_e1809_d_b37: f64 = (eq142_e1807_d_b37 * p.p246);
        let eq142_e1809_d_b38: f64 = (eq142_e1807_d_b38 * p.p246);
        let eq142_e1809_d_b39: f64 = (eq142_e1807_d_b39 * p.p246);
        let eq142_e1809_d_b40: f64 = (eq142_e1807_d_b40 * p.p246);
        let eq142_e1809_d_b41: f64 = (eq142_e1807_d_b41 * p.p246);
        let eq142_e1809_d_b42: f64 = (eq142_e1807_d_b42 * p.p246);
        let eq142_e1809_d_b43: f64 = (eq142_e1807_d_b43 * p.p246);
        let eq142_e1809_d_b44: f64 = (eq142_e1807_d_b44 * p.p246);
        let eq142_e1809_d_b45: f64 = (eq142_e1807_d_b45 * p.p246);
        let eq142_e1809_d_b46: f64 = (eq142_e1807_d_b46 * p.p246);
        let eq142_e1809_d_b47: f64 = (eq142_e1807_d_b47 * p.p246);
        let eq142_e1809_d_b48: f64 = (eq142_e1807_d_b48 * p.p246);
        let eq142_e1809_d_b49: f64 = (eq142_e1807_d_b49 * p.p246);
        let eq142_e1809_d_b50: f64 = (eq142_e1807_d_b50 * p.p246);
        let eq142_e1809_d_b51: f64 = (eq142_e1807_d_b51 * p.p246);
        let eq142_e1809_d_b52: f64 = (eq142_e1807_d_b52 * p.p246);
        let eq142_e1809_d_b53: f64 = (eq142_e1807_d_b53 * p.p246);
        let eq142_e1809_d_b54: f64 = (eq142_e1807_d_b54 * p.p246);
        let eq142_e1809_q: f64 = (eq142_e1807_q * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22, eq142_e1809_d_b0, eq142_e1809_d_b1, eq142_e1809_d_b2, eq142_e1809_d_b3, eq142_e1809_d_b4, eq142_e1809_d_b5, eq142_e1809_d_b6, eq142_e1809_d_b7, eq142_e1809_d_b8, eq142_e1809_d_b9, eq142_e1809_d_b10, eq142_e1809_d_b11, eq142_e1809_d_b12, eq142_e1809_d_b13, eq142_e1809_d_b14, eq142_e1809_d_b15, eq142_e1809_d_b16, eq142_e1809_d_b17, eq142_e1809_d_b18, eq142_e1809_d_b19, eq142_e1809_d_b20, eq142_e1809_d_b21, eq142_e1809_d_b22, eq142_e1809_d_b23, eq142_e1809_d_b24, eq142_e1809_d_b25, eq142_e1809_d_b26, eq142_e1809_d_b27, eq142_e1809_d_b28, eq142_e1809_d_b29, eq142_e1809_d_b30, eq142_e1809_d_b31, eq142_e1809_d_b32, eq142_e1809_d_b33, eq142_e1809_d_b34, eq142_e1809_d_b35, eq142_e1809_d_b36, eq142_e1809_d_b37, eq142_e1809_d_b38, eq142_e1809_d_b39, eq142_e1809_d_b40, eq142_e1809_d_b41, eq142_e1809_d_b42, eq142_e1809_d_b43, eq142_e1809_d_b44, eq142_e1809_d_b45, eq142_e1809_d_b46, eq142_e1809_d_b47, eq142_e1809_d_b48, eq142_e1809_d_b49, eq142_e1809_d_b50, eq142_e1809_d_b51, eq142_e1809_d_b52, eq142_e1809_d_b53, eq142_e1809_d_b54, eq142_e1809_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_reactive_node_derivatives: [f64; 23] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_reactive_branch_derivatives: [f64; 55] = [eq142_e1811_d_b0, eq142_e1811_d_b1, eq142_e1811_d_b2, eq142_e1811_d_b3, eq142_e1811_d_b4, eq142_e1811_d_b5, eq142_e1811_d_b6, eq142_e1811_d_b7, eq142_e1811_d_b8, eq142_e1811_d_b9, eq142_e1811_d_b10, eq142_e1811_d_b11, eq142_e1811_d_b12, eq142_e1811_d_b13, eq142_e1811_d_b14, eq142_e1811_d_b15, eq142_e1811_d_b16, eq142_e1811_d_b17, eq142_e1811_d_b18, eq142_e1811_d_b19, eq142_e1811_d_b20, eq142_e1811_d_b21, eq142_e1811_d_b22, eq142_e1811_d_b23, eq142_e1811_d_b24, eq142_e1811_d_b25, eq142_e1811_d_b26, eq142_e1811_d_b27, eq142_e1811_d_b28, eq142_e1811_d_b29, eq142_e1811_d_b30, eq142_e1811_d_b31, eq142_e1811_d_b32, eq142_e1811_d_b33, eq142_e1811_d_b34, eq142_e1811_d_b35, eq142_e1811_d_b36, eq142_e1811_d_b37, eq142_e1811_d_b38, eq142_e1811_d_b39, eq142_e1811_d_b40, eq142_e1811_d_b41, eq142_e1811_d_b42, eq142_e1811_d_b43, eq142_e1811_d_b44, eq142_e1811_d_b45, eq142_e1811_d_b46, eq142_e1811_d_b47, eq142_e1811_d_b48, eq142_e1811_d_b49, eq142_e1811_d_b50, eq142_e1811_d_b51, eq142_e1811_d_b52, eq142_e1811_d_b53, eq142_e1811_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq142_reactive_node_derivatives,
            branches,
            &eq142_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22, eq143_e1823_d_b0, eq143_e1823_d_b1, eq143_e1823_d_b2, eq143_e1823_d_b3, eq143_e1823_d_b4, eq143_e1823_d_b5, eq143_e1823_d_b6, eq143_e1823_d_b7, eq143_e1823_d_b8, eq143_e1823_d_b9, eq143_e1823_d_b10, eq143_e1823_d_b11, eq143_e1823_d_b12, eq143_e1823_d_b13, eq143_e1823_d_b14, eq143_e1823_d_b15, eq143_e1823_d_b16, eq143_e1823_d_b17, eq143_e1823_d_b18, eq143_e1823_d_b19, eq143_e1823_d_b20, eq143_e1823_d_b21, eq143_e1823_d_b22, eq143_e1823_d_b23, eq143_e1823_d_b24, eq143_e1823_d_b25, eq143_e1823_d_b26, eq143_e1823_d_b27, eq143_e1823_d_b28, eq143_e1823_d_b29, eq143_e1823_d_b30, eq143_e1823_d_b31, eq143_e1823_d_b32, eq143_e1823_d_b33, eq143_e1823_d_b34, eq143_e1823_d_b35, eq143_e1823_d_b36, eq143_e1823_d_b37, eq143_e1823_d_b38, eq143_e1823_d_b39, eq143_e1823_d_b40, eq143_e1823_d_b41, eq143_e1823_d_b42, eq143_e1823_d_b43, eq143_e1823_d_b44, eq143_e1823_d_b45, eq143_e1823_d_b46, eq143_e1823_d_b47, eq143_e1823_d_b48, eq143_e1823_d_b49, eq143_e1823_d_b50, eq143_e1823_d_b51, eq143_e1823_d_b52, eq143_e1823_d_b53, eq143_e1823_d_b54, eq143_e1823_q,) = {
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
        let eq143_e1819_d_b0: f64 = (p.p251 * s.db[240][0]);
        let eq143_e1819_d_b1: f64 = (p.p251 * s.db[240][1]);
        let eq143_e1819_d_b2: f64 = (p.p251 * s.db[240][2]);
        let eq143_e1819_d_b3: f64 = (p.p251 * s.db[240][3]);
        let eq143_e1819_d_b4: f64 = (p.p251 * s.db[240][4]);
        let eq143_e1819_d_b5: f64 = (p.p251 * s.db[240][5]);
        let eq143_e1819_d_b6: f64 = (p.p251 * s.db[240][6]);
        let eq143_e1819_d_b7: f64 = (p.p251 * s.db[240][7]);
        let eq143_e1819_d_b8: f64 = (p.p251 * s.db[240][8]);
        let eq143_e1819_d_b9: f64 = (p.p251 * s.db[240][9]);
        let eq143_e1819_d_b10: f64 = (p.p251 * s.db[240][10]);
        let eq143_e1819_d_b11: f64 = (p.p251 * s.db[240][11]);
        let eq143_e1819_d_b12: f64 = (p.p251 * s.db[240][12]);
        let eq143_e1819_d_b13: f64 = (p.p251 * s.db[240][13]);
        let eq143_e1819_d_b14: f64 = (p.p251 * s.db[240][14]);
        let eq143_e1819_d_b15: f64 = (p.p251 * s.db[240][15]);
        let eq143_e1819_d_b16: f64 = (p.p251 * s.db[240][16]);
        let eq143_e1819_d_b17: f64 = (p.p251 * s.db[240][17]);
        let eq143_e1819_d_b18: f64 = (p.p251 * s.db[240][18]);
        let eq143_e1819_d_b19: f64 = (p.p251 * s.db[240][19]);
        let eq143_e1819_d_b20: f64 = (p.p251 * s.db[240][20]);
        let eq143_e1819_d_b21: f64 = (p.p251 * s.db[240][21]);
        let eq143_e1819_d_b22: f64 = (p.p251 * s.db[240][22]);
        let eq143_e1819_d_b23: f64 = (p.p251 * s.db[240][23]);
        let eq143_e1819_d_b24: f64 = (p.p251 * s.db[240][24]);
        let eq143_e1819_d_b25: f64 = (p.p251 * s.db[240][25]);
        let eq143_e1819_d_b26: f64 = (p.p251 * s.db[240][26]);
        let eq143_e1819_d_b27: f64 = (p.p251 * s.db[240][27]);
        let eq143_e1819_d_b28: f64 = (p.p251 * s.db[240][28]);
        let eq143_e1819_d_b29: f64 = (p.p251 * s.db[240][29]);
        let eq143_e1819_d_b30: f64 = (p.p251 * s.db[240][30]);
        let eq143_e1819_d_b31: f64 = (p.p251 * s.db[240][31]);
        let eq143_e1819_d_b32: f64 = (p.p251 * s.db[240][32]);
        let eq143_e1819_d_b33: f64 = (p.p251 * s.db[240][33]);
        let eq143_e1819_d_b34: f64 = (p.p251 * s.db[240][34]);
        let eq143_e1819_d_b35: f64 = (p.p251 * s.db[240][35]);
        let eq143_e1819_d_b36: f64 = (p.p251 * s.db[240][36]);
        let eq143_e1819_d_b37: f64 = (p.p251 * s.db[240][37]);
        let eq143_e1819_d_b38: f64 = (p.p251 * s.db[240][38]);
        let eq143_e1819_d_b39: f64 = (p.p251 * s.db[240][39]);
        let eq143_e1819_d_b40: f64 = (p.p251 * s.db[240][40]);
        let eq143_e1819_d_b41: f64 = (p.p251 * s.db[240][41]);
        let eq143_e1819_d_b42: f64 = (p.p251 * s.db[240][42]);
        let eq143_e1819_d_b43: f64 = (p.p251 * s.db[240][43]);
        let eq143_e1819_d_b44: f64 = (p.p251 * s.db[240][44]);
        let eq143_e1819_d_b45: f64 = (p.p251 * s.db[240][45]);
        let eq143_e1819_d_b46: f64 = (p.p251 * s.db[240][46]);
        let eq143_e1819_d_b47: f64 = (p.p251 * s.db[240][47]);
        let eq143_e1819_d_b48: f64 = (p.p251 * s.db[240][48]);
        let eq143_e1819_d_b49: f64 = (p.p251 * s.db[240][49]);
        let eq143_e1819_d_b50: f64 = (p.p251 * s.db[240][50]);
        let eq143_e1819_d_b51: f64 = (p.p251 * s.db[240][51]);
        let eq143_e1819_d_b52: f64 = (p.p251 * s.db[240][52]);
        let eq143_e1819_d_b53: f64 = (p.p251 * s.db[240][53]);
        let eq143_e1819_d_b54: f64 = (p.p251 * s.db[240][54]);
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
        let eq143_e1821_d_b0: f64 = (p.p7 * eq143_e1819_d_b0);
        let eq143_e1821_d_b1: f64 = (p.p7 * eq143_e1819_d_b1);
        let eq143_e1821_d_b2: f64 = (p.p7 * eq143_e1819_d_b2);
        let eq143_e1821_d_b3: f64 = (p.p7 * eq143_e1819_d_b3);
        let eq143_e1821_d_b4: f64 = (p.p7 * eq143_e1819_d_b4);
        let eq143_e1821_d_b5: f64 = (p.p7 * eq143_e1819_d_b5);
        let eq143_e1821_d_b6: f64 = (p.p7 * eq143_e1819_d_b6);
        let eq143_e1821_d_b7: f64 = (p.p7 * eq143_e1819_d_b7);
        let eq143_e1821_d_b8: f64 = (p.p7 * eq143_e1819_d_b8);
        let eq143_e1821_d_b9: f64 = (p.p7 * eq143_e1819_d_b9);
        let eq143_e1821_d_b10: f64 = (p.p7 * eq143_e1819_d_b10);
        let eq143_e1821_d_b11: f64 = (p.p7 * eq143_e1819_d_b11);
        let eq143_e1821_d_b12: f64 = (p.p7 * eq143_e1819_d_b12);
        let eq143_e1821_d_b13: f64 = (p.p7 * eq143_e1819_d_b13);
        let eq143_e1821_d_b14: f64 = (p.p7 * eq143_e1819_d_b14);
        let eq143_e1821_d_b15: f64 = (p.p7 * eq143_e1819_d_b15);
        let eq143_e1821_d_b16: f64 = (p.p7 * eq143_e1819_d_b16);
        let eq143_e1821_d_b17: f64 = (p.p7 * eq143_e1819_d_b17);
        let eq143_e1821_d_b18: f64 = (p.p7 * eq143_e1819_d_b18);
        let eq143_e1821_d_b19: f64 = (p.p7 * eq143_e1819_d_b19);
        let eq143_e1821_d_b20: f64 = (p.p7 * eq143_e1819_d_b20);
        let eq143_e1821_d_b21: f64 = (p.p7 * eq143_e1819_d_b21);
        let eq143_e1821_d_b22: f64 = (p.p7 * eq143_e1819_d_b22);
        let eq143_e1821_d_b23: f64 = (p.p7 * eq143_e1819_d_b23);
        let eq143_e1821_d_b24: f64 = (p.p7 * eq143_e1819_d_b24);
        let eq143_e1821_d_b25: f64 = (p.p7 * eq143_e1819_d_b25);
        let eq143_e1821_d_b26: f64 = (p.p7 * eq143_e1819_d_b26);
        let eq143_e1821_d_b27: f64 = (p.p7 * eq143_e1819_d_b27);
        let eq143_e1821_d_b28: f64 = (p.p7 * eq143_e1819_d_b28);
        let eq143_e1821_d_b29: f64 = (p.p7 * eq143_e1819_d_b29);
        let eq143_e1821_d_b30: f64 = (p.p7 * eq143_e1819_d_b30);
        let eq143_e1821_d_b31: f64 = (p.p7 * eq143_e1819_d_b31);
        let eq143_e1821_d_b32: f64 = (p.p7 * eq143_e1819_d_b32);
        let eq143_e1821_d_b33: f64 = (p.p7 * eq143_e1819_d_b33);
        let eq143_e1821_d_b34: f64 = (p.p7 * eq143_e1819_d_b34);
        let eq143_e1821_d_b35: f64 = (p.p7 * eq143_e1819_d_b35);
        let eq143_e1821_d_b36: f64 = (p.p7 * eq143_e1819_d_b36);
        let eq143_e1821_d_b37: f64 = (p.p7 * eq143_e1819_d_b37);
        let eq143_e1821_d_b38: f64 = (p.p7 * eq143_e1819_d_b38);
        let eq143_e1821_d_b39: f64 = (p.p7 * eq143_e1819_d_b39);
        let eq143_e1821_d_b40: f64 = (p.p7 * eq143_e1819_d_b40);
        let eq143_e1821_d_b41: f64 = (p.p7 * eq143_e1819_d_b41);
        let eq143_e1821_d_b42: f64 = (p.p7 * eq143_e1819_d_b42);
        let eq143_e1821_d_b43: f64 = (p.p7 * eq143_e1819_d_b43);
        let eq143_e1821_d_b44: f64 = (p.p7 * eq143_e1819_d_b44);
        let eq143_e1821_d_b45: f64 = (p.p7 * eq143_e1819_d_b45);
        let eq143_e1821_d_b46: f64 = (p.p7 * eq143_e1819_d_b46);
        let eq143_e1821_d_b47: f64 = (p.p7 * eq143_e1819_d_b47);
        let eq143_e1821_d_b48: f64 = (p.p7 * eq143_e1819_d_b48);
        let eq143_e1821_d_b49: f64 = (p.p7 * eq143_e1819_d_b49);
        let eq143_e1821_d_b50: f64 = (p.p7 * eq143_e1819_d_b50);
        let eq143_e1821_d_b51: f64 = (p.p7 * eq143_e1819_d_b51);
        let eq143_e1821_d_b52: f64 = (p.p7 * eq143_e1819_d_b52);
        let eq143_e1821_d_b53: f64 = (p.p7 * eq143_e1819_d_b53);
        let eq143_e1821_d_b54: f64 = (p.p7 * eq143_e1819_d_b54);
        let eq143_e1821_q: f64 = (p.p7 * eq143_e1820_q);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22, eq143_e1821_d_b0, eq143_e1821_d_b1, eq143_e1821_d_b2, eq143_e1821_d_b3, eq143_e1821_d_b4, eq143_e1821_d_b5, eq143_e1821_d_b6, eq143_e1821_d_b7, eq143_e1821_d_b8, eq143_e1821_d_b9, eq143_e1821_d_b10, eq143_e1821_d_b11, eq143_e1821_d_b12, eq143_e1821_d_b13, eq143_e1821_d_b14, eq143_e1821_d_b15, eq143_e1821_d_b16, eq143_e1821_d_b17, eq143_e1821_d_b18, eq143_e1821_d_b19, eq143_e1821_d_b20, eq143_e1821_d_b21, eq143_e1821_d_b22, eq143_e1821_d_b23, eq143_e1821_d_b24, eq143_e1821_d_b25, eq143_e1821_d_b26, eq143_e1821_d_b27, eq143_e1821_d_b28, eq143_e1821_d_b29, eq143_e1821_d_b30, eq143_e1821_d_b31, eq143_e1821_d_b32, eq143_e1821_d_b33, eq143_e1821_d_b34, eq143_e1821_d_b35, eq143_e1821_d_b36, eq143_e1821_d_b37, eq143_e1821_d_b38, eq143_e1821_d_b39, eq143_e1821_d_b40, eq143_e1821_d_b41, eq143_e1821_d_b42, eq143_e1821_d_b43, eq143_e1821_d_b44, eq143_e1821_d_b45, eq143_e1821_d_b46, eq143_e1821_d_b47, eq143_e1821_d_b48, eq143_e1821_d_b49, eq143_e1821_d_b50, eq143_e1821_d_b51, eq143_e1821_d_b52, eq143_e1821_d_b53, eq143_e1821_d_b54, eq143_e1821_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_reactive_node_derivatives: [f64; 23] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_reactive_branch_derivatives: [f64; 55] = [eq143_e1823_d_b0, eq143_e1823_d_b1, eq143_e1823_d_b2, eq143_e1823_d_b3, eq143_e1823_d_b4, eq143_e1823_d_b5, eq143_e1823_d_b6, eq143_e1823_d_b7, eq143_e1823_d_b8, eq143_e1823_d_b9, eq143_e1823_d_b10, eq143_e1823_d_b11, eq143_e1823_d_b12, eq143_e1823_d_b13, eq143_e1823_d_b14, eq143_e1823_d_b15, eq143_e1823_d_b16, eq143_e1823_d_b17, eq143_e1823_d_b18, eq143_e1823_d_b19, eq143_e1823_d_b20, eq143_e1823_d_b21, eq143_e1823_d_b22, eq143_e1823_d_b23, eq143_e1823_d_b24, eq143_e1823_d_b25, eq143_e1823_d_b26, eq143_e1823_d_b27, eq143_e1823_d_b28, eq143_e1823_d_b29, eq143_e1823_d_b30, eq143_e1823_d_b31, eq143_e1823_d_b32, eq143_e1823_d_b33, eq143_e1823_d_b34, eq143_e1823_d_b35, eq143_e1823_d_b36, eq143_e1823_d_b37, eq143_e1823_d_b38, eq143_e1823_d_b39, eq143_e1823_d_b40, eq143_e1823_d_b41, eq143_e1823_d_b42, eq143_e1823_d_b43, eq143_e1823_d_b44, eq143_e1823_d_b45, eq143_e1823_d_b46, eq143_e1823_d_b47, eq143_e1823_d_b48, eq143_e1823_d_b49, eq143_e1823_d_b50, eq143_e1823_d_b51, eq143_e1823_d_b52, eq143_e1823_d_b53, eq143_e1823_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq143_reactive_node_derivatives,
            branches,
            &eq143_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22, eq144_e1832_d_b0, eq144_e1832_d_b1, eq144_e1832_d_b2, eq144_e1832_d_b3, eq144_e1832_d_b4, eq144_e1832_d_b5, eq144_e1832_d_b6, eq144_e1832_d_b7, eq144_e1832_d_b8, eq144_e1832_d_b9, eq144_e1832_d_b10, eq144_e1832_d_b11, eq144_e1832_d_b12, eq144_e1832_d_b13, eq144_e1832_d_b14, eq144_e1832_d_b15, eq144_e1832_d_b16, eq144_e1832_d_b17, eq144_e1832_d_b18, eq144_e1832_d_b19, eq144_e1832_d_b20, eq144_e1832_d_b21, eq144_e1832_d_b22, eq144_e1832_d_b23, eq144_e1832_d_b24, eq144_e1832_d_b25, eq144_e1832_d_b26, eq144_e1832_d_b27, eq144_e1832_d_b28, eq144_e1832_d_b29, eq144_e1832_d_b30, eq144_e1832_d_b31, eq144_e1832_d_b32, eq144_e1832_d_b33, eq144_e1832_d_b34, eq144_e1832_d_b35, eq144_e1832_d_b36, eq144_e1832_d_b37, eq144_e1832_d_b38, eq144_e1832_d_b39, eq144_e1832_d_b40, eq144_e1832_d_b41, eq144_e1832_d_b42, eq144_e1832_d_b43, eq144_e1832_d_b44, eq144_e1832_d_b45, eq144_e1832_d_b46, eq144_e1832_d_b47, eq144_e1832_d_b48, eq144_e1832_d_b49, eq144_e1832_d_b50, eq144_e1832_d_b51, eq144_e1832_d_b52, eq144_e1832_d_b53, eq144_e1832_d_b54, eq144_e1832_q,) = {
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
        let eq144_e1830_d_b0: f64 = (p.p7 * s.db[253][0]);
        let eq144_e1830_d_b1: f64 = (p.p7 * s.db[253][1]);
        let eq144_e1830_d_b2: f64 = (p.p7 * s.db[253][2]);
        let eq144_e1830_d_b3: f64 = (p.p7 * s.db[253][3]);
        let eq144_e1830_d_b4: f64 = (p.p7 * s.db[253][4]);
        let eq144_e1830_d_b5: f64 = (p.p7 * s.db[253][5]);
        let eq144_e1830_d_b6: f64 = (p.p7 * s.db[253][6]);
        let eq144_e1830_d_b7: f64 = (p.p7 * s.db[253][7]);
        let eq144_e1830_d_b8: f64 = (p.p7 * s.db[253][8]);
        let eq144_e1830_d_b9: f64 = (p.p7 * s.db[253][9]);
        let eq144_e1830_d_b10: f64 = (p.p7 * s.db[253][10]);
        let eq144_e1830_d_b11: f64 = (p.p7 * s.db[253][11]);
        let eq144_e1830_d_b12: f64 = (p.p7 * s.db[253][12]);
        let eq144_e1830_d_b13: f64 = (p.p7 * s.db[253][13]);
        let eq144_e1830_d_b14: f64 = (p.p7 * s.db[253][14]);
        let eq144_e1830_d_b15: f64 = (p.p7 * s.db[253][15]);
        let eq144_e1830_d_b16: f64 = (p.p7 * s.db[253][16]);
        let eq144_e1830_d_b17: f64 = (p.p7 * s.db[253][17]);
        let eq144_e1830_d_b18: f64 = (p.p7 * s.db[253][18]);
        let eq144_e1830_d_b19: f64 = (p.p7 * s.db[253][19]);
        let eq144_e1830_d_b20: f64 = (p.p7 * s.db[253][20]);
        let eq144_e1830_d_b21: f64 = (p.p7 * s.db[253][21]);
        let eq144_e1830_d_b22: f64 = (p.p7 * s.db[253][22]);
        let eq144_e1830_d_b23: f64 = (p.p7 * s.db[253][23]);
        let eq144_e1830_d_b24: f64 = (p.p7 * s.db[253][24]);
        let eq144_e1830_d_b25: f64 = (p.p7 * s.db[253][25]);
        let eq144_e1830_d_b26: f64 = (p.p7 * s.db[253][26]);
        let eq144_e1830_d_b27: f64 = (p.p7 * s.db[253][27]);
        let eq144_e1830_d_b28: f64 = (p.p7 * s.db[253][28]);
        let eq144_e1830_d_b29: f64 = (p.p7 * s.db[253][29]);
        let eq144_e1830_d_b30: f64 = (p.p7 * s.db[253][30]);
        let eq144_e1830_d_b31: f64 = (p.p7 * s.db[253][31]);
        let eq144_e1830_d_b32: f64 = (p.p7 * s.db[253][32]);
        let eq144_e1830_d_b33: f64 = (p.p7 * s.db[253][33]);
        let eq144_e1830_d_b34: f64 = (p.p7 * s.db[253][34]);
        let eq144_e1830_d_b35: f64 = (p.p7 * s.db[253][35]);
        let eq144_e1830_d_b36: f64 = (p.p7 * s.db[253][36]);
        let eq144_e1830_d_b37: f64 = (p.p7 * s.db[253][37]);
        let eq144_e1830_d_b38: f64 = (p.p7 * s.db[253][38]);
        let eq144_e1830_d_b39: f64 = (p.p7 * s.db[253][39]);
        let eq144_e1830_d_b40: f64 = (p.p7 * s.db[253][40]);
        let eq144_e1830_d_b41: f64 = (p.p7 * s.db[253][41]);
        let eq144_e1830_d_b42: f64 = (p.p7 * s.db[253][42]);
        let eq144_e1830_d_b43: f64 = (p.p7 * s.db[253][43]);
        let eq144_e1830_d_b44: f64 = (p.p7 * s.db[253][44]);
        let eq144_e1830_d_b45: f64 = (p.p7 * s.db[253][45]);
        let eq144_e1830_d_b46: f64 = (p.p7 * s.db[253][46]);
        let eq144_e1830_d_b47: f64 = (p.p7 * s.db[253][47]);
        let eq144_e1830_d_b48: f64 = (p.p7 * s.db[253][48]);
        let eq144_e1830_d_b49: f64 = (p.p7 * s.db[253][49]);
        let eq144_e1830_d_b50: f64 = (p.p7 * s.db[253][50]);
        let eq144_e1830_d_b51: f64 = (p.p7 * s.db[253][51]);
        let eq144_e1830_d_b52: f64 = (p.p7 * s.db[253][52]);
        let eq144_e1830_d_b53: f64 = (p.p7 * s.db[253][53]);
        let eq144_e1830_d_b54: f64 = (p.p7 * s.db[253][54]);
        let eq144_e1830_q: f64 = (p.p7 * eq144_e1829_q);
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22, eq144_e1830_d_b0, eq144_e1830_d_b1, eq144_e1830_d_b2, eq144_e1830_d_b3, eq144_e1830_d_b4, eq144_e1830_d_b5, eq144_e1830_d_b6, eq144_e1830_d_b7, eq144_e1830_d_b8, eq144_e1830_d_b9, eq144_e1830_d_b10, eq144_e1830_d_b11, eq144_e1830_d_b12, eq144_e1830_d_b13, eq144_e1830_d_b14, eq144_e1830_d_b15, eq144_e1830_d_b16, eq144_e1830_d_b17, eq144_e1830_d_b18, eq144_e1830_d_b19, eq144_e1830_d_b20, eq144_e1830_d_b21, eq144_e1830_d_b22, eq144_e1830_d_b23, eq144_e1830_d_b24, eq144_e1830_d_b25, eq144_e1830_d_b26, eq144_e1830_d_b27, eq144_e1830_d_b28, eq144_e1830_d_b29, eq144_e1830_d_b30, eq144_e1830_d_b31, eq144_e1830_d_b32, eq144_e1830_d_b33, eq144_e1830_d_b34, eq144_e1830_d_b35, eq144_e1830_d_b36, eq144_e1830_d_b37, eq144_e1830_d_b38, eq144_e1830_d_b39, eq144_e1830_d_b40, eq144_e1830_d_b41, eq144_e1830_d_b42, eq144_e1830_d_b43, eq144_e1830_d_b44, eq144_e1830_d_b45, eq144_e1830_d_b46, eq144_e1830_d_b47, eq144_e1830_d_b48, eq144_e1830_d_b49, eq144_e1830_d_b50, eq144_e1830_d_b51, eq144_e1830_d_b52, eq144_e1830_d_b53, eq144_e1830_d_b54, eq144_e1830_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_reactive_node_derivatives: [f64; 23] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_reactive_branch_derivatives: [f64; 55] = [eq144_e1832_d_b0, eq144_e1832_d_b1, eq144_e1832_d_b2, eq144_e1832_d_b3, eq144_e1832_d_b4, eq144_e1832_d_b5, eq144_e1832_d_b6, eq144_e1832_d_b7, eq144_e1832_d_b8, eq144_e1832_d_b9, eq144_e1832_d_b10, eq144_e1832_d_b11, eq144_e1832_d_b12, eq144_e1832_d_b13, eq144_e1832_d_b14, eq144_e1832_d_b15, eq144_e1832_d_b16, eq144_e1832_d_b17, eq144_e1832_d_b18, eq144_e1832_d_b19, eq144_e1832_d_b20, eq144_e1832_d_b21, eq144_e1832_d_b22, eq144_e1832_d_b23, eq144_e1832_d_b24, eq144_e1832_d_b25, eq144_e1832_d_b26, eq144_e1832_d_b27, eq144_e1832_d_b28, eq144_e1832_d_b29, eq144_e1832_d_b30, eq144_e1832_d_b31, eq144_e1832_d_b32, eq144_e1832_d_b33, eq144_e1832_d_b34, eq144_e1832_d_b35, eq144_e1832_d_b36, eq144_e1832_d_b37, eq144_e1832_d_b38, eq144_e1832_d_b39, eq144_e1832_d_b40, eq144_e1832_d_b41, eq144_e1832_d_b42, eq144_e1832_d_b43, eq144_e1832_d_b44, eq144_e1832_d_b45, eq144_e1832_d_b46, eq144_e1832_d_b47, eq144_e1832_d_b48, eq144_e1832_d_b49, eq144_e1832_d_b50, eq144_e1832_d_b51, eq144_e1832_d_b52, eq144_e1832_d_b53, eq144_e1832_d_b54];
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

    pub(super) fn stamp_reactive_equations_block_11(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[252][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[252][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[252][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[252][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[252][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[252][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[252][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[252][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[252][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[252][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[252][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[252][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[252][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[252][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[252][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[252][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[252][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[252][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[252][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[252][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[252][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[252][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[252][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[252][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[252][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[252][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[252][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[252][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[252][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[252][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[252][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[252][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[252][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[252][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[252][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[252][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[252][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[252][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[252][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[252][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[252][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[252][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[252][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[252][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[252][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[252][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[252][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[252][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[252][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[252][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[252][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[252][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[252][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[252][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[252][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[252][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[252][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[252][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[252][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[252][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[252][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[252][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[252][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[252][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[252][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[252][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[252][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[252][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[252][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[252][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[252][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[252][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[252][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[252][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[252][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[252][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[252][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[252][54]);
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22, eq145_e1843_d_b0, eq145_e1843_d_b1, eq145_e1843_d_b2, eq145_e1843_d_b3, eq145_e1843_d_b4, eq145_e1843_d_b5, eq145_e1843_d_b6, eq145_e1843_d_b7, eq145_e1843_d_b8, eq145_e1843_d_b9, eq145_e1843_d_b10, eq145_e1843_d_b11, eq145_e1843_d_b12, eq145_e1843_d_b13, eq145_e1843_d_b14, eq145_e1843_d_b15, eq145_e1843_d_b16, eq145_e1843_d_b17, eq145_e1843_d_b18, eq145_e1843_d_b19, eq145_e1843_d_b20, eq145_e1843_d_b21, eq145_e1843_d_b22, eq145_e1843_d_b23, eq145_e1843_d_b24, eq145_e1843_d_b25, eq145_e1843_d_b26, eq145_e1843_d_b27, eq145_e1843_d_b28, eq145_e1843_d_b29, eq145_e1843_d_b30, eq145_e1843_d_b31, eq145_e1843_d_b32, eq145_e1843_d_b33, eq145_e1843_d_b34, eq145_e1843_d_b35, eq145_e1843_d_b36, eq145_e1843_d_b37, eq145_e1843_d_b38, eq145_e1843_d_b39, eq145_e1843_d_b40, eq145_e1843_d_b41, eq145_e1843_d_b42, eq145_e1843_d_b43, eq145_e1843_d_b44, eq145_e1843_d_b45, eq145_e1843_d_b46, eq145_e1843_d_b47, eq145_e1843_d_b48, eq145_e1843_d_b49, eq145_e1843_d_b50, eq145_e1843_d_b51, eq145_e1843_d_b52, eq145_e1843_d_b53, eq145_e1843_d_b54, eq145_e1843_q,) = {
    if ((s.b[580] && s.b[581]) && s.b[582]) {
        let eq145_e1840_q: f64 = s.v[252];
        let eq145_e1841: f64 = (p.p7 * s.v[252]);
        let eq145_e1841_q: f64 = (p.p7 * eq145_e1840_q);
        (eq145_e1841, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq145_e1841_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_reactive_node_derivatives: [f64; 23] = [eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22];
        let eq145_reactive_branch_derivatives: [f64; 55] = [eq145_e1843_d_b0, eq145_e1843_d_b1, eq145_e1843_d_b2, eq145_e1843_d_b3, eq145_e1843_d_b4, eq145_e1843_d_b5, eq145_e1843_d_b6, eq145_e1843_d_b7, eq145_e1843_d_b8, eq145_e1843_d_b9, eq145_e1843_d_b10, eq145_e1843_d_b11, eq145_e1843_d_b12, eq145_e1843_d_b13, eq145_e1843_d_b14, eq145_e1843_d_b15, eq145_e1843_d_b16, eq145_e1843_d_b17, eq145_e1843_d_b18, eq145_e1843_d_b19, eq145_e1843_d_b20, eq145_e1843_d_b21, eq145_e1843_d_b22, eq145_e1843_d_b23, eq145_e1843_d_b24, eq145_e1843_d_b25, eq145_e1843_d_b26, eq145_e1843_d_b27, eq145_e1843_d_b28, eq145_e1843_d_b29, eq145_e1843_d_b30, eq145_e1843_d_b31, eq145_e1843_d_b32, eq145_e1843_d_b33, eq145_e1843_d_b34, eq145_e1843_d_b35, eq145_e1843_d_b36, eq145_e1843_d_b37, eq145_e1843_d_b38, eq145_e1843_d_b39, eq145_e1843_d_b40, eq145_e1843_d_b41, eq145_e1843_d_b42, eq145_e1843_d_b43, eq145_e1843_d_b44, eq145_e1843_d_b45, eq145_e1843_d_b46, eq145_e1843_d_b47, eq145_e1843_d_b48, eq145_e1843_d_b49, eq145_e1843_d_b50, eq145_e1843_d_b51, eq145_e1843_d_b52, eq145_e1843_d_b53, eq145_e1843_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            nodes,
            &eq145_reactive_node_derivatives,
            branches,
            &eq145_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22, eq146_e1856_d_b0, eq146_e1856_d_b1, eq146_e1856_d_b2, eq146_e1856_d_b3, eq146_e1856_d_b4, eq146_e1856_d_b5, eq146_e1856_d_b6, eq146_e1856_d_b7, eq146_e1856_d_b8, eq146_e1856_d_b9, eq146_e1856_d_b10, eq146_e1856_d_b11, eq146_e1856_d_b12, eq146_e1856_d_b13, eq146_e1856_d_b14, eq146_e1856_d_b15, eq146_e1856_d_b16, eq146_e1856_d_b17, eq146_e1856_d_b18, eq146_e1856_d_b19, eq146_e1856_d_b20, eq146_e1856_d_b21, eq146_e1856_d_b22, eq146_e1856_d_b23, eq146_e1856_d_b24, eq146_e1856_d_b25, eq146_e1856_d_b26, eq146_e1856_d_b27, eq146_e1856_d_b28, eq146_e1856_d_b29, eq146_e1856_d_b30, eq146_e1856_d_b31, eq146_e1856_d_b32, eq146_e1856_d_b33, eq146_e1856_d_b34, eq146_e1856_d_b35, eq146_e1856_d_b36, eq146_e1856_d_b37, eq146_e1856_d_b38, eq146_e1856_d_b39, eq146_e1856_d_b40, eq146_e1856_d_b41, eq146_e1856_d_b42, eq146_e1856_d_b43, eq146_e1856_d_b44, eq146_e1856_d_b45, eq146_e1856_d_b46, eq146_e1856_d_b47, eq146_e1856_d_b48, eq146_e1856_d_b49, eq146_e1856_d_b50, eq146_e1856_d_b51, eq146_e1856_d_b52, eq146_e1856_d_b53, eq146_e1856_d_b54, eq146_e1856_q,) = {
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
        let eq146_e1854_d_b0: f64 = (eq146_e1851 * s.db[252][0]);
        let eq146_e1854_d_b1: f64 = (eq146_e1851 * s.db[252][1]);
        let eq146_e1854_d_b2: f64 = (eq146_e1851 * s.db[252][2]);
        let eq146_e1854_d_b3: f64 = (eq146_e1851 * s.db[252][3]);
        let eq146_e1854_d_b4: f64 = (eq146_e1851 * s.db[252][4]);
        let eq146_e1854_d_b5: f64 = (eq146_e1851 * s.db[252][5]);
        let eq146_e1854_d_b6: f64 = (eq146_e1851 * s.db[252][6]);
        let eq146_e1854_d_b7: f64 = (eq146_e1851 * s.db[252][7]);
        let eq146_e1854_d_b8: f64 = (eq146_e1851 * s.db[252][8]);
        let eq146_e1854_d_b9: f64 = (eq146_e1851 * s.db[252][9]);
        let eq146_e1854_d_b10: f64 = (eq146_e1851 * s.db[252][10]);
        let eq146_e1854_d_b11: f64 = (eq146_e1851 * s.db[252][11]);
        let eq146_e1854_d_b12: f64 = (eq146_e1851 * s.db[252][12]);
        let eq146_e1854_d_b13: f64 = (eq146_e1851 * s.db[252][13]);
        let eq146_e1854_d_b14: f64 = (eq146_e1851 * s.db[252][14]);
        let eq146_e1854_d_b15: f64 = (eq146_e1851 * s.db[252][15]);
        let eq146_e1854_d_b16: f64 = (eq146_e1851 * s.db[252][16]);
        let eq146_e1854_d_b17: f64 = (eq146_e1851 * s.db[252][17]);
        let eq146_e1854_d_b18: f64 = (eq146_e1851 * s.db[252][18]);
        let eq146_e1854_d_b19: f64 = (eq146_e1851 * s.db[252][19]);
        let eq146_e1854_d_b20: f64 = (eq146_e1851 * s.db[252][20]);
        let eq146_e1854_d_b21: f64 = (eq146_e1851 * s.db[252][21]);
        let eq146_e1854_d_b22: f64 = (eq146_e1851 * s.db[252][22]);
        let eq146_e1854_d_b23: f64 = (eq146_e1851 * s.db[252][23]);
        let eq146_e1854_d_b24: f64 = (eq146_e1851 * s.db[252][24]);
        let eq146_e1854_d_b25: f64 = (eq146_e1851 * s.db[252][25]);
        let eq146_e1854_d_b26: f64 = (eq146_e1851 * s.db[252][26]);
        let eq146_e1854_d_b27: f64 = (eq146_e1851 * s.db[252][27]);
        let eq146_e1854_d_b28: f64 = (eq146_e1851 * s.db[252][28]);
        let eq146_e1854_d_b29: f64 = (eq146_e1851 * s.db[252][29]);
        let eq146_e1854_d_b30: f64 = (eq146_e1851 * s.db[252][30]);
        let eq146_e1854_d_b31: f64 = (eq146_e1851 * s.db[252][31]);
        let eq146_e1854_d_b32: f64 = (eq146_e1851 * s.db[252][32]);
        let eq146_e1854_d_b33: f64 = (eq146_e1851 * s.db[252][33]);
        let eq146_e1854_d_b34: f64 = (eq146_e1851 * s.db[252][34]);
        let eq146_e1854_d_b35: f64 = (eq146_e1851 * s.db[252][35]);
        let eq146_e1854_d_b36: f64 = (eq146_e1851 * s.db[252][36]);
        let eq146_e1854_d_b37: f64 = (eq146_e1851 * s.db[252][37]);
        let eq146_e1854_d_b38: f64 = (eq146_e1851 * s.db[252][38]);
        let eq146_e1854_d_b39: f64 = (eq146_e1851 * s.db[252][39]);
        let eq146_e1854_d_b40: f64 = (eq146_e1851 * s.db[252][40]);
        let eq146_e1854_d_b41: f64 = (eq146_e1851 * s.db[252][41]);
        let eq146_e1854_d_b42: f64 = (eq146_e1851 * s.db[252][42]);
        let eq146_e1854_d_b43: f64 = (eq146_e1851 * s.db[252][43]);
        let eq146_e1854_d_b44: f64 = (eq146_e1851 * s.db[252][44]);
        let eq146_e1854_d_b45: f64 = (eq146_e1851 * s.db[252][45]);
        let eq146_e1854_d_b46: f64 = (eq146_e1851 * s.db[252][46]);
        let eq146_e1854_d_b47: f64 = (eq146_e1851 * s.db[252][47]);
        let eq146_e1854_d_b48: f64 = (eq146_e1851 * s.db[252][48]);
        let eq146_e1854_d_b49: f64 = (eq146_e1851 * s.db[252][49]);
        let eq146_e1854_d_b50: f64 = (eq146_e1851 * s.db[252][50]);
        let eq146_e1854_d_b51: f64 = (eq146_e1851 * s.db[252][51]);
        let eq146_e1854_d_b52: f64 = (eq146_e1851 * s.db[252][52]);
        let eq146_e1854_d_b53: f64 = (eq146_e1851 * s.db[252][53]);
        let eq146_e1854_d_b54: f64 = (eq146_e1851 * s.db[252][54]);
        let eq146_e1854_q: f64 = (eq146_e1851 * eq146_e1853_q);
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n10, eq146_e1854_d_n11, eq146_e1854_d_n12, eq146_e1854_d_n13, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22, eq146_e1854_d_b0, eq146_e1854_d_b1, eq146_e1854_d_b2, eq146_e1854_d_b3, eq146_e1854_d_b4, eq146_e1854_d_b5, eq146_e1854_d_b6, eq146_e1854_d_b7, eq146_e1854_d_b8, eq146_e1854_d_b9, eq146_e1854_d_b10, eq146_e1854_d_b11, eq146_e1854_d_b12, eq146_e1854_d_b13, eq146_e1854_d_b14, eq146_e1854_d_b15, eq146_e1854_d_b16, eq146_e1854_d_b17, eq146_e1854_d_b18, eq146_e1854_d_b19, eq146_e1854_d_b20, eq146_e1854_d_b21, eq146_e1854_d_b22, eq146_e1854_d_b23, eq146_e1854_d_b24, eq146_e1854_d_b25, eq146_e1854_d_b26, eq146_e1854_d_b27, eq146_e1854_d_b28, eq146_e1854_d_b29, eq146_e1854_d_b30, eq146_e1854_d_b31, eq146_e1854_d_b32, eq146_e1854_d_b33, eq146_e1854_d_b34, eq146_e1854_d_b35, eq146_e1854_d_b36, eq146_e1854_d_b37, eq146_e1854_d_b38, eq146_e1854_d_b39, eq146_e1854_d_b40, eq146_e1854_d_b41, eq146_e1854_d_b42, eq146_e1854_d_b43, eq146_e1854_d_b44, eq146_e1854_d_b45, eq146_e1854_d_b46, eq146_e1854_d_b47, eq146_e1854_d_b48, eq146_e1854_d_b49, eq146_e1854_d_b50, eq146_e1854_d_b51, eq146_e1854_d_b52, eq146_e1854_d_b53, eq146_e1854_d_b54, eq146_e1854_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_reactive_node_derivatives: [f64; 23] = [eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22];
        let eq146_reactive_branch_derivatives: [f64; 55] = [eq146_e1856_d_b0, eq146_e1856_d_b1, eq146_e1856_d_b2, eq146_e1856_d_b3, eq146_e1856_d_b4, eq146_e1856_d_b5, eq146_e1856_d_b6, eq146_e1856_d_b7, eq146_e1856_d_b8, eq146_e1856_d_b9, eq146_e1856_d_b10, eq146_e1856_d_b11, eq146_e1856_d_b12, eq146_e1856_d_b13, eq146_e1856_d_b14, eq146_e1856_d_b15, eq146_e1856_d_b16, eq146_e1856_d_b17, eq146_e1856_d_b18, eq146_e1856_d_b19, eq146_e1856_d_b20, eq146_e1856_d_b21, eq146_e1856_d_b22, eq146_e1856_d_b23, eq146_e1856_d_b24, eq146_e1856_d_b25, eq146_e1856_d_b26, eq146_e1856_d_b27, eq146_e1856_d_b28, eq146_e1856_d_b29, eq146_e1856_d_b30, eq146_e1856_d_b31, eq146_e1856_d_b32, eq146_e1856_d_b33, eq146_e1856_d_b34, eq146_e1856_d_b35, eq146_e1856_d_b36, eq146_e1856_d_b37, eq146_e1856_d_b38, eq146_e1856_d_b39, eq146_e1856_d_b40, eq146_e1856_d_b41, eq146_e1856_d_b42, eq146_e1856_d_b43, eq146_e1856_d_b44, eq146_e1856_d_b45, eq146_e1856_d_b46, eq146_e1856_d_b47, eq146_e1856_d_b48, eq146_e1856_d_b49, eq146_e1856_d_b50, eq146_e1856_d_b51, eq146_e1856_d_b52, eq146_e1856_d_b53, eq146_e1856_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq146_reactive_node_derivatives,
            branches,
            &eq146_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22, eq147_e1868_d_b0, eq147_e1868_d_b1, eq147_e1868_d_b2, eq147_e1868_d_b3, eq147_e1868_d_b4, eq147_e1868_d_b5, eq147_e1868_d_b6, eq147_e1868_d_b7, eq147_e1868_d_b8, eq147_e1868_d_b9, eq147_e1868_d_b10, eq147_e1868_d_b11, eq147_e1868_d_b12, eq147_e1868_d_b13, eq147_e1868_d_b14, eq147_e1868_d_b15, eq147_e1868_d_b16, eq147_e1868_d_b17, eq147_e1868_d_b18, eq147_e1868_d_b19, eq147_e1868_d_b20, eq147_e1868_d_b21, eq147_e1868_d_b22, eq147_e1868_d_b23, eq147_e1868_d_b24, eq147_e1868_d_b25, eq147_e1868_d_b26, eq147_e1868_d_b27, eq147_e1868_d_b28, eq147_e1868_d_b29, eq147_e1868_d_b30, eq147_e1868_d_b31, eq147_e1868_d_b32, eq147_e1868_d_b33, eq147_e1868_d_b34, eq147_e1868_d_b35, eq147_e1868_d_b36, eq147_e1868_d_b37, eq147_e1868_d_b38, eq147_e1868_d_b39, eq147_e1868_d_b40, eq147_e1868_d_b41, eq147_e1868_d_b42, eq147_e1868_d_b43, eq147_e1868_d_b44, eq147_e1868_d_b45, eq147_e1868_d_b46, eq147_e1868_d_b47, eq147_e1868_d_b48, eq147_e1868_d_b49, eq147_e1868_d_b50, eq147_e1868_d_b51, eq147_e1868_d_b52, eq147_e1868_d_b53, eq147_e1868_d_b54, eq147_e1868_q,) = {
    if ((s.b[580] && s.b[581]) && (!s.b[582])) {
        let eq147_e1865_q: f64 = s.v[252];
        let eq147_e1866: f64 = (p.p7 * s.v[252]);
        let eq147_e1866_q: f64 = (p.p7 * eq147_e1865_q);
        (eq147_e1866, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq147_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_reactive_node_derivatives: [f64; 23] = [eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22];
        let eq147_reactive_branch_derivatives: [f64; 55] = [eq147_e1868_d_b0, eq147_e1868_d_b1, eq147_e1868_d_b2, eq147_e1868_d_b3, eq147_e1868_d_b4, eq147_e1868_d_b5, eq147_e1868_d_b6, eq147_e1868_d_b7, eq147_e1868_d_b8, eq147_e1868_d_b9, eq147_e1868_d_b10, eq147_e1868_d_b11, eq147_e1868_d_b12, eq147_e1868_d_b13, eq147_e1868_d_b14, eq147_e1868_d_b15, eq147_e1868_d_b16, eq147_e1868_d_b17, eq147_e1868_d_b18, eq147_e1868_d_b19, eq147_e1868_d_b20, eq147_e1868_d_b21, eq147_e1868_d_b22, eq147_e1868_d_b23, eq147_e1868_d_b24, eq147_e1868_d_b25, eq147_e1868_d_b26, eq147_e1868_d_b27, eq147_e1868_d_b28, eq147_e1868_d_b29, eq147_e1868_d_b30, eq147_e1868_d_b31, eq147_e1868_d_b32, eq147_e1868_d_b33, eq147_e1868_d_b34, eq147_e1868_d_b35, eq147_e1868_d_b36, eq147_e1868_d_b37, eq147_e1868_d_b38, eq147_e1868_d_b39, eq147_e1868_d_b40, eq147_e1868_d_b41, eq147_e1868_d_b42, eq147_e1868_d_b43, eq147_e1868_d_b44, eq147_e1868_d_b45, eq147_e1868_d_b46, eq147_e1868_d_b47, eq147_e1868_d_b48, eq147_e1868_d_b49, eq147_e1868_d_b50, eq147_e1868_d_b51, eq147_e1868_d_b52, eq147_e1868_d_b53, eq147_e1868_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            nodes,
            &eq147_reactive_node_derivatives,
            branches,
            &eq147_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22, eq148_e1882_d_b0, eq148_e1882_d_b1, eq148_e1882_d_b2, eq148_e1882_d_b3, eq148_e1882_d_b4, eq148_e1882_d_b5, eq148_e1882_d_b6, eq148_e1882_d_b7, eq148_e1882_d_b8, eq148_e1882_d_b9, eq148_e1882_d_b10, eq148_e1882_d_b11, eq148_e1882_d_b12, eq148_e1882_d_b13, eq148_e1882_d_b14, eq148_e1882_d_b15, eq148_e1882_d_b16, eq148_e1882_d_b17, eq148_e1882_d_b18, eq148_e1882_d_b19, eq148_e1882_d_b20, eq148_e1882_d_b21, eq148_e1882_d_b22, eq148_e1882_d_b23, eq148_e1882_d_b24, eq148_e1882_d_b25, eq148_e1882_d_b26, eq148_e1882_d_b27, eq148_e1882_d_b28, eq148_e1882_d_b29, eq148_e1882_d_b30, eq148_e1882_d_b31, eq148_e1882_d_b32, eq148_e1882_d_b33, eq148_e1882_d_b34, eq148_e1882_d_b35, eq148_e1882_d_b36, eq148_e1882_d_b37, eq148_e1882_d_b38, eq148_e1882_d_b39, eq148_e1882_d_b40, eq148_e1882_d_b41, eq148_e1882_d_b42, eq148_e1882_d_b43, eq148_e1882_d_b44, eq148_e1882_d_b45, eq148_e1882_d_b46, eq148_e1882_d_b47, eq148_e1882_d_b48, eq148_e1882_d_b49, eq148_e1882_d_b50, eq148_e1882_d_b51, eq148_e1882_d_b52, eq148_e1882_d_b53, eq148_e1882_d_b54, eq148_e1882_q,) = {
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
        let eq148_e1880_d_b0: f64 = (eq148_e1877 * s.db[252][0]);
        let eq148_e1880_d_b1: f64 = (eq148_e1877 * s.db[252][1]);
        let eq148_e1880_d_b2: f64 = (eq148_e1877 * s.db[252][2]);
        let eq148_e1880_d_b3: f64 = (eq148_e1877 * s.db[252][3]);
        let eq148_e1880_d_b4: f64 = (eq148_e1877 * s.db[252][4]);
        let eq148_e1880_d_b5: f64 = (eq148_e1877 * s.db[252][5]);
        let eq148_e1880_d_b6: f64 = (eq148_e1877 * s.db[252][6]);
        let eq148_e1880_d_b7: f64 = (eq148_e1877 * s.db[252][7]);
        let eq148_e1880_d_b8: f64 = (eq148_e1877 * s.db[252][8]);
        let eq148_e1880_d_b9: f64 = (eq148_e1877 * s.db[252][9]);
        let eq148_e1880_d_b10: f64 = (eq148_e1877 * s.db[252][10]);
        let eq148_e1880_d_b11: f64 = (eq148_e1877 * s.db[252][11]);
        let eq148_e1880_d_b12: f64 = (eq148_e1877 * s.db[252][12]);
        let eq148_e1880_d_b13: f64 = (eq148_e1877 * s.db[252][13]);
        let eq148_e1880_d_b14: f64 = (eq148_e1877 * s.db[252][14]);
        let eq148_e1880_d_b15: f64 = (eq148_e1877 * s.db[252][15]);
        let eq148_e1880_d_b16: f64 = (eq148_e1877 * s.db[252][16]);
        let eq148_e1880_d_b17: f64 = (eq148_e1877 * s.db[252][17]);
        let eq148_e1880_d_b18: f64 = (eq148_e1877 * s.db[252][18]);
        let eq148_e1880_d_b19: f64 = (eq148_e1877 * s.db[252][19]);
        let eq148_e1880_d_b20: f64 = (eq148_e1877 * s.db[252][20]);
        let eq148_e1880_d_b21: f64 = (eq148_e1877 * s.db[252][21]);
        let eq148_e1880_d_b22: f64 = (eq148_e1877 * s.db[252][22]);
        let eq148_e1880_d_b23: f64 = (eq148_e1877 * s.db[252][23]);
        let eq148_e1880_d_b24: f64 = (eq148_e1877 * s.db[252][24]);
        let eq148_e1880_d_b25: f64 = (eq148_e1877 * s.db[252][25]);
        let eq148_e1880_d_b26: f64 = (eq148_e1877 * s.db[252][26]);
        let eq148_e1880_d_b27: f64 = (eq148_e1877 * s.db[252][27]);
        let eq148_e1880_d_b28: f64 = (eq148_e1877 * s.db[252][28]);
        let eq148_e1880_d_b29: f64 = (eq148_e1877 * s.db[252][29]);
        let eq148_e1880_d_b30: f64 = (eq148_e1877 * s.db[252][30]);
        let eq148_e1880_d_b31: f64 = (eq148_e1877 * s.db[252][31]);
        let eq148_e1880_d_b32: f64 = (eq148_e1877 * s.db[252][32]);
        let eq148_e1880_d_b33: f64 = (eq148_e1877 * s.db[252][33]);
        let eq148_e1880_d_b34: f64 = (eq148_e1877 * s.db[252][34]);
        let eq148_e1880_d_b35: f64 = (eq148_e1877 * s.db[252][35]);
        let eq148_e1880_d_b36: f64 = (eq148_e1877 * s.db[252][36]);
        let eq148_e1880_d_b37: f64 = (eq148_e1877 * s.db[252][37]);
        let eq148_e1880_d_b38: f64 = (eq148_e1877 * s.db[252][38]);
        let eq148_e1880_d_b39: f64 = (eq148_e1877 * s.db[252][39]);
        let eq148_e1880_d_b40: f64 = (eq148_e1877 * s.db[252][40]);
        let eq148_e1880_d_b41: f64 = (eq148_e1877 * s.db[252][41]);
        let eq148_e1880_d_b42: f64 = (eq148_e1877 * s.db[252][42]);
        let eq148_e1880_d_b43: f64 = (eq148_e1877 * s.db[252][43]);
        let eq148_e1880_d_b44: f64 = (eq148_e1877 * s.db[252][44]);
        let eq148_e1880_d_b45: f64 = (eq148_e1877 * s.db[252][45]);
        let eq148_e1880_d_b46: f64 = (eq148_e1877 * s.db[252][46]);
        let eq148_e1880_d_b47: f64 = (eq148_e1877 * s.db[252][47]);
        let eq148_e1880_d_b48: f64 = (eq148_e1877 * s.db[252][48]);
        let eq148_e1880_d_b49: f64 = (eq148_e1877 * s.db[252][49]);
        let eq148_e1880_d_b50: f64 = (eq148_e1877 * s.db[252][50]);
        let eq148_e1880_d_b51: f64 = (eq148_e1877 * s.db[252][51]);
        let eq148_e1880_d_b52: f64 = (eq148_e1877 * s.db[252][52]);
        let eq148_e1880_d_b53: f64 = (eq148_e1877 * s.db[252][53]);
        let eq148_e1880_d_b54: f64 = (eq148_e1877 * s.db[252][54]);
        let eq148_e1880_q: f64 = (eq148_e1877 * eq148_e1879_q);
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n10, eq148_e1880_d_n11, eq148_e1880_d_n12, eq148_e1880_d_n13, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22, eq148_e1880_d_b0, eq148_e1880_d_b1, eq148_e1880_d_b2, eq148_e1880_d_b3, eq148_e1880_d_b4, eq148_e1880_d_b5, eq148_e1880_d_b6, eq148_e1880_d_b7, eq148_e1880_d_b8, eq148_e1880_d_b9, eq148_e1880_d_b10, eq148_e1880_d_b11, eq148_e1880_d_b12, eq148_e1880_d_b13, eq148_e1880_d_b14, eq148_e1880_d_b15, eq148_e1880_d_b16, eq148_e1880_d_b17, eq148_e1880_d_b18, eq148_e1880_d_b19, eq148_e1880_d_b20, eq148_e1880_d_b21, eq148_e1880_d_b22, eq148_e1880_d_b23, eq148_e1880_d_b24, eq148_e1880_d_b25, eq148_e1880_d_b26, eq148_e1880_d_b27, eq148_e1880_d_b28, eq148_e1880_d_b29, eq148_e1880_d_b30, eq148_e1880_d_b31, eq148_e1880_d_b32, eq148_e1880_d_b33, eq148_e1880_d_b34, eq148_e1880_d_b35, eq148_e1880_d_b36, eq148_e1880_d_b37, eq148_e1880_d_b38, eq148_e1880_d_b39, eq148_e1880_d_b40, eq148_e1880_d_b41, eq148_e1880_d_b42, eq148_e1880_d_b43, eq148_e1880_d_b44, eq148_e1880_d_b45, eq148_e1880_d_b46, eq148_e1880_d_b47, eq148_e1880_d_b48, eq148_e1880_d_b49, eq148_e1880_d_b50, eq148_e1880_d_b51, eq148_e1880_d_b52, eq148_e1880_d_b53, eq148_e1880_d_b54, eq148_e1880_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_reactive_node_derivatives: [f64; 23] = [eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22];
        let eq148_reactive_branch_derivatives: [f64; 55] = [eq148_e1882_d_b0, eq148_e1882_d_b1, eq148_e1882_d_b2, eq148_e1882_d_b3, eq148_e1882_d_b4, eq148_e1882_d_b5, eq148_e1882_d_b6, eq148_e1882_d_b7, eq148_e1882_d_b8, eq148_e1882_d_b9, eq148_e1882_d_b10, eq148_e1882_d_b11, eq148_e1882_d_b12, eq148_e1882_d_b13, eq148_e1882_d_b14, eq148_e1882_d_b15, eq148_e1882_d_b16, eq148_e1882_d_b17, eq148_e1882_d_b18, eq148_e1882_d_b19, eq148_e1882_d_b20, eq148_e1882_d_b21, eq148_e1882_d_b22, eq148_e1882_d_b23, eq148_e1882_d_b24, eq148_e1882_d_b25, eq148_e1882_d_b26, eq148_e1882_d_b27, eq148_e1882_d_b28, eq148_e1882_d_b29, eq148_e1882_d_b30, eq148_e1882_d_b31, eq148_e1882_d_b32, eq148_e1882_d_b33, eq148_e1882_d_b34, eq148_e1882_d_b35, eq148_e1882_d_b36, eq148_e1882_d_b37, eq148_e1882_d_b38, eq148_e1882_d_b39, eq148_e1882_d_b40, eq148_e1882_d_b41, eq148_e1882_d_b42, eq148_e1882_d_b43, eq148_e1882_d_b44, eq148_e1882_d_b45, eq148_e1882_d_b46, eq148_e1882_d_b47, eq148_e1882_d_b48, eq148_e1882_d_b49, eq148_e1882_d_b50, eq148_e1882_d_b51, eq148_e1882_d_b52, eq148_e1882_d_b53, eq148_e1882_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            nodes,
            &eq148_reactive_node_derivatives,
            branches,
            &eq148_reactive_branch_derivatives,
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
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22, eq149_e1893_d_b0, eq149_e1893_d_b1, eq149_e1893_d_b2, eq149_e1893_d_b3, eq149_e1893_d_b4, eq149_e1893_d_b5, eq149_e1893_d_b6, eq149_e1893_d_b7, eq149_e1893_d_b8, eq149_e1893_d_b9, eq149_e1893_d_b10, eq149_e1893_d_b11, eq149_e1893_d_b12, eq149_e1893_d_b13, eq149_e1893_d_b14, eq149_e1893_d_b15, eq149_e1893_d_b16, eq149_e1893_d_b17, eq149_e1893_d_b18, eq149_e1893_d_b19, eq149_e1893_d_b20, eq149_e1893_d_b21, eq149_e1893_d_b22, eq149_e1893_d_b23, eq149_e1893_d_b24, eq149_e1893_d_b25, eq149_e1893_d_b26, eq149_e1893_d_b27, eq149_e1893_d_b28, eq149_e1893_d_b29, eq149_e1893_d_b30, eq149_e1893_d_b31, eq149_e1893_d_b32, eq149_e1893_d_b33, eq149_e1893_d_b34, eq149_e1893_d_b35, eq149_e1893_d_b36, eq149_e1893_d_b37, eq149_e1893_d_b38, eq149_e1893_d_b39, eq149_e1893_d_b40, eq149_e1893_d_b41, eq149_e1893_d_b42, eq149_e1893_d_b43, eq149_e1893_d_b44, eq149_e1893_d_b45, eq149_e1893_d_b46, eq149_e1893_d_b47, eq149_e1893_d_b48, eq149_e1893_d_b49, eq149_e1893_d_b50, eq149_e1893_d_b51, eq149_e1893_d_b52, eq149_e1893_d_b53, eq149_e1893_d_b54, eq149_e1893_q,) = {
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
        let eq149_e1889_d_b0: f64 = (p.p252 * s.db[252][0]);
        let eq149_e1889_d_b1: f64 = (p.p252 * s.db[252][1]);
        let eq149_e1889_d_b2: f64 = (p.p252 * s.db[252][2]);
        let eq149_e1889_d_b3: f64 = (p.p252 * s.db[252][3]);
        let eq149_e1889_d_b4: f64 = (p.p252 * s.db[252][4]);
        let eq149_e1889_d_b5: f64 = (p.p252 * s.db[252][5]);
        let eq149_e1889_d_b6: f64 = (p.p252 * s.db[252][6]);
        let eq149_e1889_d_b7: f64 = (p.p252 * s.db[252][7]);
        let eq149_e1889_d_b8: f64 = (p.p252 * s.db[252][8]);
        let eq149_e1889_d_b9: f64 = (p.p252 * s.db[252][9]);
        let eq149_e1889_d_b10: f64 = (p.p252 * s.db[252][10]);
        let eq149_e1889_d_b11: f64 = (p.p252 * s.db[252][11]);
        let eq149_e1889_d_b12: f64 = (p.p252 * s.db[252][12]);
        let eq149_e1889_d_b13: f64 = (p.p252 * s.db[252][13]);
        let eq149_e1889_d_b14: f64 = (p.p252 * s.db[252][14]);
        let eq149_e1889_d_b15: f64 = (p.p252 * s.db[252][15]);
        let eq149_e1889_d_b16: f64 = (p.p252 * s.db[252][16]);
        let eq149_e1889_d_b17: f64 = (p.p252 * s.db[252][17]);
        let eq149_e1889_d_b18: f64 = (p.p252 * s.db[252][18]);
        let eq149_e1889_d_b19: f64 = (p.p252 * s.db[252][19]);
        let eq149_e1889_d_b20: f64 = (p.p252 * s.db[252][20]);
        let eq149_e1889_d_b21: f64 = (p.p252 * s.db[252][21]);
        let eq149_e1889_d_b22: f64 = (p.p252 * s.db[252][22]);
        let eq149_e1889_d_b23: f64 = (p.p252 * s.db[252][23]);
        let eq149_e1889_d_b24: f64 = (p.p252 * s.db[252][24]);
        let eq149_e1889_d_b25: f64 = (p.p252 * s.db[252][25]);
        let eq149_e1889_d_b26: f64 = (p.p252 * s.db[252][26]);
        let eq149_e1889_d_b27: f64 = (p.p252 * s.db[252][27]);
        let eq149_e1889_d_b28: f64 = (p.p252 * s.db[252][28]);
        let eq149_e1889_d_b29: f64 = (p.p252 * s.db[252][29]);
        let eq149_e1889_d_b30: f64 = (p.p252 * s.db[252][30]);
        let eq149_e1889_d_b31: f64 = (p.p252 * s.db[252][31]);
        let eq149_e1889_d_b32: f64 = (p.p252 * s.db[252][32]);
        let eq149_e1889_d_b33: f64 = (p.p252 * s.db[252][33]);
        let eq149_e1889_d_b34: f64 = (p.p252 * s.db[252][34]);
        let eq149_e1889_d_b35: f64 = (p.p252 * s.db[252][35]);
        let eq149_e1889_d_b36: f64 = (p.p252 * s.db[252][36]);
        let eq149_e1889_d_b37: f64 = (p.p252 * s.db[252][37]);
        let eq149_e1889_d_b38: f64 = (p.p252 * s.db[252][38]);
        let eq149_e1889_d_b39: f64 = (p.p252 * s.db[252][39]);
        let eq149_e1889_d_b40: f64 = (p.p252 * s.db[252][40]);
        let eq149_e1889_d_b41: f64 = (p.p252 * s.db[252][41]);
        let eq149_e1889_d_b42: f64 = (p.p252 * s.db[252][42]);
        let eq149_e1889_d_b43: f64 = (p.p252 * s.db[252][43]);
        let eq149_e1889_d_b44: f64 = (p.p252 * s.db[252][44]);
        let eq149_e1889_d_b45: f64 = (p.p252 * s.db[252][45]);
        let eq149_e1889_d_b46: f64 = (p.p252 * s.db[252][46]);
        let eq149_e1889_d_b47: f64 = (p.p252 * s.db[252][47]);
        let eq149_e1889_d_b48: f64 = (p.p252 * s.db[252][48]);
        let eq149_e1889_d_b49: f64 = (p.p252 * s.db[252][49]);
        let eq149_e1889_d_b50: f64 = (p.p252 * s.db[252][50]);
        let eq149_e1889_d_b51: f64 = (p.p252 * s.db[252][51]);
        let eq149_e1889_d_b52: f64 = (p.p252 * s.db[252][52]);
        let eq149_e1889_d_b53: f64 = (p.p252 * s.db[252][53]);
        let eq149_e1889_d_b54: f64 = (p.p252 * s.db[252][54]);
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
        let eq149_e1891_d_b0: f64 = (p.p7 * eq149_e1889_d_b0);
        let eq149_e1891_d_b1: f64 = (p.p7 * eq149_e1889_d_b1);
        let eq149_e1891_d_b2: f64 = (p.p7 * eq149_e1889_d_b2);
        let eq149_e1891_d_b3: f64 = (p.p7 * eq149_e1889_d_b3);
        let eq149_e1891_d_b4: f64 = (p.p7 * eq149_e1889_d_b4);
        let eq149_e1891_d_b5: f64 = (p.p7 * eq149_e1889_d_b5);
        let eq149_e1891_d_b6: f64 = (p.p7 * eq149_e1889_d_b6);
        let eq149_e1891_d_b7: f64 = (p.p7 * eq149_e1889_d_b7);
        let eq149_e1891_d_b8: f64 = (p.p7 * eq149_e1889_d_b8);
        let eq149_e1891_d_b9: f64 = (p.p7 * eq149_e1889_d_b9);
        let eq149_e1891_d_b10: f64 = (p.p7 * eq149_e1889_d_b10);
        let eq149_e1891_d_b11: f64 = (p.p7 * eq149_e1889_d_b11);
        let eq149_e1891_d_b12: f64 = (p.p7 * eq149_e1889_d_b12);
        let eq149_e1891_d_b13: f64 = (p.p7 * eq149_e1889_d_b13);
        let eq149_e1891_d_b14: f64 = (p.p7 * eq149_e1889_d_b14);
        let eq149_e1891_d_b15: f64 = (p.p7 * eq149_e1889_d_b15);
        let eq149_e1891_d_b16: f64 = (p.p7 * eq149_e1889_d_b16);
        let eq149_e1891_d_b17: f64 = (p.p7 * eq149_e1889_d_b17);
        let eq149_e1891_d_b18: f64 = (p.p7 * eq149_e1889_d_b18);
        let eq149_e1891_d_b19: f64 = (p.p7 * eq149_e1889_d_b19);
        let eq149_e1891_d_b20: f64 = (p.p7 * eq149_e1889_d_b20);
        let eq149_e1891_d_b21: f64 = (p.p7 * eq149_e1889_d_b21);
        let eq149_e1891_d_b22: f64 = (p.p7 * eq149_e1889_d_b22);
        let eq149_e1891_d_b23: f64 = (p.p7 * eq149_e1889_d_b23);
        let eq149_e1891_d_b24: f64 = (p.p7 * eq149_e1889_d_b24);
        let eq149_e1891_d_b25: f64 = (p.p7 * eq149_e1889_d_b25);
        let eq149_e1891_d_b26: f64 = (p.p7 * eq149_e1889_d_b26);
        let eq149_e1891_d_b27: f64 = (p.p7 * eq149_e1889_d_b27);
        let eq149_e1891_d_b28: f64 = (p.p7 * eq149_e1889_d_b28);
        let eq149_e1891_d_b29: f64 = (p.p7 * eq149_e1889_d_b29);
        let eq149_e1891_d_b30: f64 = (p.p7 * eq149_e1889_d_b30);
        let eq149_e1891_d_b31: f64 = (p.p7 * eq149_e1889_d_b31);
        let eq149_e1891_d_b32: f64 = (p.p7 * eq149_e1889_d_b32);
        let eq149_e1891_d_b33: f64 = (p.p7 * eq149_e1889_d_b33);
        let eq149_e1891_d_b34: f64 = (p.p7 * eq149_e1889_d_b34);
        let eq149_e1891_d_b35: f64 = (p.p7 * eq149_e1889_d_b35);
        let eq149_e1891_d_b36: f64 = (p.p7 * eq149_e1889_d_b36);
        let eq149_e1891_d_b37: f64 = (p.p7 * eq149_e1889_d_b37);
        let eq149_e1891_d_b38: f64 = (p.p7 * eq149_e1889_d_b38);
        let eq149_e1891_d_b39: f64 = (p.p7 * eq149_e1889_d_b39);
        let eq149_e1891_d_b40: f64 = (p.p7 * eq149_e1889_d_b40);
        let eq149_e1891_d_b41: f64 = (p.p7 * eq149_e1889_d_b41);
        let eq149_e1891_d_b42: f64 = (p.p7 * eq149_e1889_d_b42);
        let eq149_e1891_d_b43: f64 = (p.p7 * eq149_e1889_d_b43);
        let eq149_e1891_d_b44: f64 = (p.p7 * eq149_e1889_d_b44);
        let eq149_e1891_d_b45: f64 = (p.p7 * eq149_e1889_d_b45);
        let eq149_e1891_d_b46: f64 = (p.p7 * eq149_e1889_d_b46);
        let eq149_e1891_d_b47: f64 = (p.p7 * eq149_e1889_d_b47);
        let eq149_e1891_d_b48: f64 = (p.p7 * eq149_e1889_d_b48);
        let eq149_e1891_d_b49: f64 = (p.p7 * eq149_e1889_d_b49);
        let eq149_e1891_d_b50: f64 = (p.p7 * eq149_e1889_d_b50);
        let eq149_e1891_d_b51: f64 = (p.p7 * eq149_e1889_d_b51);
        let eq149_e1891_d_b52: f64 = (p.p7 * eq149_e1889_d_b52);
        let eq149_e1891_d_b53: f64 = (p.p7 * eq149_e1889_d_b53);
        let eq149_e1891_d_b54: f64 = (p.p7 * eq149_e1889_d_b54);
        let eq149_e1891_q: f64 = (p.p7 * eq149_e1890_q);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n10, eq149_e1891_d_n11, eq149_e1891_d_n12, eq149_e1891_d_n13, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22, eq149_e1891_d_b0, eq149_e1891_d_b1, eq149_e1891_d_b2, eq149_e1891_d_b3, eq149_e1891_d_b4, eq149_e1891_d_b5, eq149_e1891_d_b6, eq149_e1891_d_b7, eq149_e1891_d_b8, eq149_e1891_d_b9, eq149_e1891_d_b10, eq149_e1891_d_b11, eq149_e1891_d_b12, eq149_e1891_d_b13, eq149_e1891_d_b14, eq149_e1891_d_b15, eq149_e1891_d_b16, eq149_e1891_d_b17, eq149_e1891_d_b18, eq149_e1891_d_b19, eq149_e1891_d_b20, eq149_e1891_d_b21, eq149_e1891_d_b22, eq149_e1891_d_b23, eq149_e1891_d_b24, eq149_e1891_d_b25, eq149_e1891_d_b26, eq149_e1891_d_b27, eq149_e1891_d_b28, eq149_e1891_d_b29, eq149_e1891_d_b30, eq149_e1891_d_b31, eq149_e1891_d_b32, eq149_e1891_d_b33, eq149_e1891_d_b34, eq149_e1891_d_b35, eq149_e1891_d_b36, eq149_e1891_d_b37, eq149_e1891_d_b38, eq149_e1891_d_b39, eq149_e1891_d_b40, eq149_e1891_d_b41, eq149_e1891_d_b42, eq149_e1891_d_b43, eq149_e1891_d_b44, eq149_e1891_d_b45, eq149_e1891_d_b46, eq149_e1891_d_b47, eq149_e1891_d_b48, eq149_e1891_d_b49, eq149_e1891_d_b50, eq149_e1891_d_b51, eq149_e1891_d_b52, eq149_e1891_d_b53, eq149_e1891_d_b54, eq149_e1891_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_reactive_node_derivatives: [f64; 23] = [eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22];
        let eq149_reactive_branch_derivatives: [f64; 55] = [eq149_e1893_d_b0, eq149_e1893_d_b1, eq149_e1893_d_b2, eq149_e1893_d_b3, eq149_e1893_d_b4, eq149_e1893_d_b5, eq149_e1893_d_b6, eq149_e1893_d_b7, eq149_e1893_d_b8, eq149_e1893_d_b9, eq149_e1893_d_b10, eq149_e1893_d_b11, eq149_e1893_d_b12, eq149_e1893_d_b13, eq149_e1893_d_b14, eq149_e1893_d_b15, eq149_e1893_d_b16, eq149_e1893_d_b17, eq149_e1893_d_b18, eq149_e1893_d_b19, eq149_e1893_d_b20, eq149_e1893_d_b21, eq149_e1893_d_b22, eq149_e1893_d_b23, eq149_e1893_d_b24, eq149_e1893_d_b25, eq149_e1893_d_b26, eq149_e1893_d_b27, eq149_e1893_d_b28, eq149_e1893_d_b29, eq149_e1893_d_b30, eq149_e1893_d_b31, eq149_e1893_d_b32, eq149_e1893_d_b33, eq149_e1893_d_b34, eq149_e1893_d_b35, eq149_e1893_d_b36, eq149_e1893_d_b37, eq149_e1893_d_b38, eq149_e1893_d_b39, eq149_e1893_d_b40, eq149_e1893_d_b41, eq149_e1893_d_b42, eq149_e1893_d_b43, eq149_e1893_d_b44, eq149_e1893_d_b45, eq149_e1893_d_b46, eq149_e1893_d_b47, eq149_e1893_d_b48, eq149_e1893_d_b49, eq149_e1893_d_b50, eq149_e1893_d_b51, eq149_e1893_d_b52, eq149_e1893_d_b53, eq149_e1893_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            nodes,
            &eq149_reactive_node_derivatives,
            branches,
            &eq149_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22, eq150_e1903_d_b0, eq150_e1903_d_b1, eq150_e1903_d_b2, eq150_e1903_d_b3, eq150_e1903_d_b4, eq150_e1903_d_b5, eq150_e1903_d_b6, eq150_e1903_d_b7, eq150_e1903_d_b8, eq150_e1903_d_b9, eq150_e1903_d_b10, eq150_e1903_d_b11, eq150_e1903_d_b12, eq150_e1903_d_b13, eq150_e1903_d_b14, eq150_e1903_d_b15, eq150_e1903_d_b16, eq150_e1903_d_b17, eq150_e1903_d_b18, eq150_e1903_d_b19, eq150_e1903_d_b20, eq150_e1903_d_b21, eq150_e1903_d_b22, eq150_e1903_d_b23, eq150_e1903_d_b24, eq150_e1903_d_b25, eq150_e1903_d_b26, eq150_e1903_d_b27, eq150_e1903_d_b28, eq150_e1903_d_b29, eq150_e1903_d_b30, eq150_e1903_d_b31, eq150_e1903_d_b32, eq150_e1903_d_b33, eq150_e1903_d_b34, eq150_e1903_d_b35, eq150_e1903_d_b36, eq150_e1903_d_b37, eq150_e1903_d_b38, eq150_e1903_d_b39, eq150_e1903_d_b40, eq150_e1903_d_b41, eq150_e1903_d_b42, eq150_e1903_d_b43, eq150_e1903_d_b44, eq150_e1903_d_b45, eq150_e1903_d_b46, eq150_e1903_d_b47, eq150_e1903_d_b48, eq150_e1903_d_b49, eq150_e1903_d_b50, eq150_e1903_d_b51, eq150_e1903_d_b52, eq150_e1903_d_b53, eq150_e1903_d_b54, eq150_e1903_q,) = {
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
        let eq150_e1901_d_b0: f64 = (p.p7 * s.db[253][0]);
        let eq150_e1901_d_b1: f64 = (p.p7 * s.db[253][1]);
        let eq150_e1901_d_b2: f64 = (p.p7 * s.db[253][2]);
        let eq150_e1901_d_b3: f64 = (p.p7 * s.db[253][3]);
        let eq150_e1901_d_b4: f64 = (p.p7 * s.db[253][4]);
        let eq150_e1901_d_b5: f64 = (p.p7 * s.db[253][5]);
        let eq150_e1901_d_b6: f64 = (p.p7 * s.db[253][6]);
        let eq150_e1901_d_b7: f64 = (p.p7 * s.db[253][7]);
        let eq150_e1901_d_b8: f64 = (p.p7 * s.db[253][8]);
        let eq150_e1901_d_b9: f64 = (p.p7 * s.db[253][9]);
        let eq150_e1901_d_b10: f64 = (p.p7 * s.db[253][10]);
        let eq150_e1901_d_b11: f64 = (p.p7 * s.db[253][11]);
        let eq150_e1901_d_b12: f64 = (p.p7 * s.db[253][12]);
        let eq150_e1901_d_b13: f64 = (p.p7 * s.db[253][13]);
        let eq150_e1901_d_b14: f64 = (p.p7 * s.db[253][14]);
        let eq150_e1901_d_b15: f64 = (p.p7 * s.db[253][15]);
        let eq150_e1901_d_b16: f64 = (p.p7 * s.db[253][16]);
        let eq150_e1901_d_b17: f64 = (p.p7 * s.db[253][17]);
        let eq150_e1901_d_b18: f64 = (p.p7 * s.db[253][18]);
        let eq150_e1901_d_b19: f64 = (p.p7 * s.db[253][19]);
        let eq150_e1901_d_b20: f64 = (p.p7 * s.db[253][20]);
        let eq150_e1901_d_b21: f64 = (p.p7 * s.db[253][21]);
        let eq150_e1901_d_b22: f64 = (p.p7 * s.db[253][22]);
        let eq150_e1901_d_b23: f64 = (p.p7 * s.db[253][23]);
        let eq150_e1901_d_b24: f64 = (p.p7 * s.db[253][24]);
        let eq150_e1901_d_b25: f64 = (p.p7 * s.db[253][25]);
        let eq150_e1901_d_b26: f64 = (p.p7 * s.db[253][26]);
        let eq150_e1901_d_b27: f64 = (p.p7 * s.db[253][27]);
        let eq150_e1901_d_b28: f64 = (p.p7 * s.db[253][28]);
        let eq150_e1901_d_b29: f64 = (p.p7 * s.db[253][29]);
        let eq150_e1901_d_b30: f64 = (p.p7 * s.db[253][30]);
        let eq150_e1901_d_b31: f64 = (p.p7 * s.db[253][31]);
        let eq150_e1901_d_b32: f64 = (p.p7 * s.db[253][32]);
        let eq150_e1901_d_b33: f64 = (p.p7 * s.db[253][33]);
        let eq150_e1901_d_b34: f64 = (p.p7 * s.db[253][34]);
        let eq150_e1901_d_b35: f64 = (p.p7 * s.db[253][35]);
        let eq150_e1901_d_b36: f64 = (p.p7 * s.db[253][36]);
        let eq150_e1901_d_b37: f64 = (p.p7 * s.db[253][37]);
        let eq150_e1901_d_b38: f64 = (p.p7 * s.db[253][38]);
        let eq150_e1901_d_b39: f64 = (p.p7 * s.db[253][39]);
        let eq150_e1901_d_b40: f64 = (p.p7 * s.db[253][40]);
        let eq150_e1901_d_b41: f64 = (p.p7 * s.db[253][41]);
        let eq150_e1901_d_b42: f64 = (p.p7 * s.db[253][42]);
        let eq150_e1901_d_b43: f64 = (p.p7 * s.db[253][43]);
        let eq150_e1901_d_b44: f64 = (p.p7 * s.db[253][44]);
        let eq150_e1901_d_b45: f64 = (p.p7 * s.db[253][45]);
        let eq150_e1901_d_b46: f64 = (p.p7 * s.db[253][46]);
        let eq150_e1901_d_b47: f64 = (p.p7 * s.db[253][47]);
        let eq150_e1901_d_b48: f64 = (p.p7 * s.db[253][48]);
        let eq150_e1901_d_b49: f64 = (p.p7 * s.db[253][49]);
        let eq150_e1901_d_b50: f64 = (p.p7 * s.db[253][50]);
        let eq150_e1901_d_b51: f64 = (p.p7 * s.db[253][51]);
        let eq150_e1901_d_b52: f64 = (p.p7 * s.db[253][52]);
        let eq150_e1901_d_b53: f64 = (p.p7 * s.db[253][53]);
        let eq150_e1901_d_b54: f64 = (p.p7 * s.db[253][54]);
        let eq150_e1901_q: f64 = (p.p7 * eq150_e1900_q);
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n10, eq150_e1901_d_n11, eq150_e1901_d_n12, eq150_e1901_d_n13, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22, eq150_e1901_d_b0, eq150_e1901_d_b1, eq150_e1901_d_b2, eq150_e1901_d_b3, eq150_e1901_d_b4, eq150_e1901_d_b5, eq150_e1901_d_b6, eq150_e1901_d_b7, eq150_e1901_d_b8, eq150_e1901_d_b9, eq150_e1901_d_b10, eq150_e1901_d_b11, eq150_e1901_d_b12, eq150_e1901_d_b13, eq150_e1901_d_b14, eq150_e1901_d_b15, eq150_e1901_d_b16, eq150_e1901_d_b17, eq150_e1901_d_b18, eq150_e1901_d_b19, eq150_e1901_d_b20, eq150_e1901_d_b21, eq150_e1901_d_b22, eq150_e1901_d_b23, eq150_e1901_d_b24, eq150_e1901_d_b25, eq150_e1901_d_b26, eq150_e1901_d_b27, eq150_e1901_d_b28, eq150_e1901_d_b29, eq150_e1901_d_b30, eq150_e1901_d_b31, eq150_e1901_d_b32, eq150_e1901_d_b33, eq150_e1901_d_b34, eq150_e1901_d_b35, eq150_e1901_d_b36, eq150_e1901_d_b37, eq150_e1901_d_b38, eq150_e1901_d_b39, eq150_e1901_d_b40, eq150_e1901_d_b41, eq150_e1901_d_b42, eq150_e1901_d_b43, eq150_e1901_d_b44, eq150_e1901_d_b45, eq150_e1901_d_b46, eq150_e1901_d_b47, eq150_e1901_d_b48, eq150_e1901_d_b49, eq150_e1901_d_b50, eq150_e1901_d_b51, eq150_e1901_d_b52, eq150_e1901_d_b53, eq150_e1901_d_b54, eq150_e1901_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_reactive_node_derivatives: [f64; 23] = [eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22];
        let eq150_reactive_branch_derivatives: [f64; 55] = [eq150_e1903_d_b0, eq150_e1903_d_b1, eq150_e1903_d_b2, eq150_e1903_d_b3, eq150_e1903_d_b4, eq150_e1903_d_b5, eq150_e1903_d_b6, eq150_e1903_d_b7, eq150_e1903_d_b8, eq150_e1903_d_b9, eq150_e1903_d_b10, eq150_e1903_d_b11, eq150_e1903_d_b12, eq150_e1903_d_b13, eq150_e1903_d_b14, eq150_e1903_d_b15, eq150_e1903_d_b16, eq150_e1903_d_b17, eq150_e1903_d_b18, eq150_e1903_d_b19, eq150_e1903_d_b20, eq150_e1903_d_b21, eq150_e1903_d_b22, eq150_e1903_d_b23, eq150_e1903_d_b24, eq150_e1903_d_b25, eq150_e1903_d_b26, eq150_e1903_d_b27, eq150_e1903_d_b28, eq150_e1903_d_b29, eq150_e1903_d_b30, eq150_e1903_d_b31, eq150_e1903_d_b32, eq150_e1903_d_b33, eq150_e1903_d_b34, eq150_e1903_d_b35, eq150_e1903_d_b36, eq150_e1903_d_b37, eq150_e1903_d_b38, eq150_e1903_d_b39, eq150_e1903_d_b40, eq150_e1903_d_b41, eq150_e1903_d_b42, eq150_e1903_d_b43, eq150_e1903_d_b44, eq150_e1903_d_b45, eq150_e1903_d_b46, eq150_e1903_d_b47, eq150_e1903_d_b48, eq150_e1903_d_b49, eq150_e1903_d_b50, eq150_e1903_d_b51, eq150_e1903_d_b52, eq150_e1903_d_b53, eq150_e1903_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            nodes,
            &eq150_reactive_node_derivatives,
            branches,
            &eq150_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq151_e1915, eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22, eq151_e1915_d_b0, eq151_e1915_d_b1, eq151_e1915_d_b2, eq151_e1915_d_b3, eq151_e1915_d_b4, eq151_e1915_d_b5, eq151_e1915_d_b6, eq151_e1915_d_b7, eq151_e1915_d_b8, eq151_e1915_d_b9, eq151_e1915_d_b10, eq151_e1915_d_b11, eq151_e1915_d_b12, eq151_e1915_d_b13, eq151_e1915_d_b14, eq151_e1915_d_b15, eq151_e1915_d_b16, eq151_e1915_d_b17, eq151_e1915_d_b18, eq151_e1915_d_b19, eq151_e1915_d_b20, eq151_e1915_d_b21, eq151_e1915_d_b22, eq151_e1915_d_b23, eq151_e1915_d_b24, eq151_e1915_d_b25, eq151_e1915_d_b26, eq151_e1915_d_b27, eq151_e1915_d_b28, eq151_e1915_d_b29, eq151_e1915_d_b30, eq151_e1915_d_b31, eq151_e1915_d_b32, eq151_e1915_d_b33, eq151_e1915_d_b34, eq151_e1915_d_b35, eq151_e1915_d_b36, eq151_e1915_d_b37, eq151_e1915_d_b38, eq151_e1915_d_b39, eq151_e1915_d_b40, eq151_e1915_d_b41, eq151_e1915_d_b42, eq151_e1915_d_b43, eq151_e1915_d_b44, eq151_e1915_d_b45, eq151_e1915_d_b46, eq151_e1915_d_b47, eq151_e1915_d_b48, eq151_e1915_d_b49, eq151_e1915_d_b50, eq151_e1915_d_b51, eq151_e1915_d_b52, eq151_e1915_d_b53, eq151_e1915_d_b54, eq151_e1915_q,) = {
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
        let eq151_e1913_d_b0: f64 = (p.p7 * s.db[252][0]);
        let eq151_e1913_d_b1: f64 = (p.p7 * s.db[252][1]);
        let eq151_e1913_d_b2: f64 = (p.p7 * s.db[252][2]);
        let eq151_e1913_d_b3: f64 = (p.p7 * s.db[252][3]);
        let eq151_e1913_d_b4: f64 = (p.p7 * s.db[252][4]);
        let eq151_e1913_d_b5: f64 = (p.p7 * s.db[252][5]);
        let eq151_e1913_d_b6: f64 = (p.p7 * s.db[252][6]);
        let eq151_e1913_d_b7: f64 = (p.p7 * s.db[252][7]);
        let eq151_e1913_d_b8: f64 = (p.p7 * s.db[252][8]);
        let eq151_e1913_d_b9: f64 = (p.p7 * s.db[252][9]);
        let eq151_e1913_d_b10: f64 = (p.p7 * s.db[252][10]);
        let eq151_e1913_d_b11: f64 = (p.p7 * s.db[252][11]);
        let eq151_e1913_d_b12: f64 = (p.p7 * s.db[252][12]);
        let eq151_e1913_d_b13: f64 = (p.p7 * s.db[252][13]);
        let eq151_e1913_d_b14: f64 = (p.p7 * s.db[252][14]);
        let eq151_e1913_d_b15: f64 = (p.p7 * s.db[252][15]);
        let eq151_e1913_d_b16: f64 = (p.p7 * s.db[252][16]);
        let eq151_e1913_d_b17: f64 = (p.p7 * s.db[252][17]);
        let eq151_e1913_d_b18: f64 = (p.p7 * s.db[252][18]);
        let eq151_e1913_d_b19: f64 = (p.p7 * s.db[252][19]);
        let eq151_e1913_d_b20: f64 = (p.p7 * s.db[252][20]);
        let eq151_e1913_d_b21: f64 = (p.p7 * s.db[252][21]);
        let eq151_e1913_d_b22: f64 = (p.p7 * s.db[252][22]);
        let eq151_e1913_d_b23: f64 = (p.p7 * s.db[252][23]);
        let eq151_e1913_d_b24: f64 = (p.p7 * s.db[252][24]);
        let eq151_e1913_d_b25: f64 = (p.p7 * s.db[252][25]);
        let eq151_e1913_d_b26: f64 = (p.p7 * s.db[252][26]);
        let eq151_e1913_d_b27: f64 = (p.p7 * s.db[252][27]);
        let eq151_e1913_d_b28: f64 = (p.p7 * s.db[252][28]);
        let eq151_e1913_d_b29: f64 = (p.p7 * s.db[252][29]);
        let eq151_e1913_d_b30: f64 = (p.p7 * s.db[252][30]);
        let eq151_e1913_d_b31: f64 = (p.p7 * s.db[252][31]);
        let eq151_e1913_d_b32: f64 = (p.p7 * s.db[252][32]);
        let eq151_e1913_d_b33: f64 = (p.p7 * s.db[252][33]);
        let eq151_e1913_d_b34: f64 = (p.p7 * s.db[252][34]);
        let eq151_e1913_d_b35: f64 = (p.p7 * s.db[252][35]);
        let eq151_e1913_d_b36: f64 = (p.p7 * s.db[252][36]);
        let eq151_e1913_d_b37: f64 = (p.p7 * s.db[252][37]);
        let eq151_e1913_d_b38: f64 = (p.p7 * s.db[252][38]);
        let eq151_e1913_d_b39: f64 = (p.p7 * s.db[252][39]);
        let eq151_e1913_d_b40: f64 = (p.p7 * s.db[252][40]);
        let eq151_e1913_d_b41: f64 = (p.p7 * s.db[252][41]);
        let eq151_e1913_d_b42: f64 = (p.p7 * s.db[252][42]);
        let eq151_e1913_d_b43: f64 = (p.p7 * s.db[252][43]);
        let eq151_e1913_d_b44: f64 = (p.p7 * s.db[252][44]);
        let eq151_e1913_d_b45: f64 = (p.p7 * s.db[252][45]);
        let eq151_e1913_d_b46: f64 = (p.p7 * s.db[252][46]);
        let eq151_e1913_d_b47: f64 = (p.p7 * s.db[252][47]);
        let eq151_e1913_d_b48: f64 = (p.p7 * s.db[252][48]);
        let eq151_e1913_d_b49: f64 = (p.p7 * s.db[252][49]);
        let eq151_e1913_d_b50: f64 = (p.p7 * s.db[252][50]);
        let eq151_e1913_d_b51: f64 = (p.p7 * s.db[252][51]);
        let eq151_e1913_d_b52: f64 = (p.p7 * s.db[252][52]);
        let eq151_e1913_d_b53: f64 = (p.p7 * s.db[252][53]);
        let eq151_e1913_d_b54: f64 = (p.p7 * s.db[252][54]);
        let eq151_e1913_q: f64 = (p.p7 * eq151_e1912_q);
        (eq151_e1913, eq151_e1913_d_n0, eq151_e1913_d_n1, eq151_e1913_d_n2, eq151_e1913_d_n3, eq151_e1913_d_n4, eq151_e1913_d_n5, eq151_e1913_d_n6, eq151_e1913_d_n7, eq151_e1913_d_n8, eq151_e1913_d_n9, eq151_e1913_d_n10, eq151_e1913_d_n11, eq151_e1913_d_n12, eq151_e1913_d_n13, eq151_e1913_d_n14, eq151_e1913_d_n15, eq151_e1913_d_n16, eq151_e1913_d_n17, eq151_e1913_d_n18, eq151_e1913_d_n19, eq151_e1913_d_n20, eq151_e1913_d_n21, eq151_e1913_d_n22, eq151_e1913_d_b0, eq151_e1913_d_b1, eq151_e1913_d_b2, eq151_e1913_d_b3, eq151_e1913_d_b4, eq151_e1913_d_b5, eq151_e1913_d_b6, eq151_e1913_d_b7, eq151_e1913_d_b8, eq151_e1913_d_b9, eq151_e1913_d_b10, eq151_e1913_d_b11, eq151_e1913_d_b12, eq151_e1913_d_b13, eq151_e1913_d_b14, eq151_e1913_d_b15, eq151_e1913_d_b16, eq151_e1913_d_b17, eq151_e1913_d_b18, eq151_e1913_d_b19, eq151_e1913_d_b20, eq151_e1913_d_b21, eq151_e1913_d_b22, eq151_e1913_d_b23, eq151_e1913_d_b24, eq151_e1913_d_b25, eq151_e1913_d_b26, eq151_e1913_d_b27, eq151_e1913_d_b28, eq151_e1913_d_b29, eq151_e1913_d_b30, eq151_e1913_d_b31, eq151_e1913_d_b32, eq151_e1913_d_b33, eq151_e1913_d_b34, eq151_e1913_d_b35, eq151_e1913_d_b36, eq151_e1913_d_b37, eq151_e1913_d_b38, eq151_e1913_d_b39, eq151_e1913_d_b40, eq151_e1913_d_b41, eq151_e1913_d_b42, eq151_e1913_d_b43, eq151_e1913_d_b44, eq151_e1913_d_b45, eq151_e1913_d_b46, eq151_e1913_d_b47, eq151_e1913_d_b48, eq151_e1913_d_b49, eq151_e1913_d_b50, eq151_e1913_d_b51, eq151_e1913_d_b52, eq151_e1913_d_b53, eq151_e1913_d_b54, eq151_e1913_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_reactive_node_derivatives: [f64; 23] = [eq151_e1915_d_n0, eq151_e1915_d_n1, eq151_e1915_d_n2, eq151_e1915_d_n3, eq151_e1915_d_n4, eq151_e1915_d_n5, eq151_e1915_d_n6, eq151_e1915_d_n7, eq151_e1915_d_n8, eq151_e1915_d_n9, eq151_e1915_d_n10, eq151_e1915_d_n11, eq151_e1915_d_n12, eq151_e1915_d_n13, eq151_e1915_d_n14, eq151_e1915_d_n15, eq151_e1915_d_n16, eq151_e1915_d_n17, eq151_e1915_d_n18, eq151_e1915_d_n19, eq151_e1915_d_n20, eq151_e1915_d_n21, eq151_e1915_d_n22];
        let eq151_reactive_branch_derivatives: [f64; 55] = [eq151_e1915_d_b0, eq151_e1915_d_b1, eq151_e1915_d_b2, eq151_e1915_d_b3, eq151_e1915_d_b4, eq151_e1915_d_b5, eq151_e1915_d_b6, eq151_e1915_d_b7, eq151_e1915_d_b8, eq151_e1915_d_b9, eq151_e1915_d_b10, eq151_e1915_d_b11, eq151_e1915_d_b12, eq151_e1915_d_b13, eq151_e1915_d_b14, eq151_e1915_d_b15, eq151_e1915_d_b16, eq151_e1915_d_b17, eq151_e1915_d_b18, eq151_e1915_d_b19, eq151_e1915_d_b20, eq151_e1915_d_b21, eq151_e1915_d_b22, eq151_e1915_d_b23, eq151_e1915_d_b24, eq151_e1915_d_b25, eq151_e1915_d_b26, eq151_e1915_d_b27, eq151_e1915_d_b28, eq151_e1915_d_b29, eq151_e1915_d_b30, eq151_e1915_d_b31, eq151_e1915_d_b32, eq151_e1915_d_b33, eq151_e1915_d_b34, eq151_e1915_d_b35, eq151_e1915_d_b36, eq151_e1915_d_b37, eq151_e1915_d_b38, eq151_e1915_d_b39, eq151_e1915_d_b40, eq151_e1915_d_b41, eq151_e1915_d_b42, eq151_e1915_d_b43, eq151_e1915_d_b44, eq151_e1915_d_b45, eq151_e1915_d_b46, eq151_e1915_d_b47, eq151_e1915_d_b48, eq151_e1915_d_b49, eq151_e1915_d_b50, eq151_e1915_d_b51, eq151_e1915_d_b52, eq151_e1915_d_b53, eq151_e1915_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq151_reactive_node_derivatives,
            branches,
            &eq151_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq152_e1929, eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22, eq152_e1929_d_b0, eq152_e1929_d_b1, eq152_e1929_d_b2, eq152_e1929_d_b3, eq152_e1929_d_b4, eq152_e1929_d_b5, eq152_e1929_d_b6, eq152_e1929_d_b7, eq152_e1929_d_b8, eq152_e1929_d_b9, eq152_e1929_d_b10, eq152_e1929_d_b11, eq152_e1929_d_b12, eq152_e1929_d_b13, eq152_e1929_d_b14, eq152_e1929_d_b15, eq152_e1929_d_b16, eq152_e1929_d_b17, eq152_e1929_d_b18, eq152_e1929_d_b19, eq152_e1929_d_b20, eq152_e1929_d_b21, eq152_e1929_d_b22, eq152_e1929_d_b23, eq152_e1929_d_b24, eq152_e1929_d_b25, eq152_e1929_d_b26, eq152_e1929_d_b27, eq152_e1929_d_b28, eq152_e1929_d_b29, eq152_e1929_d_b30, eq152_e1929_d_b31, eq152_e1929_d_b32, eq152_e1929_d_b33, eq152_e1929_d_b34, eq152_e1929_d_b35, eq152_e1929_d_b36, eq152_e1929_d_b37, eq152_e1929_d_b38, eq152_e1929_d_b39, eq152_e1929_d_b40, eq152_e1929_d_b41, eq152_e1929_d_b42, eq152_e1929_d_b43, eq152_e1929_d_b44, eq152_e1929_d_b45, eq152_e1929_d_b46, eq152_e1929_d_b47, eq152_e1929_d_b48, eq152_e1929_d_b49, eq152_e1929_d_b50, eq152_e1929_d_b51, eq152_e1929_d_b52, eq152_e1929_d_b53, eq152_e1929_d_b54, eq152_e1929_q,) = {
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
        let eq152_e1927_d_b0: f64 = (eq152_e1924 * s.db[252][0]);
        let eq152_e1927_d_b1: f64 = (eq152_e1924 * s.db[252][1]);
        let eq152_e1927_d_b2: f64 = (eq152_e1924 * s.db[252][2]);
        let eq152_e1927_d_b3: f64 = (eq152_e1924 * s.db[252][3]);
        let eq152_e1927_d_b4: f64 = (eq152_e1924 * s.db[252][4]);
        let eq152_e1927_d_b5: f64 = (eq152_e1924 * s.db[252][5]);
        let eq152_e1927_d_b6: f64 = (eq152_e1924 * s.db[252][6]);
        let eq152_e1927_d_b7: f64 = (eq152_e1924 * s.db[252][7]);
        let eq152_e1927_d_b8: f64 = (eq152_e1924 * s.db[252][8]);
        let eq152_e1927_d_b9: f64 = (eq152_e1924 * s.db[252][9]);
        let eq152_e1927_d_b10: f64 = (eq152_e1924 * s.db[252][10]);
        let eq152_e1927_d_b11: f64 = (eq152_e1924 * s.db[252][11]);
        let eq152_e1927_d_b12: f64 = (eq152_e1924 * s.db[252][12]);
        let eq152_e1927_d_b13: f64 = (eq152_e1924 * s.db[252][13]);
        let eq152_e1927_d_b14: f64 = (eq152_e1924 * s.db[252][14]);
        let eq152_e1927_d_b15: f64 = (eq152_e1924 * s.db[252][15]);
        let eq152_e1927_d_b16: f64 = (eq152_e1924 * s.db[252][16]);
        let eq152_e1927_d_b17: f64 = (eq152_e1924 * s.db[252][17]);
        let eq152_e1927_d_b18: f64 = (eq152_e1924 * s.db[252][18]);
        let eq152_e1927_d_b19: f64 = (eq152_e1924 * s.db[252][19]);
        let eq152_e1927_d_b20: f64 = (eq152_e1924 * s.db[252][20]);
        let eq152_e1927_d_b21: f64 = (eq152_e1924 * s.db[252][21]);
        let eq152_e1927_d_b22: f64 = (eq152_e1924 * s.db[252][22]);
        let eq152_e1927_d_b23: f64 = (eq152_e1924 * s.db[252][23]);
        let eq152_e1927_d_b24: f64 = (eq152_e1924 * s.db[252][24]);
        let eq152_e1927_d_b25: f64 = (eq152_e1924 * s.db[252][25]);
        let eq152_e1927_d_b26: f64 = (eq152_e1924 * s.db[252][26]);
        let eq152_e1927_d_b27: f64 = (eq152_e1924 * s.db[252][27]);
        let eq152_e1927_d_b28: f64 = (eq152_e1924 * s.db[252][28]);
        let eq152_e1927_d_b29: f64 = (eq152_e1924 * s.db[252][29]);
        let eq152_e1927_d_b30: f64 = (eq152_e1924 * s.db[252][30]);
        let eq152_e1927_d_b31: f64 = (eq152_e1924 * s.db[252][31]);
        let eq152_e1927_d_b32: f64 = (eq152_e1924 * s.db[252][32]);
        let eq152_e1927_d_b33: f64 = (eq152_e1924 * s.db[252][33]);
        let eq152_e1927_d_b34: f64 = (eq152_e1924 * s.db[252][34]);
        let eq152_e1927_d_b35: f64 = (eq152_e1924 * s.db[252][35]);
        let eq152_e1927_d_b36: f64 = (eq152_e1924 * s.db[252][36]);
        let eq152_e1927_d_b37: f64 = (eq152_e1924 * s.db[252][37]);
        let eq152_e1927_d_b38: f64 = (eq152_e1924 * s.db[252][38]);
        let eq152_e1927_d_b39: f64 = (eq152_e1924 * s.db[252][39]);
        let eq152_e1927_d_b40: f64 = (eq152_e1924 * s.db[252][40]);
        let eq152_e1927_d_b41: f64 = (eq152_e1924 * s.db[252][41]);
        let eq152_e1927_d_b42: f64 = (eq152_e1924 * s.db[252][42]);
        let eq152_e1927_d_b43: f64 = (eq152_e1924 * s.db[252][43]);
        let eq152_e1927_d_b44: f64 = (eq152_e1924 * s.db[252][44]);
        let eq152_e1927_d_b45: f64 = (eq152_e1924 * s.db[252][45]);
        let eq152_e1927_d_b46: f64 = (eq152_e1924 * s.db[252][46]);
        let eq152_e1927_d_b47: f64 = (eq152_e1924 * s.db[252][47]);
        let eq152_e1927_d_b48: f64 = (eq152_e1924 * s.db[252][48]);
        let eq152_e1927_d_b49: f64 = (eq152_e1924 * s.db[252][49]);
        let eq152_e1927_d_b50: f64 = (eq152_e1924 * s.db[252][50]);
        let eq152_e1927_d_b51: f64 = (eq152_e1924 * s.db[252][51]);
        let eq152_e1927_d_b52: f64 = (eq152_e1924 * s.db[252][52]);
        let eq152_e1927_d_b53: f64 = (eq152_e1924 * s.db[252][53]);
        let eq152_e1927_d_b54: f64 = (eq152_e1924 * s.db[252][54]);
        let eq152_e1927_q: f64 = (eq152_e1924 * eq152_e1926_q);
        (eq152_e1927, eq152_e1927_d_n0, eq152_e1927_d_n1, eq152_e1927_d_n2, eq152_e1927_d_n3, eq152_e1927_d_n4, eq152_e1927_d_n5, eq152_e1927_d_n6, eq152_e1927_d_n7, eq152_e1927_d_n8, eq152_e1927_d_n9, eq152_e1927_d_n10, eq152_e1927_d_n11, eq152_e1927_d_n12, eq152_e1927_d_n13, eq152_e1927_d_n14, eq152_e1927_d_n15, eq152_e1927_d_n16, eq152_e1927_d_n17, eq152_e1927_d_n18, eq152_e1927_d_n19, eq152_e1927_d_n20, eq152_e1927_d_n21, eq152_e1927_d_n22, eq152_e1927_d_b0, eq152_e1927_d_b1, eq152_e1927_d_b2, eq152_e1927_d_b3, eq152_e1927_d_b4, eq152_e1927_d_b5, eq152_e1927_d_b6, eq152_e1927_d_b7, eq152_e1927_d_b8, eq152_e1927_d_b9, eq152_e1927_d_b10, eq152_e1927_d_b11, eq152_e1927_d_b12, eq152_e1927_d_b13, eq152_e1927_d_b14, eq152_e1927_d_b15, eq152_e1927_d_b16, eq152_e1927_d_b17, eq152_e1927_d_b18, eq152_e1927_d_b19, eq152_e1927_d_b20, eq152_e1927_d_b21, eq152_e1927_d_b22, eq152_e1927_d_b23, eq152_e1927_d_b24, eq152_e1927_d_b25, eq152_e1927_d_b26, eq152_e1927_d_b27, eq152_e1927_d_b28, eq152_e1927_d_b29, eq152_e1927_d_b30, eq152_e1927_d_b31, eq152_e1927_d_b32, eq152_e1927_d_b33, eq152_e1927_d_b34, eq152_e1927_d_b35, eq152_e1927_d_b36, eq152_e1927_d_b37, eq152_e1927_d_b38, eq152_e1927_d_b39, eq152_e1927_d_b40, eq152_e1927_d_b41, eq152_e1927_d_b42, eq152_e1927_d_b43, eq152_e1927_d_b44, eq152_e1927_d_b45, eq152_e1927_d_b46, eq152_e1927_d_b47, eq152_e1927_d_b48, eq152_e1927_d_b49, eq152_e1927_d_b50, eq152_e1927_d_b51, eq152_e1927_d_b52, eq152_e1927_d_b53, eq152_e1927_d_b54, eq152_e1927_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_reactive_node_derivatives: [f64; 23] = [eq152_e1929_d_n0, eq152_e1929_d_n1, eq152_e1929_d_n2, eq152_e1929_d_n3, eq152_e1929_d_n4, eq152_e1929_d_n5, eq152_e1929_d_n6, eq152_e1929_d_n7, eq152_e1929_d_n8, eq152_e1929_d_n9, eq152_e1929_d_n10, eq152_e1929_d_n11, eq152_e1929_d_n12, eq152_e1929_d_n13, eq152_e1929_d_n14, eq152_e1929_d_n15, eq152_e1929_d_n16, eq152_e1929_d_n17, eq152_e1929_d_n18, eq152_e1929_d_n19, eq152_e1929_d_n20, eq152_e1929_d_n21, eq152_e1929_d_n22];
        let eq152_reactive_branch_derivatives: [f64; 55] = [eq152_e1929_d_b0, eq152_e1929_d_b1, eq152_e1929_d_b2, eq152_e1929_d_b3, eq152_e1929_d_b4, eq152_e1929_d_b5, eq152_e1929_d_b6, eq152_e1929_d_b7, eq152_e1929_d_b8, eq152_e1929_d_b9, eq152_e1929_d_b10, eq152_e1929_d_b11, eq152_e1929_d_b12, eq152_e1929_d_b13, eq152_e1929_d_b14, eq152_e1929_d_b15, eq152_e1929_d_b16, eq152_e1929_d_b17, eq152_e1929_d_b18, eq152_e1929_d_b19, eq152_e1929_d_b20, eq152_e1929_d_b21, eq152_e1929_d_b22, eq152_e1929_d_b23, eq152_e1929_d_b24, eq152_e1929_d_b25, eq152_e1929_d_b26, eq152_e1929_d_b27, eq152_e1929_d_b28, eq152_e1929_d_b29, eq152_e1929_d_b30, eq152_e1929_d_b31, eq152_e1929_d_b32, eq152_e1929_d_b33, eq152_e1929_d_b34, eq152_e1929_d_b35, eq152_e1929_d_b36, eq152_e1929_d_b37, eq152_e1929_d_b38, eq152_e1929_d_b39, eq152_e1929_d_b40, eq152_e1929_d_b41, eq152_e1929_d_b42, eq152_e1929_d_b43, eq152_e1929_d_b44, eq152_e1929_d_b45, eq152_e1929_d_b46, eq152_e1929_d_b47, eq152_e1929_d_b48, eq152_e1929_d_b49, eq152_e1929_d_b50, eq152_e1929_d_b51, eq152_e1929_d_b52, eq152_e1929_d_b53, eq152_e1929_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq152_reactive_node_derivatives,
            branches,
            &eq152_reactive_branch_derivatives,
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
        let (eq153_e1942, eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22, eq153_e1942_d_b0, eq153_e1942_d_b1, eq153_e1942_d_b2, eq153_e1942_d_b3, eq153_e1942_d_b4, eq153_e1942_d_b5, eq153_e1942_d_b6, eq153_e1942_d_b7, eq153_e1942_d_b8, eq153_e1942_d_b9, eq153_e1942_d_b10, eq153_e1942_d_b11, eq153_e1942_d_b12, eq153_e1942_d_b13, eq153_e1942_d_b14, eq153_e1942_d_b15, eq153_e1942_d_b16, eq153_e1942_d_b17, eq153_e1942_d_b18, eq153_e1942_d_b19, eq153_e1942_d_b20, eq153_e1942_d_b21, eq153_e1942_d_b22, eq153_e1942_d_b23, eq153_e1942_d_b24, eq153_e1942_d_b25, eq153_e1942_d_b26, eq153_e1942_d_b27, eq153_e1942_d_b28, eq153_e1942_d_b29, eq153_e1942_d_b30, eq153_e1942_d_b31, eq153_e1942_d_b32, eq153_e1942_d_b33, eq153_e1942_d_b34, eq153_e1942_d_b35, eq153_e1942_d_b36, eq153_e1942_d_b37, eq153_e1942_d_b38, eq153_e1942_d_b39, eq153_e1942_d_b40, eq153_e1942_d_b41, eq153_e1942_d_b42, eq153_e1942_d_b43, eq153_e1942_d_b44, eq153_e1942_d_b45, eq153_e1942_d_b46, eq153_e1942_d_b47, eq153_e1942_d_b48, eq153_e1942_d_b49, eq153_e1942_d_b50, eq153_e1942_d_b51, eq153_e1942_d_b52, eq153_e1942_d_b53, eq153_e1942_d_b54, eq153_e1942_q,) = {
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
        let eq153_e1940_d_b0: f64 = (p.p7 * s.db[252][0]);
        let eq153_e1940_d_b1: f64 = (p.p7 * s.db[252][1]);
        let eq153_e1940_d_b2: f64 = (p.p7 * s.db[252][2]);
        let eq153_e1940_d_b3: f64 = (p.p7 * s.db[252][3]);
        let eq153_e1940_d_b4: f64 = (p.p7 * s.db[252][4]);
        let eq153_e1940_d_b5: f64 = (p.p7 * s.db[252][5]);
        let eq153_e1940_d_b6: f64 = (p.p7 * s.db[252][6]);
        let eq153_e1940_d_b7: f64 = (p.p7 * s.db[252][7]);
        let eq153_e1940_d_b8: f64 = (p.p7 * s.db[252][8]);
        let eq153_e1940_d_b9: f64 = (p.p7 * s.db[252][9]);
        let eq153_e1940_d_b10: f64 = (p.p7 * s.db[252][10]);
        let eq153_e1940_d_b11: f64 = (p.p7 * s.db[252][11]);
        let eq153_e1940_d_b12: f64 = (p.p7 * s.db[252][12]);
        let eq153_e1940_d_b13: f64 = (p.p7 * s.db[252][13]);
        let eq153_e1940_d_b14: f64 = (p.p7 * s.db[252][14]);
        let eq153_e1940_d_b15: f64 = (p.p7 * s.db[252][15]);
        let eq153_e1940_d_b16: f64 = (p.p7 * s.db[252][16]);
        let eq153_e1940_d_b17: f64 = (p.p7 * s.db[252][17]);
        let eq153_e1940_d_b18: f64 = (p.p7 * s.db[252][18]);
        let eq153_e1940_d_b19: f64 = (p.p7 * s.db[252][19]);
        let eq153_e1940_d_b20: f64 = (p.p7 * s.db[252][20]);
        let eq153_e1940_d_b21: f64 = (p.p7 * s.db[252][21]);
        let eq153_e1940_d_b22: f64 = (p.p7 * s.db[252][22]);
        let eq153_e1940_d_b23: f64 = (p.p7 * s.db[252][23]);
        let eq153_e1940_d_b24: f64 = (p.p7 * s.db[252][24]);
        let eq153_e1940_d_b25: f64 = (p.p7 * s.db[252][25]);
        let eq153_e1940_d_b26: f64 = (p.p7 * s.db[252][26]);
        let eq153_e1940_d_b27: f64 = (p.p7 * s.db[252][27]);
        let eq153_e1940_d_b28: f64 = (p.p7 * s.db[252][28]);
        let eq153_e1940_d_b29: f64 = (p.p7 * s.db[252][29]);
        let eq153_e1940_d_b30: f64 = (p.p7 * s.db[252][30]);
        let eq153_e1940_d_b31: f64 = (p.p7 * s.db[252][31]);
        let eq153_e1940_d_b32: f64 = (p.p7 * s.db[252][32]);
        let eq153_e1940_d_b33: f64 = (p.p7 * s.db[252][33]);
        let eq153_e1940_d_b34: f64 = (p.p7 * s.db[252][34]);
        let eq153_e1940_d_b35: f64 = (p.p7 * s.db[252][35]);
        let eq153_e1940_d_b36: f64 = (p.p7 * s.db[252][36]);
        let eq153_e1940_d_b37: f64 = (p.p7 * s.db[252][37]);
        let eq153_e1940_d_b38: f64 = (p.p7 * s.db[252][38]);
        let eq153_e1940_d_b39: f64 = (p.p7 * s.db[252][39]);
        let eq153_e1940_d_b40: f64 = (p.p7 * s.db[252][40]);
        let eq153_e1940_d_b41: f64 = (p.p7 * s.db[252][41]);
        let eq153_e1940_d_b42: f64 = (p.p7 * s.db[252][42]);
        let eq153_e1940_d_b43: f64 = (p.p7 * s.db[252][43]);
        let eq153_e1940_d_b44: f64 = (p.p7 * s.db[252][44]);
        let eq153_e1940_d_b45: f64 = (p.p7 * s.db[252][45]);
        let eq153_e1940_d_b46: f64 = (p.p7 * s.db[252][46]);
        let eq153_e1940_d_b47: f64 = (p.p7 * s.db[252][47]);
        let eq153_e1940_d_b48: f64 = (p.p7 * s.db[252][48]);
        let eq153_e1940_d_b49: f64 = (p.p7 * s.db[252][49]);
        let eq153_e1940_d_b50: f64 = (p.p7 * s.db[252][50]);
        let eq153_e1940_d_b51: f64 = (p.p7 * s.db[252][51]);
        let eq153_e1940_d_b52: f64 = (p.p7 * s.db[252][52]);
        let eq153_e1940_d_b53: f64 = (p.p7 * s.db[252][53]);
        let eq153_e1940_d_b54: f64 = (p.p7 * s.db[252][54]);
        let eq153_e1940_q: f64 = (p.p7 * eq153_e1939_q);
        (eq153_e1940, eq153_e1940_d_n0, eq153_e1940_d_n1, eq153_e1940_d_n2, eq153_e1940_d_n3, eq153_e1940_d_n4, eq153_e1940_d_n5, eq153_e1940_d_n6, eq153_e1940_d_n7, eq153_e1940_d_n8, eq153_e1940_d_n9, eq153_e1940_d_n10, eq153_e1940_d_n11, eq153_e1940_d_n12, eq153_e1940_d_n13, eq153_e1940_d_n14, eq153_e1940_d_n15, eq153_e1940_d_n16, eq153_e1940_d_n17, eq153_e1940_d_n18, eq153_e1940_d_n19, eq153_e1940_d_n20, eq153_e1940_d_n21, eq153_e1940_d_n22, eq153_e1940_d_b0, eq153_e1940_d_b1, eq153_e1940_d_b2, eq153_e1940_d_b3, eq153_e1940_d_b4, eq153_e1940_d_b5, eq153_e1940_d_b6, eq153_e1940_d_b7, eq153_e1940_d_b8, eq153_e1940_d_b9, eq153_e1940_d_b10, eq153_e1940_d_b11, eq153_e1940_d_b12, eq153_e1940_d_b13, eq153_e1940_d_b14, eq153_e1940_d_b15, eq153_e1940_d_b16, eq153_e1940_d_b17, eq153_e1940_d_b18, eq153_e1940_d_b19, eq153_e1940_d_b20, eq153_e1940_d_b21, eq153_e1940_d_b22, eq153_e1940_d_b23, eq153_e1940_d_b24, eq153_e1940_d_b25, eq153_e1940_d_b26, eq153_e1940_d_b27, eq153_e1940_d_b28, eq153_e1940_d_b29, eq153_e1940_d_b30, eq153_e1940_d_b31, eq153_e1940_d_b32, eq153_e1940_d_b33, eq153_e1940_d_b34, eq153_e1940_d_b35, eq153_e1940_d_b36, eq153_e1940_d_b37, eq153_e1940_d_b38, eq153_e1940_d_b39, eq153_e1940_d_b40, eq153_e1940_d_b41, eq153_e1940_d_b42, eq153_e1940_d_b43, eq153_e1940_d_b44, eq153_e1940_d_b45, eq153_e1940_d_b46, eq153_e1940_d_b47, eq153_e1940_d_b48, eq153_e1940_d_b49, eq153_e1940_d_b50, eq153_e1940_d_b51, eq153_e1940_d_b52, eq153_e1940_d_b53, eq153_e1940_d_b54, eq153_e1940_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_reactive_node_derivatives: [f64; 23] = [eq153_e1942_d_n0, eq153_e1942_d_n1, eq153_e1942_d_n2, eq153_e1942_d_n3, eq153_e1942_d_n4, eq153_e1942_d_n5, eq153_e1942_d_n6, eq153_e1942_d_n7, eq153_e1942_d_n8, eq153_e1942_d_n9, eq153_e1942_d_n10, eq153_e1942_d_n11, eq153_e1942_d_n12, eq153_e1942_d_n13, eq153_e1942_d_n14, eq153_e1942_d_n15, eq153_e1942_d_n16, eq153_e1942_d_n17, eq153_e1942_d_n18, eq153_e1942_d_n19, eq153_e1942_d_n20, eq153_e1942_d_n21, eq153_e1942_d_n22];
        let eq153_reactive_branch_derivatives: [f64; 55] = [eq153_e1942_d_b0, eq153_e1942_d_b1, eq153_e1942_d_b2, eq153_e1942_d_b3, eq153_e1942_d_b4, eq153_e1942_d_b5, eq153_e1942_d_b6, eq153_e1942_d_b7, eq153_e1942_d_b8, eq153_e1942_d_b9, eq153_e1942_d_b10, eq153_e1942_d_b11, eq153_e1942_d_b12, eq153_e1942_d_b13, eq153_e1942_d_b14, eq153_e1942_d_b15, eq153_e1942_d_b16, eq153_e1942_d_b17, eq153_e1942_d_b18, eq153_e1942_d_b19, eq153_e1942_d_b20, eq153_e1942_d_b21, eq153_e1942_d_b22, eq153_e1942_d_b23, eq153_e1942_d_b24, eq153_e1942_d_b25, eq153_e1942_d_b26, eq153_e1942_d_b27, eq153_e1942_d_b28, eq153_e1942_d_b29, eq153_e1942_d_b30, eq153_e1942_d_b31, eq153_e1942_d_b32, eq153_e1942_d_b33, eq153_e1942_d_b34, eq153_e1942_d_b35, eq153_e1942_d_b36, eq153_e1942_d_b37, eq153_e1942_d_b38, eq153_e1942_d_b39, eq153_e1942_d_b40, eq153_e1942_d_b41, eq153_e1942_d_b42, eq153_e1942_d_b43, eq153_e1942_d_b44, eq153_e1942_d_b45, eq153_e1942_d_b46, eq153_e1942_d_b47, eq153_e1942_d_b48, eq153_e1942_d_b49, eq153_e1942_d_b50, eq153_e1942_d_b51, eq153_e1942_d_b52, eq153_e1942_d_b53, eq153_e1942_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            nodes,
            &eq153_reactive_node_derivatives,
            branches,
            &eq153_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq154_e1957, eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22, eq154_e1957_d_b0, eq154_e1957_d_b1, eq154_e1957_d_b2, eq154_e1957_d_b3, eq154_e1957_d_b4, eq154_e1957_d_b5, eq154_e1957_d_b6, eq154_e1957_d_b7, eq154_e1957_d_b8, eq154_e1957_d_b9, eq154_e1957_d_b10, eq154_e1957_d_b11, eq154_e1957_d_b12, eq154_e1957_d_b13, eq154_e1957_d_b14, eq154_e1957_d_b15, eq154_e1957_d_b16, eq154_e1957_d_b17, eq154_e1957_d_b18, eq154_e1957_d_b19, eq154_e1957_d_b20, eq154_e1957_d_b21, eq154_e1957_d_b22, eq154_e1957_d_b23, eq154_e1957_d_b24, eq154_e1957_d_b25, eq154_e1957_d_b26, eq154_e1957_d_b27, eq154_e1957_d_b28, eq154_e1957_d_b29, eq154_e1957_d_b30, eq154_e1957_d_b31, eq154_e1957_d_b32, eq154_e1957_d_b33, eq154_e1957_d_b34, eq154_e1957_d_b35, eq154_e1957_d_b36, eq154_e1957_d_b37, eq154_e1957_d_b38, eq154_e1957_d_b39, eq154_e1957_d_b40, eq154_e1957_d_b41, eq154_e1957_d_b42, eq154_e1957_d_b43, eq154_e1957_d_b44, eq154_e1957_d_b45, eq154_e1957_d_b46, eq154_e1957_d_b47, eq154_e1957_d_b48, eq154_e1957_d_b49, eq154_e1957_d_b50, eq154_e1957_d_b51, eq154_e1957_d_b52, eq154_e1957_d_b53, eq154_e1957_d_b54, eq154_e1957_q,) = {
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
        let eq154_e1955_d_b0: f64 = (eq154_e1952 * s.db[252][0]);
        let eq154_e1955_d_b1: f64 = (eq154_e1952 * s.db[252][1]);
        let eq154_e1955_d_b2: f64 = (eq154_e1952 * s.db[252][2]);
        let eq154_e1955_d_b3: f64 = (eq154_e1952 * s.db[252][3]);
        let eq154_e1955_d_b4: f64 = (eq154_e1952 * s.db[252][4]);
        let eq154_e1955_d_b5: f64 = (eq154_e1952 * s.db[252][5]);
        let eq154_e1955_d_b6: f64 = (eq154_e1952 * s.db[252][6]);
        let eq154_e1955_d_b7: f64 = (eq154_e1952 * s.db[252][7]);
        let eq154_e1955_d_b8: f64 = (eq154_e1952 * s.db[252][8]);
        let eq154_e1955_d_b9: f64 = (eq154_e1952 * s.db[252][9]);
        let eq154_e1955_d_b10: f64 = (eq154_e1952 * s.db[252][10]);
        let eq154_e1955_d_b11: f64 = (eq154_e1952 * s.db[252][11]);
        let eq154_e1955_d_b12: f64 = (eq154_e1952 * s.db[252][12]);
        let eq154_e1955_d_b13: f64 = (eq154_e1952 * s.db[252][13]);
        let eq154_e1955_d_b14: f64 = (eq154_e1952 * s.db[252][14]);
        let eq154_e1955_d_b15: f64 = (eq154_e1952 * s.db[252][15]);
        let eq154_e1955_d_b16: f64 = (eq154_e1952 * s.db[252][16]);
        let eq154_e1955_d_b17: f64 = (eq154_e1952 * s.db[252][17]);
        let eq154_e1955_d_b18: f64 = (eq154_e1952 * s.db[252][18]);
        let eq154_e1955_d_b19: f64 = (eq154_e1952 * s.db[252][19]);
        let eq154_e1955_d_b20: f64 = (eq154_e1952 * s.db[252][20]);
        let eq154_e1955_d_b21: f64 = (eq154_e1952 * s.db[252][21]);
        let eq154_e1955_d_b22: f64 = (eq154_e1952 * s.db[252][22]);
        let eq154_e1955_d_b23: f64 = (eq154_e1952 * s.db[252][23]);
        let eq154_e1955_d_b24: f64 = (eq154_e1952 * s.db[252][24]);
        let eq154_e1955_d_b25: f64 = (eq154_e1952 * s.db[252][25]);
        let eq154_e1955_d_b26: f64 = (eq154_e1952 * s.db[252][26]);
        let eq154_e1955_d_b27: f64 = (eq154_e1952 * s.db[252][27]);
        let eq154_e1955_d_b28: f64 = (eq154_e1952 * s.db[252][28]);
        let eq154_e1955_d_b29: f64 = (eq154_e1952 * s.db[252][29]);
        let eq154_e1955_d_b30: f64 = (eq154_e1952 * s.db[252][30]);
        let eq154_e1955_d_b31: f64 = (eq154_e1952 * s.db[252][31]);
        let eq154_e1955_d_b32: f64 = (eq154_e1952 * s.db[252][32]);
        let eq154_e1955_d_b33: f64 = (eq154_e1952 * s.db[252][33]);
        let eq154_e1955_d_b34: f64 = (eq154_e1952 * s.db[252][34]);
        let eq154_e1955_d_b35: f64 = (eq154_e1952 * s.db[252][35]);
        let eq154_e1955_d_b36: f64 = (eq154_e1952 * s.db[252][36]);
        let eq154_e1955_d_b37: f64 = (eq154_e1952 * s.db[252][37]);
        let eq154_e1955_d_b38: f64 = (eq154_e1952 * s.db[252][38]);
        let eq154_e1955_d_b39: f64 = (eq154_e1952 * s.db[252][39]);
        let eq154_e1955_d_b40: f64 = (eq154_e1952 * s.db[252][40]);
        let eq154_e1955_d_b41: f64 = (eq154_e1952 * s.db[252][41]);
        let eq154_e1955_d_b42: f64 = (eq154_e1952 * s.db[252][42]);
        let eq154_e1955_d_b43: f64 = (eq154_e1952 * s.db[252][43]);
        let eq154_e1955_d_b44: f64 = (eq154_e1952 * s.db[252][44]);
        let eq154_e1955_d_b45: f64 = (eq154_e1952 * s.db[252][45]);
        let eq154_e1955_d_b46: f64 = (eq154_e1952 * s.db[252][46]);
        let eq154_e1955_d_b47: f64 = (eq154_e1952 * s.db[252][47]);
        let eq154_e1955_d_b48: f64 = (eq154_e1952 * s.db[252][48]);
        let eq154_e1955_d_b49: f64 = (eq154_e1952 * s.db[252][49]);
        let eq154_e1955_d_b50: f64 = (eq154_e1952 * s.db[252][50]);
        let eq154_e1955_d_b51: f64 = (eq154_e1952 * s.db[252][51]);
        let eq154_e1955_d_b52: f64 = (eq154_e1952 * s.db[252][52]);
        let eq154_e1955_d_b53: f64 = (eq154_e1952 * s.db[252][53]);
        let eq154_e1955_d_b54: f64 = (eq154_e1952 * s.db[252][54]);
        let eq154_e1955_q: f64 = (eq154_e1952 * eq154_e1954_q);
        (eq154_e1955, eq154_e1955_d_n0, eq154_e1955_d_n1, eq154_e1955_d_n2, eq154_e1955_d_n3, eq154_e1955_d_n4, eq154_e1955_d_n5, eq154_e1955_d_n6, eq154_e1955_d_n7, eq154_e1955_d_n8, eq154_e1955_d_n9, eq154_e1955_d_n10, eq154_e1955_d_n11, eq154_e1955_d_n12, eq154_e1955_d_n13, eq154_e1955_d_n14, eq154_e1955_d_n15, eq154_e1955_d_n16, eq154_e1955_d_n17, eq154_e1955_d_n18, eq154_e1955_d_n19, eq154_e1955_d_n20, eq154_e1955_d_n21, eq154_e1955_d_n22, eq154_e1955_d_b0, eq154_e1955_d_b1, eq154_e1955_d_b2, eq154_e1955_d_b3, eq154_e1955_d_b4, eq154_e1955_d_b5, eq154_e1955_d_b6, eq154_e1955_d_b7, eq154_e1955_d_b8, eq154_e1955_d_b9, eq154_e1955_d_b10, eq154_e1955_d_b11, eq154_e1955_d_b12, eq154_e1955_d_b13, eq154_e1955_d_b14, eq154_e1955_d_b15, eq154_e1955_d_b16, eq154_e1955_d_b17, eq154_e1955_d_b18, eq154_e1955_d_b19, eq154_e1955_d_b20, eq154_e1955_d_b21, eq154_e1955_d_b22, eq154_e1955_d_b23, eq154_e1955_d_b24, eq154_e1955_d_b25, eq154_e1955_d_b26, eq154_e1955_d_b27, eq154_e1955_d_b28, eq154_e1955_d_b29, eq154_e1955_d_b30, eq154_e1955_d_b31, eq154_e1955_d_b32, eq154_e1955_d_b33, eq154_e1955_d_b34, eq154_e1955_d_b35, eq154_e1955_d_b36, eq154_e1955_d_b37, eq154_e1955_d_b38, eq154_e1955_d_b39, eq154_e1955_d_b40, eq154_e1955_d_b41, eq154_e1955_d_b42, eq154_e1955_d_b43, eq154_e1955_d_b44, eq154_e1955_d_b45, eq154_e1955_d_b46, eq154_e1955_d_b47, eq154_e1955_d_b48, eq154_e1955_d_b49, eq154_e1955_d_b50, eq154_e1955_d_b51, eq154_e1955_d_b52, eq154_e1955_d_b53, eq154_e1955_d_b54, eq154_e1955_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_reactive_node_derivatives: [f64; 23] = [eq154_e1957_d_n0, eq154_e1957_d_n1, eq154_e1957_d_n2, eq154_e1957_d_n3, eq154_e1957_d_n4, eq154_e1957_d_n5, eq154_e1957_d_n6, eq154_e1957_d_n7, eq154_e1957_d_n8, eq154_e1957_d_n9, eq154_e1957_d_n10, eq154_e1957_d_n11, eq154_e1957_d_n12, eq154_e1957_d_n13, eq154_e1957_d_n14, eq154_e1957_d_n15, eq154_e1957_d_n16, eq154_e1957_d_n17, eq154_e1957_d_n18, eq154_e1957_d_n19, eq154_e1957_d_n20, eq154_e1957_d_n21, eq154_e1957_d_n22];
        let eq154_reactive_branch_derivatives: [f64; 55] = [eq154_e1957_d_b0, eq154_e1957_d_b1, eq154_e1957_d_b2, eq154_e1957_d_b3, eq154_e1957_d_b4, eq154_e1957_d_b5, eq154_e1957_d_b6, eq154_e1957_d_b7, eq154_e1957_d_b8, eq154_e1957_d_b9, eq154_e1957_d_b10, eq154_e1957_d_b11, eq154_e1957_d_b12, eq154_e1957_d_b13, eq154_e1957_d_b14, eq154_e1957_d_b15, eq154_e1957_d_b16, eq154_e1957_d_b17, eq154_e1957_d_b18, eq154_e1957_d_b19, eq154_e1957_d_b20, eq154_e1957_d_b21, eq154_e1957_d_b22, eq154_e1957_d_b23, eq154_e1957_d_b24, eq154_e1957_d_b25, eq154_e1957_d_b26, eq154_e1957_d_b27, eq154_e1957_d_b28, eq154_e1957_d_b29, eq154_e1957_d_b30, eq154_e1957_d_b31, eq154_e1957_d_b32, eq154_e1957_d_b33, eq154_e1957_d_b34, eq154_e1957_d_b35, eq154_e1957_d_b36, eq154_e1957_d_b37, eq154_e1957_d_b38, eq154_e1957_d_b39, eq154_e1957_d_b40, eq154_e1957_d_b41, eq154_e1957_d_b42, eq154_e1957_d_b43, eq154_e1957_d_b44, eq154_e1957_d_b45, eq154_e1957_d_b46, eq154_e1957_d_b47, eq154_e1957_d_b48, eq154_e1957_d_b49, eq154_e1957_d_b50, eq154_e1957_d_b51, eq154_e1957_d_b52, eq154_e1957_d_b53, eq154_e1957_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq154_reactive_node_derivatives,
            branches,
            &eq154_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq155_e1969, eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22, eq155_e1969_d_b0, eq155_e1969_d_b1, eq155_e1969_d_b2, eq155_e1969_d_b3, eq155_e1969_d_b4, eq155_e1969_d_b5, eq155_e1969_d_b6, eq155_e1969_d_b7, eq155_e1969_d_b8, eq155_e1969_d_b9, eq155_e1969_d_b10, eq155_e1969_d_b11, eq155_e1969_d_b12, eq155_e1969_d_b13, eq155_e1969_d_b14, eq155_e1969_d_b15, eq155_e1969_d_b16, eq155_e1969_d_b17, eq155_e1969_d_b18, eq155_e1969_d_b19, eq155_e1969_d_b20, eq155_e1969_d_b21, eq155_e1969_d_b22, eq155_e1969_d_b23, eq155_e1969_d_b24, eq155_e1969_d_b25, eq155_e1969_d_b26, eq155_e1969_d_b27, eq155_e1969_d_b28, eq155_e1969_d_b29, eq155_e1969_d_b30, eq155_e1969_d_b31, eq155_e1969_d_b32, eq155_e1969_d_b33, eq155_e1969_d_b34, eq155_e1969_d_b35, eq155_e1969_d_b36, eq155_e1969_d_b37, eq155_e1969_d_b38, eq155_e1969_d_b39, eq155_e1969_d_b40, eq155_e1969_d_b41, eq155_e1969_d_b42, eq155_e1969_d_b43, eq155_e1969_d_b44, eq155_e1969_d_b45, eq155_e1969_d_b46, eq155_e1969_d_b47, eq155_e1969_d_b48, eq155_e1969_d_b49, eq155_e1969_d_b50, eq155_e1969_d_b51, eq155_e1969_d_b52, eq155_e1969_d_b53, eq155_e1969_d_b54, eq155_e1969_q,) = {
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
        let eq155_e1965_d_b0: f64 = (p.p252 * s.db[252][0]);
        let eq155_e1965_d_b1: f64 = (p.p252 * s.db[252][1]);
        let eq155_e1965_d_b2: f64 = (p.p252 * s.db[252][2]);
        let eq155_e1965_d_b3: f64 = (p.p252 * s.db[252][3]);
        let eq155_e1965_d_b4: f64 = (p.p252 * s.db[252][4]);
        let eq155_e1965_d_b5: f64 = (p.p252 * s.db[252][5]);
        let eq155_e1965_d_b6: f64 = (p.p252 * s.db[252][6]);
        let eq155_e1965_d_b7: f64 = (p.p252 * s.db[252][7]);
        let eq155_e1965_d_b8: f64 = (p.p252 * s.db[252][8]);
        let eq155_e1965_d_b9: f64 = (p.p252 * s.db[252][9]);
        let eq155_e1965_d_b10: f64 = (p.p252 * s.db[252][10]);
        let eq155_e1965_d_b11: f64 = (p.p252 * s.db[252][11]);
        let eq155_e1965_d_b12: f64 = (p.p252 * s.db[252][12]);
        let eq155_e1965_d_b13: f64 = (p.p252 * s.db[252][13]);
        let eq155_e1965_d_b14: f64 = (p.p252 * s.db[252][14]);
        let eq155_e1965_d_b15: f64 = (p.p252 * s.db[252][15]);
        let eq155_e1965_d_b16: f64 = (p.p252 * s.db[252][16]);
        let eq155_e1965_d_b17: f64 = (p.p252 * s.db[252][17]);
        let eq155_e1965_d_b18: f64 = (p.p252 * s.db[252][18]);
        let eq155_e1965_d_b19: f64 = (p.p252 * s.db[252][19]);
        let eq155_e1965_d_b20: f64 = (p.p252 * s.db[252][20]);
        let eq155_e1965_d_b21: f64 = (p.p252 * s.db[252][21]);
        let eq155_e1965_d_b22: f64 = (p.p252 * s.db[252][22]);
        let eq155_e1965_d_b23: f64 = (p.p252 * s.db[252][23]);
        let eq155_e1965_d_b24: f64 = (p.p252 * s.db[252][24]);
        let eq155_e1965_d_b25: f64 = (p.p252 * s.db[252][25]);
        let eq155_e1965_d_b26: f64 = (p.p252 * s.db[252][26]);
        let eq155_e1965_d_b27: f64 = (p.p252 * s.db[252][27]);
        let eq155_e1965_d_b28: f64 = (p.p252 * s.db[252][28]);
        let eq155_e1965_d_b29: f64 = (p.p252 * s.db[252][29]);
        let eq155_e1965_d_b30: f64 = (p.p252 * s.db[252][30]);
        let eq155_e1965_d_b31: f64 = (p.p252 * s.db[252][31]);
        let eq155_e1965_d_b32: f64 = (p.p252 * s.db[252][32]);
        let eq155_e1965_d_b33: f64 = (p.p252 * s.db[252][33]);
        let eq155_e1965_d_b34: f64 = (p.p252 * s.db[252][34]);
        let eq155_e1965_d_b35: f64 = (p.p252 * s.db[252][35]);
        let eq155_e1965_d_b36: f64 = (p.p252 * s.db[252][36]);
        let eq155_e1965_d_b37: f64 = (p.p252 * s.db[252][37]);
        let eq155_e1965_d_b38: f64 = (p.p252 * s.db[252][38]);
        let eq155_e1965_d_b39: f64 = (p.p252 * s.db[252][39]);
        let eq155_e1965_d_b40: f64 = (p.p252 * s.db[252][40]);
        let eq155_e1965_d_b41: f64 = (p.p252 * s.db[252][41]);
        let eq155_e1965_d_b42: f64 = (p.p252 * s.db[252][42]);
        let eq155_e1965_d_b43: f64 = (p.p252 * s.db[252][43]);
        let eq155_e1965_d_b44: f64 = (p.p252 * s.db[252][44]);
        let eq155_e1965_d_b45: f64 = (p.p252 * s.db[252][45]);
        let eq155_e1965_d_b46: f64 = (p.p252 * s.db[252][46]);
        let eq155_e1965_d_b47: f64 = (p.p252 * s.db[252][47]);
        let eq155_e1965_d_b48: f64 = (p.p252 * s.db[252][48]);
        let eq155_e1965_d_b49: f64 = (p.p252 * s.db[252][49]);
        let eq155_e1965_d_b50: f64 = (p.p252 * s.db[252][50]);
        let eq155_e1965_d_b51: f64 = (p.p252 * s.db[252][51]);
        let eq155_e1965_d_b52: f64 = (p.p252 * s.db[252][52]);
        let eq155_e1965_d_b53: f64 = (p.p252 * s.db[252][53]);
        let eq155_e1965_d_b54: f64 = (p.p252 * s.db[252][54]);
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
        let eq155_e1967_d_b0: f64 = (p.p7 * eq155_e1965_d_b0);
        let eq155_e1967_d_b1: f64 = (p.p7 * eq155_e1965_d_b1);
        let eq155_e1967_d_b2: f64 = (p.p7 * eq155_e1965_d_b2);
        let eq155_e1967_d_b3: f64 = (p.p7 * eq155_e1965_d_b3);
        let eq155_e1967_d_b4: f64 = (p.p7 * eq155_e1965_d_b4);
        let eq155_e1967_d_b5: f64 = (p.p7 * eq155_e1965_d_b5);
        let eq155_e1967_d_b6: f64 = (p.p7 * eq155_e1965_d_b6);
        let eq155_e1967_d_b7: f64 = (p.p7 * eq155_e1965_d_b7);
        let eq155_e1967_d_b8: f64 = (p.p7 * eq155_e1965_d_b8);
        let eq155_e1967_d_b9: f64 = (p.p7 * eq155_e1965_d_b9);
        let eq155_e1967_d_b10: f64 = (p.p7 * eq155_e1965_d_b10);
        let eq155_e1967_d_b11: f64 = (p.p7 * eq155_e1965_d_b11);
        let eq155_e1967_d_b12: f64 = (p.p7 * eq155_e1965_d_b12);
        let eq155_e1967_d_b13: f64 = (p.p7 * eq155_e1965_d_b13);
        let eq155_e1967_d_b14: f64 = (p.p7 * eq155_e1965_d_b14);
        let eq155_e1967_d_b15: f64 = (p.p7 * eq155_e1965_d_b15);
        let eq155_e1967_d_b16: f64 = (p.p7 * eq155_e1965_d_b16);
        let eq155_e1967_d_b17: f64 = (p.p7 * eq155_e1965_d_b17);
        let eq155_e1967_d_b18: f64 = (p.p7 * eq155_e1965_d_b18);
        let eq155_e1967_d_b19: f64 = (p.p7 * eq155_e1965_d_b19);
        let eq155_e1967_d_b20: f64 = (p.p7 * eq155_e1965_d_b20);
        let eq155_e1967_d_b21: f64 = (p.p7 * eq155_e1965_d_b21);
        let eq155_e1967_d_b22: f64 = (p.p7 * eq155_e1965_d_b22);
        let eq155_e1967_d_b23: f64 = (p.p7 * eq155_e1965_d_b23);
        let eq155_e1967_d_b24: f64 = (p.p7 * eq155_e1965_d_b24);
        let eq155_e1967_d_b25: f64 = (p.p7 * eq155_e1965_d_b25);
        let eq155_e1967_d_b26: f64 = (p.p7 * eq155_e1965_d_b26);
        let eq155_e1967_d_b27: f64 = (p.p7 * eq155_e1965_d_b27);
        let eq155_e1967_d_b28: f64 = (p.p7 * eq155_e1965_d_b28);
        let eq155_e1967_d_b29: f64 = (p.p7 * eq155_e1965_d_b29);
        let eq155_e1967_d_b30: f64 = (p.p7 * eq155_e1965_d_b30);
        let eq155_e1967_d_b31: f64 = (p.p7 * eq155_e1965_d_b31);
        let eq155_e1967_d_b32: f64 = (p.p7 * eq155_e1965_d_b32);
        let eq155_e1967_d_b33: f64 = (p.p7 * eq155_e1965_d_b33);
        let eq155_e1967_d_b34: f64 = (p.p7 * eq155_e1965_d_b34);
        let eq155_e1967_d_b35: f64 = (p.p7 * eq155_e1965_d_b35);
        let eq155_e1967_d_b36: f64 = (p.p7 * eq155_e1965_d_b36);
        let eq155_e1967_d_b37: f64 = (p.p7 * eq155_e1965_d_b37);
        let eq155_e1967_d_b38: f64 = (p.p7 * eq155_e1965_d_b38);
        let eq155_e1967_d_b39: f64 = (p.p7 * eq155_e1965_d_b39);
        let eq155_e1967_d_b40: f64 = (p.p7 * eq155_e1965_d_b40);
        let eq155_e1967_d_b41: f64 = (p.p7 * eq155_e1965_d_b41);
        let eq155_e1967_d_b42: f64 = (p.p7 * eq155_e1965_d_b42);
        let eq155_e1967_d_b43: f64 = (p.p7 * eq155_e1965_d_b43);
        let eq155_e1967_d_b44: f64 = (p.p7 * eq155_e1965_d_b44);
        let eq155_e1967_d_b45: f64 = (p.p7 * eq155_e1965_d_b45);
        let eq155_e1967_d_b46: f64 = (p.p7 * eq155_e1965_d_b46);
        let eq155_e1967_d_b47: f64 = (p.p7 * eq155_e1965_d_b47);
        let eq155_e1967_d_b48: f64 = (p.p7 * eq155_e1965_d_b48);
        let eq155_e1967_d_b49: f64 = (p.p7 * eq155_e1965_d_b49);
        let eq155_e1967_d_b50: f64 = (p.p7 * eq155_e1965_d_b50);
        let eq155_e1967_d_b51: f64 = (p.p7 * eq155_e1965_d_b51);
        let eq155_e1967_d_b52: f64 = (p.p7 * eq155_e1965_d_b52);
        let eq155_e1967_d_b53: f64 = (p.p7 * eq155_e1965_d_b53);
        let eq155_e1967_d_b54: f64 = (p.p7 * eq155_e1965_d_b54);
        let eq155_e1967_q: f64 = (p.p7 * eq155_e1966_q);
        (eq155_e1967, eq155_e1967_d_n0, eq155_e1967_d_n1, eq155_e1967_d_n2, eq155_e1967_d_n3, eq155_e1967_d_n4, eq155_e1967_d_n5, eq155_e1967_d_n6, eq155_e1967_d_n7, eq155_e1967_d_n8, eq155_e1967_d_n9, eq155_e1967_d_n10, eq155_e1967_d_n11, eq155_e1967_d_n12, eq155_e1967_d_n13, eq155_e1967_d_n14, eq155_e1967_d_n15, eq155_e1967_d_n16, eq155_e1967_d_n17, eq155_e1967_d_n18, eq155_e1967_d_n19, eq155_e1967_d_n20, eq155_e1967_d_n21, eq155_e1967_d_n22, eq155_e1967_d_b0, eq155_e1967_d_b1, eq155_e1967_d_b2, eq155_e1967_d_b3, eq155_e1967_d_b4, eq155_e1967_d_b5, eq155_e1967_d_b6, eq155_e1967_d_b7, eq155_e1967_d_b8, eq155_e1967_d_b9, eq155_e1967_d_b10, eq155_e1967_d_b11, eq155_e1967_d_b12, eq155_e1967_d_b13, eq155_e1967_d_b14, eq155_e1967_d_b15, eq155_e1967_d_b16, eq155_e1967_d_b17, eq155_e1967_d_b18, eq155_e1967_d_b19, eq155_e1967_d_b20, eq155_e1967_d_b21, eq155_e1967_d_b22, eq155_e1967_d_b23, eq155_e1967_d_b24, eq155_e1967_d_b25, eq155_e1967_d_b26, eq155_e1967_d_b27, eq155_e1967_d_b28, eq155_e1967_d_b29, eq155_e1967_d_b30, eq155_e1967_d_b31, eq155_e1967_d_b32, eq155_e1967_d_b33, eq155_e1967_d_b34, eq155_e1967_d_b35, eq155_e1967_d_b36, eq155_e1967_d_b37, eq155_e1967_d_b38, eq155_e1967_d_b39, eq155_e1967_d_b40, eq155_e1967_d_b41, eq155_e1967_d_b42, eq155_e1967_d_b43, eq155_e1967_d_b44, eq155_e1967_d_b45, eq155_e1967_d_b46, eq155_e1967_d_b47, eq155_e1967_d_b48, eq155_e1967_d_b49, eq155_e1967_d_b50, eq155_e1967_d_b51, eq155_e1967_d_b52, eq155_e1967_d_b53, eq155_e1967_d_b54, eq155_e1967_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_reactive_node_derivatives: [f64; 23] = [eq155_e1969_d_n0, eq155_e1969_d_n1, eq155_e1969_d_n2, eq155_e1969_d_n3, eq155_e1969_d_n4, eq155_e1969_d_n5, eq155_e1969_d_n6, eq155_e1969_d_n7, eq155_e1969_d_n8, eq155_e1969_d_n9, eq155_e1969_d_n10, eq155_e1969_d_n11, eq155_e1969_d_n12, eq155_e1969_d_n13, eq155_e1969_d_n14, eq155_e1969_d_n15, eq155_e1969_d_n16, eq155_e1969_d_n17, eq155_e1969_d_n18, eq155_e1969_d_n19, eq155_e1969_d_n20, eq155_e1969_d_n21, eq155_e1969_d_n22];
        let eq155_reactive_branch_derivatives: [f64; 55] = [eq155_e1969_d_b0, eq155_e1969_d_b1, eq155_e1969_d_b2, eq155_e1969_d_b3, eq155_e1969_d_b4, eq155_e1969_d_b5, eq155_e1969_d_b6, eq155_e1969_d_b7, eq155_e1969_d_b8, eq155_e1969_d_b9, eq155_e1969_d_b10, eq155_e1969_d_b11, eq155_e1969_d_b12, eq155_e1969_d_b13, eq155_e1969_d_b14, eq155_e1969_d_b15, eq155_e1969_d_b16, eq155_e1969_d_b17, eq155_e1969_d_b18, eq155_e1969_d_b19, eq155_e1969_d_b20, eq155_e1969_d_b21, eq155_e1969_d_b22, eq155_e1969_d_b23, eq155_e1969_d_b24, eq155_e1969_d_b25, eq155_e1969_d_b26, eq155_e1969_d_b27, eq155_e1969_d_b28, eq155_e1969_d_b29, eq155_e1969_d_b30, eq155_e1969_d_b31, eq155_e1969_d_b32, eq155_e1969_d_b33, eq155_e1969_d_b34, eq155_e1969_d_b35, eq155_e1969_d_b36, eq155_e1969_d_b37, eq155_e1969_d_b38, eq155_e1969_d_b39, eq155_e1969_d_b40, eq155_e1969_d_b41, eq155_e1969_d_b42, eq155_e1969_d_b43, eq155_e1969_d_b44, eq155_e1969_d_b45, eq155_e1969_d_b46, eq155_e1969_d_b47, eq155_e1969_d_b48, eq155_e1969_d_b49, eq155_e1969_d_b50, eq155_e1969_d_b51, eq155_e1969_d_b52, eq155_e1969_d_b53, eq155_e1969_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq155_reactive_node_derivatives,
            branches,
            &eq155_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq156_e1978, eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22, eq156_e1978_d_b0, eq156_e1978_d_b1, eq156_e1978_d_b2, eq156_e1978_d_b3, eq156_e1978_d_b4, eq156_e1978_d_b5, eq156_e1978_d_b6, eq156_e1978_d_b7, eq156_e1978_d_b8, eq156_e1978_d_b9, eq156_e1978_d_b10, eq156_e1978_d_b11, eq156_e1978_d_b12, eq156_e1978_d_b13, eq156_e1978_d_b14, eq156_e1978_d_b15, eq156_e1978_d_b16, eq156_e1978_d_b17, eq156_e1978_d_b18, eq156_e1978_d_b19, eq156_e1978_d_b20, eq156_e1978_d_b21, eq156_e1978_d_b22, eq156_e1978_d_b23, eq156_e1978_d_b24, eq156_e1978_d_b25, eq156_e1978_d_b26, eq156_e1978_d_b27, eq156_e1978_d_b28, eq156_e1978_d_b29, eq156_e1978_d_b30, eq156_e1978_d_b31, eq156_e1978_d_b32, eq156_e1978_d_b33, eq156_e1978_d_b34, eq156_e1978_d_b35, eq156_e1978_d_b36, eq156_e1978_d_b37, eq156_e1978_d_b38, eq156_e1978_d_b39, eq156_e1978_d_b40, eq156_e1978_d_b41, eq156_e1978_d_b42, eq156_e1978_d_b43, eq156_e1978_d_b44, eq156_e1978_d_b45, eq156_e1978_d_b46, eq156_e1978_d_b47, eq156_e1978_d_b48, eq156_e1978_d_b49, eq156_e1978_d_b50, eq156_e1978_d_b51, eq156_e1978_d_b52, eq156_e1978_d_b53, eq156_e1978_d_b54, eq156_e1978_q,) = {
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
        let eq156_e1976_d_b0: f64 = (p.p7 * s.db[265][0]);
        let eq156_e1976_d_b1: f64 = (p.p7 * s.db[265][1]);
        let eq156_e1976_d_b2: f64 = (p.p7 * s.db[265][2]);
        let eq156_e1976_d_b3: f64 = (p.p7 * s.db[265][3]);
        let eq156_e1976_d_b4: f64 = (p.p7 * s.db[265][4]);
        let eq156_e1976_d_b5: f64 = (p.p7 * s.db[265][5]);
        let eq156_e1976_d_b6: f64 = (p.p7 * s.db[265][6]);
        let eq156_e1976_d_b7: f64 = (p.p7 * s.db[265][7]);
        let eq156_e1976_d_b8: f64 = (p.p7 * s.db[265][8]);
        let eq156_e1976_d_b9: f64 = (p.p7 * s.db[265][9]);
        let eq156_e1976_d_b10: f64 = (p.p7 * s.db[265][10]);
        let eq156_e1976_d_b11: f64 = (p.p7 * s.db[265][11]);
        let eq156_e1976_d_b12: f64 = (p.p7 * s.db[265][12]);
        let eq156_e1976_d_b13: f64 = (p.p7 * s.db[265][13]);
        let eq156_e1976_d_b14: f64 = (p.p7 * s.db[265][14]);
        let eq156_e1976_d_b15: f64 = (p.p7 * s.db[265][15]);
        let eq156_e1976_d_b16: f64 = (p.p7 * s.db[265][16]);
        let eq156_e1976_d_b17: f64 = (p.p7 * s.db[265][17]);
        let eq156_e1976_d_b18: f64 = (p.p7 * s.db[265][18]);
        let eq156_e1976_d_b19: f64 = (p.p7 * s.db[265][19]);
        let eq156_e1976_d_b20: f64 = (p.p7 * s.db[265][20]);
        let eq156_e1976_d_b21: f64 = (p.p7 * s.db[265][21]);
        let eq156_e1976_d_b22: f64 = (p.p7 * s.db[265][22]);
        let eq156_e1976_d_b23: f64 = (p.p7 * s.db[265][23]);
        let eq156_e1976_d_b24: f64 = (p.p7 * s.db[265][24]);
        let eq156_e1976_d_b25: f64 = (p.p7 * s.db[265][25]);
        let eq156_e1976_d_b26: f64 = (p.p7 * s.db[265][26]);
        let eq156_e1976_d_b27: f64 = (p.p7 * s.db[265][27]);
        let eq156_e1976_d_b28: f64 = (p.p7 * s.db[265][28]);
        let eq156_e1976_d_b29: f64 = (p.p7 * s.db[265][29]);
        let eq156_e1976_d_b30: f64 = (p.p7 * s.db[265][30]);
        let eq156_e1976_d_b31: f64 = (p.p7 * s.db[265][31]);
        let eq156_e1976_d_b32: f64 = (p.p7 * s.db[265][32]);
        let eq156_e1976_d_b33: f64 = (p.p7 * s.db[265][33]);
        let eq156_e1976_d_b34: f64 = (p.p7 * s.db[265][34]);
        let eq156_e1976_d_b35: f64 = (p.p7 * s.db[265][35]);
        let eq156_e1976_d_b36: f64 = (p.p7 * s.db[265][36]);
        let eq156_e1976_d_b37: f64 = (p.p7 * s.db[265][37]);
        let eq156_e1976_d_b38: f64 = (p.p7 * s.db[265][38]);
        let eq156_e1976_d_b39: f64 = (p.p7 * s.db[265][39]);
        let eq156_e1976_d_b40: f64 = (p.p7 * s.db[265][40]);
        let eq156_e1976_d_b41: f64 = (p.p7 * s.db[265][41]);
        let eq156_e1976_d_b42: f64 = (p.p7 * s.db[265][42]);
        let eq156_e1976_d_b43: f64 = (p.p7 * s.db[265][43]);
        let eq156_e1976_d_b44: f64 = (p.p7 * s.db[265][44]);
        let eq156_e1976_d_b45: f64 = (p.p7 * s.db[265][45]);
        let eq156_e1976_d_b46: f64 = (p.p7 * s.db[265][46]);
        let eq156_e1976_d_b47: f64 = (p.p7 * s.db[265][47]);
        let eq156_e1976_d_b48: f64 = (p.p7 * s.db[265][48]);
        let eq156_e1976_d_b49: f64 = (p.p7 * s.db[265][49]);
        let eq156_e1976_d_b50: f64 = (p.p7 * s.db[265][50]);
        let eq156_e1976_d_b51: f64 = (p.p7 * s.db[265][51]);
        let eq156_e1976_d_b52: f64 = (p.p7 * s.db[265][52]);
        let eq156_e1976_d_b53: f64 = (p.p7 * s.db[265][53]);
        let eq156_e1976_d_b54: f64 = (p.p7 * s.db[265][54]);
        let eq156_e1976_q: f64 = (p.p7 * eq156_e1975_q);
        (eq156_e1976, eq156_e1976_d_n0, eq156_e1976_d_n1, eq156_e1976_d_n2, eq156_e1976_d_n3, eq156_e1976_d_n4, eq156_e1976_d_n5, eq156_e1976_d_n6, eq156_e1976_d_n7, eq156_e1976_d_n8, eq156_e1976_d_n9, eq156_e1976_d_n10, eq156_e1976_d_n11, eq156_e1976_d_n12, eq156_e1976_d_n13, eq156_e1976_d_n14, eq156_e1976_d_n15, eq156_e1976_d_n16, eq156_e1976_d_n17, eq156_e1976_d_n18, eq156_e1976_d_n19, eq156_e1976_d_n20, eq156_e1976_d_n21, eq156_e1976_d_n22, eq156_e1976_d_b0, eq156_e1976_d_b1, eq156_e1976_d_b2, eq156_e1976_d_b3, eq156_e1976_d_b4, eq156_e1976_d_b5, eq156_e1976_d_b6, eq156_e1976_d_b7, eq156_e1976_d_b8, eq156_e1976_d_b9, eq156_e1976_d_b10, eq156_e1976_d_b11, eq156_e1976_d_b12, eq156_e1976_d_b13, eq156_e1976_d_b14, eq156_e1976_d_b15, eq156_e1976_d_b16, eq156_e1976_d_b17, eq156_e1976_d_b18, eq156_e1976_d_b19, eq156_e1976_d_b20, eq156_e1976_d_b21, eq156_e1976_d_b22, eq156_e1976_d_b23, eq156_e1976_d_b24, eq156_e1976_d_b25, eq156_e1976_d_b26, eq156_e1976_d_b27, eq156_e1976_d_b28, eq156_e1976_d_b29, eq156_e1976_d_b30, eq156_e1976_d_b31, eq156_e1976_d_b32, eq156_e1976_d_b33, eq156_e1976_d_b34, eq156_e1976_d_b35, eq156_e1976_d_b36, eq156_e1976_d_b37, eq156_e1976_d_b38, eq156_e1976_d_b39, eq156_e1976_d_b40, eq156_e1976_d_b41, eq156_e1976_d_b42, eq156_e1976_d_b43, eq156_e1976_d_b44, eq156_e1976_d_b45, eq156_e1976_d_b46, eq156_e1976_d_b47, eq156_e1976_d_b48, eq156_e1976_d_b49, eq156_e1976_d_b50, eq156_e1976_d_b51, eq156_e1976_d_b52, eq156_e1976_d_b53, eq156_e1976_d_b54, eq156_e1976_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_reactive_node_derivatives: [f64; 23] = [eq156_e1978_d_n0, eq156_e1978_d_n1, eq156_e1978_d_n2, eq156_e1978_d_n3, eq156_e1978_d_n4, eq156_e1978_d_n5, eq156_e1978_d_n6, eq156_e1978_d_n7, eq156_e1978_d_n8, eq156_e1978_d_n9, eq156_e1978_d_n10, eq156_e1978_d_n11, eq156_e1978_d_n12, eq156_e1978_d_n13, eq156_e1978_d_n14, eq156_e1978_d_n15, eq156_e1978_d_n16, eq156_e1978_d_n17, eq156_e1978_d_n18, eq156_e1978_d_n19, eq156_e1978_d_n20, eq156_e1978_d_n21, eq156_e1978_d_n22];
        let eq156_reactive_branch_derivatives: [f64; 55] = [eq156_e1978_d_b0, eq156_e1978_d_b1, eq156_e1978_d_b2, eq156_e1978_d_b3, eq156_e1978_d_b4, eq156_e1978_d_b5, eq156_e1978_d_b6, eq156_e1978_d_b7, eq156_e1978_d_b8, eq156_e1978_d_b9, eq156_e1978_d_b10, eq156_e1978_d_b11, eq156_e1978_d_b12, eq156_e1978_d_b13, eq156_e1978_d_b14, eq156_e1978_d_b15, eq156_e1978_d_b16, eq156_e1978_d_b17, eq156_e1978_d_b18, eq156_e1978_d_b19, eq156_e1978_d_b20, eq156_e1978_d_b21, eq156_e1978_d_b22, eq156_e1978_d_b23, eq156_e1978_d_b24, eq156_e1978_d_b25, eq156_e1978_d_b26, eq156_e1978_d_b27, eq156_e1978_d_b28, eq156_e1978_d_b29, eq156_e1978_d_b30, eq156_e1978_d_b31, eq156_e1978_d_b32, eq156_e1978_d_b33, eq156_e1978_d_b34, eq156_e1978_d_b35, eq156_e1978_d_b36, eq156_e1978_d_b37, eq156_e1978_d_b38, eq156_e1978_d_b39, eq156_e1978_d_b40, eq156_e1978_d_b41, eq156_e1978_d_b42, eq156_e1978_d_b43, eq156_e1978_d_b44, eq156_e1978_d_b45, eq156_e1978_d_b46, eq156_e1978_d_b47, eq156_e1978_d_b48, eq156_e1978_d_b49, eq156_e1978_d_b50, eq156_e1978_d_b51, eq156_e1978_d_b52, eq156_e1978_d_b53, eq156_e1978_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[20]),
            nodes,
            &eq156_reactive_node_derivatives,
            branches,
            &eq156_reactive_branch_derivatives,
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
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[264][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[264][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[264][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[264][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[264][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[264][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[264][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[264][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[264][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[264][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[264][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[264][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[264][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[264][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[264][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[264][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[264][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[264][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[264][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[264][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[264][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[264][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[264][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[264][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[264][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[264][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[264][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[264][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[264][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[264][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[264][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[264][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[264][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[264][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[264][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[264][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[264][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[264][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[264][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[264][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[264][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[264][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[264][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[264][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[264][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[264][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[264][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[264][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[264][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[264][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[264][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[264][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[264][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[264][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[264][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[264][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[264][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[264][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[264][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[264][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[264][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[264][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[264][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[264][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[264][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[264][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[264][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[264][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[264][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[264][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[264][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[264][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[264][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[264][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[264][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[264][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[264][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[264][54]);
        let (eq157_e1989, eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22, eq157_e1989_d_b0, eq157_e1989_d_b1, eq157_e1989_d_b2, eq157_e1989_d_b3, eq157_e1989_d_b4, eq157_e1989_d_b5, eq157_e1989_d_b6, eq157_e1989_d_b7, eq157_e1989_d_b8, eq157_e1989_d_b9, eq157_e1989_d_b10, eq157_e1989_d_b11, eq157_e1989_d_b12, eq157_e1989_d_b13, eq157_e1989_d_b14, eq157_e1989_d_b15, eq157_e1989_d_b16, eq157_e1989_d_b17, eq157_e1989_d_b18, eq157_e1989_d_b19, eq157_e1989_d_b20, eq157_e1989_d_b21, eq157_e1989_d_b22, eq157_e1989_d_b23, eq157_e1989_d_b24, eq157_e1989_d_b25, eq157_e1989_d_b26, eq157_e1989_d_b27, eq157_e1989_d_b28, eq157_e1989_d_b29, eq157_e1989_d_b30, eq157_e1989_d_b31, eq157_e1989_d_b32, eq157_e1989_d_b33, eq157_e1989_d_b34, eq157_e1989_d_b35, eq157_e1989_d_b36, eq157_e1989_d_b37, eq157_e1989_d_b38, eq157_e1989_d_b39, eq157_e1989_d_b40, eq157_e1989_d_b41, eq157_e1989_d_b42, eq157_e1989_d_b43, eq157_e1989_d_b44, eq157_e1989_d_b45, eq157_e1989_d_b46, eq157_e1989_d_b47, eq157_e1989_d_b48, eq157_e1989_d_b49, eq157_e1989_d_b50, eq157_e1989_d_b51, eq157_e1989_d_b52, eq157_e1989_d_b53, eq157_e1989_d_b54, eq157_e1989_q,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq157_e1986_q: f64 = s.v[264];
        let eq157_e1987: f64 = (p.p7 * s.v[264]);
        let eq157_e1987_q: f64 = (p.p7 * eq157_e1986_q);
        (eq157_e1987, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq157_e1987_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq157_reactive_node_derivatives: [f64; 23] = [eq157_e1989_d_n0, eq157_e1989_d_n1, eq157_e1989_d_n2, eq157_e1989_d_n3, eq157_e1989_d_n4, eq157_e1989_d_n5, eq157_e1989_d_n6, eq157_e1989_d_n7, eq157_e1989_d_n8, eq157_e1989_d_n9, eq157_e1989_d_n10, eq157_e1989_d_n11, eq157_e1989_d_n12, eq157_e1989_d_n13, eq157_e1989_d_n14, eq157_e1989_d_n15, eq157_e1989_d_n16, eq157_e1989_d_n17, eq157_e1989_d_n18, eq157_e1989_d_n19, eq157_e1989_d_n20, eq157_e1989_d_n21, eq157_e1989_d_n22];
        let eq157_reactive_branch_derivatives: [f64; 55] = [eq157_e1989_d_b0, eq157_e1989_d_b1, eq157_e1989_d_b2, eq157_e1989_d_b3, eq157_e1989_d_b4, eq157_e1989_d_b5, eq157_e1989_d_b6, eq157_e1989_d_b7, eq157_e1989_d_b8, eq157_e1989_d_b9, eq157_e1989_d_b10, eq157_e1989_d_b11, eq157_e1989_d_b12, eq157_e1989_d_b13, eq157_e1989_d_b14, eq157_e1989_d_b15, eq157_e1989_d_b16, eq157_e1989_d_b17, eq157_e1989_d_b18, eq157_e1989_d_b19, eq157_e1989_d_b20, eq157_e1989_d_b21, eq157_e1989_d_b22, eq157_e1989_d_b23, eq157_e1989_d_b24, eq157_e1989_d_b25, eq157_e1989_d_b26, eq157_e1989_d_b27, eq157_e1989_d_b28, eq157_e1989_d_b29, eq157_e1989_d_b30, eq157_e1989_d_b31, eq157_e1989_d_b32, eq157_e1989_d_b33, eq157_e1989_d_b34, eq157_e1989_d_b35, eq157_e1989_d_b36, eq157_e1989_d_b37, eq157_e1989_d_b38, eq157_e1989_d_b39, eq157_e1989_d_b40, eq157_e1989_d_b41, eq157_e1989_d_b42, eq157_e1989_d_b43, eq157_e1989_d_b44, eq157_e1989_d_b45, eq157_e1989_d_b46, eq157_e1989_d_b47, eq157_e1989_d_b48, eq157_e1989_d_b49, eq157_e1989_d_b50, eq157_e1989_d_b51, eq157_e1989_d_b52, eq157_e1989_d_b53, eq157_e1989_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            nodes,
            &eq157_reactive_node_derivatives,
            branches,
            &eq157_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq158_e2002, eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22, eq158_e2002_d_b0, eq158_e2002_d_b1, eq158_e2002_d_b2, eq158_e2002_d_b3, eq158_e2002_d_b4, eq158_e2002_d_b5, eq158_e2002_d_b6, eq158_e2002_d_b7, eq158_e2002_d_b8, eq158_e2002_d_b9, eq158_e2002_d_b10, eq158_e2002_d_b11, eq158_e2002_d_b12, eq158_e2002_d_b13, eq158_e2002_d_b14, eq158_e2002_d_b15, eq158_e2002_d_b16, eq158_e2002_d_b17, eq158_e2002_d_b18, eq158_e2002_d_b19, eq158_e2002_d_b20, eq158_e2002_d_b21, eq158_e2002_d_b22, eq158_e2002_d_b23, eq158_e2002_d_b24, eq158_e2002_d_b25, eq158_e2002_d_b26, eq158_e2002_d_b27, eq158_e2002_d_b28, eq158_e2002_d_b29, eq158_e2002_d_b30, eq158_e2002_d_b31, eq158_e2002_d_b32, eq158_e2002_d_b33, eq158_e2002_d_b34, eq158_e2002_d_b35, eq158_e2002_d_b36, eq158_e2002_d_b37, eq158_e2002_d_b38, eq158_e2002_d_b39, eq158_e2002_d_b40, eq158_e2002_d_b41, eq158_e2002_d_b42, eq158_e2002_d_b43, eq158_e2002_d_b44, eq158_e2002_d_b45, eq158_e2002_d_b46, eq158_e2002_d_b47, eq158_e2002_d_b48, eq158_e2002_d_b49, eq158_e2002_d_b50, eq158_e2002_d_b51, eq158_e2002_d_b52, eq158_e2002_d_b53, eq158_e2002_d_b54, eq158_e2002_q,) = {
    if ((s.b[585] && s.b[586]) && s.b[587]) {
        let eq158_e1997_q: f64 = s.v[264];
        let eq158_e1998: f64 = (p.p7 * s.v[264]);
        let eq158_e1998_q: f64 = (p.p7 * eq158_e1997_q);
        let eq158_e2000: f64 = (eq158_e1998 * p.p247);
        let eq158_e2000_d_n0: f64 = (__rspice_deriv_cse_0 * p.p247);
        let eq158_e2000_d_n1: f64 = (__rspice_deriv_cse_1 * p.p247);
        let eq158_e2000_d_n2: f64 = (__rspice_deriv_cse_2 * p.p247);
        let eq158_e2000_d_n3: f64 = (__rspice_deriv_cse_3 * p.p247);
        let eq158_e2000_d_n4: f64 = (__rspice_deriv_cse_4 * p.p247);
        let eq158_e2000_d_n5: f64 = (__rspice_deriv_cse_5 * p.p247);
        let eq158_e2000_d_n6: f64 = (__rspice_deriv_cse_6 * p.p247);
        let eq158_e2000_d_n7: f64 = (__rspice_deriv_cse_7 * p.p247);
        let eq158_e2000_d_n8: f64 = (__rspice_deriv_cse_8 * p.p247);
        let eq158_e2000_d_n9: f64 = (__rspice_deriv_cse_9 * p.p247);
        let eq158_e2000_d_n10: f64 = (__rspice_deriv_cse_10 * p.p247);
        let eq158_e2000_d_n11: f64 = (__rspice_deriv_cse_11 * p.p247);
        let eq158_e2000_d_n12: f64 = (__rspice_deriv_cse_12 * p.p247);
        let eq158_e2000_d_n13: f64 = (__rspice_deriv_cse_13 * p.p247);
        let eq158_e2000_d_n14: f64 = (__rspice_deriv_cse_14 * p.p247);
        let eq158_e2000_d_n15: f64 = (__rspice_deriv_cse_15 * p.p247);
        let eq158_e2000_d_n16: f64 = (__rspice_deriv_cse_16 * p.p247);
        let eq158_e2000_d_n17: f64 = (__rspice_deriv_cse_17 * p.p247);
        let eq158_e2000_d_n18: f64 = (__rspice_deriv_cse_18 * p.p247);
        let eq158_e2000_d_n19: f64 = (__rspice_deriv_cse_19 * p.p247);
        let eq158_e2000_d_n20: f64 = (__rspice_deriv_cse_20 * p.p247);
        let eq158_e2000_d_n21: f64 = (__rspice_deriv_cse_21 * p.p247);
        let eq158_e2000_d_n22: f64 = (__rspice_deriv_cse_22 * p.p247);
        let eq158_e2000_d_b0: f64 = (__rspice_deriv_cse_23 * p.p247);
        let eq158_e2000_d_b1: f64 = (__rspice_deriv_cse_24 * p.p247);
        let eq158_e2000_d_b2: f64 = (__rspice_deriv_cse_25 * p.p247);
        let eq158_e2000_d_b3: f64 = (__rspice_deriv_cse_26 * p.p247);
        let eq158_e2000_d_b4: f64 = (__rspice_deriv_cse_27 * p.p247);
        let eq158_e2000_d_b5: f64 = (__rspice_deriv_cse_28 * p.p247);
        let eq158_e2000_d_b6: f64 = (__rspice_deriv_cse_29 * p.p247);
        let eq158_e2000_d_b7: f64 = (__rspice_deriv_cse_30 * p.p247);
        let eq158_e2000_d_b8: f64 = (__rspice_deriv_cse_31 * p.p247);
        let eq158_e2000_d_b9: f64 = (__rspice_deriv_cse_32 * p.p247);
        let eq158_e2000_d_b10: f64 = (__rspice_deriv_cse_33 * p.p247);
        let eq158_e2000_d_b11: f64 = (__rspice_deriv_cse_34 * p.p247);
        let eq158_e2000_d_b12: f64 = (__rspice_deriv_cse_35 * p.p247);
        let eq158_e2000_d_b13: f64 = (__rspice_deriv_cse_36 * p.p247);
        let eq158_e2000_d_b14: f64 = (__rspice_deriv_cse_37 * p.p247);
        let eq158_e2000_d_b15: f64 = (__rspice_deriv_cse_38 * p.p247);
        let eq158_e2000_d_b16: f64 = (__rspice_deriv_cse_39 * p.p247);
        let eq158_e2000_d_b17: f64 = (__rspice_deriv_cse_40 * p.p247);
        let eq158_e2000_d_b18: f64 = (__rspice_deriv_cse_41 * p.p247);
        let eq158_e2000_d_b19: f64 = (__rspice_deriv_cse_42 * p.p247);
        let eq158_e2000_d_b20: f64 = (__rspice_deriv_cse_43 * p.p247);
        let eq158_e2000_d_b21: f64 = (__rspice_deriv_cse_44 * p.p247);
        let eq158_e2000_d_b22: f64 = (__rspice_deriv_cse_45 * p.p247);
        let eq158_e2000_d_b23: f64 = (__rspice_deriv_cse_46 * p.p247);
        let eq158_e2000_d_b24: f64 = (__rspice_deriv_cse_47 * p.p247);
        let eq158_e2000_d_b25: f64 = (__rspice_deriv_cse_48 * p.p247);
        let eq158_e2000_d_b26: f64 = (__rspice_deriv_cse_49 * p.p247);
        let eq158_e2000_d_b27: f64 = (__rspice_deriv_cse_50 * p.p247);
        let eq158_e2000_d_b28: f64 = (__rspice_deriv_cse_51 * p.p247);
        let eq158_e2000_d_b29: f64 = (__rspice_deriv_cse_52 * p.p247);
        let eq158_e2000_d_b30: f64 = (__rspice_deriv_cse_53 * p.p247);
        let eq158_e2000_d_b31: f64 = (__rspice_deriv_cse_54 * p.p247);
        let eq158_e2000_d_b32: f64 = (__rspice_deriv_cse_55 * p.p247);
        let eq158_e2000_d_b33: f64 = (__rspice_deriv_cse_56 * p.p247);
        let eq158_e2000_d_b34: f64 = (__rspice_deriv_cse_57 * p.p247);
        let eq158_e2000_d_b35: f64 = (__rspice_deriv_cse_58 * p.p247);
        let eq158_e2000_d_b36: f64 = (__rspice_deriv_cse_59 * p.p247);
        let eq158_e2000_d_b37: f64 = (__rspice_deriv_cse_60 * p.p247);
        let eq158_e2000_d_b38: f64 = (__rspice_deriv_cse_61 * p.p247);
        let eq158_e2000_d_b39: f64 = (__rspice_deriv_cse_62 * p.p247);
        let eq158_e2000_d_b40: f64 = (__rspice_deriv_cse_63 * p.p247);
        let eq158_e2000_d_b41: f64 = (__rspice_deriv_cse_64 * p.p247);
        let eq158_e2000_d_b42: f64 = (__rspice_deriv_cse_65 * p.p247);
        let eq158_e2000_d_b43: f64 = (__rspice_deriv_cse_66 * p.p247);
        let eq158_e2000_d_b44: f64 = (__rspice_deriv_cse_67 * p.p247);
        let eq158_e2000_d_b45: f64 = (__rspice_deriv_cse_68 * p.p247);
        let eq158_e2000_d_b46: f64 = (__rspice_deriv_cse_69 * p.p247);
        let eq158_e2000_d_b47: f64 = (__rspice_deriv_cse_70 * p.p247);
        let eq158_e2000_d_b48: f64 = (__rspice_deriv_cse_71 * p.p247);
        let eq158_e2000_d_b49: f64 = (__rspice_deriv_cse_72 * p.p247);
        let eq158_e2000_d_b50: f64 = (__rspice_deriv_cse_73 * p.p247);
        let eq158_e2000_d_b51: f64 = (__rspice_deriv_cse_74 * p.p247);
        let eq158_e2000_d_b52: f64 = (__rspice_deriv_cse_75 * p.p247);
        let eq158_e2000_d_b53: f64 = (__rspice_deriv_cse_76 * p.p247);
        let eq158_e2000_d_b54: f64 = (__rspice_deriv_cse_77 * p.p247);
        let eq158_e2000_q: f64 = (eq158_e1998_q * p.p247);
        (eq158_e2000, eq158_e2000_d_n0, eq158_e2000_d_n1, eq158_e2000_d_n2, eq158_e2000_d_n3, eq158_e2000_d_n4, eq158_e2000_d_n5, eq158_e2000_d_n6, eq158_e2000_d_n7, eq158_e2000_d_n8, eq158_e2000_d_n9, eq158_e2000_d_n10, eq158_e2000_d_n11, eq158_e2000_d_n12, eq158_e2000_d_n13, eq158_e2000_d_n14, eq158_e2000_d_n15, eq158_e2000_d_n16, eq158_e2000_d_n17, eq158_e2000_d_n18, eq158_e2000_d_n19, eq158_e2000_d_n20, eq158_e2000_d_n21, eq158_e2000_d_n22, eq158_e2000_d_b0, eq158_e2000_d_b1, eq158_e2000_d_b2, eq158_e2000_d_b3, eq158_e2000_d_b4, eq158_e2000_d_b5, eq158_e2000_d_b6, eq158_e2000_d_b7, eq158_e2000_d_b8, eq158_e2000_d_b9, eq158_e2000_d_b10, eq158_e2000_d_b11, eq158_e2000_d_b12, eq158_e2000_d_b13, eq158_e2000_d_b14, eq158_e2000_d_b15, eq158_e2000_d_b16, eq158_e2000_d_b17, eq158_e2000_d_b18, eq158_e2000_d_b19, eq158_e2000_d_b20, eq158_e2000_d_b21, eq158_e2000_d_b22, eq158_e2000_d_b23, eq158_e2000_d_b24, eq158_e2000_d_b25, eq158_e2000_d_b26, eq158_e2000_d_b27, eq158_e2000_d_b28, eq158_e2000_d_b29, eq158_e2000_d_b30, eq158_e2000_d_b31, eq158_e2000_d_b32, eq158_e2000_d_b33, eq158_e2000_d_b34, eq158_e2000_d_b35, eq158_e2000_d_b36, eq158_e2000_d_b37, eq158_e2000_d_b38, eq158_e2000_d_b39, eq158_e2000_d_b40, eq158_e2000_d_b41, eq158_e2000_d_b42, eq158_e2000_d_b43, eq158_e2000_d_b44, eq158_e2000_d_b45, eq158_e2000_d_b46, eq158_e2000_d_b47, eq158_e2000_d_b48, eq158_e2000_d_b49, eq158_e2000_d_b50, eq158_e2000_d_b51, eq158_e2000_d_b52, eq158_e2000_d_b53, eq158_e2000_d_b54, eq158_e2000_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq158_reactive_node_derivatives: [f64; 23] = [eq158_e2002_d_n0, eq158_e2002_d_n1, eq158_e2002_d_n2, eq158_e2002_d_n3, eq158_e2002_d_n4, eq158_e2002_d_n5, eq158_e2002_d_n6, eq158_e2002_d_n7, eq158_e2002_d_n8, eq158_e2002_d_n9, eq158_e2002_d_n10, eq158_e2002_d_n11, eq158_e2002_d_n12, eq158_e2002_d_n13, eq158_e2002_d_n14, eq158_e2002_d_n15, eq158_e2002_d_n16, eq158_e2002_d_n17, eq158_e2002_d_n18, eq158_e2002_d_n19, eq158_e2002_d_n20, eq158_e2002_d_n21, eq158_e2002_d_n22];
        let eq158_reactive_branch_derivatives: [f64; 55] = [eq158_e2002_d_b0, eq158_e2002_d_b1, eq158_e2002_d_b2, eq158_e2002_d_b3, eq158_e2002_d_b4, eq158_e2002_d_b5, eq158_e2002_d_b6, eq158_e2002_d_b7, eq158_e2002_d_b8, eq158_e2002_d_b9, eq158_e2002_d_b10, eq158_e2002_d_b11, eq158_e2002_d_b12, eq158_e2002_d_b13, eq158_e2002_d_b14, eq158_e2002_d_b15, eq158_e2002_d_b16, eq158_e2002_d_b17, eq158_e2002_d_b18, eq158_e2002_d_b19, eq158_e2002_d_b20, eq158_e2002_d_b21, eq158_e2002_d_b22, eq158_e2002_d_b23, eq158_e2002_d_b24, eq158_e2002_d_b25, eq158_e2002_d_b26, eq158_e2002_d_b27, eq158_e2002_d_b28, eq158_e2002_d_b29, eq158_e2002_d_b30, eq158_e2002_d_b31, eq158_e2002_d_b32, eq158_e2002_d_b33, eq158_e2002_d_b34, eq158_e2002_d_b35, eq158_e2002_d_b36, eq158_e2002_d_b37, eq158_e2002_d_b38, eq158_e2002_d_b39, eq158_e2002_d_b40, eq158_e2002_d_b41, eq158_e2002_d_b42, eq158_e2002_d_b43, eq158_e2002_d_b44, eq158_e2002_d_b45, eq158_e2002_d_b46, eq158_e2002_d_b47, eq158_e2002_d_b48, eq158_e2002_d_b49, eq158_e2002_d_b50, eq158_e2002_d_b51, eq158_e2002_d_b52, eq158_e2002_d_b53, eq158_e2002_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            nodes,
            &eq158_reactive_node_derivatives,
            branches,
            &eq158_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq159_e2014, eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22, eq159_e2014_d_b0, eq159_e2014_d_b1, eq159_e2014_d_b2, eq159_e2014_d_b3, eq159_e2014_d_b4, eq159_e2014_d_b5, eq159_e2014_d_b6, eq159_e2014_d_b7, eq159_e2014_d_b8, eq159_e2014_d_b9, eq159_e2014_d_b10, eq159_e2014_d_b11, eq159_e2014_d_b12, eq159_e2014_d_b13, eq159_e2014_d_b14, eq159_e2014_d_b15, eq159_e2014_d_b16, eq159_e2014_d_b17, eq159_e2014_d_b18, eq159_e2014_d_b19, eq159_e2014_d_b20, eq159_e2014_d_b21, eq159_e2014_d_b22, eq159_e2014_d_b23, eq159_e2014_d_b24, eq159_e2014_d_b25, eq159_e2014_d_b26, eq159_e2014_d_b27, eq159_e2014_d_b28, eq159_e2014_d_b29, eq159_e2014_d_b30, eq159_e2014_d_b31, eq159_e2014_d_b32, eq159_e2014_d_b33, eq159_e2014_d_b34, eq159_e2014_d_b35, eq159_e2014_d_b36, eq159_e2014_d_b37, eq159_e2014_d_b38, eq159_e2014_d_b39, eq159_e2014_d_b40, eq159_e2014_d_b41, eq159_e2014_d_b42, eq159_e2014_d_b43, eq159_e2014_d_b44, eq159_e2014_d_b45, eq159_e2014_d_b46, eq159_e2014_d_b47, eq159_e2014_d_b48, eq159_e2014_d_b49, eq159_e2014_d_b50, eq159_e2014_d_b51, eq159_e2014_d_b52, eq159_e2014_d_b53, eq159_e2014_d_b54, eq159_e2014_q,) = {
    if ((s.b[585] && s.b[586]) && (!s.b[587])) {
        let eq159_e2011_q: f64 = s.v[264];
        let eq159_e2012: f64 = (p.p7 * s.v[264]);
        let eq159_e2012_q: f64 = (p.p7 * eq159_e2011_q);
        (eq159_e2012, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq159_e2012_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq159_reactive_node_derivatives: [f64; 23] = [eq159_e2014_d_n0, eq159_e2014_d_n1, eq159_e2014_d_n2, eq159_e2014_d_n3, eq159_e2014_d_n4, eq159_e2014_d_n5, eq159_e2014_d_n6, eq159_e2014_d_n7, eq159_e2014_d_n8, eq159_e2014_d_n9, eq159_e2014_d_n10, eq159_e2014_d_n11, eq159_e2014_d_n12, eq159_e2014_d_n13, eq159_e2014_d_n14, eq159_e2014_d_n15, eq159_e2014_d_n16, eq159_e2014_d_n17, eq159_e2014_d_n18, eq159_e2014_d_n19, eq159_e2014_d_n20, eq159_e2014_d_n21, eq159_e2014_d_n22];
        let eq159_reactive_branch_derivatives: [f64; 55] = [eq159_e2014_d_b0, eq159_e2014_d_b1, eq159_e2014_d_b2, eq159_e2014_d_b3, eq159_e2014_d_b4, eq159_e2014_d_b5, eq159_e2014_d_b6, eq159_e2014_d_b7, eq159_e2014_d_b8, eq159_e2014_d_b9, eq159_e2014_d_b10, eq159_e2014_d_b11, eq159_e2014_d_b12, eq159_e2014_d_b13, eq159_e2014_d_b14, eq159_e2014_d_b15, eq159_e2014_d_b16, eq159_e2014_d_b17, eq159_e2014_d_b18, eq159_e2014_d_b19, eq159_e2014_d_b20, eq159_e2014_d_b21, eq159_e2014_d_b22, eq159_e2014_d_b23, eq159_e2014_d_b24, eq159_e2014_d_b25, eq159_e2014_d_b26, eq159_e2014_d_b27, eq159_e2014_d_b28, eq159_e2014_d_b29, eq159_e2014_d_b30, eq159_e2014_d_b31, eq159_e2014_d_b32, eq159_e2014_d_b33, eq159_e2014_d_b34, eq159_e2014_d_b35, eq159_e2014_d_b36, eq159_e2014_d_b37, eq159_e2014_d_b38, eq159_e2014_d_b39, eq159_e2014_d_b40, eq159_e2014_d_b41, eq159_e2014_d_b42, eq159_e2014_d_b43, eq159_e2014_d_b44, eq159_e2014_d_b45, eq159_e2014_d_b46, eq159_e2014_d_b47, eq159_e2014_d_b48, eq159_e2014_d_b49, eq159_e2014_d_b50, eq159_e2014_d_b51, eq159_e2014_d_b52, eq159_e2014_d_b53, eq159_e2014_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[20]),
            nodes,
            &eq159_reactive_node_derivatives,
            branches,
            &eq159_reactive_branch_derivatives,
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
        let (eq160_e2028, eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22, eq160_e2028_d_b0, eq160_e2028_d_b1, eq160_e2028_d_b2, eq160_e2028_d_b3, eq160_e2028_d_b4, eq160_e2028_d_b5, eq160_e2028_d_b6, eq160_e2028_d_b7, eq160_e2028_d_b8, eq160_e2028_d_b9, eq160_e2028_d_b10, eq160_e2028_d_b11, eq160_e2028_d_b12, eq160_e2028_d_b13, eq160_e2028_d_b14, eq160_e2028_d_b15, eq160_e2028_d_b16, eq160_e2028_d_b17, eq160_e2028_d_b18, eq160_e2028_d_b19, eq160_e2028_d_b20, eq160_e2028_d_b21, eq160_e2028_d_b22, eq160_e2028_d_b23, eq160_e2028_d_b24, eq160_e2028_d_b25, eq160_e2028_d_b26, eq160_e2028_d_b27, eq160_e2028_d_b28, eq160_e2028_d_b29, eq160_e2028_d_b30, eq160_e2028_d_b31, eq160_e2028_d_b32, eq160_e2028_d_b33, eq160_e2028_d_b34, eq160_e2028_d_b35, eq160_e2028_d_b36, eq160_e2028_d_b37, eq160_e2028_d_b38, eq160_e2028_d_b39, eq160_e2028_d_b40, eq160_e2028_d_b41, eq160_e2028_d_b42, eq160_e2028_d_b43, eq160_e2028_d_b44, eq160_e2028_d_b45, eq160_e2028_d_b46, eq160_e2028_d_b47, eq160_e2028_d_b48, eq160_e2028_d_b49, eq160_e2028_d_b50, eq160_e2028_d_b51, eq160_e2028_d_b52, eq160_e2028_d_b53, eq160_e2028_d_b54, eq160_e2028_q,) = {
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
        let eq160_e2024_d_b0: f64 = (p.p7 * s.db[264][0]);
        let eq160_e2024_d_b1: f64 = (p.p7 * s.db[264][1]);
        let eq160_e2024_d_b2: f64 = (p.p7 * s.db[264][2]);
        let eq160_e2024_d_b3: f64 = (p.p7 * s.db[264][3]);
        let eq160_e2024_d_b4: f64 = (p.p7 * s.db[264][4]);
        let eq160_e2024_d_b5: f64 = (p.p7 * s.db[264][5]);
        let eq160_e2024_d_b6: f64 = (p.p7 * s.db[264][6]);
        let eq160_e2024_d_b7: f64 = (p.p7 * s.db[264][7]);
        let eq160_e2024_d_b8: f64 = (p.p7 * s.db[264][8]);
        let eq160_e2024_d_b9: f64 = (p.p7 * s.db[264][9]);
        let eq160_e2024_d_b10: f64 = (p.p7 * s.db[264][10]);
        let eq160_e2024_d_b11: f64 = (p.p7 * s.db[264][11]);
        let eq160_e2024_d_b12: f64 = (p.p7 * s.db[264][12]);
        let eq160_e2024_d_b13: f64 = (p.p7 * s.db[264][13]);
        let eq160_e2024_d_b14: f64 = (p.p7 * s.db[264][14]);
        let eq160_e2024_d_b15: f64 = (p.p7 * s.db[264][15]);
        let eq160_e2024_d_b16: f64 = (p.p7 * s.db[264][16]);
        let eq160_e2024_d_b17: f64 = (p.p7 * s.db[264][17]);
        let eq160_e2024_d_b18: f64 = (p.p7 * s.db[264][18]);
        let eq160_e2024_d_b19: f64 = (p.p7 * s.db[264][19]);
        let eq160_e2024_d_b20: f64 = (p.p7 * s.db[264][20]);
        let eq160_e2024_d_b21: f64 = (p.p7 * s.db[264][21]);
        let eq160_e2024_d_b22: f64 = (p.p7 * s.db[264][22]);
        let eq160_e2024_d_b23: f64 = (p.p7 * s.db[264][23]);
        let eq160_e2024_d_b24: f64 = (p.p7 * s.db[264][24]);
        let eq160_e2024_d_b25: f64 = (p.p7 * s.db[264][25]);
        let eq160_e2024_d_b26: f64 = (p.p7 * s.db[264][26]);
        let eq160_e2024_d_b27: f64 = (p.p7 * s.db[264][27]);
        let eq160_e2024_d_b28: f64 = (p.p7 * s.db[264][28]);
        let eq160_e2024_d_b29: f64 = (p.p7 * s.db[264][29]);
        let eq160_e2024_d_b30: f64 = (p.p7 * s.db[264][30]);
        let eq160_e2024_d_b31: f64 = (p.p7 * s.db[264][31]);
        let eq160_e2024_d_b32: f64 = (p.p7 * s.db[264][32]);
        let eq160_e2024_d_b33: f64 = (p.p7 * s.db[264][33]);
        let eq160_e2024_d_b34: f64 = (p.p7 * s.db[264][34]);
        let eq160_e2024_d_b35: f64 = (p.p7 * s.db[264][35]);
        let eq160_e2024_d_b36: f64 = (p.p7 * s.db[264][36]);
        let eq160_e2024_d_b37: f64 = (p.p7 * s.db[264][37]);
        let eq160_e2024_d_b38: f64 = (p.p7 * s.db[264][38]);
        let eq160_e2024_d_b39: f64 = (p.p7 * s.db[264][39]);
        let eq160_e2024_d_b40: f64 = (p.p7 * s.db[264][40]);
        let eq160_e2024_d_b41: f64 = (p.p7 * s.db[264][41]);
        let eq160_e2024_d_b42: f64 = (p.p7 * s.db[264][42]);
        let eq160_e2024_d_b43: f64 = (p.p7 * s.db[264][43]);
        let eq160_e2024_d_b44: f64 = (p.p7 * s.db[264][44]);
        let eq160_e2024_d_b45: f64 = (p.p7 * s.db[264][45]);
        let eq160_e2024_d_b46: f64 = (p.p7 * s.db[264][46]);
        let eq160_e2024_d_b47: f64 = (p.p7 * s.db[264][47]);
        let eq160_e2024_d_b48: f64 = (p.p7 * s.db[264][48]);
        let eq160_e2024_d_b49: f64 = (p.p7 * s.db[264][49]);
        let eq160_e2024_d_b50: f64 = (p.p7 * s.db[264][50]);
        let eq160_e2024_d_b51: f64 = (p.p7 * s.db[264][51]);
        let eq160_e2024_d_b52: f64 = (p.p7 * s.db[264][52]);
        let eq160_e2024_d_b53: f64 = (p.p7 * s.db[264][53]);
        let eq160_e2024_d_b54: f64 = (p.p7 * s.db[264][54]);
        let eq160_e2024_q: f64 = (p.p7 * eq160_e2023_q);
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
        let eq160_e2026_d_b0: f64 = (eq160_e2024_d_b0 * p.p247);
        let eq160_e2026_d_b1: f64 = (eq160_e2024_d_b1 * p.p247);
        let eq160_e2026_d_b2: f64 = (eq160_e2024_d_b2 * p.p247);
        let eq160_e2026_d_b3: f64 = (eq160_e2024_d_b3 * p.p247);
        let eq160_e2026_d_b4: f64 = (eq160_e2024_d_b4 * p.p247);
        let eq160_e2026_d_b5: f64 = (eq160_e2024_d_b5 * p.p247);
        let eq160_e2026_d_b6: f64 = (eq160_e2024_d_b6 * p.p247);
        let eq160_e2026_d_b7: f64 = (eq160_e2024_d_b7 * p.p247);
        let eq160_e2026_d_b8: f64 = (eq160_e2024_d_b8 * p.p247);
        let eq160_e2026_d_b9: f64 = (eq160_e2024_d_b9 * p.p247);
        let eq160_e2026_d_b10: f64 = (eq160_e2024_d_b10 * p.p247);
        let eq160_e2026_d_b11: f64 = (eq160_e2024_d_b11 * p.p247);
        let eq160_e2026_d_b12: f64 = (eq160_e2024_d_b12 * p.p247);
        let eq160_e2026_d_b13: f64 = (eq160_e2024_d_b13 * p.p247);
        let eq160_e2026_d_b14: f64 = (eq160_e2024_d_b14 * p.p247);
        let eq160_e2026_d_b15: f64 = (eq160_e2024_d_b15 * p.p247);
        let eq160_e2026_d_b16: f64 = (eq160_e2024_d_b16 * p.p247);
        let eq160_e2026_d_b17: f64 = (eq160_e2024_d_b17 * p.p247);
        let eq160_e2026_d_b18: f64 = (eq160_e2024_d_b18 * p.p247);
        let eq160_e2026_d_b19: f64 = (eq160_e2024_d_b19 * p.p247);
        let eq160_e2026_d_b20: f64 = (eq160_e2024_d_b20 * p.p247);
        let eq160_e2026_d_b21: f64 = (eq160_e2024_d_b21 * p.p247);
        let eq160_e2026_d_b22: f64 = (eq160_e2024_d_b22 * p.p247);
        let eq160_e2026_d_b23: f64 = (eq160_e2024_d_b23 * p.p247);
        let eq160_e2026_d_b24: f64 = (eq160_e2024_d_b24 * p.p247);
        let eq160_e2026_d_b25: f64 = (eq160_e2024_d_b25 * p.p247);
        let eq160_e2026_d_b26: f64 = (eq160_e2024_d_b26 * p.p247);
        let eq160_e2026_d_b27: f64 = (eq160_e2024_d_b27 * p.p247);
        let eq160_e2026_d_b28: f64 = (eq160_e2024_d_b28 * p.p247);
        let eq160_e2026_d_b29: f64 = (eq160_e2024_d_b29 * p.p247);
        let eq160_e2026_d_b30: f64 = (eq160_e2024_d_b30 * p.p247);
        let eq160_e2026_d_b31: f64 = (eq160_e2024_d_b31 * p.p247);
        let eq160_e2026_d_b32: f64 = (eq160_e2024_d_b32 * p.p247);
        let eq160_e2026_d_b33: f64 = (eq160_e2024_d_b33 * p.p247);
        let eq160_e2026_d_b34: f64 = (eq160_e2024_d_b34 * p.p247);
        let eq160_e2026_d_b35: f64 = (eq160_e2024_d_b35 * p.p247);
        let eq160_e2026_d_b36: f64 = (eq160_e2024_d_b36 * p.p247);
        let eq160_e2026_d_b37: f64 = (eq160_e2024_d_b37 * p.p247);
        let eq160_e2026_d_b38: f64 = (eq160_e2024_d_b38 * p.p247);
        let eq160_e2026_d_b39: f64 = (eq160_e2024_d_b39 * p.p247);
        let eq160_e2026_d_b40: f64 = (eq160_e2024_d_b40 * p.p247);
        let eq160_e2026_d_b41: f64 = (eq160_e2024_d_b41 * p.p247);
        let eq160_e2026_d_b42: f64 = (eq160_e2024_d_b42 * p.p247);
        let eq160_e2026_d_b43: f64 = (eq160_e2024_d_b43 * p.p247);
        let eq160_e2026_d_b44: f64 = (eq160_e2024_d_b44 * p.p247);
        let eq160_e2026_d_b45: f64 = (eq160_e2024_d_b45 * p.p247);
        let eq160_e2026_d_b46: f64 = (eq160_e2024_d_b46 * p.p247);
        let eq160_e2026_d_b47: f64 = (eq160_e2024_d_b47 * p.p247);
        let eq160_e2026_d_b48: f64 = (eq160_e2024_d_b48 * p.p247);
        let eq160_e2026_d_b49: f64 = (eq160_e2024_d_b49 * p.p247);
        let eq160_e2026_d_b50: f64 = (eq160_e2024_d_b50 * p.p247);
        let eq160_e2026_d_b51: f64 = (eq160_e2024_d_b51 * p.p247);
        let eq160_e2026_d_b52: f64 = (eq160_e2024_d_b52 * p.p247);
        let eq160_e2026_d_b53: f64 = (eq160_e2024_d_b53 * p.p247);
        let eq160_e2026_d_b54: f64 = (eq160_e2024_d_b54 * p.p247);
        let eq160_e2026_q: f64 = (eq160_e2024_q * p.p247);
        (eq160_e2026, eq160_e2026_d_n0, eq160_e2026_d_n1, eq160_e2026_d_n2, eq160_e2026_d_n3, eq160_e2026_d_n4, eq160_e2026_d_n5, eq160_e2026_d_n6, eq160_e2026_d_n7, eq160_e2026_d_n8, eq160_e2026_d_n9, eq160_e2026_d_n10, eq160_e2026_d_n11, eq160_e2026_d_n12, eq160_e2026_d_n13, eq160_e2026_d_n14, eq160_e2026_d_n15, eq160_e2026_d_n16, eq160_e2026_d_n17, eq160_e2026_d_n18, eq160_e2026_d_n19, eq160_e2026_d_n20, eq160_e2026_d_n21, eq160_e2026_d_n22, eq160_e2026_d_b0, eq160_e2026_d_b1, eq160_e2026_d_b2, eq160_e2026_d_b3, eq160_e2026_d_b4, eq160_e2026_d_b5, eq160_e2026_d_b6, eq160_e2026_d_b7, eq160_e2026_d_b8, eq160_e2026_d_b9, eq160_e2026_d_b10, eq160_e2026_d_b11, eq160_e2026_d_b12, eq160_e2026_d_b13, eq160_e2026_d_b14, eq160_e2026_d_b15, eq160_e2026_d_b16, eq160_e2026_d_b17, eq160_e2026_d_b18, eq160_e2026_d_b19, eq160_e2026_d_b20, eq160_e2026_d_b21, eq160_e2026_d_b22, eq160_e2026_d_b23, eq160_e2026_d_b24, eq160_e2026_d_b25, eq160_e2026_d_b26, eq160_e2026_d_b27, eq160_e2026_d_b28, eq160_e2026_d_b29, eq160_e2026_d_b30, eq160_e2026_d_b31, eq160_e2026_d_b32, eq160_e2026_d_b33, eq160_e2026_d_b34, eq160_e2026_d_b35, eq160_e2026_d_b36, eq160_e2026_d_b37, eq160_e2026_d_b38, eq160_e2026_d_b39, eq160_e2026_d_b40, eq160_e2026_d_b41, eq160_e2026_d_b42, eq160_e2026_d_b43, eq160_e2026_d_b44, eq160_e2026_d_b45, eq160_e2026_d_b46, eq160_e2026_d_b47, eq160_e2026_d_b48, eq160_e2026_d_b49, eq160_e2026_d_b50, eq160_e2026_d_b51, eq160_e2026_d_b52, eq160_e2026_d_b53, eq160_e2026_d_b54, eq160_e2026_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_reactive_node_derivatives: [f64; 23] = [eq160_e2028_d_n0, eq160_e2028_d_n1, eq160_e2028_d_n2, eq160_e2028_d_n3, eq160_e2028_d_n4, eq160_e2028_d_n5, eq160_e2028_d_n6, eq160_e2028_d_n7, eq160_e2028_d_n8, eq160_e2028_d_n9, eq160_e2028_d_n10, eq160_e2028_d_n11, eq160_e2028_d_n12, eq160_e2028_d_n13, eq160_e2028_d_n14, eq160_e2028_d_n15, eq160_e2028_d_n16, eq160_e2028_d_n17, eq160_e2028_d_n18, eq160_e2028_d_n19, eq160_e2028_d_n20, eq160_e2028_d_n21, eq160_e2028_d_n22];
        let eq160_reactive_branch_derivatives: [f64; 55] = [eq160_e2028_d_b0, eq160_e2028_d_b1, eq160_e2028_d_b2, eq160_e2028_d_b3, eq160_e2028_d_b4, eq160_e2028_d_b5, eq160_e2028_d_b6, eq160_e2028_d_b7, eq160_e2028_d_b8, eq160_e2028_d_b9, eq160_e2028_d_b10, eq160_e2028_d_b11, eq160_e2028_d_b12, eq160_e2028_d_b13, eq160_e2028_d_b14, eq160_e2028_d_b15, eq160_e2028_d_b16, eq160_e2028_d_b17, eq160_e2028_d_b18, eq160_e2028_d_b19, eq160_e2028_d_b20, eq160_e2028_d_b21, eq160_e2028_d_b22, eq160_e2028_d_b23, eq160_e2028_d_b24, eq160_e2028_d_b25, eq160_e2028_d_b26, eq160_e2028_d_b27, eq160_e2028_d_b28, eq160_e2028_d_b29, eq160_e2028_d_b30, eq160_e2028_d_b31, eq160_e2028_d_b32, eq160_e2028_d_b33, eq160_e2028_d_b34, eq160_e2028_d_b35, eq160_e2028_d_b36, eq160_e2028_d_b37, eq160_e2028_d_b38, eq160_e2028_d_b39, eq160_e2028_d_b40, eq160_e2028_d_b41, eq160_e2028_d_b42, eq160_e2028_d_b43, eq160_e2028_d_b44, eq160_e2028_d_b45, eq160_e2028_d_b46, eq160_e2028_d_b47, eq160_e2028_d_b48, eq160_e2028_d_b49, eq160_e2028_d_b50, eq160_e2028_d_b51, eq160_e2028_d_b52, eq160_e2028_d_b53, eq160_e2028_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[20]),
            nodes,
            &eq160_reactive_node_derivatives,
            branches,
            &eq160_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq161_e2039, eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22, eq161_e2039_d_b0, eq161_e2039_d_b1, eq161_e2039_d_b2, eq161_e2039_d_b3, eq161_e2039_d_b4, eq161_e2039_d_b5, eq161_e2039_d_b6, eq161_e2039_d_b7, eq161_e2039_d_b8, eq161_e2039_d_b9, eq161_e2039_d_b10, eq161_e2039_d_b11, eq161_e2039_d_b12, eq161_e2039_d_b13, eq161_e2039_d_b14, eq161_e2039_d_b15, eq161_e2039_d_b16, eq161_e2039_d_b17, eq161_e2039_d_b18, eq161_e2039_d_b19, eq161_e2039_d_b20, eq161_e2039_d_b21, eq161_e2039_d_b22, eq161_e2039_d_b23, eq161_e2039_d_b24, eq161_e2039_d_b25, eq161_e2039_d_b26, eq161_e2039_d_b27, eq161_e2039_d_b28, eq161_e2039_d_b29, eq161_e2039_d_b30, eq161_e2039_d_b31, eq161_e2039_d_b32, eq161_e2039_d_b33, eq161_e2039_d_b34, eq161_e2039_d_b35, eq161_e2039_d_b36, eq161_e2039_d_b37, eq161_e2039_d_b38, eq161_e2039_d_b39, eq161_e2039_d_b40, eq161_e2039_d_b41, eq161_e2039_d_b42, eq161_e2039_d_b43, eq161_e2039_d_b44, eq161_e2039_d_b45, eq161_e2039_d_b46, eq161_e2039_d_b47, eq161_e2039_d_b48, eq161_e2039_d_b49, eq161_e2039_d_b50, eq161_e2039_d_b51, eq161_e2039_d_b52, eq161_e2039_d_b53, eq161_e2039_d_b54, eq161_e2039_q,) = {
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
        let eq161_e2035_d_b0: f64 = (p.p252 * s.db[264][0]);
        let eq161_e2035_d_b1: f64 = (p.p252 * s.db[264][1]);
        let eq161_e2035_d_b2: f64 = (p.p252 * s.db[264][2]);
        let eq161_e2035_d_b3: f64 = (p.p252 * s.db[264][3]);
        let eq161_e2035_d_b4: f64 = (p.p252 * s.db[264][4]);
        let eq161_e2035_d_b5: f64 = (p.p252 * s.db[264][5]);
        let eq161_e2035_d_b6: f64 = (p.p252 * s.db[264][6]);
        let eq161_e2035_d_b7: f64 = (p.p252 * s.db[264][7]);
        let eq161_e2035_d_b8: f64 = (p.p252 * s.db[264][8]);
        let eq161_e2035_d_b9: f64 = (p.p252 * s.db[264][9]);
        let eq161_e2035_d_b10: f64 = (p.p252 * s.db[264][10]);
        let eq161_e2035_d_b11: f64 = (p.p252 * s.db[264][11]);
        let eq161_e2035_d_b12: f64 = (p.p252 * s.db[264][12]);
        let eq161_e2035_d_b13: f64 = (p.p252 * s.db[264][13]);
        let eq161_e2035_d_b14: f64 = (p.p252 * s.db[264][14]);
        let eq161_e2035_d_b15: f64 = (p.p252 * s.db[264][15]);
        let eq161_e2035_d_b16: f64 = (p.p252 * s.db[264][16]);
        let eq161_e2035_d_b17: f64 = (p.p252 * s.db[264][17]);
        let eq161_e2035_d_b18: f64 = (p.p252 * s.db[264][18]);
        let eq161_e2035_d_b19: f64 = (p.p252 * s.db[264][19]);
        let eq161_e2035_d_b20: f64 = (p.p252 * s.db[264][20]);
        let eq161_e2035_d_b21: f64 = (p.p252 * s.db[264][21]);
        let eq161_e2035_d_b22: f64 = (p.p252 * s.db[264][22]);
        let eq161_e2035_d_b23: f64 = (p.p252 * s.db[264][23]);
        let eq161_e2035_d_b24: f64 = (p.p252 * s.db[264][24]);
        let eq161_e2035_d_b25: f64 = (p.p252 * s.db[264][25]);
        let eq161_e2035_d_b26: f64 = (p.p252 * s.db[264][26]);
        let eq161_e2035_d_b27: f64 = (p.p252 * s.db[264][27]);
        let eq161_e2035_d_b28: f64 = (p.p252 * s.db[264][28]);
        let eq161_e2035_d_b29: f64 = (p.p252 * s.db[264][29]);
        let eq161_e2035_d_b30: f64 = (p.p252 * s.db[264][30]);
        let eq161_e2035_d_b31: f64 = (p.p252 * s.db[264][31]);
        let eq161_e2035_d_b32: f64 = (p.p252 * s.db[264][32]);
        let eq161_e2035_d_b33: f64 = (p.p252 * s.db[264][33]);
        let eq161_e2035_d_b34: f64 = (p.p252 * s.db[264][34]);
        let eq161_e2035_d_b35: f64 = (p.p252 * s.db[264][35]);
        let eq161_e2035_d_b36: f64 = (p.p252 * s.db[264][36]);
        let eq161_e2035_d_b37: f64 = (p.p252 * s.db[264][37]);
        let eq161_e2035_d_b38: f64 = (p.p252 * s.db[264][38]);
        let eq161_e2035_d_b39: f64 = (p.p252 * s.db[264][39]);
        let eq161_e2035_d_b40: f64 = (p.p252 * s.db[264][40]);
        let eq161_e2035_d_b41: f64 = (p.p252 * s.db[264][41]);
        let eq161_e2035_d_b42: f64 = (p.p252 * s.db[264][42]);
        let eq161_e2035_d_b43: f64 = (p.p252 * s.db[264][43]);
        let eq161_e2035_d_b44: f64 = (p.p252 * s.db[264][44]);
        let eq161_e2035_d_b45: f64 = (p.p252 * s.db[264][45]);
        let eq161_e2035_d_b46: f64 = (p.p252 * s.db[264][46]);
        let eq161_e2035_d_b47: f64 = (p.p252 * s.db[264][47]);
        let eq161_e2035_d_b48: f64 = (p.p252 * s.db[264][48]);
        let eq161_e2035_d_b49: f64 = (p.p252 * s.db[264][49]);
        let eq161_e2035_d_b50: f64 = (p.p252 * s.db[264][50]);
        let eq161_e2035_d_b51: f64 = (p.p252 * s.db[264][51]);
        let eq161_e2035_d_b52: f64 = (p.p252 * s.db[264][52]);
        let eq161_e2035_d_b53: f64 = (p.p252 * s.db[264][53]);
        let eq161_e2035_d_b54: f64 = (p.p252 * s.db[264][54]);
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
        let eq161_e2037_d_b0: f64 = (p.p7 * eq161_e2035_d_b0);
        let eq161_e2037_d_b1: f64 = (p.p7 * eq161_e2035_d_b1);
        let eq161_e2037_d_b2: f64 = (p.p7 * eq161_e2035_d_b2);
        let eq161_e2037_d_b3: f64 = (p.p7 * eq161_e2035_d_b3);
        let eq161_e2037_d_b4: f64 = (p.p7 * eq161_e2035_d_b4);
        let eq161_e2037_d_b5: f64 = (p.p7 * eq161_e2035_d_b5);
        let eq161_e2037_d_b6: f64 = (p.p7 * eq161_e2035_d_b6);
        let eq161_e2037_d_b7: f64 = (p.p7 * eq161_e2035_d_b7);
        let eq161_e2037_d_b8: f64 = (p.p7 * eq161_e2035_d_b8);
        let eq161_e2037_d_b9: f64 = (p.p7 * eq161_e2035_d_b9);
        let eq161_e2037_d_b10: f64 = (p.p7 * eq161_e2035_d_b10);
        let eq161_e2037_d_b11: f64 = (p.p7 * eq161_e2035_d_b11);
        let eq161_e2037_d_b12: f64 = (p.p7 * eq161_e2035_d_b12);
        let eq161_e2037_d_b13: f64 = (p.p7 * eq161_e2035_d_b13);
        let eq161_e2037_d_b14: f64 = (p.p7 * eq161_e2035_d_b14);
        let eq161_e2037_d_b15: f64 = (p.p7 * eq161_e2035_d_b15);
        let eq161_e2037_d_b16: f64 = (p.p7 * eq161_e2035_d_b16);
        let eq161_e2037_d_b17: f64 = (p.p7 * eq161_e2035_d_b17);
        let eq161_e2037_d_b18: f64 = (p.p7 * eq161_e2035_d_b18);
        let eq161_e2037_d_b19: f64 = (p.p7 * eq161_e2035_d_b19);
        let eq161_e2037_d_b20: f64 = (p.p7 * eq161_e2035_d_b20);
        let eq161_e2037_d_b21: f64 = (p.p7 * eq161_e2035_d_b21);
        let eq161_e2037_d_b22: f64 = (p.p7 * eq161_e2035_d_b22);
        let eq161_e2037_d_b23: f64 = (p.p7 * eq161_e2035_d_b23);
        let eq161_e2037_d_b24: f64 = (p.p7 * eq161_e2035_d_b24);
        let eq161_e2037_d_b25: f64 = (p.p7 * eq161_e2035_d_b25);
        let eq161_e2037_d_b26: f64 = (p.p7 * eq161_e2035_d_b26);
        let eq161_e2037_d_b27: f64 = (p.p7 * eq161_e2035_d_b27);
        let eq161_e2037_d_b28: f64 = (p.p7 * eq161_e2035_d_b28);
        let eq161_e2037_d_b29: f64 = (p.p7 * eq161_e2035_d_b29);
        let eq161_e2037_d_b30: f64 = (p.p7 * eq161_e2035_d_b30);
        let eq161_e2037_d_b31: f64 = (p.p7 * eq161_e2035_d_b31);
        let eq161_e2037_d_b32: f64 = (p.p7 * eq161_e2035_d_b32);
        let eq161_e2037_d_b33: f64 = (p.p7 * eq161_e2035_d_b33);
        let eq161_e2037_d_b34: f64 = (p.p7 * eq161_e2035_d_b34);
        let eq161_e2037_d_b35: f64 = (p.p7 * eq161_e2035_d_b35);
        let eq161_e2037_d_b36: f64 = (p.p7 * eq161_e2035_d_b36);
        let eq161_e2037_d_b37: f64 = (p.p7 * eq161_e2035_d_b37);
        let eq161_e2037_d_b38: f64 = (p.p7 * eq161_e2035_d_b38);
        let eq161_e2037_d_b39: f64 = (p.p7 * eq161_e2035_d_b39);
        let eq161_e2037_d_b40: f64 = (p.p7 * eq161_e2035_d_b40);
        let eq161_e2037_d_b41: f64 = (p.p7 * eq161_e2035_d_b41);
        let eq161_e2037_d_b42: f64 = (p.p7 * eq161_e2035_d_b42);
        let eq161_e2037_d_b43: f64 = (p.p7 * eq161_e2035_d_b43);
        let eq161_e2037_d_b44: f64 = (p.p7 * eq161_e2035_d_b44);
        let eq161_e2037_d_b45: f64 = (p.p7 * eq161_e2035_d_b45);
        let eq161_e2037_d_b46: f64 = (p.p7 * eq161_e2035_d_b46);
        let eq161_e2037_d_b47: f64 = (p.p7 * eq161_e2035_d_b47);
        let eq161_e2037_d_b48: f64 = (p.p7 * eq161_e2035_d_b48);
        let eq161_e2037_d_b49: f64 = (p.p7 * eq161_e2035_d_b49);
        let eq161_e2037_d_b50: f64 = (p.p7 * eq161_e2035_d_b50);
        let eq161_e2037_d_b51: f64 = (p.p7 * eq161_e2035_d_b51);
        let eq161_e2037_d_b52: f64 = (p.p7 * eq161_e2035_d_b52);
        let eq161_e2037_d_b53: f64 = (p.p7 * eq161_e2035_d_b53);
        let eq161_e2037_d_b54: f64 = (p.p7 * eq161_e2035_d_b54);
        let eq161_e2037_q: f64 = (p.p7 * eq161_e2036_q);
        (eq161_e2037, eq161_e2037_d_n0, eq161_e2037_d_n1, eq161_e2037_d_n2, eq161_e2037_d_n3, eq161_e2037_d_n4, eq161_e2037_d_n5, eq161_e2037_d_n6, eq161_e2037_d_n7, eq161_e2037_d_n8, eq161_e2037_d_n9, eq161_e2037_d_n10, eq161_e2037_d_n11, eq161_e2037_d_n12, eq161_e2037_d_n13, eq161_e2037_d_n14, eq161_e2037_d_n15, eq161_e2037_d_n16, eq161_e2037_d_n17, eq161_e2037_d_n18, eq161_e2037_d_n19, eq161_e2037_d_n20, eq161_e2037_d_n21, eq161_e2037_d_n22, eq161_e2037_d_b0, eq161_e2037_d_b1, eq161_e2037_d_b2, eq161_e2037_d_b3, eq161_e2037_d_b4, eq161_e2037_d_b5, eq161_e2037_d_b6, eq161_e2037_d_b7, eq161_e2037_d_b8, eq161_e2037_d_b9, eq161_e2037_d_b10, eq161_e2037_d_b11, eq161_e2037_d_b12, eq161_e2037_d_b13, eq161_e2037_d_b14, eq161_e2037_d_b15, eq161_e2037_d_b16, eq161_e2037_d_b17, eq161_e2037_d_b18, eq161_e2037_d_b19, eq161_e2037_d_b20, eq161_e2037_d_b21, eq161_e2037_d_b22, eq161_e2037_d_b23, eq161_e2037_d_b24, eq161_e2037_d_b25, eq161_e2037_d_b26, eq161_e2037_d_b27, eq161_e2037_d_b28, eq161_e2037_d_b29, eq161_e2037_d_b30, eq161_e2037_d_b31, eq161_e2037_d_b32, eq161_e2037_d_b33, eq161_e2037_d_b34, eq161_e2037_d_b35, eq161_e2037_d_b36, eq161_e2037_d_b37, eq161_e2037_d_b38, eq161_e2037_d_b39, eq161_e2037_d_b40, eq161_e2037_d_b41, eq161_e2037_d_b42, eq161_e2037_d_b43, eq161_e2037_d_b44, eq161_e2037_d_b45, eq161_e2037_d_b46, eq161_e2037_d_b47, eq161_e2037_d_b48, eq161_e2037_d_b49, eq161_e2037_d_b50, eq161_e2037_d_b51, eq161_e2037_d_b52, eq161_e2037_d_b53, eq161_e2037_d_b54, eq161_e2037_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_reactive_node_derivatives: [f64; 23] = [eq161_e2039_d_n0, eq161_e2039_d_n1, eq161_e2039_d_n2, eq161_e2039_d_n3, eq161_e2039_d_n4, eq161_e2039_d_n5, eq161_e2039_d_n6, eq161_e2039_d_n7, eq161_e2039_d_n8, eq161_e2039_d_n9, eq161_e2039_d_n10, eq161_e2039_d_n11, eq161_e2039_d_n12, eq161_e2039_d_n13, eq161_e2039_d_n14, eq161_e2039_d_n15, eq161_e2039_d_n16, eq161_e2039_d_n17, eq161_e2039_d_n18, eq161_e2039_d_n19, eq161_e2039_d_n20, eq161_e2039_d_n21, eq161_e2039_d_n22];
        let eq161_reactive_branch_derivatives: [f64; 55] = [eq161_e2039_d_b0, eq161_e2039_d_b1, eq161_e2039_d_b2, eq161_e2039_d_b3, eq161_e2039_d_b4, eq161_e2039_d_b5, eq161_e2039_d_b6, eq161_e2039_d_b7, eq161_e2039_d_b8, eq161_e2039_d_b9, eq161_e2039_d_b10, eq161_e2039_d_b11, eq161_e2039_d_b12, eq161_e2039_d_b13, eq161_e2039_d_b14, eq161_e2039_d_b15, eq161_e2039_d_b16, eq161_e2039_d_b17, eq161_e2039_d_b18, eq161_e2039_d_b19, eq161_e2039_d_b20, eq161_e2039_d_b21, eq161_e2039_d_b22, eq161_e2039_d_b23, eq161_e2039_d_b24, eq161_e2039_d_b25, eq161_e2039_d_b26, eq161_e2039_d_b27, eq161_e2039_d_b28, eq161_e2039_d_b29, eq161_e2039_d_b30, eq161_e2039_d_b31, eq161_e2039_d_b32, eq161_e2039_d_b33, eq161_e2039_d_b34, eq161_e2039_d_b35, eq161_e2039_d_b36, eq161_e2039_d_b37, eq161_e2039_d_b38, eq161_e2039_d_b39, eq161_e2039_d_b40, eq161_e2039_d_b41, eq161_e2039_d_b42, eq161_e2039_d_b43, eq161_e2039_d_b44, eq161_e2039_d_b45, eq161_e2039_d_b46, eq161_e2039_d_b47, eq161_e2039_d_b48, eq161_e2039_d_b49, eq161_e2039_d_b50, eq161_e2039_d_b51, eq161_e2039_d_b52, eq161_e2039_d_b53, eq161_e2039_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[20]),
            nodes,
            &eq161_reactive_node_derivatives,
            branches,
            &eq161_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq162_e2049, eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22, eq162_e2049_d_b0, eq162_e2049_d_b1, eq162_e2049_d_b2, eq162_e2049_d_b3, eq162_e2049_d_b4, eq162_e2049_d_b5, eq162_e2049_d_b6, eq162_e2049_d_b7, eq162_e2049_d_b8, eq162_e2049_d_b9, eq162_e2049_d_b10, eq162_e2049_d_b11, eq162_e2049_d_b12, eq162_e2049_d_b13, eq162_e2049_d_b14, eq162_e2049_d_b15, eq162_e2049_d_b16, eq162_e2049_d_b17, eq162_e2049_d_b18, eq162_e2049_d_b19, eq162_e2049_d_b20, eq162_e2049_d_b21, eq162_e2049_d_b22, eq162_e2049_d_b23, eq162_e2049_d_b24, eq162_e2049_d_b25, eq162_e2049_d_b26, eq162_e2049_d_b27, eq162_e2049_d_b28, eq162_e2049_d_b29, eq162_e2049_d_b30, eq162_e2049_d_b31, eq162_e2049_d_b32, eq162_e2049_d_b33, eq162_e2049_d_b34, eq162_e2049_d_b35, eq162_e2049_d_b36, eq162_e2049_d_b37, eq162_e2049_d_b38, eq162_e2049_d_b39, eq162_e2049_d_b40, eq162_e2049_d_b41, eq162_e2049_d_b42, eq162_e2049_d_b43, eq162_e2049_d_b44, eq162_e2049_d_b45, eq162_e2049_d_b46, eq162_e2049_d_b47, eq162_e2049_d_b48, eq162_e2049_d_b49, eq162_e2049_d_b50, eq162_e2049_d_b51, eq162_e2049_d_b52, eq162_e2049_d_b53, eq162_e2049_d_b54, eq162_e2049_q,) = {
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
        let eq162_e2047_d_b0: f64 = (p.p7 * s.db[265][0]);
        let eq162_e2047_d_b1: f64 = (p.p7 * s.db[265][1]);
        let eq162_e2047_d_b2: f64 = (p.p7 * s.db[265][2]);
        let eq162_e2047_d_b3: f64 = (p.p7 * s.db[265][3]);
        let eq162_e2047_d_b4: f64 = (p.p7 * s.db[265][4]);
        let eq162_e2047_d_b5: f64 = (p.p7 * s.db[265][5]);
        let eq162_e2047_d_b6: f64 = (p.p7 * s.db[265][6]);
        let eq162_e2047_d_b7: f64 = (p.p7 * s.db[265][7]);
        let eq162_e2047_d_b8: f64 = (p.p7 * s.db[265][8]);
        let eq162_e2047_d_b9: f64 = (p.p7 * s.db[265][9]);
        let eq162_e2047_d_b10: f64 = (p.p7 * s.db[265][10]);
        let eq162_e2047_d_b11: f64 = (p.p7 * s.db[265][11]);
        let eq162_e2047_d_b12: f64 = (p.p7 * s.db[265][12]);
        let eq162_e2047_d_b13: f64 = (p.p7 * s.db[265][13]);
        let eq162_e2047_d_b14: f64 = (p.p7 * s.db[265][14]);
        let eq162_e2047_d_b15: f64 = (p.p7 * s.db[265][15]);
        let eq162_e2047_d_b16: f64 = (p.p7 * s.db[265][16]);
        let eq162_e2047_d_b17: f64 = (p.p7 * s.db[265][17]);
        let eq162_e2047_d_b18: f64 = (p.p7 * s.db[265][18]);
        let eq162_e2047_d_b19: f64 = (p.p7 * s.db[265][19]);
        let eq162_e2047_d_b20: f64 = (p.p7 * s.db[265][20]);
        let eq162_e2047_d_b21: f64 = (p.p7 * s.db[265][21]);
        let eq162_e2047_d_b22: f64 = (p.p7 * s.db[265][22]);
        let eq162_e2047_d_b23: f64 = (p.p7 * s.db[265][23]);
        let eq162_e2047_d_b24: f64 = (p.p7 * s.db[265][24]);
        let eq162_e2047_d_b25: f64 = (p.p7 * s.db[265][25]);
        let eq162_e2047_d_b26: f64 = (p.p7 * s.db[265][26]);
        let eq162_e2047_d_b27: f64 = (p.p7 * s.db[265][27]);
        let eq162_e2047_d_b28: f64 = (p.p7 * s.db[265][28]);
        let eq162_e2047_d_b29: f64 = (p.p7 * s.db[265][29]);
        let eq162_e2047_d_b30: f64 = (p.p7 * s.db[265][30]);
        let eq162_e2047_d_b31: f64 = (p.p7 * s.db[265][31]);
        let eq162_e2047_d_b32: f64 = (p.p7 * s.db[265][32]);
        let eq162_e2047_d_b33: f64 = (p.p7 * s.db[265][33]);
        let eq162_e2047_d_b34: f64 = (p.p7 * s.db[265][34]);
        let eq162_e2047_d_b35: f64 = (p.p7 * s.db[265][35]);
        let eq162_e2047_d_b36: f64 = (p.p7 * s.db[265][36]);
        let eq162_e2047_d_b37: f64 = (p.p7 * s.db[265][37]);
        let eq162_e2047_d_b38: f64 = (p.p7 * s.db[265][38]);
        let eq162_e2047_d_b39: f64 = (p.p7 * s.db[265][39]);
        let eq162_e2047_d_b40: f64 = (p.p7 * s.db[265][40]);
        let eq162_e2047_d_b41: f64 = (p.p7 * s.db[265][41]);
        let eq162_e2047_d_b42: f64 = (p.p7 * s.db[265][42]);
        let eq162_e2047_d_b43: f64 = (p.p7 * s.db[265][43]);
        let eq162_e2047_d_b44: f64 = (p.p7 * s.db[265][44]);
        let eq162_e2047_d_b45: f64 = (p.p7 * s.db[265][45]);
        let eq162_e2047_d_b46: f64 = (p.p7 * s.db[265][46]);
        let eq162_e2047_d_b47: f64 = (p.p7 * s.db[265][47]);
        let eq162_e2047_d_b48: f64 = (p.p7 * s.db[265][48]);
        let eq162_e2047_d_b49: f64 = (p.p7 * s.db[265][49]);
        let eq162_e2047_d_b50: f64 = (p.p7 * s.db[265][50]);
        let eq162_e2047_d_b51: f64 = (p.p7 * s.db[265][51]);
        let eq162_e2047_d_b52: f64 = (p.p7 * s.db[265][52]);
        let eq162_e2047_d_b53: f64 = (p.p7 * s.db[265][53]);
        let eq162_e2047_d_b54: f64 = (p.p7 * s.db[265][54]);
        let eq162_e2047_q: f64 = (p.p7 * eq162_e2046_q);
        (eq162_e2047, eq162_e2047_d_n0, eq162_e2047_d_n1, eq162_e2047_d_n2, eq162_e2047_d_n3, eq162_e2047_d_n4, eq162_e2047_d_n5, eq162_e2047_d_n6, eq162_e2047_d_n7, eq162_e2047_d_n8, eq162_e2047_d_n9, eq162_e2047_d_n10, eq162_e2047_d_n11, eq162_e2047_d_n12, eq162_e2047_d_n13, eq162_e2047_d_n14, eq162_e2047_d_n15, eq162_e2047_d_n16, eq162_e2047_d_n17, eq162_e2047_d_n18, eq162_e2047_d_n19, eq162_e2047_d_n20, eq162_e2047_d_n21, eq162_e2047_d_n22, eq162_e2047_d_b0, eq162_e2047_d_b1, eq162_e2047_d_b2, eq162_e2047_d_b3, eq162_e2047_d_b4, eq162_e2047_d_b5, eq162_e2047_d_b6, eq162_e2047_d_b7, eq162_e2047_d_b8, eq162_e2047_d_b9, eq162_e2047_d_b10, eq162_e2047_d_b11, eq162_e2047_d_b12, eq162_e2047_d_b13, eq162_e2047_d_b14, eq162_e2047_d_b15, eq162_e2047_d_b16, eq162_e2047_d_b17, eq162_e2047_d_b18, eq162_e2047_d_b19, eq162_e2047_d_b20, eq162_e2047_d_b21, eq162_e2047_d_b22, eq162_e2047_d_b23, eq162_e2047_d_b24, eq162_e2047_d_b25, eq162_e2047_d_b26, eq162_e2047_d_b27, eq162_e2047_d_b28, eq162_e2047_d_b29, eq162_e2047_d_b30, eq162_e2047_d_b31, eq162_e2047_d_b32, eq162_e2047_d_b33, eq162_e2047_d_b34, eq162_e2047_d_b35, eq162_e2047_d_b36, eq162_e2047_d_b37, eq162_e2047_d_b38, eq162_e2047_d_b39, eq162_e2047_d_b40, eq162_e2047_d_b41, eq162_e2047_d_b42, eq162_e2047_d_b43, eq162_e2047_d_b44, eq162_e2047_d_b45, eq162_e2047_d_b46, eq162_e2047_d_b47, eq162_e2047_d_b48, eq162_e2047_d_b49, eq162_e2047_d_b50, eq162_e2047_d_b51, eq162_e2047_d_b52, eq162_e2047_d_b53, eq162_e2047_d_b54, eq162_e2047_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_reactive_node_derivatives: [f64; 23] = [eq162_e2049_d_n0, eq162_e2049_d_n1, eq162_e2049_d_n2, eq162_e2049_d_n3, eq162_e2049_d_n4, eq162_e2049_d_n5, eq162_e2049_d_n6, eq162_e2049_d_n7, eq162_e2049_d_n8, eq162_e2049_d_n9, eq162_e2049_d_n10, eq162_e2049_d_n11, eq162_e2049_d_n12, eq162_e2049_d_n13, eq162_e2049_d_n14, eq162_e2049_d_n15, eq162_e2049_d_n16, eq162_e2049_d_n17, eq162_e2049_d_n18, eq162_e2049_d_n19, eq162_e2049_d_n20, eq162_e2049_d_n21, eq162_e2049_d_n22];
        let eq162_reactive_branch_derivatives: [f64; 55] = [eq162_e2049_d_b0, eq162_e2049_d_b1, eq162_e2049_d_b2, eq162_e2049_d_b3, eq162_e2049_d_b4, eq162_e2049_d_b5, eq162_e2049_d_b6, eq162_e2049_d_b7, eq162_e2049_d_b8, eq162_e2049_d_b9, eq162_e2049_d_b10, eq162_e2049_d_b11, eq162_e2049_d_b12, eq162_e2049_d_b13, eq162_e2049_d_b14, eq162_e2049_d_b15, eq162_e2049_d_b16, eq162_e2049_d_b17, eq162_e2049_d_b18, eq162_e2049_d_b19, eq162_e2049_d_b20, eq162_e2049_d_b21, eq162_e2049_d_b22, eq162_e2049_d_b23, eq162_e2049_d_b24, eq162_e2049_d_b25, eq162_e2049_d_b26, eq162_e2049_d_b27, eq162_e2049_d_b28, eq162_e2049_d_b29, eq162_e2049_d_b30, eq162_e2049_d_b31, eq162_e2049_d_b32, eq162_e2049_d_b33, eq162_e2049_d_b34, eq162_e2049_d_b35, eq162_e2049_d_b36, eq162_e2049_d_b37, eq162_e2049_d_b38, eq162_e2049_d_b39, eq162_e2049_d_b40, eq162_e2049_d_b41, eq162_e2049_d_b42, eq162_e2049_d_b43, eq162_e2049_d_b44, eq162_e2049_d_b45, eq162_e2049_d_b46, eq162_e2049_d_b47, eq162_e2049_d_b48, eq162_e2049_d_b49, eq162_e2049_d_b50, eq162_e2049_d_b51, eq162_e2049_d_b52, eq162_e2049_d_b53, eq162_e2049_d_b54];
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

    pub(super) fn stamp_reactive_equations_block_16(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * s.dn[264][0]);
        let __rspice_deriv_cse_1: f64 = (p.p7 * s.dn[264][1]);
        let __rspice_deriv_cse_2: f64 = (p.p7 * s.dn[264][2]);
        let __rspice_deriv_cse_3: f64 = (p.p7 * s.dn[264][3]);
        let __rspice_deriv_cse_4: f64 = (p.p7 * s.dn[264][4]);
        let __rspice_deriv_cse_5: f64 = (p.p7 * s.dn[264][5]);
        let __rspice_deriv_cse_6: f64 = (p.p7 * s.dn[264][6]);
        let __rspice_deriv_cse_7: f64 = (p.p7 * s.dn[264][7]);
        let __rspice_deriv_cse_8: f64 = (p.p7 * s.dn[264][8]);
        let __rspice_deriv_cse_9: f64 = (p.p7 * s.dn[264][9]);
        let __rspice_deriv_cse_10: f64 = (p.p7 * s.dn[264][10]);
        let __rspice_deriv_cse_11: f64 = (p.p7 * s.dn[264][11]);
        let __rspice_deriv_cse_12: f64 = (p.p7 * s.dn[264][12]);
        let __rspice_deriv_cse_13: f64 = (p.p7 * s.dn[264][13]);
        let __rspice_deriv_cse_14: f64 = (p.p7 * s.dn[264][14]);
        let __rspice_deriv_cse_15: f64 = (p.p7 * s.dn[264][15]);
        let __rspice_deriv_cse_16: f64 = (p.p7 * s.dn[264][16]);
        let __rspice_deriv_cse_17: f64 = (p.p7 * s.dn[264][17]);
        let __rspice_deriv_cse_18: f64 = (p.p7 * s.dn[264][18]);
        let __rspice_deriv_cse_19: f64 = (p.p7 * s.dn[264][19]);
        let __rspice_deriv_cse_20: f64 = (p.p7 * s.dn[264][20]);
        let __rspice_deriv_cse_21: f64 = (p.p7 * s.dn[264][21]);
        let __rspice_deriv_cse_22: f64 = (p.p7 * s.dn[264][22]);
        let __rspice_deriv_cse_23: f64 = (p.p7 * s.db[264][0]);
        let __rspice_deriv_cse_24: f64 = (p.p7 * s.db[264][1]);
        let __rspice_deriv_cse_25: f64 = (p.p7 * s.db[264][2]);
        let __rspice_deriv_cse_26: f64 = (p.p7 * s.db[264][3]);
        let __rspice_deriv_cse_27: f64 = (p.p7 * s.db[264][4]);
        let __rspice_deriv_cse_28: f64 = (p.p7 * s.db[264][5]);
        let __rspice_deriv_cse_29: f64 = (p.p7 * s.db[264][6]);
        let __rspice_deriv_cse_30: f64 = (p.p7 * s.db[264][7]);
        let __rspice_deriv_cse_31: f64 = (p.p7 * s.db[264][8]);
        let __rspice_deriv_cse_32: f64 = (p.p7 * s.db[264][9]);
        let __rspice_deriv_cse_33: f64 = (p.p7 * s.db[264][10]);
        let __rspice_deriv_cse_34: f64 = (p.p7 * s.db[264][11]);
        let __rspice_deriv_cse_35: f64 = (p.p7 * s.db[264][12]);
        let __rspice_deriv_cse_36: f64 = (p.p7 * s.db[264][13]);
        let __rspice_deriv_cse_37: f64 = (p.p7 * s.db[264][14]);
        let __rspice_deriv_cse_38: f64 = (p.p7 * s.db[264][15]);
        let __rspice_deriv_cse_39: f64 = (p.p7 * s.db[264][16]);
        let __rspice_deriv_cse_40: f64 = (p.p7 * s.db[264][17]);
        let __rspice_deriv_cse_41: f64 = (p.p7 * s.db[264][18]);
        let __rspice_deriv_cse_42: f64 = (p.p7 * s.db[264][19]);
        let __rspice_deriv_cse_43: f64 = (p.p7 * s.db[264][20]);
        let __rspice_deriv_cse_44: f64 = (p.p7 * s.db[264][21]);
        let __rspice_deriv_cse_45: f64 = (p.p7 * s.db[264][22]);
        let __rspice_deriv_cse_46: f64 = (p.p7 * s.db[264][23]);
        let __rspice_deriv_cse_47: f64 = (p.p7 * s.db[264][24]);
        let __rspice_deriv_cse_48: f64 = (p.p7 * s.db[264][25]);
        let __rspice_deriv_cse_49: f64 = (p.p7 * s.db[264][26]);
        let __rspice_deriv_cse_50: f64 = (p.p7 * s.db[264][27]);
        let __rspice_deriv_cse_51: f64 = (p.p7 * s.db[264][28]);
        let __rspice_deriv_cse_52: f64 = (p.p7 * s.db[264][29]);
        let __rspice_deriv_cse_53: f64 = (p.p7 * s.db[264][30]);
        let __rspice_deriv_cse_54: f64 = (p.p7 * s.db[264][31]);
        let __rspice_deriv_cse_55: f64 = (p.p7 * s.db[264][32]);
        let __rspice_deriv_cse_56: f64 = (p.p7 * s.db[264][33]);
        let __rspice_deriv_cse_57: f64 = (p.p7 * s.db[264][34]);
        let __rspice_deriv_cse_58: f64 = (p.p7 * s.db[264][35]);
        let __rspice_deriv_cse_59: f64 = (p.p7 * s.db[264][36]);
        let __rspice_deriv_cse_60: f64 = (p.p7 * s.db[264][37]);
        let __rspice_deriv_cse_61: f64 = (p.p7 * s.db[264][38]);
        let __rspice_deriv_cse_62: f64 = (p.p7 * s.db[264][39]);
        let __rspice_deriv_cse_63: f64 = (p.p7 * s.db[264][40]);
        let __rspice_deriv_cse_64: f64 = (p.p7 * s.db[264][41]);
        let __rspice_deriv_cse_65: f64 = (p.p7 * s.db[264][42]);
        let __rspice_deriv_cse_66: f64 = (p.p7 * s.db[264][43]);
        let __rspice_deriv_cse_67: f64 = (p.p7 * s.db[264][44]);
        let __rspice_deriv_cse_68: f64 = (p.p7 * s.db[264][45]);
        let __rspice_deriv_cse_69: f64 = (p.p7 * s.db[264][46]);
        let __rspice_deriv_cse_70: f64 = (p.p7 * s.db[264][47]);
        let __rspice_deriv_cse_71: f64 = (p.p7 * s.db[264][48]);
        let __rspice_deriv_cse_72: f64 = (p.p7 * s.db[264][49]);
        let __rspice_deriv_cse_73: f64 = (p.p7 * s.db[264][50]);
        let __rspice_deriv_cse_74: f64 = (p.p7 * s.db[264][51]);
        let __rspice_deriv_cse_75: f64 = (p.p7 * s.db[264][52]);
        let __rspice_deriv_cse_76: f64 = (p.p7 * s.db[264][53]);
        let __rspice_deriv_cse_77: f64 = (p.p7 * s.db[264][54]);
        let (eq163_e2061, eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22, eq163_e2061_d_b0, eq163_e2061_d_b1, eq163_e2061_d_b2, eq163_e2061_d_b3, eq163_e2061_d_b4, eq163_e2061_d_b5, eq163_e2061_d_b6, eq163_e2061_d_b7, eq163_e2061_d_b8, eq163_e2061_d_b9, eq163_e2061_d_b10, eq163_e2061_d_b11, eq163_e2061_d_b12, eq163_e2061_d_b13, eq163_e2061_d_b14, eq163_e2061_d_b15, eq163_e2061_d_b16, eq163_e2061_d_b17, eq163_e2061_d_b18, eq163_e2061_d_b19, eq163_e2061_d_b20, eq163_e2061_d_b21, eq163_e2061_d_b22, eq163_e2061_d_b23, eq163_e2061_d_b24, eq163_e2061_d_b25, eq163_e2061_d_b26, eq163_e2061_d_b27, eq163_e2061_d_b28, eq163_e2061_d_b29, eq163_e2061_d_b30, eq163_e2061_d_b31, eq163_e2061_d_b32, eq163_e2061_d_b33, eq163_e2061_d_b34, eq163_e2061_d_b35, eq163_e2061_d_b36, eq163_e2061_d_b37, eq163_e2061_d_b38, eq163_e2061_d_b39, eq163_e2061_d_b40, eq163_e2061_d_b41, eq163_e2061_d_b42, eq163_e2061_d_b43, eq163_e2061_d_b44, eq163_e2061_d_b45, eq163_e2061_d_b46, eq163_e2061_d_b47, eq163_e2061_d_b48, eq163_e2061_d_b49, eq163_e2061_d_b50, eq163_e2061_d_b51, eq163_e2061_d_b52, eq163_e2061_d_b53, eq163_e2061_d_b54, eq163_e2061_q,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq163_e2058_q: f64 = s.v[264];
        let eq163_e2059: f64 = (p.p7 * s.v[264]);
        let eq163_e2059_q: f64 = (p.p7 * eq163_e2058_q);
        (eq163_e2059, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq163_e2059_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_reactive_node_derivatives: [f64; 23] = [eq163_e2061_d_n0, eq163_e2061_d_n1, eq163_e2061_d_n2, eq163_e2061_d_n3, eq163_e2061_d_n4, eq163_e2061_d_n5, eq163_e2061_d_n6, eq163_e2061_d_n7, eq163_e2061_d_n8, eq163_e2061_d_n9, eq163_e2061_d_n10, eq163_e2061_d_n11, eq163_e2061_d_n12, eq163_e2061_d_n13, eq163_e2061_d_n14, eq163_e2061_d_n15, eq163_e2061_d_n16, eq163_e2061_d_n17, eq163_e2061_d_n18, eq163_e2061_d_n19, eq163_e2061_d_n20, eq163_e2061_d_n21, eq163_e2061_d_n22];
        let eq163_reactive_branch_derivatives: [f64; 55] = [eq163_e2061_d_b0, eq163_e2061_d_b1, eq163_e2061_d_b2, eq163_e2061_d_b3, eq163_e2061_d_b4, eq163_e2061_d_b5, eq163_e2061_d_b6, eq163_e2061_d_b7, eq163_e2061_d_b8, eq163_e2061_d_b9, eq163_e2061_d_b10, eq163_e2061_d_b11, eq163_e2061_d_b12, eq163_e2061_d_b13, eq163_e2061_d_b14, eq163_e2061_d_b15, eq163_e2061_d_b16, eq163_e2061_d_b17, eq163_e2061_d_b18, eq163_e2061_d_b19, eq163_e2061_d_b20, eq163_e2061_d_b21, eq163_e2061_d_b22, eq163_e2061_d_b23, eq163_e2061_d_b24, eq163_e2061_d_b25, eq163_e2061_d_b26, eq163_e2061_d_b27, eq163_e2061_d_b28, eq163_e2061_d_b29, eq163_e2061_d_b30, eq163_e2061_d_b31, eq163_e2061_d_b32, eq163_e2061_d_b33, eq163_e2061_d_b34, eq163_e2061_d_b35, eq163_e2061_d_b36, eq163_e2061_d_b37, eq163_e2061_d_b38, eq163_e2061_d_b39, eq163_e2061_d_b40, eq163_e2061_d_b41, eq163_e2061_d_b42, eq163_e2061_d_b43, eq163_e2061_d_b44, eq163_e2061_d_b45, eq163_e2061_d_b46, eq163_e2061_d_b47, eq163_e2061_d_b48, eq163_e2061_d_b49, eq163_e2061_d_b50, eq163_e2061_d_b51, eq163_e2061_d_b52, eq163_e2061_d_b53, eq163_e2061_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq163_reactive_node_derivatives,
            branches,
            &eq163_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq164_e2075, eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22, eq164_e2075_d_b0, eq164_e2075_d_b1, eq164_e2075_d_b2, eq164_e2075_d_b3, eq164_e2075_d_b4, eq164_e2075_d_b5, eq164_e2075_d_b6, eq164_e2075_d_b7, eq164_e2075_d_b8, eq164_e2075_d_b9, eq164_e2075_d_b10, eq164_e2075_d_b11, eq164_e2075_d_b12, eq164_e2075_d_b13, eq164_e2075_d_b14, eq164_e2075_d_b15, eq164_e2075_d_b16, eq164_e2075_d_b17, eq164_e2075_d_b18, eq164_e2075_d_b19, eq164_e2075_d_b20, eq164_e2075_d_b21, eq164_e2075_d_b22, eq164_e2075_d_b23, eq164_e2075_d_b24, eq164_e2075_d_b25, eq164_e2075_d_b26, eq164_e2075_d_b27, eq164_e2075_d_b28, eq164_e2075_d_b29, eq164_e2075_d_b30, eq164_e2075_d_b31, eq164_e2075_d_b32, eq164_e2075_d_b33, eq164_e2075_d_b34, eq164_e2075_d_b35, eq164_e2075_d_b36, eq164_e2075_d_b37, eq164_e2075_d_b38, eq164_e2075_d_b39, eq164_e2075_d_b40, eq164_e2075_d_b41, eq164_e2075_d_b42, eq164_e2075_d_b43, eq164_e2075_d_b44, eq164_e2075_d_b45, eq164_e2075_d_b46, eq164_e2075_d_b47, eq164_e2075_d_b48, eq164_e2075_d_b49, eq164_e2075_d_b50, eq164_e2075_d_b51, eq164_e2075_d_b52, eq164_e2075_d_b53, eq164_e2075_d_b54, eq164_e2075_q,) = {
    if (((!s.b[585]) && s.b[588]) && s.b[589]) {
        let eq164_e2070_q: f64 = s.v[264];
        let eq164_e2071: f64 = (p.p7 * s.v[264]);
        let eq164_e2071_q: f64 = (p.p7 * eq164_e2070_q);
        let eq164_e2073: f64 = (eq164_e2071 * p.p247);
        let eq164_e2073_d_n0: f64 = (__rspice_deriv_cse_0 * p.p247);
        let eq164_e2073_d_n1: f64 = (__rspice_deriv_cse_1 * p.p247);
        let eq164_e2073_d_n2: f64 = (__rspice_deriv_cse_2 * p.p247);
        let eq164_e2073_d_n3: f64 = (__rspice_deriv_cse_3 * p.p247);
        let eq164_e2073_d_n4: f64 = (__rspice_deriv_cse_4 * p.p247);
        let eq164_e2073_d_n5: f64 = (__rspice_deriv_cse_5 * p.p247);
        let eq164_e2073_d_n6: f64 = (__rspice_deriv_cse_6 * p.p247);
        let eq164_e2073_d_n7: f64 = (__rspice_deriv_cse_7 * p.p247);
        let eq164_e2073_d_n8: f64 = (__rspice_deriv_cse_8 * p.p247);
        let eq164_e2073_d_n9: f64 = (__rspice_deriv_cse_9 * p.p247);
        let eq164_e2073_d_n10: f64 = (__rspice_deriv_cse_10 * p.p247);
        let eq164_e2073_d_n11: f64 = (__rspice_deriv_cse_11 * p.p247);
        let eq164_e2073_d_n12: f64 = (__rspice_deriv_cse_12 * p.p247);
        let eq164_e2073_d_n13: f64 = (__rspice_deriv_cse_13 * p.p247);
        let eq164_e2073_d_n14: f64 = (__rspice_deriv_cse_14 * p.p247);
        let eq164_e2073_d_n15: f64 = (__rspice_deriv_cse_15 * p.p247);
        let eq164_e2073_d_n16: f64 = (__rspice_deriv_cse_16 * p.p247);
        let eq164_e2073_d_n17: f64 = (__rspice_deriv_cse_17 * p.p247);
        let eq164_e2073_d_n18: f64 = (__rspice_deriv_cse_18 * p.p247);
        let eq164_e2073_d_n19: f64 = (__rspice_deriv_cse_19 * p.p247);
        let eq164_e2073_d_n20: f64 = (__rspice_deriv_cse_20 * p.p247);
        let eq164_e2073_d_n21: f64 = (__rspice_deriv_cse_21 * p.p247);
        let eq164_e2073_d_n22: f64 = (__rspice_deriv_cse_22 * p.p247);
        let eq164_e2073_d_b0: f64 = (__rspice_deriv_cse_23 * p.p247);
        let eq164_e2073_d_b1: f64 = (__rspice_deriv_cse_24 * p.p247);
        let eq164_e2073_d_b2: f64 = (__rspice_deriv_cse_25 * p.p247);
        let eq164_e2073_d_b3: f64 = (__rspice_deriv_cse_26 * p.p247);
        let eq164_e2073_d_b4: f64 = (__rspice_deriv_cse_27 * p.p247);
        let eq164_e2073_d_b5: f64 = (__rspice_deriv_cse_28 * p.p247);
        let eq164_e2073_d_b6: f64 = (__rspice_deriv_cse_29 * p.p247);
        let eq164_e2073_d_b7: f64 = (__rspice_deriv_cse_30 * p.p247);
        let eq164_e2073_d_b8: f64 = (__rspice_deriv_cse_31 * p.p247);
        let eq164_e2073_d_b9: f64 = (__rspice_deriv_cse_32 * p.p247);
        let eq164_e2073_d_b10: f64 = (__rspice_deriv_cse_33 * p.p247);
        let eq164_e2073_d_b11: f64 = (__rspice_deriv_cse_34 * p.p247);
        let eq164_e2073_d_b12: f64 = (__rspice_deriv_cse_35 * p.p247);
        let eq164_e2073_d_b13: f64 = (__rspice_deriv_cse_36 * p.p247);
        let eq164_e2073_d_b14: f64 = (__rspice_deriv_cse_37 * p.p247);
        let eq164_e2073_d_b15: f64 = (__rspice_deriv_cse_38 * p.p247);
        let eq164_e2073_d_b16: f64 = (__rspice_deriv_cse_39 * p.p247);
        let eq164_e2073_d_b17: f64 = (__rspice_deriv_cse_40 * p.p247);
        let eq164_e2073_d_b18: f64 = (__rspice_deriv_cse_41 * p.p247);
        let eq164_e2073_d_b19: f64 = (__rspice_deriv_cse_42 * p.p247);
        let eq164_e2073_d_b20: f64 = (__rspice_deriv_cse_43 * p.p247);
        let eq164_e2073_d_b21: f64 = (__rspice_deriv_cse_44 * p.p247);
        let eq164_e2073_d_b22: f64 = (__rspice_deriv_cse_45 * p.p247);
        let eq164_e2073_d_b23: f64 = (__rspice_deriv_cse_46 * p.p247);
        let eq164_e2073_d_b24: f64 = (__rspice_deriv_cse_47 * p.p247);
        let eq164_e2073_d_b25: f64 = (__rspice_deriv_cse_48 * p.p247);
        let eq164_e2073_d_b26: f64 = (__rspice_deriv_cse_49 * p.p247);
        let eq164_e2073_d_b27: f64 = (__rspice_deriv_cse_50 * p.p247);
        let eq164_e2073_d_b28: f64 = (__rspice_deriv_cse_51 * p.p247);
        let eq164_e2073_d_b29: f64 = (__rspice_deriv_cse_52 * p.p247);
        let eq164_e2073_d_b30: f64 = (__rspice_deriv_cse_53 * p.p247);
        let eq164_e2073_d_b31: f64 = (__rspice_deriv_cse_54 * p.p247);
        let eq164_e2073_d_b32: f64 = (__rspice_deriv_cse_55 * p.p247);
        let eq164_e2073_d_b33: f64 = (__rspice_deriv_cse_56 * p.p247);
        let eq164_e2073_d_b34: f64 = (__rspice_deriv_cse_57 * p.p247);
        let eq164_e2073_d_b35: f64 = (__rspice_deriv_cse_58 * p.p247);
        let eq164_e2073_d_b36: f64 = (__rspice_deriv_cse_59 * p.p247);
        let eq164_e2073_d_b37: f64 = (__rspice_deriv_cse_60 * p.p247);
        let eq164_e2073_d_b38: f64 = (__rspice_deriv_cse_61 * p.p247);
        let eq164_e2073_d_b39: f64 = (__rspice_deriv_cse_62 * p.p247);
        let eq164_e2073_d_b40: f64 = (__rspice_deriv_cse_63 * p.p247);
        let eq164_e2073_d_b41: f64 = (__rspice_deriv_cse_64 * p.p247);
        let eq164_e2073_d_b42: f64 = (__rspice_deriv_cse_65 * p.p247);
        let eq164_e2073_d_b43: f64 = (__rspice_deriv_cse_66 * p.p247);
        let eq164_e2073_d_b44: f64 = (__rspice_deriv_cse_67 * p.p247);
        let eq164_e2073_d_b45: f64 = (__rspice_deriv_cse_68 * p.p247);
        let eq164_e2073_d_b46: f64 = (__rspice_deriv_cse_69 * p.p247);
        let eq164_e2073_d_b47: f64 = (__rspice_deriv_cse_70 * p.p247);
        let eq164_e2073_d_b48: f64 = (__rspice_deriv_cse_71 * p.p247);
        let eq164_e2073_d_b49: f64 = (__rspice_deriv_cse_72 * p.p247);
        let eq164_e2073_d_b50: f64 = (__rspice_deriv_cse_73 * p.p247);
        let eq164_e2073_d_b51: f64 = (__rspice_deriv_cse_74 * p.p247);
        let eq164_e2073_d_b52: f64 = (__rspice_deriv_cse_75 * p.p247);
        let eq164_e2073_d_b53: f64 = (__rspice_deriv_cse_76 * p.p247);
        let eq164_e2073_d_b54: f64 = (__rspice_deriv_cse_77 * p.p247);
        let eq164_e2073_q: f64 = (eq164_e2071_q * p.p247);
        (eq164_e2073, eq164_e2073_d_n0, eq164_e2073_d_n1, eq164_e2073_d_n2, eq164_e2073_d_n3, eq164_e2073_d_n4, eq164_e2073_d_n5, eq164_e2073_d_n6, eq164_e2073_d_n7, eq164_e2073_d_n8, eq164_e2073_d_n9, eq164_e2073_d_n10, eq164_e2073_d_n11, eq164_e2073_d_n12, eq164_e2073_d_n13, eq164_e2073_d_n14, eq164_e2073_d_n15, eq164_e2073_d_n16, eq164_e2073_d_n17, eq164_e2073_d_n18, eq164_e2073_d_n19, eq164_e2073_d_n20, eq164_e2073_d_n21, eq164_e2073_d_n22, eq164_e2073_d_b0, eq164_e2073_d_b1, eq164_e2073_d_b2, eq164_e2073_d_b3, eq164_e2073_d_b4, eq164_e2073_d_b5, eq164_e2073_d_b6, eq164_e2073_d_b7, eq164_e2073_d_b8, eq164_e2073_d_b9, eq164_e2073_d_b10, eq164_e2073_d_b11, eq164_e2073_d_b12, eq164_e2073_d_b13, eq164_e2073_d_b14, eq164_e2073_d_b15, eq164_e2073_d_b16, eq164_e2073_d_b17, eq164_e2073_d_b18, eq164_e2073_d_b19, eq164_e2073_d_b20, eq164_e2073_d_b21, eq164_e2073_d_b22, eq164_e2073_d_b23, eq164_e2073_d_b24, eq164_e2073_d_b25, eq164_e2073_d_b26, eq164_e2073_d_b27, eq164_e2073_d_b28, eq164_e2073_d_b29, eq164_e2073_d_b30, eq164_e2073_d_b31, eq164_e2073_d_b32, eq164_e2073_d_b33, eq164_e2073_d_b34, eq164_e2073_d_b35, eq164_e2073_d_b36, eq164_e2073_d_b37, eq164_e2073_d_b38, eq164_e2073_d_b39, eq164_e2073_d_b40, eq164_e2073_d_b41, eq164_e2073_d_b42, eq164_e2073_d_b43, eq164_e2073_d_b44, eq164_e2073_d_b45, eq164_e2073_d_b46, eq164_e2073_d_b47, eq164_e2073_d_b48, eq164_e2073_d_b49, eq164_e2073_d_b50, eq164_e2073_d_b51, eq164_e2073_d_b52, eq164_e2073_d_b53, eq164_e2073_d_b54, eq164_e2073_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_reactive_node_derivatives: [f64; 23] = [eq164_e2075_d_n0, eq164_e2075_d_n1, eq164_e2075_d_n2, eq164_e2075_d_n3, eq164_e2075_d_n4, eq164_e2075_d_n5, eq164_e2075_d_n6, eq164_e2075_d_n7, eq164_e2075_d_n8, eq164_e2075_d_n9, eq164_e2075_d_n10, eq164_e2075_d_n11, eq164_e2075_d_n12, eq164_e2075_d_n13, eq164_e2075_d_n14, eq164_e2075_d_n15, eq164_e2075_d_n16, eq164_e2075_d_n17, eq164_e2075_d_n18, eq164_e2075_d_n19, eq164_e2075_d_n20, eq164_e2075_d_n21, eq164_e2075_d_n22];
        let eq164_reactive_branch_derivatives: [f64; 55] = [eq164_e2075_d_b0, eq164_e2075_d_b1, eq164_e2075_d_b2, eq164_e2075_d_b3, eq164_e2075_d_b4, eq164_e2075_d_b5, eq164_e2075_d_b6, eq164_e2075_d_b7, eq164_e2075_d_b8, eq164_e2075_d_b9, eq164_e2075_d_b10, eq164_e2075_d_b11, eq164_e2075_d_b12, eq164_e2075_d_b13, eq164_e2075_d_b14, eq164_e2075_d_b15, eq164_e2075_d_b16, eq164_e2075_d_b17, eq164_e2075_d_b18, eq164_e2075_d_b19, eq164_e2075_d_b20, eq164_e2075_d_b21, eq164_e2075_d_b22, eq164_e2075_d_b23, eq164_e2075_d_b24, eq164_e2075_d_b25, eq164_e2075_d_b26, eq164_e2075_d_b27, eq164_e2075_d_b28, eq164_e2075_d_b29, eq164_e2075_d_b30, eq164_e2075_d_b31, eq164_e2075_d_b32, eq164_e2075_d_b33, eq164_e2075_d_b34, eq164_e2075_d_b35, eq164_e2075_d_b36, eq164_e2075_d_b37, eq164_e2075_d_b38, eq164_e2075_d_b39, eq164_e2075_d_b40, eq164_e2075_d_b41, eq164_e2075_d_b42, eq164_e2075_d_b43, eq164_e2075_d_b44, eq164_e2075_d_b45, eq164_e2075_d_b46, eq164_e2075_d_b47, eq164_e2075_d_b48, eq164_e2075_d_b49, eq164_e2075_d_b50, eq164_e2075_d_b51, eq164_e2075_d_b52, eq164_e2075_d_b53, eq164_e2075_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq164_reactive_node_derivatives,
            branches,
            &eq164_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq165_e2088, eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22, eq165_e2088_d_b0, eq165_e2088_d_b1, eq165_e2088_d_b2, eq165_e2088_d_b3, eq165_e2088_d_b4, eq165_e2088_d_b5, eq165_e2088_d_b6, eq165_e2088_d_b7, eq165_e2088_d_b8, eq165_e2088_d_b9, eq165_e2088_d_b10, eq165_e2088_d_b11, eq165_e2088_d_b12, eq165_e2088_d_b13, eq165_e2088_d_b14, eq165_e2088_d_b15, eq165_e2088_d_b16, eq165_e2088_d_b17, eq165_e2088_d_b18, eq165_e2088_d_b19, eq165_e2088_d_b20, eq165_e2088_d_b21, eq165_e2088_d_b22, eq165_e2088_d_b23, eq165_e2088_d_b24, eq165_e2088_d_b25, eq165_e2088_d_b26, eq165_e2088_d_b27, eq165_e2088_d_b28, eq165_e2088_d_b29, eq165_e2088_d_b30, eq165_e2088_d_b31, eq165_e2088_d_b32, eq165_e2088_d_b33, eq165_e2088_d_b34, eq165_e2088_d_b35, eq165_e2088_d_b36, eq165_e2088_d_b37, eq165_e2088_d_b38, eq165_e2088_d_b39, eq165_e2088_d_b40, eq165_e2088_d_b41, eq165_e2088_d_b42, eq165_e2088_d_b43, eq165_e2088_d_b44, eq165_e2088_d_b45, eq165_e2088_d_b46, eq165_e2088_d_b47, eq165_e2088_d_b48, eq165_e2088_d_b49, eq165_e2088_d_b50, eq165_e2088_d_b51, eq165_e2088_d_b52, eq165_e2088_d_b53, eq165_e2088_d_b54, eq165_e2088_q,) = {
    if (((!s.b[585]) && s.b[588]) && (!s.b[589])) {
        let eq165_e2085_q: f64 = s.v[264];
        let eq165_e2086: f64 = (p.p7 * s.v[264]);
        let eq165_e2086_q: f64 = (p.p7 * eq165_e2085_q);
        (eq165_e2086, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77, eq165_e2086_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq165_reactive_node_derivatives: [f64; 23] = [eq165_e2088_d_n0, eq165_e2088_d_n1, eq165_e2088_d_n2, eq165_e2088_d_n3, eq165_e2088_d_n4, eq165_e2088_d_n5, eq165_e2088_d_n6, eq165_e2088_d_n7, eq165_e2088_d_n8, eq165_e2088_d_n9, eq165_e2088_d_n10, eq165_e2088_d_n11, eq165_e2088_d_n12, eq165_e2088_d_n13, eq165_e2088_d_n14, eq165_e2088_d_n15, eq165_e2088_d_n16, eq165_e2088_d_n17, eq165_e2088_d_n18, eq165_e2088_d_n19, eq165_e2088_d_n20, eq165_e2088_d_n21, eq165_e2088_d_n22];
        let eq165_reactive_branch_derivatives: [f64; 55] = [eq165_e2088_d_b0, eq165_e2088_d_b1, eq165_e2088_d_b2, eq165_e2088_d_b3, eq165_e2088_d_b4, eq165_e2088_d_b5, eq165_e2088_d_b6, eq165_e2088_d_b7, eq165_e2088_d_b8, eq165_e2088_d_b9, eq165_e2088_d_b10, eq165_e2088_d_b11, eq165_e2088_d_b12, eq165_e2088_d_b13, eq165_e2088_d_b14, eq165_e2088_d_b15, eq165_e2088_d_b16, eq165_e2088_d_b17, eq165_e2088_d_b18, eq165_e2088_d_b19, eq165_e2088_d_b20, eq165_e2088_d_b21, eq165_e2088_d_b22, eq165_e2088_d_b23, eq165_e2088_d_b24, eq165_e2088_d_b25, eq165_e2088_d_b26, eq165_e2088_d_b27, eq165_e2088_d_b28, eq165_e2088_d_b29, eq165_e2088_d_b30, eq165_e2088_d_b31, eq165_e2088_d_b32, eq165_e2088_d_b33, eq165_e2088_d_b34, eq165_e2088_d_b35, eq165_e2088_d_b36, eq165_e2088_d_b37, eq165_e2088_d_b38, eq165_e2088_d_b39, eq165_e2088_d_b40, eq165_e2088_d_b41, eq165_e2088_d_b42, eq165_e2088_d_b43, eq165_e2088_d_b44, eq165_e2088_d_b45, eq165_e2088_d_b46, eq165_e2088_d_b47, eq165_e2088_d_b48, eq165_e2088_d_b49, eq165_e2088_d_b50, eq165_e2088_d_b51, eq165_e2088_d_b52, eq165_e2088_d_b53, eq165_e2088_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            nodes,
            &eq165_reactive_node_derivatives,
            branches,
            &eq165_reactive_branch_derivatives,
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
        let (eq166_e2103, eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22, eq166_e2103_d_b0, eq166_e2103_d_b1, eq166_e2103_d_b2, eq166_e2103_d_b3, eq166_e2103_d_b4, eq166_e2103_d_b5, eq166_e2103_d_b6, eq166_e2103_d_b7, eq166_e2103_d_b8, eq166_e2103_d_b9, eq166_e2103_d_b10, eq166_e2103_d_b11, eq166_e2103_d_b12, eq166_e2103_d_b13, eq166_e2103_d_b14, eq166_e2103_d_b15, eq166_e2103_d_b16, eq166_e2103_d_b17, eq166_e2103_d_b18, eq166_e2103_d_b19, eq166_e2103_d_b20, eq166_e2103_d_b21, eq166_e2103_d_b22, eq166_e2103_d_b23, eq166_e2103_d_b24, eq166_e2103_d_b25, eq166_e2103_d_b26, eq166_e2103_d_b27, eq166_e2103_d_b28, eq166_e2103_d_b29, eq166_e2103_d_b30, eq166_e2103_d_b31, eq166_e2103_d_b32, eq166_e2103_d_b33, eq166_e2103_d_b34, eq166_e2103_d_b35, eq166_e2103_d_b36, eq166_e2103_d_b37, eq166_e2103_d_b38, eq166_e2103_d_b39, eq166_e2103_d_b40, eq166_e2103_d_b41, eq166_e2103_d_b42, eq166_e2103_d_b43, eq166_e2103_d_b44, eq166_e2103_d_b45, eq166_e2103_d_b46, eq166_e2103_d_b47, eq166_e2103_d_b48, eq166_e2103_d_b49, eq166_e2103_d_b50, eq166_e2103_d_b51, eq166_e2103_d_b52, eq166_e2103_d_b53, eq166_e2103_d_b54, eq166_e2103_q,) = {
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
        let eq166_e2099_d_b0: f64 = (p.p7 * s.db[264][0]);
        let eq166_e2099_d_b1: f64 = (p.p7 * s.db[264][1]);
        let eq166_e2099_d_b2: f64 = (p.p7 * s.db[264][2]);
        let eq166_e2099_d_b3: f64 = (p.p7 * s.db[264][3]);
        let eq166_e2099_d_b4: f64 = (p.p7 * s.db[264][4]);
        let eq166_e2099_d_b5: f64 = (p.p7 * s.db[264][5]);
        let eq166_e2099_d_b6: f64 = (p.p7 * s.db[264][6]);
        let eq166_e2099_d_b7: f64 = (p.p7 * s.db[264][7]);
        let eq166_e2099_d_b8: f64 = (p.p7 * s.db[264][8]);
        let eq166_e2099_d_b9: f64 = (p.p7 * s.db[264][9]);
        let eq166_e2099_d_b10: f64 = (p.p7 * s.db[264][10]);
        let eq166_e2099_d_b11: f64 = (p.p7 * s.db[264][11]);
        let eq166_e2099_d_b12: f64 = (p.p7 * s.db[264][12]);
        let eq166_e2099_d_b13: f64 = (p.p7 * s.db[264][13]);
        let eq166_e2099_d_b14: f64 = (p.p7 * s.db[264][14]);
        let eq166_e2099_d_b15: f64 = (p.p7 * s.db[264][15]);
        let eq166_e2099_d_b16: f64 = (p.p7 * s.db[264][16]);
        let eq166_e2099_d_b17: f64 = (p.p7 * s.db[264][17]);
        let eq166_e2099_d_b18: f64 = (p.p7 * s.db[264][18]);
        let eq166_e2099_d_b19: f64 = (p.p7 * s.db[264][19]);
        let eq166_e2099_d_b20: f64 = (p.p7 * s.db[264][20]);
        let eq166_e2099_d_b21: f64 = (p.p7 * s.db[264][21]);
        let eq166_e2099_d_b22: f64 = (p.p7 * s.db[264][22]);
        let eq166_e2099_d_b23: f64 = (p.p7 * s.db[264][23]);
        let eq166_e2099_d_b24: f64 = (p.p7 * s.db[264][24]);
        let eq166_e2099_d_b25: f64 = (p.p7 * s.db[264][25]);
        let eq166_e2099_d_b26: f64 = (p.p7 * s.db[264][26]);
        let eq166_e2099_d_b27: f64 = (p.p7 * s.db[264][27]);
        let eq166_e2099_d_b28: f64 = (p.p7 * s.db[264][28]);
        let eq166_e2099_d_b29: f64 = (p.p7 * s.db[264][29]);
        let eq166_e2099_d_b30: f64 = (p.p7 * s.db[264][30]);
        let eq166_e2099_d_b31: f64 = (p.p7 * s.db[264][31]);
        let eq166_e2099_d_b32: f64 = (p.p7 * s.db[264][32]);
        let eq166_e2099_d_b33: f64 = (p.p7 * s.db[264][33]);
        let eq166_e2099_d_b34: f64 = (p.p7 * s.db[264][34]);
        let eq166_e2099_d_b35: f64 = (p.p7 * s.db[264][35]);
        let eq166_e2099_d_b36: f64 = (p.p7 * s.db[264][36]);
        let eq166_e2099_d_b37: f64 = (p.p7 * s.db[264][37]);
        let eq166_e2099_d_b38: f64 = (p.p7 * s.db[264][38]);
        let eq166_e2099_d_b39: f64 = (p.p7 * s.db[264][39]);
        let eq166_e2099_d_b40: f64 = (p.p7 * s.db[264][40]);
        let eq166_e2099_d_b41: f64 = (p.p7 * s.db[264][41]);
        let eq166_e2099_d_b42: f64 = (p.p7 * s.db[264][42]);
        let eq166_e2099_d_b43: f64 = (p.p7 * s.db[264][43]);
        let eq166_e2099_d_b44: f64 = (p.p7 * s.db[264][44]);
        let eq166_e2099_d_b45: f64 = (p.p7 * s.db[264][45]);
        let eq166_e2099_d_b46: f64 = (p.p7 * s.db[264][46]);
        let eq166_e2099_d_b47: f64 = (p.p7 * s.db[264][47]);
        let eq166_e2099_d_b48: f64 = (p.p7 * s.db[264][48]);
        let eq166_e2099_d_b49: f64 = (p.p7 * s.db[264][49]);
        let eq166_e2099_d_b50: f64 = (p.p7 * s.db[264][50]);
        let eq166_e2099_d_b51: f64 = (p.p7 * s.db[264][51]);
        let eq166_e2099_d_b52: f64 = (p.p7 * s.db[264][52]);
        let eq166_e2099_d_b53: f64 = (p.p7 * s.db[264][53]);
        let eq166_e2099_d_b54: f64 = (p.p7 * s.db[264][54]);
        let eq166_e2099_q: f64 = (p.p7 * eq166_e2098_q);
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
        let eq166_e2101_d_b0: f64 = (eq166_e2099_d_b0 * p.p247);
        let eq166_e2101_d_b1: f64 = (eq166_e2099_d_b1 * p.p247);
        let eq166_e2101_d_b2: f64 = (eq166_e2099_d_b2 * p.p247);
        let eq166_e2101_d_b3: f64 = (eq166_e2099_d_b3 * p.p247);
        let eq166_e2101_d_b4: f64 = (eq166_e2099_d_b4 * p.p247);
        let eq166_e2101_d_b5: f64 = (eq166_e2099_d_b5 * p.p247);
        let eq166_e2101_d_b6: f64 = (eq166_e2099_d_b6 * p.p247);
        let eq166_e2101_d_b7: f64 = (eq166_e2099_d_b7 * p.p247);
        let eq166_e2101_d_b8: f64 = (eq166_e2099_d_b8 * p.p247);
        let eq166_e2101_d_b9: f64 = (eq166_e2099_d_b9 * p.p247);
        let eq166_e2101_d_b10: f64 = (eq166_e2099_d_b10 * p.p247);
        let eq166_e2101_d_b11: f64 = (eq166_e2099_d_b11 * p.p247);
        let eq166_e2101_d_b12: f64 = (eq166_e2099_d_b12 * p.p247);
        let eq166_e2101_d_b13: f64 = (eq166_e2099_d_b13 * p.p247);
        let eq166_e2101_d_b14: f64 = (eq166_e2099_d_b14 * p.p247);
        let eq166_e2101_d_b15: f64 = (eq166_e2099_d_b15 * p.p247);
        let eq166_e2101_d_b16: f64 = (eq166_e2099_d_b16 * p.p247);
        let eq166_e2101_d_b17: f64 = (eq166_e2099_d_b17 * p.p247);
        let eq166_e2101_d_b18: f64 = (eq166_e2099_d_b18 * p.p247);
        let eq166_e2101_d_b19: f64 = (eq166_e2099_d_b19 * p.p247);
        let eq166_e2101_d_b20: f64 = (eq166_e2099_d_b20 * p.p247);
        let eq166_e2101_d_b21: f64 = (eq166_e2099_d_b21 * p.p247);
        let eq166_e2101_d_b22: f64 = (eq166_e2099_d_b22 * p.p247);
        let eq166_e2101_d_b23: f64 = (eq166_e2099_d_b23 * p.p247);
        let eq166_e2101_d_b24: f64 = (eq166_e2099_d_b24 * p.p247);
        let eq166_e2101_d_b25: f64 = (eq166_e2099_d_b25 * p.p247);
        let eq166_e2101_d_b26: f64 = (eq166_e2099_d_b26 * p.p247);
        let eq166_e2101_d_b27: f64 = (eq166_e2099_d_b27 * p.p247);
        let eq166_e2101_d_b28: f64 = (eq166_e2099_d_b28 * p.p247);
        let eq166_e2101_d_b29: f64 = (eq166_e2099_d_b29 * p.p247);
        let eq166_e2101_d_b30: f64 = (eq166_e2099_d_b30 * p.p247);
        let eq166_e2101_d_b31: f64 = (eq166_e2099_d_b31 * p.p247);
        let eq166_e2101_d_b32: f64 = (eq166_e2099_d_b32 * p.p247);
        let eq166_e2101_d_b33: f64 = (eq166_e2099_d_b33 * p.p247);
        let eq166_e2101_d_b34: f64 = (eq166_e2099_d_b34 * p.p247);
        let eq166_e2101_d_b35: f64 = (eq166_e2099_d_b35 * p.p247);
        let eq166_e2101_d_b36: f64 = (eq166_e2099_d_b36 * p.p247);
        let eq166_e2101_d_b37: f64 = (eq166_e2099_d_b37 * p.p247);
        let eq166_e2101_d_b38: f64 = (eq166_e2099_d_b38 * p.p247);
        let eq166_e2101_d_b39: f64 = (eq166_e2099_d_b39 * p.p247);
        let eq166_e2101_d_b40: f64 = (eq166_e2099_d_b40 * p.p247);
        let eq166_e2101_d_b41: f64 = (eq166_e2099_d_b41 * p.p247);
        let eq166_e2101_d_b42: f64 = (eq166_e2099_d_b42 * p.p247);
        let eq166_e2101_d_b43: f64 = (eq166_e2099_d_b43 * p.p247);
        let eq166_e2101_d_b44: f64 = (eq166_e2099_d_b44 * p.p247);
        let eq166_e2101_d_b45: f64 = (eq166_e2099_d_b45 * p.p247);
        let eq166_e2101_d_b46: f64 = (eq166_e2099_d_b46 * p.p247);
        let eq166_e2101_d_b47: f64 = (eq166_e2099_d_b47 * p.p247);
        let eq166_e2101_d_b48: f64 = (eq166_e2099_d_b48 * p.p247);
        let eq166_e2101_d_b49: f64 = (eq166_e2099_d_b49 * p.p247);
        let eq166_e2101_d_b50: f64 = (eq166_e2099_d_b50 * p.p247);
        let eq166_e2101_d_b51: f64 = (eq166_e2099_d_b51 * p.p247);
        let eq166_e2101_d_b52: f64 = (eq166_e2099_d_b52 * p.p247);
        let eq166_e2101_d_b53: f64 = (eq166_e2099_d_b53 * p.p247);
        let eq166_e2101_d_b54: f64 = (eq166_e2099_d_b54 * p.p247);
        let eq166_e2101_q: f64 = (eq166_e2099_q * p.p247);
        (eq166_e2101, eq166_e2101_d_n0, eq166_e2101_d_n1, eq166_e2101_d_n2, eq166_e2101_d_n3, eq166_e2101_d_n4, eq166_e2101_d_n5, eq166_e2101_d_n6, eq166_e2101_d_n7, eq166_e2101_d_n8, eq166_e2101_d_n9, eq166_e2101_d_n10, eq166_e2101_d_n11, eq166_e2101_d_n12, eq166_e2101_d_n13, eq166_e2101_d_n14, eq166_e2101_d_n15, eq166_e2101_d_n16, eq166_e2101_d_n17, eq166_e2101_d_n18, eq166_e2101_d_n19, eq166_e2101_d_n20, eq166_e2101_d_n21, eq166_e2101_d_n22, eq166_e2101_d_b0, eq166_e2101_d_b1, eq166_e2101_d_b2, eq166_e2101_d_b3, eq166_e2101_d_b4, eq166_e2101_d_b5, eq166_e2101_d_b6, eq166_e2101_d_b7, eq166_e2101_d_b8, eq166_e2101_d_b9, eq166_e2101_d_b10, eq166_e2101_d_b11, eq166_e2101_d_b12, eq166_e2101_d_b13, eq166_e2101_d_b14, eq166_e2101_d_b15, eq166_e2101_d_b16, eq166_e2101_d_b17, eq166_e2101_d_b18, eq166_e2101_d_b19, eq166_e2101_d_b20, eq166_e2101_d_b21, eq166_e2101_d_b22, eq166_e2101_d_b23, eq166_e2101_d_b24, eq166_e2101_d_b25, eq166_e2101_d_b26, eq166_e2101_d_b27, eq166_e2101_d_b28, eq166_e2101_d_b29, eq166_e2101_d_b30, eq166_e2101_d_b31, eq166_e2101_d_b32, eq166_e2101_d_b33, eq166_e2101_d_b34, eq166_e2101_d_b35, eq166_e2101_d_b36, eq166_e2101_d_b37, eq166_e2101_d_b38, eq166_e2101_d_b39, eq166_e2101_d_b40, eq166_e2101_d_b41, eq166_e2101_d_b42, eq166_e2101_d_b43, eq166_e2101_d_b44, eq166_e2101_d_b45, eq166_e2101_d_b46, eq166_e2101_d_b47, eq166_e2101_d_b48, eq166_e2101_d_b49, eq166_e2101_d_b50, eq166_e2101_d_b51, eq166_e2101_d_b52, eq166_e2101_d_b53, eq166_e2101_d_b54, eq166_e2101_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_reactive_node_derivatives: [f64; 23] = [eq166_e2103_d_n0, eq166_e2103_d_n1, eq166_e2103_d_n2, eq166_e2103_d_n3, eq166_e2103_d_n4, eq166_e2103_d_n5, eq166_e2103_d_n6, eq166_e2103_d_n7, eq166_e2103_d_n8, eq166_e2103_d_n9, eq166_e2103_d_n10, eq166_e2103_d_n11, eq166_e2103_d_n12, eq166_e2103_d_n13, eq166_e2103_d_n14, eq166_e2103_d_n15, eq166_e2103_d_n16, eq166_e2103_d_n17, eq166_e2103_d_n18, eq166_e2103_d_n19, eq166_e2103_d_n20, eq166_e2103_d_n21, eq166_e2103_d_n22];
        let eq166_reactive_branch_derivatives: [f64; 55] = [eq166_e2103_d_b0, eq166_e2103_d_b1, eq166_e2103_d_b2, eq166_e2103_d_b3, eq166_e2103_d_b4, eq166_e2103_d_b5, eq166_e2103_d_b6, eq166_e2103_d_b7, eq166_e2103_d_b8, eq166_e2103_d_b9, eq166_e2103_d_b10, eq166_e2103_d_b11, eq166_e2103_d_b12, eq166_e2103_d_b13, eq166_e2103_d_b14, eq166_e2103_d_b15, eq166_e2103_d_b16, eq166_e2103_d_b17, eq166_e2103_d_b18, eq166_e2103_d_b19, eq166_e2103_d_b20, eq166_e2103_d_b21, eq166_e2103_d_b22, eq166_e2103_d_b23, eq166_e2103_d_b24, eq166_e2103_d_b25, eq166_e2103_d_b26, eq166_e2103_d_b27, eq166_e2103_d_b28, eq166_e2103_d_b29, eq166_e2103_d_b30, eq166_e2103_d_b31, eq166_e2103_d_b32, eq166_e2103_d_b33, eq166_e2103_d_b34, eq166_e2103_d_b35, eq166_e2103_d_b36, eq166_e2103_d_b37, eq166_e2103_d_b38, eq166_e2103_d_b39, eq166_e2103_d_b40, eq166_e2103_d_b41, eq166_e2103_d_b42, eq166_e2103_d_b43, eq166_e2103_d_b44, eq166_e2103_d_b45, eq166_e2103_d_b46, eq166_e2103_d_b47, eq166_e2103_d_b48, eq166_e2103_d_b49, eq166_e2103_d_b50, eq166_e2103_d_b51, eq166_e2103_d_b52, eq166_e2103_d_b53, eq166_e2103_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq166_reactive_node_derivatives,
            branches,
            &eq166_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq167_e2115, eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22, eq167_e2115_d_b0, eq167_e2115_d_b1, eq167_e2115_d_b2, eq167_e2115_d_b3, eq167_e2115_d_b4, eq167_e2115_d_b5, eq167_e2115_d_b6, eq167_e2115_d_b7, eq167_e2115_d_b8, eq167_e2115_d_b9, eq167_e2115_d_b10, eq167_e2115_d_b11, eq167_e2115_d_b12, eq167_e2115_d_b13, eq167_e2115_d_b14, eq167_e2115_d_b15, eq167_e2115_d_b16, eq167_e2115_d_b17, eq167_e2115_d_b18, eq167_e2115_d_b19, eq167_e2115_d_b20, eq167_e2115_d_b21, eq167_e2115_d_b22, eq167_e2115_d_b23, eq167_e2115_d_b24, eq167_e2115_d_b25, eq167_e2115_d_b26, eq167_e2115_d_b27, eq167_e2115_d_b28, eq167_e2115_d_b29, eq167_e2115_d_b30, eq167_e2115_d_b31, eq167_e2115_d_b32, eq167_e2115_d_b33, eq167_e2115_d_b34, eq167_e2115_d_b35, eq167_e2115_d_b36, eq167_e2115_d_b37, eq167_e2115_d_b38, eq167_e2115_d_b39, eq167_e2115_d_b40, eq167_e2115_d_b41, eq167_e2115_d_b42, eq167_e2115_d_b43, eq167_e2115_d_b44, eq167_e2115_d_b45, eq167_e2115_d_b46, eq167_e2115_d_b47, eq167_e2115_d_b48, eq167_e2115_d_b49, eq167_e2115_d_b50, eq167_e2115_d_b51, eq167_e2115_d_b52, eq167_e2115_d_b53, eq167_e2115_d_b54, eq167_e2115_q,) = {
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
        let eq167_e2111_d_b0: f64 = (p.p252 * s.db[264][0]);
        let eq167_e2111_d_b1: f64 = (p.p252 * s.db[264][1]);
        let eq167_e2111_d_b2: f64 = (p.p252 * s.db[264][2]);
        let eq167_e2111_d_b3: f64 = (p.p252 * s.db[264][3]);
        let eq167_e2111_d_b4: f64 = (p.p252 * s.db[264][4]);
        let eq167_e2111_d_b5: f64 = (p.p252 * s.db[264][5]);
        let eq167_e2111_d_b6: f64 = (p.p252 * s.db[264][6]);
        let eq167_e2111_d_b7: f64 = (p.p252 * s.db[264][7]);
        let eq167_e2111_d_b8: f64 = (p.p252 * s.db[264][8]);
        let eq167_e2111_d_b9: f64 = (p.p252 * s.db[264][9]);
        let eq167_e2111_d_b10: f64 = (p.p252 * s.db[264][10]);
        let eq167_e2111_d_b11: f64 = (p.p252 * s.db[264][11]);
        let eq167_e2111_d_b12: f64 = (p.p252 * s.db[264][12]);
        let eq167_e2111_d_b13: f64 = (p.p252 * s.db[264][13]);
        let eq167_e2111_d_b14: f64 = (p.p252 * s.db[264][14]);
        let eq167_e2111_d_b15: f64 = (p.p252 * s.db[264][15]);
        let eq167_e2111_d_b16: f64 = (p.p252 * s.db[264][16]);
        let eq167_e2111_d_b17: f64 = (p.p252 * s.db[264][17]);
        let eq167_e2111_d_b18: f64 = (p.p252 * s.db[264][18]);
        let eq167_e2111_d_b19: f64 = (p.p252 * s.db[264][19]);
        let eq167_e2111_d_b20: f64 = (p.p252 * s.db[264][20]);
        let eq167_e2111_d_b21: f64 = (p.p252 * s.db[264][21]);
        let eq167_e2111_d_b22: f64 = (p.p252 * s.db[264][22]);
        let eq167_e2111_d_b23: f64 = (p.p252 * s.db[264][23]);
        let eq167_e2111_d_b24: f64 = (p.p252 * s.db[264][24]);
        let eq167_e2111_d_b25: f64 = (p.p252 * s.db[264][25]);
        let eq167_e2111_d_b26: f64 = (p.p252 * s.db[264][26]);
        let eq167_e2111_d_b27: f64 = (p.p252 * s.db[264][27]);
        let eq167_e2111_d_b28: f64 = (p.p252 * s.db[264][28]);
        let eq167_e2111_d_b29: f64 = (p.p252 * s.db[264][29]);
        let eq167_e2111_d_b30: f64 = (p.p252 * s.db[264][30]);
        let eq167_e2111_d_b31: f64 = (p.p252 * s.db[264][31]);
        let eq167_e2111_d_b32: f64 = (p.p252 * s.db[264][32]);
        let eq167_e2111_d_b33: f64 = (p.p252 * s.db[264][33]);
        let eq167_e2111_d_b34: f64 = (p.p252 * s.db[264][34]);
        let eq167_e2111_d_b35: f64 = (p.p252 * s.db[264][35]);
        let eq167_e2111_d_b36: f64 = (p.p252 * s.db[264][36]);
        let eq167_e2111_d_b37: f64 = (p.p252 * s.db[264][37]);
        let eq167_e2111_d_b38: f64 = (p.p252 * s.db[264][38]);
        let eq167_e2111_d_b39: f64 = (p.p252 * s.db[264][39]);
        let eq167_e2111_d_b40: f64 = (p.p252 * s.db[264][40]);
        let eq167_e2111_d_b41: f64 = (p.p252 * s.db[264][41]);
        let eq167_e2111_d_b42: f64 = (p.p252 * s.db[264][42]);
        let eq167_e2111_d_b43: f64 = (p.p252 * s.db[264][43]);
        let eq167_e2111_d_b44: f64 = (p.p252 * s.db[264][44]);
        let eq167_e2111_d_b45: f64 = (p.p252 * s.db[264][45]);
        let eq167_e2111_d_b46: f64 = (p.p252 * s.db[264][46]);
        let eq167_e2111_d_b47: f64 = (p.p252 * s.db[264][47]);
        let eq167_e2111_d_b48: f64 = (p.p252 * s.db[264][48]);
        let eq167_e2111_d_b49: f64 = (p.p252 * s.db[264][49]);
        let eq167_e2111_d_b50: f64 = (p.p252 * s.db[264][50]);
        let eq167_e2111_d_b51: f64 = (p.p252 * s.db[264][51]);
        let eq167_e2111_d_b52: f64 = (p.p252 * s.db[264][52]);
        let eq167_e2111_d_b53: f64 = (p.p252 * s.db[264][53]);
        let eq167_e2111_d_b54: f64 = (p.p252 * s.db[264][54]);
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
        let eq167_e2113_d_b0: f64 = (p.p7 * eq167_e2111_d_b0);
        let eq167_e2113_d_b1: f64 = (p.p7 * eq167_e2111_d_b1);
        let eq167_e2113_d_b2: f64 = (p.p7 * eq167_e2111_d_b2);
        let eq167_e2113_d_b3: f64 = (p.p7 * eq167_e2111_d_b3);
        let eq167_e2113_d_b4: f64 = (p.p7 * eq167_e2111_d_b4);
        let eq167_e2113_d_b5: f64 = (p.p7 * eq167_e2111_d_b5);
        let eq167_e2113_d_b6: f64 = (p.p7 * eq167_e2111_d_b6);
        let eq167_e2113_d_b7: f64 = (p.p7 * eq167_e2111_d_b7);
        let eq167_e2113_d_b8: f64 = (p.p7 * eq167_e2111_d_b8);
        let eq167_e2113_d_b9: f64 = (p.p7 * eq167_e2111_d_b9);
        let eq167_e2113_d_b10: f64 = (p.p7 * eq167_e2111_d_b10);
        let eq167_e2113_d_b11: f64 = (p.p7 * eq167_e2111_d_b11);
        let eq167_e2113_d_b12: f64 = (p.p7 * eq167_e2111_d_b12);
        let eq167_e2113_d_b13: f64 = (p.p7 * eq167_e2111_d_b13);
        let eq167_e2113_d_b14: f64 = (p.p7 * eq167_e2111_d_b14);
        let eq167_e2113_d_b15: f64 = (p.p7 * eq167_e2111_d_b15);
        let eq167_e2113_d_b16: f64 = (p.p7 * eq167_e2111_d_b16);
        let eq167_e2113_d_b17: f64 = (p.p7 * eq167_e2111_d_b17);
        let eq167_e2113_d_b18: f64 = (p.p7 * eq167_e2111_d_b18);
        let eq167_e2113_d_b19: f64 = (p.p7 * eq167_e2111_d_b19);
        let eq167_e2113_d_b20: f64 = (p.p7 * eq167_e2111_d_b20);
        let eq167_e2113_d_b21: f64 = (p.p7 * eq167_e2111_d_b21);
        let eq167_e2113_d_b22: f64 = (p.p7 * eq167_e2111_d_b22);
        let eq167_e2113_d_b23: f64 = (p.p7 * eq167_e2111_d_b23);
        let eq167_e2113_d_b24: f64 = (p.p7 * eq167_e2111_d_b24);
        let eq167_e2113_d_b25: f64 = (p.p7 * eq167_e2111_d_b25);
        let eq167_e2113_d_b26: f64 = (p.p7 * eq167_e2111_d_b26);
        let eq167_e2113_d_b27: f64 = (p.p7 * eq167_e2111_d_b27);
        let eq167_e2113_d_b28: f64 = (p.p7 * eq167_e2111_d_b28);
        let eq167_e2113_d_b29: f64 = (p.p7 * eq167_e2111_d_b29);
        let eq167_e2113_d_b30: f64 = (p.p7 * eq167_e2111_d_b30);
        let eq167_e2113_d_b31: f64 = (p.p7 * eq167_e2111_d_b31);
        let eq167_e2113_d_b32: f64 = (p.p7 * eq167_e2111_d_b32);
        let eq167_e2113_d_b33: f64 = (p.p7 * eq167_e2111_d_b33);
        let eq167_e2113_d_b34: f64 = (p.p7 * eq167_e2111_d_b34);
        let eq167_e2113_d_b35: f64 = (p.p7 * eq167_e2111_d_b35);
        let eq167_e2113_d_b36: f64 = (p.p7 * eq167_e2111_d_b36);
        let eq167_e2113_d_b37: f64 = (p.p7 * eq167_e2111_d_b37);
        let eq167_e2113_d_b38: f64 = (p.p7 * eq167_e2111_d_b38);
        let eq167_e2113_d_b39: f64 = (p.p7 * eq167_e2111_d_b39);
        let eq167_e2113_d_b40: f64 = (p.p7 * eq167_e2111_d_b40);
        let eq167_e2113_d_b41: f64 = (p.p7 * eq167_e2111_d_b41);
        let eq167_e2113_d_b42: f64 = (p.p7 * eq167_e2111_d_b42);
        let eq167_e2113_d_b43: f64 = (p.p7 * eq167_e2111_d_b43);
        let eq167_e2113_d_b44: f64 = (p.p7 * eq167_e2111_d_b44);
        let eq167_e2113_d_b45: f64 = (p.p7 * eq167_e2111_d_b45);
        let eq167_e2113_d_b46: f64 = (p.p7 * eq167_e2111_d_b46);
        let eq167_e2113_d_b47: f64 = (p.p7 * eq167_e2111_d_b47);
        let eq167_e2113_d_b48: f64 = (p.p7 * eq167_e2111_d_b48);
        let eq167_e2113_d_b49: f64 = (p.p7 * eq167_e2111_d_b49);
        let eq167_e2113_d_b50: f64 = (p.p7 * eq167_e2111_d_b50);
        let eq167_e2113_d_b51: f64 = (p.p7 * eq167_e2111_d_b51);
        let eq167_e2113_d_b52: f64 = (p.p7 * eq167_e2111_d_b52);
        let eq167_e2113_d_b53: f64 = (p.p7 * eq167_e2111_d_b53);
        let eq167_e2113_d_b54: f64 = (p.p7 * eq167_e2111_d_b54);
        let eq167_e2113_q: f64 = (p.p7 * eq167_e2112_q);
        (eq167_e2113, eq167_e2113_d_n0, eq167_e2113_d_n1, eq167_e2113_d_n2, eq167_e2113_d_n3, eq167_e2113_d_n4, eq167_e2113_d_n5, eq167_e2113_d_n6, eq167_e2113_d_n7, eq167_e2113_d_n8, eq167_e2113_d_n9, eq167_e2113_d_n10, eq167_e2113_d_n11, eq167_e2113_d_n12, eq167_e2113_d_n13, eq167_e2113_d_n14, eq167_e2113_d_n15, eq167_e2113_d_n16, eq167_e2113_d_n17, eq167_e2113_d_n18, eq167_e2113_d_n19, eq167_e2113_d_n20, eq167_e2113_d_n21, eq167_e2113_d_n22, eq167_e2113_d_b0, eq167_e2113_d_b1, eq167_e2113_d_b2, eq167_e2113_d_b3, eq167_e2113_d_b4, eq167_e2113_d_b5, eq167_e2113_d_b6, eq167_e2113_d_b7, eq167_e2113_d_b8, eq167_e2113_d_b9, eq167_e2113_d_b10, eq167_e2113_d_b11, eq167_e2113_d_b12, eq167_e2113_d_b13, eq167_e2113_d_b14, eq167_e2113_d_b15, eq167_e2113_d_b16, eq167_e2113_d_b17, eq167_e2113_d_b18, eq167_e2113_d_b19, eq167_e2113_d_b20, eq167_e2113_d_b21, eq167_e2113_d_b22, eq167_e2113_d_b23, eq167_e2113_d_b24, eq167_e2113_d_b25, eq167_e2113_d_b26, eq167_e2113_d_b27, eq167_e2113_d_b28, eq167_e2113_d_b29, eq167_e2113_d_b30, eq167_e2113_d_b31, eq167_e2113_d_b32, eq167_e2113_d_b33, eq167_e2113_d_b34, eq167_e2113_d_b35, eq167_e2113_d_b36, eq167_e2113_d_b37, eq167_e2113_d_b38, eq167_e2113_d_b39, eq167_e2113_d_b40, eq167_e2113_d_b41, eq167_e2113_d_b42, eq167_e2113_d_b43, eq167_e2113_d_b44, eq167_e2113_d_b45, eq167_e2113_d_b46, eq167_e2113_d_b47, eq167_e2113_d_b48, eq167_e2113_d_b49, eq167_e2113_d_b50, eq167_e2113_d_b51, eq167_e2113_d_b52, eq167_e2113_d_b53, eq167_e2113_d_b54, eq167_e2113_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq167_reactive_node_derivatives: [f64; 23] = [eq167_e2115_d_n0, eq167_e2115_d_n1, eq167_e2115_d_n2, eq167_e2115_d_n3, eq167_e2115_d_n4, eq167_e2115_d_n5, eq167_e2115_d_n6, eq167_e2115_d_n7, eq167_e2115_d_n8, eq167_e2115_d_n9, eq167_e2115_d_n10, eq167_e2115_d_n11, eq167_e2115_d_n12, eq167_e2115_d_n13, eq167_e2115_d_n14, eq167_e2115_d_n15, eq167_e2115_d_n16, eq167_e2115_d_n17, eq167_e2115_d_n18, eq167_e2115_d_n19, eq167_e2115_d_n20, eq167_e2115_d_n21, eq167_e2115_d_n22];
        let eq167_reactive_branch_derivatives: [f64; 55] = [eq167_e2115_d_b0, eq167_e2115_d_b1, eq167_e2115_d_b2, eq167_e2115_d_b3, eq167_e2115_d_b4, eq167_e2115_d_b5, eq167_e2115_d_b6, eq167_e2115_d_b7, eq167_e2115_d_b8, eq167_e2115_d_b9, eq167_e2115_d_b10, eq167_e2115_d_b11, eq167_e2115_d_b12, eq167_e2115_d_b13, eq167_e2115_d_b14, eq167_e2115_d_b15, eq167_e2115_d_b16, eq167_e2115_d_b17, eq167_e2115_d_b18, eq167_e2115_d_b19, eq167_e2115_d_b20, eq167_e2115_d_b21, eq167_e2115_d_b22, eq167_e2115_d_b23, eq167_e2115_d_b24, eq167_e2115_d_b25, eq167_e2115_d_b26, eq167_e2115_d_b27, eq167_e2115_d_b28, eq167_e2115_d_b29, eq167_e2115_d_b30, eq167_e2115_d_b31, eq167_e2115_d_b32, eq167_e2115_d_b33, eq167_e2115_d_b34, eq167_e2115_d_b35, eq167_e2115_d_b36, eq167_e2115_d_b37, eq167_e2115_d_b38, eq167_e2115_d_b39, eq167_e2115_d_b40, eq167_e2115_d_b41, eq167_e2115_d_b42, eq167_e2115_d_b43, eq167_e2115_d_b44, eq167_e2115_d_b45, eq167_e2115_d_b46, eq167_e2115_d_b47, eq167_e2115_d_b48, eq167_e2115_d_b49, eq167_e2115_d_b50, eq167_e2115_d_b51, eq167_e2115_d_b52, eq167_e2115_d_b53, eq167_e2115_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq167_reactive_node_derivatives,
            branches,
            &eq167_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq168_e2124, eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22, eq168_e2124_d_b0, eq168_e2124_d_b1, eq168_e2124_d_b2, eq168_e2124_d_b3, eq168_e2124_d_b4, eq168_e2124_d_b5, eq168_e2124_d_b6, eq168_e2124_d_b7, eq168_e2124_d_b8, eq168_e2124_d_b9, eq168_e2124_d_b10, eq168_e2124_d_b11, eq168_e2124_d_b12, eq168_e2124_d_b13, eq168_e2124_d_b14, eq168_e2124_d_b15, eq168_e2124_d_b16, eq168_e2124_d_b17, eq168_e2124_d_b18, eq168_e2124_d_b19, eq168_e2124_d_b20, eq168_e2124_d_b21, eq168_e2124_d_b22, eq168_e2124_d_b23, eq168_e2124_d_b24, eq168_e2124_d_b25, eq168_e2124_d_b26, eq168_e2124_d_b27, eq168_e2124_d_b28, eq168_e2124_d_b29, eq168_e2124_d_b30, eq168_e2124_d_b31, eq168_e2124_d_b32, eq168_e2124_d_b33, eq168_e2124_d_b34, eq168_e2124_d_b35, eq168_e2124_d_b36, eq168_e2124_d_b37, eq168_e2124_d_b38, eq168_e2124_d_b39, eq168_e2124_d_b40, eq168_e2124_d_b41, eq168_e2124_d_b42, eq168_e2124_d_b43, eq168_e2124_d_b44, eq168_e2124_d_b45, eq168_e2124_d_b46, eq168_e2124_d_b47, eq168_e2124_d_b48, eq168_e2124_d_b49, eq168_e2124_d_b50, eq168_e2124_d_b51, eq168_e2124_d_b52, eq168_e2124_d_b53, eq168_e2124_d_b54, eq168_e2124_q,) = {
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
        let eq168_e2122_d_b0: f64 = (p.p7 * s.db[277][0]);
        let eq168_e2122_d_b1: f64 = (p.p7 * s.db[277][1]);
        let eq168_e2122_d_b2: f64 = (p.p7 * s.db[277][2]);
        let eq168_e2122_d_b3: f64 = (p.p7 * s.db[277][3]);
        let eq168_e2122_d_b4: f64 = (p.p7 * s.db[277][4]);
        let eq168_e2122_d_b5: f64 = (p.p7 * s.db[277][5]);
        let eq168_e2122_d_b6: f64 = (p.p7 * s.db[277][6]);
        let eq168_e2122_d_b7: f64 = (p.p7 * s.db[277][7]);
        let eq168_e2122_d_b8: f64 = (p.p7 * s.db[277][8]);
        let eq168_e2122_d_b9: f64 = (p.p7 * s.db[277][9]);
        let eq168_e2122_d_b10: f64 = (p.p7 * s.db[277][10]);
        let eq168_e2122_d_b11: f64 = (p.p7 * s.db[277][11]);
        let eq168_e2122_d_b12: f64 = (p.p7 * s.db[277][12]);
        let eq168_e2122_d_b13: f64 = (p.p7 * s.db[277][13]);
        let eq168_e2122_d_b14: f64 = (p.p7 * s.db[277][14]);
        let eq168_e2122_d_b15: f64 = (p.p7 * s.db[277][15]);
        let eq168_e2122_d_b16: f64 = (p.p7 * s.db[277][16]);
        let eq168_e2122_d_b17: f64 = (p.p7 * s.db[277][17]);
        let eq168_e2122_d_b18: f64 = (p.p7 * s.db[277][18]);
        let eq168_e2122_d_b19: f64 = (p.p7 * s.db[277][19]);
        let eq168_e2122_d_b20: f64 = (p.p7 * s.db[277][20]);
        let eq168_e2122_d_b21: f64 = (p.p7 * s.db[277][21]);
        let eq168_e2122_d_b22: f64 = (p.p7 * s.db[277][22]);
        let eq168_e2122_d_b23: f64 = (p.p7 * s.db[277][23]);
        let eq168_e2122_d_b24: f64 = (p.p7 * s.db[277][24]);
        let eq168_e2122_d_b25: f64 = (p.p7 * s.db[277][25]);
        let eq168_e2122_d_b26: f64 = (p.p7 * s.db[277][26]);
        let eq168_e2122_d_b27: f64 = (p.p7 * s.db[277][27]);
        let eq168_e2122_d_b28: f64 = (p.p7 * s.db[277][28]);
        let eq168_e2122_d_b29: f64 = (p.p7 * s.db[277][29]);
        let eq168_e2122_d_b30: f64 = (p.p7 * s.db[277][30]);
        let eq168_e2122_d_b31: f64 = (p.p7 * s.db[277][31]);
        let eq168_e2122_d_b32: f64 = (p.p7 * s.db[277][32]);
        let eq168_e2122_d_b33: f64 = (p.p7 * s.db[277][33]);
        let eq168_e2122_d_b34: f64 = (p.p7 * s.db[277][34]);
        let eq168_e2122_d_b35: f64 = (p.p7 * s.db[277][35]);
        let eq168_e2122_d_b36: f64 = (p.p7 * s.db[277][36]);
        let eq168_e2122_d_b37: f64 = (p.p7 * s.db[277][37]);
        let eq168_e2122_d_b38: f64 = (p.p7 * s.db[277][38]);
        let eq168_e2122_d_b39: f64 = (p.p7 * s.db[277][39]);
        let eq168_e2122_d_b40: f64 = (p.p7 * s.db[277][40]);
        let eq168_e2122_d_b41: f64 = (p.p7 * s.db[277][41]);
        let eq168_e2122_d_b42: f64 = (p.p7 * s.db[277][42]);
        let eq168_e2122_d_b43: f64 = (p.p7 * s.db[277][43]);
        let eq168_e2122_d_b44: f64 = (p.p7 * s.db[277][44]);
        let eq168_e2122_d_b45: f64 = (p.p7 * s.db[277][45]);
        let eq168_e2122_d_b46: f64 = (p.p7 * s.db[277][46]);
        let eq168_e2122_d_b47: f64 = (p.p7 * s.db[277][47]);
        let eq168_e2122_d_b48: f64 = (p.p7 * s.db[277][48]);
        let eq168_e2122_d_b49: f64 = (p.p7 * s.db[277][49]);
        let eq168_e2122_d_b50: f64 = (p.p7 * s.db[277][50]);
        let eq168_e2122_d_b51: f64 = (p.p7 * s.db[277][51]);
        let eq168_e2122_d_b52: f64 = (p.p7 * s.db[277][52]);
        let eq168_e2122_d_b53: f64 = (p.p7 * s.db[277][53]);
        let eq168_e2122_d_b54: f64 = (p.p7 * s.db[277][54]);
        let eq168_e2122_q: f64 = (p.p7 * eq168_e2121_q);
        (eq168_e2122, eq168_e2122_d_n0, eq168_e2122_d_n1, eq168_e2122_d_n2, eq168_e2122_d_n3, eq168_e2122_d_n4, eq168_e2122_d_n5, eq168_e2122_d_n6, eq168_e2122_d_n7, eq168_e2122_d_n8, eq168_e2122_d_n9, eq168_e2122_d_n10, eq168_e2122_d_n11, eq168_e2122_d_n12, eq168_e2122_d_n13, eq168_e2122_d_n14, eq168_e2122_d_n15, eq168_e2122_d_n16, eq168_e2122_d_n17, eq168_e2122_d_n18, eq168_e2122_d_n19, eq168_e2122_d_n20, eq168_e2122_d_n21, eq168_e2122_d_n22, eq168_e2122_d_b0, eq168_e2122_d_b1, eq168_e2122_d_b2, eq168_e2122_d_b3, eq168_e2122_d_b4, eq168_e2122_d_b5, eq168_e2122_d_b6, eq168_e2122_d_b7, eq168_e2122_d_b8, eq168_e2122_d_b9, eq168_e2122_d_b10, eq168_e2122_d_b11, eq168_e2122_d_b12, eq168_e2122_d_b13, eq168_e2122_d_b14, eq168_e2122_d_b15, eq168_e2122_d_b16, eq168_e2122_d_b17, eq168_e2122_d_b18, eq168_e2122_d_b19, eq168_e2122_d_b20, eq168_e2122_d_b21, eq168_e2122_d_b22, eq168_e2122_d_b23, eq168_e2122_d_b24, eq168_e2122_d_b25, eq168_e2122_d_b26, eq168_e2122_d_b27, eq168_e2122_d_b28, eq168_e2122_d_b29, eq168_e2122_d_b30, eq168_e2122_d_b31, eq168_e2122_d_b32, eq168_e2122_d_b33, eq168_e2122_d_b34, eq168_e2122_d_b35, eq168_e2122_d_b36, eq168_e2122_d_b37, eq168_e2122_d_b38, eq168_e2122_d_b39, eq168_e2122_d_b40, eq168_e2122_d_b41, eq168_e2122_d_b42, eq168_e2122_d_b43, eq168_e2122_d_b44, eq168_e2122_d_b45, eq168_e2122_d_b46, eq168_e2122_d_b47, eq168_e2122_d_b48, eq168_e2122_d_b49, eq168_e2122_d_b50, eq168_e2122_d_b51, eq168_e2122_d_b52, eq168_e2122_d_b53, eq168_e2122_d_b54, eq168_e2122_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq168_reactive_node_derivatives: [f64; 23] = [eq168_e2124_d_n0, eq168_e2124_d_n1, eq168_e2124_d_n2, eq168_e2124_d_n3, eq168_e2124_d_n4, eq168_e2124_d_n5, eq168_e2124_d_n6, eq168_e2124_d_n7, eq168_e2124_d_n8, eq168_e2124_d_n9, eq168_e2124_d_n10, eq168_e2124_d_n11, eq168_e2124_d_n12, eq168_e2124_d_n13, eq168_e2124_d_n14, eq168_e2124_d_n15, eq168_e2124_d_n16, eq168_e2124_d_n17, eq168_e2124_d_n18, eq168_e2124_d_n19, eq168_e2124_d_n20, eq168_e2124_d_n21, eq168_e2124_d_n22];
        let eq168_reactive_branch_derivatives: [f64; 55] = [eq168_e2124_d_b0, eq168_e2124_d_b1, eq168_e2124_d_b2, eq168_e2124_d_b3, eq168_e2124_d_b4, eq168_e2124_d_b5, eq168_e2124_d_b6, eq168_e2124_d_b7, eq168_e2124_d_b8, eq168_e2124_d_b9, eq168_e2124_d_b10, eq168_e2124_d_b11, eq168_e2124_d_b12, eq168_e2124_d_b13, eq168_e2124_d_b14, eq168_e2124_d_b15, eq168_e2124_d_b16, eq168_e2124_d_b17, eq168_e2124_d_b18, eq168_e2124_d_b19, eq168_e2124_d_b20, eq168_e2124_d_b21, eq168_e2124_d_b22, eq168_e2124_d_b23, eq168_e2124_d_b24, eq168_e2124_d_b25, eq168_e2124_d_b26, eq168_e2124_d_b27, eq168_e2124_d_b28, eq168_e2124_d_b29, eq168_e2124_d_b30, eq168_e2124_d_b31, eq168_e2124_d_b32, eq168_e2124_d_b33, eq168_e2124_d_b34, eq168_e2124_d_b35, eq168_e2124_d_b36, eq168_e2124_d_b37, eq168_e2124_d_b38, eq168_e2124_d_b39, eq168_e2124_d_b40, eq168_e2124_d_b41, eq168_e2124_d_b42, eq168_e2124_d_b43, eq168_e2124_d_b44, eq168_e2124_d_b45, eq168_e2124_d_b46, eq168_e2124_d_b47, eq168_e2124_d_b48, eq168_e2124_d_b49, eq168_e2124_d_b50, eq168_e2124_d_b51, eq168_e2124_d_b52, eq168_e2124_d_b53, eq168_e2124_d_b54];
        stamper.stamp_current_reactive_dense(
            Some(nodes[17]),
            Some(nodes[16]),
            nodes,
            &eq168_reactive_node_derivatives,
            branches,
            &eq168_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
