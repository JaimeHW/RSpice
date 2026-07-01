#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_96(
        locals: &mut StampLocals,
    ) {
        let assign36010_e40424: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign36010_e40426: f64 = (assign36010_e40424 * 0.3333333333333);
        let assign36010_e40428: f64 = if assign36010_e40426 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1168 = assign36010_e40428;

        let (assign36020_e40442, assign36020_e40442_d_n4, assign36020_e40442_d_n6, assign36020_e40442_d_n7, assign36020_e40442_d_n8, assign36020_e40442_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1168 != 0.0)) {
        let assign36020_e40435: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign36020_e40437: f64 = (assign36020_e40435 * 0.3333333333333);
        let assign36020_e40438: f64 = (assign36020_e40437).exp();
        let assign36020_e40439: f64 = (1.0 + assign36020_e40438);
        let assign36020_e40440: f64 = (assign36020_e40439).ln();
        (assign36020_e40440, ((assign36020_e40438 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333)) / assign36020_e40439), ((assign36020_e40438 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333)) / assign36020_e40439), ((assign36020_e40438 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333)) / assign36020_e40439), ((assign36020_e40438 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333)) / assign36020_e40439), ((assign36020_e40438 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333)) / assign36020_e40439),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign36020_e40442;
        locals.var_q_temp3__blk816_dn4 = assign36020_e40442_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign36020_e40442_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign36020_e40442_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign36020_e40442_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign36020_e40442_d_n9;

        let (assign36030_e40453, assign36030_e40453_d_n4, assign36030_e40453_d_n6, assign36030_e40453_d_n7, assign36030_e40453_d_n8, assign36030_e40453_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1168 == 0.0)) {
        let assign36030_e40449: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign36030_e40451: f64 = (assign36030_e40449 * 0.3333333333333);
        (assign36030_e40451, ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign36030_e40453;
        locals.var_q_temp3__blk816_dn4 = assign36030_e40453_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign36030_e40453_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign36030_e40453_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign36030_e40453_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign36030_e40453_d_n9;

        let (assign36040_e40461, assign36040_e40461_d_n4, assign36040_e40461_d_n6, assign36040_e40461_d_n7, assign36040_e40461_d_n8, assign36040_e40461_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36040_e40458: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign36040_e40459: f64 = (locals.var_q_x2sat__blk818 - assign36040_e40458);
        (assign36040_e40459, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign36040_e40461;
        locals.var_q_x2__blk822_dn4 = assign36040_e40461_d_n4;
        locals.var_q_x2__blk822_dn6 = assign36040_e40461_d_n6;
        locals.var_q_x2__blk822_dn7 = assign36040_e40461_d_n7;
        locals.var_q_x2__blk822_dn8 = assign36040_e40461_d_n8;
        locals.var_q_x2__blk822_dn9 = assign36040_e40461_d_n9;

        let (assign36050_e40467, assign36050_e40467_d_n4, assign36050_e40467_d_n6, assign36050_e40467_d_n7, assign36050_e40467_d_n8, assign36050_e40467_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36050_e40465: f64 = (locals.var_xg1x__blk930 - locals.var_q_x1__blk821);
        (assign36050_e40465, (locals.var_xg1x__blk930_dn4 - locals.var_q_x1__blk821_dn4), (locals.var_xg1x__blk930_dn6 - locals.var_q_x1__blk821_dn6), (locals.var_xg1x__blk930_dn7 - locals.var_q_x1__blk821_dn7), (locals.var_xg1x__blk930_dn8 - locals.var_q_x1__blk821_dn8), (locals.var_xg1x__blk930_dn9 - locals.var_q_x1__blk821_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign36050_e40467;
        locals.var_q1d__blk1001_dn4 = assign36050_e40467_d_n4;
        locals.var_q1d__blk1001_dn6 = assign36050_e40467_d_n6;
        locals.var_q1d__blk1001_dn7 = assign36050_e40467_d_n7;
        locals.var_q1d__blk1001_dn8 = assign36050_e40467_d_n8;
        locals.var_q1d__blk1001_dn9 = assign36050_e40467_d_n9;

        let (assign36060_e40473, assign36060_e40473_d_n4, assign36060_e40473_d_n6, assign36060_e40473_d_n7, assign36060_e40473_d_n8, assign36060_e40473_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36060_e40471: f64 = (locals.var_xg2x__blk931 - locals.var_q_x2__blk822);
        (assign36060_e40471, (locals.var_xg2x__blk931_dn4 - locals.var_q_x2__blk822_dn4), (locals.var_xg2x__blk931_dn6 - locals.var_q_x2__blk822_dn6), (locals.var_xg2x__blk931_dn7 - locals.var_q_x2__blk822_dn7), (locals.var_xg2x__blk931_dn8 - locals.var_q_x2__blk822_dn8), (locals.var_xg2x__blk931_dn9 - locals.var_q_x2__blk822_dn9),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign36060_e40473;
        locals.var_q2d__blk1002_dn4 = assign36060_e40473_d_n4;
        locals.var_q2d__blk1002_dn6 = assign36060_e40473_d_n6;
        locals.var_q2d__blk1002_dn7 = assign36060_e40473_d_n7;
        locals.var_q2d__blk1002_dn8 = assign36060_e40473_d_n8;
        locals.var_q2d__blk1002_dn9 = assign36060_e40473_d_n9;

        let (assign36070_e40477, assign36070_e40477_d_n4, assign36070_e40477_d_n6, assign36070_e40477_d_n7, assign36070_e40477_d_n8, assign36070_e40477_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign36070_e40477;
        locals.var_q_rac_qsq__blk828_dn4 = assign36070_e40477_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign36070_e40477_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign36070_e40477_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign36070_e40477_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign36070_e40477_d_n9;

        let (assign36080_e40481, assign36080_e40481_d_n4, assign36080_e40481_d_n6, assign36080_e40481_d_n7, assign36080_e40481_d_n8, assign36080_e40481_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign36080_e40481;
        locals.var_q_invexpq__blk831_dn4 = assign36080_e40481_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign36080_e40481_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign36080_e40481_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign36080_e40481_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign36080_e40481_d_n9;

        let (assign36090_e40487, assign36090_e40487_d_n4, assign36090_e40487_d_n6, assign36090_e40487_d_n7, assign36090_e40487_d_n8, assign36090_e40487_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36090_e40485: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign36090_e40485, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign36090_e40487;
        locals.var_q_k1q1__blk823_dn4 = assign36090_e40487_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign36090_e40487_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign36090_e40487_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign36090_e40487_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign36090_e40487_d_n9;

        let assign36100_e40490: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36100_e40492: f64 = (assign36100_e40490 - locals.var_xdeff__blk1000);
        let assign36100_e40494: f64 = if assign36100_e40492 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1169 = assign36100_e40494;

        let (assign36110_e40505, assign36110_e40505_d_n4, assign36110_e40505_d_n6, assign36110_e40505_d_n7, assign36110_e40505_d_n8, assign36110_e40505_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1169 != 0.0)) {
        let assign36110_e40500: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36110_e40502: f64 = (assign36110_e40500 - locals.var_xdeff__blk1000);
        let assign36110_e40503: f64 = (assign36110_e40502).exp();
        (assign36110_e40503, (assign36110_e40503 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign36110_e40503 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign36110_e40503 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign36110_e40503 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign36110_e40503 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36110_e40505;
        locals.var_q_temp1__blk814_dn4 = assign36110_e40505_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36110_e40505_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36110_e40505_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36110_e40505_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36110_e40505_d_n9;

        let (assign36120_e40546, assign36120_e40546_d_n4, assign36120_e40546_d_n6, assign36120_e40546_d_n7, assign36120_e40546_d_n8, assign36120_e40546_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1169 == 0.0)) {
        let assign36120_e40514: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36120_e40516: f64 = (assign36120_e40514 - locals.var_xdeff__blk1000);
        let assign36120_e40518: f64 = (assign36120_e40516 - 80.0);
        let assign36120_e40523: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36120_e40525: f64 = (assign36120_e40523 - locals.var_xdeff__blk1000);
        let assign36120_e40527: f64 = (assign36120_e40525 - 80.0);
        let assign36120_e40528: f64 = (0.5 * assign36120_e40527);
        let assign36120_e40532: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36120_e40534: f64 = (assign36120_e40532 - locals.var_xdeff__blk1000);
        let assign36120_e40536: f64 = (assign36120_e40534 - 80.0);
        let assign36120_e40538: f64 = (assign36120_e40536 * 0.3333333333333);
        let assign36120_e40539: f64 = (1.0 + assign36120_e40538);
        let assign36120_e40540: f64 = (assign36120_e40528 * assign36120_e40539);
        let assign36120_e40541: f64 = (1.0 + assign36120_e40540);
        let assign36120_e40542: f64 = (assign36120_e40518 * assign36120_e40541);
        let assign36120_e40543: f64 = (1.0 + assign36120_e40542);
        let assign36120_e40544: f64 = (5.54062e34 * assign36120_e40543);
        (assign36120_e40544, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign36120_e40541) + (assign36120_e40518 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign36120_e40539) + (assign36120_e40528 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign36120_e40541) + (assign36120_e40518 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign36120_e40539) + (assign36120_e40528 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign36120_e40541) + (assign36120_e40518 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign36120_e40539) + (assign36120_e40528 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign36120_e40541) + (assign36120_e40518 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign36120_e40539) + (assign36120_e40528 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign36120_e40541) + (assign36120_e40518 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign36120_e40539) + (assign36120_e40528 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36120_e40546;
        locals.var_q_temp1__blk814_dn4 = assign36120_e40546_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36120_e40546_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36120_e40546_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36120_e40546_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36120_e40546_d_n9;

        let (assign36130_e40552, assign36130_e40552_d_n4, assign36130_e40552_d_n6, assign36130_e40552_d_n7, assign36130_e40552_d_n8, assign36130_e40552_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36130_e40550: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign36130_e40550, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign36130_e40552;
        locals.var_q_aexp__blk824_dn4 = assign36130_e40552_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign36130_e40552_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign36130_e40552_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign36130_e40552_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign36130_e40552_d_n9;

        let (assign36140_e40560, assign36140_e40560_d_n4, assign36140_e40560_d_n6, assign36140_e40560_d_n7, assign36140_e40560_d_n8, assign36140_e40560_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36140_e40556: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign36140_e40558: f64 = (assign36140_e40556 - locals.var_q_aexp__blk824);
        (assign36140_e40558, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign36140_e40560;
        locals.var_q_qsq__blk825_dn4 = assign36140_e40560_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign36140_e40560_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign36140_e40560_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign36140_e40560_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign36140_e40560_d_n9;

        let (assign36150_e40570, assign36150_e40570_d_n4, assign36150_e40570_d_n6, assign36150_e40570_d_n7, assign36150_e40570_d_n8, assign36150_e40570_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36150_e40564: f64 = (2.0 * locals.var_k1__blk932);
        let assign36150_e40566: f64 = (assign36150_e40564 * locals.var_q_k1q1__blk823);
        let assign36150_e40568: f64 = (assign36150_e40566 + locals.var_q_aexp__blk824);
        (assign36150_e40568, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign36150_e40564 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign36150_e40564 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign36150_e40564 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign36150_e40564 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign36150_e40564 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign36150_e40570;
        locals.var_q_d1_qsq__blk826_dn4 = assign36150_e40570_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign36150_e40570_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign36150_e40570_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign36150_e40570_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign36150_e40570_d_n9;

        let (assign36160_e40580, assign36160_e40580_d_n4, assign36160_e40580_d_n6, assign36160_e40580_d_n7, assign36160_e40580_d_n8, assign36160_e40580_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36160_e40574: f64 = (2.0 * locals.var_k1__blk932);
        let assign36160_e40576: f64 = (assign36160_e40574 * locals.var_k1__blk932);
        let assign36160_e40578: f64 = (assign36160_e40576 - locals.var_q_aexp__blk824);
        (assign36160_e40578, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign36160_e40574 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign36160_e40574 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign36160_e40574 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign36160_e40574 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign36160_e40574 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign36160_e40580;
        locals.var_q_d2_qsq__blk827_dn4 = assign36160_e40580_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign36160_e40580_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign36160_e40580_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign36160_e40580_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign36160_e40580_d_n9;

        let assign36170_e40583: f64 = (-0.005);
        let assign36170_e40584: f64 = if locals.var_q_qsq__blk825 < assign36170_e40583 { 1.0 } else { 0.0 };
        locals.var_guard1170 = assign36170_e40584;

        let (assign36180_e40592, assign36180_e40592_d_n4, assign36180_e40592_d_n6, assign36180_e40592_d_n7, assign36180_e40592_d_n8, assign36180_e40592_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36180_e40589: f64 = (locals.var_q_qsq__blk825).abs();
        let assign36180_e40590: f64 = (assign36180_e40589).sqrt();
        (assign36180_e40590, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign36180_e40590)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign36180_e40590)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign36180_e40590)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign36180_e40590)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign36180_e40590)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign36180_e40592;
        locals.var_q_rac_qsq__blk828_dn4 = assign36180_e40592_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign36180_e40592_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign36180_e40592_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign36180_e40592_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign36180_e40592_d_n9;

        let (assign36190_e40603, assign36190_e40603_d_n4, assign36190_e40603_d_n6, assign36190_e40603_d_n7, assign36190_e40603_d_n8, assign36190_e40603_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36190_e40599: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign36190_e40600: f64 = (assign36190_e40599).tan();
        let assign36190_e40601: f64 = (locals.var_q_rac_qsq__blk828 / assign36190_e40600);
        (assign36190_e40601, (((locals.var_q_rac_qsq__blk828_dn4 * assign36190_e40600) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign36190_e40599).cos() * (assign36190_e40599).cos())))) / (assign36190_e40600 * assign36190_e40600)), (((locals.var_q_rac_qsq__blk828_dn6 * assign36190_e40600) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign36190_e40599).cos() * (assign36190_e40599).cos())))) / (assign36190_e40600 * assign36190_e40600)), (((locals.var_q_rac_qsq__blk828_dn7 * assign36190_e40600) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign36190_e40599).cos() * (assign36190_e40599).cos())))) / (assign36190_e40600 * assign36190_e40600)), (((locals.var_q_rac_qsq__blk828_dn8 * assign36190_e40600) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign36190_e40599).cos() * (assign36190_e40599).cos())))) / (assign36190_e40600 * assign36190_e40600)), (((locals.var_q_rac_qsq__blk828_dn9 * assign36190_e40600) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign36190_e40599).cos() * (assign36190_e40599).cos())))) / (assign36190_e40600 * assign36190_e40600)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36190_e40603;
        locals.var_q_qcoth__blk829_dn4 = assign36190_e40603_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36190_e40603_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36190_e40603_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36190_e40603_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36190_e40603_d_n9;

        let (assign36200_e40613, assign36200_e40613_d_n4, assign36200_e40613_d_n6, assign36200_e40613_d_n7, assign36200_e40613_d_n8, assign36200_e40613_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36200_e40609: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign36200_e40611: f64 = (assign36200_e40609 / locals.var_q_qsq__blk825);
        (assign36200_e40611, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign36200_e40609 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign36200_e40609 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign36200_e40609 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign36200_e40609 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign36200_e40609 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36200_e40613;
        locals.var_q_temp1__blk814_dn4 = assign36200_e40613_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36200_e40613_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36200_e40613_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36200_e40613_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36200_e40613_d_n9;

        let (assign36210_e40627, assign36210_e40627_d_n4, assign36210_e40627_d_n6, assign36210_e40627_d_n7, assign36210_e40627_d_n8, assign36210_e40627_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36210_e40621: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign36210_e40622: f64 = (locals.var_q_qcoth__blk829 * assign36210_e40621);
        let assign36210_e40623: f64 = (locals.var_q_qsq__blk825 + assign36210_e40622);
        let assign36210_e40625: f64 = (assign36210_e40623 * locals.var_q_temp1__blk814);
        (assign36210_e40625, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign36210_e40621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign36210_e40623 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign36210_e40621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign36210_e40623 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign36210_e40621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign36210_e40623 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign36210_e40621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign36210_e40623 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign36210_e40621) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign36210_e40623 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign36210_e40627;
        locals.var_q_d1_qcoth__blk830_dn4 = assign36210_e40627_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign36210_e40627_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign36210_e40627_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign36210_e40627_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign36210_e40627_d_n9;

        let (assign36220_e40649, assign36220_e40649_d_n4, assign36220_e40649_d_n6, assign36220_e40649_d_n7, assign36220_e40649_d_n8, assign36220_e40649_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36220_e40634: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign36220_e40637: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign36220_e40638: f64 = (assign36220_e40634 * assign36220_e40637);
        let assign36220_e40639: f64 = (locals.var_q_d1_qsq__blk826 - assign36220_e40638);
        let assign36220_e40641: f64 = (assign36220_e40639 * locals.var_q_temp1__blk814);
        let assign36220_e40644: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign36220_e40646: f64 = (assign36220_e40644 / locals.var_q_d1_qsq__blk826);
        let assign36220_e40647: f64 = (assign36220_e40641 + assign36220_e40646);
        (assign36220_e40647, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign36220_e40637) + (assign36220_e40634 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign36220_e40639 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40644 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign36220_e40637) + (assign36220_e40634 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign36220_e40639 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40644 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign36220_e40637) + (assign36220_e40634 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign36220_e40639 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40644 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign36220_e40637) + (assign36220_e40634 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign36220_e40639 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40644 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign36220_e40637) + (assign36220_e40634 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign36220_e40639 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign36220_e40644 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign36220_e40649;
        locals.var_q_d2_qcoth__blk832_dn4 = assign36220_e40649_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign36220_e40649_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign36220_e40649_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign36220_e40649_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign36220_e40649_d_n9;

        let (assign36230_e40659, assign36230_e40659_d_n4, assign36230_e40659_d_n6, assign36230_e40659_d_n7, assign36230_e40659_d_n8, assign36230_e40659_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36230_e40656: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign36230_e40657: f64 = (1.0 - assign36230_e40656);
        (assign36230_e40657, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36230_e40659;
        locals.var_q_temp2__blk815_dn4 = assign36230_e40659_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36230_e40659_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36230_e40659_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36230_e40659_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36230_e40659_d_n9;

        let (assign36240_e40669, assign36240_e40669_d_n4, assign36240_e40669_d_n6, assign36240_e40669_d_n7, assign36240_e40669_d_n8, assign36240_e40669_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36240_e40665: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign36240_e40667: f64 = (assign36240_e40665 * locals.var_q_temp2__blk815);
        (assign36240_e40667, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40665 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40665 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40665 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40665 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36240_e40665 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign36240_e40669;
        locals.var_q_d1_ln__blk835_dn4 = assign36240_e40669_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign36240_e40669_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign36240_e40669_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign36240_e40669_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign36240_e40669_d_n9;

        let (assign36250_e40687, assign36250_e40687_d_n4, assign36250_e40687_d_n6, assign36250_e40687_d_n7, assign36250_e40687_d_n8, assign36250_e40687_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1170 != 0.0)) {
        let assign36250_e40675: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign36250_e40680: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign36250_e40681: f64 = (locals.var_q_d1_ln__blk835 + assign36250_e40680);
        let assign36250_e40682: f64 = (locals.var_q_d1_qsq__blk826 * assign36250_e40681);
        let assign36250_e40683: f64 = (assign36250_e40675 - assign36250_e40682);
        let assign36250_e40685: f64 = (assign36250_e40683 / locals.var_q_qsq__blk825);
        (assign36250_e40685, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign36250_e40681) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign36250_e40683 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign36250_e40681) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign36250_e40683 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign36250_e40681) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign36250_e40683 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign36250_e40681) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign36250_e40683 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign36250_e40681) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign36250_e40683 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign36250_e40687;
        locals.var_q_d2_ln__blk836_dn4 = assign36250_e40687_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign36250_e40687_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign36250_e40687_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign36250_e40687_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign36250_e40687_d_n9;

        let assign36260_e40690: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1171 = assign36260_e40690;

        let (assign36270_e40701, assign36270_e40701_d_n4, assign36270_e40701_d_n6, assign36270_e40701_d_n7, assign36270_e40701_d_n8, assign36270_e40701_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36270_e40698: f64 = (locals.var_q_qsq__blk825).abs();
        let assign36270_e40699: f64 = (assign36270_e40698).sqrt();
        (assign36270_e40699, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign36270_e40699)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign36270_e40699)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign36270_e40699)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign36270_e40699)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign36270_e40699)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign36270_e40701;
        locals.var_q_rac_qsq__blk828_dn4 = assign36270_e40701_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign36270_e40701_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign36270_e40701_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign36270_e40701_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign36270_e40701_d_n9;

        let (assign36280_e40712, assign36280_e40712_d_n4, assign36280_e40712_d_n6, assign36280_e40712_d_n7, assign36280_e40712_d_n8, assign36280_e40712_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36280_e40709: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign36280_e40710: f64 = (assign36280_e40709).exp();
        (assign36280_e40710, (assign36280_e40710 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign36280_e40710 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign36280_e40710 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign36280_e40710 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign36280_e40710 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign36280_e40712;
        locals.var_q_invexpq__blk831_dn4 = assign36280_e40712_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign36280_e40712_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign36280_e40712_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign36280_e40712_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign36280_e40712_d_n9;

        let (assign36290_e40729, assign36290_e40729_d_n4, assign36290_e40729_d_n6, assign36290_e40729_d_n7, assign36290_e40729_d_n8, assign36290_e40729_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36290_e40722: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign36290_e40723: f64 = (locals.var_q_rac_qsq__blk828 * assign36290_e40722);
        let assign36290_e40726: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign36290_e40727: f64 = (assign36290_e40723 / assign36290_e40726);
        (assign36290_e40727, (((((locals.var_q_rac_qsq__blk828_dn4 * assign36290_e40722) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign36290_e40726) - (assign36290_e40723 * (-locals.var_q_invexpq__blk831_dn4))) / (assign36290_e40726 * assign36290_e40726)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign36290_e40722) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign36290_e40726) - (assign36290_e40723 * (-locals.var_q_invexpq__blk831_dn6))) / (assign36290_e40726 * assign36290_e40726)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign36290_e40722) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign36290_e40726) - (assign36290_e40723 * (-locals.var_q_invexpq__blk831_dn7))) / (assign36290_e40726 * assign36290_e40726)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign36290_e40722) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign36290_e40726) - (assign36290_e40723 * (-locals.var_q_invexpq__blk831_dn8))) / (assign36290_e40726 * assign36290_e40726)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign36290_e40722) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign36290_e40726) - (assign36290_e40723 * (-locals.var_q_invexpq__blk831_dn9))) / (assign36290_e40726 * assign36290_e40726)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36290_e40729;
        locals.var_q_qcoth__blk829_dn4 = assign36290_e40729_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36290_e40729_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36290_e40729_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36290_e40729_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36290_e40729_d_n9;

        let (assign36300_e40742, assign36300_e40742_d_n4, assign36300_e40742_d_n6, assign36300_e40742_d_n7, assign36300_e40742_d_n8, assign36300_e40742_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36300_e40738: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign36300_e40740: f64 = (assign36300_e40738 / locals.var_q_qsq__blk825);
        (assign36300_e40740, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign36300_e40738 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign36300_e40738 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign36300_e40738 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign36300_e40738 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign36300_e40738 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36300_e40742;
        locals.var_q_temp1__blk814_dn4 = assign36300_e40742_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36300_e40742_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36300_e40742_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36300_e40742_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36300_e40742_d_n9;

        let (assign36310_e40759, assign36310_e40759_d_n4, assign36310_e40759_d_n6, assign36310_e40759_d_n7, assign36310_e40759_d_n8, assign36310_e40759_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36310_e40753: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign36310_e40754: f64 = (locals.var_q_qcoth__blk829 * assign36310_e40753);
        let assign36310_e40755: f64 = (locals.var_q_qsq__blk825 + assign36310_e40754);
        let assign36310_e40757: f64 = (assign36310_e40755 * locals.var_q_temp1__blk814);
        (assign36310_e40757, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign36310_e40753) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign36310_e40755 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign36310_e40753) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign36310_e40755 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign36310_e40753) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign36310_e40755 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign36310_e40753) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign36310_e40755 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign36310_e40753) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign36310_e40755 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign36310_e40759;
        locals.var_q_d1_qcoth__blk830_dn4 = assign36310_e40759_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign36310_e40759_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign36310_e40759_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign36310_e40759_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign36310_e40759_d_n9;

        let (assign36320_e40784, assign36320_e40784_d_n4, assign36320_e40784_d_n6, assign36320_e40784_d_n7, assign36320_e40784_d_n8, assign36320_e40784_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36320_e40769: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign36320_e40772: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign36320_e40773: f64 = (assign36320_e40769 * assign36320_e40772);
        let assign36320_e40774: f64 = (locals.var_q_d1_qsq__blk826 - assign36320_e40773);
        let assign36320_e40776: f64 = (assign36320_e40774 * locals.var_q_temp1__blk814);
        let assign36320_e40779: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign36320_e40781: f64 = (assign36320_e40779 / locals.var_q_d1_qsq__blk826);
        let assign36320_e40782: f64 = (assign36320_e40776 + assign36320_e40781);
        (assign36320_e40782, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign36320_e40772) + (assign36320_e40769 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign36320_e40774 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign36320_e40779 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign36320_e40772) + (assign36320_e40769 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign36320_e40774 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign36320_e40779 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign36320_e40772) + (assign36320_e40769 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign36320_e40774 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign36320_e40779 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign36320_e40772) + (assign36320_e40769 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign36320_e40774 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign36320_e40779 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign36320_e40772) + (assign36320_e40769 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign36320_e40774 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign36320_e40779 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign36320_e40784;
        locals.var_q_d2_qcoth__blk832_dn4 = assign36320_e40784_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign36320_e40784_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign36320_e40784_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign36320_e40784_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign36320_e40784_d_n9;

    }

    pub(super) fn stamp_transient_block_97(
        locals: &mut StampLocals,
    ) {
        let (assign36330_e40797, assign36330_e40797_d_n4, assign36330_e40797_d_n6, assign36330_e40797_d_n7, assign36330_e40797_d_n8, assign36330_e40797_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36330_e40794: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign36330_e40795: f64 = (1.0 - assign36330_e40794);
        (assign36330_e40795, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36330_e40797;
        locals.var_q_temp2__blk815_dn4 = assign36330_e40797_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36330_e40797_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36330_e40797_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36330_e40797_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36330_e40797_d_n9;

        let (assign36340_e40810, assign36340_e40810_d_n4, assign36340_e40810_d_n6, assign36340_e40810_d_n7, assign36340_e40810_d_n8, assign36340_e40810_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36340_e40806: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign36340_e40808: f64 = (assign36340_e40806 * locals.var_q_temp2__blk815);
        (assign36340_e40808, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36340_e40806 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36340_e40806 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36340_e40806 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36340_e40806 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign36340_e40806 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign36340_e40810;
        locals.var_q_d1_ln__blk835_dn4 = assign36340_e40810_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign36340_e40810_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign36340_e40810_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign36340_e40810_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign36340_e40810_d_n9;

        let (assign36350_e40831, assign36350_e40831_d_n4, assign36350_e40831_d_n6, assign36350_e40831_d_n7, assign36350_e40831_d_n8, assign36350_e40831_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 != 0.0)) {
        let assign36350_e40819: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign36350_e40824: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign36350_e40825: f64 = (locals.var_q_d1_ln__blk835 + assign36350_e40824);
        let assign36350_e40826: f64 = (locals.var_q_d1_qsq__blk826 * assign36350_e40825);
        let assign36350_e40827: f64 = (assign36350_e40819 - assign36350_e40826);
        let assign36350_e40829: f64 = (assign36350_e40827 / locals.var_q_qsq__blk825);
        (assign36350_e40829, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign36350_e40825) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign36350_e40827 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign36350_e40825) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign36350_e40827 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign36350_e40825) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign36350_e40827 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign36350_e40825) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign36350_e40827 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign36350_e40825) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign36350_e40827 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign36350_e40831;
        locals.var_q_d2_ln__blk836_dn4 = assign36350_e40831_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign36350_e40831_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign36350_e40831_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign36350_e40831_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign36350_e40831_d_n9;

        let (assign36360_e40859, assign36360_e40859_d_n4, assign36360_e40859_d_n6, assign36360_e40859_d_n7, assign36360_e40859_d_n8, assign36360_e40859_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36360_e40843: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign36360_e40847: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign36360_e40851: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign36360_e40852: f64 = (1.0 - assign36360_e40851);
        let assign36360_e40853: f64 = (assign36360_e40847 * assign36360_e40852);
        let assign36360_e40854: f64 = (1.0 - assign36360_e40853);
        let assign36360_e40855: f64 = (assign36360_e40843 * assign36360_e40854);
        let assign36360_e40856: f64 = (1.0 - assign36360_e40855);
        let assign36360_e40857: f64 = (0.1666666666667 * assign36360_e40856);
        (assign36360_e40857, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign36360_e40854) + (assign36360_e40843 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign36360_e40852) + (assign36360_e40847 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign36360_e40854) + (assign36360_e40843 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign36360_e40852) + (assign36360_e40847 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign36360_e40854) + (assign36360_e40843 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign36360_e40852) + (assign36360_e40847 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign36360_e40854) + (assign36360_e40843 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign36360_e40852) + (assign36360_e40847 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign36360_e40854) + (assign36360_e40843 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign36360_e40852) + (assign36360_e40847 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign36360_e40859;
        locals.var_q_temp3__blk816_dn4 = assign36360_e40859_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign36360_e40859_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign36360_e40859_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign36360_e40859_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign36360_e40859_d_n9;

        let (assign36370_e40873, assign36370_e40873_d_n4, assign36370_e40873_d_n6, assign36370_e40873_d_n7, assign36370_e40873_d_n8, assign36370_e40873_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36370_e40870: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign36370_e40871: f64 = (2.0 + assign36370_e40870);
        (assign36370_e40871, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign36370_e40873;
        locals.var_q_qcoth__blk829_dn4 = assign36370_e40873_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign36370_e40873_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign36370_e40873_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign36370_e40873_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign36370_e40873_d_n9;

        let (assign36380_e40901, assign36380_e40901_d_n4, assign36380_e40901_d_n6, assign36380_e40901_d_n7, assign36380_e40901_d_n8, assign36380_e40901_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36380_e40885: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign36380_e40889: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign36380_e40893: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign36380_e40894: f64 = (1.0 - assign36380_e40893);
        let assign36380_e40895: f64 = (assign36380_e40889 * assign36380_e40894);
        let assign36380_e40896: f64 = (1.0 - assign36380_e40895);
        let assign36380_e40897: f64 = (assign36380_e40885 * assign36380_e40896);
        let assign36380_e40898: f64 = (1.0 - assign36380_e40897);
        let assign36380_e40899: f64 = (0.1666666666667 * assign36380_e40898);
        (assign36380_e40899, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign36380_e40896) + (assign36380_e40885 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign36380_e40894) + (assign36380_e40889 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign36380_e40896) + (assign36380_e40885 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign36380_e40894) + (assign36380_e40889 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign36380_e40896) + (assign36380_e40885 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign36380_e40894) + (assign36380_e40889 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign36380_e40896) + (assign36380_e40885 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign36380_e40894) + (assign36380_e40889 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign36380_e40896) + (assign36380_e40885 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign36380_e40894) + (assign36380_e40889 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36380_e40901;
        locals.var_q_temp1__blk814_dn4 = assign36380_e40901_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36380_e40901_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36380_e40901_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36380_e40901_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36380_e40901_d_n9;

        let (assign36390_e40913, assign36390_e40913_d_n4, assign36390_e40913_d_n6, assign36390_e40913_d_n7, assign36390_e40913_d_n8, assign36390_e40913_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36390_e40911: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign36390_e40911, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign36390_e40913;
        locals.var_q_d1_qcoth__blk830_dn4 = assign36390_e40913_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign36390_e40913_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign36390_e40913_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign36390_e40913_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign36390_e40913_d_n9;

        let (assign36400_e40941, assign36400_e40941_d_n4, assign36400_e40941_d_n6, assign36400_e40941_d_n7, assign36400_e40941_d_n8, assign36400_e40941_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36400_e40925: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign36400_e40929: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign36400_e40933: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign36400_e40934: f64 = (1.0 - assign36400_e40933);
        let assign36400_e40935: f64 = (assign36400_e40929 * assign36400_e40934);
        let assign36400_e40936: f64 = (1.0 - assign36400_e40935);
        let assign36400_e40937: f64 = (assign36400_e40925 * assign36400_e40936);
        let assign36400_e40938: f64 = (1.0 - assign36400_e40937);
        let assign36400_e40939: f64 = (0.0055555555556 * assign36400_e40938);
        (assign36400_e40939, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign36400_e40936) + (assign36400_e40925 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign36400_e40934) + (assign36400_e40929 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign36400_e40936) + (assign36400_e40925 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign36400_e40934) + (assign36400_e40929 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign36400_e40936) + (assign36400_e40925 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign36400_e40934) + (assign36400_e40929 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign36400_e40936) + (assign36400_e40925 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign36400_e40934) + (assign36400_e40929 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign36400_e40936) + (assign36400_e40925 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign36400_e40934) + (assign36400_e40929 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36400_e40941;
        locals.var_q_temp2__blk815_dn4 = assign36400_e40941_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36400_e40941_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36400_e40941_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36400_e40941_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36400_e40941_d_n9;

        let (assign36410_e40959, assign36410_e40959_d_n4, assign36410_e40959_d_n6, assign36410_e40959_d_n7, assign36410_e40959_d_n8, assign36410_e40959_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36410_e40951: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign36410_e40954: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign36410_e40956: f64 = (assign36410_e40954 * locals.var_q_temp2__blk815);
        let assign36410_e40957: f64 = (assign36410_e40951 - assign36410_e40956);
        (assign36410_e40957, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign36410_e40954 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign36410_e40954 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign36410_e40954 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign36410_e40954 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign36410_e40954 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign36410_e40959;
        locals.var_q_d2_qcoth__blk832_dn4 = assign36410_e40959_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign36410_e40959_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign36410_e40959_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign36410_e40959_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign36410_e40959_d_n9;

        let (assign36420_e40974, assign36420_e40974_d_n4, assign36420_e40974_d_n6, assign36420_e40974_d_n7, assign36420_e40974_d_n8, assign36420_e40974_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36420_e40968: f64 = (-0.5);
        let assign36420_e40970: f64 = (assign36420_e40968 * locals.var_q_d1_qsq__blk826);
        let assign36420_e40972: f64 = (assign36420_e40970 * locals.var_q_temp3__blk816);
        (assign36420_e40972, (((assign36420_e40968 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign36420_e40970 * locals.var_q_temp3__blk816_dn4)), (((assign36420_e40968 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign36420_e40970 * locals.var_q_temp3__blk816_dn6)), (((assign36420_e40968 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign36420_e40970 * locals.var_q_temp3__blk816_dn7)), (((assign36420_e40968 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign36420_e40970 * locals.var_q_temp3__blk816_dn8)), (((assign36420_e40968 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign36420_e40970 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign36420_e40974;
        locals.var_q_d1_ln__blk835_dn4 = assign36420_e40974_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign36420_e40974_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign36420_e40974_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign36420_e40974_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign36420_e40974_d_n9;

        let (assign36430_e41009, assign36430_e41009_d_n4, assign36430_e41009_d_n6, assign36430_e41009_d_n7, assign36430_e41009_d_n8, assign36430_e41009_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1170 == 0.0)) && (locals.var_guard1171 == 0.0)) {
        let assign36430_e40983: f64 = (-0.5);
        let assign36430_e40985: f64 = (assign36430_e40983 * locals.var_q_d2_qsq__blk827);
        let assign36430_e40987: f64 = (assign36430_e40985 * locals.var_q_temp3__blk816);
        let assign36430_e40990: f64 = (0.25 * 0.0055555555556);
        let assign36430_e40992: f64 = (assign36430_e40990 * locals.var_q_d1_qsq__blk826);
        let assign36430_e40994: f64 = (assign36430_e40992 * locals.var_q_d1_qsq__blk826);
        let assign36430_e40998: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign36430_e41002: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign36430_e41003: f64 = (2.0 - assign36430_e41002);
        let assign36430_e41004: f64 = (assign36430_e40998 * assign36430_e41003);
        let assign36430_e41005: f64 = (1.0 - assign36430_e41004);
        let assign36430_e41006: f64 = (assign36430_e40994 * assign36430_e41005);
        let assign36430_e41007: f64 = (assign36430_e40987 + assign36430_e41006);
        (assign36430_e41007, ((((assign36430_e40983 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign36430_e40985 * locals.var_q_temp3__blk816_dn4)) + (((((assign36430_e40990 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign36430_e40992 * locals.var_q_d1_qsq__blk826_dn4)) * assign36430_e41005) + (assign36430_e40994 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign36430_e41003) + (assign36430_e40998 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign36430_e40983 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign36430_e40985 * locals.var_q_temp3__blk816_dn6)) + (((((assign36430_e40990 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign36430_e40992 * locals.var_q_d1_qsq__blk826_dn6)) * assign36430_e41005) + (assign36430_e40994 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign36430_e41003) + (assign36430_e40998 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign36430_e40983 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign36430_e40985 * locals.var_q_temp3__blk816_dn7)) + (((((assign36430_e40990 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign36430_e40992 * locals.var_q_d1_qsq__blk826_dn7)) * assign36430_e41005) + (assign36430_e40994 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign36430_e41003) + (assign36430_e40998 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign36430_e40983 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign36430_e40985 * locals.var_q_temp3__blk816_dn8)) + (((((assign36430_e40990 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign36430_e40992 * locals.var_q_d1_qsq__blk826_dn8)) * assign36430_e41005) + (assign36430_e40994 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign36430_e41003) + (assign36430_e40998 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign36430_e40983 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign36430_e40985 * locals.var_q_temp3__blk816_dn9)) + (((((assign36430_e40990 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign36430_e40992 * locals.var_q_d1_qsq__blk826_dn9)) * assign36430_e41005) + (assign36430_e40994 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign36430_e41003) + (assign36430_e40998 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign36430_e41009;
        locals.var_q_d2_ln__blk836_dn4 = assign36430_e41009_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign36430_e41009_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign36430_e41009_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign36430_e41009_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign36430_e41009_d_n9;

        let assign36440_e41012: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign36440_e41012;

        let (assign36450_e41028, assign36450_e41028_d_n4, assign36450_e41028_d_n6, assign36450_e41028_d_n7, assign36450_e41028_d_n8, assign36450_e41028_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign36450_e41018: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign36450_e41023: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign36450_e41024: f64 = (locals.var_q_invexpq__blk831 * assign36450_e41023);
        let assign36450_e41025: f64 = (1.0 - assign36450_e41024);
        let assign36450_e41026: f64 = (assign36450_e41018 / assign36450_e41025);
        (assign36450_e41026, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign36450_e41025) - (assign36450_e41018 * (-((locals.var_q_invexpq__blk831_dn4 * assign36450_e41023) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign36450_e41025 * assign36450_e41025)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign36450_e41025) - (assign36450_e41018 * (-((locals.var_q_invexpq__blk831_dn6 * assign36450_e41023) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign36450_e41025 * assign36450_e41025)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign36450_e41025) - (assign36450_e41018 * (-((locals.var_q_invexpq__blk831_dn7 * assign36450_e41023) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign36450_e41025 * assign36450_e41025)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign36450_e41025) - (assign36450_e41018 * (-((locals.var_q_invexpq__blk831_dn8 * assign36450_e41023) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign36450_e41025 * assign36450_e41025)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign36450_e41025) - (assign36450_e41018 * (-((locals.var_q_invexpq__blk831_dn9 * assign36450_e41023) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign36450_e41025 * assign36450_e41025)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36450_e41028;
        locals.var_q_temp2__blk815_dn4 = assign36450_e41028_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36450_e41028_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36450_e41028_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36450_e41028_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36450_e41028_d_n9;

        let (assign36460_e41036, assign36460_e41036_d_n4, assign36460_e41036_d_n6, assign36460_e41036_d_n7, assign36460_e41036_d_n8, assign36460_e41036_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign36460_e41034: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign36460_e41034, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign36460_e41036;
        locals.var_q_sh_term__blk833_dn4 = assign36460_e41036_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign36460_e41036_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign36460_e41036_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign36460_e41036_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign36460_e41036_d_n9;

        let (assign36470_e41045, assign36470_e41045_d_n4, assign36470_e41045_d_n6, assign36470_e41045_d_n7, assign36470_e41045_d_n8, assign36470_e41045_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1172 != 0.0)) {
        let assign36470_e41041: f64 = (locals.var_q_temp2__blk815).ln();
        let assign36470_e41043: f64 = (assign36470_e41041 - locals.var_q_rac_qsq__blk828);
        (assign36470_e41043, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign36470_e41045;
        locals.var_q_ln_term__blk834_dn4 = assign36470_e41045_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign36470_e41045_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign36470_e41045_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign36470_e41045_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign36470_e41045_d_n9;

        let assign36480_e41048: f64 = (-0.005);
        let assign36480_e41049: f64 = if locals.var_q_qsq__blk825 < assign36480_e41048 { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign36480_e41049;

        let (assign36490_e41061, assign36490_e41061_d_n4, assign36490_e41061_d_n6, assign36490_e41061_d_n7, assign36490_e41061_d_n8, assign36490_e41061_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign36490_e41058: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign36490_e41059: f64 = (assign36490_e41058).sin();
        (assign36490_e41059, ((assign36490_e41058).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign36490_e41058).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign36490_e41058).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign36490_e41058).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign36490_e41058).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36490_e41061;
        locals.var_q_temp2__blk815_dn4 = assign36490_e41061_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36490_e41061_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36490_e41061_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36490_e41061_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36490_e41061_d_n9;

        let (assign36500_e41075, assign36500_e41075_d_n4, assign36500_e41075_d_n6, assign36500_e41075_d_n7, assign36500_e41075_d_n8, assign36500_e41075_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign36500_e41069: f64 = (-locals.var_q_qsq__blk825);
        let assign36500_e41072: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign36500_e41073: f64 = (assign36500_e41069 / assign36500_e41072);
        (assign36500_e41073, ((((-locals.var_q_qsq__blk825_dn4) * assign36500_e41072) - (assign36500_e41069 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign36500_e41072 * assign36500_e41072)), ((((-locals.var_q_qsq__blk825_dn6) * assign36500_e41072) - (assign36500_e41069 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign36500_e41072 * assign36500_e41072)), ((((-locals.var_q_qsq__blk825_dn7) * assign36500_e41072) - (assign36500_e41069 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign36500_e41072 * assign36500_e41072)), ((((-locals.var_q_qsq__blk825_dn8) * assign36500_e41072) - (assign36500_e41069 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign36500_e41072 * assign36500_e41072)), ((((-locals.var_q_qsq__blk825_dn9) * assign36500_e41072) - (assign36500_e41069 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign36500_e41072 * assign36500_e41072)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign36500_e41075;
        locals.var_q_sh_term__blk833_dn4 = assign36500_e41075_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign36500_e41075_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign36500_e41075_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign36500_e41075_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign36500_e41075_d_n9;

        let (assign36510_e41085, assign36510_e41085_d_n4, assign36510_e41085_d_n6, assign36510_e41085_d_n7, assign36510_e41085_d_n8, assign36510_e41085_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 != 0.0)) {
        let assign36510_e41083: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign36510_e41083, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign36510_e41085;
        locals.var_q_ln_term__blk834_dn4 = assign36510_e41085_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign36510_e41085_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign36510_e41085_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign36510_e41085_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign36510_e41085_d_n9;

        let (assign36520_e41111, assign36520_e41111_d_n4, assign36520_e41111_d_n6, assign36520_e41111_d_n7, assign36520_e41111_d_n8, assign36520_e41111_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 == 0.0)) {
        let assign36520_e41096: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign36520_e41100: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign36520_e41104: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign36520_e41105: f64 = (1.0 - assign36520_e41104);
        let assign36520_e41106: f64 = (assign36520_e41100 * assign36520_e41105);
        let assign36520_e41107: f64 = (1.0 - assign36520_e41106);
        let assign36520_e41108: f64 = (assign36520_e41096 * assign36520_e41107);
        let assign36520_e41109: f64 = (4.0 - assign36520_e41108);
        (assign36520_e41109, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign36520_e41107) + (assign36520_e41096 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign36520_e41105) + (assign36520_e41100 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign36520_e41107) + (assign36520_e41096 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign36520_e41105) + (assign36520_e41100 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign36520_e41107) + (assign36520_e41096 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign36520_e41105) + (assign36520_e41100 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign36520_e41107) + (assign36520_e41096 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign36520_e41105) + (assign36520_e41100 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign36520_e41107) + (assign36520_e41096 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign36520_e41105) + (assign36520_e41100 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign36520_e41111;
        locals.var_q_sh_term__blk833_dn4 = assign36520_e41111_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign36520_e41111_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign36520_e41111_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign36520_e41111_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign36520_e41111_d_n9;

        let (assign36530_e41122, assign36530_e41122_d_n4, assign36530_e41122_d_n6, assign36530_e41122_d_n7, assign36530_e41122_d_n8, assign36530_e41122_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1172 == 0.0)) && (locals.var_guard1173 == 0.0)) {
        let assign36530_e41120: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign36530_e41120, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign36530_e41122;
        locals.var_q_ln_term__blk834_dn4 = assign36530_e41122_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign36530_e41122_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign36530_e41122_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign36530_e41122_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign36530_e41122_d_n9;

        let assign36540_e41125: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign36540_e41127: f64 = (assign36540_e41125 + locals.var_q_qcoth__blk829);
        let assign36540_e41129: f64 = if assign36540_e41127 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign36540_e41129;

        let (assign36550_e41137, assign36550_e41137_d_n4, assign36550_e41137_d_n6, assign36550_e41137_d_n7, assign36550_e41137_d_n8, assign36550_e41137_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 != 0.0)) {
        let assign36550_e41135: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign36550_e41135, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign36550_e41137;
        locals.var_q_expnum__blk837_dn4 = assign36550_e41137_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign36550_e41137_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign36550_e41137_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign36550_e41137_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign36550_e41137_d_n9;

        let (assign36560_e41145, assign36560_e41145_d_n4, assign36560_e41145_d_n6, assign36560_e41145_d_n7, assign36560_e41145_d_n8, assign36560_e41145_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 != 0.0)) {
        let assign36560_e41143: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign36560_e41143, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign36560_e41145;
        locals.var_q_d1_expnum__blk838_dn4 = assign36560_e41145_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign36560_e41145_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign36560_e41145_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign36560_e41145_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign36560_e41145_d_n9;

        let (assign36570_e41151, assign36570_e41151_d_n4, assign36570_e41151_d_n6, assign36570_e41151_d_n7, assign36570_e41151_d_n8, assign36570_e41151_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign36570_e41151;
        locals.var_q_d2_expnum__blk839_dn4 = assign36570_e41151_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign36570_e41151_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign36570_e41151_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign36570_e41151_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign36570_e41151_d_n9;

        let (assign36580_e41162, assign36580_e41162_d_n4, assign36580_e41162_d_n6, assign36580_e41162_d_n7, assign36580_e41162_d_n8, assign36580_e41162_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36580_e41159: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign36580_e41160: f64 = (1.0 / assign36580_e41159);
        (assign36580_e41160, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign36580_e41159 * assign36580_e41159))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign36580_e41159 * assign36580_e41159))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign36580_e41159 * assign36580_e41159))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign36580_e41159 * assign36580_e41159))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign36580_e41159 * assign36580_e41159))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign36580_e41162;
        locals.var_q_temp2__blk815_dn4 = assign36580_e41162_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign36580_e41162_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign36580_e41162_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign36580_e41162_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign36580_e41162_d_n9;

        let (assign36590_e41171, assign36590_e41171_d_n4, assign36590_e41171_d_n6, assign36590_e41171_d_n7, assign36590_e41171_d_n8, assign36590_e41171_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36590_e41169: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign36590_e41169, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign36590_e41171;
        locals.var_q_temp3__blk816_dn4 = assign36590_e41171_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign36590_e41171_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign36590_e41171_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign36590_e41171_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign36590_e41171_d_n9;

        let (assign36600_e41182, assign36600_e41182_d_n4, assign36600_e41182_d_n6, assign36600_e41182_d_n7, assign36600_e41182_d_n8, assign36600_e41182_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36600_e41178: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign36600_e41180: f64 = (assign36600_e41178 * locals.var_q_temp2__blk815);
        (assign36600_e41180, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign36600_e41178 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign36600_e41178 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign36600_e41178 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign36600_e41178 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign36600_e41178 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign36600_e41182;
        locals.var_q_expnum__blk837_dn4 = assign36600_e41182_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign36600_e41182_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign36600_e41182_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign36600_e41182_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign36600_e41182_d_n9;

        let (assign36610_e41199, assign36610_e41199_d_n4, assign36610_e41199_d_n6, assign36610_e41199_d_n7, assign36610_e41199_d_n8, assign36610_e41199_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36610_e41189: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign36610_e41191: f64 = (assign36610_e41189 - locals.var_q_aexp__blk824);
        let assign36610_e41194: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign36610_e41195: f64 = (assign36610_e41191 - assign36610_e41194);
        let assign36610_e41197: f64 = (assign36610_e41195 * locals.var_q_temp2__blk815);
        (assign36610_e41197, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign36610_e41195 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign36610_e41195 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign36610_e41195 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign36610_e41195 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign36610_e41195 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign36610_e41199;
        locals.var_q_d1_expnum__blk838_dn4 = assign36610_e41199_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign36610_e41199_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign36610_e41199_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign36610_e41199_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign36610_e41199_d_n9;

        let (assign36620_e41226, assign36620_e41226_d_n4, assign36620_e41226_d_n6, assign36620_e41226_d_n7, assign36620_e41226_d_n8, assign36620_e41226_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign36620_e41206: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign36620_e41209: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign36620_e41211: f64 = (assign36620_e41209 * locals.var_q_d1_expnum__blk838);
        let assign36620_e41212: f64 = (assign36620_e41206 + assign36620_e41211);
        let assign36620_e41214: f64 = (assign36620_e41212 + locals.var_q_aexp__blk824);
        let assign36620_e41218: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign36620_e41219: f64 = (locals.var_q_d2_ln__blk836 + assign36620_e41218);
        let assign36620_e41221: f64 = (assign36620_e41219 * locals.var_q_sh_term__blk833);
        let assign36620_e41222: f64 = (assign36620_e41214 - assign36620_e41221);
        let assign36620_e41224: f64 = (assign36620_e41222 * locals.var_q_temp2__blk815);
        (assign36620_e41224, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign36620_e41209 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign36620_e41219 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign36620_e41222 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign36620_e41209 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign36620_e41219 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign36620_e41222 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign36620_e41209 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign36620_e41219 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign36620_e41222 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign36620_e41209 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign36620_e41219 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign36620_e41222 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign36620_e41209 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign36620_e41219 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign36620_e41222 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign36620_e41226;
        locals.var_q_d2_expnum__blk839_dn4 = assign36620_e41226_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign36620_e41226_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign36620_e41226_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign36620_e41226_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign36620_e41226_d_n9;

        let assign36630_e41229: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign36630_e41229;

    }

    pub(super) fn stamp_transient_block_98(
        locals: &mut StampLocals,
    ) {
        let (assign36640_e41236, assign36640_e41236_d_n4, assign36640_e41236_d_n6, assign36640_e41236_d_n7, assign36640_e41236_d_n8, assign36640_e41236_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36640_e41234: f64 = (locals.var_q_expnum__blk837).ln();
        (assign36640_e41234, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign36640_e41236;
        locals.var_q_lnexpnum__blk840_dn4 = assign36640_e41236_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign36640_e41236_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign36640_e41236_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign36640_e41236_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign36640_e41236_d_n9;

        let (assign36650_e41244, assign36650_e41244_d_n4, assign36650_e41244_d_n6, assign36650_e41244_d_n7, assign36650_e41244_d_n8, assign36650_e41244_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36650_e41242: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign36650_e41242, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36650_e41244;
        locals.var_q_temp1__blk814_dn4 = assign36650_e41244_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36650_e41244_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36650_e41244_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36650_e41244_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36650_e41244_d_n9;

        let (assign36660_e41252, assign36660_e41252_d_n4, assign36660_e41252_d_n6, assign36660_e41252_d_n7, assign36660_e41252_d_n8, assign36660_e41252_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36660_e41250: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign36660_e41250, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign36660_e41252;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign36660_e41252_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign36660_e41252_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign36660_e41252_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign36660_e41252_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign36660_e41252_d_n9;

        let (assign36670_e41264, assign36670_e41264_d_n4, assign36670_e41264_d_n6, assign36670_e41264_d_n7, assign36670_e41264_d_n8, assign36670_e41264_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 != 0.0)) {
        let assign36670_e41258: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign36670_e41261: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign36670_e41262: f64 = (assign36670_e41258 - assign36670_e41261);
        (assign36670_e41262, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign36670_e41264;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign36670_e41264_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign36670_e41264_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign36670_e41264_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign36670_e41264_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign36670_e41264_d_n9;

        let (assign36680_e41277, assign36680_e41277_d_n4, assign36680_e41277_d_n6, assign36680_e41277_d_n7, assign36680_e41277_d_n8, assign36680_e41277_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36680_e41271: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign36680_e41273: f64 = (-locals.var_q_k1q1__blk823);
        let assign36680_e41274: f64 = (assign36680_e41273).ln();
        let assign36680_e41275: f64 = (assign36680_e41271 + assign36680_e41274);
        (assign36680_e41275, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign36680_e41273)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign36680_e41273)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign36680_e41273)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign36680_e41273)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign36680_e41273)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign36680_e41277;
        locals.var_q_lnexpnum__blk840_dn4 = assign36680_e41277_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign36680_e41277_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign36680_e41277_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign36680_e41277_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign36680_e41277_d_n9;

        let (assign36690_e41286, assign36690_e41286_d_n4, assign36690_e41286_d_n6, assign36690_e41286_d_n7, assign36690_e41286_d_n8, assign36690_e41286_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36690_e41284: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign36690_e41284, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign36690_e41286;
        locals.var_q_temp1__blk814_dn4 = assign36690_e41286_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign36690_e41286_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign36690_e41286_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign36690_e41286_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign36690_e41286_d_n9;

        let (assign36700_e41295, assign36700_e41295_d_n4, assign36700_e41295_d_n6, assign36700_e41295_d_n7, assign36700_e41295_d_n8, assign36700_e41295_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36700_e41293: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign36700_e41293, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign36700_e41295;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign36700_e41295_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign36700_e41295_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign36700_e41295_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign36700_e41295_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign36700_e41295_d_n9;

        let (assign36710_e41305, assign36710_e41305_d_n4, assign36710_e41305_d_n6, assign36710_e41305_d_n7, assign36710_e41305_d_n8, assign36710_e41305_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1175 == 0.0)) {
        let assign36710_e41301: f64 = (-locals.var_q_temp1__blk814);
        let assign36710_e41303: f64 = (assign36710_e41301 * locals.var_q_temp1__blk814);
        (assign36710_e41303, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign36710_e41301 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign36710_e41301 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign36710_e41301 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign36710_e41301 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign36710_e41301 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign36710_e41305;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign36710_e41305_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign36710_e41305_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign36710_e41305_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign36710_e41305_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign36710_e41305_d_n9;

        let (assign36720_e41319, assign36720_e41319_d_n4, assign36720_e41319_d_n6, assign36720_e41319_d_n7, assign36720_e41319_d_n8, assign36720_e41319_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36720_e41309: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign36720_e41311: f64 = (assign36720_e41309 + locals.var_q1d__blk1001);
        let assign36720_e41314: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign36720_e41315: f64 = (assign36720_e41311 + assign36720_e41314);
        let assign36720_e41317: f64 = (assign36720_e41315 - locals.var_q_ln_term__blk834);
        (assign36720_e41317, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign36720_e41319;
        locals.var_q_q2_int__blk843_dn4 = assign36720_e41319_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign36720_e41319_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign36720_e41319_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign36720_e41319_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign36720_e41319_d_n9;

        let (assign36730_e41329, assign36730_e41329_d_n4, assign36730_e41329_d_n6, assign36730_e41329_d_n7, assign36730_e41329_d_n8, assign36730_e41329_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36730_e41324: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign36730_e41325: f64 = (1.0 + assign36730_e41324);
        let assign36730_e41327: f64 = (assign36730_e41325 - locals.var_q_d1_ln__blk835);
        (assign36730_e41327, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign36730_e41329;
        locals.var_q_d1_q2__blk844_dn4 = assign36730_e41329_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign36730_e41329_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign36730_e41329_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign36730_e41329_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign36730_e41329_d_n9;

        let (assign36740_e41337, assign36740_e41337_d_n4, assign36740_e41337_d_n6, assign36740_e41337_d_n7, assign36740_e41337_d_n8, assign36740_e41337_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36740_e41333: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign36740_e41335: f64 = (assign36740_e41333 - locals.var_q_d2_ln__blk836);
        (assign36740_e41335, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign36740_e41337;
        locals.var_q_d2_q2__blk845_dn4 = assign36740_e41337_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign36740_e41337_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign36740_e41337_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign36740_e41337_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign36740_e41337_d_n9;

        let (assign36750_e41345, assign36750_e41345_d_n4, assign36750_e41345_d_n6, assign36750_e41345_d_n7, assign36750_e41345_d_n8, assign36750_e41345_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36750_e41342: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign36750_e41343: f64 = (locals.var_q_k1q1__blk823 + assign36750_e41342);
        (assign36750_e41343, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign36750_e41345;
        locals.var_q_qi_int__blk846_dn4 = assign36750_e41345_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign36750_e41345_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign36750_e41345_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign36750_e41345_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign36750_e41345_d_n9;

        let (assign36760_e41353, assign36760_e41353_d_n4, assign36760_e41353_d_n6, assign36760_e41353_d_n7, assign36760_e41353_d_n8, assign36760_e41353_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36760_e41350: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign36760_e41351: f64 = (locals.var_k1__blk932 + assign36760_e41350);
        (assign36760_e41351, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign36760_e41353;
        locals.var_q_d1_qi__blk847_dn4 = assign36760_e41353_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign36760_e41353_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign36760_e41353_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign36760_e41353_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign36760_e41353_d_n9;

        let (assign36770_e41359, assign36770_e41359_d_n4, assign36770_e41359_d_n6, assign36770_e41359_d_n7, assign36770_e41359_d_n8, assign36770_e41359_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36770_e41357: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign36770_e41357, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign36770_e41359;
        locals.var_q_d2_qi__blk848_dn4 = assign36770_e41359_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign36770_e41359_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign36770_e41359_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign36770_e41359_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign36770_e41359_d_n9;

        let (assign36780_e41367, assign36780_e41367_d_n4, assign36780_e41367_d_n6, assign36780_e41367_d_n7, assign36780_e41367_d_n8, assign36780_e41367_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36780_e41363: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign36780_e41365: f64 = (assign36780_e41363 - locals.var_q_aexp__blk824);
        (assign36780_e41365, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign36780_e41367;
        locals.var_q_zero__blk849_dn4 = assign36780_e41367_d_n4;
        locals.var_q_zero__blk849_dn6 = assign36780_e41367_d_n6;
        locals.var_q_zero__blk849_dn7 = assign36780_e41367_d_n7;
        locals.var_q_zero__blk849_dn8 = assign36780_e41367_d_n8;
        locals.var_q_zero__blk849_dn9 = assign36780_e41367_d_n9;

        let (assign36790_e41379, assign36790_e41379_d_n4, assign36790_e41379_d_n6, assign36790_e41379_d_n7, assign36790_e41379_d_n8, assign36790_e41379_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36790_e41371: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign36790_e41374: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign36790_e41375: f64 = (assign36790_e41371 + assign36790_e41374);
        let assign36790_e41377: f64 = (assign36790_e41375 + locals.var_q_aexp__blk824);
        (assign36790_e41377, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign36790_e41379;
        locals.var_q_d1_zero__blk850_dn4 = assign36790_e41379_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign36790_e41379_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign36790_e41379_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign36790_e41379_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign36790_e41379_d_n9;

        let (assign36800_e41397, assign36800_e41397_d_n4, assign36800_e41397_d_n6, assign36800_e41397_d_n7, assign36800_e41397_d_n8, assign36800_e41397_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36800_e41383: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign36800_e41386: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign36800_e41388: f64 = (assign36800_e41386 * locals.var_q_d1_expnum__blk838);
        let assign36800_e41389: f64 = (assign36800_e41383 + assign36800_e41388);
        let assign36800_e41392: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign36800_e41393: f64 = (assign36800_e41389 + assign36800_e41392);
        let assign36800_e41395: f64 = (assign36800_e41393 - locals.var_q_aexp__blk824);
        (assign36800_e41395, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign36800_e41386 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign36800_e41386 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign36800_e41386 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign36800_e41386 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign36800_e41386 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign36800_e41397;
        locals.var_q_d2_zero__blk851_dn4 = assign36800_e41397_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign36800_e41397_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign36800_e41397_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign36800_e41397_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign36800_e41397_d_n9;

        let (assign36810_e41409, assign36810_e41409_d_n4, assign36810_e41409_d_n6, assign36810_e41409_d_n7, assign36810_e41409_d_n8, assign36810_e41409_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36810_e41401: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign36810_e41404: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign36810_e41406: f64 = (assign36810_e41404 * locals.var_q_d2_zero__blk851);
        let assign36810_e41407: f64 = (assign36810_e41401 - assign36810_e41406);
        (assign36810_e41407, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign36810_e41404 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign36810_e41404 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign36810_e41404 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign36810_e41404 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign36810_e41404 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign36810_e41409;
        locals.var_q_temp__blk860_dn4 = assign36810_e41409_d_n4;
        locals.var_q_temp__blk860_dn6 = assign36810_e41409_d_n6;
        locals.var_q_temp__blk860_dn7 = assign36810_e41409_d_n7;
        locals.var_q_temp__blk860_dn8 = assign36810_e41409_d_n8;
        locals.var_q_temp__blk860_dn9 = assign36810_e41409_d_n9;

        let (assign36820_e41424, assign36820_e41424_d_n4, assign36820_e41424_d_n6, assign36820_e41424_d_n7, assign36820_e41424_d_n8, assign36820_e41424_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36820_e41412: f64 = (-locals.var_q_zero__blk849);
        let assign36820_e41414: f64 = (assign36820_e41412 * locals.var_q_d1_zero__blk850);
        let assign36820_e41416: f64 = (assign36820_e41414 * locals.var_q_temp__blk860);
        let assign36820_e41419: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign36820_e41421: f64 = (assign36820_e41419 + 1e-200);
        let assign36820_e41422: f64 = (assign36820_e41416 / assign36820_e41421);
        (assign36820_e41422, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign36820_e41412 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign36820_e41414 * locals.var_q_temp__blk860_dn4)) * assign36820_e41421) - (assign36820_e41416 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign36820_e41421 * assign36820_e41421)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign36820_e41412 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign36820_e41414 * locals.var_q_temp__blk860_dn6)) * assign36820_e41421) - (assign36820_e41416 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign36820_e41421 * assign36820_e41421)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign36820_e41412 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign36820_e41414 * locals.var_q_temp__blk860_dn7)) * assign36820_e41421) - (assign36820_e41416 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign36820_e41421 * assign36820_e41421)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign36820_e41412 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign36820_e41414 * locals.var_q_temp__blk860_dn8)) * assign36820_e41421) - (assign36820_e41416 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign36820_e41421 * assign36820_e41421)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign36820_e41412 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign36820_e41414 * locals.var_q_temp__blk860_dn9)) * assign36820_e41421) - (assign36820_e41416 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign36820_e41421 * assign36820_e41421)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign36820_e41424;
        locals.var_q_eps2__blk852_dn4 = assign36820_e41424_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign36820_e41424_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign36820_e41424_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign36820_e41424_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign36820_e41424_d_n9;

        let (assign36830_e41430, assign36830_e41430_d_n4, assign36830_e41430_d_n6, assign36830_e41430_d_n7, assign36830_e41430_d_n8, assign36830_e41430_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36830_e41428: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign36830_e41428, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign36830_e41430;
        locals.var_q1d__blk1001_dn4 = assign36830_e41430_d_n4;
        locals.var_q1d__blk1001_dn6 = assign36830_e41430_d_n6;
        locals.var_q1d__blk1001_dn7 = assign36830_e41430_d_n7;
        locals.var_q1d__blk1001_dn8 = assign36830_e41430_d_n8;
        locals.var_q1d__blk1001_dn9 = assign36830_e41430_d_n9;

        let (assign36840_e41436, assign36840_e41436_d_n4, assign36840_e41436_d_n6, assign36840_e41436_d_n7, assign36840_e41436_d_n8, assign36840_e41436_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36840_e41434: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign36840_e41434, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign36840_e41436;
        locals.var_q_k1q1__blk823_dn4 = assign36840_e41436_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign36840_e41436_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign36840_e41436_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign36840_e41436_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign36840_e41436_d_n9;

        let (assign36850_e41442, assign36850_e41442_d_n4, assign36850_e41442_d_n6, assign36850_e41442_d_n7, assign36850_e41442_d_n8, assign36850_e41442_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36850_e41440: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
        (assign36850_e41440, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)),)
    } else {
        (locals.var_q_k2q2__blk853, locals.var_q_k2q2__blk853_dn4, locals.var_q_k2q2__blk853_dn6, locals.var_q_k2q2__blk853_dn7, locals.var_q_k2q2__blk853_dn8, locals.var_q_k2q2__blk853_dn9,)
    }
};
        locals.var_q_k2q2__blk853 = assign36850_e41442;
        locals.var_q_k2q2__blk853_dn4 = assign36850_e41442_d_n4;
        locals.var_q_k2q2__blk853_dn6 = assign36850_e41442_d_n6;
        locals.var_q_k2q2__blk853_dn7 = assign36850_e41442_d_n7;
        locals.var_q_k2q2__blk853_dn8 = assign36850_e41442_d_n8;
        locals.var_q_k2q2__blk853_dn9 = assign36850_e41442_d_n9;

        let (assign36860_e41448, assign36860_e41448_d_n4, assign36860_e41448_d_n6, assign36860_e41448_d_n7, assign36860_e41448_d_n8, assign36860_e41448_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36860_e41446: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_k2q2__blk853);
        (assign36860_e41446, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_k2q2__blk853_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_k2q2__blk853_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_k2q2__blk853_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_k2q2__blk853_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_k2q2__blk853_dn9),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign36860_e41448;
        locals.var_q_qi_int__blk846_dn4 = assign36860_e41448_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign36860_e41448_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign36860_e41448_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign36860_e41448_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign36860_e41448_d_n9;

        let (assign36870_e41456, assign36870_e41456_d_n4, assign36870_e41456_d_n6, assign36870_e41456_d_n7, assign36870_e41456_d_n8, assign36870_e41456_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36870_e41453: f64 = (0.065345483024 * locals.var_q_qi_int__blk846);
        let assign36870_e41454: f64 = (1.0 + assign36870_e41453);
        (assign36870_e41454, (0.065345483024 * locals.var_q_qi_int__blk846_dn4), (0.065345483024 * locals.var_q_qi_int__blk846_dn6), (0.065345483024 * locals.var_q_qi_int__blk846_dn7), (0.065345483024 * locals.var_q_qi_int__blk846_dn8), (0.065345483024 * locals.var_q_qi_int__blk846_dn9),)
    } else {
        (locals.var_q_a__blk854, locals.var_q_a__blk854_dn4, locals.var_q_a__blk854_dn6, locals.var_q_a__blk854_dn7, locals.var_q_a__blk854_dn8, locals.var_q_a__blk854_dn9,)
    }
};
        locals.var_q_a__blk854 = assign36870_e41456;
        locals.var_q_a__blk854_dn4 = assign36870_e41456_d_n4;
        locals.var_q_a__blk854_dn6 = assign36870_e41456_d_n6;
        locals.var_q_a__blk854_dn7 = assign36870_e41456_d_n7;
        locals.var_q_a__blk854_dn8 = assign36870_e41456_d_n8;
        locals.var_q_a__blk854_dn9 = assign36870_e41456_d_n9;

        let (assign36880_e41468, assign36880_e41468_d_n4, assign36880_e41468_d_n6, assign36880_e41468_d_n7, assign36880_e41468_d_n8, assign36880_e41468_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36880_e41461: f64 = (8.5797362674 * locals.var_q_qi_int__blk846);
        let assign36880_e41462: f64 = (39.478417604 + assign36880_e41461);
        let assign36880_e41465: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign36880_e41466: f64 = (assign36880_e41462 + assign36880_e41465);
        (assign36880_e41466, ((8.5797362674 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))),)
    } else {
        (locals.var_q_b__blk855, locals.var_q_b__blk855_dn4, locals.var_q_b__blk855_dn6, locals.var_q_b__blk855_dn7, locals.var_q_b__blk855_dn8, locals.var_q_b__blk855_dn9,)
    }
};
        locals.var_q_b__blk855 = assign36880_e41468;
        locals.var_q_b__blk855_dn4 = assign36880_e41468_d_n4;
        locals.var_q_b__blk855_dn6 = assign36880_e41468_d_n6;
        locals.var_q_b__blk855_dn7 = assign36880_e41468_d_n7;
        locals.var_q_b__blk855_dn8 = assign36880_e41468_d_n8;
        locals.var_q_b__blk855_dn9 = assign36880_e41468_d_n9;

        let (assign36890_e41480, assign36890_e41480_d_n4, assign36890_e41480_d_n6, assign36890_e41480_d_n7, assign36890_e41480_d_n8, assign36890_e41480_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36890_e41473: f64 = (2.0 * locals.var_q_qi_int__blk846);
        let assign36890_e41476: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign36890_e41477: f64 = (assign36890_e41473 + assign36890_e41476);
        let assign36890_e41478: f64 = (39.478417604 * assign36890_e41477);
        (assign36890_e41478, (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9)))),)
    } else {
        (locals.var_q_c__blk856, locals.var_q_c__blk856_dn4, locals.var_q_c__blk856_dn6, locals.var_q_c__blk856_dn7, locals.var_q_c__blk856_dn8, locals.var_q_c__blk856_dn9,)
    }
};
        locals.var_q_c__blk856 = assign36890_e41480;
        locals.var_q_c__blk856_dn4 = assign36890_e41480_d_n4;
        locals.var_q_c__blk856_dn6 = assign36890_e41480_d_n6;
        locals.var_q_c__blk856_dn7 = assign36890_e41480_d_n7;
        locals.var_q_c__blk856_dn8 = assign36890_e41480_d_n8;
        locals.var_q_c__blk856_dn9 = assign36890_e41480_d_n9;

        let (assign36900_e41493, assign36900_e41493_d_n4, assign36900_e41493_d_n6, assign36900_e41493_d_n7, assign36900_e41493_d_n8, assign36900_e41493_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36900_e41484: f64 = (locals.var_q_b__blk855 * locals.var_q_b__blk855);
        let assign36900_e41487: f64 = (4.0 * locals.var_q_a__blk854);
        let assign36900_e41489: f64 = (assign36900_e41487 * locals.var_q_c__blk856);
        let assign36900_e41490: f64 = (assign36900_e41484 - assign36900_e41489);
        let assign36900_e41491: f64 = (assign36900_e41490).sqrt();
        (assign36900_e41491, ((((locals.var_q_b__blk855_dn4 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn4)) - (((4.0 * locals.var_q_a__blk854_dn4) * locals.var_q_c__blk856) + (assign36900_e41487 * locals.var_q_c__blk856_dn4))) / (2.0 * assign36900_e41491)), ((((locals.var_q_b__blk855_dn6 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn6)) - (((4.0 * locals.var_q_a__blk854_dn6) * locals.var_q_c__blk856) + (assign36900_e41487 * locals.var_q_c__blk856_dn6))) / (2.0 * assign36900_e41491)), ((((locals.var_q_b__blk855_dn7 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn7)) - (((4.0 * locals.var_q_a__blk854_dn7) * locals.var_q_c__blk856) + (assign36900_e41487 * locals.var_q_c__blk856_dn7))) / (2.0 * assign36900_e41491)), ((((locals.var_q_b__blk855_dn8 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn8)) - (((4.0 * locals.var_q_a__blk854_dn8) * locals.var_q_c__blk856) + (assign36900_e41487 * locals.var_q_c__blk856_dn8))) / (2.0 * assign36900_e41491)), ((((locals.var_q_b__blk855_dn9 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn9)) - (((4.0 * locals.var_q_a__blk854_dn9) * locals.var_q_c__blk856) + (assign36900_e41487 * locals.var_q_c__blk856_dn9))) / (2.0 * assign36900_e41491)),)
    } else {
        (locals.var_q_disc__blk857, locals.var_q_disc__blk857_dn4, locals.var_q_disc__blk857_dn6, locals.var_q_disc__blk857_dn7, locals.var_q_disc__blk857_dn8, locals.var_q_disc__blk857_dn9,)
    }
};
        locals.var_q_disc__blk857 = assign36900_e41493;
        locals.var_q_disc__blk857_dn4 = assign36900_e41493_d_n4;
        locals.var_q_disc__blk857_dn6 = assign36900_e41493_d_n6;
        locals.var_q_disc__blk857_dn7 = assign36900_e41493_d_n7;
        locals.var_q_disc__blk857_dn8 = assign36900_e41493_d_n8;
        locals.var_q_disc__blk857_dn9 = assign36900_e41493_d_n9;

        let (assign36910_e41503, assign36910_e41503_d_n4, assign36910_e41503_d_n6, assign36910_e41503_d_n7, assign36910_e41503_d_n8, assign36910_e41503_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36910_e41497: f64 = (locals.var_q_disc__blk857 - locals.var_q_b__blk855);
        let assign36910_e41500: f64 = (2.0 * locals.var_q_a__blk854);
        let assign36910_e41501: f64 = (assign36910_e41497 / assign36910_e41500);
        (assign36910_e41501, ((((locals.var_q_disc__blk857_dn4 - locals.var_q_b__blk855_dn4) * assign36910_e41500) - (assign36910_e41497 * (2.0 * locals.var_q_a__blk854_dn4))) / (assign36910_e41500 * assign36910_e41500)), ((((locals.var_q_disc__blk857_dn6 - locals.var_q_b__blk855_dn6) * assign36910_e41500) - (assign36910_e41497 * (2.0 * locals.var_q_a__blk854_dn6))) / (assign36910_e41500 * assign36910_e41500)), ((((locals.var_q_disc__blk857_dn7 - locals.var_q_b__blk855_dn7) * assign36910_e41500) - (assign36910_e41497 * (2.0 * locals.var_q_a__blk854_dn7))) / (assign36910_e41500 * assign36910_e41500)), ((((locals.var_q_disc__blk857_dn8 - locals.var_q_b__blk855_dn8) * assign36910_e41500) - (assign36910_e41497 * (2.0 * locals.var_q_a__blk854_dn8))) / (assign36910_e41500 * assign36910_e41500)), ((((locals.var_q_disc__blk857_dn9 - locals.var_q_b__blk855_dn9) * assign36910_e41500) - (assign36910_e41497 * (2.0 * locals.var_q_a__blk854_dn9))) / (assign36910_e41500 * assign36910_e41500)),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign36910_e41503;
        locals.var_q_qsq__blk825_dn4 = assign36910_e41503_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign36910_e41503_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign36910_e41503_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign36910_e41503_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign36910_e41503_d_n9;

        let (assign36920_e41511, assign36920_e41511_d_n4, assign36920_e41511_d_n6, assign36920_e41511_d_n7, assign36920_e41511_d_n8, assign36920_e41511_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36920_e41507: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign36920_e41509: f64 = (assign36920_e41507 - locals.var_q_qsq__blk825);
        (assign36920_e41509, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_qsq__blk825_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_qsq__blk825_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_qsq__blk825_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_qsq__blk825_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_qsq__blk825_dn9),)
    } else {
        (locals.var_q_delta__blk858, locals.var_q_delta__blk858_dn4, locals.var_q_delta__blk858_dn6, locals.var_q_delta__blk858_dn7, locals.var_q_delta__blk858_dn8, locals.var_q_delta__blk858_dn9,)
    }
};
        locals.var_q_delta__blk858 = assign36920_e41511;
        locals.var_q_delta__blk858_dn4 = assign36920_e41511_d_n4;
        locals.var_q_delta__blk858_dn6 = assign36920_e41511_d_n6;
        locals.var_q_delta__blk858_dn7 = assign36920_e41511_d_n7;
        locals.var_q_delta__blk858_dn8 = assign36920_e41511_d_n8;
        locals.var_q_delta__blk858_dn9 = assign36920_e41511_d_n9;

        let assign36930_e41514: f64 = if locals.var_q_delta__blk858 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign36930_e41514;

        let (assign36940_e41531, assign36940_e41531_d_n4, assign36940_e41531_d_n6, assign36940_e41531_d_n7, assign36940_e41531_d_n8, assign36940_e41531_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign36940_e41521: f64 = (locals.var_q_delta__blk858 / locals.var_a0__blk905);
        let assign36940_e41522: f64 = (assign36940_e41521).ln();
        let assign36940_e41524: f64 = (assign36940_e41522 + locals.var_xdeff__blk1000);
        let assign36940_e41526: f64 = (assign36940_e41524 - locals.var_xg1x__blk930);
        let assign36940_e41528: f64 = (assign36940_e41526 + locals.var_q1d__blk1001);
        let assign36940_e41529: f64 = (locals.var_q_delta__blk858 * assign36940_e41528);
        (assign36940_e41529, ((locals.var_q_delta__blk858_dn4 * assign36940_e41528) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn4 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36940_e41521) + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4))), ((locals.var_q_delta__blk858_dn6 * assign36940_e41528) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn6 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36940_e41521) + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6))), ((locals.var_q_delta__blk858_dn7 * assign36940_e41528) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn7 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36940_e41521) + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7))), ((locals.var_q_delta__blk858_dn8 * assign36940_e41528) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn8 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36940_e41521) + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8))), ((locals.var_q_delta__blk858_dn9 * assign36940_e41528) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn9 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign36940_e41521) + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9))),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign36940_e41531;
        locals.var_q_zero__blk849_dn4 = assign36940_e41531_d_n4;
        locals.var_q_zero__blk849_dn6 = assign36940_e41531_d_n6;
        locals.var_q_zero__blk849_dn7 = assign36940_e41531_d_n7;
        locals.var_q_zero__blk849_dn8 = assign36940_e41531_d_n8;
        locals.var_q_zero__blk849_dn9 = assign36940_e41531_d_n9;

    }

    pub(super) fn stamp_transient_block_99(
        locals: &mut StampLocals,
    ) {
        let (assign36950_e41543, assign36950_e41543_d_n4, assign36950_e41543_d_n6, assign36950_e41543_d_n7, assign36950_e41543_d_n8, assign36950_e41543_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign36950_e41537: f64 = (2.0 * locals.var_k1__blk932);
        let assign36950_e41539: f64 = (assign36950_e41537 * locals.var_q_k1q1__blk823);
        let assign36950_e41541: f64 = (assign36950_e41539 + locals.var_q_delta__blk858);
        (assign36950_e41541, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign36950_e41537 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_delta__blk858_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign36950_e41537 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_delta__blk858_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign36950_e41537 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_delta__blk858_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign36950_e41537 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_delta__blk858_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign36950_e41537 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_delta__blk858_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign36950_e41543;
        locals.var_q_d1_zero__blk850_dn4 = assign36950_e41543_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign36950_e41543_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign36950_e41543_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign36950_e41543_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign36950_e41543_d_n9;

        let (assign36960_e41553,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign36960_e41549: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign36960_e41551: f64 = (assign36960_e41549 - locals.var_q_x1sat__blk817);
        (assign36960_e41551,)
    } else {
        (locals.var_q_dx1__blk859,)
    }
};
        locals.var_q_dx1__blk859 = assign36960_e41553;

        let assign36970_e41563: f64 = (locals.var_q_dx1__blk859 + 2.3025850929941);
        let assign36970_e41565: f64 = (locals.var_k1__blk932).ln();
        let assign36970_e41566: f64 = (assign36970_e41563 + assign36970_e41565);
        let assign36970_e41573: f64 = if ((((locals.var_q_zero__blk849 < 0.0) && (locals.var_q_d1_zero__blk850 > 0.0)) && (assign36970_e41566 > 0.0)) || (locals.var_q_dx1__blk859 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign36970_e41573;

        let (assign36980_e41585, assign36980_e41585_d_n4, assign36980_e41585_d_n6, assign36980_e41585_d_n7, assign36980_e41585_d_n8, assign36980_e41585_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1176 != 0.0)) && (locals.var_guard1177 != 0.0)) {
        let assign36980_e41582: f64 = (locals.var_q_zero__blk849 / locals.var_q_d1_zero__blk850);
        let assign36980_e41583: f64 = (locals.var_q1d__blk1001 - assign36980_e41582);
        (assign36980_e41583, (locals.var_q1d__blk1001_dn4 - (((locals.var_q_zero__blk849_dn4 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn4)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn6 - (((locals.var_q_zero__blk849_dn6 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn6)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn7 - (((locals.var_q_zero__blk849_dn7 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn7)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn8 - (((locals.var_q_zero__blk849_dn8 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn8)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn9 - (((locals.var_q_zero__blk849_dn9 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn9)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign36980_e41585;
        locals.var_q1d__blk1001_dn4 = assign36980_e41585_d_n4;
        locals.var_q1d__blk1001_dn6 = assign36980_e41585_d_n6;
        locals.var_q1d__blk1001_dn7 = assign36980_e41585_d_n7;
        locals.var_q1d__blk1001_dn8 = assign36980_e41585_d_n8;
        locals.var_q1d__blk1001_dn9 = assign36980_e41585_d_n9;

        let (assign36990_e41591, assign36990_e41591_d_n4, assign36990_e41591_d_n6, assign36990_e41591_d_n7, assign36990_e41591_d_n8, assign36990_e41591_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign36990_e41589: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign36990_e41589, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign36990_e41591;
        locals.var_q_k1q1__blk823_dn4 = assign36990_e41591_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign36990_e41591_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign36990_e41591_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign36990_e41591_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign36990_e41591_d_n9;

        let (assign37000_e41597, assign37000_e41597_d_n4, assign37000_e41597_d_n6, assign37000_e41597_d_n7, assign37000_e41597_d_n8, assign37000_e41597_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37000_e41595: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
        (assign37000_e41595, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)),)
    } else {
        (locals.var_q_k2q2__blk853, locals.var_q_k2q2__blk853_dn4, locals.var_q_k2q2__blk853_dn6, locals.var_q_k2q2__blk853_dn7, locals.var_q_k2q2__blk853_dn8, locals.var_q_k2q2__blk853_dn9,)
    }
};
        locals.var_q_k2q2__blk853 = assign37000_e41597;
        locals.var_q_k2q2__blk853_dn4 = assign37000_e41597_d_n4;
        locals.var_q_k2q2__blk853_dn6 = assign37000_e41597_d_n6;
        locals.var_q_k2q2__blk853_dn7 = assign37000_e41597_d_n7;
        locals.var_q_k2q2__blk853_dn8 = assign37000_e41597_d_n8;
        locals.var_q_k2q2__blk853_dn9 = assign37000_e41597_d_n9;

        let (assign37010_e41603, assign37010_e41603_d_n4, assign37010_e41603_d_n6, assign37010_e41603_d_n7, assign37010_e41603_d_n8, assign37010_e41603_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37010_e41601: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_k2q2__blk853);
        (assign37010_e41601, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_k2q2__blk853_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_k2q2__blk853_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_k2q2__blk853_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_k2q2__blk853_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_k2q2__blk853_dn9),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign37010_e41603;
        locals.var_q_qi_int__blk846_dn4 = assign37010_e41603_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign37010_e41603_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign37010_e41603_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign37010_e41603_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign37010_e41603_d_n9;

        let (assign37020_e41611, assign37020_e41611_d_n4, assign37020_e41611_d_n6, assign37020_e41611_d_n7, assign37020_e41611_d_n8, assign37020_e41611_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37020_e41608: f64 = (0.065345483024 * locals.var_q_qi_int__blk846);
        let assign37020_e41609: f64 = (1.0 + assign37020_e41608);
        (assign37020_e41609, (0.065345483024 * locals.var_q_qi_int__blk846_dn4), (0.065345483024 * locals.var_q_qi_int__blk846_dn6), (0.065345483024 * locals.var_q_qi_int__blk846_dn7), (0.065345483024 * locals.var_q_qi_int__blk846_dn8), (0.065345483024 * locals.var_q_qi_int__blk846_dn9),)
    } else {
        (locals.var_q_a__blk854, locals.var_q_a__blk854_dn4, locals.var_q_a__blk854_dn6, locals.var_q_a__blk854_dn7, locals.var_q_a__blk854_dn8, locals.var_q_a__blk854_dn9,)
    }
};
        locals.var_q_a__blk854 = assign37020_e41611;
        locals.var_q_a__blk854_dn4 = assign37020_e41611_d_n4;
        locals.var_q_a__blk854_dn6 = assign37020_e41611_d_n6;
        locals.var_q_a__blk854_dn7 = assign37020_e41611_d_n7;
        locals.var_q_a__blk854_dn8 = assign37020_e41611_d_n8;
        locals.var_q_a__blk854_dn9 = assign37020_e41611_d_n9;

        let (assign37030_e41623, assign37030_e41623_d_n4, assign37030_e41623_d_n6, assign37030_e41623_d_n7, assign37030_e41623_d_n8, assign37030_e41623_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37030_e41616: f64 = (8.5797362674 * locals.var_q_qi_int__blk846);
        let assign37030_e41617: f64 = (39.478417604 + assign37030_e41616);
        let assign37030_e41620: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign37030_e41621: f64 = (assign37030_e41617 + assign37030_e41620);
        (assign37030_e41621, ((8.5797362674 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))),)
    } else {
        (locals.var_q_b__blk855, locals.var_q_b__blk855_dn4, locals.var_q_b__blk855_dn6, locals.var_q_b__blk855_dn7, locals.var_q_b__blk855_dn8, locals.var_q_b__blk855_dn9,)
    }
};
        locals.var_q_b__blk855 = assign37030_e41623;
        locals.var_q_b__blk855_dn4 = assign37030_e41623_d_n4;
        locals.var_q_b__blk855_dn6 = assign37030_e41623_d_n6;
        locals.var_q_b__blk855_dn7 = assign37030_e41623_d_n7;
        locals.var_q_b__blk855_dn8 = assign37030_e41623_d_n8;
        locals.var_q_b__blk855_dn9 = assign37030_e41623_d_n9;

        let (assign37040_e41635, assign37040_e41635_d_n4, assign37040_e41635_d_n6, assign37040_e41635_d_n7, assign37040_e41635_d_n8, assign37040_e41635_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37040_e41628: f64 = (2.0 * locals.var_q_qi_int__blk846);
        let assign37040_e41631: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign37040_e41632: f64 = (assign37040_e41628 + assign37040_e41631);
        let assign37040_e41633: f64 = (39.478417604 * assign37040_e41632);
        (assign37040_e41633, (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9)))),)
    } else {
        (locals.var_q_c__blk856, locals.var_q_c__blk856_dn4, locals.var_q_c__blk856_dn6, locals.var_q_c__blk856_dn7, locals.var_q_c__blk856_dn8, locals.var_q_c__blk856_dn9,)
    }
};
        locals.var_q_c__blk856 = assign37040_e41635;
        locals.var_q_c__blk856_dn4 = assign37040_e41635_d_n4;
        locals.var_q_c__blk856_dn6 = assign37040_e41635_d_n6;
        locals.var_q_c__blk856_dn7 = assign37040_e41635_d_n7;
        locals.var_q_c__blk856_dn8 = assign37040_e41635_d_n8;
        locals.var_q_c__blk856_dn9 = assign37040_e41635_d_n9;

        let (assign37050_e41648, assign37050_e41648_d_n4, assign37050_e41648_d_n6, assign37050_e41648_d_n7, assign37050_e41648_d_n8, assign37050_e41648_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37050_e41639: f64 = (locals.var_q_b__blk855 * locals.var_q_b__blk855);
        let assign37050_e41642: f64 = (4.0 * locals.var_q_a__blk854);
        let assign37050_e41644: f64 = (assign37050_e41642 * locals.var_q_c__blk856);
        let assign37050_e41645: f64 = (assign37050_e41639 - assign37050_e41644);
        let assign37050_e41646: f64 = (assign37050_e41645).sqrt();
        (assign37050_e41646, ((((locals.var_q_b__blk855_dn4 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn4)) - (((4.0 * locals.var_q_a__blk854_dn4) * locals.var_q_c__blk856) + (assign37050_e41642 * locals.var_q_c__blk856_dn4))) / (2.0 * assign37050_e41646)), ((((locals.var_q_b__blk855_dn6 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn6)) - (((4.0 * locals.var_q_a__blk854_dn6) * locals.var_q_c__blk856) + (assign37050_e41642 * locals.var_q_c__blk856_dn6))) / (2.0 * assign37050_e41646)), ((((locals.var_q_b__blk855_dn7 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn7)) - (((4.0 * locals.var_q_a__blk854_dn7) * locals.var_q_c__blk856) + (assign37050_e41642 * locals.var_q_c__blk856_dn7))) / (2.0 * assign37050_e41646)), ((((locals.var_q_b__blk855_dn8 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn8)) - (((4.0 * locals.var_q_a__blk854_dn8) * locals.var_q_c__blk856) + (assign37050_e41642 * locals.var_q_c__blk856_dn8))) / (2.0 * assign37050_e41646)), ((((locals.var_q_b__blk855_dn9 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn9)) - (((4.0 * locals.var_q_a__blk854_dn9) * locals.var_q_c__blk856) + (assign37050_e41642 * locals.var_q_c__blk856_dn9))) / (2.0 * assign37050_e41646)),)
    } else {
        (locals.var_q_disc__blk857, locals.var_q_disc__blk857_dn4, locals.var_q_disc__blk857_dn6, locals.var_q_disc__blk857_dn7, locals.var_q_disc__blk857_dn8, locals.var_q_disc__blk857_dn9,)
    }
};
        locals.var_q_disc__blk857 = assign37050_e41648;
        locals.var_q_disc__blk857_dn4 = assign37050_e41648_d_n4;
        locals.var_q_disc__blk857_dn6 = assign37050_e41648_d_n6;
        locals.var_q_disc__blk857_dn7 = assign37050_e41648_d_n7;
        locals.var_q_disc__blk857_dn8 = assign37050_e41648_d_n8;
        locals.var_q_disc__blk857_dn9 = assign37050_e41648_d_n9;

        let (assign37060_e41658, assign37060_e41658_d_n4, assign37060_e41658_d_n6, assign37060_e41658_d_n7, assign37060_e41658_d_n8, assign37060_e41658_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37060_e41652: f64 = (locals.var_q_disc__blk857 - locals.var_q_b__blk855);
        let assign37060_e41655: f64 = (2.0 * locals.var_q_a__blk854);
        let assign37060_e41656: f64 = (assign37060_e41652 / assign37060_e41655);
        (assign37060_e41656, ((((locals.var_q_disc__blk857_dn4 - locals.var_q_b__blk855_dn4) * assign37060_e41655) - (assign37060_e41652 * (2.0 * locals.var_q_a__blk854_dn4))) / (assign37060_e41655 * assign37060_e41655)), ((((locals.var_q_disc__blk857_dn6 - locals.var_q_b__blk855_dn6) * assign37060_e41655) - (assign37060_e41652 * (2.0 * locals.var_q_a__blk854_dn6))) / (assign37060_e41655 * assign37060_e41655)), ((((locals.var_q_disc__blk857_dn7 - locals.var_q_b__blk855_dn7) * assign37060_e41655) - (assign37060_e41652 * (2.0 * locals.var_q_a__blk854_dn7))) / (assign37060_e41655 * assign37060_e41655)), ((((locals.var_q_disc__blk857_dn8 - locals.var_q_b__blk855_dn8) * assign37060_e41655) - (assign37060_e41652 * (2.0 * locals.var_q_a__blk854_dn8))) / (assign37060_e41655 * assign37060_e41655)), ((((locals.var_q_disc__blk857_dn9 - locals.var_q_b__blk855_dn9) * assign37060_e41655) - (assign37060_e41652 * (2.0 * locals.var_q_a__blk854_dn9))) / (assign37060_e41655 * assign37060_e41655)),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign37060_e41658;
        locals.var_q_qsq__blk825_dn4 = assign37060_e41658_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign37060_e41658_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign37060_e41658_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign37060_e41658_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign37060_e41658_d_n9;

        let assign37070_e41661: f64 = (-0.005);
        let assign37070_e41662: f64 = if locals.var_q_qsq__blk825 < assign37070_e41661 { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign37070_e41662;

        let (assign37080_e41670, assign37080_e41670_d_n4, assign37080_e41670_d_n6, assign37080_e41670_d_n7, assign37080_e41670_d_n8, assign37080_e41670_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1178 != 0.0)) {
        let assign37080_e41667: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37080_e41668: f64 = (assign37080_e41667).sqrt();
        (assign37080_e41668, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37080_e41668)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37080_e41668)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37080_e41668)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37080_e41668)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37080_e41668)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37080_e41670;
        locals.var_q_rac_qsq__blk828_dn4 = assign37080_e41670_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37080_e41670_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37080_e41670_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37080_e41670_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37080_e41670_d_n9;

        let (assign37090_e41681, assign37090_e41681_d_n4, assign37090_e41681_d_n6, assign37090_e41681_d_n7, assign37090_e41681_d_n8, assign37090_e41681_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1178 != 0.0)) {
        let assign37090_e41677: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign37090_e41678: f64 = (assign37090_e41677).tan();
        let assign37090_e41679: f64 = (locals.var_q_rac_qsq__blk828 / assign37090_e41678);
        (assign37090_e41679, (((locals.var_q_rac_qsq__blk828_dn4 * assign37090_e41678) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign37090_e41677).cos() * (assign37090_e41677).cos())))) / (assign37090_e41678 * assign37090_e41678)), (((locals.var_q_rac_qsq__blk828_dn6 * assign37090_e41678) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign37090_e41677).cos() * (assign37090_e41677).cos())))) / (assign37090_e41678 * assign37090_e41678)), (((locals.var_q_rac_qsq__blk828_dn7 * assign37090_e41678) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign37090_e41677).cos() * (assign37090_e41677).cos())))) / (assign37090_e41678 * assign37090_e41678)), (((locals.var_q_rac_qsq__blk828_dn8 * assign37090_e41678) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign37090_e41677).cos() * (assign37090_e41677).cos())))) / (assign37090_e41678 * assign37090_e41678)), (((locals.var_q_rac_qsq__blk828_dn9 * assign37090_e41678) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign37090_e41677).cos() * (assign37090_e41677).cos())))) / (assign37090_e41678 * assign37090_e41678)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37090_e41681;
        locals.var_q_qcoth__blk829_dn4 = assign37090_e41681_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37090_e41681_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37090_e41681_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37090_e41681_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37090_e41681_d_n9;

        let (assign37100_e41697, assign37100_e41697_d_n4, assign37100_e41697_d_n6, assign37100_e41697_d_n7, assign37100_e41697_d_n8, assign37100_e41697_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1178 != 0.0)) {
        let assign37100_e41690: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37100_e41691: f64 = (locals.var_q_qcoth__blk829 * assign37100_e41690);
        let assign37100_e41692: f64 = (locals.var_q_qsq__blk825 + assign37100_e41691);
        let assign37100_e41693: f64 = (0.25 * assign37100_e41692);
        let assign37100_e41695: f64 = (assign37100_e41693 / locals.var_q_qsq__blk825);
        (assign37100_e41695, ((((0.25 * (locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37100_e41690) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4))))) * locals.var_q_qsq__blk825) - (assign37100_e41693 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37100_e41690) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6))))) * locals.var_q_qsq__blk825) - (assign37100_e41693 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37100_e41690) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7))))) * locals.var_q_qsq__blk825) - (assign37100_e41693 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37100_e41690) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8))))) * locals.var_q_qsq__blk825) - (assign37100_e41693 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37100_e41690) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9))))) * locals.var_q_qsq__blk825) - (assign37100_e41693 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37100_e41697;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37100_e41697_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37100_e41697_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37100_e41697_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37100_e41697_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37100_e41697_d_n9;

        let assign37110_e41700: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1179 = assign37110_e41700;

        let (assign37120_e41711, assign37120_e41711_d_n4, assign37120_e41711_d_n6, assign37120_e41711_d_n7, assign37120_e41711_d_n8, assign37120_e41711_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37120_e41708: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37120_e41709: f64 = (assign37120_e41708).sqrt();
        (assign37120_e41709, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37120_e41709)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37120_e41709)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37120_e41709)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37120_e41709)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37120_e41709)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37120_e41711;
        locals.var_q_rac_qsq__blk828_dn4 = assign37120_e41711_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37120_e41711_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37120_e41711_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37120_e41711_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37120_e41711_d_n9;

        let (assign37130_e41722, assign37130_e41722_d_n4, assign37130_e41722_d_n6, assign37130_e41722_d_n7, assign37130_e41722_d_n8, assign37130_e41722_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37130_e41719: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign37130_e41720: f64 = (assign37130_e41719).exp();
        (assign37130_e41720, (assign37130_e41720 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign37130_e41720 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign37130_e41720 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign37130_e41720 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign37130_e41720 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign37130_e41722;
        locals.var_q_invexpq__blk831_dn4 = assign37130_e41722_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign37130_e41722_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign37130_e41722_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign37130_e41722_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign37130_e41722_d_n9;

        let (assign37140_e41739, assign37140_e41739_d_n4, assign37140_e41739_d_n6, assign37140_e41739_d_n7, assign37140_e41739_d_n8, assign37140_e41739_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37140_e41732: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign37140_e41733: f64 = (locals.var_q_rac_qsq__blk828 * assign37140_e41732);
        let assign37140_e41736: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign37140_e41737: f64 = (assign37140_e41733 / assign37140_e41736);
        (assign37140_e41737, (((((locals.var_q_rac_qsq__blk828_dn4 * assign37140_e41732) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign37140_e41736) - (assign37140_e41733 * (-locals.var_q_invexpq__blk831_dn4))) / (assign37140_e41736 * assign37140_e41736)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign37140_e41732) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign37140_e41736) - (assign37140_e41733 * (-locals.var_q_invexpq__blk831_dn6))) / (assign37140_e41736 * assign37140_e41736)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign37140_e41732) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign37140_e41736) - (assign37140_e41733 * (-locals.var_q_invexpq__blk831_dn7))) / (assign37140_e41736 * assign37140_e41736)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign37140_e41732) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign37140_e41736) - (assign37140_e41733 * (-locals.var_q_invexpq__blk831_dn8))) / (assign37140_e41736 * assign37140_e41736)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign37140_e41732) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign37140_e41736) - (assign37140_e41733 * (-locals.var_q_invexpq__blk831_dn9))) / (assign37140_e41736 * assign37140_e41736)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37140_e41739;
        locals.var_q_qcoth__blk829_dn4 = assign37140_e41739_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37140_e41739_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37140_e41739_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37140_e41739_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37140_e41739_d_n9;

        let (assign37150_e41758, assign37150_e41758_d_n4, assign37150_e41758_d_n6, assign37150_e41758_d_n7, assign37150_e41758_d_n8, assign37150_e41758_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 != 0.0)) {
        let assign37150_e41751: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37150_e41752: f64 = (locals.var_q_qcoth__blk829 * assign37150_e41751);
        let assign37150_e41753: f64 = (locals.var_q_qsq__blk825 + assign37150_e41752);
        let assign37150_e41754: f64 = (0.25 * assign37150_e41753);
        let assign37150_e41756: f64 = (assign37150_e41754 / locals.var_q_qsq__blk825);
        (assign37150_e41756, ((((0.25 * (locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37150_e41751) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4))))) * locals.var_q_qsq__blk825) - (assign37150_e41754 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37150_e41751) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6))))) * locals.var_q_qsq__blk825) - (assign37150_e41754 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37150_e41751) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7))))) * locals.var_q_qsq__blk825) - (assign37150_e41754 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37150_e41751) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8))))) * locals.var_q_qsq__blk825) - (assign37150_e41754 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37150_e41751) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9))))) * locals.var_q_qsq__blk825) - (assign37150_e41754 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37150_e41758;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37150_e41758_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37150_e41758_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37150_e41758_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37150_e41758_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37150_e41758_d_n9;

        let (assign37160_e41784, assign37160_e41784_d_n4, assign37160_e41784_d_n6, assign37160_e41784_d_n7, assign37160_e41784_d_n8, assign37160_e41784_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 == 0.0)) {
        let assign37160_e41769: f64 = (locals.var_q_qsq__blk825 * 0.1666666666667);
        let assign37160_e41773: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign37160_e41777: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign37160_e41778: f64 = (1.0 - assign37160_e41777);
        let assign37160_e41779: f64 = (assign37160_e41773 * assign37160_e41778);
        let assign37160_e41780: f64 = (1.0 - assign37160_e41779);
        let assign37160_e41781: f64 = (assign37160_e41769 * assign37160_e41780);
        let assign37160_e41782: f64 = (2.0 + assign37160_e41781);
        (assign37160_e41782, (((locals.var_q_qsq__blk825_dn4 * 0.1666666666667) * assign37160_e41780) + (assign37160_e41769 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign37160_e41778) + (assign37160_e41773 * (-(locals.var_q_qsq__blk825_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn6 * 0.1666666666667) * assign37160_e41780) + (assign37160_e41769 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign37160_e41778) + (assign37160_e41773 * (-(locals.var_q_qsq__blk825_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn7 * 0.1666666666667) * assign37160_e41780) + (assign37160_e41769 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign37160_e41778) + (assign37160_e41773 * (-(locals.var_q_qsq__blk825_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn8 * 0.1666666666667) * assign37160_e41780) + (assign37160_e41769 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign37160_e41778) + (assign37160_e41773 * (-(locals.var_q_qsq__blk825_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn9 * 0.1666666666667) * assign37160_e41780) + (assign37160_e41769 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign37160_e41778) + (assign37160_e41773 * (-(locals.var_q_qsq__blk825_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37160_e41784;
        locals.var_q_qcoth__blk829_dn4 = assign37160_e41784_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37160_e41784_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37160_e41784_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37160_e41784_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37160_e41784_d_n9;

        let (assign37170_e41812, assign37170_e41812_d_n4, assign37170_e41812_d_n6, assign37170_e41812_d_n7, assign37170_e41812_d_n8, assign37170_e41812_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1178 == 0.0)) && (locals.var_guard1179 == 0.0)) {
        let assign37170_e41796: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37170_e41800: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign37170_e41804: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37170_e41805: f64 = (1.0 - assign37170_e41804);
        let assign37170_e41806: f64 = (assign37170_e41800 * assign37170_e41805);
        let assign37170_e41807: f64 = (1.0 - assign37170_e41806);
        let assign37170_e41808: f64 = (assign37170_e41796 * assign37170_e41807);
        let assign37170_e41809: f64 = (1.0 - assign37170_e41808);
        let assign37170_e41810: f64 = (0.1666666666667 * assign37170_e41809);
        (assign37170_e41810, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign37170_e41807) + (assign37170_e41796 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign37170_e41805) + (assign37170_e41800 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign37170_e41807) + (assign37170_e41796 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign37170_e41805) + (assign37170_e41800 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign37170_e41807) + (assign37170_e41796 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign37170_e41805) + (assign37170_e41800 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign37170_e41807) + (assign37170_e41796 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign37170_e41805) + (assign37170_e41800 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign37170_e41807) + (assign37170_e41796 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign37170_e41805) + (assign37170_e41800 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37170_e41812;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37170_e41812_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37170_e41812_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37170_e41812_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37170_e41812_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37170_e41812_d_n9;

        let (assign37180_e41832, assign37180_e41832_d_n4, assign37180_e41832_d_n6, assign37180_e41832_d_n7, assign37180_e41832_d_n8, assign37180_e41832_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37180_e41817: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829);
        let assign37180_e41820: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign37180_e41821: f64 = (assign37180_e41817 + assign37180_e41820);
        let assign37180_e41823: f64 = (assign37180_e41821 + locals.var_q_qsq__blk825);
        let assign37180_e41826: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830);
        let assign37180_e41828: f64 = (assign37180_e41826 + 1.0);
        let assign37180_e41829: f64 = (assign37180_e41823 / assign37180_e41828);
        let assign37180_e41830: f64 = (locals.var_q_qsq__blk825 - assign37180_e41829);
        (assign37180_e41830, (locals.var_q_qsq__blk825_dn4 - (((((((locals.var_q_qi_int__blk846_dn4 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn4)) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))) + locals.var_q_qsq__blk825_dn4) * assign37180_e41828) - (assign37180_e41823 * ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn4)))) / (assign37180_e41828 * assign37180_e41828))), (locals.var_q_qsq__blk825_dn6 - (((((((locals.var_q_qi_int__blk846_dn6 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn6)) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))) + locals.var_q_qsq__blk825_dn6) * assign37180_e41828) - (assign37180_e41823 * ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn6)))) / (assign37180_e41828 * assign37180_e41828))), (locals.var_q_qsq__blk825_dn7 - (((((((locals.var_q_qi_int__blk846_dn7 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn7)) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))) + locals.var_q_qsq__blk825_dn7) * assign37180_e41828) - (assign37180_e41823 * ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn7)))) / (assign37180_e41828 * assign37180_e41828))), (locals.var_q_qsq__blk825_dn8 - (((((((locals.var_q_qi_int__blk846_dn8 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn8)) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))) + locals.var_q_qsq__blk825_dn8) * assign37180_e41828) - (assign37180_e41823 * ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn8)))) / (assign37180_e41828 * assign37180_e41828))), (locals.var_q_qsq__blk825_dn9 - (((((((locals.var_q_qi_int__blk846_dn9 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn9)) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))) + locals.var_q_qsq__blk825_dn9) * assign37180_e41828) - (assign37180_e41823 * ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn9)))) / (assign37180_e41828 * assign37180_e41828))),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign37180_e41832;
        locals.var_q_qsq__blk825_dn4 = assign37180_e41832_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign37180_e41832_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign37180_e41832_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign37180_e41832_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign37180_e41832_d_n9;

        let (assign37190_e41840, assign37190_e41840_d_n4, assign37190_e41840_d_n6, assign37190_e41840_d_n7, assign37190_e41840_d_n8, assign37190_e41840_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37190_e41836: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign37190_e41838: f64 = (assign37190_e41836 - locals.var_q_qsq__blk825);
        (assign37190_e41838, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_qsq__blk825_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_qsq__blk825_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_qsq__blk825_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_qsq__blk825_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_qsq__blk825_dn9),)
    } else {
        (locals.var_q_delta__blk858, locals.var_q_delta__blk858_dn4, locals.var_q_delta__blk858_dn6, locals.var_q_delta__blk858_dn7, locals.var_q_delta__blk858_dn8, locals.var_q_delta__blk858_dn9,)
    }
};
        locals.var_q_delta__blk858 = assign37190_e41840;
        locals.var_q_delta__blk858_dn4 = assign37190_e41840_d_n4;
        locals.var_q_delta__blk858_dn6 = assign37190_e41840_d_n6;
        locals.var_q_delta__blk858_dn7 = assign37190_e41840_d_n7;
        locals.var_q_delta__blk858_dn8 = assign37190_e41840_d_n8;
        locals.var_q_delta__blk858_dn9 = assign37190_e41840_d_n9;

        let assign37200_e41843: f64 = if locals.var_q_delta__blk858 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1180 = assign37200_e41843;

        let (assign37210_e41860, assign37210_e41860_d_n4, assign37210_e41860_d_n6, assign37210_e41860_d_n7, assign37210_e41860_d_n8, assign37210_e41860_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign37210_e41850: f64 = (locals.var_q_delta__blk858 / locals.var_a0__blk905);
        let assign37210_e41851: f64 = (assign37210_e41850).ln();
        let assign37210_e41853: f64 = (assign37210_e41851 + locals.var_xdeff__blk1000);
        let assign37210_e41855: f64 = (assign37210_e41853 - locals.var_xg1x__blk930);
        let assign37210_e41857: f64 = (assign37210_e41855 + locals.var_q1d__blk1001);
        let assign37210_e41858: f64 = (locals.var_q_delta__blk858 * assign37210_e41857);
        (assign37210_e41858, ((locals.var_q_delta__blk858_dn4 * assign37210_e41857) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn4 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37210_e41850) + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4))), ((locals.var_q_delta__blk858_dn6 * assign37210_e41857) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn6 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37210_e41850) + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6))), ((locals.var_q_delta__blk858_dn7 * assign37210_e41857) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn7 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37210_e41850) + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7))), ((locals.var_q_delta__blk858_dn8 * assign37210_e41857) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn8 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37210_e41850) + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8))), ((locals.var_q_delta__blk858_dn9 * assign37210_e41857) + (locals.var_q_delta__blk858 * (((((((locals.var_q_delta__blk858_dn9 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign37210_e41850) + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9))),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign37210_e41860;
        locals.var_q_zero__blk849_dn4 = assign37210_e41860_d_n4;
        locals.var_q_zero__blk849_dn6 = assign37210_e41860_d_n6;
        locals.var_q_zero__blk849_dn7 = assign37210_e41860_d_n7;
        locals.var_q_zero__blk849_dn8 = assign37210_e41860_d_n8;
        locals.var_q_zero__blk849_dn9 = assign37210_e41860_d_n9;

        let (assign37220_e41872, assign37220_e41872_d_n4, assign37220_e41872_d_n6, assign37220_e41872_d_n7, assign37220_e41872_d_n8, assign37220_e41872_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign37220_e41866: f64 = (2.0 * locals.var_k1__blk932);
        let assign37220_e41868: f64 = (assign37220_e41866 * locals.var_q_k1q1__blk823);
        let assign37220_e41870: f64 = (assign37220_e41868 + locals.var_q_delta__blk858);
        (assign37220_e41870, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign37220_e41866 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_delta__blk858_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign37220_e41866 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_delta__blk858_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign37220_e41866 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_delta__blk858_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign37220_e41866 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_delta__blk858_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign37220_e41866 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_delta__blk858_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign37220_e41872;
        locals.var_q_d1_zero__blk850_dn4 = assign37220_e41872_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign37220_e41872_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign37220_e41872_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign37220_e41872_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign37220_e41872_d_n9;

        let (assign37230_e41882,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign37230_e41878: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37230_e41880: f64 = (assign37230_e41878 - locals.var_q_x1sat__blk817);
        (assign37230_e41880,)
    } else {
        (locals.var_q_dx1__blk859,)
    }
};
        locals.var_q_dx1__blk859 = assign37230_e41882;

        let assign37240_e41892: f64 = (locals.var_q_dx1__blk859 + 2.3025850929941);
        let assign37240_e41894: f64 = (locals.var_k1__blk932).ln();
        let assign37240_e41895: f64 = (assign37240_e41892 + assign37240_e41894);
        let assign37240_e41902: f64 = if ((((locals.var_q_zero__blk849 < 0.0) && (locals.var_q_d1_zero__blk850 > 0.0)) && (assign37240_e41895 > 0.0)) || (locals.var_q_dx1__blk859 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1181 = assign37240_e41902;

        let (assign37250_e41914, assign37250_e41914_d_n4, assign37250_e41914_d_n6, assign37250_e41914_d_n7, assign37250_e41914_d_n8, assign37250_e41914_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1180 != 0.0)) && (locals.var_guard1181 != 0.0)) {
        let assign37250_e41911: f64 = (locals.var_q_zero__blk849 / locals.var_q_d1_zero__blk850);
        let assign37250_e41912: f64 = (locals.var_q1d__blk1001 - assign37250_e41911);
        (assign37250_e41912, (locals.var_q1d__blk1001_dn4 - (((locals.var_q_zero__blk849_dn4 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn4)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn6 - (((locals.var_q_zero__blk849_dn6 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn6)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn7 - (((locals.var_q_zero__blk849_dn7 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn7)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn8 - (((locals.var_q_zero__blk849_dn8 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn8)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1d__blk1001_dn9 - (((locals.var_q_zero__blk849_dn9 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn9)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign37250_e41914;
        locals.var_q1d__blk1001_dn4 = assign37250_e41914_d_n4;
        locals.var_q1d__blk1001_dn6 = assign37250_e41914_d_n6;
        locals.var_q1d__blk1001_dn7 = assign37250_e41914_d_n7;
        locals.var_q1d__blk1001_dn8 = assign37250_e41914_d_n8;
        locals.var_q1d__blk1001_dn9 = assign37250_e41914_d_n9;

        let (assign37260_e41920, assign37260_e41920_d_n4, assign37260_e41920_d_n6, assign37260_e41920_d_n7, assign37260_e41920_d_n8, assign37260_e41920_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37260_e41918: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign37260_e41918, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign37260_e41920;
        locals.var_q_k1q1__blk823_dn4 = assign37260_e41920_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign37260_e41920_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign37260_e41920_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign37260_e41920_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign37260_e41920_d_n9;

        let assign37270_e41923: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37270_e41925: f64 = (assign37270_e41923 - locals.var_xdeff__blk1000);
        let assign37270_e41927: f64 = if assign37270_e41925 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1182 = assign37270_e41927;

        let (assign37280_e41938, assign37280_e41938_d_n4, assign37280_e41938_d_n6, assign37280_e41938_d_n7, assign37280_e41938_d_n8, assign37280_e41938_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1182 != 0.0)) {
        let assign37280_e41933: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37280_e41935: f64 = (assign37280_e41933 - locals.var_xdeff__blk1000);
        let assign37280_e41936: f64 = (assign37280_e41935).exp();
        (assign37280_e41936, (assign37280_e41936 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign37280_e41936 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign37280_e41936 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign37280_e41936 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign37280_e41936 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37280_e41938;
        locals.var_q_temp1__blk814_dn4 = assign37280_e41938_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37280_e41938_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37280_e41938_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37280_e41938_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37280_e41938_d_n9;

    }

    pub(super) fn stamp_transient_block_100(
        locals: &mut StampLocals,
    ) {
        let (assign37290_e41979, assign37290_e41979_d_n4, assign37290_e41979_d_n6, assign37290_e41979_d_n7, assign37290_e41979_d_n8, assign37290_e41979_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1182 == 0.0)) {
        let assign37290_e41947: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37290_e41949: f64 = (assign37290_e41947 - locals.var_xdeff__blk1000);
        let assign37290_e41951: f64 = (assign37290_e41949 - 80.0);
        let assign37290_e41956: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37290_e41958: f64 = (assign37290_e41956 - locals.var_xdeff__blk1000);
        let assign37290_e41960: f64 = (assign37290_e41958 - 80.0);
        let assign37290_e41961: f64 = (0.5 * assign37290_e41960);
        let assign37290_e41965: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign37290_e41967: f64 = (assign37290_e41965 - locals.var_xdeff__blk1000);
        let assign37290_e41969: f64 = (assign37290_e41967 - 80.0);
        let assign37290_e41971: f64 = (assign37290_e41969 * 0.3333333333333);
        let assign37290_e41972: f64 = (1.0 + assign37290_e41971);
        let assign37290_e41973: f64 = (assign37290_e41961 * assign37290_e41972);
        let assign37290_e41974: f64 = (1.0 + assign37290_e41973);
        let assign37290_e41975: f64 = (assign37290_e41951 * assign37290_e41974);
        let assign37290_e41976: f64 = (1.0 + assign37290_e41975);
        let assign37290_e41977: f64 = (5.54062e34 * assign37290_e41976);
        (assign37290_e41977, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign37290_e41974) + (assign37290_e41951 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign37290_e41972) + (assign37290_e41961 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign37290_e41974) + (assign37290_e41951 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign37290_e41972) + (assign37290_e41961 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign37290_e41974) + (assign37290_e41951 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign37290_e41972) + (assign37290_e41961 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign37290_e41974) + (assign37290_e41951 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign37290_e41972) + (assign37290_e41961 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign37290_e41974) + (assign37290_e41951 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign37290_e41972) + (assign37290_e41961 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37290_e41979;
        locals.var_q_temp1__blk814_dn4 = assign37290_e41979_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37290_e41979_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37290_e41979_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37290_e41979_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37290_e41979_d_n9;

        let (assign37300_e41985, assign37300_e41985_d_n4, assign37300_e41985_d_n6, assign37300_e41985_d_n7, assign37300_e41985_d_n8, assign37300_e41985_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37300_e41983: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign37300_e41983, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign37300_e41985;
        locals.var_q_aexp__blk824_dn4 = assign37300_e41985_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign37300_e41985_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign37300_e41985_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign37300_e41985_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign37300_e41985_d_n9;

        let (assign37310_e41993, assign37310_e41993_d_n4, assign37310_e41993_d_n6, assign37310_e41993_d_n7, assign37310_e41993_d_n8, assign37310_e41993_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37310_e41989: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign37310_e41991: f64 = (assign37310_e41989 - locals.var_q_aexp__blk824);
        (assign37310_e41991, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign37310_e41993;
        locals.var_q_qsq__blk825_dn4 = assign37310_e41993_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign37310_e41993_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign37310_e41993_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign37310_e41993_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign37310_e41993_d_n9;

        let (assign37320_e42003, assign37320_e42003_d_n4, assign37320_e42003_d_n6, assign37320_e42003_d_n7, assign37320_e42003_d_n8, assign37320_e42003_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37320_e41997: f64 = (2.0 * locals.var_k1__blk932);
        let assign37320_e41999: f64 = (assign37320_e41997 * locals.var_q_k1q1__blk823);
        let assign37320_e42001: f64 = (assign37320_e41999 + locals.var_q_aexp__blk824);
        (assign37320_e42001, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign37320_e41997 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign37320_e41997 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign37320_e41997 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign37320_e41997 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign37320_e41997 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign37320_e42003;
        locals.var_q_d1_qsq__blk826_dn4 = assign37320_e42003_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign37320_e42003_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign37320_e42003_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign37320_e42003_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign37320_e42003_d_n9;

        let (assign37330_e42013, assign37330_e42013_d_n4, assign37330_e42013_d_n6, assign37330_e42013_d_n7, assign37330_e42013_d_n8, assign37330_e42013_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37330_e42007: f64 = (2.0 * locals.var_k1__blk932);
        let assign37330_e42009: f64 = (assign37330_e42007 * locals.var_k1__blk932);
        let assign37330_e42011: f64 = (assign37330_e42009 - locals.var_q_aexp__blk824);
        (assign37330_e42011, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign37330_e42007 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign37330_e42007 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign37330_e42007 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign37330_e42007 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign37330_e42007 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign37330_e42013;
        locals.var_q_d2_qsq__blk827_dn4 = assign37330_e42013_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign37330_e42013_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign37330_e42013_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign37330_e42013_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign37330_e42013_d_n9;

        let assign37340_e42016: f64 = (-0.005);
        let assign37340_e42017: f64 = if locals.var_q_qsq__blk825 < assign37340_e42016 { 1.0 } else { 0.0 };
        locals.var_guard1183 = assign37340_e42017;

        let (assign37350_e42025, assign37350_e42025_d_n4, assign37350_e42025_d_n6, assign37350_e42025_d_n7, assign37350_e42025_d_n8, assign37350_e42025_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37350_e42022: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37350_e42023: f64 = (assign37350_e42022).sqrt();
        (assign37350_e42023, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37350_e42023)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37350_e42023)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37350_e42023)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37350_e42023)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37350_e42023)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37350_e42025;
        locals.var_q_rac_qsq__blk828_dn4 = assign37350_e42025_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37350_e42025_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37350_e42025_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37350_e42025_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37350_e42025_d_n9;

        let (assign37360_e42036, assign37360_e42036_d_n4, assign37360_e42036_d_n6, assign37360_e42036_d_n7, assign37360_e42036_d_n8, assign37360_e42036_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37360_e42032: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign37360_e42033: f64 = (assign37360_e42032).tan();
        let assign37360_e42034: f64 = (locals.var_q_rac_qsq__blk828 / assign37360_e42033);
        (assign37360_e42034, (((locals.var_q_rac_qsq__blk828_dn4 * assign37360_e42033) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign37360_e42032).cos() * (assign37360_e42032).cos())))) / (assign37360_e42033 * assign37360_e42033)), (((locals.var_q_rac_qsq__blk828_dn6 * assign37360_e42033) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign37360_e42032).cos() * (assign37360_e42032).cos())))) / (assign37360_e42033 * assign37360_e42033)), (((locals.var_q_rac_qsq__blk828_dn7 * assign37360_e42033) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign37360_e42032).cos() * (assign37360_e42032).cos())))) / (assign37360_e42033 * assign37360_e42033)), (((locals.var_q_rac_qsq__blk828_dn8 * assign37360_e42033) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign37360_e42032).cos() * (assign37360_e42032).cos())))) / (assign37360_e42033 * assign37360_e42033)), (((locals.var_q_rac_qsq__blk828_dn9 * assign37360_e42033) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign37360_e42032).cos() * (assign37360_e42032).cos())))) / (assign37360_e42033 * assign37360_e42033)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37360_e42036;
        locals.var_q_qcoth__blk829_dn4 = assign37360_e42036_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37360_e42036_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37360_e42036_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37360_e42036_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37360_e42036_d_n9;

        let (assign37370_e42046, assign37370_e42046_d_n4, assign37370_e42046_d_n6, assign37370_e42046_d_n7, assign37370_e42046_d_n8, assign37370_e42046_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37370_e42042: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign37370_e42044: f64 = (assign37370_e42042 / locals.var_q_qsq__blk825);
        (assign37370_e42044, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign37370_e42042 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign37370_e42042 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign37370_e42042 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign37370_e42042 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign37370_e42042 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37370_e42046;
        locals.var_q_temp1__blk814_dn4 = assign37370_e42046_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37370_e42046_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37370_e42046_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37370_e42046_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37370_e42046_d_n9;

        let (assign37380_e42060, assign37380_e42060_d_n4, assign37380_e42060_d_n6, assign37380_e42060_d_n7, assign37380_e42060_d_n8, assign37380_e42060_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37380_e42054: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37380_e42055: f64 = (locals.var_q_qcoth__blk829 * assign37380_e42054);
        let assign37380_e42056: f64 = (locals.var_q_qsq__blk825 + assign37380_e42055);
        let assign37380_e42058: f64 = (assign37380_e42056 * locals.var_q_temp1__blk814);
        (assign37380_e42058, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37380_e42054) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign37380_e42056 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37380_e42054) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign37380_e42056 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37380_e42054) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign37380_e42056 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37380_e42054) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign37380_e42056 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37380_e42054) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign37380_e42056 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37380_e42060;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37380_e42060_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37380_e42060_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37380_e42060_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37380_e42060_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37380_e42060_d_n9;

        let (assign37390_e42082, assign37390_e42082_d_n4, assign37390_e42082_d_n6, assign37390_e42082_d_n7, assign37390_e42082_d_n8, assign37390_e42082_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37390_e42067: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign37390_e42070: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign37390_e42071: f64 = (assign37390_e42067 * assign37390_e42070);
        let assign37390_e42072: f64 = (locals.var_q_d1_qsq__blk826 - assign37390_e42071);
        let assign37390_e42074: f64 = (assign37390_e42072 * locals.var_q_temp1__blk814);
        let assign37390_e42077: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign37390_e42079: f64 = (assign37390_e42077 / locals.var_q_d1_qsq__blk826);
        let assign37390_e42080: f64 = (assign37390_e42074 + assign37390_e42079);
        (assign37390_e42080, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign37390_e42070) + (assign37390_e42067 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign37390_e42072 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42077 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign37390_e42070) + (assign37390_e42067 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign37390_e42072 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42077 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign37390_e42070) + (assign37390_e42067 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign37390_e42072 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42077 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign37390_e42070) + (assign37390_e42067 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign37390_e42072 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42077 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign37390_e42070) + (assign37390_e42067 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign37390_e42072 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign37390_e42077 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign37390_e42082;
        locals.var_q_d2_qcoth__blk832_dn4 = assign37390_e42082_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign37390_e42082_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign37390_e42082_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign37390_e42082_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign37390_e42082_d_n9;

        let (assign37400_e42092, assign37400_e42092_d_n4, assign37400_e42092_d_n6, assign37400_e42092_d_n7, assign37400_e42092_d_n8, assign37400_e42092_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37400_e42089: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign37400_e42090: f64 = (1.0 - assign37400_e42089);
        (assign37400_e42090, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37400_e42092;
        locals.var_q_temp2__blk815_dn4 = assign37400_e42092_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37400_e42092_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37400_e42092_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37400_e42092_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37400_e42092_d_n9;

        let (assign37410_e42102, assign37410_e42102_d_n4, assign37410_e42102_d_n6, assign37410_e42102_d_n7, assign37410_e42102_d_n8, assign37410_e42102_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37410_e42098: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign37410_e42100: f64 = (assign37410_e42098 * locals.var_q_temp2__blk815);
        (assign37410_e42100, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42098 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42098 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42098 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42098 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37410_e42098 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign37410_e42102;
        locals.var_q_d1_ln__blk835_dn4 = assign37410_e42102_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign37410_e42102_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign37410_e42102_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign37410_e42102_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign37410_e42102_d_n9;

        let (assign37420_e42120, assign37420_e42120_d_n4, assign37420_e42120_d_n6, assign37420_e42120_d_n7, assign37420_e42120_d_n8, assign37420_e42120_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1183 != 0.0)) {
        let assign37420_e42108: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign37420_e42113: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign37420_e42114: f64 = (locals.var_q_d1_ln__blk835 + assign37420_e42113);
        let assign37420_e42115: f64 = (locals.var_q_d1_qsq__blk826 * assign37420_e42114);
        let assign37420_e42116: f64 = (assign37420_e42108 - assign37420_e42115);
        let assign37420_e42118: f64 = (assign37420_e42116 / locals.var_q_qsq__blk825);
        (assign37420_e42118, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign37420_e42114) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign37420_e42116 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign37420_e42114) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign37420_e42116 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign37420_e42114) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign37420_e42116 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign37420_e42114) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign37420_e42116 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign37420_e42114) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign37420_e42116 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign37420_e42120;
        locals.var_q_d2_ln__blk836_dn4 = assign37420_e42120_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign37420_e42120_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign37420_e42120_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign37420_e42120_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign37420_e42120_d_n9;

        let assign37430_e42123: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1184 = assign37430_e42123;

        let (assign37440_e42134, assign37440_e42134_d_n4, assign37440_e42134_d_n6, assign37440_e42134_d_n7, assign37440_e42134_d_n8, assign37440_e42134_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37440_e42131: f64 = (locals.var_q_qsq__blk825).abs();
        let assign37440_e42132: f64 = (assign37440_e42131).sqrt();
        (assign37440_e42132, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign37440_e42132)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign37440_e42132)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign37440_e42132)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign37440_e42132)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign37440_e42132)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign37440_e42134;
        locals.var_q_rac_qsq__blk828_dn4 = assign37440_e42134_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign37440_e42134_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign37440_e42134_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign37440_e42134_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign37440_e42134_d_n9;

        let (assign37450_e42145, assign37450_e42145_d_n4, assign37450_e42145_d_n6, assign37450_e42145_d_n7, assign37450_e42145_d_n8, assign37450_e42145_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37450_e42142: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign37450_e42143: f64 = (assign37450_e42142).exp();
        (assign37450_e42143, (assign37450_e42143 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign37450_e42143 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign37450_e42143 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign37450_e42143 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign37450_e42143 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign37450_e42145;
        locals.var_q_invexpq__blk831_dn4 = assign37450_e42145_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign37450_e42145_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign37450_e42145_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign37450_e42145_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign37450_e42145_d_n9;

        let (assign37460_e42162, assign37460_e42162_d_n4, assign37460_e42162_d_n6, assign37460_e42162_d_n7, assign37460_e42162_d_n8, assign37460_e42162_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37460_e42155: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign37460_e42156: f64 = (locals.var_q_rac_qsq__blk828 * assign37460_e42155);
        let assign37460_e42159: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign37460_e42160: f64 = (assign37460_e42156 / assign37460_e42159);
        (assign37460_e42160, (((((locals.var_q_rac_qsq__blk828_dn4 * assign37460_e42155) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign37460_e42159) - (assign37460_e42156 * (-locals.var_q_invexpq__blk831_dn4))) / (assign37460_e42159 * assign37460_e42159)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign37460_e42155) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign37460_e42159) - (assign37460_e42156 * (-locals.var_q_invexpq__blk831_dn6))) / (assign37460_e42159 * assign37460_e42159)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign37460_e42155) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign37460_e42159) - (assign37460_e42156 * (-locals.var_q_invexpq__blk831_dn7))) / (assign37460_e42159 * assign37460_e42159)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign37460_e42155) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign37460_e42159) - (assign37460_e42156 * (-locals.var_q_invexpq__blk831_dn8))) / (assign37460_e42159 * assign37460_e42159)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign37460_e42155) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign37460_e42159) - (assign37460_e42156 * (-locals.var_q_invexpq__blk831_dn9))) / (assign37460_e42159 * assign37460_e42159)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37460_e42162;
        locals.var_q_qcoth__blk829_dn4 = assign37460_e42162_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37460_e42162_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37460_e42162_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37460_e42162_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37460_e42162_d_n9;

        let (assign37470_e42175, assign37470_e42175_d_n4, assign37470_e42175_d_n6, assign37470_e42175_d_n7, assign37470_e42175_d_n8, assign37470_e42175_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37470_e42171: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign37470_e42173: f64 = (assign37470_e42171 / locals.var_q_qsq__blk825);
        (assign37470_e42173, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign37470_e42171 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign37470_e42171 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign37470_e42171 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign37470_e42171 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign37470_e42171 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37470_e42175;
        locals.var_q_temp1__blk814_dn4 = assign37470_e42175_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37470_e42175_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37470_e42175_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37470_e42175_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37470_e42175_d_n9;

        let (assign37480_e42192, assign37480_e42192_d_n4, assign37480_e42192_d_n6, assign37480_e42192_d_n7, assign37480_e42192_d_n8, assign37480_e42192_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37480_e42186: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign37480_e42187: f64 = (locals.var_q_qcoth__blk829 * assign37480_e42186);
        let assign37480_e42188: f64 = (locals.var_q_qsq__blk825 + assign37480_e42187);
        let assign37480_e42190: f64 = (assign37480_e42188 * locals.var_q_temp1__blk814);
        (assign37480_e42190, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign37480_e42186) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign37480_e42188 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign37480_e42186) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign37480_e42188 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign37480_e42186) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign37480_e42188 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign37480_e42186) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign37480_e42188 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign37480_e42186) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign37480_e42188 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37480_e42192;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37480_e42192_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37480_e42192_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37480_e42192_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37480_e42192_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37480_e42192_d_n9;

        let (assign37490_e42217, assign37490_e42217_d_n4, assign37490_e42217_d_n6, assign37490_e42217_d_n7, assign37490_e42217_d_n8, assign37490_e42217_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37490_e42202: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign37490_e42205: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign37490_e42206: f64 = (assign37490_e42202 * assign37490_e42205);
        let assign37490_e42207: f64 = (locals.var_q_d1_qsq__blk826 - assign37490_e42206);
        let assign37490_e42209: f64 = (assign37490_e42207 * locals.var_q_temp1__blk814);
        let assign37490_e42212: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign37490_e42214: f64 = (assign37490_e42212 / locals.var_q_d1_qsq__blk826);
        let assign37490_e42215: f64 = (assign37490_e42209 + assign37490_e42214);
        (assign37490_e42215, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign37490_e42205) + (assign37490_e42202 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign37490_e42207 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign37490_e42212 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign37490_e42205) + (assign37490_e42202 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign37490_e42207 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign37490_e42212 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign37490_e42205) + (assign37490_e42202 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign37490_e42207 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign37490_e42212 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign37490_e42205) + (assign37490_e42202 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign37490_e42207 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign37490_e42212 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign37490_e42205) + (assign37490_e42202 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign37490_e42207 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign37490_e42212 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign37490_e42217;
        locals.var_q_d2_qcoth__blk832_dn4 = assign37490_e42217_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign37490_e42217_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign37490_e42217_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign37490_e42217_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign37490_e42217_d_n9;

        let (assign37500_e42230, assign37500_e42230_d_n4, assign37500_e42230_d_n6, assign37500_e42230_d_n7, assign37500_e42230_d_n8, assign37500_e42230_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37500_e42227: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign37500_e42228: f64 = (1.0 - assign37500_e42227);
        (assign37500_e42228, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37500_e42230;
        locals.var_q_temp2__blk815_dn4 = assign37500_e42230_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37500_e42230_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37500_e42230_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37500_e42230_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37500_e42230_d_n9;

        let (assign37510_e42243, assign37510_e42243_d_n4, assign37510_e42243_d_n6, assign37510_e42243_d_n7, assign37510_e42243_d_n8, assign37510_e42243_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37510_e42239: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign37510_e42241: f64 = (assign37510_e42239 * locals.var_q_temp2__blk815);
        (assign37510_e42241, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37510_e42239 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37510_e42239 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37510_e42239 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37510_e42239 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign37510_e42239 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign37510_e42243;
        locals.var_q_d1_ln__blk835_dn4 = assign37510_e42243_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign37510_e42243_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign37510_e42243_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign37510_e42243_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign37510_e42243_d_n9;

        let (assign37520_e42264, assign37520_e42264_d_n4, assign37520_e42264_d_n6, assign37520_e42264_d_n7, assign37520_e42264_d_n8, assign37520_e42264_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 != 0.0)) {
        let assign37520_e42252: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign37520_e42257: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign37520_e42258: f64 = (locals.var_q_d1_ln__blk835 + assign37520_e42257);
        let assign37520_e42259: f64 = (locals.var_q_d1_qsq__blk826 * assign37520_e42258);
        let assign37520_e42260: f64 = (assign37520_e42252 - assign37520_e42259);
        let assign37520_e42262: f64 = (assign37520_e42260 / locals.var_q_qsq__blk825);
        (assign37520_e42262, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign37520_e42258) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign37520_e42260 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign37520_e42258) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign37520_e42260 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign37520_e42258) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign37520_e42260 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign37520_e42258) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign37520_e42260 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign37520_e42258) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign37520_e42260 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign37520_e42264;
        locals.var_q_d2_ln__blk836_dn4 = assign37520_e42264_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign37520_e42264_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign37520_e42264_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign37520_e42264_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign37520_e42264_d_n9;

        let (assign37530_e42292, assign37530_e42292_d_n4, assign37530_e42292_d_n6, assign37530_e42292_d_n7, assign37530_e42292_d_n8, assign37530_e42292_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37530_e42276: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign37530_e42280: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign37530_e42284: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign37530_e42285: f64 = (1.0 - assign37530_e42284);
        let assign37530_e42286: f64 = (assign37530_e42280 * assign37530_e42285);
        let assign37530_e42287: f64 = (1.0 - assign37530_e42286);
        let assign37530_e42288: f64 = (assign37530_e42276 * assign37530_e42287);
        let assign37530_e42289: f64 = (1.0 - assign37530_e42288);
        let assign37530_e42290: f64 = (0.1666666666667 * assign37530_e42289);
        (assign37530_e42290, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign37530_e42287) + (assign37530_e42276 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign37530_e42285) + (assign37530_e42280 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign37530_e42287) + (assign37530_e42276 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign37530_e42285) + (assign37530_e42280 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign37530_e42287) + (assign37530_e42276 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign37530_e42285) + (assign37530_e42280 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign37530_e42287) + (assign37530_e42276 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign37530_e42285) + (assign37530_e42280 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign37530_e42287) + (assign37530_e42276 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign37530_e42285) + (assign37530_e42280 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign37530_e42292;
        locals.var_q_temp3__blk816_dn4 = assign37530_e42292_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign37530_e42292_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign37530_e42292_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign37530_e42292_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign37530_e42292_d_n9;

        let (assign37540_e42306, assign37540_e42306_d_n4, assign37540_e42306_d_n6, assign37540_e42306_d_n7, assign37540_e42306_d_n8, assign37540_e42306_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37540_e42303: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign37540_e42304: f64 = (2.0 + assign37540_e42303);
        (assign37540_e42304, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign37540_e42306;
        locals.var_q_qcoth__blk829_dn4 = assign37540_e42306_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign37540_e42306_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign37540_e42306_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign37540_e42306_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign37540_e42306_d_n9;

        let (assign37550_e42334, assign37550_e42334_d_n4, assign37550_e42334_d_n6, assign37550_e42334_d_n7, assign37550_e42334_d_n8, assign37550_e42334_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37550_e42318: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37550_e42322: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign37550_e42326: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign37550_e42327: f64 = (1.0 - assign37550_e42326);
        let assign37550_e42328: f64 = (assign37550_e42322 * assign37550_e42327);
        let assign37550_e42329: f64 = (1.0 - assign37550_e42328);
        let assign37550_e42330: f64 = (assign37550_e42318 * assign37550_e42329);
        let assign37550_e42331: f64 = (1.0 - assign37550_e42330);
        let assign37550_e42332: f64 = (0.1666666666667 * assign37550_e42331);
        (assign37550_e42332, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign37550_e42329) + (assign37550_e42318 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign37550_e42327) + (assign37550_e42322 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign37550_e42329) + (assign37550_e42318 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign37550_e42327) + (assign37550_e42322 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign37550_e42329) + (assign37550_e42318 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign37550_e42327) + (assign37550_e42322 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign37550_e42329) + (assign37550_e42318 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign37550_e42327) + (assign37550_e42322 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign37550_e42329) + (assign37550_e42318 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign37550_e42327) + (assign37550_e42322 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37550_e42334;
        locals.var_q_temp1__blk814_dn4 = assign37550_e42334_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37550_e42334_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37550_e42334_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37550_e42334_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37550_e42334_d_n9;

        let (assign37560_e42346, assign37560_e42346_d_n4, assign37560_e42346_d_n6, assign37560_e42346_d_n7, assign37560_e42346_d_n8, assign37560_e42346_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37560_e42344: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign37560_e42344, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign37560_e42346;
        locals.var_q_d1_qcoth__blk830_dn4 = assign37560_e42346_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign37560_e42346_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign37560_e42346_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign37560_e42346_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign37560_e42346_d_n9;

        let (assign37570_e42374, assign37570_e42374_d_n4, assign37570_e42374_d_n6, assign37570_e42374_d_n7, assign37570_e42374_d_n8, assign37570_e42374_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37570_e42358: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign37570_e42362: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign37570_e42366: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign37570_e42367: f64 = (1.0 - assign37570_e42366);
        let assign37570_e42368: f64 = (assign37570_e42362 * assign37570_e42367);
        let assign37570_e42369: f64 = (1.0 - assign37570_e42368);
        let assign37570_e42370: f64 = (assign37570_e42358 * assign37570_e42369);
        let assign37570_e42371: f64 = (1.0 - assign37570_e42370);
        let assign37570_e42372: f64 = (0.0055555555556 * assign37570_e42371);
        (assign37570_e42372, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign37570_e42369) + (assign37570_e42358 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign37570_e42367) + (assign37570_e42362 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign37570_e42369) + (assign37570_e42358 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign37570_e42367) + (assign37570_e42362 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign37570_e42369) + (assign37570_e42358 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign37570_e42367) + (assign37570_e42362 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign37570_e42369) + (assign37570_e42358 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign37570_e42367) + (assign37570_e42362 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign37570_e42369) + (assign37570_e42358 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign37570_e42367) + (assign37570_e42362 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37570_e42374;
        locals.var_q_temp2__blk815_dn4 = assign37570_e42374_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37570_e42374_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37570_e42374_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37570_e42374_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37570_e42374_d_n9;

    }

    pub(super) fn stamp_transient_block_101(
        locals: &mut StampLocals,
    ) {
        let (assign37580_e42392, assign37580_e42392_d_n4, assign37580_e42392_d_n6, assign37580_e42392_d_n7, assign37580_e42392_d_n8, assign37580_e42392_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37580_e42384: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign37580_e42387: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign37580_e42389: f64 = (assign37580_e42387 * locals.var_q_temp2__blk815);
        let assign37580_e42390: f64 = (assign37580_e42384 - assign37580_e42389);
        (assign37580_e42390, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign37580_e42387 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign37580_e42387 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign37580_e42387 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign37580_e42387 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign37580_e42387 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign37580_e42392;
        locals.var_q_d2_qcoth__blk832_dn4 = assign37580_e42392_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign37580_e42392_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign37580_e42392_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign37580_e42392_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign37580_e42392_d_n9;

        let (assign37590_e42407, assign37590_e42407_d_n4, assign37590_e42407_d_n6, assign37590_e42407_d_n7, assign37590_e42407_d_n8, assign37590_e42407_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37590_e42401: f64 = (-0.5);
        let assign37590_e42403: f64 = (assign37590_e42401 * locals.var_q_d1_qsq__blk826);
        let assign37590_e42405: f64 = (assign37590_e42403 * locals.var_q_temp3__blk816);
        (assign37590_e42405, (((assign37590_e42401 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign37590_e42403 * locals.var_q_temp3__blk816_dn4)), (((assign37590_e42401 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign37590_e42403 * locals.var_q_temp3__blk816_dn6)), (((assign37590_e42401 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign37590_e42403 * locals.var_q_temp3__blk816_dn7)), (((assign37590_e42401 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign37590_e42403 * locals.var_q_temp3__blk816_dn8)), (((assign37590_e42401 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign37590_e42403 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign37590_e42407;
        locals.var_q_d1_ln__blk835_dn4 = assign37590_e42407_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign37590_e42407_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign37590_e42407_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign37590_e42407_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign37590_e42407_d_n9;

        let (assign37600_e42442, assign37600_e42442_d_n4, assign37600_e42442_d_n6, assign37600_e42442_d_n7, assign37600_e42442_d_n8, assign37600_e42442_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1183 == 0.0)) && (locals.var_guard1184 == 0.0)) {
        let assign37600_e42416: f64 = (-0.5);
        let assign37600_e42418: f64 = (assign37600_e42416 * locals.var_q_d2_qsq__blk827);
        let assign37600_e42420: f64 = (assign37600_e42418 * locals.var_q_temp3__blk816);
        let assign37600_e42423: f64 = (0.25 * 0.0055555555556);
        let assign37600_e42425: f64 = (assign37600_e42423 * locals.var_q_d1_qsq__blk826);
        let assign37600_e42427: f64 = (assign37600_e42425 * locals.var_q_d1_qsq__blk826);
        let assign37600_e42431: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign37600_e42435: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign37600_e42436: f64 = (2.0 - assign37600_e42435);
        let assign37600_e42437: f64 = (assign37600_e42431 * assign37600_e42436);
        let assign37600_e42438: f64 = (1.0 - assign37600_e42437);
        let assign37600_e42439: f64 = (assign37600_e42427 * assign37600_e42438);
        let assign37600_e42440: f64 = (assign37600_e42420 + assign37600_e42439);
        (assign37600_e42440, ((((assign37600_e42416 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign37600_e42418 * locals.var_q_temp3__blk816_dn4)) + (((((assign37600_e42423 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign37600_e42425 * locals.var_q_d1_qsq__blk826_dn4)) * assign37600_e42438) + (assign37600_e42427 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign37600_e42436) + (assign37600_e42431 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign37600_e42416 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign37600_e42418 * locals.var_q_temp3__blk816_dn6)) + (((((assign37600_e42423 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign37600_e42425 * locals.var_q_d1_qsq__blk826_dn6)) * assign37600_e42438) + (assign37600_e42427 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign37600_e42436) + (assign37600_e42431 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign37600_e42416 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign37600_e42418 * locals.var_q_temp3__blk816_dn7)) + (((((assign37600_e42423 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign37600_e42425 * locals.var_q_d1_qsq__blk826_dn7)) * assign37600_e42438) + (assign37600_e42427 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign37600_e42436) + (assign37600_e42431 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign37600_e42416 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign37600_e42418 * locals.var_q_temp3__blk816_dn8)) + (((((assign37600_e42423 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign37600_e42425 * locals.var_q_d1_qsq__blk826_dn8)) * assign37600_e42438) + (assign37600_e42427 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign37600_e42436) + (assign37600_e42431 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign37600_e42416 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign37600_e42418 * locals.var_q_temp3__blk816_dn9)) + (((((assign37600_e42423 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign37600_e42425 * locals.var_q_d1_qsq__blk826_dn9)) * assign37600_e42438) + (assign37600_e42427 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign37600_e42436) + (assign37600_e42431 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign37600_e42442;
        locals.var_q_d2_ln__blk836_dn4 = assign37600_e42442_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign37600_e42442_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign37600_e42442_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign37600_e42442_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign37600_e42442_d_n9;

        let assign37610_e42445: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1185 = assign37610_e42445;

        let (assign37620_e42461, assign37620_e42461_d_n4, assign37620_e42461_d_n6, assign37620_e42461_d_n7, assign37620_e42461_d_n8, assign37620_e42461_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1185 != 0.0)) {
        let assign37620_e42451: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign37620_e42456: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign37620_e42457: f64 = (locals.var_q_invexpq__blk831 * assign37620_e42456);
        let assign37620_e42458: f64 = (1.0 - assign37620_e42457);
        let assign37620_e42459: f64 = (assign37620_e42451 / assign37620_e42458);
        (assign37620_e42459, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign37620_e42458) - (assign37620_e42451 * (-((locals.var_q_invexpq__blk831_dn4 * assign37620_e42456) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign37620_e42458 * assign37620_e42458)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign37620_e42458) - (assign37620_e42451 * (-((locals.var_q_invexpq__blk831_dn6 * assign37620_e42456) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign37620_e42458 * assign37620_e42458)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign37620_e42458) - (assign37620_e42451 * (-((locals.var_q_invexpq__blk831_dn7 * assign37620_e42456) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign37620_e42458 * assign37620_e42458)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign37620_e42458) - (assign37620_e42451 * (-((locals.var_q_invexpq__blk831_dn8 * assign37620_e42456) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign37620_e42458 * assign37620_e42458)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign37620_e42458) - (assign37620_e42451 * (-((locals.var_q_invexpq__blk831_dn9 * assign37620_e42456) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign37620_e42458 * assign37620_e42458)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37620_e42461;
        locals.var_q_temp2__blk815_dn4 = assign37620_e42461_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37620_e42461_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37620_e42461_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37620_e42461_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37620_e42461_d_n9;

        let (assign37630_e42469, assign37630_e42469_d_n4, assign37630_e42469_d_n6, assign37630_e42469_d_n7, assign37630_e42469_d_n8, assign37630_e42469_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1185 != 0.0)) {
        let assign37630_e42467: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign37630_e42467, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign37630_e42469;
        locals.var_q_sh_term__blk833_dn4 = assign37630_e42469_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign37630_e42469_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign37630_e42469_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign37630_e42469_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign37630_e42469_d_n9;

        let (assign37640_e42478, assign37640_e42478_d_n4, assign37640_e42478_d_n6, assign37640_e42478_d_n7, assign37640_e42478_d_n8, assign37640_e42478_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1185 != 0.0)) {
        let assign37640_e42474: f64 = (locals.var_q_temp2__blk815).ln();
        let assign37640_e42476: f64 = (assign37640_e42474 - locals.var_q_rac_qsq__blk828);
        (assign37640_e42476, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign37640_e42478;
        locals.var_q_ln_term__blk834_dn4 = assign37640_e42478_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign37640_e42478_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign37640_e42478_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign37640_e42478_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign37640_e42478_d_n9;

        let assign37650_e42481: f64 = (-0.005);
        let assign37650_e42482: f64 = if locals.var_q_qsq__blk825 < assign37650_e42481 { 1.0 } else { 0.0 };
        locals.var_guard1186 = assign37650_e42482;

        let (assign37660_e42494, assign37660_e42494_d_n4, assign37660_e42494_d_n6, assign37660_e42494_d_n7, assign37660_e42494_d_n8, assign37660_e42494_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign37660_e42491: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign37660_e42492: f64 = (assign37660_e42491).sin();
        (assign37660_e42492, ((assign37660_e42491).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign37660_e42491).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign37660_e42491).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign37660_e42491).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign37660_e42491).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37660_e42494;
        locals.var_q_temp2__blk815_dn4 = assign37660_e42494_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37660_e42494_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37660_e42494_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37660_e42494_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37660_e42494_d_n9;

        let (assign37670_e42508, assign37670_e42508_d_n4, assign37670_e42508_d_n6, assign37670_e42508_d_n7, assign37670_e42508_d_n8, assign37670_e42508_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign37670_e42502: f64 = (-locals.var_q_qsq__blk825);
        let assign37670_e42505: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign37670_e42506: f64 = (assign37670_e42502 / assign37670_e42505);
        (assign37670_e42506, ((((-locals.var_q_qsq__blk825_dn4) * assign37670_e42505) - (assign37670_e42502 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign37670_e42505 * assign37670_e42505)), ((((-locals.var_q_qsq__blk825_dn6) * assign37670_e42505) - (assign37670_e42502 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign37670_e42505 * assign37670_e42505)), ((((-locals.var_q_qsq__blk825_dn7) * assign37670_e42505) - (assign37670_e42502 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign37670_e42505 * assign37670_e42505)), ((((-locals.var_q_qsq__blk825_dn8) * assign37670_e42505) - (assign37670_e42502 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign37670_e42505 * assign37670_e42505)), ((((-locals.var_q_qsq__blk825_dn9) * assign37670_e42505) - (assign37670_e42502 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign37670_e42505 * assign37670_e42505)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign37670_e42508;
        locals.var_q_sh_term__blk833_dn4 = assign37670_e42508_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign37670_e42508_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign37670_e42508_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign37670_e42508_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign37670_e42508_d_n9;

        let (assign37680_e42518, assign37680_e42518_d_n4, assign37680_e42518_d_n6, assign37680_e42518_d_n7, assign37680_e42518_d_n8, assign37680_e42518_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 != 0.0)) {
        let assign37680_e42516: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign37680_e42516, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign37680_e42518;
        locals.var_q_ln_term__blk834_dn4 = assign37680_e42518_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign37680_e42518_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign37680_e42518_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign37680_e42518_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign37680_e42518_d_n9;

        let (assign37690_e42544, assign37690_e42544_d_n4, assign37690_e42544_d_n6, assign37690_e42544_d_n7, assign37690_e42544_d_n8, assign37690_e42544_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 == 0.0)) {
        let assign37690_e42529: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign37690_e42533: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign37690_e42537: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign37690_e42538: f64 = (1.0 - assign37690_e42537);
        let assign37690_e42539: f64 = (assign37690_e42533 * assign37690_e42538);
        let assign37690_e42540: f64 = (1.0 - assign37690_e42539);
        let assign37690_e42541: f64 = (assign37690_e42529 * assign37690_e42540);
        let assign37690_e42542: f64 = (4.0 - assign37690_e42541);
        (assign37690_e42542, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign37690_e42540) + (assign37690_e42529 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign37690_e42538) + (assign37690_e42533 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign37690_e42540) + (assign37690_e42529 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign37690_e42538) + (assign37690_e42533 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign37690_e42540) + (assign37690_e42529 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign37690_e42538) + (assign37690_e42533 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign37690_e42540) + (assign37690_e42529 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign37690_e42538) + (assign37690_e42533 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign37690_e42540) + (assign37690_e42529 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign37690_e42538) + (assign37690_e42533 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign37690_e42544;
        locals.var_q_sh_term__blk833_dn4 = assign37690_e42544_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign37690_e42544_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign37690_e42544_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign37690_e42544_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign37690_e42544_d_n9;

        let (assign37700_e42555, assign37700_e42555_d_n4, assign37700_e42555_d_n6, assign37700_e42555_d_n7, assign37700_e42555_d_n8, assign37700_e42555_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1185 == 0.0)) && (locals.var_guard1186 == 0.0)) {
        let assign37700_e42553: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign37700_e42553, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign37700_e42555;
        locals.var_q_ln_term__blk834_dn4 = assign37700_e42555_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign37700_e42555_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign37700_e42555_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign37700_e42555_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign37700_e42555_d_n9;

        let assign37710_e42558: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign37710_e42560: f64 = (assign37710_e42558 + locals.var_q_qcoth__blk829);
        let assign37710_e42562: f64 = if assign37710_e42560 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1187 = assign37710_e42562;

        let (assign37720_e42570, assign37720_e42570_d_n4, assign37720_e42570_d_n6, assign37720_e42570_d_n7, assign37720_e42570_d_n8, assign37720_e42570_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 != 0.0)) {
        let assign37720_e42568: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign37720_e42568, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign37720_e42570;
        locals.var_q_expnum__blk837_dn4 = assign37720_e42570_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign37720_e42570_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign37720_e42570_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign37720_e42570_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign37720_e42570_d_n9;

        let (assign37730_e42578, assign37730_e42578_d_n4, assign37730_e42578_d_n6, assign37730_e42578_d_n7, assign37730_e42578_d_n8, assign37730_e42578_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 != 0.0)) {
        let assign37730_e42576: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign37730_e42576, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign37730_e42578;
        locals.var_q_d1_expnum__blk838_dn4 = assign37730_e42578_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign37730_e42578_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign37730_e42578_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign37730_e42578_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign37730_e42578_d_n9;

        let (assign37740_e42584, assign37740_e42584_d_n4, assign37740_e42584_d_n6, assign37740_e42584_d_n7, assign37740_e42584_d_n8, assign37740_e42584_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign37740_e42584;
        locals.var_q_d2_expnum__blk839_dn4 = assign37740_e42584_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign37740_e42584_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign37740_e42584_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign37740_e42584_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign37740_e42584_d_n9;

        let (assign37750_e42595, assign37750_e42595_d_n4, assign37750_e42595_d_n6, assign37750_e42595_d_n7, assign37750_e42595_d_n8, assign37750_e42595_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37750_e42592: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign37750_e42593: f64 = (1.0 / assign37750_e42592);
        (assign37750_e42593, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign37750_e42592 * assign37750_e42592))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign37750_e42592 * assign37750_e42592))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign37750_e42592 * assign37750_e42592))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign37750_e42592 * assign37750_e42592))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign37750_e42592 * assign37750_e42592))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign37750_e42595;
        locals.var_q_temp2__blk815_dn4 = assign37750_e42595_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign37750_e42595_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign37750_e42595_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign37750_e42595_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign37750_e42595_d_n9;

        let (assign37760_e42604, assign37760_e42604_d_n4, assign37760_e42604_d_n6, assign37760_e42604_d_n7, assign37760_e42604_d_n8, assign37760_e42604_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37760_e42602: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign37760_e42602, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign37760_e42604;
        locals.var_q_temp3__blk816_dn4 = assign37760_e42604_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign37760_e42604_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign37760_e42604_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign37760_e42604_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign37760_e42604_d_n9;

        let (assign37770_e42615, assign37770_e42615_d_n4, assign37770_e42615_d_n6, assign37770_e42615_d_n7, assign37770_e42615_d_n8, assign37770_e42615_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37770_e42611: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign37770_e42613: f64 = (assign37770_e42611 * locals.var_q_temp2__blk815);
        (assign37770_e42613, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign37770_e42611 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign37770_e42611 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign37770_e42611 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign37770_e42611 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign37770_e42611 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign37770_e42615;
        locals.var_q_expnum__blk837_dn4 = assign37770_e42615_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign37770_e42615_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign37770_e42615_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign37770_e42615_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign37770_e42615_d_n9;

        let (assign37780_e42632, assign37780_e42632_d_n4, assign37780_e42632_d_n6, assign37780_e42632_d_n7, assign37780_e42632_d_n8, assign37780_e42632_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37780_e42622: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign37780_e42624: f64 = (assign37780_e42622 - locals.var_q_aexp__blk824);
        let assign37780_e42627: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign37780_e42628: f64 = (assign37780_e42624 - assign37780_e42627);
        let assign37780_e42630: f64 = (assign37780_e42628 * locals.var_q_temp2__blk815);
        (assign37780_e42630, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign37780_e42628 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign37780_e42628 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign37780_e42628 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign37780_e42628 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign37780_e42628 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign37780_e42632;
        locals.var_q_d1_expnum__blk838_dn4 = assign37780_e42632_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign37780_e42632_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign37780_e42632_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign37780_e42632_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign37780_e42632_d_n9;

        let (assign37790_e42659, assign37790_e42659_d_n4, assign37790_e42659_d_n6, assign37790_e42659_d_n7, assign37790_e42659_d_n8, assign37790_e42659_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1187 == 0.0)) {
        let assign37790_e42639: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign37790_e42642: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign37790_e42644: f64 = (assign37790_e42642 * locals.var_q_d1_expnum__blk838);
        let assign37790_e42645: f64 = (assign37790_e42639 + assign37790_e42644);
        let assign37790_e42647: f64 = (assign37790_e42645 + locals.var_q_aexp__blk824);
        let assign37790_e42651: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign37790_e42652: f64 = (locals.var_q_d2_ln__blk836 + assign37790_e42651);
        let assign37790_e42654: f64 = (assign37790_e42652 * locals.var_q_sh_term__blk833);
        let assign37790_e42655: f64 = (assign37790_e42647 - assign37790_e42654);
        let assign37790_e42657: f64 = (assign37790_e42655 * locals.var_q_temp2__blk815);
        (assign37790_e42657, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign37790_e42642 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign37790_e42652 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign37790_e42655 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign37790_e42642 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign37790_e42652 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign37790_e42655 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign37790_e42642 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign37790_e42652 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign37790_e42655 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign37790_e42642 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign37790_e42652 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign37790_e42655 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign37790_e42642 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign37790_e42652 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign37790_e42655 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign37790_e42659;
        locals.var_q_d2_expnum__blk839_dn4 = assign37790_e42659_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign37790_e42659_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign37790_e42659_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign37790_e42659_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign37790_e42659_d_n9;

        let assign37800_e42662: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1188 = assign37800_e42662;

        let (assign37810_e42669, assign37810_e42669_d_n4, assign37810_e42669_d_n6, assign37810_e42669_d_n7, assign37810_e42669_d_n8, assign37810_e42669_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37810_e42667: f64 = (locals.var_q_expnum__blk837).ln();
        (assign37810_e42667, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign37810_e42669;
        locals.var_q_lnexpnum__blk840_dn4 = assign37810_e42669_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign37810_e42669_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign37810_e42669_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign37810_e42669_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign37810_e42669_d_n9;

        let (assign37820_e42677, assign37820_e42677_d_n4, assign37820_e42677_d_n6, assign37820_e42677_d_n7, assign37820_e42677_d_n8, assign37820_e42677_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37820_e42675: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign37820_e42675, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37820_e42677;
        locals.var_q_temp1__blk814_dn4 = assign37820_e42677_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37820_e42677_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37820_e42677_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37820_e42677_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37820_e42677_d_n9;

        let (assign37830_e42685, assign37830_e42685_d_n4, assign37830_e42685_d_n6, assign37830_e42685_d_n7, assign37830_e42685_d_n8, assign37830_e42685_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37830_e42683: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign37830_e42683, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign37830_e42685;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign37830_e42685_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign37830_e42685_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign37830_e42685_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign37830_e42685_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign37830_e42685_d_n9;

        let (assign37840_e42697, assign37840_e42697_d_n4, assign37840_e42697_d_n6, assign37840_e42697_d_n7, assign37840_e42697_d_n8, assign37840_e42697_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 != 0.0)) {
        let assign37840_e42691: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign37840_e42694: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign37840_e42695: f64 = (assign37840_e42691 - assign37840_e42694);
        (assign37840_e42695, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign37840_e42697;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign37840_e42697_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign37840_e42697_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign37840_e42697_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign37840_e42697_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign37840_e42697_d_n9;

        let (assign37850_e42710, assign37850_e42710_d_n4, assign37850_e42710_d_n6, assign37850_e42710_d_n7, assign37850_e42710_d_n8, assign37850_e42710_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37850_e42704: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign37850_e42706: f64 = (-locals.var_q_k1q1__blk823);
        let assign37850_e42707: f64 = (assign37850_e42706).ln();
        let assign37850_e42708: f64 = (assign37850_e42704 + assign37850_e42707);
        (assign37850_e42708, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign37850_e42706)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign37850_e42706)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign37850_e42706)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign37850_e42706)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign37850_e42706)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign37850_e42710;
        locals.var_q_lnexpnum__blk840_dn4 = assign37850_e42710_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign37850_e42710_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign37850_e42710_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign37850_e42710_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign37850_e42710_d_n9;

        let (assign37860_e42719, assign37860_e42719_d_n4, assign37860_e42719_d_n6, assign37860_e42719_d_n7, assign37860_e42719_d_n8, assign37860_e42719_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37860_e42717: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign37860_e42717, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign37860_e42719;
        locals.var_q_temp1__blk814_dn4 = assign37860_e42719_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign37860_e42719_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign37860_e42719_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign37860_e42719_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign37860_e42719_d_n9;

        let (assign37870_e42728, assign37870_e42728_d_n4, assign37870_e42728_d_n6, assign37870_e42728_d_n7, assign37870_e42728_d_n8, assign37870_e42728_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37870_e42726: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign37870_e42726, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign37870_e42728;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign37870_e42728_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign37870_e42728_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign37870_e42728_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign37870_e42728_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign37870_e42728_d_n9;

        let (assign37880_e42738, assign37880_e42738_d_n4, assign37880_e42738_d_n6, assign37880_e42738_d_n7, assign37880_e42738_d_n8, assign37880_e42738_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1188 == 0.0)) {
        let assign37880_e42734: f64 = (-locals.var_q_temp1__blk814);
        let assign37880_e42736: f64 = (assign37880_e42734 * locals.var_q_temp1__blk814);
        (assign37880_e42736, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign37880_e42734 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign37880_e42734 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign37880_e42734 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign37880_e42734 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign37880_e42734 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign37880_e42738;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign37880_e42738_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign37880_e42738_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign37880_e42738_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign37880_e42738_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign37880_e42738_d_n9;

        let (assign37890_e42752, assign37890_e42752_d_n4, assign37890_e42752_d_n6, assign37890_e42752_d_n7, assign37890_e42752_d_n8, assign37890_e42752_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37890_e42742: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign37890_e42744: f64 = (assign37890_e42742 + locals.var_q1d__blk1001);
        let assign37890_e42747: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign37890_e42748: f64 = (assign37890_e42744 + assign37890_e42747);
        let assign37890_e42750: f64 = (assign37890_e42748 - locals.var_q_ln_term__blk834);
        (assign37890_e42750, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign37890_e42752;
        locals.var_q_q2_int__blk843_dn4 = assign37890_e42752_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign37890_e42752_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign37890_e42752_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign37890_e42752_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign37890_e42752_d_n9;

        let (assign37900_e42762, assign37900_e42762_d_n4, assign37900_e42762_d_n6, assign37900_e42762_d_n7, assign37900_e42762_d_n8, assign37900_e42762_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37900_e42757: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign37900_e42758: f64 = (1.0 + assign37900_e42757);
        let assign37900_e42760: f64 = (assign37900_e42758 - locals.var_q_d1_ln__blk835);
        (assign37900_e42760, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign37900_e42762;
        locals.var_q_d1_q2__blk844_dn4 = assign37900_e42762_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign37900_e42762_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign37900_e42762_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign37900_e42762_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign37900_e42762_d_n9;

    }

    pub(super) fn stamp_transient_block_102(
        locals: &mut StampLocals,
    ) {
        let (assign37910_e42770, assign37910_e42770_d_n4, assign37910_e42770_d_n6, assign37910_e42770_d_n7, assign37910_e42770_d_n8, assign37910_e42770_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37910_e42766: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign37910_e42768: f64 = (assign37910_e42766 - locals.var_q_d2_ln__blk836);
        (assign37910_e42768, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign37910_e42770;
        locals.var_q_d2_q2__blk845_dn4 = assign37910_e42770_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign37910_e42770_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign37910_e42770_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign37910_e42770_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign37910_e42770_d_n9;

        let (assign37920_e42778, assign37920_e42778_d_n4, assign37920_e42778_d_n6, assign37920_e42778_d_n7, assign37920_e42778_d_n8, assign37920_e42778_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37920_e42775: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign37920_e42776: f64 = (locals.var_q_k1q1__blk823 + assign37920_e42775);
        (assign37920_e42776, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign37920_e42778;
        locals.var_q_qi_int__blk846_dn4 = assign37920_e42778_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign37920_e42778_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign37920_e42778_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign37920_e42778_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign37920_e42778_d_n9;

        let (assign37930_e42786, assign37930_e42786_d_n4, assign37930_e42786_d_n6, assign37930_e42786_d_n7, assign37930_e42786_d_n8, assign37930_e42786_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37930_e42783: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign37930_e42784: f64 = (locals.var_k1__blk932 + assign37930_e42783);
        (assign37930_e42784, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign37930_e42786;
        locals.var_q_d1_qi__blk847_dn4 = assign37930_e42786_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign37930_e42786_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign37930_e42786_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign37930_e42786_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign37930_e42786_d_n9;

        let (assign37940_e42792, assign37940_e42792_d_n4, assign37940_e42792_d_n6, assign37940_e42792_d_n7, assign37940_e42792_d_n8, assign37940_e42792_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37940_e42790: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign37940_e42790, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign37940_e42792;
        locals.var_q_d2_qi__blk848_dn4 = assign37940_e42792_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign37940_e42792_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign37940_e42792_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign37940_e42792_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign37940_e42792_d_n9;

        let (assign37950_e42800, assign37950_e42800_d_n4, assign37950_e42800_d_n6, assign37950_e42800_d_n7, assign37950_e42800_d_n8, assign37950_e42800_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37950_e42796: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign37950_e42798: f64 = (assign37950_e42796 - locals.var_q_aexp__blk824);
        (assign37950_e42798, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign37950_e42800;
        locals.var_q_zero__blk849_dn4 = assign37950_e42800_d_n4;
        locals.var_q_zero__blk849_dn6 = assign37950_e42800_d_n6;
        locals.var_q_zero__blk849_dn7 = assign37950_e42800_d_n7;
        locals.var_q_zero__blk849_dn8 = assign37950_e42800_d_n8;
        locals.var_q_zero__blk849_dn9 = assign37950_e42800_d_n9;

        let (assign37960_e42812, assign37960_e42812_d_n4, assign37960_e42812_d_n6, assign37960_e42812_d_n7, assign37960_e42812_d_n8, assign37960_e42812_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37960_e42804: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign37960_e42807: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign37960_e42808: f64 = (assign37960_e42804 + assign37960_e42807);
        let assign37960_e42810: f64 = (assign37960_e42808 + locals.var_q_aexp__blk824);
        (assign37960_e42810, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign37960_e42812;
        locals.var_q_d1_zero__blk850_dn4 = assign37960_e42812_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign37960_e42812_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign37960_e42812_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign37960_e42812_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign37960_e42812_d_n9;

        let (assign37970_e42830, assign37970_e42830_d_n4, assign37970_e42830_d_n6, assign37970_e42830_d_n7, assign37970_e42830_d_n8, assign37970_e42830_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37970_e42816: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign37970_e42819: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign37970_e42821: f64 = (assign37970_e42819 * locals.var_q_d1_expnum__blk838);
        let assign37970_e42822: f64 = (assign37970_e42816 + assign37970_e42821);
        let assign37970_e42825: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign37970_e42826: f64 = (assign37970_e42822 + assign37970_e42825);
        let assign37970_e42828: f64 = (assign37970_e42826 - locals.var_q_aexp__blk824);
        (assign37970_e42828, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign37970_e42819 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign37970_e42819 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign37970_e42819 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign37970_e42819 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign37970_e42819 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign37970_e42830;
        locals.var_q_d2_zero__blk851_dn4 = assign37970_e42830_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign37970_e42830_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign37970_e42830_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign37970_e42830_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign37970_e42830_d_n9;

        let (assign37980_e42842, assign37980_e42842_d_n4, assign37980_e42842_d_n6, assign37980_e42842_d_n7, assign37980_e42842_d_n8, assign37980_e42842_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37980_e42834: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign37980_e42837: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign37980_e42839: f64 = (assign37980_e42837 * locals.var_q_d2_zero__blk851);
        let assign37980_e42840: f64 = (assign37980_e42834 - assign37980_e42839);
        (assign37980_e42840, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign37980_e42837 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign37980_e42837 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign37980_e42837 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign37980_e42837 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign37980_e42837 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign37980_e42842;
        locals.var_q_temp__blk860_dn4 = assign37980_e42842_d_n4;
        locals.var_q_temp__blk860_dn6 = assign37980_e42842_d_n6;
        locals.var_q_temp__blk860_dn7 = assign37980_e42842_d_n7;
        locals.var_q_temp__blk860_dn8 = assign37980_e42842_d_n8;
        locals.var_q_temp__blk860_dn9 = assign37980_e42842_d_n9;

        let (assign37990_e42857, assign37990_e42857_d_n4, assign37990_e42857_d_n6, assign37990_e42857_d_n7, assign37990_e42857_d_n8, assign37990_e42857_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign37990_e42845: f64 = (-locals.var_q_zero__blk849);
        let assign37990_e42847: f64 = (assign37990_e42845 * locals.var_q_d1_zero__blk850);
        let assign37990_e42849: f64 = (assign37990_e42847 * locals.var_q_temp__blk860);
        let assign37990_e42852: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign37990_e42854: f64 = (assign37990_e42852 + 1e-200);
        let assign37990_e42855: f64 = (assign37990_e42849 / assign37990_e42854);
        (assign37990_e42855, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign37990_e42845 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign37990_e42847 * locals.var_q_temp__blk860_dn4)) * assign37990_e42854) - (assign37990_e42849 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign37990_e42854 * assign37990_e42854)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign37990_e42845 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign37990_e42847 * locals.var_q_temp__blk860_dn6)) * assign37990_e42854) - (assign37990_e42849 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign37990_e42854 * assign37990_e42854)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign37990_e42845 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign37990_e42847 * locals.var_q_temp__blk860_dn7)) * assign37990_e42854) - (assign37990_e42849 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign37990_e42854 * assign37990_e42854)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign37990_e42845 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign37990_e42847 * locals.var_q_temp__blk860_dn8)) * assign37990_e42854) - (assign37990_e42849 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign37990_e42854 * assign37990_e42854)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign37990_e42845 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign37990_e42847 * locals.var_q_temp__blk860_dn9)) * assign37990_e42854) - (assign37990_e42849 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign37990_e42854 * assign37990_e42854)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign37990_e42857;
        locals.var_q_eps2__blk852_dn4 = assign37990_e42857_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign37990_e42857_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign37990_e42857_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign37990_e42857_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign37990_e42857_d_n9;

        let (assign38000_e42863, assign38000_e42863_d_n4, assign38000_e42863_d_n6, assign38000_e42863_d_n7, assign38000_e42863_d_n8, assign38000_e42863_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38000_e42861: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign38000_e42861, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign38000_e42863;
        locals.var_q1d__blk1001_dn4 = assign38000_e42863_d_n4;
        locals.var_q1d__blk1001_dn6 = assign38000_e42863_d_n6;
        locals.var_q1d__blk1001_dn7 = assign38000_e42863_d_n7;
        locals.var_q1d__blk1001_dn8 = assign38000_e42863_d_n8;
        locals.var_q1d__blk1001_dn9 = assign38000_e42863_d_n9;

        let (assign38010_e42869, assign38010_e42869_d_n4, assign38010_e42869_d_n6, assign38010_e42869_d_n7, assign38010_e42869_d_n8, assign38010_e42869_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38010_e42867: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign38010_e42867, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign38010_e42869;
        locals.var_q_k1q1__blk823_dn4 = assign38010_e42869_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign38010_e42869_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign38010_e42869_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign38010_e42869_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign38010_e42869_d_n9;

        let assign38020_e42872: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38020_e42874: f64 = (assign38020_e42872 - locals.var_xdeff__blk1000);
        let assign38020_e42876: f64 = if assign38020_e42874 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign38020_e42876;

        let (assign38030_e42887, assign38030_e42887_d_n4, assign38030_e42887_d_n6, assign38030_e42887_d_n7, assign38030_e42887_d_n8, assign38030_e42887_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1189 != 0.0)) {
        let assign38030_e42882: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38030_e42884: f64 = (assign38030_e42882 - locals.var_xdeff__blk1000);
        let assign38030_e42885: f64 = (assign38030_e42884).exp();
        (assign38030_e42885, (assign38030_e42885 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign38030_e42885 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign38030_e42885 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign38030_e42885 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign38030_e42885 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38030_e42887;
        locals.var_q_temp1__blk814_dn4 = assign38030_e42887_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38030_e42887_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38030_e42887_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38030_e42887_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38030_e42887_d_n9;

        let (assign38040_e42928, assign38040_e42928_d_n4, assign38040_e42928_d_n6, assign38040_e42928_d_n7, assign38040_e42928_d_n8, assign38040_e42928_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1189 == 0.0)) {
        let assign38040_e42896: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38040_e42898: f64 = (assign38040_e42896 - locals.var_xdeff__blk1000);
        let assign38040_e42900: f64 = (assign38040_e42898 - 80.0);
        let assign38040_e42905: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38040_e42907: f64 = (assign38040_e42905 - locals.var_xdeff__blk1000);
        let assign38040_e42909: f64 = (assign38040_e42907 - 80.0);
        let assign38040_e42910: f64 = (0.5 * assign38040_e42909);
        let assign38040_e42914: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38040_e42916: f64 = (assign38040_e42914 - locals.var_xdeff__blk1000);
        let assign38040_e42918: f64 = (assign38040_e42916 - 80.0);
        let assign38040_e42920: f64 = (assign38040_e42918 * 0.3333333333333);
        let assign38040_e42921: f64 = (1.0 + assign38040_e42920);
        let assign38040_e42922: f64 = (assign38040_e42910 * assign38040_e42921);
        let assign38040_e42923: f64 = (1.0 + assign38040_e42922);
        let assign38040_e42924: f64 = (assign38040_e42900 * assign38040_e42923);
        let assign38040_e42925: f64 = (1.0 + assign38040_e42924);
        let assign38040_e42926: f64 = (5.54062e34 * assign38040_e42925);
        (assign38040_e42926, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign38040_e42923) + (assign38040_e42900 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign38040_e42921) + (assign38040_e42910 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign38040_e42923) + (assign38040_e42900 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign38040_e42921) + (assign38040_e42910 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign38040_e42923) + (assign38040_e42900 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign38040_e42921) + (assign38040_e42910 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign38040_e42923) + (assign38040_e42900 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign38040_e42921) + (assign38040_e42910 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign38040_e42923) + (assign38040_e42900 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign38040_e42921) + (assign38040_e42910 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38040_e42928;
        locals.var_q_temp1__blk814_dn4 = assign38040_e42928_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38040_e42928_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38040_e42928_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38040_e42928_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38040_e42928_d_n9;

        let (assign38050_e42934, assign38050_e42934_d_n4, assign38050_e42934_d_n6, assign38050_e42934_d_n7, assign38050_e42934_d_n8, assign38050_e42934_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38050_e42932: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign38050_e42932, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign38050_e42934;
        locals.var_q_aexp__blk824_dn4 = assign38050_e42934_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign38050_e42934_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign38050_e42934_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign38050_e42934_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign38050_e42934_d_n9;

        let (assign38060_e42942, assign38060_e42942_d_n4, assign38060_e42942_d_n6, assign38060_e42942_d_n7, assign38060_e42942_d_n8, assign38060_e42942_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38060_e42938: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign38060_e42940: f64 = (assign38060_e42938 - locals.var_q_aexp__blk824);
        (assign38060_e42940, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign38060_e42942;
        locals.var_q_qsq__blk825_dn4 = assign38060_e42942_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign38060_e42942_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign38060_e42942_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign38060_e42942_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign38060_e42942_d_n9;

        let (assign38070_e42952, assign38070_e42952_d_n4, assign38070_e42952_d_n6, assign38070_e42952_d_n7, assign38070_e42952_d_n8, assign38070_e42952_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38070_e42946: f64 = (2.0 * locals.var_k1__blk932);
        let assign38070_e42948: f64 = (assign38070_e42946 * locals.var_q_k1q1__blk823);
        let assign38070_e42950: f64 = (assign38070_e42948 + locals.var_q_aexp__blk824);
        (assign38070_e42950, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign38070_e42946 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign38070_e42946 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign38070_e42946 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign38070_e42946 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign38070_e42946 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign38070_e42952;
        locals.var_q_d1_qsq__blk826_dn4 = assign38070_e42952_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign38070_e42952_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign38070_e42952_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign38070_e42952_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign38070_e42952_d_n9;

        let (assign38080_e42962, assign38080_e42962_d_n4, assign38080_e42962_d_n6, assign38080_e42962_d_n7, assign38080_e42962_d_n8, assign38080_e42962_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38080_e42956: f64 = (2.0 * locals.var_k1__blk932);
        let assign38080_e42958: f64 = (assign38080_e42956 * locals.var_k1__blk932);
        let assign38080_e42960: f64 = (assign38080_e42958 - locals.var_q_aexp__blk824);
        (assign38080_e42960, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign38080_e42956 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign38080_e42956 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign38080_e42956 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign38080_e42956 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign38080_e42956 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign38080_e42962;
        locals.var_q_d2_qsq__blk827_dn4 = assign38080_e42962_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign38080_e42962_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign38080_e42962_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign38080_e42962_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign38080_e42962_d_n9;

        let assign38090_e42965: f64 = (-0.005);
        let assign38090_e42966: f64 = if locals.var_q_qsq__blk825 < assign38090_e42965 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign38090_e42966;

        let (assign38100_e42974, assign38100_e42974_d_n4, assign38100_e42974_d_n6, assign38100_e42974_d_n7, assign38100_e42974_d_n8, assign38100_e42974_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38100_e42971: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38100_e42972: f64 = (assign38100_e42971).sqrt();
        (assign38100_e42972, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38100_e42972)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38100_e42972)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38100_e42972)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38100_e42972)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38100_e42972)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38100_e42974;
        locals.var_q_rac_qsq__blk828_dn4 = assign38100_e42974_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38100_e42974_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38100_e42974_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38100_e42974_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38100_e42974_d_n9;

        let (assign38110_e42985, assign38110_e42985_d_n4, assign38110_e42985_d_n6, assign38110_e42985_d_n7, assign38110_e42985_d_n8, assign38110_e42985_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38110_e42981: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign38110_e42982: f64 = (assign38110_e42981).tan();
        let assign38110_e42983: f64 = (locals.var_q_rac_qsq__blk828 / assign38110_e42982);
        (assign38110_e42983, (((locals.var_q_rac_qsq__blk828_dn4 * assign38110_e42982) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38110_e42981).cos() * (assign38110_e42981).cos())))) / (assign38110_e42982 * assign38110_e42982)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38110_e42982) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38110_e42981).cos() * (assign38110_e42981).cos())))) / (assign38110_e42982 * assign38110_e42982)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38110_e42982) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38110_e42981).cos() * (assign38110_e42981).cos())))) / (assign38110_e42982 * assign38110_e42982)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38110_e42982) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38110_e42981).cos() * (assign38110_e42981).cos())))) / (assign38110_e42982 * assign38110_e42982)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38110_e42982) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38110_e42981).cos() * (assign38110_e42981).cos())))) / (assign38110_e42982 * assign38110_e42982)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38110_e42985;
        locals.var_q_qcoth__blk829_dn4 = assign38110_e42985_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38110_e42985_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38110_e42985_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38110_e42985_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38110_e42985_d_n9;

        let (assign38120_e42995, assign38120_e42995_d_n4, assign38120_e42995_d_n6, assign38120_e42995_d_n7, assign38120_e42995_d_n8, assign38120_e42995_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38120_e42991: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38120_e42993: f64 = (assign38120_e42991 / locals.var_q_qsq__blk825);
        (assign38120_e42993, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38120_e42991 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38120_e42991 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38120_e42991 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38120_e42991 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38120_e42991 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38120_e42995;
        locals.var_q_temp1__blk814_dn4 = assign38120_e42995_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38120_e42995_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38120_e42995_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38120_e42995_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38120_e42995_d_n9;

        let (assign38130_e43009, assign38130_e43009_d_n4, assign38130_e43009_d_n6, assign38130_e43009_d_n7, assign38130_e43009_d_n8, assign38130_e43009_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38130_e43003: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38130_e43004: f64 = (locals.var_q_qcoth__blk829 * assign38130_e43003);
        let assign38130_e43005: f64 = (locals.var_q_qsq__blk825 + assign38130_e43004);
        let assign38130_e43007: f64 = (assign38130_e43005 * locals.var_q_temp1__blk814);
        (assign38130_e43007, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38130_e43003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38130_e43005 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38130_e43003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38130_e43005 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38130_e43003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38130_e43005 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38130_e43003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38130_e43005 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38130_e43003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38130_e43005 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38130_e43009;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38130_e43009_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38130_e43009_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38130_e43009_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38130_e43009_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38130_e43009_d_n9;

        let (assign38140_e43031, assign38140_e43031_d_n4, assign38140_e43031_d_n6, assign38140_e43031_d_n7, assign38140_e43031_d_n8, assign38140_e43031_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38140_e43016: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38140_e43019: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38140_e43020: f64 = (assign38140_e43016 * assign38140_e43019);
        let assign38140_e43021: f64 = (locals.var_q_d1_qsq__blk826 - assign38140_e43020);
        let assign38140_e43023: f64 = (assign38140_e43021 * locals.var_q_temp1__blk814);
        let assign38140_e43026: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38140_e43028: f64 = (assign38140_e43026 / locals.var_q_d1_qsq__blk826);
        let assign38140_e43029: f64 = (assign38140_e43023 + assign38140_e43028);
        (assign38140_e43029, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38140_e43019) + (assign38140_e43016 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38140_e43021 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43026 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38140_e43019) + (assign38140_e43016 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38140_e43021 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43026 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38140_e43019) + (assign38140_e43016 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38140_e43021 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43026 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38140_e43019) + (assign38140_e43016 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38140_e43021 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43026 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38140_e43019) + (assign38140_e43016 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38140_e43021 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38140_e43026 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38140_e43031;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38140_e43031_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38140_e43031_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38140_e43031_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38140_e43031_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38140_e43031_d_n9;

        let (assign38150_e43041, assign38150_e43041_d_n4, assign38150_e43041_d_n6, assign38150_e43041_d_n7, assign38150_e43041_d_n8, assign38150_e43041_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38150_e43038: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38150_e43039: f64 = (1.0 - assign38150_e43038);
        (assign38150_e43039, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38150_e43041;
        locals.var_q_temp2__blk815_dn4 = assign38150_e43041_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38150_e43041_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38150_e43041_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38150_e43041_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38150_e43041_d_n9;

        let (assign38160_e43051, assign38160_e43051_d_n4, assign38160_e43051_d_n6, assign38160_e43051_d_n7, assign38160_e43051_d_n8, assign38160_e43051_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38160_e43047: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38160_e43049: f64 = (assign38160_e43047 * locals.var_q_temp2__blk815);
        (assign38160_e43049, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43047 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43047 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43047 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43047 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38160_e43047 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38160_e43051;
        locals.var_q_d1_ln__blk835_dn4 = assign38160_e43051_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38160_e43051_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38160_e43051_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38160_e43051_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38160_e43051_d_n9;

        let (assign38170_e43069, assign38170_e43069_d_n4, assign38170_e43069_d_n6, assign38170_e43069_d_n7, assign38170_e43069_d_n8, assign38170_e43069_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1190 != 0.0)) {
        let assign38170_e43057: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38170_e43062: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38170_e43063: f64 = (locals.var_q_d1_ln__blk835 + assign38170_e43062);
        let assign38170_e43064: f64 = (locals.var_q_d1_qsq__blk826 * assign38170_e43063);
        let assign38170_e43065: f64 = (assign38170_e43057 - assign38170_e43064);
        let assign38170_e43067: f64 = (assign38170_e43065 / locals.var_q_qsq__blk825);
        (assign38170_e43067, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38170_e43063) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38170_e43065 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38170_e43063) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38170_e43065 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38170_e43063) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38170_e43065 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38170_e43063) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38170_e43065 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38170_e43063) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38170_e43065 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38170_e43069;
        locals.var_q_d2_ln__blk836_dn4 = assign38170_e43069_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38170_e43069_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38170_e43069_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38170_e43069_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38170_e43069_d_n9;

        let assign38180_e43072: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign38180_e43072;

        let (assign38190_e43083, assign38190_e43083_d_n4, assign38190_e43083_d_n6, assign38190_e43083_d_n7, assign38190_e43083_d_n8, assign38190_e43083_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38190_e43080: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38190_e43081: f64 = (assign38190_e43080).sqrt();
        (assign38190_e43081, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38190_e43081)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38190_e43081)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38190_e43081)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38190_e43081)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38190_e43081)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38190_e43083;
        locals.var_q_rac_qsq__blk828_dn4 = assign38190_e43083_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38190_e43083_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38190_e43083_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38190_e43083_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38190_e43083_d_n9;

        let (assign38200_e43094, assign38200_e43094_d_n4, assign38200_e43094_d_n6, assign38200_e43094_d_n7, assign38200_e43094_d_n8, assign38200_e43094_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38200_e43091: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign38200_e43092: f64 = (assign38200_e43091).exp();
        (assign38200_e43092, (assign38200_e43092 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38200_e43092 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38200_e43092 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38200_e43092 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38200_e43092 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign38200_e43094;
        locals.var_q_invexpq__blk831_dn4 = assign38200_e43094_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign38200_e43094_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign38200_e43094_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign38200_e43094_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign38200_e43094_d_n9;

        let (assign38210_e43111, assign38210_e43111_d_n4, assign38210_e43111_d_n6, assign38210_e43111_d_n7, assign38210_e43111_d_n8, assign38210_e43111_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38210_e43104: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign38210_e43105: f64 = (locals.var_q_rac_qsq__blk828 * assign38210_e43104);
        let assign38210_e43108: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign38210_e43109: f64 = (assign38210_e43105 / assign38210_e43108);
        (assign38210_e43109, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38210_e43104) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38210_e43108) - (assign38210_e43105 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38210_e43108 * assign38210_e43108)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38210_e43104) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38210_e43108) - (assign38210_e43105 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38210_e43108 * assign38210_e43108)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38210_e43104) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38210_e43108) - (assign38210_e43105 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38210_e43108 * assign38210_e43108)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38210_e43104) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38210_e43108) - (assign38210_e43105 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38210_e43108 * assign38210_e43108)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38210_e43104) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38210_e43108) - (assign38210_e43105 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38210_e43108 * assign38210_e43108)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38210_e43111;
        locals.var_q_qcoth__blk829_dn4 = assign38210_e43111_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38210_e43111_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38210_e43111_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38210_e43111_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38210_e43111_d_n9;

    }

    pub(super) fn stamp_transient_block_103(
        locals: &mut StampLocals,
    ) {
        let (assign38220_e43124, assign38220_e43124_d_n4, assign38220_e43124_d_n6, assign38220_e43124_d_n7, assign38220_e43124_d_n8, assign38220_e43124_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38220_e43120: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38220_e43122: f64 = (assign38220_e43120 / locals.var_q_qsq__blk825);
        (assign38220_e43122, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38220_e43120 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38220_e43120 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38220_e43120 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38220_e43120 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38220_e43120 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38220_e43124;
        locals.var_q_temp1__blk814_dn4 = assign38220_e43124_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38220_e43124_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38220_e43124_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38220_e43124_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38220_e43124_d_n9;

        let (assign38230_e43141, assign38230_e43141_d_n4, assign38230_e43141_d_n6, assign38230_e43141_d_n7, assign38230_e43141_d_n8, assign38230_e43141_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38230_e43135: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38230_e43136: f64 = (locals.var_q_qcoth__blk829 * assign38230_e43135);
        let assign38230_e43137: f64 = (locals.var_q_qsq__blk825 + assign38230_e43136);
        let assign38230_e43139: f64 = (assign38230_e43137 * locals.var_q_temp1__blk814);
        (assign38230_e43139, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38230_e43135) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38230_e43137 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38230_e43135) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38230_e43137 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38230_e43135) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38230_e43137 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38230_e43135) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38230_e43137 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38230_e43135) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38230_e43137 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38230_e43141;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38230_e43141_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38230_e43141_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38230_e43141_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38230_e43141_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38230_e43141_d_n9;

        let (assign38240_e43166, assign38240_e43166_d_n4, assign38240_e43166_d_n6, assign38240_e43166_d_n7, assign38240_e43166_d_n8, assign38240_e43166_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38240_e43151: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38240_e43154: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38240_e43155: f64 = (assign38240_e43151 * assign38240_e43154);
        let assign38240_e43156: f64 = (locals.var_q_d1_qsq__blk826 - assign38240_e43155);
        let assign38240_e43158: f64 = (assign38240_e43156 * locals.var_q_temp1__blk814);
        let assign38240_e43161: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38240_e43163: f64 = (assign38240_e43161 / locals.var_q_d1_qsq__blk826);
        let assign38240_e43164: f64 = (assign38240_e43158 + assign38240_e43163);
        (assign38240_e43164, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38240_e43154) + (assign38240_e43151 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38240_e43156 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38240_e43161 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38240_e43154) + (assign38240_e43151 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38240_e43156 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38240_e43161 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38240_e43154) + (assign38240_e43151 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38240_e43156 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38240_e43161 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38240_e43154) + (assign38240_e43151 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38240_e43156 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38240_e43161 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38240_e43154) + (assign38240_e43151 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38240_e43156 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38240_e43161 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38240_e43166;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38240_e43166_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38240_e43166_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38240_e43166_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38240_e43166_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38240_e43166_d_n9;

        let (assign38250_e43179, assign38250_e43179_d_n4, assign38250_e43179_d_n6, assign38250_e43179_d_n7, assign38250_e43179_d_n8, assign38250_e43179_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38250_e43176: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38250_e43177: f64 = (1.0 - assign38250_e43176);
        (assign38250_e43177, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38250_e43179;
        locals.var_q_temp2__blk815_dn4 = assign38250_e43179_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38250_e43179_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38250_e43179_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38250_e43179_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38250_e43179_d_n9;

        let (assign38260_e43192, assign38260_e43192_d_n4, assign38260_e43192_d_n6, assign38260_e43192_d_n7, assign38260_e43192_d_n8, assign38260_e43192_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38260_e43188: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38260_e43190: f64 = (assign38260_e43188 * locals.var_q_temp2__blk815);
        (assign38260_e43190, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38260_e43188 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38260_e43188 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38260_e43188 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38260_e43188 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38260_e43188 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38260_e43192;
        locals.var_q_d1_ln__blk835_dn4 = assign38260_e43192_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38260_e43192_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38260_e43192_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38260_e43192_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38260_e43192_d_n9;

        let (assign38270_e43213, assign38270_e43213_d_n4, assign38270_e43213_d_n6, assign38270_e43213_d_n7, assign38270_e43213_d_n8, assign38270_e43213_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 != 0.0)) {
        let assign38270_e43201: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38270_e43206: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38270_e43207: f64 = (locals.var_q_d1_ln__blk835 + assign38270_e43206);
        let assign38270_e43208: f64 = (locals.var_q_d1_qsq__blk826 * assign38270_e43207);
        let assign38270_e43209: f64 = (assign38270_e43201 - assign38270_e43208);
        let assign38270_e43211: f64 = (assign38270_e43209 / locals.var_q_qsq__blk825);
        (assign38270_e43211, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38270_e43207) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38270_e43209 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38270_e43207) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38270_e43209 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38270_e43207) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38270_e43209 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38270_e43207) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38270_e43209 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38270_e43207) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38270_e43209 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38270_e43213;
        locals.var_q_d2_ln__blk836_dn4 = assign38270_e43213_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38270_e43213_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38270_e43213_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38270_e43213_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38270_e43213_d_n9;

        let (assign38280_e43241, assign38280_e43241_d_n4, assign38280_e43241_d_n6, assign38280_e43241_d_n7, assign38280_e43241_d_n8, assign38280_e43241_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38280_e43225: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign38280_e43229: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign38280_e43233: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign38280_e43234: f64 = (1.0 - assign38280_e43233);
        let assign38280_e43235: f64 = (assign38280_e43229 * assign38280_e43234);
        let assign38280_e43236: f64 = (1.0 - assign38280_e43235);
        let assign38280_e43237: f64 = (assign38280_e43225 * assign38280_e43236);
        let assign38280_e43238: f64 = (1.0 - assign38280_e43237);
        let assign38280_e43239: f64 = (0.1666666666667 * assign38280_e43238);
        (assign38280_e43239, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign38280_e43236) + (assign38280_e43225 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38280_e43234) + (assign38280_e43229 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign38280_e43236) + (assign38280_e43225 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38280_e43234) + (assign38280_e43229 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign38280_e43236) + (assign38280_e43225 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38280_e43234) + (assign38280_e43229 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign38280_e43236) + (assign38280_e43225 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38280_e43234) + (assign38280_e43229 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign38280_e43236) + (assign38280_e43225 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38280_e43234) + (assign38280_e43229 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign38280_e43241;
        locals.var_q_temp3__blk816_dn4 = assign38280_e43241_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign38280_e43241_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign38280_e43241_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign38280_e43241_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign38280_e43241_d_n9;

        let (assign38290_e43255, assign38290_e43255_d_n4, assign38290_e43255_d_n6, assign38290_e43255_d_n7, assign38290_e43255_d_n8, assign38290_e43255_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38290_e43252: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign38290_e43253: f64 = (2.0 + assign38290_e43252);
        (assign38290_e43253, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38290_e43255;
        locals.var_q_qcoth__blk829_dn4 = assign38290_e43255_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38290_e43255_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38290_e43255_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38290_e43255_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38290_e43255_d_n9;

        let (assign38300_e43283, assign38300_e43283_d_n4, assign38300_e43283_d_n6, assign38300_e43283_d_n7, assign38300_e43283_d_n8, assign38300_e43283_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38300_e43267: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign38300_e43271: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign38300_e43275: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign38300_e43276: f64 = (1.0 - assign38300_e43275);
        let assign38300_e43277: f64 = (assign38300_e43271 * assign38300_e43276);
        let assign38300_e43278: f64 = (1.0 - assign38300_e43277);
        let assign38300_e43279: f64 = (assign38300_e43267 * assign38300_e43278);
        let assign38300_e43280: f64 = (1.0 - assign38300_e43279);
        let assign38300_e43281: f64 = (0.1666666666667 * assign38300_e43280);
        (assign38300_e43281, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign38300_e43278) + (assign38300_e43267 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign38300_e43276) + (assign38300_e43271 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign38300_e43278) + (assign38300_e43267 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign38300_e43276) + (assign38300_e43271 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign38300_e43278) + (assign38300_e43267 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign38300_e43276) + (assign38300_e43271 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign38300_e43278) + (assign38300_e43267 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign38300_e43276) + (assign38300_e43271 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign38300_e43278) + (assign38300_e43267 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign38300_e43276) + (assign38300_e43271 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38300_e43283;
        locals.var_q_temp1__blk814_dn4 = assign38300_e43283_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38300_e43283_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38300_e43283_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38300_e43283_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38300_e43283_d_n9;

        let (assign38310_e43295, assign38310_e43295_d_n4, assign38310_e43295_d_n6, assign38310_e43295_d_n7, assign38310_e43295_d_n8, assign38310_e43295_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38310_e43293: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign38310_e43293, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38310_e43295;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38310_e43295_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38310_e43295_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38310_e43295_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38310_e43295_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38310_e43295_d_n9;

        let (assign38320_e43323, assign38320_e43323_d_n4, assign38320_e43323_d_n6, assign38320_e43323_d_n7, assign38320_e43323_d_n8, assign38320_e43323_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38320_e43307: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign38320_e43311: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign38320_e43315: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign38320_e43316: f64 = (1.0 - assign38320_e43315);
        let assign38320_e43317: f64 = (assign38320_e43311 * assign38320_e43316);
        let assign38320_e43318: f64 = (1.0 - assign38320_e43317);
        let assign38320_e43319: f64 = (assign38320_e43307 * assign38320_e43318);
        let assign38320_e43320: f64 = (1.0 - assign38320_e43319);
        let assign38320_e43321: f64 = (0.0055555555556 * assign38320_e43320);
        (assign38320_e43321, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign38320_e43318) + (assign38320_e43307 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38320_e43316) + (assign38320_e43311 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign38320_e43318) + (assign38320_e43307 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38320_e43316) + (assign38320_e43311 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign38320_e43318) + (assign38320_e43307 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38320_e43316) + (assign38320_e43311 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign38320_e43318) + (assign38320_e43307 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38320_e43316) + (assign38320_e43311 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign38320_e43318) + (assign38320_e43307 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38320_e43316) + (assign38320_e43311 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38320_e43323;
        locals.var_q_temp2__blk815_dn4 = assign38320_e43323_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38320_e43323_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38320_e43323_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38320_e43323_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38320_e43323_d_n9;

        let (assign38330_e43341, assign38330_e43341_d_n4, assign38330_e43341_d_n6, assign38330_e43341_d_n7, assign38330_e43341_d_n8, assign38330_e43341_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38330_e43333: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign38330_e43336: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign38330_e43338: f64 = (assign38330_e43336 * locals.var_q_temp2__blk815);
        let assign38330_e43339: f64 = (assign38330_e43333 - assign38330_e43338);
        (assign38330_e43339, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign38330_e43336 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign38330_e43336 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign38330_e43336 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign38330_e43336 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign38330_e43336 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38330_e43341;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38330_e43341_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38330_e43341_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38330_e43341_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38330_e43341_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38330_e43341_d_n9;

        let (assign38340_e43356, assign38340_e43356_d_n4, assign38340_e43356_d_n6, assign38340_e43356_d_n7, assign38340_e43356_d_n8, assign38340_e43356_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38340_e43350: f64 = (-0.5);
        let assign38340_e43352: f64 = (assign38340_e43350 * locals.var_q_d1_qsq__blk826);
        let assign38340_e43354: f64 = (assign38340_e43352 * locals.var_q_temp3__blk816);
        (assign38340_e43354, (((assign38340_e43350 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign38340_e43352 * locals.var_q_temp3__blk816_dn4)), (((assign38340_e43350 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign38340_e43352 * locals.var_q_temp3__blk816_dn6)), (((assign38340_e43350 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign38340_e43352 * locals.var_q_temp3__blk816_dn7)), (((assign38340_e43350 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign38340_e43352 * locals.var_q_temp3__blk816_dn8)), (((assign38340_e43350 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign38340_e43352 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38340_e43356;
        locals.var_q_d1_ln__blk835_dn4 = assign38340_e43356_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38340_e43356_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38340_e43356_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38340_e43356_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38340_e43356_d_n9;

        let (assign38350_e43391, assign38350_e43391_d_n4, assign38350_e43391_d_n6, assign38350_e43391_d_n7, assign38350_e43391_d_n8, assign38350_e43391_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1190 == 0.0)) && (locals.var_guard1191 == 0.0)) {
        let assign38350_e43365: f64 = (-0.5);
        let assign38350_e43367: f64 = (assign38350_e43365 * locals.var_q_d2_qsq__blk827);
        let assign38350_e43369: f64 = (assign38350_e43367 * locals.var_q_temp3__blk816);
        let assign38350_e43372: f64 = (0.25 * 0.0055555555556);
        let assign38350_e43374: f64 = (assign38350_e43372 * locals.var_q_d1_qsq__blk826);
        let assign38350_e43376: f64 = (assign38350_e43374 * locals.var_q_d1_qsq__blk826);
        let assign38350_e43380: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign38350_e43384: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign38350_e43385: f64 = (2.0 - assign38350_e43384);
        let assign38350_e43386: f64 = (assign38350_e43380 * assign38350_e43385);
        let assign38350_e43387: f64 = (1.0 - assign38350_e43386);
        let assign38350_e43388: f64 = (assign38350_e43376 * assign38350_e43387);
        let assign38350_e43389: f64 = (assign38350_e43369 + assign38350_e43388);
        (assign38350_e43389, ((((assign38350_e43365 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign38350_e43367 * locals.var_q_temp3__blk816_dn4)) + (((((assign38350_e43372 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign38350_e43374 * locals.var_q_d1_qsq__blk826_dn4)) * assign38350_e43387) + (assign38350_e43376 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign38350_e43385) + (assign38350_e43380 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign38350_e43365 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign38350_e43367 * locals.var_q_temp3__blk816_dn6)) + (((((assign38350_e43372 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign38350_e43374 * locals.var_q_d1_qsq__blk826_dn6)) * assign38350_e43387) + (assign38350_e43376 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign38350_e43385) + (assign38350_e43380 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign38350_e43365 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign38350_e43367 * locals.var_q_temp3__blk816_dn7)) + (((((assign38350_e43372 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign38350_e43374 * locals.var_q_d1_qsq__blk826_dn7)) * assign38350_e43387) + (assign38350_e43376 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign38350_e43385) + (assign38350_e43380 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign38350_e43365 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign38350_e43367 * locals.var_q_temp3__blk816_dn8)) + (((((assign38350_e43372 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign38350_e43374 * locals.var_q_d1_qsq__blk826_dn8)) * assign38350_e43387) + (assign38350_e43376 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign38350_e43385) + (assign38350_e43380 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign38350_e43365 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign38350_e43367 * locals.var_q_temp3__blk816_dn9)) + (((((assign38350_e43372 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign38350_e43374 * locals.var_q_d1_qsq__blk826_dn9)) * assign38350_e43387) + (assign38350_e43376 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign38350_e43385) + (assign38350_e43380 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38350_e43391;
        locals.var_q_d2_ln__blk836_dn4 = assign38350_e43391_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38350_e43391_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38350_e43391_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38350_e43391_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38350_e43391_d_n9;

        let assign38360_e43394: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign38360_e43394;

        let (assign38370_e43410, assign38370_e43410_d_n4, assign38370_e43410_d_n6, assign38370_e43410_d_n7, assign38370_e43410_d_n8, assign38370_e43410_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign38370_e43400: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign38370_e43405: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign38370_e43406: f64 = (locals.var_q_invexpq__blk831 * assign38370_e43405);
        let assign38370_e43407: f64 = (1.0 - assign38370_e43406);
        let assign38370_e43408: f64 = (assign38370_e43400 / assign38370_e43407);
        (assign38370_e43408, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign38370_e43407) - (assign38370_e43400 * (-((locals.var_q_invexpq__blk831_dn4 * assign38370_e43405) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign38370_e43407 * assign38370_e43407)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign38370_e43407) - (assign38370_e43400 * (-((locals.var_q_invexpq__blk831_dn6 * assign38370_e43405) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign38370_e43407 * assign38370_e43407)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign38370_e43407) - (assign38370_e43400 * (-((locals.var_q_invexpq__blk831_dn7 * assign38370_e43405) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign38370_e43407 * assign38370_e43407)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign38370_e43407) - (assign38370_e43400 * (-((locals.var_q_invexpq__blk831_dn8 * assign38370_e43405) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign38370_e43407 * assign38370_e43407)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign38370_e43407) - (assign38370_e43400 * (-((locals.var_q_invexpq__blk831_dn9 * assign38370_e43405) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign38370_e43407 * assign38370_e43407)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38370_e43410;
        locals.var_q_temp2__blk815_dn4 = assign38370_e43410_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38370_e43410_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38370_e43410_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38370_e43410_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38370_e43410_d_n9;

        let (assign38380_e43418, assign38380_e43418_d_n4, assign38380_e43418_d_n6, assign38380_e43418_d_n7, assign38380_e43418_d_n8, assign38380_e43418_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign38380_e43416: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign38380_e43416, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign38380_e43418;
        locals.var_q_sh_term__blk833_dn4 = assign38380_e43418_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign38380_e43418_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign38380_e43418_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign38380_e43418_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign38380_e43418_d_n9;

        let (assign38390_e43427, assign38390_e43427_d_n4, assign38390_e43427_d_n6, assign38390_e43427_d_n7, assign38390_e43427_d_n8, assign38390_e43427_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1192 != 0.0)) {
        let assign38390_e43423: f64 = (locals.var_q_temp2__blk815).ln();
        let assign38390_e43425: f64 = (assign38390_e43423 - locals.var_q_rac_qsq__blk828);
        (assign38390_e43425, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign38390_e43427;
        locals.var_q_ln_term__blk834_dn4 = assign38390_e43427_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign38390_e43427_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign38390_e43427_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign38390_e43427_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign38390_e43427_d_n9;

        let assign38400_e43430: f64 = (-0.005);
        let assign38400_e43431: f64 = if locals.var_q_qsq__blk825 < assign38400_e43430 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign38400_e43431;

        let (assign38410_e43443, assign38410_e43443_d_n4, assign38410_e43443_d_n6, assign38410_e43443_d_n7, assign38410_e43443_d_n8, assign38410_e43443_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign38410_e43440: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign38410_e43441: f64 = (assign38410_e43440).sin();
        (assign38410_e43441, ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign38410_e43440).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38410_e43443;
        locals.var_q_temp2__blk815_dn4 = assign38410_e43443_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38410_e43443_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38410_e43443_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38410_e43443_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38410_e43443_d_n9;

        let (assign38420_e43457, assign38420_e43457_d_n4, assign38420_e43457_d_n6, assign38420_e43457_d_n7, assign38420_e43457_d_n8, assign38420_e43457_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign38420_e43451: f64 = (-locals.var_q_qsq__blk825);
        let assign38420_e43454: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign38420_e43455: f64 = (assign38420_e43451 / assign38420_e43454);
        (assign38420_e43455, ((((-locals.var_q_qsq__blk825_dn4) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn6) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn7) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn8) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign38420_e43454 * assign38420_e43454)), ((((-locals.var_q_qsq__blk825_dn9) * assign38420_e43454) - (assign38420_e43451 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign38420_e43454 * assign38420_e43454)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign38420_e43457;
        locals.var_q_sh_term__blk833_dn4 = assign38420_e43457_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign38420_e43457_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign38420_e43457_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign38420_e43457_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign38420_e43457_d_n9;

        let (assign38430_e43467, assign38430_e43467_d_n4, assign38430_e43467_d_n6, assign38430_e43467_d_n7, assign38430_e43467_d_n8, assign38430_e43467_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 != 0.0)) {
        let assign38430_e43465: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign38430_e43465, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign38430_e43467;
        locals.var_q_ln_term__blk834_dn4 = assign38430_e43467_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign38430_e43467_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign38430_e43467_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign38430_e43467_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign38430_e43467_d_n9;

        let (assign38440_e43493, assign38440_e43493_d_n4, assign38440_e43493_d_n6, assign38440_e43493_d_n7, assign38440_e43493_d_n8, assign38440_e43493_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
        let assign38440_e43478: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign38440_e43482: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign38440_e43486: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign38440_e43487: f64 = (1.0 - assign38440_e43486);
        let assign38440_e43488: f64 = (assign38440_e43482 * assign38440_e43487);
        let assign38440_e43489: f64 = (1.0 - assign38440_e43488);
        let assign38440_e43490: f64 = (assign38440_e43478 * assign38440_e43489);
        let assign38440_e43491: f64 = (4.0 - assign38440_e43490);
        (assign38440_e43491, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign38440_e43489) + (assign38440_e43478 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign38440_e43487) + (assign38440_e43482 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign38440_e43493;
        locals.var_q_sh_term__blk833_dn4 = assign38440_e43493_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign38440_e43493_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign38440_e43493_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign38440_e43493_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign38440_e43493_d_n9;

        let (assign38450_e43504, assign38450_e43504_d_n4, assign38450_e43504_d_n6, assign38450_e43504_d_n7, assign38450_e43504_d_n8, assign38450_e43504_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1192 == 0.0)) && (locals.var_guard1193 == 0.0)) {
        let assign38450_e43502: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign38450_e43502, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign38450_e43504;
        locals.var_q_ln_term__blk834_dn4 = assign38450_e43504_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign38450_e43504_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign38450_e43504_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign38450_e43504_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign38450_e43504_d_n9;

        let assign38460_e43507: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign38460_e43509: f64 = (assign38460_e43507 + locals.var_q_qcoth__blk829);
        let assign38460_e43511: f64 = if assign38460_e43509 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign38460_e43511;

        let (assign38470_e43519, assign38470_e43519_d_n4, assign38470_e43519_d_n6, assign38470_e43519_d_n7, assign38470_e43519_d_n8, assign38470_e43519_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
        let assign38470_e43517: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign38470_e43517, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign38470_e43519;
        locals.var_q_expnum__blk837_dn4 = assign38470_e43519_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign38470_e43519_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign38470_e43519_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign38470_e43519_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign38470_e43519_d_n9;

        let (assign38480_e43527, assign38480_e43527_d_n4, assign38480_e43527_d_n6, assign38480_e43527_d_n7, assign38480_e43527_d_n8, assign38480_e43527_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
        let assign38480_e43525: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign38480_e43525, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign38480_e43527;
        locals.var_q_d1_expnum__blk838_dn4 = assign38480_e43527_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign38480_e43527_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign38480_e43527_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign38480_e43527_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign38480_e43527_d_n9;

        let (assign38490_e43533, assign38490_e43533_d_n4, assign38490_e43533_d_n6, assign38490_e43533_d_n7, assign38490_e43533_d_n8, assign38490_e43533_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign38490_e43533;
        locals.var_q_d2_expnum__blk839_dn4 = assign38490_e43533_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign38490_e43533_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign38490_e43533_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign38490_e43533_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign38490_e43533_d_n9;

        let (assign38500_e43544, assign38500_e43544_d_n4, assign38500_e43544_d_n6, assign38500_e43544_d_n7, assign38500_e43544_d_n8, assign38500_e43544_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38500_e43541: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign38500_e43542: f64 = (1.0 / assign38500_e43541);
        (assign38500_e43542, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign38500_e43541 * assign38500_e43541))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign38500_e43541 * assign38500_e43541))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38500_e43544;
        locals.var_q_temp2__blk815_dn4 = assign38500_e43544_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38500_e43544_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38500_e43544_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38500_e43544_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38500_e43544_d_n9;

        let (assign38510_e43553, assign38510_e43553_d_n4, assign38510_e43553_d_n6, assign38510_e43553_d_n7, assign38510_e43553_d_n8, assign38510_e43553_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38510_e43551: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign38510_e43551, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign38510_e43553;
        locals.var_q_temp3__blk816_dn4 = assign38510_e43553_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign38510_e43553_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign38510_e43553_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign38510_e43553_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign38510_e43553_d_n9;

        let (assign38520_e43564, assign38520_e43564_d_n4, assign38520_e43564_d_n6, assign38520_e43564_d_n7, assign38520_e43564_d_n8, assign38520_e43564_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38520_e43560: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign38520_e43562: f64 = (assign38520_e43560 * locals.var_q_temp2__blk815);
        (assign38520_e43562, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign38520_e43560 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign38520_e43564;
        locals.var_q_expnum__blk837_dn4 = assign38520_e43564_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign38520_e43564_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign38520_e43564_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign38520_e43564_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign38520_e43564_d_n9;

    }

    pub(super) fn stamp_transient_block_104(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign38530_e43581, assign38530_e43581_d_n4, assign38530_e43581_d_n6, assign38530_e43581_d_n7, assign38530_e43581_d_n8, assign38530_e43581_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38530_e43571: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign38530_e43573: f64 = (assign38530_e43571 - locals.var_q_aexp__blk824);
        let assign38530_e43576: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign38530_e43577: f64 = (assign38530_e43573 - assign38530_e43576);
        let assign38530_e43579: f64 = (assign38530_e43577 * locals.var_q_temp2__blk815);
        (assign38530_e43579, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38530_e43577 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign38530_e43581;
        locals.var_q_d1_expnum__blk838_dn4 = assign38530_e43581_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign38530_e43581_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign38530_e43581_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign38530_e43581_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign38530_e43581_d_n9;

        let (assign38540_e43608, assign38540_e43608_d_n4, assign38540_e43608_d_n6, assign38540_e43608_d_n7, assign38540_e43608_d_n8, assign38540_e43608_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1194 == 0.0)) {
        let assign38540_e43588: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign38540_e43591: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign38540_e43593: f64 = (assign38540_e43591 * locals.var_q_d1_expnum__blk838);
        let assign38540_e43594: f64 = (assign38540_e43588 + assign38540_e43593);
        let assign38540_e43596: f64 = (assign38540_e43594 + locals.var_q_aexp__blk824);
        let assign38540_e43600: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign38540_e43601: f64 = (locals.var_q_d2_ln__blk836 + assign38540_e43600);
        let assign38540_e43603: f64 = (assign38540_e43601 * locals.var_q_sh_term__blk833);
        let assign38540_e43604: f64 = (assign38540_e43596 - assign38540_e43603);
        let assign38540_e43606: f64 = (assign38540_e43604 * locals.var_q_temp2__blk815);
        (assign38540_e43606, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign38540_e43591 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign38540_e43601 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign38540_e43604 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign38540_e43608;
        locals.var_q_d2_expnum__blk839_dn4 = assign38540_e43608_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign38540_e43608_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign38540_e43608_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign38540_e43608_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign38540_e43608_d_n9;

        let assign38550_e43611: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign38550_e43611;

        let (assign38560_e43618, assign38560_e43618_d_n4, assign38560_e43618_d_n6, assign38560_e43618_d_n7, assign38560_e43618_d_n8, assign38560_e43618_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38560_e43616: f64 = (locals.var_q_expnum__blk837).ln();
        (assign38560_e43616, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign38560_e43618;
        locals.var_q_lnexpnum__blk840_dn4 = assign38560_e43618_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign38560_e43618_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign38560_e43618_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign38560_e43618_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign38560_e43618_d_n9;

        let (assign38570_e43626, assign38570_e43626_d_n4, assign38570_e43626_d_n6, assign38570_e43626_d_n7, assign38570_e43626_d_n8, assign38570_e43626_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38570_e43624: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign38570_e43624, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38570_e43626;
        locals.var_q_temp1__blk814_dn4 = assign38570_e43626_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38570_e43626_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38570_e43626_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38570_e43626_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38570_e43626_d_n9;

        let (assign38580_e43634, assign38580_e43634_d_n4, assign38580_e43634_d_n6, assign38580_e43634_d_n7, assign38580_e43634_d_n8, assign38580_e43634_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38580_e43632: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign38580_e43632, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign38580_e43634;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign38580_e43634_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign38580_e43634_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign38580_e43634_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign38580_e43634_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign38580_e43634_d_n9;

        let (assign38590_e43646, assign38590_e43646_d_n4, assign38590_e43646_d_n6, assign38590_e43646_d_n7, assign38590_e43646_d_n8, assign38590_e43646_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 != 0.0)) {
        let assign38590_e43640: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign38590_e43643: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign38590_e43644: f64 = (assign38590_e43640 - assign38590_e43643);
        (assign38590_e43644, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign38590_e43646;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign38590_e43646_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign38590_e43646_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign38590_e43646_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign38590_e43646_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign38590_e43646_d_n9;

        let (assign38600_e43659, assign38600_e43659_d_n4, assign38600_e43659_d_n6, assign38600_e43659_d_n7, assign38600_e43659_d_n8, assign38600_e43659_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38600_e43653: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign38600_e43655: f64 = (-locals.var_q_k1q1__blk823);
        let assign38600_e43656: f64 = (assign38600_e43655).ln();
        let assign38600_e43657: f64 = (assign38600_e43653 + assign38600_e43656);
        (assign38600_e43657, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign38600_e43655)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign38600_e43655)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign38600_e43659;
        locals.var_q_lnexpnum__blk840_dn4 = assign38600_e43659_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign38600_e43659_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign38600_e43659_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign38600_e43659_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign38600_e43659_d_n9;

        let (assign38610_e43668, assign38610_e43668_d_n4, assign38610_e43668_d_n6, assign38610_e43668_d_n7, assign38610_e43668_d_n8, assign38610_e43668_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38610_e43666: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign38610_e43666, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38610_e43668;
        locals.var_q_temp1__blk814_dn4 = assign38610_e43668_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38610_e43668_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38610_e43668_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38610_e43668_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38610_e43668_d_n9;

        let (assign38620_e43677, assign38620_e43677_d_n4, assign38620_e43677_d_n6, assign38620_e43677_d_n7, assign38620_e43677_d_n8, assign38620_e43677_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38620_e43675: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign38620_e43675, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign38620_e43677;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign38620_e43677_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign38620_e43677_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign38620_e43677_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign38620_e43677_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign38620_e43677_d_n9;

        let (assign38630_e43687, assign38630_e43687_d_n4, assign38630_e43687_d_n6, assign38630_e43687_d_n7, assign38630_e43687_d_n8, assign38630_e43687_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign38630_e43683: f64 = (-locals.var_q_temp1__blk814);
        let assign38630_e43685: f64 = (assign38630_e43683 * locals.var_q_temp1__blk814);
        (assign38630_e43685, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign38630_e43683 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign38630_e43687;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign38630_e43687_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign38630_e43687_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign38630_e43687_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign38630_e43687_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign38630_e43687_d_n9;

        let (assign38640_e43701, assign38640_e43701_d_n4, assign38640_e43701_d_n6, assign38640_e43701_d_n7, assign38640_e43701_d_n8, assign38640_e43701_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38640_e43691: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign38640_e43693: f64 = (assign38640_e43691 + locals.var_q1d__blk1001);
        let assign38640_e43696: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign38640_e43697: f64 = (assign38640_e43693 + assign38640_e43696);
        let assign38640_e43699: f64 = (assign38640_e43697 - locals.var_q_ln_term__blk834);
        (assign38640_e43699, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign38640_e43701;
        locals.var_q_q2_int__blk843_dn4 = assign38640_e43701_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign38640_e43701_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign38640_e43701_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign38640_e43701_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign38640_e43701_d_n9;

        let (assign38650_e43711, assign38650_e43711_d_n4, assign38650_e43711_d_n6, assign38650_e43711_d_n7, assign38650_e43711_d_n8, assign38650_e43711_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38650_e43706: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign38650_e43707: f64 = (1.0 + assign38650_e43706);
        let assign38650_e43709: f64 = (assign38650_e43707 - locals.var_q_d1_ln__blk835);
        (assign38650_e43709, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign38650_e43711;
        locals.var_q_d1_q2__blk844_dn4 = assign38650_e43711_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign38650_e43711_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign38650_e43711_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign38650_e43711_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign38650_e43711_d_n9;

        let (assign38660_e43719, assign38660_e43719_d_n4, assign38660_e43719_d_n6, assign38660_e43719_d_n7, assign38660_e43719_d_n8, assign38660_e43719_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38660_e43715: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign38660_e43717: f64 = (assign38660_e43715 - locals.var_q_d2_ln__blk836);
        (assign38660_e43717, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign38660_e43719;
        locals.var_q_d2_q2__blk845_dn4 = assign38660_e43719_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign38660_e43719_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign38660_e43719_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign38660_e43719_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign38660_e43719_d_n9;

        let (assign38670_e43727, assign38670_e43727_d_n4, assign38670_e43727_d_n6, assign38670_e43727_d_n7, assign38670_e43727_d_n8, assign38670_e43727_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38670_e43724: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign38670_e43725: f64 = (locals.var_q_k1q1__blk823 + assign38670_e43724);
        (assign38670_e43725, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign38670_e43727;
        locals.var_q_qi_int__blk846_dn4 = assign38670_e43727_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign38670_e43727_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign38670_e43727_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign38670_e43727_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign38670_e43727_d_n9;

        let (assign38680_e43735, assign38680_e43735_d_n4, assign38680_e43735_d_n6, assign38680_e43735_d_n7, assign38680_e43735_d_n8, assign38680_e43735_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38680_e43732: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign38680_e43733: f64 = (locals.var_k1__blk932 + assign38680_e43732);
        (assign38680_e43733, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign38680_e43735;
        locals.var_q_d1_qi__blk847_dn4 = assign38680_e43735_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign38680_e43735_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign38680_e43735_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign38680_e43735_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign38680_e43735_d_n9;

        let (assign38690_e43741, assign38690_e43741_d_n4, assign38690_e43741_d_n6, assign38690_e43741_d_n7, assign38690_e43741_d_n8, assign38690_e43741_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38690_e43739: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign38690_e43739, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign38690_e43741;
        locals.var_q_d2_qi__blk848_dn4 = assign38690_e43741_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign38690_e43741_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign38690_e43741_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign38690_e43741_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign38690_e43741_d_n9;

        let (assign38700_e43749, assign38700_e43749_d_n4, assign38700_e43749_d_n6, assign38700_e43749_d_n7, assign38700_e43749_d_n8, assign38700_e43749_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38700_e43745: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign38700_e43747: f64 = (assign38700_e43745 - locals.var_q_aexp__blk824);
        (assign38700_e43747, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign38700_e43749;
        locals.var_q_zero__blk849_dn4 = assign38700_e43749_d_n4;
        locals.var_q_zero__blk849_dn6 = assign38700_e43749_d_n6;
        locals.var_q_zero__blk849_dn7 = assign38700_e43749_d_n7;
        locals.var_q_zero__blk849_dn8 = assign38700_e43749_d_n8;
        locals.var_q_zero__blk849_dn9 = assign38700_e43749_d_n9;

        let (assign38710_e43761, assign38710_e43761_d_n4, assign38710_e43761_d_n6, assign38710_e43761_d_n7, assign38710_e43761_d_n8, assign38710_e43761_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38710_e43753: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign38710_e43756: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign38710_e43757: f64 = (assign38710_e43753 + assign38710_e43756);
        let assign38710_e43759: f64 = (assign38710_e43757 + locals.var_q_aexp__blk824);
        (assign38710_e43759, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign38710_e43761;
        locals.var_q_d1_zero__blk850_dn4 = assign38710_e43761_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign38710_e43761_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign38710_e43761_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign38710_e43761_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign38710_e43761_d_n9;

        let (assign38720_e43779, assign38720_e43779_d_n4, assign38720_e43779_d_n6, assign38720_e43779_d_n7, assign38720_e43779_d_n8, assign38720_e43779_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38720_e43765: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign38720_e43768: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign38720_e43770: f64 = (assign38720_e43768 * locals.var_q_d1_expnum__blk838);
        let assign38720_e43771: f64 = (assign38720_e43765 + assign38720_e43770);
        let assign38720_e43774: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign38720_e43775: f64 = (assign38720_e43771 + assign38720_e43774);
        let assign38720_e43777: f64 = (assign38720_e43775 - locals.var_q_aexp__blk824);
        (assign38720_e43777, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign38720_e43768 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign38720_e43779;
        locals.var_q_d2_zero__blk851_dn4 = assign38720_e43779_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign38720_e43779_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign38720_e43779_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign38720_e43779_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign38720_e43779_d_n9;

        let (assign38730_e43791, assign38730_e43791_d_n4, assign38730_e43791_d_n6, assign38730_e43791_d_n7, assign38730_e43791_d_n8, assign38730_e43791_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38730_e43783: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign38730_e43786: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign38730_e43788: f64 = (assign38730_e43786 * locals.var_q_d2_zero__blk851);
        let assign38730_e43789: f64 = (assign38730_e43783 - assign38730_e43788);
        (assign38730_e43789, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign38730_e43786 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign38730_e43791;
        locals.var_q_temp__blk860_dn4 = assign38730_e43791_d_n4;
        locals.var_q_temp__blk860_dn6 = assign38730_e43791_d_n6;
        locals.var_q_temp__blk860_dn7 = assign38730_e43791_d_n7;
        locals.var_q_temp__blk860_dn8 = assign38730_e43791_d_n8;
        locals.var_q_temp__blk860_dn9 = assign38730_e43791_d_n9;

        let (assign38740_e43806, assign38740_e43806_d_n4, assign38740_e43806_d_n6, assign38740_e43806_d_n7, assign38740_e43806_d_n8, assign38740_e43806_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38740_e43794: f64 = (-locals.var_q_zero__blk849);
        let assign38740_e43796: f64 = (assign38740_e43794 * locals.var_q_d1_zero__blk850);
        let assign38740_e43798: f64 = (assign38740_e43796 * locals.var_q_temp__blk860);
        let assign38740_e43801: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign38740_e43803: f64 = (assign38740_e43801 + 1e-200);
        let assign38740_e43804: f64 = (assign38740_e43798 / assign38740_e43803);
        (assign38740_e43804, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn4)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn6)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn7)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn8)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign38740_e43803 * assign38740_e43803)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign38740_e43794 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign38740_e43796 * locals.var_q_temp__blk860_dn9)) * assign38740_e43803) - (assign38740_e43798 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign38740_e43803 * assign38740_e43803)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign38740_e43806;
        locals.var_q_eps2__blk852_dn4 = assign38740_e43806_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign38740_e43806_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign38740_e43806_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign38740_e43806_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign38740_e43806_d_n9;

        let (assign38750_e43812, assign38750_e43812_d_n4, assign38750_e43812_d_n6, assign38750_e43812_d_n7, assign38750_e43812_d_n8, assign38750_e43812_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign38750_e43810: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign38750_e43810, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign38750_e43812;
        locals.var_q1d__blk1001_dn4 = assign38750_e43812_d_n4;
        locals.var_q1d__blk1001_dn6 = assign38750_e43812_d_n6;
        locals.var_q1d__blk1001_dn7 = assign38750_e43812_d_n7;
        locals.var_q1d__blk1001_dn8 = assign38750_e43812_d_n8;
        locals.var_q1d__blk1001_dn9 = assign38750_e43812_d_n9;

        let assign38760_e43815: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign38760_e43815;

        let assign38770_e43817: f64 = (locals.var_q_eps2__blk852).abs();
        let assign38770_e43819: f64 = if assign38770_e43817 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign38770_e43819;

        let (assign38780_e43829, assign38780_e43829_d_n4, assign38780_e43829_d_n6, assign38780_e43829_d_n7, assign38780_e43829_d_n8, assign38780_e43829_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38780_e43827: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign38780_e43827, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign38780_e43829;
        locals.var_q_k1q1__blk823_dn4 = assign38780_e43829_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign38780_e43829_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign38780_e43829_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign38780_e43829_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign38780_e43829_d_n9;

        let assign38790_e43832: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38790_e43834: f64 = (assign38790_e43832 - locals.var_xdeff__blk1000);
        let assign38790_e43836: f64 = if assign38790_e43834 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign38790_e43836;

        let (assign38800_e43851, assign38800_e43851_d_n4, assign38800_e43851_d_n6, assign38800_e43851_d_n7, assign38800_e43851_d_n8, assign38800_e43851_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 != 0.0)) {
        let assign38800_e43846: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38800_e43848: f64 = (assign38800_e43846 - locals.var_xdeff__blk1000);
        let assign38800_e43849: f64 = (assign38800_e43848).exp();
        (assign38800_e43849, (assign38800_e43849 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign38800_e43849 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38800_e43851;
        locals.var_q_temp1__blk814_dn4 = assign38800_e43851_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38800_e43851_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38800_e43851_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38800_e43851_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38800_e43851_d_n9;

        let (assign38810_e43896, assign38810_e43896_d_n4, assign38810_e43896_d_n6, assign38810_e43896_d_n7, assign38810_e43896_d_n8, assign38810_e43896_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1198 == 0.0)) {
        let assign38810_e43864: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38810_e43866: f64 = (assign38810_e43864 - locals.var_xdeff__blk1000);
        let assign38810_e43868: f64 = (assign38810_e43866 - 80.0);
        let assign38810_e43873: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38810_e43875: f64 = (assign38810_e43873 - locals.var_xdeff__blk1000);
        let assign38810_e43877: f64 = (assign38810_e43875 - 80.0);
        let assign38810_e43878: f64 = (0.5 * assign38810_e43877);
        let assign38810_e43882: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign38810_e43884: f64 = (assign38810_e43882 - locals.var_xdeff__blk1000);
        let assign38810_e43886: f64 = (assign38810_e43884 - 80.0);
        let assign38810_e43888: f64 = (assign38810_e43886 * 0.3333333333333);
        let assign38810_e43889: f64 = (1.0 + assign38810_e43888);
        let assign38810_e43890: f64 = (assign38810_e43878 * assign38810_e43889);
        let assign38810_e43891: f64 = (1.0 + assign38810_e43890);
        let assign38810_e43892: f64 = (assign38810_e43868 * assign38810_e43891);
        let assign38810_e43893: f64 = (1.0 + assign38810_e43892);
        let assign38810_e43894: f64 = (5.54062e34 * assign38810_e43893);
        (assign38810_e43894, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign38810_e43891) + (assign38810_e43868 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign38810_e43889) + (assign38810_e43878 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38810_e43896;
        locals.var_q_temp1__blk814_dn4 = assign38810_e43896_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38810_e43896_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38810_e43896_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38810_e43896_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38810_e43896_d_n9;

        let (assign38820_e43906, assign38820_e43906_d_n4, assign38820_e43906_d_n6, assign38820_e43906_d_n7, assign38820_e43906_d_n8, assign38820_e43906_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38820_e43904: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign38820_e43904, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign38820_e43906;
        locals.var_q_aexp__blk824_dn4 = assign38820_e43906_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign38820_e43906_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign38820_e43906_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign38820_e43906_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign38820_e43906_d_n9;

        let (assign38830_e43918, assign38830_e43918_d_n4, assign38830_e43918_d_n6, assign38830_e43918_d_n7, assign38830_e43918_d_n8, assign38830_e43918_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38830_e43914: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign38830_e43916: f64 = (assign38830_e43914 - locals.var_q_aexp__blk824);
        (assign38830_e43916, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign38830_e43918;
        locals.var_q_qsq__blk825_dn4 = assign38830_e43918_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign38830_e43918_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign38830_e43918_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign38830_e43918_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign38830_e43918_d_n9;

        let (assign38840_e43932, assign38840_e43932_d_n4, assign38840_e43932_d_n6, assign38840_e43932_d_n7, assign38840_e43932_d_n8, assign38840_e43932_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38840_e43926: f64 = (2.0 * locals.var_k1__blk932);
        let assign38840_e43928: f64 = (assign38840_e43926 * locals.var_q_k1q1__blk823);
        let assign38840_e43930: f64 = (assign38840_e43928 + locals.var_q_aexp__blk824);
        (assign38840_e43930, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign38840_e43926 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign38840_e43932;
        locals.var_q_d1_qsq__blk826_dn4 = assign38840_e43932_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign38840_e43932_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign38840_e43932_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign38840_e43932_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign38840_e43932_d_n9;

    }

    pub(super) fn stamp_transient_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign38850_e43946, assign38850_e43946_d_n4, assign38850_e43946_d_n6, assign38850_e43946_d_n7, assign38850_e43946_d_n8, assign38850_e43946_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign38850_e43940: f64 = (2.0 * locals.var_k1__blk932);
        let assign38850_e43942: f64 = (assign38850_e43940 * locals.var_k1__blk932);
        let assign38850_e43944: f64 = (assign38850_e43942 - locals.var_q_aexp__blk824);
        (assign38850_e43944, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign38850_e43940 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign38850_e43946;
        locals.var_q_d2_qsq__blk827_dn4 = assign38850_e43946_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign38850_e43946_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign38850_e43946_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign38850_e43946_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign38850_e43946_d_n9;

        let assign38860_e43949: f64 = (-0.005);
        let assign38860_e43950: f64 = if locals.var_q_qsq__blk825 < assign38860_e43949 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign38860_e43950;

        let (assign38870_e43962, assign38870_e43962_d_n4, assign38870_e43962_d_n6, assign38870_e43962_d_n7, assign38870_e43962_d_n8, assign38870_e43962_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38870_e43959: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38870_e43960: f64 = (assign38870_e43959).sqrt();
        (assign38870_e43960, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38870_e43960)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38870_e43960)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38870_e43962;
        locals.var_q_rac_qsq__blk828_dn4 = assign38870_e43962_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38870_e43962_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38870_e43962_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38870_e43962_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38870_e43962_d_n9;

        let (assign38880_e43977, assign38880_e43977_d_n4, assign38880_e43977_d_n6, assign38880_e43977_d_n7, assign38880_e43977_d_n8, assign38880_e43977_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38880_e43973: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign38880_e43974: f64 = (assign38880_e43973).tan();
        let assign38880_e43975: f64 = (locals.var_q_rac_qsq__blk828 / assign38880_e43974);
        (assign38880_e43975, (((locals.var_q_rac_qsq__blk828_dn4 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn6 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn7 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn8 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)), (((locals.var_q_rac_qsq__blk828_dn9 * assign38880_e43974) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign38880_e43973).cos() * (assign38880_e43973).cos())))) / (assign38880_e43974 * assign38880_e43974)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38880_e43977;
        locals.var_q_qcoth__blk829_dn4 = assign38880_e43977_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38880_e43977_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38880_e43977_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38880_e43977_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38880_e43977_d_n9;

        let (assign38890_e43991, assign38890_e43991_d_n4, assign38890_e43991_d_n6, assign38890_e43991_d_n7, assign38890_e43991_d_n8, assign38890_e43991_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38890_e43987: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38890_e43989: f64 = (assign38890_e43987 / locals.var_q_qsq__blk825);
        (assign38890_e43989, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38890_e43987 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38890_e43991;
        locals.var_q_temp1__blk814_dn4 = assign38890_e43991_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38890_e43991_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38890_e43991_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38890_e43991_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38890_e43991_d_n9;

        let (assign38900_e44009, assign38900_e44009_d_n4, assign38900_e44009_d_n6, assign38900_e44009_d_n7, assign38900_e44009_d_n8, assign38900_e44009_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38900_e44003: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign38900_e44004: f64 = (locals.var_q_qcoth__blk829 * assign38900_e44003);
        let assign38900_e44005: f64 = (locals.var_q_qsq__blk825 + assign38900_e44004);
        let assign38900_e44007: f64 = (assign38900_e44005 * locals.var_q_temp1__blk814);
        (assign38900_e44007, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign38900_e44003) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign38900_e44005 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign38900_e44009;
        locals.var_q_d1_qcoth__blk830_dn4 = assign38900_e44009_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign38900_e44009_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign38900_e44009_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign38900_e44009_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign38900_e44009_d_n9;

        let (assign38910_e44035, assign38910_e44035_d_n4, assign38910_e44035_d_n6, assign38910_e44035_d_n7, assign38910_e44035_d_n8, assign38910_e44035_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38910_e44020: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign38910_e44023: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign38910_e44024: f64 = (assign38910_e44020 * assign38910_e44023);
        let assign38910_e44025: f64 = (locals.var_q_d1_qsq__blk826 - assign38910_e44024);
        let assign38910_e44027: f64 = (assign38910_e44025 * locals.var_q_temp1__blk814);
        let assign38910_e44030: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign38910_e44032: f64 = (assign38910_e44030 / locals.var_q_d1_qsq__blk826);
        let assign38910_e44033: f64 = (assign38910_e44027 + assign38910_e44032);
        (assign38910_e44033, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign38910_e44023) + (assign38910_e44020 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign38910_e44025 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign38910_e44030 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign38910_e44035;
        locals.var_q_d2_qcoth__blk832_dn4 = assign38910_e44035_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign38910_e44035_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign38910_e44035_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign38910_e44035_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign38910_e44035_d_n9;

        let (assign38920_e44049, assign38920_e44049_d_n4, assign38920_e44049_d_n6, assign38920_e44049_d_n7, assign38920_e44049_d_n8, assign38920_e44049_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38920_e44046: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign38920_e44047: f64 = (1.0 - assign38920_e44046);
        (assign38920_e44047, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign38920_e44049;
        locals.var_q_temp2__blk815_dn4 = assign38920_e44049_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign38920_e44049_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign38920_e44049_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign38920_e44049_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign38920_e44049_d_n9;

        let (assign38930_e44063, assign38930_e44063_d_n4, assign38930_e44063_d_n6, assign38930_e44063_d_n7, assign38930_e44063_d_n8, assign38930_e44063_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38930_e44059: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign38930_e44061: f64 = (assign38930_e44059 * locals.var_q_temp2__blk815);
        (assign38930_e44061, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign38930_e44059 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign38930_e44063;
        locals.var_q_d1_ln__blk835_dn4 = assign38930_e44063_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign38930_e44063_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign38930_e44063_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign38930_e44063_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign38930_e44063_d_n9;

        let (assign38940_e44085, assign38940_e44085_d_n4, assign38940_e44085_d_n6, assign38940_e44085_d_n7, assign38940_e44085_d_n8, assign38940_e44085_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 != 0.0)) {
        let assign38940_e44073: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign38940_e44078: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign38940_e44079: f64 = (locals.var_q_d1_ln__blk835 + assign38940_e44078);
        let assign38940_e44080: f64 = (locals.var_q_d1_qsq__blk826 * assign38940_e44079);
        let assign38940_e44081: f64 = (assign38940_e44073 - assign38940_e44080);
        let assign38940_e44083: f64 = (assign38940_e44081 / locals.var_q_qsq__blk825);
        (assign38940_e44083, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign38940_e44079) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign38940_e44081 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign38940_e44085;
        locals.var_q_d2_ln__blk836_dn4 = assign38940_e44085_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign38940_e44085_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign38940_e44085_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign38940_e44085_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign38940_e44085_d_n9;

        let assign38950_e44088: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign38950_e44088;

        let (assign38960_e44103, assign38960_e44103_d_n4, assign38960_e44103_d_n6, assign38960_e44103_d_n7, assign38960_e44103_d_n8, assign38960_e44103_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38960_e44100: f64 = (locals.var_q_qsq__blk825).abs();
        let assign38960_e44101: f64 = (assign38960_e44100).sqrt();
        (assign38960_e44101, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign38960_e44101)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign38960_e44101)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign38960_e44103;
        locals.var_q_rac_qsq__blk828_dn4 = assign38960_e44103_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign38960_e44103_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign38960_e44103_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign38960_e44103_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign38960_e44103_d_n9;

        let (assign38970_e44118, assign38970_e44118_d_n4, assign38970_e44118_d_n6, assign38970_e44118_d_n7, assign38970_e44118_d_n8, assign38970_e44118_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38970_e44115: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign38970_e44116: f64 = (assign38970_e44115).exp();
        (assign38970_e44116, (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign38970_e44116 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign38970_e44118;
        locals.var_q_invexpq__blk831_dn4 = assign38970_e44118_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign38970_e44118_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign38970_e44118_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign38970_e44118_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign38970_e44118_d_n9;

        let (assign38980_e44139, assign38980_e44139_d_n4, assign38980_e44139_d_n6, assign38980_e44139_d_n7, assign38980_e44139_d_n8, assign38980_e44139_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38980_e44132: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign38980_e44133: f64 = (locals.var_q_rac_qsq__blk828 * assign38980_e44132);
        let assign38980_e44136: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign38980_e44137: f64 = (assign38980_e44133 / assign38980_e44136);
        (assign38980_e44137, (((((locals.var_q_rac_qsq__blk828_dn4 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn4))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn6))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn7))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn8))) / (assign38980_e44136 * assign38980_e44136)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign38980_e44132) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign38980_e44136) - (assign38980_e44133 * (-locals.var_q_invexpq__blk831_dn9))) / (assign38980_e44136 * assign38980_e44136)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign38980_e44139;
        locals.var_q_qcoth__blk829_dn4 = assign38980_e44139_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign38980_e44139_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign38980_e44139_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign38980_e44139_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign38980_e44139_d_n9;

        let (assign38990_e44156, assign38990_e44156_d_n4, assign38990_e44156_d_n6, assign38990_e44156_d_n7, assign38990_e44156_d_n8, assign38990_e44156_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign38990_e44152: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign38990_e44154: f64 = (assign38990_e44152 / locals.var_q_qsq__blk825);
        (assign38990_e44154, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign38990_e44152 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign38990_e44156;
        locals.var_q_temp1__blk814_dn4 = assign38990_e44156_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign38990_e44156_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign38990_e44156_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign38990_e44156_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign38990_e44156_d_n9;

        let (assign39000_e44177, assign39000_e44177_d_n4, assign39000_e44177_d_n6, assign39000_e44177_d_n7, assign39000_e44177_d_n8, assign39000_e44177_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign39000_e44171: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign39000_e44172: f64 = (locals.var_q_qcoth__blk829 * assign39000_e44171);
        let assign39000_e44173: f64 = (locals.var_q_qsq__blk825 + assign39000_e44172);
        let assign39000_e44175: f64 = (assign39000_e44173 * locals.var_q_temp1__blk814);
        (assign39000_e44175, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign39000_e44171) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign39000_e44173 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign39000_e44177;
        locals.var_q_d1_qcoth__blk830_dn4 = assign39000_e44177_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign39000_e44177_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign39000_e44177_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign39000_e44177_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign39000_e44177_d_n9;

        let (assign39010_e44206, assign39010_e44206_d_n4, assign39010_e44206_d_n6, assign39010_e44206_d_n7, assign39010_e44206_d_n8, assign39010_e44206_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign39010_e44191: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign39010_e44194: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign39010_e44195: f64 = (assign39010_e44191 * assign39010_e44194);
        let assign39010_e44196: f64 = (locals.var_q_d1_qsq__blk826 - assign39010_e44195);
        let assign39010_e44198: f64 = (assign39010_e44196 * locals.var_q_temp1__blk814);
        let assign39010_e44201: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign39010_e44203: f64 = (assign39010_e44201 / locals.var_q_d1_qsq__blk826);
        let assign39010_e44204: f64 = (assign39010_e44198 + assign39010_e44203);
        (assign39010_e44204, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign39010_e44194) + (assign39010_e44191 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign39010_e44196 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign39010_e44201 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign39010_e44206;
        locals.var_q_d2_qcoth__blk832_dn4 = assign39010_e44206_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign39010_e44206_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign39010_e44206_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign39010_e44206_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign39010_e44206_d_n9;

        let (assign39020_e44223, assign39020_e44223_d_n4, assign39020_e44223_d_n6, assign39020_e44223_d_n7, assign39020_e44223_d_n8, assign39020_e44223_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign39020_e44220: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign39020_e44221: f64 = (1.0 - assign39020_e44220);
        (assign39020_e44221, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39020_e44223;
        locals.var_q_temp2__blk815_dn4 = assign39020_e44223_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39020_e44223_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39020_e44223_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39020_e44223_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39020_e44223_d_n9;

        let (assign39030_e44240, assign39030_e44240_d_n4, assign39030_e44240_d_n6, assign39030_e44240_d_n7, assign39030_e44240_d_n8, assign39030_e44240_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign39030_e44236: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign39030_e44238: f64 = (assign39030_e44236 * locals.var_q_temp2__blk815);
        (assign39030_e44238, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign39030_e44236 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign39030_e44240;
        locals.var_q_d1_ln__blk835_dn4 = assign39030_e44240_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign39030_e44240_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign39030_e44240_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign39030_e44240_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign39030_e44240_d_n9;

        let (assign39040_e44265, assign39040_e44265_d_n4, assign39040_e44265_d_n6, assign39040_e44265_d_n7, assign39040_e44265_d_n8, assign39040_e44265_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 != 0.0)) {
        let assign39040_e44253: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign39040_e44258: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign39040_e44259: f64 = (locals.var_q_d1_ln__blk835 + assign39040_e44258);
        let assign39040_e44260: f64 = (locals.var_q_d1_qsq__blk826 * assign39040_e44259);
        let assign39040_e44261: f64 = (assign39040_e44253 - assign39040_e44260);
        let assign39040_e44263: f64 = (assign39040_e44261 / locals.var_q_qsq__blk825);
        (assign39040_e44263, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign39040_e44259) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign39040_e44261 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign39040_e44265;
        locals.var_q_d2_ln__blk836_dn4 = assign39040_e44265_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign39040_e44265_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign39040_e44265_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign39040_e44265_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign39040_e44265_d_n9;

        let (assign39050_e44297, assign39050_e44297_d_n4, assign39050_e44297_d_n6, assign39050_e44297_d_n7, assign39050_e44297_d_n8, assign39050_e44297_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39050_e44281: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign39050_e44285: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign39050_e44289: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign39050_e44290: f64 = (1.0 - assign39050_e44289);
        let assign39050_e44291: f64 = (assign39050_e44285 * assign39050_e44290);
        let assign39050_e44292: f64 = (1.0 - assign39050_e44291);
        let assign39050_e44293: f64 = (assign39050_e44281 * assign39050_e44292);
        let assign39050_e44294: f64 = (1.0 - assign39050_e44293);
        let assign39050_e44295: f64 = (0.1666666666667 * assign39050_e44294);
        (assign39050_e44295, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign39050_e44292) + (assign39050_e44281 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign39050_e44290) + (assign39050_e44285 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39050_e44297;
        locals.var_q_temp3__blk816_dn4 = assign39050_e44297_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39050_e44297_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39050_e44297_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39050_e44297_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39050_e44297_d_n9;

        let (assign39060_e44315, assign39060_e44315_d_n4, assign39060_e44315_d_n6, assign39060_e44315_d_n7, assign39060_e44315_d_n8, assign39060_e44315_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39060_e44312: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign39060_e44313: f64 = (2.0 + assign39060_e44312);
        (assign39060_e44313, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39060_e44315;
        locals.var_q_qcoth__blk829_dn4 = assign39060_e44315_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39060_e44315_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39060_e44315_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39060_e44315_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39060_e44315_d_n9;

        let (assign39070_e44347, assign39070_e44347_d_n4, assign39070_e44347_d_n6, assign39070_e44347_d_n7, assign39070_e44347_d_n8, assign39070_e44347_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39070_e44331: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign39070_e44335: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign39070_e44339: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign39070_e44340: f64 = (1.0 - assign39070_e44339);
        let assign39070_e44341: f64 = (assign39070_e44335 * assign39070_e44340);
        let assign39070_e44342: f64 = (1.0 - assign39070_e44341);
        let assign39070_e44343: f64 = (assign39070_e44331 * assign39070_e44342);
        let assign39070_e44344: f64 = (1.0 - assign39070_e44343);
        let assign39070_e44345: f64 = (0.1666666666667 * assign39070_e44344);
        (assign39070_e44345, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign39070_e44342) + (assign39070_e44331 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign39070_e44340) + (assign39070_e44335 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39070_e44347;
        locals.var_q_temp1__blk814_dn4 = assign39070_e44347_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39070_e44347_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39070_e44347_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39070_e44347_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39070_e44347_d_n9;

        let (assign39080_e44363, assign39080_e44363_d_n4, assign39080_e44363_d_n6, assign39080_e44363_d_n7, assign39080_e44363_d_n8, assign39080_e44363_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39080_e44361: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign39080_e44361, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign39080_e44363;
        locals.var_q_d1_qcoth__blk830_dn4 = assign39080_e44363_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign39080_e44363_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign39080_e44363_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign39080_e44363_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign39080_e44363_d_n9;

        let (assign39090_e44395, assign39090_e44395_d_n4, assign39090_e44395_d_n6, assign39090_e44395_d_n7, assign39090_e44395_d_n8, assign39090_e44395_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39090_e44379: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign39090_e44383: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign39090_e44387: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign39090_e44388: f64 = (1.0 - assign39090_e44387);
        let assign39090_e44389: f64 = (assign39090_e44383 * assign39090_e44388);
        let assign39090_e44390: f64 = (1.0 - assign39090_e44389);
        let assign39090_e44391: f64 = (assign39090_e44379 * assign39090_e44390);
        let assign39090_e44392: f64 = (1.0 - assign39090_e44391);
        let assign39090_e44393: f64 = (0.0055555555556 * assign39090_e44392);
        (assign39090_e44393, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign39090_e44390) + (assign39090_e44379 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign39090_e44388) + (assign39090_e44383 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39090_e44395;
        locals.var_q_temp2__blk815_dn4 = assign39090_e44395_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39090_e44395_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39090_e44395_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39090_e44395_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39090_e44395_d_n9;

        let (assign39100_e44417, assign39100_e44417_d_n4, assign39100_e44417_d_n6, assign39100_e44417_d_n7, assign39100_e44417_d_n8, assign39100_e44417_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39100_e44409: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign39100_e44412: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign39100_e44414: f64 = (assign39100_e44412 * locals.var_q_temp2__blk815);
        let assign39100_e44415: f64 = (assign39100_e44409 - assign39100_e44414);
        (assign39100_e44415, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign39100_e44412 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign39100_e44417;
        locals.var_q_d2_qcoth__blk832_dn4 = assign39100_e44417_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign39100_e44417_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign39100_e44417_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign39100_e44417_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign39100_e44417_d_n9;

        let (assign39110_e44436, assign39110_e44436_d_n4, assign39110_e44436_d_n6, assign39110_e44436_d_n7, assign39110_e44436_d_n8, assign39110_e44436_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39110_e44430: f64 = (-0.5);
        let assign39110_e44432: f64 = (assign39110_e44430 * locals.var_q_d1_qsq__blk826);
        let assign39110_e44434: f64 = (assign39110_e44432 * locals.var_q_temp3__blk816);
        (assign39110_e44434, (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn4)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn6)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn7)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn8)), (((assign39110_e44430 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign39110_e44432 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign39110_e44436;
        locals.var_q_d1_ln__blk835_dn4 = assign39110_e44436_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign39110_e44436_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign39110_e44436_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign39110_e44436_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign39110_e44436_d_n9;

        let (assign39120_e44475, assign39120_e44475_d_n4, assign39120_e44475_d_n6, assign39120_e44475_d_n7, assign39120_e44475_d_n8, assign39120_e44475_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1199 == 0.0)) && (locals.var_guard1200 == 0.0)) {
        let assign39120_e44449: f64 = (-0.5);
        let assign39120_e44451: f64 = (assign39120_e44449 * locals.var_q_d2_qsq__blk827);
        let assign39120_e44453: f64 = (assign39120_e44451 * locals.var_q_temp3__blk816);
        let assign39120_e44456: f64 = (0.25 * 0.0055555555556);
        let assign39120_e44458: f64 = (assign39120_e44456 * locals.var_q_d1_qsq__blk826);
        let assign39120_e44460: f64 = (assign39120_e44458 * locals.var_q_d1_qsq__blk826);
        let assign39120_e44464: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign39120_e44468: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign39120_e44469: f64 = (2.0 - assign39120_e44468);
        let assign39120_e44470: f64 = (assign39120_e44464 * assign39120_e44469);
        let assign39120_e44471: f64 = (1.0 - assign39120_e44470);
        let assign39120_e44472: f64 = (assign39120_e44460 * assign39120_e44471);
        let assign39120_e44473: f64 = (assign39120_e44453 + assign39120_e44472);
        (assign39120_e44473, ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn4)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn4)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn6)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn6)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn7)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn7)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn8)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn8)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign39120_e44449 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign39120_e44451 * locals.var_q_temp3__blk816_dn9)) + (((((assign39120_e44456 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign39120_e44458 * locals.var_q_d1_qsq__blk826_dn9)) * assign39120_e44471) + (assign39120_e44460 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign39120_e44469) + (assign39120_e44464 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign39120_e44475;
        locals.var_q_d2_ln__blk836_dn4 = assign39120_e44475_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign39120_e44475_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign39120_e44475_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign39120_e44475_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign39120_e44475_d_n9;

        let assign39130_e44478: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign39130_e44478;

        let (assign39140_e44498, assign39140_e44498_d_n4, assign39140_e44498_d_n6, assign39140_e44498_d_n7, assign39140_e44498_d_n8, assign39140_e44498_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign39140_e44488: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign39140_e44493: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign39140_e44494: f64 = (locals.var_q_invexpq__blk831 * assign39140_e44493);
        let assign39140_e44495: f64 = (1.0 - assign39140_e44494);
        let assign39140_e44496: f64 = (assign39140_e44488 / assign39140_e44495);
        (assign39140_e44496, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn4 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn6 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn7 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn8 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39140_e44495 * assign39140_e44495)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign39140_e44495) - (assign39140_e44488 * (-((locals.var_q_invexpq__blk831_dn9 * assign39140_e44493) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39140_e44495 * assign39140_e44495)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39140_e44498;
        locals.var_q_temp2__blk815_dn4 = assign39140_e44498_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39140_e44498_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39140_e44498_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39140_e44498_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39140_e44498_d_n9;

    }

    pub(super) fn stamp_transient_block_106(
        locals: &mut StampLocals,
    ) {
        let (assign39150_e44510, assign39150_e44510_d_n4, assign39150_e44510_d_n6, assign39150_e44510_d_n7, assign39150_e44510_d_n8, assign39150_e44510_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign39150_e44508: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign39150_e44508, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign39150_e44510;
        locals.var_q_sh_term__blk833_dn4 = assign39150_e44510_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign39150_e44510_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign39150_e44510_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign39150_e44510_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign39150_e44510_d_n9;

        let (assign39160_e44523, assign39160_e44523_d_n4, assign39160_e44523_d_n6, assign39160_e44523_d_n7, assign39160_e44523_d_n8, assign39160_e44523_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign39160_e44519: f64 = (locals.var_q_temp2__blk815).ln();
        let assign39160_e44521: f64 = (assign39160_e44519 - locals.var_q_rac_qsq__blk828);
        (assign39160_e44521, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign39160_e44523;
        locals.var_q_ln_term__blk834_dn4 = assign39160_e44523_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign39160_e44523_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign39160_e44523_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign39160_e44523_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign39160_e44523_d_n9;

        let assign39170_e44526: f64 = (-0.005);
        let assign39170_e44527: f64 = if locals.var_q_qsq__blk825 < assign39170_e44526 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign39170_e44527;

        let (assign39180_e44543, assign39180_e44543_d_n4, assign39180_e44543_d_n6, assign39180_e44543_d_n7, assign39180_e44543_d_n8, assign39180_e44543_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign39180_e44540: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39180_e44541: f64 = (assign39180_e44540).sin();
        (assign39180_e44541, ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39180_e44540).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39180_e44543;
        locals.var_q_temp2__blk815_dn4 = assign39180_e44543_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39180_e44543_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39180_e44543_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39180_e44543_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39180_e44543_d_n9;

        let (assign39190_e44561, assign39190_e44561_d_n4, assign39190_e44561_d_n6, assign39190_e44561_d_n7, assign39190_e44561_d_n8, assign39190_e44561_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign39190_e44555: f64 = (-locals.var_q_qsq__blk825);
        let assign39190_e44558: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign39190_e44559: f64 = (assign39190_e44555 / assign39190_e44558);
        (assign39190_e44559, ((((-locals.var_q_qsq__blk825_dn4) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn6) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn7) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn8) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39190_e44558 * assign39190_e44558)), ((((-locals.var_q_qsq__blk825_dn9) * assign39190_e44558) - (assign39190_e44555 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39190_e44558 * assign39190_e44558)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign39190_e44561;
        locals.var_q_sh_term__blk833_dn4 = assign39190_e44561_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign39190_e44561_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign39190_e44561_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign39190_e44561_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign39190_e44561_d_n9;

        let (assign39200_e44575, assign39200_e44575_d_n4, assign39200_e44575_d_n6, assign39200_e44575_d_n7, assign39200_e44575_d_n8, assign39200_e44575_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign39200_e44573: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign39200_e44573, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign39200_e44575;
        locals.var_q_ln_term__blk834_dn4 = assign39200_e44575_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign39200_e44575_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign39200_e44575_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign39200_e44575_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign39200_e44575_d_n9;

        let (assign39210_e44605, assign39210_e44605_d_n4, assign39210_e44605_d_n6, assign39210_e44605_d_n7, assign39210_e44605_d_n8, assign39210_e44605_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign39210_e44590: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign39210_e44594: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign39210_e44598: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign39210_e44599: f64 = (1.0 - assign39210_e44598);
        let assign39210_e44600: f64 = (assign39210_e44594 * assign39210_e44599);
        let assign39210_e44601: f64 = (1.0 - assign39210_e44600);
        let assign39210_e44602: f64 = (assign39210_e44590 * assign39210_e44601);
        let assign39210_e44603: f64 = (4.0 - assign39210_e44602);
        (assign39210_e44603, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign39210_e44601) + (assign39210_e44590 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign39210_e44599) + (assign39210_e44594 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign39210_e44605;
        locals.var_q_sh_term__blk833_dn4 = assign39210_e44605_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign39210_e44605_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign39210_e44605_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign39210_e44605_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign39210_e44605_d_n9;

        let (assign39220_e44620, assign39220_e44620_d_n4, assign39220_e44620_d_n6, assign39220_e44620_d_n7, assign39220_e44620_d_n8, assign39220_e44620_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1201 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign39220_e44618: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign39220_e44618, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign39220_e44620;
        locals.var_q_ln_term__blk834_dn4 = assign39220_e44620_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign39220_e44620_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign39220_e44620_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign39220_e44620_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign39220_e44620_d_n9;

        let assign39230_e44623: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign39230_e44625: f64 = (assign39230_e44623 + locals.var_q_qcoth__blk829);
        let assign39230_e44627: f64 = if assign39230_e44625 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign39230_e44627;

        let (assign39240_e44639, assign39240_e44639_d_n4, assign39240_e44639_d_n6, assign39240_e44639_d_n7, assign39240_e44639_d_n8, assign39240_e44639_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign39240_e44637: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign39240_e44637, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign39240_e44639;
        locals.var_q_expnum__blk837_dn4 = assign39240_e44639_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign39240_e44639_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign39240_e44639_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign39240_e44639_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign39240_e44639_d_n9;

        let (assign39250_e44651, assign39250_e44651_d_n4, assign39250_e44651_d_n6, assign39250_e44651_d_n7, assign39250_e44651_d_n8, assign39250_e44651_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign39250_e44649: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign39250_e44649, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign39250_e44651;
        locals.var_q_d1_expnum__blk838_dn4 = assign39250_e44651_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign39250_e44651_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign39250_e44651_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign39250_e44651_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign39250_e44651_d_n9;

        let (assign39260_e44661, assign39260_e44661_d_n4, assign39260_e44661_d_n6, assign39260_e44661_d_n7, assign39260_e44661_d_n8, assign39260_e44661_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign39260_e44661;
        locals.var_q_d2_expnum__blk839_dn4 = assign39260_e44661_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign39260_e44661_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign39260_e44661_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign39260_e44661_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign39260_e44661_d_n9;

        let (assign39270_e44676, assign39270_e44676_d_n4, assign39270_e44676_d_n6, assign39270_e44676_d_n7, assign39270_e44676_d_n8, assign39270_e44676_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39270_e44673: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign39270_e44674: f64 = (1.0 / assign39270_e44673);
        (assign39270_e44674, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign39270_e44673 * assign39270_e44673))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign39270_e44673 * assign39270_e44673))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39270_e44676;
        locals.var_q_temp2__blk815_dn4 = assign39270_e44676_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39270_e44676_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39270_e44676_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39270_e44676_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39270_e44676_d_n9;

        let (assign39280_e44689, assign39280_e44689_d_n4, assign39280_e44689_d_n6, assign39280_e44689_d_n7, assign39280_e44689_d_n8, assign39280_e44689_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39280_e44687: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign39280_e44687, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39280_e44689;
        locals.var_q_temp3__blk816_dn4 = assign39280_e44689_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39280_e44689_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39280_e44689_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39280_e44689_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39280_e44689_d_n9;

        let (assign39290_e44704, assign39290_e44704_d_n4, assign39290_e44704_d_n6, assign39290_e44704_d_n7, assign39290_e44704_d_n8, assign39290_e44704_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39290_e44700: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign39290_e44702: f64 = (assign39290_e44700 * locals.var_q_temp2__blk815);
        (assign39290_e44702, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign39290_e44700 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign39290_e44704;
        locals.var_q_expnum__blk837_dn4 = assign39290_e44704_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign39290_e44704_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign39290_e44704_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign39290_e44704_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign39290_e44704_d_n9;

        let (assign39300_e44725, assign39300_e44725_d_n4, assign39300_e44725_d_n6, assign39300_e44725_d_n7, assign39300_e44725_d_n8, assign39300_e44725_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39300_e44715: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign39300_e44717: f64 = (assign39300_e44715 - locals.var_q_aexp__blk824);
        let assign39300_e44720: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign39300_e44721: f64 = (assign39300_e44717 - assign39300_e44720);
        let assign39300_e44723: f64 = (assign39300_e44721 * locals.var_q_temp2__blk815);
        (assign39300_e44723, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39300_e44721 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign39300_e44725;
        locals.var_q_d1_expnum__blk838_dn4 = assign39300_e44725_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign39300_e44725_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign39300_e44725_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign39300_e44725_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign39300_e44725_d_n9;

        let (assign39310_e44756, assign39310_e44756_d_n4, assign39310_e44756_d_n6, assign39310_e44756_d_n7, assign39310_e44756_d_n8, assign39310_e44756_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1203 == 0.0)) {
        let assign39310_e44736: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign39310_e44739: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign39310_e44741: f64 = (assign39310_e44739 * locals.var_q_d1_expnum__blk838);
        let assign39310_e44742: f64 = (assign39310_e44736 + assign39310_e44741);
        let assign39310_e44744: f64 = (assign39310_e44742 + locals.var_q_aexp__blk824);
        let assign39310_e44748: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign39310_e44749: f64 = (locals.var_q_d2_ln__blk836 + assign39310_e44748);
        let assign39310_e44751: f64 = (assign39310_e44749 * locals.var_q_sh_term__blk833);
        let assign39310_e44752: f64 = (assign39310_e44744 - assign39310_e44751);
        let assign39310_e44754: f64 = (assign39310_e44752 * locals.var_q_temp2__blk815);
        (assign39310_e44754, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign39310_e44739 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign39310_e44749 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign39310_e44752 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign39310_e44756;
        locals.var_q_d2_expnum__blk839_dn4 = assign39310_e44756_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign39310_e44756_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign39310_e44756_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign39310_e44756_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign39310_e44756_d_n9;

        let assign39320_e44759: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign39320_e44759;

        let (assign39330_e44770, assign39330_e44770_d_n4, assign39330_e44770_d_n6, assign39330_e44770_d_n7, assign39330_e44770_d_n8, assign39330_e44770_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39330_e44768: f64 = (locals.var_q_expnum__blk837).ln();
        (assign39330_e44768, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign39330_e44770;
        locals.var_q_lnexpnum__blk840_dn4 = assign39330_e44770_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign39330_e44770_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign39330_e44770_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign39330_e44770_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign39330_e44770_d_n9;

        let (assign39340_e44782, assign39340_e44782_d_n4, assign39340_e44782_d_n6, assign39340_e44782_d_n7, assign39340_e44782_d_n8, assign39340_e44782_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39340_e44780: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign39340_e44780, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39340_e44782;
        locals.var_q_temp1__blk814_dn4 = assign39340_e44782_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39340_e44782_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39340_e44782_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39340_e44782_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39340_e44782_d_n9;

        let (assign39350_e44794, assign39350_e44794_d_n4, assign39350_e44794_d_n6, assign39350_e44794_d_n7, assign39350_e44794_d_n8, assign39350_e44794_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39350_e44792: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign39350_e44792, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign39350_e44794;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign39350_e44794_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign39350_e44794_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign39350_e44794_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign39350_e44794_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign39350_e44794_d_n9;

        let (assign39360_e44810, assign39360_e44810_d_n4, assign39360_e44810_d_n6, assign39360_e44810_d_n7, assign39360_e44810_d_n8, assign39360_e44810_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign39360_e44804: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign39360_e44807: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign39360_e44808: f64 = (assign39360_e44804 - assign39360_e44807);
        (assign39360_e44808, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign39360_e44810;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign39360_e44810_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign39360_e44810_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign39360_e44810_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign39360_e44810_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign39360_e44810_d_n9;

        let (assign39370_e44827, assign39370_e44827_d_n4, assign39370_e44827_d_n6, assign39370_e44827_d_n7, assign39370_e44827_d_n8, assign39370_e44827_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39370_e44821: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign39370_e44823: f64 = (-locals.var_q_k1q1__blk823);
        let assign39370_e44824: f64 = (assign39370_e44823).ln();
        let assign39370_e44825: f64 = (assign39370_e44821 + assign39370_e44824);
        (assign39370_e44825, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign39370_e44823)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign39370_e44823)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign39370_e44827;
        locals.var_q_lnexpnum__blk840_dn4 = assign39370_e44827_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign39370_e44827_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign39370_e44827_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign39370_e44827_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign39370_e44827_d_n9;

        let (assign39380_e44840, assign39380_e44840_d_n4, assign39380_e44840_d_n6, assign39380_e44840_d_n7, assign39380_e44840_d_n8, assign39380_e44840_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39380_e44838: f64 = (1.0 / locals.var_q1d__blk1001);
        (assign39380_e44838, (-(locals.var_q1d__blk1001_dn4 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn6 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn7 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn8 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))), (-(locals.var_q1d__blk1001_dn9 / (locals.var_q1d__blk1001 * locals.var_q1d__blk1001))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39380_e44840;
        locals.var_q_temp1__blk814_dn4 = assign39380_e44840_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39380_e44840_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39380_e44840_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39380_e44840_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39380_e44840_d_n9;

        let (assign39390_e44853, assign39390_e44853_d_n4, assign39390_e44853_d_n6, assign39390_e44853_d_n7, assign39390_e44853_d_n8, assign39390_e44853_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39390_e44851: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign39390_e44851, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign39390_e44853;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign39390_e44853_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign39390_e44853_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign39390_e44853_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign39390_e44853_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign39390_e44853_d_n9;

        let (assign39400_e44867, assign39400_e44867_d_n4, assign39400_e44867_d_n6, assign39400_e44867_d_n7, assign39400_e44867_d_n8, assign39400_e44867_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign39400_e44863: f64 = (-locals.var_q_temp1__blk814);
        let assign39400_e44865: f64 = (assign39400_e44863 * locals.var_q_temp1__blk814);
        (assign39400_e44865, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign39400_e44863 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign39400_e44867;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign39400_e44867_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign39400_e44867_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign39400_e44867_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign39400_e44867_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign39400_e44867_d_n9;

        let (assign39410_e44885, assign39410_e44885_d_n4, assign39410_e44885_d_n6, assign39410_e44885_d_n7, assign39410_e44885_d_n8, assign39410_e44885_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39410_e44875: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign39410_e44877: f64 = (assign39410_e44875 + locals.var_q1d__blk1001);
        let assign39410_e44880: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign39410_e44881: f64 = (assign39410_e44877 + assign39410_e44880);
        let assign39410_e44883: f64 = (assign39410_e44881 - locals.var_q_ln_term__blk834);
        (assign39410_e44883, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign39410_e44885;
        locals.var_q_q2_int__blk843_dn4 = assign39410_e44885_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign39410_e44885_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign39410_e44885_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign39410_e44885_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign39410_e44885_d_n9;

        let (assign39420_e44899, assign39420_e44899_d_n4, assign39420_e44899_d_n6, assign39420_e44899_d_n7, assign39420_e44899_d_n8, assign39420_e44899_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39420_e44894: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign39420_e44895: f64 = (1.0 + assign39420_e44894);
        let assign39420_e44897: f64 = (assign39420_e44895 - locals.var_q_d1_ln__blk835);
        (assign39420_e44897, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign39420_e44899;
        locals.var_q_d1_q2__blk844_dn4 = assign39420_e44899_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign39420_e44899_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign39420_e44899_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign39420_e44899_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign39420_e44899_d_n9;

        let (assign39430_e44911, assign39430_e44911_d_n4, assign39430_e44911_d_n6, assign39430_e44911_d_n7, assign39430_e44911_d_n8, assign39430_e44911_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39430_e44907: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign39430_e44909: f64 = (assign39430_e44907 - locals.var_q_d2_ln__blk836);
        (assign39430_e44909, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign39430_e44911;
        locals.var_q_d2_q2__blk845_dn4 = assign39430_e44911_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign39430_e44911_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign39430_e44911_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign39430_e44911_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign39430_e44911_d_n9;

        let (assign39440_e44923, assign39440_e44923_d_n4, assign39440_e44923_d_n6, assign39440_e44923_d_n7, assign39440_e44923_d_n8, assign39440_e44923_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39440_e44920: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign39440_e44921: f64 = (locals.var_q_k1q1__blk823 + assign39440_e44920);
        (assign39440_e44921, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign39440_e44923;
        locals.var_q_qi_int__blk846_dn4 = assign39440_e44923_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign39440_e44923_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign39440_e44923_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign39440_e44923_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign39440_e44923_d_n9;

        let (assign39450_e44935, assign39450_e44935_d_n4, assign39450_e44935_d_n6, assign39450_e44935_d_n7, assign39450_e44935_d_n8, assign39450_e44935_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39450_e44932: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign39450_e44933: f64 = (locals.var_k1__blk932 + assign39450_e44932);
        (assign39450_e44933, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign39450_e44935;
        locals.var_q_d1_qi__blk847_dn4 = assign39450_e44935_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign39450_e44935_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign39450_e44935_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign39450_e44935_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign39450_e44935_d_n9;

        let (assign39460_e44945, assign39460_e44945_d_n4, assign39460_e44945_d_n6, assign39460_e44945_d_n7, assign39460_e44945_d_n8, assign39460_e44945_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39460_e44943: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign39460_e44943, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign39460_e44945;
        locals.var_q_d2_qi__blk848_dn4 = assign39460_e44945_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign39460_e44945_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign39460_e44945_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign39460_e44945_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign39460_e44945_d_n9;

        let (assign39470_e44957, assign39470_e44957_d_n4, assign39470_e44957_d_n6, assign39470_e44957_d_n7, assign39470_e44957_d_n8, assign39470_e44957_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39470_e44953: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign39470_e44955: f64 = (assign39470_e44953 - locals.var_q_aexp__blk824);
        (assign39470_e44955, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign39470_e44957;
        locals.var_q_zero__blk849_dn4 = assign39470_e44957_d_n4;
        locals.var_q_zero__blk849_dn6 = assign39470_e44957_d_n6;
        locals.var_q_zero__blk849_dn7 = assign39470_e44957_d_n7;
        locals.var_q_zero__blk849_dn8 = assign39470_e44957_d_n8;
        locals.var_q_zero__blk849_dn9 = assign39470_e44957_d_n9;

    }

    pub(super) fn stamp_transient_block_107(
        locals: &mut StampLocals,
    ) {
        let (assign39480_e44973, assign39480_e44973_d_n4, assign39480_e44973_d_n6, assign39480_e44973_d_n7, assign39480_e44973_d_n8, assign39480_e44973_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39480_e44965: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign39480_e44968: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign39480_e44969: f64 = (assign39480_e44965 + assign39480_e44968);
        let assign39480_e44971: f64 = (assign39480_e44969 + locals.var_q_aexp__blk824);
        (assign39480_e44971, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign39480_e44973;
        locals.var_q_d1_zero__blk850_dn4 = assign39480_e44973_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign39480_e44973_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign39480_e44973_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign39480_e44973_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign39480_e44973_d_n9;

        let (assign39490_e44995, assign39490_e44995_d_n4, assign39490_e44995_d_n6, assign39490_e44995_d_n7, assign39490_e44995_d_n8, assign39490_e44995_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39490_e44981: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign39490_e44984: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign39490_e44986: f64 = (assign39490_e44984 * locals.var_q_d1_expnum__blk838);
        let assign39490_e44987: f64 = (assign39490_e44981 + assign39490_e44986);
        let assign39490_e44990: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign39490_e44991: f64 = (assign39490_e44987 + assign39490_e44990);
        let assign39490_e44993: f64 = (assign39490_e44991 - locals.var_q_aexp__blk824);
        (assign39490_e44993, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign39490_e44984 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign39490_e44995;
        locals.var_q_d2_zero__blk851_dn4 = assign39490_e44995_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign39490_e44995_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign39490_e44995_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign39490_e44995_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign39490_e44995_d_n9;

        let (assign39500_e45011, assign39500_e45011_d_n4, assign39500_e45011_d_n6, assign39500_e45011_d_n7, assign39500_e45011_d_n8, assign39500_e45011_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39500_e45003: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign39500_e45006: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign39500_e45008: f64 = (assign39500_e45006 * locals.var_q_d2_zero__blk851);
        let assign39500_e45009: f64 = (assign39500_e45003 - assign39500_e45008);
        (assign39500_e45009, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign39500_e45006 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign39500_e45011;
        locals.var_q_temp__blk860_dn4 = assign39500_e45011_d_n4;
        locals.var_q_temp__blk860_dn6 = assign39500_e45011_d_n6;
        locals.var_q_temp__blk860_dn7 = assign39500_e45011_d_n7;
        locals.var_q_temp__blk860_dn8 = assign39500_e45011_d_n8;
        locals.var_q_temp__blk860_dn9 = assign39500_e45011_d_n9;

        let (assign39510_e45030, assign39510_e45030_d_n4, assign39510_e45030_d_n6, assign39510_e45030_d_n7, assign39510_e45030_d_n8, assign39510_e45030_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39510_e45018: f64 = (-locals.var_q_zero__blk849);
        let assign39510_e45020: f64 = (assign39510_e45018 * locals.var_q_d1_zero__blk850);
        let assign39510_e45022: f64 = (assign39510_e45020 * locals.var_q_temp__blk860);
        let assign39510_e45025: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign39510_e45027: f64 = (assign39510_e45025 + 1e-200);
        let assign39510_e45028: f64 = (assign39510_e45022 / assign39510_e45027);
        (assign39510_e45028, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn4)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn6)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn7)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn8)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign39510_e45027 * assign39510_e45027)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign39510_e45018 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign39510_e45020 * locals.var_q_temp__blk860_dn9)) * assign39510_e45027) - (assign39510_e45022 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign39510_e45027 * assign39510_e45027)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign39510_e45030;
        locals.var_q_eps2__blk852_dn4 = assign39510_e45030_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign39510_e45030_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign39510_e45030_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign39510_e45030_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign39510_e45030_d_n9;

        let (assign39520_e45040, assign39520_e45040_d_n4, assign39520_e45040_d_n6, assign39520_e45040_d_n7, assign39520_e45040_d_n8, assign39520_e45040_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1196 != 0.0)) && (locals.var_guard1197 != 0.0)) {
        let assign39520_e45038: f64 = (locals.var_q1d__blk1001 + locals.var_q_eps2__blk852);
        (assign39520_e45038, (locals.var_q1d__blk1001_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1d__blk1001_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1d__blk1001_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1d__blk1001_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1d__blk1001_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1d__blk1001, locals.var_q1d__blk1001_dn4, locals.var_q1d__blk1001_dn6, locals.var_q1d__blk1001_dn7, locals.var_q1d__blk1001_dn8, locals.var_q1d__blk1001_dn9,)
    }
};
        locals.var_q1d__blk1001 = assign39520_e45040;
        locals.var_q1d__blk1001_dn4 = assign39520_e45040_d_n4;
        locals.var_q1d__blk1001_dn6 = assign39520_e45040_d_n6;
        locals.var_q1d__blk1001_dn7 = assign39520_e45040_d_n7;
        locals.var_q1d__blk1001_dn8 = assign39520_e45040_d_n8;
        locals.var_q1d__blk1001_dn9 = assign39520_e45040_d_n9;

        let (assign39530_e45046, assign39530_e45046_d_n4, assign39530_e45046_d_n6, assign39530_e45046_d_n7, assign39530_e45046_d_n8, assign39530_e45046_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39530_e45044: f64 = (locals.var_k1__blk932 * locals.var_q1d__blk1001);
        (assign39530_e45044, ((locals.var_k1__blk932_dn4 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1d__blk1001) + (locals.var_k1__blk932 * locals.var_q1d__blk1001_dn9)),)
    } else {
        (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9,)
    }
};
        locals.var_k1q1d__blk1004 = assign39530_e45046;
        locals.var_k1q1d__blk1004_dn4 = assign39530_e45046_d_n4;
        locals.var_k1q1d__blk1004_dn6 = assign39530_e45046_d_n6;
        locals.var_k1q1d__blk1004_dn7 = assign39530_e45046_d_n7;
        locals.var_k1q1d__blk1004_dn8 = assign39530_e45046_d_n8;
        locals.var_k1q1d__blk1004_dn9 = assign39530_e45046_d_n9;

        let assign39540_e45049: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39540_e45051: f64 = (assign39540_e45049 - locals.var_xdeff__blk1000);
        let assign39540_e45053: f64 = if assign39540_e45051 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign39540_e45053;

        let (assign39550_e45064, assign39550_e45064_d_n4, assign39550_e45064_d_n6, assign39550_e45064_d_n7, assign39550_e45064_d_n8, assign39550_e45064_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 != 0.0)) {
        let assign39550_e45059: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39550_e45061: f64 = (assign39550_e45059 - locals.var_xdeff__blk1000);
        let assign39550_e45062: f64 = (assign39550_e45061).exp();
        (assign39550_e45062, (assign39550_e45062 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)), (assign39550_e45062 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39550_e45064;
        locals.var_q_temp1__blk814_dn4 = assign39550_e45064_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39550_e45064_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39550_e45064_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39550_e45064_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39550_e45064_d_n9;

        let (assign39560_e45105, assign39560_e45105_d_n4, assign39560_e45105_d_n6, assign39560_e45105_d_n7, assign39560_e45105_d_n8, assign39560_e45105_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1205 == 0.0)) {
        let assign39560_e45073: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39560_e45075: f64 = (assign39560_e45073 - locals.var_xdeff__blk1000);
        let assign39560_e45077: f64 = (assign39560_e45075 - 80.0);
        let assign39560_e45082: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39560_e45084: f64 = (assign39560_e45082 - locals.var_xdeff__blk1000);
        let assign39560_e45086: f64 = (assign39560_e45084 - 80.0);
        let assign39560_e45087: f64 = (0.5 * assign39560_e45086);
        let assign39560_e45091: f64 = (locals.var_xg1x__blk930 - locals.var_q1d__blk1001);
        let assign39560_e45093: f64 = (assign39560_e45091 - locals.var_xdeff__blk1000);
        let assign39560_e45095: f64 = (assign39560_e45093 - 80.0);
        let assign39560_e45097: f64 = (assign39560_e45095 * 0.3333333333333);
        let assign39560_e45098: f64 = (1.0 + assign39560_e45097);
        let assign39560_e45099: f64 = (assign39560_e45087 * assign39560_e45098);
        let assign39560_e45100: f64 = (1.0 + assign39560_e45099);
        let assign39560_e45101: f64 = (assign39560_e45077 * assign39560_e45100);
        let assign39560_e45102: f64 = (1.0 + assign39560_e45101);
        let assign39560_e45103: f64 = (5.54062e34 * assign39560_e45102);
        (assign39560_e45103, (5.54062e34 * ((((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1d__blk1001_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1d__blk1001_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1d__blk1001_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1d__blk1001_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * assign39560_e45100) + (assign39560_e45077 * (((0.5 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9)) * assign39560_e45098) + (assign39560_e45087 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1d__blk1001_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39560_e45105;
        locals.var_q_temp1__blk814_dn4 = assign39560_e45105_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39560_e45105_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39560_e45105_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39560_e45105_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39560_e45105_d_n9;

        let (assign39570_e45111, assign39570_e45111_d_n4, assign39570_e45111_d_n6, assign39570_e45111_d_n7, assign39570_e45111_d_n8, assign39570_e45111_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39570_e45109: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign39570_e45109, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_aexp1d__blk1007, locals.var_aexp1d__blk1007_dn4, locals.var_aexp1d__blk1007_dn6, locals.var_aexp1d__blk1007_dn7, locals.var_aexp1d__blk1007_dn8, locals.var_aexp1d__blk1007_dn9,)
    }
};
        locals.var_aexp1d__blk1007 = assign39570_e45111;
        locals.var_aexp1d__blk1007_dn4 = assign39570_e45111_d_n4;
        locals.var_aexp1d__blk1007_dn6 = assign39570_e45111_d_n6;
        locals.var_aexp1d__blk1007_dn7 = assign39570_e45111_d_n7;
        locals.var_aexp1d__blk1007_dn8 = assign39570_e45111_d_n8;
        locals.var_aexp1d__blk1007_dn9 = assign39570_e45111_d_n9;

        let (assign39580_e45119, assign39580_e45119_d_n4, assign39580_e45119_d_n6, assign39580_e45119_d_n7, assign39580_e45119_d_n8, assign39580_e45119_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign39580_e45115: f64 = (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004);
        let assign39580_e45117: f64 = (assign39580_e45115 - locals.var_aexp1d__blk1007);
        (assign39580_e45117, (((locals.var_k1q1d__blk1004_dn4 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn4)) - locals.var_aexp1d__blk1007_dn4), (((locals.var_k1q1d__blk1004_dn6 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn6)) - locals.var_aexp1d__blk1007_dn6), (((locals.var_k1q1d__blk1004_dn7 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn7)) - locals.var_aexp1d__blk1007_dn7), (((locals.var_k1q1d__blk1004_dn8 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn8)) - locals.var_aexp1d__blk1007_dn8), (((locals.var_k1q1d__blk1004_dn9 * locals.var_k1q1d__blk1004) + (locals.var_k1q1d__blk1004 * locals.var_k1q1d__blk1004_dn9)) - locals.var_aexp1d__blk1007_dn9),)
    } else {
        (locals.var_qsqd__blk1006, locals.var_qsqd__blk1006_dn4, locals.var_qsqd__blk1006_dn6, locals.var_qsqd__blk1006_dn7, locals.var_qsqd__blk1006_dn8, locals.var_qsqd__blk1006_dn9,)
    }
};
        locals.var_qsqd__blk1006 = assign39580_e45119;
        locals.var_qsqd__blk1006_dn4 = assign39580_e45119_d_n4;
        locals.var_qsqd__blk1006_dn6 = assign39580_e45119_d_n6;
        locals.var_qsqd__blk1006_dn7 = assign39580_e45119_d_n7;
        locals.var_qsqd__blk1006_dn8 = assign39580_e45119_d_n8;
        locals.var_qsqd__blk1006_dn9 = assign39580_e45119_d_n9;

        let assign39590_e45122: f64 = if locals.var_aexp1d__blk1007 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign39590_e45122;

        let (assign39600_e45128, assign39600_e45128_d_n4, assign39600_e45128_d_n6, assign39600_e45128_d_n7, assign39600_e45128_d_n8, assign39600_e45128_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39600_e45128;
        locals.var_qid__blk1003_dn4 = assign39600_e45128_d_n4;
        locals.var_qid__blk1003_dn6 = assign39600_e45128_d_n6;
        locals.var_qid__blk1003_dn7 = assign39600_e45128_d_n7;
        locals.var_qid__blk1003_dn8 = assign39600_e45128_d_n8;
        locals.var_qid__blk1003_dn9 = assign39600_e45128_d_n9;

        let (assign39610_e45136, assign39610_e45136_d_n4, assign39610_e45136_d_n6, assign39610_e45136_d_n7, assign39610_e45136_d_n8, assign39610_e45136_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign39610_e45134: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
        (assign39610_e45134, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39610_e45136;
        locals.var_k2q2d__blk1005_dn4 = assign39610_e45136_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39610_e45136_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39610_e45136_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39610_e45136_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39610_e45136_d_n9;

        let (assign39620_e45144, assign39620_e45144_d_n4, assign39620_e45144_d_n6, assign39620_e45144_d_n7, assign39620_e45144_d_n8, assign39620_e45144_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign39620_e45142: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
        (assign39620_e45142, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39620_e45144;
        locals.var_q2d__blk1002_dn4 = assign39620_e45144_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39620_e45144_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39620_e45144_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39620_e45144_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39620_e45144_d_n9;

        let assign39630_e45147: f64 = (-0.005);
        let assign39630_e45148: f64 = if locals.var_qsqd__blk1006 < assign39630_e45147 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign39630_e45148;

        let (assign39640_e45159, assign39640_e45159_d_n4, assign39640_e45159_d_n6, assign39640_e45159_d_n7, assign39640_e45159_d_n8, assign39640_e45159_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign39640_e45156: f64 = (locals.var_qsqd__blk1006).abs();
        let assign39640_e45157: f64 = (assign39640_e45156).sqrt();
        (assign39640_e45157, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39640_e45157)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39640_e45157)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign39640_e45159;
        locals.var_q_rac_qsq__blk828_dn4 = assign39640_e45159_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign39640_e45159_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign39640_e45159_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign39640_e45159_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign39640_e45159_d_n9;

        let (assign39650_e45173, assign39650_e45173_d_n4, assign39650_e45173_d_n6, assign39650_e45173_d_n7, assign39650_e45173_d_n8, assign39650_e45173_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign39650_e45169: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39650_e45170: f64 = (assign39650_e45169).tan();
        let assign39650_e45171: f64 = (locals.var_q_rac_qsq__blk828 / assign39650_e45170);
        (assign39650_e45171, (((locals.var_q_rac_qsq__blk828_dn4 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn6 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn7 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn8 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)), (((locals.var_q_rac_qsq__blk828_dn9 * assign39650_e45170) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign39650_e45169).cos() * (assign39650_e45169).cos())))) / (assign39650_e45170 * assign39650_e45170)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39650_e45173;
        locals.var_q_qcoth__blk829_dn4 = assign39650_e45173_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39650_e45173_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39650_e45173_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39650_e45173_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39650_e45173_d_n9;

        let assign39660_e45176: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign39660_e45176;

        let (assign39670_e45190, assign39670_e45190_d_n4, assign39670_e45190_d_n6, assign39670_e45190_d_n7, assign39670_e45190_d_n8, assign39670_e45190_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
        let assign39670_e45187: f64 = (locals.var_qsqd__blk1006).abs();
        let assign39670_e45188: f64 = (assign39670_e45187).sqrt();
        (assign39670_e45188, (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn4 } else { (-locals.var_qsqd__blk1006_dn4) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn6 } else { (-locals.var_qsqd__blk1006_dn6) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn7 } else { (-locals.var_qsqd__blk1006_dn7) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn8 } else { (-locals.var_qsqd__blk1006_dn8) } / (2.0 * assign39670_e45188)), (if locals.var_qsqd__blk1006 >= 0.0 { locals.var_qsqd__blk1006_dn9 } else { (-locals.var_qsqd__blk1006_dn9) } / (2.0 * assign39670_e45188)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign39670_e45190;
        locals.var_q_rac_qsq__blk828_dn4 = assign39670_e45190_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign39670_e45190_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign39670_e45190_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign39670_e45190_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign39670_e45190_d_n9;

        let (assign39680_e45204, assign39680_e45204_d_n4, assign39680_e45204_d_n6, assign39680_e45204_d_n7, assign39680_e45204_d_n8, assign39680_e45204_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
        let assign39680_e45201: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign39680_e45202: f64 = (assign39680_e45201).exp();
        (assign39680_e45202, (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign39680_e45202 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign39680_e45204;
        locals.var_q_invexpq__blk831_dn4 = assign39680_e45204_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign39680_e45204_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign39680_e45204_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign39680_e45204_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign39680_e45204_d_n9;

        let (assign39690_e45224, assign39690_e45224_d_n4, assign39690_e45224_d_n6, assign39690_e45224_d_n7, assign39690_e45224_d_n8, assign39690_e45224_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 != 0.0)) {
        let assign39690_e45217: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign39690_e45218: f64 = (locals.var_q_rac_qsq__blk828 * assign39690_e45217);
        let assign39690_e45221: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign39690_e45222: f64 = (assign39690_e45218 / assign39690_e45221);
        (assign39690_e45222, (((((locals.var_q_rac_qsq__blk828_dn4 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn4))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn6))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn7))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn8))) / (assign39690_e45221 * assign39690_e45221)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign39690_e45217) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign39690_e45221) - (assign39690_e45218 * (-locals.var_q_invexpq__blk831_dn9))) / (assign39690_e45221 * assign39690_e45221)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39690_e45224;
        locals.var_q_qcoth__blk829_dn4 = assign39690_e45224_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39690_e45224_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39690_e45224_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39690_e45224_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39690_e45224_d_n9;

        let (assign39700_e45253, assign39700_e45253_d_n4, assign39700_e45253_d_n6, assign39700_e45253_d_n7, assign39700_e45253_d_n8, assign39700_e45253_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) && (locals.var_guard1208 == 0.0)) {
        let assign39700_e45238: f64 = (locals.var_qsqd__blk1006 * 0.1666666666667);
        let assign39700_e45242: f64 = (locals.var_qsqd__blk1006 * 0.0166666666667);
        let assign39700_e45246: f64 = (locals.var_qsqd__blk1006 * 0.0238095238095);
        let assign39700_e45247: f64 = (1.0 - assign39700_e45246);
        let assign39700_e45248: f64 = (assign39700_e45242 * assign39700_e45247);
        let assign39700_e45249: f64 = (1.0 - assign39700_e45248);
        let assign39700_e45250: f64 = (assign39700_e45238 * assign39700_e45249);
        let assign39700_e45251: f64 = (2.0 + assign39700_e45250);
        (assign39700_e45251, (((locals.var_qsqd__blk1006_dn4 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn4 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn6 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn6 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn7 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn7 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn8 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn8 * 0.0238095238095))))))), (((locals.var_qsqd__blk1006_dn9 * 0.1666666666667) * assign39700_e45249) + (assign39700_e45238 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0166666666667) * assign39700_e45247) + (assign39700_e45242 * (-(locals.var_qsqd__blk1006_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign39700_e45253;
        locals.var_q_qcoth__blk829_dn4 = assign39700_e45253_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign39700_e45253_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign39700_e45253_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign39700_e45253_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign39700_e45253_d_n9;

        let assign39710_e45256: f64 = (1.01 * locals.var_k1q1d__blk1004);
        let assign39710_e45258: f64 = (assign39710_e45256 + locals.var_q_qcoth__blk829);
        let assign39710_e45260: f64 = if assign39710_e45258 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign39710_e45260;

        let (assign39720_e45271, assign39720_e45271_d_n4, assign39720_e45271_d_n6, assign39720_e45271_d_n7, assign39720_e45271_d_n8, assign39720_e45271_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) {
        let assign39720_e45269: f64 = (locals.var_k1q1d__blk1004 + locals.var_q_qcoth__blk829);
        (assign39720_e45269, (locals.var_k1q1d__blk1004_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39720_e45271;
        locals.var_q_temp1__blk814_dn4 = assign39720_e45271_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39720_e45271_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39720_e45271_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39720_e45271_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39720_e45271_d_n9;

        let assign39730_e45274: f64 = (locals.var_aexp1d__blk1007 * locals.var_k1q1d__blk1004);
        let assign39730_e45277: f64 = (0.9 * locals.var_k1q1d__blk1004);
        let assign39730_e45279: f64 = (assign39730_e45277 * locals.var_k1q1d__blk1004);
        let assign39730_e45281: f64 = (assign39730_e45279 * locals.var_q_temp1__blk814);
        let assign39730_e45282: f64 = if assign39730_e45274 < assign39730_e45281 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign39730_e45282;

        let (assign39740_e45297, assign39740_e45297_d_n4, assign39740_e45297_d_n6, assign39740_e45297_d_n7, assign39740_e45297_d_n8, assign39740_e45297_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign39740_e45293: f64 = (locals.var_aexp1d__blk1007 / locals.var_q_temp1__blk814);
        let assign39740_e45295: f64 = (assign39740_e45293 + 1e-80);
        (assign39740_e45295, (((locals.var_aexp1d__blk1007_dn4 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn4)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn6 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn6)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn7 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn7)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn8 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn8)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1d__blk1007_dn9 * locals.var_q_temp1__blk814) - (locals.var_aexp1d__blk1007 * locals.var_q_temp1__blk814_dn9)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)),)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39740_e45297;
        locals.var_qid__blk1003_dn4 = assign39740_e45297_d_n4;
        locals.var_qid__blk1003_dn6 = assign39740_e45297_d_n6;
        locals.var_qid__blk1003_dn7 = assign39740_e45297_d_n7;
        locals.var_qid__blk1003_dn8 = assign39740_e45297_d_n8;
        locals.var_qid__blk1003_dn9 = assign39740_e45297_d_n9;

        let (assign39750_e45310, assign39750_e45310_d_n4, assign39750_e45310_d_n6, assign39750_e45310_d_n7, assign39750_e45310_d_n8, assign39750_e45310_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign39750_e45308: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
        (assign39750_e45308, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39750_e45310;
        locals.var_k2q2d__blk1005_dn4 = assign39750_e45310_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39750_e45310_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39750_e45310_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39750_e45310_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39750_e45310_d_n9;

        let (assign39760_e45323, assign39760_e45323_d_n4, assign39760_e45323_d_n6, assign39760_e45323_d_n7, assign39760_e45323_d_n8, assign39760_e45323_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign39760_e45321: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
        (assign39760_e45321, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39760_e45323;
        locals.var_q2d__blk1002_dn4 = assign39760_e45323_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39760_e45323_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39760_e45323_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39760_e45323_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39760_e45323_d_n9;

        let assign39770_e45326: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign39770_e45326;

        let (assign39780_e45353, assign39780_e45353_d_n4, assign39780_e45353_d_n6, assign39780_e45353_d_n7, assign39780_e45353_d_n8, assign39780_e45353_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign39780_e45340: f64 = (4.0 * locals.var_qsqd__blk1006);
        let assign39780_e45345: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign39780_e45346: f64 = (locals.var_q_invexpq__blk831 * assign39780_e45345);
        let assign39780_e45347: f64 = (1.0 - assign39780_e45346);
        let assign39780_e45348: f64 = (assign39780_e45340 / assign39780_e45347);
        let assign39780_e45349: f64 = (assign39780_e45348).ln();
        let assign39780_e45351: f64 = (assign39780_e45349 - locals.var_q_rac_qsq__blk828);
        (assign39780_e45351, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn4 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn4), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn6 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn6), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn7 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn7), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn8 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn8), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * assign39780_e45347) - (assign39780_e45340 * (-((locals.var_q_invexpq__blk831_dn9 * assign39780_e45345) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39780_e45347 * assign39780_e45347)) / assign39780_e45348) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39780_e45353;
        locals.var_q_temp2__blk815_dn4 = assign39780_e45353_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39780_e45353_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39780_e45353_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39780_e45353_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39780_e45353_d_n9;

        let assign39790_e45356: f64 = (-0.005);
        let assign39790_e45357: f64 = if locals.var_qsqd__blk1006 < assign39790_e45356 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign39790_e45357;

        let (assign39800_e45377, assign39800_e45377_d_n4, assign39800_e45377_d_n6, assign39800_e45377_d_n7, assign39800_e45377_d_n8, assign39800_e45377_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
        let assign39800_e45374: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39800_e45375: f64 = (assign39800_e45374).sin();
        (assign39800_e45375, ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39800_e45374).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39800_e45377;
        locals.var_q_temp3__blk816_dn4 = assign39800_e45377_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39800_e45377_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39800_e45377_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39800_e45377_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39800_e45377_d_n9;

        let (assign39810_e45400, assign39810_e45400_d_n4, assign39810_e45400_d_n6, assign39810_e45400_d_n7, assign39810_e45400_d_n8, assign39810_e45400_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 != 0.0)) {
        let assign39810_e45393: f64 = (-locals.var_qsqd__blk1006);
        let assign39810_e45396: f64 = (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816);
        let assign39810_e45397: f64 = (assign39810_e45393 / assign39810_e45396);
        let assign39810_e45398: f64 = (assign39810_e45397).ln();
        (assign39810_e45398, (((((-locals.var_qsqd__blk1006_dn4) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn4)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn6) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn6)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn7) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn7)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn8) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn8)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397), (((((-locals.var_qsqd__blk1006_dn9) * assign39810_e45396) - (assign39810_e45393 * ((locals.var_q_temp3__blk816_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn9)))) / (assign39810_e45396 * assign39810_e45396)) / assign39810_e45397),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39810_e45400;
        locals.var_q_temp2__blk815_dn4 = assign39810_e45400_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39810_e45400_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39810_e45400_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39810_e45400_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39810_e45400_d_n9;

        let (assign39820_e45435, assign39820_e45435_d_n4, assign39820_e45435_d_n6, assign39820_e45435_d_n7, assign39820_e45435_d_n8, assign39820_e45435_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) && (locals.var_guard1211 == 0.0)) && (locals.var_guard1212 == 0.0)) {
        let assign39820_e45419: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
        let assign39820_e45423: f64 = (0.05 * locals.var_qsqd__blk1006);
        let assign39820_e45427: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
        let assign39820_e45428: f64 = (1.0 - assign39820_e45427);
        let assign39820_e45429: f64 = (assign39820_e45423 * assign39820_e45428);
        let assign39820_e45430: f64 = (1.0 - assign39820_e45429);
        let assign39820_e45431: f64 = (assign39820_e45419 * assign39820_e45430);
        let assign39820_e45432: f64 = (4.0 - assign39820_e45431);
        let assign39820_e45433: f64 = (assign39820_e45432).ln();
        (assign39820_e45433, ((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) / assign39820_e45432), ((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39820_e45430) + (assign39820_e45419 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39820_e45428) + (assign39820_e45423 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) / assign39820_e45432),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39820_e45435;
        locals.var_q_temp2__blk815_dn4 = assign39820_e45435_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39820_e45435_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39820_e45435_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39820_e45435_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39820_e45435_d_n9;

    }

    pub(super) fn stamp_transient_block_108(
        locals: &mut StampLocals,
    ) {
        let (assign39830_e45458, assign39830_e45458_d_n4, assign39830_e45458_d_n6, assign39830_e45458_d_n7, assign39830_e45458_d_n8, assign39830_e45458_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign39830_e45447: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign39830_e45449: f64 = (assign39830_e45447 + locals.var_q1d__blk1001);
        let assign39830_e45452: f64 = (locals.var_q_temp1__blk814).ln();
        let assign39830_e45453: f64 = (2.0 * assign39830_e45452);
        let assign39830_e45454: f64 = (assign39830_e45449 + assign39830_e45453);
        let assign39830_e45456: f64 = (assign39830_e45454 - locals.var_q_temp2__blk815);
        (assign39830_e45456, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1d__blk1001_dn4) + (2.0 * (locals.var_q_temp1__blk814_dn4 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1d__blk1001_dn6) + (2.0 * (locals.var_q_temp1__blk814_dn6 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1d__blk1001_dn7) + (2.0 * (locals.var_q_temp1__blk814_dn7 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1d__blk1001_dn8) + (2.0 * (locals.var_q_temp1__blk814_dn8 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1d__blk1001_dn9) + (2.0 * (locals.var_q_temp1__blk814_dn9 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn9),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39830_e45458;
        locals.var_q2d__blk1002_dn4 = assign39830_e45458_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39830_e45458_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39830_e45458_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39830_e45458_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39830_e45458_d_n9;

        let (assign39840_e45472, assign39840_e45472_d_n4, assign39840_e45472_d_n6, assign39840_e45472_d_n7, assign39840_e45472_d_n8, assign39840_e45472_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign39840_e45470: f64 = (locals.var_k2__blk933 * locals.var_q2d__blk1002);
        (assign39840_e45470, ((locals.var_k2__blk933_dn4 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2d__blk1002) + (locals.var_k2__blk933 * locals.var_q2d__blk1002_dn9)),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39840_e45472;
        locals.var_k2q2d__blk1005_dn4 = assign39840_e45472_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39840_e45472_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39840_e45472_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39840_e45472_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39840_e45472_d_n9;

        let (assign39850_e45486, assign39850_e45486_d_n4, assign39850_e45486_d_n6, assign39850_e45486_d_n7, assign39850_e45486_d_n8, assign39850_e45486_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign39850_e45484: f64 = (locals.var_k1q1d__blk1004 + locals.var_k2q2d__blk1005);
        (assign39850_e45484, (locals.var_k1q1d__blk1004_dn4 + locals.var_k2q2d__blk1005_dn4), (locals.var_k1q1d__blk1004_dn6 + locals.var_k2q2d__blk1005_dn6), (locals.var_k1q1d__blk1004_dn7 + locals.var_k2q2d__blk1005_dn7), (locals.var_k1q1d__blk1004_dn8 + locals.var_k2q2d__blk1005_dn8), (locals.var_k1q1d__blk1004_dn9 + locals.var_k2q2d__blk1005_dn9),)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39850_e45486;
        locals.var_qid__blk1003_dn4 = assign39850_e45486_d_n4;
        locals.var_qid__blk1003_dn6 = assign39850_e45486_d_n6;
        locals.var_qid__blk1003_dn7 = assign39850_e45486_d_n7;
        locals.var_qid__blk1003_dn8 = assign39850_e45486_d_n8;
        locals.var_qid__blk1003_dn9 = assign39850_e45486_d_n9;

        let assign39860_e45489: f64 = if locals.var_qsqd__blk1006 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign39860_e45489;

        let assign39870_e45492: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39870_e45494: f64 = (assign39870_e45492 - locals.var_xg1x__blk930);
        let assign39870_e45496: f64 = (assign39870_e45494 - locals.var_q_rac_qsq__blk828);
        let assign39870_e45498: f64 = if assign39870_e45496 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign39870_e45498;

        let (assign39880_e45519, assign39880_e45519_d_n4, assign39880_e45519_d_n6, assign39880_e45519_d_n7, assign39880_e45519_d_n8, assign39880_e45519_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 != 0.0)) {
        let assign39880_e45512: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39880_e45514: f64 = (assign39880_e45512 - locals.var_xg1x__blk930);
        let assign39880_e45516: f64 = (assign39880_e45514 - locals.var_q_rac_qsq__blk828);
        let assign39880_e45517: f64 = (assign39880_e45516).exp();
        (assign39880_e45517, (assign39880_e45517 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)), (assign39880_e45517 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39880_e45519;
        locals.var_q_temp3__blk816_dn4 = assign39880_e45519_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39880_e45519_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39880_e45519_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39880_e45519_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39880_e45519_d_n9;

        let (assign39890_e45574, assign39890_e45574_d_n4, assign39890_e45574_d_n6, assign39890_e45574_d_n7, assign39890_e45574_d_n8, assign39890_e45574_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) && (locals.var_guard1214 == 0.0)) {
        let assign39890_e45536: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39890_e45538: f64 = (assign39890_e45536 - locals.var_xg1x__blk930);
        let assign39890_e45540: f64 = (assign39890_e45538 - locals.var_q_rac_qsq__blk828);
        let assign39890_e45542: f64 = (assign39890_e45540 - 80.0);
        let assign39890_e45547: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39890_e45549: f64 = (assign39890_e45547 - locals.var_xg1x__blk930);
        let assign39890_e45551: f64 = (assign39890_e45549 - locals.var_q_rac_qsq__blk828);
        let assign39890_e45553: f64 = (assign39890_e45551 - 80.0);
        let assign39890_e45554: f64 = (0.5 * assign39890_e45553);
        let assign39890_e45558: f64 = (locals.var_q1d__blk1001 + locals.var_xdeff__blk1000);
        let assign39890_e45560: f64 = (assign39890_e45558 - locals.var_xg1x__blk930);
        let assign39890_e45562: f64 = (assign39890_e45560 - locals.var_q_rac_qsq__blk828);
        let assign39890_e45564: f64 = (assign39890_e45562 - 80.0);
        let assign39890_e45566: f64 = (assign39890_e45564 * 0.3333333333333);
        let assign39890_e45567: f64 = (1.0 + assign39890_e45566);
        let assign39890_e45568: f64 = (assign39890_e45554 * assign39890_e45567);
        let assign39890_e45569: f64 = (1.0 + assign39890_e45568);
        let assign39890_e45570: f64 = (assign39890_e45542 * assign39890_e45569);
        let assign39890_e45571: f64 = (1.0 + assign39890_e45570);
        let assign39890_e45572: f64 = (5.54062e34 * assign39890_e45571);
        (assign39890_e45572, (5.54062e34 * (((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn4 + locals.var_xdeff__blk1000_dn4) - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn6 + locals.var_xdeff__blk1000_dn6) - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn7 + locals.var_xdeff__blk1000_dn7) - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn8 + locals.var_xdeff__blk1000_dn8) - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * assign39890_e45569) + (assign39890_e45542 * (((0.5 * (((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)) * assign39890_e45567) + (assign39890_e45554 * ((((locals.var_q1d__blk1001_dn9 + locals.var_xdeff__blk1000_dn9) - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign39890_e45574;
        locals.var_q_temp3__blk816_dn4 = assign39890_e45574_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign39890_e45574_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign39890_e45574_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign39890_e45574_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign39890_e45574_d_n9;

        let (assign39900_e45588, assign39900_e45588_d_n4, assign39900_e45588_d_n6, assign39900_e45588_d_n7, assign39900_e45588_d_n8, assign39900_e45588_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
        let assign39900_e45586: f64 = (locals.var_q_temp3__blk816 / locals.var_a0__blk905);
        (assign39900_e45586, (((locals.var_q_temp3__blk816_dn4 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn6 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn7 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn8 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn9 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39900_e45588;
        locals.var_q_temp2__blk815_dn4 = assign39900_e45588_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39900_e45588_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39900_e45588_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39900_e45588_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39900_e45588_d_n9;

        let (assign39910_e45612, assign39910_e45612_d_n4, assign39910_e45612_d_n6, assign39910_e45612_d_n7, assign39910_e45612_d_n8, assign39910_e45612_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 != 0.0)) {
        let assign39910_e45600: f64 = (4.0 * locals.var_qsqd__blk1006);
        let assign39910_e45602: f64 = (assign39910_e45600 * locals.var_q_temp2__blk815);
        let assign39910_e45607: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign39910_e45608: f64 = (locals.var_q_invexpq__blk831 * assign39910_e45607);
        let assign39910_e45609: f64 = (1.0 - assign39910_e45608);
        let assign39910_e45610: f64 = (assign39910_e45602 / assign39910_e45609);
        (assign39910_e45610, ((((((4.0 * locals.var_qsqd__blk1006_dn4) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn4)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn4 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn6) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn6)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn6 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn7) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn7)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn7 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn8) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn8)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn8 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign39910_e45609 * assign39910_e45609)), ((((((4.0 * locals.var_qsqd__blk1006_dn9) * locals.var_q_temp2__blk815) + (assign39910_e45600 * locals.var_q_temp2__blk815_dn9)) * assign39910_e45609) - (assign39910_e45602 * (-((locals.var_q_invexpq__blk831_dn9 * assign39910_e45607) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign39910_e45609 * assign39910_e45609)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39910_e45612;
        locals.var_q_temp1__blk814_dn4 = assign39910_e45612_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39910_e45612_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39910_e45612_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39910_e45612_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39910_e45612_d_n9;

        let assign39920_e45615: f64 = (-0.005);
        let assign39920_e45616: f64 = if locals.var_qsqd__blk1006 < assign39920_e45615 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign39920_e45616;

        let (assign39930_e45634, assign39930_e45634_d_n4, assign39930_e45634_d_n6, assign39930_e45634_d_n7, assign39930_e45634_d_n8, assign39930_e45634_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign39930_e45631: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign39930_e45632: f64 = (assign39930_e45631).sin();
        (assign39930_e45632, ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign39930_e45631).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign39930_e45634;
        locals.var_q_temp2__blk815_dn4 = assign39930_e45634_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign39930_e45634_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign39930_e45634_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign39930_e45634_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign39930_e45634_d_n9;

        let (assign39940_e45656, assign39940_e45656_d_n4, assign39940_e45656_d_n6, assign39940_e45656_d_n7, assign39940_e45656_d_n8, assign39940_e45656_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 != 0.0)) {
        let assign39940_e45648: f64 = (-locals.var_qsqd__blk1006);
        let assign39940_e45651: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign39940_e45652: f64 = (assign39940_e45648 / assign39940_e45651);
        let assign39940_e45654: f64 = (assign39940_e45652 / locals.var_aexp1d__blk1007);
        (assign39940_e45654, (((((((-locals.var_qsqd__blk1006_dn4) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn6) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn7) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn8) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), (((((((-locals.var_qsqd__blk1006_dn9) * assign39940_e45651) - (assign39940_e45648 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign39940_e45651 * assign39940_e45651)) * locals.var_aexp1d__blk1007) - (assign39940_e45652 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39940_e45656;
        locals.var_q_temp1__blk814_dn4 = assign39940_e45656_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39940_e45656_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39940_e45656_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39940_e45656_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39940_e45656_d_n9;

        let (assign39950_e45690, assign39950_e45690_d_n4, assign39950_e45690_d_n6, assign39950_e45690_d_n7, assign39950_e45690_d_n8, assign39950_e45690_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) && (locals.var_guard1213 == 0.0)) && (locals.var_guard1215 == 0.0)) {
        let assign39950_e45673: f64 = (locals.var_qsqd__blk1006 * 0.3333333333333);
        let assign39950_e45677: f64 = (0.05 * locals.var_qsqd__blk1006);
        let assign39950_e45681: f64 = (0.0396825396825397 * locals.var_qsqd__blk1006);
        let assign39950_e45682: f64 = (1.0 - assign39950_e45681);
        let assign39950_e45683: f64 = (assign39950_e45677 * assign39950_e45682);
        let assign39950_e45684: f64 = (1.0 - assign39950_e45683);
        let assign39950_e45685: f64 = (assign39950_e45673 * assign39950_e45684);
        let assign39950_e45686: f64 = (4.0 - assign39950_e45685);
        let assign39950_e45688: f64 = (assign39950_e45686 / locals.var_aexp1d__blk1007);
        (assign39950_e45688, ((((-(((locals.var_qsqd__blk1006_dn4 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn4) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn4)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn4)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn6 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn6) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn6)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn6)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn7 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn7) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn7)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn7)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn8 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn8) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn8)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn8)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)), ((((-(((locals.var_qsqd__blk1006_dn9 * 0.3333333333333) * assign39950_e45684) + (assign39950_e45673 * (-(((0.05 * locals.var_qsqd__blk1006_dn9) * assign39950_e45682) + (assign39950_e45677 * (-(0.0396825396825397 * locals.var_qsqd__blk1006_dn9)))))))) * locals.var_aexp1d__blk1007) - (assign39950_e45686 * locals.var_aexp1d__blk1007_dn9)) / (locals.var_aexp1d__blk1007 * locals.var_aexp1d__blk1007)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign39950_e45690;
        locals.var_q_temp1__blk814_dn4 = assign39950_e45690_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign39950_e45690_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign39950_e45690_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign39950_e45690_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign39950_e45690_d_n9;

        let (assign39960_e45708, assign39960_e45708_d_n4, assign39960_e45708_d_n6, assign39960_e45708_d_n7, assign39960_e45708_d_n8, assign39960_e45708_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
        let assign39960_e45700: f64 = (locals.var_k1q1d__blk1004 - locals.var_q_qcoth__blk829);
        let assign39960_e45703: f64 = (1.0 - locals.var_q_temp1__blk814);
        let assign39960_e45704: f64 = (assign39960_e45700 / assign39960_e45703);
        let assign39960_e45706: f64 = (assign39960_e45704 + 1e-80);
        (assign39960_e45706, ((((locals.var_k1q1d__blk1004_dn4 - locals.var_q_qcoth__blk829_dn4) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn4))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn6 - locals.var_q_qcoth__blk829_dn6) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn6))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn7 - locals.var_q_qcoth__blk829_dn7) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn7))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn8 - locals.var_q_qcoth__blk829_dn8) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn8))) / (assign39960_e45703 * assign39960_e45703)), ((((locals.var_k1q1d__blk1004_dn9 - locals.var_q_qcoth__blk829_dn9) * assign39960_e45703) - (assign39960_e45700 * (-locals.var_q_temp1__blk814_dn9))) / (assign39960_e45703 * assign39960_e45703)),)
    } else {
        (locals.var_qid__blk1003, locals.var_qid__blk1003_dn4, locals.var_qid__blk1003_dn6, locals.var_qid__blk1003_dn7, locals.var_qid__blk1003_dn8, locals.var_qid__blk1003_dn9,)
    }
};
        locals.var_qid__blk1003 = assign39960_e45708;
        locals.var_qid__blk1003_dn4 = assign39960_e45708_d_n4;
        locals.var_qid__blk1003_dn6 = assign39960_e45708_d_n6;
        locals.var_qid__blk1003_dn7 = assign39960_e45708_d_n7;
        locals.var_qid__blk1003_dn8 = assign39960_e45708_d_n8;
        locals.var_qid__blk1003_dn9 = assign39960_e45708_d_n9;

        let (assign39970_e45720, assign39970_e45720_d_n4, assign39970_e45720_d_n6, assign39970_e45720_d_n7, assign39970_e45720_d_n8, assign39970_e45720_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
        let assign39970_e45718: f64 = (locals.var_qid__blk1003 - locals.var_k1q1d__blk1004);
        (assign39970_e45718, (locals.var_qid__blk1003_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_qid__blk1003_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_qid__blk1003_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_qid__blk1003_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_qid__blk1003_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    }
};
        locals.var_k2q2d__blk1005 = assign39970_e45720;
        locals.var_k2q2d__blk1005_dn4 = assign39970_e45720_d_n4;
        locals.var_k2q2d__blk1005_dn6 = assign39970_e45720_d_n6;
        locals.var_k2q2d__blk1005_dn7 = assign39970_e45720_d_n7;
        locals.var_k2q2d__blk1005_dn8 = assign39970_e45720_d_n8;
        locals.var_k2q2d__blk1005_dn9 = assign39970_e45720_d_n9;

        let (assign39980_e45732, assign39980_e45732_d_n4, assign39980_e45732_d_n6, assign39980_e45732_d_n7, assign39980_e45732_d_n8, assign39980_e45732_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1209 == 0.0)) {
        let assign39980_e45730: f64 = (locals.var_k2q2d__blk1005 / locals.var_k2__blk933);
        (assign39980_e45730, (((locals.var_k2q2d__blk1005_dn4 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn6 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn7 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn8 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2d__blk1005_dn9 * locals.var_k2__blk933) - (locals.var_k2q2d__blk1005 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2d__blk1002, locals.var_q2d__blk1002_dn4, locals.var_q2d__blk1002_dn6, locals.var_q2d__blk1002_dn7, locals.var_q2d__blk1002_dn8, locals.var_q2d__blk1002_dn9,)
    }
};
        locals.var_q2d__blk1002 = assign39980_e45732;
        locals.var_q2d__blk1002_dn4 = assign39980_e45732_d_n4;
        locals.var_q2d__blk1002_dn6 = assign39980_e45732_d_n6;
        locals.var_q2d__blk1002_dn7 = assign39980_e45732_d_n7;
        locals.var_q2d__blk1002_dn8 = assign39980_e45732_d_n8;
        locals.var_q2d__blk1002_dn9 = assign39980_e45732_d_n9;

        let assign39990_e45735: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign39990_e45737: f64 = (assign39990_e45735 - locals.var_xdeff__blk1000);
        let assign39990_e45739: f64 = if assign39990_e45737 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign39990_e45739;

        let (assign40000_e45750, assign40000_e45750_d_n4, assign40000_e45750_d_n6, assign40000_e45750_d_n7, assign40000_e45750_d_n8, assign40000_e45750_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 != 0.0)) {
        let assign40000_e45745: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign40000_e45747: f64 = (assign40000_e45745 - locals.var_xdeff__blk1000);
        let assign40000_e45748: f64 = (assign40000_e45747).exp();
        (assign40000_e45748, (assign40000_e45748 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)), (assign40000_e45748 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign40000_e45750;
        locals.var_q_temp1__blk814_dn4 = assign40000_e45750_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign40000_e45750_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign40000_e45750_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign40000_e45750_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign40000_e45750_d_n9;

        let (assign40010_e45791, assign40010_e45791_d_n4, assign40010_e45791_d_n6, assign40010_e45791_d_n7, assign40010_e45791_d_n8, assign40010_e45791_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1216 == 0.0)) {
        let assign40010_e45759: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign40010_e45761: f64 = (assign40010_e45759 - locals.var_xdeff__blk1000);
        let assign40010_e45763: f64 = (assign40010_e45761 - 80.0);
        let assign40010_e45768: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign40010_e45770: f64 = (assign40010_e45768 - locals.var_xdeff__blk1000);
        let assign40010_e45772: f64 = (assign40010_e45770 - 80.0);
        let assign40010_e45773: f64 = (0.5 * assign40010_e45772);
        let assign40010_e45777: f64 = (locals.var_xg2x__blk931 - locals.var_q2d__blk1002);
        let assign40010_e45779: f64 = (assign40010_e45777 - locals.var_xdeff__blk1000);
        let assign40010_e45781: f64 = (assign40010_e45779 - 80.0);
        let assign40010_e45783: f64 = (assign40010_e45781 * 0.3333333333333);
        let assign40010_e45784: f64 = (1.0 + assign40010_e45783);
        let assign40010_e45785: f64 = (assign40010_e45773 * assign40010_e45784);
        let assign40010_e45786: f64 = (1.0 + assign40010_e45785);
        let assign40010_e45787: f64 = (assign40010_e45763 * assign40010_e45786);
        let assign40010_e45788: f64 = (1.0 + assign40010_e45787);
        let assign40010_e45789: f64 = (5.54062e34 * assign40010_e45788);
        (assign40010_e45789, (5.54062e34 * ((((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn4 - locals.var_q2d__blk1002_dn4) - locals.var_xdeff__blk1000_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn6 - locals.var_q2d__blk1002_dn6) - locals.var_xdeff__blk1000_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn7 - locals.var_q2d__blk1002_dn7) - locals.var_xdeff__blk1000_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn8 - locals.var_q2d__blk1002_dn8) - locals.var_xdeff__blk1000_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * assign40010_e45786) + (assign40010_e45763 * (((0.5 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9)) * assign40010_e45784) + (assign40010_e45773 * (((locals.var_xg2x__blk931_dn9 - locals.var_q2d__blk1002_dn9) - locals.var_xdeff__blk1000_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign40010_e45791;
        locals.var_q_temp1__blk814_dn4 = assign40010_e45791_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign40010_e45791_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign40010_e45791_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign40010_e45791_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign40010_e45791_d_n9;

        let (assign40020_e45797, assign40020_e45797_d_n4, assign40020_e45797_d_n6, assign40020_e45797_d_n7, assign40020_e45797_d_n8, assign40020_e45797_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40020_e45795: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign40020_e45795, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_aexp2d__blk1008, locals.var_aexp2d__blk1008_dn4, locals.var_aexp2d__blk1008_dn6, locals.var_aexp2d__blk1008_dn7, locals.var_aexp2d__blk1008_dn8, locals.var_aexp2d__blk1008_dn9,)
    }
};
        locals.var_aexp2d__blk1008 = assign40020_e45797;
        locals.var_aexp2d__blk1008_dn4 = assign40020_e45797_d_n4;
        locals.var_aexp2d__blk1008_dn6 = assign40020_e45797_d_n6;
        locals.var_aexp2d__blk1008_dn7 = assign40020_e45797_d_n7;
        locals.var_aexp2d__blk1008_dn8 = assign40020_e45797_d_n8;
        locals.var_aexp2d__blk1008_dn9 = assign40020_e45797_d_n9;

        let (assign40030_e45801, assign40030_e45801_d_n4, assign40030_e45801_d_n6, assign40030_e45801_d_n7, assign40030_e45801_d_n8, assign40030_e45801_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9,)
    }
};
        locals.var_a1d__blk1011 = assign40030_e45801;
        locals.var_a1d__blk1011_dn4 = assign40030_e45801_d_n4;
        locals.var_a1d__blk1011_dn6 = assign40030_e45801_d_n6;
        locals.var_a1d__blk1011_dn7 = assign40030_e45801_d_n7;
        locals.var_a1d__blk1011_dn8 = assign40030_e45801_d_n8;
        locals.var_a1d__blk1011_dn9 = assign40030_e45801_d_n9;

        let (assign40040_e45805, assign40040_e45805_d_n4, assign40040_e45805_d_n6, assign40040_e45805_d_n7, assign40040_e45805_d_n8, assign40040_e45805_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9,)
    }
};
        locals.var_a2d__blk1012 = assign40040_e45805;
        locals.var_a2d__blk1012_dn4 = assign40040_e45805_d_n4;
        locals.var_a2d__blk1012_dn6 = assign40040_e45805_d_n6;
        locals.var_a2d__blk1012_dn7 = assign40040_e45805_d_n7;
        locals.var_a2d__blk1012_dn8 = assign40040_e45805_d_n8;
        locals.var_a2d__blk1012_dn9 = assign40040_e45805_d_n9;

        let (assign40050_e45809, assign40050_e45809_d_n4, assign40050_e45809_d_n6, assign40050_e45809_d_n7, assign40050_e45809_d_n8, assign40050_e45809_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9,)
    }
};
        locals.var_b1d__blk1009 = assign40050_e45809;
        locals.var_b1d__blk1009_dn4 = assign40050_e45809_d_n4;
        locals.var_b1d__blk1009_dn6 = assign40050_e45809_d_n6;
        locals.var_b1d__blk1009_dn7 = assign40050_e45809_d_n7;
        locals.var_b1d__blk1009_dn8 = assign40050_e45809_d_n8;
        locals.var_b1d__blk1009_dn9 = assign40050_e45809_d_n9;

        let (assign40060_e45813, assign40060_e45813_d_n4, assign40060_e45813_d_n6, assign40060_e45813_d_n7, assign40060_e45813_d_n8, assign40060_e45813_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9,)
    }
};
        locals.var_b2d__blk1010 = assign40060_e45813;
        locals.var_b2d__blk1010_dn4 = assign40060_e45813_d_n4;
        locals.var_b2d__blk1010_dn6 = assign40060_e45813_d_n6;
        locals.var_b2d__blk1010_dn7 = assign40060_e45813_d_n7;
        locals.var_b2d__blk1010_dn8 = assign40060_e45813_d_n8;
        locals.var_b2d__blk1010_dn9 = assign40060_e45813_d_n9;

        let (assign40070_e45817, assign40070_e45817_d_n4, assign40070_e45817_d_n6, assign40070_e45817_d_n7, assign40070_e45817_d_n8, assign40070_e45817_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9,)
    }
};
        locals.var_sumd__blk1013 = assign40070_e45817;
        locals.var_sumd__blk1013_dn4 = assign40070_e45817_d_n4;
        locals.var_sumd__blk1013_dn6 = assign40070_e45817_d_n6;
        locals.var_sumd__blk1013_dn7 = assign40070_e45817_d_n7;
        locals.var_sumd__blk1013_dn8 = assign40070_e45817_d_n8;
        locals.var_sumd__blk1013_dn9 = assign40070_e45817_d_n9;

        let (assign40080_e45821, assign40080_e45821_d_n4, assign40080_e45821_d_n6, assign40080_e45821_d_n7, assign40080_e45821_d_n8, assign40080_e45821_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi__blk1014 = assign40080_e45821;
        locals.var_dqsqd_dxn_qi__blk1014_dn4 = assign40080_e45821_d_n4;
        locals.var_dqsqd_dxn_qi__blk1014_dn6 = assign40080_e45821_d_n6;
        locals.var_dqsqd_dxn_qi__blk1014_dn7 = assign40080_e45821_d_n7;
        locals.var_dqsqd_dxn_qi__blk1014_dn8 = assign40080_e45821_d_n8;
        locals.var_dqsqd_dxn_qi__blk1014_dn9 = assign40080_e45821_d_n9;

        let assign40090_e45824: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign40090_e45824;

        let (assign40100_e45832, assign40100_e45832_d_n4, assign40100_e45832_d_n6, assign40100_e45832_d_n7, assign40100_e45832_d_n8, assign40100_e45832_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40100_e45830: f64 = (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906);
        (assign40100_e45830, ((locals.var_aexp1d__blk1007_dn4 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn4)), ((locals.var_aexp1d__blk1007_dn6 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn6)), ((locals.var_aexp1d__blk1007_dn7 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn7)), ((locals.var_aexp1d__blk1007_dn8 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn8)), ((locals.var_aexp1d__blk1007_dn9 * locals.var_inv_k1__blk906) + (locals.var_aexp1d__blk1007 * locals.var_inv_k1__blk906_dn9)),)
    } else {
        (locals.var_b1d__blk1009, locals.var_b1d__blk1009_dn4, locals.var_b1d__blk1009_dn6, locals.var_b1d__blk1009_dn7, locals.var_b1d__blk1009_dn8, locals.var_b1d__blk1009_dn9,)
    }
};
        locals.var_b1d__blk1009 = assign40100_e45832;
        locals.var_b1d__blk1009_dn4 = assign40100_e45832_d_n4;
        locals.var_b1d__blk1009_dn6 = assign40100_e45832_d_n6;
        locals.var_b1d__blk1009_dn7 = assign40100_e45832_d_n7;
        locals.var_b1d__blk1009_dn8 = assign40100_e45832_d_n8;
        locals.var_b1d__blk1009_dn9 = assign40100_e45832_d_n9;

        let (assign40110_e45840, assign40110_e45840_d_n4, assign40110_e45840_d_n6, assign40110_e45840_d_n7, assign40110_e45840_d_n8, assign40110_e45840_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40110_e45838: f64 = (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907);
        (assign40110_e45838, ((locals.var_aexp2d__blk1008_dn4 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn4)), ((locals.var_aexp2d__blk1008_dn6 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn6)), ((locals.var_aexp2d__blk1008_dn7 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn7)), ((locals.var_aexp2d__blk1008_dn8 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn8)), ((locals.var_aexp2d__blk1008_dn9 * locals.var_inv_k2__blk907) + (locals.var_aexp2d__blk1008 * locals.var_inv_k2__blk907_dn9)),)
    } else {
        (locals.var_b2d__blk1010, locals.var_b2d__blk1010_dn4, locals.var_b2d__blk1010_dn6, locals.var_b2d__blk1010_dn7, locals.var_b2d__blk1010_dn8, locals.var_b2d__blk1010_dn9,)
    }
};
        locals.var_b2d__blk1010 = assign40110_e45840;
        locals.var_b2d__blk1010_dn4 = assign40110_e45840_d_n4;
        locals.var_b2d__blk1010_dn6 = assign40110_e45840_d_n6;
        locals.var_b2d__blk1010_dn7 = assign40110_e45840_d_n7;
        locals.var_b2d__blk1010_dn8 = assign40110_e45840_d_n8;
        locals.var_b2d__blk1010_dn9 = assign40110_e45840_d_n9;

        let (assign40120_e45850, assign40120_e45850_d_n4, assign40120_e45850_d_n6, assign40120_e45850_d_n7, assign40120_e45850_d_n8, assign40120_e45850_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40120_e45847: f64 = (2.0 * locals.var_k1q1d__blk1004);
        let assign40120_e45848: f64 = (locals.var_b1d__blk1009 + assign40120_e45847);
        (assign40120_e45848, (locals.var_b1d__blk1009_dn4 + (2.0 * locals.var_k1q1d__blk1004_dn4)), (locals.var_b1d__blk1009_dn6 + (2.0 * locals.var_k1q1d__blk1004_dn6)), (locals.var_b1d__blk1009_dn7 + (2.0 * locals.var_k1q1d__blk1004_dn7)), (locals.var_b1d__blk1009_dn8 + (2.0 * locals.var_k1q1d__blk1004_dn8)), (locals.var_b1d__blk1009_dn9 + (2.0 * locals.var_k1q1d__blk1004_dn9)),)
    } else {
        (locals.var_a1d__blk1011, locals.var_a1d__blk1011_dn4, locals.var_a1d__blk1011_dn6, locals.var_a1d__blk1011_dn7, locals.var_a1d__blk1011_dn8, locals.var_a1d__blk1011_dn9,)
    }
};
        locals.var_a1d__blk1011 = assign40120_e45850;
        locals.var_a1d__blk1011_dn4 = assign40120_e45850_d_n4;
        locals.var_a1d__blk1011_dn6 = assign40120_e45850_d_n6;
        locals.var_a1d__blk1011_dn7 = assign40120_e45850_d_n7;
        locals.var_a1d__blk1011_dn8 = assign40120_e45850_d_n8;
        locals.var_a1d__blk1011_dn9 = assign40120_e45850_d_n9;

        let (assign40130_e45860, assign40130_e45860_d_n4, assign40130_e45860_d_n6, assign40130_e45860_d_n7, assign40130_e45860_d_n8, assign40130_e45860_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40130_e45857: f64 = (2.0 * locals.var_k2q2d__blk1005);
        let assign40130_e45858: f64 = (locals.var_b2d__blk1010 + assign40130_e45857);
        (assign40130_e45858, (locals.var_b2d__blk1010_dn4 + (2.0 * locals.var_k2q2d__blk1005_dn4)), (locals.var_b2d__blk1010_dn6 + (2.0 * locals.var_k2q2d__blk1005_dn6)), (locals.var_b2d__blk1010_dn7 + (2.0 * locals.var_k2q2d__blk1005_dn7)), (locals.var_b2d__blk1010_dn8 + (2.0 * locals.var_k2q2d__blk1005_dn8)), (locals.var_b2d__blk1010_dn9 + (2.0 * locals.var_k2q2d__blk1005_dn9)),)
    } else {
        (locals.var_a2d__blk1012, locals.var_a2d__blk1012_dn4, locals.var_a2d__blk1012_dn6, locals.var_a2d__blk1012_dn7, locals.var_a2d__blk1012_dn8, locals.var_a2d__blk1012_dn9,)
    }
};
        locals.var_a2d__blk1012 = assign40130_e45860;
        locals.var_a2d__blk1012_dn4 = assign40130_e45860_d_n4;
        locals.var_a2d__blk1012_dn6 = assign40130_e45860_d_n6;
        locals.var_a2d__blk1012_dn7 = assign40130_e45860_d_n7;
        locals.var_a2d__blk1012_dn8 = assign40130_e45860_d_n8;
        locals.var_a2d__blk1012_dn9 = assign40130_e45860_d_n9;

        let (assign40140_e45872, assign40140_e45872_d_n4, assign40140_e45872_d_n6, assign40140_e45872_d_n7, assign40140_e45872_d_n8, assign40140_e45872_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign40140_e45866: f64 = (2.0 * locals.var_qid__blk1003);
        let assign40140_e45868: f64 = (assign40140_e45866 + locals.var_b1d__blk1009);
        let assign40140_e45870: f64 = (assign40140_e45868 + locals.var_b2d__blk1010);
        (assign40140_e45870, (((2.0 * locals.var_qid__blk1003_dn4) + locals.var_b1d__blk1009_dn4) + locals.var_b2d__blk1010_dn4), (((2.0 * locals.var_qid__blk1003_dn6) + locals.var_b1d__blk1009_dn6) + locals.var_b2d__blk1010_dn6), (((2.0 * locals.var_qid__blk1003_dn7) + locals.var_b1d__blk1009_dn7) + locals.var_b2d__blk1010_dn7), (((2.0 * locals.var_qid__blk1003_dn8) + locals.var_b1d__blk1009_dn8) + locals.var_b2d__blk1010_dn8), (((2.0 * locals.var_qid__blk1003_dn9) + locals.var_b1d__blk1009_dn9) + locals.var_b2d__blk1010_dn9),)
    } else {
        (locals.var_sumd__blk1013, locals.var_sumd__blk1013_dn4, locals.var_sumd__blk1013_dn6, locals.var_sumd__blk1013_dn7, locals.var_sumd__blk1013_dn8, locals.var_sumd__blk1013_dn9,)
    }
};
        locals.var_sumd__blk1013 = assign40140_e45872;
        locals.var_sumd__blk1013_dn4 = assign40140_e45872_d_n4;
        locals.var_sumd__blk1013_dn6 = assign40140_e45872_d_n6;
        locals.var_sumd__blk1013_dn7 = assign40140_e45872_d_n7;
        locals.var_sumd__blk1013_dn8 = assign40140_e45872_d_n8;
        locals.var_sumd__blk1013_dn9 = assign40140_e45872_d_n9;

        let assign40150_e45874: f64 = (locals.var_qsqd__blk1006).abs();
        let assign40150_e45876: f64 = if assign40150_e45874 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign40150_e45876;

    }

    pub(super) fn stamp_transient_block_109(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign40160_e45902, assign40160_e45902_d_n4, assign40160_e45902_d_n6, assign40160_e45902_d_n7, assign40160_e45902_d_n8, assign40160_e45902_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign40160_e45884: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
        let assign40160_e45888: f64 = (locals.var_q1d__blk1001 + 2.0);
        let assign40160_e45889: f64 = (2.0 * assign40160_e45888);
        let assign40160_e45891: f64 = (assign40160_e45889 * locals.var_a2d__blk1012);
        let assign40160_e45892: f64 = (assign40160_e45884 + assign40160_e45891);
        let assign40160_e45896: f64 = (locals.var_q2d__blk1002 + 2.0);
        let assign40160_e45897: f64 = (2.0 * assign40160_e45896);
        let assign40160_e45899: f64 = (assign40160_e45897 * locals.var_a1d__blk1011);
        let assign40160_e45900: f64 = (assign40160_e45892 + assign40160_e45899);
        (assign40160_e45900, ((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) + (((2.0 * locals.var_q1d__blk1001_dn4) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn4))) + (((2.0 * locals.var_q2d__blk1002_dn4) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn4))), ((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) + (((2.0 * locals.var_q1d__blk1001_dn6) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn6))) + (((2.0 * locals.var_q2d__blk1002_dn6) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn6))), ((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) + (((2.0 * locals.var_q1d__blk1001_dn7) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn7))) + (((2.0 * locals.var_q2d__blk1002_dn7) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn7))), ((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) + (((2.0 * locals.var_q1d__blk1001_dn8) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn8))) + (((2.0 * locals.var_q2d__blk1002_dn8) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn8))), ((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) + (((2.0 * locals.var_q1d__blk1001_dn9) * locals.var_a2d__blk1012) + (assign40160_e45889 * locals.var_a2d__blk1012_dn9))) + (((2.0 * locals.var_q2d__blk1002_dn9) * locals.var_a1d__blk1011) + (assign40160_e45897 * locals.var_a1d__blk1011_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40160_e45902;
        locals.var_temp1_dn4 = assign40160_e45902_d_n4;
        locals.var_temp1_dn6 = assign40160_e45902_d_n6;
        locals.var_temp1_dn7 = assign40160_e45902_d_n7;
        locals.var_temp1_dn8 = assign40160_e45902_d_n8;
        locals.var_temp1_dn9 = assign40160_e45902_d_n9;

        let (assign40170_e45919, assign40170_e45919_d_n4, assign40170_e45919_d_n6, assign40170_e45919_d_n7, assign40170_e45919_d_n8, assign40170_e45919_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign40170_e45909: f64 = (-4.0);
        let assign40170_e45911: f64 = (assign40170_e45909 * locals.var_qsqd__blk1006);
        let assign40170_e45913: f64 = (assign40170_e45911 * locals.var_sumd__blk1013);
        let assign40170_e45916: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
        let assign40170_e45917: f64 = (assign40170_e45913 / assign40170_e45916);
        (assign40170_e45917, ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn4) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn4)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn6) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn6)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn7) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn7)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn8) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn8)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8)))) / (assign40170_e45916 * assign40170_e45916)), ((((((assign40170_e45909 * locals.var_qsqd__blk1006_dn9) * locals.var_sumd__blk1013) + (assign40170_e45911 * locals.var_sumd__blk1013_dn9)) * assign40170_e45916) - (assign40170_e45913 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9)))) / (assign40170_e45916 * assign40170_e45916)),)
    } else {
        (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi__blk1014 = assign40170_e45919;
        locals.var_dqsqd_dxn_qi__blk1014_dn4 = assign40170_e45919_d_n4;
        locals.var_dqsqd_dxn_qi__blk1014_dn6 = assign40170_e45919_d_n6;
        locals.var_dqsqd_dxn_qi__blk1014_dn7 = assign40170_e45919_d_n7;
        locals.var_dqsqd_dxn_qi__blk1014_dn8 = assign40170_e45919_d_n8;
        locals.var_dqsqd_dxn_qi__blk1014_dn9 = assign40170_e45919_d_n9;

        let (assign40180_e45946, assign40180_e45946_d_n4, assign40180_e45946_d_n6, assign40180_e45946_d_n7, assign40180_e45946_d_n8, assign40180_e45946_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        let assign40180_e45930: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
        let assign40180_e45934: f64 = (locals.var_qsqd__blk1006 * 0.0357142857143);
        let assign40180_e45938: f64 = (locals.var_qsqd__blk1006 * 0.0333333333333);
        let assign40180_e45939: f64 = (1.0 - assign40180_e45938);
        let assign40180_e45940: f64 = (assign40180_e45934 * assign40180_e45939);
        let assign40180_e45941: f64 = (1.0 - assign40180_e45940);
        let assign40180_e45942: f64 = (assign40180_e45930 * assign40180_e45941);
        let assign40180_e45943: f64 = (1.0 - assign40180_e45942);
        let assign40180_e45944: f64 = (0.1666666666667 * assign40180_e45943);
        (assign40180_e45944, (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn4 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn6 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn7 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn8 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0333333333333) * assign40180_e45941) + (assign40180_e45930 * (-(((locals.var_qsqd__blk1006_dn9 * 0.0357142857143) * assign40180_e45939) + (assign40180_e45934 * (-(locals.var_qsqd__blk1006_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40180_e45946;
        locals.var_temp1_dn4 = assign40180_e45946_d_n4;
        locals.var_temp1_dn6 = assign40180_e45946_d_n6;
        locals.var_temp1_dn7 = assign40180_e45946_d_n7;
        locals.var_temp1_dn8 = assign40180_e45946_d_n8;
        locals.var_temp1_dn9 = assign40180_e45946_d_n9;

        let (assign40190_e45973, assign40190_e45973_d_n4, assign40190_e45973_d_n6, assign40190_e45973_d_n7, assign40190_e45973_d_n8, assign40190_e45973_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        let assign40190_e45955: f64 = (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007);
        let assign40190_e45958: f64 = (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008);
        let assign40190_e45959: f64 = (assign40190_e45955 + assign40190_e45958);
        let assign40190_e45962: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
        let assign40190_e45964: f64 = (assign40190_e45962 * locals.var_qid__blk1003);
        let assign40190_e45968: f64 = (locals.var_qid__blk1003 * locals.var_temp1);
        let assign40190_e45969: f64 = (1.0 + assign40190_e45968);
        let assign40190_e45970: f64 = (assign40190_e45964 * assign40190_e45969);
        let assign40190_e45971: f64 = (assign40190_e45959 + assign40190_e45970);
        (assign40190_e45971, ((((locals.var_a1d__blk1011_dn4 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn4)) + ((locals.var_a2d__blk1012_dn4 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn4))) + ((((((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn4)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn4 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn4))))), ((((locals.var_a1d__blk1011_dn6 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn6)) + ((locals.var_a2d__blk1012_dn6 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn6))) + ((((((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn6)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn6 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn6))))), ((((locals.var_a1d__blk1011_dn7 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn7)) + ((locals.var_a2d__blk1012_dn7 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn7))) + ((((((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn7)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn7 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn7))))), ((((locals.var_a1d__blk1011_dn8 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn8)) + ((locals.var_a2d__blk1012_dn8 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn8))) + ((((((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn8)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn8 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn8))))), ((((locals.var_a1d__blk1011_dn9 * locals.var_aexp1d__blk1007) + (locals.var_a1d__blk1011 * locals.var_aexp1d__blk1007_dn9)) + ((locals.var_a2d__blk1012_dn9 * locals.var_aexp2d__blk1008) + (locals.var_a2d__blk1012 * locals.var_aexp2d__blk1008_dn9))) + ((((((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)) * locals.var_qid__blk1003) + (assign40190_e45962 * locals.var_qid__blk1003_dn9)) * assign40190_e45969) + (assign40190_e45964 * ((locals.var_qid__blk1003_dn9 * locals.var_temp1) + (locals.var_qid__blk1003 * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40190_e45973;
        locals.var_temp2_dn4 = assign40190_e45973_d_n4;
        locals.var_temp2_dn6 = assign40190_e45973_d_n6;
        locals.var_temp2_dn7 = assign40190_e45973_d_n7;
        locals.var_temp2_dn8 = assign40190_e45973_d_n8;
        locals.var_temp2_dn9 = assign40190_e45973_d_n9;

        let (assign40200_e45990, assign40200_e45990_d_n4, assign40200_e45990_d_n6, assign40200_e45990_d_n7, assign40200_e45990_d_n8, assign40200_e45990_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1217 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        let assign40200_e45982: f64 = (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008);
        let assign40200_e45984: f64 = (assign40200_e45982 * locals.var_sumd__blk1013);
        let assign40200_e45987: f64 = (locals.var_qid__blk1003 * locals.var_temp2);
        let assign40200_e45988: f64 = (assign40200_e45984 / assign40200_e45987);
        (assign40200_e45988, (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn4)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn4)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn4 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn4)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn6)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn6)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn6 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn6)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn7)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn7)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn7 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn7)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn8)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn8)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn8 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn8)))) / (assign40200_e45987 * assign40200_e45987)), (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_aexp2d__blk1008) + (locals.var_aexp1d__blk1007 * locals.var_aexp2d__blk1008_dn9)) * locals.var_sumd__blk1013) + (assign40200_e45982 * locals.var_sumd__blk1013_dn9)) * assign40200_e45987) - (assign40200_e45984 * ((locals.var_qid__blk1003_dn9 * locals.var_temp2) + (locals.var_qid__blk1003 * locals.var_temp2_dn9)))) / (assign40200_e45987 * assign40200_e45987)),)
    } else {
        (locals.var_dqsqd_dxn_qi__blk1014, locals.var_dqsqd_dxn_qi__blk1014_dn4, locals.var_dqsqd_dxn_qi__blk1014_dn6, locals.var_dqsqd_dxn_qi__blk1014_dn7, locals.var_dqsqd_dxn_qi__blk1014_dn8, locals.var_dqsqd_dxn_qi__blk1014_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi__blk1014 = assign40200_e45990;
        locals.var_dqsqd_dxn_qi__blk1014_dn4 = assign40200_e45990_d_n4;
        locals.var_dqsqd_dxn_qi__blk1014_dn6 = assign40200_e45990_d_n6;
        locals.var_dqsqd_dxn_qi__blk1014_dn7 = assign40200_e45990_d_n7;
        locals.var_dqsqd_dxn_qi__blk1014_dn8 = assign40200_e45990_d_n8;
        locals.var_dqsqd_dxn_qi__blk1014_dn9 = assign40200_e45990_d_n9;

        let (assign40210_e45997, assign40210_e45997_d_n4, assign40210_e45997_d_n6, assign40210_e45997_d_n7, assign40210_e45997_d_n8, assign40210_e45997_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40210_e45994: f64 = (locals.var_qid__blk1003).ln();
        let assign40210_e45995: f64 = (locals.var_xdeff__blk1000 + assign40210_e45994);
        (assign40210_e45995, (locals.var_xdeff__blk1000_dn4 + (locals.var_qid__blk1003_dn4 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn6 + (locals.var_qid__blk1003_dn6 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn7 + (locals.var_qid__blk1003_dn7 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn8 + (locals.var_qid__blk1003_dn8 / locals.var_qid__blk1003)), (locals.var_xdeff__blk1000_dn9 + (locals.var_qid__blk1003_dn9 / locals.var_qid__blk1003)),)
    } else {
        (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9,)
    }
};
        locals.var_xdriftd__blk1015 = assign40210_e45997;
        locals.var_xdriftd__blk1015_dn4 = assign40210_e45997_d_n4;
        locals.var_xdriftd__blk1015_dn6 = assign40210_e45997_d_n6;
        locals.var_xdriftd__blk1015_dn7 = assign40210_e45997_d_n7;
        locals.var_xdriftd__blk1015_dn8 = assign40210_e45997_d_n8;
        locals.var_xdriftd__blk1015_dn9 = assign40210_e45997_d_n9;

        let (assign40220_e46005, assign40220_e46005_d_n4, assign40220_e46005_d_n6, assign40220_e46005_d_n7, assign40220_e46005_d_n8, assign40220_e46005_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40220_e46002: f64 = (locals.var_qis__blk938 + locals.var_qid__blk1003);
        let assign40220_e46003: f64 = (0.5 * assign40220_e46002);
        (assign40220_e46003, (0.5 * (locals.var_qis__blk938_dn4 + locals.var_qid__blk1003_dn4)), (0.5 * (locals.var_qis__blk938_dn6 + locals.var_qid__blk1003_dn6)), (0.5 * (locals.var_qis__blk938_dn7 + locals.var_qid__blk1003_dn7)), (0.5 * (locals.var_qis__blk938_dn8 + locals.var_qid__blk1003_dn8)), (0.5 * (locals.var_qis__blk938_dn9 + locals.var_qid__blk1003_dn9)),)
    } else {
        (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9,)
    }
};
        locals.var_qim__blk1016 = assign40220_e46005;
        locals.var_qim__blk1016_dn4 = assign40220_e46005_d_n4;
        locals.var_qim__blk1016_dn6 = assign40220_e46005_d_n6;
        locals.var_qim__blk1016_dn7 = assign40220_e46005_d_n7;
        locals.var_qim__blk1016_dn8 = assign40220_e46005_d_n8;
        locals.var_qim__blk1016_dn9 = assign40220_e46005_d_n9;

        let (assign40230_e46011, assign40230_e46011_d_n4, assign40230_e46011_d_n6, assign40230_e46011_d_n7, assign40230_e46011_d_n8, assign40230_e46011_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40230_e46009: f64 = (locals.var_xdriftd__blk1015 - locals.var_xdrifts__blk951);
        (assign40230_e46009, (locals.var_xdriftd__blk1015_dn4 - locals.var_xdrifts__blk951_dn4), (locals.var_xdriftd__blk1015_dn6 - locals.var_xdrifts__blk951_dn6), (locals.var_xdriftd__blk1015_dn7 - locals.var_xdrifts__blk951_dn7), (locals.var_xdriftd__blk1015_dn8 - locals.var_xdrifts__blk951_dn8), (locals.var_xdriftd__blk1015_dn9 - locals.var_xdrifts__blk951_dn9),)
    } else {
        (locals.var_dxdrift__blk1017, locals.var_dxdrift__blk1017_dn4, locals.var_dxdrift__blk1017_dn6, locals.var_dxdrift__blk1017_dn7, locals.var_dxdrift__blk1017_dn8, locals.var_dxdrift__blk1017_dn9,)
    }
};
        locals.var_dxdrift__blk1017 = assign40230_e46011;
        locals.var_dxdrift__blk1017_dn4 = assign40230_e46011_d_n4;
        locals.var_dxdrift__blk1017_dn6 = assign40230_e46011_d_n6;
        locals.var_dxdrift__blk1017_dn7 = assign40230_e46011_d_n7;
        locals.var_dxdrift__blk1017_dn8 = assign40230_e46011_d_n8;
        locals.var_dxdrift__blk1017_dn9 = assign40230_e46011_d_n9;

        let (assign40240_e46015, assign40240_e46015_d_n4, assign40240_e46015_d_n6, assign40240_e46015_d_n7, assign40240_e46015_d_n8, assign40240_e46015_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9,)
    }
};
        locals.var_ratio_pd__blk1020 = assign40240_e46015;
        locals.var_ratio_pd__blk1020_dn4 = assign40240_e46015_d_n4;
        locals.var_ratio_pd__blk1020_dn6 = assign40240_e46015_d_n6;
        locals.var_ratio_pd__blk1020_dn7 = assign40240_e46015_d_n7;
        locals.var_ratio_pd__blk1020_dn8 = assign40240_e46015_d_n8;
        locals.var_ratio_pd__blk1020_dn9 = assign40240_e46015_d_n9;

        let assign40250_e46018: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign40250_e46018;

        let (assign40260_e46030, assign40260_e46030_d_n4, assign40260_e46030_d_n6, assign40260_e46030_d_n7, assign40260_e46030_d_n8, assign40260_e46030_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40260_e46025: f64 = (locals.var_k1q1s__blk939 + locals.var_k1q1d__blk1004);
        let assign40260_e46026: f64 = (0.5 * assign40260_e46025);
        let assign40260_e46028: f64 = (assign40260_e46026 / locals.var_k1__blk932);
        (assign40260_e46028, ((((0.5 * (locals.var_k1q1s__blk939_dn4 + locals.var_k1q1d__blk1004_dn4)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn4)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn6 + locals.var_k1q1d__blk1004_dn6)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn6)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn7 + locals.var_k1q1d__blk1004_dn7)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn7)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn8 + locals.var_k1q1d__blk1004_dn8)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn8)) / (locals.var_k1__blk932 * locals.var_k1__blk932)), ((((0.5 * (locals.var_k1q1s__blk939_dn9 + locals.var_k1q1d__blk1004_dn9)) * locals.var_k1__blk932) - (assign40260_e46026 * locals.var_k1__blk932_dn9)) / (locals.var_k1__blk932 * locals.var_k1__blk932)),)
    } else {
        (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9,)
    }
};
        locals.var_qim_pd__blk1018 = assign40260_e46030;
        locals.var_qim_pd__blk1018_dn4 = assign40260_e46030_d_n4;
        locals.var_qim_pd__blk1018_dn6 = assign40260_e46030_d_n6;
        locals.var_qim_pd__blk1018_dn7 = assign40260_e46030_d_n7;
        locals.var_qim_pd__blk1018_dn8 = assign40260_e46030_d_n8;
        locals.var_qim_pd__blk1018_dn9 = assign40260_e46030_d_n9;

        let (assign40270_e46051, assign40270_e46051_d_n4, assign40270_e46051_d_n6, assign40270_e46051_d_n7, assign40270_e46051_d_n8, assign40270_e46051_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40270_e46037: f64 = (locals.var_qim_pd__blk1018 + 1e-5);
        let assign40270_e46040: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
        let assign40270_e46043: f64 = (locals.var_qim_pd__blk1018 - 1e-5);
        let assign40270_e46044: f64 = (assign40270_e46040 * assign40270_e46043);
        let assign40270_e46046: f64 = (assign40270_e46044 + 1.0);
        let assign40270_e46047: f64 = (assign40270_e46046).sqrt();
        let assign40270_e46048: f64 = (assign40270_e46037 + assign40270_e46047);
        let assign40270_e46049: f64 = (0.5 * assign40270_e46048);
        (assign40270_e46049, (0.5 * (locals.var_qim_pd__blk1018_dn4 + (((locals.var_qim_pd__blk1018_dn4 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn4)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn6 + (((locals.var_qim_pd__blk1018_dn6 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn6)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn7 + (((locals.var_qim_pd__blk1018_dn7 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn7)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn8 + (((locals.var_qim_pd__blk1018_dn8 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn8)) / (2.0 * assign40270_e46047)))), (0.5 * (locals.var_qim_pd__blk1018_dn9 + (((locals.var_qim_pd__blk1018_dn9 * assign40270_e46043) + (assign40270_e46040 * locals.var_qim_pd__blk1018_dn9)) / (2.0 * assign40270_e46047)))),)
    } else {
        (locals.var_qim_pd__blk1018, locals.var_qim_pd__blk1018_dn4, locals.var_qim_pd__blk1018_dn6, locals.var_qim_pd__blk1018_dn7, locals.var_qim_pd__blk1018_dn8, locals.var_qim_pd__blk1018_dn9,)
    }
};
        locals.var_qim_pd__blk1018 = assign40270_e46051;
        locals.var_qim_pd__blk1018_dn4 = assign40270_e46051_d_n4;
        locals.var_qim_pd__blk1018_dn6 = assign40270_e46051_d_n6;
        locals.var_qim_pd__blk1018_dn7 = assign40270_e46051_d_n7;
        locals.var_qim_pd__blk1018_dn8 = assign40270_e46051_d_n8;
        locals.var_qim_pd__blk1018_dn9 = assign40270_e46051_d_n9;

        let (assign40280_e46070, assign40280_e46070_d_n4, assign40280_e46070_d_n6, assign40280_e46070_d_n7, assign40280_e46070_d_n8, assign40280_e46070_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40280_e46057: f64 = (locals.var_qim_pd__blk1018 / locals.var_inv_phit);
        let assign40280_e46060: f64 = (0.25 * locals.var_kp);
        let assign40280_e46062: f64 = (assign40280_e46060 * locals.var_kp);
        let assign40280_e46063: f64 = (assign40280_e46057 + assign40280_e46062);
        let assign40280_e46064: f64 = (assign40280_e46063).sqrt();
        let assign40280_e46067: f64 = (0.5 * locals.var_kp);
        let assign40280_e46068: f64 = (assign40280_e46064 - assign40280_e46067);
        (assign40280_e46068, ((((((locals.var_qim_pd__blk1018_dn4 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn4) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn4))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn4)), ((((((locals.var_qim_pd__blk1018_dn6 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn6) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn6))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn6)), ((((((locals.var_qim_pd__blk1018_dn7 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn7) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn7))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn7)), ((((((locals.var_qim_pd__blk1018_dn8 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn8) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn8))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn8)), ((((((locals.var_qim_pd__blk1018_dn9 * locals.var_inv_phit) - (locals.var_qim_pd__blk1018 * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn9) * locals.var_kp) + (assign40280_e46060 * locals.var_kp_dn9))) / (2.0 * assign40280_e46064)) - (0.5 * locals.var_kp_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign40280_e46070;
        locals.var_temp0_dn4 = assign40280_e46070_d_n4;
        locals.var_temp0_dn6 = assign40280_e46070_d_n6;
        locals.var_temp0_dn7 = assign40280_e46070_d_n7;
        locals.var_temp0_dn8 = assign40280_e46070_d_n8;
        locals.var_temp0_dn9 = assign40280_e46070_d_n9;

        let (assign40290_e46080, assign40290_e46080_d_n4, assign40290_e46080_d_n6, assign40290_e46080_d_n7, assign40290_e46080_d_n8, assign40290_e46080_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40290_e46076: f64 = (locals.var_temp0).powf(2.0);
        let assign40290_e46078: f64 = (assign40290_e46076 * locals.var_inv_phit);
        (assign40290_e46078, ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn4)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn4 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn4)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn6)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn6 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn6)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn7)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn7 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn7)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn8)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn8 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn8)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn9)) } } else { (assign40290_e46076 * (2.0 * (locals.var_temp0_dn9 / locals.var_temp0))) } * locals.var_inv_phit) + (assign40290_e46076 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_xp_pd__blk1019, locals.var_xp_pd__blk1019_dn4, locals.var_xp_pd__blk1019_dn6, locals.var_xp_pd__blk1019_dn7, locals.var_xp_pd__blk1019_dn8, locals.var_xp_pd__blk1019_dn9,)
    }
};
        locals.var_xp_pd__blk1019 = assign40290_e46080;
        locals.var_xp_pd__blk1019_dn4 = assign40290_e46080_d_n4;
        locals.var_xp_pd__blk1019_dn6 = assign40290_e46080_d_n6;
        locals.var_xp_pd__blk1019_dn7 = assign40290_e46080_d_n7;
        locals.var_xp_pd__blk1019_dn8 = assign40290_e46080_d_n8;
        locals.var_xp_pd__blk1019_dn9 = assign40290_e46080_d_n9;

        let (assign40300_e46090, assign40300_e46090_d_n4, assign40300_e46090_d_n6, assign40300_e46090_d_n7, assign40300_e46090_d_n8, assign40300_e46090_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1219 != 0.0)) {
        let assign40300_e46087: f64 = (locals.var_xp_pd__blk1019 / locals.var_qim_pd__blk1018);
        let assign40300_e46088: f64 = (1.0 - assign40300_e46087);
        (assign40300_e46088, (-(((locals.var_xp_pd__blk1019_dn4 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn4)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn6 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn6)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn7 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn7)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn8 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn8)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))), (-(((locals.var_xp_pd__blk1019_dn9 * locals.var_qim_pd__blk1018) - (locals.var_xp_pd__blk1019 * locals.var_qim_pd__blk1018_dn9)) / (locals.var_qim_pd__blk1018 * locals.var_qim_pd__blk1018))),)
    } else {
        (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9,)
    }
};
        locals.var_ratio_pd__blk1020 = assign40300_e46090;
        locals.var_ratio_pd__blk1020_dn4 = assign40300_e46090_d_n4;
        locals.var_ratio_pd__blk1020_dn6 = assign40300_e46090_d_n6;
        locals.var_ratio_pd__blk1020_dn7 = assign40300_e46090_d_n7;
        locals.var_ratio_pd__blk1020_dn8 = assign40300_e46090_d_n8;
        locals.var_ratio_pd__blk1020_dn9 = assign40300_e46090_d_n9;

        let assign40310_e46093: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        let assign40310_e46095: f64 = if assign40310_e46093 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign40310_e46095;

        let (assign40320_e46107, assign40320_e46107_d_n4, assign40320_e46107_d_n6, assign40320_e46107_d_n7, assign40320_e46107_d_n8, assign40320_e46107_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 != 0.0)) {
        let assign40320_e46102: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        let assign40320_e46103: f64 = (assign40320_e46102).exp();
        let assign40320_e46104: f64 = (1.0 + assign40320_e46103);
        let assign40320_e46105: f64 = (assign40320_e46104).ln();
        (assign40320_e46105, ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn4 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn6 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn7 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn8 / 2.0)) / assign40320_e46104), ((assign40320_e46103 * (locals.var_k1q1d__blk1004_dn9 / 2.0)) / assign40320_e46104),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40320_e46107;
        locals.var_temp1_dn4 = assign40320_e46107_d_n4;
        locals.var_temp1_dn6 = assign40320_e46107_d_n6;
        locals.var_temp1_dn7 = assign40320_e46107_d_n7;
        locals.var_temp1_dn8 = assign40320_e46107_d_n8;
        locals.var_temp1_dn9 = assign40320_e46107_d_n9;

        let (assign40330_e46116, assign40330_e46116_d_n4, assign40330_e46116_d_n6, assign40330_e46116_d_n7, assign40330_e46116_d_n8, assign40330_e46116_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1220 == 0.0)) {
        let assign40330_e46114: f64 = (locals.var_k1q1d__blk1004 / 2.0);
        (assign40330_e46114, (locals.var_k1q1d__blk1004_dn4 / 2.0), (locals.var_k1q1d__blk1004_dn6 / 2.0), (locals.var_k1q1d__blk1004_dn7 / 2.0), (locals.var_k1q1d__blk1004_dn8 / 2.0), (locals.var_k1q1d__blk1004_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40330_e46116;
        locals.var_temp1_dn4 = assign40330_e46116_d_n4;
        locals.var_temp1_dn6 = assign40330_e46116_d_n6;
        locals.var_temp1_dn7 = assign40330_e46116_d_n7;
        locals.var_temp1_dn8 = assign40330_e46116_d_n8;
        locals.var_temp1_dn9 = assign40330_e46116_d_n9;

        let (assign40340_e46122, assign40340_e46122_d_n4, assign40340_e46122_d_n6, assign40340_e46122_d_n7, assign40340_e46122_d_n8, assign40340_e46122_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40340_e46120: f64 = (2.0 * locals.var_temp1);
        (assign40340_e46120, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_esurf1d__blk1021, locals.var_esurf1d__blk1021_dn4, locals.var_esurf1d__blk1021_dn6, locals.var_esurf1d__blk1021_dn7, locals.var_esurf1d__blk1021_dn8, locals.var_esurf1d__blk1021_dn9,)
    }
};
        locals.var_esurf1d__blk1021 = assign40340_e46122;
        locals.var_esurf1d__blk1021_dn4 = assign40340_e46122_d_n4;
        locals.var_esurf1d__blk1021_dn6 = assign40340_e46122_d_n6;
        locals.var_esurf1d__blk1021_dn7 = assign40340_e46122_d_n7;
        locals.var_esurf1d__blk1021_dn8 = assign40340_e46122_d_n8;
        locals.var_esurf1d__blk1021_dn9 = assign40340_e46122_d_n9;

        let assign40350_e46125: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        let assign40350_e46127: f64 = if assign40350_e46125 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign40350_e46127;

        let (assign40360_e46139, assign40360_e46139_d_n4, assign40360_e46139_d_n6, assign40360_e46139_d_n7, assign40360_e46139_d_n8, assign40360_e46139_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 != 0.0)) {
        let assign40360_e46134: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        let assign40360_e46135: f64 = (assign40360_e46134).exp();
        let assign40360_e46136: f64 = (1.0 + assign40360_e46135);
        let assign40360_e46137: f64 = (assign40360_e46136).ln();
        (assign40360_e46137, ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn4 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn6 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn7 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn8 / 2.0)) / assign40360_e46136), ((assign40360_e46135 * (locals.var_k2q2d__blk1005_dn9 / 2.0)) / assign40360_e46136),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40360_e46139;
        locals.var_temp2_dn4 = assign40360_e46139_d_n4;
        locals.var_temp2_dn6 = assign40360_e46139_d_n6;
        locals.var_temp2_dn7 = assign40360_e46139_d_n7;
        locals.var_temp2_dn8 = assign40360_e46139_d_n8;
        locals.var_temp2_dn9 = assign40360_e46139_d_n9;

        let (assign40370_e46148, assign40370_e46148_d_n4, assign40370_e46148_d_n6, assign40370_e46148_d_n7, assign40370_e46148_d_n8, assign40370_e46148_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1221 == 0.0)) {
        let assign40370_e46146: f64 = (locals.var_k2q2d__blk1005 / 2.0);
        (assign40370_e46146, (locals.var_k2q2d__blk1005_dn4 / 2.0), (locals.var_k2q2d__blk1005_dn6 / 2.0), (locals.var_k2q2d__blk1005_dn7 / 2.0), (locals.var_k2q2d__blk1005_dn8 / 2.0), (locals.var_k2q2d__blk1005_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40370_e46148;
        locals.var_temp2_dn4 = assign40370_e46148_d_n4;
        locals.var_temp2_dn6 = assign40370_e46148_d_n6;
        locals.var_temp2_dn7 = assign40370_e46148_d_n7;
        locals.var_temp2_dn8 = assign40370_e46148_d_n8;
        locals.var_temp2_dn9 = assign40370_e46148_d_n9;

        let (assign40380_e46154, assign40380_e46154_d_n4, assign40380_e46154_d_n6, assign40380_e46154_d_n7, assign40380_e46154_d_n8, assign40380_e46154_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40380_e46152: f64 = (2.0 * locals.var_temp2);
        (assign40380_e46152, (2.0 * locals.var_temp2_dn4), (2.0 * locals.var_temp2_dn6), (2.0 * locals.var_temp2_dn7), (2.0 * locals.var_temp2_dn8), (2.0 * locals.var_temp2_dn9),)
    } else {
        (locals.var_esurf2d__blk1022, locals.var_esurf2d__blk1022_dn4, locals.var_esurf2d__blk1022_dn6, locals.var_esurf2d__blk1022_dn7, locals.var_esurf2d__blk1022_dn8, locals.var_esurf2d__blk1022_dn9,)
    }
};
        locals.var_esurf2d__blk1022 = assign40380_e46154;
        locals.var_esurf2d__blk1022_dn4 = assign40380_e46154_d_n4;
        locals.var_esurf2d__blk1022_dn6 = assign40380_e46154_d_n6;
        locals.var_esurf2d__blk1022_dn7 = assign40380_e46154_d_n7;
        locals.var_esurf2d__blk1022_dn8 = assign40380_e46154_d_n8;
        locals.var_esurf2d__blk1022_dn9 = assign40380_e46154_d_n9;

        let (assign40390_e46160, assign40390_e46160_d_n4, assign40390_e46160_d_n6, assign40390_e46160_d_n7, assign40390_e46160_d_n8, assign40390_e46160_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40390_e46158: f64 = (locals.var_esurf2d__blk1022 - locals.var_k2q2d__blk1005);
        (assign40390_e46158, (locals.var_esurf2d__blk1022_dn4 - locals.var_k2q2d__blk1005_dn4), (locals.var_esurf2d__blk1022_dn6 - locals.var_k2q2d__blk1005_dn6), (locals.var_esurf2d__blk1022_dn7 - locals.var_k2q2d__blk1005_dn7), (locals.var_esurf2d__blk1022_dn8 - locals.var_k2q2d__blk1005_dn8), (locals.var_esurf2d__blk1022_dn9 - locals.var_k2q2d__blk1005_dn9),)
    } else {
        (locals.var_ecpl1d__blk1023, locals.var_ecpl1d__blk1023_dn4, locals.var_ecpl1d__blk1023_dn6, locals.var_ecpl1d__blk1023_dn7, locals.var_ecpl1d__blk1023_dn8, locals.var_ecpl1d__blk1023_dn9,)
    }
};
        locals.var_ecpl1d__blk1023 = assign40390_e46160;
        locals.var_ecpl1d__blk1023_dn4 = assign40390_e46160_d_n4;
        locals.var_ecpl1d__blk1023_dn6 = assign40390_e46160_d_n6;
        locals.var_ecpl1d__blk1023_dn7 = assign40390_e46160_d_n7;
        locals.var_ecpl1d__blk1023_dn8 = assign40390_e46160_d_n8;
        locals.var_ecpl1d__blk1023_dn9 = assign40390_e46160_d_n9;

        let (assign40400_e46166, assign40400_e46166_d_n4, assign40400_e46166_d_n6, assign40400_e46166_d_n7, assign40400_e46166_d_n8, assign40400_e46166_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40400_e46164: f64 = (locals.var_esurf1d__blk1021 - locals.var_k1q1d__blk1004);
        (assign40400_e46164, (locals.var_esurf1d__blk1021_dn4 - locals.var_k1q1d__blk1004_dn4), (locals.var_esurf1d__blk1021_dn6 - locals.var_k1q1d__blk1004_dn6), (locals.var_esurf1d__blk1021_dn7 - locals.var_k1q1d__blk1004_dn7), (locals.var_esurf1d__blk1021_dn8 - locals.var_k1q1d__blk1004_dn8), (locals.var_esurf1d__blk1021_dn9 - locals.var_k1q1d__blk1004_dn9),)
    } else {
        (locals.var_ecpl2d__blk1024, locals.var_ecpl2d__blk1024_dn4, locals.var_ecpl2d__blk1024_dn6, locals.var_ecpl2d__blk1024_dn7, locals.var_ecpl2d__blk1024_dn8, locals.var_ecpl2d__blk1024_dn9,)
    }
};
        locals.var_ecpl2d__blk1024 = assign40400_e46166;
        locals.var_ecpl2d__blk1024_dn4 = assign40400_e46166_d_n4;
        locals.var_ecpl2d__blk1024_dn6 = assign40400_e46166_d_n6;
        locals.var_ecpl2d__blk1024_dn7 = assign40400_e46166_d_n7;
        locals.var_ecpl2d__blk1024_dn8 = assign40400_e46166_d_n8;
        locals.var_ecpl2d__blk1024_dn9 = assign40400_e46166_d_n9;

        let (assign40410_e46176, assign40410_e46176_d_n4, assign40410_e46176_d_n6, assign40410_e46176_d_n7, assign40410_e46176_d_n8, assign40410_e46176_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40410_e46170: f64 = (locals.var_eta_mu * locals.var_esurf1d__blk1021);
        let assign40410_e46173: f64 = (locals.var_one_m_eta * locals.var_ecpl1d__blk1023);
        let assign40410_e46174: f64 = (assign40410_e46170 + assign40410_e46173);
        (assign40410_e46174, ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn4)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn6)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn7)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn8)), ((locals.var_eta_mu * locals.var_esurf1d__blk1021_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d__blk1023_dn9)),)
    } else {
        (locals.var_eeff1d__blk1025, locals.var_eeff1d__blk1025_dn4, locals.var_eeff1d__blk1025_dn6, locals.var_eeff1d__blk1025_dn7, locals.var_eeff1d__blk1025_dn8, locals.var_eeff1d__blk1025_dn9,)
    }
};
        locals.var_eeff1d__blk1025 = assign40410_e46176;
        locals.var_eeff1d__blk1025_dn4 = assign40410_e46176_d_n4;
        locals.var_eeff1d__blk1025_dn6 = assign40410_e46176_d_n6;
        locals.var_eeff1d__blk1025_dn7 = assign40410_e46176_d_n7;
        locals.var_eeff1d__blk1025_dn8 = assign40410_e46176_d_n8;
        locals.var_eeff1d__blk1025_dn9 = assign40410_e46176_d_n9;

        let (assign40420_e46186, assign40420_e46186_d_n4, assign40420_e46186_d_n6, assign40420_e46186_d_n7, assign40420_e46186_d_n8, assign40420_e46186_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40420_e46180: f64 = (locals.var_eta_mu * locals.var_esurf2d__blk1022);
        let assign40420_e46183: f64 = (locals.var_one_m_eta * locals.var_ecpl2d__blk1024);
        let assign40420_e46184: f64 = (assign40420_e46180 + assign40420_e46183);
        (assign40420_e46184, ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn4)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn6)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn7)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn8)), ((locals.var_eta_mu * locals.var_esurf2d__blk1022_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d__blk1024_dn9)),)
    } else {
        (locals.var_eeff2d__blk1026, locals.var_eeff2d__blk1026_dn4, locals.var_eeff2d__blk1026_dn6, locals.var_eeff2d__blk1026_dn7, locals.var_eeff2d__blk1026_dn8, locals.var_eeff2d__blk1026_dn9,)
    }
};
        locals.var_eeff2d__blk1026 = assign40420_e46186;
        locals.var_eeff2d__blk1026_dn4 = assign40420_e46186_d_n4;
        locals.var_eeff2d__blk1026_dn6 = assign40420_e46186_d_n6;
        locals.var_eeff2d__blk1026_dn7 = assign40420_e46186_d_n7;
        locals.var_eeff2d__blk1026_dn8 = assign40420_e46186_d_n8;
        locals.var_eeff2d__blk1026_dn9 = assign40420_e46186_d_n9;

        let (assign40430_e46194, assign40430_e46194_d_n4, assign40430_e46194_d_n6, assign40430_e46194_d_n7, assign40430_e46194_d_n8, assign40430_e46194_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40430_e46191: f64 = (locals.var_esurf1s__blk952 + locals.var_esurf1d__blk1021);
        let assign40430_e46192: f64 = (0.5 * assign40430_e46191);
        (assign40430_e46192, (0.5 * (locals.var_esurf1s__blk952_dn4 + locals.var_esurf1d__blk1021_dn4)), (0.5 * (locals.var_esurf1s__blk952_dn6 + locals.var_esurf1d__blk1021_dn6)), (0.5 * (locals.var_esurf1s__blk952_dn7 + locals.var_esurf1d__blk1021_dn7)), (0.5 * (locals.var_esurf1s__blk952_dn8 + locals.var_esurf1d__blk1021_dn8)), (0.5 * (locals.var_esurf1s__blk952_dn9 + locals.var_esurf1d__blk1021_dn9)),)
    } else {
        (locals.var_esurf1__blk1027, locals.var_esurf1__blk1027_dn4, locals.var_esurf1__blk1027_dn6, locals.var_esurf1__blk1027_dn7, locals.var_esurf1__blk1027_dn8, locals.var_esurf1__blk1027_dn9,)
    }
};
        locals.var_esurf1__blk1027 = assign40430_e46194;
        locals.var_esurf1__blk1027_dn4 = assign40430_e46194_d_n4;
        locals.var_esurf1__blk1027_dn6 = assign40430_e46194_d_n6;
        locals.var_esurf1__blk1027_dn7 = assign40430_e46194_d_n7;
        locals.var_esurf1__blk1027_dn8 = assign40430_e46194_d_n8;
        locals.var_esurf1__blk1027_dn9 = assign40430_e46194_d_n9;

        let (assign40440_e46202, assign40440_e46202_d_n4, assign40440_e46202_d_n6, assign40440_e46202_d_n7, assign40440_e46202_d_n8, assign40440_e46202_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40440_e46199: f64 = (locals.var_esurf2s__blk953 + locals.var_esurf2d__blk1022);
        let assign40440_e46200: f64 = (0.5 * assign40440_e46199);
        (assign40440_e46200, (0.5 * (locals.var_esurf2s__blk953_dn4 + locals.var_esurf2d__blk1022_dn4)), (0.5 * (locals.var_esurf2s__blk953_dn6 + locals.var_esurf2d__blk1022_dn6)), (0.5 * (locals.var_esurf2s__blk953_dn7 + locals.var_esurf2d__blk1022_dn7)), (0.5 * (locals.var_esurf2s__blk953_dn8 + locals.var_esurf2d__blk1022_dn8)), (0.5 * (locals.var_esurf2s__blk953_dn9 + locals.var_esurf2d__blk1022_dn9)),)
    } else {
        (locals.var_esurf2__blk1028, locals.var_esurf2__blk1028_dn4, locals.var_esurf2__blk1028_dn6, locals.var_esurf2__blk1028_dn7, locals.var_esurf2__blk1028_dn8, locals.var_esurf2__blk1028_dn9,)
    }
};
        locals.var_esurf2__blk1028 = assign40440_e46202;
        locals.var_esurf2__blk1028_dn4 = assign40440_e46202_d_n4;
        locals.var_esurf2__blk1028_dn6 = assign40440_e46202_d_n6;
        locals.var_esurf2__blk1028_dn7 = assign40440_e46202_d_n7;
        locals.var_esurf2__blk1028_dn8 = assign40440_e46202_d_n8;
        locals.var_esurf2__blk1028_dn9 = assign40440_e46202_d_n9;

        let (assign40450_e46210, assign40450_e46210_d_n4, assign40450_e46210_d_n6, assign40450_e46210_d_n7, assign40450_e46210_d_n8, assign40450_e46210_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40450_e46207: f64 = (locals.var_esurf1__blk1027 + locals.var_esurf2__blk1028);
        let assign40450_e46208: f64 = (1.0 / assign40450_e46207);
        (assign40450_e46208, (-((locals.var_esurf1__blk1027_dn4 + locals.var_esurf2__blk1028_dn4) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn6 + locals.var_esurf2__blk1028_dn6) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn7 + locals.var_esurf2__blk1028_dn7) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn8 + locals.var_esurf2__blk1028_dn8) / (assign40450_e46207 * assign40450_e46207))), (-((locals.var_esurf1__blk1027_dn9 + locals.var_esurf2__blk1028_dn9) / (assign40450_e46207 * assign40450_e46207))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40450_e46210;
        locals.var_temp_dn4 = assign40450_e46210_d_n4;
        locals.var_temp_dn6 = assign40450_e46210_d_n6;
        locals.var_temp_dn7 = assign40450_e46210_d_n7;
        locals.var_temp_dn8 = assign40450_e46210_d_n8;
        locals.var_temp_dn9 = assign40450_e46210_d_n9;

        let (assign40460_e46218, assign40460_e46218_d_n4, assign40460_e46218_d_n6, assign40460_e46218_d_n7, assign40460_e46218_d_n8, assign40460_e46218_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40460_e46214: f64 = (locals.var_qim__blk1016 * locals.var_esurf1__blk1027);
        let assign40460_e46216: f64 = (assign40460_e46214 * locals.var_temp);
        (assign40460_e46216, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn4)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn6)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn7)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn8)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf1__blk1027) + (locals.var_qim__blk1016 * locals.var_esurf1__blk1027_dn9)) * locals.var_temp) + (assign40460_e46214 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9,)
    }
};
        locals.var_qi1m__blk1029 = assign40460_e46218;
        locals.var_qi1m__blk1029_dn4 = assign40460_e46218_d_n4;
        locals.var_qi1m__blk1029_dn6 = assign40460_e46218_d_n6;
        locals.var_qi1m__blk1029_dn7 = assign40460_e46218_d_n7;
        locals.var_qi1m__blk1029_dn8 = assign40460_e46218_d_n8;
        locals.var_qi1m__blk1029_dn9 = assign40460_e46218_d_n9;

        let (assign40470_e46226, assign40470_e46226_d_n4, assign40470_e46226_d_n6, assign40470_e46226_d_n7, assign40470_e46226_d_n8, assign40470_e46226_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40470_e46222: f64 = (locals.var_qim__blk1016 * locals.var_esurf2__blk1028);
        let assign40470_e46224: f64 = (assign40470_e46222 * locals.var_temp);
        (assign40470_e46224, ((((locals.var_qim__blk1016_dn4 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn4)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn6)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn7)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn8)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_esurf2__blk1028) + (locals.var_qim__blk1016 * locals.var_esurf2__blk1028_dn9)) * locals.var_temp) + (assign40470_e46222 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9,)
    }
};
        locals.var_qi2m__blk1030 = assign40470_e46226;
        locals.var_qi2m__blk1030_dn4 = assign40470_e46226_d_n4;
        locals.var_qi2m__blk1030_dn6 = assign40470_e46226_d_n6;
        locals.var_qi2m__blk1030_dn7 = assign40470_e46226_d_n7;
        locals.var_qi2m__blk1030_dn8 = assign40470_e46226_d_n8;
        locals.var_qi2m__blk1030_dn9 = assign40470_e46226_d_n9;

    }

    pub(super) fn stamp_transient_block_110(
        locals: &mut StampLocals,
    ) {
        let (assign40480_e46234, assign40480_e46234_d_n4, assign40480_e46234_d_n6, assign40480_e46234_d_n7, assign40480_e46234_d_n8, assign40480_e46234_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40480_e46231: f64 = (locals.var_ecpl1s__blk954 + locals.var_ecpl1d__blk1023);
        let assign40480_e46232: f64 = (0.5 * assign40480_e46231);
        (assign40480_e46232, (0.5 * (locals.var_ecpl1s__blk954_dn4 + locals.var_ecpl1d__blk1023_dn4)), (0.5 * (locals.var_ecpl1s__blk954_dn6 + locals.var_ecpl1d__blk1023_dn6)), (0.5 * (locals.var_ecpl1s__blk954_dn7 + locals.var_ecpl1d__blk1023_dn7)), (0.5 * (locals.var_ecpl1s__blk954_dn8 + locals.var_ecpl1d__blk1023_dn8)), (0.5 * (locals.var_ecpl1s__blk954_dn9 + locals.var_ecpl1d__blk1023_dn9)),)
    } else {
        (locals.var_ecpl1__blk1031, locals.var_ecpl1__blk1031_dn4, locals.var_ecpl1__blk1031_dn6, locals.var_ecpl1__blk1031_dn7, locals.var_ecpl1__blk1031_dn8, locals.var_ecpl1__blk1031_dn9,)
    }
};
        locals.var_ecpl1__blk1031 = assign40480_e46234;
        locals.var_ecpl1__blk1031_dn4 = assign40480_e46234_d_n4;
        locals.var_ecpl1__blk1031_dn6 = assign40480_e46234_d_n6;
        locals.var_ecpl1__blk1031_dn7 = assign40480_e46234_d_n7;
        locals.var_ecpl1__blk1031_dn8 = assign40480_e46234_d_n8;
        locals.var_ecpl1__blk1031_dn9 = assign40480_e46234_d_n9;

        let (assign40490_e46242, assign40490_e46242_d_n4, assign40490_e46242_d_n6, assign40490_e46242_d_n7, assign40490_e46242_d_n8, assign40490_e46242_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40490_e46239: f64 = (locals.var_ecpl2s__blk955 + locals.var_ecpl2d__blk1024);
        let assign40490_e46240: f64 = (0.5 * assign40490_e46239);
        (assign40490_e46240, (0.5 * (locals.var_ecpl2s__blk955_dn4 + locals.var_ecpl2d__blk1024_dn4)), (0.5 * (locals.var_ecpl2s__blk955_dn6 + locals.var_ecpl2d__blk1024_dn6)), (0.5 * (locals.var_ecpl2s__blk955_dn7 + locals.var_ecpl2d__blk1024_dn7)), (0.5 * (locals.var_ecpl2s__blk955_dn8 + locals.var_ecpl2d__blk1024_dn8)), (0.5 * (locals.var_ecpl2s__blk955_dn9 + locals.var_ecpl2d__blk1024_dn9)),)
    } else {
        (locals.var_ecpl2__blk1032, locals.var_ecpl2__blk1032_dn4, locals.var_ecpl2__blk1032_dn6, locals.var_ecpl2__blk1032_dn7, locals.var_ecpl2__blk1032_dn8, locals.var_ecpl2__blk1032_dn9,)
    }
};
        locals.var_ecpl2__blk1032 = assign40490_e46242;
        locals.var_ecpl2__blk1032_dn4 = assign40490_e46242_d_n4;
        locals.var_ecpl2__blk1032_dn6 = assign40490_e46242_d_n6;
        locals.var_ecpl2__blk1032_dn7 = assign40490_e46242_d_n7;
        locals.var_ecpl2__blk1032_dn8 = assign40490_e46242_d_n8;
        locals.var_ecpl2__blk1032_dn9 = assign40490_e46242_d_n9;

        let (assign40500_e46250, assign40500_e46250_d_n4, assign40500_e46250_d_n6, assign40500_e46250_d_n7, assign40500_e46250_d_n8, assign40500_e46250_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40500_e46247: f64 = (locals.var_eeff1s__blk956 + locals.var_eeff1d__blk1025);
        let assign40500_e46248: f64 = (0.5 * assign40500_e46247);
        (assign40500_e46248, (0.5 * (locals.var_eeff1s__blk956_dn4 + locals.var_eeff1d__blk1025_dn4)), (0.5 * (locals.var_eeff1s__blk956_dn6 + locals.var_eeff1d__blk1025_dn6)), (0.5 * (locals.var_eeff1s__blk956_dn7 + locals.var_eeff1d__blk1025_dn7)), (0.5 * (locals.var_eeff1s__blk956_dn8 + locals.var_eeff1d__blk1025_dn8)), (0.5 * (locals.var_eeff1s__blk956_dn9 + locals.var_eeff1d__blk1025_dn9)),)
    } else {
        (locals.var_eeff1__blk1033, locals.var_eeff1__blk1033_dn4, locals.var_eeff1__blk1033_dn6, locals.var_eeff1__blk1033_dn7, locals.var_eeff1__blk1033_dn8, locals.var_eeff1__blk1033_dn9,)
    }
};
        locals.var_eeff1__blk1033 = assign40500_e46250;
        locals.var_eeff1__blk1033_dn4 = assign40500_e46250_d_n4;
        locals.var_eeff1__blk1033_dn6 = assign40500_e46250_d_n6;
        locals.var_eeff1__blk1033_dn7 = assign40500_e46250_d_n7;
        locals.var_eeff1__blk1033_dn8 = assign40500_e46250_d_n8;
        locals.var_eeff1__blk1033_dn9 = assign40500_e46250_d_n9;

        let (assign40510_e46258, assign40510_e46258_d_n4, assign40510_e46258_d_n6, assign40510_e46258_d_n7, assign40510_e46258_d_n8, assign40510_e46258_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40510_e46255: f64 = (locals.var_eeff2s__blk957 + locals.var_eeff2d__blk1026);
        let assign40510_e46256: f64 = (0.5 * assign40510_e46255);
        (assign40510_e46256, (0.5 * (locals.var_eeff2s__blk957_dn4 + locals.var_eeff2d__blk1026_dn4)), (0.5 * (locals.var_eeff2s__blk957_dn6 + locals.var_eeff2d__blk1026_dn6)), (0.5 * (locals.var_eeff2s__blk957_dn7 + locals.var_eeff2d__blk1026_dn7)), (0.5 * (locals.var_eeff2s__blk957_dn8 + locals.var_eeff2d__blk1026_dn8)), (0.5 * (locals.var_eeff2s__blk957_dn9 + locals.var_eeff2d__blk1026_dn9)),)
    } else {
        (locals.var_eeff2__blk1034, locals.var_eeff2__blk1034_dn4, locals.var_eeff2__blk1034_dn6, locals.var_eeff2__blk1034_dn7, locals.var_eeff2__blk1034_dn8, locals.var_eeff2__blk1034_dn9,)
    }
};
        locals.var_eeff2__blk1034 = assign40510_e46258;
        locals.var_eeff2__blk1034_dn4 = assign40510_e46258_d_n4;
        locals.var_eeff2__blk1034_dn6 = assign40510_e46258_d_n6;
        locals.var_eeff2__blk1034_dn7 = assign40510_e46258_d_n7;
        locals.var_eeff2__blk1034_dn8 = assign40510_e46258_d_n8;
        locals.var_eeff2__blk1034_dn9 = assign40510_e46258_d_n9;

        let (assign40520_e46271, assign40520_e46271_d_n4, assign40520_e46271_d_n6, assign40520_e46271_d_n7, assign40520_e46271_d_n8, assign40520_e46271_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40520_e46262: f64 = (locals.var_esurf1__blk1027 * locals.var_betn1_t);
        let assign40520_e46265: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign40520_e46266: f64 = (assign40520_e46265).exp();
        let assign40520_e46267: f64 = (assign40520_e46262 * assign40520_e46266);
        let assign40520_e46269: f64 = (assign40520_e46267 * locals.var_ratio_pd__blk1020);
        (assign40520_e46269, ((((((locals.var_esurf1__blk1027_dn4 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn4)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn4)), ((((((locals.var_esurf1__blk1027_dn6 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn6)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn6)), ((((((locals.var_esurf1__blk1027_dn7 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn7)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn7)), ((((((locals.var_esurf1__blk1027_dn8 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn8)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn8)), ((((((locals.var_esurf1__blk1027_dn9 * locals.var_betn1_t) + (locals.var_esurf1__blk1027 * locals.var_betn1_t_dn9)) * assign40520_e46266) + (assign40520_e46262 * (assign40520_e46266 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd__blk1020) + (assign40520_e46267 * locals.var_ratio_pd__blk1020_dn9)),)
    } else {
        (locals.var_c1__blk1035, locals.var_c1__blk1035_dn4, locals.var_c1__blk1035_dn6, locals.var_c1__blk1035_dn7, locals.var_c1__blk1035_dn8, locals.var_c1__blk1035_dn9,)
    }
};
        locals.var_c1__blk1035 = assign40520_e46271;
        locals.var_c1__blk1035_dn4 = assign40520_e46271_d_n4;
        locals.var_c1__blk1035_dn6 = assign40520_e46271_d_n6;
        locals.var_c1__blk1035_dn7 = assign40520_e46271_d_n7;
        locals.var_c1__blk1035_dn8 = assign40520_e46271_d_n8;
        locals.var_c1__blk1035_dn9 = assign40520_e46271_d_n9;

        let (assign40530_e46282, assign40530_e46282_d_n4, assign40530_e46282_d_n6, assign40530_e46282_d_n7, assign40530_e46282_d_n8, assign40530_e46282_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40530_e46275: f64 = (locals.var_esurf2__blk1028 * locals.var_betn2_t);
        let assign40530_e46278: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign40530_e46279: f64 = (assign40530_e46278).exp();
        let assign40530_e46280: f64 = (assign40530_e46275 * assign40530_e46279);
        (assign40530_e46280, ((((locals.var_esurf2__blk1028_dn4 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn4)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf2__blk1028_dn6 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn6)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf2__blk1028_dn7 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn7)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf2__blk1028_dn8 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn8)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf2__blk1028_dn9 * locals.var_betn2_t) + (locals.var_esurf2__blk1028 * locals.var_betn2_t_dn9)) * assign40530_e46279) + (assign40530_e46275 * (assign40530_e46279 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))),)
    } else {
        (locals.var_c2__blk1036, locals.var_c2__blk1036_dn4, locals.var_c2__blk1036_dn6, locals.var_c2__blk1036_dn7, locals.var_c2__blk1036_dn8, locals.var_c2__blk1036_dn9,)
    }
};
        locals.var_c2__blk1036 = assign40530_e46282;
        locals.var_c2__blk1036_dn4 = assign40530_e46282_d_n4;
        locals.var_c2__blk1036_dn6 = assign40530_e46282_d_n6;
        locals.var_c2__blk1036_dn7 = assign40530_e46282_d_n7;
        locals.var_c2__blk1036_dn8 = assign40530_e46282_d_n8;
        locals.var_c2__blk1036_dn9 = assign40530_e46282_d_n9;

        let (assign40540_e46288, assign40540_e46288_d_n4, assign40540_e46288_d_n6, assign40540_e46288_d_n7, assign40540_e46288_d_n8, assign40540_e46288_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40540_e46286: f64 = (locals.var_c1__blk1035 + locals.var_c2__blk1036);
        (assign40540_e46286, (locals.var_c1__blk1035_dn4 + locals.var_c2__blk1036_dn4), (locals.var_c1__blk1035_dn6 + locals.var_c2__blk1036_dn6), (locals.var_c1__blk1035_dn7 + locals.var_c2__blk1036_dn7), (locals.var_c1__blk1035_dn8 + locals.var_c2__blk1036_dn8), (locals.var_c1__blk1035_dn9 + locals.var_c2__blk1036_dn9),)
    } else {
        (locals.var_csum__blk1037, locals.var_csum__blk1037_dn4, locals.var_csum__blk1037_dn6, locals.var_csum__blk1037_dn7, locals.var_csum__blk1037_dn8, locals.var_csum__blk1037_dn9,)
    }
};
        locals.var_csum__blk1037 = assign40540_e46288;
        locals.var_csum__blk1037_dn4 = assign40540_e46288_d_n4;
        locals.var_csum__blk1037_dn6 = assign40540_e46288_d_n6;
        locals.var_csum__blk1037_dn7 = assign40540_e46288_d_n7;
        locals.var_csum__blk1037_dn8 = assign40540_e46288_d_n8;
        locals.var_csum__blk1037_dn9 = assign40540_e46288_d_n9;

        let (assign40550_e46298, assign40550_e46298_d_n4, assign40550_e46298_d_n6, assign40550_e46298_d_n7, assign40550_e46298_d_n8, assign40550_e46298_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40550_e46294: f64 = (locals.var_xcorb_i * locals.var_ecpl2__blk1032);
        let assign40550_e46295: f64 = (locals.var_ecpl1__blk1031 + assign40550_e46294);
        let assign40550_e46296: f64 = (locals.var_xcor_i * assign40550_e46295);
        (assign40550_e46296, ((locals.var_xcor_i_dn4 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn4 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn4)))), ((locals.var_xcor_i_dn6 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn6 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn6)))), ((locals.var_xcor_i_dn7 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn7 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn7)))), ((locals.var_xcor_i_dn8 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn8 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn8)))), ((locals.var_xcor_i_dn9 * assign40550_e46295) + (locals.var_xcor_i * (locals.var_ecpl1__blk1031_dn9 + (locals.var_xcorb_i * locals.var_ecpl2__blk1032_dn9)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40550_e46298;
        locals.var_temp1_dn4 = assign40550_e46298_d_n4;
        locals.var_temp1_dn6 = assign40550_e46298_d_n6;
        locals.var_temp1_dn7 = assign40550_e46298_d_n7;
        locals.var_temp1_dn8 = assign40550_e46298_d_n8;
        locals.var_temp1_dn9 = assign40550_e46298_d_n9;

        let (assign40560_e46323, assign40560_e46323_d_n4, assign40560_e46323_d_n6, assign40560_e46323_d_n7, assign40560_e46323_d_n8, assign40560_e46323_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40560_e46303: f64 = (1.0 + locals.var_temp1);
        let assign40560_e46305: f64 = assign40560_e46303;
        let assign40560_e46308: f64 = (1.0 + locals.var_temp1);
        let assign40560_e46310: f64 = assign40560_e46308;
        let assign40560_e46313: f64 = (1.0 + locals.var_temp1);
        let assign40560_e46315: f64 = assign40560_e46313;
        let assign40560_e46316: f64 = (assign40560_e46310 * assign40560_e46315);
        let assign40560_e46318: f64 = (assign40560_e46316 + 0.01);
        let assign40560_e46319: f64 = (assign40560_e46318).sqrt();
        let assign40560_e46320: f64 = (assign40560_e46305 + assign40560_e46319);
        let assign40560_e46321: f64 = (0.5 * assign40560_e46320);
        (assign40560_e46321, (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn4)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn6)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn7)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn8)) / (2.0 * assign40560_e46319)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign40560_e46315) + (assign40560_e46310 * locals.var_temp1_dn9)) / (2.0 * assign40560_e46319)))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40560_e46323;
        locals.var_temp2_dn4 = assign40560_e46323_d_n4;
        locals.var_temp2_dn6 = assign40560_e46323_d_n6;
        locals.var_temp2_dn7 = assign40560_e46323_d_n7;
        locals.var_temp2_dn8 = assign40560_e46323_d_n8;
        locals.var_temp2_dn9 = assign40560_e46323_d_n9;

        let (assign40570_e46354, assign40570_e46354_d_n4, assign40570_e46354_d_n6, assign40570_e46354_d_n7, assign40570_e46354_d_n8, assign40570_e46354_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40570_e46329: f64 = (0.2 * locals.var_temp1);
        let assign40570_e46330: f64 = (1.0 + assign40570_e46329);
        let assign40570_e46332: f64 = assign40570_e46330;
        let assign40570_e46336: f64 = (0.2 * locals.var_temp1);
        let assign40570_e46337: f64 = (1.0 + assign40570_e46336);
        let assign40570_e46339: f64 = assign40570_e46337;
        let assign40570_e46343: f64 = (0.2 * locals.var_temp1);
        let assign40570_e46344: f64 = (1.0 + assign40570_e46343);
        let assign40570_e46346: f64 = assign40570_e46344;
        let assign40570_e46347: f64 = (assign40570_e46339 * assign40570_e46346);
        let assign40570_e46349: f64 = (assign40570_e46347 + 0.01);
        let assign40570_e46350: f64 = (assign40570_e46349).sqrt();
        let assign40570_e46351: f64 = (assign40570_e46332 + assign40570_e46350);
        let assign40570_e46352: f64 = (0.5 * assign40570_e46351);
        (assign40570_e46352, (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign40570_e46350)))), (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign40570_e46346) + (assign40570_e46339 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign40570_e46350)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40570_e46354;
        locals.var_temp3_dn4 = assign40570_e46354_d_n4;
        locals.var_temp3_dn6 = assign40570_e46354_d_n6;
        locals.var_temp3_dn7 = assign40570_e46354_d_n7;
        locals.var_temp3_dn8 = assign40570_e46354_d_n8;
        locals.var_temp3_dn9 = assign40570_e46354_d_n9;

        let (assign40580_e46360, assign40580_e46360_d_n4, assign40580_e46360_d_n6, assign40580_e46360_d_n7, assign40580_e46360_d_n8, assign40580_e46360_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40580_e46358: f64 = (locals.var_temp2 / locals.var_temp3);
        (assign40580_e46358, (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3)),)
    } else {
        (locals.var_fcor__blk1038, locals.var_fcor__blk1038_dn4, locals.var_fcor__blk1038_dn6, locals.var_fcor__blk1038_dn7, locals.var_fcor__blk1038_dn8, locals.var_fcor__blk1038_dn9,)
    }
};
        locals.var_fcor__blk1038 = assign40580_e46360;
        locals.var_fcor__blk1038_dn4 = assign40580_e46360_d_n4;
        locals.var_fcor__blk1038_dn6 = assign40580_e46360_d_n6;
        locals.var_fcor__blk1038_dn7 = assign40580_e46360_d_n7;
        locals.var_fcor__blk1038_dn8 = assign40580_e46360_d_n8;
        locals.var_fcor__blk1038_dn9 = assign40580_e46360_d_n9;

        let (assign40590_e46389, assign40590_e46389_d_n4, assign40590_e46389_d_n6, assign40590_e46389_d_n7, assign40590_e46389_d_n8, assign40590_e46389_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40590_e46366: f64 = (locals.var_csfi_i * locals.var_ecpl1__blk1031);
        let assign40590_e46367: f64 = (1.0 + assign40590_e46366);
        let assign40590_e46370: f64 = (locals.var_csbi_i * locals.var_ecpl2__blk1032);
        let assign40590_e46371: f64 = (assign40590_e46367 + assign40590_e46370);
        let assign40590_e46372: f64 = (locals.var_cs_i * assign40590_e46371);
        let assign40590_e46374: f64 = (-locals.var_thecs_i);
        let assign40590_e46378: f64 = (locals.var_qi1m__blk1029 * locals.var_inv_qi1cs);
        let assign40590_e46379: f64 = (1.0 + assign40590_e46378);
        let assign40590_e46382: f64 = (locals.var_qi2m__blk1030 * locals.var_inv_qi2cs);
        let assign40590_e46383: f64 = (assign40590_e46379 + assign40590_e46382);
        let assign40590_e46384: f64 = (assign40590_e46383).ln();
        let assign40590_e46385: f64 = (assign40590_e46374 * assign40590_e46384);
        let assign40590_e46386: f64 = (assign40590_e46385).exp();
        let assign40590_e46387: f64 = (assign40590_e46372 * assign40590_e46386);
        (assign40590_e46387, ((((locals.var_cs_i_dn4 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn4) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn4)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn4) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn4 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn6 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn6) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn6)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn6) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn6 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn7 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn7) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn7)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn7) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn7 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn8 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn8) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn8)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn8) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn8 * locals.var_inv_qi2cs)) / assign40590_e46383)))))), ((((locals.var_cs_i_dn9 * assign40590_e46371) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1__blk1031_dn9) + (locals.var_csbi_i * locals.var_ecpl2__blk1032_dn9)))) * assign40590_e46386) + (assign40590_e46372 * (assign40590_e46386 * (((-locals.var_thecs_i_dn9) * assign40590_e46384) + (assign40590_e46374 * (((locals.var_qi1m__blk1029_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m__blk1030_dn9 * locals.var_inv_qi2cs)) / assign40590_e46383)))))),)
    } else {
        (locals.var_gcs__blk1039, locals.var_gcs__blk1039_dn4, locals.var_gcs__blk1039_dn6, locals.var_gcs__blk1039_dn7, locals.var_gcs__blk1039_dn8, locals.var_gcs__blk1039_dn9,)
    }
};
        locals.var_gcs__blk1039 = assign40590_e46389;
        locals.var_gcs__blk1039_dn4 = assign40590_e46389_d_n4;
        locals.var_gcs__blk1039_dn6 = assign40590_e46389_d_n6;
        locals.var_gcs__blk1039_dn7 = assign40590_e46389_d_n7;
        locals.var_gcs__blk1039_dn8 = assign40590_e46389_d_n8;
        locals.var_gcs__blk1039_dn9 = assign40590_e46389_d_n9;

        let assign40600_e46392: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign40600_e46392;

        let (assign40610_e46398, assign40610_e46398_d_n4, assign40610_e46398_d_n6, assign40610_e46398_d_n7, assign40610_e46398_d_n8, assign40610_e46398_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1222 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40610_e46398;
        locals.var_temp3_dn4 = assign40610_e46398_d_n4;
        locals.var_temp3_dn6 = assign40610_e46398_d_n6;
        locals.var_temp3_dn7 = assign40610_e46398_d_n7;
        locals.var_temp3_dn8 = assign40610_e46398_d_n8;
        locals.var_temp3_dn9 = assign40610_e46398_d_n9;

        let assign40620_e46401: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign40620_e46401;

        let (assign40630_e46418, assign40630_e46418_d_n4, assign40630_e46418_d_n6, assign40630_e46418_d_n7, assign40630_e46418_d_n8, assign40630_e46418_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign40630_e46412: f64 = (locals.var_qim__blk1016 + 1e-12);
        let assign40630_e46413: f64 = (assign40630_e46412).ln();
        let assign40630_e46414: f64 = (locals.var_thersg_i * assign40630_e46413);
        let assign40630_e46415: f64 = (assign40630_e46414).exp();
        let assign40630_e46416: f64 = (locals.var_rsg_i * assign40630_e46415);
        (assign40630_e46416, (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40630_e46412)))), (locals.var_rsg_i * (assign40630_e46415 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40630_e46412)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40630_e46418;
        locals.var_temp1_dn4 = assign40630_e46418_d_n4;
        locals.var_temp1_dn6 = assign40630_e46418_d_n6;
        locals.var_temp1_dn7 = assign40630_e46418_d_n7;
        locals.var_temp1_dn8 = assign40630_e46418_d_n8;
        locals.var_temp1_dn9 = assign40630_e46418_d_n9;

        let (assign40640_e46429, assign40640_e46429_d_n4, assign40640_e46429_d_n6, assign40640_e46429_d_n7, assign40640_e46429_d_n8, assign40640_e46429_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign40640_e46427: f64 = (1.0 - locals.var_temp1);
        (assign40640_e46427, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40640_e46429;
        locals.var_temp3_dn4 = assign40640_e46429_d_n4;
        locals.var_temp3_dn6 = assign40640_e46429_d_n6;
        locals.var_temp3_dn7 = assign40640_e46429_d_n7;
        locals.var_temp3_dn8 = assign40640_e46429_d_n8;
        locals.var_temp3_dn9 = assign40640_e46429_d_n9;

        let (assign40650_e46447, assign40650_e46447_d_n4, assign40650_e46447_d_n6, assign40650_e46447_d_n7, assign40650_e46447_d_n8, assign40650_e46447_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
        let assign40650_e46441: f64 = (locals.var_qim__blk1016 + 1e-12);
        let assign40650_e46442: f64 = (assign40650_e46441).ln();
        let assign40650_e46443: f64 = (locals.var_thersg_i * assign40650_e46442);
        let assign40650_e46444: f64 = (assign40650_e46443).exp();
        let assign40650_e46445: f64 = (locals.var_rsg_i * assign40650_e46444);
        (assign40650_e46445, (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn4 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn6 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn7 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn8 / assign40650_e46441)))), (locals.var_rsg_i * (assign40650_e46444 * (locals.var_thersg_i * (locals.var_qim__blk1016_dn9 / assign40650_e46441)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40650_e46447;
        locals.var_temp1_dn4 = assign40650_e46447_d_n4;
        locals.var_temp1_dn6 = assign40650_e46447_d_n6;
        locals.var_temp1_dn7 = assign40650_e46447_d_n7;
        locals.var_temp1_dn8 = assign40650_e46447_d_n8;
        locals.var_temp1_dn9 = assign40650_e46447_d_n9;

        let (assign40660_e46461, assign40660_e46461_d_n4, assign40660_e46461_d_n6, assign40660_e46461_d_n7, assign40660_e46461_d_n8, assign40660_e46461_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) {
        let assign40660_e46458: f64 = (1.0 + locals.var_temp1);
        let assign40660_e46459: f64 = (1.0 / assign40660_e46458);
        (assign40660_e46459, (-(locals.var_temp1_dn4 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn6 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn7 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn8 / (assign40660_e46458 * assign40660_e46458))), (-(locals.var_temp1_dn9 / (assign40660_e46458 * assign40660_e46458))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign40660_e46461;
        locals.var_temp3_dn4 = assign40660_e46461_d_n4;
        locals.var_temp3_dn6 = assign40660_e46461_d_n6;
        locals.var_temp3_dn7 = assign40660_e46461_d_n7;
        locals.var_temp3_dn8 = assign40660_e46461_d_n8;
        locals.var_temp3_dn9 = assign40660_e46461_d_n9;

        let (assign40670_e46471, assign40670_e46471_d_n4, assign40670_e46471_d_n6, assign40670_e46471_d_n7, assign40670_e46471_d_n8, assign40670_e46471_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40670_e46466: f64 = (locals.var_qim__blk1016 * locals.var_temp3);
        let assign40670_e46468: f64 = (assign40670_e46466 + locals.var_rsig_i);
        let assign40670_e46469: f64 = (locals.var_frscsi__blk964 * assign40670_e46468);
        (assign40670_e46469, ((locals.var_frscsi__blk964_dn4 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn4 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn4)))), ((locals.var_frscsi__blk964_dn6 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn6 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn6)))), ((locals.var_frscsi__blk964_dn7 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn7 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn7)))), ((locals.var_frscsi__blk964_dn8 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn8 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn8)))), ((locals.var_frscsi__blk964_dn9 * assign40670_e46468) + (locals.var_frscsi__blk964 * ((locals.var_qim__blk1016_dn9 * locals.var_temp3) + (locals.var_qim__blk1016 * locals.var_temp3_dn9)))),)
    } else {
        (locals.var_grs__blk1040, locals.var_grs__blk1040_dn4, locals.var_grs__blk1040_dn6, locals.var_grs__blk1040_dn7, locals.var_grs__blk1040_dn8, locals.var_grs__blk1040_dn9,)
    }
};
        locals.var_grs__blk1040 = assign40670_e46471;
        locals.var_grs__blk1040_dn4 = assign40670_e46471_d_n4;
        locals.var_grs__blk1040_dn6 = assign40670_e46471_d_n6;
        locals.var_grs__blk1040_dn7 = assign40670_e46471_d_n7;
        locals.var_grs__blk1040_dn8 = assign40670_e46471_d_n8;
        locals.var_grs__blk1040_dn9 = assign40670_e46471_d_n9;

        let (assign40680_e46491, assign40680_e46491_d_n4, assign40680_e46491_d_n6, assign40680_e46491_d_n7, assign40680_e46491_d_n8, assign40680_e46491_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40680_e46477: f64 = (locals.var_fmue * locals.var_eeff1__blk1033);
        let assign40680_e46479: f64 = (assign40680_e46477 + 1e-6);
        let assign40680_e46480: f64 = (assign40680_e46479).ln();
        let assign40680_e46481: f64 = (locals.var_themu_i * assign40680_e46480);
        let assign40680_e46482: f64 = (assign40680_e46481).exp();
        let assign40680_e46483: f64 = (1.0 + assign40680_e46482);
        let assign40680_e46485: f64 = (assign40680_e46483 + locals.var_gcs__blk1039);
        let assign40680_e46488: f64 = (locals.var_betn1_i * locals.var_grs__blk1040);
        let assign40680_e46489: f64 = (assign40680_e46485 + assign40680_e46488);
        (assign40680_e46489, (((assign40680_e46482 * ((locals.var_themu_i_dn4 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn4)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn4))), (((assign40680_e46482 * ((locals.var_themu_i_dn6 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn6)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn6))), (((assign40680_e46482 * ((locals.var_themu_i_dn7 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn7)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn7))), (((assign40680_e46482 * ((locals.var_themu_i_dn8 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn8)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn8))), (((assign40680_e46482 * ((locals.var_themu_i_dn9 * assign40680_e46480) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1__blk1033) + (locals.var_fmue * locals.var_eeff1__blk1033_dn9)) / assign40680_e46479)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn1_i * locals.var_grs__blk1040_dn9))),)
    } else {
        (locals.var_gmob1__blk1041, locals.var_gmob1__blk1041_dn4, locals.var_gmob1__blk1041_dn6, locals.var_gmob1__blk1041_dn7, locals.var_gmob1__blk1041_dn8, locals.var_gmob1__blk1041_dn9,)
    }
};
        locals.var_gmob1__blk1041 = assign40680_e46491;
        locals.var_gmob1__blk1041_dn4 = assign40680_e46491_d_n4;
        locals.var_gmob1__blk1041_dn6 = assign40680_e46491_d_n6;
        locals.var_gmob1__blk1041_dn7 = assign40680_e46491_d_n7;
        locals.var_gmob1__blk1041_dn8 = assign40680_e46491_d_n8;
        locals.var_gmob1__blk1041_dn9 = assign40680_e46491_d_n9;

        let (assign40690_e46511, assign40690_e46511_d_n4, assign40690_e46511_d_n6, assign40690_e46511_d_n7, assign40690_e46511_d_n8, assign40690_e46511_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40690_e46497: f64 = (locals.var_fmue * locals.var_eeff2__blk1034);
        let assign40690_e46499: f64 = (assign40690_e46497 + 1e-6);
        let assign40690_e46500: f64 = (assign40690_e46499).ln();
        let assign40690_e46501: f64 = (locals.var_themu_i * assign40690_e46500);
        let assign40690_e46502: f64 = (assign40690_e46501).exp();
        let assign40690_e46503: f64 = (1.0 + assign40690_e46502);
        let assign40690_e46505: f64 = (assign40690_e46503 + locals.var_gcs__blk1039);
        let assign40690_e46508: f64 = (locals.var_betn2_i * locals.var_grs__blk1040);
        let assign40690_e46509: f64 = (assign40690_e46505 + assign40690_e46508);
        (assign40690_e46509, (((assign40690_e46502 * ((locals.var_themu_i_dn4 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn4)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn4))), (((assign40690_e46502 * ((locals.var_themu_i_dn6 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn6)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn6))), (((assign40690_e46502 * ((locals.var_themu_i_dn7 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn7)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn7))), (((assign40690_e46502 * ((locals.var_themu_i_dn8 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn8)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn8))), (((assign40690_e46502 * ((locals.var_themu_i_dn9 * assign40690_e46500) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2__blk1034) + (locals.var_fmue * locals.var_eeff2__blk1034_dn9)) / assign40690_e46499)))) + locals.var_gcs__blk1039_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grs__blk1040) + (locals.var_betn2_i * locals.var_grs__blk1040_dn9))),)
    } else {
        (locals.var_gmob2__blk1042, locals.var_gmob2__blk1042_dn4, locals.var_gmob2__blk1042_dn6, locals.var_gmob2__blk1042_dn7, locals.var_gmob2__blk1042_dn8, locals.var_gmob2__blk1042_dn9,)
    }
};
        locals.var_gmob2__blk1042 = assign40690_e46511;
        locals.var_gmob2__blk1042_dn4 = assign40690_e46511_d_n4;
        locals.var_gmob2__blk1042_dn6 = assign40690_e46511_d_n6;
        locals.var_gmob2__blk1042_dn7 = assign40690_e46511_d_n7;
        locals.var_gmob2__blk1042_dn8 = assign40690_e46511_d_n8;
        locals.var_gmob2__blk1042_dn9 = assign40690_e46511_d_n9;

        let (assign40700_e46525, assign40700_e46525_d_n4, assign40700_e46525_d_n6, assign40700_e46525_d_n7, assign40700_e46525_d_n8, assign40700_e46525_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40700_e46515: f64 = (locals.var_fcor__blk1038 * locals.var_csum__blk1037);
        let assign40700_e46518: f64 = (locals.var_c1__blk1035 / locals.var_gmob1__blk1041);
        let assign40700_e46521: f64 = (locals.var_c2__blk1036 / locals.var_gmob2__blk1042);
        let assign40700_e46522: f64 = (assign40700_e46518 + assign40700_e46521);
        let assign40700_e46523: f64 = (assign40700_e46515 / assign40700_e46522);
        (assign40700_e46523, (((((locals.var_fcor__blk1038_dn4 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn4)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn4 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn4)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn4 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn4)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn6 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn6)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn6 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn6)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn6 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn6)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn7 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn7)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn7 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn7)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn7 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn7)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn8 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn8)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn8 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn8)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn8 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn8)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)), (((((locals.var_fcor__blk1038_dn9 * locals.var_csum__blk1037) + (locals.var_fcor__blk1038 * locals.var_csum__blk1037_dn9)) * assign40700_e46522) - (assign40700_e46515 * ((((locals.var_c1__blk1035_dn9 * locals.var_gmob1__blk1041) - (locals.var_c1__blk1035 * locals.var_gmob1__blk1041_dn9)) / (locals.var_gmob1__blk1041 * locals.var_gmob1__blk1041)) + (((locals.var_c2__blk1036_dn9 * locals.var_gmob2__blk1042) - (locals.var_c2__blk1036 * locals.var_gmob2__blk1042_dn9)) / (locals.var_gmob2__blk1042 * locals.var_gmob2__blk1042))))) / (assign40700_e46522 * assign40700_e46522)),)
    } else {
        (locals.var_gmob__blk1043, locals.var_gmob__blk1043_dn4, locals.var_gmob__blk1043_dn6, locals.var_gmob__blk1043_dn7, locals.var_gmob__blk1043_dn8, locals.var_gmob__blk1043_dn9,)
    }
};
        locals.var_gmob__blk1043 = assign40700_e46525;
        locals.var_gmob__blk1043_dn4 = assign40700_e46525_d_n4;
        locals.var_gmob__blk1043_dn6 = assign40700_e46525_d_n6;
        locals.var_gmob__blk1043_dn7 = assign40700_e46525_d_n7;
        locals.var_gmob__blk1043_dn8 = assign40700_e46525_d_n8;
        locals.var_gmob__blk1043_dn9 = assign40700_e46525_d_n9;

        let (assign40710_e46533, assign40710_e46533_d_n4, assign40710_e46533_d_n6, assign40710_e46533_d_n7, assign40710_e46533_d_n8, assign40710_e46533_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40710_e46530: f64 = (4.0 + locals.var_qim__blk1016);
        let assign40710_e46531: f64 = (1.0 / assign40710_e46530);
        (assign40710_e46531, (-(locals.var_qim__blk1016_dn4 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn6 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn7 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn8 / (assign40710_e46530 * assign40710_e46530))), (-(locals.var_qim__blk1016_dn9 / (assign40710_e46530 * assign40710_e46530))),)
    } else {
        (locals.var_inv_qimstar1__blk1044, locals.var_inv_qimstar1__blk1044_dn4, locals.var_inv_qimstar1__blk1044_dn6, locals.var_inv_qimstar1__blk1044_dn7, locals.var_inv_qimstar1__blk1044_dn8, locals.var_inv_qimstar1__blk1044_dn9,)
    }
};
        locals.var_inv_qimstar1__blk1044 = assign40710_e46533;
        locals.var_inv_qimstar1__blk1044_dn4 = assign40710_e46533_d_n4;
        locals.var_inv_qimstar1__blk1044_dn6 = assign40710_e46533_d_n6;
        locals.var_inv_qimstar1__blk1044_dn7 = assign40710_e46533_d_n7;
        locals.var_inv_qimstar1__blk1044_dn8 = assign40710_e46533_d_n8;
        locals.var_inv_qimstar1__blk1044_dn9 = assign40710_e46533_d_n9;

        let assign40720_e46536: f64 = if locals.var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign40720_e46536;

        let (assign40730_e46548, assign40730_e46548_d_n4, assign40730_e46548_d_n6, assign40730_e46548_d_n7, assign40730_e46548_d_n8, assign40730_e46548_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let assign40730_e46544: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
        let assign40730_e46545: f64 = (1.0 + assign40730_e46544);
        let assign40730_e46546: f64 = (1.0 / assign40730_e46545);
        (assign40730_e46546, (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn4) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn6) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn7) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn8) / (assign40730_e46545 * assign40730_e46545))), (-((locals.var_alpb_i * locals.var_qi2m__blk1030_dn9) / (assign40730_e46545 * assign40730_e46545))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40730_e46548;
        locals.var_temp_dn4 = assign40730_e46548_d_n4;
        locals.var_temp_dn6 = assign40730_e46548_d_n6;
        locals.var_temp_dn7 = assign40730_e46548_d_n7;
        locals.var_temp_dn8 = assign40730_e46548_d_n8;
        locals.var_temp_dn9 = assign40730_e46548_d_n9;

        let (assign40740_e46559, assign40740_e46559_d_n4, assign40740_e46559_d_n6, assign40740_e46559_d_n7, assign40740_e46559_d_n8, assign40740_e46559_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1224 == 0.0)) {
        let assign40740_e46556: f64 = (locals.var_alpb_i * locals.var_qi2m__blk1030);
        let assign40740_e46557: f64 = (1.0 - assign40740_e46556);
        (assign40740_e46557, (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn4)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn6)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn7)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn8)), (-(locals.var_alpb_i * locals.var_qi2m__blk1030_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign40740_e46559;
        locals.var_temp_dn4 = assign40740_e46559_d_n4;
        locals.var_temp_dn6 = assign40740_e46559_d_n6;
        locals.var_temp_dn7 = assign40740_e46559_d_n7;
        locals.var_temp_dn8 = assign40740_e46559_d_n8;
        locals.var_temp_dn9 = assign40740_e46559_d_n9;

        let (assign40750_e46567, assign40750_e46567_d_n4, assign40750_e46567_d_n6, assign40750_e46567_d_n7, assign40750_e46567_d_n8, assign40750_e46567_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40750_e46563: f64 = (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044);
        let assign40750_e46565: f64 = (assign40750_e46563 * locals.var_temp);
        (assign40750_e46565, ((((locals.var_qim__blk1016_dn4 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn4)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn4)), ((((locals.var_qim__blk1016_dn6 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn6)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn6)), ((((locals.var_qim__blk1016_dn7 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn7)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn7)), ((((locals.var_qim__blk1016_dn8 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn8)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn8)), ((((locals.var_qim__blk1016_dn9 * locals.var_inv_qimstar1__blk1044) + (locals.var_qim__blk1016 * locals.var_inv_qimstar1__blk1044_dn9)) * locals.var_temp) + (assign40750_e46563 * locals.var_temp_dn9)),)
    } else {
        (locals.var_r1__blk1045, locals.var_r1__blk1045_dn4, locals.var_r1__blk1045_dn6, locals.var_r1__blk1045_dn7, locals.var_r1__blk1045_dn8, locals.var_r1__blk1045_dn9,)
    }
};
        locals.var_r1__blk1045 = assign40750_e46567;
        locals.var_r1__blk1045_dn4 = assign40750_e46567_d_n4;
        locals.var_r1__blk1045_dn6 = assign40750_e46567_d_n6;
        locals.var_r1__blk1045_dn7 = assign40750_e46567_d_n7;
        locals.var_r1__blk1045_dn8 = assign40750_e46567_d_n8;
        locals.var_r1__blk1045_dn9 = assign40750_e46567_d_n9;

        let (assign40760_e46588, assign40760_e46588_d_n4, assign40760_e46588_d_n6, assign40760_e46588_d_n7, assign40760_e46588_d_n8, assign40760_e46588_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40760_e46572: f64 = (locals.var_xd - locals.var_xdeff__blk1000);
        let assign40760_e46575: f64 = (locals.var_vp_i * locals.var_inv_phit);
        let assign40760_e46578: f64 = (locals.var_vpg_i * locals.var_qim__blk1016);
        let assign40760_e46580: f64 = (assign40760_e46578 * locals.var_qim__blk1016);
        let assign40760_e46581: f64 = (assign40760_e46575 + assign40760_e46580);
        let assign40760_e46582: f64 = (assign40760_e46572 / assign40760_e46581);
        let assign40760_e46583: f64 = (1.0 + assign40760_e46582);
        let assign40760_e46584: f64 = (assign40760_e46583).ln();
        let assign40760_e46586: f64 = (assign40760_e46584 * locals.var_r1__blk1045);
        (assign40760_e46586, (((((((locals.var_xd_dn4 - locals.var_xdeff__blk1000_dn4) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn4) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn4) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn4))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn4)), (((((((locals.var_xd_dn6 - locals.var_xdeff__blk1000_dn6) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn6) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn6) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn6))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn6)), (((((((locals.var_xd_dn7 - locals.var_xdeff__blk1000_dn7) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn7) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn7) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn7))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn7)), (((((((locals.var_xd_dn8 - locals.var_xdeff__blk1000_dn8) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn8) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn8) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn8))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn8)), (((((((locals.var_xd_dn9 - locals.var_xdeff__blk1000_dn9) * assign40760_e46581) - (assign40760_e46572 * ((locals.var_vp_i * locals.var_inv_phit_dn9) + (((locals.var_vpg_i * locals.var_qim__blk1016_dn9) * locals.var_qim__blk1016) + (assign40760_e46578 * locals.var_qim__blk1016_dn9))))) / (assign40760_e46581 * assign40760_e46581)) / assign40760_e46583) * locals.var_r1__blk1045) + (assign40760_e46584 * locals.var_r1__blk1045_dn9)),)
    } else {
        (locals.var_dl_l_fact__blk1046, locals.var_dl_l_fact__blk1046_dn4, locals.var_dl_l_fact__blk1046_dn6, locals.var_dl_l_fact__blk1046_dn7, locals.var_dl_l_fact__blk1046_dn8, locals.var_dl_l_fact__blk1046_dn9,)
    }
};
        locals.var_dl_l_fact__blk1046 = assign40760_e46588;
        locals.var_dl_l_fact__blk1046_dn4 = assign40760_e46588_d_n4;
        locals.var_dl_l_fact__blk1046_dn6 = assign40760_e46588_d_n6;
        locals.var_dl_l_fact__blk1046_dn7 = assign40760_e46588_d_n7;
        locals.var_dl_l_fact__blk1046_dn8 = assign40760_e46588_d_n8;
        locals.var_dl_l_fact__blk1046_dn9 = assign40760_e46588_d_n9;

        let (assign40770_e46594, assign40770_e46594_d_n4, assign40770_e46594_d_n6, assign40770_e46594_d_n7, assign40770_e46594_d_n8, assign40770_e46594_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40770_e46592: f64 = (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046);
        (assign40770_e46592, (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn4), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn6), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn7), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn8), (locals.var_alp_loc__blk898 * locals.var_dl_l_fact__blk1046_dn9),)
    } else {
        (locals.var_dl_l__blk1047, locals.var_dl_l__blk1047_dn4, locals.var_dl_l__blk1047_dn6, locals.var_dl_l__blk1047_dn7, locals.var_dl_l__blk1047_dn8, locals.var_dl_l__blk1047_dn9,)
    }
};
        locals.var_dl_l__blk1047 = assign40770_e46594;
        locals.var_dl_l__blk1047_dn4 = assign40770_e46594_d_n4;
        locals.var_dl_l__blk1047_dn6 = assign40770_e46594_d_n6;
        locals.var_dl_l__blk1047_dn7 = assign40770_e46594_d_n7;
        locals.var_dl_l__blk1047_dn8 = assign40770_e46594_d_n8;
        locals.var_dl_l__blk1047_dn9 = assign40770_e46594_d_n9;

    }

    pub(super) fn stamp_transient_block_111(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign40780_e46606, assign40780_e46606_d_n4, assign40780_e46606_d_n6, assign40780_e46606_d_n7, assign40780_e46606_d_n8, assign40780_e46606_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40780_e46601: f64 = (1.0 + locals.var_dl_l__blk1047);
        let assign40780_e46602: f64 = (locals.var_dl_l__blk1047 * assign40780_e46601);
        let assign40780_e46603: f64 = (1.0 + assign40780_e46602);
        let assign40780_e46604: f64 = (1.0 / assign40780_e46603);
        (assign40780_e46604, (-(((locals.var_dl_l__blk1047_dn4 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn4)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn6 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn6)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn7 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn7)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn8 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn8)) / (assign40780_e46603 * assign40780_e46603))), (-(((locals.var_dl_l__blk1047_dn9 * assign40780_e46601) + (locals.var_dl_l__blk1047 * locals.var_dl_l__blk1047_dn9)) / (assign40780_e46603 * assign40780_e46603))),)
    } else {
        (locals.var_gdl__blk1048, locals.var_gdl__blk1048_dn4, locals.var_gdl__blk1048_dn6, locals.var_gdl__blk1048_dn7, locals.var_gdl__blk1048_dn8, locals.var_gdl__blk1048_dn9,)
    }
};
        locals.var_gdl__blk1048 = assign40780_e46606;
        locals.var_gdl__blk1048_dn4 = assign40780_e46606_d_n4;
        locals.var_gdl__blk1048_dn6 = assign40780_e46606_d_n6;
        locals.var_gdl__blk1048_dn7 = assign40780_e46606_d_n7;
        locals.var_gdl__blk1048_dn8 = assign40780_e46606_d_n8;
        locals.var_gdl__blk1048_dn9 = assign40780_e46606_d_n9;

        let (assign40790_e46616, assign40790_e46616_d_n4, assign40790_e46616_d_n6, assign40790_e46616_d_n7, assign40790_e46616_d_n8, assign40790_e46616_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40790_e46610: f64 = (100.0 * locals.var_esurf1__blk1027);
        let assign40790_e46613: f64 = (100.0 + locals.var_esurf1__blk1027);
        let assign40790_e46614: f64 = (assign40790_e46610 / assign40790_e46613);
        (assign40790_e46614, ((((100.0 * locals.var_esurf1__blk1027_dn4) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn4)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn6) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn6)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn7) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn7)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn8) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn8)) / (assign40790_e46613 * assign40790_e46613)), ((((100.0 * locals.var_esurf1__blk1027_dn9) * assign40790_e46613) - (assign40790_e46610 * locals.var_esurf1__blk1027_dn9)) / (assign40790_e46613 * assign40790_e46613)),)
    } else {
        (locals.var_wsat1__blk976, locals.var_wsat1__blk976_dn4, locals.var_wsat1__blk976_dn6, locals.var_wsat1__blk976_dn7, locals.var_wsat1__blk976_dn8, locals.var_wsat1__blk976_dn9,)
    }
};
        locals.var_wsat1__blk976 = assign40790_e46616;
        locals.var_wsat1__blk976_dn4 = assign40790_e46616_d_n4;
        locals.var_wsat1__blk976_dn6 = assign40790_e46616_d_n6;
        locals.var_wsat1__blk976_dn7 = assign40790_e46616_d_n7;
        locals.var_wsat1__blk976_dn8 = assign40790_e46616_d_n8;
        locals.var_wsat1__blk976_dn9 = assign40790_e46616_d_n9;

        let assign40800_e46619: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign40800_e46619;

        let (assign40810_e46631, assign40810_e46631_d_n4, assign40810_e46631_d_n6, assign40810_e46631_d_n7, assign40810_e46631_d_n8, assign40810_e46631_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign40810_e46627: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
        let assign40810_e46628: f64 = (1.0 - assign40810_e46627);
        let assign40810_e46629: f64 = (1.0 / assign40810_e46628);
        (assign40810_e46629, (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn4)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn6)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn7)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn8)) / (assign40810_e46628 * assign40810_e46628))), (-((-(locals.var_thesat1_i * locals.var_wsat1__blk976_dn9)) / (assign40810_e46628 * assign40810_e46628))),)
    } else {
        (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9,)
    }
};
        locals.var_sat_fact1__blk977 = assign40810_e46631;
        locals.var_sat_fact1__blk977_dn4 = assign40810_e46631_d_n4;
        locals.var_sat_fact1__blk977_dn6 = assign40810_e46631_d_n6;
        locals.var_sat_fact1__blk977_dn7 = assign40810_e46631_d_n7;
        locals.var_sat_fact1__blk977_dn8 = assign40810_e46631_d_n8;
        locals.var_sat_fact1__blk977_dn9 = assign40810_e46631_d_n9;

        let (assign40820_e46642, assign40820_e46642_d_n4, assign40820_e46642_d_n6, assign40820_e46642_d_n7, assign40820_e46642_d_n8, assign40820_e46642_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1225 == 0.0)) {
        let assign40820_e46639: f64 = (locals.var_thesat1_i * locals.var_wsat1__blk976);
        let assign40820_e46640: f64 = (1.0 + assign40820_e46639);
        (assign40820_e46640, (locals.var_thesat1_i * locals.var_wsat1__blk976_dn4), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn6), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn7), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn8), (locals.var_thesat1_i * locals.var_wsat1__blk976_dn9),)
    } else {
        (locals.var_sat_fact1__blk977, locals.var_sat_fact1__blk977_dn4, locals.var_sat_fact1__blk977_dn6, locals.var_sat_fact1__blk977_dn7, locals.var_sat_fact1__blk977_dn8, locals.var_sat_fact1__blk977_dn9,)
    }
};
        locals.var_sat_fact1__blk977 = assign40820_e46642;
        locals.var_sat_fact1__blk977_dn4 = assign40820_e46642_d_n4;
        locals.var_sat_fact1__blk977_dn6 = assign40820_e46642_d_n6;
        locals.var_sat_fact1__blk977_dn7 = assign40820_e46642_d_n7;
        locals.var_sat_fact1__blk977_dn8 = assign40820_e46642_d_n8;
        locals.var_sat_fact1__blk977_dn9 = assign40820_e46642_d_n9;

        let (assign40830_e46652, assign40830_e46652_d_n4, assign40830_e46652_d_n6, assign40830_e46652_d_n7, assign40830_e46652_d_n8, assign40830_e46652_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40830_e46646: f64 = (100.0 * locals.var_esurf2__blk1028);
        let assign40830_e46649: f64 = (100.0 + locals.var_esurf2__blk1028);
        let assign40830_e46650: f64 = (assign40830_e46646 / assign40830_e46649);
        (assign40830_e46650, ((((100.0 * locals.var_esurf2__blk1028_dn4) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn4)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn6) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn6)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn7) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn7)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn8) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn8)) / (assign40830_e46649 * assign40830_e46649)), ((((100.0 * locals.var_esurf2__blk1028_dn9) * assign40830_e46649) - (assign40830_e46646 * locals.var_esurf2__blk1028_dn9)) / (assign40830_e46649 * assign40830_e46649)),)
    } else {
        (locals.var_wsat2__blk978, locals.var_wsat2__blk978_dn4, locals.var_wsat2__blk978_dn6, locals.var_wsat2__blk978_dn7, locals.var_wsat2__blk978_dn8, locals.var_wsat2__blk978_dn9,)
    }
};
        locals.var_wsat2__blk978 = assign40830_e46652;
        locals.var_wsat2__blk978_dn4 = assign40830_e46652_d_n4;
        locals.var_wsat2__blk978_dn6 = assign40830_e46652_d_n6;
        locals.var_wsat2__blk978_dn7 = assign40830_e46652_d_n7;
        locals.var_wsat2__blk978_dn8 = assign40830_e46652_d_n8;
        locals.var_wsat2__blk978_dn9 = assign40830_e46652_d_n9;

        let assign40840_e46655: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign40840_e46655;

        let (assign40850_e46667, assign40850_e46667_d_n4, assign40850_e46667_d_n6, assign40850_e46667_d_n7, assign40850_e46667_d_n8, assign40850_e46667_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign40850_e46663: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
        let assign40850_e46664: f64 = (1.0 - assign40850_e46663);
        let assign40850_e46665: f64 = (1.0 / assign40850_e46664);
        (assign40850_e46665, (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn4)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn6)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn7)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn8)) / (assign40850_e46664 * assign40850_e46664))), (-((-(locals.var_thesat2_i * locals.var_wsat2__blk978_dn9)) / (assign40850_e46664 * assign40850_e46664))),)
    } else {
        (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9,)
    }
};
        locals.var_sat_fact2__blk979 = assign40850_e46667;
        locals.var_sat_fact2__blk979_dn4 = assign40850_e46667_d_n4;
        locals.var_sat_fact2__blk979_dn6 = assign40850_e46667_d_n6;
        locals.var_sat_fact2__blk979_dn7 = assign40850_e46667_d_n7;
        locals.var_sat_fact2__blk979_dn8 = assign40850_e46667_d_n8;
        locals.var_sat_fact2__blk979_dn9 = assign40850_e46667_d_n9;

        let (assign40860_e46678, assign40860_e46678_d_n4, assign40860_e46678_d_n6, assign40860_e46678_d_n7, assign40860_e46678_d_n8, assign40860_e46678_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1226 == 0.0)) {
        let assign40860_e46675: f64 = (locals.var_thesat2_i * locals.var_wsat2__blk978);
        let assign40860_e46676: f64 = (1.0 + assign40860_e46675);
        (assign40860_e46676, (locals.var_thesat2_i * locals.var_wsat2__blk978_dn4), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn6), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn7), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn8), (locals.var_thesat2_i * locals.var_wsat2__blk978_dn9),)
    } else {
        (locals.var_sat_fact2__blk979, locals.var_sat_fact2__blk979_dn4, locals.var_sat_fact2__blk979_dn6, locals.var_sat_fact2__blk979_dn7, locals.var_sat_fact2__blk979_dn8, locals.var_sat_fact2__blk979_dn9,)
    }
};
        locals.var_sat_fact2__blk979 = assign40860_e46678;
        locals.var_sat_fact2__blk979_dn4 = assign40860_e46678_d_n4;
        locals.var_sat_fact2__blk979_dn6 = assign40860_e46678_d_n6;
        locals.var_sat_fact2__blk979_dn7 = assign40860_e46678_d_n7;
        locals.var_sat_fact2__blk979_dn8 = assign40860_e46678_d_n8;
        locals.var_sat_fact2__blk979_dn9 = assign40860_e46678_d_n9;

        let (assign40870_e46690, assign40870_e46690_d_n4, assign40870_e46690_d_n6, assign40870_e46690_d_n7, assign40870_e46690_d_n8, assign40870_e46690_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40870_e46682: f64 = (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017);
        let assign40870_e46684: f64 = (assign40870_e46682 * 0.5);
        let assign40870_e46687: f64 = (locals.var_sat_fact1__blk977 + locals.var_sat_fact2__blk979);
        let assign40870_e46688: f64 = (assign40870_e46684 * assign40870_e46687);
        (assign40870_e46688, (((((locals.var_sat_phit_loc__blk896_dn4 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn4)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn4 + locals.var_sat_fact2__blk979_dn4))), (((((locals.var_sat_phit_loc__blk896_dn6 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn6)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn6 + locals.var_sat_fact2__blk979_dn6))), (((((locals.var_sat_phit_loc__blk896_dn7 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn7)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn7 + locals.var_sat_fact2__blk979_dn7))), (((((locals.var_sat_phit_loc__blk896_dn8 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn8)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn8 + locals.var_sat_fact2__blk979_dn8))), (((((locals.var_sat_phit_loc__blk896_dn9 * locals.var_dxdrift__blk1017) + (locals.var_sat_phit_loc__blk896 * locals.var_dxdrift__blk1017_dn9)) * 0.5) * assign40870_e46687) + (assign40870_e46684 * (locals.var_sat_fact1__blk977_dn9 + locals.var_sat_fact2__blk979_dn9))),)
    } else {
        (locals.var_ggamma__blk1049, locals.var_ggamma__blk1049_dn4, locals.var_ggamma__blk1049_dn6, locals.var_ggamma__blk1049_dn7, locals.var_ggamma__blk1049_dn8, locals.var_ggamma__blk1049_dn9,)
    }
};
        locals.var_ggamma__blk1049 = assign40870_e46690;
        locals.var_ggamma__blk1049_dn4 = assign40870_e46690_d_n4;
        locals.var_ggamma__blk1049_dn6 = assign40870_e46690_d_n6;
        locals.var_ggamma__blk1049_dn7 = assign40870_e46690_d_n7;
        locals.var_ggamma__blk1049_dn8 = assign40870_e46690_d_n8;
        locals.var_ggamma__blk1049_dn9 = assign40870_e46690_d_n9;

        let (assign40880_e46698, assign40880_e46698_d_n4, assign40880_e46698_d_n6, assign40880_e46698_d_n7, assign40880_e46698_d_n8, assign40880_e46698_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40880_e46695: f64 = (locals.var_gmob__blk1043 * locals.var_gdl__blk1048);
        let assign40880_e46696: f64 = (locals.var_ggamma__blk1049 / assign40880_e46695);
        (assign40880_e46696, (((locals.var_ggamma__blk1049_dn4 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn4 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn4)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn6 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn6 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn6)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn7 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn7 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn7)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn8 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn8 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn8)))) / (assign40880_e46695 * assign40880_e46695)), (((locals.var_ggamma__blk1049_dn9 * assign40880_e46695) - (locals.var_ggamma__blk1049 * ((locals.var_gmob__blk1043_dn9 * locals.var_gdl__blk1048) + (locals.var_gmob__blk1043 * locals.var_gdl__blk1048_dn9)))) / (assign40880_e46695 * assign40880_e46695)),)
    } else {
        (locals.var_sqrt_zsat__blk1050, locals.var_sqrt_zsat__blk1050_dn4, locals.var_sqrt_zsat__blk1050_dn6, locals.var_sqrt_zsat__blk1050_dn7, locals.var_sqrt_zsat__blk1050_dn8, locals.var_sqrt_zsat__blk1050_dn9,)
    }
};
        locals.var_sqrt_zsat__blk1050 = assign40880_e46698;
        locals.var_sqrt_zsat__blk1050_dn4 = assign40880_e46698_d_n4;
        locals.var_sqrt_zsat__blk1050_dn6 = assign40880_e46698_d_n6;
        locals.var_sqrt_zsat__blk1050_dn7 = assign40880_e46698_d_n7;
        locals.var_sqrt_zsat__blk1050_dn8 = assign40880_e46698_d_n8;
        locals.var_sqrt_zsat__blk1050_dn9 = assign40880_e46698_d_n9;

        let (assign40890_e46704, assign40890_e46704_d_n4, assign40890_e46704_d_n6, assign40890_e46704_d_n7, assign40890_e46704_d_n8, assign40890_e46704_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40890_e46702: f64 = (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050);
        (assign40890_e46702, ((locals.var_sqrt_zsat__blk1050_dn4 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn4)), ((locals.var_sqrt_zsat__blk1050_dn6 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn6)), ((locals.var_sqrt_zsat__blk1050_dn7 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn7)), ((locals.var_sqrt_zsat__blk1050_dn8 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn8)), ((locals.var_sqrt_zsat__blk1050_dn9 * locals.var_sqrt_zsat__blk1050) + (locals.var_sqrt_zsat__blk1050 * locals.var_sqrt_zsat__blk1050_dn9)),)
    } else {
        (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9,)
    }
};
        locals.var_zsat__blk1051 = assign40890_e46704;
        locals.var_zsat__blk1051_dn4 = assign40890_e46704_d_n4;
        locals.var_zsat__blk1051_dn6 = assign40890_e46704_d_n6;
        locals.var_zsat__blk1051_dn7 = assign40890_e46704_d_n7;
        locals.var_zsat__blk1051_dn8 = assign40890_e46704_d_n8;
        locals.var_zsat__blk1051_dn9 = assign40890_e46704_d_n9;

        let (assign40900_e46711, assign40900_e46711_d_n4, assign40900_e46711_d_n6, assign40900_e46711_d_n7, assign40900_e46711_d_n8, assign40900_e46711_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40900_e46708: f64 = (1.0 + locals.var_zsat__blk1051);
        let assign40900_e46709: f64 = (assign40900_e46708).sqrt();
        (assign40900_e46709, (locals.var_zsat__blk1051_dn4 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn6 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn7 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn8 / (2.0 * assign40900_e46709)), (locals.var_zsat__blk1051_dn9 / (2.0 * assign40900_e46709)),)
    } else {
        (locals.var_vsat_fact__blk1052, locals.var_vsat_fact__blk1052_dn4, locals.var_vsat_fact__blk1052_dn6, locals.var_vsat_fact__blk1052_dn7, locals.var_vsat_fact__blk1052_dn8, locals.var_vsat_fact__blk1052_dn9,)
    }
};
        locals.var_vsat_fact__blk1052 = assign40900_e46711;
        locals.var_vsat_fact__blk1052_dn4 = assign40900_e46711_d_n4;
        locals.var_vsat_fact__blk1052_dn6 = assign40900_e46711_d_n6;
        locals.var_vsat_fact__blk1052_dn7 = assign40900_e46711_d_n7;
        locals.var_vsat_fact__blk1052_dn8 = assign40900_e46711_d_n8;
        locals.var_vsat_fact__blk1052_dn9 = assign40900_e46711_d_n9;

        let (assign40910_e46721, assign40910_e46721_d_n4, assign40910_e46721_d_n6, assign40910_e46721_d_n7, assign40910_e46721_d_n8, assign40910_e46721_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign40910_e46716: f64 = (1.5 * locals.var_zsat__blk1051);
        let assign40910_e46717: f64 = (1.0 + assign40910_e46716);
        let assign40910_e46719: f64 = (assign40910_e46717 / locals.var_vsat_fact__blk1052);
        (assign40910_e46719, ((((1.5 * locals.var_zsat__blk1051_dn4) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn4)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn6) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn6)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn7) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn7)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn8) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn8)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)), ((((1.5 * locals.var_zsat__blk1051_dn9) * locals.var_vsat_fact__blk1052) - (assign40910_e46717 * locals.var_vsat_fact__blk1052_dn9)) / (locals.var_vsat_fact__blk1052 * locals.var_vsat_fact__blk1052)),)
    } else {
        (locals.var_hsat__blk1053, locals.var_hsat__blk1053_dn4, locals.var_hsat__blk1053_dn6, locals.var_hsat__blk1053_dn7, locals.var_hsat__blk1053_dn8, locals.var_hsat__blk1053_dn9,)
    }
};
        locals.var_hsat__blk1053 = assign40910_e46721;
        locals.var_hsat__blk1053_dn4 = assign40910_e46721_d_n4;
        locals.var_hsat__blk1053_dn6 = assign40910_e46721_d_n6;
        locals.var_hsat__blk1053_dn7 = assign40910_e46721_d_n7;
        locals.var_hsat__blk1053_dn8 = assign40910_e46721_d_n8;
        locals.var_hsat__blk1053_dn9 = assign40910_e46721_d_n9;

        let assign40920_e46724: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign40920_e46724;

        let (assign40930_e46743, assign40930_e46743_d_n4, assign40930_e46743_d_n6, assign40930_e46743_d_n7, assign40930_e46743_d_n8, assign40930_e46743_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40930_e46730: f64 = (0.6 * locals.var_qq);
        let assign40930_e46732: f64 = (-0.1666666666667);
        let assign40930_e46735: f64 = (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027);
        let assign40930_e46737: f64 = (assign40930_e46735 + 60.0);
        let assign40930_e46738: f64 = (assign40930_e46737).ln();
        let assign40930_e46739: f64 = (assign40930_e46732 * assign40930_e46738);
        let assign40930_e46740: f64 = (assign40930_e46739).exp();
        let assign40930_e46741: f64 = (assign40930_e46730 * assign40930_e46740);
        (assign40930_e46741, (((0.6 * locals.var_qq_dn4) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn4 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn4)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn6) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn6 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn6)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn7) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn7 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn7)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn8) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn8 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn8)) / assign40930_e46737))))), (((0.6 * locals.var_qq_dn9) * assign40930_e46740) + (assign40930_e46730 * (assign40930_e46740 * (assign40930_e46732 * (((locals.var_esurf1__blk1027_dn9 * locals.var_esurf1__blk1027) + (locals.var_esurf1__blk1027 * locals.var_esurf1__blk1027_dn9)) / assign40930_e46737))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign40930_e46743;
        locals.var_temp1_dn4 = assign40930_e46743_d_n4;
        locals.var_temp1_dn6 = assign40930_e46743_d_n6;
        locals.var_temp1_dn7 = assign40930_e46743_d_n7;
        locals.var_temp1_dn8 = assign40930_e46743_d_n8;
        locals.var_temp1_dn9 = assign40930_e46743_d_n9;

        let (assign40940_e46762, assign40940_e46762_d_n4, assign40940_e46762_d_n6, assign40940_e46762_d_n7, assign40940_e46762_d_n8, assign40940_e46762_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40940_e46749: f64 = (0.6 * locals.var_qq);
        let assign40940_e46751: f64 = (-0.1666666666667);
        let assign40940_e46754: f64 = (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028);
        let assign40940_e46756: f64 = (assign40940_e46754 + 60.0);
        let assign40940_e46757: f64 = (assign40940_e46756).ln();
        let assign40940_e46758: f64 = (assign40940_e46751 * assign40940_e46757);
        let assign40940_e46759: f64 = (assign40940_e46758).exp();
        let assign40940_e46760: f64 = (assign40940_e46749 * assign40940_e46759);
        (assign40940_e46760, (((0.6 * locals.var_qq_dn4) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn4 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn4)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn6) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn6 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn6)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn7) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn7 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn7)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn8) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn8 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn8)) / assign40940_e46756))))), (((0.6 * locals.var_qq_dn9) * assign40940_e46759) + (assign40940_e46749 * (assign40940_e46759 * (assign40940_e46751 * (((locals.var_esurf2__blk1028_dn9 * locals.var_esurf2__blk1028) + (locals.var_esurf2__blk1028 * locals.var_esurf2__blk1028_dn9)) / assign40940_e46756))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign40940_e46762;
        locals.var_temp2_dn4 = assign40940_e46762_d_n4;
        locals.var_temp2_dn6 = assign40940_e46762_d_n6;
        locals.var_temp2_dn7 = assign40940_e46762_d_n7;
        locals.var_temp2_dn8 = assign40940_e46762_d_n8;
        locals.var_temp2_dn9 = assign40940_e46762_d_n9;

        let (assign40950_e46774, assign40950_e46774_d_n4, assign40950_e46774_d_n6, assign40950_e46774_d_n7, assign40950_e46774_d_n8, assign40950_e46774_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40950_e46769: f64 = (locals.var_k1__blk932 * locals.var_temp1);
        let assign40950_e46770: f64 = (1.0 + assign40950_e46769);
        let assign40950_e46772: f64 = (assign40950_e46770 / locals.var_tox1fact__blk913);
        (assign40950_e46772, (((((locals.var_k1__blk932_dn4 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn4)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn4)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn6 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn6)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn6)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn7 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn7)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn7)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn8 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn8)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn8)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)), (((((locals.var_k1__blk932_dn9 * locals.var_temp1) + (locals.var_k1__blk932 * locals.var_temp1_dn9)) * locals.var_tox1fact__blk913) - (assign40950_e46770 * locals.var_tox1fact__blk913_dn9)) / (locals.var_tox1fact__blk913 * locals.var_tox1fact__blk913)),)
    } else {
        (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9,)
    }
};
        locals.var_qmfact1__blk1054 = assign40950_e46774;
        locals.var_qmfact1__blk1054_dn4 = assign40950_e46774_d_n4;
        locals.var_qmfact1__blk1054_dn6 = assign40950_e46774_d_n6;
        locals.var_qmfact1__blk1054_dn7 = assign40950_e46774_d_n7;
        locals.var_qmfact1__blk1054_dn8 = assign40950_e46774_d_n8;
        locals.var_qmfact1__blk1054_dn9 = assign40950_e46774_d_n9;

        let (assign40960_e46786, assign40960_e46786_d_n4, assign40960_e46786_d_n6, assign40960_e46786_d_n7, assign40960_e46786_d_n8, assign40960_e46786_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 != 0.0)) {
        let assign40960_e46781: f64 = (locals.var_k2__blk933 * locals.var_temp2);
        let assign40960_e46782: f64 = (1.0 + assign40960_e46781);
        let assign40960_e46784: f64 = (assign40960_e46782 / locals.var_tox2fact__blk914);
        (assign40960_e46784, (((((locals.var_k2__blk933_dn4 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn4)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn4)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn6 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn6)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn6)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn7 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn7)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn7)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn8 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn8)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn8)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)), (((((locals.var_k2__blk933_dn9 * locals.var_temp2) + (locals.var_k2__blk933 * locals.var_temp2_dn9)) * locals.var_tox2fact__blk914) - (assign40960_e46782 * locals.var_tox2fact__blk914_dn9)) / (locals.var_tox2fact__blk914 * locals.var_tox2fact__blk914)),)
    } else {
        (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9,)
    }
};
        locals.var_qmfact2__blk1055 = assign40960_e46786;
        locals.var_qmfact2__blk1055_dn4 = assign40960_e46786_d_n4;
        locals.var_qmfact2__blk1055_dn6 = assign40960_e46786_d_n6;
        locals.var_qmfact2__blk1055_dn7 = assign40960_e46786_d_n7;
        locals.var_qmfact2__blk1055_dn8 = assign40960_e46786_d_n8;
        locals.var_qmfact2__blk1055_dn9 = assign40960_e46786_d_n9;

        let (assign40970_e46793, assign40970_e46793_d_n4, assign40970_e46793_d_n6, assign40970_e46793_d_n7, assign40970_e46793_d_n8, assign40970_e46793_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9,)
    }
};
        locals.var_qmfact1__blk1054 = assign40970_e46793;
        locals.var_qmfact1__blk1054_dn4 = assign40970_e46793_d_n4;
        locals.var_qmfact1__blk1054_dn6 = assign40970_e46793_d_n6;
        locals.var_qmfact1__blk1054_dn7 = assign40970_e46793_d_n7;
        locals.var_qmfact1__blk1054_dn8 = assign40970_e46793_d_n8;
        locals.var_qmfact1__blk1054_dn9 = assign40970_e46793_d_n9;

        let (assign40980_e46800, assign40980_e46800_d_n4, assign40980_e46800_d_n6, assign40980_e46800_d_n7, assign40980_e46800_d_n8, assign40980_e46800_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9,)
    }
};
        locals.var_qmfact2__blk1055 = assign40980_e46800;
        locals.var_qmfact2__blk1055_dn4 = assign40980_e46800_d_n4;
        locals.var_qmfact2__blk1055_dn6 = assign40980_e46800_d_n6;
        locals.var_qmfact2__blk1055_dn7 = assign40980_e46800_d_n7;
        locals.var_qmfact2__blk1055_dn8 = assign40980_e46800_d_n8;
        locals.var_qmfact2__blk1055_dn9 = assign40980_e46800_d_n9;

        let assign40990_e46803: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign40990_e46803;

        let assign41000_e46806: f64 = if locals.var_qid__blk1003 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign41000_e46806;

        let assign41010_e46808: f64 = (locals.var_a2d__blk1012).abs();
        let assign41010_e46810: f64 = if assign41010_e46808 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign41010_e46810;

        let (assign41020_e46832, assign41020_e46832_d_n4, assign41020_e46832_d_n6, assign41020_e46832_d_n7, assign41020_e46832_d_n8, assign41020_e46832_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41020_e46820: f64 = (2.0 + locals.var_q1d__blk1001);
        let assign41020_e46823: f64 = (0.5 * locals.var_a1d__blk1011);
        let assign41020_e46824: f64 = (assign41020_e46820 + assign41020_e46823);
        let assign41020_e46827: f64 = (2.0 + locals.var_q2d__blk1002);
        let assign41020_e46829: f64 = (assign41020_e46827 * locals.var_a1d__blk1011);
        let assign41020_e46830: f64 = (assign41020_e46824 / assign41020_e46829);
        (assign41020_e46830, ((((locals.var_q1d__blk1001_dn4 + (0.5 * locals.var_a1d__blk1011_dn4)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn4 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn4)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn6 + (0.5 * locals.var_a1d__blk1011_dn6)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn6 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn6)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn7 + (0.5 * locals.var_a1d__blk1011_dn7)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn7 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn7)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn8 + (0.5 * locals.var_a1d__blk1011_dn8)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn8 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn8)))) / (assign41020_e46829 * assign41020_e46829)), ((((locals.var_q1d__blk1001_dn9 + (0.5 * locals.var_a1d__blk1011_dn9)) * assign41020_e46829) - (assign41020_e46824 * ((locals.var_q2d__blk1002_dn9 * locals.var_a1d__blk1011) + (assign41020_e46827 * locals.var_a1d__blk1011_dn9)))) / (assign41020_e46829 * assign41020_e46829)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign41020_e46832;
        locals.var_temp_dn4 = assign41020_e46832_d_n4;
        locals.var_temp_dn6 = assign41020_e46832_d_n6;
        locals.var_temp_dn7 = assign41020_e46832_d_n7;
        locals.var_temp_dn8 = assign41020_e46832_d_n8;
        locals.var_temp_dn9 = assign41020_e46832_d_n9;

        let (assign41030_e46844, assign41030_e46844_d_n4, assign41030_e46844_d_n6, assign41030_e46844_d_n7, assign41030_e46844_d_n8, assign41030_e46844_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41030_e46842: f64 = (locals.var_temp * locals.var_a2d__blk1012);
        (assign41030_e46842, ((locals.var_temp_dn4 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn4)), ((locals.var_temp_dn6 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn6)), ((locals.var_temp_dn7 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn7)), ((locals.var_temp_dn8 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn8)), ((locals.var_temp_dn9 * locals.var_a2d__blk1012) + (locals.var_temp * locals.var_a2d__blk1012_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41030_e46844;
        locals.var_temp1_dn4 = assign41030_e46844_d_n4;
        locals.var_temp1_dn6 = assign41030_e46844_d_n6;
        locals.var_temp1_dn7 = assign41030_e46844_d_n7;
        locals.var_temp1_dn8 = assign41030_e46844_d_n8;
        locals.var_temp1_dn9 = assign41030_e46844_d_n9;

        let (assign41040_e46856, assign41040_e46856_d_n4, assign41040_e46856_d_n6, assign41040_e46856_d_n7, assign41040_e46856_d_n8, assign41040_e46856_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41040_e46854: f64 = (locals.var_temp1 * locals.var_temp1);
        (assign41040_e46854, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41040_e46856;
        locals.var_temp2_dn4 = assign41040_e46856_d_n4;
        locals.var_temp2_dn6 = assign41040_e46856_d_n6;
        locals.var_temp2_dn7 = assign41040_e46856_d_n7;
        locals.var_temp2_dn8 = assign41040_e46856_d_n8;
        locals.var_temp2_dn9 = assign41040_e46856_d_n9;

        let (assign41050_e46870, assign41050_e46870_d_n4, assign41050_e46870_d_n6, assign41050_e46870_d_n7, assign41050_e46870_d_n8, assign41050_e46870_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41050_e46866: f64 = (1.0 - locals.var_temp1);
        let assign41050_e46868: f64 = (assign41050_e46866 + locals.var_temp2);
        (assign41050_e46868, ((-locals.var_temp1_dn4) + locals.var_temp2_dn4), ((-locals.var_temp1_dn6) + locals.var_temp2_dn6), ((-locals.var_temp1_dn7) + locals.var_temp2_dn7), ((-locals.var_temp1_dn8) + locals.var_temp2_dn8), ((-locals.var_temp1_dn9) + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign41050_e46870;
        locals.var_temp3_dn4 = assign41050_e46870_d_n4;
        locals.var_temp3_dn6 = assign41050_e46870_d_n6;
        locals.var_temp3_dn7 = assign41050_e46870_d_n7;
        locals.var_temp3_dn8 = assign41050_e46870_d_n8;
        locals.var_temp3_dn9 = assign41050_e46870_d_n9;

        let (assign41060_e46884, assign41060_e46884_d_n4, assign41060_e46884_d_n6, assign41060_e46884_d_n7, assign41060_e46884_d_n8, assign41060_e46884_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41060_e46881: f64 = (locals.var_temp1 * locals.var_temp2);
        let assign41060_e46882: f64 = (locals.var_temp3 - assign41060_e46881);
        (assign41060_e46882, (locals.var_temp3_dn4 - ((locals.var_temp1_dn4 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn4))), (locals.var_temp3_dn6 - ((locals.var_temp1_dn6 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn6))), (locals.var_temp3_dn7 - ((locals.var_temp1_dn7 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn7))), (locals.var_temp3_dn8 - ((locals.var_temp1_dn8 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn8))), (locals.var_temp3_dn9 - ((locals.var_temp1_dn9 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn9))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign41060_e46884;
        locals.var_temp4_dn4 = assign41060_e46884_d_n4;
        locals.var_temp4_dn6 = assign41060_e46884_d_n6;
        locals.var_temp4_dn7 = assign41060_e46884_d_n7;
        locals.var_temp4_dn8 = assign41060_e46884_d_n8;
        locals.var_temp4_dn9 = assign41060_e46884_d_n9;

        let (assign41070_e46910, assign41070_e46910_d_n4, assign41070_e46910_d_n6, assign41070_e46910_d_n7, assign41070_e46910_d_n8, assign41070_e46910_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41070_e46895: f64 = (2.0 * locals.var_qsqd__blk1006);
        let assign41070_e46899: f64 = (1.0 / locals.var_a1d__blk1011);
        let assign41070_e46900: f64 = (locals.var_temp - assign41070_e46899);
        let assign41070_e46901: f64 = (assign41070_e46895 * assign41070_e46900);
        let assign41070_e46903: f64 = (assign41070_e46901 * locals.var_temp4);
        let assign41070_e46904: f64 = (locals.var_k2q2d__blk1005 - assign41070_e46903);
        let assign41070_e46907: f64 = (2.0 + locals.var_q2d__blk1002);
        let assign41070_e46908: f64 = (assign41070_e46904 / assign41070_e46907);
        (assign41070_e46908, ((((locals.var_k2q2d__blk1005_dn4 - (((((2.0 * locals.var_qsqd__blk1006_dn4) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn4 - (-(locals.var_a1d__blk1011_dn4 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn4))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn4)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn6 - (((((2.0 * locals.var_qsqd__blk1006_dn6) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn6 - (-(locals.var_a1d__blk1011_dn6 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn6))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn6)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn7 - (((((2.0 * locals.var_qsqd__blk1006_dn7) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn7 - (-(locals.var_a1d__blk1011_dn7 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn7))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn7)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn8 - (((((2.0 * locals.var_qsqd__blk1006_dn8) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn8 - (-(locals.var_a1d__blk1011_dn8 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn8))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn8)) / (assign41070_e46907 * assign41070_e46907)), ((((locals.var_k2q2d__blk1005_dn9 - (((((2.0 * locals.var_qsqd__blk1006_dn9) * assign41070_e46900) + (assign41070_e46895 * (locals.var_temp_dn9 - (-(locals.var_a1d__blk1011_dn9 / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)))))) * locals.var_temp4) + (assign41070_e46901 * locals.var_temp4_dn9))) * assign41070_e46907) - (assign41070_e46904 * locals.var_q2d__blk1002_dn9)) / (assign41070_e46907 * assign41070_e46907)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41070_e46910;
        locals.var_temp1_dn4 = assign41070_e46910_d_n4;
        locals.var_temp1_dn6 = assign41070_e46910_d_n6;
        locals.var_temp1_dn7 = assign41070_e46910_d_n7;
        locals.var_temp1_dn8 = assign41070_e46910_d_n8;
        locals.var_temp1_dn9 = assign41070_e46910_d_n9;

        let (assign41080_e46930, assign41080_e46930_d_n4, assign41080_e46930_d_n6, assign41080_e46930_d_n7, assign41080_e46930_d_n8, assign41080_e46930_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41080_e46920: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003);
        let assign41080_e46922: f64 = (assign41080_e46920 - locals.var_aexp1d__blk1007);
        let assign41080_e46924: f64 = (assign41080_e46922 / locals.var_a1d__blk1011);
        let assign41080_e46926: f64 = (assign41080_e46924 - locals.var_temp1);
        let assign41080_e46928: f64 = (assign41080_e46926 / locals.var_qid__blk1003);
        (assign41080_e46928, ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn4)) - locals.var_aexp1d__blk1007_dn4) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn4) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn6)) - locals.var_aexp1d__blk1007_dn6) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn6) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn7)) - locals.var_aexp1d__blk1007_dn7) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn7) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn8)) - locals.var_aexp1d__blk1007_dn8) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn8) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)), ((((((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_qid__blk1003) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_qid__blk1003_dn9)) - locals.var_aexp1d__blk1007_dn9) * locals.var_a1d__blk1011) - (assign41080_e46922 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) - locals.var_temp1_dn9) * locals.var_qid__blk1003) - (assign41080_e46926 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)),)
    } else {
        (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9,)
    }
};
        locals.var_dqid_dxn_qi__blk1056 = assign41080_e46930;
        locals.var_dqid_dxn_qi__blk1056_dn4 = assign41080_e46930_d_n4;
        locals.var_dqid_dxn_qi__blk1056_dn6 = assign41080_e46930_d_n6;
        locals.var_dqid_dxn_qi__blk1056_dn7 = assign41080_e46930_d_n7;
        locals.var_dqid_dxn_qi__blk1056_dn8 = assign41080_e46930_d_n8;
        locals.var_dqid_dxn_qi__blk1056_dn9 = assign41080_e46930_d_n9;

        let (assign41090_e46946, assign41090_e46946_d_n4, assign41090_e46946_d_n6, assign41090_e46946_d_n7, assign41090_e46946_d_n8, assign41090_e46946_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 != 0.0)) {
        let assign41090_e46940: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
        let assign41090_e46943: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
        let assign41090_e46944: f64 = (assign41090_e46940 / assign41090_e46943);
        (assign41090_e46944, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41090_e46943 * assign41090_e46943)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41090_e46943) - (assign41090_e46940 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41090_e46943 * assign41090_e46943)),)
    } else {
        (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9,)
    }
};
        locals.var_dd__blk1057 = assign41090_e46946;
        locals.var_dd__blk1057_dn4 = assign41090_e46946_d_n4;
        locals.var_dd__blk1057_dn6 = assign41090_e46946_d_n6;
        locals.var_dd__blk1057_dn7 = assign41090_e46946_d_n7;
        locals.var_dd__blk1057_dn8 = assign41090_e46946_d_n8;
        locals.var_dd__blk1057_dn9 = assign41090_e46946_d_n9;

        let (assign41100_e46973, assign41100_e46973_d_n4, assign41100_e46973_d_n6, assign41100_e46973_d_n7, assign41100_e46973_d_n8, assign41100_e46973_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
        let assign41100_e46957: f64 = (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013);
        let assign41100_e46960: f64 = (locals.var_a1d__blk1011 * locals.var_a2d__blk1012);
        let assign41100_e46961: f64 = (assign41100_e46957 / assign41100_e46960);
        let assign41100_e46964: f64 = (locals.var_aexp1d__blk1007 / locals.var_a1d__blk1011);
        let assign41100_e46967: f64 = (locals.var_aexp2d__blk1008 / locals.var_a2d__blk1012);
        let assign41100_e46968: f64 = (assign41100_e46964 + assign41100_e46967);
        let assign41100_e46970: f64 = (assign41100_e46968 / locals.var_qid__blk1003);
        let assign41100_e46971: f64 = (assign41100_e46961 - assign41100_e46970);
        (assign41100_e46971, ((((((locals.var_dqsqd_dxn_qi__blk1014_dn4 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn4)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn4 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn4)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn4 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn4)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn4 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn4)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn6 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn6)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn6 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn6)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn6 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn6)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn6 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn6)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn7 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn7)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn7 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn7)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn7 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn7)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn7 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn7)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn8 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn8)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn8 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn8)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn8 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn8)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn8 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn8)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))), ((((((locals.var_dqsqd_dxn_qi__blk1014_dn9 * locals.var_sumd__blk1013) + (locals.var_dqsqd_dxn_qi__blk1014 * locals.var_sumd__blk1013_dn9)) * assign41100_e46960) - (assign41100_e46957 * ((locals.var_a1d__blk1011_dn9 * locals.var_a2d__blk1012) + (locals.var_a1d__blk1011 * locals.var_a2d__blk1012_dn9)))) / (assign41100_e46960 * assign41100_e46960)) - (((((((locals.var_aexp1d__blk1007_dn9 * locals.var_a1d__blk1011) - (locals.var_aexp1d__blk1007 * locals.var_a1d__blk1011_dn9)) / (locals.var_a1d__blk1011 * locals.var_a1d__blk1011)) + (((locals.var_aexp2d__blk1008_dn9 * locals.var_a2d__blk1012) - (locals.var_aexp2d__blk1008 * locals.var_a2d__blk1012_dn9)) / (locals.var_a2d__blk1012 * locals.var_a2d__blk1012))) * locals.var_qid__blk1003) - (assign41100_e46968 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003))),)
    } else {
        (locals.var_dqid_dxn_qi__blk1056, locals.var_dqid_dxn_qi__blk1056_dn4, locals.var_dqid_dxn_qi__blk1056_dn6, locals.var_dqid_dxn_qi__blk1056_dn7, locals.var_dqid_dxn_qi__blk1056_dn8, locals.var_dqid_dxn_qi__blk1056_dn9,)
    }
};
        locals.var_dqid_dxn_qi__blk1056 = assign41100_e46973;
        locals.var_dqid_dxn_qi__blk1056_dn4 = assign41100_e46973_d_n4;
        locals.var_dqid_dxn_qi__blk1056_dn6 = assign41100_e46973_d_n6;
        locals.var_dqid_dxn_qi__blk1056_dn7 = assign41100_e46973_d_n7;
        locals.var_dqid_dxn_qi__blk1056_dn8 = assign41100_e46973_d_n8;
        locals.var_dqid_dxn_qi__blk1056_dn9 = assign41100_e46973_d_n9;

        let (assign41110_e46990, assign41110_e46990_d_n4, assign41110_e46990_d_n6, assign41110_e46990_d_n7, assign41110_e46990_d_n8, assign41110_e46990_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1228 != 0.0)) && (locals.var_guard1229 != 0.0)) && (locals.var_guard1230 == 0.0)) {
        let assign41110_e46984: f64 = (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003);
        let assign41110_e46987: f64 = (locals.var_dqid_dxn_qi__blk1056 + 1.0);
        let assign41110_e46988: f64 = (assign41110_e46984 / assign41110_e46987);
        (assign41110_e46988, (((((locals.var_dqid_dxn_qi__blk1056_dn4 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn4)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn4)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn6 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn6)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn6)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn7 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn7)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn7)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn8 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn8)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn8)) / (assign41110_e46987 * assign41110_e46987)), (((((locals.var_dqid_dxn_qi__blk1056_dn9 * locals.var_qid__blk1003) + (locals.var_dqid_dxn_qi__blk1056 * locals.var_qid__blk1003_dn9)) * assign41110_e46987) - (assign41110_e46984 * locals.var_dqid_dxn_qi__blk1056_dn9)) / (assign41110_e46987 * assign41110_e46987)),)
    } else {
        (locals.var_dd__blk1057, locals.var_dd__blk1057_dn4, locals.var_dd__blk1057_dn6, locals.var_dd__blk1057_dn7, locals.var_dd__blk1057_dn8, locals.var_dd__blk1057_dn9,)
    }
};
        locals.var_dd__blk1057 = assign41110_e46990;
        locals.var_dd__blk1057_dn4 = assign41110_e46990_d_n4;
        locals.var_dd__blk1057_dn6 = assign41110_e46990_d_n6;
        locals.var_dd__blk1057_dn7 = assign41110_e46990_d_n7;
        locals.var_dd__blk1057_dn8 = assign41110_e46990_d_n8;
        locals.var_dd__blk1057_dn9 = assign41110_e46990_d_n9;

    }
}
