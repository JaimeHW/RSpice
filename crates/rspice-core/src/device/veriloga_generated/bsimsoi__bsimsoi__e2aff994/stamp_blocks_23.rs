#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_157(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign45480_e77495, assign45480_e77495_d_n3, assign45480_e77495_d_n4, assign45480_e77495_d_n5, assign45480_e77495_d_n6, assign45480_e77495_d_n7, assign45480_e77495_d_n8, assign45480_e77495_d_n9, assign45480_e77495_d_n10, assign45480_e77495_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 != 0.0)) {
        let assign45480_e77473: f64 = (1.0 + locals.var_t1);
        let assign45480_e77475: f64 = (assign45480_e77473 - locals.var_t8);
        let assign45480_e77478: f64 = (2.0 * locals.var_t0);
        let assign45480_e77481: f64 = (locals.var_t3 * 2.0);
        let assign45480_e77483: f64 = (assign45480_e77481 * locals.var_t0);
        let assign45480_e77486: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign45480_e77487: f64 = (assign45480_e77483 + assign45480_e77486);
        let assign45480_e77488: f64 = (assign45480_e77478 * assign45480_e77487);
        let assign45480_e77490: f64 = (assign45480_e77488).max(1e-38);
        let assign45480_e77491: f64 = (assign45480_e77490).ln();
        let assign45480_e77492: f64 = (assign45480_e77475 - assign45480_e77491);
        let assign45480_e77493: f64 = (locals.var_t3 * assign45480_e77492);
        (assign45480_e77493, ((locals.var_t3_dn3 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn4 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn5 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn6 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn7 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn8 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn9 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn10 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign45480_e77490)))), ((locals.var_t3_dn11 * assign45480_e77492) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign45480_e77488 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign45480_e77487) + (assign45480_e77478 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign45480_e77481 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign45480_e77490)))),)
    } else {
        (locals.var_qs_1, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11,)
    }
};
        locals.var_qs_1 = assign45480_e77495;
        locals.var_qs_1_dn3 = assign45480_e77495_d_n3;
        locals.var_qs_1_dn4 = assign45480_e77495_d_n4;
        locals.var_qs_1_dn5 = assign45480_e77495_d_n5;
        locals.var_qs_1_dn6 = assign45480_e77495_d_n6;
        locals.var_qs_1_dn7 = assign45480_e77495_d_n7;
        locals.var_qs_1_dn8 = assign45480_e77495_d_n8;
        locals.var_qs_1_dn9 = assign45480_e77495_d_n9;
        locals.var_qs_1_dn10 = assign45480_e77495_d_n10;
        locals.var_qs_1_dn11 = assign45480_e77495_d_n11;
        locals.var_qs_1_rv = 0.0;

        let (assign45490_e77504, assign45490_e77504_d_n3, assign45490_e77504_d_n4, assign45490_e77504_d_n5, assign45490_e77504_d_n6, assign45490_e77504_d_n7, assign45490_e77504_d_n8, assign45490_e77504_d_n9, assign45490_e77504_d_n10, assign45490_e77504_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45490_e77502: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign45490_e77502, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign45490_e77504;
        locals.var_t3_dn3 = assign45490_e77504_d_n3;
        locals.var_t3_dn4 = assign45490_e77504_d_n4;
        locals.var_t3_dn5 = assign45490_e77504_d_n5;
        locals.var_t3_dn6 = assign45490_e77504_d_n6;
        locals.var_t3_dn7 = assign45490_e77504_d_n7;
        locals.var_t3_dn8 = assign45490_e77504_d_n8;
        locals.var_t3_dn9 = assign45490_e77504_d_n9;
        locals.var_t3_dn10 = assign45490_e77504_d_n10;
        locals.var_t3_dn11 = assign45490_e77504_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign45500_e77514, assign45500_e77514_d_n3, assign45500_e77514_d_n4, assign45500_e77514_d_n5, assign45500_e77514_d_n6, assign45500_e77514_d_n7, assign45500_e77514_d_n8, assign45500_e77514_d_n9, assign45500_e77514_d_n10, assign45500_e77514_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45500_e77512: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign45500_e77512, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign45500_e77514;
        locals.var_sqrtpsisainv_dn3 = assign45500_e77514_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign45500_e77514_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign45500_e77514_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign45500_e77514_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign45500_e77514_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign45500_e77514_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign45500_e77514_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign45500_e77514_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign45500_e77514_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign45510_e77545, assign45510_e77545_d_n3, assign45510_e77545_d_n4, assign45510_e77545_d_n5, assign45510_e77545_d_n6, assign45510_e77545_d_n7, assign45510_e77545_d_n8, assign45510_e77545_d_n9, assign45510_e77545_d_n10, assign45510_e77545_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45510_e77522: f64 = (2.0 * locals.var_t3);
        let assign45510_e77525: f64 = (locals.var_t3 * 2.0);
        let assign45510_e77527: f64 = (assign45510_e77525 * locals.var_t0);
        let assign45510_e77530: f64 = (locals.var_t3 * 2.0);
        let assign45510_e77532: f64 = (assign45510_e77530 * locals.var_t0);
        let assign45510_e77535: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign45510_e77536: f64 = (assign45510_e77532 + assign45510_e77535);
        let assign45510_e77537: f64 = (assign45510_e77527 * assign45510_e77536);
        let assign45510_e77539: f64 = (assign45510_e77537).max(1e-38);
        let assign45510_e77540: f64 = (assign45510_e77539).ln();
        let assign45510_e77541: f64 = (assign45510_e77522 + assign45510_e77540);
        let assign45510_e77543: f64 = (assign45510_e77541 - locals.var_t1);
        (assign45510_e77543, (((2.0 * locals.var_t3_dn3) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn3)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn4)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn5)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn6)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn7)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn8)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn9)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn10)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign45510_e77537 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign45510_e77525 * locals.var_t0_dn11)) * assign45510_e77536) + (assign45510_e77527 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign45510_e77530 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign45510_e77539)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign45510_e77545;
        locals.var_t4_dn3 = assign45510_e77545_d_n3;
        locals.var_t4_dn4 = assign45510_e77545_d_n4;
        locals.var_t4_dn5 = assign45510_e77545_d_n5;
        locals.var_t4_dn6 = assign45510_e77545_d_n6;
        locals.var_t4_dn7 = assign45510_e77545_d_n7;
        locals.var_t4_dn8 = assign45510_e77545_d_n8;
        locals.var_t4_dn9 = assign45510_e77545_d_n9;
        locals.var_t4_dn10 = assign45510_e77545_d_n10;
        locals.var_t4_dn11 = assign45510_e77545_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign45520_e77567, assign45520_e77567_d_n3, assign45520_e77567_d_n4, assign45520_e77567_d_n5, assign45520_e77567_d_n6, assign45520_e77567_d_n7, assign45520_e77567_d_n8, assign45520_e77567_d_n9, assign45520_e77567_d_n10, assign45520_e77567_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45520_e77554: f64 = (1.0 / locals.var_t3);
        let assign45520_e77555: f64 = (2.0 + assign45520_e77554);
        let assign45520_e77558: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign45520_e77561: f64 = (locals.var_t0 * locals.var_t3);
        let assign45520_e77563: f64 = (assign45520_e77561 + locals.var_sqrtpsisa);
        let assign45520_e77564: f64 = (assign45520_e77558 / assign45520_e77563);
        let assign45520_e77565: f64 = (assign45520_e77555 + assign45520_e77564);
        (assign45520_e77565, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign45520_e77563 * assign45520_e77563))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign45520_e77563) - (assign45520_e77558 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign45520_e77563 * assign45520_e77563))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign45520_e77567;
        locals.var_t5_dn3 = assign45520_e77567_d_n3;
        locals.var_t5_dn4 = assign45520_e77567_d_n4;
        locals.var_t5_dn5 = assign45520_e77567_d_n5;
        locals.var_t5_dn6 = assign45520_e77567_d_n6;
        locals.var_t5_dn7 = assign45520_e77567_d_n7;
        locals.var_t5_dn8 = assign45520_e77567_d_n8;
        locals.var_t5_dn9 = assign45520_e77567_d_n9;
        locals.var_t5_dn10 = assign45520_e77567_d_n10;
        locals.var_t5_dn11 = assign45520_e77567_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign45530_e77579, assign45530_e77579_d_n3, assign45530_e77579_d_n4, assign45530_e77579_d_n5, assign45530_e77579_d_n6, assign45530_e77579_d_n7, assign45530_e77579_d_n8, assign45530_e77579_d_n9, assign45530_e77579_d_n10, assign45530_e77579_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45530_e77576: f64 = (locals.var_t4 / locals.var_t5);
        let assign45530_e77577: f64 = (locals.var_t3 - assign45530_e77576);
        (assign45530_e77577, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign45530_e77579;
        locals.var_t3_dn3 = assign45530_e77579_d_n3;
        locals.var_t3_dn4 = assign45530_e77579_d_n4;
        locals.var_t3_dn5 = assign45530_e77579_d_n5;
        locals.var_t3_dn6 = assign45530_e77579_d_n6;
        locals.var_t3_dn7 = assign45530_e77579_d_n7;
        locals.var_t3_dn8 = assign45530_e77579_d_n8;
        locals.var_t3_dn9 = assign45530_e77579_d_n9;
        locals.var_t3_dn10 = assign45530_e77579_d_n10;
        locals.var_t3_dn11 = assign45530_e77579_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign45540_e77610, assign45540_e77610_d_n3, assign45540_e77610_d_n4, assign45540_e77610_d_n5, assign45540_e77610_d_n6, assign45540_e77610_d_n7, assign45540_e77610_d_n8, assign45540_e77610_d_n9, assign45540_e77610_d_n10, assign45540_e77610_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45540_e77587: f64 = (2.0 * locals.var_t3);
        let assign45540_e77590: f64 = (locals.var_t3 * 2.0);
        let assign45540_e77592: f64 = (assign45540_e77590 * locals.var_t0);
        let assign45540_e77595: f64 = (locals.var_t3 * 2.0);
        let assign45540_e77597: f64 = (assign45540_e77595 * locals.var_t0);
        let assign45540_e77600: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign45540_e77601: f64 = (assign45540_e77597 + assign45540_e77600);
        let assign45540_e77602: f64 = (assign45540_e77592 * assign45540_e77601);
        let assign45540_e77604: f64 = (assign45540_e77602).max(1e-38);
        let assign45540_e77605: f64 = (assign45540_e77604).ln();
        let assign45540_e77606: f64 = (assign45540_e77587 + assign45540_e77605);
        let assign45540_e77608: f64 = (assign45540_e77606 - locals.var_t1);
        (assign45540_e77608, (((2.0 * locals.var_t3_dn3) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn3)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn4)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn5)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn6)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn7)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn8)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn9)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn10)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign45540_e77602 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign45540_e77590 * locals.var_t0_dn11)) * assign45540_e77601) + (assign45540_e77592 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign45540_e77595 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign45540_e77604)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign45540_e77610;
        locals.var_t4_dn3 = assign45540_e77610_d_n3;
        locals.var_t4_dn4 = assign45540_e77610_d_n4;
        locals.var_t4_dn5 = assign45540_e77610_d_n5;
        locals.var_t4_dn6 = assign45540_e77610_d_n6;
        locals.var_t4_dn7 = assign45540_e77610_d_n7;
        locals.var_t4_dn8 = assign45540_e77610_d_n8;
        locals.var_t4_dn9 = assign45540_e77610_d_n9;
        locals.var_t4_dn10 = assign45540_e77610_d_n10;
        locals.var_t4_dn11 = assign45540_e77610_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign45550_e77632, assign45550_e77632_d_n3, assign45550_e77632_d_n4, assign45550_e77632_d_n5, assign45550_e77632_d_n6, assign45550_e77632_d_n7, assign45550_e77632_d_n8, assign45550_e77632_d_n9, assign45550_e77632_d_n10, assign45550_e77632_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45550_e77619: f64 = (1.0 / locals.var_t3);
        let assign45550_e77620: f64 = (2.0 + assign45550_e77619);
        let assign45550_e77623: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign45550_e77626: f64 = (locals.var_t0 * locals.var_t3);
        let assign45550_e77628: f64 = (assign45550_e77626 + locals.var_sqrtpsisa);
        let assign45550_e77629: f64 = (assign45550_e77623 / assign45550_e77628);
        let assign45550_e77630: f64 = (assign45550_e77620 + assign45550_e77629);
        (assign45550_e77630, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign45550_e77628 * assign45550_e77628))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign45550_e77628) - (assign45550_e77623 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign45550_e77628 * assign45550_e77628))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign45550_e77632;
        locals.var_t5_dn3 = assign45550_e77632_d_n3;
        locals.var_t5_dn4 = assign45550_e77632_d_n4;
        locals.var_t5_dn5 = assign45550_e77632_d_n5;
        locals.var_t5_dn6 = assign45550_e77632_d_n6;
        locals.var_t5_dn7 = assign45550_e77632_d_n7;
        locals.var_t5_dn8 = assign45550_e77632_d_n8;
        locals.var_t5_dn9 = assign45550_e77632_d_n9;
        locals.var_t5_dn10 = assign45550_e77632_d_n10;
        locals.var_t5_dn11 = assign45550_e77632_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign45560_e77658, assign45560_e77658_d_n3, assign45560_e77658_d_n4, assign45560_e77658_d_n5, assign45560_e77658_d_n6, assign45560_e77658_d_n7, assign45560_e77658_d_n8, assign45560_e77658_d_n9, assign45560_e77658_d_n10, assign45560_e77658_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45560_e77640: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign45560_e77643: f64 = (locals.var_t0 * locals.var_t3);
        let assign45560_e77645: f64 = (assign45560_e77643 + locals.var_sqrtpsisa);
        let assign45560_e77646: f64 = (assign45560_e77640 / assign45560_e77645);
        let assign45560_e77649: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign45560_e77652: f64 = (locals.var_t0 * locals.var_t3);
        let assign45560_e77654: f64 = (assign45560_e77652 + locals.var_sqrtpsisa);
        let assign45560_e77655: f64 = (assign45560_e77649 / assign45560_e77654);
        let assign45560_e77656: f64 = (assign45560_e77646 * assign45560_e77655);
        (assign45560_e77656, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign45560_e77654 * assign45560_e77654)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign45560_e77645) - (assign45560_e77640 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign45560_e77645 * assign45560_e77645)) * assign45560_e77655) + (assign45560_e77646 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign45560_e77654) - (assign45560_e77649 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign45560_e77654 * assign45560_e77654)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign45560_e77658;
        locals.var_t6_dn3 = assign45560_e77658_d_n3;
        locals.var_t6_dn4 = assign45560_e77658_d_n4;
        locals.var_t6_dn5 = assign45560_e77658_d_n5;
        locals.var_t6_dn6 = assign45560_e77658_d_n6;
        locals.var_t6_dn7 = assign45560_e77658_d_n7;
        locals.var_t6_dn8 = assign45560_e77658_d_n8;
        locals.var_t6_dn9 = assign45560_e77658_d_n9;
        locals.var_t6_dn10 = assign45560_e77658_d_n10;
        locals.var_t6_dn11 = assign45560_e77658_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign45570_e77689, assign45570_e77689_d_n3, assign45570_e77689_d_n4, assign45570_e77689_d_n5, assign45570_e77689_d_n6, assign45570_e77689_d_n7, assign45570_e77689_d_n8, assign45570_e77689_d_n9, assign45570_e77689_d_n10, assign45570_e77689_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign45570_e77666: f64 = (1.0 * __rspice_inv_cse_0);
        let assign45570_e77669: f64 = (1.0 * __rspice_inv_cse_0);
        let assign45570_e77670: f64 = (assign45570_e77666 * assign45570_e77669);
        let assign45570_e77671: f64 = (-assign45570_e77670);
        let assign45570_e77675: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign45570_e77677: f64 = (assign45570_e77675 * locals.var_sqrtpsisa);
        let assign45570_e77680: f64 = (locals.var_t0 * locals.var_t3);
        let assign45570_e77682: f64 = (assign45570_e77680 + locals.var_sqrtpsisa);
        let assign45570_e77683: f64 = (assign45570_e77677 * assign45570_e77682);
        let assign45570_e77684: f64 = (1.0 / assign45570_e77683);
        let assign45570_e77685: f64 = (assign45570_e77671 - assign45570_e77684);
        let assign45570_e77687: f64 = (assign45570_e77685 - locals.var_t6);
        (assign45570_e77687, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn3)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn4)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn5)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn6)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn7)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn8)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn9)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn10)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign45570_e77669) + (assign45570_e77666 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign45570_e77675 * locals.var_sqrtpsisa_dn11)) * assign45570_e77682) + (assign45570_e77677 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign45570_e77683 * assign45570_e77683)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign45570_e77689;
        locals.var_t7_dn3 = assign45570_e77689_d_n3;
        locals.var_t7_dn4 = assign45570_e77689_d_n4;
        locals.var_t7_dn5 = assign45570_e77689_d_n5;
        locals.var_t7_dn6 = assign45570_e77689_d_n6;
        locals.var_t7_dn7 = assign45570_e77689_d_n7;
        locals.var_t7_dn8 = assign45570_e77689_d_n8;
        locals.var_t7_dn9 = assign45570_e77689_d_n9;
        locals.var_t7_dn10 = assign45570_e77689_d_n10;
        locals.var_t7_dn11 = assign45570_e77689_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign45580_e77713, assign45580_e77713_d_n3, assign45580_e77713_d_n4, assign45580_e77713_d_n5, assign45580_e77713_d_n6, assign45580_e77713_d_n7, assign45580_e77713_d_n8, assign45580_e77713_d_n9, assign45580_e77713_d_n10, assign45580_e77713_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard709 == 0.0)) {
        let assign45580_e77698: f64 = (locals.var_t4 / locals.var_t5);
        let assign45580_e77702: f64 = (locals.var_t4 * locals.var_t7);
        let assign45580_e77705: f64 = (2.0 * locals.var_t5);
        let assign45580_e77707: f64 = (assign45580_e77705 * locals.var_t5);
        let assign45580_e77708: f64 = (assign45580_e77702 / assign45580_e77707);
        let assign45580_e77709: f64 = (1.0 + assign45580_e77708);
        let assign45580_e77710: f64 = (assign45580_e77698 * assign45580_e77709);
        let assign45580_e77711: f64 = (locals.var_t3 - assign45580_e77710);
        (assign45580_e77711, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn3)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn4)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn5)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn6)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn7)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn8)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn9)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn10)))) / (assign45580_e77707 * assign45580_e77707))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign45580_e77709) + (assign45580_e77698 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign45580_e77707) - (assign45580_e77702 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign45580_e77705 * locals.var_t5_dn11)))) / (assign45580_e77707 * assign45580_e77707))))),)
    } else {
        (locals.var_qs_1, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11,)
    }
};
        locals.var_qs_1 = assign45580_e77713;
        locals.var_qs_1_dn3 = assign45580_e77713_d_n3;
        locals.var_qs_1_dn4 = assign45580_e77713_d_n4;
        locals.var_qs_1_dn5 = assign45580_e77713_d_n5;
        locals.var_qs_1_dn6 = assign45580_e77713_d_n6;
        locals.var_qs_1_dn7 = assign45580_e77713_d_n7;
        locals.var_qs_1_dn8 = assign45580_e77713_d_n8;
        locals.var_qs_1_dn9 = assign45580_e77713_d_n9;
        locals.var_qs_1_dn10 = assign45580_e77713_d_n10;
        locals.var_qs_1_dn11 = assign45580_e77713_d_n11;
        locals.var_qs_1_rv = 0.0;

        let (assign45590_e77737, assign45590_e77737_d_n3, assign45590_e77737_d_n4, assign45590_e77737_d_n5, assign45590_e77737_d_n6, assign45590_e77737_d_n7, assign45590_e77737_d_n8, assign45590_e77737_d_n9, assign45590_e77737_d_n10, assign45590_e77737_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45590_e77719: f64 = (locals.var_psip + 1.0);
        let assign45590_e77722: f64 = (locals.var_psip - 1.0);
        let assign45590_e77725: f64 = (locals.var_psip - 1.0);
        let assign45590_e77726: f64 = (assign45590_e77722 * assign45590_e77725);
        let assign45590_e77729: f64 = (0.25 * 2.0);
        let assign45590_e77731: f64 = (assign45590_e77729 * 2.0);
        let assign45590_e77732: f64 = (assign45590_e77726 + assign45590_e77731);
        let assign45590_e77733: f64 = (assign45590_e77732).sqrt();
        let assign45590_e77734: f64 = (assign45590_e77719 + assign45590_e77733);
        let assign45590_e77735: f64 = (0.5 * assign45590_e77734);
        (assign45590_e77735, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn3)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn4)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn5)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn6)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn7)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn8)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn9)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn10)) / (2.0 * assign45590_e77733)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign45590_e77725) + (assign45590_e77722 * locals.var_psip_dn11)) / (2.0 * assign45590_e77733)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11,)
    }
};
        locals.var_psipclamp = assign45590_e77737;
        locals.var_psipclamp_dn3 = assign45590_e77737_d_n3;
        locals.var_psipclamp_dn4 = assign45590_e77737_d_n4;
        locals.var_psipclamp_dn5 = assign45590_e77737_d_n5;
        locals.var_psipclamp_dn6 = assign45590_e77737_d_n6;
        locals.var_psipclamp_dn7 = assign45590_e77737_d_n7;
        locals.var_psipclamp_dn8 = assign45590_e77737_d_n8;
        locals.var_psipclamp_dn9 = assign45590_e77737_d_n9;
        locals.var_psipclamp_dn10 = assign45590_e77737_d_n10;
        locals.var_psipclamp_dn11 = assign45590_e77737_d_n11;
        locals.var_psipclamp_rv = 0.0;

        let (assign45600_e77743, assign45600_e77743_d_n3, assign45600_e77743_d_n4, assign45600_e77743_d_n5, assign45600_e77743_d_n6, assign45600_e77743_d_n7, assign45600_e77743_d_n8, assign45600_e77743_d_n9, assign45600_e77743_d_n10, assign45600_e77743_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45600_e77741: f64 = (locals.var_psipclamp).sqrt();
        (assign45600_e77741, (locals.var_psipclamp_dn3 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn4 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn5 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn6 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn7 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn8 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn9 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn10 / (2.0 * assign45600_e77741)), (locals.var_psipclamp_dn11 / (2.0 * assign45600_e77741)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign45600_e77743;
        locals.var_sqrtpsip_dn3 = assign45600_e77743_d_n3;
        locals.var_sqrtpsip_dn4 = assign45600_e77743_d_n4;
        locals.var_sqrtpsip_dn5 = assign45600_e77743_d_n5;
        locals.var_sqrtpsip_dn6 = assign45600_e77743_d_n6;
        locals.var_sqrtpsip_dn7 = assign45600_e77743_d_n7;
        locals.var_sqrtpsip_dn8 = assign45600_e77743_d_n8;
        locals.var_sqrtpsip_dn9 = assign45600_e77743_d_n9;
        locals.var_sqrtpsip_dn10 = assign45600_e77743_d_n10;
        locals.var_sqrtpsip_dn11 = assign45600_e77743_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign45610_e77752, assign45610_e77752_d_n3, assign45610_e77752_d_n4, assign45610_e77752_d_n5, assign45610_e77752_d_n6, assign45610_e77752_d_n7, assign45610_e77752_d_n8, assign45610_e77752_d_n9, assign45610_e77752_d_n10, assign45610_e77752_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45610_e77749: f64 = (2.0 * locals.var_qs_1);
        let assign45610_e77750: f64 = (locals.var_psip - assign45610_e77749);
        (assign45610_e77750, (locals.var_psip_dn3 - (2.0 * locals.var_qs_1_dn3)), (locals.var_psip_dn4 - (2.0 * locals.var_qs_1_dn4)), (locals.var_psip_dn5 - (2.0 * locals.var_qs_1_dn5)), (locals.var_psip_dn6 - (2.0 * locals.var_qs_1_dn6)), (locals.var_psip_dn7 - (2.0 * locals.var_qs_1_dn7)), (locals.var_psip_dn8 - (2.0 * locals.var_qs_1_dn8)), (locals.var_psip_dn9 - (2.0 * locals.var_qs_1_dn9)), (locals.var_psip_dn10 - (2.0 * locals.var_qs_1_dn10)), (locals.var_psip_dn11 - (2.0 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign45610_e77752;
        locals.var_psiavg_dn3 = assign45610_e77752_d_n3;
        locals.var_psiavg_dn4 = assign45610_e77752_d_n4;
        locals.var_psiavg_dn5 = assign45610_e77752_d_n5;
        locals.var_psiavg_dn6 = assign45610_e77752_d_n6;
        locals.var_psiavg_dn7 = assign45610_e77752_d_n7;
        locals.var_psiavg_dn8 = assign45610_e77752_d_n8;
        locals.var_psiavg_dn9 = assign45610_e77752_d_n9;
        locals.var_psiavg_dn10 = assign45610_e77752_d_n10;
        locals.var_psiavg_dn11 = assign45610_e77752_d_n11;
        locals.var_psiavg_rv = 0.0;

        let (assign45620_e77776, assign45620_e77776_d_n3, assign45620_e77776_d_n4, assign45620_e77776_d_n5, assign45620_e77776_d_n6, assign45620_e77776_d_n7, assign45620_e77776_d_n8, assign45620_e77776_d_n9, assign45620_e77776_d_n10, assign45620_e77776_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45620_e77758: f64 = (locals.var_psiavg + 1.0);
        let assign45620_e77761: f64 = (locals.var_psiavg - 1.0);
        let assign45620_e77764: f64 = (locals.var_psiavg - 1.0);
        let assign45620_e77765: f64 = (assign45620_e77761 * assign45620_e77764);
        let assign45620_e77768: f64 = (0.25 * 2.0);
        let assign45620_e77770: f64 = (assign45620_e77768 * 2.0);
        let assign45620_e77771: f64 = (assign45620_e77765 + assign45620_e77770);
        let assign45620_e77772: f64 = (assign45620_e77771).sqrt();
        let assign45620_e77773: f64 = (assign45620_e77758 + assign45620_e77772);
        let assign45620_e77774: f64 = (0.5 * assign45620_e77773);
        (assign45620_e77774, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn3)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn4)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn5)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn6)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn7)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn8)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn9)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn10)) / (2.0 * assign45620_e77772)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign45620_e77764) + (assign45620_e77761 * locals.var_psiavg_dn11)) / (2.0 * assign45620_e77772)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign45620_e77776;
        locals.var_t0_dn3 = assign45620_e77776_d_n3;
        locals.var_t0_dn4 = assign45620_e77776_d_n4;
        locals.var_t0_dn5 = assign45620_e77776_d_n5;
        locals.var_t0_dn6 = assign45620_e77776_d_n6;
        locals.var_t0_dn7 = assign45620_e77776_d_n7;
        locals.var_t0_dn8 = assign45620_e77776_d_n8;
        locals.var_t0_dn9 = assign45620_e77776_d_n9;
        locals.var_t0_dn10 = assign45620_e77776_d_n10;
        locals.var_t0_dn11 = assign45620_e77776_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign45630_e77788, assign45630_e77788_d_n3, assign45630_e77788_d_n4, assign45630_e77788_d_n5, assign45630_e77788_d_n6, assign45630_e77788_d_n7, assign45630_e77788_d_n8, assign45630_e77788_d_n9, assign45630_e77788_d_n10, assign45630_e77788_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45630_e77783: f64 = (locals.var_t0).sqrt();
        let assign45630_e77784: f64 = (locals.var_sqrtpsip + assign45630_e77783);
        let assign45630_e77785: f64 = (locals.var_gam / assign45630_e77784);
        let assign45630_e77786: f64 = (1.0 + assign45630_e77785);
        (assign45630_e77786, (((locals.var_gam_dn3 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn3 + (locals.var_t0_dn3 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn4 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn4 + (locals.var_t0_dn4 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn5 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn5 + (locals.var_t0_dn5 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn6 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn6 + (locals.var_t0_dn6 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn7 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn7 + (locals.var_t0_dn7 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn8 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn8 + (locals.var_t0_dn8 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn9 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn9 + (locals.var_t0_dn9 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn10 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn10 + (locals.var_t0_dn10 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)), (((locals.var_gam_dn11 * assign45630_e77784) - (locals.var_gam * (locals.var_sqrtpsip_dn11 + (locals.var_t0_dn11 / (2.0 * assign45630_e77783))))) / (assign45630_e77784 * assign45630_e77784)),)
    } else {
        (locals.var_nq, locals.var_nq_dn3, locals.var_nq_dn4, locals.var_nq_dn5, locals.var_nq_dn6, locals.var_nq_dn7, locals.var_nq_dn8, locals.var_nq_dn9, locals.var_nq_dn10, locals.var_nq_dn11,)
    }
};
        locals.var_nq = assign45630_e77788;
        locals.var_nq_dn3 = assign45630_e77788_d_n3;
        locals.var_nq_dn4 = assign45630_e77788_d_n4;
        locals.var_nq_dn5 = assign45630_e77788_d_n5;
        locals.var_nq_dn6 = assign45630_e77788_d_n6;
        locals.var_nq_dn7 = assign45630_e77788_d_n7;
        locals.var_nq_dn8 = assign45630_e77788_d_n8;
        locals.var_nq_dn9 = assign45630_e77788_d_n9;
        locals.var_nq_dn10 = assign45630_e77788_d_n10;
        locals.var_nq_dn11 = assign45630_e77788_d_n11;
        locals.var_nq_rv = 0.0;

        let (assign45640_e77797,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45640_e77794: f64 = (locals.var_epsratio * p.p76);
        let assign45640_e77795: f64 = (1e-8 / assign45640_e77794);
        (assign45640_e77795,)
    } else {
        (locals.var_eefffactor,)
    }
};
        locals.var_eefffactor = assign45640_e77797;
        locals.var_eefffactor_rv = 0.0;

        let (assign45650_e77814, assign45650_e77814_d_n3, assign45650_e77814_d_n4, assign45650_e77814_d_n5, assign45650_e77814_d_n6, assign45650_e77814_d_n7, assign45650_e77814_d_n8, assign45650_e77814_d_n9, assign45650_e77814_d_n10, assign45650_e77814_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45650_e77803: f64 = (locals.var_vgfb - locals.var_psip);
        let assign45650_e77806: f64 = (2.0 * locals.var_qs_1);
        let assign45650_e77809: f64 = (locals.var_nq - 1.0);
        let assign45650_e77810: f64 = (assign45650_e77806 * assign45650_e77809);
        let assign45650_e77811: f64 = (assign45650_e77803 - assign45650_e77810);
        let assign45650_e77812: f64 = (locals.var_nvt * assign45650_e77811);
        (assign45650_e77812, ((locals.var_nvt_dn3 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn3 - locals.var_psip_dn3) - (((2.0 * locals.var_qs_1_dn3) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn3))))), ((locals.var_nvt_dn4 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn4 - locals.var_psip_dn4) - (((2.0 * locals.var_qs_1_dn4) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn4))))), ((locals.var_nvt_dn5 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn5 - locals.var_psip_dn5) - (((2.0 * locals.var_qs_1_dn5) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn5))))), ((locals.var_nvt_dn6 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn6 - locals.var_psip_dn6) - (((2.0 * locals.var_qs_1_dn6) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn6))))), ((locals.var_nvt_dn7 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn7 - locals.var_psip_dn7) - (((2.0 * locals.var_qs_1_dn7) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn7))))), ((locals.var_nvt_dn8 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn8 - locals.var_psip_dn8) - (((2.0 * locals.var_qs_1_dn8) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn8))))), ((locals.var_nvt_dn9 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn9 - locals.var_psip_dn9) - (((2.0 * locals.var_qs_1_dn9) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn9))))), ((locals.var_nvt_dn10 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn10 - locals.var_psip_dn10) - (((2.0 * locals.var_qs_1_dn10) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn10))))), ((locals.var_nvt_dn11 * assign45650_e77811) + (locals.var_nvt * ((locals.var_vgfb_dn11 - locals.var_psip_dn11) - (((2.0 * locals.var_qs_1_dn11) * assign45650_e77809) + (assign45650_e77806 * locals.var_nq_dn11))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign45650_e77814;
        locals.var_t0_dn3 = assign45650_e77814_d_n3;
        locals.var_t0_dn4 = assign45650_e77814_d_n4;
        locals.var_t0_dn5 = assign45650_e77814_d_n5;
        locals.var_t0_dn6 = assign45650_e77814_d_n6;
        locals.var_t0_dn7 = assign45650_e77814_d_n7;
        locals.var_t0_dn8 = assign45650_e77814_d_n8;
        locals.var_t0_dn9 = assign45650_e77814_d_n9;
        locals.var_t0_dn10 = assign45650_e77814_d_n10;
        locals.var_t0_dn11 = assign45650_e77814_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign45660_e77838, assign45660_e77838_d_n3, assign45660_e77838_d_n4, assign45660_e77838_d_n5, assign45660_e77838_d_n6, assign45660_e77838_d_n7, assign45660_e77838_d_n8, assign45660_e77838_d_n9, assign45660_e77838_d_n10, assign45660_e77838_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45660_e77820: f64 = locals.var_t0;
        let assign45660_e77823: f64 = locals.var_t0;
        let assign45660_e77826: f64 = locals.var_t0;
        let assign45660_e77827: f64 = (assign45660_e77823 * assign45660_e77826);
        let assign45660_e77830: f64 = (0.25 * 0.1);
        let assign45660_e77832: f64 = (assign45660_e77830 * 0.1);
        let assign45660_e77833: f64 = (assign45660_e77827 + assign45660_e77832);
        let assign45660_e77834: f64 = (assign45660_e77833).sqrt();
        let assign45660_e77835: f64 = (assign45660_e77820 + assign45660_e77834);
        let assign45660_e77836: f64 = (0.5 * assign45660_e77835);
        (assign45660_e77836, (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn3)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn4)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn5)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn6)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn7)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn8)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn9)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn10)) / (2.0 * assign45660_e77834)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign45660_e77826) + (assign45660_e77823 * locals.var_t0_dn11)) / (2.0 * assign45660_e77834)))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn3, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11,)
    }
};
        locals.var_qbs = assign45660_e77838;
        locals.var_qbs_dn3 = assign45660_e77838_d_n3;
        locals.var_qbs_dn4 = assign45660_e77838_d_n4;
        locals.var_qbs_dn5 = assign45660_e77838_d_n5;
        locals.var_qbs_dn6 = assign45660_e77838_d_n6;
        locals.var_qbs_dn7 = assign45660_e77838_d_n7;
        locals.var_qbs_dn8 = assign45660_e77838_d_n8;
        locals.var_qbs_dn9 = assign45660_e77838_d_n9;
        locals.var_qbs_dn10 = assign45660_e77838_d_n10;
        locals.var_qbs_dn11 = assign45660_e77838_d_n11;
        locals.var_qbs_rv = 0.0;

        let (assign45670_e77849, assign45670_e77849_d_n3, assign45670_e77849_d_n4, assign45670_e77849_d_n5, assign45670_e77849_d_n6, assign45670_e77849_d_n7, assign45670_e77849_d_n8, assign45670_e77849_d_n9, assign45670_e77849_d_n10, assign45670_e77849_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45670_e77843: f64 = (2.0 * locals.var_nq);
        let assign45670_e77845: f64 = (assign45670_e77843 * locals.var_nvt);
        let assign45670_e77847: f64 = (assign45670_e77845 * locals.var_qs_1);
        (assign45670_e77847, (((((2.0 * locals.var_nq_dn3) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn3)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn3)), (((((2.0 * locals.var_nq_dn4) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn4)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn4)), (((((2.0 * locals.var_nq_dn5) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn5)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn5)), (((((2.0 * locals.var_nq_dn6) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn6)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn6)), (((((2.0 * locals.var_nq_dn7) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn7)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn7)), (((((2.0 * locals.var_nq_dn8) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn8)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn8)), (((((2.0 * locals.var_nq_dn9) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn9)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn9)), (((((2.0 * locals.var_nq_dn10) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn10)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn10)), (((((2.0 * locals.var_nq_dn11) * locals.var_nvt) + (assign45670_e77843 * locals.var_nvt_dn11)) * locals.var_qs_1) + (assign45670_e77845 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_qis, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11,)
    }
};
        locals.var_qis = assign45670_e77849;
        locals.var_qis_dn3 = assign45670_e77849_d_n3;
        locals.var_qis_dn4 = assign45670_e77849_d_n4;
        locals.var_qis_dn5 = assign45670_e77849_d_n5;
        locals.var_qis_dn6 = assign45670_e77849_d_n6;
        locals.var_qis_dn7 = assign45670_e77849_d_n7;
        locals.var_qis_dn8 = assign45670_e77849_d_n8;
        locals.var_qis_dn9 = assign45670_e77849_d_n9;
        locals.var_qis_dn10 = assign45670_e77849_d_n10;
        locals.var_qis_dn11 = assign45670_e77849_d_n11;
        locals.var_qis_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_158(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign45680_e77860, assign45680_e77860_d_n3, assign45680_e77860_d_n4, assign45680_e77860_d_n5, assign45680_e77860_d_n6, assign45680_e77860_d_n7, assign45680_e77860_d_n8, assign45680_e77860_d_n9, assign45680_e77860_d_n10, assign45680_e77860_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45680_e77856: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign45680_e77857: f64 = (locals.var_qbs + assign45680_e77856);
        let assign45680_e77858: f64 = (locals.var_eefffactor * assign45680_e77857);
        (assign45680_e77858, (locals.var_eefffactor * (locals.var_qbs_dn3 + (locals.var_eta_mu * locals.var_qis_dn3))), (locals.var_eefffactor * (locals.var_qbs_dn4 + (locals.var_eta_mu * locals.var_qis_dn4))), (locals.var_eefffactor * (locals.var_qbs_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_eefffactor * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_eefffactor * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_eefffactor * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_eefffactor * (locals.var_qbs_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))), (locals.var_eefffactor * (locals.var_qbs_dn10 + (locals.var_eta_mu * locals.var_qis_dn10))), (locals.var_eefffactor * (locals.var_qbs_dn11 + (locals.var_eta_mu * locals.var_qis_dn11))),)
    } else {
        (locals.var_eeffs, locals.var_eeffs_dn3, locals.var_eeffs_dn4, locals.var_eeffs_dn5, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8, locals.var_eeffs_dn9, locals.var_eeffs_dn10, locals.var_eeffs_dn11,)
    }
};
        locals.var_eeffs = assign45680_e77860;
        locals.var_eeffs_dn3 = assign45680_e77860_d_n3;
        locals.var_eeffs_dn4 = assign45680_e77860_d_n4;
        locals.var_eeffs_dn5 = assign45680_e77860_d_n5;
        locals.var_eeffs_dn6 = assign45680_e77860_d_n6;
        locals.var_eeffs_dn7 = assign45680_e77860_d_n7;
        locals.var_eeffs_dn8 = assign45680_e77860_d_n8;
        locals.var_eeffs_dn9 = assign45680_e77860_d_n9;
        locals.var_eeffs_dn10 = assign45680_e77860_d_n10;
        locals.var_eeffs_dn11 = assign45680_e77860_d_n11;
        locals.var_eeffs_rv = 0.0;

        let (assign45690_e77873, assign45690_e77873_d_n3, assign45690_e77873_d_n4, assign45690_e77873_d_n5, assign45690_e77873_d_n6, assign45690_e77873_d_n7, assign45690_e77873_d_n8, assign45690_e77873_d_n9, assign45690_e77873_d_n10, assign45690_e77873_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45690_e77867: f64 = (locals.var_qis / locals.var_qbs);
        let assign45690_e77868: f64 = (1.0 + assign45690_e77867);
        let assign45690_e77869: f64 = (0.5 * assign45690_e77868);
        let assign45690_e77871: f64 = (assign45690_e77869).powf(locals.var_ucs_a);
        (assign45690_e77871, if locals.var_ucs_a_dn3 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn3 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn3)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn3 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn3 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn3)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn4 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn4 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn4)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn4 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn4 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn4)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn5 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn5 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn5)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn5 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn5 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn5)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn6 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn6 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn6)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn6 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn6 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn6)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn7 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn7 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn7)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn7 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn7 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn7)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn8 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn8 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn8)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn8 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn8 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn8)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn9 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn9 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn9)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn9 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn9 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn9)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn10 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn10 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn10)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn10 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn10 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn10)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) }, if locals.var_ucs_a_dn11 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign45690_e77869).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qis_dn11 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn11)) / (locals.var_qbs * locals.var_qbs))))) } } else { (assign45690_e77871 * ((locals.var_ucs_a_dn11 * (assign45690_e77869).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qis_dn11 * locals.var_qbs) - (locals.var_qis * locals.var_qbs_dn11)) / (locals.var_qbs * locals.var_qbs))) / assign45690_e77869)))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign45690_e77873;
        locals.var_t2_dn3 = assign45690_e77873_d_n3;
        locals.var_t2_dn4 = assign45690_e77873_d_n4;
        locals.var_t2_dn5 = assign45690_e77873_d_n5;
        locals.var_t2_dn6 = assign45690_e77873_d_n6;
        locals.var_t2_dn7 = assign45690_e77873_d_n7;
        locals.var_t2_dn8 = assign45690_e77873_d_n8;
        locals.var_t2_dn9 = assign45690_e77873_d_n9;
        locals.var_t2_dn10 = assign45690_e77873_d_n10;
        locals.var_t2_dn11 = assign45690_e77873_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign45700_e77890, assign45700_e77890_d_n3, assign45700_e77890_d_n4, assign45700_e77890_d_n5, assign45700_e77890_d_n6, assign45700_e77890_d_n7, assign45700_e77890_d_n8, assign45700_e77890_d_n9, assign45700_e77890_d_n10, assign45700_e77890_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45700_e77879: f64 = (locals.var_uc_a * locals.var_vbsx);
        let assign45700_e77880: f64 = (locals.var_ua_a + assign45700_e77879);
        let assign45700_e77883: f64 = (locals.var_eeffs).powf(locals.var_eu_t);
        let assign45700_e77884: f64 = (assign45700_e77880 * assign45700_e77883);
        let assign45700_e77887: f64 = (locals.var_ud_a / locals.var_t2);
        let assign45700_e77888: f64 = (assign45700_e77884 + assign45700_e77887);
        (assign45700_e77888, ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn3))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn3 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn3)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn3 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn3 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn3 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn4))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn4 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn4)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn4 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn4 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn4 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn5))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn5 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn5)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn5 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn5 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn5 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn6))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn6 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn6)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn6 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn6 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn6 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn7))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn7 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn7)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn7 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn7 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn7 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn8))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn8 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn8)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn8 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn8 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn8 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn9))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn9 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn9)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn9 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn9 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn9 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn10))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn10 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn10)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn10 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn10 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn10 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn11))) * assign45700_e77883) + (assign45700_e77880 * if locals.var_eu_t_dn11 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn11)) } } else { (assign45700_e77883 * ((locals.var_eu_t_dn11 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn11 / locals.var_eeffs)))) })) + (((locals.var_ud_a_dn11 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign45700_e77890;
        locals.var_t3_dn3 = assign45700_e77890_d_n3;
        locals.var_t3_dn4 = assign45700_e77890_d_n4;
        locals.var_t3_dn5 = assign45700_e77890_d_n5;
        locals.var_t3_dn6 = assign45700_e77890_d_n6;
        locals.var_t3_dn7 = assign45700_e77890_d_n7;
        locals.var_t3_dn8 = assign45700_e77890_d_n8;
        locals.var_t3_dn9 = assign45700_e77890_d_n9;
        locals.var_t3_dn10 = assign45700_e77890_d_n10;
        locals.var_t3_dn11 = assign45700_e77890_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign45710_e77897, assign45710_e77897_d_n3, assign45710_e77897_d_n4, assign45710_e77897_d_n5, assign45710_e77897_d_n6, assign45710_e77897_d_n7, assign45710_e77897_d_n8, assign45710_e77897_d_n9, assign45710_e77897_d_n10, assign45710_e77897_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45710_e77895: f64 = (1.0 + locals.var_t3);
        (assign45710_e77895, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign45710_e77897;
        locals.var_t4_dn3 = assign45710_e77897_d_n3;
        locals.var_t4_dn4 = assign45710_e77897_d_n4;
        locals.var_t4_dn5 = assign45710_e77897_d_n5;
        locals.var_t4_dn6 = assign45710_e77897_d_n6;
        locals.var_t4_dn7 = assign45710_e77897_d_n7;
        locals.var_t4_dn8 = assign45710_e77897_d_n8;
        locals.var_t4_dn9 = assign45710_e77897_d_n9;
        locals.var_t4_dn10 = assign45710_e77897_d_n10;
        locals.var_t4_dn11 = assign45710_e77897_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign45720_e77921, assign45720_e77921_d_n3, assign45720_e77921_d_n4, assign45720_e77921_d_n5, assign45720_e77921_d_n6, assign45720_e77921_d_n7, assign45720_e77921_d_n8, assign45720_e77921_d_n9, assign45720_e77921_d_n10, assign45720_e77921_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45720_e77903: f64 = (locals.var_t4 + 1.0);
        let assign45720_e77906: f64 = (locals.var_t4 - 1.0);
        let assign45720_e77909: f64 = (locals.var_t4 - 1.0);
        let assign45720_e77910: f64 = (assign45720_e77906 * assign45720_e77909);
        let assign45720_e77913: f64 = (0.25 * 0.0015);
        let assign45720_e77915: f64 = (assign45720_e77913 * 0.0015);
        let assign45720_e77916: f64 = (assign45720_e77910 + assign45720_e77915);
        let assign45720_e77917: f64 = (assign45720_e77916).sqrt();
        let assign45720_e77918: f64 = (assign45720_e77903 + assign45720_e77917);
        let assign45720_e77919: f64 = (0.5 * assign45720_e77918);
        (assign45720_e77919, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn3)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn4)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn5)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn6)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn7)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn8)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn9)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn10)) / (2.0 * assign45720_e77917)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign45720_e77909) + (assign45720_e77906 * locals.var_t4_dn11)) / (2.0 * assign45720_e77917)))),)
    } else {
        (locals.var_dmobs, locals.var_dmobs_dn3, locals.var_dmobs_dn4, locals.var_dmobs_dn5, locals.var_dmobs_dn6, locals.var_dmobs_dn7, locals.var_dmobs_dn8, locals.var_dmobs_dn9, locals.var_dmobs_dn10, locals.var_dmobs_dn11,)
    }
};
        locals.var_dmobs = assign45720_e77921;
        locals.var_dmobs_dn3 = assign45720_e77921_d_n3;
        locals.var_dmobs_dn4 = assign45720_e77921_d_n4;
        locals.var_dmobs_dn5 = assign45720_e77921_d_n5;
        locals.var_dmobs_dn6 = assign45720_e77921_d_n6;
        locals.var_dmobs_dn7 = assign45720_e77921_d_n7;
        locals.var_dmobs_dn8 = assign45720_e77921_d_n8;
        locals.var_dmobs_dn9 = assign45720_e77921_d_n9;
        locals.var_dmobs_dn10 = assign45720_e77921_d_n10;
        locals.var_dmobs_dn11 = assign45720_e77921_d_n11;
        locals.var_dmobs_rv = 0.0;

        let (assign45730_e77934,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45730_e77927: f64 = (locals.var_weff * 1000000.0);
        let assign45730_e77929: f64 = (assign45730_e77927).powf(locals.var_wr_i);
        let assign45730_e77931: f64 = (assign45730_e77929 * p.p2);
        let assign45730_e77932: f64 = (1.0 / assign45730_e77931);
        (assign45730_e77932,)
    } else {
        (locals.var_weffwrfactor,)
    }
};
        locals.var_weffwrfactor = assign45730_e77934;
        locals.var_weffwrfactor_rv = 0.0;

        let assign45740_e77937: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard712 = assign45740_e77937;
        locals.var_guard712_rv = 0.0;

        let (assign45750_e77944, assign45750_e77944_d_n3, assign45750_e77944_d_n4, assign45750_e77944_d_n5, assign45750_e77944_d_n6, assign45750_e77944_d_n7, assign45750_e77944_d_n8, assign45750_e77944_d_n9, assign45750_e77944_d_n10, assign45750_e77944_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard712 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdss, locals.var_rdss_dn3, locals.var_rdss_dn4, locals.var_rdss_dn5, locals.var_rdss_dn6, locals.var_rdss_dn7, locals.var_rdss_dn8, locals.var_rdss_dn9, locals.var_rdss_dn10, locals.var_rdss_dn11,)
    }
};
        locals.var_rdss = assign45750_e77944;
        locals.var_rdss_dn3 = assign45750_e77944_d_n3;
        locals.var_rdss_dn4 = assign45750_e77944_d_n4;
        locals.var_rdss_dn5 = assign45750_e77944_d_n5;
        locals.var_rdss_dn6 = assign45750_e77944_d_n6;
        locals.var_rdss_dn7 = assign45750_e77944_d_n7;
        locals.var_rdss_dn8 = assign45750_e77944_d_n8;
        locals.var_rdss_dn9 = assign45750_e77944_d_n9;
        locals.var_rdss_dn10 = assign45750_e77944_d_n10;
        locals.var_rdss_dn11 = assign45750_e77944_d_n11;
        locals.var_rdss_rv = 0.0;

        let (assign45760_e77956, assign45760_e77956_d_n3, assign45760_e77956_d_n4, assign45760_e77956_d_n5, assign45760_e77956_d_n6, assign45760_e77956_d_n7, assign45760_e77956_d_n8, assign45760_e77956_d_n9, assign45760_e77956_d_n10, assign45760_e77956_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard712 == 0.0)) {
        let assign45760_e77953: f64 = (locals.var_prwg_i * locals.var_qis);
        let assign45760_e77954: f64 = (1.0 + assign45760_e77953);
        (assign45760_e77954, (locals.var_prwg_i * locals.var_qis_dn3), (locals.var_prwg_i * locals.var_qis_dn4), (locals.var_prwg_i * locals.var_qis_dn5), (locals.var_prwg_i * locals.var_qis_dn6), (locals.var_prwg_i * locals.var_qis_dn7), (locals.var_prwg_i * locals.var_qis_dn8), (locals.var_prwg_i * locals.var_qis_dn9), (locals.var_prwg_i * locals.var_qis_dn10), (locals.var_prwg_i * locals.var_qis_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign45760_e77956;
        locals.var_t0_dn3 = assign45760_e77956_d_n3;
        locals.var_t0_dn4 = assign45760_e77956_d_n4;
        locals.var_t0_dn5 = assign45760_e77956_d_n5;
        locals.var_t0_dn6 = assign45760_e77956_d_n6;
        locals.var_t0_dn7 = assign45760_e77956_d_n7;
        locals.var_t0_dn8 = assign45760_e77956_d_n8;
        locals.var_t0_dn9 = assign45760_e77956_d_n9;
        locals.var_t0_dn10 = assign45760_e77956_d_n10;
        locals.var_t0_dn11 = assign45760_e77956_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign45770_e77968, assign45770_e77968_d_n3, assign45770_e77968_d_n4, assign45770_e77968_d_n5, assign45770_e77968_d_n6, assign45770_e77968_d_n7, assign45770_e77968_d_n8, assign45770_e77968_d_n9, assign45770_e77968_d_n10, assign45770_e77968_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard712 == 0.0)) {
        let assign45770_e77965: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign45770_e77966: f64 = (locals.var_prwb_i * assign45770_e77965);
        (assign45770_e77966, (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign45770_e77968;
        locals.var_t1_dn3 = assign45770_e77968_d_n3;
        locals.var_t1_dn4 = assign45770_e77968_d_n4;
        locals.var_t1_dn5 = assign45770_e77968_d_n5;
        locals.var_t1_dn6 = assign45770_e77968_d_n6;
        locals.var_t1_dn7 = assign45770_e77968_d_n7;
        locals.var_t1_dn8 = assign45770_e77968_d_n8;
        locals.var_t1_dn9 = assign45770_e77968_d_n9;
        locals.var_t1_dn10 = assign45770_e77968_d_n10;
        locals.var_t1_dn11 = assign45770_e77968_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign45780_e77980, assign45780_e77980_d_n3, assign45780_e77980_d_n4, assign45780_e77980_d_n5, assign45780_e77980_d_n6, assign45780_e77980_d_n7, assign45780_e77980_d_n8, assign45780_e77980_d_n9, assign45780_e77980_d_n10, assign45780_e77980_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard712 == 0.0)) {
        let assign45780_e77976: f64 = (1.0 / locals.var_t0);
        let assign45780_e77978: f64 = (assign45780_e77976 + locals.var_t1);
        (assign45780_e77978, ((-(locals.var_t0_dn3 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn3), ((-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn4), ((-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn5), ((-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn6), ((-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn7), ((-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn8), ((-(locals.var_t0_dn9 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn9), ((-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn10), ((-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign45780_e77980;
        locals.var_t2_dn3 = assign45780_e77980_d_n3;
        locals.var_t2_dn4 = assign45780_e77980_d_n4;
        locals.var_t2_dn5 = assign45780_e77980_d_n5;
        locals.var_t2_dn6 = assign45780_e77980_d_n6;
        locals.var_t2_dn7 = assign45780_e77980_d_n7;
        locals.var_t2_dn8 = assign45780_e77980_d_n8;
        locals.var_t2_dn9 = assign45780_e77980_d_n9;
        locals.var_t2_dn10 = assign45780_e77980_d_n10;
        locals.var_t2_dn11 = assign45780_e77980_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign45790_e77995, assign45790_e77995_d_n3, assign45790_e77995_d_n4, assign45790_e77995_d_n5, assign45790_e77995_d_n6, assign45790_e77995_d_n7, assign45790_e77995_d_n8, assign45790_e77995_d_n9, assign45790_e77995_d_n10, assign45790_e77995_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard712 == 0.0)) {
        let assign45790_e77989: f64 = (locals.var_t2 * locals.var_t2);
        let assign45790_e77991: f64 = (assign45790_e77989 + 0.01);
        let assign45790_e77992: f64 = (assign45790_e77991).sqrt();
        let assign45790_e77993: f64 = (locals.var_t2 + assign45790_e77992);
        (assign45790_e77993, (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign45790_e77992))), (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign45790_e77992))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign45790_e77995;
        locals.var_t3_dn3 = assign45790_e77995_d_n3;
        locals.var_t3_dn4 = assign45790_e77995_d_n4;
        locals.var_t3_dn5 = assign45790_e77995_d_n5;
        locals.var_t3_dn6 = assign45790_e77995_d_n6;
        locals.var_t3_dn7 = assign45790_e77995_d_n7;
        locals.var_t3_dn8 = assign45790_e77995_d_n8;
        locals.var_t3_dn9 = assign45790_e77995_d_n9;
        locals.var_t3_dn10 = assign45790_e77995_d_n10;
        locals.var_t3_dn11 = assign45790_e77995_d_n11;
        locals.var_t3_rv = 0.0;

        let assign45800_e77998: f64 = if p.p33 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard713 = assign45800_e77998;
        locals.var_guard713_rv = 0.0;

        let (assign45810_e78018, assign45810_e78018_d_n3, assign45810_e78018_d_n4, assign45810_e78018_d_n5, assign45810_e78018_d_n6, assign45810_e78018_d_n7, assign45810_e78018_d_n8, assign45810_e78018_d_n9, assign45810_e78018_d_n10, assign45810_e78018_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard712 == 0.0)) && (locals.var_guard713 != 0.0)) {
        let assign45810_e78009: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign45810_e78010: f64 = (locals.var_rdswmin_i + assign45810_e78009);
        let assign45810_e78012: f64 = (assign45810_e78010 * locals.var_weffwrfactor);
        let assign45810_e78014: f64 = (assign45810_e78012 * p.p2);
        let assign45810_e78016: f64 = (assign45810_e78014 * locals.var_rdstemp);
        (assign45810_e78016, ((((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), (((((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp) + (assign45810_e78014 * locals.var_rdstemp_dn4)), (((((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp) + (assign45810_e78014 * locals.var_rdstemp_dn5)), ((((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp),)
    } else {
        (locals.var_rdss, locals.var_rdss_dn3, locals.var_rdss_dn4, locals.var_rdss_dn5, locals.var_rdss_dn6, locals.var_rdss_dn7, locals.var_rdss_dn8, locals.var_rdss_dn9, locals.var_rdss_dn10, locals.var_rdss_dn11,)
    }
};
        locals.var_rdss = assign45810_e78018;
        locals.var_rdss_dn3 = assign45810_e78018_d_n3;
        locals.var_rdss_dn4 = assign45810_e78018_d_n4;
        locals.var_rdss_dn5 = assign45810_e78018_d_n5;
        locals.var_rdss_dn6 = assign45810_e78018_d_n6;
        locals.var_rdss_dn7 = assign45810_e78018_d_n7;
        locals.var_rdss_dn8 = assign45810_e78018_d_n8;
        locals.var_rdss_dn9 = assign45810_e78018_d_n9;
        locals.var_rdss_dn10 = assign45810_e78018_d_n10;
        locals.var_rdss_dn11 = assign45810_e78018_d_n11;
        locals.var_rdss_rv = 0.0;

        let (assign45820_e78043, assign45820_e78043_d_n3, assign45820_e78043_d_n4, assign45820_e78043_d_n5, assign45820_e78043_d_n6, assign45820_e78043_d_n7, assign45820_e78043_d_n8, assign45820_e78043_d_n9, assign45820_e78043_d_n10, assign45820_e78043_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard712 == 0.0)) && (locals.var_guard713 == 0.0)) {
        let assign45820_e78031: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign45820_e78032: f64 = (locals.var_rdswmin_i + assign45820_e78031);
        let assign45820_e78034: f64 = (assign45820_e78032 * locals.var_weffwrfactor);
        let assign45820_e78036: f64 = (assign45820_e78034 * p.p2);
        let assign45820_e78037: f64 = (locals.var_rsourcegeo + assign45820_e78036);
        let assign45820_e78039: f64 = (assign45820_e78037 + locals.var_rdraingeo);
        let assign45820_e78041: f64 = (assign45820_e78039 * locals.var_rdstemp);
        (assign45820_e78041, ((((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), (((((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp) + (assign45820_e78039 * locals.var_rdstemp_dn4)), (((((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp) + (assign45820_e78039 * locals.var_rdstemp_dn5)), ((((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp), ((((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2) * locals.var_rdstemp),)
    } else {
        (locals.var_rdss, locals.var_rdss_dn3, locals.var_rdss_dn4, locals.var_rdss_dn5, locals.var_rdss_dn6, locals.var_rdss_dn7, locals.var_rdss_dn8, locals.var_rdss_dn9, locals.var_rdss_dn10, locals.var_rdss_dn11,)
    }
};
        locals.var_rdss = assign45820_e78043;
        locals.var_rdss_dn3 = assign45820_e78043_d_n3;
        locals.var_rdss_dn4 = assign45820_e78043_d_n4;
        locals.var_rdss_dn5 = assign45820_e78043_d_n5;
        locals.var_rdss_dn6 = assign45820_e78043_d_n6;
        locals.var_rdss_dn7 = assign45820_e78043_d_n7;
        locals.var_rdss_dn8 = assign45820_e78043_d_n8;
        locals.var_rdss_dn9 = assign45820_e78043_d_n9;
        locals.var_rdss_dn10 = assign45820_e78043_d_n10;
        locals.var_rdss_dn11 = assign45820_e78043_d_n11;
        locals.var_rdss_rv = 0.0;

        let (assign45830_e78052, assign45830_e78052_d_n3, assign45830_e78052_d_n4, assign45830_e78052_d_n5, assign45830_e78052_d_n6, assign45830_e78052_d_n7, assign45830_e78052_d_n8, assign45830_e78052_d_n9, assign45830_e78052_d_n10, assign45830_e78052_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45830_e78049: f64 = (1.0 / locals.var_psat_a);
        let assign45830_e78050: f64 = (locals.var_dmobs).powf(assign45830_e78049);
        (assign45830_e78050, if (-(locals.var_psat_a_dn3 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn3)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn3 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn3 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn4 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn4)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn4 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn4 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn5 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn5)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn5 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn5 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn6 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn6)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn6 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn6 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn7 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn7)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn7 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn7 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn8 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn8)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn8 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn8 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn9 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn9)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn9 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn9 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn10 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn10)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn10 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn10 / locals.var_dmobs)))) }, if (-(locals.var_psat_a_dn11 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign45830_e78049) as f64).is_finite() && ((assign45830_e78049) as f64).fract() == 0.0 { if assign45830_e78049 == 0.0 { 0.0 } else { (assign45830_e78049 * ((locals.var_dmobs).powf(assign45830_e78049 - 1.0) * locals.var_dmobs_dn11)) } } else { (assign45830_e78050 * (((-(locals.var_psat_a_dn11 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmobs).ln()) + (assign45830_e78049 * (locals.var_dmobs_dn11 / locals.var_dmobs)))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign45830_e78052;
        locals.var_t0_dn3 = assign45830_e78052_d_n3;
        locals.var_t0_dn4 = assign45830_e78052_d_n4;
        locals.var_t0_dn5 = assign45830_e78052_d_n5;
        locals.var_t0_dn6 = assign45830_e78052_d_n6;
        locals.var_t0_dn7 = assign45830_e78052_d_n7;
        locals.var_t0_dn8 = assign45830_e78052_d_n8;
        locals.var_t0_dn9 = assign45830_e78052_d_n9;
        locals.var_t0_dn10 = assign45830_e78052_d_n10;
        locals.var_t0_dn11 = assign45830_e78052_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign45840_e78059, assign45840_e78059_d_n3, assign45840_e78059_d_n4, assign45840_e78059_d_n5, assign45840_e78059_d_n6, assign45840_e78059_d_n7, assign45840_e78059_d_n8, assign45840_e78059_d_n9, assign45840_e78059_d_n10, assign45840_e78059_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45840_e78057: f64 = (locals.var_psatb_i * locals.var_vbsx);
        (assign45840_e78057, (locals.var_psatb_i * locals.var_vbsx_dn3), (locals.var_psatb_i * locals.var_vbsx_dn4), (locals.var_psatb_i * locals.var_vbsx_dn5), (locals.var_psatb_i * locals.var_vbsx_dn6), (locals.var_psatb_i * locals.var_vbsx_dn7), (locals.var_psatb_i * locals.var_vbsx_dn8), (locals.var_psatb_i * locals.var_vbsx_dn9), (locals.var_psatb_i * locals.var_vbsx_dn10), (locals.var_psatb_i * locals.var_vbsx_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign45840_e78059;
        locals.var_t11_dn3 = assign45840_e78059_d_n3;
        locals.var_t11_dn4 = assign45840_e78059_d_n4;
        locals.var_t11_dn5 = assign45840_e78059_d_n5;
        locals.var_t11_dn6 = assign45840_e78059_d_n6;
        locals.var_t11_dn7 = assign45840_e78059_d_n7;
        locals.var_t11_dn8 = assign45840_e78059_d_n8;
        locals.var_t11_dn9 = assign45840_e78059_d_n9;
        locals.var_t11_dn10 = assign45840_e78059_d_n10;
        locals.var_t11_dn11 = assign45840_e78059_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign45850_e78069, assign45850_e78069_d_n3, assign45850_e78069_d_n4, assign45850_e78069_d_n5, assign45850_e78069_d_n6, assign45850_e78069_d_n7, assign45850_e78069_d_n8, assign45850_e78069_d_n9, assign45850_e78069_d_n10, assign45850_e78069_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45850_e78065: f64 = (locals.var_t11 * locals.var_t11);
        let assign45850_e78066: f64 = (0.1 + assign45850_e78065);
        let assign45850_e78067: f64 = (assign45850_e78066).sqrt();
        (assign45850_e78067, (((locals.var_t11_dn3 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn3)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn4 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn4)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn5 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn5)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn6 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn6)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn7 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn7)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn8 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn8)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn9 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn9)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn10 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn10)) / (2.0 * assign45850_e78067)), (((locals.var_t11_dn11 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn11)) / (2.0 * assign45850_e78067)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign45850_e78069;
        locals.var_t12_dn3 = assign45850_e78069_d_n3;
        locals.var_t12_dn4 = assign45850_e78069_d_n4;
        locals.var_t12_dn5 = assign45850_e78069_d_n5;
        locals.var_t12_dn6 = assign45850_e78069_d_n6;
        locals.var_t12_dn7 = assign45850_e78069_d_n7;
        locals.var_t12_dn8 = assign45850_e78069_d_n8;
        locals.var_t12_dn9 = assign45850_e78069_d_n9;
        locals.var_t12_dn10 = assign45850_e78069_d_n10;
        locals.var_t12_dn11 = assign45850_e78069_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign45860_e78089, assign45860_e78089_d_n3, assign45860_e78089_d_n4, assign45860_e78089_d_n5, assign45860_e78089_d_n6, assign45860_e78089_d_n7, assign45860_e78089_d_n8, assign45860_e78089_d_n9, assign45860_e78089_d_n10, assign45860_e78089_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45860_e78075: f64 = (1.0 - locals.var_t11);
        let assign45860_e78078: f64 = (1.0 - locals.var_t11);
        let assign45860_e78081: f64 = (1.0 - locals.var_t11);
        let assign45860_e78082: f64 = (assign45860_e78078 * assign45860_e78081);
        let assign45860_e78084: f64 = (assign45860_e78082 + locals.var_t12);
        let assign45860_e78085: f64 = (assign45860_e78084).sqrt();
        let assign45860_e78086: f64 = (assign45860_e78075 + assign45860_e78085);
        let assign45860_e78087: f64 = (0.5 * assign45860_e78086);
        (assign45860_e78087, (0.5 * ((-locals.var_t11_dn3) + (((((-locals.var_t11_dn3) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn3))) + locals.var_t12_dn3) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn4) + (((((-locals.var_t11_dn4) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn4))) + locals.var_t12_dn4) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn5) + (((((-locals.var_t11_dn5) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn5))) + locals.var_t12_dn5) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn6) + (((((-locals.var_t11_dn6) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn6))) + locals.var_t12_dn6) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn7) + (((((-locals.var_t11_dn7) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn7))) + locals.var_t12_dn7) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn8) + (((((-locals.var_t11_dn8) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn8))) + locals.var_t12_dn8) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn9) + (((((-locals.var_t11_dn9) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn9))) + locals.var_t12_dn9) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn10) + (((((-locals.var_t11_dn10) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn10))) + locals.var_t12_dn10) / (2.0 * assign45860_e78085)))), (0.5 * ((-locals.var_t11_dn11) + (((((-locals.var_t11_dn11) * assign45860_e78081) + (assign45860_e78078 * (-locals.var_t11_dn11))) + locals.var_t12_dn11) / (2.0 * assign45860_e78085)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign45860_e78089;
        locals.var_t1_dn3 = assign45860_e78089_d_n3;
        locals.var_t1_dn4 = assign45860_e78089_d_n4;
        locals.var_t1_dn5 = assign45860_e78089_d_n5;
        locals.var_t1_dn6 = assign45860_e78089_d_n6;
        locals.var_t1_dn7 = assign45860_e78089_d_n7;
        locals.var_t1_dn8 = assign45860_e78089_d_n8;
        locals.var_t1_dn9 = assign45860_e78089_d_n9;
        locals.var_t1_dn10 = assign45860_e78089_d_n10;
        locals.var_t1_dn11 = assign45860_e78089_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign45870_e78108, assign45870_e78108_d_n3, assign45870_e78108_d_n4, assign45870_e78108_d_n5, assign45870_e78108_d_n6, assign45870_e78108_d_n7, assign45870_e78108_d_n8, assign45870_e78108_d_n9, assign45870_e78108_d_n10, assign45870_e78108_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign45870_e78094: f64 = (10.0 * p.p497);
        let assign45870_e78096: f64 = (assign45870_e78094 * locals.var_qs_1);
        let assign45870_e78098: f64 = (assign45870_e78096 * locals.var_t1);
        let assign45870_e78101: f64 = (10.0 * p.p497);
        let assign45870_e78104: f64 = (locals.var_qs_1 * locals.var_t1);
        let assign45870_e78105: f64 = (assign45870_e78101 + assign45870_e78104);
        let assign45870_e78106: f64 = (assign45870_e78098 / assign45870_e78105);
        (assign45870_e78106, ((((((assign45870_e78094 * locals.var_qs_1_dn3) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn3)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn3 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn3)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn4) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn4)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn4 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn4)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn5) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn5)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn5 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn5)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn6) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn6)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn6 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn6)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn7) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn7)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn7 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn7)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn8) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn8)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn8 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn8)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn9) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn9)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn9 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn9)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn10) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn10)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn10 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn10)))) / (assign45870_e78105 * assign45870_e78105)), ((((((assign45870_e78094 * locals.var_qs_1_dn11) * locals.var_t1) + (assign45870_e78096 * locals.var_t1_dn11)) * assign45870_e78105) - (assign45870_e78098 * ((locals.var_qs_1_dn11 * locals.var_t1) + (locals.var_qs_1 * locals.var_t1_dn11)))) / (assign45870_e78105 * assign45870_e78105)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign45870_e78108;
        locals.var_t2_dn3 = assign45870_e78108_d_n3;
        locals.var_t2_dn4 = assign45870_e78108_d_n4;
        locals.var_t2_dn5 = assign45870_e78108_d_n5;
        locals.var_t2_dn6 = assign45870_e78108_d_n6;
        locals.var_t2_dn7 = assign45870_e78108_d_n7;
        locals.var_t2_dn8 = assign45870_e78108_d_n8;
        locals.var_t2_dn9 = assign45870_e78108_d_n9;
        locals.var_t2_dn10 = assign45870_e78108_d_n10;
        locals.var_t2_dn11 = assign45870_e78108_d_n11;
        locals.var_t2_rv = 0.0;

        let assign45880_e78111: f64 = if locals.var_ptwg_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard714 = assign45880_e78111;
        locals.var_guard714_rv = 0.0;

        let (assign45890_e78136, assign45890_e78136_d_n3, assign45890_e78136_d_n4, assign45890_e78136_d_n5, assign45890_e78136_d_n6, assign45890_e78136_d_n7, assign45890_e78136_d_n8, assign45890_e78136_d_n9, assign45890_e78136_d_n10, assign45890_e78136_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard714 != 0.0)) {
        let assign45890_e78119: f64 = (locals.var_u0_a / locals.var_t0);
        let assign45890_e78121: f64 = (assign45890_e78119 * locals.var_nvt);
        let assign45890_e78124: f64 = (locals.var_vsat_a * locals.var_leff);
        let assign45890_e78125: f64 = (assign45890_e78121 / assign45890_e78124);
        let assign45890_e78126: f64 = (2.0 * assign45890_e78125);
        let assign45890_e78131: f64 = (locals.var_ptwg_a * locals.var_t2);
        let assign45890_e78132: f64 = (1.0 - assign45890_e78131);
        let assign45890_e78133: f64 = (1.0 / assign45890_e78132);
        let assign45890_e78134: f64 = (assign45890_e78126 * assign45890_e78133);
        (assign45890_e78134, (((2.0 * ((((((((locals.var_u0_a_dn3 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn3)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn3 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn3 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn3))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn4 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn4)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn4 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn4 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn4))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn5 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn5)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn5 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn5 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn5))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn6 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn6)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn6 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn6 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn6))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn7 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn7)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn7 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn7 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn7))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn8 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn8)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn8 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn8 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn8))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn9 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn9)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn9 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn9 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn9))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn10 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn10)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn10 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn10 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn10))) / (assign45890_e78132 * assign45890_e78132))))), (((2.0 * ((((((((locals.var_u0_a_dn11 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45890_e78119 * locals.var_nvt_dn11)) * assign45890_e78124) - (assign45890_e78121 * (locals.var_vsat_a_dn11 * locals.var_leff))) / (assign45890_e78124 * assign45890_e78124))) * assign45890_e78133) + (assign45890_e78126 * (-((-((locals.var_ptwg_a_dn11 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn11))) / (assign45890_e78132 * assign45890_e78132))))),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign45890_e78136;
        locals.var_lambdac_dn3 = assign45890_e78136_d_n3;
        locals.var_lambdac_dn4 = assign45890_e78136_d_n4;
        locals.var_lambdac_dn5 = assign45890_e78136_d_n5;
        locals.var_lambdac_dn6 = assign45890_e78136_d_n6;
        locals.var_lambdac_dn7 = assign45890_e78136_d_n7;
        locals.var_lambdac_dn8 = assign45890_e78136_d_n8;
        locals.var_lambdac_dn9 = assign45890_e78136_d_n9;
        locals.var_lambdac_dn10 = assign45890_e78136_d_n10;
        locals.var_lambdac_dn11 = assign45890_e78136_d_n11;
        locals.var_lambdac_rv = 0.0;

        let (assign45900_e78160, assign45900_e78160_d_n3, assign45900_e78160_d_n4, assign45900_e78160_d_n5, assign45900_e78160_d_n6, assign45900_e78160_d_n7, assign45900_e78160_d_n8, assign45900_e78160_d_n9, assign45900_e78160_d_n10, assign45900_e78160_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard714 == 0.0)) {
        let assign45900_e78145: f64 = (locals.var_u0_a / locals.var_t0);
        let assign45900_e78147: f64 = (assign45900_e78145 * locals.var_nvt);
        let assign45900_e78150: f64 = (locals.var_vsat_a * locals.var_leff);
        let assign45900_e78151: f64 = (assign45900_e78147 / assign45900_e78150);
        let assign45900_e78152: f64 = (2.0 * assign45900_e78151);
        let assign45900_e78156: f64 = (locals.var_ptwg_a * locals.var_t2);
        let assign45900_e78157: f64 = (1.0 + assign45900_e78156);
        let assign45900_e78158: f64 = (assign45900_e78152 * assign45900_e78157);
        (assign45900_e78158, (((2.0 * ((((((((locals.var_u0_a_dn3 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn3)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn3 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn3 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn3)))), (((2.0 * ((((((((locals.var_u0_a_dn4 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn4)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn4 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn4 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn4)))), (((2.0 * ((((((((locals.var_u0_a_dn5 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn5)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn5 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn5 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn5)))), (((2.0 * ((((((((locals.var_u0_a_dn6 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn6)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn6 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn6 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn6)))), (((2.0 * ((((((((locals.var_u0_a_dn7 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn7)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn7 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn7 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn7)))), (((2.0 * ((((((((locals.var_u0_a_dn8 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn8)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn8 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn8 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn8)))), (((2.0 * ((((((((locals.var_u0_a_dn9 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn9)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn9 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn9 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn9)))), (((2.0 * ((((((((locals.var_u0_a_dn10 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn10)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn10 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn10 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn10)))), (((2.0 * ((((((((locals.var_u0_a_dn11 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign45900_e78145 * locals.var_nvt_dn11)) * assign45900_e78150) - (assign45900_e78147 * (locals.var_vsat_a_dn11 * locals.var_leff))) / (assign45900_e78150 * assign45900_e78150))) * assign45900_e78157) + (assign45900_e78152 * ((locals.var_ptwg_a_dn11 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn11)))),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign45900_e78160;
        locals.var_lambdac_dn3 = assign45900_e78160_d_n3;
        locals.var_lambdac_dn4 = assign45900_e78160_d_n4;
        locals.var_lambdac_dn5 = assign45900_e78160_d_n5;
        locals.var_lambdac_dn6 = assign45900_e78160_d_n6;
        locals.var_lambdac_dn7 = assign45900_e78160_d_n7;
        locals.var_lambdac_dn8 = assign45900_e78160_d_n8;
        locals.var_lambdac_dn9 = assign45900_e78160_d_n9;
        locals.var_lambdac_dn10 = assign45900_e78160_d_n10;
        locals.var_lambdac_dn11 = assign45900_e78160_d_n11;
        locals.var_lambdac_rv = 0.0;

        let assign45910_e78163: f64 = if locals.var_rdss > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard715 = assign45910_e78163;
        locals.var_guard715_rv = 0.0;

        let (assign45920_e78180, assign45920_e78180_d_n3, assign45920_e78180_d_n4, assign45920_e78180_d_n5, assign45920_e78180_d_n6, assign45920_e78180_d_n7, assign45920_e78180_d_n8, assign45920_e78180_d_n9, assign45920_e78180_d_n10, assign45920_e78180_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign45920_e78170: f64 = (locals.var_weff * 2.0);
        let assign45920_e78172: f64 = (assign45920_e78170 * locals.var_nq);
        let assign45920_e78174: f64 = (assign45920_e78172 * locals.var_cox);
        let assign45920_e78176: f64 = (assign45920_e78174 * locals.var_nvt);
        let assign45920_e78178: f64 = (assign45920_e78176 * locals.var_vsat_a);
        (assign45920_e78178, ((((((assign45920_e78170 * locals.var_nq_dn3) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn3)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn3)), ((((((assign45920_e78170 * locals.var_nq_dn4) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn4)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn4)), ((((((assign45920_e78170 * locals.var_nq_dn5) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn5)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn5)), ((((((assign45920_e78170 * locals.var_nq_dn6) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn6)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn6)), ((((((assign45920_e78170 * locals.var_nq_dn7) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn7)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn7)), ((((((assign45920_e78170 * locals.var_nq_dn8) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn8)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn8)), ((((((assign45920_e78170 * locals.var_nq_dn9) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn9)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn9)), ((((((assign45920_e78170 * locals.var_nq_dn10) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn10)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn10)), ((((((assign45920_e78170 * locals.var_nq_dn11) * locals.var_cox) * locals.var_nvt) + (assign45920_e78174 * locals.var_nvt_dn11)) * locals.var_vsat_a) + (assign45920_e78176 * locals.var_vsat_a_dn11)),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign45920_e78180;
        locals.var_t11_dn3 = assign45920_e78180_d_n3;
        locals.var_t11_dn4 = assign45920_e78180_d_n4;
        locals.var_t11_dn5 = assign45920_e78180_d_n5;
        locals.var_t11_dn6 = assign45920_e78180_d_n6;
        locals.var_t11_dn7 = assign45920_e78180_d_n7;
        locals.var_t11_dn8 = assign45920_e78180_d_n8;
        locals.var_t11_dn9 = assign45920_e78180_d_n9;
        locals.var_t11_dn10 = assign45920_e78180_d_n10;
        locals.var_t11_dn11 = assign45920_e78180_d_n11;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_159(
        locals: &mut StampLocals,
    ) {
        let (assign45930_e78195, assign45930_e78195_d_n3, assign45930_e78195_d_n4, assign45930_e78195_d_n5, assign45930_e78195_d_n6, assign45930_e78195_d_n7, assign45930_e78195_d_n8, assign45930_e78195_d_n9, assign45930_e78195_d_n10, assign45930_e78195_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign45930_e78187: f64 = (locals.var_t11 * locals.var_lambdac);
        let assign45930_e78189: f64 = (assign45930_e78187 * locals.var_rdss);
        let assign45930_e78192: f64 = (2.0 * locals.var_nvt);
        let assign45930_e78193: f64 = (assign45930_e78189 / assign45930_e78192);
        (assign45930_e78193, (((((((locals.var_t11_dn3 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn3)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn3)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn3))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn4 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn4)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn4)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn4))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn5 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn5)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn5)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn5))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn6 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn6)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn6)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn6))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn7 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn7)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn7)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn7))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn8 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn8)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn8)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn8))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn9 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn9)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn9)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn9))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn10 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn10)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn10)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn10))) / (assign45930_e78192 * assign45930_e78192)), (((((((locals.var_t11_dn11 * locals.var_lambdac) + (locals.var_t11 * locals.var_lambdac_dn11)) * locals.var_rdss) + (assign45930_e78187 * locals.var_rdss_dn11)) * assign45930_e78192) - (assign45930_e78189 * (2.0 * locals.var_nvt_dn11))) / (assign45930_e78192 * assign45930_e78192)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign45930_e78195;
        locals.var_t12_dn3 = assign45930_e78195_d_n3;
        locals.var_t12_dn4 = assign45930_e78195_d_n4;
        locals.var_t12_dn5 = assign45930_e78195_d_n5;
        locals.var_t12_dn6 = assign45930_e78195_d_n6;
        locals.var_t12_dn7 = assign45930_e78195_d_n7;
        locals.var_t12_dn8 = assign45930_e78195_d_n8;
        locals.var_t12_dn9 = assign45930_e78195_d_n9;
        locals.var_t12_dn10 = assign45930_e78195_d_n10;
        locals.var_t12_dn11 = assign45930_e78195_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign45940_e78220, assign45940_e78220_d_n3, assign45940_e78220_d_n4, assign45940_e78220_d_n5, assign45940_e78220_d_n6, assign45940_e78220_d_n7, assign45940_e78220_d_n8, assign45940_e78220_d_n9, assign45940_e78220_d_n10, assign45940_e78220_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign45940_e78202: f64 = (0.5 * locals.var_lambdac);
        let assign45940_e78205: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign45940_e78207: f64 = (assign45940_e78205 + locals.var_qs_1);
        let assign45940_e78208: f64 = (assign45940_e78202 * assign45940_e78207);
        let assign45940_e78212: f64 = (0.5 * locals.var_lambdac);
        let assign45940_e78215: f64 = (1.0 + locals.var_qs_1);
        let assign45940_e78216: f64 = (assign45940_e78212 * assign45940_e78215);
        let assign45940_e78217: f64 = (1.0 + assign45940_e78216);
        let assign45940_e78218: f64 = (assign45940_e78208 / assign45940_e78217);
        (assign45940_e78218, ((((((0.5 * locals.var_lambdac_dn3) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn3) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn3)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn4) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn4) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn4)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn5) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn5) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn5)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn6) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn6) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn6)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn7) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn7) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn7)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn8) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn8) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn8)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn9) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn9) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn9)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn10) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn10) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn10)))) / (assign45940_e78217 * assign45940_e78217)), ((((((0.5 * locals.var_lambdac_dn11) * assign45940_e78207) + (assign45940_e78202 * (((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11))) * assign45940_e78217) - (assign45940_e78208 * (((0.5 * locals.var_lambdac_dn11) * assign45940_e78215) + (assign45940_e78212 * locals.var_qs_1_dn11)))) / (assign45940_e78217 * assign45940_e78217)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign45940_e78220;
        locals.var_t0_dn3 = assign45940_e78220_d_n3;
        locals.var_t0_dn4 = assign45940_e78220_d_n4;
        locals.var_t0_dn5 = assign45940_e78220_d_n5;
        locals.var_t0_dn6 = assign45940_e78220_d_n6;
        locals.var_t0_dn7 = assign45940_e78220_d_n7;
        locals.var_t0_dn8 = assign45940_e78220_d_n8;
        locals.var_t0_dn9 = assign45940_e78220_d_n9;
        locals.var_t0_dn10 = assign45940_e78220_d_n10;
        locals.var_t0_dn11 = assign45940_e78220_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign45950_e78233, assign45950_e78233_d_n3, assign45950_e78233_d_n4, assign45950_e78233_d_n5, assign45950_e78233_d_n6, assign45950_e78233_d_n7, assign45950_e78233_d_n8, assign45950_e78233_d_n9, assign45950_e78233_d_n10, assign45950_e78233_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign45950_e78227: f64 = (2.0 * locals.var_lambdac);
        let assign45950_e78230: f64 = (locals.var_qs_1 - locals.var_t0);
        let assign45950_e78231: f64 = (assign45950_e78227 * assign45950_e78230);
        (assign45950_e78231, (((2.0 * locals.var_lambdac_dn3) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn3 - locals.var_t0_dn3))), (((2.0 * locals.var_lambdac_dn4) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn4 - locals.var_t0_dn4))), (((2.0 * locals.var_lambdac_dn5) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn5 - locals.var_t0_dn5))), (((2.0 * locals.var_lambdac_dn6) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn6 - locals.var_t0_dn6))), (((2.0 * locals.var_lambdac_dn7) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn7 - locals.var_t0_dn7))), (((2.0 * locals.var_lambdac_dn8) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn8 - locals.var_t0_dn8))), (((2.0 * locals.var_lambdac_dn9) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn9 - locals.var_t0_dn9))), (((2.0 * locals.var_lambdac_dn10) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn10 - locals.var_t0_dn10))), (((2.0 * locals.var_lambdac_dn11) * assign45950_e78230) + (assign45950_e78227 * (locals.var_qs_1_dn11 - locals.var_t0_dn11))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign45950_e78233;
        locals.var_t1_dn3 = assign45950_e78233_d_n3;
        locals.var_t1_dn4 = assign45950_e78233_d_n4;
        locals.var_t1_dn5 = assign45950_e78233_d_n5;
        locals.var_t1_dn6 = assign45950_e78233_d_n6;
        locals.var_t1_dn7 = assign45950_e78233_d_n7;
        locals.var_t1_dn8 = assign45950_e78233_d_n8;
        locals.var_t1_dn9 = assign45950_e78233_d_n9;
        locals.var_t1_dn10 = assign45950_e78233_d_n10;
        locals.var_t1_dn11 = assign45950_e78233_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign45960_e78245, assign45960_e78245_d_n3, assign45960_e78245_d_n4, assign45960_e78245_d_n5, assign45960_e78245_d_n6, assign45960_e78245_d_n7, assign45960_e78245_d_n8, assign45960_e78245_d_n9, assign45960_e78245_d_n10, assign45960_e78245_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign45960_e78241: f64 = (locals.var_t1 * locals.var_t1);
        let assign45960_e78242: f64 = (1.0 + assign45960_e78241);
        let assign45960_e78243: f64 = (assign45960_e78242).sqrt();
        (assign45960_e78243, (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign45960_e78243)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign45960_e78243)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign45960_e78245;
        locals.var_t2_dn3 = assign45960_e78245_d_n3;
        locals.var_t2_dn4 = assign45960_e78245_d_n4;
        locals.var_t2_dn5 = assign45960_e78245_d_n5;
        locals.var_t2_dn6 = assign45960_e78245_d_n6;
        locals.var_t2_dn7 = assign45960_e78245_d_n7;
        locals.var_t2_dn8 = assign45960_e78245_d_n8;
        locals.var_t2_dn9 = assign45960_e78245_d_n9;
        locals.var_t2_dn10 = assign45960_e78245_d_n10;
        locals.var_t2_dn11 = assign45960_e78245_d_n11;
        locals.var_t2_rv = 0.0;

        let assign45970_e78248: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard716 = assign45970_e78248;
        locals.var_guard716_rv = 0.0;

        let (assign45980_e78258, assign45980_e78258_d_n3, assign45980_e78258_d_n4, assign45980_e78258_d_n5, assign45980_e78258_d_n6, assign45980_e78258_d_n7, assign45980_e78258_d_n8, assign45980_e78258_d_n9, assign45980_e78258_d_n10, assign45980_e78258_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign45980_e78256: f64 = (locals.var_t1).asinh();
        (assign45980_e78256, (locals.var_t1_dn3 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn4 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn5 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn6 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn7 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn8 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn9 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn10 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn11 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()),)
    } else {
        (locals.var_ln_t1_t2, locals.var_ln_t1_t2_dn3, locals.var_ln_t1_t2_dn4, locals.var_ln_t1_t2_dn5, locals.var_ln_t1_t2_dn6, locals.var_ln_t1_t2_dn7, locals.var_ln_t1_t2_dn8, locals.var_ln_t1_t2_dn9, locals.var_ln_t1_t2_dn10, locals.var_ln_t1_t2_dn11,)
    }
};
        locals.var_ln_t1_t2 = assign45980_e78258;
        locals.var_ln_t1_t2_dn3 = assign45980_e78258_d_n3;
        locals.var_ln_t1_t2_dn4 = assign45980_e78258_d_n4;
        locals.var_ln_t1_t2_dn5 = assign45980_e78258_d_n5;
        locals.var_ln_t1_t2_dn6 = assign45980_e78258_d_n6;
        locals.var_ln_t1_t2_dn7 = assign45980_e78258_d_n7;
        locals.var_ln_t1_t2_dn8 = assign45980_e78258_d_n8;
        locals.var_ln_t1_t2_dn9 = assign45980_e78258_d_n9;
        locals.var_ln_t1_t2_dn10 = assign45980_e78258_d_n10;
        locals.var_ln_t1_t2_dn11 = assign45980_e78258_d_n11;
        locals.var_ln_t1_t2_rv = 0.0;

        let (assign45990_e78273, assign45990_e78273_d_n3, assign45990_e78273_d_n4, assign45990_e78273_d_n5, assign45990_e78273_d_n6, assign45990_e78273_d_n7, assign45990_e78273_d_n8, assign45990_e78273_d_n9, assign45990_e78273_d_n10, assign45990_e78273_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign45990_e78268: f64 = (1.0 / locals.var_t1);
        let assign45990_e78270: f64 = (assign45990_e78268 * locals.var_ln_t1_t2);
        let assign45990_e78271: f64 = (locals.var_t2 + assign45990_e78270);
        (assign45990_e78271, (locals.var_t2_dn3 + (((-(locals.var_t1_dn3 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn3))), (locals.var_t2_dn4 + (((-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn4))), (locals.var_t2_dn5 + (((-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn5))), (locals.var_t2_dn6 + (((-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn6))), (locals.var_t2_dn7 + (((-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn7))), (locals.var_t2_dn8 + (((-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn8))), (locals.var_t2_dn9 + (((-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn9))), (locals.var_t2_dn10 + (((-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn10))), (locals.var_t2_dn11 + (((-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign45990_e78268 * locals.var_ln_t1_t2_dn11))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign45990_e78273;
        locals.var_t3_dn3 = assign45990_e78273_d_n3;
        locals.var_t3_dn4 = assign45990_e78273_d_n4;
        locals.var_t3_dn5 = assign45990_e78273_d_n5;
        locals.var_t3_dn6 = assign45990_e78273_d_n6;
        locals.var_t3_dn7 = assign45990_e78273_d_n7;
        locals.var_t3_dn8 = assign45990_e78273_d_n8;
        locals.var_t3_dn9 = assign45990_e78273_d_n9;
        locals.var_t3_dn10 = assign45990_e78273_d_n10;
        locals.var_t3_dn11 = assign45990_e78273_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46000_e78287, assign46000_e78287_d_n3, assign46000_e78287_d_n4, assign46000_e78287_d_n5, assign46000_e78287_d_n6, assign46000_e78287_d_n7, assign46000_e78287_d_n8, assign46000_e78287_d_n9, assign46000_e78287_d_n10, assign46000_e78287_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard716 == 0.0)) {
        let assign46000_e78284: f64 = (1.0 / locals.var_t2);
        let assign46000_e78285: f64 = (locals.var_t2 + assign46000_e78284);
        (assign46000_e78285, (locals.var_t2_dn3 + (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn4 + (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn5 + (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn6 + (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn7 + (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn8 + (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn9 + (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn10 + (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn11 + (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46000_e78287;
        locals.var_t3_dn3 = assign46000_e78287_d_n3;
        locals.var_t3_dn4 = assign46000_e78287_d_n4;
        locals.var_t3_dn5 = assign46000_e78287_d_n5;
        locals.var_t3_dn6 = assign46000_e78287_d_n6;
        locals.var_t3_dn7 = assign46000_e78287_d_n7;
        locals.var_t3_dn8 = assign46000_e78287_d_n8;
        locals.var_t3_dn9 = assign46000_e78287_d_n9;
        locals.var_t3_dn10 = assign46000_e78287_d_n10;
        locals.var_t3_dn11 = assign46000_e78287_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46010_e78320, assign46010_e78320_d_n3, assign46010_e78320_d_n4, assign46010_e78320_d_n5, assign46010_e78320_d_n6, assign46010_e78320_d_n7, assign46010_e78320_d_n8, assign46010_e78320_d_n9, assign46010_e78320_d_n10, assign46010_e78320_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46010_e78294: f64 = (locals.var_t0 * locals.var_t3);
        let assign46010_e78297: f64 = (locals.var_t12 * locals.var_t0);
        let assign46010_e78300: f64 = (locals.var_qs_1 + locals.var_t0);
        let assign46010_e78302: f64 = (assign46010_e78300 + 1.0);
        let assign46010_e78303: f64 = (assign46010_e78297 * assign46010_e78302);
        let assign46010_e78304: f64 = (assign46010_e78294 + assign46010_e78303);
        let assign46010_e78308: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign46010_e78310: f64 = (assign46010_e78308 + locals.var_qs_1);
        let assign46010_e78313: f64 = (locals.var_t0 * locals.var_t0);
        let assign46010_e78315: f64 = (assign46010_e78313 + locals.var_t0);
        let assign46010_e78316: f64 = (assign46010_e78310 - assign46010_e78315);
        let assign46010_e78317: f64 = (locals.var_lambdac * assign46010_e78316);
        let assign46010_e78318: f64 = (assign46010_e78304 - assign46010_e78317);
        (assign46010_e78318, ((((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + ((((locals.var_t12_dn3 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn3)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn3 + locals.var_t0_dn3)))) - ((locals.var_lambdac_dn3 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3) - (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + locals.var_t0_dn3))))), ((((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + ((((locals.var_t12_dn4 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn4)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn4 + locals.var_t0_dn4)))) - ((locals.var_lambdac_dn4 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4) - (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + locals.var_t0_dn4))))), ((((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + ((((locals.var_t12_dn5 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn5)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn5 + locals.var_t0_dn5)))) - ((locals.var_lambdac_dn5 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5) - (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + locals.var_t0_dn5))))), ((((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + ((((locals.var_t12_dn6 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn6)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn6 + locals.var_t0_dn6)))) - ((locals.var_lambdac_dn6 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6) - (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + locals.var_t0_dn6))))), ((((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + ((((locals.var_t12_dn7 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn7)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn7 + locals.var_t0_dn7)))) - ((locals.var_lambdac_dn7 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7) - (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + locals.var_t0_dn7))))), ((((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + ((((locals.var_t12_dn8 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn8)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn8 + locals.var_t0_dn8)))) - ((locals.var_lambdac_dn8 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8) - (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + locals.var_t0_dn8))))), ((((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + ((((locals.var_t12_dn9 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn9)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn9 + locals.var_t0_dn9)))) - ((locals.var_lambdac_dn9 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9) - (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + locals.var_t0_dn9))))), ((((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + ((((locals.var_t12_dn10 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn10)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn10 + locals.var_t0_dn10)))) - ((locals.var_lambdac_dn10 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10) - (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + locals.var_t0_dn10))))), ((((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + ((((locals.var_t12_dn11 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn11)) * assign46010_e78302) + (assign46010_e78297 * (locals.var_qs_1_dn11 + locals.var_t0_dn11)))) - ((locals.var_lambdac_dn11 * assign46010_e78316) + (locals.var_lambdac * ((((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11) - (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + locals.var_t0_dn11))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46010_e78320;
        locals.var_t4_dn3 = assign46010_e78320_d_n3;
        locals.var_t4_dn4 = assign46010_e78320_d_n4;
        locals.var_t4_dn5 = assign46010_e78320_d_n5;
        locals.var_t4_dn6 = assign46010_e78320_d_n6;
        locals.var_t4_dn7 = assign46010_e78320_d_n7;
        locals.var_t4_dn8 = assign46010_e78320_d_n8;
        locals.var_t4_dn9 = assign46010_e78320_d_n9;
        locals.var_t4_dn10 = assign46010_e78320_d_n10;
        locals.var_t4_dn11 = assign46010_e78320_d_n11;
        locals.var_t4_rv = 0.0;

        let assign46020_e78323: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard717 = assign46020_e78323;
        locals.var_guard717_rv = 0.0;

        let (assign46030_e78345, assign46030_e78345_d_n3, assign46030_e78345_d_n4, assign46030_e78345_d_n5, assign46030_e78345_d_n6, assign46030_e78345_d_n7, assign46030_e78345_d_n8, assign46030_e78345_d_n9, assign46030_e78345_d_n10, assign46030_e78345_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign46030_e78331: f64 = (-2.0);
        let assign46030_e78333: f64 = (assign46030_e78331 * locals.var_lambdac);
        let assign46030_e78336: f64 = (locals.var_t1 * locals.var_t2);
        let assign46030_e78338: f64 = (assign46030_e78336 - locals.var_ln_t1_t2);
        let assign46030_e78339: f64 = (assign46030_e78333 * assign46030_e78338);
        let assign46030_e78342: f64 = (locals.var_t1 * locals.var_t1);
        let assign46030_e78343: f64 = (assign46030_e78339 / assign46030_e78342);
        (assign46030_e78343, ((((((assign46030_e78331 * locals.var_lambdac_dn3) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) - locals.var_ln_t1_t2_dn3))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn4) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) - locals.var_ln_t1_t2_dn4))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn5) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) - locals.var_ln_t1_t2_dn5))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn6) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) - locals.var_ln_t1_t2_dn6))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn7) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) - locals.var_ln_t1_t2_dn7))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn8) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) - locals.var_ln_t1_t2_dn8))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn9) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) - locals.var_ln_t1_t2_dn9))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn10) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) - locals.var_ln_t1_t2_dn10))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)))) / (assign46030_e78342 * assign46030_e78342)), ((((((assign46030_e78331 * locals.var_lambdac_dn11) * assign46030_e78338) + (assign46030_e78333 * (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) - locals.var_ln_t1_t2_dn11))) * assign46030_e78342) - (assign46030_e78339 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)))) / (assign46030_e78342 * assign46030_e78342)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46030_e78345;
        locals.var_t5_dn3 = assign46030_e78345_d_n3;
        locals.var_t5_dn4 = assign46030_e78345_d_n4;
        locals.var_t5_dn5 = assign46030_e78345_d_n5;
        locals.var_t5_dn6 = assign46030_e78345_d_n6;
        locals.var_t5_dn7 = assign46030_e78345_d_n7;
        locals.var_t5_dn8 = assign46030_e78345_d_n8;
        locals.var_t5_dn9 = assign46030_e78345_d_n9;
        locals.var_t5_dn10 = assign46030_e78345_d_n10;
        locals.var_t5_dn11 = assign46030_e78345_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46040_e78362, assign46040_e78362_d_n3, assign46040_e78362_d_n4, assign46040_e78362_d_n5, assign46040_e78362_d_n6, assign46040_e78362_d_n7, assign46040_e78362_d_n8, assign46040_e78362_d_n9, assign46040_e78362_d_n10, assign46040_e78362_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard717 == 0.0)) {
        let assign46040_e78354: f64 = (-2.0);
        let assign46040_e78356: f64 = (assign46040_e78354 * locals.var_lambdac);
        let assign46040_e78359: f64 = (locals.var_t1 / locals.var_t2);
        let assign46040_e78360: f64 = (assign46040_e78356 * assign46040_e78359);
        (assign46040_e78360, (((assign46040_e78354 * locals.var_lambdac_dn3) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn4) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn5) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn6) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn7) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn8) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn9) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn10) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)))), (((assign46040_e78354 * locals.var_lambdac_dn11) * assign46040_e78359) + (assign46040_e78356 * (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46040_e78362;
        locals.var_t5_dn3 = assign46040_e78362_d_n3;
        locals.var_t5_dn4 = assign46040_e78362_d_n4;
        locals.var_t5_dn5 = assign46040_e78362_d_n5;
        locals.var_t5_dn6 = assign46040_e78362_d_n6;
        locals.var_t5_dn7 = assign46040_e78362_d_n7;
        locals.var_t5_dn8 = assign46040_e78362_d_n8;
        locals.var_t5_dn9 = assign46040_e78362_d_n9;
        locals.var_t5_dn10 = assign46040_e78362_d_n10;
        locals.var_t5_dn11 = assign46040_e78362_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46050_e78391, assign46050_e78391_d_n3, assign46050_e78391_d_n4, assign46050_e78391_d_n5, assign46050_e78391_d_n6, assign46050_e78391_d_n7, assign46050_e78391_d_n8, assign46050_e78391_d_n9, assign46050_e78391_d_n10, assign46050_e78391_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46050_e78369: f64 = (locals.var_t0 * locals.var_t5);
        let assign46050_e78371: f64 = (assign46050_e78369 + locals.var_t3);
        let assign46050_e78376: f64 = (2.0 * locals.var_t0);
        let assign46050_e78377: f64 = (locals.var_qs_1 + assign46050_e78376);
        let assign46050_e78379: f64 = (assign46050_e78377 + 1.0);
        let assign46050_e78380: f64 = (locals.var_t12 * assign46050_e78379);
        let assign46050_e78381: f64 = (assign46050_e78371 + assign46050_e78380);
        let assign46050_e78385: f64 = (2.0 * locals.var_t0);
        let assign46050_e78387: f64 = (assign46050_e78385 + 1.0);
        let assign46050_e78388: f64 = (locals.var_lambdac * assign46050_e78387);
        let assign46050_e78389: f64 = (assign46050_e78381 + assign46050_e78388);
        (assign46050_e78389, (((((locals.var_t0_dn3 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn3)) + locals.var_t3_dn3) + ((locals.var_t12_dn3 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn3 + (2.0 * locals.var_t0_dn3))))) + ((locals.var_lambdac_dn3 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn3)))), (((((locals.var_t0_dn4 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn4)) + locals.var_t3_dn4) + ((locals.var_t12_dn4 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn4 + (2.0 * locals.var_t0_dn4))))) + ((locals.var_lambdac_dn4 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn4)))), (((((locals.var_t0_dn5 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn5)) + locals.var_t3_dn5) + ((locals.var_t12_dn5 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn5 + (2.0 * locals.var_t0_dn5))))) + ((locals.var_lambdac_dn5 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn5)))), (((((locals.var_t0_dn6 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn6)) + locals.var_t3_dn6) + ((locals.var_t12_dn6 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn6 + (2.0 * locals.var_t0_dn6))))) + ((locals.var_lambdac_dn6 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn6)))), (((((locals.var_t0_dn7 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn7)) + locals.var_t3_dn7) + ((locals.var_t12_dn7 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn7 + (2.0 * locals.var_t0_dn7))))) + ((locals.var_lambdac_dn7 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn7)))), (((((locals.var_t0_dn8 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn8)) + locals.var_t3_dn8) + ((locals.var_t12_dn8 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn8 + (2.0 * locals.var_t0_dn8))))) + ((locals.var_lambdac_dn8 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn8)))), (((((locals.var_t0_dn9 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn9)) + locals.var_t3_dn9) + ((locals.var_t12_dn9 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn9 + (2.0 * locals.var_t0_dn9))))) + ((locals.var_lambdac_dn9 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn9)))), (((((locals.var_t0_dn10 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn10)) + locals.var_t3_dn10) + ((locals.var_t12_dn10 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn10 + (2.0 * locals.var_t0_dn10))))) + ((locals.var_lambdac_dn10 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn10)))), (((((locals.var_t0_dn11 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn11)) + locals.var_t3_dn11) + ((locals.var_t12_dn11 * assign46050_e78379) + (locals.var_t12 * (locals.var_qs_1_dn11 + (2.0 * locals.var_t0_dn11))))) + ((locals.var_lambdac_dn11 * assign46050_e78387) + (locals.var_lambdac * (2.0 * locals.var_t0_dn11)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign46050_e78391;
        locals.var_t6_dn3 = assign46050_e78391_d_n3;
        locals.var_t6_dn4 = assign46050_e78391_d_n4;
        locals.var_t6_dn5 = assign46050_e78391_d_n5;
        locals.var_t6_dn6 = assign46050_e78391_d_n6;
        locals.var_t6_dn7 = assign46050_e78391_d_n7;
        locals.var_t6_dn8 = assign46050_e78391_d_n8;
        locals.var_t6_dn9 = assign46050_e78391_d_n9;
        locals.var_t6_dn10 = assign46050_e78391_d_n10;
        locals.var_t6_dn11 = assign46050_e78391_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign46060_e78402, assign46060_e78402_d_n3, assign46060_e78402_d_n4, assign46060_e78402_d_n5, assign46060_e78402_d_n6, assign46060_e78402_d_n7, assign46060_e78402_d_n8, assign46060_e78402_d_n9, assign46060_e78402_d_n10, assign46060_e78402_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46060_e78399: f64 = (locals.var_t4 / locals.var_t6);
        let assign46060_e78400: f64 = (locals.var_t0 - assign46060_e78399);
        (assign46060_e78400, (locals.var_t0_dn3 - (((locals.var_t4_dn3 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn4 - (((locals.var_t4_dn4 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn5 - (((locals.var_t4_dn5 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn6 - (((locals.var_t4_dn6 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn7 - (((locals.var_t4_dn7 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn8 - (((locals.var_t4_dn8 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn9 - (((locals.var_t4_dn9 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn10 - (((locals.var_t4_dn10 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn11 - (((locals.var_t4_dn11 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign46060_e78402;
        locals.var_t0_dn3 = assign46060_e78402_d_n3;
        locals.var_t0_dn4 = assign46060_e78402_d_n4;
        locals.var_t0_dn5 = assign46060_e78402_d_n5;
        locals.var_t0_dn6 = assign46060_e78402_d_n6;
        locals.var_t0_dn7 = assign46060_e78402_d_n7;
        locals.var_t0_dn8 = assign46060_e78402_d_n8;
        locals.var_t0_dn9 = assign46060_e78402_d_n9;
        locals.var_t0_dn10 = assign46060_e78402_d_n10;
        locals.var_t0_dn11 = assign46060_e78402_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign46070_e78415, assign46070_e78415_d_n3, assign46070_e78415_d_n4, assign46070_e78415_d_n5, assign46070_e78415_d_n6, assign46070_e78415_d_n7, assign46070_e78415_d_n8, assign46070_e78415_d_n9, assign46070_e78415_d_n10, assign46070_e78415_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46070_e78409: f64 = (2.0 * locals.var_lambdac);
        let assign46070_e78412: f64 = (locals.var_qs_1 - locals.var_t0);
        let assign46070_e78413: f64 = (assign46070_e78409 * assign46070_e78412);
        (assign46070_e78413, (((2.0 * locals.var_lambdac_dn3) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn3 - locals.var_t0_dn3))), (((2.0 * locals.var_lambdac_dn4) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn4 - locals.var_t0_dn4))), (((2.0 * locals.var_lambdac_dn5) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn5 - locals.var_t0_dn5))), (((2.0 * locals.var_lambdac_dn6) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn6 - locals.var_t0_dn6))), (((2.0 * locals.var_lambdac_dn7) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn7 - locals.var_t0_dn7))), (((2.0 * locals.var_lambdac_dn8) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn8 - locals.var_t0_dn8))), (((2.0 * locals.var_lambdac_dn9) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn9 - locals.var_t0_dn9))), (((2.0 * locals.var_lambdac_dn10) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn10 - locals.var_t0_dn10))), (((2.0 * locals.var_lambdac_dn11) * assign46070_e78412) + (assign46070_e78409 * (locals.var_qs_1_dn11 - locals.var_t0_dn11))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign46070_e78415;
        locals.var_t1_dn3 = assign46070_e78415_d_n3;
        locals.var_t1_dn4 = assign46070_e78415_d_n4;
        locals.var_t1_dn5 = assign46070_e78415_d_n5;
        locals.var_t1_dn6 = assign46070_e78415_d_n6;
        locals.var_t1_dn7 = assign46070_e78415_d_n7;
        locals.var_t1_dn8 = assign46070_e78415_d_n8;
        locals.var_t1_dn9 = assign46070_e78415_d_n9;
        locals.var_t1_dn10 = assign46070_e78415_d_n10;
        locals.var_t1_dn11 = assign46070_e78415_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign46080_e78427, assign46080_e78427_d_n3, assign46080_e78427_d_n4, assign46080_e78427_d_n5, assign46080_e78427_d_n6, assign46080_e78427_d_n7, assign46080_e78427_d_n8, assign46080_e78427_d_n9, assign46080_e78427_d_n10, assign46080_e78427_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46080_e78423: f64 = (locals.var_t1 * locals.var_t1);
        let assign46080_e78424: f64 = (1.0 + assign46080_e78423);
        let assign46080_e78425: f64 = (assign46080_e78424).sqrt();
        (assign46080_e78425, (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign46080_e78425)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign46080_e78425)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46080_e78427;
        locals.var_t2_dn3 = assign46080_e78427_d_n3;
        locals.var_t2_dn4 = assign46080_e78427_d_n4;
        locals.var_t2_dn5 = assign46080_e78427_d_n5;
        locals.var_t2_dn6 = assign46080_e78427_d_n6;
        locals.var_t2_dn7 = assign46080_e78427_d_n7;
        locals.var_t2_dn8 = assign46080_e78427_d_n8;
        locals.var_t2_dn9 = assign46080_e78427_d_n9;
        locals.var_t2_dn10 = assign46080_e78427_d_n10;
        locals.var_t2_dn11 = assign46080_e78427_d_n11;
        locals.var_t2_rv = 0.0;

        let assign46090_e78430: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard718 = assign46090_e78430;
        locals.var_guard718_rv = 0.0;

        let (assign46100_e78440, assign46100_e78440_d_n3, assign46100_e78440_d_n4, assign46100_e78440_d_n5, assign46100_e78440_d_n6, assign46100_e78440_d_n7, assign46100_e78440_d_n8, assign46100_e78440_d_n9, assign46100_e78440_d_n10, assign46100_e78440_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard718 != 0.0)) {
        let assign46100_e78438: f64 = (locals.var_t1).asinh();
        (assign46100_e78438, (locals.var_t1_dn3 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn4 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn5 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn6 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn7 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn8 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn9 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn10 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn11 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()),)
    } else {
        (locals.var_ln_t1_t2, locals.var_ln_t1_t2_dn3, locals.var_ln_t1_t2_dn4, locals.var_ln_t1_t2_dn5, locals.var_ln_t1_t2_dn6, locals.var_ln_t1_t2_dn7, locals.var_ln_t1_t2_dn8, locals.var_ln_t1_t2_dn9, locals.var_ln_t1_t2_dn10, locals.var_ln_t1_t2_dn11,)
    }
};
        locals.var_ln_t1_t2 = assign46100_e78440;
        locals.var_ln_t1_t2_dn3 = assign46100_e78440_d_n3;
        locals.var_ln_t1_t2_dn4 = assign46100_e78440_d_n4;
        locals.var_ln_t1_t2_dn5 = assign46100_e78440_d_n5;
        locals.var_ln_t1_t2_dn6 = assign46100_e78440_d_n6;
        locals.var_ln_t1_t2_dn7 = assign46100_e78440_d_n7;
        locals.var_ln_t1_t2_dn8 = assign46100_e78440_d_n8;
        locals.var_ln_t1_t2_dn9 = assign46100_e78440_d_n9;
        locals.var_ln_t1_t2_dn10 = assign46100_e78440_d_n10;
        locals.var_ln_t1_t2_dn11 = assign46100_e78440_d_n11;
        locals.var_ln_t1_t2_rv = 0.0;

        let (assign46110_e78455, assign46110_e78455_d_n3, assign46110_e78455_d_n4, assign46110_e78455_d_n5, assign46110_e78455_d_n6, assign46110_e78455_d_n7, assign46110_e78455_d_n8, assign46110_e78455_d_n9, assign46110_e78455_d_n10, assign46110_e78455_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard718 != 0.0)) {
        let assign46110_e78450: f64 = (1.0 / locals.var_t1);
        let assign46110_e78452: f64 = (assign46110_e78450 * locals.var_ln_t1_t2);
        let assign46110_e78453: f64 = (locals.var_t2 + assign46110_e78452);
        (assign46110_e78453, (locals.var_t2_dn3 + (((-(locals.var_t1_dn3 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn3))), (locals.var_t2_dn4 + (((-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn4))), (locals.var_t2_dn5 + (((-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn5))), (locals.var_t2_dn6 + (((-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn6))), (locals.var_t2_dn7 + (((-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn7))), (locals.var_t2_dn8 + (((-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn8))), (locals.var_t2_dn9 + (((-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn9))), (locals.var_t2_dn10 + (((-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn10))), (locals.var_t2_dn11 + (((-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46110_e78450 * locals.var_ln_t1_t2_dn11))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46110_e78455;
        locals.var_t3_dn3 = assign46110_e78455_d_n3;
        locals.var_t3_dn4 = assign46110_e78455_d_n4;
        locals.var_t3_dn5 = assign46110_e78455_d_n5;
        locals.var_t3_dn6 = assign46110_e78455_d_n6;
        locals.var_t3_dn7 = assign46110_e78455_d_n7;
        locals.var_t3_dn8 = assign46110_e78455_d_n8;
        locals.var_t3_dn9 = assign46110_e78455_d_n9;
        locals.var_t3_dn10 = assign46110_e78455_d_n10;
        locals.var_t3_dn11 = assign46110_e78455_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46120_e78469, assign46120_e78469_d_n3, assign46120_e78469_d_n4, assign46120_e78469_d_n5, assign46120_e78469_d_n6, assign46120_e78469_d_n7, assign46120_e78469_d_n8, assign46120_e78469_d_n9, assign46120_e78469_d_n10, assign46120_e78469_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard718 == 0.0)) {
        let assign46120_e78466: f64 = (1.0 / locals.var_t2);
        let assign46120_e78467: f64 = (locals.var_t2 + assign46120_e78466);
        (assign46120_e78467, (locals.var_t2_dn3 + (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn4 + (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn5 + (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn6 + (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn7 + (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn8 + (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn9 + (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn10 + (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn11 + (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46120_e78469;
        locals.var_t3_dn3 = assign46120_e78469_d_n3;
        locals.var_t3_dn4 = assign46120_e78469_d_n4;
        locals.var_t3_dn5 = assign46120_e78469_d_n5;
        locals.var_t3_dn6 = assign46120_e78469_d_n6;
        locals.var_t3_dn7 = assign46120_e78469_d_n7;
        locals.var_t3_dn8 = assign46120_e78469_d_n8;
        locals.var_t3_dn9 = assign46120_e78469_d_n9;
        locals.var_t3_dn10 = assign46120_e78469_d_n10;
        locals.var_t3_dn11 = assign46120_e78469_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46130_e78502, assign46130_e78502_d_n3, assign46130_e78502_d_n4, assign46130_e78502_d_n5, assign46130_e78502_d_n6, assign46130_e78502_d_n7, assign46130_e78502_d_n8, assign46130_e78502_d_n9, assign46130_e78502_d_n10, assign46130_e78502_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46130_e78476: f64 = (locals.var_t0 * locals.var_t3);
        let assign46130_e78479: f64 = (locals.var_t12 * locals.var_t0);
        let assign46130_e78482: f64 = (locals.var_qs_1 + locals.var_t0);
        let assign46130_e78484: f64 = (assign46130_e78482 + 1.0);
        let assign46130_e78485: f64 = (assign46130_e78479 * assign46130_e78484);
        let assign46130_e78486: f64 = (assign46130_e78476 + assign46130_e78485);
        let assign46130_e78490: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign46130_e78492: f64 = (assign46130_e78490 + locals.var_qs_1);
        let assign46130_e78495: f64 = (locals.var_t0 * locals.var_t0);
        let assign46130_e78497: f64 = (assign46130_e78495 + locals.var_t0);
        let assign46130_e78498: f64 = (assign46130_e78492 - assign46130_e78497);
        let assign46130_e78499: f64 = (locals.var_lambdac * assign46130_e78498);
        let assign46130_e78500: f64 = (assign46130_e78486 - assign46130_e78499);
        (assign46130_e78500, ((((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + ((((locals.var_t12_dn3 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn3)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn3 + locals.var_t0_dn3)))) - ((locals.var_lambdac_dn3 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3) - (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + locals.var_t0_dn3))))), ((((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + ((((locals.var_t12_dn4 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn4)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn4 + locals.var_t0_dn4)))) - ((locals.var_lambdac_dn4 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4) - (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + locals.var_t0_dn4))))), ((((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + ((((locals.var_t12_dn5 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn5)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn5 + locals.var_t0_dn5)))) - ((locals.var_lambdac_dn5 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5) - (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + locals.var_t0_dn5))))), ((((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + ((((locals.var_t12_dn6 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn6)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn6 + locals.var_t0_dn6)))) - ((locals.var_lambdac_dn6 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6) - (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + locals.var_t0_dn6))))), ((((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + ((((locals.var_t12_dn7 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn7)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn7 + locals.var_t0_dn7)))) - ((locals.var_lambdac_dn7 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7) - (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + locals.var_t0_dn7))))), ((((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + ((((locals.var_t12_dn8 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn8)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn8 + locals.var_t0_dn8)))) - ((locals.var_lambdac_dn8 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8) - (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + locals.var_t0_dn8))))), ((((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + ((((locals.var_t12_dn9 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn9)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn9 + locals.var_t0_dn9)))) - ((locals.var_lambdac_dn9 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9) - (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + locals.var_t0_dn9))))), ((((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + ((((locals.var_t12_dn10 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn10)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn10 + locals.var_t0_dn10)))) - ((locals.var_lambdac_dn10 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10) - (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + locals.var_t0_dn10))))), ((((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + ((((locals.var_t12_dn11 * locals.var_t0) + (locals.var_t12 * locals.var_t0_dn11)) * assign46130_e78484) + (assign46130_e78479 * (locals.var_qs_1_dn11 + locals.var_t0_dn11)))) - ((locals.var_lambdac_dn11 * assign46130_e78498) + (locals.var_lambdac * ((((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11) - (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + locals.var_t0_dn11))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46130_e78502;
        locals.var_t4_dn3 = assign46130_e78502_d_n3;
        locals.var_t4_dn4 = assign46130_e78502_d_n4;
        locals.var_t4_dn5 = assign46130_e78502_d_n5;
        locals.var_t4_dn6 = assign46130_e78502_d_n6;
        locals.var_t4_dn7 = assign46130_e78502_d_n7;
        locals.var_t4_dn8 = assign46130_e78502_d_n8;
        locals.var_t4_dn9 = assign46130_e78502_d_n9;
        locals.var_t4_dn10 = assign46130_e78502_d_n10;
        locals.var_t4_dn11 = assign46130_e78502_d_n11;
        locals.var_t4_rv = 0.0;

        let assign46140_e78505: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign46140_e78505;
        locals.var_guard719_rv = 0.0;

        let (assign46150_e78527, assign46150_e78527_d_n3, assign46150_e78527_d_n4, assign46150_e78527_d_n5, assign46150_e78527_d_n6, assign46150_e78527_d_n7, assign46150_e78527_d_n8, assign46150_e78527_d_n9, assign46150_e78527_d_n10, assign46150_e78527_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard719 != 0.0)) {
        let assign46150_e78513: f64 = (-2.0);
        let assign46150_e78515: f64 = (assign46150_e78513 * locals.var_lambdac);
        let assign46150_e78518: f64 = (locals.var_t1 * locals.var_t2);
        let assign46150_e78520: f64 = (assign46150_e78518 - locals.var_ln_t1_t2);
        let assign46150_e78521: f64 = (assign46150_e78515 * assign46150_e78520);
        let assign46150_e78524: f64 = (locals.var_t1 * locals.var_t1);
        let assign46150_e78525: f64 = (assign46150_e78521 / assign46150_e78524);
        (assign46150_e78525, ((((((assign46150_e78513 * locals.var_lambdac_dn3) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) - locals.var_ln_t1_t2_dn3))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn4) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) - locals.var_ln_t1_t2_dn4))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn5) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) - locals.var_ln_t1_t2_dn5))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn6) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) - locals.var_ln_t1_t2_dn6))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn7) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) - locals.var_ln_t1_t2_dn7))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn8) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) - locals.var_ln_t1_t2_dn8))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn9) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) - locals.var_ln_t1_t2_dn9))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn10) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) - locals.var_ln_t1_t2_dn10))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)))) / (assign46150_e78524 * assign46150_e78524)), ((((((assign46150_e78513 * locals.var_lambdac_dn11) * assign46150_e78520) + (assign46150_e78515 * (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) - locals.var_ln_t1_t2_dn11))) * assign46150_e78524) - (assign46150_e78521 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)))) / (assign46150_e78524 * assign46150_e78524)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46150_e78527;
        locals.var_t5_dn3 = assign46150_e78527_d_n3;
        locals.var_t5_dn4 = assign46150_e78527_d_n4;
        locals.var_t5_dn5 = assign46150_e78527_d_n5;
        locals.var_t5_dn6 = assign46150_e78527_d_n6;
        locals.var_t5_dn7 = assign46150_e78527_d_n7;
        locals.var_t5_dn8 = assign46150_e78527_d_n8;
        locals.var_t5_dn9 = assign46150_e78527_d_n9;
        locals.var_t5_dn10 = assign46150_e78527_d_n10;
        locals.var_t5_dn11 = assign46150_e78527_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46160_e78544, assign46160_e78544_d_n3, assign46160_e78544_d_n4, assign46160_e78544_d_n5, assign46160_e78544_d_n6, assign46160_e78544_d_n7, assign46160_e78544_d_n8, assign46160_e78544_d_n9, assign46160_e78544_d_n10, assign46160_e78544_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) && (locals.var_guard719 == 0.0)) {
        let assign46160_e78536: f64 = (-2.0);
        let assign46160_e78538: f64 = (assign46160_e78536 * locals.var_lambdac);
        let assign46160_e78541: f64 = (locals.var_t1 / locals.var_t2);
        let assign46160_e78542: f64 = (assign46160_e78538 * assign46160_e78541);
        (assign46160_e78542, (((assign46160_e78536 * locals.var_lambdac_dn3) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn4) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn5) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn6) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn7) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn8) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn9) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn10) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)))), (((assign46160_e78536 * locals.var_lambdac_dn11) * assign46160_e78541) + (assign46160_e78538 * (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46160_e78544;
        locals.var_t5_dn3 = assign46160_e78544_d_n3;
        locals.var_t5_dn4 = assign46160_e78544_d_n4;
        locals.var_t5_dn5 = assign46160_e78544_d_n5;
        locals.var_t5_dn6 = assign46160_e78544_d_n6;
        locals.var_t5_dn7 = assign46160_e78544_d_n7;
        locals.var_t5_dn8 = assign46160_e78544_d_n8;
        locals.var_t5_dn9 = assign46160_e78544_d_n9;
        locals.var_t5_dn10 = assign46160_e78544_d_n10;
        locals.var_t5_dn11 = assign46160_e78544_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_160(
        locals: &mut StampLocals,
    ) {
        let (assign46170_e78573, assign46170_e78573_d_n3, assign46170_e78573_d_n4, assign46170_e78573_d_n5, assign46170_e78573_d_n6, assign46170_e78573_d_n7, assign46170_e78573_d_n8, assign46170_e78573_d_n9, assign46170_e78573_d_n10, assign46170_e78573_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46170_e78551: f64 = (locals.var_t0 * locals.var_t5);
        let assign46170_e78553: f64 = (assign46170_e78551 + locals.var_t3);
        let assign46170_e78558: f64 = (2.0 * locals.var_t0);
        let assign46170_e78559: f64 = (locals.var_qs_1 + assign46170_e78558);
        let assign46170_e78561: f64 = (assign46170_e78559 + 1.0);
        let assign46170_e78562: f64 = (locals.var_t12 * assign46170_e78561);
        let assign46170_e78563: f64 = (assign46170_e78553 + assign46170_e78562);
        let assign46170_e78567: f64 = (2.0 * locals.var_t0);
        let assign46170_e78569: f64 = (assign46170_e78567 + 1.0);
        let assign46170_e78570: f64 = (locals.var_lambdac * assign46170_e78569);
        let assign46170_e78571: f64 = (assign46170_e78563 + assign46170_e78570);
        (assign46170_e78571, (((((locals.var_t0_dn3 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn3)) + locals.var_t3_dn3) + ((locals.var_t12_dn3 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn3 + (2.0 * locals.var_t0_dn3))))) + ((locals.var_lambdac_dn3 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn3)))), (((((locals.var_t0_dn4 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn4)) + locals.var_t3_dn4) + ((locals.var_t12_dn4 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn4 + (2.0 * locals.var_t0_dn4))))) + ((locals.var_lambdac_dn4 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn4)))), (((((locals.var_t0_dn5 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn5)) + locals.var_t3_dn5) + ((locals.var_t12_dn5 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn5 + (2.0 * locals.var_t0_dn5))))) + ((locals.var_lambdac_dn5 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn5)))), (((((locals.var_t0_dn6 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn6)) + locals.var_t3_dn6) + ((locals.var_t12_dn6 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn6 + (2.0 * locals.var_t0_dn6))))) + ((locals.var_lambdac_dn6 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn6)))), (((((locals.var_t0_dn7 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn7)) + locals.var_t3_dn7) + ((locals.var_t12_dn7 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn7 + (2.0 * locals.var_t0_dn7))))) + ((locals.var_lambdac_dn7 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn7)))), (((((locals.var_t0_dn8 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn8)) + locals.var_t3_dn8) + ((locals.var_t12_dn8 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn8 + (2.0 * locals.var_t0_dn8))))) + ((locals.var_lambdac_dn8 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn8)))), (((((locals.var_t0_dn9 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn9)) + locals.var_t3_dn9) + ((locals.var_t12_dn9 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn9 + (2.0 * locals.var_t0_dn9))))) + ((locals.var_lambdac_dn9 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn9)))), (((((locals.var_t0_dn10 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn10)) + locals.var_t3_dn10) + ((locals.var_t12_dn10 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn10 + (2.0 * locals.var_t0_dn10))))) + ((locals.var_lambdac_dn10 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn10)))), (((((locals.var_t0_dn11 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn11)) + locals.var_t3_dn11) + ((locals.var_t12_dn11 * assign46170_e78561) + (locals.var_t12 * (locals.var_qs_1_dn11 + (2.0 * locals.var_t0_dn11))))) + ((locals.var_lambdac_dn11 * assign46170_e78569) + (locals.var_lambdac * (2.0 * locals.var_t0_dn11)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign46170_e78573;
        locals.var_t6_dn3 = assign46170_e78573_d_n3;
        locals.var_t6_dn4 = assign46170_e78573_d_n4;
        locals.var_t6_dn5 = assign46170_e78573_d_n5;
        locals.var_t6_dn6 = assign46170_e78573_d_n6;
        locals.var_t6_dn7 = assign46170_e78573_d_n7;
        locals.var_t6_dn8 = assign46170_e78573_d_n8;
        locals.var_t6_dn9 = assign46170_e78573_d_n9;
        locals.var_t6_dn10 = assign46170_e78573_d_n10;
        locals.var_t6_dn11 = assign46170_e78573_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign46180_e78584, assign46180_e78584_d_n3, assign46180_e78584_d_n4, assign46180_e78584_d_n5, assign46180_e78584_d_n6, assign46180_e78584_d_n7, assign46180_e78584_d_n8, assign46180_e78584_d_n9, assign46180_e78584_d_n10, assign46180_e78584_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 != 0.0)) {
        let assign46180_e78581: f64 = (locals.var_t4 / locals.var_t6);
        let assign46180_e78582: f64 = (locals.var_t0 - assign46180_e78581);
        (assign46180_e78582, (locals.var_t0_dn3 - (((locals.var_t4_dn3 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn4 - (((locals.var_t4_dn4 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn5 - (((locals.var_t4_dn5 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn6 - (((locals.var_t4_dn6 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn7 - (((locals.var_t4_dn7 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn8 - (((locals.var_t4_dn8 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn9 - (((locals.var_t4_dn9 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn10 - (((locals.var_t4_dn10 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn11 - (((locals.var_t4_dn11 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6))),)
    } else {
        (locals.var_qdsat, locals.var_qdsat_dn3, locals.var_qdsat_dn4, locals.var_qdsat_dn5, locals.var_qdsat_dn6, locals.var_qdsat_dn7, locals.var_qdsat_dn8, locals.var_qdsat_dn9, locals.var_qdsat_dn10, locals.var_qdsat_dn11,)
    }
};
        locals.var_qdsat = assign46180_e78584;
        locals.var_qdsat_dn3 = assign46180_e78584_d_n3;
        locals.var_qdsat_dn4 = assign46180_e78584_d_n4;
        locals.var_qdsat_dn5 = assign46180_e78584_d_n5;
        locals.var_qdsat_dn6 = assign46180_e78584_d_n6;
        locals.var_qdsat_dn7 = assign46180_e78584_d_n7;
        locals.var_qdsat_dn8 = assign46180_e78584_d_n8;
        locals.var_qdsat_dn9 = assign46180_e78584_d_n9;
        locals.var_qdsat_dn10 = assign46180_e78584_d_n10;
        locals.var_qdsat_dn11 = assign46180_e78584_d_n11;
        locals.var_qdsat_rv = 0.0;

        let (assign46190_e78610, assign46190_e78610_d_n3, assign46190_e78610_d_n4, assign46190_e78610_d_n5, assign46190_e78610_d_n6, assign46190_e78610_d_n7, assign46190_e78610_d_n8, assign46190_e78610_d_n9, assign46190_e78610_d_n10, assign46190_e78610_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46190_e78592: f64 = (0.5 * locals.var_lambdac);
        let assign46190_e78595: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign46190_e78597: f64 = (assign46190_e78595 + locals.var_qs_1);
        let assign46190_e78598: f64 = (assign46190_e78592 * assign46190_e78597);
        let assign46190_e78602: f64 = (0.5 * locals.var_lambdac);
        let assign46190_e78605: f64 = (1.0 + locals.var_qs_1);
        let assign46190_e78606: f64 = (assign46190_e78602 * assign46190_e78605);
        let assign46190_e78607: f64 = (1.0 + assign46190_e78606);
        let assign46190_e78608: f64 = (assign46190_e78598 / assign46190_e78607);
        (assign46190_e78608, ((((((0.5 * locals.var_lambdac_dn3) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn3) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn3)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn4) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn4) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn4)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn5) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn5) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn5)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn6) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn6) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn6)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn7) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn7) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn7)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn8) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn8) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn8)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn9) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn9) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn9)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn10) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn10) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn10)))) / (assign46190_e78607 * assign46190_e78607)), ((((((0.5 * locals.var_lambdac_dn11) * assign46190_e78597) + (assign46190_e78592 * (((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11))) * assign46190_e78607) - (assign46190_e78598 * (((0.5 * locals.var_lambdac_dn11) * assign46190_e78605) + (assign46190_e78602 * locals.var_qs_1_dn11)))) / (assign46190_e78607 * assign46190_e78607)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign46190_e78610;
        locals.var_t0_dn3 = assign46190_e78610_d_n3;
        locals.var_t0_dn4 = assign46190_e78610_d_n4;
        locals.var_t0_dn5 = assign46190_e78610_d_n5;
        locals.var_t0_dn6 = assign46190_e78610_d_n6;
        locals.var_t0_dn7 = assign46190_e78610_d_n7;
        locals.var_t0_dn8 = assign46190_e78610_d_n8;
        locals.var_t0_dn9 = assign46190_e78610_d_n9;
        locals.var_t0_dn10 = assign46190_e78610_d_n10;
        locals.var_t0_dn11 = assign46190_e78610_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign46200_e78624, assign46200_e78624_d_n3, assign46200_e78624_d_n4, assign46200_e78624_d_n5, assign46200_e78624_d_n6, assign46200_e78624_d_n7, assign46200_e78624_d_n8, assign46200_e78624_d_n9, assign46200_e78624_d_n10, assign46200_e78624_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46200_e78618: f64 = (2.0 * locals.var_lambdac);
        let assign46200_e78621: f64 = (locals.var_qs_1 - locals.var_t0);
        let assign46200_e78622: f64 = (assign46200_e78618 * assign46200_e78621);
        (assign46200_e78622, (((2.0 * locals.var_lambdac_dn3) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn3 - locals.var_t0_dn3))), (((2.0 * locals.var_lambdac_dn4) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn4 - locals.var_t0_dn4))), (((2.0 * locals.var_lambdac_dn5) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn5 - locals.var_t0_dn5))), (((2.0 * locals.var_lambdac_dn6) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn6 - locals.var_t0_dn6))), (((2.0 * locals.var_lambdac_dn7) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn7 - locals.var_t0_dn7))), (((2.0 * locals.var_lambdac_dn8) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn8 - locals.var_t0_dn8))), (((2.0 * locals.var_lambdac_dn9) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn9 - locals.var_t0_dn9))), (((2.0 * locals.var_lambdac_dn10) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn10 - locals.var_t0_dn10))), (((2.0 * locals.var_lambdac_dn11) * assign46200_e78621) + (assign46200_e78618 * (locals.var_qs_1_dn11 - locals.var_t0_dn11))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign46200_e78624;
        locals.var_t1_dn3 = assign46200_e78624_d_n3;
        locals.var_t1_dn4 = assign46200_e78624_d_n4;
        locals.var_t1_dn5 = assign46200_e78624_d_n5;
        locals.var_t1_dn6 = assign46200_e78624_d_n6;
        locals.var_t1_dn7 = assign46200_e78624_d_n7;
        locals.var_t1_dn8 = assign46200_e78624_d_n8;
        locals.var_t1_dn9 = assign46200_e78624_d_n9;
        locals.var_t1_dn10 = assign46200_e78624_d_n10;
        locals.var_t1_dn11 = assign46200_e78624_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign46210_e78637, assign46210_e78637_d_n3, assign46210_e78637_d_n4, assign46210_e78637_d_n5, assign46210_e78637_d_n6, assign46210_e78637_d_n7, assign46210_e78637_d_n8, assign46210_e78637_d_n9, assign46210_e78637_d_n10, assign46210_e78637_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46210_e78633: f64 = (locals.var_t1 * locals.var_t1);
        let assign46210_e78634: f64 = (1.0 + assign46210_e78633);
        let assign46210_e78635: f64 = (assign46210_e78634).sqrt();
        (assign46210_e78635, (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign46210_e78635)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign46210_e78635)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46210_e78637;
        locals.var_t2_dn3 = assign46210_e78637_d_n3;
        locals.var_t2_dn4 = assign46210_e78637_d_n4;
        locals.var_t2_dn5 = assign46210_e78637_d_n5;
        locals.var_t2_dn6 = assign46210_e78637_d_n6;
        locals.var_t2_dn7 = assign46210_e78637_d_n7;
        locals.var_t2_dn8 = assign46210_e78637_d_n8;
        locals.var_t2_dn9 = assign46210_e78637_d_n9;
        locals.var_t2_dn10 = assign46210_e78637_d_n10;
        locals.var_t2_dn11 = assign46210_e78637_d_n11;
        locals.var_t2_rv = 0.0;

        let assign46220_e78640: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard720 = assign46220_e78640;
        locals.var_guard720_rv = 0.0;

        let (assign46230_e78651, assign46230_e78651_d_n3, assign46230_e78651_d_n4, assign46230_e78651_d_n5, assign46230_e78651_d_n6, assign46230_e78651_d_n7, assign46230_e78651_d_n8, assign46230_e78651_d_n9, assign46230_e78651_d_n10, assign46230_e78651_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard720 != 0.0)) {
        let assign46230_e78649: f64 = (locals.var_t1).asinh();
        (assign46230_e78649, (locals.var_t1_dn3 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn4 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn5 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn6 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn7 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn8 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn9 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn10 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn11 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()),)
    } else {
        (locals.var_ln_t1_t2, locals.var_ln_t1_t2_dn3, locals.var_ln_t1_t2_dn4, locals.var_ln_t1_t2_dn5, locals.var_ln_t1_t2_dn6, locals.var_ln_t1_t2_dn7, locals.var_ln_t1_t2_dn8, locals.var_ln_t1_t2_dn9, locals.var_ln_t1_t2_dn10, locals.var_ln_t1_t2_dn11,)
    }
};
        locals.var_ln_t1_t2 = assign46230_e78651;
        locals.var_ln_t1_t2_dn3 = assign46230_e78651_d_n3;
        locals.var_ln_t1_t2_dn4 = assign46230_e78651_d_n4;
        locals.var_ln_t1_t2_dn5 = assign46230_e78651_d_n5;
        locals.var_ln_t1_t2_dn6 = assign46230_e78651_d_n6;
        locals.var_ln_t1_t2_dn7 = assign46230_e78651_d_n7;
        locals.var_ln_t1_t2_dn8 = assign46230_e78651_d_n8;
        locals.var_ln_t1_t2_dn9 = assign46230_e78651_d_n9;
        locals.var_ln_t1_t2_dn10 = assign46230_e78651_d_n10;
        locals.var_ln_t1_t2_dn11 = assign46230_e78651_d_n11;
        locals.var_ln_t1_t2_rv = 0.0;

        let (assign46240_e78667, assign46240_e78667_d_n3, assign46240_e78667_d_n4, assign46240_e78667_d_n5, assign46240_e78667_d_n6, assign46240_e78667_d_n7, assign46240_e78667_d_n8, assign46240_e78667_d_n9, assign46240_e78667_d_n10, assign46240_e78667_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard720 != 0.0)) {
        let assign46240_e78662: f64 = (1.0 / locals.var_t1);
        let assign46240_e78664: f64 = (assign46240_e78662 * locals.var_ln_t1_t2);
        let assign46240_e78665: f64 = (locals.var_t2 + assign46240_e78664);
        (assign46240_e78665, (locals.var_t2_dn3 + (((-(locals.var_t1_dn3 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn3))), (locals.var_t2_dn4 + (((-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn4))), (locals.var_t2_dn5 + (((-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn5))), (locals.var_t2_dn6 + (((-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn6))), (locals.var_t2_dn7 + (((-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn7))), (locals.var_t2_dn8 + (((-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn8))), (locals.var_t2_dn9 + (((-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn9))), (locals.var_t2_dn10 + (((-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn10))), (locals.var_t2_dn11 + (((-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46240_e78662 * locals.var_ln_t1_t2_dn11))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46240_e78667;
        locals.var_t3_dn3 = assign46240_e78667_d_n3;
        locals.var_t3_dn4 = assign46240_e78667_d_n4;
        locals.var_t3_dn5 = assign46240_e78667_d_n5;
        locals.var_t3_dn6 = assign46240_e78667_d_n6;
        locals.var_t3_dn7 = assign46240_e78667_d_n7;
        locals.var_t3_dn8 = assign46240_e78667_d_n8;
        locals.var_t3_dn9 = assign46240_e78667_d_n9;
        locals.var_t3_dn10 = assign46240_e78667_d_n10;
        locals.var_t3_dn11 = assign46240_e78667_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46250_e78682, assign46250_e78682_d_n3, assign46250_e78682_d_n4, assign46250_e78682_d_n5, assign46250_e78682_d_n6, assign46250_e78682_d_n7, assign46250_e78682_d_n8, assign46250_e78682_d_n9, assign46250_e78682_d_n10, assign46250_e78682_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard720 == 0.0)) {
        let assign46250_e78679: f64 = (1.0 / locals.var_t2);
        let assign46250_e78680: f64 = (locals.var_t2 + assign46250_e78679);
        (assign46250_e78680, (locals.var_t2_dn3 + (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn4 + (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn5 + (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn6 + (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn7 + (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn8 + (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn9 + (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn10 + (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn11 + (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46250_e78682;
        locals.var_t3_dn3 = assign46250_e78682_d_n3;
        locals.var_t3_dn4 = assign46250_e78682_d_n4;
        locals.var_t3_dn5 = assign46250_e78682_d_n5;
        locals.var_t3_dn6 = assign46250_e78682_d_n6;
        locals.var_t3_dn7 = assign46250_e78682_d_n7;
        locals.var_t3_dn8 = assign46250_e78682_d_n8;
        locals.var_t3_dn9 = assign46250_e78682_d_n9;
        locals.var_t3_dn10 = assign46250_e78682_d_n10;
        locals.var_t3_dn11 = assign46250_e78682_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46260_e78706, assign46260_e78706_d_n3, assign46260_e78706_d_n4, assign46260_e78706_d_n5, assign46260_e78706_d_n6, assign46260_e78706_d_n7, assign46260_e78706_d_n8, assign46260_e78706_d_n9, assign46260_e78706_d_n10, assign46260_e78706_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46260_e78690: f64 = (locals.var_t0 * locals.var_t3);
        let assign46260_e78694: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign46260_e78696: f64 = (assign46260_e78694 + locals.var_qs_1);
        let assign46260_e78699: f64 = (locals.var_t0 * locals.var_t0);
        let assign46260_e78701: f64 = (assign46260_e78699 + locals.var_t0);
        let assign46260_e78702: f64 = (assign46260_e78696 - assign46260_e78701);
        let assign46260_e78703: f64 = (locals.var_lambdac * assign46260_e78702);
        let assign46260_e78704: f64 = (assign46260_e78690 - assign46260_e78703);
        (assign46260_e78704, (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) - ((locals.var_lambdac_dn3 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3) - (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + locals.var_t0_dn3))))), (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) - ((locals.var_lambdac_dn4 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4) - (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + locals.var_t0_dn4))))), (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) - ((locals.var_lambdac_dn5 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5) - (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + locals.var_t0_dn5))))), (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) - ((locals.var_lambdac_dn6 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6) - (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + locals.var_t0_dn6))))), (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) - ((locals.var_lambdac_dn7 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7) - (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + locals.var_t0_dn7))))), (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) - ((locals.var_lambdac_dn8 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8) - (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + locals.var_t0_dn8))))), (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) - ((locals.var_lambdac_dn9 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9) - (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + locals.var_t0_dn9))))), (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) - ((locals.var_lambdac_dn10 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10) - (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + locals.var_t0_dn10))))), (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) - ((locals.var_lambdac_dn11 * assign46260_e78702) + (locals.var_lambdac * ((((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11) - (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + locals.var_t0_dn11))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46260_e78706;
        locals.var_t4_dn3 = assign46260_e78706_d_n3;
        locals.var_t4_dn4 = assign46260_e78706_d_n4;
        locals.var_t4_dn5 = assign46260_e78706_d_n5;
        locals.var_t4_dn6 = assign46260_e78706_d_n6;
        locals.var_t4_dn7 = assign46260_e78706_d_n7;
        locals.var_t4_dn8 = assign46260_e78706_d_n8;
        locals.var_t4_dn9 = assign46260_e78706_d_n9;
        locals.var_t4_dn10 = assign46260_e78706_d_n10;
        locals.var_t4_dn11 = assign46260_e78706_d_n11;
        locals.var_t4_rv = 0.0;

        let assign46270_e78709: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard721 = assign46270_e78709;
        locals.var_guard721_rv = 0.0;

        let (assign46280_e78732, assign46280_e78732_d_n3, assign46280_e78732_d_n4, assign46280_e78732_d_n5, assign46280_e78732_d_n6, assign46280_e78732_d_n7, assign46280_e78732_d_n8, assign46280_e78732_d_n9, assign46280_e78732_d_n10, assign46280_e78732_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard721 != 0.0)) {
        let assign46280_e78718: f64 = (-2.0);
        let assign46280_e78720: f64 = (assign46280_e78718 * locals.var_lambdac);
        let assign46280_e78723: f64 = (locals.var_t1 * locals.var_t2);
        let assign46280_e78725: f64 = (assign46280_e78723 - locals.var_ln_t1_t2);
        let assign46280_e78726: f64 = (assign46280_e78720 * assign46280_e78725);
        let assign46280_e78729: f64 = (locals.var_t1 * locals.var_t1);
        let assign46280_e78730: f64 = (assign46280_e78726 / assign46280_e78729);
        (assign46280_e78730, ((((((assign46280_e78718 * locals.var_lambdac_dn3) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) - locals.var_ln_t1_t2_dn3))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn4) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) - locals.var_ln_t1_t2_dn4))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn5) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) - locals.var_ln_t1_t2_dn5))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn6) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) - locals.var_ln_t1_t2_dn6))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn7) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) - locals.var_ln_t1_t2_dn7))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn8) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) - locals.var_ln_t1_t2_dn8))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn9) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) - locals.var_ln_t1_t2_dn9))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn10) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) - locals.var_ln_t1_t2_dn10))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)))) / (assign46280_e78729 * assign46280_e78729)), ((((((assign46280_e78718 * locals.var_lambdac_dn11) * assign46280_e78725) + (assign46280_e78720 * (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) - locals.var_ln_t1_t2_dn11))) * assign46280_e78729) - (assign46280_e78726 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)))) / (assign46280_e78729 * assign46280_e78729)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46280_e78732;
        locals.var_t5_dn3 = assign46280_e78732_d_n3;
        locals.var_t5_dn4 = assign46280_e78732_d_n4;
        locals.var_t5_dn5 = assign46280_e78732_d_n5;
        locals.var_t5_dn6 = assign46280_e78732_d_n6;
        locals.var_t5_dn7 = assign46280_e78732_d_n7;
        locals.var_t5_dn8 = assign46280_e78732_d_n8;
        locals.var_t5_dn9 = assign46280_e78732_d_n9;
        locals.var_t5_dn10 = assign46280_e78732_d_n10;
        locals.var_t5_dn11 = assign46280_e78732_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46290_e78750, assign46290_e78750_d_n3, assign46290_e78750_d_n4, assign46290_e78750_d_n5, assign46290_e78750_d_n6, assign46290_e78750_d_n7, assign46290_e78750_d_n8, assign46290_e78750_d_n9, assign46290_e78750_d_n10, assign46290_e78750_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard721 == 0.0)) {
        let assign46290_e78742: f64 = (-2.0);
        let assign46290_e78744: f64 = (assign46290_e78742 * locals.var_lambdac);
        let assign46290_e78747: f64 = (locals.var_t1 / locals.var_t2);
        let assign46290_e78748: f64 = (assign46290_e78744 * assign46290_e78747);
        (assign46290_e78748, (((assign46290_e78742 * locals.var_lambdac_dn3) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn4) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn5) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn6) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn7) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn8) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn9) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn10) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)))), (((assign46290_e78742 * locals.var_lambdac_dn11) * assign46290_e78747) + (assign46290_e78744 * (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46290_e78750;
        locals.var_t5_dn3 = assign46290_e78750_d_n3;
        locals.var_t5_dn4 = assign46290_e78750_d_n4;
        locals.var_t5_dn5 = assign46290_e78750_d_n5;
        locals.var_t5_dn6 = assign46290_e78750_d_n6;
        locals.var_t5_dn7 = assign46290_e78750_d_n7;
        locals.var_t5_dn8 = assign46290_e78750_d_n8;
        locals.var_t5_dn9 = assign46290_e78750_d_n9;
        locals.var_t5_dn10 = assign46290_e78750_d_n10;
        locals.var_t5_dn11 = assign46290_e78750_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46300_e78770, assign46300_e78770_d_n3, assign46300_e78770_d_n4, assign46300_e78770_d_n5, assign46300_e78770_d_n6, assign46300_e78770_d_n7, assign46300_e78770_d_n8, assign46300_e78770_d_n9, assign46300_e78770_d_n10, assign46300_e78770_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46300_e78758: f64 = (locals.var_t0 * locals.var_t5);
        let assign46300_e78760: f64 = (assign46300_e78758 + locals.var_t3);
        let assign46300_e78764: f64 = (2.0 * locals.var_t0);
        let assign46300_e78766: f64 = (assign46300_e78764 + 1.0);
        let assign46300_e78767: f64 = (locals.var_lambdac * assign46300_e78766);
        let assign46300_e78768: f64 = (assign46300_e78760 + assign46300_e78767);
        (assign46300_e78768, ((((locals.var_t0_dn3 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn3)) + locals.var_t3_dn3) + ((locals.var_lambdac_dn3 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn3)))), ((((locals.var_t0_dn4 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn4)) + locals.var_t3_dn4) + ((locals.var_lambdac_dn4 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn4)))), ((((locals.var_t0_dn5 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn5)) + locals.var_t3_dn5) + ((locals.var_lambdac_dn5 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn5)))), ((((locals.var_t0_dn6 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn6)) + locals.var_t3_dn6) + ((locals.var_lambdac_dn6 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn6)))), ((((locals.var_t0_dn7 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn7)) + locals.var_t3_dn7) + ((locals.var_lambdac_dn7 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn7)))), ((((locals.var_t0_dn8 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn8)) + locals.var_t3_dn8) + ((locals.var_lambdac_dn8 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn8)))), ((((locals.var_t0_dn9 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn9)) + locals.var_t3_dn9) + ((locals.var_lambdac_dn9 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn9)))), ((((locals.var_t0_dn10 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn10)) + locals.var_t3_dn10) + ((locals.var_lambdac_dn10 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn10)))), ((((locals.var_t0_dn11 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn11)) + locals.var_t3_dn11) + ((locals.var_lambdac_dn11 * assign46300_e78766) + (locals.var_lambdac * (2.0 * locals.var_t0_dn11)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign46300_e78770;
        locals.var_t6_dn3 = assign46300_e78770_d_n3;
        locals.var_t6_dn4 = assign46300_e78770_d_n4;
        locals.var_t6_dn5 = assign46300_e78770_d_n5;
        locals.var_t6_dn6 = assign46300_e78770_d_n6;
        locals.var_t6_dn7 = assign46300_e78770_d_n7;
        locals.var_t6_dn8 = assign46300_e78770_d_n8;
        locals.var_t6_dn9 = assign46300_e78770_d_n9;
        locals.var_t6_dn10 = assign46300_e78770_d_n10;
        locals.var_t6_dn11 = assign46300_e78770_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign46310_e78782, assign46310_e78782_d_n3, assign46310_e78782_d_n4, assign46310_e78782_d_n5, assign46310_e78782_d_n6, assign46310_e78782_d_n7, assign46310_e78782_d_n8, assign46310_e78782_d_n9, assign46310_e78782_d_n10, assign46310_e78782_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46310_e78779: f64 = (locals.var_t4 / locals.var_t6);
        let assign46310_e78780: f64 = (locals.var_t0 - assign46310_e78779);
        (assign46310_e78780, (locals.var_t0_dn3 - (((locals.var_t4_dn3 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn4 - (((locals.var_t4_dn4 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn5 - (((locals.var_t4_dn5 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn6 - (((locals.var_t4_dn6 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn7 - (((locals.var_t4_dn7 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn8 - (((locals.var_t4_dn8 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn9 - (((locals.var_t4_dn9 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn10 - (((locals.var_t4_dn10 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn11 - (((locals.var_t4_dn11 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign46310_e78782;
        locals.var_t0_dn3 = assign46310_e78782_d_n3;
        locals.var_t0_dn4 = assign46310_e78782_d_n4;
        locals.var_t0_dn5 = assign46310_e78782_d_n5;
        locals.var_t0_dn6 = assign46310_e78782_d_n6;
        locals.var_t0_dn7 = assign46310_e78782_d_n7;
        locals.var_t0_dn8 = assign46310_e78782_d_n8;
        locals.var_t0_dn9 = assign46310_e78782_d_n9;
        locals.var_t0_dn10 = assign46310_e78782_d_n10;
        locals.var_t0_dn11 = assign46310_e78782_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign46320_e78796, assign46320_e78796_d_n3, assign46320_e78796_d_n4, assign46320_e78796_d_n5, assign46320_e78796_d_n6, assign46320_e78796_d_n7, assign46320_e78796_d_n8, assign46320_e78796_d_n9, assign46320_e78796_d_n10, assign46320_e78796_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46320_e78790: f64 = (2.0 * locals.var_lambdac);
        let assign46320_e78793: f64 = (locals.var_qs_1 - locals.var_t0);
        let assign46320_e78794: f64 = (assign46320_e78790 * assign46320_e78793);
        (assign46320_e78794, (((2.0 * locals.var_lambdac_dn3) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn3 - locals.var_t0_dn3))), (((2.0 * locals.var_lambdac_dn4) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn4 - locals.var_t0_dn4))), (((2.0 * locals.var_lambdac_dn5) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn5 - locals.var_t0_dn5))), (((2.0 * locals.var_lambdac_dn6) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn6 - locals.var_t0_dn6))), (((2.0 * locals.var_lambdac_dn7) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn7 - locals.var_t0_dn7))), (((2.0 * locals.var_lambdac_dn8) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn8 - locals.var_t0_dn8))), (((2.0 * locals.var_lambdac_dn9) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn9 - locals.var_t0_dn9))), (((2.0 * locals.var_lambdac_dn10) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn10 - locals.var_t0_dn10))), (((2.0 * locals.var_lambdac_dn11) * assign46320_e78793) + (assign46320_e78790 * (locals.var_qs_1_dn11 - locals.var_t0_dn11))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign46320_e78796;
        locals.var_t1_dn3 = assign46320_e78796_d_n3;
        locals.var_t1_dn4 = assign46320_e78796_d_n4;
        locals.var_t1_dn5 = assign46320_e78796_d_n5;
        locals.var_t1_dn6 = assign46320_e78796_d_n6;
        locals.var_t1_dn7 = assign46320_e78796_d_n7;
        locals.var_t1_dn8 = assign46320_e78796_d_n8;
        locals.var_t1_dn9 = assign46320_e78796_d_n9;
        locals.var_t1_dn10 = assign46320_e78796_d_n10;
        locals.var_t1_dn11 = assign46320_e78796_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign46330_e78809, assign46330_e78809_d_n3, assign46330_e78809_d_n4, assign46330_e78809_d_n5, assign46330_e78809_d_n6, assign46330_e78809_d_n7, assign46330_e78809_d_n8, assign46330_e78809_d_n9, assign46330_e78809_d_n10, assign46330_e78809_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46330_e78805: f64 = (locals.var_t1 * locals.var_t1);
        let assign46330_e78806: f64 = (1.0 + assign46330_e78805);
        let assign46330_e78807: f64 = (assign46330_e78806).sqrt();
        (assign46330_e78807, (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign46330_e78807)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign46330_e78807)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46330_e78809;
        locals.var_t2_dn3 = assign46330_e78809_d_n3;
        locals.var_t2_dn4 = assign46330_e78809_d_n4;
        locals.var_t2_dn5 = assign46330_e78809_d_n5;
        locals.var_t2_dn6 = assign46330_e78809_d_n6;
        locals.var_t2_dn7 = assign46330_e78809_d_n7;
        locals.var_t2_dn8 = assign46330_e78809_d_n8;
        locals.var_t2_dn9 = assign46330_e78809_d_n9;
        locals.var_t2_dn10 = assign46330_e78809_d_n10;
        locals.var_t2_dn11 = assign46330_e78809_d_n11;
        locals.var_t2_rv = 0.0;

        let assign46340_e78812: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard722 = assign46340_e78812;
        locals.var_guard722_rv = 0.0;

        let (assign46350_e78823, assign46350_e78823_d_n3, assign46350_e78823_d_n4, assign46350_e78823_d_n5, assign46350_e78823_d_n6, assign46350_e78823_d_n7, assign46350_e78823_d_n8, assign46350_e78823_d_n9, assign46350_e78823_d_n10, assign46350_e78823_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard722 != 0.0)) {
        let assign46350_e78821: f64 = (locals.var_t1).asinh();
        (assign46350_e78821, (locals.var_t1_dn3 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn4 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn5 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn6 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn7 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn8 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn9 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn10 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()), (locals.var_t1_dn11 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()),)
    } else {
        (locals.var_ln_t1_t2, locals.var_ln_t1_t2_dn3, locals.var_ln_t1_t2_dn4, locals.var_ln_t1_t2_dn5, locals.var_ln_t1_t2_dn6, locals.var_ln_t1_t2_dn7, locals.var_ln_t1_t2_dn8, locals.var_ln_t1_t2_dn9, locals.var_ln_t1_t2_dn10, locals.var_ln_t1_t2_dn11,)
    }
};
        locals.var_ln_t1_t2 = assign46350_e78823;
        locals.var_ln_t1_t2_dn3 = assign46350_e78823_d_n3;
        locals.var_ln_t1_t2_dn4 = assign46350_e78823_d_n4;
        locals.var_ln_t1_t2_dn5 = assign46350_e78823_d_n5;
        locals.var_ln_t1_t2_dn6 = assign46350_e78823_d_n6;
        locals.var_ln_t1_t2_dn7 = assign46350_e78823_d_n7;
        locals.var_ln_t1_t2_dn8 = assign46350_e78823_d_n8;
        locals.var_ln_t1_t2_dn9 = assign46350_e78823_d_n9;
        locals.var_ln_t1_t2_dn10 = assign46350_e78823_d_n10;
        locals.var_ln_t1_t2_dn11 = assign46350_e78823_d_n11;
        locals.var_ln_t1_t2_rv = 0.0;

        let (assign46360_e78839, assign46360_e78839_d_n3, assign46360_e78839_d_n4, assign46360_e78839_d_n5, assign46360_e78839_d_n6, assign46360_e78839_d_n7, assign46360_e78839_d_n8, assign46360_e78839_d_n9, assign46360_e78839_d_n10, assign46360_e78839_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard722 != 0.0)) {
        let assign46360_e78834: f64 = (1.0 / locals.var_t1);
        let assign46360_e78836: f64 = (assign46360_e78834 * locals.var_ln_t1_t2);
        let assign46360_e78837: f64 = (locals.var_t2 + assign46360_e78836);
        (assign46360_e78837, (locals.var_t2_dn3 + (((-(locals.var_t1_dn3 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn3))), (locals.var_t2_dn4 + (((-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn4))), (locals.var_t2_dn5 + (((-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn5))), (locals.var_t2_dn6 + (((-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn6))), (locals.var_t2_dn7 + (((-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn7))), (locals.var_t2_dn8 + (((-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn8))), (locals.var_t2_dn9 + (((-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn9))), (locals.var_t2_dn10 + (((-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn10))), (locals.var_t2_dn11 + (((-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))) * locals.var_ln_t1_t2) + (assign46360_e78834 * locals.var_ln_t1_t2_dn11))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46360_e78839;
        locals.var_t3_dn3 = assign46360_e78839_d_n3;
        locals.var_t3_dn4 = assign46360_e78839_d_n4;
        locals.var_t3_dn5 = assign46360_e78839_d_n5;
        locals.var_t3_dn6 = assign46360_e78839_d_n6;
        locals.var_t3_dn7 = assign46360_e78839_d_n7;
        locals.var_t3_dn8 = assign46360_e78839_d_n8;
        locals.var_t3_dn9 = assign46360_e78839_d_n9;
        locals.var_t3_dn10 = assign46360_e78839_d_n10;
        locals.var_t3_dn11 = assign46360_e78839_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46370_e78854, assign46370_e78854_d_n3, assign46370_e78854_d_n4, assign46370_e78854_d_n5, assign46370_e78854_d_n6, assign46370_e78854_d_n7, assign46370_e78854_d_n8, assign46370_e78854_d_n9, assign46370_e78854_d_n10, assign46370_e78854_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard722 == 0.0)) {
        let assign46370_e78851: f64 = (1.0 / locals.var_t2);
        let assign46370_e78852: f64 = (locals.var_t2 + assign46370_e78851);
        (assign46370_e78852, (locals.var_t2_dn3 + (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn4 + (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn5 + (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn6 + (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn7 + (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn8 + (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn9 + (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn10 + (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2)))), (locals.var_t2_dn11 + (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46370_e78854;
        locals.var_t3_dn3 = assign46370_e78854_d_n3;
        locals.var_t3_dn4 = assign46370_e78854_d_n4;
        locals.var_t3_dn5 = assign46370_e78854_d_n5;
        locals.var_t3_dn6 = assign46370_e78854_d_n6;
        locals.var_t3_dn7 = assign46370_e78854_d_n7;
        locals.var_t3_dn8 = assign46370_e78854_d_n8;
        locals.var_t3_dn9 = assign46370_e78854_d_n9;
        locals.var_t3_dn10 = assign46370_e78854_d_n10;
        locals.var_t3_dn11 = assign46370_e78854_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46380_e78878, assign46380_e78878_d_n3, assign46380_e78878_d_n4, assign46380_e78878_d_n5, assign46380_e78878_d_n6, assign46380_e78878_d_n7, assign46380_e78878_d_n8, assign46380_e78878_d_n9, assign46380_e78878_d_n10, assign46380_e78878_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46380_e78862: f64 = (locals.var_t0 * locals.var_t3);
        let assign46380_e78866: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign46380_e78868: f64 = (assign46380_e78866 + locals.var_qs_1);
        let assign46380_e78871: f64 = (locals.var_t0 * locals.var_t0);
        let assign46380_e78873: f64 = (assign46380_e78871 + locals.var_t0);
        let assign46380_e78874: f64 = (assign46380_e78868 - assign46380_e78873);
        let assign46380_e78875: f64 = (locals.var_lambdac * assign46380_e78874);
        let assign46380_e78876: f64 = (assign46380_e78862 - assign46380_e78875);
        (assign46380_e78876, (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) - ((locals.var_lambdac_dn3 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3) - (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + locals.var_t0_dn3))))), (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) - ((locals.var_lambdac_dn4 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4) - (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + locals.var_t0_dn4))))), (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) - ((locals.var_lambdac_dn5 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5) - (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + locals.var_t0_dn5))))), (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) - ((locals.var_lambdac_dn6 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6) - (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + locals.var_t0_dn6))))), (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) - ((locals.var_lambdac_dn7 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7) - (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + locals.var_t0_dn7))))), (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) - ((locals.var_lambdac_dn8 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8) - (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + locals.var_t0_dn8))))), (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) - ((locals.var_lambdac_dn9 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9) - (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + locals.var_t0_dn9))))), (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) - ((locals.var_lambdac_dn10 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10) - (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + locals.var_t0_dn10))))), (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) - ((locals.var_lambdac_dn11 * assign46380_e78874) + (locals.var_lambdac * ((((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11) - (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + locals.var_t0_dn11))))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46380_e78878;
        locals.var_t4_dn3 = assign46380_e78878_d_n3;
        locals.var_t4_dn4 = assign46380_e78878_d_n4;
        locals.var_t4_dn5 = assign46380_e78878_d_n5;
        locals.var_t4_dn6 = assign46380_e78878_d_n6;
        locals.var_t4_dn7 = assign46380_e78878_d_n7;
        locals.var_t4_dn8 = assign46380_e78878_d_n8;
        locals.var_t4_dn9 = assign46380_e78878_d_n9;
        locals.var_t4_dn10 = assign46380_e78878_d_n10;
        locals.var_t4_dn11 = assign46380_e78878_d_n11;
        locals.var_t4_rv = 0.0;

        let assign46390_e78881: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard723 = assign46390_e78881;
        locals.var_guard723_rv = 0.0;

        let (assign46400_e78904, assign46400_e78904_d_n3, assign46400_e78904_d_n4, assign46400_e78904_d_n5, assign46400_e78904_d_n6, assign46400_e78904_d_n7, assign46400_e78904_d_n8, assign46400_e78904_d_n9, assign46400_e78904_d_n10, assign46400_e78904_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign46400_e78890: f64 = (-2.0);
        let assign46400_e78892: f64 = (assign46400_e78890 * locals.var_lambdac);
        let assign46400_e78895: f64 = (locals.var_t1 * locals.var_t2);
        let assign46400_e78897: f64 = (assign46400_e78895 - locals.var_ln_t1_t2);
        let assign46400_e78898: f64 = (assign46400_e78892 * assign46400_e78897);
        let assign46400_e78901: f64 = (locals.var_t1 * locals.var_t1);
        let assign46400_e78902: f64 = (assign46400_e78898 / assign46400_e78901);
        (assign46400_e78902, ((((((assign46400_e78890 * locals.var_lambdac_dn3) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) - locals.var_ln_t1_t2_dn3))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn4) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) - locals.var_ln_t1_t2_dn4))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn5) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) - locals.var_ln_t1_t2_dn5))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn6) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) - locals.var_ln_t1_t2_dn6))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn7) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) - locals.var_ln_t1_t2_dn7))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn8) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) - locals.var_ln_t1_t2_dn8))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn9) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) - locals.var_ln_t1_t2_dn9))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn10) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) - locals.var_ln_t1_t2_dn10))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)))) / (assign46400_e78901 * assign46400_e78901)), ((((((assign46400_e78890 * locals.var_lambdac_dn11) * assign46400_e78897) + (assign46400_e78892 * (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) - locals.var_ln_t1_t2_dn11))) * assign46400_e78901) - (assign46400_e78898 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)))) / (assign46400_e78901 * assign46400_e78901)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46400_e78904;
        locals.var_t5_dn3 = assign46400_e78904_d_n3;
        locals.var_t5_dn4 = assign46400_e78904_d_n4;
        locals.var_t5_dn5 = assign46400_e78904_d_n5;
        locals.var_t5_dn6 = assign46400_e78904_d_n6;
        locals.var_t5_dn7 = assign46400_e78904_d_n7;
        locals.var_t5_dn8 = assign46400_e78904_d_n8;
        locals.var_t5_dn9 = assign46400_e78904_d_n9;
        locals.var_t5_dn10 = assign46400_e78904_d_n10;
        locals.var_t5_dn11 = assign46400_e78904_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46410_e78922, assign46410_e78922_d_n3, assign46410_e78922_d_n4, assign46410_e78922_d_n5, assign46410_e78922_d_n6, assign46410_e78922_d_n7, assign46410_e78922_d_n8, assign46410_e78922_d_n9, assign46410_e78922_d_n10, assign46410_e78922_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) && (locals.var_guard723 == 0.0)) {
        let assign46410_e78914: f64 = (-2.0);
        let assign46410_e78916: f64 = (assign46410_e78914 * locals.var_lambdac);
        let assign46410_e78919: f64 = (locals.var_t1 / locals.var_t2);
        let assign46410_e78920: f64 = (assign46410_e78916 * assign46410_e78919);
        (assign46410_e78920, (((assign46410_e78914 * locals.var_lambdac_dn3) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn4) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn5) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn6) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn7) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn8) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn9) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn10) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)))), (((assign46410_e78914 * locals.var_lambdac_dn11) * assign46410_e78919) + (assign46410_e78916 * (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46410_e78922;
        locals.var_t5_dn3 = assign46410_e78922_d_n3;
        locals.var_t5_dn4 = assign46410_e78922_d_n4;
        locals.var_t5_dn5 = assign46410_e78922_d_n5;
        locals.var_t5_dn6 = assign46410_e78922_d_n6;
        locals.var_t5_dn7 = assign46410_e78922_d_n7;
        locals.var_t5_dn8 = assign46410_e78922_d_n8;
        locals.var_t5_dn9 = assign46410_e78922_d_n9;
        locals.var_t5_dn10 = assign46410_e78922_d_n10;
        locals.var_t5_dn11 = assign46410_e78922_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_161(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign46420_e78942, assign46420_e78942_d_n3, assign46420_e78942_d_n4, assign46420_e78942_d_n5, assign46420_e78942_d_n6, assign46420_e78942_d_n7, assign46420_e78942_d_n8, assign46420_e78942_d_n9, assign46420_e78942_d_n10, assign46420_e78942_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46420_e78930: f64 = (locals.var_t0 * locals.var_t5);
        let assign46420_e78932: f64 = (assign46420_e78930 + locals.var_t3);
        let assign46420_e78936: f64 = (2.0 * locals.var_t0);
        let assign46420_e78938: f64 = (assign46420_e78936 + 1.0);
        let assign46420_e78939: f64 = (locals.var_lambdac * assign46420_e78938);
        let assign46420_e78940: f64 = (assign46420_e78932 + assign46420_e78939);
        (assign46420_e78940, ((((locals.var_t0_dn3 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn3)) + locals.var_t3_dn3) + ((locals.var_lambdac_dn3 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn3)))), ((((locals.var_t0_dn4 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn4)) + locals.var_t3_dn4) + ((locals.var_lambdac_dn4 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn4)))), ((((locals.var_t0_dn5 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn5)) + locals.var_t3_dn5) + ((locals.var_lambdac_dn5 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn5)))), ((((locals.var_t0_dn6 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn6)) + locals.var_t3_dn6) + ((locals.var_lambdac_dn6 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn6)))), ((((locals.var_t0_dn7 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn7)) + locals.var_t3_dn7) + ((locals.var_lambdac_dn7 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn7)))), ((((locals.var_t0_dn8 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn8)) + locals.var_t3_dn8) + ((locals.var_lambdac_dn8 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn8)))), ((((locals.var_t0_dn9 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn9)) + locals.var_t3_dn9) + ((locals.var_lambdac_dn9 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn9)))), ((((locals.var_t0_dn10 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn10)) + locals.var_t3_dn10) + ((locals.var_lambdac_dn10 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn10)))), ((((locals.var_t0_dn11 * locals.var_t5) + (locals.var_t0 * locals.var_t5_dn11)) + locals.var_t3_dn11) + ((locals.var_lambdac_dn11 * assign46420_e78938) + (locals.var_lambdac * (2.0 * locals.var_t0_dn11)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign46420_e78942;
        locals.var_t6_dn3 = assign46420_e78942_d_n3;
        locals.var_t6_dn4 = assign46420_e78942_d_n4;
        locals.var_t6_dn5 = assign46420_e78942_d_n5;
        locals.var_t6_dn6 = assign46420_e78942_d_n6;
        locals.var_t6_dn7 = assign46420_e78942_d_n7;
        locals.var_t6_dn8 = assign46420_e78942_d_n8;
        locals.var_t6_dn9 = assign46420_e78942_d_n9;
        locals.var_t6_dn10 = assign46420_e78942_d_n10;
        locals.var_t6_dn11 = assign46420_e78942_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign46430_e78954, assign46430_e78954_d_n3, assign46430_e78954_d_n4, assign46430_e78954_d_n5, assign46430_e78954_d_n6, assign46430_e78954_d_n7, assign46430_e78954_d_n8, assign46430_e78954_d_n9, assign46430_e78954_d_n10, assign46430_e78954_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard715 == 0.0)) {
        let assign46430_e78951: f64 = (locals.var_t4 / locals.var_t6);
        let assign46430_e78952: f64 = (locals.var_t0 - assign46430_e78951);
        (assign46430_e78952, (locals.var_t0_dn3 - (((locals.var_t4_dn3 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn4 - (((locals.var_t4_dn4 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn5 - (((locals.var_t4_dn5 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn6 - (((locals.var_t4_dn6 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn7 - (((locals.var_t4_dn7 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn8 - (((locals.var_t4_dn8 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn9 - (((locals.var_t4_dn9 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn10 - (((locals.var_t4_dn10 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6))), (locals.var_t0_dn11 - (((locals.var_t4_dn11 * locals.var_t6) - (locals.var_t4 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6))),)
    } else {
        (locals.var_qdsat, locals.var_qdsat_dn3, locals.var_qdsat_dn4, locals.var_qdsat_dn5, locals.var_qdsat_dn6, locals.var_qdsat_dn7, locals.var_qdsat_dn8, locals.var_qdsat_dn9, locals.var_qdsat_dn10, locals.var_qdsat_dn11,)
    }
};
        locals.var_qdsat = assign46430_e78954;
        locals.var_qdsat_dn3 = assign46430_e78954_d_n3;
        locals.var_qdsat_dn4 = assign46430_e78954_d_n4;
        locals.var_qdsat_dn5 = assign46430_e78954_d_n5;
        locals.var_qdsat_dn6 = assign46430_e78954_d_n6;
        locals.var_qdsat_dn7 = assign46430_e78954_d_n7;
        locals.var_qdsat_dn8 = assign46430_e78954_d_n8;
        locals.var_qdsat_dn9 = assign46430_e78954_d_n9;
        locals.var_qdsat_dn10 = assign46430_e78954_d_n10;
        locals.var_qdsat_dn11 = assign46430_e78954_d_n11;
        locals.var_qdsat_rv = 0.0;

        let (assign46440_e78992, assign46440_e78992_d_n3, assign46440_e78992_d_n4, assign46440_e78992_d_n5, assign46440_e78992_d_n6, assign46440_e78992_d_n7, assign46440_e78992_d_n8, assign46440_e78992_d_n9, assign46440_e78992_d_n10, assign46440_e78992_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46440_e78960: f64 = (2.0 * locals.var_phib_n);
        let assign46440_e78961: f64 = (locals.var_psip - assign46440_e78960);
        let assign46440_e78964: f64 = (2.0 * locals.var_qdsat);
        let assign46440_e78967: f64 = (locals.var_qdsat * 2.0);
        let assign46440_e78969: f64 = (assign46440_e78967 * locals.var_nq);
        let assign46440_e78971: f64 = (assign46440_e78969 * locals.var_inv_gam);
        let assign46440_e78974: f64 = (locals.var_qdsat * 2.0);
        let assign46440_e78976: f64 = (assign46440_e78974 * locals.var_nq);
        let assign46440_e78978: f64 = (assign46440_e78976 * locals.var_inv_gam);
        let assign46440_e78982: f64 = (locals.var_nq - 1.0);
        let assign46440_e78983: f64 = (locals.var_gam / assign46440_e78982);
        let assign46440_e78984: f64 = (assign46440_e78978 + assign46440_e78983);
        let assign46440_e78985: f64 = (assign46440_e78971 * assign46440_e78984);
        let assign46440_e78987: f64 = (assign46440_e78985).max(1e-38);
        let assign46440_e78988: f64 = (assign46440_e78987).ln();
        let assign46440_e78989: f64 = (assign46440_e78964 + assign46440_e78988);
        let assign46440_e78990: f64 = (assign46440_e78961 - assign46440_e78989);
        (assign46440_e78990, ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_dn3)) - ((2.0 * locals.var_qdsat_dn3) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn3 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn3)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn3)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn3 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn3)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn3)) + (((locals.var_gam_dn3 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn3)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_dn4)) - ((2.0 * locals.var_qdsat_dn4) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn4 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn4)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn4)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn4 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn4)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn4)) + (((locals.var_gam_dn4 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn4)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_dn5)) - ((2.0 * locals.var_qdsat_dn5) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn5 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn5)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn5)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn5 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn5)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn5)) + (((locals.var_gam_dn5 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn5)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_dn6)) - ((2.0 * locals.var_qdsat_dn6) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn6 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn6)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn6)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn6 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn6)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn6)) + (((locals.var_gam_dn6 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn6)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_dn7)) - ((2.0 * locals.var_qdsat_dn7) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn7 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn7)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn7)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn7 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn7)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn7)) + (((locals.var_gam_dn7 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn7)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_dn8)) - ((2.0 * locals.var_qdsat_dn8) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn8 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn8)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn8)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn8 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn8)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn8)) + (((locals.var_gam_dn8 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn8)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_dn9)) - ((2.0 * locals.var_qdsat_dn9) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn9 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn9)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn9)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn9 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn9)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn9)) + (((locals.var_gam_dn9 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn9)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_dn10)) - ((2.0 * locals.var_qdsat_dn10) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn10 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn10)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn10)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn10 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn10)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn10)) + (((locals.var_gam_dn10 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn10)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_dn11)) - ((2.0 * locals.var_qdsat_dn11) + (if assign46440_e78985 >= 1e-38 { (((((((locals.var_qdsat_dn11 * 2.0) * locals.var_nq) + (assign46440_e78967 * locals.var_nq_dn11)) * locals.var_inv_gam) + (assign46440_e78969 * locals.var_inv_gam_dn11)) * assign46440_e78984) + (assign46440_e78971 * ((((((locals.var_qdsat_dn11 * 2.0) * locals.var_nq) + (assign46440_e78974 * locals.var_nq_dn11)) * locals.var_inv_gam) + (assign46440_e78976 * locals.var_inv_gam_dn11)) + (((locals.var_gam_dn11 * assign46440_e78982) - (locals.var_gam * locals.var_nq_dn11)) / (assign46440_e78982 * assign46440_e78982))))) } else { 0.0 } / assign46440_e78987))),)
    } else {
        (locals.var_vdsat_1, locals.var_vdsat_1_dn3, locals.var_vdsat_1_dn4, locals.var_vdsat_1_dn5, locals.var_vdsat_1_dn6, locals.var_vdsat_1_dn7, locals.var_vdsat_1_dn8, locals.var_vdsat_1_dn9, locals.var_vdsat_1_dn10, locals.var_vdsat_1_dn11,)
    }
};
        locals.var_vdsat_1 = assign46440_e78992;
        locals.var_vdsat_1_dn3 = assign46440_e78992_d_n3;
        locals.var_vdsat_1_dn4 = assign46440_e78992_d_n4;
        locals.var_vdsat_1_dn5 = assign46440_e78992_d_n5;
        locals.var_vdsat_1_dn6 = assign46440_e78992_d_n6;
        locals.var_vdsat_1_dn7 = assign46440_e78992_d_n7;
        locals.var_vdsat_1_dn8 = assign46440_e78992_d_n8;
        locals.var_vdsat_1_dn9 = assign46440_e78992_d_n9;
        locals.var_vdsat_1_dn10 = assign46440_e78992_d_n10;
        locals.var_vdsat_1_dn11 = assign46440_e78992_d_n11;
        locals.var_vdsat_1_rv = 0.0;

        let (assign46450_e78999, assign46450_e78999_d_n3, assign46450_e78999_d_n4, assign46450_e78999_d_n5, assign46450_e78999_d_n6, assign46450_e78999_d_n7, assign46450_e78999_d_n8, assign46450_e78999_d_n9, assign46450_e78999_d_n10, assign46450_e78999_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46450_e78997: f64 = (locals.var_vdsat_1 * locals.var_nvt);
        (assign46450_e78997, ((locals.var_vdsat_1_dn3 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn3)), ((locals.var_vdsat_1_dn4 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn4)), ((locals.var_vdsat_1_dn5 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn5)), ((locals.var_vdsat_1_dn6 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn6)), ((locals.var_vdsat_1_dn7 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn7)), ((locals.var_vdsat_1_dn8 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn8)), ((locals.var_vdsat_1_dn9 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn9)), ((locals.var_vdsat_1_dn10 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn10)), ((locals.var_vdsat_1_dn11 * locals.var_nvt) + (locals.var_vdsat_1 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vdsat, locals.var_vdsat_dn3, locals.var_vdsat_dn4, locals.var_vdsat_dn5, locals.var_vdsat_dn6, locals.var_vdsat_dn7, locals.var_vdsat_dn8, locals.var_vdsat_dn9, locals.var_vdsat_dn10, locals.var_vdsat_dn11,)
    }
};
        locals.var_vdsat = assign46450_e78999;
        locals.var_vdsat_dn3 = assign46450_e78999_d_n3;
        locals.var_vdsat_dn4 = assign46450_e78999_d_n4;
        locals.var_vdsat_dn5 = assign46450_e78999_d_n5;
        locals.var_vdsat_dn6 = assign46450_e78999_d_n6;
        locals.var_vdsat_dn7 = assign46450_e78999_d_n7;
        locals.var_vdsat_dn8 = assign46450_e78999_d_n8;
        locals.var_vdsat_dn9 = assign46450_e78999_d_n9;
        locals.var_vdsat_dn10 = assign46450_e78999_d_n10;
        locals.var_vdsat_dn11 = assign46450_e78999_d_n11;
        locals.var_vdsat_rv = 0.0;

        let assign46460_e79006: f64 = if ((p.p1349 == 0.0) && (p.p1350 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard724 = assign46460_e79006;
        locals.var_guard724_rv = 0.0;

        let (assign46470_e79013, assign46470_e79013_d_n3, assign46470_e79013_d_n4, assign46470_e79013_d_n5, assign46470_e79013_d_n6, assign46470_e79013_d_n7, assign46470_e79013_d_n8, assign46470_e79013_d_n9, assign46470_e79013_d_n10, assign46470_e79013_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard724 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abulkiv, locals.var_abulkiv_dn3, locals.var_abulkiv_dn4, locals.var_abulkiv_dn5, locals.var_abulkiv_dn6, locals.var_abulkiv_dn7, locals.var_abulkiv_dn8, locals.var_abulkiv_dn9, locals.var_abulkiv_dn10, locals.var_abulkiv_dn11,)
    }
};
        locals.var_abulkiv = assign46470_e79013;
        locals.var_abulkiv_dn3 = assign46470_e79013_d_n3;
        locals.var_abulkiv_dn4 = assign46470_e79013_d_n4;
        locals.var_abulkiv_dn5 = assign46470_e79013_d_n5;
        locals.var_abulkiv_dn6 = assign46470_e79013_d_n6;
        locals.var_abulkiv_dn7 = assign46470_e79013_d_n7;
        locals.var_abulkiv_dn8 = assign46470_e79013_d_n8;
        locals.var_abulkiv_dn9 = assign46470_e79013_d_n9;
        locals.var_abulkiv_dn10 = assign46470_e79013_d_n10;
        locals.var_abulkiv_dn11 = assign46470_e79013_d_n11;
        locals.var_abulkiv_rv = 0.0;

        let (assign46480_e79028, assign46480_e79028_d_n3, assign46480_e79028_d_n4, assign46480_e79028_d_n5, assign46480_e79028_d_n6, assign46480_e79028_d_n7, assign46480_e79028_d_n8, assign46480_e79028_d_n9, assign46480_e79028_d_n10, assign46480_e79028_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard724 == 0.0)) {
        let assign46480_e79023: f64 = (locals.var_xj_i * locals.var_xdep);
        let assign46480_e79024: f64 = (assign46480_e79023).sqrt();
        let assign46480_e79025: f64 = (locals.var_leff + assign46480_e79024);
        let assign46480_e79026: f64 = (locals.var_leff / assign46480_e79025);
        (assign46480_e79026, (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn3) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn4) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn5) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn6) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn7) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn8) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn9) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn10) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))), (-((locals.var_leff * ((locals.var_xj_i * locals.var_xdep_dn11) / (2.0 * assign46480_e79024))) / (assign46480_e79025 * assign46480_e79025))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign46480_e79028;
        locals.var_t1_dn3 = assign46480_e79028_d_n3;
        locals.var_t1_dn4 = assign46480_e79028_d_n4;
        locals.var_t1_dn5 = assign46480_e79028_d_n5;
        locals.var_t1_dn6 = assign46480_e79028_d_n6;
        locals.var_t1_dn7 = assign46480_e79028_d_n7;
        locals.var_t1_dn8 = assign46480_e79028_d_n8;
        locals.var_t1_dn9 = assign46480_e79028_d_n9;
        locals.var_t1_dn10 = assign46480_e79028_d_n10;
        locals.var_t1_dn11 = assign46480_e79028_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign46490_e79056, assign46490_e79056_d_n3, assign46490_e79056_d_n4, assign46490_e79056_d_n5, assign46490_e79056_d_n6, assign46490_e79056_d_n7, assign46490_e79056_d_n8, assign46490_e79056_d_n9, assign46490_e79056_d_n10, assign46490_e79056_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard724 == 0.0)) {
        let assign46490_e79037: f64 = (p.p1349 * locals.var_t1);
        let assign46490_e79040: f64 = (p.p1350 * locals.var_t1);
        let assign46490_e79043: f64 = (locals.var_qs_1).powf(p.p1351);
        let assign46490_e79044: f64 = (assign46490_e79040 * assign46490_e79043);
        let assign46490_e79046: f64 = (assign46490_e79044 * locals.var_nvt);
        let assign46490_e79047: f64 = (assign46490_e79037 - assign46490_e79046);
        let assign46490_e79051: f64 = (p.p1352 * locals.var_vbsx);
        let assign46490_e79052: f64 = (1.0 + assign46490_e79051);
        let assign46490_e79053: f64 = (assign46490_e79047 / assign46490_e79052);
        let assign46490_e79054: f64 = (1.0 + assign46490_e79053);
        (assign46490_e79054, (((((p.p1349 * locals.var_t1_dn3) - (((((p.p1350 * locals.var_t1_dn3) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn3)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn3 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn3))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn3))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn4) - (((((p.p1350 * locals.var_t1_dn4) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn4)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn4 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn4))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn4))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn5) - (((((p.p1350 * locals.var_t1_dn5) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn5)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn5 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn5))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn5))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn6) - (((((p.p1350 * locals.var_t1_dn6) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn6)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn6 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn6))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn6))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn7) - (((((p.p1350 * locals.var_t1_dn7) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn7)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn7 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn7))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn7))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn8) - (((((p.p1350 * locals.var_t1_dn8) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn8)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn8 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn8))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn8))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn9) - (((((p.p1350 * locals.var_t1_dn9) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn9)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn9 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn9))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn9))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn10) - (((((p.p1350 * locals.var_t1_dn10) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn10)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn10 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn10))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn10))) / (assign46490_e79052 * assign46490_e79052)), (((((p.p1349 * locals.var_t1_dn11) - (((((p.p1350 * locals.var_t1_dn11) * assign46490_e79043) + (assign46490_e79040 * if 0.0 == 0.0 && ((p.p1351) as f64).is_finite() && ((p.p1351) as f64).fract() == 0.0 { if p.p1351 == 0.0 { 0.0 } else { (p.p1351 * ((locals.var_qs_1).powf(p.p1351 - 1.0) * locals.var_qs_1_dn11)) } } else { (assign46490_e79043 * (p.p1351 * (locals.var_qs_1_dn11 / locals.var_qs_1))) })) * locals.var_nvt) + (assign46490_e79044 * locals.var_nvt_dn11))) * assign46490_e79052) - (assign46490_e79047 * (p.p1352 * locals.var_vbsx_dn11))) / (assign46490_e79052 * assign46490_e79052)),)
    } else {
        (locals.var_abulkiv, locals.var_abulkiv_dn3, locals.var_abulkiv_dn4, locals.var_abulkiv_dn5, locals.var_abulkiv_dn6, locals.var_abulkiv_dn7, locals.var_abulkiv_dn8, locals.var_abulkiv_dn9, locals.var_abulkiv_dn10, locals.var_abulkiv_dn11,)
    }
};
        locals.var_abulkiv = assign46490_e79056;
        locals.var_abulkiv_dn3 = assign46490_e79056_d_n3;
        locals.var_abulkiv_dn4 = assign46490_e79056_d_n4;
        locals.var_abulkiv_dn5 = assign46490_e79056_d_n5;
        locals.var_abulkiv_dn6 = assign46490_e79056_d_n6;
        locals.var_abulkiv_dn7 = assign46490_e79056_d_n7;
        locals.var_abulkiv_dn8 = assign46490_e79056_d_n8;
        locals.var_abulkiv_dn9 = assign46490_e79056_d_n9;
        locals.var_abulkiv_dn10 = assign46490_e79056_d_n10;
        locals.var_abulkiv_dn11 = assign46490_e79056_d_n11;
        locals.var_abulkiv_rv = 0.0;

        let (assign46500_e79083, assign46500_e79083_d_n3, assign46500_e79083_d_n4, assign46500_e79083_d_n5, assign46500_e79083_d_n6, assign46500_e79083_d_n7, assign46500_e79083_d_n8, assign46500_e79083_d_n9, assign46500_e79083_d_n10, assign46500_e79083_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard724 == 0.0)) {
        let assign46500_e79065: f64 = (locals.var_abulkiv + 0.1);
        let assign46500_e79068: f64 = (locals.var_abulkiv - 0.1);
        let assign46500_e79071: f64 = (locals.var_abulkiv - 0.1);
        let assign46500_e79072: f64 = (assign46500_e79068 * assign46500_e79071);
        let assign46500_e79075: f64 = (0.25 * 0.0005);
        let assign46500_e79077: f64 = (assign46500_e79075 * 0.0005);
        let assign46500_e79078: f64 = (assign46500_e79072 + assign46500_e79077);
        let assign46500_e79079: f64 = (assign46500_e79078).sqrt();
        let assign46500_e79080: f64 = (assign46500_e79065 + assign46500_e79079);
        let assign46500_e79081: f64 = (0.5 * assign46500_e79080);
        (assign46500_e79081, (0.5 * (locals.var_abulkiv_dn3 + (((locals.var_abulkiv_dn3 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn3)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn4 + (((locals.var_abulkiv_dn4 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn4)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn5 + (((locals.var_abulkiv_dn5 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn5)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn6 + (((locals.var_abulkiv_dn6 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn6)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn7 + (((locals.var_abulkiv_dn7 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn7)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn8 + (((locals.var_abulkiv_dn8 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn8)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn9 + (((locals.var_abulkiv_dn9 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn9)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn10 + (((locals.var_abulkiv_dn10 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn10)) / (2.0 * assign46500_e79079)))), (0.5 * (locals.var_abulkiv_dn11 + (((locals.var_abulkiv_dn11 * assign46500_e79071) + (assign46500_e79068 * locals.var_abulkiv_dn11)) / (2.0 * assign46500_e79079)))),)
    } else {
        (locals.var_abulkiv, locals.var_abulkiv_dn3, locals.var_abulkiv_dn4, locals.var_abulkiv_dn5, locals.var_abulkiv_dn6, locals.var_abulkiv_dn7, locals.var_abulkiv_dn8, locals.var_abulkiv_dn9, locals.var_abulkiv_dn10, locals.var_abulkiv_dn11,)
    }
};
        locals.var_abulkiv = assign46500_e79083;
        locals.var_abulkiv_dn3 = assign46500_e79083_d_n3;
        locals.var_abulkiv_dn4 = assign46500_e79083_d_n4;
        locals.var_abulkiv_dn5 = assign46500_e79083_d_n5;
        locals.var_abulkiv_dn6 = assign46500_e79083_d_n6;
        locals.var_abulkiv_dn7 = assign46500_e79083_d_n7;
        locals.var_abulkiv_dn8 = assign46500_e79083_d_n8;
        locals.var_abulkiv_dn9 = assign46500_e79083_d_n9;
        locals.var_abulkiv_dn10 = assign46500_e79083_d_n10;
        locals.var_abulkiv_dn11 = assign46500_e79083_d_n11;
        locals.var_abulkiv_rv = 0.0;

        let (assign46510_e79113, assign46510_e79113_d_n3, assign46510_e79113_d_n4, assign46510_e79113_d_n5, assign46510_e79113_d_n6, assign46510_e79113_d_n7, assign46510_e79113_d_n8, assign46510_e79113_d_n9, assign46510_e79113_d_n10, assign46510_e79113_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46510_e79089: f64 = (locals.var_vdsat - locals.var_vs);
        let assign46510_e79091: f64 = assign46510_e79089;
        let assign46510_e79094: f64 = (locals.var_vdsat - locals.var_vs);
        let assign46510_e79096: f64 = assign46510_e79094;
        let assign46510_e79099: f64 = (locals.var_vdsat - locals.var_vs);
        let assign46510_e79101: f64 = assign46510_e79099;
        let assign46510_e79102: f64 = (assign46510_e79096 * assign46510_e79101);
        let assign46510_e79105: f64 = (0.25 * 0.001);
        let assign46510_e79107: f64 = (assign46510_e79105 * 0.001);
        let assign46510_e79108: f64 = (assign46510_e79102 + assign46510_e79107);
        let assign46510_e79109: f64 = (assign46510_e79108).sqrt();
        let assign46510_e79110: f64 = (assign46510_e79091 + assign46510_e79109);
        let assign46510_e79111: f64 = (0.5 * assign46510_e79110);
        (assign46510_e79111, (0.5 * (locals.var_vdsat_dn3 + (((locals.var_vdsat_dn3 * assign46510_e79101) + (assign46510_e79096 * locals.var_vdsat_dn3)) / (2.0 * assign46510_e79109)))), (0.5 * (locals.var_vdsat_dn4 + (((locals.var_vdsat_dn4 * assign46510_e79101) + (assign46510_e79096 * locals.var_vdsat_dn4)) / (2.0 * assign46510_e79109)))), (0.5 * (locals.var_vdsat_dn5 + (((locals.var_vdsat_dn5 * assign46510_e79101) + (assign46510_e79096 * locals.var_vdsat_dn5)) / (2.0 * assign46510_e79109)))), (0.5 * ((locals.var_vdsat_dn6 - locals.var_vs_dn6) + ((((locals.var_vdsat_dn6 - locals.var_vs_dn6) * assign46510_e79101) + (assign46510_e79096 * (locals.var_vdsat_dn6 - locals.var_vs_dn6))) / (2.0 * assign46510_e79109)))), (0.5 * ((locals.var_vdsat_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsat_dn7 - locals.var_vs_dn7) * assign46510_e79101) + (assign46510_e79096 * (locals.var_vdsat_dn7 - locals.var_vs_dn7))) / (2.0 * assign46510_e79109)))), (0.5 * (locals.var_vdsat_dn8 + (((locals.var_vdsat_dn8 * assign46510_e79101) + (assign46510_e79096 * locals.var_vdsat_dn8)) / (2.0 * assign46510_e79109)))), (0.5 * (locals.var_vdsat_dn9 + (((locals.var_vdsat_dn9 * assign46510_e79101) + (assign46510_e79096 * locals.var_vdsat_dn9)) / (2.0 * assign46510_e79109)))), (0.5 * ((locals.var_vdsat_dn10 - locals.var_vs_dn10) + ((((locals.var_vdsat_dn10 - locals.var_vs_dn10) * assign46510_e79101) + (assign46510_e79096 * (locals.var_vdsat_dn10 - locals.var_vs_dn10))) / (2.0 * assign46510_e79109)))), (0.5 * (locals.var_vdsat_dn11 + (((locals.var_vdsat_dn11 * assign46510_e79101) + (assign46510_e79096 * locals.var_vdsat_dn11)) / (2.0 * assign46510_e79109)))),)
    } else {
        (locals.var_vdssat, locals.var_vdssat_dn3, locals.var_vdssat_dn4, locals.var_vdssat_dn5, locals.var_vdssat_dn6, locals.var_vdssat_dn7, locals.var_vdssat_dn8, locals.var_vdssat_dn9, locals.var_vdssat_dn10, locals.var_vdssat_dn11,)
    }
};
        locals.var_vdssat = assign46510_e79113;
        locals.var_vdssat_dn3 = assign46510_e79113_d_n3;
        locals.var_vdssat_dn4 = assign46510_e79113_d_n4;
        locals.var_vdssat_dn5 = assign46510_e79113_d_n5;
        locals.var_vdssat_dn6 = assign46510_e79113_d_n6;
        locals.var_vdssat_dn7 = assign46510_e79113_d_n7;
        locals.var_vdssat_dn8 = assign46510_e79113_d_n8;
        locals.var_vdssat_dn9 = assign46510_e79113_d_n9;
        locals.var_vdssat_dn10 = assign46510_e79113_d_n10;
        locals.var_vdssat_dn11 = assign46510_e79113_d_n11;
        locals.var_vdssat_rv = 0.0;

        let (assign46520_e79120, assign46520_e79120_d_n3, assign46520_e79120_d_n4, assign46520_e79120_d_n5, assign46520_e79120_d_n6, assign46520_e79120_d_n7, assign46520_e79120_d_n8, assign46520_e79120_d_n9, assign46520_e79120_d_n10, assign46520_e79120_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46520_e79118: f64 = (locals.var_vdssat / locals.var_abulkiv);
        (assign46520_e79118, (((locals.var_vdssat_dn3 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn3)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn4 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn4)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn5 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn5)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn6 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn6)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn7 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn7)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn8 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn8)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn9 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn9)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn10 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn10)) / (locals.var_abulkiv * locals.var_abulkiv)), (((locals.var_vdssat_dn11 * locals.var_abulkiv) - (locals.var_vdssat * locals.var_abulkiv_dn11)) / (locals.var_abulkiv * locals.var_abulkiv)),)
    } else {
        (locals.var_vdssat, locals.var_vdssat_dn3, locals.var_vdssat_dn4, locals.var_vdssat_dn5, locals.var_vdssat_dn6, locals.var_vdssat_dn7, locals.var_vdssat_dn8, locals.var_vdssat_dn9, locals.var_vdssat_dn10, locals.var_vdssat_dn11,)
    }
};
        locals.var_vdssat = assign46520_e79120;
        locals.var_vdssat_dn3 = assign46520_e79120_d_n3;
        locals.var_vdssat_dn4 = assign46520_e79120_d_n4;
        locals.var_vdssat_dn5 = assign46520_e79120_d_n5;
        locals.var_vdssat_dn6 = assign46520_e79120_d_n6;
        locals.var_vdssat_dn7 = assign46520_e79120_d_n7;
        locals.var_vdssat_dn8 = assign46520_e79120_d_n8;
        locals.var_vdssat_dn9 = assign46520_e79120_d_n9;
        locals.var_vdssat_dn10 = assign46520_e79120_d_n10;
        locals.var_vdssat_dn11 = assign46520_e79120_d_n11;
        locals.var_vdssat_rv = 0.0;

        let (assign46530_e79133, assign46530_e79133_d_n3, assign46530_e79133_d_n4, assign46530_e79133_d_n5, assign46530_e79133_d_n6, assign46530_e79133_d_n7, assign46530_e79133_d_n8, assign46530_e79133_d_n9, assign46530_e79133_d_n10, assign46530_e79133_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46530_e79125: f64 = (locals.var_vds / locals.var_vdssat);
        let assign46530_e79127: f64 = (assign46530_e79125 + 1e-6);
        let assign46530_e79130: f64 = (1.0 / locals.var_delta_t);
        let assign46530_e79131: f64 = (assign46530_e79127).powf(assign46530_e79130);
        (assign46530_e79131, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (-((locals.var_vds * locals.var_vdssat_dn3) / (locals.var_vdssat * locals.var_vdssat))))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((-((locals.var_vds * locals.var_vdssat_dn3) / (locals.var_vdssat * locals.var_vdssat))) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (-((locals.var_vds * locals.var_vdssat_dn4) / (locals.var_vdssat * locals.var_vdssat))))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((-((locals.var_vds * locals.var_vdssat_dn4) / (locals.var_vdssat * locals.var_vdssat))) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (-((locals.var_vds * locals.var_vdssat_dn5) / (locals.var_vdssat * locals.var_vdssat))))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((-((locals.var_vds * locals.var_vdssat_dn5) / (locals.var_vdssat * locals.var_vdssat))) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (((locals.var_vds_dn6 * locals.var_vdssat) - (locals.var_vds * locals.var_vdssat_dn6)) / (locals.var_vdssat * locals.var_vdssat)))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((((locals.var_vds_dn6 * locals.var_vdssat) - (locals.var_vds * locals.var_vdssat_dn6)) / (locals.var_vdssat * locals.var_vdssat)) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssat) - (locals.var_vds * locals.var_vdssat_dn7)) / (locals.var_vdssat * locals.var_vdssat)))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((((locals.var_vds_dn7 * locals.var_vdssat) - (locals.var_vds * locals.var_vdssat_dn7)) / (locals.var_vdssat * locals.var_vdssat)) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (-((locals.var_vds * locals.var_vdssat_dn8) / (locals.var_vdssat * locals.var_vdssat))))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((-((locals.var_vds * locals.var_vdssat_dn8) / (locals.var_vdssat * locals.var_vdssat))) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (-((locals.var_vds * locals.var_vdssat_dn9) / (locals.var_vdssat * locals.var_vdssat))))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((-((locals.var_vds * locals.var_vdssat_dn9) / (locals.var_vdssat * locals.var_vdssat))) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (((locals.var_vds_dn10 * locals.var_vdssat) - (locals.var_vds * locals.var_vdssat_dn10)) / (locals.var_vdssat * locals.var_vdssat)))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((((locals.var_vds_dn10 * locals.var_vdssat) - (locals.var_vds * locals.var_vdssat_dn10)) / (locals.var_vdssat * locals.var_vdssat)) / assign46530_e79127)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign46530_e79130) as f64).is_finite() && ((assign46530_e79130) as f64).fract() == 0.0 { if assign46530_e79130 == 0.0 { 0.0 } else { (assign46530_e79130 * ((assign46530_e79127).powf(assign46530_e79130 - 1.0) * (-((locals.var_vds * locals.var_vdssat_dn11) / (locals.var_vdssat * locals.var_vdssat))))) } } else { (assign46530_e79131 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign46530_e79127).ln()) + (assign46530_e79130 * ((-((locals.var_vds * locals.var_vdssat_dn11) / (locals.var_vdssat * locals.var_vdssat))) / assign46530_e79127)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign46530_e79133;
        locals.var_t7_dn3 = assign46530_e79133_d_n3;
        locals.var_t7_dn4 = assign46530_e79133_d_n4;
        locals.var_t7_dn5 = assign46530_e79133_d_n5;
        locals.var_t7_dn6 = assign46530_e79133_d_n6;
        locals.var_t7_dn7 = assign46530_e79133_d_n7;
        locals.var_t7_dn8 = assign46530_e79133_d_n8;
        locals.var_t7_dn9 = assign46530_e79133_d_n9;
        locals.var_t7_dn10 = assign46530_e79133_d_n10;
        locals.var_t7_dn11 = assign46530_e79133_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign46540_e79143, assign46540_e79143_d_n3, assign46540_e79143_d_n4, assign46540_e79143_d_n5, assign46540_e79143_d_n6, assign46540_e79143_d_n7, assign46540_e79143_d_n8, assign46540_e79143_d_n9, assign46540_e79143_d_n10, assign46540_e79143_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46540_e79138: f64 = (1.0 + locals.var_t7);
        let assign46540_e79140: f64 = (-locals.var_delta_t);
        let assign46540_e79141: f64 = (assign46540_e79138).powf(assign46540_e79140);
        (assign46540_e79141, if (-locals.var_delta_t_dn3) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn3)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn3) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn3 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn4)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn4) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn4 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn5)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn5) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn5 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn6)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn6) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn6 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn7)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn7) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn7 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn8)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn8) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn8 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn9)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn9) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn9 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn10)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn10) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn10 / assign46540_e79138)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign46540_e79140) as f64).is_finite() && ((assign46540_e79140) as f64).fract() == 0.0 { if assign46540_e79140 == 0.0 { 0.0 } else { (assign46540_e79140 * ((assign46540_e79138).powf(assign46540_e79140 - 1.0) * locals.var_t7_dn11)) } } else { (assign46540_e79141 * (((-locals.var_delta_t_dn11) * (assign46540_e79138).ln()) + (assign46540_e79140 * (locals.var_t7_dn11 / assign46540_e79138)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign46540_e79143;
        locals.var_t8_dn3 = assign46540_e79143_d_n3;
        locals.var_t8_dn4 = assign46540_e79143_d_n4;
        locals.var_t8_dn5 = assign46540_e79143_d_n5;
        locals.var_t8_dn6 = assign46540_e79143_d_n6;
        locals.var_t8_dn7 = assign46540_e79143_d_n7;
        locals.var_t8_dn8 = assign46540_e79143_d_n8;
        locals.var_t8_dn9 = assign46540_e79143_d_n9;
        locals.var_t8_dn10 = assign46540_e79143_d_n10;
        locals.var_t8_dn11 = assign46540_e79143_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign46550_e79150, assign46550_e79150_d_n3, assign46550_e79150_d_n4, assign46550_e79150_d_n5, assign46550_e79150_d_n6, assign46550_e79150_d_n7, assign46550_e79150_d_n8, assign46550_e79150_d_n9, assign46550_e79150_d_n10, assign46550_e79150_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46550_e79148: f64 = (locals.var_vds * locals.var_t8);
        (assign46550_e79148, (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), (locals.var_vds * locals.var_t8_dn5), ((locals.var_vds_dn6 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn6)), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), ((locals.var_vds_dn10 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn10)), (locals.var_vds * locals.var_t8_dn11),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11,)
    }
};
        locals.var_vdseff = assign46550_e79150;
        locals.var_vdseff_dn3 = assign46550_e79150_d_n3;
        locals.var_vdseff_dn4 = assign46550_e79150_d_n4;
        locals.var_vdseff_dn5 = assign46550_e79150_d_n5;
        locals.var_vdseff_dn6 = assign46550_e79150_d_n6;
        locals.var_vdseff_dn7 = assign46550_e79150_d_n7;
        locals.var_vdseff_dn8 = assign46550_e79150_d_n8;
        locals.var_vdseff_dn9 = assign46550_e79150_d_n9;
        locals.var_vdseff_dn10 = assign46550_e79150_d_n10;
        locals.var_vdseff_dn11 = assign46550_e79150_d_n11;
        locals.var_vdseff_rv = 0.0;

        let (assign46560_e79159, assign46560_e79159_d_n3, assign46560_e79159_d_n4, assign46560_e79159_d_n5, assign46560_e79159_d_n6, assign46560_e79159_d_n7, assign46560_e79159_d_n8, assign46560_e79159_d_n9, assign46560_e79159_d_n10, assign46560_e79159_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46560_e79155: f64 = (locals.var_vdseff + locals.var_vs);
        let assign46560_e79157: f64 = (assign46560_e79155 * locals.var_inv_nvt);
        (assign46560_e79157, ((locals.var_vdseff_dn3 * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn3)), ((locals.var_vdseff_dn4 * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn4)), ((locals.var_vdseff_dn5 * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn5)), (((locals.var_vdseff_dn6 + locals.var_vs_dn6) * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn6)), (((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn7)), ((locals.var_vdseff_dn8 * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn8)), ((locals.var_vdseff_dn9 * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn9)), (((locals.var_vdseff_dn10 + locals.var_vs_dn10) * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn10)), ((locals.var_vdseff_dn11 * locals.var_inv_nvt) + (assign46560_e79155 * locals.var_inv_nvt_dn11)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11,)
    }
};
        locals.var_vdeff = assign46560_e79159;
        locals.var_vdeff_dn3 = assign46560_e79159_d_n3;
        locals.var_vdeff_dn4 = assign46560_e79159_d_n4;
        locals.var_vdeff_dn5 = assign46560_e79159_d_n5;
        locals.var_vdeff_dn6 = assign46560_e79159_d_n6;
        locals.var_vdeff_dn7 = assign46560_e79159_d_n7;
        locals.var_vdeff_dn8 = assign46560_e79159_d_n8;
        locals.var_vdeff_dn9 = assign46560_e79159_d_n9;
        locals.var_vdeff_dn10 = assign46560_e79159_d_n10;
        locals.var_vdeff_dn11 = assign46560_e79159_d_n11;
        locals.var_vdeff_rv = 0.0;

        let (assign46570_e79183, assign46570_e79183_d_n3, assign46570_e79183_d_n4, assign46570_e79183_d_n5, assign46570_e79183_d_n6, assign46570_e79183_d_n7, assign46570_e79183_d_n8, assign46570_e79183_d_n9, assign46570_e79183_d_n10, assign46570_e79183_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46570_e79165: f64 = (locals.var_psip + 1.0);
        let assign46570_e79168: f64 = (locals.var_psip - 1.0);
        let assign46570_e79171: f64 = (locals.var_psip - 1.0);
        let assign46570_e79172: f64 = (assign46570_e79168 * assign46570_e79171);
        let assign46570_e79175: f64 = (0.25 * 2.0);
        let assign46570_e79177: f64 = (assign46570_e79175 * 2.0);
        let assign46570_e79178: f64 = (assign46570_e79172 + assign46570_e79177);
        let assign46570_e79179: f64 = (assign46570_e79178).sqrt();
        let assign46570_e79180: f64 = (assign46570_e79165 + assign46570_e79179);
        let assign46570_e79181: f64 = (0.5 * assign46570_e79180);
        (assign46570_e79181, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn3)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn4)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn5)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn6)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn7)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn8)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn9)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn10)) / (2.0 * assign46570_e79179)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign46570_e79171) + (assign46570_e79168 * locals.var_psip_dn11)) / (2.0 * assign46570_e79179)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign46570_e79183;
        locals.var_t8_dn3 = assign46570_e79183_d_n3;
        locals.var_t8_dn4 = assign46570_e79183_d_n4;
        locals.var_t8_dn5 = assign46570_e79183_d_n5;
        locals.var_t8_dn6 = assign46570_e79183_d_n6;
        locals.var_t8_dn7 = assign46570_e79183_d_n7;
        locals.var_t8_dn8 = assign46570_e79183_d_n8;
        locals.var_t8_dn9 = assign46570_e79183_d_n9;
        locals.var_t8_dn10 = assign46570_e79183_d_n10;
        locals.var_t8_dn11 = assign46570_e79183_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign46580_e79189, assign46580_e79189_d_n3, assign46580_e79189_d_n4, assign46580_e79189_d_n5, assign46580_e79189_d_n6, assign46580_e79189_d_n7, assign46580_e79189_d_n8, assign46580_e79189_d_n9, assign46580_e79189_d_n10, assign46580_e79189_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46580_e79187: f64 = (locals.var_t8).sqrt();
        (assign46580_e79187, (locals.var_t8_dn3 / (2.0 * assign46580_e79187)), (locals.var_t8_dn4 / (2.0 * assign46580_e79187)), (locals.var_t8_dn5 / (2.0 * assign46580_e79187)), (locals.var_t8_dn6 / (2.0 * assign46580_e79187)), (locals.var_t8_dn7 / (2.0 * assign46580_e79187)), (locals.var_t8_dn8 / (2.0 * assign46580_e79187)), (locals.var_t8_dn9 / (2.0 * assign46580_e79187)), (locals.var_t8_dn10 / (2.0 * assign46580_e79187)), (locals.var_t8_dn11 / (2.0 * assign46580_e79187)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign46580_e79189;
        locals.var_sqrtpsip_dn3 = assign46580_e79189_d_n3;
        locals.var_sqrtpsip_dn4 = assign46580_e79189_d_n4;
        locals.var_sqrtpsip_dn5 = assign46580_e79189_d_n5;
        locals.var_sqrtpsip_dn6 = assign46580_e79189_d_n6;
        locals.var_sqrtpsip_dn7 = assign46580_e79189_d_n7;
        locals.var_sqrtpsip_dn8 = assign46580_e79189_d_n8;
        locals.var_sqrtpsip_dn9 = assign46580_e79189_d_n9;
        locals.var_sqrtpsip_dn10 = assign46580_e79189_d_n10;
        locals.var_sqrtpsip_dn11 = assign46580_e79189_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign46590_e79202, assign46590_e79202_d_n3, assign46590_e79202_d_n4, assign46590_e79202_d_n5, assign46590_e79202_d_n6, assign46590_e79202_d_n7, assign46590_e79202_d_n8, assign46590_e79202_d_n9, assign46590_e79202_d_n10, assign46590_e79202_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46590_e79196: f64 = (2.0 * locals.var_sqrtpsip);
        let assign46590_e79197: f64 = (locals.var_gam / assign46590_e79196);
        let assign46590_e79198: f64 = (1.0 + assign46590_e79197);
        let assign46590_e79200: f64 = (assign46590_e79198 / locals.var_gam);
        (assign46590_e79200, ((((((locals.var_gam_dn3 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn3))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn3)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn4 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn4))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn4)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn5 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn5))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn5)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn6 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn6))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn6)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn7 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn7))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn7)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn8 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn8))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn8)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn9 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn9))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn9)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn10 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn10))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn10)) / (locals.var_gam * locals.var_gam)), ((((((locals.var_gam_dn11 * assign46590_e79196) - (locals.var_gam * (2.0 * locals.var_sqrtpsip_dn11))) / (assign46590_e79196 * assign46590_e79196)) * locals.var_gam) - (assign46590_e79198 * locals.var_gam_dn11)) / (locals.var_gam * locals.var_gam)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign46590_e79202;
        locals.var_t0_dn3 = assign46590_e79202_d_n3;
        locals.var_t0_dn4 = assign46590_e79202_d_n4;
        locals.var_t0_dn5 = assign46590_e79202_d_n5;
        locals.var_t0_dn6 = assign46590_e79202_d_n6;
        locals.var_t0_dn7 = assign46590_e79202_d_n7;
        locals.var_t0_dn8 = assign46590_e79202_d_n8;
        locals.var_t0_dn9 = assign46590_e79202_d_n9;
        locals.var_t0_dn10 = assign46590_e79202_d_n10;
        locals.var_t0_dn11 = assign46590_e79202_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign46600_e79213, assign46600_e79213_d_n3, assign46600_e79213_d_n4, assign46600_e79213_d_n5, assign46600_e79213_d_n6, assign46600_e79213_d_n7, assign46600_e79213_d_n8, assign46600_e79213_d_n9, assign46600_e79213_d_n10, assign46600_e79213_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46600_e79208: f64 = (2.0 * locals.var_phib_n);
        let assign46600_e79209: f64 = (locals.var_psip - assign46600_e79208);
        let assign46600_e79211: f64 = (assign46600_e79209 - locals.var_vdeff);
        (assign46600_e79211, ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_dn11)) - locals.var_vdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign46600_e79213;
        locals.var_t1_dn3 = assign46600_e79213_d_n3;
        locals.var_t1_dn4 = assign46600_e79213_d_n4;
        locals.var_t1_dn5 = assign46600_e79213_d_n5;
        locals.var_t1_dn6 = assign46600_e79213_d_n6;
        locals.var_t1_dn7 = assign46600_e79213_d_n7;
        locals.var_t1_dn8 = assign46600_e79213_d_n8;
        locals.var_t1_dn9 = assign46600_e79213_d_n9;
        locals.var_t1_dn10 = assign46600_e79213_d_n10;
        locals.var_t1_dn11 = assign46600_e79213_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign46610_e79227, assign46610_e79227_d_n3, assign46610_e79227_d_n4, assign46610_e79227_d_n5, assign46610_e79227_d_n6, assign46610_e79227_d_n7, assign46610_e79227_d_n8, assign46610_e79227_d_n9, assign46610_e79227_d_n10, assign46610_e79227_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46610_e79219: f64 = (4.0 * locals.var_t0);
        let assign46610_e79221: f64 = (assign46610_e79219 * locals.var_sqrtpsip);
        let assign46610_e79223: f64 = (assign46610_e79221).max(1e-38);
        let assign46610_e79224: f64 = (assign46610_e79223).ln();
        let assign46610_e79225: f64 = (locals.var_t1 - assign46610_e79224);
        (assign46610_e79225, (locals.var_t1_dn3 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn4 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn5 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn6 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn7 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn8 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn9 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn10 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign46610_e79223)), (locals.var_t1_dn11 - (if assign46610_e79221 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign46610_e79219 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign46610_e79223)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46610_e79227;
        locals.var_t2_dn3 = assign46610_e79227_d_n3;
        locals.var_t2_dn4 = assign46610_e79227_d_n4;
        locals.var_t2_dn5 = assign46610_e79227_d_n5;
        locals.var_t2_dn6 = assign46610_e79227_d_n6;
        locals.var_t2_dn7 = assign46610_e79227_d_n7;
        locals.var_t2_dn8 = assign46610_e79227_d_n8;
        locals.var_t2_dn9 = assign46610_e79227_d_n9;
        locals.var_t2_dn10 = assign46610_e79227_d_n10;
        locals.var_t2_dn11 = assign46610_e79227_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign46620_e79245, assign46620_e79245_d_n3, assign46620_e79245_d_n4, assign46620_e79245_d_n5, assign46620_e79245_d_n6, assign46620_e79245_d_n7, assign46620_e79245_d_n8, assign46620_e79245_d_n9, assign46620_e79245_d_n10, assign46620_e79245_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46620_e79233: f64 = (locals.var_t2 - 0.201491);
        let assign46620_e79237: f64 = (locals.var_t2 + 0.402982);
        let assign46620_e79238: f64 = (locals.var_t2 * assign46620_e79237);
        let assign46620_e79240: f64 = (assign46620_e79238 + 2.446562);
        let assign46620_e79241: f64 = (assign46620_e79240).sqrt();
        let assign46620_e79242: f64 = (assign46620_e79233 - assign46620_e79241);
        let assign46620_e79243: f64 = (0.5 * assign46620_e79242);
        (assign46620_e79243, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign46620_e79241)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign46620_e79237) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign46620_e79241)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign46620_e79245;
        locals.var_t8_dn3 = assign46620_e79245_d_n3;
        locals.var_t8_dn4 = assign46620_e79245_d_n4;
        locals.var_t8_dn5 = assign46620_e79245_d_n5;
        locals.var_t8_dn6 = assign46620_e79245_d_n6;
        locals.var_t8_dn7 = assign46620_e79245_d_n7;
        locals.var_t8_dn8 = assign46620_e79245_d_n8;
        locals.var_t8_dn9 = assign46620_e79245_d_n9;
        locals.var_t8_dn10 = assign46620_e79245_d_n10;
        locals.var_t8_dn11 = assign46620_e79245_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign46630_e79250, assign46630_e79250_d_n3, assign46630_e79250_d_n4, assign46630_e79250_d_n5, assign46630_e79250_d_n6, assign46630_e79250_d_n7, assign46630_e79250_d_n8, assign46630_e79250_d_n9, assign46630_e79250_d_n10, assign46630_e79250_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign46630_e79250;
        locals.var_sqrtpsisa_dn3 = assign46630_e79250_d_n3;
        locals.var_sqrtpsisa_dn4 = assign46630_e79250_d_n4;
        locals.var_sqrtpsisa_dn5 = assign46630_e79250_d_n5;
        locals.var_sqrtpsisa_dn6 = assign46630_e79250_d_n6;
        locals.var_sqrtpsisa_dn7 = assign46630_e79250_d_n7;
        locals.var_sqrtpsisa_dn8 = assign46630_e79250_d_n8;
        locals.var_sqrtpsisa_dn9 = assign46630_e79250_d_n9;
        locals.var_sqrtpsisa_dn10 = assign46630_e79250_d_n10;
        locals.var_sqrtpsisa_dn11 = assign46630_e79250_d_n11;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign46640_e79253: f64 = (-68.0);
        let assign46640_e79254: f64 = if locals.var_t8 <= assign46640_e79253 { 1.0 } else { 0.0 };
        locals.var_guard725 = assign46640_e79254;
        locals.var_guard725_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_162(
        locals: &mut StampLocals,
    ) {
        let (assign46650_e79262, assign46650_e79262_d_n3, assign46650_e79262_d_n4, assign46650_e79262_d_n5, assign46650_e79262_d_n6, assign46650_e79262_d_n7, assign46650_e79262_d_n8, assign46650_e79262_d_n9, assign46650_e79262_d_n10, assign46650_e79262_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) {
        let assign46650_e79260: f64 = (-100.0);
        (assign46650_e79260, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46650_e79262;
        locals.var_t4_dn3 = assign46650_e79262_d_n3;
        locals.var_t4_dn4 = assign46650_e79262_d_n4;
        locals.var_t4_dn5 = assign46650_e79262_d_n5;
        locals.var_t4_dn6 = assign46650_e79262_d_n6;
        locals.var_t4_dn7 = assign46650_e79262_d_n7;
        locals.var_t4_dn8 = assign46650_e79262_d_n8;
        locals.var_t4_dn9 = assign46650_e79262_d_n9;
        locals.var_t4_dn10 = assign46650_e79262_d_n10;
        locals.var_t4_dn11 = assign46650_e79262_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign46660_e79269, assign46660_e79269_d_n3, assign46660_e79269_d_n4, assign46660_e79269_d_n5, assign46660_e79269_d_n6, assign46660_e79269_d_n7, assign46660_e79269_d_n8, assign46660_e79269_d_n9, assign46660_e79269_d_n10, assign46660_e79269_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46660_e79269;
        locals.var_t5_dn3 = assign46660_e79269_d_n3;
        locals.var_t5_dn4 = assign46660_e79269_d_n4;
        locals.var_t5_dn5 = assign46660_e79269_d_n5;
        locals.var_t5_dn6 = assign46660_e79269_d_n6;
        locals.var_t5_dn7 = assign46660_e79269_d_n7;
        locals.var_t5_dn8 = assign46660_e79269_d_n8;
        locals.var_t5_dn9 = assign46660_e79269_d_n9;
        locals.var_t5_dn10 = assign46660_e79269_d_n10;
        locals.var_t5_dn11 = assign46660_e79269_d_n11;
        locals.var_t5_rv = 0.0;

        let assign46670_e79274: f64 = (0.5 * locals.var_t5);
        let assign46670_e79275: f64 = (locals.var_t4 - assign46670_e79274);
        let assign46670_e79276: f64 = if locals.var_t8 < assign46670_e79275 { 1.0 } else { 0.0 };
        locals.var_guard726 = assign46670_e79276;
        locals.var_guard726_rv = 0.0;

        let (assign46680_e79286, assign46680_e79286_d_n3, assign46680_e79286_d_n4, assign46680_e79286_d_n5, assign46680_e79286_d_n6, assign46680_e79286_d_n7, assign46680_e79286_d_n8, assign46680_e79286_d_n9, assign46680_e79286_d_n10, assign46680_e79286_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) && (locals.var_guard726 != 0.0)) {
        let assign46680_e79284: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign46680_e79284, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46680_e79286;
        locals.var_t3_dn3 = assign46680_e79286_d_n3;
        locals.var_t3_dn4 = assign46680_e79286_d_n4;
        locals.var_t3_dn5 = assign46680_e79286_d_n5;
        locals.var_t3_dn6 = assign46680_e79286_d_n6;
        locals.var_t3_dn7 = assign46680_e79286_d_n7;
        locals.var_t3_dn8 = assign46680_e79286_d_n8;
        locals.var_t3_dn9 = assign46680_e79286_d_n9;
        locals.var_t3_dn10 = assign46680_e79286_d_n10;
        locals.var_t3_dn11 = assign46680_e79286_d_n11;
        locals.var_t3_rv = 0.0;

        let assign46690_e79291: f64 = (0.5 * locals.var_t5);
        let assign46690_e79292: f64 = (locals.var_t4 + assign46690_e79291);
        let assign46690_e79293: f64 = if locals.var_t8 > assign46690_e79292 { 1.0 } else { 0.0 };
        locals.var_guard727 = assign46690_e79293;
        locals.var_guard727_rv = 0.0;

        let (assign46700_e79306, assign46700_e79306_d_n3, assign46700_e79306_d_n4, assign46700_e79306_d_n5, assign46700_e79306_d_n6, assign46700_e79306_d_n7, assign46700_e79306_d_n8, assign46700_e79306_d_n9, assign46700_e79306_d_n10, assign46700_e79306_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 != 0.0)) {
        let assign46700_e79304: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign46700_e79304, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46700_e79306;
        locals.var_t3_dn3 = assign46700_e79306_d_n3;
        locals.var_t3_dn4 = assign46700_e79306_d_n4;
        locals.var_t3_dn5 = assign46700_e79306_d_n5;
        locals.var_t3_dn6 = assign46700_e79306_d_n6;
        locals.var_t3_dn7 = assign46700_e79306_d_n7;
        locals.var_t3_dn8 = assign46700_e79306_d_n8;
        locals.var_t3_dn9 = assign46700_e79306_d_n9;
        locals.var_t3_dn10 = assign46700_e79306_d_n10;
        locals.var_t3_dn11 = assign46700_e79306_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46710_e79323, assign46710_e79323_d_n3, assign46710_e79323_d_n4, assign46710_e79323_d_n5, assign46710_e79323_d_n6, assign46710_e79323_d_n7, assign46710_e79323_d_n8, assign46710_e79323_d_n9, assign46710_e79323_d_n10, assign46710_e79323_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign46710_e79319: f64 = (locals.var_t8 - locals.var_t4);
        let assign46710_e79321: f64 = (assign46710_e79319 / locals.var_t5);
        (assign46710_e79321, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign46710_e79319 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46710_e79323;
        locals.var_t2_dn3 = assign46710_e79323_d_n3;
        locals.var_t2_dn4 = assign46710_e79323_d_n4;
        locals.var_t2_dn5 = assign46710_e79323_d_n5;
        locals.var_t2_dn6 = assign46710_e79323_d_n6;
        locals.var_t2_dn7 = assign46710_e79323_d_n7;
        locals.var_t2_dn8 = assign46710_e79323_d_n8;
        locals.var_t2_dn9 = assign46710_e79323_d_n9;
        locals.var_t2_dn10 = assign46710_e79323_d_n10;
        locals.var_t2_dn11 = assign46710_e79323_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign46720_e79338, assign46720_e79338_d_n3, assign46720_e79338_d_n4, assign46720_e79338_d_n5, assign46720_e79338_d_n6, assign46720_e79338_d_n7, assign46720_e79338_d_n8, assign46720_e79338_d_n9, assign46720_e79338_d_n10, assign46720_e79338_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign46720_e79336: f64 = (locals.var_t2 * locals.var_t2);
        (assign46720_e79336, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign46720_e79338;
        locals.var_t6_dn3 = assign46720_e79338_d_n3;
        locals.var_t6_dn4 = assign46720_e79338_d_n4;
        locals.var_t6_dn5 = assign46720_e79338_d_n5;
        locals.var_t6_dn6 = assign46720_e79338_d_n6;
        locals.var_t6_dn7 = assign46720_e79338_d_n7;
        locals.var_t6_dn8 = assign46720_e79338_d_n8;
        locals.var_t6_dn9 = assign46720_e79338_d_n9;
        locals.var_t6_dn10 = assign46720_e79338_d_n10;
        locals.var_t6_dn11 = assign46720_e79338_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign46730_e79374, assign46730_e79374_d_n3, assign46730_e79374_d_n4, assign46730_e79374_d_n5, assign46730_e79374_d_n6, assign46730_e79374_d_n7, assign46730_e79374_d_n8, assign46730_e79374_d_n9, assign46730_e79374_d_n10, assign46730_e79374_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) && (locals.var_guard726 == 0.0)) && (locals.var_guard727 == 0.0)) {
        let assign46730_e79353: f64 = (5.0 / 64.0);
        let assign46730_e79356: f64 = (0.5 * locals.var_t2);
        let assign46730_e79357: f64 = (assign46730_e79353 + assign46730_e79356);
        let assign46730_e79361: f64 = (15.0 / 16.0);
        let assign46730_e79365: f64 = (1.25 - locals.var_t6);
        let assign46730_e79366: f64 = (locals.var_t6 * assign46730_e79365);
        let assign46730_e79367: f64 = (assign46730_e79361 - assign46730_e79366);
        let assign46730_e79368: f64 = (locals.var_t6 * assign46730_e79367);
        let assign46730_e79369: f64 = (assign46730_e79357 + assign46730_e79368);
        let assign46730_e79370: f64 = (locals.var_t5 * assign46730_e79369);
        let assign46730_e79371: f64 = (locals.var_t4 + assign46730_e79370);
        let assign46730_e79372: f64 = { let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign46730_e79372, ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign46730_e79371; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign46730_e79369) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign46730_e79367) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign46730_e79365) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46730_e79374;
        locals.var_t3_dn3 = assign46730_e79374_d_n3;
        locals.var_t3_dn4 = assign46730_e79374_d_n4;
        locals.var_t3_dn5 = assign46730_e79374_d_n5;
        locals.var_t3_dn6 = assign46730_e79374_d_n6;
        locals.var_t3_dn7 = assign46730_e79374_d_n7;
        locals.var_t3_dn8 = assign46730_e79374_d_n8;
        locals.var_t3_dn9 = assign46730_e79374_d_n9;
        locals.var_t3_dn10 = assign46730_e79374_d_n10;
        locals.var_t3_dn11 = assign46730_e79374_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46740_e79404, assign46740_e79404_d_n3, assign46740_e79404_d_n4, assign46740_e79404_d_n5, assign46740_e79404_d_n6, assign46740_e79404_d_n7, assign46740_e79404_d_n8, assign46740_e79404_d_n9, assign46740_e79404_d_n10, assign46740_e79404_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 != 0.0)) {
        let assign46740_e79382: f64 = (1.0 + locals.var_t1);
        let assign46740_e79384: f64 = (assign46740_e79382 - locals.var_t8);
        let assign46740_e79387: f64 = (2.0 * locals.var_t0);
        let assign46740_e79390: f64 = (locals.var_t3 * 2.0);
        let assign46740_e79392: f64 = (assign46740_e79390 * locals.var_t0);
        let assign46740_e79395: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign46740_e79396: f64 = (assign46740_e79392 + assign46740_e79395);
        let assign46740_e79397: f64 = (assign46740_e79387 * assign46740_e79396);
        let assign46740_e79399: f64 = (assign46740_e79397).max(1e-38);
        let assign46740_e79400: f64 = (assign46740_e79399).ln();
        let assign46740_e79401: f64 = (assign46740_e79384 - assign46740_e79400);
        let assign46740_e79402: f64 = (locals.var_t3 * assign46740_e79401);
        (assign46740_e79402, ((locals.var_t3_dn3 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn4 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn5 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn6 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn7 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn8 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn9 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn10 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign46740_e79399)))), ((locals.var_t3_dn11 * assign46740_e79401) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign46740_e79397 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign46740_e79396) + (assign46740_e79387 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign46740_e79390 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign46740_e79399)))),)
    } else {
        (locals.var_qdeff, locals.var_qdeff_dn3, locals.var_qdeff_dn4, locals.var_qdeff_dn5, locals.var_qdeff_dn6, locals.var_qdeff_dn7, locals.var_qdeff_dn8, locals.var_qdeff_dn9, locals.var_qdeff_dn10, locals.var_qdeff_dn11,)
    }
};
        locals.var_qdeff = assign46740_e79404;
        locals.var_qdeff_dn3 = assign46740_e79404_d_n3;
        locals.var_qdeff_dn4 = assign46740_e79404_d_n4;
        locals.var_qdeff_dn5 = assign46740_e79404_d_n5;
        locals.var_qdeff_dn6 = assign46740_e79404_d_n6;
        locals.var_qdeff_dn7 = assign46740_e79404_d_n7;
        locals.var_qdeff_dn8 = assign46740_e79404_d_n8;
        locals.var_qdeff_dn9 = assign46740_e79404_d_n9;
        locals.var_qdeff_dn10 = assign46740_e79404_d_n10;
        locals.var_qdeff_dn11 = assign46740_e79404_d_n11;
        locals.var_qdeff_rv = 0.0;

        let (assign46750_e79413, assign46750_e79413_d_n3, assign46750_e79413_d_n4, assign46750_e79413_d_n5, assign46750_e79413_d_n6, assign46750_e79413_d_n7, assign46750_e79413_d_n8, assign46750_e79413_d_n9, assign46750_e79413_d_n10, assign46750_e79413_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46750_e79411: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign46750_e79411, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46750_e79413;
        locals.var_t3_dn3 = assign46750_e79413_d_n3;
        locals.var_t3_dn4 = assign46750_e79413_d_n4;
        locals.var_t3_dn5 = assign46750_e79413_d_n5;
        locals.var_t3_dn6 = assign46750_e79413_d_n6;
        locals.var_t3_dn7 = assign46750_e79413_d_n7;
        locals.var_t3_dn8 = assign46750_e79413_d_n8;
        locals.var_t3_dn9 = assign46750_e79413_d_n9;
        locals.var_t3_dn10 = assign46750_e79413_d_n10;
        locals.var_t3_dn11 = assign46750_e79413_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46760_e79423, assign46760_e79423_d_n3, assign46760_e79423_d_n4, assign46760_e79423_d_n5, assign46760_e79423_d_n6, assign46760_e79423_d_n7, assign46760_e79423_d_n8, assign46760_e79423_d_n9, assign46760_e79423_d_n10, assign46760_e79423_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46760_e79421: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign46760_e79421, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign46760_e79423;
        locals.var_sqrtpsisainv_dn3 = assign46760_e79423_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign46760_e79423_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign46760_e79423_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign46760_e79423_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign46760_e79423_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign46760_e79423_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign46760_e79423_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign46760_e79423_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign46760_e79423_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign46770_e79454, assign46770_e79454_d_n3, assign46770_e79454_d_n4, assign46770_e79454_d_n5, assign46770_e79454_d_n6, assign46770_e79454_d_n7, assign46770_e79454_d_n8, assign46770_e79454_d_n9, assign46770_e79454_d_n10, assign46770_e79454_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46770_e79431: f64 = (2.0 * locals.var_t3);
        let assign46770_e79434: f64 = (locals.var_t3 * 2.0);
        let assign46770_e79436: f64 = (assign46770_e79434 * locals.var_t0);
        let assign46770_e79439: f64 = (locals.var_t3 * 2.0);
        let assign46770_e79441: f64 = (assign46770_e79439 * locals.var_t0);
        let assign46770_e79444: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign46770_e79445: f64 = (assign46770_e79441 + assign46770_e79444);
        let assign46770_e79446: f64 = (assign46770_e79436 * assign46770_e79445);
        let assign46770_e79448: f64 = (assign46770_e79446).max(1e-38);
        let assign46770_e79449: f64 = (assign46770_e79448).ln();
        let assign46770_e79450: f64 = (assign46770_e79431 + assign46770_e79449);
        let assign46770_e79452: f64 = (assign46770_e79450 - locals.var_t1);
        (assign46770_e79452, (((2.0 * locals.var_t3_dn3) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn3)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn4)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn5)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn6)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn7)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn8)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn9)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn10)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign46770_e79446 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign46770_e79434 * locals.var_t0_dn11)) * assign46770_e79445) + (assign46770_e79436 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign46770_e79439 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign46770_e79448)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46770_e79454;
        locals.var_t4_dn3 = assign46770_e79454_d_n3;
        locals.var_t4_dn4 = assign46770_e79454_d_n4;
        locals.var_t4_dn5 = assign46770_e79454_d_n5;
        locals.var_t4_dn6 = assign46770_e79454_d_n6;
        locals.var_t4_dn7 = assign46770_e79454_d_n7;
        locals.var_t4_dn8 = assign46770_e79454_d_n8;
        locals.var_t4_dn9 = assign46770_e79454_d_n9;
        locals.var_t4_dn10 = assign46770_e79454_d_n10;
        locals.var_t4_dn11 = assign46770_e79454_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign46780_e79476, assign46780_e79476_d_n3, assign46780_e79476_d_n4, assign46780_e79476_d_n5, assign46780_e79476_d_n6, assign46780_e79476_d_n7, assign46780_e79476_d_n8, assign46780_e79476_d_n9, assign46780_e79476_d_n10, assign46780_e79476_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46780_e79463: f64 = (1.0 / locals.var_t3);
        let assign46780_e79464: f64 = (2.0 + assign46780_e79463);
        let assign46780_e79467: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign46780_e79470: f64 = (locals.var_t0 * locals.var_t3);
        let assign46780_e79472: f64 = (assign46780_e79470 + locals.var_sqrtpsisa);
        let assign46780_e79473: f64 = (assign46780_e79467 / assign46780_e79472);
        let assign46780_e79474: f64 = (assign46780_e79464 + assign46780_e79473);
        (assign46780_e79474, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign46780_e79472 * assign46780_e79472))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign46780_e79472) - (assign46780_e79467 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign46780_e79472 * assign46780_e79472))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46780_e79476;
        locals.var_t5_dn3 = assign46780_e79476_d_n3;
        locals.var_t5_dn4 = assign46780_e79476_d_n4;
        locals.var_t5_dn5 = assign46780_e79476_d_n5;
        locals.var_t5_dn6 = assign46780_e79476_d_n6;
        locals.var_t5_dn7 = assign46780_e79476_d_n7;
        locals.var_t5_dn8 = assign46780_e79476_d_n8;
        locals.var_t5_dn9 = assign46780_e79476_d_n9;
        locals.var_t5_dn10 = assign46780_e79476_d_n10;
        locals.var_t5_dn11 = assign46780_e79476_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46790_e79488, assign46790_e79488_d_n3, assign46790_e79488_d_n4, assign46790_e79488_d_n5, assign46790_e79488_d_n6, assign46790_e79488_d_n7, assign46790_e79488_d_n8, assign46790_e79488_d_n9, assign46790_e79488_d_n10, assign46790_e79488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46790_e79485: f64 = (locals.var_t4 / locals.var_t5);
        let assign46790_e79486: f64 = (locals.var_t3 - assign46790_e79485);
        (assign46790_e79486, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46790_e79488;
        locals.var_t3_dn3 = assign46790_e79488_d_n3;
        locals.var_t3_dn4 = assign46790_e79488_d_n4;
        locals.var_t3_dn5 = assign46790_e79488_d_n5;
        locals.var_t3_dn6 = assign46790_e79488_d_n6;
        locals.var_t3_dn7 = assign46790_e79488_d_n7;
        locals.var_t3_dn8 = assign46790_e79488_d_n8;
        locals.var_t3_dn9 = assign46790_e79488_d_n9;
        locals.var_t3_dn10 = assign46790_e79488_d_n10;
        locals.var_t3_dn11 = assign46790_e79488_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46800_e79519, assign46800_e79519_d_n3, assign46800_e79519_d_n4, assign46800_e79519_d_n5, assign46800_e79519_d_n6, assign46800_e79519_d_n7, assign46800_e79519_d_n8, assign46800_e79519_d_n9, assign46800_e79519_d_n10, assign46800_e79519_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46800_e79496: f64 = (2.0 * locals.var_t3);
        let assign46800_e79499: f64 = (locals.var_t3 * 2.0);
        let assign46800_e79501: f64 = (assign46800_e79499 * locals.var_t0);
        let assign46800_e79504: f64 = (locals.var_t3 * 2.0);
        let assign46800_e79506: f64 = (assign46800_e79504 * locals.var_t0);
        let assign46800_e79509: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign46800_e79510: f64 = (assign46800_e79506 + assign46800_e79509);
        let assign46800_e79511: f64 = (assign46800_e79501 * assign46800_e79510);
        let assign46800_e79513: f64 = (assign46800_e79511).max(1e-38);
        let assign46800_e79514: f64 = (assign46800_e79513).ln();
        let assign46800_e79515: f64 = (assign46800_e79496 + assign46800_e79514);
        let assign46800_e79517: f64 = (assign46800_e79515 - locals.var_t1);
        (assign46800_e79517, (((2.0 * locals.var_t3_dn3) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn3)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn4)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn5)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn6)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn7)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn8)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn9)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn10)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign46800_e79511 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign46800_e79499 * locals.var_t0_dn11)) * assign46800_e79510) + (assign46800_e79501 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign46800_e79504 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign46800_e79513)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign46800_e79519;
        locals.var_t4_dn3 = assign46800_e79519_d_n3;
        locals.var_t4_dn4 = assign46800_e79519_d_n4;
        locals.var_t4_dn5 = assign46800_e79519_d_n5;
        locals.var_t4_dn6 = assign46800_e79519_d_n6;
        locals.var_t4_dn7 = assign46800_e79519_d_n7;
        locals.var_t4_dn8 = assign46800_e79519_d_n8;
        locals.var_t4_dn9 = assign46800_e79519_d_n9;
        locals.var_t4_dn10 = assign46800_e79519_d_n10;
        locals.var_t4_dn11 = assign46800_e79519_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign46810_e79541, assign46810_e79541_d_n3, assign46810_e79541_d_n4, assign46810_e79541_d_n5, assign46810_e79541_d_n6, assign46810_e79541_d_n7, assign46810_e79541_d_n8, assign46810_e79541_d_n9, assign46810_e79541_d_n10, assign46810_e79541_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46810_e79528: f64 = (1.0 / locals.var_t3);
        let assign46810_e79529: f64 = (2.0 + assign46810_e79528);
        let assign46810_e79532: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign46810_e79535: f64 = (locals.var_t0 * locals.var_t3);
        let assign46810_e79537: f64 = (assign46810_e79535 + locals.var_sqrtpsisa);
        let assign46810_e79538: f64 = (assign46810_e79532 / assign46810_e79537);
        let assign46810_e79539: f64 = (assign46810_e79529 + assign46810_e79538);
        (assign46810_e79539, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign46810_e79537 * assign46810_e79537))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign46810_e79537) - (assign46810_e79532 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign46810_e79537 * assign46810_e79537))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign46810_e79541;
        locals.var_t5_dn3 = assign46810_e79541_d_n3;
        locals.var_t5_dn4 = assign46810_e79541_d_n4;
        locals.var_t5_dn5 = assign46810_e79541_d_n5;
        locals.var_t5_dn6 = assign46810_e79541_d_n6;
        locals.var_t5_dn7 = assign46810_e79541_d_n7;
        locals.var_t5_dn8 = assign46810_e79541_d_n8;
        locals.var_t5_dn9 = assign46810_e79541_d_n9;
        locals.var_t5_dn10 = assign46810_e79541_d_n10;
        locals.var_t5_dn11 = assign46810_e79541_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign46820_e79567, assign46820_e79567_d_n3, assign46820_e79567_d_n4, assign46820_e79567_d_n5, assign46820_e79567_d_n6, assign46820_e79567_d_n7, assign46820_e79567_d_n8, assign46820_e79567_d_n9, assign46820_e79567_d_n10, assign46820_e79567_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46820_e79549: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign46820_e79552: f64 = (locals.var_t0 * locals.var_t3);
        let assign46820_e79554: f64 = (assign46820_e79552 + locals.var_sqrtpsisa);
        let assign46820_e79555: f64 = (assign46820_e79549 / assign46820_e79554);
        let assign46820_e79558: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign46820_e79561: f64 = (locals.var_t0 * locals.var_t3);
        let assign46820_e79563: f64 = (assign46820_e79561 + locals.var_sqrtpsisa);
        let assign46820_e79564: f64 = (assign46820_e79558 / assign46820_e79563);
        let assign46820_e79565: f64 = (assign46820_e79555 * assign46820_e79564);
        (assign46820_e79565, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign46820_e79563 * assign46820_e79563)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign46820_e79554) - (assign46820_e79549 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign46820_e79554 * assign46820_e79554)) * assign46820_e79564) + (assign46820_e79555 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign46820_e79563) - (assign46820_e79558 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign46820_e79563 * assign46820_e79563)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign46820_e79567;
        locals.var_t6_dn3 = assign46820_e79567_d_n3;
        locals.var_t6_dn4 = assign46820_e79567_d_n4;
        locals.var_t6_dn5 = assign46820_e79567_d_n5;
        locals.var_t6_dn6 = assign46820_e79567_d_n6;
        locals.var_t6_dn7 = assign46820_e79567_d_n7;
        locals.var_t6_dn8 = assign46820_e79567_d_n8;
        locals.var_t6_dn9 = assign46820_e79567_d_n9;
        locals.var_t6_dn10 = assign46820_e79567_d_n10;
        locals.var_t6_dn11 = assign46820_e79567_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign46830_e79598, assign46830_e79598_d_n3, assign46830_e79598_d_n4, assign46830_e79598_d_n5, assign46830_e79598_d_n6, assign46830_e79598_d_n7, assign46830_e79598_d_n8, assign46830_e79598_d_n9, assign46830_e79598_d_n10, assign46830_e79598_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign46830_e79575: f64 = (1.0 * __rspice_inv_cse_0);
        let assign46830_e79578: f64 = (1.0 * __rspice_inv_cse_0);
        let assign46830_e79579: f64 = (assign46830_e79575 * assign46830_e79578);
        let assign46830_e79580: f64 = (-assign46830_e79579);
        let assign46830_e79584: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign46830_e79586: f64 = (assign46830_e79584 * locals.var_sqrtpsisa);
        let assign46830_e79589: f64 = (locals.var_t0 * locals.var_t3);
        let assign46830_e79591: f64 = (assign46830_e79589 + locals.var_sqrtpsisa);
        let assign46830_e79592: f64 = (assign46830_e79586 * assign46830_e79591);
        let assign46830_e79593: f64 = (1.0 / assign46830_e79592);
        let assign46830_e79594: f64 = (assign46830_e79580 - assign46830_e79593);
        let assign46830_e79596: f64 = (assign46830_e79594 - locals.var_t6);
        (assign46830_e79596, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn3)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn4)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn5)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn6)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn7)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn8)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn9)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn10)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign46830_e79578) + (assign46830_e79575 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign46830_e79584 * locals.var_sqrtpsisa_dn11)) * assign46830_e79591) + (assign46830_e79586 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign46830_e79592 * assign46830_e79592)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign46830_e79598;
        locals.var_t7_dn3 = assign46830_e79598_d_n3;
        locals.var_t7_dn4 = assign46830_e79598_d_n4;
        locals.var_t7_dn5 = assign46830_e79598_d_n5;
        locals.var_t7_dn6 = assign46830_e79598_d_n6;
        locals.var_t7_dn7 = assign46830_e79598_d_n7;
        locals.var_t7_dn8 = assign46830_e79598_d_n8;
        locals.var_t7_dn9 = assign46830_e79598_d_n9;
        locals.var_t7_dn10 = assign46830_e79598_d_n10;
        locals.var_t7_dn11 = assign46830_e79598_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign46840_e79622, assign46840_e79622_d_n3, assign46840_e79622_d_n4, assign46840_e79622_d_n5, assign46840_e79622_d_n6, assign46840_e79622_d_n7, assign46840_e79622_d_n8, assign46840_e79622_d_n9, assign46840_e79622_d_n10, assign46840_e79622_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard725 == 0.0)) {
        let assign46840_e79607: f64 = (locals.var_t4 / locals.var_t5);
        let assign46840_e79611: f64 = (locals.var_t4 * locals.var_t7);
        let assign46840_e79614: f64 = (2.0 * locals.var_t5);
        let assign46840_e79616: f64 = (assign46840_e79614 * locals.var_t5);
        let assign46840_e79617: f64 = (assign46840_e79611 / assign46840_e79616);
        let assign46840_e79618: f64 = (1.0 + assign46840_e79617);
        let assign46840_e79619: f64 = (assign46840_e79607 * assign46840_e79618);
        let assign46840_e79620: f64 = (locals.var_t3 - assign46840_e79619);
        (assign46840_e79620, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn3)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn4)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn5)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn6)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn7)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn8)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn9)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn10)))) / (assign46840_e79616 * assign46840_e79616))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign46840_e79618) + (assign46840_e79607 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign46840_e79616) - (assign46840_e79611 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign46840_e79614 * locals.var_t5_dn11)))) / (assign46840_e79616 * assign46840_e79616))))),)
    } else {
        (locals.var_qdeff, locals.var_qdeff_dn3, locals.var_qdeff_dn4, locals.var_qdeff_dn5, locals.var_qdeff_dn6, locals.var_qdeff_dn7, locals.var_qdeff_dn8, locals.var_qdeff_dn9, locals.var_qdeff_dn10, locals.var_qdeff_dn11,)
    }
};
        locals.var_qdeff = assign46840_e79622;
        locals.var_qdeff_dn3 = assign46840_e79622_d_n3;
        locals.var_qdeff_dn4 = assign46840_e79622_d_n4;
        locals.var_qdeff_dn5 = assign46840_e79622_d_n5;
        locals.var_qdeff_dn6 = assign46840_e79622_d_n6;
        locals.var_qdeff_dn7 = assign46840_e79622_d_n7;
        locals.var_qdeff_dn8 = assign46840_e79622_d_n8;
        locals.var_qdeff_dn9 = assign46840_e79622_d_n9;
        locals.var_qdeff_dn10 = assign46840_e79622_d_n10;
        locals.var_qdeff_dn11 = assign46840_e79622_d_n11;
        locals.var_qdeff_rv = 0.0;

        let (assign46850_e79633, assign46850_e79633_d_n3, assign46850_e79633_d_n4, assign46850_e79633_d_n5, assign46850_e79633_d_n6, assign46850_e79633_d_n7, assign46850_e79633_d_n8, assign46850_e79633_d_n9, assign46850_e79633_d_n10, assign46850_e79633_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46850_e79627: f64 = (locals.var_psip - locals.var_qs_1);
        let assign46850_e79629: f64 = (assign46850_e79627 - locals.var_qdeff);
        let assign46850_e79631: f64 = (assign46850_e79629 - 1.0);
        (assign46850_e79631, ((locals.var_psip_dn3 - locals.var_qs_1_dn3) - locals.var_qdeff_dn3), ((locals.var_psip_dn4 - locals.var_qs_1_dn4) - locals.var_qdeff_dn4), ((locals.var_psip_dn5 - locals.var_qs_1_dn5) - locals.var_qdeff_dn5), ((locals.var_psip_dn6 - locals.var_qs_1_dn6) - locals.var_qdeff_dn6), ((locals.var_psip_dn7 - locals.var_qs_1_dn7) - locals.var_qdeff_dn7), ((locals.var_psip_dn8 - locals.var_qs_1_dn8) - locals.var_qdeff_dn8), ((locals.var_psip_dn9 - locals.var_qs_1_dn9) - locals.var_qdeff_dn9), ((locals.var_psip_dn10 - locals.var_qs_1_dn10) - locals.var_qdeff_dn10), ((locals.var_psip_dn11 - locals.var_qs_1_dn11) - locals.var_qdeff_dn11),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign46850_e79633;
        locals.var_psiavg_dn3 = assign46850_e79633_d_n3;
        locals.var_psiavg_dn4 = assign46850_e79633_d_n4;
        locals.var_psiavg_dn5 = assign46850_e79633_d_n5;
        locals.var_psiavg_dn6 = assign46850_e79633_d_n6;
        locals.var_psiavg_dn7 = assign46850_e79633_d_n7;
        locals.var_psiavg_dn8 = assign46850_e79633_d_n8;
        locals.var_psiavg_dn9 = assign46850_e79633_d_n9;
        locals.var_psiavg_dn10 = assign46850_e79633_d_n10;
        locals.var_psiavg_dn11 = assign46850_e79633_d_n11;
        locals.var_psiavg_rv = 0.0;

        let (assign46860_e79657, assign46860_e79657_d_n3, assign46860_e79657_d_n4, assign46860_e79657_d_n5, assign46860_e79657_d_n6, assign46860_e79657_d_n7, assign46860_e79657_d_n8, assign46860_e79657_d_n9, assign46860_e79657_d_n10, assign46860_e79657_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46860_e79639: f64 = (locals.var_psiavg + 1.0);
        let assign46860_e79642: f64 = (locals.var_psiavg - 1.0);
        let assign46860_e79645: f64 = (locals.var_psiavg - 1.0);
        let assign46860_e79646: f64 = (assign46860_e79642 * assign46860_e79645);
        let assign46860_e79649: f64 = (0.25 * 2.0);
        let assign46860_e79651: f64 = (assign46860_e79649 * 2.0);
        let assign46860_e79652: f64 = (assign46860_e79646 + assign46860_e79651);
        let assign46860_e79653: f64 = (assign46860_e79652).sqrt();
        let assign46860_e79654: f64 = (assign46860_e79639 + assign46860_e79653);
        let assign46860_e79655: f64 = (0.5 * assign46860_e79654);
        (assign46860_e79655, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn3)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn4)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn5)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn6)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn7)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn8)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn9)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn10)) / (2.0 * assign46860_e79653)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign46860_e79645) + (assign46860_e79642 * locals.var_psiavg_dn11)) / (2.0 * assign46860_e79653)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign46860_e79657;
        locals.var_t0_dn3 = assign46860_e79657_d_n3;
        locals.var_t0_dn4 = assign46860_e79657_d_n4;
        locals.var_t0_dn5 = assign46860_e79657_d_n5;
        locals.var_t0_dn6 = assign46860_e79657_d_n6;
        locals.var_t0_dn7 = assign46860_e79657_d_n7;
        locals.var_t0_dn8 = assign46860_e79657_d_n8;
        locals.var_t0_dn9 = assign46860_e79657_d_n9;
        locals.var_t0_dn10 = assign46860_e79657_d_n10;
        locals.var_t0_dn11 = assign46860_e79657_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_163(
        locals: &mut StampLocals,
    ) {
        let (assign46870_e79663, assign46870_e79663_d_n3, assign46870_e79663_d_n4, assign46870_e79663_d_n5, assign46870_e79663_d_n6, assign46870_e79663_d_n7, assign46870_e79663_d_n8, assign46870_e79663_d_n9, assign46870_e79663_d_n10, assign46870_e79663_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46870_e79661: f64 = (locals.var_t0).sqrt();
        (assign46870_e79661, (locals.var_t0_dn3 / (2.0 * assign46870_e79661)), (locals.var_t0_dn4 / (2.0 * assign46870_e79661)), (locals.var_t0_dn5 / (2.0 * assign46870_e79661)), (locals.var_t0_dn6 / (2.0 * assign46870_e79661)), (locals.var_t0_dn7 / (2.0 * assign46870_e79661)), (locals.var_t0_dn8 / (2.0 * assign46870_e79661)), (locals.var_t0_dn9 / (2.0 * assign46870_e79661)), (locals.var_t0_dn10 / (2.0 * assign46870_e79661)), (locals.var_t0_dn11 / (2.0 * assign46870_e79661)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46870_e79663;
        locals.var_t2_dn3 = assign46870_e79663_d_n3;
        locals.var_t2_dn4 = assign46870_e79663_d_n4;
        locals.var_t2_dn5 = assign46870_e79663_d_n5;
        locals.var_t2_dn6 = assign46870_e79663_d_n6;
        locals.var_t2_dn7 = assign46870_e79663_d_n7;
        locals.var_t2_dn8 = assign46870_e79663_d_n8;
        locals.var_t2_dn9 = assign46870_e79663_d_n9;
        locals.var_t2_dn10 = assign46870_e79663_d_n10;
        locals.var_t2_dn11 = assign46870_e79663_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign46880_e79674, assign46880_e79674_d_n3, assign46880_e79674_d_n4, assign46880_e79674_d_n5, assign46880_e79674_d_n6, assign46880_e79674_d_n7, assign46880_e79674_d_n8, assign46880_e79674_d_n9, assign46880_e79674_d_n10, assign46880_e79674_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46880_e79670: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign46880_e79671: f64 = (locals.var_gam / assign46880_e79670);
        let assign46880_e79672: f64 = (1.0 + assign46880_e79671);
        (assign46880_e79672, (((locals.var_gam_dn3 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn4 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn5 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn6 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn7 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn8 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn9 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn10 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign46880_e79670 * assign46880_e79670)), (((locals.var_gam_dn11 * assign46880_e79670) - (locals.var_gam * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign46880_e79670 * assign46880_e79670)),)
    } else {
        (locals.var_nq, locals.var_nq_dn3, locals.var_nq_dn4, locals.var_nq_dn5, locals.var_nq_dn6, locals.var_nq_dn7, locals.var_nq_dn8, locals.var_nq_dn9, locals.var_nq_dn10, locals.var_nq_dn11,)
    }
};
        locals.var_nq = assign46880_e79674;
        locals.var_nq_dn3 = assign46880_e79674_d_n3;
        locals.var_nq_dn4 = assign46880_e79674_d_n4;
        locals.var_nq_dn5 = assign46880_e79674_d_n5;
        locals.var_nq_dn6 = assign46880_e79674_d_n6;
        locals.var_nq_dn7 = assign46880_e79674_d_n7;
        locals.var_nq_dn8 = assign46880_e79674_d_n8;
        locals.var_nq_dn9 = assign46880_e79674_d_n9;
        locals.var_nq_dn10 = assign46880_e79674_d_n10;
        locals.var_nq_dn11 = assign46880_e79674_d_n11;
        locals.var_nq_rv = 0.0;

        let (assign46890_e79685, assign46890_e79685_d_n3, assign46890_e79685_d_n4, assign46890_e79685_d_n5, assign46890_e79685_d_n6, assign46890_e79685_d_n7, assign46890_e79685_d_n8, assign46890_e79685_d_n9, assign46890_e79685_d_n10, assign46890_e79685_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46890_e79679: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign46890_e79682: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign46890_e79683: f64 = (assign46890_e79679 * assign46890_e79682);
        (assign46890_e79683, (((locals.var_qs_1_dn3 - locals.var_qdeff_dn3) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn3 - locals.var_qdeff_dn3))), (((locals.var_qs_1_dn4 - locals.var_qdeff_dn4) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn4 - locals.var_qdeff_dn4))), (((locals.var_qs_1_dn5 - locals.var_qdeff_dn5) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn5 - locals.var_qdeff_dn5))), (((locals.var_qs_1_dn6 - locals.var_qdeff_dn6) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn6 - locals.var_qdeff_dn6))), (((locals.var_qs_1_dn7 - locals.var_qdeff_dn7) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn7 - locals.var_qdeff_dn7))), (((locals.var_qs_1_dn8 - locals.var_qdeff_dn8) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn8 - locals.var_qdeff_dn8))), (((locals.var_qs_1_dn9 - locals.var_qdeff_dn9) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn9 - locals.var_qdeff_dn9))), (((locals.var_qs_1_dn10 - locals.var_qdeff_dn10) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn10 - locals.var_qdeff_dn10))), (((locals.var_qs_1_dn11 - locals.var_qdeff_dn11) * assign46890_e79682) + (assign46890_e79679 * (locals.var_qs_1_dn11 - locals.var_qdeff_dn11))),)
    } else {
        (locals.var_dqsd2, locals.var_dqsd2_dn3, locals.var_dqsd2_dn4, locals.var_dqsd2_dn5, locals.var_dqsd2_dn6, locals.var_dqsd2_dn7, locals.var_dqsd2_dn8, locals.var_dqsd2_dn9, locals.var_dqsd2_dn10, locals.var_dqsd2_dn11,)
    }
};
        locals.var_dqsd2 = assign46890_e79685;
        locals.var_dqsd2_dn3 = assign46890_e79685_d_n3;
        locals.var_dqsd2_dn4 = assign46890_e79685_d_n4;
        locals.var_dqsd2_dn5 = assign46890_e79685_d_n5;
        locals.var_dqsd2_dn6 = assign46890_e79685_d_n6;
        locals.var_dqsd2_dn7 = assign46890_e79685_d_n7;
        locals.var_dqsd2_dn8 = assign46890_e79685_d_n8;
        locals.var_dqsd2_dn9 = assign46890_e79685_d_n9;
        locals.var_dqsd2_dn10 = assign46890_e79685_d_n10;
        locals.var_dqsd2_dn11 = assign46890_e79685_d_n11;
        locals.var_dqsd2_rv = 0.0;

        let (assign46900_e79696, assign46900_e79696_d_n3, assign46900_e79696_d_n4, assign46900_e79696_d_n5, assign46900_e79696_d_n6, assign46900_e79696_d_n7, assign46900_e79696_d_n8, assign46900_e79696_d_n9, assign46900_e79696_d_n10, assign46900_e79696_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46900_e79691: f64 = (1.0 + locals.var_qs_1);
        let assign46900_e79693: f64 = (assign46900_e79691 + locals.var_qdeff);
        let assign46900_e79694: f64 = (1.0 / assign46900_e79693);
        (assign46900_e79694, (-((locals.var_qs_1_dn3 + locals.var_qdeff_dn3) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn4 + locals.var_qdeff_dn4) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn5 + locals.var_qdeff_dn5) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn6 + locals.var_qdeff_dn6) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn7 + locals.var_qdeff_dn7) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn8 + locals.var_qdeff_dn8) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn9 + locals.var_qdeff_dn9) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn10 + locals.var_qdeff_dn10) / (assign46900_e79693 * assign46900_e79693))), (-((locals.var_qs_1_dn11 + locals.var_qdeff_dn11) / (assign46900_e79693 * assign46900_e79693))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign46900_e79696;
        locals.var_t0_dn3 = assign46900_e79696_d_n3;
        locals.var_t0_dn4 = assign46900_e79696_d_n4;
        locals.var_t0_dn5 = assign46900_e79696_d_n5;
        locals.var_t0_dn6 = assign46900_e79696_d_n6;
        locals.var_t0_dn7 = assign46900_e79696_d_n7;
        locals.var_t0_dn8 = assign46900_e79696_d_n8;
        locals.var_t0_dn9 = assign46900_e79696_d_n9;
        locals.var_t0_dn10 = assign46900_e79696_d_n10;
        locals.var_t0_dn11 = assign46900_e79696_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign46910_e79703, assign46910_e79703_d_n3, assign46910_e79703_d_n4, assign46910_e79703_d_n5, assign46910_e79703_d_n6, assign46910_e79703_d_n7, assign46910_e79703_d_n8, assign46910_e79703_d_n9, assign46910_e79703_d_n10, assign46910_e79703_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46910_e79701: f64 = (locals.var_dqsd2 * locals.var_t0);
        (assign46910_e79701, ((locals.var_dqsd2_dn3 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn3)), ((locals.var_dqsd2_dn4 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn4)), ((locals.var_dqsd2_dn5 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn5)), ((locals.var_dqsd2_dn6 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn6)), ((locals.var_dqsd2_dn7 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn7)), ((locals.var_dqsd2_dn8 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn8)), ((locals.var_dqsd2_dn9 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn9)), ((locals.var_dqsd2_dn10 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn10)), ((locals.var_dqsd2_dn11 * locals.var_t0) + (locals.var_dqsd2 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign46910_e79703;
        locals.var_t1_dn3 = assign46910_e79703_d_n3;
        locals.var_t1_dn4 = assign46910_e79703_d_n4;
        locals.var_t1_dn5 = assign46910_e79703_d_n5;
        locals.var_t1_dn6 = assign46910_e79703_d_n6;
        locals.var_t1_dn7 = assign46910_e79703_d_n7;
        locals.var_t1_dn8 = assign46910_e79703_d_n8;
        locals.var_t1_dn9 = assign46910_e79703_d_n9;
        locals.var_t1_dn10 = assign46910_e79703_d_n10;
        locals.var_t1_dn11 = assign46910_e79703_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign46920_e79722, assign46920_e79722_d_n3, assign46920_e79722_d_n4, assign46920_e79722_d_n5, assign46920_e79722_d_n6, assign46920_e79722_d_n7, assign46920_e79722_d_n8, assign46920_e79722_d_n9, assign46920_e79722_d_n10, assign46920_e79722_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46920_e79708: f64 = (locals.var_vgfb - locals.var_psip);
        let assign46920_e79711: f64 = (locals.var_nq - 1.0);
        let assign46920_e79714: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign46920_e79717: f64 = (0.3333333333333333 * locals.var_t1);
        let assign46920_e79718: f64 = (assign46920_e79714 + assign46920_e79717);
        let assign46920_e79719: f64 = (assign46920_e79711 * assign46920_e79718);
        let assign46920_e79720: f64 = (assign46920_e79708 - assign46920_e79719);
        (assign46920_e79720, ((locals.var_vgfb_dn3 - locals.var_psip_dn3) - ((locals.var_nq_dn3 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn3 + locals.var_qdeff_dn3) + (0.3333333333333333 * locals.var_t1_dn3))))), ((locals.var_vgfb_dn4 - locals.var_psip_dn4) - ((locals.var_nq_dn4 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn4 + locals.var_qdeff_dn4) + (0.3333333333333333 * locals.var_t1_dn4))))), ((locals.var_vgfb_dn5 - locals.var_psip_dn5) - ((locals.var_nq_dn5 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn5 + locals.var_qdeff_dn5) + (0.3333333333333333 * locals.var_t1_dn5))))), ((locals.var_vgfb_dn6 - locals.var_psip_dn6) - ((locals.var_nq_dn6 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn6 + locals.var_qdeff_dn6) + (0.3333333333333333 * locals.var_t1_dn6))))), ((locals.var_vgfb_dn7 - locals.var_psip_dn7) - ((locals.var_nq_dn7 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn7 + locals.var_qdeff_dn7) + (0.3333333333333333 * locals.var_t1_dn7))))), ((locals.var_vgfb_dn8 - locals.var_psip_dn8) - ((locals.var_nq_dn8 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn8 + locals.var_qdeff_dn8) + (0.3333333333333333 * locals.var_t1_dn8))))), ((locals.var_vgfb_dn9 - locals.var_psip_dn9) - ((locals.var_nq_dn9 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn9 + locals.var_qdeff_dn9) + (0.3333333333333333 * locals.var_t1_dn9))))), ((locals.var_vgfb_dn10 - locals.var_psip_dn10) - ((locals.var_nq_dn10 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn10 + locals.var_qdeff_dn10) + (0.3333333333333333 * locals.var_t1_dn10))))), ((locals.var_vgfb_dn11 - locals.var_psip_dn11) - ((locals.var_nq_dn11 * assign46920_e79718) + (assign46920_e79711 * ((locals.var_qs_1_dn11 + locals.var_qdeff_dn11) + (0.3333333333333333 * locals.var_t1_dn11))))),)
    } else {
        (locals.var_qb_1, locals.var_qb_1_dn3, locals.var_qb_1_dn4, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, locals.var_qb_1_dn9, locals.var_qb_1_dn10, locals.var_qb_1_dn11,)
    }
};
        locals.var_qb_1 = assign46920_e79722;
        locals.var_qb_1_dn3 = assign46920_e79722_d_n3;
        locals.var_qb_1_dn4 = assign46920_e79722_d_n4;
        locals.var_qb_1_dn5 = assign46920_e79722_d_n5;
        locals.var_qb_1_dn6 = assign46920_e79722_d_n6;
        locals.var_qb_1_dn7 = assign46920_e79722_d_n7;
        locals.var_qb_1_dn8 = assign46920_e79722_d_n8;
        locals.var_qb_1_dn9 = assign46920_e79722_d_n9;
        locals.var_qb_1_dn10 = assign46920_e79722_d_n10;
        locals.var_qb_1_dn11 = assign46920_e79722_d_n11;
        locals.var_qb_1_rv = 0.0;

        let (assign46930_e79729, assign46930_e79729_d_n3, assign46930_e79729_d_n4, assign46930_e79729_d_n5, assign46930_e79729_d_n6, assign46930_e79729_d_n7, assign46930_e79729_d_n8, assign46930_e79729_d_n9, assign46930_e79729_d_n10, assign46930_e79729_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46930_e79727: f64 = (0.3333333333333333 * locals.var_nq);
        (assign46930_e79727, (0.3333333333333333 * locals.var_nq_dn3), (0.3333333333333333 * locals.var_nq_dn4), (0.3333333333333333 * locals.var_nq_dn5), (0.3333333333333333 * locals.var_nq_dn6), (0.3333333333333333 * locals.var_nq_dn7), (0.3333333333333333 * locals.var_nq_dn8), (0.3333333333333333 * locals.var_nq_dn9), (0.3333333333333333 * locals.var_nq_dn10), (0.3333333333333333 * locals.var_nq_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign46930_e79729;
        locals.var_t2_dn3 = assign46930_e79729_d_n3;
        locals.var_t2_dn4 = assign46930_e79729_d_n4;
        locals.var_t2_dn5 = assign46930_e79729_d_n5;
        locals.var_t2_dn6 = assign46930_e79729_d_n6;
        locals.var_t2_dn7 = assign46930_e79729_d_n7;
        locals.var_t2_dn8 = assign46930_e79729_d_n8;
        locals.var_t2_dn9 = assign46930_e79729_d_n9;
        locals.var_t2_dn10 = assign46930_e79729_d_n10;
        locals.var_t2_dn11 = assign46930_e79729_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign46940_e79736, assign46940_e79736_d_n3, assign46940_e79736_d_n4, assign46940_e79736_d_n5, assign46940_e79736_d_n6, assign46940_e79736_d_n7, assign46940_e79736_d_n8, assign46940_e79736_d_n9, assign46940_e79736_d_n10, assign46940_e79736_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46940_e79734: f64 = (locals.var_t1 * locals.var_t0);
        (assign46940_e79734, ((locals.var_t1_dn3 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn3)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign46940_e79736;
        locals.var_t3_dn3 = assign46940_e79736_d_n3;
        locals.var_t3_dn4 = assign46940_e79736_d_n4;
        locals.var_t3_dn5 = assign46940_e79736_d_n5;
        locals.var_t3_dn6 = assign46940_e79736_d_n6;
        locals.var_t3_dn7 = assign46940_e79736_d_n7;
        locals.var_t3_dn8 = assign46940_e79736_d_n8;
        locals.var_t3_dn9 = assign46940_e79736_d_n9;
        locals.var_t3_dn10 = assign46940_e79736_d_n10;
        locals.var_t3_dn11 = assign46940_e79736_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign46950_e79761, assign46950_e79761_d_n3, assign46950_e79761_d_n4, assign46950_e79761_d_n5, assign46950_e79761_d_n6, assign46950_e79761_d_n7, assign46950_e79761_d_n8, assign46950_e79761_d_n9, assign46950_e79761_d_n10, assign46950_e79761_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46950_e79742: f64 = (2.0 * locals.var_qs_1);
        let assign46950_e79744: f64 = (assign46950_e79742 + locals.var_qdeff);
        let assign46950_e79749: f64 = (0.8 * locals.var_qs_1);
        let assign46950_e79750: f64 = (1.0 + assign46950_e79749);
        let assign46950_e79753: f64 = (1.2 * locals.var_qdeff);
        let assign46950_e79754: f64 = (assign46950_e79750 + assign46950_e79753);
        let assign46950_e79755: f64 = (0.5 * assign46950_e79754);
        let assign46950_e79757: f64 = (assign46950_e79755 * locals.var_t3);
        let assign46950_e79758: f64 = (assign46950_e79744 + assign46950_e79757);
        let assign46950_e79759: f64 = (locals.var_t2 * assign46950_e79758);
        (assign46950_e79759, ((locals.var_t2_dn3 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn3) + locals.var_qdeff_dn3) + (((0.5 * ((0.8 * locals.var_qs_1_dn3) + (1.2 * locals.var_qdeff_dn3))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn3))))), ((locals.var_t2_dn4 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn4) + locals.var_qdeff_dn4) + (((0.5 * ((0.8 * locals.var_qs_1_dn4) + (1.2 * locals.var_qdeff_dn4))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn4))))), ((locals.var_t2_dn5 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn5) + locals.var_qdeff_dn5) + (((0.5 * ((0.8 * locals.var_qs_1_dn5) + (1.2 * locals.var_qdeff_dn5))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn5))))), ((locals.var_t2_dn6 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn6) + locals.var_qdeff_dn6) + (((0.5 * ((0.8 * locals.var_qs_1_dn6) + (1.2 * locals.var_qdeff_dn6))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn6))))), ((locals.var_t2_dn7 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn7) + locals.var_qdeff_dn7) + (((0.5 * ((0.8 * locals.var_qs_1_dn7) + (1.2 * locals.var_qdeff_dn7))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn7))))), ((locals.var_t2_dn8 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn8) + locals.var_qdeff_dn8) + (((0.5 * ((0.8 * locals.var_qs_1_dn8) + (1.2 * locals.var_qdeff_dn8))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn8))))), ((locals.var_t2_dn9 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn9) + locals.var_qdeff_dn9) + (((0.5 * ((0.8 * locals.var_qs_1_dn9) + (1.2 * locals.var_qdeff_dn9))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn9))))), ((locals.var_t2_dn10 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn10) + locals.var_qdeff_dn10) + (((0.5 * ((0.8 * locals.var_qs_1_dn10) + (1.2 * locals.var_qdeff_dn10))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn10))))), ((locals.var_t2_dn11 * assign46950_e79758) + (locals.var_t2 * (((2.0 * locals.var_qs_1_dn11) + locals.var_qdeff_dn11) + (((0.5 * ((0.8 * locals.var_qs_1_dn11) + (1.2 * locals.var_qdeff_dn11))) * locals.var_t3) + (assign46950_e79755 * locals.var_t3_dn11))))),)
    } else {
        (locals.var_qs, locals.var_qs_dn3, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11,)
    }
};
        locals.var_qs = assign46950_e79761;
        locals.var_qs_dn3 = assign46950_e79761_d_n3;
        locals.var_qs_dn4 = assign46950_e79761_d_n4;
        locals.var_qs_dn5 = assign46950_e79761_d_n5;
        locals.var_qs_dn6 = assign46950_e79761_d_n6;
        locals.var_qs_dn7 = assign46950_e79761_d_n7;
        locals.var_qs_dn8 = assign46950_e79761_d_n8;
        locals.var_qs_dn9 = assign46950_e79761_d_n9;
        locals.var_qs_dn10 = assign46950_e79761_d_n10;
        locals.var_qs_dn11 = assign46950_e79761_d_n11;
        locals.var_qs_rv = 0.0;

        let (assign46960_e79786, assign46960_e79786_d_n3, assign46960_e79786_d_n4, assign46960_e79786_d_n5, assign46960_e79786_d_n6, assign46960_e79786_d_n7, assign46960_e79786_d_n8, assign46960_e79786_d_n9, assign46960_e79786_d_n10, assign46960_e79786_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46960_e79768: f64 = (2.0 * locals.var_qdeff);
        let assign46960_e79769: f64 = (locals.var_qs_1 + assign46960_e79768);
        let assign46960_e79774: f64 = (1.2 * locals.var_qs_1);
        let assign46960_e79775: f64 = (1.0 + assign46960_e79774);
        let assign46960_e79778: f64 = (0.8 * locals.var_qdeff);
        let assign46960_e79779: f64 = (assign46960_e79775 + assign46960_e79778);
        let assign46960_e79780: f64 = (0.5 * assign46960_e79779);
        let assign46960_e79782: f64 = (assign46960_e79780 * locals.var_t3);
        let assign46960_e79783: f64 = (assign46960_e79769 + assign46960_e79782);
        let assign46960_e79784: f64 = (locals.var_t2 * assign46960_e79783);
        (assign46960_e79784, ((locals.var_t2_dn3 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn3 + (2.0 * locals.var_qdeff_dn3)) + (((0.5 * ((1.2 * locals.var_qs_1_dn3) + (0.8 * locals.var_qdeff_dn3))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn3))))), ((locals.var_t2_dn4 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn4 + (2.0 * locals.var_qdeff_dn4)) + (((0.5 * ((1.2 * locals.var_qs_1_dn4) + (0.8 * locals.var_qdeff_dn4))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn4))))), ((locals.var_t2_dn5 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn5 + (2.0 * locals.var_qdeff_dn5)) + (((0.5 * ((1.2 * locals.var_qs_1_dn5) + (0.8 * locals.var_qdeff_dn5))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn5))))), ((locals.var_t2_dn6 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn6 + (2.0 * locals.var_qdeff_dn6)) + (((0.5 * ((1.2 * locals.var_qs_1_dn6) + (0.8 * locals.var_qdeff_dn6))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn6))))), ((locals.var_t2_dn7 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn7 + (2.0 * locals.var_qdeff_dn7)) + (((0.5 * ((1.2 * locals.var_qs_1_dn7) + (0.8 * locals.var_qdeff_dn7))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn7))))), ((locals.var_t2_dn8 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn8 + (2.0 * locals.var_qdeff_dn8)) + (((0.5 * ((1.2 * locals.var_qs_1_dn8) + (0.8 * locals.var_qdeff_dn8))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn8))))), ((locals.var_t2_dn9 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn9 + (2.0 * locals.var_qdeff_dn9)) + (((0.5 * ((1.2 * locals.var_qs_1_dn9) + (0.8 * locals.var_qdeff_dn9))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn9))))), ((locals.var_t2_dn10 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn10 + (2.0 * locals.var_qdeff_dn10)) + (((0.5 * ((1.2 * locals.var_qs_1_dn10) + (0.8 * locals.var_qdeff_dn10))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn10))))), ((locals.var_t2_dn11 * assign46960_e79783) + (locals.var_t2 * ((locals.var_qs_1_dn11 + (2.0 * locals.var_qdeff_dn11)) + (((0.5 * ((1.2 * locals.var_qs_1_dn11) + (0.8 * locals.var_qdeff_dn11))) * locals.var_t3) + (assign46960_e79780 * locals.var_t3_dn11))))),)
    } else {
        (locals.var_qd, locals.var_qd_dn3, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11,)
    }
};
        locals.var_qd = assign46960_e79786;
        locals.var_qd_dn3 = assign46960_e79786_d_n3;
        locals.var_qd_dn4 = assign46960_e79786_d_n4;
        locals.var_qd_dn5 = assign46960_e79786_d_n5;
        locals.var_qd_dn6 = assign46960_e79786_d_n6;
        locals.var_qd_dn7 = assign46960_e79786_d_n7;
        locals.var_qd_dn8 = assign46960_e79786_d_n8;
        locals.var_qd_dn9 = assign46960_e79786_d_n9;
        locals.var_qd_dn10 = assign46960_e79786_d_n10;
        locals.var_qd_dn11 = assign46960_e79786_d_n11;
        locals.var_qd_rv = 0.0;

        let (assign46970_e79816, assign46970_e79816_d_n3, assign46970_e79816_d_n4, assign46970_e79816_d_n5, assign46970_e79816_d_n6, assign46970_e79816_d_n7, assign46970_e79816_d_n8, assign46970_e79816_d_n9, assign46970_e79816_d_n10, assign46970_e79816_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46970_e79792: f64 = (locals.var_nvt * locals.var_qb_1);
        let assign46970_e79794: f64 = assign46970_e79792;
        let assign46970_e79797: f64 = (locals.var_nvt * locals.var_qb_1);
        let assign46970_e79799: f64 = assign46970_e79797;
        let assign46970_e79802: f64 = (locals.var_nvt * locals.var_qb_1);
        let assign46970_e79804: f64 = assign46970_e79802;
        let assign46970_e79805: f64 = (assign46970_e79799 * assign46970_e79804);
        let assign46970_e79808: f64 = (0.25 * 0.1);
        let assign46970_e79810: f64 = (assign46970_e79808 * 0.1);
        let assign46970_e79811: f64 = (assign46970_e79805 + assign46970_e79810);
        let assign46970_e79812: f64 = (assign46970_e79811).sqrt();
        let assign46970_e79813: f64 = (assign46970_e79794 + assign46970_e79812);
        let assign46970_e79814: f64 = (0.5 * assign46970_e79813);
        (assign46970_e79814, (0.5 * (((locals.var_nvt_dn3 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn3)) + (((((locals.var_nvt_dn3 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn3)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn3 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn3)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn4 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn4)) + (((((locals.var_nvt_dn4 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn4)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn4 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn4)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn5 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn5)) + (((((locals.var_nvt_dn5 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn5)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn5 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn5)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn6 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn6)) + (((((locals.var_nvt_dn6 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn6)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn6 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn6)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn7 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn7)) + (((((locals.var_nvt_dn7 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn7)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn7 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn7)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn8 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn8)) + (((((locals.var_nvt_dn8 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn8)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn8 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn8)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn9 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn9)) + (((((locals.var_nvt_dn9 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn9)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn9 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn9)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn10 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn10)) + (((((locals.var_nvt_dn10 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn10)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn10 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn10)))) / (2.0 * assign46970_e79812)))), (0.5 * (((locals.var_nvt_dn11 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn11)) + (((((locals.var_nvt_dn11 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn11)) * assign46970_e79804) + (assign46970_e79799 * ((locals.var_nvt_dn11 * locals.var_qb_1) + (locals.var_nvt * locals.var_qb_1_dn11)))) / (2.0 * assign46970_e79812)))),)
    } else {
        (locals.var_qba, locals.var_qba_dn3, locals.var_qba_dn4, locals.var_qba_dn5, locals.var_qba_dn6, locals.var_qba_dn7, locals.var_qba_dn8, locals.var_qba_dn9, locals.var_qba_dn10, locals.var_qba_dn11,)
    }
};
        locals.var_qba = assign46970_e79816;
        locals.var_qba_dn3 = assign46970_e79816_d_n3;
        locals.var_qba_dn4 = assign46970_e79816_d_n4;
        locals.var_qba_dn5 = assign46970_e79816_d_n5;
        locals.var_qba_dn6 = assign46970_e79816_d_n6;
        locals.var_qba_dn7 = assign46970_e79816_d_n7;
        locals.var_qba_dn8 = assign46970_e79816_d_n8;
        locals.var_qba_dn9 = assign46970_e79816_d_n9;
        locals.var_qba_dn10 = assign46970_e79816_d_n10;
        locals.var_qba_dn11 = assign46970_e79816_d_n11;
        locals.var_qba_rv = 0.0;

        let (assign46980_e79825, assign46980_e79825_d_n3, assign46980_e79825_d_n4, assign46980_e79825_d_n5, assign46980_e79825_d_n6, assign46980_e79825_d_n7, assign46980_e79825_d_n8, assign46980_e79825_d_n9, assign46980_e79825_d_n10, assign46980_e79825_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46980_e79822: f64 = (locals.var_qs + locals.var_qd);
        let assign46980_e79823: f64 = (locals.var_nvt * assign46980_e79822);
        (assign46980_e79823, ((locals.var_nvt_dn3 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn3 + locals.var_qd_dn3))), ((locals.var_nvt_dn4 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn4 + locals.var_qd_dn4))), ((locals.var_nvt_dn5 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn5 + locals.var_qd_dn5))), ((locals.var_nvt_dn6 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn6 + locals.var_qd_dn6))), ((locals.var_nvt_dn7 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn7 + locals.var_qd_dn7))), ((locals.var_nvt_dn8 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn8 + locals.var_qd_dn8))), ((locals.var_nvt_dn9 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn9 + locals.var_qd_dn9))), ((locals.var_nvt_dn10 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn10 + locals.var_qd_dn10))), ((locals.var_nvt_dn11 * assign46980_e79822) + (locals.var_nvt * (locals.var_qs_dn11 + locals.var_qd_dn11))),)
    } else {
        (locals.var_qia, locals.var_qia_dn3, locals.var_qia_dn4, locals.var_qia_dn5, locals.var_qia_dn6, locals.var_qia_dn7, locals.var_qia_dn8, locals.var_qia_dn9, locals.var_qia_dn10, locals.var_qia_dn11,)
    }
};
        locals.var_qia = assign46980_e79825;
        locals.var_qia_dn3 = assign46980_e79825_d_n3;
        locals.var_qia_dn4 = assign46980_e79825_d_n4;
        locals.var_qia_dn5 = assign46980_e79825_d_n5;
        locals.var_qia_dn6 = assign46980_e79825_d_n6;
        locals.var_qia_dn7 = assign46980_e79825_d_n7;
        locals.var_qia_dn8 = assign46980_e79825_d_n8;
        locals.var_qia_dn9 = assign46980_e79825_d_n9;
        locals.var_qia_dn10 = assign46980_e79825_d_n10;
        locals.var_qia_dn11 = assign46980_e79825_d_n11;
        locals.var_qia_rv = 0.0;

        let (assign46990_e79836, assign46990_e79836_d_n3, assign46990_e79836_d_n4, assign46990_e79836_d_n5, assign46990_e79836_d_n6, assign46990_e79836_d_n7, assign46990_e79836_d_n8, assign46990_e79836_d_n9, assign46990_e79836_d_n10, assign46990_e79836_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign46990_e79832: f64 = (locals.var_eta_mu * locals.var_qia);
        let assign46990_e79833: f64 = (locals.var_qba + assign46990_e79832);
        let assign46990_e79834: f64 = (locals.var_eefffactor * assign46990_e79833);
        (assign46990_e79834, (locals.var_eefffactor * (locals.var_qba_dn3 + (locals.var_eta_mu * locals.var_qia_dn3))), (locals.var_eefffactor * (locals.var_qba_dn4 + (locals.var_eta_mu * locals.var_qia_dn4))), (locals.var_eefffactor * (locals.var_qba_dn5 + (locals.var_eta_mu * locals.var_qia_dn5))), (locals.var_eefffactor * (locals.var_qba_dn6 + (locals.var_eta_mu * locals.var_qia_dn6))), (locals.var_eefffactor * (locals.var_qba_dn7 + (locals.var_eta_mu * locals.var_qia_dn7))), (locals.var_eefffactor * (locals.var_qba_dn8 + (locals.var_eta_mu * locals.var_qia_dn8))), (locals.var_eefffactor * (locals.var_qba_dn9 + (locals.var_eta_mu * locals.var_qia_dn9))), (locals.var_eefffactor * (locals.var_qba_dn10 + (locals.var_eta_mu * locals.var_qia_dn10))), (locals.var_eefffactor * (locals.var_qba_dn11 + (locals.var_eta_mu * locals.var_qia_dn11))),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn3, locals.var_eeffm_dn4, locals.var_eeffm_dn5, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8, locals.var_eeffm_dn9, locals.var_eeffm_dn10, locals.var_eeffm_dn11,)
    }
};
        locals.var_eeffm = assign46990_e79836;
        locals.var_eeffm_dn3 = assign46990_e79836_d_n3;
        locals.var_eeffm_dn4 = assign46990_e79836_d_n4;
        locals.var_eeffm_dn5 = assign46990_e79836_d_n5;
        locals.var_eeffm_dn6 = assign46990_e79836_d_n6;
        locals.var_eeffm_dn7 = assign46990_e79836_d_n7;
        locals.var_eeffm_dn8 = assign46990_e79836_d_n8;
        locals.var_eeffm_dn9 = assign46990_e79836_d_n9;
        locals.var_eeffm_dn10 = assign46990_e79836_d_n10;
        locals.var_eeffm_dn11 = assign46990_e79836_d_n11;
        locals.var_eeffm_rv = 0.0;

        let (assign47000_e79849, assign47000_e79849_d_n3, assign47000_e79849_d_n4, assign47000_e79849_d_n5, assign47000_e79849_d_n6, assign47000_e79849_d_n7, assign47000_e79849_d_n8, assign47000_e79849_d_n9, assign47000_e79849_d_n10, assign47000_e79849_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47000_e79843: f64 = (locals.var_qia / locals.var_qba);
        let assign47000_e79844: f64 = (1.0 + assign47000_e79843);
        let assign47000_e79845: f64 = (0.5 * assign47000_e79844);
        let assign47000_e79847: f64 = (assign47000_e79845).powf(locals.var_ucs_a);
        (assign47000_e79847, if locals.var_ucs_a_dn3 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn3 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn3)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn3 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn3 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn3)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn4 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn4 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn4)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn4 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn4 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn4)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn5 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn5 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn5)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn5 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn5 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn5)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn6 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn6 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn6)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn6 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn6 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn6)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn7 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn7 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn7)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn7 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn7 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn7)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn8 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn8 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn8)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn8 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn8 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn8)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn9 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn9 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn9)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn9 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn9 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn9)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn10 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn10 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn10)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn10 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn10 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn10)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) }, if locals.var_ucs_a_dn11 == 0.0 && ((locals.var_ucs_a) as f64).is_finite() && ((locals.var_ucs_a) as f64).fract() == 0.0 { if locals.var_ucs_a == 0.0 { 0.0 } else { (locals.var_ucs_a * ((assign47000_e79845).powf(locals.var_ucs_a - 1.0) * (0.5 * (((locals.var_qia_dn11 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn11)) / (locals.var_qba * locals.var_qba))))) } } else { (assign47000_e79847 * ((locals.var_ucs_a_dn11 * (assign47000_e79845).ln()) + (locals.var_ucs_a * ((0.5 * (((locals.var_qia_dn11 * locals.var_qba) - (locals.var_qia * locals.var_qba_dn11)) / (locals.var_qba * locals.var_qba))) / assign47000_e79845)))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47000_e79849;
        locals.var_t2_dn3 = assign47000_e79849_d_n3;
        locals.var_t2_dn4 = assign47000_e79849_d_n4;
        locals.var_t2_dn5 = assign47000_e79849_d_n5;
        locals.var_t2_dn6 = assign47000_e79849_d_n6;
        locals.var_t2_dn7 = assign47000_e79849_d_n7;
        locals.var_t2_dn8 = assign47000_e79849_d_n8;
        locals.var_t2_dn9 = assign47000_e79849_d_n9;
        locals.var_t2_dn10 = assign47000_e79849_d_n10;
        locals.var_t2_dn11 = assign47000_e79849_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47010_e79866, assign47010_e79866_d_n3, assign47010_e79866_d_n4, assign47010_e79866_d_n5, assign47010_e79866_d_n6, assign47010_e79866_d_n7, assign47010_e79866_d_n8, assign47010_e79866_d_n9, assign47010_e79866_d_n10, assign47010_e79866_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47010_e79855: f64 = (locals.var_uc_a * locals.var_vbsx);
        let assign47010_e79856: f64 = (locals.var_ua_a + assign47010_e79855);
        let assign47010_e79859: f64 = (locals.var_eeffm).powf(locals.var_eu_t);
        let assign47010_e79860: f64 = (assign47010_e79856 * assign47010_e79859);
        let assign47010_e79863: f64 = (locals.var_ud_a / locals.var_t2);
        let assign47010_e79864: f64 = (assign47010_e79860 + assign47010_e79863);
        (assign47010_e79864, ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn3))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn3 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn3)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn3 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn3 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn3 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn4))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn4 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn4)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn4 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn4 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn4 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn5))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn5 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn5)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn5 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn5 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn5 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn6))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn6 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn6)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn6 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn6 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn6 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn7))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn7 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn7)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn7 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn7 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn7 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn8))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn8 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn8)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn8 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn8 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn8 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn9))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn9 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn9)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn9 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn9 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn9 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn10))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn10 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn10)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn10 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn10 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn10 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn11))) * assign47010_e79859) + (assign47010_e79856 * if locals.var_eu_t_dn11 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn11)) } } else { (assign47010_e79859 * ((locals.var_eu_t_dn11 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn11 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn11 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47010_e79866;
        locals.var_t3_dn3 = assign47010_e79866_d_n3;
        locals.var_t3_dn4 = assign47010_e79866_d_n4;
        locals.var_t3_dn5 = assign47010_e79866_d_n5;
        locals.var_t3_dn6 = assign47010_e79866_d_n6;
        locals.var_t3_dn7 = assign47010_e79866_d_n7;
        locals.var_t3_dn8 = assign47010_e79866_d_n8;
        locals.var_t3_dn9 = assign47010_e79866_d_n9;
        locals.var_t3_dn10 = assign47010_e79866_d_n10;
        locals.var_t3_dn11 = assign47010_e79866_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign47020_e79873, assign47020_e79873_d_n3, assign47020_e79873_d_n4, assign47020_e79873_d_n5, assign47020_e79873_d_n6, assign47020_e79873_d_n7, assign47020_e79873_d_n8, assign47020_e79873_d_n9, assign47020_e79873_d_n10, assign47020_e79873_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47020_e79871: f64 = (1.0 + locals.var_t3);
        (assign47020_e79871, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47020_e79873;
        locals.var_t4_dn3 = assign47020_e79873_d_n3;
        locals.var_t4_dn4 = assign47020_e79873_d_n4;
        locals.var_t4_dn5 = assign47020_e79873_d_n5;
        locals.var_t4_dn6 = assign47020_e79873_d_n6;
        locals.var_t4_dn7 = assign47020_e79873_d_n7;
        locals.var_t4_dn8 = assign47020_e79873_d_n8;
        locals.var_t4_dn9 = assign47020_e79873_d_n9;
        locals.var_t4_dn10 = assign47020_e79873_d_n10;
        locals.var_t4_dn11 = assign47020_e79873_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47030_e79897, assign47030_e79897_d_n3, assign47030_e79897_d_n4, assign47030_e79897_d_n5, assign47030_e79897_d_n6, assign47030_e79897_d_n7, assign47030_e79897_d_n8, assign47030_e79897_d_n9, assign47030_e79897_d_n10, assign47030_e79897_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47030_e79879: f64 = (locals.var_t4 + 1.0);
        let assign47030_e79882: f64 = (locals.var_t4 - 1.0);
        let assign47030_e79885: f64 = (locals.var_t4 - 1.0);
        let assign47030_e79886: f64 = (assign47030_e79882 * assign47030_e79885);
        let assign47030_e79889: f64 = (0.25 * 0.0015);
        let assign47030_e79891: f64 = (assign47030_e79889 * 0.0015);
        let assign47030_e79892: f64 = (assign47030_e79886 + assign47030_e79891);
        let assign47030_e79893: f64 = (assign47030_e79892).sqrt();
        let assign47030_e79894: f64 = (assign47030_e79879 + assign47030_e79893);
        let assign47030_e79895: f64 = (0.5 * assign47030_e79894);
        (assign47030_e79895, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn3)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn4)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn5)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn6)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn7)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn8)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn9)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn10)) / (2.0 * assign47030_e79893)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign47030_e79885) + (assign47030_e79882 * locals.var_t4_dn11)) / (2.0 * assign47030_e79893)))),)
    } else {
        (locals.var_dmob, locals.var_dmob_dn3, locals.var_dmob_dn4, locals.var_dmob_dn5, locals.var_dmob_dn6, locals.var_dmob_dn7, locals.var_dmob_dn8, locals.var_dmob_dn9, locals.var_dmob_dn10, locals.var_dmob_dn11,)
    }
};
        locals.var_dmob = assign47030_e79897;
        locals.var_dmob_dn3 = assign47030_e79897_d_n3;
        locals.var_dmob_dn4 = assign47030_e79897_d_n4;
        locals.var_dmob_dn5 = assign47030_e79897_d_n5;
        locals.var_dmob_dn6 = assign47030_e79897_d_n6;
        locals.var_dmob_dn7 = assign47030_e79897_d_n7;
        locals.var_dmob_dn8 = assign47030_e79897_d_n8;
        locals.var_dmob_dn9 = assign47030_e79897_d_n9;
        locals.var_dmob_dn10 = assign47030_e79897_d_n10;
        locals.var_dmob_dn11 = assign47030_e79897_d_n11;
        locals.var_dmob_rv = 0.0;

        let (assign47040_e79908, assign47040_e79908_d_n3, assign47040_e79908_d_n4, assign47040_e79908_d_n5, assign47040_e79908_d_n6, assign47040_e79908_d_n7, assign47040_e79908_d_n8, assign47040_e79908_d_n9, assign47040_e79908_d_n10, assign47040_e79908_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47040_e79902: f64 = (2.0 * locals.var_vsat_a);
        let assign47040_e79905: f64 = (locals.var_u0_a / locals.var_dmob);
        let assign47040_e79906: f64 = (assign47040_e79902 / assign47040_e79905);
        (assign47040_e79906, ((((2.0 * locals.var_vsat_a_dn3) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn3 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn3)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn4) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn4 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn5) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn5 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn5)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn6) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn6 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn6)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn7) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn7 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn7)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn8) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn8 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn8)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn9) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn9 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn9)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn10) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn10 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn10)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)), ((((2.0 * locals.var_vsat_a_dn11) * assign47040_e79905) - (assign47040_e79902 * (((locals.var_u0_a_dn11 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn11)) / (locals.var_dmob * locals.var_dmob)))) / (assign47040_e79905 * assign47040_e79905)),)
    } else {
        (locals.var_esat, locals.var_esat_dn3, locals.var_esat_dn4, locals.var_esat_dn5, locals.var_esat_dn6, locals.var_esat_dn7, locals.var_esat_dn8, locals.var_esat_dn9, locals.var_esat_dn10, locals.var_esat_dn11,)
    }
};
        locals.var_esat = assign47040_e79908;
        locals.var_esat_dn3 = assign47040_e79908_d_n3;
        locals.var_esat_dn4 = assign47040_e79908_d_n4;
        locals.var_esat_dn5 = assign47040_e79908_d_n5;
        locals.var_esat_dn6 = assign47040_e79908_d_n6;
        locals.var_esat_dn7 = assign47040_e79908_d_n7;
        locals.var_esat_dn8 = assign47040_e79908_d_n8;
        locals.var_esat_dn9 = assign47040_e79908_d_n9;
        locals.var_esat_dn10 = assign47040_e79908_d_n10;
        locals.var_esat_dn11 = assign47040_e79908_d_n11;
        locals.var_esat_rv = 0.0;

        let (assign47050_e79915, assign47050_e79915_d_n3, assign47050_e79915_d_n4, assign47050_e79915_d_n5, assign47050_e79915_d_n6, assign47050_e79915_d_n7, assign47050_e79915_d_n8, assign47050_e79915_d_n9, assign47050_e79915_d_n10, assign47050_e79915_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47050_e79913: f64 = (locals.var_esat * locals.var_leff);
        (assign47050_e79913, (locals.var_esat_dn3 * locals.var_leff), (locals.var_esat_dn4 * locals.var_leff), (locals.var_esat_dn5 * locals.var_leff), (locals.var_esat_dn6 * locals.var_leff), (locals.var_esat_dn7 * locals.var_leff), (locals.var_esat_dn8 * locals.var_leff), (locals.var_esat_dn9 * locals.var_leff), (locals.var_esat_dn10 * locals.var_leff), (locals.var_esat_dn11 * locals.var_leff),)
    } else {
        (locals.var_esatl, locals.var_esatl_dn3, locals.var_esatl_dn4, locals.var_esatl_dn5, locals.var_esatl_dn6, locals.var_esatl_dn7, locals.var_esatl_dn8, locals.var_esatl_dn9, locals.var_esatl_dn10, locals.var_esatl_dn11,)
    }
};
        locals.var_esatl = assign47050_e79915;
        locals.var_esatl_dn3 = assign47050_e79915_d_n3;
        locals.var_esatl_dn4 = assign47050_e79915_d_n4;
        locals.var_esatl_dn5 = assign47050_e79915_d_n5;
        locals.var_esatl_dn6 = assign47050_e79915_d_n6;
        locals.var_esatl_dn7 = assign47050_e79915_d_n7;
        locals.var_esatl_dn8 = assign47050_e79915_d_n8;
        locals.var_esatl_dn9 = assign47050_e79915_d_n9;
        locals.var_esatl_dn10 = assign47050_e79915_d_n10;
        locals.var_esatl_dn11 = assign47050_e79915_d_n11;
        locals.var_esatl_rv = 0.0;

        let assign47060_e79918: f64 = if locals.var_pvag_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard728 = assign47060_e79918;
        locals.var_guard728_rv = 0.0;

        let (assign47070_e79931, assign47070_e79931_d_n3, assign47070_e79931_d_n4, assign47070_e79931_d_n5, assign47070_e79931_d_n6, assign47070_e79931_d_n7, assign47070_e79931_d_n8, assign47070_e79931_d_n9, assign47070_e79931_d_n10, assign47070_e79931_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard728 != 0.0)) {
        let assign47070_e79926: f64 = (locals.var_pvag_i * locals.var_qia);
        let assign47070_e79928: f64 = (assign47070_e79926 / locals.var_esatl);
        let assign47070_e79929: f64 = (1.0 + assign47070_e79928);
        (assign47070_e79929, ((((locals.var_pvag_i * locals.var_qia_dn3) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn4) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn5) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn6) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn7) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn8) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn9) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn10) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn11) * locals.var_esatl) - (assign47070_e79926 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_pvagfactor, locals.var_pvagfactor_dn3, locals.var_pvagfactor_dn4, locals.var_pvagfactor_dn5, locals.var_pvagfactor_dn6, locals.var_pvagfactor_dn7, locals.var_pvagfactor_dn8, locals.var_pvagfactor_dn9, locals.var_pvagfactor_dn10, locals.var_pvagfactor_dn11,)
    }
};
        locals.var_pvagfactor = assign47070_e79931;
        locals.var_pvagfactor_dn3 = assign47070_e79931_d_n3;
        locals.var_pvagfactor_dn4 = assign47070_e79931_d_n4;
        locals.var_pvagfactor_dn5 = assign47070_e79931_d_n5;
        locals.var_pvagfactor_dn6 = assign47070_e79931_d_n6;
        locals.var_pvagfactor_dn7 = assign47070_e79931_d_n7;
        locals.var_pvagfactor_dn8 = assign47070_e79931_d_n8;
        locals.var_pvagfactor_dn9 = assign47070_e79931_d_n9;
        locals.var_pvagfactor_dn10 = assign47070_e79931_d_n10;
        locals.var_pvagfactor_dn11 = assign47070_e79931_d_n11;
        locals.var_pvagfactor_rv = 0.0;

        let (assign47080_e79947, assign47080_e79947_d_n3, assign47080_e79947_d_n4, assign47080_e79947_d_n5, assign47080_e79947_d_n6, assign47080_e79947_d_n7, assign47080_e79947_d_n8, assign47080_e79947_d_n9, assign47080_e79947_d_n10, assign47080_e79947_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard728 == 0.0)) {
        let assign47080_e79941: f64 = (locals.var_pvag_i * locals.var_qia);
        let assign47080_e79943: f64 = (assign47080_e79941 / locals.var_esatl);
        let assign47080_e79944: f64 = (1.0 - assign47080_e79943);
        let assign47080_e79945: f64 = (1.0 / assign47080_e79944);
        (assign47080_e79945, (-((-((((locals.var_pvag_i * locals.var_qia_dn3) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn4) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn5) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn6) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn7) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn8) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn9) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn10) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))), (-((-((((locals.var_pvag_i * locals.var_qia_dn11) * locals.var_esatl) - (assign47080_e79941 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl))) / (assign47080_e79944 * assign47080_e79944))),)
    } else {
        (locals.var_pvagfactor, locals.var_pvagfactor_dn3, locals.var_pvagfactor_dn4, locals.var_pvagfactor_dn5, locals.var_pvagfactor_dn6, locals.var_pvagfactor_dn7, locals.var_pvagfactor_dn8, locals.var_pvagfactor_dn9, locals.var_pvagfactor_dn10, locals.var_pvagfactor_dn11,)
    }
};
        locals.var_pvagfactor = assign47080_e79947;
        locals.var_pvagfactor_dn3 = assign47080_e79947_d_n3;
        locals.var_pvagfactor_dn4 = assign47080_e79947_d_n4;
        locals.var_pvagfactor_dn5 = assign47080_e79947_d_n5;
        locals.var_pvagfactor_dn6 = assign47080_e79947_d_n6;
        locals.var_pvagfactor_dn7 = assign47080_e79947_d_n7;
        locals.var_pvagfactor_dn8 = assign47080_e79947_d_n8;
        locals.var_pvagfactor_dn9 = assign47080_e79947_d_n9;
        locals.var_pvagfactor_dn10 = assign47080_e79947_d_n10;
        locals.var_pvagfactor_dn11 = assign47080_e79947_d_n11;
        locals.var_pvagfactor_rv = 0.0;

        let (assign47090_e79952, assign47090_e79952_d_n3, assign47090_e79952_d_n4, assign47090_e79952_d_n5, assign47090_e79952_d_n6, assign47090_e79952_d_n7, assign47090_e79952_d_n8, assign47090_e79952_d_n9, assign47090_e79952_d_n10, assign47090_e79952_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_pdiblc_a, locals.var_pdiblc_a_dn3, locals.var_pdiblc_a_dn4, locals.var_pdiblc_a_dn5, locals.var_pdiblc_a_dn6, locals.var_pdiblc_a_dn7, locals.var_pdiblc_a_dn8, locals.var_pdiblc_a_dn9, locals.var_pdiblc_a_dn10, locals.var_pdiblc_a_dn11,)
    } else {
        (locals.var_diblfactor, locals.var_diblfactor_dn3, locals.var_diblfactor_dn4, locals.var_diblfactor_dn5, locals.var_diblfactor_dn6, locals.var_diblfactor_dn7, locals.var_diblfactor_dn8, locals.var_diblfactor_dn9, locals.var_diblfactor_dn10, locals.var_diblfactor_dn11,)
    }
};
        locals.var_diblfactor = assign47090_e79952;
        locals.var_diblfactor_dn3 = assign47090_e79952_d_n3;
        locals.var_diblfactor_dn4 = assign47090_e79952_d_n4;
        locals.var_diblfactor_dn5 = assign47090_e79952_d_n5;
        locals.var_diblfactor_dn6 = assign47090_e79952_d_n6;
        locals.var_diblfactor_dn7 = assign47090_e79952_d_n7;
        locals.var_diblfactor_dn8 = assign47090_e79952_d_n8;
        locals.var_diblfactor_dn9 = assign47090_e79952_d_n9;
        locals.var_diblfactor_dn10 = assign47090_e79952_d_n10;
        locals.var_diblfactor_dn11 = assign47090_e79952_d_n11;
        locals.var_diblfactor_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_164(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47100_e79959, assign47100_e79959_d_n3, assign47100_e79959_d_n4, assign47100_e79959_d_n5, assign47100_e79959_d_n6, assign47100_e79959_d_n7, assign47100_e79959_d_n8, assign47100_e79959_d_n9, assign47100_e79959_d_n10, assign47100_e79959_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47100_e79957: f64 = (locals.var_vds - locals.var_vdseff);
        (assign47100_e79957, (-locals.var_vdseff_dn3), (-locals.var_vdseff_dn4), (-locals.var_vdseff_dn5), (locals.var_vds_dn6 - locals.var_vdseff_dn6), (locals.var_vds_dn7 - locals.var_vdseff_dn7), (-locals.var_vdseff_dn8), (-locals.var_vdseff_dn9), (locals.var_vds_dn10 - locals.var_vdseff_dn10), (-locals.var_vdseff_dn11),)
    } else {
        (locals.var_diffvds, locals.var_diffvds_dn3, locals.var_diffvds_dn4, locals.var_diffvds_dn5, locals.var_diffvds_dn6, locals.var_diffvds_dn7, locals.var_diffvds_dn8, locals.var_diffvds_dn9, locals.var_diffvds_dn10, locals.var_diffvds_dn11,)
    }
};
        locals.var_diffvds = assign47100_e79959;
        locals.var_diffvds_dn3 = assign47100_e79959_d_n3;
        locals.var_diffvds_dn4 = assign47100_e79959_d_n4;
        locals.var_diffvds_dn5 = assign47100_e79959_d_n5;
        locals.var_diffvds_dn6 = assign47100_e79959_d_n6;
        locals.var_diffvds_dn7 = assign47100_e79959_d_n7;
        locals.var_diffvds_dn8 = assign47100_e79959_d_n8;
        locals.var_diffvds_dn9 = assign47100_e79959_d_n9;
        locals.var_diffvds_dn10 = assign47100_e79959_d_n10;
        locals.var_diffvds_dn11 = assign47100_e79959_d_n11;
        locals.var_diffvds_rv = 0.0;

        let (assign47110_e79968, assign47110_e79968_d_n3, assign47110_e79968_d_n4, assign47110_e79968_d_n5, assign47110_e79968_d_n6, assign47110_e79968_d_n7, assign47110_e79968_d_n8, assign47110_e79968_d_n9, assign47110_e79968_d_n10, assign47110_e79968_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47110_e79965: f64 = (2.0 * locals.var_nvt);
        let assign47110_e79966: f64 = (locals.var_qia + assign47110_e79965);
        (assign47110_e79966, (locals.var_qia_dn3 + (2.0 * locals.var_nvt_dn3)), (locals.var_qia_dn4 + (2.0 * locals.var_nvt_dn4)), (locals.var_qia_dn5 + (2.0 * locals.var_nvt_dn5)), (locals.var_qia_dn6 + (2.0 * locals.var_nvt_dn6)), (locals.var_qia_dn7 + (2.0 * locals.var_nvt_dn7)), (locals.var_qia_dn8 + (2.0 * locals.var_nvt_dn8)), (locals.var_qia_dn9 + (2.0 * locals.var_nvt_dn9)), (locals.var_qia_dn10 + (2.0 * locals.var_nvt_dn10)), (locals.var_qia_dn11 + (2.0 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vgst2vtm, locals.var_vgst2vtm_dn3, locals.var_vgst2vtm_dn4, locals.var_vgst2vtm_dn5, locals.var_vgst2vtm_dn6, locals.var_vgst2vtm_dn7, locals.var_vgst2vtm_dn8, locals.var_vgst2vtm_dn9, locals.var_vgst2vtm_dn10, locals.var_vgst2vtm_dn11,)
    }
};
        locals.var_vgst2vtm = assign47110_e79968;
        locals.var_vgst2vtm_dn3 = assign47110_e79968_d_n3;
        locals.var_vgst2vtm_dn4 = assign47110_e79968_d_n4;
        locals.var_vgst2vtm_dn5 = assign47110_e79968_d_n5;
        locals.var_vgst2vtm_dn6 = assign47110_e79968_d_n6;
        locals.var_vgst2vtm_dn7 = assign47110_e79968_d_n7;
        locals.var_vgst2vtm_dn8 = assign47110_e79968_d_n8;
        locals.var_vgst2vtm_dn9 = assign47110_e79968_d_n9;
        locals.var_vgst2vtm_dn10 = assign47110_e79968_d_n10;
        locals.var_vgst2vtm_dn11 = assign47110_e79968_d_n11;
        locals.var_vgst2vtm_rv = 0.0;

        let assign47120_e79971: f64 = if locals.var_diblfactor > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard729 = assign47120_e79971;
        locals.var_guard729_rv = 0.0;

        let (assign47130_e79982, assign47130_e79982_d_n3, assign47130_e79982_d_n4, assign47130_e79982_d_n5, assign47130_e79982_d_n6, assign47130_e79982_d_n7, assign47130_e79982_d_n8, assign47130_e79982_d_n9, assign47130_e79982_d_n10, assign47130_e79982_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard729 != 0.0)) {
        let assign47130_e79979: f64 = (locals.var_vdssat + locals.var_vgst2vtm);
        let assign47130_e79980: f64 = (locals.var_vgst2vtm / assign47130_e79979);
        (assign47130_e79980, (((locals.var_vgst2vtm_dn3 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn3 + locals.var_vgst2vtm_dn3))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn4 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn4 + locals.var_vgst2vtm_dn4))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn5 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn5 + locals.var_vgst2vtm_dn5))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn6 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn6 + locals.var_vgst2vtm_dn6))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn7 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn7 + locals.var_vgst2vtm_dn7))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn8 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn8 + locals.var_vgst2vtm_dn8))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn9 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn9 + locals.var_vgst2vtm_dn9))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn10 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn10 + locals.var_vgst2vtm_dn10))) / (assign47130_e79979 * assign47130_e79979)), (((locals.var_vgst2vtm_dn11 * assign47130_e79979) - (locals.var_vgst2vtm * (locals.var_vdssat_dn11 + locals.var_vgst2vtm_dn11))) / (assign47130_e79979 * assign47130_e79979)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47130_e79982;
        locals.var_t3_dn3 = assign47130_e79982_d_n3;
        locals.var_t3_dn4 = assign47130_e79982_d_n4;
        locals.var_t3_dn5 = assign47130_e79982_d_n5;
        locals.var_t3_dn6 = assign47130_e79982_d_n6;
        locals.var_t3_dn7 = assign47130_e79982_d_n7;
        locals.var_t3_dn8 = assign47130_e79982_d_n8;
        locals.var_t3_dn9 = assign47130_e79982_d_n9;
        locals.var_t3_dn10 = assign47130_e79982_d_n10;
        locals.var_t3_dn11 = assign47130_e79982_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign47140_e80014, assign47140_e80014_d_n3, assign47140_e80014_d_n4, assign47140_e80014_d_n5, assign47140_e80014_d_n6, assign47140_e80014_d_n7, assign47140_e80014_d_n8, assign47140_e80014_d_n9, assign47140_e80014_d_n10, assign47140_e80014_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard729 != 0.0)) {
        let assign47140_e79991: f64 = (locals.var_pdiblcb_i * locals.var_vbsx);
        let assign47140_e79992: f64 = (1.0 + assign47140_e79991);
        let assign47140_e79996: f64 = (locals.var_pdiblcb_i * locals.var_vbsx);
        let assign47140_e79997: f64 = (1.0 + assign47140_e79996);
        let assign47140_e80001: f64 = (locals.var_pdiblcb_i * locals.var_vbsx);
        let assign47140_e80002: f64 = (1.0 + assign47140_e80001);
        let assign47140_e80003: f64 = (assign47140_e79997 * assign47140_e80002);
        let assign47140_e80006: f64 = (4.0 * 0.001);
        let assign47140_e80008: f64 = (assign47140_e80006 * 0.001);
        let assign47140_e80009: f64 = (assign47140_e80003 + assign47140_e80008);
        let assign47140_e80010: f64 = (assign47140_e80009).sqrt();
        let assign47140_e80011: f64 = (assign47140_e79992 + assign47140_e80010);
        let assign47140_e80012: f64 = (0.5 * assign47140_e80011);
        (assign47140_e80012, (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn3) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn3) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn3))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn4) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn4) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn4))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn5) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn5) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn5))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn6) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn6) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn6))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn7) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn7) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn7))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn8) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn8) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn8))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn9) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn9) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn9))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn10) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn10) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn10))) / (2.0 * assign47140_e80010)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn11) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn11) * assign47140_e80002) + (assign47140_e79997 * (locals.var_pdiblcb_i * locals.var_vbsx_dn11))) / (2.0 * assign47140_e80010)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47140_e80014;
        locals.var_t4_dn3 = assign47140_e80014_d_n3;
        locals.var_t4_dn4 = assign47140_e80014_d_n4;
        locals.var_t4_dn5 = assign47140_e80014_d_n5;
        locals.var_t4_dn6 = assign47140_e80014_d_n6;
        locals.var_t4_dn7 = assign47140_e80014_d_n7;
        locals.var_t4_dn8 = assign47140_e80014_d_n8;
        locals.var_t4_dn9 = assign47140_e80014_d_n9;
        locals.var_t4_dn10 = assign47140_e80014_d_n10;
        locals.var_t4_dn11 = assign47140_e80014_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47150_e80023, assign47150_e80023_d_n3, assign47150_e80023_d_n4, assign47150_e80023_d_n5, assign47150_e80023_d_n6, assign47150_e80023_d_n7, assign47150_e80023_d_n8, assign47150_e80023_d_n9, assign47150_e80023_d_n10, assign47150_e80023_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard729 != 0.0)) {
        let assign47150_e80021: f64 = (1.0 / locals.var_t4);
        (assign47150_e80021, (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47150_e80023;
        locals.var_t5_dn3 = assign47150_e80023_d_n3;
        locals.var_t5_dn4 = assign47150_e80023_d_n4;
        locals.var_t5_dn5 = assign47150_e80023_d_n5;
        locals.var_t5_dn6 = assign47150_e80023_d_n6;
        locals.var_t5_dn7 = assign47150_e80023_d_n7;
        locals.var_t5_dn8 = assign47150_e80023_d_n8;
        locals.var_t5_dn9 = assign47150_e80023_d_n9;
        locals.var_t5_dn10 = assign47150_e80023_d_n10;
        locals.var_t5_dn11 = assign47150_e80023_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign47160_e80038, assign47160_e80038_d_n3, assign47160_e80038_d_n4, assign47160_e80038_d_n5, assign47160_e80038_d_n6, assign47160_e80038_d_n7, assign47160_e80038_d_n8, assign47160_e80038_d_n9, assign47160_e80038_d_n10, assign47160_e80038_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard729 != 0.0)) {
        let assign47160_e80030: f64 = (locals.var_vgst2vtm / locals.var_diblfactor);
        let assign47160_e80032: f64 = (assign47160_e80030 * locals.var_t3);
        let assign47160_e80034: f64 = (assign47160_e80032 * locals.var_pvagfactor);
        let assign47160_e80036: f64 = (assign47160_e80034 * locals.var_t5);
        (assign47160_e80036, (((((((((locals.var_vgst2vtm_dn3 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn3)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn3)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn3)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn3)), (((((((((locals.var_vgst2vtm_dn4 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn4)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn4)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn4)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn4)), (((((((((locals.var_vgst2vtm_dn5 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn5)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn5)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn5)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn5)), (((((((((locals.var_vgst2vtm_dn6 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn6)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn6)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn6)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn6)), (((((((((locals.var_vgst2vtm_dn7 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn7)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn7)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn7)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn7)), (((((((((locals.var_vgst2vtm_dn8 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn8)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn8)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn8)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn8)), (((((((((locals.var_vgst2vtm_dn9 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn9)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn9)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn9)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn9)), (((((((((locals.var_vgst2vtm_dn10 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn10)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn10)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn10)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn10)), (((((((((locals.var_vgst2vtm_dn11 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn11)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign47160_e80030 * locals.var_t3_dn11)) * locals.var_pvagfactor) + (assign47160_e80032 * locals.var_pvagfactor_dn11)) * locals.var_t5) + (assign47160_e80034 * locals.var_t5_dn11)),)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8, locals.var_vadibl_dn9, locals.var_vadibl_dn10, locals.var_vadibl_dn11,)
    }
};
        locals.var_vadibl = assign47160_e80038;
        locals.var_vadibl_dn3 = assign47160_e80038_d_n3;
        locals.var_vadibl_dn4 = assign47160_e80038_d_n4;
        locals.var_vadibl_dn5 = assign47160_e80038_d_n5;
        locals.var_vadibl_dn6 = assign47160_e80038_d_n6;
        locals.var_vadibl_dn7 = assign47160_e80038_d_n7;
        locals.var_vadibl_dn8 = assign47160_e80038_d_n8;
        locals.var_vadibl_dn9 = assign47160_e80038_d_n9;
        locals.var_vadibl_dn10 = assign47160_e80038_d_n10;
        locals.var_vadibl_dn11 = assign47160_e80038_d_n11;
        locals.var_vadibl_rv = 0.0;

        let (assign47170_e80049, assign47170_e80049_d_n3, assign47170_e80049_d_n4, assign47170_e80049_d_n5, assign47170_e80049_d_n6, assign47170_e80049_d_n7, assign47170_e80049_d_n8, assign47170_e80049_d_n9, assign47170_e80049_d_n10, assign47170_e80049_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard729 != 0.0)) {
        let assign47170_e80046: f64 = (locals.var_diffvds / locals.var_vadibl);
        let assign47170_e80047: f64 = (1.0 + assign47170_e80046);
        (assign47170_e80047, (((locals.var_diffvds_dn3 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn3)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn4 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn4)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn5 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn5)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn6 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn6)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn7 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn7)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn8 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn8)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn9 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn9)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn10 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn10)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn11 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn11)) / (locals.var_vadibl * locals.var_vadibl)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47170_e80049;
        locals.var_moc_dn3 = assign47170_e80049_d_n3;
        locals.var_moc_dn4 = assign47170_e80049_d_n4;
        locals.var_moc_dn5 = assign47170_e80049_d_n5;
        locals.var_moc_dn6 = assign47170_e80049_d_n6;
        locals.var_moc_dn7 = assign47170_e80049_d_n7;
        locals.var_moc_dn8 = assign47170_e80049_d_n8;
        locals.var_moc_dn9 = assign47170_e80049_d_n9;
        locals.var_moc_dn10 = assign47170_e80049_d_n10;
        locals.var_moc_dn11 = assign47170_e80049_d_n11;
        locals.var_moc_rv = 0.0;

        let (assign47180_e80057, assign47180_e80057_d_n3, assign47180_e80057_d_n4, assign47180_e80057_d_n5, assign47180_e80057_d_n6, assign47180_e80057_d_n7, assign47180_e80057_d_n8, assign47180_e80057_d_n9, assign47180_e80057_d_n10, assign47180_e80057_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard729 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47180_e80057;
        locals.var_moc_dn3 = assign47180_e80057_d_n3;
        locals.var_moc_dn4 = assign47180_e80057_d_n4;
        locals.var_moc_dn5 = assign47180_e80057_d_n5;
        locals.var_moc_dn6 = assign47180_e80057_d_n6;
        locals.var_moc_dn7 = assign47180_e80057_d_n7;
        locals.var_moc_dn8 = assign47180_e80057_d_n8;
        locals.var_moc_dn9 = assign47180_e80057_d_n9;
        locals.var_moc_dn10 = assign47180_e80057_d_n10;
        locals.var_moc_dn11 = assign47180_e80057_d_n11;
        locals.var_moc_rv = 0.0;

        let assign47190_e80060: f64 = if locals.var_fprout_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard730 = assign47190_e80060;
        locals.var_guard730_rv = 0.0;

        let (assign47200_e80067, assign47200_e80067_d_n3, assign47200_e80067_d_n4, assign47200_e80067_d_n5, assign47200_e80067_d_n6, assign47200_e80067_d_n7, assign47200_e80067_d_n8, assign47200_e80067_d_n9, assign47200_e80067_d_n10, assign47200_e80067_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard730 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fp, locals.var_fp_dn3, locals.var_fp_dn4, locals.var_fp_dn5, locals.var_fp_dn6, locals.var_fp_dn7, locals.var_fp_dn8, locals.var_fp_dn9, locals.var_fp_dn10, locals.var_fp_dn11,)
    }
};
        locals.var_fp = assign47200_e80067;
        locals.var_fp_dn3 = assign47200_e80067_d_n3;
        locals.var_fp_dn4 = assign47200_e80067_d_n4;
        locals.var_fp_dn5 = assign47200_e80067_d_n5;
        locals.var_fp_dn6 = assign47200_e80067_d_n6;
        locals.var_fp_dn7 = assign47200_e80067_d_n7;
        locals.var_fp_dn8 = assign47200_e80067_d_n8;
        locals.var_fp_dn9 = assign47200_e80067_d_n9;
        locals.var_fp_dn10 = assign47200_e80067_d_n10;
        locals.var_fp_dn11 = assign47200_e80067_d_n11;
        locals.var_fp_rv = 0.0;

        let (assign47210_e80080, assign47210_e80080_d_n3, assign47210_e80080_d_n4, assign47210_e80080_d_n5, assign47210_e80080_d_n6, assign47210_e80080_d_n7, assign47210_e80080_d_n8, assign47210_e80080_d_n9, assign47210_e80080_d_n10, assign47210_e80080_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard730 == 0.0)) {
        let assign47210_e80075: f64 = (locals.var_leff).sqrt();
        let assign47210_e80076: f64 = (locals.var_fprout_i * assign47210_e80075);
        let assign47210_e80078: f64 = (assign47210_e80076 / locals.var_vgst2vtm);
        (assign47210_e80078, (-((assign47210_e80076 * locals.var_vgst2vtm_dn3) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn4) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn5) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn6) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn7) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn8) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn9) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn10) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign47210_e80076 * locals.var_vgst2vtm_dn11) / (locals.var_vgst2vtm * locals.var_vgst2vtm))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign47210_e80080;
        locals.var_t9_dn3 = assign47210_e80080_d_n3;
        locals.var_t9_dn4 = assign47210_e80080_d_n4;
        locals.var_t9_dn5 = assign47210_e80080_d_n5;
        locals.var_t9_dn6 = assign47210_e80080_d_n6;
        locals.var_t9_dn7 = assign47210_e80080_d_n7;
        locals.var_t9_dn8 = assign47210_e80080_d_n8;
        locals.var_t9_dn9 = assign47210_e80080_d_n9;
        locals.var_t9_dn10 = assign47210_e80080_d_n10;
        locals.var_t9_dn11 = assign47210_e80080_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign47220_e80092, assign47220_e80092_d_n3, assign47220_e80092_d_n4, assign47220_e80092_d_n5, assign47220_e80092_d_n6, assign47220_e80092_d_n7, assign47220_e80092_d_n8, assign47220_e80092_d_n9, assign47220_e80092_d_n10, assign47220_e80092_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard730 == 0.0)) {
        let assign47220_e80089: f64 = (1.0 + locals.var_t9);
        let assign47220_e80090: f64 = (1.0 / assign47220_e80089);
        (assign47220_e80090, (-(locals.var_t9_dn3 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn4 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn5 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn6 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn7 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn8 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn9 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn10 / (assign47220_e80089 * assign47220_e80089))), (-(locals.var_t9_dn11 / (assign47220_e80089 * assign47220_e80089))),)
    } else {
        (locals.var_fp, locals.var_fp_dn3, locals.var_fp_dn4, locals.var_fp_dn5, locals.var_fp_dn6, locals.var_fp_dn7, locals.var_fp_dn8, locals.var_fp_dn9, locals.var_fp_dn10, locals.var_fp_dn11,)
    }
};
        locals.var_fp = assign47220_e80092;
        locals.var_fp_dn3 = assign47220_e80092_d_n3;
        locals.var_fp_dn4 = assign47220_e80092_d_n4;
        locals.var_fp_dn5 = assign47220_e80092_d_n5;
        locals.var_fp_dn6 = assign47220_e80092_d_n6;
        locals.var_fp_dn7 = assign47220_e80092_d_n7;
        locals.var_fp_dn8 = assign47220_e80092_d_n8;
        locals.var_fp_dn9 = assign47220_e80092_d_n9;
        locals.var_fp_dn10 = assign47220_e80092_d_n10;
        locals.var_fp_dn11 = assign47220_e80092_d_n11;
        locals.var_fp_rv = 0.0;

        let (assign47230_e80099, assign47230_e80099_d_n3, assign47230_e80099_d_n4, assign47230_e80099_d_n5, assign47230_e80099_d_n6, assign47230_e80099_d_n7, assign47230_e80099_d_n8, assign47230_e80099_d_n9, assign47230_e80099_d_n10, assign47230_e80099_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47230_e80097: f64 = (locals.var_vdssat + locals.var_esatl);
        (assign47230_e80097, (locals.var_vdssat_dn3 + locals.var_esatl_dn3), (locals.var_vdssat_dn4 + locals.var_esatl_dn4), (locals.var_vdssat_dn5 + locals.var_esatl_dn5), (locals.var_vdssat_dn6 + locals.var_esatl_dn6), (locals.var_vdssat_dn7 + locals.var_esatl_dn7), (locals.var_vdssat_dn8 + locals.var_esatl_dn8), (locals.var_vdssat_dn9 + locals.var_esatl_dn9), (locals.var_vdssat_dn10 + locals.var_esatl_dn10), (locals.var_vdssat_dn11 + locals.var_esatl_dn11),)
    } else {
        (locals.var_vasat, locals.var_vasat_dn3, locals.var_vasat_dn4, locals.var_vasat_dn5, locals.var_vasat_dn6, locals.var_vasat_dn7, locals.var_vasat_dn8, locals.var_vasat_dn9, locals.var_vasat_dn10, locals.var_vasat_dn11,)
    }
};
        locals.var_vasat = assign47230_e80099;
        locals.var_vasat_dn3 = assign47230_e80099_d_n3;
        locals.var_vasat_dn4 = assign47230_e80099_d_n4;
        locals.var_vasat_dn5 = assign47230_e80099_d_n5;
        locals.var_vasat_dn6 = assign47230_e80099_d_n6;
        locals.var_vasat_dn7 = assign47230_e80099_d_n7;
        locals.var_vasat_dn8 = assign47230_e80099_d_n8;
        locals.var_vasat_dn9 = assign47230_e80099_d_n9;
        locals.var_vasat_dn10 = assign47230_e80099_d_n10;
        locals.var_vasat_dn11 = assign47230_e80099_d_n11;
        locals.var_vasat_rv = 0.0;

        let assign47240_e80102: f64 = if locals.var_pclm_a > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard731 = assign47240_e80102;
        locals.var_guard731_rv = 0.0;

        let assign47250_e80105: f64 = if p.p414 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard732 = assign47250_e80105;
        locals.var_guard732_rv = 0.0;

        let (assign47260_e80124, assign47260_e80124_d_n3, assign47260_e80124_d_n4, assign47260_e80124_d_n5, assign47260_e80124_d_n6, assign47260_e80124_d_n7, assign47260_e80124_d_n8, assign47260_e80124_d_n9, assign47260_e80124_d_n10, assign47260_e80124_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard731 != 0.0)) && (locals.var_guard732 != 0.0)) {
        let assign47260_e80116: f64 = (p.p414 * locals.var_qia);
        let assign47260_e80118: f64 = (assign47260_e80116 / locals.var_esatl);
        let assign47260_e80119: f64 = (1.0 - assign47260_e80118);
        let assign47260_e80120: f64 = (locals.var_pclm_a / assign47260_e80119);
        let assign47260_e80122: f64 = (assign47260_e80120 / locals.var_fp);
        (assign47260_e80122, ((((((locals.var_pclm_a_dn3 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn3) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn4 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn4) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn5 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn5) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn6 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn6) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn7 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn7) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn8 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn8) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn9 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn9) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn10 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn10) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn11 * assign47260_e80119) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn11) * locals.var_esatl) - (assign47260_e80116 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl))))) / (assign47260_e80119 * assign47260_e80119)) * locals.var_fp) - (assign47260_e80120 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47260_e80124;
        locals.var_t1_dn3 = assign47260_e80124_d_n3;
        locals.var_t1_dn4 = assign47260_e80124_d_n4;
        locals.var_t1_dn5 = assign47260_e80124_d_n5;
        locals.var_t1_dn6 = assign47260_e80124_d_n6;
        locals.var_t1_dn7 = assign47260_e80124_d_n7;
        locals.var_t1_dn8 = assign47260_e80124_d_n8;
        locals.var_t1_dn9 = assign47260_e80124_d_n9;
        locals.var_t1_dn10 = assign47260_e80124_d_n10;
        locals.var_t1_dn11 = assign47260_e80124_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47270_e80144, assign47270_e80144_d_n3, assign47270_e80144_d_n4, assign47270_e80144_d_n5, assign47270_e80144_d_n6, assign47270_e80144_d_n7, assign47270_e80144_d_n8, assign47270_e80144_d_n9, assign47270_e80144_d_n10, assign47270_e80144_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard731 != 0.0)) && (locals.var_guard732 == 0.0)) {
        let assign47270_e80136: f64 = (p.p414 * locals.var_qia);
        let assign47270_e80138: f64 = (assign47270_e80136 / locals.var_esatl);
        let assign47270_e80139: f64 = (1.0 + assign47270_e80138);
        let assign47270_e80140: f64 = (locals.var_pclm_a * assign47270_e80139);
        let assign47270_e80142: f64 = (assign47270_e80140 / locals.var_fp);
        (assign47270_e80142, (((((locals.var_pclm_a_dn3 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn3) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn4 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn4) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn5 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn5) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn6 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn6) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn7 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn7) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn8 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn8) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn9 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn9) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn10 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn10) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn11 * assign47270_e80139) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn11) * locals.var_esatl) - (assign47270_e80136 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47270_e80140 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47270_e80144;
        locals.var_t1_dn3 = assign47270_e80144_d_n3;
        locals.var_t1_dn4 = assign47270_e80144_d_n4;
        locals.var_t1_dn5 = assign47270_e80144_d_n5;
        locals.var_t1_dn6 = assign47270_e80144_d_n6;
        locals.var_t1_dn7 = assign47270_e80144_d_n7;
        locals.var_t1_dn8 = assign47270_e80144_d_n8;
        locals.var_t1_dn9 = assign47270_e80144_d_n9;
        locals.var_t1_dn10 = assign47270_e80144_d_n10;
        locals.var_t1_dn11 = assign47270_e80144_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47280_e80164, assign47280_e80164_d_n3, assign47280_e80164_d_n4, assign47280_e80164_d_n5, assign47280_e80164_d_n6, assign47280_e80164_d_n7, assign47280_e80164_d_n8, assign47280_e80164_d_n9, assign47280_e80164_d_n10, assign47280_e80164_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard731 != 0.0)) {
        let assign47280_e80154: f64 = (locals.var_diffvds / locals.var_t1);
        let assign47280_e80156: f64 = (assign47280_e80154 / locals.var_vasat);
        let assign47280_e80157: f64 = (1.0 + assign47280_e80156);
        let assign47280_e80159: f64 = (assign47280_e80157).max(1e-38);
        let assign47280_e80160: f64 = (assign47280_e80159).ln();
        let assign47280_e80161: f64 = (locals.var_t1 * assign47280_e80160);
        let assign47280_e80162: f64 = (1.0 + assign47280_e80161);
        (assign47280_e80162, ((locals.var_t1_dn3 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn3 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn3)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn4 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn4 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn4)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn5 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn5 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn5)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn6 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn6 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn6)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn7 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn7 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn7)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn8 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn8 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn8)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn9 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn9 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn9)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn10 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn10 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn10)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))), ((locals.var_t1_dn11 * assign47280_e80160) + (locals.var_t1 * (if assign47280_e80157 >= 1e-38 { ((((((locals.var_diffvds_dn11 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign47280_e80154 * locals.var_vasat_dn11)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign47280_e80159))),)
    } else {
        (locals.var_mdl, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11,)
    }
};
        locals.var_mdl = assign47280_e80164;
        locals.var_mdl_dn3 = assign47280_e80164_d_n3;
        locals.var_mdl_dn4 = assign47280_e80164_d_n4;
        locals.var_mdl_dn5 = assign47280_e80164_d_n5;
        locals.var_mdl_dn6 = assign47280_e80164_d_n6;
        locals.var_mdl_dn7 = assign47280_e80164_d_n7;
        locals.var_mdl_dn8 = assign47280_e80164_d_n8;
        locals.var_mdl_dn9 = assign47280_e80164_d_n9;
        locals.var_mdl_dn10 = assign47280_e80164_d_n10;
        locals.var_mdl_dn11 = assign47280_e80164_d_n11;
        locals.var_mdl_rv = 0.0;

        let assign47290_e80167: f64 = if p.p414 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign47290_e80167;
        locals.var_guard733_rv = 0.0;

        let (assign47300_e80187, assign47300_e80187_d_n3, assign47300_e80187_d_n4, assign47300_e80187_d_n5, assign47300_e80187_d_n6, assign47300_e80187_d_n7, assign47300_e80187_d_n8, assign47300_e80187_d_n9, assign47300_e80187_d_n10, assign47300_e80187_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard731 == 0.0)) && (locals.var_guard733 != 0.0)) {
        let assign47300_e80179: f64 = (p.p414 * locals.var_qia);
        let assign47300_e80181: f64 = (assign47300_e80179 / locals.var_esatl);
        let assign47300_e80182: f64 = (1.0 - assign47300_e80181);
        let assign47300_e80183: f64 = (locals.var_pclm_a / assign47300_e80182);
        let assign47300_e80185: f64 = (assign47300_e80183 / locals.var_fp);
        (assign47300_e80185, ((((((locals.var_pclm_a_dn3 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn3) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn4 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn4) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn5 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn5) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn6 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn6) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn7 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn7) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn8 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn8) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn9 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn9) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn10 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn10) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn11 * assign47300_e80182) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qia_dn11) * locals.var_esatl) - (assign47300_e80179 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl))))) / (assign47300_e80182 * assign47300_e80182)) * locals.var_fp) - (assign47300_e80183 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47300_e80187;
        locals.var_t1_dn3 = assign47300_e80187_d_n3;
        locals.var_t1_dn4 = assign47300_e80187_d_n4;
        locals.var_t1_dn5 = assign47300_e80187_d_n5;
        locals.var_t1_dn6 = assign47300_e80187_d_n6;
        locals.var_t1_dn7 = assign47300_e80187_d_n7;
        locals.var_t1_dn8 = assign47300_e80187_d_n8;
        locals.var_t1_dn9 = assign47300_e80187_d_n9;
        locals.var_t1_dn10 = assign47300_e80187_d_n10;
        locals.var_t1_dn11 = assign47300_e80187_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47310_e80208, assign47310_e80208_d_n3, assign47310_e80208_d_n4, assign47310_e80208_d_n5, assign47310_e80208_d_n6, assign47310_e80208_d_n7, assign47310_e80208_d_n8, assign47310_e80208_d_n9, assign47310_e80208_d_n10, assign47310_e80208_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard731 == 0.0)) && (locals.var_guard733 == 0.0)) {
        let assign47310_e80200: f64 = (p.p414 * locals.var_qia);
        let assign47310_e80202: f64 = (assign47310_e80200 / locals.var_esatl);
        let assign47310_e80203: f64 = (1.0 + assign47310_e80202);
        let assign47310_e80204: f64 = (locals.var_pclm_a * assign47310_e80203);
        let assign47310_e80206: f64 = (assign47310_e80204 / locals.var_fp);
        (assign47310_e80206, (((((locals.var_pclm_a_dn3 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn3) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn4 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn4) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn5 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn5) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn6 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn6) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn7 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn7) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn8 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn8) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn9 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn9) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn10 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn10) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn11 * assign47310_e80203) + (locals.var_pclm_a * ((((p.p414 * locals.var_qia_dn11) * locals.var_esatl) - (assign47310_e80200 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign47310_e80204 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47310_e80208;
        locals.var_t1_dn3 = assign47310_e80208_d_n3;
        locals.var_t1_dn4 = assign47310_e80208_d_n4;
        locals.var_t1_dn5 = assign47310_e80208_d_n5;
        locals.var_t1_dn6 = assign47310_e80208_d_n6;
        locals.var_t1_dn7 = assign47310_e80208_d_n7;
        locals.var_t1_dn8 = assign47310_e80208_d_n8;
        locals.var_t1_dn9 = assign47310_e80208_d_n9;
        locals.var_t1_dn10 = assign47310_e80208_d_n10;
        locals.var_t1_dn11 = assign47310_e80208_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47320_e80218, assign47320_e80218_d_n3, assign47320_e80218_d_n4, assign47320_e80218_d_n5, assign47320_e80218_d_n6, assign47320_e80218_d_n7, assign47320_e80218_d_n8, assign47320_e80218_d_n9, assign47320_e80218_d_n10, assign47320_e80218_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard731 == 0.0)) {
        let assign47320_e80216: f64 = (1.0 + locals.var_t1);
        (assign47320_e80216, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_mdl, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11,)
    }
};
        locals.var_mdl = assign47320_e80218;
        locals.var_mdl_dn3 = assign47320_e80218_d_n3;
        locals.var_mdl_dn4 = assign47320_e80218_d_n4;
        locals.var_mdl_dn5 = assign47320_e80218_d_n5;
        locals.var_mdl_dn6 = assign47320_e80218_d_n6;
        locals.var_mdl_dn7 = assign47320_e80218_d_n7;
        locals.var_mdl_dn8 = assign47320_e80218_d_n8;
        locals.var_mdl_dn9 = assign47320_e80218_d_n9;
        locals.var_mdl_dn10 = assign47320_e80218_d_n10;
        locals.var_mdl_dn11 = assign47320_e80218_d_n11;
        locals.var_mdl_rv = 0.0;

        let (assign47330_e80225, assign47330_e80225_d_n3, assign47330_e80225_d_n4, assign47330_e80225_d_n5, assign47330_e80225_d_n6, assign47330_e80225_d_n7, assign47330_e80225_d_n8, assign47330_e80225_d_n9, assign47330_e80225_d_n10, assign47330_e80225_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47330_e80223: f64 = (locals.var_moc * locals.var_mdl);
        (assign47330_e80223, ((locals.var_moc_dn3 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn3)), ((locals.var_moc_dn4 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn4)), ((locals.var_moc_dn5 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn5)), ((locals.var_moc_dn6 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn6)), ((locals.var_moc_dn7 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn7)), ((locals.var_moc_dn8 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn8)), ((locals.var_moc_dn9 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn9)), ((locals.var_moc_dn10 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn10)), ((locals.var_moc_dn11 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47330_e80225;
        locals.var_moc_dn3 = assign47330_e80225_d_n3;
        locals.var_moc_dn4 = assign47330_e80225_d_n4;
        locals.var_moc_dn5 = assign47330_e80225_d_n5;
        locals.var_moc_dn6 = assign47330_e80225_d_n6;
        locals.var_moc_dn7 = assign47330_e80225_d_n7;
        locals.var_moc_dn8 = assign47330_e80225_d_n8;
        locals.var_moc_dn9 = assign47330_e80225_d_n9;
        locals.var_moc_dn10 = assign47330_e80225_d_n10;
        locals.var_moc_dn11 = assign47330_e80225_d_n11;
        locals.var_moc_rv = 0.0;

        let (assign47340_e80233, assign47340_e80233_d_n3, assign47340_e80233_d_n4, assign47340_e80233_d_n5, assign47340_e80233_d_n6, assign47340_e80233_d_n7, assign47340_e80233_d_n8, assign47340_e80233_d_n9, assign47340_e80233_d_n10, assign47340_e80233_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47340_e80230: f64 = (locals.var_pditsd_i * locals.var_vds);
        let assign47340_e80231: f64 = { let limited_exp_arg = assign47340_e80230; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign47340_e80231, 0.0, 0.0, 0.0, ({ let limited_exp_arg = assign47340_e80230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pditsd_i * locals.var_vds_dn6)), ({ let limited_exp_arg = assign47340_e80230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pditsd_i * locals.var_vds_dn7)), 0.0, 0.0, ({ let limited_exp_arg = assign47340_e80230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pditsd_i * locals.var_vds_dn10)), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47340_e80233;
        locals.var_t1_dn3 = assign47340_e80233_d_n3;
        locals.var_t1_dn4 = assign47340_e80233_d_n4;
        locals.var_t1_dn5 = assign47340_e80233_d_n5;
        locals.var_t1_dn6 = assign47340_e80233_d_n6;
        locals.var_t1_dn7 = assign47340_e80233_d_n7;
        locals.var_t1_dn8 = assign47340_e80233_d_n8;
        locals.var_t1_dn9 = assign47340_e80233_d_n9;
        locals.var_t1_dn10 = assign47340_e80233_d_n10;
        locals.var_t1_dn11 = assign47340_e80233_d_n11;
        locals.var_t1_rv = 0.0;

        let assign47350_e80236: f64 = if locals.var_pdits_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign47350_e80236;
        locals.var_guard734_rv = 0.0;

        let (assign47360_e80247, assign47360_e80247_d_n3, assign47360_e80247_d_n4, assign47360_e80247_d_n5, assign47360_e80247_d_n6, assign47360_e80247_d_n7, assign47360_e80247_d_n8, assign47360_e80247_d_n9, assign47360_e80247_d_n10, assign47360_e80247_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 != 0.0)) {
        let assign47360_e80244: f64 = (p.p433 * locals.var_leff);
        let assign47360_e80245: f64 = (1.0 + assign47360_e80244);
        (assign47360_e80245, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47360_e80247;
        locals.var_t2_dn3 = assign47360_e80247_d_n3;
        locals.var_t2_dn4 = assign47360_e80247_d_n4;
        locals.var_t2_dn5 = assign47360_e80247_d_n5;
        locals.var_t2_dn6 = assign47360_e80247_d_n6;
        locals.var_t2_dn7 = assign47360_e80247_d_n7;
        locals.var_t2_dn8 = assign47360_e80247_d_n8;
        locals.var_t2_dn9 = assign47360_e80247_d_n9;
        locals.var_t2_dn10 = assign47360_e80247_d_n10;
        locals.var_t2_dn11 = assign47360_e80247_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47370_e80260, assign47370_e80260_d_n3, assign47370_e80260_d_n4, assign47370_e80260_d_n5, assign47370_e80260_d_n6, assign47370_e80260_d_n7, assign47370_e80260_d_n8, assign47370_e80260_d_n9, assign47370_e80260_d_n10, assign47370_e80260_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 != 0.0)) {
        let assign47370_e80255: f64 = (locals.var_t2 * locals.var_t1);
        let assign47370_e80256: f64 = (1.0 + assign47370_e80255);
        let assign47370_e80258: f64 = (assign47370_e80256 / locals.var_pdits_i);
        (assign47370_e80258, (((locals.var_t2_dn3 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn3)) / locals.var_pdits_i), (((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)) / locals.var_pdits_i), (((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)) / locals.var_pdits_i), (((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)) / locals.var_pdits_i), (((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)) / locals.var_pdits_i), (((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)) / locals.var_pdits_i), (((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)) / locals.var_pdits_i), (((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)) / locals.var_pdits_i), (((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)) / locals.var_pdits_i),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign47370_e80260;
        locals.var_vadits_dn3 = assign47370_e80260_d_n3;
        locals.var_vadits_dn4 = assign47370_e80260_d_n4;
        locals.var_vadits_dn5 = assign47370_e80260_d_n5;
        locals.var_vadits_dn6 = assign47370_e80260_d_n6;
        locals.var_vadits_dn7 = assign47370_e80260_d_n7;
        locals.var_vadits_dn8 = assign47370_e80260_d_n8;
        locals.var_vadits_dn9 = assign47370_e80260_d_n9;
        locals.var_vadits_dn10 = assign47370_e80260_d_n10;
        locals.var_vadits_dn11 = assign47370_e80260_d_n11;
        locals.var_vadits_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_165(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47380_e80269, assign47380_e80269_d_n3, assign47380_e80269_d_n4, assign47380_e80269_d_n5, assign47380_e80269_d_n6, assign47380_e80269_d_n7, assign47380_e80269_d_n8, assign47380_e80269_d_n9, assign47380_e80269_d_n10, assign47380_e80269_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 != 0.0)) {
        let assign47380_e80267: f64 = (locals.var_vadits * locals.var_fp);
        (assign47380_e80267, ((locals.var_vadits_dn3 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn3)), ((locals.var_vadits_dn4 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn4)), ((locals.var_vadits_dn5 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn5)), ((locals.var_vadits_dn6 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn6)), ((locals.var_vadits_dn7 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn7)), ((locals.var_vadits_dn8 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn8)), ((locals.var_vadits_dn9 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn9)), ((locals.var_vadits_dn10 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn10)), ((locals.var_vadits_dn11 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn11)),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign47380_e80269;
        locals.var_vadits_dn3 = assign47380_e80269_d_n3;
        locals.var_vadits_dn4 = assign47380_e80269_d_n4;
        locals.var_vadits_dn5 = assign47380_e80269_d_n5;
        locals.var_vadits_dn6 = assign47380_e80269_d_n6;
        locals.var_vadits_dn7 = assign47380_e80269_d_n7;
        locals.var_vadits_dn8 = assign47380_e80269_d_n8;
        locals.var_vadits_dn9 = assign47380_e80269_d_n9;
        locals.var_vadits_dn10 = assign47380_e80269_d_n10;
        locals.var_vadits_dn11 = assign47380_e80269_d_n11;
        locals.var_vadits_rv = 0.0;

        let (assign47390_e80277, assign47390_e80277_d_n3, assign47390_e80277_d_n4, assign47390_e80277_d_n5, assign47390_e80277_d_n6, assign47390_e80277_d_n7, assign47390_e80277_d_n8, assign47390_e80277_d_n9, assign47390_e80277_d_n10, assign47390_e80277_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard734 == 0.0)) {
        (5.540622384e34, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign47390_e80277;
        locals.var_vadits_dn3 = assign47390_e80277_d_n3;
        locals.var_vadits_dn4 = assign47390_e80277_d_n4;
        locals.var_vadits_dn5 = assign47390_e80277_d_n5;
        locals.var_vadits_dn6 = assign47390_e80277_d_n6;
        locals.var_vadits_dn7 = assign47390_e80277_d_n7;
        locals.var_vadits_dn8 = assign47390_e80277_d_n8;
        locals.var_vadits_dn9 = assign47390_e80277_d_n9;
        locals.var_vadits_dn10 = assign47390_e80277_d_n10;
        locals.var_vadits_dn11 = assign47390_e80277_d_n11;
        locals.var_vadits_rv = 0.0;

        let (assign47400_e80284, assign47400_e80284_d_n3, assign47400_e80284_d_n4, assign47400_e80284_d_n5, assign47400_e80284_d_n6, assign47400_e80284_d_n7, assign47400_e80284_d_n8, assign47400_e80284_d_n9, assign47400_e80284_d_n10, assign47400_e80284_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47400_e80282: f64 = (locals.var_diffvds / locals.var_vadits);
        (assign47400_e80282, (((locals.var_diffvds_dn3 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn3)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn4 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn4)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn5 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn5)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn6 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn6)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn7 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn7)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn8 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn8)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn9 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn9)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn10 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn10)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn11 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn11)) / (locals.var_vadits * locals.var_vadits)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47400_e80284;
        locals.var_t4_dn3 = assign47400_e80284_d_n3;
        locals.var_t4_dn4 = assign47400_e80284_d_n4;
        locals.var_t4_dn5 = assign47400_e80284_d_n5;
        locals.var_t4_dn6 = assign47400_e80284_d_n6;
        locals.var_t4_dn7 = assign47400_e80284_d_n7;
        locals.var_t4_dn8 = assign47400_e80284_d_n8;
        locals.var_t4_dn9 = assign47400_e80284_d_n9;
        locals.var_t4_dn10 = assign47400_e80284_d_n10;
        locals.var_t4_dn11 = assign47400_e80284_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47410_e80291, assign47410_e80291_d_n3, assign47410_e80291_d_n4, assign47410_e80291_d_n5, assign47410_e80291_d_n6, assign47410_e80291_d_n7, assign47410_e80291_d_n8, assign47410_e80291_d_n9, assign47410_e80291_d_n10, assign47410_e80291_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47410_e80289: f64 = (1.0 + locals.var_t4);
        (assign47410_e80289, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47410_e80291;
        locals.var_t0_dn3 = assign47410_e80291_d_n3;
        locals.var_t0_dn4 = assign47410_e80291_d_n4;
        locals.var_t0_dn5 = assign47410_e80291_d_n5;
        locals.var_t0_dn6 = assign47410_e80291_d_n6;
        locals.var_t0_dn7 = assign47410_e80291_d_n7;
        locals.var_t0_dn8 = assign47410_e80291_d_n8;
        locals.var_t0_dn9 = assign47410_e80291_d_n9;
        locals.var_t0_dn10 = assign47410_e80291_d_n10;
        locals.var_t0_dn11 = assign47410_e80291_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47420_e80298, assign47420_e80298_d_n3, assign47420_e80298_d_n4, assign47420_e80298_d_n5, assign47420_e80298_d_n6, assign47420_e80298_d_n7, assign47420_e80298_d_n8, assign47420_e80298_d_n9, assign47420_e80298_d_n10, assign47420_e80298_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47420_e80296: f64 = (locals.var_moc * locals.var_t0);
        (assign47420_e80296, ((locals.var_moc_dn3 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn3)), ((locals.var_moc_dn4 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn4)), ((locals.var_moc_dn5 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn5)), ((locals.var_moc_dn6 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn6)), ((locals.var_moc_dn7 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn7)), ((locals.var_moc_dn8 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn8)), ((locals.var_moc_dn9 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn9)), ((locals.var_moc_dn10 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn10)), ((locals.var_moc_dn11 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47420_e80298;
        locals.var_moc_dn3 = assign47420_e80298_d_n3;
        locals.var_moc_dn4 = assign47420_e80298_d_n4;
        locals.var_moc_dn5 = assign47420_e80298_d_n5;
        locals.var_moc_dn6 = assign47420_e80298_d_n6;
        locals.var_moc_dn7 = assign47420_e80298_d_n7;
        locals.var_moc_dn8 = assign47420_e80298_d_n8;
        locals.var_moc_dn9 = assign47420_e80298_d_n9;
        locals.var_moc_dn10 = assign47420_e80298_d_n10;
        locals.var_moc_dn11 = assign47420_e80298_d_n11;
        locals.var_moc_rv = 0.0;

        let assign47430_e80301: f64 = if locals.var_pscbe2_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign47430_e80301;
        locals.var_guard735_rv = 0.0;

        let assign47440_e80305: f64 = (locals.var_pscbe1_i * locals.var_litl);
        let assign47440_e80307: f64 = (assign47440_e80305 / 80.0);
        let assign47440_e80308: f64 = if locals.var_diffvds > assign47440_e80307 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign47440_e80308;
        locals.var_guard736_rv = 0.0;

        let (assign47450_e80321, assign47450_e80321_d_n3, assign47450_e80321_d_n4, assign47450_e80321_d_n5, assign47450_e80321_d_n6, assign47450_e80321_d_n7, assign47450_e80321_d_n8, assign47450_e80321_d_n9, assign47450_e80321_d_n10, assign47450_e80321_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign47450_e80317: f64 = (locals.var_pscbe1_i * locals.var_litl);
        let assign47450_e80319: f64 = (assign47450_e80317 / locals.var_diffvds);
        (assign47450_e80319, (-((assign47450_e80317 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn4) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn5) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign47450_e80317 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47450_e80321;
        locals.var_t0_dn3 = assign47450_e80321_d_n3;
        locals.var_t0_dn4 = assign47450_e80321_d_n4;
        locals.var_t0_dn5 = assign47450_e80321_d_n5;
        locals.var_t0_dn6 = assign47450_e80321_d_n6;
        locals.var_t0_dn7 = assign47450_e80321_d_n7;
        locals.var_t0_dn8 = assign47450_e80321_d_n8;
        locals.var_t0_dn9 = assign47450_e80321_d_n9;
        locals.var_t0_dn10 = assign47450_e80321_d_n10;
        locals.var_t0_dn11 = assign47450_e80321_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47460_e80335, assign47460_e80335_d_n3, assign47460_e80335_d_n4, assign47460_e80335_d_n5, assign47460_e80335_d_n6, assign47460_e80335_d_n7, assign47460_e80335_d_n8, assign47460_e80335_d_n9, assign47460_e80335_d_n10, assign47460_e80335_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 != 0.0)) {
        let assign47460_e80330: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign47460_e80331: f64 = (locals.var_leff * assign47460_e80330);
        let assign47460_e80333: f64 = (assign47460_e80331 / locals.var_pscbe2_i);
        (assign47460_e80333, ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11)) / locals.var_pscbe2_i),)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign47460_e80335;
        locals.var_vascbe_dn3 = assign47460_e80335_d_n3;
        locals.var_vascbe_dn4 = assign47460_e80335_d_n4;
        locals.var_vascbe_dn5 = assign47460_e80335_d_n5;
        locals.var_vascbe_dn6 = assign47460_e80335_d_n6;
        locals.var_vascbe_dn7 = assign47460_e80335_d_n7;
        locals.var_vascbe_dn8 = assign47460_e80335_d_n8;
        locals.var_vascbe_dn9 = assign47460_e80335_d_n9;
        locals.var_vascbe_dn10 = assign47460_e80335_d_n10;
        locals.var_vascbe_dn11 = assign47460_e80335_d_n11;
        locals.var_vascbe_rv = 0.0;

        let (assign47470_e80349, assign47470_e80349_d_n3, assign47470_e80349_d_n4, assign47470_e80349_d_n5, assign47470_e80349_d_n6, assign47470_e80349_d_n7, assign47470_e80349_d_n8, assign47470_e80349_d_n9, assign47470_e80349_d_n10, assign47470_e80349_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard735 != 0.0)) && (locals.var_guard736 == 0.0)) {
        let assign47470_e80345: f64 = (5.540622384e34 * locals.var_leff);
        let assign47470_e80347: f64 = (assign47470_e80345 / locals.var_pscbe2_i);
        (assign47470_e80347, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign47470_e80349;
        locals.var_vascbe_dn3 = assign47470_e80349_d_n3;
        locals.var_vascbe_dn4 = assign47470_e80349_d_n4;
        locals.var_vascbe_dn5 = assign47470_e80349_d_n5;
        locals.var_vascbe_dn6 = assign47470_e80349_d_n6;
        locals.var_vascbe_dn7 = assign47470_e80349_d_n7;
        locals.var_vascbe_dn8 = assign47470_e80349_d_n8;
        locals.var_vascbe_dn9 = assign47470_e80349_d_n9;
        locals.var_vascbe_dn10 = assign47470_e80349_d_n10;
        locals.var_vascbe_dn11 = assign47470_e80349_d_n11;
        locals.var_vascbe_rv = 0.0;

        let (assign47480_e80357, assign47480_e80357_d_n3, assign47480_e80357_d_n4, assign47480_e80357_d_n5, assign47480_e80357_d_n6, assign47480_e80357_d_n7, assign47480_e80357_d_n8, assign47480_e80357_d_n9, assign47480_e80357_d_n10, assign47480_e80357_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard735 == 0.0)) {
        (5.540622384e34, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign47480_e80357;
        locals.var_vascbe_dn3 = assign47480_e80357_d_n3;
        locals.var_vascbe_dn4 = assign47480_e80357_d_n4;
        locals.var_vascbe_dn5 = assign47480_e80357_d_n5;
        locals.var_vascbe_dn6 = assign47480_e80357_d_n6;
        locals.var_vascbe_dn7 = assign47480_e80357_d_n7;
        locals.var_vascbe_dn8 = assign47480_e80357_d_n8;
        locals.var_vascbe_dn9 = assign47480_e80357_d_n9;
        locals.var_vascbe_dn10 = assign47480_e80357_d_n10;
        locals.var_vascbe_dn11 = assign47480_e80357_d_n11;
        locals.var_vascbe_rv = 0.0;

        let (assign47490_e80366, assign47490_e80366_d_n3, assign47490_e80366_d_n4, assign47490_e80366_d_n5, assign47490_e80366_d_n6, assign47490_e80366_d_n7, assign47490_e80366_d_n8, assign47490_e80366_d_n9, assign47490_e80366_d_n10, assign47490_e80366_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47490_e80363: f64 = (locals.var_diffvds / locals.var_vascbe);
        let assign47490_e80364: f64 = (1.0 + assign47490_e80363);
        (assign47490_e80364, (((locals.var_diffvds_dn3 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn3)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn4 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn4)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn5 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn5)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn6 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn6)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn7 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn7)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn8 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn8)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn9 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn9)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn10 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn10)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn11 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn11)) / (locals.var_vascbe * locals.var_vascbe)),)
    } else {
        (locals.var_mscbe, locals.var_mscbe_dn3, locals.var_mscbe_dn4, locals.var_mscbe_dn5, locals.var_mscbe_dn6, locals.var_mscbe_dn7, locals.var_mscbe_dn8, locals.var_mscbe_dn9, locals.var_mscbe_dn10, locals.var_mscbe_dn11,)
    }
};
        locals.var_mscbe = assign47490_e80366;
        locals.var_mscbe_dn3 = assign47490_e80366_d_n3;
        locals.var_mscbe_dn4 = assign47490_e80366_d_n4;
        locals.var_mscbe_dn5 = assign47490_e80366_d_n5;
        locals.var_mscbe_dn6 = assign47490_e80366_d_n6;
        locals.var_mscbe_dn7 = assign47490_e80366_d_n7;
        locals.var_mscbe_dn8 = assign47490_e80366_d_n8;
        locals.var_mscbe_dn9 = assign47490_e80366_d_n9;
        locals.var_mscbe_dn10 = assign47490_e80366_d_n10;
        locals.var_mscbe_dn11 = assign47490_e80366_d_n11;
        locals.var_mscbe_rv = 0.0;

        let (assign47500_e80373, assign47500_e80373_d_n3, assign47500_e80373_d_n4, assign47500_e80373_d_n5, assign47500_e80373_d_n6, assign47500_e80373_d_n7, assign47500_e80373_d_n8, assign47500_e80373_d_n9, assign47500_e80373_d_n10, assign47500_e80373_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47500_e80371: f64 = (locals.var_moc * locals.var_mscbe);
        (assign47500_e80371, ((locals.var_moc_dn3 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn3)), ((locals.var_moc_dn4 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn4)), ((locals.var_moc_dn5 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn5)), ((locals.var_moc_dn6 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn6)), ((locals.var_moc_dn7 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn7)), ((locals.var_moc_dn8 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn8)), ((locals.var_moc_dn9 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn9)), ((locals.var_moc_dn10 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn10)), ((locals.var_moc_dn11 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign47500_e80373;
        locals.var_moc_dn3 = assign47500_e80373_d_n3;
        locals.var_moc_dn4 = assign47500_e80373_d_n4;
        locals.var_moc_dn5 = assign47500_e80373_d_n5;
        locals.var_moc_dn6 = assign47500_e80373_d_n6;
        locals.var_moc_dn7 = assign47500_e80373_d_n7;
        locals.var_moc_dn8 = assign47500_e80373_d_n8;
        locals.var_moc_dn9 = assign47500_e80373_d_n9;
        locals.var_moc_dn10 = assign47500_e80373_d_n10;
        locals.var_moc_dn11 = assign47500_e80373_d_n11;
        locals.var_moc_rv = 0.0;

        let (assign47510_e80382, assign47510_e80382_d_n3, assign47510_e80382_d_n4, assign47510_e80382_d_n5, assign47510_e80382_d_n6, assign47510_e80382_d_n7, assign47510_e80382_d_n8, assign47510_e80382_d_n9, assign47510_e80382_d_n10, assign47510_e80382_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47510_e80379: f64 = (1.0 / locals.var_psat_a);
        let assign47510_e80380: f64 = (locals.var_dmob).powf(assign47510_e80379);
        (assign47510_e80380, if (-(locals.var_psat_a_dn3 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn3)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn3 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn3 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn4 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn4)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn4 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn4 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn5 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn5)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn5 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn5 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn6 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn6)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn6 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn6 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn7 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn7)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn7 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn7 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn8 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn8)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn8 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn8 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn9 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn9)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn9 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn9 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn10 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn10)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn10 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn10 / locals.var_dmob)))) }, if (-(locals.var_psat_a_dn11 / (locals.var_psat_a * locals.var_psat_a))) == 0.0 && ((assign47510_e80379) as f64).is_finite() && ((assign47510_e80379) as f64).fract() == 0.0 { if assign47510_e80379 == 0.0 { 0.0 } else { (assign47510_e80379 * ((locals.var_dmob).powf(assign47510_e80379 - 1.0) * locals.var_dmob_dn11)) } } else { (assign47510_e80380 * (((-(locals.var_psat_a_dn11 / (locals.var_psat_a * locals.var_psat_a))) * (locals.var_dmob).ln()) + (assign47510_e80379 * (locals.var_dmob_dn11 / locals.var_dmob)))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47510_e80382;
        locals.var_t0_dn3 = assign47510_e80382_d_n3;
        locals.var_t0_dn4 = assign47510_e80382_d_n4;
        locals.var_t0_dn5 = assign47510_e80382_d_n5;
        locals.var_t0_dn6 = assign47510_e80382_d_n6;
        locals.var_t0_dn7 = assign47510_e80382_d_n7;
        locals.var_t0_dn8 = assign47510_e80382_d_n8;
        locals.var_t0_dn9 = assign47510_e80382_d_n9;
        locals.var_t0_dn10 = assign47510_e80382_d_n10;
        locals.var_t0_dn11 = assign47510_e80382_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47520_e80389, assign47520_e80389_d_n3, assign47520_e80389_d_n4, assign47520_e80389_d_n5, assign47520_e80389_d_n6, assign47520_e80389_d_n7, assign47520_e80389_d_n8, assign47520_e80389_d_n9, assign47520_e80389_d_n10, assign47520_e80389_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47520_e80387: f64 = (locals.var_psatb_i * locals.var_vbsx);
        (assign47520_e80387, (locals.var_psatb_i * locals.var_vbsx_dn3), (locals.var_psatb_i * locals.var_vbsx_dn4), (locals.var_psatb_i * locals.var_vbsx_dn5), (locals.var_psatb_i * locals.var_vbsx_dn6), (locals.var_psatb_i * locals.var_vbsx_dn7), (locals.var_psatb_i * locals.var_vbsx_dn8), (locals.var_psatb_i * locals.var_vbsx_dn9), (locals.var_psatb_i * locals.var_vbsx_dn10), (locals.var_psatb_i * locals.var_vbsx_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign47520_e80389;
        locals.var_t11_dn3 = assign47520_e80389_d_n3;
        locals.var_t11_dn4 = assign47520_e80389_d_n4;
        locals.var_t11_dn5 = assign47520_e80389_d_n5;
        locals.var_t11_dn6 = assign47520_e80389_d_n6;
        locals.var_t11_dn7 = assign47520_e80389_d_n7;
        locals.var_t11_dn8 = assign47520_e80389_d_n8;
        locals.var_t11_dn9 = assign47520_e80389_d_n9;
        locals.var_t11_dn10 = assign47520_e80389_d_n10;
        locals.var_t11_dn11 = assign47520_e80389_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign47530_e80399, assign47530_e80399_d_n3, assign47530_e80399_d_n4, assign47530_e80399_d_n5, assign47530_e80399_d_n6, assign47530_e80399_d_n7, assign47530_e80399_d_n8, assign47530_e80399_d_n9, assign47530_e80399_d_n10, assign47530_e80399_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47530_e80395: f64 = (locals.var_t11 * locals.var_t11);
        let assign47530_e80396: f64 = (0.1 + assign47530_e80395);
        let assign47530_e80397: f64 = (assign47530_e80396).sqrt();
        (assign47530_e80397, (((locals.var_t11_dn3 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn3)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn4 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn4)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn5 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn5)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn6 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn6)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn7 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn7)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn8 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn8)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn9 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn9)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn10 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn10)) / (2.0 * assign47530_e80397)), (((locals.var_t11_dn11 * locals.var_t11) + (locals.var_t11 * locals.var_t11_dn11)) / (2.0 * assign47530_e80397)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign47530_e80399;
        locals.var_t12_dn3 = assign47530_e80399_d_n3;
        locals.var_t12_dn4 = assign47530_e80399_d_n4;
        locals.var_t12_dn5 = assign47530_e80399_d_n5;
        locals.var_t12_dn6 = assign47530_e80399_d_n6;
        locals.var_t12_dn7 = assign47530_e80399_d_n7;
        locals.var_t12_dn8 = assign47530_e80399_d_n8;
        locals.var_t12_dn9 = assign47530_e80399_d_n9;
        locals.var_t12_dn10 = assign47530_e80399_d_n10;
        locals.var_t12_dn11 = assign47530_e80399_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign47540_e80419, assign47540_e80419_d_n3, assign47540_e80419_d_n4, assign47540_e80419_d_n5, assign47540_e80419_d_n6, assign47540_e80419_d_n7, assign47540_e80419_d_n8, assign47540_e80419_d_n9, assign47540_e80419_d_n10, assign47540_e80419_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47540_e80405: f64 = (1.0 - locals.var_t11);
        let assign47540_e80408: f64 = (1.0 - locals.var_t11);
        let assign47540_e80411: f64 = (1.0 - locals.var_t11);
        let assign47540_e80412: f64 = (assign47540_e80408 * assign47540_e80411);
        let assign47540_e80414: f64 = (assign47540_e80412 + locals.var_t12);
        let assign47540_e80415: f64 = (assign47540_e80414).sqrt();
        let assign47540_e80416: f64 = (assign47540_e80405 + assign47540_e80415);
        let assign47540_e80417: f64 = (0.5 * assign47540_e80416);
        (assign47540_e80417, (0.5 * ((-locals.var_t11_dn3) + (((((-locals.var_t11_dn3) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn3))) + locals.var_t12_dn3) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn4) + (((((-locals.var_t11_dn4) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn4))) + locals.var_t12_dn4) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn5) + (((((-locals.var_t11_dn5) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn5))) + locals.var_t12_dn5) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn6) + (((((-locals.var_t11_dn6) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn6))) + locals.var_t12_dn6) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn7) + (((((-locals.var_t11_dn7) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn7))) + locals.var_t12_dn7) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn8) + (((((-locals.var_t11_dn8) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn8))) + locals.var_t12_dn8) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn9) + (((((-locals.var_t11_dn9) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn9))) + locals.var_t12_dn9) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn10) + (((((-locals.var_t11_dn10) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn10))) + locals.var_t12_dn10) / (2.0 * assign47540_e80415)))), (0.5 * ((-locals.var_t11_dn11) + (((((-locals.var_t11_dn11) * assign47540_e80411) + (assign47540_e80408 * (-locals.var_t11_dn11))) + locals.var_t12_dn11) / (2.0 * assign47540_e80415)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47540_e80419;
        locals.var_t1_dn3 = assign47540_e80419_d_n3;
        locals.var_t1_dn4 = assign47540_e80419_d_n4;
        locals.var_t1_dn5 = assign47540_e80419_d_n5;
        locals.var_t1_dn6 = assign47540_e80419_d_n6;
        locals.var_t1_dn7 = assign47540_e80419_d_n7;
        locals.var_t1_dn8 = assign47540_e80419_d_n8;
        locals.var_t1_dn9 = assign47540_e80419_d_n9;
        locals.var_t1_dn10 = assign47540_e80419_d_n10;
        locals.var_t1_dn11 = assign47540_e80419_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47550_e80438, assign47550_e80438_d_n3, assign47550_e80438_d_n4, assign47550_e80438_d_n5, assign47550_e80438_d_n6, assign47550_e80438_d_n7, assign47550_e80438_d_n8, assign47550_e80438_d_n9, assign47550_e80438_d_n10, assign47550_e80438_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47550_e80424: f64 = (10.0 * p.p497);
        let assign47550_e80426: f64 = (assign47550_e80424 * locals.var_qia);
        let assign47550_e80428: f64 = (assign47550_e80426 * locals.var_t1);
        let assign47550_e80431: f64 = (10.0 * p.p497);
        let assign47550_e80434: f64 = (locals.var_qia * locals.var_t1);
        let assign47550_e80435: f64 = (assign47550_e80431 + assign47550_e80434);
        let assign47550_e80436: f64 = (assign47550_e80428 / assign47550_e80435);
        (assign47550_e80436, ((((((assign47550_e80424 * locals.var_qia_dn3) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn3)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn3 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn3)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn4) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn4)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn4 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn4)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn5) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn5)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn5 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn5)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn6) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn6)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn6 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn6)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn7) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn7)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn7 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn7)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn8) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn8)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn8 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn8)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn9) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn9)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn9 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn9)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn10) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn10)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn10 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn10)))) / (assign47550_e80435 * assign47550_e80435)), ((((((assign47550_e80424 * locals.var_qia_dn11) * locals.var_t1) + (assign47550_e80426 * locals.var_t1_dn11)) * assign47550_e80435) - (assign47550_e80428 * ((locals.var_qia_dn11 * locals.var_t1) + (locals.var_qia * locals.var_t1_dn11)))) / (assign47550_e80435 * assign47550_e80435)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47550_e80438;
        locals.var_t2_dn3 = assign47550_e80438_d_n3;
        locals.var_t2_dn4 = assign47550_e80438_d_n4;
        locals.var_t2_dn5 = assign47550_e80438_d_n5;
        locals.var_t2_dn6 = assign47550_e80438_d_n6;
        locals.var_t2_dn7 = assign47550_e80438_d_n7;
        locals.var_t2_dn8 = assign47550_e80438_d_n8;
        locals.var_t2_dn9 = assign47550_e80438_d_n9;
        locals.var_t2_dn10 = assign47550_e80438_d_n10;
        locals.var_t2_dn11 = assign47550_e80438_d_n11;
        locals.var_t2_rv = 0.0;

        let assign47560_e80441: f64 = if locals.var_ptwg_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign47560_e80441;
        locals.var_guard737_rv = 0.0;

        let (assign47570_e80466, assign47570_e80466_d_n3, assign47570_e80466_d_n4, assign47570_e80466_d_n5, assign47570_e80466_d_n6, assign47570_e80466_d_n7, assign47570_e80466_d_n8, assign47570_e80466_d_n9, assign47570_e80466_d_n10, assign47570_e80466_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard737 != 0.0)) {
        let assign47570_e80449: f64 = (locals.var_u0_a / locals.var_t0);
        let assign47570_e80451: f64 = (assign47570_e80449 * locals.var_nvt);
        let assign47570_e80454: f64 = (locals.var_vsat_a * locals.var_leff);
        let assign47570_e80455: f64 = (assign47570_e80451 / assign47570_e80454);
        let assign47570_e80456: f64 = (2.0 * assign47570_e80455);
        let assign47570_e80461: f64 = (locals.var_ptwg_a * locals.var_t2);
        let assign47570_e80462: f64 = (1.0 - assign47570_e80461);
        let assign47570_e80463: f64 = (1.0 / assign47570_e80462);
        let assign47570_e80464: f64 = (assign47570_e80456 * assign47570_e80463);
        (assign47570_e80464, (((2.0 * ((((((((locals.var_u0_a_dn3 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn3)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn3 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn3 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn3))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn4 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn4)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn4 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn4 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn4))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn5 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn5)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn5 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn5 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn5))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn6 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn6)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn6 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn6 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn6))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn7 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn7)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn7 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn7 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn7))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn8 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn8)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn8 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn8 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn8))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn9 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn9)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn9 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn9 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn9))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn10 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn10)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn10 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn10 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn10))) / (assign47570_e80462 * assign47570_e80462))))), (((2.0 * ((((((((locals.var_u0_a_dn11 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47570_e80449 * locals.var_nvt_dn11)) * assign47570_e80454) - (assign47570_e80451 * (locals.var_vsat_a_dn11 * locals.var_leff))) / (assign47570_e80454 * assign47570_e80454))) * assign47570_e80463) + (assign47570_e80456 * (-((-((locals.var_ptwg_a_dn11 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn11))) / (assign47570_e80462 * assign47570_e80462))))),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign47570_e80466;
        locals.var_lambdac_dn3 = assign47570_e80466_d_n3;
        locals.var_lambdac_dn4 = assign47570_e80466_d_n4;
        locals.var_lambdac_dn5 = assign47570_e80466_d_n5;
        locals.var_lambdac_dn6 = assign47570_e80466_d_n6;
        locals.var_lambdac_dn7 = assign47570_e80466_d_n7;
        locals.var_lambdac_dn8 = assign47570_e80466_d_n8;
        locals.var_lambdac_dn9 = assign47570_e80466_d_n9;
        locals.var_lambdac_dn10 = assign47570_e80466_d_n10;
        locals.var_lambdac_dn11 = assign47570_e80466_d_n11;
        locals.var_lambdac_rv = 0.0;

        let (assign47580_e80490, assign47580_e80490_d_n3, assign47580_e80490_d_n4, assign47580_e80490_d_n5, assign47580_e80490_d_n6, assign47580_e80490_d_n7, assign47580_e80490_d_n8, assign47580_e80490_d_n9, assign47580_e80490_d_n10, assign47580_e80490_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard737 == 0.0)) {
        let assign47580_e80475: f64 = (locals.var_u0_a / locals.var_t0);
        let assign47580_e80477: f64 = (assign47580_e80475 * locals.var_nvt);
        let assign47580_e80480: f64 = (locals.var_vsat_a * locals.var_leff);
        let assign47580_e80481: f64 = (assign47580_e80477 / assign47580_e80480);
        let assign47580_e80482: f64 = (2.0 * assign47580_e80481);
        let assign47580_e80486: f64 = (locals.var_ptwg_a * locals.var_t2);
        let assign47580_e80487: f64 = (1.0 + assign47580_e80486);
        let assign47580_e80488: f64 = (assign47580_e80482 * assign47580_e80487);
        (assign47580_e80488, (((2.0 * ((((((((locals.var_u0_a_dn3 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn3)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn3 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn3 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn3)))), (((2.0 * ((((((((locals.var_u0_a_dn4 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn4)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn4 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn4 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn4)))), (((2.0 * ((((((((locals.var_u0_a_dn5 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn5)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn5 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn5 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn5)))), (((2.0 * ((((((((locals.var_u0_a_dn6 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn6)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn6 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn6 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn6)))), (((2.0 * ((((((((locals.var_u0_a_dn7 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn7)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn7 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn7 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn7)))), (((2.0 * ((((((((locals.var_u0_a_dn8 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn8)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn8 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn8 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn8)))), (((2.0 * ((((((((locals.var_u0_a_dn9 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn9)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn9 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn9 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn9)))), (((2.0 * ((((((((locals.var_u0_a_dn10 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn10)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn10 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn10 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn10)))), (((2.0 * ((((((((locals.var_u0_a_dn11 * locals.var_t0) - (locals.var_u0_a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_nvt) + (assign47580_e80475 * locals.var_nvt_dn11)) * assign47580_e80480) - (assign47580_e80477 * (locals.var_vsat_a_dn11 * locals.var_leff))) / (assign47580_e80480 * assign47580_e80480))) * assign47580_e80487) + (assign47580_e80482 * ((locals.var_ptwg_a_dn11 * locals.var_t2) + (locals.var_ptwg_a * locals.var_t2_dn11)))),)
    } else {
        (locals.var_lambdac, locals.var_lambdac_dn3, locals.var_lambdac_dn4, locals.var_lambdac_dn5, locals.var_lambdac_dn6, locals.var_lambdac_dn7, locals.var_lambdac_dn8, locals.var_lambdac_dn9, locals.var_lambdac_dn10, locals.var_lambdac_dn11,)
    }
};
        locals.var_lambdac = assign47580_e80490;
        locals.var_lambdac_dn3 = assign47580_e80490_d_n3;
        locals.var_lambdac_dn4 = assign47580_e80490_d_n4;
        locals.var_lambdac_dn5 = assign47580_e80490_d_n5;
        locals.var_lambdac_dn6 = assign47580_e80490_d_n6;
        locals.var_lambdac_dn7 = assign47580_e80490_d_n7;
        locals.var_lambdac_dn8 = assign47580_e80490_d_n8;
        locals.var_lambdac_dn9 = assign47580_e80490_d_n9;
        locals.var_lambdac_dn10 = assign47580_e80490_d_n10;
        locals.var_lambdac_dn11 = assign47580_e80490_d_n11;
        locals.var_lambdac_rv = 0.0;

        let (assign47590_e80501, assign47590_e80501_d_n3, assign47590_e80501_d_n4, assign47590_e80501_d_n5, assign47590_e80501_d_n6, assign47590_e80501_d_n7, assign47590_e80501_d_n8, assign47590_e80501_d_n9, assign47590_e80501_d_n10, assign47590_e80501_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47590_e80495: f64 = (2.0 * locals.var_lambdac);
        let assign47590_e80498: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign47590_e80499: f64 = (assign47590_e80495 * assign47590_e80498);
        (assign47590_e80499, (((2.0 * locals.var_lambdac_dn3) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn3 - locals.var_qdeff_dn3))), (((2.0 * locals.var_lambdac_dn4) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn4 - locals.var_qdeff_dn4))), (((2.0 * locals.var_lambdac_dn5) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn5 - locals.var_qdeff_dn5))), (((2.0 * locals.var_lambdac_dn6) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn6 - locals.var_qdeff_dn6))), (((2.0 * locals.var_lambdac_dn7) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn7 - locals.var_qdeff_dn7))), (((2.0 * locals.var_lambdac_dn8) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn8 - locals.var_qdeff_dn8))), (((2.0 * locals.var_lambdac_dn9) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn9 - locals.var_qdeff_dn9))), (((2.0 * locals.var_lambdac_dn10) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn10 - locals.var_qdeff_dn10))), (((2.0 * locals.var_lambdac_dn11) * assign47590_e80498) + (assign47590_e80495 * (locals.var_qs_1_dn11 - locals.var_qdeff_dn11))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47590_e80501;
        locals.var_t1_dn3 = assign47590_e80501_d_n3;
        locals.var_t1_dn4 = assign47590_e80501_d_n4;
        locals.var_t1_dn5 = assign47590_e80501_d_n5;
        locals.var_t1_dn6 = assign47590_e80501_d_n6;
        locals.var_t1_dn7 = assign47590_e80501_d_n7;
        locals.var_t1_dn8 = assign47590_e80501_d_n8;
        locals.var_t1_dn9 = assign47590_e80501_d_n9;
        locals.var_t1_dn10 = assign47590_e80501_d_n10;
        locals.var_t1_dn11 = assign47590_e80501_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47600_e80511, assign47600_e80511_d_n3, assign47600_e80511_d_n4, assign47600_e80511_d_n5, assign47600_e80511_d_n6, assign47600_e80511_d_n7, assign47600_e80511_d_n8, assign47600_e80511_d_n9, assign47600_e80511_d_n10, assign47600_e80511_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47600_e80507: f64 = (locals.var_t1 * locals.var_t1);
        let assign47600_e80508: f64 = (1.0 + assign47600_e80507);
        let assign47600_e80509: f64 = (assign47600_e80508).sqrt();
        (assign47600_e80509, (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign47600_e80509)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign47600_e80509)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47600_e80511;
        locals.var_t2_dn3 = assign47600_e80511_d_n3;
        locals.var_t2_dn4 = assign47600_e80511_d_n4;
        locals.var_t2_dn5 = assign47600_e80511_d_n5;
        locals.var_t2_dn6 = assign47600_e80511_d_n6;
        locals.var_t2_dn7 = assign47600_e80511_d_n7;
        locals.var_t2_dn8 = assign47600_e80511_d_n8;
        locals.var_t2_dn9 = assign47600_e80511_d_n9;
        locals.var_t2_dn10 = assign47600_e80511_d_n10;
        locals.var_t2_dn11 = assign47600_e80511_d_n11;
        locals.var_t2_rv = 0.0;

        let assign47610_e80514: f64 = if locals.var_t1 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign47610_e80514;
        locals.var_guard738_rv = 0.0;

        let (assign47620_e80530, assign47620_e80530_d_n3, assign47620_e80530_d_n4, assign47620_e80530_d_n5, assign47620_e80530_d_n6, assign47620_e80530_d_n7, assign47620_e80530_d_n8, assign47620_e80530_d_n9, assign47620_e80530_d_n10, assign47620_e80530_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard738 != 0.0)) {
        let assign47620_e80523: f64 = (1.0 / locals.var_t1);
        let assign47620_e80525: f64 = (locals.var_t1).asinh();
        let assign47620_e80526: f64 = (assign47620_e80523 * assign47620_e80525);
        let assign47620_e80527: f64 = (locals.var_t2 + assign47620_e80526);
        let assign47620_e80528: f64 = (0.5 * assign47620_e80527);
        (assign47620_e80528, (0.5 * (locals.var_t2_dn3 + (((-(locals.var_t1_dn3 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn3 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn4 + (((-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn4 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn5 + (((-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn5 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn6 + (((-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn6 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn7 + (((-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn7 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn8 + (((-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn8 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn9 + (((-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn9 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn10 + (((-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn10 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))), (0.5 * (locals.var_t2_dn11 + (((-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))) * assign47620_e80525) + (assign47620_e80523 * (locals.var_t1_dn11 / ((locals.var_t1 * locals.var_t1) + 1.0).sqrt()))))),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign47620_e80530;
        locals.var_dvsat_dn3 = assign47620_e80530_d_n3;
        locals.var_dvsat_dn4 = assign47620_e80530_d_n4;
        locals.var_dvsat_dn5 = assign47620_e80530_d_n5;
        locals.var_dvsat_dn6 = assign47620_e80530_d_n6;
        locals.var_dvsat_dn7 = assign47620_e80530_d_n7;
        locals.var_dvsat_dn8 = assign47620_e80530_d_n8;
        locals.var_dvsat_dn9 = assign47620_e80530_d_n9;
        locals.var_dvsat_dn10 = assign47620_e80530_d_n10;
        locals.var_dvsat_dn11 = assign47620_e80530_d_n11;
        locals.var_dvsat_rv = 0.0;

        let (assign47630_e80544, assign47630_e80544_d_n3, assign47630_e80544_d_n4, assign47630_e80544_d_n5, assign47630_e80544_d_n6, assign47630_e80544_d_n7, assign47630_e80544_d_n8, assign47630_e80544_d_n9, assign47630_e80544_d_n10, assign47630_e80544_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard738 == 0.0)) {
        let assign47630_e80540: f64 = (1.0 / locals.var_t2);
        let assign47630_e80541: f64 = (locals.var_t2 + assign47630_e80540);
        let assign47630_e80542: f64 = (0.5 * assign47630_e80541);
        (assign47630_e80542, (0.5 * (locals.var_t2_dn3 + (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn4 + (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn5 + (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn6 + (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn7 + (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn8 + (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn9 + (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn10 + (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))))), (0.5 * (locals.var_t2_dn11 + (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))))),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign47630_e80544;
        locals.var_dvsat_dn3 = assign47630_e80544_d_n3;
        locals.var_dvsat_dn4 = assign47630_e80544_d_n4;
        locals.var_dvsat_dn5 = assign47630_e80544_d_n5;
        locals.var_dvsat_dn6 = assign47630_e80544_d_n6;
        locals.var_dvsat_dn7 = assign47630_e80544_d_n7;
        locals.var_dvsat_dn8 = assign47630_e80544_d_n8;
        locals.var_dvsat_dn9 = assign47630_e80544_d_n9;
        locals.var_dvsat_dn10 = assign47630_e80544_d_n10;
        locals.var_dvsat_dn11 = assign47630_e80544_d_n11;
        locals.var_dvsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_166(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47640_e80549, assign47640_e80549_d_n3, assign47640_e80549_d_n4, assign47640_e80549_d_n5, assign47640_e80549_d_n6, assign47640_e80549_d_n7, assign47640_e80549_d_n8, assign47640_e80549_d_n9, assign47640_e80549_d_n10, assign47640_e80549_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    } else {
        (locals.var_dptwg, locals.var_dptwg_dn3, locals.var_dptwg_dn4, locals.var_dptwg_dn5, locals.var_dptwg_dn6, locals.var_dptwg_dn7, locals.var_dptwg_dn8, locals.var_dptwg_dn9, locals.var_dptwg_dn10, locals.var_dptwg_dn11,)
    }
};
        locals.var_dptwg = assign47640_e80549;
        locals.var_dptwg_dn3 = assign47640_e80549_d_n3;
        locals.var_dptwg_dn4 = assign47640_e80549_d_n4;
        locals.var_dptwg_dn5 = assign47640_e80549_d_n5;
        locals.var_dptwg_dn6 = assign47640_e80549_d_n6;
        locals.var_dptwg_dn7 = assign47640_e80549_d_n7;
        locals.var_dptwg_dn8 = assign47640_e80549_d_n8;
        locals.var_dptwg_dn9 = assign47640_e80549_d_n9;
        locals.var_dptwg_dn10 = assign47640_e80549_d_n10;
        locals.var_dptwg_dn11 = assign47640_e80549_d_n11;
        locals.var_dptwg_rv = 0.0;

        let (assign47650_e80554, assign47650_e80554_d_n3, assign47650_e80554_d_n4, assign47650_e80554_d_n5, assign47650_e80554_d_n6, assign47650_e80554_d_n7, assign47650_e80554_d_n8, assign47650_e80554_d_n9, assign47650_e80554_d_n10, assign47650_e80554_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47650_e80554;
        locals.var_rsource_dn3 = assign47650_e80554_d_n3;
        locals.var_rsource_dn4 = assign47650_e80554_d_n4;
        locals.var_rsource_dn5 = assign47650_e80554_d_n5;
        locals.var_rsource_dn6 = assign47650_e80554_d_n6;
        locals.var_rsource_dn7 = assign47650_e80554_d_n7;
        locals.var_rsource_dn8 = assign47650_e80554_d_n8;
        locals.var_rsource_dn9 = assign47650_e80554_d_n9;
        locals.var_rsource_dn10 = assign47650_e80554_d_n10;
        locals.var_rsource_dn11 = assign47650_e80554_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47660_e80559, assign47660_e80559_d_n3, assign47660_e80559_d_n4, assign47660_e80559_d_n5, assign47660_e80559_d_n6, assign47660_e80559_d_n7, assign47660_e80559_d_n8, assign47660_e80559_d_n9, assign47660_e80559_d_n10, assign47660_e80559_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47660_e80559;
        locals.var_rdrain_dn3 = assign47660_e80559_d_n3;
        locals.var_rdrain_dn4 = assign47660_e80559_d_n4;
        locals.var_rdrain_dn5 = assign47660_e80559_d_n5;
        locals.var_rdrain_dn6 = assign47660_e80559_d_n6;
        locals.var_rdrain_dn7 = assign47660_e80559_d_n7;
        locals.var_rdrain_dn8 = assign47660_e80559_d_n8;
        locals.var_rdrain_dn9 = assign47660_e80559_d_n9;
        locals.var_rdrain_dn10 = assign47660_e80559_d_n10;
        locals.var_rdrain_dn11 = assign47660_e80559_d_n11;
        locals.var_rdrain_rv = 0.0;

        let assign47670_e80562: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard739 = assign47670_e80562;
        locals.var_guard739_rv = 0.0;

        let (assign47680_e80569, assign47680_e80569_d_n3, assign47680_e80569_d_n4, assign47680_e80569_d_n5, assign47680_e80569_d_n6, assign47680_e80569_d_n7, assign47680_e80569_d_n8, assign47680_e80569_d_n9, assign47680_e80569_d_n10, assign47680_e80569_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47680_e80569;
        locals.var_rdsi_dn3 = assign47680_e80569_d_n3;
        locals.var_rdsi_dn4 = assign47680_e80569_d_n4;
        locals.var_rdsi_dn5 = assign47680_e80569_d_n5;
        locals.var_rdsi_dn6 = assign47680_e80569_d_n6;
        locals.var_rdsi_dn7 = assign47680_e80569_d_n7;
        locals.var_rdsi_dn8 = assign47680_e80569_d_n8;
        locals.var_rdsi_dn9 = assign47680_e80569_d_n9;
        locals.var_rdsi_dn10 = assign47680_e80569_d_n10;
        locals.var_rdsi_dn11 = assign47680_e80569_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign47690_e80576, assign47690_e80576_d_n3, assign47690_e80576_d_n4, assign47690_e80576_d_n5, assign47690_e80576_d_n6, assign47690_e80576_d_n7, assign47690_e80576_d_n8, assign47690_e80576_d_n9, assign47690_e80576_d_n10, assign47690_e80576_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47690_e80576;
        locals.var_dr_dn3 = assign47690_e80576_d_n3;
        locals.var_dr_dn4 = assign47690_e80576_d_n4;
        locals.var_dr_dn5 = assign47690_e80576_d_n5;
        locals.var_dr_dn6 = assign47690_e80576_d_n6;
        locals.var_dr_dn7 = assign47690_e80576_d_n7;
        locals.var_dr_dn8 = assign47690_e80576_d_n8;
        locals.var_dr_dn9 = assign47690_e80576_d_n9;
        locals.var_dr_dn10 = assign47690_e80576_d_n10;
        locals.var_dr_dn11 = assign47690_e80576_d_n11;
        locals.var_dr_rv = 0.0;

        let (assign47700_e80585, assign47700_e80585_d_n3, assign47700_e80585_d_n4, assign47700_e80585_d_n5, assign47700_e80585_d_n6, assign47700_e80585_d_n7, assign47700_e80585_d_n8, assign47700_e80585_d_n9, assign47700_e80585_d_n10, assign47700_e80585_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47700_e80583: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign47700_e80583, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47700_e80585;
        locals.var_t2_dn3 = assign47700_e80585_d_n3;
        locals.var_t2_dn4 = assign47700_e80585_d_n4;
        locals.var_t2_dn5 = assign47700_e80585_d_n5;
        locals.var_t2_dn6 = assign47700_e80585_d_n6;
        locals.var_t2_dn7 = assign47700_e80585_d_n7;
        locals.var_t2_dn8 = assign47700_e80585_d_n8;
        locals.var_t2_dn9 = assign47700_e80585_d_n9;
        locals.var_t2_dn10 = assign47700_e80585_d_n10;
        locals.var_t2_dn11 = assign47700_e80585_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47710_e80597, assign47710_e80597_d_n3, assign47710_e80597_d_n4, assign47710_e80597_d_n5, assign47710_e80597_d_n6, assign47710_e80597_d_n7, assign47710_e80597_d_n8, assign47710_e80597_d_n9, assign47710_e80597_d_n10, assign47710_e80597_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47710_e80592: f64 = (locals.var_t2 * locals.var_t2);
        let assign47710_e80594: f64 = (assign47710_e80592 + 0.01);
        let assign47710_e80595: f64 = (assign47710_e80594).sqrt();
        (assign47710_e80595, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47710_e80595)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47710_e80597;
        locals.var_t3_dn3 = assign47710_e80597_d_n3;
        locals.var_t3_dn4 = assign47710_e80597_d_n4;
        locals.var_t3_dn5 = assign47710_e80597_d_n5;
        locals.var_t3_dn6 = assign47710_e80597_d_n6;
        locals.var_t3_dn7 = assign47710_e80597_d_n7;
        locals.var_t3_dn8 = assign47710_e80597_d_n8;
        locals.var_t3_dn9 = assign47710_e80597_d_n9;
        locals.var_t3_dn10 = assign47710_e80597_d_n10;
        locals.var_t3_dn11 = assign47710_e80597_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign47720_e80608, assign47720_e80608_d_n3, assign47720_e80608_d_n4, assign47720_e80608_d_n5, assign47720_e80608_d_n6, assign47720_e80608_d_n7, assign47720_e80608_d_n8, assign47720_e80608_d_n9, assign47720_e80608_d_n10, assign47720_e80608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47720_e80605: f64 = (locals.var_t2 + locals.var_t3);
        let assign47720_e80606: f64 = (0.5 * assign47720_e80605);
        (assign47720_e80606, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign47720_e80608;
        locals.var_vgs_eff_dn3 = assign47720_e80608_d_n3;
        locals.var_vgs_eff_dn4 = assign47720_e80608_d_n4;
        locals.var_vgs_eff_dn5 = assign47720_e80608_d_n5;
        locals.var_vgs_eff_dn6 = assign47720_e80608_d_n6;
        locals.var_vgs_eff_dn7 = assign47720_e80608_d_n7;
        locals.var_vgs_eff_dn8 = assign47720_e80608_d_n8;
        locals.var_vgs_eff_dn9 = assign47720_e80608_d_n9;
        locals.var_vgs_eff_dn10 = assign47720_e80608_d_n10;
        locals.var_vgs_eff_dn11 = assign47720_e80608_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let (assign47730_e80619, assign47730_e80619_d_n3, assign47730_e80619_d_n4, assign47730_e80619_d_n5, assign47730_e80619_d_n6, assign47730_e80619_d_n7, assign47730_e80619_d_n8, assign47730_e80619_d_n9, assign47730_e80619_d_n10, assign47730_e80619_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47730_e80616: f64 = (locals.var_prwg_i * locals.var_vgs_eff);
        let assign47730_e80617: f64 = (1.0 + assign47730_e80616);
        (assign47730_e80617, (locals.var_prwg_i * locals.var_vgs_eff_dn3), (locals.var_prwg_i * locals.var_vgs_eff_dn4), (locals.var_prwg_i * locals.var_vgs_eff_dn5), (locals.var_prwg_i * locals.var_vgs_eff_dn6), (locals.var_prwg_i * locals.var_vgs_eff_dn7), (locals.var_prwg_i * locals.var_vgs_eff_dn8), (locals.var_prwg_i * locals.var_vgs_eff_dn9), (locals.var_prwg_i * locals.var_vgs_eff_dn10), (locals.var_prwg_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47730_e80619;
        locals.var_t5_dn3 = assign47730_e80619_d_n3;
        locals.var_t5_dn4 = assign47730_e80619_d_n4;
        locals.var_t5_dn5 = assign47730_e80619_d_n5;
        locals.var_t5_dn6 = assign47730_e80619_d_n6;
        locals.var_t5_dn7 = assign47730_e80619_d_n7;
        locals.var_t5_dn8 = assign47730_e80619_d_n8;
        locals.var_t5_dn9 = assign47730_e80619_d_n9;
        locals.var_t5_dn10 = assign47730_e80619_d_n10;
        locals.var_t5_dn11 = assign47730_e80619_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign47740_e80632, assign47740_e80632_d_n3, assign47740_e80632_d_n4, assign47740_e80632_d_n5, assign47740_e80632_d_n6, assign47740_e80632_d_n7, assign47740_e80632_d_n8, assign47740_e80632_d_n9, assign47740_e80632_d_n10, assign47740_e80632_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47740_e80626: f64 = (1.0 / locals.var_t5);
        let assign47740_e80629: f64 = (locals.var_prwb_i * locals.var_vsb_noswap);
        let assign47740_e80630: f64 = (assign47740_e80626 + assign47740_e80629);
        (assign47740_e80630, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign47740_e80632;
        locals.var_t6_dn3 = assign47740_e80632_d_n3;
        locals.var_t6_dn4 = assign47740_e80632_d_n4;
        locals.var_t6_dn5 = assign47740_e80632_d_n5;
        locals.var_t6_dn6 = assign47740_e80632_d_n6;
        locals.var_t6_dn7 = assign47740_e80632_d_n7;
        locals.var_t6_dn8 = assign47740_e80632_d_n8;
        locals.var_t6_dn9 = assign47740_e80632_d_n9;
        locals.var_t6_dn10 = assign47740_e80632_d_n10;
        locals.var_t6_dn11 = assign47740_e80632_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign47750_e80648, assign47750_e80648_d_n3, assign47750_e80648_d_n4, assign47750_e80648_d_n5, assign47750_e80648_d_n6, assign47750_e80648_d_n7, assign47750_e80648_d_n8, assign47750_e80648_d_n9, assign47750_e80648_d_n10, assign47750_e80648_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47750_e80641: f64 = (locals.var_t6 * locals.var_t6);
        let assign47750_e80643: f64 = (assign47750_e80641 + 0.01);
        let assign47750_e80644: f64 = (assign47750_e80643).sqrt();
        let assign47750_e80645: f64 = (locals.var_t6 + assign47750_e80644);
        let assign47750_e80646: f64 = (0.5 * assign47750_e80645);
        (assign47750_e80646, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign47750_e80644)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47750_e80648;
        locals.var_t4_dn3 = assign47750_e80648_d_n3;
        locals.var_t4_dn4 = assign47750_e80648_d_n4;
        locals.var_t4_dn5 = assign47750_e80648_d_n5;
        locals.var_t4_dn6 = assign47750_e80648_d_n6;
        locals.var_t4_dn7 = assign47750_e80648_d_n7;
        locals.var_t4_dn8 = assign47750_e80648_d_n8;
        locals.var_t4_dn9 = assign47750_e80648_d_n9;
        locals.var_t4_dn10 = assign47750_e80648_d_n10;
        locals.var_t4_dn11 = assign47750_e80648_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47760_e80665, assign47760_e80665_d_n3, assign47760_e80665_d_n4, assign47760_e80665_d_n5, assign47760_e80665_d_n6, assign47760_e80665_d_n7, assign47760_e80665_d_n8, assign47760_e80665_d_n9, assign47760_e80665_d_n10, assign47760_e80665_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47760_e80658: f64 = (locals.var_rsw_i * locals.var_t4);
        let assign47760_e80659: f64 = (locals.var_rswmin_i + assign47760_e80658);
        let assign47760_e80661: f64 = (assign47760_e80659 * locals.var_weffwrfactor);
        let assign47760_e80662: f64 = (locals.var_rsourcegeo + assign47760_e80661);
        let assign47760_e80663: f64 = (locals.var_rdstemp * assign47760_e80662);
        (assign47760_e80663, (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign47760_e80662) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign47760_e80662) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47760_e80665;
        locals.var_rsource_dn3 = assign47760_e80665_d_n3;
        locals.var_rsource_dn4 = assign47760_e80665_d_n4;
        locals.var_rsource_dn5 = assign47760_e80665_d_n5;
        locals.var_rsource_dn6 = assign47760_e80665_d_n6;
        locals.var_rsource_dn7 = assign47760_e80665_d_n7;
        locals.var_rsource_dn8 = assign47760_e80665_d_n8;
        locals.var_rsource_dn9 = assign47760_e80665_d_n9;
        locals.var_rsource_dn10 = assign47760_e80665_d_n10;
        locals.var_rsource_dn11 = assign47760_e80665_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47770_e80674, assign47770_e80674_d_n3, assign47770_e80674_d_n4, assign47770_e80674_d_n5, assign47770_e80674_d_n6, assign47770_e80674_d_n7, assign47770_e80674_d_n8, assign47770_e80674_d_n9, assign47770_e80674_d_n10, assign47770_e80674_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47770_e80672: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign47770_e80672, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47770_e80674;
        locals.var_t2_dn3 = assign47770_e80674_d_n3;
        locals.var_t2_dn4 = assign47770_e80674_d_n4;
        locals.var_t2_dn5 = assign47770_e80674_d_n5;
        locals.var_t2_dn6 = assign47770_e80674_d_n6;
        locals.var_t2_dn7 = assign47770_e80674_d_n7;
        locals.var_t2_dn8 = assign47770_e80674_d_n8;
        locals.var_t2_dn9 = assign47770_e80674_d_n9;
        locals.var_t2_dn10 = assign47770_e80674_d_n10;
        locals.var_t2_dn11 = assign47770_e80674_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47780_e80686, assign47780_e80686_d_n3, assign47780_e80686_d_n4, assign47780_e80686_d_n5, assign47780_e80686_d_n6, assign47780_e80686_d_n7, assign47780_e80686_d_n8, assign47780_e80686_d_n9, assign47780_e80686_d_n10, assign47780_e80686_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47780_e80681: f64 = (locals.var_t2 * locals.var_t2);
        let assign47780_e80683: f64 = (assign47780_e80681 + 0.01);
        let assign47780_e80684: f64 = (assign47780_e80683).sqrt();
        (assign47780_e80684, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47780_e80684)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47780_e80686;
        locals.var_t3_dn3 = assign47780_e80686_d_n3;
        locals.var_t3_dn4 = assign47780_e80686_d_n4;
        locals.var_t3_dn5 = assign47780_e80686_d_n5;
        locals.var_t3_dn6 = assign47780_e80686_d_n6;
        locals.var_t3_dn7 = assign47780_e80686_d_n7;
        locals.var_t3_dn8 = assign47780_e80686_d_n8;
        locals.var_t3_dn9 = assign47780_e80686_d_n9;
        locals.var_t3_dn10 = assign47780_e80686_d_n10;
        locals.var_t3_dn11 = assign47780_e80686_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign47790_e80697, assign47790_e80697_d_n3, assign47790_e80697_d_n4, assign47790_e80697_d_n5, assign47790_e80697_d_n6, assign47790_e80697_d_n7, assign47790_e80697_d_n8, assign47790_e80697_d_n9, assign47790_e80697_d_n10, assign47790_e80697_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47790_e80694: f64 = (locals.var_t2 + locals.var_t3);
        let assign47790_e80695: f64 = (0.5 * assign47790_e80694);
        (assign47790_e80695, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign47790_e80697;
        locals.var_vgd_eff_dn3 = assign47790_e80697_d_n3;
        locals.var_vgd_eff_dn4 = assign47790_e80697_d_n4;
        locals.var_vgd_eff_dn5 = assign47790_e80697_d_n5;
        locals.var_vgd_eff_dn6 = assign47790_e80697_d_n6;
        locals.var_vgd_eff_dn7 = assign47790_e80697_d_n7;
        locals.var_vgd_eff_dn8 = assign47790_e80697_d_n8;
        locals.var_vgd_eff_dn9 = assign47790_e80697_d_n9;
        locals.var_vgd_eff_dn10 = assign47790_e80697_d_n10;
        locals.var_vgd_eff_dn11 = assign47790_e80697_d_n11;
        locals.var_vgd_eff_rv = 0.0;

        let (assign47800_e80708, assign47800_e80708_d_n3, assign47800_e80708_d_n4, assign47800_e80708_d_n5, assign47800_e80708_d_n6, assign47800_e80708_d_n7, assign47800_e80708_d_n8, assign47800_e80708_d_n9, assign47800_e80708_d_n10, assign47800_e80708_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47800_e80705: f64 = (locals.var_prwg_i * locals.var_vgd_eff);
        let assign47800_e80706: f64 = (1.0 + assign47800_e80705);
        (assign47800_e80706, (locals.var_prwg_i * locals.var_vgd_eff_dn3), (locals.var_prwg_i * locals.var_vgd_eff_dn4), (locals.var_prwg_i * locals.var_vgd_eff_dn5), (locals.var_prwg_i * locals.var_vgd_eff_dn6), (locals.var_prwg_i * locals.var_vgd_eff_dn7), (locals.var_prwg_i * locals.var_vgd_eff_dn8), (locals.var_prwg_i * locals.var_vgd_eff_dn9), (locals.var_prwg_i * locals.var_vgd_eff_dn10), (locals.var_prwg_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47800_e80708;
        locals.var_t5_dn3 = assign47800_e80708_d_n3;
        locals.var_t5_dn4 = assign47800_e80708_d_n4;
        locals.var_t5_dn5 = assign47800_e80708_d_n5;
        locals.var_t5_dn6 = assign47800_e80708_d_n6;
        locals.var_t5_dn7 = assign47800_e80708_d_n7;
        locals.var_t5_dn8 = assign47800_e80708_d_n8;
        locals.var_t5_dn9 = assign47800_e80708_d_n9;
        locals.var_t5_dn10 = assign47800_e80708_d_n10;
        locals.var_t5_dn11 = assign47800_e80708_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign47810_e80721, assign47810_e80721_d_n3, assign47810_e80721_d_n4, assign47810_e80721_d_n5, assign47810_e80721_d_n6, assign47810_e80721_d_n7, assign47810_e80721_d_n8, assign47810_e80721_d_n9, assign47810_e80721_d_n10, assign47810_e80721_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47810_e80715: f64 = (1.0 / locals.var_t5);
        let assign47810_e80718: f64 = (locals.var_prwb_i * locals.var_vdb_noswap);
        let assign47810_e80719: f64 = (assign47810_e80715 + assign47810_e80718);
        (assign47810_e80719, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign47810_e80721;
        locals.var_t6_dn3 = assign47810_e80721_d_n3;
        locals.var_t6_dn4 = assign47810_e80721_d_n4;
        locals.var_t6_dn5 = assign47810_e80721_d_n5;
        locals.var_t6_dn6 = assign47810_e80721_d_n6;
        locals.var_t6_dn7 = assign47810_e80721_d_n7;
        locals.var_t6_dn8 = assign47810_e80721_d_n8;
        locals.var_t6_dn9 = assign47810_e80721_d_n9;
        locals.var_t6_dn10 = assign47810_e80721_d_n10;
        locals.var_t6_dn11 = assign47810_e80721_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign47820_e80737, assign47820_e80737_d_n3, assign47820_e80737_d_n4, assign47820_e80737_d_n5, assign47820_e80737_d_n6, assign47820_e80737_d_n7, assign47820_e80737_d_n8, assign47820_e80737_d_n9, assign47820_e80737_d_n10, assign47820_e80737_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47820_e80730: f64 = (locals.var_t6 * locals.var_t6);
        let assign47820_e80732: f64 = (assign47820_e80730 + 0.01);
        let assign47820_e80733: f64 = (assign47820_e80732).sqrt();
        let assign47820_e80734: f64 = (locals.var_t6 + assign47820_e80733);
        let assign47820_e80735: f64 = (0.5 * assign47820_e80734);
        (assign47820_e80735, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign47820_e80733)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47820_e80737;
        locals.var_t4_dn3 = assign47820_e80737_d_n3;
        locals.var_t4_dn4 = assign47820_e80737_d_n4;
        locals.var_t4_dn5 = assign47820_e80737_d_n5;
        locals.var_t4_dn6 = assign47820_e80737_d_n6;
        locals.var_t4_dn7 = assign47820_e80737_d_n7;
        locals.var_t4_dn8 = assign47820_e80737_d_n8;
        locals.var_t4_dn9 = assign47820_e80737_d_n9;
        locals.var_t4_dn10 = assign47820_e80737_d_n10;
        locals.var_t4_dn11 = assign47820_e80737_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47830_e80754, assign47830_e80754_d_n3, assign47830_e80754_d_n4, assign47830_e80754_d_n5, assign47830_e80754_d_n6, assign47830_e80754_d_n7, assign47830_e80754_d_n8, assign47830_e80754_d_n9, assign47830_e80754_d_n10, assign47830_e80754_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47830_e80747: f64 = (locals.var_rdw_i * locals.var_t4);
        let assign47830_e80748: f64 = (locals.var_rdwmin_i + assign47830_e80747);
        let assign47830_e80750: f64 = (assign47830_e80748 * locals.var_weffwrfactor);
        let assign47830_e80751: f64 = (locals.var_rdraingeo + assign47830_e80750);
        let assign47830_e80752: f64 = (locals.var_rdstemp * assign47830_e80751);
        (assign47830_e80752, (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign47830_e80751) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign47830_e80751) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47830_e80754;
        locals.var_rdrain_dn3 = assign47830_e80754_d_n3;
        locals.var_rdrain_dn4 = assign47830_e80754_d_n4;
        locals.var_rdrain_dn5 = assign47830_e80754_d_n5;
        locals.var_rdrain_dn6 = assign47830_e80754_d_n6;
        locals.var_rdrain_dn7 = assign47830_e80754_d_n7;
        locals.var_rdrain_dn8 = assign47830_e80754_d_n8;
        locals.var_rdrain_dn9 = assign47830_e80754_d_n9;
        locals.var_rdrain_dn10 = assign47830_e80754_d_n10;
        locals.var_rdrain_dn11 = assign47830_e80754_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign47840_e80766, assign47840_e80766_d_n3, assign47840_e80766_d_n4, assign47840_e80766_d_n5, assign47840_e80766_d_n6, assign47840_e80766_d_n7, assign47840_e80766_d_n8, assign47840_e80766_d_n9, assign47840_e80766_d_n10, assign47840_e80766_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47840_e80763: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign47840_e80764: f64 = (1.0 + assign47840_e80763);
        (assign47840_e80764, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8), (locals.var_prwg_i * locals.var_qia_dn9), (locals.var_prwg_i * locals.var_qia_dn10), (locals.var_prwg_i * locals.var_qia_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47840_e80766;
        locals.var_t0_dn3 = assign47840_e80766_d_n3;
        locals.var_t0_dn4 = assign47840_e80766_d_n4;
        locals.var_t0_dn5 = assign47840_e80766_d_n5;
        locals.var_t0_dn6 = assign47840_e80766_d_n6;
        locals.var_t0_dn7 = assign47840_e80766_d_n7;
        locals.var_t0_dn8 = assign47840_e80766_d_n8;
        locals.var_t0_dn9 = assign47840_e80766_d_n9;
        locals.var_t0_dn10 = assign47840_e80766_d_n10;
        locals.var_t0_dn11 = assign47840_e80766_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47850_e80778, assign47850_e80778_d_n3, assign47850_e80778_d_n4, assign47850_e80778_d_n5, assign47850_e80778_d_n6, assign47850_e80778_d_n7, assign47850_e80778_d_n8, assign47850_e80778_d_n9, assign47850_e80778_d_n10, assign47850_e80778_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47850_e80775: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign47850_e80776: f64 = (locals.var_prwb_i * assign47850_e80775);
        (assign47850_e80776, (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47850_e80778;
        locals.var_t1_dn3 = assign47850_e80778_d_n3;
        locals.var_t1_dn4 = assign47850_e80778_d_n4;
        locals.var_t1_dn5 = assign47850_e80778_d_n5;
        locals.var_t1_dn6 = assign47850_e80778_d_n6;
        locals.var_t1_dn7 = assign47850_e80778_d_n7;
        locals.var_t1_dn8 = assign47850_e80778_d_n8;
        locals.var_t1_dn9 = assign47850_e80778_d_n9;
        locals.var_t1_dn10 = assign47850_e80778_d_n10;
        locals.var_t1_dn11 = assign47850_e80778_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47860_e80790, assign47860_e80790_d_n3, assign47860_e80790_d_n4, assign47860_e80790_d_n5, assign47860_e80790_d_n6, assign47860_e80790_d_n7, assign47860_e80790_d_n8, assign47860_e80790_d_n9, assign47860_e80790_d_n10, assign47860_e80790_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47860_e80786: f64 = (1.0 / locals.var_t0);
        let assign47860_e80788: f64 = (assign47860_e80786 + locals.var_t1);
        (assign47860_e80788, ((-(locals.var_t0_dn3 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn3), ((-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn4), ((-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn5), ((-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn6), ((-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn7), ((-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn8), ((-(locals.var_t0_dn9 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn9), ((-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn10), ((-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47860_e80790;
        locals.var_t2_dn3 = assign47860_e80790_d_n3;
        locals.var_t2_dn4 = assign47860_e80790_d_n4;
        locals.var_t2_dn5 = assign47860_e80790_d_n5;
        locals.var_t2_dn6 = assign47860_e80790_d_n6;
        locals.var_t2_dn7 = assign47860_e80790_d_n7;
        locals.var_t2_dn8 = assign47860_e80790_d_n8;
        locals.var_t2_dn9 = assign47860_e80790_d_n9;
        locals.var_t2_dn10 = assign47860_e80790_d_n10;
        locals.var_t2_dn11 = assign47860_e80790_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47870_e80807, assign47870_e80807_d_n3, assign47870_e80807_d_n4, assign47870_e80807_d_n5, assign47870_e80807_d_n6, assign47870_e80807_d_n7, assign47870_e80807_d_n8, assign47870_e80807_d_n9, assign47870_e80807_d_n10, assign47870_e80807_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47870_e80800: f64 = (locals.var_t2 * locals.var_t2);
        let assign47870_e80802: f64 = (assign47870_e80800 + 0.01);
        let assign47870_e80803: f64 = (assign47870_e80802).sqrt();
        let assign47870_e80804: f64 = (locals.var_t2 + assign47870_e80803);
        let assign47870_e80805: f64 = (0.5 * assign47870_e80804);
        (assign47870_e80805, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47870_e80803)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47870_e80807;
        locals.var_t3_dn3 = assign47870_e80807_d_n3;
        locals.var_t3_dn4 = assign47870_e80807_d_n4;
        locals.var_t3_dn5 = assign47870_e80807_d_n5;
        locals.var_t3_dn6 = assign47870_e80807_d_n6;
        locals.var_t3_dn7 = assign47870_e80807_d_n7;
        locals.var_t3_dn8 = assign47870_e80807_d_n8;
        locals.var_t3_dn9 = assign47870_e80807_d_n9;
        locals.var_t3_dn10 = assign47870_e80807_d_n10;
        locals.var_t3_dn11 = assign47870_e80807_d_n11;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_167(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47880_e80825, assign47880_e80825_d_n3, assign47880_e80825_d_n4, assign47880_e80825_d_n5, assign47880_e80825_d_n6, assign47880_e80825_d_n7, assign47880_e80825_d_n8, assign47880_e80825_d_n9, assign47880_e80825_d_n10, assign47880_e80825_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47880_e80817: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign47880_e80818: f64 = (locals.var_rdswmin_i + assign47880_e80817);
        let assign47880_e80819: f64 = (locals.var_rdstemp * assign47880_e80818);
        let assign47880_e80821: f64 = (assign47880_e80819 * locals.var_weffwrfactor);
        let assign47880_e80823: f64 = (assign47880_e80821 * p.p2);
        (assign47880_e80823, (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn3)) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn4 * assign47880_e80818) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn4))) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn5 * assign47880_e80818) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn5))) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn6)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn7)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn8)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn9)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn10)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn11)) * locals.var_weffwrfactor) * p.p2),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47880_e80825;
        locals.var_rdsi_dn3 = assign47880_e80825_d_n3;
        locals.var_rdsi_dn4 = assign47880_e80825_d_n4;
        locals.var_rdsi_dn5 = assign47880_e80825_d_n5;
        locals.var_rdsi_dn6 = assign47880_e80825_d_n6;
        locals.var_rdsi_dn7 = assign47880_e80825_d_n7;
        locals.var_rdsi_dn8 = assign47880_e80825_d_n8;
        locals.var_rdsi_dn9 = assign47880_e80825_d_n9;
        locals.var_rdsi_dn10 = assign47880_e80825_d_n10;
        locals.var_rdsi_dn11 = assign47880_e80825_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign47890_e80833, assign47890_e80833_d_n3, assign47890_e80833_d_n4, assign47890_e80833_d_n5, assign47890_e80833_d_n6, assign47890_e80833_d_n7, assign47890_e80833_d_n8, assign47890_e80833_d_n9, assign47890_e80833_d_n10, assign47890_e80833_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        (locals.var_rdraingeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47890_e80833;
        locals.var_rdrain_dn3 = assign47890_e80833_d_n3;
        locals.var_rdrain_dn4 = assign47890_e80833_d_n4;
        locals.var_rdrain_dn5 = assign47890_e80833_d_n5;
        locals.var_rdrain_dn6 = assign47890_e80833_d_n6;
        locals.var_rdrain_dn7 = assign47890_e80833_d_n7;
        locals.var_rdrain_dn8 = assign47890_e80833_d_n8;
        locals.var_rdrain_dn9 = assign47890_e80833_d_n9;
        locals.var_rdrain_dn10 = assign47890_e80833_d_n10;
        locals.var_rdrain_dn11 = assign47890_e80833_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign47900_e80841, assign47900_e80841_d_n3, assign47900_e80841_d_n4, assign47900_e80841_d_n5, assign47900_e80841_d_n6, assign47900_e80841_d_n7, assign47900_e80841_d_n8, assign47900_e80841_d_n9, assign47900_e80841_d_n10, assign47900_e80841_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        (locals.var_rsourcegeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47900_e80841;
        locals.var_rsource_dn3 = assign47900_e80841_d_n3;
        locals.var_rsource_dn4 = assign47900_e80841_d_n4;
        locals.var_rsource_dn5 = assign47900_e80841_d_n5;
        locals.var_rsource_dn6 = assign47900_e80841_d_n6;
        locals.var_rsource_dn7 = assign47900_e80841_d_n7;
        locals.var_rsource_dn8 = assign47900_e80841_d_n8;
        locals.var_rsource_dn9 = assign47900_e80841_d_n9;
        locals.var_rsource_dn10 = assign47900_e80841_d_n10;
        locals.var_rsource_dn11 = assign47900_e80841_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47910_e80865, assign47910_e80865_d_n3, assign47910_e80865_d_n4, assign47910_e80865_d_n5, assign47910_e80865_d_n6, assign47910_e80865_d_n7, assign47910_e80865_d_n8, assign47910_e80865_d_n9, assign47910_e80865_d_n10, assign47910_e80865_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47910_e80851: f64 = (locals.var_dvsat * locals.var_dmob);
        let assign47910_e80852: f64 = (locals.var_u0_a / assign47910_e80851);
        let assign47910_e80854: f64 = (assign47910_e80852 * locals.var_cox);
        let assign47910_e80856: f64 = (assign47910_e80854 * locals.var_weff);
        let assign47910_e80858: f64 = (assign47910_e80856 / locals.var_leff);
        let assign47910_e80860: f64 = (assign47910_e80858 * locals.var_qia);
        let assign47910_e80862: f64 = (assign47910_e80860 * locals.var_rdsi);
        let assign47910_e80863: f64 = (1.0 + assign47910_e80862);
        (assign47910_e80863, ((((((((((locals.var_u0_a_dn3 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn3 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn3)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn4 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn4)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn5 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn5)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn6 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn6)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn7 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn7)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn8 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn8)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn9 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn9)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn10 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn10)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn11 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn11)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47910_e80865;
        locals.var_dr_dn3 = assign47910_e80865_d_n3;
        locals.var_dr_dn4 = assign47910_e80865_d_n4;
        locals.var_dr_dn5 = assign47910_e80865_d_n5;
        locals.var_dr_dn6 = assign47910_e80865_d_n6;
        locals.var_dr_dn7 = assign47910_e80865_d_n7;
        locals.var_dr_dn8 = assign47910_e80865_d_n8;
        locals.var_dr_dn9 = assign47910_e80865_d_n9;
        locals.var_dr_dn10 = assign47910_e80865_d_n10;
        locals.var_dr_dn11 = assign47910_e80865_d_n11;
        locals.var_dr_rv = 0.0;

        let assign47920_e80868: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign47920_e80868;
        locals.var_guard740_rv = 0.0;

        let (assign47930_e80892, assign47930_e80892_d_n3, assign47930_e80892_d_n4, assign47930_e80892_d_n5, assign47930_e80892_d_n6, assign47930_e80892_d_n7, assign47930_e80892_d_n8, assign47930_e80892_d_n9, assign47930_e80892_d_n10, assign47930_e80892_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign47930_e80881: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign47930_e80882: f64 = (locals.var_rdswmin_i + assign47930_e80881);
        let assign47930_e80884: f64 = (assign47930_e80882 * locals.var_weffwrfactor);
        let assign47930_e80886: f64 = (assign47930_e80884 * p.p2);
        let assign47930_e80887: f64 = (locals.var_rsourcegeo + assign47930_e80886);
        let assign47930_e80889: f64 = (assign47930_e80887 + locals.var_rdraingeo);
        let assign47930_e80890: f64 = (locals.var_rdstemp * assign47930_e80889);
        (assign47930_e80890, (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2)), ((locals.var_rdstemp_dn4 * assign47930_e80889) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2))), ((locals.var_rdstemp_dn5 * assign47930_e80889) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2))), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2)),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47930_e80892;
        locals.var_rdsi_dn3 = assign47930_e80892_d_n3;
        locals.var_rdsi_dn4 = assign47930_e80892_d_n4;
        locals.var_rdsi_dn5 = assign47930_e80892_d_n5;
        locals.var_rdsi_dn6 = assign47930_e80892_d_n6;
        locals.var_rdsi_dn7 = assign47930_e80892_d_n7;
        locals.var_rdsi_dn8 = assign47930_e80892_d_n8;
        locals.var_rdsi_dn9 = assign47930_e80892_d_n9;
        locals.var_rdsi_dn10 = assign47930_e80892_d_n10;
        locals.var_rdsi_dn11 = assign47930_e80892_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign47940_e80902, assign47940_e80902_d_n3, assign47940_e80902_d_n4, assign47940_e80902_d_n5, assign47940_e80902_d_n6, assign47940_e80902_d_n7, assign47940_e80902_d_n8, assign47940_e80902_d_n9, assign47940_e80902_d_n10, assign47940_e80902_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47940_e80902;
        locals.var_rdrain_dn3 = assign47940_e80902_d_n3;
        locals.var_rdrain_dn4 = assign47940_e80902_d_n4;
        locals.var_rdrain_dn5 = assign47940_e80902_d_n5;
        locals.var_rdrain_dn6 = assign47940_e80902_d_n6;
        locals.var_rdrain_dn7 = assign47940_e80902_d_n7;
        locals.var_rdrain_dn8 = assign47940_e80902_d_n8;
        locals.var_rdrain_dn9 = assign47940_e80902_d_n9;
        locals.var_rdrain_dn10 = assign47940_e80902_d_n10;
        locals.var_rdrain_dn11 = assign47940_e80902_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign47950_e80912, assign47950_e80912_d_n3, assign47950_e80912_d_n4, assign47950_e80912_d_n5, assign47950_e80912_d_n6, assign47950_e80912_d_n7, assign47950_e80912_d_n8, assign47950_e80912_d_n9, assign47950_e80912_d_n10, assign47950_e80912_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47950_e80912;
        locals.var_rsource_dn3 = assign47950_e80912_d_n3;
        locals.var_rsource_dn4 = assign47950_e80912_d_n4;
        locals.var_rsource_dn5 = assign47950_e80912_d_n5;
        locals.var_rsource_dn6 = assign47950_e80912_d_n6;
        locals.var_rsource_dn7 = assign47950_e80912_d_n7;
        locals.var_rsource_dn8 = assign47950_e80912_d_n8;
        locals.var_rsource_dn9 = assign47950_e80912_d_n9;
        locals.var_rsource_dn10 = assign47950_e80912_d_n10;
        locals.var_rsource_dn11 = assign47950_e80912_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47960_e80938, assign47960_e80938_d_n3, assign47960_e80938_d_n4, assign47960_e80938_d_n5, assign47960_e80938_d_n6, assign47960_e80938_d_n7, assign47960_e80938_d_n8, assign47960_e80938_d_n9, assign47960_e80938_d_n10, assign47960_e80938_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign47960_e80924: f64 = (locals.var_dvsat * locals.var_dmob);
        let assign47960_e80925: f64 = (locals.var_u0_a / assign47960_e80924);
        let assign47960_e80927: f64 = (assign47960_e80925 * locals.var_cox);
        let assign47960_e80929: f64 = (assign47960_e80927 * locals.var_weff);
        let assign47960_e80931: f64 = (assign47960_e80929 / locals.var_leff);
        let assign47960_e80933: f64 = (assign47960_e80931 * locals.var_qia);
        let assign47960_e80935: f64 = (assign47960_e80933 * locals.var_rdsi);
        let assign47960_e80936: f64 = (1.0 + assign47960_e80935);
        (assign47960_e80936, ((((((((((locals.var_u0_a_dn3 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn3 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn3)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn4 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn4)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn5 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn5)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn6 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn6)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn7 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn7)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn8 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn8)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn9 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn9)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn10 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn10)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn11 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn11)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47960_e80938;
        locals.var_dr_dn3 = assign47960_e80938_d_n3;
        locals.var_dr_dn4 = assign47960_e80938_d_n4;
        locals.var_dr_dn5 = assign47960_e80938_d_n5;
        locals.var_dr_dn6 = assign47960_e80938_d_n6;
        locals.var_dr_dn7 = assign47960_e80938_d_n7;
        locals.var_dr_dn8 = assign47960_e80938_d_n8;
        locals.var_dr_dn9 = assign47960_e80938_d_n9;
        locals.var_dr_dn10 = assign47960_e80938_d_n10;
        locals.var_dr_dn11 = assign47960_e80938_d_n11;
        locals.var_dr_rv = 0.0;

        let (assign47970_e80953, assign47970_e80953_d_n3, assign47970_e80953_d_n4, assign47970_e80953_d_n5, assign47970_e80953_d_n6, assign47970_e80953_d_n7, assign47970_e80953_d_n8, assign47970_e80953_d_n9, assign47970_e80953_d_n10, assign47970_e80953_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47970_e80946: f64 = (2.0 * locals.var_n);
        let assign47970_e80948: f64 = (assign47970_e80946 * locals.var_vtm);
        let assign47970_e80949: f64 = (locals.var_qia + assign47970_e80948);
        let assign47970_e80950: f64 = (locals.var_a2_t / assign47970_e80949);
        let assign47970_e80951: f64 = (locals.var_a1_t + assign47970_e80950);
        (assign47970_e80951, (-((locals.var_a2_t * (locals.var_qia_dn3 + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (locals.var_a1_t_dn4 + (((locals.var_a2_t_dn4 * assign47970_e80949) - (locals.var_a2_t * (locals.var_qia_dn4 + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign47970_e80946 * locals.var_vtm_dn4))))) / (assign47970_e80949 * assign47970_e80949))), (locals.var_a1_t_dn5 + (((locals.var_a2_t_dn5 * assign47970_e80949) - (locals.var_a2_t * (locals.var_qia_dn5 + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign47970_e80946 * locals.var_vtm_dn5))))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn6 + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn7 + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn8 + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn9 + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn10 + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn11 + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47970_e80953;
        locals.var_t0_dn3 = assign47970_e80953_d_n3;
        locals.var_t0_dn4 = assign47970_e80953_d_n4;
        locals.var_t0_dn5 = assign47970_e80953_d_n5;
        locals.var_t0_dn6 = assign47970_e80953_d_n6;
        locals.var_t0_dn7 = assign47970_e80953_d_n7;
        locals.var_t0_dn8 = assign47970_e80953_d_n8;
        locals.var_t0_dn9 = assign47970_e80953_d_n9;
        locals.var_t0_dn10 = assign47970_e80953_d_n10;
        locals.var_t0_dn11 = assign47970_e80953_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47980_e80960, assign47980_e80960_d_n3, assign47980_e80960_d_n4, assign47980_e80960_d_n5, assign47980_e80960_d_n6, assign47980_e80960_d_n7, assign47980_e80960_d_n8, assign47980_e80960_d_n9, assign47980_e80960_d_n10, assign47980_e80960_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47980_e80958: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign47980_e80958, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_dqsd, locals.var_dqsd_dn3, locals.var_dqsd_dn4, locals.var_dqsd_dn5, locals.var_dqsd_dn6, locals.var_dqsd_dn7, locals.var_dqsd_dn8, locals.var_dqsd_dn9, locals.var_dqsd_dn10, locals.var_dqsd_dn11,)
    }
};
        locals.var_dqsd = assign47980_e80960;
        locals.var_dqsd_dn3 = assign47980_e80960_d_n3;
        locals.var_dqsd_dn4 = assign47980_e80960_d_n4;
        locals.var_dqsd_dn5 = assign47980_e80960_d_n5;
        locals.var_dqsd_dn6 = assign47980_e80960_d_n6;
        locals.var_dqsd_dn7 = assign47980_e80960_d_n7;
        locals.var_dqsd_dn8 = assign47980_e80960_d_n8;
        locals.var_dqsd_dn9 = assign47980_e80960_d_n9;
        locals.var_dqsd_dn10 = assign47980_e80960_d_n10;
        locals.var_dqsd_dn11 = assign47980_e80960_d_n11;
        locals.var_dqsd_rv = 0.0;

        let (assign47990_e80969, assign47990_e80969_d_n3, assign47990_e80969_d_n4, assign47990_e80969_d_n5, assign47990_e80969_d_n6, assign47990_e80969_d_n7, assign47990_e80969_d_n8, assign47990_e80969_d_n9, assign47990_e80969_d_n10, assign47990_e80969_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47990_e80965: f64 = (locals.var_t0 * locals.var_dqsd);
        let assign47990_e80967: f64 = (assign47990_e80965 * locals.var_dqsd);
        (assign47990_e80967, ((((locals.var_t0_dn3 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn3)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn3)), ((((locals.var_t0_dn4 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn4)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn4)), ((((locals.var_t0_dn5 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn5)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn5)), ((((locals.var_t0_dn6 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn6)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn6)), ((((locals.var_t0_dn7 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn7)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn7)), ((((locals.var_t0_dn8 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn8)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn8)), ((((locals.var_t0_dn9 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn9)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn9)), ((((locals.var_t0_dn10 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn10)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn10)), ((((locals.var_t0_dn11 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn11)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47990_e80969;
        locals.var_t1_dn3 = assign47990_e80969_d_n3;
        locals.var_t1_dn4 = assign47990_e80969_d_n4;
        locals.var_t1_dn5 = assign47990_e80969_d_n5;
        locals.var_t1_dn6 = assign47990_e80969_d_n6;
        locals.var_t1_dn7 = assign47990_e80969_d_n7;
        locals.var_t1_dn8 = assign47990_e80969_d_n8;
        locals.var_t1_dn9 = assign47990_e80969_d_n9;
        locals.var_t1_dn10 = assign47990_e80969_d_n10;
        locals.var_t1_dn11 = assign47990_e80969_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48000_e80978, assign48000_e80978_d_n3, assign48000_e80978_d_n4, assign48000_e80978_d_n5, assign48000_e80978_d_n6, assign48000_e80978_d_n7, assign48000_e80978_d_n8, assign48000_e80978_d_n9, assign48000_e80978_d_n10, assign48000_e80978_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48000_e80974: f64 = (locals.var_t1 + 1.0);
        let assign48000_e80976: f64 = (assign48000_e80974 - 0.001);
        (assign48000_e80976, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48000_e80978;
        locals.var_t2_dn3 = assign48000_e80978_d_n3;
        locals.var_t2_dn4 = assign48000_e80978_d_n4;
        locals.var_t2_dn5 = assign48000_e80978_d_n5;
        locals.var_t2_dn6 = assign48000_e80978_d_n6;
        locals.var_t2_dn7 = assign48000_e80978_d_n7;
        locals.var_t2_dn8 = assign48000_e80978_d_n8;
        locals.var_t2_dn9 = assign48000_e80978_d_n9;
        locals.var_t2_dn10 = assign48000_e80978_d_n10;
        locals.var_t2_dn11 = assign48000_e80978_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign48010_e80995, assign48010_e80995_d_n3, assign48010_e80995_d_n4, assign48010_e80995_d_n5, assign48010_e80995_d_n6, assign48010_e80995_d_n7, assign48010_e80995_d_n8, assign48010_e80995_d_n9, assign48010_e80995_d_n10, assign48010_e80995_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48010_e80982: f64 = (-1.0);
        let assign48010_e80987: f64 = (locals.var_t2 * locals.var_t2);
        let assign48010_e80989: f64 = (assign48010_e80987 + 0.004);
        let assign48010_e80990: f64 = (assign48010_e80989).sqrt();
        let assign48010_e80991: f64 = (locals.var_t2 + assign48010_e80990);
        let assign48010_e80992: f64 = (0.5 * assign48010_e80991);
        let assign48010_e80993: f64 = (assign48010_e80982 + assign48010_e80992);
        (assign48010_e80993, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign48010_e80990)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48010_e80995;
        locals.var_t3_dn3 = assign48010_e80995_d_n3;
        locals.var_t3_dn4 = assign48010_e80995_d_n4;
        locals.var_t3_dn5 = assign48010_e80995_d_n5;
        locals.var_t3_dn6 = assign48010_e80995_d_n6;
        locals.var_t3_dn7 = assign48010_e80995_d_n7;
        locals.var_t3_dn8 = assign48010_e80995_d_n8;
        locals.var_t3_dn9 = assign48010_e80995_d_n9;
        locals.var_t3_dn10 = assign48010_e80995_d_n10;
        locals.var_t3_dn11 = assign48010_e80995_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign48020_e81007, assign48020_e81007_d_n3, assign48020_e81007_d_n4, assign48020_e81007_d_n5, assign48020_e81007_d_n6, assign48020_e81007_d_n7, assign48020_e81007_d_n8, assign48020_e81007_d_n9, assign48020_e81007_d_n10, assign48020_e81007_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48020_e81002: f64 = (1.0 + locals.var_t3);
        let assign48020_e81003: f64 = (assign48020_e81002).sqrt();
        let assign48020_e81004: f64 = (1.0 + assign48020_e81003);
        let assign48020_e81005: f64 = (0.5 * assign48020_e81004);
        (assign48020_e81005, (0.5 * (locals.var_t3_dn3 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn4 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn5 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn6 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn7 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn8 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn9 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn10 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn11 / (2.0 * assign48020_e81003))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign48020_e81007;
        locals.var_nsat_dn3 = assign48020_e81007_d_n3;
        locals.var_nsat_dn4 = assign48020_e81007_d_n4;
        locals.var_nsat_dn5 = assign48020_e81007_d_n5;
        locals.var_nsat_dn6 = assign48020_e81007_d_n6;
        locals.var_nsat_dn7 = assign48020_e81007_d_n7;
        locals.var_nsat_dn8 = assign48020_e81007_d_n8;
        locals.var_nsat_dn9 = assign48020_e81007_d_n9;
        locals.var_nsat_dn10 = assign48020_e81007_d_n10;
        locals.var_nsat_dn11 = assign48020_e81007_d_n11;
        locals.var_nsat_rv = 0.0;

        let (assign48030_e81035, assign48030_e81035_d_n3, assign48030_e81035_d_n4, assign48030_e81035_d_n5, assign48030_e81035_d_n6, assign48030_e81035_d_n7, assign48030_e81035_d_n8, assign48030_e81035_d_n9, assign48030_e81035_d_n10, assign48030_e81035_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48030_e81013: f64 = (locals.var_nsat + 1.0);
        let assign48030_e81016: f64 = (locals.var_nsat - 1.0);
        let assign48030_e81019: f64 = (locals.var_nsat - 1.0);
        let assign48030_e81020: f64 = (assign48030_e81016 * assign48030_e81019);
        let assign48030_e81023: f64 = (0.25 * 0.01);
        let assign48030_e81025: f64 = (assign48030_e81023 * 0.01);
        let assign48030_e81026: f64 = (assign48030_e81020 + assign48030_e81025);
        let assign48030_e81027: f64 = (assign48030_e81026).sqrt();
        let assign48030_e81028: f64 = (assign48030_e81013 - assign48030_e81027);
        let assign48030_e81029: f64 = (0.5 * assign48030_e81028);
        let assign48030_e81032: f64 = (0.25 * 0.01);
        let assign48030_e81033: f64 = (assign48030_e81029 + assign48030_e81032);
        (assign48030_e81033, (0.5 * (locals.var_nsat_dn3 - (((locals.var_nsat_dn3 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn3)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn4 - (((locals.var_nsat_dn4 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn4)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn5 - (((locals.var_nsat_dn5 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn5)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn6 - (((locals.var_nsat_dn6 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn6)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn7 - (((locals.var_nsat_dn7 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn7)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn8 - (((locals.var_nsat_dn8 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn8)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn9 - (((locals.var_nsat_dn9 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn9)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn10 - (((locals.var_nsat_dn10 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn10)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn11 - (((locals.var_nsat_dn11 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn11)) / (2.0 * assign48030_e81027)))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign48030_e81035;
        locals.var_nsat_dn3 = assign48030_e81035_d_n3;
        locals.var_nsat_dn4 = assign48030_e81035_d_n4;
        locals.var_nsat_dn5 = assign48030_e81035_d_n5;
        locals.var_nsat_dn6 = assign48030_e81035_d_n6;
        locals.var_nsat_dn7 = assign48030_e81035_d_n7;
        locals.var_nsat_dn8 = assign48030_e81035_d_n8;
        locals.var_nsat_dn9 = assign48030_e81035_d_n9;
        locals.var_nsat_dn10 = assign48030_e81035_d_n10;
        locals.var_nsat_dn11 = assign48030_e81035_d_n11;
        locals.var_nsat_rv = 0.0;

        let (assign48040_e81042, assign48040_e81042_d_n3, assign48040_e81042_d_n4, assign48040_e81042_d_n5, assign48040_e81042_d_n6, assign48040_e81042_d_n7, assign48040_e81042_d_n8, assign48040_e81042_d_n9, assign48040_e81042_d_n10, assign48040_e81042_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48040_e81040: f64 = (locals.var_qs_1 + locals.var_qdeff);
        (assign48040_e81040, (locals.var_qs_1_dn3 + locals.var_qdeff_dn3), (locals.var_qs_1_dn4 + locals.var_qdeff_dn4), (locals.var_qs_1_dn5 + locals.var_qdeff_dn5), (locals.var_qs_1_dn6 + locals.var_qdeff_dn6), (locals.var_qs_1_dn7 + locals.var_qdeff_dn7), (locals.var_qs_1_dn8 + locals.var_qdeff_dn8), (locals.var_qs_1_dn9 + locals.var_qdeff_dn9), (locals.var_qs_1_dn10 + locals.var_qdeff_dn10), (locals.var_qs_1_dn11 + locals.var_qdeff_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48040_e81042;
        locals.var_t0_dn3 = assign48040_e81042_d_n3;
        locals.var_t0_dn4 = assign48040_e81042_d_n4;
        locals.var_t0_dn5 = assign48040_e81042_d_n5;
        locals.var_t0_dn6 = assign48040_e81042_d_n6;
        locals.var_t0_dn7 = assign48040_e81042_d_n7;
        locals.var_t0_dn8 = assign48040_e81042_d_n8;
        locals.var_t0_dn9 = assign48040_e81042_d_n9;
        locals.var_t0_dn10 = assign48040_e81042_d_n10;
        locals.var_t0_dn11 = assign48040_e81042_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48050_e81049, assign48050_e81049_d_n3, assign48050_e81049_d_n4, assign48050_e81049_d_n5, assign48050_e81049_d_n6, assign48050_e81049_d_n7, assign48050_e81049_d_n8, assign48050_e81049_d_n9, assign48050_e81049_d_n10, assign48050_e81049_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48050_e81047: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign48050_e81047, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48050_e81049;
        locals.var_t1_dn3 = assign48050_e81049_d_n3;
        locals.var_t1_dn4 = assign48050_e81049_d_n4;
        locals.var_t1_dn5 = assign48050_e81049_d_n5;
        locals.var_t1_dn6 = assign48050_e81049_d_n6;
        locals.var_t1_dn7 = assign48050_e81049_d_n7;
        locals.var_t1_dn8 = assign48050_e81049_d_n8;
        locals.var_t1_dn9 = assign48050_e81049_d_n9;
        locals.var_t1_dn10 = assign48050_e81049_d_n10;
        locals.var_t1_dn11 = assign48050_e81049_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48060_e81058, assign48060_e81058_d_n3, assign48060_e81058_d_n4, assign48060_e81058_d_n5, assign48060_e81058_d_n6, assign48060_e81058_d_n7, assign48060_e81058_d_n8, assign48060_e81058_d_n9, assign48060_e81058_d_n10, assign48060_e81058_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48060_e81055: f64 = (locals.var_t0 + locals.var_m0_t);
        let assign48060_e81056: f64 = (locals.var_t1 / assign48060_e81055);
        (assign48060_e81056, (((locals.var_t1_dn3 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn3)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn4 * assign48060_e81055) - (locals.var_t1 * (locals.var_t0_dn4 + locals.var_m0_t_dn4))) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn5 * assign48060_e81055) - (locals.var_t1 * (locals.var_t0_dn5 + locals.var_m0_t_dn5))) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn6 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn6)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn7 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn7)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn8 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn8)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn9 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn9)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn10 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn10)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn11 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn11)) / (assign48060_e81055 * assign48060_e81055)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48060_e81058;
        locals.var_t2_dn3 = assign48060_e81058_d_n3;
        locals.var_t2_dn4 = assign48060_e81058_d_n4;
        locals.var_t2_dn5 = assign48060_e81058_d_n5;
        locals.var_t2_dn6 = assign48060_e81058_d_n6;
        locals.var_t2_dn7 = assign48060_e81058_d_n7;
        locals.var_t2_dn8 = assign48060_e81058_d_n8;
        locals.var_t2_dn9 = assign48060_e81058_d_n9;
        locals.var_t2_dn10 = assign48060_e81058_d_n10;
        locals.var_t2_dn11 = assign48060_e81058_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign48070_e81067, assign48070_e81067_d_n3, assign48070_e81067_d_n4, assign48070_e81067_d_n5, assign48070_e81067_d_n6, assign48070_e81067_d_n7, assign48070_e81067_d_n8, assign48070_e81067_d_n9, assign48070_e81067_d_n10, assign48070_e81067_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48070_e81063: f64 = (locals.var_k0_t * locals.var_t2);
        let assign48070_e81065: f64 = (assign48070_e81063 * locals.var_t2);
        (assign48070_e81065, (((locals.var_k0_t * locals.var_t2_dn3) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn3)), ((((locals.var_k0_t_dn4 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn4)) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn4)), ((((locals.var_k0_t_dn5 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn5)) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn5)), (((locals.var_k0_t * locals.var_t2_dn6) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn6)), (((locals.var_k0_t * locals.var_t2_dn7) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn7)), (((locals.var_k0_t * locals.var_t2_dn8) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn8)), (((locals.var_k0_t * locals.var_t2_dn9) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn9)), (((locals.var_k0_t * locals.var_t2_dn10) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn10)), (((locals.var_k0_t * locals.var_t2_dn11) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48070_e81067;
        locals.var_t3_dn3 = assign48070_e81067_d_n3;
        locals.var_t3_dn4 = assign48070_e81067_d_n4;
        locals.var_t3_dn5 = assign48070_e81067_d_n5;
        locals.var_t3_dn6 = assign48070_e81067_d_n6;
        locals.var_t3_dn7 = assign48070_e81067_d_n7;
        locals.var_t3_dn8 = assign48070_e81067_d_n8;
        locals.var_t3_dn9 = assign48070_e81067_d_n9;
        locals.var_t3_dn10 = assign48070_e81067_d_n10;
        locals.var_t3_dn11 = assign48070_e81067_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign48080_e81074, assign48080_e81074_d_n3, assign48080_e81074_d_n4, assign48080_e81074_d_n5, assign48080_e81074_d_n6, assign48080_e81074_d_n7, assign48080_e81074_d_n8, assign48080_e81074_d_n9, assign48080_e81074_d_n10, assign48080_e81074_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48080_e81072: f64 = (1.0 + locals.var_t3);
        (assign48080_e81072, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8, locals.var_mnud_dn9, locals.var_mnud_dn10, locals.var_mnud_dn11,)
    }
};
        locals.var_mnud = assign48080_e81074;
        locals.var_mnud_dn3 = assign48080_e81074_d_n3;
        locals.var_mnud_dn4 = assign48080_e81074_d_n4;
        locals.var_mnud_dn5 = assign48080_e81074_d_n5;
        locals.var_mnud_dn6 = assign48080_e81074_d_n6;
        locals.var_mnud_dn7 = assign48080_e81074_d_n7;
        locals.var_mnud_dn8 = assign48080_e81074_d_n8;
        locals.var_mnud_dn9 = assign48080_e81074_d_n9;
        locals.var_mnud_dn10 = assign48080_e81074_d_n10;
        locals.var_mnud_dn11 = assign48080_e81074_d_n11;
        locals.var_mnud_rv = 0.0;

        let (assign48090_e81097, assign48090_e81097_d_n3, assign48090_e81097_d_n4, assign48090_e81097_d_n5, assign48090_e81097_d_n6, assign48090_e81097_d_n7, assign48090_e81097_d_n8, assign48090_e81097_d_n9, assign48090_e81097_d_n10, assign48090_e81097_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48090_e81082: f64 = (locals.var_c0sisat_t * locals.var_t1);
        let assign48090_e81084: f64 = (assign48090_e81082 * locals.var_t1);
        let assign48090_e81085: f64 = (locals.var_c0si_t + assign48090_e81084);
        let assign48090_e81086: f64 = (0.0_f64).max(assign48090_e81085);
        let assign48090_e81088: f64 = (assign48090_e81086 * locals.var_t0);
        let assign48090_e81091: f64 = (2.0 * locals.var_n);
        let assign48090_e81093: f64 = (assign48090_e81091 * locals.var_vtm);
        let assign48090_e81094: f64 = (assign48090_e81088 + assign48090_e81093);
        let assign48090_e81095: f64 = (locals.var_c0_t / assign48090_e81094);
        (assign48090_e81095, (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn3) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn3)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn3)) + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (((locals.var_c0_t_dn4 * assign48090_e81094) - (locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (locals.var_c0si_t_dn4 + ((((locals.var_c0sisat_t_dn4 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn4)) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn4))) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn4)) + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign48090_e81091 * locals.var_vtm_dn4))))) / (assign48090_e81094 * assign48090_e81094)), (((locals.var_c0_t_dn5 * assign48090_e81094) - (locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (locals.var_c0si_t_dn5 + ((((locals.var_c0sisat_t_dn5 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn5)) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn5))) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn5)) + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign48090_e81091 * locals.var_vtm_dn5))))) / (assign48090_e81094 * assign48090_e81094)), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn6) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn6)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn6)) + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn7) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn7)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn7)) + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn8) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn8)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn8)) + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn9) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn9)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn9)) + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn10) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn10)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn10)) + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn11) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn11)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn11)) + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign48090_e81097;
        locals.var_t9_dn3 = assign48090_e81097_d_n3;
        locals.var_t9_dn4 = assign48090_e81097_d_n4;
        locals.var_t9_dn5 = assign48090_e81097_d_n5;
        locals.var_t9_dn6 = assign48090_e81097_d_n6;
        locals.var_t9_dn7 = assign48090_e81097_d_n7;
        locals.var_t9_dn8 = assign48090_e81097_d_n8;
        locals.var_t9_dn9 = assign48090_e81097_d_n9;
        locals.var_t9_dn10 = assign48090_e81097_d_n10;
        locals.var_t9_dn11 = assign48090_e81097_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign48100_e81104, assign48100_e81104_d_n3, assign48100_e81104_d_n4, assign48100_e81104_d_n5, assign48100_e81104_d_n6, assign48100_e81104_d_n7, assign48100_e81104_d_n8, assign48100_e81104_d_n9, assign48100_e81104_d_n10, assign48100_e81104_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48100_e81101: f64 = (-locals.var_t9);
        let assign48100_e81102: f64 = { let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48100_e81102, ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn3)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn4)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn5)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn6)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn7)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn8)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn9)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn10)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn11)),)
    } else {
        (locals.var_mnud1, locals.var_mnud1_dn3, locals.var_mnud1_dn4, locals.var_mnud1_dn5, locals.var_mnud1_dn6, locals.var_mnud1_dn7, locals.var_mnud1_dn8, locals.var_mnud1_dn9, locals.var_mnud1_dn10, locals.var_mnud1_dn11,)
    }
};
        locals.var_mnud1 = assign48100_e81104;
        locals.var_mnud1_dn3 = assign48100_e81104_d_n3;
        locals.var_mnud1_dn4 = assign48100_e81104_d_n4;
        locals.var_mnud1_dn5 = assign48100_e81104_d_n5;
        locals.var_mnud1_dn6 = assign48100_e81104_d_n6;
        locals.var_mnud1_dn7 = assign48100_e81104_d_n7;
        locals.var_mnud1_dn8 = assign48100_e81104_d_n8;
        locals.var_mnud1_dn9 = assign48100_e81104_d_n9;
        locals.var_mnud1_dn10 = assign48100_e81104_d_n10;
        locals.var_mnud1_dn11 = assign48100_e81104_d_n11;
        locals.var_mnud1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_168(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48110_e81113, assign48110_e81113_d_n3, assign48110_e81113_d_n4, assign48110_e81113_d_n5, assign48110_e81113_d_n6, assign48110_e81113_d_n7, assign48110_e81113_d_n8, assign48110_e81113_d_n9, assign48110_e81113_d_n10, assign48110_e81113_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48110_e81109: f64 = (locals.var_dmob * locals.var_dvsat);
        let assign48110_e81111: f64 = (assign48110_e81109 * locals.var_dr);
        (assign48110_e81111, ((((locals.var_dmob_dn3 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn3)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn3)), ((((locals.var_dmob_dn4 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn4)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn4)), ((((locals.var_dmob_dn5 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn5)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn5)), ((((locals.var_dmob_dn6 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn6)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn6)), ((((locals.var_dmob_dn7 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn7)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn7)), ((((locals.var_dmob_dn8 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn8)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn8)), ((((locals.var_dmob_dn9 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn9)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn9)), ((((locals.var_dmob_dn10 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn10)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn10)), ((((locals.var_dmob_dn11 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn11)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn11)),)
    } else {
        (locals.var_dtot, locals.var_dtot_dn3, locals.var_dtot_dn4, locals.var_dtot_dn5, locals.var_dtot_dn6, locals.var_dtot_dn7, locals.var_dtot_dn8, locals.var_dtot_dn9, locals.var_dtot_dn10, locals.var_dtot_dn11,)
    }
};
        locals.var_dtot = assign48110_e81113;
        locals.var_dtot_dn3 = assign48110_e81113_d_n3;
        locals.var_dtot_dn4 = assign48110_e81113_d_n4;
        locals.var_dtot_dn5 = assign48110_e81113_d_n5;
        locals.var_dtot_dn6 = assign48110_e81113_d_n6;
        locals.var_dtot_dn7 = assign48110_e81113_d_n7;
        locals.var_dtot_dn8 = assign48110_e81113_d_n8;
        locals.var_dtot_dn9 = assign48110_e81113_d_n9;
        locals.var_dtot_dn10 = assign48110_e81113_d_n10;
        locals.var_dtot_dn11 = assign48110_e81113_d_n11;
        locals.var_dtot_rv = 0.0;

        let (assign48120_e81120, assign48120_e81120_d_n3, assign48120_e81120_d_n4, assign48120_e81120_d_n5, assign48120_e81120_d_n6, assign48120_e81120_d_n7, assign48120_e81120_d_n8, assign48120_e81120_d_n9, assign48120_e81120_d_n10, assign48120_e81120_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48120_e81118: f64 = (locals.var_u0_a / locals.var_dtot);
        (assign48120_e81118, (((locals.var_u0_a_dn3 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn3)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn4 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn4)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn5 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn5)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn6 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn6)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn7 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn7)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn8 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn8)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn9 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn9)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn10 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn10)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn11 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn11)) / (locals.var_dtot * locals.var_dtot)),)
    } else {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    }
};
        locals.var_ueff = assign48120_e81120;
        locals.var_ueff_dn3 = assign48120_e81120_d_n3;
        locals.var_ueff_dn4 = assign48120_e81120_d_n4;
        locals.var_ueff_dn5 = assign48120_e81120_d_n5;
        locals.var_ueff_dn6 = assign48120_e81120_d_n6;
        locals.var_ueff_dn7 = assign48120_e81120_d_n7;
        locals.var_ueff_dn8 = assign48120_e81120_d_n8;
        locals.var_ueff_dn9 = assign48120_e81120_d_n9;
        locals.var_ueff_dn10 = assign48120_e81120_d_n10;
        locals.var_ueff_dn11 = assign48120_e81120_d_n11;
        locals.var_ueff_rv = 0.0;

        let (assign48130_e81159, assign48130_e81159_d_n3, assign48130_e81159_d_n4, assign48130_e81159_d_n5, assign48130_e81159_d_n6, assign48130_e81159_d_n7, assign48130_e81159_d_n8, assign48130_e81159_d_n9, assign48130_e81159_d_n10, assign48130_e81159_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48130_e81125: f64 = (2.0 * p.p2);
        let assign48130_e81127: f64 = (assign48130_e81125 * locals.var_nq);
        let assign48130_e81129: f64 = (assign48130_e81127 * locals.var_ueff);
        let assign48130_e81131: f64 = (assign48130_e81129 * locals.var_weff);
        let assign48130_e81133: f64 = (assign48130_e81131 / locals.var_leff);
        let assign48130_e81135: f64 = (assign48130_e81133 * locals.var_cox);
        let assign48130_e81137: f64 = (assign48130_e81135 * locals.var_nvt);
        let assign48130_e81139: f64 = (assign48130_e81137 * locals.var_nvt);
        let assign48130_e81142: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign48130_e81145: f64 = (1.0 + locals.var_qs_1);
        let assign48130_e81147: f64 = (assign48130_e81145 + locals.var_qdeff);
        let assign48130_e81148: f64 = (assign48130_e81142 * assign48130_e81147);
        let assign48130_e81149: f64 = (assign48130_e81139 * assign48130_e81148);
        let assign48130_e81151: f64 = (assign48130_e81149 * locals.var_moc);
        let assign48130_e81153: f64 = (assign48130_e81151 / locals.var_nsat);
        let assign48130_e81155: f64 = (assign48130_e81153 * locals.var_mnud);
        let assign48130_e81157: f64 = (assign48130_e81155 * locals.var_mnud1);
        (assign48130_e81157, (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn3) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn3)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn3 - locals.var_qdeff_dn3) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn3)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn3)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn3)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn3)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn4) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn4)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn4 - locals.var_qdeff_dn4) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn4)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn4)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn4)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn4)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn5) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn5)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn5 - locals.var_qdeff_dn5) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn5)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn5)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn5)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn5)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn6) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn6)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn6 - locals.var_qdeff_dn6) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn6)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn6)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn6)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn6)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn7) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn7)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn7 - locals.var_qdeff_dn7) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn7)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn7)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn7)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn7)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn8) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn8)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn8 - locals.var_qdeff_dn8) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn8)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn8)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn8)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn8)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn9) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn9)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn9 - locals.var_qdeff_dn9) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn9)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn9)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn9)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn9)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn10) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn10)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn10 - locals.var_qdeff_dn10) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn10)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn10)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn10)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn10)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn11) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn11)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn11 - locals.var_qdeff_dn11) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn11)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn11)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn11)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn11)),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign48130_e81159;
        locals.var_ids_dn3 = assign48130_e81159_d_n3;
        locals.var_ids_dn4 = assign48130_e81159_d_n4;
        locals.var_ids_dn5 = assign48130_e81159_d_n5;
        locals.var_ids_dn6 = assign48130_e81159_d_n6;
        locals.var_ids_dn7 = assign48130_e81159_d_n7;
        locals.var_ids_dn8 = assign48130_e81159_d_n8;
        locals.var_ids_dn9 = assign48130_e81159_d_n9;
        locals.var_ids_dn10 = assign48130_e81159_d_n10;
        locals.var_ids_dn11 = assign48130_e81159_d_n11;
        locals.var_ids_rv = 0.0;

        let (assign48140_e81166, assign48140_e81166_d_n3, assign48140_e81166_d_n4, assign48140_e81166_d_n5, assign48140_e81166_d_n6, assign48140_e81166_d_n7, assign48140_e81166_d_n8, assign48140_e81166_d_n9, assign48140_e81166_d_n10, assign48140_e81166_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48140_e81164: f64 = (locals.var_ids * p.p26);
        (assign48140_e81164, (locals.var_ids_dn3 * p.p26), (locals.var_ids_dn4 * p.p26), (locals.var_ids_dn5 * p.p26), (locals.var_ids_dn6 * p.p26), (locals.var_ids_dn7 * p.p26), (locals.var_ids_dn8 * p.p26), (locals.var_ids_dn9 * p.p26), (locals.var_ids_dn10 * p.p26), (locals.var_ids_dn11 * p.p26),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign48140_e81166;
        locals.var_ids_dn3 = assign48140_e81166_d_n3;
        locals.var_ids_dn4 = assign48140_e81166_d_n4;
        locals.var_ids_dn5 = assign48140_e81166_d_n5;
        locals.var_ids_dn6 = assign48140_e81166_d_n6;
        locals.var_ids_dn7 = assign48140_e81166_d_n7;
        locals.var_ids_dn8 = assign48140_e81166_d_n8;
        locals.var_ids_dn9 = assign48140_e81166_d_n9;
        locals.var_ids_dn10 = assign48140_e81166_d_n10;
        locals.var_ids_dn11 = assign48140_e81166_d_n11;
        locals.var_ids_rv = 0.0;

        let (assign48150_e81171, assign48150_e81171_d_n3, assign48150_e81171_d_n4, assign48150_e81171_d_n5, assign48150_e81171_d_n6, assign48150_e81171_d_n7, assign48150_e81171_d_n8, assign48150_e81171_d_n9, assign48150_e81171_d_n10, assign48150_e81171_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48150_e81171;
        locals.var_gcrg_dn3 = assign48150_e81171_d_n3;
        locals.var_gcrg_dn4 = assign48150_e81171_d_n4;
        locals.var_gcrg_dn5 = assign48150_e81171_d_n5;
        locals.var_gcrg_dn6 = assign48150_e81171_d_n6;
        locals.var_gcrg_dn7 = assign48150_e81171_d_n7;
        locals.var_gcrg_dn8 = assign48150_e81171_d_n8;
        locals.var_gcrg_dn9 = assign48150_e81171_d_n9;
        locals.var_gcrg_dn10 = assign48150_e81171_d_n10;
        locals.var_gcrg_dn11 = assign48150_e81171_d_n11;
        locals.var_gcrg_rv = 0.0;

        let assign48160_e81174: f64 = if p.p7 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign48160_e81174;
        locals.var_guard741_rv = 0.0;

        let (assign48170_e81189, assign48170_e81189_d_n3, assign48170_e81189_d_n4, assign48170_e81189_d_n5, assign48170_e81189_d_n6, assign48170_e81189_d_n7, assign48170_e81189_d_n8, assign48170_e81189_d_n9, assign48170_e81189_d_n10, assign48170_e81189_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48170_e81181: f64 = (locals.var_ueff * locals.var_weff);
        let assign48170_e81183: f64 = (assign48170_e81181 / locals.var_leff);
        let assign48170_e81185: f64 = (assign48170_e81183 * locals.var_cox);
        let assign48170_e81187: f64 = (assign48170_e81185 * locals.var_qia);
        (assign48170_e81187, (((((locals.var_ueff_dn3 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn3)), (((((locals.var_ueff_dn4 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn4)), (((((locals.var_ueff_dn5 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn5)), (((((locals.var_ueff_dn6 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn6)), (((((locals.var_ueff_dn7 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn7)), (((((locals.var_ueff_dn8 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn8)), (((((locals.var_ueff_dn9 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn9)), (((((locals.var_ueff_dn10 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn10)), (((((locals.var_ueff_dn11 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn11)),)
    } else {
        (locals.var_idsovvds, locals.var_idsovvds_dn3, locals.var_idsovvds_dn4, locals.var_idsovvds_dn5, locals.var_idsovvds_dn6, locals.var_idsovvds_dn7, locals.var_idsovvds_dn8, locals.var_idsovvds_dn9, locals.var_idsovvds_dn10, locals.var_idsovvds_dn11,)
    }
};
        locals.var_idsovvds = assign48170_e81189;
        locals.var_idsovvds_dn3 = assign48170_e81189_d_n3;
        locals.var_idsovvds_dn4 = assign48170_e81189_d_n4;
        locals.var_idsovvds_dn5 = assign48170_e81189_d_n5;
        locals.var_idsovvds_dn6 = assign48170_e81189_d_n6;
        locals.var_idsovvds_dn7 = assign48170_e81189_d_n7;
        locals.var_idsovvds_dn8 = assign48170_e81189_d_n8;
        locals.var_idsovvds_dn9 = assign48170_e81189_d_n9;
        locals.var_idsovvds_dn10 = assign48170_e81189_d_n10;
        locals.var_idsovvds_dn11 = assign48170_e81189_d_n11;
        locals.var_idsovvds_rv = 0.0;

        let (assign48180_e81198, assign48180_e81198_d_n3, assign48180_e81198_d_n4, assign48180_e81198_d_n5, assign48180_e81198_d_n6, assign48180_e81198_d_n7, assign48180_e81198_d_n8, assign48180_e81198_d_n9, assign48180_e81198_d_n10, assign48180_e81198_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48180_e81196: f64 = (p.p1009 * locals.var_vt);
        (assign48180_e81196, 0.0, (p.p1009 * locals.var_vt_dn4), (p.p1009 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign48180_e81198;
        locals.var_t9_dn3 = assign48180_e81198_d_n3;
        locals.var_t9_dn4 = assign48180_e81198_d_n4;
        locals.var_t9_dn5 = assign48180_e81198_d_n5;
        locals.var_t9_dn6 = assign48180_e81198_d_n6;
        locals.var_t9_dn7 = assign48180_e81198_d_n7;
        locals.var_t9_dn8 = assign48180_e81198_d_n8;
        locals.var_t9_dn9 = assign48180_e81198_d_n9;
        locals.var_t9_dn10 = assign48180_e81198_d_n10;
        locals.var_t9_dn11 = assign48180_e81198_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign48190_e81213, assign48190_e81213_d_n3, assign48190_e81213_d_n4, assign48190_e81213_d_n5, assign48190_e81213_d_n6, assign48190_e81213_d_n7, assign48190_e81213_d_n8, assign48190_e81213_d_n9, assign48190_e81213_d_n10, assign48190_e81213_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48190_e81205: f64 = (locals.var_t9 * locals.var_ueff);
        let assign48190_e81207: f64 = (assign48190_e81205 * locals.var_weff);
        let assign48190_e81209: f64 = (assign48190_e81207 / locals.var_leff);
        let assign48190_e81211: f64 = (assign48190_e81209 * locals.var_cox);
        (assign48190_e81211, (((((locals.var_t9_dn3 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn4 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn5 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn6 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn7 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn8 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn9 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn10 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn11 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48190_e81213;
        locals.var_t0_dn3 = assign48190_e81213_d_n3;
        locals.var_t0_dn4 = assign48190_e81213_d_n4;
        locals.var_t0_dn5 = assign48190_e81213_d_n5;
        locals.var_t0_dn6 = assign48190_e81213_d_n6;
        locals.var_t0_dn7 = assign48190_e81213_d_n7;
        locals.var_t0_dn8 = assign48190_e81213_d_n8;
        locals.var_t0_dn9 = assign48190_e81213_d_n9;
        locals.var_t0_dn10 = assign48190_e81213_d_n10;
        locals.var_t0_dn11 = assign48190_e81213_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48200_e81226, assign48200_e81226_d_n3, assign48200_e81226_d_n4, assign48200_e81226_d_n5, assign48200_e81226_d_n6, assign48200_e81226_d_n7, assign48200_e81226_d_n8, assign48200_e81226_d_n9, assign48200_e81226_d_n10, assign48200_e81226_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48200_e81220: f64 = (p.p1008 * p.p2);
        let assign48200_e81223: f64 = (locals.var_t0 + locals.var_idsovvds);
        let assign48200_e81224: f64 = (assign48200_e81220 * assign48200_e81223);
        (assign48200_e81224, (assign48200_e81220 * (locals.var_t0_dn3 + locals.var_idsovvds_dn3)), (assign48200_e81220 * (locals.var_t0_dn4 + locals.var_idsovvds_dn4)), (assign48200_e81220 * (locals.var_t0_dn5 + locals.var_idsovvds_dn5)), (assign48200_e81220 * (locals.var_t0_dn6 + locals.var_idsovvds_dn6)), (assign48200_e81220 * (locals.var_t0_dn7 + locals.var_idsovvds_dn7)), (assign48200_e81220 * (locals.var_t0_dn8 + locals.var_idsovvds_dn8)), (assign48200_e81220 * (locals.var_t0_dn9 + locals.var_idsovvds_dn9)), (assign48200_e81220 * (locals.var_t0_dn10 + locals.var_idsovvds_dn10)), (assign48200_e81220 * (locals.var_t0_dn11 + locals.var_idsovvds_dn11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48200_e81226;
        locals.var_gcrg_dn3 = assign48200_e81226_d_n3;
        locals.var_gcrg_dn4 = assign48200_e81226_d_n4;
        locals.var_gcrg_dn5 = assign48200_e81226_d_n5;
        locals.var_gcrg_dn6 = assign48200_e81226_d_n6;
        locals.var_gcrg_dn7 = assign48200_e81226_d_n7;
        locals.var_gcrg_dn8 = assign48200_e81226_d_n8;
        locals.var_gcrg_dn9 = assign48200_e81226_d_n9;
        locals.var_gcrg_dn10 = assign48200_e81226_d_n10;
        locals.var_gcrg_dn11 = assign48200_e81226_d_n11;
        locals.var_gcrg_rv = 0.0;

        let assign48210_e81229: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign48210_e81229;
        locals.var_guard742_rv = 0.0;

        let (assign48220_e81240,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48220_e81238: f64 = (1.0 / locals.var_grgeltd);
        (assign48220_e81238,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign48220_e81240;
        locals.var_rgeltd_rv = 0.0;

        let assign48230_e81243: f64 = if locals.var_rgeltd < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign48230_e81243;
        locals.var_guard743_rv = 0.0;

        let (assign48240_e81254,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) && (locals.var_guard743 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign48240_e81254;
        locals.var_rgeltd_rv = 0.0;

        let (assign48250_e81267,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign48250_e81265: f64 = (1.0 / locals.var_rgeltd);
        (assign48250_e81265,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign48250_e81267;
        locals.var_grgeltd_rv = 0.0;

        let (assign48260_e81278, assign48260_e81278_d_n3, assign48260_e81278_d_n4, assign48260_e81278_d_n5, assign48260_e81278_d_n6, assign48260_e81278_d_n7, assign48260_e81278_d_n8, assign48260_e81278_d_n9, assign48260_e81278_d_n10, assign48260_e81278_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48260_e81276: f64 = (locals.var_grgeltd + locals.var_gcrg);
        (assign48260_e81276, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48260_e81278;
        locals.var_t11_dn3 = assign48260_e81278_d_n3;
        locals.var_t11_dn4 = assign48260_e81278_d_n4;
        locals.var_t11_dn5 = assign48260_e81278_d_n5;
        locals.var_t11_dn6 = assign48260_e81278_d_n6;
        locals.var_t11_dn7 = assign48260_e81278_d_n7;
        locals.var_t11_dn8 = assign48260_e81278_d_n8;
        locals.var_t11_dn9 = assign48260_e81278_d_n9;
        locals.var_t11_dn10 = assign48260_e81278_d_n10;
        locals.var_t11_dn11 = assign48260_e81278_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48270_e81291, assign48270_e81291_d_n3, assign48270_e81291_d_n4, assign48270_e81291_d_n5, assign48270_e81291_d_n6, assign48270_e81291_d_n7, assign48270_e81291_d_n8, assign48270_e81291_d_n9, assign48270_e81291_d_n10, assign48270_e81291_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48270_e81287: f64 = (locals.var_grgeltd * locals.var_gcrg);
        let assign48270_e81289: f64 = (assign48270_e81287 / locals.var_t11);
        (assign48270_e81289, ((((locals.var_grgeltd * locals.var_gcrg_dn3) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn4) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn5) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn6) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn7) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn8) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn9) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn10) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn11) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48270_e81291;
        locals.var_gcrg_dn3 = assign48270_e81291_d_n3;
        locals.var_gcrg_dn4 = assign48270_e81291_d_n4;
        locals.var_gcrg_dn5 = assign48270_e81291_d_n5;
        locals.var_gcrg_dn6 = assign48270_e81291_d_n6;
        locals.var_gcrg_dn7 = assign48270_e81291_d_n7;
        locals.var_gcrg_dn8 = assign48270_e81291_d_n8;
        locals.var_gcrg_dn9 = assign48270_e81291_d_n9;
        locals.var_gcrg_dn10 = assign48270_e81291_d_n10;
        locals.var_gcrg_dn11 = assign48270_e81291_d_n11;
        locals.var_gcrg_rv = 0.0;

        let (assign48280_e81300,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48280_e81296: f64 = (locals.var_weff / p.p1373);
        let assign48280_e81298: f64 = (assign48280_e81296 + p.p1377);
        (assign48280_e81298,)
    } else {
        (locals.var_wdiod,)
    }
};
        locals.var_wdiod = assign48280_e81300;
        locals.var_wdiod_rv = 0.0;

        let (assign48290_e81309,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48290_e81305: f64 = (locals.var_weff / p.p1373);
        let assign48290_e81307: f64 = (assign48290_e81305 + p.p1378);
        (assign48290_e81307,)
    } else {
        (locals.var_wdios,)
    }
};
        locals.var_wdios = assign48290_e81309;
        locals.var_wdios_rv = 0.0;

        let (assign48300_e81316,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48300_e81314: f64 = (locals.var_wdios * p.p74);
        (assign48300_e81314,)
    } else {
        (locals.var_wstsi,)
    }
};
        locals.var_wstsi = assign48300_e81316;
        locals.var_wstsi_rv = 0.0;

        let (assign48310_e81323,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48310_e81321: f64 = (locals.var_wdiod * p.p74);
        (assign48310_e81321,)
    } else {
        (locals.var_wdtsi,)
    }
};
        locals.var_wdtsi = assign48310_e81323;
        locals.var_wdtsi_rv = 0.0;

        let (assign48320_e81330, assign48320_e81330_d_n4, assign48320_e81330_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48320_e81328: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign48320_e81328, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5,)
    }
};
        locals.var_nvtm1 = assign48320_e81330;
        locals.var_nvtm1_dn4 = assign48320_e81330_d_n4;
        locals.var_nvtm1_dn5 = assign48320_e81330_d_n5;
        locals.var_nvtm1_rv = 0.0;

        let (assign48330_e81337, assign48330_e81337_d_n3, assign48330_e81337_d_n4, assign48330_e81337_d_n5, assign48330_e81337_d_n6, assign48330_e81337_d_n7, assign48330_e81337_d_n8, assign48330_e81337_d_n9, assign48330_e81337_d_n10, assign48330_e81337_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48330_e81335: f64 = (locals.var_vbs_jct / locals.var_nvtm1);
        (assign48330_e81335, 0.0, (-((locals.var_vbs_jct * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vbs_jct * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtm1), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtm1), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48330_e81337;
        locals.var_t0_dn3 = assign48330_e81337_d_n3;
        locals.var_t0_dn4 = assign48330_e81337_d_n4;
        locals.var_t0_dn5 = assign48330_e81337_d_n5;
        locals.var_t0_dn6 = assign48330_e81337_d_n6;
        locals.var_t0_dn7 = assign48330_e81337_d_n7;
        locals.var_t0_dn8 = assign48330_e81337_d_n8;
        locals.var_t0_dn9 = assign48330_e81337_d_n9;
        locals.var_t0_dn10 = assign48330_e81337_d_n10;
        locals.var_t0_dn11 = assign48330_e81337_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48340_e81343, assign48340_e81343_d_n3, assign48340_e81343_d_n4, assign48340_e81343_d_n5, assign48340_e81343_d_n6, assign48340_e81343_d_n7, assign48340_e81343_d_n8, assign48340_e81343_d_n9, assign48340_e81343_d_n10, assign48340_e81343_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48340_e81341: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48340_e81341, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11,)
    }
};
        locals.var_expvbsnvtm = assign48340_e81343;
        locals.var_expvbsnvtm_dn3 = assign48340_e81343_d_n3;
        locals.var_expvbsnvtm_dn4 = assign48340_e81343_d_n4;
        locals.var_expvbsnvtm_dn5 = assign48340_e81343_d_n5;
        locals.var_expvbsnvtm_dn6 = assign48340_e81343_d_n6;
        locals.var_expvbsnvtm_dn7 = assign48340_e81343_d_n7;
        locals.var_expvbsnvtm_dn8 = assign48340_e81343_d_n8;
        locals.var_expvbsnvtm_dn9 = assign48340_e81343_d_n9;
        locals.var_expvbsnvtm_dn10 = assign48340_e81343_d_n10;
        locals.var_expvbsnvtm_dn11 = assign48340_e81343_d_n11;
        locals.var_expvbsnvtm_rv = 0.0;

        let (assign48350_e81350, assign48350_e81350_d_n4, assign48350_e81350_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48350_e81348: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign48350_e81348, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign48350_e81350;
        locals.var_nvtm2_dn4 = assign48350_e81350_d_n4;
        locals.var_nvtm2_dn5 = assign48350_e81350_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let (assign48360_e81357, assign48360_e81357_d_n3, assign48360_e81357_d_n4, assign48360_e81357_d_n5, assign48360_e81357_d_n6, assign48360_e81357_d_n7, assign48360_e81357_d_n8, assign48360_e81357_d_n9, assign48360_e81357_d_n10, assign48360_e81357_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48360_e81355: f64 = (locals.var_vbd_jct / locals.var_nvtm2);
        (assign48360_e81355, 0.0, (-((locals.var_vbd_jct * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))), (-((locals.var_vbd_jct * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))), (locals.var_vbd_jct_dn6 / locals.var_nvtm2), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtm2), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48360_e81357;
        locals.var_t0_dn3 = assign48360_e81357_d_n3;
        locals.var_t0_dn4 = assign48360_e81357_d_n4;
        locals.var_t0_dn5 = assign48360_e81357_d_n5;
        locals.var_t0_dn6 = assign48360_e81357_d_n6;
        locals.var_t0_dn7 = assign48360_e81357_d_n7;
        locals.var_t0_dn8 = assign48360_e81357_d_n8;
        locals.var_t0_dn9 = assign48360_e81357_d_n9;
        locals.var_t0_dn10 = assign48360_e81357_d_n10;
        locals.var_t0_dn11 = assign48360_e81357_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48370_e81363, assign48370_e81363_d_n3, assign48370_e81363_d_n4, assign48370_e81363_d_n5, assign48370_e81363_d_n6, assign48370_e81363_d_n7, assign48370_e81363_d_n8, assign48370_e81363_d_n9, assign48370_e81363_d_n10, assign48370_e81363_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48370_e81361: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48370_e81361, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbdnvtm, locals.var_expvbdnvtm_dn3, locals.var_expvbdnvtm_dn4, locals.var_expvbdnvtm_dn5, locals.var_expvbdnvtm_dn6, locals.var_expvbdnvtm_dn7, locals.var_expvbdnvtm_dn8, locals.var_expvbdnvtm_dn9, locals.var_expvbdnvtm_dn10, locals.var_expvbdnvtm_dn11,)
    }
};
        locals.var_expvbdnvtm = assign48370_e81363;
        locals.var_expvbdnvtm_dn3 = assign48370_e81363_d_n3;
        locals.var_expvbdnvtm_dn4 = assign48370_e81363_d_n4;
        locals.var_expvbdnvtm_dn5 = assign48370_e81363_d_n5;
        locals.var_expvbdnvtm_dn6 = assign48370_e81363_d_n6;
        locals.var_expvbdnvtm_dn7 = assign48370_e81363_d_n7;
        locals.var_expvbdnvtm_dn8 = assign48370_e81363_d_n8;
        locals.var_expvbdnvtm_dn9 = assign48370_e81363_d_n9;
        locals.var_expvbdnvtm_dn10 = assign48370_e81363_d_n10;
        locals.var_expvbdnvtm_dn11 = assign48370_e81363_d_n11;
        locals.var_expvbdnvtm_rv = 0.0;

        let (assign48380_e81374, assign48380_e81374_d_n3, assign48380_e81374_d_n4, assign48380_e81374_d_n5, assign48380_e81374_d_n6, assign48380_e81374_d_n7, assign48380_e81374_d_n8, assign48380_e81374_d_n9, assign48380_e81374_d_n10, assign48380_e81374_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48380_e81368: f64 = (1.115 / locals.var_vtm);
        let assign48380_e81371: f64 = (locals.var_tratio - 1.0);
        let assign48380_e81372: f64 = (assign48380_e81368 * assign48380_e81371);
        (assign48380_e81372, 0.0, (((-((1.115 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) * assign48380_e81371) + (assign48380_e81368 * locals.var_tratio_dn4)), (((-((1.115 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) * assign48380_e81371) + (assign48380_e81368 * locals.var_tratio_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign48380_e81374;
        locals.var_t4_dn3 = assign48380_e81374_d_n3;
        locals.var_t4_dn4 = assign48380_e81374_d_n4;
        locals.var_t4_dn5 = assign48380_e81374_d_n5;
        locals.var_t4_dn6 = assign48380_e81374_d_n6;
        locals.var_t4_dn7 = assign48380_e81374_d_n7;
        locals.var_t4_dn8 = assign48380_e81374_d_n8;
        locals.var_t4_dn9 = assign48380_e81374_d_n9;
        locals.var_t4_dn10 = assign48380_e81374_d_n10;
        locals.var_t4_dn11 = assign48380_e81374_d_n11;
        locals.var_t4_rv = 0.0;

        let assign48390_e81377: f64 = if locals.var_isdif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign48390_e81377;
        locals.var_guard744_rv = 0.0;

        let (assign48410_e81396, assign48410_e81396_d_n3, assign48410_e81396_d_n4, assign48410_e81396_d_n5, assign48410_e81396_d_n6, assign48410_e81396_d_n7, assign48410_e81396_d_n8, assign48410_e81396_d_n9, assign48410_e81396_d_n10, assign48410_e81396_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48410_e81392: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign48410_e81394: f64 = (assign48410_e81392 / locals.var_ndiode_i);
        (assign48410_e81394, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48410_e81396;
        locals.var_t7_dn3 = assign48410_e81396_d_n3;
        locals.var_t7_dn4 = assign48410_e81396_d_n4;
        locals.var_t7_dn5 = assign48410_e81396_d_n5;
        locals.var_t7_dn6 = assign48410_e81396_d_n6;
        locals.var_t7_dn7 = assign48410_e81396_d_n7;
        locals.var_t7_dn8 = assign48410_e81396_d_n8;
        locals.var_t7_dn9 = assign48410_e81396_d_n9;
        locals.var_t7_dn10 = assign48410_e81396_d_n10;
        locals.var_t7_dn11 = assign48410_e81396_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48420_e81405, assign48420_e81405_d_n3, assign48420_e81405_d_n4, assign48420_e81405_d_n5, assign48420_e81405_d_n6, assign48420_e81405_d_n7, assign48420_e81405_d_n8, assign48420_e81405_d_n9, assign48420_e81405_d_n10, assign48420_e81405_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48420_e81403: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48420_e81403, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48420_e81405;
        locals.var_t1_dn3 = assign48420_e81405_d_n3;
        locals.var_t1_dn4 = assign48420_e81405_d_n4;
        locals.var_t1_dn5 = assign48420_e81405_d_n5;
        locals.var_t1_dn6 = assign48420_e81405_d_n6;
        locals.var_t1_dn7 = assign48420_e81405_d_n7;
        locals.var_t1_dn8 = assign48420_e81405_d_n8;
        locals.var_t1_dn9 = assign48420_e81405_d_n9;
        locals.var_t1_dn10 = assign48420_e81405_d_n10;
        locals.var_t1_dn11 = assign48420_e81405_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_169(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48430_e81415, assign48430_e81415_d_n3, assign48430_e81415_d_n4, assign48430_e81415_d_n5, assign48430_e81415_d_n6, assign48430_e81415_d_n7, assign48430_e81415_d_n8, assign48430_e81415_d_n9, assign48430_e81415_d_n10, assign48430_e81415_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48430_e81413: f64 = (locals.var_isdif_i * locals.var_t1);
        (assign48430_e81413, (locals.var_isdif_i * locals.var_t1_dn3), (locals.var_isdif_i * locals.var_t1_dn4), (locals.var_isdif_i * locals.var_t1_dn5), (locals.var_isdif_i * locals.var_t1_dn6), (locals.var_isdif_i * locals.var_t1_dn7), (locals.var_isdif_i * locals.var_t1_dn8), (locals.var_isdif_i * locals.var_t1_dn9), (locals.var_isdif_i * locals.var_t1_dn10), (locals.var_isdif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11,)
    }
};
        locals.var_jdifs = assign48430_e81415;
        locals.var_jdifs_dn3 = assign48430_e81415_d_n3;
        locals.var_jdifs_dn4 = assign48430_e81415_d_n4;
        locals.var_jdifs_dn5 = assign48430_e81415_d_n5;
        locals.var_jdifs_dn6 = assign48430_e81415_d_n6;
        locals.var_jdifs_dn7 = assign48430_e81415_d_n7;
        locals.var_jdifs_dn8 = assign48430_e81415_d_n8;
        locals.var_jdifs_dn9 = assign48430_e81415_d_n9;
        locals.var_jdifs_dn10 = assign48430_e81415_d_n10;
        locals.var_jdifs_dn11 = assign48430_e81415_d_n11;
        locals.var_jdifs_rv = 0.0;

        let (assign48440_e81425, assign48440_e81425_d_n3, assign48440_e81425_d_n4, assign48440_e81425_d_n5, assign48440_e81425_d_n6, assign48440_e81425_d_n7, assign48440_e81425_d_n8, assign48440_e81425_d_n9, assign48440_e81425_d_n10, assign48440_e81425_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48440_e81423: f64 = (locals.var_wstsi * locals.var_jdifs);
        (assign48440_e81423, (locals.var_wstsi * locals.var_jdifs_dn3), (locals.var_wstsi * locals.var_jdifs_dn4), (locals.var_wstsi * locals.var_jdifs_dn5), (locals.var_wstsi * locals.var_jdifs_dn6), (locals.var_wstsi * locals.var_jdifs_dn7), (locals.var_wstsi * locals.var_jdifs_dn8), (locals.var_wstsi * locals.var_jdifs_dn9), (locals.var_wstsi * locals.var_jdifs_dn10), (locals.var_wstsi * locals.var_jdifs_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48440_e81425;
        locals.var_t0_dn3 = assign48440_e81425_d_n3;
        locals.var_t0_dn4 = assign48440_e81425_d_n4;
        locals.var_t0_dn5 = assign48440_e81425_d_n5;
        locals.var_t0_dn6 = assign48440_e81425_d_n6;
        locals.var_t0_dn7 = assign48440_e81425_d_n7;
        locals.var_t0_dn8 = assign48440_e81425_d_n8;
        locals.var_t0_dn9 = assign48440_e81425_d_n9;
        locals.var_t0_dn10 = assign48440_e81425_d_n10;
        locals.var_t0_dn11 = assign48440_e81425_d_n11;
        locals.var_t0_rv = 0.0;

        let assign48460_e81440: f64 = if locals.var_iddif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign48460_e81440;
        locals.var_guard745_rv = 0.0;

        let (assign48480_e81459, assign48480_e81459_d_n3, assign48480_e81459_d_n4, assign48480_e81459_d_n5, assign48480_e81459_d_n6, assign48480_e81459_d_n7, assign48480_e81459_d_n8, assign48480_e81459_d_n9, assign48480_e81459_d_n10, assign48480_e81459_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48480_e81455: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign48480_e81457: f64 = (assign48480_e81455 / locals.var_ndiode_i);
        (assign48480_e81457, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48480_e81459;
        locals.var_t7_dn3 = assign48480_e81459_d_n3;
        locals.var_t7_dn4 = assign48480_e81459_d_n4;
        locals.var_t7_dn5 = assign48480_e81459_d_n5;
        locals.var_t7_dn6 = assign48480_e81459_d_n6;
        locals.var_t7_dn7 = assign48480_e81459_d_n7;
        locals.var_t7_dn8 = assign48480_e81459_d_n8;
        locals.var_t7_dn9 = assign48480_e81459_d_n9;
        locals.var_t7_dn10 = assign48480_e81459_d_n10;
        locals.var_t7_dn11 = assign48480_e81459_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48490_e81468, assign48490_e81468_d_n3, assign48490_e81468_d_n4, assign48490_e81468_d_n5, assign48490_e81468_d_n6, assign48490_e81468_d_n7, assign48490_e81468_d_n8, assign48490_e81468_d_n9, assign48490_e81468_d_n10, assign48490_e81468_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48490_e81466: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48490_e81466, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48490_e81468;
        locals.var_t1_dn3 = assign48490_e81468_d_n3;
        locals.var_t1_dn4 = assign48490_e81468_d_n4;
        locals.var_t1_dn5 = assign48490_e81468_d_n5;
        locals.var_t1_dn6 = assign48490_e81468_d_n6;
        locals.var_t1_dn7 = assign48490_e81468_d_n7;
        locals.var_t1_dn8 = assign48490_e81468_d_n8;
        locals.var_t1_dn9 = assign48490_e81468_d_n9;
        locals.var_t1_dn10 = assign48490_e81468_d_n10;
        locals.var_t1_dn11 = assign48490_e81468_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48500_e81478, assign48500_e81478_d_n3, assign48500_e81478_d_n4, assign48500_e81478_d_n5, assign48500_e81478_d_n6, assign48500_e81478_d_n7, assign48500_e81478_d_n8, assign48500_e81478_d_n9, assign48500_e81478_d_n10, assign48500_e81478_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48500_e81476: f64 = (locals.var_iddif_i * locals.var_t1);
        (assign48500_e81476, (locals.var_iddif_i * locals.var_t1_dn3), (locals.var_iddif_i * locals.var_t1_dn4), (locals.var_iddif_i * locals.var_t1_dn5), (locals.var_iddif_i * locals.var_t1_dn6), (locals.var_iddif_i * locals.var_t1_dn7), (locals.var_iddif_i * locals.var_t1_dn8), (locals.var_iddif_i * locals.var_t1_dn9), (locals.var_iddif_i * locals.var_t1_dn10), (locals.var_iddif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11,)
    }
};
        locals.var_jdifd = assign48500_e81478;
        locals.var_jdifd_dn3 = assign48500_e81478_d_n3;
        locals.var_jdifd_dn4 = assign48500_e81478_d_n4;
        locals.var_jdifd_dn5 = assign48500_e81478_d_n5;
        locals.var_jdifd_dn6 = assign48500_e81478_d_n6;
        locals.var_jdifd_dn7 = assign48500_e81478_d_n7;
        locals.var_jdifd_dn8 = assign48500_e81478_d_n8;
        locals.var_jdifd_dn9 = assign48500_e81478_d_n9;
        locals.var_jdifd_dn10 = assign48500_e81478_d_n10;
        locals.var_jdifd_dn11 = assign48500_e81478_d_n11;
        locals.var_jdifd_rv = 0.0;

        let (assign48510_e81488, assign48510_e81488_d_n3, assign48510_e81488_d_n4, assign48510_e81488_d_n5, assign48510_e81488_d_n6, assign48510_e81488_d_n7, assign48510_e81488_d_n8, assign48510_e81488_d_n9, assign48510_e81488_d_n10, assign48510_e81488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48510_e81486: f64 = (locals.var_wdtsi * locals.var_jdifd);
        (assign48510_e81486, (locals.var_wdtsi * locals.var_jdifd_dn3), (locals.var_wdtsi * locals.var_jdifd_dn4), (locals.var_wdtsi * locals.var_jdifd_dn5), (locals.var_wdtsi * locals.var_jdifd_dn6), (locals.var_wdtsi * locals.var_jdifd_dn7), (locals.var_wdtsi * locals.var_jdifd_dn8), (locals.var_wdtsi * locals.var_jdifd_dn9), (locals.var_wdtsi * locals.var_jdifd_dn10), (locals.var_wdtsi * locals.var_jdifd_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48510_e81488;
        locals.var_t0_dn3 = assign48510_e81488_d_n3;
        locals.var_t0_dn4 = assign48510_e81488_d_n4;
        locals.var_t0_dn5 = assign48510_e81488_d_n5;
        locals.var_t0_dn6 = assign48510_e81488_d_n6;
        locals.var_t0_dn7 = assign48510_e81488_d_n7;
        locals.var_t0_dn8 = assign48510_e81488_d_n8;
        locals.var_t0_dn9 = assign48510_e81488_d_n9;
        locals.var_t0_dn10 = assign48510_e81488_d_n10;
        locals.var_t0_dn11 = assign48510_e81488_d_n11;
        locals.var_t0_rv = 0.0;

        let assign48530_e81503: f64 = if locals.var_isrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign48530_e81503;
        locals.var_guard746_rv = 0.0;

        let (assign48550_e81522, assign48550_e81522_d_n3, assign48550_e81522_d_n4, assign48550_e81522_d_n5, assign48550_e81522_d_n6, assign48550_e81522_d_n7, assign48550_e81522_d_n8, assign48550_e81522_d_n9, assign48550_e81522_d_n10, assign48550_e81522_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48550_e81518: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign48550_e81520: f64 = (assign48550_e81518 / locals.var_nrecf0_i);
        (assign48550_e81520, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48550_e81522;
        locals.var_t7_dn3 = assign48550_e81522_d_n3;
        locals.var_t7_dn4 = assign48550_e81522_d_n4;
        locals.var_t7_dn5 = assign48550_e81522_d_n5;
        locals.var_t7_dn6 = assign48550_e81522_d_n6;
        locals.var_t7_dn7 = assign48550_e81522_d_n7;
        locals.var_t7_dn8 = assign48550_e81522_d_n8;
        locals.var_t7_dn9 = assign48550_e81522_d_n9;
        locals.var_t7_dn10 = assign48550_e81522_d_n10;
        locals.var_t7_dn11 = assign48550_e81522_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48560_e81531, assign48560_e81531_d_n3, assign48560_e81531_d_n4, assign48560_e81531_d_n5, assign48560_e81531_d_n6, assign48560_e81531_d_n7, assign48560_e81531_d_n8, assign48560_e81531_d_n9, assign48560_e81531_d_n10, assign48560_e81531_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48560_e81529: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48560_e81529, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48560_e81531;
        locals.var_t2_dn3 = assign48560_e81531_d_n3;
        locals.var_t2_dn4 = assign48560_e81531_d_n4;
        locals.var_t2_dn5 = assign48560_e81531_d_n5;
        locals.var_t2_dn6 = assign48560_e81531_d_n6;
        locals.var_t2_dn7 = assign48560_e81531_d_n7;
        locals.var_t2_dn8 = assign48560_e81531_d_n8;
        locals.var_t2_dn9 = assign48560_e81531_d_n9;
        locals.var_t2_dn10 = assign48560_e81531_d_n10;
        locals.var_t2_dn11 = assign48560_e81531_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign48570_e81541, assign48570_e81541_d_n3, assign48570_e81541_d_n4, assign48570_e81541_d_n5, assign48570_e81541_d_n6, assign48570_e81541_d_n7, assign48570_e81541_d_n8, assign48570_e81541_d_n9, assign48570_e81541_d_n10, assign48570_e81541_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48570_e81539: f64 = (locals.var_isrec_i * locals.var_t2);
        (assign48570_e81539, (locals.var_isrec_i * locals.var_t2_dn3), (locals.var_isrec_i * locals.var_t2_dn4), (locals.var_isrec_i * locals.var_t2_dn5), (locals.var_isrec_i * locals.var_t2_dn6), (locals.var_isrec_i * locals.var_t2_dn7), (locals.var_isrec_i * locals.var_t2_dn8), (locals.var_isrec_i * locals.var_t2_dn9), (locals.var_isrec_i * locals.var_t2_dn10), (locals.var_isrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11,)
    }
};
        locals.var_jrecs = assign48570_e81541;
        locals.var_jrecs_dn3 = assign48570_e81541_d_n3;
        locals.var_jrecs_dn4 = assign48570_e81541_d_n4;
        locals.var_jrecs_dn5 = assign48570_e81541_d_n5;
        locals.var_jrecs_dn6 = assign48570_e81541_d_n6;
        locals.var_jrecs_dn7 = assign48570_e81541_d_n7;
        locals.var_jrecs_dn8 = assign48570_e81541_d_n8;
        locals.var_jrecs_dn9 = assign48570_e81541_d_n9;
        locals.var_jrecs_dn10 = assign48570_e81541_d_n10;
        locals.var_jrecs_dn11 = assign48570_e81541_d_n11;
        locals.var_jrecs_rv = 0.0;

        let (assign48580_e81559, assign48580_e81559_d_n4, assign48580_e81559_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48580_e81549: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign48580_e81554: f64 = (locals.var_tratio - 1.0);
        let assign48580_e81555: f64 = (locals.var_ntrecf_i * assign48580_e81554);
        let assign48580_e81556: f64 = (1.0 + assign48580_e81555);
        let assign48580_e81557: f64 = (assign48580_e81549 * assign48580_e81556);
        (assign48580_e81557, (assign48580_e81549 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign48580_e81549 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign48580_e81559;
        locals.var_nvtmf_dn4 = assign48580_e81559_d_n4;
        locals.var_nvtmf_dn5 = assign48580_e81559_d_n5;
        locals.var_nvtmf_rv = 0.0;

        let (assign48590_e81577, assign48590_e81577_d_n4, assign48590_e81577_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48590_e81567: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign48590_e81572: f64 = (locals.var_tratio - 1.0);
        let assign48590_e81573: f64 = (locals.var_ntrecr_i * assign48590_e81572);
        let assign48590_e81574: f64 = (1.0 + assign48590_e81573);
        let assign48590_e81575: f64 = (assign48590_e81567 * assign48590_e81574);
        (assign48590_e81575, (assign48590_e81567 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign48590_e81567 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign48590_e81577;
        locals.var_nvtmr_dn4 = assign48590_e81577_d_n4;
        locals.var_nvtmr_dn5 = assign48590_e81577_d_n5;
        locals.var_nvtmr_rv = 0.0;

        let (assign48600_e81587, assign48600_e81587_d_n3, assign48600_e81587_d_n4, assign48600_e81587_d_n5, assign48600_e81587_d_n6, assign48600_e81587_d_n7, assign48600_e81587_d_n8, assign48600_e81587_d_n9, assign48600_e81587_d_n10, assign48600_e81587_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48600_e81585: f64 = (locals.var_vbs_jct / locals.var_nvtmf);
        (assign48600_e81585, 0.0, (-((locals.var_vbs_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbs_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtmf), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48600_e81587;
        locals.var_t0_dn3 = assign48600_e81587_d_n3;
        locals.var_t0_dn4 = assign48600_e81587_d_n4;
        locals.var_t0_dn5 = assign48600_e81587_d_n5;
        locals.var_t0_dn6 = assign48600_e81587_d_n6;
        locals.var_t0_dn7 = assign48600_e81587_d_n7;
        locals.var_t0_dn8 = assign48600_e81587_d_n8;
        locals.var_t0_dn9 = assign48600_e81587_d_n9;
        locals.var_t0_dn10 = assign48600_e81587_d_n10;
        locals.var_t0_dn11 = assign48600_e81587_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48610_e81596, assign48610_e81596_d_n3, assign48610_e81596_d_n4, assign48610_e81596_d_n5, assign48610_e81596_d_n6, assign48610_e81596_d_n7, assign48610_e81596_d_n8, assign48610_e81596_d_n9, assign48610_e81596_d_n10, assign48610_e81596_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48610_e81594: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48610_e81594, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign48610_e81596;
        locals.var_t10_dn3 = assign48610_e81596_d_n3;
        locals.var_t10_dn4 = assign48610_e81596_d_n4;
        locals.var_t10_dn5 = assign48610_e81596_d_n5;
        locals.var_t10_dn6 = assign48610_e81596_d_n6;
        locals.var_t10_dn7 = assign48610_e81596_d_n7;
        locals.var_t10_dn8 = assign48610_e81596_d_n8;
        locals.var_t10_dn9 = assign48610_e81596_d_n9;
        locals.var_t10_dn10 = assign48610_e81596_d_n10;
        locals.var_t10_dn11 = assign48610_e81596_d_n11;
        locals.var_t10_rv = 0.0;

        let assign48620_e81599: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign48620_e81601: f64 = if assign48620_e81599 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign48620_e81601;
        locals.var_guard747_rv = 0.0;

        let (assign48630_e81611, assign48630_e81611_d_n3, assign48630_e81611_d_n4, assign48630_e81611_d_n5, assign48630_e81611_d_n6, assign48630_e81611_d_n7, assign48630_e81611_d_n8, assign48630_e81611_d_n9, assign48630_e81611_d_n10, assign48630_e81611_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48630_e81611;
        locals.var_t1_dn3 = assign48630_e81611_d_n3;
        locals.var_t1_dn4 = assign48630_e81611_d_n4;
        locals.var_t1_dn5 = assign48630_e81611_d_n5;
        locals.var_t1_dn6 = assign48630_e81611_d_n6;
        locals.var_t1_dn7 = assign48630_e81611_d_n7;
        locals.var_t1_dn8 = assign48630_e81611_d_n8;
        locals.var_t1_dn9 = assign48630_e81611_d_n9;
        locals.var_t1_dn10 = assign48630_e81611_d_n10;
        locals.var_t1_dn11 = assign48630_e81611_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48640_e81628, assign48640_e81628_d_n3, assign48640_e81628_d_n4, assign48640_e81628_d_n5, assign48640_e81628_d_n6, assign48640_e81628_d_n7, assign48640_e81628_d_n8, assign48640_e81628_d_n9, assign48640_e81628_d_n10, assign48640_e81628_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48640_e81620: f64 = (-locals.var_vbs_jct);
        let assign48640_e81622: f64 = (assign48640_e81620 / locals.var_nvtmr);
        let assign48640_e81624: f64 = (assign48640_e81622 * locals.var_vrec0_i);
        let assign48640_e81626: f64 = (assign48640_e81624 * locals.var_t1);
        (assign48640_e81626, (assign48640_e81624 * locals.var_t1_dn3), ((((-((assign48640_e81620 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn4)), ((((-((assign48640_e81620 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn5)), (assign48640_e81624 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn7)), (assign48640_e81624 * locals.var_t1_dn8), (assign48640_e81624 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn10)), (assign48640_e81624 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48640_e81628;
        locals.var_t0_dn3 = assign48640_e81628_d_n3;
        locals.var_t0_dn4 = assign48640_e81628_d_n4;
        locals.var_t0_dn5 = assign48640_e81628_d_n5;
        locals.var_t0_dn6 = assign48640_e81628_d_n6;
        locals.var_t0_dn7 = assign48640_e81628_d_n7;
        locals.var_t0_dn8 = assign48640_e81628_d_n8;
        locals.var_t0_dn9 = assign48640_e81628_d_n9;
        locals.var_t0_dn10 = assign48640_e81628_d_n10;
        locals.var_t0_dn11 = assign48640_e81628_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48650_e81639, assign48650_e81639_d_n3, assign48650_e81639_d_n4, assign48650_e81639_d_n5, assign48650_e81639_d_n6, assign48650_e81639_d_n7, assign48650_e81639_d_n8, assign48650_e81639_d_n9, assign48650_e81639_d_n10, assign48650_e81639_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48650_e81637: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48650_e81637, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48650_e81639;
        locals.var_t11_dn3 = assign48650_e81639_d_n3;
        locals.var_t11_dn4 = assign48650_e81639_d_n4;
        locals.var_t11_dn5 = assign48650_e81639_d_n5;
        locals.var_t11_dn6 = assign48650_e81639_d_n6;
        locals.var_t11_dn7 = assign48650_e81639_d_n7;
        locals.var_t11_dn8 = assign48650_e81639_d_n8;
        locals.var_t11_dn9 = assign48650_e81639_d_n9;
        locals.var_t11_dn10 = assign48650_e81639_d_n10;
        locals.var_t11_dn11 = assign48650_e81639_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48660_e81650, assign48660_e81650_d_n3, assign48660_e81650_d_n4, assign48660_e81650_d_n5, assign48660_e81650_d_n6, assign48660_e81650_d_n7, assign48660_e81650_d_n8, assign48660_e81650_d_n9, assign48660_e81650_d_n10, assign48660_e81650_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48660_e81648: f64 = (-locals.var_t11);
        (assign48660_e81648, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48660_e81650;
        locals.var_t11_dn3 = assign48660_e81650_d_n3;
        locals.var_t11_dn4 = assign48660_e81650_d_n4;
        locals.var_t11_dn5 = assign48660_e81650_d_n5;
        locals.var_t11_dn6 = assign48660_e81650_d_n6;
        locals.var_t11_dn7 = assign48660_e81650_d_n7;
        locals.var_t11_dn8 = assign48660_e81650_d_n8;
        locals.var_t11_dn9 = assign48660_e81650_d_n9;
        locals.var_t11_dn10 = assign48660_e81650_d_n10;
        locals.var_t11_dn11 = assign48660_e81650_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48670_e81665, assign48670_e81665_d_n3, assign48670_e81665_d_n4, assign48670_e81665_d_n5, assign48670_e81665_d_n6, assign48670_e81665_d_n7, assign48670_e81665_d_n8, assign48670_e81665_d_n9, assign48670_e81665_d_n10, assign48670_e81665_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48670_e81662: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign48670_e81663: f64 = (1.0 / assign48670_e81662);
        (assign48670_e81663, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign48670_e81662 * assign48670_e81662))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign48670_e81662 * assign48670_e81662))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48670_e81665;
        locals.var_t1_dn3 = assign48670_e81665_d_n3;
        locals.var_t1_dn4 = assign48670_e81665_d_n4;
        locals.var_t1_dn5 = assign48670_e81665_d_n5;
        locals.var_t1_dn6 = assign48670_e81665_d_n6;
        locals.var_t1_dn7 = assign48670_e81665_d_n7;
        locals.var_t1_dn8 = assign48670_e81665_d_n8;
        locals.var_t1_dn9 = assign48670_e81665_d_n9;
        locals.var_t1_dn10 = assign48670_e81665_d_n10;
        locals.var_t1_dn11 = assign48670_e81665_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48680_e81683, assign48680_e81683_d_n3, assign48680_e81683_d_n4, assign48680_e81683_d_n5, assign48680_e81683_d_n6, assign48680_e81683_d_n7, assign48680_e81683_d_n8, assign48680_e81683_d_n9, assign48680_e81683_d_n10, assign48680_e81683_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48680_e81675: f64 = (-locals.var_vbs_jct);
        let assign48680_e81677: f64 = (assign48680_e81675 / locals.var_nvtmr);
        let assign48680_e81679: f64 = (assign48680_e81677 * locals.var_vrec0_i);
        let assign48680_e81681: f64 = (assign48680_e81679 * locals.var_t1);
        (assign48680_e81681, (assign48680_e81679 * locals.var_t1_dn3), ((((-((assign48680_e81675 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn4)), ((((-((assign48680_e81675 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn5)), (assign48680_e81679 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn7)), (assign48680_e81679 * locals.var_t1_dn8), (assign48680_e81679 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn10)), (assign48680_e81679 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48680_e81683;
        locals.var_t0_dn3 = assign48680_e81683_d_n3;
        locals.var_t0_dn4 = assign48680_e81683_d_n4;
        locals.var_t0_dn5 = assign48680_e81683_d_n5;
        locals.var_t0_dn6 = assign48680_e81683_d_n6;
        locals.var_t0_dn7 = assign48680_e81683_d_n7;
        locals.var_t0_dn8 = assign48680_e81683_d_n8;
        locals.var_t0_dn9 = assign48680_e81683_d_n9;
        locals.var_t0_dn10 = assign48680_e81683_d_n10;
        locals.var_t0_dn11 = assign48680_e81683_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48690_e81695, assign48690_e81695_d_n3, assign48690_e81695_d_n4, assign48690_e81695_d_n5, assign48690_e81695_d_n6, assign48690_e81695_d_n7, assign48690_e81695_d_n8, assign48690_e81695_d_n9, assign48690_e81695_d_n10, assign48690_e81695_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48690_e81693: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48690_e81693, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48690_e81695;
        locals.var_t11_dn3 = assign48690_e81695_d_n3;
        locals.var_t11_dn4 = assign48690_e81695_d_n4;
        locals.var_t11_dn5 = assign48690_e81695_d_n5;
        locals.var_t11_dn6 = assign48690_e81695_d_n6;
        locals.var_t11_dn7 = assign48690_e81695_d_n7;
        locals.var_t11_dn8 = assign48690_e81695_d_n8;
        locals.var_t11_dn9 = assign48690_e81695_d_n9;
        locals.var_t11_dn10 = assign48690_e81695_d_n10;
        locals.var_t11_dn11 = assign48690_e81695_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48700_e81707, assign48700_e81707_d_n3, assign48700_e81707_d_n4, assign48700_e81707_d_n5, assign48700_e81707_d_n6, assign48700_e81707_d_n7, assign48700_e81707_d_n8, assign48700_e81707_d_n9, assign48700_e81707_d_n10, assign48700_e81707_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48700_e81705: f64 = (-locals.var_t11);
        (assign48700_e81705, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48700_e81707;
        locals.var_t11_dn3 = assign48700_e81707_d_n3;
        locals.var_t11_dn4 = assign48700_e81707_d_n4;
        locals.var_t11_dn5 = assign48700_e81707_d_n5;
        locals.var_t11_dn6 = assign48700_e81707_d_n6;
        locals.var_t11_dn7 = assign48700_e81707_d_n7;
        locals.var_t11_dn8 = assign48700_e81707_d_n8;
        locals.var_t11_dn9 = assign48700_e81707_d_n9;
        locals.var_t11_dn10 = assign48700_e81707_d_n10;
        locals.var_t11_dn11 = assign48700_e81707_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48710_e81717, assign48710_e81717_d_n3, assign48710_e81717_d_n4, assign48710_e81717_d_n5, assign48710_e81717_d_n6, assign48710_e81717_d_n7, assign48710_e81717_d_n8, assign48710_e81717_d_n9, assign48710_e81717_d_n10, assign48710_e81717_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48710_e81715: f64 = (locals.var_wstsi * locals.var_jrecs);
        (assign48710_e81715, (locals.var_wstsi * locals.var_jrecs_dn3), (locals.var_wstsi * locals.var_jrecs_dn4), (locals.var_wstsi * locals.var_jrecs_dn5), (locals.var_wstsi * locals.var_jrecs_dn6), (locals.var_wstsi * locals.var_jrecs_dn7), (locals.var_wstsi * locals.var_jrecs_dn8), (locals.var_wstsi * locals.var_jrecs_dn9), (locals.var_wstsi * locals.var_jrecs_dn10), (locals.var_wstsi * locals.var_jrecs_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48710_e81717;
        locals.var_t3_dn3 = assign48710_e81717_d_n3;
        locals.var_t3_dn4 = assign48710_e81717_d_n4;
        locals.var_t3_dn5 = assign48710_e81717_d_n5;
        locals.var_t3_dn6 = assign48710_e81717_d_n6;
        locals.var_t3_dn7 = assign48710_e81717_d_n7;
        locals.var_t3_dn8 = assign48710_e81717_d_n8;
        locals.var_t3_dn9 = assign48710_e81717_d_n9;
        locals.var_t3_dn10 = assign48710_e81717_d_n10;
        locals.var_t3_dn11 = assign48710_e81717_d_n11;
        locals.var_t3_rv = 0.0;

        let assign48730_e81732: f64 = if locals.var_idrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign48730_e81732;
        locals.var_guard748_rv = 0.0;

        let (assign48750_e81751, assign48750_e81751_d_n3, assign48750_e81751_d_n4, assign48750_e81751_d_n5, assign48750_e81751_d_n6, assign48750_e81751_d_n7, assign48750_e81751_d_n8, assign48750_e81751_d_n9, assign48750_e81751_d_n10, assign48750_e81751_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48750_e81747: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign48750_e81749: f64 = (assign48750_e81747 / locals.var_nrecf0_i);
        (assign48750_e81749, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48750_e81751;
        locals.var_t7_dn3 = assign48750_e81751_d_n3;
        locals.var_t7_dn4 = assign48750_e81751_d_n4;
        locals.var_t7_dn5 = assign48750_e81751_d_n5;
        locals.var_t7_dn6 = assign48750_e81751_d_n6;
        locals.var_t7_dn7 = assign48750_e81751_d_n7;
        locals.var_t7_dn8 = assign48750_e81751_d_n8;
        locals.var_t7_dn9 = assign48750_e81751_d_n9;
        locals.var_t7_dn10 = assign48750_e81751_d_n10;
        locals.var_t7_dn11 = assign48750_e81751_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48760_e81760, assign48760_e81760_d_n3, assign48760_e81760_d_n4, assign48760_e81760_d_n5, assign48760_e81760_d_n6, assign48760_e81760_d_n7, assign48760_e81760_d_n8, assign48760_e81760_d_n9, assign48760_e81760_d_n10, assign48760_e81760_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48760_e81758: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48760_e81758, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48760_e81760;
        locals.var_t2_dn3 = assign48760_e81760_d_n3;
        locals.var_t2_dn4 = assign48760_e81760_d_n4;
        locals.var_t2_dn5 = assign48760_e81760_d_n5;
        locals.var_t2_dn6 = assign48760_e81760_d_n6;
        locals.var_t2_dn7 = assign48760_e81760_d_n7;
        locals.var_t2_dn8 = assign48760_e81760_d_n8;
        locals.var_t2_dn9 = assign48760_e81760_d_n9;
        locals.var_t2_dn10 = assign48760_e81760_d_n10;
        locals.var_t2_dn11 = assign48760_e81760_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_170(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48770_e81770, assign48770_e81770_d_n3, assign48770_e81770_d_n4, assign48770_e81770_d_n5, assign48770_e81770_d_n6, assign48770_e81770_d_n7, assign48770_e81770_d_n8, assign48770_e81770_d_n9, assign48770_e81770_d_n10, assign48770_e81770_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48770_e81768: f64 = (locals.var_idrec_i * locals.var_t2);
        (assign48770_e81768, (locals.var_idrec_i * locals.var_t2_dn3), (locals.var_idrec_i * locals.var_t2_dn4), (locals.var_idrec_i * locals.var_t2_dn5), (locals.var_idrec_i * locals.var_t2_dn6), (locals.var_idrec_i * locals.var_t2_dn7), (locals.var_idrec_i * locals.var_t2_dn8), (locals.var_idrec_i * locals.var_t2_dn9), (locals.var_idrec_i * locals.var_t2_dn10), (locals.var_idrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11,)
    }
};
        locals.var_jrecd = assign48770_e81770;
        locals.var_jrecd_dn3 = assign48770_e81770_d_n3;
        locals.var_jrecd_dn4 = assign48770_e81770_d_n4;
        locals.var_jrecd_dn5 = assign48770_e81770_d_n5;
        locals.var_jrecd_dn6 = assign48770_e81770_d_n6;
        locals.var_jrecd_dn7 = assign48770_e81770_d_n7;
        locals.var_jrecd_dn8 = assign48770_e81770_d_n8;
        locals.var_jrecd_dn9 = assign48770_e81770_d_n9;
        locals.var_jrecd_dn10 = assign48770_e81770_d_n10;
        locals.var_jrecd_dn11 = assign48770_e81770_d_n11;
        locals.var_jrecd_rv = 0.0;

        let (assign48780_e81788, assign48780_e81788_d_n4, assign48780_e81788_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48780_e81778: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign48780_e81783: f64 = (locals.var_tratio - 1.0);
        let assign48780_e81784: f64 = (locals.var_ntrecf_i * assign48780_e81783);
        let assign48780_e81785: f64 = (1.0 + assign48780_e81784);
        let assign48780_e81786: f64 = (assign48780_e81778 * assign48780_e81785);
        (assign48780_e81786, (assign48780_e81778 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign48780_e81778 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign48780_e81788;
        locals.var_nvtmf_dn4 = assign48780_e81788_d_n4;
        locals.var_nvtmf_dn5 = assign48780_e81788_d_n5;
        locals.var_nvtmf_rv = 0.0;

        let (assign48790_e81806, assign48790_e81806_d_n4, assign48790_e81806_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48790_e81796: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign48790_e81801: f64 = (locals.var_tratio - 1.0);
        let assign48790_e81802: f64 = (locals.var_ntrecr_i * assign48790_e81801);
        let assign48790_e81803: f64 = (1.0 + assign48790_e81802);
        let assign48790_e81804: f64 = (assign48790_e81796 * assign48790_e81803);
        (assign48790_e81804, (assign48790_e81796 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign48790_e81796 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign48790_e81806;
        locals.var_nvtmr_dn4 = assign48790_e81806_d_n4;
        locals.var_nvtmr_dn5 = assign48790_e81806_d_n5;
        locals.var_nvtmr_rv = 0.0;

        let (assign48800_e81816, assign48800_e81816_d_n3, assign48800_e81816_d_n4, assign48800_e81816_d_n5, assign48800_e81816_d_n6, assign48800_e81816_d_n7, assign48800_e81816_d_n8, assign48800_e81816_d_n9, assign48800_e81816_d_n10, assign48800_e81816_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48800_e81814: f64 = (locals.var_vbd_jct / locals.var_nvtmf);
        (assign48800_e81814, 0.0, (-((locals.var_vbd_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbd_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), (locals.var_vbd_jct_dn6 / locals.var_nvtmf), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48800_e81816;
        locals.var_t0_dn3 = assign48800_e81816_d_n3;
        locals.var_t0_dn4 = assign48800_e81816_d_n4;
        locals.var_t0_dn5 = assign48800_e81816_d_n5;
        locals.var_t0_dn6 = assign48800_e81816_d_n6;
        locals.var_t0_dn7 = assign48800_e81816_d_n7;
        locals.var_t0_dn8 = assign48800_e81816_d_n8;
        locals.var_t0_dn9 = assign48800_e81816_d_n9;
        locals.var_t0_dn10 = assign48800_e81816_d_n10;
        locals.var_t0_dn11 = assign48800_e81816_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48810_e81825, assign48810_e81825_d_n3, assign48810_e81825_d_n4, assign48810_e81825_d_n5, assign48810_e81825_d_n6, assign48810_e81825_d_n7, assign48810_e81825_d_n8, assign48810_e81825_d_n9, assign48810_e81825_d_n10, assign48810_e81825_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48810_e81823: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48810_e81823, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign48810_e81825;
        locals.var_t10_dn3 = assign48810_e81825_d_n3;
        locals.var_t10_dn4 = assign48810_e81825_d_n4;
        locals.var_t10_dn5 = assign48810_e81825_d_n5;
        locals.var_t10_dn6 = assign48810_e81825_d_n6;
        locals.var_t10_dn7 = assign48810_e81825_d_n7;
        locals.var_t10_dn8 = assign48810_e81825_d_n8;
        locals.var_t10_dn9 = assign48810_e81825_d_n9;
        locals.var_t10_dn10 = assign48810_e81825_d_n10;
        locals.var_t10_dn11 = assign48810_e81825_d_n11;
        locals.var_t10_rv = 0.0;

        let assign48820_e81828: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign48820_e81830: f64 = if assign48820_e81828 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign48820_e81830;
        locals.var_guard749_rv = 0.0;

        let (assign48830_e81840, assign48830_e81840_d_n3, assign48830_e81840_d_n4, assign48830_e81840_d_n5, assign48830_e81840_d_n6, assign48830_e81840_d_n7, assign48830_e81840_d_n8, assign48830_e81840_d_n9, assign48830_e81840_d_n10, assign48830_e81840_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48830_e81840;
        locals.var_t1_dn3 = assign48830_e81840_d_n3;
        locals.var_t1_dn4 = assign48830_e81840_d_n4;
        locals.var_t1_dn5 = assign48830_e81840_d_n5;
        locals.var_t1_dn6 = assign48830_e81840_d_n6;
        locals.var_t1_dn7 = assign48830_e81840_d_n7;
        locals.var_t1_dn8 = assign48830_e81840_d_n8;
        locals.var_t1_dn9 = assign48830_e81840_d_n9;
        locals.var_t1_dn10 = assign48830_e81840_d_n10;
        locals.var_t1_dn11 = assign48830_e81840_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48840_e81857, assign48840_e81857_d_n3, assign48840_e81857_d_n4, assign48840_e81857_d_n5, assign48840_e81857_d_n6, assign48840_e81857_d_n7, assign48840_e81857_d_n8, assign48840_e81857_d_n9, assign48840_e81857_d_n10, assign48840_e81857_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48840_e81849: f64 = (-locals.var_vbd_jct);
        let assign48840_e81851: f64 = (assign48840_e81849 / locals.var_nvtmr);
        let assign48840_e81853: f64 = (assign48840_e81851 * locals.var_vrec0d_i);
        let assign48840_e81855: f64 = (assign48840_e81853 * locals.var_t1);
        (assign48840_e81855, (assign48840_e81853 * locals.var_t1_dn3), ((((-((assign48840_e81849 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn4)), ((((-((assign48840_e81849 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn6)), (assign48840_e81853 * locals.var_t1_dn7), (assign48840_e81853 * locals.var_t1_dn8), (assign48840_e81853 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn10)), (assign48840_e81853 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48840_e81857;
        locals.var_t0_dn3 = assign48840_e81857_d_n3;
        locals.var_t0_dn4 = assign48840_e81857_d_n4;
        locals.var_t0_dn5 = assign48840_e81857_d_n5;
        locals.var_t0_dn6 = assign48840_e81857_d_n6;
        locals.var_t0_dn7 = assign48840_e81857_d_n7;
        locals.var_t0_dn8 = assign48840_e81857_d_n8;
        locals.var_t0_dn9 = assign48840_e81857_d_n9;
        locals.var_t0_dn10 = assign48840_e81857_d_n10;
        locals.var_t0_dn11 = assign48840_e81857_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48850_e81868, assign48850_e81868_d_n3, assign48850_e81868_d_n4, assign48850_e81868_d_n5, assign48850_e81868_d_n6, assign48850_e81868_d_n7, assign48850_e81868_d_n8, assign48850_e81868_d_n9, assign48850_e81868_d_n10, assign48850_e81868_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48850_e81866: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48850_e81866, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48850_e81868;
        locals.var_t11_dn3 = assign48850_e81868_d_n3;
        locals.var_t11_dn4 = assign48850_e81868_d_n4;
        locals.var_t11_dn5 = assign48850_e81868_d_n5;
        locals.var_t11_dn6 = assign48850_e81868_d_n6;
        locals.var_t11_dn7 = assign48850_e81868_d_n7;
        locals.var_t11_dn8 = assign48850_e81868_d_n8;
        locals.var_t11_dn9 = assign48850_e81868_d_n9;
        locals.var_t11_dn10 = assign48850_e81868_d_n10;
        locals.var_t11_dn11 = assign48850_e81868_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48860_e81879, assign48860_e81879_d_n3, assign48860_e81879_d_n4, assign48860_e81879_d_n5, assign48860_e81879_d_n6, assign48860_e81879_d_n7, assign48860_e81879_d_n8, assign48860_e81879_d_n9, assign48860_e81879_d_n10, assign48860_e81879_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48860_e81877: f64 = (-locals.var_t11);
        (assign48860_e81877, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48860_e81879;
        locals.var_t11_dn3 = assign48860_e81879_d_n3;
        locals.var_t11_dn4 = assign48860_e81879_d_n4;
        locals.var_t11_dn5 = assign48860_e81879_d_n5;
        locals.var_t11_dn6 = assign48860_e81879_d_n6;
        locals.var_t11_dn7 = assign48860_e81879_d_n7;
        locals.var_t11_dn8 = assign48860_e81879_d_n8;
        locals.var_t11_dn9 = assign48860_e81879_d_n9;
        locals.var_t11_dn10 = assign48860_e81879_d_n10;
        locals.var_t11_dn11 = assign48860_e81879_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48870_e81894, assign48870_e81894_d_n3, assign48870_e81894_d_n4, assign48870_e81894_d_n5, assign48870_e81894_d_n6, assign48870_e81894_d_n7, assign48870_e81894_d_n8, assign48870_e81894_d_n9, assign48870_e81894_d_n10, assign48870_e81894_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48870_e81891: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign48870_e81892: f64 = (1.0 / assign48870_e81891);
        (assign48870_e81892, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign48870_e81891 * assign48870_e81891))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign48870_e81891 * assign48870_e81891))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48870_e81894;
        locals.var_t1_dn3 = assign48870_e81894_d_n3;
        locals.var_t1_dn4 = assign48870_e81894_d_n4;
        locals.var_t1_dn5 = assign48870_e81894_d_n5;
        locals.var_t1_dn6 = assign48870_e81894_d_n6;
        locals.var_t1_dn7 = assign48870_e81894_d_n7;
        locals.var_t1_dn8 = assign48870_e81894_d_n8;
        locals.var_t1_dn9 = assign48870_e81894_d_n9;
        locals.var_t1_dn10 = assign48870_e81894_d_n10;
        locals.var_t1_dn11 = assign48870_e81894_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48880_e81912, assign48880_e81912_d_n3, assign48880_e81912_d_n4, assign48880_e81912_d_n5, assign48880_e81912_d_n6, assign48880_e81912_d_n7, assign48880_e81912_d_n8, assign48880_e81912_d_n9, assign48880_e81912_d_n10, assign48880_e81912_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48880_e81904: f64 = (-locals.var_vbd_jct);
        let assign48880_e81906: f64 = (assign48880_e81904 / locals.var_nvtmr);
        let assign48880_e81908: f64 = (assign48880_e81906 * locals.var_vrec0d_i);
        let assign48880_e81910: f64 = (assign48880_e81908 * locals.var_t1);
        (assign48880_e81910, (assign48880_e81908 * locals.var_t1_dn3), ((((-((assign48880_e81904 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn4)), ((((-((assign48880_e81904 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn6)), (assign48880_e81908 * locals.var_t1_dn7), (assign48880_e81908 * locals.var_t1_dn8), (assign48880_e81908 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn10)), (assign48880_e81908 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48880_e81912;
        locals.var_t0_dn3 = assign48880_e81912_d_n3;
        locals.var_t0_dn4 = assign48880_e81912_d_n4;
        locals.var_t0_dn5 = assign48880_e81912_d_n5;
        locals.var_t0_dn6 = assign48880_e81912_d_n6;
        locals.var_t0_dn7 = assign48880_e81912_d_n7;
        locals.var_t0_dn8 = assign48880_e81912_d_n8;
        locals.var_t0_dn9 = assign48880_e81912_d_n9;
        locals.var_t0_dn10 = assign48880_e81912_d_n10;
        locals.var_t0_dn11 = assign48880_e81912_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48890_e81924, assign48890_e81924_d_n3, assign48890_e81924_d_n4, assign48890_e81924_d_n5, assign48890_e81924_d_n6, assign48890_e81924_d_n7, assign48890_e81924_d_n8, assign48890_e81924_d_n9, assign48890_e81924_d_n10, assign48890_e81924_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48890_e81922: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48890_e81922, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48890_e81924;
        locals.var_t11_dn3 = assign48890_e81924_d_n3;
        locals.var_t11_dn4 = assign48890_e81924_d_n4;
        locals.var_t11_dn5 = assign48890_e81924_d_n5;
        locals.var_t11_dn6 = assign48890_e81924_d_n6;
        locals.var_t11_dn7 = assign48890_e81924_d_n7;
        locals.var_t11_dn8 = assign48890_e81924_d_n8;
        locals.var_t11_dn9 = assign48890_e81924_d_n9;
        locals.var_t11_dn10 = assign48890_e81924_d_n10;
        locals.var_t11_dn11 = assign48890_e81924_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48900_e81936, assign48900_e81936_d_n3, assign48900_e81936_d_n4, assign48900_e81936_d_n5, assign48900_e81936_d_n6, assign48900_e81936_d_n7, assign48900_e81936_d_n8, assign48900_e81936_d_n9, assign48900_e81936_d_n10, assign48900_e81936_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48900_e81934: f64 = (-locals.var_t11);
        (assign48900_e81934, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48900_e81936;
        locals.var_t11_dn3 = assign48900_e81936_d_n3;
        locals.var_t11_dn4 = assign48900_e81936_d_n4;
        locals.var_t11_dn5 = assign48900_e81936_d_n5;
        locals.var_t11_dn6 = assign48900_e81936_d_n6;
        locals.var_t11_dn7 = assign48900_e81936_d_n7;
        locals.var_t11_dn8 = assign48900_e81936_d_n8;
        locals.var_t11_dn9 = assign48900_e81936_d_n9;
        locals.var_t11_dn10 = assign48900_e81936_d_n10;
        locals.var_t11_dn11 = assign48900_e81936_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48910_e81946, assign48910_e81946_d_n3, assign48910_e81946_d_n4, assign48910_e81946_d_n5, assign48910_e81946_d_n6, assign48910_e81946_d_n7, assign48910_e81946_d_n8, assign48910_e81946_d_n9, assign48910_e81946_d_n10, assign48910_e81946_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48910_e81944: f64 = (locals.var_wdtsi * locals.var_jrecd);
        (assign48910_e81944, (locals.var_wdtsi * locals.var_jrecd_dn3), (locals.var_wdtsi * locals.var_jrecd_dn4), (locals.var_wdtsi * locals.var_jrecd_dn5), (locals.var_wdtsi * locals.var_jrecd_dn6), (locals.var_wdtsi * locals.var_jrecd_dn7), (locals.var_wdtsi * locals.var_jrecd_dn8), (locals.var_wdtsi * locals.var_jrecd_dn9), (locals.var_wdtsi * locals.var_jrecd_dn10), (locals.var_wdtsi * locals.var_jrecd_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48910_e81946;
        locals.var_t3_dn3 = assign48910_e81946_d_n3;
        locals.var_t3_dn4 = assign48910_e81946_d_n4;
        locals.var_t3_dn5 = assign48910_e81946_d_n5;
        locals.var_t3_dn6 = assign48910_e81946_d_n6;
        locals.var_t3_dn7 = assign48910_e81946_d_n7;
        locals.var_t3_dn8 = assign48910_e81946_d_n8;
        locals.var_t3_dn9 = assign48910_e81946_d_n9;
        locals.var_t3_dn10 = assign48910_e81946_d_n10;
        locals.var_t3_dn11 = assign48910_e81946_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign48930_e81967,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48930_e81963: f64 = (locals.var_weff / p.p1373);
        let assign48930_e81965: f64 = (assign48930_e81963 * p.p74);
        (assign48930_e81965,)
    } else {
        (locals.var_wtsi,)
    }
};
        locals.var_wtsi = assign48930_e81967;
        locals.var_wtsi_rv = 0.0;

        let assign48940_e81974: f64 = if ((locals.var_isbjt_i == 0.0) && (locals.var_idbjt_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard750 = assign48940_e81974;
        locals.var_guard750_rv = 0.0;

        let (assign48980_e82007, assign48980_e82007_d_n3, assign48980_e82007_d_n4, assign48980_e82007_d_n5, assign48980_e82007_d_n6, assign48980_e82007_d_n7, assign48980_e82007_d_n8, assign48980_e82007_d_n9, assign48980_e82007_d_n10, assign48980_e82007_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign48980_e82003: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign48980_e82005: f64 = (assign48980_e82003 / locals.var_ndiode_i);
        (assign48980_e82005, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48980_e82007;
        locals.var_t7_dn3 = assign48980_e82007_d_n3;
        locals.var_t7_dn4 = assign48980_e82007_d_n4;
        locals.var_t7_dn5 = assign48980_e82007_d_n5;
        locals.var_t7_dn6 = assign48980_e82007_d_n6;
        locals.var_t7_dn7 = assign48980_e82007_d_n7;
        locals.var_t7_dn8 = assign48980_e82007_d_n8;
        locals.var_t7_dn9 = assign48980_e82007_d_n9;
        locals.var_t7_dn10 = assign48980_e82007_d_n10;
        locals.var_t7_dn11 = assign48980_e82007_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48990_e82016, assign48990_e82016_d_n3, assign48990_e82016_d_n4, assign48990_e82016_d_n5, assign48990_e82016_d_n6, assign48990_e82016_d_n7, assign48990_e82016_d_n8, assign48990_e82016_d_n9, assign48990_e82016_d_n10, assign48990_e82016_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign48990_e82014: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48990_e82014, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48990_e82016;
        locals.var_t0_dn3 = assign48990_e82016_d_n3;
        locals.var_t0_dn4 = assign48990_e82016_d_n4;
        locals.var_t0_dn5 = assign48990_e82016_d_n5;
        locals.var_t0_dn6 = assign48990_e82016_d_n6;
        locals.var_t0_dn7 = assign48990_e82016_d_n7;
        locals.var_t0_dn8 = assign48990_e82016_d_n8;
        locals.var_t0_dn9 = assign48990_e82016_d_n9;
        locals.var_t0_dn10 = assign48990_e82016_d_n10;
        locals.var_t0_dn11 = assign48990_e82016_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49000_e82026, assign49000_e82026_d_n3, assign49000_e82026_d_n4, assign49000_e82026_d_n5, assign49000_e82026_d_n6, assign49000_e82026_d_n7, assign49000_e82026_d_n8, assign49000_e82026_d_n9, assign49000_e82026_d_n10, assign49000_e82026_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49000_e82024: f64 = (locals.var_ahli_i * locals.var_t0);
        (assign49000_e82024, (locals.var_ahli_i * locals.var_t0_dn3), (locals.var_ahli_i * locals.var_t0_dn4), (locals.var_ahli_i * locals.var_t0_dn5), (locals.var_ahli_i * locals.var_t0_dn6), (locals.var_ahli_i * locals.var_t0_dn7), (locals.var_ahli_i * locals.var_t0_dn8), (locals.var_ahli_i * locals.var_t0_dn9), (locals.var_ahli_i * locals.var_t0_dn10), (locals.var_ahli_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11,)
    }
};
        locals.var_ahlis = assign49000_e82026;
        locals.var_ahlis_dn3 = assign49000_e82026_d_n3;
        locals.var_ahlis_dn4 = assign49000_e82026_d_n4;
        locals.var_ahlis_dn5 = assign49000_e82026_d_n5;
        locals.var_ahlis_dn6 = assign49000_e82026_d_n6;
        locals.var_ahlis_dn7 = assign49000_e82026_d_n7;
        locals.var_ahlis_dn8 = assign49000_e82026_d_n8;
        locals.var_ahlis_dn9 = assign49000_e82026_d_n9;
        locals.var_ahlis_dn10 = assign49000_e82026_d_n10;
        locals.var_ahlis_dn11 = assign49000_e82026_d_n11;
        locals.var_ahlis_rv = 0.0;

        let (assign49010_e82036, assign49010_e82036_d_n3, assign49010_e82036_d_n4, assign49010_e82036_d_n5, assign49010_e82036_d_n6, assign49010_e82036_d_n7, assign49010_e82036_d_n8, assign49010_e82036_d_n9, assign49010_e82036_d_n10, assign49010_e82036_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49010_e82034: f64 = (locals.var_isbjt_i * locals.var_t0);
        (assign49010_e82034, (locals.var_isbjt_i * locals.var_t0_dn3), (locals.var_isbjt_i * locals.var_t0_dn4), (locals.var_isbjt_i * locals.var_t0_dn5), (locals.var_isbjt_i * locals.var_t0_dn6), (locals.var_isbjt_i * locals.var_t0_dn7), (locals.var_isbjt_i * locals.var_t0_dn8), (locals.var_isbjt_i * locals.var_t0_dn9), (locals.var_isbjt_i * locals.var_t0_dn10), (locals.var_isbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11,)
    }
};
        locals.var_jbjts = assign49010_e82036;
        locals.var_jbjts_dn3 = assign49010_e82036_d_n3;
        locals.var_jbjts_dn4 = assign49010_e82036_d_n4;
        locals.var_jbjts_dn5 = assign49010_e82036_d_n5;
        locals.var_jbjts_dn6 = assign49010_e82036_d_n6;
        locals.var_jbjts_dn7 = assign49010_e82036_d_n7;
        locals.var_jbjts_dn8 = assign49010_e82036_d_n8;
        locals.var_jbjts_dn9 = assign49010_e82036_d_n9;
        locals.var_jbjts_dn10 = assign49010_e82036_d_n10;
        locals.var_jbjts_dn11 = assign49010_e82036_d_n11;
        locals.var_jbjts_rv = 0.0;

        let (assign49020_e82048, assign49020_e82048_d_n3, assign49020_e82048_d_n4, assign49020_e82048_d_n5, assign49020_e82048_d_n6, assign49020_e82048_d_n7, assign49020_e82048_d_n8, assign49020_e82048_d_n9, assign49020_e82048_d_n10, assign49020_e82048_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49020_e82044: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign49020_e82046: f64 = (assign49020_e82044 / locals.var_ndiode_i);
        (assign49020_e82046, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49020_e82048;
        locals.var_t7_dn3 = assign49020_e82048_d_n3;
        locals.var_t7_dn4 = assign49020_e82048_d_n4;
        locals.var_t7_dn5 = assign49020_e82048_d_n5;
        locals.var_t7_dn6 = assign49020_e82048_d_n6;
        locals.var_t7_dn7 = assign49020_e82048_d_n7;
        locals.var_t7_dn8 = assign49020_e82048_d_n8;
        locals.var_t7_dn9 = assign49020_e82048_d_n9;
        locals.var_t7_dn10 = assign49020_e82048_d_n10;
        locals.var_t7_dn11 = assign49020_e82048_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign49030_e82057, assign49030_e82057_d_n3, assign49030_e82057_d_n4, assign49030_e82057_d_n5, assign49030_e82057_d_n6, assign49030_e82057_d_n7, assign49030_e82057_d_n8, assign49030_e82057_d_n9, assign49030_e82057_d_n10, assign49030_e82057_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49030_e82055: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49030_e82055, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49030_e82057;
        locals.var_t0_dn3 = assign49030_e82057_d_n3;
        locals.var_t0_dn4 = assign49030_e82057_d_n4;
        locals.var_t0_dn5 = assign49030_e82057_d_n5;
        locals.var_t0_dn6 = assign49030_e82057_d_n6;
        locals.var_t0_dn7 = assign49030_e82057_d_n7;
        locals.var_t0_dn8 = assign49030_e82057_d_n8;
        locals.var_t0_dn9 = assign49030_e82057_d_n9;
        locals.var_t0_dn10 = assign49030_e82057_d_n10;
        locals.var_t0_dn11 = assign49030_e82057_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49040_e82067, assign49040_e82067_d_n3, assign49040_e82067_d_n4, assign49040_e82067_d_n5, assign49040_e82067_d_n6, assign49040_e82067_d_n7, assign49040_e82067_d_n8, assign49040_e82067_d_n9, assign49040_e82067_d_n10, assign49040_e82067_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49040_e82065: f64 = (locals.var_ahlid_i * locals.var_t0);
        (assign49040_e82065, (locals.var_ahlid_i * locals.var_t0_dn3), (locals.var_ahlid_i * locals.var_t0_dn4), (locals.var_ahlid_i * locals.var_t0_dn5), (locals.var_ahlid_i * locals.var_t0_dn6), (locals.var_ahlid_i * locals.var_t0_dn7), (locals.var_ahlid_i * locals.var_t0_dn8), (locals.var_ahlid_i * locals.var_t0_dn9), (locals.var_ahlid_i * locals.var_t0_dn10), (locals.var_ahlid_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11,)
    }
};
        locals.var_ahlid = assign49040_e82067;
        locals.var_ahlid_dn3 = assign49040_e82067_d_n3;
        locals.var_ahlid_dn4 = assign49040_e82067_d_n4;
        locals.var_ahlid_dn5 = assign49040_e82067_d_n5;
        locals.var_ahlid_dn6 = assign49040_e82067_d_n6;
        locals.var_ahlid_dn7 = assign49040_e82067_d_n7;
        locals.var_ahlid_dn8 = assign49040_e82067_d_n8;
        locals.var_ahlid_dn9 = assign49040_e82067_d_n9;
        locals.var_ahlid_dn10 = assign49040_e82067_d_n10;
        locals.var_ahlid_dn11 = assign49040_e82067_d_n11;
        locals.var_ahlid_rv = 0.0;

        let (assign49050_e82077, assign49050_e82077_d_n3, assign49050_e82077_d_n4, assign49050_e82077_d_n5, assign49050_e82077_d_n6, assign49050_e82077_d_n7, assign49050_e82077_d_n8, assign49050_e82077_d_n9, assign49050_e82077_d_n10, assign49050_e82077_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49050_e82075: f64 = (locals.var_idbjt_i * locals.var_t0);
        (assign49050_e82075, (locals.var_idbjt_i * locals.var_t0_dn3), (locals.var_idbjt_i * locals.var_t0_dn4), (locals.var_idbjt_i * locals.var_t0_dn5), (locals.var_idbjt_i * locals.var_t0_dn6), (locals.var_idbjt_i * locals.var_t0_dn7), (locals.var_idbjt_i * locals.var_t0_dn8), (locals.var_idbjt_i * locals.var_t0_dn9), (locals.var_idbjt_i * locals.var_t0_dn10), (locals.var_idbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11,)
    }
};
        locals.var_jbjtd = assign49050_e82077;
        locals.var_jbjtd_dn3 = assign49050_e82077_d_n3;
        locals.var_jbjtd_dn4 = assign49050_e82077_d_n4;
        locals.var_jbjtd_dn5 = assign49050_e82077_d_n5;
        locals.var_jbjtd_dn6 = assign49050_e82077_d_n6;
        locals.var_jbjtd_dn7 = assign49050_e82077_d_n7;
        locals.var_jbjtd_dn8 = assign49050_e82077_d_n8;
        locals.var_jbjtd_dn9 = assign49050_e82077_d_n9;
        locals.var_jbjtd_dn10 = assign49050_e82077_d_n10;
        locals.var_jbjtd_dn11 = assign49050_e82077_d_n11;
        locals.var_jbjtd_rv = 0.0;

        let (assign49060_e82089, assign49060_e82089_d_n3, assign49060_e82089_d_n4, assign49060_e82089_d_n5, assign49060_e82089_d_n6, assign49060_e82089_d_n7, assign49060_e82089_d_n8, assign49060_e82089_d_n9, assign49060_e82089_d_n10, assign49060_e82089_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49060_e82086: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49060_e82087: f64 = (locals.var_ahlis * assign49060_e82086);
        (assign49060_e82087, ((locals.var_ahlis_dn3 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn3)), ((locals.var_ahlis_dn4 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn4)), ((locals.var_ahlis_dn5 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn5)), ((locals.var_ahlis_dn6 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn6)), ((locals.var_ahlis_dn7 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn7)), ((locals.var_ahlis_dn8 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn8)), ((locals.var_ahlis_dn9 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn9)), ((locals.var_ahlis_dn10 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn10)), ((locals.var_ahlis_dn11 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn11)),)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign49060_e82089;
        locals.var_ehlis_dn3 = assign49060_e82089_d_n3;
        locals.var_ehlis_dn4 = assign49060_e82089_d_n4;
        locals.var_ehlis_dn5 = assign49060_e82089_d_n5;
        locals.var_ehlis_dn6 = assign49060_e82089_d_n6;
        locals.var_ehlis_dn7 = assign49060_e82089_d_n7;
        locals.var_ehlis_dn8 = assign49060_e82089_d_n8;
        locals.var_ehlis_dn9 = assign49060_e82089_d_n9;
        locals.var_ehlis_dn10 = assign49060_e82089_d_n10;
        locals.var_ehlis_dn11 = assign49060_e82089_d_n11;
        locals.var_ehlis_rv = 0.0;

        let assign49070_e82092: f64 = if locals.var_ehlis < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign49070_e82092;
        locals.var_guard751_rv = 0.0;

        let (assign49080_e82102, assign49080_e82102_d_n3, assign49080_e82102_d_n4, assign49080_e82102_d_n5, assign49080_e82102_d_n6, assign49080_e82102_d_n7, assign49080_e82102_d_n8, assign49080_e82102_d_n9, assign49080_e82102_d_n10, assign49080_e82102_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign49080_e82102;
        locals.var_ehlis_dn3 = assign49080_e82102_d_n3;
        locals.var_ehlis_dn4 = assign49080_e82102_d_n4;
        locals.var_ehlis_dn5 = assign49080_e82102_d_n5;
        locals.var_ehlis_dn6 = assign49080_e82102_d_n6;
        locals.var_ehlis_dn7 = assign49080_e82102_d_n7;
        locals.var_ehlis_dn8 = assign49080_e82102_d_n8;
        locals.var_ehlis_dn9 = assign49080_e82102_d_n9;
        locals.var_ehlis_dn10 = assign49080_e82102_d_n10;
        locals.var_ehlis_dn11 = assign49080_e82102_d_n11;
        locals.var_ehlis_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_171(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49090_e82112, assign49090_e82112_d_n3, assign49090_e82112_d_n4, assign49090_e82112_d_n5, assign49090_e82112_d_n6, assign49090_e82112_d_n7, assign49090_e82112_d_n8, assign49090_e82112_d_n9, assign49090_e82112_d_n10, assign49090_e82112_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign49090_e82112;
        locals.var_ehlisfactor_dn3 = assign49090_e82112_d_n3;
        locals.var_ehlisfactor_dn4 = assign49090_e82112_d_n4;
        locals.var_ehlisfactor_dn5 = assign49090_e82112_d_n5;
        locals.var_ehlisfactor_dn6 = assign49090_e82112_d_n6;
        locals.var_ehlisfactor_dn7 = assign49090_e82112_d_n7;
        locals.var_ehlisfactor_dn8 = assign49090_e82112_d_n8;
        locals.var_ehlisfactor_dn9 = assign49090_e82112_d_n9;
        locals.var_ehlisfactor_dn10 = assign49090_e82112_d_n10;
        locals.var_ehlisfactor_dn11 = assign49090_e82112_d_n11;
        locals.var_ehlisfactor_rv = 0.0;

        let (assign49100_e82128, assign49100_e82128_d_n3, assign49100_e82128_d_n4, assign49100_e82128_d_n5, assign49100_e82128_d_n6, assign49100_e82128_d_n7, assign49100_e82128_d_n8, assign49100_e82128_d_n9, assign49100_e82128_d_n10, assign49100_e82128_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 == 0.0)) {
        let assign49100_e82124: f64 = (1.0 + locals.var_ehlis);
        let assign49100_e82125: f64 = (assign49100_e82124).sqrt();
        let assign49100_e82126: f64 = (1.0 / assign49100_e82125);
        (assign49100_e82126, (-((locals.var_ehlis_dn3 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn4 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn5 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn6 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn7 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn8 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn9 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn10 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn11 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))),)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign49100_e82128;
        locals.var_ehlisfactor_dn3 = assign49100_e82128_d_n3;
        locals.var_ehlisfactor_dn4 = assign49100_e82128_d_n4;
        locals.var_ehlisfactor_dn5 = assign49100_e82128_d_n5;
        locals.var_ehlisfactor_dn6 = assign49100_e82128_d_n6;
        locals.var_ehlisfactor_dn7 = assign49100_e82128_d_n7;
        locals.var_ehlisfactor_dn8 = assign49100_e82128_d_n8;
        locals.var_ehlisfactor_dn9 = assign49100_e82128_d_n9;
        locals.var_ehlisfactor_dn10 = assign49100_e82128_d_n10;
        locals.var_ehlisfactor_dn11 = assign49100_e82128_d_n11;
        locals.var_ehlisfactor_rv = 0.0;

        let (assign49110_e82140, assign49110_e82140_d_n3, assign49110_e82140_d_n4, assign49110_e82140_d_n5, assign49110_e82140_d_n6, assign49110_e82140_d_n7, assign49110_e82140_d_n8, assign49110_e82140_d_n9, assign49110_e82140_d_n10, assign49110_e82140_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49110_e82137: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49110_e82138: f64 = (locals.var_ahlid * assign49110_e82137);
        (assign49110_e82138, ((locals.var_ahlid_dn3 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn3)), ((locals.var_ahlid_dn4 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn4)), ((locals.var_ahlid_dn5 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn5)), ((locals.var_ahlid_dn6 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn6)), ((locals.var_ahlid_dn7 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn7)), ((locals.var_ahlid_dn8 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn8)), ((locals.var_ahlid_dn9 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn9)), ((locals.var_ahlid_dn10 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn10)), ((locals.var_ahlid_dn11 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn11)),)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign49110_e82140;
        locals.var_ehlid_dn3 = assign49110_e82140_d_n3;
        locals.var_ehlid_dn4 = assign49110_e82140_d_n4;
        locals.var_ehlid_dn5 = assign49110_e82140_d_n5;
        locals.var_ehlid_dn6 = assign49110_e82140_d_n6;
        locals.var_ehlid_dn7 = assign49110_e82140_d_n7;
        locals.var_ehlid_dn8 = assign49110_e82140_d_n8;
        locals.var_ehlid_dn9 = assign49110_e82140_d_n9;
        locals.var_ehlid_dn10 = assign49110_e82140_d_n10;
        locals.var_ehlid_dn11 = assign49110_e82140_d_n11;
        locals.var_ehlid_rv = 0.0;

        let assign49120_e82143: f64 = if locals.var_ehlid < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign49120_e82143;
        locals.var_guard752_rv = 0.0;

        let (assign49130_e82153, assign49130_e82153_d_n3, assign49130_e82153_d_n4, assign49130_e82153_d_n5, assign49130_e82153_d_n6, assign49130_e82153_d_n7, assign49130_e82153_d_n8, assign49130_e82153_d_n9, assign49130_e82153_d_n10, assign49130_e82153_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign49130_e82153;
        locals.var_ehlid_dn3 = assign49130_e82153_d_n3;
        locals.var_ehlid_dn4 = assign49130_e82153_d_n4;
        locals.var_ehlid_dn5 = assign49130_e82153_d_n5;
        locals.var_ehlid_dn6 = assign49130_e82153_d_n6;
        locals.var_ehlid_dn7 = assign49130_e82153_d_n7;
        locals.var_ehlid_dn8 = assign49130_e82153_d_n8;
        locals.var_ehlid_dn9 = assign49130_e82153_d_n9;
        locals.var_ehlid_dn10 = assign49130_e82153_d_n10;
        locals.var_ehlid_dn11 = assign49130_e82153_d_n11;
        locals.var_ehlid_rv = 0.0;

        let (assign49140_e82163, assign49140_e82163_d_n3, assign49140_e82163_d_n4, assign49140_e82163_d_n5, assign49140_e82163_d_n6, assign49140_e82163_d_n7, assign49140_e82163_d_n8, assign49140_e82163_d_n9, assign49140_e82163_d_n10, assign49140_e82163_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign49140_e82163;
        locals.var_ehlidfactor_dn3 = assign49140_e82163_d_n3;
        locals.var_ehlidfactor_dn4 = assign49140_e82163_d_n4;
        locals.var_ehlidfactor_dn5 = assign49140_e82163_d_n5;
        locals.var_ehlidfactor_dn6 = assign49140_e82163_d_n6;
        locals.var_ehlidfactor_dn7 = assign49140_e82163_d_n7;
        locals.var_ehlidfactor_dn8 = assign49140_e82163_d_n8;
        locals.var_ehlidfactor_dn9 = assign49140_e82163_d_n9;
        locals.var_ehlidfactor_dn10 = assign49140_e82163_d_n10;
        locals.var_ehlidfactor_dn11 = assign49140_e82163_d_n11;
        locals.var_ehlidfactor_rv = 0.0;

        let (assign49150_e82179, assign49150_e82179_d_n3, assign49150_e82179_d_n4, assign49150_e82179_d_n5, assign49150_e82179_d_n6, assign49150_e82179_d_n7, assign49150_e82179_d_n8, assign49150_e82179_d_n9, assign49150_e82179_d_n10, assign49150_e82179_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 == 0.0)) {
        let assign49150_e82175: f64 = (1.0 + locals.var_ehlid);
        let assign49150_e82176: f64 = (assign49150_e82175).sqrt();
        let assign49150_e82177: f64 = (1.0 / assign49150_e82176);
        (assign49150_e82177, (-((locals.var_ehlid_dn3 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn4 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn5 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn6 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn7 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn8 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn9 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn10 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn11 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign49150_e82179;
        locals.var_ehlidfactor_dn3 = assign49150_e82179_d_n3;
        locals.var_ehlidfactor_dn4 = assign49150_e82179_d_n4;
        locals.var_ehlidfactor_dn5 = assign49150_e82179_d_n5;
        locals.var_ehlidfactor_dn6 = assign49150_e82179_d_n6;
        locals.var_ehlidfactor_dn7 = assign49150_e82179_d_n7;
        locals.var_ehlidfactor_dn8 = assign49150_e82179_d_n8;
        locals.var_ehlidfactor_dn9 = assign49150_e82179_d_n9;
        locals.var_ehlidfactor_dn10 = assign49150_e82179_d_n10;
        locals.var_ehlidfactor_dn11 = assign49150_e82179_d_n11;
        locals.var_ehlidfactor_rv = 0.0;

        let (assign49160_e82196, assign49160_e82196_d_n3, assign49160_e82196_d_n4, assign49160_e82196_d_n5, assign49160_e82196_d_n6, assign49160_e82196_d_n7, assign49160_e82196_d_n8, assign49160_e82196_d_n9, assign49160_e82196_d_n10, assign49160_e82196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49160_e82186: f64 = (-0.5);
        let assign49160_e82188: f64 = (assign49160_e82186 * locals.var_leff);
        let assign49160_e82190: f64 = (assign49160_e82188 * locals.var_leff);
        let __rspice_inv_cse_0: f64 = 1.0 / p.p595;
        let assign49160_e82192: f64 = (assign49160_e82190 * __rspice_inv_cse_0);
        let assign49160_e82194: f64 = (assign49160_e82192 * __rspice_inv_cse_0);
        (assign49160_e82194, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49160_e82196;
        locals.var_t0_dn3 = assign49160_e82196_d_n3;
        locals.var_t0_dn4 = assign49160_e82196_d_n4;
        locals.var_t0_dn5 = assign49160_e82196_d_n5;
        locals.var_t0_dn6 = assign49160_e82196_d_n6;
        locals.var_t0_dn7 = assign49160_e82196_d_n7;
        locals.var_t0_dn8 = assign49160_e82196_d_n8;
        locals.var_t0_dn9 = assign49160_e82196_d_n9;
        locals.var_t0_dn10 = assign49160_e82196_d_n10;
        locals.var_t0_dn11 = assign49160_e82196_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49170_e82205, assign49170_e82205_d_n3, assign49170_e82205_d_n4, assign49170_e82205_d_n5, assign49170_e82205_d_n6, assign49170_e82205_d_n7, assign49170_e82205_d_n8, assign49170_e82205_d_n9, assign49170_e82205_d_n10, assign49170_e82205_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49170_e82203: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49170_e82203, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_alphabjt, locals.var_alphabjt_dn3, locals.var_alphabjt_dn4, locals.var_alphabjt_dn5, locals.var_alphabjt_dn6, locals.var_alphabjt_dn7, locals.var_alphabjt_dn8, locals.var_alphabjt_dn9, locals.var_alphabjt_dn10, locals.var_alphabjt_dn11,)
    }
};
        locals.var_alphabjt = assign49170_e82205;
        locals.var_alphabjt_dn3 = assign49170_e82205_d_n3;
        locals.var_alphabjt_dn4 = assign49170_e82205_d_n4;
        locals.var_alphabjt_dn5 = assign49170_e82205_d_n5;
        locals.var_alphabjt_dn6 = assign49170_e82205_d_n6;
        locals.var_alphabjt_dn7 = assign49170_e82205_d_n7;
        locals.var_alphabjt_dn8 = assign49170_e82205_d_n8;
        locals.var_alphabjt_dn9 = assign49170_e82205_d_n9;
        locals.var_alphabjt_dn10 = assign49170_e82205_d_n10;
        locals.var_alphabjt_dn11 = assign49170_e82205_d_n11;
        locals.var_alphabjt_rv = 0.0;

        let (assign49180_e82215, assign49180_e82215_d_n3, assign49180_e82215_d_n4, assign49180_e82215_d_n5, assign49180_e82215_d_n6, assign49180_e82215_d_n7, assign49180_e82215_d_n8, assign49180_e82215_d_n9, assign49180_e82215_d_n10, assign49180_e82215_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49180_e82213: f64 = (1.0 - locals.var_alphabjt);
        (assign49180_e82213, (-locals.var_alphabjt_dn3), (-locals.var_alphabjt_dn4), (-locals.var_alphabjt_dn5), (-locals.var_alphabjt_dn6), (-locals.var_alphabjt_dn7), (-locals.var_alphabjt_dn8), (-locals.var_alphabjt_dn9), (-locals.var_alphabjt_dn10), (-locals.var_alphabjt_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49180_e82215;
        locals.var_t2_dn3 = assign49180_e82215_d_n3;
        locals.var_t2_dn4 = assign49180_e82215_d_n4;
        locals.var_t2_dn5 = assign49180_e82215_d_n5;
        locals.var_t2_dn6 = assign49180_e82215_d_n6;
        locals.var_t2_dn7 = assign49180_e82215_d_n7;
        locals.var_t2_dn8 = assign49180_e82215_d_n8;
        locals.var_t2_dn9 = assign49180_e82215_d_n9;
        locals.var_t2_dn10 = assign49180_e82215_d_n10;
        locals.var_t2_dn11 = assign49180_e82215_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign49190_e82231, assign49190_e82231_d_n3, assign49190_e82231_d_n4, assign49190_e82231_d_n5, assign49190_e82231_d_n6, assign49190_e82231_d_n7, assign49190_e82231_d_n8, assign49190_e82231_d_n9, assign49190_e82231_d_n10, assign49190_e82231_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49190_e82224: f64 = (1.0 / locals.var_leff);
        let assign49190_e82227: f64 = (1.0 / p.p595);
        let assign49190_e82228: f64 = (assign49190_e82224 + assign49190_e82227);
        let assign49190_e82229: f64 = (locals.var_lbjt0_i * assign49190_e82228);
        (assign49190_e82229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49190_e82231;
        locals.var_t0_dn3 = assign49190_e82231_d_n3;
        locals.var_t0_dn4 = assign49190_e82231_d_n4;
        locals.var_t0_dn5 = assign49190_e82231_d_n5;
        locals.var_t0_dn6 = assign49190_e82231_d_n6;
        locals.var_t0_dn7 = assign49190_e82231_d_n7;
        locals.var_t0_dn8 = assign49190_e82231_d_n8;
        locals.var_t0_dn9 = assign49190_e82231_d_n9;
        locals.var_t0_dn10 = assign49190_e82231_d_n10;
        locals.var_t0_dn11 = assign49190_e82231_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49200_e82241, assign49200_e82241_d_n3, assign49200_e82241_d_n4, assign49200_e82241_d_n5, assign49200_e82241_d_n6, assign49200_e82241_d_n7, assign49200_e82241_d_n8, assign49200_e82241_d_n9, assign49200_e82241_d_n10, assign49200_e82241_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49200_e82239: f64 = (locals.var_t0).powf(locals.var_nbjt_i);
        (assign49200_e82239, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn3)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn3 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn4)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn4 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn5)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn5 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn6)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn6 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn7)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn7 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn8)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn8 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn9)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn9 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn10)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn10 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn11)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn11 / locals.var_t0))) },)
    } else {
        (locals.var_lratio, locals.var_lratio_dn3, locals.var_lratio_dn4, locals.var_lratio_dn5, locals.var_lratio_dn6, locals.var_lratio_dn7, locals.var_lratio_dn8, locals.var_lratio_dn9, locals.var_lratio_dn10, locals.var_lratio_dn11,)
    }
};
        locals.var_lratio = assign49200_e82241;
        locals.var_lratio_dn3 = assign49200_e82241_d_n3;
        locals.var_lratio_dn4 = assign49200_e82241_d_n4;
        locals.var_lratio_dn5 = assign49200_e82241_d_n5;
        locals.var_lratio_dn6 = assign49200_e82241_d_n6;
        locals.var_lratio_dn7 = assign49200_e82241_d_n7;
        locals.var_lratio_dn8 = assign49200_e82241_d_n8;
        locals.var_lratio_dn9 = assign49200_e82241_d_n9;
        locals.var_lratio_dn10 = assign49200_e82241_d_n10;
        locals.var_lratio_dn11 = assign49200_e82241_d_n11;
        locals.var_lratio_rv = 0.0;

        let (assign49210_e82253, assign49210_e82253_d_n3, assign49210_e82253_d_n4, assign49210_e82253_d_n5, assign49210_e82253_d_n6, assign49210_e82253_d_n7, assign49210_e82253_d_n8, assign49210_e82253_d_n9, assign49210_e82253_d_n10, assign49210_e82253_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49210_e82249: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign49210_e82251: f64 = (assign49210_e82249 * locals.var_lratio);
        (assign49210_e82251, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign49210_e82253;
        locals.var_ien_dn3 = assign49210_e82253_d_n3;
        locals.var_ien_dn4 = assign49210_e82253_d_n4;
        locals.var_ien_dn5 = assign49210_e82253_d_n5;
        locals.var_ien_dn6 = assign49210_e82253_d_n6;
        locals.var_ien_dn7 = assign49210_e82253_d_n7;
        locals.var_ien_dn8 = assign49210_e82253_d_n8;
        locals.var_ien_dn9 = assign49210_e82253_d_n9;
        locals.var_ien_dn10 = assign49210_e82253_d_n10;
        locals.var_ien_dn11 = assign49210_e82253_d_n11;
        locals.var_ien_rv = 0.0;

        let (assign49220_e82263, assign49220_e82263_d_n3, assign49220_e82263_d_n4, assign49220_e82263_d_n5, assign49220_e82263_d_n6, assign49220_e82263_d_n7, assign49220_e82263_d_n8, assign49220_e82263_d_n9, assign49220_e82263_d_n10, assign49220_e82263_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49220_e82261: f64 = (locals.var_t0 * locals.var_ien);
        (assign49220_e82261, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49220_e82263;
        locals.var_t1_dn3 = assign49220_e82263_d_n3;
        locals.var_t1_dn4 = assign49220_e82263_d_n4;
        locals.var_t1_dn5 = assign49220_e82263_d_n5;
        locals.var_t1_dn6 = assign49220_e82263_d_n6;
        locals.var_t1_dn7 = assign49220_e82263_d_n7;
        locals.var_t1_dn8 = assign49220_e82263_d_n8;
        locals.var_t1_dn9 = assign49220_e82263_d_n9;
        locals.var_t1_dn10 = assign49220_e82263_d_n10;
        locals.var_t1_dn11 = assign49220_e82263_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49240_e82289, assign49240_e82289_d_n3, assign49240_e82289_d_n4, assign49240_e82289_d_n5, assign49240_e82289_d_n6, assign49240_e82289_d_n7, assign49240_e82289_d_n8, assign49240_e82289_d_n9, assign49240_e82289_d_n10, assign49240_e82289_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49240_e82285: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign49240_e82287: f64 = (assign49240_e82285 * locals.var_lratio);
        (assign49240_e82287, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign49240_e82289;
        locals.var_ien_dn3 = assign49240_e82289_d_n3;
        locals.var_ien_dn4 = assign49240_e82289_d_n4;
        locals.var_ien_dn5 = assign49240_e82289_d_n5;
        locals.var_ien_dn6 = assign49240_e82289_d_n6;
        locals.var_ien_dn7 = assign49240_e82289_d_n7;
        locals.var_ien_dn8 = assign49240_e82289_d_n8;
        locals.var_ien_dn9 = assign49240_e82289_d_n9;
        locals.var_ien_dn10 = assign49240_e82289_d_n10;
        locals.var_ien_dn11 = assign49240_e82289_d_n11;
        locals.var_ien_rv = 0.0;

        let (assign49250_e82299, assign49250_e82299_d_n3, assign49250_e82299_d_n4, assign49250_e82299_d_n5, assign49250_e82299_d_n6, assign49250_e82299_d_n7, assign49250_e82299_d_n8, assign49250_e82299_d_n9, assign49250_e82299_d_n10, assign49250_e82299_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49250_e82297: f64 = (locals.var_t0 * locals.var_ien);
        (assign49250_e82297, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49250_e82299;
        locals.var_t1_dn3 = assign49250_e82299_d_n3;
        locals.var_t1_dn4 = assign49250_e82299_d_n4;
        locals.var_t1_dn5 = assign49250_e82299_d_n5;
        locals.var_t1_dn6 = assign49250_e82299_d_n6;
        locals.var_t1_dn7 = assign49250_e82299_d_n7;
        locals.var_t1_dn8 = assign49250_e82299_d_n8;
        locals.var_t1_dn9 = assign49250_e82299_d_n9;
        locals.var_t1_dn10 = assign49250_e82299_d_n10;
        locals.var_t1_dn11 = assign49250_e82299_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49270_e82327, assign49270_e82327_d_n3, assign49270_e82327_d_n4, assign49270_e82327_d_n5, assign49270_e82327_d_n6, assign49270_e82327_d_n7, assign49270_e82327_d_n8, assign49270_e82327_d_n9, assign49270_e82327_d_n10, assign49270_e82327_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49270_e82323: f64 = (locals.var_t0).powf(locals.var_ndif_i);
        let assign49270_e82324: f64 = (p.p920 * assign49270_e82323);
        let assign49270_e82325: f64 = (1.0 + assign49270_e82324);
        (assign49270_e82325, (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn3)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn3 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn4)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn4 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn5)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn5 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn6)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn6 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn7)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn7 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn8)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn8 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn9)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn9 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn10)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn10 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn11)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn11 / locals.var_t0))) }),)
    } else {
        (locals.var_lratiodif, locals.var_lratiodif_dn3, locals.var_lratiodif_dn4, locals.var_lratiodif_dn5, locals.var_lratiodif_dn6, locals.var_lratiodif_dn7, locals.var_lratiodif_dn8, locals.var_lratiodif_dn9, locals.var_lratiodif_dn10, locals.var_lratiodif_dn11,)
    }
};
        locals.var_lratiodif = assign49270_e82327;
        locals.var_lratiodif_dn3 = assign49270_e82327_d_n3;
        locals.var_lratiodif_dn4 = assign49270_e82327_d_n4;
        locals.var_lratiodif_dn5 = assign49270_e82327_d_n5;
        locals.var_lratiodif_dn6 = assign49270_e82327_d_n6;
        locals.var_lratiodif_dn7 = assign49270_e82327_d_n7;
        locals.var_lratiodif_dn8 = assign49270_e82327_d_n8;
        locals.var_lratiodif_dn9 = assign49270_e82327_d_n9;
        locals.var_lratiodif_dn10 = assign49270_e82327_d_n10;
        locals.var_lratiodif_dn11 = assign49270_e82327_d_n11;
        locals.var_lratiodif_rv = 0.0;

        let (assign49280_e82339, assign49280_e82339_d_n3, assign49280_e82339_d_n4, assign49280_e82339_d_n5, assign49280_e82339_d_n6, assign49280_e82339_d_n7, assign49280_e82339_d_n8, assign49280_e82339_d_n9, assign49280_e82339_d_n10, assign49280_e82339_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49280_e82335: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign49280_e82337: f64 = (assign49280_e82335 * locals.var_lratiodif);
        (assign49280_e82337, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign49280_e82339;
        locals.var_iendif_dn3 = assign49280_e82339_d_n3;
        locals.var_iendif_dn4 = assign49280_e82339_d_n4;
        locals.var_iendif_dn5 = assign49280_e82339_d_n5;
        locals.var_iendif_dn6 = assign49280_e82339_d_n6;
        locals.var_iendif_dn7 = assign49280_e82339_d_n7;
        locals.var_iendif_dn8 = assign49280_e82339_d_n8;
        locals.var_iendif_dn9 = assign49280_e82339_d_n9;
        locals.var_iendif_dn10 = assign49280_e82339_d_n10;
        locals.var_iendif_dn11 = assign49280_e82339_d_n11;
        locals.var_iendif_rv = 0.0;

        let (assign49290_e82353, assign49290_e82353_d_n3, assign49290_e82353_d_n4, assign49290_e82353_d_n5, assign49290_e82353_d_n6, assign49290_e82353_d_n7, assign49290_e82353_d_n8, assign49290_e82353_d_n9, assign49290_e82353_d_n10, assign49290_e82353_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49290_e82348: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49290_e82349: f64 = (locals.var_iendif * assign49290_e82348);
        let assign49290_e82351: f64 = (assign49290_e82349 * locals.var_ehlisfactor);
        (assign49290_e82351, ((((locals.var_iendif_dn3 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn11)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11,)
    }
};
        locals.var_ibsdif = assign49290_e82353;
        locals.var_ibsdif_dn3 = assign49290_e82353_d_n3;
        locals.var_ibsdif_dn4 = assign49290_e82353_d_n4;
        locals.var_ibsdif_dn5 = assign49290_e82353_d_n5;
        locals.var_ibsdif_dn6 = assign49290_e82353_d_n6;
        locals.var_ibsdif_dn7 = assign49290_e82353_d_n7;
        locals.var_ibsdif_dn8 = assign49290_e82353_d_n8;
        locals.var_ibsdif_dn9 = assign49290_e82353_d_n9;
        locals.var_ibsdif_dn10 = assign49290_e82353_d_n10;
        locals.var_ibsdif_dn11 = assign49290_e82353_d_n11;
        locals.var_ibsdif_rv = 0.0;

        let (assign49300_e82365, assign49300_e82365_d_n3, assign49300_e82365_d_n4, assign49300_e82365_d_n5, assign49300_e82365_d_n6, assign49300_e82365_d_n7, assign49300_e82365_d_n8, assign49300_e82365_d_n9, assign49300_e82365_d_n10, assign49300_e82365_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49300_e82361: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign49300_e82363: f64 = (assign49300_e82361 * locals.var_lratiodif);
        (assign49300_e82363, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign49300_e82365;
        locals.var_iendif_dn3 = assign49300_e82365_d_n3;
        locals.var_iendif_dn4 = assign49300_e82365_d_n4;
        locals.var_iendif_dn5 = assign49300_e82365_d_n5;
        locals.var_iendif_dn6 = assign49300_e82365_d_n6;
        locals.var_iendif_dn7 = assign49300_e82365_d_n7;
        locals.var_iendif_dn8 = assign49300_e82365_d_n8;
        locals.var_iendif_dn9 = assign49300_e82365_d_n9;
        locals.var_iendif_dn10 = assign49300_e82365_d_n10;
        locals.var_iendif_dn11 = assign49300_e82365_d_n11;
        locals.var_iendif_rv = 0.0;

        let (assign49310_e82379, assign49310_e82379_d_n3, assign49310_e82379_d_n4, assign49310_e82379_d_n5, assign49310_e82379_d_n6, assign49310_e82379_d_n7, assign49310_e82379_d_n8, assign49310_e82379_d_n9, assign49310_e82379_d_n10, assign49310_e82379_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49310_e82374: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49310_e82375: f64 = (locals.var_iendif * assign49310_e82374);
        let assign49310_e82377: f64 = (assign49310_e82375 * locals.var_ehlidfactor);
        (assign49310_e82377, ((((locals.var_iendif_dn3 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn11)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11,)
    }
};
        locals.var_ibddif = assign49310_e82379;
        locals.var_ibddif_dn3 = assign49310_e82379_d_n3;
        locals.var_ibddif_dn4 = assign49310_e82379_d_n4;
        locals.var_ibddif_dn5 = assign49310_e82379_d_n5;
        locals.var_ibddif_dn6 = assign49310_e82379_d_n6;
        locals.var_ibddif_dn7 = assign49310_e82379_d_n7;
        locals.var_ibddif_dn8 = assign49310_e82379_d_n8;
        locals.var_ibddif_dn9 = assign49310_e82379_d_n9;
        locals.var_ibddif_dn10 = assign49310_e82379_d_n10;
        locals.var_ibddif_dn11 = assign49310_e82379_d_n11;
        locals.var_ibddif_rv = 0.0;

        let (assign49320_e82391,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49320_e82388: f64 = (locals.var_aely_i * locals.var_leff);
        let assign49320_e82389: f64 = (locals.var_vabjt_i + assign49320_e82388);
        (assign49320_e82389,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign49320_e82391;
        locals.var_vearly_rv = 0.0;

        let assign49330_e82394: f64 = if locals.var_vearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign49330_e82394;
        locals.var_guard753_rv = 0.0;

        let (assign49340_e82404,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard753 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign49340_e82404;
        locals.var_vearly_rv = 0.0;

        let assign49350_e82407: f64 = if p.p554 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign49350_e82407;
        locals.var_guard754_rv = 0.0;

        let (assign49370_e82434, assign49370_e82434_d_n3, assign49370_e82434_d_n4, assign49370_e82434_d_n5, assign49370_e82434_d_n6, assign49370_e82434_d_n7, assign49370_e82434_d_n8, assign49370_e82434_d_n9, assign49370_e82434_d_n10, assign49370_e82434_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49370_e82429: f64 = (locals.var_vbs_jct + locals.var_vbd_jct);
        let assign49370_e82431: f64 = (assign49370_e82429 / locals.var_vearly);
        let assign49370_e82432: f64 = (1.0 + assign49370_e82431);
        (assign49370_e82432, 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn6 / locals.var_vearly), (locals.var_vbs_jct_dn7 / locals.var_vearly), 0.0, 0.0, ((locals.var_vbs_jct_dn10 + locals.var_vbd_jct_dn10) / locals.var_vearly), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49370_e82434;
        locals.var_t0_dn3 = assign49370_e82434_d_n3;
        locals.var_t0_dn4 = assign49370_e82434_d_n4;
        locals.var_t0_dn5 = assign49370_e82434_d_n5;
        locals.var_t0_dn6 = assign49370_e82434_d_n6;
        locals.var_t0_dn7 = assign49370_e82434_d_n7;
        locals.var_t0_dn8 = assign49370_e82434_d_n8;
        locals.var_t0_dn9 = assign49370_e82434_d_n9;
        locals.var_t0_dn10 = assign49370_e82434_d_n10;
        locals.var_t0_dn11 = assign49370_e82434_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49380_e82447, assign49380_e82447_d_n3, assign49380_e82447_d_n4, assign49380_e82447_d_n5, assign49380_e82447_d_n6, assign49380_e82447_d_n7, assign49380_e82447_d_n8, assign49380_e82447_d_n9, assign49380_e82447_d_n10, assign49380_e82447_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49380_e82445: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign49380_e82445, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49380_e82447;
        locals.var_t1_dn3 = assign49380_e82447_d_n3;
        locals.var_t1_dn4 = assign49380_e82447_d_n4;
        locals.var_t1_dn5 = assign49380_e82447_d_n5;
        locals.var_t1_dn6 = assign49380_e82447_d_n6;
        locals.var_t1_dn7 = assign49380_e82447_d_n7;
        locals.var_t1_dn8 = assign49380_e82447_d_n8;
        locals.var_t1_dn9 = assign49380_e82447_d_n9;
        locals.var_t1_dn10 = assign49380_e82447_d_n10;
        locals.var_t1_dn11 = assign49380_e82447_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_172(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49390_e82465, assign49390_e82465_d_n3, assign49390_e82465_d_n4, assign49390_e82465_d_n5, assign49390_e82465_d_n6, assign49390_e82465_d_n7, assign49390_e82465_d_n8, assign49390_e82465_d_n9, assign49390_e82465_d_n10, assign49390_e82465_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49390_e82458: f64 = (locals.var_t0 * locals.var_t0);
        let assign49390_e82461: f64 = (4.0 * locals.var_t1);
        let assign49390_e82462: f64 = (assign49390_e82458 + assign49390_e82461);
        let assign49390_e82463: f64 = (assign49390_e82462).sqrt();
        (assign49390_e82463, ((((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + (4.0 * locals.var_t1_dn3)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + (4.0 * locals.var_t1_dn4)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + (4.0 * locals.var_t1_dn5)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + (4.0 * locals.var_t1_dn6)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + (4.0 * locals.var_t1_dn7)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + (4.0 * locals.var_t1_dn8)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + (4.0 * locals.var_t1_dn9)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + (4.0 * locals.var_t1_dn10)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + (4.0 * locals.var_t1_dn11)) / (2.0 * assign49390_e82463)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49390_e82465;
        locals.var_t3_dn3 = assign49390_e82465_d_n3;
        locals.var_t3_dn4 = assign49390_e82465_d_n4;
        locals.var_t3_dn5 = assign49390_e82465_d_n5;
        locals.var_t3_dn6 = assign49390_e82465_d_n6;
        locals.var_t3_dn7 = assign49390_e82465_d_n7;
        locals.var_t3_dn8 = assign49390_e82465_d_n8;
        locals.var_t3_dn9 = assign49390_e82465_d_n9;
        locals.var_t3_dn10 = assign49390_e82465_d_n10;
        locals.var_t3_dn11 = assign49390_e82465_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49400_e82480, assign49400_e82480_d_n3, assign49400_e82480_d_n4, assign49400_e82480_d_n5, assign49400_e82480_d_n6, assign49400_e82480_d_n7, assign49400_e82480_d_n8, assign49400_e82480_d_n9, assign49400_e82480_d_n10, assign49400_e82480_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49400_e82476: f64 = (locals.var_t0 + locals.var_t3);
        let assign49400_e82478: f64 = (assign49400_e82476 / 2.0);
        (assign49400_e82478, ((locals.var_t0_dn3 + locals.var_t3_dn3) / 2.0), ((locals.var_t0_dn4 + locals.var_t3_dn4) / 2.0), ((locals.var_t0_dn5 + locals.var_t3_dn5) / 2.0), ((locals.var_t0_dn6 + locals.var_t3_dn6) / 2.0), ((locals.var_t0_dn7 + locals.var_t3_dn7) / 2.0), ((locals.var_t0_dn8 + locals.var_t3_dn8) / 2.0), ((locals.var_t0_dn9 + locals.var_t3_dn9) / 2.0), ((locals.var_t0_dn10 + locals.var_t3_dn10) / 2.0), ((locals.var_t0_dn11 + locals.var_t3_dn11) / 2.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49400_e82480;
        locals.var_t2_dn3 = assign49400_e82480_d_n3;
        locals.var_t2_dn4 = assign49400_e82480_d_n4;
        locals.var_t2_dn5 = assign49400_e82480_d_n5;
        locals.var_t2_dn6 = assign49400_e82480_d_n6;
        locals.var_t2_dn7 = assign49400_e82480_d_n7;
        locals.var_t2_dn8 = assign49400_e82480_d_n8;
        locals.var_t2_dn9 = assign49400_e82480_d_n9;
        locals.var_t2_dn10 = assign49400_e82480_d_n10;
        locals.var_t2_dn11 = assign49400_e82480_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign49440_e82525, assign49440_e82525_d_n3, assign49440_e82525_d_n4, assign49440_e82525_d_n5, assign49440_e82525_d_n6, assign49440_e82525_d_n7, assign49440_e82525_d_n8, assign49440_e82525_d_n9, assign49440_e82525_d_n10, assign49440_e82525_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49440_e82523: f64 = (locals.var_alphabjt * locals.var_ien);
        (assign49440_e82523, ((locals.var_alphabjt_dn3 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn3)), ((locals.var_alphabjt_dn4 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn4)), ((locals.var_alphabjt_dn5 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn5)), ((locals.var_alphabjt_dn6 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn6)), ((locals.var_alphabjt_dn7 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn7)), ((locals.var_alphabjt_dn8 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn8)), ((locals.var_alphabjt_dn9 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn9)), ((locals.var_alphabjt_dn10 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn10)), ((locals.var_alphabjt_dn11 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49440_e82525;
        locals.var_t0_dn3 = assign49440_e82525_d_n3;
        locals.var_t0_dn4 = assign49440_e82525_d_n4;
        locals.var_t0_dn5 = assign49440_e82525_d_n5;
        locals.var_t0_dn6 = assign49440_e82525_d_n6;
        locals.var_t0_dn7 = assign49440_e82525_d_n7;
        locals.var_t0_dn8 = assign49440_e82525_d_n8;
        locals.var_t0_dn9 = assign49440_e82525_d_n9;
        locals.var_t0_dn10 = assign49440_e82525_d_n10;
        locals.var_t0_dn11 = assign49440_e82525_d_n11;
        locals.var_t0_rv = 0.0;

        let assign49460_e82551: f64 = if ((locals.var_istun_i == 0.0) && (locals.var_idtun_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign49460_e82551;
        locals.var_guard756_rv = 0.0;

        let (assign49490_e82577, assign49490_e82577_d_n3, assign49490_e82577_d_n4, assign49490_e82577_d_n5, assign49490_e82577_d_n6, assign49490_e82577_d_n7, assign49490_e82577_d_n8, assign49490_e82577_d_n9, assign49490_e82577_d_n10, assign49490_e82577_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49490_e82574: f64 = (locals.var_tratio - 1.0);
        let assign49490_e82575: f64 = (locals.var_xtun_i * assign49490_e82574);
        (assign49490_e82575, 0.0, (locals.var_xtun_i * locals.var_tratio_dn4), (locals.var_xtun_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49490_e82577;
        locals.var_t7_dn3 = assign49490_e82577_d_n3;
        locals.var_t7_dn4 = assign49490_e82577_d_n4;
        locals.var_t7_dn5 = assign49490_e82577_d_n5;
        locals.var_t7_dn6 = assign49490_e82577_d_n6;
        locals.var_t7_dn7 = assign49490_e82577_d_n7;
        locals.var_t7_dn8 = assign49490_e82577_d_n8;
        locals.var_t7_dn9 = assign49490_e82577_d_n9;
        locals.var_t7_dn10 = assign49490_e82577_d_n10;
        locals.var_t7_dn11 = assign49490_e82577_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign49500_e82586, assign49500_e82586_d_n3, assign49500_e82586_d_n4, assign49500_e82586_d_n5, assign49500_e82586_d_n6, assign49500_e82586_d_n7, assign49500_e82586_d_n8, assign49500_e82586_d_n9, assign49500_e82586_d_n10, assign49500_e82586_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49500_e82584: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49500_e82584, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49500_e82586;
        locals.var_t0_dn3 = assign49500_e82586_d_n3;
        locals.var_t0_dn4 = assign49500_e82586_d_n4;
        locals.var_t0_dn5 = assign49500_e82586_d_n5;
        locals.var_t0_dn6 = assign49500_e82586_d_n6;
        locals.var_t0_dn7 = assign49500_e82586_d_n7;
        locals.var_t0_dn8 = assign49500_e82586_d_n8;
        locals.var_t0_dn9 = assign49500_e82586_d_n9;
        locals.var_t0_dn10 = assign49500_e82586_d_n10;
        locals.var_t0_dn11 = assign49500_e82586_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49510_e82596, assign49510_e82596_d_n3, assign49510_e82596_d_n4, assign49510_e82596_d_n5, assign49510_e82596_d_n6, assign49510_e82596_d_n7, assign49510_e82596_d_n8, assign49510_e82596_d_n9, assign49510_e82596_d_n10, assign49510_e82596_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49510_e82594: f64 = (locals.var_istun_i * locals.var_t0);
        (assign49510_e82594, (locals.var_istun_i * locals.var_t0_dn3), (locals.var_istun_i * locals.var_t0_dn4), (locals.var_istun_i * locals.var_t0_dn5), (locals.var_istun_i * locals.var_t0_dn6), (locals.var_istun_i * locals.var_t0_dn7), (locals.var_istun_i * locals.var_t0_dn8), (locals.var_istun_i * locals.var_t0_dn9), (locals.var_istun_i * locals.var_t0_dn10), (locals.var_istun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11,)
    }
};
        locals.var_jtuns = assign49510_e82596;
        locals.var_jtuns_dn3 = assign49510_e82596_d_n3;
        locals.var_jtuns_dn4 = assign49510_e82596_d_n4;
        locals.var_jtuns_dn5 = assign49510_e82596_d_n5;
        locals.var_jtuns_dn6 = assign49510_e82596_d_n6;
        locals.var_jtuns_dn7 = assign49510_e82596_d_n7;
        locals.var_jtuns_dn8 = assign49510_e82596_d_n8;
        locals.var_jtuns_dn9 = assign49510_e82596_d_n9;
        locals.var_jtuns_dn10 = assign49510_e82596_d_n10;
        locals.var_jtuns_dn11 = assign49510_e82596_d_n11;
        locals.var_jtuns_rv = 0.0;

        let (assign49520_e82608, assign49520_e82608_d_n3, assign49520_e82608_d_n4, assign49520_e82608_d_n5, assign49520_e82608_d_n6, assign49520_e82608_d_n7, assign49520_e82608_d_n8, assign49520_e82608_d_n9, assign49520_e82608_d_n10, assign49520_e82608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49520_e82605: f64 = (locals.var_tratio - 1.0);
        let assign49520_e82606: f64 = (locals.var_xtund_i * assign49520_e82605);
        (assign49520_e82606, 0.0, (locals.var_xtund_i * locals.var_tratio_dn4), (locals.var_xtund_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49520_e82608;
        locals.var_t7_dn3 = assign49520_e82608_d_n3;
        locals.var_t7_dn4 = assign49520_e82608_d_n4;
        locals.var_t7_dn5 = assign49520_e82608_d_n5;
        locals.var_t7_dn6 = assign49520_e82608_d_n6;
        locals.var_t7_dn7 = assign49520_e82608_d_n7;
        locals.var_t7_dn8 = assign49520_e82608_d_n8;
        locals.var_t7_dn9 = assign49520_e82608_d_n9;
        locals.var_t7_dn10 = assign49520_e82608_d_n10;
        locals.var_t7_dn11 = assign49520_e82608_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign49530_e82617, assign49530_e82617_d_n3, assign49530_e82617_d_n4, assign49530_e82617_d_n5, assign49530_e82617_d_n6, assign49530_e82617_d_n7, assign49530_e82617_d_n8, assign49530_e82617_d_n9, assign49530_e82617_d_n10, assign49530_e82617_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49530_e82615: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49530_e82615, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49530_e82617;
        locals.var_t0_dn3 = assign49530_e82617_d_n3;
        locals.var_t0_dn4 = assign49530_e82617_d_n4;
        locals.var_t0_dn5 = assign49530_e82617_d_n5;
        locals.var_t0_dn6 = assign49530_e82617_d_n6;
        locals.var_t0_dn7 = assign49530_e82617_d_n7;
        locals.var_t0_dn8 = assign49530_e82617_d_n8;
        locals.var_t0_dn9 = assign49530_e82617_d_n9;
        locals.var_t0_dn10 = assign49530_e82617_d_n10;
        locals.var_t0_dn11 = assign49530_e82617_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49540_e82627, assign49540_e82627_d_n3, assign49540_e82627_d_n4, assign49540_e82627_d_n5, assign49540_e82627_d_n6, assign49540_e82627_d_n7, assign49540_e82627_d_n8, assign49540_e82627_d_n9, assign49540_e82627_d_n10, assign49540_e82627_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49540_e82625: f64 = (locals.var_idtun_i * locals.var_t0);
        (assign49540_e82625, (locals.var_idtun_i * locals.var_t0_dn3), (locals.var_idtun_i * locals.var_t0_dn4), (locals.var_idtun_i * locals.var_t0_dn5), (locals.var_idtun_i * locals.var_t0_dn6), (locals.var_idtun_i * locals.var_t0_dn7), (locals.var_idtun_i * locals.var_t0_dn8), (locals.var_idtun_i * locals.var_t0_dn9), (locals.var_idtun_i * locals.var_t0_dn10), (locals.var_idtun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11,)
    }
};
        locals.var_jtund = assign49540_e82627;
        locals.var_jtund_dn3 = assign49540_e82627_d_n3;
        locals.var_jtund_dn4 = assign49540_e82627_d_n4;
        locals.var_jtund_dn5 = assign49540_e82627_d_n5;
        locals.var_jtund_dn6 = assign49540_e82627_d_n6;
        locals.var_jtund_dn7 = assign49540_e82627_d_n7;
        locals.var_jtund_dn8 = assign49540_e82627_d_n8;
        locals.var_jtund_dn9 = assign49540_e82627_d_n9;
        locals.var_jtund_dn10 = assign49540_e82627_d_n10;
        locals.var_jtund_dn11 = assign49540_e82627_d_n11;
        locals.var_jtund_rv = 0.0;

        let (assign49550_e82637, assign49550_e82637_d_n4, assign49550_e82637_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49550_e82635: f64 = (p.p925 * locals.var_ntun_i);
        (assign49550_e82635, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign49550_e82637;
        locals.var_nvtm2_dn4 = assign49550_e82637_d_n4;
        locals.var_nvtm2_dn5 = assign49550_e82637_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let assign49560_e82640: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign49560_e82642: f64 = if assign49560_e82640 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign49560_e82642;
        locals.var_guard757_rv = 0.0;

        let (assign49570_e82652, assign49570_e82652_d_n3, assign49570_e82652_d_n4, assign49570_e82652_d_n5, assign49570_e82652_d_n6, assign49570_e82652_d_n7, assign49570_e82652_d_n8, assign49570_e82652_d_n9, assign49570_e82652_d_n10, assign49570_e82652_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49570_e82652;
        locals.var_t1_dn3 = assign49570_e82652_d_n3;
        locals.var_t1_dn4 = assign49570_e82652_d_n4;
        locals.var_t1_dn5 = assign49570_e82652_d_n5;
        locals.var_t1_dn6 = assign49570_e82652_d_n6;
        locals.var_t1_dn7 = assign49570_e82652_d_n7;
        locals.var_t1_dn8 = assign49570_e82652_d_n8;
        locals.var_t1_dn9 = assign49570_e82652_d_n9;
        locals.var_t1_dn10 = assign49570_e82652_d_n10;
        locals.var_t1_dn11 = assign49570_e82652_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49580_e82669, assign49580_e82669_d_n3, assign49580_e82669_d_n4, assign49580_e82669_d_n5, assign49580_e82669_d_n6, assign49580_e82669_d_n7, assign49580_e82669_d_n8, assign49580_e82669_d_n9, assign49580_e82669_d_n10, assign49580_e82669_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49580_e82661: f64 = (-locals.var_vbs_jct);
        let assign49580_e82663: f64 = (assign49580_e82661 / locals.var_nvtm2);
        let assign49580_e82665: f64 = (assign49580_e82663 * locals.var_vtun0_i);
        let assign49580_e82667: f64 = (assign49580_e82665 * locals.var_t1);
        (assign49580_e82667, (assign49580_e82665 * locals.var_t1_dn3), ((((-((assign49580_e82661 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn4)), ((((-((assign49580_e82661 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn5)), (assign49580_e82665 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn7)), (assign49580_e82665 * locals.var_t1_dn8), (assign49580_e82665 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn10)), (assign49580_e82665 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49580_e82669;
        locals.var_t0_dn3 = assign49580_e82669_d_n3;
        locals.var_t0_dn4 = assign49580_e82669_d_n4;
        locals.var_t0_dn5 = assign49580_e82669_d_n5;
        locals.var_t0_dn6 = assign49580_e82669_d_n6;
        locals.var_t0_dn7 = assign49580_e82669_d_n7;
        locals.var_t0_dn8 = assign49580_e82669_d_n8;
        locals.var_t0_dn9 = assign49580_e82669_d_n9;
        locals.var_t0_dn10 = assign49580_e82669_d_n10;
        locals.var_t0_dn11 = assign49580_e82669_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49590_e82680, assign49590_e82680_d_n3, assign49590_e82680_d_n4, assign49590_e82680_d_n5, assign49590_e82680_d_n6, assign49590_e82680_d_n7, assign49590_e82680_d_n8, assign49590_e82680_d_n9, assign49590_e82680_d_n10, assign49590_e82680_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49590_e82678: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49590_e82678, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49590_e82680;
        locals.var_t1_dn3 = assign49590_e82680_d_n3;
        locals.var_t1_dn4 = assign49590_e82680_d_n4;
        locals.var_t1_dn5 = assign49590_e82680_d_n5;
        locals.var_t1_dn6 = assign49590_e82680_d_n6;
        locals.var_t1_dn7 = assign49590_e82680_d_n7;
        locals.var_t1_dn8 = assign49590_e82680_d_n8;
        locals.var_t1_dn9 = assign49590_e82680_d_n9;
        locals.var_t1_dn10 = assign49590_e82680_d_n10;
        locals.var_t1_dn11 = assign49590_e82680_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49600_e82692, assign49600_e82692_d_n3, assign49600_e82692_d_n4, assign49600_e82692_d_n5, assign49600_e82692_d_n6, assign49600_e82692_d_n7, assign49600_e82692_d_n8, assign49600_e82692_d_n9, assign49600_e82692_d_n10, assign49600_e82692_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49600_e82690: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign49600_e82690, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49600_e82692;
        locals.var_t3_dn3 = assign49600_e82692_d_n3;
        locals.var_t3_dn4 = assign49600_e82692_d_n4;
        locals.var_t3_dn5 = assign49600_e82692_d_n5;
        locals.var_t3_dn6 = assign49600_e82692_d_n6;
        locals.var_t3_dn7 = assign49600_e82692_d_n7;
        locals.var_t3_dn8 = assign49600_e82692_d_n8;
        locals.var_t3_dn9 = assign49600_e82692_d_n9;
        locals.var_t3_dn10 = assign49600_e82692_d_n10;
        locals.var_t3_dn11 = assign49600_e82692_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49620_e82721, assign49620_e82721_d_n3, assign49620_e82721_d_n4, assign49620_e82721_d_n5, assign49620_e82721_d_n6, assign49620_e82721_d_n7, assign49620_e82721_d_n8, assign49620_e82721_d_n9, assign49620_e82721_d_n10, assign49620_e82721_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49620_e82718: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign49620_e82719: f64 = (1.0 / assign49620_e82718);
        (assign49620_e82719, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign49620_e82718 * assign49620_e82718))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign49620_e82718 * assign49620_e82718))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49620_e82721;
        locals.var_t1_dn3 = assign49620_e82721_d_n3;
        locals.var_t1_dn4 = assign49620_e82721_d_n4;
        locals.var_t1_dn5 = assign49620_e82721_d_n5;
        locals.var_t1_dn6 = assign49620_e82721_d_n6;
        locals.var_t1_dn7 = assign49620_e82721_d_n7;
        locals.var_t1_dn8 = assign49620_e82721_d_n8;
        locals.var_t1_dn9 = assign49620_e82721_d_n9;
        locals.var_t1_dn10 = assign49620_e82721_d_n10;
        locals.var_t1_dn11 = assign49620_e82721_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49630_e82739, assign49630_e82739_d_n3, assign49630_e82739_d_n4, assign49630_e82739_d_n5, assign49630_e82739_d_n6, assign49630_e82739_d_n7, assign49630_e82739_d_n8, assign49630_e82739_d_n9, assign49630_e82739_d_n10, assign49630_e82739_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49630_e82731: f64 = (-locals.var_vbs_jct);
        let assign49630_e82733: f64 = (assign49630_e82731 / locals.var_nvtm2);
        let assign49630_e82735: f64 = (assign49630_e82733 * locals.var_vtun0_i);
        let assign49630_e82737: f64 = (assign49630_e82735 * locals.var_t1);
        (assign49630_e82737, (assign49630_e82735 * locals.var_t1_dn3), ((((-((assign49630_e82731 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn4)), ((((-((assign49630_e82731 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn5)), (assign49630_e82735 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn7)), (assign49630_e82735 * locals.var_t1_dn8), (assign49630_e82735 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn10)), (assign49630_e82735 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49630_e82739;
        locals.var_t0_dn3 = assign49630_e82739_d_n3;
        locals.var_t0_dn4 = assign49630_e82739_d_n4;
        locals.var_t0_dn5 = assign49630_e82739_d_n5;
        locals.var_t0_dn6 = assign49630_e82739_d_n6;
        locals.var_t0_dn7 = assign49630_e82739_d_n7;
        locals.var_t0_dn8 = assign49630_e82739_d_n8;
        locals.var_t0_dn9 = assign49630_e82739_d_n9;
        locals.var_t0_dn10 = assign49630_e82739_d_n10;
        locals.var_t0_dn11 = assign49630_e82739_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49640_e82751, assign49640_e82751_d_n3, assign49640_e82751_d_n4, assign49640_e82751_d_n5, assign49640_e82751_d_n6, assign49640_e82751_d_n7, assign49640_e82751_d_n8, assign49640_e82751_d_n9, assign49640_e82751_d_n10, assign49640_e82751_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49640_e82749: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49640_e82749, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49640_e82751;
        locals.var_t1_dn3 = assign49640_e82751_d_n3;
        locals.var_t1_dn4 = assign49640_e82751_d_n4;
        locals.var_t1_dn5 = assign49640_e82751_d_n5;
        locals.var_t1_dn6 = assign49640_e82751_d_n6;
        locals.var_t1_dn7 = assign49640_e82751_d_n7;
        locals.var_t1_dn8 = assign49640_e82751_d_n8;
        locals.var_t1_dn9 = assign49640_e82751_d_n9;
        locals.var_t1_dn10 = assign49640_e82751_d_n10;
        locals.var_t1_dn11 = assign49640_e82751_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49650_e82764, assign49650_e82764_d_n3, assign49650_e82764_d_n4, assign49650_e82764_d_n5, assign49650_e82764_d_n6, assign49650_e82764_d_n7, assign49650_e82764_d_n8, assign49650_e82764_d_n9, assign49650_e82764_d_n10, assign49650_e82764_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49650_e82762: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign49650_e82762, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49650_e82764;
        locals.var_t3_dn3 = assign49650_e82764_d_n3;
        locals.var_t3_dn4 = assign49650_e82764_d_n4;
        locals.var_t3_dn5 = assign49650_e82764_d_n5;
        locals.var_t3_dn6 = assign49650_e82764_d_n6;
        locals.var_t3_dn7 = assign49650_e82764_d_n7;
        locals.var_t3_dn8 = assign49650_e82764_d_n8;
        locals.var_t3_dn9 = assign49650_e82764_d_n9;
        locals.var_t3_dn10 = assign49650_e82764_d_n10;
        locals.var_t3_dn11 = assign49650_e82764_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49670_e82789, assign49670_e82789_d_n4, assign49670_e82789_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49670_e82787: f64 = (p.p925 * locals.var_ntund_i);
        (assign49670_e82787, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign49670_e82789;
        locals.var_nvtm2_dn4 = assign49670_e82789_d_n4;
        locals.var_nvtm2_dn5 = assign49670_e82789_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let assign49680_e82792: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign49680_e82794: f64 = if assign49680_e82792 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign49680_e82794;
        locals.var_guard758_rv = 0.0;

        let (assign49690_e82804, assign49690_e82804_d_n3, assign49690_e82804_d_n4, assign49690_e82804_d_n5, assign49690_e82804_d_n6, assign49690_e82804_d_n7, assign49690_e82804_d_n8, assign49690_e82804_d_n9, assign49690_e82804_d_n10, assign49690_e82804_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49690_e82804;
        locals.var_t1_dn3 = assign49690_e82804_d_n3;
        locals.var_t1_dn4 = assign49690_e82804_d_n4;
        locals.var_t1_dn5 = assign49690_e82804_d_n5;
        locals.var_t1_dn6 = assign49690_e82804_d_n6;
        locals.var_t1_dn7 = assign49690_e82804_d_n7;
        locals.var_t1_dn8 = assign49690_e82804_d_n8;
        locals.var_t1_dn9 = assign49690_e82804_d_n9;
        locals.var_t1_dn10 = assign49690_e82804_d_n10;
        locals.var_t1_dn11 = assign49690_e82804_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49700_e82821, assign49700_e82821_d_n3, assign49700_e82821_d_n4, assign49700_e82821_d_n5, assign49700_e82821_d_n6, assign49700_e82821_d_n7, assign49700_e82821_d_n8, assign49700_e82821_d_n9, assign49700_e82821_d_n10, assign49700_e82821_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49700_e82813: f64 = (-locals.var_vbd_jct);
        let assign49700_e82815: f64 = (assign49700_e82813 / locals.var_nvtm2);
        let assign49700_e82817: f64 = (assign49700_e82815 * locals.var_vtun0d_i);
        let assign49700_e82819: f64 = (assign49700_e82817 * locals.var_t1);
        (assign49700_e82819, (assign49700_e82817 * locals.var_t1_dn3), ((((-((assign49700_e82813 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn4)), ((((-((assign49700_e82813 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn6)), (assign49700_e82817 * locals.var_t1_dn7), (assign49700_e82817 * locals.var_t1_dn8), (assign49700_e82817 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn10)), (assign49700_e82817 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49700_e82821;
        locals.var_t0_dn3 = assign49700_e82821_d_n3;
        locals.var_t0_dn4 = assign49700_e82821_d_n4;
        locals.var_t0_dn5 = assign49700_e82821_d_n5;
        locals.var_t0_dn6 = assign49700_e82821_d_n6;
        locals.var_t0_dn7 = assign49700_e82821_d_n7;
        locals.var_t0_dn8 = assign49700_e82821_d_n8;
        locals.var_t0_dn9 = assign49700_e82821_d_n9;
        locals.var_t0_dn10 = assign49700_e82821_d_n10;
        locals.var_t0_dn11 = assign49700_e82821_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49710_e82832, assign49710_e82832_d_n3, assign49710_e82832_d_n4, assign49710_e82832_d_n5, assign49710_e82832_d_n6, assign49710_e82832_d_n7, assign49710_e82832_d_n8, assign49710_e82832_d_n9, assign49710_e82832_d_n10, assign49710_e82832_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49710_e82830: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49710_e82830, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49710_e82832;
        locals.var_t1_dn3 = assign49710_e82832_d_n3;
        locals.var_t1_dn4 = assign49710_e82832_d_n4;
        locals.var_t1_dn5 = assign49710_e82832_d_n5;
        locals.var_t1_dn6 = assign49710_e82832_d_n6;
        locals.var_t1_dn7 = assign49710_e82832_d_n7;
        locals.var_t1_dn8 = assign49710_e82832_d_n8;
        locals.var_t1_dn9 = assign49710_e82832_d_n9;
        locals.var_t1_dn10 = assign49710_e82832_d_n10;
        locals.var_t1_dn11 = assign49710_e82832_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49720_e82844, assign49720_e82844_d_n3, assign49720_e82844_d_n4, assign49720_e82844_d_n5, assign49720_e82844_d_n6, assign49720_e82844_d_n7, assign49720_e82844_d_n8, assign49720_e82844_d_n9, assign49720_e82844_d_n10, assign49720_e82844_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49720_e82842: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign49720_e82842, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49720_e82844;
        locals.var_t3_dn3 = assign49720_e82844_d_n3;
        locals.var_t3_dn4 = assign49720_e82844_d_n4;
        locals.var_t3_dn5 = assign49720_e82844_d_n5;
        locals.var_t3_dn6 = assign49720_e82844_d_n6;
        locals.var_t3_dn7 = assign49720_e82844_d_n7;
        locals.var_t3_dn8 = assign49720_e82844_d_n8;
        locals.var_t3_dn9 = assign49720_e82844_d_n9;
        locals.var_t3_dn10 = assign49720_e82844_d_n10;
        locals.var_t3_dn11 = assign49720_e82844_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49740_e82873, assign49740_e82873_d_n3, assign49740_e82873_d_n4, assign49740_e82873_d_n5, assign49740_e82873_d_n6, assign49740_e82873_d_n7, assign49740_e82873_d_n8, assign49740_e82873_d_n9, assign49740_e82873_d_n10, assign49740_e82873_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49740_e82870: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign49740_e82871: f64 = (1.0 / assign49740_e82870);
        (assign49740_e82871, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign49740_e82870 * assign49740_e82870))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign49740_e82870 * assign49740_e82870))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49740_e82873;
        locals.var_t1_dn3 = assign49740_e82873_d_n3;
        locals.var_t1_dn4 = assign49740_e82873_d_n4;
        locals.var_t1_dn5 = assign49740_e82873_d_n5;
        locals.var_t1_dn6 = assign49740_e82873_d_n6;
        locals.var_t1_dn7 = assign49740_e82873_d_n7;
        locals.var_t1_dn8 = assign49740_e82873_d_n8;
        locals.var_t1_dn9 = assign49740_e82873_d_n9;
        locals.var_t1_dn10 = assign49740_e82873_d_n10;
        locals.var_t1_dn11 = assign49740_e82873_d_n11;
        locals.var_t1_rv = 0.0;

    }
}
