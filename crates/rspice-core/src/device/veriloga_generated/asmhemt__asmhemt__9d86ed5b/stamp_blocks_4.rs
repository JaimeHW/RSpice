#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq79_e1230, eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22, eq79_e1230_d_b0, eq79_e1230_d_b1, eq79_e1230_d_b2, eq79_e1230_d_b3, eq79_e1230_d_b4, eq79_e1230_d_b5, eq79_e1230_d_b6, eq79_e1230_d_b7, eq79_e1230_d_b8, eq79_e1230_d_b9, eq79_e1230_d_b10, eq79_e1230_d_b11, eq79_e1230_d_b12, eq79_e1230_d_b13, eq79_e1230_d_b14, eq79_e1230_d_b15, eq79_e1230_d_b16, eq79_e1230_d_b17, eq79_e1230_d_b18, eq79_e1230_d_b19, eq79_e1230_d_b20, eq79_e1230_d_b21, eq79_e1230_d_b22, eq79_e1230_d_b23, eq79_e1230_d_b24, eq79_e1230_d_b25, eq79_e1230_d_b26, eq79_e1230_d_b27, eq79_e1230_d_b28, eq79_e1230_d_b29, eq79_e1230_d_b30, eq79_e1230_d_b31, eq79_e1230_d_b32, eq79_e1230_d_b33, eq79_e1230_d_b34, eq79_e1230_d_b35, eq79_e1230_d_b36, eq79_e1230_d_b37, eq79_e1230_d_b38, eq79_e1230_d_b39, eq79_e1230_d_b40, eq79_e1230_d_b41, eq79_e1230_d_b42, eq79_e1230_d_b43, eq79_e1230_d_b44, eq79_e1230_d_b45, eq79_e1230_d_b46, eq79_e1230_d_b47, eq79_e1230_d_b48, eq79_e1230_d_b49, eq79_e1230_d_b50, eq79_e1230_d_b51, eq79_e1230_d_b52, eq79_e1230_d_b53, eq79_e1230_d_b54,) = {
    if (s.b[463] && s.b[464]) {
        let eq79_e1220: f64 = (p.p6 * s.v[56]);
        let eq79_e1222: f64 = (eq79_e1220 * s.v[257]);
        let eq79_e1222_d_n0: f64 = (((p.p6 * s.dn[56][0]) * s.v[257]) + (eq79_e1220 * s.dn[257][0]));
        let eq79_e1222_d_n1: f64 = (((p.p6 * s.dn[56][1]) * s.v[257]) + (eq79_e1220 * s.dn[257][1]));
        let eq79_e1222_d_n2: f64 = (((p.p6 * s.dn[56][2]) * s.v[257]) + (eq79_e1220 * s.dn[257][2]));
        let eq79_e1222_d_n3: f64 = (((p.p6 * s.dn[56][3]) * s.v[257]) + (eq79_e1220 * s.dn[257][3]));
        let eq79_e1222_d_n4: f64 = (((p.p6 * s.dn[56][4]) * s.v[257]) + (eq79_e1220 * s.dn[257][4]));
        let eq79_e1222_d_n5: f64 = (((p.p6 * s.dn[56][5]) * s.v[257]) + (eq79_e1220 * s.dn[257][5]));
        let eq79_e1222_d_n6: f64 = (((p.p6 * s.dn[56][6]) * s.v[257]) + (eq79_e1220 * s.dn[257][6]));
        let eq79_e1222_d_n7: f64 = (((p.p6 * s.dn[56][7]) * s.v[257]) + (eq79_e1220 * s.dn[257][7]));
        let eq79_e1222_d_n8: f64 = (((p.p6 * s.dn[56][8]) * s.v[257]) + (eq79_e1220 * s.dn[257][8]));
        let eq79_e1222_d_n9: f64 = (((p.p6 * s.dn[56][9]) * s.v[257]) + (eq79_e1220 * s.dn[257][9]));
        let eq79_e1222_d_n10: f64 = (((p.p6 * s.dn[56][10]) * s.v[257]) + (eq79_e1220 * s.dn[257][10]));
        let eq79_e1222_d_n11: f64 = (((p.p6 * s.dn[56][11]) * s.v[257]) + (eq79_e1220 * s.dn[257][11]));
        let eq79_e1222_d_n12: f64 = (((p.p6 * s.dn[56][12]) * s.v[257]) + (eq79_e1220 * s.dn[257][12]));
        let eq79_e1222_d_n13: f64 = (((p.p6 * s.dn[56][13]) * s.v[257]) + (eq79_e1220 * s.dn[257][13]));
        let eq79_e1222_d_n14: f64 = (((p.p6 * s.dn[56][14]) * s.v[257]) + (eq79_e1220 * s.dn[257][14]));
        let eq79_e1222_d_n15: f64 = (((p.p6 * s.dn[56][15]) * s.v[257]) + (eq79_e1220 * s.dn[257][15]));
        let eq79_e1222_d_n16: f64 = (((p.p6 * s.dn[56][16]) * s.v[257]) + (eq79_e1220 * s.dn[257][16]));
        let eq79_e1222_d_n17: f64 = (((p.p6 * s.dn[56][17]) * s.v[257]) + (eq79_e1220 * s.dn[257][17]));
        let eq79_e1222_d_n18: f64 = (((p.p6 * s.dn[56][18]) * s.v[257]) + (eq79_e1220 * s.dn[257][18]));
        let eq79_e1222_d_n19: f64 = (((p.p6 * s.dn[56][19]) * s.v[257]) + (eq79_e1220 * s.dn[257][19]));
        let eq79_e1222_d_n20: f64 = (((p.p6 * s.dn[56][20]) * s.v[257]) + (eq79_e1220 * s.dn[257][20]));
        let eq79_e1222_d_n21: f64 = (((p.p6 * s.dn[56][21]) * s.v[257]) + (eq79_e1220 * s.dn[257][21]));
        let eq79_e1222_d_n22: f64 = (((p.p6 * s.dn[56][22]) * s.v[257]) + (eq79_e1220 * s.dn[257][22]));
        let eq79_e1222_d_b0: f64 = (((p.p6 * s.db[56][0]) * s.v[257]) + (eq79_e1220 * s.db[257][0]));
        let eq79_e1222_d_b1: f64 = (((p.p6 * s.db[56][1]) * s.v[257]) + (eq79_e1220 * s.db[257][1]));
        let eq79_e1222_d_b2: f64 = (((p.p6 * s.db[56][2]) * s.v[257]) + (eq79_e1220 * s.db[257][2]));
        let eq79_e1222_d_b3: f64 = (((p.p6 * s.db[56][3]) * s.v[257]) + (eq79_e1220 * s.db[257][3]));
        let eq79_e1222_d_b4: f64 = (((p.p6 * s.db[56][4]) * s.v[257]) + (eq79_e1220 * s.db[257][4]));
        let eq79_e1222_d_b5: f64 = (((p.p6 * s.db[56][5]) * s.v[257]) + (eq79_e1220 * s.db[257][5]));
        let eq79_e1222_d_b6: f64 = (((p.p6 * s.db[56][6]) * s.v[257]) + (eq79_e1220 * s.db[257][6]));
        let eq79_e1222_d_b7: f64 = (((p.p6 * s.db[56][7]) * s.v[257]) + (eq79_e1220 * s.db[257][7]));
        let eq79_e1222_d_b8: f64 = (((p.p6 * s.db[56][8]) * s.v[257]) + (eq79_e1220 * s.db[257][8]));
        let eq79_e1222_d_b9: f64 = (((p.p6 * s.db[56][9]) * s.v[257]) + (eq79_e1220 * s.db[257][9]));
        let eq79_e1222_d_b10: f64 = (((p.p6 * s.db[56][10]) * s.v[257]) + (eq79_e1220 * s.db[257][10]));
        let eq79_e1222_d_b11: f64 = (((p.p6 * s.db[56][11]) * s.v[257]) + (eq79_e1220 * s.db[257][11]));
        let eq79_e1222_d_b12: f64 = (((p.p6 * s.db[56][12]) * s.v[257]) + (eq79_e1220 * s.db[257][12]));
        let eq79_e1222_d_b13: f64 = (((p.p6 * s.db[56][13]) * s.v[257]) + (eq79_e1220 * s.db[257][13]));
        let eq79_e1222_d_b14: f64 = (((p.p6 * s.db[56][14]) * s.v[257]) + (eq79_e1220 * s.db[257][14]));
        let eq79_e1222_d_b15: f64 = (((p.p6 * s.db[56][15]) * s.v[257]) + (eq79_e1220 * s.db[257][15]));
        let eq79_e1222_d_b16: f64 = (((p.p6 * s.db[56][16]) * s.v[257]) + (eq79_e1220 * s.db[257][16]));
        let eq79_e1222_d_b17: f64 = (((p.p6 * s.db[56][17]) * s.v[257]) + (eq79_e1220 * s.db[257][17]));
        let eq79_e1222_d_b18: f64 = (((p.p6 * s.db[56][18]) * s.v[257]) + (eq79_e1220 * s.db[257][18]));
        let eq79_e1222_d_b19: f64 = (((p.p6 * s.db[56][19]) * s.v[257]) + (eq79_e1220 * s.db[257][19]));
        let eq79_e1222_d_b20: f64 = (((p.p6 * s.db[56][20]) * s.v[257]) + (eq79_e1220 * s.db[257][20]));
        let eq79_e1222_d_b21: f64 = (((p.p6 * s.db[56][21]) * s.v[257]) + (eq79_e1220 * s.db[257][21]));
        let eq79_e1222_d_b22: f64 = (((p.p6 * s.db[56][22]) * s.v[257]) + (eq79_e1220 * s.db[257][22]));
        let eq79_e1222_d_b23: f64 = (((p.p6 * s.db[56][23]) * s.v[257]) + (eq79_e1220 * s.db[257][23]));
        let eq79_e1222_d_b24: f64 = (((p.p6 * s.db[56][24]) * s.v[257]) + (eq79_e1220 * s.db[257][24]));
        let eq79_e1222_d_b25: f64 = (((p.p6 * s.db[56][25]) * s.v[257]) + (eq79_e1220 * s.db[257][25]));
        let eq79_e1222_d_b26: f64 = (((p.p6 * s.db[56][26]) * s.v[257]) + (eq79_e1220 * s.db[257][26]));
        let eq79_e1222_d_b27: f64 = (((p.p6 * s.db[56][27]) * s.v[257]) + (eq79_e1220 * s.db[257][27]));
        let eq79_e1222_d_b28: f64 = (((p.p6 * s.db[56][28]) * s.v[257]) + (eq79_e1220 * s.db[257][28]));
        let eq79_e1222_d_b29: f64 = (((p.p6 * s.db[56][29]) * s.v[257]) + (eq79_e1220 * s.db[257][29]));
        let eq79_e1222_d_b30: f64 = (((p.p6 * s.db[56][30]) * s.v[257]) + (eq79_e1220 * s.db[257][30]));
        let eq79_e1222_d_b31: f64 = (((p.p6 * s.db[56][31]) * s.v[257]) + (eq79_e1220 * s.db[257][31]));
        let eq79_e1222_d_b32: f64 = (((p.p6 * s.db[56][32]) * s.v[257]) + (eq79_e1220 * s.db[257][32]));
        let eq79_e1222_d_b33: f64 = (((p.p6 * s.db[56][33]) * s.v[257]) + (eq79_e1220 * s.db[257][33]));
        let eq79_e1222_d_b34: f64 = (((p.p6 * s.db[56][34]) * s.v[257]) + (eq79_e1220 * s.db[257][34]));
        let eq79_e1222_d_b35: f64 = (((p.p6 * s.db[56][35]) * s.v[257]) + (eq79_e1220 * s.db[257][35]));
        let eq79_e1222_d_b36: f64 = (((p.p6 * s.db[56][36]) * s.v[257]) + (eq79_e1220 * s.db[257][36]));
        let eq79_e1222_d_b37: f64 = (((p.p6 * s.db[56][37]) * s.v[257]) + (eq79_e1220 * s.db[257][37]));
        let eq79_e1222_d_b38: f64 = (((p.p6 * s.db[56][38]) * s.v[257]) + (eq79_e1220 * s.db[257][38]));
        let eq79_e1222_d_b39: f64 = (((p.p6 * s.db[56][39]) * s.v[257]) + (eq79_e1220 * s.db[257][39]));
        let eq79_e1222_d_b40: f64 = (((p.p6 * s.db[56][40]) * s.v[257]) + (eq79_e1220 * s.db[257][40]));
        let eq79_e1222_d_b41: f64 = (((p.p6 * s.db[56][41]) * s.v[257]) + (eq79_e1220 * s.db[257][41]));
        let eq79_e1222_d_b42: f64 = (((p.p6 * s.db[56][42]) * s.v[257]) + (eq79_e1220 * s.db[257][42]));
        let eq79_e1222_d_b43: f64 = (((p.p6 * s.db[56][43]) * s.v[257]) + (eq79_e1220 * s.db[257][43]));
        let eq79_e1222_d_b44: f64 = (((p.p6 * s.db[56][44]) * s.v[257]) + (eq79_e1220 * s.db[257][44]));
        let eq79_e1222_d_b45: f64 = (((p.p6 * s.db[56][45]) * s.v[257]) + (eq79_e1220 * s.db[257][45]));
        let eq79_e1222_d_b46: f64 = (((p.p6 * s.db[56][46]) * s.v[257]) + (eq79_e1220 * s.db[257][46]));
        let eq79_e1222_d_b47: f64 = (((p.p6 * s.db[56][47]) * s.v[257]) + (eq79_e1220 * s.db[257][47]));
        let eq79_e1222_d_b48: f64 = (((p.p6 * s.db[56][48]) * s.v[257]) + (eq79_e1220 * s.db[257][48]));
        let eq79_e1222_d_b49: f64 = (((p.p6 * s.db[56][49]) * s.v[257]) + (eq79_e1220 * s.db[257][49]));
        let eq79_e1222_d_b50: f64 = (((p.p6 * s.db[56][50]) * s.v[257]) + (eq79_e1220 * s.db[257][50]));
        let eq79_e1222_d_b51: f64 = (((p.p6 * s.db[56][51]) * s.v[257]) + (eq79_e1220 * s.db[257][51]));
        let eq79_e1222_d_b52: f64 = (((p.p6 * s.db[56][52]) * s.v[257]) + (eq79_e1220 * s.db[257][52]));
        let eq79_e1222_d_b53: f64 = (((p.p6 * s.db[56][53]) * s.v[257]) + (eq79_e1220 * s.db[257][53]));
        let eq79_e1222_d_b54: f64 = (((p.p6 * s.db[56][54]) * s.v[257]) + (eq79_e1220 * s.db[257][54]));
        let eq79_e1225: f64 = (p.p6 * s.v[379]);
        let eq79_e1227: f64 = (eq79_e1225 * (nv16 - nv15));
        let eq79_e1227_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv16 - nv15));
        let eq79_e1227_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv16 - nv15));
        let eq79_e1227_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv16 - nv15));
        let eq79_e1227_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv16 - nv15));
        let eq79_e1227_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv16 - nv15));
        let eq79_e1227_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv16 - nv15));
        let eq79_e1227_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv16 - nv15));
        let eq79_e1227_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv16 - nv15));
        let eq79_e1227_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv16 - nv15));
        let eq79_e1227_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv16 - nv15));
        let eq79_e1227_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv16 - nv15));
        let eq79_e1227_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv16 - nv15));
        let eq79_e1227_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv16 - nv15));
        let eq79_e1227_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv16 - nv15));
        let eq79_e1227_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv16 - nv15));
        let eq79_e1227_d_n15: f64 = (((p.p6 * s.dn[379][15]) * (nv16 - nv15)) + (-eq79_e1225));
        let eq79_e1227_d_n16: f64 = (((p.p6 * s.dn[379][16]) * (nv16 - nv15)) + eq79_e1225);
        let eq79_e1227_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv16 - nv15));
        let eq79_e1227_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv16 - nv15));
        let eq79_e1227_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv16 - nv15));
        let eq79_e1227_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv16 - nv15));
        let eq79_e1227_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv16 - nv15));
        let eq79_e1227_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv16 - nv15));
        let eq79_e1227_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv16 - nv15));
        let eq79_e1227_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv16 - nv15));
        let eq79_e1227_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv16 - nv15));
        let eq79_e1227_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv16 - nv15));
        let eq79_e1227_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv16 - nv15));
        let eq79_e1227_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv16 - nv15));
        let eq79_e1227_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv16 - nv15));
        let eq79_e1227_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv16 - nv15));
        let eq79_e1227_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv16 - nv15));
        let eq79_e1227_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv16 - nv15));
        let eq79_e1227_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv16 - nv15));
        let eq79_e1227_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv16 - nv15));
        let eq79_e1227_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv16 - nv15));
        let eq79_e1227_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv16 - nv15));
        let eq79_e1227_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv16 - nv15));
        let eq79_e1227_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv16 - nv15));
        let eq79_e1227_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv16 - nv15));
        let eq79_e1227_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv16 - nv15));
        let eq79_e1227_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv16 - nv15));
        let eq79_e1227_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv16 - nv15));
        let eq79_e1227_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv16 - nv15));
        let eq79_e1227_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv16 - nv15));
        let eq79_e1227_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv16 - nv15));
        let eq79_e1227_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv16 - nv15));
        let eq79_e1227_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv16 - nv15));
        let eq79_e1227_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv16 - nv15));
        let eq79_e1227_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv16 - nv15));
        let eq79_e1227_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv16 - nv15));
        let eq79_e1227_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv16 - nv15));
        let eq79_e1227_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv16 - nv15));
        let eq79_e1227_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv16 - nv15));
        let eq79_e1227_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv16 - nv15));
        let eq79_e1227_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv16 - nv15));
        let eq79_e1227_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv16 - nv15));
        let eq79_e1227_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv16 - nv15));
        let eq79_e1227_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv16 - nv15));
        let eq79_e1227_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv16 - nv15));
        let eq79_e1227_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv16 - nv15));
        let eq79_e1227_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv16 - nv15));
        let eq79_e1227_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv16 - nv15));
        let eq79_e1227_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv16 - nv15));
        let eq79_e1227_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv16 - nv15));
        let eq79_e1227_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv16 - nv15));
        let eq79_e1227_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv16 - nv15));
        let eq79_e1227_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv16 - nv15));
        let eq79_e1227_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv16 - nv15));
        let eq79_e1227_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv16 - nv15));
        let eq79_e1227_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv16 - nv15));
        let eq79_e1227_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv16 - nv15));
        let eq79_e1227_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv16 - nv15));
        let eq79_e1227_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv16 - nv15));
        let eq79_e1227_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv16 - nv15));
        let eq79_e1227_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv16 - nv15));
        let eq79_e1227_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv16 - nv15));
        let eq79_e1227_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv16 - nv15));
        let eq79_e1228: f64 = (eq79_e1222 + eq79_e1227);
        let eq79_e1228_d_n0: f64 = (eq79_e1222_d_n0 + eq79_e1227_d_n0);
        let eq79_e1228_d_n1: f64 = (eq79_e1222_d_n1 + eq79_e1227_d_n1);
        let eq79_e1228_d_n2: f64 = (eq79_e1222_d_n2 + eq79_e1227_d_n2);
        let eq79_e1228_d_n3: f64 = (eq79_e1222_d_n3 + eq79_e1227_d_n3);
        let eq79_e1228_d_n4: f64 = (eq79_e1222_d_n4 + eq79_e1227_d_n4);
        let eq79_e1228_d_n5: f64 = (eq79_e1222_d_n5 + eq79_e1227_d_n5);
        let eq79_e1228_d_n6: f64 = (eq79_e1222_d_n6 + eq79_e1227_d_n6);
        let eq79_e1228_d_n7: f64 = (eq79_e1222_d_n7 + eq79_e1227_d_n7);
        let eq79_e1228_d_n8: f64 = (eq79_e1222_d_n8 + eq79_e1227_d_n8);
        let eq79_e1228_d_n9: f64 = (eq79_e1222_d_n9 + eq79_e1227_d_n9);
        let eq79_e1228_d_n10: f64 = (eq79_e1222_d_n10 + eq79_e1227_d_n10);
        let eq79_e1228_d_n11: f64 = (eq79_e1222_d_n11 + eq79_e1227_d_n11);
        let eq79_e1228_d_n12: f64 = (eq79_e1222_d_n12 + eq79_e1227_d_n12);
        let eq79_e1228_d_n13: f64 = (eq79_e1222_d_n13 + eq79_e1227_d_n13);
        let eq79_e1228_d_n14: f64 = (eq79_e1222_d_n14 + eq79_e1227_d_n14);
        let eq79_e1228_d_n15: f64 = (eq79_e1222_d_n15 + eq79_e1227_d_n15);
        let eq79_e1228_d_n16: f64 = (eq79_e1222_d_n16 + eq79_e1227_d_n16);
        let eq79_e1228_d_n17: f64 = (eq79_e1222_d_n17 + eq79_e1227_d_n17);
        let eq79_e1228_d_n18: f64 = (eq79_e1222_d_n18 + eq79_e1227_d_n18);
        let eq79_e1228_d_n19: f64 = (eq79_e1222_d_n19 + eq79_e1227_d_n19);
        let eq79_e1228_d_n20: f64 = (eq79_e1222_d_n20 + eq79_e1227_d_n20);
        let eq79_e1228_d_n21: f64 = (eq79_e1222_d_n21 + eq79_e1227_d_n21);
        let eq79_e1228_d_n22: f64 = (eq79_e1222_d_n22 + eq79_e1227_d_n22);
        let eq79_e1228_d_b0: f64 = (eq79_e1222_d_b0 + eq79_e1227_d_b0);
        let eq79_e1228_d_b1: f64 = (eq79_e1222_d_b1 + eq79_e1227_d_b1);
        let eq79_e1228_d_b2: f64 = (eq79_e1222_d_b2 + eq79_e1227_d_b2);
        let eq79_e1228_d_b3: f64 = (eq79_e1222_d_b3 + eq79_e1227_d_b3);
        let eq79_e1228_d_b4: f64 = (eq79_e1222_d_b4 + eq79_e1227_d_b4);
        let eq79_e1228_d_b5: f64 = (eq79_e1222_d_b5 + eq79_e1227_d_b5);
        let eq79_e1228_d_b6: f64 = (eq79_e1222_d_b6 + eq79_e1227_d_b6);
        let eq79_e1228_d_b7: f64 = (eq79_e1222_d_b7 + eq79_e1227_d_b7);
        let eq79_e1228_d_b8: f64 = (eq79_e1222_d_b8 + eq79_e1227_d_b8);
        let eq79_e1228_d_b9: f64 = (eq79_e1222_d_b9 + eq79_e1227_d_b9);
        let eq79_e1228_d_b10: f64 = (eq79_e1222_d_b10 + eq79_e1227_d_b10);
        let eq79_e1228_d_b11: f64 = (eq79_e1222_d_b11 + eq79_e1227_d_b11);
        let eq79_e1228_d_b12: f64 = (eq79_e1222_d_b12 + eq79_e1227_d_b12);
        let eq79_e1228_d_b13: f64 = (eq79_e1222_d_b13 + eq79_e1227_d_b13);
        let eq79_e1228_d_b14: f64 = (eq79_e1222_d_b14 + eq79_e1227_d_b14);
        let eq79_e1228_d_b15: f64 = (eq79_e1222_d_b15 + eq79_e1227_d_b15);
        let eq79_e1228_d_b16: f64 = (eq79_e1222_d_b16 + eq79_e1227_d_b16);
        let eq79_e1228_d_b17: f64 = (eq79_e1222_d_b17 + eq79_e1227_d_b17);
        let eq79_e1228_d_b18: f64 = (eq79_e1222_d_b18 + eq79_e1227_d_b18);
        let eq79_e1228_d_b19: f64 = (eq79_e1222_d_b19 + eq79_e1227_d_b19);
        let eq79_e1228_d_b20: f64 = (eq79_e1222_d_b20 + eq79_e1227_d_b20);
        let eq79_e1228_d_b21: f64 = (eq79_e1222_d_b21 + eq79_e1227_d_b21);
        let eq79_e1228_d_b22: f64 = (eq79_e1222_d_b22 + eq79_e1227_d_b22);
        let eq79_e1228_d_b23: f64 = (eq79_e1222_d_b23 + eq79_e1227_d_b23);
        let eq79_e1228_d_b24: f64 = (eq79_e1222_d_b24 + eq79_e1227_d_b24);
        let eq79_e1228_d_b25: f64 = (eq79_e1222_d_b25 + eq79_e1227_d_b25);
        let eq79_e1228_d_b26: f64 = (eq79_e1222_d_b26 + eq79_e1227_d_b26);
        let eq79_e1228_d_b27: f64 = (eq79_e1222_d_b27 + eq79_e1227_d_b27);
        let eq79_e1228_d_b28: f64 = (eq79_e1222_d_b28 + eq79_e1227_d_b28);
        let eq79_e1228_d_b29: f64 = (eq79_e1222_d_b29 + eq79_e1227_d_b29);
        let eq79_e1228_d_b30: f64 = (eq79_e1222_d_b30 + eq79_e1227_d_b30);
        let eq79_e1228_d_b31: f64 = (eq79_e1222_d_b31 + eq79_e1227_d_b31);
        let eq79_e1228_d_b32: f64 = (eq79_e1222_d_b32 + eq79_e1227_d_b32);
        let eq79_e1228_d_b33: f64 = (eq79_e1222_d_b33 + eq79_e1227_d_b33);
        let eq79_e1228_d_b34: f64 = (eq79_e1222_d_b34 + eq79_e1227_d_b34);
        let eq79_e1228_d_b35: f64 = (eq79_e1222_d_b35 + eq79_e1227_d_b35);
        let eq79_e1228_d_b36: f64 = (eq79_e1222_d_b36 + eq79_e1227_d_b36);
        let eq79_e1228_d_b37: f64 = (eq79_e1222_d_b37 + eq79_e1227_d_b37);
        let eq79_e1228_d_b38: f64 = (eq79_e1222_d_b38 + eq79_e1227_d_b38);
        let eq79_e1228_d_b39: f64 = (eq79_e1222_d_b39 + eq79_e1227_d_b39);
        let eq79_e1228_d_b40: f64 = (eq79_e1222_d_b40 + eq79_e1227_d_b40);
        let eq79_e1228_d_b41: f64 = (eq79_e1222_d_b41 + eq79_e1227_d_b41);
        let eq79_e1228_d_b42: f64 = (eq79_e1222_d_b42 + eq79_e1227_d_b42);
        let eq79_e1228_d_b43: f64 = (eq79_e1222_d_b43 + eq79_e1227_d_b43);
        let eq79_e1228_d_b44: f64 = (eq79_e1222_d_b44 + eq79_e1227_d_b44);
        let eq79_e1228_d_b45: f64 = (eq79_e1222_d_b45 + eq79_e1227_d_b45);
        let eq79_e1228_d_b46: f64 = (eq79_e1222_d_b46 + eq79_e1227_d_b46);
        let eq79_e1228_d_b47: f64 = (eq79_e1222_d_b47 + eq79_e1227_d_b47);
        let eq79_e1228_d_b48: f64 = (eq79_e1222_d_b48 + eq79_e1227_d_b48);
        let eq79_e1228_d_b49: f64 = (eq79_e1222_d_b49 + eq79_e1227_d_b49);
        let eq79_e1228_d_b50: f64 = (eq79_e1222_d_b50 + eq79_e1227_d_b50);
        let eq79_e1228_d_b51: f64 = (eq79_e1222_d_b51 + eq79_e1227_d_b51);
        let eq79_e1228_d_b52: f64 = (eq79_e1222_d_b52 + eq79_e1227_d_b52);
        let eq79_e1228_d_b53: f64 = (eq79_e1222_d_b53 + eq79_e1227_d_b53);
        let eq79_e1228_d_b54: f64 = (eq79_e1222_d_b54 + eq79_e1227_d_b54);
        (eq79_e1228, eq79_e1228_d_n0, eq79_e1228_d_n1, eq79_e1228_d_n2, eq79_e1228_d_n3, eq79_e1228_d_n4, eq79_e1228_d_n5, eq79_e1228_d_n6, eq79_e1228_d_n7, eq79_e1228_d_n8, eq79_e1228_d_n9, eq79_e1228_d_n10, eq79_e1228_d_n11, eq79_e1228_d_n12, eq79_e1228_d_n13, eq79_e1228_d_n14, eq79_e1228_d_n15, eq79_e1228_d_n16, eq79_e1228_d_n17, eq79_e1228_d_n18, eq79_e1228_d_n19, eq79_e1228_d_n20, eq79_e1228_d_n21, eq79_e1228_d_n22, eq79_e1228_d_b0, eq79_e1228_d_b1, eq79_e1228_d_b2, eq79_e1228_d_b3, eq79_e1228_d_b4, eq79_e1228_d_b5, eq79_e1228_d_b6, eq79_e1228_d_b7, eq79_e1228_d_b8, eq79_e1228_d_b9, eq79_e1228_d_b10, eq79_e1228_d_b11, eq79_e1228_d_b12, eq79_e1228_d_b13, eq79_e1228_d_b14, eq79_e1228_d_b15, eq79_e1228_d_b16, eq79_e1228_d_b17, eq79_e1228_d_b18, eq79_e1228_d_b19, eq79_e1228_d_b20, eq79_e1228_d_b21, eq79_e1228_d_b22, eq79_e1228_d_b23, eq79_e1228_d_b24, eq79_e1228_d_b25, eq79_e1228_d_b26, eq79_e1228_d_b27, eq79_e1228_d_b28, eq79_e1228_d_b29, eq79_e1228_d_b30, eq79_e1228_d_b31, eq79_e1228_d_b32, eq79_e1228_d_b33, eq79_e1228_d_b34, eq79_e1228_d_b35, eq79_e1228_d_b36, eq79_e1228_d_b37, eq79_e1228_d_b38, eq79_e1228_d_b39, eq79_e1228_d_b40, eq79_e1228_d_b41, eq79_e1228_d_b42, eq79_e1228_d_b43, eq79_e1228_d_b44, eq79_e1228_d_b45, eq79_e1228_d_b46, eq79_e1228_d_b47, eq79_e1228_d_b48, eq79_e1228_d_b49, eq79_e1228_d_b50, eq79_e1228_d_b51, eq79_e1228_d_b52, eq79_e1228_d_b53, eq79_e1228_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1230;
        let eq79_node_derivatives: [f64; 23] = [eq79_e1230_d_n0, eq79_e1230_d_n1, eq79_e1230_d_n2, eq79_e1230_d_n3, eq79_e1230_d_n4, eq79_e1230_d_n5, eq79_e1230_d_n6, eq79_e1230_d_n7, eq79_e1230_d_n8, eq79_e1230_d_n9, eq79_e1230_d_n10, eq79_e1230_d_n11, eq79_e1230_d_n12, eq79_e1230_d_n13, eq79_e1230_d_n14, eq79_e1230_d_n15, eq79_e1230_d_n16, eq79_e1230_d_n17, eq79_e1230_d_n18, eq79_e1230_d_n19, eq79_e1230_d_n20, eq79_e1230_d_n21, eq79_e1230_d_n22];
        let eq79_branch_derivatives: [f64; 55] = [eq79_e1230_d_b0, eq79_e1230_d_b1, eq79_e1230_d_b2, eq79_e1230_d_b3, eq79_e1230_d_b4, eq79_e1230_d_b5, eq79_e1230_d_b6, eq79_e1230_d_b7, eq79_e1230_d_b8, eq79_e1230_d_b9, eq79_e1230_d_b10, eq79_e1230_d_b11, eq79_e1230_d_b12, eq79_e1230_d_b13, eq79_e1230_d_b14, eq79_e1230_d_b15, eq79_e1230_d_b16, eq79_e1230_d_b17, eq79_e1230_d_b18, eq79_e1230_d_b19, eq79_e1230_d_b20, eq79_e1230_d_b21, eq79_e1230_d_b22, eq79_e1230_d_b23, eq79_e1230_d_b24, eq79_e1230_d_b25, eq79_e1230_d_b26, eq79_e1230_d_b27, eq79_e1230_d_b28, eq79_e1230_d_b29, eq79_e1230_d_b30, eq79_e1230_d_b31, eq79_e1230_d_b32, eq79_e1230_d_b33, eq79_e1230_d_b34, eq79_e1230_d_b35, eq79_e1230_d_b36, eq79_e1230_d_b37, eq79_e1230_d_b38, eq79_e1230_d_b39, eq79_e1230_d_b40, eq79_e1230_d_b41, eq79_e1230_d_b42, eq79_e1230_d_b43, eq79_e1230_d_b44, eq79_e1230_d_b45, eq79_e1230_d_b46, eq79_e1230_d_b47, eq79_e1230_d_b48, eq79_e1230_d_b49, eq79_e1230_d_b50, eq79_e1230_d_b51, eq79_e1230_d_b52, eq79_e1230_d_b53, eq79_e1230_d_b54];
        stamper.stamp_current_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq79_value),
            &eq79_node_derivatives,
            &eq79_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq82_e1258, eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22, eq82_e1258_d_b0, eq82_e1258_d_b1, eq82_e1258_d_b2, eq82_e1258_d_b3, eq82_e1258_d_b4, eq82_e1258_d_b5, eq82_e1258_d_b6, eq82_e1258_d_b7, eq82_e1258_d_b8, eq82_e1258_d_b9, eq82_e1258_d_b10, eq82_e1258_d_b11, eq82_e1258_d_b12, eq82_e1258_d_b13, eq82_e1258_d_b14, eq82_e1258_d_b15, eq82_e1258_d_b16, eq82_e1258_d_b17, eq82_e1258_d_b18, eq82_e1258_d_b19, eq82_e1258_d_b20, eq82_e1258_d_b21, eq82_e1258_d_b22, eq82_e1258_d_b23, eq82_e1258_d_b24, eq82_e1258_d_b25, eq82_e1258_d_b26, eq82_e1258_d_b27, eq82_e1258_d_b28, eq82_e1258_d_b29, eq82_e1258_d_b30, eq82_e1258_d_b31, eq82_e1258_d_b32, eq82_e1258_d_b33, eq82_e1258_d_b34, eq82_e1258_d_b35, eq82_e1258_d_b36, eq82_e1258_d_b37, eq82_e1258_d_b38, eq82_e1258_d_b39, eq82_e1258_d_b40, eq82_e1258_d_b41, eq82_e1258_d_b42, eq82_e1258_d_b43, eq82_e1258_d_b44, eq82_e1258_d_b45, eq82_e1258_d_b46, eq82_e1258_d_b47, eq82_e1258_d_b48, eq82_e1258_d_b49, eq82_e1258_d_b50, eq82_e1258_d_b51, eq82_e1258_d_b52, eq82_e1258_d_b53, eq82_e1258_d_b54,) = {
    if (s.b[478] && s.b[479]) {
        let eq82_e1248: f64 = (p.p6 * s.v[60]);
        let eq82_e1250: f64 = (eq82_e1248 * s.v[269]);
        let eq82_e1250_d_n0: f64 = (((p.p6 * s.dn[60][0]) * s.v[269]) + (eq82_e1248 * s.dn[269][0]));
        let eq82_e1250_d_n1: f64 = (((p.p6 * s.dn[60][1]) * s.v[269]) + (eq82_e1248 * s.dn[269][1]));
        let eq82_e1250_d_n2: f64 = (((p.p6 * s.dn[60][2]) * s.v[269]) + (eq82_e1248 * s.dn[269][2]));
        let eq82_e1250_d_n3: f64 = (((p.p6 * s.dn[60][3]) * s.v[269]) + (eq82_e1248 * s.dn[269][3]));
        let eq82_e1250_d_n4: f64 = (((p.p6 * s.dn[60][4]) * s.v[269]) + (eq82_e1248 * s.dn[269][4]));
        let eq82_e1250_d_n5: f64 = (((p.p6 * s.dn[60][5]) * s.v[269]) + (eq82_e1248 * s.dn[269][5]));
        let eq82_e1250_d_n6: f64 = (((p.p6 * s.dn[60][6]) * s.v[269]) + (eq82_e1248 * s.dn[269][6]));
        let eq82_e1250_d_n7: f64 = (((p.p6 * s.dn[60][7]) * s.v[269]) + (eq82_e1248 * s.dn[269][7]));
        let eq82_e1250_d_n8: f64 = (((p.p6 * s.dn[60][8]) * s.v[269]) + (eq82_e1248 * s.dn[269][8]));
        let eq82_e1250_d_n9: f64 = (((p.p6 * s.dn[60][9]) * s.v[269]) + (eq82_e1248 * s.dn[269][9]));
        let eq82_e1250_d_n10: f64 = (((p.p6 * s.dn[60][10]) * s.v[269]) + (eq82_e1248 * s.dn[269][10]));
        let eq82_e1250_d_n11: f64 = (((p.p6 * s.dn[60][11]) * s.v[269]) + (eq82_e1248 * s.dn[269][11]));
        let eq82_e1250_d_n12: f64 = (((p.p6 * s.dn[60][12]) * s.v[269]) + (eq82_e1248 * s.dn[269][12]));
        let eq82_e1250_d_n13: f64 = (((p.p6 * s.dn[60][13]) * s.v[269]) + (eq82_e1248 * s.dn[269][13]));
        let eq82_e1250_d_n14: f64 = (((p.p6 * s.dn[60][14]) * s.v[269]) + (eq82_e1248 * s.dn[269][14]));
        let eq82_e1250_d_n15: f64 = (((p.p6 * s.dn[60][15]) * s.v[269]) + (eq82_e1248 * s.dn[269][15]));
        let eq82_e1250_d_n16: f64 = (((p.p6 * s.dn[60][16]) * s.v[269]) + (eq82_e1248 * s.dn[269][16]));
        let eq82_e1250_d_n17: f64 = (((p.p6 * s.dn[60][17]) * s.v[269]) + (eq82_e1248 * s.dn[269][17]));
        let eq82_e1250_d_n18: f64 = (((p.p6 * s.dn[60][18]) * s.v[269]) + (eq82_e1248 * s.dn[269][18]));
        let eq82_e1250_d_n19: f64 = (((p.p6 * s.dn[60][19]) * s.v[269]) + (eq82_e1248 * s.dn[269][19]));
        let eq82_e1250_d_n20: f64 = (((p.p6 * s.dn[60][20]) * s.v[269]) + (eq82_e1248 * s.dn[269][20]));
        let eq82_e1250_d_n21: f64 = (((p.p6 * s.dn[60][21]) * s.v[269]) + (eq82_e1248 * s.dn[269][21]));
        let eq82_e1250_d_n22: f64 = (((p.p6 * s.dn[60][22]) * s.v[269]) + (eq82_e1248 * s.dn[269][22]));
        let eq82_e1250_d_b0: f64 = (((p.p6 * s.db[60][0]) * s.v[269]) + (eq82_e1248 * s.db[269][0]));
        let eq82_e1250_d_b1: f64 = (((p.p6 * s.db[60][1]) * s.v[269]) + (eq82_e1248 * s.db[269][1]));
        let eq82_e1250_d_b2: f64 = (((p.p6 * s.db[60][2]) * s.v[269]) + (eq82_e1248 * s.db[269][2]));
        let eq82_e1250_d_b3: f64 = (((p.p6 * s.db[60][3]) * s.v[269]) + (eq82_e1248 * s.db[269][3]));
        let eq82_e1250_d_b4: f64 = (((p.p6 * s.db[60][4]) * s.v[269]) + (eq82_e1248 * s.db[269][4]));
        let eq82_e1250_d_b5: f64 = (((p.p6 * s.db[60][5]) * s.v[269]) + (eq82_e1248 * s.db[269][5]));
        let eq82_e1250_d_b6: f64 = (((p.p6 * s.db[60][6]) * s.v[269]) + (eq82_e1248 * s.db[269][6]));
        let eq82_e1250_d_b7: f64 = (((p.p6 * s.db[60][7]) * s.v[269]) + (eq82_e1248 * s.db[269][7]));
        let eq82_e1250_d_b8: f64 = (((p.p6 * s.db[60][8]) * s.v[269]) + (eq82_e1248 * s.db[269][8]));
        let eq82_e1250_d_b9: f64 = (((p.p6 * s.db[60][9]) * s.v[269]) + (eq82_e1248 * s.db[269][9]));
        let eq82_e1250_d_b10: f64 = (((p.p6 * s.db[60][10]) * s.v[269]) + (eq82_e1248 * s.db[269][10]));
        let eq82_e1250_d_b11: f64 = (((p.p6 * s.db[60][11]) * s.v[269]) + (eq82_e1248 * s.db[269][11]));
        let eq82_e1250_d_b12: f64 = (((p.p6 * s.db[60][12]) * s.v[269]) + (eq82_e1248 * s.db[269][12]));
        let eq82_e1250_d_b13: f64 = (((p.p6 * s.db[60][13]) * s.v[269]) + (eq82_e1248 * s.db[269][13]));
        let eq82_e1250_d_b14: f64 = (((p.p6 * s.db[60][14]) * s.v[269]) + (eq82_e1248 * s.db[269][14]));
        let eq82_e1250_d_b15: f64 = (((p.p6 * s.db[60][15]) * s.v[269]) + (eq82_e1248 * s.db[269][15]));
        let eq82_e1250_d_b16: f64 = (((p.p6 * s.db[60][16]) * s.v[269]) + (eq82_e1248 * s.db[269][16]));
        let eq82_e1250_d_b17: f64 = (((p.p6 * s.db[60][17]) * s.v[269]) + (eq82_e1248 * s.db[269][17]));
        let eq82_e1250_d_b18: f64 = (((p.p6 * s.db[60][18]) * s.v[269]) + (eq82_e1248 * s.db[269][18]));
        let eq82_e1250_d_b19: f64 = (((p.p6 * s.db[60][19]) * s.v[269]) + (eq82_e1248 * s.db[269][19]));
        let eq82_e1250_d_b20: f64 = (((p.p6 * s.db[60][20]) * s.v[269]) + (eq82_e1248 * s.db[269][20]));
        let eq82_e1250_d_b21: f64 = (((p.p6 * s.db[60][21]) * s.v[269]) + (eq82_e1248 * s.db[269][21]));
        let eq82_e1250_d_b22: f64 = (((p.p6 * s.db[60][22]) * s.v[269]) + (eq82_e1248 * s.db[269][22]));
        let eq82_e1250_d_b23: f64 = (((p.p6 * s.db[60][23]) * s.v[269]) + (eq82_e1248 * s.db[269][23]));
        let eq82_e1250_d_b24: f64 = (((p.p6 * s.db[60][24]) * s.v[269]) + (eq82_e1248 * s.db[269][24]));
        let eq82_e1250_d_b25: f64 = (((p.p6 * s.db[60][25]) * s.v[269]) + (eq82_e1248 * s.db[269][25]));
        let eq82_e1250_d_b26: f64 = (((p.p6 * s.db[60][26]) * s.v[269]) + (eq82_e1248 * s.db[269][26]));
        let eq82_e1250_d_b27: f64 = (((p.p6 * s.db[60][27]) * s.v[269]) + (eq82_e1248 * s.db[269][27]));
        let eq82_e1250_d_b28: f64 = (((p.p6 * s.db[60][28]) * s.v[269]) + (eq82_e1248 * s.db[269][28]));
        let eq82_e1250_d_b29: f64 = (((p.p6 * s.db[60][29]) * s.v[269]) + (eq82_e1248 * s.db[269][29]));
        let eq82_e1250_d_b30: f64 = (((p.p6 * s.db[60][30]) * s.v[269]) + (eq82_e1248 * s.db[269][30]));
        let eq82_e1250_d_b31: f64 = (((p.p6 * s.db[60][31]) * s.v[269]) + (eq82_e1248 * s.db[269][31]));
        let eq82_e1250_d_b32: f64 = (((p.p6 * s.db[60][32]) * s.v[269]) + (eq82_e1248 * s.db[269][32]));
        let eq82_e1250_d_b33: f64 = (((p.p6 * s.db[60][33]) * s.v[269]) + (eq82_e1248 * s.db[269][33]));
        let eq82_e1250_d_b34: f64 = (((p.p6 * s.db[60][34]) * s.v[269]) + (eq82_e1248 * s.db[269][34]));
        let eq82_e1250_d_b35: f64 = (((p.p6 * s.db[60][35]) * s.v[269]) + (eq82_e1248 * s.db[269][35]));
        let eq82_e1250_d_b36: f64 = (((p.p6 * s.db[60][36]) * s.v[269]) + (eq82_e1248 * s.db[269][36]));
        let eq82_e1250_d_b37: f64 = (((p.p6 * s.db[60][37]) * s.v[269]) + (eq82_e1248 * s.db[269][37]));
        let eq82_e1250_d_b38: f64 = (((p.p6 * s.db[60][38]) * s.v[269]) + (eq82_e1248 * s.db[269][38]));
        let eq82_e1250_d_b39: f64 = (((p.p6 * s.db[60][39]) * s.v[269]) + (eq82_e1248 * s.db[269][39]));
        let eq82_e1250_d_b40: f64 = (((p.p6 * s.db[60][40]) * s.v[269]) + (eq82_e1248 * s.db[269][40]));
        let eq82_e1250_d_b41: f64 = (((p.p6 * s.db[60][41]) * s.v[269]) + (eq82_e1248 * s.db[269][41]));
        let eq82_e1250_d_b42: f64 = (((p.p6 * s.db[60][42]) * s.v[269]) + (eq82_e1248 * s.db[269][42]));
        let eq82_e1250_d_b43: f64 = (((p.p6 * s.db[60][43]) * s.v[269]) + (eq82_e1248 * s.db[269][43]));
        let eq82_e1250_d_b44: f64 = (((p.p6 * s.db[60][44]) * s.v[269]) + (eq82_e1248 * s.db[269][44]));
        let eq82_e1250_d_b45: f64 = (((p.p6 * s.db[60][45]) * s.v[269]) + (eq82_e1248 * s.db[269][45]));
        let eq82_e1250_d_b46: f64 = (((p.p6 * s.db[60][46]) * s.v[269]) + (eq82_e1248 * s.db[269][46]));
        let eq82_e1250_d_b47: f64 = (((p.p6 * s.db[60][47]) * s.v[269]) + (eq82_e1248 * s.db[269][47]));
        let eq82_e1250_d_b48: f64 = (((p.p6 * s.db[60][48]) * s.v[269]) + (eq82_e1248 * s.db[269][48]));
        let eq82_e1250_d_b49: f64 = (((p.p6 * s.db[60][49]) * s.v[269]) + (eq82_e1248 * s.db[269][49]));
        let eq82_e1250_d_b50: f64 = (((p.p6 * s.db[60][50]) * s.v[269]) + (eq82_e1248 * s.db[269][50]));
        let eq82_e1250_d_b51: f64 = (((p.p6 * s.db[60][51]) * s.v[269]) + (eq82_e1248 * s.db[269][51]));
        let eq82_e1250_d_b52: f64 = (((p.p6 * s.db[60][52]) * s.v[269]) + (eq82_e1248 * s.db[269][52]));
        let eq82_e1250_d_b53: f64 = (((p.p6 * s.db[60][53]) * s.v[269]) + (eq82_e1248 * s.db[269][53]));
        let eq82_e1250_d_b54: f64 = (((p.p6 * s.db[60][54]) * s.v[269]) + (eq82_e1248 * s.db[269][54]));
        let eq82_e1253: f64 = (p.p6 * s.v[379]);
        let eq82_e1255: f64 = (eq82_e1253 * (nv19 - nv20));
        let eq82_e1255_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv19 - nv20));
        let eq82_e1255_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv19 - nv20));
        let eq82_e1255_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv19 - nv20));
        let eq82_e1255_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv19 - nv20));
        let eq82_e1255_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv19 - nv20));
        let eq82_e1255_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv19 - nv20));
        let eq82_e1255_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv19 - nv20));
        let eq82_e1255_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv19 - nv20));
        let eq82_e1255_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv19 - nv20));
        let eq82_e1255_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv19 - nv20));
        let eq82_e1255_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv19 - nv20));
        let eq82_e1255_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv19 - nv20));
        let eq82_e1255_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv19 - nv20));
        let eq82_e1255_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv19 - nv20));
        let eq82_e1255_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv19 - nv20));
        let eq82_e1255_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv19 - nv20));
        let eq82_e1255_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv19 - nv20));
        let eq82_e1255_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv19 - nv20));
        let eq82_e1255_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv19 - nv20));
        let eq82_e1255_d_n19: f64 = (((p.p6 * s.dn[379][19]) * (nv19 - nv20)) + eq82_e1253);
        let eq82_e1255_d_n20: f64 = (((p.p6 * s.dn[379][20]) * (nv19 - nv20)) + (-eq82_e1253));
        let eq82_e1255_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv19 - nv20));
        let eq82_e1255_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv19 - nv20));
        let eq82_e1255_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv19 - nv20));
        let eq82_e1255_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv19 - nv20));
        let eq82_e1255_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv19 - nv20));
        let eq82_e1255_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv19 - nv20));
        let eq82_e1255_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv19 - nv20));
        let eq82_e1255_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv19 - nv20));
        let eq82_e1255_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv19 - nv20));
        let eq82_e1255_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv19 - nv20));
        let eq82_e1255_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv19 - nv20));
        let eq82_e1255_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv19 - nv20));
        let eq82_e1255_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv19 - nv20));
        let eq82_e1255_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv19 - nv20));
        let eq82_e1255_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv19 - nv20));
        let eq82_e1255_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv19 - nv20));
        let eq82_e1255_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv19 - nv20));
        let eq82_e1255_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv19 - nv20));
        let eq82_e1255_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv19 - nv20));
        let eq82_e1255_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv19 - nv20));
        let eq82_e1255_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv19 - nv20));
        let eq82_e1255_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv19 - nv20));
        let eq82_e1255_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv19 - nv20));
        let eq82_e1255_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv19 - nv20));
        let eq82_e1255_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv19 - nv20));
        let eq82_e1255_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv19 - nv20));
        let eq82_e1255_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv19 - nv20));
        let eq82_e1255_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv19 - nv20));
        let eq82_e1255_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv19 - nv20));
        let eq82_e1255_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv19 - nv20));
        let eq82_e1255_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv19 - nv20));
        let eq82_e1255_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv19 - nv20));
        let eq82_e1255_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv19 - nv20));
        let eq82_e1255_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv19 - nv20));
        let eq82_e1255_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv19 - nv20));
        let eq82_e1255_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv19 - nv20));
        let eq82_e1255_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv19 - nv20));
        let eq82_e1255_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv19 - nv20));
        let eq82_e1255_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv19 - nv20));
        let eq82_e1255_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv19 - nv20));
        let eq82_e1255_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv19 - nv20));
        let eq82_e1255_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv19 - nv20));
        let eq82_e1255_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv19 - nv20));
        let eq82_e1255_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv19 - nv20));
        let eq82_e1255_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv19 - nv20));
        let eq82_e1255_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv19 - nv20));
        let eq82_e1255_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv19 - nv20));
        let eq82_e1255_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv19 - nv20));
        let eq82_e1255_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv19 - nv20));
        let eq82_e1255_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv19 - nv20));
        let eq82_e1255_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv19 - nv20));
        let eq82_e1255_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv19 - nv20));
        let eq82_e1255_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv19 - nv20));
        let eq82_e1255_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv19 - nv20));
        let eq82_e1255_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv19 - nv20));
        let eq82_e1255_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv19 - nv20));
        let eq82_e1255_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv19 - nv20));
        let eq82_e1256: f64 = (eq82_e1250 + eq82_e1255);
        let eq82_e1256_d_n0: f64 = (eq82_e1250_d_n0 + eq82_e1255_d_n0);
        let eq82_e1256_d_n1: f64 = (eq82_e1250_d_n1 + eq82_e1255_d_n1);
        let eq82_e1256_d_n2: f64 = (eq82_e1250_d_n2 + eq82_e1255_d_n2);
        let eq82_e1256_d_n3: f64 = (eq82_e1250_d_n3 + eq82_e1255_d_n3);
        let eq82_e1256_d_n4: f64 = (eq82_e1250_d_n4 + eq82_e1255_d_n4);
        let eq82_e1256_d_n5: f64 = (eq82_e1250_d_n5 + eq82_e1255_d_n5);
        let eq82_e1256_d_n6: f64 = (eq82_e1250_d_n6 + eq82_e1255_d_n6);
        let eq82_e1256_d_n7: f64 = (eq82_e1250_d_n7 + eq82_e1255_d_n7);
        let eq82_e1256_d_n8: f64 = (eq82_e1250_d_n8 + eq82_e1255_d_n8);
        let eq82_e1256_d_n9: f64 = (eq82_e1250_d_n9 + eq82_e1255_d_n9);
        let eq82_e1256_d_n10: f64 = (eq82_e1250_d_n10 + eq82_e1255_d_n10);
        let eq82_e1256_d_n11: f64 = (eq82_e1250_d_n11 + eq82_e1255_d_n11);
        let eq82_e1256_d_n12: f64 = (eq82_e1250_d_n12 + eq82_e1255_d_n12);
        let eq82_e1256_d_n13: f64 = (eq82_e1250_d_n13 + eq82_e1255_d_n13);
        let eq82_e1256_d_n14: f64 = (eq82_e1250_d_n14 + eq82_e1255_d_n14);
        let eq82_e1256_d_n15: f64 = (eq82_e1250_d_n15 + eq82_e1255_d_n15);
        let eq82_e1256_d_n16: f64 = (eq82_e1250_d_n16 + eq82_e1255_d_n16);
        let eq82_e1256_d_n17: f64 = (eq82_e1250_d_n17 + eq82_e1255_d_n17);
        let eq82_e1256_d_n18: f64 = (eq82_e1250_d_n18 + eq82_e1255_d_n18);
        let eq82_e1256_d_n19: f64 = (eq82_e1250_d_n19 + eq82_e1255_d_n19);
        let eq82_e1256_d_n20: f64 = (eq82_e1250_d_n20 + eq82_e1255_d_n20);
        let eq82_e1256_d_n21: f64 = (eq82_e1250_d_n21 + eq82_e1255_d_n21);
        let eq82_e1256_d_n22: f64 = (eq82_e1250_d_n22 + eq82_e1255_d_n22);
        let eq82_e1256_d_b0: f64 = (eq82_e1250_d_b0 + eq82_e1255_d_b0);
        let eq82_e1256_d_b1: f64 = (eq82_e1250_d_b1 + eq82_e1255_d_b1);
        let eq82_e1256_d_b2: f64 = (eq82_e1250_d_b2 + eq82_e1255_d_b2);
        let eq82_e1256_d_b3: f64 = (eq82_e1250_d_b3 + eq82_e1255_d_b3);
        let eq82_e1256_d_b4: f64 = (eq82_e1250_d_b4 + eq82_e1255_d_b4);
        let eq82_e1256_d_b5: f64 = (eq82_e1250_d_b5 + eq82_e1255_d_b5);
        let eq82_e1256_d_b6: f64 = (eq82_e1250_d_b6 + eq82_e1255_d_b6);
        let eq82_e1256_d_b7: f64 = (eq82_e1250_d_b7 + eq82_e1255_d_b7);
        let eq82_e1256_d_b8: f64 = (eq82_e1250_d_b8 + eq82_e1255_d_b8);
        let eq82_e1256_d_b9: f64 = (eq82_e1250_d_b9 + eq82_e1255_d_b9);
        let eq82_e1256_d_b10: f64 = (eq82_e1250_d_b10 + eq82_e1255_d_b10);
        let eq82_e1256_d_b11: f64 = (eq82_e1250_d_b11 + eq82_e1255_d_b11);
        let eq82_e1256_d_b12: f64 = (eq82_e1250_d_b12 + eq82_e1255_d_b12);
        let eq82_e1256_d_b13: f64 = (eq82_e1250_d_b13 + eq82_e1255_d_b13);
        let eq82_e1256_d_b14: f64 = (eq82_e1250_d_b14 + eq82_e1255_d_b14);
        let eq82_e1256_d_b15: f64 = (eq82_e1250_d_b15 + eq82_e1255_d_b15);
        let eq82_e1256_d_b16: f64 = (eq82_e1250_d_b16 + eq82_e1255_d_b16);
        let eq82_e1256_d_b17: f64 = (eq82_e1250_d_b17 + eq82_e1255_d_b17);
        let eq82_e1256_d_b18: f64 = (eq82_e1250_d_b18 + eq82_e1255_d_b18);
        let eq82_e1256_d_b19: f64 = (eq82_e1250_d_b19 + eq82_e1255_d_b19);
        let eq82_e1256_d_b20: f64 = (eq82_e1250_d_b20 + eq82_e1255_d_b20);
        let eq82_e1256_d_b21: f64 = (eq82_e1250_d_b21 + eq82_e1255_d_b21);
        let eq82_e1256_d_b22: f64 = (eq82_e1250_d_b22 + eq82_e1255_d_b22);
        let eq82_e1256_d_b23: f64 = (eq82_e1250_d_b23 + eq82_e1255_d_b23);
        let eq82_e1256_d_b24: f64 = (eq82_e1250_d_b24 + eq82_e1255_d_b24);
        let eq82_e1256_d_b25: f64 = (eq82_e1250_d_b25 + eq82_e1255_d_b25);
        let eq82_e1256_d_b26: f64 = (eq82_e1250_d_b26 + eq82_e1255_d_b26);
        let eq82_e1256_d_b27: f64 = (eq82_e1250_d_b27 + eq82_e1255_d_b27);
        let eq82_e1256_d_b28: f64 = (eq82_e1250_d_b28 + eq82_e1255_d_b28);
        let eq82_e1256_d_b29: f64 = (eq82_e1250_d_b29 + eq82_e1255_d_b29);
        let eq82_e1256_d_b30: f64 = (eq82_e1250_d_b30 + eq82_e1255_d_b30);
        let eq82_e1256_d_b31: f64 = (eq82_e1250_d_b31 + eq82_e1255_d_b31);
        let eq82_e1256_d_b32: f64 = (eq82_e1250_d_b32 + eq82_e1255_d_b32);
        let eq82_e1256_d_b33: f64 = (eq82_e1250_d_b33 + eq82_e1255_d_b33);
        let eq82_e1256_d_b34: f64 = (eq82_e1250_d_b34 + eq82_e1255_d_b34);
        let eq82_e1256_d_b35: f64 = (eq82_e1250_d_b35 + eq82_e1255_d_b35);
        let eq82_e1256_d_b36: f64 = (eq82_e1250_d_b36 + eq82_e1255_d_b36);
        let eq82_e1256_d_b37: f64 = (eq82_e1250_d_b37 + eq82_e1255_d_b37);
        let eq82_e1256_d_b38: f64 = (eq82_e1250_d_b38 + eq82_e1255_d_b38);
        let eq82_e1256_d_b39: f64 = (eq82_e1250_d_b39 + eq82_e1255_d_b39);
        let eq82_e1256_d_b40: f64 = (eq82_e1250_d_b40 + eq82_e1255_d_b40);
        let eq82_e1256_d_b41: f64 = (eq82_e1250_d_b41 + eq82_e1255_d_b41);
        let eq82_e1256_d_b42: f64 = (eq82_e1250_d_b42 + eq82_e1255_d_b42);
        let eq82_e1256_d_b43: f64 = (eq82_e1250_d_b43 + eq82_e1255_d_b43);
        let eq82_e1256_d_b44: f64 = (eq82_e1250_d_b44 + eq82_e1255_d_b44);
        let eq82_e1256_d_b45: f64 = (eq82_e1250_d_b45 + eq82_e1255_d_b45);
        let eq82_e1256_d_b46: f64 = (eq82_e1250_d_b46 + eq82_e1255_d_b46);
        let eq82_e1256_d_b47: f64 = (eq82_e1250_d_b47 + eq82_e1255_d_b47);
        let eq82_e1256_d_b48: f64 = (eq82_e1250_d_b48 + eq82_e1255_d_b48);
        let eq82_e1256_d_b49: f64 = (eq82_e1250_d_b49 + eq82_e1255_d_b49);
        let eq82_e1256_d_b50: f64 = (eq82_e1250_d_b50 + eq82_e1255_d_b50);
        let eq82_e1256_d_b51: f64 = (eq82_e1250_d_b51 + eq82_e1255_d_b51);
        let eq82_e1256_d_b52: f64 = (eq82_e1250_d_b52 + eq82_e1255_d_b52);
        let eq82_e1256_d_b53: f64 = (eq82_e1250_d_b53 + eq82_e1255_d_b53);
        let eq82_e1256_d_b54: f64 = (eq82_e1250_d_b54 + eq82_e1255_d_b54);
        (eq82_e1256, eq82_e1256_d_n0, eq82_e1256_d_n1, eq82_e1256_d_n2, eq82_e1256_d_n3, eq82_e1256_d_n4, eq82_e1256_d_n5, eq82_e1256_d_n6, eq82_e1256_d_n7, eq82_e1256_d_n8, eq82_e1256_d_n9, eq82_e1256_d_n10, eq82_e1256_d_n11, eq82_e1256_d_n12, eq82_e1256_d_n13, eq82_e1256_d_n14, eq82_e1256_d_n15, eq82_e1256_d_n16, eq82_e1256_d_n17, eq82_e1256_d_n18, eq82_e1256_d_n19, eq82_e1256_d_n20, eq82_e1256_d_n21, eq82_e1256_d_n22, eq82_e1256_d_b0, eq82_e1256_d_b1, eq82_e1256_d_b2, eq82_e1256_d_b3, eq82_e1256_d_b4, eq82_e1256_d_b5, eq82_e1256_d_b6, eq82_e1256_d_b7, eq82_e1256_d_b8, eq82_e1256_d_b9, eq82_e1256_d_b10, eq82_e1256_d_b11, eq82_e1256_d_b12, eq82_e1256_d_b13, eq82_e1256_d_b14, eq82_e1256_d_b15, eq82_e1256_d_b16, eq82_e1256_d_b17, eq82_e1256_d_b18, eq82_e1256_d_b19, eq82_e1256_d_b20, eq82_e1256_d_b21, eq82_e1256_d_b22, eq82_e1256_d_b23, eq82_e1256_d_b24, eq82_e1256_d_b25, eq82_e1256_d_b26, eq82_e1256_d_b27, eq82_e1256_d_b28, eq82_e1256_d_b29, eq82_e1256_d_b30, eq82_e1256_d_b31, eq82_e1256_d_b32, eq82_e1256_d_b33, eq82_e1256_d_b34, eq82_e1256_d_b35, eq82_e1256_d_b36, eq82_e1256_d_b37, eq82_e1256_d_b38, eq82_e1256_d_b39, eq82_e1256_d_b40, eq82_e1256_d_b41, eq82_e1256_d_b42, eq82_e1256_d_b43, eq82_e1256_d_b44, eq82_e1256_d_b45, eq82_e1256_d_b46, eq82_e1256_d_b47, eq82_e1256_d_b48, eq82_e1256_d_b49, eq82_e1256_d_b50, eq82_e1256_d_b51, eq82_e1256_d_b52, eq82_e1256_d_b53, eq82_e1256_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1258;
        let eq82_node_derivatives: [f64; 23] = [eq82_e1258_d_n0, eq82_e1258_d_n1, eq82_e1258_d_n2, eq82_e1258_d_n3, eq82_e1258_d_n4, eq82_e1258_d_n5, eq82_e1258_d_n6, eq82_e1258_d_n7, eq82_e1258_d_n8, eq82_e1258_d_n9, eq82_e1258_d_n10, eq82_e1258_d_n11, eq82_e1258_d_n12, eq82_e1258_d_n13, eq82_e1258_d_n14, eq82_e1258_d_n15, eq82_e1258_d_n16, eq82_e1258_d_n17, eq82_e1258_d_n18, eq82_e1258_d_n19, eq82_e1258_d_n20, eq82_e1258_d_n21, eq82_e1258_d_n22];
        let eq82_branch_derivatives: [f64; 55] = [eq82_e1258_d_b0, eq82_e1258_d_b1, eq82_e1258_d_b2, eq82_e1258_d_b3, eq82_e1258_d_b4, eq82_e1258_d_b5, eq82_e1258_d_b6, eq82_e1258_d_b7, eq82_e1258_d_b8, eq82_e1258_d_b9, eq82_e1258_d_b10, eq82_e1258_d_b11, eq82_e1258_d_b12, eq82_e1258_d_b13, eq82_e1258_d_b14, eq82_e1258_d_b15, eq82_e1258_d_b16, eq82_e1258_d_b17, eq82_e1258_d_b18, eq82_e1258_d_b19, eq82_e1258_d_b20, eq82_e1258_d_b21, eq82_e1258_d_b22, eq82_e1258_d_b23, eq82_e1258_d_b24, eq82_e1258_d_b25, eq82_e1258_d_b26, eq82_e1258_d_b27, eq82_e1258_d_b28, eq82_e1258_d_b29, eq82_e1258_d_b30, eq82_e1258_d_b31, eq82_e1258_d_b32, eq82_e1258_d_b33, eq82_e1258_d_b34, eq82_e1258_d_b35, eq82_e1258_d_b36, eq82_e1258_d_b37, eq82_e1258_d_b38, eq82_e1258_d_b39, eq82_e1258_d_b40, eq82_e1258_d_b41, eq82_e1258_d_b42, eq82_e1258_d_b43, eq82_e1258_d_b44, eq82_e1258_d_b45, eq82_e1258_d_b46, eq82_e1258_d_b47, eq82_e1258_d_b48, eq82_e1258_d_b49, eq82_e1258_d_b50, eq82_e1258_d_b51, eq82_e1258_d_b52, eq82_e1258_d_b53, eq82_e1258_d_b54];
        stamper.stamp_current_dense_local(
            Some(19),
            Some(20),
            multiplicity * (eq82_value),
            &eq82_node_derivatives,
            &eq82_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_9(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq86_e1294, eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22, eq86_e1294_d_b0, eq86_e1294_d_b1, eq86_e1294_d_b2, eq86_e1294_d_b3, eq86_e1294_d_b4, eq86_e1294_d_b5, eq86_e1294_d_b6, eq86_e1294_d_b7, eq86_e1294_d_b8, eq86_e1294_d_b9, eq86_e1294_d_b10, eq86_e1294_d_b11, eq86_e1294_d_b12, eq86_e1294_d_b13, eq86_e1294_d_b14, eq86_e1294_d_b15, eq86_e1294_d_b16, eq86_e1294_d_b17, eq86_e1294_d_b18, eq86_e1294_d_b19, eq86_e1294_d_b20, eq86_e1294_d_b21, eq86_e1294_d_b22, eq86_e1294_d_b23, eq86_e1294_d_b24, eq86_e1294_d_b25, eq86_e1294_d_b26, eq86_e1294_d_b27, eq86_e1294_d_b28, eq86_e1294_d_b29, eq86_e1294_d_b30, eq86_e1294_d_b31, eq86_e1294_d_b32, eq86_e1294_d_b33, eq86_e1294_d_b34, eq86_e1294_d_b35, eq86_e1294_d_b36, eq86_e1294_d_b37, eq86_e1294_d_b38, eq86_e1294_d_b39, eq86_e1294_d_b40, eq86_e1294_d_b41, eq86_e1294_d_b42, eq86_e1294_d_b43, eq86_e1294_d_b44, eq86_e1294_d_b45, eq86_e1294_d_b46, eq86_e1294_d_b47, eq86_e1294_d_b48, eq86_e1294_d_b49, eq86_e1294_d_b50, eq86_e1294_d_b51, eq86_e1294_d_b52, eq86_e1294_d_b53, eq86_e1294_d_b54,) = {
    if (s.b[493] && s.b[494]) {
        let eq86_e1284: f64 = (p.p6 * s.v[64]);
        let eq86_e1286: f64 = (eq86_e1284 * s.v[281]);
        let eq86_e1286_d_n0: f64 = (((p.p6 * s.dn[64][0]) * s.v[281]) + (eq86_e1284 * s.dn[281][0]));
        let eq86_e1286_d_n1: f64 = (((p.p6 * s.dn[64][1]) * s.v[281]) + (eq86_e1284 * s.dn[281][1]));
        let eq86_e1286_d_n2: f64 = (((p.p6 * s.dn[64][2]) * s.v[281]) + (eq86_e1284 * s.dn[281][2]));
        let eq86_e1286_d_n3: f64 = (((p.p6 * s.dn[64][3]) * s.v[281]) + (eq86_e1284 * s.dn[281][3]));
        let eq86_e1286_d_n4: f64 = (((p.p6 * s.dn[64][4]) * s.v[281]) + (eq86_e1284 * s.dn[281][4]));
        let eq86_e1286_d_n5: f64 = (((p.p6 * s.dn[64][5]) * s.v[281]) + (eq86_e1284 * s.dn[281][5]));
        let eq86_e1286_d_n6: f64 = (((p.p6 * s.dn[64][6]) * s.v[281]) + (eq86_e1284 * s.dn[281][6]));
        let eq86_e1286_d_n7: f64 = (((p.p6 * s.dn[64][7]) * s.v[281]) + (eq86_e1284 * s.dn[281][7]));
        let eq86_e1286_d_n8: f64 = (((p.p6 * s.dn[64][8]) * s.v[281]) + (eq86_e1284 * s.dn[281][8]));
        let eq86_e1286_d_n9: f64 = (((p.p6 * s.dn[64][9]) * s.v[281]) + (eq86_e1284 * s.dn[281][9]));
        let eq86_e1286_d_n10: f64 = (((p.p6 * s.dn[64][10]) * s.v[281]) + (eq86_e1284 * s.dn[281][10]));
        let eq86_e1286_d_n11: f64 = (((p.p6 * s.dn[64][11]) * s.v[281]) + (eq86_e1284 * s.dn[281][11]));
        let eq86_e1286_d_n12: f64 = (((p.p6 * s.dn[64][12]) * s.v[281]) + (eq86_e1284 * s.dn[281][12]));
        let eq86_e1286_d_n13: f64 = (((p.p6 * s.dn[64][13]) * s.v[281]) + (eq86_e1284 * s.dn[281][13]));
        let eq86_e1286_d_n14: f64 = (((p.p6 * s.dn[64][14]) * s.v[281]) + (eq86_e1284 * s.dn[281][14]));
        let eq86_e1286_d_n15: f64 = (((p.p6 * s.dn[64][15]) * s.v[281]) + (eq86_e1284 * s.dn[281][15]));
        let eq86_e1286_d_n16: f64 = (((p.p6 * s.dn[64][16]) * s.v[281]) + (eq86_e1284 * s.dn[281][16]));
        let eq86_e1286_d_n17: f64 = (((p.p6 * s.dn[64][17]) * s.v[281]) + (eq86_e1284 * s.dn[281][17]));
        let eq86_e1286_d_n18: f64 = (((p.p6 * s.dn[64][18]) * s.v[281]) + (eq86_e1284 * s.dn[281][18]));
        let eq86_e1286_d_n19: f64 = (((p.p6 * s.dn[64][19]) * s.v[281]) + (eq86_e1284 * s.dn[281][19]));
        let eq86_e1286_d_n20: f64 = (((p.p6 * s.dn[64][20]) * s.v[281]) + (eq86_e1284 * s.dn[281][20]));
        let eq86_e1286_d_n21: f64 = (((p.p6 * s.dn[64][21]) * s.v[281]) + (eq86_e1284 * s.dn[281][21]));
        let eq86_e1286_d_n22: f64 = (((p.p6 * s.dn[64][22]) * s.v[281]) + (eq86_e1284 * s.dn[281][22]));
        let eq86_e1286_d_b0: f64 = (((p.p6 * s.db[64][0]) * s.v[281]) + (eq86_e1284 * s.db[281][0]));
        let eq86_e1286_d_b1: f64 = (((p.p6 * s.db[64][1]) * s.v[281]) + (eq86_e1284 * s.db[281][1]));
        let eq86_e1286_d_b2: f64 = (((p.p6 * s.db[64][2]) * s.v[281]) + (eq86_e1284 * s.db[281][2]));
        let eq86_e1286_d_b3: f64 = (((p.p6 * s.db[64][3]) * s.v[281]) + (eq86_e1284 * s.db[281][3]));
        let eq86_e1286_d_b4: f64 = (((p.p6 * s.db[64][4]) * s.v[281]) + (eq86_e1284 * s.db[281][4]));
        let eq86_e1286_d_b5: f64 = (((p.p6 * s.db[64][5]) * s.v[281]) + (eq86_e1284 * s.db[281][5]));
        let eq86_e1286_d_b6: f64 = (((p.p6 * s.db[64][6]) * s.v[281]) + (eq86_e1284 * s.db[281][6]));
        let eq86_e1286_d_b7: f64 = (((p.p6 * s.db[64][7]) * s.v[281]) + (eq86_e1284 * s.db[281][7]));
        let eq86_e1286_d_b8: f64 = (((p.p6 * s.db[64][8]) * s.v[281]) + (eq86_e1284 * s.db[281][8]));
        let eq86_e1286_d_b9: f64 = (((p.p6 * s.db[64][9]) * s.v[281]) + (eq86_e1284 * s.db[281][9]));
        let eq86_e1286_d_b10: f64 = (((p.p6 * s.db[64][10]) * s.v[281]) + (eq86_e1284 * s.db[281][10]));
        let eq86_e1286_d_b11: f64 = (((p.p6 * s.db[64][11]) * s.v[281]) + (eq86_e1284 * s.db[281][11]));
        let eq86_e1286_d_b12: f64 = (((p.p6 * s.db[64][12]) * s.v[281]) + (eq86_e1284 * s.db[281][12]));
        let eq86_e1286_d_b13: f64 = (((p.p6 * s.db[64][13]) * s.v[281]) + (eq86_e1284 * s.db[281][13]));
        let eq86_e1286_d_b14: f64 = (((p.p6 * s.db[64][14]) * s.v[281]) + (eq86_e1284 * s.db[281][14]));
        let eq86_e1286_d_b15: f64 = (((p.p6 * s.db[64][15]) * s.v[281]) + (eq86_e1284 * s.db[281][15]));
        let eq86_e1286_d_b16: f64 = (((p.p6 * s.db[64][16]) * s.v[281]) + (eq86_e1284 * s.db[281][16]));
        let eq86_e1286_d_b17: f64 = (((p.p6 * s.db[64][17]) * s.v[281]) + (eq86_e1284 * s.db[281][17]));
        let eq86_e1286_d_b18: f64 = (((p.p6 * s.db[64][18]) * s.v[281]) + (eq86_e1284 * s.db[281][18]));
        let eq86_e1286_d_b19: f64 = (((p.p6 * s.db[64][19]) * s.v[281]) + (eq86_e1284 * s.db[281][19]));
        let eq86_e1286_d_b20: f64 = (((p.p6 * s.db[64][20]) * s.v[281]) + (eq86_e1284 * s.db[281][20]));
        let eq86_e1286_d_b21: f64 = (((p.p6 * s.db[64][21]) * s.v[281]) + (eq86_e1284 * s.db[281][21]));
        let eq86_e1286_d_b22: f64 = (((p.p6 * s.db[64][22]) * s.v[281]) + (eq86_e1284 * s.db[281][22]));
        let eq86_e1286_d_b23: f64 = (((p.p6 * s.db[64][23]) * s.v[281]) + (eq86_e1284 * s.db[281][23]));
        let eq86_e1286_d_b24: f64 = (((p.p6 * s.db[64][24]) * s.v[281]) + (eq86_e1284 * s.db[281][24]));
        let eq86_e1286_d_b25: f64 = (((p.p6 * s.db[64][25]) * s.v[281]) + (eq86_e1284 * s.db[281][25]));
        let eq86_e1286_d_b26: f64 = (((p.p6 * s.db[64][26]) * s.v[281]) + (eq86_e1284 * s.db[281][26]));
        let eq86_e1286_d_b27: f64 = (((p.p6 * s.db[64][27]) * s.v[281]) + (eq86_e1284 * s.db[281][27]));
        let eq86_e1286_d_b28: f64 = (((p.p6 * s.db[64][28]) * s.v[281]) + (eq86_e1284 * s.db[281][28]));
        let eq86_e1286_d_b29: f64 = (((p.p6 * s.db[64][29]) * s.v[281]) + (eq86_e1284 * s.db[281][29]));
        let eq86_e1286_d_b30: f64 = (((p.p6 * s.db[64][30]) * s.v[281]) + (eq86_e1284 * s.db[281][30]));
        let eq86_e1286_d_b31: f64 = (((p.p6 * s.db[64][31]) * s.v[281]) + (eq86_e1284 * s.db[281][31]));
        let eq86_e1286_d_b32: f64 = (((p.p6 * s.db[64][32]) * s.v[281]) + (eq86_e1284 * s.db[281][32]));
        let eq86_e1286_d_b33: f64 = (((p.p6 * s.db[64][33]) * s.v[281]) + (eq86_e1284 * s.db[281][33]));
        let eq86_e1286_d_b34: f64 = (((p.p6 * s.db[64][34]) * s.v[281]) + (eq86_e1284 * s.db[281][34]));
        let eq86_e1286_d_b35: f64 = (((p.p6 * s.db[64][35]) * s.v[281]) + (eq86_e1284 * s.db[281][35]));
        let eq86_e1286_d_b36: f64 = (((p.p6 * s.db[64][36]) * s.v[281]) + (eq86_e1284 * s.db[281][36]));
        let eq86_e1286_d_b37: f64 = (((p.p6 * s.db[64][37]) * s.v[281]) + (eq86_e1284 * s.db[281][37]));
        let eq86_e1286_d_b38: f64 = (((p.p6 * s.db[64][38]) * s.v[281]) + (eq86_e1284 * s.db[281][38]));
        let eq86_e1286_d_b39: f64 = (((p.p6 * s.db[64][39]) * s.v[281]) + (eq86_e1284 * s.db[281][39]));
        let eq86_e1286_d_b40: f64 = (((p.p6 * s.db[64][40]) * s.v[281]) + (eq86_e1284 * s.db[281][40]));
        let eq86_e1286_d_b41: f64 = (((p.p6 * s.db[64][41]) * s.v[281]) + (eq86_e1284 * s.db[281][41]));
        let eq86_e1286_d_b42: f64 = (((p.p6 * s.db[64][42]) * s.v[281]) + (eq86_e1284 * s.db[281][42]));
        let eq86_e1286_d_b43: f64 = (((p.p6 * s.db[64][43]) * s.v[281]) + (eq86_e1284 * s.db[281][43]));
        let eq86_e1286_d_b44: f64 = (((p.p6 * s.db[64][44]) * s.v[281]) + (eq86_e1284 * s.db[281][44]));
        let eq86_e1286_d_b45: f64 = (((p.p6 * s.db[64][45]) * s.v[281]) + (eq86_e1284 * s.db[281][45]));
        let eq86_e1286_d_b46: f64 = (((p.p6 * s.db[64][46]) * s.v[281]) + (eq86_e1284 * s.db[281][46]));
        let eq86_e1286_d_b47: f64 = (((p.p6 * s.db[64][47]) * s.v[281]) + (eq86_e1284 * s.db[281][47]));
        let eq86_e1286_d_b48: f64 = (((p.p6 * s.db[64][48]) * s.v[281]) + (eq86_e1284 * s.db[281][48]));
        let eq86_e1286_d_b49: f64 = (((p.p6 * s.db[64][49]) * s.v[281]) + (eq86_e1284 * s.db[281][49]));
        let eq86_e1286_d_b50: f64 = (((p.p6 * s.db[64][50]) * s.v[281]) + (eq86_e1284 * s.db[281][50]));
        let eq86_e1286_d_b51: f64 = (((p.p6 * s.db[64][51]) * s.v[281]) + (eq86_e1284 * s.db[281][51]));
        let eq86_e1286_d_b52: f64 = (((p.p6 * s.db[64][52]) * s.v[281]) + (eq86_e1284 * s.db[281][52]));
        let eq86_e1286_d_b53: f64 = (((p.p6 * s.db[64][53]) * s.v[281]) + (eq86_e1284 * s.db[281][53]));
        let eq86_e1286_d_b54: f64 = (((p.p6 * s.db[64][54]) * s.v[281]) + (eq86_e1284 * s.db[281][54]));
        let eq86_e1289: f64 = (p.p6 * s.v[379]);
        let eq86_e1291: f64 = (eq86_e1289 * (nv17 - nv16));
        let eq86_e1291_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv17 - nv16));
        let eq86_e1291_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv17 - nv16));
        let eq86_e1291_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv17 - nv16));
        let eq86_e1291_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv17 - nv16));
        let eq86_e1291_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv17 - nv16));
        let eq86_e1291_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv17 - nv16));
        let eq86_e1291_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv17 - nv16));
        let eq86_e1291_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv17 - nv16));
        let eq86_e1291_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv17 - nv16));
        let eq86_e1291_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv17 - nv16));
        let eq86_e1291_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv17 - nv16));
        let eq86_e1291_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv17 - nv16));
        let eq86_e1291_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv17 - nv16));
        let eq86_e1291_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv17 - nv16));
        let eq86_e1291_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv17 - nv16));
        let eq86_e1291_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv17 - nv16));
        let eq86_e1291_d_n16: f64 = (((p.p6 * s.dn[379][16]) * (nv17 - nv16)) + (-eq86_e1289));
        let eq86_e1291_d_n17: f64 = (((p.p6 * s.dn[379][17]) * (nv17 - nv16)) + eq86_e1289);
        let eq86_e1291_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv17 - nv16));
        let eq86_e1291_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv17 - nv16));
        let eq86_e1291_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv17 - nv16));
        let eq86_e1291_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv17 - nv16));
        let eq86_e1291_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv17 - nv16));
        let eq86_e1291_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv17 - nv16));
        let eq86_e1291_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv17 - nv16));
        let eq86_e1291_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv17 - nv16));
        let eq86_e1291_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv17 - nv16));
        let eq86_e1291_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv17 - nv16));
        let eq86_e1291_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv17 - nv16));
        let eq86_e1291_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv17 - nv16));
        let eq86_e1291_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv17 - nv16));
        let eq86_e1291_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv17 - nv16));
        let eq86_e1291_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv17 - nv16));
        let eq86_e1291_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv17 - nv16));
        let eq86_e1291_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv17 - nv16));
        let eq86_e1291_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv17 - nv16));
        let eq86_e1291_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv17 - nv16));
        let eq86_e1291_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv17 - nv16));
        let eq86_e1291_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv17 - nv16));
        let eq86_e1291_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv17 - nv16));
        let eq86_e1291_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv17 - nv16));
        let eq86_e1291_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv17 - nv16));
        let eq86_e1291_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv17 - nv16));
        let eq86_e1291_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv17 - nv16));
        let eq86_e1291_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv17 - nv16));
        let eq86_e1291_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv17 - nv16));
        let eq86_e1291_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv17 - nv16));
        let eq86_e1291_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv17 - nv16));
        let eq86_e1291_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv17 - nv16));
        let eq86_e1291_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv17 - nv16));
        let eq86_e1291_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv17 - nv16));
        let eq86_e1291_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv17 - nv16));
        let eq86_e1291_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv17 - nv16));
        let eq86_e1291_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv17 - nv16));
        let eq86_e1291_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv17 - nv16));
        let eq86_e1291_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv17 - nv16));
        let eq86_e1291_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv17 - nv16));
        let eq86_e1291_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv17 - nv16));
        let eq86_e1291_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv17 - nv16));
        let eq86_e1291_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv17 - nv16));
        let eq86_e1291_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv17 - nv16));
        let eq86_e1291_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv17 - nv16));
        let eq86_e1291_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv17 - nv16));
        let eq86_e1291_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv17 - nv16));
        let eq86_e1291_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv17 - nv16));
        let eq86_e1291_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv17 - nv16));
        let eq86_e1291_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv17 - nv16));
        let eq86_e1291_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv17 - nv16));
        let eq86_e1291_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv17 - nv16));
        let eq86_e1291_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv17 - nv16));
        let eq86_e1291_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv17 - nv16));
        let eq86_e1291_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv17 - nv16));
        let eq86_e1291_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv17 - nv16));
        let eq86_e1291_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv17 - nv16));
        let eq86_e1291_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv17 - nv16));
        let eq86_e1291_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv17 - nv16));
        let eq86_e1291_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv17 - nv16));
        let eq86_e1291_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv17 - nv16));
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
        let eq86_e1292_d_b0: f64 = (eq86_e1286_d_b0 + eq86_e1291_d_b0);
        let eq86_e1292_d_b1: f64 = (eq86_e1286_d_b1 + eq86_e1291_d_b1);
        let eq86_e1292_d_b2: f64 = (eq86_e1286_d_b2 + eq86_e1291_d_b2);
        let eq86_e1292_d_b3: f64 = (eq86_e1286_d_b3 + eq86_e1291_d_b3);
        let eq86_e1292_d_b4: f64 = (eq86_e1286_d_b4 + eq86_e1291_d_b4);
        let eq86_e1292_d_b5: f64 = (eq86_e1286_d_b5 + eq86_e1291_d_b5);
        let eq86_e1292_d_b6: f64 = (eq86_e1286_d_b6 + eq86_e1291_d_b6);
        let eq86_e1292_d_b7: f64 = (eq86_e1286_d_b7 + eq86_e1291_d_b7);
        let eq86_e1292_d_b8: f64 = (eq86_e1286_d_b8 + eq86_e1291_d_b8);
        let eq86_e1292_d_b9: f64 = (eq86_e1286_d_b9 + eq86_e1291_d_b9);
        let eq86_e1292_d_b10: f64 = (eq86_e1286_d_b10 + eq86_e1291_d_b10);
        let eq86_e1292_d_b11: f64 = (eq86_e1286_d_b11 + eq86_e1291_d_b11);
        let eq86_e1292_d_b12: f64 = (eq86_e1286_d_b12 + eq86_e1291_d_b12);
        let eq86_e1292_d_b13: f64 = (eq86_e1286_d_b13 + eq86_e1291_d_b13);
        let eq86_e1292_d_b14: f64 = (eq86_e1286_d_b14 + eq86_e1291_d_b14);
        let eq86_e1292_d_b15: f64 = (eq86_e1286_d_b15 + eq86_e1291_d_b15);
        let eq86_e1292_d_b16: f64 = (eq86_e1286_d_b16 + eq86_e1291_d_b16);
        let eq86_e1292_d_b17: f64 = (eq86_e1286_d_b17 + eq86_e1291_d_b17);
        let eq86_e1292_d_b18: f64 = (eq86_e1286_d_b18 + eq86_e1291_d_b18);
        let eq86_e1292_d_b19: f64 = (eq86_e1286_d_b19 + eq86_e1291_d_b19);
        let eq86_e1292_d_b20: f64 = (eq86_e1286_d_b20 + eq86_e1291_d_b20);
        let eq86_e1292_d_b21: f64 = (eq86_e1286_d_b21 + eq86_e1291_d_b21);
        let eq86_e1292_d_b22: f64 = (eq86_e1286_d_b22 + eq86_e1291_d_b22);
        let eq86_e1292_d_b23: f64 = (eq86_e1286_d_b23 + eq86_e1291_d_b23);
        let eq86_e1292_d_b24: f64 = (eq86_e1286_d_b24 + eq86_e1291_d_b24);
        let eq86_e1292_d_b25: f64 = (eq86_e1286_d_b25 + eq86_e1291_d_b25);
        let eq86_e1292_d_b26: f64 = (eq86_e1286_d_b26 + eq86_e1291_d_b26);
        let eq86_e1292_d_b27: f64 = (eq86_e1286_d_b27 + eq86_e1291_d_b27);
        let eq86_e1292_d_b28: f64 = (eq86_e1286_d_b28 + eq86_e1291_d_b28);
        let eq86_e1292_d_b29: f64 = (eq86_e1286_d_b29 + eq86_e1291_d_b29);
        let eq86_e1292_d_b30: f64 = (eq86_e1286_d_b30 + eq86_e1291_d_b30);
        let eq86_e1292_d_b31: f64 = (eq86_e1286_d_b31 + eq86_e1291_d_b31);
        let eq86_e1292_d_b32: f64 = (eq86_e1286_d_b32 + eq86_e1291_d_b32);
        let eq86_e1292_d_b33: f64 = (eq86_e1286_d_b33 + eq86_e1291_d_b33);
        let eq86_e1292_d_b34: f64 = (eq86_e1286_d_b34 + eq86_e1291_d_b34);
        let eq86_e1292_d_b35: f64 = (eq86_e1286_d_b35 + eq86_e1291_d_b35);
        let eq86_e1292_d_b36: f64 = (eq86_e1286_d_b36 + eq86_e1291_d_b36);
        let eq86_e1292_d_b37: f64 = (eq86_e1286_d_b37 + eq86_e1291_d_b37);
        let eq86_e1292_d_b38: f64 = (eq86_e1286_d_b38 + eq86_e1291_d_b38);
        let eq86_e1292_d_b39: f64 = (eq86_e1286_d_b39 + eq86_e1291_d_b39);
        let eq86_e1292_d_b40: f64 = (eq86_e1286_d_b40 + eq86_e1291_d_b40);
        let eq86_e1292_d_b41: f64 = (eq86_e1286_d_b41 + eq86_e1291_d_b41);
        let eq86_e1292_d_b42: f64 = (eq86_e1286_d_b42 + eq86_e1291_d_b42);
        let eq86_e1292_d_b43: f64 = (eq86_e1286_d_b43 + eq86_e1291_d_b43);
        let eq86_e1292_d_b44: f64 = (eq86_e1286_d_b44 + eq86_e1291_d_b44);
        let eq86_e1292_d_b45: f64 = (eq86_e1286_d_b45 + eq86_e1291_d_b45);
        let eq86_e1292_d_b46: f64 = (eq86_e1286_d_b46 + eq86_e1291_d_b46);
        let eq86_e1292_d_b47: f64 = (eq86_e1286_d_b47 + eq86_e1291_d_b47);
        let eq86_e1292_d_b48: f64 = (eq86_e1286_d_b48 + eq86_e1291_d_b48);
        let eq86_e1292_d_b49: f64 = (eq86_e1286_d_b49 + eq86_e1291_d_b49);
        let eq86_e1292_d_b50: f64 = (eq86_e1286_d_b50 + eq86_e1291_d_b50);
        let eq86_e1292_d_b51: f64 = (eq86_e1286_d_b51 + eq86_e1291_d_b51);
        let eq86_e1292_d_b52: f64 = (eq86_e1286_d_b52 + eq86_e1291_d_b52);
        let eq86_e1292_d_b53: f64 = (eq86_e1286_d_b53 + eq86_e1291_d_b53);
        let eq86_e1292_d_b54: f64 = (eq86_e1286_d_b54 + eq86_e1291_d_b54);
        (eq86_e1292, eq86_e1292_d_n0, eq86_e1292_d_n1, eq86_e1292_d_n2, eq86_e1292_d_n3, eq86_e1292_d_n4, eq86_e1292_d_n5, eq86_e1292_d_n6, eq86_e1292_d_n7, eq86_e1292_d_n8, eq86_e1292_d_n9, eq86_e1292_d_n10, eq86_e1292_d_n11, eq86_e1292_d_n12, eq86_e1292_d_n13, eq86_e1292_d_n14, eq86_e1292_d_n15, eq86_e1292_d_n16, eq86_e1292_d_n17, eq86_e1292_d_n18, eq86_e1292_d_n19, eq86_e1292_d_n20, eq86_e1292_d_n21, eq86_e1292_d_n22, eq86_e1292_d_b0, eq86_e1292_d_b1, eq86_e1292_d_b2, eq86_e1292_d_b3, eq86_e1292_d_b4, eq86_e1292_d_b5, eq86_e1292_d_b6, eq86_e1292_d_b7, eq86_e1292_d_b8, eq86_e1292_d_b9, eq86_e1292_d_b10, eq86_e1292_d_b11, eq86_e1292_d_b12, eq86_e1292_d_b13, eq86_e1292_d_b14, eq86_e1292_d_b15, eq86_e1292_d_b16, eq86_e1292_d_b17, eq86_e1292_d_b18, eq86_e1292_d_b19, eq86_e1292_d_b20, eq86_e1292_d_b21, eq86_e1292_d_b22, eq86_e1292_d_b23, eq86_e1292_d_b24, eq86_e1292_d_b25, eq86_e1292_d_b26, eq86_e1292_d_b27, eq86_e1292_d_b28, eq86_e1292_d_b29, eq86_e1292_d_b30, eq86_e1292_d_b31, eq86_e1292_d_b32, eq86_e1292_d_b33, eq86_e1292_d_b34, eq86_e1292_d_b35, eq86_e1292_d_b36, eq86_e1292_d_b37, eq86_e1292_d_b38, eq86_e1292_d_b39, eq86_e1292_d_b40, eq86_e1292_d_b41, eq86_e1292_d_b42, eq86_e1292_d_b43, eq86_e1292_d_b44, eq86_e1292_d_b45, eq86_e1292_d_b46, eq86_e1292_d_b47, eq86_e1292_d_b48, eq86_e1292_d_b49, eq86_e1292_d_b50, eq86_e1292_d_b51, eq86_e1292_d_b52, eq86_e1292_d_b53, eq86_e1292_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1294;
        let eq86_node_derivatives: [f64; 23] = [eq86_e1294_d_n0, eq86_e1294_d_n1, eq86_e1294_d_n2, eq86_e1294_d_n3, eq86_e1294_d_n4, eq86_e1294_d_n5, eq86_e1294_d_n6, eq86_e1294_d_n7, eq86_e1294_d_n8, eq86_e1294_d_n9, eq86_e1294_d_n10, eq86_e1294_d_n11, eq86_e1294_d_n12, eq86_e1294_d_n13, eq86_e1294_d_n14, eq86_e1294_d_n15, eq86_e1294_d_n16, eq86_e1294_d_n17, eq86_e1294_d_n18, eq86_e1294_d_n19, eq86_e1294_d_n20, eq86_e1294_d_n21, eq86_e1294_d_n22];
        let eq86_branch_derivatives: [f64; 55] = [eq86_e1294_d_b0, eq86_e1294_d_b1, eq86_e1294_d_b2, eq86_e1294_d_b3, eq86_e1294_d_b4, eq86_e1294_d_b5, eq86_e1294_d_b6, eq86_e1294_d_b7, eq86_e1294_d_b8, eq86_e1294_d_b9, eq86_e1294_d_b10, eq86_e1294_d_b11, eq86_e1294_d_b12, eq86_e1294_d_b13, eq86_e1294_d_b14, eq86_e1294_d_b15, eq86_e1294_d_b16, eq86_e1294_d_b17, eq86_e1294_d_b18, eq86_e1294_d_b19, eq86_e1294_d_b20, eq86_e1294_d_b21, eq86_e1294_d_b22, eq86_e1294_d_b23, eq86_e1294_d_b24, eq86_e1294_d_b25, eq86_e1294_d_b26, eq86_e1294_d_b27, eq86_e1294_d_b28, eq86_e1294_d_b29, eq86_e1294_d_b30, eq86_e1294_d_b31, eq86_e1294_d_b32, eq86_e1294_d_b33, eq86_e1294_d_b34, eq86_e1294_d_b35, eq86_e1294_d_b36, eq86_e1294_d_b37, eq86_e1294_d_b38, eq86_e1294_d_b39, eq86_e1294_d_b40, eq86_e1294_d_b41, eq86_e1294_d_b42, eq86_e1294_d_b43, eq86_e1294_d_b44, eq86_e1294_d_b45, eq86_e1294_d_b46, eq86_e1294_d_b47, eq86_e1294_d_b48, eq86_e1294_d_b49, eq86_e1294_d_b50, eq86_e1294_d_b51, eq86_e1294_d_b52, eq86_e1294_d_b53, eq86_e1294_d_b54];
        stamper.stamp_current_dense_local(
            Some(17),
            Some(16),
            multiplicity * (eq86_value),
            &eq86_node_derivatives,
            &eq86_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq89_e1322, eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22, eq89_e1322_d_b0, eq89_e1322_d_b1, eq89_e1322_d_b2, eq89_e1322_d_b3, eq89_e1322_d_b4, eq89_e1322_d_b5, eq89_e1322_d_b6, eq89_e1322_d_b7, eq89_e1322_d_b8, eq89_e1322_d_b9, eq89_e1322_d_b10, eq89_e1322_d_b11, eq89_e1322_d_b12, eq89_e1322_d_b13, eq89_e1322_d_b14, eq89_e1322_d_b15, eq89_e1322_d_b16, eq89_e1322_d_b17, eq89_e1322_d_b18, eq89_e1322_d_b19, eq89_e1322_d_b20, eq89_e1322_d_b21, eq89_e1322_d_b22, eq89_e1322_d_b23, eq89_e1322_d_b24, eq89_e1322_d_b25, eq89_e1322_d_b26, eq89_e1322_d_b27, eq89_e1322_d_b28, eq89_e1322_d_b29, eq89_e1322_d_b30, eq89_e1322_d_b31, eq89_e1322_d_b32, eq89_e1322_d_b33, eq89_e1322_d_b34, eq89_e1322_d_b35, eq89_e1322_d_b36, eq89_e1322_d_b37, eq89_e1322_d_b38, eq89_e1322_d_b39, eq89_e1322_d_b40, eq89_e1322_d_b41, eq89_e1322_d_b42, eq89_e1322_d_b43, eq89_e1322_d_b44, eq89_e1322_d_b45, eq89_e1322_d_b46, eq89_e1322_d_b47, eq89_e1322_d_b48, eq89_e1322_d_b49, eq89_e1322_d_b50, eq89_e1322_d_b51, eq89_e1322_d_b52, eq89_e1322_d_b53, eq89_e1322_d_b54,) = {
    if (s.b[508] && s.b[509]) {
        let eq89_e1312: f64 = (p.p6 * s.v[68]);
        let eq89_e1314: f64 = (eq89_e1312 * s.v[293]);
        let eq89_e1314_d_n0: f64 = (((p.p6 * s.dn[68][0]) * s.v[293]) + (eq89_e1312 * s.dn[293][0]));
        let eq89_e1314_d_n1: f64 = (((p.p6 * s.dn[68][1]) * s.v[293]) + (eq89_e1312 * s.dn[293][1]));
        let eq89_e1314_d_n2: f64 = (((p.p6 * s.dn[68][2]) * s.v[293]) + (eq89_e1312 * s.dn[293][2]));
        let eq89_e1314_d_n3: f64 = (((p.p6 * s.dn[68][3]) * s.v[293]) + (eq89_e1312 * s.dn[293][3]));
        let eq89_e1314_d_n4: f64 = (((p.p6 * s.dn[68][4]) * s.v[293]) + (eq89_e1312 * s.dn[293][4]));
        let eq89_e1314_d_n5: f64 = (((p.p6 * s.dn[68][5]) * s.v[293]) + (eq89_e1312 * s.dn[293][5]));
        let eq89_e1314_d_n6: f64 = (((p.p6 * s.dn[68][6]) * s.v[293]) + (eq89_e1312 * s.dn[293][6]));
        let eq89_e1314_d_n7: f64 = (((p.p6 * s.dn[68][7]) * s.v[293]) + (eq89_e1312 * s.dn[293][7]));
        let eq89_e1314_d_n8: f64 = (((p.p6 * s.dn[68][8]) * s.v[293]) + (eq89_e1312 * s.dn[293][8]));
        let eq89_e1314_d_n9: f64 = (((p.p6 * s.dn[68][9]) * s.v[293]) + (eq89_e1312 * s.dn[293][9]));
        let eq89_e1314_d_n10: f64 = (((p.p6 * s.dn[68][10]) * s.v[293]) + (eq89_e1312 * s.dn[293][10]));
        let eq89_e1314_d_n11: f64 = (((p.p6 * s.dn[68][11]) * s.v[293]) + (eq89_e1312 * s.dn[293][11]));
        let eq89_e1314_d_n12: f64 = (((p.p6 * s.dn[68][12]) * s.v[293]) + (eq89_e1312 * s.dn[293][12]));
        let eq89_e1314_d_n13: f64 = (((p.p6 * s.dn[68][13]) * s.v[293]) + (eq89_e1312 * s.dn[293][13]));
        let eq89_e1314_d_n14: f64 = (((p.p6 * s.dn[68][14]) * s.v[293]) + (eq89_e1312 * s.dn[293][14]));
        let eq89_e1314_d_n15: f64 = (((p.p6 * s.dn[68][15]) * s.v[293]) + (eq89_e1312 * s.dn[293][15]));
        let eq89_e1314_d_n16: f64 = (((p.p6 * s.dn[68][16]) * s.v[293]) + (eq89_e1312 * s.dn[293][16]));
        let eq89_e1314_d_n17: f64 = (((p.p6 * s.dn[68][17]) * s.v[293]) + (eq89_e1312 * s.dn[293][17]));
        let eq89_e1314_d_n18: f64 = (((p.p6 * s.dn[68][18]) * s.v[293]) + (eq89_e1312 * s.dn[293][18]));
        let eq89_e1314_d_n19: f64 = (((p.p6 * s.dn[68][19]) * s.v[293]) + (eq89_e1312 * s.dn[293][19]));
        let eq89_e1314_d_n20: f64 = (((p.p6 * s.dn[68][20]) * s.v[293]) + (eq89_e1312 * s.dn[293][20]));
        let eq89_e1314_d_n21: f64 = (((p.p6 * s.dn[68][21]) * s.v[293]) + (eq89_e1312 * s.dn[293][21]));
        let eq89_e1314_d_n22: f64 = (((p.p6 * s.dn[68][22]) * s.v[293]) + (eq89_e1312 * s.dn[293][22]));
        let eq89_e1314_d_b0: f64 = (((p.p6 * s.db[68][0]) * s.v[293]) + (eq89_e1312 * s.db[293][0]));
        let eq89_e1314_d_b1: f64 = (((p.p6 * s.db[68][1]) * s.v[293]) + (eq89_e1312 * s.db[293][1]));
        let eq89_e1314_d_b2: f64 = (((p.p6 * s.db[68][2]) * s.v[293]) + (eq89_e1312 * s.db[293][2]));
        let eq89_e1314_d_b3: f64 = (((p.p6 * s.db[68][3]) * s.v[293]) + (eq89_e1312 * s.db[293][3]));
        let eq89_e1314_d_b4: f64 = (((p.p6 * s.db[68][4]) * s.v[293]) + (eq89_e1312 * s.db[293][4]));
        let eq89_e1314_d_b5: f64 = (((p.p6 * s.db[68][5]) * s.v[293]) + (eq89_e1312 * s.db[293][5]));
        let eq89_e1314_d_b6: f64 = (((p.p6 * s.db[68][6]) * s.v[293]) + (eq89_e1312 * s.db[293][6]));
        let eq89_e1314_d_b7: f64 = (((p.p6 * s.db[68][7]) * s.v[293]) + (eq89_e1312 * s.db[293][7]));
        let eq89_e1314_d_b8: f64 = (((p.p6 * s.db[68][8]) * s.v[293]) + (eq89_e1312 * s.db[293][8]));
        let eq89_e1314_d_b9: f64 = (((p.p6 * s.db[68][9]) * s.v[293]) + (eq89_e1312 * s.db[293][9]));
        let eq89_e1314_d_b10: f64 = (((p.p6 * s.db[68][10]) * s.v[293]) + (eq89_e1312 * s.db[293][10]));
        let eq89_e1314_d_b11: f64 = (((p.p6 * s.db[68][11]) * s.v[293]) + (eq89_e1312 * s.db[293][11]));
        let eq89_e1314_d_b12: f64 = (((p.p6 * s.db[68][12]) * s.v[293]) + (eq89_e1312 * s.db[293][12]));
        let eq89_e1314_d_b13: f64 = (((p.p6 * s.db[68][13]) * s.v[293]) + (eq89_e1312 * s.db[293][13]));
        let eq89_e1314_d_b14: f64 = (((p.p6 * s.db[68][14]) * s.v[293]) + (eq89_e1312 * s.db[293][14]));
        let eq89_e1314_d_b15: f64 = (((p.p6 * s.db[68][15]) * s.v[293]) + (eq89_e1312 * s.db[293][15]));
        let eq89_e1314_d_b16: f64 = (((p.p6 * s.db[68][16]) * s.v[293]) + (eq89_e1312 * s.db[293][16]));
        let eq89_e1314_d_b17: f64 = (((p.p6 * s.db[68][17]) * s.v[293]) + (eq89_e1312 * s.db[293][17]));
        let eq89_e1314_d_b18: f64 = (((p.p6 * s.db[68][18]) * s.v[293]) + (eq89_e1312 * s.db[293][18]));
        let eq89_e1314_d_b19: f64 = (((p.p6 * s.db[68][19]) * s.v[293]) + (eq89_e1312 * s.db[293][19]));
        let eq89_e1314_d_b20: f64 = (((p.p6 * s.db[68][20]) * s.v[293]) + (eq89_e1312 * s.db[293][20]));
        let eq89_e1314_d_b21: f64 = (((p.p6 * s.db[68][21]) * s.v[293]) + (eq89_e1312 * s.db[293][21]));
        let eq89_e1314_d_b22: f64 = (((p.p6 * s.db[68][22]) * s.v[293]) + (eq89_e1312 * s.db[293][22]));
        let eq89_e1314_d_b23: f64 = (((p.p6 * s.db[68][23]) * s.v[293]) + (eq89_e1312 * s.db[293][23]));
        let eq89_e1314_d_b24: f64 = (((p.p6 * s.db[68][24]) * s.v[293]) + (eq89_e1312 * s.db[293][24]));
        let eq89_e1314_d_b25: f64 = (((p.p6 * s.db[68][25]) * s.v[293]) + (eq89_e1312 * s.db[293][25]));
        let eq89_e1314_d_b26: f64 = (((p.p6 * s.db[68][26]) * s.v[293]) + (eq89_e1312 * s.db[293][26]));
        let eq89_e1314_d_b27: f64 = (((p.p6 * s.db[68][27]) * s.v[293]) + (eq89_e1312 * s.db[293][27]));
        let eq89_e1314_d_b28: f64 = (((p.p6 * s.db[68][28]) * s.v[293]) + (eq89_e1312 * s.db[293][28]));
        let eq89_e1314_d_b29: f64 = (((p.p6 * s.db[68][29]) * s.v[293]) + (eq89_e1312 * s.db[293][29]));
        let eq89_e1314_d_b30: f64 = (((p.p6 * s.db[68][30]) * s.v[293]) + (eq89_e1312 * s.db[293][30]));
        let eq89_e1314_d_b31: f64 = (((p.p6 * s.db[68][31]) * s.v[293]) + (eq89_e1312 * s.db[293][31]));
        let eq89_e1314_d_b32: f64 = (((p.p6 * s.db[68][32]) * s.v[293]) + (eq89_e1312 * s.db[293][32]));
        let eq89_e1314_d_b33: f64 = (((p.p6 * s.db[68][33]) * s.v[293]) + (eq89_e1312 * s.db[293][33]));
        let eq89_e1314_d_b34: f64 = (((p.p6 * s.db[68][34]) * s.v[293]) + (eq89_e1312 * s.db[293][34]));
        let eq89_e1314_d_b35: f64 = (((p.p6 * s.db[68][35]) * s.v[293]) + (eq89_e1312 * s.db[293][35]));
        let eq89_e1314_d_b36: f64 = (((p.p6 * s.db[68][36]) * s.v[293]) + (eq89_e1312 * s.db[293][36]));
        let eq89_e1314_d_b37: f64 = (((p.p6 * s.db[68][37]) * s.v[293]) + (eq89_e1312 * s.db[293][37]));
        let eq89_e1314_d_b38: f64 = (((p.p6 * s.db[68][38]) * s.v[293]) + (eq89_e1312 * s.db[293][38]));
        let eq89_e1314_d_b39: f64 = (((p.p6 * s.db[68][39]) * s.v[293]) + (eq89_e1312 * s.db[293][39]));
        let eq89_e1314_d_b40: f64 = (((p.p6 * s.db[68][40]) * s.v[293]) + (eq89_e1312 * s.db[293][40]));
        let eq89_e1314_d_b41: f64 = (((p.p6 * s.db[68][41]) * s.v[293]) + (eq89_e1312 * s.db[293][41]));
        let eq89_e1314_d_b42: f64 = (((p.p6 * s.db[68][42]) * s.v[293]) + (eq89_e1312 * s.db[293][42]));
        let eq89_e1314_d_b43: f64 = (((p.p6 * s.db[68][43]) * s.v[293]) + (eq89_e1312 * s.db[293][43]));
        let eq89_e1314_d_b44: f64 = (((p.p6 * s.db[68][44]) * s.v[293]) + (eq89_e1312 * s.db[293][44]));
        let eq89_e1314_d_b45: f64 = (((p.p6 * s.db[68][45]) * s.v[293]) + (eq89_e1312 * s.db[293][45]));
        let eq89_e1314_d_b46: f64 = (((p.p6 * s.db[68][46]) * s.v[293]) + (eq89_e1312 * s.db[293][46]));
        let eq89_e1314_d_b47: f64 = (((p.p6 * s.db[68][47]) * s.v[293]) + (eq89_e1312 * s.db[293][47]));
        let eq89_e1314_d_b48: f64 = (((p.p6 * s.db[68][48]) * s.v[293]) + (eq89_e1312 * s.db[293][48]));
        let eq89_e1314_d_b49: f64 = (((p.p6 * s.db[68][49]) * s.v[293]) + (eq89_e1312 * s.db[293][49]));
        let eq89_e1314_d_b50: f64 = (((p.p6 * s.db[68][50]) * s.v[293]) + (eq89_e1312 * s.db[293][50]));
        let eq89_e1314_d_b51: f64 = (((p.p6 * s.db[68][51]) * s.v[293]) + (eq89_e1312 * s.db[293][51]));
        let eq89_e1314_d_b52: f64 = (((p.p6 * s.db[68][52]) * s.v[293]) + (eq89_e1312 * s.db[293][52]));
        let eq89_e1314_d_b53: f64 = (((p.p6 * s.db[68][53]) * s.v[293]) + (eq89_e1312 * s.db[293][53]));
        let eq89_e1314_d_b54: f64 = (((p.p6 * s.db[68][54]) * s.v[293]) + (eq89_e1312 * s.db[293][54]));
        let eq89_e1317: f64 = (p.p6 * s.v[379]);
        let eq89_e1319: f64 = (eq89_e1317 * (nv20 - nv21));
        let eq89_e1319_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv20 - nv21));
        let eq89_e1319_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv20 - nv21));
        let eq89_e1319_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv20 - nv21));
        let eq89_e1319_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv20 - nv21));
        let eq89_e1319_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv20 - nv21));
        let eq89_e1319_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv20 - nv21));
        let eq89_e1319_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv20 - nv21));
        let eq89_e1319_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv20 - nv21));
        let eq89_e1319_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv20 - nv21));
        let eq89_e1319_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv20 - nv21));
        let eq89_e1319_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv20 - nv21));
        let eq89_e1319_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv20 - nv21));
        let eq89_e1319_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv20 - nv21));
        let eq89_e1319_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv20 - nv21));
        let eq89_e1319_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv20 - nv21));
        let eq89_e1319_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv20 - nv21));
        let eq89_e1319_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv20 - nv21));
        let eq89_e1319_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv20 - nv21));
        let eq89_e1319_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv20 - nv21));
        let eq89_e1319_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv20 - nv21));
        let eq89_e1319_d_n20: f64 = (((p.p6 * s.dn[379][20]) * (nv20 - nv21)) + eq89_e1317);
        let eq89_e1319_d_n21: f64 = (((p.p6 * s.dn[379][21]) * (nv20 - nv21)) + (-eq89_e1317));
        let eq89_e1319_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv20 - nv21));
        let eq89_e1319_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv20 - nv21));
        let eq89_e1319_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv20 - nv21));
        let eq89_e1319_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv20 - nv21));
        let eq89_e1319_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv20 - nv21));
        let eq89_e1319_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv20 - nv21));
        let eq89_e1319_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv20 - nv21));
        let eq89_e1319_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv20 - nv21));
        let eq89_e1319_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv20 - nv21));
        let eq89_e1319_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv20 - nv21));
        let eq89_e1319_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv20 - nv21));
        let eq89_e1319_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv20 - nv21));
        let eq89_e1319_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv20 - nv21));
        let eq89_e1319_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv20 - nv21));
        let eq89_e1319_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv20 - nv21));
        let eq89_e1319_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv20 - nv21));
        let eq89_e1319_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv20 - nv21));
        let eq89_e1319_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv20 - nv21));
        let eq89_e1319_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv20 - nv21));
        let eq89_e1319_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv20 - nv21));
        let eq89_e1319_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv20 - nv21));
        let eq89_e1319_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv20 - nv21));
        let eq89_e1319_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv20 - nv21));
        let eq89_e1319_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv20 - nv21));
        let eq89_e1319_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv20 - nv21));
        let eq89_e1319_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv20 - nv21));
        let eq89_e1319_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv20 - nv21));
        let eq89_e1319_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv20 - nv21));
        let eq89_e1319_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv20 - nv21));
        let eq89_e1319_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv20 - nv21));
        let eq89_e1319_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv20 - nv21));
        let eq89_e1319_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv20 - nv21));
        let eq89_e1319_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv20 - nv21));
        let eq89_e1319_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv20 - nv21));
        let eq89_e1319_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv20 - nv21));
        let eq89_e1319_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv20 - nv21));
        let eq89_e1319_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv20 - nv21));
        let eq89_e1319_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv20 - nv21));
        let eq89_e1319_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv20 - nv21));
        let eq89_e1319_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv20 - nv21));
        let eq89_e1319_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv20 - nv21));
        let eq89_e1319_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv20 - nv21));
        let eq89_e1319_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv20 - nv21));
        let eq89_e1319_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv20 - nv21));
        let eq89_e1319_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv20 - nv21));
        let eq89_e1319_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv20 - nv21));
        let eq89_e1319_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv20 - nv21));
        let eq89_e1319_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv20 - nv21));
        let eq89_e1319_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv20 - nv21));
        let eq89_e1319_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv20 - nv21));
        let eq89_e1319_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv20 - nv21));
        let eq89_e1319_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv20 - nv21));
        let eq89_e1319_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv20 - nv21));
        let eq89_e1319_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv20 - nv21));
        let eq89_e1319_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv20 - nv21));
        let eq89_e1319_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv20 - nv21));
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
        let eq89_e1320_d_b0: f64 = (eq89_e1314_d_b0 + eq89_e1319_d_b0);
        let eq89_e1320_d_b1: f64 = (eq89_e1314_d_b1 + eq89_e1319_d_b1);
        let eq89_e1320_d_b2: f64 = (eq89_e1314_d_b2 + eq89_e1319_d_b2);
        let eq89_e1320_d_b3: f64 = (eq89_e1314_d_b3 + eq89_e1319_d_b3);
        let eq89_e1320_d_b4: f64 = (eq89_e1314_d_b4 + eq89_e1319_d_b4);
        let eq89_e1320_d_b5: f64 = (eq89_e1314_d_b5 + eq89_e1319_d_b5);
        let eq89_e1320_d_b6: f64 = (eq89_e1314_d_b6 + eq89_e1319_d_b6);
        let eq89_e1320_d_b7: f64 = (eq89_e1314_d_b7 + eq89_e1319_d_b7);
        let eq89_e1320_d_b8: f64 = (eq89_e1314_d_b8 + eq89_e1319_d_b8);
        let eq89_e1320_d_b9: f64 = (eq89_e1314_d_b9 + eq89_e1319_d_b9);
        let eq89_e1320_d_b10: f64 = (eq89_e1314_d_b10 + eq89_e1319_d_b10);
        let eq89_e1320_d_b11: f64 = (eq89_e1314_d_b11 + eq89_e1319_d_b11);
        let eq89_e1320_d_b12: f64 = (eq89_e1314_d_b12 + eq89_e1319_d_b12);
        let eq89_e1320_d_b13: f64 = (eq89_e1314_d_b13 + eq89_e1319_d_b13);
        let eq89_e1320_d_b14: f64 = (eq89_e1314_d_b14 + eq89_e1319_d_b14);
        let eq89_e1320_d_b15: f64 = (eq89_e1314_d_b15 + eq89_e1319_d_b15);
        let eq89_e1320_d_b16: f64 = (eq89_e1314_d_b16 + eq89_e1319_d_b16);
        let eq89_e1320_d_b17: f64 = (eq89_e1314_d_b17 + eq89_e1319_d_b17);
        let eq89_e1320_d_b18: f64 = (eq89_e1314_d_b18 + eq89_e1319_d_b18);
        let eq89_e1320_d_b19: f64 = (eq89_e1314_d_b19 + eq89_e1319_d_b19);
        let eq89_e1320_d_b20: f64 = (eq89_e1314_d_b20 + eq89_e1319_d_b20);
        let eq89_e1320_d_b21: f64 = (eq89_e1314_d_b21 + eq89_e1319_d_b21);
        let eq89_e1320_d_b22: f64 = (eq89_e1314_d_b22 + eq89_e1319_d_b22);
        let eq89_e1320_d_b23: f64 = (eq89_e1314_d_b23 + eq89_e1319_d_b23);
        let eq89_e1320_d_b24: f64 = (eq89_e1314_d_b24 + eq89_e1319_d_b24);
        let eq89_e1320_d_b25: f64 = (eq89_e1314_d_b25 + eq89_e1319_d_b25);
        let eq89_e1320_d_b26: f64 = (eq89_e1314_d_b26 + eq89_e1319_d_b26);
        let eq89_e1320_d_b27: f64 = (eq89_e1314_d_b27 + eq89_e1319_d_b27);
        let eq89_e1320_d_b28: f64 = (eq89_e1314_d_b28 + eq89_e1319_d_b28);
        let eq89_e1320_d_b29: f64 = (eq89_e1314_d_b29 + eq89_e1319_d_b29);
        let eq89_e1320_d_b30: f64 = (eq89_e1314_d_b30 + eq89_e1319_d_b30);
        let eq89_e1320_d_b31: f64 = (eq89_e1314_d_b31 + eq89_e1319_d_b31);
        let eq89_e1320_d_b32: f64 = (eq89_e1314_d_b32 + eq89_e1319_d_b32);
        let eq89_e1320_d_b33: f64 = (eq89_e1314_d_b33 + eq89_e1319_d_b33);
        let eq89_e1320_d_b34: f64 = (eq89_e1314_d_b34 + eq89_e1319_d_b34);
        let eq89_e1320_d_b35: f64 = (eq89_e1314_d_b35 + eq89_e1319_d_b35);
        let eq89_e1320_d_b36: f64 = (eq89_e1314_d_b36 + eq89_e1319_d_b36);
        let eq89_e1320_d_b37: f64 = (eq89_e1314_d_b37 + eq89_e1319_d_b37);
        let eq89_e1320_d_b38: f64 = (eq89_e1314_d_b38 + eq89_e1319_d_b38);
        let eq89_e1320_d_b39: f64 = (eq89_e1314_d_b39 + eq89_e1319_d_b39);
        let eq89_e1320_d_b40: f64 = (eq89_e1314_d_b40 + eq89_e1319_d_b40);
        let eq89_e1320_d_b41: f64 = (eq89_e1314_d_b41 + eq89_e1319_d_b41);
        let eq89_e1320_d_b42: f64 = (eq89_e1314_d_b42 + eq89_e1319_d_b42);
        let eq89_e1320_d_b43: f64 = (eq89_e1314_d_b43 + eq89_e1319_d_b43);
        let eq89_e1320_d_b44: f64 = (eq89_e1314_d_b44 + eq89_e1319_d_b44);
        let eq89_e1320_d_b45: f64 = (eq89_e1314_d_b45 + eq89_e1319_d_b45);
        let eq89_e1320_d_b46: f64 = (eq89_e1314_d_b46 + eq89_e1319_d_b46);
        let eq89_e1320_d_b47: f64 = (eq89_e1314_d_b47 + eq89_e1319_d_b47);
        let eq89_e1320_d_b48: f64 = (eq89_e1314_d_b48 + eq89_e1319_d_b48);
        let eq89_e1320_d_b49: f64 = (eq89_e1314_d_b49 + eq89_e1319_d_b49);
        let eq89_e1320_d_b50: f64 = (eq89_e1314_d_b50 + eq89_e1319_d_b50);
        let eq89_e1320_d_b51: f64 = (eq89_e1314_d_b51 + eq89_e1319_d_b51);
        let eq89_e1320_d_b52: f64 = (eq89_e1314_d_b52 + eq89_e1319_d_b52);
        let eq89_e1320_d_b53: f64 = (eq89_e1314_d_b53 + eq89_e1319_d_b53);
        let eq89_e1320_d_b54: f64 = (eq89_e1314_d_b54 + eq89_e1319_d_b54);
        (eq89_e1320, eq89_e1320_d_n0, eq89_e1320_d_n1, eq89_e1320_d_n2, eq89_e1320_d_n3, eq89_e1320_d_n4, eq89_e1320_d_n5, eq89_e1320_d_n6, eq89_e1320_d_n7, eq89_e1320_d_n8, eq89_e1320_d_n9, eq89_e1320_d_n10, eq89_e1320_d_n11, eq89_e1320_d_n12, eq89_e1320_d_n13, eq89_e1320_d_n14, eq89_e1320_d_n15, eq89_e1320_d_n16, eq89_e1320_d_n17, eq89_e1320_d_n18, eq89_e1320_d_n19, eq89_e1320_d_n20, eq89_e1320_d_n21, eq89_e1320_d_n22, eq89_e1320_d_b0, eq89_e1320_d_b1, eq89_e1320_d_b2, eq89_e1320_d_b3, eq89_e1320_d_b4, eq89_e1320_d_b5, eq89_e1320_d_b6, eq89_e1320_d_b7, eq89_e1320_d_b8, eq89_e1320_d_b9, eq89_e1320_d_b10, eq89_e1320_d_b11, eq89_e1320_d_b12, eq89_e1320_d_b13, eq89_e1320_d_b14, eq89_e1320_d_b15, eq89_e1320_d_b16, eq89_e1320_d_b17, eq89_e1320_d_b18, eq89_e1320_d_b19, eq89_e1320_d_b20, eq89_e1320_d_b21, eq89_e1320_d_b22, eq89_e1320_d_b23, eq89_e1320_d_b24, eq89_e1320_d_b25, eq89_e1320_d_b26, eq89_e1320_d_b27, eq89_e1320_d_b28, eq89_e1320_d_b29, eq89_e1320_d_b30, eq89_e1320_d_b31, eq89_e1320_d_b32, eq89_e1320_d_b33, eq89_e1320_d_b34, eq89_e1320_d_b35, eq89_e1320_d_b36, eq89_e1320_d_b37, eq89_e1320_d_b38, eq89_e1320_d_b39, eq89_e1320_d_b40, eq89_e1320_d_b41, eq89_e1320_d_b42, eq89_e1320_d_b43, eq89_e1320_d_b44, eq89_e1320_d_b45, eq89_e1320_d_b46, eq89_e1320_d_b47, eq89_e1320_d_b48, eq89_e1320_d_b49, eq89_e1320_d_b50, eq89_e1320_d_b51, eq89_e1320_d_b52, eq89_e1320_d_b53, eq89_e1320_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1322;
        let eq89_node_derivatives: [f64; 23] = [eq89_e1322_d_n0, eq89_e1322_d_n1, eq89_e1322_d_n2, eq89_e1322_d_n3, eq89_e1322_d_n4, eq89_e1322_d_n5, eq89_e1322_d_n6, eq89_e1322_d_n7, eq89_e1322_d_n8, eq89_e1322_d_n9, eq89_e1322_d_n10, eq89_e1322_d_n11, eq89_e1322_d_n12, eq89_e1322_d_n13, eq89_e1322_d_n14, eq89_e1322_d_n15, eq89_e1322_d_n16, eq89_e1322_d_n17, eq89_e1322_d_n18, eq89_e1322_d_n19, eq89_e1322_d_n20, eq89_e1322_d_n21, eq89_e1322_d_n22];
        let eq89_branch_derivatives: [f64; 55] = [eq89_e1322_d_b0, eq89_e1322_d_b1, eq89_e1322_d_b2, eq89_e1322_d_b3, eq89_e1322_d_b4, eq89_e1322_d_b5, eq89_e1322_d_b6, eq89_e1322_d_b7, eq89_e1322_d_b8, eq89_e1322_d_b9, eq89_e1322_d_b10, eq89_e1322_d_b11, eq89_e1322_d_b12, eq89_e1322_d_b13, eq89_e1322_d_b14, eq89_e1322_d_b15, eq89_e1322_d_b16, eq89_e1322_d_b17, eq89_e1322_d_b18, eq89_e1322_d_b19, eq89_e1322_d_b20, eq89_e1322_d_b21, eq89_e1322_d_b22, eq89_e1322_d_b23, eq89_e1322_d_b24, eq89_e1322_d_b25, eq89_e1322_d_b26, eq89_e1322_d_b27, eq89_e1322_d_b28, eq89_e1322_d_b29, eq89_e1322_d_b30, eq89_e1322_d_b31, eq89_e1322_d_b32, eq89_e1322_d_b33, eq89_e1322_d_b34, eq89_e1322_d_b35, eq89_e1322_d_b36, eq89_e1322_d_b37, eq89_e1322_d_b38, eq89_e1322_d_b39, eq89_e1322_d_b40, eq89_e1322_d_b41, eq89_e1322_d_b42, eq89_e1322_d_b43, eq89_e1322_d_b44, eq89_e1322_d_b45, eq89_e1322_d_b46, eq89_e1322_d_b47, eq89_e1322_d_b48, eq89_e1322_d_b49, eq89_e1322_d_b50, eq89_e1322_d_b51, eq89_e1322_d_b52, eq89_e1322_d_b53, eq89_e1322_d_b54];
        stamper.stamp_current_dense_local(
            Some(20),
            Some(21),
            multiplicity * (eq89_value),
            &eq89_node_derivatives,
            &eq89_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq93_e1358, eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22, eq93_e1358_d_b0, eq93_e1358_d_b1, eq93_e1358_d_b2, eq93_e1358_d_b3, eq93_e1358_d_b4, eq93_e1358_d_b5, eq93_e1358_d_b6, eq93_e1358_d_b7, eq93_e1358_d_b8, eq93_e1358_d_b9, eq93_e1358_d_b10, eq93_e1358_d_b11, eq93_e1358_d_b12, eq93_e1358_d_b13, eq93_e1358_d_b14, eq93_e1358_d_b15, eq93_e1358_d_b16, eq93_e1358_d_b17, eq93_e1358_d_b18, eq93_e1358_d_b19, eq93_e1358_d_b20, eq93_e1358_d_b21, eq93_e1358_d_b22, eq93_e1358_d_b23, eq93_e1358_d_b24, eq93_e1358_d_b25, eq93_e1358_d_b26, eq93_e1358_d_b27, eq93_e1358_d_b28, eq93_e1358_d_b29, eq93_e1358_d_b30, eq93_e1358_d_b31, eq93_e1358_d_b32, eq93_e1358_d_b33, eq93_e1358_d_b34, eq93_e1358_d_b35, eq93_e1358_d_b36, eq93_e1358_d_b37, eq93_e1358_d_b38, eq93_e1358_d_b39, eq93_e1358_d_b40, eq93_e1358_d_b41, eq93_e1358_d_b42, eq93_e1358_d_b43, eq93_e1358_d_b44, eq93_e1358_d_b45, eq93_e1358_d_b46, eq93_e1358_d_b47, eq93_e1358_d_b48, eq93_e1358_d_b49, eq93_e1358_d_b50, eq93_e1358_d_b51, eq93_e1358_d_b52, eq93_e1358_d_b53, eq93_e1358_d_b54,) = {
    if (s.b[523] && s.b[524]) {
        let eq93_e1348: f64 = (p.p6 * s.v[72]);
        let eq93_e1350: f64 = (eq93_e1348 * s.v[305]);
        let eq93_e1350_d_n0: f64 = (((p.p6 * s.dn[72][0]) * s.v[305]) + (eq93_e1348 * s.dn[305][0]));
        let eq93_e1350_d_n1: f64 = (((p.p6 * s.dn[72][1]) * s.v[305]) + (eq93_e1348 * s.dn[305][1]));
        let eq93_e1350_d_n2: f64 = (((p.p6 * s.dn[72][2]) * s.v[305]) + (eq93_e1348 * s.dn[305][2]));
        let eq93_e1350_d_n3: f64 = (((p.p6 * s.dn[72][3]) * s.v[305]) + (eq93_e1348 * s.dn[305][3]));
        let eq93_e1350_d_n4: f64 = (((p.p6 * s.dn[72][4]) * s.v[305]) + (eq93_e1348 * s.dn[305][4]));
        let eq93_e1350_d_n5: f64 = (((p.p6 * s.dn[72][5]) * s.v[305]) + (eq93_e1348 * s.dn[305][5]));
        let eq93_e1350_d_n6: f64 = (((p.p6 * s.dn[72][6]) * s.v[305]) + (eq93_e1348 * s.dn[305][6]));
        let eq93_e1350_d_n7: f64 = (((p.p6 * s.dn[72][7]) * s.v[305]) + (eq93_e1348 * s.dn[305][7]));
        let eq93_e1350_d_n8: f64 = (((p.p6 * s.dn[72][8]) * s.v[305]) + (eq93_e1348 * s.dn[305][8]));
        let eq93_e1350_d_n9: f64 = (((p.p6 * s.dn[72][9]) * s.v[305]) + (eq93_e1348 * s.dn[305][9]));
        let eq93_e1350_d_n10: f64 = (((p.p6 * s.dn[72][10]) * s.v[305]) + (eq93_e1348 * s.dn[305][10]));
        let eq93_e1350_d_n11: f64 = (((p.p6 * s.dn[72][11]) * s.v[305]) + (eq93_e1348 * s.dn[305][11]));
        let eq93_e1350_d_n12: f64 = (((p.p6 * s.dn[72][12]) * s.v[305]) + (eq93_e1348 * s.dn[305][12]));
        let eq93_e1350_d_n13: f64 = (((p.p6 * s.dn[72][13]) * s.v[305]) + (eq93_e1348 * s.dn[305][13]));
        let eq93_e1350_d_n14: f64 = (((p.p6 * s.dn[72][14]) * s.v[305]) + (eq93_e1348 * s.dn[305][14]));
        let eq93_e1350_d_n15: f64 = (((p.p6 * s.dn[72][15]) * s.v[305]) + (eq93_e1348 * s.dn[305][15]));
        let eq93_e1350_d_n16: f64 = (((p.p6 * s.dn[72][16]) * s.v[305]) + (eq93_e1348 * s.dn[305][16]));
        let eq93_e1350_d_n17: f64 = (((p.p6 * s.dn[72][17]) * s.v[305]) + (eq93_e1348 * s.dn[305][17]));
        let eq93_e1350_d_n18: f64 = (((p.p6 * s.dn[72][18]) * s.v[305]) + (eq93_e1348 * s.dn[305][18]));
        let eq93_e1350_d_n19: f64 = (((p.p6 * s.dn[72][19]) * s.v[305]) + (eq93_e1348 * s.dn[305][19]));
        let eq93_e1350_d_n20: f64 = (((p.p6 * s.dn[72][20]) * s.v[305]) + (eq93_e1348 * s.dn[305][20]));
        let eq93_e1350_d_n21: f64 = (((p.p6 * s.dn[72][21]) * s.v[305]) + (eq93_e1348 * s.dn[305][21]));
        let eq93_e1350_d_n22: f64 = (((p.p6 * s.dn[72][22]) * s.v[305]) + (eq93_e1348 * s.dn[305][22]));
        let eq93_e1350_d_b0: f64 = (((p.p6 * s.db[72][0]) * s.v[305]) + (eq93_e1348 * s.db[305][0]));
        let eq93_e1350_d_b1: f64 = (((p.p6 * s.db[72][1]) * s.v[305]) + (eq93_e1348 * s.db[305][1]));
        let eq93_e1350_d_b2: f64 = (((p.p6 * s.db[72][2]) * s.v[305]) + (eq93_e1348 * s.db[305][2]));
        let eq93_e1350_d_b3: f64 = (((p.p6 * s.db[72][3]) * s.v[305]) + (eq93_e1348 * s.db[305][3]));
        let eq93_e1350_d_b4: f64 = (((p.p6 * s.db[72][4]) * s.v[305]) + (eq93_e1348 * s.db[305][4]));
        let eq93_e1350_d_b5: f64 = (((p.p6 * s.db[72][5]) * s.v[305]) + (eq93_e1348 * s.db[305][5]));
        let eq93_e1350_d_b6: f64 = (((p.p6 * s.db[72][6]) * s.v[305]) + (eq93_e1348 * s.db[305][6]));
        let eq93_e1350_d_b7: f64 = (((p.p6 * s.db[72][7]) * s.v[305]) + (eq93_e1348 * s.db[305][7]));
        let eq93_e1350_d_b8: f64 = (((p.p6 * s.db[72][8]) * s.v[305]) + (eq93_e1348 * s.db[305][8]));
        let eq93_e1350_d_b9: f64 = (((p.p6 * s.db[72][9]) * s.v[305]) + (eq93_e1348 * s.db[305][9]));
        let eq93_e1350_d_b10: f64 = (((p.p6 * s.db[72][10]) * s.v[305]) + (eq93_e1348 * s.db[305][10]));
        let eq93_e1350_d_b11: f64 = (((p.p6 * s.db[72][11]) * s.v[305]) + (eq93_e1348 * s.db[305][11]));
        let eq93_e1350_d_b12: f64 = (((p.p6 * s.db[72][12]) * s.v[305]) + (eq93_e1348 * s.db[305][12]));
        let eq93_e1350_d_b13: f64 = (((p.p6 * s.db[72][13]) * s.v[305]) + (eq93_e1348 * s.db[305][13]));
        let eq93_e1350_d_b14: f64 = (((p.p6 * s.db[72][14]) * s.v[305]) + (eq93_e1348 * s.db[305][14]));
        let eq93_e1350_d_b15: f64 = (((p.p6 * s.db[72][15]) * s.v[305]) + (eq93_e1348 * s.db[305][15]));
        let eq93_e1350_d_b16: f64 = (((p.p6 * s.db[72][16]) * s.v[305]) + (eq93_e1348 * s.db[305][16]));
        let eq93_e1350_d_b17: f64 = (((p.p6 * s.db[72][17]) * s.v[305]) + (eq93_e1348 * s.db[305][17]));
        let eq93_e1350_d_b18: f64 = (((p.p6 * s.db[72][18]) * s.v[305]) + (eq93_e1348 * s.db[305][18]));
        let eq93_e1350_d_b19: f64 = (((p.p6 * s.db[72][19]) * s.v[305]) + (eq93_e1348 * s.db[305][19]));
        let eq93_e1350_d_b20: f64 = (((p.p6 * s.db[72][20]) * s.v[305]) + (eq93_e1348 * s.db[305][20]));
        let eq93_e1350_d_b21: f64 = (((p.p6 * s.db[72][21]) * s.v[305]) + (eq93_e1348 * s.db[305][21]));
        let eq93_e1350_d_b22: f64 = (((p.p6 * s.db[72][22]) * s.v[305]) + (eq93_e1348 * s.db[305][22]));
        let eq93_e1350_d_b23: f64 = (((p.p6 * s.db[72][23]) * s.v[305]) + (eq93_e1348 * s.db[305][23]));
        let eq93_e1350_d_b24: f64 = (((p.p6 * s.db[72][24]) * s.v[305]) + (eq93_e1348 * s.db[305][24]));
        let eq93_e1350_d_b25: f64 = (((p.p6 * s.db[72][25]) * s.v[305]) + (eq93_e1348 * s.db[305][25]));
        let eq93_e1350_d_b26: f64 = (((p.p6 * s.db[72][26]) * s.v[305]) + (eq93_e1348 * s.db[305][26]));
        let eq93_e1350_d_b27: f64 = (((p.p6 * s.db[72][27]) * s.v[305]) + (eq93_e1348 * s.db[305][27]));
        let eq93_e1350_d_b28: f64 = (((p.p6 * s.db[72][28]) * s.v[305]) + (eq93_e1348 * s.db[305][28]));
        let eq93_e1350_d_b29: f64 = (((p.p6 * s.db[72][29]) * s.v[305]) + (eq93_e1348 * s.db[305][29]));
        let eq93_e1350_d_b30: f64 = (((p.p6 * s.db[72][30]) * s.v[305]) + (eq93_e1348 * s.db[305][30]));
        let eq93_e1350_d_b31: f64 = (((p.p6 * s.db[72][31]) * s.v[305]) + (eq93_e1348 * s.db[305][31]));
        let eq93_e1350_d_b32: f64 = (((p.p6 * s.db[72][32]) * s.v[305]) + (eq93_e1348 * s.db[305][32]));
        let eq93_e1350_d_b33: f64 = (((p.p6 * s.db[72][33]) * s.v[305]) + (eq93_e1348 * s.db[305][33]));
        let eq93_e1350_d_b34: f64 = (((p.p6 * s.db[72][34]) * s.v[305]) + (eq93_e1348 * s.db[305][34]));
        let eq93_e1350_d_b35: f64 = (((p.p6 * s.db[72][35]) * s.v[305]) + (eq93_e1348 * s.db[305][35]));
        let eq93_e1350_d_b36: f64 = (((p.p6 * s.db[72][36]) * s.v[305]) + (eq93_e1348 * s.db[305][36]));
        let eq93_e1350_d_b37: f64 = (((p.p6 * s.db[72][37]) * s.v[305]) + (eq93_e1348 * s.db[305][37]));
        let eq93_e1350_d_b38: f64 = (((p.p6 * s.db[72][38]) * s.v[305]) + (eq93_e1348 * s.db[305][38]));
        let eq93_e1350_d_b39: f64 = (((p.p6 * s.db[72][39]) * s.v[305]) + (eq93_e1348 * s.db[305][39]));
        let eq93_e1350_d_b40: f64 = (((p.p6 * s.db[72][40]) * s.v[305]) + (eq93_e1348 * s.db[305][40]));
        let eq93_e1350_d_b41: f64 = (((p.p6 * s.db[72][41]) * s.v[305]) + (eq93_e1348 * s.db[305][41]));
        let eq93_e1350_d_b42: f64 = (((p.p6 * s.db[72][42]) * s.v[305]) + (eq93_e1348 * s.db[305][42]));
        let eq93_e1350_d_b43: f64 = (((p.p6 * s.db[72][43]) * s.v[305]) + (eq93_e1348 * s.db[305][43]));
        let eq93_e1350_d_b44: f64 = (((p.p6 * s.db[72][44]) * s.v[305]) + (eq93_e1348 * s.db[305][44]));
        let eq93_e1350_d_b45: f64 = (((p.p6 * s.db[72][45]) * s.v[305]) + (eq93_e1348 * s.db[305][45]));
        let eq93_e1350_d_b46: f64 = (((p.p6 * s.db[72][46]) * s.v[305]) + (eq93_e1348 * s.db[305][46]));
        let eq93_e1350_d_b47: f64 = (((p.p6 * s.db[72][47]) * s.v[305]) + (eq93_e1348 * s.db[305][47]));
        let eq93_e1350_d_b48: f64 = (((p.p6 * s.db[72][48]) * s.v[305]) + (eq93_e1348 * s.db[305][48]));
        let eq93_e1350_d_b49: f64 = (((p.p6 * s.db[72][49]) * s.v[305]) + (eq93_e1348 * s.db[305][49]));
        let eq93_e1350_d_b50: f64 = (((p.p6 * s.db[72][50]) * s.v[305]) + (eq93_e1348 * s.db[305][50]));
        let eq93_e1350_d_b51: f64 = (((p.p6 * s.db[72][51]) * s.v[305]) + (eq93_e1348 * s.db[305][51]));
        let eq93_e1350_d_b52: f64 = (((p.p6 * s.db[72][52]) * s.v[305]) + (eq93_e1348 * s.db[305][52]));
        let eq93_e1350_d_b53: f64 = (((p.p6 * s.db[72][53]) * s.v[305]) + (eq93_e1348 * s.db[305][53]));
        let eq93_e1350_d_b54: f64 = (((p.p6 * s.db[72][54]) * s.v[305]) + (eq93_e1348 * s.db[305][54]));
        let eq93_e1353: f64 = (p.p6 * s.v[379]);
        let eq93_e1355: f64 = (eq93_e1353 * (nv18 - nv17));
        let eq93_e1355_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv18 - nv17));
        let eq93_e1355_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv18 - nv17));
        let eq93_e1355_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv18 - nv17));
        let eq93_e1355_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv18 - nv17));
        let eq93_e1355_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv18 - nv17));
        let eq93_e1355_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv18 - nv17));
        let eq93_e1355_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv18 - nv17));
        let eq93_e1355_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv18 - nv17));
        let eq93_e1355_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv18 - nv17));
        let eq93_e1355_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv18 - nv17));
        let eq93_e1355_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv18 - nv17));
        let eq93_e1355_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv18 - nv17));
        let eq93_e1355_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv18 - nv17));
        let eq93_e1355_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv18 - nv17));
        let eq93_e1355_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv18 - nv17));
        let eq93_e1355_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv18 - nv17));
        let eq93_e1355_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv18 - nv17));
        let eq93_e1355_d_n17: f64 = (((p.p6 * s.dn[379][17]) * (nv18 - nv17)) + (-eq93_e1353));
        let eq93_e1355_d_n18: f64 = (((p.p6 * s.dn[379][18]) * (nv18 - nv17)) + eq93_e1353);
        let eq93_e1355_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv18 - nv17));
        let eq93_e1355_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv18 - nv17));
        let eq93_e1355_d_n21: f64 = ((p.p6 * s.dn[379][21]) * (nv18 - nv17));
        let eq93_e1355_d_n22: f64 = ((p.p6 * s.dn[379][22]) * (nv18 - nv17));
        let eq93_e1355_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv18 - nv17));
        let eq93_e1355_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv18 - nv17));
        let eq93_e1355_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv18 - nv17));
        let eq93_e1355_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv18 - nv17));
        let eq93_e1355_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv18 - nv17));
        let eq93_e1355_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv18 - nv17));
        let eq93_e1355_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv18 - nv17));
        let eq93_e1355_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv18 - nv17));
        let eq93_e1355_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv18 - nv17));
        let eq93_e1355_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv18 - nv17));
        let eq93_e1355_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv18 - nv17));
        let eq93_e1355_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv18 - nv17));
        let eq93_e1355_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv18 - nv17));
        let eq93_e1355_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv18 - nv17));
        let eq93_e1355_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv18 - nv17));
        let eq93_e1355_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv18 - nv17));
        let eq93_e1355_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv18 - nv17));
        let eq93_e1355_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv18 - nv17));
        let eq93_e1355_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv18 - nv17));
        let eq93_e1355_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv18 - nv17));
        let eq93_e1355_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv18 - nv17));
        let eq93_e1355_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv18 - nv17));
        let eq93_e1355_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv18 - nv17));
        let eq93_e1355_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv18 - nv17));
        let eq93_e1355_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv18 - nv17));
        let eq93_e1355_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv18 - nv17));
        let eq93_e1355_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv18 - nv17));
        let eq93_e1355_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv18 - nv17));
        let eq93_e1355_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv18 - nv17));
        let eq93_e1355_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv18 - nv17));
        let eq93_e1355_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv18 - nv17));
        let eq93_e1355_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv18 - nv17));
        let eq93_e1355_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv18 - nv17));
        let eq93_e1355_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv18 - nv17));
        let eq93_e1355_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv18 - nv17));
        let eq93_e1355_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv18 - nv17));
        let eq93_e1355_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv18 - nv17));
        let eq93_e1355_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv18 - nv17));
        let eq93_e1355_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv18 - nv17));
        let eq93_e1355_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv18 - nv17));
        let eq93_e1355_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv18 - nv17));
        let eq93_e1355_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv18 - nv17));
        let eq93_e1355_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv18 - nv17));
        let eq93_e1355_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv18 - nv17));
        let eq93_e1355_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv18 - nv17));
        let eq93_e1355_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv18 - nv17));
        let eq93_e1355_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv18 - nv17));
        let eq93_e1355_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv18 - nv17));
        let eq93_e1355_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv18 - nv17));
        let eq93_e1355_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv18 - nv17));
        let eq93_e1355_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv18 - nv17));
        let eq93_e1355_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv18 - nv17));
        let eq93_e1355_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv18 - nv17));
        let eq93_e1355_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv18 - nv17));
        let eq93_e1355_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv18 - nv17));
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
        let eq93_e1356_d_b0: f64 = (eq93_e1350_d_b0 + eq93_e1355_d_b0);
        let eq93_e1356_d_b1: f64 = (eq93_e1350_d_b1 + eq93_e1355_d_b1);
        let eq93_e1356_d_b2: f64 = (eq93_e1350_d_b2 + eq93_e1355_d_b2);
        let eq93_e1356_d_b3: f64 = (eq93_e1350_d_b3 + eq93_e1355_d_b3);
        let eq93_e1356_d_b4: f64 = (eq93_e1350_d_b4 + eq93_e1355_d_b4);
        let eq93_e1356_d_b5: f64 = (eq93_e1350_d_b5 + eq93_e1355_d_b5);
        let eq93_e1356_d_b6: f64 = (eq93_e1350_d_b6 + eq93_e1355_d_b6);
        let eq93_e1356_d_b7: f64 = (eq93_e1350_d_b7 + eq93_e1355_d_b7);
        let eq93_e1356_d_b8: f64 = (eq93_e1350_d_b8 + eq93_e1355_d_b8);
        let eq93_e1356_d_b9: f64 = (eq93_e1350_d_b9 + eq93_e1355_d_b9);
        let eq93_e1356_d_b10: f64 = (eq93_e1350_d_b10 + eq93_e1355_d_b10);
        let eq93_e1356_d_b11: f64 = (eq93_e1350_d_b11 + eq93_e1355_d_b11);
        let eq93_e1356_d_b12: f64 = (eq93_e1350_d_b12 + eq93_e1355_d_b12);
        let eq93_e1356_d_b13: f64 = (eq93_e1350_d_b13 + eq93_e1355_d_b13);
        let eq93_e1356_d_b14: f64 = (eq93_e1350_d_b14 + eq93_e1355_d_b14);
        let eq93_e1356_d_b15: f64 = (eq93_e1350_d_b15 + eq93_e1355_d_b15);
        let eq93_e1356_d_b16: f64 = (eq93_e1350_d_b16 + eq93_e1355_d_b16);
        let eq93_e1356_d_b17: f64 = (eq93_e1350_d_b17 + eq93_e1355_d_b17);
        let eq93_e1356_d_b18: f64 = (eq93_e1350_d_b18 + eq93_e1355_d_b18);
        let eq93_e1356_d_b19: f64 = (eq93_e1350_d_b19 + eq93_e1355_d_b19);
        let eq93_e1356_d_b20: f64 = (eq93_e1350_d_b20 + eq93_e1355_d_b20);
        let eq93_e1356_d_b21: f64 = (eq93_e1350_d_b21 + eq93_e1355_d_b21);
        let eq93_e1356_d_b22: f64 = (eq93_e1350_d_b22 + eq93_e1355_d_b22);
        let eq93_e1356_d_b23: f64 = (eq93_e1350_d_b23 + eq93_e1355_d_b23);
        let eq93_e1356_d_b24: f64 = (eq93_e1350_d_b24 + eq93_e1355_d_b24);
        let eq93_e1356_d_b25: f64 = (eq93_e1350_d_b25 + eq93_e1355_d_b25);
        let eq93_e1356_d_b26: f64 = (eq93_e1350_d_b26 + eq93_e1355_d_b26);
        let eq93_e1356_d_b27: f64 = (eq93_e1350_d_b27 + eq93_e1355_d_b27);
        let eq93_e1356_d_b28: f64 = (eq93_e1350_d_b28 + eq93_e1355_d_b28);
        let eq93_e1356_d_b29: f64 = (eq93_e1350_d_b29 + eq93_e1355_d_b29);
        let eq93_e1356_d_b30: f64 = (eq93_e1350_d_b30 + eq93_e1355_d_b30);
        let eq93_e1356_d_b31: f64 = (eq93_e1350_d_b31 + eq93_e1355_d_b31);
        let eq93_e1356_d_b32: f64 = (eq93_e1350_d_b32 + eq93_e1355_d_b32);
        let eq93_e1356_d_b33: f64 = (eq93_e1350_d_b33 + eq93_e1355_d_b33);
        let eq93_e1356_d_b34: f64 = (eq93_e1350_d_b34 + eq93_e1355_d_b34);
        let eq93_e1356_d_b35: f64 = (eq93_e1350_d_b35 + eq93_e1355_d_b35);
        let eq93_e1356_d_b36: f64 = (eq93_e1350_d_b36 + eq93_e1355_d_b36);
        let eq93_e1356_d_b37: f64 = (eq93_e1350_d_b37 + eq93_e1355_d_b37);
        let eq93_e1356_d_b38: f64 = (eq93_e1350_d_b38 + eq93_e1355_d_b38);
        let eq93_e1356_d_b39: f64 = (eq93_e1350_d_b39 + eq93_e1355_d_b39);
        let eq93_e1356_d_b40: f64 = (eq93_e1350_d_b40 + eq93_e1355_d_b40);
        let eq93_e1356_d_b41: f64 = (eq93_e1350_d_b41 + eq93_e1355_d_b41);
        let eq93_e1356_d_b42: f64 = (eq93_e1350_d_b42 + eq93_e1355_d_b42);
        let eq93_e1356_d_b43: f64 = (eq93_e1350_d_b43 + eq93_e1355_d_b43);
        let eq93_e1356_d_b44: f64 = (eq93_e1350_d_b44 + eq93_e1355_d_b44);
        let eq93_e1356_d_b45: f64 = (eq93_e1350_d_b45 + eq93_e1355_d_b45);
        let eq93_e1356_d_b46: f64 = (eq93_e1350_d_b46 + eq93_e1355_d_b46);
        let eq93_e1356_d_b47: f64 = (eq93_e1350_d_b47 + eq93_e1355_d_b47);
        let eq93_e1356_d_b48: f64 = (eq93_e1350_d_b48 + eq93_e1355_d_b48);
        let eq93_e1356_d_b49: f64 = (eq93_e1350_d_b49 + eq93_e1355_d_b49);
        let eq93_e1356_d_b50: f64 = (eq93_e1350_d_b50 + eq93_e1355_d_b50);
        let eq93_e1356_d_b51: f64 = (eq93_e1350_d_b51 + eq93_e1355_d_b51);
        let eq93_e1356_d_b52: f64 = (eq93_e1350_d_b52 + eq93_e1355_d_b52);
        let eq93_e1356_d_b53: f64 = (eq93_e1350_d_b53 + eq93_e1355_d_b53);
        let eq93_e1356_d_b54: f64 = (eq93_e1350_d_b54 + eq93_e1355_d_b54);
        (eq93_e1356, eq93_e1356_d_n0, eq93_e1356_d_n1, eq93_e1356_d_n2, eq93_e1356_d_n3, eq93_e1356_d_n4, eq93_e1356_d_n5, eq93_e1356_d_n6, eq93_e1356_d_n7, eq93_e1356_d_n8, eq93_e1356_d_n9, eq93_e1356_d_n10, eq93_e1356_d_n11, eq93_e1356_d_n12, eq93_e1356_d_n13, eq93_e1356_d_n14, eq93_e1356_d_n15, eq93_e1356_d_n16, eq93_e1356_d_n17, eq93_e1356_d_n18, eq93_e1356_d_n19, eq93_e1356_d_n20, eq93_e1356_d_n21, eq93_e1356_d_n22, eq93_e1356_d_b0, eq93_e1356_d_b1, eq93_e1356_d_b2, eq93_e1356_d_b3, eq93_e1356_d_b4, eq93_e1356_d_b5, eq93_e1356_d_b6, eq93_e1356_d_b7, eq93_e1356_d_b8, eq93_e1356_d_b9, eq93_e1356_d_b10, eq93_e1356_d_b11, eq93_e1356_d_b12, eq93_e1356_d_b13, eq93_e1356_d_b14, eq93_e1356_d_b15, eq93_e1356_d_b16, eq93_e1356_d_b17, eq93_e1356_d_b18, eq93_e1356_d_b19, eq93_e1356_d_b20, eq93_e1356_d_b21, eq93_e1356_d_b22, eq93_e1356_d_b23, eq93_e1356_d_b24, eq93_e1356_d_b25, eq93_e1356_d_b26, eq93_e1356_d_b27, eq93_e1356_d_b28, eq93_e1356_d_b29, eq93_e1356_d_b30, eq93_e1356_d_b31, eq93_e1356_d_b32, eq93_e1356_d_b33, eq93_e1356_d_b34, eq93_e1356_d_b35, eq93_e1356_d_b36, eq93_e1356_d_b37, eq93_e1356_d_b38, eq93_e1356_d_b39, eq93_e1356_d_b40, eq93_e1356_d_b41, eq93_e1356_d_b42, eq93_e1356_d_b43, eq93_e1356_d_b44, eq93_e1356_d_b45, eq93_e1356_d_b46, eq93_e1356_d_b47, eq93_e1356_d_b48, eq93_e1356_d_b49, eq93_e1356_d_b50, eq93_e1356_d_b51, eq93_e1356_d_b52, eq93_e1356_d_b53, eq93_e1356_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq93_value: f64 = eq93_e1358;
        let eq93_node_derivatives: [f64; 23] = [eq93_e1358_d_n0, eq93_e1358_d_n1, eq93_e1358_d_n2, eq93_e1358_d_n3, eq93_e1358_d_n4, eq93_e1358_d_n5, eq93_e1358_d_n6, eq93_e1358_d_n7, eq93_e1358_d_n8, eq93_e1358_d_n9, eq93_e1358_d_n10, eq93_e1358_d_n11, eq93_e1358_d_n12, eq93_e1358_d_n13, eq93_e1358_d_n14, eq93_e1358_d_n15, eq93_e1358_d_n16, eq93_e1358_d_n17, eq93_e1358_d_n18, eq93_e1358_d_n19, eq93_e1358_d_n20, eq93_e1358_d_n21, eq93_e1358_d_n22];
        let eq93_branch_derivatives: [f64; 55] = [eq93_e1358_d_b0, eq93_e1358_d_b1, eq93_e1358_d_b2, eq93_e1358_d_b3, eq93_e1358_d_b4, eq93_e1358_d_b5, eq93_e1358_d_b6, eq93_e1358_d_b7, eq93_e1358_d_b8, eq93_e1358_d_b9, eq93_e1358_d_b10, eq93_e1358_d_b11, eq93_e1358_d_b12, eq93_e1358_d_b13, eq93_e1358_d_b14, eq93_e1358_d_b15, eq93_e1358_d_b16, eq93_e1358_d_b17, eq93_e1358_d_b18, eq93_e1358_d_b19, eq93_e1358_d_b20, eq93_e1358_d_b21, eq93_e1358_d_b22, eq93_e1358_d_b23, eq93_e1358_d_b24, eq93_e1358_d_b25, eq93_e1358_d_b26, eq93_e1358_d_b27, eq93_e1358_d_b28, eq93_e1358_d_b29, eq93_e1358_d_b30, eq93_e1358_d_b31, eq93_e1358_d_b32, eq93_e1358_d_b33, eq93_e1358_d_b34, eq93_e1358_d_b35, eq93_e1358_d_b36, eq93_e1358_d_b37, eq93_e1358_d_b38, eq93_e1358_d_b39, eq93_e1358_d_b40, eq93_e1358_d_b41, eq93_e1358_d_b42, eq93_e1358_d_b43, eq93_e1358_d_b44, eq93_e1358_d_b45, eq93_e1358_d_b46, eq93_e1358_d_b47, eq93_e1358_d_b48, eq93_e1358_d_b49, eq93_e1358_d_b50, eq93_e1358_d_b51, eq93_e1358_d_b52, eq93_e1358_d_b53, eq93_e1358_d_b54];
        stamper.stamp_current_dense_local(
            Some(18),
            Some(17),
            multiplicity * (eq93_value),
            &eq93_node_derivatives,
            &eq93_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq96_e1386, eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22, eq96_e1386_d_b0, eq96_e1386_d_b1, eq96_e1386_d_b2, eq96_e1386_d_b3, eq96_e1386_d_b4, eq96_e1386_d_b5, eq96_e1386_d_b6, eq96_e1386_d_b7, eq96_e1386_d_b8, eq96_e1386_d_b9, eq96_e1386_d_b10, eq96_e1386_d_b11, eq96_e1386_d_b12, eq96_e1386_d_b13, eq96_e1386_d_b14, eq96_e1386_d_b15, eq96_e1386_d_b16, eq96_e1386_d_b17, eq96_e1386_d_b18, eq96_e1386_d_b19, eq96_e1386_d_b20, eq96_e1386_d_b21, eq96_e1386_d_b22, eq96_e1386_d_b23, eq96_e1386_d_b24, eq96_e1386_d_b25, eq96_e1386_d_b26, eq96_e1386_d_b27, eq96_e1386_d_b28, eq96_e1386_d_b29, eq96_e1386_d_b30, eq96_e1386_d_b31, eq96_e1386_d_b32, eq96_e1386_d_b33, eq96_e1386_d_b34, eq96_e1386_d_b35, eq96_e1386_d_b36, eq96_e1386_d_b37, eq96_e1386_d_b38, eq96_e1386_d_b39, eq96_e1386_d_b40, eq96_e1386_d_b41, eq96_e1386_d_b42, eq96_e1386_d_b43, eq96_e1386_d_b44, eq96_e1386_d_b45, eq96_e1386_d_b46, eq96_e1386_d_b47, eq96_e1386_d_b48, eq96_e1386_d_b49, eq96_e1386_d_b50, eq96_e1386_d_b51, eq96_e1386_d_b52, eq96_e1386_d_b53, eq96_e1386_d_b54,) = {
    if (s.b[538] && s.b[539]) {
        let eq96_e1376: f64 = (p.p6 * s.v[76]);
        let eq96_e1378: f64 = (eq96_e1376 * s.v[317]);
        let eq96_e1378_d_n0: f64 = (((p.p6 * s.dn[76][0]) * s.v[317]) + (eq96_e1376 * s.dn[317][0]));
        let eq96_e1378_d_n1: f64 = (((p.p6 * s.dn[76][1]) * s.v[317]) + (eq96_e1376 * s.dn[317][1]));
        let eq96_e1378_d_n2: f64 = (((p.p6 * s.dn[76][2]) * s.v[317]) + (eq96_e1376 * s.dn[317][2]));
        let eq96_e1378_d_n3: f64 = (((p.p6 * s.dn[76][3]) * s.v[317]) + (eq96_e1376 * s.dn[317][3]));
        let eq96_e1378_d_n4: f64 = (((p.p6 * s.dn[76][4]) * s.v[317]) + (eq96_e1376 * s.dn[317][4]));
        let eq96_e1378_d_n5: f64 = (((p.p6 * s.dn[76][5]) * s.v[317]) + (eq96_e1376 * s.dn[317][5]));
        let eq96_e1378_d_n6: f64 = (((p.p6 * s.dn[76][6]) * s.v[317]) + (eq96_e1376 * s.dn[317][6]));
        let eq96_e1378_d_n7: f64 = (((p.p6 * s.dn[76][7]) * s.v[317]) + (eq96_e1376 * s.dn[317][7]));
        let eq96_e1378_d_n8: f64 = (((p.p6 * s.dn[76][8]) * s.v[317]) + (eq96_e1376 * s.dn[317][8]));
        let eq96_e1378_d_n9: f64 = (((p.p6 * s.dn[76][9]) * s.v[317]) + (eq96_e1376 * s.dn[317][9]));
        let eq96_e1378_d_n10: f64 = (((p.p6 * s.dn[76][10]) * s.v[317]) + (eq96_e1376 * s.dn[317][10]));
        let eq96_e1378_d_n11: f64 = (((p.p6 * s.dn[76][11]) * s.v[317]) + (eq96_e1376 * s.dn[317][11]));
        let eq96_e1378_d_n12: f64 = (((p.p6 * s.dn[76][12]) * s.v[317]) + (eq96_e1376 * s.dn[317][12]));
        let eq96_e1378_d_n13: f64 = (((p.p6 * s.dn[76][13]) * s.v[317]) + (eq96_e1376 * s.dn[317][13]));
        let eq96_e1378_d_n14: f64 = (((p.p6 * s.dn[76][14]) * s.v[317]) + (eq96_e1376 * s.dn[317][14]));
        let eq96_e1378_d_n15: f64 = (((p.p6 * s.dn[76][15]) * s.v[317]) + (eq96_e1376 * s.dn[317][15]));
        let eq96_e1378_d_n16: f64 = (((p.p6 * s.dn[76][16]) * s.v[317]) + (eq96_e1376 * s.dn[317][16]));
        let eq96_e1378_d_n17: f64 = (((p.p6 * s.dn[76][17]) * s.v[317]) + (eq96_e1376 * s.dn[317][17]));
        let eq96_e1378_d_n18: f64 = (((p.p6 * s.dn[76][18]) * s.v[317]) + (eq96_e1376 * s.dn[317][18]));
        let eq96_e1378_d_n19: f64 = (((p.p6 * s.dn[76][19]) * s.v[317]) + (eq96_e1376 * s.dn[317][19]));
        let eq96_e1378_d_n20: f64 = (((p.p6 * s.dn[76][20]) * s.v[317]) + (eq96_e1376 * s.dn[317][20]));
        let eq96_e1378_d_n21: f64 = (((p.p6 * s.dn[76][21]) * s.v[317]) + (eq96_e1376 * s.dn[317][21]));
        let eq96_e1378_d_n22: f64 = (((p.p6 * s.dn[76][22]) * s.v[317]) + (eq96_e1376 * s.dn[317][22]));
        let eq96_e1378_d_b0: f64 = (((p.p6 * s.db[76][0]) * s.v[317]) + (eq96_e1376 * s.db[317][0]));
        let eq96_e1378_d_b1: f64 = (((p.p6 * s.db[76][1]) * s.v[317]) + (eq96_e1376 * s.db[317][1]));
        let eq96_e1378_d_b2: f64 = (((p.p6 * s.db[76][2]) * s.v[317]) + (eq96_e1376 * s.db[317][2]));
        let eq96_e1378_d_b3: f64 = (((p.p6 * s.db[76][3]) * s.v[317]) + (eq96_e1376 * s.db[317][3]));
        let eq96_e1378_d_b4: f64 = (((p.p6 * s.db[76][4]) * s.v[317]) + (eq96_e1376 * s.db[317][4]));
        let eq96_e1378_d_b5: f64 = (((p.p6 * s.db[76][5]) * s.v[317]) + (eq96_e1376 * s.db[317][5]));
        let eq96_e1378_d_b6: f64 = (((p.p6 * s.db[76][6]) * s.v[317]) + (eq96_e1376 * s.db[317][6]));
        let eq96_e1378_d_b7: f64 = (((p.p6 * s.db[76][7]) * s.v[317]) + (eq96_e1376 * s.db[317][7]));
        let eq96_e1378_d_b8: f64 = (((p.p6 * s.db[76][8]) * s.v[317]) + (eq96_e1376 * s.db[317][8]));
        let eq96_e1378_d_b9: f64 = (((p.p6 * s.db[76][9]) * s.v[317]) + (eq96_e1376 * s.db[317][9]));
        let eq96_e1378_d_b10: f64 = (((p.p6 * s.db[76][10]) * s.v[317]) + (eq96_e1376 * s.db[317][10]));
        let eq96_e1378_d_b11: f64 = (((p.p6 * s.db[76][11]) * s.v[317]) + (eq96_e1376 * s.db[317][11]));
        let eq96_e1378_d_b12: f64 = (((p.p6 * s.db[76][12]) * s.v[317]) + (eq96_e1376 * s.db[317][12]));
        let eq96_e1378_d_b13: f64 = (((p.p6 * s.db[76][13]) * s.v[317]) + (eq96_e1376 * s.db[317][13]));
        let eq96_e1378_d_b14: f64 = (((p.p6 * s.db[76][14]) * s.v[317]) + (eq96_e1376 * s.db[317][14]));
        let eq96_e1378_d_b15: f64 = (((p.p6 * s.db[76][15]) * s.v[317]) + (eq96_e1376 * s.db[317][15]));
        let eq96_e1378_d_b16: f64 = (((p.p6 * s.db[76][16]) * s.v[317]) + (eq96_e1376 * s.db[317][16]));
        let eq96_e1378_d_b17: f64 = (((p.p6 * s.db[76][17]) * s.v[317]) + (eq96_e1376 * s.db[317][17]));
        let eq96_e1378_d_b18: f64 = (((p.p6 * s.db[76][18]) * s.v[317]) + (eq96_e1376 * s.db[317][18]));
        let eq96_e1378_d_b19: f64 = (((p.p6 * s.db[76][19]) * s.v[317]) + (eq96_e1376 * s.db[317][19]));
        let eq96_e1378_d_b20: f64 = (((p.p6 * s.db[76][20]) * s.v[317]) + (eq96_e1376 * s.db[317][20]));
        let eq96_e1378_d_b21: f64 = (((p.p6 * s.db[76][21]) * s.v[317]) + (eq96_e1376 * s.db[317][21]));
        let eq96_e1378_d_b22: f64 = (((p.p6 * s.db[76][22]) * s.v[317]) + (eq96_e1376 * s.db[317][22]));
        let eq96_e1378_d_b23: f64 = (((p.p6 * s.db[76][23]) * s.v[317]) + (eq96_e1376 * s.db[317][23]));
        let eq96_e1378_d_b24: f64 = (((p.p6 * s.db[76][24]) * s.v[317]) + (eq96_e1376 * s.db[317][24]));
        let eq96_e1378_d_b25: f64 = (((p.p6 * s.db[76][25]) * s.v[317]) + (eq96_e1376 * s.db[317][25]));
        let eq96_e1378_d_b26: f64 = (((p.p6 * s.db[76][26]) * s.v[317]) + (eq96_e1376 * s.db[317][26]));
        let eq96_e1378_d_b27: f64 = (((p.p6 * s.db[76][27]) * s.v[317]) + (eq96_e1376 * s.db[317][27]));
        let eq96_e1378_d_b28: f64 = (((p.p6 * s.db[76][28]) * s.v[317]) + (eq96_e1376 * s.db[317][28]));
        let eq96_e1378_d_b29: f64 = (((p.p6 * s.db[76][29]) * s.v[317]) + (eq96_e1376 * s.db[317][29]));
        let eq96_e1378_d_b30: f64 = (((p.p6 * s.db[76][30]) * s.v[317]) + (eq96_e1376 * s.db[317][30]));
        let eq96_e1378_d_b31: f64 = (((p.p6 * s.db[76][31]) * s.v[317]) + (eq96_e1376 * s.db[317][31]));
        let eq96_e1378_d_b32: f64 = (((p.p6 * s.db[76][32]) * s.v[317]) + (eq96_e1376 * s.db[317][32]));
        let eq96_e1378_d_b33: f64 = (((p.p6 * s.db[76][33]) * s.v[317]) + (eq96_e1376 * s.db[317][33]));
        let eq96_e1378_d_b34: f64 = (((p.p6 * s.db[76][34]) * s.v[317]) + (eq96_e1376 * s.db[317][34]));
        let eq96_e1378_d_b35: f64 = (((p.p6 * s.db[76][35]) * s.v[317]) + (eq96_e1376 * s.db[317][35]));
        let eq96_e1378_d_b36: f64 = (((p.p6 * s.db[76][36]) * s.v[317]) + (eq96_e1376 * s.db[317][36]));
        let eq96_e1378_d_b37: f64 = (((p.p6 * s.db[76][37]) * s.v[317]) + (eq96_e1376 * s.db[317][37]));
        let eq96_e1378_d_b38: f64 = (((p.p6 * s.db[76][38]) * s.v[317]) + (eq96_e1376 * s.db[317][38]));
        let eq96_e1378_d_b39: f64 = (((p.p6 * s.db[76][39]) * s.v[317]) + (eq96_e1376 * s.db[317][39]));
        let eq96_e1378_d_b40: f64 = (((p.p6 * s.db[76][40]) * s.v[317]) + (eq96_e1376 * s.db[317][40]));
        let eq96_e1378_d_b41: f64 = (((p.p6 * s.db[76][41]) * s.v[317]) + (eq96_e1376 * s.db[317][41]));
        let eq96_e1378_d_b42: f64 = (((p.p6 * s.db[76][42]) * s.v[317]) + (eq96_e1376 * s.db[317][42]));
        let eq96_e1378_d_b43: f64 = (((p.p6 * s.db[76][43]) * s.v[317]) + (eq96_e1376 * s.db[317][43]));
        let eq96_e1378_d_b44: f64 = (((p.p6 * s.db[76][44]) * s.v[317]) + (eq96_e1376 * s.db[317][44]));
        let eq96_e1378_d_b45: f64 = (((p.p6 * s.db[76][45]) * s.v[317]) + (eq96_e1376 * s.db[317][45]));
        let eq96_e1378_d_b46: f64 = (((p.p6 * s.db[76][46]) * s.v[317]) + (eq96_e1376 * s.db[317][46]));
        let eq96_e1378_d_b47: f64 = (((p.p6 * s.db[76][47]) * s.v[317]) + (eq96_e1376 * s.db[317][47]));
        let eq96_e1378_d_b48: f64 = (((p.p6 * s.db[76][48]) * s.v[317]) + (eq96_e1376 * s.db[317][48]));
        let eq96_e1378_d_b49: f64 = (((p.p6 * s.db[76][49]) * s.v[317]) + (eq96_e1376 * s.db[317][49]));
        let eq96_e1378_d_b50: f64 = (((p.p6 * s.db[76][50]) * s.v[317]) + (eq96_e1376 * s.db[317][50]));
        let eq96_e1378_d_b51: f64 = (((p.p6 * s.db[76][51]) * s.v[317]) + (eq96_e1376 * s.db[317][51]));
        let eq96_e1378_d_b52: f64 = (((p.p6 * s.db[76][52]) * s.v[317]) + (eq96_e1376 * s.db[317][52]));
        let eq96_e1378_d_b53: f64 = (((p.p6 * s.db[76][53]) * s.v[317]) + (eq96_e1376 * s.db[317][53]));
        let eq96_e1378_d_b54: f64 = (((p.p6 * s.db[76][54]) * s.v[317]) + (eq96_e1376 * s.db[317][54]));
        let eq96_e1381: f64 = (p.p6 * s.v[379]);
        let eq96_e1383: f64 = (eq96_e1381 * (nv21 - nv22));
        let eq96_e1383_d_n0: f64 = ((p.p6 * s.dn[379][0]) * (nv21 - nv22));
        let eq96_e1383_d_n1: f64 = ((p.p6 * s.dn[379][1]) * (nv21 - nv22));
        let eq96_e1383_d_n2: f64 = ((p.p6 * s.dn[379][2]) * (nv21 - nv22));
        let eq96_e1383_d_n3: f64 = ((p.p6 * s.dn[379][3]) * (nv21 - nv22));
        let eq96_e1383_d_n4: f64 = ((p.p6 * s.dn[379][4]) * (nv21 - nv22));
        let eq96_e1383_d_n5: f64 = ((p.p6 * s.dn[379][5]) * (nv21 - nv22));
        let eq96_e1383_d_n6: f64 = ((p.p6 * s.dn[379][6]) * (nv21 - nv22));
        let eq96_e1383_d_n7: f64 = ((p.p6 * s.dn[379][7]) * (nv21 - nv22));
        let eq96_e1383_d_n8: f64 = ((p.p6 * s.dn[379][8]) * (nv21 - nv22));
        let eq96_e1383_d_n9: f64 = ((p.p6 * s.dn[379][9]) * (nv21 - nv22));
        let eq96_e1383_d_n10: f64 = ((p.p6 * s.dn[379][10]) * (nv21 - nv22));
        let eq96_e1383_d_n11: f64 = ((p.p6 * s.dn[379][11]) * (nv21 - nv22));
        let eq96_e1383_d_n12: f64 = ((p.p6 * s.dn[379][12]) * (nv21 - nv22));
        let eq96_e1383_d_n13: f64 = ((p.p6 * s.dn[379][13]) * (nv21 - nv22));
        let eq96_e1383_d_n14: f64 = ((p.p6 * s.dn[379][14]) * (nv21 - nv22));
        let eq96_e1383_d_n15: f64 = ((p.p6 * s.dn[379][15]) * (nv21 - nv22));
        let eq96_e1383_d_n16: f64 = ((p.p6 * s.dn[379][16]) * (nv21 - nv22));
        let eq96_e1383_d_n17: f64 = ((p.p6 * s.dn[379][17]) * (nv21 - nv22));
        let eq96_e1383_d_n18: f64 = ((p.p6 * s.dn[379][18]) * (nv21 - nv22));
        let eq96_e1383_d_n19: f64 = ((p.p6 * s.dn[379][19]) * (nv21 - nv22));
        let eq96_e1383_d_n20: f64 = ((p.p6 * s.dn[379][20]) * (nv21 - nv22));
        let eq96_e1383_d_n21: f64 = (((p.p6 * s.dn[379][21]) * (nv21 - nv22)) + eq96_e1381);
        let eq96_e1383_d_n22: f64 = (((p.p6 * s.dn[379][22]) * (nv21 - nv22)) + (-eq96_e1381));
        let eq96_e1383_d_b0: f64 = ((p.p6 * s.db[379][0]) * (nv21 - nv22));
        let eq96_e1383_d_b1: f64 = ((p.p6 * s.db[379][1]) * (nv21 - nv22));
        let eq96_e1383_d_b2: f64 = ((p.p6 * s.db[379][2]) * (nv21 - nv22));
        let eq96_e1383_d_b3: f64 = ((p.p6 * s.db[379][3]) * (nv21 - nv22));
        let eq96_e1383_d_b4: f64 = ((p.p6 * s.db[379][4]) * (nv21 - nv22));
        let eq96_e1383_d_b5: f64 = ((p.p6 * s.db[379][5]) * (nv21 - nv22));
        let eq96_e1383_d_b6: f64 = ((p.p6 * s.db[379][6]) * (nv21 - nv22));
        let eq96_e1383_d_b7: f64 = ((p.p6 * s.db[379][7]) * (nv21 - nv22));
        let eq96_e1383_d_b8: f64 = ((p.p6 * s.db[379][8]) * (nv21 - nv22));
        let eq96_e1383_d_b9: f64 = ((p.p6 * s.db[379][9]) * (nv21 - nv22));
        let eq96_e1383_d_b10: f64 = ((p.p6 * s.db[379][10]) * (nv21 - nv22));
        let eq96_e1383_d_b11: f64 = ((p.p6 * s.db[379][11]) * (nv21 - nv22));
        let eq96_e1383_d_b12: f64 = ((p.p6 * s.db[379][12]) * (nv21 - nv22));
        let eq96_e1383_d_b13: f64 = ((p.p6 * s.db[379][13]) * (nv21 - nv22));
        let eq96_e1383_d_b14: f64 = ((p.p6 * s.db[379][14]) * (nv21 - nv22));
        let eq96_e1383_d_b15: f64 = ((p.p6 * s.db[379][15]) * (nv21 - nv22));
        let eq96_e1383_d_b16: f64 = ((p.p6 * s.db[379][16]) * (nv21 - nv22));
        let eq96_e1383_d_b17: f64 = ((p.p6 * s.db[379][17]) * (nv21 - nv22));
        let eq96_e1383_d_b18: f64 = ((p.p6 * s.db[379][18]) * (nv21 - nv22));
        let eq96_e1383_d_b19: f64 = ((p.p6 * s.db[379][19]) * (nv21 - nv22));
        let eq96_e1383_d_b20: f64 = ((p.p6 * s.db[379][20]) * (nv21 - nv22));
        let eq96_e1383_d_b21: f64 = ((p.p6 * s.db[379][21]) * (nv21 - nv22));
        let eq96_e1383_d_b22: f64 = ((p.p6 * s.db[379][22]) * (nv21 - nv22));
        let eq96_e1383_d_b23: f64 = ((p.p6 * s.db[379][23]) * (nv21 - nv22));
        let eq96_e1383_d_b24: f64 = ((p.p6 * s.db[379][24]) * (nv21 - nv22));
        let eq96_e1383_d_b25: f64 = ((p.p6 * s.db[379][25]) * (nv21 - nv22));
        let eq96_e1383_d_b26: f64 = ((p.p6 * s.db[379][26]) * (nv21 - nv22));
        let eq96_e1383_d_b27: f64 = ((p.p6 * s.db[379][27]) * (nv21 - nv22));
        let eq96_e1383_d_b28: f64 = ((p.p6 * s.db[379][28]) * (nv21 - nv22));
        let eq96_e1383_d_b29: f64 = ((p.p6 * s.db[379][29]) * (nv21 - nv22));
        let eq96_e1383_d_b30: f64 = ((p.p6 * s.db[379][30]) * (nv21 - nv22));
        let eq96_e1383_d_b31: f64 = ((p.p6 * s.db[379][31]) * (nv21 - nv22));
        let eq96_e1383_d_b32: f64 = ((p.p6 * s.db[379][32]) * (nv21 - nv22));
        let eq96_e1383_d_b33: f64 = ((p.p6 * s.db[379][33]) * (nv21 - nv22));
        let eq96_e1383_d_b34: f64 = ((p.p6 * s.db[379][34]) * (nv21 - nv22));
        let eq96_e1383_d_b35: f64 = ((p.p6 * s.db[379][35]) * (nv21 - nv22));
        let eq96_e1383_d_b36: f64 = ((p.p6 * s.db[379][36]) * (nv21 - nv22));
        let eq96_e1383_d_b37: f64 = ((p.p6 * s.db[379][37]) * (nv21 - nv22));
        let eq96_e1383_d_b38: f64 = ((p.p6 * s.db[379][38]) * (nv21 - nv22));
        let eq96_e1383_d_b39: f64 = ((p.p6 * s.db[379][39]) * (nv21 - nv22));
        let eq96_e1383_d_b40: f64 = ((p.p6 * s.db[379][40]) * (nv21 - nv22));
        let eq96_e1383_d_b41: f64 = ((p.p6 * s.db[379][41]) * (nv21 - nv22));
        let eq96_e1383_d_b42: f64 = ((p.p6 * s.db[379][42]) * (nv21 - nv22));
        let eq96_e1383_d_b43: f64 = ((p.p6 * s.db[379][43]) * (nv21 - nv22));
        let eq96_e1383_d_b44: f64 = ((p.p6 * s.db[379][44]) * (nv21 - nv22));
        let eq96_e1383_d_b45: f64 = ((p.p6 * s.db[379][45]) * (nv21 - nv22));
        let eq96_e1383_d_b46: f64 = ((p.p6 * s.db[379][46]) * (nv21 - nv22));
        let eq96_e1383_d_b47: f64 = ((p.p6 * s.db[379][47]) * (nv21 - nv22));
        let eq96_e1383_d_b48: f64 = ((p.p6 * s.db[379][48]) * (nv21 - nv22));
        let eq96_e1383_d_b49: f64 = ((p.p6 * s.db[379][49]) * (nv21 - nv22));
        let eq96_e1383_d_b50: f64 = ((p.p6 * s.db[379][50]) * (nv21 - nv22));
        let eq96_e1383_d_b51: f64 = ((p.p6 * s.db[379][51]) * (nv21 - nv22));
        let eq96_e1383_d_b52: f64 = ((p.p6 * s.db[379][52]) * (nv21 - nv22));
        let eq96_e1383_d_b53: f64 = ((p.p6 * s.db[379][53]) * (nv21 - nv22));
        let eq96_e1383_d_b54: f64 = ((p.p6 * s.db[379][54]) * (nv21 - nv22));
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
        let eq96_e1384_d_b0: f64 = (eq96_e1378_d_b0 + eq96_e1383_d_b0);
        let eq96_e1384_d_b1: f64 = (eq96_e1378_d_b1 + eq96_e1383_d_b1);
        let eq96_e1384_d_b2: f64 = (eq96_e1378_d_b2 + eq96_e1383_d_b2);
        let eq96_e1384_d_b3: f64 = (eq96_e1378_d_b3 + eq96_e1383_d_b3);
        let eq96_e1384_d_b4: f64 = (eq96_e1378_d_b4 + eq96_e1383_d_b4);
        let eq96_e1384_d_b5: f64 = (eq96_e1378_d_b5 + eq96_e1383_d_b5);
        let eq96_e1384_d_b6: f64 = (eq96_e1378_d_b6 + eq96_e1383_d_b6);
        let eq96_e1384_d_b7: f64 = (eq96_e1378_d_b7 + eq96_e1383_d_b7);
        let eq96_e1384_d_b8: f64 = (eq96_e1378_d_b8 + eq96_e1383_d_b8);
        let eq96_e1384_d_b9: f64 = (eq96_e1378_d_b9 + eq96_e1383_d_b9);
        let eq96_e1384_d_b10: f64 = (eq96_e1378_d_b10 + eq96_e1383_d_b10);
        let eq96_e1384_d_b11: f64 = (eq96_e1378_d_b11 + eq96_e1383_d_b11);
        let eq96_e1384_d_b12: f64 = (eq96_e1378_d_b12 + eq96_e1383_d_b12);
        let eq96_e1384_d_b13: f64 = (eq96_e1378_d_b13 + eq96_e1383_d_b13);
        let eq96_e1384_d_b14: f64 = (eq96_e1378_d_b14 + eq96_e1383_d_b14);
        let eq96_e1384_d_b15: f64 = (eq96_e1378_d_b15 + eq96_e1383_d_b15);
        let eq96_e1384_d_b16: f64 = (eq96_e1378_d_b16 + eq96_e1383_d_b16);
        let eq96_e1384_d_b17: f64 = (eq96_e1378_d_b17 + eq96_e1383_d_b17);
        let eq96_e1384_d_b18: f64 = (eq96_e1378_d_b18 + eq96_e1383_d_b18);
        let eq96_e1384_d_b19: f64 = (eq96_e1378_d_b19 + eq96_e1383_d_b19);
        let eq96_e1384_d_b20: f64 = (eq96_e1378_d_b20 + eq96_e1383_d_b20);
        let eq96_e1384_d_b21: f64 = (eq96_e1378_d_b21 + eq96_e1383_d_b21);
        let eq96_e1384_d_b22: f64 = (eq96_e1378_d_b22 + eq96_e1383_d_b22);
        let eq96_e1384_d_b23: f64 = (eq96_e1378_d_b23 + eq96_e1383_d_b23);
        let eq96_e1384_d_b24: f64 = (eq96_e1378_d_b24 + eq96_e1383_d_b24);
        let eq96_e1384_d_b25: f64 = (eq96_e1378_d_b25 + eq96_e1383_d_b25);
        let eq96_e1384_d_b26: f64 = (eq96_e1378_d_b26 + eq96_e1383_d_b26);
        let eq96_e1384_d_b27: f64 = (eq96_e1378_d_b27 + eq96_e1383_d_b27);
        let eq96_e1384_d_b28: f64 = (eq96_e1378_d_b28 + eq96_e1383_d_b28);
        let eq96_e1384_d_b29: f64 = (eq96_e1378_d_b29 + eq96_e1383_d_b29);
        let eq96_e1384_d_b30: f64 = (eq96_e1378_d_b30 + eq96_e1383_d_b30);
        let eq96_e1384_d_b31: f64 = (eq96_e1378_d_b31 + eq96_e1383_d_b31);
        let eq96_e1384_d_b32: f64 = (eq96_e1378_d_b32 + eq96_e1383_d_b32);
        let eq96_e1384_d_b33: f64 = (eq96_e1378_d_b33 + eq96_e1383_d_b33);
        let eq96_e1384_d_b34: f64 = (eq96_e1378_d_b34 + eq96_e1383_d_b34);
        let eq96_e1384_d_b35: f64 = (eq96_e1378_d_b35 + eq96_e1383_d_b35);
        let eq96_e1384_d_b36: f64 = (eq96_e1378_d_b36 + eq96_e1383_d_b36);
        let eq96_e1384_d_b37: f64 = (eq96_e1378_d_b37 + eq96_e1383_d_b37);
        let eq96_e1384_d_b38: f64 = (eq96_e1378_d_b38 + eq96_e1383_d_b38);
        let eq96_e1384_d_b39: f64 = (eq96_e1378_d_b39 + eq96_e1383_d_b39);
        let eq96_e1384_d_b40: f64 = (eq96_e1378_d_b40 + eq96_e1383_d_b40);
        let eq96_e1384_d_b41: f64 = (eq96_e1378_d_b41 + eq96_e1383_d_b41);
        let eq96_e1384_d_b42: f64 = (eq96_e1378_d_b42 + eq96_e1383_d_b42);
        let eq96_e1384_d_b43: f64 = (eq96_e1378_d_b43 + eq96_e1383_d_b43);
        let eq96_e1384_d_b44: f64 = (eq96_e1378_d_b44 + eq96_e1383_d_b44);
        let eq96_e1384_d_b45: f64 = (eq96_e1378_d_b45 + eq96_e1383_d_b45);
        let eq96_e1384_d_b46: f64 = (eq96_e1378_d_b46 + eq96_e1383_d_b46);
        let eq96_e1384_d_b47: f64 = (eq96_e1378_d_b47 + eq96_e1383_d_b47);
        let eq96_e1384_d_b48: f64 = (eq96_e1378_d_b48 + eq96_e1383_d_b48);
        let eq96_e1384_d_b49: f64 = (eq96_e1378_d_b49 + eq96_e1383_d_b49);
        let eq96_e1384_d_b50: f64 = (eq96_e1378_d_b50 + eq96_e1383_d_b50);
        let eq96_e1384_d_b51: f64 = (eq96_e1378_d_b51 + eq96_e1383_d_b51);
        let eq96_e1384_d_b52: f64 = (eq96_e1378_d_b52 + eq96_e1383_d_b52);
        let eq96_e1384_d_b53: f64 = (eq96_e1378_d_b53 + eq96_e1383_d_b53);
        let eq96_e1384_d_b54: f64 = (eq96_e1378_d_b54 + eq96_e1383_d_b54);
        (eq96_e1384, eq96_e1384_d_n0, eq96_e1384_d_n1, eq96_e1384_d_n2, eq96_e1384_d_n3, eq96_e1384_d_n4, eq96_e1384_d_n5, eq96_e1384_d_n6, eq96_e1384_d_n7, eq96_e1384_d_n8, eq96_e1384_d_n9, eq96_e1384_d_n10, eq96_e1384_d_n11, eq96_e1384_d_n12, eq96_e1384_d_n13, eq96_e1384_d_n14, eq96_e1384_d_n15, eq96_e1384_d_n16, eq96_e1384_d_n17, eq96_e1384_d_n18, eq96_e1384_d_n19, eq96_e1384_d_n20, eq96_e1384_d_n21, eq96_e1384_d_n22, eq96_e1384_d_b0, eq96_e1384_d_b1, eq96_e1384_d_b2, eq96_e1384_d_b3, eq96_e1384_d_b4, eq96_e1384_d_b5, eq96_e1384_d_b6, eq96_e1384_d_b7, eq96_e1384_d_b8, eq96_e1384_d_b9, eq96_e1384_d_b10, eq96_e1384_d_b11, eq96_e1384_d_b12, eq96_e1384_d_b13, eq96_e1384_d_b14, eq96_e1384_d_b15, eq96_e1384_d_b16, eq96_e1384_d_b17, eq96_e1384_d_b18, eq96_e1384_d_b19, eq96_e1384_d_b20, eq96_e1384_d_b21, eq96_e1384_d_b22, eq96_e1384_d_b23, eq96_e1384_d_b24, eq96_e1384_d_b25, eq96_e1384_d_b26, eq96_e1384_d_b27, eq96_e1384_d_b28, eq96_e1384_d_b29, eq96_e1384_d_b30, eq96_e1384_d_b31, eq96_e1384_d_b32, eq96_e1384_d_b33, eq96_e1384_d_b34, eq96_e1384_d_b35, eq96_e1384_d_b36, eq96_e1384_d_b37, eq96_e1384_d_b38, eq96_e1384_d_b39, eq96_e1384_d_b40, eq96_e1384_d_b41, eq96_e1384_d_b42, eq96_e1384_d_b43, eq96_e1384_d_b44, eq96_e1384_d_b45, eq96_e1384_d_b46, eq96_e1384_d_b47, eq96_e1384_d_b48, eq96_e1384_d_b49, eq96_e1384_d_b50, eq96_e1384_d_b51, eq96_e1384_d_b52, eq96_e1384_d_b53, eq96_e1384_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1386;
        let eq96_node_derivatives: [f64; 23] = [eq96_e1386_d_n0, eq96_e1386_d_n1, eq96_e1386_d_n2, eq96_e1386_d_n3, eq96_e1386_d_n4, eq96_e1386_d_n5, eq96_e1386_d_n6, eq96_e1386_d_n7, eq96_e1386_d_n8, eq96_e1386_d_n9, eq96_e1386_d_n10, eq96_e1386_d_n11, eq96_e1386_d_n12, eq96_e1386_d_n13, eq96_e1386_d_n14, eq96_e1386_d_n15, eq96_e1386_d_n16, eq96_e1386_d_n17, eq96_e1386_d_n18, eq96_e1386_d_n19, eq96_e1386_d_n20, eq96_e1386_d_n21, eq96_e1386_d_n22];
        let eq96_branch_derivatives: [f64; 55] = [eq96_e1386_d_b0, eq96_e1386_d_b1, eq96_e1386_d_b2, eq96_e1386_d_b3, eq96_e1386_d_b4, eq96_e1386_d_b5, eq96_e1386_d_b6, eq96_e1386_d_b7, eq96_e1386_d_b8, eq96_e1386_d_b9, eq96_e1386_d_b10, eq96_e1386_d_b11, eq96_e1386_d_b12, eq96_e1386_d_b13, eq96_e1386_d_b14, eq96_e1386_d_b15, eq96_e1386_d_b16, eq96_e1386_d_b17, eq96_e1386_d_b18, eq96_e1386_d_b19, eq96_e1386_d_b20, eq96_e1386_d_b21, eq96_e1386_d_b22, eq96_e1386_d_b23, eq96_e1386_d_b24, eq96_e1386_d_b25, eq96_e1386_d_b26, eq96_e1386_d_b27, eq96_e1386_d_b28, eq96_e1386_d_b29, eq96_e1386_d_b30, eq96_e1386_d_b31, eq96_e1386_d_b32, eq96_e1386_d_b33, eq96_e1386_d_b34, eq96_e1386_d_b35, eq96_e1386_d_b36, eq96_e1386_d_b37, eq96_e1386_d_b38, eq96_e1386_d_b39, eq96_e1386_d_b40, eq96_e1386_d_b41, eq96_e1386_d_b42, eq96_e1386_d_b43, eq96_e1386_d_b44, eq96_e1386_d_b45, eq96_e1386_d_b46, eq96_e1386_d_b47, eq96_e1386_d_b48, eq96_e1386_d_b49, eq96_e1386_d_b50, eq96_e1386_d_b51, eq96_e1386_d_b52, eq96_e1386_d_b53, eq96_e1386_d_b54];
        stamper.stamp_current_dense_local(
            Some(21),
            Some(22),
            multiplicity * (eq96_value),
            &eq96_node_derivatives,
            &eq96_branch_derivatives,
            multiplicity,
        );
        let eq106_e1459: f64 = (p.p6 * s.v[369]);
        let eq106_value: f64 = eq106_e1459;
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * (eq106_value),
            &s.dn[369],
            &s.db[369],
            (multiplicity) * (p.p6),
        );
        let eq107_e1462: f64 = (p.p6 * s.v[370]);
        let eq107_value: f64 = eq107_e1462;
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * (eq107_value),
            &s.dn[370],
            &s.db[370],
            (multiplicity) * (p.p6),
        );
        let eq109_e1474: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, s.v[165]);
        let eq109_e1475: f64 = (p.p7 * eq109_e1474);
        let eq109_e1475_d_n0: f64 = (p.p7 * (s.dn[165][0] * ddt_scale));
        let eq109_e1475_d_n1: f64 = (p.p7 * (s.dn[165][1] * ddt_scale));
        let eq109_e1475_d_n2: f64 = (p.p7 * (s.dn[165][2] * ddt_scale));
        let eq109_e1475_d_n3: f64 = (p.p7 * (s.dn[165][3] * ddt_scale));
        let eq109_e1475_d_n4: f64 = (p.p7 * (s.dn[165][4] * ddt_scale));
        let eq109_e1475_d_n5: f64 = (p.p7 * (s.dn[165][5] * ddt_scale));
        let eq109_e1475_d_n6: f64 = (p.p7 * (s.dn[165][6] * ddt_scale));
        let eq109_e1475_d_n7: f64 = (p.p7 * (s.dn[165][7] * ddt_scale));
        let eq109_e1475_d_n8: f64 = (p.p7 * (s.dn[165][8] * ddt_scale));
        let eq109_e1475_d_n9: f64 = (p.p7 * (s.dn[165][9] * ddt_scale));
        let eq109_e1475_d_n10: f64 = (p.p7 * (s.dn[165][10] * ddt_scale));
        let eq109_e1475_d_n11: f64 = (p.p7 * (s.dn[165][11] * ddt_scale));
        let eq109_e1475_d_n12: f64 = (p.p7 * (s.dn[165][12] * ddt_scale));
        let eq109_e1475_d_n13: f64 = (p.p7 * (s.dn[165][13] * ddt_scale));
        let eq109_e1475_d_n14: f64 = (p.p7 * (s.dn[165][14] * ddt_scale));
        let eq109_e1475_d_n15: f64 = (p.p7 * (s.dn[165][15] * ddt_scale));
        let eq109_e1475_d_n16: f64 = (p.p7 * (s.dn[165][16] * ddt_scale));
        let eq109_e1475_d_n17: f64 = (p.p7 * (s.dn[165][17] * ddt_scale));
        let eq109_e1475_d_n18: f64 = (p.p7 * (s.dn[165][18] * ddt_scale));
        let eq109_e1475_d_n19: f64 = (p.p7 * (s.dn[165][19] * ddt_scale));
        let eq109_e1475_d_n20: f64 = (p.p7 * (s.dn[165][20] * ddt_scale));
        let eq109_e1475_d_n21: f64 = (p.p7 * (s.dn[165][21] * ddt_scale));
        let eq109_e1475_d_n22: f64 = (p.p7 * (s.dn[165][22] * ddt_scale));
        let eq109_e1475_d_b0: f64 = (p.p7 * (s.db[165][0] * ddt_scale));
        let eq109_e1475_d_b1: f64 = (p.p7 * (s.db[165][1] * ddt_scale));
        let eq109_e1475_d_b2: f64 = (p.p7 * (s.db[165][2] * ddt_scale));
        let eq109_e1475_d_b3: f64 = (p.p7 * (s.db[165][3] * ddt_scale));
        let eq109_e1475_d_b4: f64 = (p.p7 * (s.db[165][4] * ddt_scale));
        let eq109_e1475_d_b5: f64 = (p.p7 * (s.db[165][5] * ddt_scale));
        let eq109_e1475_d_b6: f64 = (p.p7 * (s.db[165][6] * ddt_scale));
        let eq109_e1475_d_b7: f64 = (p.p7 * (s.db[165][7] * ddt_scale));
        let eq109_e1475_d_b8: f64 = (p.p7 * (s.db[165][8] * ddt_scale));
        let eq109_e1475_d_b9: f64 = (p.p7 * (s.db[165][9] * ddt_scale));
        let eq109_e1475_d_b10: f64 = (p.p7 * (s.db[165][10] * ddt_scale));
        let eq109_e1475_d_b11: f64 = (p.p7 * (s.db[165][11] * ddt_scale));
        let eq109_e1475_d_b12: f64 = (p.p7 * (s.db[165][12] * ddt_scale));
        let eq109_e1475_d_b13: f64 = (p.p7 * (s.db[165][13] * ddt_scale));
        let eq109_e1475_d_b14: f64 = (p.p7 * (s.db[165][14] * ddt_scale));
        let eq109_e1475_d_b15: f64 = (p.p7 * (s.db[165][15] * ddt_scale));
        let eq109_e1475_d_b16: f64 = (p.p7 * (s.db[165][16] * ddt_scale));
        let eq109_e1475_d_b17: f64 = (p.p7 * (s.db[165][17] * ddt_scale));
        let eq109_e1475_d_b18: f64 = (p.p7 * (s.db[165][18] * ddt_scale));
        let eq109_e1475_d_b19: f64 = (p.p7 * (s.db[165][19] * ddt_scale));
        let eq109_e1475_d_b20: f64 = (p.p7 * (s.db[165][20] * ddt_scale));
        let eq109_e1475_d_b21: f64 = (p.p7 * (s.db[165][21] * ddt_scale));
        let eq109_e1475_d_b22: f64 = (p.p7 * (s.db[165][22] * ddt_scale));
        let eq109_e1475_d_b23: f64 = (p.p7 * (s.db[165][23] * ddt_scale));
        let eq109_e1475_d_b24: f64 = (p.p7 * (s.db[165][24] * ddt_scale));
        let eq109_e1475_d_b25: f64 = (p.p7 * (s.db[165][25] * ddt_scale));
        let eq109_e1475_d_b26: f64 = (p.p7 * (s.db[165][26] * ddt_scale));
        let eq109_e1475_d_b27: f64 = (p.p7 * (s.db[165][27] * ddt_scale));
        let eq109_e1475_d_b28: f64 = (p.p7 * (s.db[165][28] * ddt_scale));
        let eq109_e1475_d_b29: f64 = (p.p7 * (s.db[165][29] * ddt_scale));
        let eq109_e1475_d_b30: f64 = (p.p7 * (s.db[165][30] * ddt_scale));
        let eq109_e1475_d_b31: f64 = (p.p7 * (s.db[165][31] * ddt_scale));
        let eq109_e1475_d_b32: f64 = (p.p7 * (s.db[165][32] * ddt_scale));
        let eq109_e1475_d_b33: f64 = (p.p7 * (s.db[165][33] * ddt_scale));
        let eq109_e1475_d_b34: f64 = (p.p7 * (s.db[165][34] * ddt_scale));
        let eq109_e1475_d_b35: f64 = (p.p7 * (s.db[165][35] * ddt_scale));
        let eq109_e1475_d_b36: f64 = (p.p7 * (s.db[165][36] * ddt_scale));
        let eq109_e1475_d_b37: f64 = (p.p7 * (s.db[165][37] * ddt_scale));
        let eq109_e1475_d_b38: f64 = (p.p7 * (s.db[165][38] * ddt_scale));
        let eq109_e1475_d_b39: f64 = (p.p7 * (s.db[165][39] * ddt_scale));
        let eq109_e1475_d_b40: f64 = (p.p7 * (s.db[165][40] * ddt_scale));
        let eq109_e1475_d_b41: f64 = (p.p7 * (s.db[165][41] * ddt_scale));
        let eq109_e1475_d_b42: f64 = (p.p7 * (s.db[165][42] * ddt_scale));
        let eq109_e1475_d_b43: f64 = (p.p7 * (s.db[165][43] * ddt_scale));
        let eq109_e1475_d_b44: f64 = (p.p7 * (s.db[165][44] * ddt_scale));
        let eq109_e1475_d_b45: f64 = (p.p7 * (s.db[165][45] * ddt_scale));
        let eq109_e1475_d_b46: f64 = (p.p7 * (s.db[165][46] * ddt_scale));
        let eq109_e1475_d_b47: f64 = (p.p7 * (s.db[165][47] * ddt_scale));
        let eq109_e1475_d_b48: f64 = (p.p7 * (s.db[165][48] * ddt_scale));
        let eq109_e1475_d_b49: f64 = (p.p7 * (s.db[165][49] * ddt_scale));
        let eq109_e1475_d_b50: f64 = (p.p7 * (s.db[165][50] * ddt_scale));
        let eq109_e1475_d_b51: f64 = (p.p7 * (s.db[165][51] * ddt_scale));
        let eq109_e1475_d_b52: f64 = (p.p7 * (s.db[165][52] * ddt_scale));
        let eq109_e1475_d_b53: f64 = (p.p7 * (s.db[165][53] * ddt_scale));
        let eq109_e1475_d_b54: f64 = (p.p7 * (s.db[165][54] * ddt_scale));
        let eq109_value: f64 = eq109_e1475;
        let eq109_node_derivatives: [f64; 23] = [eq109_e1475_d_n0, eq109_e1475_d_n1, eq109_e1475_d_n2, eq109_e1475_d_n3, eq109_e1475_d_n4, eq109_e1475_d_n5, eq109_e1475_d_n6, eq109_e1475_d_n7, eq109_e1475_d_n8, eq109_e1475_d_n9, eq109_e1475_d_n10, eq109_e1475_d_n11, eq109_e1475_d_n12, eq109_e1475_d_n13, eq109_e1475_d_n14, eq109_e1475_d_n15, eq109_e1475_d_n16, eq109_e1475_d_n17, eq109_e1475_d_n18, eq109_e1475_d_n19, eq109_e1475_d_n20, eq109_e1475_d_n21, eq109_e1475_d_n22];
        let eq109_branch_derivatives: [f64; 55] = [eq109_e1475_d_b0, eq109_e1475_d_b1, eq109_e1475_d_b2, eq109_e1475_d_b3, eq109_e1475_d_b4, eq109_e1475_d_b5, eq109_e1475_d_b6, eq109_e1475_d_b7, eq109_e1475_d_b8, eq109_e1475_d_b9, eq109_e1475_d_b10, eq109_e1475_d_b11, eq109_e1475_d_b12, eq109_e1475_d_b13, eq109_e1475_d_b14, eq109_e1475_d_b15, eq109_e1475_d_b16, eq109_e1475_d_b17, eq109_e1475_d_b18, eq109_e1475_d_b19, eq109_e1475_d_b20, eq109_e1475_d_b21, eq109_e1475_d_b22, eq109_e1475_d_b23, eq109_e1475_d_b24, eq109_e1475_d_b25, eq109_e1475_d_b26, eq109_e1475_d_b27, eq109_e1475_d_b28, eq109_e1475_d_b29, eq109_e1475_d_b30, eq109_e1475_d_b31, eq109_e1475_d_b32, eq109_e1475_d_b33, eq109_e1475_d_b34, eq109_e1475_d_b35, eq109_e1475_d_b36, eq109_e1475_d_b37, eq109_e1475_d_b38, eq109_e1475_d_b39, eq109_e1475_d_b40, eq109_e1475_d_b41, eq109_e1475_d_b42, eq109_e1475_d_b43, eq109_e1475_d_b44, eq109_e1475_d_b45, eq109_e1475_d_b46, eq109_e1475_d_b47, eq109_e1475_d_b48, eq109_e1475_d_b49, eq109_e1475_d_b50, eq109_e1475_d_b51, eq109_e1475_d_b52, eq109_e1475_d_b53, eq109_e1475_d_b54];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq109_value),
            &eq109_node_derivatives,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let eq110_e1478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, s.v[161]);
        let eq110_e1479: f64 = (p.p7 * eq110_e1478);
        let eq110_e1479_d_n0: f64 = (p.p7 * (s.dn[161][0] * ddt_scale));
        let eq110_e1479_d_n1: f64 = (p.p7 * (s.dn[161][1] * ddt_scale));
        let eq110_e1479_d_n2: f64 = (p.p7 * (s.dn[161][2] * ddt_scale));
        let eq110_e1479_d_n3: f64 = (p.p7 * (s.dn[161][3] * ddt_scale));
        let eq110_e1479_d_n4: f64 = (p.p7 * (s.dn[161][4] * ddt_scale));
        let eq110_e1479_d_n5: f64 = (p.p7 * (s.dn[161][5] * ddt_scale));
        let eq110_e1479_d_n6: f64 = (p.p7 * (s.dn[161][6] * ddt_scale));
        let eq110_e1479_d_n7: f64 = (p.p7 * (s.dn[161][7] * ddt_scale));
        let eq110_e1479_d_n8: f64 = (p.p7 * (s.dn[161][8] * ddt_scale));
        let eq110_e1479_d_n9: f64 = (p.p7 * (s.dn[161][9] * ddt_scale));
        let eq110_e1479_d_n10: f64 = (p.p7 * (s.dn[161][10] * ddt_scale));
        let eq110_e1479_d_n11: f64 = (p.p7 * (s.dn[161][11] * ddt_scale));
        let eq110_e1479_d_n12: f64 = (p.p7 * (s.dn[161][12] * ddt_scale));
        let eq110_e1479_d_n13: f64 = (p.p7 * (s.dn[161][13] * ddt_scale));
        let eq110_e1479_d_n14: f64 = (p.p7 * (s.dn[161][14] * ddt_scale));
        let eq110_e1479_d_n15: f64 = (p.p7 * (s.dn[161][15] * ddt_scale));
        let eq110_e1479_d_n16: f64 = (p.p7 * (s.dn[161][16] * ddt_scale));
        let eq110_e1479_d_n17: f64 = (p.p7 * (s.dn[161][17] * ddt_scale));
        let eq110_e1479_d_n18: f64 = (p.p7 * (s.dn[161][18] * ddt_scale));
        let eq110_e1479_d_n19: f64 = (p.p7 * (s.dn[161][19] * ddt_scale));
        let eq110_e1479_d_n20: f64 = (p.p7 * (s.dn[161][20] * ddt_scale));
        let eq110_e1479_d_n21: f64 = (p.p7 * (s.dn[161][21] * ddt_scale));
        let eq110_e1479_d_n22: f64 = (p.p7 * (s.dn[161][22] * ddt_scale));
        let eq110_e1479_d_b0: f64 = (p.p7 * (s.db[161][0] * ddt_scale));
        let eq110_e1479_d_b1: f64 = (p.p7 * (s.db[161][1] * ddt_scale));
        let eq110_e1479_d_b2: f64 = (p.p7 * (s.db[161][2] * ddt_scale));
        let eq110_e1479_d_b3: f64 = (p.p7 * (s.db[161][3] * ddt_scale));
        let eq110_e1479_d_b4: f64 = (p.p7 * (s.db[161][4] * ddt_scale));
        let eq110_e1479_d_b5: f64 = (p.p7 * (s.db[161][5] * ddt_scale));
        let eq110_e1479_d_b6: f64 = (p.p7 * (s.db[161][6] * ddt_scale));
        let eq110_e1479_d_b7: f64 = (p.p7 * (s.db[161][7] * ddt_scale));
        let eq110_e1479_d_b8: f64 = (p.p7 * (s.db[161][8] * ddt_scale));
        let eq110_e1479_d_b9: f64 = (p.p7 * (s.db[161][9] * ddt_scale));
        let eq110_e1479_d_b10: f64 = (p.p7 * (s.db[161][10] * ddt_scale));
        let eq110_e1479_d_b11: f64 = (p.p7 * (s.db[161][11] * ddt_scale));
        let eq110_e1479_d_b12: f64 = (p.p7 * (s.db[161][12] * ddt_scale));
        let eq110_e1479_d_b13: f64 = (p.p7 * (s.db[161][13] * ddt_scale));
        let eq110_e1479_d_b14: f64 = (p.p7 * (s.db[161][14] * ddt_scale));
        let eq110_e1479_d_b15: f64 = (p.p7 * (s.db[161][15] * ddt_scale));
        let eq110_e1479_d_b16: f64 = (p.p7 * (s.db[161][16] * ddt_scale));
        let eq110_e1479_d_b17: f64 = (p.p7 * (s.db[161][17] * ddt_scale));
        let eq110_e1479_d_b18: f64 = (p.p7 * (s.db[161][18] * ddt_scale));
        let eq110_e1479_d_b19: f64 = (p.p7 * (s.db[161][19] * ddt_scale));
        let eq110_e1479_d_b20: f64 = (p.p7 * (s.db[161][20] * ddt_scale));
        let eq110_e1479_d_b21: f64 = (p.p7 * (s.db[161][21] * ddt_scale));
        let eq110_e1479_d_b22: f64 = (p.p7 * (s.db[161][22] * ddt_scale));
        let eq110_e1479_d_b23: f64 = (p.p7 * (s.db[161][23] * ddt_scale));
        let eq110_e1479_d_b24: f64 = (p.p7 * (s.db[161][24] * ddt_scale));
        let eq110_e1479_d_b25: f64 = (p.p7 * (s.db[161][25] * ddt_scale));
        let eq110_e1479_d_b26: f64 = (p.p7 * (s.db[161][26] * ddt_scale));
        let eq110_e1479_d_b27: f64 = (p.p7 * (s.db[161][27] * ddt_scale));
        let eq110_e1479_d_b28: f64 = (p.p7 * (s.db[161][28] * ddt_scale));
        let eq110_e1479_d_b29: f64 = (p.p7 * (s.db[161][29] * ddt_scale));
        let eq110_e1479_d_b30: f64 = (p.p7 * (s.db[161][30] * ddt_scale));
        let eq110_e1479_d_b31: f64 = (p.p7 * (s.db[161][31] * ddt_scale));
        let eq110_e1479_d_b32: f64 = (p.p7 * (s.db[161][32] * ddt_scale));
        let eq110_e1479_d_b33: f64 = (p.p7 * (s.db[161][33] * ddt_scale));
        let eq110_e1479_d_b34: f64 = (p.p7 * (s.db[161][34] * ddt_scale));
        let eq110_e1479_d_b35: f64 = (p.p7 * (s.db[161][35] * ddt_scale));
        let eq110_e1479_d_b36: f64 = (p.p7 * (s.db[161][36] * ddt_scale));
        let eq110_e1479_d_b37: f64 = (p.p7 * (s.db[161][37] * ddt_scale));
        let eq110_e1479_d_b38: f64 = (p.p7 * (s.db[161][38] * ddt_scale));
        let eq110_e1479_d_b39: f64 = (p.p7 * (s.db[161][39] * ddt_scale));
        let eq110_e1479_d_b40: f64 = (p.p7 * (s.db[161][40] * ddt_scale));
        let eq110_e1479_d_b41: f64 = (p.p7 * (s.db[161][41] * ddt_scale));
        let eq110_e1479_d_b42: f64 = (p.p7 * (s.db[161][42] * ddt_scale));
        let eq110_e1479_d_b43: f64 = (p.p7 * (s.db[161][43] * ddt_scale));
        let eq110_e1479_d_b44: f64 = (p.p7 * (s.db[161][44] * ddt_scale));
        let eq110_e1479_d_b45: f64 = (p.p7 * (s.db[161][45] * ddt_scale));
        let eq110_e1479_d_b46: f64 = (p.p7 * (s.db[161][46] * ddt_scale));
        let eq110_e1479_d_b47: f64 = (p.p7 * (s.db[161][47] * ddt_scale));
        let eq110_e1479_d_b48: f64 = (p.p7 * (s.db[161][48] * ddt_scale));
        let eq110_e1479_d_b49: f64 = (p.p7 * (s.db[161][49] * ddt_scale));
        let eq110_e1479_d_b50: f64 = (p.p7 * (s.db[161][50] * ddt_scale));
        let eq110_e1479_d_b51: f64 = (p.p7 * (s.db[161][51] * ddt_scale));
        let eq110_e1479_d_b52: f64 = (p.p7 * (s.db[161][52] * ddt_scale));
        let eq110_e1479_d_b53: f64 = (p.p7 * (s.db[161][53] * ddt_scale));
        let eq110_e1479_d_b54: f64 = (p.p7 * (s.db[161][54] * ddt_scale));
        let eq110_value: f64 = eq110_e1479;
        let eq110_node_derivatives: [f64; 23] = [eq110_e1479_d_n0, eq110_e1479_d_n1, eq110_e1479_d_n2, eq110_e1479_d_n3, eq110_e1479_d_n4, eq110_e1479_d_n5, eq110_e1479_d_n6, eq110_e1479_d_n7, eq110_e1479_d_n8, eq110_e1479_d_n9, eq110_e1479_d_n10, eq110_e1479_d_n11, eq110_e1479_d_n12, eq110_e1479_d_n13, eq110_e1479_d_n14, eq110_e1479_d_n15, eq110_e1479_d_n16, eq110_e1479_d_n17, eq110_e1479_d_n18, eq110_e1479_d_n19, eq110_e1479_d_n20, eq110_e1479_d_n21, eq110_e1479_d_n22];
        let eq110_branch_derivatives: [f64; 55] = [eq110_e1479_d_b0, eq110_e1479_d_b1, eq110_e1479_d_b2, eq110_e1479_d_b3, eq110_e1479_d_b4, eq110_e1479_d_b5, eq110_e1479_d_b6, eq110_e1479_d_b7, eq110_e1479_d_b8, eq110_e1479_d_b9, eq110_e1479_d_b10, eq110_e1479_d_b11, eq110_e1479_d_b12, eq110_e1479_d_b13, eq110_e1479_d_b14, eq110_e1479_d_b15, eq110_e1479_d_b16, eq110_e1479_d_b17, eq110_e1479_d_b18, eq110_e1479_d_b19, eq110_e1479_d_b20, eq110_e1479_d_b21, eq110_e1479_d_b22, eq110_e1479_d_b23, eq110_e1479_d_b24, eq110_e1479_d_b25, eq110_e1479_d_b26, eq110_e1479_d_b27, eq110_e1479_d_b28, eq110_e1479_d_b29, eq110_e1479_d_b30, eq110_e1479_d_b31, eq110_e1479_d_b32, eq110_e1479_d_b33, eq110_e1479_d_b34, eq110_e1479_d_b35, eq110_e1479_d_b36, eq110_e1479_d_b37, eq110_e1479_d_b38, eq110_e1479_d_b39, eq110_e1479_d_b40, eq110_e1479_d_b41, eq110_e1479_d_b42, eq110_e1479_d_b43, eq110_e1479_d_b44, eq110_e1479_d_b45, eq110_e1479_d_b46, eq110_e1479_d_b47, eq110_e1479_d_b48, eq110_e1479_d_b49, eq110_e1479_d_b50, eq110_e1479_d_b51, eq110_e1479_d_b52, eq110_e1479_d_b53, eq110_e1479_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq110_value),
            &eq110_node_derivatives,
            &eq110_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_13(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[162][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[162][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[162][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[162][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[162][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[162][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[162][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[162][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[162][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[162][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[162][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[162][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[162][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[162][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[162][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[162][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[162][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[162][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[162][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[162][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[162][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[162][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[162][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[162][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[162][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[162][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[162][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[162][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[162][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[162][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[162][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[162][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[162][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[162][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[162][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[162][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[162][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[162][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[162][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[162][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[162][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[162][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[162][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[162][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[162][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[162][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[162][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[162][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[162][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[162][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[162][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[162][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[162][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[162][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[162][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[162][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[162][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[162][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[162][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[162][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[162][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[162][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[162][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[162][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[162][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[162][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[162][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[162][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[162][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[162][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[162][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[162][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[162][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[162][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[162][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[162][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[162][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[162][54] * ddt_scale));
        let __rspice_deriv_cse_78: f64 = (p.p7 * (s.dn[163][0] * ddt_scale));
        let __rspice_deriv_cse_79: f64 = (p.p7 * (s.dn[163][1] * ddt_scale));
        let __rspice_deriv_cse_80: f64 = (p.p7 * (s.dn[163][2] * ddt_scale));
        let __rspice_deriv_cse_81: f64 = (p.p7 * (s.dn[163][3] * ddt_scale));
        let __rspice_deriv_cse_82: f64 = (p.p7 * (s.dn[163][4] * ddt_scale));
        let __rspice_deriv_cse_83: f64 = (p.p7 * (s.dn[163][5] * ddt_scale));
        let __rspice_deriv_cse_84: f64 = (p.p7 * (s.dn[163][6] * ddt_scale));
        let __rspice_deriv_cse_85: f64 = (p.p7 * (s.dn[163][7] * ddt_scale));
        let __rspice_deriv_cse_86: f64 = (p.p7 * (s.dn[163][8] * ddt_scale));
        let __rspice_deriv_cse_87: f64 = (p.p7 * (s.dn[163][9] * ddt_scale));
        let __rspice_deriv_cse_88: f64 = (p.p7 * (s.dn[163][10] * ddt_scale));
        let __rspice_deriv_cse_89: f64 = (p.p7 * (s.dn[163][11] * ddt_scale));
        let __rspice_deriv_cse_90: f64 = (p.p7 * (s.dn[163][12] * ddt_scale));
        let __rspice_deriv_cse_91: f64 = (p.p7 * (s.dn[163][13] * ddt_scale));
        let __rspice_deriv_cse_92: f64 = (p.p7 * (s.dn[163][14] * ddt_scale));
        let __rspice_deriv_cse_93: f64 = (p.p7 * (s.dn[163][15] * ddt_scale));
        let __rspice_deriv_cse_94: f64 = (p.p7 * (s.dn[163][16] * ddt_scale));
        let __rspice_deriv_cse_95: f64 = (p.p7 * (s.dn[163][17] * ddt_scale));
        let __rspice_deriv_cse_96: f64 = (p.p7 * (s.dn[163][18] * ddt_scale));
        let __rspice_deriv_cse_97: f64 = (p.p7 * (s.dn[163][19] * ddt_scale));
        let __rspice_deriv_cse_98: f64 = (p.p7 * (s.dn[163][20] * ddt_scale));
        let __rspice_deriv_cse_99: f64 = (p.p7 * (s.dn[163][21] * ddt_scale));
        let __rspice_deriv_cse_100: f64 = (p.p7 * (s.dn[163][22] * ddt_scale));
        let __rspice_deriv_cse_101: f64 = (p.p7 * (s.db[163][0] * ddt_scale));
        let __rspice_deriv_cse_102: f64 = (p.p7 * (s.db[163][1] * ddt_scale));
        let __rspice_deriv_cse_103: f64 = (p.p7 * (s.db[163][2] * ddt_scale));
        let __rspice_deriv_cse_104: f64 = (p.p7 * (s.db[163][3] * ddt_scale));
        let __rspice_deriv_cse_105: f64 = (p.p7 * (s.db[163][4] * ddt_scale));
        let __rspice_deriv_cse_106: f64 = (p.p7 * (s.db[163][5] * ddt_scale));
        let __rspice_deriv_cse_107: f64 = (p.p7 * (s.db[163][6] * ddt_scale));
        let __rspice_deriv_cse_108: f64 = (p.p7 * (s.db[163][7] * ddt_scale));
        let __rspice_deriv_cse_109: f64 = (p.p7 * (s.db[163][8] * ddt_scale));
        let __rspice_deriv_cse_110: f64 = (p.p7 * (s.db[163][9] * ddt_scale));
        let __rspice_deriv_cse_111: f64 = (p.p7 * (s.db[163][10] * ddt_scale));
        let __rspice_deriv_cse_112: f64 = (p.p7 * (s.db[163][11] * ddt_scale));
        let __rspice_deriv_cse_113: f64 = (p.p7 * (s.db[163][12] * ddt_scale));
        let __rspice_deriv_cse_114: f64 = (p.p7 * (s.db[163][13] * ddt_scale));
        let __rspice_deriv_cse_115: f64 = (p.p7 * (s.db[163][14] * ddt_scale));
        let __rspice_deriv_cse_116: f64 = (p.p7 * (s.db[163][15] * ddt_scale));
        let __rspice_deriv_cse_117: f64 = (p.p7 * (s.db[163][16] * ddt_scale));
        let __rspice_deriv_cse_118: f64 = (p.p7 * (s.db[163][17] * ddt_scale));
        let __rspice_deriv_cse_119: f64 = (p.p7 * (s.db[163][18] * ddt_scale));
        let __rspice_deriv_cse_120: f64 = (p.p7 * (s.db[163][19] * ddt_scale));
        let __rspice_deriv_cse_121: f64 = (p.p7 * (s.db[163][20] * ddt_scale));
        let __rspice_deriv_cse_122: f64 = (p.p7 * (s.db[163][21] * ddt_scale));
        let __rspice_deriv_cse_123: f64 = (p.p7 * (s.db[163][22] * ddt_scale));
        let __rspice_deriv_cse_124: f64 = (p.p7 * (s.db[163][23] * ddt_scale));
        let __rspice_deriv_cse_125: f64 = (p.p7 * (s.db[163][24] * ddt_scale));
        let __rspice_deriv_cse_126: f64 = (p.p7 * (s.db[163][25] * ddt_scale));
        let __rspice_deriv_cse_127: f64 = (p.p7 * (s.db[163][26] * ddt_scale));
        let __rspice_deriv_cse_128: f64 = (p.p7 * (s.db[163][27] * ddt_scale));
        let __rspice_deriv_cse_129: f64 = (p.p7 * (s.db[163][28] * ddt_scale));
        let __rspice_deriv_cse_130: f64 = (p.p7 * (s.db[163][29] * ddt_scale));
        let __rspice_deriv_cse_131: f64 = (p.p7 * (s.db[163][30] * ddt_scale));
        let __rspice_deriv_cse_132: f64 = (p.p7 * (s.db[163][31] * ddt_scale));
        let __rspice_deriv_cse_133: f64 = (p.p7 * (s.db[163][32] * ddt_scale));
        let __rspice_deriv_cse_134: f64 = (p.p7 * (s.db[163][33] * ddt_scale));
        let __rspice_deriv_cse_135: f64 = (p.p7 * (s.db[163][34] * ddt_scale));
        let __rspice_deriv_cse_136: f64 = (p.p7 * (s.db[163][35] * ddt_scale));
        let __rspice_deriv_cse_137: f64 = (p.p7 * (s.db[163][36] * ddt_scale));
        let __rspice_deriv_cse_138: f64 = (p.p7 * (s.db[163][37] * ddt_scale));
        let __rspice_deriv_cse_139: f64 = (p.p7 * (s.db[163][38] * ddt_scale));
        let __rspice_deriv_cse_140: f64 = (p.p7 * (s.db[163][39] * ddt_scale));
        let __rspice_deriv_cse_141: f64 = (p.p7 * (s.db[163][40] * ddt_scale));
        let __rspice_deriv_cse_142: f64 = (p.p7 * (s.db[163][41] * ddt_scale));
        let __rspice_deriv_cse_143: f64 = (p.p7 * (s.db[163][42] * ddt_scale));
        let __rspice_deriv_cse_144: f64 = (p.p7 * (s.db[163][43] * ddt_scale));
        let __rspice_deriv_cse_145: f64 = (p.p7 * (s.db[163][44] * ddt_scale));
        let __rspice_deriv_cse_146: f64 = (p.p7 * (s.db[163][45] * ddt_scale));
        let __rspice_deriv_cse_147: f64 = (p.p7 * (s.db[163][46] * ddt_scale));
        let __rspice_deriv_cse_148: f64 = (p.p7 * (s.db[163][47] * ddt_scale));
        let __rspice_deriv_cse_149: f64 = (p.p7 * (s.db[163][48] * ddt_scale));
        let __rspice_deriv_cse_150: f64 = (p.p7 * (s.db[163][49] * ddt_scale));
        let __rspice_deriv_cse_151: f64 = (p.p7 * (s.db[163][50] * ddt_scale));
        let __rspice_deriv_cse_152: f64 = (p.p7 * (s.db[163][51] * ddt_scale));
        let __rspice_deriv_cse_153: f64 = (p.p7 * (s.db[163][52] * ddt_scale));
        let __rspice_deriv_cse_154: f64 = (p.p7 * (s.db[163][53] * ddt_scale));
        let __rspice_deriv_cse_155: f64 = (p.p7 * (s.db[163][54] * ddt_scale));
        let (eq111_e1486, eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22, eq111_e1486_d_b0, eq111_e1486_d_b1, eq111_e1486_d_b2, eq111_e1486_d_b3, eq111_e1486_d_b4, eq111_e1486_d_b5, eq111_e1486_d_b6, eq111_e1486_d_b7, eq111_e1486_d_b8, eq111_e1486_d_b9, eq111_e1486_d_b10, eq111_e1486_d_b11, eq111_e1486_d_b12, eq111_e1486_d_b13, eq111_e1486_d_b14, eq111_e1486_d_b15, eq111_e1486_d_b16, eq111_e1486_d_b17, eq111_e1486_d_b18, eq111_e1486_d_b19, eq111_e1486_d_b20, eq111_e1486_d_b21, eq111_e1486_d_b22, eq111_e1486_d_b23, eq111_e1486_d_b24, eq111_e1486_d_b25, eq111_e1486_d_b26, eq111_e1486_d_b27, eq111_e1486_d_b28, eq111_e1486_d_b29, eq111_e1486_d_b30, eq111_e1486_d_b31, eq111_e1486_d_b32, eq111_e1486_d_b33, eq111_e1486_d_b34, eq111_e1486_d_b35, eq111_e1486_d_b36, eq111_e1486_d_b37, eq111_e1486_d_b38, eq111_e1486_d_b39, eq111_e1486_d_b40, eq111_e1486_d_b41, eq111_e1486_d_b42, eq111_e1486_d_b43, eq111_e1486_d_b44, eq111_e1486_d_b45, eq111_e1486_d_b46, eq111_e1486_d_b47, eq111_e1486_d_b48, eq111_e1486_d_b49, eq111_e1486_d_b50, eq111_e1486_d_b51, eq111_e1486_d_b52, eq111_e1486_d_b53, eq111_e1486_d_b54,) = {
    if s.b[569] {
        let eq111_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, s.v[162]);
        let eq111_e1484: f64 = (p.p7 * eq111_e1483);
        (eq111_e1484, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1486;
        let eq111_node_derivatives: [f64; 23] = [eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22];
        let eq111_branch_derivatives: [f64; 55] = [eq111_e1486_d_b0, eq111_e1486_d_b1, eq111_e1486_d_b2, eq111_e1486_d_b3, eq111_e1486_d_b4, eq111_e1486_d_b5, eq111_e1486_d_b6, eq111_e1486_d_b7, eq111_e1486_d_b8, eq111_e1486_d_b9, eq111_e1486_d_b10, eq111_e1486_d_b11, eq111_e1486_d_b12, eq111_e1486_d_b13, eq111_e1486_d_b14, eq111_e1486_d_b15, eq111_e1486_d_b16, eq111_e1486_d_b17, eq111_e1486_d_b18, eq111_e1486_d_b19, eq111_e1486_d_b20, eq111_e1486_d_b21, eq111_e1486_d_b22, eq111_e1486_d_b23, eq111_e1486_d_b24, eq111_e1486_d_b25, eq111_e1486_d_b26, eq111_e1486_d_b27, eq111_e1486_d_b28, eq111_e1486_d_b29, eq111_e1486_d_b30, eq111_e1486_d_b31, eq111_e1486_d_b32, eq111_e1486_d_b33, eq111_e1486_d_b34, eq111_e1486_d_b35, eq111_e1486_d_b36, eq111_e1486_d_b37, eq111_e1486_d_b38, eq111_e1486_d_b39, eq111_e1486_d_b40, eq111_e1486_d_b41, eq111_e1486_d_b42, eq111_e1486_d_b43, eq111_e1486_d_b44, eq111_e1486_d_b45, eq111_e1486_d_b46, eq111_e1486_d_b47, eq111_e1486_d_b48, eq111_e1486_d_b49, eq111_e1486_d_b50, eq111_e1486_d_b51, eq111_e1486_d_b52, eq111_e1486_d_b53, eq111_e1486_d_b54];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq111_value),
            &eq111_node_derivatives,
            &eq111_branch_derivatives,
            multiplicity,
        );
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22, eq112_e1493_d_b0, eq112_e1493_d_b1, eq112_e1493_d_b2, eq112_e1493_d_b3, eq112_e1493_d_b4, eq112_e1493_d_b5, eq112_e1493_d_b6, eq112_e1493_d_b7, eq112_e1493_d_b8, eq112_e1493_d_b9, eq112_e1493_d_b10, eq112_e1493_d_b11, eq112_e1493_d_b12, eq112_e1493_d_b13, eq112_e1493_d_b14, eq112_e1493_d_b15, eq112_e1493_d_b16, eq112_e1493_d_b17, eq112_e1493_d_b18, eq112_e1493_d_b19, eq112_e1493_d_b20, eq112_e1493_d_b21, eq112_e1493_d_b22, eq112_e1493_d_b23, eq112_e1493_d_b24, eq112_e1493_d_b25, eq112_e1493_d_b26, eq112_e1493_d_b27, eq112_e1493_d_b28, eq112_e1493_d_b29, eq112_e1493_d_b30, eq112_e1493_d_b31, eq112_e1493_d_b32, eq112_e1493_d_b33, eq112_e1493_d_b34, eq112_e1493_d_b35, eq112_e1493_d_b36, eq112_e1493_d_b37, eq112_e1493_d_b38, eq112_e1493_d_b39, eq112_e1493_d_b40, eq112_e1493_d_b41, eq112_e1493_d_b42, eq112_e1493_d_b43, eq112_e1493_d_b44, eq112_e1493_d_b45, eq112_e1493_d_b46, eq112_e1493_d_b47, eq112_e1493_d_b48, eq112_e1493_d_b49, eq112_e1493_d_b50, eq112_e1493_d_b51, eq112_e1493_d_b52, eq112_e1493_d_b53, eq112_e1493_d_b54,) = {
    if s.b[569] {
        let eq112_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, s.v[163]);
        let eq112_e1491: f64 = (p.p7 * eq112_e1490);
        (eq112_e1491, __rspice_deriv_cse_78, __rspice_deriv_cse_79, __rspice_deriv_cse_80, __rspice_deriv_cse_81, __rspice_deriv_cse_82, __rspice_deriv_cse_83, __rspice_deriv_cse_84, __rspice_deriv_cse_85, __rspice_deriv_cse_86, __rspice_deriv_cse_87, __rspice_deriv_cse_88, __rspice_deriv_cse_89, __rspice_deriv_cse_90, __rspice_deriv_cse_91, __rspice_deriv_cse_92, __rspice_deriv_cse_93, __rspice_deriv_cse_94, __rspice_deriv_cse_95, __rspice_deriv_cse_96, __rspice_deriv_cse_97, __rspice_deriv_cse_98, __rspice_deriv_cse_99, __rspice_deriv_cse_100, __rspice_deriv_cse_101, __rspice_deriv_cse_102, __rspice_deriv_cse_103, __rspice_deriv_cse_104, __rspice_deriv_cse_105, __rspice_deriv_cse_106, __rspice_deriv_cse_107, __rspice_deriv_cse_108, __rspice_deriv_cse_109, __rspice_deriv_cse_110, __rspice_deriv_cse_111, __rspice_deriv_cse_112, __rspice_deriv_cse_113, __rspice_deriv_cse_114, __rspice_deriv_cse_115, __rspice_deriv_cse_116, __rspice_deriv_cse_117, __rspice_deriv_cse_118, __rspice_deriv_cse_119, __rspice_deriv_cse_120, __rspice_deriv_cse_121, __rspice_deriv_cse_122, __rspice_deriv_cse_123, __rspice_deriv_cse_124, __rspice_deriv_cse_125, __rspice_deriv_cse_126, __rspice_deriv_cse_127, __rspice_deriv_cse_128, __rspice_deriv_cse_129, __rspice_deriv_cse_130, __rspice_deriv_cse_131, __rspice_deriv_cse_132, __rspice_deriv_cse_133, __rspice_deriv_cse_134, __rspice_deriv_cse_135, __rspice_deriv_cse_136, __rspice_deriv_cse_137, __rspice_deriv_cse_138, __rspice_deriv_cse_139, __rspice_deriv_cse_140, __rspice_deriv_cse_141, __rspice_deriv_cse_142, __rspice_deriv_cse_143, __rspice_deriv_cse_144, __rspice_deriv_cse_145, __rspice_deriv_cse_146, __rspice_deriv_cse_147, __rspice_deriv_cse_148, __rspice_deriv_cse_149, __rspice_deriv_cse_150, __rspice_deriv_cse_151, __rspice_deriv_cse_152, __rspice_deriv_cse_153, __rspice_deriv_cse_154, __rspice_deriv_cse_155,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1493;
        let eq112_node_derivatives: [f64; 23] = [eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22];
        let eq112_branch_derivatives: [f64; 55] = [eq112_e1493_d_b0, eq112_e1493_d_b1, eq112_e1493_d_b2, eq112_e1493_d_b3, eq112_e1493_d_b4, eq112_e1493_d_b5, eq112_e1493_d_b6, eq112_e1493_d_b7, eq112_e1493_d_b8, eq112_e1493_d_b9, eq112_e1493_d_b10, eq112_e1493_d_b11, eq112_e1493_d_b12, eq112_e1493_d_b13, eq112_e1493_d_b14, eq112_e1493_d_b15, eq112_e1493_d_b16, eq112_e1493_d_b17, eq112_e1493_d_b18, eq112_e1493_d_b19, eq112_e1493_d_b20, eq112_e1493_d_b21, eq112_e1493_d_b22, eq112_e1493_d_b23, eq112_e1493_d_b24, eq112_e1493_d_b25, eq112_e1493_d_b26, eq112_e1493_d_b27, eq112_e1493_d_b28, eq112_e1493_d_b29, eq112_e1493_d_b30, eq112_e1493_d_b31, eq112_e1493_d_b32, eq112_e1493_d_b33, eq112_e1493_d_b34, eq112_e1493_d_b35, eq112_e1493_d_b36, eq112_e1493_d_b37, eq112_e1493_d_b38, eq112_e1493_d_b39, eq112_e1493_d_b40, eq112_e1493_d_b41, eq112_e1493_d_b42, eq112_e1493_d_b43, eq112_e1493_d_b44, eq112_e1493_d_b45, eq112_e1493_d_b46, eq112_e1493_d_b47, eq112_e1493_d_b48, eq112_e1493_d_b49, eq112_e1493_d_b50, eq112_e1493_d_b51, eq112_e1493_d_b52, eq112_e1493_d_b53, eq112_e1493_d_b54];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq112_value),
            &eq112_node_derivatives,
            &eq112_branch_derivatives,
            multiplicity,
        );
        let (eq113_e1501, eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22, eq113_e1501_d_b0, eq113_e1501_d_b1, eq113_e1501_d_b2, eq113_e1501_d_b3, eq113_e1501_d_b4, eq113_e1501_d_b5, eq113_e1501_d_b6, eq113_e1501_d_b7, eq113_e1501_d_b8, eq113_e1501_d_b9, eq113_e1501_d_b10, eq113_e1501_d_b11, eq113_e1501_d_b12, eq113_e1501_d_b13, eq113_e1501_d_b14, eq113_e1501_d_b15, eq113_e1501_d_b16, eq113_e1501_d_b17, eq113_e1501_d_b18, eq113_e1501_d_b19, eq113_e1501_d_b20, eq113_e1501_d_b21, eq113_e1501_d_b22, eq113_e1501_d_b23, eq113_e1501_d_b24, eq113_e1501_d_b25, eq113_e1501_d_b26, eq113_e1501_d_b27, eq113_e1501_d_b28, eq113_e1501_d_b29, eq113_e1501_d_b30, eq113_e1501_d_b31, eq113_e1501_d_b32, eq113_e1501_d_b33, eq113_e1501_d_b34, eq113_e1501_d_b35, eq113_e1501_d_b36, eq113_e1501_d_b37, eq113_e1501_d_b38, eq113_e1501_d_b39, eq113_e1501_d_b40, eq113_e1501_d_b41, eq113_e1501_d_b42, eq113_e1501_d_b43, eq113_e1501_d_b44, eq113_e1501_d_b45, eq113_e1501_d_b46, eq113_e1501_d_b47, eq113_e1501_d_b48, eq113_e1501_d_b49, eq113_e1501_d_b50, eq113_e1501_d_b51, eq113_e1501_d_b52, eq113_e1501_d_b53, eq113_e1501_d_b54,) = {
    if (!s.b[569]) {
        let eq113_e1498: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, s.v[162]);
        let eq113_e1499: f64 = (p.p7 * eq113_e1498);
        (eq113_e1499, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1501;
        let eq113_node_derivatives: [f64; 23] = [eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22];
        let eq113_branch_derivatives: [f64; 55] = [eq113_e1501_d_b0, eq113_e1501_d_b1, eq113_e1501_d_b2, eq113_e1501_d_b3, eq113_e1501_d_b4, eq113_e1501_d_b5, eq113_e1501_d_b6, eq113_e1501_d_b7, eq113_e1501_d_b8, eq113_e1501_d_b9, eq113_e1501_d_b10, eq113_e1501_d_b11, eq113_e1501_d_b12, eq113_e1501_d_b13, eq113_e1501_d_b14, eq113_e1501_d_b15, eq113_e1501_d_b16, eq113_e1501_d_b17, eq113_e1501_d_b18, eq113_e1501_d_b19, eq113_e1501_d_b20, eq113_e1501_d_b21, eq113_e1501_d_b22, eq113_e1501_d_b23, eq113_e1501_d_b24, eq113_e1501_d_b25, eq113_e1501_d_b26, eq113_e1501_d_b27, eq113_e1501_d_b28, eq113_e1501_d_b29, eq113_e1501_d_b30, eq113_e1501_d_b31, eq113_e1501_d_b32, eq113_e1501_d_b33, eq113_e1501_d_b34, eq113_e1501_d_b35, eq113_e1501_d_b36, eq113_e1501_d_b37, eq113_e1501_d_b38, eq113_e1501_d_b39, eq113_e1501_d_b40, eq113_e1501_d_b41, eq113_e1501_d_b42, eq113_e1501_d_b43, eq113_e1501_d_b44, eq113_e1501_d_b45, eq113_e1501_d_b46, eq113_e1501_d_b47, eq113_e1501_d_b48, eq113_e1501_d_b49, eq113_e1501_d_b50, eq113_e1501_d_b51, eq113_e1501_d_b52, eq113_e1501_d_b53, eq113_e1501_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq113_value),
            &eq113_node_derivatives,
            &eq113_branch_derivatives,
            multiplicity,
        );
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22, eq114_e1509_d_b0, eq114_e1509_d_b1, eq114_e1509_d_b2, eq114_e1509_d_b3, eq114_e1509_d_b4, eq114_e1509_d_b5, eq114_e1509_d_b6, eq114_e1509_d_b7, eq114_e1509_d_b8, eq114_e1509_d_b9, eq114_e1509_d_b10, eq114_e1509_d_b11, eq114_e1509_d_b12, eq114_e1509_d_b13, eq114_e1509_d_b14, eq114_e1509_d_b15, eq114_e1509_d_b16, eq114_e1509_d_b17, eq114_e1509_d_b18, eq114_e1509_d_b19, eq114_e1509_d_b20, eq114_e1509_d_b21, eq114_e1509_d_b22, eq114_e1509_d_b23, eq114_e1509_d_b24, eq114_e1509_d_b25, eq114_e1509_d_b26, eq114_e1509_d_b27, eq114_e1509_d_b28, eq114_e1509_d_b29, eq114_e1509_d_b30, eq114_e1509_d_b31, eq114_e1509_d_b32, eq114_e1509_d_b33, eq114_e1509_d_b34, eq114_e1509_d_b35, eq114_e1509_d_b36, eq114_e1509_d_b37, eq114_e1509_d_b38, eq114_e1509_d_b39, eq114_e1509_d_b40, eq114_e1509_d_b41, eq114_e1509_d_b42, eq114_e1509_d_b43, eq114_e1509_d_b44, eq114_e1509_d_b45, eq114_e1509_d_b46, eq114_e1509_d_b47, eq114_e1509_d_b48, eq114_e1509_d_b49, eq114_e1509_d_b50, eq114_e1509_d_b51, eq114_e1509_d_b52, eq114_e1509_d_b53, eq114_e1509_d_b54,) = {
    if (!s.b[569]) {
        let eq114_e1506: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, s.v[163]);
        let eq114_e1507: f64 = (p.p7 * eq114_e1506);
        (eq114_e1507, __rspice_deriv_cse_78, __rspice_deriv_cse_79, __rspice_deriv_cse_80, __rspice_deriv_cse_81, __rspice_deriv_cse_82, __rspice_deriv_cse_83, __rspice_deriv_cse_84, __rspice_deriv_cse_85, __rspice_deriv_cse_86, __rspice_deriv_cse_87, __rspice_deriv_cse_88, __rspice_deriv_cse_89, __rspice_deriv_cse_90, __rspice_deriv_cse_91, __rspice_deriv_cse_92, __rspice_deriv_cse_93, __rspice_deriv_cse_94, __rspice_deriv_cse_95, __rspice_deriv_cse_96, __rspice_deriv_cse_97, __rspice_deriv_cse_98, __rspice_deriv_cse_99, __rspice_deriv_cse_100, __rspice_deriv_cse_101, __rspice_deriv_cse_102, __rspice_deriv_cse_103, __rspice_deriv_cse_104, __rspice_deriv_cse_105, __rspice_deriv_cse_106, __rspice_deriv_cse_107, __rspice_deriv_cse_108, __rspice_deriv_cse_109, __rspice_deriv_cse_110, __rspice_deriv_cse_111, __rspice_deriv_cse_112, __rspice_deriv_cse_113, __rspice_deriv_cse_114, __rspice_deriv_cse_115, __rspice_deriv_cse_116, __rspice_deriv_cse_117, __rspice_deriv_cse_118, __rspice_deriv_cse_119, __rspice_deriv_cse_120, __rspice_deriv_cse_121, __rspice_deriv_cse_122, __rspice_deriv_cse_123, __rspice_deriv_cse_124, __rspice_deriv_cse_125, __rspice_deriv_cse_126, __rspice_deriv_cse_127, __rspice_deriv_cse_128, __rspice_deriv_cse_129, __rspice_deriv_cse_130, __rspice_deriv_cse_131, __rspice_deriv_cse_132, __rspice_deriv_cse_133, __rspice_deriv_cse_134, __rspice_deriv_cse_135, __rspice_deriv_cse_136, __rspice_deriv_cse_137, __rspice_deriv_cse_138, __rspice_deriv_cse_139, __rspice_deriv_cse_140, __rspice_deriv_cse_141, __rspice_deriv_cse_142, __rspice_deriv_cse_143, __rspice_deriv_cse_144, __rspice_deriv_cse_145, __rspice_deriv_cse_146, __rspice_deriv_cse_147, __rspice_deriv_cse_148, __rspice_deriv_cse_149, __rspice_deriv_cse_150, __rspice_deriv_cse_151, __rspice_deriv_cse_152, __rspice_deriv_cse_153, __rspice_deriv_cse_154, __rspice_deriv_cse_155,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_value: f64 = eq114_e1509;
        let eq114_node_derivatives: [f64; 23] = [eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22];
        let eq114_branch_derivatives: [f64; 55] = [eq114_e1509_d_b0, eq114_e1509_d_b1, eq114_e1509_d_b2, eq114_e1509_d_b3, eq114_e1509_d_b4, eq114_e1509_d_b5, eq114_e1509_d_b6, eq114_e1509_d_b7, eq114_e1509_d_b8, eq114_e1509_d_b9, eq114_e1509_d_b10, eq114_e1509_d_b11, eq114_e1509_d_b12, eq114_e1509_d_b13, eq114_e1509_d_b14, eq114_e1509_d_b15, eq114_e1509_d_b16, eq114_e1509_d_b17, eq114_e1509_d_b18, eq114_e1509_d_b19, eq114_e1509_d_b20, eq114_e1509_d_b21, eq114_e1509_d_b22, eq114_e1509_d_b23, eq114_e1509_d_b24, eq114_e1509_d_b25, eq114_e1509_d_b26, eq114_e1509_d_b27, eq114_e1509_d_b28, eq114_e1509_d_b29, eq114_e1509_d_b30, eq114_e1509_d_b31, eq114_e1509_d_b32, eq114_e1509_d_b33, eq114_e1509_d_b34, eq114_e1509_d_b35, eq114_e1509_d_b36, eq114_e1509_d_b37, eq114_e1509_d_b38, eq114_e1509_d_b39, eq114_e1509_d_b40, eq114_e1509_d_b41, eq114_e1509_d_b42, eq114_e1509_d_b43, eq114_e1509_d_b44, eq114_e1509_d_b45, eq114_e1509_d_b46, eq114_e1509_d_b47, eq114_e1509_d_b48, eq114_e1509_d_b49, eq114_e1509_d_b50, eq114_e1509_d_b51, eq114_e1509_d_b52, eq114_e1509_d_b53, eq114_e1509_d_b54];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq114_value),
            &eq114_node_derivatives,
            &eq114_branch_derivatives,
            multiplicity,
        );
        let eq115_e1512: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, s.v[164]);
        let eq115_e1513: f64 = (p.p7 * eq115_e1512);
        let eq115_e1513_d_n0: f64 = (p.p7 * (s.dn[164][0] * ddt_scale));
        let eq115_e1513_d_n1: f64 = (p.p7 * (s.dn[164][1] * ddt_scale));
        let eq115_e1513_d_n2: f64 = (p.p7 * (s.dn[164][2] * ddt_scale));
        let eq115_e1513_d_n3: f64 = (p.p7 * (s.dn[164][3] * ddt_scale));
        let eq115_e1513_d_n4: f64 = (p.p7 * (s.dn[164][4] * ddt_scale));
        let eq115_e1513_d_n5: f64 = (p.p7 * (s.dn[164][5] * ddt_scale));
        let eq115_e1513_d_n6: f64 = (p.p7 * (s.dn[164][6] * ddt_scale));
        let eq115_e1513_d_n7: f64 = (p.p7 * (s.dn[164][7] * ddt_scale));
        let eq115_e1513_d_n8: f64 = (p.p7 * (s.dn[164][8] * ddt_scale));
        let eq115_e1513_d_n9: f64 = (p.p7 * (s.dn[164][9] * ddt_scale));
        let eq115_e1513_d_n10: f64 = (p.p7 * (s.dn[164][10] * ddt_scale));
        let eq115_e1513_d_n11: f64 = (p.p7 * (s.dn[164][11] * ddt_scale));
        let eq115_e1513_d_n12: f64 = (p.p7 * (s.dn[164][12] * ddt_scale));
        let eq115_e1513_d_n13: f64 = (p.p7 * (s.dn[164][13] * ddt_scale));
        let eq115_e1513_d_n14: f64 = (p.p7 * (s.dn[164][14] * ddt_scale));
        let eq115_e1513_d_n15: f64 = (p.p7 * (s.dn[164][15] * ddt_scale));
        let eq115_e1513_d_n16: f64 = (p.p7 * (s.dn[164][16] * ddt_scale));
        let eq115_e1513_d_n17: f64 = (p.p7 * (s.dn[164][17] * ddt_scale));
        let eq115_e1513_d_n18: f64 = (p.p7 * (s.dn[164][18] * ddt_scale));
        let eq115_e1513_d_n19: f64 = (p.p7 * (s.dn[164][19] * ddt_scale));
        let eq115_e1513_d_n20: f64 = (p.p7 * (s.dn[164][20] * ddt_scale));
        let eq115_e1513_d_n21: f64 = (p.p7 * (s.dn[164][21] * ddt_scale));
        let eq115_e1513_d_n22: f64 = (p.p7 * (s.dn[164][22] * ddt_scale));
        let eq115_e1513_d_b0: f64 = (p.p7 * (s.db[164][0] * ddt_scale));
        let eq115_e1513_d_b1: f64 = (p.p7 * (s.db[164][1] * ddt_scale));
        let eq115_e1513_d_b2: f64 = (p.p7 * (s.db[164][2] * ddt_scale));
        let eq115_e1513_d_b3: f64 = (p.p7 * (s.db[164][3] * ddt_scale));
        let eq115_e1513_d_b4: f64 = (p.p7 * (s.db[164][4] * ddt_scale));
        let eq115_e1513_d_b5: f64 = (p.p7 * (s.db[164][5] * ddt_scale));
        let eq115_e1513_d_b6: f64 = (p.p7 * (s.db[164][6] * ddt_scale));
        let eq115_e1513_d_b7: f64 = (p.p7 * (s.db[164][7] * ddt_scale));
        let eq115_e1513_d_b8: f64 = (p.p7 * (s.db[164][8] * ddt_scale));
        let eq115_e1513_d_b9: f64 = (p.p7 * (s.db[164][9] * ddt_scale));
        let eq115_e1513_d_b10: f64 = (p.p7 * (s.db[164][10] * ddt_scale));
        let eq115_e1513_d_b11: f64 = (p.p7 * (s.db[164][11] * ddt_scale));
        let eq115_e1513_d_b12: f64 = (p.p7 * (s.db[164][12] * ddt_scale));
        let eq115_e1513_d_b13: f64 = (p.p7 * (s.db[164][13] * ddt_scale));
        let eq115_e1513_d_b14: f64 = (p.p7 * (s.db[164][14] * ddt_scale));
        let eq115_e1513_d_b15: f64 = (p.p7 * (s.db[164][15] * ddt_scale));
        let eq115_e1513_d_b16: f64 = (p.p7 * (s.db[164][16] * ddt_scale));
        let eq115_e1513_d_b17: f64 = (p.p7 * (s.db[164][17] * ddt_scale));
        let eq115_e1513_d_b18: f64 = (p.p7 * (s.db[164][18] * ddt_scale));
        let eq115_e1513_d_b19: f64 = (p.p7 * (s.db[164][19] * ddt_scale));
        let eq115_e1513_d_b20: f64 = (p.p7 * (s.db[164][20] * ddt_scale));
        let eq115_e1513_d_b21: f64 = (p.p7 * (s.db[164][21] * ddt_scale));
        let eq115_e1513_d_b22: f64 = (p.p7 * (s.db[164][22] * ddt_scale));
        let eq115_e1513_d_b23: f64 = (p.p7 * (s.db[164][23] * ddt_scale));
        let eq115_e1513_d_b24: f64 = (p.p7 * (s.db[164][24] * ddt_scale));
        let eq115_e1513_d_b25: f64 = (p.p7 * (s.db[164][25] * ddt_scale));
        let eq115_e1513_d_b26: f64 = (p.p7 * (s.db[164][26] * ddt_scale));
        let eq115_e1513_d_b27: f64 = (p.p7 * (s.db[164][27] * ddt_scale));
        let eq115_e1513_d_b28: f64 = (p.p7 * (s.db[164][28] * ddt_scale));
        let eq115_e1513_d_b29: f64 = (p.p7 * (s.db[164][29] * ddt_scale));
        let eq115_e1513_d_b30: f64 = (p.p7 * (s.db[164][30] * ddt_scale));
        let eq115_e1513_d_b31: f64 = (p.p7 * (s.db[164][31] * ddt_scale));
        let eq115_e1513_d_b32: f64 = (p.p7 * (s.db[164][32] * ddt_scale));
        let eq115_e1513_d_b33: f64 = (p.p7 * (s.db[164][33] * ddt_scale));
        let eq115_e1513_d_b34: f64 = (p.p7 * (s.db[164][34] * ddt_scale));
        let eq115_e1513_d_b35: f64 = (p.p7 * (s.db[164][35] * ddt_scale));
        let eq115_e1513_d_b36: f64 = (p.p7 * (s.db[164][36] * ddt_scale));
        let eq115_e1513_d_b37: f64 = (p.p7 * (s.db[164][37] * ddt_scale));
        let eq115_e1513_d_b38: f64 = (p.p7 * (s.db[164][38] * ddt_scale));
        let eq115_e1513_d_b39: f64 = (p.p7 * (s.db[164][39] * ddt_scale));
        let eq115_e1513_d_b40: f64 = (p.p7 * (s.db[164][40] * ddt_scale));
        let eq115_e1513_d_b41: f64 = (p.p7 * (s.db[164][41] * ddt_scale));
        let eq115_e1513_d_b42: f64 = (p.p7 * (s.db[164][42] * ddt_scale));
        let eq115_e1513_d_b43: f64 = (p.p7 * (s.db[164][43] * ddt_scale));
        let eq115_e1513_d_b44: f64 = (p.p7 * (s.db[164][44] * ddt_scale));
        let eq115_e1513_d_b45: f64 = (p.p7 * (s.db[164][45] * ddt_scale));
        let eq115_e1513_d_b46: f64 = (p.p7 * (s.db[164][46] * ddt_scale));
        let eq115_e1513_d_b47: f64 = (p.p7 * (s.db[164][47] * ddt_scale));
        let eq115_e1513_d_b48: f64 = (p.p7 * (s.db[164][48] * ddt_scale));
        let eq115_e1513_d_b49: f64 = (p.p7 * (s.db[164][49] * ddt_scale));
        let eq115_e1513_d_b50: f64 = (p.p7 * (s.db[164][50] * ddt_scale));
        let eq115_e1513_d_b51: f64 = (p.p7 * (s.db[164][51] * ddt_scale));
        let eq115_e1513_d_b52: f64 = (p.p7 * (s.db[164][52] * ddt_scale));
        let eq115_e1513_d_b53: f64 = (p.p7 * (s.db[164][53] * ddt_scale));
        let eq115_e1513_d_b54: f64 = (p.p7 * (s.db[164][54] * ddt_scale));
        let eq115_value: f64 = eq115_e1513;
        let eq115_node_derivatives: [f64; 23] = [eq115_e1513_d_n0, eq115_e1513_d_n1, eq115_e1513_d_n2, eq115_e1513_d_n3, eq115_e1513_d_n4, eq115_e1513_d_n5, eq115_e1513_d_n6, eq115_e1513_d_n7, eq115_e1513_d_n8, eq115_e1513_d_n9, eq115_e1513_d_n10, eq115_e1513_d_n11, eq115_e1513_d_n12, eq115_e1513_d_n13, eq115_e1513_d_n14, eq115_e1513_d_n15, eq115_e1513_d_n16, eq115_e1513_d_n17, eq115_e1513_d_n18, eq115_e1513_d_n19, eq115_e1513_d_n20, eq115_e1513_d_n21, eq115_e1513_d_n22];
        let eq115_branch_derivatives: [f64; 55] = [eq115_e1513_d_b0, eq115_e1513_d_b1, eq115_e1513_d_b2, eq115_e1513_d_b3, eq115_e1513_d_b4, eq115_e1513_d_b5, eq115_e1513_d_b6, eq115_e1513_d_b7, eq115_e1513_d_b8, eq115_e1513_d_b9, eq115_e1513_d_b10, eq115_e1513_d_b11, eq115_e1513_d_b12, eq115_e1513_d_b13, eq115_e1513_d_b14, eq115_e1513_d_b15, eq115_e1513_d_b16, eq115_e1513_d_b17, eq115_e1513_d_b18, eq115_e1513_d_b19, eq115_e1513_d_b20, eq115_e1513_d_b21, eq115_e1513_d_b22, eq115_e1513_d_b23, eq115_e1513_d_b24, eq115_e1513_d_b25, eq115_e1513_d_b26, eq115_e1513_d_b27, eq115_e1513_d_b28, eq115_e1513_d_b29, eq115_e1513_d_b30, eq115_e1513_d_b31, eq115_e1513_d_b32, eq115_e1513_d_b33, eq115_e1513_d_b34, eq115_e1513_d_b35, eq115_e1513_d_b36, eq115_e1513_d_b37, eq115_e1513_d_b38, eq115_e1513_d_b39, eq115_e1513_d_b40, eq115_e1513_d_b41, eq115_e1513_d_b42, eq115_e1513_d_b43, eq115_e1513_d_b44, eq115_e1513_d_b45, eq115_e1513_d_b46, eq115_e1513_d_b47, eq115_e1513_d_b48, eq115_e1513_d_b49, eq115_e1513_d_b50, eq115_e1513_d_b51, eq115_e1513_d_b52, eq115_e1513_d_b53, eq115_e1513_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq115_value),
            &eq115_node_derivatives,
            &eq115_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_14(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let eq116_e1516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, s.v[219]);
        let eq116_e1517: f64 = (p.p7 * eq116_e1516);
        let eq116_e1517_d_n0: f64 = (p.p7 * (s.dn[219][0] * ddt_scale));
        let eq116_e1517_d_n1: f64 = (p.p7 * (s.dn[219][1] * ddt_scale));
        let eq116_e1517_d_n2: f64 = (p.p7 * (s.dn[219][2] * ddt_scale));
        let eq116_e1517_d_n3: f64 = (p.p7 * (s.dn[219][3] * ddt_scale));
        let eq116_e1517_d_n4: f64 = (p.p7 * (s.dn[219][4] * ddt_scale));
        let eq116_e1517_d_n5: f64 = (p.p7 * (s.dn[219][5] * ddt_scale));
        let eq116_e1517_d_n6: f64 = (p.p7 * (s.dn[219][6] * ddt_scale));
        let eq116_e1517_d_n7: f64 = (p.p7 * (s.dn[219][7] * ddt_scale));
        let eq116_e1517_d_n8: f64 = (p.p7 * (s.dn[219][8] * ddt_scale));
        let eq116_e1517_d_n9: f64 = (p.p7 * (s.dn[219][9] * ddt_scale));
        let eq116_e1517_d_n10: f64 = (p.p7 * (s.dn[219][10] * ddt_scale));
        let eq116_e1517_d_n11: f64 = (p.p7 * (s.dn[219][11] * ddt_scale));
        let eq116_e1517_d_n12: f64 = (p.p7 * (s.dn[219][12] * ddt_scale));
        let eq116_e1517_d_n13: f64 = (p.p7 * (s.dn[219][13] * ddt_scale));
        let eq116_e1517_d_n14: f64 = (p.p7 * (s.dn[219][14] * ddt_scale));
        let eq116_e1517_d_n15: f64 = (p.p7 * (s.dn[219][15] * ddt_scale));
        let eq116_e1517_d_n16: f64 = (p.p7 * (s.dn[219][16] * ddt_scale));
        let eq116_e1517_d_n17: f64 = (p.p7 * (s.dn[219][17] * ddt_scale));
        let eq116_e1517_d_n18: f64 = (p.p7 * (s.dn[219][18] * ddt_scale));
        let eq116_e1517_d_n19: f64 = (p.p7 * (s.dn[219][19] * ddt_scale));
        let eq116_e1517_d_n20: f64 = (p.p7 * (s.dn[219][20] * ddt_scale));
        let eq116_e1517_d_n21: f64 = (p.p7 * (s.dn[219][21] * ddt_scale));
        let eq116_e1517_d_n22: f64 = (p.p7 * (s.dn[219][22] * ddt_scale));
        let eq116_e1517_d_b0: f64 = (p.p7 * (s.db[219][0] * ddt_scale));
        let eq116_e1517_d_b1: f64 = (p.p7 * (s.db[219][1] * ddt_scale));
        let eq116_e1517_d_b2: f64 = (p.p7 * (s.db[219][2] * ddt_scale));
        let eq116_e1517_d_b3: f64 = (p.p7 * (s.db[219][3] * ddt_scale));
        let eq116_e1517_d_b4: f64 = (p.p7 * (s.db[219][4] * ddt_scale));
        let eq116_e1517_d_b5: f64 = (p.p7 * (s.db[219][5] * ddt_scale));
        let eq116_e1517_d_b6: f64 = (p.p7 * (s.db[219][6] * ddt_scale));
        let eq116_e1517_d_b7: f64 = (p.p7 * (s.db[219][7] * ddt_scale));
        let eq116_e1517_d_b8: f64 = (p.p7 * (s.db[219][8] * ddt_scale));
        let eq116_e1517_d_b9: f64 = (p.p7 * (s.db[219][9] * ddt_scale));
        let eq116_e1517_d_b10: f64 = (p.p7 * (s.db[219][10] * ddt_scale));
        let eq116_e1517_d_b11: f64 = (p.p7 * (s.db[219][11] * ddt_scale));
        let eq116_e1517_d_b12: f64 = (p.p7 * (s.db[219][12] * ddt_scale));
        let eq116_e1517_d_b13: f64 = (p.p7 * (s.db[219][13] * ddt_scale));
        let eq116_e1517_d_b14: f64 = (p.p7 * (s.db[219][14] * ddt_scale));
        let eq116_e1517_d_b15: f64 = (p.p7 * (s.db[219][15] * ddt_scale));
        let eq116_e1517_d_b16: f64 = (p.p7 * (s.db[219][16] * ddt_scale));
        let eq116_e1517_d_b17: f64 = (p.p7 * (s.db[219][17] * ddt_scale));
        let eq116_e1517_d_b18: f64 = (p.p7 * (s.db[219][18] * ddt_scale));
        let eq116_e1517_d_b19: f64 = (p.p7 * (s.db[219][19] * ddt_scale));
        let eq116_e1517_d_b20: f64 = (p.p7 * (s.db[219][20] * ddt_scale));
        let eq116_e1517_d_b21: f64 = (p.p7 * (s.db[219][21] * ddt_scale));
        let eq116_e1517_d_b22: f64 = (p.p7 * (s.db[219][22] * ddt_scale));
        let eq116_e1517_d_b23: f64 = (p.p7 * (s.db[219][23] * ddt_scale));
        let eq116_e1517_d_b24: f64 = (p.p7 * (s.db[219][24] * ddt_scale));
        let eq116_e1517_d_b25: f64 = (p.p7 * (s.db[219][25] * ddt_scale));
        let eq116_e1517_d_b26: f64 = (p.p7 * (s.db[219][26] * ddt_scale));
        let eq116_e1517_d_b27: f64 = (p.p7 * (s.db[219][27] * ddt_scale));
        let eq116_e1517_d_b28: f64 = (p.p7 * (s.db[219][28] * ddt_scale));
        let eq116_e1517_d_b29: f64 = (p.p7 * (s.db[219][29] * ddt_scale));
        let eq116_e1517_d_b30: f64 = (p.p7 * (s.db[219][30] * ddt_scale));
        let eq116_e1517_d_b31: f64 = (p.p7 * (s.db[219][31] * ddt_scale));
        let eq116_e1517_d_b32: f64 = (p.p7 * (s.db[219][32] * ddt_scale));
        let eq116_e1517_d_b33: f64 = (p.p7 * (s.db[219][33] * ddt_scale));
        let eq116_e1517_d_b34: f64 = (p.p7 * (s.db[219][34] * ddt_scale));
        let eq116_e1517_d_b35: f64 = (p.p7 * (s.db[219][35] * ddt_scale));
        let eq116_e1517_d_b36: f64 = (p.p7 * (s.db[219][36] * ddt_scale));
        let eq116_e1517_d_b37: f64 = (p.p7 * (s.db[219][37] * ddt_scale));
        let eq116_e1517_d_b38: f64 = (p.p7 * (s.db[219][38] * ddt_scale));
        let eq116_e1517_d_b39: f64 = (p.p7 * (s.db[219][39] * ddt_scale));
        let eq116_e1517_d_b40: f64 = (p.p7 * (s.db[219][40] * ddt_scale));
        let eq116_e1517_d_b41: f64 = (p.p7 * (s.db[219][41] * ddt_scale));
        let eq116_e1517_d_b42: f64 = (p.p7 * (s.db[219][42] * ddt_scale));
        let eq116_e1517_d_b43: f64 = (p.p7 * (s.db[219][43] * ddt_scale));
        let eq116_e1517_d_b44: f64 = (p.p7 * (s.db[219][44] * ddt_scale));
        let eq116_e1517_d_b45: f64 = (p.p7 * (s.db[219][45] * ddt_scale));
        let eq116_e1517_d_b46: f64 = (p.p7 * (s.db[219][46] * ddt_scale));
        let eq116_e1517_d_b47: f64 = (p.p7 * (s.db[219][47] * ddt_scale));
        let eq116_e1517_d_b48: f64 = (p.p7 * (s.db[219][48] * ddt_scale));
        let eq116_e1517_d_b49: f64 = (p.p7 * (s.db[219][49] * ddt_scale));
        let eq116_e1517_d_b50: f64 = (p.p7 * (s.db[219][50] * ddt_scale));
        let eq116_e1517_d_b51: f64 = (p.p7 * (s.db[219][51] * ddt_scale));
        let eq116_e1517_d_b52: f64 = (p.p7 * (s.db[219][52] * ddt_scale));
        let eq116_e1517_d_b53: f64 = (p.p7 * (s.db[219][53] * ddt_scale));
        let eq116_e1517_d_b54: f64 = (p.p7 * (s.db[219][54] * ddt_scale));
        let eq116_value: f64 = eq116_e1517;
        let eq116_node_derivatives: [f64; 23] = [eq116_e1517_d_n0, eq116_e1517_d_n1, eq116_e1517_d_n2, eq116_e1517_d_n3, eq116_e1517_d_n4, eq116_e1517_d_n5, eq116_e1517_d_n6, eq116_e1517_d_n7, eq116_e1517_d_n8, eq116_e1517_d_n9, eq116_e1517_d_n10, eq116_e1517_d_n11, eq116_e1517_d_n12, eq116_e1517_d_n13, eq116_e1517_d_n14, eq116_e1517_d_n15, eq116_e1517_d_n16, eq116_e1517_d_n17, eq116_e1517_d_n18, eq116_e1517_d_n19, eq116_e1517_d_n20, eq116_e1517_d_n21, eq116_e1517_d_n22];
        let eq116_branch_derivatives: [f64; 55] = [eq116_e1517_d_b0, eq116_e1517_d_b1, eq116_e1517_d_b2, eq116_e1517_d_b3, eq116_e1517_d_b4, eq116_e1517_d_b5, eq116_e1517_d_b6, eq116_e1517_d_b7, eq116_e1517_d_b8, eq116_e1517_d_b9, eq116_e1517_d_b10, eq116_e1517_d_b11, eq116_e1517_d_b12, eq116_e1517_d_b13, eq116_e1517_d_b14, eq116_e1517_d_b15, eq116_e1517_d_b16, eq116_e1517_d_b17, eq116_e1517_d_b18, eq116_e1517_d_b19, eq116_e1517_d_b20, eq116_e1517_d_b21, eq116_e1517_d_b22, eq116_e1517_d_b23, eq116_e1517_d_b24, eq116_e1517_d_b25, eq116_e1517_d_b26, eq116_e1517_d_b27, eq116_e1517_d_b28, eq116_e1517_d_b29, eq116_e1517_d_b30, eq116_e1517_d_b31, eq116_e1517_d_b32, eq116_e1517_d_b33, eq116_e1517_d_b34, eq116_e1517_d_b35, eq116_e1517_d_b36, eq116_e1517_d_b37, eq116_e1517_d_b38, eq116_e1517_d_b39, eq116_e1517_d_b40, eq116_e1517_d_b41, eq116_e1517_d_b42, eq116_e1517_d_b43, eq116_e1517_d_b44, eq116_e1517_d_b45, eq116_e1517_d_b46, eq116_e1517_d_b47, eq116_e1517_d_b48, eq116_e1517_d_b49, eq116_e1517_d_b50, eq116_e1517_d_b51, eq116_e1517_d_b52, eq116_e1517_d_b53, eq116_e1517_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(0),
            multiplicity * (eq116_value),
            &eq116_node_derivatives,
            &eq116_branch_derivatives,
            multiplicity,
        );
        let eq117_e1520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, s.v[220]);
        let eq117_e1521: f64 = (p.p7 * eq117_e1520);
        let eq117_e1521_d_n0: f64 = (p.p7 * (s.dn[220][0] * ddt_scale));
        let eq117_e1521_d_n1: f64 = (p.p7 * (s.dn[220][1] * ddt_scale));
        let eq117_e1521_d_n2: f64 = (p.p7 * (s.dn[220][2] * ddt_scale));
        let eq117_e1521_d_n3: f64 = (p.p7 * (s.dn[220][3] * ddt_scale));
        let eq117_e1521_d_n4: f64 = (p.p7 * (s.dn[220][4] * ddt_scale));
        let eq117_e1521_d_n5: f64 = (p.p7 * (s.dn[220][5] * ddt_scale));
        let eq117_e1521_d_n6: f64 = (p.p7 * (s.dn[220][6] * ddt_scale));
        let eq117_e1521_d_n7: f64 = (p.p7 * (s.dn[220][7] * ddt_scale));
        let eq117_e1521_d_n8: f64 = (p.p7 * (s.dn[220][8] * ddt_scale));
        let eq117_e1521_d_n9: f64 = (p.p7 * (s.dn[220][9] * ddt_scale));
        let eq117_e1521_d_n10: f64 = (p.p7 * (s.dn[220][10] * ddt_scale));
        let eq117_e1521_d_n11: f64 = (p.p7 * (s.dn[220][11] * ddt_scale));
        let eq117_e1521_d_n12: f64 = (p.p7 * (s.dn[220][12] * ddt_scale));
        let eq117_e1521_d_n13: f64 = (p.p7 * (s.dn[220][13] * ddt_scale));
        let eq117_e1521_d_n14: f64 = (p.p7 * (s.dn[220][14] * ddt_scale));
        let eq117_e1521_d_n15: f64 = (p.p7 * (s.dn[220][15] * ddt_scale));
        let eq117_e1521_d_n16: f64 = (p.p7 * (s.dn[220][16] * ddt_scale));
        let eq117_e1521_d_n17: f64 = (p.p7 * (s.dn[220][17] * ddt_scale));
        let eq117_e1521_d_n18: f64 = (p.p7 * (s.dn[220][18] * ddt_scale));
        let eq117_e1521_d_n19: f64 = (p.p7 * (s.dn[220][19] * ddt_scale));
        let eq117_e1521_d_n20: f64 = (p.p7 * (s.dn[220][20] * ddt_scale));
        let eq117_e1521_d_n21: f64 = (p.p7 * (s.dn[220][21] * ddt_scale));
        let eq117_e1521_d_n22: f64 = (p.p7 * (s.dn[220][22] * ddt_scale));
        let eq117_e1521_d_b0: f64 = (p.p7 * (s.db[220][0] * ddt_scale));
        let eq117_e1521_d_b1: f64 = (p.p7 * (s.db[220][1] * ddt_scale));
        let eq117_e1521_d_b2: f64 = (p.p7 * (s.db[220][2] * ddt_scale));
        let eq117_e1521_d_b3: f64 = (p.p7 * (s.db[220][3] * ddt_scale));
        let eq117_e1521_d_b4: f64 = (p.p7 * (s.db[220][4] * ddt_scale));
        let eq117_e1521_d_b5: f64 = (p.p7 * (s.db[220][5] * ddt_scale));
        let eq117_e1521_d_b6: f64 = (p.p7 * (s.db[220][6] * ddt_scale));
        let eq117_e1521_d_b7: f64 = (p.p7 * (s.db[220][7] * ddt_scale));
        let eq117_e1521_d_b8: f64 = (p.p7 * (s.db[220][8] * ddt_scale));
        let eq117_e1521_d_b9: f64 = (p.p7 * (s.db[220][9] * ddt_scale));
        let eq117_e1521_d_b10: f64 = (p.p7 * (s.db[220][10] * ddt_scale));
        let eq117_e1521_d_b11: f64 = (p.p7 * (s.db[220][11] * ddt_scale));
        let eq117_e1521_d_b12: f64 = (p.p7 * (s.db[220][12] * ddt_scale));
        let eq117_e1521_d_b13: f64 = (p.p7 * (s.db[220][13] * ddt_scale));
        let eq117_e1521_d_b14: f64 = (p.p7 * (s.db[220][14] * ddt_scale));
        let eq117_e1521_d_b15: f64 = (p.p7 * (s.db[220][15] * ddt_scale));
        let eq117_e1521_d_b16: f64 = (p.p7 * (s.db[220][16] * ddt_scale));
        let eq117_e1521_d_b17: f64 = (p.p7 * (s.db[220][17] * ddt_scale));
        let eq117_e1521_d_b18: f64 = (p.p7 * (s.db[220][18] * ddt_scale));
        let eq117_e1521_d_b19: f64 = (p.p7 * (s.db[220][19] * ddt_scale));
        let eq117_e1521_d_b20: f64 = (p.p7 * (s.db[220][20] * ddt_scale));
        let eq117_e1521_d_b21: f64 = (p.p7 * (s.db[220][21] * ddt_scale));
        let eq117_e1521_d_b22: f64 = (p.p7 * (s.db[220][22] * ddt_scale));
        let eq117_e1521_d_b23: f64 = (p.p7 * (s.db[220][23] * ddt_scale));
        let eq117_e1521_d_b24: f64 = (p.p7 * (s.db[220][24] * ddt_scale));
        let eq117_e1521_d_b25: f64 = (p.p7 * (s.db[220][25] * ddt_scale));
        let eq117_e1521_d_b26: f64 = (p.p7 * (s.db[220][26] * ddt_scale));
        let eq117_e1521_d_b27: f64 = (p.p7 * (s.db[220][27] * ddt_scale));
        let eq117_e1521_d_b28: f64 = (p.p7 * (s.db[220][28] * ddt_scale));
        let eq117_e1521_d_b29: f64 = (p.p7 * (s.db[220][29] * ddt_scale));
        let eq117_e1521_d_b30: f64 = (p.p7 * (s.db[220][30] * ddt_scale));
        let eq117_e1521_d_b31: f64 = (p.p7 * (s.db[220][31] * ddt_scale));
        let eq117_e1521_d_b32: f64 = (p.p7 * (s.db[220][32] * ddt_scale));
        let eq117_e1521_d_b33: f64 = (p.p7 * (s.db[220][33] * ddt_scale));
        let eq117_e1521_d_b34: f64 = (p.p7 * (s.db[220][34] * ddt_scale));
        let eq117_e1521_d_b35: f64 = (p.p7 * (s.db[220][35] * ddt_scale));
        let eq117_e1521_d_b36: f64 = (p.p7 * (s.db[220][36] * ddt_scale));
        let eq117_e1521_d_b37: f64 = (p.p7 * (s.db[220][37] * ddt_scale));
        let eq117_e1521_d_b38: f64 = (p.p7 * (s.db[220][38] * ddt_scale));
        let eq117_e1521_d_b39: f64 = (p.p7 * (s.db[220][39] * ddt_scale));
        let eq117_e1521_d_b40: f64 = (p.p7 * (s.db[220][40] * ddt_scale));
        let eq117_e1521_d_b41: f64 = (p.p7 * (s.db[220][41] * ddt_scale));
        let eq117_e1521_d_b42: f64 = (p.p7 * (s.db[220][42] * ddt_scale));
        let eq117_e1521_d_b43: f64 = (p.p7 * (s.db[220][43] * ddt_scale));
        let eq117_e1521_d_b44: f64 = (p.p7 * (s.db[220][44] * ddt_scale));
        let eq117_e1521_d_b45: f64 = (p.p7 * (s.db[220][45] * ddt_scale));
        let eq117_e1521_d_b46: f64 = (p.p7 * (s.db[220][46] * ddt_scale));
        let eq117_e1521_d_b47: f64 = (p.p7 * (s.db[220][47] * ddt_scale));
        let eq117_e1521_d_b48: f64 = (p.p7 * (s.db[220][48] * ddt_scale));
        let eq117_e1521_d_b49: f64 = (p.p7 * (s.db[220][49] * ddt_scale));
        let eq117_e1521_d_b50: f64 = (p.p7 * (s.db[220][50] * ddt_scale));
        let eq117_e1521_d_b51: f64 = (p.p7 * (s.db[220][51] * ddt_scale));
        let eq117_e1521_d_b52: f64 = (p.p7 * (s.db[220][52] * ddt_scale));
        let eq117_e1521_d_b53: f64 = (p.p7 * (s.db[220][53] * ddt_scale));
        let eq117_e1521_d_b54: f64 = (p.p7 * (s.db[220][54] * ddt_scale));
        let eq117_value: f64 = eq117_e1521;
        let eq117_node_derivatives: [f64; 23] = [eq117_e1521_d_n0, eq117_e1521_d_n1, eq117_e1521_d_n2, eq117_e1521_d_n3, eq117_e1521_d_n4, eq117_e1521_d_n5, eq117_e1521_d_n6, eq117_e1521_d_n7, eq117_e1521_d_n8, eq117_e1521_d_n9, eq117_e1521_d_n10, eq117_e1521_d_n11, eq117_e1521_d_n12, eq117_e1521_d_n13, eq117_e1521_d_n14, eq117_e1521_d_n15, eq117_e1521_d_n16, eq117_e1521_d_n17, eq117_e1521_d_n18, eq117_e1521_d_n19, eq117_e1521_d_n20, eq117_e1521_d_n21, eq117_e1521_d_n22];
        let eq117_branch_derivatives: [f64; 55] = [eq117_e1521_d_b0, eq117_e1521_d_b1, eq117_e1521_d_b2, eq117_e1521_d_b3, eq117_e1521_d_b4, eq117_e1521_d_b5, eq117_e1521_d_b6, eq117_e1521_d_b7, eq117_e1521_d_b8, eq117_e1521_d_b9, eq117_e1521_d_b10, eq117_e1521_d_b11, eq117_e1521_d_b12, eq117_e1521_d_b13, eq117_e1521_d_b14, eq117_e1521_d_b15, eq117_e1521_d_b16, eq117_e1521_d_b17, eq117_e1521_d_b18, eq117_e1521_d_b19, eq117_e1521_d_b20, eq117_e1521_d_b21, eq117_e1521_d_b22, eq117_e1521_d_b23, eq117_e1521_d_b24, eq117_e1521_d_b25, eq117_e1521_d_b26, eq117_e1521_d_b27, eq117_e1521_d_b28, eq117_e1521_d_b29, eq117_e1521_d_b30, eq117_e1521_d_b31, eq117_e1521_d_b32, eq117_e1521_d_b33, eq117_e1521_d_b34, eq117_e1521_d_b35, eq117_e1521_d_b36, eq117_e1521_d_b37, eq117_e1521_d_b38, eq117_e1521_d_b39, eq117_e1521_d_b40, eq117_e1521_d_b41, eq117_e1521_d_b42, eq117_e1521_d_b43, eq117_e1521_d_b44, eq117_e1521_d_b45, eq117_e1521_d_b46, eq117_e1521_d_b47, eq117_e1521_d_b48, eq117_e1521_d_b49, eq117_e1521_d_b50, eq117_e1521_d_b51, eq117_e1521_d_b52, eq117_e1521_d_b53, eq117_e1521_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(2),
            multiplicity * (eq117_value),
            &eq117_node_derivatives,
            &eq117_branch_derivatives,
            multiplicity,
        );
        let eq118_e1524: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, s.v[221]);
        let eq118_e1525: f64 = (p.p7 * eq118_e1524);
        let eq118_e1525_d_n0: f64 = (p.p7 * (s.dn[221][0] * ddt_scale));
        let eq118_e1525_d_n1: f64 = (p.p7 * (s.dn[221][1] * ddt_scale));
        let eq118_e1525_d_n2: f64 = (p.p7 * (s.dn[221][2] * ddt_scale));
        let eq118_e1525_d_n3: f64 = (p.p7 * (s.dn[221][3] * ddt_scale));
        let eq118_e1525_d_n4: f64 = (p.p7 * (s.dn[221][4] * ddt_scale));
        let eq118_e1525_d_n5: f64 = (p.p7 * (s.dn[221][5] * ddt_scale));
        let eq118_e1525_d_n6: f64 = (p.p7 * (s.dn[221][6] * ddt_scale));
        let eq118_e1525_d_n7: f64 = (p.p7 * (s.dn[221][7] * ddt_scale));
        let eq118_e1525_d_n8: f64 = (p.p7 * (s.dn[221][8] * ddt_scale));
        let eq118_e1525_d_n9: f64 = (p.p7 * (s.dn[221][9] * ddt_scale));
        let eq118_e1525_d_n10: f64 = (p.p7 * (s.dn[221][10] * ddt_scale));
        let eq118_e1525_d_n11: f64 = (p.p7 * (s.dn[221][11] * ddt_scale));
        let eq118_e1525_d_n12: f64 = (p.p7 * (s.dn[221][12] * ddt_scale));
        let eq118_e1525_d_n13: f64 = (p.p7 * (s.dn[221][13] * ddt_scale));
        let eq118_e1525_d_n14: f64 = (p.p7 * (s.dn[221][14] * ddt_scale));
        let eq118_e1525_d_n15: f64 = (p.p7 * (s.dn[221][15] * ddt_scale));
        let eq118_e1525_d_n16: f64 = (p.p7 * (s.dn[221][16] * ddt_scale));
        let eq118_e1525_d_n17: f64 = (p.p7 * (s.dn[221][17] * ddt_scale));
        let eq118_e1525_d_n18: f64 = (p.p7 * (s.dn[221][18] * ddt_scale));
        let eq118_e1525_d_n19: f64 = (p.p7 * (s.dn[221][19] * ddt_scale));
        let eq118_e1525_d_n20: f64 = (p.p7 * (s.dn[221][20] * ddt_scale));
        let eq118_e1525_d_n21: f64 = (p.p7 * (s.dn[221][21] * ddt_scale));
        let eq118_e1525_d_n22: f64 = (p.p7 * (s.dn[221][22] * ddt_scale));
        let eq118_e1525_d_b0: f64 = (p.p7 * (s.db[221][0] * ddt_scale));
        let eq118_e1525_d_b1: f64 = (p.p7 * (s.db[221][1] * ddt_scale));
        let eq118_e1525_d_b2: f64 = (p.p7 * (s.db[221][2] * ddt_scale));
        let eq118_e1525_d_b3: f64 = (p.p7 * (s.db[221][3] * ddt_scale));
        let eq118_e1525_d_b4: f64 = (p.p7 * (s.db[221][4] * ddt_scale));
        let eq118_e1525_d_b5: f64 = (p.p7 * (s.db[221][5] * ddt_scale));
        let eq118_e1525_d_b6: f64 = (p.p7 * (s.db[221][6] * ddt_scale));
        let eq118_e1525_d_b7: f64 = (p.p7 * (s.db[221][7] * ddt_scale));
        let eq118_e1525_d_b8: f64 = (p.p7 * (s.db[221][8] * ddt_scale));
        let eq118_e1525_d_b9: f64 = (p.p7 * (s.db[221][9] * ddt_scale));
        let eq118_e1525_d_b10: f64 = (p.p7 * (s.db[221][10] * ddt_scale));
        let eq118_e1525_d_b11: f64 = (p.p7 * (s.db[221][11] * ddt_scale));
        let eq118_e1525_d_b12: f64 = (p.p7 * (s.db[221][12] * ddt_scale));
        let eq118_e1525_d_b13: f64 = (p.p7 * (s.db[221][13] * ddt_scale));
        let eq118_e1525_d_b14: f64 = (p.p7 * (s.db[221][14] * ddt_scale));
        let eq118_e1525_d_b15: f64 = (p.p7 * (s.db[221][15] * ddt_scale));
        let eq118_e1525_d_b16: f64 = (p.p7 * (s.db[221][16] * ddt_scale));
        let eq118_e1525_d_b17: f64 = (p.p7 * (s.db[221][17] * ddt_scale));
        let eq118_e1525_d_b18: f64 = (p.p7 * (s.db[221][18] * ddt_scale));
        let eq118_e1525_d_b19: f64 = (p.p7 * (s.db[221][19] * ddt_scale));
        let eq118_e1525_d_b20: f64 = (p.p7 * (s.db[221][20] * ddt_scale));
        let eq118_e1525_d_b21: f64 = (p.p7 * (s.db[221][21] * ddt_scale));
        let eq118_e1525_d_b22: f64 = (p.p7 * (s.db[221][22] * ddt_scale));
        let eq118_e1525_d_b23: f64 = (p.p7 * (s.db[221][23] * ddt_scale));
        let eq118_e1525_d_b24: f64 = (p.p7 * (s.db[221][24] * ddt_scale));
        let eq118_e1525_d_b25: f64 = (p.p7 * (s.db[221][25] * ddt_scale));
        let eq118_e1525_d_b26: f64 = (p.p7 * (s.db[221][26] * ddt_scale));
        let eq118_e1525_d_b27: f64 = (p.p7 * (s.db[221][27] * ddt_scale));
        let eq118_e1525_d_b28: f64 = (p.p7 * (s.db[221][28] * ddt_scale));
        let eq118_e1525_d_b29: f64 = (p.p7 * (s.db[221][29] * ddt_scale));
        let eq118_e1525_d_b30: f64 = (p.p7 * (s.db[221][30] * ddt_scale));
        let eq118_e1525_d_b31: f64 = (p.p7 * (s.db[221][31] * ddt_scale));
        let eq118_e1525_d_b32: f64 = (p.p7 * (s.db[221][32] * ddt_scale));
        let eq118_e1525_d_b33: f64 = (p.p7 * (s.db[221][33] * ddt_scale));
        let eq118_e1525_d_b34: f64 = (p.p7 * (s.db[221][34] * ddt_scale));
        let eq118_e1525_d_b35: f64 = (p.p7 * (s.db[221][35] * ddt_scale));
        let eq118_e1525_d_b36: f64 = (p.p7 * (s.db[221][36] * ddt_scale));
        let eq118_e1525_d_b37: f64 = (p.p7 * (s.db[221][37] * ddt_scale));
        let eq118_e1525_d_b38: f64 = (p.p7 * (s.db[221][38] * ddt_scale));
        let eq118_e1525_d_b39: f64 = (p.p7 * (s.db[221][39] * ddt_scale));
        let eq118_e1525_d_b40: f64 = (p.p7 * (s.db[221][40] * ddt_scale));
        let eq118_e1525_d_b41: f64 = (p.p7 * (s.db[221][41] * ddt_scale));
        let eq118_e1525_d_b42: f64 = (p.p7 * (s.db[221][42] * ddt_scale));
        let eq118_e1525_d_b43: f64 = (p.p7 * (s.db[221][43] * ddt_scale));
        let eq118_e1525_d_b44: f64 = (p.p7 * (s.db[221][44] * ddt_scale));
        let eq118_e1525_d_b45: f64 = (p.p7 * (s.db[221][45] * ddt_scale));
        let eq118_e1525_d_b46: f64 = (p.p7 * (s.db[221][46] * ddt_scale));
        let eq118_e1525_d_b47: f64 = (p.p7 * (s.db[221][47] * ddt_scale));
        let eq118_e1525_d_b48: f64 = (p.p7 * (s.db[221][48] * ddt_scale));
        let eq118_e1525_d_b49: f64 = (p.p7 * (s.db[221][49] * ddt_scale));
        let eq118_e1525_d_b50: f64 = (p.p7 * (s.db[221][50] * ddt_scale));
        let eq118_e1525_d_b51: f64 = (p.p7 * (s.db[221][51] * ddt_scale));
        let eq118_e1525_d_b52: f64 = (p.p7 * (s.db[221][52] * ddt_scale));
        let eq118_e1525_d_b53: f64 = (p.p7 * (s.db[221][53] * ddt_scale));
        let eq118_e1525_d_b54: f64 = (p.p7 * (s.db[221][54] * ddt_scale));
        let eq118_value: f64 = eq118_e1525;
        let eq118_node_derivatives: [f64; 23] = [eq118_e1525_d_n0, eq118_e1525_d_n1, eq118_e1525_d_n2, eq118_e1525_d_n3, eq118_e1525_d_n4, eq118_e1525_d_n5, eq118_e1525_d_n6, eq118_e1525_d_n7, eq118_e1525_d_n8, eq118_e1525_d_n9, eq118_e1525_d_n10, eq118_e1525_d_n11, eq118_e1525_d_n12, eq118_e1525_d_n13, eq118_e1525_d_n14, eq118_e1525_d_n15, eq118_e1525_d_n16, eq118_e1525_d_n17, eq118_e1525_d_n18, eq118_e1525_d_n19, eq118_e1525_d_n20, eq118_e1525_d_n21, eq118_e1525_d_n22];
        let eq118_branch_derivatives: [f64; 55] = [eq118_e1525_d_b0, eq118_e1525_d_b1, eq118_e1525_d_b2, eq118_e1525_d_b3, eq118_e1525_d_b4, eq118_e1525_d_b5, eq118_e1525_d_b6, eq118_e1525_d_b7, eq118_e1525_d_b8, eq118_e1525_d_b9, eq118_e1525_d_b10, eq118_e1525_d_b11, eq118_e1525_d_b12, eq118_e1525_d_b13, eq118_e1525_d_b14, eq118_e1525_d_b15, eq118_e1525_d_b16, eq118_e1525_d_b17, eq118_e1525_d_b18, eq118_e1525_d_b19, eq118_e1525_d_b20, eq118_e1525_d_b21, eq118_e1525_d_b22, eq118_e1525_d_b23, eq118_e1525_d_b24, eq118_e1525_d_b25, eq118_e1525_d_b26, eq118_e1525_d_b27, eq118_e1525_d_b28, eq118_e1525_d_b29, eq118_e1525_d_b30, eq118_e1525_d_b31, eq118_e1525_d_b32, eq118_e1525_d_b33, eq118_e1525_d_b34, eq118_e1525_d_b35, eq118_e1525_d_b36, eq118_e1525_d_b37, eq118_e1525_d_b38, eq118_e1525_d_b39, eq118_e1525_d_b40, eq118_e1525_d_b41, eq118_e1525_d_b42, eq118_e1525_d_b43, eq118_e1525_d_b44, eq118_e1525_d_b45, eq118_e1525_d_b46, eq118_e1525_d_b47, eq118_e1525_d_b48, eq118_e1525_d_b49, eq118_e1525_d_b50, eq118_e1525_d_b51, eq118_e1525_d_b52, eq118_e1525_d_b53, eq118_e1525_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(1),
            multiplicity * (eq118_value),
            &eq118_node_derivatives,
            &eq118_branch_derivatives,
            multiplicity,
        );
        let eq119_e1529: f64 = (p.p250 * s.v[161]);
        let eq119_e1530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq119_e1529);
        let eq119_e1530_d_n0: f64 = ((p.p250 * s.dn[161][0]) * ddt_scale);
        let eq119_e1530_d_n1: f64 = ((p.p250 * s.dn[161][1]) * ddt_scale);
        let eq119_e1530_d_n2: f64 = ((p.p250 * s.dn[161][2]) * ddt_scale);
        let eq119_e1530_d_n3: f64 = ((p.p250 * s.dn[161][3]) * ddt_scale);
        let eq119_e1530_d_n4: f64 = ((p.p250 * s.dn[161][4]) * ddt_scale);
        let eq119_e1530_d_n5: f64 = ((p.p250 * s.dn[161][5]) * ddt_scale);
        let eq119_e1530_d_n6: f64 = ((p.p250 * s.dn[161][6]) * ddt_scale);
        let eq119_e1530_d_n7: f64 = ((p.p250 * s.dn[161][7]) * ddt_scale);
        let eq119_e1530_d_n8: f64 = ((p.p250 * s.dn[161][8]) * ddt_scale);
        let eq119_e1530_d_n9: f64 = ((p.p250 * s.dn[161][9]) * ddt_scale);
        let eq119_e1530_d_n10: f64 = ((p.p250 * s.dn[161][10]) * ddt_scale);
        let eq119_e1530_d_n11: f64 = ((p.p250 * s.dn[161][11]) * ddt_scale);
        let eq119_e1530_d_n12: f64 = ((p.p250 * s.dn[161][12]) * ddt_scale);
        let eq119_e1530_d_n13: f64 = ((p.p250 * s.dn[161][13]) * ddt_scale);
        let eq119_e1530_d_n14: f64 = ((p.p250 * s.dn[161][14]) * ddt_scale);
        let eq119_e1530_d_n15: f64 = ((p.p250 * s.dn[161][15]) * ddt_scale);
        let eq119_e1530_d_n16: f64 = ((p.p250 * s.dn[161][16]) * ddt_scale);
        let eq119_e1530_d_n17: f64 = ((p.p250 * s.dn[161][17]) * ddt_scale);
        let eq119_e1530_d_n18: f64 = ((p.p250 * s.dn[161][18]) * ddt_scale);
        let eq119_e1530_d_n19: f64 = ((p.p250 * s.dn[161][19]) * ddt_scale);
        let eq119_e1530_d_n20: f64 = ((p.p250 * s.dn[161][20]) * ddt_scale);
        let eq119_e1530_d_n21: f64 = ((p.p250 * s.dn[161][21]) * ddt_scale);
        let eq119_e1530_d_n22: f64 = ((p.p250 * s.dn[161][22]) * ddt_scale);
        let eq119_e1530_d_b0: f64 = ((p.p250 * s.db[161][0]) * ddt_scale);
        let eq119_e1530_d_b1: f64 = ((p.p250 * s.db[161][1]) * ddt_scale);
        let eq119_e1530_d_b2: f64 = ((p.p250 * s.db[161][2]) * ddt_scale);
        let eq119_e1530_d_b3: f64 = ((p.p250 * s.db[161][3]) * ddt_scale);
        let eq119_e1530_d_b4: f64 = ((p.p250 * s.db[161][4]) * ddt_scale);
        let eq119_e1530_d_b5: f64 = ((p.p250 * s.db[161][5]) * ddt_scale);
        let eq119_e1530_d_b6: f64 = ((p.p250 * s.db[161][6]) * ddt_scale);
        let eq119_e1530_d_b7: f64 = ((p.p250 * s.db[161][7]) * ddt_scale);
        let eq119_e1530_d_b8: f64 = ((p.p250 * s.db[161][8]) * ddt_scale);
        let eq119_e1530_d_b9: f64 = ((p.p250 * s.db[161][9]) * ddt_scale);
        let eq119_e1530_d_b10: f64 = ((p.p250 * s.db[161][10]) * ddt_scale);
        let eq119_e1530_d_b11: f64 = ((p.p250 * s.db[161][11]) * ddt_scale);
        let eq119_e1530_d_b12: f64 = ((p.p250 * s.db[161][12]) * ddt_scale);
        let eq119_e1530_d_b13: f64 = ((p.p250 * s.db[161][13]) * ddt_scale);
        let eq119_e1530_d_b14: f64 = ((p.p250 * s.db[161][14]) * ddt_scale);
        let eq119_e1530_d_b15: f64 = ((p.p250 * s.db[161][15]) * ddt_scale);
        let eq119_e1530_d_b16: f64 = ((p.p250 * s.db[161][16]) * ddt_scale);
        let eq119_e1530_d_b17: f64 = ((p.p250 * s.db[161][17]) * ddt_scale);
        let eq119_e1530_d_b18: f64 = ((p.p250 * s.db[161][18]) * ddt_scale);
        let eq119_e1530_d_b19: f64 = ((p.p250 * s.db[161][19]) * ddt_scale);
        let eq119_e1530_d_b20: f64 = ((p.p250 * s.db[161][20]) * ddt_scale);
        let eq119_e1530_d_b21: f64 = ((p.p250 * s.db[161][21]) * ddt_scale);
        let eq119_e1530_d_b22: f64 = ((p.p250 * s.db[161][22]) * ddt_scale);
        let eq119_e1530_d_b23: f64 = ((p.p250 * s.db[161][23]) * ddt_scale);
        let eq119_e1530_d_b24: f64 = ((p.p250 * s.db[161][24]) * ddt_scale);
        let eq119_e1530_d_b25: f64 = ((p.p250 * s.db[161][25]) * ddt_scale);
        let eq119_e1530_d_b26: f64 = ((p.p250 * s.db[161][26]) * ddt_scale);
        let eq119_e1530_d_b27: f64 = ((p.p250 * s.db[161][27]) * ddt_scale);
        let eq119_e1530_d_b28: f64 = ((p.p250 * s.db[161][28]) * ddt_scale);
        let eq119_e1530_d_b29: f64 = ((p.p250 * s.db[161][29]) * ddt_scale);
        let eq119_e1530_d_b30: f64 = ((p.p250 * s.db[161][30]) * ddt_scale);
        let eq119_e1530_d_b31: f64 = ((p.p250 * s.db[161][31]) * ddt_scale);
        let eq119_e1530_d_b32: f64 = ((p.p250 * s.db[161][32]) * ddt_scale);
        let eq119_e1530_d_b33: f64 = ((p.p250 * s.db[161][33]) * ddt_scale);
        let eq119_e1530_d_b34: f64 = ((p.p250 * s.db[161][34]) * ddt_scale);
        let eq119_e1530_d_b35: f64 = ((p.p250 * s.db[161][35]) * ddt_scale);
        let eq119_e1530_d_b36: f64 = ((p.p250 * s.db[161][36]) * ddt_scale);
        let eq119_e1530_d_b37: f64 = ((p.p250 * s.db[161][37]) * ddt_scale);
        let eq119_e1530_d_b38: f64 = ((p.p250 * s.db[161][38]) * ddt_scale);
        let eq119_e1530_d_b39: f64 = ((p.p250 * s.db[161][39]) * ddt_scale);
        let eq119_e1530_d_b40: f64 = ((p.p250 * s.db[161][40]) * ddt_scale);
        let eq119_e1530_d_b41: f64 = ((p.p250 * s.db[161][41]) * ddt_scale);
        let eq119_e1530_d_b42: f64 = ((p.p250 * s.db[161][42]) * ddt_scale);
        let eq119_e1530_d_b43: f64 = ((p.p250 * s.db[161][43]) * ddt_scale);
        let eq119_e1530_d_b44: f64 = ((p.p250 * s.db[161][44]) * ddt_scale);
        let eq119_e1530_d_b45: f64 = ((p.p250 * s.db[161][45]) * ddt_scale);
        let eq119_e1530_d_b46: f64 = ((p.p250 * s.db[161][46]) * ddt_scale);
        let eq119_e1530_d_b47: f64 = ((p.p250 * s.db[161][47]) * ddt_scale);
        let eq119_e1530_d_b48: f64 = ((p.p250 * s.db[161][48]) * ddt_scale);
        let eq119_e1530_d_b49: f64 = ((p.p250 * s.db[161][49]) * ddt_scale);
        let eq119_e1530_d_b50: f64 = ((p.p250 * s.db[161][50]) * ddt_scale);
        let eq119_e1530_d_b51: f64 = ((p.p250 * s.db[161][51]) * ddt_scale);
        let eq119_e1530_d_b52: f64 = ((p.p250 * s.db[161][52]) * ddt_scale);
        let eq119_e1530_d_b53: f64 = ((p.p250 * s.db[161][53]) * ddt_scale);
        let eq119_e1530_d_b54: f64 = ((p.p250 * s.db[161][54]) * ddt_scale);
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
        let eq119_e1531_d_b0: f64 = (p.p7 * eq119_e1530_d_b0);
        let eq119_e1531_d_b1: f64 = (p.p7 * eq119_e1530_d_b1);
        let eq119_e1531_d_b2: f64 = (p.p7 * eq119_e1530_d_b2);
        let eq119_e1531_d_b3: f64 = (p.p7 * eq119_e1530_d_b3);
        let eq119_e1531_d_b4: f64 = (p.p7 * eq119_e1530_d_b4);
        let eq119_e1531_d_b5: f64 = (p.p7 * eq119_e1530_d_b5);
        let eq119_e1531_d_b6: f64 = (p.p7 * eq119_e1530_d_b6);
        let eq119_e1531_d_b7: f64 = (p.p7 * eq119_e1530_d_b7);
        let eq119_e1531_d_b8: f64 = (p.p7 * eq119_e1530_d_b8);
        let eq119_e1531_d_b9: f64 = (p.p7 * eq119_e1530_d_b9);
        let eq119_e1531_d_b10: f64 = (p.p7 * eq119_e1530_d_b10);
        let eq119_e1531_d_b11: f64 = (p.p7 * eq119_e1530_d_b11);
        let eq119_e1531_d_b12: f64 = (p.p7 * eq119_e1530_d_b12);
        let eq119_e1531_d_b13: f64 = (p.p7 * eq119_e1530_d_b13);
        let eq119_e1531_d_b14: f64 = (p.p7 * eq119_e1530_d_b14);
        let eq119_e1531_d_b15: f64 = (p.p7 * eq119_e1530_d_b15);
        let eq119_e1531_d_b16: f64 = (p.p7 * eq119_e1530_d_b16);
        let eq119_e1531_d_b17: f64 = (p.p7 * eq119_e1530_d_b17);
        let eq119_e1531_d_b18: f64 = (p.p7 * eq119_e1530_d_b18);
        let eq119_e1531_d_b19: f64 = (p.p7 * eq119_e1530_d_b19);
        let eq119_e1531_d_b20: f64 = (p.p7 * eq119_e1530_d_b20);
        let eq119_e1531_d_b21: f64 = (p.p7 * eq119_e1530_d_b21);
        let eq119_e1531_d_b22: f64 = (p.p7 * eq119_e1530_d_b22);
        let eq119_e1531_d_b23: f64 = (p.p7 * eq119_e1530_d_b23);
        let eq119_e1531_d_b24: f64 = (p.p7 * eq119_e1530_d_b24);
        let eq119_e1531_d_b25: f64 = (p.p7 * eq119_e1530_d_b25);
        let eq119_e1531_d_b26: f64 = (p.p7 * eq119_e1530_d_b26);
        let eq119_e1531_d_b27: f64 = (p.p7 * eq119_e1530_d_b27);
        let eq119_e1531_d_b28: f64 = (p.p7 * eq119_e1530_d_b28);
        let eq119_e1531_d_b29: f64 = (p.p7 * eq119_e1530_d_b29);
        let eq119_e1531_d_b30: f64 = (p.p7 * eq119_e1530_d_b30);
        let eq119_e1531_d_b31: f64 = (p.p7 * eq119_e1530_d_b31);
        let eq119_e1531_d_b32: f64 = (p.p7 * eq119_e1530_d_b32);
        let eq119_e1531_d_b33: f64 = (p.p7 * eq119_e1530_d_b33);
        let eq119_e1531_d_b34: f64 = (p.p7 * eq119_e1530_d_b34);
        let eq119_e1531_d_b35: f64 = (p.p7 * eq119_e1530_d_b35);
        let eq119_e1531_d_b36: f64 = (p.p7 * eq119_e1530_d_b36);
        let eq119_e1531_d_b37: f64 = (p.p7 * eq119_e1530_d_b37);
        let eq119_e1531_d_b38: f64 = (p.p7 * eq119_e1530_d_b38);
        let eq119_e1531_d_b39: f64 = (p.p7 * eq119_e1530_d_b39);
        let eq119_e1531_d_b40: f64 = (p.p7 * eq119_e1530_d_b40);
        let eq119_e1531_d_b41: f64 = (p.p7 * eq119_e1530_d_b41);
        let eq119_e1531_d_b42: f64 = (p.p7 * eq119_e1530_d_b42);
        let eq119_e1531_d_b43: f64 = (p.p7 * eq119_e1530_d_b43);
        let eq119_e1531_d_b44: f64 = (p.p7 * eq119_e1530_d_b44);
        let eq119_e1531_d_b45: f64 = (p.p7 * eq119_e1530_d_b45);
        let eq119_e1531_d_b46: f64 = (p.p7 * eq119_e1530_d_b46);
        let eq119_e1531_d_b47: f64 = (p.p7 * eq119_e1530_d_b47);
        let eq119_e1531_d_b48: f64 = (p.p7 * eq119_e1530_d_b48);
        let eq119_e1531_d_b49: f64 = (p.p7 * eq119_e1530_d_b49);
        let eq119_e1531_d_b50: f64 = (p.p7 * eq119_e1530_d_b50);
        let eq119_e1531_d_b51: f64 = (p.p7 * eq119_e1530_d_b51);
        let eq119_e1531_d_b52: f64 = (p.p7 * eq119_e1530_d_b52);
        let eq119_e1531_d_b53: f64 = (p.p7 * eq119_e1530_d_b53);
        let eq119_e1531_d_b54: f64 = (p.p7 * eq119_e1530_d_b54);
        let eq119_value: f64 = eq119_e1531;
        let eq119_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n10, eq119_e1531_d_n11, eq119_e1531_d_n12, eq119_e1531_d_n13, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_branch_derivatives: [f64; 55] = [eq119_e1531_d_b0, eq119_e1531_d_b1, eq119_e1531_d_b2, eq119_e1531_d_b3, eq119_e1531_d_b4, eq119_e1531_d_b5, eq119_e1531_d_b6, eq119_e1531_d_b7, eq119_e1531_d_b8, eq119_e1531_d_b9, eq119_e1531_d_b10, eq119_e1531_d_b11, eq119_e1531_d_b12, eq119_e1531_d_b13, eq119_e1531_d_b14, eq119_e1531_d_b15, eq119_e1531_d_b16, eq119_e1531_d_b17, eq119_e1531_d_b18, eq119_e1531_d_b19, eq119_e1531_d_b20, eq119_e1531_d_b21, eq119_e1531_d_b22, eq119_e1531_d_b23, eq119_e1531_d_b24, eq119_e1531_d_b25, eq119_e1531_d_b26, eq119_e1531_d_b27, eq119_e1531_d_b28, eq119_e1531_d_b29, eq119_e1531_d_b30, eq119_e1531_d_b31, eq119_e1531_d_b32, eq119_e1531_d_b33, eq119_e1531_d_b34, eq119_e1531_d_b35, eq119_e1531_d_b36, eq119_e1531_d_b37, eq119_e1531_d_b38, eq119_e1531_d_b39, eq119_e1531_d_b40, eq119_e1531_d_b41, eq119_e1531_d_b42, eq119_e1531_d_b43, eq119_e1531_d_b44, eq119_e1531_d_b45, eq119_e1531_d_b46, eq119_e1531_d_b47, eq119_e1531_d_b48, eq119_e1531_d_b49, eq119_e1531_d_b50, eq119_e1531_d_b51, eq119_e1531_d_b52, eq119_e1531_d_b53, eq119_e1531_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq119_value),
            &eq119_node_derivatives,
            &eq119_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_15(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22, eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54,) = {
    if (s.b[570] && s.b[571]) {
        let eq120_e1537: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, s.v[229]);
        let eq120_e1538: f64 = (p.p7 * eq120_e1537);
        let eq120_e1538_d_n0: f64 = (p.p7 * (s.dn[229][0] * ddt_scale));
        let eq120_e1538_d_n1: f64 = (p.p7 * (s.dn[229][1] * ddt_scale));
        let eq120_e1538_d_n2: f64 = (p.p7 * (s.dn[229][2] * ddt_scale));
        let eq120_e1538_d_n3: f64 = (p.p7 * (s.dn[229][3] * ddt_scale));
        let eq120_e1538_d_n4: f64 = (p.p7 * (s.dn[229][4] * ddt_scale));
        let eq120_e1538_d_n5: f64 = (p.p7 * (s.dn[229][5] * ddt_scale));
        let eq120_e1538_d_n6: f64 = (p.p7 * (s.dn[229][6] * ddt_scale));
        let eq120_e1538_d_n7: f64 = (p.p7 * (s.dn[229][7] * ddt_scale));
        let eq120_e1538_d_n8: f64 = (p.p7 * (s.dn[229][8] * ddt_scale));
        let eq120_e1538_d_n9: f64 = (p.p7 * (s.dn[229][9] * ddt_scale));
        let eq120_e1538_d_n10: f64 = (p.p7 * (s.dn[229][10] * ddt_scale));
        let eq120_e1538_d_n11: f64 = (p.p7 * (s.dn[229][11] * ddt_scale));
        let eq120_e1538_d_n12: f64 = (p.p7 * (s.dn[229][12] * ddt_scale));
        let eq120_e1538_d_n13: f64 = (p.p7 * (s.dn[229][13] * ddt_scale));
        let eq120_e1538_d_n14: f64 = (p.p7 * (s.dn[229][14] * ddt_scale));
        let eq120_e1538_d_n15: f64 = (p.p7 * (s.dn[229][15] * ddt_scale));
        let eq120_e1538_d_n16: f64 = (p.p7 * (s.dn[229][16] * ddt_scale));
        let eq120_e1538_d_n17: f64 = (p.p7 * (s.dn[229][17] * ddt_scale));
        let eq120_e1538_d_n18: f64 = (p.p7 * (s.dn[229][18] * ddt_scale));
        let eq120_e1538_d_n19: f64 = (p.p7 * (s.dn[229][19] * ddt_scale));
        let eq120_e1538_d_n20: f64 = (p.p7 * (s.dn[229][20] * ddt_scale));
        let eq120_e1538_d_n21: f64 = (p.p7 * (s.dn[229][21] * ddt_scale));
        let eq120_e1538_d_n22: f64 = (p.p7 * (s.dn[229][22] * ddt_scale));
        let eq120_e1538_d_b0: f64 = (p.p7 * (s.db[229][0] * ddt_scale));
        let eq120_e1538_d_b1: f64 = (p.p7 * (s.db[229][1] * ddt_scale));
        let eq120_e1538_d_b2: f64 = (p.p7 * (s.db[229][2] * ddt_scale));
        let eq120_e1538_d_b3: f64 = (p.p7 * (s.db[229][3] * ddt_scale));
        let eq120_e1538_d_b4: f64 = (p.p7 * (s.db[229][4] * ddt_scale));
        let eq120_e1538_d_b5: f64 = (p.p7 * (s.db[229][5] * ddt_scale));
        let eq120_e1538_d_b6: f64 = (p.p7 * (s.db[229][6] * ddt_scale));
        let eq120_e1538_d_b7: f64 = (p.p7 * (s.db[229][7] * ddt_scale));
        let eq120_e1538_d_b8: f64 = (p.p7 * (s.db[229][8] * ddt_scale));
        let eq120_e1538_d_b9: f64 = (p.p7 * (s.db[229][9] * ddt_scale));
        let eq120_e1538_d_b10: f64 = (p.p7 * (s.db[229][10] * ddt_scale));
        let eq120_e1538_d_b11: f64 = (p.p7 * (s.db[229][11] * ddt_scale));
        let eq120_e1538_d_b12: f64 = (p.p7 * (s.db[229][12] * ddt_scale));
        let eq120_e1538_d_b13: f64 = (p.p7 * (s.db[229][13] * ddt_scale));
        let eq120_e1538_d_b14: f64 = (p.p7 * (s.db[229][14] * ddt_scale));
        let eq120_e1538_d_b15: f64 = (p.p7 * (s.db[229][15] * ddt_scale));
        let eq120_e1538_d_b16: f64 = (p.p7 * (s.db[229][16] * ddt_scale));
        let eq120_e1538_d_b17: f64 = (p.p7 * (s.db[229][17] * ddt_scale));
        let eq120_e1538_d_b18: f64 = (p.p7 * (s.db[229][18] * ddt_scale));
        let eq120_e1538_d_b19: f64 = (p.p7 * (s.db[229][19] * ddt_scale));
        let eq120_e1538_d_b20: f64 = (p.p7 * (s.db[229][20] * ddt_scale));
        let eq120_e1538_d_b21: f64 = (p.p7 * (s.db[229][21] * ddt_scale));
        let eq120_e1538_d_b22: f64 = (p.p7 * (s.db[229][22] * ddt_scale));
        let eq120_e1538_d_b23: f64 = (p.p7 * (s.db[229][23] * ddt_scale));
        let eq120_e1538_d_b24: f64 = (p.p7 * (s.db[229][24] * ddt_scale));
        let eq120_e1538_d_b25: f64 = (p.p7 * (s.db[229][25] * ddt_scale));
        let eq120_e1538_d_b26: f64 = (p.p7 * (s.db[229][26] * ddt_scale));
        let eq120_e1538_d_b27: f64 = (p.p7 * (s.db[229][27] * ddt_scale));
        let eq120_e1538_d_b28: f64 = (p.p7 * (s.db[229][28] * ddt_scale));
        let eq120_e1538_d_b29: f64 = (p.p7 * (s.db[229][29] * ddt_scale));
        let eq120_e1538_d_b30: f64 = (p.p7 * (s.db[229][30] * ddt_scale));
        let eq120_e1538_d_b31: f64 = (p.p7 * (s.db[229][31] * ddt_scale));
        let eq120_e1538_d_b32: f64 = (p.p7 * (s.db[229][32] * ddt_scale));
        let eq120_e1538_d_b33: f64 = (p.p7 * (s.db[229][33] * ddt_scale));
        let eq120_e1538_d_b34: f64 = (p.p7 * (s.db[229][34] * ddt_scale));
        let eq120_e1538_d_b35: f64 = (p.p7 * (s.db[229][35] * ddt_scale));
        let eq120_e1538_d_b36: f64 = (p.p7 * (s.db[229][36] * ddt_scale));
        let eq120_e1538_d_b37: f64 = (p.p7 * (s.db[229][37] * ddt_scale));
        let eq120_e1538_d_b38: f64 = (p.p7 * (s.db[229][38] * ddt_scale));
        let eq120_e1538_d_b39: f64 = (p.p7 * (s.db[229][39] * ddt_scale));
        let eq120_e1538_d_b40: f64 = (p.p7 * (s.db[229][40] * ddt_scale));
        let eq120_e1538_d_b41: f64 = (p.p7 * (s.db[229][41] * ddt_scale));
        let eq120_e1538_d_b42: f64 = (p.p7 * (s.db[229][42] * ddt_scale));
        let eq120_e1538_d_b43: f64 = (p.p7 * (s.db[229][43] * ddt_scale));
        let eq120_e1538_d_b44: f64 = (p.p7 * (s.db[229][44] * ddt_scale));
        let eq120_e1538_d_b45: f64 = (p.p7 * (s.db[229][45] * ddt_scale));
        let eq120_e1538_d_b46: f64 = (p.p7 * (s.db[229][46] * ddt_scale));
        let eq120_e1538_d_b47: f64 = (p.p7 * (s.db[229][47] * ddt_scale));
        let eq120_e1538_d_b48: f64 = (p.p7 * (s.db[229][48] * ddt_scale));
        let eq120_e1538_d_b49: f64 = (p.p7 * (s.db[229][49] * ddt_scale));
        let eq120_e1538_d_b50: f64 = (p.p7 * (s.db[229][50] * ddt_scale));
        let eq120_e1538_d_b51: f64 = (p.p7 * (s.db[229][51] * ddt_scale));
        let eq120_e1538_d_b52: f64 = (p.p7 * (s.db[229][52] * ddt_scale));
        let eq120_e1538_d_b53: f64 = (p.p7 * (s.db[229][53] * ddt_scale));
        let eq120_e1538_d_b54: f64 = (p.p7 * (s.db[229][54] * ddt_scale));
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n10, eq120_e1538_d_n11, eq120_e1538_d_n12, eq120_e1538_d_n13, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22, eq120_e1538_d_b0, eq120_e1538_d_b1, eq120_e1538_d_b2, eq120_e1538_d_b3, eq120_e1538_d_b4, eq120_e1538_d_b5, eq120_e1538_d_b6, eq120_e1538_d_b7, eq120_e1538_d_b8, eq120_e1538_d_b9, eq120_e1538_d_b10, eq120_e1538_d_b11, eq120_e1538_d_b12, eq120_e1538_d_b13, eq120_e1538_d_b14, eq120_e1538_d_b15, eq120_e1538_d_b16, eq120_e1538_d_b17, eq120_e1538_d_b18, eq120_e1538_d_b19, eq120_e1538_d_b20, eq120_e1538_d_b21, eq120_e1538_d_b22, eq120_e1538_d_b23, eq120_e1538_d_b24, eq120_e1538_d_b25, eq120_e1538_d_b26, eq120_e1538_d_b27, eq120_e1538_d_b28, eq120_e1538_d_b29, eq120_e1538_d_b30, eq120_e1538_d_b31, eq120_e1538_d_b32, eq120_e1538_d_b33, eq120_e1538_d_b34, eq120_e1538_d_b35, eq120_e1538_d_b36, eq120_e1538_d_b37, eq120_e1538_d_b38, eq120_e1538_d_b39, eq120_e1538_d_b40, eq120_e1538_d_b41, eq120_e1538_d_b42, eq120_e1538_d_b43, eq120_e1538_d_b44, eq120_e1538_d_b45, eq120_e1538_d_b46, eq120_e1538_d_b47, eq120_e1538_d_b48, eq120_e1538_d_b49, eq120_e1538_d_b50, eq120_e1538_d_b51, eq120_e1538_d_b52, eq120_e1538_d_b53, eq120_e1538_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_value: f64 = eq120_e1540;
        let eq120_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_branch_derivatives: [f64; 55] = [eq120_e1540_d_b0, eq120_e1540_d_b1, eq120_e1540_d_b2, eq120_e1540_d_b3, eq120_e1540_d_b4, eq120_e1540_d_b5, eq120_e1540_d_b6, eq120_e1540_d_b7, eq120_e1540_d_b8, eq120_e1540_d_b9, eq120_e1540_d_b10, eq120_e1540_d_b11, eq120_e1540_d_b12, eq120_e1540_d_b13, eq120_e1540_d_b14, eq120_e1540_d_b15, eq120_e1540_d_b16, eq120_e1540_d_b17, eq120_e1540_d_b18, eq120_e1540_d_b19, eq120_e1540_d_b20, eq120_e1540_d_b21, eq120_e1540_d_b22, eq120_e1540_d_b23, eq120_e1540_d_b24, eq120_e1540_d_b25, eq120_e1540_d_b26, eq120_e1540_d_b27, eq120_e1540_d_b28, eq120_e1540_d_b29, eq120_e1540_d_b30, eq120_e1540_d_b31, eq120_e1540_d_b32, eq120_e1540_d_b33, eq120_e1540_d_b34, eq120_e1540_d_b35, eq120_e1540_d_b36, eq120_e1540_d_b37, eq120_e1540_d_b38, eq120_e1540_d_b39, eq120_e1540_d_b40, eq120_e1540_d_b41, eq120_e1540_d_b42, eq120_e1540_d_b43, eq120_e1540_d_b44, eq120_e1540_d_b45, eq120_e1540_d_b46, eq120_e1540_d_b47, eq120_e1540_d_b48, eq120_e1540_d_b49, eq120_e1540_d_b50, eq120_e1540_d_b51, eq120_e1540_d_b52, eq120_e1540_d_b53, eq120_e1540_d_b54];
        stamper.stamp_current_dense_local(
            Some(15),
            Some(7),
            multiplicity * (eq120_value),
            &eq120_node_derivatives,
            &eq120_branch_derivatives,
            multiplicity,
        );
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22, eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq121_e1548: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, s.v[228]);
        let eq121_e1549: f64 = (p.p7 * eq121_e1548);
        (eq121_e1549, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_value: f64 = eq121_e1551;
        let eq121_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_branch_derivatives: [f64; 55] = [eq121_e1551_d_b0, eq121_e1551_d_b1, eq121_e1551_d_b2, eq121_e1551_d_b3, eq121_e1551_d_b4, eq121_e1551_d_b5, eq121_e1551_d_b6, eq121_e1551_d_b7, eq121_e1551_d_b8, eq121_e1551_d_b9, eq121_e1551_d_b10, eq121_e1551_d_b11, eq121_e1551_d_b12, eq121_e1551_d_b13, eq121_e1551_d_b14, eq121_e1551_d_b15, eq121_e1551_d_b16, eq121_e1551_d_b17, eq121_e1551_d_b18, eq121_e1551_d_b19, eq121_e1551_d_b20, eq121_e1551_d_b21, eq121_e1551_d_b22, eq121_e1551_d_b23, eq121_e1551_d_b24, eq121_e1551_d_b25, eq121_e1551_d_b26, eq121_e1551_d_b27, eq121_e1551_d_b28, eq121_e1551_d_b29, eq121_e1551_d_b30, eq121_e1551_d_b31, eq121_e1551_d_b32, eq121_e1551_d_b33, eq121_e1551_d_b34, eq121_e1551_d_b35, eq121_e1551_d_b36, eq121_e1551_d_b37, eq121_e1551_d_b38, eq121_e1551_d_b39, eq121_e1551_d_b40, eq121_e1551_d_b41, eq121_e1551_d_b42, eq121_e1551_d_b43, eq121_e1551_d_b44, eq121_e1551_d_b45, eq121_e1551_d_b46, eq121_e1551_d_b47, eq121_e1551_d_b48, eq121_e1551_d_b49, eq121_e1551_d_b50, eq121_e1551_d_b51, eq121_e1551_d_b52, eq121_e1551_d_b53, eq121_e1551_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq121_value),
            &eq121_node_derivatives,
            &eq121_branch_derivatives,
            multiplicity,
        );
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22, eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54,) = {
    if ((s.b[570] && s.b[571]) && s.b[572]) {
        let eq122_e1559: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, s.v[228]);
        let eq122_e1560: f64 = (p.p7 * eq122_e1559);
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
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n10, eq122_e1562_d_n11, eq122_e1562_d_n12, eq122_e1562_d_n13, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22, eq122_e1562_d_b0, eq122_e1562_d_b1, eq122_e1562_d_b2, eq122_e1562_d_b3, eq122_e1562_d_b4, eq122_e1562_d_b5, eq122_e1562_d_b6, eq122_e1562_d_b7, eq122_e1562_d_b8, eq122_e1562_d_b9, eq122_e1562_d_b10, eq122_e1562_d_b11, eq122_e1562_d_b12, eq122_e1562_d_b13, eq122_e1562_d_b14, eq122_e1562_d_b15, eq122_e1562_d_b16, eq122_e1562_d_b17, eq122_e1562_d_b18, eq122_e1562_d_b19, eq122_e1562_d_b20, eq122_e1562_d_b21, eq122_e1562_d_b22, eq122_e1562_d_b23, eq122_e1562_d_b24, eq122_e1562_d_b25, eq122_e1562_d_b26, eq122_e1562_d_b27, eq122_e1562_d_b28, eq122_e1562_d_b29, eq122_e1562_d_b30, eq122_e1562_d_b31, eq122_e1562_d_b32, eq122_e1562_d_b33, eq122_e1562_d_b34, eq122_e1562_d_b35, eq122_e1562_d_b36, eq122_e1562_d_b37, eq122_e1562_d_b38, eq122_e1562_d_b39, eq122_e1562_d_b40, eq122_e1562_d_b41, eq122_e1562_d_b42, eq122_e1562_d_b43, eq122_e1562_d_b44, eq122_e1562_d_b45, eq122_e1562_d_b46, eq122_e1562_d_b47, eq122_e1562_d_b48, eq122_e1562_d_b49, eq122_e1562_d_b50, eq122_e1562_d_b51, eq122_e1562_d_b52, eq122_e1562_d_b53, eq122_e1562_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1564;
        let eq122_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_branch_derivatives: [f64; 55] = [eq122_e1564_d_b0, eq122_e1564_d_b1, eq122_e1564_d_b2, eq122_e1564_d_b3, eq122_e1564_d_b4, eq122_e1564_d_b5, eq122_e1564_d_b6, eq122_e1564_d_b7, eq122_e1564_d_b8, eq122_e1564_d_b9, eq122_e1564_d_b10, eq122_e1564_d_b11, eq122_e1564_d_b12, eq122_e1564_d_b13, eq122_e1564_d_b14, eq122_e1564_d_b15, eq122_e1564_d_b16, eq122_e1564_d_b17, eq122_e1564_d_b18, eq122_e1564_d_b19, eq122_e1564_d_b20, eq122_e1564_d_b21, eq122_e1564_d_b22, eq122_e1564_d_b23, eq122_e1564_d_b24, eq122_e1564_d_b25, eq122_e1564_d_b26, eq122_e1564_d_b27, eq122_e1564_d_b28, eq122_e1564_d_b29, eq122_e1564_d_b30, eq122_e1564_d_b31, eq122_e1564_d_b32, eq122_e1564_d_b33, eq122_e1564_d_b34, eq122_e1564_d_b35, eq122_e1564_d_b36, eq122_e1564_d_b37, eq122_e1564_d_b38, eq122_e1564_d_b39, eq122_e1564_d_b40, eq122_e1564_d_b41, eq122_e1564_d_b42, eq122_e1564_d_b43, eq122_e1564_d_b44, eq122_e1564_d_b45, eq122_e1564_d_b46, eq122_e1564_d_b47, eq122_e1564_d_b48, eq122_e1564_d_b49, eq122_e1564_d_b50, eq122_e1564_d_b51, eq122_e1564_d_b52, eq122_e1564_d_b53, eq122_e1564_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq122_value),
            &eq122_node_derivatives,
            &eq122_branch_derivatives,
            multiplicity,
        );
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22, eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq123_e1573: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, s.v[228]);
        let eq123_e1574: f64 = (p.p7 * eq123_e1573);
        (eq123_e1574, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_value: f64 = eq123_e1576;
        let eq123_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_branch_derivatives: [f64; 55] = [eq123_e1576_d_b0, eq123_e1576_d_b1, eq123_e1576_d_b2, eq123_e1576_d_b3, eq123_e1576_d_b4, eq123_e1576_d_b5, eq123_e1576_d_b6, eq123_e1576_d_b7, eq123_e1576_d_b8, eq123_e1576_d_b9, eq123_e1576_d_b10, eq123_e1576_d_b11, eq123_e1576_d_b12, eq123_e1576_d_b13, eq123_e1576_d_b14, eq123_e1576_d_b15, eq123_e1576_d_b16, eq123_e1576_d_b17, eq123_e1576_d_b18, eq123_e1576_d_b19, eq123_e1576_d_b20, eq123_e1576_d_b21, eq123_e1576_d_b22, eq123_e1576_d_b23, eq123_e1576_d_b24, eq123_e1576_d_b25, eq123_e1576_d_b26, eq123_e1576_d_b27, eq123_e1576_d_b28, eq123_e1576_d_b29, eq123_e1576_d_b30, eq123_e1576_d_b31, eq123_e1576_d_b32, eq123_e1576_d_b33, eq123_e1576_d_b34, eq123_e1576_d_b35, eq123_e1576_d_b36, eq123_e1576_d_b37, eq123_e1576_d_b38, eq123_e1576_d_b39, eq123_e1576_d_b40, eq123_e1576_d_b41, eq123_e1576_d_b42, eq123_e1576_d_b43, eq123_e1576_d_b44, eq123_e1576_d_b45, eq123_e1576_d_b46, eq123_e1576_d_b47, eq123_e1576_d_b48, eq123_e1576_d_b49, eq123_e1576_d_b50, eq123_e1576_d_b51, eq123_e1576_d_b52, eq123_e1576_d_b53, eq123_e1576_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq123_value),
            &eq123_node_derivatives,
            &eq123_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_16(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22, eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54,) = {
    if ((s.b[570] && s.b[571]) && (!s.b[572])) {
        let eq124_e1585: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, s.v[228]);
        let eq124_e1586: f64 = (p.p7 * eq124_e1585);
        let eq124_e1586_d_n0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let eq124_e1586_d_n1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let eq124_e1586_d_n2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let eq124_e1586_d_n3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let eq124_e1586_d_n4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let eq124_e1586_d_n5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let eq124_e1586_d_n6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let eq124_e1586_d_n7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let eq124_e1586_d_n8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let eq124_e1586_d_n9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let eq124_e1586_d_n10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let eq124_e1586_d_n11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let eq124_e1586_d_n12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let eq124_e1586_d_n13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let eq124_e1586_d_n14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let eq124_e1586_d_n15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let eq124_e1586_d_n16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let eq124_e1586_d_n17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let eq124_e1586_d_n18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let eq124_e1586_d_n19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let eq124_e1586_d_n20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let eq124_e1586_d_n21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let eq124_e1586_d_n22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let eq124_e1586_d_b0: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let eq124_e1586_d_b1: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let eq124_e1586_d_b2: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let eq124_e1586_d_b3: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let eq124_e1586_d_b4: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let eq124_e1586_d_b5: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let eq124_e1586_d_b6: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let eq124_e1586_d_b7: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let eq124_e1586_d_b8: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let eq124_e1586_d_b9: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let eq124_e1586_d_b10: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let eq124_e1586_d_b11: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let eq124_e1586_d_b12: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let eq124_e1586_d_b13: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let eq124_e1586_d_b14: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let eq124_e1586_d_b15: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let eq124_e1586_d_b16: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let eq124_e1586_d_b17: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let eq124_e1586_d_b18: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let eq124_e1586_d_b19: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let eq124_e1586_d_b20: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let eq124_e1586_d_b21: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let eq124_e1586_d_b22: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let eq124_e1586_d_b23: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let eq124_e1586_d_b24: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let eq124_e1586_d_b25: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let eq124_e1586_d_b26: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let eq124_e1586_d_b27: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let eq124_e1586_d_b28: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let eq124_e1586_d_b29: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let eq124_e1586_d_b30: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let eq124_e1586_d_b31: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let eq124_e1586_d_b32: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let eq124_e1586_d_b33: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let eq124_e1586_d_b34: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let eq124_e1586_d_b35: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let eq124_e1586_d_b36: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let eq124_e1586_d_b37: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let eq124_e1586_d_b38: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let eq124_e1586_d_b39: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let eq124_e1586_d_b40: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let eq124_e1586_d_b41: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let eq124_e1586_d_b42: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let eq124_e1586_d_b43: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let eq124_e1586_d_b44: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let eq124_e1586_d_b45: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let eq124_e1586_d_b46: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let eq124_e1586_d_b47: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let eq124_e1586_d_b48: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let eq124_e1586_d_b49: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let eq124_e1586_d_b50: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let eq124_e1586_d_b51: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let eq124_e1586_d_b52: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let eq124_e1586_d_b53: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let eq124_e1586_d_b54: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
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
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22, eq124_e1588_d_b0, eq124_e1588_d_b1, eq124_e1588_d_b2, eq124_e1588_d_b3, eq124_e1588_d_b4, eq124_e1588_d_b5, eq124_e1588_d_b6, eq124_e1588_d_b7, eq124_e1588_d_b8, eq124_e1588_d_b9, eq124_e1588_d_b10, eq124_e1588_d_b11, eq124_e1588_d_b12, eq124_e1588_d_b13, eq124_e1588_d_b14, eq124_e1588_d_b15, eq124_e1588_d_b16, eq124_e1588_d_b17, eq124_e1588_d_b18, eq124_e1588_d_b19, eq124_e1588_d_b20, eq124_e1588_d_b21, eq124_e1588_d_b22, eq124_e1588_d_b23, eq124_e1588_d_b24, eq124_e1588_d_b25, eq124_e1588_d_b26, eq124_e1588_d_b27, eq124_e1588_d_b28, eq124_e1588_d_b29, eq124_e1588_d_b30, eq124_e1588_d_b31, eq124_e1588_d_b32, eq124_e1588_d_b33, eq124_e1588_d_b34, eq124_e1588_d_b35, eq124_e1588_d_b36, eq124_e1588_d_b37, eq124_e1588_d_b38, eq124_e1588_d_b39, eq124_e1588_d_b40, eq124_e1588_d_b41, eq124_e1588_d_b42, eq124_e1588_d_b43, eq124_e1588_d_b44, eq124_e1588_d_b45, eq124_e1588_d_b46, eq124_e1588_d_b47, eq124_e1588_d_b48, eq124_e1588_d_b49, eq124_e1588_d_b50, eq124_e1588_d_b51, eq124_e1588_d_b52, eq124_e1588_d_b53, eq124_e1588_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1590;
        let eq124_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_branch_derivatives: [f64; 55] = [eq124_e1590_d_b0, eq124_e1590_d_b1, eq124_e1590_d_b2, eq124_e1590_d_b3, eq124_e1590_d_b4, eq124_e1590_d_b5, eq124_e1590_d_b6, eq124_e1590_d_b7, eq124_e1590_d_b8, eq124_e1590_d_b9, eq124_e1590_d_b10, eq124_e1590_d_b11, eq124_e1590_d_b12, eq124_e1590_d_b13, eq124_e1590_d_b14, eq124_e1590_d_b15, eq124_e1590_d_b16, eq124_e1590_d_b17, eq124_e1590_d_b18, eq124_e1590_d_b19, eq124_e1590_d_b20, eq124_e1590_d_b21, eq124_e1590_d_b22, eq124_e1590_d_b23, eq124_e1590_d_b24, eq124_e1590_d_b25, eq124_e1590_d_b26, eq124_e1590_d_b27, eq124_e1590_d_b28, eq124_e1590_d_b29, eq124_e1590_d_b30, eq124_e1590_d_b31, eq124_e1590_d_b32, eq124_e1590_d_b33, eq124_e1590_d_b34, eq124_e1590_d_b35, eq124_e1590_d_b36, eq124_e1590_d_b37, eq124_e1590_d_b38, eq124_e1590_d_b39, eq124_e1590_d_b40, eq124_e1590_d_b41, eq124_e1590_d_b42, eq124_e1590_d_b43, eq124_e1590_d_b44, eq124_e1590_d_b45, eq124_e1590_d_b46, eq124_e1590_d_b47, eq124_e1590_d_b48, eq124_e1590_d_b49, eq124_e1590_d_b50, eq124_e1590_d_b51, eq124_e1590_d_b52, eq124_e1590_d_b53, eq124_e1590_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq124_value),
            &eq124_node_derivatives,
            &eq124_branch_derivatives,
            multiplicity,
        );
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22, eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54,) = {
    if (s.b[570] && s.b[571]) {
        let eq125_e1597: f64 = (p.p251 * s.v[228]);
        let eq125_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, eq125_e1597);
        let eq125_e1598_d_n0: f64 = ((p.p251 * s.dn[228][0]) * ddt_scale);
        let eq125_e1598_d_n1: f64 = ((p.p251 * s.dn[228][1]) * ddt_scale);
        let eq125_e1598_d_n2: f64 = ((p.p251 * s.dn[228][2]) * ddt_scale);
        let eq125_e1598_d_n3: f64 = ((p.p251 * s.dn[228][3]) * ddt_scale);
        let eq125_e1598_d_n4: f64 = ((p.p251 * s.dn[228][4]) * ddt_scale);
        let eq125_e1598_d_n5: f64 = ((p.p251 * s.dn[228][5]) * ddt_scale);
        let eq125_e1598_d_n6: f64 = ((p.p251 * s.dn[228][6]) * ddt_scale);
        let eq125_e1598_d_n7: f64 = ((p.p251 * s.dn[228][7]) * ddt_scale);
        let eq125_e1598_d_n8: f64 = ((p.p251 * s.dn[228][8]) * ddt_scale);
        let eq125_e1598_d_n9: f64 = ((p.p251 * s.dn[228][9]) * ddt_scale);
        let eq125_e1598_d_n10: f64 = ((p.p251 * s.dn[228][10]) * ddt_scale);
        let eq125_e1598_d_n11: f64 = ((p.p251 * s.dn[228][11]) * ddt_scale);
        let eq125_e1598_d_n12: f64 = ((p.p251 * s.dn[228][12]) * ddt_scale);
        let eq125_e1598_d_n13: f64 = ((p.p251 * s.dn[228][13]) * ddt_scale);
        let eq125_e1598_d_n14: f64 = ((p.p251 * s.dn[228][14]) * ddt_scale);
        let eq125_e1598_d_n15: f64 = ((p.p251 * s.dn[228][15]) * ddt_scale);
        let eq125_e1598_d_n16: f64 = ((p.p251 * s.dn[228][16]) * ddt_scale);
        let eq125_e1598_d_n17: f64 = ((p.p251 * s.dn[228][17]) * ddt_scale);
        let eq125_e1598_d_n18: f64 = ((p.p251 * s.dn[228][18]) * ddt_scale);
        let eq125_e1598_d_n19: f64 = ((p.p251 * s.dn[228][19]) * ddt_scale);
        let eq125_e1598_d_n20: f64 = ((p.p251 * s.dn[228][20]) * ddt_scale);
        let eq125_e1598_d_n21: f64 = ((p.p251 * s.dn[228][21]) * ddt_scale);
        let eq125_e1598_d_n22: f64 = ((p.p251 * s.dn[228][22]) * ddt_scale);
        let eq125_e1598_d_b0: f64 = ((p.p251 * s.db[228][0]) * ddt_scale);
        let eq125_e1598_d_b1: f64 = ((p.p251 * s.db[228][1]) * ddt_scale);
        let eq125_e1598_d_b2: f64 = ((p.p251 * s.db[228][2]) * ddt_scale);
        let eq125_e1598_d_b3: f64 = ((p.p251 * s.db[228][3]) * ddt_scale);
        let eq125_e1598_d_b4: f64 = ((p.p251 * s.db[228][4]) * ddt_scale);
        let eq125_e1598_d_b5: f64 = ((p.p251 * s.db[228][5]) * ddt_scale);
        let eq125_e1598_d_b6: f64 = ((p.p251 * s.db[228][6]) * ddt_scale);
        let eq125_e1598_d_b7: f64 = ((p.p251 * s.db[228][7]) * ddt_scale);
        let eq125_e1598_d_b8: f64 = ((p.p251 * s.db[228][8]) * ddt_scale);
        let eq125_e1598_d_b9: f64 = ((p.p251 * s.db[228][9]) * ddt_scale);
        let eq125_e1598_d_b10: f64 = ((p.p251 * s.db[228][10]) * ddt_scale);
        let eq125_e1598_d_b11: f64 = ((p.p251 * s.db[228][11]) * ddt_scale);
        let eq125_e1598_d_b12: f64 = ((p.p251 * s.db[228][12]) * ddt_scale);
        let eq125_e1598_d_b13: f64 = ((p.p251 * s.db[228][13]) * ddt_scale);
        let eq125_e1598_d_b14: f64 = ((p.p251 * s.db[228][14]) * ddt_scale);
        let eq125_e1598_d_b15: f64 = ((p.p251 * s.db[228][15]) * ddt_scale);
        let eq125_e1598_d_b16: f64 = ((p.p251 * s.db[228][16]) * ddt_scale);
        let eq125_e1598_d_b17: f64 = ((p.p251 * s.db[228][17]) * ddt_scale);
        let eq125_e1598_d_b18: f64 = ((p.p251 * s.db[228][18]) * ddt_scale);
        let eq125_e1598_d_b19: f64 = ((p.p251 * s.db[228][19]) * ddt_scale);
        let eq125_e1598_d_b20: f64 = ((p.p251 * s.db[228][20]) * ddt_scale);
        let eq125_e1598_d_b21: f64 = ((p.p251 * s.db[228][21]) * ddt_scale);
        let eq125_e1598_d_b22: f64 = ((p.p251 * s.db[228][22]) * ddt_scale);
        let eq125_e1598_d_b23: f64 = ((p.p251 * s.db[228][23]) * ddt_scale);
        let eq125_e1598_d_b24: f64 = ((p.p251 * s.db[228][24]) * ddt_scale);
        let eq125_e1598_d_b25: f64 = ((p.p251 * s.db[228][25]) * ddt_scale);
        let eq125_e1598_d_b26: f64 = ((p.p251 * s.db[228][26]) * ddt_scale);
        let eq125_e1598_d_b27: f64 = ((p.p251 * s.db[228][27]) * ddt_scale);
        let eq125_e1598_d_b28: f64 = ((p.p251 * s.db[228][28]) * ddt_scale);
        let eq125_e1598_d_b29: f64 = ((p.p251 * s.db[228][29]) * ddt_scale);
        let eq125_e1598_d_b30: f64 = ((p.p251 * s.db[228][30]) * ddt_scale);
        let eq125_e1598_d_b31: f64 = ((p.p251 * s.db[228][31]) * ddt_scale);
        let eq125_e1598_d_b32: f64 = ((p.p251 * s.db[228][32]) * ddt_scale);
        let eq125_e1598_d_b33: f64 = ((p.p251 * s.db[228][33]) * ddt_scale);
        let eq125_e1598_d_b34: f64 = ((p.p251 * s.db[228][34]) * ddt_scale);
        let eq125_e1598_d_b35: f64 = ((p.p251 * s.db[228][35]) * ddt_scale);
        let eq125_e1598_d_b36: f64 = ((p.p251 * s.db[228][36]) * ddt_scale);
        let eq125_e1598_d_b37: f64 = ((p.p251 * s.db[228][37]) * ddt_scale);
        let eq125_e1598_d_b38: f64 = ((p.p251 * s.db[228][38]) * ddt_scale);
        let eq125_e1598_d_b39: f64 = ((p.p251 * s.db[228][39]) * ddt_scale);
        let eq125_e1598_d_b40: f64 = ((p.p251 * s.db[228][40]) * ddt_scale);
        let eq125_e1598_d_b41: f64 = ((p.p251 * s.db[228][41]) * ddt_scale);
        let eq125_e1598_d_b42: f64 = ((p.p251 * s.db[228][42]) * ddt_scale);
        let eq125_e1598_d_b43: f64 = ((p.p251 * s.db[228][43]) * ddt_scale);
        let eq125_e1598_d_b44: f64 = ((p.p251 * s.db[228][44]) * ddt_scale);
        let eq125_e1598_d_b45: f64 = ((p.p251 * s.db[228][45]) * ddt_scale);
        let eq125_e1598_d_b46: f64 = ((p.p251 * s.db[228][46]) * ddt_scale);
        let eq125_e1598_d_b47: f64 = ((p.p251 * s.db[228][47]) * ddt_scale);
        let eq125_e1598_d_b48: f64 = ((p.p251 * s.db[228][48]) * ddt_scale);
        let eq125_e1598_d_b49: f64 = ((p.p251 * s.db[228][49]) * ddt_scale);
        let eq125_e1598_d_b50: f64 = ((p.p251 * s.db[228][50]) * ddt_scale);
        let eq125_e1598_d_b51: f64 = ((p.p251 * s.db[228][51]) * ddt_scale);
        let eq125_e1598_d_b52: f64 = ((p.p251 * s.db[228][52]) * ddt_scale);
        let eq125_e1598_d_b53: f64 = ((p.p251 * s.db[228][53]) * ddt_scale);
        let eq125_e1598_d_b54: f64 = ((p.p251 * s.db[228][54]) * ddt_scale);
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
        let eq125_e1599_d_b0: f64 = (p.p7 * eq125_e1598_d_b0);
        let eq125_e1599_d_b1: f64 = (p.p7 * eq125_e1598_d_b1);
        let eq125_e1599_d_b2: f64 = (p.p7 * eq125_e1598_d_b2);
        let eq125_e1599_d_b3: f64 = (p.p7 * eq125_e1598_d_b3);
        let eq125_e1599_d_b4: f64 = (p.p7 * eq125_e1598_d_b4);
        let eq125_e1599_d_b5: f64 = (p.p7 * eq125_e1598_d_b5);
        let eq125_e1599_d_b6: f64 = (p.p7 * eq125_e1598_d_b6);
        let eq125_e1599_d_b7: f64 = (p.p7 * eq125_e1598_d_b7);
        let eq125_e1599_d_b8: f64 = (p.p7 * eq125_e1598_d_b8);
        let eq125_e1599_d_b9: f64 = (p.p7 * eq125_e1598_d_b9);
        let eq125_e1599_d_b10: f64 = (p.p7 * eq125_e1598_d_b10);
        let eq125_e1599_d_b11: f64 = (p.p7 * eq125_e1598_d_b11);
        let eq125_e1599_d_b12: f64 = (p.p7 * eq125_e1598_d_b12);
        let eq125_e1599_d_b13: f64 = (p.p7 * eq125_e1598_d_b13);
        let eq125_e1599_d_b14: f64 = (p.p7 * eq125_e1598_d_b14);
        let eq125_e1599_d_b15: f64 = (p.p7 * eq125_e1598_d_b15);
        let eq125_e1599_d_b16: f64 = (p.p7 * eq125_e1598_d_b16);
        let eq125_e1599_d_b17: f64 = (p.p7 * eq125_e1598_d_b17);
        let eq125_e1599_d_b18: f64 = (p.p7 * eq125_e1598_d_b18);
        let eq125_e1599_d_b19: f64 = (p.p7 * eq125_e1598_d_b19);
        let eq125_e1599_d_b20: f64 = (p.p7 * eq125_e1598_d_b20);
        let eq125_e1599_d_b21: f64 = (p.p7 * eq125_e1598_d_b21);
        let eq125_e1599_d_b22: f64 = (p.p7 * eq125_e1598_d_b22);
        let eq125_e1599_d_b23: f64 = (p.p7 * eq125_e1598_d_b23);
        let eq125_e1599_d_b24: f64 = (p.p7 * eq125_e1598_d_b24);
        let eq125_e1599_d_b25: f64 = (p.p7 * eq125_e1598_d_b25);
        let eq125_e1599_d_b26: f64 = (p.p7 * eq125_e1598_d_b26);
        let eq125_e1599_d_b27: f64 = (p.p7 * eq125_e1598_d_b27);
        let eq125_e1599_d_b28: f64 = (p.p7 * eq125_e1598_d_b28);
        let eq125_e1599_d_b29: f64 = (p.p7 * eq125_e1598_d_b29);
        let eq125_e1599_d_b30: f64 = (p.p7 * eq125_e1598_d_b30);
        let eq125_e1599_d_b31: f64 = (p.p7 * eq125_e1598_d_b31);
        let eq125_e1599_d_b32: f64 = (p.p7 * eq125_e1598_d_b32);
        let eq125_e1599_d_b33: f64 = (p.p7 * eq125_e1598_d_b33);
        let eq125_e1599_d_b34: f64 = (p.p7 * eq125_e1598_d_b34);
        let eq125_e1599_d_b35: f64 = (p.p7 * eq125_e1598_d_b35);
        let eq125_e1599_d_b36: f64 = (p.p7 * eq125_e1598_d_b36);
        let eq125_e1599_d_b37: f64 = (p.p7 * eq125_e1598_d_b37);
        let eq125_e1599_d_b38: f64 = (p.p7 * eq125_e1598_d_b38);
        let eq125_e1599_d_b39: f64 = (p.p7 * eq125_e1598_d_b39);
        let eq125_e1599_d_b40: f64 = (p.p7 * eq125_e1598_d_b40);
        let eq125_e1599_d_b41: f64 = (p.p7 * eq125_e1598_d_b41);
        let eq125_e1599_d_b42: f64 = (p.p7 * eq125_e1598_d_b42);
        let eq125_e1599_d_b43: f64 = (p.p7 * eq125_e1598_d_b43);
        let eq125_e1599_d_b44: f64 = (p.p7 * eq125_e1598_d_b44);
        let eq125_e1599_d_b45: f64 = (p.p7 * eq125_e1598_d_b45);
        let eq125_e1599_d_b46: f64 = (p.p7 * eq125_e1598_d_b46);
        let eq125_e1599_d_b47: f64 = (p.p7 * eq125_e1598_d_b47);
        let eq125_e1599_d_b48: f64 = (p.p7 * eq125_e1598_d_b48);
        let eq125_e1599_d_b49: f64 = (p.p7 * eq125_e1598_d_b49);
        let eq125_e1599_d_b50: f64 = (p.p7 * eq125_e1598_d_b50);
        let eq125_e1599_d_b51: f64 = (p.p7 * eq125_e1598_d_b51);
        let eq125_e1599_d_b52: f64 = (p.p7 * eq125_e1598_d_b52);
        let eq125_e1599_d_b53: f64 = (p.p7 * eq125_e1598_d_b53);
        let eq125_e1599_d_b54: f64 = (p.p7 * eq125_e1598_d_b54);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22, eq125_e1599_d_b0, eq125_e1599_d_b1, eq125_e1599_d_b2, eq125_e1599_d_b3, eq125_e1599_d_b4, eq125_e1599_d_b5, eq125_e1599_d_b6, eq125_e1599_d_b7, eq125_e1599_d_b8, eq125_e1599_d_b9, eq125_e1599_d_b10, eq125_e1599_d_b11, eq125_e1599_d_b12, eq125_e1599_d_b13, eq125_e1599_d_b14, eq125_e1599_d_b15, eq125_e1599_d_b16, eq125_e1599_d_b17, eq125_e1599_d_b18, eq125_e1599_d_b19, eq125_e1599_d_b20, eq125_e1599_d_b21, eq125_e1599_d_b22, eq125_e1599_d_b23, eq125_e1599_d_b24, eq125_e1599_d_b25, eq125_e1599_d_b26, eq125_e1599_d_b27, eq125_e1599_d_b28, eq125_e1599_d_b29, eq125_e1599_d_b30, eq125_e1599_d_b31, eq125_e1599_d_b32, eq125_e1599_d_b33, eq125_e1599_d_b34, eq125_e1599_d_b35, eq125_e1599_d_b36, eq125_e1599_d_b37, eq125_e1599_d_b38, eq125_e1599_d_b39, eq125_e1599_d_b40, eq125_e1599_d_b41, eq125_e1599_d_b42, eq125_e1599_d_b43, eq125_e1599_d_b44, eq125_e1599_d_b45, eq125_e1599_d_b46, eq125_e1599_d_b47, eq125_e1599_d_b48, eq125_e1599_d_b49, eq125_e1599_d_b50, eq125_e1599_d_b51, eq125_e1599_d_b52, eq125_e1599_d_b53, eq125_e1599_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1601;
        let eq125_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_branch_derivatives: [f64; 55] = [eq125_e1601_d_b0, eq125_e1601_d_b1, eq125_e1601_d_b2, eq125_e1601_d_b3, eq125_e1601_d_b4, eq125_e1601_d_b5, eq125_e1601_d_b6, eq125_e1601_d_b7, eq125_e1601_d_b8, eq125_e1601_d_b9, eq125_e1601_d_b10, eq125_e1601_d_b11, eq125_e1601_d_b12, eq125_e1601_d_b13, eq125_e1601_d_b14, eq125_e1601_d_b15, eq125_e1601_d_b16, eq125_e1601_d_b17, eq125_e1601_d_b18, eq125_e1601_d_b19, eq125_e1601_d_b20, eq125_e1601_d_b21, eq125_e1601_d_b22, eq125_e1601_d_b23, eq125_e1601_d_b24, eq125_e1601_d_b25, eq125_e1601_d_b26, eq125_e1601_d_b27, eq125_e1601_d_b28, eq125_e1601_d_b29, eq125_e1601_d_b30, eq125_e1601_d_b31, eq125_e1601_d_b32, eq125_e1601_d_b33, eq125_e1601_d_b34, eq125_e1601_d_b35, eq125_e1601_d_b36, eq125_e1601_d_b37, eq125_e1601_d_b38, eq125_e1601_d_b39, eq125_e1601_d_b40, eq125_e1601_d_b41, eq125_e1601_d_b42, eq125_e1601_d_b43, eq125_e1601_d_b44, eq125_e1601_d_b45, eq125_e1601_d_b46, eq125_e1601_d_b47, eq125_e1601_d_b48, eq125_e1601_d_b49, eq125_e1601_d_b50, eq125_e1601_d_b51, eq125_e1601_d_b52, eq125_e1601_d_b53, eq125_e1601_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq125_value),
            &eq125_node_derivatives,
            &eq125_branch_derivatives,
            multiplicity,
        );
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22, eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq126_e1608: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, s.v[229]);
        let eq126_e1609: f64 = (p.p7 * eq126_e1608);
        let eq126_e1609_d_n0: f64 = (p.p7 * (s.dn[229][0] * ddt_scale));
        let eq126_e1609_d_n1: f64 = (p.p7 * (s.dn[229][1] * ddt_scale));
        let eq126_e1609_d_n2: f64 = (p.p7 * (s.dn[229][2] * ddt_scale));
        let eq126_e1609_d_n3: f64 = (p.p7 * (s.dn[229][3] * ddt_scale));
        let eq126_e1609_d_n4: f64 = (p.p7 * (s.dn[229][4] * ddt_scale));
        let eq126_e1609_d_n5: f64 = (p.p7 * (s.dn[229][5] * ddt_scale));
        let eq126_e1609_d_n6: f64 = (p.p7 * (s.dn[229][6] * ddt_scale));
        let eq126_e1609_d_n7: f64 = (p.p7 * (s.dn[229][7] * ddt_scale));
        let eq126_e1609_d_n8: f64 = (p.p7 * (s.dn[229][8] * ddt_scale));
        let eq126_e1609_d_n9: f64 = (p.p7 * (s.dn[229][9] * ddt_scale));
        let eq126_e1609_d_n10: f64 = (p.p7 * (s.dn[229][10] * ddt_scale));
        let eq126_e1609_d_n11: f64 = (p.p7 * (s.dn[229][11] * ddt_scale));
        let eq126_e1609_d_n12: f64 = (p.p7 * (s.dn[229][12] * ddt_scale));
        let eq126_e1609_d_n13: f64 = (p.p7 * (s.dn[229][13] * ddt_scale));
        let eq126_e1609_d_n14: f64 = (p.p7 * (s.dn[229][14] * ddt_scale));
        let eq126_e1609_d_n15: f64 = (p.p7 * (s.dn[229][15] * ddt_scale));
        let eq126_e1609_d_n16: f64 = (p.p7 * (s.dn[229][16] * ddt_scale));
        let eq126_e1609_d_n17: f64 = (p.p7 * (s.dn[229][17] * ddt_scale));
        let eq126_e1609_d_n18: f64 = (p.p7 * (s.dn[229][18] * ddt_scale));
        let eq126_e1609_d_n19: f64 = (p.p7 * (s.dn[229][19] * ddt_scale));
        let eq126_e1609_d_n20: f64 = (p.p7 * (s.dn[229][20] * ddt_scale));
        let eq126_e1609_d_n21: f64 = (p.p7 * (s.dn[229][21] * ddt_scale));
        let eq126_e1609_d_n22: f64 = (p.p7 * (s.dn[229][22] * ddt_scale));
        let eq126_e1609_d_b0: f64 = (p.p7 * (s.db[229][0] * ddt_scale));
        let eq126_e1609_d_b1: f64 = (p.p7 * (s.db[229][1] * ddt_scale));
        let eq126_e1609_d_b2: f64 = (p.p7 * (s.db[229][2] * ddt_scale));
        let eq126_e1609_d_b3: f64 = (p.p7 * (s.db[229][3] * ddt_scale));
        let eq126_e1609_d_b4: f64 = (p.p7 * (s.db[229][4] * ddt_scale));
        let eq126_e1609_d_b5: f64 = (p.p7 * (s.db[229][5] * ddt_scale));
        let eq126_e1609_d_b6: f64 = (p.p7 * (s.db[229][6] * ddt_scale));
        let eq126_e1609_d_b7: f64 = (p.p7 * (s.db[229][7] * ddt_scale));
        let eq126_e1609_d_b8: f64 = (p.p7 * (s.db[229][8] * ddt_scale));
        let eq126_e1609_d_b9: f64 = (p.p7 * (s.db[229][9] * ddt_scale));
        let eq126_e1609_d_b10: f64 = (p.p7 * (s.db[229][10] * ddt_scale));
        let eq126_e1609_d_b11: f64 = (p.p7 * (s.db[229][11] * ddt_scale));
        let eq126_e1609_d_b12: f64 = (p.p7 * (s.db[229][12] * ddt_scale));
        let eq126_e1609_d_b13: f64 = (p.p7 * (s.db[229][13] * ddt_scale));
        let eq126_e1609_d_b14: f64 = (p.p7 * (s.db[229][14] * ddt_scale));
        let eq126_e1609_d_b15: f64 = (p.p7 * (s.db[229][15] * ddt_scale));
        let eq126_e1609_d_b16: f64 = (p.p7 * (s.db[229][16] * ddt_scale));
        let eq126_e1609_d_b17: f64 = (p.p7 * (s.db[229][17] * ddt_scale));
        let eq126_e1609_d_b18: f64 = (p.p7 * (s.db[229][18] * ddt_scale));
        let eq126_e1609_d_b19: f64 = (p.p7 * (s.db[229][19] * ddt_scale));
        let eq126_e1609_d_b20: f64 = (p.p7 * (s.db[229][20] * ddt_scale));
        let eq126_e1609_d_b21: f64 = (p.p7 * (s.db[229][21] * ddt_scale));
        let eq126_e1609_d_b22: f64 = (p.p7 * (s.db[229][22] * ddt_scale));
        let eq126_e1609_d_b23: f64 = (p.p7 * (s.db[229][23] * ddt_scale));
        let eq126_e1609_d_b24: f64 = (p.p7 * (s.db[229][24] * ddt_scale));
        let eq126_e1609_d_b25: f64 = (p.p7 * (s.db[229][25] * ddt_scale));
        let eq126_e1609_d_b26: f64 = (p.p7 * (s.db[229][26] * ddt_scale));
        let eq126_e1609_d_b27: f64 = (p.p7 * (s.db[229][27] * ddt_scale));
        let eq126_e1609_d_b28: f64 = (p.p7 * (s.db[229][28] * ddt_scale));
        let eq126_e1609_d_b29: f64 = (p.p7 * (s.db[229][29] * ddt_scale));
        let eq126_e1609_d_b30: f64 = (p.p7 * (s.db[229][30] * ddt_scale));
        let eq126_e1609_d_b31: f64 = (p.p7 * (s.db[229][31] * ddt_scale));
        let eq126_e1609_d_b32: f64 = (p.p7 * (s.db[229][32] * ddt_scale));
        let eq126_e1609_d_b33: f64 = (p.p7 * (s.db[229][33] * ddt_scale));
        let eq126_e1609_d_b34: f64 = (p.p7 * (s.db[229][34] * ddt_scale));
        let eq126_e1609_d_b35: f64 = (p.p7 * (s.db[229][35] * ddt_scale));
        let eq126_e1609_d_b36: f64 = (p.p7 * (s.db[229][36] * ddt_scale));
        let eq126_e1609_d_b37: f64 = (p.p7 * (s.db[229][37] * ddt_scale));
        let eq126_e1609_d_b38: f64 = (p.p7 * (s.db[229][38] * ddt_scale));
        let eq126_e1609_d_b39: f64 = (p.p7 * (s.db[229][39] * ddt_scale));
        let eq126_e1609_d_b40: f64 = (p.p7 * (s.db[229][40] * ddt_scale));
        let eq126_e1609_d_b41: f64 = (p.p7 * (s.db[229][41] * ddt_scale));
        let eq126_e1609_d_b42: f64 = (p.p7 * (s.db[229][42] * ddt_scale));
        let eq126_e1609_d_b43: f64 = (p.p7 * (s.db[229][43] * ddt_scale));
        let eq126_e1609_d_b44: f64 = (p.p7 * (s.db[229][44] * ddt_scale));
        let eq126_e1609_d_b45: f64 = (p.p7 * (s.db[229][45] * ddt_scale));
        let eq126_e1609_d_b46: f64 = (p.p7 * (s.db[229][46] * ddt_scale));
        let eq126_e1609_d_b47: f64 = (p.p7 * (s.db[229][47] * ddt_scale));
        let eq126_e1609_d_b48: f64 = (p.p7 * (s.db[229][48] * ddt_scale));
        let eq126_e1609_d_b49: f64 = (p.p7 * (s.db[229][49] * ddt_scale));
        let eq126_e1609_d_b50: f64 = (p.p7 * (s.db[229][50] * ddt_scale));
        let eq126_e1609_d_b51: f64 = (p.p7 * (s.db[229][51] * ddt_scale));
        let eq126_e1609_d_b52: f64 = (p.p7 * (s.db[229][52] * ddt_scale));
        let eq126_e1609_d_b53: f64 = (p.p7 * (s.db[229][53] * ddt_scale));
        let eq126_e1609_d_b54: f64 = (p.p7 * (s.db[229][54] * ddt_scale));
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22, eq126_e1609_d_b0, eq126_e1609_d_b1, eq126_e1609_d_b2, eq126_e1609_d_b3, eq126_e1609_d_b4, eq126_e1609_d_b5, eq126_e1609_d_b6, eq126_e1609_d_b7, eq126_e1609_d_b8, eq126_e1609_d_b9, eq126_e1609_d_b10, eq126_e1609_d_b11, eq126_e1609_d_b12, eq126_e1609_d_b13, eq126_e1609_d_b14, eq126_e1609_d_b15, eq126_e1609_d_b16, eq126_e1609_d_b17, eq126_e1609_d_b18, eq126_e1609_d_b19, eq126_e1609_d_b20, eq126_e1609_d_b21, eq126_e1609_d_b22, eq126_e1609_d_b23, eq126_e1609_d_b24, eq126_e1609_d_b25, eq126_e1609_d_b26, eq126_e1609_d_b27, eq126_e1609_d_b28, eq126_e1609_d_b29, eq126_e1609_d_b30, eq126_e1609_d_b31, eq126_e1609_d_b32, eq126_e1609_d_b33, eq126_e1609_d_b34, eq126_e1609_d_b35, eq126_e1609_d_b36, eq126_e1609_d_b37, eq126_e1609_d_b38, eq126_e1609_d_b39, eq126_e1609_d_b40, eq126_e1609_d_b41, eq126_e1609_d_b42, eq126_e1609_d_b43, eq126_e1609_d_b44, eq126_e1609_d_b45, eq126_e1609_d_b46, eq126_e1609_d_b47, eq126_e1609_d_b48, eq126_e1609_d_b49, eq126_e1609_d_b50, eq126_e1609_d_b51, eq126_e1609_d_b52, eq126_e1609_d_b53, eq126_e1609_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1611;
        let eq126_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_branch_derivatives: [f64; 55] = [eq126_e1611_d_b0, eq126_e1611_d_b1, eq126_e1611_d_b2, eq126_e1611_d_b3, eq126_e1611_d_b4, eq126_e1611_d_b5, eq126_e1611_d_b6, eq126_e1611_d_b7, eq126_e1611_d_b8, eq126_e1611_d_b9, eq126_e1611_d_b10, eq126_e1611_d_b11, eq126_e1611_d_b12, eq126_e1611_d_b13, eq126_e1611_d_b14, eq126_e1611_d_b15, eq126_e1611_d_b16, eq126_e1611_d_b17, eq126_e1611_d_b18, eq126_e1611_d_b19, eq126_e1611_d_b20, eq126_e1611_d_b21, eq126_e1611_d_b22, eq126_e1611_d_b23, eq126_e1611_d_b24, eq126_e1611_d_b25, eq126_e1611_d_b26, eq126_e1611_d_b27, eq126_e1611_d_b28, eq126_e1611_d_b29, eq126_e1611_d_b30, eq126_e1611_d_b31, eq126_e1611_d_b32, eq126_e1611_d_b33, eq126_e1611_d_b34, eq126_e1611_d_b35, eq126_e1611_d_b36, eq126_e1611_d_b37, eq126_e1611_d_b38, eq126_e1611_d_b39, eq126_e1611_d_b40, eq126_e1611_d_b41, eq126_e1611_d_b42, eq126_e1611_d_b43, eq126_e1611_d_b44, eq126_e1611_d_b45, eq126_e1611_d_b46, eq126_e1611_d_b47, eq126_e1611_d_b48, eq126_e1611_d_b49, eq126_e1611_d_b50, eq126_e1611_d_b51, eq126_e1611_d_b52, eq126_e1611_d_b53, eq126_e1611_d_b54];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq126_value),
            &eq126_node_derivatives,
            &eq126_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_17(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22, eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq127_e1620: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, s.v[228]);
        let eq127_e1621: f64 = (p.p7 * eq127_e1620);
        (eq127_e1621, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_value: f64 = eq127_e1623;
        let eq127_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_branch_derivatives: [f64; 55] = [eq127_e1623_d_b0, eq127_e1623_d_b1, eq127_e1623_d_b2, eq127_e1623_d_b3, eq127_e1623_d_b4, eq127_e1623_d_b5, eq127_e1623_d_b6, eq127_e1623_d_b7, eq127_e1623_d_b8, eq127_e1623_d_b9, eq127_e1623_d_b10, eq127_e1623_d_b11, eq127_e1623_d_b12, eq127_e1623_d_b13, eq127_e1623_d_b14, eq127_e1623_d_b15, eq127_e1623_d_b16, eq127_e1623_d_b17, eq127_e1623_d_b18, eq127_e1623_d_b19, eq127_e1623_d_b20, eq127_e1623_d_b21, eq127_e1623_d_b22, eq127_e1623_d_b23, eq127_e1623_d_b24, eq127_e1623_d_b25, eq127_e1623_d_b26, eq127_e1623_d_b27, eq127_e1623_d_b28, eq127_e1623_d_b29, eq127_e1623_d_b30, eq127_e1623_d_b31, eq127_e1623_d_b32, eq127_e1623_d_b33, eq127_e1623_d_b34, eq127_e1623_d_b35, eq127_e1623_d_b36, eq127_e1623_d_b37, eq127_e1623_d_b38, eq127_e1623_d_b39, eq127_e1623_d_b40, eq127_e1623_d_b41, eq127_e1623_d_b42, eq127_e1623_d_b43, eq127_e1623_d_b44, eq127_e1623_d_b45, eq127_e1623_d_b46, eq127_e1623_d_b47, eq127_e1623_d_b48, eq127_e1623_d_b49, eq127_e1623_d_b50, eq127_e1623_d_b51, eq127_e1623_d_b52, eq127_e1623_d_b53, eq127_e1623_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq127_value),
            &eq127_node_derivatives,
            &eq127_branch_derivatives,
            multiplicity,
        );
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22, eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && s.b[574]) {
        let eq128_e1632: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, s.v[228]);
        let eq128_e1633: f64 = (p.p7 * eq128_e1632);
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
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22, eq128_e1635_d_b0, eq128_e1635_d_b1, eq128_e1635_d_b2, eq128_e1635_d_b3, eq128_e1635_d_b4, eq128_e1635_d_b5, eq128_e1635_d_b6, eq128_e1635_d_b7, eq128_e1635_d_b8, eq128_e1635_d_b9, eq128_e1635_d_b10, eq128_e1635_d_b11, eq128_e1635_d_b12, eq128_e1635_d_b13, eq128_e1635_d_b14, eq128_e1635_d_b15, eq128_e1635_d_b16, eq128_e1635_d_b17, eq128_e1635_d_b18, eq128_e1635_d_b19, eq128_e1635_d_b20, eq128_e1635_d_b21, eq128_e1635_d_b22, eq128_e1635_d_b23, eq128_e1635_d_b24, eq128_e1635_d_b25, eq128_e1635_d_b26, eq128_e1635_d_b27, eq128_e1635_d_b28, eq128_e1635_d_b29, eq128_e1635_d_b30, eq128_e1635_d_b31, eq128_e1635_d_b32, eq128_e1635_d_b33, eq128_e1635_d_b34, eq128_e1635_d_b35, eq128_e1635_d_b36, eq128_e1635_d_b37, eq128_e1635_d_b38, eq128_e1635_d_b39, eq128_e1635_d_b40, eq128_e1635_d_b41, eq128_e1635_d_b42, eq128_e1635_d_b43, eq128_e1635_d_b44, eq128_e1635_d_b45, eq128_e1635_d_b46, eq128_e1635_d_b47, eq128_e1635_d_b48, eq128_e1635_d_b49, eq128_e1635_d_b50, eq128_e1635_d_b51, eq128_e1635_d_b52, eq128_e1635_d_b53, eq128_e1635_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1637;
        let eq128_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_branch_derivatives: [f64; 55] = [eq128_e1637_d_b0, eq128_e1637_d_b1, eq128_e1637_d_b2, eq128_e1637_d_b3, eq128_e1637_d_b4, eq128_e1637_d_b5, eq128_e1637_d_b6, eq128_e1637_d_b7, eq128_e1637_d_b8, eq128_e1637_d_b9, eq128_e1637_d_b10, eq128_e1637_d_b11, eq128_e1637_d_b12, eq128_e1637_d_b13, eq128_e1637_d_b14, eq128_e1637_d_b15, eq128_e1637_d_b16, eq128_e1637_d_b17, eq128_e1637_d_b18, eq128_e1637_d_b19, eq128_e1637_d_b20, eq128_e1637_d_b21, eq128_e1637_d_b22, eq128_e1637_d_b23, eq128_e1637_d_b24, eq128_e1637_d_b25, eq128_e1637_d_b26, eq128_e1637_d_b27, eq128_e1637_d_b28, eq128_e1637_d_b29, eq128_e1637_d_b30, eq128_e1637_d_b31, eq128_e1637_d_b32, eq128_e1637_d_b33, eq128_e1637_d_b34, eq128_e1637_d_b35, eq128_e1637_d_b36, eq128_e1637_d_b37, eq128_e1637_d_b38, eq128_e1637_d_b39, eq128_e1637_d_b40, eq128_e1637_d_b41, eq128_e1637_d_b42, eq128_e1637_d_b43, eq128_e1637_d_b44, eq128_e1637_d_b45, eq128_e1637_d_b46, eq128_e1637_d_b47, eq128_e1637_d_b48, eq128_e1637_d_b49, eq128_e1637_d_b50, eq128_e1637_d_b51, eq128_e1637_d_b52, eq128_e1637_d_b53, eq128_e1637_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq128_value),
            &eq128_node_derivatives,
            &eq128_branch_derivatives,
            multiplicity,
        );
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22, eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq129_e1647: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 28, s.v[228]);
        let eq129_e1648: f64 = (p.p7 * eq129_e1647);
        (eq129_e1648, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1650;
        let eq129_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_branch_derivatives: [f64; 55] = [eq129_e1650_d_b0, eq129_e1650_d_b1, eq129_e1650_d_b2, eq129_e1650_d_b3, eq129_e1650_d_b4, eq129_e1650_d_b5, eq129_e1650_d_b6, eq129_e1650_d_b7, eq129_e1650_d_b8, eq129_e1650_d_b9, eq129_e1650_d_b10, eq129_e1650_d_b11, eq129_e1650_d_b12, eq129_e1650_d_b13, eq129_e1650_d_b14, eq129_e1650_d_b15, eq129_e1650_d_b16, eq129_e1650_d_b17, eq129_e1650_d_b18, eq129_e1650_d_b19, eq129_e1650_d_b20, eq129_e1650_d_b21, eq129_e1650_d_b22, eq129_e1650_d_b23, eq129_e1650_d_b24, eq129_e1650_d_b25, eq129_e1650_d_b26, eq129_e1650_d_b27, eq129_e1650_d_b28, eq129_e1650_d_b29, eq129_e1650_d_b30, eq129_e1650_d_b31, eq129_e1650_d_b32, eq129_e1650_d_b33, eq129_e1650_d_b34, eq129_e1650_d_b35, eq129_e1650_d_b36, eq129_e1650_d_b37, eq129_e1650_d_b38, eq129_e1650_d_b39, eq129_e1650_d_b40, eq129_e1650_d_b41, eq129_e1650_d_b42, eq129_e1650_d_b43, eq129_e1650_d_b44, eq129_e1650_d_b45, eq129_e1650_d_b46, eq129_e1650_d_b47, eq129_e1650_d_b48, eq129_e1650_d_b49, eq129_e1650_d_b50, eq129_e1650_d_b51, eq129_e1650_d_b52, eq129_e1650_d_b53, eq129_e1650_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq129_value),
            &eq129_node_derivatives,
            &eq129_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_18(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22, eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54,) = {
    if (((!s.b[570]) && s.b[573]) && (!s.b[574])) {
        let eq130_e1660: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 29, s.v[228]);
        let eq130_e1661: f64 = (p.p7 * eq130_e1660);
        let eq130_e1661_d_n0: f64 = (p.p7 * (s.dn[228][0] * ddt_scale));
        let eq130_e1661_d_n1: f64 = (p.p7 * (s.dn[228][1] * ddt_scale));
        let eq130_e1661_d_n2: f64 = (p.p7 * (s.dn[228][2] * ddt_scale));
        let eq130_e1661_d_n3: f64 = (p.p7 * (s.dn[228][3] * ddt_scale));
        let eq130_e1661_d_n4: f64 = (p.p7 * (s.dn[228][4] * ddt_scale));
        let eq130_e1661_d_n5: f64 = (p.p7 * (s.dn[228][5] * ddt_scale));
        let eq130_e1661_d_n6: f64 = (p.p7 * (s.dn[228][6] * ddt_scale));
        let eq130_e1661_d_n7: f64 = (p.p7 * (s.dn[228][7] * ddt_scale));
        let eq130_e1661_d_n8: f64 = (p.p7 * (s.dn[228][8] * ddt_scale));
        let eq130_e1661_d_n9: f64 = (p.p7 * (s.dn[228][9] * ddt_scale));
        let eq130_e1661_d_n10: f64 = (p.p7 * (s.dn[228][10] * ddt_scale));
        let eq130_e1661_d_n11: f64 = (p.p7 * (s.dn[228][11] * ddt_scale));
        let eq130_e1661_d_n12: f64 = (p.p7 * (s.dn[228][12] * ddt_scale));
        let eq130_e1661_d_n13: f64 = (p.p7 * (s.dn[228][13] * ddt_scale));
        let eq130_e1661_d_n14: f64 = (p.p7 * (s.dn[228][14] * ddt_scale));
        let eq130_e1661_d_n15: f64 = (p.p7 * (s.dn[228][15] * ddt_scale));
        let eq130_e1661_d_n16: f64 = (p.p7 * (s.dn[228][16] * ddt_scale));
        let eq130_e1661_d_n17: f64 = (p.p7 * (s.dn[228][17] * ddt_scale));
        let eq130_e1661_d_n18: f64 = (p.p7 * (s.dn[228][18] * ddt_scale));
        let eq130_e1661_d_n19: f64 = (p.p7 * (s.dn[228][19] * ddt_scale));
        let eq130_e1661_d_n20: f64 = (p.p7 * (s.dn[228][20] * ddt_scale));
        let eq130_e1661_d_n21: f64 = (p.p7 * (s.dn[228][21] * ddt_scale));
        let eq130_e1661_d_n22: f64 = (p.p7 * (s.dn[228][22] * ddt_scale));
        let eq130_e1661_d_b0: f64 = (p.p7 * (s.db[228][0] * ddt_scale));
        let eq130_e1661_d_b1: f64 = (p.p7 * (s.db[228][1] * ddt_scale));
        let eq130_e1661_d_b2: f64 = (p.p7 * (s.db[228][2] * ddt_scale));
        let eq130_e1661_d_b3: f64 = (p.p7 * (s.db[228][3] * ddt_scale));
        let eq130_e1661_d_b4: f64 = (p.p7 * (s.db[228][4] * ddt_scale));
        let eq130_e1661_d_b5: f64 = (p.p7 * (s.db[228][5] * ddt_scale));
        let eq130_e1661_d_b6: f64 = (p.p7 * (s.db[228][6] * ddt_scale));
        let eq130_e1661_d_b7: f64 = (p.p7 * (s.db[228][7] * ddt_scale));
        let eq130_e1661_d_b8: f64 = (p.p7 * (s.db[228][8] * ddt_scale));
        let eq130_e1661_d_b9: f64 = (p.p7 * (s.db[228][9] * ddt_scale));
        let eq130_e1661_d_b10: f64 = (p.p7 * (s.db[228][10] * ddt_scale));
        let eq130_e1661_d_b11: f64 = (p.p7 * (s.db[228][11] * ddt_scale));
        let eq130_e1661_d_b12: f64 = (p.p7 * (s.db[228][12] * ddt_scale));
        let eq130_e1661_d_b13: f64 = (p.p7 * (s.db[228][13] * ddt_scale));
        let eq130_e1661_d_b14: f64 = (p.p7 * (s.db[228][14] * ddt_scale));
        let eq130_e1661_d_b15: f64 = (p.p7 * (s.db[228][15] * ddt_scale));
        let eq130_e1661_d_b16: f64 = (p.p7 * (s.db[228][16] * ddt_scale));
        let eq130_e1661_d_b17: f64 = (p.p7 * (s.db[228][17] * ddt_scale));
        let eq130_e1661_d_b18: f64 = (p.p7 * (s.db[228][18] * ddt_scale));
        let eq130_e1661_d_b19: f64 = (p.p7 * (s.db[228][19] * ddt_scale));
        let eq130_e1661_d_b20: f64 = (p.p7 * (s.db[228][20] * ddt_scale));
        let eq130_e1661_d_b21: f64 = (p.p7 * (s.db[228][21] * ddt_scale));
        let eq130_e1661_d_b22: f64 = (p.p7 * (s.db[228][22] * ddt_scale));
        let eq130_e1661_d_b23: f64 = (p.p7 * (s.db[228][23] * ddt_scale));
        let eq130_e1661_d_b24: f64 = (p.p7 * (s.db[228][24] * ddt_scale));
        let eq130_e1661_d_b25: f64 = (p.p7 * (s.db[228][25] * ddt_scale));
        let eq130_e1661_d_b26: f64 = (p.p7 * (s.db[228][26] * ddt_scale));
        let eq130_e1661_d_b27: f64 = (p.p7 * (s.db[228][27] * ddt_scale));
        let eq130_e1661_d_b28: f64 = (p.p7 * (s.db[228][28] * ddt_scale));
        let eq130_e1661_d_b29: f64 = (p.p7 * (s.db[228][29] * ddt_scale));
        let eq130_e1661_d_b30: f64 = (p.p7 * (s.db[228][30] * ddt_scale));
        let eq130_e1661_d_b31: f64 = (p.p7 * (s.db[228][31] * ddt_scale));
        let eq130_e1661_d_b32: f64 = (p.p7 * (s.db[228][32] * ddt_scale));
        let eq130_e1661_d_b33: f64 = (p.p7 * (s.db[228][33] * ddt_scale));
        let eq130_e1661_d_b34: f64 = (p.p7 * (s.db[228][34] * ddt_scale));
        let eq130_e1661_d_b35: f64 = (p.p7 * (s.db[228][35] * ddt_scale));
        let eq130_e1661_d_b36: f64 = (p.p7 * (s.db[228][36] * ddt_scale));
        let eq130_e1661_d_b37: f64 = (p.p7 * (s.db[228][37] * ddt_scale));
        let eq130_e1661_d_b38: f64 = (p.p7 * (s.db[228][38] * ddt_scale));
        let eq130_e1661_d_b39: f64 = (p.p7 * (s.db[228][39] * ddt_scale));
        let eq130_e1661_d_b40: f64 = (p.p7 * (s.db[228][40] * ddt_scale));
        let eq130_e1661_d_b41: f64 = (p.p7 * (s.db[228][41] * ddt_scale));
        let eq130_e1661_d_b42: f64 = (p.p7 * (s.db[228][42] * ddt_scale));
        let eq130_e1661_d_b43: f64 = (p.p7 * (s.db[228][43] * ddt_scale));
        let eq130_e1661_d_b44: f64 = (p.p7 * (s.db[228][44] * ddt_scale));
        let eq130_e1661_d_b45: f64 = (p.p7 * (s.db[228][45] * ddt_scale));
        let eq130_e1661_d_b46: f64 = (p.p7 * (s.db[228][46] * ddt_scale));
        let eq130_e1661_d_b47: f64 = (p.p7 * (s.db[228][47] * ddt_scale));
        let eq130_e1661_d_b48: f64 = (p.p7 * (s.db[228][48] * ddt_scale));
        let eq130_e1661_d_b49: f64 = (p.p7 * (s.db[228][49] * ddt_scale));
        let eq130_e1661_d_b50: f64 = (p.p7 * (s.db[228][50] * ddt_scale));
        let eq130_e1661_d_b51: f64 = (p.p7 * (s.db[228][51] * ddt_scale));
        let eq130_e1661_d_b52: f64 = (p.p7 * (s.db[228][52] * ddt_scale));
        let eq130_e1661_d_b53: f64 = (p.p7 * (s.db[228][53] * ddt_scale));
        let eq130_e1661_d_b54: f64 = (p.p7 * (s.db[228][54] * ddt_scale));
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
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22, eq130_e1663_d_b0, eq130_e1663_d_b1, eq130_e1663_d_b2, eq130_e1663_d_b3, eq130_e1663_d_b4, eq130_e1663_d_b5, eq130_e1663_d_b6, eq130_e1663_d_b7, eq130_e1663_d_b8, eq130_e1663_d_b9, eq130_e1663_d_b10, eq130_e1663_d_b11, eq130_e1663_d_b12, eq130_e1663_d_b13, eq130_e1663_d_b14, eq130_e1663_d_b15, eq130_e1663_d_b16, eq130_e1663_d_b17, eq130_e1663_d_b18, eq130_e1663_d_b19, eq130_e1663_d_b20, eq130_e1663_d_b21, eq130_e1663_d_b22, eq130_e1663_d_b23, eq130_e1663_d_b24, eq130_e1663_d_b25, eq130_e1663_d_b26, eq130_e1663_d_b27, eq130_e1663_d_b28, eq130_e1663_d_b29, eq130_e1663_d_b30, eq130_e1663_d_b31, eq130_e1663_d_b32, eq130_e1663_d_b33, eq130_e1663_d_b34, eq130_e1663_d_b35, eq130_e1663_d_b36, eq130_e1663_d_b37, eq130_e1663_d_b38, eq130_e1663_d_b39, eq130_e1663_d_b40, eq130_e1663_d_b41, eq130_e1663_d_b42, eq130_e1663_d_b43, eq130_e1663_d_b44, eq130_e1663_d_b45, eq130_e1663_d_b46, eq130_e1663_d_b47, eq130_e1663_d_b48, eq130_e1663_d_b49, eq130_e1663_d_b50, eq130_e1663_d_b51, eq130_e1663_d_b52, eq130_e1663_d_b53, eq130_e1663_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1665;
        let eq130_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_branch_derivatives: [f64; 55] = [eq130_e1665_d_b0, eq130_e1665_d_b1, eq130_e1665_d_b2, eq130_e1665_d_b3, eq130_e1665_d_b4, eq130_e1665_d_b5, eq130_e1665_d_b6, eq130_e1665_d_b7, eq130_e1665_d_b8, eq130_e1665_d_b9, eq130_e1665_d_b10, eq130_e1665_d_b11, eq130_e1665_d_b12, eq130_e1665_d_b13, eq130_e1665_d_b14, eq130_e1665_d_b15, eq130_e1665_d_b16, eq130_e1665_d_b17, eq130_e1665_d_b18, eq130_e1665_d_b19, eq130_e1665_d_b20, eq130_e1665_d_b21, eq130_e1665_d_b22, eq130_e1665_d_b23, eq130_e1665_d_b24, eq130_e1665_d_b25, eq130_e1665_d_b26, eq130_e1665_d_b27, eq130_e1665_d_b28, eq130_e1665_d_b29, eq130_e1665_d_b30, eq130_e1665_d_b31, eq130_e1665_d_b32, eq130_e1665_d_b33, eq130_e1665_d_b34, eq130_e1665_d_b35, eq130_e1665_d_b36, eq130_e1665_d_b37, eq130_e1665_d_b38, eq130_e1665_d_b39, eq130_e1665_d_b40, eq130_e1665_d_b41, eq130_e1665_d_b42, eq130_e1665_d_b43, eq130_e1665_d_b44, eq130_e1665_d_b45, eq130_e1665_d_b46, eq130_e1665_d_b47, eq130_e1665_d_b48, eq130_e1665_d_b49, eq130_e1665_d_b50, eq130_e1665_d_b51, eq130_e1665_d_b52, eq130_e1665_d_b53, eq130_e1665_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq130_value),
            &eq130_node_derivatives,
            &eq130_branch_derivatives,
            multiplicity,
        );
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22, eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54,) = {
    if ((!s.b[570]) && s.b[573]) {
        let eq131_e1673: f64 = (p.p251 * s.v[228]);
        let eq131_e1674: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 30, eq131_e1673);
        let eq131_e1674_d_n0: f64 = ((p.p251 * s.dn[228][0]) * ddt_scale);
        let eq131_e1674_d_n1: f64 = ((p.p251 * s.dn[228][1]) * ddt_scale);
        let eq131_e1674_d_n2: f64 = ((p.p251 * s.dn[228][2]) * ddt_scale);
        let eq131_e1674_d_n3: f64 = ((p.p251 * s.dn[228][3]) * ddt_scale);
        let eq131_e1674_d_n4: f64 = ((p.p251 * s.dn[228][4]) * ddt_scale);
        let eq131_e1674_d_n5: f64 = ((p.p251 * s.dn[228][5]) * ddt_scale);
        let eq131_e1674_d_n6: f64 = ((p.p251 * s.dn[228][6]) * ddt_scale);
        let eq131_e1674_d_n7: f64 = ((p.p251 * s.dn[228][7]) * ddt_scale);
        let eq131_e1674_d_n8: f64 = ((p.p251 * s.dn[228][8]) * ddt_scale);
        let eq131_e1674_d_n9: f64 = ((p.p251 * s.dn[228][9]) * ddt_scale);
        let eq131_e1674_d_n10: f64 = ((p.p251 * s.dn[228][10]) * ddt_scale);
        let eq131_e1674_d_n11: f64 = ((p.p251 * s.dn[228][11]) * ddt_scale);
        let eq131_e1674_d_n12: f64 = ((p.p251 * s.dn[228][12]) * ddt_scale);
        let eq131_e1674_d_n13: f64 = ((p.p251 * s.dn[228][13]) * ddt_scale);
        let eq131_e1674_d_n14: f64 = ((p.p251 * s.dn[228][14]) * ddt_scale);
        let eq131_e1674_d_n15: f64 = ((p.p251 * s.dn[228][15]) * ddt_scale);
        let eq131_e1674_d_n16: f64 = ((p.p251 * s.dn[228][16]) * ddt_scale);
        let eq131_e1674_d_n17: f64 = ((p.p251 * s.dn[228][17]) * ddt_scale);
        let eq131_e1674_d_n18: f64 = ((p.p251 * s.dn[228][18]) * ddt_scale);
        let eq131_e1674_d_n19: f64 = ((p.p251 * s.dn[228][19]) * ddt_scale);
        let eq131_e1674_d_n20: f64 = ((p.p251 * s.dn[228][20]) * ddt_scale);
        let eq131_e1674_d_n21: f64 = ((p.p251 * s.dn[228][21]) * ddt_scale);
        let eq131_e1674_d_n22: f64 = ((p.p251 * s.dn[228][22]) * ddt_scale);
        let eq131_e1674_d_b0: f64 = ((p.p251 * s.db[228][0]) * ddt_scale);
        let eq131_e1674_d_b1: f64 = ((p.p251 * s.db[228][1]) * ddt_scale);
        let eq131_e1674_d_b2: f64 = ((p.p251 * s.db[228][2]) * ddt_scale);
        let eq131_e1674_d_b3: f64 = ((p.p251 * s.db[228][3]) * ddt_scale);
        let eq131_e1674_d_b4: f64 = ((p.p251 * s.db[228][4]) * ddt_scale);
        let eq131_e1674_d_b5: f64 = ((p.p251 * s.db[228][5]) * ddt_scale);
        let eq131_e1674_d_b6: f64 = ((p.p251 * s.db[228][6]) * ddt_scale);
        let eq131_e1674_d_b7: f64 = ((p.p251 * s.db[228][7]) * ddt_scale);
        let eq131_e1674_d_b8: f64 = ((p.p251 * s.db[228][8]) * ddt_scale);
        let eq131_e1674_d_b9: f64 = ((p.p251 * s.db[228][9]) * ddt_scale);
        let eq131_e1674_d_b10: f64 = ((p.p251 * s.db[228][10]) * ddt_scale);
        let eq131_e1674_d_b11: f64 = ((p.p251 * s.db[228][11]) * ddt_scale);
        let eq131_e1674_d_b12: f64 = ((p.p251 * s.db[228][12]) * ddt_scale);
        let eq131_e1674_d_b13: f64 = ((p.p251 * s.db[228][13]) * ddt_scale);
        let eq131_e1674_d_b14: f64 = ((p.p251 * s.db[228][14]) * ddt_scale);
        let eq131_e1674_d_b15: f64 = ((p.p251 * s.db[228][15]) * ddt_scale);
        let eq131_e1674_d_b16: f64 = ((p.p251 * s.db[228][16]) * ddt_scale);
        let eq131_e1674_d_b17: f64 = ((p.p251 * s.db[228][17]) * ddt_scale);
        let eq131_e1674_d_b18: f64 = ((p.p251 * s.db[228][18]) * ddt_scale);
        let eq131_e1674_d_b19: f64 = ((p.p251 * s.db[228][19]) * ddt_scale);
        let eq131_e1674_d_b20: f64 = ((p.p251 * s.db[228][20]) * ddt_scale);
        let eq131_e1674_d_b21: f64 = ((p.p251 * s.db[228][21]) * ddt_scale);
        let eq131_e1674_d_b22: f64 = ((p.p251 * s.db[228][22]) * ddt_scale);
        let eq131_e1674_d_b23: f64 = ((p.p251 * s.db[228][23]) * ddt_scale);
        let eq131_e1674_d_b24: f64 = ((p.p251 * s.db[228][24]) * ddt_scale);
        let eq131_e1674_d_b25: f64 = ((p.p251 * s.db[228][25]) * ddt_scale);
        let eq131_e1674_d_b26: f64 = ((p.p251 * s.db[228][26]) * ddt_scale);
        let eq131_e1674_d_b27: f64 = ((p.p251 * s.db[228][27]) * ddt_scale);
        let eq131_e1674_d_b28: f64 = ((p.p251 * s.db[228][28]) * ddt_scale);
        let eq131_e1674_d_b29: f64 = ((p.p251 * s.db[228][29]) * ddt_scale);
        let eq131_e1674_d_b30: f64 = ((p.p251 * s.db[228][30]) * ddt_scale);
        let eq131_e1674_d_b31: f64 = ((p.p251 * s.db[228][31]) * ddt_scale);
        let eq131_e1674_d_b32: f64 = ((p.p251 * s.db[228][32]) * ddt_scale);
        let eq131_e1674_d_b33: f64 = ((p.p251 * s.db[228][33]) * ddt_scale);
        let eq131_e1674_d_b34: f64 = ((p.p251 * s.db[228][34]) * ddt_scale);
        let eq131_e1674_d_b35: f64 = ((p.p251 * s.db[228][35]) * ddt_scale);
        let eq131_e1674_d_b36: f64 = ((p.p251 * s.db[228][36]) * ddt_scale);
        let eq131_e1674_d_b37: f64 = ((p.p251 * s.db[228][37]) * ddt_scale);
        let eq131_e1674_d_b38: f64 = ((p.p251 * s.db[228][38]) * ddt_scale);
        let eq131_e1674_d_b39: f64 = ((p.p251 * s.db[228][39]) * ddt_scale);
        let eq131_e1674_d_b40: f64 = ((p.p251 * s.db[228][40]) * ddt_scale);
        let eq131_e1674_d_b41: f64 = ((p.p251 * s.db[228][41]) * ddt_scale);
        let eq131_e1674_d_b42: f64 = ((p.p251 * s.db[228][42]) * ddt_scale);
        let eq131_e1674_d_b43: f64 = ((p.p251 * s.db[228][43]) * ddt_scale);
        let eq131_e1674_d_b44: f64 = ((p.p251 * s.db[228][44]) * ddt_scale);
        let eq131_e1674_d_b45: f64 = ((p.p251 * s.db[228][45]) * ddt_scale);
        let eq131_e1674_d_b46: f64 = ((p.p251 * s.db[228][46]) * ddt_scale);
        let eq131_e1674_d_b47: f64 = ((p.p251 * s.db[228][47]) * ddt_scale);
        let eq131_e1674_d_b48: f64 = ((p.p251 * s.db[228][48]) * ddt_scale);
        let eq131_e1674_d_b49: f64 = ((p.p251 * s.db[228][49]) * ddt_scale);
        let eq131_e1674_d_b50: f64 = ((p.p251 * s.db[228][50]) * ddt_scale);
        let eq131_e1674_d_b51: f64 = ((p.p251 * s.db[228][51]) * ddt_scale);
        let eq131_e1674_d_b52: f64 = ((p.p251 * s.db[228][52]) * ddt_scale);
        let eq131_e1674_d_b53: f64 = ((p.p251 * s.db[228][53]) * ddt_scale);
        let eq131_e1674_d_b54: f64 = ((p.p251 * s.db[228][54]) * ddt_scale);
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
        let eq131_e1675_d_b0: f64 = (p.p7 * eq131_e1674_d_b0);
        let eq131_e1675_d_b1: f64 = (p.p7 * eq131_e1674_d_b1);
        let eq131_e1675_d_b2: f64 = (p.p7 * eq131_e1674_d_b2);
        let eq131_e1675_d_b3: f64 = (p.p7 * eq131_e1674_d_b3);
        let eq131_e1675_d_b4: f64 = (p.p7 * eq131_e1674_d_b4);
        let eq131_e1675_d_b5: f64 = (p.p7 * eq131_e1674_d_b5);
        let eq131_e1675_d_b6: f64 = (p.p7 * eq131_e1674_d_b6);
        let eq131_e1675_d_b7: f64 = (p.p7 * eq131_e1674_d_b7);
        let eq131_e1675_d_b8: f64 = (p.p7 * eq131_e1674_d_b8);
        let eq131_e1675_d_b9: f64 = (p.p7 * eq131_e1674_d_b9);
        let eq131_e1675_d_b10: f64 = (p.p7 * eq131_e1674_d_b10);
        let eq131_e1675_d_b11: f64 = (p.p7 * eq131_e1674_d_b11);
        let eq131_e1675_d_b12: f64 = (p.p7 * eq131_e1674_d_b12);
        let eq131_e1675_d_b13: f64 = (p.p7 * eq131_e1674_d_b13);
        let eq131_e1675_d_b14: f64 = (p.p7 * eq131_e1674_d_b14);
        let eq131_e1675_d_b15: f64 = (p.p7 * eq131_e1674_d_b15);
        let eq131_e1675_d_b16: f64 = (p.p7 * eq131_e1674_d_b16);
        let eq131_e1675_d_b17: f64 = (p.p7 * eq131_e1674_d_b17);
        let eq131_e1675_d_b18: f64 = (p.p7 * eq131_e1674_d_b18);
        let eq131_e1675_d_b19: f64 = (p.p7 * eq131_e1674_d_b19);
        let eq131_e1675_d_b20: f64 = (p.p7 * eq131_e1674_d_b20);
        let eq131_e1675_d_b21: f64 = (p.p7 * eq131_e1674_d_b21);
        let eq131_e1675_d_b22: f64 = (p.p7 * eq131_e1674_d_b22);
        let eq131_e1675_d_b23: f64 = (p.p7 * eq131_e1674_d_b23);
        let eq131_e1675_d_b24: f64 = (p.p7 * eq131_e1674_d_b24);
        let eq131_e1675_d_b25: f64 = (p.p7 * eq131_e1674_d_b25);
        let eq131_e1675_d_b26: f64 = (p.p7 * eq131_e1674_d_b26);
        let eq131_e1675_d_b27: f64 = (p.p7 * eq131_e1674_d_b27);
        let eq131_e1675_d_b28: f64 = (p.p7 * eq131_e1674_d_b28);
        let eq131_e1675_d_b29: f64 = (p.p7 * eq131_e1674_d_b29);
        let eq131_e1675_d_b30: f64 = (p.p7 * eq131_e1674_d_b30);
        let eq131_e1675_d_b31: f64 = (p.p7 * eq131_e1674_d_b31);
        let eq131_e1675_d_b32: f64 = (p.p7 * eq131_e1674_d_b32);
        let eq131_e1675_d_b33: f64 = (p.p7 * eq131_e1674_d_b33);
        let eq131_e1675_d_b34: f64 = (p.p7 * eq131_e1674_d_b34);
        let eq131_e1675_d_b35: f64 = (p.p7 * eq131_e1674_d_b35);
        let eq131_e1675_d_b36: f64 = (p.p7 * eq131_e1674_d_b36);
        let eq131_e1675_d_b37: f64 = (p.p7 * eq131_e1674_d_b37);
        let eq131_e1675_d_b38: f64 = (p.p7 * eq131_e1674_d_b38);
        let eq131_e1675_d_b39: f64 = (p.p7 * eq131_e1674_d_b39);
        let eq131_e1675_d_b40: f64 = (p.p7 * eq131_e1674_d_b40);
        let eq131_e1675_d_b41: f64 = (p.p7 * eq131_e1674_d_b41);
        let eq131_e1675_d_b42: f64 = (p.p7 * eq131_e1674_d_b42);
        let eq131_e1675_d_b43: f64 = (p.p7 * eq131_e1674_d_b43);
        let eq131_e1675_d_b44: f64 = (p.p7 * eq131_e1674_d_b44);
        let eq131_e1675_d_b45: f64 = (p.p7 * eq131_e1674_d_b45);
        let eq131_e1675_d_b46: f64 = (p.p7 * eq131_e1674_d_b46);
        let eq131_e1675_d_b47: f64 = (p.p7 * eq131_e1674_d_b47);
        let eq131_e1675_d_b48: f64 = (p.p7 * eq131_e1674_d_b48);
        let eq131_e1675_d_b49: f64 = (p.p7 * eq131_e1674_d_b49);
        let eq131_e1675_d_b50: f64 = (p.p7 * eq131_e1674_d_b50);
        let eq131_e1675_d_b51: f64 = (p.p7 * eq131_e1674_d_b51);
        let eq131_e1675_d_b52: f64 = (p.p7 * eq131_e1674_d_b52);
        let eq131_e1675_d_b53: f64 = (p.p7 * eq131_e1674_d_b53);
        let eq131_e1675_d_b54: f64 = (p.p7 * eq131_e1674_d_b54);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22, eq131_e1675_d_b0, eq131_e1675_d_b1, eq131_e1675_d_b2, eq131_e1675_d_b3, eq131_e1675_d_b4, eq131_e1675_d_b5, eq131_e1675_d_b6, eq131_e1675_d_b7, eq131_e1675_d_b8, eq131_e1675_d_b9, eq131_e1675_d_b10, eq131_e1675_d_b11, eq131_e1675_d_b12, eq131_e1675_d_b13, eq131_e1675_d_b14, eq131_e1675_d_b15, eq131_e1675_d_b16, eq131_e1675_d_b17, eq131_e1675_d_b18, eq131_e1675_d_b19, eq131_e1675_d_b20, eq131_e1675_d_b21, eq131_e1675_d_b22, eq131_e1675_d_b23, eq131_e1675_d_b24, eq131_e1675_d_b25, eq131_e1675_d_b26, eq131_e1675_d_b27, eq131_e1675_d_b28, eq131_e1675_d_b29, eq131_e1675_d_b30, eq131_e1675_d_b31, eq131_e1675_d_b32, eq131_e1675_d_b33, eq131_e1675_d_b34, eq131_e1675_d_b35, eq131_e1675_d_b36, eq131_e1675_d_b37, eq131_e1675_d_b38, eq131_e1675_d_b39, eq131_e1675_d_b40, eq131_e1675_d_b41, eq131_e1675_d_b42, eq131_e1675_d_b43, eq131_e1675_d_b44, eq131_e1675_d_b45, eq131_e1675_d_b46, eq131_e1675_d_b47, eq131_e1675_d_b48, eq131_e1675_d_b49, eq131_e1675_d_b50, eq131_e1675_d_b51, eq131_e1675_d_b52, eq131_e1675_d_b53, eq131_e1675_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1677;
        let eq131_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_branch_derivatives: [f64; 55] = [eq131_e1677_d_b0, eq131_e1677_d_b1, eq131_e1677_d_b2, eq131_e1677_d_b3, eq131_e1677_d_b4, eq131_e1677_d_b5, eq131_e1677_d_b6, eq131_e1677_d_b7, eq131_e1677_d_b8, eq131_e1677_d_b9, eq131_e1677_d_b10, eq131_e1677_d_b11, eq131_e1677_d_b12, eq131_e1677_d_b13, eq131_e1677_d_b14, eq131_e1677_d_b15, eq131_e1677_d_b16, eq131_e1677_d_b17, eq131_e1677_d_b18, eq131_e1677_d_b19, eq131_e1677_d_b20, eq131_e1677_d_b21, eq131_e1677_d_b22, eq131_e1677_d_b23, eq131_e1677_d_b24, eq131_e1677_d_b25, eq131_e1677_d_b26, eq131_e1677_d_b27, eq131_e1677_d_b28, eq131_e1677_d_b29, eq131_e1677_d_b30, eq131_e1677_d_b31, eq131_e1677_d_b32, eq131_e1677_d_b33, eq131_e1677_d_b34, eq131_e1677_d_b35, eq131_e1677_d_b36, eq131_e1677_d_b37, eq131_e1677_d_b38, eq131_e1677_d_b39, eq131_e1677_d_b40, eq131_e1677_d_b41, eq131_e1677_d_b42, eq131_e1677_d_b43, eq131_e1677_d_b44, eq131_e1677_d_b45, eq131_e1677_d_b46, eq131_e1677_d_b47, eq131_e1677_d_b48, eq131_e1677_d_b49, eq131_e1677_d_b50, eq131_e1677_d_b51, eq131_e1677_d_b52, eq131_e1677_d_b53, eq131_e1677_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq131_value),
            &eq131_node_derivatives,
            &eq131_branch_derivatives,
            multiplicity,
        );
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22, eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54,) = {
    if (s.b[575] && s.b[576]) {
        let eq132_e1683: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 31, s.v[241]);
        let eq132_e1684: f64 = (p.p7 * eq132_e1683);
        let eq132_e1684_d_n0: f64 = (p.p7 * (s.dn[241][0] * ddt_scale));
        let eq132_e1684_d_n1: f64 = (p.p7 * (s.dn[241][1] * ddt_scale));
        let eq132_e1684_d_n2: f64 = (p.p7 * (s.dn[241][2] * ddt_scale));
        let eq132_e1684_d_n3: f64 = (p.p7 * (s.dn[241][3] * ddt_scale));
        let eq132_e1684_d_n4: f64 = (p.p7 * (s.dn[241][4] * ddt_scale));
        let eq132_e1684_d_n5: f64 = (p.p7 * (s.dn[241][5] * ddt_scale));
        let eq132_e1684_d_n6: f64 = (p.p7 * (s.dn[241][6] * ddt_scale));
        let eq132_e1684_d_n7: f64 = (p.p7 * (s.dn[241][7] * ddt_scale));
        let eq132_e1684_d_n8: f64 = (p.p7 * (s.dn[241][8] * ddt_scale));
        let eq132_e1684_d_n9: f64 = (p.p7 * (s.dn[241][9] * ddt_scale));
        let eq132_e1684_d_n10: f64 = (p.p7 * (s.dn[241][10] * ddt_scale));
        let eq132_e1684_d_n11: f64 = (p.p7 * (s.dn[241][11] * ddt_scale));
        let eq132_e1684_d_n12: f64 = (p.p7 * (s.dn[241][12] * ddt_scale));
        let eq132_e1684_d_n13: f64 = (p.p7 * (s.dn[241][13] * ddt_scale));
        let eq132_e1684_d_n14: f64 = (p.p7 * (s.dn[241][14] * ddt_scale));
        let eq132_e1684_d_n15: f64 = (p.p7 * (s.dn[241][15] * ddt_scale));
        let eq132_e1684_d_n16: f64 = (p.p7 * (s.dn[241][16] * ddt_scale));
        let eq132_e1684_d_n17: f64 = (p.p7 * (s.dn[241][17] * ddt_scale));
        let eq132_e1684_d_n18: f64 = (p.p7 * (s.dn[241][18] * ddt_scale));
        let eq132_e1684_d_n19: f64 = (p.p7 * (s.dn[241][19] * ddt_scale));
        let eq132_e1684_d_n20: f64 = (p.p7 * (s.dn[241][20] * ddt_scale));
        let eq132_e1684_d_n21: f64 = (p.p7 * (s.dn[241][21] * ddt_scale));
        let eq132_e1684_d_n22: f64 = (p.p7 * (s.dn[241][22] * ddt_scale));
        let eq132_e1684_d_b0: f64 = (p.p7 * (s.db[241][0] * ddt_scale));
        let eq132_e1684_d_b1: f64 = (p.p7 * (s.db[241][1] * ddt_scale));
        let eq132_e1684_d_b2: f64 = (p.p7 * (s.db[241][2] * ddt_scale));
        let eq132_e1684_d_b3: f64 = (p.p7 * (s.db[241][3] * ddt_scale));
        let eq132_e1684_d_b4: f64 = (p.p7 * (s.db[241][4] * ddt_scale));
        let eq132_e1684_d_b5: f64 = (p.p7 * (s.db[241][5] * ddt_scale));
        let eq132_e1684_d_b6: f64 = (p.p7 * (s.db[241][6] * ddt_scale));
        let eq132_e1684_d_b7: f64 = (p.p7 * (s.db[241][7] * ddt_scale));
        let eq132_e1684_d_b8: f64 = (p.p7 * (s.db[241][8] * ddt_scale));
        let eq132_e1684_d_b9: f64 = (p.p7 * (s.db[241][9] * ddt_scale));
        let eq132_e1684_d_b10: f64 = (p.p7 * (s.db[241][10] * ddt_scale));
        let eq132_e1684_d_b11: f64 = (p.p7 * (s.db[241][11] * ddt_scale));
        let eq132_e1684_d_b12: f64 = (p.p7 * (s.db[241][12] * ddt_scale));
        let eq132_e1684_d_b13: f64 = (p.p7 * (s.db[241][13] * ddt_scale));
        let eq132_e1684_d_b14: f64 = (p.p7 * (s.db[241][14] * ddt_scale));
        let eq132_e1684_d_b15: f64 = (p.p7 * (s.db[241][15] * ddt_scale));
        let eq132_e1684_d_b16: f64 = (p.p7 * (s.db[241][16] * ddt_scale));
        let eq132_e1684_d_b17: f64 = (p.p7 * (s.db[241][17] * ddt_scale));
        let eq132_e1684_d_b18: f64 = (p.p7 * (s.db[241][18] * ddt_scale));
        let eq132_e1684_d_b19: f64 = (p.p7 * (s.db[241][19] * ddt_scale));
        let eq132_e1684_d_b20: f64 = (p.p7 * (s.db[241][20] * ddt_scale));
        let eq132_e1684_d_b21: f64 = (p.p7 * (s.db[241][21] * ddt_scale));
        let eq132_e1684_d_b22: f64 = (p.p7 * (s.db[241][22] * ddt_scale));
        let eq132_e1684_d_b23: f64 = (p.p7 * (s.db[241][23] * ddt_scale));
        let eq132_e1684_d_b24: f64 = (p.p7 * (s.db[241][24] * ddt_scale));
        let eq132_e1684_d_b25: f64 = (p.p7 * (s.db[241][25] * ddt_scale));
        let eq132_e1684_d_b26: f64 = (p.p7 * (s.db[241][26] * ddt_scale));
        let eq132_e1684_d_b27: f64 = (p.p7 * (s.db[241][27] * ddt_scale));
        let eq132_e1684_d_b28: f64 = (p.p7 * (s.db[241][28] * ddt_scale));
        let eq132_e1684_d_b29: f64 = (p.p7 * (s.db[241][29] * ddt_scale));
        let eq132_e1684_d_b30: f64 = (p.p7 * (s.db[241][30] * ddt_scale));
        let eq132_e1684_d_b31: f64 = (p.p7 * (s.db[241][31] * ddt_scale));
        let eq132_e1684_d_b32: f64 = (p.p7 * (s.db[241][32] * ddt_scale));
        let eq132_e1684_d_b33: f64 = (p.p7 * (s.db[241][33] * ddt_scale));
        let eq132_e1684_d_b34: f64 = (p.p7 * (s.db[241][34] * ddt_scale));
        let eq132_e1684_d_b35: f64 = (p.p7 * (s.db[241][35] * ddt_scale));
        let eq132_e1684_d_b36: f64 = (p.p7 * (s.db[241][36] * ddt_scale));
        let eq132_e1684_d_b37: f64 = (p.p7 * (s.db[241][37] * ddt_scale));
        let eq132_e1684_d_b38: f64 = (p.p7 * (s.db[241][38] * ddt_scale));
        let eq132_e1684_d_b39: f64 = (p.p7 * (s.db[241][39] * ddt_scale));
        let eq132_e1684_d_b40: f64 = (p.p7 * (s.db[241][40] * ddt_scale));
        let eq132_e1684_d_b41: f64 = (p.p7 * (s.db[241][41] * ddt_scale));
        let eq132_e1684_d_b42: f64 = (p.p7 * (s.db[241][42] * ddt_scale));
        let eq132_e1684_d_b43: f64 = (p.p7 * (s.db[241][43] * ddt_scale));
        let eq132_e1684_d_b44: f64 = (p.p7 * (s.db[241][44] * ddt_scale));
        let eq132_e1684_d_b45: f64 = (p.p7 * (s.db[241][45] * ddt_scale));
        let eq132_e1684_d_b46: f64 = (p.p7 * (s.db[241][46] * ddt_scale));
        let eq132_e1684_d_b47: f64 = (p.p7 * (s.db[241][47] * ddt_scale));
        let eq132_e1684_d_b48: f64 = (p.p7 * (s.db[241][48] * ddt_scale));
        let eq132_e1684_d_b49: f64 = (p.p7 * (s.db[241][49] * ddt_scale));
        let eq132_e1684_d_b50: f64 = (p.p7 * (s.db[241][50] * ddt_scale));
        let eq132_e1684_d_b51: f64 = (p.p7 * (s.db[241][51] * ddt_scale));
        let eq132_e1684_d_b52: f64 = (p.p7 * (s.db[241][52] * ddt_scale));
        let eq132_e1684_d_b53: f64 = (p.p7 * (s.db[241][53] * ddt_scale));
        let eq132_e1684_d_b54: f64 = (p.p7 * (s.db[241][54] * ddt_scale));
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22, eq132_e1684_d_b0, eq132_e1684_d_b1, eq132_e1684_d_b2, eq132_e1684_d_b3, eq132_e1684_d_b4, eq132_e1684_d_b5, eq132_e1684_d_b6, eq132_e1684_d_b7, eq132_e1684_d_b8, eq132_e1684_d_b9, eq132_e1684_d_b10, eq132_e1684_d_b11, eq132_e1684_d_b12, eq132_e1684_d_b13, eq132_e1684_d_b14, eq132_e1684_d_b15, eq132_e1684_d_b16, eq132_e1684_d_b17, eq132_e1684_d_b18, eq132_e1684_d_b19, eq132_e1684_d_b20, eq132_e1684_d_b21, eq132_e1684_d_b22, eq132_e1684_d_b23, eq132_e1684_d_b24, eq132_e1684_d_b25, eq132_e1684_d_b26, eq132_e1684_d_b27, eq132_e1684_d_b28, eq132_e1684_d_b29, eq132_e1684_d_b30, eq132_e1684_d_b31, eq132_e1684_d_b32, eq132_e1684_d_b33, eq132_e1684_d_b34, eq132_e1684_d_b35, eq132_e1684_d_b36, eq132_e1684_d_b37, eq132_e1684_d_b38, eq132_e1684_d_b39, eq132_e1684_d_b40, eq132_e1684_d_b41, eq132_e1684_d_b42, eq132_e1684_d_b43, eq132_e1684_d_b44, eq132_e1684_d_b45, eq132_e1684_d_b46, eq132_e1684_d_b47, eq132_e1684_d_b48, eq132_e1684_d_b49, eq132_e1684_d_b50, eq132_e1684_d_b51, eq132_e1684_d_b52, eq132_e1684_d_b53, eq132_e1684_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_value: f64 = eq132_e1686;
        let eq132_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_branch_derivatives: [f64; 55] = [eq132_e1686_d_b0, eq132_e1686_d_b1, eq132_e1686_d_b2, eq132_e1686_d_b3, eq132_e1686_d_b4, eq132_e1686_d_b5, eq132_e1686_d_b6, eq132_e1686_d_b7, eq132_e1686_d_b8, eq132_e1686_d_b9, eq132_e1686_d_b10, eq132_e1686_d_b11, eq132_e1686_d_b12, eq132_e1686_d_b13, eq132_e1686_d_b14, eq132_e1686_d_b15, eq132_e1686_d_b16, eq132_e1686_d_b17, eq132_e1686_d_b18, eq132_e1686_d_b19, eq132_e1686_d_b20, eq132_e1686_d_b21, eq132_e1686_d_b22, eq132_e1686_d_b23, eq132_e1686_d_b24, eq132_e1686_d_b25, eq132_e1686_d_b26, eq132_e1686_d_b27, eq132_e1686_d_b28, eq132_e1686_d_b29, eq132_e1686_d_b30, eq132_e1686_d_b31, eq132_e1686_d_b32, eq132_e1686_d_b33, eq132_e1686_d_b34, eq132_e1686_d_b35, eq132_e1686_d_b36, eq132_e1686_d_b37, eq132_e1686_d_b38, eq132_e1686_d_b39, eq132_e1686_d_b40, eq132_e1686_d_b41, eq132_e1686_d_b42, eq132_e1686_d_b43, eq132_e1686_d_b44, eq132_e1686_d_b45, eq132_e1686_d_b46, eq132_e1686_d_b47, eq132_e1686_d_b48, eq132_e1686_d_b49, eq132_e1686_d_b50, eq132_e1686_d_b51, eq132_e1686_d_b52, eq132_e1686_d_b53, eq132_e1686_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(19),
            multiplicity * (eq132_value),
            &eq132_node_derivatives,
            &eq132_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_19(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22, eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq133_e1694: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 32, s.v[240]);
        let eq133_e1695: f64 = (p.p7 * eq133_e1694);
        (eq133_e1695, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_value: f64 = eq133_e1697;
        let eq133_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_branch_derivatives: [f64; 55] = [eq133_e1697_d_b0, eq133_e1697_d_b1, eq133_e1697_d_b2, eq133_e1697_d_b3, eq133_e1697_d_b4, eq133_e1697_d_b5, eq133_e1697_d_b6, eq133_e1697_d_b7, eq133_e1697_d_b8, eq133_e1697_d_b9, eq133_e1697_d_b10, eq133_e1697_d_b11, eq133_e1697_d_b12, eq133_e1697_d_b13, eq133_e1697_d_b14, eq133_e1697_d_b15, eq133_e1697_d_b16, eq133_e1697_d_b17, eq133_e1697_d_b18, eq133_e1697_d_b19, eq133_e1697_d_b20, eq133_e1697_d_b21, eq133_e1697_d_b22, eq133_e1697_d_b23, eq133_e1697_d_b24, eq133_e1697_d_b25, eq133_e1697_d_b26, eq133_e1697_d_b27, eq133_e1697_d_b28, eq133_e1697_d_b29, eq133_e1697_d_b30, eq133_e1697_d_b31, eq133_e1697_d_b32, eq133_e1697_d_b33, eq133_e1697_d_b34, eq133_e1697_d_b35, eq133_e1697_d_b36, eq133_e1697_d_b37, eq133_e1697_d_b38, eq133_e1697_d_b39, eq133_e1697_d_b40, eq133_e1697_d_b41, eq133_e1697_d_b42, eq133_e1697_d_b43, eq133_e1697_d_b44, eq133_e1697_d_b45, eq133_e1697_d_b46, eq133_e1697_d_b47, eq133_e1697_d_b48, eq133_e1697_d_b49, eq133_e1697_d_b50, eq133_e1697_d_b51, eq133_e1697_d_b52, eq133_e1697_d_b53, eq133_e1697_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(19),
            multiplicity * (eq133_value),
            &eq133_node_derivatives,
            &eq133_branch_derivatives,
            multiplicity,
        );
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22, eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54,) = {
    if ((s.b[575] && s.b[576]) && s.b[577]) {
        let eq134_e1705: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 33, s.v[240]);
        let eq134_e1706: f64 = (p.p7 * eq134_e1705);
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
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22, eq134_e1708_d_b0, eq134_e1708_d_b1, eq134_e1708_d_b2, eq134_e1708_d_b3, eq134_e1708_d_b4, eq134_e1708_d_b5, eq134_e1708_d_b6, eq134_e1708_d_b7, eq134_e1708_d_b8, eq134_e1708_d_b9, eq134_e1708_d_b10, eq134_e1708_d_b11, eq134_e1708_d_b12, eq134_e1708_d_b13, eq134_e1708_d_b14, eq134_e1708_d_b15, eq134_e1708_d_b16, eq134_e1708_d_b17, eq134_e1708_d_b18, eq134_e1708_d_b19, eq134_e1708_d_b20, eq134_e1708_d_b21, eq134_e1708_d_b22, eq134_e1708_d_b23, eq134_e1708_d_b24, eq134_e1708_d_b25, eq134_e1708_d_b26, eq134_e1708_d_b27, eq134_e1708_d_b28, eq134_e1708_d_b29, eq134_e1708_d_b30, eq134_e1708_d_b31, eq134_e1708_d_b32, eq134_e1708_d_b33, eq134_e1708_d_b34, eq134_e1708_d_b35, eq134_e1708_d_b36, eq134_e1708_d_b37, eq134_e1708_d_b38, eq134_e1708_d_b39, eq134_e1708_d_b40, eq134_e1708_d_b41, eq134_e1708_d_b42, eq134_e1708_d_b43, eq134_e1708_d_b44, eq134_e1708_d_b45, eq134_e1708_d_b46, eq134_e1708_d_b47, eq134_e1708_d_b48, eq134_e1708_d_b49, eq134_e1708_d_b50, eq134_e1708_d_b51, eq134_e1708_d_b52, eq134_e1708_d_b53, eq134_e1708_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_value: f64 = eq134_e1710;
        let eq134_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_branch_derivatives: [f64; 55] = [eq134_e1710_d_b0, eq134_e1710_d_b1, eq134_e1710_d_b2, eq134_e1710_d_b3, eq134_e1710_d_b4, eq134_e1710_d_b5, eq134_e1710_d_b6, eq134_e1710_d_b7, eq134_e1710_d_b8, eq134_e1710_d_b9, eq134_e1710_d_b10, eq134_e1710_d_b11, eq134_e1710_d_b12, eq134_e1710_d_b13, eq134_e1710_d_b14, eq134_e1710_d_b15, eq134_e1710_d_b16, eq134_e1710_d_b17, eq134_e1710_d_b18, eq134_e1710_d_b19, eq134_e1710_d_b20, eq134_e1710_d_b21, eq134_e1710_d_b22, eq134_e1710_d_b23, eq134_e1710_d_b24, eq134_e1710_d_b25, eq134_e1710_d_b26, eq134_e1710_d_b27, eq134_e1710_d_b28, eq134_e1710_d_b29, eq134_e1710_d_b30, eq134_e1710_d_b31, eq134_e1710_d_b32, eq134_e1710_d_b33, eq134_e1710_d_b34, eq134_e1710_d_b35, eq134_e1710_d_b36, eq134_e1710_d_b37, eq134_e1710_d_b38, eq134_e1710_d_b39, eq134_e1710_d_b40, eq134_e1710_d_b41, eq134_e1710_d_b42, eq134_e1710_d_b43, eq134_e1710_d_b44, eq134_e1710_d_b45, eq134_e1710_d_b46, eq134_e1710_d_b47, eq134_e1710_d_b48, eq134_e1710_d_b49, eq134_e1710_d_b50, eq134_e1710_d_b51, eq134_e1710_d_b52, eq134_e1710_d_b53, eq134_e1710_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(19),
            multiplicity * (eq134_value),
            &eq134_node_derivatives,
            &eq134_branch_derivatives,
            multiplicity,
        );
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22, eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq135_e1719: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 34, s.v[240]);
        let eq135_e1720: f64 = (p.p7 * eq135_e1719);
        (eq135_e1720, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1722;
        let eq135_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_branch_derivatives: [f64; 55] = [eq135_e1722_d_b0, eq135_e1722_d_b1, eq135_e1722_d_b2, eq135_e1722_d_b3, eq135_e1722_d_b4, eq135_e1722_d_b5, eq135_e1722_d_b6, eq135_e1722_d_b7, eq135_e1722_d_b8, eq135_e1722_d_b9, eq135_e1722_d_b10, eq135_e1722_d_b11, eq135_e1722_d_b12, eq135_e1722_d_b13, eq135_e1722_d_b14, eq135_e1722_d_b15, eq135_e1722_d_b16, eq135_e1722_d_b17, eq135_e1722_d_b18, eq135_e1722_d_b19, eq135_e1722_d_b20, eq135_e1722_d_b21, eq135_e1722_d_b22, eq135_e1722_d_b23, eq135_e1722_d_b24, eq135_e1722_d_b25, eq135_e1722_d_b26, eq135_e1722_d_b27, eq135_e1722_d_b28, eq135_e1722_d_b29, eq135_e1722_d_b30, eq135_e1722_d_b31, eq135_e1722_d_b32, eq135_e1722_d_b33, eq135_e1722_d_b34, eq135_e1722_d_b35, eq135_e1722_d_b36, eq135_e1722_d_b37, eq135_e1722_d_b38, eq135_e1722_d_b39, eq135_e1722_d_b40, eq135_e1722_d_b41, eq135_e1722_d_b42, eq135_e1722_d_b43, eq135_e1722_d_b44, eq135_e1722_d_b45, eq135_e1722_d_b46, eq135_e1722_d_b47, eq135_e1722_d_b48, eq135_e1722_d_b49, eq135_e1722_d_b50, eq135_e1722_d_b51, eq135_e1722_d_b52, eq135_e1722_d_b53, eq135_e1722_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(19),
            multiplicity * (eq135_value),
            &eq135_node_derivatives,
            &eq135_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_20(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22, eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54,) = {
    if ((s.b[575] && s.b[576]) && (!s.b[577])) {
        let eq136_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 35, s.v[240]);
        let eq136_e1732: f64 = (p.p7 * eq136_e1731);
        let eq136_e1732_d_n0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let eq136_e1732_d_n1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let eq136_e1732_d_n2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let eq136_e1732_d_n3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let eq136_e1732_d_n4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let eq136_e1732_d_n5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let eq136_e1732_d_n6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let eq136_e1732_d_n7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let eq136_e1732_d_n8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let eq136_e1732_d_n9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let eq136_e1732_d_n10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let eq136_e1732_d_n11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let eq136_e1732_d_n12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let eq136_e1732_d_n13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let eq136_e1732_d_n14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let eq136_e1732_d_n15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let eq136_e1732_d_n16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let eq136_e1732_d_n17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let eq136_e1732_d_n18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let eq136_e1732_d_n19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let eq136_e1732_d_n20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let eq136_e1732_d_n21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let eq136_e1732_d_n22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let eq136_e1732_d_b0: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let eq136_e1732_d_b1: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let eq136_e1732_d_b2: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let eq136_e1732_d_b3: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let eq136_e1732_d_b4: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let eq136_e1732_d_b5: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let eq136_e1732_d_b6: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let eq136_e1732_d_b7: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let eq136_e1732_d_b8: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let eq136_e1732_d_b9: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let eq136_e1732_d_b10: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let eq136_e1732_d_b11: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let eq136_e1732_d_b12: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let eq136_e1732_d_b13: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let eq136_e1732_d_b14: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let eq136_e1732_d_b15: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let eq136_e1732_d_b16: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let eq136_e1732_d_b17: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let eq136_e1732_d_b18: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let eq136_e1732_d_b19: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let eq136_e1732_d_b20: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let eq136_e1732_d_b21: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let eq136_e1732_d_b22: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let eq136_e1732_d_b23: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let eq136_e1732_d_b24: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let eq136_e1732_d_b25: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let eq136_e1732_d_b26: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let eq136_e1732_d_b27: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let eq136_e1732_d_b28: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let eq136_e1732_d_b29: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let eq136_e1732_d_b30: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let eq136_e1732_d_b31: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let eq136_e1732_d_b32: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let eq136_e1732_d_b33: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let eq136_e1732_d_b34: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let eq136_e1732_d_b35: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let eq136_e1732_d_b36: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let eq136_e1732_d_b37: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let eq136_e1732_d_b38: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let eq136_e1732_d_b39: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let eq136_e1732_d_b40: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let eq136_e1732_d_b41: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let eq136_e1732_d_b42: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let eq136_e1732_d_b43: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let eq136_e1732_d_b44: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let eq136_e1732_d_b45: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let eq136_e1732_d_b46: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let eq136_e1732_d_b47: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let eq136_e1732_d_b48: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let eq136_e1732_d_b49: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let eq136_e1732_d_b50: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let eq136_e1732_d_b51: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let eq136_e1732_d_b52: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let eq136_e1732_d_b53: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let eq136_e1732_d_b54: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
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
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22, eq136_e1734_d_b0, eq136_e1734_d_b1, eq136_e1734_d_b2, eq136_e1734_d_b3, eq136_e1734_d_b4, eq136_e1734_d_b5, eq136_e1734_d_b6, eq136_e1734_d_b7, eq136_e1734_d_b8, eq136_e1734_d_b9, eq136_e1734_d_b10, eq136_e1734_d_b11, eq136_e1734_d_b12, eq136_e1734_d_b13, eq136_e1734_d_b14, eq136_e1734_d_b15, eq136_e1734_d_b16, eq136_e1734_d_b17, eq136_e1734_d_b18, eq136_e1734_d_b19, eq136_e1734_d_b20, eq136_e1734_d_b21, eq136_e1734_d_b22, eq136_e1734_d_b23, eq136_e1734_d_b24, eq136_e1734_d_b25, eq136_e1734_d_b26, eq136_e1734_d_b27, eq136_e1734_d_b28, eq136_e1734_d_b29, eq136_e1734_d_b30, eq136_e1734_d_b31, eq136_e1734_d_b32, eq136_e1734_d_b33, eq136_e1734_d_b34, eq136_e1734_d_b35, eq136_e1734_d_b36, eq136_e1734_d_b37, eq136_e1734_d_b38, eq136_e1734_d_b39, eq136_e1734_d_b40, eq136_e1734_d_b41, eq136_e1734_d_b42, eq136_e1734_d_b43, eq136_e1734_d_b44, eq136_e1734_d_b45, eq136_e1734_d_b46, eq136_e1734_d_b47, eq136_e1734_d_b48, eq136_e1734_d_b49, eq136_e1734_d_b50, eq136_e1734_d_b51, eq136_e1734_d_b52, eq136_e1734_d_b53, eq136_e1734_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_value: f64 = eq136_e1736;
        let eq136_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_branch_derivatives: [f64; 55] = [eq136_e1736_d_b0, eq136_e1736_d_b1, eq136_e1736_d_b2, eq136_e1736_d_b3, eq136_e1736_d_b4, eq136_e1736_d_b5, eq136_e1736_d_b6, eq136_e1736_d_b7, eq136_e1736_d_b8, eq136_e1736_d_b9, eq136_e1736_d_b10, eq136_e1736_d_b11, eq136_e1736_d_b12, eq136_e1736_d_b13, eq136_e1736_d_b14, eq136_e1736_d_b15, eq136_e1736_d_b16, eq136_e1736_d_b17, eq136_e1736_d_b18, eq136_e1736_d_b19, eq136_e1736_d_b20, eq136_e1736_d_b21, eq136_e1736_d_b22, eq136_e1736_d_b23, eq136_e1736_d_b24, eq136_e1736_d_b25, eq136_e1736_d_b26, eq136_e1736_d_b27, eq136_e1736_d_b28, eq136_e1736_d_b29, eq136_e1736_d_b30, eq136_e1736_d_b31, eq136_e1736_d_b32, eq136_e1736_d_b33, eq136_e1736_d_b34, eq136_e1736_d_b35, eq136_e1736_d_b36, eq136_e1736_d_b37, eq136_e1736_d_b38, eq136_e1736_d_b39, eq136_e1736_d_b40, eq136_e1736_d_b41, eq136_e1736_d_b42, eq136_e1736_d_b43, eq136_e1736_d_b44, eq136_e1736_d_b45, eq136_e1736_d_b46, eq136_e1736_d_b47, eq136_e1736_d_b48, eq136_e1736_d_b49, eq136_e1736_d_b50, eq136_e1736_d_b51, eq136_e1736_d_b52, eq136_e1736_d_b53, eq136_e1736_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(19),
            multiplicity * (eq136_value),
            &eq136_node_derivatives,
            &eq136_branch_derivatives,
            multiplicity,
        );
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22, eq137_e1747_d_b0, eq137_e1747_d_b1, eq137_e1747_d_b2, eq137_e1747_d_b3, eq137_e1747_d_b4, eq137_e1747_d_b5, eq137_e1747_d_b6, eq137_e1747_d_b7, eq137_e1747_d_b8, eq137_e1747_d_b9, eq137_e1747_d_b10, eq137_e1747_d_b11, eq137_e1747_d_b12, eq137_e1747_d_b13, eq137_e1747_d_b14, eq137_e1747_d_b15, eq137_e1747_d_b16, eq137_e1747_d_b17, eq137_e1747_d_b18, eq137_e1747_d_b19, eq137_e1747_d_b20, eq137_e1747_d_b21, eq137_e1747_d_b22, eq137_e1747_d_b23, eq137_e1747_d_b24, eq137_e1747_d_b25, eq137_e1747_d_b26, eq137_e1747_d_b27, eq137_e1747_d_b28, eq137_e1747_d_b29, eq137_e1747_d_b30, eq137_e1747_d_b31, eq137_e1747_d_b32, eq137_e1747_d_b33, eq137_e1747_d_b34, eq137_e1747_d_b35, eq137_e1747_d_b36, eq137_e1747_d_b37, eq137_e1747_d_b38, eq137_e1747_d_b39, eq137_e1747_d_b40, eq137_e1747_d_b41, eq137_e1747_d_b42, eq137_e1747_d_b43, eq137_e1747_d_b44, eq137_e1747_d_b45, eq137_e1747_d_b46, eq137_e1747_d_b47, eq137_e1747_d_b48, eq137_e1747_d_b49, eq137_e1747_d_b50, eq137_e1747_d_b51, eq137_e1747_d_b52, eq137_e1747_d_b53, eq137_e1747_d_b54,) = {
    if (s.b[575] && s.b[576]) {
        let eq137_e1743: f64 = (p.p251 * s.v[240]);
        let eq137_e1744: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 36, eq137_e1743);
        let eq137_e1744_d_n0: f64 = ((p.p251 * s.dn[240][0]) * ddt_scale);
        let eq137_e1744_d_n1: f64 = ((p.p251 * s.dn[240][1]) * ddt_scale);
        let eq137_e1744_d_n2: f64 = ((p.p251 * s.dn[240][2]) * ddt_scale);
        let eq137_e1744_d_n3: f64 = ((p.p251 * s.dn[240][3]) * ddt_scale);
        let eq137_e1744_d_n4: f64 = ((p.p251 * s.dn[240][4]) * ddt_scale);
        let eq137_e1744_d_n5: f64 = ((p.p251 * s.dn[240][5]) * ddt_scale);
        let eq137_e1744_d_n6: f64 = ((p.p251 * s.dn[240][6]) * ddt_scale);
        let eq137_e1744_d_n7: f64 = ((p.p251 * s.dn[240][7]) * ddt_scale);
        let eq137_e1744_d_n8: f64 = ((p.p251 * s.dn[240][8]) * ddt_scale);
        let eq137_e1744_d_n9: f64 = ((p.p251 * s.dn[240][9]) * ddt_scale);
        let eq137_e1744_d_n10: f64 = ((p.p251 * s.dn[240][10]) * ddt_scale);
        let eq137_e1744_d_n11: f64 = ((p.p251 * s.dn[240][11]) * ddt_scale);
        let eq137_e1744_d_n12: f64 = ((p.p251 * s.dn[240][12]) * ddt_scale);
        let eq137_e1744_d_n13: f64 = ((p.p251 * s.dn[240][13]) * ddt_scale);
        let eq137_e1744_d_n14: f64 = ((p.p251 * s.dn[240][14]) * ddt_scale);
        let eq137_e1744_d_n15: f64 = ((p.p251 * s.dn[240][15]) * ddt_scale);
        let eq137_e1744_d_n16: f64 = ((p.p251 * s.dn[240][16]) * ddt_scale);
        let eq137_e1744_d_n17: f64 = ((p.p251 * s.dn[240][17]) * ddt_scale);
        let eq137_e1744_d_n18: f64 = ((p.p251 * s.dn[240][18]) * ddt_scale);
        let eq137_e1744_d_n19: f64 = ((p.p251 * s.dn[240][19]) * ddt_scale);
        let eq137_e1744_d_n20: f64 = ((p.p251 * s.dn[240][20]) * ddt_scale);
        let eq137_e1744_d_n21: f64 = ((p.p251 * s.dn[240][21]) * ddt_scale);
        let eq137_e1744_d_n22: f64 = ((p.p251 * s.dn[240][22]) * ddt_scale);
        let eq137_e1744_d_b0: f64 = ((p.p251 * s.db[240][0]) * ddt_scale);
        let eq137_e1744_d_b1: f64 = ((p.p251 * s.db[240][1]) * ddt_scale);
        let eq137_e1744_d_b2: f64 = ((p.p251 * s.db[240][2]) * ddt_scale);
        let eq137_e1744_d_b3: f64 = ((p.p251 * s.db[240][3]) * ddt_scale);
        let eq137_e1744_d_b4: f64 = ((p.p251 * s.db[240][4]) * ddt_scale);
        let eq137_e1744_d_b5: f64 = ((p.p251 * s.db[240][5]) * ddt_scale);
        let eq137_e1744_d_b6: f64 = ((p.p251 * s.db[240][6]) * ddt_scale);
        let eq137_e1744_d_b7: f64 = ((p.p251 * s.db[240][7]) * ddt_scale);
        let eq137_e1744_d_b8: f64 = ((p.p251 * s.db[240][8]) * ddt_scale);
        let eq137_e1744_d_b9: f64 = ((p.p251 * s.db[240][9]) * ddt_scale);
        let eq137_e1744_d_b10: f64 = ((p.p251 * s.db[240][10]) * ddt_scale);
        let eq137_e1744_d_b11: f64 = ((p.p251 * s.db[240][11]) * ddt_scale);
        let eq137_e1744_d_b12: f64 = ((p.p251 * s.db[240][12]) * ddt_scale);
        let eq137_e1744_d_b13: f64 = ((p.p251 * s.db[240][13]) * ddt_scale);
        let eq137_e1744_d_b14: f64 = ((p.p251 * s.db[240][14]) * ddt_scale);
        let eq137_e1744_d_b15: f64 = ((p.p251 * s.db[240][15]) * ddt_scale);
        let eq137_e1744_d_b16: f64 = ((p.p251 * s.db[240][16]) * ddt_scale);
        let eq137_e1744_d_b17: f64 = ((p.p251 * s.db[240][17]) * ddt_scale);
        let eq137_e1744_d_b18: f64 = ((p.p251 * s.db[240][18]) * ddt_scale);
        let eq137_e1744_d_b19: f64 = ((p.p251 * s.db[240][19]) * ddt_scale);
        let eq137_e1744_d_b20: f64 = ((p.p251 * s.db[240][20]) * ddt_scale);
        let eq137_e1744_d_b21: f64 = ((p.p251 * s.db[240][21]) * ddt_scale);
        let eq137_e1744_d_b22: f64 = ((p.p251 * s.db[240][22]) * ddt_scale);
        let eq137_e1744_d_b23: f64 = ((p.p251 * s.db[240][23]) * ddt_scale);
        let eq137_e1744_d_b24: f64 = ((p.p251 * s.db[240][24]) * ddt_scale);
        let eq137_e1744_d_b25: f64 = ((p.p251 * s.db[240][25]) * ddt_scale);
        let eq137_e1744_d_b26: f64 = ((p.p251 * s.db[240][26]) * ddt_scale);
        let eq137_e1744_d_b27: f64 = ((p.p251 * s.db[240][27]) * ddt_scale);
        let eq137_e1744_d_b28: f64 = ((p.p251 * s.db[240][28]) * ddt_scale);
        let eq137_e1744_d_b29: f64 = ((p.p251 * s.db[240][29]) * ddt_scale);
        let eq137_e1744_d_b30: f64 = ((p.p251 * s.db[240][30]) * ddt_scale);
        let eq137_e1744_d_b31: f64 = ((p.p251 * s.db[240][31]) * ddt_scale);
        let eq137_e1744_d_b32: f64 = ((p.p251 * s.db[240][32]) * ddt_scale);
        let eq137_e1744_d_b33: f64 = ((p.p251 * s.db[240][33]) * ddt_scale);
        let eq137_e1744_d_b34: f64 = ((p.p251 * s.db[240][34]) * ddt_scale);
        let eq137_e1744_d_b35: f64 = ((p.p251 * s.db[240][35]) * ddt_scale);
        let eq137_e1744_d_b36: f64 = ((p.p251 * s.db[240][36]) * ddt_scale);
        let eq137_e1744_d_b37: f64 = ((p.p251 * s.db[240][37]) * ddt_scale);
        let eq137_e1744_d_b38: f64 = ((p.p251 * s.db[240][38]) * ddt_scale);
        let eq137_e1744_d_b39: f64 = ((p.p251 * s.db[240][39]) * ddt_scale);
        let eq137_e1744_d_b40: f64 = ((p.p251 * s.db[240][40]) * ddt_scale);
        let eq137_e1744_d_b41: f64 = ((p.p251 * s.db[240][41]) * ddt_scale);
        let eq137_e1744_d_b42: f64 = ((p.p251 * s.db[240][42]) * ddt_scale);
        let eq137_e1744_d_b43: f64 = ((p.p251 * s.db[240][43]) * ddt_scale);
        let eq137_e1744_d_b44: f64 = ((p.p251 * s.db[240][44]) * ddt_scale);
        let eq137_e1744_d_b45: f64 = ((p.p251 * s.db[240][45]) * ddt_scale);
        let eq137_e1744_d_b46: f64 = ((p.p251 * s.db[240][46]) * ddt_scale);
        let eq137_e1744_d_b47: f64 = ((p.p251 * s.db[240][47]) * ddt_scale);
        let eq137_e1744_d_b48: f64 = ((p.p251 * s.db[240][48]) * ddt_scale);
        let eq137_e1744_d_b49: f64 = ((p.p251 * s.db[240][49]) * ddt_scale);
        let eq137_e1744_d_b50: f64 = ((p.p251 * s.db[240][50]) * ddt_scale);
        let eq137_e1744_d_b51: f64 = ((p.p251 * s.db[240][51]) * ddt_scale);
        let eq137_e1744_d_b52: f64 = ((p.p251 * s.db[240][52]) * ddt_scale);
        let eq137_e1744_d_b53: f64 = ((p.p251 * s.db[240][53]) * ddt_scale);
        let eq137_e1744_d_b54: f64 = ((p.p251 * s.db[240][54]) * ddt_scale);
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
        let eq137_e1745_d_b0: f64 = (p.p7 * eq137_e1744_d_b0);
        let eq137_e1745_d_b1: f64 = (p.p7 * eq137_e1744_d_b1);
        let eq137_e1745_d_b2: f64 = (p.p7 * eq137_e1744_d_b2);
        let eq137_e1745_d_b3: f64 = (p.p7 * eq137_e1744_d_b3);
        let eq137_e1745_d_b4: f64 = (p.p7 * eq137_e1744_d_b4);
        let eq137_e1745_d_b5: f64 = (p.p7 * eq137_e1744_d_b5);
        let eq137_e1745_d_b6: f64 = (p.p7 * eq137_e1744_d_b6);
        let eq137_e1745_d_b7: f64 = (p.p7 * eq137_e1744_d_b7);
        let eq137_e1745_d_b8: f64 = (p.p7 * eq137_e1744_d_b8);
        let eq137_e1745_d_b9: f64 = (p.p7 * eq137_e1744_d_b9);
        let eq137_e1745_d_b10: f64 = (p.p7 * eq137_e1744_d_b10);
        let eq137_e1745_d_b11: f64 = (p.p7 * eq137_e1744_d_b11);
        let eq137_e1745_d_b12: f64 = (p.p7 * eq137_e1744_d_b12);
        let eq137_e1745_d_b13: f64 = (p.p7 * eq137_e1744_d_b13);
        let eq137_e1745_d_b14: f64 = (p.p7 * eq137_e1744_d_b14);
        let eq137_e1745_d_b15: f64 = (p.p7 * eq137_e1744_d_b15);
        let eq137_e1745_d_b16: f64 = (p.p7 * eq137_e1744_d_b16);
        let eq137_e1745_d_b17: f64 = (p.p7 * eq137_e1744_d_b17);
        let eq137_e1745_d_b18: f64 = (p.p7 * eq137_e1744_d_b18);
        let eq137_e1745_d_b19: f64 = (p.p7 * eq137_e1744_d_b19);
        let eq137_e1745_d_b20: f64 = (p.p7 * eq137_e1744_d_b20);
        let eq137_e1745_d_b21: f64 = (p.p7 * eq137_e1744_d_b21);
        let eq137_e1745_d_b22: f64 = (p.p7 * eq137_e1744_d_b22);
        let eq137_e1745_d_b23: f64 = (p.p7 * eq137_e1744_d_b23);
        let eq137_e1745_d_b24: f64 = (p.p7 * eq137_e1744_d_b24);
        let eq137_e1745_d_b25: f64 = (p.p7 * eq137_e1744_d_b25);
        let eq137_e1745_d_b26: f64 = (p.p7 * eq137_e1744_d_b26);
        let eq137_e1745_d_b27: f64 = (p.p7 * eq137_e1744_d_b27);
        let eq137_e1745_d_b28: f64 = (p.p7 * eq137_e1744_d_b28);
        let eq137_e1745_d_b29: f64 = (p.p7 * eq137_e1744_d_b29);
        let eq137_e1745_d_b30: f64 = (p.p7 * eq137_e1744_d_b30);
        let eq137_e1745_d_b31: f64 = (p.p7 * eq137_e1744_d_b31);
        let eq137_e1745_d_b32: f64 = (p.p7 * eq137_e1744_d_b32);
        let eq137_e1745_d_b33: f64 = (p.p7 * eq137_e1744_d_b33);
        let eq137_e1745_d_b34: f64 = (p.p7 * eq137_e1744_d_b34);
        let eq137_e1745_d_b35: f64 = (p.p7 * eq137_e1744_d_b35);
        let eq137_e1745_d_b36: f64 = (p.p7 * eq137_e1744_d_b36);
        let eq137_e1745_d_b37: f64 = (p.p7 * eq137_e1744_d_b37);
        let eq137_e1745_d_b38: f64 = (p.p7 * eq137_e1744_d_b38);
        let eq137_e1745_d_b39: f64 = (p.p7 * eq137_e1744_d_b39);
        let eq137_e1745_d_b40: f64 = (p.p7 * eq137_e1744_d_b40);
        let eq137_e1745_d_b41: f64 = (p.p7 * eq137_e1744_d_b41);
        let eq137_e1745_d_b42: f64 = (p.p7 * eq137_e1744_d_b42);
        let eq137_e1745_d_b43: f64 = (p.p7 * eq137_e1744_d_b43);
        let eq137_e1745_d_b44: f64 = (p.p7 * eq137_e1744_d_b44);
        let eq137_e1745_d_b45: f64 = (p.p7 * eq137_e1744_d_b45);
        let eq137_e1745_d_b46: f64 = (p.p7 * eq137_e1744_d_b46);
        let eq137_e1745_d_b47: f64 = (p.p7 * eq137_e1744_d_b47);
        let eq137_e1745_d_b48: f64 = (p.p7 * eq137_e1744_d_b48);
        let eq137_e1745_d_b49: f64 = (p.p7 * eq137_e1744_d_b49);
        let eq137_e1745_d_b50: f64 = (p.p7 * eq137_e1744_d_b50);
        let eq137_e1745_d_b51: f64 = (p.p7 * eq137_e1744_d_b51);
        let eq137_e1745_d_b52: f64 = (p.p7 * eq137_e1744_d_b52);
        let eq137_e1745_d_b53: f64 = (p.p7 * eq137_e1744_d_b53);
        let eq137_e1745_d_b54: f64 = (p.p7 * eq137_e1744_d_b54);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22, eq137_e1745_d_b0, eq137_e1745_d_b1, eq137_e1745_d_b2, eq137_e1745_d_b3, eq137_e1745_d_b4, eq137_e1745_d_b5, eq137_e1745_d_b6, eq137_e1745_d_b7, eq137_e1745_d_b8, eq137_e1745_d_b9, eq137_e1745_d_b10, eq137_e1745_d_b11, eq137_e1745_d_b12, eq137_e1745_d_b13, eq137_e1745_d_b14, eq137_e1745_d_b15, eq137_e1745_d_b16, eq137_e1745_d_b17, eq137_e1745_d_b18, eq137_e1745_d_b19, eq137_e1745_d_b20, eq137_e1745_d_b21, eq137_e1745_d_b22, eq137_e1745_d_b23, eq137_e1745_d_b24, eq137_e1745_d_b25, eq137_e1745_d_b26, eq137_e1745_d_b27, eq137_e1745_d_b28, eq137_e1745_d_b29, eq137_e1745_d_b30, eq137_e1745_d_b31, eq137_e1745_d_b32, eq137_e1745_d_b33, eq137_e1745_d_b34, eq137_e1745_d_b35, eq137_e1745_d_b36, eq137_e1745_d_b37, eq137_e1745_d_b38, eq137_e1745_d_b39, eq137_e1745_d_b40, eq137_e1745_d_b41, eq137_e1745_d_b42, eq137_e1745_d_b43, eq137_e1745_d_b44, eq137_e1745_d_b45, eq137_e1745_d_b46, eq137_e1745_d_b47, eq137_e1745_d_b48, eq137_e1745_d_b49, eq137_e1745_d_b50, eq137_e1745_d_b51, eq137_e1745_d_b52, eq137_e1745_d_b53, eq137_e1745_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1747;
        let eq137_node_derivatives: [f64; 23] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_branch_derivatives: [f64; 55] = [eq137_e1747_d_b0, eq137_e1747_d_b1, eq137_e1747_d_b2, eq137_e1747_d_b3, eq137_e1747_d_b4, eq137_e1747_d_b5, eq137_e1747_d_b6, eq137_e1747_d_b7, eq137_e1747_d_b8, eq137_e1747_d_b9, eq137_e1747_d_b10, eq137_e1747_d_b11, eq137_e1747_d_b12, eq137_e1747_d_b13, eq137_e1747_d_b14, eq137_e1747_d_b15, eq137_e1747_d_b16, eq137_e1747_d_b17, eq137_e1747_d_b18, eq137_e1747_d_b19, eq137_e1747_d_b20, eq137_e1747_d_b21, eq137_e1747_d_b22, eq137_e1747_d_b23, eq137_e1747_d_b24, eq137_e1747_d_b25, eq137_e1747_d_b26, eq137_e1747_d_b27, eq137_e1747_d_b28, eq137_e1747_d_b29, eq137_e1747_d_b30, eq137_e1747_d_b31, eq137_e1747_d_b32, eq137_e1747_d_b33, eq137_e1747_d_b34, eq137_e1747_d_b35, eq137_e1747_d_b36, eq137_e1747_d_b37, eq137_e1747_d_b38, eq137_e1747_d_b39, eq137_e1747_d_b40, eq137_e1747_d_b41, eq137_e1747_d_b42, eq137_e1747_d_b43, eq137_e1747_d_b44, eq137_e1747_d_b45, eq137_e1747_d_b46, eq137_e1747_d_b47, eq137_e1747_d_b48, eq137_e1747_d_b49, eq137_e1747_d_b50, eq137_e1747_d_b51, eq137_e1747_d_b52, eq137_e1747_d_b53, eq137_e1747_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(19),
            multiplicity * (eq137_value),
            &eq137_node_derivatives,
            &eq137_branch_derivatives,
            multiplicity,
        );
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22, eq138_e1757_d_b0, eq138_e1757_d_b1, eq138_e1757_d_b2, eq138_e1757_d_b3, eq138_e1757_d_b4, eq138_e1757_d_b5, eq138_e1757_d_b6, eq138_e1757_d_b7, eq138_e1757_d_b8, eq138_e1757_d_b9, eq138_e1757_d_b10, eq138_e1757_d_b11, eq138_e1757_d_b12, eq138_e1757_d_b13, eq138_e1757_d_b14, eq138_e1757_d_b15, eq138_e1757_d_b16, eq138_e1757_d_b17, eq138_e1757_d_b18, eq138_e1757_d_b19, eq138_e1757_d_b20, eq138_e1757_d_b21, eq138_e1757_d_b22, eq138_e1757_d_b23, eq138_e1757_d_b24, eq138_e1757_d_b25, eq138_e1757_d_b26, eq138_e1757_d_b27, eq138_e1757_d_b28, eq138_e1757_d_b29, eq138_e1757_d_b30, eq138_e1757_d_b31, eq138_e1757_d_b32, eq138_e1757_d_b33, eq138_e1757_d_b34, eq138_e1757_d_b35, eq138_e1757_d_b36, eq138_e1757_d_b37, eq138_e1757_d_b38, eq138_e1757_d_b39, eq138_e1757_d_b40, eq138_e1757_d_b41, eq138_e1757_d_b42, eq138_e1757_d_b43, eq138_e1757_d_b44, eq138_e1757_d_b45, eq138_e1757_d_b46, eq138_e1757_d_b47, eq138_e1757_d_b48, eq138_e1757_d_b49, eq138_e1757_d_b50, eq138_e1757_d_b51, eq138_e1757_d_b52, eq138_e1757_d_b53, eq138_e1757_d_b54,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq138_e1754: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 37, s.v[241]);
        let eq138_e1755: f64 = (p.p7 * eq138_e1754);
        let eq138_e1755_d_n0: f64 = (p.p7 * (s.dn[241][0] * ddt_scale));
        let eq138_e1755_d_n1: f64 = (p.p7 * (s.dn[241][1] * ddt_scale));
        let eq138_e1755_d_n2: f64 = (p.p7 * (s.dn[241][2] * ddt_scale));
        let eq138_e1755_d_n3: f64 = (p.p7 * (s.dn[241][3] * ddt_scale));
        let eq138_e1755_d_n4: f64 = (p.p7 * (s.dn[241][4] * ddt_scale));
        let eq138_e1755_d_n5: f64 = (p.p7 * (s.dn[241][5] * ddt_scale));
        let eq138_e1755_d_n6: f64 = (p.p7 * (s.dn[241][6] * ddt_scale));
        let eq138_e1755_d_n7: f64 = (p.p7 * (s.dn[241][7] * ddt_scale));
        let eq138_e1755_d_n8: f64 = (p.p7 * (s.dn[241][8] * ddt_scale));
        let eq138_e1755_d_n9: f64 = (p.p7 * (s.dn[241][9] * ddt_scale));
        let eq138_e1755_d_n10: f64 = (p.p7 * (s.dn[241][10] * ddt_scale));
        let eq138_e1755_d_n11: f64 = (p.p7 * (s.dn[241][11] * ddt_scale));
        let eq138_e1755_d_n12: f64 = (p.p7 * (s.dn[241][12] * ddt_scale));
        let eq138_e1755_d_n13: f64 = (p.p7 * (s.dn[241][13] * ddt_scale));
        let eq138_e1755_d_n14: f64 = (p.p7 * (s.dn[241][14] * ddt_scale));
        let eq138_e1755_d_n15: f64 = (p.p7 * (s.dn[241][15] * ddt_scale));
        let eq138_e1755_d_n16: f64 = (p.p7 * (s.dn[241][16] * ddt_scale));
        let eq138_e1755_d_n17: f64 = (p.p7 * (s.dn[241][17] * ddt_scale));
        let eq138_e1755_d_n18: f64 = (p.p7 * (s.dn[241][18] * ddt_scale));
        let eq138_e1755_d_n19: f64 = (p.p7 * (s.dn[241][19] * ddt_scale));
        let eq138_e1755_d_n20: f64 = (p.p7 * (s.dn[241][20] * ddt_scale));
        let eq138_e1755_d_n21: f64 = (p.p7 * (s.dn[241][21] * ddt_scale));
        let eq138_e1755_d_n22: f64 = (p.p7 * (s.dn[241][22] * ddt_scale));
        let eq138_e1755_d_b0: f64 = (p.p7 * (s.db[241][0] * ddt_scale));
        let eq138_e1755_d_b1: f64 = (p.p7 * (s.db[241][1] * ddt_scale));
        let eq138_e1755_d_b2: f64 = (p.p7 * (s.db[241][2] * ddt_scale));
        let eq138_e1755_d_b3: f64 = (p.p7 * (s.db[241][3] * ddt_scale));
        let eq138_e1755_d_b4: f64 = (p.p7 * (s.db[241][4] * ddt_scale));
        let eq138_e1755_d_b5: f64 = (p.p7 * (s.db[241][5] * ddt_scale));
        let eq138_e1755_d_b6: f64 = (p.p7 * (s.db[241][6] * ddt_scale));
        let eq138_e1755_d_b7: f64 = (p.p7 * (s.db[241][7] * ddt_scale));
        let eq138_e1755_d_b8: f64 = (p.p7 * (s.db[241][8] * ddt_scale));
        let eq138_e1755_d_b9: f64 = (p.p7 * (s.db[241][9] * ddt_scale));
        let eq138_e1755_d_b10: f64 = (p.p7 * (s.db[241][10] * ddt_scale));
        let eq138_e1755_d_b11: f64 = (p.p7 * (s.db[241][11] * ddt_scale));
        let eq138_e1755_d_b12: f64 = (p.p7 * (s.db[241][12] * ddt_scale));
        let eq138_e1755_d_b13: f64 = (p.p7 * (s.db[241][13] * ddt_scale));
        let eq138_e1755_d_b14: f64 = (p.p7 * (s.db[241][14] * ddt_scale));
        let eq138_e1755_d_b15: f64 = (p.p7 * (s.db[241][15] * ddt_scale));
        let eq138_e1755_d_b16: f64 = (p.p7 * (s.db[241][16] * ddt_scale));
        let eq138_e1755_d_b17: f64 = (p.p7 * (s.db[241][17] * ddt_scale));
        let eq138_e1755_d_b18: f64 = (p.p7 * (s.db[241][18] * ddt_scale));
        let eq138_e1755_d_b19: f64 = (p.p7 * (s.db[241][19] * ddt_scale));
        let eq138_e1755_d_b20: f64 = (p.p7 * (s.db[241][20] * ddt_scale));
        let eq138_e1755_d_b21: f64 = (p.p7 * (s.db[241][21] * ddt_scale));
        let eq138_e1755_d_b22: f64 = (p.p7 * (s.db[241][22] * ddt_scale));
        let eq138_e1755_d_b23: f64 = (p.p7 * (s.db[241][23] * ddt_scale));
        let eq138_e1755_d_b24: f64 = (p.p7 * (s.db[241][24] * ddt_scale));
        let eq138_e1755_d_b25: f64 = (p.p7 * (s.db[241][25] * ddt_scale));
        let eq138_e1755_d_b26: f64 = (p.p7 * (s.db[241][26] * ddt_scale));
        let eq138_e1755_d_b27: f64 = (p.p7 * (s.db[241][27] * ddt_scale));
        let eq138_e1755_d_b28: f64 = (p.p7 * (s.db[241][28] * ddt_scale));
        let eq138_e1755_d_b29: f64 = (p.p7 * (s.db[241][29] * ddt_scale));
        let eq138_e1755_d_b30: f64 = (p.p7 * (s.db[241][30] * ddt_scale));
        let eq138_e1755_d_b31: f64 = (p.p7 * (s.db[241][31] * ddt_scale));
        let eq138_e1755_d_b32: f64 = (p.p7 * (s.db[241][32] * ddt_scale));
        let eq138_e1755_d_b33: f64 = (p.p7 * (s.db[241][33] * ddt_scale));
        let eq138_e1755_d_b34: f64 = (p.p7 * (s.db[241][34] * ddt_scale));
        let eq138_e1755_d_b35: f64 = (p.p7 * (s.db[241][35] * ddt_scale));
        let eq138_e1755_d_b36: f64 = (p.p7 * (s.db[241][36] * ddt_scale));
        let eq138_e1755_d_b37: f64 = (p.p7 * (s.db[241][37] * ddt_scale));
        let eq138_e1755_d_b38: f64 = (p.p7 * (s.db[241][38] * ddt_scale));
        let eq138_e1755_d_b39: f64 = (p.p7 * (s.db[241][39] * ddt_scale));
        let eq138_e1755_d_b40: f64 = (p.p7 * (s.db[241][40] * ddt_scale));
        let eq138_e1755_d_b41: f64 = (p.p7 * (s.db[241][41] * ddt_scale));
        let eq138_e1755_d_b42: f64 = (p.p7 * (s.db[241][42] * ddt_scale));
        let eq138_e1755_d_b43: f64 = (p.p7 * (s.db[241][43] * ddt_scale));
        let eq138_e1755_d_b44: f64 = (p.p7 * (s.db[241][44] * ddt_scale));
        let eq138_e1755_d_b45: f64 = (p.p7 * (s.db[241][45] * ddt_scale));
        let eq138_e1755_d_b46: f64 = (p.p7 * (s.db[241][46] * ddt_scale));
        let eq138_e1755_d_b47: f64 = (p.p7 * (s.db[241][47] * ddt_scale));
        let eq138_e1755_d_b48: f64 = (p.p7 * (s.db[241][48] * ddt_scale));
        let eq138_e1755_d_b49: f64 = (p.p7 * (s.db[241][49] * ddt_scale));
        let eq138_e1755_d_b50: f64 = (p.p7 * (s.db[241][50] * ddt_scale));
        let eq138_e1755_d_b51: f64 = (p.p7 * (s.db[241][51] * ddt_scale));
        let eq138_e1755_d_b52: f64 = (p.p7 * (s.db[241][52] * ddt_scale));
        let eq138_e1755_d_b53: f64 = (p.p7 * (s.db[241][53] * ddt_scale));
        let eq138_e1755_d_b54: f64 = (p.p7 * (s.db[241][54] * ddt_scale));
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22, eq138_e1755_d_b0, eq138_e1755_d_b1, eq138_e1755_d_b2, eq138_e1755_d_b3, eq138_e1755_d_b4, eq138_e1755_d_b5, eq138_e1755_d_b6, eq138_e1755_d_b7, eq138_e1755_d_b8, eq138_e1755_d_b9, eq138_e1755_d_b10, eq138_e1755_d_b11, eq138_e1755_d_b12, eq138_e1755_d_b13, eq138_e1755_d_b14, eq138_e1755_d_b15, eq138_e1755_d_b16, eq138_e1755_d_b17, eq138_e1755_d_b18, eq138_e1755_d_b19, eq138_e1755_d_b20, eq138_e1755_d_b21, eq138_e1755_d_b22, eq138_e1755_d_b23, eq138_e1755_d_b24, eq138_e1755_d_b25, eq138_e1755_d_b26, eq138_e1755_d_b27, eq138_e1755_d_b28, eq138_e1755_d_b29, eq138_e1755_d_b30, eq138_e1755_d_b31, eq138_e1755_d_b32, eq138_e1755_d_b33, eq138_e1755_d_b34, eq138_e1755_d_b35, eq138_e1755_d_b36, eq138_e1755_d_b37, eq138_e1755_d_b38, eq138_e1755_d_b39, eq138_e1755_d_b40, eq138_e1755_d_b41, eq138_e1755_d_b42, eq138_e1755_d_b43, eq138_e1755_d_b44, eq138_e1755_d_b45, eq138_e1755_d_b46, eq138_e1755_d_b47, eq138_e1755_d_b48, eq138_e1755_d_b49, eq138_e1755_d_b50, eq138_e1755_d_b51, eq138_e1755_d_b52, eq138_e1755_d_b53, eq138_e1755_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_value: f64 = eq138_e1757;
        let eq138_node_derivatives: [f64; 23] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_branch_derivatives: [f64; 55] = [eq138_e1757_d_b0, eq138_e1757_d_b1, eq138_e1757_d_b2, eq138_e1757_d_b3, eq138_e1757_d_b4, eq138_e1757_d_b5, eq138_e1757_d_b6, eq138_e1757_d_b7, eq138_e1757_d_b8, eq138_e1757_d_b9, eq138_e1757_d_b10, eq138_e1757_d_b11, eq138_e1757_d_b12, eq138_e1757_d_b13, eq138_e1757_d_b14, eq138_e1757_d_b15, eq138_e1757_d_b16, eq138_e1757_d_b17, eq138_e1757_d_b18, eq138_e1757_d_b19, eq138_e1757_d_b20, eq138_e1757_d_b21, eq138_e1757_d_b22, eq138_e1757_d_b23, eq138_e1757_d_b24, eq138_e1757_d_b25, eq138_e1757_d_b26, eq138_e1757_d_b27, eq138_e1757_d_b28, eq138_e1757_d_b29, eq138_e1757_d_b30, eq138_e1757_d_b31, eq138_e1757_d_b32, eq138_e1757_d_b33, eq138_e1757_d_b34, eq138_e1757_d_b35, eq138_e1757_d_b36, eq138_e1757_d_b37, eq138_e1757_d_b38, eq138_e1757_d_b39, eq138_e1757_d_b40, eq138_e1757_d_b41, eq138_e1757_d_b42, eq138_e1757_d_b43, eq138_e1757_d_b44, eq138_e1757_d_b45, eq138_e1757_d_b46, eq138_e1757_d_b47, eq138_e1757_d_b48, eq138_e1757_d_b49, eq138_e1757_d_b50, eq138_e1757_d_b51, eq138_e1757_d_b52, eq138_e1757_d_b53, eq138_e1757_d_b54];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq138_value),
            &eq138_node_derivatives,
            &eq138_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_21(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let __rspice_deriv_cse_1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let __rspice_deriv_cse_2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let __rspice_deriv_cse_3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let __rspice_deriv_cse_4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let __rspice_deriv_cse_5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let __rspice_deriv_cse_6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let __rspice_deriv_cse_7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let __rspice_deriv_cse_8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let __rspice_deriv_cse_9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let __rspice_deriv_cse_10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let __rspice_deriv_cse_11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let __rspice_deriv_cse_12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let __rspice_deriv_cse_13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let __rspice_deriv_cse_14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let __rspice_deriv_cse_15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let __rspice_deriv_cse_16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let __rspice_deriv_cse_17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let __rspice_deriv_cse_18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let __rspice_deriv_cse_19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let __rspice_deriv_cse_20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let __rspice_deriv_cse_21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let __rspice_deriv_cse_22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let __rspice_deriv_cse_23: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let __rspice_deriv_cse_24: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let __rspice_deriv_cse_25: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let __rspice_deriv_cse_26: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let __rspice_deriv_cse_27: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let __rspice_deriv_cse_28: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let __rspice_deriv_cse_29: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let __rspice_deriv_cse_30: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let __rspice_deriv_cse_31: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let __rspice_deriv_cse_32: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let __rspice_deriv_cse_33: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let __rspice_deriv_cse_34: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let __rspice_deriv_cse_35: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let __rspice_deriv_cse_36: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let __rspice_deriv_cse_37: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let __rspice_deriv_cse_38: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let __rspice_deriv_cse_39: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let __rspice_deriv_cse_40: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let __rspice_deriv_cse_41: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let __rspice_deriv_cse_42: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let __rspice_deriv_cse_43: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let __rspice_deriv_cse_44: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let __rspice_deriv_cse_45: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let __rspice_deriv_cse_46: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let __rspice_deriv_cse_47: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let __rspice_deriv_cse_48: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let __rspice_deriv_cse_49: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let __rspice_deriv_cse_50: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let __rspice_deriv_cse_51: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let __rspice_deriv_cse_52: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let __rspice_deriv_cse_53: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let __rspice_deriv_cse_54: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let __rspice_deriv_cse_55: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let __rspice_deriv_cse_56: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let __rspice_deriv_cse_57: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let __rspice_deriv_cse_58: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let __rspice_deriv_cse_59: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let __rspice_deriv_cse_60: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let __rspice_deriv_cse_61: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let __rspice_deriv_cse_62: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let __rspice_deriv_cse_63: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let __rspice_deriv_cse_64: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let __rspice_deriv_cse_65: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let __rspice_deriv_cse_66: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let __rspice_deriv_cse_67: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let __rspice_deriv_cse_68: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let __rspice_deriv_cse_69: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let __rspice_deriv_cse_70: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let __rspice_deriv_cse_71: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let __rspice_deriv_cse_72: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let __rspice_deriv_cse_73: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let __rspice_deriv_cse_74: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let __rspice_deriv_cse_75: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let __rspice_deriv_cse_76: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let __rspice_deriv_cse_77: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22, eq139_e1769_d_b0, eq139_e1769_d_b1, eq139_e1769_d_b2, eq139_e1769_d_b3, eq139_e1769_d_b4, eq139_e1769_d_b5, eq139_e1769_d_b6, eq139_e1769_d_b7, eq139_e1769_d_b8, eq139_e1769_d_b9, eq139_e1769_d_b10, eq139_e1769_d_b11, eq139_e1769_d_b12, eq139_e1769_d_b13, eq139_e1769_d_b14, eq139_e1769_d_b15, eq139_e1769_d_b16, eq139_e1769_d_b17, eq139_e1769_d_b18, eq139_e1769_d_b19, eq139_e1769_d_b20, eq139_e1769_d_b21, eq139_e1769_d_b22, eq139_e1769_d_b23, eq139_e1769_d_b24, eq139_e1769_d_b25, eq139_e1769_d_b26, eq139_e1769_d_b27, eq139_e1769_d_b28, eq139_e1769_d_b29, eq139_e1769_d_b30, eq139_e1769_d_b31, eq139_e1769_d_b32, eq139_e1769_d_b33, eq139_e1769_d_b34, eq139_e1769_d_b35, eq139_e1769_d_b36, eq139_e1769_d_b37, eq139_e1769_d_b38, eq139_e1769_d_b39, eq139_e1769_d_b40, eq139_e1769_d_b41, eq139_e1769_d_b42, eq139_e1769_d_b43, eq139_e1769_d_b44, eq139_e1769_d_b45, eq139_e1769_d_b46, eq139_e1769_d_b47, eq139_e1769_d_b48, eq139_e1769_d_b49, eq139_e1769_d_b50, eq139_e1769_d_b51, eq139_e1769_d_b52, eq139_e1769_d_b53, eq139_e1769_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq139_e1766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 38, s.v[240]);
        let eq139_e1767: f64 = (p.p7 * eq139_e1766);
        (eq139_e1767, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_value: f64 = eq139_e1769;
        let eq139_node_derivatives: [f64; 23] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_branch_derivatives: [f64; 55] = [eq139_e1769_d_b0, eq139_e1769_d_b1, eq139_e1769_d_b2, eq139_e1769_d_b3, eq139_e1769_d_b4, eq139_e1769_d_b5, eq139_e1769_d_b6, eq139_e1769_d_b7, eq139_e1769_d_b8, eq139_e1769_d_b9, eq139_e1769_d_b10, eq139_e1769_d_b11, eq139_e1769_d_b12, eq139_e1769_d_b13, eq139_e1769_d_b14, eq139_e1769_d_b15, eq139_e1769_d_b16, eq139_e1769_d_b17, eq139_e1769_d_b18, eq139_e1769_d_b19, eq139_e1769_d_b20, eq139_e1769_d_b21, eq139_e1769_d_b22, eq139_e1769_d_b23, eq139_e1769_d_b24, eq139_e1769_d_b25, eq139_e1769_d_b26, eq139_e1769_d_b27, eq139_e1769_d_b28, eq139_e1769_d_b29, eq139_e1769_d_b30, eq139_e1769_d_b31, eq139_e1769_d_b32, eq139_e1769_d_b33, eq139_e1769_d_b34, eq139_e1769_d_b35, eq139_e1769_d_b36, eq139_e1769_d_b37, eq139_e1769_d_b38, eq139_e1769_d_b39, eq139_e1769_d_b40, eq139_e1769_d_b41, eq139_e1769_d_b42, eq139_e1769_d_b43, eq139_e1769_d_b44, eq139_e1769_d_b45, eq139_e1769_d_b46, eq139_e1769_d_b47, eq139_e1769_d_b48, eq139_e1769_d_b49, eq139_e1769_d_b50, eq139_e1769_d_b51, eq139_e1769_d_b52, eq139_e1769_d_b53, eq139_e1769_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq139_value),
            &eq139_node_derivatives,
            &eq139_branch_derivatives,
            multiplicity,
        );
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22, eq140_e1783_d_b0, eq140_e1783_d_b1, eq140_e1783_d_b2, eq140_e1783_d_b3, eq140_e1783_d_b4, eq140_e1783_d_b5, eq140_e1783_d_b6, eq140_e1783_d_b7, eq140_e1783_d_b8, eq140_e1783_d_b9, eq140_e1783_d_b10, eq140_e1783_d_b11, eq140_e1783_d_b12, eq140_e1783_d_b13, eq140_e1783_d_b14, eq140_e1783_d_b15, eq140_e1783_d_b16, eq140_e1783_d_b17, eq140_e1783_d_b18, eq140_e1783_d_b19, eq140_e1783_d_b20, eq140_e1783_d_b21, eq140_e1783_d_b22, eq140_e1783_d_b23, eq140_e1783_d_b24, eq140_e1783_d_b25, eq140_e1783_d_b26, eq140_e1783_d_b27, eq140_e1783_d_b28, eq140_e1783_d_b29, eq140_e1783_d_b30, eq140_e1783_d_b31, eq140_e1783_d_b32, eq140_e1783_d_b33, eq140_e1783_d_b34, eq140_e1783_d_b35, eq140_e1783_d_b36, eq140_e1783_d_b37, eq140_e1783_d_b38, eq140_e1783_d_b39, eq140_e1783_d_b40, eq140_e1783_d_b41, eq140_e1783_d_b42, eq140_e1783_d_b43, eq140_e1783_d_b44, eq140_e1783_d_b45, eq140_e1783_d_b46, eq140_e1783_d_b47, eq140_e1783_d_b48, eq140_e1783_d_b49, eq140_e1783_d_b50, eq140_e1783_d_b51, eq140_e1783_d_b52, eq140_e1783_d_b53, eq140_e1783_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && s.b[579]) {
        let eq140_e1778: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 39, s.v[240]);
        let eq140_e1779: f64 = (p.p7 * eq140_e1778);
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
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22, eq140_e1781_d_b0, eq140_e1781_d_b1, eq140_e1781_d_b2, eq140_e1781_d_b3, eq140_e1781_d_b4, eq140_e1781_d_b5, eq140_e1781_d_b6, eq140_e1781_d_b7, eq140_e1781_d_b8, eq140_e1781_d_b9, eq140_e1781_d_b10, eq140_e1781_d_b11, eq140_e1781_d_b12, eq140_e1781_d_b13, eq140_e1781_d_b14, eq140_e1781_d_b15, eq140_e1781_d_b16, eq140_e1781_d_b17, eq140_e1781_d_b18, eq140_e1781_d_b19, eq140_e1781_d_b20, eq140_e1781_d_b21, eq140_e1781_d_b22, eq140_e1781_d_b23, eq140_e1781_d_b24, eq140_e1781_d_b25, eq140_e1781_d_b26, eq140_e1781_d_b27, eq140_e1781_d_b28, eq140_e1781_d_b29, eq140_e1781_d_b30, eq140_e1781_d_b31, eq140_e1781_d_b32, eq140_e1781_d_b33, eq140_e1781_d_b34, eq140_e1781_d_b35, eq140_e1781_d_b36, eq140_e1781_d_b37, eq140_e1781_d_b38, eq140_e1781_d_b39, eq140_e1781_d_b40, eq140_e1781_d_b41, eq140_e1781_d_b42, eq140_e1781_d_b43, eq140_e1781_d_b44, eq140_e1781_d_b45, eq140_e1781_d_b46, eq140_e1781_d_b47, eq140_e1781_d_b48, eq140_e1781_d_b49, eq140_e1781_d_b50, eq140_e1781_d_b51, eq140_e1781_d_b52, eq140_e1781_d_b53, eq140_e1781_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_value: f64 = eq140_e1783;
        let eq140_node_derivatives: [f64; 23] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_branch_derivatives: [f64; 55] = [eq140_e1783_d_b0, eq140_e1783_d_b1, eq140_e1783_d_b2, eq140_e1783_d_b3, eq140_e1783_d_b4, eq140_e1783_d_b5, eq140_e1783_d_b6, eq140_e1783_d_b7, eq140_e1783_d_b8, eq140_e1783_d_b9, eq140_e1783_d_b10, eq140_e1783_d_b11, eq140_e1783_d_b12, eq140_e1783_d_b13, eq140_e1783_d_b14, eq140_e1783_d_b15, eq140_e1783_d_b16, eq140_e1783_d_b17, eq140_e1783_d_b18, eq140_e1783_d_b19, eq140_e1783_d_b20, eq140_e1783_d_b21, eq140_e1783_d_b22, eq140_e1783_d_b23, eq140_e1783_d_b24, eq140_e1783_d_b25, eq140_e1783_d_b26, eq140_e1783_d_b27, eq140_e1783_d_b28, eq140_e1783_d_b29, eq140_e1783_d_b30, eq140_e1783_d_b31, eq140_e1783_d_b32, eq140_e1783_d_b33, eq140_e1783_d_b34, eq140_e1783_d_b35, eq140_e1783_d_b36, eq140_e1783_d_b37, eq140_e1783_d_b38, eq140_e1783_d_b39, eq140_e1783_d_b40, eq140_e1783_d_b41, eq140_e1783_d_b42, eq140_e1783_d_b43, eq140_e1783_d_b44, eq140_e1783_d_b45, eq140_e1783_d_b46, eq140_e1783_d_b47, eq140_e1783_d_b48, eq140_e1783_d_b49, eq140_e1783_d_b50, eq140_e1783_d_b51, eq140_e1783_d_b52, eq140_e1783_d_b53, eq140_e1783_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq140_value),
            &eq140_node_derivatives,
            &eq140_branch_derivatives,
            multiplicity,
        );
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22, eq141_e1796_d_b0, eq141_e1796_d_b1, eq141_e1796_d_b2, eq141_e1796_d_b3, eq141_e1796_d_b4, eq141_e1796_d_b5, eq141_e1796_d_b6, eq141_e1796_d_b7, eq141_e1796_d_b8, eq141_e1796_d_b9, eq141_e1796_d_b10, eq141_e1796_d_b11, eq141_e1796_d_b12, eq141_e1796_d_b13, eq141_e1796_d_b14, eq141_e1796_d_b15, eq141_e1796_d_b16, eq141_e1796_d_b17, eq141_e1796_d_b18, eq141_e1796_d_b19, eq141_e1796_d_b20, eq141_e1796_d_b21, eq141_e1796_d_b22, eq141_e1796_d_b23, eq141_e1796_d_b24, eq141_e1796_d_b25, eq141_e1796_d_b26, eq141_e1796_d_b27, eq141_e1796_d_b28, eq141_e1796_d_b29, eq141_e1796_d_b30, eq141_e1796_d_b31, eq141_e1796_d_b32, eq141_e1796_d_b33, eq141_e1796_d_b34, eq141_e1796_d_b35, eq141_e1796_d_b36, eq141_e1796_d_b37, eq141_e1796_d_b38, eq141_e1796_d_b39, eq141_e1796_d_b40, eq141_e1796_d_b41, eq141_e1796_d_b42, eq141_e1796_d_b43, eq141_e1796_d_b44, eq141_e1796_d_b45, eq141_e1796_d_b46, eq141_e1796_d_b47, eq141_e1796_d_b48, eq141_e1796_d_b49, eq141_e1796_d_b50, eq141_e1796_d_b51, eq141_e1796_d_b52, eq141_e1796_d_b53, eq141_e1796_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq141_e1793: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 40, s.v[240]);
        let eq141_e1794: f64 = (p.p7 * eq141_e1793);
        (eq141_e1794, __rspice_deriv_cse_0, __rspice_deriv_cse_1, __rspice_deriv_cse_2, __rspice_deriv_cse_3, __rspice_deriv_cse_4, __rspice_deriv_cse_5, __rspice_deriv_cse_6, __rspice_deriv_cse_7, __rspice_deriv_cse_8, __rspice_deriv_cse_9, __rspice_deriv_cse_10, __rspice_deriv_cse_11, __rspice_deriv_cse_12, __rspice_deriv_cse_13, __rspice_deriv_cse_14, __rspice_deriv_cse_15, __rspice_deriv_cse_16, __rspice_deriv_cse_17, __rspice_deriv_cse_18, __rspice_deriv_cse_19, __rspice_deriv_cse_20, __rspice_deriv_cse_21, __rspice_deriv_cse_22, __rspice_deriv_cse_23, __rspice_deriv_cse_24, __rspice_deriv_cse_25, __rspice_deriv_cse_26, __rspice_deriv_cse_27, __rspice_deriv_cse_28, __rspice_deriv_cse_29, __rspice_deriv_cse_30, __rspice_deriv_cse_31, __rspice_deriv_cse_32, __rspice_deriv_cse_33, __rspice_deriv_cse_34, __rspice_deriv_cse_35, __rspice_deriv_cse_36, __rspice_deriv_cse_37, __rspice_deriv_cse_38, __rspice_deriv_cse_39, __rspice_deriv_cse_40, __rspice_deriv_cse_41, __rspice_deriv_cse_42, __rspice_deriv_cse_43, __rspice_deriv_cse_44, __rspice_deriv_cse_45, __rspice_deriv_cse_46, __rspice_deriv_cse_47, __rspice_deriv_cse_48, __rspice_deriv_cse_49, __rspice_deriv_cse_50, __rspice_deriv_cse_51, __rspice_deriv_cse_52, __rspice_deriv_cse_53, __rspice_deriv_cse_54, __rspice_deriv_cse_55, __rspice_deriv_cse_56, __rspice_deriv_cse_57, __rspice_deriv_cse_58, __rspice_deriv_cse_59, __rspice_deriv_cse_60, __rspice_deriv_cse_61, __rspice_deriv_cse_62, __rspice_deriv_cse_63, __rspice_deriv_cse_64, __rspice_deriv_cse_65, __rspice_deriv_cse_66, __rspice_deriv_cse_67, __rspice_deriv_cse_68, __rspice_deriv_cse_69, __rspice_deriv_cse_70, __rspice_deriv_cse_71, __rspice_deriv_cse_72, __rspice_deriv_cse_73, __rspice_deriv_cse_74, __rspice_deriv_cse_75, __rspice_deriv_cse_76, __rspice_deriv_cse_77,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1796;
        let eq141_node_derivatives: [f64; 23] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_branch_derivatives: [f64; 55] = [eq141_e1796_d_b0, eq141_e1796_d_b1, eq141_e1796_d_b2, eq141_e1796_d_b3, eq141_e1796_d_b4, eq141_e1796_d_b5, eq141_e1796_d_b6, eq141_e1796_d_b7, eq141_e1796_d_b8, eq141_e1796_d_b9, eq141_e1796_d_b10, eq141_e1796_d_b11, eq141_e1796_d_b12, eq141_e1796_d_b13, eq141_e1796_d_b14, eq141_e1796_d_b15, eq141_e1796_d_b16, eq141_e1796_d_b17, eq141_e1796_d_b18, eq141_e1796_d_b19, eq141_e1796_d_b20, eq141_e1796_d_b21, eq141_e1796_d_b22, eq141_e1796_d_b23, eq141_e1796_d_b24, eq141_e1796_d_b25, eq141_e1796_d_b26, eq141_e1796_d_b27, eq141_e1796_d_b28, eq141_e1796_d_b29, eq141_e1796_d_b30, eq141_e1796_d_b31, eq141_e1796_d_b32, eq141_e1796_d_b33, eq141_e1796_d_b34, eq141_e1796_d_b35, eq141_e1796_d_b36, eq141_e1796_d_b37, eq141_e1796_d_b38, eq141_e1796_d_b39, eq141_e1796_d_b40, eq141_e1796_d_b41, eq141_e1796_d_b42, eq141_e1796_d_b43, eq141_e1796_d_b44, eq141_e1796_d_b45, eq141_e1796_d_b46, eq141_e1796_d_b47, eq141_e1796_d_b48, eq141_e1796_d_b49, eq141_e1796_d_b50, eq141_e1796_d_b51, eq141_e1796_d_b52, eq141_e1796_d_b53, eq141_e1796_d_b54];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq141_value),
            &eq141_node_derivatives,
            &eq141_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_22(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22, eq142_e1811_d_b0, eq142_e1811_d_b1, eq142_e1811_d_b2, eq142_e1811_d_b3, eq142_e1811_d_b4, eq142_e1811_d_b5, eq142_e1811_d_b6, eq142_e1811_d_b7, eq142_e1811_d_b8, eq142_e1811_d_b9, eq142_e1811_d_b10, eq142_e1811_d_b11, eq142_e1811_d_b12, eq142_e1811_d_b13, eq142_e1811_d_b14, eq142_e1811_d_b15, eq142_e1811_d_b16, eq142_e1811_d_b17, eq142_e1811_d_b18, eq142_e1811_d_b19, eq142_e1811_d_b20, eq142_e1811_d_b21, eq142_e1811_d_b22, eq142_e1811_d_b23, eq142_e1811_d_b24, eq142_e1811_d_b25, eq142_e1811_d_b26, eq142_e1811_d_b27, eq142_e1811_d_b28, eq142_e1811_d_b29, eq142_e1811_d_b30, eq142_e1811_d_b31, eq142_e1811_d_b32, eq142_e1811_d_b33, eq142_e1811_d_b34, eq142_e1811_d_b35, eq142_e1811_d_b36, eq142_e1811_d_b37, eq142_e1811_d_b38, eq142_e1811_d_b39, eq142_e1811_d_b40, eq142_e1811_d_b41, eq142_e1811_d_b42, eq142_e1811_d_b43, eq142_e1811_d_b44, eq142_e1811_d_b45, eq142_e1811_d_b46, eq142_e1811_d_b47, eq142_e1811_d_b48, eq142_e1811_d_b49, eq142_e1811_d_b50, eq142_e1811_d_b51, eq142_e1811_d_b52, eq142_e1811_d_b53, eq142_e1811_d_b54,) = {
    if (((!s.b[575]) && s.b[578]) && (!s.b[579])) {
        let eq142_e1806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 41, s.v[240]);
        let eq142_e1807: f64 = (p.p7 * eq142_e1806);
        let eq142_e1807_d_n0: f64 = (p.p7 * (s.dn[240][0] * ddt_scale));
        let eq142_e1807_d_n1: f64 = (p.p7 * (s.dn[240][1] * ddt_scale));
        let eq142_e1807_d_n2: f64 = (p.p7 * (s.dn[240][2] * ddt_scale));
        let eq142_e1807_d_n3: f64 = (p.p7 * (s.dn[240][3] * ddt_scale));
        let eq142_e1807_d_n4: f64 = (p.p7 * (s.dn[240][4] * ddt_scale));
        let eq142_e1807_d_n5: f64 = (p.p7 * (s.dn[240][5] * ddt_scale));
        let eq142_e1807_d_n6: f64 = (p.p7 * (s.dn[240][6] * ddt_scale));
        let eq142_e1807_d_n7: f64 = (p.p7 * (s.dn[240][7] * ddt_scale));
        let eq142_e1807_d_n8: f64 = (p.p7 * (s.dn[240][8] * ddt_scale));
        let eq142_e1807_d_n9: f64 = (p.p7 * (s.dn[240][9] * ddt_scale));
        let eq142_e1807_d_n10: f64 = (p.p7 * (s.dn[240][10] * ddt_scale));
        let eq142_e1807_d_n11: f64 = (p.p7 * (s.dn[240][11] * ddt_scale));
        let eq142_e1807_d_n12: f64 = (p.p7 * (s.dn[240][12] * ddt_scale));
        let eq142_e1807_d_n13: f64 = (p.p7 * (s.dn[240][13] * ddt_scale));
        let eq142_e1807_d_n14: f64 = (p.p7 * (s.dn[240][14] * ddt_scale));
        let eq142_e1807_d_n15: f64 = (p.p7 * (s.dn[240][15] * ddt_scale));
        let eq142_e1807_d_n16: f64 = (p.p7 * (s.dn[240][16] * ddt_scale));
        let eq142_e1807_d_n17: f64 = (p.p7 * (s.dn[240][17] * ddt_scale));
        let eq142_e1807_d_n18: f64 = (p.p7 * (s.dn[240][18] * ddt_scale));
        let eq142_e1807_d_n19: f64 = (p.p7 * (s.dn[240][19] * ddt_scale));
        let eq142_e1807_d_n20: f64 = (p.p7 * (s.dn[240][20] * ddt_scale));
        let eq142_e1807_d_n21: f64 = (p.p7 * (s.dn[240][21] * ddt_scale));
        let eq142_e1807_d_n22: f64 = (p.p7 * (s.dn[240][22] * ddt_scale));
        let eq142_e1807_d_b0: f64 = (p.p7 * (s.db[240][0] * ddt_scale));
        let eq142_e1807_d_b1: f64 = (p.p7 * (s.db[240][1] * ddt_scale));
        let eq142_e1807_d_b2: f64 = (p.p7 * (s.db[240][2] * ddt_scale));
        let eq142_e1807_d_b3: f64 = (p.p7 * (s.db[240][3] * ddt_scale));
        let eq142_e1807_d_b4: f64 = (p.p7 * (s.db[240][4] * ddt_scale));
        let eq142_e1807_d_b5: f64 = (p.p7 * (s.db[240][5] * ddt_scale));
        let eq142_e1807_d_b6: f64 = (p.p7 * (s.db[240][6] * ddt_scale));
        let eq142_e1807_d_b7: f64 = (p.p7 * (s.db[240][7] * ddt_scale));
        let eq142_e1807_d_b8: f64 = (p.p7 * (s.db[240][8] * ddt_scale));
        let eq142_e1807_d_b9: f64 = (p.p7 * (s.db[240][9] * ddt_scale));
        let eq142_e1807_d_b10: f64 = (p.p7 * (s.db[240][10] * ddt_scale));
        let eq142_e1807_d_b11: f64 = (p.p7 * (s.db[240][11] * ddt_scale));
        let eq142_e1807_d_b12: f64 = (p.p7 * (s.db[240][12] * ddt_scale));
        let eq142_e1807_d_b13: f64 = (p.p7 * (s.db[240][13] * ddt_scale));
        let eq142_e1807_d_b14: f64 = (p.p7 * (s.db[240][14] * ddt_scale));
        let eq142_e1807_d_b15: f64 = (p.p7 * (s.db[240][15] * ddt_scale));
        let eq142_e1807_d_b16: f64 = (p.p7 * (s.db[240][16] * ddt_scale));
        let eq142_e1807_d_b17: f64 = (p.p7 * (s.db[240][17] * ddt_scale));
        let eq142_e1807_d_b18: f64 = (p.p7 * (s.db[240][18] * ddt_scale));
        let eq142_e1807_d_b19: f64 = (p.p7 * (s.db[240][19] * ddt_scale));
        let eq142_e1807_d_b20: f64 = (p.p7 * (s.db[240][20] * ddt_scale));
        let eq142_e1807_d_b21: f64 = (p.p7 * (s.db[240][21] * ddt_scale));
        let eq142_e1807_d_b22: f64 = (p.p7 * (s.db[240][22] * ddt_scale));
        let eq142_e1807_d_b23: f64 = (p.p7 * (s.db[240][23] * ddt_scale));
        let eq142_e1807_d_b24: f64 = (p.p7 * (s.db[240][24] * ddt_scale));
        let eq142_e1807_d_b25: f64 = (p.p7 * (s.db[240][25] * ddt_scale));
        let eq142_e1807_d_b26: f64 = (p.p7 * (s.db[240][26] * ddt_scale));
        let eq142_e1807_d_b27: f64 = (p.p7 * (s.db[240][27] * ddt_scale));
        let eq142_e1807_d_b28: f64 = (p.p7 * (s.db[240][28] * ddt_scale));
        let eq142_e1807_d_b29: f64 = (p.p7 * (s.db[240][29] * ddt_scale));
        let eq142_e1807_d_b30: f64 = (p.p7 * (s.db[240][30] * ddt_scale));
        let eq142_e1807_d_b31: f64 = (p.p7 * (s.db[240][31] * ddt_scale));
        let eq142_e1807_d_b32: f64 = (p.p7 * (s.db[240][32] * ddt_scale));
        let eq142_e1807_d_b33: f64 = (p.p7 * (s.db[240][33] * ddt_scale));
        let eq142_e1807_d_b34: f64 = (p.p7 * (s.db[240][34] * ddt_scale));
        let eq142_e1807_d_b35: f64 = (p.p7 * (s.db[240][35] * ddt_scale));
        let eq142_e1807_d_b36: f64 = (p.p7 * (s.db[240][36] * ddt_scale));
        let eq142_e1807_d_b37: f64 = (p.p7 * (s.db[240][37] * ddt_scale));
        let eq142_e1807_d_b38: f64 = (p.p7 * (s.db[240][38] * ddt_scale));
        let eq142_e1807_d_b39: f64 = (p.p7 * (s.db[240][39] * ddt_scale));
        let eq142_e1807_d_b40: f64 = (p.p7 * (s.db[240][40] * ddt_scale));
        let eq142_e1807_d_b41: f64 = (p.p7 * (s.db[240][41] * ddt_scale));
        let eq142_e1807_d_b42: f64 = (p.p7 * (s.db[240][42] * ddt_scale));
        let eq142_e1807_d_b43: f64 = (p.p7 * (s.db[240][43] * ddt_scale));
        let eq142_e1807_d_b44: f64 = (p.p7 * (s.db[240][44] * ddt_scale));
        let eq142_e1807_d_b45: f64 = (p.p7 * (s.db[240][45] * ddt_scale));
        let eq142_e1807_d_b46: f64 = (p.p7 * (s.db[240][46] * ddt_scale));
        let eq142_e1807_d_b47: f64 = (p.p7 * (s.db[240][47] * ddt_scale));
        let eq142_e1807_d_b48: f64 = (p.p7 * (s.db[240][48] * ddt_scale));
        let eq142_e1807_d_b49: f64 = (p.p7 * (s.db[240][49] * ddt_scale));
        let eq142_e1807_d_b50: f64 = (p.p7 * (s.db[240][50] * ddt_scale));
        let eq142_e1807_d_b51: f64 = (p.p7 * (s.db[240][51] * ddt_scale));
        let eq142_e1807_d_b52: f64 = (p.p7 * (s.db[240][52] * ddt_scale));
        let eq142_e1807_d_b53: f64 = (p.p7 * (s.db[240][53] * ddt_scale));
        let eq142_e1807_d_b54: f64 = (p.p7 * (s.db[240][54] * ddt_scale));
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
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22, eq142_e1809_d_b0, eq142_e1809_d_b1, eq142_e1809_d_b2, eq142_e1809_d_b3, eq142_e1809_d_b4, eq142_e1809_d_b5, eq142_e1809_d_b6, eq142_e1809_d_b7, eq142_e1809_d_b8, eq142_e1809_d_b9, eq142_e1809_d_b10, eq142_e1809_d_b11, eq142_e1809_d_b12, eq142_e1809_d_b13, eq142_e1809_d_b14, eq142_e1809_d_b15, eq142_e1809_d_b16, eq142_e1809_d_b17, eq142_e1809_d_b18, eq142_e1809_d_b19, eq142_e1809_d_b20, eq142_e1809_d_b21, eq142_e1809_d_b22, eq142_e1809_d_b23, eq142_e1809_d_b24, eq142_e1809_d_b25, eq142_e1809_d_b26, eq142_e1809_d_b27, eq142_e1809_d_b28, eq142_e1809_d_b29, eq142_e1809_d_b30, eq142_e1809_d_b31, eq142_e1809_d_b32, eq142_e1809_d_b33, eq142_e1809_d_b34, eq142_e1809_d_b35, eq142_e1809_d_b36, eq142_e1809_d_b37, eq142_e1809_d_b38, eq142_e1809_d_b39, eq142_e1809_d_b40, eq142_e1809_d_b41, eq142_e1809_d_b42, eq142_e1809_d_b43, eq142_e1809_d_b44, eq142_e1809_d_b45, eq142_e1809_d_b46, eq142_e1809_d_b47, eq142_e1809_d_b48, eq142_e1809_d_b49, eq142_e1809_d_b50, eq142_e1809_d_b51, eq142_e1809_d_b52, eq142_e1809_d_b53, eq142_e1809_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1811;
        let eq142_node_derivatives: [f64; 23] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_branch_derivatives: [f64; 55] = [eq142_e1811_d_b0, eq142_e1811_d_b1, eq142_e1811_d_b2, eq142_e1811_d_b3, eq142_e1811_d_b4, eq142_e1811_d_b5, eq142_e1811_d_b6, eq142_e1811_d_b7, eq142_e1811_d_b8, eq142_e1811_d_b9, eq142_e1811_d_b10, eq142_e1811_d_b11, eq142_e1811_d_b12, eq142_e1811_d_b13, eq142_e1811_d_b14, eq142_e1811_d_b15, eq142_e1811_d_b16, eq142_e1811_d_b17, eq142_e1811_d_b18, eq142_e1811_d_b19, eq142_e1811_d_b20, eq142_e1811_d_b21, eq142_e1811_d_b22, eq142_e1811_d_b23, eq142_e1811_d_b24, eq142_e1811_d_b25, eq142_e1811_d_b26, eq142_e1811_d_b27, eq142_e1811_d_b28, eq142_e1811_d_b29, eq142_e1811_d_b30, eq142_e1811_d_b31, eq142_e1811_d_b32, eq142_e1811_d_b33, eq142_e1811_d_b34, eq142_e1811_d_b35, eq142_e1811_d_b36, eq142_e1811_d_b37, eq142_e1811_d_b38, eq142_e1811_d_b39, eq142_e1811_d_b40, eq142_e1811_d_b41, eq142_e1811_d_b42, eq142_e1811_d_b43, eq142_e1811_d_b44, eq142_e1811_d_b45, eq142_e1811_d_b46, eq142_e1811_d_b47, eq142_e1811_d_b48, eq142_e1811_d_b49, eq142_e1811_d_b50, eq142_e1811_d_b51, eq142_e1811_d_b52, eq142_e1811_d_b53, eq142_e1811_d_b54];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq142_value),
            &eq142_node_derivatives,
            &eq142_branch_derivatives,
            multiplicity,
        );
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22, eq143_e1823_d_b0, eq143_e1823_d_b1, eq143_e1823_d_b2, eq143_e1823_d_b3, eq143_e1823_d_b4, eq143_e1823_d_b5, eq143_e1823_d_b6, eq143_e1823_d_b7, eq143_e1823_d_b8, eq143_e1823_d_b9, eq143_e1823_d_b10, eq143_e1823_d_b11, eq143_e1823_d_b12, eq143_e1823_d_b13, eq143_e1823_d_b14, eq143_e1823_d_b15, eq143_e1823_d_b16, eq143_e1823_d_b17, eq143_e1823_d_b18, eq143_e1823_d_b19, eq143_e1823_d_b20, eq143_e1823_d_b21, eq143_e1823_d_b22, eq143_e1823_d_b23, eq143_e1823_d_b24, eq143_e1823_d_b25, eq143_e1823_d_b26, eq143_e1823_d_b27, eq143_e1823_d_b28, eq143_e1823_d_b29, eq143_e1823_d_b30, eq143_e1823_d_b31, eq143_e1823_d_b32, eq143_e1823_d_b33, eq143_e1823_d_b34, eq143_e1823_d_b35, eq143_e1823_d_b36, eq143_e1823_d_b37, eq143_e1823_d_b38, eq143_e1823_d_b39, eq143_e1823_d_b40, eq143_e1823_d_b41, eq143_e1823_d_b42, eq143_e1823_d_b43, eq143_e1823_d_b44, eq143_e1823_d_b45, eq143_e1823_d_b46, eq143_e1823_d_b47, eq143_e1823_d_b48, eq143_e1823_d_b49, eq143_e1823_d_b50, eq143_e1823_d_b51, eq143_e1823_d_b52, eq143_e1823_d_b53, eq143_e1823_d_b54,) = {
    if ((!s.b[575]) && s.b[578]) {
        let eq143_e1819: f64 = (p.p251 * s.v[240]);
        let eq143_e1820: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 42, eq143_e1819);
        let eq143_e1820_d_n0: f64 = ((p.p251 * s.dn[240][0]) * ddt_scale);
        let eq143_e1820_d_n1: f64 = ((p.p251 * s.dn[240][1]) * ddt_scale);
        let eq143_e1820_d_n2: f64 = ((p.p251 * s.dn[240][2]) * ddt_scale);
        let eq143_e1820_d_n3: f64 = ((p.p251 * s.dn[240][3]) * ddt_scale);
        let eq143_e1820_d_n4: f64 = ((p.p251 * s.dn[240][4]) * ddt_scale);
        let eq143_e1820_d_n5: f64 = ((p.p251 * s.dn[240][5]) * ddt_scale);
        let eq143_e1820_d_n6: f64 = ((p.p251 * s.dn[240][6]) * ddt_scale);
        let eq143_e1820_d_n7: f64 = ((p.p251 * s.dn[240][7]) * ddt_scale);
        let eq143_e1820_d_n8: f64 = ((p.p251 * s.dn[240][8]) * ddt_scale);
        let eq143_e1820_d_n9: f64 = ((p.p251 * s.dn[240][9]) * ddt_scale);
        let eq143_e1820_d_n10: f64 = ((p.p251 * s.dn[240][10]) * ddt_scale);
        let eq143_e1820_d_n11: f64 = ((p.p251 * s.dn[240][11]) * ddt_scale);
        let eq143_e1820_d_n12: f64 = ((p.p251 * s.dn[240][12]) * ddt_scale);
        let eq143_e1820_d_n13: f64 = ((p.p251 * s.dn[240][13]) * ddt_scale);
        let eq143_e1820_d_n14: f64 = ((p.p251 * s.dn[240][14]) * ddt_scale);
        let eq143_e1820_d_n15: f64 = ((p.p251 * s.dn[240][15]) * ddt_scale);
        let eq143_e1820_d_n16: f64 = ((p.p251 * s.dn[240][16]) * ddt_scale);
        let eq143_e1820_d_n17: f64 = ((p.p251 * s.dn[240][17]) * ddt_scale);
        let eq143_e1820_d_n18: f64 = ((p.p251 * s.dn[240][18]) * ddt_scale);
        let eq143_e1820_d_n19: f64 = ((p.p251 * s.dn[240][19]) * ddt_scale);
        let eq143_e1820_d_n20: f64 = ((p.p251 * s.dn[240][20]) * ddt_scale);
        let eq143_e1820_d_n21: f64 = ((p.p251 * s.dn[240][21]) * ddt_scale);
        let eq143_e1820_d_n22: f64 = ((p.p251 * s.dn[240][22]) * ddt_scale);
        let eq143_e1820_d_b0: f64 = ((p.p251 * s.db[240][0]) * ddt_scale);
        let eq143_e1820_d_b1: f64 = ((p.p251 * s.db[240][1]) * ddt_scale);
        let eq143_e1820_d_b2: f64 = ((p.p251 * s.db[240][2]) * ddt_scale);
        let eq143_e1820_d_b3: f64 = ((p.p251 * s.db[240][3]) * ddt_scale);
        let eq143_e1820_d_b4: f64 = ((p.p251 * s.db[240][4]) * ddt_scale);
        let eq143_e1820_d_b5: f64 = ((p.p251 * s.db[240][5]) * ddt_scale);
        let eq143_e1820_d_b6: f64 = ((p.p251 * s.db[240][6]) * ddt_scale);
        let eq143_e1820_d_b7: f64 = ((p.p251 * s.db[240][7]) * ddt_scale);
        let eq143_e1820_d_b8: f64 = ((p.p251 * s.db[240][8]) * ddt_scale);
        let eq143_e1820_d_b9: f64 = ((p.p251 * s.db[240][9]) * ddt_scale);
        let eq143_e1820_d_b10: f64 = ((p.p251 * s.db[240][10]) * ddt_scale);
        let eq143_e1820_d_b11: f64 = ((p.p251 * s.db[240][11]) * ddt_scale);
        let eq143_e1820_d_b12: f64 = ((p.p251 * s.db[240][12]) * ddt_scale);
        let eq143_e1820_d_b13: f64 = ((p.p251 * s.db[240][13]) * ddt_scale);
        let eq143_e1820_d_b14: f64 = ((p.p251 * s.db[240][14]) * ddt_scale);
        let eq143_e1820_d_b15: f64 = ((p.p251 * s.db[240][15]) * ddt_scale);
        let eq143_e1820_d_b16: f64 = ((p.p251 * s.db[240][16]) * ddt_scale);
        let eq143_e1820_d_b17: f64 = ((p.p251 * s.db[240][17]) * ddt_scale);
        let eq143_e1820_d_b18: f64 = ((p.p251 * s.db[240][18]) * ddt_scale);
        let eq143_e1820_d_b19: f64 = ((p.p251 * s.db[240][19]) * ddt_scale);
        let eq143_e1820_d_b20: f64 = ((p.p251 * s.db[240][20]) * ddt_scale);
        let eq143_e1820_d_b21: f64 = ((p.p251 * s.db[240][21]) * ddt_scale);
        let eq143_e1820_d_b22: f64 = ((p.p251 * s.db[240][22]) * ddt_scale);
        let eq143_e1820_d_b23: f64 = ((p.p251 * s.db[240][23]) * ddt_scale);
        let eq143_e1820_d_b24: f64 = ((p.p251 * s.db[240][24]) * ddt_scale);
        let eq143_e1820_d_b25: f64 = ((p.p251 * s.db[240][25]) * ddt_scale);
        let eq143_e1820_d_b26: f64 = ((p.p251 * s.db[240][26]) * ddt_scale);
        let eq143_e1820_d_b27: f64 = ((p.p251 * s.db[240][27]) * ddt_scale);
        let eq143_e1820_d_b28: f64 = ((p.p251 * s.db[240][28]) * ddt_scale);
        let eq143_e1820_d_b29: f64 = ((p.p251 * s.db[240][29]) * ddt_scale);
        let eq143_e1820_d_b30: f64 = ((p.p251 * s.db[240][30]) * ddt_scale);
        let eq143_e1820_d_b31: f64 = ((p.p251 * s.db[240][31]) * ddt_scale);
        let eq143_e1820_d_b32: f64 = ((p.p251 * s.db[240][32]) * ddt_scale);
        let eq143_e1820_d_b33: f64 = ((p.p251 * s.db[240][33]) * ddt_scale);
        let eq143_e1820_d_b34: f64 = ((p.p251 * s.db[240][34]) * ddt_scale);
        let eq143_e1820_d_b35: f64 = ((p.p251 * s.db[240][35]) * ddt_scale);
        let eq143_e1820_d_b36: f64 = ((p.p251 * s.db[240][36]) * ddt_scale);
        let eq143_e1820_d_b37: f64 = ((p.p251 * s.db[240][37]) * ddt_scale);
        let eq143_e1820_d_b38: f64 = ((p.p251 * s.db[240][38]) * ddt_scale);
        let eq143_e1820_d_b39: f64 = ((p.p251 * s.db[240][39]) * ddt_scale);
        let eq143_e1820_d_b40: f64 = ((p.p251 * s.db[240][40]) * ddt_scale);
        let eq143_e1820_d_b41: f64 = ((p.p251 * s.db[240][41]) * ddt_scale);
        let eq143_e1820_d_b42: f64 = ((p.p251 * s.db[240][42]) * ddt_scale);
        let eq143_e1820_d_b43: f64 = ((p.p251 * s.db[240][43]) * ddt_scale);
        let eq143_e1820_d_b44: f64 = ((p.p251 * s.db[240][44]) * ddt_scale);
        let eq143_e1820_d_b45: f64 = ((p.p251 * s.db[240][45]) * ddt_scale);
        let eq143_e1820_d_b46: f64 = ((p.p251 * s.db[240][46]) * ddt_scale);
        let eq143_e1820_d_b47: f64 = ((p.p251 * s.db[240][47]) * ddt_scale);
        let eq143_e1820_d_b48: f64 = ((p.p251 * s.db[240][48]) * ddt_scale);
        let eq143_e1820_d_b49: f64 = ((p.p251 * s.db[240][49]) * ddt_scale);
        let eq143_e1820_d_b50: f64 = ((p.p251 * s.db[240][50]) * ddt_scale);
        let eq143_e1820_d_b51: f64 = ((p.p251 * s.db[240][51]) * ddt_scale);
        let eq143_e1820_d_b52: f64 = ((p.p251 * s.db[240][52]) * ddt_scale);
        let eq143_e1820_d_b53: f64 = ((p.p251 * s.db[240][53]) * ddt_scale);
        let eq143_e1820_d_b54: f64 = ((p.p251 * s.db[240][54]) * ddt_scale);
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
        let eq143_e1821_d_b0: f64 = (p.p7 * eq143_e1820_d_b0);
        let eq143_e1821_d_b1: f64 = (p.p7 * eq143_e1820_d_b1);
        let eq143_e1821_d_b2: f64 = (p.p7 * eq143_e1820_d_b2);
        let eq143_e1821_d_b3: f64 = (p.p7 * eq143_e1820_d_b3);
        let eq143_e1821_d_b4: f64 = (p.p7 * eq143_e1820_d_b4);
        let eq143_e1821_d_b5: f64 = (p.p7 * eq143_e1820_d_b5);
        let eq143_e1821_d_b6: f64 = (p.p7 * eq143_e1820_d_b6);
        let eq143_e1821_d_b7: f64 = (p.p7 * eq143_e1820_d_b7);
        let eq143_e1821_d_b8: f64 = (p.p7 * eq143_e1820_d_b8);
        let eq143_e1821_d_b9: f64 = (p.p7 * eq143_e1820_d_b9);
        let eq143_e1821_d_b10: f64 = (p.p7 * eq143_e1820_d_b10);
        let eq143_e1821_d_b11: f64 = (p.p7 * eq143_e1820_d_b11);
        let eq143_e1821_d_b12: f64 = (p.p7 * eq143_e1820_d_b12);
        let eq143_e1821_d_b13: f64 = (p.p7 * eq143_e1820_d_b13);
        let eq143_e1821_d_b14: f64 = (p.p7 * eq143_e1820_d_b14);
        let eq143_e1821_d_b15: f64 = (p.p7 * eq143_e1820_d_b15);
        let eq143_e1821_d_b16: f64 = (p.p7 * eq143_e1820_d_b16);
        let eq143_e1821_d_b17: f64 = (p.p7 * eq143_e1820_d_b17);
        let eq143_e1821_d_b18: f64 = (p.p7 * eq143_e1820_d_b18);
        let eq143_e1821_d_b19: f64 = (p.p7 * eq143_e1820_d_b19);
        let eq143_e1821_d_b20: f64 = (p.p7 * eq143_e1820_d_b20);
        let eq143_e1821_d_b21: f64 = (p.p7 * eq143_e1820_d_b21);
        let eq143_e1821_d_b22: f64 = (p.p7 * eq143_e1820_d_b22);
        let eq143_e1821_d_b23: f64 = (p.p7 * eq143_e1820_d_b23);
        let eq143_e1821_d_b24: f64 = (p.p7 * eq143_e1820_d_b24);
        let eq143_e1821_d_b25: f64 = (p.p7 * eq143_e1820_d_b25);
        let eq143_e1821_d_b26: f64 = (p.p7 * eq143_e1820_d_b26);
        let eq143_e1821_d_b27: f64 = (p.p7 * eq143_e1820_d_b27);
        let eq143_e1821_d_b28: f64 = (p.p7 * eq143_e1820_d_b28);
        let eq143_e1821_d_b29: f64 = (p.p7 * eq143_e1820_d_b29);
        let eq143_e1821_d_b30: f64 = (p.p7 * eq143_e1820_d_b30);
        let eq143_e1821_d_b31: f64 = (p.p7 * eq143_e1820_d_b31);
        let eq143_e1821_d_b32: f64 = (p.p7 * eq143_e1820_d_b32);
        let eq143_e1821_d_b33: f64 = (p.p7 * eq143_e1820_d_b33);
        let eq143_e1821_d_b34: f64 = (p.p7 * eq143_e1820_d_b34);
        let eq143_e1821_d_b35: f64 = (p.p7 * eq143_e1820_d_b35);
        let eq143_e1821_d_b36: f64 = (p.p7 * eq143_e1820_d_b36);
        let eq143_e1821_d_b37: f64 = (p.p7 * eq143_e1820_d_b37);
        let eq143_e1821_d_b38: f64 = (p.p7 * eq143_e1820_d_b38);
        let eq143_e1821_d_b39: f64 = (p.p7 * eq143_e1820_d_b39);
        let eq143_e1821_d_b40: f64 = (p.p7 * eq143_e1820_d_b40);
        let eq143_e1821_d_b41: f64 = (p.p7 * eq143_e1820_d_b41);
        let eq143_e1821_d_b42: f64 = (p.p7 * eq143_e1820_d_b42);
        let eq143_e1821_d_b43: f64 = (p.p7 * eq143_e1820_d_b43);
        let eq143_e1821_d_b44: f64 = (p.p7 * eq143_e1820_d_b44);
        let eq143_e1821_d_b45: f64 = (p.p7 * eq143_e1820_d_b45);
        let eq143_e1821_d_b46: f64 = (p.p7 * eq143_e1820_d_b46);
        let eq143_e1821_d_b47: f64 = (p.p7 * eq143_e1820_d_b47);
        let eq143_e1821_d_b48: f64 = (p.p7 * eq143_e1820_d_b48);
        let eq143_e1821_d_b49: f64 = (p.p7 * eq143_e1820_d_b49);
        let eq143_e1821_d_b50: f64 = (p.p7 * eq143_e1820_d_b50);
        let eq143_e1821_d_b51: f64 = (p.p7 * eq143_e1820_d_b51);
        let eq143_e1821_d_b52: f64 = (p.p7 * eq143_e1820_d_b52);
        let eq143_e1821_d_b53: f64 = (p.p7 * eq143_e1820_d_b53);
        let eq143_e1821_d_b54: f64 = (p.p7 * eq143_e1820_d_b54);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22, eq143_e1821_d_b0, eq143_e1821_d_b1, eq143_e1821_d_b2, eq143_e1821_d_b3, eq143_e1821_d_b4, eq143_e1821_d_b5, eq143_e1821_d_b6, eq143_e1821_d_b7, eq143_e1821_d_b8, eq143_e1821_d_b9, eq143_e1821_d_b10, eq143_e1821_d_b11, eq143_e1821_d_b12, eq143_e1821_d_b13, eq143_e1821_d_b14, eq143_e1821_d_b15, eq143_e1821_d_b16, eq143_e1821_d_b17, eq143_e1821_d_b18, eq143_e1821_d_b19, eq143_e1821_d_b20, eq143_e1821_d_b21, eq143_e1821_d_b22, eq143_e1821_d_b23, eq143_e1821_d_b24, eq143_e1821_d_b25, eq143_e1821_d_b26, eq143_e1821_d_b27, eq143_e1821_d_b28, eq143_e1821_d_b29, eq143_e1821_d_b30, eq143_e1821_d_b31, eq143_e1821_d_b32, eq143_e1821_d_b33, eq143_e1821_d_b34, eq143_e1821_d_b35, eq143_e1821_d_b36, eq143_e1821_d_b37, eq143_e1821_d_b38, eq143_e1821_d_b39, eq143_e1821_d_b40, eq143_e1821_d_b41, eq143_e1821_d_b42, eq143_e1821_d_b43, eq143_e1821_d_b44, eq143_e1821_d_b45, eq143_e1821_d_b46, eq143_e1821_d_b47, eq143_e1821_d_b48, eq143_e1821_d_b49, eq143_e1821_d_b50, eq143_e1821_d_b51, eq143_e1821_d_b52, eq143_e1821_d_b53, eq143_e1821_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1823;
        let eq143_node_derivatives: [f64; 23] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_branch_derivatives: [f64; 55] = [eq143_e1823_d_b0, eq143_e1823_d_b1, eq143_e1823_d_b2, eq143_e1823_d_b3, eq143_e1823_d_b4, eq143_e1823_d_b5, eq143_e1823_d_b6, eq143_e1823_d_b7, eq143_e1823_d_b8, eq143_e1823_d_b9, eq143_e1823_d_b10, eq143_e1823_d_b11, eq143_e1823_d_b12, eq143_e1823_d_b13, eq143_e1823_d_b14, eq143_e1823_d_b15, eq143_e1823_d_b16, eq143_e1823_d_b17, eq143_e1823_d_b18, eq143_e1823_d_b19, eq143_e1823_d_b20, eq143_e1823_d_b21, eq143_e1823_d_b22, eq143_e1823_d_b23, eq143_e1823_d_b24, eq143_e1823_d_b25, eq143_e1823_d_b26, eq143_e1823_d_b27, eq143_e1823_d_b28, eq143_e1823_d_b29, eq143_e1823_d_b30, eq143_e1823_d_b31, eq143_e1823_d_b32, eq143_e1823_d_b33, eq143_e1823_d_b34, eq143_e1823_d_b35, eq143_e1823_d_b36, eq143_e1823_d_b37, eq143_e1823_d_b38, eq143_e1823_d_b39, eq143_e1823_d_b40, eq143_e1823_d_b41, eq143_e1823_d_b42, eq143_e1823_d_b43, eq143_e1823_d_b44, eq143_e1823_d_b45, eq143_e1823_d_b46, eq143_e1823_d_b47, eq143_e1823_d_b48, eq143_e1823_d_b49, eq143_e1823_d_b50, eq143_e1823_d_b51, eq143_e1823_d_b52, eq143_e1823_d_b53, eq143_e1823_d_b54];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq143_value),
            &eq143_node_derivatives,
            &eq143_branch_derivatives,
            multiplicity,
        );
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22, eq144_e1832_d_b0, eq144_e1832_d_b1, eq144_e1832_d_b2, eq144_e1832_d_b3, eq144_e1832_d_b4, eq144_e1832_d_b5, eq144_e1832_d_b6, eq144_e1832_d_b7, eq144_e1832_d_b8, eq144_e1832_d_b9, eq144_e1832_d_b10, eq144_e1832_d_b11, eq144_e1832_d_b12, eq144_e1832_d_b13, eq144_e1832_d_b14, eq144_e1832_d_b15, eq144_e1832_d_b16, eq144_e1832_d_b17, eq144_e1832_d_b18, eq144_e1832_d_b19, eq144_e1832_d_b20, eq144_e1832_d_b21, eq144_e1832_d_b22, eq144_e1832_d_b23, eq144_e1832_d_b24, eq144_e1832_d_b25, eq144_e1832_d_b26, eq144_e1832_d_b27, eq144_e1832_d_b28, eq144_e1832_d_b29, eq144_e1832_d_b30, eq144_e1832_d_b31, eq144_e1832_d_b32, eq144_e1832_d_b33, eq144_e1832_d_b34, eq144_e1832_d_b35, eq144_e1832_d_b36, eq144_e1832_d_b37, eq144_e1832_d_b38, eq144_e1832_d_b39, eq144_e1832_d_b40, eq144_e1832_d_b41, eq144_e1832_d_b42, eq144_e1832_d_b43, eq144_e1832_d_b44, eq144_e1832_d_b45, eq144_e1832_d_b46, eq144_e1832_d_b47, eq144_e1832_d_b48, eq144_e1832_d_b49, eq144_e1832_d_b50, eq144_e1832_d_b51, eq144_e1832_d_b52, eq144_e1832_d_b53, eq144_e1832_d_b54,) = {
    if (s.b[580] && s.b[581]) {
        let eq144_e1829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 43, s.v[253]);
        let eq144_e1830: f64 = (p.p7 * eq144_e1829);
        let eq144_e1830_d_n0: f64 = (p.p7 * (s.dn[253][0] * ddt_scale));
        let eq144_e1830_d_n1: f64 = (p.p7 * (s.dn[253][1] * ddt_scale));
        let eq144_e1830_d_n2: f64 = (p.p7 * (s.dn[253][2] * ddt_scale));
        let eq144_e1830_d_n3: f64 = (p.p7 * (s.dn[253][3] * ddt_scale));
        let eq144_e1830_d_n4: f64 = (p.p7 * (s.dn[253][4] * ddt_scale));
        let eq144_e1830_d_n5: f64 = (p.p7 * (s.dn[253][5] * ddt_scale));
        let eq144_e1830_d_n6: f64 = (p.p7 * (s.dn[253][6] * ddt_scale));
        let eq144_e1830_d_n7: f64 = (p.p7 * (s.dn[253][7] * ddt_scale));
        let eq144_e1830_d_n8: f64 = (p.p7 * (s.dn[253][8] * ddt_scale));
        let eq144_e1830_d_n9: f64 = (p.p7 * (s.dn[253][9] * ddt_scale));
        let eq144_e1830_d_n10: f64 = (p.p7 * (s.dn[253][10] * ddt_scale));
        let eq144_e1830_d_n11: f64 = (p.p7 * (s.dn[253][11] * ddt_scale));
        let eq144_e1830_d_n12: f64 = (p.p7 * (s.dn[253][12] * ddt_scale));
        let eq144_e1830_d_n13: f64 = (p.p7 * (s.dn[253][13] * ddt_scale));
        let eq144_e1830_d_n14: f64 = (p.p7 * (s.dn[253][14] * ddt_scale));
        let eq144_e1830_d_n15: f64 = (p.p7 * (s.dn[253][15] * ddt_scale));
        let eq144_e1830_d_n16: f64 = (p.p7 * (s.dn[253][16] * ddt_scale));
        let eq144_e1830_d_n17: f64 = (p.p7 * (s.dn[253][17] * ddt_scale));
        let eq144_e1830_d_n18: f64 = (p.p7 * (s.dn[253][18] * ddt_scale));
        let eq144_e1830_d_n19: f64 = (p.p7 * (s.dn[253][19] * ddt_scale));
        let eq144_e1830_d_n20: f64 = (p.p7 * (s.dn[253][20] * ddt_scale));
        let eq144_e1830_d_n21: f64 = (p.p7 * (s.dn[253][21] * ddt_scale));
        let eq144_e1830_d_n22: f64 = (p.p7 * (s.dn[253][22] * ddt_scale));
        let eq144_e1830_d_b0: f64 = (p.p7 * (s.db[253][0] * ddt_scale));
        let eq144_e1830_d_b1: f64 = (p.p7 * (s.db[253][1] * ddt_scale));
        let eq144_e1830_d_b2: f64 = (p.p7 * (s.db[253][2] * ddt_scale));
        let eq144_e1830_d_b3: f64 = (p.p7 * (s.db[253][3] * ddt_scale));
        let eq144_e1830_d_b4: f64 = (p.p7 * (s.db[253][4] * ddt_scale));
        let eq144_e1830_d_b5: f64 = (p.p7 * (s.db[253][5] * ddt_scale));
        let eq144_e1830_d_b6: f64 = (p.p7 * (s.db[253][6] * ddt_scale));
        let eq144_e1830_d_b7: f64 = (p.p7 * (s.db[253][7] * ddt_scale));
        let eq144_e1830_d_b8: f64 = (p.p7 * (s.db[253][8] * ddt_scale));
        let eq144_e1830_d_b9: f64 = (p.p7 * (s.db[253][9] * ddt_scale));
        let eq144_e1830_d_b10: f64 = (p.p7 * (s.db[253][10] * ddt_scale));
        let eq144_e1830_d_b11: f64 = (p.p7 * (s.db[253][11] * ddt_scale));
        let eq144_e1830_d_b12: f64 = (p.p7 * (s.db[253][12] * ddt_scale));
        let eq144_e1830_d_b13: f64 = (p.p7 * (s.db[253][13] * ddt_scale));
        let eq144_e1830_d_b14: f64 = (p.p7 * (s.db[253][14] * ddt_scale));
        let eq144_e1830_d_b15: f64 = (p.p7 * (s.db[253][15] * ddt_scale));
        let eq144_e1830_d_b16: f64 = (p.p7 * (s.db[253][16] * ddt_scale));
        let eq144_e1830_d_b17: f64 = (p.p7 * (s.db[253][17] * ddt_scale));
        let eq144_e1830_d_b18: f64 = (p.p7 * (s.db[253][18] * ddt_scale));
        let eq144_e1830_d_b19: f64 = (p.p7 * (s.db[253][19] * ddt_scale));
        let eq144_e1830_d_b20: f64 = (p.p7 * (s.db[253][20] * ddt_scale));
        let eq144_e1830_d_b21: f64 = (p.p7 * (s.db[253][21] * ddt_scale));
        let eq144_e1830_d_b22: f64 = (p.p7 * (s.db[253][22] * ddt_scale));
        let eq144_e1830_d_b23: f64 = (p.p7 * (s.db[253][23] * ddt_scale));
        let eq144_e1830_d_b24: f64 = (p.p7 * (s.db[253][24] * ddt_scale));
        let eq144_e1830_d_b25: f64 = (p.p7 * (s.db[253][25] * ddt_scale));
        let eq144_e1830_d_b26: f64 = (p.p7 * (s.db[253][26] * ddt_scale));
        let eq144_e1830_d_b27: f64 = (p.p7 * (s.db[253][27] * ddt_scale));
        let eq144_e1830_d_b28: f64 = (p.p7 * (s.db[253][28] * ddt_scale));
        let eq144_e1830_d_b29: f64 = (p.p7 * (s.db[253][29] * ddt_scale));
        let eq144_e1830_d_b30: f64 = (p.p7 * (s.db[253][30] * ddt_scale));
        let eq144_e1830_d_b31: f64 = (p.p7 * (s.db[253][31] * ddt_scale));
        let eq144_e1830_d_b32: f64 = (p.p7 * (s.db[253][32] * ddt_scale));
        let eq144_e1830_d_b33: f64 = (p.p7 * (s.db[253][33] * ddt_scale));
        let eq144_e1830_d_b34: f64 = (p.p7 * (s.db[253][34] * ddt_scale));
        let eq144_e1830_d_b35: f64 = (p.p7 * (s.db[253][35] * ddt_scale));
        let eq144_e1830_d_b36: f64 = (p.p7 * (s.db[253][36] * ddt_scale));
        let eq144_e1830_d_b37: f64 = (p.p7 * (s.db[253][37] * ddt_scale));
        let eq144_e1830_d_b38: f64 = (p.p7 * (s.db[253][38] * ddt_scale));
        let eq144_e1830_d_b39: f64 = (p.p7 * (s.db[253][39] * ddt_scale));
        let eq144_e1830_d_b40: f64 = (p.p7 * (s.db[253][40] * ddt_scale));
        let eq144_e1830_d_b41: f64 = (p.p7 * (s.db[253][41] * ddt_scale));
        let eq144_e1830_d_b42: f64 = (p.p7 * (s.db[253][42] * ddt_scale));
        let eq144_e1830_d_b43: f64 = (p.p7 * (s.db[253][43] * ddt_scale));
        let eq144_e1830_d_b44: f64 = (p.p7 * (s.db[253][44] * ddt_scale));
        let eq144_e1830_d_b45: f64 = (p.p7 * (s.db[253][45] * ddt_scale));
        let eq144_e1830_d_b46: f64 = (p.p7 * (s.db[253][46] * ddt_scale));
        let eq144_e1830_d_b47: f64 = (p.p7 * (s.db[253][47] * ddt_scale));
        let eq144_e1830_d_b48: f64 = (p.p7 * (s.db[253][48] * ddt_scale));
        let eq144_e1830_d_b49: f64 = (p.p7 * (s.db[253][49] * ddt_scale));
        let eq144_e1830_d_b50: f64 = (p.p7 * (s.db[253][50] * ddt_scale));
        let eq144_e1830_d_b51: f64 = (p.p7 * (s.db[253][51] * ddt_scale));
        let eq144_e1830_d_b52: f64 = (p.p7 * (s.db[253][52] * ddt_scale));
        let eq144_e1830_d_b53: f64 = (p.p7 * (s.db[253][53] * ddt_scale));
        let eq144_e1830_d_b54: f64 = (p.p7 * (s.db[253][54] * ddt_scale));
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22, eq144_e1830_d_b0, eq144_e1830_d_b1, eq144_e1830_d_b2, eq144_e1830_d_b3, eq144_e1830_d_b4, eq144_e1830_d_b5, eq144_e1830_d_b6, eq144_e1830_d_b7, eq144_e1830_d_b8, eq144_e1830_d_b9, eq144_e1830_d_b10, eq144_e1830_d_b11, eq144_e1830_d_b12, eq144_e1830_d_b13, eq144_e1830_d_b14, eq144_e1830_d_b15, eq144_e1830_d_b16, eq144_e1830_d_b17, eq144_e1830_d_b18, eq144_e1830_d_b19, eq144_e1830_d_b20, eq144_e1830_d_b21, eq144_e1830_d_b22, eq144_e1830_d_b23, eq144_e1830_d_b24, eq144_e1830_d_b25, eq144_e1830_d_b26, eq144_e1830_d_b27, eq144_e1830_d_b28, eq144_e1830_d_b29, eq144_e1830_d_b30, eq144_e1830_d_b31, eq144_e1830_d_b32, eq144_e1830_d_b33, eq144_e1830_d_b34, eq144_e1830_d_b35, eq144_e1830_d_b36, eq144_e1830_d_b37, eq144_e1830_d_b38, eq144_e1830_d_b39, eq144_e1830_d_b40, eq144_e1830_d_b41, eq144_e1830_d_b42, eq144_e1830_d_b43, eq144_e1830_d_b44, eq144_e1830_d_b45, eq144_e1830_d_b46, eq144_e1830_d_b47, eq144_e1830_d_b48, eq144_e1830_d_b49, eq144_e1830_d_b50, eq144_e1830_d_b51, eq144_e1830_d_b52, eq144_e1830_d_b53, eq144_e1830_d_b54,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1832;
        let eq144_node_derivatives: [f64; 23] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_branch_derivatives: [f64; 55] = [eq144_e1832_d_b0, eq144_e1832_d_b1, eq144_e1832_d_b2, eq144_e1832_d_b3, eq144_e1832_d_b4, eq144_e1832_d_b5, eq144_e1832_d_b6, eq144_e1832_d_b7, eq144_e1832_d_b8, eq144_e1832_d_b9, eq144_e1832_d_b10, eq144_e1832_d_b11, eq144_e1832_d_b12, eq144_e1832_d_b13, eq144_e1832_d_b14, eq144_e1832_d_b15, eq144_e1832_d_b16, eq144_e1832_d_b17, eq144_e1832_d_b18, eq144_e1832_d_b19, eq144_e1832_d_b20, eq144_e1832_d_b21, eq144_e1832_d_b22, eq144_e1832_d_b23, eq144_e1832_d_b24, eq144_e1832_d_b25, eq144_e1832_d_b26, eq144_e1832_d_b27, eq144_e1832_d_b28, eq144_e1832_d_b29, eq144_e1832_d_b30, eq144_e1832_d_b31, eq144_e1832_d_b32, eq144_e1832_d_b33, eq144_e1832_d_b34, eq144_e1832_d_b35, eq144_e1832_d_b36, eq144_e1832_d_b37, eq144_e1832_d_b38, eq144_e1832_d_b39, eq144_e1832_d_b40, eq144_e1832_d_b41, eq144_e1832_d_b42, eq144_e1832_d_b43, eq144_e1832_d_b44, eq144_e1832_d_b45, eq144_e1832_d_b46, eq144_e1832_d_b47, eq144_e1832_d_b48, eq144_e1832_d_b49, eq144_e1832_d_b50, eq144_e1832_d_b51, eq144_e1832_d_b52, eq144_e1832_d_b53, eq144_e1832_d_b54];
        stamper.stamp_current_dense_local(
            Some(16),
            Some(15),
            multiplicity * (eq144_value),
            &eq144_node_derivatives,
            &eq144_branch_derivatives,
            multiplicity,
        );
    }
}
