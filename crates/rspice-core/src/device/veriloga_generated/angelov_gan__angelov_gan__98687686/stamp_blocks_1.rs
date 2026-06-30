#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let (assign940_e1280, assign940_e1280_d_n0, assign940_e1280_d_n1, assign940_e1280_d_n2, assign940_e1280_d_n3, assign940_e1280_d_n4, assign940_e1280_d_n5, assign940_e1280_d_n6, assign940_e1280_d_n7, assign940_e1280_d_n8, assign940_e1280_d_n9, assign940_e1280_d_n10, assign940_e1280_d_n11, assign940_e1280_d_n12, assign940_e1280_d_n13, assign940_e1280_d_n14, assign940_e1280_d_n15, assign940_e1280_d_n16, assign940_e1280_d_n17, assign940_e1280_d_n18, assign940_e1280_d_b0, assign940_e1280_d_b1, assign940_e1280_d_b2, assign940_e1280_d_b3, assign940_e1280_d_b4, assign940_e1280_d_b5, assign940_e1280_d_b6, assign940_e1280_d_b7, assign940_e1280_d_b8, assign940_e1280_d_b9, assign940_e1280_d_b10, assign940_e1280_d_b11, assign940_e1280_d_b12, assign940_e1280_d_b13, assign940_e1280_d_b14, assign940_e1280_d_b15, assign940_e1280_d_b16, assign940_e1280_d_b17, assign940_e1280_d_b18,) = {
    if (s.b[108] && (!(s.b[106] || s.b[107]))) {
        let assign940_e1277: f64 = (p.p15 * s.v[76]);
        let assign940_e1278: f64 = (p.p14 + assign940_e1277);
        (assign940_e1278, (p.p15 * s.dn[76][0]), (p.p15 * s.dn[76][1]), (p.p15 * s.dn[76][2]), (p.p15 * s.dn[76][3]), (p.p15 * s.dn[76][4]), (p.p15 * s.dn[76][5]), (p.p15 * s.dn[76][6]), (p.p15 * s.dn[76][7]), (p.p15 * s.dn[76][8]), (p.p15 * s.dn[76][9]), (p.p15 * s.dn[76][10]), (p.p15 * s.dn[76][11]), (p.p15 * s.dn[76][12]), (p.p15 * s.dn[76][13]), (p.p15 * s.dn[76][14]), (p.p15 * s.dn[76][15]), (p.p15 * s.dn[76][16]), (p.p15 * s.dn[76][17]), (p.p15 * s.dn[76][18]), (p.p15 * s.db[76][0]), (p.p15 * s.db[76][1]), (p.p15 * s.db[76][2]), (p.p15 * s.db[76][3]), (p.p15 * s.db[76][4]), (p.p15 * s.db[76][5]), (p.p15 * s.db[76][6]), (p.p15 * s.db[76][7]), (p.p15 * s.db[76][8]), (p.p15 * s.db[76][9]), (p.p15 * s.db[76][10]), (p.p15 * s.db[76][11]), (p.p15 * s.db[76][12]), (p.p15 * s.db[76][13]), (p.p15 * s.db[76][14]), (p.p15 * s.db[76][15]), (p.p15 * s.db[76][16]), (p.p15 * s.db[76][17]), (p.p15 * s.db[76][18]),)
    } else {
        (s.v[1], s.dn[1][0], s.dn[1][1], s.dn[1][2], s.dn[1][3], s.dn[1][4], s.dn[1][5], s.dn[1][6], s.dn[1][7], s.dn[1][8], s.dn[1][9], s.dn[1][10], s.dn[1][11], s.dn[1][12], s.dn[1][13], s.dn[1][14], s.dn[1][15], s.dn[1][16], s.dn[1][17], s.dn[1][18], s.db[1][0], s.db[1][1], s.db[1][2], s.db[1][3], s.db[1][4], s.db[1][5], s.db[1][6], s.db[1][7], s.db[1][8], s.db[1][9], s.db[1][10], s.db[1][11], s.db[1][12], s.db[1][13], s.db[1][14], s.db[1][15], s.db[1][16], s.db[1][17], s.db[1][18],)
    }
};
        s.v[1] = assign940_e1280;
        s.mark_derivatives_dirty(1);
        s.dn[1][0] = assign940_e1280_d_n0;
        s.dn[1][1] = assign940_e1280_d_n1;
        s.dn[1][2] = assign940_e1280_d_n2;
        s.dn[1][3] = assign940_e1280_d_n3;
        s.dn[1][4] = assign940_e1280_d_n4;
        s.dn[1][5] = assign940_e1280_d_n5;
        s.dn[1][6] = assign940_e1280_d_n6;
        s.dn[1][7] = assign940_e1280_d_n7;
        s.dn[1][8] = assign940_e1280_d_n8;
        s.dn[1][9] = assign940_e1280_d_n9;
        s.dn[1][10] = assign940_e1280_d_n10;
        s.dn[1][11] = assign940_e1280_d_n11;
        s.dn[1][12] = assign940_e1280_d_n12;
        s.dn[1][13] = assign940_e1280_d_n13;
        s.dn[1][14] = assign940_e1280_d_n14;
        s.dn[1][15] = assign940_e1280_d_n15;
        s.dn[1][16] = assign940_e1280_d_n16;
        s.dn[1][17] = assign940_e1280_d_n17;
        s.dn[1][18] = assign940_e1280_d_n18;
        s.db[1][0] = assign940_e1280_d_b0;
        s.db[1][1] = assign940_e1280_d_b1;
        s.db[1][2] = assign940_e1280_d_b2;
        s.db[1][3] = assign940_e1280_d_b3;
        s.db[1][4] = assign940_e1280_d_b4;
        s.db[1][5] = assign940_e1280_d_b5;
        s.db[1][6] = assign940_e1280_d_b6;
        s.db[1][7] = assign940_e1280_d_b7;
        s.db[1][8] = assign940_e1280_d_b8;
        s.db[1][9] = assign940_e1280_d_b9;
        s.db[1][10] = assign940_e1280_d_b10;
        s.db[1][11] = assign940_e1280_d_b11;
        s.db[1][12] = assign940_e1280_d_b12;
        s.db[1][13] = assign940_e1280_d_b13;
        s.db[1][14] = assign940_e1280_d_b14;
        s.db[1][15] = assign940_e1280_d_b15;
        s.db[1][16] = assign940_e1280_d_b16;
        s.db[1][17] = assign940_e1280_d_b17;
        s.db[1][18] = assign940_e1280_d_b18;
        s.rv[1] = 0.0;

        let (assign950_e1292, assign950_e1292_d_n0, assign950_e1292_d_n1, assign950_e1292_d_n2, assign950_e1292_d_n3, assign950_e1292_d_n4, assign950_e1292_d_n5, assign950_e1292_d_n6, assign950_e1292_d_n7, assign950_e1292_d_n8, assign950_e1292_d_n9, assign950_e1292_d_n10, assign950_e1292_d_n11, assign950_e1292_d_n12, assign950_e1292_d_n13, assign950_e1292_d_n14, assign950_e1292_d_n15, assign950_e1292_d_n16, assign950_e1292_d_n17, assign950_e1292_d_n18, assign950_e1292_d_b0, assign950_e1292_d_b1, assign950_e1292_d_b2, assign950_e1292_d_b3, assign950_e1292_d_b4, assign950_e1292_d_b5, assign950_e1292_d_b6, assign950_e1292_d_b7, assign950_e1292_d_b8, assign950_e1292_d_b9, assign950_e1292_d_b10, assign950_e1292_d_b11, assign950_e1292_d_b12, assign950_e1292_d_b13, assign950_e1292_d_b14, assign950_e1292_d_b15, assign950_e1292_d_b16, assign950_e1292_d_b17, assign950_e1292_d_b18,) = {
    if (s.b[108] && (!(s.b[106] || s.b[107]))) {
        let assign950_e1289: f64 = (s.v[1] * s.v[5]);
        let assign950_e1290: f64 = (assign950_e1289).tanh();
        (assign950_e1290, (((s.dn[1][0] * s.v[5]) + (s.v[1] * s.dn[5][0])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][1] * s.v[5]) + (s.v[1] * s.dn[5][1])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][2] * s.v[5]) + (s.v[1] * s.dn[5][2])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][3] * s.v[5]) + (s.v[1] * s.dn[5][3])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][4] * s.v[5]) + (s.v[1] * s.dn[5][4])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][5] * s.v[5]) + (s.v[1] * s.dn[5][5])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][6] * s.v[5]) + (s.v[1] * s.dn[5][6])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][7] * s.v[5]) + (s.v[1] * s.dn[5][7])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][8] * s.v[5]) + (s.v[1] * s.dn[5][8])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][9] * s.v[5]) + (s.v[1] * s.dn[5][9])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][10] * s.v[5]) + (s.v[1] * s.dn[5][10])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][11] * s.v[5]) + (s.v[1] * s.dn[5][11])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][12] * s.v[5]) + (s.v[1] * s.dn[5][12])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][13] * s.v[5]) + (s.v[1] * s.dn[5][13])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][14] * s.v[5]) + (s.v[1] * s.dn[5][14])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][15] * s.v[5]) + (s.v[1] * s.dn[5][15])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][16] * s.v[5]) + (s.v[1] * s.dn[5][16])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][17] * s.v[5]) + (s.v[1] * s.dn[5][17])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.dn[1][18] * s.v[5]) + (s.v[1] * s.dn[5][18])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][0] * s.v[5]) + (s.v[1] * s.db[5][0])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][1] * s.v[5]) + (s.v[1] * s.db[5][1])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][2] * s.v[5]) + (s.v[1] * s.db[5][2])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][3] * s.v[5]) + (s.v[1] * s.db[5][3])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][4] * s.v[5]) + (s.v[1] * s.db[5][4])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][5] * s.v[5]) + (s.v[1] * s.db[5][5])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][6] * s.v[5]) + (s.v[1] * s.db[5][6])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][7] * s.v[5]) + (s.v[1] * s.db[5][7])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][8] * s.v[5]) + (s.v[1] * s.db[5][8])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][9] * s.v[5]) + (s.v[1] * s.db[5][9])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][10] * s.v[5]) + (s.v[1] * s.db[5][10])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][11] * s.v[5]) + (s.v[1] * s.db[5][11])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][12] * s.v[5]) + (s.v[1] * s.db[5][12])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][13] * s.v[5]) + (s.v[1] * s.db[5][13])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][14] * s.v[5]) + (s.v[1] * s.db[5][14])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][15] * s.v[5]) + (s.v[1] * s.db[5][15])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][16] * s.v[5]) + (s.v[1] * s.db[5][16])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][17] * s.v[5]) + (s.v[1] * s.db[5][17])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())), (((s.db[1][18] * s.v[5]) + (s.v[1] * s.db[5][18])) / ((assign950_e1289).cosh() * (assign950_e1289).cosh())),)
    } else {
        (s.v[81], s.dn[81][0], s.dn[81][1], s.dn[81][2], s.dn[81][3], s.dn[81][4], s.dn[81][5], s.dn[81][6], s.dn[81][7], s.dn[81][8], s.dn[81][9], s.dn[81][10], s.dn[81][11], s.dn[81][12], s.dn[81][13], s.dn[81][14], s.dn[81][15], s.dn[81][16], s.dn[81][17], s.dn[81][18], s.db[81][0], s.db[81][1], s.db[81][2], s.db[81][3], s.db[81][4], s.db[81][5], s.db[81][6], s.db[81][7], s.db[81][8], s.db[81][9], s.db[81][10], s.db[81][11], s.db[81][12], s.db[81][13], s.db[81][14], s.db[81][15], s.db[81][16], s.db[81][17], s.db[81][18],)
    }
};
        s.v[81] = assign950_e1292;
        s.mark_derivatives_dirty(81);
        s.dn[81][0] = assign950_e1292_d_n0;
        s.dn[81][1] = assign950_e1292_d_n1;
        s.dn[81][2] = assign950_e1292_d_n2;
        s.dn[81][3] = assign950_e1292_d_n3;
        s.dn[81][4] = assign950_e1292_d_n4;
        s.dn[81][5] = assign950_e1292_d_n5;
        s.dn[81][6] = assign950_e1292_d_n6;
        s.dn[81][7] = assign950_e1292_d_n7;
        s.dn[81][8] = assign950_e1292_d_n8;
        s.dn[81][9] = assign950_e1292_d_n9;
        s.dn[81][10] = assign950_e1292_d_n10;
        s.dn[81][11] = assign950_e1292_d_n11;
        s.dn[81][12] = assign950_e1292_d_n12;
        s.dn[81][13] = assign950_e1292_d_n13;
        s.dn[81][14] = assign950_e1292_d_n14;
        s.dn[81][15] = assign950_e1292_d_n15;
        s.dn[81][16] = assign950_e1292_d_n16;
        s.dn[81][17] = assign950_e1292_d_n17;
        s.dn[81][18] = assign950_e1292_d_n18;
        s.db[81][0] = assign950_e1292_d_b0;
        s.db[81][1] = assign950_e1292_d_b1;
        s.db[81][2] = assign950_e1292_d_b2;
        s.db[81][3] = assign950_e1292_d_b3;
        s.db[81][4] = assign950_e1292_d_b4;
        s.db[81][5] = assign950_e1292_d_b5;
        s.db[81][6] = assign950_e1292_d_b6;
        s.db[81][7] = assign950_e1292_d_b7;
        s.db[81][8] = assign950_e1292_d_b8;
        s.db[81][9] = assign950_e1292_d_b9;
        s.db[81][10] = assign950_e1292_d_b10;
        s.db[81][11] = assign950_e1292_d_b11;
        s.db[81][12] = assign950_e1292_d_b12;
        s.db[81][13] = assign950_e1292_d_b13;
        s.db[81][14] = assign950_e1292_d_b14;
        s.db[81][15] = assign950_e1292_d_b15;
        s.db[81][16] = assign950_e1292_d_b16;
        s.db[81][17] = assign950_e1292_d_b17;
        s.db[81][18] = assign950_e1292_d_b18;
        s.rv[81] = 0.0;

        let (assign960_e1305, assign960_e1305_d_n0, assign960_e1305_d_n1, assign960_e1305_d_n2, assign960_e1305_d_n3, assign960_e1305_d_n4, assign960_e1305_d_n5, assign960_e1305_d_n6, assign960_e1305_d_n7, assign960_e1305_d_n8, assign960_e1305_d_n9, assign960_e1305_d_n10, assign960_e1305_d_n11, assign960_e1305_d_n12, assign960_e1305_d_n13, assign960_e1305_d_n14, assign960_e1305_d_n15, assign960_e1305_d_n16, assign960_e1305_d_n17, assign960_e1305_d_n18, assign960_e1305_d_b0, assign960_e1305_d_b1, assign960_e1305_d_b2, assign960_e1305_d_b3, assign960_e1305_d_b4, assign960_e1305_d_b5, assign960_e1305_d_b6, assign960_e1305_d_b7, assign960_e1305_d_b8, assign960_e1305_d_b9, assign960_e1305_d_b10, assign960_e1305_d_b11, assign960_e1305_d_b12, assign960_e1305_d_b13, assign960_e1305_d_b14, assign960_e1305_d_b15, assign960_e1305_d_b16, assign960_e1305_d_b17, assign960_e1305_d_b18,) = {
    if (s.b[108] && (!(s.b[106] || s.b[107]))) {
        let assign960_e1302: f64 = (p.p17 * s.v[76]);
        let assign960_e1303: f64 = (p.p16 + assign960_e1302);
        (assign960_e1303, (p.p17 * s.dn[76][0]), (p.p17 * s.dn[76][1]), (p.p17 * s.dn[76][2]), (p.p17 * s.dn[76][3]), (p.p17 * s.dn[76][4]), (p.p17 * s.dn[76][5]), (p.p17 * s.dn[76][6]), (p.p17 * s.dn[76][7]), (p.p17 * s.dn[76][8]), (p.p17 * s.dn[76][9]), (p.p17 * s.dn[76][10]), (p.p17 * s.dn[76][11]), (p.p17 * s.dn[76][12]), (p.p17 * s.dn[76][13]), (p.p17 * s.dn[76][14]), (p.p17 * s.dn[76][15]), (p.p17 * s.dn[76][16]), (p.p17 * s.dn[76][17]), (p.p17 * s.dn[76][18]), (p.p17 * s.db[76][0]), (p.p17 * s.db[76][1]), (p.p17 * s.db[76][2]), (p.p17 * s.db[76][3]), (p.p17 * s.db[76][4]), (p.p17 * s.db[76][5]), (p.p17 * s.db[76][6]), (p.p17 * s.db[76][7]), (p.p17 * s.db[76][8]), (p.p17 * s.db[76][9]), (p.p17 * s.db[76][10]), (p.p17 * s.db[76][11]), (p.p17 * s.db[76][12]), (p.p17 * s.db[76][13]), (p.p17 * s.db[76][14]), (p.p17 * s.db[76][15]), (p.p17 * s.db[76][16]), (p.p17 * s.db[76][17]), (p.p17 * s.db[76][18]),)
    } else {
        (s.v[69], s.dn[69][0], s.dn[69][1], s.dn[69][2], s.dn[69][3], s.dn[69][4], s.dn[69][5], s.dn[69][6], s.dn[69][7], s.dn[69][8], s.dn[69][9], s.dn[69][10], s.dn[69][11], s.dn[69][12], s.dn[69][13], s.dn[69][14], s.dn[69][15], s.dn[69][16], s.dn[69][17], s.dn[69][18], s.db[69][0], s.db[69][1], s.db[69][2], s.db[69][3], s.db[69][4], s.db[69][5], s.db[69][6], s.db[69][7], s.db[69][8], s.db[69][9], s.db[69][10], s.db[69][11], s.db[69][12], s.db[69][13], s.db[69][14], s.db[69][15], s.db[69][16], s.db[69][17], s.db[69][18],)
    }
};
        s.v[69] = assign960_e1305;
        s.mark_derivatives_dirty(69);
        s.dn[69][0] = assign960_e1305_d_n0;
        s.dn[69][1] = assign960_e1305_d_n1;
        s.dn[69][2] = assign960_e1305_d_n2;
        s.dn[69][3] = assign960_e1305_d_n3;
        s.dn[69][4] = assign960_e1305_d_n4;
        s.dn[69][5] = assign960_e1305_d_n5;
        s.dn[69][6] = assign960_e1305_d_n6;
        s.dn[69][7] = assign960_e1305_d_n7;
        s.dn[69][8] = assign960_e1305_d_n8;
        s.dn[69][9] = assign960_e1305_d_n9;
        s.dn[69][10] = assign960_e1305_d_n10;
        s.dn[69][11] = assign960_e1305_d_n11;
        s.dn[69][12] = assign960_e1305_d_n12;
        s.dn[69][13] = assign960_e1305_d_n13;
        s.dn[69][14] = assign960_e1305_d_n14;
        s.dn[69][15] = assign960_e1305_d_n15;
        s.dn[69][16] = assign960_e1305_d_n16;
        s.dn[69][17] = assign960_e1305_d_n17;
        s.dn[69][18] = assign960_e1305_d_n18;
        s.db[69][0] = assign960_e1305_d_b0;
        s.db[69][1] = assign960_e1305_d_b1;
        s.db[69][2] = assign960_e1305_d_b2;
        s.db[69][3] = assign960_e1305_d_b3;
        s.db[69][4] = assign960_e1305_d_b4;
        s.db[69][5] = assign960_e1305_d_b5;
        s.db[69][6] = assign960_e1305_d_b6;
        s.db[69][7] = assign960_e1305_d_b7;
        s.db[69][8] = assign960_e1305_d_b8;
        s.db[69][9] = assign960_e1305_d_b9;
        s.db[69][10] = assign960_e1305_d_b10;
        s.db[69][11] = assign960_e1305_d_b11;
        s.db[69][12] = assign960_e1305_d_b12;
        s.db[69][13] = assign960_e1305_d_b13;
        s.db[69][14] = assign960_e1305_d_b14;
        s.db[69][15] = assign960_e1305_d_b15;
        s.db[69][16] = assign960_e1305_d_b16;
        s.db[69][17] = assign960_e1305_d_b17;
        s.db[69][18] = assign960_e1305_d_b18;
        s.rv[69] = 0.0;

        let (assign970_e1333, assign970_e1333_d_n0, assign970_e1333_d_n1, assign970_e1333_d_n2, assign970_e1333_d_n3, assign970_e1333_d_n4, assign970_e1333_d_n5, assign970_e1333_d_n6, assign970_e1333_d_n7, assign970_e1333_d_n8, assign970_e1333_d_n9, assign970_e1333_d_n10, assign970_e1333_d_n11, assign970_e1333_d_n12, assign970_e1333_d_n13, assign970_e1333_d_n14, assign970_e1333_d_n15, assign970_e1333_d_n16, assign970_e1333_d_n17, assign970_e1333_d_n18, assign970_e1333_d_b0, assign970_e1333_d_b1, assign970_e1333_d_b2, assign970_e1333_d_b3, assign970_e1333_d_b4, assign970_e1333_d_b5, assign970_e1333_d_b6, assign970_e1333_d_b7, assign970_e1333_d_b8, assign970_e1333_d_b9, assign970_e1333_d_b10, assign970_e1333_d_b11, assign970_e1333_d_b12, assign970_e1333_d_b13, assign970_e1333_d_b14, assign970_e1333_d_b15, assign970_e1333_d_b16, assign970_e1333_d_b17, assign970_e1333_d_b18,) = {
    if (s.b[108] && (!(s.b[106] || s.b[107]))) {
        let assign970_e1314: f64 = (s.v[39] * s.v[76]);
        let assign970_e1316: f64 = (assign970_e1314 * s.v[81]);
        let assign970_e1320: f64 = (s.v[69] * s.v[5]);
        let assign970_e1321: f64 = (1.0 + assign970_e1320);
        let assign970_e1326: f64 = (s.v[6] - s.v[53]);
        let assign970_e1327: f64 = (p.p23 * assign970_e1326);
        let assign970_e1328: f64 = { let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign970_e1329: f64 = (s.v[43] * assign970_e1328);
        let assign970_e1330: f64 = (assign970_e1321 + assign970_e1329);
        let assign970_e1331: f64 = (assign970_e1316 * assign970_e1330);
        (assign970_e1331, ((((((s.dn[39][0] * s.v[76]) + (s.v[39] * s.dn[76][0])) * s.v[81]) + (assign970_e1314 * s.dn[81][0])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][0] * s.v[5]) + (s.v[69] * s.dn[5][0])) + ((s.dn[43][0] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][0] - s.dn[53][0])))))))), ((((((s.dn[39][1] * s.v[76]) + (s.v[39] * s.dn[76][1])) * s.v[81]) + (assign970_e1314 * s.dn[81][1])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][1] * s.v[5]) + (s.v[69] * s.dn[5][1])) + ((s.dn[43][1] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][1] - s.dn[53][1])))))))), ((((((s.dn[39][2] * s.v[76]) + (s.v[39] * s.dn[76][2])) * s.v[81]) + (assign970_e1314 * s.dn[81][2])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][2] * s.v[5]) + (s.v[69] * s.dn[5][2])) + ((s.dn[43][2] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][2] - s.dn[53][2])))))))), ((((((s.dn[39][3] * s.v[76]) + (s.v[39] * s.dn[76][3])) * s.v[81]) + (assign970_e1314 * s.dn[81][3])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][3] * s.v[5]) + (s.v[69] * s.dn[5][3])) + ((s.dn[43][3] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][3] - s.dn[53][3])))))))), ((((((s.dn[39][4] * s.v[76]) + (s.v[39] * s.dn[76][4])) * s.v[81]) + (assign970_e1314 * s.dn[81][4])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][4] * s.v[5]) + (s.v[69] * s.dn[5][4])) + ((s.dn[43][4] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][4] - s.dn[53][4])))))))), ((((((s.dn[39][5] * s.v[76]) + (s.v[39] * s.dn[76][5])) * s.v[81]) + (assign970_e1314 * s.dn[81][5])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][5] * s.v[5]) + (s.v[69] * s.dn[5][5])) + ((s.dn[43][5] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][5] - s.dn[53][5])))))))), ((((((s.dn[39][6] * s.v[76]) + (s.v[39] * s.dn[76][6])) * s.v[81]) + (assign970_e1314 * s.dn[81][6])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][6] * s.v[5]) + (s.v[69] * s.dn[5][6])) + ((s.dn[43][6] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][6] - s.dn[53][6])))))))), ((((((s.dn[39][7] * s.v[76]) + (s.v[39] * s.dn[76][7])) * s.v[81]) + (assign970_e1314 * s.dn[81][7])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][7] * s.v[5]) + (s.v[69] * s.dn[5][7])) + ((s.dn[43][7] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][7] - s.dn[53][7])))))))), ((((((s.dn[39][8] * s.v[76]) + (s.v[39] * s.dn[76][8])) * s.v[81]) + (assign970_e1314 * s.dn[81][8])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][8] * s.v[5]) + (s.v[69] * s.dn[5][8])) + ((s.dn[43][8] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][8] - s.dn[53][8])))))))), ((((((s.dn[39][9] * s.v[76]) + (s.v[39] * s.dn[76][9])) * s.v[81]) + (assign970_e1314 * s.dn[81][9])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][9] * s.v[5]) + (s.v[69] * s.dn[5][9])) + ((s.dn[43][9] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][9] - s.dn[53][9])))))))), ((((((s.dn[39][10] * s.v[76]) + (s.v[39] * s.dn[76][10])) * s.v[81]) + (assign970_e1314 * s.dn[81][10])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][10] * s.v[5]) + (s.v[69] * s.dn[5][10])) + ((s.dn[43][10] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][10] - s.dn[53][10])))))))), ((((((s.dn[39][11] * s.v[76]) + (s.v[39] * s.dn[76][11])) * s.v[81]) + (assign970_e1314 * s.dn[81][11])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][11] * s.v[5]) + (s.v[69] * s.dn[5][11])) + ((s.dn[43][11] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][11] - s.dn[53][11])))))))), ((((((s.dn[39][12] * s.v[76]) + (s.v[39] * s.dn[76][12])) * s.v[81]) + (assign970_e1314 * s.dn[81][12])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][12] * s.v[5]) + (s.v[69] * s.dn[5][12])) + ((s.dn[43][12] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][12] - s.dn[53][12])))))))), ((((((s.dn[39][13] * s.v[76]) + (s.v[39] * s.dn[76][13])) * s.v[81]) + (assign970_e1314 * s.dn[81][13])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][13] * s.v[5]) + (s.v[69] * s.dn[5][13])) + ((s.dn[43][13] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][13] - s.dn[53][13])))))))), ((((((s.dn[39][14] * s.v[76]) + (s.v[39] * s.dn[76][14])) * s.v[81]) + (assign970_e1314 * s.dn[81][14])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][14] * s.v[5]) + (s.v[69] * s.dn[5][14])) + ((s.dn[43][14] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][14] - s.dn[53][14])))))))), ((((((s.dn[39][15] * s.v[76]) + (s.v[39] * s.dn[76][15])) * s.v[81]) + (assign970_e1314 * s.dn[81][15])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][15] * s.v[5]) + (s.v[69] * s.dn[5][15])) + ((s.dn[43][15] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][15] - s.dn[53][15])))))))), ((((((s.dn[39][16] * s.v[76]) + (s.v[39] * s.dn[76][16])) * s.v[81]) + (assign970_e1314 * s.dn[81][16])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][16] * s.v[5]) + (s.v[69] * s.dn[5][16])) + ((s.dn[43][16] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][16] - s.dn[53][16])))))))), ((((((s.dn[39][17] * s.v[76]) + (s.v[39] * s.dn[76][17])) * s.v[81]) + (assign970_e1314 * s.dn[81][17])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][17] * s.v[5]) + (s.v[69] * s.dn[5][17])) + ((s.dn[43][17] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][17] - s.dn[53][17])))))))), ((((((s.dn[39][18] * s.v[76]) + (s.v[39] * s.dn[76][18])) * s.v[81]) + (assign970_e1314 * s.dn[81][18])) * assign970_e1330) + (assign970_e1316 * (((s.dn[69][18] * s.v[5]) + (s.v[69] * s.dn[5][18])) + ((s.dn[43][18] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[6][18] - s.dn[53][18])))))))), ((((((s.db[39][0] * s.v[76]) + (s.v[39] * s.db[76][0])) * s.v[81]) + (assign970_e1314 * s.db[81][0])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][0] * s.v[5]) + (s.v[69] * s.db[5][0])) + ((s.db[43][0] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][0] - s.db[53][0])))))))), ((((((s.db[39][1] * s.v[76]) + (s.v[39] * s.db[76][1])) * s.v[81]) + (assign970_e1314 * s.db[81][1])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][1] * s.v[5]) + (s.v[69] * s.db[5][1])) + ((s.db[43][1] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][1] - s.db[53][1])))))))), ((((((s.db[39][2] * s.v[76]) + (s.v[39] * s.db[76][2])) * s.v[81]) + (assign970_e1314 * s.db[81][2])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][2] * s.v[5]) + (s.v[69] * s.db[5][2])) + ((s.db[43][2] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][2] - s.db[53][2])))))))), ((((((s.db[39][3] * s.v[76]) + (s.v[39] * s.db[76][3])) * s.v[81]) + (assign970_e1314 * s.db[81][3])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][3] * s.v[5]) + (s.v[69] * s.db[5][3])) + ((s.db[43][3] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][3] - s.db[53][3])))))))), ((((((s.db[39][4] * s.v[76]) + (s.v[39] * s.db[76][4])) * s.v[81]) + (assign970_e1314 * s.db[81][4])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][4] * s.v[5]) + (s.v[69] * s.db[5][4])) + ((s.db[43][4] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][4] - s.db[53][4])))))))), ((((((s.db[39][5] * s.v[76]) + (s.v[39] * s.db[76][5])) * s.v[81]) + (assign970_e1314 * s.db[81][5])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][5] * s.v[5]) + (s.v[69] * s.db[5][5])) + ((s.db[43][5] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][5] - s.db[53][5])))))))), ((((((s.db[39][6] * s.v[76]) + (s.v[39] * s.db[76][6])) * s.v[81]) + (assign970_e1314 * s.db[81][6])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][6] * s.v[5]) + (s.v[69] * s.db[5][6])) + ((s.db[43][6] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][6] - s.db[53][6])))))))), ((((((s.db[39][7] * s.v[76]) + (s.v[39] * s.db[76][7])) * s.v[81]) + (assign970_e1314 * s.db[81][7])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][7] * s.v[5]) + (s.v[69] * s.db[5][7])) + ((s.db[43][7] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][7] - s.db[53][7])))))))), ((((((s.db[39][8] * s.v[76]) + (s.v[39] * s.db[76][8])) * s.v[81]) + (assign970_e1314 * s.db[81][8])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][8] * s.v[5]) + (s.v[69] * s.db[5][8])) + ((s.db[43][8] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][8] - s.db[53][8])))))))), ((((((s.db[39][9] * s.v[76]) + (s.v[39] * s.db[76][9])) * s.v[81]) + (assign970_e1314 * s.db[81][9])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][9] * s.v[5]) + (s.v[69] * s.db[5][9])) + ((s.db[43][9] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][9] - s.db[53][9])))))))), ((((((s.db[39][10] * s.v[76]) + (s.v[39] * s.db[76][10])) * s.v[81]) + (assign970_e1314 * s.db[81][10])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][10] * s.v[5]) + (s.v[69] * s.db[5][10])) + ((s.db[43][10] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][10] - s.db[53][10])))))))), ((((((s.db[39][11] * s.v[76]) + (s.v[39] * s.db[76][11])) * s.v[81]) + (assign970_e1314 * s.db[81][11])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][11] * s.v[5]) + (s.v[69] * s.db[5][11])) + ((s.db[43][11] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][11] - s.db[53][11])))))))), ((((((s.db[39][12] * s.v[76]) + (s.v[39] * s.db[76][12])) * s.v[81]) + (assign970_e1314 * s.db[81][12])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][12] * s.v[5]) + (s.v[69] * s.db[5][12])) + ((s.db[43][12] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][12] - s.db[53][12])))))))), ((((((s.db[39][13] * s.v[76]) + (s.v[39] * s.db[76][13])) * s.v[81]) + (assign970_e1314 * s.db[81][13])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][13] * s.v[5]) + (s.v[69] * s.db[5][13])) + ((s.db[43][13] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][13] - s.db[53][13])))))))), ((((((s.db[39][14] * s.v[76]) + (s.v[39] * s.db[76][14])) * s.v[81]) + (assign970_e1314 * s.db[81][14])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][14] * s.v[5]) + (s.v[69] * s.db[5][14])) + ((s.db[43][14] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][14] - s.db[53][14])))))))), ((((((s.db[39][15] * s.v[76]) + (s.v[39] * s.db[76][15])) * s.v[81]) + (assign970_e1314 * s.db[81][15])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][15] * s.v[5]) + (s.v[69] * s.db[5][15])) + ((s.db[43][15] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][15] - s.db[53][15])))))))), ((((((s.db[39][16] * s.v[76]) + (s.v[39] * s.db[76][16])) * s.v[81]) + (assign970_e1314 * s.db[81][16])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][16] * s.v[5]) + (s.v[69] * s.db[5][16])) + ((s.db[43][16] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][16] - s.db[53][16])))))))), ((((((s.db[39][17] * s.v[76]) + (s.v[39] * s.db[76][17])) * s.v[81]) + (assign970_e1314 * s.db[81][17])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][17] * s.v[5]) + (s.v[69] * s.db[5][17])) + ((s.db[43][17] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][17] - s.db[53][17])))))))), ((((((s.db[39][18] * s.v[76]) + (s.v[39] * s.db[76][18])) * s.v[81]) + (assign970_e1314 * s.db[81][18])) * assign970_e1330) + (assign970_e1316 * (((s.db[69][18] * s.v[5]) + (s.v[69] * s.db[5][18])) + ((s.db[43][18] * assign970_e1328) + (s.v[43] * ({ let limexp_arg = assign970_e1327; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[6][18] - s.db[53][18])))))))),)
    } else {
        (s.v[98], s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5], s.dn[98][6], s.dn[98][7], s.dn[98][8], s.dn[98][9], s.dn[98][10], s.dn[98][11], s.dn[98][12], s.dn[98][13], s.dn[98][14], s.dn[98][15], s.dn[98][16], s.dn[98][17], s.dn[98][18], s.db[98][0], s.db[98][1], s.db[98][2], s.db[98][3], s.db[98][4], s.db[98][5], s.db[98][6], s.db[98][7], s.db[98][8], s.db[98][9], s.db[98][10], s.db[98][11], s.db[98][12], s.db[98][13], s.db[98][14], s.db[98][15], s.db[98][16], s.db[98][17], s.db[98][18],)
    }
};
        s.v[98] = assign970_e1333;
        s.mark_derivatives_dirty(98);
        s.dn[98][0] = assign970_e1333_d_n0;
        s.dn[98][1] = assign970_e1333_d_n1;
        s.dn[98][2] = assign970_e1333_d_n2;
        s.dn[98][3] = assign970_e1333_d_n3;
        s.dn[98][4] = assign970_e1333_d_n4;
        s.dn[98][5] = assign970_e1333_d_n5;
        s.dn[98][6] = assign970_e1333_d_n6;
        s.dn[98][7] = assign970_e1333_d_n7;
        s.dn[98][8] = assign970_e1333_d_n8;
        s.dn[98][9] = assign970_e1333_d_n9;
        s.dn[98][10] = assign970_e1333_d_n10;
        s.dn[98][11] = assign970_e1333_d_n11;
        s.dn[98][12] = assign970_e1333_d_n12;
        s.dn[98][13] = assign970_e1333_d_n13;
        s.dn[98][14] = assign970_e1333_d_n14;
        s.dn[98][15] = assign970_e1333_d_n15;
        s.dn[98][16] = assign970_e1333_d_n16;
        s.dn[98][17] = assign970_e1333_d_n17;
        s.dn[98][18] = assign970_e1333_d_n18;
        s.db[98][0] = assign970_e1333_d_b0;
        s.db[98][1] = assign970_e1333_d_b1;
        s.db[98][2] = assign970_e1333_d_b2;
        s.db[98][3] = assign970_e1333_d_b3;
        s.db[98][4] = assign970_e1333_d_b4;
        s.db[98][5] = assign970_e1333_d_b5;
        s.db[98][6] = assign970_e1333_d_b6;
        s.db[98][7] = assign970_e1333_d_b7;
        s.db[98][8] = assign970_e1333_d_b8;
        s.db[98][9] = assign970_e1333_d_b9;
        s.db[98][10] = assign970_e1333_d_b10;
        s.db[98][11] = assign970_e1333_d_b11;
        s.db[98][12] = assign970_e1333_d_b12;
        s.db[98][13] = assign970_e1333_d_b13;
        s.db[98][14] = assign970_e1333_d_b14;
        s.db[98][15] = assign970_e1333_d_b15;
        s.db[98][16] = assign970_e1333_d_b16;
        s.db[98][17] = assign970_e1333_d_b17;
        s.db[98][18] = assign970_e1333_d_b18;
        s.rv[98] = 0.0;

        let (assign980_e1346, assign980_e1346_d_n0, assign980_e1346_d_n1, assign980_e1346_d_n2, assign980_e1346_d_n3, assign980_e1346_d_n4, assign980_e1346_d_n5, assign980_e1346_d_n6, assign980_e1346_d_n7, assign980_e1346_d_n8, assign980_e1346_d_n9, assign980_e1346_d_n10, assign980_e1346_d_n11, assign980_e1346_d_n12, assign980_e1346_d_n13, assign980_e1346_d_n14, assign980_e1346_d_n15, assign980_e1346_d_n16, assign980_e1346_d_n17, assign980_e1346_d_n18, assign980_e1346_d_b0, assign980_e1346_d_b1, assign980_e1346_d_b2, assign980_e1346_d_b3, assign980_e1346_d_b4, assign980_e1346_d_b5, assign980_e1346_d_b6, assign980_e1346_d_b7, assign980_e1346_d_b8, assign980_e1346_d_b9, assign980_e1346_d_b10, assign980_e1346_d_b11, assign980_e1346_d_b12, assign980_e1346_d_b13, assign980_e1346_d_b14, assign980_e1346_d_b15, assign980_e1346_d_b16, assign980_e1346_d_b17, assign980_e1346_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign980_e1344: f64 = (s.v[3] - s.v[58]);
        (assign980_e1344, (s.dn[3][0] - s.dn[58][0]), (s.dn[3][1] - s.dn[58][1]), (s.dn[3][2] - s.dn[58][2]), (s.dn[3][3] - s.dn[58][3]), (s.dn[3][4] - s.dn[58][4]), (s.dn[3][5] - s.dn[58][5]), (s.dn[3][6] - s.dn[58][6]), (s.dn[3][7] - s.dn[58][7]), (s.dn[3][8] - s.dn[58][8]), (s.dn[3][9] - s.dn[58][9]), (s.dn[3][10] - s.dn[58][10]), (s.dn[3][11] - s.dn[58][11]), (s.dn[3][12] - s.dn[58][12]), (s.dn[3][13] - s.dn[58][13]), (s.dn[3][14] - s.dn[58][14]), (s.dn[3][15] - s.dn[58][15]), (s.dn[3][16] - s.dn[58][16]), (s.dn[3][17] - s.dn[58][17]), (s.dn[3][18] - s.dn[58][18]), (s.db[3][0] - s.db[58][0]), (s.db[3][1] - s.db[58][1]), (s.db[3][2] - s.db[58][2]), (s.db[3][3] - s.db[58][3]), (s.db[3][4] - s.db[58][4]), (s.db[3][5] - s.db[58][5]), (s.db[3][6] - s.db[58][6]), (s.db[3][7] - s.db[58][7]), (s.db[3][8] - s.db[58][8]), (s.db[3][9] - s.db[58][9]), (s.db[3][10] - s.db[58][10]), (s.db[3][11] - s.db[58][11]), (s.db[3][12] - s.db[58][12]), (s.db[3][13] - s.db[58][13]), (s.db[3][14] - s.db[58][14]), (s.db[3][15] - s.db[58][15]), (s.db[3][16] - s.db[58][16]), (s.db[3][17] - s.db[58][17]), (s.db[3][18] - s.db[58][18]),)
    } else {
        (s.v[63], s.dn[63][0], s.dn[63][1], s.dn[63][2], s.dn[63][3], s.dn[63][4], s.dn[63][5], s.dn[63][6], s.dn[63][7], s.dn[63][8], s.dn[63][9], s.dn[63][10], s.dn[63][11], s.dn[63][12], s.dn[63][13], s.dn[63][14], s.dn[63][15], s.dn[63][16], s.dn[63][17], s.dn[63][18], s.db[63][0], s.db[63][1], s.db[63][2], s.db[63][3], s.db[63][4], s.db[63][5], s.db[63][6], s.db[63][7], s.db[63][8], s.db[63][9], s.db[63][10], s.db[63][11], s.db[63][12], s.db[63][13], s.db[63][14], s.db[63][15], s.db[63][16], s.db[63][17], s.db[63][18],)
    }
};
        s.v[63] = assign980_e1346;
        s.mark_derivatives_dirty(63);
        s.dn[63][0] = assign980_e1346_d_n0;
        s.dn[63][1] = assign980_e1346_d_n1;
        s.dn[63][2] = assign980_e1346_d_n2;
        s.dn[63][3] = assign980_e1346_d_n3;
        s.dn[63][4] = assign980_e1346_d_n4;
        s.dn[63][5] = assign980_e1346_d_n5;
        s.dn[63][6] = assign980_e1346_d_n6;
        s.dn[63][7] = assign980_e1346_d_n7;
        s.dn[63][8] = assign980_e1346_d_n8;
        s.dn[63][9] = assign980_e1346_d_n9;
        s.dn[63][10] = assign980_e1346_d_n10;
        s.dn[63][11] = assign980_e1346_d_n11;
        s.dn[63][12] = assign980_e1346_d_n12;
        s.dn[63][13] = assign980_e1346_d_n13;
        s.dn[63][14] = assign980_e1346_d_n14;
        s.dn[63][15] = assign980_e1346_d_n15;
        s.dn[63][16] = assign980_e1346_d_n16;
        s.dn[63][17] = assign980_e1346_d_n17;
        s.dn[63][18] = assign980_e1346_d_n18;
        s.db[63][0] = assign980_e1346_d_b0;
        s.db[63][1] = assign980_e1346_d_b1;
        s.db[63][2] = assign980_e1346_d_b2;
        s.db[63][3] = assign980_e1346_d_b3;
        s.db[63][4] = assign980_e1346_d_b4;
        s.db[63][5] = assign980_e1346_d_b5;
        s.db[63][6] = assign980_e1346_d_b6;
        s.db[63][7] = assign980_e1346_d_b7;
        s.db[63][8] = assign980_e1346_d_b8;
        s.db[63][9] = assign980_e1346_d_b9;
        s.db[63][10] = assign980_e1346_d_b10;
        s.db[63][11] = assign980_e1346_d_b11;
        s.db[63][12] = assign980_e1346_d_b12;
        s.db[63][13] = assign980_e1346_d_b13;
        s.db[63][14] = assign980_e1346_d_b14;
        s.db[63][15] = assign980_e1346_d_b15;
        s.db[63][16] = assign980_e1346_d_b16;
        s.db[63][17] = assign980_e1346_d_b17;
        s.db[63][18] = assign980_e1346_d_b18;
        s.rv[63] = 0.0;

        let (assign990_e1359, assign990_e1359_d_n0, assign990_e1359_d_n1, assign990_e1359_d_n2, assign990_e1359_d_n3, assign990_e1359_d_n4, assign990_e1359_d_n5, assign990_e1359_d_n6, assign990_e1359_d_n7, assign990_e1359_d_n8, assign990_e1359_d_n9, assign990_e1359_d_n10, assign990_e1359_d_n11, assign990_e1359_d_n12, assign990_e1359_d_n13, assign990_e1359_d_n14, assign990_e1359_d_n15, assign990_e1359_d_n16, assign990_e1359_d_n17, assign990_e1359_d_n18, assign990_e1359_d_b0, assign990_e1359_d_b1, assign990_e1359_d_b2, assign990_e1359_d_b3, assign990_e1359_d_b4, assign990_e1359_d_b5, assign990_e1359_d_b6, assign990_e1359_d_b7, assign990_e1359_d_b8, assign990_e1359_d_b9, assign990_e1359_d_b10, assign990_e1359_d_b11, assign990_e1359_d_b12, assign990_e1359_d_b13, assign990_e1359_d_b14, assign990_e1359_d_b15, assign990_e1359_d_b16, assign990_e1359_d_b17, assign990_e1359_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign990_e1357: f64 = (s.v[63] * s.v[63]);
        (assign990_e1357, ((s.dn[63][0] * s.v[63]) + (s.v[63] * s.dn[63][0])), ((s.dn[63][1] * s.v[63]) + (s.v[63] * s.dn[63][1])), ((s.dn[63][2] * s.v[63]) + (s.v[63] * s.dn[63][2])), ((s.dn[63][3] * s.v[63]) + (s.v[63] * s.dn[63][3])), ((s.dn[63][4] * s.v[63]) + (s.v[63] * s.dn[63][4])), ((s.dn[63][5] * s.v[63]) + (s.v[63] * s.dn[63][5])), ((s.dn[63][6] * s.v[63]) + (s.v[63] * s.dn[63][6])), ((s.dn[63][7] * s.v[63]) + (s.v[63] * s.dn[63][7])), ((s.dn[63][8] * s.v[63]) + (s.v[63] * s.dn[63][8])), ((s.dn[63][9] * s.v[63]) + (s.v[63] * s.dn[63][9])), ((s.dn[63][10] * s.v[63]) + (s.v[63] * s.dn[63][10])), ((s.dn[63][11] * s.v[63]) + (s.v[63] * s.dn[63][11])), ((s.dn[63][12] * s.v[63]) + (s.v[63] * s.dn[63][12])), ((s.dn[63][13] * s.v[63]) + (s.v[63] * s.dn[63][13])), ((s.dn[63][14] * s.v[63]) + (s.v[63] * s.dn[63][14])), ((s.dn[63][15] * s.v[63]) + (s.v[63] * s.dn[63][15])), ((s.dn[63][16] * s.v[63]) + (s.v[63] * s.dn[63][16])), ((s.dn[63][17] * s.v[63]) + (s.v[63] * s.dn[63][17])), ((s.dn[63][18] * s.v[63]) + (s.v[63] * s.dn[63][18])), ((s.db[63][0] * s.v[63]) + (s.v[63] * s.db[63][0])), ((s.db[63][1] * s.v[63]) + (s.v[63] * s.db[63][1])), ((s.db[63][2] * s.v[63]) + (s.v[63] * s.db[63][2])), ((s.db[63][3] * s.v[63]) + (s.v[63] * s.db[63][3])), ((s.db[63][4] * s.v[63]) + (s.v[63] * s.db[63][4])), ((s.db[63][5] * s.v[63]) + (s.v[63] * s.db[63][5])), ((s.db[63][6] * s.v[63]) + (s.v[63] * s.db[63][6])), ((s.db[63][7] * s.v[63]) + (s.v[63] * s.db[63][7])), ((s.db[63][8] * s.v[63]) + (s.v[63] * s.db[63][8])), ((s.db[63][9] * s.v[63]) + (s.v[63] * s.db[63][9])), ((s.db[63][10] * s.v[63]) + (s.v[63] * s.db[63][10])), ((s.db[63][11] * s.v[63]) + (s.v[63] * s.db[63][11])), ((s.db[63][12] * s.v[63]) + (s.v[63] * s.db[63][12])), ((s.db[63][13] * s.v[63]) + (s.v[63] * s.db[63][13])), ((s.db[63][14] * s.v[63]) + (s.v[63] * s.db[63][14])), ((s.db[63][15] * s.v[63]) + (s.v[63] * s.db[63][15])), ((s.db[63][16] * s.v[63]) + (s.v[63] * s.db[63][16])), ((s.db[63][17] * s.v[63]) + (s.v[63] * s.db[63][17])), ((s.db[63][18] * s.v[63]) + (s.v[63] * s.db[63][18])),)
    } else {
        (s.v[64], s.dn[64][0], s.dn[64][1], s.dn[64][2], s.dn[64][3], s.dn[64][4], s.dn[64][5], s.dn[64][6], s.dn[64][7], s.dn[64][8], s.dn[64][9], s.dn[64][10], s.dn[64][11], s.dn[64][12], s.dn[64][13], s.dn[64][14], s.dn[64][15], s.dn[64][16], s.dn[64][17], s.dn[64][18], s.db[64][0], s.db[64][1], s.db[64][2], s.db[64][3], s.db[64][4], s.db[64][5], s.db[64][6], s.db[64][7], s.db[64][8], s.db[64][9], s.db[64][10], s.db[64][11], s.db[64][12], s.db[64][13], s.db[64][14], s.db[64][15], s.db[64][16], s.db[64][17], s.db[64][18],)
    }
};
        s.v[64] = assign990_e1359;
        s.mark_derivatives_dirty(64);
        s.dn[64][0] = assign990_e1359_d_n0;
        s.dn[64][1] = assign990_e1359_d_n1;
        s.dn[64][2] = assign990_e1359_d_n2;
        s.dn[64][3] = assign990_e1359_d_n3;
        s.dn[64][4] = assign990_e1359_d_n4;
        s.dn[64][5] = assign990_e1359_d_n5;
        s.dn[64][6] = assign990_e1359_d_n6;
        s.dn[64][7] = assign990_e1359_d_n7;
        s.dn[64][8] = assign990_e1359_d_n8;
        s.dn[64][9] = assign990_e1359_d_n9;
        s.dn[64][10] = assign990_e1359_d_n10;
        s.dn[64][11] = assign990_e1359_d_n11;
        s.dn[64][12] = assign990_e1359_d_n12;
        s.dn[64][13] = assign990_e1359_d_n13;
        s.dn[64][14] = assign990_e1359_d_n14;
        s.dn[64][15] = assign990_e1359_d_n15;
        s.dn[64][16] = assign990_e1359_d_n16;
        s.dn[64][17] = assign990_e1359_d_n17;
        s.dn[64][18] = assign990_e1359_d_n18;
        s.db[64][0] = assign990_e1359_d_b0;
        s.db[64][1] = assign990_e1359_d_b1;
        s.db[64][2] = assign990_e1359_d_b2;
        s.db[64][3] = assign990_e1359_d_b3;
        s.db[64][4] = assign990_e1359_d_b4;
        s.db[64][5] = assign990_e1359_d_b5;
        s.db[64][6] = assign990_e1359_d_b6;
        s.db[64][7] = assign990_e1359_d_b7;
        s.db[64][8] = assign990_e1359_d_b8;
        s.db[64][9] = assign990_e1359_d_b9;
        s.db[64][10] = assign990_e1359_d_b10;
        s.db[64][11] = assign990_e1359_d_b11;
        s.db[64][12] = assign990_e1359_d_b12;
        s.db[64][13] = assign990_e1359_d_b13;
        s.db[64][14] = assign990_e1359_d_b14;
        s.db[64][15] = assign990_e1359_d_b15;
        s.db[64][16] = assign990_e1359_d_b16;
        s.db[64][17] = assign990_e1359_d_b17;
        s.db[64][18] = assign990_e1359_d_b18;
        s.rv[64] = 0.0;

        let (assign1000_e1382, assign1000_e1382_d_n0, assign1000_e1382_d_n1, assign1000_e1382_d_n2, assign1000_e1382_d_n3, assign1000_e1382_d_n4, assign1000_e1382_d_n5, assign1000_e1382_d_n6, assign1000_e1382_d_n7, assign1000_e1382_d_n8, assign1000_e1382_d_n9, assign1000_e1382_d_n10, assign1000_e1382_d_n11, assign1000_e1382_d_n12, assign1000_e1382_d_n13, assign1000_e1382_d_n14, assign1000_e1382_d_n15, assign1000_e1382_d_n16, assign1000_e1382_d_n17, assign1000_e1382_d_n18, assign1000_e1382_d_b0, assign1000_e1382_d_b1, assign1000_e1382_d_b2, assign1000_e1382_d_b3, assign1000_e1382_d_b4, assign1000_e1382_d_b5, assign1000_e1382_d_b6, assign1000_e1382_d_b7, assign1000_e1382_d_b8, assign1000_e1382_d_b9, assign1000_e1382_d_b10, assign1000_e1382_d_b11, assign1000_e1382_d_b12, assign1000_e1382_d_b13, assign1000_e1382_d_b14, assign1000_e1382_d_b15, assign1000_e1382_d_b16, assign1000_e1382_d_b17, assign1000_e1382_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1000_e1372: f64 = (p.p12 * s.v[64]);
        let assign1000_e1373: f64 = (s.v[63] + assign1000_e1372);
        let assign1000_e1376: f64 = (s.v[61] * s.v[64]);
        let assign1000_e1378: f64 = (assign1000_e1376 * s.v[63]);
        let assign1000_e1379: f64 = (assign1000_e1373 + assign1000_e1378);
        let assign1000_e1380: f64 = (s.v[60] * assign1000_e1379);
        (assign1000_e1380, ((s.dn[60][0] * assign1000_e1379) + (s.v[60] * ((s.dn[63][0] + (p.p12 * s.dn[64][0])) + ((((s.dn[61][0] * s.v[64]) + (s.v[61] * s.dn[64][0])) * s.v[63]) + (assign1000_e1376 * s.dn[63][0]))))), ((s.dn[60][1] * assign1000_e1379) + (s.v[60] * ((s.dn[63][1] + (p.p12 * s.dn[64][1])) + ((((s.dn[61][1] * s.v[64]) + (s.v[61] * s.dn[64][1])) * s.v[63]) + (assign1000_e1376 * s.dn[63][1]))))), ((s.dn[60][2] * assign1000_e1379) + (s.v[60] * ((s.dn[63][2] + (p.p12 * s.dn[64][2])) + ((((s.dn[61][2] * s.v[64]) + (s.v[61] * s.dn[64][2])) * s.v[63]) + (assign1000_e1376 * s.dn[63][2]))))), ((s.dn[60][3] * assign1000_e1379) + (s.v[60] * ((s.dn[63][3] + (p.p12 * s.dn[64][3])) + ((((s.dn[61][3] * s.v[64]) + (s.v[61] * s.dn[64][3])) * s.v[63]) + (assign1000_e1376 * s.dn[63][3]))))), ((s.dn[60][4] * assign1000_e1379) + (s.v[60] * ((s.dn[63][4] + (p.p12 * s.dn[64][4])) + ((((s.dn[61][4] * s.v[64]) + (s.v[61] * s.dn[64][4])) * s.v[63]) + (assign1000_e1376 * s.dn[63][4]))))), ((s.dn[60][5] * assign1000_e1379) + (s.v[60] * ((s.dn[63][5] + (p.p12 * s.dn[64][5])) + ((((s.dn[61][5] * s.v[64]) + (s.v[61] * s.dn[64][5])) * s.v[63]) + (assign1000_e1376 * s.dn[63][5]))))), ((s.dn[60][6] * assign1000_e1379) + (s.v[60] * ((s.dn[63][6] + (p.p12 * s.dn[64][6])) + ((((s.dn[61][6] * s.v[64]) + (s.v[61] * s.dn[64][6])) * s.v[63]) + (assign1000_e1376 * s.dn[63][6]))))), ((s.dn[60][7] * assign1000_e1379) + (s.v[60] * ((s.dn[63][7] + (p.p12 * s.dn[64][7])) + ((((s.dn[61][7] * s.v[64]) + (s.v[61] * s.dn[64][7])) * s.v[63]) + (assign1000_e1376 * s.dn[63][7]))))), ((s.dn[60][8] * assign1000_e1379) + (s.v[60] * ((s.dn[63][8] + (p.p12 * s.dn[64][8])) + ((((s.dn[61][8] * s.v[64]) + (s.v[61] * s.dn[64][8])) * s.v[63]) + (assign1000_e1376 * s.dn[63][8]))))), ((s.dn[60][9] * assign1000_e1379) + (s.v[60] * ((s.dn[63][9] + (p.p12 * s.dn[64][9])) + ((((s.dn[61][9] * s.v[64]) + (s.v[61] * s.dn[64][9])) * s.v[63]) + (assign1000_e1376 * s.dn[63][9]))))), ((s.dn[60][10] * assign1000_e1379) + (s.v[60] * ((s.dn[63][10] + (p.p12 * s.dn[64][10])) + ((((s.dn[61][10] * s.v[64]) + (s.v[61] * s.dn[64][10])) * s.v[63]) + (assign1000_e1376 * s.dn[63][10]))))), ((s.dn[60][11] * assign1000_e1379) + (s.v[60] * ((s.dn[63][11] + (p.p12 * s.dn[64][11])) + ((((s.dn[61][11] * s.v[64]) + (s.v[61] * s.dn[64][11])) * s.v[63]) + (assign1000_e1376 * s.dn[63][11]))))), ((s.dn[60][12] * assign1000_e1379) + (s.v[60] * ((s.dn[63][12] + (p.p12 * s.dn[64][12])) + ((((s.dn[61][12] * s.v[64]) + (s.v[61] * s.dn[64][12])) * s.v[63]) + (assign1000_e1376 * s.dn[63][12]))))), ((s.dn[60][13] * assign1000_e1379) + (s.v[60] * ((s.dn[63][13] + (p.p12 * s.dn[64][13])) + ((((s.dn[61][13] * s.v[64]) + (s.v[61] * s.dn[64][13])) * s.v[63]) + (assign1000_e1376 * s.dn[63][13]))))), ((s.dn[60][14] * assign1000_e1379) + (s.v[60] * ((s.dn[63][14] + (p.p12 * s.dn[64][14])) + ((((s.dn[61][14] * s.v[64]) + (s.v[61] * s.dn[64][14])) * s.v[63]) + (assign1000_e1376 * s.dn[63][14]))))), ((s.dn[60][15] * assign1000_e1379) + (s.v[60] * ((s.dn[63][15] + (p.p12 * s.dn[64][15])) + ((((s.dn[61][15] * s.v[64]) + (s.v[61] * s.dn[64][15])) * s.v[63]) + (assign1000_e1376 * s.dn[63][15]))))), ((s.dn[60][16] * assign1000_e1379) + (s.v[60] * ((s.dn[63][16] + (p.p12 * s.dn[64][16])) + ((((s.dn[61][16] * s.v[64]) + (s.v[61] * s.dn[64][16])) * s.v[63]) + (assign1000_e1376 * s.dn[63][16]))))), ((s.dn[60][17] * assign1000_e1379) + (s.v[60] * ((s.dn[63][17] + (p.p12 * s.dn[64][17])) + ((((s.dn[61][17] * s.v[64]) + (s.v[61] * s.dn[64][17])) * s.v[63]) + (assign1000_e1376 * s.dn[63][17]))))), ((s.dn[60][18] * assign1000_e1379) + (s.v[60] * ((s.dn[63][18] + (p.p12 * s.dn[64][18])) + ((((s.dn[61][18] * s.v[64]) + (s.v[61] * s.dn[64][18])) * s.v[63]) + (assign1000_e1376 * s.dn[63][18]))))), ((s.db[60][0] * assign1000_e1379) + (s.v[60] * ((s.db[63][0] + (p.p12 * s.db[64][0])) + ((((s.db[61][0] * s.v[64]) + (s.v[61] * s.db[64][0])) * s.v[63]) + (assign1000_e1376 * s.db[63][0]))))), ((s.db[60][1] * assign1000_e1379) + (s.v[60] * ((s.db[63][1] + (p.p12 * s.db[64][1])) + ((((s.db[61][1] * s.v[64]) + (s.v[61] * s.db[64][1])) * s.v[63]) + (assign1000_e1376 * s.db[63][1]))))), ((s.db[60][2] * assign1000_e1379) + (s.v[60] * ((s.db[63][2] + (p.p12 * s.db[64][2])) + ((((s.db[61][2] * s.v[64]) + (s.v[61] * s.db[64][2])) * s.v[63]) + (assign1000_e1376 * s.db[63][2]))))), ((s.db[60][3] * assign1000_e1379) + (s.v[60] * ((s.db[63][3] + (p.p12 * s.db[64][3])) + ((((s.db[61][3] * s.v[64]) + (s.v[61] * s.db[64][3])) * s.v[63]) + (assign1000_e1376 * s.db[63][3]))))), ((s.db[60][4] * assign1000_e1379) + (s.v[60] * ((s.db[63][4] + (p.p12 * s.db[64][4])) + ((((s.db[61][4] * s.v[64]) + (s.v[61] * s.db[64][4])) * s.v[63]) + (assign1000_e1376 * s.db[63][4]))))), ((s.db[60][5] * assign1000_e1379) + (s.v[60] * ((s.db[63][5] + (p.p12 * s.db[64][5])) + ((((s.db[61][5] * s.v[64]) + (s.v[61] * s.db[64][5])) * s.v[63]) + (assign1000_e1376 * s.db[63][5]))))), ((s.db[60][6] * assign1000_e1379) + (s.v[60] * ((s.db[63][6] + (p.p12 * s.db[64][6])) + ((((s.db[61][6] * s.v[64]) + (s.v[61] * s.db[64][6])) * s.v[63]) + (assign1000_e1376 * s.db[63][6]))))), ((s.db[60][7] * assign1000_e1379) + (s.v[60] * ((s.db[63][7] + (p.p12 * s.db[64][7])) + ((((s.db[61][7] * s.v[64]) + (s.v[61] * s.db[64][7])) * s.v[63]) + (assign1000_e1376 * s.db[63][7]))))), ((s.db[60][8] * assign1000_e1379) + (s.v[60] * ((s.db[63][8] + (p.p12 * s.db[64][8])) + ((((s.db[61][8] * s.v[64]) + (s.v[61] * s.db[64][8])) * s.v[63]) + (assign1000_e1376 * s.db[63][8]))))), ((s.db[60][9] * assign1000_e1379) + (s.v[60] * ((s.db[63][9] + (p.p12 * s.db[64][9])) + ((((s.db[61][9] * s.v[64]) + (s.v[61] * s.db[64][9])) * s.v[63]) + (assign1000_e1376 * s.db[63][9]))))), ((s.db[60][10] * assign1000_e1379) + (s.v[60] * ((s.db[63][10] + (p.p12 * s.db[64][10])) + ((((s.db[61][10] * s.v[64]) + (s.v[61] * s.db[64][10])) * s.v[63]) + (assign1000_e1376 * s.db[63][10]))))), ((s.db[60][11] * assign1000_e1379) + (s.v[60] * ((s.db[63][11] + (p.p12 * s.db[64][11])) + ((((s.db[61][11] * s.v[64]) + (s.v[61] * s.db[64][11])) * s.v[63]) + (assign1000_e1376 * s.db[63][11]))))), ((s.db[60][12] * assign1000_e1379) + (s.v[60] * ((s.db[63][12] + (p.p12 * s.db[64][12])) + ((((s.db[61][12] * s.v[64]) + (s.v[61] * s.db[64][12])) * s.v[63]) + (assign1000_e1376 * s.db[63][12]))))), ((s.db[60][13] * assign1000_e1379) + (s.v[60] * ((s.db[63][13] + (p.p12 * s.db[64][13])) + ((((s.db[61][13] * s.v[64]) + (s.v[61] * s.db[64][13])) * s.v[63]) + (assign1000_e1376 * s.db[63][13]))))), ((s.db[60][14] * assign1000_e1379) + (s.v[60] * ((s.db[63][14] + (p.p12 * s.db[64][14])) + ((((s.db[61][14] * s.v[64]) + (s.v[61] * s.db[64][14])) * s.v[63]) + (assign1000_e1376 * s.db[63][14]))))), ((s.db[60][15] * assign1000_e1379) + (s.v[60] * ((s.db[63][15] + (p.p12 * s.db[64][15])) + ((((s.db[61][15] * s.v[64]) + (s.v[61] * s.db[64][15])) * s.v[63]) + (assign1000_e1376 * s.db[63][15]))))), ((s.db[60][16] * assign1000_e1379) + (s.v[60] * ((s.db[63][16] + (p.p12 * s.db[64][16])) + ((((s.db[61][16] * s.v[64]) + (s.v[61] * s.db[64][16])) * s.v[63]) + (assign1000_e1376 * s.db[63][16]))))), ((s.db[60][17] * assign1000_e1379) + (s.v[60] * ((s.db[63][17] + (p.p12 * s.db[64][17])) + ((((s.db[61][17] * s.v[64]) + (s.v[61] * s.db[64][17])) * s.v[63]) + (assign1000_e1376 * s.db[63][17]))))), ((s.db[60][18] * assign1000_e1379) + (s.v[60] * ((s.db[63][18] + (p.p12 * s.db[64][18])) + ((((s.db[61][18] * s.v[64]) + (s.v[61] * s.db[64][18])) * s.v[63]) + (assign1000_e1376 * s.db[63][18]))))),)
    } else {
        (s.v[17], s.dn[17][0], s.dn[17][1], s.dn[17][2], s.dn[17][3], s.dn[17][4], s.dn[17][5], s.dn[17][6], s.dn[17][7], s.dn[17][8], s.dn[17][9], s.dn[17][10], s.dn[17][11], s.dn[17][12], s.dn[17][13], s.dn[17][14], s.dn[17][15], s.dn[17][16], s.dn[17][17], s.dn[17][18], s.db[17][0], s.db[17][1], s.db[17][2], s.db[17][3], s.db[17][4], s.db[17][5], s.db[17][6], s.db[17][7], s.db[17][8], s.db[17][9], s.db[17][10], s.db[17][11], s.db[17][12], s.db[17][13], s.db[17][14], s.db[17][15], s.db[17][16], s.db[17][17], s.db[17][18],)
    }
};
        s.v[17] = assign1000_e1382;
        s.mark_derivatives_dirty(17);
        s.dn[17][0] = assign1000_e1382_d_n0;
        s.dn[17][1] = assign1000_e1382_d_n1;
        s.dn[17][2] = assign1000_e1382_d_n2;
        s.dn[17][3] = assign1000_e1382_d_n3;
        s.dn[17][4] = assign1000_e1382_d_n4;
        s.dn[17][5] = assign1000_e1382_d_n5;
        s.dn[17][6] = assign1000_e1382_d_n6;
        s.dn[17][7] = assign1000_e1382_d_n7;
        s.dn[17][8] = assign1000_e1382_d_n8;
        s.dn[17][9] = assign1000_e1382_d_n9;
        s.dn[17][10] = assign1000_e1382_d_n10;
        s.dn[17][11] = assign1000_e1382_d_n11;
        s.dn[17][12] = assign1000_e1382_d_n12;
        s.dn[17][13] = assign1000_e1382_d_n13;
        s.dn[17][14] = assign1000_e1382_d_n14;
        s.dn[17][15] = assign1000_e1382_d_n15;
        s.dn[17][16] = assign1000_e1382_d_n16;
        s.dn[17][17] = assign1000_e1382_d_n17;
        s.dn[17][18] = assign1000_e1382_d_n18;
        s.db[17][0] = assign1000_e1382_d_b0;
        s.db[17][1] = assign1000_e1382_d_b1;
        s.db[17][2] = assign1000_e1382_d_b2;
        s.db[17][3] = assign1000_e1382_d_b3;
        s.db[17][4] = assign1000_e1382_d_b4;
        s.db[17][5] = assign1000_e1382_d_b5;
        s.db[17][6] = assign1000_e1382_d_b6;
        s.db[17][7] = assign1000_e1382_d_b7;
        s.db[17][8] = assign1000_e1382_d_b8;
        s.db[17][9] = assign1000_e1382_d_b9;
        s.db[17][10] = assign1000_e1382_d_b10;
        s.db[17][11] = assign1000_e1382_d_b11;
        s.db[17][12] = assign1000_e1382_d_b12;
        s.db[17][13] = assign1000_e1382_d_b13;
        s.db[17][14] = assign1000_e1382_d_b14;
        s.db[17][15] = assign1000_e1382_d_b15;
        s.db[17][16] = assign1000_e1382_d_b16;
        s.db[17][17] = assign1000_e1382_d_b17;
        s.db[17][18] = assign1000_e1382_d_b18;
        s.rv[17] = 0.0;

        let (assign1010_e1395, assign1010_e1395_d_n0, assign1010_e1395_d_n1, assign1010_e1395_d_n2, assign1010_e1395_d_n3, assign1010_e1395_d_n4, assign1010_e1395_d_n5, assign1010_e1395_d_n6, assign1010_e1395_d_n7, assign1010_e1395_d_n8, assign1010_e1395_d_n9, assign1010_e1395_d_n10, assign1010_e1395_d_n11, assign1010_e1395_d_n12, assign1010_e1395_d_n13, assign1010_e1395_d_n14, assign1010_e1395_d_n15, assign1010_e1395_d_n16, assign1010_e1395_d_n17, assign1010_e1395_d_n18, assign1010_e1395_d_b0, assign1010_e1395_d_b1, assign1010_e1395_d_b2, assign1010_e1395_d_b3, assign1010_e1395_d_b4, assign1010_e1395_d_b5, assign1010_e1395_d_b6, assign1010_e1395_d_b7, assign1010_e1395_d_b8, assign1010_e1395_d_b9, assign1010_e1395_d_b10, assign1010_e1395_d_b11, assign1010_e1395_d_b12, assign1010_e1395_d_b13, assign1010_e1395_d_b14, assign1010_e1395_d_b15, assign1010_e1395_d_b16, assign1010_e1395_d_b17, assign1010_e1395_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1010_e1393: f64 = (s.v[4] - s.v[58]);
        (assign1010_e1393, (s.dn[4][0] - s.dn[58][0]), (s.dn[4][1] - s.dn[58][1]), (s.dn[4][2] - s.dn[58][2]), (s.dn[4][3] - s.dn[58][3]), (s.dn[4][4] - s.dn[58][4]), (s.dn[4][5] - s.dn[58][5]), (s.dn[4][6] - s.dn[58][6]), (s.dn[4][7] - s.dn[58][7]), (s.dn[4][8] - s.dn[58][8]), (s.dn[4][9] - s.dn[58][9]), (s.dn[4][10] - s.dn[58][10]), (s.dn[4][11] - s.dn[58][11]), (s.dn[4][12] - s.dn[58][12]), (s.dn[4][13] - s.dn[58][13]), (s.dn[4][14] - s.dn[58][14]), (s.dn[4][15] - s.dn[58][15]), (s.dn[4][16] - s.dn[58][16]), (s.dn[4][17] - s.dn[58][17]), (s.dn[4][18] - s.dn[58][18]), (s.db[4][0] - s.db[58][0]), (s.db[4][1] - s.db[58][1]), (s.db[4][2] - s.db[58][2]), (s.db[4][3] - s.db[58][3]), (s.db[4][4] - s.db[58][4]), (s.db[4][5] - s.db[58][5]), (s.db[4][6] - s.db[58][6]), (s.db[4][7] - s.db[58][7]), (s.db[4][8] - s.db[58][8]), (s.db[4][9] - s.db[58][9]), (s.db[4][10] - s.db[58][10]), (s.db[4][11] - s.db[58][11]), (s.db[4][12] - s.db[58][12]), (s.db[4][13] - s.db[58][13]), (s.db[4][14] - s.db[58][14]), (s.db[4][15] - s.db[58][15]), (s.db[4][16] - s.db[58][16]), (s.db[4][17] - s.db[58][17]), (s.db[4][18] - s.db[58][18]),)
    } else {
        (s.v[65], s.dn[65][0], s.dn[65][1], s.dn[65][2], s.dn[65][3], s.dn[65][4], s.dn[65][5], s.dn[65][6], s.dn[65][7], s.dn[65][8], s.dn[65][9], s.dn[65][10], s.dn[65][11], s.dn[65][12], s.dn[65][13], s.dn[65][14], s.dn[65][15], s.dn[65][16], s.dn[65][17], s.dn[65][18], s.db[65][0], s.db[65][1], s.db[65][2], s.db[65][3], s.db[65][4], s.db[65][5], s.db[65][6], s.db[65][7], s.db[65][8], s.db[65][9], s.db[65][10], s.db[65][11], s.db[65][12], s.db[65][13], s.db[65][14], s.db[65][15], s.db[65][16], s.db[65][17], s.db[65][18],)
    }
};
        s.v[65] = assign1010_e1395;
        s.mark_derivatives_dirty(65);
        s.dn[65][0] = assign1010_e1395_d_n0;
        s.dn[65][1] = assign1010_e1395_d_n1;
        s.dn[65][2] = assign1010_e1395_d_n2;
        s.dn[65][3] = assign1010_e1395_d_n3;
        s.dn[65][4] = assign1010_e1395_d_n4;
        s.dn[65][5] = assign1010_e1395_d_n5;
        s.dn[65][6] = assign1010_e1395_d_n6;
        s.dn[65][7] = assign1010_e1395_d_n7;
        s.dn[65][8] = assign1010_e1395_d_n8;
        s.dn[65][9] = assign1010_e1395_d_n9;
        s.dn[65][10] = assign1010_e1395_d_n10;
        s.dn[65][11] = assign1010_e1395_d_n11;
        s.dn[65][12] = assign1010_e1395_d_n12;
        s.dn[65][13] = assign1010_e1395_d_n13;
        s.dn[65][14] = assign1010_e1395_d_n14;
        s.dn[65][15] = assign1010_e1395_d_n15;
        s.dn[65][16] = assign1010_e1395_d_n16;
        s.dn[65][17] = assign1010_e1395_d_n17;
        s.dn[65][18] = assign1010_e1395_d_n18;
        s.db[65][0] = assign1010_e1395_d_b0;
        s.db[65][1] = assign1010_e1395_d_b1;
        s.db[65][2] = assign1010_e1395_d_b2;
        s.db[65][3] = assign1010_e1395_d_b3;
        s.db[65][4] = assign1010_e1395_d_b4;
        s.db[65][5] = assign1010_e1395_d_b5;
        s.db[65][6] = assign1010_e1395_d_b6;
        s.db[65][7] = assign1010_e1395_d_b7;
        s.db[65][8] = assign1010_e1395_d_b8;
        s.db[65][9] = assign1010_e1395_d_b9;
        s.db[65][10] = assign1010_e1395_d_b10;
        s.db[65][11] = assign1010_e1395_d_b11;
        s.db[65][12] = assign1010_e1395_d_b12;
        s.db[65][13] = assign1010_e1395_d_b13;
        s.db[65][14] = assign1010_e1395_d_b14;
        s.db[65][15] = assign1010_e1395_d_b15;
        s.db[65][16] = assign1010_e1395_d_b16;
        s.db[65][17] = assign1010_e1395_d_b17;
        s.db[65][18] = assign1010_e1395_d_b18;
        s.rv[65] = 0.0;

        let (assign1020_e1408, assign1020_e1408_d_n0, assign1020_e1408_d_n1, assign1020_e1408_d_n2, assign1020_e1408_d_n3, assign1020_e1408_d_n4, assign1020_e1408_d_n5, assign1020_e1408_d_n6, assign1020_e1408_d_n7, assign1020_e1408_d_n8, assign1020_e1408_d_n9, assign1020_e1408_d_n10, assign1020_e1408_d_n11, assign1020_e1408_d_n12, assign1020_e1408_d_n13, assign1020_e1408_d_n14, assign1020_e1408_d_n15, assign1020_e1408_d_n16, assign1020_e1408_d_n17, assign1020_e1408_d_n18, assign1020_e1408_d_b0, assign1020_e1408_d_b1, assign1020_e1408_d_b2, assign1020_e1408_d_b3, assign1020_e1408_d_b4, assign1020_e1408_d_b5, assign1020_e1408_d_b6, assign1020_e1408_d_b7, assign1020_e1408_d_b8, assign1020_e1408_d_b9, assign1020_e1408_d_b10, assign1020_e1408_d_b11, assign1020_e1408_d_b12, assign1020_e1408_d_b13, assign1020_e1408_d_b14, assign1020_e1408_d_b15, assign1020_e1408_d_b16, assign1020_e1408_d_b17, assign1020_e1408_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1020_e1406: f64 = (s.v[65] * s.v[65]);
        (assign1020_e1406, ((s.dn[65][0] * s.v[65]) + (s.v[65] * s.dn[65][0])), ((s.dn[65][1] * s.v[65]) + (s.v[65] * s.dn[65][1])), ((s.dn[65][2] * s.v[65]) + (s.v[65] * s.dn[65][2])), ((s.dn[65][3] * s.v[65]) + (s.v[65] * s.dn[65][3])), ((s.dn[65][4] * s.v[65]) + (s.v[65] * s.dn[65][4])), ((s.dn[65][5] * s.v[65]) + (s.v[65] * s.dn[65][5])), ((s.dn[65][6] * s.v[65]) + (s.v[65] * s.dn[65][6])), ((s.dn[65][7] * s.v[65]) + (s.v[65] * s.dn[65][7])), ((s.dn[65][8] * s.v[65]) + (s.v[65] * s.dn[65][8])), ((s.dn[65][9] * s.v[65]) + (s.v[65] * s.dn[65][9])), ((s.dn[65][10] * s.v[65]) + (s.v[65] * s.dn[65][10])), ((s.dn[65][11] * s.v[65]) + (s.v[65] * s.dn[65][11])), ((s.dn[65][12] * s.v[65]) + (s.v[65] * s.dn[65][12])), ((s.dn[65][13] * s.v[65]) + (s.v[65] * s.dn[65][13])), ((s.dn[65][14] * s.v[65]) + (s.v[65] * s.dn[65][14])), ((s.dn[65][15] * s.v[65]) + (s.v[65] * s.dn[65][15])), ((s.dn[65][16] * s.v[65]) + (s.v[65] * s.dn[65][16])), ((s.dn[65][17] * s.v[65]) + (s.v[65] * s.dn[65][17])), ((s.dn[65][18] * s.v[65]) + (s.v[65] * s.dn[65][18])), ((s.db[65][0] * s.v[65]) + (s.v[65] * s.db[65][0])), ((s.db[65][1] * s.v[65]) + (s.v[65] * s.db[65][1])), ((s.db[65][2] * s.v[65]) + (s.v[65] * s.db[65][2])), ((s.db[65][3] * s.v[65]) + (s.v[65] * s.db[65][3])), ((s.db[65][4] * s.v[65]) + (s.v[65] * s.db[65][4])), ((s.db[65][5] * s.v[65]) + (s.v[65] * s.db[65][5])), ((s.db[65][6] * s.v[65]) + (s.v[65] * s.db[65][6])), ((s.db[65][7] * s.v[65]) + (s.v[65] * s.db[65][7])), ((s.db[65][8] * s.v[65]) + (s.v[65] * s.db[65][8])), ((s.db[65][9] * s.v[65]) + (s.v[65] * s.db[65][9])), ((s.db[65][10] * s.v[65]) + (s.v[65] * s.db[65][10])), ((s.db[65][11] * s.v[65]) + (s.v[65] * s.db[65][11])), ((s.db[65][12] * s.v[65]) + (s.v[65] * s.db[65][12])), ((s.db[65][13] * s.v[65]) + (s.v[65] * s.db[65][13])), ((s.db[65][14] * s.v[65]) + (s.v[65] * s.db[65][14])), ((s.db[65][15] * s.v[65]) + (s.v[65] * s.db[65][15])), ((s.db[65][16] * s.v[65]) + (s.v[65] * s.db[65][16])), ((s.db[65][17] * s.v[65]) + (s.v[65] * s.db[65][17])), ((s.db[65][18] * s.v[65]) + (s.v[65] * s.db[65][18])),)
    } else {
        (s.v[66], s.dn[66][0], s.dn[66][1], s.dn[66][2], s.dn[66][3], s.dn[66][4], s.dn[66][5], s.dn[66][6], s.dn[66][7], s.dn[66][8], s.dn[66][9], s.dn[66][10], s.dn[66][11], s.dn[66][12], s.dn[66][13], s.dn[66][14], s.dn[66][15], s.dn[66][16], s.dn[66][17], s.dn[66][18], s.db[66][0], s.db[66][1], s.db[66][2], s.db[66][3], s.db[66][4], s.db[66][5], s.db[66][6], s.db[66][7], s.db[66][8], s.db[66][9], s.db[66][10], s.db[66][11], s.db[66][12], s.db[66][13], s.db[66][14], s.db[66][15], s.db[66][16], s.db[66][17], s.db[66][18],)
    }
};
        s.v[66] = assign1020_e1408;
        s.mark_derivatives_dirty(66);
        s.dn[66][0] = assign1020_e1408_d_n0;
        s.dn[66][1] = assign1020_e1408_d_n1;
        s.dn[66][2] = assign1020_e1408_d_n2;
        s.dn[66][3] = assign1020_e1408_d_n3;
        s.dn[66][4] = assign1020_e1408_d_n4;
        s.dn[66][5] = assign1020_e1408_d_n5;
        s.dn[66][6] = assign1020_e1408_d_n6;
        s.dn[66][7] = assign1020_e1408_d_n7;
        s.dn[66][8] = assign1020_e1408_d_n8;
        s.dn[66][9] = assign1020_e1408_d_n9;
        s.dn[66][10] = assign1020_e1408_d_n10;
        s.dn[66][11] = assign1020_e1408_d_n11;
        s.dn[66][12] = assign1020_e1408_d_n12;
        s.dn[66][13] = assign1020_e1408_d_n13;
        s.dn[66][14] = assign1020_e1408_d_n14;
        s.dn[66][15] = assign1020_e1408_d_n15;
        s.dn[66][16] = assign1020_e1408_d_n16;
        s.dn[66][17] = assign1020_e1408_d_n17;
        s.dn[66][18] = assign1020_e1408_d_n18;
        s.db[66][0] = assign1020_e1408_d_b0;
        s.db[66][1] = assign1020_e1408_d_b1;
        s.db[66][2] = assign1020_e1408_d_b2;
        s.db[66][3] = assign1020_e1408_d_b3;
        s.db[66][4] = assign1020_e1408_d_b4;
        s.db[66][5] = assign1020_e1408_d_b5;
        s.db[66][6] = assign1020_e1408_d_b6;
        s.db[66][7] = assign1020_e1408_d_b7;
        s.db[66][8] = assign1020_e1408_d_b8;
        s.db[66][9] = assign1020_e1408_d_b9;
        s.db[66][10] = assign1020_e1408_d_b10;
        s.db[66][11] = assign1020_e1408_d_b11;
        s.db[66][12] = assign1020_e1408_d_b12;
        s.db[66][13] = assign1020_e1408_d_b13;
        s.db[66][14] = assign1020_e1408_d_b14;
        s.db[66][15] = assign1020_e1408_d_b15;
        s.db[66][16] = assign1020_e1408_d_b16;
        s.db[66][17] = assign1020_e1408_d_b17;
        s.db[66][18] = assign1020_e1408_d_b18;
        s.rv[66] = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let (assign1030_e1431, assign1030_e1431_d_n0, assign1030_e1431_d_n1, assign1030_e1431_d_n2, assign1030_e1431_d_n3, assign1030_e1431_d_n4, assign1030_e1431_d_n5, assign1030_e1431_d_n6, assign1030_e1431_d_n7, assign1030_e1431_d_n8, assign1030_e1431_d_n9, assign1030_e1431_d_n10, assign1030_e1431_d_n11, assign1030_e1431_d_n12, assign1030_e1431_d_n13, assign1030_e1431_d_n14, assign1030_e1431_d_n15, assign1030_e1431_d_n16, assign1030_e1431_d_n17, assign1030_e1431_d_n18, assign1030_e1431_d_b0, assign1030_e1431_d_b1, assign1030_e1431_d_b2, assign1030_e1431_d_b3, assign1030_e1431_d_b4, assign1030_e1431_d_b5, assign1030_e1431_d_b6, assign1030_e1431_d_b7, assign1030_e1431_d_b8, assign1030_e1431_d_b9, assign1030_e1431_d_b10, assign1030_e1431_d_b11, assign1030_e1431_d_b12, assign1030_e1431_d_b13, assign1030_e1431_d_b14, assign1030_e1431_d_b15, assign1030_e1431_d_b16, assign1030_e1431_d_b17, assign1030_e1431_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1030_e1421: f64 = (p.p12 * s.v[66]);
        let assign1030_e1422: f64 = (s.v[65] + assign1030_e1421);
        let assign1030_e1425: f64 = (s.v[61] * s.v[65]);
        let assign1030_e1427: f64 = (assign1030_e1425 * s.v[66]);
        let assign1030_e1428: f64 = (assign1030_e1422 + assign1030_e1427);
        let assign1030_e1429: f64 = (s.v[60] * assign1030_e1428);
        (assign1030_e1429, ((s.dn[60][0] * assign1030_e1428) + (s.v[60] * ((s.dn[65][0] + (p.p12 * s.dn[66][0])) + ((((s.dn[61][0] * s.v[65]) + (s.v[61] * s.dn[65][0])) * s.v[66]) + (assign1030_e1425 * s.dn[66][0]))))), ((s.dn[60][1] * assign1030_e1428) + (s.v[60] * ((s.dn[65][1] + (p.p12 * s.dn[66][1])) + ((((s.dn[61][1] * s.v[65]) + (s.v[61] * s.dn[65][1])) * s.v[66]) + (assign1030_e1425 * s.dn[66][1]))))), ((s.dn[60][2] * assign1030_e1428) + (s.v[60] * ((s.dn[65][2] + (p.p12 * s.dn[66][2])) + ((((s.dn[61][2] * s.v[65]) + (s.v[61] * s.dn[65][2])) * s.v[66]) + (assign1030_e1425 * s.dn[66][2]))))), ((s.dn[60][3] * assign1030_e1428) + (s.v[60] * ((s.dn[65][3] + (p.p12 * s.dn[66][3])) + ((((s.dn[61][3] * s.v[65]) + (s.v[61] * s.dn[65][3])) * s.v[66]) + (assign1030_e1425 * s.dn[66][3]))))), ((s.dn[60][4] * assign1030_e1428) + (s.v[60] * ((s.dn[65][4] + (p.p12 * s.dn[66][4])) + ((((s.dn[61][4] * s.v[65]) + (s.v[61] * s.dn[65][4])) * s.v[66]) + (assign1030_e1425 * s.dn[66][4]))))), ((s.dn[60][5] * assign1030_e1428) + (s.v[60] * ((s.dn[65][5] + (p.p12 * s.dn[66][5])) + ((((s.dn[61][5] * s.v[65]) + (s.v[61] * s.dn[65][5])) * s.v[66]) + (assign1030_e1425 * s.dn[66][5]))))), ((s.dn[60][6] * assign1030_e1428) + (s.v[60] * ((s.dn[65][6] + (p.p12 * s.dn[66][6])) + ((((s.dn[61][6] * s.v[65]) + (s.v[61] * s.dn[65][6])) * s.v[66]) + (assign1030_e1425 * s.dn[66][6]))))), ((s.dn[60][7] * assign1030_e1428) + (s.v[60] * ((s.dn[65][7] + (p.p12 * s.dn[66][7])) + ((((s.dn[61][7] * s.v[65]) + (s.v[61] * s.dn[65][7])) * s.v[66]) + (assign1030_e1425 * s.dn[66][7]))))), ((s.dn[60][8] * assign1030_e1428) + (s.v[60] * ((s.dn[65][8] + (p.p12 * s.dn[66][8])) + ((((s.dn[61][8] * s.v[65]) + (s.v[61] * s.dn[65][8])) * s.v[66]) + (assign1030_e1425 * s.dn[66][8]))))), ((s.dn[60][9] * assign1030_e1428) + (s.v[60] * ((s.dn[65][9] + (p.p12 * s.dn[66][9])) + ((((s.dn[61][9] * s.v[65]) + (s.v[61] * s.dn[65][9])) * s.v[66]) + (assign1030_e1425 * s.dn[66][9]))))), ((s.dn[60][10] * assign1030_e1428) + (s.v[60] * ((s.dn[65][10] + (p.p12 * s.dn[66][10])) + ((((s.dn[61][10] * s.v[65]) + (s.v[61] * s.dn[65][10])) * s.v[66]) + (assign1030_e1425 * s.dn[66][10]))))), ((s.dn[60][11] * assign1030_e1428) + (s.v[60] * ((s.dn[65][11] + (p.p12 * s.dn[66][11])) + ((((s.dn[61][11] * s.v[65]) + (s.v[61] * s.dn[65][11])) * s.v[66]) + (assign1030_e1425 * s.dn[66][11]))))), ((s.dn[60][12] * assign1030_e1428) + (s.v[60] * ((s.dn[65][12] + (p.p12 * s.dn[66][12])) + ((((s.dn[61][12] * s.v[65]) + (s.v[61] * s.dn[65][12])) * s.v[66]) + (assign1030_e1425 * s.dn[66][12]))))), ((s.dn[60][13] * assign1030_e1428) + (s.v[60] * ((s.dn[65][13] + (p.p12 * s.dn[66][13])) + ((((s.dn[61][13] * s.v[65]) + (s.v[61] * s.dn[65][13])) * s.v[66]) + (assign1030_e1425 * s.dn[66][13]))))), ((s.dn[60][14] * assign1030_e1428) + (s.v[60] * ((s.dn[65][14] + (p.p12 * s.dn[66][14])) + ((((s.dn[61][14] * s.v[65]) + (s.v[61] * s.dn[65][14])) * s.v[66]) + (assign1030_e1425 * s.dn[66][14]))))), ((s.dn[60][15] * assign1030_e1428) + (s.v[60] * ((s.dn[65][15] + (p.p12 * s.dn[66][15])) + ((((s.dn[61][15] * s.v[65]) + (s.v[61] * s.dn[65][15])) * s.v[66]) + (assign1030_e1425 * s.dn[66][15]))))), ((s.dn[60][16] * assign1030_e1428) + (s.v[60] * ((s.dn[65][16] + (p.p12 * s.dn[66][16])) + ((((s.dn[61][16] * s.v[65]) + (s.v[61] * s.dn[65][16])) * s.v[66]) + (assign1030_e1425 * s.dn[66][16]))))), ((s.dn[60][17] * assign1030_e1428) + (s.v[60] * ((s.dn[65][17] + (p.p12 * s.dn[66][17])) + ((((s.dn[61][17] * s.v[65]) + (s.v[61] * s.dn[65][17])) * s.v[66]) + (assign1030_e1425 * s.dn[66][17]))))), ((s.dn[60][18] * assign1030_e1428) + (s.v[60] * ((s.dn[65][18] + (p.p12 * s.dn[66][18])) + ((((s.dn[61][18] * s.v[65]) + (s.v[61] * s.dn[65][18])) * s.v[66]) + (assign1030_e1425 * s.dn[66][18]))))), ((s.db[60][0] * assign1030_e1428) + (s.v[60] * ((s.db[65][0] + (p.p12 * s.db[66][0])) + ((((s.db[61][0] * s.v[65]) + (s.v[61] * s.db[65][0])) * s.v[66]) + (assign1030_e1425 * s.db[66][0]))))), ((s.db[60][1] * assign1030_e1428) + (s.v[60] * ((s.db[65][1] + (p.p12 * s.db[66][1])) + ((((s.db[61][1] * s.v[65]) + (s.v[61] * s.db[65][1])) * s.v[66]) + (assign1030_e1425 * s.db[66][1]))))), ((s.db[60][2] * assign1030_e1428) + (s.v[60] * ((s.db[65][2] + (p.p12 * s.db[66][2])) + ((((s.db[61][2] * s.v[65]) + (s.v[61] * s.db[65][2])) * s.v[66]) + (assign1030_e1425 * s.db[66][2]))))), ((s.db[60][3] * assign1030_e1428) + (s.v[60] * ((s.db[65][3] + (p.p12 * s.db[66][3])) + ((((s.db[61][3] * s.v[65]) + (s.v[61] * s.db[65][3])) * s.v[66]) + (assign1030_e1425 * s.db[66][3]))))), ((s.db[60][4] * assign1030_e1428) + (s.v[60] * ((s.db[65][4] + (p.p12 * s.db[66][4])) + ((((s.db[61][4] * s.v[65]) + (s.v[61] * s.db[65][4])) * s.v[66]) + (assign1030_e1425 * s.db[66][4]))))), ((s.db[60][5] * assign1030_e1428) + (s.v[60] * ((s.db[65][5] + (p.p12 * s.db[66][5])) + ((((s.db[61][5] * s.v[65]) + (s.v[61] * s.db[65][5])) * s.v[66]) + (assign1030_e1425 * s.db[66][5]))))), ((s.db[60][6] * assign1030_e1428) + (s.v[60] * ((s.db[65][6] + (p.p12 * s.db[66][6])) + ((((s.db[61][6] * s.v[65]) + (s.v[61] * s.db[65][6])) * s.v[66]) + (assign1030_e1425 * s.db[66][6]))))), ((s.db[60][7] * assign1030_e1428) + (s.v[60] * ((s.db[65][7] + (p.p12 * s.db[66][7])) + ((((s.db[61][7] * s.v[65]) + (s.v[61] * s.db[65][7])) * s.v[66]) + (assign1030_e1425 * s.db[66][7]))))), ((s.db[60][8] * assign1030_e1428) + (s.v[60] * ((s.db[65][8] + (p.p12 * s.db[66][8])) + ((((s.db[61][8] * s.v[65]) + (s.v[61] * s.db[65][8])) * s.v[66]) + (assign1030_e1425 * s.db[66][8]))))), ((s.db[60][9] * assign1030_e1428) + (s.v[60] * ((s.db[65][9] + (p.p12 * s.db[66][9])) + ((((s.db[61][9] * s.v[65]) + (s.v[61] * s.db[65][9])) * s.v[66]) + (assign1030_e1425 * s.db[66][9]))))), ((s.db[60][10] * assign1030_e1428) + (s.v[60] * ((s.db[65][10] + (p.p12 * s.db[66][10])) + ((((s.db[61][10] * s.v[65]) + (s.v[61] * s.db[65][10])) * s.v[66]) + (assign1030_e1425 * s.db[66][10]))))), ((s.db[60][11] * assign1030_e1428) + (s.v[60] * ((s.db[65][11] + (p.p12 * s.db[66][11])) + ((((s.db[61][11] * s.v[65]) + (s.v[61] * s.db[65][11])) * s.v[66]) + (assign1030_e1425 * s.db[66][11]))))), ((s.db[60][12] * assign1030_e1428) + (s.v[60] * ((s.db[65][12] + (p.p12 * s.db[66][12])) + ((((s.db[61][12] * s.v[65]) + (s.v[61] * s.db[65][12])) * s.v[66]) + (assign1030_e1425 * s.db[66][12]))))), ((s.db[60][13] * assign1030_e1428) + (s.v[60] * ((s.db[65][13] + (p.p12 * s.db[66][13])) + ((((s.db[61][13] * s.v[65]) + (s.v[61] * s.db[65][13])) * s.v[66]) + (assign1030_e1425 * s.db[66][13]))))), ((s.db[60][14] * assign1030_e1428) + (s.v[60] * ((s.db[65][14] + (p.p12 * s.db[66][14])) + ((((s.db[61][14] * s.v[65]) + (s.v[61] * s.db[65][14])) * s.v[66]) + (assign1030_e1425 * s.db[66][14]))))), ((s.db[60][15] * assign1030_e1428) + (s.v[60] * ((s.db[65][15] + (p.p12 * s.db[66][15])) + ((((s.db[61][15] * s.v[65]) + (s.v[61] * s.db[65][15])) * s.v[66]) + (assign1030_e1425 * s.db[66][15]))))), ((s.db[60][16] * assign1030_e1428) + (s.v[60] * ((s.db[65][16] + (p.p12 * s.db[66][16])) + ((((s.db[61][16] * s.v[65]) + (s.v[61] * s.db[65][16])) * s.v[66]) + (assign1030_e1425 * s.db[66][16]))))), ((s.db[60][17] * assign1030_e1428) + (s.v[60] * ((s.db[65][17] + (p.p12 * s.db[66][17])) + ((((s.db[61][17] * s.v[65]) + (s.v[61] * s.db[65][17])) * s.v[66]) + (assign1030_e1425 * s.db[66][17]))))), ((s.db[60][18] * assign1030_e1428) + (s.v[60] * ((s.db[65][18] + (p.p12 * s.db[66][18])) + ((((s.db[61][18] * s.v[65]) + (s.v[61] * s.db[65][18])) * s.v[66]) + (assign1030_e1425 * s.db[66][18]))))),)
    } else {
        (s.v[71], s.dn[71][0], s.dn[71][1], s.dn[71][2], s.dn[71][3], s.dn[71][4], s.dn[71][5], s.dn[71][6], s.dn[71][7], s.dn[71][8], s.dn[71][9], s.dn[71][10], s.dn[71][11], s.dn[71][12], s.dn[71][13], s.dn[71][14], s.dn[71][15], s.dn[71][16], s.dn[71][17], s.dn[71][18], s.db[71][0], s.db[71][1], s.db[71][2], s.db[71][3], s.db[71][4], s.db[71][5], s.db[71][6], s.db[71][7], s.db[71][8], s.db[71][9], s.db[71][10], s.db[71][11], s.db[71][12], s.db[71][13], s.db[71][14], s.db[71][15], s.db[71][16], s.db[71][17], s.db[71][18],)
    }
};
        s.v[71] = assign1030_e1431;
        s.mark_derivatives_dirty(71);
        s.dn[71][0] = assign1030_e1431_d_n0;
        s.dn[71][1] = assign1030_e1431_d_n1;
        s.dn[71][2] = assign1030_e1431_d_n2;
        s.dn[71][3] = assign1030_e1431_d_n3;
        s.dn[71][4] = assign1030_e1431_d_n4;
        s.dn[71][5] = assign1030_e1431_d_n5;
        s.dn[71][6] = assign1030_e1431_d_n6;
        s.dn[71][7] = assign1030_e1431_d_n7;
        s.dn[71][8] = assign1030_e1431_d_n8;
        s.dn[71][9] = assign1030_e1431_d_n9;
        s.dn[71][10] = assign1030_e1431_d_n10;
        s.dn[71][11] = assign1030_e1431_d_n11;
        s.dn[71][12] = assign1030_e1431_d_n12;
        s.dn[71][13] = assign1030_e1431_d_n13;
        s.dn[71][14] = assign1030_e1431_d_n14;
        s.dn[71][15] = assign1030_e1431_d_n15;
        s.dn[71][16] = assign1030_e1431_d_n16;
        s.dn[71][17] = assign1030_e1431_d_n17;
        s.dn[71][18] = assign1030_e1431_d_n18;
        s.db[71][0] = assign1030_e1431_d_b0;
        s.db[71][1] = assign1030_e1431_d_b1;
        s.db[71][2] = assign1030_e1431_d_b2;
        s.db[71][3] = assign1030_e1431_d_b3;
        s.db[71][4] = assign1030_e1431_d_b4;
        s.db[71][5] = assign1030_e1431_d_b5;
        s.db[71][6] = assign1030_e1431_d_b6;
        s.db[71][7] = assign1030_e1431_d_b7;
        s.db[71][8] = assign1030_e1431_d_b8;
        s.db[71][9] = assign1030_e1431_d_b9;
        s.db[71][10] = assign1030_e1431_d_b10;
        s.db[71][11] = assign1030_e1431_d_b11;
        s.db[71][12] = assign1030_e1431_d_b12;
        s.db[71][13] = assign1030_e1431_d_b13;
        s.db[71][14] = assign1030_e1431_d_b14;
        s.db[71][15] = assign1030_e1431_d_b15;
        s.db[71][16] = assign1030_e1431_d_b16;
        s.db[71][17] = assign1030_e1431_d_b17;
        s.db[71][18] = assign1030_e1431_d_b18;
        s.rv[71] = 0.0;

        let (assign1040_e1452, assign1040_e1452_d_n0, assign1040_e1452_d_n1, assign1040_e1452_d_n2, assign1040_e1452_d_n3, assign1040_e1452_d_n4, assign1040_e1452_d_n5, assign1040_e1452_d_n6, assign1040_e1452_d_n7, assign1040_e1452_d_n8, assign1040_e1452_d_n9, assign1040_e1452_d_n10, assign1040_e1452_d_n11, assign1040_e1452_d_n12, assign1040_e1452_d_n13, assign1040_e1452_d_n14, assign1040_e1452_d_n15, assign1040_e1452_d_n16, assign1040_e1452_d_n17, assign1040_e1452_d_n18, assign1040_e1452_d_b0, assign1040_e1452_d_b1, assign1040_e1452_d_b2, assign1040_e1452_d_b3, assign1040_e1452_d_b4, assign1040_e1452_d_b5, assign1040_e1452_d_b6, assign1040_e1452_d_b7, assign1040_e1452_d_b8, assign1040_e1452_d_b9, assign1040_e1452_d_b10, assign1040_e1452_d_b11, assign1040_e1452_d_b12, assign1040_e1452_d_b13, assign1040_e1452_d_b14, assign1040_e1452_d_b15, assign1040_e1452_d_b16, assign1040_e1452_d_b17, assign1040_e1452_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1040_e1443: f64 = { let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1040_e1445: f64 = (-s.v[17]);
        let assign1040_e1446: f64 = { let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1040_e1447: f64 = (assign1040_e1443 - assign1040_e1446);
        let assign1040_e1448: f64 = (0.5 * assign1040_e1447);
        let assign1040_e1449: f64 = (assign1040_e1448).tanh();
        let assign1040_e1450: f64 = (1.0 + assign1040_e1449);
        (assign1040_e1450, ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][0]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][0])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][1]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][1])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][2]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][2])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][3]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][3])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][4]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][4])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][5]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][5])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][6]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][6])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][7]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][7])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][8]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][8])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][9]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][9])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][10]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][10])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][11]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][11])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][12]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][12])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][13]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][13])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][14]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][14])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][15]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][15])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][16]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][16])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][17]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][17])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[17][18]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[17][18])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][0]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][0])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][1]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][1])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][2]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][2])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][3]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][3])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][4]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][4])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][5]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][5])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][6]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][6])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][7]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][7])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][8]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][8])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][9]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][9])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][10]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][10])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][11]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][11])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][12]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][12])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][13]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][13])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][14]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][14])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][15]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][15])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][16]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][16])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][17]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][17])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = s.v[17]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[17][18]) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[17][18])))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())),)
    } else {
        (s.v[76], s.dn[76][0], s.dn[76][1], s.dn[76][2], s.dn[76][3], s.dn[76][4], s.dn[76][5], s.dn[76][6], s.dn[76][7], s.dn[76][8], s.dn[76][9], s.dn[76][10], s.dn[76][11], s.dn[76][12], s.dn[76][13], s.dn[76][14], s.dn[76][15], s.dn[76][16], s.dn[76][17], s.dn[76][18], s.db[76][0], s.db[76][1], s.db[76][2], s.db[76][3], s.db[76][4], s.db[76][5], s.db[76][6], s.db[76][7], s.db[76][8], s.db[76][9], s.db[76][10], s.db[76][11], s.db[76][12], s.db[76][13], s.db[76][14], s.db[76][15], s.db[76][16], s.db[76][17], s.db[76][18],)
    }
};
        s.v[76] = assign1040_e1452;
        s.mark_derivatives_dirty(76);
        s.dn[76][0] = assign1040_e1452_d_n0;
        s.dn[76][1] = assign1040_e1452_d_n1;
        s.dn[76][2] = assign1040_e1452_d_n2;
        s.dn[76][3] = assign1040_e1452_d_n3;
        s.dn[76][4] = assign1040_e1452_d_n4;
        s.dn[76][5] = assign1040_e1452_d_n5;
        s.dn[76][6] = assign1040_e1452_d_n6;
        s.dn[76][7] = assign1040_e1452_d_n7;
        s.dn[76][8] = assign1040_e1452_d_n8;
        s.dn[76][9] = assign1040_e1452_d_n9;
        s.dn[76][10] = assign1040_e1452_d_n10;
        s.dn[76][11] = assign1040_e1452_d_n11;
        s.dn[76][12] = assign1040_e1452_d_n12;
        s.dn[76][13] = assign1040_e1452_d_n13;
        s.dn[76][14] = assign1040_e1452_d_n14;
        s.dn[76][15] = assign1040_e1452_d_n15;
        s.dn[76][16] = assign1040_e1452_d_n16;
        s.dn[76][17] = assign1040_e1452_d_n17;
        s.dn[76][18] = assign1040_e1452_d_n18;
        s.db[76][0] = assign1040_e1452_d_b0;
        s.db[76][1] = assign1040_e1452_d_b1;
        s.db[76][2] = assign1040_e1452_d_b2;
        s.db[76][3] = assign1040_e1452_d_b3;
        s.db[76][4] = assign1040_e1452_d_b4;
        s.db[76][5] = assign1040_e1452_d_b5;
        s.db[76][6] = assign1040_e1452_d_b6;
        s.db[76][7] = assign1040_e1452_d_b7;
        s.db[76][8] = assign1040_e1452_d_b8;
        s.db[76][9] = assign1040_e1452_d_b9;
        s.db[76][10] = assign1040_e1452_d_b10;
        s.db[76][11] = assign1040_e1452_d_b11;
        s.db[76][12] = assign1040_e1452_d_b12;
        s.db[76][13] = assign1040_e1452_d_b13;
        s.db[76][14] = assign1040_e1452_d_b14;
        s.db[76][15] = assign1040_e1452_d_b15;
        s.db[76][16] = assign1040_e1452_d_b16;
        s.db[76][17] = assign1040_e1452_d_b17;
        s.db[76][18] = assign1040_e1452_d_b18;
        s.rv[76] = 0.0;

        let (assign1050_e1473, assign1050_e1473_d_n0, assign1050_e1473_d_n1, assign1050_e1473_d_n2, assign1050_e1473_d_n3, assign1050_e1473_d_n4, assign1050_e1473_d_n5, assign1050_e1473_d_n6, assign1050_e1473_d_n7, assign1050_e1473_d_n8, assign1050_e1473_d_n9, assign1050_e1473_d_n10, assign1050_e1473_d_n11, assign1050_e1473_d_n12, assign1050_e1473_d_n13, assign1050_e1473_d_n14, assign1050_e1473_d_n15, assign1050_e1473_d_n16, assign1050_e1473_d_n17, assign1050_e1473_d_n18, assign1050_e1473_d_b0, assign1050_e1473_d_b1, assign1050_e1473_d_b2, assign1050_e1473_d_b3, assign1050_e1473_d_b4, assign1050_e1473_d_b5, assign1050_e1473_d_b6, assign1050_e1473_d_b7, assign1050_e1473_d_b8, assign1050_e1473_d_b9, assign1050_e1473_d_b10, assign1050_e1473_d_b11, assign1050_e1473_d_b12, assign1050_e1473_d_b13, assign1050_e1473_d_b14, assign1050_e1473_d_b15, assign1050_e1473_d_b16, assign1050_e1473_d_b17, assign1050_e1473_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1050_e1464: f64 = { let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1050_e1466: f64 = (-s.v[71]);
        let assign1050_e1467: f64 = { let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1050_e1468: f64 = (assign1050_e1464 - assign1050_e1467);
        let assign1050_e1469: f64 = (0.5 * assign1050_e1468);
        let assign1050_e1470: f64 = (assign1050_e1469).tanh();
        let assign1050_e1471: f64 = (1.0 + assign1050_e1470);
        (assign1050_e1471, ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][0]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][0])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][1]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][1])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][2]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][2])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][3]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][3])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][4]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][4])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][5]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][5])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][6]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][6])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][7]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][7])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][8]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][8])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][9]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][9])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][10]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][10])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][11]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][11])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][12]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][12])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][13]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][13])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][14]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][14])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][15]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][15])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][16]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][16])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][17]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][17])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.dn[71][18]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.dn[71][18])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][0]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][0])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][1]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][1])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][2]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][2])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][3]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][3])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][4]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][4])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][5]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][5])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][6]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][6])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][7]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][7])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][8]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][8])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][9]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][9])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][10]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][10])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][11]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][11])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][12]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][12])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][13]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][13])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][14]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][14])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][15]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][15])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][16]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][16])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][17]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][17])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())), ((0.5 * (({ let limexp_arg = s.v[71]; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * s.db[71][18]) - ({ let limexp_arg = assign1050_e1466; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-s.db[71][18])))) / ((assign1050_e1469).cosh() * (assign1050_e1469).cosh())),)
    } else {
        (s.v[78], s.dn[78][0], s.dn[78][1], s.dn[78][2], s.dn[78][3], s.dn[78][4], s.dn[78][5], s.dn[78][6], s.dn[78][7], s.dn[78][8], s.dn[78][9], s.dn[78][10], s.dn[78][11], s.dn[78][12], s.dn[78][13], s.dn[78][14], s.dn[78][15], s.dn[78][16], s.dn[78][17], s.dn[78][18], s.db[78][0], s.db[78][1], s.db[78][2], s.db[78][3], s.db[78][4], s.db[78][5], s.db[78][6], s.db[78][7], s.db[78][8], s.db[78][9], s.db[78][10], s.db[78][11], s.db[78][12], s.db[78][13], s.db[78][14], s.db[78][15], s.db[78][16], s.db[78][17], s.db[78][18],)
    }
};
        s.v[78] = assign1050_e1473;
        s.mark_derivatives_dirty(78);
        s.dn[78][0] = assign1050_e1473_d_n0;
        s.dn[78][1] = assign1050_e1473_d_n1;
        s.dn[78][2] = assign1050_e1473_d_n2;
        s.dn[78][3] = assign1050_e1473_d_n3;
        s.dn[78][4] = assign1050_e1473_d_n4;
        s.dn[78][5] = assign1050_e1473_d_n5;
        s.dn[78][6] = assign1050_e1473_d_n6;
        s.dn[78][7] = assign1050_e1473_d_n7;
        s.dn[78][8] = assign1050_e1473_d_n8;
        s.dn[78][9] = assign1050_e1473_d_n9;
        s.dn[78][10] = assign1050_e1473_d_n10;
        s.dn[78][11] = assign1050_e1473_d_n11;
        s.dn[78][12] = assign1050_e1473_d_n12;
        s.dn[78][13] = assign1050_e1473_d_n13;
        s.dn[78][14] = assign1050_e1473_d_n14;
        s.dn[78][15] = assign1050_e1473_d_n15;
        s.dn[78][16] = assign1050_e1473_d_n16;
        s.dn[78][17] = assign1050_e1473_d_n17;
        s.dn[78][18] = assign1050_e1473_d_n18;
        s.db[78][0] = assign1050_e1473_d_b0;
        s.db[78][1] = assign1050_e1473_d_b1;
        s.db[78][2] = assign1050_e1473_d_b2;
        s.db[78][3] = assign1050_e1473_d_b3;
        s.db[78][4] = assign1050_e1473_d_b4;
        s.db[78][5] = assign1050_e1473_d_b5;
        s.db[78][6] = assign1050_e1473_d_b6;
        s.db[78][7] = assign1050_e1473_d_b7;
        s.db[78][8] = assign1050_e1473_d_b8;
        s.db[78][9] = assign1050_e1473_d_b9;
        s.db[78][10] = assign1050_e1473_d_b10;
        s.db[78][11] = assign1050_e1473_d_b11;
        s.db[78][12] = assign1050_e1473_d_b12;
        s.db[78][13] = assign1050_e1473_d_b13;
        s.db[78][14] = assign1050_e1473_d_b14;
        s.db[78][15] = assign1050_e1473_d_b15;
        s.db[78][16] = assign1050_e1473_d_b16;
        s.db[78][17] = assign1050_e1473_d_b17;
        s.db[78][18] = assign1050_e1473_d_b18;
        s.rv[78] = 0.0;

        let (assign1060_e1488, assign1060_e1488_d_n0, assign1060_e1488_d_n1, assign1060_e1488_d_n2, assign1060_e1488_d_n3, assign1060_e1488_d_n4, assign1060_e1488_d_n5, assign1060_e1488_d_n6, assign1060_e1488_d_n7, assign1060_e1488_d_n8, assign1060_e1488_d_n9, assign1060_e1488_d_n10, assign1060_e1488_d_n11, assign1060_e1488_d_n12, assign1060_e1488_d_n13, assign1060_e1488_d_n14, assign1060_e1488_d_n15, assign1060_e1488_d_n16, assign1060_e1488_d_n17, assign1060_e1488_d_n18, assign1060_e1488_d_b0, assign1060_e1488_d_b1, assign1060_e1488_d_b2, assign1060_e1488_d_b3, assign1060_e1488_d_b4, assign1060_e1488_d_b5, assign1060_e1488_d_b6, assign1060_e1488_d_b7, assign1060_e1488_d_b8, assign1060_e1488_d_b9, assign1060_e1488_d_b10, assign1060_e1488_d_b11, assign1060_e1488_d_b12, assign1060_e1488_d_b13, assign1060_e1488_d_b14, assign1060_e1488_d_b15, assign1060_e1488_d_b16, assign1060_e1488_d_b17, assign1060_e1488_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1060_e1485: f64 = (p.p15 * s.v[76]);
        let assign1060_e1486: f64 = (p.p14 + assign1060_e1485);
        (assign1060_e1486, (p.p15 * s.dn[76][0]), (p.p15 * s.dn[76][1]), (p.p15 * s.dn[76][2]), (p.p15 * s.dn[76][3]), (p.p15 * s.dn[76][4]), (p.p15 * s.dn[76][5]), (p.p15 * s.dn[76][6]), (p.p15 * s.dn[76][7]), (p.p15 * s.dn[76][8]), (p.p15 * s.dn[76][9]), (p.p15 * s.dn[76][10]), (p.p15 * s.dn[76][11]), (p.p15 * s.dn[76][12]), (p.p15 * s.dn[76][13]), (p.p15 * s.dn[76][14]), (p.p15 * s.dn[76][15]), (p.p15 * s.dn[76][16]), (p.p15 * s.dn[76][17]), (p.p15 * s.dn[76][18]), (p.p15 * s.db[76][0]), (p.p15 * s.db[76][1]), (p.p15 * s.db[76][2]), (p.p15 * s.db[76][3]), (p.p15 * s.db[76][4]), (p.p15 * s.db[76][5]), (p.p15 * s.db[76][6]), (p.p15 * s.db[76][7]), (p.p15 * s.db[76][8]), (p.p15 * s.db[76][9]), (p.p15 * s.db[76][10]), (p.p15 * s.db[76][11]), (p.p15 * s.db[76][12]), (p.p15 * s.db[76][13]), (p.p15 * s.db[76][14]), (p.p15 * s.db[76][15]), (p.p15 * s.db[76][16]), (p.p15 * s.db[76][17]), (p.p15 * s.db[76][18]),)
    } else {
        (s.v[1], s.dn[1][0], s.dn[1][1], s.dn[1][2], s.dn[1][3], s.dn[1][4], s.dn[1][5], s.dn[1][6], s.dn[1][7], s.dn[1][8], s.dn[1][9], s.dn[1][10], s.dn[1][11], s.dn[1][12], s.dn[1][13], s.dn[1][14], s.dn[1][15], s.dn[1][16], s.dn[1][17], s.dn[1][18], s.db[1][0], s.db[1][1], s.db[1][2], s.db[1][3], s.db[1][4], s.db[1][5], s.db[1][6], s.db[1][7], s.db[1][8], s.db[1][9], s.db[1][10], s.db[1][11], s.db[1][12], s.db[1][13], s.db[1][14], s.db[1][15], s.db[1][16], s.db[1][17], s.db[1][18],)
    }
};
        s.v[1] = assign1060_e1488;
        s.mark_derivatives_dirty(1);
        s.dn[1][0] = assign1060_e1488_d_n0;
        s.dn[1][1] = assign1060_e1488_d_n1;
        s.dn[1][2] = assign1060_e1488_d_n2;
        s.dn[1][3] = assign1060_e1488_d_n3;
        s.dn[1][4] = assign1060_e1488_d_n4;
        s.dn[1][5] = assign1060_e1488_d_n5;
        s.dn[1][6] = assign1060_e1488_d_n6;
        s.dn[1][7] = assign1060_e1488_d_n7;
        s.dn[1][8] = assign1060_e1488_d_n8;
        s.dn[1][9] = assign1060_e1488_d_n9;
        s.dn[1][10] = assign1060_e1488_d_n10;
        s.dn[1][11] = assign1060_e1488_d_n11;
        s.dn[1][12] = assign1060_e1488_d_n12;
        s.dn[1][13] = assign1060_e1488_d_n13;
        s.dn[1][14] = assign1060_e1488_d_n14;
        s.dn[1][15] = assign1060_e1488_d_n15;
        s.dn[1][16] = assign1060_e1488_d_n16;
        s.dn[1][17] = assign1060_e1488_d_n17;
        s.dn[1][18] = assign1060_e1488_d_n18;
        s.db[1][0] = assign1060_e1488_d_b0;
        s.db[1][1] = assign1060_e1488_d_b1;
        s.db[1][2] = assign1060_e1488_d_b2;
        s.db[1][3] = assign1060_e1488_d_b3;
        s.db[1][4] = assign1060_e1488_d_b4;
        s.db[1][5] = assign1060_e1488_d_b5;
        s.db[1][6] = assign1060_e1488_d_b6;
        s.db[1][7] = assign1060_e1488_d_b7;
        s.db[1][8] = assign1060_e1488_d_b8;
        s.db[1][9] = assign1060_e1488_d_b9;
        s.db[1][10] = assign1060_e1488_d_b10;
        s.db[1][11] = assign1060_e1488_d_b11;
        s.db[1][12] = assign1060_e1488_d_b12;
        s.db[1][13] = assign1060_e1488_d_b13;
        s.db[1][14] = assign1060_e1488_d_b14;
        s.db[1][15] = assign1060_e1488_d_b15;
        s.db[1][16] = assign1060_e1488_d_b16;
        s.db[1][17] = assign1060_e1488_d_b17;
        s.db[1][18] = assign1060_e1488_d_b18;
        s.rv[1] = 0.0;

        let (assign1070_e1503, assign1070_e1503_d_n0, assign1070_e1503_d_n1, assign1070_e1503_d_n2, assign1070_e1503_d_n3, assign1070_e1503_d_n4, assign1070_e1503_d_n5, assign1070_e1503_d_n6, assign1070_e1503_d_n7, assign1070_e1503_d_n8, assign1070_e1503_d_n9, assign1070_e1503_d_n10, assign1070_e1503_d_n11, assign1070_e1503_d_n12, assign1070_e1503_d_n13, assign1070_e1503_d_n14, assign1070_e1503_d_n15, assign1070_e1503_d_n16, assign1070_e1503_d_n17, assign1070_e1503_d_n18, assign1070_e1503_d_b0, assign1070_e1503_d_b1, assign1070_e1503_d_b2, assign1070_e1503_d_b3, assign1070_e1503_d_b4, assign1070_e1503_d_b5, assign1070_e1503_d_b6, assign1070_e1503_d_b7, assign1070_e1503_d_b8, assign1070_e1503_d_b9, assign1070_e1503_d_b10, assign1070_e1503_d_b11, assign1070_e1503_d_b12, assign1070_e1503_d_b13, assign1070_e1503_d_b14, assign1070_e1503_d_b15, assign1070_e1503_d_b16, assign1070_e1503_d_b17, assign1070_e1503_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1070_e1500: f64 = (p.p15 * s.v[78]);
        let assign1070_e1501: f64 = (p.p14 + assign1070_e1500);
        (assign1070_e1501, (p.p15 * s.dn[78][0]), (p.p15 * s.dn[78][1]), (p.p15 * s.dn[78][2]), (p.p15 * s.dn[78][3]), (p.p15 * s.dn[78][4]), (p.p15 * s.dn[78][5]), (p.p15 * s.dn[78][6]), (p.p15 * s.dn[78][7]), (p.p15 * s.dn[78][8]), (p.p15 * s.dn[78][9]), (p.p15 * s.dn[78][10]), (p.p15 * s.dn[78][11]), (p.p15 * s.dn[78][12]), (p.p15 * s.dn[78][13]), (p.p15 * s.dn[78][14]), (p.p15 * s.dn[78][15]), (p.p15 * s.dn[78][16]), (p.p15 * s.dn[78][17]), (p.p15 * s.dn[78][18]), (p.p15 * s.db[78][0]), (p.p15 * s.db[78][1]), (p.p15 * s.db[78][2]), (p.p15 * s.db[78][3]), (p.p15 * s.db[78][4]), (p.p15 * s.db[78][5]), (p.p15 * s.db[78][6]), (p.p15 * s.db[78][7]), (p.p15 * s.db[78][8]), (p.p15 * s.db[78][9]), (p.p15 * s.db[78][10]), (p.p15 * s.db[78][11]), (p.p15 * s.db[78][12]), (p.p15 * s.db[78][13]), (p.p15 * s.db[78][14]), (p.p15 * s.db[78][15]), (p.p15 * s.db[78][16]), (p.p15 * s.db[78][17]), (p.p15 * s.db[78][18]),)
    } else {
        (s.v[2], s.dn[2][0], s.dn[2][1], s.dn[2][2], s.dn[2][3], s.dn[2][4], s.dn[2][5], s.dn[2][6], s.dn[2][7], s.dn[2][8], s.dn[2][9], s.dn[2][10], s.dn[2][11], s.dn[2][12], s.dn[2][13], s.dn[2][14], s.dn[2][15], s.dn[2][16], s.dn[2][17], s.dn[2][18], s.db[2][0], s.db[2][1], s.db[2][2], s.db[2][3], s.db[2][4], s.db[2][5], s.db[2][6], s.db[2][7], s.db[2][8], s.db[2][9], s.db[2][10], s.db[2][11], s.db[2][12], s.db[2][13], s.db[2][14], s.db[2][15], s.db[2][16], s.db[2][17], s.db[2][18],)
    }
};
        s.v[2] = assign1070_e1503;
        s.mark_derivatives_dirty(2);
        s.dn[2][0] = assign1070_e1503_d_n0;
        s.dn[2][1] = assign1070_e1503_d_n1;
        s.dn[2][2] = assign1070_e1503_d_n2;
        s.dn[2][3] = assign1070_e1503_d_n3;
        s.dn[2][4] = assign1070_e1503_d_n4;
        s.dn[2][5] = assign1070_e1503_d_n5;
        s.dn[2][6] = assign1070_e1503_d_n6;
        s.dn[2][7] = assign1070_e1503_d_n7;
        s.dn[2][8] = assign1070_e1503_d_n8;
        s.dn[2][9] = assign1070_e1503_d_n9;
        s.dn[2][10] = assign1070_e1503_d_n10;
        s.dn[2][11] = assign1070_e1503_d_n11;
        s.dn[2][12] = assign1070_e1503_d_n12;
        s.dn[2][13] = assign1070_e1503_d_n13;
        s.dn[2][14] = assign1070_e1503_d_n14;
        s.dn[2][15] = assign1070_e1503_d_n15;
        s.dn[2][16] = assign1070_e1503_d_n16;
        s.dn[2][17] = assign1070_e1503_d_n17;
        s.dn[2][18] = assign1070_e1503_d_n18;
        s.db[2][0] = assign1070_e1503_d_b0;
        s.db[2][1] = assign1070_e1503_d_b1;
        s.db[2][2] = assign1070_e1503_d_b2;
        s.db[2][3] = assign1070_e1503_d_b3;
        s.db[2][4] = assign1070_e1503_d_b4;
        s.db[2][5] = assign1070_e1503_d_b5;
        s.db[2][6] = assign1070_e1503_d_b6;
        s.db[2][7] = assign1070_e1503_d_b7;
        s.db[2][8] = assign1070_e1503_d_b8;
        s.db[2][9] = assign1070_e1503_d_b9;
        s.db[2][10] = assign1070_e1503_d_b10;
        s.db[2][11] = assign1070_e1503_d_b11;
        s.db[2][12] = assign1070_e1503_d_b12;
        s.db[2][13] = assign1070_e1503_d_b13;
        s.db[2][14] = assign1070_e1503_d_b14;
        s.db[2][15] = assign1070_e1503_d_b15;
        s.db[2][16] = assign1070_e1503_d_b16;
        s.db[2][17] = assign1070_e1503_d_b17;
        s.db[2][18] = assign1070_e1503_d_b18;
        s.rv[2] = 0.0;

        let (assign1080_e1517, assign1080_e1517_d_n0, assign1080_e1517_d_n1, assign1080_e1517_d_n2, assign1080_e1517_d_n3, assign1080_e1517_d_n4, assign1080_e1517_d_n5, assign1080_e1517_d_n6, assign1080_e1517_d_n7, assign1080_e1517_d_n8, assign1080_e1517_d_n9, assign1080_e1517_d_n10, assign1080_e1517_d_n11, assign1080_e1517_d_n12, assign1080_e1517_d_n13, assign1080_e1517_d_n14, assign1080_e1517_d_n15, assign1080_e1517_d_n16, assign1080_e1517_d_n17, assign1080_e1517_d_n18, assign1080_e1517_d_b0, assign1080_e1517_d_b1, assign1080_e1517_d_b2, assign1080_e1517_d_b3, assign1080_e1517_d_b4, assign1080_e1517_d_b5, assign1080_e1517_d_b6, assign1080_e1517_d_b7, assign1080_e1517_d_b8, assign1080_e1517_d_b9, assign1080_e1517_d_b10, assign1080_e1517_d_b11, assign1080_e1517_d_b12, assign1080_e1517_d_b13, assign1080_e1517_d_b14, assign1080_e1517_d_b15, assign1080_e1517_d_b16, assign1080_e1517_d_b17, assign1080_e1517_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1080_e1514: f64 = (s.v[1] * s.v[5]);
        let assign1080_e1515: f64 = (assign1080_e1514).tanh();
        (assign1080_e1515, (((s.dn[1][0] * s.v[5]) + (s.v[1] * s.dn[5][0])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][1] * s.v[5]) + (s.v[1] * s.dn[5][1])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][2] * s.v[5]) + (s.v[1] * s.dn[5][2])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][3] * s.v[5]) + (s.v[1] * s.dn[5][3])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][4] * s.v[5]) + (s.v[1] * s.dn[5][4])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][5] * s.v[5]) + (s.v[1] * s.dn[5][5])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][6] * s.v[5]) + (s.v[1] * s.dn[5][6])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][7] * s.v[5]) + (s.v[1] * s.dn[5][7])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][8] * s.v[5]) + (s.v[1] * s.dn[5][8])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][9] * s.v[5]) + (s.v[1] * s.dn[5][9])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][10] * s.v[5]) + (s.v[1] * s.dn[5][10])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][11] * s.v[5]) + (s.v[1] * s.dn[5][11])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][12] * s.v[5]) + (s.v[1] * s.dn[5][12])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][13] * s.v[5]) + (s.v[1] * s.dn[5][13])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][14] * s.v[5]) + (s.v[1] * s.dn[5][14])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][15] * s.v[5]) + (s.v[1] * s.dn[5][15])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][16] * s.v[5]) + (s.v[1] * s.dn[5][16])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][17] * s.v[5]) + (s.v[1] * s.dn[5][17])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.dn[1][18] * s.v[5]) + (s.v[1] * s.dn[5][18])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][0] * s.v[5]) + (s.v[1] * s.db[5][0])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][1] * s.v[5]) + (s.v[1] * s.db[5][1])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][2] * s.v[5]) + (s.v[1] * s.db[5][2])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][3] * s.v[5]) + (s.v[1] * s.db[5][3])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][4] * s.v[5]) + (s.v[1] * s.db[5][4])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][5] * s.v[5]) + (s.v[1] * s.db[5][5])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][6] * s.v[5]) + (s.v[1] * s.db[5][6])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][7] * s.v[5]) + (s.v[1] * s.db[5][7])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][8] * s.v[5]) + (s.v[1] * s.db[5][8])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][9] * s.v[5]) + (s.v[1] * s.db[5][9])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][10] * s.v[5]) + (s.v[1] * s.db[5][10])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][11] * s.v[5]) + (s.v[1] * s.db[5][11])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][12] * s.v[5]) + (s.v[1] * s.db[5][12])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][13] * s.v[5]) + (s.v[1] * s.db[5][13])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][14] * s.v[5]) + (s.v[1] * s.db[5][14])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][15] * s.v[5]) + (s.v[1] * s.db[5][15])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][16] * s.v[5]) + (s.v[1] * s.db[5][16])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][17] * s.v[5]) + (s.v[1] * s.db[5][17])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())), (((s.db[1][18] * s.v[5]) + (s.v[1] * s.db[5][18])) / ((assign1080_e1514).cosh() * (assign1080_e1514).cosh())),)
    } else {
        (s.v[81], s.dn[81][0], s.dn[81][1], s.dn[81][2], s.dn[81][3], s.dn[81][4], s.dn[81][5], s.dn[81][6], s.dn[81][7], s.dn[81][8], s.dn[81][9], s.dn[81][10], s.dn[81][11], s.dn[81][12], s.dn[81][13], s.dn[81][14], s.dn[81][15], s.dn[81][16], s.dn[81][17], s.dn[81][18], s.db[81][0], s.db[81][1], s.db[81][2], s.db[81][3], s.db[81][4], s.db[81][5], s.db[81][6], s.db[81][7], s.db[81][8], s.db[81][9], s.db[81][10], s.db[81][11], s.db[81][12], s.db[81][13], s.db[81][14], s.db[81][15], s.db[81][16], s.db[81][17], s.db[81][18],)
    }
};
        s.v[81] = assign1080_e1517;
        s.mark_derivatives_dirty(81);
        s.dn[81][0] = assign1080_e1517_d_n0;
        s.dn[81][1] = assign1080_e1517_d_n1;
        s.dn[81][2] = assign1080_e1517_d_n2;
        s.dn[81][3] = assign1080_e1517_d_n3;
        s.dn[81][4] = assign1080_e1517_d_n4;
        s.dn[81][5] = assign1080_e1517_d_n5;
        s.dn[81][6] = assign1080_e1517_d_n6;
        s.dn[81][7] = assign1080_e1517_d_n7;
        s.dn[81][8] = assign1080_e1517_d_n8;
        s.dn[81][9] = assign1080_e1517_d_n9;
        s.dn[81][10] = assign1080_e1517_d_n10;
        s.dn[81][11] = assign1080_e1517_d_n11;
        s.dn[81][12] = assign1080_e1517_d_n12;
        s.dn[81][13] = assign1080_e1517_d_n13;
        s.dn[81][14] = assign1080_e1517_d_n14;
        s.dn[81][15] = assign1080_e1517_d_n15;
        s.dn[81][16] = assign1080_e1517_d_n16;
        s.dn[81][17] = assign1080_e1517_d_n17;
        s.dn[81][18] = assign1080_e1517_d_n18;
        s.db[81][0] = assign1080_e1517_d_b0;
        s.db[81][1] = assign1080_e1517_d_b1;
        s.db[81][2] = assign1080_e1517_d_b2;
        s.db[81][3] = assign1080_e1517_d_b3;
        s.db[81][4] = assign1080_e1517_d_b4;
        s.db[81][5] = assign1080_e1517_d_b5;
        s.db[81][6] = assign1080_e1517_d_b6;
        s.db[81][7] = assign1080_e1517_d_b7;
        s.db[81][8] = assign1080_e1517_d_b8;
        s.db[81][9] = assign1080_e1517_d_b9;
        s.db[81][10] = assign1080_e1517_d_b10;
        s.db[81][11] = assign1080_e1517_d_b11;
        s.db[81][12] = assign1080_e1517_d_b12;
        s.db[81][13] = assign1080_e1517_d_b13;
        s.db[81][14] = assign1080_e1517_d_b14;
        s.db[81][15] = assign1080_e1517_d_b15;
        s.db[81][16] = assign1080_e1517_d_b16;
        s.db[81][17] = assign1080_e1517_d_b17;
        s.db[81][18] = assign1080_e1517_d_b18;
        s.rv[81] = 0.0;

        let (assign1090_e1531, assign1090_e1531_d_n0, assign1090_e1531_d_n1, assign1090_e1531_d_n2, assign1090_e1531_d_n3, assign1090_e1531_d_n4, assign1090_e1531_d_n5, assign1090_e1531_d_n6, assign1090_e1531_d_n7, assign1090_e1531_d_n8, assign1090_e1531_d_n9, assign1090_e1531_d_n10, assign1090_e1531_d_n11, assign1090_e1531_d_n12, assign1090_e1531_d_n13, assign1090_e1531_d_n14, assign1090_e1531_d_n15, assign1090_e1531_d_n16, assign1090_e1531_d_n17, assign1090_e1531_d_n18, assign1090_e1531_d_b0, assign1090_e1531_d_b1, assign1090_e1531_d_b2, assign1090_e1531_d_b3, assign1090_e1531_d_b4, assign1090_e1531_d_b5, assign1090_e1531_d_b6, assign1090_e1531_d_b7, assign1090_e1531_d_b8, assign1090_e1531_d_b9, assign1090_e1531_d_b10, assign1090_e1531_d_b11, assign1090_e1531_d_b12, assign1090_e1531_d_b13, assign1090_e1531_d_b14, assign1090_e1531_d_b15, assign1090_e1531_d_b16, assign1090_e1531_d_b17, assign1090_e1531_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1090_e1528: f64 = (s.v[2] * s.v[5]);
        let assign1090_e1529: f64 = (assign1090_e1528).tanh();
        (assign1090_e1529, (((s.dn[2][0] * s.v[5]) + (s.v[2] * s.dn[5][0])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][1] * s.v[5]) + (s.v[2] * s.dn[5][1])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][2] * s.v[5]) + (s.v[2] * s.dn[5][2])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][3] * s.v[5]) + (s.v[2] * s.dn[5][3])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][4] * s.v[5]) + (s.v[2] * s.dn[5][4])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][5] * s.v[5]) + (s.v[2] * s.dn[5][5])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][6] * s.v[5]) + (s.v[2] * s.dn[5][6])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][7] * s.v[5]) + (s.v[2] * s.dn[5][7])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][8] * s.v[5]) + (s.v[2] * s.dn[5][8])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][9] * s.v[5]) + (s.v[2] * s.dn[5][9])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][10] * s.v[5]) + (s.v[2] * s.dn[5][10])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][11] * s.v[5]) + (s.v[2] * s.dn[5][11])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][12] * s.v[5]) + (s.v[2] * s.dn[5][12])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][13] * s.v[5]) + (s.v[2] * s.dn[5][13])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][14] * s.v[5]) + (s.v[2] * s.dn[5][14])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][15] * s.v[5]) + (s.v[2] * s.dn[5][15])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][16] * s.v[5]) + (s.v[2] * s.dn[5][16])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][17] * s.v[5]) + (s.v[2] * s.dn[5][17])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.dn[2][18] * s.v[5]) + (s.v[2] * s.dn[5][18])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][0] * s.v[5]) + (s.v[2] * s.db[5][0])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][1] * s.v[5]) + (s.v[2] * s.db[5][1])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][2] * s.v[5]) + (s.v[2] * s.db[5][2])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][3] * s.v[5]) + (s.v[2] * s.db[5][3])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][4] * s.v[5]) + (s.v[2] * s.db[5][4])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][5] * s.v[5]) + (s.v[2] * s.db[5][5])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][6] * s.v[5]) + (s.v[2] * s.db[5][6])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][7] * s.v[5]) + (s.v[2] * s.db[5][7])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][8] * s.v[5]) + (s.v[2] * s.db[5][8])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][9] * s.v[5]) + (s.v[2] * s.db[5][9])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][10] * s.v[5]) + (s.v[2] * s.db[5][10])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][11] * s.v[5]) + (s.v[2] * s.db[5][11])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][12] * s.v[5]) + (s.v[2] * s.db[5][12])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][13] * s.v[5]) + (s.v[2] * s.db[5][13])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][14] * s.v[5]) + (s.v[2] * s.db[5][14])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][15] * s.v[5]) + (s.v[2] * s.db[5][15])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][16] * s.v[5]) + (s.v[2] * s.db[5][16])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][17] * s.v[5]) + (s.v[2] * s.db[5][17])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())), (((s.db[2][18] * s.v[5]) + (s.v[2] * s.db[5][18])) / ((assign1090_e1528).cosh() * (assign1090_e1528).cosh())),)
    } else {
        (s.v[82], s.dn[82][0], s.dn[82][1], s.dn[82][2], s.dn[82][3], s.dn[82][4], s.dn[82][5], s.dn[82][6], s.dn[82][7], s.dn[82][8], s.dn[82][9], s.dn[82][10], s.dn[82][11], s.dn[82][12], s.dn[82][13], s.dn[82][14], s.dn[82][15], s.dn[82][16], s.dn[82][17], s.dn[82][18], s.db[82][0], s.db[82][1], s.db[82][2], s.db[82][3], s.db[82][4], s.db[82][5], s.db[82][6], s.db[82][7], s.db[82][8], s.db[82][9], s.db[82][10], s.db[82][11], s.db[82][12], s.db[82][13], s.db[82][14], s.db[82][15], s.db[82][16], s.db[82][17], s.db[82][18],)
    }
};
        s.v[82] = assign1090_e1531;
        s.mark_derivatives_dirty(82);
        s.dn[82][0] = assign1090_e1531_d_n0;
        s.dn[82][1] = assign1090_e1531_d_n1;
        s.dn[82][2] = assign1090_e1531_d_n2;
        s.dn[82][3] = assign1090_e1531_d_n3;
        s.dn[82][4] = assign1090_e1531_d_n4;
        s.dn[82][5] = assign1090_e1531_d_n5;
        s.dn[82][6] = assign1090_e1531_d_n6;
        s.dn[82][7] = assign1090_e1531_d_n7;
        s.dn[82][8] = assign1090_e1531_d_n8;
        s.dn[82][9] = assign1090_e1531_d_n9;
        s.dn[82][10] = assign1090_e1531_d_n10;
        s.dn[82][11] = assign1090_e1531_d_n11;
        s.dn[82][12] = assign1090_e1531_d_n12;
        s.dn[82][13] = assign1090_e1531_d_n13;
        s.dn[82][14] = assign1090_e1531_d_n14;
        s.dn[82][15] = assign1090_e1531_d_n15;
        s.dn[82][16] = assign1090_e1531_d_n16;
        s.dn[82][17] = assign1090_e1531_d_n17;
        s.dn[82][18] = assign1090_e1531_d_n18;
        s.db[82][0] = assign1090_e1531_d_b0;
        s.db[82][1] = assign1090_e1531_d_b1;
        s.db[82][2] = assign1090_e1531_d_b2;
        s.db[82][3] = assign1090_e1531_d_b3;
        s.db[82][4] = assign1090_e1531_d_b4;
        s.db[82][5] = assign1090_e1531_d_b5;
        s.db[82][6] = assign1090_e1531_d_b6;
        s.db[82][7] = assign1090_e1531_d_b7;
        s.db[82][8] = assign1090_e1531_d_b8;
        s.db[82][9] = assign1090_e1531_d_b9;
        s.db[82][10] = assign1090_e1531_d_b10;
        s.db[82][11] = assign1090_e1531_d_b11;
        s.db[82][12] = assign1090_e1531_d_b12;
        s.db[82][13] = assign1090_e1531_d_b13;
        s.db[82][14] = assign1090_e1531_d_b14;
        s.db[82][15] = assign1090_e1531_d_b15;
        s.db[82][16] = assign1090_e1531_d_b16;
        s.db[82][17] = assign1090_e1531_d_b17;
        s.db[82][18] = assign1090_e1531_d_b18;
        s.rv[82] = 0.0;

        let (assign1100_e1546, assign1100_e1546_d_n0, assign1100_e1546_d_n1, assign1100_e1546_d_n2, assign1100_e1546_d_n3, assign1100_e1546_d_n4, assign1100_e1546_d_n5, assign1100_e1546_d_n6, assign1100_e1546_d_n7, assign1100_e1546_d_n8, assign1100_e1546_d_n9, assign1100_e1546_d_n10, assign1100_e1546_d_n11, assign1100_e1546_d_n12, assign1100_e1546_d_n13, assign1100_e1546_d_n14, assign1100_e1546_d_n15, assign1100_e1546_d_n16, assign1100_e1546_d_n17, assign1100_e1546_d_n18, assign1100_e1546_d_b0, assign1100_e1546_d_b1, assign1100_e1546_d_b2, assign1100_e1546_d_b3, assign1100_e1546_d_b4, assign1100_e1546_d_b5, assign1100_e1546_d_b6, assign1100_e1546_d_b7, assign1100_e1546_d_b8, assign1100_e1546_d_b9, assign1100_e1546_d_b10, assign1100_e1546_d_b11, assign1100_e1546_d_b12, assign1100_e1546_d_b13, assign1100_e1546_d_b14, assign1100_e1546_d_b15, assign1100_e1546_d_b16, assign1100_e1546_d_b17, assign1100_e1546_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1100_e1543: f64 = (p.p17 * s.v[78]);
        let assign1100_e1544: f64 = (p.p16 + assign1100_e1543);
        (assign1100_e1544, (p.p17 * s.dn[78][0]), (p.p17 * s.dn[78][1]), (p.p17 * s.dn[78][2]), (p.p17 * s.dn[78][3]), (p.p17 * s.dn[78][4]), (p.p17 * s.dn[78][5]), (p.p17 * s.dn[78][6]), (p.p17 * s.dn[78][7]), (p.p17 * s.dn[78][8]), (p.p17 * s.dn[78][9]), (p.p17 * s.dn[78][10]), (p.p17 * s.dn[78][11]), (p.p17 * s.dn[78][12]), (p.p17 * s.dn[78][13]), (p.p17 * s.dn[78][14]), (p.p17 * s.dn[78][15]), (p.p17 * s.dn[78][16]), (p.p17 * s.dn[78][17]), (p.p17 * s.dn[78][18]), (p.p17 * s.db[78][0]), (p.p17 * s.db[78][1]), (p.p17 * s.db[78][2]), (p.p17 * s.db[78][3]), (p.p17 * s.db[78][4]), (p.p17 * s.db[78][5]), (p.p17 * s.db[78][6]), (p.p17 * s.db[78][7]), (p.p17 * s.db[78][8]), (p.p17 * s.db[78][9]), (p.p17 * s.db[78][10]), (p.p17 * s.db[78][11]), (p.p17 * s.db[78][12]), (p.p17 * s.db[78][13]), (p.p17 * s.db[78][14]), (p.p17 * s.db[78][15]), (p.p17 * s.db[78][16]), (p.p17 * s.db[78][17]), (p.p17 * s.db[78][18]),)
    } else {
        (s.v[68], s.dn[68][0], s.dn[68][1], s.dn[68][2], s.dn[68][3], s.dn[68][4], s.dn[68][5], s.dn[68][6], s.dn[68][7], s.dn[68][8], s.dn[68][9], s.dn[68][10], s.dn[68][11], s.dn[68][12], s.dn[68][13], s.dn[68][14], s.dn[68][15], s.dn[68][16], s.dn[68][17], s.dn[68][18], s.db[68][0], s.db[68][1], s.db[68][2], s.db[68][3], s.db[68][4], s.db[68][5], s.db[68][6], s.db[68][7], s.db[68][8], s.db[68][9], s.db[68][10], s.db[68][11], s.db[68][12], s.db[68][13], s.db[68][14], s.db[68][15], s.db[68][16], s.db[68][17], s.db[68][18],)
    }
};
        s.v[68] = assign1100_e1546;
        s.mark_derivatives_dirty(68);
        s.dn[68][0] = assign1100_e1546_d_n0;
        s.dn[68][1] = assign1100_e1546_d_n1;
        s.dn[68][2] = assign1100_e1546_d_n2;
        s.dn[68][3] = assign1100_e1546_d_n3;
        s.dn[68][4] = assign1100_e1546_d_n4;
        s.dn[68][5] = assign1100_e1546_d_n5;
        s.dn[68][6] = assign1100_e1546_d_n6;
        s.dn[68][7] = assign1100_e1546_d_n7;
        s.dn[68][8] = assign1100_e1546_d_n8;
        s.dn[68][9] = assign1100_e1546_d_n9;
        s.dn[68][10] = assign1100_e1546_d_n10;
        s.dn[68][11] = assign1100_e1546_d_n11;
        s.dn[68][12] = assign1100_e1546_d_n12;
        s.dn[68][13] = assign1100_e1546_d_n13;
        s.dn[68][14] = assign1100_e1546_d_n14;
        s.dn[68][15] = assign1100_e1546_d_n15;
        s.dn[68][16] = assign1100_e1546_d_n16;
        s.dn[68][17] = assign1100_e1546_d_n17;
        s.dn[68][18] = assign1100_e1546_d_n18;
        s.db[68][0] = assign1100_e1546_d_b0;
        s.db[68][1] = assign1100_e1546_d_b1;
        s.db[68][2] = assign1100_e1546_d_b2;
        s.db[68][3] = assign1100_e1546_d_b3;
        s.db[68][4] = assign1100_e1546_d_b4;
        s.db[68][5] = assign1100_e1546_d_b5;
        s.db[68][6] = assign1100_e1546_d_b6;
        s.db[68][7] = assign1100_e1546_d_b7;
        s.db[68][8] = assign1100_e1546_d_b8;
        s.db[68][9] = assign1100_e1546_d_b9;
        s.db[68][10] = assign1100_e1546_d_b10;
        s.db[68][11] = assign1100_e1546_d_b11;
        s.db[68][12] = assign1100_e1546_d_b12;
        s.db[68][13] = assign1100_e1546_d_b13;
        s.db[68][14] = assign1100_e1546_d_b14;
        s.db[68][15] = assign1100_e1546_d_b15;
        s.db[68][16] = assign1100_e1546_d_b16;
        s.db[68][17] = assign1100_e1546_d_b17;
        s.db[68][18] = assign1100_e1546_d_b18;
        s.rv[68] = 0.0;

        let (assign1110_e1561, assign1110_e1561_d_n0, assign1110_e1561_d_n1, assign1110_e1561_d_n2, assign1110_e1561_d_n3, assign1110_e1561_d_n4, assign1110_e1561_d_n5, assign1110_e1561_d_n6, assign1110_e1561_d_n7, assign1110_e1561_d_n8, assign1110_e1561_d_n9, assign1110_e1561_d_n10, assign1110_e1561_d_n11, assign1110_e1561_d_n12, assign1110_e1561_d_n13, assign1110_e1561_d_n14, assign1110_e1561_d_n15, assign1110_e1561_d_n16, assign1110_e1561_d_n17, assign1110_e1561_d_n18, assign1110_e1561_d_b0, assign1110_e1561_d_b1, assign1110_e1561_d_b2, assign1110_e1561_d_b3, assign1110_e1561_d_b4, assign1110_e1561_d_b5, assign1110_e1561_d_b6, assign1110_e1561_d_b7, assign1110_e1561_d_b8, assign1110_e1561_d_b9, assign1110_e1561_d_b10, assign1110_e1561_d_b11, assign1110_e1561_d_b12, assign1110_e1561_d_b13, assign1110_e1561_d_b14, assign1110_e1561_d_b15, assign1110_e1561_d_b16, assign1110_e1561_d_b17, assign1110_e1561_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1110_e1558: f64 = (p.p17 * s.v[76]);
        let assign1110_e1559: f64 = (p.p16 + assign1110_e1558);
        (assign1110_e1559, (p.p17 * s.dn[76][0]), (p.p17 * s.dn[76][1]), (p.p17 * s.dn[76][2]), (p.p17 * s.dn[76][3]), (p.p17 * s.dn[76][4]), (p.p17 * s.dn[76][5]), (p.p17 * s.dn[76][6]), (p.p17 * s.dn[76][7]), (p.p17 * s.dn[76][8]), (p.p17 * s.dn[76][9]), (p.p17 * s.dn[76][10]), (p.p17 * s.dn[76][11]), (p.p17 * s.dn[76][12]), (p.p17 * s.dn[76][13]), (p.p17 * s.dn[76][14]), (p.p17 * s.dn[76][15]), (p.p17 * s.dn[76][16]), (p.p17 * s.dn[76][17]), (p.p17 * s.dn[76][18]), (p.p17 * s.db[76][0]), (p.p17 * s.db[76][1]), (p.p17 * s.db[76][2]), (p.p17 * s.db[76][3]), (p.p17 * s.db[76][4]), (p.p17 * s.db[76][5]), (p.p17 * s.db[76][6]), (p.p17 * s.db[76][7]), (p.p17 * s.db[76][8]), (p.p17 * s.db[76][9]), (p.p17 * s.db[76][10]), (p.p17 * s.db[76][11]), (p.p17 * s.db[76][12]), (p.p17 * s.db[76][13]), (p.p17 * s.db[76][14]), (p.p17 * s.db[76][15]), (p.p17 * s.db[76][16]), (p.p17 * s.db[76][17]), (p.p17 * s.db[76][18]),)
    } else {
        (s.v[70], s.dn[70][0], s.dn[70][1], s.dn[70][2], s.dn[70][3], s.dn[70][4], s.dn[70][5], s.dn[70][6], s.dn[70][7], s.dn[70][8], s.dn[70][9], s.dn[70][10], s.dn[70][11], s.dn[70][12], s.dn[70][13], s.dn[70][14], s.dn[70][15], s.dn[70][16], s.dn[70][17], s.dn[70][18], s.db[70][0], s.db[70][1], s.db[70][2], s.db[70][3], s.db[70][4], s.db[70][5], s.db[70][6], s.db[70][7], s.db[70][8], s.db[70][9], s.db[70][10], s.db[70][11], s.db[70][12], s.db[70][13], s.db[70][14], s.db[70][15], s.db[70][16], s.db[70][17], s.db[70][18],)
    }
};
        s.v[70] = assign1110_e1561;
        s.mark_derivatives_dirty(70);
        s.dn[70][0] = assign1110_e1561_d_n0;
        s.dn[70][1] = assign1110_e1561_d_n1;
        s.dn[70][2] = assign1110_e1561_d_n2;
        s.dn[70][3] = assign1110_e1561_d_n3;
        s.dn[70][4] = assign1110_e1561_d_n4;
        s.dn[70][5] = assign1110_e1561_d_n5;
        s.dn[70][6] = assign1110_e1561_d_n6;
        s.dn[70][7] = assign1110_e1561_d_n7;
        s.dn[70][8] = assign1110_e1561_d_n8;
        s.dn[70][9] = assign1110_e1561_d_n9;
        s.dn[70][10] = assign1110_e1561_d_n10;
        s.dn[70][11] = assign1110_e1561_d_n11;
        s.dn[70][12] = assign1110_e1561_d_n12;
        s.dn[70][13] = assign1110_e1561_d_n13;
        s.dn[70][14] = assign1110_e1561_d_n14;
        s.dn[70][15] = assign1110_e1561_d_n15;
        s.dn[70][16] = assign1110_e1561_d_n16;
        s.dn[70][17] = assign1110_e1561_d_n17;
        s.dn[70][18] = assign1110_e1561_d_n18;
        s.db[70][0] = assign1110_e1561_d_b0;
        s.db[70][1] = assign1110_e1561_d_b1;
        s.db[70][2] = assign1110_e1561_d_b2;
        s.db[70][3] = assign1110_e1561_d_b3;
        s.db[70][4] = assign1110_e1561_d_b4;
        s.db[70][5] = assign1110_e1561_d_b5;
        s.db[70][6] = assign1110_e1561_d_b6;
        s.db[70][7] = assign1110_e1561_d_b7;
        s.db[70][8] = assign1110_e1561_d_b8;
        s.db[70][9] = assign1110_e1561_d_b9;
        s.db[70][10] = assign1110_e1561_d_b10;
        s.db[70][11] = assign1110_e1561_d_b11;
        s.db[70][12] = assign1110_e1561_d_b12;
        s.db[70][13] = assign1110_e1561_d_b13;
        s.db[70][14] = assign1110_e1561_d_b14;
        s.db[70][15] = assign1110_e1561_d_b15;
        s.db[70][16] = assign1110_e1561_d_b16;
        s.db[70][17] = assign1110_e1561_d_b17;
        s.db[70][18] = assign1110_e1561_d_b18;
        s.rv[70] = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let (assign1120_e1593, assign1120_e1593_d_n0, assign1120_e1593_d_n1, assign1120_e1593_d_n2, assign1120_e1593_d_n3, assign1120_e1593_d_n4, assign1120_e1593_d_n5, assign1120_e1593_d_n6, assign1120_e1593_d_n7, assign1120_e1593_d_n8, assign1120_e1593_d_n9, assign1120_e1593_d_n10, assign1120_e1593_d_n11, assign1120_e1593_d_n12, assign1120_e1593_d_n13, assign1120_e1593_d_n14, assign1120_e1593_d_n15, assign1120_e1593_d_n16, assign1120_e1593_d_n17, assign1120_e1593_d_n18, assign1120_e1593_d_b0, assign1120_e1593_d_b1, assign1120_e1593_d_b2, assign1120_e1593_d_b3, assign1120_e1593_d_b4, assign1120_e1593_d_b5, assign1120_e1593_d_b6, assign1120_e1593_d_b7, assign1120_e1593_d_b8, assign1120_e1593_d_b9, assign1120_e1593_d_b10, assign1120_e1593_d_b11, assign1120_e1593_d_b12, assign1120_e1593_d_b13, assign1120_e1593_d_b14, assign1120_e1593_d_b15, assign1120_e1593_d_b16, assign1120_e1593_d_b17, assign1120_e1593_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1120_e1572: f64 = (s.v[39] * s.v[76]);
        let assign1120_e1575: f64 = (1.0 + s.v[81]);
        let assign1120_e1576: f64 = (assign1120_e1572 * assign1120_e1575);
        let assign1120_e1580: f64 = (s.v[70] * s.v[5]);
        let assign1120_e1581: f64 = (1.0 + assign1120_e1580);
        let assign1120_e1586: f64 = (s.v[5] - s.v[53]);
        let assign1120_e1587: f64 = (p.p23 * assign1120_e1586);
        let assign1120_e1588: f64 = { let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1120_e1589: f64 = (s.v[43] * assign1120_e1588);
        let assign1120_e1590: f64 = (assign1120_e1581 + assign1120_e1589);
        let assign1120_e1591: f64 = (assign1120_e1576 * assign1120_e1590);
        (assign1120_e1591, ((((((s.dn[39][0] * s.v[76]) + (s.v[39] * s.dn[76][0])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][0])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][0] * s.v[5]) + (s.v[70] * s.dn[5][0])) + ((s.dn[43][0] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][0] - s.dn[53][0])))))))), ((((((s.dn[39][1] * s.v[76]) + (s.v[39] * s.dn[76][1])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][1])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][1] * s.v[5]) + (s.v[70] * s.dn[5][1])) + ((s.dn[43][1] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][1] - s.dn[53][1])))))))), ((((((s.dn[39][2] * s.v[76]) + (s.v[39] * s.dn[76][2])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][2])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][2] * s.v[5]) + (s.v[70] * s.dn[5][2])) + ((s.dn[43][2] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][2] - s.dn[53][2])))))))), ((((((s.dn[39][3] * s.v[76]) + (s.v[39] * s.dn[76][3])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][3])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][3] * s.v[5]) + (s.v[70] * s.dn[5][3])) + ((s.dn[43][3] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][3] - s.dn[53][3])))))))), ((((((s.dn[39][4] * s.v[76]) + (s.v[39] * s.dn[76][4])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][4])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][4] * s.v[5]) + (s.v[70] * s.dn[5][4])) + ((s.dn[43][4] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][4] - s.dn[53][4])))))))), ((((((s.dn[39][5] * s.v[76]) + (s.v[39] * s.dn[76][5])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][5])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][5] * s.v[5]) + (s.v[70] * s.dn[5][5])) + ((s.dn[43][5] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][5] - s.dn[53][5])))))))), ((((((s.dn[39][6] * s.v[76]) + (s.v[39] * s.dn[76][6])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][6])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][6] * s.v[5]) + (s.v[70] * s.dn[5][6])) + ((s.dn[43][6] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][6] - s.dn[53][6])))))))), ((((((s.dn[39][7] * s.v[76]) + (s.v[39] * s.dn[76][7])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][7])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][7] * s.v[5]) + (s.v[70] * s.dn[5][7])) + ((s.dn[43][7] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][7] - s.dn[53][7])))))))), ((((((s.dn[39][8] * s.v[76]) + (s.v[39] * s.dn[76][8])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][8])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][8] * s.v[5]) + (s.v[70] * s.dn[5][8])) + ((s.dn[43][8] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][8] - s.dn[53][8])))))))), ((((((s.dn[39][9] * s.v[76]) + (s.v[39] * s.dn[76][9])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][9])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][9] * s.v[5]) + (s.v[70] * s.dn[5][9])) + ((s.dn[43][9] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][9] - s.dn[53][9])))))))), ((((((s.dn[39][10] * s.v[76]) + (s.v[39] * s.dn[76][10])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][10])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][10] * s.v[5]) + (s.v[70] * s.dn[5][10])) + ((s.dn[43][10] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][10] - s.dn[53][10])))))))), ((((((s.dn[39][11] * s.v[76]) + (s.v[39] * s.dn[76][11])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][11])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][11] * s.v[5]) + (s.v[70] * s.dn[5][11])) + ((s.dn[43][11] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][11] - s.dn[53][11])))))))), ((((((s.dn[39][12] * s.v[76]) + (s.v[39] * s.dn[76][12])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][12])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][12] * s.v[5]) + (s.v[70] * s.dn[5][12])) + ((s.dn[43][12] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][12] - s.dn[53][12])))))))), ((((((s.dn[39][13] * s.v[76]) + (s.v[39] * s.dn[76][13])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][13])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][13] * s.v[5]) + (s.v[70] * s.dn[5][13])) + ((s.dn[43][13] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][13] - s.dn[53][13])))))))), ((((((s.dn[39][14] * s.v[76]) + (s.v[39] * s.dn[76][14])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][14])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][14] * s.v[5]) + (s.v[70] * s.dn[5][14])) + ((s.dn[43][14] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][14] - s.dn[53][14])))))))), ((((((s.dn[39][15] * s.v[76]) + (s.v[39] * s.dn[76][15])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][15])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][15] * s.v[5]) + (s.v[70] * s.dn[5][15])) + ((s.dn[43][15] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][15] - s.dn[53][15])))))))), ((((((s.dn[39][16] * s.v[76]) + (s.v[39] * s.dn[76][16])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][16])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][16] * s.v[5]) + (s.v[70] * s.dn[5][16])) + ((s.dn[43][16] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][16] - s.dn[53][16])))))))), ((((((s.dn[39][17] * s.v[76]) + (s.v[39] * s.dn[76][17])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][17])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][17] * s.v[5]) + (s.v[70] * s.dn[5][17])) + ((s.dn[43][17] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][17] - s.dn[53][17])))))))), ((((((s.dn[39][18] * s.v[76]) + (s.v[39] * s.dn[76][18])) * assign1120_e1575) + (assign1120_e1572 * s.dn[81][18])) * assign1120_e1590) + (assign1120_e1576 * (((s.dn[70][18] * s.v[5]) + (s.v[70] * s.dn[5][18])) + ((s.dn[43][18] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][18] - s.dn[53][18])))))))), ((((((s.db[39][0] * s.v[76]) + (s.v[39] * s.db[76][0])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][0])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][0] * s.v[5]) + (s.v[70] * s.db[5][0])) + ((s.db[43][0] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][0] - s.db[53][0])))))))), ((((((s.db[39][1] * s.v[76]) + (s.v[39] * s.db[76][1])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][1])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][1] * s.v[5]) + (s.v[70] * s.db[5][1])) + ((s.db[43][1] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][1] - s.db[53][1])))))))), ((((((s.db[39][2] * s.v[76]) + (s.v[39] * s.db[76][2])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][2])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][2] * s.v[5]) + (s.v[70] * s.db[5][2])) + ((s.db[43][2] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][2] - s.db[53][2])))))))), ((((((s.db[39][3] * s.v[76]) + (s.v[39] * s.db[76][3])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][3])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][3] * s.v[5]) + (s.v[70] * s.db[5][3])) + ((s.db[43][3] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][3] - s.db[53][3])))))))), ((((((s.db[39][4] * s.v[76]) + (s.v[39] * s.db[76][4])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][4])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][4] * s.v[5]) + (s.v[70] * s.db[5][4])) + ((s.db[43][4] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][4] - s.db[53][4])))))))), ((((((s.db[39][5] * s.v[76]) + (s.v[39] * s.db[76][5])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][5])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][5] * s.v[5]) + (s.v[70] * s.db[5][5])) + ((s.db[43][5] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][5] - s.db[53][5])))))))), ((((((s.db[39][6] * s.v[76]) + (s.v[39] * s.db[76][6])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][6])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][6] * s.v[5]) + (s.v[70] * s.db[5][6])) + ((s.db[43][6] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][6] - s.db[53][6])))))))), ((((((s.db[39][7] * s.v[76]) + (s.v[39] * s.db[76][7])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][7])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][7] * s.v[5]) + (s.v[70] * s.db[5][7])) + ((s.db[43][7] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][7] - s.db[53][7])))))))), ((((((s.db[39][8] * s.v[76]) + (s.v[39] * s.db[76][8])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][8])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][8] * s.v[5]) + (s.v[70] * s.db[5][8])) + ((s.db[43][8] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][8] - s.db[53][8])))))))), ((((((s.db[39][9] * s.v[76]) + (s.v[39] * s.db[76][9])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][9])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][9] * s.v[5]) + (s.v[70] * s.db[5][9])) + ((s.db[43][9] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][9] - s.db[53][9])))))))), ((((((s.db[39][10] * s.v[76]) + (s.v[39] * s.db[76][10])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][10])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][10] * s.v[5]) + (s.v[70] * s.db[5][10])) + ((s.db[43][10] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][10] - s.db[53][10])))))))), ((((((s.db[39][11] * s.v[76]) + (s.v[39] * s.db[76][11])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][11])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][11] * s.v[5]) + (s.v[70] * s.db[5][11])) + ((s.db[43][11] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][11] - s.db[53][11])))))))), ((((((s.db[39][12] * s.v[76]) + (s.v[39] * s.db[76][12])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][12])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][12] * s.v[5]) + (s.v[70] * s.db[5][12])) + ((s.db[43][12] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][12] - s.db[53][12])))))))), ((((((s.db[39][13] * s.v[76]) + (s.v[39] * s.db[76][13])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][13])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][13] * s.v[5]) + (s.v[70] * s.db[5][13])) + ((s.db[43][13] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][13] - s.db[53][13])))))))), ((((((s.db[39][14] * s.v[76]) + (s.v[39] * s.db[76][14])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][14])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][14] * s.v[5]) + (s.v[70] * s.db[5][14])) + ((s.db[43][14] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][14] - s.db[53][14])))))))), ((((((s.db[39][15] * s.v[76]) + (s.v[39] * s.db[76][15])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][15])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][15] * s.v[5]) + (s.v[70] * s.db[5][15])) + ((s.db[43][15] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][15] - s.db[53][15])))))))), ((((((s.db[39][16] * s.v[76]) + (s.v[39] * s.db[76][16])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][16])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][16] * s.v[5]) + (s.v[70] * s.db[5][16])) + ((s.db[43][16] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][16] - s.db[53][16])))))))), ((((((s.db[39][17] * s.v[76]) + (s.v[39] * s.db[76][17])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][17])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][17] * s.v[5]) + (s.v[70] * s.db[5][17])) + ((s.db[43][17] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][17] - s.db[53][17])))))))), ((((((s.db[39][18] * s.v[76]) + (s.v[39] * s.db[76][18])) * assign1120_e1575) + (assign1120_e1572 * s.db[81][18])) * assign1120_e1590) + (assign1120_e1576 * (((s.db[70][18] * s.v[5]) + (s.v[70] * s.db[5][18])) + ((s.db[43][18] * assign1120_e1588) + (s.v[43] * ({ let limexp_arg = assign1120_e1587; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][18] - s.db[53][18])))))))),)
    } else {
        (s.v[73], s.dn[73][0], s.dn[73][1], s.dn[73][2], s.dn[73][3], s.dn[73][4], s.dn[73][5], s.dn[73][6], s.dn[73][7], s.dn[73][8], s.dn[73][9], s.dn[73][10], s.dn[73][11], s.dn[73][12], s.dn[73][13], s.dn[73][14], s.dn[73][15], s.dn[73][16], s.dn[73][17], s.dn[73][18], s.db[73][0], s.db[73][1], s.db[73][2], s.db[73][3], s.db[73][4], s.db[73][5], s.db[73][6], s.db[73][7], s.db[73][8], s.db[73][9], s.db[73][10], s.db[73][11], s.db[73][12], s.db[73][13], s.db[73][14], s.db[73][15], s.db[73][16], s.db[73][17], s.db[73][18],)
    }
};
        s.v[73] = assign1120_e1593;
        s.mark_derivatives_dirty(73);
        s.dn[73][0] = assign1120_e1593_d_n0;
        s.dn[73][1] = assign1120_e1593_d_n1;
        s.dn[73][2] = assign1120_e1593_d_n2;
        s.dn[73][3] = assign1120_e1593_d_n3;
        s.dn[73][4] = assign1120_e1593_d_n4;
        s.dn[73][5] = assign1120_e1593_d_n5;
        s.dn[73][6] = assign1120_e1593_d_n6;
        s.dn[73][7] = assign1120_e1593_d_n7;
        s.dn[73][8] = assign1120_e1593_d_n8;
        s.dn[73][9] = assign1120_e1593_d_n9;
        s.dn[73][10] = assign1120_e1593_d_n10;
        s.dn[73][11] = assign1120_e1593_d_n11;
        s.dn[73][12] = assign1120_e1593_d_n12;
        s.dn[73][13] = assign1120_e1593_d_n13;
        s.dn[73][14] = assign1120_e1593_d_n14;
        s.dn[73][15] = assign1120_e1593_d_n15;
        s.dn[73][16] = assign1120_e1593_d_n16;
        s.dn[73][17] = assign1120_e1593_d_n17;
        s.dn[73][18] = assign1120_e1593_d_n18;
        s.db[73][0] = assign1120_e1593_d_b0;
        s.db[73][1] = assign1120_e1593_d_b1;
        s.db[73][2] = assign1120_e1593_d_b2;
        s.db[73][3] = assign1120_e1593_d_b3;
        s.db[73][4] = assign1120_e1593_d_b4;
        s.db[73][5] = assign1120_e1593_d_b5;
        s.db[73][6] = assign1120_e1593_d_b6;
        s.db[73][7] = assign1120_e1593_d_b7;
        s.db[73][8] = assign1120_e1593_d_b8;
        s.db[73][9] = assign1120_e1593_d_b9;
        s.db[73][10] = assign1120_e1593_d_b10;
        s.db[73][11] = assign1120_e1593_d_b11;
        s.db[73][12] = assign1120_e1593_d_b12;
        s.db[73][13] = assign1120_e1593_d_b13;
        s.db[73][14] = assign1120_e1593_d_b14;
        s.db[73][15] = assign1120_e1593_d_b15;
        s.db[73][16] = assign1120_e1593_d_b16;
        s.db[73][17] = assign1120_e1593_d_b17;
        s.db[73][18] = assign1120_e1593_d_b18;
        s.rv[73] = 0.0;

        let (assign1130_e1616, assign1130_e1616_d_n0, assign1130_e1616_d_n1, assign1130_e1616_d_n2, assign1130_e1616_d_n3, assign1130_e1616_d_n4, assign1130_e1616_d_n5, assign1130_e1616_d_n6, assign1130_e1616_d_n7, assign1130_e1616_d_n8, assign1130_e1616_d_n9, assign1130_e1616_d_n10, assign1130_e1616_d_n11, assign1130_e1616_d_n12, assign1130_e1616_d_n13, assign1130_e1616_d_n14, assign1130_e1616_d_n15, assign1130_e1616_d_n16, assign1130_e1616_d_n17, assign1130_e1616_d_n18, assign1130_e1616_d_b0, assign1130_e1616_d_b1, assign1130_e1616_d_b2, assign1130_e1616_d_b3, assign1130_e1616_d_b4, assign1130_e1616_d_b5, assign1130_e1616_d_b6, assign1130_e1616_d_b7, assign1130_e1616_d_b8, assign1130_e1616_d_b9, assign1130_e1616_d_b10, assign1130_e1616_d_b11, assign1130_e1616_d_b12, assign1130_e1616_d_b13, assign1130_e1616_d_b14, assign1130_e1616_d_b15, assign1130_e1616_d_b16, assign1130_e1616_d_b17, assign1130_e1616_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1130_e1604: f64 = (s.v[39] * s.v[78]);
        let assign1130_e1607: f64 = (1.0 - s.v[82]);
        let assign1130_e1608: f64 = (assign1130_e1604 * assign1130_e1607);
        let assign1130_e1612: f64 = (s.v[68] * s.v[5]);
        let assign1130_e1613: f64 = (1.0 - assign1130_e1612);
        let assign1130_e1614: f64 = (assign1130_e1608 * assign1130_e1613);
        (assign1130_e1614, ((((((s.dn[39][0] * s.v[78]) + (s.v[39] * s.dn[78][0])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][0]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][0] * s.v[5]) + (s.v[68] * s.dn[5][0]))))), ((((((s.dn[39][1] * s.v[78]) + (s.v[39] * s.dn[78][1])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][1]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][1] * s.v[5]) + (s.v[68] * s.dn[5][1]))))), ((((((s.dn[39][2] * s.v[78]) + (s.v[39] * s.dn[78][2])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][2]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][2] * s.v[5]) + (s.v[68] * s.dn[5][2]))))), ((((((s.dn[39][3] * s.v[78]) + (s.v[39] * s.dn[78][3])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][3]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][3] * s.v[5]) + (s.v[68] * s.dn[5][3]))))), ((((((s.dn[39][4] * s.v[78]) + (s.v[39] * s.dn[78][4])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][4]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][4] * s.v[5]) + (s.v[68] * s.dn[5][4]))))), ((((((s.dn[39][5] * s.v[78]) + (s.v[39] * s.dn[78][5])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][5]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][5] * s.v[5]) + (s.v[68] * s.dn[5][5]))))), ((((((s.dn[39][6] * s.v[78]) + (s.v[39] * s.dn[78][6])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][6]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][6] * s.v[5]) + (s.v[68] * s.dn[5][6]))))), ((((((s.dn[39][7] * s.v[78]) + (s.v[39] * s.dn[78][7])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][7]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][7] * s.v[5]) + (s.v[68] * s.dn[5][7]))))), ((((((s.dn[39][8] * s.v[78]) + (s.v[39] * s.dn[78][8])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][8]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][8] * s.v[5]) + (s.v[68] * s.dn[5][8]))))), ((((((s.dn[39][9] * s.v[78]) + (s.v[39] * s.dn[78][9])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][9]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][9] * s.v[5]) + (s.v[68] * s.dn[5][9]))))), ((((((s.dn[39][10] * s.v[78]) + (s.v[39] * s.dn[78][10])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][10]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][10] * s.v[5]) + (s.v[68] * s.dn[5][10]))))), ((((((s.dn[39][11] * s.v[78]) + (s.v[39] * s.dn[78][11])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][11]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][11] * s.v[5]) + (s.v[68] * s.dn[5][11]))))), ((((((s.dn[39][12] * s.v[78]) + (s.v[39] * s.dn[78][12])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][12]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][12] * s.v[5]) + (s.v[68] * s.dn[5][12]))))), ((((((s.dn[39][13] * s.v[78]) + (s.v[39] * s.dn[78][13])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][13]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][13] * s.v[5]) + (s.v[68] * s.dn[5][13]))))), ((((((s.dn[39][14] * s.v[78]) + (s.v[39] * s.dn[78][14])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][14]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][14] * s.v[5]) + (s.v[68] * s.dn[5][14]))))), ((((((s.dn[39][15] * s.v[78]) + (s.v[39] * s.dn[78][15])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][15]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][15] * s.v[5]) + (s.v[68] * s.dn[5][15]))))), ((((((s.dn[39][16] * s.v[78]) + (s.v[39] * s.dn[78][16])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][16]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][16] * s.v[5]) + (s.v[68] * s.dn[5][16]))))), ((((((s.dn[39][17] * s.v[78]) + (s.v[39] * s.dn[78][17])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][17]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][17] * s.v[5]) + (s.v[68] * s.dn[5][17]))))), ((((((s.dn[39][18] * s.v[78]) + (s.v[39] * s.dn[78][18])) * assign1130_e1607) + (assign1130_e1604 * (-s.dn[82][18]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.dn[68][18] * s.v[5]) + (s.v[68] * s.dn[5][18]))))), ((((((s.db[39][0] * s.v[78]) + (s.v[39] * s.db[78][0])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][0]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][0] * s.v[5]) + (s.v[68] * s.db[5][0]))))), ((((((s.db[39][1] * s.v[78]) + (s.v[39] * s.db[78][1])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][1]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][1] * s.v[5]) + (s.v[68] * s.db[5][1]))))), ((((((s.db[39][2] * s.v[78]) + (s.v[39] * s.db[78][2])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][2]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][2] * s.v[5]) + (s.v[68] * s.db[5][2]))))), ((((((s.db[39][3] * s.v[78]) + (s.v[39] * s.db[78][3])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][3]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][3] * s.v[5]) + (s.v[68] * s.db[5][3]))))), ((((((s.db[39][4] * s.v[78]) + (s.v[39] * s.db[78][4])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][4]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][4] * s.v[5]) + (s.v[68] * s.db[5][4]))))), ((((((s.db[39][5] * s.v[78]) + (s.v[39] * s.db[78][5])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][5]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][5] * s.v[5]) + (s.v[68] * s.db[5][5]))))), ((((((s.db[39][6] * s.v[78]) + (s.v[39] * s.db[78][6])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][6]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][6] * s.v[5]) + (s.v[68] * s.db[5][6]))))), ((((((s.db[39][7] * s.v[78]) + (s.v[39] * s.db[78][7])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][7]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][7] * s.v[5]) + (s.v[68] * s.db[5][7]))))), ((((((s.db[39][8] * s.v[78]) + (s.v[39] * s.db[78][8])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][8]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][8] * s.v[5]) + (s.v[68] * s.db[5][8]))))), ((((((s.db[39][9] * s.v[78]) + (s.v[39] * s.db[78][9])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][9]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][9] * s.v[5]) + (s.v[68] * s.db[5][9]))))), ((((((s.db[39][10] * s.v[78]) + (s.v[39] * s.db[78][10])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][10]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][10] * s.v[5]) + (s.v[68] * s.db[5][10]))))), ((((((s.db[39][11] * s.v[78]) + (s.v[39] * s.db[78][11])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][11]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][11] * s.v[5]) + (s.v[68] * s.db[5][11]))))), ((((((s.db[39][12] * s.v[78]) + (s.v[39] * s.db[78][12])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][12]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][12] * s.v[5]) + (s.v[68] * s.db[5][12]))))), ((((((s.db[39][13] * s.v[78]) + (s.v[39] * s.db[78][13])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][13]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][13] * s.v[5]) + (s.v[68] * s.db[5][13]))))), ((((((s.db[39][14] * s.v[78]) + (s.v[39] * s.db[78][14])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][14]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][14] * s.v[5]) + (s.v[68] * s.db[5][14]))))), ((((((s.db[39][15] * s.v[78]) + (s.v[39] * s.db[78][15])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][15]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][15] * s.v[5]) + (s.v[68] * s.db[5][15]))))), ((((((s.db[39][16] * s.v[78]) + (s.v[39] * s.db[78][16])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][16]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][16] * s.v[5]) + (s.v[68] * s.db[5][16]))))), ((((((s.db[39][17] * s.v[78]) + (s.v[39] * s.db[78][17])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][17]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][17] * s.v[5]) + (s.v[68] * s.db[5][17]))))), ((((((s.db[39][18] * s.v[78]) + (s.v[39] * s.db[78][18])) * assign1130_e1607) + (assign1130_e1604 * (-s.db[82][18]))) * assign1130_e1613) + (assign1130_e1608 * (-((s.db[68][18] * s.v[5]) + (s.v[68] * s.db[5][18]))))),)
    } else {
        (s.v[74], s.dn[74][0], s.dn[74][1], s.dn[74][2], s.dn[74][3], s.dn[74][4], s.dn[74][5], s.dn[74][6], s.dn[74][7], s.dn[74][8], s.dn[74][9], s.dn[74][10], s.dn[74][11], s.dn[74][12], s.dn[74][13], s.dn[74][14], s.dn[74][15], s.dn[74][16], s.dn[74][17], s.dn[74][18], s.db[74][0], s.db[74][1], s.db[74][2], s.db[74][3], s.db[74][4], s.db[74][5], s.db[74][6], s.db[74][7], s.db[74][8], s.db[74][9], s.db[74][10], s.db[74][11], s.db[74][12], s.db[74][13], s.db[74][14], s.db[74][15], s.db[74][16], s.db[74][17], s.db[74][18],)
    }
};
        s.v[74] = assign1130_e1616;
        s.mark_derivatives_dirty(74);
        s.dn[74][0] = assign1130_e1616_d_n0;
        s.dn[74][1] = assign1130_e1616_d_n1;
        s.dn[74][2] = assign1130_e1616_d_n2;
        s.dn[74][3] = assign1130_e1616_d_n3;
        s.dn[74][4] = assign1130_e1616_d_n4;
        s.dn[74][5] = assign1130_e1616_d_n5;
        s.dn[74][6] = assign1130_e1616_d_n6;
        s.dn[74][7] = assign1130_e1616_d_n7;
        s.dn[74][8] = assign1130_e1616_d_n8;
        s.dn[74][9] = assign1130_e1616_d_n9;
        s.dn[74][10] = assign1130_e1616_d_n10;
        s.dn[74][11] = assign1130_e1616_d_n11;
        s.dn[74][12] = assign1130_e1616_d_n12;
        s.dn[74][13] = assign1130_e1616_d_n13;
        s.dn[74][14] = assign1130_e1616_d_n14;
        s.dn[74][15] = assign1130_e1616_d_n15;
        s.dn[74][16] = assign1130_e1616_d_n16;
        s.dn[74][17] = assign1130_e1616_d_n17;
        s.dn[74][18] = assign1130_e1616_d_n18;
        s.db[74][0] = assign1130_e1616_d_b0;
        s.db[74][1] = assign1130_e1616_d_b1;
        s.db[74][2] = assign1130_e1616_d_b2;
        s.db[74][3] = assign1130_e1616_d_b3;
        s.db[74][4] = assign1130_e1616_d_b4;
        s.db[74][5] = assign1130_e1616_d_b5;
        s.db[74][6] = assign1130_e1616_d_b6;
        s.db[74][7] = assign1130_e1616_d_b7;
        s.db[74][8] = assign1130_e1616_d_b8;
        s.db[74][9] = assign1130_e1616_d_b9;
        s.db[74][10] = assign1130_e1616_d_b10;
        s.db[74][11] = assign1130_e1616_d_b11;
        s.db[74][12] = assign1130_e1616_d_b12;
        s.db[74][13] = assign1130_e1616_d_b13;
        s.db[74][14] = assign1130_e1616_d_b14;
        s.db[74][15] = assign1130_e1616_d_b15;
        s.db[74][16] = assign1130_e1616_d_b16;
        s.db[74][17] = assign1130_e1616_d_b17;
        s.db[74][18] = assign1130_e1616_d_b18;
        s.rv[74] = 0.0;

        let (assign1140_e1631, assign1140_e1631_d_n0, assign1140_e1631_d_n1, assign1140_e1631_d_n2, assign1140_e1631_d_n3, assign1140_e1631_d_n4, assign1140_e1631_d_n5, assign1140_e1631_d_n6, assign1140_e1631_d_n7, assign1140_e1631_d_n8, assign1140_e1631_d_n9, assign1140_e1631_d_n10, assign1140_e1631_d_n11, assign1140_e1631_d_n12, assign1140_e1631_d_n13, assign1140_e1631_d_n14, assign1140_e1631_d_n15, assign1140_e1631_d_n16, assign1140_e1631_d_n17, assign1140_e1631_d_n18, assign1140_e1631_d_b0, assign1140_e1631_d_b1, assign1140_e1631_d_b2, assign1140_e1631_d_b3, assign1140_e1631_d_b4, assign1140_e1631_d_b5, assign1140_e1631_d_b6, assign1140_e1631_d_b7, assign1140_e1631_d_b8, assign1140_e1631_d_b9, assign1140_e1631_d_b10, assign1140_e1631_d_b11, assign1140_e1631_d_b12, assign1140_e1631_d_b13, assign1140_e1631_d_b14, assign1140_e1631_d_b15, assign1140_e1631_d_b16, assign1140_e1631_d_b17, assign1140_e1631_d_b18,) = {
    if (s.b[109] && (!((s.b[106] || s.b[107]) || s.b[108]))) {
        let assign1140_e1628: f64 = (s.v[73] - s.v[74]);
        let assign1140_e1629: f64 = (0.5 * assign1140_e1628);
        (assign1140_e1629, (0.5 * (s.dn[73][0] - s.dn[74][0])), (0.5 * (s.dn[73][1] - s.dn[74][1])), (0.5 * (s.dn[73][2] - s.dn[74][2])), (0.5 * (s.dn[73][3] - s.dn[74][3])), (0.5 * (s.dn[73][4] - s.dn[74][4])), (0.5 * (s.dn[73][5] - s.dn[74][5])), (0.5 * (s.dn[73][6] - s.dn[74][6])), (0.5 * (s.dn[73][7] - s.dn[74][7])), (0.5 * (s.dn[73][8] - s.dn[74][8])), (0.5 * (s.dn[73][9] - s.dn[74][9])), (0.5 * (s.dn[73][10] - s.dn[74][10])), (0.5 * (s.dn[73][11] - s.dn[74][11])), (0.5 * (s.dn[73][12] - s.dn[74][12])), (0.5 * (s.dn[73][13] - s.dn[74][13])), (0.5 * (s.dn[73][14] - s.dn[74][14])), (0.5 * (s.dn[73][15] - s.dn[74][15])), (0.5 * (s.dn[73][16] - s.dn[74][16])), (0.5 * (s.dn[73][17] - s.dn[74][17])), (0.5 * (s.dn[73][18] - s.dn[74][18])), (0.5 * (s.db[73][0] - s.db[74][0])), (0.5 * (s.db[73][1] - s.db[74][1])), (0.5 * (s.db[73][2] - s.db[74][2])), (0.5 * (s.db[73][3] - s.db[74][3])), (0.5 * (s.db[73][4] - s.db[74][4])), (0.5 * (s.db[73][5] - s.db[74][5])), (0.5 * (s.db[73][6] - s.db[74][6])), (0.5 * (s.db[73][7] - s.db[74][7])), (0.5 * (s.db[73][8] - s.db[74][8])), (0.5 * (s.db[73][9] - s.db[74][9])), (0.5 * (s.db[73][10] - s.db[74][10])), (0.5 * (s.db[73][11] - s.db[74][11])), (0.5 * (s.db[73][12] - s.db[74][12])), (0.5 * (s.db[73][13] - s.db[74][13])), (0.5 * (s.db[73][14] - s.db[74][14])), (0.5 * (s.db[73][15] - s.db[74][15])), (0.5 * (s.db[73][16] - s.db[74][16])), (0.5 * (s.db[73][17] - s.db[74][17])), (0.5 * (s.db[73][18] - s.db[74][18])),)
    } else {
        (s.v[98], s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5], s.dn[98][6], s.dn[98][7], s.dn[98][8], s.dn[98][9], s.dn[98][10], s.dn[98][11], s.dn[98][12], s.dn[98][13], s.dn[98][14], s.dn[98][15], s.dn[98][16], s.dn[98][17], s.dn[98][18], s.db[98][0], s.db[98][1], s.db[98][2], s.db[98][3], s.db[98][4], s.db[98][5], s.db[98][6], s.db[98][7], s.db[98][8], s.db[98][9], s.db[98][10], s.db[98][11], s.db[98][12], s.db[98][13], s.db[98][14], s.db[98][15], s.db[98][16], s.db[98][17], s.db[98][18],)
    }
};
        s.v[98] = assign1140_e1631;
        s.mark_derivatives_dirty(98);
        s.dn[98][0] = assign1140_e1631_d_n0;
        s.dn[98][1] = assign1140_e1631_d_n1;
        s.dn[98][2] = assign1140_e1631_d_n2;
        s.dn[98][3] = assign1140_e1631_d_n3;
        s.dn[98][4] = assign1140_e1631_d_n4;
        s.dn[98][5] = assign1140_e1631_d_n5;
        s.dn[98][6] = assign1140_e1631_d_n6;
        s.dn[98][7] = assign1140_e1631_d_n7;
        s.dn[98][8] = assign1140_e1631_d_n8;
        s.dn[98][9] = assign1140_e1631_d_n9;
        s.dn[98][10] = assign1140_e1631_d_n10;
        s.dn[98][11] = assign1140_e1631_d_n11;
        s.dn[98][12] = assign1140_e1631_d_n12;
        s.dn[98][13] = assign1140_e1631_d_n13;
        s.dn[98][14] = assign1140_e1631_d_n14;
        s.dn[98][15] = assign1140_e1631_d_n15;
        s.dn[98][16] = assign1140_e1631_d_n16;
        s.dn[98][17] = assign1140_e1631_d_n17;
        s.dn[98][18] = assign1140_e1631_d_n18;
        s.db[98][0] = assign1140_e1631_d_b0;
        s.db[98][1] = assign1140_e1631_d_b1;
        s.db[98][2] = assign1140_e1631_d_b2;
        s.db[98][3] = assign1140_e1631_d_b3;
        s.db[98][4] = assign1140_e1631_d_b4;
        s.db[98][5] = assign1140_e1631_d_b5;
        s.db[98][6] = assign1140_e1631_d_b6;
        s.db[98][7] = assign1140_e1631_d_b7;
        s.db[98][8] = assign1140_e1631_d_b8;
        s.db[98][9] = assign1140_e1631_d_b9;
        s.db[98][10] = assign1140_e1631_d_b10;
        s.db[98][11] = assign1140_e1631_d_b11;
        s.db[98][12] = assign1140_e1631_d_b12;
        s.db[98][13] = assign1140_e1631_d_b13;
        s.db[98][14] = assign1140_e1631_d_b14;
        s.db[98][15] = assign1140_e1631_d_b15;
        s.db[98][16] = assign1140_e1631_d_b16;
        s.db[98][17] = assign1140_e1631_d_b17;
        s.db[98][18] = assign1140_e1631_d_b18;
        s.rv[98] = 0.0;

        let (assign1150_e1648, assign1150_e1648_d_n0, assign1150_e1648_d_n1, assign1150_e1648_d_n2, assign1150_e1648_d_n3, assign1150_e1648_d_n4, assign1150_e1648_d_n5, assign1150_e1648_d_n6, assign1150_e1648_d_n7, assign1150_e1648_d_n8, assign1150_e1648_d_n9, assign1150_e1648_d_n10, assign1150_e1648_d_n11, assign1150_e1648_d_n12, assign1150_e1648_d_n13, assign1150_e1648_d_n14, assign1150_e1648_d_n15, assign1150_e1648_d_n16, assign1150_e1648_d_n17, assign1150_e1648_d_n18, assign1150_e1648_d_b0, assign1150_e1648_d_b1, assign1150_e1648_d_b2, assign1150_e1648_d_b3, assign1150_e1648_d_b4, assign1150_e1648_d_b5, assign1150_e1648_d_b6, assign1150_e1648_d_b7, assign1150_e1648_d_b8, assign1150_e1648_d_b9, assign1150_e1648_d_b10, assign1150_e1648_d_b11, assign1150_e1648_d_b12, assign1150_e1648_d_b13, assign1150_e1648_d_b14, assign1150_e1648_d_b15, assign1150_e1648_d_b16, assign1150_e1648_d_b17, assign1150_e1648_d_b18,) = {
    if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
        let assign1150_e1645: f64 = (p.p17 * s.v[75]);
        let assign1150_e1646: f64 = (p.p16 + assign1150_e1645);
        (assign1150_e1646, (p.p17 * s.dn[75][0]), (p.p17 * s.dn[75][1]), (p.p17 * s.dn[75][2]), (p.p17 * s.dn[75][3]), (p.p17 * s.dn[75][4]), (p.p17 * s.dn[75][5]), (p.p17 * s.dn[75][6]), (p.p17 * s.dn[75][7]), (p.p17 * s.dn[75][8]), (p.p17 * s.dn[75][9]), (p.p17 * s.dn[75][10]), (p.p17 * s.dn[75][11]), (p.p17 * s.dn[75][12]), (p.p17 * s.dn[75][13]), (p.p17 * s.dn[75][14]), (p.p17 * s.dn[75][15]), (p.p17 * s.dn[75][16]), (p.p17 * s.dn[75][17]), (p.p17 * s.dn[75][18]), (p.p17 * s.db[75][0]), (p.p17 * s.db[75][1]), (p.p17 * s.db[75][2]), (p.p17 * s.db[75][3]), (p.p17 * s.db[75][4]), (p.p17 * s.db[75][5]), (p.p17 * s.db[75][6]), (p.p17 * s.db[75][7]), (p.p17 * s.db[75][8]), (p.p17 * s.db[75][9]), (p.p17 * s.db[75][10]), (p.p17 * s.db[75][11]), (p.p17 * s.db[75][12]), (p.p17 * s.db[75][13]), (p.p17 * s.db[75][14]), (p.p17 * s.db[75][15]), (p.p17 * s.db[75][16]), (p.p17 * s.db[75][17]), (p.p17 * s.db[75][18]),)
    } else {
        (s.v[69], s.dn[69][0], s.dn[69][1], s.dn[69][2], s.dn[69][3], s.dn[69][4], s.dn[69][5], s.dn[69][6], s.dn[69][7], s.dn[69][8], s.dn[69][9], s.dn[69][10], s.dn[69][11], s.dn[69][12], s.dn[69][13], s.dn[69][14], s.dn[69][15], s.dn[69][16], s.dn[69][17], s.dn[69][18], s.db[69][0], s.db[69][1], s.db[69][2], s.db[69][3], s.db[69][4], s.db[69][5], s.db[69][6], s.db[69][7], s.db[69][8], s.db[69][9], s.db[69][10], s.db[69][11], s.db[69][12], s.db[69][13], s.db[69][14], s.db[69][15], s.db[69][16], s.db[69][17], s.db[69][18],)
    }
};
        s.v[69] = assign1150_e1648;
        s.mark_derivatives_dirty(69);
        s.dn[69][0] = assign1150_e1648_d_n0;
        s.dn[69][1] = assign1150_e1648_d_n1;
        s.dn[69][2] = assign1150_e1648_d_n2;
        s.dn[69][3] = assign1150_e1648_d_n3;
        s.dn[69][4] = assign1150_e1648_d_n4;
        s.dn[69][5] = assign1150_e1648_d_n5;
        s.dn[69][6] = assign1150_e1648_d_n6;
        s.dn[69][7] = assign1150_e1648_d_n7;
        s.dn[69][8] = assign1150_e1648_d_n8;
        s.dn[69][9] = assign1150_e1648_d_n9;
        s.dn[69][10] = assign1150_e1648_d_n10;
        s.dn[69][11] = assign1150_e1648_d_n11;
        s.dn[69][12] = assign1150_e1648_d_n12;
        s.dn[69][13] = assign1150_e1648_d_n13;
        s.dn[69][14] = assign1150_e1648_d_n14;
        s.dn[69][15] = assign1150_e1648_d_n15;
        s.dn[69][16] = assign1150_e1648_d_n16;
        s.dn[69][17] = assign1150_e1648_d_n17;
        s.dn[69][18] = assign1150_e1648_d_n18;
        s.db[69][0] = assign1150_e1648_d_b0;
        s.db[69][1] = assign1150_e1648_d_b1;
        s.db[69][2] = assign1150_e1648_d_b2;
        s.db[69][3] = assign1150_e1648_d_b3;
        s.db[69][4] = assign1150_e1648_d_b4;
        s.db[69][5] = assign1150_e1648_d_b5;
        s.db[69][6] = assign1150_e1648_d_b6;
        s.db[69][7] = assign1150_e1648_d_b7;
        s.db[69][8] = assign1150_e1648_d_b8;
        s.db[69][9] = assign1150_e1648_d_b9;
        s.db[69][10] = assign1150_e1648_d_b10;
        s.db[69][11] = assign1150_e1648_d_b11;
        s.db[69][12] = assign1150_e1648_d_b12;
        s.db[69][13] = assign1150_e1648_d_b13;
        s.db[69][14] = assign1150_e1648_d_b14;
        s.db[69][15] = assign1150_e1648_d_b15;
        s.db[69][16] = assign1150_e1648_d_b16;
        s.db[69][17] = assign1150_e1648_d_b17;
        s.db[69][18] = assign1150_e1648_d_b18;
        s.rv[69] = 0.0;

        let (assign1160_e1665, assign1160_e1665_d_n0, assign1160_e1665_d_n1, assign1160_e1665_d_n2, assign1160_e1665_d_n3, assign1160_e1665_d_n4, assign1160_e1665_d_n5, assign1160_e1665_d_n6, assign1160_e1665_d_n7, assign1160_e1665_d_n8, assign1160_e1665_d_n9, assign1160_e1665_d_n10, assign1160_e1665_d_n11, assign1160_e1665_d_n12, assign1160_e1665_d_n13, assign1160_e1665_d_n14, assign1160_e1665_d_n15, assign1160_e1665_d_n16, assign1160_e1665_d_n17, assign1160_e1665_d_n18, assign1160_e1665_d_b0, assign1160_e1665_d_b1, assign1160_e1665_d_b2, assign1160_e1665_d_b3, assign1160_e1665_d_b4, assign1160_e1665_d_b5, assign1160_e1665_d_b6, assign1160_e1665_d_b7, assign1160_e1665_d_b8, assign1160_e1665_d_b9, assign1160_e1665_d_b10, assign1160_e1665_d_b11, assign1160_e1665_d_b12, assign1160_e1665_d_b13, assign1160_e1665_d_b14, assign1160_e1665_d_b15, assign1160_e1665_d_b16, assign1160_e1665_d_b17, assign1160_e1665_d_b18,) = {
    if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
        let assign1160_e1662: f64 = (p.p15 * s.v[76]);
        let assign1160_e1663: f64 = (p.p14 + assign1160_e1662);
        (assign1160_e1663, (p.p15 * s.dn[76][0]), (p.p15 * s.dn[76][1]), (p.p15 * s.dn[76][2]), (p.p15 * s.dn[76][3]), (p.p15 * s.dn[76][4]), (p.p15 * s.dn[76][5]), (p.p15 * s.dn[76][6]), (p.p15 * s.dn[76][7]), (p.p15 * s.dn[76][8]), (p.p15 * s.dn[76][9]), (p.p15 * s.dn[76][10]), (p.p15 * s.dn[76][11]), (p.p15 * s.dn[76][12]), (p.p15 * s.dn[76][13]), (p.p15 * s.dn[76][14]), (p.p15 * s.dn[76][15]), (p.p15 * s.dn[76][16]), (p.p15 * s.dn[76][17]), (p.p15 * s.dn[76][18]), (p.p15 * s.db[76][0]), (p.p15 * s.db[76][1]), (p.p15 * s.db[76][2]), (p.p15 * s.db[76][3]), (p.p15 * s.db[76][4]), (p.p15 * s.db[76][5]), (p.p15 * s.db[76][6]), (p.p15 * s.db[76][7]), (p.p15 * s.db[76][8]), (p.p15 * s.db[76][9]), (p.p15 * s.db[76][10]), (p.p15 * s.db[76][11]), (p.p15 * s.db[76][12]), (p.p15 * s.db[76][13]), (p.p15 * s.db[76][14]), (p.p15 * s.db[76][15]), (p.p15 * s.db[76][16]), (p.p15 * s.db[76][17]), (p.p15 * s.db[76][18]),)
    } else {
        (s.v[1], s.dn[1][0], s.dn[1][1], s.dn[1][2], s.dn[1][3], s.dn[1][4], s.dn[1][5], s.dn[1][6], s.dn[1][7], s.dn[1][8], s.dn[1][9], s.dn[1][10], s.dn[1][11], s.dn[1][12], s.dn[1][13], s.dn[1][14], s.dn[1][15], s.dn[1][16], s.dn[1][17], s.dn[1][18], s.db[1][0], s.db[1][1], s.db[1][2], s.db[1][3], s.db[1][4], s.db[1][5], s.db[1][6], s.db[1][7], s.db[1][8], s.db[1][9], s.db[1][10], s.db[1][11], s.db[1][12], s.db[1][13], s.db[1][14], s.db[1][15], s.db[1][16], s.db[1][17], s.db[1][18],)
    }
};
        s.v[1] = assign1160_e1665;
        s.mark_derivatives_dirty(1);
        s.dn[1][0] = assign1160_e1665_d_n0;
        s.dn[1][1] = assign1160_e1665_d_n1;
        s.dn[1][2] = assign1160_e1665_d_n2;
        s.dn[1][3] = assign1160_e1665_d_n3;
        s.dn[1][4] = assign1160_e1665_d_n4;
        s.dn[1][5] = assign1160_e1665_d_n5;
        s.dn[1][6] = assign1160_e1665_d_n6;
        s.dn[1][7] = assign1160_e1665_d_n7;
        s.dn[1][8] = assign1160_e1665_d_n8;
        s.dn[1][9] = assign1160_e1665_d_n9;
        s.dn[1][10] = assign1160_e1665_d_n10;
        s.dn[1][11] = assign1160_e1665_d_n11;
        s.dn[1][12] = assign1160_e1665_d_n12;
        s.dn[1][13] = assign1160_e1665_d_n13;
        s.dn[1][14] = assign1160_e1665_d_n14;
        s.dn[1][15] = assign1160_e1665_d_n15;
        s.dn[1][16] = assign1160_e1665_d_n16;
        s.dn[1][17] = assign1160_e1665_d_n17;
        s.dn[1][18] = assign1160_e1665_d_n18;
        s.db[1][0] = assign1160_e1665_d_b0;
        s.db[1][1] = assign1160_e1665_d_b1;
        s.db[1][2] = assign1160_e1665_d_b2;
        s.db[1][3] = assign1160_e1665_d_b3;
        s.db[1][4] = assign1160_e1665_d_b4;
        s.db[1][5] = assign1160_e1665_d_b5;
        s.db[1][6] = assign1160_e1665_d_b6;
        s.db[1][7] = assign1160_e1665_d_b7;
        s.db[1][8] = assign1160_e1665_d_b8;
        s.db[1][9] = assign1160_e1665_d_b9;
        s.db[1][10] = assign1160_e1665_d_b10;
        s.db[1][11] = assign1160_e1665_d_b11;
        s.db[1][12] = assign1160_e1665_d_b12;
        s.db[1][13] = assign1160_e1665_d_b13;
        s.db[1][14] = assign1160_e1665_d_b14;
        s.db[1][15] = assign1160_e1665_d_b15;
        s.db[1][16] = assign1160_e1665_d_b16;
        s.db[1][17] = assign1160_e1665_d_b17;
        s.db[1][18] = assign1160_e1665_d_b18;
        s.rv[1] = 0.0;

        let (assign1170_e1681, assign1170_e1681_d_n0, assign1170_e1681_d_n1, assign1170_e1681_d_n2, assign1170_e1681_d_n3, assign1170_e1681_d_n4, assign1170_e1681_d_n5, assign1170_e1681_d_n6, assign1170_e1681_d_n7, assign1170_e1681_d_n8, assign1170_e1681_d_n9, assign1170_e1681_d_n10, assign1170_e1681_d_n11, assign1170_e1681_d_n12, assign1170_e1681_d_n13, assign1170_e1681_d_n14, assign1170_e1681_d_n15, assign1170_e1681_d_n16, assign1170_e1681_d_n17, assign1170_e1681_d_n18, assign1170_e1681_d_b0, assign1170_e1681_d_b1, assign1170_e1681_d_b2, assign1170_e1681_d_b3, assign1170_e1681_d_b4, assign1170_e1681_d_b5, assign1170_e1681_d_b6, assign1170_e1681_d_b7, assign1170_e1681_d_b8, assign1170_e1681_d_b9, assign1170_e1681_d_b10, assign1170_e1681_d_b11, assign1170_e1681_d_b12, assign1170_e1681_d_b13, assign1170_e1681_d_b14, assign1170_e1681_d_b15, assign1170_e1681_d_b16, assign1170_e1681_d_b17, assign1170_e1681_d_b18,) = {
    if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
        let assign1170_e1678: f64 = (s.v[1] * s.v[5]);
        let assign1170_e1679: f64 = (assign1170_e1678).tanh();
        (assign1170_e1679, (((s.dn[1][0] * s.v[5]) + (s.v[1] * s.dn[5][0])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][1] * s.v[5]) + (s.v[1] * s.dn[5][1])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][2] * s.v[5]) + (s.v[1] * s.dn[5][2])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][3] * s.v[5]) + (s.v[1] * s.dn[5][3])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][4] * s.v[5]) + (s.v[1] * s.dn[5][4])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][5] * s.v[5]) + (s.v[1] * s.dn[5][5])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][6] * s.v[5]) + (s.v[1] * s.dn[5][6])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][7] * s.v[5]) + (s.v[1] * s.dn[5][7])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][8] * s.v[5]) + (s.v[1] * s.dn[5][8])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][9] * s.v[5]) + (s.v[1] * s.dn[5][9])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][10] * s.v[5]) + (s.v[1] * s.dn[5][10])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][11] * s.v[5]) + (s.v[1] * s.dn[5][11])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][12] * s.v[5]) + (s.v[1] * s.dn[5][12])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][13] * s.v[5]) + (s.v[1] * s.dn[5][13])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][14] * s.v[5]) + (s.v[1] * s.dn[5][14])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][15] * s.v[5]) + (s.v[1] * s.dn[5][15])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][16] * s.v[5]) + (s.v[1] * s.dn[5][16])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][17] * s.v[5]) + (s.v[1] * s.dn[5][17])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.dn[1][18] * s.v[5]) + (s.v[1] * s.dn[5][18])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][0] * s.v[5]) + (s.v[1] * s.db[5][0])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][1] * s.v[5]) + (s.v[1] * s.db[5][1])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][2] * s.v[5]) + (s.v[1] * s.db[5][2])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][3] * s.v[5]) + (s.v[1] * s.db[5][3])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][4] * s.v[5]) + (s.v[1] * s.db[5][4])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][5] * s.v[5]) + (s.v[1] * s.db[5][5])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][6] * s.v[5]) + (s.v[1] * s.db[5][6])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][7] * s.v[5]) + (s.v[1] * s.db[5][7])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][8] * s.v[5]) + (s.v[1] * s.db[5][8])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][9] * s.v[5]) + (s.v[1] * s.db[5][9])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][10] * s.v[5]) + (s.v[1] * s.db[5][10])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][11] * s.v[5]) + (s.v[1] * s.db[5][11])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][12] * s.v[5]) + (s.v[1] * s.db[5][12])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][13] * s.v[5]) + (s.v[1] * s.db[5][13])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][14] * s.v[5]) + (s.v[1] * s.db[5][14])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][15] * s.v[5]) + (s.v[1] * s.db[5][15])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][16] * s.v[5]) + (s.v[1] * s.db[5][16])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][17] * s.v[5]) + (s.v[1] * s.db[5][17])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())), (((s.db[1][18] * s.v[5]) + (s.v[1] * s.db[5][18])) / ((assign1170_e1678).cosh() * (assign1170_e1678).cosh())),)
    } else {
        (s.v[81], s.dn[81][0], s.dn[81][1], s.dn[81][2], s.dn[81][3], s.dn[81][4], s.dn[81][5], s.dn[81][6], s.dn[81][7], s.dn[81][8], s.dn[81][9], s.dn[81][10], s.dn[81][11], s.dn[81][12], s.dn[81][13], s.dn[81][14], s.dn[81][15], s.dn[81][16], s.dn[81][17], s.dn[81][18], s.db[81][0], s.db[81][1], s.db[81][2], s.db[81][3], s.db[81][4], s.db[81][5], s.db[81][6], s.db[81][7], s.db[81][8], s.db[81][9], s.db[81][10], s.db[81][11], s.db[81][12], s.db[81][13], s.db[81][14], s.db[81][15], s.db[81][16], s.db[81][17], s.db[81][18],)
    }
};
        s.v[81] = assign1170_e1681;
        s.mark_derivatives_dirty(81);
        s.dn[81][0] = assign1170_e1681_d_n0;
        s.dn[81][1] = assign1170_e1681_d_n1;
        s.dn[81][2] = assign1170_e1681_d_n2;
        s.dn[81][3] = assign1170_e1681_d_n3;
        s.dn[81][4] = assign1170_e1681_d_n4;
        s.dn[81][5] = assign1170_e1681_d_n5;
        s.dn[81][6] = assign1170_e1681_d_n6;
        s.dn[81][7] = assign1170_e1681_d_n7;
        s.dn[81][8] = assign1170_e1681_d_n8;
        s.dn[81][9] = assign1170_e1681_d_n9;
        s.dn[81][10] = assign1170_e1681_d_n10;
        s.dn[81][11] = assign1170_e1681_d_n11;
        s.dn[81][12] = assign1170_e1681_d_n12;
        s.dn[81][13] = assign1170_e1681_d_n13;
        s.dn[81][14] = assign1170_e1681_d_n14;
        s.dn[81][15] = assign1170_e1681_d_n15;
        s.dn[81][16] = assign1170_e1681_d_n16;
        s.dn[81][17] = assign1170_e1681_d_n17;
        s.dn[81][18] = assign1170_e1681_d_n18;
        s.db[81][0] = assign1170_e1681_d_b0;
        s.db[81][1] = assign1170_e1681_d_b1;
        s.db[81][2] = assign1170_e1681_d_b2;
        s.db[81][3] = assign1170_e1681_d_b3;
        s.db[81][4] = assign1170_e1681_d_b4;
        s.db[81][5] = assign1170_e1681_d_b5;
        s.db[81][6] = assign1170_e1681_d_b6;
        s.db[81][7] = assign1170_e1681_d_b7;
        s.db[81][8] = assign1170_e1681_d_b8;
        s.db[81][9] = assign1170_e1681_d_b9;
        s.db[81][10] = assign1170_e1681_d_b10;
        s.db[81][11] = assign1170_e1681_d_b11;
        s.db[81][12] = assign1170_e1681_d_b12;
        s.db[81][13] = assign1170_e1681_d_b13;
        s.db[81][14] = assign1170_e1681_d_b14;
        s.db[81][15] = assign1170_e1681_d_b15;
        s.db[81][16] = assign1170_e1681_d_b16;
        s.db[81][17] = assign1170_e1681_d_b17;
        s.db[81][18] = assign1170_e1681_d_b18;
        s.rv[81] = 0.0;

        let (assign1180_e1697, assign1180_e1697_d_n0, assign1180_e1697_d_n1, assign1180_e1697_d_n2, assign1180_e1697_d_n3, assign1180_e1697_d_n4, assign1180_e1697_d_n5, assign1180_e1697_d_n6, assign1180_e1697_d_n7, assign1180_e1697_d_n8, assign1180_e1697_d_n9, assign1180_e1697_d_n10, assign1180_e1697_d_n11, assign1180_e1697_d_n12, assign1180_e1697_d_n13, assign1180_e1697_d_n14, assign1180_e1697_d_n15, assign1180_e1697_d_n16, assign1180_e1697_d_n17, assign1180_e1697_d_n18, assign1180_e1697_d_b0, assign1180_e1697_d_b1, assign1180_e1697_d_b2, assign1180_e1697_d_b3, assign1180_e1697_d_b4, assign1180_e1697_d_b5, assign1180_e1697_d_b6, assign1180_e1697_d_b7, assign1180_e1697_d_b8, assign1180_e1697_d_b9, assign1180_e1697_d_b10, assign1180_e1697_d_b11, assign1180_e1697_d_b12, assign1180_e1697_d_b13, assign1180_e1697_d_b14, assign1180_e1697_d_b15, assign1180_e1697_d_b16, assign1180_e1697_d_b17, assign1180_e1697_d_b18,) = {
    if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
        let assign1180_e1694: f64 = (s.v[1] * s.v[11]);
        let assign1180_e1695: f64 = (assign1180_e1694).tanh();
        (assign1180_e1695, (((s.dn[1][0] * s.v[11]) + (s.v[1] * s.dn[11][0])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][1] * s.v[11]) + (s.v[1] * s.dn[11][1])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][2] * s.v[11]) + (s.v[1] * s.dn[11][2])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][3] * s.v[11]) + (s.v[1] * s.dn[11][3])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][4] * s.v[11]) + (s.v[1] * s.dn[11][4])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][5] * s.v[11]) + (s.v[1] * s.dn[11][5])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][6] * s.v[11]) + (s.v[1] * s.dn[11][6])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][7] * s.v[11]) + (s.v[1] * s.dn[11][7])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][8] * s.v[11]) + (s.v[1] * s.dn[11][8])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][9] * s.v[11]) + (s.v[1] * s.dn[11][9])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][10] * s.v[11]) + (s.v[1] * s.dn[11][10])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][11] * s.v[11]) + (s.v[1] * s.dn[11][11])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][12] * s.v[11]) + (s.v[1] * s.dn[11][12])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][13] * s.v[11]) + (s.v[1] * s.dn[11][13])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][14] * s.v[11]) + (s.v[1] * s.dn[11][14])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][15] * s.v[11]) + (s.v[1] * s.dn[11][15])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][16] * s.v[11]) + (s.v[1] * s.dn[11][16])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][17] * s.v[11]) + (s.v[1] * s.dn[11][17])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.dn[1][18] * s.v[11]) + (s.v[1] * s.dn[11][18])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][0] * s.v[11]) + (s.v[1] * s.db[11][0])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][1] * s.v[11]) + (s.v[1] * s.db[11][1])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][2] * s.v[11]) + (s.v[1] * s.db[11][2])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][3] * s.v[11]) + (s.v[1] * s.db[11][3])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][4] * s.v[11]) + (s.v[1] * s.db[11][4])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][5] * s.v[11]) + (s.v[1] * s.db[11][5])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][6] * s.v[11]) + (s.v[1] * s.db[11][6])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][7] * s.v[11]) + (s.v[1] * s.db[11][7])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][8] * s.v[11]) + (s.v[1] * s.db[11][8])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][9] * s.v[11]) + (s.v[1] * s.db[11][9])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][10] * s.v[11]) + (s.v[1] * s.db[11][10])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][11] * s.v[11]) + (s.v[1] * s.db[11][11])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][12] * s.v[11]) + (s.v[1] * s.db[11][12])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][13] * s.v[11]) + (s.v[1] * s.db[11][13])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][14] * s.v[11]) + (s.v[1] * s.db[11][14])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][15] * s.v[11]) + (s.v[1] * s.db[11][15])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][16] * s.v[11]) + (s.v[1] * s.db[11][16])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][17] * s.v[11]) + (s.v[1] * s.db[11][17])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())), (((s.db[1][18] * s.v[11]) + (s.v[1] * s.db[11][18])) / ((assign1180_e1694).cosh() * (assign1180_e1694).cosh())),)
    } else {
        (s.v[83], s.dn[83][0], s.dn[83][1], s.dn[83][2], s.dn[83][3], s.dn[83][4], s.dn[83][5], s.dn[83][6], s.dn[83][7], s.dn[83][8], s.dn[83][9], s.dn[83][10], s.dn[83][11], s.dn[83][12], s.dn[83][13], s.dn[83][14], s.dn[83][15], s.dn[83][16], s.dn[83][17], s.dn[83][18], s.db[83][0], s.db[83][1], s.db[83][2], s.db[83][3], s.db[83][4], s.db[83][5], s.db[83][6], s.db[83][7], s.db[83][8], s.db[83][9], s.db[83][10], s.db[83][11], s.db[83][12], s.db[83][13], s.db[83][14], s.db[83][15], s.db[83][16], s.db[83][17], s.db[83][18],)
    }
};
        s.v[83] = assign1180_e1697;
        s.mark_derivatives_dirty(83);
        s.dn[83][0] = assign1180_e1697_d_n0;
        s.dn[83][1] = assign1180_e1697_d_n1;
        s.dn[83][2] = assign1180_e1697_d_n2;
        s.dn[83][3] = assign1180_e1697_d_n3;
        s.dn[83][4] = assign1180_e1697_d_n4;
        s.dn[83][5] = assign1180_e1697_d_n5;
        s.dn[83][6] = assign1180_e1697_d_n6;
        s.dn[83][7] = assign1180_e1697_d_n7;
        s.dn[83][8] = assign1180_e1697_d_n8;
        s.dn[83][9] = assign1180_e1697_d_n9;
        s.dn[83][10] = assign1180_e1697_d_n10;
        s.dn[83][11] = assign1180_e1697_d_n11;
        s.dn[83][12] = assign1180_e1697_d_n12;
        s.dn[83][13] = assign1180_e1697_d_n13;
        s.dn[83][14] = assign1180_e1697_d_n14;
        s.dn[83][15] = assign1180_e1697_d_n15;
        s.dn[83][16] = assign1180_e1697_d_n16;
        s.dn[83][17] = assign1180_e1697_d_n17;
        s.dn[83][18] = assign1180_e1697_d_n18;
        s.db[83][0] = assign1180_e1697_d_b0;
        s.db[83][1] = assign1180_e1697_d_b1;
        s.db[83][2] = assign1180_e1697_d_b2;
        s.db[83][3] = assign1180_e1697_d_b3;
        s.db[83][4] = assign1180_e1697_d_b4;
        s.db[83][5] = assign1180_e1697_d_b5;
        s.db[83][6] = assign1180_e1697_d_b6;
        s.db[83][7] = assign1180_e1697_d_b7;
        s.db[83][8] = assign1180_e1697_d_b8;
        s.db[83][9] = assign1180_e1697_d_b9;
        s.db[83][10] = assign1180_e1697_d_b10;
        s.db[83][11] = assign1180_e1697_d_b11;
        s.db[83][12] = assign1180_e1697_d_b12;
        s.db[83][13] = assign1180_e1697_d_b13;
        s.db[83][14] = assign1180_e1697_d_b14;
        s.db[83][15] = assign1180_e1697_d_b15;
        s.db[83][16] = assign1180_e1697_d_b16;
        s.db[83][17] = assign1180_e1697_d_b17;
        s.db[83][18] = assign1180_e1697_d_b18;
        s.rv[83] = 0.0;

        let (assign1190_e1737, assign1190_e1737_d_n0, assign1190_e1737_d_n1, assign1190_e1737_d_n2, assign1190_e1737_d_n3, assign1190_e1737_d_n4, assign1190_e1737_d_n5, assign1190_e1737_d_n6, assign1190_e1737_d_n7, assign1190_e1737_d_n8, assign1190_e1737_d_n9, assign1190_e1737_d_n10, assign1190_e1737_d_n11, assign1190_e1737_d_n12, assign1190_e1737_d_n13, assign1190_e1737_d_n14, assign1190_e1737_d_n15, assign1190_e1737_d_n16, assign1190_e1737_d_n17, assign1190_e1737_d_n18, assign1190_e1737_d_b0, assign1190_e1737_d_b1, assign1190_e1737_d_b2, assign1190_e1737_d_b3, assign1190_e1737_d_b4, assign1190_e1737_d_b5, assign1190_e1737_d_b6, assign1190_e1737_d_b7, assign1190_e1737_d_b8, assign1190_e1737_d_b9, assign1190_e1737_d_b10, assign1190_e1737_d_b11, assign1190_e1737_d_b12, assign1190_e1737_d_b13, assign1190_e1737_d_b14, assign1190_e1737_d_b15, assign1190_e1737_d_b16, assign1190_e1737_d_b17, assign1190_e1737_d_b18,) = {
    if (s.b[110] && (!(((s.b[106] || s.b[107]) || s.b[108]) || s.b[109]))) {
        let assign1190_e1710: f64 = (s.v[39] * s.v[75]);
        let assign1190_e1714: f64 = (p.p65 * s.v[83]);
        let assign1190_e1715: f64 = (s.v[81] + assign1190_e1714);
        let assign1190_e1716: f64 = (assign1190_e1710 * assign1190_e1715);
        let assign1190_e1722: f64 = (p.p65 * s.v[11]);
        let assign1190_e1723: f64 = (s.v[5] + assign1190_e1722);
        let assign1190_e1724: f64 = (s.v[69] * assign1190_e1723);
        let assign1190_e1725: f64 = (1.0 + assign1190_e1724);
        let assign1190_e1730: f64 = (s.v[5] - s.v[53]);
        let assign1190_e1731: f64 = (p.p23 * assign1190_e1730);
        let assign1190_e1732: f64 = { let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1190_e1733: f64 = (s.v[43] * assign1190_e1732);
        let assign1190_e1734: f64 = (assign1190_e1725 + assign1190_e1733);
        let assign1190_e1735: f64 = (assign1190_e1716 * assign1190_e1734);
        (assign1190_e1735, ((((((s.dn[39][0] * s.v[75]) + (s.v[39] * s.dn[75][0])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][0] + (p.p65 * s.dn[83][0])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][0] * assign1190_e1723) + (s.v[69] * (s.dn[5][0] + (p.p65 * s.dn[11][0])))) + ((s.dn[43][0] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][0] - s.dn[53][0])))))))), ((((((s.dn[39][1] * s.v[75]) + (s.v[39] * s.dn[75][1])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][1] + (p.p65 * s.dn[83][1])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][1] * assign1190_e1723) + (s.v[69] * (s.dn[5][1] + (p.p65 * s.dn[11][1])))) + ((s.dn[43][1] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][1] - s.dn[53][1])))))))), ((((((s.dn[39][2] * s.v[75]) + (s.v[39] * s.dn[75][2])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][2] + (p.p65 * s.dn[83][2])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][2] * assign1190_e1723) + (s.v[69] * (s.dn[5][2] + (p.p65 * s.dn[11][2])))) + ((s.dn[43][2] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][2] - s.dn[53][2])))))))), ((((((s.dn[39][3] * s.v[75]) + (s.v[39] * s.dn[75][3])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][3] + (p.p65 * s.dn[83][3])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][3] * assign1190_e1723) + (s.v[69] * (s.dn[5][3] + (p.p65 * s.dn[11][3])))) + ((s.dn[43][3] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][3] - s.dn[53][3])))))))), ((((((s.dn[39][4] * s.v[75]) + (s.v[39] * s.dn[75][4])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][4] + (p.p65 * s.dn[83][4])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][4] * assign1190_e1723) + (s.v[69] * (s.dn[5][4] + (p.p65 * s.dn[11][4])))) + ((s.dn[43][4] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][4] - s.dn[53][4])))))))), ((((((s.dn[39][5] * s.v[75]) + (s.v[39] * s.dn[75][5])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][5] + (p.p65 * s.dn[83][5])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][5] * assign1190_e1723) + (s.v[69] * (s.dn[5][5] + (p.p65 * s.dn[11][5])))) + ((s.dn[43][5] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][5] - s.dn[53][5])))))))), ((((((s.dn[39][6] * s.v[75]) + (s.v[39] * s.dn[75][6])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][6] + (p.p65 * s.dn[83][6])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][6] * assign1190_e1723) + (s.v[69] * (s.dn[5][6] + (p.p65 * s.dn[11][6])))) + ((s.dn[43][6] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][6] - s.dn[53][6])))))))), ((((((s.dn[39][7] * s.v[75]) + (s.v[39] * s.dn[75][7])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][7] + (p.p65 * s.dn[83][7])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][7] * assign1190_e1723) + (s.v[69] * (s.dn[5][7] + (p.p65 * s.dn[11][7])))) + ((s.dn[43][7] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][7] - s.dn[53][7])))))))), ((((((s.dn[39][8] * s.v[75]) + (s.v[39] * s.dn[75][8])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][8] + (p.p65 * s.dn[83][8])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][8] * assign1190_e1723) + (s.v[69] * (s.dn[5][8] + (p.p65 * s.dn[11][8])))) + ((s.dn[43][8] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][8] - s.dn[53][8])))))))), ((((((s.dn[39][9] * s.v[75]) + (s.v[39] * s.dn[75][9])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][9] + (p.p65 * s.dn[83][9])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][9] * assign1190_e1723) + (s.v[69] * (s.dn[5][9] + (p.p65 * s.dn[11][9])))) + ((s.dn[43][9] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][9] - s.dn[53][9])))))))), ((((((s.dn[39][10] * s.v[75]) + (s.v[39] * s.dn[75][10])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][10] + (p.p65 * s.dn[83][10])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][10] * assign1190_e1723) + (s.v[69] * (s.dn[5][10] + (p.p65 * s.dn[11][10])))) + ((s.dn[43][10] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][10] - s.dn[53][10])))))))), ((((((s.dn[39][11] * s.v[75]) + (s.v[39] * s.dn[75][11])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][11] + (p.p65 * s.dn[83][11])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][11] * assign1190_e1723) + (s.v[69] * (s.dn[5][11] + (p.p65 * s.dn[11][11])))) + ((s.dn[43][11] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][11] - s.dn[53][11])))))))), ((((((s.dn[39][12] * s.v[75]) + (s.v[39] * s.dn[75][12])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][12] + (p.p65 * s.dn[83][12])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][12] * assign1190_e1723) + (s.v[69] * (s.dn[5][12] + (p.p65 * s.dn[11][12])))) + ((s.dn[43][12] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][12] - s.dn[53][12])))))))), ((((((s.dn[39][13] * s.v[75]) + (s.v[39] * s.dn[75][13])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][13] + (p.p65 * s.dn[83][13])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][13] * assign1190_e1723) + (s.v[69] * (s.dn[5][13] + (p.p65 * s.dn[11][13])))) + ((s.dn[43][13] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][13] - s.dn[53][13])))))))), ((((((s.dn[39][14] * s.v[75]) + (s.v[39] * s.dn[75][14])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][14] + (p.p65 * s.dn[83][14])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][14] * assign1190_e1723) + (s.v[69] * (s.dn[5][14] + (p.p65 * s.dn[11][14])))) + ((s.dn[43][14] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][14] - s.dn[53][14])))))))), ((((((s.dn[39][15] * s.v[75]) + (s.v[39] * s.dn[75][15])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][15] + (p.p65 * s.dn[83][15])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][15] * assign1190_e1723) + (s.v[69] * (s.dn[5][15] + (p.p65 * s.dn[11][15])))) + ((s.dn[43][15] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][15] - s.dn[53][15])))))))), ((((((s.dn[39][16] * s.v[75]) + (s.v[39] * s.dn[75][16])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][16] + (p.p65 * s.dn[83][16])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][16] * assign1190_e1723) + (s.v[69] * (s.dn[5][16] + (p.p65 * s.dn[11][16])))) + ((s.dn[43][16] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][16] - s.dn[53][16])))))))), ((((((s.dn[39][17] * s.v[75]) + (s.v[39] * s.dn[75][17])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][17] + (p.p65 * s.dn[83][17])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][17] * assign1190_e1723) + (s.v[69] * (s.dn[5][17] + (p.p65 * s.dn[11][17])))) + ((s.dn[43][17] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][17] - s.dn[53][17])))))))), ((((((s.dn[39][18] * s.v[75]) + (s.v[39] * s.dn[75][18])) * assign1190_e1715) + (assign1190_e1710 * (s.dn[81][18] + (p.p65 * s.dn[83][18])))) * assign1190_e1734) + (assign1190_e1716 * (((s.dn[69][18] * assign1190_e1723) + (s.v[69] * (s.dn[5][18] + (p.p65 * s.dn[11][18])))) + ((s.dn[43][18] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.dn[5][18] - s.dn[53][18])))))))), ((((((s.db[39][0] * s.v[75]) + (s.v[39] * s.db[75][0])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][0] + (p.p65 * s.db[83][0])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][0] * assign1190_e1723) + (s.v[69] * (s.db[5][0] + (p.p65 * s.db[11][0])))) + ((s.db[43][0] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][0] - s.db[53][0])))))))), ((((((s.db[39][1] * s.v[75]) + (s.v[39] * s.db[75][1])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][1] + (p.p65 * s.db[83][1])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][1] * assign1190_e1723) + (s.v[69] * (s.db[5][1] + (p.p65 * s.db[11][1])))) + ((s.db[43][1] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][1] - s.db[53][1])))))))), ((((((s.db[39][2] * s.v[75]) + (s.v[39] * s.db[75][2])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][2] + (p.p65 * s.db[83][2])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][2] * assign1190_e1723) + (s.v[69] * (s.db[5][2] + (p.p65 * s.db[11][2])))) + ((s.db[43][2] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][2] - s.db[53][2])))))))), ((((((s.db[39][3] * s.v[75]) + (s.v[39] * s.db[75][3])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][3] + (p.p65 * s.db[83][3])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][3] * assign1190_e1723) + (s.v[69] * (s.db[5][3] + (p.p65 * s.db[11][3])))) + ((s.db[43][3] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][3] - s.db[53][3])))))))), ((((((s.db[39][4] * s.v[75]) + (s.v[39] * s.db[75][4])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][4] + (p.p65 * s.db[83][4])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][4] * assign1190_e1723) + (s.v[69] * (s.db[5][4] + (p.p65 * s.db[11][4])))) + ((s.db[43][4] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][4] - s.db[53][4])))))))), ((((((s.db[39][5] * s.v[75]) + (s.v[39] * s.db[75][5])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][5] + (p.p65 * s.db[83][5])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][5] * assign1190_e1723) + (s.v[69] * (s.db[5][5] + (p.p65 * s.db[11][5])))) + ((s.db[43][5] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][5] - s.db[53][5])))))))), ((((((s.db[39][6] * s.v[75]) + (s.v[39] * s.db[75][6])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][6] + (p.p65 * s.db[83][6])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][6] * assign1190_e1723) + (s.v[69] * (s.db[5][6] + (p.p65 * s.db[11][6])))) + ((s.db[43][6] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][6] - s.db[53][6])))))))), ((((((s.db[39][7] * s.v[75]) + (s.v[39] * s.db[75][7])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][7] + (p.p65 * s.db[83][7])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][7] * assign1190_e1723) + (s.v[69] * (s.db[5][7] + (p.p65 * s.db[11][7])))) + ((s.db[43][7] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][7] - s.db[53][7])))))))), ((((((s.db[39][8] * s.v[75]) + (s.v[39] * s.db[75][8])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][8] + (p.p65 * s.db[83][8])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][8] * assign1190_e1723) + (s.v[69] * (s.db[5][8] + (p.p65 * s.db[11][8])))) + ((s.db[43][8] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][8] - s.db[53][8])))))))), ((((((s.db[39][9] * s.v[75]) + (s.v[39] * s.db[75][9])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][9] + (p.p65 * s.db[83][9])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][9] * assign1190_e1723) + (s.v[69] * (s.db[5][9] + (p.p65 * s.db[11][9])))) + ((s.db[43][9] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][9] - s.db[53][9])))))))), ((((((s.db[39][10] * s.v[75]) + (s.v[39] * s.db[75][10])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][10] + (p.p65 * s.db[83][10])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][10] * assign1190_e1723) + (s.v[69] * (s.db[5][10] + (p.p65 * s.db[11][10])))) + ((s.db[43][10] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][10] - s.db[53][10])))))))), ((((((s.db[39][11] * s.v[75]) + (s.v[39] * s.db[75][11])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][11] + (p.p65 * s.db[83][11])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][11] * assign1190_e1723) + (s.v[69] * (s.db[5][11] + (p.p65 * s.db[11][11])))) + ((s.db[43][11] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][11] - s.db[53][11])))))))), ((((((s.db[39][12] * s.v[75]) + (s.v[39] * s.db[75][12])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][12] + (p.p65 * s.db[83][12])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][12] * assign1190_e1723) + (s.v[69] * (s.db[5][12] + (p.p65 * s.db[11][12])))) + ((s.db[43][12] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][12] - s.db[53][12])))))))), ((((((s.db[39][13] * s.v[75]) + (s.v[39] * s.db[75][13])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][13] + (p.p65 * s.db[83][13])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][13] * assign1190_e1723) + (s.v[69] * (s.db[5][13] + (p.p65 * s.db[11][13])))) + ((s.db[43][13] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][13] - s.db[53][13])))))))), ((((((s.db[39][14] * s.v[75]) + (s.v[39] * s.db[75][14])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][14] + (p.p65 * s.db[83][14])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][14] * assign1190_e1723) + (s.v[69] * (s.db[5][14] + (p.p65 * s.db[11][14])))) + ((s.db[43][14] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][14] - s.db[53][14])))))))), ((((((s.db[39][15] * s.v[75]) + (s.v[39] * s.db[75][15])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][15] + (p.p65 * s.db[83][15])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][15] * assign1190_e1723) + (s.v[69] * (s.db[5][15] + (p.p65 * s.db[11][15])))) + ((s.db[43][15] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][15] - s.db[53][15])))))))), ((((((s.db[39][16] * s.v[75]) + (s.v[39] * s.db[75][16])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][16] + (p.p65 * s.db[83][16])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][16] * assign1190_e1723) + (s.v[69] * (s.db[5][16] + (p.p65 * s.db[11][16])))) + ((s.db[43][16] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][16] - s.db[53][16])))))))), ((((((s.db[39][17] * s.v[75]) + (s.v[39] * s.db[75][17])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][17] + (p.p65 * s.db[83][17])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][17] * assign1190_e1723) + (s.v[69] * (s.db[5][17] + (p.p65 * s.db[11][17])))) + ((s.db[43][17] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][17] - s.db[53][17])))))))), ((((((s.db[39][18] * s.v[75]) + (s.v[39] * s.db[75][18])) * assign1190_e1715) + (assign1190_e1710 * (s.db[81][18] + (p.p65 * s.db[83][18])))) * assign1190_e1734) + (assign1190_e1716 * (((s.db[69][18] * assign1190_e1723) + (s.v[69] * (s.db[5][18] + (p.p65 * s.db[11][18])))) + ((s.db[43][18] * assign1190_e1732) + (s.v[43] * ({ let limexp_arg = assign1190_e1731; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (p.p23 * (s.db[5][18] - s.db[53][18])))))))),)
    } else {
        (s.v[98], s.dn[98][0], s.dn[98][1], s.dn[98][2], s.dn[98][3], s.dn[98][4], s.dn[98][5], s.dn[98][6], s.dn[98][7], s.dn[98][8], s.dn[98][9], s.dn[98][10], s.dn[98][11], s.dn[98][12], s.dn[98][13], s.dn[98][14], s.dn[98][15], s.dn[98][16], s.dn[98][17], s.dn[98][18], s.db[98][0], s.db[98][1], s.db[98][2], s.db[98][3], s.db[98][4], s.db[98][5], s.db[98][6], s.db[98][7], s.db[98][8], s.db[98][9], s.db[98][10], s.db[98][11], s.db[98][12], s.db[98][13], s.db[98][14], s.db[98][15], s.db[98][16], s.db[98][17], s.db[98][18],)
    }
};
        s.v[98] = assign1190_e1737;
        s.mark_derivatives_dirty(98);
        s.dn[98][0] = assign1190_e1737_d_n0;
        s.dn[98][1] = assign1190_e1737_d_n1;
        s.dn[98][2] = assign1190_e1737_d_n2;
        s.dn[98][3] = assign1190_e1737_d_n3;
        s.dn[98][4] = assign1190_e1737_d_n4;
        s.dn[98][5] = assign1190_e1737_d_n5;
        s.dn[98][6] = assign1190_e1737_d_n6;
        s.dn[98][7] = assign1190_e1737_d_n7;
        s.dn[98][8] = assign1190_e1737_d_n8;
        s.dn[98][9] = assign1190_e1737_d_n9;
        s.dn[98][10] = assign1190_e1737_d_n10;
        s.dn[98][11] = assign1190_e1737_d_n11;
        s.dn[98][12] = assign1190_e1737_d_n12;
        s.dn[98][13] = assign1190_e1737_d_n13;
        s.dn[98][14] = assign1190_e1737_d_n14;
        s.dn[98][15] = assign1190_e1737_d_n15;
        s.dn[98][16] = assign1190_e1737_d_n16;
        s.dn[98][17] = assign1190_e1737_d_n17;
        s.dn[98][18] = assign1190_e1737_d_n18;
        s.db[98][0] = assign1190_e1737_d_b0;
        s.db[98][1] = assign1190_e1737_d_b1;
        s.db[98][2] = assign1190_e1737_d_b2;
        s.db[98][3] = assign1190_e1737_d_b3;
        s.db[98][4] = assign1190_e1737_d_b4;
        s.db[98][5] = assign1190_e1737_d_b5;
        s.db[98][6] = assign1190_e1737_d_b6;
        s.db[98][7] = assign1190_e1737_d_b7;
        s.db[98][8] = assign1190_e1737_d_b8;
        s.db[98][9] = assign1190_e1737_d_b9;
        s.db[98][10] = assign1190_e1737_d_b10;
        s.db[98][11] = assign1190_e1737_d_b11;
        s.db[98][12] = assign1190_e1737_d_b12;
        s.db[98][13] = assign1190_e1737_d_b13;
        s.db[98][14] = assign1190_e1737_d_b14;
        s.db[98][15] = assign1190_e1737_d_b15;
        s.db[98][16] = assign1190_e1737_d_b16;
        s.db[98][17] = assign1190_e1737_d_b17;
        s.db[98][18] = assign1190_e1737_d_b18;
        s.rv[98] = 0.0;

        s.b[111] = (((p.p4 == 0.0) || (p.p4 == 1.0)) || (p.p4 == 4.0));
        s.store_scalar(111, if s.b[111] { 1.0 } else { 0.0 });

        let (assign1210_e1758, assign1210_e1758_d_n0, assign1210_e1758_d_n1, assign1210_e1758_d_n2, assign1210_e1758_d_n3, assign1210_e1758_d_n4, assign1210_e1758_d_n5, assign1210_e1758_d_n6, assign1210_e1758_d_n7, assign1210_e1758_d_n8, assign1210_e1758_d_n9, assign1210_e1758_d_n10, assign1210_e1758_d_n11, assign1210_e1758_d_n12, assign1210_e1758_d_n13, assign1210_e1758_d_n14, assign1210_e1758_d_n15, assign1210_e1758_d_n16, assign1210_e1758_d_n17, assign1210_e1758_d_n18, assign1210_e1758_d_b0, assign1210_e1758_d_b1, assign1210_e1758_d_b2, assign1210_e1758_d_b3, assign1210_e1758_d_b4, assign1210_e1758_d_b5, assign1210_e1758_d_b6, assign1210_e1758_d_b7, assign1210_e1758_d_b8, assign1210_e1758_d_b9, assign1210_e1758_d_b10, assign1210_e1758_d_b11, assign1210_e1758_d_b12, assign1210_e1758_d_b13, assign1210_e1758_d_b14, assign1210_e1758_d_b15, assign1210_e1758_d_b16, assign1210_e1758_d_b17, assign1210_e1758_d_b18,) = {
    if s.b[111] {
        let assign1210_e1754: f64 = (1.0 + s.v[75]);
        let assign1210_e1755: f64 = (s.v[46] / assign1210_e1754);
        let assign1210_e1756: f64 = (p.p57 + assign1210_e1755);
        (assign1210_e1756, (((s.dn[46][0] * assign1210_e1754) - (s.v[46] * s.dn[75][0])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][1] * assign1210_e1754) - (s.v[46] * s.dn[75][1])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][2] * assign1210_e1754) - (s.v[46] * s.dn[75][2])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][3] * assign1210_e1754) - (s.v[46] * s.dn[75][3])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][4] * assign1210_e1754) - (s.v[46] * s.dn[75][4])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][5] * assign1210_e1754) - (s.v[46] * s.dn[75][5])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][6] * assign1210_e1754) - (s.v[46] * s.dn[75][6])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][7] * assign1210_e1754) - (s.v[46] * s.dn[75][7])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][8] * assign1210_e1754) - (s.v[46] * s.dn[75][8])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][9] * assign1210_e1754) - (s.v[46] * s.dn[75][9])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][10] * assign1210_e1754) - (s.v[46] * s.dn[75][10])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][11] * assign1210_e1754) - (s.v[46] * s.dn[75][11])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][12] * assign1210_e1754) - (s.v[46] * s.dn[75][12])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][13] * assign1210_e1754) - (s.v[46] * s.dn[75][13])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][14] * assign1210_e1754) - (s.v[46] * s.dn[75][14])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][15] * assign1210_e1754) - (s.v[46] * s.dn[75][15])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][16] * assign1210_e1754) - (s.v[46] * s.dn[75][16])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][17] * assign1210_e1754) - (s.v[46] * s.dn[75][17])) / (assign1210_e1754 * assign1210_e1754)), (((s.dn[46][18] * assign1210_e1754) - (s.v[46] * s.dn[75][18])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][0] * assign1210_e1754) - (s.v[46] * s.db[75][0])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][1] * assign1210_e1754) - (s.v[46] * s.db[75][1])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][2] * assign1210_e1754) - (s.v[46] * s.db[75][2])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][3] * assign1210_e1754) - (s.v[46] * s.db[75][3])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][4] * assign1210_e1754) - (s.v[46] * s.db[75][4])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][5] * assign1210_e1754) - (s.v[46] * s.db[75][5])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][6] * assign1210_e1754) - (s.v[46] * s.db[75][6])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][7] * assign1210_e1754) - (s.v[46] * s.db[75][7])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][8] * assign1210_e1754) - (s.v[46] * s.db[75][8])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][9] * assign1210_e1754) - (s.v[46] * s.db[75][9])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][10] * assign1210_e1754) - (s.v[46] * s.db[75][10])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][11] * assign1210_e1754) - (s.v[46] * s.db[75][11])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][12] * assign1210_e1754) - (s.v[46] * s.db[75][12])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][13] * assign1210_e1754) - (s.v[46] * s.db[75][13])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][14] * assign1210_e1754) - (s.v[46] * s.db[75][14])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][15] * assign1210_e1754) - (s.v[46] * s.db[75][15])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][16] * assign1210_e1754) - (s.v[46] * s.db[75][16])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][17] * assign1210_e1754) - (s.v[46] * s.db[75][17])) / (assign1210_e1754 * assign1210_e1754)), (((s.db[46][18] * assign1210_e1754) - (s.v[46] * s.db[75][18])) / (assign1210_e1754 * assign1210_e1754)),)
    } else {
        (s.v[40], s.dn[40][0], s.dn[40][1], s.dn[40][2], s.dn[40][3], s.dn[40][4], s.dn[40][5], s.dn[40][6], s.dn[40][7], s.dn[40][8], s.dn[40][9], s.dn[40][10], s.dn[40][11], s.dn[40][12], s.dn[40][13], s.dn[40][14], s.dn[40][15], s.dn[40][16], s.dn[40][17], s.dn[40][18], s.db[40][0], s.db[40][1], s.db[40][2], s.db[40][3], s.db[40][4], s.db[40][5], s.db[40][6], s.db[40][7], s.db[40][8], s.db[40][9], s.db[40][10], s.db[40][11], s.db[40][12], s.db[40][13], s.db[40][14], s.db[40][15], s.db[40][16], s.db[40][17], s.db[40][18],)
    }
};
        s.v[40] = assign1210_e1758;
        s.mark_derivatives_dirty(40);
        s.dn[40][0] = assign1210_e1758_d_n0;
        s.dn[40][1] = assign1210_e1758_d_n1;
        s.dn[40][2] = assign1210_e1758_d_n2;
        s.dn[40][3] = assign1210_e1758_d_n3;
        s.dn[40][4] = assign1210_e1758_d_n4;
        s.dn[40][5] = assign1210_e1758_d_n5;
        s.dn[40][6] = assign1210_e1758_d_n6;
        s.dn[40][7] = assign1210_e1758_d_n7;
        s.dn[40][8] = assign1210_e1758_d_n8;
        s.dn[40][9] = assign1210_e1758_d_n9;
        s.dn[40][10] = assign1210_e1758_d_n10;
        s.dn[40][11] = assign1210_e1758_d_n11;
        s.dn[40][12] = assign1210_e1758_d_n12;
        s.dn[40][13] = assign1210_e1758_d_n13;
        s.dn[40][14] = assign1210_e1758_d_n14;
        s.dn[40][15] = assign1210_e1758_d_n15;
        s.dn[40][16] = assign1210_e1758_d_n16;
        s.dn[40][17] = assign1210_e1758_d_n17;
        s.dn[40][18] = assign1210_e1758_d_n18;
        s.db[40][0] = assign1210_e1758_d_b0;
        s.db[40][1] = assign1210_e1758_d_b1;
        s.db[40][2] = assign1210_e1758_d_b2;
        s.db[40][3] = assign1210_e1758_d_b3;
        s.db[40][4] = assign1210_e1758_d_b4;
        s.db[40][5] = assign1210_e1758_d_b5;
        s.db[40][6] = assign1210_e1758_d_b6;
        s.db[40][7] = assign1210_e1758_d_b7;
        s.db[40][8] = assign1210_e1758_d_b8;
        s.db[40][9] = assign1210_e1758_d_b9;
        s.db[40][10] = assign1210_e1758_d_b10;
        s.db[40][11] = assign1210_e1758_d_b11;
        s.db[40][12] = assign1210_e1758_d_b12;
        s.db[40][13] = assign1210_e1758_d_b13;
        s.db[40][14] = assign1210_e1758_d_b14;
        s.db[40][15] = assign1210_e1758_d_b15;
        s.db[40][16] = assign1210_e1758_d_b16;
        s.db[40][17] = assign1210_e1758_d_b17;
        s.db[40][18] = assign1210_e1758_d_b18;
        s.rv[40] = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let (assign1220_e1766, assign1220_e1766_d_n0, assign1220_e1766_d_n1, assign1220_e1766_d_n2, assign1220_e1766_d_n3, assign1220_e1766_d_n4, assign1220_e1766_d_n5, assign1220_e1766_d_n6, assign1220_e1766_d_n7, assign1220_e1766_d_n8, assign1220_e1766_d_n9, assign1220_e1766_d_n10, assign1220_e1766_d_n11, assign1220_e1766_d_n12, assign1220_e1766_d_n13, assign1220_e1766_d_n14, assign1220_e1766_d_n15, assign1220_e1766_d_n16, assign1220_e1766_d_n17, assign1220_e1766_d_n18, assign1220_e1766_d_b0, assign1220_e1766_d_b1, assign1220_e1766_d_b2, assign1220_e1766_d_b3, assign1220_e1766_d_b4, assign1220_e1766_d_b5, assign1220_e1766_d_b6, assign1220_e1766_d_b7, assign1220_e1766_d_b8, assign1220_e1766_d_b9, assign1220_e1766_d_b10, assign1220_e1766_d_b11, assign1220_e1766_d_b12, assign1220_e1766_d_b13, assign1220_e1766_d_b14, assign1220_e1766_d_b15, assign1220_e1766_d_b16, assign1220_e1766_d_b17, assign1220_e1766_d_b18,) = {
    if s.b[111] {
        let assign1220_e1763: f64 = (p.p48 * s.v[75]);
        let assign1220_e1764: f64 = (p.p47 + assign1220_e1763);
        (assign1220_e1764, (p.p48 * s.dn[75][0]), (p.p48 * s.dn[75][1]), (p.p48 * s.dn[75][2]), (p.p48 * s.dn[75][3]), (p.p48 * s.dn[75][4]), (p.p48 * s.dn[75][5]), (p.p48 * s.dn[75][6]), (p.p48 * s.dn[75][7]), (p.p48 * s.dn[75][8]), (p.p48 * s.dn[75][9]), (p.p48 * s.dn[75][10]), (p.p48 * s.dn[75][11]), (p.p48 * s.dn[75][12]), (p.p48 * s.dn[75][13]), (p.p48 * s.dn[75][14]), (p.p48 * s.dn[75][15]), (p.p48 * s.dn[75][16]), (p.p48 * s.dn[75][17]), (p.p48 * s.dn[75][18]), (p.p48 * s.db[75][0]), (p.p48 * s.db[75][1]), (p.p48 * s.db[75][2]), (p.p48 * s.db[75][3]), (p.p48 * s.db[75][4]), (p.p48 * s.db[75][5]), (p.p48 * s.db[75][6]), (p.p48 * s.db[75][7]), (p.p48 * s.db[75][8]), (p.p48 * s.db[75][9]), (p.p48 * s.db[75][10]), (p.p48 * s.db[75][11]), (p.p48 * s.db[75][12]), (p.p48 * s.db[75][13]), (p.p48 * s.db[75][14]), (p.p48 * s.db[75][15]), (p.p48 * s.db[75][16]), (p.p48 * s.db[75][17]), (p.p48 * s.db[75][18]),)
    } else {
        (s.v[41], s.dn[41][0], s.dn[41][1], s.dn[41][2], s.dn[41][3], s.dn[41][4], s.dn[41][5], s.dn[41][6], s.dn[41][7], s.dn[41][8], s.dn[41][9], s.dn[41][10], s.dn[41][11], s.dn[41][12], s.dn[41][13], s.dn[41][14], s.dn[41][15], s.dn[41][16], s.dn[41][17], s.dn[41][18], s.db[41][0], s.db[41][1], s.db[41][2], s.db[41][3], s.db[41][4], s.db[41][5], s.db[41][6], s.db[41][7], s.db[41][8], s.db[41][9], s.db[41][10], s.db[41][11], s.db[41][12], s.db[41][13], s.db[41][14], s.db[41][15], s.db[41][16], s.db[41][17], s.db[41][18],)
    }
};
        s.v[41] = assign1220_e1766;
        s.mark_derivatives_dirty(41);
        s.dn[41][0] = assign1220_e1766_d_n0;
        s.dn[41][1] = assign1220_e1766_d_n1;
        s.dn[41][2] = assign1220_e1766_d_n2;
        s.dn[41][3] = assign1220_e1766_d_n3;
        s.dn[41][4] = assign1220_e1766_d_n4;
        s.dn[41][5] = assign1220_e1766_d_n5;
        s.dn[41][6] = assign1220_e1766_d_n6;
        s.dn[41][7] = assign1220_e1766_d_n7;
        s.dn[41][8] = assign1220_e1766_d_n8;
        s.dn[41][9] = assign1220_e1766_d_n9;
        s.dn[41][10] = assign1220_e1766_d_n10;
        s.dn[41][11] = assign1220_e1766_d_n11;
        s.dn[41][12] = assign1220_e1766_d_n12;
        s.dn[41][13] = assign1220_e1766_d_n13;
        s.dn[41][14] = assign1220_e1766_d_n14;
        s.dn[41][15] = assign1220_e1766_d_n15;
        s.dn[41][16] = assign1220_e1766_d_n16;
        s.dn[41][17] = assign1220_e1766_d_n17;
        s.dn[41][18] = assign1220_e1766_d_n18;
        s.db[41][0] = assign1220_e1766_d_b0;
        s.db[41][1] = assign1220_e1766_d_b1;
        s.db[41][2] = assign1220_e1766_d_b2;
        s.db[41][3] = assign1220_e1766_d_b3;
        s.db[41][4] = assign1220_e1766_d_b4;
        s.db[41][5] = assign1220_e1766_d_b5;
        s.db[41][6] = assign1220_e1766_d_b6;
        s.db[41][7] = assign1220_e1766_d_b7;
        s.db[41][8] = assign1220_e1766_d_b8;
        s.db[41][9] = assign1220_e1766_d_b9;
        s.db[41][10] = assign1220_e1766_d_b10;
        s.db[41][11] = assign1220_e1766_d_b11;
        s.db[41][12] = assign1220_e1766_d_b12;
        s.db[41][13] = assign1220_e1766_d_b13;
        s.db[41][14] = assign1220_e1766_d_b14;
        s.db[41][15] = assign1220_e1766_d_b15;
        s.db[41][16] = assign1220_e1766_d_b16;
        s.db[41][17] = assign1220_e1766_d_b17;
        s.db[41][18] = assign1220_e1766_d_b18;
        s.rv[41] = 0.0;

        let (assign1230_e1774, assign1230_e1774_d_n0, assign1230_e1774_d_n1, assign1230_e1774_d_n2, assign1230_e1774_d_n3, assign1230_e1774_d_n4, assign1230_e1774_d_n5, assign1230_e1774_d_n6, assign1230_e1774_d_n7, assign1230_e1774_d_n8, assign1230_e1774_d_n9, assign1230_e1774_d_n10, assign1230_e1774_d_n11, assign1230_e1774_d_n12, assign1230_e1774_d_n13, assign1230_e1774_d_n14, assign1230_e1774_d_n15, assign1230_e1774_d_n16, assign1230_e1774_d_n17, assign1230_e1774_d_n18, assign1230_e1774_d_b0, assign1230_e1774_d_b1, assign1230_e1774_d_b2, assign1230_e1774_d_b3, assign1230_e1774_d_b4, assign1230_e1774_d_b5, assign1230_e1774_d_b6, assign1230_e1774_d_b7, assign1230_e1774_d_b8, assign1230_e1774_d_b9, assign1230_e1774_d_b10, assign1230_e1774_d_b11, assign1230_e1774_d_b12, assign1230_e1774_d_b13, assign1230_e1774_d_b14, assign1230_e1774_d_b15, assign1230_e1774_d_b16, assign1230_e1774_d_b17, assign1230_e1774_d_b18,) = {
    if s.b[111] {
        let assign1230_e1771: f64 = (p.p48 * s.v[75]);
        let assign1230_e1772: f64 = (p.p50 + assign1230_e1771);
        (assign1230_e1772, (p.p48 * s.dn[75][0]), (p.p48 * s.dn[75][1]), (p.p48 * s.dn[75][2]), (p.p48 * s.dn[75][3]), (p.p48 * s.dn[75][4]), (p.p48 * s.dn[75][5]), (p.p48 * s.dn[75][6]), (p.p48 * s.dn[75][7]), (p.p48 * s.dn[75][8]), (p.p48 * s.dn[75][9]), (p.p48 * s.dn[75][10]), (p.p48 * s.dn[75][11]), (p.p48 * s.dn[75][12]), (p.p48 * s.dn[75][13]), (p.p48 * s.dn[75][14]), (p.p48 * s.dn[75][15]), (p.p48 * s.dn[75][16]), (p.p48 * s.dn[75][17]), (p.p48 * s.dn[75][18]), (p.p48 * s.db[75][0]), (p.p48 * s.db[75][1]), (p.p48 * s.db[75][2]), (p.p48 * s.db[75][3]), (p.p48 * s.db[75][4]), (p.p48 * s.db[75][5]), (p.p48 * s.db[75][6]), (p.p48 * s.db[75][7]), (p.p48 * s.db[75][8]), (p.p48 * s.db[75][9]), (p.p48 * s.db[75][10]), (p.p48 * s.db[75][11]), (p.p48 * s.db[75][12]), (p.p48 * s.db[75][13]), (p.p48 * s.db[75][14]), (p.p48 * s.db[75][15]), (p.p48 * s.db[75][16]), (p.p48 * s.db[75][17]), (p.p48 * s.db[75][18]),)
    } else {
        (s.v[42], s.dn[42][0], s.dn[42][1], s.dn[42][2], s.dn[42][3], s.dn[42][4], s.dn[42][5], s.dn[42][6], s.dn[42][7], s.dn[42][8], s.dn[42][9], s.dn[42][10], s.dn[42][11], s.dn[42][12], s.dn[42][13], s.dn[42][14], s.dn[42][15], s.dn[42][16], s.dn[42][17], s.dn[42][18], s.db[42][0], s.db[42][1], s.db[42][2], s.db[42][3], s.db[42][4], s.db[42][5], s.db[42][6], s.db[42][7], s.db[42][8], s.db[42][9], s.db[42][10], s.db[42][11], s.db[42][12], s.db[42][13], s.db[42][14], s.db[42][15], s.db[42][16], s.db[42][17], s.db[42][18],)
    }
};
        s.v[42] = assign1230_e1774;
        s.mark_derivatives_dirty(42);
        s.dn[42][0] = assign1230_e1774_d_n0;
        s.dn[42][1] = assign1230_e1774_d_n1;
        s.dn[42][2] = assign1230_e1774_d_n2;
        s.dn[42][3] = assign1230_e1774_d_n3;
        s.dn[42][4] = assign1230_e1774_d_n4;
        s.dn[42][5] = assign1230_e1774_d_n5;
        s.dn[42][6] = assign1230_e1774_d_n6;
        s.dn[42][7] = assign1230_e1774_d_n7;
        s.dn[42][8] = assign1230_e1774_d_n8;
        s.dn[42][9] = assign1230_e1774_d_n9;
        s.dn[42][10] = assign1230_e1774_d_n10;
        s.dn[42][11] = assign1230_e1774_d_n11;
        s.dn[42][12] = assign1230_e1774_d_n12;
        s.dn[42][13] = assign1230_e1774_d_n13;
        s.dn[42][14] = assign1230_e1774_d_n14;
        s.dn[42][15] = assign1230_e1774_d_n15;
        s.dn[42][16] = assign1230_e1774_d_n16;
        s.dn[42][17] = assign1230_e1774_d_n17;
        s.dn[42][18] = assign1230_e1774_d_n18;
        s.db[42][0] = assign1230_e1774_d_b0;
        s.db[42][1] = assign1230_e1774_d_b1;
        s.db[42][2] = assign1230_e1774_d_b2;
        s.db[42][3] = assign1230_e1774_d_b3;
        s.db[42][4] = assign1230_e1774_d_b4;
        s.db[42][5] = assign1230_e1774_d_b5;
        s.db[42][6] = assign1230_e1774_d_b6;
        s.db[42][7] = assign1230_e1774_d_b7;
        s.db[42][8] = assign1230_e1774_d_b8;
        s.db[42][9] = assign1230_e1774_d_b9;
        s.db[42][10] = assign1230_e1774_d_b10;
        s.db[42][11] = assign1230_e1774_d_b11;
        s.db[42][12] = assign1230_e1774_d_b12;
        s.db[42][13] = assign1230_e1774_d_b13;
        s.db[42][14] = assign1230_e1774_d_b14;
        s.db[42][15] = assign1230_e1774_d_b15;
        s.db[42][16] = assign1230_e1774_d_b16;
        s.db[42][17] = assign1230_e1774_d_b17;
        s.db[42][18] = assign1230_e1774_d_b18;
        s.rv[42] = 0.0;

        let (assign1240_e1785, assign1240_e1785_d_n0, assign1240_e1785_d_n1, assign1240_e1785_d_n2, assign1240_e1785_d_n3, assign1240_e1785_d_n4, assign1240_e1785_d_n5, assign1240_e1785_d_n6, assign1240_e1785_d_n7, assign1240_e1785_d_n8, assign1240_e1785_d_n9, assign1240_e1785_d_n10, assign1240_e1785_d_n11, assign1240_e1785_d_n12, assign1240_e1785_d_n13, assign1240_e1785_d_n14, assign1240_e1785_d_n15, assign1240_e1785_d_n16, assign1240_e1785_d_n17, assign1240_e1785_d_n18, assign1240_e1785_d_b0, assign1240_e1785_d_b1, assign1240_e1785_d_b2, assign1240_e1785_d_b3, assign1240_e1785_d_b4, assign1240_e1785_d_b5, assign1240_e1785_d_b6, assign1240_e1785_d_b7, assign1240_e1785_d_b8, assign1240_e1785_d_b9, assign1240_e1785_d_b10, assign1240_e1785_d_b11, assign1240_e1785_d_b12, assign1240_e1785_d_b13, assign1240_e1785_d_b14, assign1240_e1785_d_b15, assign1240_e1785_d_b16, assign1240_e1785_d_b17, assign1240_e1785_d_b18,) = {
    if (!s.b[111]) {
        let assign1240_e1781: f64 = (1.0 + s.v[76]);
        let assign1240_e1782: f64 = (s.v[46] / assign1240_e1781);
        let assign1240_e1783: f64 = (p.p57 + assign1240_e1782);
        (assign1240_e1783, (((s.dn[46][0] * assign1240_e1781) - (s.v[46] * s.dn[76][0])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][1] * assign1240_e1781) - (s.v[46] * s.dn[76][1])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][2] * assign1240_e1781) - (s.v[46] * s.dn[76][2])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][3] * assign1240_e1781) - (s.v[46] * s.dn[76][3])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][4] * assign1240_e1781) - (s.v[46] * s.dn[76][4])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][5] * assign1240_e1781) - (s.v[46] * s.dn[76][5])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][6] * assign1240_e1781) - (s.v[46] * s.dn[76][6])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][7] * assign1240_e1781) - (s.v[46] * s.dn[76][7])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][8] * assign1240_e1781) - (s.v[46] * s.dn[76][8])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][9] * assign1240_e1781) - (s.v[46] * s.dn[76][9])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][10] * assign1240_e1781) - (s.v[46] * s.dn[76][10])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][11] * assign1240_e1781) - (s.v[46] * s.dn[76][11])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][12] * assign1240_e1781) - (s.v[46] * s.dn[76][12])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][13] * assign1240_e1781) - (s.v[46] * s.dn[76][13])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][14] * assign1240_e1781) - (s.v[46] * s.dn[76][14])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][15] * assign1240_e1781) - (s.v[46] * s.dn[76][15])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][16] * assign1240_e1781) - (s.v[46] * s.dn[76][16])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][17] * assign1240_e1781) - (s.v[46] * s.dn[76][17])) / (assign1240_e1781 * assign1240_e1781)), (((s.dn[46][18] * assign1240_e1781) - (s.v[46] * s.dn[76][18])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][0] * assign1240_e1781) - (s.v[46] * s.db[76][0])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][1] * assign1240_e1781) - (s.v[46] * s.db[76][1])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][2] * assign1240_e1781) - (s.v[46] * s.db[76][2])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][3] * assign1240_e1781) - (s.v[46] * s.db[76][3])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][4] * assign1240_e1781) - (s.v[46] * s.db[76][4])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][5] * assign1240_e1781) - (s.v[46] * s.db[76][5])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][6] * assign1240_e1781) - (s.v[46] * s.db[76][6])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][7] * assign1240_e1781) - (s.v[46] * s.db[76][7])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][8] * assign1240_e1781) - (s.v[46] * s.db[76][8])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][9] * assign1240_e1781) - (s.v[46] * s.db[76][9])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][10] * assign1240_e1781) - (s.v[46] * s.db[76][10])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][11] * assign1240_e1781) - (s.v[46] * s.db[76][11])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][12] * assign1240_e1781) - (s.v[46] * s.db[76][12])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][13] * assign1240_e1781) - (s.v[46] * s.db[76][13])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][14] * assign1240_e1781) - (s.v[46] * s.db[76][14])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][15] * assign1240_e1781) - (s.v[46] * s.db[76][15])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][16] * assign1240_e1781) - (s.v[46] * s.db[76][16])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][17] * assign1240_e1781) - (s.v[46] * s.db[76][17])) / (assign1240_e1781 * assign1240_e1781)), (((s.db[46][18] * assign1240_e1781) - (s.v[46] * s.db[76][18])) / (assign1240_e1781 * assign1240_e1781)),)
    } else {
        (s.v[40], s.dn[40][0], s.dn[40][1], s.dn[40][2], s.dn[40][3], s.dn[40][4], s.dn[40][5], s.dn[40][6], s.dn[40][7], s.dn[40][8], s.dn[40][9], s.dn[40][10], s.dn[40][11], s.dn[40][12], s.dn[40][13], s.dn[40][14], s.dn[40][15], s.dn[40][16], s.dn[40][17], s.dn[40][18], s.db[40][0], s.db[40][1], s.db[40][2], s.db[40][3], s.db[40][4], s.db[40][5], s.db[40][6], s.db[40][7], s.db[40][8], s.db[40][9], s.db[40][10], s.db[40][11], s.db[40][12], s.db[40][13], s.db[40][14], s.db[40][15], s.db[40][16], s.db[40][17], s.db[40][18],)
    }
};
        s.v[40] = assign1240_e1785;
        s.mark_derivatives_dirty(40);
        s.dn[40][0] = assign1240_e1785_d_n0;
        s.dn[40][1] = assign1240_e1785_d_n1;
        s.dn[40][2] = assign1240_e1785_d_n2;
        s.dn[40][3] = assign1240_e1785_d_n3;
        s.dn[40][4] = assign1240_e1785_d_n4;
        s.dn[40][5] = assign1240_e1785_d_n5;
        s.dn[40][6] = assign1240_e1785_d_n6;
        s.dn[40][7] = assign1240_e1785_d_n7;
        s.dn[40][8] = assign1240_e1785_d_n8;
        s.dn[40][9] = assign1240_e1785_d_n9;
        s.dn[40][10] = assign1240_e1785_d_n10;
        s.dn[40][11] = assign1240_e1785_d_n11;
        s.dn[40][12] = assign1240_e1785_d_n12;
        s.dn[40][13] = assign1240_e1785_d_n13;
        s.dn[40][14] = assign1240_e1785_d_n14;
        s.dn[40][15] = assign1240_e1785_d_n15;
        s.dn[40][16] = assign1240_e1785_d_n16;
        s.dn[40][17] = assign1240_e1785_d_n17;
        s.dn[40][18] = assign1240_e1785_d_n18;
        s.db[40][0] = assign1240_e1785_d_b0;
        s.db[40][1] = assign1240_e1785_d_b1;
        s.db[40][2] = assign1240_e1785_d_b2;
        s.db[40][3] = assign1240_e1785_d_b3;
        s.db[40][4] = assign1240_e1785_d_b4;
        s.db[40][5] = assign1240_e1785_d_b5;
        s.db[40][6] = assign1240_e1785_d_b6;
        s.db[40][7] = assign1240_e1785_d_b7;
        s.db[40][8] = assign1240_e1785_d_b8;
        s.db[40][9] = assign1240_e1785_d_b9;
        s.db[40][10] = assign1240_e1785_d_b10;
        s.db[40][11] = assign1240_e1785_d_b11;
        s.db[40][12] = assign1240_e1785_d_b12;
        s.db[40][13] = assign1240_e1785_d_b13;
        s.db[40][14] = assign1240_e1785_d_b14;
        s.db[40][15] = assign1240_e1785_d_b15;
        s.db[40][16] = assign1240_e1785_d_b16;
        s.db[40][17] = assign1240_e1785_d_b17;
        s.db[40][18] = assign1240_e1785_d_b18;
        s.rv[40] = 0.0;

        let (assign1250_e1794, assign1250_e1794_d_n0, assign1250_e1794_d_n1, assign1250_e1794_d_n2, assign1250_e1794_d_n3, assign1250_e1794_d_n4, assign1250_e1794_d_n5, assign1250_e1794_d_n6, assign1250_e1794_d_n7, assign1250_e1794_d_n8, assign1250_e1794_d_n9, assign1250_e1794_d_n10, assign1250_e1794_d_n11, assign1250_e1794_d_n12, assign1250_e1794_d_n13, assign1250_e1794_d_n14, assign1250_e1794_d_n15, assign1250_e1794_d_n16, assign1250_e1794_d_n17, assign1250_e1794_d_n18, assign1250_e1794_d_b0, assign1250_e1794_d_b1, assign1250_e1794_d_b2, assign1250_e1794_d_b3, assign1250_e1794_d_b4, assign1250_e1794_d_b5, assign1250_e1794_d_b6, assign1250_e1794_d_b7, assign1250_e1794_d_b8, assign1250_e1794_d_b9, assign1250_e1794_d_b10, assign1250_e1794_d_b11, assign1250_e1794_d_b12, assign1250_e1794_d_b13, assign1250_e1794_d_b14, assign1250_e1794_d_b15, assign1250_e1794_d_b16, assign1250_e1794_d_b17, assign1250_e1794_d_b18,) = {
    if (!s.b[111]) {
        let assign1250_e1791: f64 = (p.p48 * s.v[76]);
        let assign1250_e1792: f64 = (p.p47 + assign1250_e1791);
        (assign1250_e1792, (p.p48 * s.dn[76][0]), (p.p48 * s.dn[76][1]), (p.p48 * s.dn[76][2]), (p.p48 * s.dn[76][3]), (p.p48 * s.dn[76][4]), (p.p48 * s.dn[76][5]), (p.p48 * s.dn[76][6]), (p.p48 * s.dn[76][7]), (p.p48 * s.dn[76][8]), (p.p48 * s.dn[76][9]), (p.p48 * s.dn[76][10]), (p.p48 * s.dn[76][11]), (p.p48 * s.dn[76][12]), (p.p48 * s.dn[76][13]), (p.p48 * s.dn[76][14]), (p.p48 * s.dn[76][15]), (p.p48 * s.dn[76][16]), (p.p48 * s.dn[76][17]), (p.p48 * s.dn[76][18]), (p.p48 * s.db[76][0]), (p.p48 * s.db[76][1]), (p.p48 * s.db[76][2]), (p.p48 * s.db[76][3]), (p.p48 * s.db[76][4]), (p.p48 * s.db[76][5]), (p.p48 * s.db[76][6]), (p.p48 * s.db[76][7]), (p.p48 * s.db[76][8]), (p.p48 * s.db[76][9]), (p.p48 * s.db[76][10]), (p.p48 * s.db[76][11]), (p.p48 * s.db[76][12]), (p.p48 * s.db[76][13]), (p.p48 * s.db[76][14]), (p.p48 * s.db[76][15]), (p.p48 * s.db[76][16]), (p.p48 * s.db[76][17]), (p.p48 * s.db[76][18]),)
    } else {
        (s.v[41], s.dn[41][0], s.dn[41][1], s.dn[41][2], s.dn[41][3], s.dn[41][4], s.dn[41][5], s.dn[41][6], s.dn[41][7], s.dn[41][8], s.dn[41][9], s.dn[41][10], s.dn[41][11], s.dn[41][12], s.dn[41][13], s.dn[41][14], s.dn[41][15], s.dn[41][16], s.dn[41][17], s.dn[41][18], s.db[41][0], s.db[41][1], s.db[41][2], s.db[41][3], s.db[41][4], s.db[41][5], s.db[41][6], s.db[41][7], s.db[41][8], s.db[41][9], s.db[41][10], s.db[41][11], s.db[41][12], s.db[41][13], s.db[41][14], s.db[41][15], s.db[41][16], s.db[41][17], s.db[41][18],)
    }
};
        s.v[41] = assign1250_e1794;
        s.mark_derivatives_dirty(41);
        s.dn[41][0] = assign1250_e1794_d_n0;
        s.dn[41][1] = assign1250_e1794_d_n1;
        s.dn[41][2] = assign1250_e1794_d_n2;
        s.dn[41][3] = assign1250_e1794_d_n3;
        s.dn[41][4] = assign1250_e1794_d_n4;
        s.dn[41][5] = assign1250_e1794_d_n5;
        s.dn[41][6] = assign1250_e1794_d_n6;
        s.dn[41][7] = assign1250_e1794_d_n7;
        s.dn[41][8] = assign1250_e1794_d_n8;
        s.dn[41][9] = assign1250_e1794_d_n9;
        s.dn[41][10] = assign1250_e1794_d_n10;
        s.dn[41][11] = assign1250_e1794_d_n11;
        s.dn[41][12] = assign1250_e1794_d_n12;
        s.dn[41][13] = assign1250_e1794_d_n13;
        s.dn[41][14] = assign1250_e1794_d_n14;
        s.dn[41][15] = assign1250_e1794_d_n15;
        s.dn[41][16] = assign1250_e1794_d_n16;
        s.dn[41][17] = assign1250_e1794_d_n17;
        s.dn[41][18] = assign1250_e1794_d_n18;
        s.db[41][0] = assign1250_e1794_d_b0;
        s.db[41][1] = assign1250_e1794_d_b1;
        s.db[41][2] = assign1250_e1794_d_b2;
        s.db[41][3] = assign1250_e1794_d_b3;
        s.db[41][4] = assign1250_e1794_d_b4;
        s.db[41][5] = assign1250_e1794_d_b5;
        s.db[41][6] = assign1250_e1794_d_b6;
        s.db[41][7] = assign1250_e1794_d_b7;
        s.db[41][8] = assign1250_e1794_d_b8;
        s.db[41][9] = assign1250_e1794_d_b9;
        s.db[41][10] = assign1250_e1794_d_b10;
        s.db[41][11] = assign1250_e1794_d_b11;
        s.db[41][12] = assign1250_e1794_d_b12;
        s.db[41][13] = assign1250_e1794_d_b13;
        s.db[41][14] = assign1250_e1794_d_b14;
        s.db[41][15] = assign1250_e1794_d_b15;
        s.db[41][16] = assign1250_e1794_d_b16;
        s.db[41][17] = assign1250_e1794_d_b17;
        s.db[41][18] = assign1250_e1794_d_b18;
        s.rv[41] = 0.0;

        let (assign1260_e1803, assign1260_e1803_d_n0, assign1260_e1803_d_n1, assign1260_e1803_d_n2, assign1260_e1803_d_n3, assign1260_e1803_d_n4, assign1260_e1803_d_n5, assign1260_e1803_d_n6, assign1260_e1803_d_n7, assign1260_e1803_d_n8, assign1260_e1803_d_n9, assign1260_e1803_d_n10, assign1260_e1803_d_n11, assign1260_e1803_d_n12, assign1260_e1803_d_n13, assign1260_e1803_d_n14, assign1260_e1803_d_n15, assign1260_e1803_d_n16, assign1260_e1803_d_n17, assign1260_e1803_d_n18, assign1260_e1803_d_b0, assign1260_e1803_d_b1, assign1260_e1803_d_b2, assign1260_e1803_d_b3, assign1260_e1803_d_b4, assign1260_e1803_d_b5, assign1260_e1803_d_b6, assign1260_e1803_d_b7, assign1260_e1803_d_b8, assign1260_e1803_d_b9, assign1260_e1803_d_b10, assign1260_e1803_d_b11, assign1260_e1803_d_b12, assign1260_e1803_d_b13, assign1260_e1803_d_b14, assign1260_e1803_d_b15, assign1260_e1803_d_b16, assign1260_e1803_d_b17, assign1260_e1803_d_b18,) = {
    if (!s.b[111]) {
        let assign1260_e1800: f64 = (p.p48 * s.v[76]);
        let assign1260_e1801: f64 = (p.p50 + assign1260_e1800);
        (assign1260_e1801, (p.p48 * s.dn[76][0]), (p.p48 * s.dn[76][1]), (p.p48 * s.dn[76][2]), (p.p48 * s.dn[76][3]), (p.p48 * s.dn[76][4]), (p.p48 * s.dn[76][5]), (p.p48 * s.dn[76][6]), (p.p48 * s.dn[76][7]), (p.p48 * s.dn[76][8]), (p.p48 * s.dn[76][9]), (p.p48 * s.dn[76][10]), (p.p48 * s.dn[76][11]), (p.p48 * s.dn[76][12]), (p.p48 * s.dn[76][13]), (p.p48 * s.dn[76][14]), (p.p48 * s.dn[76][15]), (p.p48 * s.dn[76][16]), (p.p48 * s.dn[76][17]), (p.p48 * s.dn[76][18]), (p.p48 * s.db[76][0]), (p.p48 * s.db[76][1]), (p.p48 * s.db[76][2]), (p.p48 * s.db[76][3]), (p.p48 * s.db[76][4]), (p.p48 * s.db[76][5]), (p.p48 * s.db[76][6]), (p.p48 * s.db[76][7]), (p.p48 * s.db[76][8]), (p.p48 * s.db[76][9]), (p.p48 * s.db[76][10]), (p.p48 * s.db[76][11]), (p.p48 * s.db[76][12]), (p.p48 * s.db[76][13]), (p.p48 * s.db[76][14]), (p.p48 * s.db[76][15]), (p.p48 * s.db[76][16]), (p.p48 * s.db[76][17]), (p.p48 * s.db[76][18]),)
    } else {
        (s.v[42], s.dn[42][0], s.dn[42][1], s.dn[42][2], s.dn[42][3], s.dn[42][4], s.dn[42][5], s.dn[42][6], s.dn[42][7], s.dn[42][8], s.dn[42][9], s.dn[42][10], s.dn[42][11], s.dn[42][12], s.dn[42][13], s.dn[42][14], s.dn[42][15], s.dn[42][16], s.dn[42][17], s.dn[42][18], s.db[42][0], s.db[42][1], s.db[42][2], s.db[42][3], s.db[42][4], s.db[42][5], s.db[42][6], s.db[42][7], s.db[42][8], s.db[42][9], s.db[42][10], s.db[42][11], s.db[42][12], s.db[42][13], s.db[42][14], s.db[42][15], s.db[42][16], s.db[42][17], s.db[42][18],)
    }
};
        s.v[42] = assign1260_e1803;
        s.mark_derivatives_dirty(42);
        s.dn[42][0] = assign1260_e1803_d_n0;
        s.dn[42][1] = assign1260_e1803_d_n1;
        s.dn[42][2] = assign1260_e1803_d_n2;
        s.dn[42][3] = assign1260_e1803_d_n3;
        s.dn[42][4] = assign1260_e1803_d_n4;
        s.dn[42][5] = assign1260_e1803_d_n5;
        s.dn[42][6] = assign1260_e1803_d_n6;
        s.dn[42][7] = assign1260_e1803_d_n7;
        s.dn[42][8] = assign1260_e1803_d_n8;
        s.dn[42][9] = assign1260_e1803_d_n9;
        s.dn[42][10] = assign1260_e1803_d_n10;
        s.dn[42][11] = assign1260_e1803_d_n11;
        s.dn[42][12] = assign1260_e1803_d_n12;
        s.dn[42][13] = assign1260_e1803_d_n13;
        s.dn[42][14] = assign1260_e1803_d_n14;
        s.dn[42][15] = assign1260_e1803_d_n15;
        s.dn[42][16] = assign1260_e1803_d_n16;
        s.dn[42][17] = assign1260_e1803_d_n17;
        s.dn[42][18] = assign1260_e1803_d_n18;
        s.db[42][0] = assign1260_e1803_d_b0;
        s.db[42][1] = assign1260_e1803_d_b1;
        s.db[42][2] = assign1260_e1803_d_b2;
        s.db[42][3] = assign1260_e1803_d_b3;
        s.db[42][4] = assign1260_e1803_d_b4;
        s.db[42][5] = assign1260_e1803_d_b5;
        s.db[42][6] = assign1260_e1803_d_b6;
        s.db[42][7] = assign1260_e1803_d_b7;
        s.db[42][8] = assign1260_e1803_d_b8;
        s.db[42][9] = assign1260_e1803_d_b9;
        s.db[42][10] = assign1260_e1803_d_b10;
        s.db[42][11] = assign1260_e1803_d_b11;
        s.db[42][12] = assign1260_e1803_d_b12;
        s.db[42][13] = assign1260_e1803_d_b13;
        s.db[42][14] = assign1260_e1803_d_b14;
        s.db[42][15] = assign1260_e1803_d_b15;
        s.db[42][16] = assign1260_e1803_d_b16;
        s.db[42][17] = assign1260_e1803_d_b17;
        s.db[42][18] = assign1260_e1803_d_b18;
        s.rv[42] = 0.0;

        let assign1270_e1808: f64 = (s.v[16]).abs();
        let assign1270_e1809: f64 = (p.p76 * assign1270_e1808);
        let assign1270_e1810: f64 = (1.0 + assign1270_e1809);
        let assign1270_e1811: f64 = (s.v[42] * assign1270_e1810);
        s.v[50] = assign1270_e1811;
        s.mark_derivatives_dirty(50);
        s.dn[50][0] = ((s.dn[42][0] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][0] } else { (-s.dn[16][0]) })));
        s.dn[50][1] = ((s.dn[42][1] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][1] } else { (-s.dn[16][1]) })));
        s.dn[50][2] = ((s.dn[42][2] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][2] } else { (-s.dn[16][2]) })));
        s.dn[50][3] = ((s.dn[42][3] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][3] } else { (-s.dn[16][3]) })));
        s.dn[50][4] = ((s.dn[42][4] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][4] } else { (-s.dn[16][4]) })));
        s.dn[50][5] = ((s.dn[42][5] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][5] } else { (-s.dn[16][5]) })));
        s.dn[50][6] = ((s.dn[42][6] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][6] } else { (-s.dn[16][6]) })));
        s.dn[50][7] = ((s.dn[42][7] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][7] } else { (-s.dn[16][7]) })));
        s.dn[50][8] = ((s.dn[42][8] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][8] } else { (-s.dn[16][8]) })));
        s.dn[50][9] = ((s.dn[42][9] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][9] } else { (-s.dn[16][9]) })));
        s.dn[50][10] = ((s.dn[42][10] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][10] } else { (-s.dn[16][10]) })));
        s.dn[50][11] = ((s.dn[42][11] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][11] } else { (-s.dn[16][11]) })));
        s.dn[50][12] = ((s.dn[42][12] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][12] } else { (-s.dn[16][12]) })));
        s.dn[50][13] = ((s.dn[42][13] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][13] } else { (-s.dn[16][13]) })));
        s.dn[50][14] = ((s.dn[42][14] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][14] } else { (-s.dn[16][14]) })));
        s.dn[50][15] = ((s.dn[42][15] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][15] } else { (-s.dn[16][15]) })));
        s.dn[50][16] = ((s.dn[42][16] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][16] } else { (-s.dn[16][16]) })));
        s.dn[50][17] = ((s.dn[42][17] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][17] } else { (-s.dn[16][17]) })));
        s.dn[50][18] = ((s.dn[42][18] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][18] } else { (-s.dn[16][18]) })));
        s.db[50][0] = ((s.db[42][0] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][0] } else { (-s.db[16][0]) })));
        s.db[50][1] = ((s.db[42][1] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][1] } else { (-s.db[16][1]) })));
        s.db[50][2] = ((s.db[42][2] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][2] } else { (-s.db[16][2]) })));
        s.db[50][3] = ((s.db[42][3] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][3] } else { (-s.db[16][3]) })));
        s.db[50][4] = ((s.db[42][4] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][4] } else { (-s.db[16][4]) })));
        s.db[50][5] = ((s.db[42][5] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][5] } else { (-s.db[16][5]) })));
        s.db[50][6] = ((s.db[42][6] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][6] } else { (-s.db[16][6]) })));
        s.db[50][7] = ((s.db[42][7] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][7] } else { (-s.db[16][7]) })));
        s.db[50][8] = ((s.db[42][8] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][8] } else { (-s.db[16][8]) })));
        s.db[50][9] = ((s.db[42][9] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][9] } else { (-s.db[16][9]) })));
        s.db[50][10] = ((s.db[42][10] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][10] } else { (-s.db[16][10]) })));
        s.db[50][11] = ((s.db[42][11] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][11] } else { (-s.db[16][11]) })));
        s.db[50][12] = ((s.db[42][12] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][12] } else { (-s.db[16][12]) })));
        s.db[50][13] = ((s.db[42][13] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][13] } else { (-s.db[16][13]) })));
        s.db[50][14] = ((s.db[42][14] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][14] } else { (-s.db[16][14]) })));
        s.db[50][15] = ((s.db[42][15] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][15] } else { (-s.db[16][15]) })));
        s.db[50][16] = ((s.db[42][16] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][16] } else { (-s.db[16][16]) })));
        s.db[50][17] = ((s.db[42][17] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][17] } else { (-s.db[16][17]) })));
        s.db[50][18] = ((s.db[42][18] * assign1270_e1810) + (s.v[42] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][18] } else { (-s.db[16][18]) })));
        s.rv[50] = 0.0;

        let assign1280_e1816: f64 = (s.v[16]).abs();
        let assign1280_e1817: f64 = (p.p76 * assign1280_e1816);
        let assign1280_e1818: f64 = (1.0 + assign1280_e1817);
        let assign1280_e1819: f64 = (s.v[41] * assign1280_e1818);
        s.v[49] = assign1280_e1819;
        s.mark_derivatives_dirty(49);
        s.dn[49][0] = ((s.dn[41][0] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][0] } else { (-s.dn[16][0]) })));
        s.dn[49][1] = ((s.dn[41][1] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][1] } else { (-s.dn[16][1]) })));
        s.dn[49][2] = ((s.dn[41][2] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][2] } else { (-s.dn[16][2]) })));
        s.dn[49][3] = ((s.dn[41][3] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][3] } else { (-s.dn[16][3]) })));
        s.dn[49][4] = ((s.dn[41][4] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][4] } else { (-s.dn[16][4]) })));
        s.dn[49][5] = ((s.dn[41][5] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][5] } else { (-s.dn[16][5]) })));
        s.dn[49][6] = ((s.dn[41][6] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][6] } else { (-s.dn[16][6]) })));
        s.dn[49][7] = ((s.dn[41][7] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][7] } else { (-s.dn[16][7]) })));
        s.dn[49][8] = ((s.dn[41][8] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][8] } else { (-s.dn[16][8]) })));
        s.dn[49][9] = ((s.dn[41][9] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][9] } else { (-s.dn[16][9]) })));
        s.dn[49][10] = ((s.dn[41][10] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][10] } else { (-s.dn[16][10]) })));
        s.dn[49][11] = ((s.dn[41][11] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][11] } else { (-s.dn[16][11]) })));
        s.dn[49][12] = ((s.dn[41][12] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][12] } else { (-s.dn[16][12]) })));
        s.dn[49][13] = ((s.dn[41][13] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][13] } else { (-s.dn[16][13]) })));
        s.dn[49][14] = ((s.dn[41][14] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][14] } else { (-s.dn[16][14]) })));
        s.dn[49][15] = ((s.dn[41][15] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][15] } else { (-s.dn[16][15]) })));
        s.dn[49][16] = ((s.dn[41][16] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][16] } else { (-s.dn[16][16]) })));
        s.dn[49][17] = ((s.dn[41][17] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][17] } else { (-s.dn[16][17]) })));
        s.dn[49][18] = ((s.dn[41][18] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.dn[16][18] } else { (-s.dn[16][18]) })));
        s.db[49][0] = ((s.db[41][0] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][0] } else { (-s.db[16][0]) })));
        s.db[49][1] = ((s.db[41][1] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][1] } else { (-s.db[16][1]) })));
        s.db[49][2] = ((s.db[41][2] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][2] } else { (-s.db[16][2]) })));
        s.db[49][3] = ((s.db[41][3] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][3] } else { (-s.db[16][3]) })));
        s.db[49][4] = ((s.db[41][4] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][4] } else { (-s.db[16][4]) })));
        s.db[49][5] = ((s.db[41][5] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][5] } else { (-s.db[16][5]) })));
        s.db[49][6] = ((s.db[41][6] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][6] } else { (-s.db[16][6]) })));
        s.db[49][7] = ((s.db[41][7] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][7] } else { (-s.db[16][7]) })));
        s.db[49][8] = ((s.db[41][8] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][8] } else { (-s.db[16][8]) })));
        s.db[49][9] = ((s.db[41][9] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][9] } else { (-s.db[16][9]) })));
        s.db[49][10] = ((s.db[41][10] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][10] } else { (-s.db[16][10]) })));
        s.db[49][11] = ((s.db[41][11] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][11] } else { (-s.db[16][11]) })));
        s.db[49][12] = ((s.db[41][12] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][12] } else { (-s.db[16][12]) })));
        s.db[49][13] = ((s.db[41][13] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][13] } else { (-s.db[16][13]) })));
        s.db[49][14] = ((s.db[41][14] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][14] } else { (-s.db[16][14]) })));
        s.db[49][15] = ((s.db[41][15] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][15] } else { (-s.db[16][15]) })));
        s.db[49][16] = ((s.db[41][16] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][16] } else { (-s.db[16][16]) })));
        s.db[49][17] = ((s.db[41][17] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][17] } else { (-s.db[16][17]) })));
        s.db[49][18] = ((s.db[41][18] * assign1280_e1818) + (s.v[41] * (p.p76 * if s.v[16] >= 0.0 { s.db[16][18] } else { (-s.db[16][18]) })));
        s.rv[49] = 0.0;

        s.b[112] = (p.p5 == 0.0);
        s.store_scalar(112, if s.b[112] { 1.0 } else { 0.0 });

        let (assign1310_e1841, assign1310_e1841_d_n0, assign1310_e1841_d_n1, assign1310_e1841_d_n2, assign1310_e1841_d_n3, assign1310_e1841_d_n4, assign1310_e1841_d_n5, assign1310_e1841_d_n6, assign1310_e1841_d_n7, assign1310_e1841_d_n8, assign1310_e1841_d_n9, assign1310_e1841_d_n10, assign1310_e1841_d_n11, assign1310_e1841_d_n12, assign1310_e1841_d_n13, assign1310_e1841_d_n14, assign1310_e1841_d_n15, assign1310_e1841_d_n16, assign1310_e1841_d_n17, assign1310_e1841_d_n18, assign1310_e1841_d_b0, assign1310_e1841_d_b1, assign1310_e1841_d_b2, assign1310_e1841_d_b3, assign1310_e1841_d_b4, assign1310_e1841_d_b5, assign1310_e1841_d_b6, assign1310_e1841_d_b7, assign1310_e1841_d_b8, assign1310_e1841_d_b9, assign1310_e1841_d_b10, assign1310_e1841_d_b11, assign1310_e1841_d_b12, assign1310_e1841_d_b13, assign1310_e1841_d_b14, assign1310_e1841_d_b15, assign1310_e1841_d_b16, assign1310_e1841_d_b17, assign1310_e1841_d_b18,) = {
    if s.b[112] {
        let assign1310_e1834: f64 = (-1.0);
        let assign1310_e1836: f64 = (assign1310_e1834 * s.v[57]);
        let assign1310_e1837: f64 = (assign1310_e1836).tanh();
        let assign1310_e1838: f64 = (s.v[19] * assign1310_e1837);
        let assign1310_e1839: f64 = { let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1310_e1839, ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][0] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][0]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][1] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][1]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][2] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][2]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][3] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][3]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][4] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][4]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][5] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][5]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][6] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][6]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][7] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][7]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][8] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][8]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][9] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][9]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][10] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][10]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][11] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][11]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][12] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][12]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][13] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][13]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][14] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][14]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][15] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][15]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][16] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][16]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][17] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][17]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][18] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.dn[57][18]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][0] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][0]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][1] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][1]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][2] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][2]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][3] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][3]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][4] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][4]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][5] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][5]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][6] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][6]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][7] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][7]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][8] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][8]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][9] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][9]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][10] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][10]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][11] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][11]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][12] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][12]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][13] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][13]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][14] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][14]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][15] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][15]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][16] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][16]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][17] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][17]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][18] * assign1310_e1837) + (s.v[19] * ((assign1310_e1834 * s.db[57][18]) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))),)
    } else {
        (s.v[63], s.dn[63][0], s.dn[63][1], s.dn[63][2], s.dn[63][3], s.dn[63][4], s.dn[63][5], s.dn[63][6], s.dn[63][7], s.dn[63][8], s.dn[63][9], s.dn[63][10], s.dn[63][11], s.dn[63][12], s.dn[63][13], s.dn[63][14], s.dn[63][15], s.dn[63][16], s.dn[63][17], s.dn[63][18], s.db[63][0], s.db[63][1], s.db[63][2], s.db[63][3], s.db[63][4], s.db[63][5], s.db[63][6], s.db[63][7], s.db[63][8], s.db[63][9], s.db[63][10], s.db[63][11], s.db[63][12], s.db[63][13], s.db[63][14], s.db[63][15], s.db[63][16], s.db[63][17], s.db[63][18],)
    }
};
        s.v[63] = assign1310_e1841;
        s.mark_derivatives_dirty(63);
        s.dn[63][0] = assign1310_e1841_d_n0;
        s.dn[63][1] = assign1310_e1841_d_n1;
        s.dn[63][2] = assign1310_e1841_d_n2;
        s.dn[63][3] = assign1310_e1841_d_n3;
        s.dn[63][4] = assign1310_e1841_d_n4;
        s.dn[63][5] = assign1310_e1841_d_n5;
        s.dn[63][6] = assign1310_e1841_d_n6;
        s.dn[63][7] = assign1310_e1841_d_n7;
        s.dn[63][8] = assign1310_e1841_d_n8;
        s.dn[63][9] = assign1310_e1841_d_n9;
        s.dn[63][10] = assign1310_e1841_d_n10;
        s.dn[63][11] = assign1310_e1841_d_n11;
        s.dn[63][12] = assign1310_e1841_d_n12;
        s.dn[63][13] = assign1310_e1841_d_n13;
        s.dn[63][14] = assign1310_e1841_d_n14;
        s.dn[63][15] = assign1310_e1841_d_n15;
        s.dn[63][16] = assign1310_e1841_d_n16;
        s.dn[63][17] = assign1310_e1841_d_n17;
        s.dn[63][18] = assign1310_e1841_d_n18;
        s.db[63][0] = assign1310_e1841_d_b0;
        s.db[63][1] = assign1310_e1841_d_b1;
        s.db[63][2] = assign1310_e1841_d_b2;
        s.db[63][3] = assign1310_e1841_d_b3;
        s.db[63][4] = assign1310_e1841_d_b4;
        s.db[63][5] = assign1310_e1841_d_b5;
        s.db[63][6] = assign1310_e1841_d_b6;
        s.db[63][7] = assign1310_e1841_d_b7;
        s.db[63][8] = assign1310_e1841_d_b8;
        s.db[63][9] = assign1310_e1841_d_b9;
        s.db[63][10] = assign1310_e1841_d_b10;
        s.db[63][11] = assign1310_e1841_d_b11;
        s.db[63][12] = assign1310_e1841_d_b12;
        s.db[63][13] = assign1310_e1841_d_b13;
        s.db[63][14] = assign1310_e1841_d_b14;
        s.db[63][15] = assign1310_e1841_d_b15;
        s.db[63][16] = assign1310_e1841_d_b16;
        s.db[63][17] = assign1310_e1841_d_b17;
        s.db[63][18] = assign1310_e1841_d_b18;
        s.rv[63] = 0.0;

        if s.b[112] {
            s.store_sub(20, 96, 57);
            s.store_offset_scaled(21, 96, -1.0, (-p.p83));
            s.store_sub(22, 97, 57);
            s.store_offset_scaled(23, 97, -1.0, (-p.p84));
        }

        let (assign1360_e1880, assign1360_e1880_d_n0, assign1360_e1880_d_n1, assign1360_e1880_d_n2, assign1360_e1880_d_n3, assign1360_e1880_d_n4, assign1360_e1880_d_n5, assign1360_e1880_d_n6, assign1360_e1880_d_n7, assign1360_e1880_d_n8, assign1360_e1880_d_n9, assign1360_e1880_d_n10, assign1360_e1880_d_n11, assign1360_e1880_d_n12, assign1360_e1880_d_n13, assign1360_e1880_d_n14, assign1360_e1880_d_n15, assign1360_e1880_d_n16, assign1360_e1880_d_n17, assign1360_e1880_d_n18, assign1360_e1880_d_b0, assign1360_e1880_d_b1, assign1360_e1880_d_b2, assign1360_e1880_d_b3, assign1360_e1880_d_b4, assign1360_e1880_d_b5, assign1360_e1880_d_b6, assign1360_e1880_d_b7, assign1360_e1880_d_b8, assign1360_e1880_d_b9, assign1360_e1880_d_b10, assign1360_e1880_d_b11, assign1360_e1880_d_b12, assign1360_e1880_d_b13, assign1360_e1880_d_b14, assign1360_e1880_d_b15, assign1360_e1880_d_b16, assign1360_e1880_d_b17, assign1360_e1880_d_b18,) = {
    if (!s.b[112]) {
        let assign1360_e1875: f64 = (-s.v[19]);
        let assign1360_e1877: f64 = (assign1360_e1875 * s.v[57]);
        let assign1360_e1878: f64 = { let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1360_e1878, ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][0]) * s.v[57]) + (assign1360_e1875 * s.dn[57][0]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][1]) * s.v[57]) + (assign1360_e1875 * s.dn[57][1]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][2]) * s.v[57]) + (assign1360_e1875 * s.dn[57][2]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][3]) * s.v[57]) + (assign1360_e1875 * s.dn[57][3]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][4]) * s.v[57]) + (assign1360_e1875 * s.dn[57][4]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][5]) * s.v[57]) + (assign1360_e1875 * s.dn[57][5]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][6]) * s.v[57]) + (assign1360_e1875 * s.dn[57][6]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][7]) * s.v[57]) + (assign1360_e1875 * s.dn[57][7]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][8]) * s.v[57]) + (assign1360_e1875 * s.dn[57][8]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][9]) * s.v[57]) + (assign1360_e1875 * s.dn[57][9]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][10]) * s.v[57]) + (assign1360_e1875 * s.dn[57][10]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][11]) * s.v[57]) + (assign1360_e1875 * s.dn[57][11]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][12]) * s.v[57]) + (assign1360_e1875 * s.dn[57][12]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][13]) * s.v[57]) + (assign1360_e1875 * s.dn[57][13]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][14]) * s.v[57]) + (assign1360_e1875 * s.dn[57][14]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][15]) * s.v[57]) + (assign1360_e1875 * s.dn[57][15]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][16]) * s.v[57]) + (assign1360_e1875 * s.dn[57][16]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][17]) * s.v[57]) + (assign1360_e1875 * s.dn[57][17]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.dn[19][18]) * s.v[57]) + (assign1360_e1875 * s.dn[57][18]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][0]) * s.v[57]) + (assign1360_e1875 * s.db[57][0]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][1]) * s.v[57]) + (assign1360_e1875 * s.db[57][1]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][2]) * s.v[57]) + (assign1360_e1875 * s.db[57][2]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][3]) * s.v[57]) + (assign1360_e1875 * s.db[57][3]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][4]) * s.v[57]) + (assign1360_e1875 * s.db[57][4]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][5]) * s.v[57]) + (assign1360_e1875 * s.db[57][5]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][6]) * s.v[57]) + (assign1360_e1875 * s.db[57][6]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][7]) * s.v[57]) + (assign1360_e1875 * s.db[57][7]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][8]) * s.v[57]) + (assign1360_e1875 * s.db[57][8]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][9]) * s.v[57]) + (assign1360_e1875 * s.db[57][9]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][10]) * s.v[57]) + (assign1360_e1875 * s.db[57][10]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][11]) * s.v[57]) + (assign1360_e1875 * s.db[57][11]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][12]) * s.v[57]) + (assign1360_e1875 * s.db[57][12]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][13]) * s.v[57]) + (assign1360_e1875 * s.db[57][13]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][14]) * s.v[57]) + (assign1360_e1875 * s.db[57][14]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][15]) * s.v[57]) + (assign1360_e1875 * s.db[57][15]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][16]) * s.v[57]) + (assign1360_e1875 * s.db[57][16]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][17]) * s.v[57]) + (assign1360_e1875 * s.db[57][17]))), ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-s.db[19][18]) * s.v[57]) + (assign1360_e1875 * s.db[57][18]))),)
    } else {
        (s.v[63], s.dn[63][0], s.dn[63][1], s.dn[63][2], s.dn[63][3], s.dn[63][4], s.dn[63][5], s.dn[63][6], s.dn[63][7], s.dn[63][8], s.dn[63][9], s.dn[63][10], s.dn[63][11], s.dn[63][12], s.dn[63][13], s.dn[63][14], s.dn[63][15], s.dn[63][16], s.dn[63][17], s.dn[63][18], s.db[63][0], s.db[63][1], s.db[63][2], s.db[63][3], s.db[63][4], s.db[63][5], s.db[63][6], s.db[63][7], s.db[63][8], s.db[63][9], s.db[63][10], s.db[63][11], s.db[63][12], s.db[63][13], s.db[63][14], s.db[63][15], s.db[63][16], s.db[63][17], s.db[63][18],)
    }
};
        s.v[63] = assign1360_e1880;
        s.mark_derivatives_dirty(63);
        s.dn[63][0] = assign1360_e1880_d_n0;
        s.dn[63][1] = assign1360_e1880_d_n1;
        s.dn[63][2] = assign1360_e1880_d_n2;
        s.dn[63][3] = assign1360_e1880_d_n3;
        s.dn[63][4] = assign1360_e1880_d_n4;
        s.dn[63][5] = assign1360_e1880_d_n5;
        s.dn[63][6] = assign1360_e1880_d_n6;
        s.dn[63][7] = assign1360_e1880_d_n7;
        s.dn[63][8] = assign1360_e1880_d_n8;
        s.dn[63][9] = assign1360_e1880_d_n9;
        s.dn[63][10] = assign1360_e1880_d_n10;
        s.dn[63][11] = assign1360_e1880_d_n11;
        s.dn[63][12] = assign1360_e1880_d_n12;
        s.dn[63][13] = assign1360_e1880_d_n13;
        s.dn[63][14] = assign1360_e1880_d_n14;
        s.dn[63][15] = assign1360_e1880_d_n15;
        s.dn[63][16] = assign1360_e1880_d_n16;
        s.dn[63][17] = assign1360_e1880_d_n17;
        s.dn[63][18] = assign1360_e1880_d_n18;
        s.db[63][0] = assign1360_e1880_d_b0;
        s.db[63][1] = assign1360_e1880_d_b1;
        s.db[63][2] = assign1360_e1880_d_b2;
        s.db[63][3] = assign1360_e1880_d_b3;
        s.db[63][4] = assign1360_e1880_d_b4;
        s.db[63][5] = assign1360_e1880_d_b5;
        s.db[63][6] = assign1360_e1880_d_b6;
        s.db[63][7] = assign1360_e1880_d_b7;
        s.db[63][8] = assign1360_e1880_d_b8;
        s.db[63][9] = assign1360_e1880_d_b9;
        s.db[63][10] = assign1360_e1880_d_b10;
        s.db[63][11] = assign1360_e1880_d_b11;
        s.db[63][12] = assign1360_e1880_d_b12;
        s.db[63][13] = assign1360_e1880_d_b13;
        s.db[63][14] = assign1360_e1880_d_b14;
        s.db[63][15] = assign1360_e1880_d_b15;
        s.db[63][16] = assign1360_e1880_d_b16;
        s.db[63][17] = assign1360_e1880_d_b17;
        s.db[63][18] = assign1360_e1880_d_b18;
        s.rv[63] = 0.0;

        if (!s.b[112]) {
            s.store_scalar(24, { let limexp_arg = ((-p.p85) * p.p83); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        }

        if (!s.b[112]) {
            s.store_scalar(25, { let limexp_arg = ((-p.p85) * p.p84); if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } });
        }

        s.b[113] = (p.p5 == 1.0);
        s.store_scalar(113, if s.b[113] { 1.0 } else { 0.0 });

        if ((!s.b[112]) && s.b[113]) {
            s.store_tanh_ad(20, A::sub(s.ad_value(96), s.ad_value(57)));
            s.store_tanh_ad(22, A::sub(s.ad_value(97), s.ad_value(57)));
        }

        if ((!s.b[112]) && (!s.b[113])) {
            s.store_sub(20, 96, 57);
            s.store_sub(22, 97, 57);
        }

        if (!s.b[112]) {
            s.store_offset_scaled(21, 96, -1.0, (-p.p83));
            s.store_offset_scaled(23, 97, -1.0, (-p.p84));
        }

        s.store_sub_ad_lhs(8, A::limexp_scaled_input(s.ad_value(21), p.p85), 24);

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
    ) {
        let bi1 = ctx.branch_current(branches[1]);
        let assign1470_e1967: f64 = (s.v[19] * s.v[20]);
        let assign1470_e1968: f64 = { let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1470_e1971: f64 = (0.001 * p.p82);
        let assign1470_e1973: f64 = (assign1470_e1971 * s.v[8]);
        let assign1470_e1974: f64 = (assign1470_e1968 - assign1470_e1973);
        let assign1470_e1976: f64 = (assign1470_e1974 - s.v[63]);
        let assign1470_e1977: f64 = (p.p42 * assign1470_e1976);
        s.v[7] = assign1470_e1977;
        s.mark_derivatives_dirty(7);
        s.dn[7][0] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][0] * s.v[20]) + (s.v[19] * s.dn[20][0]))) - (assign1470_e1971 * s.dn[8][0])) - s.dn[63][0]));
        s.dn[7][1] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][1] * s.v[20]) + (s.v[19] * s.dn[20][1]))) - (assign1470_e1971 * s.dn[8][1])) - s.dn[63][1]));
        s.dn[7][2] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][2] * s.v[20]) + (s.v[19] * s.dn[20][2]))) - (assign1470_e1971 * s.dn[8][2])) - s.dn[63][2]));
        s.dn[7][3] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][3] * s.v[20]) + (s.v[19] * s.dn[20][3]))) - (assign1470_e1971 * s.dn[8][3])) - s.dn[63][3]));
        s.dn[7][4] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][4] * s.v[20]) + (s.v[19] * s.dn[20][4]))) - (assign1470_e1971 * s.dn[8][4])) - s.dn[63][4]));
        s.dn[7][5] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][5] * s.v[20]) + (s.v[19] * s.dn[20][5]))) - (assign1470_e1971 * s.dn[8][5])) - s.dn[63][5]));
        s.dn[7][6] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][6] * s.v[20]) + (s.v[19] * s.dn[20][6]))) - (assign1470_e1971 * s.dn[8][6])) - s.dn[63][6]));
        s.dn[7][7] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][7] * s.v[20]) + (s.v[19] * s.dn[20][7]))) - (assign1470_e1971 * s.dn[8][7])) - s.dn[63][7]));
        s.dn[7][8] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][8] * s.v[20]) + (s.v[19] * s.dn[20][8]))) - (assign1470_e1971 * s.dn[8][8])) - s.dn[63][8]));
        s.dn[7][9] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][9] * s.v[20]) + (s.v[19] * s.dn[20][9]))) - (assign1470_e1971 * s.dn[8][9])) - s.dn[63][9]));
        s.dn[7][10] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][10] * s.v[20]) + (s.v[19] * s.dn[20][10]))) - (assign1470_e1971 * s.dn[8][10])) - s.dn[63][10]));
        s.dn[7][11] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][11] * s.v[20]) + (s.v[19] * s.dn[20][11]))) - (assign1470_e1971 * s.dn[8][11])) - s.dn[63][11]));
        s.dn[7][12] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][12] * s.v[20]) + (s.v[19] * s.dn[20][12]))) - (assign1470_e1971 * s.dn[8][12])) - s.dn[63][12]));
        s.dn[7][13] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][13] * s.v[20]) + (s.v[19] * s.dn[20][13]))) - (assign1470_e1971 * s.dn[8][13])) - s.dn[63][13]));
        s.dn[7][14] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][14] * s.v[20]) + (s.v[19] * s.dn[20][14]))) - (assign1470_e1971 * s.dn[8][14])) - s.dn[63][14]));
        s.dn[7][15] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][15] * s.v[20]) + (s.v[19] * s.dn[20][15]))) - (assign1470_e1971 * s.dn[8][15])) - s.dn[63][15]));
        s.dn[7][16] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][16] * s.v[20]) + (s.v[19] * s.dn[20][16]))) - (assign1470_e1971 * s.dn[8][16])) - s.dn[63][16]));
        s.dn[7][17] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][17] * s.v[20]) + (s.v[19] * s.dn[20][17]))) - (assign1470_e1971 * s.dn[8][17])) - s.dn[63][17]));
        s.dn[7][18] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][18] * s.v[20]) + (s.v[19] * s.dn[20][18]))) - (assign1470_e1971 * s.dn[8][18])) - s.dn[63][18]));
        s.db[7][0] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][0] * s.v[20]) + (s.v[19] * s.db[20][0]))) - (assign1470_e1971 * s.db[8][0])) - s.db[63][0]));
        s.db[7][1] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][1] * s.v[20]) + (s.v[19] * s.db[20][1]))) - (assign1470_e1971 * s.db[8][1])) - s.db[63][1]));
        s.db[7][2] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][2] * s.v[20]) + (s.v[19] * s.db[20][2]))) - (assign1470_e1971 * s.db[8][2])) - s.db[63][2]));
        s.db[7][3] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][3] * s.v[20]) + (s.v[19] * s.db[20][3]))) - (assign1470_e1971 * s.db[8][3])) - s.db[63][3]));
        s.db[7][4] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][4] * s.v[20]) + (s.v[19] * s.db[20][4]))) - (assign1470_e1971 * s.db[8][4])) - s.db[63][4]));
        s.db[7][5] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][5] * s.v[20]) + (s.v[19] * s.db[20][5]))) - (assign1470_e1971 * s.db[8][5])) - s.db[63][5]));
        s.db[7][6] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][6] * s.v[20]) + (s.v[19] * s.db[20][6]))) - (assign1470_e1971 * s.db[8][6])) - s.db[63][6]));
        s.db[7][7] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][7] * s.v[20]) + (s.v[19] * s.db[20][7]))) - (assign1470_e1971 * s.db[8][7])) - s.db[63][7]));
        s.db[7][8] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][8] * s.v[20]) + (s.v[19] * s.db[20][8]))) - (assign1470_e1971 * s.db[8][8])) - s.db[63][8]));
        s.db[7][9] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][9] * s.v[20]) + (s.v[19] * s.db[20][9]))) - (assign1470_e1971 * s.db[8][9])) - s.db[63][9]));
        s.db[7][10] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][10] * s.v[20]) + (s.v[19] * s.db[20][10]))) - (assign1470_e1971 * s.db[8][10])) - s.db[63][10]));
        s.db[7][11] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][11] * s.v[20]) + (s.v[19] * s.db[20][11]))) - (assign1470_e1971 * s.db[8][11])) - s.db[63][11]));
        s.db[7][12] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][12] * s.v[20]) + (s.v[19] * s.db[20][12]))) - (assign1470_e1971 * s.db[8][12])) - s.db[63][12]));
        s.db[7][13] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][13] * s.v[20]) + (s.v[19] * s.db[20][13]))) - (assign1470_e1971 * s.db[8][13])) - s.db[63][13]));
        s.db[7][14] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][14] * s.v[20]) + (s.v[19] * s.db[20][14]))) - (assign1470_e1971 * s.db[8][14])) - s.db[63][14]));
        s.db[7][15] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][15] * s.v[20]) + (s.v[19] * s.db[20][15]))) - (assign1470_e1971 * s.db[8][15])) - s.db[63][15]));
        s.db[7][16] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][16] * s.v[20]) + (s.v[19] * s.db[20][16]))) - (assign1470_e1971 * s.db[8][16])) - s.db[63][16]));
        s.db[7][17] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][17] * s.v[20]) + (s.v[19] * s.db[20][17]))) - (assign1470_e1971 * s.db[8][17])) - s.db[63][17]));
        s.db[7][18] = (p.p42 * ((({ let limexp_arg = assign1470_e1967; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][18] * s.v[20]) + (s.v[19] * s.db[20][18]))) - (assign1470_e1971 * s.db[8][18])) - s.db[63][18]));
        s.rv[7] = 0.0;

        s.store_sub_ad_lhs(10, A::limexp_scaled_input(s.ad_value(23), p.p85), 25);

        let assign1490_e1987: f64 = (s.v[19] * s.v[22]);
        let assign1490_e1988: f64 = { let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1490_e1991: f64 = (0.001 * p.p82);
        let assign1490_e1993: f64 = (assign1490_e1991 * s.v[10]);
        let assign1490_e1994: f64 = (assign1490_e1988 - assign1490_e1993);
        let assign1490_e1996: f64 = (assign1490_e1994 - s.v[63]);
        let assign1490_e1997: f64 = (p.p42 * assign1490_e1996);
        s.v[9] = assign1490_e1997;
        s.mark_derivatives_dirty(9);
        s.dn[9][0] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][0] * s.v[22]) + (s.v[19] * s.dn[22][0]))) - (assign1490_e1991 * s.dn[10][0])) - s.dn[63][0]));
        s.dn[9][1] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][1] * s.v[22]) + (s.v[19] * s.dn[22][1]))) - (assign1490_e1991 * s.dn[10][1])) - s.dn[63][1]));
        s.dn[9][2] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][2] * s.v[22]) + (s.v[19] * s.dn[22][2]))) - (assign1490_e1991 * s.dn[10][2])) - s.dn[63][2]));
        s.dn[9][3] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][3] * s.v[22]) + (s.v[19] * s.dn[22][3]))) - (assign1490_e1991 * s.dn[10][3])) - s.dn[63][3]));
        s.dn[9][4] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][4] * s.v[22]) + (s.v[19] * s.dn[22][4]))) - (assign1490_e1991 * s.dn[10][4])) - s.dn[63][4]));
        s.dn[9][5] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][5] * s.v[22]) + (s.v[19] * s.dn[22][5]))) - (assign1490_e1991 * s.dn[10][5])) - s.dn[63][5]));
        s.dn[9][6] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][6] * s.v[22]) + (s.v[19] * s.dn[22][6]))) - (assign1490_e1991 * s.dn[10][6])) - s.dn[63][6]));
        s.dn[9][7] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][7] * s.v[22]) + (s.v[19] * s.dn[22][7]))) - (assign1490_e1991 * s.dn[10][7])) - s.dn[63][7]));
        s.dn[9][8] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][8] * s.v[22]) + (s.v[19] * s.dn[22][8]))) - (assign1490_e1991 * s.dn[10][8])) - s.dn[63][8]));
        s.dn[9][9] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][9] * s.v[22]) + (s.v[19] * s.dn[22][9]))) - (assign1490_e1991 * s.dn[10][9])) - s.dn[63][9]));
        s.dn[9][10] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][10] * s.v[22]) + (s.v[19] * s.dn[22][10]))) - (assign1490_e1991 * s.dn[10][10])) - s.dn[63][10]));
        s.dn[9][11] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][11] * s.v[22]) + (s.v[19] * s.dn[22][11]))) - (assign1490_e1991 * s.dn[10][11])) - s.dn[63][11]));
        s.dn[9][12] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][12] * s.v[22]) + (s.v[19] * s.dn[22][12]))) - (assign1490_e1991 * s.dn[10][12])) - s.dn[63][12]));
        s.dn[9][13] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][13] * s.v[22]) + (s.v[19] * s.dn[22][13]))) - (assign1490_e1991 * s.dn[10][13])) - s.dn[63][13]));
        s.dn[9][14] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][14] * s.v[22]) + (s.v[19] * s.dn[22][14]))) - (assign1490_e1991 * s.dn[10][14])) - s.dn[63][14]));
        s.dn[9][15] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][15] * s.v[22]) + (s.v[19] * s.dn[22][15]))) - (assign1490_e1991 * s.dn[10][15])) - s.dn[63][15]));
        s.dn[9][16] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][16] * s.v[22]) + (s.v[19] * s.dn[22][16]))) - (assign1490_e1991 * s.dn[10][16])) - s.dn[63][16]));
        s.dn[9][17] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][17] * s.v[22]) + (s.v[19] * s.dn[22][17]))) - (assign1490_e1991 * s.dn[10][17])) - s.dn[63][17]));
        s.dn[9][18] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.dn[19][18] * s.v[22]) + (s.v[19] * s.dn[22][18]))) - (assign1490_e1991 * s.dn[10][18])) - s.dn[63][18]));
        s.db[9][0] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][0] * s.v[22]) + (s.v[19] * s.db[22][0]))) - (assign1490_e1991 * s.db[10][0])) - s.db[63][0]));
        s.db[9][1] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][1] * s.v[22]) + (s.v[19] * s.db[22][1]))) - (assign1490_e1991 * s.db[10][1])) - s.db[63][1]));
        s.db[9][2] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][2] * s.v[22]) + (s.v[19] * s.db[22][2]))) - (assign1490_e1991 * s.db[10][2])) - s.db[63][2]));
        s.db[9][3] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][3] * s.v[22]) + (s.v[19] * s.db[22][3]))) - (assign1490_e1991 * s.db[10][3])) - s.db[63][3]));
        s.db[9][4] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][4] * s.v[22]) + (s.v[19] * s.db[22][4]))) - (assign1490_e1991 * s.db[10][4])) - s.db[63][4]));
        s.db[9][5] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][5] * s.v[22]) + (s.v[19] * s.db[22][5]))) - (assign1490_e1991 * s.db[10][5])) - s.db[63][5]));
        s.db[9][6] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][6] * s.v[22]) + (s.v[19] * s.db[22][6]))) - (assign1490_e1991 * s.db[10][6])) - s.db[63][6]));
        s.db[9][7] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][7] * s.v[22]) + (s.v[19] * s.db[22][7]))) - (assign1490_e1991 * s.db[10][7])) - s.db[63][7]));
        s.db[9][8] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][8] * s.v[22]) + (s.v[19] * s.db[22][8]))) - (assign1490_e1991 * s.db[10][8])) - s.db[63][8]));
        s.db[9][9] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][9] * s.v[22]) + (s.v[19] * s.db[22][9]))) - (assign1490_e1991 * s.db[10][9])) - s.db[63][9]));
        s.db[9][10] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][10] * s.v[22]) + (s.v[19] * s.db[22][10]))) - (assign1490_e1991 * s.db[10][10])) - s.db[63][10]));
        s.db[9][11] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][11] * s.v[22]) + (s.v[19] * s.db[22][11]))) - (assign1490_e1991 * s.db[10][11])) - s.db[63][11]));
        s.db[9][12] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][12] * s.v[22]) + (s.v[19] * s.db[22][12]))) - (assign1490_e1991 * s.db[10][12])) - s.db[63][12]));
        s.db[9][13] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][13] * s.v[22]) + (s.v[19] * s.db[22][13]))) - (assign1490_e1991 * s.db[10][13])) - s.db[63][13]));
        s.db[9][14] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][14] * s.v[22]) + (s.v[19] * s.db[22][14]))) - (assign1490_e1991 * s.db[10][14])) - s.db[63][14]));
        s.db[9][15] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][15] * s.v[22]) + (s.v[19] * s.db[22][15]))) - (assign1490_e1991 * s.db[10][15])) - s.db[63][15]));
        s.db[9][16] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][16] * s.v[22]) + (s.v[19] * s.db[22][16]))) - (assign1490_e1991 * s.db[10][16])) - s.db[63][16]));
        s.db[9][17] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][17] * s.v[22]) + (s.v[19] * s.db[22][17]))) - (assign1490_e1991 * s.db[10][17])) - s.db[63][17]));
        s.db[9][18] = (p.p42 * ((({ let limexp_arg = assign1490_e1987; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((s.db[19][18] * s.v[22]) + (s.v[19] * s.db[22][18]))) - (assign1490_e1991 * s.db[10][18])) - s.db[63][18]));
        s.rv[9] = 0.0;

        s.store_add_scaled_inputs3_indices(35, 55, 1.0, 96, p.p31, 5, p.p38);

        s.store_offset_tanh_ad(84, s.ad_value(35), 1.0);

        s.store_offset_scaled(36, 5, p.p33, p.p32);

        s.store_offset_tanh_ad(85, s.ad_value(36), 1.0);

        s.store_sub_from_scalar_scaled_input(37, p.p34, 5, p.p35);

        s.store_offset_tanh_ad(86, s.ad_value(37), ((1.0) + ((-p.p38))));

        s.store_add_scaled_inputs3_indices(38, 56, 1.0, 97, p.p37, 5, (-p.p38));

        s.store_offset_tanh_ad(87, s.ad_value(38), 1.0);

        s.b[114] = (p.p6 == 0.0);
        s.store_scalar(114, if s.b[114] { 1.0 } else { 0.0 });

        s.b[115] = (p.p6 == 1.0);
        s.store_scalar(115, if s.b[115] { 1.0 } else { 0.0 });

        s.b[116] = (p.p6 == 2.0);
        s.store_scalar(116, if s.b[116] { 1.0 } else { 0.0 });

        s.b[117] = (p.p6 == 3.0);
        s.store_scalar(117, if s.b[117] { 1.0 } else { 0.0 });

        s.b[118] = (p.p6 == 4.0);
        s.store_scalar(118, if s.b[118] { 1.0 } else { 0.0 });

        if s.b[114] {
            s.store_scalar(28, p.p25);
            s.store_scalar(29, p.p27);
        }

        if (s.b[115] && (!s.b[114])) {
            s.store_offset_product3(28, s.ad_value(44), s.ad_value(84), s.ad_value(85), 1.0, p.p25);
            s.store_offset_mul_offset_rhs_ad_rhs(29, 45, A::mul(s.ad_value(86), s.ad_value(87)), (2.0 * p.p38), p.p27);
        }

        if (s.b[116] && (!(s.b[114] || s.b[115]))) {
            s.store_offset(85, 85, (-p.p38));
            s.store_cosh_ad(88, A::add_scaled_inputs(s.ad_value(55), 1.0, s.ad_value(5), p.p38));
            s.store_ln(91, 88);
            s.store_cosh(89, 35);
            s.store_ln(90, 89);
            s.store_add_scaled_inputs3_indices(94, 55, 1.0, 5, p.p38, 91, 1.0);
            s.store_add_scaled_product_right_ad(26, 96, p.p25, 44, A::add_scaled_product(s.ad_value(96), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(35), 1.0, s.ad_value(90), 1.0, s.ad_value(94), -1.0), s.ad_value(85), 1.0 / (p.p31)), 1.0);
            s.store_cosh_ad(88, A::sub_scaled_inputs(s.ad_value(56), 1.0, s.ad_value(5), p.p38));
            s.store_ln(93, 88);
            s.store_cosh(89, 38);
            s.store_ln(92, 89);
            s.store_add_scaled_inputs3_indices(95, 56, 1.0, 5, (-p.p38), 93, 1.0);
            s.store_add_scaled_product_right_ad(27, 97, p.p27, 45, A::add_scaled_product(s.ad_value(97), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(38), 1.0, s.ad_value(92), 1.0, s.ad_value(95), -1.0), s.ad_value(86), 1.0 / (p.p37)), 1.0);
            s.store_scalar(28, A::ddx_projection(&s.ad_value(26), Some(11), None));
            s.store_scalar(29, A::ddx_projection(&s.ad_value(27), Some(10), None));
        }

        if (s.b[117] && (!((s.b[114] || s.b[115]) || s.b[116]))) {
            s.store_offset_scaled(30, 96, 1.0 / (p.p40), (-1.0));
            s.store_scalar(31, 0.5);
            s.store_mul_offset_rhs_ad(32, A::pow(A::offset(A::square(s.ad_value(30)), p.p41), A::sub_from_scalar((-1.0), s.ad_value(31))), A::mul_sub_from_scalar_lhs(1.0, A::scale(s.ad_value(31), 2.0), A::square(s.ad_value(30))), p.p41);
            s.store_offset_tanh_ad(84, A::add_scaled_inputs3(s.ad_value(55), 1.0, s.ad_value(96), p.p31, s.ad_value(5), (p.p38 * p.p31)), 1.0);
            s.store_offset_tanh_ad(85, A::scale_offset(s.ad_value(5), p.p33, p.p32), 1.0);
            s.store_offset_tanh_ad(86, A::sub_from_scalar(p.p34, A::scale(s.ad_value(5), p.p35)), (1.0 - p.p38));
            s.store_offset_tanh_ad(87, A::add_scaled_inputs3(s.ad_value(56), 1.0, s.ad_value(97), p.p37, s.ad_value(5), ((1.0 - p.p38) * p.p37)), 1.0);
            s.store_offset_product3(28, s.ad_value(44), A::add_scaled_inputs(s.ad_value(84), 1.0, s.ad_value(32), p.p39), s.ad_value(85), 1.0, p.p25);
            s.store_offset_mul_offset_rhs_ad_rhs(29, 45, A::mul(s.ad_value(86), s.ad_value(87)), (2.0 * p.p38), p.p27);
        }

        if (s.b[118] && (!(((s.b[114] || s.b[115]) || s.b[116]) || s.b[117]))) {
            s.store_cosh_ad(88, A::add_scaled_inputs(s.ad_value(55), 1.0, s.ad_value(5), p.p38));
            s.store_ln(91, 88);
            s.store_cosh(89, 35);
            s.store_ln(90, 89);
            s.store_scalar(31, 0.5);
            s.store_scaled_mul_ad(33, A::offset(s.ad_value(96), p.p40), A::pow(A::offset(A::square(A::scale_offset(s.ad_value(96), 1.0 / (p.p40), (-1.0))), p.p41), A::neg(s.ad_value(31))), p.p39);
            s.store_scale_ad(34, A::pow_from_scalar((p.p41 + 1.0), A::neg(s.ad_value(31))), (p.p39 * p.p40));
            s.store_add_scaled_inputs3_indices(94, 55, 1.0, 5, p.p38, 91, 1.0);
            s.store_add_scaled_product_right_ad(26, 96, p.p25, 44, A::add_scaled_offset_product_rhs(s.ad_value(96), (2.0 * p.p38), A::sub(A::add_scaled_inputs4(s.ad_value(35), 1.0, s.ad_value(90), 1.0, s.ad_value(94), -1.0, s.ad_value(33), 1.0), s.ad_value(34)), A::tanh(s.ad_value(36)), (1.0 - p.p38), 1.0 / (p.p31)), 1.0);
            s.store_cosh_ad(88, A::sub_scaled_inputs(s.ad_value(56), 1.0, s.ad_value(5), p.p38));
            s.store_ln(93, 88);
            s.store_cosh(89, 38);
            s.store_ln(92, 89);
            s.store_add_scaled_inputs3_indices(95, 56, 1.0, 5, (-p.p38), 93, 1.0);
            s.store_add_scaled_product_right_ad(27, 97, p.p27, 45, A::add_scaled_offset_product_rhs(s.ad_value(97), (2.0 * p.p38), A::add_scaled_inputs3(s.ad_value(38), 1.0, s.ad_value(92), 1.0, s.ad_value(95), -1.0), A::tanh(s.ad_value(37)), (1.0 - p.p38), 1.0 / (p.p37)), 1.0);
            s.store_scalar(28, A::ddx_projection(&s.ad_value(26), Some(11), None));
            s.store_scalar(29, A::ddx_projection(&s.ad_value(27), Some(10), None));
        }

        s.b[119] = ((p.p6 == 2.0) || (p.p6 == 4.0));
        s.store_scalar(119, if s.b[119] { 1.0 } else { 0.0 });

        let assign2090_e2833: f64 = (p.p55 * bi1);
        let assign2090_e2834_q: f64 = assign2090_e2833;
        s.v[63] = assign2090_e2833;
        s.mark_derivatives_dirty(63);
        s.db[63][1] = p.p55;
        s.rv[63] = assign2090_e2834_q;
        s.rdb[63][1] = p.p55;

        s.b[120] = (p.p58 > 0.0);
        s.store_scalar(120, if s.b[120] { 1.0 } else { 0.0 });

        s.b[121] = ((p.p63 > 0.0) || (p.p62 > 0.0));
        s.store_scalar(121, if s.b[121] { 1.0 } else { 0.0 });

        s.b[126] = (p.p50 > 0.0);
        s.store_scalar(126, if s.b[126] { 1.0 } else { 0.0 });

        s.b[127] = ((p.p47 > 0.0) || (p.p48 > 0.0));
        s.store_scalar(127, if s.b[127] { 1.0 } else { 0.0 });

        let assign2180_e2869: f64 = s.dn[98][12];
        s.store_scalar(99, assign2180_e2869);
        s.rv[99] = 0.0;

        let assign2190_e2874: f64 = (s.v[99] * p.p50);
        let assign2190_e2875: f64 = (1.0 + assign2190_e2874);
        let assign2190_e2876: f64 = (s.v[99] / assign2190_e2875);
        s.store_scalar(99, assign2190_e2876);
        s.rv[99] = 0.0;

        s.b[128] = (p.p7 == 0.0);
        s.store_scalar(128, if s.b[128] { 1.0 } else { 0.0 });

        s.b[129] = (p.p7 == 1.0);
        s.store_scalar(129, if s.b[129] { 1.0 } else { 0.0 });

        let (assign2240_e2896, assign2240_e2896_d_n0, assign2240_e2896_d_n1, assign2240_e2896_d_n2, assign2240_e2896_d_n3, assign2240_e2896_d_n4, assign2240_e2896_d_n5, assign2240_e2896_d_n6, assign2240_e2896_d_n7, assign2240_e2896_d_n8, assign2240_e2896_d_n9, assign2240_e2896_d_n10, assign2240_e2896_d_n11, assign2240_e2896_d_n12, assign2240_e2896_d_n13, assign2240_e2896_d_n14, assign2240_e2896_d_n15, assign2240_e2896_d_n16, assign2240_e2896_d_n17, assign2240_e2896_d_n18, assign2240_e2896_d_b0, assign2240_e2896_d_b1, assign2240_e2896_d_b2, assign2240_e2896_d_b3, assign2240_e2896_d_b4, assign2240_e2896_d_b5, assign2240_e2896_d_b6, assign2240_e2896_d_b7, assign2240_e2896_d_b8, assign2240_e2896_d_b9, assign2240_e2896_d_b10, assign2240_e2896_d_b11, assign2240_e2896_d_b12, assign2240_e2896_d_b13, assign2240_e2896_d_b14, assign2240_e2896_d_b15, assign2240_e2896_d_b16, assign2240_e2896_d_b17, assign2240_e2896_d_b18,) = {
    if s.b[128] {
        let assign2240_e2891: f64 = (s.v[18]).abs();
        let assign2240_e2893: f64 = (s.v[9]).abs();
        let assign2240_e2894: f64 = (assign2240_e2891 + assign2240_e2893);
        (assign2240_e2894, (if s.v[18] >= 0.0 { s.dn[18][0] } else { (-s.dn[18][0]) } + if s.v[9] >= 0.0 { s.dn[9][0] } else { (-s.dn[9][0]) }), (if s.v[18] >= 0.0 { s.dn[18][1] } else { (-s.dn[18][1]) } + if s.v[9] >= 0.0 { s.dn[9][1] } else { (-s.dn[9][1]) }), (if s.v[18] >= 0.0 { s.dn[18][2] } else { (-s.dn[18][2]) } + if s.v[9] >= 0.0 { s.dn[9][2] } else { (-s.dn[9][2]) }), (if s.v[18] >= 0.0 { s.dn[18][3] } else { (-s.dn[18][3]) } + if s.v[9] >= 0.0 { s.dn[9][3] } else { (-s.dn[9][3]) }), (if s.v[18] >= 0.0 { s.dn[18][4] } else { (-s.dn[18][4]) } + if s.v[9] >= 0.0 { s.dn[9][4] } else { (-s.dn[9][4]) }), (if s.v[18] >= 0.0 { s.dn[18][5] } else { (-s.dn[18][5]) } + if s.v[9] >= 0.0 { s.dn[9][5] } else { (-s.dn[9][5]) }), (if s.v[18] >= 0.0 { s.dn[18][6] } else { (-s.dn[18][6]) } + if s.v[9] >= 0.0 { s.dn[9][6] } else { (-s.dn[9][6]) }), (if s.v[18] >= 0.0 { s.dn[18][7] } else { (-s.dn[18][7]) } + if s.v[9] >= 0.0 { s.dn[9][7] } else { (-s.dn[9][7]) }), (if s.v[18] >= 0.0 { s.dn[18][8] } else { (-s.dn[18][8]) } + if s.v[9] >= 0.0 { s.dn[9][8] } else { (-s.dn[9][8]) }), (if s.v[18] >= 0.0 { s.dn[18][9] } else { (-s.dn[18][9]) } + if s.v[9] >= 0.0 { s.dn[9][9] } else { (-s.dn[9][9]) }), (if s.v[18] >= 0.0 { s.dn[18][10] } else { (-s.dn[18][10]) } + if s.v[9] >= 0.0 { s.dn[9][10] } else { (-s.dn[9][10]) }), (if s.v[18] >= 0.0 { s.dn[18][11] } else { (-s.dn[18][11]) } + if s.v[9] >= 0.0 { s.dn[9][11] } else { (-s.dn[9][11]) }), (if s.v[18] >= 0.0 { s.dn[18][12] } else { (-s.dn[18][12]) } + if s.v[9] >= 0.0 { s.dn[9][12] } else { (-s.dn[9][12]) }), (if s.v[18] >= 0.0 { s.dn[18][13] } else { (-s.dn[18][13]) } + if s.v[9] >= 0.0 { s.dn[9][13] } else { (-s.dn[9][13]) }), (if s.v[18] >= 0.0 { s.dn[18][14] } else { (-s.dn[18][14]) } + if s.v[9] >= 0.0 { s.dn[9][14] } else { (-s.dn[9][14]) }), (if s.v[18] >= 0.0 { s.dn[18][15] } else { (-s.dn[18][15]) } + if s.v[9] >= 0.0 { s.dn[9][15] } else { (-s.dn[9][15]) }), (if s.v[18] >= 0.0 { s.dn[18][16] } else { (-s.dn[18][16]) } + if s.v[9] >= 0.0 { s.dn[9][16] } else { (-s.dn[9][16]) }), (if s.v[18] >= 0.0 { s.dn[18][17] } else { (-s.dn[18][17]) } + if s.v[9] >= 0.0 { s.dn[9][17] } else { (-s.dn[9][17]) }), (if s.v[18] >= 0.0 { s.dn[18][18] } else { (-s.dn[18][18]) } + if s.v[9] >= 0.0 { s.dn[9][18] } else { (-s.dn[9][18]) }), (if s.v[18] >= 0.0 { s.db[18][0] } else { (-s.db[18][0]) } + if s.v[9] >= 0.0 { s.db[9][0] } else { (-s.db[9][0]) }), (if s.v[18] >= 0.0 { s.db[18][1] } else { (-s.db[18][1]) } + if s.v[9] >= 0.0 { s.db[9][1] } else { (-s.db[9][1]) }), (if s.v[18] >= 0.0 { s.db[18][2] } else { (-s.db[18][2]) } + if s.v[9] >= 0.0 { s.db[9][2] } else { (-s.db[9][2]) }), (if s.v[18] >= 0.0 { s.db[18][3] } else { (-s.db[18][3]) } + if s.v[9] >= 0.0 { s.db[9][3] } else { (-s.db[9][3]) }), (if s.v[18] >= 0.0 { s.db[18][4] } else { (-s.db[18][4]) } + if s.v[9] >= 0.0 { s.db[9][4] } else { (-s.db[9][4]) }), (if s.v[18] >= 0.0 { s.db[18][5] } else { (-s.db[18][5]) } + if s.v[9] >= 0.0 { s.db[9][5] } else { (-s.db[9][5]) }), (if s.v[18] >= 0.0 { s.db[18][6] } else { (-s.db[18][6]) } + if s.v[9] >= 0.0 { s.db[9][6] } else { (-s.db[9][6]) }), (if s.v[18] >= 0.0 { s.db[18][7] } else { (-s.db[18][7]) } + if s.v[9] >= 0.0 { s.db[9][7] } else { (-s.db[9][7]) }), (if s.v[18] >= 0.0 { s.db[18][8] } else { (-s.db[18][8]) } + if s.v[9] >= 0.0 { s.db[9][8] } else { (-s.db[9][8]) }), (if s.v[18] >= 0.0 { s.db[18][9] } else { (-s.db[18][9]) } + if s.v[9] >= 0.0 { s.db[9][9] } else { (-s.db[9][9]) }), (if s.v[18] >= 0.0 { s.db[18][10] } else { (-s.db[18][10]) } + if s.v[9] >= 0.0 { s.db[9][10] } else { (-s.db[9][10]) }), (if s.v[18] >= 0.0 { s.db[18][11] } else { (-s.db[18][11]) } + if s.v[9] >= 0.0 { s.db[9][11] } else { (-s.db[9][11]) }), (if s.v[18] >= 0.0 { s.db[18][12] } else { (-s.db[18][12]) } + if s.v[9] >= 0.0 { s.db[9][12] } else { (-s.db[9][12]) }), (if s.v[18] >= 0.0 { s.db[18][13] } else { (-s.db[18][13]) } + if s.v[9] >= 0.0 { s.db[9][13] } else { (-s.db[9][13]) }), (if s.v[18] >= 0.0 { s.db[18][14] } else { (-s.db[18][14]) } + if s.v[9] >= 0.0 { s.db[9][14] } else { (-s.db[9][14]) }), (if s.v[18] >= 0.0 { s.db[18][15] } else { (-s.db[18][15]) } + if s.v[9] >= 0.0 { s.db[9][15] } else { (-s.db[9][15]) }), (if s.v[18] >= 0.0 { s.db[18][16] } else { (-s.db[18][16]) } + if s.v[9] >= 0.0 { s.db[9][16] } else { (-s.db[9][16]) }), (if s.v[18] >= 0.0 { s.db[18][17] } else { (-s.db[18][17]) } + if s.v[9] >= 0.0 { s.db[9][17] } else { (-s.db[9][17]) }), (if s.v[18] >= 0.0 { s.db[18][18] } else { (-s.db[18][18]) } + if s.v[9] >= 0.0 { s.db[9][18] } else { (-s.db[9][18]) }),)
    } else {
        (s.v[132], s.dn[132][0], s.dn[132][1], s.dn[132][2], s.dn[132][3], s.dn[132][4], s.dn[132][5], s.dn[132][6], s.dn[132][7], s.dn[132][8], s.dn[132][9], s.dn[132][10], s.dn[132][11], s.dn[132][12], s.dn[132][13], s.dn[132][14], s.dn[132][15], s.dn[132][16], s.dn[132][17], s.dn[132][18], s.db[132][0], s.db[132][1], s.db[132][2], s.db[132][3], s.db[132][4], s.db[132][5], s.db[132][6], s.db[132][7], s.db[132][8], s.db[132][9], s.db[132][10], s.db[132][11], s.db[132][12], s.db[132][13], s.db[132][14], s.db[132][15], s.db[132][16], s.db[132][17], s.db[132][18],)
    }
};
        s.v[132] = assign2240_e2896;
        s.mark_derivatives_dirty(132);
        s.dn[132][0] = assign2240_e2896_d_n0;
        s.dn[132][1] = assign2240_e2896_d_n1;
        s.dn[132][2] = assign2240_e2896_d_n2;
        s.dn[132][3] = assign2240_e2896_d_n3;
        s.dn[132][4] = assign2240_e2896_d_n4;
        s.dn[132][5] = assign2240_e2896_d_n5;
        s.dn[132][6] = assign2240_e2896_d_n6;
        s.dn[132][7] = assign2240_e2896_d_n7;
        s.dn[132][8] = assign2240_e2896_d_n8;
        s.dn[132][9] = assign2240_e2896_d_n9;
        s.dn[132][10] = assign2240_e2896_d_n10;
        s.dn[132][11] = assign2240_e2896_d_n11;
        s.dn[132][12] = assign2240_e2896_d_n12;
        s.dn[132][13] = assign2240_e2896_d_n13;
        s.dn[132][14] = assign2240_e2896_d_n14;
        s.dn[132][15] = assign2240_e2896_d_n15;
        s.dn[132][16] = assign2240_e2896_d_n16;
        s.dn[132][17] = assign2240_e2896_d_n17;
        s.dn[132][18] = assign2240_e2896_d_n18;
        s.db[132][0] = assign2240_e2896_d_b0;
        s.db[132][1] = assign2240_e2896_d_b1;
        s.db[132][2] = assign2240_e2896_d_b2;
        s.db[132][3] = assign2240_e2896_d_b3;
        s.db[132][4] = assign2240_e2896_d_b4;
        s.db[132][5] = assign2240_e2896_d_b5;
        s.db[132][6] = assign2240_e2896_d_b6;
        s.db[132][7] = assign2240_e2896_d_b7;
        s.db[132][8] = assign2240_e2896_d_b8;
        s.db[132][9] = assign2240_e2896_d_b9;
        s.db[132][10] = assign2240_e2896_d_b10;
        s.db[132][11] = assign2240_e2896_d_b11;
        s.db[132][12] = assign2240_e2896_d_b12;
        s.db[132][13] = assign2240_e2896_d_b13;
        s.db[132][14] = assign2240_e2896_d_b14;
        s.db[132][15] = assign2240_e2896_d_b15;
        s.db[132][16] = assign2240_e2896_d_b16;
        s.db[132][17] = assign2240_e2896_d_b17;
        s.db[132][18] = assign2240_e2896_d_b18;
        s.rv[132] = 0.0;

        let (assign2250_e2917, assign2250_e2917_d_n0, assign2250_e2917_d_n1, assign2250_e2917_d_n2, assign2250_e2917_d_n3, assign2250_e2917_d_n4, assign2250_e2917_d_n5, assign2250_e2917_d_n6, assign2250_e2917_d_n7, assign2250_e2917_d_n8, assign2250_e2917_d_n9, assign2250_e2917_d_n10, assign2250_e2917_d_n11, assign2250_e2917_d_n12, assign2250_e2917_d_n13, assign2250_e2917_d_n14, assign2250_e2917_d_n15, assign2250_e2917_d_n16, assign2250_e2917_d_n17, assign2250_e2917_d_n18, assign2250_e2917_d_b0, assign2250_e2917_d_b1, assign2250_e2917_d_b2, assign2250_e2917_d_b3, assign2250_e2917_d_b4, assign2250_e2917_d_b5, assign2250_e2917_d_b6, assign2250_e2917_d_b7, assign2250_e2917_d_b8, assign2250_e2917_d_b9, assign2250_e2917_d_b10, assign2250_e2917_d_b11, assign2250_e2917_d_b12, assign2250_e2917_d_b13, assign2250_e2917_d_b14, assign2250_e2917_d_b15, assign2250_e2917_d_b16, assign2250_e2917_d_b17, assign2250_e2917_d_b18,) = {
    if s.b[128] {
        let assign2250_e2900: f64 = (p.p93 + 273.15);
        let assign2250_e2904: f64 = (p.p95 * s.v[75]);
        let assign2250_e2906: f64 = (s.v[79]).abs();
        let assign2250_e2907: f64 = (assign2250_e2904 * assign2250_e2906);
        let assign2250_e2911: f64 = (p.p16 * s.v[5]);
        let assign2250_e2912: f64 = (1.0 + assign2250_e2911);
        let assign2250_e2913: f64 = (assign2250_e2907 * assign2250_e2912);
        let assign2250_e2914: f64 = (1.0 + assign2250_e2913);
        let assign2250_e2915: f64 = (assign2250_e2900 * assign2250_e2914);
        (assign2250_e2915, (assign2250_e2900 * (((((p.p95 * s.dn[75][0]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][0] } else { (-s.dn[79][0]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][0])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][1]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][1] } else { (-s.dn[79][1]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][1])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][2]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][2] } else { (-s.dn[79][2]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][2])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][3]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][3] } else { (-s.dn[79][3]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][3])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][4]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][4] } else { (-s.dn[79][4]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][4])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][5]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][5] } else { (-s.dn[79][5]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][5])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][6]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][6] } else { (-s.dn[79][6]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][6])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][7]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][7] } else { (-s.dn[79][7]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][7])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][8]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][8] } else { (-s.dn[79][8]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][8])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][9]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][9] } else { (-s.dn[79][9]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][9])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][10]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][10] } else { (-s.dn[79][10]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][10])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][11]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][11] } else { (-s.dn[79][11]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][11])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][12]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][12] } else { (-s.dn[79][12]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][12])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][13]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][13] } else { (-s.dn[79][13]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][13])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][14]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][14] } else { (-s.dn[79][14]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][14])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][15]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][15] } else { (-s.dn[79][15]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][15])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][16]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][16] } else { (-s.dn[79][16]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][16])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][17]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][17] } else { (-s.dn[79][17]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][17])))), (assign2250_e2900 * (((((p.p95 * s.dn[75][18]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.dn[79][18] } else { (-s.dn[79][18]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.dn[5][18])))), (assign2250_e2900 * (((((p.p95 * s.db[75][0]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][0] } else { (-s.db[79][0]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][0])))), (assign2250_e2900 * (((((p.p95 * s.db[75][1]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][1] } else { (-s.db[79][1]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][1])))), (assign2250_e2900 * (((((p.p95 * s.db[75][2]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][2] } else { (-s.db[79][2]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][2])))), (assign2250_e2900 * (((((p.p95 * s.db[75][3]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][3] } else { (-s.db[79][3]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][3])))), (assign2250_e2900 * (((((p.p95 * s.db[75][4]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][4] } else { (-s.db[79][4]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][4])))), (assign2250_e2900 * (((((p.p95 * s.db[75][5]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][5] } else { (-s.db[79][5]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][5])))), (assign2250_e2900 * (((((p.p95 * s.db[75][6]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][6] } else { (-s.db[79][6]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][6])))), (assign2250_e2900 * (((((p.p95 * s.db[75][7]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][7] } else { (-s.db[79][7]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][7])))), (assign2250_e2900 * (((((p.p95 * s.db[75][8]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][8] } else { (-s.db[79][8]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][8])))), (assign2250_e2900 * (((((p.p95 * s.db[75][9]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][9] } else { (-s.db[79][9]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][9])))), (assign2250_e2900 * (((((p.p95 * s.db[75][10]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][10] } else { (-s.db[79][10]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][10])))), (assign2250_e2900 * (((((p.p95 * s.db[75][11]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][11] } else { (-s.db[79][11]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][11])))), (assign2250_e2900 * (((((p.p95 * s.db[75][12]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][12] } else { (-s.db[79][12]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][12])))), (assign2250_e2900 * (((((p.p95 * s.db[75][13]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][13] } else { (-s.db[79][13]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][13])))), (assign2250_e2900 * (((((p.p95 * s.db[75][14]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][14] } else { (-s.db[79][14]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][14])))), (assign2250_e2900 * (((((p.p95 * s.db[75][15]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][15] } else { (-s.db[79][15]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][15])))), (assign2250_e2900 * (((((p.p95 * s.db[75][16]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][16] } else { (-s.db[79][16]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][16])))), (assign2250_e2900 * (((((p.p95 * s.db[75][17]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][17] } else { (-s.db[79][17]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][17])))), (assign2250_e2900 * (((((p.p95 * s.db[75][18]) * assign2250_e2906) + (assign2250_e2904 * if s.v[79] >= 0.0 { s.db[79][18] } else { (-s.db[79][18]) })) * assign2250_e2912) + (assign2250_e2907 * (p.p16 * s.db[5][18])))),)
    } else {
        (s.v[133], s.dn[133][0], s.dn[133][1], s.dn[133][2], s.dn[133][3], s.dn[133][4], s.dn[133][5], s.dn[133][6], s.dn[133][7], s.dn[133][8], s.dn[133][9], s.dn[133][10], s.dn[133][11], s.dn[133][12], s.dn[133][13], s.dn[133][14], s.dn[133][15], s.dn[133][16], s.dn[133][17], s.dn[133][18], s.db[133][0], s.db[133][1], s.db[133][2], s.db[133][3], s.db[133][4], s.db[133][5], s.db[133][6], s.db[133][7], s.db[133][8], s.db[133][9], s.db[133][10], s.db[133][11], s.db[133][12], s.db[133][13], s.db[133][14], s.db[133][15], s.db[133][16], s.db[133][17], s.db[133][18],)
    }
};
        s.v[133] = assign2250_e2917;
        s.mark_derivatives_dirty(133);
        s.dn[133][0] = assign2250_e2917_d_n0;
        s.dn[133][1] = assign2250_e2917_d_n1;
        s.dn[133][2] = assign2250_e2917_d_n2;
        s.dn[133][3] = assign2250_e2917_d_n3;
        s.dn[133][4] = assign2250_e2917_d_n4;
        s.dn[133][5] = assign2250_e2917_d_n5;
        s.dn[133][6] = assign2250_e2917_d_n6;
        s.dn[133][7] = assign2250_e2917_d_n7;
        s.dn[133][8] = assign2250_e2917_d_n8;
        s.dn[133][9] = assign2250_e2917_d_n9;
        s.dn[133][10] = assign2250_e2917_d_n10;
        s.dn[133][11] = assign2250_e2917_d_n11;
        s.dn[133][12] = assign2250_e2917_d_n12;
        s.dn[133][13] = assign2250_e2917_d_n13;
        s.dn[133][14] = assign2250_e2917_d_n14;
        s.dn[133][15] = assign2250_e2917_d_n15;
        s.dn[133][16] = assign2250_e2917_d_n16;
        s.dn[133][17] = assign2250_e2917_d_n17;
        s.dn[133][18] = assign2250_e2917_d_n18;
        s.db[133][0] = assign2250_e2917_d_b0;
        s.db[133][1] = assign2250_e2917_d_b1;
        s.db[133][2] = assign2250_e2917_d_b2;
        s.db[133][3] = assign2250_e2917_d_b3;
        s.db[133][4] = assign2250_e2917_d_b4;
        s.db[133][5] = assign2250_e2917_d_b5;
        s.db[133][6] = assign2250_e2917_d_b6;
        s.db[133][7] = assign2250_e2917_d_b7;
        s.db[133][8] = assign2250_e2917_d_b8;
        s.db[133][9] = assign2250_e2917_d_b9;
        s.db[133][10] = assign2250_e2917_d_b10;
        s.db[133][11] = assign2250_e2917_d_b11;
        s.db[133][12] = assign2250_e2917_d_b12;
        s.db[133][13] = assign2250_e2917_d_b13;
        s.db[133][14] = assign2250_e2917_d_b14;
        s.db[133][15] = assign2250_e2917_d_b15;
        s.db[133][16] = assign2250_e2917_d_b16;
        s.db[133][17] = assign2250_e2917_d_b17;
        s.db[133][18] = assign2250_e2917_d_b18;
        s.rv[133] = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let (assign2260_e2941, assign2260_e2941_d_n0, assign2260_e2941_d_n1, assign2260_e2941_d_n2, assign2260_e2941_d_n3, assign2260_e2941_d_n4, assign2260_e2941_d_n5, assign2260_e2941_d_n6, assign2260_e2941_d_n7, assign2260_e2941_d_n8, assign2260_e2941_d_n9, assign2260_e2941_d_n10, assign2260_e2941_d_n11, assign2260_e2941_d_n12, assign2260_e2941_d_n13, assign2260_e2941_d_n14, assign2260_e2941_d_n15, assign2260_e2941_d_n16, assign2260_e2941_d_n17, assign2260_e2941_d_n18, assign2260_e2941_d_b0, assign2260_e2941_d_b1, assign2260_e2941_d_b2, assign2260_e2941_d_b3, assign2260_e2941_d_b4, assign2260_e2941_d_b5, assign2260_e2941_d_b6, assign2260_e2941_d_b7, assign2260_e2941_d_b8, assign2260_e2941_d_b9, assign2260_e2941_d_b10, assign2260_e2941_d_b11, assign2260_e2941_d_b12, assign2260_e2941_d_b13, assign2260_e2941_d_b14, assign2260_e2941_d_b15, assign2260_e2941_d_b16, assign2260_e2941_d_b17, assign2260_e2941_d_b18,) = {
    if s.b[128] {
        let assign2260_e2921: f64 = (p.p99 * 4.0);
        let assign2260_e2923: f64 = (assign2260_e2921 * 1.3806503e-23);
        let assign2260_e2925: f64 = (assign2260_e2923 * s.v[15]);
        let assign2260_e2928: f64 = (s.v[133] / s.v[15]);
        let assign2260_e2930: f64 = (assign2260_e2928 * s.v[132]);
        let assign2260_e2933: f64 = (p.p94 * s.v[132]);
        let assign2260_e2935: f64 = (assign2260_e2933 * s.v[132]);
        let assign2260_e2936: f64 = (assign2260_e2930 + assign2260_e2935);
        let assign2260_e2937: f64 = (assign2260_e2936).abs();
        let assign2260_e2938: f64 = (assign2260_e2937).sqrt();
        let assign2260_e2939: f64 = (assign2260_e2925 * assign2260_e2938);
        (assign2260_e2939, (((assign2260_e2923 * s.dn[15][0]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][0] * s.v[15]) - (s.v[133] * s.dn[15][0])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][0])) + (((p.p94 * s.dn[132][0]) * s.v[132]) + (assign2260_e2933 * s.dn[132][0]))) } else { (-((((((s.dn[133][0] * s.v[15]) - (s.v[133] * s.dn[15][0])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][0])) + (((p.p94 * s.dn[132][0]) * s.v[132]) + (assign2260_e2933 * s.dn[132][0])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][1]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][1] * s.v[15]) - (s.v[133] * s.dn[15][1])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][1])) + (((p.p94 * s.dn[132][1]) * s.v[132]) + (assign2260_e2933 * s.dn[132][1]))) } else { (-((((((s.dn[133][1] * s.v[15]) - (s.v[133] * s.dn[15][1])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][1])) + (((p.p94 * s.dn[132][1]) * s.v[132]) + (assign2260_e2933 * s.dn[132][1])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][2]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][2] * s.v[15]) - (s.v[133] * s.dn[15][2])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][2])) + (((p.p94 * s.dn[132][2]) * s.v[132]) + (assign2260_e2933 * s.dn[132][2]))) } else { (-((((((s.dn[133][2] * s.v[15]) - (s.v[133] * s.dn[15][2])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][2])) + (((p.p94 * s.dn[132][2]) * s.v[132]) + (assign2260_e2933 * s.dn[132][2])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][3]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][3] * s.v[15]) - (s.v[133] * s.dn[15][3])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][3])) + (((p.p94 * s.dn[132][3]) * s.v[132]) + (assign2260_e2933 * s.dn[132][3]))) } else { (-((((((s.dn[133][3] * s.v[15]) - (s.v[133] * s.dn[15][3])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][3])) + (((p.p94 * s.dn[132][3]) * s.v[132]) + (assign2260_e2933 * s.dn[132][3])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][4]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][4] * s.v[15]) - (s.v[133] * s.dn[15][4])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][4])) + (((p.p94 * s.dn[132][4]) * s.v[132]) + (assign2260_e2933 * s.dn[132][4]))) } else { (-((((((s.dn[133][4] * s.v[15]) - (s.v[133] * s.dn[15][4])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][4])) + (((p.p94 * s.dn[132][4]) * s.v[132]) + (assign2260_e2933 * s.dn[132][4])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][5]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][5] * s.v[15]) - (s.v[133] * s.dn[15][5])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][5])) + (((p.p94 * s.dn[132][5]) * s.v[132]) + (assign2260_e2933 * s.dn[132][5]))) } else { (-((((((s.dn[133][5] * s.v[15]) - (s.v[133] * s.dn[15][5])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][5])) + (((p.p94 * s.dn[132][5]) * s.v[132]) + (assign2260_e2933 * s.dn[132][5])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][6]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][6] * s.v[15]) - (s.v[133] * s.dn[15][6])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][6])) + (((p.p94 * s.dn[132][6]) * s.v[132]) + (assign2260_e2933 * s.dn[132][6]))) } else { (-((((((s.dn[133][6] * s.v[15]) - (s.v[133] * s.dn[15][6])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][6])) + (((p.p94 * s.dn[132][6]) * s.v[132]) + (assign2260_e2933 * s.dn[132][6])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][7]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][7] * s.v[15]) - (s.v[133] * s.dn[15][7])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][7])) + (((p.p94 * s.dn[132][7]) * s.v[132]) + (assign2260_e2933 * s.dn[132][7]))) } else { (-((((((s.dn[133][7] * s.v[15]) - (s.v[133] * s.dn[15][7])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][7])) + (((p.p94 * s.dn[132][7]) * s.v[132]) + (assign2260_e2933 * s.dn[132][7])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][8]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][8] * s.v[15]) - (s.v[133] * s.dn[15][8])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][8])) + (((p.p94 * s.dn[132][8]) * s.v[132]) + (assign2260_e2933 * s.dn[132][8]))) } else { (-((((((s.dn[133][8] * s.v[15]) - (s.v[133] * s.dn[15][8])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][8])) + (((p.p94 * s.dn[132][8]) * s.v[132]) + (assign2260_e2933 * s.dn[132][8])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][9]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][9] * s.v[15]) - (s.v[133] * s.dn[15][9])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][9])) + (((p.p94 * s.dn[132][9]) * s.v[132]) + (assign2260_e2933 * s.dn[132][9]))) } else { (-((((((s.dn[133][9] * s.v[15]) - (s.v[133] * s.dn[15][9])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][9])) + (((p.p94 * s.dn[132][9]) * s.v[132]) + (assign2260_e2933 * s.dn[132][9])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][10]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][10] * s.v[15]) - (s.v[133] * s.dn[15][10])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][10])) + (((p.p94 * s.dn[132][10]) * s.v[132]) + (assign2260_e2933 * s.dn[132][10]))) } else { (-((((((s.dn[133][10] * s.v[15]) - (s.v[133] * s.dn[15][10])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][10])) + (((p.p94 * s.dn[132][10]) * s.v[132]) + (assign2260_e2933 * s.dn[132][10])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][11]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][11] * s.v[15]) - (s.v[133] * s.dn[15][11])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][11])) + (((p.p94 * s.dn[132][11]) * s.v[132]) + (assign2260_e2933 * s.dn[132][11]))) } else { (-((((((s.dn[133][11] * s.v[15]) - (s.v[133] * s.dn[15][11])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][11])) + (((p.p94 * s.dn[132][11]) * s.v[132]) + (assign2260_e2933 * s.dn[132][11])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][12]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][12] * s.v[15]) - (s.v[133] * s.dn[15][12])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][12])) + (((p.p94 * s.dn[132][12]) * s.v[132]) + (assign2260_e2933 * s.dn[132][12]))) } else { (-((((((s.dn[133][12] * s.v[15]) - (s.v[133] * s.dn[15][12])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][12])) + (((p.p94 * s.dn[132][12]) * s.v[132]) + (assign2260_e2933 * s.dn[132][12])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][13]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][13] * s.v[15]) - (s.v[133] * s.dn[15][13])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][13])) + (((p.p94 * s.dn[132][13]) * s.v[132]) + (assign2260_e2933 * s.dn[132][13]))) } else { (-((((((s.dn[133][13] * s.v[15]) - (s.v[133] * s.dn[15][13])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][13])) + (((p.p94 * s.dn[132][13]) * s.v[132]) + (assign2260_e2933 * s.dn[132][13])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][14]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][14] * s.v[15]) - (s.v[133] * s.dn[15][14])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][14])) + (((p.p94 * s.dn[132][14]) * s.v[132]) + (assign2260_e2933 * s.dn[132][14]))) } else { (-((((((s.dn[133][14] * s.v[15]) - (s.v[133] * s.dn[15][14])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][14])) + (((p.p94 * s.dn[132][14]) * s.v[132]) + (assign2260_e2933 * s.dn[132][14])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][15]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][15] * s.v[15]) - (s.v[133] * s.dn[15][15])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][15])) + (((p.p94 * s.dn[132][15]) * s.v[132]) + (assign2260_e2933 * s.dn[132][15]))) } else { (-((((((s.dn[133][15] * s.v[15]) - (s.v[133] * s.dn[15][15])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][15])) + (((p.p94 * s.dn[132][15]) * s.v[132]) + (assign2260_e2933 * s.dn[132][15])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][16]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][16] * s.v[15]) - (s.v[133] * s.dn[15][16])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][16])) + (((p.p94 * s.dn[132][16]) * s.v[132]) + (assign2260_e2933 * s.dn[132][16]))) } else { (-((((((s.dn[133][16] * s.v[15]) - (s.v[133] * s.dn[15][16])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][16])) + (((p.p94 * s.dn[132][16]) * s.v[132]) + (assign2260_e2933 * s.dn[132][16])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][17]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][17] * s.v[15]) - (s.v[133] * s.dn[15][17])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][17])) + (((p.p94 * s.dn[132][17]) * s.v[132]) + (assign2260_e2933 * s.dn[132][17]))) } else { (-((((((s.dn[133][17] * s.v[15]) - (s.v[133] * s.dn[15][17])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][17])) + (((p.p94 * s.dn[132][17]) * s.v[132]) + (assign2260_e2933 * s.dn[132][17])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.dn[15][18]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.dn[133][18] * s.v[15]) - (s.v[133] * s.dn[15][18])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][18])) + (((p.p94 * s.dn[132][18]) * s.v[132]) + (assign2260_e2933 * s.dn[132][18]))) } else { (-((((((s.dn[133][18] * s.v[15]) - (s.v[133] * s.dn[15][18])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.dn[132][18])) + (((p.p94 * s.dn[132][18]) * s.v[132]) + (assign2260_e2933 * s.dn[132][18])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][0]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][0] * s.v[15]) - (s.v[133] * s.db[15][0])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][0])) + (((p.p94 * s.db[132][0]) * s.v[132]) + (assign2260_e2933 * s.db[132][0]))) } else { (-((((((s.db[133][0] * s.v[15]) - (s.v[133] * s.db[15][0])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][0])) + (((p.p94 * s.db[132][0]) * s.v[132]) + (assign2260_e2933 * s.db[132][0])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][1]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][1] * s.v[15]) - (s.v[133] * s.db[15][1])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][1])) + (((p.p94 * s.db[132][1]) * s.v[132]) + (assign2260_e2933 * s.db[132][1]))) } else { (-((((((s.db[133][1] * s.v[15]) - (s.v[133] * s.db[15][1])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][1])) + (((p.p94 * s.db[132][1]) * s.v[132]) + (assign2260_e2933 * s.db[132][1])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][2]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][2] * s.v[15]) - (s.v[133] * s.db[15][2])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][2])) + (((p.p94 * s.db[132][2]) * s.v[132]) + (assign2260_e2933 * s.db[132][2]))) } else { (-((((((s.db[133][2] * s.v[15]) - (s.v[133] * s.db[15][2])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][2])) + (((p.p94 * s.db[132][2]) * s.v[132]) + (assign2260_e2933 * s.db[132][2])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][3]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][3] * s.v[15]) - (s.v[133] * s.db[15][3])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][3])) + (((p.p94 * s.db[132][3]) * s.v[132]) + (assign2260_e2933 * s.db[132][3]))) } else { (-((((((s.db[133][3] * s.v[15]) - (s.v[133] * s.db[15][3])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][3])) + (((p.p94 * s.db[132][3]) * s.v[132]) + (assign2260_e2933 * s.db[132][3])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][4]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][4] * s.v[15]) - (s.v[133] * s.db[15][4])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][4])) + (((p.p94 * s.db[132][4]) * s.v[132]) + (assign2260_e2933 * s.db[132][4]))) } else { (-((((((s.db[133][4] * s.v[15]) - (s.v[133] * s.db[15][4])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][4])) + (((p.p94 * s.db[132][4]) * s.v[132]) + (assign2260_e2933 * s.db[132][4])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][5]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][5] * s.v[15]) - (s.v[133] * s.db[15][5])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][5])) + (((p.p94 * s.db[132][5]) * s.v[132]) + (assign2260_e2933 * s.db[132][5]))) } else { (-((((((s.db[133][5] * s.v[15]) - (s.v[133] * s.db[15][5])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][5])) + (((p.p94 * s.db[132][5]) * s.v[132]) + (assign2260_e2933 * s.db[132][5])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][6]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][6] * s.v[15]) - (s.v[133] * s.db[15][6])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][6])) + (((p.p94 * s.db[132][6]) * s.v[132]) + (assign2260_e2933 * s.db[132][6]))) } else { (-((((((s.db[133][6] * s.v[15]) - (s.v[133] * s.db[15][6])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][6])) + (((p.p94 * s.db[132][6]) * s.v[132]) + (assign2260_e2933 * s.db[132][6])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][7]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][7] * s.v[15]) - (s.v[133] * s.db[15][7])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][7])) + (((p.p94 * s.db[132][7]) * s.v[132]) + (assign2260_e2933 * s.db[132][7]))) } else { (-((((((s.db[133][7] * s.v[15]) - (s.v[133] * s.db[15][7])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][7])) + (((p.p94 * s.db[132][7]) * s.v[132]) + (assign2260_e2933 * s.db[132][7])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][8]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][8] * s.v[15]) - (s.v[133] * s.db[15][8])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][8])) + (((p.p94 * s.db[132][8]) * s.v[132]) + (assign2260_e2933 * s.db[132][8]))) } else { (-((((((s.db[133][8] * s.v[15]) - (s.v[133] * s.db[15][8])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][8])) + (((p.p94 * s.db[132][8]) * s.v[132]) + (assign2260_e2933 * s.db[132][8])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][9]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][9] * s.v[15]) - (s.v[133] * s.db[15][9])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][9])) + (((p.p94 * s.db[132][9]) * s.v[132]) + (assign2260_e2933 * s.db[132][9]))) } else { (-((((((s.db[133][9] * s.v[15]) - (s.v[133] * s.db[15][9])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][9])) + (((p.p94 * s.db[132][9]) * s.v[132]) + (assign2260_e2933 * s.db[132][9])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][10]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][10] * s.v[15]) - (s.v[133] * s.db[15][10])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][10])) + (((p.p94 * s.db[132][10]) * s.v[132]) + (assign2260_e2933 * s.db[132][10]))) } else { (-((((((s.db[133][10] * s.v[15]) - (s.v[133] * s.db[15][10])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][10])) + (((p.p94 * s.db[132][10]) * s.v[132]) + (assign2260_e2933 * s.db[132][10])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][11]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][11] * s.v[15]) - (s.v[133] * s.db[15][11])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][11])) + (((p.p94 * s.db[132][11]) * s.v[132]) + (assign2260_e2933 * s.db[132][11]))) } else { (-((((((s.db[133][11] * s.v[15]) - (s.v[133] * s.db[15][11])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][11])) + (((p.p94 * s.db[132][11]) * s.v[132]) + (assign2260_e2933 * s.db[132][11])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][12]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][12] * s.v[15]) - (s.v[133] * s.db[15][12])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][12])) + (((p.p94 * s.db[132][12]) * s.v[132]) + (assign2260_e2933 * s.db[132][12]))) } else { (-((((((s.db[133][12] * s.v[15]) - (s.v[133] * s.db[15][12])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][12])) + (((p.p94 * s.db[132][12]) * s.v[132]) + (assign2260_e2933 * s.db[132][12])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][13]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][13] * s.v[15]) - (s.v[133] * s.db[15][13])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][13])) + (((p.p94 * s.db[132][13]) * s.v[132]) + (assign2260_e2933 * s.db[132][13]))) } else { (-((((((s.db[133][13] * s.v[15]) - (s.v[133] * s.db[15][13])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][13])) + (((p.p94 * s.db[132][13]) * s.v[132]) + (assign2260_e2933 * s.db[132][13])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][14]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][14] * s.v[15]) - (s.v[133] * s.db[15][14])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][14])) + (((p.p94 * s.db[132][14]) * s.v[132]) + (assign2260_e2933 * s.db[132][14]))) } else { (-((((((s.db[133][14] * s.v[15]) - (s.v[133] * s.db[15][14])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][14])) + (((p.p94 * s.db[132][14]) * s.v[132]) + (assign2260_e2933 * s.db[132][14])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][15]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][15] * s.v[15]) - (s.v[133] * s.db[15][15])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][15])) + (((p.p94 * s.db[132][15]) * s.v[132]) + (assign2260_e2933 * s.db[132][15]))) } else { (-((((((s.db[133][15] * s.v[15]) - (s.v[133] * s.db[15][15])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][15])) + (((p.p94 * s.db[132][15]) * s.v[132]) + (assign2260_e2933 * s.db[132][15])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][16]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][16] * s.v[15]) - (s.v[133] * s.db[15][16])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][16])) + (((p.p94 * s.db[132][16]) * s.v[132]) + (assign2260_e2933 * s.db[132][16]))) } else { (-((((((s.db[133][16] * s.v[15]) - (s.v[133] * s.db[15][16])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][16])) + (((p.p94 * s.db[132][16]) * s.v[132]) + (assign2260_e2933 * s.db[132][16])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][17]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][17] * s.v[15]) - (s.v[133] * s.db[15][17])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][17])) + (((p.p94 * s.db[132][17]) * s.v[132]) + (assign2260_e2933 * s.db[132][17]))) } else { (-((((((s.db[133][17] * s.v[15]) - (s.v[133] * s.db[15][17])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][17])) + (((p.p94 * s.db[132][17]) * s.v[132]) + (assign2260_e2933 * s.db[132][17])))) } / (2.0 * assign2260_e2938)))), (((assign2260_e2923 * s.db[15][18]) * assign2260_e2938) + (assign2260_e2925 * (if assign2260_e2936 >= 0.0 { ((((((s.db[133][18] * s.v[15]) - (s.v[133] * s.db[15][18])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][18])) + (((p.p94 * s.db[132][18]) * s.v[132]) + (assign2260_e2933 * s.db[132][18]))) } else { (-((((((s.db[133][18] * s.v[15]) - (s.v[133] * s.db[15][18])) / (s.v[15] * s.v[15])) * s.v[132]) + (assign2260_e2928 * s.db[132][18])) + (((p.p94 * s.db[132][18]) * s.v[132]) + (assign2260_e2933 * s.db[132][18])))) } / (2.0 * assign2260_e2938)))),)
    } else {
        (s.v[131], s.dn[131][0], s.dn[131][1], s.dn[131][2], s.dn[131][3], s.dn[131][4], s.dn[131][5], s.dn[131][6], s.dn[131][7], s.dn[131][8], s.dn[131][9], s.dn[131][10], s.dn[131][11], s.dn[131][12], s.dn[131][13], s.dn[131][14], s.dn[131][15], s.dn[131][16], s.dn[131][17], s.dn[131][18], s.db[131][0], s.db[131][1], s.db[131][2], s.db[131][3], s.db[131][4], s.db[131][5], s.db[131][6], s.db[131][7], s.db[131][8], s.db[131][9], s.db[131][10], s.db[131][11], s.db[131][12], s.db[131][13], s.db[131][14], s.db[131][15], s.db[131][16], s.db[131][17], s.db[131][18],)
    }
};
        s.v[131] = assign2260_e2941;
        s.mark_derivatives_dirty(131);
        s.dn[131][0] = assign2260_e2941_d_n0;
        s.dn[131][1] = assign2260_e2941_d_n1;
        s.dn[131][2] = assign2260_e2941_d_n2;
        s.dn[131][3] = assign2260_e2941_d_n3;
        s.dn[131][4] = assign2260_e2941_d_n4;
        s.dn[131][5] = assign2260_e2941_d_n5;
        s.dn[131][6] = assign2260_e2941_d_n6;
        s.dn[131][7] = assign2260_e2941_d_n7;
        s.dn[131][8] = assign2260_e2941_d_n8;
        s.dn[131][9] = assign2260_e2941_d_n9;
        s.dn[131][10] = assign2260_e2941_d_n10;
        s.dn[131][11] = assign2260_e2941_d_n11;
        s.dn[131][12] = assign2260_e2941_d_n12;
        s.dn[131][13] = assign2260_e2941_d_n13;
        s.dn[131][14] = assign2260_e2941_d_n14;
        s.dn[131][15] = assign2260_e2941_d_n15;
        s.dn[131][16] = assign2260_e2941_d_n16;
        s.dn[131][17] = assign2260_e2941_d_n17;
        s.dn[131][18] = assign2260_e2941_d_n18;
        s.db[131][0] = assign2260_e2941_d_b0;
        s.db[131][1] = assign2260_e2941_d_b1;
        s.db[131][2] = assign2260_e2941_d_b2;
        s.db[131][3] = assign2260_e2941_d_b3;
        s.db[131][4] = assign2260_e2941_d_b4;
        s.db[131][5] = assign2260_e2941_d_b5;
        s.db[131][6] = assign2260_e2941_d_b6;
        s.db[131][7] = assign2260_e2941_d_b7;
        s.db[131][8] = assign2260_e2941_d_b8;
        s.db[131][9] = assign2260_e2941_d_b9;
        s.db[131][10] = assign2260_e2941_d_b10;
        s.db[131][11] = assign2260_e2941_d_b11;
        s.db[131][12] = assign2260_e2941_d_b12;
        s.db[131][13] = assign2260_e2941_d_b13;
        s.db[131][14] = assign2260_e2941_d_b14;
        s.db[131][15] = assign2260_e2941_d_b15;
        s.db[131][16] = assign2260_e2941_d_b16;
        s.db[131][17] = assign2260_e2941_d_b17;
        s.db[131][18] = assign2260_e2941_d_b18;
        s.rv[131] = 0.0;

        let (assign2270_e2958, assign2270_e2958_d_n0, assign2270_e2958_d_n1, assign2270_e2958_d_n2, assign2270_e2958_d_n3, assign2270_e2958_d_n4, assign2270_e2958_d_n5, assign2270_e2958_d_n6, assign2270_e2958_d_n7, assign2270_e2958_d_n8, assign2270_e2958_d_n9, assign2270_e2958_d_n10, assign2270_e2958_d_n11, assign2270_e2958_d_n12, assign2270_e2958_d_n13, assign2270_e2958_d_n14, assign2270_e2958_d_n15, assign2270_e2958_d_n16, assign2270_e2958_d_n17, assign2270_e2958_d_n18, assign2270_e2958_d_b0, assign2270_e2958_d_b1, assign2270_e2958_d_b2, assign2270_e2958_d_b3, assign2270_e2958_d_b4, assign2270_e2958_d_b5, assign2270_e2958_d_b6, assign2270_e2958_d_b7, assign2270_e2958_d_b8, assign2270_e2958_d_b9, assign2270_e2958_d_b10, assign2270_e2958_d_b11, assign2270_e2958_d_b12, assign2270_e2958_d_b13, assign2270_e2958_d_b14, assign2270_e2958_d_b15, assign2270_e2958_d_b16, assign2270_e2958_d_b17, assign2270_e2958_d_b18,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        let assign2270_e2950: f64 = (4.0 * 1.3806503e-23);
        let assign2270_e2952: f64 = (assign2270_e2950 * s.v[15]);
        let assign2270_e2954: f64 = (assign2270_e2952 * s.v[99]);
        let assign2270_e2956: f64 = (assign2270_e2954 * p.p87);
        (assign2270_e2956, (((assign2270_e2950 * s.dn[15][0]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][1]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][2]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][3]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][4]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][5]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][6]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][7]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][8]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][9]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][10]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][11]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][12]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][13]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][14]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][15]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][16]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][17]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.dn[15][18]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][0]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][1]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][2]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][3]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][4]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][5]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][6]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][7]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][8]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][9]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][10]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][11]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][12]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][13]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][14]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][15]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][16]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][17]) * s.v[99]) * p.p87), (((assign2270_e2950 * s.db[15][18]) * s.v[99]) * p.p87),)
    } else {
        (s.v[134], s.dn[134][0], s.dn[134][1], s.dn[134][2], s.dn[134][3], s.dn[134][4], s.dn[134][5], s.dn[134][6], s.dn[134][7], s.dn[134][8], s.dn[134][9], s.dn[134][10], s.dn[134][11], s.dn[134][12], s.dn[134][13], s.dn[134][14], s.dn[134][15], s.dn[134][16], s.dn[134][17], s.dn[134][18], s.db[134][0], s.db[134][1], s.db[134][2], s.db[134][3], s.db[134][4], s.db[134][5], s.db[134][6], s.db[134][7], s.db[134][8], s.db[134][9], s.db[134][10], s.db[134][11], s.db[134][12], s.db[134][13], s.db[134][14], s.db[134][15], s.db[134][16], s.db[134][17], s.db[134][18],)
    }
};
        s.v[134] = assign2270_e2958;
        s.mark_derivatives_dirty(134);
        s.dn[134][0] = assign2270_e2958_d_n0;
        s.dn[134][1] = assign2270_e2958_d_n1;
        s.dn[134][2] = assign2270_e2958_d_n2;
        s.dn[134][3] = assign2270_e2958_d_n3;
        s.dn[134][4] = assign2270_e2958_d_n4;
        s.dn[134][5] = assign2270_e2958_d_n5;
        s.dn[134][6] = assign2270_e2958_d_n6;
        s.dn[134][7] = assign2270_e2958_d_n7;
        s.dn[134][8] = assign2270_e2958_d_n8;
        s.dn[134][9] = assign2270_e2958_d_n9;
        s.dn[134][10] = assign2270_e2958_d_n10;
        s.dn[134][11] = assign2270_e2958_d_n11;
        s.dn[134][12] = assign2270_e2958_d_n12;
        s.dn[134][13] = assign2270_e2958_d_n13;
        s.dn[134][14] = assign2270_e2958_d_n14;
        s.dn[134][15] = assign2270_e2958_d_n15;
        s.dn[134][16] = assign2270_e2958_d_n16;
        s.dn[134][17] = assign2270_e2958_d_n17;
        s.dn[134][18] = assign2270_e2958_d_n18;
        s.db[134][0] = assign2270_e2958_d_b0;
        s.db[134][1] = assign2270_e2958_d_b1;
        s.db[134][2] = assign2270_e2958_d_b2;
        s.db[134][3] = assign2270_e2958_d_b3;
        s.db[134][4] = assign2270_e2958_d_b4;
        s.db[134][5] = assign2270_e2958_d_b5;
        s.db[134][6] = assign2270_e2958_d_b6;
        s.db[134][7] = assign2270_e2958_d_b7;
        s.db[134][8] = assign2270_e2958_d_b8;
        s.db[134][9] = assign2270_e2958_d_b9;
        s.db[134][10] = assign2270_e2958_d_b10;
        s.db[134][11] = assign2270_e2958_d_b11;
        s.db[134][12] = assign2270_e2958_d_b12;
        s.db[134][13] = assign2270_e2958_d_b13;
        s.db[134][14] = assign2270_e2958_d_b14;
        s.db[134][15] = assign2270_e2958_d_b15;
        s.db[134][16] = assign2270_e2958_d_b16;
        s.db[134][17] = assign2270_e2958_d_b17;
        s.db[134][18] = assign2270_e2958_d_b18;
        s.rv[134] = 0.0;

        let assign2280_e2961: f64 = if s.v[99] > 0.0 { 1.0 } else { 0.0 };
        s.store_scalar(136, assign2280_e2961);
        s.rv[136] = 0.0;

        let (assign2290_e2984, assign2290_e2984_d_n0, assign2290_e2984_d_n1, assign2290_e2984_d_n2, assign2290_e2984_d_n3, assign2290_e2984_d_n4, assign2290_e2984_d_n5, assign2290_e2984_d_n6, assign2290_e2984_d_n7, assign2290_e2984_d_n8, assign2290_e2984_d_n9, assign2290_e2984_d_n10, assign2290_e2984_d_n11, assign2290_e2984_d_n12, assign2290_e2984_d_n13, assign2290_e2984_d_n14, assign2290_e2984_d_n15, assign2290_e2984_d_n16, assign2290_e2984_d_n17, assign2290_e2984_d_n18, assign2290_e2984_d_b0, assign2290_e2984_d_b1, assign2290_e2984_d_b2, assign2290_e2984_d_b3, assign2290_e2984_d_b4, assign2290_e2984_d_b5, assign2290_e2984_d_b6, assign2290_e2984_d_b7, assign2290_e2984_d_b8, assign2290_e2984_d_b9, assign2290_e2984_d_b10, assign2290_e2984_d_b11, assign2290_e2984_d_b12, assign2290_e2984_d_b13, assign2290_e2984_d_b14, assign2290_e2984_d_b15, assign2290_e2984_d_b16, assign2290_e2984_d_b17, assign2290_e2984_d_b18,) = {
    if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && (s.v[136] != 0.0)) {
        let assign2290_e2972: f64 = (s.v[44] * s.v[44]);
        let assign2290_e2974: f64 = (assign2290_e2972 * 4.0);
        let assign2290_e2976: f64 = (assign2290_e2974 * 1.3806503e-23);
        let assign2290_e2978: f64 = (assign2290_e2976 * s.v[15]);
        let assign2290_e2980: f64 = (assign2290_e2978 * p.p86);
        let assign2290_e2982: f64 = (assign2290_e2980 / s.v[99]);
        (assign2290_e2982, ((((((((s.dn[44][0] * s.v[44]) + (s.v[44] * s.dn[44][0])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][0])) * p.p86) / s.v[99]), ((((((((s.dn[44][1] * s.v[44]) + (s.v[44] * s.dn[44][1])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][1])) * p.p86) / s.v[99]), ((((((((s.dn[44][2] * s.v[44]) + (s.v[44] * s.dn[44][2])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][2])) * p.p86) / s.v[99]), ((((((((s.dn[44][3] * s.v[44]) + (s.v[44] * s.dn[44][3])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][3])) * p.p86) / s.v[99]), ((((((((s.dn[44][4] * s.v[44]) + (s.v[44] * s.dn[44][4])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][4])) * p.p86) / s.v[99]), ((((((((s.dn[44][5] * s.v[44]) + (s.v[44] * s.dn[44][5])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][5])) * p.p86) / s.v[99]), ((((((((s.dn[44][6] * s.v[44]) + (s.v[44] * s.dn[44][6])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][6])) * p.p86) / s.v[99]), ((((((((s.dn[44][7] * s.v[44]) + (s.v[44] * s.dn[44][7])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][7])) * p.p86) / s.v[99]), ((((((((s.dn[44][8] * s.v[44]) + (s.v[44] * s.dn[44][8])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][8])) * p.p86) / s.v[99]), ((((((((s.dn[44][9] * s.v[44]) + (s.v[44] * s.dn[44][9])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][9])) * p.p86) / s.v[99]), ((((((((s.dn[44][10] * s.v[44]) + (s.v[44] * s.dn[44][10])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][10])) * p.p86) / s.v[99]), ((((((((s.dn[44][11] * s.v[44]) + (s.v[44] * s.dn[44][11])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][11])) * p.p86) / s.v[99]), ((((((((s.dn[44][12] * s.v[44]) + (s.v[44] * s.dn[44][12])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][12])) * p.p86) / s.v[99]), ((((((((s.dn[44][13] * s.v[44]) + (s.v[44] * s.dn[44][13])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][13])) * p.p86) / s.v[99]), ((((((((s.dn[44][14] * s.v[44]) + (s.v[44] * s.dn[44][14])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][14])) * p.p86) / s.v[99]), ((((((((s.dn[44][15] * s.v[44]) + (s.v[44] * s.dn[44][15])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][15])) * p.p86) / s.v[99]), ((((((((s.dn[44][16] * s.v[44]) + (s.v[44] * s.dn[44][16])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][16])) * p.p86) / s.v[99]), ((((((((s.dn[44][17] * s.v[44]) + (s.v[44] * s.dn[44][17])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][17])) * p.p86) / s.v[99]), ((((((((s.dn[44][18] * s.v[44]) + (s.v[44] * s.dn[44][18])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.dn[15][18])) * p.p86) / s.v[99]), ((((((((s.db[44][0] * s.v[44]) + (s.v[44] * s.db[44][0])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][0])) * p.p86) / s.v[99]), ((((((((s.db[44][1] * s.v[44]) + (s.v[44] * s.db[44][1])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][1])) * p.p86) / s.v[99]), ((((((((s.db[44][2] * s.v[44]) + (s.v[44] * s.db[44][2])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][2])) * p.p86) / s.v[99]), ((((((((s.db[44][3] * s.v[44]) + (s.v[44] * s.db[44][3])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][3])) * p.p86) / s.v[99]), ((((((((s.db[44][4] * s.v[44]) + (s.v[44] * s.db[44][4])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][4])) * p.p86) / s.v[99]), ((((((((s.db[44][5] * s.v[44]) + (s.v[44] * s.db[44][5])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][5])) * p.p86) / s.v[99]), ((((((((s.db[44][6] * s.v[44]) + (s.v[44] * s.db[44][6])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][6])) * p.p86) / s.v[99]), ((((((((s.db[44][7] * s.v[44]) + (s.v[44] * s.db[44][7])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][7])) * p.p86) / s.v[99]), ((((((((s.db[44][8] * s.v[44]) + (s.v[44] * s.db[44][8])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][8])) * p.p86) / s.v[99]), ((((((((s.db[44][9] * s.v[44]) + (s.v[44] * s.db[44][9])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][9])) * p.p86) / s.v[99]), ((((((((s.db[44][10] * s.v[44]) + (s.v[44] * s.db[44][10])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][10])) * p.p86) / s.v[99]), ((((((((s.db[44][11] * s.v[44]) + (s.v[44] * s.db[44][11])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][11])) * p.p86) / s.v[99]), ((((((((s.db[44][12] * s.v[44]) + (s.v[44] * s.db[44][12])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][12])) * p.p86) / s.v[99]), ((((((((s.db[44][13] * s.v[44]) + (s.v[44] * s.db[44][13])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][13])) * p.p86) / s.v[99]), ((((((((s.db[44][14] * s.v[44]) + (s.v[44] * s.db[44][14])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][14])) * p.p86) / s.v[99]), ((((((((s.db[44][15] * s.v[44]) + (s.v[44] * s.db[44][15])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][15])) * p.p86) / s.v[99]), ((((((((s.db[44][16] * s.v[44]) + (s.v[44] * s.db[44][16])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][16])) * p.p86) / s.v[99]), ((((((((s.db[44][17] * s.v[44]) + (s.v[44] * s.db[44][17])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][17])) * p.p86) / s.v[99]), ((((((((s.db[44][18] * s.v[44]) + (s.v[44] * s.db[44][18])) * 4.0) * 1.3806503e-23) * s.v[15]) + (assign2290_e2976 * s.db[15][18])) * p.p86) / s.v[99]),)
    } else {
        (s.v[135], s.dn[135][0], s.dn[135][1], s.dn[135][2], s.dn[135][3], s.dn[135][4], s.dn[135][5], s.dn[135][6], s.dn[135][7], s.dn[135][8], s.dn[135][9], s.dn[135][10], s.dn[135][11], s.dn[135][12], s.dn[135][13], s.dn[135][14], s.dn[135][15], s.dn[135][16], s.dn[135][17], s.dn[135][18], s.db[135][0], s.db[135][1], s.db[135][2], s.db[135][3], s.db[135][4], s.db[135][5], s.db[135][6], s.db[135][7], s.db[135][8], s.db[135][9], s.db[135][10], s.db[135][11], s.db[135][12], s.db[135][13], s.db[135][14], s.db[135][15], s.db[135][16], s.db[135][17], s.db[135][18],)
    }
};
        s.v[135] = assign2290_e2984;
        s.mark_derivatives_dirty(135);
        s.dn[135][0] = assign2290_e2984_d_n0;
        s.dn[135][1] = assign2290_e2984_d_n1;
        s.dn[135][2] = assign2290_e2984_d_n2;
        s.dn[135][3] = assign2290_e2984_d_n3;
        s.dn[135][4] = assign2290_e2984_d_n4;
        s.dn[135][5] = assign2290_e2984_d_n5;
        s.dn[135][6] = assign2290_e2984_d_n6;
        s.dn[135][7] = assign2290_e2984_d_n7;
        s.dn[135][8] = assign2290_e2984_d_n8;
        s.dn[135][9] = assign2290_e2984_d_n9;
        s.dn[135][10] = assign2290_e2984_d_n10;
        s.dn[135][11] = assign2290_e2984_d_n11;
        s.dn[135][12] = assign2290_e2984_d_n12;
        s.dn[135][13] = assign2290_e2984_d_n13;
        s.dn[135][14] = assign2290_e2984_d_n14;
        s.dn[135][15] = assign2290_e2984_d_n15;
        s.dn[135][16] = assign2290_e2984_d_n16;
        s.dn[135][17] = assign2290_e2984_d_n17;
        s.dn[135][18] = assign2290_e2984_d_n18;
        s.db[135][0] = assign2290_e2984_d_b0;
        s.db[135][1] = assign2290_e2984_d_b1;
        s.db[135][2] = assign2290_e2984_d_b2;
        s.db[135][3] = assign2290_e2984_d_b3;
        s.db[135][4] = assign2290_e2984_d_b4;
        s.db[135][5] = assign2290_e2984_d_b5;
        s.db[135][6] = assign2290_e2984_d_b6;
        s.db[135][7] = assign2290_e2984_d_b7;
        s.db[135][8] = assign2290_e2984_d_b8;
        s.db[135][9] = assign2290_e2984_d_b9;
        s.db[135][10] = assign2290_e2984_d_b10;
        s.db[135][11] = assign2290_e2984_d_b11;
        s.db[135][12] = assign2290_e2984_d_b12;
        s.db[135][13] = assign2290_e2984_d_b13;
        s.db[135][14] = assign2290_e2984_d_b14;
        s.db[135][15] = assign2290_e2984_d_b15;
        s.db[135][16] = assign2290_e2984_d_b16;
        s.db[135][17] = assign2290_e2984_d_b17;
        s.db[135][18] = assign2290_e2984_d_b18;
        s.rv[135] = 0.0;

        let (assign2300_e2996, assign2300_e2996_d_n0, assign2300_e2996_d_n1, assign2300_e2996_d_n2, assign2300_e2996_d_n3, assign2300_e2996_d_n4, assign2300_e2996_d_n5, assign2300_e2996_d_n6, assign2300_e2996_d_n7, assign2300_e2996_d_n8, assign2300_e2996_d_n9, assign2300_e2996_d_n10, assign2300_e2996_d_n11, assign2300_e2996_d_n12, assign2300_e2996_d_n13, assign2300_e2996_d_n14, assign2300_e2996_d_n15, assign2300_e2996_d_n16, assign2300_e2996_d_n17, assign2300_e2996_d_n18, assign2300_e2996_d_b0, assign2300_e2996_d_b1, assign2300_e2996_d_b2, assign2300_e2996_d_b3, assign2300_e2996_d_b4, assign2300_e2996_d_b5, assign2300_e2996_d_b6, assign2300_e2996_d_b7, assign2300_e2996_d_b8, assign2300_e2996_d_b9, assign2300_e2996_d_b10, assign2300_e2996_d_b11, assign2300_e2996_d_b12, assign2300_e2996_d_b13, assign2300_e2996_d_b14, assign2300_e2996_d_b15, assign2300_e2996_d_b16, assign2300_e2996_d_b17, assign2300_e2996_d_b18,) = {
    if (((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) && (s.v[136] == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (s.v[135], s.dn[135][0], s.dn[135][1], s.dn[135][2], s.dn[135][3], s.dn[135][4], s.dn[135][5], s.dn[135][6], s.dn[135][7], s.dn[135][8], s.dn[135][9], s.dn[135][10], s.dn[135][11], s.dn[135][12], s.dn[135][13], s.dn[135][14], s.dn[135][15], s.dn[135][16], s.dn[135][17], s.dn[135][18], s.db[135][0], s.db[135][1], s.db[135][2], s.db[135][3], s.db[135][4], s.db[135][5], s.db[135][6], s.db[135][7], s.db[135][8], s.db[135][9], s.db[135][10], s.db[135][11], s.db[135][12], s.db[135][13], s.db[135][14], s.db[135][15], s.db[135][16], s.db[135][17], s.db[135][18],)
    }
};
        s.v[135] = assign2300_e2996;
        s.mark_derivatives_dirty(135);
        s.dn[135][0] = assign2300_e2996_d_n0;
        s.dn[135][1] = assign2300_e2996_d_n1;
        s.dn[135][2] = assign2300_e2996_d_n2;
        s.dn[135][3] = assign2300_e2996_d_n3;
        s.dn[135][4] = assign2300_e2996_d_n4;
        s.dn[135][5] = assign2300_e2996_d_n5;
        s.dn[135][6] = assign2300_e2996_d_n6;
        s.dn[135][7] = assign2300_e2996_d_n7;
        s.dn[135][8] = assign2300_e2996_d_n8;
        s.dn[135][9] = assign2300_e2996_d_n9;
        s.dn[135][10] = assign2300_e2996_d_n10;
        s.dn[135][11] = assign2300_e2996_d_n11;
        s.dn[135][12] = assign2300_e2996_d_n12;
        s.dn[135][13] = assign2300_e2996_d_n13;
        s.dn[135][14] = assign2300_e2996_d_n14;
        s.dn[135][15] = assign2300_e2996_d_n15;
        s.dn[135][16] = assign2300_e2996_d_n16;
        s.dn[135][17] = assign2300_e2996_d_n17;
        s.dn[135][18] = assign2300_e2996_d_n18;
        s.db[135][0] = assign2300_e2996_d_b0;
        s.db[135][1] = assign2300_e2996_d_b1;
        s.db[135][2] = assign2300_e2996_d_b2;
        s.db[135][3] = assign2300_e2996_d_b3;
        s.db[135][4] = assign2300_e2996_d_b4;
        s.db[135][5] = assign2300_e2996_d_b5;
        s.db[135][6] = assign2300_e2996_d_b6;
        s.db[135][7] = assign2300_e2996_d_b7;
        s.db[135][8] = assign2300_e2996_d_b8;
        s.db[135][9] = assign2300_e2996_d_b9;
        s.db[135][10] = assign2300_e2996_d_b10;
        s.db[135][11] = assign2300_e2996_d_b11;
        s.db[135][12] = assign2300_e2996_d_b12;
        s.db[135][13] = assign2300_e2996_d_b13;
        s.db[135][14] = assign2300_e2996_d_b14;
        s.db[135][15] = assign2300_e2996_d_b15;
        s.db[135][16] = assign2300_e2996_d_b16;
        s.db[135][17] = assign2300_e2996_d_b17;
        s.db[135][18] = assign2300_e2996_d_b18;
        s.rv[135] = 0.0;

        if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
            s.store_scaled_mul(140, 15, 44, (((4.0 * 1.3806503e-23) * p.p88) * (((p.p87 * p.p86)) as f64).sqrt()));
            s.store_scale(138, 140, 3.141592653589793);
        }

        let (assign2350_e3074, assign2350_e3074_d_n0, assign2350_e3074_d_n1, assign2350_e3074_d_n2, assign2350_e3074_d_n3, assign2350_e3074_d_n4, assign2350_e3074_d_n5, assign2350_e3074_d_n6, assign2350_e3074_d_n7, assign2350_e3074_d_n8, assign2350_e3074_d_n9, assign2350_e3074_d_n10, assign2350_e3074_d_n11, assign2350_e3074_d_n12, assign2350_e3074_d_n13, assign2350_e3074_d_n14, assign2350_e3074_d_n15, assign2350_e3074_d_n16, assign2350_e3074_d_n17, assign2350_e3074_d_n18, assign2350_e3074_d_b0, assign2350_e3074_d_b1, assign2350_e3074_d_b2, assign2350_e3074_d_b3, assign2350_e3074_d_b4, assign2350_e3074_d_b5, assign2350_e3074_d_b6, assign2350_e3074_d_b7, assign2350_e3074_d_b8, assign2350_e3074_d_b9, assign2350_e3074_d_b10, assign2350_e3074_d_b11, assign2350_e3074_d_b12, assign2350_e3074_d_b13, assign2350_e3074_d_b14, assign2350_e3074_d_b15, assign2350_e3074_d_b16, assign2350_e3074_d_b17, assign2350_e3074_d_b18,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        let assign2350_e3064: f64 = (4.0 * 1.3806503e-23);
        let assign2350_e3066: f64 = (assign2350_e3064 * s.v[15]);
        let assign2350_e3068: f64 = (assign2350_e3066 * s.v[99]);
        let assign2350_e3070: f64 = (assign2350_e3068 * p.p87);
        let assign2350_e3072: f64 = (assign2350_e3070 * p.p89);
        (assign2350_e3072, ((((assign2350_e3064 * s.dn[15][0]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][1]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][2]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][3]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][4]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][5]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][6]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][7]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][8]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][9]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][10]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][11]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][12]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][13]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][14]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][15]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][16]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][17]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.dn[15][18]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][0]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][1]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][2]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][3]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][4]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][5]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][6]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][7]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][8]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][9]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][10]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][11]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][12]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][13]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][14]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][15]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][16]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][17]) * s.v[99]) * p.p87) * p.p89), ((((assign2350_e3064 * s.db[15][18]) * s.v[99]) * p.p87) * p.p89),)
    } else {
        (s.v[141], s.dn[141][0], s.dn[141][1], s.dn[141][2], s.dn[141][3], s.dn[141][4], s.dn[141][5], s.dn[141][6], s.dn[141][7], s.dn[141][8], s.dn[141][9], s.dn[141][10], s.dn[141][11], s.dn[141][12], s.dn[141][13], s.dn[141][14], s.dn[141][15], s.dn[141][16], s.dn[141][17], s.dn[141][18], s.db[141][0], s.db[141][1], s.db[141][2], s.db[141][3], s.db[141][4], s.db[141][5], s.db[141][6], s.db[141][7], s.db[141][8], s.db[141][9], s.db[141][10], s.db[141][11], s.db[141][12], s.db[141][13], s.db[141][14], s.db[141][15], s.db[141][16], s.db[141][17], s.db[141][18],)
    }
};
        s.v[141] = assign2350_e3074;
        s.mark_derivatives_dirty(141);
        s.dn[141][0] = assign2350_e3074_d_n0;
        s.dn[141][1] = assign2350_e3074_d_n1;
        s.dn[141][2] = assign2350_e3074_d_n2;
        s.dn[141][3] = assign2350_e3074_d_n3;
        s.dn[141][4] = assign2350_e3074_d_n4;
        s.dn[141][5] = assign2350_e3074_d_n5;
        s.dn[141][6] = assign2350_e3074_d_n6;
        s.dn[141][7] = assign2350_e3074_d_n7;
        s.dn[141][8] = assign2350_e3074_d_n8;
        s.dn[141][9] = assign2350_e3074_d_n9;
        s.dn[141][10] = assign2350_e3074_d_n10;
        s.dn[141][11] = assign2350_e3074_d_n11;
        s.dn[141][12] = assign2350_e3074_d_n12;
        s.dn[141][13] = assign2350_e3074_d_n13;
        s.dn[141][14] = assign2350_e3074_d_n14;
        s.dn[141][15] = assign2350_e3074_d_n15;
        s.dn[141][16] = assign2350_e3074_d_n16;
        s.dn[141][17] = assign2350_e3074_d_n17;
        s.dn[141][18] = assign2350_e3074_d_n18;
        s.db[141][0] = assign2350_e3074_d_b0;
        s.db[141][1] = assign2350_e3074_d_b1;
        s.db[141][2] = assign2350_e3074_d_b2;
        s.db[141][3] = assign2350_e3074_d_b3;
        s.db[141][4] = assign2350_e3074_d_b4;
        s.db[141][5] = assign2350_e3074_d_b5;
        s.db[141][6] = assign2350_e3074_d_b6;
        s.db[141][7] = assign2350_e3074_d_b7;
        s.db[141][8] = assign2350_e3074_d_b8;
        s.db[141][9] = assign2350_e3074_d_b9;
        s.db[141][10] = assign2350_e3074_d_b10;
        s.db[141][11] = assign2350_e3074_d_b11;
        s.db[141][12] = assign2350_e3074_d_b12;
        s.db[141][13] = assign2350_e3074_d_b13;
        s.db[141][14] = assign2350_e3074_d_b14;
        s.db[141][15] = assign2350_e3074_d_b15;
        s.db[141][16] = assign2350_e3074_d_b16;
        s.db[141][17] = assign2350_e3074_d_b17;
        s.db[141][18] = assign2350_e3074_d_b18;
        s.rv[141] = 0.0;

        s.b[143] = (p.p90 > 0.0);
        s.store_scalar(143, if s.b[143] { 1.0 } else { 0.0 });

        s.b[144] = (p.p1 == 1.0);
        s.store_scalar(144, if s.b[144] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        var_cdel_t: f64,
        var_cdel_t_db0: f64,
        var_cdel_t_db1: f64,
        var_cdel_t_db10: f64,
        var_cdel_t_db11: f64,
        var_cdel_t_db12: f64,
        var_cdel_t_db13: f64,
        var_cdel_t_db14: f64,
        var_cdel_t_db15: f64,
        var_cdel_t_db16: f64,
        var_cdel_t_db17: f64,
        var_cdel_t_db18: f64,
        var_cdel_t_db2: f64,
        var_cdel_t_db3: f64,
        var_cdel_t_db4: f64,
        var_cdel_t_db5: f64,
        var_cdel_t_db6: f64,
        var_cdel_t_db7: f64,
        var_cdel_t_db8: f64,
        var_cdel_t_db9: f64,
        var_cdel_t_dn0: f64,
        var_cdel_t_dn1: f64,
        var_cdel_t_dn10: f64,
        var_cdel_t_dn11: f64,
        var_cdel_t_dn12: f64,
        var_cdel_t_dn13: f64,
        var_cdel_t_dn14: f64,
        var_cdel_t_dn15: f64,
        var_cdel_t_dn16: f64,
        var_cdel_t_dn17: f64,
        var_cdel_t_dn18: f64,
        var_cdel_t_dn2: f64,
        var_cdel_t_dn3: f64,
        var_cdel_t_dn4: f64,
        var_cdel_t_dn5: f64,
        var_cdel_t_dn6: f64,
        var_cdel_t_dn7: f64,
        var_cdel_t_dn8: f64,
        var_cdel_t_dn9: f64,
        var_cgd: f64,
        var_cgd_db0: f64,
        var_cgd_db1: f64,
        var_cgd_db10: f64,
        var_cgd_db11: f64,
        var_cgd_db12: f64,
        var_cgd_db13: f64,
        var_cgd_db14: f64,
        var_cgd_db15: f64,
        var_cgd_db16: f64,
        var_cgd_db17: f64,
        var_cgd_db18: f64,
        var_cgd_db2: f64,
        var_cgd_db3: f64,
        var_cgd_db4: f64,
        var_cgd_db5: f64,
        var_cgd_db6: f64,
        var_cgd_db7: f64,
        var_cgd_db8: f64,
        var_cgd_db9: f64,
        var_cgd_dn0: f64,
        var_cgd_dn1: f64,
        var_cgd_dn10: f64,
        var_cgd_dn11: f64,
        var_cgd_dn12: f64,
        var_cgd_dn13: f64,
        var_cgd_dn14: f64,
        var_cgd_dn15: f64,
        var_cgd_dn16: f64,
        var_cgd_dn17: f64,
        var_cgd_dn18: f64,
        var_cgd_dn2: f64,
        var_cgd_dn3: f64,
        var_cgd_dn4: f64,
        var_cgd_dn5: f64,
        var_cgd_dn6: f64,
        var_cgd_dn7: f64,
        var_cgd_dn8: f64,
        var_cgd_dn9: f64,
        var_cgs: f64,
        var_cgs_db0: f64,
        var_cgs_db1: f64,
        var_cgs_db10: f64,
        var_cgs_db11: f64,
        var_cgs_db12: f64,
        var_cgs_db13: f64,
        var_cgs_db14: f64,
        var_cgs_db15: f64,
        var_cgs_db16: f64,
        var_cgs_db17: f64,
        var_cgs_db18: f64,
        var_cgs_db2: f64,
        var_cgs_db3: f64,
        var_cgs_db4: f64,
        var_cgs_db5: f64,
        var_cgs_db6: f64,
        var_cgs_db7: f64,
        var_cgs_db8: f64,
        var_cgs_db9: f64,
        var_cgs_dn0: f64,
        var_cgs_dn1: f64,
        var_cgs_dn10: f64,
        var_cgs_dn11: f64,
        var_cgs_dn12: f64,
        var_cgs_dn13: f64,
        var_cgs_dn14: f64,
        var_cgs_dn15: f64,
        var_cgs_dn16: f64,
        var_cgs_dn17: f64,
        var_cgs_dn18: f64,
        var_cgs_dn2: f64,
        var_cgs_dn3: f64,
        var_cgs_dn4: f64,
        var_cgs_dn5: f64,
        var_cgs_dn6: f64,
        var_cgs_dn7: f64,
        var_cgs_dn8: f64,
        var_cgs_dn9: f64,
        var_guard19: f64,
        var_guard20: f64,
        var_guard21: f64,
        var_guard25: f64,
        var_guard26: f64,
        var_guard27: f64,
        var_qgd: f64,
        var_qgd_db0: f64,
        var_qgd_db1: f64,
        var_qgd_db10: f64,
        var_qgd_db11: f64,
        var_qgd_db12: f64,
        var_qgd_db13: f64,
        var_qgd_db14: f64,
        var_qgd_db15: f64,
        var_qgd_db16: f64,
        var_qgd_db17: f64,
        var_qgd_db18: f64,
        var_qgd_db2: f64,
        var_qgd_db3: f64,
        var_qgd_db4: f64,
        var_qgd_db5: f64,
        var_qgd_db6: f64,
        var_qgd_db7: f64,
        var_qgd_db8: f64,
        var_qgd_db9: f64,
        var_qgd_dn0: f64,
        var_qgd_dn1: f64,
        var_qgd_dn10: f64,
        var_qgd_dn11: f64,
        var_qgd_dn12: f64,
        var_qgd_dn13: f64,
        var_qgd_dn14: f64,
        var_qgd_dn15: f64,
        var_qgd_dn16: f64,
        var_qgd_dn17: f64,
        var_qgd_dn18: f64,
        var_qgd_dn2: f64,
        var_qgd_dn3: f64,
        var_qgd_dn4: f64,
        var_qgd_dn5: f64,
        var_qgd_dn6: f64,
        var_qgd_dn7: f64,
        var_qgd_dn8: f64,
        var_qgd_dn9: f64,
        var_qgs: f64,
        var_qgs_db0: f64,
        var_qgs_db1: f64,
        var_qgs_db10: f64,
        var_qgs_db11: f64,
        var_qgs_db12: f64,
        var_qgs_db13: f64,
        var_qgs_db14: f64,
        var_qgs_db15: f64,
        var_qgs_db16: f64,
        var_qgs_db17: f64,
        var_qgs_db18: f64,
        var_qgs_db2: f64,
        var_qgs_db3: f64,
        var_qgs_db4: f64,
        var_qgs_db5: f64,
        var_qgs_db6: f64,
        var_qgs_db7: f64,
        var_qgs_db8: f64,
        var_qgs_db9: f64,
        var_qgs_dn0: f64,
        var_qgs_dn1: f64,
        var_qgs_dn10: f64,
        var_qgs_dn11: f64,
        var_qgs_dn12: f64,
        var_qgs_dn13: f64,
        var_qgs_dn14: f64,
        var_qgs_dn15: f64,
        var_qgs_dn16: f64,
        var_qgs_dn17: f64,
        var_qgs_dn18: f64,
        var_qgs_dn2: f64,
        var_qgs_dn3: f64,
        var_qgs_dn4: f64,
        var_qgs_dn5: f64,
        var_qgs_dn6: f64,
        var_qgs_dn7: f64,
        var_qgs_dn8: f64,
        var_qgs_dn9: f64,
        var_rc1: f64,
        var_rc1_db0: f64,
        var_rc1_db1: f64,
        var_rc1_db10: f64,
        var_rc1_db11: f64,
        var_rc1_db12: f64,
        var_rc1_db13: f64,
        var_rc1_db14: f64,
        var_rc1_db15: f64,
        var_rc1_db16: f64,
        var_rc1_db17: f64,
        var_rc1_db18: f64,
        var_rc1_db2: f64,
        var_rc1_db3: f64,
        var_rc1_db4: f64,
        var_rc1_db5: f64,
        var_rc1_db6: f64,
        var_rc1_db7: f64,
        var_rc1_db8: f64,
        var_rc1_db9: f64,
        var_rc1_dn0: f64,
        var_rc1_dn1: f64,
        var_rc1_dn10: f64,
        var_rc1_dn11: f64,
        var_rc1_dn12: f64,
        var_rc1_dn13: f64,
        var_rc1_dn14: f64,
        var_rc1_dn15: f64,
        var_rc1_dn16: f64,
        var_rc1_dn17: f64,
        var_rc1_dn18: f64,
        var_rc1_dn2: f64,
        var_rc1_dn3: f64,
        var_rc1_dn4: f64,
        var_rc1_dn5: f64,
        var_rc1_dn6: f64,
        var_rc1_dn7: f64,
        var_rc1_dn8: f64,
        var_rc1_dn9: f64,
        var_rd1_t: f64,
        var_rd1_t_db0: f64,
        var_rd1_t_db1: f64,
        var_rd1_t_db10: f64,
        var_rd1_t_db11: f64,
        var_rd1_t_db12: f64,
        var_rd1_t_db13: f64,
        var_rd1_t_db14: f64,
        var_rd1_t_db15: f64,
        var_rd1_t_db16: f64,
        var_rd1_t_db17: f64,
        var_rd1_t_db18: f64,
        var_rd1_t_db2: f64,
        var_rd1_t_db3: f64,
        var_rd1_t_db4: f64,
        var_rd1_t_db5: f64,
        var_rd1_t_db6: f64,
        var_rd1_t_db7: f64,
        var_rd1_t_db8: f64,
        var_rd1_t_db9: f64,
        var_rd1_t_dn0: f64,
        var_rd1_t_dn1: f64,
        var_rd1_t_dn10: f64,
        var_rd1_t_dn11: f64,
        var_rd1_t_dn12: f64,
        var_rd1_t_dn13: f64,
        var_rd1_t_dn14: f64,
        var_rd1_t_dn15: f64,
        var_rd1_t_dn16: f64,
        var_rd1_t_dn17: f64,
        var_rd1_t_dn18: f64,
        var_rd1_t_dn2: f64,
        var_rd1_t_dn3: f64,
        var_rd1_t_dn4: f64,
        var_rd1_t_dn5: f64,
        var_rd1_t_dn6: f64,
        var_rd1_t_dn7: f64,
        var_rd1_t_dn8: f64,
        var_rd1_t_dn9: f64,
        var_rs_t: f64,
        var_rs_t_db0: f64,
        var_rs_t_db1: f64,
        var_rs_t_db10: f64,
        var_rs_t_db11: f64,
        var_rs_t_db12: f64,
        var_rs_t_db13: f64,
        var_rs_t_db14: f64,
        var_rs_t_db15: f64,
        var_rs_t_db16: f64,
        var_rs_t_db17: f64,
        var_rs_t_db18: f64,
        var_rs_t_db2: f64,
        var_rs_t_db3: f64,
        var_rs_t_db4: f64,
        var_rs_t_db5: f64,
        var_rs_t_db6: f64,
        var_rs_t_db7: f64,
        var_rs_t_db8: f64,
        var_rs_t_db9: f64,
        var_rs_t_dn0: f64,
        var_rs_t_dn1: f64,
        var_rs_t_dn10: f64,
        var_rs_t_dn11: f64,
        var_rs_t_dn12: f64,
        var_rs_t_dn13: f64,
        var_rs_t_dn14: f64,
        var_rs_t_dn15: f64,
        var_rs_t_dn16: f64,
        var_rs_t_dn17: f64,
        var_rs_t_dn18: f64,
        var_rs_t_dn2: f64,
        var_rs_t_dn3: f64,
        var_rs_t_dn4: f64,
        var_rs_t_dn5: f64,
        var_rs_t_dn6: f64,
        var_rs_t_dn7: f64,
        var_rs_t_dn8: f64,
        var_rs_t_dn9: f64,
        var_t0: f64,
        var_t0_db0: f64,
        var_t0_db1: f64,
        var_t0_db10: f64,
        var_t0_db11: f64,
        var_t0_db12: f64,
        var_t0_db13: f64,
        var_t0_db14: f64,
        var_t0_db15: f64,
        var_t0_db16: f64,
        var_t0_db17: f64,
        var_t0_db18: f64,
        var_t0_db2: f64,
        var_t0_db3: f64,
        var_t0_db4: f64,
        var_t0_db5: f64,
        var_t0_db6: f64,
        var_t0_db7: f64,
        var_t0_db8: f64,
        var_t0_db9: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn12: f64,
        var_t0_dn13: f64,
        var_t0_dn14: f64,
        var_t0_dn15: f64,
        var_t0_dn16: f64,
        var_t0_dn17: f64,
        var_t0_dn18: f64,
        var_t0_dn2: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_vgdc: f64,
        var_vgdc_db0: f64,
        var_vgdc_db1: f64,
        var_vgdc_db10: f64,
        var_vgdc_db11: f64,
        var_vgdc_db12: f64,
        var_vgdc_db13: f64,
        var_vgdc_db14: f64,
        var_vgdc_db15: f64,
        var_vgdc_db16: f64,
        var_vgdc_db17: f64,
        var_vgdc_db18: f64,
        var_vgdc_db2: f64,
        var_vgdc_db3: f64,
        var_vgdc_db4: f64,
        var_vgdc_db5: f64,
        var_vgdc_db6: f64,
        var_vgdc_db7: f64,
        var_vgdc_db8: f64,
        var_vgdc_db9: f64,
        var_vgdc_dn0: f64,
        var_vgdc_dn1: f64,
        var_vgdc_dn10: f64,
        var_vgdc_dn11: f64,
        var_vgdc_dn12: f64,
        var_vgdc_dn13: f64,
        var_vgdc_dn14: f64,
        var_vgdc_dn15: f64,
        var_vgdc_dn16: f64,
        var_vgdc_dn17: f64,
        var_vgdc_dn18: f64,
        var_vgdc_dn2: f64,
        var_vgdc_dn3: f64,
        var_vgdc_dn4: f64,
        var_vgdc_dn5: f64,
        var_vgdc_dn6: f64,
        var_vgdc_dn7: f64,
        var_vgdc_dn8: f64,
        var_vgdc_dn9: f64,
        var_vgsc: f64,
        var_vgsc_db0: f64,
        var_vgsc_db1: f64,
        var_vgsc_db10: f64,
        var_vgsc_db11: f64,
        var_vgsc_db12: f64,
        var_vgsc_db13: f64,
        var_vgsc_db14: f64,
        var_vgsc_db15: f64,
        var_vgsc_db16: f64,
        var_vgsc_db17: f64,
        var_vgsc_db18: f64,
        var_vgsc_db2: f64,
        var_vgsc_db3: f64,
        var_vgsc_db4: f64,
        var_vgsc_db5: f64,
        var_vgsc_db6: f64,
        var_vgsc_db7: f64,
        var_vgsc_db8: f64,
        var_vgsc_db9: f64,
        var_vgsc_dn0: f64,
        var_vgsc_dn1: f64,
        var_vgsc_dn10: f64,
        var_vgsc_dn11: f64,
        var_vgsc_dn12: f64,
        var_vgsc_dn13: f64,
        var_vgsc_dn14: f64,
        var_vgsc_dn15: f64,
        var_vgsc_dn16: f64,
        var_vgsc_dn17: f64,
        var_vgsc_dn18: f64,
        var_vgsc_dn2: f64,
        var_vgsc_dn3: f64,
        var_vgsc_dn4: f64,
        var_vgsc_dn5: f64,
        var_vgsc_dn6: f64,
        var_vgsc_dn7: f64,
        var_vgsc_dn8: f64,
        var_vgsc_dn9: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi7 = ctx.branch_current(branches[7]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi11 = ctx.branch_current(branches[11]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi15 = ctx.branch_current(branches[15]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq3_e116);
        let eq3_value: f64 = eq3_e117;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            (eq3_e114 * ddt_scale),
        );
        let (eq7_e125, eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18, eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18,) = {
    if (var_guard19 != 0.0) {
        let eq7_e123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, var_qgd);
        (eq7_e123, (var_qgd_dn0 * ddt_scale), (var_qgd_dn1 * ddt_scale), (var_qgd_dn2 * ddt_scale), (var_qgd_dn3 * ddt_scale), (var_qgd_dn4 * ddt_scale), (var_qgd_dn5 * ddt_scale), (var_qgd_dn6 * ddt_scale), (var_qgd_dn7 * ddt_scale), (var_qgd_dn8 * ddt_scale), (var_qgd_dn9 * ddt_scale), (var_qgd_dn10 * ddt_scale), (var_qgd_dn11 * ddt_scale), (var_qgd_dn12 * ddt_scale), (var_qgd_dn13 * ddt_scale), (var_qgd_dn14 * ddt_scale), (var_qgd_dn15 * ddt_scale), (var_qgd_dn16 * ddt_scale), (var_qgd_dn17 * ddt_scale), (var_qgd_dn18 * ddt_scale), (var_qgd_db0 * ddt_scale), (var_qgd_db1 * ddt_scale), (var_qgd_db2 * ddt_scale), (var_qgd_db3 * ddt_scale), (var_qgd_db4 * ddt_scale), (var_qgd_db5 * ddt_scale), (var_qgd_db6 * ddt_scale), (var_qgd_db7 * ddt_scale), (var_qgd_db8 * ddt_scale), (var_qgd_db9 * ddt_scale), (var_qgd_db10 * ddt_scale), (var_qgd_db11 * ddt_scale), (var_qgd_db12 * ddt_scale), (var_qgd_db13 * ddt_scale), (var_qgd_db14 * ddt_scale), (var_qgd_db15 * ddt_scale), (var_qgd_db16 * ddt_scale), (var_qgd_db17 * ddt_scale), (var_qgd_db18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e125;
        let eq7_node_derivatives: [f64; 19] = [eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18];
        let eq7_branch_derivatives: [f64; 19] = [eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18, eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18,) = {
    if (var_guard19 != 0.0) {
        let eq8_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, var_qgs);
        (eq8_e128, (var_qgs_dn0 * ddt_scale), (var_qgs_dn1 * ddt_scale), (var_qgs_dn2 * ddt_scale), (var_qgs_dn3 * ddt_scale), (var_qgs_dn4 * ddt_scale), (var_qgs_dn5 * ddt_scale), (var_qgs_dn6 * ddt_scale), (var_qgs_dn7 * ddt_scale), (var_qgs_dn8 * ddt_scale), (var_qgs_dn9 * ddt_scale), (var_qgs_dn10 * ddt_scale), (var_qgs_dn11 * ddt_scale), (var_qgs_dn12 * ddt_scale), (var_qgs_dn13 * ddt_scale), (var_qgs_dn14 * ddt_scale), (var_qgs_dn15 * ddt_scale), (var_qgs_dn16 * ddt_scale), (var_qgs_dn17 * ddt_scale), (var_qgs_dn18 * ddt_scale), (var_qgs_db0 * ddt_scale), (var_qgs_db1 * ddt_scale), (var_qgs_db2 * ddt_scale), (var_qgs_db3 * ddt_scale), (var_qgs_db4 * ddt_scale), (var_qgs_db5 * ddt_scale), (var_qgs_db6 * ddt_scale), (var_qgs_db7 * ddt_scale), (var_qgs_db8 * ddt_scale), (var_qgs_db9 * ddt_scale), (var_qgs_db10 * ddt_scale), (var_qgs_db11 * ddt_scale), (var_qgs_db12 * ddt_scale), (var_qgs_db13 * ddt_scale), (var_qgs_db14 * ddt_scale), (var_qgs_db15 * ddt_scale), (var_qgs_db16 * ddt_scale), (var_qgs_db17 * ddt_scale), (var_qgs_db18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e130;
        let eq8_node_derivatives: [f64; 19] = [eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18];
        let eq8_branch_derivatives: [f64; 19] = [eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18, eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18,) = {
    if (var_guard19 == 0.0) {
        let eq9_e135: f64 = (var_cgd * var_vgdc);
        let eq9_e135_d_n0: f64 = ((var_cgd_dn0 * var_vgdc) + (var_cgd * var_vgdc_dn0));
        let eq9_e135_d_n1: f64 = ((var_cgd_dn1 * var_vgdc) + (var_cgd * var_vgdc_dn1));
        let eq9_e135_d_n2: f64 = ((var_cgd_dn2 * var_vgdc) + (var_cgd * var_vgdc_dn2));
        let eq9_e135_d_n3: f64 = ((var_cgd_dn3 * var_vgdc) + (var_cgd * var_vgdc_dn3));
        let eq9_e135_d_n4: f64 = ((var_cgd_dn4 * var_vgdc) + (var_cgd * var_vgdc_dn4));
        let eq9_e135_d_n5: f64 = ((var_cgd_dn5 * var_vgdc) + (var_cgd * var_vgdc_dn5));
        let eq9_e135_d_n6: f64 = ((var_cgd_dn6 * var_vgdc) + (var_cgd * var_vgdc_dn6));
        let eq9_e135_d_n7: f64 = ((var_cgd_dn7 * var_vgdc) + (var_cgd * var_vgdc_dn7));
        let eq9_e135_d_n8: f64 = ((var_cgd_dn8 * var_vgdc) + (var_cgd * var_vgdc_dn8));
        let eq9_e135_d_n9: f64 = ((var_cgd_dn9 * var_vgdc) + (var_cgd * var_vgdc_dn9));
        let eq9_e135_d_n10: f64 = ((var_cgd_dn10 * var_vgdc) + (var_cgd * var_vgdc_dn10));
        let eq9_e135_d_n11: f64 = ((var_cgd_dn11 * var_vgdc) + (var_cgd * var_vgdc_dn11));
        let eq9_e135_d_n12: f64 = ((var_cgd_dn12 * var_vgdc) + (var_cgd * var_vgdc_dn12));
        let eq9_e135_d_n13: f64 = ((var_cgd_dn13 * var_vgdc) + (var_cgd * var_vgdc_dn13));
        let eq9_e135_d_n14: f64 = ((var_cgd_dn14 * var_vgdc) + (var_cgd * var_vgdc_dn14));
        let eq9_e135_d_n15: f64 = ((var_cgd_dn15 * var_vgdc) + (var_cgd * var_vgdc_dn15));
        let eq9_e135_d_n16: f64 = ((var_cgd_dn16 * var_vgdc) + (var_cgd * var_vgdc_dn16));
        let eq9_e135_d_n17: f64 = ((var_cgd_dn17 * var_vgdc) + (var_cgd * var_vgdc_dn17));
        let eq9_e135_d_n18: f64 = ((var_cgd_dn18 * var_vgdc) + (var_cgd * var_vgdc_dn18));
        let eq9_e135_d_b0: f64 = ((var_cgd_db0 * var_vgdc) + (var_cgd * var_vgdc_db0));
        let eq9_e135_d_b1: f64 = ((var_cgd_db1 * var_vgdc) + (var_cgd * var_vgdc_db1));
        let eq9_e135_d_b2: f64 = ((var_cgd_db2 * var_vgdc) + (var_cgd * var_vgdc_db2));
        let eq9_e135_d_b3: f64 = ((var_cgd_db3 * var_vgdc) + (var_cgd * var_vgdc_db3));
        let eq9_e135_d_b4: f64 = ((var_cgd_db4 * var_vgdc) + (var_cgd * var_vgdc_db4));
        let eq9_e135_d_b5: f64 = ((var_cgd_db5 * var_vgdc) + (var_cgd * var_vgdc_db5));
        let eq9_e135_d_b6: f64 = ((var_cgd_db6 * var_vgdc) + (var_cgd * var_vgdc_db6));
        let eq9_e135_d_b7: f64 = ((var_cgd_db7 * var_vgdc) + (var_cgd * var_vgdc_db7));
        let eq9_e135_d_b8: f64 = ((var_cgd_db8 * var_vgdc) + (var_cgd * var_vgdc_db8));
        let eq9_e135_d_b9: f64 = ((var_cgd_db9 * var_vgdc) + (var_cgd * var_vgdc_db9));
        let eq9_e135_d_b10: f64 = ((var_cgd_db10 * var_vgdc) + (var_cgd * var_vgdc_db10));
        let eq9_e135_d_b11: f64 = ((var_cgd_db11 * var_vgdc) + (var_cgd * var_vgdc_db11));
        let eq9_e135_d_b12: f64 = ((var_cgd_db12 * var_vgdc) + (var_cgd * var_vgdc_db12));
        let eq9_e135_d_b13: f64 = ((var_cgd_db13 * var_vgdc) + (var_cgd * var_vgdc_db13));
        let eq9_e135_d_b14: f64 = ((var_cgd_db14 * var_vgdc) + (var_cgd * var_vgdc_db14));
        let eq9_e135_d_b15: f64 = ((var_cgd_db15 * var_vgdc) + (var_cgd * var_vgdc_db15));
        let eq9_e135_d_b16: f64 = ((var_cgd_db16 * var_vgdc) + (var_cgd * var_vgdc_db16));
        let eq9_e135_d_b17: f64 = ((var_cgd_db17 * var_vgdc) + (var_cgd * var_vgdc_db17));
        let eq9_e135_d_b18: f64 = ((var_cgd_db18 * var_vgdc) + (var_cgd * var_vgdc_db18));
        let eq9_e136: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq9_e135);
        (eq9_e136, (eq9_e135_d_n0 * ddt_scale), (eq9_e135_d_n1 * ddt_scale), (eq9_e135_d_n2 * ddt_scale), (eq9_e135_d_n3 * ddt_scale), (eq9_e135_d_n4 * ddt_scale), (eq9_e135_d_n5 * ddt_scale), (eq9_e135_d_n6 * ddt_scale), (eq9_e135_d_n7 * ddt_scale), (eq9_e135_d_n8 * ddt_scale), (eq9_e135_d_n9 * ddt_scale), (eq9_e135_d_n10 * ddt_scale), (eq9_e135_d_n11 * ddt_scale), (eq9_e135_d_n12 * ddt_scale), (eq9_e135_d_n13 * ddt_scale), (eq9_e135_d_n14 * ddt_scale), (eq9_e135_d_n15 * ddt_scale), (eq9_e135_d_n16 * ddt_scale), (eq9_e135_d_n17 * ddt_scale), (eq9_e135_d_n18 * ddt_scale), (eq9_e135_d_b0 * ddt_scale), (eq9_e135_d_b1 * ddt_scale), (eq9_e135_d_b2 * ddt_scale), (eq9_e135_d_b3 * ddt_scale), (eq9_e135_d_b4 * ddt_scale), (eq9_e135_d_b5 * ddt_scale), (eq9_e135_d_b6 * ddt_scale), (eq9_e135_d_b7 * ddt_scale), (eq9_e135_d_b8 * ddt_scale), (eq9_e135_d_b9 * ddt_scale), (eq9_e135_d_b10 * ddt_scale), (eq9_e135_d_b11 * ddt_scale), (eq9_e135_d_b12 * ddt_scale), (eq9_e135_d_b13 * ddt_scale), (eq9_e135_d_b14 * ddt_scale), (eq9_e135_d_b15 * ddt_scale), (eq9_e135_d_b16 * ddt_scale), (eq9_e135_d_b17 * ddt_scale), (eq9_e135_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e138;
        let eq9_node_derivatives: [f64; 19] = [eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18];
        let eq9_branch_derivatives: [f64; 19] = [eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18, eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18,) = {
    if (var_guard19 == 0.0) {
        let eq10_e143: f64 = (var_cgs * var_vgsc);
        let eq10_e143_d_n0: f64 = ((var_cgs_dn0 * var_vgsc) + (var_cgs * var_vgsc_dn0));
        let eq10_e143_d_n1: f64 = ((var_cgs_dn1 * var_vgsc) + (var_cgs * var_vgsc_dn1));
        let eq10_e143_d_n2: f64 = ((var_cgs_dn2 * var_vgsc) + (var_cgs * var_vgsc_dn2));
        let eq10_e143_d_n3: f64 = ((var_cgs_dn3 * var_vgsc) + (var_cgs * var_vgsc_dn3));
        let eq10_e143_d_n4: f64 = ((var_cgs_dn4 * var_vgsc) + (var_cgs * var_vgsc_dn4));
        let eq10_e143_d_n5: f64 = ((var_cgs_dn5 * var_vgsc) + (var_cgs * var_vgsc_dn5));
        let eq10_e143_d_n6: f64 = ((var_cgs_dn6 * var_vgsc) + (var_cgs * var_vgsc_dn6));
        let eq10_e143_d_n7: f64 = ((var_cgs_dn7 * var_vgsc) + (var_cgs * var_vgsc_dn7));
        let eq10_e143_d_n8: f64 = ((var_cgs_dn8 * var_vgsc) + (var_cgs * var_vgsc_dn8));
        let eq10_e143_d_n9: f64 = ((var_cgs_dn9 * var_vgsc) + (var_cgs * var_vgsc_dn9));
        let eq10_e143_d_n10: f64 = ((var_cgs_dn10 * var_vgsc) + (var_cgs * var_vgsc_dn10));
        let eq10_e143_d_n11: f64 = ((var_cgs_dn11 * var_vgsc) + (var_cgs * var_vgsc_dn11));
        let eq10_e143_d_n12: f64 = ((var_cgs_dn12 * var_vgsc) + (var_cgs * var_vgsc_dn12));
        let eq10_e143_d_n13: f64 = ((var_cgs_dn13 * var_vgsc) + (var_cgs * var_vgsc_dn13));
        let eq10_e143_d_n14: f64 = ((var_cgs_dn14 * var_vgsc) + (var_cgs * var_vgsc_dn14));
        let eq10_e143_d_n15: f64 = ((var_cgs_dn15 * var_vgsc) + (var_cgs * var_vgsc_dn15));
        let eq10_e143_d_n16: f64 = ((var_cgs_dn16 * var_vgsc) + (var_cgs * var_vgsc_dn16));
        let eq10_e143_d_n17: f64 = ((var_cgs_dn17 * var_vgsc) + (var_cgs * var_vgsc_dn17));
        let eq10_e143_d_n18: f64 = ((var_cgs_dn18 * var_vgsc) + (var_cgs * var_vgsc_dn18));
        let eq10_e143_d_b0: f64 = ((var_cgs_db0 * var_vgsc) + (var_cgs * var_vgsc_db0));
        let eq10_e143_d_b1: f64 = ((var_cgs_db1 * var_vgsc) + (var_cgs * var_vgsc_db1));
        let eq10_e143_d_b2: f64 = ((var_cgs_db2 * var_vgsc) + (var_cgs * var_vgsc_db2));
        let eq10_e143_d_b3: f64 = ((var_cgs_db3 * var_vgsc) + (var_cgs * var_vgsc_db3));
        let eq10_e143_d_b4: f64 = ((var_cgs_db4 * var_vgsc) + (var_cgs * var_vgsc_db4));
        let eq10_e143_d_b5: f64 = ((var_cgs_db5 * var_vgsc) + (var_cgs * var_vgsc_db5));
        let eq10_e143_d_b6: f64 = ((var_cgs_db6 * var_vgsc) + (var_cgs * var_vgsc_db6));
        let eq10_e143_d_b7: f64 = ((var_cgs_db7 * var_vgsc) + (var_cgs * var_vgsc_db7));
        let eq10_e143_d_b8: f64 = ((var_cgs_db8 * var_vgsc) + (var_cgs * var_vgsc_db8));
        let eq10_e143_d_b9: f64 = ((var_cgs_db9 * var_vgsc) + (var_cgs * var_vgsc_db9));
        let eq10_e143_d_b10: f64 = ((var_cgs_db10 * var_vgsc) + (var_cgs * var_vgsc_db10));
        let eq10_e143_d_b11: f64 = ((var_cgs_db11 * var_vgsc) + (var_cgs * var_vgsc_db11));
        let eq10_e143_d_b12: f64 = ((var_cgs_db12 * var_vgsc) + (var_cgs * var_vgsc_db12));
        let eq10_e143_d_b13: f64 = ((var_cgs_db13 * var_vgsc) + (var_cgs * var_vgsc_db13));
        let eq10_e143_d_b14: f64 = ((var_cgs_db14 * var_vgsc) + (var_cgs * var_vgsc_db14));
        let eq10_e143_d_b15: f64 = ((var_cgs_db15 * var_vgsc) + (var_cgs * var_vgsc_db15));
        let eq10_e143_d_b16: f64 = ((var_cgs_db16 * var_vgsc) + (var_cgs * var_vgsc_db16));
        let eq10_e143_d_b17: f64 = ((var_cgs_db17 * var_vgsc) + (var_cgs * var_vgsc_db17));
        let eq10_e143_d_b18: f64 = ((var_cgs_db18 * var_vgsc) + (var_cgs * var_vgsc_db18));
        let eq10_e144: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq10_e143);
        (eq10_e144, (eq10_e143_d_n0 * ddt_scale), (eq10_e143_d_n1 * ddt_scale), (eq10_e143_d_n2 * ddt_scale), (eq10_e143_d_n3 * ddt_scale), (eq10_e143_d_n4 * ddt_scale), (eq10_e143_d_n5 * ddt_scale), (eq10_e143_d_n6 * ddt_scale), (eq10_e143_d_n7 * ddt_scale), (eq10_e143_d_n8 * ddt_scale), (eq10_e143_d_n9 * ddt_scale), (eq10_e143_d_n10 * ddt_scale), (eq10_e143_d_n11 * ddt_scale), (eq10_e143_d_n12 * ddt_scale), (eq10_e143_d_n13 * ddt_scale), (eq10_e143_d_n14 * ddt_scale), (eq10_e143_d_n15 * ddt_scale), (eq10_e143_d_n16 * ddt_scale), (eq10_e143_d_n17 * ddt_scale), (eq10_e143_d_n18 * ddt_scale), (eq10_e143_d_b0 * ddt_scale), (eq10_e143_d_b1 * ddt_scale), (eq10_e143_d_b2 * ddt_scale), (eq10_e143_d_b3 * ddt_scale), (eq10_e143_d_b4 * ddt_scale), (eq10_e143_d_b5 * ddt_scale), (eq10_e143_d_b6 * ddt_scale), (eq10_e143_d_b7 * ddt_scale), (eq10_e143_d_b8 * ddt_scale), (eq10_e143_d_b9 * ddt_scale), (eq10_e143_d_b10 * ddt_scale), (eq10_e143_d_b11 * ddt_scale), (eq10_e143_d_b12 * ddt_scale), (eq10_e143_d_b13 * ddt_scale), (eq10_e143_d_b14 * ddt_scale), (eq10_e143_d_b15 * ddt_scale), (eq10_e143_d_b16 * ddt_scale), (eq10_e143_d_b17 * ddt_scale), (eq10_e143_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e146;
        let eq10_node_derivatives: [f64; 19] = [eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18];
        let eq10_branch_derivatives: [f64; 19] = [eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq15_e169, eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18, eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18,) = {
    if (var_guard20 != 0.0) {
        let eq15_e165: f64 = (bi1 * var_rc1);
        let eq15_e165_d_n0: f64 = (bi1 * var_rc1_dn0);
        let eq15_e165_d_n1: f64 = (bi1 * var_rc1_dn1);
        let eq15_e165_d_n2: f64 = (bi1 * var_rc1_dn2);
        let eq15_e165_d_n3: f64 = (bi1 * var_rc1_dn3);
        let eq15_e165_d_n4: f64 = (bi1 * var_rc1_dn4);
        let eq15_e165_d_n5: f64 = (bi1 * var_rc1_dn5);
        let eq15_e165_d_n6: f64 = (bi1 * var_rc1_dn6);
        let eq15_e165_d_n7: f64 = (bi1 * var_rc1_dn7);
        let eq15_e165_d_n8: f64 = (bi1 * var_rc1_dn8);
        let eq15_e165_d_n9: f64 = (bi1 * var_rc1_dn9);
        let eq15_e165_d_n10: f64 = (bi1 * var_rc1_dn10);
        let eq15_e165_d_n11: f64 = (bi1 * var_rc1_dn11);
        let eq15_e165_d_n12: f64 = (bi1 * var_rc1_dn12);
        let eq15_e165_d_n13: f64 = (bi1 * var_rc1_dn13);
        let eq15_e165_d_n14: f64 = (bi1 * var_rc1_dn14);
        let eq15_e165_d_n15: f64 = (bi1 * var_rc1_dn15);
        let eq15_e165_d_n16: f64 = (bi1 * var_rc1_dn16);
        let eq15_e165_d_n17: f64 = (bi1 * var_rc1_dn17);
        let eq15_e165_d_n18: f64 = (bi1 * var_rc1_dn18);
        let eq15_e165_d_b0: f64 = (bi1 * var_rc1_db0);
        let eq15_e165_d_b1: f64 = (var_rc1 + (bi1 * var_rc1_db1));
        let eq15_e165_d_b2: f64 = (bi1 * var_rc1_db2);
        let eq15_e165_d_b3: f64 = (bi1 * var_rc1_db3);
        let eq15_e165_d_b4: f64 = (bi1 * var_rc1_db4);
        let eq15_e165_d_b5: f64 = (bi1 * var_rc1_db5);
        let eq15_e165_d_b6: f64 = (bi1 * var_rc1_db6);
        let eq15_e165_d_b7: f64 = (bi1 * var_rc1_db7);
        let eq15_e165_d_b8: f64 = (bi1 * var_rc1_db8);
        let eq15_e165_d_b9: f64 = (bi1 * var_rc1_db9);
        let eq15_e165_d_b10: f64 = (bi1 * var_rc1_db10);
        let eq15_e165_d_b11: f64 = (bi1 * var_rc1_db11);
        let eq15_e165_d_b12: f64 = (bi1 * var_rc1_db12);
        let eq15_e165_d_b13: f64 = (bi1 * var_rc1_db13);
        let eq15_e165_d_b14: f64 = (bi1 * var_rc1_db14);
        let eq15_e165_d_b15: f64 = (bi1 * var_rc1_db15);
        let eq15_e165_d_b16: f64 = (bi1 * var_rc1_db16);
        let eq15_e165_d_b17: f64 = (bi1 * var_rc1_db17);
        let eq15_e165_d_b18: f64 = (bi1 * var_rc1_db18);
        let eq15_e167: f64 = (eq15_e165 + var_t0);
        let eq15_e167_d_n0: f64 = (eq15_e165_d_n0 + var_t0_dn0);
        let eq15_e167_d_n1: f64 = (eq15_e165_d_n1 + var_t0_dn1);
        let eq15_e167_d_n2: f64 = (eq15_e165_d_n2 + var_t0_dn2);
        let eq15_e167_d_n3: f64 = (eq15_e165_d_n3 + var_t0_dn3);
        let eq15_e167_d_n4: f64 = (eq15_e165_d_n4 + var_t0_dn4);
        let eq15_e167_d_n5: f64 = (eq15_e165_d_n5 + var_t0_dn5);
        let eq15_e167_d_n6: f64 = (eq15_e165_d_n6 + var_t0_dn6);
        let eq15_e167_d_n7: f64 = (eq15_e165_d_n7 + var_t0_dn7);
        let eq15_e167_d_n8: f64 = (eq15_e165_d_n8 + var_t0_dn8);
        let eq15_e167_d_n9: f64 = (eq15_e165_d_n9 + var_t0_dn9);
        let eq15_e167_d_n10: f64 = (eq15_e165_d_n10 + var_t0_dn10);
        let eq15_e167_d_n11: f64 = (eq15_e165_d_n11 + var_t0_dn11);
        let eq15_e167_d_n12: f64 = (eq15_e165_d_n12 + var_t0_dn12);
        let eq15_e167_d_n13: f64 = (eq15_e165_d_n13 + var_t0_dn13);
        let eq15_e167_d_n14: f64 = (eq15_e165_d_n14 + var_t0_dn14);
        let eq15_e167_d_n15: f64 = (eq15_e165_d_n15 + var_t0_dn15);
        let eq15_e167_d_n16: f64 = (eq15_e165_d_n16 + var_t0_dn16);
        let eq15_e167_d_n17: f64 = (eq15_e165_d_n17 + var_t0_dn17);
        let eq15_e167_d_n18: f64 = (eq15_e165_d_n18 + var_t0_dn18);
        let eq15_e167_d_b0: f64 = (eq15_e165_d_b0 + var_t0_db0);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + var_t0_db1);
        let eq15_e167_d_b2: f64 = (eq15_e165_d_b2 + var_t0_db2);
        let eq15_e167_d_b3: f64 = (eq15_e165_d_b3 + var_t0_db3);
        let eq15_e167_d_b4: f64 = (eq15_e165_d_b4 + var_t0_db4);
        let eq15_e167_d_b5: f64 = (eq15_e165_d_b5 + var_t0_db5);
        let eq15_e167_d_b6: f64 = (eq15_e165_d_b6 + var_t0_db6);
        let eq15_e167_d_b7: f64 = (eq15_e165_d_b7 + var_t0_db7);
        let eq15_e167_d_b8: f64 = (eq15_e165_d_b8 + var_t0_db8);
        let eq15_e167_d_b9: f64 = (eq15_e165_d_b9 + var_t0_db9);
        let eq15_e167_d_b10: f64 = (eq15_e165_d_b10 + var_t0_db10);
        let eq15_e167_d_b11: f64 = (eq15_e165_d_b11 + var_t0_db11);
        let eq15_e167_d_b12: f64 = (eq15_e165_d_b12 + var_t0_db12);
        let eq15_e167_d_b13: f64 = (eq15_e165_d_b13 + var_t0_db13);
        let eq15_e167_d_b14: f64 = (eq15_e165_d_b14 + var_t0_db14);
        let eq15_e167_d_b15: f64 = (eq15_e165_d_b15 + var_t0_db15);
        let eq15_e167_d_b16: f64 = (eq15_e165_d_b16 + var_t0_db16);
        let eq15_e167_d_b17: f64 = (eq15_e165_d_b17 + var_t0_db17);
        let eq15_e167_d_b18: f64 = (eq15_e165_d_b18 + var_t0_db18);
        (eq15_e167, eq15_e167_d_n0, eq15_e167_d_n1, eq15_e167_d_n2, eq15_e167_d_n3, eq15_e167_d_n4, eq15_e167_d_n5, eq15_e167_d_n6, eq15_e167_d_n7, eq15_e167_d_n8, eq15_e167_d_n9, eq15_e167_d_n10, eq15_e167_d_n11, eq15_e167_d_n12, eq15_e167_d_n13, eq15_e167_d_n14, eq15_e167_d_n15, eq15_e167_d_n16, eq15_e167_d_n17, eq15_e167_d_n18, eq15_e167_d_b0, eq15_e167_d_b1, eq15_e167_d_b2, eq15_e167_d_b3, eq15_e167_d_b4, eq15_e167_d_b5, eq15_e167_d_b6, eq15_e167_d_b7, eq15_e167_d_b8, eq15_e167_d_b9, eq15_e167_d_b10, eq15_e167_d_b11, eq15_e167_d_b12, eq15_e167_d_b13, eq15_e167_d_b14, eq15_e167_d_b15, eq15_e167_d_b16, eq15_e167_d_b17, eq15_e167_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e169;
        let eq15_node_derivatives: [f64; 19] = [eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18];
        let eq15_branch_derivatives: [f64; 19] = [eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18];
        stamper.stamp_potential_dense_local(
            1,
            eq15_value,
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
        );
        let (eq18_e187, eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18, eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18,) = {
    if (var_guard21 != 0.0) {
        let eq18_e184: f64 = (var_cdel_t * (nv12 - nv8));
        let eq18_e184_d_n0: f64 = (var_cdel_t_dn0 * (nv12 - nv8));
        let eq18_e184_d_n1: f64 = (var_cdel_t_dn1 * (nv12 - nv8));
        let eq18_e184_d_n2: f64 = (var_cdel_t_dn2 * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (var_cdel_t_dn3 * (nv12 - nv8));
        let eq18_e184_d_n4: f64 = (var_cdel_t_dn4 * (nv12 - nv8));
        let eq18_e184_d_n5: f64 = (var_cdel_t_dn5 * (nv12 - nv8));
        let eq18_e184_d_n6: f64 = (var_cdel_t_dn6 * (nv12 - nv8));
        let eq18_e184_d_n7: f64 = (var_cdel_t_dn7 * (nv12 - nv8));
        let eq18_e184_d_n8: f64 = ((var_cdel_t_dn8 * (nv12 - nv8)) + (-var_cdel_t));
        let eq18_e184_d_n9: f64 = (var_cdel_t_dn9 * (nv12 - nv8));
        let eq18_e184_d_n10: f64 = (var_cdel_t_dn10 * (nv12 - nv8));
        let eq18_e184_d_n11: f64 = (var_cdel_t_dn11 * (nv12 - nv8));
        let eq18_e184_d_n12: f64 = ((var_cdel_t_dn12 * (nv12 - nv8)) + var_cdel_t);
        let eq18_e184_d_n13: f64 = (var_cdel_t_dn13 * (nv12 - nv8));
        let eq18_e184_d_n14: f64 = (var_cdel_t_dn14 * (nv12 - nv8));
        let eq18_e184_d_n15: f64 = (var_cdel_t_dn15 * (nv12 - nv8));
        let eq18_e184_d_n16: f64 = (var_cdel_t_dn16 * (nv12 - nv8));
        let eq18_e184_d_n17: f64 = (var_cdel_t_dn17 * (nv12 - nv8));
        let eq18_e184_d_n18: f64 = (var_cdel_t_dn18 * (nv12 - nv8));
        let eq18_e184_d_b0: f64 = (var_cdel_t_db0 * (nv12 - nv8));
        let eq18_e184_d_b1: f64 = (var_cdel_t_db1 * (nv12 - nv8));
        let eq18_e184_d_b2: f64 = (var_cdel_t_db2 * (nv12 - nv8));
        let eq18_e184_d_b3: f64 = (var_cdel_t_db3 * (nv12 - nv8));
        let eq18_e184_d_b4: f64 = (var_cdel_t_db4 * (nv12 - nv8));
        let eq18_e184_d_b5: f64 = (var_cdel_t_db5 * (nv12 - nv8));
        let eq18_e184_d_b6: f64 = (var_cdel_t_db6 * (nv12 - nv8));
        let eq18_e184_d_b7: f64 = (var_cdel_t_db7 * (nv12 - nv8));
        let eq18_e184_d_b8: f64 = (var_cdel_t_db8 * (nv12 - nv8));
        let eq18_e184_d_b9: f64 = (var_cdel_t_db9 * (nv12 - nv8));
        let eq18_e184_d_b10: f64 = (var_cdel_t_db10 * (nv12 - nv8));
        let eq18_e184_d_b11: f64 = (var_cdel_t_db11 * (nv12 - nv8));
        let eq18_e184_d_b12: f64 = (var_cdel_t_db12 * (nv12 - nv8));
        let eq18_e184_d_b13: f64 = (var_cdel_t_db13 * (nv12 - nv8));
        let eq18_e184_d_b14: f64 = (var_cdel_t_db14 * (nv12 - nv8));
        let eq18_e184_d_b15: f64 = (var_cdel_t_db15 * (nv12 - nv8));
        let eq18_e184_d_b16: f64 = (var_cdel_t_db16 * (nv12 - nv8));
        let eq18_e184_d_b17: f64 = (var_cdel_t_db17 * (nv12 - nv8));
        let eq18_e184_d_b18: f64 = (var_cdel_t_db18 * (nv12 - nv8));
        let eq18_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq18_e184);
        (eq18_e185, (eq18_e184_d_n0 * ddt_scale), (eq18_e184_d_n1 * ddt_scale), (eq18_e184_d_n2 * ddt_scale), (eq18_e184_d_n3 * ddt_scale), (eq18_e184_d_n4 * ddt_scale), (eq18_e184_d_n5 * ddt_scale), (eq18_e184_d_n6 * ddt_scale), (eq18_e184_d_n7 * ddt_scale), (eq18_e184_d_n8 * ddt_scale), (eq18_e184_d_n9 * ddt_scale), (eq18_e184_d_n10 * ddt_scale), (eq18_e184_d_n11 * ddt_scale), (eq18_e184_d_n12 * ddt_scale), (eq18_e184_d_n13 * ddt_scale), (eq18_e184_d_n14 * ddt_scale), (eq18_e184_d_n15 * ddt_scale), (eq18_e184_d_n16 * ddt_scale), (eq18_e184_d_n17 * ddt_scale), (eq18_e184_d_n18 * ddt_scale), (eq18_e184_d_b0 * ddt_scale), (eq18_e184_d_b1 * ddt_scale), (eq18_e184_d_b2 * ddt_scale), (eq18_e184_d_b3 * ddt_scale), (eq18_e184_d_b4 * ddt_scale), (eq18_e184_d_b5 * ddt_scale), (eq18_e184_d_b6 * ddt_scale), (eq18_e184_d_b7 * ddt_scale), (eq18_e184_d_b8 * ddt_scale), (eq18_e184_d_b9 * ddt_scale), (eq18_e184_d_b10 * ddt_scale), (eq18_e184_d_b11 * ddt_scale), (eq18_e184_d_b12 * ddt_scale), (eq18_e184_d_b13 * ddt_scale), (eq18_e184_d_b14 * ddt_scale), (eq18_e184_d_b15 * ddt_scale), (eq18_e184_d_b16 * ddt_scale), (eq18_e184_d_b17 * ddt_scale), (eq18_e184_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e187;
        let eq18_node_derivatives: [f64; 19] = [eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18];
        let eq18_branch_derivatives: [f64; 19] = [eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq28_e247, eq28_e247_d_b7,) = {
    if (var_guard25 != 0.0) {
        let eq28_e245: f64 = (bi7 * p.p46);
        (eq28_e245, p.p46,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e247;
        stamper.stamp_potential_branch1_local(
            7,
            eq28_value,
            7,
            eq28_e247_d_b7,
        );
        let (eq29_e261,) = {
    if ((var_guard25 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e261;
        stamper.stamp_potential_const_local(
            8,
            eq29_value,
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq31_e269);
        let eq31_value: f64 = eq31_e270;
        stamper.stamp_potential_branch1_local(
            10,
            eq31_value,
            10,
            (p.p54 * ddt_scale),
        );
        let (eq32_e276, eq32_e276_d_n0, eq32_e276_d_n1, eq32_e276_d_n2, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n6, eq32_e276_d_n7, eq32_e276_d_n8, eq32_e276_d_n9, eq32_e276_d_n10, eq32_e276_d_n11, eq32_e276_d_n12, eq32_e276_d_n13, eq32_e276_d_n14, eq32_e276_d_n15, eq32_e276_d_n16, eq32_e276_d_n17, eq32_e276_d_n18, eq32_e276_d_b0, eq32_e276_d_b1, eq32_e276_d_b2, eq32_e276_d_b3, eq32_e276_d_b4, eq32_e276_d_b5, eq32_e276_d_b6, eq32_e276_d_b7, eq32_e276_d_b8, eq32_e276_d_b9, eq32_e276_d_b10, eq32_e276_d_b11, eq32_e276_d_b12, eq32_e276_d_b13, eq32_e276_d_b14, eq32_e276_d_b15, eq32_e276_d_b16, eq32_e276_d_b17, eq32_e276_d_b18,) = {
    if (var_guard26 != 0.0) {
        let eq32_e274: f64 = (bi11 * var_rs_t);
        let eq32_e274_d_n0: f64 = (bi11 * var_rs_t_dn0);
        let eq32_e274_d_n1: f64 = (bi11 * var_rs_t_dn1);
        let eq32_e274_d_n2: f64 = (bi11 * var_rs_t_dn2);
        let eq32_e274_d_n3: f64 = (bi11 * var_rs_t_dn3);
        let eq32_e274_d_n4: f64 = (bi11 * var_rs_t_dn4);
        let eq32_e274_d_n5: f64 = (bi11 * var_rs_t_dn5);
        let eq32_e274_d_n6: f64 = (bi11 * var_rs_t_dn6);
        let eq32_e274_d_n7: f64 = (bi11 * var_rs_t_dn7);
        let eq32_e274_d_n8: f64 = (bi11 * var_rs_t_dn8);
        let eq32_e274_d_n9: f64 = (bi11 * var_rs_t_dn9);
        let eq32_e274_d_n10: f64 = (bi11 * var_rs_t_dn10);
        let eq32_e274_d_n11: f64 = (bi11 * var_rs_t_dn11);
        let eq32_e274_d_n12: f64 = (bi11 * var_rs_t_dn12);
        let eq32_e274_d_n13: f64 = (bi11 * var_rs_t_dn13);
        let eq32_e274_d_n14: f64 = (bi11 * var_rs_t_dn14);
        let eq32_e274_d_n15: f64 = (bi11 * var_rs_t_dn15);
        let eq32_e274_d_n16: f64 = (bi11 * var_rs_t_dn16);
        let eq32_e274_d_n17: f64 = (bi11 * var_rs_t_dn17);
        let eq32_e274_d_n18: f64 = (bi11 * var_rs_t_dn18);
        let eq32_e274_d_b0: f64 = (bi11 * var_rs_t_db0);
        let eq32_e274_d_b1: f64 = (bi11 * var_rs_t_db1);
        let eq32_e274_d_b2: f64 = (bi11 * var_rs_t_db2);
        let eq32_e274_d_b3: f64 = (bi11 * var_rs_t_db3);
        let eq32_e274_d_b4: f64 = (bi11 * var_rs_t_db4);
        let eq32_e274_d_b5: f64 = (bi11 * var_rs_t_db5);
        let eq32_e274_d_b6: f64 = (bi11 * var_rs_t_db6);
        let eq32_e274_d_b7: f64 = (bi11 * var_rs_t_db7);
        let eq32_e274_d_b8: f64 = (bi11 * var_rs_t_db8);
        let eq32_e274_d_b9: f64 = (bi11 * var_rs_t_db9);
        let eq32_e274_d_b10: f64 = (bi11 * var_rs_t_db10);
        let eq32_e274_d_b11: f64 = (var_rs_t + (bi11 * var_rs_t_db11));
        let eq32_e274_d_b12: f64 = (bi11 * var_rs_t_db12);
        let eq32_e274_d_b13: f64 = (bi11 * var_rs_t_db13);
        let eq32_e274_d_b14: f64 = (bi11 * var_rs_t_db14);
        let eq32_e274_d_b15: f64 = (bi11 * var_rs_t_db15);
        let eq32_e274_d_b16: f64 = (bi11 * var_rs_t_db16);
        let eq32_e274_d_b17: f64 = (bi11 * var_rs_t_db17);
        let eq32_e274_d_b18: f64 = (bi11 * var_rs_t_db18);
        (eq32_e274, eq32_e274_d_n0, eq32_e274_d_n1, eq32_e274_d_n2, eq32_e274_d_n3, eq32_e274_d_n4, eq32_e274_d_n5, eq32_e274_d_n6, eq32_e274_d_n7, eq32_e274_d_n8, eq32_e274_d_n9, eq32_e274_d_n10, eq32_e274_d_n11, eq32_e274_d_n12, eq32_e274_d_n13, eq32_e274_d_n14, eq32_e274_d_n15, eq32_e274_d_n16, eq32_e274_d_n17, eq32_e274_d_n18, eq32_e274_d_b0, eq32_e274_d_b1, eq32_e274_d_b2, eq32_e274_d_b3, eq32_e274_d_b4, eq32_e274_d_b5, eq32_e274_d_b6, eq32_e274_d_b7, eq32_e274_d_b8, eq32_e274_d_b9, eq32_e274_d_b10, eq32_e274_d_b11, eq32_e274_d_b12, eq32_e274_d_b13, eq32_e274_d_b14, eq32_e274_d_b15, eq32_e274_d_b16, eq32_e274_d_b17, eq32_e274_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e276;
        let eq32_node_derivatives: [f64; 19] = [eq32_e276_d_n0, eq32_e276_d_n1, eq32_e276_d_n2, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n6, eq32_e276_d_n7, eq32_e276_d_n8, eq32_e276_d_n9, eq32_e276_d_n10, eq32_e276_d_n11, eq32_e276_d_n12, eq32_e276_d_n13, eq32_e276_d_n14, eq32_e276_d_n15, eq32_e276_d_n16, eq32_e276_d_n17, eq32_e276_d_n18];
        let eq32_branch_derivatives: [f64; 19] = [eq32_e276_d_b0, eq32_e276_d_b1, eq32_e276_d_b2, eq32_e276_d_b3, eq32_e276_d_b4, eq32_e276_d_b5, eq32_e276_d_b6, eq32_e276_d_b7, eq32_e276_d_b8, eq32_e276_d_b9, eq32_e276_d_b10, eq32_e276_d_b11, eq32_e276_d_b12, eq32_e276_d_b13, eq32_e276_d_b14, eq32_e276_d_b15, eq32_e276_d_b16, eq32_e276_d_b17, eq32_e276_d_b18];
        stamper.stamp_potential_dense_local(
            11,
            eq32_value,
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
        );
        let (eq33_e290,) = {
    if ((var_guard26 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq33_value: f64 = eq33_e290;
        stamper.stamp_potential_const_local(
            12,
            eq33_value,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq35_e298);
        let eq35_value: f64 = eq35_e299;
        stamper.stamp_potential_branch1_local(
            14,
            eq35_value,
            14,
            (p.p53 * ddt_scale),
        );
        let (eq36_e305, eq36_e305_d_n0, eq36_e305_d_n1, eq36_e305_d_n2, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n6, eq36_e305_d_n7, eq36_e305_d_n8, eq36_e305_d_n9, eq36_e305_d_n10, eq36_e305_d_n11, eq36_e305_d_n12, eq36_e305_d_n13, eq36_e305_d_n14, eq36_e305_d_n15, eq36_e305_d_n16, eq36_e305_d_n17, eq36_e305_d_n18, eq36_e305_d_b0, eq36_e305_d_b1, eq36_e305_d_b2, eq36_e305_d_b3, eq36_e305_d_b4, eq36_e305_d_b5, eq36_e305_d_b6, eq36_e305_d_b7, eq36_e305_d_b8, eq36_e305_d_b9, eq36_e305_d_b10, eq36_e305_d_b11, eq36_e305_d_b12, eq36_e305_d_b13, eq36_e305_d_b14, eq36_e305_d_b15, eq36_e305_d_b16, eq36_e305_d_b17, eq36_e305_d_b18,) = {
    if (var_guard27 != 0.0) {
        let eq36_e303: f64 = (bi15 * var_rd1_t);
        let eq36_e303_d_n0: f64 = (bi15 * var_rd1_t_dn0);
        let eq36_e303_d_n1: f64 = (bi15 * var_rd1_t_dn1);
        let eq36_e303_d_n2: f64 = (bi15 * var_rd1_t_dn2);
        let eq36_e303_d_n3: f64 = (bi15 * var_rd1_t_dn3);
        let eq36_e303_d_n4: f64 = (bi15 * var_rd1_t_dn4);
        let eq36_e303_d_n5: f64 = (bi15 * var_rd1_t_dn5);
        let eq36_e303_d_n6: f64 = (bi15 * var_rd1_t_dn6);
        let eq36_e303_d_n7: f64 = (bi15 * var_rd1_t_dn7);
        let eq36_e303_d_n8: f64 = (bi15 * var_rd1_t_dn8);
        let eq36_e303_d_n9: f64 = (bi15 * var_rd1_t_dn9);
        let eq36_e303_d_n10: f64 = (bi15 * var_rd1_t_dn10);
        let eq36_e303_d_n11: f64 = (bi15 * var_rd1_t_dn11);
        let eq36_e303_d_n12: f64 = (bi15 * var_rd1_t_dn12);
        let eq36_e303_d_n13: f64 = (bi15 * var_rd1_t_dn13);
        let eq36_e303_d_n14: f64 = (bi15 * var_rd1_t_dn14);
        let eq36_e303_d_n15: f64 = (bi15 * var_rd1_t_dn15);
        let eq36_e303_d_n16: f64 = (bi15 * var_rd1_t_dn16);
        let eq36_e303_d_n17: f64 = (bi15 * var_rd1_t_dn17);
        let eq36_e303_d_n18: f64 = (bi15 * var_rd1_t_dn18);
        let eq36_e303_d_b0: f64 = (bi15 * var_rd1_t_db0);
        let eq36_e303_d_b1: f64 = (bi15 * var_rd1_t_db1);
        let eq36_e303_d_b2: f64 = (bi15 * var_rd1_t_db2);
        let eq36_e303_d_b3: f64 = (bi15 * var_rd1_t_db3);
        let eq36_e303_d_b4: f64 = (bi15 * var_rd1_t_db4);
        let eq36_e303_d_b5: f64 = (bi15 * var_rd1_t_db5);
        let eq36_e303_d_b6: f64 = (bi15 * var_rd1_t_db6);
        let eq36_e303_d_b7: f64 = (bi15 * var_rd1_t_db7);
        let eq36_e303_d_b8: f64 = (bi15 * var_rd1_t_db8);
        let eq36_e303_d_b9: f64 = (bi15 * var_rd1_t_db9);
        let eq36_e303_d_b10: f64 = (bi15 * var_rd1_t_db10);
        let eq36_e303_d_b11: f64 = (bi15 * var_rd1_t_db11);
        let eq36_e303_d_b12: f64 = (bi15 * var_rd1_t_db12);
        let eq36_e303_d_b13: f64 = (bi15 * var_rd1_t_db13);
        let eq36_e303_d_b14: f64 = (bi15 * var_rd1_t_db14);
        let eq36_e303_d_b15: f64 = (var_rd1_t + (bi15 * var_rd1_t_db15));
        let eq36_e303_d_b16: f64 = (bi15 * var_rd1_t_db16);
        let eq36_e303_d_b17: f64 = (bi15 * var_rd1_t_db17);
        let eq36_e303_d_b18: f64 = (bi15 * var_rd1_t_db18);
        (eq36_e303, eq36_e303_d_n0, eq36_e303_d_n1, eq36_e303_d_n2, eq36_e303_d_n3, eq36_e303_d_n4, eq36_e303_d_n5, eq36_e303_d_n6, eq36_e303_d_n7, eq36_e303_d_n8, eq36_e303_d_n9, eq36_e303_d_n10, eq36_e303_d_n11, eq36_e303_d_n12, eq36_e303_d_n13, eq36_e303_d_n14, eq36_e303_d_n15, eq36_e303_d_n16, eq36_e303_d_n17, eq36_e303_d_n18, eq36_e303_d_b0, eq36_e303_d_b1, eq36_e303_d_b2, eq36_e303_d_b3, eq36_e303_d_b4, eq36_e303_d_b5, eq36_e303_d_b6, eq36_e303_d_b7, eq36_e303_d_b8, eq36_e303_d_b9, eq36_e303_d_b10, eq36_e303_d_b11, eq36_e303_d_b12, eq36_e303_d_b13, eq36_e303_d_b14, eq36_e303_d_b15, eq36_e303_d_b16, eq36_e303_d_b17, eq36_e303_d_b18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e305;
        let eq36_node_derivatives: [f64; 19] = [eq36_e305_d_n0, eq36_e305_d_n1, eq36_e305_d_n2, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n6, eq36_e305_d_n7, eq36_e305_d_n8, eq36_e305_d_n9, eq36_e305_d_n10, eq36_e305_d_n11, eq36_e305_d_n12, eq36_e305_d_n13, eq36_e305_d_n14, eq36_e305_d_n15, eq36_e305_d_n16, eq36_e305_d_n17, eq36_e305_d_n18];
        let eq36_branch_derivatives: [f64; 19] = [eq36_e305_d_b0, eq36_e305_d_b1, eq36_e305_d_b2, eq36_e305_d_b3, eq36_e305_d_b4, eq36_e305_d_b5, eq36_e305_d_b6, eq36_e305_d_b7, eq36_e305_d_b8, eq36_e305_d_b9, eq36_e305_d_b10, eq36_e305_d_b11, eq36_e305_d_b12, eq36_e305_d_b13, eq36_e305_d_b14, eq36_e305_d_b15, eq36_e305_d_b16, eq36_e305_d_b17, eq36_e305_d_b18];
        stamper.stamp_potential_dense_local(
            15,
            eq36_value,
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
        );
        let (eq37_e319,) = {
    if ((var_guard27 != 0.0) && (p.p0 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq37_value: f64 = eq37_e319;
        stamper.stamp_potential_const_local(
            16,
            eq37_value,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq39_e327);
        let eq39_value: f64 = eq39_e328;
        stamper.stamp_potential_branch1_local(
            18,
            eq39_value,
            18,
            (p.p52 * ddt_scale),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        var_ci: f64,
        var_ci_db0: f64,
        var_ci_db1: f64,
        var_ci_db10: f64,
        var_ci_db11: f64,
        var_ci_db12: f64,
        var_ci_db13: f64,
        var_ci_db14: f64,
        var_ci_db15: f64,
        var_ci_db16: f64,
        var_ci_db17: f64,
        var_ci_db18: f64,
        var_ci_db2: f64,
        var_ci_db3: f64,
        var_ci_db4: f64,
        var_ci_db5: f64,
        var_ci_db6: f64,
        var_ci_db7: f64,
        var_ci_db8: f64,
        var_ci_db9: f64,
        var_ci_dn0: f64,
        var_ci_dn1: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn12: f64,
        var_ci_dn13: f64,
        var_ci_dn14: f64,
        var_ci_dn15: f64,
        var_ci_dn16: f64,
        var_ci_dn17: f64,
        var_ci_dn18: f64,
        var_ci_dn2: f64,
        var_ci_dn3: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_guard28: f64,
        var_guard29: f64,
        var_guard44: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq51_e429, eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18, eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18,) = {
    if (((var_guard29 != 0.0) && (var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-var_ci);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n0: f64 = ((-var_ci_dn0) * (nv17 - 0.0));
        let eq51_e426_d_n1: f64 = ((-var_ci_dn1) * (nv17 - 0.0));
        let eq51_e426_d_n2: f64 = ((-var_ci_dn2) * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = ((-var_ci_dn3) * (nv17 - 0.0));
        let eq51_e426_d_n4: f64 = ((-var_ci_dn4) * (nv17 - 0.0));
        let eq51_e426_d_n5: f64 = ((-var_ci_dn5) * (nv17 - 0.0));
        let eq51_e426_d_n6: f64 = ((-var_ci_dn6) * (nv17 - 0.0));
        let eq51_e426_d_n7: f64 = ((-var_ci_dn7) * (nv17 - 0.0));
        let eq51_e426_d_n8: f64 = ((-var_ci_dn8) * (nv17 - 0.0));
        let eq51_e426_d_n9: f64 = ((-var_ci_dn9) * (nv17 - 0.0));
        let eq51_e426_d_n10: f64 = ((-var_ci_dn10) * (nv17 - 0.0));
        let eq51_e426_d_n11: f64 = ((-var_ci_dn11) * (nv17 - 0.0));
        let eq51_e426_d_n12: f64 = ((-var_ci_dn12) * (nv17 - 0.0));
        let eq51_e426_d_n13: f64 = ((-var_ci_dn13) * (nv17 - 0.0));
        let eq51_e426_d_n14: f64 = ((-var_ci_dn14) * (nv17 - 0.0));
        let eq51_e426_d_n15: f64 = ((-var_ci_dn15) * (nv17 - 0.0));
        let eq51_e426_d_n16: f64 = ((-var_ci_dn16) * (nv17 - 0.0));
        let eq51_e426_d_n17: f64 = (((-var_ci_dn17) * (nv17 - 0.0)) + eq51_e424);
        let eq51_e426_d_n18: f64 = ((-var_ci_dn18) * (nv17 - 0.0));
        let eq51_e426_d_b0: f64 = ((-var_ci_db0) * (nv17 - 0.0));
        let eq51_e426_d_b1: f64 = ((-var_ci_db1) * (nv17 - 0.0));
        let eq51_e426_d_b2: f64 = ((-var_ci_db2) * (nv17 - 0.0));
        let eq51_e426_d_b3: f64 = ((-var_ci_db3) * (nv17 - 0.0));
        let eq51_e426_d_b4: f64 = ((-var_ci_db4) * (nv17 - 0.0));
        let eq51_e426_d_b5: f64 = ((-var_ci_db5) * (nv17 - 0.0));
        let eq51_e426_d_b6: f64 = ((-var_ci_db6) * (nv17 - 0.0));
        let eq51_e426_d_b7: f64 = ((-var_ci_db7) * (nv17 - 0.0));
        let eq51_e426_d_b8: f64 = ((-var_ci_db8) * (nv17 - 0.0));
        let eq51_e426_d_b9: f64 = ((-var_ci_db9) * (nv17 - 0.0));
        let eq51_e426_d_b10: f64 = ((-var_ci_db10) * (nv17 - 0.0));
        let eq51_e426_d_b11: f64 = ((-var_ci_db11) * (nv17 - 0.0));
        let eq51_e426_d_b12: f64 = ((-var_ci_db12) * (nv17 - 0.0));
        let eq51_e426_d_b13: f64 = ((-var_ci_db13) * (nv17 - 0.0));
        let eq51_e426_d_b14: f64 = ((-var_ci_db14) * (nv17 - 0.0));
        let eq51_e426_d_b15: f64 = ((-var_ci_db15) * (nv17 - 0.0));
        let eq51_e426_d_b16: f64 = ((-var_ci_db16) * (nv17 - 0.0));
        let eq51_e426_d_b17: f64 = ((-var_ci_db17) * (nv17 - 0.0));
        let eq51_e426_d_b18: f64 = ((-var_ci_db18) * (nv17 - 0.0));
        let eq51_e427: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq51_e426);
        (eq51_e427, (eq51_e426_d_n0 * ddt_scale), (eq51_e426_d_n1 * ddt_scale), (eq51_e426_d_n2 * ddt_scale), (eq51_e426_d_n3 * ddt_scale), (eq51_e426_d_n4 * ddt_scale), (eq51_e426_d_n5 * ddt_scale), (eq51_e426_d_n6 * ddt_scale), (eq51_e426_d_n7 * ddt_scale), (eq51_e426_d_n8 * ddt_scale), (eq51_e426_d_n9 * ddt_scale), (eq51_e426_d_n10 * ddt_scale), (eq51_e426_d_n11 * ddt_scale), (eq51_e426_d_n12 * ddt_scale), (eq51_e426_d_n13 * ddt_scale), (eq51_e426_d_n14 * ddt_scale), (eq51_e426_d_n15 * ddt_scale), (eq51_e426_d_n16 * ddt_scale), (eq51_e426_d_n17 * ddt_scale), (eq51_e426_d_n18 * ddt_scale), (eq51_e426_d_b0 * ddt_scale), (eq51_e426_d_b1 * ddt_scale), (eq51_e426_d_b2 * ddt_scale), (eq51_e426_d_b3 * ddt_scale), (eq51_e426_d_b4 * ddt_scale), (eq51_e426_d_b5 * ddt_scale), (eq51_e426_d_b6 * ddt_scale), (eq51_e426_d_b7 * ddt_scale), (eq51_e426_d_b8 * ddt_scale), (eq51_e426_d_b9 * ddt_scale), (eq51_e426_d_b10 * ddt_scale), (eq51_e426_d_b11 * ddt_scale), (eq51_e426_d_b12 * ddt_scale), (eq51_e426_d_b13 * ddt_scale), (eq51_e426_d_b14 * ddt_scale), (eq51_e426_d_b15 * ddt_scale), (eq51_e426_d_b16 * ddt_scale), (eq51_e426_d_b17 * ddt_scale), (eq51_e426_d_b18 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e429;
        let eq51_node_derivatives: [f64; 19] = [eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18];
        let eq51_branch_derivatives: [f64; 19] = [eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq64_e562, eq64_e562_d_n3,) = {
    if (var_guard44 != 0.0) {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq64_e559);
        (eq64_e560, (p.p67 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e562;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq64_value),
            3,
            multiplicity * (eq64_e562_d_n3),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117_q: f64 = eq3_e116;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e114,
        );
        let (eq7_e125, eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18, eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18, eq7_e125_q,) = {
    if s.b[119] {
        let eq7_e123_q: f64 = s.v[27];
        (s.v[27], s.dn[27][0], s.dn[27][1], s.dn[27][2], s.dn[27][3], s.dn[27][4], s.dn[27][5], s.dn[27][6], s.dn[27][7], s.dn[27][8], s.dn[27][9], s.dn[27][10], s.dn[27][11], s.dn[27][12], s.dn[27][13], s.dn[27][14], s.dn[27][15], s.dn[27][16], s.dn[27][17], s.dn[27][18], s.db[27][0], s.db[27][1], s.db[27][2], s.db[27][3], s.db[27][4], s.db[27][5], s.db[27][6], s.db[27][7], s.db[27][8], s.db[27][9], s.db[27][10], s.db[27][11], s.db[27][12], s.db[27][13], s.db[27][14], s.db[27][15], s.db[27][16], s.db[27][17], s.db[27][18], eq7_e123_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 19] = [eq7_e125_d_n0, eq7_e125_d_n1, eq7_e125_d_n2, eq7_e125_d_n3, eq7_e125_d_n4, eq7_e125_d_n5, eq7_e125_d_n6, eq7_e125_d_n7, eq7_e125_d_n8, eq7_e125_d_n9, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_d_n12, eq7_e125_d_n13, eq7_e125_d_n14, eq7_e125_d_n15, eq7_e125_d_n16, eq7_e125_d_n17, eq7_e125_d_n18];
        let eq7_reactive_branch_derivatives: [f64; 19] = [eq7_e125_d_b0, eq7_e125_d_b1, eq7_e125_d_b2, eq7_e125_d_b3, eq7_e125_d_b4, eq7_e125_d_b5, eq7_e125_d_b6, eq7_e125_d_b7, eq7_e125_d_b8, eq7_e125_d_b9, eq7_e125_d_b10, eq7_e125_d_b11, eq7_e125_d_b12, eq7_e125_d_b13, eq7_e125_d_b14, eq7_e125_d_b15, eq7_e125_d_b16, eq7_e125_d_b17, eq7_e125_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18, eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18, eq8_e130_q,) = {
    if s.b[119] {
        let eq8_e128_q: f64 = s.v[26];
        (s.v[26], s.dn[26][0], s.dn[26][1], s.dn[26][2], s.dn[26][3], s.dn[26][4], s.dn[26][5], s.dn[26][6], s.dn[26][7], s.dn[26][8], s.dn[26][9], s.dn[26][10], s.dn[26][11], s.dn[26][12], s.dn[26][13], s.dn[26][14], s.dn[26][15], s.dn[26][16], s.dn[26][17], s.dn[26][18], s.db[26][0], s.db[26][1], s.db[26][2], s.db[26][3], s.db[26][4], s.db[26][5], s.db[26][6], s.db[26][7], s.db[26][8], s.db[26][9], s.db[26][10], s.db[26][11], s.db[26][12], s.db[26][13], s.db[26][14], s.db[26][15], s.db[26][16], s.db[26][17], s.db[26][18], eq8_e128_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 19] = [eq8_e130_d_n0, eq8_e130_d_n1, eq8_e130_d_n2, eq8_e130_d_n3, eq8_e130_d_n4, eq8_e130_d_n5, eq8_e130_d_n6, eq8_e130_d_n7, eq8_e130_d_n8, eq8_e130_d_n9, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_d_n12, eq8_e130_d_n13, eq8_e130_d_n14, eq8_e130_d_n15, eq8_e130_d_n16, eq8_e130_d_n17, eq8_e130_d_n18];
        let eq8_reactive_branch_derivatives: [f64; 19] = [eq8_e130_d_b0, eq8_e130_d_b1, eq8_e130_d_b2, eq8_e130_d_b3, eq8_e130_d_b4, eq8_e130_d_b5, eq8_e130_d_b6, eq8_e130_d_b7, eq8_e130_d_b8, eq8_e130_d_b9, eq8_e130_d_b10, eq8_e130_d_b11, eq8_e130_d_b12, eq8_e130_d_b13, eq8_e130_d_b14, eq8_e130_d_b15, eq8_e130_d_b16, eq8_e130_d_b17, eq8_e130_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18, eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18, eq9_e138_q,) = {
    if (!s.b[119]) {
        let eq9_e135: f64 = (s.v[29] * s.v[97]);
        let eq9_e135_d_n0: f64 = ((s.dn[29][0] * s.v[97]) + (s.v[29] * s.dn[97][0]));
        let eq9_e135_d_n1: f64 = ((s.dn[29][1] * s.v[97]) + (s.v[29] * s.dn[97][1]));
        let eq9_e135_d_n2: f64 = ((s.dn[29][2] * s.v[97]) + (s.v[29] * s.dn[97][2]));
        let eq9_e135_d_n3: f64 = ((s.dn[29][3] * s.v[97]) + (s.v[29] * s.dn[97][3]));
        let eq9_e135_d_n4: f64 = ((s.dn[29][4] * s.v[97]) + (s.v[29] * s.dn[97][4]));
        let eq9_e135_d_n5: f64 = ((s.dn[29][5] * s.v[97]) + (s.v[29] * s.dn[97][5]));
        let eq9_e135_d_n6: f64 = ((s.dn[29][6] * s.v[97]) + (s.v[29] * s.dn[97][6]));
        let eq9_e135_d_n7: f64 = ((s.dn[29][7] * s.v[97]) + (s.v[29] * s.dn[97][7]));
        let eq9_e135_d_n8: f64 = ((s.dn[29][8] * s.v[97]) + (s.v[29] * s.dn[97][8]));
        let eq9_e135_d_n9: f64 = ((s.dn[29][9] * s.v[97]) + (s.v[29] * s.dn[97][9]));
        let eq9_e135_d_n10: f64 = ((s.dn[29][10] * s.v[97]) + (s.v[29] * s.dn[97][10]));
        let eq9_e135_d_n11: f64 = ((s.dn[29][11] * s.v[97]) + (s.v[29] * s.dn[97][11]));
        let eq9_e135_d_n12: f64 = ((s.dn[29][12] * s.v[97]) + (s.v[29] * s.dn[97][12]));
        let eq9_e135_d_n13: f64 = ((s.dn[29][13] * s.v[97]) + (s.v[29] * s.dn[97][13]));
        let eq9_e135_d_n14: f64 = ((s.dn[29][14] * s.v[97]) + (s.v[29] * s.dn[97][14]));
        let eq9_e135_d_n15: f64 = ((s.dn[29][15] * s.v[97]) + (s.v[29] * s.dn[97][15]));
        let eq9_e135_d_n16: f64 = ((s.dn[29][16] * s.v[97]) + (s.v[29] * s.dn[97][16]));
        let eq9_e135_d_n17: f64 = ((s.dn[29][17] * s.v[97]) + (s.v[29] * s.dn[97][17]));
        let eq9_e135_d_n18: f64 = ((s.dn[29][18] * s.v[97]) + (s.v[29] * s.dn[97][18]));
        let eq9_e135_d_b0: f64 = ((s.db[29][0] * s.v[97]) + (s.v[29] * s.db[97][0]));
        let eq9_e135_d_b1: f64 = ((s.db[29][1] * s.v[97]) + (s.v[29] * s.db[97][1]));
        let eq9_e135_d_b2: f64 = ((s.db[29][2] * s.v[97]) + (s.v[29] * s.db[97][2]));
        let eq9_e135_d_b3: f64 = ((s.db[29][3] * s.v[97]) + (s.v[29] * s.db[97][3]));
        let eq9_e135_d_b4: f64 = ((s.db[29][4] * s.v[97]) + (s.v[29] * s.db[97][4]));
        let eq9_e135_d_b5: f64 = ((s.db[29][5] * s.v[97]) + (s.v[29] * s.db[97][5]));
        let eq9_e135_d_b6: f64 = ((s.db[29][6] * s.v[97]) + (s.v[29] * s.db[97][6]));
        let eq9_e135_d_b7: f64 = ((s.db[29][7] * s.v[97]) + (s.v[29] * s.db[97][7]));
        let eq9_e135_d_b8: f64 = ((s.db[29][8] * s.v[97]) + (s.v[29] * s.db[97][8]));
        let eq9_e135_d_b9: f64 = ((s.db[29][9] * s.v[97]) + (s.v[29] * s.db[97][9]));
        let eq9_e135_d_b10: f64 = ((s.db[29][10] * s.v[97]) + (s.v[29] * s.db[97][10]));
        let eq9_e135_d_b11: f64 = ((s.db[29][11] * s.v[97]) + (s.v[29] * s.db[97][11]));
        let eq9_e135_d_b12: f64 = ((s.db[29][12] * s.v[97]) + (s.v[29] * s.db[97][12]));
        let eq9_e135_d_b13: f64 = ((s.db[29][13] * s.v[97]) + (s.v[29] * s.db[97][13]));
        let eq9_e135_d_b14: f64 = ((s.db[29][14] * s.v[97]) + (s.v[29] * s.db[97][14]));
        let eq9_e135_d_b15: f64 = ((s.db[29][15] * s.v[97]) + (s.v[29] * s.db[97][15]));
        let eq9_e135_d_b16: f64 = ((s.db[29][16] * s.v[97]) + (s.v[29] * s.db[97][16]));
        let eq9_e135_d_b17: f64 = ((s.db[29][17] * s.v[97]) + (s.v[29] * s.db[97][17]));
        let eq9_e135_d_b18: f64 = ((s.db[29][18] * s.v[97]) + (s.v[29] * s.db[97][18]));
        let eq9_e136_q: f64 = eq9_e135;
        (eq9_e135, eq9_e135_d_n0, eq9_e135_d_n1, eq9_e135_d_n2, eq9_e135_d_n3, eq9_e135_d_n4, eq9_e135_d_n5, eq9_e135_d_n6, eq9_e135_d_n7, eq9_e135_d_n8, eq9_e135_d_n9, eq9_e135_d_n10, eq9_e135_d_n11, eq9_e135_d_n12, eq9_e135_d_n13, eq9_e135_d_n14, eq9_e135_d_n15, eq9_e135_d_n16, eq9_e135_d_n17, eq9_e135_d_n18, eq9_e135_d_b0, eq9_e135_d_b1, eq9_e135_d_b2, eq9_e135_d_b3, eq9_e135_d_b4, eq9_e135_d_b5, eq9_e135_d_b6, eq9_e135_d_b7, eq9_e135_d_b8, eq9_e135_d_b9, eq9_e135_d_b10, eq9_e135_d_b11, eq9_e135_d_b12, eq9_e135_d_b13, eq9_e135_d_b14, eq9_e135_d_b15, eq9_e135_d_b16, eq9_e135_d_b17, eq9_e135_d_b18, eq9_e136_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 19] = [eq9_e138_d_n0, eq9_e138_d_n1, eq9_e138_d_n2, eq9_e138_d_n3, eq9_e138_d_n4, eq9_e138_d_n5, eq9_e138_d_n6, eq9_e138_d_n7, eq9_e138_d_n8, eq9_e138_d_n9, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_d_n12, eq9_e138_d_n13, eq9_e138_d_n14, eq9_e138_d_n15, eq9_e138_d_n16, eq9_e138_d_n17, eq9_e138_d_n18];
        let eq9_reactive_branch_derivatives: [f64; 19] = [eq9_e138_d_b0, eq9_e138_d_b1, eq9_e138_d_b2, eq9_e138_d_b3, eq9_e138_d_b4, eq9_e138_d_b5, eq9_e138_d_b6, eq9_e138_d_b7, eq9_e138_d_b8, eq9_e138_d_b9, eq9_e138_d_b10, eq9_e138_d_b11, eq9_e138_d_b12, eq9_e138_d_b13, eq9_e138_d_b14, eq9_e138_d_b15, eq9_e138_d_b16, eq9_e138_d_b17, eq9_e138_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18, eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18, eq10_e146_q,) = {
    if (!s.b[119]) {
        let eq10_e143: f64 = (s.v[28] * s.v[96]);
        let eq10_e143_d_n0: f64 = ((s.dn[28][0] * s.v[96]) + (s.v[28] * s.dn[96][0]));
        let eq10_e143_d_n1: f64 = ((s.dn[28][1] * s.v[96]) + (s.v[28] * s.dn[96][1]));
        let eq10_e143_d_n2: f64 = ((s.dn[28][2] * s.v[96]) + (s.v[28] * s.dn[96][2]));
        let eq10_e143_d_n3: f64 = ((s.dn[28][3] * s.v[96]) + (s.v[28] * s.dn[96][3]));
        let eq10_e143_d_n4: f64 = ((s.dn[28][4] * s.v[96]) + (s.v[28] * s.dn[96][4]));
        let eq10_e143_d_n5: f64 = ((s.dn[28][5] * s.v[96]) + (s.v[28] * s.dn[96][5]));
        let eq10_e143_d_n6: f64 = ((s.dn[28][6] * s.v[96]) + (s.v[28] * s.dn[96][6]));
        let eq10_e143_d_n7: f64 = ((s.dn[28][7] * s.v[96]) + (s.v[28] * s.dn[96][7]));
        let eq10_e143_d_n8: f64 = ((s.dn[28][8] * s.v[96]) + (s.v[28] * s.dn[96][8]));
        let eq10_e143_d_n9: f64 = ((s.dn[28][9] * s.v[96]) + (s.v[28] * s.dn[96][9]));
        let eq10_e143_d_n10: f64 = ((s.dn[28][10] * s.v[96]) + (s.v[28] * s.dn[96][10]));
        let eq10_e143_d_n11: f64 = ((s.dn[28][11] * s.v[96]) + (s.v[28] * s.dn[96][11]));
        let eq10_e143_d_n12: f64 = ((s.dn[28][12] * s.v[96]) + (s.v[28] * s.dn[96][12]));
        let eq10_e143_d_n13: f64 = ((s.dn[28][13] * s.v[96]) + (s.v[28] * s.dn[96][13]));
        let eq10_e143_d_n14: f64 = ((s.dn[28][14] * s.v[96]) + (s.v[28] * s.dn[96][14]));
        let eq10_e143_d_n15: f64 = ((s.dn[28][15] * s.v[96]) + (s.v[28] * s.dn[96][15]));
        let eq10_e143_d_n16: f64 = ((s.dn[28][16] * s.v[96]) + (s.v[28] * s.dn[96][16]));
        let eq10_e143_d_n17: f64 = ((s.dn[28][17] * s.v[96]) + (s.v[28] * s.dn[96][17]));
        let eq10_e143_d_n18: f64 = ((s.dn[28][18] * s.v[96]) + (s.v[28] * s.dn[96][18]));
        let eq10_e143_d_b0: f64 = ((s.db[28][0] * s.v[96]) + (s.v[28] * s.db[96][0]));
        let eq10_e143_d_b1: f64 = ((s.db[28][1] * s.v[96]) + (s.v[28] * s.db[96][1]));
        let eq10_e143_d_b2: f64 = ((s.db[28][2] * s.v[96]) + (s.v[28] * s.db[96][2]));
        let eq10_e143_d_b3: f64 = ((s.db[28][3] * s.v[96]) + (s.v[28] * s.db[96][3]));
        let eq10_e143_d_b4: f64 = ((s.db[28][4] * s.v[96]) + (s.v[28] * s.db[96][4]));
        let eq10_e143_d_b5: f64 = ((s.db[28][5] * s.v[96]) + (s.v[28] * s.db[96][5]));
        let eq10_e143_d_b6: f64 = ((s.db[28][6] * s.v[96]) + (s.v[28] * s.db[96][6]));
        let eq10_e143_d_b7: f64 = ((s.db[28][7] * s.v[96]) + (s.v[28] * s.db[96][7]));
        let eq10_e143_d_b8: f64 = ((s.db[28][8] * s.v[96]) + (s.v[28] * s.db[96][8]));
        let eq10_e143_d_b9: f64 = ((s.db[28][9] * s.v[96]) + (s.v[28] * s.db[96][9]));
        let eq10_e143_d_b10: f64 = ((s.db[28][10] * s.v[96]) + (s.v[28] * s.db[96][10]));
        let eq10_e143_d_b11: f64 = ((s.db[28][11] * s.v[96]) + (s.v[28] * s.db[96][11]));
        let eq10_e143_d_b12: f64 = ((s.db[28][12] * s.v[96]) + (s.v[28] * s.db[96][12]));
        let eq10_e143_d_b13: f64 = ((s.db[28][13] * s.v[96]) + (s.v[28] * s.db[96][13]));
        let eq10_e143_d_b14: f64 = ((s.db[28][14] * s.v[96]) + (s.v[28] * s.db[96][14]));
        let eq10_e143_d_b15: f64 = ((s.db[28][15] * s.v[96]) + (s.v[28] * s.db[96][15]));
        let eq10_e143_d_b16: f64 = ((s.db[28][16] * s.v[96]) + (s.v[28] * s.db[96][16]));
        let eq10_e143_d_b17: f64 = ((s.db[28][17] * s.v[96]) + (s.v[28] * s.db[96][17]));
        let eq10_e143_d_b18: f64 = ((s.db[28][18] * s.v[96]) + (s.v[28] * s.db[96][18]));
        let eq10_e144_q: f64 = eq10_e143;
        (eq10_e143, eq10_e143_d_n0, eq10_e143_d_n1, eq10_e143_d_n2, eq10_e143_d_n3, eq10_e143_d_n4, eq10_e143_d_n5, eq10_e143_d_n6, eq10_e143_d_n7, eq10_e143_d_n8, eq10_e143_d_n9, eq10_e143_d_n10, eq10_e143_d_n11, eq10_e143_d_n12, eq10_e143_d_n13, eq10_e143_d_n14, eq10_e143_d_n15, eq10_e143_d_n16, eq10_e143_d_n17, eq10_e143_d_n18, eq10_e143_d_b0, eq10_e143_d_b1, eq10_e143_d_b2, eq10_e143_d_b3, eq10_e143_d_b4, eq10_e143_d_b5, eq10_e143_d_b6, eq10_e143_d_b7, eq10_e143_d_b8, eq10_e143_d_b9, eq10_e143_d_b10, eq10_e143_d_b11, eq10_e143_d_b12, eq10_e143_d_b13, eq10_e143_d_b14, eq10_e143_d_b15, eq10_e143_d_b16, eq10_e143_d_b17, eq10_e143_d_b18, eq10_e144_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e146_d_n0, eq10_e146_d_n1, eq10_e146_d_n2, eq10_e146_d_n3, eq10_e146_d_n4, eq10_e146_d_n5, eq10_e146_d_n6, eq10_e146_d_n7, eq10_e146_d_n8, eq10_e146_d_n9, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_d_n12, eq10_e146_d_n13, eq10_e146_d_n14, eq10_e146_d_n15, eq10_e146_d_n16, eq10_e146_d_n17, eq10_e146_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 19] = [eq10_e146_d_b0, eq10_e146_d_b1, eq10_e146_d_b2, eq10_e146_d_b3, eq10_e146_d_b4, eq10_e146_d_b5, eq10_e146_d_b6, eq10_e146_d_b7, eq10_e146_d_b8, eq10_e146_d_b9, eq10_e146_d_b10, eq10_e146_d_b11, eq10_e146_d_b12, eq10_e146_d_b13, eq10_e146_d_b14, eq10_e146_d_b15, eq10_e146_d_b16, eq10_e146_d_b17, eq10_e146_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e169, eq15_e169_d_n0, eq15_e169_d_n1, eq15_e169_d_n2, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n6, eq15_e169_d_n7, eq15_e169_d_n8, eq15_e169_d_n9, eq15_e169_d_n10, eq15_e169_d_n11, eq15_e169_d_n12, eq15_e169_d_n13, eq15_e169_d_n14, eq15_e169_d_n15, eq15_e169_d_n16, eq15_e169_d_n17, eq15_e169_d_n18, eq15_e169_d_b0, eq15_e169_d_b1, eq15_e169_d_b2, eq15_e169_d_b3, eq15_e169_d_b4, eq15_e169_d_b5, eq15_e169_d_b6, eq15_e169_d_b7, eq15_e169_d_b8, eq15_e169_d_b9, eq15_e169_d_b10, eq15_e169_d_b11, eq15_e169_d_b12, eq15_e169_d_b13, eq15_e169_d_b14, eq15_e169_d_b15, eq15_e169_d_b16, eq15_e169_d_b17, eq15_e169_d_b18, eq15_e169_q, eq15_e169_q_d_b1,) = {
    if s.b[120] {
        let eq15_e165: f64 = (bi1 * s.v[40]);
        let eq15_e165_d_n0: f64 = (bi1 * s.dn[40][0]);
        let eq15_e165_d_n1: f64 = (bi1 * s.dn[40][1]);
        let eq15_e165_d_n2: f64 = (bi1 * s.dn[40][2]);
        let eq15_e165_d_n3: f64 = (bi1 * s.dn[40][3]);
        let eq15_e165_d_n4: f64 = (bi1 * s.dn[40][4]);
        let eq15_e165_d_n5: f64 = (bi1 * s.dn[40][5]);
        let eq15_e165_d_n6: f64 = (bi1 * s.dn[40][6]);
        let eq15_e165_d_n7: f64 = (bi1 * s.dn[40][7]);
        let eq15_e165_d_n8: f64 = (bi1 * s.dn[40][8]);
        let eq15_e165_d_n9: f64 = (bi1 * s.dn[40][9]);
        let eq15_e165_d_n10: f64 = (bi1 * s.dn[40][10]);
        let eq15_e165_d_n11: f64 = (bi1 * s.dn[40][11]);
        let eq15_e165_d_n12: f64 = (bi1 * s.dn[40][12]);
        let eq15_e165_d_n13: f64 = (bi1 * s.dn[40][13]);
        let eq15_e165_d_n14: f64 = (bi1 * s.dn[40][14]);
        let eq15_e165_d_n15: f64 = (bi1 * s.dn[40][15]);
        let eq15_e165_d_n16: f64 = (bi1 * s.dn[40][16]);
        let eq15_e165_d_n17: f64 = (bi1 * s.dn[40][17]);
        let eq15_e165_d_n18: f64 = (bi1 * s.dn[40][18]);
        let eq15_e165_d_b0: f64 = (bi1 * s.db[40][0]);
        let eq15_e165_d_b1: f64 = (s.v[40] + (bi1 * s.db[40][1]));
        let eq15_e165_d_b2: f64 = (bi1 * s.db[40][2]);
        let eq15_e165_d_b3: f64 = (bi1 * s.db[40][3]);
        let eq15_e165_d_b4: f64 = (bi1 * s.db[40][4]);
        let eq15_e165_d_b5: f64 = (bi1 * s.db[40][5]);
        let eq15_e165_d_b6: f64 = (bi1 * s.db[40][6]);
        let eq15_e165_d_b7: f64 = (bi1 * s.db[40][7]);
        let eq15_e165_d_b8: f64 = (bi1 * s.db[40][8]);
        let eq15_e165_d_b9: f64 = (bi1 * s.db[40][9]);
        let eq15_e165_d_b10: f64 = (bi1 * s.db[40][10]);
        let eq15_e165_d_b11: f64 = (bi1 * s.db[40][11]);
        let eq15_e165_d_b12: f64 = (bi1 * s.db[40][12]);
        let eq15_e165_d_b13: f64 = (bi1 * s.db[40][13]);
        let eq15_e165_d_b14: f64 = (bi1 * s.db[40][14]);
        let eq15_e165_d_b15: f64 = (bi1 * s.db[40][15]);
        let eq15_e165_d_b16: f64 = (bi1 * s.db[40][16]);
        let eq15_e165_d_b17: f64 = (bi1 * s.db[40][17]);
        let eq15_e165_d_b18: f64 = (bi1 * s.db[40][18]);
        let eq15_e166_q: f64 = s.rv[63];
        let eq15_e167: f64 = (eq15_e165 + s.v[63]);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + s.db[63][1]);
        let eq15_e167_q: f64 = eq15_e166_q;
        (eq15_e167, eq15_e165_d_n0, eq15_e165_d_n1, eq15_e165_d_n2, eq15_e165_d_n3, eq15_e165_d_n4, eq15_e165_d_n5, eq15_e165_d_n6, eq15_e165_d_n7, eq15_e165_d_n8, eq15_e165_d_n9, eq15_e165_d_n10, eq15_e165_d_n11, eq15_e165_d_n12, eq15_e165_d_n13, eq15_e165_d_n14, eq15_e165_d_n15, eq15_e165_d_n16, eq15_e165_d_n17, eq15_e165_d_n18, eq15_e165_d_b0, eq15_e167_d_b1, eq15_e165_d_b2, eq15_e165_d_b3, eq15_e165_d_b4, eq15_e165_d_b5, eq15_e165_d_b6, eq15_e165_d_b7, eq15_e165_d_b8, eq15_e165_d_b9, eq15_e165_d_b10, eq15_e165_d_b11, eq15_e165_d_b12, eq15_e165_d_b13, eq15_e165_d_b14, eq15_e165_d_b15, eq15_e165_d_b16, eq15_e165_d_b17, eq15_e165_d_b18, eq15_e167_q, s.rdb[63][1],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[1],
            branches[1],
            eq15_e169_q_d_b1,
        );
        let (eq18_e187, eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18, eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18, eq18_e187_q,) = {
    if s.b[121] {
        let eq18_e184: f64 = (s.v[51] * (nv12 - nv8));
        let eq18_e184_d_n0: f64 = (s.dn[51][0] * (nv12 - nv8));
        let eq18_e184_d_n1: f64 = (s.dn[51][1] * (nv12 - nv8));
        let eq18_e184_d_n2: f64 = (s.dn[51][2] * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (s.dn[51][3] * (nv12 - nv8));
        let eq18_e184_d_n4: f64 = (s.dn[51][4] * (nv12 - nv8));
        let eq18_e184_d_n5: f64 = (s.dn[51][5] * (nv12 - nv8));
        let eq18_e184_d_n6: f64 = (s.dn[51][6] * (nv12 - nv8));
        let eq18_e184_d_n7: f64 = (s.dn[51][7] * (nv12 - nv8));
        let eq18_e184_d_n8: f64 = ((s.dn[51][8] * (nv12 - nv8)) + (-s.v[51]));
        let eq18_e184_d_n9: f64 = (s.dn[51][9] * (nv12 - nv8));
        let eq18_e184_d_n10: f64 = (s.dn[51][10] * (nv12 - nv8));
        let eq18_e184_d_n11: f64 = (s.dn[51][11] * (nv12 - nv8));
        let eq18_e184_d_n12: f64 = ((s.dn[51][12] * (nv12 - nv8)) + s.v[51]);
        let eq18_e184_d_n13: f64 = (s.dn[51][13] * (nv12 - nv8));
        let eq18_e184_d_n14: f64 = (s.dn[51][14] * (nv12 - nv8));
        let eq18_e184_d_n15: f64 = (s.dn[51][15] * (nv12 - nv8));
        let eq18_e184_d_n16: f64 = (s.dn[51][16] * (nv12 - nv8));
        let eq18_e184_d_n17: f64 = (s.dn[51][17] * (nv12 - nv8));
        let eq18_e184_d_n18: f64 = (s.dn[51][18] * (nv12 - nv8));
        let eq18_e184_d_b0: f64 = (s.db[51][0] * (nv12 - nv8));
        let eq18_e184_d_b1: f64 = (s.db[51][1] * (nv12 - nv8));
        let eq18_e184_d_b2: f64 = (s.db[51][2] * (nv12 - nv8));
        let eq18_e184_d_b3: f64 = (s.db[51][3] * (nv12 - nv8));
        let eq18_e184_d_b4: f64 = (s.db[51][4] * (nv12 - nv8));
        let eq18_e184_d_b5: f64 = (s.db[51][5] * (nv12 - nv8));
        let eq18_e184_d_b6: f64 = (s.db[51][6] * (nv12 - nv8));
        let eq18_e184_d_b7: f64 = (s.db[51][7] * (nv12 - nv8));
        let eq18_e184_d_b8: f64 = (s.db[51][8] * (nv12 - nv8));
        let eq18_e184_d_b9: f64 = (s.db[51][9] * (nv12 - nv8));
        let eq18_e184_d_b10: f64 = (s.db[51][10] * (nv12 - nv8));
        let eq18_e184_d_b11: f64 = (s.db[51][11] * (nv12 - nv8));
        let eq18_e184_d_b12: f64 = (s.db[51][12] * (nv12 - nv8));
        let eq18_e184_d_b13: f64 = (s.db[51][13] * (nv12 - nv8));
        let eq18_e184_d_b14: f64 = (s.db[51][14] * (nv12 - nv8));
        let eq18_e184_d_b15: f64 = (s.db[51][15] * (nv12 - nv8));
        let eq18_e184_d_b16: f64 = (s.db[51][16] * (nv12 - nv8));
        let eq18_e184_d_b17: f64 = (s.db[51][17] * (nv12 - nv8));
        let eq18_e184_d_b18: f64 = (s.db[51][18] * (nv12 - nv8));
        let eq18_e185_q: f64 = eq18_e184;
        (eq18_e184, eq18_e184_d_n0, eq18_e184_d_n1, eq18_e184_d_n2, eq18_e184_d_n3, eq18_e184_d_n4, eq18_e184_d_n5, eq18_e184_d_n6, eq18_e184_d_n7, eq18_e184_d_n8, eq18_e184_d_n9, eq18_e184_d_n10, eq18_e184_d_n11, eq18_e184_d_n12, eq18_e184_d_n13, eq18_e184_d_n14, eq18_e184_d_n15, eq18_e184_d_n16, eq18_e184_d_n17, eq18_e184_d_n18, eq18_e184_d_b0, eq18_e184_d_b1, eq18_e184_d_b2, eq18_e184_d_b3, eq18_e184_d_b4, eq18_e184_d_b5, eq18_e184_d_b6, eq18_e184_d_b7, eq18_e184_d_b8, eq18_e184_d_b9, eq18_e184_d_b10, eq18_e184_d_b11, eq18_e184_d_b12, eq18_e184_d_b13, eq18_e184_d_b14, eq18_e184_d_b15, eq18_e184_d_b16, eq18_e184_d_b17, eq18_e184_d_b18, eq18_e185_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e187_d_n0, eq18_e187_d_n1, eq18_e187_d_n2, eq18_e187_d_n3, eq18_e187_d_n4, eq18_e187_d_n5, eq18_e187_d_n6, eq18_e187_d_n7, eq18_e187_d_n8, eq18_e187_d_n9, eq18_e187_d_n10, eq18_e187_d_n11, eq18_e187_d_n12, eq18_e187_d_n13, eq18_e187_d_n14, eq18_e187_d_n15, eq18_e187_d_n16, eq18_e187_d_n17, eq18_e187_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 19] = [eq18_e187_d_b0, eq18_e187_d_b1, eq18_e187_d_b2, eq18_e187_d_b3, eq18_e187_d_b4, eq18_e187_d_b5, eq18_e187_d_b6, eq18_e187_d_b7, eq18_e187_d_b8, eq18_e187_d_b9, eq18_e187_d_b10, eq18_e187_d_b11, eq18_e187_d_b12, eq18_e187_d_b13, eq18_e187_d_b14, eq18_e187_d_b15, eq18_e187_d_b16, eq18_e187_d_b17, eq18_e187_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e270_q: f64 = eq31_e269;
        stamper.stamp_potential_reactive_branch1(
            branches[10],
            branches[10],
            p.p54,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e299_q: f64 = eq35_e298;
        stamper.stamp_potential_reactive_branch1(
            branches[14],
            branches[14],
            p.p53,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e328_q: f64 = eq39_e327;
        stamper.stamp_potential_reactive_branch1(
            branches[18],
            branches[18],
            p.p52,
        );
        let (eq51_e429, eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18, eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18, eq51_e429_q,) = {
    if ((s.b[129] && (!s.b[128])) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-s.v[138]);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n0: f64 = ((-s.dn[138][0]) * (nv17 - 0.0));
        let eq51_e426_d_n1: f64 = ((-s.dn[138][1]) * (nv17 - 0.0));
        let eq51_e426_d_n2: f64 = ((-s.dn[138][2]) * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = ((-s.dn[138][3]) * (nv17 - 0.0));
        let eq51_e426_d_n4: f64 = ((-s.dn[138][4]) * (nv17 - 0.0));
        let eq51_e426_d_n5: f64 = ((-s.dn[138][5]) * (nv17 - 0.0));
        let eq51_e426_d_n6: f64 = ((-s.dn[138][6]) * (nv17 - 0.0));
        let eq51_e426_d_n7: f64 = ((-s.dn[138][7]) * (nv17 - 0.0));
        let eq51_e426_d_n8: f64 = ((-s.dn[138][8]) * (nv17 - 0.0));
        let eq51_e426_d_n9: f64 = ((-s.dn[138][9]) * (nv17 - 0.0));
        let eq51_e426_d_n10: f64 = ((-s.dn[138][10]) * (nv17 - 0.0));
        let eq51_e426_d_n11: f64 = ((-s.dn[138][11]) * (nv17 - 0.0));
        let eq51_e426_d_n12: f64 = ((-s.dn[138][12]) * (nv17 - 0.0));
        let eq51_e426_d_n13: f64 = ((-s.dn[138][13]) * (nv17 - 0.0));
        let eq51_e426_d_n14: f64 = ((-s.dn[138][14]) * (nv17 - 0.0));
        let eq51_e426_d_n15: f64 = ((-s.dn[138][15]) * (nv17 - 0.0));
        let eq51_e426_d_n16: f64 = ((-s.dn[138][16]) * (nv17 - 0.0));
        let eq51_e426_d_n17: f64 = (((-s.dn[138][17]) * (nv17 - 0.0)) + eq51_e424);
        let eq51_e426_d_n18: f64 = ((-s.dn[138][18]) * (nv17 - 0.0));
        let eq51_e426_d_b0: f64 = ((-s.db[138][0]) * (nv17 - 0.0));
        let eq51_e426_d_b1: f64 = ((-s.db[138][1]) * (nv17 - 0.0));
        let eq51_e426_d_b2: f64 = ((-s.db[138][2]) * (nv17 - 0.0));
        let eq51_e426_d_b3: f64 = ((-s.db[138][3]) * (nv17 - 0.0));
        let eq51_e426_d_b4: f64 = ((-s.db[138][4]) * (nv17 - 0.0));
        let eq51_e426_d_b5: f64 = ((-s.db[138][5]) * (nv17 - 0.0));
        let eq51_e426_d_b6: f64 = ((-s.db[138][6]) * (nv17 - 0.0));
        let eq51_e426_d_b7: f64 = ((-s.db[138][7]) * (nv17 - 0.0));
        let eq51_e426_d_b8: f64 = ((-s.db[138][8]) * (nv17 - 0.0));
        let eq51_e426_d_b9: f64 = ((-s.db[138][9]) * (nv17 - 0.0));
        let eq51_e426_d_b10: f64 = ((-s.db[138][10]) * (nv17 - 0.0));
        let eq51_e426_d_b11: f64 = ((-s.db[138][11]) * (nv17 - 0.0));
        let eq51_e426_d_b12: f64 = ((-s.db[138][12]) * (nv17 - 0.0));
        let eq51_e426_d_b13: f64 = ((-s.db[138][13]) * (nv17 - 0.0));
        let eq51_e426_d_b14: f64 = ((-s.db[138][14]) * (nv17 - 0.0));
        let eq51_e426_d_b15: f64 = ((-s.db[138][15]) * (nv17 - 0.0));
        let eq51_e426_d_b16: f64 = ((-s.db[138][16]) * (nv17 - 0.0));
        let eq51_e426_d_b17: f64 = ((-s.db[138][17]) * (nv17 - 0.0));
        let eq51_e426_d_b18: f64 = ((-s.db[138][18]) * (nv17 - 0.0));
        let eq51_e427_q: f64 = eq51_e426;
        (eq51_e426, eq51_e426_d_n0, eq51_e426_d_n1, eq51_e426_d_n2, eq51_e426_d_n3, eq51_e426_d_n4, eq51_e426_d_n5, eq51_e426_d_n6, eq51_e426_d_n7, eq51_e426_d_n8, eq51_e426_d_n9, eq51_e426_d_n10, eq51_e426_d_n11, eq51_e426_d_n12, eq51_e426_d_n13, eq51_e426_d_n14, eq51_e426_d_n15, eq51_e426_d_n16, eq51_e426_d_n17, eq51_e426_d_n18, eq51_e426_d_b0, eq51_e426_d_b1, eq51_e426_d_b2, eq51_e426_d_b3, eq51_e426_d_b4, eq51_e426_d_b5, eq51_e426_d_b6, eq51_e426_d_b7, eq51_e426_d_b8, eq51_e426_d_b9, eq51_e426_d_b10, eq51_e426_d_b11, eq51_e426_d_b12, eq51_e426_d_b13, eq51_e426_d_b14, eq51_e426_d_b15, eq51_e426_d_b16, eq51_e426_d_b17, eq51_e426_d_b18, eq51_e427_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 19] = [eq51_e429_d_n0, eq51_e429_d_n1, eq51_e429_d_n2, eq51_e429_d_n3, eq51_e429_d_n4, eq51_e429_d_n5, eq51_e429_d_n6, eq51_e429_d_n7, eq51_e429_d_n8, eq51_e429_d_n9, eq51_e429_d_n10, eq51_e429_d_n11, eq51_e429_d_n12, eq51_e429_d_n13, eq51_e429_d_n14, eq51_e429_d_n15, eq51_e429_d_n16, eq51_e429_d_n17, eq51_e429_d_n18];
        let eq51_reactive_branch_derivatives: [f64; 19] = [eq51_e429_d_b0, eq51_e429_d_b1, eq51_e429_d_b2, eq51_e429_d_b3, eq51_e429_d_b4, eq51_e429_d_b5, eq51_e429_d_b6, eq51_e429_d_b7, eq51_e429_d_b8, eq51_e429_d_b9, eq51_e429_d_b10, eq51_e429_d_b11, eq51_e429_d_b12, eq51_e429_d_b13, eq51_e429_d_b14, eq51_e429_d_b15, eq51_e429_d_b16, eq51_e429_d_b17, eq51_e429_d_b18];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq64_e562, eq64_e562_d_n3, eq64_e562_q,) = {
    if s.b[144] {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e560_q: f64 = eq64_e559;
        (eq64_e559, p.p67, eq64_e560_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq64_e562_d_n3),
        );
    }
}
