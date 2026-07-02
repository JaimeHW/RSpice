#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_198(
        locals: &mut StampLocals,
    ) {
        let (assign57180_e93299, assign57180_e93299_d_n3, assign57180_e93299_d_n4, assign57180_e93299_d_n5, assign57180_e93299_d_n6, assign57180_e93299_d_n7, assign57180_e93299_d_n8, assign57180_e93299_d_n9, assign57180_e93299_d_n10, assign57180_e93299_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57180_e93287: f64 = (locals.var_t2 - 0.201491);
        let assign57180_e93291: f64 = (locals.var_t2 + 0.402982);
        let assign57180_e93292: f64 = (locals.var_t2 * assign57180_e93291);
        let assign57180_e93294: f64 = (assign57180_e93292 + 2.446562);
        let assign57180_e93295: f64 = (assign57180_e93294).sqrt();
        let assign57180_e93296: f64 = (assign57180_e93287 - assign57180_e93295);
        let assign57180_e93297: f64 = (0.5 * assign57180_e93296);
        (assign57180_e93297, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign57180_e93295)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign57180_e93291) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign57180_e93295)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign57180_e93299;
        locals.var_t8_dn3 = assign57180_e93299_d_n3;
        locals.var_t8_dn4 = assign57180_e93299_d_n4;
        locals.var_t8_dn5 = assign57180_e93299_d_n5;
        locals.var_t8_dn6 = assign57180_e93299_d_n6;
        locals.var_t8_dn7 = assign57180_e93299_d_n7;
        locals.var_t8_dn8 = assign57180_e93299_d_n8;
        locals.var_t8_dn9 = assign57180_e93299_d_n9;
        locals.var_t8_dn10 = assign57180_e93299_d_n10;
        locals.var_t8_dn11 = assign57180_e93299_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign57190_e93306, assign57190_e93306_d_n3, assign57190_e93306_d_n4, assign57190_e93306_d_n5, assign57190_e93306_d_n6, assign57190_e93306_d_n7, assign57190_e93306_d_n8, assign57190_e93306_d_n9, assign57190_e93306_d_n10, assign57190_e93306_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign57190_e93306;
        locals.var_sqrtpsisa_dn3 = assign57190_e93306_d_n3;
        locals.var_sqrtpsisa_dn4 = assign57190_e93306_d_n4;
        locals.var_sqrtpsisa_dn5 = assign57190_e93306_d_n5;
        locals.var_sqrtpsisa_dn6 = assign57190_e93306_d_n6;
        locals.var_sqrtpsisa_dn7 = assign57190_e93306_d_n7;
        locals.var_sqrtpsisa_dn8 = assign57190_e93306_d_n8;
        locals.var_sqrtpsisa_dn9 = assign57190_e93306_d_n9;
        locals.var_sqrtpsisa_dn10 = assign57190_e93306_d_n10;
        locals.var_sqrtpsisa_dn11 = assign57190_e93306_d_n11;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign57200_e93309: f64 = (-68.0);
        let assign57200_e93310: f64 = if locals.var_t8 <= assign57200_e93309 { 1.0 } else { 0.0 };
        locals.var_guard855 = assign57200_e93310;
        locals.var_guard855_rv = 0.0;

        let (assign57210_e93320, assign57210_e93320_d_n3, assign57210_e93320_d_n4, assign57210_e93320_d_n5, assign57210_e93320_d_n6, assign57210_e93320_d_n7, assign57210_e93320_d_n8, assign57210_e93320_d_n9, assign57210_e93320_d_n10, assign57210_e93320_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign57210_e93318: f64 = (-100.0);
        (assign57210_e93318, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57210_e93320;
        locals.var_t4_dn3 = assign57210_e93320_d_n3;
        locals.var_t4_dn4 = assign57210_e93320_d_n4;
        locals.var_t4_dn5 = assign57210_e93320_d_n5;
        locals.var_t4_dn6 = assign57210_e93320_d_n6;
        locals.var_t4_dn7 = assign57210_e93320_d_n7;
        locals.var_t4_dn8 = assign57210_e93320_d_n8;
        locals.var_t4_dn9 = assign57210_e93320_d_n9;
        locals.var_t4_dn10 = assign57210_e93320_d_n10;
        locals.var_t4_dn11 = assign57210_e93320_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57220_e93329, assign57220_e93329_d_n3, assign57220_e93329_d_n4, assign57220_e93329_d_n5, assign57220_e93329_d_n6, assign57220_e93329_d_n7, assign57220_e93329_d_n8, assign57220_e93329_d_n9, assign57220_e93329_d_n10, assign57220_e93329_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57220_e93329;
        locals.var_t5_dn3 = assign57220_e93329_d_n3;
        locals.var_t5_dn4 = assign57220_e93329_d_n4;
        locals.var_t5_dn5 = assign57220_e93329_d_n5;
        locals.var_t5_dn6 = assign57220_e93329_d_n6;
        locals.var_t5_dn7 = assign57220_e93329_d_n7;
        locals.var_t5_dn8 = assign57220_e93329_d_n8;
        locals.var_t5_dn9 = assign57220_e93329_d_n9;
        locals.var_t5_dn10 = assign57220_e93329_d_n10;
        locals.var_t5_dn11 = assign57220_e93329_d_n11;
        locals.var_t5_rv = 0.0;

        let assign57230_e93334: f64 = (0.5 * locals.var_t5);
        let assign57230_e93335: f64 = (locals.var_t4 - assign57230_e93334);
        let assign57230_e93336: f64 = if locals.var_t8 < assign57230_e93335 { 1.0 } else { 0.0 };
        locals.var_guard856 = assign57230_e93336;
        locals.var_guard856_rv = 0.0;

        let (assign57240_e93348, assign57240_e93348_d_n3, assign57240_e93348_d_n4, assign57240_e93348_d_n5, assign57240_e93348_d_n6, assign57240_e93348_d_n7, assign57240_e93348_d_n8, assign57240_e93348_d_n9, assign57240_e93348_d_n10, assign57240_e93348_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 != 0.0)) {
        let assign57240_e93346: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57240_e93346, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57240_e93348;
        locals.var_t3_dn3 = assign57240_e93348_d_n3;
        locals.var_t3_dn4 = assign57240_e93348_d_n4;
        locals.var_t3_dn5 = assign57240_e93348_d_n5;
        locals.var_t3_dn6 = assign57240_e93348_d_n6;
        locals.var_t3_dn7 = assign57240_e93348_d_n7;
        locals.var_t3_dn8 = assign57240_e93348_d_n8;
        locals.var_t3_dn9 = assign57240_e93348_d_n9;
        locals.var_t3_dn10 = assign57240_e93348_d_n10;
        locals.var_t3_dn11 = assign57240_e93348_d_n11;
        locals.var_t3_rv = 0.0;

        let assign57250_e93353: f64 = (0.5 * locals.var_t5);
        let assign57250_e93354: f64 = (locals.var_t4 + assign57250_e93353);
        let assign57250_e93355: f64 = if locals.var_t8 > assign57250_e93354 { 1.0 } else { 0.0 };
        locals.var_guard857 = assign57250_e93355;
        locals.var_guard857_rv = 0.0;

        let (assign57260_e93370, assign57260_e93370_d_n3, assign57260_e93370_d_n4, assign57260_e93370_d_n5, assign57260_e93370_d_n6, assign57260_e93370_d_n7, assign57260_e93370_d_n8, assign57260_e93370_d_n9, assign57260_e93370_d_n10, assign57260_e93370_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) && (locals.var_guard857 != 0.0)) {
        let assign57260_e93368: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57260_e93368, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57260_e93370;
        locals.var_t3_dn3 = assign57260_e93370_d_n3;
        locals.var_t3_dn4 = assign57260_e93370_d_n4;
        locals.var_t3_dn5 = assign57260_e93370_d_n5;
        locals.var_t3_dn6 = assign57260_e93370_d_n6;
        locals.var_t3_dn7 = assign57260_e93370_d_n7;
        locals.var_t3_dn8 = assign57260_e93370_d_n8;
        locals.var_t3_dn9 = assign57260_e93370_d_n9;
        locals.var_t3_dn10 = assign57260_e93370_d_n10;
        locals.var_t3_dn11 = assign57260_e93370_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57270_e93389, assign57270_e93389_d_n3, assign57270_e93389_d_n4, assign57270_e93389_d_n5, assign57270_e93389_d_n6, assign57270_e93389_d_n7, assign57270_e93389_d_n8, assign57270_e93389_d_n9, assign57270_e93389_d_n10, assign57270_e93389_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign57270_e93385: f64 = (locals.var_t8 - locals.var_t4);
        let assign57270_e93387: f64 = (assign57270_e93385 / locals.var_t5);
        (assign57270_e93387, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign57270_e93385 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign57270_e93389;
        locals.var_t2_dn3 = assign57270_e93389_d_n3;
        locals.var_t2_dn4 = assign57270_e93389_d_n4;
        locals.var_t2_dn5 = assign57270_e93389_d_n5;
        locals.var_t2_dn6 = assign57270_e93389_d_n6;
        locals.var_t2_dn7 = assign57270_e93389_d_n7;
        locals.var_t2_dn8 = assign57270_e93389_d_n8;
        locals.var_t2_dn9 = assign57270_e93389_d_n9;
        locals.var_t2_dn10 = assign57270_e93389_d_n10;
        locals.var_t2_dn11 = assign57270_e93389_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign57280_e93406, assign57280_e93406_d_n3, assign57280_e93406_d_n4, assign57280_e93406_d_n5, assign57280_e93406_d_n6, assign57280_e93406_d_n7, assign57280_e93406_d_n8, assign57280_e93406_d_n9, assign57280_e93406_d_n10, assign57280_e93406_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign57280_e93404: f64 = (locals.var_t2 * locals.var_t2);
        (assign57280_e93404, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign57280_e93406;
        locals.var_t6_dn3 = assign57280_e93406_d_n3;
        locals.var_t6_dn4 = assign57280_e93406_d_n4;
        locals.var_t6_dn5 = assign57280_e93406_d_n5;
        locals.var_t6_dn6 = assign57280_e93406_d_n6;
        locals.var_t6_dn7 = assign57280_e93406_d_n7;
        locals.var_t6_dn8 = assign57280_e93406_d_n8;
        locals.var_t6_dn9 = assign57280_e93406_d_n9;
        locals.var_t6_dn10 = assign57280_e93406_d_n10;
        locals.var_t6_dn11 = assign57280_e93406_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign57290_e93444, assign57290_e93444_d_n3, assign57290_e93444_d_n4, assign57290_e93444_d_n5, assign57290_e93444_d_n6, assign57290_e93444_d_n7, assign57290_e93444_d_n8, assign57290_e93444_d_n9, assign57290_e93444_d_n10, assign57290_e93444_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) && (locals.var_guard856 == 0.0)) && (locals.var_guard857 == 0.0)) {
        let assign57290_e93423: f64 = (5.0 / 64.0);
        let assign57290_e93426: f64 = (0.5 * locals.var_t2);
        let assign57290_e93427: f64 = (assign57290_e93423 + assign57290_e93426);
        let assign57290_e93431: f64 = (15.0 / 16.0);
        let assign57290_e93435: f64 = (1.25 - locals.var_t6);
        let assign57290_e93436: f64 = (locals.var_t6 * assign57290_e93435);
        let assign57290_e93437: f64 = (assign57290_e93431 - assign57290_e93436);
        let assign57290_e93438: f64 = (locals.var_t6 * assign57290_e93437);
        let assign57290_e93439: f64 = (assign57290_e93427 + assign57290_e93438);
        let assign57290_e93440: f64 = (locals.var_t5 * assign57290_e93439);
        let assign57290_e93441: f64 = (locals.var_t4 + assign57290_e93440);
        let assign57290_e93442: f64 = { let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57290_e93442, ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign57290_e93441; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign57290_e93439) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign57290_e93437) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign57290_e93435) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57290_e93444;
        locals.var_t3_dn3 = assign57290_e93444_d_n3;
        locals.var_t3_dn4 = assign57290_e93444_d_n4;
        locals.var_t3_dn5 = assign57290_e93444_d_n5;
        locals.var_t3_dn6 = assign57290_e93444_d_n6;
        locals.var_t3_dn7 = assign57290_e93444_d_n7;
        locals.var_t3_dn8 = assign57290_e93444_d_n8;
        locals.var_t3_dn9 = assign57290_e93444_d_n9;
        locals.var_t3_dn10 = assign57290_e93444_d_n10;
        locals.var_t3_dn11 = assign57290_e93444_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57300_e93476, assign57300_e93476_d_n3, assign57300_e93476_d_n4, assign57300_e93476_d_n5, assign57300_e93476_d_n6, assign57300_e93476_d_n7, assign57300_e93476_d_n8, assign57300_e93476_d_n9, assign57300_e93476_d_n10, assign57300_e93476_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 != 0.0)) {
        let assign57300_e93454: f64 = (1.0 + locals.var_t1);
        let assign57300_e93456: f64 = (assign57300_e93454 - locals.var_t8);
        let assign57300_e93459: f64 = (2.0 * locals.var_t0);
        let assign57300_e93462: f64 = (locals.var_t3 * 2.0);
        let assign57300_e93464: f64 = (assign57300_e93462 * locals.var_t0);
        let assign57300_e93467: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign57300_e93468: f64 = (assign57300_e93464 + assign57300_e93467);
        let assign57300_e93469: f64 = (assign57300_e93459 * assign57300_e93468);
        let assign57300_e93471: f64 = (assign57300_e93469).max(1e-38);
        let assign57300_e93472: f64 = (assign57300_e93471).ln();
        let assign57300_e93473: f64 = (assign57300_e93456 - assign57300_e93472);
        let assign57300_e93474: f64 = (locals.var_t3 * assign57300_e93473);
        (assign57300_e93474, ((locals.var_t3_dn3 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn4 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn5 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn6 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn7 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn8 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn9 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn10 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign57300_e93471)))), ((locals.var_t3_dn11 * assign57300_e93473) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign57300_e93469 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign57300_e93468) + (assign57300_e93459 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57300_e93462 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign57300_e93471)))),)
    } else {
        (locals.var_qs_1, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11,)
    }
};
        locals.var_qs_1 = assign57300_e93476;
        locals.var_qs_1_dn3 = assign57300_e93476_d_n3;
        locals.var_qs_1_dn4 = assign57300_e93476_d_n4;
        locals.var_qs_1_dn5 = assign57300_e93476_d_n5;
        locals.var_qs_1_dn6 = assign57300_e93476_d_n6;
        locals.var_qs_1_dn7 = assign57300_e93476_d_n7;
        locals.var_qs_1_dn8 = assign57300_e93476_d_n8;
        locals.var_qs_1_dn9 = assign57300_e93476_d_n9;
        locals.var_qs_1_dn10 = assign57300_e93476_d_n10;
        locals.var_qs_1_dn11 = assign57300_e93476_d_n11;
        locals.var_qs_1_rv = 0.0;

        let (assign57310_e93487, assign57310_e93487_d_n3, assign57310_e93487_d_n4, assign57310_e93487_d_n5, assign57310_e93487_d_n6, assign57310_e93487_d_n7, assign57310_e93487_d_n8, assign57310_e93487_d_n9, assign57310_e93487_d_n10, assign57310_e93487_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57310_e93485: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57310_e93485, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57310_e93487;
        locals.var_t3_dn3 = assign57310_e93487_d_n3;
        locals.var_t3_dn4 = assign57310_e93487_d_n4;
        locals.var_t3_dn5 = assign57310_e93487_d_n5;
        locals.var_t3_dn6 = assign57310_e93487_d_n6;
        locals.var_t3_dn7 = assign57310_e93487_d_n7;
        locals.var_t3_dn8 = assign57310_e93487_d_n8;
        locals.var_t3_dn9 = assign57310_e93487_d_n9;
        locals.var_t3_dn10 = assign57310_e93487_d_n10;
        locals.var_t3_dn11 = assign57310_e93487_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57320_e93499, assign57320_e93499_d_n3, assign57320_e93499_d_n4, assign57320_e93499_d_n5, assign57320_e93499_d_n6, assign57320_e93499_d_n7, assign57320_e93499_d_n8, assign57320_e93499_d_n9, assign57320_e93499_d_n10, assign57320_e93499_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57320_e93497: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign57320_e93497, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign57320_e93499;
        locals.var_sqrtpsisainv_dn3 = assign57320_e93499_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign57320_e93499_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign57320_e93499_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign57320_e93499_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign57320_e93499_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign57320_e93499_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign57320_e93499_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign57320_e93499_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign57320_e93499_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign57330_e93532, assign57330_e93532_d_n3, assign57330_e93532_d_n4, assign57330_e93532_d_n5, assign57330_e93532_d_n6, assign57330_e93532_d_n7, assign57330_e93532_d_n8, assign57330_e93532_d_n9, assign57330_e93532_d_n10, assign57330_e93532_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57330_e93509: f64 = (2.0 * locals.var_t3);
        let assign57330_e93512: f64 = (locals.var_t3 * 2.0);
        let assign57330_e93514: f64 = (assign57330_e93512 * locals.var_t0);
        let assign57330_e93517: f64 = (locals.var_t3 * 2.0);
        let assign57330_e93519: f64 = (assign57330_e93517 * locals.var_t0);
        let assign57330_e93522: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign57330_e93523: f64 = (assign57330_e93519 + assign57330_e93522);
        let assign57330_e93524: f64 = (assign57330_e93514 * assign57330_e93523);
        let assign57330_e93526: f64 = (assign57330_e93524).max(1e-38);
        let assign57330_e93527: f64 = (assign57330_e93526).ln();
        let assign57330_e93528: f64 = (assign57330_e93509 + assign57330_e93527);
        let assign57330_e93530: f64 = (assign57330_e93528 - locals.var_t1);
        (assign57330_e93530, (((2.0 * locals.var_t3_dn3) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn3)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn4)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn5)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn6)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn7)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn8)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn9)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn10)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign57330_e93524 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57330_e93512 * locals.var_t0_dn11)) * assign57330_e93523) + (assign57330_e93514 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57330_e93517 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign57330_e93526)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57330_e93532;
        locals.var_t4_dn3 = assign57330_e93532_d_n3;
        locals.var_t4_dn4 = assign57330_e93532_d_n4;
        locals.var_t4_dn5 = assign57330_e93532_d_n5;
        locals.var_t4_dn6 = assign57330_e93532_d_n6;
        locals.var_t4_dn7 = assign57330_e93532_d_n7;
        locals.var_t4_dn8 = assign57330_e93532_d_n8;
        locals.var_t4_dn9 = assign57330_e93532_d_n9;
        locals.var_t4_dn10 = assign57330_e93532_d_n10;
        locals.var_t4_dn11 = assign57330_e93532_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57340_e93556, assign57340_e93556_d_n3, assign57340_e93556_d_n4, assign57340_e93556_d_n5, assign57340_e93556_d_n6, assign57340_e93556_d_n7, assign57340_e93556_d_n8, assign57340_e93556_d_n9, assign57340_e93556_d_n10, assign57340_e93556_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57340_e93543: f64 = (1.0 / locals.var_t3);
        let assign57340_e93544: f64 = (2.0 + assign57340_e93543);
        let assign57340_e93547: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57340_e93550: f64 = (locals.var_t0 * locals.var_t3);
        let assign57340_e93552: f64 = (assign57340_e93550 + locals.var_sqrtpsisa);
        let assign57340_e93553: f64 = (assign57340_e93547 / assign57340_e93552);
        let assign57340_e93554: f64 = (assign57340_e93544 + assign57340_e93553);
        (assign57340_e93554, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57340_e93552 * assign57340_e93552))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57340_e93552) - (assign57340_e93547 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57340_e93552 * assign57340_e93552))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57340_e93556;
        locals.var_t5_dn3 = assign57340_e93556_d_n3;
        locals.var_t5_dn4 = assign57340_e93556_d_n4;
        locals.var_t5_dn5 = assign57340_e93556_d_n5;
        locals.var_t5_dn6 = assign57340_e93556_d_n6;
        locals.var_t5_dn7 = assign57340_e93556_d_n7;
        locals.var_t5_dn8 = assign57340_e93556_d_n8;
        locals.var_t5_dn9 = assign57340_e93556_d_n9;
        locals.var_t5_dn10 = assign57340_e93556_d_n10;
        locals.var_t5_dn11 = assign57340_e93556_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign57350_e93570, assign57350_e93570_d_n3, assign57350_e93570_d_n4, assign57350_e93570_d_n5, assign57350_e93570_d_n6, assign57350_e93570_d_n7, assign57350_e93570_d_n8, assign57350_e93570_d_n9, assign57350_e93570_d_n10, assign57350_e93570_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57350_e93567: f64 = (locals.var_t4 / locals.var_t5);
        let assign57350_e93568: f64 = (locals.var_t3 - assign57350_e93567);
        (assign57350_e93568, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57350_e93570;
        locals.var_t3_dn3 = assign57350_e93570_d_n3;
        locals.var_t3_dn4 = assign57350_e93570_d_n4;
        locals.var_t3_dn5 = assign57350_e93570_d_n5;
        locals.var_t3_dn6 = assign57350_e93570_d_n6;
        locals.var_t3_dn7 = assign57350_e93570_d_n7;
        locals.var_t3_dn8 = assign57350_e93570_d_n8;
        locals.var_t3_dn9 = assign57350_e93570_d_n9;
        locals.var_t3_dn10 = assign57350_e93570_d_n10;
        locals.var_t3_dn11 = assign57350_e93570_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57360_e93603, assign57360_e93603_d_n3, assign57360_e93603_d_n4, assign57360_e93603_d_n5, assign57360_e93603_d_n6, assign57360_e93603_d_n7, assign57360_e93603_d_n8, assign57360_e93603_d_n9, assign57360_e93603_d_n10, assign57360_e93603_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57360_e93580: f64 = (2.0 * locals.var_t3);
        let assign57360_e93583: f64 = (locals.var_t3 * 2.0);
        let assign57360_e93585: f64 = (assign57360_e93583 * locals.var_t0);
        let assign57360_e93588: f64 = (locals.var_t3 * 2.0);
        let assign57360_e93590: f64 = (assign57360_e93588 * locals.var_t0);
        let assign57360_e93593: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign57360_e93594: f64 = (assign57360_e93590 + assign57360_e93593);
        let assign57360_e93595: f64 = (assign57360_e93585 * assign57360_e93594);
        let assign57360_e93597: f64 = (assign57360_e93595).max(1e-38);
        let assign57360_e93598: f64 = (assign57360_e93597).ln();
        let assign57360_e93599: f64 = (assign57360_e93580 + assign57360_e93598);
        let assign57360_e93601: f64 = (assign57360_e93599 - locals.var_t1);
        (assign57360_e93601, (((2.0 * locals.var_t3_dn3) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn3)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn4)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn5)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn6)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn7)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn8)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn9)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn10)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign57360_e93595 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57360_e93583 * locals.var_t0_dn11)) * assign57360_e93594) + (assign57360_e93585 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57360_e93588 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign57360_e93597)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57360_e93603;
        locals.var_t4_dn3 = assign57360_e93603_d_n3;
        locals.var_t4_dn4 = assign57360_e93603_d_n4;
        locals.var_t4_dn5 = assign57360_e93603_d_n5;
        locals.var_t4_dn6 = assign57360_e93603_d_n6;
        locals.var_t4_dn7 = assign57360_e93603_d_n7;
        locals.var_t4_dn8 = assign57360_e93603_d_n8;
        locals.var_t4_dn9 = assign57360_e93603_d_n9;
        locals.var_t4_dn10 = assign57360_e93603_d_n10;
        locals.var_t4_dn11 = assign57360_e93603_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57370_e93627, assign57370_e93627_d_n3, assign57370_e93627_d_n4, assign57370_e93627_d_n5, assign57370_e93627_d_n6, assign57370_e93627_d_n7, assign57370_e93627_d_n8, assign57370_e93627_d_n9, assign57370_e93627_d_n10, assign57370_e93627_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57370_e93614: f64 = (1.0 / locals.var_t3);
        let assign57370_e93615: f64 = (2.0 + assign57370_e93614);
        let assign57370_e93618: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57370_e93621: f64 = (locals.var_t0 * locals.var_t3);
        let assign57370_e93623: f64 = (assign57370_e93621 + locals.var_sqrtpsisa);
        let assign57370_e93624: f64 = (assign57370_e93618 / assign57370_e93623);
        let assign57370_e93625: f64 = (assign57370_e93615 + assign57370_e93624);
        (assign57370_e93625, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57370_e93623 * assign57370_e93623))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57370_e93623) - (assign57370_e93618 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57370_e93623 * assign57370_e93623))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57370_e93627;
        locals.var_t5_dn3 = assign57370_e93627_d_n3;
        locals.var_t5_dn4 = assign57370_e93627_d_n4;
        locals.var_t5_dn5 = assign57370_e93627_d_n5;
        locals.var_t5_dn6 = assign57370_e93627_d_n6;
        locals.var_t5_dn7 = assign57370_e93627_d_n7;
        locals.var_t5_dn8 = assign57370_e93627_d_n8;
        locals.var_t5_dn9 = assign57370_e93627_d_n9;
        locals.var_t5_dn10 = assign57370_e93627_d_n10;
        locals.var_t5_dn11 = assign57370_e93627_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign57380_e93655, assign57380_e93655_d_n3, assign57380_e93655_d_n4, assign57380_e93655_d_n5, assign57380_e93655_d_n6, assign57380_e93655_d_n7, assign57380_e93655_d_n8, assign57380_e93655_d_n9, assign57380_e93655_d_n10, assign57380_e93655_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57380_e93637: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57380_e93640: f64 = (locals.var_t0 * locals.var_t3);
        let assign57380_e93642: f64 = (assign57380_e93640 + locals.var_sqrtpsisa);
        let assign57380_e93643: f64 = (assign57380_e93637 / assign57380_e93642);
        let assign57380_e93646: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57380_e93649: f64 = (locals.var_t0 * locals.var_t3);
        let assign57380_e93651: f64 = (assign57380_e93649 + locals.var_sqrtpsisa);
        let assign57380_e93652: f64 = (assign57380_e93646 / assign57380_e93651);
        let assign57380_e93653: f64 = (assign57380_e93643 * assign57380_e93652);
        (assign57380_e93653, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57380_e93651 * assign57380_e93651)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57380_e93642) - (assign57380_e93637 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57380_e93642 * assign57380_e93642)) * assign57380_e93652) + (assign57380_e93643 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57380_e93651) - (assign57380_e93646 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57380_e93651 * assign57380_e93651)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign57380_e93655;
        locals.var_t6_dn3 = assign57380_e93655_d_n3;
        locals.var_t6_dn4 = assign57380_e93655_d_n4;
        locals.var_t6_dn5 = assign57380_e93655_d_n5;
        locals.var_t6_dn6 = assign57380_e93655_d_n6;
        locals.var_t6_dn7 = assign57380_e93655_d_n7;
        locals.var_t6_dn8 = assign57380_e93655_d_n8;
        locals.var_t6_dn9 = assign57380_e93655_d_n9;
        locals.var_t6_dn10 = assign57380_e93655_d_n10;
        locals.var_t6_dn11 = assign57380_e93655_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign57390_e93688, assign57390_e93688_d_n3, assign57390_e93688_d_n4, assign57390_e93688_d_n5, assign57390_e93688_d_n6, assign57390_e93688_d_n7, assign57390_e93688_d_n8, assign57390_e93688_d_n9, assign57390_e93688_d_n10, assign57390_e93688_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign57390_e93665: f64 = (1.0 * __rspice_inv_cse_0);
        let assign57390_e93668: f64 = (1.0 * __rspice_inv_cse_0);
        let assign57390_e93669: f64 = (assign57390_e93665 * assign57390_e93668);
        let assign57390_e93670: f64 = (-assign57390_e93669);
        let assign57390_e93674: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign57390_e93676: f64 = (assign57390_e93674 * locals.var_sqrtpsisa);
        let assign57390_e93679: f64 = (locals.var_t0 * locals.var_t3);
        let assign57390_e93681: f64 = (assign57390_e93679 + locals.var_sqrtpsisa);
        let assign57390_e93682: f64 = (assign57390_e93676 * assign57390_e93681);
        let assign57390_e93683: f64 = (1.0 / assign57390_e93682);
        let assign57390_e93684: f64 = (assign57390_e93670 - assign57390_e93683);
        let assign57390_e93686: f64 = (assign57390_e93684 - locals.var_t6);
        (assign57390_e93686, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn3)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn4)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn5)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn6)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn7)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn8)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn9)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn10)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign57390_e93668) + (assign57390_e93665 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign57390_e93674 * locals.var_sqrtpsisa_dn11)) * assign57390_e93681) + (assign57390_e93676 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57390_e93682 * assign57390_e93682)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign57390_e93688;
        locals.var_t7_dn3 = assign57390_e93688_d_n3;
        locals.var_t7_dn4 = assign57390_e93688_d_n4;
        locals.var_t7_dn5 = assign57390_e93688_d_n5;
        locals.var_t7_dn6 = assign57390_e93688_d_n6;
        locals.var_t7_dn7 = assign57390_e93688_d_n7;
        locals.var_t7_dn8 = assign57390_e93688_d_n8;
        locals.var_t7_dn9 = assign57390_e93688_d_n9;
        locals.var_t7_dn10 = assign57390_e93688_d_n10;
        locals.var_t7_dn11 = assign57390_e93688_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign57400_e93714, assign57400_e93714_d_n3, assign57400_e93714_d_n4, assign57400_e93714_d_n5, assign57400_e93714_d_n6, assign57400_e93714_d_n7, assign57400_e93714_d_n8, assign57400_e93714_d_n9, assign57400_e93714_d_n10, assign57400_e93714_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard855 == 0.0)) {
        let assign57400_e93699: f64 = (locals.var_t4 / locals.var_t5);
        let assign57400_e93703: f64 = (locals.var_t4 * locals.var_t7);
        let assign57400_e93706: f64 = (2.0 * locals.var_t5);
        let assign57400_e93708: f64 = (assign57400_e93706 * locals.var_t5);
        let assign57400_e93709: f64 = (assign57400_e93703 / assign57400_e93708);
        let assign57400_e93710: f64 = (1.0 + assign57400_e93709);
        let assign57400_e93711: f64 = (assign57400_e93699 * assign57400_e93710);
        let assign57400_e93712: f64 = (locals.var_t3 - assign57400_e93711);
        (assign57400_e93712, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn3)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn4)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn5)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn6)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn7)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn8)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn9)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn10)))) / (assign57400_e93708 * assign57400_e93708))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign57400_e93710) + (assign57400_e93699 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign57400_e93708) - (assign57400_e93703 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign57400_e93706 * locals.var_t5_dn11)))) / (assign57400_e93708 * assign57400_e93708))))),)
    } else {
        (locals.var_qs_1, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11,)
    }
};
        locals.var_qs_1 = assign57400_e93714;
        locals.var_qs_1_dn3 = assign57400_e93714_d_n3;
        locals.var_qs_1_dn4 = assign57400_e93714_d_n4;
        locals.var_qs_1_dn5 = assign57400_e93714_d_n5;
        locals.var_qs_1_dn6 = assign57400_e93714_d_n6;
        locals.var_qs_1_dn7 = assign57400_e93714_d_n7;
        locals.var_qs_1_dn8 = assign57400_e93714_d_n8;
        locals.var_qs_1_dn9 = assign57400_e93714_d_n9;
        locals.var_qs_1_dn10 = assign57400_e93714_d_n10;
        locals.var_qs_1_dn11 = assign57400_e93714_d_n11;
        locals.var_qs_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_199(
        locals: &mut StampLocals,
    ) {
        let (assign57410_e93740, assign57410_e93740_d_n3, assign57410_e93740_d_n4, assign57410_e93740_d_n5, assign57410_e93740_d_n6, assign57410_e93740_d_n7, assign57410_e93740_d_n8, assign57410_e93740_d_n9, assign57410_e93740_d_n10, assign57410_e93740_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57410_e93722: f64 = (locals.var_psip + 1.0);
        let assign57410_e93725: f64 = (locals.var_psip - 1.0);
        let assign57410_e93728: f64 = (locals.var_psip - 1.0);
        let assign57410_e93729: f64 = (assign57410_e93725 * assign57410_e93728);
        let assign57410_e93732: f64 = (0.25 * 2.0);
        let assign57410_e93734: f64 = (assign57410_e93732 * 2.0);
        let assign57410_e93735: f64 = (assign57410_e93729 + assign57410_e93734);
        let assign57410_e93736: f64 = (assign57410_e93735).sqrt();
        let assign57410_e93737: f64 = (assign57410_e93722 + assign57410_e93736);
        let assign57410_e93738: f64 = (0.5 * assign57410_e93737);
        (assign57410_e93738, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn3)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn4)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn5)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn6)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn7)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn8)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn9)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn10)) / (2.0 * assign57410_e93736)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign57410_e93728) + (assign57410_e93725 * locals.var_psip_dn11)) / (2.0 * assign57410_e93736)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11,)
    }
};
        locals.var_psipclamp = assign57410_e93740;
        locals.var_psipclamp_dn3 = assign57410_e93740_d_n3;
        locals.var_psipclamp_dn4 = assign57410_e93740_d_n4;
        locals.var_psipclamp_dn5 = assign57410_e93740_d_n5;
        locals.var_psipclamp_dn6 = assign57410_e93740_d_n6;
        locals.var_psipclamp_dn7 = assign57410_e93740_d_n7;
        locals.var_psipclamp_dn8 = assign57410_e93740_d_n8;
        locals.var_psipclamp_dn9 = assign57410_e93740_d_n9;
        locals.var_psipclamp_dn10 = assign57410_e93740_d_n10;
        locals.var_psipclamp_dn11 = assign57410_e93740_d_n11;
        locals.var_psipclamp_rv = 0.0;

        let (assign57420_e93748, assign57420_e93748_d_n3, assign57420_e93748_d_n4, assign57420_e93748_d_n5, assign57420_e93748_d_n6, assign57420_e93748_d_n7, assign57420_e93748_d_n8, assign57420_e93748_d_n9, assign57420_e93748_d_n10, assign57420_e93748_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57420_e93746: f64 = (locals.var_psipclamp).sqrt();
        (assign57420_e93746, (locals.var_psipclamp_dn3 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn4 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn5 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn6 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn7 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn8 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn9 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn10 / (2.0 * assign57420_e93746)), (locals.var_psipclamp_dn11 / (2.0 * assign57420_e93746)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign57420_e93748;
        locals.var_sqrtpsip_dn3 = assign57420_e93748_d_n3;
        locals.var_sqrtpsip_dn4 = assign57420_e93748_d_n4;
        locals.var_sqrtpsip_dn5 = assign57420_e93748_d_n5;
        locals.var_sqrtpsip_dn6 = assign57420_e93748_d_n6;
        locals.var_sqrtpsip_dn7 = assign57420_e93748_d_n7;
        locals.var_sqrtpsip_dn8 = assign57420_e93748_d_n8;
        locals.var_sqrtpsip_dn9 = assign57420_e93748_d_n9;
        locals.var_sqrtpsip_dn10 = assign57420_e93748_d_n10;
        locals.var_sqrtpsip_dn11 = assign57420_e93748_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign57430_e93759, assign57430_e93759_d_n3, assign57430_e93759_d_n4, assign57430_e93759_d_n5, assign57430_e93759_d_n6, assign57430_e93759_d_n7, assign57430_e93759_d_n8, assign57430_e93759_d_n9, assign57430_e93759_d_n10, assign57430_e93759_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57430_e93756: f64 = (2.0 * locals.var_qs_1);
        let assign57430_e93757: f64 = (locals.var_psip - assign57430_e93756);
        (assign57430_e93757, (locals.var_psip_dn3 - (2.0 * locals.var_qs_1_dn3)), (locals.var_psip_dn4 - (2.0 * locals.var_qs_1_dn4)), (locals.var_psip_dn5 - (2.0 * locals.var_qs_1_dn5)), (locals.var_psip_dn6 - (2.0 * locals.var_qs_1_dn6)), (locals.var_psip_dn7 - (2.0 * locals.var_qs_1_dn7)), (locals.var_psip_dn8 - (2.0 * locals.var_qs_1_dn8)), (locals.var_psip_dn9 - (2.0 * locals.var_qs_1_dn9)), (locals.var_psip_dn10 - (2.0 * locals.var_qs_1_dn10)), (locals.var_psip_dn11 - (2.0 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign57430_e93759;
        locals.var_psiavg_dn3 = assign57430_e93759_d_n3;
        locals.var_psiavg_dn4 = assign57430_e93759_d_n4;
        locals.var_psiavg_dn5 = assign57430_e93759_d_n5;
        locals.var_psiavg_dn6 = assign57430_e93759_d_n6;
        locals.var_psiavg_dn7 = assign57430_e93759_d_n7;
        locals.var_psiavg_dn8 = assign57430_e93759_d_n8;
        locals.var_psiavg_dn9 = assign57430_e93759_d_n9;
        locals.var_psiavg_dn10 = assign57430_e93759_d_n10;
        locals.var_psiavg_dn11 = assign57430_e93759_d_n11;
        locals.var_psiavg_rv = 0.0;

        let (assign57440_e93785, assign57440_e93785_d_n3, assign57440_e93785_d_n4, assign57440_e93785_d_n5, assign57440_e93785_d_n6, assign57440_e93785_d_n7, assign57440_e93785_d_n8, assign57440_e93785_d_n9, assign57440_e93785_d_n10, assign57440_e93785_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57440_e93767: f64 = (locals.var_psiavg + 1.0);
        let assign57440_e93770: f64 = (locals.var_psiavg - 1.0);
        let assign57440_e93773: f64 = (locals.var_psiavg - 1.0);
        let assign57440_e93774: f64 = (assign57440_e93770 * assign57440_e93773);
        let assign57440_e93777: f64 = (0.25 * 2.0);
        let assign57440_e93779: f64 = (assign57440_e93777 * 2.0);
        let assign57440_e93780: f64 = (assign57440_e93774 + assign57440_e93779);
        let assign57440_e93781: f64 = (assign57440_e93780).sqrt();
        let assign57440_e93782: f64 = (assign57440_e93767 + assign57440_e93781);
        let assign57440_e93783: f64 = (0.5 * assign57440_e93782);
        (assign57440_e93783, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn3)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn4)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn5)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn6)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn7)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn8)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn9)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn10)) / (2.0 * assign57440_e93781)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign57440_e93773) + (assign57440_e93770 * locals.var_psiavg_dn11)) / (2.0 * assign57440_e93781)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign57440_e93785;
        locals.var_t0_dn3 = assign57440_e93785_d_n3;
        locals.var_t0_dn4 = assign57440_e93785_d_n4;
        locals.var_t0_dn5 = assign57440_e93785_d_n5;
        locals.var_t0_dn6 = assign57440_e93785_d_n6;
        locals.var_t0_dn7 = assign57440_e93785_d_n7;
        locals.var_t0_dn8 = assign57440_e93785_d_n8;
        locals.var_t0_dn9 = assign57440_e93785_d_n9;
        locals.var_t0_dn10 = assign57440_e93785_d_n10;
        locals.var_t0_dn11 = assign57440_e93785_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign57450_e93799, assign57450_e93799_d_n3, assign57450_e93799_d_n4, assign57450_e93799_d_n5, assign57450_e93799_d_n6, assign57450_e93799_d_n7, assign57450_e93799_d_n8, assign57450_e93799_d_n9, assign57450_e93799_d_n10, assign57450_e93799_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57450_e93794: f64 = (locals.var_t0).sqrt();
        let assign57450_e93795: f64 = (locals.var_sqrtpsip + assign57450_e93794);
        let assign57450_e93796: f64 = (locals.var_gamagbcp2 / assign57450_e93795);
        let assign57450_e93797: f64 = (1.0 + assign57450_e93796);
        (assign57450_e93797, (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn3 + (locals.var_t0_dn3 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))), (((locals.var_gamagbcp2_dn4 * assign57450_e93795) - (locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn4 + (locals.var_t0_dn4 / (2.0 * assign57450_e93794))))) / (assign57450_e93795 * assign57450_e93795)), (((locals.var_gamagbcp2_dn5 * assign57450_e93795) - (locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn5 + (locals.var_t0_dn5 / (2.0 * assign57450_e93794))))) / (assign57450_e93795 * assign57450_e93795)), (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn6 + (locals.var_t0_dn6 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))), (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn7 + (locals.var_t0_dn7 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))), (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn8 + (locals.var_t0_dn8 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))), (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn9 + (locals.var_t0_dn9 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))), (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn10 + (locals.var_t0_dn10 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))), (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn11 + (locals.var_t0_dn11 / (2.0 * assign57450_e93794)))) / (assign57450_e93795 * assign57450_e93795))),)
    } else {
        (locals.var_nq, locals.var_nq_dn3, locals.var_nq_dn4, locals.var_nq_dn5, locals.var_nq_dn6, locals.var_nq_dn7, locals.var_nq_dn8, locals.var_nq_dn9, locals.var_nq_dn10, locals.var_nq_dn11,)
    }
};
        locals.var_nq = assign57450_e93799;
        locals.var_nq_dn3 = assign57450_e93799_d_n3;
        locals.var_nq_dn4 = assign57450_e93799_d_n4;
        locals.var_nq_dn5 = assign57450_e93799_d_n5;
        locals.var_nq_dn6 = assign57450_e93799_d_n6;
        locals.var_nq_dn7 = assign57450_e93799_d_n7;
        locals.var_nq_dn8 = assign57450_e93799_d_n8;
        locals.var_nq_dn9 = assign57450_e93799_d_n9;
        locals.var_nq_dn10 = assign57450_e93799_d_n10;
        locals.var_nq_dn11 = assign57450_e93799_d_n11;
        locals.var_nq_rv = 0.0;

        let (assign57460_e93818, assign57460_e93818_d_n3, assign57460_e93818_d_n4, assign57460_e93818_d_n5, assign57460_e93818_d_n6, assign57460_e93818_d_n7, assign57460_e93818_d_n8, assign57460_e93818_d_n9, assign57460_e93818_d_n10, assign57460_e93818_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57460_e93807: f64 = (locals.var_vgfbagbcp2 - locals.var_psip);
        let assign57460_e93810: f64 = (2.0 * locals.var_qs_1);
        let assign57460_e93813: f64 = (locals.var_nq - 1.0);
        let assign57460_e93814: f64 = (assign57460_e93810 * assign57460_e93813);
        let assign57460_e93815: f64 = (assign57460_e93807 - assign57460_e93814);
        let assign57460_e93816: f64 = (locals.var_vt * assign57460_e93815);
        (assign57460_e93816, (locals.var_vt * ((locals.var_vgfbagbcp2_dn3 - locals.var_psip_dn3) - (((2.0 * locals.var_qs_1_dn3) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn3)))), ((locals.var_vt_dn4 * assign57460_e93815) + (locals.var_vt * ((locals.var_vgfbagbcp2_dn4 - locals.var_psip_dn4) - (((2.0 * locals.var_qs_1_dn4) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn4))))), ((locals.var_vt_dn5 * assign57460_e93815) + (locals.var_vt * ((locals.var_vgfbagbcp2_dn5 - locals.var_psip_dn5) - (((2.0 * locals.var_qs_1_dn5) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn5))))), (locals.var_vt * ((locals.var_vgfbagbcp2_dn6 - locals.var_psip_dn6) - (((2.0 * locals.var_qs_1_dn6) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn6)))), (locals.var_vt * ((locals.var_vgfbagbcp2_dn7 - locals.var_psip_dn7) - (((2.0 * locals.var_qs_1_dn7) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn7)))), (locals.var_vt * ((locals.var_vgfbagbcp2_dn8 - locals.var_psip_dn8) - (((2.0 * locals.var_qs_1_dn8) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn8)))), (locals.var_vt * ((locals.var_vgfbagbcp2_dn9 - locals.var_psip_dn9) - (((2.0 * locals.var_qs_1_dn9) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn9)))), (locals.var_vt * ((locals.var_vgfbagbcp2_dn10 - locals.var_psip_dn10) - (((2.0 * locals.var_qs_1_dn10) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn10)))), (locals.var_vt * ((locals.var_vgfbagbcp2_dn11 - locals.var_psip_dn11) - (((2.0 * locals.var_qs_1_dn11) * assign57460_e93813) + (assign57460_e93810 * locals.var_nq_dn11)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign57460_e93818;
        locals.var_t0_dn3 = assign57460_e93818_d_n3;
        locals.var_t0_dn4 = assign57460_e93818_d_n4;
        locals.var_t0_dn5 = assign57460_e93818_d_n5;
        locals.var_t0_dn6 = assign57460_e93818_d_n6;
        locals.var_t0_dn7 = assign57460_e93818_d_n7;
        locals.var_t0_dn8 = assign57460_e93818_d_n8;
        locals.var_t0_dn9 = assign57460_e93818_d_n9;
        locals.var_t0_dn10 = assign57460_e93818_d_n10;
        locals.var_t0_dn11 = assign57460_e93818_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign57470_e93844, assign57470_e93844_d_n3, assign57470_e93844_d_n4, assign57470_e93844_d_n5, assign57470_e93844_d_n6, assign57470_e93844_d_n7, assign57470_e93844_d_n8, assign57470_e93844_d_n9, assign57470_e93844_d_n10, assign57470_e93844_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57470_e93826: f64 = locals.var_t0;
        let assign57470_e93829: f64 = locals.var_t0;
        let assign57470_e93832: f64 = locals.var_t0;
        let assign57470_e93833: f64 = (assign57470_e93829 * assign57470_e93832);
        let assign57470_e93836: f64 = (0.25 * 0.1);
        let assign57470_e93838: f64 = (assign57470_e93836 * 0.1);
        let assign57470_e93839: f64 = (assign57470_e93833 + assign57470_e93838);
        let assign57470_e93840: f64 = (assign57470_e93839).sqrt();
        let assign57470_e93841: f64 = (assign57470_e93826 + assign57470_e93840);
        let assign57470_e93842: f64 = (0.5 * assign57470_e93841);
        (assign57470_e93842, (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn3)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn4)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn5)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn6)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn7)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn8)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn9)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn10)) / (2.0 * assign57470_e93840)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign57470_e93832) + (assign57470_e93829 * locals.var_t0_dn11)) / (2.0 * assign57470_e93840)))),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn3, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn11,)
    }
};
        locals.var_qbs = assign57470_e93844;
        locals.var_qbs_dn3 = assign57470_e93844_d_n3;
        locals.var_qbs_dn4 = assign57470_e93844_d_n4;
        locals.var_qbs_dn5 = assign57470_e93844_d_n5;
        locals.var_qbs_dn6 = assign57470_e93844_d_n6;
        locals.var_qbs_dn7 = assign57470_e93844_d_n7;
        locals.var_qbs_dn8 = assign57470_e93844_d_n8;
        locals.var_qbs_dn9 = assign57470_e93844_d_n9;
        locals.var_qbs_dn10 = assign57470_e93844_d_n10;
        locals.var_qbs_dn11 = assign57470_e93844_d_n11;
        locals.var_qbs_rv = 0.0;

        let (assign57480_e93857, assign57480_e93857_d_n3, assign57480_e93857_d_n4, assign57480_e93857_d_n5, assign57480_e93857_d_n6, assign57480_e93857_d_n7, assign57480_e93857_d_n8, assign57480_e93857_d_n9, assign57480_e93857_d_n10, assign57480_e93857_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57480_e93851: f64 = (2.0 * locals.var_nq);
        let assign57480_e93853: f64 = (assign57480_e93851 * locals.var_vt);
        let assign57480_e93855: f64 = (assign57480_e93853 * locals.var_qs_1);
        (assign57480_e93855, ((((2.0 * locals.var_nq_dn3) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn3)), (((((2.0 * locals.var_nq_dn4) * locals.var_vt) + (assign57480_e93851 * locals.var_vt_dn4)) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn4)), (((((2.0 * locals.var_nq_dn5) * locals.var_vt) + (assign57480_e93851 * locals.var_vt_dn5)) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn5)), ((((2.0 * locals.var_nq_dn6) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn6)), ((((2.0 * locals.var_nq_dn7) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn7)), ((((2.0 * locals.var_nq_dn8) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn8)), ((((2.0 * locals.var_nq_dn9) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn9)), ((((2.0 * locals.var_nq_dn10) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn10)), ((((2.0 * locals.var_nq_dn11) * locals.var_vt) * locals.var_qs_1) + (assign57480_e93853 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_qis, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11,)
    }
};
        locals.var_qis = assign57480_e93857;
        locals.var_qis_dn3 = assign57480_e93857_d_n3;
        locals.var_qis_dn4 = assign57480_e93857_d_n4;
        locals.var_qis_dn5 = assign57480_e93857_d_n5;
        locals.var_qis_dn6 = assign57480_e93857_d_n6;
        locals.var_qis_dn7 = assign57480_e93857_d_n7;
        locals.var_qis_dn8 = assign57480_e93857_d_n8;
        locals.var_qis_dn9 = assign57480_e93857_d_n9;
        locals.var_qis_dn10 = assign57480_e93857_d_n10;
        locals.var_qis_dn11 = assign57480_e93857_d_n11;
        locals.var_qis_rv = 0.0;

        let (assign57490_e93870, assign57490_e93870_d_n3, assign57490_e93870_d_n4, assign57490_e93870_d_n5, assign57490_e93870_d_n6, assign57490_e93870_d_n7, assign57490_e93870_d_n8, assign57490_e93870_d_n9, assign57490_e93870_d_n10, assign57490_e93870_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57490_e93866: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign57490_e93867: f64 = (locals.var_qbs + assign57490_e93866);
        let assign57490_e93868: f64 = (locals.var_eefffactor * assign57490_e93867);
        (assign57490_e93868, (locals.var_eefffactor * (locals.var_qbs_dn3 + (locals.var_eta_mu * locals.var_qis_dn3))), (locals.var_eefffactor * (locals.var_qbs_dn4 + (locals.var_eta_mu * locals.var_qis_dn4))), (locals.var_eefffactor * (locals.var_qbs_dn5 + (locals.var_eta_mu * locals.var_qis_dn5))), (locals.var_eefffactor * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_eefffactor * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_eefffactor * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_eefffactor * (locals.var_qbs_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))), (locals.var_eefffactor * (locals.var_qbs_dn10 + (locals.var_eta_mu * locals.var_qis_dn10))), (locals.var_eefffactor * (locals.var_qbs_dn11 + (locals.var_eta_mu * locals.var_qis_dn11))),)
    } else {
        (locals.var_eeffs, locals.var_eeffs_dn3, locals.var_eeffs_dn4, locals.var_eeffs_dn5, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8, locals.var_eeffs_dn9, locals.var_eeffs_dn10, locals.var_eeffs_dn11,)
    }
};
        locals.var_eeffs = assign57490_e93870;
        locals.var_eeffs_dn3 = assign57490_e93870_d_n3;
        locals.var_eeffs_dn4 = assign57490_e93870_d_n4;
        locals.var_eeffs_dn5 = assign57490_e93870_d_n5;
        locals.var_eeffs_dn6 = assign57490_e93870_d_n6;
        locals.var_eeffs_dn7 = assign57490_e93870_d_n7;
        locals.var_eeffs_dn8 = assign57490_e93870_d_n8;
        locals.var_eeffs_dn9 = assign57490_e93870_d_n9;
        locals.var_eeffs_dn10 = assign57490_e93870_d_n10;
        locals.var_eeffs_dn11 = assign57490_e93870_d_n11;
        locals.var_eeffs_rv = 0.0;

        let (assign57500_e93885, assign57500_e93885_d_n3, assign57500_e93885_d_n4, assign57500_e93885_d_n5, assign57500_e93885_d_n6, assign57500_e93885_d_n7, assign57500_e93885_d_n8, assign57500_e93885_d_n9, assign57500_e93885_d_n10, assign57500_e93885_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57500_e93878: f64 = (locals.var_uc_a * locals.var_vbsx1);
        let assign57500_e93879: f64 = (locals.var_ua_a + assign57500_e93878);
        let assign57500_e93882: f64 = (locals.var_eeffs).powf(locals.var_eu_t);
        let assign57500_e93883: f64 = (assign57500_e93879 * assign57500_e93882);
        (assign57500_e93883, (((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn3))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn3 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn3)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn3 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn3 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn4))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn4 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn4)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn4 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn4 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn5))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn5 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn5)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn5 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn5 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn6))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn6 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn6)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn6 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn6 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn7))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn7 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn7)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn7 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn7 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn8))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn8 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn8)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn8 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn8 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn9))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn9 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn9)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn9 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn9 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn10))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn10 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn10)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn10 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn10 / locals.var_eeffs)))) })), (((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_vbsx1) + (locals.var_uc_a * locals.var_vbsx1_dn11))) * assign57500_e93882) + (assign57500_e93879 * if locals.var_eu_t_dn11 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffs).powf(locals.var_eu_t - 1.0) * locals.var_eeffs_dn11)) } } else { (assign57500_e93882 * ((locals.var_eu_t_dn11 * (locals.var_eeffs).ln()) + (locals.var_eu_t * (locals.var_eeffs_dn11 / locals.var_eeffs)))) })),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57500_e93885;
        locals.var_t3_dn3 = assign57500_e93885_d_n3;
        locals.var_t3_dn4 = assign57500_e93885_d_n4;
        locals.var_t3_dn5 = assign57500_e93885_d_n5;
        locals.var_t3_dn6 = assign57500_e93885_d_n6;
        locals.var_t3_dn7 = assign57500_e93885_d_n7;
        locals.var_t3_dn8 = assign57500_e93885_d_n8;
        locals.var_t3_dn9 = assign57500_e93885_d_n9;
        locals.var_t3_dn10 = assign57500_e93885_d_n10;
        locals.var_t3_dn11 = assign57500_e93885_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57510_e93894, assign57510_e93894_d_n3, assign57510_e93894_d_n4, assign57510_e93894_d_n5, assign57510_e93894_d_n6, assign57510_e93894_d_n7, assign57510_e93894_d_n8, assign57510_e93894_d_n9, assign57510_e93894_d_n10, assign57510_e93894_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57510_e93892: f64 = (1.0 + locals.var_t3);
        (assign57510_e93892, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57510_e93894;
        locals.var_t4_dn3 = assign57510_e93894_d_n3;
        locals.var_t4_dn4 = assign57510_e93894_d_n4;
        locals.var_t4_dn5 = assign57510_e93894_d_n5;
        locals.var_t4_dn6 = assign57510_e93894_d_n6;
        locals.var_t4_dn7 = assign57510_e93894_d_n7;
        locals.var_t4_dn8 = assign57510_e93894_d_n8;
        locals.var_t4_dn9 = assign57510_e93894_d_n9;
        locals.var_t4_dn10 = assign57510_e93894_d_n10;
        locals.var_t4_dn11 = assign57510_e93894_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57520_e93920, assign57520_e93920_d_n3, assign57520_e93920_d_n4, assign57520_e93920_d_n5, assign57520_e93920_d_n6, assign57520_e93920_d_n7, assign57520_e93920_d_n8, assign57520_e93920_d_n9, assign57520_e93920_d_n10, assign57520_e93920_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57520_e93902: f64 = (locals.var_t4 + 1.0);
        let assign57520_e93905: f64 = (locals.var_t4 - 1.0);
        let assign57520_e93908: f64 = (locals.var_t4 - 1.0);
        let assign57520_e93909: f64 = (assign57520_e93905 * assign57520_e93908);
        let assign57520_e93912: f64 = (0.25 * 0.0015);
        let assign57520_e93914: f64 = (assign57520_e93912 * 0.0015);
        let assign57520_e93915: f64 = (assign57520_e93909 + assign57520_e93914);
        let assign57520_e93916: f64 = (assign57520_e93915).sqrt();
        let assign57520_e93917: f64 = (assign57520_e93902 + assign57520_e93916);
        let assign57520_e93918: f64 = (0.5 * assign57520_e93917);
        (assign57520_e93918, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn3)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn4)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn5)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn6)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn7)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn8)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn9)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn10)) / (2.0 * assign57520_e93916)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign57520_e93908) + (assign57520_e93905 * locals.var_t4_dn11)) / (2.0 * assign57520_e93916)))),)
    } else {
        (locals.var_dmobs, locals.var_dmobs_dn3, locals.var_dmobs_dn4, locals.var_dmobs_dn5, locals.var_dmobs_dn6, locals.var_dmobs_dn7, locals.var_dmobs_dn8, locals.var_dmobs_dn9, locals.var_dmobs_dn10, locals.var_dmobs_dn11,)
    }
};
        locals.var_dmobs = assign57520_e93920;
        locals.var_dmobs_dn3 = assign57520_e93920_d_n3;
        locals.var_dmobs_dn4 = assign57520_e93920_d_n4;
        locals.var_dmobs_dn5 = assign57520_e93920_d_n5;
        locals.var_dmobs_dn6 = assign57520_e93920_d_n6;
        locals.var_dmobs_dn7 = assign57520_e93920_d_n7;
        locals.var_dmobs_dn8 = assign57520_e93920_d_n8;
        locals.var_dmobs_dn9 = assign57520_e93920_d_n9;
        locals.var_dmobs_dn10 = assign57520_e93920_d_n10;
        locals.var_dmobs_dn11 = assign57520_e93920_d_n11;
        locals.var_dmobs_rv = 0.0;

        let (assign57530_e93935, assign57530_e93935_d_n3, assign57530_e93935_d_n4, assign57530_e93935_d_n5, assign57530_e93935_d_n6, assign57530_e93935_d_n7, assign57530_e93935_d_n8, assign57530_e93935_d_n9, assign57530_e93935_d_n10, assign57530_e93935_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57530_e93927: f64 = (locals.var_u0_a / locals.var_dmobs);
        let assign57530_e93929: f64 = (assign57530_e93927 * locals.var_vt);
        let assign57530_e93932: f64 = (locals.var_vsatcv_t * locals.var_lact);
        let assign57530_e93933: f64 = (assign57530_e93929 / assign57530_e93932);
        (assign57530_e93933, (((((((locals.var_u0_a_dn3 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn3)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn3 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), ((((((((locals.var_u0_a_dn4 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn4)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) + (assign57530_e93927 * locals.var_vt_dn4)) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn4 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), ((((((((locals.var_u0_a_dn5 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn5)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) + (assign57530_e93927 * locals.var_vt_dn5)) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn5 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), (((((((locals.var_u0_a_dn6 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn6)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn6 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), (((((((locals.var_u0_a_dn7 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn7)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn7 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), (((((((locals.var_u0_a_dn8 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn8)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn8 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), (((((((locals.var_u0_a_dn9 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn9)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn9 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), (((((((locals.var_u0_a_dn10 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn10)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn10 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)), (((((((locals.var_u0_a_dn11 * locals.var_dmobs) - (locals.var_u0_a * locals.var_dmobs_dn11)) / (locals.var_dmobs * locals.var_dmobs)) * locals.var_vt) * assign57530_e93932) - (assign57530_e93929 * (locals.var_vsatcv_t_dn11 * locals.var_lact))) / (assign57530_e93932 * assign57530_e93932)),)
    } else {
        (locals.var_lambdac_by2, locals.var_lambdac_by2_dn3, locals.var_lambdac_by2_dn4, locals.var_lambdac_by2_dn5, locals.var_lambdac_by2_dn6, locals.var_lambdac_by2_dn7, locals.var_lambdac_by2_dn8, locals.var_lambdac_by2_dn9, locals.var_lambdac_by2_dn10, locals.var_lambdac_by2_dn11,)
    }
};
        locals.var_lambdac_by2 = assign57530_e93935;
        locals.var_lambdac_by2_dn3 = assign57530_e93935_d_n3;
        locals.var_lambdac_by2_dn4 = assign57530_e93935_d_n4;
        locals.var_lambdac_by2_dn5 = assign57530_e93935_d_n5;
        locals.var_lambdac_by2_dn6 = assign57530_e93935_d_n6;
        locals.var_lambdac_by2_dn7 = assign57530_e93935_d_n7;
        locals.var_lambdac_by2_dn8 = assign57530_e93935_d_n8;
        locals.var_lambdac_by2_dn9 = assign57530_e93935_d_n9;
        locals.var_lambdac_by2_dn10 = assign57530_e93935_d_n10;
        locals.var_lambdac_by2_dn11 = assign57530_e93935_d_n11;
        locals.var_lambdac_by2_rv = 0.0;

        let (assign57540_e93956, assign57540_e93956_d_n3, assign57540_e93956_d_n4, assign57540_e93956_d_n5, assign57540_e93956_d_n6, assign57540_e93956_d_n7, assign57540_e93956_d_n8, assign57540_e93956_d_n9, assign57540_e93956_d_n10, assign57540_e93956_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57540_e93943: f64 = (locals.var_qs_1 * locals.var_qs_1);
        let assign57540_e93945: f64 = (assign57540_e93943 + locals.var_qs_1);
        let assign57540_e93946: f64 = (locals.var_lambdac_by2 * assign57540_e93945);
        let assign57540_e93951: f64 = (1.0 + locals.var_qs_1);
        let assign57540_e93952: f64 = (locals.var_lambdac_by2 * assign57540_e93951);
        let assign57540_e93953: f64 = (1.0 + assign57540_e93952);
        let assign57540_e93954: f64 = (assign57540_e93946 / assign57540_e93953);
        (assign57540_e93954, (((((locals.var_lambdac_by2_dn3 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn3 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn3)) + locals.var_qs_1_dn3))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn3 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn3)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn4 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn4 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn4)) + locals.var_qs_1_dn4))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn4 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn4)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn5 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn5 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn5)) + locals.var_qs_1_dn5))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn5 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn5)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn6 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn6 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn6)) + locals.var_qs_1_dn6))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn6 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn6)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn7 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn7 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn7)) + locals.var_qs_1_dn7))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn7 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn7)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn8 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn8 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn8)) + locals.var_qs_1_dn8))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn8 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn8)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn9 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn9 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn9)) + locals.var_qs_1_dn9))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn9 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn9)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn10 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn10 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn10)) + locals.var_qs_1_dn10))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn10 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn10)))) / (assign57540_e93953 * assign57540_e93953)), (((((locals.var_lambdac_by2_dn11 * assign57540_e93945) + (locals.var_lambdac_by2 * (((locals.var_qs_1_dn11 * locals.var_qs_1) + (locals.var_qs_1 * locals.var_qs_1_dn11)) + locals.var_qs_1_dn11))) * assign57540_e93953) - (assign57540_e93946 * ((locals.var_lambdac_by2_dn11 * assign57540_e93951) + (locals.var_lambdac_by2 * locals.var_qs_1_dn11)))) / (assign57540_e93953 * assign57540_e93953)),)
    } else {
        (locals.var_qdsat, locals.var_qdsat_dn3, locals.var_qdsat_dn4, locals.var_qdsat_dn5, locals.var_qdsat_dn6, locals.var_qdsat_dn7, locals.var_qdsat_dn8, locals.var_qdsat_dn9, locals.var_qdsat_dn10, locals.var_qdsat_dn11,)
    }
};
        locals.var_qdsat = assign57540_e93956;
        locals.var_qdsat_dn3 = assign57540_e93956_d_n3;
        locals.var_qdsat_dn4 = assign57540_e93956_d_n4;
        locals.var_qdsat_dn5 = assign57540_e93956_d_n5;
        locals.var_qdsat_dn6 = assign57540_e93956_d_n6;
        locals.var_qdsat_dn7 = assign57540_e93956_d_n7;
        locals.var_qdsat_dn8 = assign57540_e93956_d_n8;
        locals.var_qdsat_dn9 = assign57540_e93956_d_n9;
        locals.var_qdsat_dn10 = assign57540_e93956_d_n10;
        locals.var_qdsat_dn11 = assign57540_e93956_d_n11;
        locals.var_qdsat_rv = 0.0;

        let (assign57550_e93996, assign57550_e93996_d_n3, assign57550_e93996_d_n4, assign57550_e93996_d_n5, assign57550_e93996_d_n6, assign57550_e93996_d_n7, assign57550_e93996_d_n8, assign57550_e93996_d_n9, assign57550_e93996_d_n10, assign57550_e93996_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57550_e93964: f64 = (2.0 * locals.var_phibagbcp2);
        let assign57550_e93965: f64 = (locals.var_psip - assign57550_e93964);
        let assign57550_e93968: f64 = (2.0 * locals.var_qdsat);
        let assign57550_e93971: f64 = (locals.var_qdsat * 2.0);
        let assign57550_e93973: f64 = (assign57550_e93971 * locals.var_nq);
        let assign57550_e93975: f64 = (assign57550_e93973 * locals.var_inv_gam);
        let assign57550_e93978: f64 = (locals.var_qdsat * 2.0);
        let assign57550_e93980: f64 = (assign57550_e93978 * locals.var_nq);
        let assign57550_e93982: f64 = (assign57550_e93980 * locals.var_inv_gam);
        let assign57550_e93986: f64 = (locals.var_nq - 1.0);
        let assign57550_e93987: f64 = (locals.var_gamagbcp2 / assign57550_e93986);
        let assign57550_e93988: f64 = (assign57550_e93982 + assign57550_e93987);
        let assign57550_e93989: f64 = (assign57550_e93975 * assign57550_e93988);
        let assign57550_e93991: f64 = (assign57550_e93989).max(1e-38);
        let assign57550_e93992: f64 = (assign57550_e93991).ln();
        let assign57550_e93993: f64 = (assign57550_e93968 + assign57550_e93992);
        let assign57550_e93994: f64 = (assign57550_e93965 - assign57550_e93993);
        (assign57550_e93994, ((locals.var_psip_dn3 - (2.0 * locals.var_phibagbcp2_dn3)) - ((2.0 * locals.var_qdsat_dn3) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn3 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn3)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn3)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn3 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn3)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn3)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn3) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn4 - (2.0 * locals.var_phibagbcp2_dn4)) - ((2.0 * locals.var_qdsat_dn4) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn4 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn4)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn4)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn4 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn4)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn4)) + (((locals.var_gamagbcp2_dn4 * assign57550_e93986) - (locals.var_gamagbcp2 * locals.var_nq_dn4)) / (assign57550_e93986 * assign57550_e93986))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn5 - (2.0 * locals.var_phibagbcp2_dn5)) - ((2.0 * locals.var_qdsat_dn5) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn5 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn5)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn5)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn5 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn5)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn5)) + (((locals.var_gamagbcp2_dn5 * assign57550_e93986) - (locals.var_gamagbcp2 * locals.var_nq_dn5)) / (assign57550_e93986 * assign57550_e93986))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn6 - (2.0 * locals.var_phibagbcp2_dn6)) - ((2.0 * locals.var_qdsat_dn6) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn6 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn6)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn6)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn6 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn6)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn6)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn6) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn7 - (2.0 * locals.var_phibagbcp2_dn7)) - ((2.0 * locals.var_qdsat_dn7) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn7 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn7)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn7)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn7 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn7)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn7)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn7) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn8 - (2.0 * locals.var_phibagbcp2_dn8)) - ((2.0 * locals.var_qdsat_dn8) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn8 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn8)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn8)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn8 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn8)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn8)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn8) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn9 - (2.0 * locals.var_phibagbcp2_dn9)) - ((2.0 * locals.var_qdsat_dn9) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn9 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn9)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn9)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn9 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn9)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn9)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn9) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn10 - (2.0 * locals.var_phibagbcp2_dn10)) - ((2.0 * locals.var_qdsat_dn10) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn10 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn10)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn10)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn10 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn10)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn10)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn10) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))), ((locals.var_psip_dn11 - (2.0 * locals.var_phibagbcp2_dn11)) - ((2.0 * locals.var_qdsat_dn11) + (if assign57550_e93989 >= 1e-38 { (((((((locals.var_qdsat_dn11 * 2.0) * locals.var_nq) + (assign57550_e93971 * locals.var_nq_dn11)) * locals.var_inv_gam) + (assign57550_e93973 * locals.var_inv_gam_dn11)) * assign57550_e93988) + (assign57550_e93975 * ((((((locals.var_qdsat_dn11 * 2.0) * locals.var_nq) + (assign57550_e93978 * locals.var_nq_dn11)) * locals.var_inv_gam) + (assign57550_e93980 * locals.var_inv_gam_dn11)) + (-((locals.var_gamagbcp2 * locals.var_nq_dn11) / (assign57550_e93986 * assign57550_e93986)))))) } else { 0.0 } / assign57550_e93991))),)
    } else {
        (locals.var_vdsatcv, locals.var_vdsatcv_dn3, locals.var_vdsatcv_dn4, locals.var_vdsatcv_dn5, locals.var_vdsatcv_dn6, locals.var_vdsatcv_dn7, locals.var_vdsatcv_dn8, locals.var_vdsatcv_dn9, locals.var_vdsatcv_dn10, locals.var_vdsatcv_dn11,)
    }
};
        locals.var_vdsatcv = assign57550_e93996;
        locals.var_vdsatcv_dn3 = assign57550_e93996_d_n3;
        locals.var_vdsatcv_dn4 = assign57550_e93996_d_n4;
        locals.var_vdsatcv_dn5 = assign57550_e93996_d_n5;
        locals.var_vdsatcv_dn6 = assign57550_e93996_d_n6;
        locals.var_vdsatcv_dn7 = assign57550_e93996_d_n7;
        locals.var_vdsatcv_dn8 = assign57550_e93996_d_n8;
        locals.var_vdsatcv_dn9 = assign57550_e93996_d_n9;
        locals.var_vdsatcv_dn10 = assign57550_e93996_d_n10;
        locals.var_vdsatcv_dn11 = assign57550_e93996_d_n11;
        locals.var_vdsatcv_rv = 0.0;

        let (assign57560_e94005, assign57560_e94005_d_n3, assign57560_e94005_d_n4, assign57560_e94005_d_n5, assign57560_e94005_d_n6, assign57560_e94005_d_n7, assign57560_e94005_d_n8, assign57560_e94005_d_n9, assign57560_e94005_d_n10, assign57560_e94005_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57560_e94003: f64 = (locals.var_vdsatcv * locals.var_vt);
        (assign57560_e94003, (locals.var_vdsatcv_dn3 * locals.var_vt), ((locals.var_vdsatcv_dn4 * locals.var_vt) + (locals.var_vdsatcv * locals.var_vt_dn4)), ((locals.var_vdsatcv_dn5 * locals.var_vt) + (locals.var_vdsatcv * locals.var_vt_dn5)), (locals.var_vdsatcv_dn6 * locals.var_vt), (locals.var_vdsatcv_dn7 * locals.var_vt), (locals.var_vdsatcv_dn8 * locals.var_vt), (locals.var_vdsatcv_dn9 * locals.var_vt), (locals.var_vdsatcv_dn10 * locals.var_vt), (locals.var_vdsatcv_dn11 * locals.var_vt),)
    } else {
        (locals.var_vdsatcv_1, locals.var_vdsatcv_1_dn3, locals.var_vdsatcv_1_dn4, locals.var_vdsatcv_1_dn5, locals.var_vdsatcv_1_dn6, locals.var_vdsatcv_1_dn7, locals.var_vdsatcv_1_dn8, locals.var_vdsatcv_1_dn9, locals.var_vdsatcv_1_dn10, locals.var_vdsatcv_1_dn11,)
    }
};
        locals.var_vdsatcv_1 = assign57560_e94005;
        locals.var_vdsatcv_1_dn3 = assign57560_e94005_d_n3;
        locals.var_vdsatcv_1_dn4 = assign57560_e94005_d_n4;
        locals.var_vdsatcv_1_dn5 = assign57560_e94005_d_n5;
        locals.var_vdsatcv_1_dn6 = assign57560_e94005_d_n6;
        locals.var_vdsatcv_1_dn7 = assign57560_e94005_d_n7;
        locals.var_vdsatcv_1_dn8 = assign57560_e94005_d_n8;
        locals.var_vdsatcv_1_dn9 = assign57560_e94005_d_n9;
        locals.var_vdsatcv_1_dn10 = assign57560_e94005_d_n10;
        locals.var_vdsatcv_1_dn11 = assign57560_e94005_d_n11;
        locals.var_vdsatcv_1_rv = 0.0;

        let (assign57570_e94037, assign57570_e94037_d_n3, assign57570_e94037_d_n4, assign57570_e94037_d_n5, assign57570_e94037_d_n6, assign57570_e94037_d_n7, assign57570_e94037_d_n8, assign57570_e94037_d_n9, assign57570_e94037_d_n10, assign57570_e94037_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57570_e94013: f64 = (locals.var_vdsatcv_1 - locals.var_vs1);
        let assign57570_e94015: f64 = assign57570_e94013;
        let assign57570_e94018: f64 = (locals.var_vdsatcv_1 - locals.var_vs1);
        let assign57570_e94020: f64 = assign57570_e94018;
        let assign57570_e94023: f64 = (locals.var_vdsatcv_1 - locals.var_vs1);
        let assign57570_e94025: f64 = assign57570_e94023;
        let assign57570_e94026: f64 = (assign57570_e94020 * assign57570_e94025);
        let assign57570_e94029: f64 = (0.25 * 0.001);
        let assign57570_e94031: f64 = (assign57570_e94029 * 0.001);
        let assign57570_e94032: f64 = (assign57570_e94026 + assign57570_e94031);
        let assign57570_e94033: f64 = (assign57570_e94032).sqrt();
        let assign57570_e94034: f64 = (assign57570_e94015 + assign57570_e94033);
        let assign57570_e94035: f64 = (0.5 * assign57570_e94034);
        (assign57570_e94035, (0.5 * (locals.var_vdsatcv_1_dn3 + (((locals.var_vdsatcv_1_dn3 * assign57570_e94025) + (assign57570_e94020 * locals.var_vdsatcv_1_dn3)) / (2.0 * assign57570_e94033)))), (0.5 * (locals.var_vdsatcv_1_dn4 + (((locals.var_vdsatcv_1_dn4 * assign57570_e94025) + (assign57570_e94020 * locals.var_vdsatcv_1_dn4)) / (2.0 * assign57570_e94033)))), (0.5 * (locals.var_vdsatcv_1_dn5 + (((locals.var_vdsatcv_1_dn5 * assign57570_e94025) + (assign57570_e94020 * locals.var_vdsatcv_1_dn5)) / (2.0 * assign57570_e94033)))), (0.5 * ((locals.var_vdsatcv_1_dn6 - locals.var_vs1_dn6) + ((((locals.var_vdsatcv_1_dn6 - locals.var_vs1_dn6) * assign57570_e94025) + (assign57570_e94020 * (locals.var_vdsatcv_1_dn6 - locals.var_vs1_dn6))) / (2.0 * assign57570_e94033)))), (0.5 * ((locals.var_vdsatcv_1_dn7 - locals.var_vs1_dn7) + ((((locals.var_vdsatcv_1_dn7 - locals.var_vs1_dn7) * assign57570_e94025) + (assign57570_e94020 * (locals.var_vdsatcv_1_dn7 - locals.var_vs1_dn7))) / (2.0 * assign57570_e94033)))), (0.5 * (locals.var_vdsatcv_1_dn8 + (((locals.var_vdsatcv_1_dn8 * assign57570_e94025) + (assign57570_e94020 * locals.var_vdsatcv_1_dn8)) / (2.0 * assign57570_e94033)))), (0.5 * (locals.var_vdsatcv_1_dn9 + (((locals.var_vdsatcv_1_dn9 * assign57570_e94025) + (assign57570_e94020 * locals.var_vdsatcv_1_dn9)) / (2.0 * assign57570_e94033)))), (0.5 * (locals.var_vdsatcv_1_dn10 + (((locals.var_vdsatcv_1_dn10 * assign57570_e94025) + (assign57570_e94020 * locals.var_vdsatcv_1_dn10)) / (2.0 * assign57570_e94033)))), (0.5 * ((locals.var_vdsatcv_1_dn11 - locals.var_vs1_dn11) + ((((locals.var_vdsatcv_1_dn11 - locals.var_vs1_dn11) * assign57570_e94025) + (assign57570_e94020 * (locals.var_vdsatcv_1_dn11 - locals.var_vs1_dn11))) / (2.0 * assign57570_e94033)))),)
    } else {
        (locals.var_vdssatcv, locals.var_vdssatcv_dn3, locals.var_vdssatcv_dn4, locals.var_vdssatcv_dn5, locals.var_vdssatcv_dn6, locals.var_vdssatcv_dn7, locals.var_vdssatcv_dn8, locals.var_vdssatcv_dn9, locals.var_vdssatcv_dn10, locals.var_vdssatcv_dn11,)
    }
};
        locals.var_vdssatcv = assign57570_e94037;
        locals.var_vdssatcv_dn3 = assign57570_e94037_d_n3;
        locals.var_vdssatcv_dn4 = assign57570_e94037_d_n4;
        locals.var_vdssatcv_dn5 = assign57570_e94037_d_n5;
        locals.var_vdssatcv_dn6 = assign57570_e94037_d_n6;
        locals.var_vdssatcv_dn7 = assign57570_e94037_d_n7;
        locals.var_vdssatcv_dn8 = assign57570_e94037_d_n8;
        locals.var_vdssatcv_dn9 = assign57570_e94037_d_n9;
        locals.var_vdssatcv_dn10 = assign57570_e94037_d_n10;
        locals.var_vdssatcv_dn11 = assign57570_e94037_d_n11;
        locals.var_vdssatcv_rv = 0.0;

        let (assign57580_e94044, assign57580_e94044_d_n3, assign57580_e94044_d_n4, assign57580_e94044_d_n5, assign57580_e94044_d_n6, assign57580_e94044_d_n7, assign57580_e94044_d_n8, assign57580_e94044_d_n9, assign57580_e94044_d_n10, assign57580_e94044_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        (locals.var_vdssatcv, locals.var_vdssatcv_dn3, locals.var_vdssatcv_dn4, locals.var_vdssatcv_dn5, locals.var_vdssatcv_dn6, locals.var_vdssatcv_dn7, locals.var_vdssatcv_dn8, locals.var_vdssatcv_dn9, locals.var_vdssatcv_dn10, locals.var_vdssatcv_dn11,)
    } else {
        (locals.var_vdssatcv, locals.var_vdssatcv_dn3, locals.var_vdssatcv_dn4, locals.var_vdssatcv_dn5, locals.var_vdssatcv_dn6, locals.var_vdssatcv_dn7, locals.var_vdssatcv_dn8, locals.var_vdssatcv_dn9, locals.var_vdssatcv_dn10, locals.var_vdssatcv_dn11,)
    }
};
        locals.var_vdssatcv = assign57580_e94044;
        locals.var_vdssatcv_dn3 = assign57580_e94044_d_n3;
        locals.var_vdssatcv_dn4 = assign57580_e94044_d_n4;
        locals.var_vdssatcv_dn5 = assign57580_e94044_d_n5;
        locals.var_vdssatcv_dn6 = assign57580_e94044_d_n6;
        locals.var_vdssatcv_dn7 = assign57580_e94044_d_n7;
        locals.var_vdssatcv_dn8 = assign57580_e94044_d_n8;
        locals.var_vdssatcv_dn9 = assign57580_e94044_d_n9;
        locals.var_vdssatcv_dn10 = assign57580_e94044_d_n10;
        locals.var_vdssatcv_dn11 = assign57580_e94044_d_n11;
        locals.var_vdssatcv_rv = 0.0;

        let (assign57590_e94059, assign57590_e94059_d_n3, assign57590_e94059_d_n4, assign57590_e94059_d_n5, assign57590_e94059_d_n6, assign57590_e94059_d_n7, assign57590_e94059_d_n8, assign57590_e94059_d_n9, assign57590_e94059_d_n10, assign57590_e94059_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57590_e94051: f64 = (locals.var_vds / locals.var_vdssatcv);
        let assign57590_e94053: f64 = (assign57590_e94051 + 1e-6);
        let assign57590_e94056: f64 = (1.0 / locals.var_delta_t);
        let assign57590_e94057: f64 = (assign57590_e94053).powf(assign57590_e94056);
        (assign57590_e94057, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn3) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((-((locals.var_vds * locals.var_vdssatcv_dn3) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn4) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((-((locals.var_vds * locals.var_vdssatcv_dn4) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn5) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((-((locals.var_vds * locals.var_vdssatcv_dn5) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (((locals.var_vds_dn6 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn6)) / (locals.var_vdssatcv * locals.var_vdssatcv)))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((((locals.var_vds_dn6 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn6)) / (locals.var_vdssatcv * locals.var_vdssatcv)) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn7)) / (locals.var_vdssatcv * locals.var_vdssatcv)))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((((locals.var_vds_dn7 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn7)) / (locals.var_vdssatcv * locals.var_vdssatcv)) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn8) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((-((locals.var_vds * locals.var_vdssatcv_dn8) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn9) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((-((locals.var_vds * locals.var_vdssatcv_dn9) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (((locals.var_vds_dn10 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn10)) / (locals.var_vdssatcv * locals.var_vdssatcv)))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((((locals.var_vds_dn10 * locals.var_vdssatcv) - (locals.var_vds * locals.var_vdssatcv_dn10)) / (locals.var_vdssatcv * locals.var_vdssatcv)) / assign57590_e94053)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign57590_e94056) as f64).is_finite() && ((assign57590_e94056) as f64).fract() == 0.0 { if assign57590_e94056 == 0.0 { 0.0 } else { (assign57590_e94056 * ((assign57590_e94053).powf(assign57590_e94056 - 1.0) * (-((locals.var_vds * locals.var_vdssatcv_dn11) / (locals.var_vdssatcv * locals.var_vdssatcv))))) } } else { (assign57590_e94057 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign57590_e94053).ln()) + (assign57590_e94056 * ((-((locals.var_vds * locals.var_vdssatcv_dn11) / (locals.var_vdssatcv * locals.var_vdssatcv))) / assign57590_e94053)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign57590_e94059;
        locals.var_t7_dn3 = assign57590_e94059_d_n3;
        locals.var_t7_dn4 = assign57590_e94059_d_n4;
        locals.var_t7_dn5 = assign57590_e94059_d_n5;
        locals.var_t7_dn6 = assign57590_e94059_d_n6;
        locals.var_t7_dn7 = assign57590_e94059_d_n7;
        locals.var_t7_dn8 = assign57590_e94059_d_n8;
        locals.var_t7_dn9 = assign57590_e94059_d_n9;
        locals.var_t7_dn10 = assign57590_e94059_d_n10;
        locals.var_t7_dn11 = assign57590_e94059_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign57600_e94071, assign57600_e94071_d_n3, assign57600_e94071_d_n4, assign57600_e94071_d_n5, assign57600_e94071_d_n6, assign57600_e94071_d_n7, assign57600_e94071_d_n8, assign57600_e94071_d_n9, assign57600_e94071_d_n10, assign57600_e94071_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57600_e94066: f64 = (1.0 + locals.var_t7);
        let assign57600_e94068: f64 = (-locals.var_delta_t);
        let assign57600_e94069: f64 = (assign57600_e94066).powf(assign57600_e94068);
        (assign57600_e94069, if (-locals.var_delta_t_dn3) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn3)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn3) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn3 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn4)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn4) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn4 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn5)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn5) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn5 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn6)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn6) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn6 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn7)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn7) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn7 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn8)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn8) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn8 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn9)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn9) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn9 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn10)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn10) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn10 / assign57600_e94066)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign57600_e94068) as f64).is_finite() && ((assign57600_e94068) as f64).fract() == 0.0 { if assign57600_e94068 == 0.0 { 0.0 } else { (assign57600_e94068 * ((assign57600_e94066).powf(assign57600_e94068 - 1.0) * locals.var_t7_dn11)) } } else { (assign57600_e94069 * (((-locals.var_delta_t_dn11) * (assign57600_e94066).ln()) + (assign57600_e94068 * (locals.var_t7_dn11 / assign57600_e94066)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign57600_e94071;
        locals.var_t8_dn3 = assign57600_e94071_d_n3;
        locals.var_t8_dn4 = assign57600_e94071_d_n4;
        locals.var_t8_dn5 = assign57600_e94071_d_n5;
        locals.var_t8_dn6 = assign57600_e94071_d_n6;
        locals.var_t8_dn7 = assign57600_e94071_d_n7;
        locals.var_t8_dn8 = assign57600_e94071_d_n8;
        locals.var_t8_dn9 = assign57600_e94071_d_n9;
        locals.var_t8_dn10 = assign57600_e94071_d_n10;
        locals.var_t8_dn11 = assign57600_e94071_d_n11;
        locals.var_t8_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_200(
        locals: &mut StampLocals,
    ) {
        let (assign57610_e94080, assign57610_e94080_d_n3, assign57610_e94080_d_n4, assign57610_e94080_d_n5, assign57610_e94080_d_n6, assign57610_e94080_d_n7, assign57610_e94080_d_n8, assign57610_e94080_d_n9, assign57610_e94080_d_n10, assign57610_e94080_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57610_e94078: f64 = (locals.var_vds * locals.var_t8);
        (assign57610_e94078, (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), (locals.var_vds * locals.var_t8_dn5), ((locals.var_vds_dn6 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn6)), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), ((locals.var_vds_dn10 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn10)), (locals.var_vds * locals.var_t8_dn11),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11,)
    }
};
        locals.var_vdseff = assign57610_e94080;
        locals.var_vdseff_dn3 = assign57610_e94080_d_n3;
        locals.var_vdseff_dn4 = assign57610_e94080_d_n4;
        locals.var_vdseff_dn5 = assign57610_e94080_d_n5;
        locals.var_vdseff_dn6 = assign57610_e94080_d_n6;
        locals.var_vdseff_dn7 = assign57610_e94080_d_n7;
        locals.var_vdseff_dn8 = assign57610_e94080_d_n8;
        locals.var_vdseff_dn9 = assign57610_e94080_d_n9;
        locals.var_vdseff_dn10 = assign57610_e94080_d_n10;
        locals.var_vdseff_dn11 = assign57610_e94080_d_n11;
        locals.var_vdseff_rv = 0.0;

        let (assign57620_e94091, assign57620_e94091_d_n3, assign57620_e94091_d_n4, assign57620_e94091_d_n5, assign57620_e94091_d_n6, assign57620_e94091_d_n7, assign57620_e94091_d_n8, assign57620_e94091_d_n9, assign57620_e94091_d_n10, assign57620_e94091_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57620_e94087: f64 = (locals.var_vdseff + locals.var_vs1);
        let assign57620_e94089: f64 = (assign57620_e94087 * locals.var_inv_vt);
        (assign57620_e94089, (locals.var_vdseff_dn3 * locals.var_inv_vt), ((locals.var_vdseff_dn4 * locals.var_inv_vt) + (assign57620_e94087 * locals.var_inv_vt_dn4)), ((locals.var_vdseff_dn5 * locals.var_inv_vt) + (assign57620_e94087 * locals.var_inv_vt_dn5)), ((locals.var_vdseff_dn6 + locals.var_vs1_dn6) * locals.var_inv_vt), ((locals.var_vdseff_dn7 + locals.var_vs1_dn7) * locals.var_inv_vt), (locals.var_vdseff_dn8 * locals.var_inv_vt), (locals.var_vdseff_dn9 * locals.var_inv_vt), (locals.var_vdseff_dn10 * locals.var_inv_vt), ((locals.var_vdseff_dn11 + locals.var_vs1_dn11) * locals.var_inv_vt),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11,)
    }
};
        locals.var_vdeff = assign57620_e94091;
        locals.var_vdeff_dn3 = assign57620_e94091_d_n3;
        locals.var_vdeff_dn4 = assign57620_e94091_d_n4;
        locals.var_vdeff_dn5 = assign57620_e94091_d_n5;
        locals.var_vdeff_dn6 = assign57620_e94091_d_n6;
        locals.var_vdeff_dn7 = assign57620_e94091_d_n7;
        locals.var_vdeff_dn8 = assign57620_e94091_d_n8;
        locals.var_vdeff_dn9 = assign57620_e94091_d_n9;
        locals.var_vdeff_dn10 = assign57620_e94091_d_n10;
        locals.var_vdeff_dn11 = assign57620_e94091_d_n11;
        locals.var_vdeff_rv = 0.0;

        let (assign57630_e94117, assign57630_e94117_d_n3, assign57630_e94117_d_n4, assign57630_e94117_d_n5, assign57630_e94117_d_n6, assign57630_e94117_d_n7, assign57630_e94117_d_n8, assign57630_e94117_d_n9, assign57630_e94117_d_n10, assign57630_e94117_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57630_e94099: f64 = (locals.var_psip + 1.0);
        let assign57630_e94102: f64 = (locals.var_psip - 1.0);
        let assign57630_e94105: f64 = (locals.var_psip - 1.0);
        let assign57630_e94106: f64 = (assign57630_e94102 * assign57630_e94105);
        let assign57630_e94109: f64 = (0.25 * 2.0);
        let assign57630_e94111: f64 = (assign57630_e94109 * 2.0);
        let assign57630_e94112: f64 = (assign57630_e94106 + assign57630_e94111);
        let assign57630_e94113: f64 = (assign57630_e94112).sqrt();
        let assign57630_e94114: f64 = (assign57630_e94099 + assign57630_e94113);
        let assign57630_e94115: f64 = (0.5 * assign57630_e94114);
        (assign57630_e94115, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn3)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn4)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn5)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn6)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn7)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn8)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn9)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn10)) / (2.0 * assign57630_e94113)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign57630_e94105) + (assign57630_e94102 * locals.var_psip_dn11)) / (2.0 * assign57630_e94113)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign57630_e94117;
        locals.var_t8_dn3 = assign57630_e94117_d_n3;
        locals.var_t8_dn4 = assign57630_e94117_d_n4;
        locals.var_t8_dn5 = assign57630_e94117_d_n5;
        locals.var_t8_dn6 = assign57630_e94117_d_n6;
        locals.var_t8_dn7 = assign57630_e94117_d_n7;
        locals.var_t8_dn8 = assign57630_e94117_d_n8;
        locals.var_t8_dn9 = assign57630_e94117_d_n9;
        locals.var_t8_dn10 = assign57630_e94117_d_n10;
        locals.var_t8_dn11 = assign57630_e94117_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign57640_e94125, assign57640_e94125_d_n3, assign57640_e94125_d_n4, assign57640_e94125_d_n5, assign57640_e94125_d_n6, assign57640_e94125_d_n7, assign57640_e94125_d_n8, assign57640_e94125_d_n9, assign57640_e94125_d_n10, assign57640_e94125_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57640_e94123: f64 = (locals.var_t8).sqrt();
        (assign57640_e94123, (locals.var_t8_dn3 / (2.0 * assign57640_e94123)), (locals.var_t8_dn4 / (2.0 * assign57640_e94123)), (locals.var_t8_dn5 / (2.0 * assign57640_e94123)), (locals.var_t8_dn6 / (2.0 * assign57640_e94123)), (locals.var_t8_dn7 / (2.0 * assign57640_e94123)), (locals.var_t8_dn8 / (2.0 * assign57640_e94123)), (locals.var_t8_dn9 / (2.0 * assign57640_e94123)), (locals.var_t8_dn10 / (2.0 * assign57640_e94123)), (locals.var_t8_dn11 / (2.0 * assign57640_e94123)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign57640_e94125;
        locals.var_sqrtpsip_dn3 = assign57640_e94125_d_n3;
        locals.var_sqrtpsip_dn4 = assign57640_e94125_d_n4;
        locals.var_sqrtpsip_dn5 = assign57640_e94125_d_n5;
        locals.var_sqrtpsip_dn6 = assign57640_e94125_d_n6;
        locals.var_sqrtpsip_dn7 = assign57640_e94125_d_n7;
        locals.var_sqrtpsip_dn8 = assign57640_e94125_d_n8;
        locals.var_sqrtpsip_dn9 = assign57640_e94125_d_n9;
        locals.var_sqrtpsip_dn10 = assign57640_e94125_d_n10;
        locals.var_sqrtpsip_dn11 = assign57640_e94125_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign57650_e94140, assign57650_e94140_d_n3, assign57650_e94140_d_n4, assign57650_e94140_d_n5, assign57650_e94140_d_n6, assign57650_e94140_d_n7, assign57650_e94140_d_n8, assign57650_e94140_d_n9, assign57650_e94140_d_n10, assign57650_e94140_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57650_e94134: f64 = (2.0 * locals.var_sqrtpsip);
        let assign57650_e94135: f64 = (locals.var_gamagbcp2 / assign57650_e94134);
        let assign57650_e94136: f64 = (1.0 + assign57650_e94135);
        let assign57650_e94138: f64 = (assign57650_e94136 / locals.var_gamagbcp2);
        (assign57650_e94138, ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn3)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2), ((((((locals.var_gamagbcp2_dn4 * assign57650_e94134) - (locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn4))) / (assign57650_e94134 * assign57650_e94134)) * locals.var_gamagbcp2) - (assign57650_e94136 * locals.var_gamagbcp2_dn4)) / (locals.var_gamagbcp2 * locals.var_gamagbcp2)), ((((((locals.var_gamagbcp2_dn5 * assign57650_e94134) - (locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn5))) / (assign57650_e94134 * assign57650_e94134)) * locals.var_gamagbcp2) - (assign57650_e94136 * locals.var_gamagbcp2_dn5)) / (locals.var_gamagbcp2 * locals.var_gamagbcp2)), ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn6)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2), ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn7)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2), ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn8)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2), ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn9)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2), ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn10)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2), ((-((locals.var_gamagbcp2 * (2.0 * locals.var_sqrtpsip_dn11)) / (assign57650_e94134 * assign57650_e94134))) / locals.var_gamagbcp2),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign57650_e94140;
        locals.var_t0_dn3 = assign57650_e94140_d_n3;
        locals.var_t0_dn4 = assign57650_e94140_d_n4;
        locals.var_t0_dn5 = assign57650_e94140_d_n5;
        locals.var_t0_dn6 = assign57650_e94140_d_n6;
        locals.var_t0_dn7 = assign57650_e94140_d_n7;
        locals.var_t0_dn8 = assign57650_e94140_d_n8;
        locals.var_t0_dn9 = assign57650_e94140_d_n9;
        locals.var_t0_dn10 = assign57650_e94140_d_n10;
        locals.var_t0_dn11 = assign57650_e94140_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign57660_e94153, assign57660_e94153_d_n3, assign57660_e94153_d_n4, assign57660_e94153_d_n5, assign57660_e94153_d_n6, assign57660_e94153_d_n7, assign57660_e94153_d_n8, assign57660_e94153_d_n9, assign57660_e94153_d_n10, assign57660_e94153_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57660_e94148: f64 = (2.0 * locals.var_phibagbcp2);
        let assign57660_e94149: f64 = (locals.var_psip - assign57660_e94148);
        let assign57660_e94151: f64 = (assign57660_e94149 - locals.var_vdeff);
        (assign57660_e94151, ((locals.var_psip_dn3 - (2.0 * locals.var_phibagbcp2_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phibagbcp2_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phibagbcp2_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phibagbcp2_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phibagbcp2_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phibagbcp2_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phibagbcp2_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phibagbcp2_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phibagbcp2_dn11)) - locals.var_vdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign57660_e94153;
        locals.var_t1_dn3 = assign57660_e94153_d_n3;
        locals.var_t1_dn4 = assign57660_e94153_d_n4;
        locals.var_t1_dn5 = assign57660_e94153_d_n5;
        locals.var_t1_dn6 = assign57660_e94153_d_n6;
        locals.var_t1_dn7 = assign57660_e94153_d_n7;
        locals.var_t1_dn8 = assign57660_e94153_d_n8;
        locals.var_t1_dn9 = assign57660_e94153_d_n9;
        locals.var_t1_dn10 = assign57660_e94153_d_n10;
        locals.var_t1_dn11 = assign57660_e94153_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign57670_e94169, assign57670_e94169_d_n3, assign57670_e94169_d_n4, assign57670_e94169_d_n5, assign57670_e94169_d_n6, assign57670_e94169_d_n7, assign57670_e94169_d_n8, assign57670_e94169_d_n9, assign57670_e94169_d_n10, assign57670_e94169_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57670_e94161: f64 = (4.0 * locals.var_t0);
        let assign57670_e94163: f64 = (assign57670_e94161 * locals.var_sqrtpsip);
        let assign57670_e94165: f64 = (assign57670_e94163).max(1e-38);
        let assign57670_e94166: f64 = (assign57670_e94165).ln();
        let assign57670_e94167: f64 = (locals.var_t1 - assign57670_e94166);
        (assign57670_e94167, (locals.var_t1_dn3 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn4 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn5 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn6 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn7 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn8 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn9 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn10 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign57670_e94165)), (locals.var_t1_dn11 - (if assign57670_e94163 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign57670_e94161 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign57670_e94165)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign57670_e94169;
        locals.var_t2_dn3 = assign57670_e94169_d_n3;
        locals.var_t2_dn4 = assign57670_e94169_d_n4;
        locals.var_t2_dn5 = assign57670_e94169_d_n5;
        locals.var_t2_dn6 = assign57670_e94169_d_n6;
        locals.var_t2_dn7 = assign57670_e94169_d_n7;
        locals.var_t2_dn8 = assign57670_e94169_d_n8;
        locals.var_t2_dn9 = assign57670_e94169_d_n9;
        locals.var_t2_dn10 = assign57670_e94169_d_n10;
        locals.var_t2_dn11 = assign57670_e94169_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign57680_e94189, assign57680_e94189_d_n3, assign57680_e94189_d_n4, assign57680_e94189_d_n5, assign57680_e94189_d_n6, assign57680_e94189_d_n7, assign57680_e94189_d_n8, assign57680_e94189_d_n9, assign57680_e94189_d_n10, assign57680_e94189_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57680_e94177: f64 = (locals.var_t2 - 0.201491);
        let assign57680_e94181: f64 = (locals.var_t2 + 0.402982);
        let assign57680_e94182: f64 = (locals.var_t2 * assign57680_e94181);
        let assign57680_e94184: f64 = (assign57680_e94182 + 2.446562);
        let assign57680_e94185: f64 = (assign57680_e94184).sqrt();
        let assign57680_e94186: f64 = (assign57680_e94177 - assign57680_e94185);
        let assign57680_e94187: f64 = (0.5 * assign57680_e94186);
        (assign57680_e94187, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign57680_e94185)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign57680_e94181) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign57680_e94185)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign57680_e94189;
        locals.var_t8_dn3 = assign57680_e94189_d_n3;
        locals.var_t8_dn4 = assign57680_e94189_d_n4;
        locals.var_t8_dn5 = assign57680_e94189_d_n5;
        locals.var_t8_dn6 = assign57680_e94189_d_n6;
        locals.var_t8_dn7 = assign57680_e94189_d_n7;
        locals.var_t8_dn8 = assign57680_e94189_d_n8;
        locals.var_t8_dn9 = assign57680_e94189_d_n9;
        locals.var_t8_dn10 = assign57680_e94189_d_n10;
        locals.var_t8_dn11 = assign57680_e94189_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign57690_e94196, assign57690_e94196_d_n3, assign57690_e94196_d_n4, assign57690_e94196_d_n5, assign57690_e94196_d_n6, assign57690_e94196_d_n7, assign57690_e94196_d_n8, assign57690_e94196_d_n9, assign57690_e94196_d_n10, assign57690_e94196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign57690_e94196;
        locals.var_sqrtpsisa_dn3 = assign57690_e94196_d_n3;
        locals.var_sqrtpsisa_dn4 = assign57690_e94196_d_n4;
        locals.var_sqrtpsisa_dn5 = assign57690_e94196_d_n5;
        locals.var_sqrtpsisa_dn6 = assign57690_e94196_d_n6;
        locals.var_sqrtpsisa_dn7 = assign57690_e94196_d_n7;
        locals.var_sqrtpsisa_dn8 = assign57690_e94196_d_n8;
        locals.var_sqrtpsisa_dn9 = assign57690_e94196_d_n9;
        locals.var_sqrtpsisa_dn10 = assign57690_e94196_d_n10;
        locals.var_sqrtpsisa_dn11 = assign57690_e94196_d_n11;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign57700_e94199: f64 = (-68.0);
        let assign57700_e94200: f64 = if locals.var_t8 <= assign57700_e94199 { 1.0 } else { 0.0 };
        locals.var_guard858 = assign57700_e94200;
        locals.var_guard858_rv = 0.0;

        let (assign57710_e94210, assign57710_e94210_d_n3, assign57710_e94210_d_n4, assign57710_e94210_d_n5, assign57710_e94210_d_n6, assign57710_e94210_d_n7, assign57710_e94210_d_n8, assign57710_e94210_d_n9, assign57710_e94210_d_n10, assign57710_e94210_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) {
        let assign57710_e94208: f64 = (-100.0);
        (assign57710_e94208, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57710_e94210;
        locals.var_t4_dn3 = assign57710_e94210_d_n3;
        locals.var_t4_dn4 = assign57710_e94210_d_n4;
        locals.var_t4_dn5 = assign57710_e94210_d_n5;
        locals.var_t4_dn6 = assign57710_e94210_d_n6;
        locals.var_t4_dn7 = assign57710_e94210_d_n7;
        locals.var_t4_dn8 = assign57710_e94210_d_n8;
        locals.var_t4_dn9 = assign57710_e94210_d_n9;
        locals.var_t4_dn10 = assign57710_e94210_d_n10;
        locals.var_t4_dn11 = assign57710_e94210_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57720_e94219, assign57720_e94219_d_n3, assign57720_e94219_d_n4, assign57720_e94219_d_n5, assign57720_e94219_d_n6, assign57720_e94219_d_n7, assign57720_e94219_d_n8, assign57720_e94219_d_n9, assign57720_e94219_d_n10, assign57720_e94219_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57720_e94219;
        locals.var_t5_dn3 = assign57720_e94219_d_n3;
        locals.var_t5_dn4 = assign57720_e94219_d_n4;
        locals.var_t5_dn5 = assign57720_e94219_d_n5;
        locals.var_t5_dn6 = assign57720_e94219_d_n6;
        locals.var_t5_dn7 = assign57720_e94219_d_n7;
        locals.var_t5_dn8 = assign57720_e94219_d_n8;
        locals.var_t5_dn9 = assign57720_e94219_d_n9;
        locals.var_t5_dn10 = assign57720_e94219_d_n10;
        locals.var_t5_dn11 = assign57720_e94219_d_n11;
        locals.var_t5_rv = 0.0;

        let assign57730_e94224: f64 = (0.5 * locals.var_t5);
        let assign57730_e94225: f64 = (locals.var_t4 - assign57730_e94224);
        let assign57730_e94226: f64 = if locals.var_t8 < assign57730_e94225 { 1.0 } else { 0.0 };
        locals.var_guard859 = assign57730_e94226;
        locals.var_guard859_rv = 0.0;

        let (assign57740_e94238, assign57740_e94238_d_n3, assign57740_e94238_d_n4, assign57740_e94238_d_n5, assign57740_e94238_d_n6, assign57740_e94238_d_n7, assign57740_e94238_d_n8, assign57740_e94238_d_n9, assign57740_e94238_d_n10, assign57740_e94238_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) && (locals.var_guard859 != 0.0)) {
        let assign57740_e94236: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57740_e94236, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57740_e94238;
        locals.var_t3_dn3 = assign57740_e94238_d_n3;
        locals.var_t3_dn4 = assign57740_e94238_d_n4;
        locals.var_t3_dn5 = assign57740_e94238_d_n5;
        locals.var_t3_dn6 = assign57740_e94238_d_n6;
        locals.var_t3_dn7 = assign57740_e94238_d_n7;
        locals.var_t3_dn8 = assign57740_e94238_d_n8;
        locals.var_t3_dn9 = assign57740_e94238_d_n9;
        locals.var_t3_dn10 = assign57740_e94238_d_n10;
        locals.var_t3_dn11 = assign57740_e94238_d_n11;
        locals.var_t3_rv = 0.0;

        let assign57750_e94243: f64 = (0.5 * locals.var_t5);
        let assign57750_e94244: f64 = (locals.var_t4 + assign57750_e94243);
        let assign57750_e94245: f64 = if locals.var_t8 > assign57750_e94244 { 1.0 } else { 0.0 };
        locals.var_guard860 = assign57750_e94245;
        locals.var_guard860_rv = 0.0;

        let (assign57760_e94260, assign57760_e94260_d_n3, assign57760_e94260_d_n4, assign57760_e94260_d_n5, assign57760_e94260_d_n6, assign57760_e94260_d_n7, assign57760_e94260_d_n8, assign57760_e94260_d_n9, assign57760_e94260_d_n10, assign57760_e94260_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) && (locals.var_guard859 == 0.0)) && (locals.var_guard860 != 0.0)) {
        let assign57760_e94258: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57760_e94258, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57760_e94260;
        locals.var_t3_dn3 = assign57760_e94260_d_n3;
        locals.var_t3_dn4 = assign57760_e94260_d_n4;
        locals.var_t3_dn5 = assign57760_e94260_d_n5;
        locals.var_t3_dn6 = assign57760_e94260_d_n6;
        locals.var_t3_dn7 = assign57760_e94260_d_n7;
        locals.var_t3_dn8 = assign57760_e94260_d_n8;
        locals.var_t3_dn9 = assign57760_e94260_d_n9;
        locals.var_t3_dn10 = assign57760_e94260_d_n10;
        locals.var_t3_dn11 = assign57760_e94260_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57770_e94279, assign57770_e94279_d_n3, assign57770_e94279_d_n4, assign57770_e94279_d_n5, assign57770_e94279_d_n6, assign57770_e94279_d_n7, assign57770_e94279_d_n8, assign57770_e94279_d_n9, assign57770_e94279_d_n10, assign57770_e94279_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) && (locals.var_guard859 == 0.0)) && (locals.var_guard860 == 0.0)) {
        let assign57770_e94275: f64 = (locals.var_t8 - locals.var_t4);
        let assign57770_e94277: f64 = (assign57770_e94275 / locals.var_t5);
        (assign57770_e94277, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign57770_e94275 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign57770_e94279;
        locals.var_t2_dn3 = assign57770_e94279_d_n3;
        locals.var_t2_dn4 = assign57770_e94279_d_n4;
        locals.var_t2_dn5 = assign57770_e94279_d_n5;
        locals.var_t2_dn6 = assign57770_e94279_d_n6;
        locals.var_t2_dn7 = assign57770_e94279_d_n7;
        locals.var_t2_dn8 = assign57770_e94279_d_n8;
        locals.var_t2_dn9 = assign57770_e94279_d_n9;
        locals.var_t2_dn10 = assign57770_e94279_d_n10;
        locals.var_t2_dn11 = assign57770_e94279_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign57780_e94296, assign57780_e94296_d_n3, assign57780_e94296_d_n4, assign57780_e94296_d_n5, assign57780_e94296_d_n6, assign57780_e94296_d_n7, assign57780_e94296_d_n8, assign57780_e94296_d_n9, assign57780_e94296_d_n10, assign57780_e94296_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) && (locals.var_guard859 == 0.0)) && (locals.var_guard860 == 0.0)) {
        let assign57780_e94294: f64 = (locals.var_t2 * locals.var_t2);
        (assign57780_e94294, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign57780_e94296;
        locals.var_t6_dn3 = assign57780_e94296_d_n3;
        locals.var_t6_dn4 = assign57780_e94296_d_n4;
        locals.var_t6_dn5 = assign57780_e94296_d_n5;
        locals.var_t6_dn6 = assign57780_e94296_d_n6;
        locals.var_t6_dn7 = assign57780_e94296_d_n7;
        locals.var_t6_dn8 = assign57780_e94296_d_n8;
        locals.var_t6_dn9 = assign57780_e94296_d_n9;
        locals.var_t6_dn10 = assign57780_e94296_d_n10;
        locals.var_t6_dn11 = assign57780_e94296_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign57790_e94334, assign57790_e94334_d_n3, assign57790_e94334_d_n4, assign57790_e94334_d_n5, assign57790_e94334_d_n6, assign57790_e94334_d_n7, assign57790_e94334_d_n8, assign57790_e94334_d_n9, assign57790_e94334_d_n10, assign57790_e94334_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) && (locals.var_guard859 == 0.0)) && (locals.var_guard860 == 0.0)) {
        let assign57790_e94313: f64 = (5.0 / 64.0);
        let assign57790_e94316: f64 = (0.5 * locals.var_t2);
        let assign57790_e94317: f64 = (assign57790_e94313 + assign57790_e94316);
        let assign57790_e94321: f64 = (15.0 / 16.0);
        let assign57790_e94325: f64 = (1.25 - locals.var_t6);
        let assign57790_e94326: f64 = (locals.var_t6 * assign57790_e94325);
        let assign57790_e94327: f64 = (assign57790_e94321 - assign57790_e94326);
        let assign57790_e94328: f64 = (locals.var_t6 * assign57790_e94327);
        let assign57790_e94329: f64 = (assign57790_e94317 + assign57790_e94328);
        let assign57790_e94330: f64 = (locals.var_t5 * assign57790_e94329);
        let assign57790_e94331: f64 = (locals.var_t4 + assign57790_e94330);
        let assign57790_e94332: f64 = { let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57790_e94332, ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign57790_e94331; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign57790_e94329) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign57790_e94327) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign57790_e94325) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57790_e94334;
        locals.var_t3_dn3 = assign57790_e94334_d_n3;
        locals.var_t3_dn4 = assign57790_e94334_d_n4;
        locals.var_t3_dn5 = assign57790_e94334_d_n5;
        locals.var_t3_dn6 = assign57790_e94334_d_n6;
        locals.var_t3_dn7 = assign57790_e94334_d_n7;
        locals.var_t3_dn8 = assign57790_e94334_d_n8;
        locals.var_t3_dn9 = assign57790_e94334_d_n9;
        locals.var_t3_dn10 = assign57790_e94334_d_n10;
        locals.var_t3_dn11 = assign57790_e94334_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57800_e94366, assign57800_e94366_d_n3, assign57800_e94366_d_n4, assign57800_e94366_d_n5, assign57800_e94366_d_n6, assign57800_e94366_d_n7, assign57800_e94366_d_n8, assign57800_e94366_d_n9, assign57800_e94366_d_n10, assign57800_e94366_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 != 0.0)) {
        let assign57800_e94344: f64 = (1.0 + locals.var_t1);
        let assign57800_e94346: f64 = (assign57800_e94344 - locals.var_t8);
        let assign57800_e94349: f64 = (2.0 * locals.var_t0);
        let assign57800_e94352: f64 = (locals.var_t3 * 2.0);
        let assign57800_e94354: f64 = (assign57800_e94352 * locals.var_t0);
        let assign57800_e94357: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign57800_e94358: f64 = (assign57800_e94354 + assign57800_e94357);
        let assign57800_e94359: f64 = (assign57800_e94349 * assign57800_e94358);
        let assign57800_e94361: f64 = (assign57800_e94359).max(1e-38);
        let assign57800_e94362: f64 = (assign57800_e94361).ln();
        let assign57800_e94363: f64 = (assign57800_e94346 - assign57800_e94362);
        let assign57800_e94364: f64 = (locals.var_t3 * assign57800_e94363);
        (assign57800_e94364, ((locals.var_t3_dn3 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn4 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn5 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn6 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn7 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn8 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn9 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn10 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign57800_e94361)))), ((locals.var_t3_dn11 * assign57800_e94363) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign57800_e94359 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign57800_e94358) + (assign57800_e94349 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57800_e94352 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign57800_e94361)))),)
    } else {
        (locals.var_qdeff, locals.var_qdeff_dn3, locals.var_qdeff_dn4, locals.var_qdeff_dn5, locals.var_qdeff_dn6, locals.var_qdeff_dn7, locals.var_qdeff_dn8, locals.var_qdeff_dn9, locals.var_qdeff_dn10, locals.var_qdeff_dn11,)
    }
};
        locals.var_qdeff = assign57800_e94366;
        locals.var_qdeff_dn3 = assign57800_e94366_d_n3;
        locals.var_qdeff_dn4 = assign57800_e94366_d_n4;
        locals.var_qdeff_dn5 = assign57800_e94366_d_n5;
        locals.var_qdeff_dn6 = assign57800_e94366_d_n6;
        locals.var_qdeff_dn7 = assign57800_e94366_d_n7;
        locals.var_qdeff_dn8 = assign57800_e94366_d_n8;
        locals.var_qdeff_dn9 = assign57800_e94366_d_n9;
        locals.var_qdeff_dn10 = assign57800_e94366_d_n10;
        locals.var_qdeff_dn11 = assign57800_e94366_d_n11;
        locals.var_qdeff_rv = 0.0;

        let (assign57810_e94377, assign57810_e94377_d_n3, assign57810_e94377_d_n4, assign57810_e94377_d_n5, assign57810_e94377_d_n6, assign57810_e94377_d_n7, assign57810_e94377_d_n8, assign57810_e94377_d_n9, assign57810_e94377_d_n10, assign57810_e94377_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57810_e94375: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign57810_e94375, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57810_e94377;
        locals.var_t3_dn3 = assign57810_e94377_d_n3;
        locals.var_t3_dn4 = assign57810_e94377_d_n4;
        locals.var_t3_dn5 = assign57810_e94377_d_n5;
        locals.var_t3_dn6 = assign57810_e94377_d_n6;
        locals.var_t3_dn7 = assign57810_e94377_d_n7;
        locals.var_t3_dn8 = assign57810_e94377_d_n8;
        locals.var_t3_dn9 = assign57810_e94377_d_n9;
        locals.var_t3_dn10 = assign57810_e94377_d_n10;
        locals.var_t3_dn11 = assign57810_e94377_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57820_e94389, assign57820_e94389_d_n3, assign57820_e94389_d_n4, assign57820_e94389_d_n5, assign57820_e94389_d_n6, assign57820_e94389_d_n7, assign57820_e94389_d_n8, assign57820_e94389_d_n9, assign57820_e94389_d_n10, assign57820_e94389_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57820_e94387: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign57820_e94387, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign57820_e94389;
        locals.var_sqrtpsisainv_dn3 = assign57820_e94389_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign57820_e94389_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign57820_e94389_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign57820_e94389_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign57820_e94389_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign57820_e94389_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign57820_e94389_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign57820_e94389_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign57820_e94389_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign57830_e94422, assign57830_e94422_d_n3, assign57830_e94422_d_n4, assign57830_e94422_d_n5, assign57830_e94422_d_n6, assign57830_e94422_d_n7, assign57830_e94422_d_n8, assign57830_e94422_d_n9, assign57830_e94422_d_n10, assign57830_e94422_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57830_e94399: f64 = (2.0 * locals.var_t3);
        let assign57830_e94402: f64 = (locals.var_t3 * 2.0);
        let assign57830_e94404: f64 = (assign57830_e94402 * locals.var_t0);
        let assign57830_e94407: f64 = (locals.var_t3 * 2.0);
        let assign57830_e94409: f64 = (assign57830_e94407 * locals.var_t0);
        let assign57830_e94412: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign57830_e94413: f64 = (assign57830_e94409 + assign57830_e94412);
        let assign57830_e94414: f64 = (assign57830_e94404 * assign57830_e94413);
        let assign57830_e94416: f64 = (assign57830_e94414).max(1e-38);
        let assign57830_e94417: f64 = (assign57830_e94416).ln();
        let assign57830_e94418: f64 = (assign57830_e94399 + assign57830_e94417);
        let assign57830_e94420: f64 = (assign57830_e94418 - locals.var_t1);
        (assign57830_e94420, (((2.0 * locals.var_t3_dn3) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn3)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn4)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn5)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn6)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn7)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn8)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn9)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn10)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign57830_e94414 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57830_e94402 * locals.var_t0_dn11)) * assign57830_e94413) + (assign57830_e94404 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57830_e94407 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign57830_e94416)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57830_e94422;
        locals.var_t4_dn3 = assign57830_e94422_d_n3;
        locals.var_t4_dn4 = assign57830_e94422_d_n4;
        locals.var_t4_dn5 = assign57830_e94422_d_n5;
        locals.var_t4_dn6 = assign57830_e94422_d_n6;
        locals.var_t4_dn7 = assign57830_e94422_d_n7;
        locals.var_t4_dn8 = assign57830_e94422_d_n8;
        locals.var_t4_dn9 = assign57830_e94422_d_n9;
        locals.var_t4_dn10 = assign57830_e94422_d_n10;
        locals.var_t4_dn11 = assign57830_e94422_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57840_e94446, assign57840_e94446_d_n3, assign57840_e94446_d_n4, assign57840_e94446_d_n5, assign57840_e94446_d_n6, assign57840_e94446_d_n7, assign57840_e94446_d_n8, assign57840_e94446_d_n9, assign57840_e94446_d_n10, assign57840_e94446_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57840_e94433: f64 = (1.0 / locals.var_t3);
        let assign57840_e94434: f64 = (2.0 + assign57840_e94433);
        let assign57840_e94437: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57840_e94440: f64 = (locals.var_t0 * locals.var_t3);
        let assign57840_e94442: f64 = (assign57840_e94440 + locals.var_sqrtpsisa);
        let assign57840_e94443: f64 = (assign57840_e94437 / assign57840_e94442);
        let assign57840_e94444: f64 = (assign57840_e94434 + assign57840_e94443);
        (assign57840_e94444, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57840_e94442 * assign57840_e94442))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57840_e94442) - (assign57840_e94437 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57840_e94442 * assign57840_e94442))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57840_e94446;
        locals.var_t5_dn3 = assign57840_e94446_d_n3;
        locals.var_t5_dn4 = assign57840_e94446_d_n4;
        locals.var_t5_dn5 = assign57840_e94446_d_n5;
        locals.var_t5_dn6 = assign57840_e94446_d_n6;
        locals.var_t5_dn7 = assign57840_e94446_d_n7;
        locals.var_t5_dn8 = assign57840_e94446_d_n8;
        locals.var_t5_dn9 = assign57840_e94446_d_n9;
        locals.var_t5_dn10 = assign57840_e94446_d_n10;
        locals.var_t5_dn11 = assign57840_e94446_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_201(
        locals: &mut StampLocals,
    ) {
        let (assign57850_e94460, assign57850_e94460_d_n3, assign57850_e94460_d_n4, assign57850_e94460_d_n5, assign57850_e94460_d_n6, assign57850_e94460_d_n7, assign57850_e94460_d_n8, assign57850_e94460_d_n9, assign57850_e94460_d_n10, assign57850_e94460_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57850_e94457: f64 = (locals.var_t4 / locals.var_t5);
        let assign57850_e94458: f64 = (locals.var_t3 - assign57850_e94457);
        (assign57850_e94458, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57850_e94460;
        locals.var_t3_dn3 = assign57850_e94460_d_n3;
        locals.var_t3_dn4 = assign57850_e94460_d_n4;
        locals.var_t3_dn5 = assign57850_e94460_d_n5;
        locals.var_t3_dn6 = assign57850_e94460_d_n6;
        locals.var_t3_dn7 = assign57850_e94460_d_n7;
        locals.var_t3_dn8 = assign57850_e94460_d_n8;
        locals.var_t3_dn9 = assign57850_e94460_d_n9;
        locals.var_t3_dn10 = assign57850_e94460_d_n10;
        locals.var_t3_dn11 = assign57850_e94460_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57860_e94493, assign57860_e94493_d_n3, assign57860_e94493_d_n4, assign57860_e94493_d_n5, assign57860_e94493_d_n6, assign57860_e94493_d_n7, assign57860_e94493_d_n8, assign57860_e94493_d_n9, assign57860_e94493_d_n10, assign57860_e94493_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57860_e94470: f64 = (2.0 * locals.var_t3);
        let assign57860_e94473: f64 = (locals.var_t3 * 2.0);
        let assign57860_e94475: f64 = (assign57860_e94473 * locals.var_t0);
        let assign57860_e94478: f64 = (locals.var_t3 * 2.0);
        let assign57860_e94480: f64 = (assign57860_e94478 * locals.var_t0);
        let assign57860_e94483: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign57860_e94484: f64 = (assign57860_e94480 + assign57860_e94483);
        let assign57860_e94485: f64 = (assign57860_e94475 * assign57860_e94484);
        let assign57860_e94487: f64 = (assign57860_e94485).max(1e-38);
        let assign57860_e94488: f64 = (assign57860_e94487).ln();
        let assign57860_e94489: f64 = (assign57860_e94470 + assign57860_e94488);
        let assign57860_e94491: f64 = (assign57860_e94489 - locals.var_t1);
        (assign57860_e94491, (((2.0 * locals.var_t3_dn3) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn3)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn4)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn5)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn6)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn7)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn8)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn9)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn10)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign57860_e94485 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57860_e94473 * locals.var_t0_dn11)) * assign57860_e94484) + (assign57860_e94475 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign57860_e94478 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign57860_e94487)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57860_e94493;
        locals.var_t4_dn3 = assign57860_e94493_d_n3;
        locals.var_t4_dn4 = assign57860_e94493_d_n4;
        locals.var_t4_dn5 = assign57860_e94493_d_n5;
        locals.var_t4_dn6 = assign57860_e94493_d_n6;
        locals.var_t4_dn7 = assign57860_e94493_d_n7;
        locals.var_t4_dn8 = assign57860_e94493_d_n8;
        locals.var_t4_dn9 = assign57860_e94493_d_n9;
        locals.var_t4_dn10 = assign57860_e94493_d_n10;
        locals.var_t4_dn11 = assign57860_e94493_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57870_e94517, assign57870_e94517_d_n3, assign57870_e94517_d_n4, assign57870_e94517_d_n5, assign57870_e94517_d_n6, assign57870_e94517_d_n7, assign57870_e94517_d_n8, assign57870_e94517_d_n9, assign57870_e94517_d_n10, assign57870_e94517_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57870_e94504: f64 = (1.0 / locals.var_t3);
        let assign57870_e94505: f64 = (2.0 + assign57870_e94504);
        let assign57870_e94508: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57870_e94511: f64 = (locals.var_t0 * locals.var_t3);
        let assign57870_e94513: f64 = (assign57870_e94511 + locals.var_sqrtpsisa);
        let assign57870_e94514: f64 = (assign57870_e94508 / assign57870_e94513);
        let assign57870_e94515: f64 = (assign57870_e94505 + assign57870_e94514);
        (assign57870_e94515, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57870_e94513 * assign57870_e94513))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57870_e94513) - (assign57870_e94508 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57870_e94513 * assign57870_e94513))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57870_e94517;
        locals.var_t5_dn3 = assign57870_e94517_d_n3;
        locals.var_t5_dn4 = assign57870_e94517_d_n4;
        locals.var_t5_dn5 = assign57870_e94517_d_n5;
        locals.var_t5_dn6 = assign57870_e94517_d_n6;
        locals.var_t5_dn7 = assign57870_e94517_d_n7;
        locals.var_t5_dn8 = assign57870_e94517_d_n8;
        locals.var_t5_dn9 = assign57870_e94517_d_n9;
        locals.var_t5_dn10 = assign57870_e94517_d_n10;
        locals.var_t5_dn11 = assign57870_e94517_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign57880_e94545, assign57880_e94545_d_n3, assign57880_e94545_d_n4, assign57880_e94545_d_n5, assign57880_e94545_d_n6, assign57880_e94545_d_n7, assign57880_e94545_d_n8, assign57880_e94545_d_n9, assign57880_e94545_d_n10, assign57880_e94545_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57880_e94527: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57880_e94530: f64 = (locals.var_t0 * locals.var_t3);
        let assign57880_e94532: f64 = (assign57880_e94530 + locals.var_sqrtpsisa);
        let assign57880_e94533: f64 = (assign57880_e94527 / assign57880_e94532);
        let assign57880_e94536: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign57880_e94539: f64 = (locals.var_t0 * locals.var_t3);
        let assign57880_e94541: f64 = (assign57880_e94539 + locals.var_sqrtpsisa);
        let assign57880_e94542: f64 = (assign57880_e94536 / assign57880_e94541);
        let assign57880_e94543: f64 = (assign57880_e94533 * assign57880_e94542);
        (assign57880_e94543, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57880_e94541 * assign57880_e94541)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57880_e94532) - (assign57880_e94527 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57880_e94532 * assign57880_e94532)) * assign57880_e94542) + (assign57880_e94533 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign57880_e94541) - (assign57880_e94536 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57880_e94541 * assign57880_e94541)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign57880_e94545;
        locals.var_t6_dn3 = assign57880_e94545_d_n3;
        locals.var_t6_dn4 = assign57880_e94545_d_n4;
        locals.var_t6_dn5 = assign57880_e94545_d_n5;
        locals.var_t6_dn6 = assign57880_e94545_d_n6;
        locals.var_t6_dn7 = assign57880_e94545_d_n7;
        locals.var_t6_dn8 = assign57880_e94545_d_n8;
        locals.var_t6_dn9 = assign57880_e94545_d_n9;
        locals.var_t6_dn10 = assign57880_e94545_d_n10;
        locals.var_t6_dn11 = assign57880_e94545_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign57890_e94578, assign57890_e94578_d_n3, assign57890_e94578_d_n4, assign57890_e94578_d_n5, assign57890_e94578_d_n6, assign57890_e94578_d_n7, assign57890_e94578_d_n8, assign57890_e94578_d_n9, assign57890_e94578_d_n10, assign57890_e94578_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign57890_e94555: f64 = (1.0 * __rspice_inv_cse_0);
        let assign57890_e94558: f64 = (1.0 * __rspice_inv_cse_0);
        let assign57890_e94559: f64 = (assign57890_e94555 * assign57890_e94558);
        let assign57890_e94560: f64 = (-assign57890_e94559);
        let assign57890_e94564: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign57890_e94566: f64 = (assign57890_e94564 * locals.var_sqrtpsisa);
        let assign57890_e94569: f64 = (locals.var_t0 * locals.var_t3);
        let assign57890_e94571: f64 = (assign57890_e94569 + locals.var_sqrtpsisa);
        let assign57890_e94572: f64 = (assign57890_e94566 * assign57890_e94571);
        let assign57890_e94573: f64 = (1.0 / assign57890_e94572);
        let assign57890_e94574: f64 = (assign57890_e94560 - assign57890_e94573);
        let assign57890_e94576: f64 = (assign57890_e94574 - locals.var_t6);
        (assign57890_e94576, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn3)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn4)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn5)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn6)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn7)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn8)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn9)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn10)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign57890_e94558) + (assign57890_e94555 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign57890_e94564 * locals.var_sqrtpsisa_dn11)) * assign57890_e94571) + (assign57890_e94566 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign57890_e94572 * assign57890_e94572)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign57890_e94578;
        locals.var_t7_dn3 = assign57890_e94578_d_n3;
        locals.var_t7_dn4 = assign57890_e94578_d_n4;
        locals.var_t7_dn5 = assign57890_e94578_d_n5;
        locals.var_t7_dn6 = assign57890_e94578_d_n6;
        locals.var_t7_dn7 = assign57890_e94578_d_n7;
        locals.var_t7_dn8 = assign57890_e94578_d_n8;
        locals.var_t7_dn9 = assign57890_e94578_d_n9;
        locals.var_t7_dn10 = assign57890_e94578_d_n10;
        locals.var_t7_dn11 = assign57890_e94578_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign57900_e94604, assign57900_e94604_d_n3, assign57900_e94604_d_n4, assign57900_e94604_d_n5, assign57900_e94604_d_n6, assign57900_e94604_d_n7, assign57900_e94604_d_n8, assign57900_e94604_d_n9, assign57900_e94604_d_n10, assign57900_e94604_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) && (locals.var_guard858 == 0.0)) {
        let assign57900_e94589: f64 = (locals.var_t4 / locals.var_t5);
        let assign57900_e94593: f64 = (locals.var_t4 * locals.var_t7);
        let assign57900_e94596: f64 = (2.0 * locals.var_t5);
        let assign57900_e94598: f64 = (assign57900_e94596 * locals.var_t5);
        let assign57900_e94599: f64 = (assign57900_e94593 / assign57900_e94598);
        let assign57900_e94600: f64 = (1.0 + assign57900_e94599);
        let assign57900_e94601: f64 = (assign57900_e94589 * assign57900_e94600);
        let assign57900_e94602: f64 = (locals.var_t3 - assign57900_e94601);
        (assign57900_e94602, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn3)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn4)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn5)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn6)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn7)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn8)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn9)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn10)))) / (assign57900_e94598 * assign57900_e94598))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign57900_e94600) + (assign57900_e94589 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign57900_e94598) - (assign57900_e94593 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign57900_e94596 * locals.var_t5_dn11)))) / (assign57900_e94598 * assign57900_e94598))))),)
    } else {
        (locals.var_qdeff, locals.var_qdeff_dn3, locals.var_qdeff_dn4, locals.var_qdeff_dn5, locals.var_qdeff_dn6, locals.var_qdeff_dn7, locals.var_qdeff_dn8, locals.var_qdeff_dn9, locals.var_qdeff_dn10, locals.var_qdeff_dn11,)
    }
};
        locals.var_qdeff = assign57900_e94604;
        locals.var_qdeff_dn3 = assign57900_e94604_d_n3;
        locals.var_qdeff_dn4 = assign57900_e94604_d_n4;
        locals.var_qdeff_dn5 = assign57900_e94604_d_n5;
        locals.var_qdeff_dn6 = assign57900_e94604_d_n6;
        locals.var_qdeff_dn7 = assign57900_e94604_d_n7;
        locals.var_qdeff_dn8 = assign57900_e94604_d_n8;
        locals.var_qdeff_dn9 = assign57900_e94604_d_n9;
        locals.var_qdeff_dn10 = assign57900_e94604_d_n10;
        locals.var_qdeff_dn11 = assign57900_e94604_d_n11;
        locals.var_qdeff_rv = 0.0;

        let (assign57910_e94617, assign57910_e94617_d_n3, assign57910_e94617_d_n4, assign57910_e94617_d_n5, assign57910_e94617_d_n6, assign57910_e94617_d_n7, assign57910_e94617_d_n8, assign57910_e94617_d_n9, assign57910_e94617_d_n10, assign57910_e94617_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57910_e94611: f64 = (locals.var_psip - locals.var_qs_1);
        let assign57910_e94613: f64 = (assign57910_e94611 - locals.var_qdeff);
        let assign57910_e94615: f64 = (assign57910_e94613 - 1.0);
        (assign57910_e94615, ((locals.var_psip_dn3 - locals.var_qs_1_dn3) - locals.var_qdeff_dn3), ((locals.var_psip_dn4 - locals.var_qs_1_dn4) - locals.var_qdeff_dn4), ((locals.var_psip_dn5 - locals.var_qs_1_dn5) - locals.var_qdeff_dn5), ((locals.var_psip_dn6 - locals.var_qs_1_dn6) - locals.var_qdeff_dn6), ((locals.var_psip_dn7 - locals.var_qs_1_dn7) - locals.var_qdeff_dn7), ((locals.var_psip_dn8 - locals.var_qs_1_dn8) - locals.var_qdeff_dn8), ((locals.var_psip_dn9 - locals.var_qs_1_dn9) - locals.var_qdeff_dn9), ((locals.var_psip_dn10 - locals.var_qs_1_dn10) - locals.var_qdeff_dn10), ((locals.var_psip_dn11 - locals.var_qs_1_dn11) - locals.var_qdeff_dn11),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign57910_e94617;
        locals.var_psiavg_dn3 = assign57910_e94617_d_n3;
        locals.var_psiavg_dn4 = assign57910_e94617_d_n4;
        locals.var_psiavg_dn5 = assign57910_e94617_d_n5;
        locals.var_psiavg_dn6 = assign57910_e94617_d_n6;
        locals.var_psiavg_dn7 = assign57910_e94617_d_n7;
        locals.var_psiavg_dn8 = assign57910_e94617_d_n8;
        locals.var_psiavg_dn9 = assign57910_e94617_d_n9;
        locals.var_psiavg_dn10 = assign57910_e94617_d_n10;
        locals.var_psiavg_dn11 = assign57910_e94617_d_n11;
        locals.var_psiavg_rv = 0.0;

        let (assign57920_e94643, assign57920_e94643_d_n3, assign57920_e94643_d_n4, assign57920_e94643_d_n5, assign57920_e94643_d_n6, assign57920_e94643_d_n7, assign57920_e94643_d_n8, assign57920_e94643_d_n9, assign57920_e94643_d_n10, assign57920_e94643_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57920_e94625: f64 = (locals.var_psiavg + 1.0);
        let assign57920_e94628: f64 = (locals.var_psiavg - 1.0);
        let assign57920_e94631: f64 = (locals.var_psiavg - 1.0);
        let assign57920_e94632: f64 = (assign57920_e94628 * assign57920_e94631);
        let assign57920_e94635: f64 = (0.25 * 2.0);
        let assign57920_e94637: f64 = (assign57920_e94635 * 2.0);
        let assign57920_e94638: f64 = (assign57920_e94632 + assign57920_e94637);
        let assign57920_e94639: f64 = (assign57920_e94638).sqrt();
        let assign57920_e94640: f64 = (assign57920_e94625 + assign57920_e94639);
        let assign57920_e94641: f64 = (0.5 * assign57920_e94640);
        (assign57920_e94641, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn3)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn4)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn5)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn6)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn7)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn8)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn9)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn10)) / (2.0 * assign57920_e94639)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign57920_e94631) + (assign57920_e94628 * locals.var_psiavg_dn11)) / (2.0 * assign57920_e94639)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign57920_e94643;
        locals.var_t0_dn3 = assign57920_e94643_d_n3;
        locals.var_t0_dn4 = assign57920_e94643_d_n4;
        locals.var_t0_dn5 = assign57920_e94643_d_n5;
        locals.var_t0_dn6 = assign57920_e94643_d_n6;
        locals.var_t0_dn7 = assign57920_e94643_d_n7;
        locals.var_t0_dn8 = assign57920_e94643_d_n8;
        locals.var_t0_dn9 = assign57920_e94643_d_n9;
        locals.var_t0_dn10 = assign57920_e94643_d_n10;
        locals.var_t0_dn11 = assign57920_e94643_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign57930_e94651, assign57930_e94651_d_n3, assign57930_e94651_d_n4, assign57930_e94651_d_n5, assign57930_e94651_d_n6, assign57930_e94651_d_n7, assign57930_e94651_d_n8, assign57930_e94651_d_n9, assign57930_e94651_d_n10, assign57930_e94651_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57930_e94649: f64 = (locals.var_t0).sqrt();
        (assign57930_e94649, (locals.var_t0_dn3 / (2.0 * assign57930_e94649)), (locals.var_t0_dn4 / (2.0 * assign57930_e94649)), (locals.var_t0_dn5 / (2.0 * assign57930_e94649)), (locals.var_t0_dn6 / (2.0 * assign57930_e94649)), (locals.var_t0_dn7 / (2.0 * assign57930_e94649)), (locals.var_t0_dn8 / (2.0 * assign57930_e94649)), (locals.var_t0_dn9 / (2.0 * assign57930_e94649)), (locals.var_t0_dn10 / (2.0 * assign57930_e94649)), (locals.var_t0_dn11 / (2.0 * assign57930_e94649)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign57930_e94651;
        locals.var_t2_dn3 = assign57930_e94651_d_n3;
        locals.var_t2_dn4 = assign57930_e94651_d_n4;
        locals.var_t2_dn5 = assign57930_e94651_d_n5;
        locals.var_t2_dn6 = assign57930_e94651_d_n6;
        locals.var_t2_dn7 = assign57930_e94651_d_n7;
        locals.var_t2_dn8 = assign57930_e94651_d_n8;
        locals.var_t2_dn9 = assign57930_e94651_d_n9;
        locals.var_t2_dn10 = assign57930_e94651_d_n10;
        locals.var_t2_dn11 = assign57930_e94651_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign57940_e94666, assign57940_e94666_d_n3, assign57940_e94666_d_n4, assign57940_e94666_d_n5, assign57940_e94666_d_n6, assign57940_e94666_d_n7, assign57940_e94666_d_n8, assign57940_e94666_d_n9, assign57940_e94666_d_n10, assign57940_e94666_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57940_e94658: f64 = (1.0 + locals.var_dpd);
        let assign57940_e94662: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign57940_e94663: f64 = (locals.var_gamagbcp2 / assign57940_e94662);
        let assign57940_e94664: f64 = (assign57940_e94658 + assign57940_e94663);
        (assign57940_e94664, (locals.var_dpd_dn3 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3)) / (assign57940_e94662 * assign57940_e94662)))), (locals.var_dpd_dn4 + (((locals.var_gamagbcp2_dn4 * assign57940_e94662) - (locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign57940_e94662 * assign57940_e94662))), (locals.var_dpd_dn5 + (((locals.var_gamagbcp2_dn5 * assign57940_e94662) - (locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign57940_e94662 * assign57940_e94662))), (locals.var_dpd_dn6 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6)) / (assign57940_e94662 * assign57940_e94662)))), (locals.var_dpd_dn7 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7)) / (assign57940_e94662 * assign57940_e94662)))), (locals.var_dpd_dn8 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8)) / (assign57940_e94662 * assign57940_e94662)))), (locals.var_dpd_dn9 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9)) / (assign57940_e94662 * assign57940_e94662)))), (locals.var_dpd_dn10 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10)) / (assign57940_e94662 * assign57940_e94662)))), (locals.var_dpd_dn11 + (-((locals.var_gamagbcp2 * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11)) / (assign57940_e94662 * assign57940_e94662)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign57940_e94666;
        locals.var_t3_dn3 = assign57940_e94666_d_n3;
        locals.var_t3_dn4 = assign57940_e94666_d_n4;
        locals.var_t3_dn5 = assign57940_e94666_d_n5;
        locals.var_t3_dn6 = assign57940_e94666_d_n6;
        locals.var_t3_dn7 = assign57940_e94666_d_n7;
        locals.var_t3_dn8 = assign57940_e94666_d_n8;
        locals.var_t3_dn9 = assign57940_e94666_d_n9;
        locals.var_t3_dn10 = assign57940_e94666_d_n10;
        locals.var_t3_dn11 = assign57940_e94666_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign57950_e94679, assign57950_e94679_d_n3, assign57950_e94679_d_n4, assign57950_e94679_d_n5, assign57950_e94679_d_n6, assign57950_e94679_d_n7, assign57950_e94679_d_n8, assign57950_e94679_d_n9, assign57950_e94679_d_n10, assign57950_e94679_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57950_e94674: f64 = (locals.var_dpd * locals.var_t2);
        let assign57950_e94676: f64 = (assign57950_e94674 * locals.var_inv_gam);
        let assign57950_e94677: f64 = (0.5 + assign57950_e94676);
        (assign57950_e94677, ((((locals.var_dpd_dn3 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn3)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn3)), ((((locals.var_dpd_dn4 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn4)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn4)), ((((locals.var_dpd_dn5 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn5)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn5)), ((((locals.var_dpd_dn6 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn6)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn6)), ((((locals.var_dpd_dn7 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn7)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn7)), ((((locals.var_dpd_dn8 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn8)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn8)), ((((locals.var_dpd_dn9 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn9)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn9)), ((((locals.var_dpd_dn10 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn10)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn10)), ((((locals.var_dpd_dn11 * locals.var_t2) + (locals.var_dpd * locals.var_t2_dn11)) * locals.var_inv_gam) + (assign57950_e94674 * locals.var_inv_gam_dn11)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign57950_e94679;
        locals.var_t4_dn3 = assign57950_e94679_d_n3;
        locals.var_t4_dn4 = assign57950_e94679_d_n4;
        locals.var_t4_dn5 = assign57950_e94679_d_n5;
        locals.var_t4_dn6 = assign57950_e94679_d_n6;
        locals.var_t4_dn7 = assign57950_e94679_d_n7;
        locals.var_t4_dn8 = assign57950_e94679_d_n8;
        locals.var_t4_dn9 = assign57950_e94679_d_n9;
        locals.var_t4_dn10 = assign57950_e94679_d_n10;
        locals.var_t4_dn11 = assign57950_e94679_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign57960_e94697, assign57960_e94697_d_n3, assign57960_e94697_d_n4, assign57960_e94697_d_n5, assign57960_e94697_d_n6, assign57960_e94697_d_n7, assign57960_e94697_d_n8, assign57960_e94697_d_n9, assign57960_e94697_d_n10, assign57960_e94697_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57960_e94686: f64 = (locals.var_t4 * locals.var_t4);
        let assign57960_e94690: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign57960_e94691: f64 = (locals.var_t3 * assign57960_e94690);
        let assign57960_e94693: f64 = (assign57960_e94691 * locals.var_invgamg2);
        let assign57960_e94694: f64 = (assign57960_e94686 + assign57960_e94693);
        let assign57960_e94695: f64 = (assign57960_e94694).sqrt();
        (assign57960_e94695, ((((locals.var_t4_dn3 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn3)) + (((locals.var_t3_dn3 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) + ((((locals.var_t3_dn4 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) * locals.var_invgamg2) + (assign57960_e94691 * locals.var_invgamg2_dn4))) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) + ((((locals.var_t3_dn5 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) * locals.var_invgamg2) + (assign57960_e94691 * locals.var_invgamg2_dn5))) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) + (((locals.var_t3_dn6 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)) + (((locals.var_t3_dn7 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) + (((locals.var_t3_dn8 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)) + (((locals.var_t3_dn9 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) + (((locals.var_t3_dn10 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)), ((((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) + (((locals.var_t3_dn11 * assign57960_e94690) + (locals.var_t3 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) * locals.var_invgamg2)) / (2.0 * assign57960_e94695)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign57960_e94697;
        locals.var_t5_dn3 = assign57960_e94697_d_n3;
        locals.var_t5_dn4 = assign57960_e94697_d_n4;
        locals.var_t5_dn5 = assign57960_e94697_d_n5;
        locals.var_t5_dn6 = assign57960_e94697_d_n6;
        locals.var_t5_dn7 = assign57960_e94697_d_n7;
        locals.var_t5_dn8 = assign57960_e94697_d_n8;
        locals.var_t5_dn9 = assign57960_e94697_d_n9;
        locals.var_t5_dn10 = assign57960_e94697_d_n10;
        locals.var_t5_dn11 = assign57960_e94697_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign57970_e94708, assign57970_e94708_d_n3, assign57970_e94708_d_n4, assign57970_e94708_d_n5, assign57970_e94708_d_n6, assign57970_e94708_d_n7, assign57970_e94708_d_n8, assign57970_e94708_d_n9, assign57970_e94708_d_n10, assign57970_e94708_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57970_e94705: f64 = (locals.var_t4 + locals.var_t5);
        let assign57970_e94706: f64 = (locals.var_t3 / assign57970_e94705);
        (assign57970_e94706, (((locals.var_t3_dn3 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn3 + locals.var_t5_dn3))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn4 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn4 + locals.var_t5_dn4))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn5 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn5 + locals.var_t5_dn5))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn6 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn6 + locals.var_t5_dn6))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn7 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn7 + locals.var_t5_dn7))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn8 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn8 + locals.var_t5_dn8))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn9 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn9 + locals.var_t5_dn9))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn10 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn10 + locals.var_t5_dn10))) / (assign57970_e94705 * assign57970_e94705)), (((locals.var_t3_dn11 * assign57970_e94705) - (locals.var_t3 * (locals.var_t4_dn11 + locals.var_t5_dn11))) / (assign57970_e94705 * assign57970_e94705)),)
    } else {
        (locals.var_nq, locals.var_nq_dn3, locals.var_nq_dn4, locals.var_nq_dn5, locals.var_nq_dn6, locals.var_nq_dn7, locals.var_nq_dn8, locals.var_nq_dn9, locals.var_nq_dn10, locals.var_nq_dn11,)
    }
};
        locals.var_nq = assign57970_e94708;
        locals.var_nq_dn3 = assign57970_e94708_d_n3;
        locals.var_nq_dn4 = assign57970_e94708_d_n4;
        locals.var_nq_dn5 = assign57970_e94708_d_n5;
        locals.var_nq_dn6 = assign57970_e94708_d_n6;
        locals.var_nq_dn7 = assign57970_e94708_d_n7;
        locals.var_nq_dn8 = assign57970_e94708_d_n8;
        locals.var_nq_dn9 = assign57970_e94708_d_n9;
        locals.var_nq_dn10 = assign57970_e94708_d_n10;
        locals.var_nq_dn11 = assign57970_e94708_d_n11;
        locals.var_nq_rv = 0.0;

        let (assign57980_e94715, assign57980_e94715_d_n3, assign57980_e94715_d_n4, assign57980_e94715_d_n5, assign57980_e94715_d_n6, assign57980_e94715_d_n7, assign57980_e94715_d_n8, assign57980_e94715_d_n9, assign57980_e94715_d_n10, assign57980_e94715_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mdl, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11,)
    }
};
        locals.var_mdl = assign57980_e94715;
        locals.var_mdl_dn3 = assign57980_e94715_d_n3;
        locals.var_mdl_dn4 = assign57980_e94715_d_n4;
        locals.var_mdl_dn5 = assign57980_e94715_d_n5;
        locals.var_mdl_dn6 = assign57980_e94715_d_n6;
        locals.var_mdl_dn7 = assign57980_e94715_d_n7;
        locals.var_mdl_dn8 = assign57980_e94715_d_n8;
        locals.var_mdl_dn9 = assign57980_e94715_d_n9;
        locals.var_mdl_dn10 = assign57980_e94715_d_n10;
        locals.var_mdl_dn11 = assign57980_e94715_d_n11;
        locals.var_mdl_rv = 0.0;

        let (assign57990_e94724, assign57990_e94724_d_n3, assign57990_e94724_d_n4, assign57990_e94724_d_n5, assign57990_e94724_d_n6, assign57990_e94724_d_n7, assign57990_e94724_d_n8, assign57990_e94724_d_n9, assign57990_e94724_d_n10, assign57990_e94724_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign57990_e94722: f64 = (locals.var_mdl * locals.var_mdl);
        (assign57990_e94722, ((locals.var_mdl_dn3 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn3)), ((locals.var_mdl_dn4 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn4)), ((locals.var_mdl_dn5 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn5)), ((locals.var_mdl_dn6 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn6)), ((locals.var_mdl_dn7 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn7)), ((locals.var_mdl_dn8 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn8)), ((locals.var_mdl_dn9 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn9)), ((locals.var_mdl_dn10 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn10)), ((locals.var_mdl_dn11 * locals.var_mdl) + (locals.var_mdl * locals.var_mdl_dn11)),)
    } else {
        (locals.var_mdl_2, locals.var_mdl_2_dn3, locals.var_mdl_2_dn4, locals.var_mdl_2_dn5, locals.var_mdl_2_dn6, locals.var_mdl_2_dn7, locals.var_mdl_2_dn8, locals.var_mdl_2_dn9, locals.var_mdl_2_dn10, locals.var_mdl_2_dn11,)
    }
};
        locals.var_mdl_2 = assign57990_e94724;
        locals.var_mdl_2_dn3 = assign57990_e94724_d_n3;
        locals.var_mdl_2_dn4 = assign57990_e94724_d_n4;
        locals.var_mdl_2_dn5 = assign57990_e94724_d_n5;
        locals.var_mdl_2_dn6 = assign57990_e94724_d_n6;
        locals.var_mdl_2_dn7 = assign57990_e94724_d_n7;
        locals.var_mdl_2_dn8 = assign57990_e94724_d_n8;
        locals.var_mdl_2_dn9 = assign57990_e94724_d_n9;
        locals.var_mdl_2_dn10 = assign57990_e94724_d_n10;
        locals.var_mdl_2_dn11 = assign57990_e94724_d_n11;
        locals.var_mdl_2_rv = 0.0;

        let (assign58000_e94733, assign58000_e94733_d_n3, assign58000_e94733_d_n4, assign58000_e94733_d_n5, assign58000_e94733_d_n6, assign58000_e94733_d_n7, assign58000_e94733_d_n8, assign58000_e94733_d_n9, assign58000_e94733_d_n10, assign58000_e94733_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58000_e94731: f64 = (1.0 / locals.var_mdl);
        (assign58000_e94731, (-(locals.var_mdl_dn3 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn4 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn5 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn6 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn7 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn8 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn9 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn10 / (locals.var_mdl * locals.var_mdl))), (-(locals.var_mdl_dn11 / (locals.var_mdl * locals.var_mdl))),)
    } else {
        (locals.var_inv_mdl, locals.var_inv_mdl_dn3, locals.var_inv_mdl_dn4, locals.var_inv_mdl_dn5, locals.var_inv_mdl_dn6, locals.var_inv_mdl_dn7, locals.var_inv_mdl_dn8, locals.var_inv_mdl_dn9, locals.var_inv_mdl_dn10, locals.var_inv_mdl_dn11,)
    }
};
        locals.var_inv_mdl = assign58000_e94733;
        locals.var_inv_mdl_dn3 = assign58000_e94733_d_n3;
        locals.var_inv_mdl_dn4 = assign58000_e94733_d_n4;
        locals.var_inv_mdl_dn5 = assign58000_e94733_d_n5;
        locals.var_inv_mdl_dn6 = assign58000_e94733_d_n6;
        locals.var_inv_mdl_dn7 = assign58000_e94733_d_n7;
        locals.var_inv_mdl_dn8 = assign58000_e94733_d_n8;
        locals.var_inv_mdl_dn9 = assign58000_e94733_d_n9;
        locals.var_inv_mdl_dn10 = assign58000_e94733_d_n10;
        locals.var_inv_mdl_dn11 = assign58000_e94733_d_n11;
        locals.var_inv_mdl_rv = 0.0;

        let (assign58010_e94742, assign58010_e94742_d_n3, assign58010_e94742_d_n4, assign58010_e94742_d_n5, assign58010_e94742_d_n6, assign58010_e94742_d_n7, assign58010_e94742_d_n8, assign58010_e94742_d_n9, assign58010_e94742_d_n10, assign58010_e94742_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58010_e94740: f64 = (1.0 / locals.var_mdl_2);
        (assign58010_e94740, (-(locals.var_mdl_2_dn3 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn4 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn5 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn6 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn7 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn8 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn9 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn10 / (locals.var_mdl_2 * locals.var_mdl_2))), (-(locals.var_mdl_2_dn11 / (locals.var_mdl_2 * locals.var_mdl_2))),)
    } else {
        (locals.var_inv_mdl_2, locals.var_inv_mdl_2_dn3, locals.var_inv_mdl_2_dn4, locals.var_inv_mdl_2_dn5, locals.var_inv_mdl_2_dn6, locals.var_inv_mdl_2_dn7, locals.var_inv_mdl_2_dn8, locals.var_inv_mdl_2_dn9, locals.var_inv_mdl_2_dn10, locals.var_inv_mdl_2_dn11,)
    }
};
        locals.var_inv_mdl_2 = assign58010_e94742;
        locals.var_inv_mdl_2_dn3 = assign58010_e94742_d_n3;
        locals.var_inv_mdl_2_dn4 = assign58010_e94742_d_n4;
        locals.var_inv_mdl_2_dn5 = assign58010_e94742_d_n5;
        locals.var_inv_mdl_2_dn6 = assign58010_e94742_d_n6;
        locals.var_inv_mdl_2_dn7 = assign58010_e94742_d_n7;
        locals.var_inv_mdl_2_dn8 = assign58010_e94742_d_n8;
        locals.var_inv_mdl_2_dn9 = assign58010_e94742_d_n9;
        locals.var_inv_mdl_2_dn10 = assign58010_e94742_d_n10;
        locals.var_inv_mdl_2_dn11 = assign58010_e94742_d_n11;
        locals.var_inv_mdl_2_rv = 0.0;

        let (assign58020_e94751, assign58020_e94751_d_n3, assign58020_e94751_d_n4, assign58020_e94751_d_n5, assign58020_e94751_d_n6, assign58020_e94751_d_n7, assign58020_e94751_d_n8, assign58020_e94751_d_n9, assign58020_e94751_d_n10, assign58020_e94751_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58020_e94749: f64 = (locals.var_mdl - 1.0);
        (assign58020_e94749, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11,)
    } else {
        (locals.var_mdl_less_1, locals.var_mdl_less_1_dn3, locals.var_mdl_less_1_dn4, locals.var_mdl_less_1_dn5, locals.var_mdl_less_1_dn6, locals.var_mdl_less_1_dn7, locals.var_mdl_less_1_dn8, locals.var_mdl_less_1_dn9, locals.var_mdl_less_1_dn10, locals.var_mdl_less_1_dn11,)
    }
};
        locals.var_mdl_less_1 = assign58020_e94751;
        locals.var_mdl_less_1_dn3 = assign58020_e94751_d_n3;
        locals.var_mdl_less_1_dn4 = assign58020_e94751_d_n4;
        locals.var_mdl_less_1_dn5 = assign58020_e94751_d_n5;
        locals.var_mdl_less_1_dn6 = assign58020_e94751_d_n6;
        locals.var_mdl_less_1_dn7 = assign58020_e94751_d_n7;
        locals.var_mdl_less_1_dn8 = assign58020_e94751_d_n8;
        locals.var_mdl_less_1_dn9 = assign58020_e94751_d_n9;
        locals.var_mdl_less_1_dn10 = assign58020_e94751_d_n10;
        locals.var_mdl_less_1_dn11 = assign58020_e94751_d_n11;
        locals.var_mdl_less_1_rv = 0.0;

        let (assign58030_e94760, assign58030_e94760_d_n3, assign58030_e94760_d_n4, assign58030_e94760_d_n5, assign58030_e94760_d_n6, assign58030_e94760_d_n7, assign58030_e94760_d_n8, assign58030_e94760_d_n9, assign58030_e94760_d_n10, assign58030_e94760_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58030_e94758: f64 = (locals.var_vgfbagbcp2 - locals.var_psip);
        (assign58030_e94758, (locals.var_vgfbagbcp2_dn3 - locals.var_psip_dn3), (locals.var_vgfbagbcp2_dn4 - locals.var_psip_dn4), (locals.var_vgfbagbcp2_dn5 - locals.var_psip_dn5), (locals.var_vgfbagbcp2_dn6 - locals.var_psip_dn6), (locals.var_vgfbagbcp2_dn7 - locals.var_psip_dn7), (locals.var_vgfbagbcp2_dn8 - locals.var_psip_dn8), (locals.var_vgfbagbcp2_dn9 - locals.var_psip_dn9), (locals.var_vgfbagbcp2_dn10 - locals.var_psip_dn10), (locals.var_vgfbagbcp2_dn11 - locals.var_psip_dn11),)
    } else {
        (locals.var_vgpqm, locals.var_vgpqm_dn3, locals.var_vgpqm_dn4, locals.var_vgpqm_dn5, locals.var_vgpqm_dn6, locals.var_vgpqm_dn7, locals.var_vgpqm_dn8, locals.var_vgpqm_dn9, locals.var_vgpqm_dn10, locals.var_vgpqm_dn11,)
    }
};
        locals.var_vgpqm = assign58030_e94760;
        locals.var_vgpqm_dn3 = assign58030_e94760_d_n3;
        locals.var_vgpqm_dn4 = assign58030_e94760_d_n4;
        locals.var_vgpqm_dn5 = assign58030_e94760_d_n5;
        locals.var_vgpqm_dn6 = assign58030_e94760_d_n6;
        locals.var_vgpqm_dn7 = assign58030_e94760_d_n7;
        locals.var_vgpqm_dn8 = assign58030_e94760_d_n8;
        locals.var_vgpqm_dn9 = assign58030_e94760_d_n9;
        locals.var_vgpqm_dn10 = assign58030_e94760_d_n10;
        locals.var_vgpqm_dn11 = assign58030_e94760_d_n11;
        locals.var_vgpqm_rv = 0.0;

        let (assign58040_e94769, assign58040_e94769_d_n3, assign58040_e94769_d_n4, assign58040_e94769_d_n5, assign58040_e94769_d_n6, assign58040_e94769_d_n7, assign58040_e94769_d_n8, assign58040_e94769_d_n9, assign58040_e94769_d_n10, assign58040_e94769_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58040_e94767: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign58040_e94767, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_dqsd, locals.var_dqsd_dn3, locals.var_dqsd_dn4, locals.var_dqsd_dn5, locals.var_dqsd_dn6, locals.var_dqsd_dn7, locals.var_dqsd_dn8, locals.var_dqsd_dn9, locals.var_dqsd_dn10, locals.var_dqsd_dn11,)
    }
};
        locals.var_dqsd = assign58040_e94769;
        locals.var_dqsd_dn3 = assign58040_e94769_d_n3;
        locals.var_dqsd_dn4 = assign58040_e94769_d_n4;
        locals.var_dqsd_dn5 = assign58040_e94769_d_n5;
        locals.var_dqsd_dn6 = assign58040_e94769_d_n6;
        locals.var_dqsd_dn7 = assign58040_e94769_d_n7;
        locals.var_dqsd_dn8 = assign58040_e94769_d_n8;
        locals.var_dqsd_dn9 = assign58040_e94769_d_n9;
        locals.var_dqsd_dn10 = assign58040_e94769_d_n10;
        locals.var_dqsd_dn11 = assign58040_e94769_d_n11;
        locals.var_dqsd_rv = 0.0;

        let (assign58050_e94782, assign58050_e94782_d_n3, assign58050_e94782_d_n4, assign58050_e94782_d_n5, assign58050_e94782_d_n6, assign58050_e94782_d_n7, assign58050_e94782_d_n8, assign58050_e94782_d_n9, assign58050_e94782_d_n10, assign58050_e94782_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58050_e94776: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign58050_e94779: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign58050_e94780: f64 = (assign58050_e94776 * assign58050_e94779);
        (assign58050_e94780, (((locals.var_qs_1_dn3 - locals.var_qdeff_dn3) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn3 - locals.var_qdeff_dn3))), (((locals.var_qs_1_dn4 - locals.var_qdeff_dn4) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn4 - locals.var_qdeff_dn4))), (((locals.var_qs_1_dn5 - locals.var_qdeff_dn5) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn5 - locals.var_qdeff_dn5))), (((locals.var_qs_1_dn6 - locals.var_qdeff_dn6) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn6 - locals.var_qdeff_dn6))), (((locals.var_qs_1_dn7 - locals.var_qdeff_dn7) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn7 - locals.var_qdeff_dn7))), (((locals.var_qs_1_dn8 - locals.var_qdeff_dn8) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn8 - locals.var_qdeff_dn8))), (((locals.var_qs_1_dn9 - locals.var_qdeff_dn9) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn9 - locals.var_qdeff_dn9))), (((locals.var_qs_1_dn10 - locals.var_qdeff_dn10) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn10 - locals.var_qdeff_dn10))), (((locals.var_qs_1_dn11 - locals.var_qdeff_dn11) * assign58050_e94779) + (assign58050_e94776 * (locals.var_qs_1_dn11 - locals.var_qdeff_dn11))),)
    } else {
        (locals.var_dqsd2, locals.var_dqsd2_dn3, locals.var_dqsd2_dn4, locals.var_dqsd2_dn5, locals.var_dqsd2_dn6, locals.var_dqsd2_dn7, locals.var_dqsd2_dn8, locals.var_dqsd2_dn9, locals.var_dqsd2_dn10, locals.var_dqsd2_dn11,)
    }
};
        locals.var_dqsd2 = assign58050_e94782;
        locals.var_dqsd2_dn3 = assign58050_e94782_d_n3;
        locals.var_dqsd2_dn4 = assign58050_e94782_d_n4;
        locals.var_dqsd2_dn5 = assign58050_e94782_d_n5;
        locals.var_dqsd2_dn6 = assign58050_e94782_d_n6;
        locals.var_dqsd2_dn7 = assign58050_e94782_d_n7;
        locals.var_dqsd2_dn8 = assign58050_e94782_d_n8;
        locals.var_dqsd2_dn9 = assign58050_e94782_d_n9;
        locals.var_dqsd2_dn10 = assign58050_e94782_d_n10;
        locals.var_dqsd2_dn11 = assign58050_e94782_d_n11;
        locals.var_dqsd2_rv = 0.0;

        let (assign58060_e94793, assign58060_e94793_d_n3, assign58060_e94793_d_n4, assign58060_e94793_d_n5, assign58060_e94793_d_n6, assign58060_e94793_d_n7, assign58060_e94793_d_n8, assign58060_e94793_d_n9, assign58060_e94793_d_n10, assign58060_e94793_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58060_e94790: f64 = (2.0 * locals.var_qs_1);
        let assign58060_e94791: f64 = (locals.var_vgpqm + assign58060_e94790);
        (assign58060_e94791, (locals.var_vgpqm_dn3 + (2.0 * locals.var_qs_1_dn3)), (locals.var_vgpqm_dn4 + (2.0 * locals.var_qs_1_dn4)), (locals.var_vgpqm_dn5 + (2.0 * locals.var_qs_1_dn5)), (locals.var_vgpqm_dn6 + (2.0 * locals.var_qs_1_dn6)), (locals.var_vgpqm_dn7 + (2.0 * locals.var_qs_1_dn7)), (locals.var_vgpqm_dn8 + (2.0 * locals.var_qs_1_dn8)), (locals.var_vgpqm_dn9 + (2.0 * locals.var_qs_1_dn9)), (locals.var_vgpqm_dn10 + (2.0 * locals.var_qs_1_dn10)), (locals.var_vgpqm_dn11 + (2.0 * locals.var_qs_1_dn11)),)
    } else {
        (locals.var_sis, locals.var_sis_dn3, locals.var_sis_dn4, locals.var_sis_dn5, locals.var_sis_dn6, locals.var_sis_dn7, locals.var_sis_dn8, locals.var_sis_dn9, locals.var_sis_dn10, locals.var_sis_dn11,)
    }
};
        locals.var_sis = assign58060_e94793;
        locals.var_sis_dn3 = assign58060_e94793_d_n3;
        locals.var_sis_dn4 = assign58060_e94793_d_n4;
        locals.var_sis_dn5 = assign58060_e94793_d_n5;
        locals.var_sis_dn6 = assign58060_e94793_d_n6;
        locals.var_sis_dn7 = assign58060_e94793_d_n7;
        locals.var_sis_dn8 = assign58060_e94793_d_n8;
        locals.var_sis_dn9 = assign58060_e94793_d_n9;
        locals.var_sis_dn10 = assign58060_e94793_d_n10;
        locals.var_sis_dn11 = assign58060_e94793_d_n11;
        locals.var_sis_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_202(
        locals: &mut StampLocals,
    ) {
        let (assign58070_e94804, assign58070_e94804_d_n3, assign58070_e94804_d_n4, assign58070_e94804_d_n5, assign58070_e94804_d_n6, assign58070_e94804_d_n7, assign58070_e94804_d_n8, assign58070_e94804_d_n9, assign58070_e94804_d_n10, assign58070_e94804_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58070_e94801: f64 = (2.0 * locals.var_qdeff);
        let assign58070_e94802: f64 = (locals.var_vgpqm + assign58070_e94801);
        (assign58070_e94802, (locals.var_vgpqm_dn3 + (2.0 * locals.var_qdeff_dn3)), (locals.var_vgpqm_dn4 + (2.0 * locals.var_qdeff_dn4)), (locals.var_vgpqm_dn5 + (2.0 * locals.var_qdeff_dn5)), (locals.var_vgpqm_dn6 + (2.0 * locals.var_qdeff_dn6)), (locals.var_vgpqm_dn7 + (2.0 * locals.var_qdeff_dn7)), (locals.var_vgpqm_dn8 + (2.0 * locals.var_qdeff_dn8)), (locals.var_vgpqm_dn9 + (2.0 * locals.var_qdeff_dn9)), (locals.var_vgpqm_dn10 + (2.0 * locals.var_qdeff_dn10)), (locals.var_vgpqm_dn11 + (2.0 * locals.var_qdeff_dn11)),)
    } else {
        (locals.var_sid, locals.var_sid_dn3, locals.var_sid_dn4, locals.var_sid_dn5, locals.var_sid_dn6, locals.var_sid_dn7, locals.var_sid_dn8, locals.var_sid_dn9, locals.var_sid_dn10, locals.var_sid_dn11,)
    }
};
        locals.var_sid = assign58070_e94804;
        locals.var_sid_dn3 = assign58070_e94804_d_n3;
        locals.var_sid_dn4 = assign58070_e94804_d_n4;
        locals.var_sid_dn5 = assign58070_e94804_d_n5;
        locals.var_sid_dn6 = assign58070_e94804_d_n6;
        locals.var_sid_dn7 = assign58070_e94804_d_n7;
        locals.var_sid_dn8 = assign58070_e94804_d_n8;
        locals.var_sid_dn9 = assign58070_e94804_d_n9;
        locals.var_sid_dn10 = assign58070_e94804_d_n10;
        locals.var_sid_dn11 = assign58070_e94804_d_n11;
        locals.var_sid_rv = 0.0;

        let (assign58080_e94830, assign58080_e94830_d_n3, assign58080_e94830_d_n4, assign58080_e94830_d_n5, assign58080_e94830_d_n6, assign58080_e94830_d_n7, assign58080_e94830_d_n8, assign58080_e94830_d_n9, assign58080_e94830_d_n10, assign58080_e94830_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58080_e94812: f64 = locals.var_sis;
        let assign58080_e94815: f64 = locals.var_sis;
        let assign58080_e94818: f64 = locals.var_sis;
        let assign58080_e94819: f64 = (assign58080_e94815 * assign58080_e94818);
        let assign58080_e94822: f64 = (0.25 * 0.5);
        let assign58080_e94824: f64 = (assign58080_e94822 * 0.5);
        let assign58080_e94825: f64 = (assign58080_e94819 + assign58080_e94824);
        let assign58080_e94826: f64 = (assign58080_e94825).sqrt();
        let assign58080_e94827: f64 = (assign58080_e94812 + assign58080_e94826);
        let assign58080_e94828: f64 = (0.5 * assign58080_e94827);
        (assign58080_e94828, (0.5 * (locals.var_sis_dn3 + (((locals.var_sis_dn3 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn3)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn4 + (((locals.var_sis_dn4 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn4)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn5 + (((locals.var_sis_dn5 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn5)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn6 + (((locals.var_sis_dn6 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn6)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn7 + (((locals.var_sis_dn7 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn7)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn8 + (((locals.var_sis_dn8 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn8)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn9 + (((locals.var_sis_dn9 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn9)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn10 + (((locals.var_sis_dn10 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn10)) / (2.0 * assign58080_e94826)))), (0.5 * (locals.var_sis_dn11 + (((locals.var_sis_dn11 * assign58080_e94818) + (assign58080_e94815 * locals.var_sis_dn11)) / (2.0 * assign58080_e94826)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58080_e94830;
        locals.var_t1_dn3 = assign58080_e94830_d_n3;
        locals.var_t1_dn4 = assign58080_e94830_d_n4;
        locals.var_t1_dn5 = assign58080_e94830_d_n5;
        locals.var_t1_dn6 = assign58080_e94830_d_n6;
        locals.var_t1_dn7 = assign58080_e94830_d_n7;
        locals.var_t1_dn8 = assign58080_e94830_d_n8;
        locals.var_t1_dn9 = assign58080_e94830_d_n9;
        locals.var_t1_dn10 = assign58080_e94830_d_n10;
        locals.var_t1_dn11 = assign58080_e94830_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign58090_e94856, assign58090_e94856_d_n3, assign58090_e94856_d_n4, assign58090_e94856_d_n5, assign58090_e94856_d_n6, assign58090_e94856_d_n7, assign58090_e94856_d_n8, assign58090_e94856_d_n9, assign58090_e94856_d_n10, assign58090_e94856_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58090_e94838: f64 = locals.var_sid;
        let assign58090_e94841: f64 = locals.var_sid;
        let assign58090_e94844: f64 = locals.var_sid;
        let assign58090_e94845: f64 = (assign58090_e94841 * assign58090_e94844);
        let assign58090_e94848: f64 = (0.25 * 0.5);
        let assign58090_e94850: f64 = (assign58090_e94848 * 0.5);
        let assign58090_e94851: f64 = (assign58090_e94845 + assign58090_e94850);
        let assign58090_e94852: f64 = (assign58090_e94851).sqrt();
        let assign58090_e94853: f64 = (assign58090_e94838 + assign58090_e94852);
        let assign58090_e94854: f64 = (0.5 * assign58090_e94853);
        (assign58090_e94854, (0.5 * (locals.var_sid_dn3 + (((locals.var_sid_dn3 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn3)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn4 + (((locals.var_sid_dn4 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn4)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn5 + (((locals.var_sid_dn5 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn5)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn6 + (((locals.var_sid_dn6 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn6)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn7 + (((locals.var_sid_dn7 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn7)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn8 + (((locals.var_sid_dn8 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn8)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn9 + (((locals.var_sid_dn9 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn9)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn10 + (((locals.var_sid_dn10 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn10)) / (2.0 * assign58090_e94852)))), (0.5 * (locals.var_sid_dn11 + (((locals.var_sid_dn11 * assign58090_e94844) + (assign58090_e94841 * locals.var_sid_dn11)) / (2.0 * assign58090_e94852)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign58090_e94856;
        locals.var_t2_dn3 = assign58090_e94856_d_n3;
        locals.var_t2_dn4 = assign58090_e94856_d_n4;
        locals.var_t2_dn5 = assign58090_e94856_d_n5;
        locals.var_t2_dn6 = assign58090_e94856_d_n6;
        locals.var_t2_dn7 = assign58090_e94856_d_n7;
        locals.var_t2_dn8 = assign58090_e94856_d_n8;
        locals.var_t2_dn9 = assign58090_e94856_d_n9;
        locals.var_t2_dn10 = assign58090_e94856_d_n10;
        locals.var_t2_dn11 = assign58090_e94856_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign58100_e94868, assign58100_e94868_d_n3, assign58100_e94868_d_n4, assign58100_e94868_d_n5, assign58100_e94868_d_n6, assign58100_e94868_d_n7, assign58100_e94868_d_n8, assign58100_e94868_d_n9, assign58100_e94868_d_n10, assign58100_e94868_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58100_e94864: f64 = (locals.var_t1 * locals.var_invgamg2);
        let assign58100_e94865: f64 = (0.25 + assign58100_e94864);
        let assign58100_e94866: f64 = (assign58100_e94865).sqrt();
        (assign58100_e94866, ((locals.var_t1_dn3 * locals.var_invgamg2) / (2.0 * assign58100_e94866)), (((locals.var_t1_dn4 * locals.var_invgamg2) + (locals.var_t1 * locals.var_invgamg2_dn4)) / (2.0 * assign58100_e94866)), (((locals.var_t1_dn5 * locals.var_invgamg2) + (locals.var_t1 * locals.var_invgamg2_dn5)) / (2.0 * assign58100_e94866)), ((locals.var_t1_dn6 * locals.var_invgamg2) / (2.0 * assign58100_e94866)), ((locals.var_t1_dn7 * locals.var_invgamg2) / (2.0 * assign58100_e94866)), ((locals.var_t1_dn8 * locals.var_invgamg2) / (2.0 * assign58100_e94866)), ((locals.var_t1_dn9 * locals.var_invgamg2) / (2.0 * assign58100_e94866)), ((locals.var_t1_dn10 * locals.var_invgamg2) / (2.0 * assign58100_e94866)), ((locals.var_t1_dn11 * locals.var_invgamg2) / (2.0 * assign58100_e94866)),)
    } else {
        (locals.var_temps, locals.var_temps_dn3, locals.var_temps_dn4, locals.var_temps_dn5, locals.var_temps_dn6, locals.var_temps_dn7, locals.var_temps_dn8, locals.var_temps_dn9, locals.var_temps_dn10, locals.var_temps_dn11,)
    }
};
        locals.var_temps = assign58100_e94868;
        locals.var_temps_dn3 = assign58100_e94868_d_n3;
        locals.var_temps_dn4 = assign58100_e94868_d_n4;
        locals.var_temps_dn5 = assign58100_e94868_d_n5;
        locals.var_temps_dn6 = assign58100_e94868_d_n6;
        locals.var_temps_dn7 = assign58100_e94868_d_n7;
        locals.var_temps_dn8 = assign58100_e94868_d_n8;
        locals.var_temps_dn9 = assign58100_e94868_d_n9;
        locals.var_temps_dn10 = assign58100_e94868_d_n10;
        locals.var_temps_dn11 = assign58100_e94868_d_n11;
        locals.var_temps_rv = 0.0;

        let (assign58110_e94880, assign58110_e94880_d_n3, assign58110_e94880_d_n4, assign58110_e94880_d_n5, assign58110_e94880_d_n6, assign58110_e94880_d_n7, assign58110_e94880_d_n8, assign58110_e94880_d_n9, assign58110_e94880_d_n10, assign58110_e94880_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58110_e94876: f64 = (locals.var_t2 * locals.var_invgamg2);
        let assign58110_e94877: f64 = (0.25 + assign58110_e94876);
        let assign58110_e94878: f64 = (assign58110_e94877).sqrt();
        (assign58110_e94878, ((locals.var_t2_dn3 * locals.var_invgamg2) / (2.0 * assign58110_e94878)), (((locals.var_t2_dn4 * locals.var_invgamg2) + (locals.var_t2 * locals.var_invgamg2_dn4)) / (2.0 * assign58110_e94878)), (((locals.var_t2_dn5 * locals.var_invgamg2) + (locals.var_t2 * locals.var_invgamg2_dn5)) / (2.0 * assign58110_e94878)), ((locals.var_t2_dn6 * locals.var_invgamg2) / (2.0 * assign58110_e94878)), ((locals.var_t2_dn7 * locals.var_invgamg2) / (2.0 * assign58110_e94878)), ((locals.var_t2_dn8 * locals.var_invgamg2) / (2.0 * assign58110_e94878)), ((locals.var_t2_dn9 * locals.var_invgamg2) / (2.0 * assign58110_e94878)), ((locals.var_t2_dn10 * locals.var_invgamg2) / (2.0 * assign58110_e94878)), ((locals.var_t2_dn11 * locals.var_invgamg2) / (2.0 * assign58110_e94878)),)
    } else {
        (locals.var_tempd, locals.var_tempd_dn3, locals.var_tempd_dn4, locals.var_tempd_dn5, locals.var_tempd_dn6, locals.var_tempd_dn7, locals.var_tempd_dn8, locals.var_tempd_dn9, locals.var_tempd_dn10, locals.var_tempd_dn11,)
    }
};
        locals.var_tempd = assign58110_e94880;
        locals.var_tempd_dn3 = assign58110_e94880_d_n3;
        locals.var_tempd_dn4 = assign58110_e94880_d_n4;
        locals.var_tempd_dn5 = assign58110_e94880_d_n5;
        locals.var_tempd_dn6 = assign58110_e94880_d_n6;
        locals.var_tempd_dn7 = assign58110_e94880_d_n7;
        locals.var_tempd_dn8 = assign58110_e94880_d_n8;
        locals.var_tempd_dn9 = assign58110_e94880_d_n9;
        locals.var_tempd_dn10 = assign58110_e94880_d_n10;
        locals.var_tempd_dn11 = assign58110_e94880_d_n11;
        locals.var_tempd_rv = 0.0;

        let (assign58120_e94893, assign58120_e94893_d_n3, assign58120_e94893_d_n4, assign58120_e94893_d_n5, assign58120_e94893_d_n6, assign58120_e94893_d_n7, assign58120_e94893_d_n8, assign58120_e94893_d_n9, assign58120_e94893_d_n10, assign58120_e94893_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58120_e94889: f64 = (2.0 * locals.var_temps);
        let assign58120_e94890: f64 = (1.0 + assign58120_e94889);
        let assign58120_e94891: f64 = (locals.var_sis / assign58120_e94890);
        (assign58120_e94891, (((locals.var_sis_dn3 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn3))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn4 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn4))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn5 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn5))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn6 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn6))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn7 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn7))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn8 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn8))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn9 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn9))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn10 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn10))) / (assign58120_e94890 * assign58120_e94890)), (((locals.var_sis_dn11 * assign58120_e94890) - (locals.var_sis * (2.0 * locals.var_temps_dn11))) / (assign58120_e94890 * assign58120_e94890)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58120_e94893;
        locals.var_t1_dn3 = assign58120_e94893_d_n3;
        locals.var_t1_dn4 = assign58120_e94893_d_n4;
        locals.var_t1_dn5 = assign58120_e94893_d_n5;
        locals.var_t1_dn6 = assign58120_e94893_d_n6;
        locals.var_t1_dn7 = assign58120_e94893_d_n7;
        locals.var_t1_dn8 = assign58120_e94893_d_n8;
        locals.var_t1_dn9 = assign58120_e94893_d_n9;
        locals.var_t1_dn10 = assign58120_e94893_d_n10;
        locals.var_t1_dn11 = assign58120_e94893_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign58130_e94906, assign58130_e94906_d_n3, assign58130_e94906_d_n4, assign58130_e94906_d_n5, assign58130_e94906_d_n6, assign58130_e94906_d_n7, assign58130_e94906_d_n8, assign58130_e94906_d_n9, assign58130_e94906_d_n10, assign58130_e94906_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58130_e94902: f64 = (2.0 * locals.var_tempd);
        let assign58130_e94903: f64 = (1.0 + assign58130_e94902);
        let assign58130_e94904: f64 = (locals.var_sid / assign58130_e94903);
        (assign58130_e94904, (((locals.var_sid_dn3 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn3))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn4 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn4))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn5 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn5))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn6 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn6))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn7 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn7))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn8 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn8))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn9 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn9))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn10 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn10))) / (assign58130_e94903 * assign58130_e94903)), (((locals.var_sid_dn11 * assign58130_e94903) - (locals.var_sid * (2.0 * locals.var_tempd_dn11))) / (assign58130_e94903 * assign58130_e94903)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign58130_e94906;
        locals.var_t2_dn3 = assign58130_e94906_d_n3;
        locals.var_t2_dn4 = assign58130_e94906_d_n4;
        locals.var_t2_dn5 = assign58130_e94906_d_n5;
        locals.var_t2_dn6 = assign58130_e94906_d_n6;
        locals.var_t2_dn7 = assign58130_e94906_d_n7;
        locals.var_t2_dn8 = assign58130_e94906_d_n8;
        locals.var_t2_dn9 = assign58130_e94906_d_n9;
        locals.var_t2_dn10 = assign58130_e94906_d_n10;
        locals.var_t2_dn11 = assign58130_e94906_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign58140_e94915, assign58140_e94915_d_n3, assign58140_e94915_d_n4, assign58140_e94915_d_n5, assign58140_e94915_d_n6, assign58140_e94915_d_n7, assign58140_e94915_d_n8, assign58140_e94915_d_n9, assign58140_e94915_d_n10, assign58140_e94915_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58140_e94913: f64 = (locals.var_temps + locals.var_tempd);
        (assign58140_e94913, (locals.var_temps_dn3 + locals.var_tempd_dn3), (locals.var_temps_dn4 + locals.var_tempd_dn4), (locals.var_temps_dn5 + locals.var_tempd_dn5), (locals.var_temps_dn6 + locals.var_tempd_dn6), (locals.var_temps_dn7 + locals.var_tempd_dn7), (locals.var_temps_dn8 + locals.var_tempd_dn8), (locals.var_temps_dn9 + locals.var_tempd_dn9), (locals.var_temps_dn10 + locals.var_tempd_dn10), (locals.var_temps_dn11 + locals.var_tempd_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign58140_e94915;
        locals.var_t3_dn3 = assign58140_e94915_d_n3;
        locals.var_t3_dn4 = assign58140_e94915_d_n4;
        locals.var_t3_dn5 = assign58140_e94915_d_n5;
        locals.var_t3_dn6 = assign58140_e94915_d_n6;
        locals.var_t3_dn7 = assign58140_e94915_d_n7;
        locals.var_t3_dn8 = assign58140_e94915_d_n8;
        locals.var_t3_dn9 = assign58140_e94915_d_n9;
        locals.var_t3_dn10 = assign58140_e94915_d_n10;
        locals.var_t3_dn11 = assign58140_e94915_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign58150_e94930, assign58150_e94930_d_n3, assign58150_e94930_d_n4, assign58150_e94930_d_n5, assign58150_e94930_d_n6, assign58150_e94930_d_n7, assign58150_e94930_d_n8, assign58150_e94930_d_n9, assign58150_e94930_d_n10, assign58150_e94930_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58150_e94924: f64 = (locals.var_t3 * locals.var_t3);
        let assign58150_e94926: f64 = (assign58150_e94924 * locals.var_t3);
        let assign58150_e94927: f64 = (locals.var_dqsd2 / assign58150_e94926);
        let assign58150_e94928: f64 = (0.3333333333333333 * assign58150_e94927);
        (assign58150_e94928, (0.3333333333333333 * (((locals.var_dqsd2_dn3 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn3)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn4 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn4)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn5 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn5)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn6 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn6)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn7 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn7)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn8 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn8)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn9 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn9)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn10 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn10)))) / (assign58150_e94926 * assign58150_e94926))), (0.3333333333333333 * (((locals.var_dqsd2_dn11 * assign58150_e94926) - (locals.var_dqsd2 * ((((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) * locals.var_t3) + (assign58150_e94924 * locals.var_t3_dn11)))) / (assign58150_e94926 * assign58150_e94926))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign58150_e94930;
        locals.var_t4_dn3 = assign58150_e94930_d_n3;
        locals.var_t4_dn4 = assign58150_e94930_d_n4;
        locals.var_t4_dn5 = assign58150_e94930_d_n5;
        locals.var_t4_dn6 = assign58150_e94930_d_n6;
        locals.var_t4_dn7 = assign58150_e94930_d_n7;
        locals.var_t4_dn8 = assign58150_e94930_d_n8;
        locals.var_t4_dn9 = assign58150_e94930_d_n9;
        locals.var_t4_dn10 = assign58150_e94930_d_n10;
        locals.var_t4_dn11 = assign58150_e94930_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign58160_e94937, assign58160_e94937_d_n3, assign58160_e94937_d_n4, assign58160_e94937_d_n5, assign58160_e94937_d_n6, assign58160_e94937_d_n7, assign58160_e94937_d_n8, assign58160_e94937_d_n9, assign58160_e94937_d_n10, assign58160_e94937_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign58160_e94937;
        locals.var_dvsat_dn3 = assign58160_e94937_d_n3;
        locals.var_dvsat_dn4 = assign58160_e94937_d_n4;
        locals.var_dvsat_dn5 = assign58160_e94937_d_n5;
        locals.var_dvsat_dn6 = assign58160_e94937_d_n6;
        locals.var_dvsat_dn7 = assign58160_e94937_d_n7;
        locals.var_dvsat_dn8 = assign58160_e94937_d_n8;
        locals.var_dvsat_dn9 = assign58160_e94937_d_n9;
        locals.var_dvsat_dn10 = assign58160_e94937_d_n10;
        locals.var_dvsat_dn11 = assign58160_e94937_d_n11;
        locals.var_dvsat_rv = 0.0;

        let (assign58170_e94952, assign58170_e94952_d_n3, assign58170_e94952_d_n4, assign58170_e94952_d_n5, assign58170_e94952_d_n6, assign58170_e94952_d_n7, assign58170_e94952_d_n8, assign58170_e94952_d_n9, assign58170_e94952_d_n10, assign58170_e94952_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58170_e94944: f64 = (locals.var_dvsat * locals.var_inv_mdl);
        let assign58170_e94947: f64 = (1.0 + locals.var_qs_1);
        let assign58170_e94949: f64 = (assign58170_e94947 + locals.var_qdeff);
        let assign58170_e94950: f64 = (assign58170_e94944 / assign58170_e94949);
        (assign58170_e94950, (((((locals.var_dvsat_dn3 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn3)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn4 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn4)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn5 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn5)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn6 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn6)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn7 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn7)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn8 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn8)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn9 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn9)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn10 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn10)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) / (assign58170_e94949 * assign58170_e94949)), (((((locals.var_dvsat_dn11 * locals.var_inv_mdl) + (locals.var_dvsat * locals.var_inv_mdl_dn11)) * assign58170_e94949) - (assign58170_e94944 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) / (assign58170_e94949 * assign58170_e94949)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign58170_e94952;
        locals.var_t5_dn3 = assign58170_e94952_d_n3;
        locals.var_t5_dn4 = assign58170_e94952_d_n4;
        locals.var_t5_dn5 = assign58170_e94952_d_n5;
        locals.var_t5_dn6 = assign58170_e94952_d_n6;
        locals.var_t5_dn7 = assign58170_e94952_d_n7;
        locals.var_t5_dn8 = assign58170_e94952_d_n8;
        locals.var_t5_dn9 = assign58170_e94952_d_n9;
        locals.var_t5_dn10 = assign58170_e94952_d_n10;
        locals.var_t5_dn11 = assign58170_e94952_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign58180_e94969, assign58180_e94969_d_n3, assign58180_e94969_d_n4, assign58180_e94969_d_n5, assign58180_e94969_d_n6, assign58180_e94969_d_n7, assign58180_e94969_d_n8, assign58180_e94969_d_n9, assign58180_e94969_d_n10, assign58180_e94969_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58180_e94960: f64 = (locals.var_t3 * locals.var_t3);
        let assign58180_e94963: f64 = (locals.var_temps * locals.var_tempd);
        let assign58180_e94964: f64 = (assign58180_e94960 + assign58180_e94963);
        let assign58180_e94965: f64 = (0.8 * assign58180_e94964);
        let assign58180_e94967: f64 = (assign58180_e94965 * locals.var_t5);
        (assign58180_e94967, (((0.8 * (((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3)) + ((locals.var_temps_dn3 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn3)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn3)), (((0.8 * (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) + ((locals.var_temps_dn4 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn4)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn4)), (((0.8 * (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) + ((locals.var_temps_dn5 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn5)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn5)), (((0.8 * (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) + ((locals.var_temps_dn6 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn6)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn6)), (((0.8 * (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) + ((locals.var_temps_dn7 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn7)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn7)), (((0.8 * (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) + ((locals.var_temps_dn8 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn8)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn8)), (((0.8 * (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) + ((locals.var_temps_dn9 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn9)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn9)), (((0.8 * (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) + ((locals.var_temps_dn10 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn10)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn10)), (((0.8 * (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) + ((locals.var_temps_dn11 * locals.var_tempd) + (locals.var_temps * locals.var_tempd_dn11)))) * locals.var_t5) + (assign58180_e94965 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign58180_e94969;
        locals.var_t6_dn3 = assign58180_e94969_d_n3;
        locals.var_t6_dn4 = assign58180_e94969_d_n4;
        locals.var_t6_dn5 = assign58180_e94969_d_n5;
        locals.var_t6_dn6 = assign58180_e94969_d_n6;
        locals.var_t6_dn7 = assign58180_e94969_d_n7;
        locals.var_t6_dn8 = assign58180_e94969_d_n8;
        locals.var_t6_dn9 = assign58180_e94969_d_n9;
        locals.var_t6_dn10 = assign58180_e94969_d_n10;
        locals.var_t6_dn11 = assign58180_e94969_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign58190_e94980, assign58190_e94980_d_n3, assign58190_e94980_d_n4, assign58190_e94980_d_n5, assign58190_e94980_d_n6, assign58190_e94980_d_n7, assign58190_e94980_d_n8, assign58190_e94980_d_n9, assign58190_e94980_d_n10, assign58190_e94980_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58190_e94977: f64 = (2.0 * locals.var_invgamg2);
        let assign58190_e94978: f64 = (locals.var_t6 + assign58190_e94977);
        (assign58190_e94978, locals.var_t6_dn3, (locals.var_t6_dn4 + (2.0 * locals.var_invgamg2_dn4)), (locals.var_t6_dn5 + (2.0 * locals.var_invgamg2_dn5)), locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign58190_e94980;
        locals.var_t7_dn3 = assign58190_e94980_d_n3;
        locals.var_t7_dn4 = assign58190_e94980_d_n4;
        locals.var_t7_dn5 = assign58190_e94980_d_n5;
        locals.var_t7_dn6 = assign58190_e94980_d_n6;
        locals.var_t7_dn7 = assign58190_e94980_d_n7;
        locals.var_t7_dn8 = assign58190_e94980_d_n8;
        locals.var_t7_dn9 = assign58190_e94980_d_n9;
        locals.var_t7_dn10 = assign58190_e94980_d_n10;
        locals.var_t7_dn11 = assign58190_e94980_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign58200_e94991, assign58200_e94991_d_n3, assign58200_e94991_d_n4, assign58200_e94991_d_n5, assign58200_e94991_d_n6, assign58200_e94991_d_n7, assign58200_e94991_d_n8, assign58200_e94991_d_n9, assign58200_e94991_d_n10, assign58200_e94991_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58200_e94987: f64 = (0.3333333333333333 * locals.var_dqsd2);
        let assign58200_e94989: f64 = (assign58200_e94987 * locals.var_t5);
        (assign58200_e94989, (((0.3333333333333333 * locals.var_dqsd2_dn3) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn3)), (((0.3333333333333333 * locals.var_dqsd2_dn4) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn4)), (((0.3333333333333333 * locals.var_dqsd2_dn5) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn5)), (((0.3333333333333333 * locals.var_dqsd2_dn6) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn6)), (((0.3333333333333333 * locals.var_dqsd2_dn7) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn7)), (((0.3333333333333333 * locals.var_dqsd2_dn8) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn8)), (((0.3333333333333333 * locals.var_dqsd2_dn9) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn9)), (((0.3333333333333333 * locals.var_dqsd2_dn10) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn10)), (((0.3333333333333333 * locals.var_dqsd2_dn11) * locals.var_t5) + (assign58200_e94987 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign58200_e94991;
        locals.var_t8_dn3 = assign58200_e94991_d_n3;
        locals.var_t8_dn4 = assign58200_e94991_d_n4;
        locals.var_t8_dn5 = assign58200_e94991_d_n5;
        locals.var_t8_dn6 = assign58200_e94991_d_n6;
        locals.var_t8_dn7 = assign58200_e94991_d_n7;
        locals.var_t8_dn8 = assign58200_e94991_d_n8;
        locals.var_t8_dn9 = assign58200_e94991_d_n9;
        locals.var_t8_dn10 = assign58200_e94991_d_n10;
        locals.var_t8_dn11 = assign58200_e94991_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign58210_e95010, assign58210_e95010_d_n3, assign58210_e95010_d_n4, assign58210_e95010_d_n5, assign58210_e95010_d_n6, assign58210_e95010_d_n7, assign58210_e95010_d_n8, assign58210_e95010_d_n9, assign58210_e95010_d_n10, assign58210_e95010_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58210_e94999: f64 = (2.0 * locals.var_tempd);
        let assign58210_e95001: f64 = (assign58210_e94999 - 1.0);
        let assign58210_e95002: f64 = (locals.var_sid * assign58210_e95001);
        let assign58210_e95005: f64 = (2.0 * locals.var_tempd);
        let assign58210_e95007: f64 = (assign58210_e95005 + 1.0);
        let assign58210_e95008: f64 = (assign58210_e95002 / assign58210_e95007);
        (assign58210_e95008, (((((locals.var_sid_dn3 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn3))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn3))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn4 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn4))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn4))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn5 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn5))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn5))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn6 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn6))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn6))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn7 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn7))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn7))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn8 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn8))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn8))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn9 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn9))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn9))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn10 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn10))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn10))) / (assign58210_e95007 * assign58210_e95007)), (((((locals.var_sid_dn11 * assign58210_e95001) + (locals.var_sid * (2.0 * locals.var_tempd_dn11))) * assign58210_e95007) - (assign58210_e95002 * (2.0 * locals.var_tempd_dn11))) / (assign58210_e95007 * assign58210_e95007)),)
    } else {
        (locals.var_dqgeff, locals.var_dqgeff_dn3, locals.var_dqgeff_dn4, locals.var_dqgeff_dn5, locals.var_dqgeff_dn6, locals.var_dqgeff_dn7, locals.var_dqgeff_dn8, locals.var_dqgeff_dn9, locals.var_dqgeff_dn10, locals.var_dqgeff_dn11,)
    }
};
        locals.var_dqgeff = assign58210_e95010;
        locals.var_dqgeff_dn3 = assign58210_e95010_d_n3;
        locals.var_dqgeff_dn4 = assign58210_e95010_d_n4;
        locals.var_dqgeff_dn5 = assign58210_e95010_d_n5;
        locals.var_dqgeff_dn6 = assign58210_e95010_d_n6;
        locals.var_dqgeff_dn7 = assign58210_e95010_d_n7;
        locals.var_dqgeff_dn8 = assign58210_e95010_d_n8;
        locals.var_dqgeff_dn9 = assign58210_e95010_d_n9;
        locals.var_dqgeff_dn10 = assign58210_e95010_d_n10;
        locals.var_dqgeff_dn11 = assign58210_e95010_d_n11;
        locals.var_dqgeff_rv = 0.0;

        let (assign58220_e95027, assign58220_e95027_d_n3, assign58220_e95027_d_n4, assign58220_e95027_d_n5, assign58220_e95027_d_n6, assign58220_e95027_d_n7, assign58220_e95027_d_n8, assign58220_e95027_d_n9, assign58220_e95027_d_n10, assign58220_e95027_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58220_e95019: f64 = (locals.var_nq - 1.0);
        let assign58220_e95020: f64 = (2.0 * assign58220_e95019);
        let assign58220_e95022: f64 = (assign58220_e95020 * locals.var_qdeff);
        let assign58220_e95023: f64 = (locals.var_vgpqm - assign58220_e95022);
        let assign58220_e95025: f64 = (assign58220_e95023 + locals.var_dqgeff);
        (assign58220_e95025, ((locals.var_vgpqm_dn3 - (((2.0 * locals.var_nq_dn3) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn3))) + locals.var_dqgeff_dn3), ((locals.var_vgpqm_dn4 - (((2.0 * locals.var_nq_dn4) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn4))) + locals.var_dqgeff_dn4), ((locals.var_vgpqm_dn5 - (((2.0 * locals.var_nq_dn5) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn5))) + locals.var_dqgeff_dn5), ((locals.var_vgpqm_dn6 - (((2.0 * locals.var_nq_dn6) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn6))) + locals.var_dqgeff_dn6), ((locals.var_vgpqm_dn7 - (((2.0 * locals.var_nq_dn7) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn7))) + locals.var_dqgeff_dn7), ((locals.var_vgpqm_dn8 - (((2.0 * locals.var_nq_dn8) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn8))) + locals.var_dqgeff_dn8), ((locals.var_vgpqm_dn9 - (((2.0 * locals.var_nq_dn9) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn9))) + locals.var_dqgeff_dn9), ((locals.var_vgpqm_dn10 - (((2.0 * locals.var_nq_dn10) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn10))) + locals.var_dqgeff_dn10), ((locals.var_vgpqm_dn11 - (((2.0 * locals.var_nq_dn11) * locals.var_qdeff) + (assign58220_e95020 * locals.var_qdeff_dn11))) + locals.var_dqgeff_dn11),)
    } else {
        (locals.var_qbeff, locals.var_qbeff_dn3, locals.var_qbeff_dn4, locals.var_qbeff_dn5, locals.var_qbeff_dn6, locals.var_qbeff_dn7, locals.var_qbeff_dn8, locals.var_qbeff_dn9, locals.var_qbeff_dn10, locals.var_qbeff_dn11,)
    }
};
        locals.var_qbeff = assign58220_e95027;
        locals.var_qbeff_dn3 = assign58220_e95027_d_n3;
        locals.var_qbeff_dn4 = assign58220_e95027_d_n4;
        locals.var_qbeff_dn5 = assign58220_e95027_d_n5;
        locals.var_qbeff_dn6 = assign58220_e95027_d_n6;
        locals.var_qbeff_dn7 = assign58220_e95027_d_n7;
        locals.var_qbeff_dn8 = assign58220_e95027_d_n8;
        locals.var_qbeff_dn9 = assign58220_e95027_d_n9;
        locals.var_qbeff_dn10 = assign58220_e95027_d_n10;
        locals.var_qbeff_dn11 = assign58220_e95027_d_n11;
        locals.var_qbeff_rv = 0.0;

        let (assign58230_e95054, assign58230_e95054_d_n3, assign58230_e95054_d_n4, assign58230_e95054_d_n5, assign58230_e95054_d_n6, assign58230_e95054_d_n7, assign58230_e95054_d_n8, assign58230_e95054_d_n9, assign58230_e95054_d_n10, assign58230_e95054_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58230_e95035: f64 = (locals.var_t1 + locals.var_t2);
        let assign58230_e95038: f64 = (locals.var_t4 * locals.var_t7);
        let assign58230_e95042: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign58230_e95044: f64 = (assign58230_e95042 + locals.var_t8);
        let assign58230_e95045: f64 = (locals.var_nq * assign58230_e95044);
        let assign58230_e95046: f64 = (assign58230_e95038 - assign58230_e95045);
        let assign58230_e95047: f64 = (assign58230_e95035 + assign58230_e95046);
        let assign58230_e95048: f64 = (locals.var_inv_mdl * assign58230_e95047);
        let assign58230_e95051: f64 = (locals.var_mdl_less_1 * locals.var_qbeff);
        let assign58230_e95052: f64 = (assign58230_e95048 + assign58230_e95051);
        (assign58230_e95052, (((locals.var_inv_mdl_dn3 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn3 + locals.var_t2_dn3) + (((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) - ((locals.var_nq_dn3 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn3 + locals.var_qdeff_dn3) + locals.var_t8_dn3))))))) + ((locals.var_mdl_less_1_dn3 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn3))), (((locals.var_inv_mdl_dn4 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn4 + locals.var_t2_dn4) + (((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) - ((locals.var_nq_dn4 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn4 + locals.var_qdeff_dn4) + locals.var_t8_dn4))))))) + ((locals.var_mdl_less_1_dn4 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn4))), (((locals.var_inv_mdl_dn5 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn5 + locals.var_t2_dn5) + (((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) - ((locals.var_nq_dn5 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn5 + locals.var_qdeff_dn5) + locals.var_t8_dn5))))))) + ((locals.var_mdl_less_1_dn5 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn5))), (((locals.var_inv_mdl_dn6 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn6 + locals.var_t2_dn6) + (((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) - ((locals.var_nq_dn6 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn6 + locals.var_qdeff_dn6) + locals.var_t8_dn6))))))) + ((locals.var_mdl_less_1_dn6 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn6))), (((locals.var_inv_mdl_dn7 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn7 + locals.var_t2_dn7) + (((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) - ((locals.var_nq_dn7 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn7 + locals.var_qdeff_dn7) + locals.var_t8_dn7))))))) + ((locals.var_mdl_less_1_dn7 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn7))), (((locals.var_inv_mdl_dn8 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn8 + locals.var_t2_dn8) + (((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) - ((locals.var_nq_dn8 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn8 + locals.var_qdeff_dn8) + locals.var_t8_dn8))))))) + ((locals.var_mdl_less_1_dn8 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn8))), (((locals.var_inv_mdl_dn9 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn9 + locals.var_t2_dn9) + (((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) - ((locals.var_nq_dn9 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn9 + locals.var_qdeff_dn9) + locals.var_t8_dn9))))))) + ((locals.var_mdl_less_1_dn9 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn9))), (((locals.var_inv_mdl_dn10 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn10 + locals.var_t2_dn10) + (((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) - ((locals.var_nq_dn10 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn10 + locals.var_qdeff_dn10) + locals.var_t8_dn10))))))) + ((locals.var_mdl_less_1_dn10 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn10))), (((locals.var_inv_mdl_dn11 * assign58230_e95047) + (locals.var_inv_mdl * ((locals.var_t1_dn11 + locals.var_t2_dn11) + (((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) - ((locals.var_nq_dn11 * assign58230_e95044) + (locals.var_nq * ((locals.var_qs_1_dn11 + locals.var_qdeff_dn11) + locals.var_t8_dn11))))))) + ((locals.var_mdl_less_1_dn11 * locals.var_qbeff) + (locals.var_mdl_less_1 * locals.var_qbeff_dn11))),)
    } else {
        (locals.var_qb_1, locals.var_qb_1_dn3, locals.var_qb_1_dn4, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, locals.var_qb_1_dn9, locals.var_qb_1_dn10, locals.var_qb_1_dn11,)
    }
};
        locals.var_qb_1 = assign58230_e95054;
        locals.var_qb_1_dn3 = assign58230_e95054_d_n3;
        locals.var_qb_1_dn4 = assign58230_e95054_d_n4;
        locals.var_qb_1_dn5 = assign58230_e95054_d_n5;
        locals.var_qb_1_dn6 = assign58230_e95054_d_n6;
        locals.var_qb_1_dn7 = assign58230_e95054_d_n7;
        locals.var_qb_1_dn8 = assign58230_e95054_d_n8;
        locals.var_qb_1_dn9 = assign58230_e95054_d_n9;
        locals.var_qb_1_dn10 = assign58230_e95054_d_n10;
        locals.var_qb_1_dn11 = assign58230_e95054_d_n11;
        locals.var_qb_1_rv = 0.0;

        let (assign58240_e95063, assign58240_e95063_d_n3, assign58240_e95063_d_n4, assign58240_e95063_d_n5, assign58240_e95063_d_n6, assign58240_e95063_d_n7, assign58240_e95063_d_n8, assign58240_e95063_d_n9, assign58240_e95063_d_n10, assign58240_e95063_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58240_e95061: f64 = (locals.var_qs_1 + locals.var_qdeff);
        (assign58240_e95061, (locals.var_qs_1_dn3 + locals.var_qdeff_dn3), (locals.var_qs_1_dn4 + locals.var_qdeff_dn4), (locals.var_qs_1_dn5 + locals.var_qdeff_dn5), (locals.var_qs_1_dn6 + locals.var_qdeff_dn6), (locals.var_qs_1_dn7 + locals.var_qdeff_dn7), (locals.var_qs_1_dn8 + locals.var_qdeff_dn8), (locals.var_qs_1_dn9 + locals.var_qdeff_dn9), (locals.var_qs_1_dn10 + locals.var_qdeff_dn10), (locals.var_qs_1_dn11 + locals.var_qdeff_dn11),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign58240_e95063;
        locals.var_t9_dn3 = assign58240_e95063_d_n3;
        locals.var_t9_dn4 = assign58240_e95063_d_n4;
        locals.var_t9_dn5 = assign58240_e95063_d_n5;
        locals.var_t9_dn6 = assign58240_e95063_d_n6;
        locals.var_t9_dn7 = assign58240_e95063_d_n7;
        locals.var_t9_dn8 = assign58240_e95063_d_n8;
        locals.var_t9_dn9 = assign58240_e95063_d_n9;
        locals.var_t9_dn10 = assign58240_e95063_d_n10;
        locals.var_t9_dn11 = assign58240_e95063_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign58250_e95074, assign58250_e95074_d_n3, assign58250_e95074_d_n4, assign58250_e95074_d_n5, assign58250_e95074_d_n6, assign58250_e95074_d_n7, assign58250_e95074_d_n8, assign58250_e95074_d_n9, assign58250_e95074_d_n10, assign58250_e95074_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58250_e95070: f64 = (locals.var_dqsd2 * locals.var_t5);
        let assign58250_e95072: f64 = (assign58250_e95070 * locals.var_t5);
        (assign58250_e95072, ((((locals.var_dqsd2_dn3 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn3)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn3)), ((((locals.var_dqsd2_dn4 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn4)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn4)), ((((locals.var_dqsd2_dn5 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn5)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn5)), ((((locals.var_dqsd2_dn6 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn6)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn6)), ((((locals.var_dqsd2_dn7 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn7)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn7)), ((((locals.var_dqsd2_dn8 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn8)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn8)), ((((locals.var_dqsd2_dn9 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn9)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn9)), ((((locals.var_dqsd2_dn10 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn10)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn10)), ((((locals.var_dqsd2_dn11 * locals.var_t5) + (locals.var_dqsd2 * locals.var_t5_dn11)) * locals.var_t5) + (assign58250_e95070 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign58250_e95074;
        locals.var_t10_dn3 = assign58250_e95074_d_n3;
        locals.var_t10_dn4 = assign58250_e95074_d_n4;
        locals.var_t10_dn5 = assign58250_e95074_d_n5;
        locals.var_t10_dn6 = assign58250_e95074_d_n6;
        locals.var_t10_dn7 = assign58250_e95074_d_n7;
        locals.var_t10_dn8 = assign58250_e95074_d_n8;
        locals.var_t10_dn9 = assign58250_e95074_d_n9;
        locals.var_t10_dn10 = assign58250_e95074_d_n10;
        locals.var_t10_dn11 = assign58250_e95074_d_n11;
        locals.var_t10_rv = 0.0;

        let (assign58260_e95099, assign58260_e95099_d_n3, assign58260_e95099_d_n4, assign58260_e95099_d_n5, assign58260_e95099_d_n6, assign58260_e95099_d_n7, assign58260_e95099_d_n8, assign58260_e95099_d_n9, assign58260_e95099_d_n10, assign58260_e95099_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58260_e95081: f64 = (locals.var_nq * locals.var_inv_mdl);
        let assign58260_e95085: f64 = (0.3333333333333333 * locals.var_dqsd2);
        let assign58260_e95087: f64 = (assign58260_e95085 * locals.var_t5);
        let assign58260_e95088: f64 = (locals.var_t9 + assign58260_e95087);
        let assign58260_e95089: f64 = (assign58260_e95081 * assign58260_e95088);
        let assign58260_e95092: f64 = (2.0 * locals.var_nq);
        let assign58260_e95094: f64 = (assign58260_e95092 * locals.var_mdl_less_1);
        let assign58260_e95096: f64 = (assign58260_e95094 * locals.var_qdeff);
        let assign58260_e95097: f64 = (assign58260_e95089 + assign58260_e95096);
        (assign58260_e95097, (((((locals.var_nq_dn3 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn3)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn3 + (((0.3333333333333333 * locals.var_dqsd2_dn3) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn3))))) + (((((2.0 * locals.var_nq_dn3) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn3)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn3))), (((((locals.var_nq_dn4 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn4)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn4 + (((0.3333333333333333 * locals.var_dqsd2_dn4) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn4))))) + (((((2.0 * locals.var_nq_dn4) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn4)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn4))), (((((locals.var_nq_dn5 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn5)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn5 + (((0.3333333333333333 * locals.var_dqsd2_dn5) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn5))))) + (((((2.0 * locals.var_nq_dn5) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn5)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn5))), (((((locals.var_nq_dn6 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn6)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn6 + (((0.3333333333333333 * locals.var_dqsd2_dn6) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn6))))) + (((((2.0 * locals.var_nq_dn6) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn6)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn6))), (((((locals.var_nq_dn7 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn7)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn7 + (((0.3333333333333333 * locals.var_dqsd2_dn7) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn7))))) + (((((2.0 * locals.var_nq_dn7) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn7)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn7))), (((((locals.var_nq_dn8 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn8)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn8 + (((0.3333333333333333 * locals.var_dqsd2_dn8) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn8))))) + (((((2.0 * locals.var_nq_dn8) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn8)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn8))), (((((locals.var_nq_dn9 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn9)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn9 + (((0.3333333333333333 * locals.var_dqsd2_dn9) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn9))))) + (((((2.0 * locals.var_nq_dn9) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn9)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn9))), (((((locals.var_nq_dn10 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn10)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn10 + (((0.3333333333333333 * locals.var_dqsd2_dn10) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn10))))) + (((((2.0 * locals.var_nq_dn10) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn10)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn10))), (((((locals.var_nq_dn11 * locals.var_inv_mdl) + (locals.var_nq * locals.var_inv_mdl_dn11)) * assign58260_e95088) + (assign58260_e95081 * (locals.var_t9_dn11 + (((0.3333333333333333 * locals.var_dqsd2_dn11) * locals.var_t5) + (assign58260_e95085 * locals.var_t5_dn11))))) + (((((2.0 * locals.var_nq_dn11) * locals.var_mdl_less_1) + (assign58260_e95092 * locals.var_mdl_less_1_dn11)) * locals.var_qdeff) + (assign58260_e95094 * locals.var_qdeff_dn11))),)
    } else {
        (locals.var_qi_1, locals.var_qi_1_dn3, locals.var_qi_1_dn4, locals.var_qi_1_dn5, locals.var_qi_1_dn6, locals.var_qi_1_dn7, locals.var_qi_1_dn8, locals.var_qi_1_dn9, locals.var_qi_1_dn10, locals.var_qi_1_dn11,)
    }
};
        locals.var_qi_1 = assign58260_e95099;
        locals.var_qi_1_dn3 = assign58260_e95099_d_n3;
        locals.var_qi_1_dn4 = assign58260_e95099_d_n4;
        locals.var_qi_1_dn5 = assign58260_e95099_d_n5;
        locals.var_qi_1_dn6 = assign58260_e95099_d_n6;
        locals.var_qi_1_dn7 = assign58260_e95099_d_n7;
        locals.var_qi_1_dn8 = assign58260_e95099_d_n8;
        locals.var_qi_1_dn9 = assign58260_e95099_d_n9;
        locals.var_qi_1_dn10 = assign58260_e95099_d_n10;
        locals.var_qi_1_dn11 = assign58260_e95099_d_n11;
        locals.var_qi_1_rv = 0.0;

        let (assign58270_e95126, assign58270_e95126_d_n3, assign58270_e95126_d_n4, assign58270_e95126_d_n5, assign58270_e95126_d_n6, assign58270_e95126_d_n7, assign58270_e95126_d_n8, assign58270_e95126_d_n9, assign58270_e95126_d_n10, assign58270_e95126_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58270_e95106: f64 = (locals.var_nq * locals.var_inv_mdl_2);
        let assign58270_e95109: f64 = (0.5 * locals.var_t9);
        let assign58270_e95112: f64 = (locals.var_dqsd / 6.0);
        let assign58270_e95116: f64 = (locals.var_dqsd * locals.var_t5);
        let assign58270_e95117: f64 = (1.0 - assign58270_e95116);
        let assign58270_e95120: f64 = (0.2 * locals.var_t10);
        let assign58270_e95121: f64 = (assign58270_e95117 - assign58270_e95120);
        let assign58270_e95122: f64 = (assign58270_e95112 * assign58270_e95121);
        let assign58270_e95123: f64 = (assign58270_e95109 - assign58270_e95122);
        let assign58270_e95124: f64 = (assign58270_e95106 * assign58270_e95123);
        (assign58270_e95124, ((((locals.var_nq_dn3 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn3)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn3) - (((locals.var_dqsd_dn3 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn3 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn3))) - (0.2 * locals.var_t10_dn3))))))), ((((locals.var_nq_dn4 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn4)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn4) - (((locals.var_dqsd_dn4 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn4 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn4))) - (0.2 * locals.var_t10_dn4))))))), ((((locals.var_nq_dn5 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn5)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn5) - (((locals.var_dqsd_dn5 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn5 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn5))) - (0.2 * locals.var_t10_dn5))))))), ((((locals.var_nq_dn6 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn6)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn6) - (((locals.var_dqsd_dn6 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn6 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn6))) - (0.2 * locals.var_t10_dn6))))))), ((((locals.var_nq_dn7 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn7)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn7) - (((locals.var_dqsd_dn7 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn7 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn7))) - (0.2 * locals.var_t10_dn7))))))), ((((locals.var_nq_dn8 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn8)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn8) - (((locals.var_dqsd_dn8 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn8 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn8))) - (0.2 * locals.var_t10_dn8))))))), ((((locals.var_nq_dn9 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn9)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn9) - (((locals.var_dqsd_dn9 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn9 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn9))) - (0.2 * locals.var_t10_dn9))))))), ((((locals.var_nq_dn10 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn10)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn10) - (((locals.var_dqsd_dn10 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn10 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn10))) - (0.2 * locals.var_t10_dn10))))))), ((((locals.var_nq_dn11 * locals.var_inv_mdl_2) + (locals.var_nq * locals.var_inv_mdl_2_dn11)) * assign58270_e95123) + (assign58270_e95106 * ((0.5 * locals.var_t9_dn11) - (((locals.var_dqsd_dn11 / 6.0) * assign58270_e95121) + (assign58270_e95112 * ((-((locals.var_dqsd_dn11 * locals.var_t5) + (locals.var_dqsd * locals.var_t5_dn11))) - (0.2 * locals.var_t10_dn11))))))),)
    } else {
        (locals.var_qd1, locals.var_qd1_dn3, locals.var_qd1_dn4, locals.var_qd1_dn5, locals.var_qd1_dn6, locals.var_qd1_dn7, locals.var_qd1_dn8, locals.var_qd1_dn9, locals.var_qd1_dn10, locals.var_qd1_dn11,)
    }
};
        locals.var_qd1 = assign58270_e95126;
        locals.var_qd1_dn3 = assign58270_e95126_d_n3;
        locals.var_qd1_dn4 = assign58270_e95126_d_n4;
        locals.var_qd1_dn5 = assign58270_e95126_d_n5;
        locals.var_qd1_dn6 = assign58270_e95126_d_n6;
        locals.var_qd1_dn7 = assign58270_e95126_d_n7;
        locals.var_qd1_dn8 = assign58270_e95126_d_n8;
        locals.var_qd1_dn9 = assign58270_e95126_d_n9;
        locals.var_qd1_dn10 = assign58270_e95126_d_n10;
        locals.var_qd1_dn11 = assign58270_e95126_d_n11;
        locals.var_qd1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_203(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign58280_e95139, assign58280_e95139_d_n3, assign58280_e95139_d_n4, assign58280_e95139_d_n5, assign58280_e95139_d_n6, assign58280_e95139_d_n7, assign58280_e95139_d_n8, assign58280_e95139_d_n9, assign58280_e95139_d_n10, assign58280_e95139_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58280_e95134: f64 = (locals.var_mdl - locals.var_inv_mdl);
        let assign58280_e95135: f64 = (locals.var_nq * assign58280_e95134);
        let assign58280_e95137: f64 = (assign58280_e95135 * locals.var_qdeff);
        (assign58280_e95137, ((((locals.var_nq_dn3 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn3 - locals.var_inv_mdl_dn3))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn3)), ((((locals.var_nq_dn4 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn4 - locals.var_inv_mdl_dn4))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn4)), ((((locals.var_nq_dn5 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn5 - locals.var_inv_mdl_dn5))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn5)), ((((locals.var_nq_dn6 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn6 - locals.var_inv_mdl_dn6))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn6)), ((((locals.var_nq_dn7 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn7 - locals.var_inv_mdl_dn7))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn7)), ((((locals.var_nq_dn8 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn8 - locals.var_inv_mdl_dn8))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn8)), ((((locals.var_nq_dn9 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn9 - locals.var_inv_mdl_dn9))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn9)), ((((locals.var_nq_dn10 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn10 - locals.var_inv_mdl_dn10))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn10)), ((((locals.var_nq_dn11 * assign58280_e95134) + (locals.var_nq * (locals.var_mdl_dn11 - locals.var_inv_mdl_dn11))) * locals.var_qdeff) + (assign58280_e95135 * locals.var_qdeff_dn11)),)
    } else {
        (locals.var_qd2, locals.var_qd2_dn3, locals.var_qd2_dn4, locals.var_qd2_dn5, locals.var_qd2_dn6, locals.var_qd2_dn7, locals.var_qd2_dn8, locals.var_qd2_dn9, locals.var_qd2_dn10, locals.var_qd2_dn11,)
    }
};
        locals.var_qd2 = assign58280_e95139;
        locals.var_qd2_dn3 = assign58280_e95139_d_n3;
        locals.var_qd2_dn4 = assign58280_e95139_d_n4;
        locals.var_qd2_dn5 = assign58280_e95139_d_n5;
        locals.var_qd2_dn6 = assign58280_e95139_d_n6;
        locals.var_qd2_dn7 = assign58280_e95139_d_n7;
        locals.var_qd2_dn8 = assign58280_e95139_d_n8;
        locals.var_qd2_dn9 = assign58280_e95139_d_n9;
        locals.var_qd2_dn10 = assign58280_e95139_d_n10;
        locals.var_qd2_dn11 = assign58280_e95139_d_n11;
        locals.var_qd2_rv = 0.0;

        let (assign58290_e95148, assign58290_e95148_d_n3, assign58290_e95148_d_n4, assign58290_e95148_d_n5, assign58290_e95148_d_n6, assign58290_e95148_d_n7, assign58290_e95148_d_n8, assign58290_e95148_d_n9, assign58290_e95148_d_n10, assign58290_e95148_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58290_e95146: f64 = (locals.var_qd1 + locals.var_qd2);
        (assign58290_e95146, (locals.var_qd1_dn3 + locals.var_qd2_dn3), (locals.var_qd1_dn4 + locals.var_qd2_dn4), (locals.var_qd1_dn5 + locals.var_qd2_dn5), (locals.var_qd1_dn6 + locals.var_qd2_dn6), (locals.var_qd1_dn7 + locals.var_qd2_dn7), (locals.var_qd1_dn8 + locals.var_qd2_dn8), (locals.var_qd1_dn9 + locals.var_qd2_dn9), (locals.var_qd1_dn10 + locals.var_qd2_dn10), (locals.var_qd1_dn11 + locals.var_qd2_dn11),)
    } else {
        (locals.var_qd, locals.var_qd_dn3, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11,)
    }
};
        locals.var_qd = assign58290_e95148;
        locals.var_qd_dn3 = assign58290_e95148_d_n3;
        locals.var_qd_dn4 = assign58290_e95148_d_n4;
        locals.var_qd_dn5 = assign58290_e95148_d_n5;
        locals.var_qd_dn6 = assign58290_e95148_d_n6;
        locals.var_qd_dn7 = assign58290_e95148_d_n7;
        locals.var_qd_dn8 = assign58290_e95148_d_n8;
        locals.var_qd_dn9 = assign58290_e95148_d_n9;
        locals.var_qd_dn10 = assign58290_e95148_d_n10;
        locals.var_qd_dn11 = assign58290_e95148_d_n11;
        locals.var_qd_rv = 0.0;

        let (assign58300_e95163, assign58300_e95163_d_n4, assign58300_e95163_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58300_e95156: f64 = (8.8541878128e-12 * p.p110);
        let assign58300_e95158: f64 = (assign58300_e95156 / locals.var_bsimbulktoxp);
        let assign58300_e95159: f64 = (p.p1380 * assign58300_e95158);
        let assign58300_e95161: f64 = (assign58300_e95159 * locals.var_vt);
        (assign58300_e95161, (assign58300_e95159 * locals.var_vt_dn4), (assign58300_e95159 * locals.var_vt_dn5),)
    } else {
        (locals.var_wlcox, locals.var_wlcox_dn4, locals.var_wlcox_dn5,)
    }
};
        locals.var_wlcox = assign58300_e95163;
        locals.var_wlcox_dn4 = assign58300_e95163_d_n4;
        locals.var_wlcox_dn5 = assign58300_e95163_d_n5;
        locals.var_wlcox_rv = 0.0;

        let (assign58310_e95172, assign58310_e95172_d_n3, assign58310_e95172_d_n4, assign58310_e95172_d_n5, assign58310_e95172_d_n6, assign58310_e95172_d_n7, assign58310_e95172_d_n8, assign58310_e95172_d_n9, assign58310_e95172_d_n10, assign58310_e95172_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58310_e95170: f64 = (locals.var_wlcox * locals.var_qb_1);
        (assign58310_e95170, (locals.var_wlcox * locals.var_qb_1_dn3), ((locals.var_wlcox_dn4 * locals.var_qb_1) + (locals.var_wlcox * locals.var_qb_1_dn4)), ((locals.var_wlcox_dn5 * locals.var_qb_1) + (locals.var_wlcox * locals.var_qb_1_dn5)), (locals.var_wlcox * locals.var_qb_1_dn6), (locals.var_wlcox * locals.var_qb_1_dn7), (locals.var_wlcox * locals.var_qb_1_dn8), (locals.var_wlcox * locals.var_qb_1_dn9), (locals.var_wlcox * locals.var_qb_1_dn10), (locals.var_wlcox * locals.var_qb_1_dn11),)
    } else {
        (locals.var_qbi_agbcp2, locals.var_qbi_agbcp2_dn3, locals.var_qbi_agbcp2_dn4, locals.var_qbi_agbcp2_dn5, locals.var_qbi_agbcp2_dn6, locals.var_qbi_agbcp2_dn7, locals.var_qbi_agbcp2_dn8, locals.var_qbi_agbcp2_dn9, locals.var_qbi_agbcp2_dn10, locals.var_qbi_agbcp2_dn11,)
    }
};
        locals.var_qbi_agbcp2 = assign58310_e95172;
        locals.var_qbi_agbcp2_dn3 = assign58310_e95172_d_n3;
        locals.var_qbi_agbcp2_dn4 = assign58310_e95172_d_n4;
        locals.var_qbi_agbcp2_dn5 = assign58310_e95172_d_n5;
        locals.var_qbi_agbcp2_dn6 = assign58310_e95172_d_n6;
        locals.var_qbi_agbcp2_dn7 = assign58310_e95172_d_n7;
        locals.var_qbi_agbcp2_dn8 = assign58310_e95172_d_n8;
        locals.var_qbi_agbcp2_dn9 = assign58310_e95172_d_n9;
        locals.var_qbi_agbcp2_dn10 = assign58310_e95172_d_n10;
        locals.var_qbi_agbcp2_dn11 = assign58310_e95172_d_n11;
        locals.var_qbi_agbcp2_rv = 0.0;

        let (assign58320_e95181, assign58320_e95181_d_n3, assign58320_e95181_d_n4, assign58320_e95181_d_n5, assign58320_e95181_d_n6, assign58320_e95181_d_n7, assign58320_e95181_d_n8, assign58320_e95181_d_n9, assign58320_e95181_d_n10, assign58320_e95181_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58320_e95179: f64 = (locals.var_wlcox * locals.var_qd);
        (assign58320_e95179, (locals.var_wlcox * locals.var_qd_dn3), ((locals.var_wlcox_dn4 * locals.var_qd) + (locals.var_wlcox * locals.var_qd_dn4)), ((locals.var_wlcox_dn5 * locals.var_qd) + (locals.var_wlcox * locals.var_qd_dn5)), (locals.var_wlcox * locals.var_qd_dn6), (locals.var_wlcox * locals.var_qd_dn7), (locals.var_wlcox * locals.var_qd_dn8), (locals.var_wlcox * locals.var_qd_dn9), (locals.var_wlcox * locals.var_qd_dn10), (locals.var_wlcox * locals.var_qd_dn11),)
    } else {
        (locals.var_qdi_agbcp2, locals.var_qdi_agbcp2_dn3, locals.var_qdi_agbcp2_dn4, locals.var_qdi_agbcp2_dn5, locals.var_qdi_agbcp2_dn6, locals.var_qdi_agbcp2_dn7, locals.var_qdi_agbcp2_dn8, locals.var_qdi_agbcp2_dn9, locals.var_qdi_agbcp2_dn10, locals.var_qdi_agbcp2_dn11,)
    }
};
        locals.var_qdi_agbcp2 = assign58320_e95181;
        locals.var_qdi_agbcp2_dn3 = assign58320_e95181_d_n3;
        locals.var_qdi_agbcp2_dn4 = assign58320_e95181_d_n4;
        locals.var_qdi_agbcp2_dn5 = assign58320_e95181_d_n5;
        locals.var_qdi_agbcp2_dn6 = assign58320_e95181_d_n6;
        locals.var_qdi_agbcp2_dn7 = assign58320_e95181_d_n7;
        locals.var_qdi_agbcp2_dn8 = assign58320_e95181_d_n8;
        locals.var_qdi_agbcp2_dn9 = assign58320_e95181_d_n9;
        locals.var_qdi_agbcp2_dn10 = assign58320_e95181_d_n10;
        locals.var_qdi_agbcp2_dn11 = assign58320_e95181_d_n11;
        locals.var_qdi_agbcp2_rv = 0.0;

        let (assign58330_e95190, assign58330_e95190_d_n3, assign58330_e95190_d_n4, assign58330_e95190_d_n5, assign58330_e95190_d_n6, assign58330_e95190_d_n7, assign58330_e95190_d_n8, assign58330_e95190_d_n9, assign58330_e95190_d_n10, assign58330_e95190_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 != 0.0)) {
        let assign58330_e95188: f64 = (locals.var_wlcox * locals.var_qi_1);
        (assign58330_e95188, (locals.var_wlcox * locals.var_qi_1_dn3), ((locals.var_wlcox_dn4 * locals.var_qi_1) + (locals.var_wlcox * locals.var_qi_1_dn4)), ((locals.var_wlcox_dn5 * locals.var_qi_1) + (locals.var_wlcox * locals.var_qi_1_dn5)), (locals.var_wlcox * locals.var_qi_1_dn6), (locals.var_wlcox * locals.var_qi_1_dn7), (locals.var_wlcox * locals.var_qi_1_dn8), (locals.var_wlcox * locals.var_qi_1_dn9), (locals.var_wlcox * locals.var_qi_1_dn10), (locals.var_wlcox * locals.var_qi_1_dn11),)
    } else {
        (locals.var_qi_agbcp2, locals.var_qi_agbcp2_dn3, locals.var_qi_agbcp2_dn4, locals.var_qi_agbcp2_dn5, locals.var_qi_agbcp2_dn6, locals.var_qi_agbcp2_dn7, locals.var_qi_agbcp2_dn8, locals.var_qi_agbcp2_dn9, locals.var_qi_agbcp2_dn10, locals.var_qi_agbcp2_dn11,)
    }
};
        locals.var_qi_agbcp2 = assign58330_e95190;
        locals.var_qi_agbcp2_dn3 = assign58330_e95190_d_n3;
        locals.var_qi_agbcp2_dn4 = assign58330_e95190_d_n4;
        locals.var_qi_agbcp2_dn5 = assign58330_e95190_d_n5;
        locals.var_qi_agbcp2_dn6 = assign58330_e95190_d_n6;
        locals.var_qi_agbcp2_dn7 = assign58330_e95190_d_n7;
        locals.var_qi_agbcp2_dn8 = assign58330_e95190_d_n8;
        locals.var_qi_agbcp2_dn9 = assign58330_e95190_d_n9;
        locals.var_qi_agbcp2_dn10 = assign58330_e95190_d_n10;
        locals.var_qi_agbcp2_dn11 = assign58330_e95190_d_n11;
        locals.var_qi_agbcp2_rv = 0.0;

        let (assign58340_e95198, assign58340_e95198_d_n3, assign58340_e95198_d_n4, assign58340_e95198_d_n5, assign58340_e95198_d_n6, assign58340_e95198_d_n7, assign58340_e95198_d_n8, assign58340_e95198_d_n9, assign58340_e95198_d_n10, assign58340_e95198_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbi_agbcp2, locals.var_qbi_agbcp2_dn3, locals.var_qbi_agbcp2_dn4, locals.var_qbi_agbcp2_dn5, locals.var_qbi_agbcp2_dn6, locals.var_qbi_agbcp2_dn7, locals.var_qbi_agbcp2_dn8, locals.var_qbi_agbcp2_dn9, locals.var_qbi_agbcp2_dn10, locals.var_qbi_agbcp2_dn11,)
    }
};
        locals.var_qbi_agbcp2 = assign58340_e95198;
        locals.var_qbi_agbcp2_dn3 = assign58340_e95198_d_n3;
        locals.var_qbi_agbcp2_dn4 = assign58340_e95198_d_n4;
        locals.var_qbi_agbcp2_dn5 = assign58340_e95198_d_n5;
        locals.var_qbi_agbcp2_dn6 = assign58340_e95198_d_n6;
        locals.var_qbi_agbcp2_dn7 = assign58340_e95198_d_n7;
        locals.var_qbi_agbcp2_dn8 = assign58340_e95198_d_n8;
        locals.var_qbi_agbcp2_dn9 = assign58340_e95198_d_n9;
        locals.var_qbi_agbcp2_dn10 = assign58340_e95198_d_n10;
        locals.var_qbi_agbcp2_dn11 = assign58340_e95198_d_n11;
        locals.var_qbi_agbcp2_rv = 0.0;

        let (assign58350_e95206, assign58350_e95206_d_n3, assign58350_e95206_d_n4, assign58350_e95206_d_n5, assign58350_e95206_d_n6, assign58350_e95206_d_n7, assign58350_e95206_d_n8, assign58350_e95206_d_n9, assign58350_e95206_d_n10, assign58350_e95206_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdi_agbcp2, locals.var_qdi_agbcp2_dn3, locals.var_qdi_agbcp2_dn4, locals.var_qdi_agbcp2_dn5, locals.var_qdi_agbcp2_dn6, locals.var_qdi_agbcp2_dn7, locals.var_qdi_agbcp2_dn8, locals.var_qdi_agbcp2_dn9, locals.var_qdi_agbcp2_dn10, locals.var_qdi_agbcp2_dn11,)
    }
};
        locals.var_qdi_agbcp2 = assign58350_e95206;
        locals.var_qdi_agbcp2_dn3 = assign58350_e95206_d_n3;
        locals.var_qdi_agbcp2_dn4 = assign58350_e95206_d_n4;
        locals.var_qdi_agbcp2_dn5 = assign58350_e95206_d_n5;
        locals.var_qdi_agbcp2_dn6 = assign58350_e95206_d_n6;
        locals.var_qdi_agbcp2_dn7 = assign58350_e95206_d_n7;
        locals.var_qdi_agbcp2_dn8 = assign58350_e95206_d_n8;
        locals.var_qdi_agbcp2_dn9 = assign58350_e95206_d_n9;
        locals.var_qdi_agbcp2_dn10 = assign58350_e95206_d_n10;
        locals.var_qdi_agbcp2_dn11 = assign58350_e95206_d_n11;
        locals.var_qdi_agbcp2_rv = 0.0;

        let (assign58360_e95214, assign58360_e95214_d_n3, assign58360_e95214_d_n4, assign58360_e95214_d_n5, assign58360_e95214_d_n6, assign58360_e95214_d_n7, assign58360_e95214_d_n8, assign58360_e95214_d_n9, assign58360_e95214_d_n10, assign58360_e95214_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard853 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qi_agbcp2, locals.var_qi_agbcp2_dn3, locals.var_qi_agbcp2_dn4, locals.var_qi_agbcp2_dn5, locals.var_qi_agbcp2_dn6, locals.var_qi_agbcp2_dn7, locals.var_qi_agbcp2_dn8, locals.var_qi_agbcp2_dn9, locals.var_qi_agbcp2_dn10, locals.var_qi_agbcp2_dn11,)
    }
};
        locals.var_qi_agbcp2 = assign58360_e95214;
        locals.var_qi_agbcp2_dn3 = assign58360_e95214_d_n3;
        locals.var_qi_agbcp2_dn4 = assign58360_e95214_d_n4;
        locals.var_qi_agbcp2_dn5 = assign58360_e95214_d_n5;
        locals.var_qi_agbcp2_dn6 = assign58360_e95214_d_n6;
        locals.var_qi_agbcp2_dn7 = assign58360_e95214_d_n7;
        locals.var_qi_agbcp2_dn8 = assign58360_e95214_d_n8;
        locals.var_qi_agbcp2_dn9 = assign58360_e95214_d_n9;
        locals.var_qi_agbcp2_dn10 = assign58360_e95214_d_n10;
        locals.var_qi_agbcp2_dn11 = assign58360_e95214_d_n11;
        locals.var_qi_agbcp2_rv = 0.0;

        let (assign58450_e95288, assign58450_e95288_d_n3, assign58450_e95288_d_n4, assign58450_e95288_d_n5, assign58450_e95288_d_n6, assign58450_e95288_d_n7, assign58450_e95288_d_n8, assign58450_e95288_d_n9, assign58450_e95288_d_n10, assign58450_e95288_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58450_e95281: f64 = (-locals.var_qdi);
        let assign58450_e95284: f64 = (p.p45 * locals.var_qdi_agbcp2);
        let assign58450_e95285: f64 = (assign58450_e95281 + assign58450_e95284);
        let assign58450_e95286: f64 = (-assign58450_e95285);
        (assign58450_e95286, (-((-locals.var_qdi_dn3) + (p.p45 * locals.var_qdi_agbcp2_dn3))), (-((-locals.var_qdi_dn4) + (p.p45 * locals.var_qdi_agbcp2_dn4))), (-((-locals.var_qdi_dn5) + (p.p45 * locals.var_qdi_agbcp2_dn5))), (-((-locals.var_qdi_dn6) + (p.p45 * locals.var_qdi_agbcp2_dn6))), (-((-locals.var_qdi_dn7) + (p.p45 * locals.var_qdi_agbcp2_dn7))), (-((-locals.var_qdi_dn8) + (p.p45 * locals.var_qdi_agbcp2_dn8))), (-((-locals.var_qdi_dn9) + (p.p45 * locals.var_qdi_agbcp2_dn9))), (-((-locals.var_qdi_dn10) + (p.p45 * locals.var_qdi_agbcp2_dn10))), (-((-locals.var_qdi_dn11) + (p.p45 * locals.var_qdi_agbcp2_dn11))),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8, locals.var_qdi_dn9, locals.var_qdi_dn10, locals.var_qdi_dn11,)
    }
};
        locals.var_qdi = assign58450_e95288;
        locals.var_qdi_dn3 = assign58450_e95288_d_n3;
        locals.var_qdi_dn4 = assign58450_e95288_d_n4;
        locals.var_qdi_dn5 = assign58450_e95288_d_n5;
        locals.var_qdi_dn6 = assign58450_e95288_d_n6;
        locals.var_qdi_dn7 = assign58450_e95288_d_n7;
        locals.var_qdi_dn8 = assign58450_e95288_d_n8;
        locals.var_qdi_dn9 = assign58450_e95288_d_n9;
        locals.var_qdi_dn10 = assign58450_e95288_d_n10;
        locals.var_qdi_dn11 = assign58450_e95288_d_n11;
        locals.var_qdi_rv = 0.0;

        let (assign58460_e95303, assign58460_e95303_d_n3, assign58460_e95303_d_n4, assign58460_e95303_d_n5, assign58460_e95303_d_n6, assign58460_e95303_d_n7, assign58460_e95303_d_n8, assign58460_e95303_d_n9, assign58460_e95303_d_n10, assign58460_e95303_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58460_e95292: f64 = (-locals.var_qsi);
        let assign58460_e95295: f64 = (p.p45 * locals.var_qi_agbcp2);
        let assign58460_e95296: f64 = (assign58460_e95292 + assign58460_e95295);
        let assign58460_e95299: f64 = (p.p45 * locals.var_qdi_agbcp2);
        let assign58460_e95300: f64 = (assign58460_e95296 - assign58460_e95299);
        let assign58460_e95301: f64 = (-assign58460_e95300);
        (assign58460_e95301, (-(((-locals.var_qsi_dn3) + (p.p45 * locals.var_qi_agbcp2_dn3)) - (p.p45 * locals.var_qdi_agbcp2_dn3))), (-(((-locals.var_qsi_dn4) + (p.p45 * locals.var_qi_agbcp2_dn4)) - (p.p45 * locals.var_qdi_agbcp2_dn4))), (-(((-locals.var_qsi_dn5) + (p.p45 * locals.var_qi_agbcp2_dn5)) - (p.p45 * locals.var_qdi_agbcp2_dn5))), (-(((-locals.var_qsi_dn6) + (p.p45 * locals.var_qi_agbcp2_dn6)) - (p.p45 * locals.var_qdi_agbcp2_dn6))), (-(((-locals.var_qsi_dn7) + (p.p45 * locals.var_qi_agbcp2_dn7)) - (p.p45 * locals.var_qdi_agbcp2_dn7))), (-(((-locals.var_qsi_dn8) + (p.p45 * locals.var_qi_agbcp2_dn8)) - (p.p45 * locals.var_qdi_agbcp2_dn8))), (-(((-locals.var_qsi_dn9) + (p.p45 * locals.var_qi_agbcp2_dn9)) - (p.p45 * locals.var_qdi_agbcp2_dn9))), (-(((-locals.var_qsi_dn10) + (p.p45 * locals.var_qi_agbcp2_dn10)) - (p.p45 * locals.var_qdi_agbcp2_dn10))), (-(((-locals.var_qsi_dn11) + (p.p45 * locals.var_qi_agbcp2_dn11)) - (p.p45 * locals.var_qdi_agbcp2_dn11))),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8, locals.var_qsi_dn9, locals.var_qsi_dn10, locals.var_qsi_dn11,)
    }
};
        locals.var_qsi = assign58460_e95303;
        locals.var_qsi_dn3 = assign58460_e95303_d_n3;
        locals.var_qsi_dn4 = assign58460_e95303_d_n4;
        locals.var_qsi_dn5 = assign58460_e95303_d_n5;
        locals.var_qsi_dn6 = assign58460_e95303_d_n6;
        locals.var_qsi_dn7 = assign58460_e95303_d_n7;
        locals.var_qsi_dn8 = assign58460_e95303_d_n8;
        locals.var_qsi_dn9 = assign58460_e95303_d_n9;
        locals.var_qsi_dn10 = assign58460_e95303_d_n10;
        locals.var_qsi_dn11 = assign58460_e95303_d_n11;
        locals.var_qsi_rv = 0.0;

        let (assign58470_e95314, assign58470_e95314_d_n3, assign58470_e95314_d_n4, assign58470_e95314_d_n5, assign58470_e95314_d_n6, assign58470_e95314_d_n7, assign58470_e95314_d_n8, assign58470_e95314_d_n9, assign58470_e95314_d_n10, assign58470_e95314_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58470_e95307: f64 = (-locals.var_qbi);
        let assign58470_e95310: f64 = (p.p45 * locals.var_qbi_agbcp2);
        let assign58470_e95311: f64 = (assign58470_e95307 + assign58470_e95310);
        let assign58470_e95312: f64 = (-assign58470_e95311);
        (assign58470_e95312, (-((-locals.var_qbi_dn3) + (p.p45 * locals.var_qbi_agbcp2_dn3))), (-((-locals.var_qbi_dn4) + (p.p45 * locals.var_qbi_agbcp2_dn4))), (-((-locals.var_qbi_dn5) + (p.p45 * locals.var_qbi_agbcp2_dn5))), (-((-locals.var_qbi_dn6) + (p.p45 * locals.var_qbi_agbcp2_dn6))), (-((-locals.var_qbi_dn7) + (p.p45 * locals.var_qbi_agbcp2_dn7))), (-((-locals.var_qbi_dn8) + (p.p45 * locals.var_qbi_agbcp2_dn8))), (-((-locals.var_qbi_dn9) + (p.p45 * locals.var_qbi_agbcp2_dn9))), (-((-locals.var_qbi_dn10) + (p.p45 * locals.var_qbi_agbcp2_dn10))), (-((-locals.var_qbi_dn11) + (p.p45 * locals.var_qbi_agbcp2_dn11))),)
    } else {
        (locals.var_qbi, locals.var_qbi_dn3, locals.var_qbi_dn4, locals.var_qbi_dn5, locals.var_qbi_dn6, locals.var_qbi_dn7, locals.var_qbi_dn8, locals.var_qbi_dn9, locals.var_qbi_dn10, locals.var_qbi_dn11,)
    }
};
        locals.var_qbi = assign58470_e95314;
        locals.var_qbi_dn3 = assign58470_e95314_d_n3;
        locals.var_qbi_dn4 = assign58470_e95314_d_n4;
        locals.var_qbi_dn5 = assign58470_e95314_d_n5;
        locals.var_qbi_dn6 = assign58470_e95314_d_n6;
        locals.var_qbi_dn7 = assign58470_e95314_d_n7;
        locals.var_qbi_dn8 = assign58470_e95314_d_n8;
        locals.var_qbi_dn9 = assign58470_e95314_d_n9;
        locals.var_qbi_dn10 = assign58470_e95314_d_n10;
        locals.var_qbi_dn11 = assign58470_e95314_d_n11;
        locals.var_qbi_rv = 0.0;

        let (assign58480_e95324, assign58480_e95324_d_n3, assign58480_e95324_d_n4, assign58480_e95324_d_n5, assign58480_e95324_d_n6, assign58480_e95324_d_n7, assign58480_e95324_d_n8, assign58480_e95324_d_n9, assign58480_e95324_d_n10, assign58480_e95324_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58480_e95319: f64 = (locals.var_qbi + locals.var_qsi);
        let assign58480_e95321: f64 = (assign58480_e95319 + locals.var_qdi);
        let assign58480_e95322: f64 = (-assign58480_e95321);
        (assign58480_e95322, (-((locals.var_qbi_dn3 + locals.var_qsi_dn3) + locals.var_qdi_dn3)), (-((locals.var_qbi_dn4 + locals.var_qsi_dn4) + locals.var_qdi_dn4)), (-((locals.var_qbi_dn5 + locals.var_qsi_dn5) + locals.var_qdi_dn5)), (-((locals.var_qbi_dn6 + locals.var_qsi_dn6) + locals.var_qdi_dn6)), (-((locals.var_qbi_dn7 + locals.var_qsi_dn7) + locals.var_qdi_dn7)), (-((locals.var_qbi_dn8 + locals.var_qsi_dn8) + locals.var_qdi_dn8)), (-((locals.var_qbi_dn9 + locals.var_qsi_dn9) + locals.var_qdi_dn9)), (-((locals.var_qbi_dn10 + locals.var_qsi_dn10) + locals.var_qdi_dn10)), (-((locals.var_qbi_dn11 + locals.var_qsi_dn11) + locals.var_qdi_dn11)),)
    } else {
        (locals.var_qgi, locals.var_qgi_dn3, locals.var_qgi_dn4, locals.var_qgi_dn5, locals.var_qgi_dn6, locals.var_qgi_dn7, locals.var_qgi_dn8, locals.var_qgi_dn9, locals.var_qgi_dn10, locals.var_qgi_dn11,)
    }
};
        locals.var_qgi = assign58480_e95324;
        locals.var_qgi_dn3 = assign58480_e95324_d_n3;
        locals.var_qgi_dn4 = assign58480_e95324_d_n4;
        locals.var_qgi_dn5 = assign58480_e95324_d_n5;
        locals.var_qgi_dn6 = assign58480_e95324_d_n6;
        locals.var_qgi_dn7 = assign58480_e95324_d_n7;
        locals.var_qgi_dn8 = assign58480_e95324_d_n8;
        locals.var_qgi_dn9 = assign58480_e95324_d_n9;
        locals.var_qgi_dn10 = assign58480_e95324_d_n10;
        locals.var_qgi_dn11 = assign58480_e95324_d_n11;
        locals.var_qgi_rv = 0.0;

        let assign58490_e95327: f64 = if (!param_given[867]) { 1.0 } else { 0.0 };
        locals.var_guard861 = assign58490_e95327;
        locals.var_guard861_rv = 0.0;

        let (assign58500_e95351,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard861 != 0.0)) {
        let assign58500_e95334: f64 = (2.0 * p.p110);
        let assign58500_e95336: f64 = (assign58500_e95334 * 8.8541878128e-12);
        let assign58500_e95338: f64 = (assign58500_e95336 / 3.141592653589793);
        let assign58500_e95343: f64 = (4e-7 / p.p76);
        let assign58500_e95344: f64 = (1.0 + assign58500_e95343);
        let assign58500_e95345: f64 = (p.p871 * assign58500_e95344);
        let assign58500_e95347: f64 = (assign58500_e95345).max(1e-38);
        let assign58500_e95348: f64 = (assign58500_e95347).ln();
        let assign58500_e95349: f64 = (assign58500_e95338 * assign58500_e95348);
        (assign58500_e95349,)
    } else {
        (locals.var_cf_i,)
    }
};
        locals.var_cf_i = assign58500_e95351;
        locals.var_cf_i_rv = 0.0;

        let (assign58510_e95358,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58510_e95356: f64 = (p.p872 + locals.var_cf_i);
        (assign58510_e95356,)
    } else {
        (locals.var_cgsof,)
    }
};
        locals.var_cgsof = assign58510_e95358;
        locals.var_cgsof_rv = 0.0;

        let (assign58520_e95365,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58520_e95363: f64 = (p.p873 + locals.var_cf_i);
        (assign58520_e95363,)
    } else {
        (locals.var_cgdof,)
    }
};
        locals.var_cgdof = assign58520_e95365;
        locals.var_cgdof_rv = 0.0;

        let assign58530_e95368: f64 = if p.p32 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard862 = assign58530_e95368;
        locals.var_guard862_rv = 0.0;

        let (assign58540_e95382, assign58540_e95382_d_n3, assign58540_e95382_d_n4, assign58540_e95382_d_n5, assign58540_e95382_d_n6, assign58540_e95382_d_n7, assign58540_e95382_d_n8, assign58540_e95382_d_n9, assign58540_e95382_d_n10, assign58540_e95382_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 != 0.0)) {
        let assign58540_e95374: f64 = (-locals.var_wact);
        let assign58540_e95376: f64 = (assign58540_e95374 * p.p2);
        let assign58540_e95378: f64 = (assign58540_e95376 * locals.var_cgsof);
        let assign58540_e95380: f64 = (assign58540_e95378 * locals.var_vgs_ov_noswap);
        (assign58540_e95380, 0.0, 0.0, 0.0, 0.0, (assign58540_e95378 * locals.var_vgs_ov_noswap_dn7), 0.0, (assign58540_e95378 * locals.var_vgs_ov_noswap_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qovs, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11,)
    }
};
        locals.var_qovs = assign58540_e95382;
        locals.var_qovs_dn3 = assign58540_e95382_d_n3;
        locals.var_qovs_dn4 = assign58540_e95382_d_n4;
        locals.var_qovs_dn5 = assign58540_e95382_d_n5;
        locals.var_qovs_dn6 = assign58540_e95382_d_n6;
        locals.var_qovs_dn7 = assign58540_e95382_d_n7;
        locals.var_qovs_dn8 = assign58540_e95382_d_n8;
        locals.var_qovs_dn9 = assign58540_e95382_d_n9;
        locals.var_qovs_dn10 = assign58540_e95382_d_n10;
        locals.var_qovs_dn11 = assign58540_e95382_d_n11;
        locals.var_qovs_rv = 0.0;

        let (assign58550_e95396, assign58550_e95396_d_n3, assign58550_e95396_d_n4, assign58550_e95396_d_n5, assign58550_e95396_d_n6, assign58550_e95396_d_n7, assign58550_e95396_d_n8, assign58550_e95396_d_n9, assign58550_e95396_d_n10, assign58550_e95396_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 != 0.0)) {
        let assign58550_e95388: f64 = (-locals.var_wact);
        let assign58550_e95390: f64 = (assign58550_e95388 * p.p2);
        let assign58550_e95392: f64 = (assign58550_e95390 * locals.var_cgdof);
        let assign58550_e95394: f64 = (assign58550_e95392 * locals.var_vgd_ov_noswap);
        (assign58550_e95394, 0.0, 0.0, 0.0, (assign58550_e95392 * locals.var_vgd_ov_noswap_dn6), 0.0, 0.0, (assign58550_e95392 * locals.var_vgd_ov_noswap_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qovd, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11,)
    }
};
        locals.var_qovd = assign58550_e95396;
        locals.var_qovd_dn3 = assign58550_e95396_d_n3;
        locals.var_qovd_dn4 = assign58550_e95396_d_n4;
        locals.var_qovd_dn5 = assign58550_e95396_d_n5;
        locals.var_qovd_dn6 = assign58550_e95396_d_n6;
        locals.var_qovd_dn7 = assign58550_e95396_d_n7;
        locals.var_qovd_dn8 = assign58550_e95396_d_n8;
        locals.var_qovd_dn9 = assign58550_e95396_d_n9;
        locals.var_qovd_dn10 = assign58550_e95396_d_n10;
        locals.var_qovd_dn11 = assign58550_e95396_d_n11;
        locals.var_qovd_rv = 0.0;

        let (assign58560_e95419, assign58560_e95419_d_n3, assign58560_e95419_d_n4, assign58560_e95419_d_n5, assign58560_e95419_d_n6, assign58560_e95419_d_n7, assign58560_e95419_d_n8, assign58560_e95419_d_n9, assign58560_e95419_d_n10, assign58560_e95419_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58560_e95404: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58560_e95406: f64 = (assign58560_e95404 + 0.02);
        let assign58560_e95409: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58560_e95411: f64 = (assign58560_e95409 + 0.02);
        let assign58560_e95412: f64 = (assign58560_e95406 * assign58560_e95411);
        let assign58560_e95415: f64 = (4.0 * 0.02);
        let assign58560_e95416: f64 = (assign58560_e95412 + assign58560_e95415);
        let assign58560_e95417: f64 = (assign58560_e95416).sqrt();
        (assign58560_e95417, 0.0, ((((-locals.var_vfbsdr_dn4) * assign58560_e95411) + (assign58560_e95406 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign58560_e95417)), ((((-locals.var_vfbsdr_dn5) * assign58560_e95411) + (assign58560_e95406 * (-locals.var_vfbsdr_dn5))) / (2.0 * assign58560_e95417)), 0.0, (((locals.var_vgs_ov_noswap_dn7 * assign58560_e95411) + (assign58560_e95406 * locals.var_vgs_ov_noswap_dn7)) / (2.0 * assign58560_e95417)), 0.0, (((locals.var_vgs_ov_noswap_dn9 * assign58560_e95411) + (assign58560_e95406 * locals.var_vgs_ov_noswap_dn9)) / (2.0 * assign58560_e95417)), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58560_e95419;
        locals.var_t0_dn3 = assign58560_e95419_d_n3;
        locals.var_t0_dn4 = assign58560_e95419_d_n4;
        locals.var_t0_dn5 = assign58560_e95419_d_n5;
        locals.var_t0_dn6 = assign58560_e95419_d_n6;
        locals.var_t0_dn7 = assign58560_e95419_d_n7;
        locals.var_t0_dn8 = assign58560_e95419_d_n8;
        locals.var_t0_dn9 = assign58560_e95419_d_n9;
        locals.var_t0_dn10 = assign58560_e95419_d_n10;
        locals.var_t0_dn11 = assign58560_e95419_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign58570_e95435, assign58570_e95435_d_n3, assign58570_e95435_d_n4, assign58570_e95435_d_n5, assign58570_e95435_d_n6, assign58570_e95435_d_n7, assign58570_e95435_d_n8, assign58570_e95435_d_n9, assign58570_e95435_d_n10, assign58570_e95435_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58570_e95428: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58570_e95430: f64 = (assign58570_e95428 + 0.02);
        let assign58570_e95432: f64 = (assign58570_e95430 - locals.var_t0);
        let assign58570_e95433: f64 = (0.5 * assign58570_e95432);
        (assign58570_e95433, (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * ((-locals.var_vfbsdr_dn5) - locals.var_t0_dn5)), (0.5 * (-locals.var_t0_dn6)), (0.5 * (locals.var_vgs_ov_noswap_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (locals.var_vgs_ov_noswap_dn9 - locals.var_t0_dn9)), (0.5 * (-locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)),)
    } else {
        (locals.var_vgsov, locals.var_vgsov_dn3, locals.var_vgsov_dn4, locals.var_vgsov_dn5, locals.var_vgsov_dn6, locals.var_vgsov_dn7, locals.var_vgsov_dn8, locals.var_vgsov_dn9, locals.var_vgsov_dn10, locals.var_vgsov_dn11,)
    }
};
        locals.var_vgsov = assign58570_e95435;
        locals.var_vgsov_dn3 = assign58570_e95435_d_n3;
        locals.var_vgsov_dn4 = assign58570_e95435_d_n4;
        locals.var_vgsov_dn5 = assign58570_e95435_d_n5;
        locals.var_vgsov_dn6 = assign58570_e95435_d_n6;
        locals.var_vgsov_dn7 = assign58570_e95435_d_n7;
        locals.var_vgsov_dn8 = assign58570_e95435_d_n8;
        locals.var_vgsov_dn9 = assign58570_e95435_d_n9;
        locals.var_vgsov_dn10 = assign58570_e95435_d_n10;
        locals.var_vgsov_dn11 = assign58570_e95435_d_n11;
        locals.var_vgsov_rv = 0.0;

        let (assign58580_e95456, assign58580_e95456_d_n3, assign58580_e95456_d_n4, assign58580_e95456_d_n5, assign58580_e95456_d_n6, assign58580_e95456_d_n7, assign58580_e95456_d_n8, assign58580_e95456_d_n9, assign58580_e95456_d_n10, assign58580_e95456_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58580_e95444: f64 = (-locals.var_vgsov);
        let assign58580_e95446: f64 = (assign58580_e95444 / p.p893);
        let assign58580_e95448: f64 = (assign58580_e95446).powf(p.p894);
        let assign58580_e95449: f64 = (1.0 + assign58580_e95448);
        let assign58580_e95452: f64 = (1.0 / p.p894);
        let assign58580_e95453: f64 = (assign58580_e95449).powf(assign58580_e95452);
        let assign58580_e95454: f64 = (locals.var_vgsov / assign58580_e95453);
        (assign58580_e95454, (((locals.var_vgsov_dn3 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn3) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn3) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn3) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn3) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn4 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn4) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn4) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn4) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn4) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn5 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn5) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn5) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn5) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn5) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn6 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn6) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn6) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn6) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn6) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn7 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn7) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn7) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn7) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn7) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn8 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn8) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn8) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn8) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn8) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn9 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn9) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn9) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn9) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn9) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn10 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn10) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn10) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn10) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn10) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)), (((locals.var_vgsov_dn11 * assign58580_e95453) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign58580_e95452) as f64).is_finite() && ((assign58580_e95452) as f64).fract() == 0.0 { if assign58580_e95452 == 0.0 { 0.0 } else { (assign58580_e95452 * ((assign58580_e95449).powf(assign58580_e95452 - 1.0) * if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn11) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn11) / p.p893) / assign58580_e95446))) })) } } else { (assign58580_e95453 * (assign58580_e95452 * (if 0.0 == 0.0 && ((p.p894) as f64).is_finite() && ((p.p894) as f64).fract() == 0.0 { if p.p894 == 0.0 { 0.0 } else { (p.p894 * ((assign58580_e95446).powf(p.p894 - 1.0) * ((-locals.var_vgsov_dn11) / p.p893))) } } else { (assign58580_e95448 * (p.p894 * (((-locals.var_vgsov_dn11) / p.p893) / assign58580_e95446))) } / assign58580_e95449))) })) / (assign58580_e95453 * assign58580_e95453)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign58580_e95456;
        locals.var_t6_dn3 = assign58580_e95456_d_n3;
        locals.var_t6_dn4 = assign58580_e95456_d_n4;
        locals.var_t6_dn5 = assign58580_e95456_d_n5;
        locals.var_t6_dn6 = assign58580_e95456_d_n6;
        locals.var_t6_dn7 = assign58580_e95456_d_n7;
        locals.var_t6_dn8 = assign58580_e95456_d_n8;
        locals.var_t6_dn9 = assign58580_e95456_d_n9;
        locals.var_t6_dn10 = assign58580_e95456_d_n10;
        locals.var_t6_dn11 = assign58580_e95456_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign58590_e95471, assign58590_e95471_d_n3, assign58590_e95471_d_n4, assign58590_e95471_d_n5, assign58590_e95471_d_n6, assign58590_e95471_d_n7, assign58590_e95471_d_n8, assign58590_e95471_d_n9, assign58590_e95471_d_n10, assign58590_e95471_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58590_e95465: f64 = (4.0 * locals.var_t6);
        let assign58590_e95467: f64 = (assign58590_e95465 / locals.var_ckappas_i);
        let assign58590_e95468: f64 = (1.0 - assign58590_e95467);
        let assign58590_e95469: f64 = (assign58590_e95468).sqrt();
        (assign58590_e95469, ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappas_i)) / (2.0 * assign58590_e95469)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58590_e95471;
        locals.var_t1_dn3 = assign58590_e95471_d_n3;
        locals.var_t1_dn4 = assign58590_e95471_d_n4;
        locals.var_t1_dn5 = assign58590_e95471_d_n5;
        locals.var_t1_dn6 = assign58590_e95471_d_n6;
        locals.var_t1_dn7 = assign58590_e95471_d_n7;
        locals.var_t1_dn8 = assign58590_e95471_d_n8;
        locals.var_t1_dn9 = assign58590_e95471_d_n9;
        locals.var_t1_dn10 = assign58590_e95471_d_n10;
        locals.var_t1_dn11 = assign58590_e95471_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign58600_e95503, assign58600_e95503_d_n3, assign58600_e95503_d_n4, assign58600_e95503_d_n5, assign58600_e95503_d_n6, assign58600_e95503_d_n7, assign58600_e95503_d_n8, assign58600_e95503_d_n9, assign58600_e95503_d_n10, assign58600_e95503_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58600_e95478: f64 = (-locals.var_wact);
        let assign58600_e95480: f64 = (assign58600_e95478 * p.p2);
        let assign58600_e95483: f64 = (locals.var_cgsof * locals.var_vgs_ov_noswap);
        let assign58600_e95487: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign58600_e95489: f64 = (assign58600_e95487 - locals.var_vgsov);
        let assign58600_e95492: f64 = (0.5 * locals.var_ckappas_i);
        let assign58600_e95494: f64 = (-1.0);
        let assign58600_e95496: f64 = (assign58600_e95494 + locals.var_t1);
        let assign58600_e95497: f64 = (assign58600_e95492 * assign58600_e95496);
        let assign58600_e95498: f64 = (assign58600_e95489 - assign58600_e95497);
        let assign58600_e95499: f64 = (locals.var_cgsl_i * assign58600_e95498);
        let assign58600_e95500: f64 = (assign58600_e95483 + assign58600_e95499);
        let assign58600_e95501: f64 = (assign58600_e95480 * assign58600_e95500);
        (assign58600_e95501, (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn3) - (assign58600_e95492 * locals.var_t1_dn3)))), (assign58600_e95480 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgsov_dn4) - (assign58600_e95492 * locals.var_t1_dn4)))), (assign58600_e95480 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn5) - locals.var_vgsov_dn5) - (assign58600_e95492 * locals.var_t1_dn5)))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn6) - (assign58600_e95492 * locals.var_t1_dn6)))), (assign58600_e95480 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn7) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn7 - locals.var_vgsov_dn7) - (assign58600_e95492 * locals.var_t1_dn7))))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn8) - (assign58600_e95492 * locals.var_t1_dn8)))), (assign58600_e95480 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn9) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn9 - locals.var_vgsov_dn9) - (assign58600_e95492 * locals.var_t1_dn9))))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn10) - (assign58600_e95492 * locals.var_t1_dn10)))), (assign58600_e95480 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn11) - (assign58600_e95492 * locals.var_t1_dn11)))),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11,)
    }
};
        locals.var_qovs = assign58600_e95503;
        locals.var_qovs_dn3 = assign58600_e95503_d_n3;
        locals.var_qovs_dn4 = assign58600_e95503_d_n4;
        locals.var_qovs_dn5 = assign58600_e95503_d_n5;
        locals.var_qovs_dn6 = assign58600_e95503_d_n6;
        locals.var_qovs_dn7 = assign58600_e95503_d_n7;
        locals.var_qovs_dn8 = assign58600_e95503_d_n8;
        locals.var_qovs_dn9 = assign58600_e95503_d_n9;
        locals.var_qovs_dn10 = assign58600_e95503_d_n10;
        locals.var_qovs_dn11 = assign58600_e95503_d_n11;
        locals.var_qovs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_204(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (assign58610_e95526, assign58610_e95526_d_n3, assign58610_e95526_d_n4, assign58610_e95526_d_n5, assign58610_e95526_d_n6, assign58610_e95526_d_n7, assign58610_e95526_d_n8, assign58610_e95526_d_n9, assign58610_e95526_d_n10, assign58610_e95526_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58610_e95511: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58610_e95513: f64 = (assign58610_e95511 + 0.02);
        let assign58610_e95516: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58610_e95518: f64 = (assign58610_e95516 + 0.02);
        let assign58610_e95519: f64 = (assign58610_e95513 * assign58610_e95518);
        let assign58610_e95522: f64 = (4.0 * 0.02);
        let assign58610_e95523: f64 = (assign58610_e95519 + assign58610_e95522);
        let assign58610_e95524: f64 = (assign58610_e95523).sqrt();
        (assign58610_e95524, 0.0, ((((-locals.var_vfbsdr_dn4) * assign58610_e95518) + (assign58610_e95513 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign58610_e95524)), ((((-locals.var_vfbsdr_dn5) * assign58610_e95518) + (assign58610_e95513 * (-locals.var_vfbsdr_dn5))) / (2.0 * assign58610_e95524)), (((locals.var_vgd_ov_noswap_dn6 * assign58610_e95518) + (assign58610_e95513 * locals.var_vgd_ov_noswap_dn6)) / (2.0 * assign58610_e95524)), 0.0, 0.0, (((locals.var_vgd_ov_noswap_dn9 * assign58610_e95518) + (assign58610_e95513 * locals.var_vgd_ov_noswap_dn9)) / (2.0 * assign58610_e95524)), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58610_e95526;
        locals.var_t0_dn3 = assign58610_e95526_d_n3;
        locals.var_t0_dn4 = assign58610_e95526_d_n4;
        locals.var_t0_dn5 = assign58610_e95526_d_n5;
        locals.var_t0_dn6 = assign58610_e95526_d_n6;
        locals.var_t0_dn7 = assign58610_e95526_d_n7;
        locals.var_t0_dn8 = assign58610_e95526_d_n8;
        locals.var_t0_dn9 = assign58610_e95526_d_n9;
        locals.var_t0_dn10 = assign58610_e95526_d_n10;
        locals.var_t0_dn11 = assign58610_e95526_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign58620_e95542, assign58620_e95542_d_n3, assign58620_e95542_d_n4, assign58620_e95542_d_n5, assign58620_e95542_d_n6, assign58620_e95542_d_n7, assign58620_e95542_d_n8, assign58620_e95542_d_n9, assign58620_e95542_d_n10, assign58620_e95542_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58620_e95535: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58620_e95537: f64 = (assign58620_e95535 + 0.02);
        let assign58620_e95539: f64 = (assign58620_e95537 - locals.var_t0);
        let assign58620_e95540: f64 = (0.5 * assign58620_e95539);
        (assign58620_e95540, (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * ((-locals.var_vfbsdr_dn5) - locals.var_t0_dn5)), (0.5 * (locals.var_vgd_ov_noswap_dn6 - locals.var_t0_dn6)), (0.5 * (-locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (locals.var_vgd_ov_noswap_dn9 - locals.var_t0_dn9)), (0.5 * (-locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)),)
    } else {
        (locals.var_vgdov, locals.var_vgdov_dn3, locals.var_vgdov_dn4, locals.var_vgdov_dn5, locals.var_vgdov_dn6, locals.var_vgdov_dn7, locals.var_vgdov_dn8, locals.var_vgdov_dn9, locals.var_vgdov_dn10, locals.var_vgdov_dn11,)
    }
};
        locals.var_vgdov = assign58620_e95542;
        locals.var_vgdov_dn3 = assign58620_e95542_d_n3;
        locals.var_vgdov_dn4 = assign58620_e95542_d_n4;
        locals.var_vgdov_dn5 = assign58620_e95542_d_n5;
        locals.var_vgdov_dn6 = assign58620_e95542_d_n6;
        locals.var_vgdov_dn7 = assign58620_e95542_d_n7;
        locals.var_vgdov_dn8 = assign58620_e95542_d_n8;
        locals.var_vgdov_dn9 = assign58620_e95542_d_n9;
        locals.var_vgdov_dn10 = assign58620_e95542_d_n10;
        locals.var_vgdov_dn11 = assign58620_e95542_d_n11;
        locals.var_vgdov_rv = 0.0;

        let (assign58630_e95563, assign58630_e95563_d_n3, assign58630_e95563_d_n4, assign58630_e95563_d_n5, assign58630_e95563_d_n6, assign58630_e95563_d_n7, assign58630_e95563_d_n8, assign58630_e95563_d_n9, assign58630_e95563_d_n10, assign58630_e95563_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58630_e95551: f64 = (-locals.var_vgdov);
        let assign58630_e95553: f64 = (assign58630_e95551 / p.p891);
        let assign58630_e95555: f64 = (assign58630_e95553).powf(p.p892);
        let assign58630_e95556: f64 = (1.0 + assign58630_e95555);
        let assign58630_e95559: f64 = (1.0 / p.p892);
        let assign58630_e95560: f64 = (assign58630_e95556).powf(assign58630_e95559);
        let assign58630_e95561: f64 = (locals.var_vgdov / assign58630_e95560);
        (assign58630_e95561, (((locals.var_vgdov_dn3 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn3) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn3) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn3) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn3) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn4 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn4) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn4) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn4) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn4) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn5 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn5) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn5) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn5) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn5) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn6 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn6) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn6) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn6) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn6) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn7 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn7) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn7) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn7) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn7) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn8 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn8) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn8) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn8) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn8) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn9 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn9) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn9) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn9) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn9) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn10 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn10) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn10) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn10) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn10) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)), (((locals.var_vgdov_dn11 * assign58630_e95560) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign58630_e95559) as f64).is_finite() && ((assign58630_e95559) as f64).fract() == 0.0 { if assign58630_e95559 == 0.0 { 0.0 } else { (assign58630_e95559 * ((assign58630_e95556).powf(assign58630_e95559 - 1.0) * if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn11) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn11) / p.p891) / assign58630_e95553))) })) } } else { (assign58630_e95560 * (assign58630_e95559 * (if 0.0 == 0.0 && ((p.p892) as f64).is_finite() && ((p.p892) as f64).fract() == 0.0 { if p.p892 == 0.0 { 0.0 } else { (p.p892 * ((assign58630_e95553).powf(p.p892 - 1.0) * ((-locals.var_vgdov_dn11) / p.p891))) } } else { (assign58630_e95555 * (p.p892 * (((-locals.var_vgdov_dn11) / p.p891) / assign58630_e95553))) } / assign58630_e95556))) })) / (assign58630_e95560 * assign58630_e95560)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign58630_e95563;
        locals.var_t6_dn3 = assign58630_e95563_d_n3;
        locals.var_t6_dn4 = assign58630_e95563_d_n4;
        locals.var_t6_dn5 = assign58630_e95563_d_n5;
        locals.var_t6_dn6 = assign58630_e95563_d_n6;
        locals.var_t6_dn7 = assign58630_e95563_d_n7;
        locals.var_t6_dn8 = assign58630_e95563_d_n8;
        locals.var_t6_dn9 = assign58630_e95563_d_n9;
        locals.var_t6_dn10 = assign58630_e95563_d_n10;
        locals.var_t6_dn11 = assign58630_e95563_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign58640_e95578, assign58640_e95578_d_n3, assign58640_e95578_d_n4, assign58640_e95578_d_n5, assign58640_e95578_d_n6, assign58640_e95578_d_n7, assign58640_e95578_d_n8, assign58640_e95578_d_n9, assign58640_e95578_d_n10, assign58640_e95578_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58640_e95572: f64 = (4.0 * locals.var_t6);
        let assign58640_e95574: f64 = (assign58640_e95572 / locals.var_ckappad_i);
        let assign58640_e95575: f64 = (1.0 - assign58640_e95574);
        let assign58640_e95576: f64 = (assign58640_e95575).sqrt();
        (assign58640_e95576, ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappad_i)) / (2.0 * assign58640_e95576)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign58640_e95578;
        locals.var_t2_dn3 = assign58640_e95578_d_n3;
        locals.var_t2_dn4 = assign58640_e95578_d_n4;
        locals.var_t2_dn5 = assign58640_e95578_d_n5;
        locals.var_t2_dn6 = assign58640_e95578_d_n6;
        locals.var_t2_dn7 = assign58640_e95578_d_n7;
        locals.var_t2_dn8 = assign58640_e95578_d_n8;
        locals.var_t2_dn9 = assign58640_e95578_d_n9;
        locals.var_t2_dn10 = assign58640_e95578_d_n10;
        locals.var_t2_dn11 = assign58640_e95578_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign58650_e95610, assign58650_e95610_d_n3, assign58650_e95610_d_n4, assign58650_e95610_d_n5, assign58650_e95610_d_n6, assign58650_e95610_d_n7, assign58650_e95610_d_n8, assign58650_e95610_d_n9, assign58650_e95610_d_n10, assign58650_e95610_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard862 == 0.0)) {
        let assign58650_e95585: f64 = (-locals.var_wact);
        let assign58650_e95587: f64 = (assign58650_e95585 * p.p2);
        let assign58650_e95590: f64 = (locals.var_cgdof * locals.var_vgd_ov_noswap);
        let assign58650_e95594: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsdr);
        let assign58650_e95596: f64 = (assign58650_e95594 - locals.var_vgdov);
        let assign58650_e95599: f64 = (0.5 * locals.var_ckappad_i);
        let assign58650_e95601: f64 = (-1.0);
        let assign58650_e95603: f64 = (assign58650_e95601 + locals.var_t2);
        let assign58650_e95604: f64 = (assign58650_e95599 * assign58650_e95603);
        let assign58650_e95605: f64 = (assign58650_e95596 - assign58650_e95604);
        let assign58650_e95606: f64 = (locals.var_cgdl_i * assign58650_e95605);
        let assign58650_e95607: f64 = (assign58650_e95590 + assign58650_e95606);
        let assign58650_e95608: f64 = (assign58650_e95587 * assign58650_e95607);
        (assign58650_e95608, (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn3) - (assign58650_e95599 * locals.var_t2_dn3)))), (assign58650_e95587 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgdov_dn4) - (assign58650_e95599 * locals.var_t2_dn4)))), (assign58650_e95587 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn5) - locals.var_vgdov_dn5) - (assign58650_e95599 * locals.var_t2_dn5)))), (assign58650_e95587 * ((locals.var_cgdof * locals.var_vgd_ov_noswap_dn6) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswap_dn6 - locals.var_vgdov_dn6) - (assign58650_e95599 * locals.var_t2_dn6))))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn7) - (assign58650_e95599 * locals.var_t2_dn7)))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn8) - (assign58650_e95599 * locals.var_t2_dn8)))), (assign58650_e95587 * ((locals.var_cgdof * locals.var_vgd_ov_noswap_dn9) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswap_dn9 - locals.var_vgdov_dn9) - (assign58650_e95599 * locals.var_t2_dn9))))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn10) - (assign58650_e95599 * locals.var_t2_dn10)))), (assign58650_e95587 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn11) - (assign58650_e95599 * locals.var_t2_dn11)))),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11,)
    }
};
        locals.var_qovd = assign58650_e95610;
        locals.var_qovd_dn3 = assign58650_e95610_d_n3;
        locals.var_qovd_dn4 = assign58650_e95610_d_n4;
        locals.var_qovd_dn5 = assign58650_e95610_d_n5;
        locals.var_qovd_dn6 = assign58650_e95610_d_n6;
        locals.var_qovd_dn7 = assign58650_e95610_d_n7;
        locals.var_qovd_dn8 = assign58650_e95610_d_n8;
        locals.var_qovd_dn9 = assign58650_e95610_d_n9;
        locals.var_qovd_dn10 = assign58650_e95610_d_n10;
        locals.var_qovd_dn11 = assign58650_e95610_d_n11;
        locals.var_qovd_rv = 0.0;

        let (assign58660_e95624, assign58660_e95624_d_n9, assign58660_e95624_d_n10,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58660_e95614: f64 = (-locals.var_devsign);
        let assign58660_e95616: f64 = (assign58660_e95614 * p.p2);
        let assign58660_e95618: f64 = (assign58660_e95616 * locals.var_lact);
        let assign58660_e95620: f64 = (assign58660_e95618 * p.p874);
        let assign58660_e95622: f64 = (assign58660_e95620 * (nv9 - nv10));
        (assign58660_e95622, assign58660_e95620, (-assign58660_e95620),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn9, locals.var_qovb_dn10,)
    }
};
        locals.var_qovb = assign58660_e95624;
        locals.var_qovb_dn9 = assign58660_e95624_d_n9;
        locals.var_qovb_dn10 = assign58660_e95624_d_n10;
        locals.var_qovb_rv = 0.0;

        let (assign58670_e95634, assign58670_e95634_d_n3, assign58670_e95634_d_n4, assign58670_e95634_d_n5, assign58670_e95634_d_n6, assign58670_e95634_d_n7, assign58670_e95634_d_n8, assign58670_e95634_d_n9, assign58670_e95634_d_n10, assign58670_e95634_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58670_e95629: f64 = (locals.var_qovs + locals.var_qovd);
        let assign58670_e95631: f64 = (assign58670_e95629 + locals.var_qovb);
        let assign58670_e95632: f64 = (-assign58670_e95631);
        (assign58670_e95632, (-(locals.var_qovs_dn3 + locals.var_qovd_dn3)), (-(locals.var_qovs_dn4 + locals.var_qovd_dn4)), (-(locals.var_qovs_dn5 + locals.var_qovd_dn5)), (-(locals.var_qovs_dn6 + locals.var_qovd_dn6)), (-(locals.var_qovs_dn7 + locals.var_qovd_dn7)), (-(locals.var_qovs_dn8 + locals.var_qovd_dn8)), (-((locals.var_qovs_dn9 + locals.var_qovd_dn9) + locals.var_qovb_dn9)), (-((locals.var_qovs_dn10 + locals.var_qovd_dn10) + locals.var_qovb_dn10)), (-(locals.var_qovs_dn11 + locals.var_qovd_dn11)),)
    } else {
        (locals.var_qovg, locals.var_qovg_dn3, locals.var_qovg_dn4, locals.var_qovg_dn5, locals.var_qovg_dn6, locals.var_qovg_dn7, locals.var_qovg_dn8, locals.var_qovg_dn9, locals.var_qovg_dn10, locals.var_qovg_dn11,)
    }
};
        locals.var_qovg = assign58670_e95634;
        locals.var_qovg_dn3 = assign58670_e95634_d_n3;
        locals.var_qovg_dn4 = assign58670_e95634_d_n4;
        locals.var_qovg_dn5 = assign58670_e95634_d_n5;
        locals.var_qovg_dn6 = assign58670_e95634_d_n6;
        locals.var_qovg_dn7 = assign58670_e95634_d_n7;
        locals.var_qovg_dn8 = assign58670_e95634_d_n8;
        locals.var_qovg_dn9 = assign58670_e95634_d_n9;
        locals.var_qovg_dn10 = assign58670_e95634_d_n10;
        locals.var_qovg_dn11 = assign58670_e95634_d_n11;
        locals.var_qovg_rv = 0.0;

        let (assign58680_e95645,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58680_e95640: f64 = (2.0 * locals.var_dlcv);
        let assign58680_e95641: f64 = (locals.var_lnew - assign58680_e95640);
        let assign58680_e95643: f64 = (assign58680_e95641 - p.p1394);
        (assign58680_e95643,)
    } else {
        (locals.var_leffcvb,)
    }
};
        locals.var_leffcvb = assign58680_e95645;
        locals.var_leffcvb_rv = 0.0;

        let (assign58690_e95654,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58690_e95651: f64 = (2.0 * p.p1393);
        let assign58690_e95652: f64 = (locals.var_leffcvb + assign58690_e95651);
        (assign58690_e95652,)
    } else {
        (locals.var_leffcvbg,)
    }
};
        locals.var_leffcvbg = assign58690_e95654;
        locals.var_leffcvbg_rv = 0.0;

        let assign58700_e95657: f64 = if locals.var_nsub_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard863 = assign58700_e95657;
        locals.var_guard863_rv = 0.0;

        let (assign58710_e95669, assign58710_e95669_d_n3, assign58710_e95669_d_n4, assign58710_e95669_d_n5, assign58710_e95669_d_n6, assign58710_e95669_d_n7, assign58710_e95669_d_n8, assign58710_e95669_d_n9, assign58710_e95669_d_n10, assign58710_e95669_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 != 0.0)) {
        let assign58710_e95664: f64 = (locals.var_ndep_i / locals.var_nsub_i);
        let assign58710_e95666: f64 = (assign58710_e95664).max(1e-38);
        let assign58710_e95667: f64 = (assign58710_e95666).ln();
        (assign58710_e95667, (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn3 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn4 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn5 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn6 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn7 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn8 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn9 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn10 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666), (if assign58710_e95664 >= 1e-38 { (locals.var_ndep_i_dn11 / locals.var_nsub_i) } else { 0.0 } / assign58710_e95666),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58710_e95669;
        locals.var_t0_dn3 = assign58710_e95669_d_n3;
        locals.var_t0_dn4 = assign58710_e95669_d_n4;
        locals.var_t0_dn5 = assign58710_e95669_d_n5;
        locals.var_t0_dn6 = assign58710_e95669_d_n6;
        locals.var_t0_dn7 = assign58710_e95669_d_n7;
        locals.var_t0_dn8 = assign58710_e95669_d_n8;
        locals.var_t0_dn9 = assign58710_e95669_d_n9;
        locals.var_t0_dn10 = assign58710_e95669_d_n10;
        locals.var_t0_dn11 = assign58710_e95669_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign58720_e95681, assign58720_e95681_d_n3, assign58720_e95681_d_n4, assign58720_e95681_d_n5, assign58720_e95681_d_n6, assign58720_e95681_d_n7, assign58720_e95681_d_n8, assign58720_e95681_d_n9, assign58720_e95681_d_n10, assign58720_e95681_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 != 0.0)) {
        let assign58720_e95675: f64 = (-locals.var_devsign);
        let assign58720_e95677: f64 = (assign58720_e95675 * locals.var_vtm);
        let assign58720_e95679: f64 = (assign58720_e95677 * locals.var_t0);
        (assign58720_e95679, (assign58720_e95677 * locals.var_t0_dn3), (((assign58720_e95675 * locals.var_vtm_dn4) * locals.var_t0) + (assign58720_e95677 * locals.var_t0_dn4)), (((assign58720_e95675 * locals.var_vtm_dn5) * locals.var_t0) + (assign58720_e95677 * locals.var_t0_dn5)), (assign58720_e95677 * locals.var_t0_dn6), (assign58720_e95677 * locals.var_t0_dn7), (assign58720_e95677 * locals.var_t0_dn8), (assign58720_e95677 * locals.var_t0_dn9), (assign58720_e95677 * locals.var_t0_dn10), (assign58720_e95677 * locals.var_t0_dn11),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign58720_e95681;
        locals.var_vfbb_dn3 = assign58720_e95681_d_n3;
        locals.var_vfbb_dn4 = assign58720_e95681_d_n4;
        locals.var_vfbb_dn5 = assign58720_e95681_d_n5;
        locals.var_vfbb_dn6 = assign58720_e95681_d_n6;
        locals.var_vfbb_dn7 = assign58720_e95681_d_n7;
        locals.var_vfbb_dn8 = assign58720_e95681_d_n8;
        locals.var_vfbb_dn9 = assign58720_e95681_d_n9;
        locals.var_vfbb_dn10 = assign58720_e95681_d_n10;
        locals.var_vfbb_dn11 = assign58720_e95681_d_n11;
        locals.var_vfbb_rv = 0.0;

        let (assign58730_e95699, assign58730_e95699_d_n3, assign58730_e95699_d_n4, assign58730_e95699_d_n5, assign58730_e95699_d_n6, assign58730_e95699_d_n7, assign58730_e95699_d_n8, assign58730_e95699_d_n9, assign58730_e95699_d_n10, assign58730_e95699_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 == 0.0)) {
        let assign58730_e95688: f64 = (-locals.var_ndep_i);
        let assign58730_e95690: f64 = (assign58730_e95688 * locals.var_nsub_i);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_ni;
        let assign58730_e95692: f64 = (assign58730_e95690 * __rspice_inv_cse_0);
        let assign58730_e95694: f64 = (assign58730_e95692 * __rspice_inv_cse_0);
        let assign58730_e95696: f64 = (assign58730_e95694).max(1e-38);
        let assign58730_e95697: f64 = (assign58730_e95696).ln();
        (assign58730_e95697, (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn3) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn4) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn5) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn6) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn7) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn8) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn9) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn10) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696), (if assign58730_e95694 >= 1e-38 { ((((((((-locals.var_ndep_i_dn11) * locals.var_nsub_i) * locals.var_ni) - (assign58730_e95690 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign58730_e95692 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign58730_e95696),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58730_e95699;
        locals.var_t0_dn3 = assign58730_e95699_d_n3;
        locals.var_t0_dn4 = assign58730_e95699_d_n4;
        locals.var_t0_dn5 = assign58730_e95699_d_n5;
        locals.var_t0_dn6 = assign58730_e95699_d_n6;
        locals.var_t0_dn7 = assign58730_e95699_d_n7;
        locals.var_t0_dn8 = assign58730_e95699_d_n8;
        locals.var_t0_dn9 = assign58730_e95699_d_n9;
        locals.var_t0_dn10 = assign58730_e95699_d_n10;
        locals.var_t0_dn11 = assign58730_e95699_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign58740_e95712, assign58740_e95712_d_n3, assign58740_e95712_d_n4, assign58740_e95712_d_n5, assign58740_e95712_d_n6, assign58740_e95712_d_n7, assign58740_e95712_d_n8, assign58740_e95712_d_n9, assign58740_e95712_d_n10, assign58740_e95712_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard863 == 0.0)) {
        let assign58740_e95706: f64 = (-locals.var_devsign);
        let assign58740_e95708: f64 = (assign58740_e95706 * locals.var_vtm);
        let assign58740_e95710: f64 = (assign58740_e95708 * locals.var_t0);
        (assign58740_e95710, (assign58740_e95708 * locals.var_t0_dn3), (((assign58740_e95706 * locals.var_vtm_dn4) * locals.var_t0) + (assign58740_e95708 * locals.var_t0_dn4)), (((assign58740_e95706 * locals.var_vtm_dn5) * locals.var_t0) + (assign58740_e95708 * locals.var_t0_dn5)), (assign58740_e95708 * locals.var_t0_dn6), (assign58740_e95708 * locals.var_t0_dn7), (assign58740_e95708 * locals.var_t0_dn8), (assign58740_e95708 * locals.var_t0_dn9), (assign58740_e95708 * locals.var_t0_dn10), (assign58740_e95708 * locals.var_t0_dn11),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11,)
    }
};
        locals.var_vfbb = assign58740_e95712;
        locals.var_vfbb_dn3 = assign58740_e95712_d_n3;
        locals.var_vfbb_dn4 = assign58740_e95712_d_n4;
        locals.var_vfbb_dn5 = assign58740_e95712_d_n5;
        locals.var_vfbb_dn6 = assign58740_e95712_d_n6;
        locals.var_vfbb_dn7 = assign58740_e95712_d_n7;
        locals.var_vfbb_dn8 = assign58740_e95712_d_n8;
        locals.var_vfbb_dn9 = assign58740_e95712_d_n9;
        locals.var_vfbb_dn10 = assign58740_e95712_d_n10;
        locals.var_vfbb_dn11 = assign58740_e95712_d_n11;
        locals.var_vfbb_rv = 0.0;

        let (assign58750_e95719, assign58750_e95719_d_n3, assign58750_e95719_d_n4, assign58750_e95719_d_n5, assign58750_e95719_d_n6, assign58750_e95719_d_n7, assign58750_e95719_d_n8, assign58750_e95719_d_n9, assign58750_e95719_d_n10, assign58750_e95719_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58750_e95717: f64 = (locals.var_ves - locals.var_vfbb);
        (assign58750_e95717, (locals.var_ves_dn3 - locals.var_vfbb_dn3), (-locals.var_vfbb_dn4), (-locals.var_vfbb_dn5), (locals.var_ves_dn6 - locals.var_vfbb_dn6), (locals.var_ves_dn7 - locals.var_vfbb_dn7), (-locals.var_vfbb_dn8), (-locals.var_vfbb_dn9), (locals.var_ves_dn10 - locals.var_vfbb_dn10), (-locals.var_vfbb_dn11),)
    } else {
        (locals.var_vesfb, locals.var_vesfb_dn3, locals.var_vesfb_dn4, locals.var_vesfb_dn5, locals.var_vesfb_dn6, locals.var_vesfb_dn7, locals.var_vesfb_dn8, locals.var_vesfb_dn9, locals.var_vesfb_dn10, locals.var_vesfb_dn11,)
    }
};
        locals.var_vesfb = assign58750_e95719;
        locals.var_vesfb_dn3 = assign58750_e95719_d_n3;
        locals.var_vesfb_dn4 = assign58750_e95719_d_n4;
        locals.var_vesfb_dn5 = assign58750_e95719_d_n5;
        locals.var_vesfb_dn6 = assign58750_e95719_d_n6;
        locals.var_vesfb_dn7 = assign58750_e95719_d_n7;
        locals.var_vesfb_dn8 = assign58750_e95719_d_n8;
        locals.var_vesfb_dn9 = assign58750_e95719_d_n9;
        locals.var_vesfb_dn10 = assign58750_e95719_d_n10;
        locals.var_vesfb_dn11 = assign58750_e95719_d_n11;
        locals.var_vesfb_rv = 0.0;

        let (assign58760_e95726,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58760_e95724: f64 = (3.453133e-11 / p.p75);
        (assign58760_e95724,)
    } else {
        (locals.var_cbox_1,)
    }
};
        locals.var_cbox_1 = assign58760_e95726;
        locals.var_cbox_1_rv = 0.0;

        let (assign58770_e95745,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58770_e95731: f64 = (locals.var_kb1_i * p.p1388);
        let assign58770_e95733: f64 = (assign58770_e95731 * locals.var_cbox_1);
        let assign58770_e95736: f64 = (locals.var_wact / p.p1373);
        let assign58770_e95738: f64 = (assign58770_e95736 * p.p2);
        let assign58770_e95740: f64 = (assign58770_e95738 * locals.var_leffcvbg);
        let assign58770_e95742: f64 = (assign58770_e95740 + p.p1382);
        let assign58770_e95743: f64 = (assign58770_e95733 * assign58770_e95742);
        (assign58770_e95743,)
    } else {
        (locals.var_cboxwl,)
    }
};
        locals.var_cboxwl = assign58770_e95745;
        locals.var_cboxwl_rv = 0.0;

        let (assign58780_e95754, assign58780_e95754_d_n3, assign58780_e95754_d_n4, assign58780_e95754_d_n5, assign58780_e95754_d_n6, assign58780_e95754_d_n7, assign58780_e95754_d_n8, assign58780_e95754_d_n9, assign58780_e95754_d_n10, assign58780_e95754_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58780_e95751: f64 = (locals.var_vesfb - locals.var_vbs);
        let assign58780_e95752: f64 = (locals.var_cboxwl * assign58780_e95751);
        (assign58780_e95752, (locals.var_cboxwl * locals.var_vesfb_dn3), (locals.var_cboxwl * locals.var_vesfb_dn4), (locals.var_cboxwl * locals.var_vesfb_dn5), (locals.var_cboxwl * (locals.var_vesfb_dn6 - locals.var_vbs_dn6)), (locals.var_cboxwl * (locals.var_vesfb_dn7 - locals.var_vbs_dn7)), (locals.var_cboxwl * locals.var_vesfb_dn8), (locals.var_cboxwl * locals.var_vesfb_dn9), (locals.var_cboxwl * (locals.var_vesfb_dn10 - locals.var_vbs_dn10)), (locals.var_cboxwl * locals.var_vesfb_dn11),)
    } else {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11,)
    }
};
        locals.var_qe1 = assign58780_e95754;
        locals.var_qe1_dn3 = assign58780_e95754_d_n3;
        locals.var_qe1_dn4 = assign58780_e95754_d_n4;
        locals.var_qe1_dn5 = assign58780_e95754_d_n5;
        locals.var_qe1_dn6 = assign58780_e95754_d_n6;
        locals.var_qe1_dn7 = assign58780_e95754_d_n7;
        locals.var_qe1_dn8 = assign58780_e95754_d_n8;
        locals.var_qe1_dn9 = assign58780_e95754_d_n9;
        locals.var_qe1_dn10 = assign58780_e95754_d_n10;
        locals.var_qe1_dn11 = assign58780_e95754_d_n11;
        locals.var_qe1_rv = 0.0;

        let (assign58790_e95759, assign58790_e95759_d_n3, assign58790_e95759_d_n4, assign58790_e95759_d_n5, assign58790_e95759_d_n6, assign58790_e95759_d_n7, assign58790_e95759_d_n8, assign58790_e95759_d_n9, assign58790_e95759_d_n10, assign58790_e95759_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_qe1, locals.var_qe1_dn3, locals.var_qe1_dn4, locals.var_qe1_dn5, locals.var_qe1_dn6, locals.var_qe1_dn7, locals.var_qe1_dn8, locals.var_qe1_dn9, locals.var_qe1_dn10, locals.var_qe1_dn11,)
    } else {
        (locals.var_qsub, locals.var_qsub_dn3, locals.var_qsub_dn4, locals.var_qsub_dn5, locals.var_qsub_dn6, locals.var_qsub_dn7, locals.var_qsub_dn8, locals.var_qsub_dn9, locals.var_qsub_dn10, locals.var_qsub_dn11,)
    }
};
        locals.var_qsub = assign58790_e95759;
        locals.var_qsub_dn3 = assign58790_e95759_d_n3;
        locals.var_qsub_dn4 = assign58790_e95759_d_n4;
        locals.var_qsub_dn5 = assign58790_e95759_d_n5;
        locals.var_qsub_dn6 = assign58790_e95759_d_n6;
        locals.var_qsub_dn7 = assign58790_e95759_d_n7;
        locals.var_qsub_dn8 = assign58790_e95759_d_n8;
        locals.var_qsub_dn9 = assign58790_e95759_d_n9;
        locals.var_qsub_dn10 = assign58790_e95759_d_n10;
        locals.var_qsub_dn11 = assign58790_e95759_d_n11;
        locals.var_qsub_rv = 0.0;

        let (assign58800_e95775, assign58800_e95775_d_n3, assign58800_e95775_d_n4, assign58800_e95775_d_n5, assign58800_e95775_d_n6, assign58800_e95775_d_n7, assign58800_e95775_d_n8, assign58800_e95775_d_n9, assign58800_e95775_d_n10, assign58800_e95775_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58800_e95767: f64 = (p.p74 / p.p75);
        let assign58800_e95768: f64 = (1.0 + assign58800_e95767);
        let assign58800_e95769: f64 = (p.p871 * assign58800_e95768);
        let assign58800_e95771: f64 = (assign58800_e95769).max(1e-38);
        let assign58800_e95772: f64 = (assign58800_e95771).ln();
        let assign58800_e95773: f64 = (p.p1395 * assign58800_e95772);
        (assign58800_e95773, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign58800_e95775;
        locals.var_t0_dn3 = assign58800_e95775_d_n3;
        locals.var_t0_dn4 = assign58800_e95775_d_n4;
        locals.var_t0_dn5 = assign58800_e95775_d_n5;
        locals.var_t0_dn6 = assign58800_e95775_d_n6;
        locals.var_t0_dn7 = assign58800_e95775_d_n7;
        locals.var_t0_dn8 = assign58800_e95775_d_n8;
        locals.var_t0_dn9 = assign58800_e95775_d_n9;
        locals.var_t0_dn10 = assign58800_e95775_d_n10;
        locals.var_t0_dn11 = assign58800_e95775_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign58810_e95782, assign58810_e95782_d_n3, assign58810_e95782_d_n4, assign58810_e95782_d_n5, assign58810_e95782_d_n6, assign58810_e95782_d_n7, assign58810_e95782_d_n8, assign58810_e95782_d_n9, assign58810_e95782_d_n10, assign58810_e95782_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58810_e95780: f64 = (p.p19 - p.p1);
        (assign58810_e95780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58810_e95782;
        locals.var_t1_dn3 = assign58810_e95782_d_n3;
        locals.var_t1_dn4 = assign58810_e95782_d_n4;
        locals.var_t1_dn5 = assign58810_e95782_d_n5;
        locals.var_t1_dn6 = assign58810_e95782_d_n6;
        locals.var_t1_dn7 = assign58810_e95782_d_n7;
        locals.var_t1_dn8 = assign58810_e95782_d_n8;
        locals.var_t1_dn9 = assign58810_e95782_d_n9;
        locals.var_t1_dn10 = assign58810_e95782_d_n10;
        locals.var_t1_dn11 = assign58810_e95782_d_n11;
        locals.var_t1_rv = 0.0;

        let assign58820_e95785: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard864 = assign58820_e95785;
        locals.var_guard864_rv = 0.0;

        let (assign58830_e95794, assign58830_e95794_d_n3, assign58830_e95794_d_n4, assign58830_e95794_d_n5, assign58830_e95794_d_n6, assign58830_e95794_d_n7, assign58830_e95794_d_n8, assign58830_e95794_d_n9, assign58830_e95794_d_n10, assign58830_e95794_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard864 != 0.0)) {
        let assign58830_e95792: f64 = (locals.var_t0 * locals.var_t1);
        (assign58830_e95792, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)),)
    } else {
        (locals.var_csesw, locals.var_csesw_dn3, locals.var_csesw_dn4, locals.var_csesw_dn5, locals.var_csesw_dn6, locals.var_csesw_dn7, locals.var_csesw_dn8, locals.var_csesw_dn9, locals.var_csesw_dn10, locals.var_csesw_dn11,)
    }
};
        locals.var_csesw = assign58830_e95794;
        locals.var_csesw_dn3 = assign58830_e95794_d_n3;
        locals.var_csesw_dn4 = assign58830_e95794_d_n4;
        locals.var_csesw_dn5 = assign58830_e95794_d_n5;
        locals.var_csesw_dn6 = assign58830_e95794_d_n6;
        locals.var_csesw_dn7 = assign58830_e95794_d_n7;
        locals.var_csesw_dn8 = assign58830_e95794_d_n8;
        locals.var_csesw_dn9 = assign58830_e95794_d_n9;
        locals.var_csesw_dn10 = assign58830_e95794_d_n10;
        locals.var_csesw_dn11 = assign58830_e95794_d_n11;
        locals.var_csesw_rv = 0.0;

        let (assign58840_e95802, assign58840_e95802_d_n3, assign58840_e95802_d_n4, assign58840_e95802_d_n5, assign58840_e95802_d_n6, assign58840_e95802_d_n7, assign58840_e95802_d_n8, assign58840_e95802_d_n9, assign58840_e95802_d_n10, assign58840_e95802_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard864 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_csesw, locals.var_csesw_dn3, locals.var_csesw_dn4, locals.var_csesw_dn5, locals.var_csesw_dn6, locals.var_csesw_dn7, locals.var_csesw_dn8, locals.var_csesw_dn9, locals.var_csesw_dn10, locals.var_csesw_dn11,)
    }
};
        locals.var_csesw = assign58840_e95802;
        locals.var_csesw_dn3 = assign58840_e95802_d_n3;
        locals.var_csesw_dn4 = assign58840_e95802_d_n4;
        locals.var_csesw_dn5 = assign58840_e95802_d_n5;
        locals.var_csesw_dn6 = assign58840_e95802_d_n6;
        locals.var_csesw_dn7 = assign58840_e95802_d_n7;
        locals.var_csesw_dn8 = assign58840_e95802_d_n8;
        locals.var_csesw_dn9 = assign58840_e95802_d_n9;
        locals.var_csesw_dn10 = assign58840_e95802_d_n10;
        locals.var_csesw_dn11 = assign58840_e95802_d_n11;
        locals.var_csesw_rv = 0.0;

        let (assign58850_e95809, assign58850_e95809_d_n3, assign58850_e95809_d_n4, assign58850_e95809_d_n5, assign58850_e95809_d_n6, assign58850_e95809_d_n7, assign58850_e95809_d_n8, assign58850_e95809_d_n9, assign58850_e95809_d_n10, assign58850_e95809_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58850_e95807: f64 = (p.p20 - p.p1);
        (assign58850_e95807, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58850_e95809;
        locals.var_t1_dn3 = assign58850_e95809_d_n3;
        locals.var_t1_dn4 = assign58850_e95809_d_n4;
        locals.var_t1_dn5 = assign58850_e95809_d_n5;
        locals.var_t1_dn6 = assign58850_e95809_d_n6;
        locals.var_t1_dn7 = assign58850_e95809_d_n7;
        locals.var_t1_dn8 = assign58850_e95809_d_n8;
        locals.var_t1_dn9 = assign58850_e95809_d_n9;
        locals.var_t1_dn10 = assign58850_e95809_d_n10;
        locals.var_t1_dn11 = assign58850_e95809_d_n11;
        locals.var_t1_rv = 0.0;

        let assign58860_e95812: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard865 = assign58860_e95812;
        locals.var_guard865_rv = 0.0;

        let (assign58870_e95821, assign58870_e95821_d_n3, assign58870_e95821_d_n4, assign58870_e95821_d_n5, assign58870_e95821_d_n6, assign58870_e95821_d_n7, assign58870_e95821_d_n8, assign58870_e95821_d_n9, assign58870_e95821_d_n10, assign58870_e95821_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard865 != 0.0)) {
        let assign58870_e95819: f64 = (locals.var_t0 * locals.var_t1);
        (assign58870_e95819, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)),)
    } else {
        (locals.var_cdesw, locals.var_cdesw_dn3, locals.var_cdesw_dn4, locals.var_cdesw_dn5, locals.var_cdesw_dn6, locals.var_cdesw_dn7, locals.var_cdesw_dn8, locals.var_cdesw_dn9, locals.var_cdesw_dn10, locals.var_cdesw_dn11,)
    }
};
        locals.var_cdesw = assign58870_e95821;
        locals.var_cdesw_dn3 = assign58870_e95821_d_n3;
        locals.var_cdesw_dn4 = assign58870_e95821_d_n4;
        locals.var_cdesw_dn5 = assign58870_e95821_d_n5;
        locals.var_cdesw_dn6 = assign58870_e95821_d_n6;
        locals.var_cdesw_dn7 = assign58870_e95821_d_n7;
        locals.var_cdesw_dn8 = assign58870_e95821_d_n8;
        locals.var_cdesw_dn9 = assign58870_e95821_d_n9;
        locals.var_cdesw_dn10 = assign58870_e95821_d_n10;
        locals.var_cdesw_dn11 = assign58870_e95821_d_n11;
        locals.var_cdesw_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_205(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign58880_e95829, assign58880_e95829_d_n3, assign58880_e95829_d_n4, assign58880_e95829_d_n5, assign58880_e95829_d_n6, assign58880_e95829_d_n7, assign58880_e95829_d_n8, assign58880_e95829_d_n9, assign58880_e95829_d_n10, assign58880_e95829_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard865 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdesw, locals.var_cdesw_dn3, locals.var_cdesw_dn4, locals.var_cdesw_dn5, locals.var_cdesw_dn6, locals.var_cdesw_dn7, locals.var_cdesw_dn8, locals.var_cdesw_dn9, locals.var_cdesw_dn10, locals.var_cdesw_dn11,)
    }
};
        locals.var_cdesw = assign58880_e95829;
        locals.var_cdesw_dn3 = assign58880_e95829_d_n3;
        locals.var_cdesw_dn4 = assign58880_e95829_d_n4;
        locals.var_cdesw_dn5 = assign58880_e95829_d_n5;
        locals.var_cdesw_dn6 = assign58880_e95829_d_n6;
        locals.var_cdesw_dn7 = assign58880_e95829_d_n7;
        locals.var_cdesw_dn8 = assign58880_e95829_d_n8;
        locals.var_cdesw_dn9 = assign58880_e95829_d_n9;
        locals.var_cdesw_dn10 = assign58880_e95829_d_n10;
        locals.var_cdesw_dn11 = assign58880_e95829_d_n11;
        locals.var_cdesw_rv = 0.0;

        let (assign58890_e95836,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58890_e95834: f64 = (locals.var_cbox_1 * p.p17);
        (assign58890_e95834,)
    } else {
        (locals.var_csbox,)
    }
};
        locals.var_csbox = assign58890_e95836;
        locals.var_csbox_rv = 0.0;

        let (assign58900_e95843,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58900_e95841: f64 = (p.p1396 * p.p17);
        (assign58900_e95841,)
    } else {
        (locals.var_csmin,)
    }
};
        locals.var_csmin = assign58900_e95843;
        locals.var_csmin_rv = 0.0;

        let (assign58910_e95850,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58910_e95848: f64 = (locals.var_cbox_1 * p.p18);
        (assign58910_e95848,)
    } else {
        (locals.var_cdbox,)
    }
};
        locals.var_cdbox = assign58910_e95850;
        locals.var_cdbox_rv = 0.0;

        let (assign58920_e95857,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58920_e95855: f64 = (p.p1396 * p.p18);
        (assign58920_e95855,)
    } else {
        (locals.var_cdmin,)
    }
};
        locals.var_cdmin = assign58920_e95857;
        locals.var_cdmin_rv = 0.0;

        let (assign58930_e95865, assign58930_e95865_d_n3, assign58930_e95865_d_n4, assign58930_e95865_d_n5, assign58930_e95865_d_n6, assign58930_e95865_d_n7, assign58930_e95865_d_n8, assign58930_e95865_d_n9, assign58930_e95865_d_n10, assign58930_e95865_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58930_e95861: f64 = (-locals.var_devsign);
        let assign58930_e95863: f64 = (assign58930_e95861 * locals.var_ves_1);
        (assign58930_e95863, (assign58930_e95861 * locals.var_ves_1_dn3), 0.0, 0.0, (assign58930_e95861 * locals.var_ves_1_dn6), (assign58930_e95861 * locals.var_ves_1_dn7), 0.0, 0.0, (assign58930_e95861 * locals.var_ves_1_dn10), 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign58930_e95865;
        locals.var_t10_dn3 = assign58930_e95865_d_n3;
        locals.var_t10_dn4 = assign58930_e95865_d_n4;
        locals.var_t10_dn5 = assign58930_e95865_d_n5;
        locals.var_t10_dn6 = assign58930_e95865_d_n6;
        locals.var_t10_dn7 = assign58930_e95865_d_n7;
        locals.var_t10_dn8 = assign58930_e95865_d_n8;
        locals.var_t10_dn9 = assign58930_e95865_d_n9;
        locals.var_t10_dn10 = assign58930_e95865_d_n10;
        locals.var_t10_dn11 = assign58930_e95865_d_n11;
        locals.var_t10_rv = 0.0;

        let (assign58940_e95873, assign58940_e95873_d_n3, assign58940_e95873_d_n4, assign58940_e95873_d_n5, assign58940_e95873_d_n6, assign58940_e95873_d_n7, assign58940_e95873_d_n8, assign58940_e95873_d_n9, assign58940_e95873_d_n10, assign58940_e95873_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign58940_e95869: f64 = (-locals.var_devsign);
        let assign58940_e95871: f64 = (assign58940_e95869 * locals.var_ved);
        (assign58940_e95871, (assign58940_e95869 * locals.var_ved_dn3), 0.0, 0.0, (assign58940_e95869 * locals.var_ved_dn6), (assign58940_e95869 * locals.var_ved_dn7), 0.0, 0.0, (assign58940_e95869 * locals.var_ved_dn10), 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign58940_e95873;
        locals.var_t11_dn3 = assign58940_e95873_d_n3;
        locals.var_t11_dn4 = assign58940_e95873_d_n4;
        locals.var_t11_dn5 = assign58940_e95873_d_n5;
        locals.var_t11_dn6 = assign58940_e95873_d_n6;
        locals.var_t11_dn7 = assign58940_e95873_d_n7;
        locals.var_t11_dn8 = assign58940_e95873_d_n8;
        locals.var_t11_dn9 = assign58940_e95873_d_n9;
        locals.var_t11_dn10 = assign58940_e95873_d_n10;
        locals.var_t11_dn11 = assign58940_e95873_d_n11;
        locals.var_t11_rv = 0.0;

        let assign58950_e95876: f64 = if p.p1396 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard866 = assign58950_e95876;
        locals.var_guard866_rv = 0.0;

        let (assign58960_e95890, assign58960_e95890_d_n3, assign58960_e95890_d_n4, assign58960_e95890_d_n5, assign58960_e95890_d_n6, assign58960_e95890_d_n7, assign58960_e95890_d_n8, assign58960_e95890_d_n9, assign58960_e95890_d_n10, assign58960_e95890_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58960_e95882: f64 = (-0.5);
        let assign58960_e95885: f64 = (locals.var_cdbox - locals.var_cdmin);
        let assign58960_e95886: f64 = (assign58960_e95882 * assign58960_e95885);
        let assign58960_e95888: f64 = (assign58960_e95886 / p.p1399);
        (assign58960_e95888, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign58960_e95890;
        locals.var_t1_dn3 = assign58960_e95890_d_n3;
        locals.var_t1_dn4 = assign58960_e95890_d_n4;
        locals.var_t1_dn5 = assign58960_e95890_d_n5;
        locals.var_t1_dn6 = assign58960_e95890_d_n6;
        locals.var_t1_dn7 = assign58960_e95890_d_n7;
        locals.var_t1_dn8 = assign58960_e95890_d_n8;
        locals.var_t1_dn9 = assign58960_e95890_d_n9;
        locals.var_t1_dn10 = assign58960_e95890_d_n10;
        locals.var_t1_dn11 = assign58960_e95890_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign58970_e95906, assign58970_e95906_d_n3, assign58970_e95906_d_n4, assign58970_e95906_d_n5, assign58970_e95906_d_n6, assign58970_e95906_d_n7, assign58970_e95906_d_n8, assign58970_e95906_d_n9, assign58970_e95906_d_n10, assign58970_e95906_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58970_e95896: f64 = (-p.p1399);
        let assign58970_e95898: f64 = (assign58970_e95896 * locals.var_t11);
        let assign58970_e95900: f64 = (assign58970_e95898 + p.p1400);
        let assign58970_e95901: f64 = (assign58970_e95900).cosh();
        let assign58970_e95903: f64 = (assign58970_e95901).max(1e-38);
        let assign58970_e95904: f64 = (assign58970_e95903).ln();
        (assign58970_e95904, (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn3)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn4)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn5)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn6)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn7)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn8)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn9)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn10)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * locals.var_t11_dn11)) } else { 0.0 } / assign58970_e95903),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign58970_e95906;
        locals.var_t2_dn3 = assign58970_e95906_d_n3;
        locals.var_t2_dn4 = assign58970_e95906_d_n4;
        locals.var_t2_dn5 = assign58970_e95906_d_n5;
        locals.var_t2_dn6 = assign58970_e95906_d_n6;
        locals.var_t2_dn7 = assign58970_e95906_d_n7;
        locals.var_t2_dn8 = assign58970_e95906_d_n8;
        locals.var_t2_dn9 = assign58970_e95906_d_n9;
        locals.var_t2_dn10 = assign58970_e95906_d_n10;
        locals.var_t2_dn11 = assign58970_e95906_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign58980_e95919, assign58980_e95919_d_n3, assign58980_e95919_d_n4, assign58980_e95919_d_n5, assign58980_e95919_d_n6, assign58980_e95919_d_n7, assign58980_e95919_d_n8, assign58980_e95919_d_n9, assign58980_e95919_d_n10, assign58980_e95919_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58980_e95914: f64 = (locals.var_cdbox + locals.var_cdmin);
        let assign58980_e95915: f64 = (0.5 * assign58980_e95914);
        let assign58980_e95917: f64 = (assign58980_e95915 * locals.var_t11);
        (assign58980_e95917, (assign58980_e95915 * locals.var_t11_dn3), (assign58980_e95915 * locals.var_t11_dn4), (assign58980_e95915 * locals.var_t11_dn5), (assign58980_e95915 * locals.var_t11_dn6), (assign58980_e95915 * locals.var_t11_dn7), (assign58980_e95915 * locals.var_t11_dn8), (assign58980_e95915 * locals.var_t11_dn9), (assign58980_e95915 * locals.var_t11_dn10), (assign58980_e95915 * locals.var_t11_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign58980_e95919;
        locals.var_t3_dn3 = assign58980_e95919_d_n3;
        locals.var_t3_dn4 = assign58980_e95919_d_n4;
        locals.var_t3_dn5 = assign58980_e95919_d_n5;
        locals.var_t3_dn6 = assign58980_e95919_d_n6;
        locals.var_t3_dn7 = assign58980_e95919_d_n7;
        locals.var_t3_dn8 = assign58980_e95919_d_n8;
        locals.var_t3_dn9 = assign58980_e95919_d_n9;
        locals.var_t3_dn10 = assign58980_e95919_d_n10;
        locals.var_t3_dn11 = assign58980_e95919_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign58990_e95930, assign58990_e95930_d_n3, assign58990_e95930_d_n4, assign58990_e95930_d_n5, assign58990_e95930_d_n6, assign58990_e95930_d_n7, assign58990_e95930_d_n8, assign58990_e95930_d_n9, assign58990_e95930_d_n10, assign58990_e95930_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign58990_e95926: f64 = (locals.var_t1 * locals.var_t2);
        let assign58990_e95928: f64 = (assign58990_e95926 + locals.var_t3);
        (assign58990_e95928, (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) + locals.var_t3_dn3), (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) + locals.var_t3_dn4), (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) + locals.var_t3_dn5), (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) + locals.var_t3_dn6), (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) + locals.var_t3_dn7), (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) + locals.var_t3_dn8), (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) + locals.var_t3_dn9), (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) + locals.var_t3_dn10), (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) + locals.var_t3_dn11),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign58990_e95930;
        locals.var_qde_dn3 = assign58990_e95930_d_n3;
        locals.var_qde_dn4 = assign58990_e95930_d_n4;
        locals.var_qde_dn5 = assign58990_e95930_d_n5;
        locals.var_qde_dn6 = assign58990_e95930_d_n6;
        locals.var_qde_dn7 = assign58990_e95930_d_n7;
        locals.var_qde_dn8 = assign58990_e95930_d_n8;
        locals.var_qde_dn9 = assign58990_e95930_d_n9;
        locals.var_qde_dn10 = assign58990_e95930_d_n10;
        locals.var_qde_dn11 = assign58990_e95930_d_n11;
        locals.var_qde_rv = 0.0;

        let (assign59000_e95944, assign59000_e95944_d_n3, assign59000_e95944_d_n4, assign59000_e95944_d_n5, assign59000_e95944_d_n6, assign59000_e95944_d_n7, assign59000_e95944_d_n8, assign59000_e95944_d_n9, assign59000_e95944_d_n10, assign59000_e95944_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59000_e95936: f64 = (-0.5);
        let assign59000_e95939: f64 = (locals.var_csbox - locals.var_csmin);
        let assign59000_e95940: f64 = (assign59000_e95936 * assign59000_e95939);
        let assign59000_e95942: f64 = (assign59000_e95940 / p.p1397);
        (assign59000_e95942, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59000_e95944;
        locals.var_t1_dn3 = assign59000_e95944_d_n3;
        locals.var_t1_dn4 = assign59000_e95944_d_n4;
        locals.var_t1_dn5 = assign59000_e95944_d_n5;
        locals.var_t1_dn6 = assign59000_e95944_d_n6;
        locals.var_t1_dn7 = assign59000_e95944_d_n7;
        locals.var_t1_dn8 = assign59000_e95944_d_n8;
        locals.var_t1_dn9 = assign59000_e95944_d_n9;
        locals.var_t1_dn10 = assign59000_e95944_d_n10;
        locals.var_t1_dn11 = assign59000_e95944_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59010_e95960, assign59010_e95960_d_n3, assign59010_e95960_d_n4, assign59010_e95960_d_n5, assign59010_e95960_d_n6, assign59010_e95960_d_n7, assign59010_e95960_d_n8, assign59010_e95960_d_n9, assign59010_e95960_d_n10, assign59010_e95960_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59010_e95950: f64 = (-p.p1397);
        let assign59010_e95952: f64 = (assign59010_e95950 * locals.var_t10);
        let assign59010_e95954: f64 = (assign59010_e95952 + p.p1398);
        let assign59010_e95955: f64 = (assign59010_e95954).cosh();
        let assign59010_e95957: f64 = (assign59010_e95955).max(1e-38);
        let assign59010_e95958: f64 = (assign59010_e95957).ln();
        (assign59010_e95958, (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn3)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn4)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn5)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn6)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn7)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn8)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn9)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn10)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * locals.var_t10_dn11)) } else { 0.0 } / assign59010_e95957),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59010_e95960;
        locals.var_t2_dn3 = assign59010_e95960_d_n3;
        locals.var_t2_dn4 = assign59010_e95960_d_n4;
        locals.var_t2_dn5 = assign59010_e95960_d_n5;
        locals.var_t2_dn6 = assign59010_e95960_d_n6;
        locals.var_t2_dn7 = assign59010_e95960_d_n7;
        locals.var_t2_dn8 = assign59010_e95960_d_n8;
        locals.var_t2_dn9 = assign59010_e95960_d_n9;
        locals.var_t2_dn10 = assign59010_e95960_d_n10;
        locals.var_t2_dn11 = assign59010_e95960_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign59020_e95973, assign59020_e95973_d_n3, assign59020_e95973_d_n4, assign59020_e95973_d_n5, assign59020_e95973_d_n6, assign59020_e95973_d_n7, assign59020_e95973_d_n8, assign59020_e95973_d_n9, assign59020_e95973_d_n10, assign59020_e95973_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59020_e95968: f64 = (locals.var_csbox + locals.var_csmin);
        let assign59020_e95969: f64 = (0.5 * assign59020_e95968);
        let assign59020_e95971: f64 = (assign59020_e95969 * locals.var_t10);
        (assign59020_e95971, (assign59020_e95969 * locals.var_t10_dn3), (assign59020_e95969 * locals.var_t10_dn4), (assign59020_e95969 * locals.var_t10_dn5), (assign59020_e95969 * locals.var_t10_dn6), (assign59020_e95969 * locals.var_t10_dn7), (assign59020_e95969 * locals.var_t10_dn8), (assign59020_e95969 * locals.var_t10_dn9), (assign59020_e95969 * locals.var_t10_dn10), (assign59020_e95969 * locals.var_t10_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59020_e95973;
        locals.var_t3_dn3 = assign59020_e95973_d_n3;
        locals.var_t3_dn4 = assign59020_e95973_d_n4;
        locals.var_t3_dn5 = assign59020_e95973_d_n5;
        locals.var_t3_dn6 = assign59020_e95973_d_n6;
        locals.var_t3_dn7 = assign59020_e95973_d_n7;
        locals.var_t3_dn8 = assign59020_e95973_d_n8;
        locals.var_t3_dn9 = assign59020_e95973_d_n9;
        locals.var_t3_dn10 = assign59020_e95973_d_n10;
        locals.var_t3_dn11 = assign59020_e95973_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59030_e95984, assign59030_e95984_d_n3, assign59030_e95984_d_n4, assign59030_e95984_d_n5, assign59030_e95984_d_n6, assign59030_e95984_d_n7, assign59030_e95984_d_n8, assign59030_e95984_d_n9, assign59030_e95984_d_n10, assign59030_e95984_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 != 0.0)) {
        let assign59030_e95980: f64 = (locals.var_t1 * locals.var_t2);
        let assign59030_e95982: f64 = (assign59030_e95980 + locals.var_t3);
        (assign59030_e95982, (((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) + locals.var_t3_dn3), (((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) + locals.var_t3_dn4), (((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) + locals.var_t3_dn5), (((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) + locals.var_t3_dn6), (((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) + locals.var_t3_dn7), (((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) + locals.var_t3_dn8), (((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) + locals.var_t3_dn9), (((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) + locals.var_t3_dn10), (((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) + locals.var_t3_dn11),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign59030_e95984;
        locals.var_qse_dn3 = assign59030_e95984_d_n3;
        locals.var_qse_dn4 = assign59030_e95984_d_n4;
        locals.var_qse_dn5 = assign59030_e95984_d_n5;
        locals.var_qse_dn6 = assign59030_e95984_d_n6;
        locals.var_qse_dn7 = assign59030_e95984_d_n7;
        locals.var_qse_dn8 = assign59030_e95984_d_n8;
        locals.var_qse_dn9 = assign59030_e95984_d_n9;
        locals.var_qse_dn10 = assign59030_e95984_d_n10;
        locals.var_qse_dn11 = assign59030_e95984_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign59040_e95994, assign59040_e95994_d_n3, assign59040_e95994_d_n4, assign59040_e95994_d_n5, assign59040_e95994_d_n6, assign59040_e95994_d_n7, assign59040_e95994_d_n8, assign59040_e95994_d_n9, assign59040_e95994_d_n10, assign59040_e95994_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 == 0.0)) {
        let assign59040_e95992: f64 = (locals.var_csbox * locals.var_t10);
        (assign59040_e95992, (locals.var_csbox * locals.var_t10_dn3), (locals.var_csbox * locals.var_t10_dn4), (locals.var_csbox * locals.var_t10_dn5), (locals.var_csbox * locals.var_t10_dn6), (locals.var_csbox * locals.var_t10_dn7), (locals.var_csbox * locals.var_t10_dn8), (locals.var_csbox * locals.var_t10_dn9), (locals.var_csbox * locals.var_t10_dn10), (locals.var_csbox * locals.var_t10_dn11),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign59040_e95994;
        locals.var_qse_dn3 = assign59040_e95994_d_n3;
        locals.var_qse_dn4 = assign59040_e95994_d_n4;
        locals.var_qse_dn5 = assign59040_e95994_d_n5;
        locals.var_qse_dn6 = assign59040_e95994_d_n6;
        locals.var_qse_dn7 = assign59040_e95994_d_n7;
        locals.var_qse_dn8 = assign59040_e95994_d_n8;
        locals.var_qse_dn9 = assign59040_e95994_d_n9;
        locals.var_qse_dn10 = assign59040_e95994_d_n10;
        locals.var_qse_dn11 = assign59040_e95994_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign59050_e96004, assign59050_e96004_d_n3, assign59050_e96004_d_n4, assign59050_e96004_d_n5, assign59050_e96004_d_n6, assign59050_e96004_d_n7, assign59050_e96004_d_n8, assign59050_e96004_d_n9, assign59050_e96004_d_n10, assign59050_e96004_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard866 == 0.0)) {
        let assign59050_e96002: f64 = (locals.var_cdbox * locals.var_t11);
        (assign59050_e96002, (locals.var_cdbox * locals.var_t11_dn3), (locals.var_cdbox * locals.var_t11_dn4), (locals.var_cdbox * locals.var_t11_dn5), (locals.var_cdbox * locals.var_t11_dn6), (locals.var_cdbox * locals.var_t11_dn7), (locals.var_cdbox * locals.var_t11_dn8), (locals.var_cdbox * locals.var_t11_dn9), (locals.var_cdbox * locals.var_t11_dn10), (locals.var_cdbox * locals.var_t11_dn11),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign59050_e96004;
        locals.var_qde_dn3 = assign59050_e96004_d_n3;
        locals.var_qde_dn4 = assign59050_e96004_d_n4;
        locals.var_qde_dn5 = assign59050_e96004_d_n5;
        locals.var_qde_dn6 = assign59050_e96004_d_n6;
        locals.var_qde_dn7 = assign59050_e96004_d_n7;
        locals.var_qde_dn8 = assign59050_e96004_d_n8;
        locals.var_qde_dn9 = assign59050_e96004_d_n9;
        locals.var_qde_dn10 = assign59050_e96004_d_n10;
        locals.var_qde_dn11 = assign59050_e96004_d_n11;
        locals.var_qde_rv = 0.0;

        let (assign59060_e96013, assign59060_e96013_d_n3, assign59060_e96013_d_n4, assign59060_e96013_d_n5, assign59060_e96013_d_n6, assign59060_e96013_d_n7, assign59060_e96013_d_n8, assign59060_e96013_d_n9, assign59060_e96013_d_n10, assign59060_e96013_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign59060_e96010: f64 = (locals.var_csesw * locals.var_t10);
        let assign59060_e96011: f64 = (locals.var_qse + assign59060_e96010);
        (assign59060_e96011, (locals.var_qse_dn3 + ((locals.var_csesw_dn3 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn3))), (locals.var_qse_dn4 + ((locals.var_csesw_dn4 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn4))), (locals.var_qse_dn5 + ((locals.var_csesw_dn5 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn5))), (locals.var_qse_dn6 + ((locals.var_csesw_dn6 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn6))), (locals.var_qse_dn7 + ((locals.var_csesw_dn7 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn7))), (locals.var_qse_dn8 + ((locals.var_csesw_dn8 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn8))), (locals.var_qse_dn9 + ((locals.var_csesw_dn9 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn9))), (locals.var_qse_dn10 + ((locals.var_csesw_dn10 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn10))), (locals.var_qse_dn11 + ((locals.var_csesw_dn11 * locals.var_t10) + (locals.var_csesw * locals.var_t10_dn11))),)
    } else {
        (locals.var_qse, locals.var_qse_dn3, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn11,)
    }
};
        locals.var_qse = assign59060_e96013;
        locals.var_qse_dn3 = assign59060_e96013_d_n3;
        locals.var_qse_dn4 = assign59060_e96013_d_n4;
        locals.var_qse_dn5 = assign59060_e96013_d_n5;
        locals.var_qse_dn6 = assign59060_e96013_d_n6;
        locals.var_qse_dn7 = assign59060_e96013_d_n7;
        locals.var_qse_dn8 = assign59060_e96013_d_n8;
        locals.var_qse_dn9 = assign59060_e96013_d_n9;
        locals.var_qse_dn10 = assign59060_e96013_d_n10;
        locals.var_qse_dn11 = assign59060_e96013_d_n11;
        locals.var_qse_rv = 0.0;

        let (assign59070_e96022, assign59070_e96022_d_n3, assign59070_e96022_d_n4, assign59070_e96022_d_n5, assign59070_e96022_d_n6, assign59070_e96022_d_n7, assign59070_e96022_d_n8, assign59070_e96022_d_n9, assign59070_e96022_d_n10, assign59070_e96022_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign59070_e96019: f64 = (locals.var_cdesw * locals.var_t11);
        let assign59070_e96020: f64 = (locals.var_qde + assign59070_e96019);
        (assign59070_e96020, (locals.var_qde_dn3 + ((locals.var_cdesw_dn3 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn3))), (locals.var_qde_dn4 + ((locals.var_cdesw_dn4 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn4))), (locals.var_qde_dn5 + ((locals.var_cdesw_dn5 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn5))), (locals.var_qde_dn6 + ((locals.var_cdesw_dn6 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn6))), (locals.var_qde_dn7 + ((locals.var_cdesw_dn7 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn7))), (locals.var_qde_dn8 + ((locals.var_cdesw_dn8 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn8))), (locals.var_qde_dn9 + ((locals.var_cdesw_dn9 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn9))), (locals.var_qde_dn10 + ((locals.var_cdesw_dn10 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn10))), (locals.var_qde_dn11 + ((locals.var_cdesw_dn11 * locals.var_t11) + (locals.var_cdesw * locals.var_t11_dn11))),)
    } else {
        (locals.var_qde, locals.var_qde_dn3, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11,)
    }
};
        locals.var_qde = assign59070_e96022;
        locals.var_qde_dn3 = assign59070_e96022_d_n3;
        locals.var_qde_dn4 = assign59070_e96022_d_n4;
        locals.var_qde_dn5 = assign59070_e96022_d_n5;
        locals.var_qde_dn6 = assign59070_e96022_d_n6;
        locals.var_qde_dn7 = assign59070_e96022_d_n7;
        locals.var_qde_dn8 = assign59070_e96022_d_n8;
        locals.var_qde_dn9 = assign59070_e96022_d_n9;
        locals.var_qde_dn10 = assign59070_e96022_d_n10;
        locals.var_qde_dn11 = assign59070_e96022_d_n11;
        locals.var_qde_rv = 0.0;

        let assign59080_e96025: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard867 = assign59080_e96025;
        locals.var_guard867_rv = 0.0;

        let (assign59090_e96037, assign59090_e96037_d_n3, assign59090_e96037_d_n4, assign59090_e96037_d_n5, assign59090_e96037_d_n6, assign59090_e96037_d_n7, assign59090_e96037_d_n8, assign59090_e96037_d_n9, assign59090_e96037_d_n10, assign59090_e96037_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59090_e96032: f64 = (locals.var_ndepedge_i / locals.var_ni);
        let assign59090_e96034: f64 = (assign59090_e96032).max(1e-38);
        let assign59090_e96035: f64 = (assign59090_e96034).ln();
        (assign59090_e96035, (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign59090_e96034),)
    } else {
        (locals.var_phib_edge, locals.var_phib_edge_dn3, locals.var_phib_edge_dn4, locals.var_phib_edge_dn5, locals.var_phib_edge_dn6, locals.var_phib_edge_dn7, locals.var_phib_edge_dn8, locals.var_phib_edge_dn9, locals.var_phib_edge_dn10, locals.var_phib_edge_dn11,)
    }
};
        locals.var_phib_edge = assign59090_e96037;
        locals.var_phib_edge_dn3 = assign59090_e96037_d_n3;
        locals.var_phib_edge_dn4 = assign59090_e96037_d_n4;
        locals.var_phib_edge_dn5 = assign59090_e96037_d_n5;
        locals.var_phib_edge_dn6 = assign59090_e96037_d_n6;
        locals.var_phib_edge_dn7 = assign59090_e96037_d_n7;
        locals.var_phib_edge_dn8 = assign59090_e96037_d_n8;
        locals.var_phib_edge_dn9 = assign59090_e96037_d_n9;
        locals.var_phib_edge_dn10 = assign59090_e96037_d_n10;
        locals.var_phib_edge_dn11 = assign59090_e96037_d_n11;
        locals.var_phib_edge_rv = 0.0;

        let (assign59100_e96052, assign59100_e96052_d_n3, assign59100_e96052_d_n4, assign59100_e96052_d_n5, assign59100_e96052_d_n6, assign59100_e96052_d_n7, assign59100_e96052_d_n8, assign59100_e96052_d_n9, assign59100_e96052_d_n10, assign59100_e96052_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59100_e96045: f64 = (locals.var_vt * locals.var_phib_edge);
        let assign59100_e96046: f64 = (0.4 + assign59100_e96045);
        let assign59100_e96048: f64 = (assign59100_e96046 + locals.var_phin_i);
        let assign59100_e96050: f64 = (assign59100_e96048).max(0.4);
        (assign59100_e96050, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn3) } else { 0.0 }, if assign59100_e96048 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib_edge) + (locals.var_vt * locals.var_phib_edge_dn4)) } else { 0.0 }, if assign59100_e96048 >= 0.4 { ((locals.var_vt_dn5 * locals.var_phib_edge) + (locals.var_vt * locals.var_phib_edge_dn5)) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn6) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn7) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn8) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn9) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn10) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn11) } else { 0.0 },)
    } else {
        (locals.var_phist, locals.var_phist_dn3, locals.var_phist_dn4, locals.var_phist_dn5, locals.var_phist_dn6, locals.var_phist_dn7, locals.var_phist_dn8, locals.var_phist_dn9, locals.var_phist_dn10, locals.var_phist_dn11,)
    }
};
        locals.var_phist = assign59100_e96052;
        locals.var_phist_dn3 = assign59100_e96052_d_n3;
        locals.var_phist_dn4 = assign59100_e96052_d_n4;
        locals.var_phist_dn5 = assign59100_e96052_d_n5;
        locals.var_phist_dn6 = assign59100_e96052_d_n6;
        locals.var_phist_dn7 = assign59100_e96052_d_n7;
        locals.var_phist_dn8 = assign59100_e96052_d_n8;
        locals.var_phist_dn9 = assign59100_e96052_d_n9;
        locals.var_phist_dn10 = assign59100_e96052_d_n10;
        locals.var_phist_dn11 = assign59100_e96052_d_n11;
        locals.var_phist_rv = 0.0;

        let (assign59110_e96066, assign59110_e96066_d_n3, assign59110_e96066_d_n4, assign59110_e96066_d_n5, assign59110_e96066_d_n6, assign59110_e96066_d_n7, assign59110_e96066_d_n8, assign59110_e96066_d_n9, assign59110_e96066_d_n10, assign59110_e96066_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59110_e96059: f64 = (2.0 * locals.var_epssi);
        let assign59110_e96062: f64 = (1.602176462e-19 * locals.var_ndepedge_i);
        let assign59110_e96063: f64 = (assign59110_e96059 / assign59110_e96062);
        let assign59110_e96064: f64 = (assign59110_e96063).sqrt();
        (assign59110_e96064, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1dep, locals.var_t1dep_dn3, locals.var_t1dep_dn4, locals.var_t1dep_dn5, locals.var_t1dep_dn6, locals.var_t1dep_dn7, locals.var_t1dep_dn8, locals.var_t1dep_dn9, locals.var_t1dep_dn10, locals.var_t1dep_dn11,)
    }
};
        locals.var_t1dep = assign59110_e96066;
        locals.var_t1dep_dn3 = assign59110_e96066_d_n3;
        locals.var_t1dep_dn4 = assign59110_e96066_d_n4;
        locals.var_t1dep_dn5 = assign59110_e96066_d_n5;
        locals.var_t1dep_dn6 = assign59110_e96066_d_n6;
        locals.var_t1dep_dn7 = assign59110_e96066_d_n7;
        locals.var_t1dep_dn8 = assign59110_e96066_d_n8;
        locals.var_t1dep_dn9 = assign59110_e96066_d_n9;
        locals.var_t1dep_dn10 = assign59110_e96066_d_n10;
        locals.var_t1dep_dn11 = assign59110_e96066_d_n11;
        locals.var_t1dep_rv = 0.0;

        let (assign59120_e96106, assign59120_e96106_d_n4, assign59120_e96106_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59120_e96077: f64 = (locals.var_tratio - 1.0);
        let assign59120_e96078: f64 = (locals.var_tnfactoredge_i * assign59120_e96077);
        let assign59120_e96079: f64 = (1.0 + assign59120_e96078);
        let assign59120_e96084: f64 = (locals.var_tratio - 1.0);
        let assign59120_e96085: f64 = (locals.var_tnfactoredge_i * assign59120_e96084);
        let assign59120_e96086: f64 = (1.0 + assign59120_e96085);
        let assign59120_e96091: f64 = (locals.var_tratio - 1.0);
        let assign59120_e96092: f64 = (locals.var_tnfactoredge_i * assign59120_e96091);
        let assign59120_e96093: f64 = (1.0 + assign59120_e96092);
        let assign59120_e96094: f64 = (assign59120_e96086 * assign59120_e96093);
        let assign59120_e96097: f64 = (4.0 * 0.001);
        let assign59120_e96099: f64 = (assign59120_e96097 * 0.001);
        let assign59120_e96100: f64 = (assign59120_e96094 + assign59120_e96099);
        let assign59120_e96101: f64 = (assign59120_e96100).sqrt();
        let assign59120_e96102: f64 = (assign59120_e96079 + assign59120_e96101);
        let assign59120_e96103: f64 = (0.5 * assign59120_e96102);
        let assign59120_e96104: f64 = (locals.var_nfactoredge_i * assign59120_e96103);
        (assign59120_e96104, (locals.var_nfactoredge_i * (0.5 * ((locals.var_tnfactoredge_i * locals.var_tratio_dn4) + ((((locals.var_tnfactoredge_i * locals.var_tratio_dn4) * assign59120_e96093) + (assign59120_e96086 * (locals.var_tnfactoredge_i * locals.var_tratio_dn4))) / (2.0 * assign59120_e96101))))), (locals.var_nfactoredge_i * (0.5 * ((locals.var_tnfactoredge_i * locals.var_tratio_dn5) + ((((locals.var_tnfactoredge_i * locals.var_tratio_dn5) * assign59120_e96093) + (assign59120_e96086 * (locals.var_tnfactoredge_i * locals.var_tratio_dn5))) / (2.0 * assign59120_e96101))))),)
    } else {
        (locals.var_nfactoredge_t, locals.var_nfactoredge_t_dn4, locals.var_nfactoredge_t_dn5,)
    }
};
        locals.var_nfactoredge_t = assign59120_e96106;
        locals.var_nfactoredge_t_dn4 = assign59120_e96106_d_n4;
        locals.var_nfactoredge_t_dn5 = assign59120_e96106_d_n5;
        locals.var_nfactoredge_t_rv = 0.0;

        let (assign59130_e96121, assign59130_e96121_d_n3, assign59130_e96121_d_n4, assign59130_e96121_d_n5, assign59130_e96121_d_n6, assign59130_e96121_d_n7, assign59130_e96121_d_n8, assign59130_e96121_d_n9, assign59130_e96121_d_n10, assign59130_e96121_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59130_e96116: f64 = (locals.var_tratio - 1.0);
        let assign59130_e96117: f64 = (locals.var_teta0edge_i * assign59130_e96116);
        let assign59130_e96118: f64 = (1.0 + assign59130_e96117);
        let assign59130_e96119: f64 = (locals.var_eta0edge_i * assign59130_e96118);
        (assign59130_e96119, (locals.var_eta0edge_i_dn3 * assign59130_e96118), ((locals.var_eta0edge_i_dn4 * assign59130_e96118) + (locals.var_eta0edge_i * (locals.var_teta0edge_i * locals.var_tratio_dn4))), ((locals.var_eta0edge_i_dn5 * assign59130_e96118) + (locals.var_eta0edge_i * (locals.var_teta0edge_i * locals.var_tratio_dn5))), (locals.var_eta0edge_i_dn6 * assign59130_e96118), (locals.var_eta0edge_i_dn7 * assign59130_e96118), (locals.var_eta0edge_i_dn8 * assign59130_e96118), (locals.var_eta0edge_i_dn9 * assign59130_e96118), (locals.var_eta0edge_i_dn10 * assign59130_e96118), (locals.var_eta0edge_i_dn11 * assign59130_e96118),)
    } else {
        (locals.var_eta0edge_t, locals.var_eta0edge_t_dn3, locals.var_eta0edge_t_dn4, locals.var_eta0edge_t_dn5, locals.var_eta0edge_t_dn6, locals.var_eta0edge_t_dn7, locals.var_eta0edge_t_dn8, locals.var_eta0edge_t_dn9, locals.var_eta0edge_t_dn10, locals.var_eta0edge_t_dn11,)
    }
};
        locals.var_eta0edge_t = assign59130_e96121;
        locals.var_eta0edge_t_dn3 = assign59130_e96121_d_n3;
        locals.var_eta0edge_t_dn4 = assign59130_e96121_d_n4;
        locals.var_eta0edge_t_dn5 = assign59130_e96121_d_n5;
        locals.var_eta0edge_t_dn6 = assign59130_e96121_d_n6;
        locals.var_eta0edge_t_dn7 = assign59130_e96121_d_n7;
        locals.var_eta0edge_t_dn8 = assign59130_e96121_d_n8;
        locals.var_eta0edge_t_dn9 = assign59130_e96121_d_n9;
        locals.var_eta0edge_t_dn10 = assign59130_e96121_d_n10;
        locals.var_eta0edge_t_dn11 = assign59130_e96121_d_n11;
        locals.var_eta0edge_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_206(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign59140_e96153, assign59140_e96153_d_n3, assign59140_e96153_d_n4, assign59140_e96153_d_n5, assign59140_e96153_d_n6, assign59140_e96153_d_n7, assign59140_e96153_d_n8, assign59140_e96153_d_n9, assign59140_e96153_d_n10, assign59140_e96153_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59140_e96129: f64 = (locals.var_phist - locals.var_vbsx);
        let assign59140_e96131: f64 = (assign59140_e96129 + 0.05);
        let assign59140_e96134: f64 = (locals.var_phist - locals.var_vbsx);
        let assign59140_e96136: f64 = (assign59140_e96134 - 0.05);
        let assign59140_e96139: f64 = (locals.var_phist - locals.var_vbsx);
        let assign59140_e96141: f64 = (assign59140_e96139 - 0.05);
        let assign59140_e96142: f64 = (assign59140_e96136 * assign59140_e96141);
        let assign59140_e96145: f64 = (0.25 * 0.1);
        let assign59140_e96147: f64 = (assign59140_e96145 * 0.1);
        let assign59140_e96148: f64 = (assign59140_e96142 + assign59140_e96147);
        let assign59140_e96149: f64 = (assign59140_e96148).sqrt();
        let assign59140_e96150: f64 = (assign59140_e96131 + assign59140_e96149);
        let assign59140_e96151: f64 = (0.5 * assign59140_e96150);
        (assign59140_e96151, (0.5 * ((locals.var_phist_dn3 - locals.var_vbsx_dn3) + ((((locals.var_phist_dn3 - locals.var_vbsx_dn3) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn4 - locals.var_vbsx_dn4) + ((((locals.var_phist_dn4 - locals.var_vbsx_dn4) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn5 - locals.var_vbsx_dn5) + ((((locals.var_phist_dn5 - locals.var_vbsx_dn5) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn6 - locals.var_vbsx_dn6) + ((((locals.var_phist_dn6 - locals.var_vbsx_dn6) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn7 - locals.var_vbsx_dn7) + ((((locals.var_phist_dn7 - locals.var_vbsx_dn7) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn8 - locals.var_vbsx_dn8) + ((((locals.var_phist_dn8 - locals.var_vbsx_dn8) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn9 - locals.var_vbsx_dn9) + ((((locals.var_phist_dn9 - locals.var_vbsx_dn9) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn10 - locals.var_vbsx_dn10) + ((((locals.var_phist_dn10 - locals.var_vbsx_dn10) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (2.0 * assign59140_e96149)))), (0.5 * ((locals.var_phist_dn11 - locals.var_vbsx_dn11) + ((((locals.var_phist_dn11 - locals.var_vbsx_dn11) * assign59140_e96141) + (assign59140_e96136 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (2.0 * assign59140_e96149)))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11,)
    }
};
        locals.var_phistvbs = assign59140_e96153;
        locals.var_phistvbs_dn3 = assign59140_e96153_d_n3;
        locals.var_phistvbs_dn4 = assign59140_e96153_d_n4;
        locals.var_phistvbs_dn5 = assign59140_e96153_d_n5;
        locals.var_phistvbs_dn6 = assign59140_e96153_d_n6;
        locals.var_phistvbs_dn7 = assign59140_e96153_d_n7;
        locals.var_phistvbs_dn8 = assign59140_e96153_d_n8;
        locals.var_phistvbs_dn9 = assign59140_e96153_d_n9;
        locals.var_phistvbs_dn10 = assign59140_e96153_d_n10;
        locals.var_phistvbs_dn11 = assign59140_e96153_d_n11;
        locals.var_phistvbs_rv = 0.0;

        let (assign59150_e96161, assign59150_e96161_d_n3, assign59150_e96161_d_n4, assign59150_e96161_d_n5, assign59150_e96161_d_n6, assign59150_e96161_d_n7, assign59150_e96161_d_n8, assign59150_e96161_d_n9, assign59150_e96161_d_n10, assign59150_e96161_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59150_e96159: f64 = (locals.var_phistvbs).sqrt();
        (assign59150_e96159, (locals.var_phistvbs_dn3 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn4 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn5 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn6 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn7 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn8 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn9 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn10 / (2.0 * assign59150_e96159)), (locals.var_phistvbs_dn11 / (2.0 * assign59150_e96159)),)
    } else {
        (locals.var_sqrtphistvbs, locals.var_sqrtphistvbs_dn3, locals.var_sqrtphistvbs_dn4, locals.var_sqrtphistvbs_dn5, locals.var_sqrtphistvbs_dn6, locals.var_sqrtphistvbs_dn7, locals.var_sqrtphistvbs_dn8, locals.var_sqrtphistvbs_dn9, locals.var_sqrtphistvbs_dn10, locals.var_sqrtphistvbs_dn11,)
    }
};
        locals.var_sqrtphistvbs = assign59150_e96161;
        locals.var_sqrtphistvbs_dn3 = assign59150_e96161_d_n3;
        locals.var_sqrtphistvbs_dn4 = assign59150_e96161_d_n4;
        locals.var_sqrtphistvbs_dn5 = assign59150_e96161_d_n5;
        locals.var_sqrtphistvbs_dn6 = assign59150_e96161_d_n6;
        locals.var_sqrtphistvbs_dn7 = assign59150_e96161_d_n7;
        locals.var_sqrtphistvbs_dn8 = assign59150_e96161_d_n8;
        locals.var_sqrtphistvbs_dn9 = assign59150_e96161_d_n9;
        locals.var_sqrtphistvbs_dn10 = assign59150_e96161_d_n10;
        locals.var_sqrtphistvbs_dn11 = assign59150_e96161_d_n11;
        locals.var_sqrtphistvbs_rv = 0.0;

        let (assign59160_e96170, assign59160_e96170_d_n3, assign59160_e96170_d_n4, assign59160_e96170_d_n5, assign59160_e96170_d_n6, assign59160_e96170_d_n7, assign59160_e96170_d_n8, assign59160_e96170_d_n9, assign59160_e96170_d_n10, assign59160_e96170_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59160_e96168: f64 = (locals.var_t1dep * locals.var_sqrtphistvbs);
        (assign59160_e96168, ((locals.var_t1dep_dn3 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn3)), ((locals.var_t1dep_dn4 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn4)), ((locals.var_t1dep_dn5 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn5)), ((locals.var_t1dep_dn6 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn6)), ((locals.var_t1dep_dn7 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn7)), ((locals.var_t1dep_dn8 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn8)), ((locals.var_t1dep_dn9 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn9)), ((locals.var_t1dep_dn10 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn10)), ((locals.var_t1dep_dn11 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn11)),)
    } else {
        (locals.var_xdep, locals.var_xdep_dn3, locals.var_xdep_dn4, locals.var_xdep_dn5, locals.var_xdep_dn6, locals.var_xdep_dn7, locals.var_xdep_dn8, locals.var_xdep_dn9, locals.var_xdep_dn10, locals.var_xdep_dn11,)
    }
};
        locals.var_xdep = assign59160_e96170;
        locals.var_xdep_dn3 = assign59160_e96170_d_n3;
        locals.var_xdep_dn4 = assign59160_e96170_d_n4;
        locals.var_xdep_dn5 = assign59160_e96170_d_n5;
        locals.var_xdep_dn6 = assign59160_e96170_d_n6;
        locals.var_xdep_dn7 = assign59160_e96170_d_n7;
        locals.var_xdep_dn8 = assign59160_e96170_d_n8;
        locals.var_xdep_dn9 = assign59160_e96170_d_n9;
        locals.var_xdep_dn10 = assign59160_e96170_d_n10;
        locals.var_xdep_dn11 = assign59160_e96170_d_n11;
        locals.var_xdep_rv = 0.0;

        let (assign59170_e96179, assign59170_e96179_d_n3, assign59170_e96179_d_n4, assign59170_e96179_d_n5, assign59170_e96179_d_n6, assign59170_e96179_d_n7, assign59170_e96179_d_n8, assign59170_e96179_d_n9, assign59170_e96179_d_n10, assign59170_e96179_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59170_e96177: f64 = (locals.var_epssi / locals.var_xdep);
        (assign59170_e96177, (-((locals.var_epssi * locals.var_xdep_dn3) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn4) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn5) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn6) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn7) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn8) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn9) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn10) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn11) / (locals.var_xdep * locals.var_xdep))),)
    } else {
        (locals.var_cdep, locals.var_cdep_dn3, locals.var_cdep_dn4, locals.var_cdep_dn5, locals.var_cdep_dn6, locals.var_cdep_dn7, locals.var_cdep_dn8, locals.var_cdep_dn9, locals.var_cdep_dn10, locals.var_cdep_dn11,)
    }
};
        locals.var_cdep = assign59170_e96179;
        locals.var_cdep_dn3 = assign59170_e96179_d_n3;
        locals.var_cdep_dn4 = assign59170_e96179_d_n4;
        locals.var_cdep_dn5 = assign59170_e96179_d_n5;
        locals.var_cdep_dn6 = assign59170_e96179_d_n6;
        locals.var_cdep_dn7 = assign59170_e96179_d_n7;
        locals.var_cdep_dn8 = assign59170_e96179_d_n8;
        locals.var_cdep_dn9 = assign59170_e96179_d_n9;
        locals.var_cdep_dn10 = assign59170_e96179_d_n10;
        locals.var_cdep_dn11 = assign59170_e96179_d_n11;
        locals.var_cdep_rv = 0.0;

        let (assign59180_e96196, assign59180_e96196_d_n3, assign59180_e96196_d_n4, assign59180_e96196_d_n5, assign59180_e96196_d_n6, assign59180_e96196_d_n7, assign59180_e96196_d_n8, assign59180_e96196_d_n9, assign59180_e96196_d_n10, assign59180_e96196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59180_e96186: f64 = (locals.var_citedge_i + locals.var_nfactoredge_t);
        let assign59180_e96189: f64 = (locals.var_cdscdedge_a * locals.var_vdsx);
        let assign59180_e96190: f64 = (assign59180_e96186 + assign59180_e96189);
        let assign59180_e96193: f64 = (locals.var_cdscbedge_i * locals.var_vbsx);
        let assign59180_e96194: f64 = (assign59180_e96190 - assign59180_e96193);
        (assign59180_e96194, (((locals.var_cdscdedge_a_dn3 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn3)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn3)), ((locals.var_nfactoredge_t_dn4 + ((locals.var_cdscdedge_a_dn4 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn4))) - (locals.var_cdscbedge_i * locals.var_vbsx_dn4)), ((locals.var_nfactoredge_t_dn5 + ((locals.var_cdscdedge_a_dn5 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn5))) - (locals.var_cdscbedge_i * locals.var_vbsx_dn5)), (((locals.var_cdscdedge_a_dn6 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn6)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn6)), (((locals.var_cdscdedge_a_dn7 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn7)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn7)), (((locals.var_cdscdedge_a_dn8 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn8)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn8)), (((locals.var_cdscdedge_a_dn9 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn9)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn9)), (((locals.var_cdscdedge_a_dn10 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn10)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn10)), (((locals.var_cdscdedge_a_dn11 * locals.var_vdsx) + (locals.var_cdscdedge_a * locals.var_vdsx_dn11)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn11)),)
    } else {
        (locals.var_cdsc, locals.var_cdsc_dn3, locals.var_cdsc_dn4, locals.var_cdsc_dn5, locals.var_cdsc_dn6, locals.var_cdsc_dn7, locals.var_cdsc_dn8, locals.var_cdsc_dn9, locals.var_cdsc_dn10, locals.var_cdsc_dn11,)
    }
};
        locals.var_cdsc = assign59180_e96196;
        locals.var_cdsc_dn3 = assign59180_e96196_d_n3;
        locals.var_cdsc_dn4 = assign59180_e96196_d_n4;
        locals.var_cdsc_dn5 = assign59180_e96196_d_n5;
        locals.var_cdsc_dn6 = assign59180_e96196_d_n6;
        locals.var_cdsc_dn7 = assign59180_e96196_d_n7;
        locals.var_cdsc_dn8 = assign59180_e96196_d_n8;
        locals.var_cdsc_dn9 = assign59180_e96196_d_n9;
        locals.var_cdsc_dn10 = assign59180_e96196_d_n10;
        locals.var_cdsc_dn11 = assign59180_e96196_d_n11;
        locals.var_cdsc_rv = 0.0;

        let (assign59190_e96207, assign59190_e96207_d_n3, assign59190_e96207_d_n4, assign59190_e96207_d_n5, assign59190_e96207_d_n6, assign59190_e96207_d_n7, assign59190_e96207_d_n8, assign59190_e96207_d_n9, assign59190_e96207_d_n10, assign59190_e96207_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59190_e96204: f64 = (locals.var_cdsc / locals.var_cox);
        let assign59190_e96205: f64 = (1.0 + assign59190_e96204);
        (assign59190_e96205, (locals.var_cdsc_dn3 / locals.var_cox), (locals.var_cdsc_dn4 / locals.var_cox), (locals.var_cdsc_dn5 / locals.var_cox), (locals.var_cdsc_dn6 / locals.var_cox), (locals.var_cdsc_dn7 / locals.var_cox), (locals.var_cdsc_dn8 / locals.var_cox), (locals.var_cdsc_dn9 / locals.var_cox), (locals.var_cdsc_dn10 / locals.var_cox), (locals.var_cdsc_dn11 / locals.var_cox),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59190_e96207;
        locals.var_t1_dn3 = assign59190_e96207_d_n3;
        locals.var_t1_dn4 = assign59190_e96207_d_n4;
        locals.var_t1_dn5 = assign59190_e96207_d_n5;
        locals.var_t1_dn6 = assign59190_e96207_d_n6;
        locals.var_t1_dn7 = assign59190_e96207_d_n7;
        locals.var_t1_dn8 = assign59190_e96207_d_n8;
        locals.var_t1_dn9 = assign59190_e96207_d_n9;
        locals.var_t1_dn10 = assign59190_e96207_d_n10;
        locals.var_t1_dn11 = assign59190_e96207_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59200_e96233, assign59200_e96233_d_n3, assign59200_e96233_d_n4, assign59200_e96233_d_n5, assign59200_e96233_d_n6, assign59200_e96233_d_n7, assign59200_e96233_d_n8, assign59200_e96233_d_n9, assign59200_e96233_d_n10, assign59200_e96233_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59200_e96215: f64 = (locals.var_t1 + 1.0);
        let assign59200_e96218: f64 = (locals.var_t1 - 1.0);
        let assign59200_e96221: f64 = (locals.var_t1 - 1.0);
        let assign59200_e96222: f64 = (assign59200_e96218 * assign59200_e96221);
        let assign59200_e96225: f64 = (0.25 * 0.05);
        let assign59200_e96227: f64 = (assign59200_e96225 * 0.05);
        let assign59200_e96228: f64 = (assign59200_e96222 + assign59200_e96227);
        let assign59200_e96229: f64 = (assign59200_e96228).sqrt();
        let assign59200_e96230: f64 = (assign59200_e96215 + assign59200_e96229);
        let assign59200_e96231: f64 = (0.5 * assign59200_e96230);
        (assign59200_e96231, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn3)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn4)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn5)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn6)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn7)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn8)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn9)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn10)) / (2.0 * assign59200_e96229)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign59200_e96221) + (assign59200_e96218 * locals.var_t1_dn11)) / (2.0 * assign59200_e96229)))),)
    } else {
        (locals.var_n, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11,)
    }
};
        locals.var_n = assign59200_e96233;
        locals.var_n_dn3 = assign59200_e96233_d_n3;
        locals.var_n_dn4 = assign59200_e96233_d_n4;
        locals.var_n_dn5 = assign59200_e96233_d_n5;
        locals.var_n_dn6 = assign59200_e96233_d_n6;
        locals.var_n_dn7 = assign59200_e96233_d_n7;
        locals.var_n_dn8 = assign59200_e96233_d_n8;
        locals.var_n_dn9 = assign59200_e96233_d_n9;
        locals.var_n_dn10 = assign59200_e96233_d_n10;
        locals.var_n_dn11 = assign59200_e96233_d_n11;
        locals.var_n_rv = 0.0;

        let (assign59210_e96242, assign59210_e96242_d_n3, assign59210_e96242_d_n4, assign59210_e96242_d_n5, assign59210_e96242_d_n6, assign59210_e96242_d_n7, assign59210_e96242_d_n8, assign59210_e96242_d_n9, assign59210_e96242_d_n10, assign59210_e96242_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59210_e96240: f64 = (locals.var_n * locals.var_vt);
        (assign59210_e96240, (locals.var_n_dn3 * locals.var_vt), ((locals.var_n_dn4 * locals.var_vt) + (locals.var_n * locals.var_vt_dn4)), ((locals.var_n_dn5 * locals.var_vt) + (locals.var_n * locals.var_vt_dn5)), (locals.var_n_dn6 * locals.var_vt), (locals.var_n_dn7 * locals.var_vt), (locals.var_n_dn8 * locals.var_vt), (locals.var_n_dn9 * locals.var_vt), (locals.var_n_dn10 * locals.var_vt), (locals.var_n_dn11 * locals.var_vt),)
    } else {
        (locals.var_nvt, locals.var_nvt_dn3, locals.var_nvt_dn4, locals.var_nvt_dn5, locals.var_nvt_dn6, locals.var_nvt_dn7, locals.var_nvt_dn8, locals.var_nvt_dn9, locals.var_nvt_dn10, locals.var_nvt_dn11,)
    }
};
        locals.var_nvt = assign59210_e96242;
        locals.var_nvt_dn3 = assign59210_e96242_d_n3;
        locals.var_nvt_dn4 = assign59210_e96242_d_n4;
        locals.var_nvt_dn5 = assign59210_e96242_d_n5;
        locals.var_nvt_dn6 = assign59210_e96242_d_n6;
        locals.var_nvt_dn7 = assign59210_e96242_d_n7;
        locals.var_nvt_dn8 = assign59210_e96242_d_n8;
        locals.var_nvt_dn9 = assign59210_e96242_d_n9;
        locals.var_nvt_dn10 = assign59210_e96242_d_n10;
        locals.var_nvt_dn11 = assign59210_e96242_d_n11;
        locals.var_nvt_rv = 0.0;

        let (assign59220_e96251, assign59220_e96251_d_n3, assign59220_e96251_d_n4, assign59220_e96251_d_n5, assign59220_e96251_d_n6, assign59220_e96251_d_n7, assign59220_e96251_d_n8, assign59220_e96251_d_n9, assign59220_e96251_d_n10, assign59220_e96251_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59220_e96249: f64 = (1.0 / locals.var_nvt);
        (assign59220_e96249, (-(locals.var_nvt_dn3 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn4 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn5 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn6 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn7 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn8 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn9 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn10 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn11 / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_inv_nvt, locals.var_inv_nvt_dn3, locals.var_inv_nvt_dn4, locals.var_inv_nvt_dn5, locals.var_inv_nvt_dn6, locals.var_inv_nvt_dn7, locals.var_inv_nvt_dn8, locals.var_inv_nvt_dn9, locals.var_inv_nvt_dn10, locals.var_inv_nvt_dn11,)
    }
};
        locals.var_inv_nvt = assign59220_e96251;
        locals.var_inv_nvt_dn3 = assign59220_e96251_d_n3;
        locals.var_inv_nvt_dn4 = assign59220_e96251_d_n4;
        locals.var_inv_nvt_dn5 = assign59220_e96251_d_n5;
        locals.var_inv_nvt_dn6 = assign59220_e96251_d_n6;
        locals.var_inv_nvt_dn7 = assign59220_e96251_d_n7;
        locals.var_inv_nvt_dn8 = assign59220_e96251_d_n8;
        locals.var_inv_nvt_dn9 = assign59220_e96251_d_n9;
        locals.var_inv_nvt_dn10 = assign59220_e96251_d_n10;
        locals.var_inv_nvt_dn11 = assign59220_e96251_d_n11;
        locals.var_inv_nvt_rv = 0.0;

        let (assign59230_e96260, assign59230_e96260_d_n3, assign59230_e96260_d_n4, assign59230_e96260_d_n5, assign59230_e96260_d_n6, assign59230_e96260_d_n7, assign59230_e96260_d_n8, assign59230_e96260_d_n9, assign59230_e96260_d_n10, assign59230_e96260_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59230_e96258: f64 = (locals.var_vg * locals.var_inv_nvt);
        (assign59230_e96258, (locals.var_vg * locals.var_inv_nvt_dn3), (locals.var_vg * locals.var_inv_nvt_dn4), (locals.var_vg * locals.var_inv_nvt_dn5), (locals.var_vg * locals.var_inv_nvt_dn6), (locals.var_vg * locals.var_inv_nvt_dn7), ((locals.var_vg_dn8 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn8)), (locals.var_vg * locals.var_inv_nvt_dn9), ((locals.var_vg_dn10 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn10)), (locals.var_vg * locals.var_inv_nvt_dn11),)
    } else {
        (locals.var_vg_1, locals.var_vg_1_dn3, locals.var_vg_1_dn4, locals.var_vg_1_dn5, locals.var_vg_1_dn6, locals.var_vg_1_dn7, locals.var_vg_1_dn8, locals.var_vg_1_dn9, locals.var_vg_1_dn10, locals.var_vg_1_dn11,)
    }
};
        locals.var_vg_1 = assign59230_e96260;
        locals.var_vg_1_dn3 = assign59230_e96260_d_n3;
        locals.var_vg_1_dn4 = assign59230_e96260_d_n4;
        locals.var_vg_1_dn5 = assign59230_e96260_d_n5;
        locals.var_vg_1_dn6 = assign59230_e96260_d_n6;
        locals.var_vg_1_dn7 = assign59230_e96260_d_n7;
        locals.var_vg_1_dn8 = assign59230_e96260_d_n8;
        locals.var_vg_1_dn9 = assign59230_e96260_d_n9;
        locals.var_vg_1_dn10 = assign59230_e96260_d_n10;
        locals.var_vg_1_dn11 = assign59230_e96260_d_n11;
        locals.var_vg_1_rv = 0.0;

        let (assign59240_e96269, assign59240_e96269_d_n3, assign59240_e96269_d_n4, assign59240_e96269_d_n5, assign59240_e96269_d_n6, assign59240_e96269_d_n7, assign59240_e96269_d_n8, assign59240_e96269_d_n9, assign59240_e96269_d_n10, assign59240_e96269_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59240_e96267: f64 = (locals.var_vs * locals.var_inv_nvt);
        (assign59240_e96267, (locals.var_vs * locals.var_inv_nvt_dn3), (locals.var_vs * locals.var_inv_nvt_dn4), (locals.var_vs * locals.var_inv_nvt_dn5), ((locals.var_vs_dn6 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn6)), ((locals.var_vs_dn7 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn7)), (locals.var_vs * locals.var_inv_nvt_dn8), (locals.var_vs * locals.var_inv_nvt_dn9), ((locals.var_vs_dn10 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn10)), (locals.var_vs * locals.var_inv_nvt_dn11),)
    } else {
        (locals.var_vs_1, locals.var_vs_1_dn3, locals.var_vs_1_dn4, locals.var_vs_1_dn5, locals.var_vs_1_dn6, locals.var_vs_1_dn7, locals.var_vs_1_dn8, locals.var_vs_1_dn9, locals.var_vs_1_dn10, locals.var_vs_1_dn11,)
    }
};
        locals.var_vs_1 = assign59240_e96269;
        locals.var_vs_1_dn3 = assign59240_e96269_d_n3;
        locals.var_vs_1_dn4 = assign59240_e96269_d_n4;
        locals.var_vs_1_dn5 = assign59240_e96269_d_n5;
        locals.var_vs_1_dn6 = assign59240_e96269_d_n6;
        locals.var_vs_1_dn7 = assign59240_e96269_d_n7;
        locals.var_vs_1_dn8 = assign59240_e96269_d_n8;
        locals.var_vs_1_dn9 = assign59240_e96269_d_n9;
        locals.var_vs_1_dn10 = assign59240_e96269_d_n10;
        locals.var_vs_1_dn11 = assign59240_e96269_d_n11;
        locals.var_vs_1_rv = 0.0;

        let (assign59250_e96278, assign59250_e96278_d_n3, assign59250_e96278_d_n4, assign59250_e96278_d_n5, assign59250_e96278_d_n6, assign59250_e96278_d_n7, assign59250_e96278_d_n8, assign59250_e96278_d_n9, assign59250_e96278_d_n10, assign59250_e96278_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59250_e96276: f64 = (locals.var_vfb_i * locals.var_inv_nvt);
        (assign59250_e96276, ((locals.var_vfb_i_dn3 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn3)), ((locals.var_vfb_i_dn4 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn4)), ((locals.var_vfb_i_dn5 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn5)), ((locals.var_vfb_i_dn6 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn6)), ((locals.var_vfb_i_dn7 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn7)), ((locals.var_vfb_i_dn8 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn8)), ((locals.var_vfb_i_dn9 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn9)), ((locals.var_vfb_i_dn10 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn10)), ((locals.var_vfb_i_dn11 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn11)),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11,)
    }
};
        locals.var_vfb = assign59250_e96278;
        locals.var_vfb_dn3 = assign59250_e96278_d_n3;
        locals.var_vfb_dn4 = assign59250_e96278_d_n4;
        locals.var_vfb_dn5 = assign59250_e96278_d_n5;
        locals.var_vfb_dn6 = assign59250_e96278_d_n6;
        locals.var_vfb_dn7 = assign59250_e96278_d_n7;
        locals.var_vfb_dn8 = assign59250_e96278_d_n8;
        locals.var_vfb_dn9 = assign59250_e96278_d_n9;
        locals.var_vfb_dn10 = assign59250_e96278_d_n10;
        locals.var_vfb_dn11 = assign59250_e96278_d_n11;
        locals.var_vfb_rv = 0.0;

        let (assign59260_e96292, assign59260_e96292_d_n3, assign59260_e96292_d_n4, assign59260_e96292_d_n5, assign59260_e96292_d_n6, assign59260_e96292_d_n7, assign59260_e96292_d_n8, assign59260_e96292_d_n9, assign59260_e96292_d_n10, assign59260_e96292_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59260_e96286: f64 = (locals.var_etabedge_i * locals.var_vbsx);
        let assign59260_e96287: f64 = (locals.var_eta0edge_t + assign59260_e96286);
        let assign59260_e96288: f64 = (-assign59260_e96287);
        let assign59260_e96290: f64 = (assign59260_e96288 * locals.var_vdsx);
        (assign59260_e96290, (((-(locals.var_eta0edge_t_dn3 + (locals.var_etabedge_i * locals.var_vbsx_dn3))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn3)), (((-(locals.var_eta0edge_t_dn4 + (locals.var_etabedge_i * locals.var_vbsx_dn4))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn4)), (((-(locals.var_eta0edge_t_dn5 + (locals.var_etabedge_i * locals.var_vbsx_dn5))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn5)), (((-(locals.var_eta0edge_t_dn6 + (locals.var_etabedge_i * locals.var_vbsx_dn6))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn6)), (((-(locals.var_eta0edge_t_dn7 + (locals.var_etabedge_i * locals.var_vbsx_dn7))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn7)), (((-(locals.var_eta0edge_t_dn8 + (locals.var_etabedge_i * locals.var_vbsx_dn8))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn8)), (((-(locals.var_eta0edge_t_dn9 + (locals.var_etabedge_i * locals.var_vbsx_dn9))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn9)), (((-(locals.var_eta0edge_t_dn10 + (locals.var_etabedge_i * locals.var_vbsx_dn10))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn10)), (((-(locals.var_eta0edge_t_dn11 + (locals.var_etabedge_i * locals.var_vbsx_dn11))) * locals.var_vdsx) + (assign59260_e96288 * locals.var_vdsx_dn11)),)
    } else {
        (locals.var_dvth_dibl_1, locals.var_dvth_dibl_1_dn3, locals.var_dvth_dibl_1_dn4, locals.var_dvth_dibl_1_dn5, locals.var_dvth_dibl_1_dn6, locals.var_dvth_dibl_1_dn7, locals.var_dvth_dibl_1_dn8, locals.var_dvth_dibl_1_dn9, locals.var_dvth_dibl_1_dn10, locals.var_dvth_dibl_1_dn11,)
    }
};
        locals.var_dvth_dibl_1 = assign59260_e96292;
        locals.var_dvth_dibl_1_dn3 = assign59260_e96292_d_n3;
        locals.var_dvth_dibl_1_dn4 = assign59260_e96292_d_n4;
        locals.var_dvth_dibl_1_dn5 = assign59260_e96292_d_n5;
        locals.var_dvth_dibl_1_dn6 = assign59260_e96292_d_n6;
        locals.var_dvth_dibl_1_dn7 = assign59260_e96292_d_n7;
        locals.var_dvth_dibl_1_dn8 = assign59260_e96292_d_n8;
        locals.var_dvth_dibl_1_dn9 = assign59260_e96292_d_n9;
        locals.var_dvth_dibl_1_dn10 = assign59260_e96292_d_n10;
        locals.var_dvth_dibl_1_dn11 = assign59260_e96292_d_n11;
        locals.var_dvth_dibl_1_rv = 0.0;

        let (assign59270_e96313, assign59270_e96313_d_n3, assign59270_e96313_d_n4, assign59270_e96313_d_n5, assign59270_e96313_d_n6, assign59270_e96313_d_n7, assign59270_e96313_d_n8, assign59270_e96313_d_n9, assign59270_e96313_d_n10, assign59270_e96313_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59270_e96300: f64 = (locals.var_kt1ledge_i / locals.var_leff);
        let assign59270_e96301: f64 = (locals.var_kt1edge_i + assign59270_e96300);
        let assign59270_e96304: f64 = (locals.var_kt2edge_i * locals.var_vbsx);
        let assign59270_e96305: f64 = (assign59270_e96301 + assign59270_e96304);
        let assign59270_e96308: f64 = (locals.var_tratio).powf(locals.var_kt1expedge_i);
        let assign59270_e96310: f64 = (assign59270_e96308 - 1.0);
        let assign59270_e96311: f64 = (assign59270_e96305 * assign59270_e96310);
        (assign59270_e96311, ((locals.var_kt2edge_i * locals.var_vbsx_dn3) * assign59270_e96310), (((locals.var_kt2edge_i * locals.var_vbsx_dn4) * assign59270_e96310) + (assign59270_e96305 * if 0.0 == 0.0 && ((locals.var_kt1expedge_i) as f64).is_finite() && ((locals.var_kt1expedge_i) as f64).fract() == 0.0 { if locals.var_kt1expedge_i == 0.0 { 0.0 } else { (locals.var_kt1expedge_i * ((locals.var_tratio).powf(locals.var_kt1expedge_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign59270_e96308 * (locals.var_kt1expedge_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (((locals.var_kt2edge_i * locals.var_vbsx_dn5) * assign59270_e96310) + (assign59270_e96305 * if 0.0 == 0.0 && ((locals.var_kt1expedge_i) as f64).is_finite() && ((locals.var_kt1expedge_i) as f64).fract() == 0.0 { if locals.var_kt1expedge_i == 0.0 { 0.0 } else { (locals.var_kt1expedge_i * ((locals.var_tratio).powf(locals.var_kt1expedge_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign59270_e96308 * (locals.var_kt1expedge_i * (locals.var_tratio_dn5 / locals.var_tratio))) })), ((locals.var_kt2edge_i * locals.var_vbsx_dn6) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn7) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn8) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn9) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn10) * assign59270_e96310), ((locals.var_kt2edge_i * locals.var_vbsx_dn11) * assign59270_e96310),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11,)
    }
};
        locals.var_dvth_temp = assign59270_e96313;
        locals.var_dvth_temp_dn3 = assign59270_e96313_d_n3;
        locals.var_dvth_temp_dn4 = assign59270_e96313_d_n4;
        locals.var_dvth_temp_dn5 = assign59270_e96313_d_n5;
        locals.var_dvth_temp_dn6 = assign59270_e96313_d_n6;
        locals.var_dvth_temp_dn7 = assign59270_e96313_d_n7;
        locals.var_dvth_temp_dn8 = assign59270_e96313_d_n8;
        locals.var_dvth_temp_dn9 = assign59270_e96313_d_n9;
        locals.var_dvth_temp_dn10 = assign59270_e96313_d_n10;
        locals.var_dvth_temp_dn11 = assign59270_e96313_d_n11;
        locals.var_dvth_temp_rv = 0.0;

        let (assign59280_e96326, assign59280_e96326_d_n3, assign59280_e96326_d_n4, assign59280_e96326_d_n5, assign59280_e96326_d_n6, assign59280_e96326_d_n7, assign59280_e96326_d_n8, assign59280_e96326_d_n9, assign59280_e96326_d_n10, assign59280_e96326_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59280_e96322: f64 = (p.p1264 * locals.var_vbsx);
        let assign59280_e96323: f64 = (1.0 + assign59280_e96322);
        let assign59280_e96324: f64 = (locals.var_litl * assign59280_e96323);
        (assign59280_e96324, (locals.var_litl * (p.p1264 * locals.var_vbsx_dn3)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn4)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn5)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn6)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn7)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn8)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn9)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn10)), (locals.var_litl * (p.p1264 * locals.var_vbsx_dn11)),)
    } else {
        (locals.var_litl_edge, locals.var_litl_edge_dn3, locals.var_litl_edge_dn4, locals.var_litl_edge_dn5, locals.var_litl_edge_dn6, locals.var_litl_edge_dn7, locals.var_litl_edge_dn8, locals.var_litl_edge_dn9, locals.var_litl_edge_dn10, locals.var_litl_edge_dn11,)
    }
};
        locals.var_litl_edge = assign59280_e96326;
        locals.var_litl_edge_dn3 = assign59280_e96326_d_n3;
        locals.var_litl_edge_dn4 = assign59280_e96326_d_n4;
        locals.var_litl_edge_dn5 = assign59280_e96326_d_n5;
        locals.var_litl_edge_dn6 = assign59280_e96326_d_n6;
        locals.var_litl_edge_dn7 = assign59280_e96326_d_n7;
        locals.var_litl_edge_dn8 = assign59280_e96326_d_n8;
        locals.var_litl_edge_dn9 = assign59280_e96326_d_n9;
        locals.var_litl_edge_dn10 = assign59280_e96326_d_n10;
        locals.var_litl_edge_dn11 = assign59280_e96326_d_n11;
        locals.var_litl_edge_rv = 0.0;

        let assign59290_e96329: f64 = if locals.var_litl_edge > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard868 = assign59290_e96329;
        locals.var_guard868_rv = 0.0;

        let (assign59300_e96342, assign59300_e96342_d_n3, assign59300_e96342_d_n4, assign59300_e96342_d_n5, assign59300_e96342_d_n6, assign59300_e96342_d_n7, assign59300_e96342_d_n8, assign59300_e96342_d_n9, assign59300_e96342_d_n10, assign59300_e96342_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 != 0.0)) {
        let assign59300_e96338: f64 = (p.p1263 * locals.var_leff);
        let assign59300_e96340: f64 = (assign59300_e96338 / locals.var_litl_edge);
        (assign59300_e96340, (-((assign59300_e96338 * locals.var_litl_edge_dn3) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn4) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn5) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn6) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn7) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn8) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn9) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn10) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign59300_e96338 * locals.var_litl_edge_dn11) / (locals.var_litl_edge * locals.var_litl_edge))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign59300_e96342;
        locals.var_t0_dn3 = assign59300_e96342_d_n3;
        locals.var_t0_dn4 = assign59300_e96342_d_n4;
        locals.var_t0_dn5 = assign59300_e96342_d_n5;
        locals.var_t0_dn6 = assign59300_e96342_d_n6;
        locals.var_t0_dn7 = assign59300_e96342_d_n7;
        locals.var_t0_dn8 = assign59300_e96342_d_n8;
        locals.var_t0_dn9 = assign59300_e96342_d_n9;
        locals.var_t0_dn10 = assign59300_e96342_d_n10;
        locals.var_t0_dn11 = assign59300_e96342_d_n11;
        locals.var_t0_rv = 0.0;

        let assign59310_e96345: f64 = if locals.var_t0 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard869 = assign59310_e96345;
        locals.var_guard869_rv = 0.0;

        let (assign59320_e96363, assign59320_e96363_d_n3, assign59320_e96363_d_n4, assign59320_e96363_d_n5, assign59320_e96363_d_n6, assign59320_e96363_d_n7, assign59320_e96363_d_n8, assign59320_e96363_d_n9, assign59320_e96363_d_n10, assign59320_e96363_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_guard869 != 0.0)) {
        let assign59320_e96356: f64 = (0.5 * p.p1262);
        let assign59320_e96358: f64 = (locals.var_t0).cosh();
        let assign59320_e96360: f64 = (assign59320_e96358 - 1.0);
        let assign59320_e96361: f64 = (assign59320_e96356 / assign59320_e96360);
        (assign59320_e96361, (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn3)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn4)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn5)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn6)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn7)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn8)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn9)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn10)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((locals.var_t0).sinh() * locals.var_t0_dn11)) / (assign59320_e96360 * assign59320_e96360))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11,)
    }
};
        locals.var_theta_sce_edge = assign59320_e96363;
        locals.var_theta_sce_edge_dn3 = assign59320_e96363_d_n3;
        locals.var_theta_sce_edge_dn4 = assign59320_e96363_d_n4;
        locals.var_theta_sce_edge_dn5 = assign59320_e96363_d_n5;
        locals.var_theta_sce_edge_dn6 = assign59320_e96363_d_n6;
        locals.var_theta_sce_edge_dn7 = assign59320_e96363_d_n7;
        locals.var_theta_sce_edge_dn8 = assign59320_e96363_d_n8;
        locals.var_theta_sce_edge_dn9 = assign59320_e96363_d_n9;
        locals.var_theta_sce_edge_dn10 = assign59320_e96363_d_n10;
        locals.var_theta_sce_edge_dn11 = assign59320_e96363_d_n11;
        locals.var_theta_sce_edge_rv = 0.0;

        let (assign59330_e96379, assign59330_e96379_d_n3, assign59330_e96379_d_n4, assign59330_e96379_d_n5, assign59330_e96379_d_n6, assign59330_e96379_d_n7, assign59330_e96379_d_n8, assign59330_e96379_d_n9, assign59330_e96379_d_n10, assign59330_e96379_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 != 0.0)) && (locals.var_guard869 == 0.0)) {
        let assign59330_e96375: f64 = (-locals.var_t0);
        let assign59330_e96376: f64 = { let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign59330_e96377: f64 = (p.p1262 * assign59330_e96376);
        (assign59330_e96377, (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn3))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11,)
    }
};
        locals.var_theta_sce_edge = assign59330_e96379;
        locals.var_theta_sce_edge_dn3 = assign59330_e96379_d_n3;
        locals.var_theta_sce_edge_dn4 = assign59330_e96379_d_n4;
        locals.var_theta_sce_edge_dn5 = assign59330_e96379_d_n5;
        locals.var_theta_sce_edge_dn6 = assign59330_e96379_d_n6;
        locals.var_theta_sce_edge_dn7 = assign59330_e96379_d_n7;
        locals.var_theta_sce_edge_dn8 = assign59330_e96379_d_n8;
        locals.var_theta_sce_edge_dn9 = assign59330_e96379_d_n9;
        locals.var_theta_sce_edge_dn10 = assign59330_e96379_d_n10;
        locals.var_theta_sce_edge_dn11 = assign59330_e96379_d_n11;
        locals.var_theta_sce_edge_rv = 0.0;

        let (assign59340_e96389, assign59340_e96389_d_n3, assign59340_e96389_d_n4, assign59340_e96389_d_n5, assign59340_e96389_d_n6, assign59340_e96389_d_n7, assign59340_e96389_d_n8, assign59340_e96389_d_n9, assign59340_e96389_d_n10, assign59340_e96389_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard868 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11,)
    }
};
        locals.var_theta_sce_edge = assign59340_e96389;
        locals.var_theta_sce_edge_dn3 = assign59340_e96389_d_n3;
        locals.var_theta_sce_edge_dn4 = assign59340_e96389_d_n4;
        locals.var_theta_sce_edge_dn5 = assign59340_e96389_d_n5;
        locals.var_theta_sce_edge_dn6 = assign59340_e96389_d_n6;
        locals.var_theta_sce_edge_dn7 = assign59340_e96389_d_n7;
        locals.var_theta_sce_edge_dn8 = assign59340_e96389_d_n8;
        locals.var_theta_sce_edge_dn9 = assign59340_e96389_d_n9;
        locals.var_theta_sce_edge_dn10 = assign59340_e96389_d_n10;
        locals.var_theta_sce_edge_dn11 = assign59340_e96389_d_n11;
        locals.var_theta_sce_edge_rv = 0.0;

        let (assign59350_e96400, assign59350_e96400_d_n3, assign59350_e96400_d_n4, assign59350_e96400_d_n5, assign59350_e96400_d_n6, assign59350_e96400_d_n7, assign59350_e96400_d_n8, assign59350_e96400_d_n9, assign59350_e96400_d_n10, assign59350_e96400_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59350_e96397: f64 = (locals.var_vbi_edge - locals.var_phist);
        let assign59350_e96398: f64 = (locals.var_theta_sce_edge * assign59350_e96397);
        (assign59350_e96398, ((locals.var_theta_sce_edge_dn3 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn3 - locals.var_phist_dn3))), ((locals.var_theta_sce_edge_dn4 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn4 - locals.var_phist_dn4))), ((locals.var_theta_sce_edge_dn5 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn5 - locals.var_phist_dn5))), ((locals.var_theta_sce_edge_dn6 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn6 - locals.var_phist_dn6))), ((locals.var_theta_sce_edge_dn7 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn7 - locals.var_phist_dn7))), ((locals.var_theta_sce_edge_dn8 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn8 - locals.var_phist_dn8))), ((locals.var_theta_sce_edge_dn9 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn9 - locals.var_phist_dn9))), ((locals.var_theta_sce_edge_dn10 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn10 - locals.var_phist_dn10))), ((locals.var_theta_sce_edge_dn11 * assign59350_e96397) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn11 - locals.var_phist_dn11))),)
    } else {
        (locals.var_dvth_sce, locals.var_dvth_sce_dn3, locals.var_dvth_sce_dn4, locals.var_dvth_sce_dn5, locals.var_dvth_sce_dn6, locals.var_dvth_sce_dn7, locals.var_dvth_sce_dn8, locals.var_dvth_sce_dn9, locals.var_dvth_sce_dn10, locals.var_dvth_sce_dn11,)
    }
};
        locals.var_dvth_sce = assign59350_e96400;
        locals.var_dvth_sce_dn3 = assign59350_e96400_d_n3;
        locals.var_dvth_sce_dn4 = assign59350_e96400_d_n4;
        locals.var_dvth_sce_dn5 = assign59350_e96400_d_n5;
        locals.var_dvth_sce_dn6 = assign59350_e96400_d_n6;
        locals.var_dvth_sce_dn7 = assign59350_e96400_d_n7;
        locals.var_dvth_sce_dn8 = assign59350_e96400_d_n8;
        locals.var_dvth_sce_dn9 = assign59350_e96400_d_n9;
        locals.var_dvth_sce_dn10 = assign59350_e96400_d_n10;
        locals.var_dvth_sce_dn11 = assign59350_e96400_d_n11;
        locals.var_dvth_sce_rv = 0.0;

        let (assign59360_e96421, assign59360_e96421_d_n3, assign59360_e96421_d_n4, assign59360_e96421_d_n5, assign59360_e96421_d_n6, assign59360_e96421_d_n7, assign59360_e96421_d_n8, assign59360_e96421_d_n9, assign59360_e96421_d_n10, assign59360_e96421_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59360_e96407: f64 = (locals.var_dvth_dibl_1 - locals.var_dvth_temp);
        let assign59360_e96409: f64 = (assign59360_e96407 + locals.var_dvth_sce);
        let assign59360_e96411: f64 = (assign59360_e96409 + p.p1151);
        let assign59360_e96413: f64 = (assign59360_e96411 + locals.var_vth0_stress_edge);
        let assign59360_e96416: f64 = (locals.var_k2edge_i * locals.var_vbsx);
        let assign59360_e96417: f64 = (assign59360_e96413 - assign59360_e96416);
        let assign59360_e96419: f64 = (assign59360_e96417 + locals.var_vth0_well_edge);
        (assign59360_e96419, (((((locals.var_dvth_dibl_1_dn3 - locals.var_dvth_temp_dn3) + locals.var_dvth_sce_dn3) + locals.var_vth0_stress_edge_dn3) - ((locals.var_k2edge_i_dn3 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn3))) + locals.var_vth0_well_edge_dn3), (((((locals.var_dvth_dibl_1_dn4 - locals.var_dvth_temp_dn4) + locals.var_dvth_sce_dn4) + locals.var_vth0_stress_edge_dn4) - ((locals.var_k2edge_i_dn4 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn4))) + locals.var_vth0_well_edge_dn4), (((((locals.var_dvth_dibl_1_dn5 - locals.var_dvth_temp_dn5) + locals.var_dvth_sce_dn5) + locals.var_vth0_stress_edge_dn5) - ((locals.var_k2edge_i_dn5 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn5))) + locals.var_vth0_well_edge_dn5), (((((locals.var_dvth_dibl_1_dn6 - locals.var_dvth_temp_dn6) + locals.var_dvth_sce_dn6) + locals.var_vth0_stress_edge_dn6) - ((locals.var_k2edge_i_dn6 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn6))) + locals.var_vth0_well_edge_dn6), (((((locals.var_dvth_dibl_1_dn7 - locals.var_dvth_temp_dn7) + locals.var_dvth_sce_dn7) + locals.var_vth0_stress_edge_dn7) - ((locals.var_k2edge_i_dn7 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn7))) + locals.var_vth0_well_edge_dn7), (((((locals.var_dvth_dibl_1_dn8 - locals.var_dvth_temp_dn8) + locals.var_dvth_sce_dn8) + locals.var_vth0_stress_edge_dn8) - ((locals.var_k2edge_i_dn8 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn8))) + locals.var_vth0_well_edge_dn8), (((((locals.var_dvth_dibl_1_dn9 - locals.var_dvth_temp_dn9) + locals.var_dvth_sce_dn9) + locals.var_vth0_stress_edge_dn9) - ((locals.var_k2edge_i_dn9 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn9))) + locals.var_vth0_well_edge_dn9), (((((locals.var_dvth_dibl_1_dn10 - locals.var_dvth_temp_dn10) + locals.var_dvth_sce_dn10) + locals.var_vth0_stress_edge_dn10) - ((locals.var_k2edge_i_dn10 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn10))) + locals.var_vth0_well_edge_dn10), (((((locals.var_dvth_dibl_1_dn11 - locals.var_dvth_temp_dn11) + locals.var_dvth_sce_dn11) + locals.var_vth0_stress_edge_dn11) - ((locals.var_k2edge_i_dn11 * locals.var_vbsx) + (locals.var_k2edge_i * locals.var_vbsx_dn11))) + locals.var_vth0_well_edge_dn11),)
    } else {
        (locals.var_vth_shift, locals.var_vth_shift_dn3, locals.var_vth_shift_dn4, locals.var_vth_shift_dn5, locals.var_vth_shift_dn6, locals.var_vth_shift_dn7, locals.var_vth_shift_dn8, locals.var_vth_shift_dn9, locals.var_vth_shift_dn10, locals.var_vth_shift_dn11,)
    }
};
        locals.var_vth_shift = assign59360_e96421;
        locals.var_vth_shift_dn3 = assign59360_e96421_d_n3;
        locals.var_vth_shift_dn4 = assign59360_e96421_d_n4;
        locals.var_vth_shift_dn5 = assign59360_e96421_d_n5;
        locals.var_vth_shift_dn6 = assign59360_e96421_d_n6;
        locals.var_vth_shift_dn7 = assign59360_e96421_d_n7;
        locals.var_vth_shift_dn8 = assign59360_e96421_d_n8;
        locals.var_vth_shift_dn9 = assign59360_e96421_d_n9;
        locals.var_vth_shift_dn10 = assign59360_e96421_d_n10;
        locals.var_vth_shift_dn11 = assign59360_e96421_d_n11;
        locals.var_vth_shift_rv = 0.0;

        let (assign59370_e96434, assign59370_e96434_d_n3, assign59370_e96434_d_n4, assign59370_e96434_d_n5, assign59370_e96434_d_n6, assign59370_e96434_d_n7, assign59370_e96434_d_n8, assign59370_e96434_d_n9, assign59370_e96434_d_n10, assign59370_e96434_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59370_e96428: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign59370_e96431: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign59370_e96432: f64 = (assign59370_e96428 - assign59370_e96431);
        (assign59370_e96432, ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3))), ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4))), ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5))), ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6))), ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7))), ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8))), ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9))), ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10))), ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11))),)
    } else {
        (locals.var_vgfb, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11,)
    }
};
        locals.var_vgfb = assign59370_e96434;
        locals.var_vgfb_dn3 = assign59370_e96434_d_n3;
        locals.var_vgfb_dn4 = assign59370_e96434_d_n4;
        locals.var_vgfb_dn5 = assign59370_e96434_d_n5;
        locals.var_vgfb_dn6 = assign59370_e96434_d_n6;
        locals.var_vgfb_dn7 = assign59370_e96434_d_n7;
        locals.var_vgfb_dn8 = assign59370_e96434_d_n8;
        locals.var_vgfb_dn9 = assign59370_e96434_d_n9;
        locals.var_vgfb_dn10 = assign59370_e96434_d_n10;
        locals.var_vgfb_dn11 = assign59370_e96434_d_n11;
        locals.var_vgfb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_207(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign59380_e96450,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59380_e96444: f64 = (-p.p1150);
        let assign59380_e96445: f64 = (locals.var_leff).powf(assign59380_e96444);
        let assign59380_e96446: f64 = (p.p1149 * assign59380_e96445);
        let assign59380_e96447: f64 = (1.0 + assign59380_e96446);
        let assign59380_e96448: f64 = (p.p1148 * assign59380_e96447);
        (assign59380_e96448,)
    } else {
        (locals.var_dgammaedge_i,)
    }
};
        locals.var_dgammaedge_i = assign59380_e96450;
        locals.var_dgammaedge_i_rv = 0.0;

        let (assign59390_e96468, assign59390_e96468_d_n3, assign59390_e96468_d_n4, assign59390_e96468_d_n5, assign59390_e96468_d_n6, assign59390_e96468_d_n7, assign59390_e96468_d_n8, assign59390_e96468_d_n9, assign59390_e96468_d_n10, assign59390_e96468_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59390_e96457: f64 = (2.0 * 1.602176462e-19);
        let assign59390_e96459: f64 = (assign59390_e96457 * locals.var_epssi);
        let assign59390_e96461: f64 = (assign59390_e96459 * locals.var_ndepedge_i);
        let assign59390_e96463: f64 = (assign59390_e96461 * locals.var_inv_nvt);
        let assign59390_e96464: f64 = (assign59390_e96463).sqrt();
        let assign59390_e96466: f64 = (assign59390_e96464 / locals.var_cox);
        (assign59390_e96466, (((assign59390_e96461 * locals.var_inv_nvt_dn3) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn4) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn5) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn6) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn7) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn8) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn9) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn10) / (2.0 * assign59390_e96464)) / locals.var_cox), (((assign59390_e96461 * locals.var_inv_nvt_dn11) / (2.0 * assign59390_e96464)) / locals.var_cox),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11,)
    }
};
        locals.var_gam_edge = assign59390_e96468;
        locals.var_gam_edge_dn3 = assign59390_e96468_d_n3;
        locals.var_gam_edge_dn4 = assign59390_e96468_d_n4;
        locals.var_gam_edge_dn5 = assign59390_e96468_d_n5;
        locals.var_gam_edge_dn6 = assign59390_e96468_d_n6;
        locals.var_gam_edge_dn7 = assign59390_e96468_d_n7;
        locals.var_gam_edge_dn8 = assign59390_e96468_d_n8;
        locals.var_gam_edge_dn9 = assign59390_e96468_d_n9;
        locals.var_gam_edge_dn10 = assign59390_e96468_d_n10;
        locals.var_gam_edge_dn11 = assign59390_e96468_d_n11;
        locals.var_gam_edge_rv = 0.0;

        let (assign59400_e96479, assign59400_e96479_d_n3, assign59400_e96479_d_n4, assign59400_e96479_d_n5, assign59400_e96479_d_n6, assign59400_e96479_d_n7, assign59400_e96479_d_n8, assign59400_e96479_d_n9, assign59400_e96479_d_n10, assign59400_e96479_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59400_e96476: f64 = (1.0 + locals.var_dgammaedge_i);
        let assign59400_e96477: f64 = (locals.var_gam_edge * assign59400_e96476);
        (assign59400_e96477, (locals.var_gam_edge_dn3 * assign59400_e96476), (locals.var_gam_edge_dn4 * assign59400_e96476), (locals.var_gam_edge_dn5 * assign59400_e96476), (locals.var_gam_edge_dn6 * assign59400_e96476), (locals.var_gam_edge_dn7 * assign59400_e96476), (locals.var_gam_edge_dn8 * assign59400_e96476), (locals.var_gam_edge_dn9 * assign59400_e96476), (locals.var_gam_edge_dn10 * assign59400_e96476), (locals.var_gam_edge_dn11 * assign59400_e96476),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11,)
    }
};
        locals.var_gam_edge = assign59400_e96479;
        locals.var_gam_edge_dn3 = assign59400_e96479_d_n3;
        locals.var_gam_edge_dn4 = assign59400_e96479_d_n4;
        locals.var_gam_edge_dn5 = assign59400_e96479_d_n5;
        locals.var_gam_edge_dn6 = assign59400_e96479_d_n6;
        locals.var_gam_edge_dn7 = assign59400_e96479_d_n7;
        locals.var_gam_edge_dn8 = assign59400_e96479_d_n8;
        locals.var_gam_edge_dn9 = assign59400_e96479_d_n9;
        locals.var_gam_edge_dn10 = assign59400_e96479_d_n10;
        locals.var_gam_edge_dn11 = assign59400_e96479_d_n11;
        locals.var_gam_edge_rv = 0.0;

        let (assign59410_e96488, assign59410_e96488_d_n3, assign59410_e96488_d_n4, assign59410_e96488_d_n5, assign59410_e96488_d_n6, assign59410_e96488_d_n7, assign59410_e96488_d_n8, assign59410_e96488_d_n9, assign59410_e96488_d_n10, assign59410_e96488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59410_e96486: f64 = (locals.var_phib_edge / locals.var_n);
        (assign59410_e96486, (((locals.var_phib_edge_dn3 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn3)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn4 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn4)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn5 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn5)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn6 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn6)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn7 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn7)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn8 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn8)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn9 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn9)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn10 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn10)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn11 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn11)) / (locals.var_n * locals.var_n)),)
    } else {
        (locals.var_phib_n_edge, locals.var_phib_n_edge_dn3, locals.var_phib_n_edge_dn4, locals.var_phib_n_edge_dn5, locals.var_phib_n_edge_dn6, locals.var_phib_n_edge_dn7, locals.var_phib_n_edge_dn8, locals.var_phib_n_edge_dn9, locals.var_phib_n_edge_dn10, locals.var_phib_n_edge_dn11,)
    }
};
        locals.var_phib_n_edge = assign59410_e96488;
        locals.var_phib_n_edge_dn3 = assign59410_e96488_d_n3;
        locals.var_phib_n_edge_dn4 = assign59410_e96488_d_n4;
        locals.var_phib_n_edge_dn5 = assign59410_e96488_d_n5;
        locals.var_phib_n_edge_dn6 = assign59410_e96488_d_n6;
        locals.var_phib_n_edge_dn7 = assign59410_e96488_d_n7;
        locals.var_phib_n_edge_dn8 = assign59410_e96488_d_n8;
        locals.var_phib_n_edge_dn9 = assign59410_e96488_d_n9;
        locals.var_phib_n_edge_dn10 = assign59410_e96488_d_n10;
        locals.var_phib_n_edge_dn11 = assign59410_e96488_d_n11;
        locals.var_phib_n_edge_rv = 0.0;

        let (assign59420_e96497, assign59420_e96497_d_n3, assign59420_e96497_d_n4, assign59420_e96497_d_n5, assign59420_e96497_d_n6, assign59420_e96497_d_n7, assign59420_e96497_d_n8, assign59420_e96497_d_n9, assign59420_e96497_d_n10, assign59420_e96497_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59420_e96495: f64 = 1.0;
        (assign59420_e96495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59420_e96497;
        locals.var_t1_dn3 = assign59420_e96497_d_n3;
        locals.var_t1_dn4 = assign59420_e96497_d_n4;
        locals.var_t1_dn5 = assign59420_e96497_d_n5;
        locals.var_t1_dn6 = assign59420_e96497_d_n6;
        locals.var_t1_dn7 = assign59420_e96497_d_n7;
        locals.var_t1_dn8 = assign59420_e96497_d_n8;
        locals.var_t1_dn9 = assign59420_e96497_d_n9;
        locals.var_t1_dn10 = assign59420_e96497_d_n10;
        locals.var_t1_dn11 = assign59420_e96497_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59430_e96506, assign59430_e96506_d_n3, assign59430_e96506_d_n4, assign59430_e96506_d_n5, assign59430_e96506_d_n6, assign59430_e96506_d_n7, assign59430_e96506_d_n8, assign59430_e96506_d_n9, assign59430_e96506_d_n10, assign59430_e96506_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59430_e96504: f64 = (locals.var_vgfb / locals.var_t1);
        (assign59430_e96504, (((locals.var_vgfb_dn3 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn4 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn5 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn6 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn7 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn8 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn9 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn10 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn11 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11,)
    }
};
        locals.var_vgfbpd = assign59430_e96506;
        locals.var_vgfbpd_dn3 = assign59430_e96506_d_n3;
        locals.var_vgfbpd_dn4 = assign59430_e96506_d_n4;
        locals.var_vgfbpd_dn5 = assign59430_e96506_d_n5;
        locals.var_vgfbpd_dn6 = assign59430_e96506_d_n6;
        locals.var_vgfbpd_dn7 = assign59430_e96506_d_n7;
        locals.var_vgfbpd_dn8 = assign59430_e96506_d_n8;
        locals.var_vgfbpd_dn9 = assign59430_e96506_d_n9;
        locals.var_vgfbpd_dn10 = assign59430_e96506_d_n10;
        locals.var_vgfbpd_dn11 = assign59430_e96506_d_n11;
        locals.var_vgfbpd_rv = 0.0;

        let (assign59440_e96515, assign59440_e96515_d_n3, assign59440_e96515_d_n4, assign59440_e96515_d_n5, assign59440_e96515_d_n6, assign59440_e96515_d_n7, assign59440_e96515_d_n8, assign59440_e96515_d_n9, assign59440_e96515_d_n10, assign59440_e96515_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59440_e96513: f64 = (locals.var_gam_edge / locals.var_t1);
        (assign59440_e96513, (((locals.var_gam_edge_dn3 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn4 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn5 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn6 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn7 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn8 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn9 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn10 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn11 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11,)
    }
};
        locals.var_gammapd = assign59440_e96515;
        locals.var_gammapd_dn3 = assign59440_e96515_d_n3;
        locals.var_gammapd_dn4 = assign59440_e96515_d_n4;
        locals.var_gammapd_dn5 = assign59440_e96515_d_n5;
        locals.var_gammapd_dn6 = assign59440_e96515_d_n6;
        locals.var_gammapd_dn7 = assign59440_e96515_d_n7;
        locals.var_gammapd_dn8 = assign59440_e96515_d_n8;
        locals.var_gammapd_dn9 = assign59440_e96515_d_n9;
        locals.var_gammapd_dn10 = assign59440_e96515_d_n10;
        locals.var_gammapd_dn11 = assign59440_e96515_d_n11;
        locals.var_gammapd_rv = 0.0;

        let (assign59450_e96532, assign59450_e96532_d_n3, assign59450_e96532_d_n4, assign59450_e96532_d_n5, assign59450_e96532_d_n6, assign59450_e96532_d_n7, assign59450_e96532_d_n8, assign59450_e96532_d_n9, assign59450_e96532_d_n10, assign59450_e96532_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59450_e96522: f64 = (0.5 * locals.var_vgfbpd);
        let assign59450_e96527: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign59450_e96528: f64 = (1.0 + assign59450_e96527);
        let assign59450_e96529: f64 = (3.0 * assign59450_e96528);
        let assign59450_e96530: f64 = (assign59450_e96522 - assign59450_e96529);
        (assign59450_e96530, ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59450_e96532;
        locals.var_t1_dn3 = assign59450_e96532_d_n3;
        locals.var_t1_dn4 = assign59450_e96532_d_n4;
        locals.var_t1_dn5 = assign59450_e96532_d_n5;
        locals.var_t1_dn6 = assign59450_e96532_d_n6;
        locals.var_t1_dn7 = assign59450_e96532_d_n7;
        locals.var_t1_dn8 = assign59450_e96532_d_n8;
        locals.var_t1_dn9 = assign59450_e96532_d_n9;
        locals.var_t1_dn10 = assign59450_e96532_d_n10;
        locals.var_t1_dn11 = assign59450_e96532_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59460_e96548, assign59460_e96548_d_n3, assign59460_e96548_d_n4, assign59460_e96548_d_n5, assign59460_e96548_d_n6, assign59460_e96548_d_n7, assign59460_e96548_d_n8, assign59460_e96548_d_n9, assign59460_e96548_d_n10, assign59460_e96548_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59460_e96540: f64 = (locals.var_t1 * locals.var_t1);
        let assign59460_e96543: f64 = (6.0 * locals.var_vgfbpd);
        let assign59460_e96544: f64 = (assign59460_e96540 + assign59460_e96543);
        let assign59460_e96545: f64 = (assign59460_e96544).sqrt();
        let assign59460_e96546: f64 = (locals.var_t1 + assign59460_e96545);
        (assign59460_e96546, (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign59460_e96545))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign59460_e96545))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59460_e96548;
        locals.var_t2_dn3 = assign59460_e96548_d_n3;
        locals.var_t2_dn4 = assign59460_e96548_d_n4;
        locals.var_t2_dn5 = assign59460_e96548_d_n5;
        locals.var_t2_dn6 = assign59460_e96548_d_n6;
        locals.var_t2_dn7 = assign59460_e96548_d_n7;
        locals.var_t2_dn8 = assign59460_e96548_d_n8;
        locals.var_t2_dn9 = assign59460_e96548_d_n9;
        locals.var_t2_dn10 = assign59460_e96548_d_n10;
        locals.var_t2_dn11 = assign59460_e96548_d_n11;
        locals.var_t2_rv = 0.0;

        let assign59470_e96551: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard870 = assign59470_e96551;
        locals.var_guard870_rv = 0.0;

        let (assign59480_e96564, assign59480_e96564_d_n3, assign59480_e96564_d_n4, assign59480_e96564_d_n5, assign59480_e96564_d_n6, assign59480_e96564_d_n7, assign59480_e96564_d_n8, assign59480_e96564_d_n9, assign59480_e96564_d_n10, assign59480_e96564_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign59480_e96560: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign59480_e96562: f64 = (assign59480_e96560 / locals.var_gammapd);
        (assign59480_e96562, ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign59480_e96560 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59480_e96564;
        locals.var_t3_dn3 = assign59480_e96564_d_n3;
        locals.var_t3_dn4 = assign59480_e96564_d_n4;
        locals.var_t3_dn5 = assign59480_e96564_d_n5;
        locals.var_t3_dn6 = assign59480_e96564_d_n6;
        locals.var_t3_dn7 = assign59480_e96564_d_n7;
        locals.var_t3_dn8 = assign59480_e96564_d_n8;
        locals.var_t3_dn9 = assign59480_e96564_d_n9;
        locals.var_t3_dn10 = assign59480_e96564_d_n10;
        locals.var_t3_dn11 = assign59480_e96564_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59490_e96583, assign59490_e96583_d_n3, assign59490_e96583_d_n4, assign59490_e96583_d_n5, assign59490_e96583_d_n6, assign59490_e96583_d_n7, assign59490_e96583_d_n8, assign59490_e96583_d_n9, assign59490_e96583_d_n10, assign59490_e96583_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 != 0.0)) {
        let assign59490_e96573: f64 = (1.0 - locals.var_t2);
        let assign59490_e96576: f64 = (locals.var_t3 * locals.var_t3);
        let assign59490_e96577: f64 = (assign59490_e96573 + assign59490_e96576);
        let assign59490_e96579: f64 = (assign59490_e96577).max(1e-38);
        let assign59490_e96580: f64 = (assign59490_e96579).ln();
        let assign59490_e96581: f64 = (-assign59490_e96580);
        (assign59490_e96581, (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign59490_e96579)),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign59490_e96583;
        locals.var_psip_dn3 = assign59490_e96583_d_n3;
        locals.var_psip_dn4 = assign59490_e96583_d_n4;
        locals.var_psip_dn5 = assign59490_e96583_d_n5;
        locals.var_psip_dn6 = assign59490_e96583_d_n6;
        locals.var_psip_dn7 = assign59490_e96583_d_n7;
        locals.var_psip_dn8 = assign59490_e96583_d_n8;
        locals.var_psip_dn9 = assign59490_e96583_d_n9;
        locals.var_psip_dn10 = assign59490_e96583_d_n10;
        locals.var_psip_dn11 = assign59490_e96583_d_n11;
        locals.var_psip_rv = 0.0;

        let (assign59500_e96595, assign59500_e96595_d_n3, assign59500_e96595_d_n4, assign59500_e96595_d_n5, assign59500_e96595_d_n6, assign59500_e96595_d_n7, assign59500_e96595_d_n8, assign59500_e96595_d_n9, assign59500_e96595_d_n10, assign59500_e96595_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59500_e96592: f64 = (-locals.var_t2);
        let assign59500_e96593: f64 = { let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59500_e96593, ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59500_e96595;
        locals.var_t3_dn3 = assign59500_e96595_d_n3;
        locals.var_t3_dn4 = assign59500_e96595_d_n4;
        locals.var_t3_dn5 = assign59500_e96595_d_n5;
        locals.var_t3_dn6 = assign59500_e96595_d_n6;
        locals.var_t3_dn7 = assign59500_e96595_d_n7;
        locals.var_t3_dn8 = assign59500_e96595_d_n8;
        locals.var_t3_dn9 = assign59500_e96595_d_n9;
        locals.var_t3_dn10 = assign59500_e96595_d_n10;
        locals.var_t3_dn11 = assign59500_e96595_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59510_e96607, assign59510_e96607_d_n3, assign59510_e96607_d_n4, assign59510_e96607_d_n5, assign59510_e96607_d_n6, assign59510_e96607_d_n7, assign59510_e96607_d_n8, assign59510_e96607_d_n9, assign59510_e96607_d_n10, assign59510_e96607_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59510_e96605: f64 = (0.5 * locals.var_gammapd);
        (assign59510_e96605, (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59510_e96607;
        locals.var_t1_dn3 = assign59510_e96607_d_n3;
        locals.var_t1_dn4 = assign59510_e96607_d_n4;
        locals.var_t1_dn5 = assign59510_e96607_d_n5;
        locals.var_t1_dn6 = assign59510_e96607_d_n6;
        locals.var_t1_dn7 = assign59510_e96607_d_n7;
        locals.var_t1_dn8 = assign59510_e96607_d_n8;
        locals.var_t1_dn9 = assign59510_e96607_d_n9;
        locals.var_t1_dn10 = assign59510_e96607_d_n10;
        locals.var_t1_dn11 = assign59510_e96607_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59520_e96628, assign59520_e96628_d_n3, assign59520_e96628_d_n4, assign59520_e96628_d_n5, assign59520_e96628_d_n6, assign59520_e96628_d_n7, assign59520_e96628_d_n8, assign59520_e96628_d_n9, assign59520_e96628_d_n10, assign59520_e96628_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59520_e96617: f64 = (locals.var_vgfbpd - 1.0);
        let assign59520_e96619: f64 = (assign59520_e96617 + locals.var_t3);
        let assign59520_e96622: f64 = (locals.var_t1 * locals.var_t1);
        let assign59520_e96623: f64 = (assign59520_e96619 + assign59520_e96622);
        let assign59520_e96624: f64 = (assign59520_e96623).sqrt();
        let assign59520_e96626: f64 = (assign59520_e96624 - locals.var_t1);
        (assign59520_e96626, ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign59520_e96624)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59520_e96628;
        locals.var_t2_dn3 = assign59520_e96628_d_n3;
        locals.var_t2_dn4 = assign59520_e96628_d_n4;
        locals.var_t2_dn5 = assign59520_e96628_d_n5;
        locals.var_t2_dn6 = assign59520_e96628_d_n6;
        locals.var_t2_dn7 = assign59520_e96628_d_n7;
        locals.var_t2_dn8 = assign59520_e96628_d_n8;
        locals.var_t2_dn9 = assign59520_e96628_d_n9;
        locals.var_t2_dn10 = assign59520_e96628_d_n10;
        locals.var_t2_dn11 = assign59520_e96628_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign59530_e96644, assign59530_e96644_d_n3, assign59530_e96644_d_n4, assign59530_e96644_d_n5, assign59530_e96644_d_n6, assign59530_e96644_d_n7, assign59530_e96644_d_n8, assign59530_e96644_d_n9, assign59530_e96644_d_n10, assign59530_e96644_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard870 == 0.0)) {
        let assign59530_e96638: f64 = (locals.var_t2 * locals.var_t2);
        let assign59530_e96640: f64 = (assign59530_e96638 + 1.0);
        let assign59530_e96642: f64 = (assign59530_e96640 - locals.var_t3);
        (assign59530_e96642, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11),)
    } else {
        (locals.var_psip, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11,)
    }
};
        locals.var_psip = assign59530_e96644;
        locals.var_psip_dn3 = assign59530_e96644_d_n3;
        locals.var_psip_dn4 = assign59530_e96644_d_n4;
        locals.var_psip_dn5 = assign59530_e96644_d_n5;
        locals.var_psip_dn6 = assign59530_e96644_d_n6;
        locals.var_psip_dn7 = assign59530_e96644_d_n7;
        locals.var_psip_dn8 = assign59530_e96644_d_n8;
        locals.var_psip_dn9 = assign59530_e96644_d_n9;
        locals.var_psip_dn10 = assign59530_e96644_d_n10;
        locals.var_psip_dn11 = assign59530_e96644_d_n11;
        locals.var_psip_rv = 0.0;

        let (assign59540_e96670, assign59540_e96670_d_n3, assign59540_e96670_d_n4, assign59540_e96670_d_n5, assign59540_e96670_d_n6, assign59540_e96670_d_n7, assign59540_e96670_d_n8, assign59540_e96670_d_n9, assign59540_e96670_d_n10, assign59540_e96670_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59540_e96652: f64 = (locals.var_psip + 1.0);
        let assign59540_e96655: f64 = (locals.var_psip - 1.0);
        let assign59540_e96658: f64 = (locals.var_psip - 1.0);
        let assign59540_e96659: f64 = (assign59540_e96655 * assign59540_e96658);
        let assign59540_e96662: f64 = (0.25 * 2.0);
        let assign59540_e96664: f64 = (assign59540_e96662 * 2.0);
        let assign59540_e96665: f64 = (assign59540_e96659 + assign59540_e96664);
        let assign59540_e96666: f64 = (assign59540_e96665).sqrt();
        let assign59540_e96667: f64 = (assign59540_e96652 + assign59540_e96666);
        let assign59540_e96668: f64 = (0.5 * assign59540_e96667);
        (assign59540_e96668, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn3)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn4)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn5)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn6)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn7)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn8)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn9)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn10)) / (2.0 * assign59540_e96666)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign59540_e96658) + (assign59540_e96655 * locals.var_psip_dn11)) / (2.0 * assign59540_e96666)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59540_e96670;
        locals.var_t8_dn3 = assign59540_e96670_d_n3;
        locals.var_t8_dn4 = assign59540_e96670_d_n4;
        locals.var_t8_dn5 = assign59540_e96670_d_n5;
        locals.var_t8_dn6 = assign59540_e96670_d_n6;
        locals.var_t8_dn7 = assign59540_e96670_d_n7;
        locals.var_t8_dn8 = assign59540_e96670_d_n8;
        locals.var_t8_dn9 = assign59540_e96670_d_n9;
        locals.var_t8_dn10 = assign59540_e96670_d_n10;
        locals.var_t8_dn11 = assign59540_e96670_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign59550_e96678, assign59550_e96678_d_n3, assign59550_e96678_d_n4, assign59550_e96678_d_n5, assign59550_e96678_d_n6, assign59550_e96678_d_n7, assign59550_e96678_d_n8, assign59550_e96678_d_n9, assign59550_e96678_d_n10, assign59550_e96678_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59550_e96676: f64 = (locals.var_t8).sqrt();
        (assign59550_e96676, (locals.var_t8_dn3 / (2.0 * assign59550_e96676)), (locals.var_t8_dn4 / (2.0 * assign59550_e96676)), (locals.var_t8_dn5 / (2.0 * assign59550_e96676)), (locals.var_t8_dn6 / (2.0 * assign59550_e96676)), (locals.var_t8_dn7 / (2.0 * assign59550_e96676)), (locals.var_t8_dn8 / (2.0 * assign59550_e96676)), (locals.var_t8_dn9 / (2.0 * assign59550_e96676)), (locals.var_t8_dn10 / (2.0 * assign59550_e96676)), (locals.var_t8_dn11 / (2.0 * assign59550_e96676)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign59550_e96678;
        locals.var_sqrtpsip_dn3 = assign59550_e96678_d_n3;
        locals.var_sqrtpsip_dn4 = assign59550_e96678_d_n4;
        locals.var_sqrtpsip_dn5 = assign59550_e96678_d_n5;
        locals.var_sqrtpsip_dn6 = assign59550_e96678_d_n6;
        locals.var_sqrtpsip_dn7 = assign59550_e96678_d_n7;
        locals.var_sqrtpsip_dn8 = assign59550_e96678_d_n8;
        locals.var_sqrtpsip_dn9 = assign59550_e96678_d_n9;
        locals.var_sqrtpsip_dn10 = assign59550_e96678_d_n10;
        locals.var_sqrtpsip_dn11 = assign59550_e96678_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign59560_e96693, assign59560_e96693_d_n3, assign59560_e96693_d_n4, assign59560_e96693_d_n5, assign59560_e96693_d_n6, assign59560_e96693_d_n7, assign59560_e96693_d_n8, assign59560_e96693_d_n9, assign59560_e96693_d_n10, assign59560_e96693_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59560_e96687: f64 = (2.0 * locals.var_sqrtpsip);
        let assign59560_e96688: f64 = (locals.var_gam_edge / assign59560_e96687);
        let assign59560_e96689: f64 = (1.0 + assign59560_e96688);
        let assign59560_e96691: f64 = (assign59560_e96689 / locals.var_gam_edge);
        (assign59560_e96691, ((((((locals.var_gam_edge_dn3 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign59560_e96687) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign59560_e96687 * assign59560_e96687)) * locals.var_gam_edge) - (assign59560_e96689 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign59560_e96693;
        locals.var_t0_dn3 = assign59560_e96693_d_n3;
        locals.var_t0_dn4 = assign59560_e96693_d_n4;
        locals.var_t0_dn5 = assign59560_e96693_d_n5;
        locals.var_t0_dn6 = assign59560_e96693_d_n6;
        locals.var_t0_dn7 = assign59560_e96693_d_n7;
        locals.var_t0_dn8 = assign59560_e96693_d_n8;
        locals.var_t0_dn9 = assign59560_e96693_d_n9;
        locals.var_t0_dn10 = assign59560_e96693_d_n10;
        locals.var_t0_dn11 = assign59560_e96693_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign59570_e96706, assign59570_e96706_d_n3, assign59570_e96706_d_n4, assign59570_e96706_d_n5, assign59570_e96706_d_n6, assign59570_e96706_d_n7, assign59570_e96706_d_n8, assign59570_e96706_d_n9, assign59570_e96706_d_n10, assign59570_e96706_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59570_e96701: f64 = (2.0 * locals.var_phib_n_edge);
        let assign59570_e96702: f64 = (locals.var_psip - assign59570_e96701);
        let assign59570_e96704: f64 = (assign59570_e96702 - locals.var_vs_1);
        (assign59570_e96704, ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vs_1_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vs_1_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vs_1_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vs_1_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vs_1_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vs_1_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vs_1_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vs_1_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vs_1_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59570_e96706;
        locals.var_t1_dn3 = assign59570_e96706_d_n3;
        locals.var_t1_dn4 = assign59570_e96706_d_n4;
        locals.var_t1_dn5 = assign59570_e96706_d_n5;
        locals.var_t1_dn6 = assign59570_e96706_d_n6;
        locals.var_t1_dn7 = assign59570_e96706_d_n7;
        locals.var_t1_dn8 = assign59570_e96706_d_n8;
        locals.var_t1_dn9 = assign59570_e96706_d_n9;
        locals.var_t1_dn10 = assign59570_e96706_d_n10;
        locals.var_t1_dn11 = assign59570_e96706_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59580_e96722, assign59580_e96722_d_n3, assign59580_e96722_d_n4, assign59580_e96722_d_n5, assign59580_e96722_d_n6, assign59580_e96722_d_n7, assign59580_e96722_d_n8, assign59580_e96722_d_n9, assign59580_e96722_d_n10, assign59580_e96722_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59580_e96714: f64 = (4.0 * locals.var_t0);
        let assign59580_e96716: f64 = (assign59580_e96714 * locals.var_sqrtpsip);
        let assign59580_e96718: f64 = (assign59580_e96716).max(1e-38);
        let assign59580_e96719: f64 = (assign59580_e96718).ln();
        let assign59580_e96720: f64 = (locals.var_t1 - assign59580_e96719);
        (assign59580_e96720, (locals.var_t1_dn3 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn4 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn5 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn6 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn7 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn8 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn9 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn10 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign59580_e96718)), (locals.var_t1_dn11 - (if assign59580_e96716 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign59580_e96714 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign59580_e96718)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59580_e96722;
        locals.var_t2_dn3 = assign59580_e96722_d_n3;
        locals.var_t2_dn4 = assign59580_e96722_d_n4;
        locals.var_t2_dn5 = assign59580_e96722_d_n5;
        locals.var_t2_dn6 = assign59580_e96722_d_n6;
        locals.var_t2_dn7 = assign59580_e96722_d_n7;
        locals.var_t2_dn8 = assign59580_e96722_d_n8;
        locals.var_t2_dn9 = assign59580_e96722_d_n9;
        locals.var_t2_dn10 = assign59580_e96722_d_n10;
        locals.var_t2_dn11 = assign59580_e96722_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign59590_e96742, assign59590_e96742_d_n3, assign59590_e96742_d_n4, assign59590_e96742_d_n5, assign59590_e96742_d_n6, assign59590_e96742_d_n7, assign59590_e96742_d_n8, assign59590_e96742_d_n9, assign59590_e96742_d_n10, assign59590_e96742_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59590_e96730: f64 = (locals.var_t2 - 0.201491);
        let assign59590_e96734: f64 = (locals.var_t2 + 0.402982);
        let assign59590_e96735: f64 = (locals.var_t2 * assign59590_e96734);
        let assign59590_e96737: f64 = (assign59590_e96735 + 2.446562);
        let assign59590_e96738: f64 = (assign59590_e96737).sqrt();
        let assign59590_e96739: f64 = (assign59590_e96730 - assign59590_e96738);
        let assign59590_e96740: f64 = (0.5 * assign59590_e96739);
        (assign59590_e96740, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign59590_e96738)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign59590_e96734) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign59590_e96738)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59590_e96742;
        locals.var_t8_dn3 = assign59590_e96742_d_n3;
        locals.var_t8_dn4 = assign59590_e96742_d_n4;
        locals.var_t8_dn5 = assign59590_e96742_d_n5;
        locals.var_t8_dn6 = assign59590_e96742_d_n6;
        locals.var_t8_dn7 = assign59590_e96742_d_n7;
        locals.var_t8_dn8 = assign59590_e96742_d_n8;
        locals.var_t8_dn9 = assign59590_e96742_d_n9;
        locals.var_t8_dn10 = assign59590_e96742_d_n10;
        locals.var_t8_dn11 = assign59590_e96742_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign59600_e96749, assign59600_e96749_d_n3, assign59600_e96749_d_n4, assign59600_e96749_d_n5, assign59600_e96749_d_n6, assign59600_e96749_d_n7, assign59600_e96749_d_n8, assign59600_e96749_d_n9, assign59600_e96749_d_n10, assign59600_e96749_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign59600_e96749;
        locals.var_sqrtpsisa_dn3 = assign59600_e96749_d_n3;
        locals.var_sqrtpsisa_dn4 = assign59600_e96749_d_n4;
        locals.var_sqrtpsisa_dn5 = assign59600_e96749_d_n5;
        locals.var_sqrtpsisa_dn6 = assign59600_e96749_d_n6;
        locals.var_sqrtpsisa_dn7 = assign59600_e96749_d_n7;
        locals.var_sqrtpsisa_dn8 = assign59600_e96749_d_n8;
        locals.var_sqrtpsisa_dn9 = assign59600_e96749_d_n9;
        locals.var_sqrtpsisa_dn10 = assign59600_e96749_d_n10;
        locals.var_sqrtpsisa_dn11 = assign59600_e96749_d_n11;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign59610_e96752: f64 = (-68.0);
        let assign59610_e96753: f64 = if locals.var_t8 <= assign59610_e96752 { 1.0 } else { 0.0 };
        locals.var_guard871 = assign59610_e96753;
        locals.var_guard871_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_208(
        locals: &mut StampLocals,
    ) {
        let (assign59620_e96763, assign59620_e96763_d_n3, assign59620_e96763_d_n4, assign59620_e96763_d_n5, assign59620_e96763_d_n6, assign59620_e96763_d_n7, assign59620_e96763_d_n8, assign59620_e96763_d_n9, assign59620_e96763_d_n10, assign59620_e96763_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign59620_e96761: f64 = (-100.0);
        (assign59620_e96761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59620_e96763;
        locals.var_t4_dn3 = assign59620_e96763_d_n3;
        locals.var_t4_dn4 = assign59620_e96763_d_n4;
        locals.var_t4_dn5 = assign59620_e96763_d_n5;
        locals.var_t4_dn6 = assign59620_e96763_d_n6;
        locals.var_t4_dn7 = assign59620_e96763_d_n7;
        locals.var_t4_dn8 = assign59620_e96763_d_n8;
        locals.var_t4_dn9 = assign59620_e96763_d_n9;
        locals.var_t4_dn10 = assign59620_e96763_d_n10;
        locals.var_t4_dn11 = assign59620_e96763_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign59630_e96772, assign59630_e96772_d_n3, assign59630_e96772_d_n4, assign59630_e96772_d_n5, assign59630_e96772_d_n6, assign59630_e96772_d_n7, assign59630_e96772_d_n8, assign59630_e96772_d_n9, assign59630_e96772_d_n10, assign59630_e96772_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59630_e96772;
        locals.var_t5_dn3 = assign59630_e96772_d_n3;
        locals.var_t5_dn4 = assign59630_e96772_d_n4;
        locals.var_t5_dn5 = assign59630_e96772_d_n5;
        locals.var_t5_dn6 = assign59630_e96772_d_n6;
        locals.var_t5_dn7 = assign59630_e96772_d_n7;
        locals.var_t5_dn8 = assign59630_e96772_d_n8;
        locals.var_t5_dn9 = assign59630_e96772_d_n9;
        locals.var_t5_dn10 = assign59630_e96772_d_n10;
        locals.var_t5_dn11 = assign59630_e96772_d_n11;
        locals.var_t5_rv = 0.0;

        let assign59640_e96777: f64 = (0.5 * locals.var_t5);
        let assign59640_e96778: f64 = (locals.var_t4 - assign59640_e96777);
        let assign59640_e96779: f64 = if locals.var_t8 < assign59640_e96778 { 1.0 } else { 0.0 };
        locals.var_guard872 = assign59640_e96779;
        locals.var_guard872_rv = 0.0;

        let (assign59650_e96791, assign59650_e96791_d_n3, assign59650_e96791_d_n4, assign59650_e96791_d_n5, assign59650_e96791_d_n6, assign59650_e96791_d_n7, assign59650_e96791_d_n8, assign59650_e96791_d_n9, assign59650_e96791_d_n10, assign59650_e96791_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign59650_e96789: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59650_e96789, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59650_e96791;
        locals.var_t3_dn3 = assign59650_e96791_d_n3;
        locals.var_t3_dn4 = assign59650_e96791_d_n4;
        locals.var_t3_dn5 = assign59650_e96791_d_n5;
        locals.var_t3_dn6 = assign59650_e96791_d_n6;
        locals.var_t3_dn7 = assign59650_e96791_d_n7;
        locals.var_t3_dn8 = assign59650_e96791_d_n8;
        locals.var_t3_dn9 = assign59650_e96791_d_n9;
        locals.var_t3_dn10 = assign59650_e96791_d_n10;
        locals.var_t3_dn11 = assign59650_e96791_d_n11;
        locals.var_t3_rv = 0.0;

        let assign59660_e96796: f64 = (0.5 * locals.var_t5);
        let assign59660_e96797: f64 = (locals.var_t4 + assign59660_e96796);
        let assign59660_e96798: f64 = if locals.var_t8 > assign59660_e96797 { 1.0 } else { 0.0 };
        locals.var_guard873 = assign59660_e96798;
        locals.var_guard873_rv = 0.0;

        let (assign59670_e96813, assign59670_e96813_d_n3, assign59670_e96813_d_n4, assign59670_e96813_d_n5, assign59670_e96813_d_n6, assign59670_e96813_d_n7, assign59670_e96813_d_n8, assign59670_e96813_d_n9, assign59670_e96813_d_n10, assign59670_e96813_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign59670_e96811: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59670_e96811, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59670_e96813;
        locals.var_t3_dn3 = assign59670_e96813_d_n3;
        locals.var_t3_dn4 = assign59670_e96813_d_n4;
        locals.var_t3_dn5 = assign59670_e96813_d_n5;
        locals.var_t3_dn6 = assign59670_e96813_d_n6;
        locals.var_t3_dn7 = assign59670_e96813_d_n7;
        locals.var_t3_dn8 = assign59670_e96813_d_n8;
        locals.var_t3_dn9 = assign59670_e96813_d_n9;
        locals.var_t3_dn10 = assign59670_e96813_d_n10;
        locals.var_t3_dn11 = assign59670_e96813_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59680_e96832, assign59680_e96832_d_n3, assign59680_e96832_d_n4, assign59680_e96832_d_n5, assign59680_e96832_d_n6, assign59680_e96832_d_n7, assign59680_e96832_d_n8, assign59680_e96832_d_n9, assign59680_e96832_d_n10, assign59680_e96832_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign59680_e96828: f64 = (locals.var_t8 - locals.var_t4);
        let assign59680_e96830: f64 = (assign59680_e96828 / locals.var_t5);
        (assign59680_e96830, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign59680_e96828 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59680_e96832;
        locals.var_t2_dn3 = assign59680_e96832_d_n3;
        locals.var_t2_dn4 = assign59680_e96832_d_n4;
        locals.var_t2_dn5 = assign59680_e96832_d_n5;
        locals.var_t2_dn6 = assign59680_e96832_d_n6;
        locals.var_t2_dn7 = assign59680_e96832_d_n7;
        locals.var_t2_dn8 = assign59680_e96832_d_n8;
        locals.var_t2_dn9 = assign59680_e96832_d_n9;
        locals.var_t2_dn10 = assign59680_e96832_d_n10;
        locals.var_t2_dn11 = assign59680_e96832_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign59690_e96849, assign59690_e96849_d_n3, assign59690_e96849_d_n4, assign59690_e96849_d_n5, assign59690_e96849_d_n6, assign59690_e96849_d_n7, assign59690_e96849_d_n8, assign59690_e96849_d_n9, assign59690_e96849_d_n10, assign59690_e96849_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign59690_e96847: f64 = (locals.var_t2 * locals.var_t2);
        (assign59690_e96847, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign59690_e96849;
        locals.var_t6_dn3 = assign59690_e96849_d_n3;
        locals.var_t6_dn4 = assign59690_e96849_d_n4;
        locals.var_t6_dn5 = assign59690_e96849_d_n5;
        locals.var_t6_dn6 = assign59690_e96849_d_n6;
        locals.var_t6_dn7 = assign59690_e96849_d_n7;
        locals.var_t6_dn8 = assign59690_e96849_d_n8;
        locals.var_t6_dn9 = assign59690_e96849_d_n9;
        locals.var_t6_dn10 = assign59690_e96849_d_n10;
        locals.var_t6_dn11 = assign59690_e96849_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign59700_e96887, assign59700_e96887_d_n3, assign59700_e96887_d_n4, assign59700_e96887_d_n5, assign59700_e96887_d_n6, assign59700_e96887_d_n7, assign59700_e96887_d_n8, assign59700_e96887_d_n9, assign59700_e96887_d_n10, assign59700_e96887_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) && (locals.var_guard873 == 0.0)) {
        let assign59700_e96866: f64 = (5.0 / 64.0);
        let assign59700_e96869: f64 = (0.5 * locals.var_t2);
        let assign59700_e96870: f64 = (assign59700_e96866 + assign59700_e96869);
        let assign59700_e96874: f64 = (15.0 / 16.0);
        let assign59700_e96878: f64 = (1.25 - locals.var_t6);
        let assign59700_e96879: f64 = (locals.var_t6 * assign59700_e96878);
        let assign59700_e96880: f64 = (assign59700_e96874 - assign59700_e96879);
        let assign59700_e96881: f64 = (locals.var_t6 * assign59700_e96880);
        let assign59700_e96882: f64 = (assign59700_e96870 + assign59700_e96881);
        let assign59700_e96883: f64 = (locals.var_t5 * assign59700_e96882);
        let assign59700_e96884: f64 = (locals.var_t4 + assign59700_e96883);
        let assign59700_e96885: f64 = { let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59700_e96885, ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign59700_e96882) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign59700_e96880) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign59700_e96878) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59700_e96887;
        locals.var_t3_dn3 = assign59700_e96887_d_n3;
        locals.var_t3_dn4 = assign59700_e96887_d_n4;
        locals.var_t3_dn5 = assign59700_e96887_d_n5;
        locals.var_t3_dn6 = assign59700_e96887_d_n6;
        locals.var_t3_dn7 = assign59700_e96887_d_n7;
        locals.var_t3_dn8 = assign59700_e96887_d_n8;
        locals.var_t3_dn9 = assign59700_e96887_d_n9;
        locals.var_t3_dn10 = assign59700_e96887_d_n10;
        locals.var_t3_dn11 = assign59700_e96887_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59710_e96919, assign59710_e96919_d_n3, assign59710_e96919_d_n4, assign59710_e96919_d_n5, assign59710_e96919_d_n6, assign59710_e96919_d_n7, assign59710_e96919_d_n8, assign59710_e96919_d_n9, assign59710_e96919_d_n10, assign59710_e96919_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign59710_e96897: f64 = (1.0 + locals.var_t1);
        let assign59710_e96899: f64 = (assign59710_e96897 - locals.var_t8);
        let assign59710_e96902: f64 = (2.0 * locals.var_t0);
        let assign59710_e96905: f64 = (locals.var_t3 * 2.0);
        let assign59710_e96907: f64 = (assign59710_e96905 * locals.var_t0);
        let assign59710_e96910: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign59710_e96911: f64 = (assign59710_e96907 + assign59710_e96910);
        let assign59710_e96912: f64 = (assign59710_e96902 * assign59710_e96911);
        let assign59710_e96914: f64 = (assign59710_e96912).max(1e-38);
        let assign59710_e96915: f64 = (assign59710_e96914).ln();
        let assign59710_e96916: f64 = (assign59710_e96899 - assign59710_e96915);
        let assign59710_e96917: f64 = (locals.var_t3 * assign59710_e96916);
        (assign59710_e96917, ((locals.var_t3_dn3 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn4 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn5 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn6 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn7 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn8 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn9 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn10 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59710_e96914)))), ((locals.var_t3_dn11 * assign59710_e96916) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign59710_e96912 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign59710_e96911) + (assign59710_e96902 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59710_e96905 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59710_e96914)))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11,)
    }
};
        locals.var_qs_edge = assign59710_e96919;
        locals.var_qs_edge_dn3 = assign59710_e96919_d_n3;
        locals.var_qs_edge_dn4 = assign59710_e96919_d_n4;
        locals.var_qs_edge_dn5 = assign59710_e96919_d_n5;
        locals.var_qs_edge_dn6 = assign59710_e96919_d_n6;
        locals.var_qs_edge_dn7 = assign59710_e96919_d_n7;
        locals.var_qs_edge_dn8 = assign59710_e96919_d_n8;
        locals.var_qs_edge_dn9 = assign59710_e96919_d_n9;
        locals.var_qs_edge_dn10 = assign59710_e96919_d_n10;
        locals.var_qs_edge_dn11 = assign59710_e96919_d_n11;
        locals.var_qs_edge_rv = 0.0;

        let (assign59720_e96930, assign59720_e96930_d_n3, assign59720_e96930_d_n4, assign59720_e96930_d_n5, assign59720_e96930_d_n6, assign59720_e96930_d_n7, assign59720_e96930_d_n8, assign59720_e96930_d_n9, assign59720_e96930_d_n10, assign59720_e96930_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59720_e96928: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59720_e96928, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59720_e96930;
        locals.var_t3_dn3 = assign59720_e96930_d_n3;
        locals.var_t3_dn4 = assign59720_e96930_d_n4;
        locals.var_t3_dn5 = assign59720_e96930_d_n5;
        locals.var_t3_dn6 = assign59720_e96930_d_n6;
        locals.var_t3_dn7 = assign59720_e96930_d_n7;
        locals.var_t3_dn8 = assign59720_e96930_d_n8;
        locals.var_t3_dn9 = assign59720_e96930_d_n9;
        locals.var_t3_dn10 = assign59720_e96930_d_n10;
        locals.var_t3_dn11 = assign59720_e96930_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59730_e96942, assign59730_e96942_d_n3, assign59730_e96942_d_n4, assign59730_e96942_d_n5, assign59730_e96942_d_n6, assign59730_e96942_d_n7, assign59730_e96942_d_n8, assign59730_e96942_d_n9, assign59730_e96942_d_n10, assign59730_e96942_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59730_e96940: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign59730_e96940, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign59730_e96942;
        locals.var_sqrtpsisainv_dn3 = assign59730_e96942_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign59730_e96942_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign59730_e96942_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign59730_e96942_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign59730_e96942_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign59730_e96942_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign59730_e96942_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign59730_e96942_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign59730_e96942_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign59740_e96975, assign59740_e96975_d_n3, assign59740_e96975_d_n4, assign59740_e96975_d_n5, assign59740_e96975_d_n6, assign59740_e96975_d_n7, assign59740_e96975_d_n8, assign59740_e96975_d_n9, assign59740_e96975_d_n10, assign59740_e96975_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59740_e96952: f64 = (2.0 * locals.var_t3);
        let assign59740_e96955: f64 = (locals.var_t3 * 2.0);
        let assign59740_e96957: f64 = (assign59740_e96955 * locals.var_t0);
        let assign59740_e96960: f64 = (locals.var_t3 * 2.0);
        let assign59740_e96962: f64 = (assign59740_e96960 * locals.var_t0);
        let assign59740_e96965: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign59740_e96966: f64 = (assign59740_e96962 + assign59740_e96965);
        let assign59740_e96967: f64 = (assign59740_e96957 * assign59740_e96966);
        let assign59740_e96969: f64 = (assign59740_e96967).max(1e-38);
        let assign59740_e96970: f64 = (assign59740_e96969).ln();
        let assign59740_e96971: f64 = (assign59740_e96952 + assign59740_e96970);
        let assign59740_e96973: f64 = (assign59740_e96971 - locals.var_t1);
        (assign59740_e96973, (((2.0 * locals.var_t3_dn3) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn3)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn4)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn5)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn6)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn7)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn8)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn9)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn10)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign59740_e96967 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59740_e96955 * locals.var_t0_dn11)) * assign59740_e96966) + (assign59740_e96957 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59740_e96960 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59740_e96969)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59740_e96975;
        locals.var_t4_dn3 = assign59740_e96975_d_n3;
        locals.var_t4_dn4 = assign59740_e96975_d_n4;
        locals.var_t4_dn5 = assign59740_e96975_d_n5;
        locals.var_t4_dn6 = assign59740_e96975_d_n6;
        locals.var_t4_dn7 = assign59740_e96975_d_n7;
        locals.var_t4_dn8 = assign59740_e96975_d_n8;
        locals.var_t4_dn9 = assign59740_e96975_d_n9;
        locals.var_t4_dn10 = assign59740_e96975_d_n10;
        locals.var_t4_dn11 = assign59740_e96975_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign59750_e96999, assign59750_e96999_d_n3, assign59750_e96999_d_n4, assign59750_e96999_d_n5, assign59750_e96999_d_n6, assign59750_e96999_d_n7, assign59750_e96999_d_n8, assign59750_e96999_d_n9, assign59750_e96999_d_n10, assign59750_e96999_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59750_e96986: f64 = (1.0 / locals.var_t3);
        let assign59750_e96987: f64 = (2.0 + assign59750_e96986);
        let assign59750_e96990: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59750_e96993: f64 = (locals.var_t0 * locals.var_t3);
        let assign59750_e96995: f64 = (assign59750_e96993 + locals.var_sqrtpsisa);
        let assign59750_e96996: f64 = (assign59750_e96990 / assign59750_e96995);
        let assign59750_e96997: f64 = (assign59750_e96987 + assign59750_e96996);
        (assign59750_e96997, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59750_e96995 * assign59750_e96995))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59750_e96995) - (assign59750_e96990 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59750_e96995 * assign59750_e96995))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59750_e96999;
        locals.var_t5_dn3 = assign59750_e96999_d_n3;
        locals.var_t5_dn4 = assign59750_e96999_d_n4;
        locals.var_t5_dn5 = assign59750_e96999_d_n5;
        locals.var_t5_dn6 = assign59750_e96999_d_n6;
        locals.var_t5_dn7 = assign59750_e96999_d_n7;
        locals.var_t5_dn8 = assign59750_e96999_d_n8;
        locals.var_t5_dn9 = assign59750_e96999_d_n9;
        locals.var_t5_dn10 = assign59750_e96999_d_n10;
        locals.var_t5_dn11 = assign59750_e96999_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign59760_e97013, assign59760_e97013_d_n3, assign59760_e97013_d_n4, assign59760_e97013_d_n5, assign59760_e97013_d_n6, assign59760_e97013_d_n7, assign59760_e97013_d_n8, assign59760_e97013_d_n9, assign59760_e97013_d_n10, assign59760_e97013_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59760_e97010: f64 = (locals.var_t4 / locals.var_t5);
        let assign59760_e97011: f64 = (locals.var_t3 - assign59760_e97010);
        (assign59760_e97011, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign59760_e97013;
        locals.var_t3_dn3 = assign59760_e97013_d_n3;
        locals.var_t3_dn4 = assign59760_e97013_d_n4;
        locals.var_t3_dn5 = assign59760_e97013_d_n5;
        locals.var_t3_dn6 = assign59760_e97013_d_n6;
        locals.var_t3_dn7 = assign59760_e97013_d_n7;
        locals.var_t3_dn8 = assign59760_e97013_d_n8;
        locals.var_t3_dn9 = assign59760_e97013_d_n9;
        locals.var_t3_dn10 = assign59760_e97013_d_n10;
        locals.var_t3_dn11 = assign59760_e97013_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign59770_e97046, assign59770_e97046_d_n3, assign59770_e97046_d_n4, assign59770_e97046_d_n5, assign59770_e97046_d_n6, assign59770_e97046_d_n7, assign59770_e97046_d_n8, assign59770_e97046_d_n9, assign59770_e97046_d_n10, assign59770_e97046_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59770_e97023: f64 = (2.0 * locals.var_t3);
        let assign59770_e97026: f64 = (locals.var_t3 * 2.0);
        let assign59770_e97028: f64 = (assign59770_e97026 * locals.var_t0);
        let assign59770_e97031: f64 = (locals.var_t3 * 2.0);
        let assign59770_e97033: f64 = (assign59770_e97031 * locals.var_t0);
        let assign59770_e97036: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign59770_e97037: f64 = (assign59770_e97033 + assign59770_e97036);
        let assign59770_e97038: f64 = (assign59770_e97028 * assign59770_e97037);
        let assign59770_e97040: f64 = (assign59770_e97038).max(1e-38);
        let assign59770_e97041: f64 = (assign59770_e97040).ln();
        let assign59770_e97042: f64 = (assign59770_e97023 + assign59770_e97041);
        let assign59770_e97044: f64 = (assign59770_e97042 - locals.var_t1);
        (assign59770_e97044, (((2.0 * locals.var_t3_dn3) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn3)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn4)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn5)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn6)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn7)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn8)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn9)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn10)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign59770_e97038 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59770_e97026 * locals.var_t0_dn11)) * assign59770_e97037) + (assign59770_e97028 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign59770_e97031 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59770_e97040)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59770_e97046;
        locals.var_t4_dn3 = assign59770_e97046_d_n3;
        locals.var_t4_dn4 = assign59770_e97046_d_n4;
        locals.var_t4_dn5 = assign59770_e97046_d_n5;
        locals.var_t4_dn6 = assign59770_e97046_d_n6;
        locals.var_t4_dn7 = assign59770_e97046_d_n7;
        locals.var_t4_dn8 = assign59770_e97046_d_n8;
        locals.var_t4_dn9 = assign59770_e97046_d_n9;
        locals.var_t4_dn10 = assign59770_e97046_d_n10;
        locals.var_t4_dn11 = assign59770_e97046_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign59780_e97070, assign59780_e97070_d_n3, assign59780_e97070_d_n4, assign59780_e97070_d_n5, assign59780_e97070_d_n6, assign59780_e97070_d_n7, assign59780_e97070_d_n8, assign59780_e97070_d_n9, assign59780_e97070_d_n10, assign59780_e97070_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59780_e97057: f64 = (1.0 / locals.var_t3);
        let assign59780_e97058: f64 = (2.0 + assign59780_e97057);
        let assign59780_e97061: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59780_e97064: f64 = (locals.var_t0 * locals.var_t3);
        let assign59780_e97066: f64 = (assign59780_e97064 + locals.var_sqrtpsisa);
        let assign59780_e97067: f64 = (assign59780_e97061 / assign59780_e97066);
        let assign59780_e97068: f64 = (assign59780_e97058 + assign59780_e97067);
        (assign59780_e97068, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59780_e97066 * assign59780_e97066))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59780_e97066) - (assign59780_e97061 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59780_e97066 * assign59780_e97066))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59780_e97070;
        locals.var_t5_dn3 = assign59780_e97070_d_n3;
        locals.var_t5_dn4 = assign59780_e97070_d_n4;
        locals.var_t5_dn5 = assign59780_e97070_d_n5;
        locals.var_t5_dn6 = assign59780_e97070_d_n6;
        locals.var_t5_dn7 = assign59780_e97070_d_n7;
        locals.var_t5_dn8 = assign59780_e97070_d_n8;
        locals.var_t5_dn9 = assign59780_e97070_d_n9;
        locals.var_t5_dn10 = assign59780_e97070_d_n10;
        locals.var_t5_dn11 = assign59780_e97070_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign59790_e97098, assign59790_e97098_d_n3, assign59790_e97098_d_n4, assign59790_e97098_d_n5, assign59790_e97098_d_n6, assign59790_e97098_d_n7, assign59790_e97098_d_n8, assign59790_e97098_d_n9, assign59790_e97098_d_n10, assign59790_e97098_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59790_e97080: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59790_e97083: f64 = (locals.var_t0 * locals.var_t3);
        let assign59790_e97085: f64 = (assign59790_e97083 + locals.var_sqrtpsisa);
        let assign59790_e97086: f64 = (assign59790_e97080 / assign59790_e97085);
        let assign59790_e97089: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign59790_e97092: f64 = (locals.var_t0 * locals.var_t3);
        let assign59790_e97094: f64 = (assign59790_e97092 + locals.var_sqrtpsisa);
        let assign59790_e97095: f64 = (assign59790_e97089 / assign59790_e97094);
        let assign59790_e97096: f64 = (assign59790_e97086 * assign59790_e97095);
        (assign59790_e97096, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59790_e97094 * assign59790_e97094)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59790_e97085) - (assign59790_e97080 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign59790_e97094) - (assign59790_e97089 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59790_e97094 * assign59790_e97094)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign59790_e97098;
        locals.var_t6_dn3 = assign59790_e97098_d_n3;
        locals.var_t6_dn4 = assign59790_e97098_d_n4;
        locals.var_t6_dn5 = assign59790_e97098_d_n5;
        locals.var_t6_dn6 = assign59790_e97098_d_n6;
        locals.var_t6_dn7 = assign59790_e97098_d_n7;
        locals.var_t6_dn8 = assign59790_e97098_d_n8;
        locals.var_t6_dn9 = assign59790_e97098_d_n9;
        locals.var_t6_dn10 = assign59790_e97098_d_n10;
        locals.var_t6_dn11 = assign59790_e97098_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign59800_e97131, assign59800_e97131_d_n3, assign59800_e97131_d_n4, assign59800_e97131_d_n5, assign59800_e97131_d_n6, assign59800_e97131_d_n7, assign59800_e97131_d_n8, assign59800_e97131_d_n9, assign59800_e97131_d_n10, assign59800_e97131_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign59800_e97108: f64 = (1.0 * __rspice_inv_cse_0);
        let assign59800_e97111: f64 = (1.0 * __rspice_inv_cse_0);
        let assign59800_e97112: f64 = (assign59800_e97108 * assign59800_e97111);
        let assign59800_e97113: f64 = (-assign59800_e97112);
        let assign59800_e97117: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign59800_e97119: f64 = (assign59800_e97117 * locals.var_sqrtpsisa);
        let assign59800_e97122: f64 = (locals.var_t0 * locals.var_t3);
        let assign59800_e97124: f64 = (assign59800_e97122 + locals.var_sqrtpsisa);
        let assign59800_e97125: f64 = (assign59800_e97119 * assign59800_e97124);
        let assign59800_e97126: f64 = (1.0 / assign59800_e97125);
        let assign59800_e97127: f64 = (assign59800_e97113 - assign59800_e97126);
        let assign59800_e97129: f64 = (assign59800_e97127 - locals.var_t6);
        (assign59800_e97129, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn3)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn4)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn5)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn6)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn7)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn8)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn9)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn10)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign59800_e97117 * locals.var_sqrtpsisa_dn11)) * assign59800_e97124) + (assign59800_e97119 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign59800_e97125 * assign59800_e97125)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign59800_e97131;
        locals.var_t7_dn3 = assign59800_e97131_d_n3;
        locals.var_t7_dn4 = assign59800_e97131_d_n4;
        locals.var_t7_dn5 = assign59800_e97131_d_n5;
        locals.var_t7_dn6 = assign59800_e97131_d_n6;
        locals.var_t7_dn7 = assign59800_e97131_d_n7;
        locals.var_t7_dn8 = assign59800_e97131_d_n8;
        locals.var_t7_dn9 = assign59800_e97131_d_n9;
        locals.var_t7_dn10 = assign59800_e97131_d_n10;
        locals.var_t7_dn11 = assign59800_e97131_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign59810_e97157, assign59810_e97157_d_n3, assign59810_e97157_d_n4, assign59810_e97157_d_n5, assign59810_e97157_d_n6, assign59810_e97157_d_n7, assign59810_e97157_d_n8, assign59810_e97157_d_n9, assign59810_e97157_d_n10, assign59810_e97157_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign59810_e97142: f64 = (locals.var_t4 / locals.var_t5);
        let assign59810_e97146: f64 = (locals.var_t4 * locals.var_t7);
        let assign59810_e97149: f64 = (2.0 * locals.var_t5);
        let assign59810_e97151: f64 = (assign59810_e97149 * locals.var_t5);
        let assign59810_e97152: f64 = (assign59810_e97146 / assign59810_e97151);
        let assign59810_e97153: f64 = (1.0 + assign59810_e97152);
        let assign59810_e97154: f64 = (assign59810_e97142 * assign59810_e97153);
        let assign59810_e97155: f64 = (locals.var_t3 - assign59810_e97154);
        (assign59810_e97155, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn3)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn4)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn5)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn6)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn7)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn8)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn9)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn10)))) / (assign59810_e97151 * assign59810_e97151))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign59810_e97149 * locals.var_t5_dn11)))) / (assign59810_e97151 * assign59810_e97151))))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11,)
    }
};
        locals.var_qs_edge = assign59810_e97157;
        locals.var_qs_edge_dn3 = assign59810_e97157_d_n3;
        locals.var_qs_edge_dn4 = assign59810_e97157_d_n4;
        locals.var_qs_edge_dn5 = assign59810_e97157_d_n5;
        locals.var_qs_edge_dn6 = assign59810_e97157_d_n6;
        locals.var_qs_edge_dn7 = assign59810_e97157_d_n7;
        locals.var_qs_edge_dn8 = assign59810_e97157_d_n8;
        locals.var_qs_edge_dn9 = assign59810_e97157_d_n9;
        locals.var_qs_edge_dn10 = assign59810_e97157_d_n10;
        locals.var_qs_edge_dn11 = assign59810_e97157_d_n11;
        locals.var_qs_edge_rv = 0.0;

        let (assign59820_e97172, assign59820_e97172_d_n3, assign59820_e97172_d_n4, assign59820_e97172_d_n5, assign59820_e97172_d_n6, assign59820_e97172_d_n7, assign59820_e97172_d_n8, assign59820_e97172_d_n9, assign59820_e97172_d_n10, assign59820_e97172_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59820_e97164: f64 = (2.0 * locals.var_nvt);
        let assign59820_e97166: f64 = (assign59820_e97164 * locals.var_qs_edge);
        let assign59820_e97169: f64 = (2.0 * locals.var_nvt);
        let assign59820_e97170: f64 = (assign59820_e97166 + assign59820_e97169);
        (assign59820_e97170, ((((2.0 * locals.var_nvt_dn3) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn3)) + (2.0 * locals.var_nvt_dn3)), ((((2.0 * locals.var_nvt_dn4) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn4)) + (2.0 * locals.var_nvt_dn4)), ((((2.0 * locals.var_nvt_dn5) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn5)) + (2.0 * locals.var_nvt_dn5)), ((((2.0 * locals.var_nvt_dn6) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn6)) + (2.0 * locals.var_nvt_dn6)), ((((2.0 * locals.var_nvt_dn7) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn7)) + (2.0 * locals.var_nvt_dn7)), ((((2.0 * locals.var_nvt_dn8) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn8)) + (2.0 * locals.var_nvt_dn8)), ((((2.0 * locals.var_nvt_dn9) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn9)) + (2.0 * locals.var_nvt_dn9)), ((((2.0 * locals.var_nvt_dn10) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn10)) + (2.0 * locals.var_nvt_dn10)), ((((2.0 * locals.var_nvt_dn11) * locals.var_qs_edge) + (assign59820_e97164 * locals.var_qs_edge_dn11)) + (2.0 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11,)
    }
};
        locals.var_vdsatedge = assign59820_e97172;
        locals.var_vdsatedge_dn3 = assign59820_e97172_d_n3;
        locals.var_vdsatedge_dn4 = assign59820_e97172_d_n4;
        locals.var_vdsatedge_dn5 = assign59820_e97172_d_n5;
        locals.var_vdsatedge_dn6 = assign59820_e97172_d_n6;
        locals.var_vdsatedge_dn7 = assign59820_e97172_d_n7;
        locals.var_vdsatedge_dn8 = assign59820_e97172_d_n8;
        locals.var_vdsatedge_dn9 = assign59820_e97172_d_n9;
        locals.var_vdsatedge_dn10 = assign59820_e97172_d_n10;
        locals.var_vdsatedge_dn11 = assign59820_e97172_d_n11;
        locals.var_vdsatedge_rv = 0.0;

        let (assign59830_e97179, assign59830_e97179_d_n3, assign59830_e97179_d_n4, assign59830_e97179_d_n5, assign59830_e97179_d_n6, assign59830_e97179_d_n7, assign59830_e97179_d_n8, assign59830_e97179_d_n9, assign59830_e97179_d_n10, assign59830_e97179_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11,)
    }
};
        locals.var_vdsatedge_1 = assign59830_e97179;
        locals.var_vdsatedge_1_dn3 = assign59830_e97179_d_n3;
        locals.var_vdsatedge_1_dn4 = assign59830_e97179_d_n4;
        locals.var_vdsatedge_1_dn5 = assign59830_e97179_d_n5;
        locals.var_vdsatedge_1_dn6 = assign59830_e97179_d_n6;
        locals.var_vdsatedge_1_dn7 = assign59830_e97179_d_n7;
        locals.var_vdsatedge_1_dn8 = assign59830_e97179_d_n8;
        locals.var_vdsatedge_1_dn9 = assign59830_e97179_d_n9;
        locals.var_vdsatedge_1_dn10 = assign59830_e97179_d_n10;
        locals.var_vdsatedge_1_dn11 = assign59830_e97179_d_n11;
        locals.var_vdsatedge_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_209(
        locals: &mut StampLocals,
    ) {
        let (assign59840_e97188, assign59840_e97188_d_n3, assign59840_e97188_d_n4, assign59840_e97188_d_n5, assign59840_e97188_d_n6, assign59840_e97188_d_n7, assign59840_e97188_d_n8, assign59840_e97188_d_n9, assign59840_e97188_d_n10, assign59840_e97188_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59840_e97186: f64 = (locals.var_vdsatedge_1 + locals.var_vs);
        (assign59840_e97186, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, (locals.var_vdsatedge_1_dn6 + locals.var_vs_dn6), (locals.var_vdsatedge_1_dn7 + locals.var_vs_dn7), locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, (locals.var_vdsatedge_1_dn10 + locals.var_vs_dn10), locals.var_vdsatedge_1_dn11,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11,)
    }
};
        locals.var_vdsatedge_1 = assign59840_e97188;
        locals.var_vdsatedge_1_dn3 = assign59840_e97188_d_n3;
        locals.var_vdsatedge_1_dn4 = assign59840_e97188_d_n4;
        locals.var_vdsatedge_1_dn5 = assign59840_e97188_d_n5;
        locals.var_vdsatedge_1_dn6 = assign59840_e97188_d_n6;
        locals.var_vdsatedge_1_dn7 = assign59840_e97188_d_n7;
        locals.var_vdsatedge_1_dn8 = assign59840_e97188_d_n8;
        locals.var_vdsatedge_1_dn9 = assign59840_e97188_d_n9;
        locals.var_vdsatedge_1_dn10 = assign59840_e97188_d_n10;
        locals.var_vdsatedge_1_dn11 = assign59840_e97188_d_n11;
        locals.var_vdsatedge_1_rv = 0.0;

        let (assign59850_e97220, assign59850_e97220_d_n3, assign59850_e97220_d_n4, assign59850_e97220_d_n5, assign59850_e97220_d_n6, assign59850_e97220_d_n7, assign59850_e97220_d_n8, assign59850_e97220_d_n9, assign59850_e97220_d_n10, assign59850_e97220_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59850_e97196: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign59850_e97198: f64 = assign59850_e97196;
        let assign59850_e97201: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign59850_e97203: f64 = assign59850_e97201;
        let assign59850_e97206: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign59850_e97208: f64 = assign59850_e97206;
        let assign59850_e97209: f64 = (assign59850_e97203 * assign59850_e97208);
        let assign59850_e97212: f64 = (0.25 * 0.001);
        let assign59850_e97214: f64 = (assign59850_e97212 * 0.001);
        let assign59850_e97215: f64 = (assign59850_e97209 + assign59850_e97214);
        let assign59850_e97216: f64 = (assign59850_e97215).sqrt();
        let assign59850_e97217: f64 = (assign59850_e97198 + assign59850_e97216);
        let assign59850_e97218: f64 = (0.5 * assign59850_e97217);
        (assign59850_e97218, (0.5 * (locals.var_vdsatedge_1_dn3 + (((locals.var_vdsatedge_1_dn3 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn3)) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn4 + (((locals.var_vdsatedge_1_dn4 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn4)) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn5 + (((locals.var_vdsatedge_1_dn5 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn5)) / (2.0 * assign59850_e97216)))), (0.5 * ((locals.var_vdsatedge_1_dn6 - locals.var_vs_dn6) + ((((locals.var_vdsatedge_1_dn6 - locals.var_vs_dn6) * assign59850_e97208) + (assign59850_e97203 * (locals.var_vdsatedge_1_dn6 - locals.var_vs_dn6))) / (2.0 * assign59850_e97216)))), (0.5 * ((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) * assign59850_e97208) + (assign59850_e97203 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn8 + (((locals.var_vdsatedge_1_dn8 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn8)) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn9 + (((locals.var_vdsatedge_1_dn9 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn9)) / (2.0 * assign59850_e97216)))), (0.5 * ((locals.var_vdsatedge_1_dn10 - locals.var_vs_dn10) + ((((locals.var_vdsatedge_1_dn10 - locals.var_vs_dn10) * assign59850_e97208) + (assign59850_e97203 * (locals.var_vdsatedge_1_dn10 - locals.var_vs_dn10))) / (2.0 * assign59850_e97216)))), (0.5 * (locals.var_vdsatedge_1_dn11 + (((locals.var_vdsatedge_1_dn11 * assign59850_e97208) + (assign59850_e97203 * locals.var_vdsatedge_1_dn11)) / (2.0 * assign59850_e97216)))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11,)
    }
};
        locals.var_vdssate = assign59850_e97220;
        locals.var_vdssate_dn3 = assign59850_e97220_d_n3;
        locals.var_vdssate_dn4 = assign59850_e97220_d_n4;
        locals.var_vdssate_dn5 = assign59850_e97220_d_n5;
        locals.var_vdssate_dn6 = assign59850_e97220_d_n6;
        locals.var_vdssate_dn7 = assign59850_e97220_d_n7;
        locals.var_vdssate_dn8 = assign59850_e97220_d_n8;
        locals.var_vdssate_dn9 = assign59850_e97220_d_n9;
        locals.var_vdssate_dn10 = assign59850_e97220_d_n10;
        locals.var_vdssate_dn11 = assign59850_e97220_d_n11;
        locals.var_vdssate_rv = 0.0;

        let (assign59860_e97235, assign59860_e97235_d_n3, assign59860_e97235_d_n4, assign59860_e97235_d_n5, assign59860_e97235_d_n6, assign59860_e97235_d_n7, assign59860_e97235_d_n8, assign59860_e97235_d_n9, assign59860_e97235_d_n10, assign59860_e97235_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59860_e97227: f64 = (locals.var_vds / locals.var_vdssate);
        let assign59860_e97229: f64 = (assign59860_e97227 + 1e-6);
        let assign59860_e97232: f64 = (1.0 / locals.var_delta_t);
        let assign59860_e97233: f64 = (assign59860_e97229).powf(assign59860_e97232);
        (assign59860_e97233, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn5) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn5) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((locals.var_vds_dn6 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn6)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((locals.var_vds_dn6 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn6)) / (locals.var_vdssate * locals.var_vdssate)) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((locals.var_vds_dn10 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn10)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((locals.var_vds_dn10 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn10)) / (locals.var_vdssate * locals.var_vdssate)) / assign59860_e97229)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn11) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign59860_e97233 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((locals.var_vds * locals.var_vdssate_dn11) / (locals.var_vdssate * locals.var_vdssate))) / assign59860_e97229)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign59860_e97235;
        locals.var_t7_dn3 = assign59860_e97235_d_n3;
        locals.var_t7_dn4 = assign59860_e97235_d_n4;
        locals.var_t7_dn5 = assign59860_e97235_d_n5;
        locals.var_t7_dn6 = assign59860_e97235_d_n6;
        locals.var_t7_dn7 = assign59860_e97235_d_n7;
        locals.var_t7_dn8 = assign59860_e97235_d_n8;
        locals.var_t7_dn9 = assign59860_e97235_d_n9;
        locals.var_t7_dn10 = assign59860_e97235_d_n10;
        locals.var_t7_dn11 = assign59860_e97235_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign59870_e97247, assign59870_e97247_d_n3, assign59870_e97247_d_n4, assign59870_e97247_d_n5, assign59870_e97247_d_n6, assign59870_e97247_d_n7, assign59870_e97247_d_n8, assign59870_e97247_d_n9, assign59870_e97247_d_n10, assign59870_e97247_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59870_e97242: f64 = (1.0 + locals.var_t7);
        let assign59870_e97244: f64 = (-locals.var_delta_t);
        let assign59870_e97245: f64 = (assign59870_e97242).powf(assign59870_e97244);
        (assign59870_e97245, if (-locals.var_delta_t_dn3) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn3)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn3) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn3 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn4)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn4) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn4 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn5)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn5) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn5 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn6)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn6) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn6 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn7)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn7) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn7 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn8)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn8) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn8 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn9)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn9) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn9 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn10)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn10) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn10 / assign59870_e97242)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * locals.var_t7_dn11)) } } else { (assign59870_e97245 * (((-locals.var_delta_t_dn11) * (assign59870_e97242).ln()) + (assign59870_e97244 * (locals.var_t7_dn11 / assign59870_e97242)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59870_e97247;
        locals.var_t8_dn3 = assign59870_e97247_d_n3;
        locals.var_t8_dn4 = assign59870_e97247_d_n4;
        locals.var_t8_dn5 = assign59870_e97247_d_n5;
        locals.var_t8_dn6 = assign59870_e97247_d_n6;
        locals.var_t8_dn7 = assign59870_e97247_d_n7;
        locals.var_t8_dn8 = assign59870_e97247_d_n8;
        locals.var_t8_dn9 = assign59870_e97247_d_n9;
        locals.var_t8_dn10 = assign59870_e97247_d_n10;
        locals.var_t8_dn11 = assign59870_e97247_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign59880_e97256, assign59880_e97256_d_n3, assign59880_e97256_d_n4, assign59880_e97256_d_n5, assign59880_e97256_d_n6, assign59880_e97256_d_n7, assign59880_e97256_d_n8, assign59880_e97256_d_n9, assign59880_e97256_d_n10, assign59880_e97256_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59880_e97254: f64 = (locals.var_vds * locals.var_t8);
        (assign59880_e97254, (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), (locals.var_vds * locals.var_t8_dn5), ((locals.var_vds_dn6 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn6)), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), ((locals.var_vds_dn10 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn10)), (locals.var_vds * locals.var_t8_dn11),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11,)
    }
};
        locals.var_vdseff = assign59880_e97256;
        locals.var_vdseff_dn3 = assign59880_e97256_d_n3;
        locals.var_vdseff_dn4 = assign59880_e97256_d_n4;
        locals.var_vdseff_dn5 = assign59880_e97256_d_n5;
        locals.var_vdseff_dn6 = assign59880_e97256_d_n6;
        locals.var_vdseff_dn7 = assign59880_e97256_d_n7;
        locals.var_vdseff_dn8 = assign59880_e97256_d_n8;
        locals.var_vdseff_dn9 = assign59880_e97256_d_n9;
        locals.var_vdseff_dn10 = assign59880_e97256_d_n10;
        locals.var_vdseff_dn11 = assign59880_e97256_d_n11;
        locals.var_vdseff_rv = 0.0;

        let (assign59890_e97267, assign59890_e97267_d_n3, assign59890_e97267_d_n4, assign59890_e97267_d_n5, assign59890_e97267_d_n6, assign59890_e97267_d_n7, assign59890_e97267_d_n8, assign59890_e97267_d_n9, assign59890_e97267_d_n10, assign59890_e97267_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59890_e97263: f64 = (locals.var_vdseff + locals.var_vs);
        let assign59890_e97265: f64 = (assign59890_e97263 * locals.var_inv_nvt);
        (assign59890_e97265, ((locals.var_vdseff_dn3 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn3)), ((locals.var_vdseff_dn4 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn4)), ((locals.var_vdseff_dn5 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn5)), (((locals.var_vdseff_dn6 + locals.var_vs_dn6) * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn6)), (((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn7)), ((locals.var_vdseff_dn8 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn8)), ((locals.var_vdseff_dn9 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn9)), (((locals.var_vdseff_dn10 + locals.var_vs_dn10) * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn10)), ((locals.var_vdseff_dn11 * locals.var_inv_nvt) + (assign59890_e97263 * locals.var_inv_nvt_dn11)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11,)
    }
};
        locals.var_vdeff = assign59890_e97267;
        locals.var_vdeff_dn3 = assign59890_e97267_d_n3;
        locals.var_vdeff_dn4 = assign59890_e97267_d_n4;
        locals.var_vdeff_dn5 = assign59890_e97267_d_n5;
        locals.var_vdeff_dn6 = assign59890_e97267_d_n6;
        locals.var_vdeff_dn7 = assign59890_e97267_d_n7;
        locals.var_vdeff_dn8 = assign59890_e97267_d_n8;
        locals.var_vdeff_dn9 = assign59890_e97267_d_n9;
        locals.var_vdeff_dn10 = assign59890_e97267_d_n10;
        locals.var_vdeff_dn11 = assign59890_e97267_d_n11;
        locals.var_vdeff_rv = 0.0;

        let (assign59900_e97293, assign59900_e97293_d_n3, assign59900_e97293_d_n4, assign59900_e97293_d_n5, assign59900_e97293_d_n6, assign59900_e97293_d_n7, assign59900_e97293_d_n8, assign59900_e97293_d_n9, assign59900_e97293_d_n10, assign59900_e97293_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59900_e97275: f64 = (locals.var_psip + 1.0);
        let assign59900_e97278: f64 = (locals.var_psip - 1.0);
        let assign59900_e97281: f64 = (locals.var_psip - 1.0);
        let assign59900_e97282: f64 = (assign59900_e97278 * assign59900_e97281);
        let assign59900_e97285: f64 = (0.25 * 2.0);
        let assign59900_e97287: f64 = (assign59900_e97285 * 2.0);
        let assign59900_e97288: f64 = (assign59900_e97282 + assign59900_e97287);
        let assign59900_e97289: f64 = (assign59900_e97288).sqrt();
        let assign59900_e97290: f64 = (assign59900_e97275 + assign59900_e97289);
        let assign59900_e97291: f64 = (0.5 * assign59900_e97290);
        (assign59900_e97291, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn3)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn4)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn5)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn6)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn7)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn8)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn9)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn10)) / (2.0 * assign59900_e97289)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign59900_e97281) + (assign59900_e97278 * locals.var_psip_dn11)) / (2.0 * assign59900_e97289)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59900_e97293;
        locals.var_t8_dn3 = assign59900_e97293_d_n3;
        locals.var_t8_dn4 = assign59900_e97293_d_n4;
        locals.var_t8_dn5 = assign59900_e97293_d_n5;
        locals.var_t8_dn6 = assign59900_e97293_d_n6;
        locals.var_t8_dn7 = assign59900_e97293_d_n7;
        locals.var_t8_dn8 = assign59900_e97293_d_n8;
        locals.var_t8_dn9 = assign59900_e97293_d_n9;
        locals.var_t8_dn10 = assign59900_e97293_d_n10;
        locals.var_t8_dn11 = assign59900_e97293_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign59910_e97301, assign59910_e97301_d_n3, assign59910_e97301_d_n4, assign59910_e97301_d_n5, assign59910_e97301_d_n6, assign59910_e97301_d_n7, assign59910_e97301_d_n8, assign59910_e97301_d_n9, assign59910_e97301_d_n10, assign59910_e97301_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59910_e97299: f64 = (locals.var_t8).sqrt();
        (assign59910_e97299, (locals.var_t8_dn3 / (2.0 * assign59910_e97299)), (locals.var_t8_dn4 / (2.0 * assign59910_e97299)), (locals.var_t8_dn5 / (2.0 * assign59910_e97299)), (locals.var_t8_dn6 / (2.0 * assign59910_e97299)), (locals.var_t8_dn7 / (2.0 * assign59910_e97299)), (locals.var_t8_dn8 / (2.0 * assign59910_e97299)), (locals.var_t8_dn9 / (2.0 * assign59910_e97299)), (locals.var_t8_dn10 / (2.0 * assign59910_e97299)), (locals.var_t8_dn11 / (2.0 * assign59910_e97299)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign59910_e97301;
        locals.var_sqrtpsip_dn3 = assign59910_e97301_d_n3;
        locals.var_sqrtpsip_dn4 = assign59910_e97301_d_n4;
        locals.var_sqrtpsip_dn5 = assign59910_e97301_d_n5;
        locals.var_sqrtpsip_dn6 = assign59910_e97301_d_n6;
        locals.var_sqrtpsip_dn7 = assign59910_e97301_d_n7;
        locals.var_sqrtpsip_dn8 = assign59910_e97301_d_n8;
        locals.var_sqrtpsip_dn9 = assign59910_e97301_d_n9;
        locals.var_sqrtpsip_dn10 = assign59910_e97301_d_n10;
        locals.var_sqrtpsip_dn11 = assign59910_e97301_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign59920_e97316, assign59920_e97316_d_n3, assign59920_e97316_d_n4, assign59920_e97316_d_n5, assign59920_e97316_d_n6, assign59920_e97316_d_n7, assign59920_e97316_d_n8, assign59920_e97316_d_n9, assign59920_e97316_d_n10, assign59920_e97316_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59920_e97310: f64 = (2.0 * locals.var_sqrtpsip);
        let assign59920_e97311: f64 = (locals.var_gam_edge / assign59920_e97310);
        let assign59920_e97312: f64 = (1.0 + assign59920_e97311);
        let assign59920_e97314: f64 = (assign59920_e97312 / locals.var_gam_edge);
        (assign59920_e97314, ((((((locals.var_gam_edge_dn3 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign59920_e97310) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign59920_e97310 * assign59920_e97310)) * locals.var_gam_edge) - (assign59920_e97312 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign59920_e97316;
        locals.var_t0_dn3 = assign59920_e97316_d_n3;
        locals.var_t0_dn4 = assign59920_e97316_d_n4;
        locals.var_t0_dn5 = assign59920_e97316_d_n5;
        locals.var_t0_dn6 = assign59920_e97316_d_n6;
        locals.var_t0_dn7 = assign59920_e97316_d_n7;
        locals.var_t0_dn8 = assign59920_e97316_d_n8;
        locals.var_t0_dn9 = assign59920_e97316_d_n9;
        locals.var_t0_dn10 = assign59920_e97316_d_n10;
        locals.var_t0_dn11 = assign59920_e97316_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign59930_e97329, assign59930_e97329_d_n3, assign59930_e97329_d_n4, assign59930_e97329_d_n5, assign59930_e97329_d_n6, assign59930_e97329_d_n7, assign59930_e97329_d_n8, assign59930_e97329_d_n9, assign59930_e97329_d_n10, assign59930_e97329_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59930_e97324: f64 = (2.0 * locals.var_phib_n_edge);
        let assign59930_e97325: f64 = (locals.var_psip - assign59930_e97324);
        let assign59930_e97327: f64 = (assign59930_e97325 - locals.var_vdeff);
        (assign59930_e97327, ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign59930_e97329;
        locals.var_t1_dn3 = assign59930_e97329_d_n3;
        locals.var_t1_dn4 = assign59930_e97329_d_n4;
        locals.var_t1_dn5 = assign59930_e97329_d_n5;
        locals.var_t1_dn6 = assign59930_e97329_d_n6;
        locals.var_t1_dn7 = assign59930_e97329_d_n7;
        locals.var_t1_dn8 = assign59930_e97329_d_n8;
        locals.var_t1_dn9 = assign59930_e97329_d_n9;
        locals.var_t1_dn10 = assign59930_e97329_d_n10;
        locals.var_t1_dn11 = assign59930_e97329_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign59940_e97345, assign59940_e97345_d_n3, assign59940_e97345_d_n4, assign59940_e97345_d_n5, assign59940_e97345_d_n6, assign59940_e97345_d_n7, assign59940_e97345_d_n8, assign59940_e97345_d_n9, assign59940_e97345_d_n10, assign59940_e97345_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59940_e97337: f64 = (4.0 * locals.var_t0);
        let assign59940_e97339: f64 = (assign59940_e97337 * locals.var_sqrtpsip);
        let assign59940_e97341: f64 = (assign59940_e97339).max(1e-38);
        let assign59940_e97342: f64 = (assign59940_e97341).ln();
        let assign59940_e97343: f64 = (locals.var_t1 - assign59940_e97342);
        (assign59940_e97343, (locals.var_t1_dn3 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn4 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn5 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn6 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn7 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn8 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn9 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn10 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign59940_e97341)), (locals.var_t1_dn11 - (if assign59940_e97339 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign59940_e97337 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign59940_e97341)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign59940_e97345;
        locals.var_t2_dn3 = assign59940_e97345_d_n3;
        locals.var_t2_dn4 = assign59940_e97345_d_n4;
        locals.var_t2_dn5 = assign59940_e97345_d_n5;
        locals.var_t2_dn6 = assign59940_e97345_d_n6;
        locals.var_t2_dn7 = assign59940_e97345_d_n7;
        locals.var_t2_dn8 = assign59940_e97345_d_n8;
        locals.var_t2_dn9 = assign59940_e97345_d_n9;
        locals.var_t2_dn10 = assign59940_e97345_d_n10;
        locals.var_t2_dn11 = assign59940_e97345_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign59950_e97365, assign59950_e97365_d_n3, assign59950_e97365_d_n4, assign59950_e97365_d_n5, assign59950_e97365_d_n6, assign59950_e97365_d_n7, assign59950_e97365_d_n8, assign59950_e97365_d_n9, assign59950_e97365_d_n10, assign59950_e97365_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign59950_e97353: f64 = (locals.var_t2 - 0.201491);
        let assign59950_e97357: f64 = (locals.var_t2 + 0.402982);
        let assign59950_e97358: f64 = (locals.var_t2 * assign59950_e97357);
        let assign59950_e97360: f64 = (assign59950_e97358 + 2.446562);
        let assign59950_e97361: f64 = (assign59950_e97360).sqrt();
        let assign59950_e97362: f64 = (assign59950_e97353 - assign59950_e97361);
        let assign59950_e97363: f64 = (0.5 * assign59950_e97362);
        (assign59950_e97363, (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign59950_e97361)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign59950_e97357) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign59950_e97361)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign59950_e97365;
        locals.var_t8_dn3 = assign59950_e97365_d_n3;
        locals.var_t8_dn4 = assign59950_e97365_d_n4;
        locals.var_t8_dn5 = assign59950_e97365_d_n5;
        locals.var_t8_dn6 = assign59950_e97365_d_n6;
        locals.var_t8_dn7 = assign59950_e97365_d_n7;
        locals.var_t8_dn8 = assign59950_e97365_d_n8;
        locals.var_t8_dn9 = assign59950_e97365_d_n9;
        locals.var_t8_dn10 = assign59950_e97365_d_n10;
        locals.var_t8_dn11 = assign59950_e97365_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign59960_e97372, assign59960_e97372_d_n3, assign59960_e97372_d_n4, assign59960_e97372_d_n5, assign59960_e97372_d_n6, assign59960_e97372_d_n7, assign59960_e97372_d_n8, assign59960_e97372_d_n9, assign59960_e97372_d_n10, assign59960_e97372_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11,)
    }
};
        locals.var_sqrtpsisa = assign59960_e97372;
        locals.var_sqrtpsisa_dn3 = assign59960_e97372_d_n3;
        locals.var_sqrtpsisa_dn4 = assign59960_e97372_d_n4;
        locals.var_sqrtpsisa_dn5 = assign59960_e97372_d_n5;
        locals.var_sqrtpsisa_dn6 = assign59960_e97372_d_n6;
        locals.var_sqrtpsisa_dn7 = assign59960_e97372_d_n7;
        locals.var_sqrtpsisa_dn8 = assign59960_e97372_d_n8;
        locals.var_sqrtpsisa_dn9 = assign59960_e97372_d_n9;
        locals.var_sqrtpsisa_dn10 = assign59960_e97372_d_n10;
        locals.var_sqrtpsisa_dn11 = assign59960_e97372_d_n11;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign59970_e97375: f64 = (-68.0);
        let assign59970_e97376: f64 = if locals.var_t8 <= assign59970_e97375 { 1.0 } else { 0.0 };
        locals.var_guard874 = assign59970_e97376;
        locals.var_guard874_rv = 0.0;

        let (assign59980_e97386, assign59980_e97386_d_n3, assign59980_e97386_d_n4, assign59980_e97386_d_n5, assign59980_e97386_d_n6, assign59980_e97386_d_n7, assign59980_e97386_d_n8, assign59980_e97386_d_n9, assign59980_e97386_d_n10, assign59980_e97386_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign59980_e97384: f64 = (-100.0);
        (assign59980_e97384, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign59980_e97386;
        locals.var_t4_dn3 = assign59980_e97386_d_n3;
        locals.var_t4_dn4 = assign59980_e97386_d_n4;
        locals.var_t4_dn5 = assign59980_e97386_d_n5;
        locals.var_t4_dn6 = assign59980_e97386_d_n6;
        locals.var_t4_dn7 = assign59980_e97386_d_n7;
        locals.var_t4_dn8 = assign59980_e97386_d_n8;
        locals.var_t4_dn9 = assign59980_e97386_d_n9;
        locals.var_t4_dn10 = assign59980_e97386_d_n10;
        locals.var_t4_dn11 = assign59980_e97386_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign59990_e97395, assign59990_e97395_d_n3, assign59990_e97395_d_n4, assign59990_e97395_d_n5, assign59990_e97395_d_n6, assign59990_e97395_d_n7, assign59990_e97395_d_n8, assign59990_e97395_d_n9, assign59990_e97395_d_n10, assign59990_e97395_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign59990_e97395;
        locals.var_t5_dn3 = assign59990_e97395_d_n3;
        locals.var_t5_dn4 = assign59990_e97395_d_n4;
        locals.var_t5_dn5 = assign59990_e97395_d_n5;
        locals.var_t5_dn6 = assign59990_e97395_d_n6;
        locals.var_t5_dn7 = assign59990_e97395_d_n7;
        locals.var_t5_dn8 = assign59990_e97395_d_n8;
        locals.var_t5_dn9 = assign59990_e97395_d_n9;
        locals.var_t5_dn10 = assign59990_e97395_d_n10;
        locals.var_t5_dn11 = assign59990_e97395_d_n11;
        locals.var_t5_rv = 0.0;

        let assign60000_e97400: f64 = (0.5 * locals.var_t5);
        let assign60000_e97401: f64 = (locals.var_t4 - assign60000_e97400);
        let assign60000_e97402: f64 = if locals.var_t8 < assign60000_e97401 { 1.0 } else { 0.0 };
        locals.var_guard875 = assign60000_e97402;
        locals.var_guard875_rv = 0.0;

        let (assign60010_e97414, assign60010_e97414_d_n3, assign60010_e97414_d_n4, assign60010_e97414_d_n5, assign60010_e97414_d_n6, assign60010_e97414_d_n7, assign60010_e97414_d_n8, assign60010_e97414_d_n9, assign60010_e97414_d_n10, assign60010_e97414_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign60010_e97412: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60010_e97412, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60010_e97414;
        locals.var_t3_dn3 = assign60010_e97414_d_n3;
        locals.var_t3_dn4 = assign60010_e97414_d_n4;
        locals.var_t3_dn5 = assign60010_e97414_d_n5;
        locals.var_t3_dn6 = assign60010_e97414_d_n6;
        locals.var_t3_dn7 = assign60010_e97414_d_n7;
        locals.var_t3_dn8 = assign60010_e97414_d_n8;
        locals.var_t3_dn9 = assign60010_e97414_d_n9;
        locals.var_t3_dn10 = assign60010_e97414_d_n10;
        locals.var_t3_dn11 = assign60010_e97414_d_n11;
        locals.var_t3_rv = 0.0;

        let assign60020_e97419: f64 = (0.5 * locals.var_t5);
        let assign60020_e97420: f64 = (locals.var_t4 + assign60020_e97419);
        let assign60020_e97421: f64 = if locals.var_t8 > assign60020_e97420 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign60020_e97421;
        locals.var_guard876_rv = 0.0;

        let (assign60030_e97436, assign60030_e97436_d_n3, assign60030_e97436_d_n4, assign60030_e97436_d_n5, assign60030_e97436_d_n6, assign60030_e97436_d_n7, assign60030_e97436_d_n8, assign60030_e97436_d_n9, assign60030_e97436_d_n10, assign60030_e97436_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 != 0.0)) {
        let assign60030_e97434: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60030_e97434, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60030_e97436;
        locals.var_t3_dn3 = assign60030_e97436_d_n3;
        locals.var_t3_dn4 = assign60030_e97436_d_n4;
        locals.var_t3_dn5 = assign60030_e97436_d_n5;
        locals.var_t3_dn6 = assign60030_e97436_d_n6;
        locals.var_t3_dn7 = assign60030_e97436_d_n7;
        locals.var_t3_dn8 = assign60030_e97436_d_n8;
        locals.var_t3_dn9 = assign60030_e97436_d_n9;
        locals.var_t3_dn10 = assign60030_e97436_d_n10;
        locals.var_t3_dn11 = assign60030_e97436_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign60040_e97455, assign60040_e97455_d_n3, assign60040_e97455_d_n4, assign60040_e97455_d_n5, assign60040_e97455_d_n6, assign60040_e97455_d_n7, assign60040_e97455_d_n8, assign60040_e97455_d_n9, assign60040_e97455_d_n10, assign60040_e97455_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign60040_e97451: f64 = (locals.var_t8 - locals.var_t4);
        let assign60040_e97453: f64 = (assign60040_e97451 / locals.var_t5);
        (assign60040_e97453, ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign60040_e97451 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign60040_e97455;
        locals.var_t2_dn3 = assign60040_e97455_d_n3;
        locals.var_t2_dn4 = assign60040_e97455_d_n4;
        locals.var_t2_dn5 = assign60040_e97455_d_n5;
        locals.var_t2_dn6 = assign60040_e97455_d_n6;
        locals.var_t2_dn7 = assign60040_e97455_d_n7;
        locals.var_t2_dn8 = assign60040_e97455_d_n8;
        locals.var_t2_dn9 = assign60040_e97455_d_n9;
        locals.var_t2_dn10 = assign60040_e97455_d_n10;
        locals.var_t2_dn11 = assign60040_e97455_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign60050_e97472, assign60050_e97472_d_n3, assign60050_e97472_d_n4, assign60050_e97472_d_n5, assign60050_e97472_d_n6, assign60050_e97472_d_n7, assign60050_e97472_d_n8, assign60050_e97472_d_n9, assign60050_e97472_d_n10, assign60050_e97472_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign60050_e97470: f64 = (locals.var_t2 * locals.var_t2);
        (assign60050_e97470, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60050_e97472;
        locals.var_t6_dn3 = assign60050_e97472_d_n3;
        locals.var_t6_dn4 = assign60050_e97472_d_n4;
        locals.var_t6_dn5 = assign60050_e97472_d_n5;
        locals.var_t6_dn6 = assign60050_e97472_d_n6;
        locals.var_t6_dn7 = assign60050_e97472_d_n7;
        locals.var_t6_dn8 = assign60050_e97472_d_n8;
        locals.var_t6_dn9 = assign60050_e97472_d_n9;
        locals.var_t6_dn10 = assign60050_e97472_d_n10;
        locals.var_t6_dn11 = assign60050_e97472_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign60060_e97510, assign60060_e97510_d_n3, assign60060_e97510_d_n4, assign60060_e97510_d_n5, assign60060_e97510_d_n6, assign60060_e97510_d_n7, assign60060_e97510_d_n8, assign60060_e97510_d_n9, assign60060_e97510_d_n10, assign60060_e97510_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) && (locals.var_guard875 == 0.0)) && (locals.var_guard876 == 0.0)) {
        let assign60060_e97489: f64 = (5.0 / 64.0);
        let assign60060_e97492: f64 = (0.5 * locals.var_t2);
        let assign60060_e97493: f64 = (assign60060_e97489 + assign60060_e97492);
        let assign60060_e97497: f64 = (15.0 / 16.0);
        let assign60060_e97501: f64 = (1.25 - locals.var_t6);
        let assign60060_e97502: f64 = (locals.var_t6 * assign60060_e97501);
        let assign60060_e97503: f64 = (assign60060_e97497 - assign60060_e97502);
        let assign60060_e97504: f64 = (locals.var_t6 * assign60060_e97503);
        let assign60060_e97505: f64 = (assign60060_e97493 + assign60060_e97504);
        let assign60060_e97506: f64 = (locals.var_t5 * assign60060_e97505);
        let assign60060_e97507: f64 = (locals.var_t4 + assign60060_e97506);
        let assign60060_e97508: f64 = { let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60060_e97508, ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign60060_e97505) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign60060_e97503) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign60060_e97501) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60060_e97510;
        locals.var_t3_dn3 = assign60060_e97510_d_n3;
        locals.var_t3_dn4 = assign60060_e97510_d_n4;
        locals.var_t3_dn5 = assign60060_e97510_d_n5;
        locals.var_t3_dn6 = assign60060_e97510_d_n6;
        locals.var_t3_dn7 = assign60060_e97510_d_n7;
        locals.var_t3_dn8 = assign60060_e97510_d_n8;
        locals.var_t3_dn9 = assign60060_e97510_d_n9;
        locals.var_t3_dn10 = assign60060_e97510_d_n10;
        locals.var_t3_dn11 = assign60060_e97510_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign60070_e97542, assign60070_e97542_d_n3, assign60070_e97542_d_n4, assign60070_e97542_d_n5, assign60070_e97542_d_n6, assign60070_e97542_d_n7, assign60070_e97542_d_n8, assign60070_e97542_d_n9, assign60070_e97542_d_n10, assign60070_e97542_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign60070_e97520: f64 = (1.0 + locals.var_t1);
        let assign60070_e97522: f64 = (assign60070_e97520 - locals.var_t8);
        let assign60070_e97525: f64 = (2.0 * locals.var_t0);
        let assign60070_e97528: f64 = (locals.var_t3 * 2.0);
        let assign60070_e97530: f64 = (assign60070_e97528 * locals.var_t0);
        let assign60070_e97533: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign60070_e97534: f64 = (assign60070_e97530 + assign60070_e97533);
        let assign60070_e97535: f64 = (assign60070_e97525 * assign60070_e97534);
        let assign60070_e97537: f64 = (assign60070_e97535).max(1e-38);
        let assign60070_e97538: f64 = (assign60070_e97537).ln();
        let assign60070_e97539: f64 = (assign60070_e97522 - assign60070_e97538);
        let assign60070_e97540: f64 = (locals.var_t3 * assign60070_e97539);
        (assign60070_e97540, ((locals.var_t3_dn3 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn4 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn5 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn6 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn7 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn8 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn9 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn10 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60070_e97537)))), ((locals.var_t3_dn11 * assign60070_e97539) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign60070_e97535 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign60070_e97534) + (assign60070_e97525 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60070_e97528 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60070_e97537)))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11,)
    }
};
        locals.var_qdeff_edge = assign60070_e97542;
        locals.var_qdeff_edge_dn3 = assign60070_e97542_d_n3;
        locals.var_qdeff_edge_dn4 = assign60070_e97542_d_n4;
        locals.var_qdeff_edge_dn5 = assign60070_e97542_d_n5;
        locals.var_qdeff_edge_dn6 = assign60070_e97542_d_n6;
        locals.var_qdeff_edge_dn7 = assign60070_e97542_d_n7;
        locals.var_qdeff_edge_dn8 = assign60070_e97542_d_n8;
        locals.var_qdeff_edge_dn9 = assign60070_e97542_d_n9;
        locals.var_qdeff_edge_dn10 = assign60070_e97542_d_n10;
        locals.var_qdeff_edge_dn11 = assign60070_e97542_d_n11;
        locals.var_qdeff_edge_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_210(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign60080_e97553, assign60080_e97553_d_n3, assign60080_e97553_d_n4, assign60080_e97553_d_n5, assign60080_e97553_d_n6, assign60080_e97553_d_n7, assign60080_e97553_d_n8, assign60080_e97553_d_n9, assign60080_e97553_d_n10, assign60080_e97553_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60080_e97551: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60080_e97551, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60080_e97553;
        locals.var_t3_dn3 = assign60080_e97553_d_n3;
        locals.var_t3_dn4 = assign60080_e97553_d_n4;
        locals.var_t3_dn5 = assign60080_e97553_d_n5;
        locals.var_t3_dn6 = assign60080_e97553_d_n6;
        locals.var_t3_dn7 = assign60080_e97553_d_n7;
        locals.var_t3_dn8 = assign60080_e97553_d_n8;
        locals.var_t3_dn9 = assign60080_e97553_d_n9;
        locals.var_t3_dn10 = assign60080_e97553_d_n10;
        locals.var_t3_dn11 = assign60080_e97553_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign60090_e97565, assign60090_e97565_d_n3, assign60090_e97565_d_n4, assign60090_e97565_d_n5, assign60090_e97565_d_n6, assign60090_e97565_d_n7, assign60090_e97565_d_n8, assign60090_e97565_d_n9, assign60090_e97565_d_n10, assign60090_e97565_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60090_e97563: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign60090_e97563, (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11,)
    }
};
        locals.var_sqrtpsisainv = assign60090_e97565;
        locals.var_sqrtpsisainv_dn3 = assign60090_e97565_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign60090_e97565_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign60090_e97565_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign60090_e97565_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign60090_e97565_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign60090_e97565_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign60090_e97565_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign60090_e97565_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign60090_e97565_d_n11;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign60100_e97598, assign60100_e97598_d_n3, assign60100_e97598_d_n4, assign60100_e97598_d_n5, assign60100_e97598_d_n6, assign60100_e97598_d_n7, assign60100_e97598_d_n8, assign60100_e97598_d_n9, assign60100_e97598_d_n10, assign60100_e97598_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60100_e97575: f64 = (2.0 * locals.var_t3);
        let assign60100_e97578: f64 = (locals.var_t3 * 2.0);
        let assign60100_e97580: f64 = (assign60100_e97578 * locals.var_t0);
        let assign60100_e97583: f64 = (locals.var_t3 * 2.0);
        let assign60100_e97585: f64 = (assign60100_e97583 * locals.var_t0);
        let assign60100_e97588: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign60100_e97589: f64 = (assign60100_e97585 + assign60100_e97588);
        let assign60100_e97590: f64 = (assign60100_e97580 * assign60100_e97589);
        let assign60100_e97592: f64 = (assign60100_e97590).max(1e-38);
        let assign60100_e97593: f64 = (assign60100_e97592).ln();
        let assign60100_e97594: f64 = (assign60100_e97575 + assign60100_e97593);
        let assign60100_e97596: f64 = (assign60100_e97594 - locals.var_t1);
        (assign60100_e97596, (((2.0 * locals.var_t3_dn3) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn3)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn4)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn5)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn6)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn7)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn8)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn9)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn10)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign60100_e97590 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60100_e97578 * locals.var_t0_dn11)) * assign60100_e97589) + (assign60100_e97580 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60100_e97583 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60100_e97592)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60100_e97598;
        locals.var_t4_dn3 = assign60100_e97598_d_n3;
        locals.var_t4_dn4 = assign60100_e97598_d_n4;
        locals.var_t4_dn5 = assign60100_e97598_d_n5;
        locals.var_t4_dn6 = assign60100_e97598_d_n6;
        locals.var_t4_dn7 = assign60100_e97598_d_n7;
        locals.var_t4_dn8 = assign60100_e97598_d_n8;
        locals.var_t4_dn9 = assign60100_e97598_d_n9;
        locals.var_t4_dn10 = assign60100_e97598_d_n10;
        locals.var_t4_dn11 = assign60100_e97598_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign60110_e97622, assign60110_e97622_d_n3, assign60110_e97622_d_n4, assign60110_e97622_d_n5, assign60110_e97622_d_n6, assign60110_e97622_d_n7, assign60110_e97622_d_n8, assign60110_e97622_d_n9, assign60110_e97622_d_n10, assign60110_e97622_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60110_e97609: f64 = (1.0 / locals.var_t3);
        let assign60110_e97610: f64 = (2.0 + assign60110_e97609);
        let assign60110_e97613: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60110_e97616: f64 = (locals.var_t0 * locals.var_t3);
        let assign60110_e97618: f64 = (assign60110_e97616 + locals.var_sqrtpsisa);
        let assign60110_e97619: f64 = (assign60110_e97613 / assign60110_e97618);
        let assign60110_e97620: f64 = (assign60110_e97610 + assign60110_e97619);
        (assign60110_e97620, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60110_e97618 * assign60110_e97618))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60110_e97618) - (assign60110_e97613 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60110_e97618 * assign60110_e97618))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60110_e97622;
        locals.var_t5_dn3 = assign60110_e97622_d_n3;
        locals.var_t5_dn4 = assign60110_e97622_d_n4;
        locals.var_t5_dn5 = assign60110_e97622_d_n5;
        locals.var_t5_dn6 = assign60110_e97622_d_n6;
        locals.var_t5_dn7 = assign60110_e97622_d_n7;
        locals.var_t5_dn8 = assign60110_e97622_d_n8;
        locals.var_t5_dn9 = assign60110_e97622_d_n9;
        locals.var_t5_dn10 = assign60110_e97622_d_n10;
        locals.var_t5_dn11 = assign60110_e97622_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign60120_e97636, assign60120_e97636_d_n3, assign60120_e97636_d_n4, assign60120_e97636_d_n5, assign60120_e97636_d_n6, assign60120_e97636_d_n7, assign60120_e97636_d_n8, assign60120_e97636_d_n9, assign60120_e97636_d_n10, assign60120_e97636_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60120_e97633: f64 = (locals.var_t4 / locals.var_t5);
        let assign60120_e97634: f64 = (locals.var_t3 - assign60120_e97633);
        (assign60120_e97634, (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60120_e97636;
        locals.var_t3_dn3 = assign60120_e97636_d_n3;
        locals.var_t3_dn4 = assign60120_e97636_d_n4;
        locals.var_t3_dn5 = assign60120_e97636_d_n5;
        locals.var_t3_dn6 = assign60120_e97636_d_n6;
        locals.var_t3_dn7 = assign60120_e97636_d_n7;
        locals.var_t3_dn8 = assign60120_e97636_d_n8;
        locals.var_t3_dn9 = assign60120_e97636_d_n9;
        locals.var_t3_dn10 = assign60120_e97636_d_n10;
        locals.var_t3_dn11 = assign60120_e97636_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign60130_e97669, assign60130_e97669_d_n3, assign60130_e97669_d_n4, assign60130_e97669_d_n5, assign60130_e97669_d_n6, assign60130_e97669_d_n7, assign60130_e97669_d_n8, assign60130_e97669_d_n9, assign60130_e97669_d_n10, assign60130_e97669_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60130_e97646: f64 = (2.0 * locals.var_t3);
        let assign60130_e97649: f64 = (locals.var_t3 * 2.0);
        let assign60130_e97651: f64 = (assign60130_e97649 * locals.var_t0);
        let assign60130_e97654: f64 = (locals.var_t3 * 2.0);
        let assign60130_e97656: f64 = (assign60130_e97654 * locals.var_t0);
        let assign60130_e97659: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign60130_e97660: f64 = (assign60130_e97656 + assign60130_e97659);
        let assign60130_e97661: f64 = (assign60130_e97651 * assign60130_e97660);
        let assign60130_e97663: f64 = (assign60130_e97661).max(1e-38);
        let assign60130_e97664: f64 = (assign60130_e97663).ln();
        let assign60130_e97665: f64 = (assign60130_e97646 + assign60130_e97664);
        let assign60130_e97667: f64 = (assign60130_e97665 - locals.var_t1);
        (assign60130_e97667, (((2.0 * locals.var_t3_dn3) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn3)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn4)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn5)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn6)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn7)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn8)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn9)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn10)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign60130_e97661 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60130_e97649 * locals.var_t0_dn11)) * assign60130_e97660) + (assign60130_e97651 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign60130_e97654 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60130_e97663)) - locals.var_t1_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60130_e97669;
        locals.var_t4_dn3 = assign60130_e97669_d_n3;
        locals.var_t4_dn4 = assign60130_e97669_d_n4;
        locals.var_t4_dn5 = assign60130_e97669_d_n5;
        locals.var_t4_dn6 = assign60130_e97669_d_n6;
        locals.var_t4_dn7 = assign60130_e97669_d_n7;
        locals.var_t4_dn8 = assign60130_e97669_d_n8;
        locals.var_t4_dn9 = assign60130_e97669_d_n9;
        locals.var_t4_dn10 = assign60130_e97669_d_n10;
        locals.var_t4_dn11 = assign60130_e97669_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign60140_e97693, assign60140_e97693_d_n3, assign60140_e97693_d_n4, assign60140_e97693_d_n5, assign60140_e97693_d_n6, assign60140_e97693_d_n7, assign60140_e97693_d_n8, assign60140_e97693_d_n9, assign60140_e97693_d_n10, assign60140_e97693_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60140_e97680: f64 = (1.0 / locals.var_t3);
        let assign60140_e97681: f64 = (2.0 + assign60140_e97680);
        let assign60140_e97684: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60140_e97687: f64 = (locals.var_t0 * locals.var_t3);
        let assign60140_e97689: f64 = (assign60140_e97687 + locals.var_sqrtpsisa);
        let assign60140_e97690: f64 = (assign60140_e97684 / assign60140_e97689);
        let assign60140_e97691: f64 = (assign60140_e97681 + assign60140_e97690);
        (assign60140_e97691, ((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60140_e97689 * assign60140_e97689))), ((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60140_e97689) - (assign60140_e97684 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60140_e97689 * assign60140_e97689))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60140_e97693;
        locals.var_t5_dn3 = assign60140_e97693_d_n3;
        locals.var_t5_dn4 = assign60140_e97693_d_n4;
        locals.var_t5_dn5 = assign60140_e97693_d_n5;
        locals.var_t5_dn6 = assign60140_e97693_d_n6;
        locals.var_t5_dn7 = assign60140_e97693_d_n7;
        locals.var_t5_dn8 = assign60140_e97693_d_n8;
        locals.var_t5_dn9 = assign60140_e97693_d_n9;
        locals.var_t5_dn10 = assign60140_e97693_d_n10;
        locals.var_t5_dn11 = assign60140_e97693_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign60150_e97721, assign60150_e97721_d_n3, assign60150_e97721_d_n4, assign60150_e97721_d_n5, assign60150_e97721_d_n6, assign60150_e97721_d_n7, assign60150_e97721_d_n8, assign60150_e97721_d_n9, assign60150_e97721_d_n10, assign60150_e97721_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60150_e97703: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60150_e97706: f64 = (locals.var_t0 * locals.var_t3);
        let assign60150_e97708: f64 = (assign60150_e97706 + locals.var_sqrtpsisa);
        let assign60150_e97709: f64 = (assign60150_e97703 / assign60150_e97708);
        let assign60150_e97712: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign60150_e97715: f64 = (locals.var_t0 * locals.var_t3);
        let assign60150_e97717: f64 = (assign60150_e97715 + locals.var_sqrtpsisa);
        let assign60150_e97718: f64 = (assign60150_e97712 / assign60150_e97717);
        let assign60150_e97719: f64 = (assign60150_e97709 * assign60150_e97718);
        (assign60150_e97719, ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60150_e97717 * assign60150_e97717)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60150_e97708) - (assign60150_e97703 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign60150_e97717) - (assign60150_e97712 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60150_e97717 * assign60150_e97717)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60150_e97721;
        locals.var_t6_dn3 = assign60150_e97721_d_n3;
        locals.var_t6_dn4 = assign60150_e97721_d_n4;
        locals.var_t6_dn5 = assign60150_e97721_d_n5;
        locals.var_t6_dn6 = assign60150_e97721_d_n6;
        locals.var_t6_dn7 = assign60150_e97721_d_n7;
        locals.var_t6_dn8 = assign60150_e97721_d_n8;
        locals.var_t6_dn9 = assign60150_e97721_d_n9;
        locals.var_t6_dn10 = assign60150_e97721_d_n10;
        locals.var_t6_dn11 = assign60150_e97721_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign60160_e97754, assign60160_e97754_d_n3, assign60160_e97754_d_n4, assign60160_e97754_d_n5, assign60160_e97754_d_n6, assign60160_e97754_d_n7, assign60160_e97754_d_n8, assign60160_e97754_d_n9, assign60160_e97754_d_n10, assign60160_e97754_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign60160_e97731: f64 = (1.0 * __rspice_inv_cse_0);
        let assign60160_e97734: f64 = (1.0 * __rspice_inv_cse_0);
        let assign60160_e97735: f64 = (assign60160_e97731 * assign60160_e97734);
        let assign60160_e97736: f64 = (-assign60160_e97735);
        let assign60160_e97740: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign60160_e97742: f64 = (assign60160_e97740 * locals.var_sqrtpsisa);
        let assign60160_e97745: f64 = (locals.var_t0 * locals.var_t3);
        let assign60160_e97747: f64 = (assign60160_e97745 + locals.var_sqrtpsisa);
        let assign60160_e97748: f64 = (assign60160_e97742 * assign60160_e97747);
        let assign60160_e97749: f64 = (1.0 / assign60160_e97748);
        let assign60160_e97750: f64 = (assign60160_e97736 - assign60160_e97749);
        let assign60160_e97752: f64 = (assign60160_e97750 - locals.var_t6);
        (assign60160_e97752, (((-(((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn3)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn3), (((-(((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn4)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn4), (((-(((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn5)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn5), (((-(((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn6)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn6), (((-(((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn7)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn7), (((-(((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn8)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn8), (((-(((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn9)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn9), (((-(((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn10)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn10), (((-(((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-(((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign60160_e97740 * locals.var_sqrtpsisa_dn11)) * assign60160_e97747) + (assign60160_e97742 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign60160_e97748 * assign60160_e97748)))) - locals.var_t6_dn11),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign60160_e97754;
        locals.var_t7_dn3 = assign60160_e97754_d_n3;
        locals.var_t7_dn4 = assign60160_e97754_d_n4;
        locals.var_t7_dn5 = assign60160_e97754_d_n5;
        locals.var_t7_dn6 = assign60160_e97754_d_n6;
        locals.var_t7_dn7 = assign60160_e97754_d_n7;
        locals.var_t7_dn8 = assign60160_e97754_d_n8;
        locals.var_t7_dn9 = assign60160_e97754_d_n9;
        locals.var_t7_dn10 = assign60160_e97754_d_n10;
        locals.var_t7_dn11 = assign60160_e97754_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign60170_e97780, assign60170_e97780_d_n3, assign60170_e97780_d_n4, assign60170_e97780_d_n5, assign60170_e97780_d_n6, assign60170_e97780_d_n7, assign60170_e97780_d_n8, assign60170_e97780_d_n9, assign60170_e97780_d_n10, assign60170_e97780_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard874 == 0.0)) {
        let assign60170_e97765: f64 = (locals.var_t4 / locals.var_t5);
        let assign60170_e97769: f64 = (locals.var_t4 * locals.var_t7);
        let assign60170_e97772: f64 = (2.0 * locals.var_t5);
        let assign60170_e97774: f64 = (assign60170_e97772 * locals.var_t5);
        let assign60170_e97775: f64 = (assign60170_e97769 / assign60170_e97774);
        let assign60170_e97776: f64 = (1.0 + assign60170_e97775);
        let assign60170_e97777: f64 = (assign60170_e97765 * assign60170_e97776);
        let assign60170_e97778: f64 = (locals.var_t3 - assign60170_e97777);
        (assign60170_e97778, (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn3)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn4)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn5)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn6)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn7)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn8)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn9)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn10)))) / (assign60170_e97774 * assign60170_e97774))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign60170_e97772 * locals.var_t5_dn11)))) / (assign60170_e97774 * assign60170_e97774))))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11,)
    }
};
        locals.var_qdeff_edge = assign60170_e97780;
        locals.var_qdeff_edge_dn3 = assign60170_e97780_d_n3;
        locals.var_qdeff_edge_dn4 = assign60170_e97780_d_n4;
        locals.var_qdeff_edge_dn5 = assign60170_e97780_d_n5;
        locals.var_qdeff_edge_dn6 = assign60170_e97780_d_n6;
        locals.var_qdeff_edge_dn7 = assign60170_e97780_d_n7;
        locals.var_qdeff_edge_dn8 = assign60170_e97780_d_n8;
        locals.var_qdeff_edge_dn9 = assign60170_e97780_d_n9;
        locals.var_qdeff_edge_dn10 = assign60170_e97780_d_n10;
        locals.var_qdeff_edge_dn11 = assign60170_e97780_d_n11;
        locals.var_qdeff_edge_rv = 0.0;

        let (assign60180_e97806, assign60180_e97806_d_n3, assign60180_e97806_d_n4, assign60180_e97806_d_n5, assign60180_e97806_d_n6, assign60180_e97806_d_n7, assign60180_e97806_d_n8, assign60180_e97806_d_n9, assign60180_e97806_d_n10, assign60180_e97806_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60180_e97788: f64 = (locals.var_psip + 1.0);
        let assign60180_e97791: f64 = (locals.var_psip - 1.0);
        let assign60180_e97794: f64 = (locals.var_psip - 1.0);
        let assign60180_e97795: f64 = (assign60180_e97791 * assign60180_e97794);
        let assign60180_e97798: f64 = (0.25 * 2.0);
        let assign60180_e97800: f64 = (assign60180_e97798 * 2.0);
        let assign60180_e97801: f64 = (assign60180_e97795 + assign60180_e97800);
        let assign60180_e97802: f64 = (assign60180_e97801).sqrt();
        let assign60180_e97803: f64 = (assign60180_e97788 + assign60180_e97802);
        let assign60180_e97804: f64 = (0.5 * assign60180_e97803);
        (assign60180_e97804, (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn3)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn4)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn5)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn6)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn7)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn8)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn9)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn10)) / (2.0 * assign60180_e97802)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign60180_e97794) + (assign60180_e97791 * locals.var_psip_dn11)) / (2.0 * assign60180_e97802)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11,)
    }
};
        locals.var_psipclamp = assign60180_e97806;
        locals.var_psipclamp_dn3 = assign60180_e97806_d_n3;
        locals.var_psipclamp_dn4 = assign60180_e97806_d_n4;
        locals.var_psipclamp_dn5 = assign60180_e97806_d_n5;
        locals.var_psipclamp_dn6 = assign60180_e97806_d_n6;
        locals.var_psipclamp_dn7 = assign60180_e97806_d_n7;
        locals.var_psipclamp_dn8 = assign60180_e97806_d_n8;
        locals.var_psipclamp_dn9 = assign60180_e97806_d_n9;
        locals.var_psipclamp_dn10 = assign60180_e97806_d_n10;
        locals.var_psipclamp_dn11 = assign60180_e97806_d_n11;
        locals.var_psipclamp_rv = 0.0;

        let (assign60190_e97814, assign60190_e97814_d_n3, assign60190_e97814_d_n4, assign60190_e97814_d_n5, assign60190_e97814_d_n6, assign60190_e97814_d_n7, assign60190_e97814_d_n8, assign60190_e97814_d_n9, assign60190_e97814_d_n10, assign60190_e97814_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60190_e97812: f64 = (locals.var_psipclamp).sqrt();
        (assign60190_e97812, (locals.var_psipclamp_dn3 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn4 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn5 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn6 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn7 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn8 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn9 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn10 / (2.0 * assign60190_e97812)), (locals.var_psipclamp_dn11 / (2.0 * assign60190_e97812)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11,)
    }
};
        locals.var_sqrtpsip = assign60190_e97814;
        locals.var_sqrtpsip_dn3 = assign60190_e97814_d_n3;
        locals.var_sqrtpsip_dn4 = assign60190_e97814_d_n4;
        locals.var_sqrtpsip_dn5 = assign60190_e97814_d_n5;
        locals.var_sqrtpsip_dn6 = assign60190_e97814_d_n6;
        locals.var_sqrtpsip_dn7 = assign60190_e97814_d_n7;
        locals.var_sqrtpsip_dn8 = assign60190_e97814_d_n8;
        locals.var_sqrtpsip_dn9 = assign60190_e97814_d_n9;
        locals.var_sqrtpsip_dn10 = assign60190_e97814_d_n10;
        locals.var_sqrtpsip_dn11 = assign60190_e97814_d_n11;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign60200_e97827, assign60200_e97827_d_n3, assign60200_e97827_d_n4, assign60200_e97827_d_n5, assign60200_e97827_d_n6, assign60200_e97827_d_n7, assign60200_e97827_d_n8, assign60200_e97827_d_n9, assign60200_e97827_d_n10, assign60200_e97827_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60200_e97821: f64 = (locals.var_psip - locals.var_qs_edge);
        let assign60200_e97823: f64 = (assign60200_e97821 - locals.var_qdeff_edge);
        let assign60200_e97825: f64 = (assign60200_e97823 - 1.0);
        (assign60200_e97825, ((locals.var_psip_dn3 - locals.var_qs_edge_dn3) - locals.var_qdeff_edge_dn3), ((locals.var_psip_dn4 - locals.var_qs_edge_dn4) - locals.var_qdeff_edge_dn4), ((locals.var_psip_dn5 - locals.var_qs_edge_dn5) - locals.var_qdeff_edge_dn5), ((locals.var_psip_dn6 - locals.var_qs_edge_dn6) - locals.var_qdeff_edge_dn6), ((locals.var_psip_dn7 - locals.var_qs_edge_dn7) - locals.var_qdeff_edge_dn7), ((locals.var_psip_dn8 - locals.var_qs_edge_dn8) - locals.var_qdeff_edge_dn8), ((locals.var_psip_dn9 - locals.var_qs_edge_dn9) - locals.var_qdeff_edge_dn9), ((locals.var_psip_dn10 - locals.var_qs_edge_dn10) - locals.var_qdeff_edge_dn10), ((locals.var_psip_dn11 - locals.var_qs_edge_dn11) - locals.var_qdeff_edge_dn11),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11,)
    }
};
        locals.var_psiavg = assign60200_e97827;
        locals.var_psiavg_dn3 = assign60200_e97827_d_n3;
        locals.var_psiavg_dn4 = assign60200_e97827_d_n4;
        locals.var_psiavg_dn5 = assign60200_e97827_d_n5;
        locals.var_psiavg_dn6 = assign60200_e97827_d_n6;
        locals.var_psiavg_dn7 = assign60200_e97827_d_n7;
        locals.var_psiavg_dn8 = assign60200_e97827_d_n8;
        locals.var_psiavg_dn9 = assign60200_e97827_d_n9;
        locals.var_psiavg_dn10 = assign60200_e97827_d_n10;
        locals.var_psiavg_dn11 = assign60200_e97827_d_n11;
        locals.var_psiavg_rv = 0.0;

        let (assign60210_e97853, assign60210_e97853_d_n3, assign60210_e97853_d_n4, assign60210_e97853_d_n5, assign60210_e97853_d_n6, assign60210_e97853_d_n7, assign60210_e97853_d_n8, assign60210_e97853_d_n9, assign60210_e97853_d_n10, assign60210_e97853_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60210_e97835: f64 = (locals.var_psiavg + 1.0);
        let assign60210_e97838: f64 = (locals.var_psiavg - 1.0);
        let assign60210_e97841: f64 = (locals.var_psiavg - 1.0);
        let assign60210_e97842: f64 = (assign60210_e97838 * assign60210_e97841);
        let assign60210_e97845: f64 = (0.25 * 2.0);
        let assign60210_e97847: f64 = (assign60210_e97845 * 2.0);
        let assign60210_e97848: f64 = (assign60210_e97842 + assign60210_e97847);
        let assign60210_e97849: f64 = (assign60210_e97848).sqrt();
        let assign60210_e97850: f64 = (assign60210_e97835 + assign60210_e97849);
        let assign60210_e97851: f64 = (0.5 * assign60210_e97850);
        (assign60210_e97851, (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn3)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn4)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn5)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn6)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn7)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn8)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn9)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn10)) / (2.0 * assign60210_e97849)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign60210_e97841) + (assign60210_e97838 * locals.var_psiavg_dn11)) / (2.0 * assign60210_e97849)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign60210_e97853;
        locals.var_t0_dn3 = assign60210_e97853_d_n3;
        locals.var_t0_dn4 = assign60210_e97853_d_n4;
        locals.var_t0_dn5 = assign60210_e97853_d_n5;
        locals.var_t0_dn6 = assign60210_e97853_d_n6;
        locals.var_t0_dn7 = assign60210_e97853_d_n7;
        locals.var_t0_dn8 = assign60210_e97853_d_n8;
        locals.var_t0_dn9 = assign60210_e97853_d_n9;
        locals.var_t0_dn10 = assign60210_e97853_d_n10;
        locals.var_t0_dn11 = assign60210_e97853_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign60220_e97861, assign60220_e97861_d_n3, assign60220_e97861_d_n4, assign60220_e97861_d_n5, assign60220_e97861_d_n6, assign60220_e97861_d_n7, assign60220_e97861_d_n8, assign60220_e97861_d_n9, assign60220_e97861_d_n10, assign60220_e97861_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60220_e97859: f64 = (locals.var_t0).sqrt();
        (assign60220_e97859, (locals.var_t0_dn3 / (2.0 * assign60220_e97859)), (locals.var_t0_dn4 / (2.0 * assign60220_e97859)), (locals.var_t0_dn5 / (2.0 * assign60220_e97859)), (locals.var_t0_dn6 / (2.0 * assign60220_e97859)), (locals.var_t0_dn7 / (2.0 * assign60220_e97859)), (locals.var_t0_dn8 / (2.0 * assign60220_e97859)), (locals.var_t0_dn9 / (2.0 * assign60220_e97859)), (locals.var_t0_dn10 / (2.0 * assign60220_e97859)), (locals.var_t0_dn11 / (2.0 * assign60220_e97859)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign60220_e97861;
        locals.var_t2_dn3 = assign60220_e97861_d_n3;
        locals.var_t2_dn4 = assign60220_e97861_d_n4;
        locals.var_t2_dn5 = assign60220_e97861_d_n5;
        locals.var_t2_dn6 = assign60220_e97861_d_n6;
        locals.var_t2_dn7 = assign60220_e97861_d_n7;
        locals.var_t2_dn8 = assign60220_e97861_d_n8;
        locals.var_t2_dn9 = assign60220_e97861_d_n9;
        locals.var_t2_dn10 = assign60220_e97861_d_n10;
        locals.var_t2_dn11 = assign60220_e97861_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign60230_e97874, assign60230_e97874_d_n3, assign60230_e97874_d_n4, assign60230_e97874_d_n5, assign60230_e97874_d_n6, assign60230_e97874_d_n7, assign60230_e97874_d_n8, assign60230_e97874_d_n9, assign60230_e97874_d_n10, assign60230_e97874_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60230_e97870: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign60230_e97871: f64 = (locals.var_gam_edge / assign60230_e97870);
        let assign60230_e97872: f64 = (1.0 + assign60230_e97871);
        (assign60230_e97872, (((locals.var_gam_edge_dn3 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn4 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn5 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn6 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn7 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn8 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn9 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn10 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign60230_e97870 * assign60230_e97870)), (((locals.var_gam_edge_dn11 * assign60230_e97870) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign60230_e97870 * assign60230_e97870)),)
    } else {
        (locals.var_nq_edge, locals.var_nq_edge_dn3, locals.var_nq_edge_dn4, locals.var_nq_edge_dn5, locals.var_nq_edge_dn6, locals.var_nq_edge_dn7, locals.var_nq_edge_dn8, locals.var_nq_edge_dn9, locals.var_nq_edge_dn10, locals.var_nq_edge_dn11,)
    }
};
        locals.var_nq_edge = assign60230_e97874;
        locals.var_nq_edge_dn3 = assign60230_e97874_d_n3;
        locals.var_nq_edge_dn4 = assign60230_e97874_d_n4;
        locals.var_nq_edge_dn5 = assign60230_e97874_d_n5;
        locals.var_nq_edge_dn6 = assign60230_e97874_d_n6;
        locals.var_nq_edge_dn7 = assign60230_e97874_d_n7;
        locals.var_nq_edge_dn8 = assign60230_e97874_d_n8;
        locals.var_nq_edge_dn9 = assign60230_e97874_d_n9;
        locals.var_nq_edge_dn10 = assign60230_e97874_d_n10;
        locals.var_nq_edge_dn11 = assign60230_e97874_d_n11;
        locals.var_nq_edge_rv = 0.0;

        let (assign60240_e97909, assign60240_e97909_d_n3, assign60240_e97909_d_n4, assign60240_e97909_d_n5, assign60240_e97909_d_n6, assign60240_e97909_d_n7, assign60240_e97909_d_n8, assign60240_e97909_d_n9, assign60240_e97909_d_n10, assign60240_e97909_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60240_e97881: f64 = (2.0 * p.p2);
        let assign60240_e97883: f64 = (assign60240_e97881 * locals.var_nq_edge);
        let assign60240_e97885: f64 = (assign60240_e97883 * locals.var_ueff);
        let assign60240_e97887: f64 = (assign60240_e97885 * p.p1147);
        let assign60240_e97889: f64 = (assign60240_e97887 / locals.var_leff);
        let assign60240_e97891: f64 = (assign60240_e97889 * locals.var_cox);
        let assign60240_e97893: f64 = (assign60240_e97891 * locals.var_nvt);
        let assign60240_e97895: f64 = (assign60240_e97893 * locals.var_nvt);
        let assign60240_e97898: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign60240_e97901: f64 = (1.0 + locals.var_qs_edge);
        let assign60240_e97903: f64 = (assign60240_e97901 + locals.var_qdeff_edge);
        let assign60240_e97904: f64 = (assign60240_e97898 * assign60240_e97903);
        let assign60240_e97905: f64 = (assign60240_e97895 * assign60240_e97904);
        let assign60240_e97907: f64 = (assign60240_e97905 * locals.var_moc);
        (assign60240_e97907, ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn3) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn3)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn3)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn3 + locals.var_qdeff_edge_dn3))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn3)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn4) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn4)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn4)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn4 + locals.var_qdeff_edge_dn4))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn4)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn5) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn5)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn5)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn5 + locals.var_qdeff_edge_dn5))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn5)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn6) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn6)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn6)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn6 + locals.var_qdeff_edge_dn6))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn6)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn7) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn7)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn7)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn7 + locals.var_qdeff_edge_dn7))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn7)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn8) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn8)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn8)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn8 + locals.var_qdeff_edge_dn8))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn8)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn9) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn9)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn9)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn9 + locals.var_qdeff_edge_dn9))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn9)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn10) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn10)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn10)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn10 + locals.var_qdeff_edge_dn10))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn10)), ((((((((((((((assign60240_e97881 * locals.var_nq_edge_dn11) * locals.var_ueff) + (assign60240_e97883 * locals.var_ueff_dn11)) * p.p1147) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign60240_e97891 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign60240_e97893 * locals.var_nvt_dn11)) * assign60240_e97904) + (assign60240_e97895 * (((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) * assign60240_e97903) + (assign60240_e97898 * (locals.var_qs_edge_dn11 + locals.var_qdeff_edge_dn11))))) * locals.var_moc) + (assign60240_e97905 * locals.var_moc_dn11)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn3, locals.var_ids_edge_dn4, locals.var_ids_edge_dn5, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9, locals.var_ids_edge_dn10, locals.var_ids_edge_dn11,)
    }
};
        locals.var_ids_edge = assign60240_e97909;
        locals.var_ids_edge_dn3 = assign60240_e97909_d_n3;
        locals.var_ids_edge_dn4 = assign60240_e97909_d_n4;
        locals.var_ids_edge_dn5 = assign60240_e97909_d_n5;
        locals.var_ids_edge_dn6 = assign60240_e97909_d_n6;
        locals.var_ids_edge_dn7 = assign60240_e97909_d_n7;
        locals.var_ids_edge_dn8 = assign60240_e97909_d_n8;
        locals.var_ids_edge_dn9 = assign60240_e97909_d_n9;
        locals.var_ids_edge_dn10 = assign60240_e97909_d_n10;
        locals.var_ids_edge_dn11 = assign60240_e97909_d_n11;
        locals.var_ids_edge_rv = 0.0;

        let (assign60250_e97918, assign60250_e97918_d_n3, assign60250_e97918_d_n4, assign60250_e97918_d_n5, assign60250_e97918_d_n6, assign60250_e97918_d_n7, assign60250_e97918_d_n8, assign60250_e97918_d_n9, assign60250_e97918_d_n10, assign60250_e97918_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60250_e97916: f64 = (locals.var_ids_edge + locals.var_ids);
        (assign60250_e97916, (locals.var_ids_edge_dn3 + locals.var_ids_dn3), (locals.var_ids_edge_dn4 + locals.var_ids_dn4), (locals.var_ids_edge_dn5 + locals.var_ids_dn5), (locals.var_ids_edge_dn6 + locals.var_ids_dn6), (locals.var_ids_edge_dn7 + locals.var_ids_dn7), (locals.var_ids_edge_dn8 + locals.var_ids_dn8), (locals.var_ids_edge_dn9 + locals.var_ids_dn9), (locals.var_ids_edge_dn10 + locals.var_ids_dn10), (locals.var_ids_edge_dn11 + locals.var_ids_dn11),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign60250_e97918;
        locals.var_ids_dn3 = assign60250_e97918_d_n3;
        locals.var_ids_dn4 = assign60250_e97918_d_n4;
        locals.var_ids_dn5 = assign60250_e97918_d_n5;
        locals.var_ids_dn6 = assign60250_e97918_d_n6;
        locals.var_ids_dn7 = assign60250_e97918_d_n7;
        locals.var_ids_dn8 = assign60250_e97918_d_n8;
        locals.var_ids_dn9 = assign60250_e97918_d_n9;
        locals.var_ids_dn10 = assign60250_e97918_d_n10;
        locals.var_ids_dn11 = assign60250_e97918_d_n11;
        locals.var_ids_rv = 0.0;

        let (assign60260_e97927,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60260_e97925: f64 = (p.p1012 * p.p1316);
        (assign60260_e97925,)
    } else {
        (locals.var_noia_edge,)
    }
};
        locals.var_noia_edge = assign60260_e97927;
        locals.var_noia_edge_rv = 0.0;

        let (assign60270_e97936,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60270_e97934: f64 = (p.p1013 * p.p1316);
        (assign60270_e97934,)
    } else {
        (locals.var_noib_edge,)
    }
};
        locals.var_noib_edge = assign60270_e97936;
        locals.var_noib_edge_rv = 0.0;

        let (assign60280_e97945,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60280_e97943: f64 = (p.p1014 * p.p1316);
        (assign60280_e97943,)
    } else {
        (locals.var_noic_edge,)
    }
};
        locals.var_noic_edge = assign60280_e97945;
        locals.var_noic_edge_rv = 0.0;

        let (assign60290_e97956,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60290_e97953: f64 = (2.0 * locals.var_lintnoi_i);
        let assign60290_e97954: f64 = (locals.var_leff - assign60290_e97953);
        (assign60290_e97954,)
    } else {
        (locals.var_leffnoi_edge,)
    }
};
        locals.var_leffnoi_edge = assign60290_e97956;
        locals.var_leffnoi_edge_rv = 0.0;

        let (assign60300_e97965,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60300_e97963: f64 = (locals.var_leffnoi_edge * locals.var_leffnoi_edge);
        (assign60300_e97963,)
    } else {
        (locals.var_leffnoisq_edge,)
    }
};
        locals.var_leffnoisq_edge = assign60300_e97965;
        locals.var_leffnoisq_edge_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_211(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign60310_e97980, assign60310_e97980_d_n3, assign60310_e97980_d_n4, assign60310_e97980_d_n5, assign60310_e97980_d_n6, assign60310_e97980_d_n7, assign60310_e97980_d_n8, assign60310_e97980_d_n9, assign60310_e97980_d_n10, assign60310_e97980_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60310_e97972: f64 = (locals.var_vt / 1.602176462e-19);
        let assign60310_e97975: f64 = (locals.var_cox + locals.var_cdep);
        let assign60310_e97977: f64 = (assign60310_e97975 + locals.var_citedge_i);
        let assign60310_e97978: f64 = (assign60310_e97972 * assign60310_e97977);
        (assign60310_e97978, (assign60310_e97972 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.602176462e-19) * assign60310_e97977) + (assign60310_e97972 * locals.var_cdep_dn4)), (((locals.var_vt_dn5 / 1.602176462e-19) * assign60310_e97977) + (assign60310_e97972 * locals.var_cdep_dn5)), (assign60310_e97972 * locals.var_cdep_dn6), (assign60310_e97972 * locals.var_cdep_dn7), (assign60310_e97972 * locals.var_cdep_dn8), (assign60310_e97972 * locals.var_cdep_dn9), (assign60310_e97972 * locals.var_cdep_dn10), (assign60310_e97972 * locals.var_cdep_dn11),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11,)
    }
};
        locals.var_nstar = assign60310_e97980;
        locals.var_nstar_dn3 = assign60310_e97980_d_n3;
        locals.var_nstar_dn4 = assign60310_e97980_d_n4;
        locals.var_nstar_dn5 = assign60310_e97980_d_n5;
        locals.var_nstar_dn6 = assign60310_e97980_d_n6;
        locals.var_nstar_dn7 = assign60310_e97980_d_n7;
        locals.var_nstar_dn8 = assign60310_e97980_d_n8;
        locals.var_nstar_dn9 = assign60310_e97980_d_n9;
        locals.var_nstar_dn10 = assign60310_e97980_d_n10;
        locals.var_nstar_dn11 = assign60310_e97980_d_n11;
        locals.var_nstar_rv = 0.0;

        let (assign60320_e97997, assign60320_e97997_d_n3, assign60320_e97997_d_n4, assign60320_e97997_d_n5, assign60320_e97997_d_n6, assign60320_e97997_d_n7, assign60320_e97997_d_n8, assign60320_e97997_d_n9, assign60320_e97997_d_n10, assign60320_e97997_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60320_e97987: f64 = (2.0 * locals.var_nq_edge);
        let assign60320_e97989: f64 = (assign60320_e97987 * locals.var_cox);
        let assign60320_e97991: f64 = (assign60320_e97989 * locals.var_vt);
        let assign60320_e97993: f64 = (assign60320_e97991 * locals.var_qdeff_edge);
        let assign60320_e97995: f64 = (assign60320_e97993 / 1.602176462e-19);
        (assign60320_e97995, ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn3)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign60320_e97989 * locals.var_vt_dn4)) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn4)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) + (assign60320_e97989 * locals.var_vt_dn5)) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn5)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn6)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn7)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn8)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn9)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn10)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign60320_e97991 * locals.var_qdeff_edge_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11,)
    }
};
        locals.var_nl = assign60320_e97997;
        locals.var_nl_dn3 = assign60320_e97997_d_n3;
        locals.var_nl_dn4 = assign60320_e97997_d_n4;
        locals.var_nl_dn5 = assign60320_e97997_d_n5;
        locals.var_nl_dn6 = assign60320_e97997_d_n6;
        locals.var_nl_dn7 = assign60320_e97997_d_n7;
        locals.var_nl_dn8 = assign60320_e97997_d_n8;
        locals.var_nl_dn9 = assign60320_e97997_d_n9;
        locals.var_nl_dn10 = assign60320_e97997_d_n10;
        locals.var_nl_dn11 = assign60320_e97997_d_n11;
        locals.var_nl_rv = 0.0;

        let (assign60330_e98015, assign60330_e98015_d_n3, assign60330_e98015_d_n4, assign60330_e98015_d_n5, assign60330_e98015_d_n6, assign60330_e98015_d_n7, assign60330_e98015_d_n8, assign60330_e98015_d_n9, assign60330_e98015_d_n10, assign60330_e98015_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60330_e98004: f64 = (1.602176462e-19 * 1.602176462e-19);
        let assign60330_e98006: f64 = (assign60330_e98004 * 1.602176462e-19);
        let assign60330_e98008: f64 = (assign60330_e98006 * locals.var_vt);
        let assign60330_e98010: f64 = (locals.var_ids_edge).abs();
        let assign60330_e98011: f64 = (assign60330_e98008 * assign60330_e98010);
        let assign60330_e98013: f64 = (assign60330_e98011 * locals.var_ueff);
        (assign60330_e98013, (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn3 } else { (-locals.var_ids_edge_dn3) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn3)), (((((assign60330_e98006 * locals.var_vt_dn4) * assign60330_e98010) + (assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn4 } else { (-locals.var_ids_edge_dn4) })) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn4)), (((((assign60330_e98006 * locals.var_vt_dn5) * assign60330_e98010) + (assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn5 } else { (-locals.var_ids_edge_dn5) })) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn5)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn6 } else { (-locals.var_ids_edge_dn6) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn6)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn7 } else { (-locals.var_ids_edge_dn7) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn7)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn8 } else { (-locals.var_ids_edge_dn8) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn8)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn9 } else { (-locals.var_ids_edge_dn9) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn9)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn10 } else { (-locals.var_ids_edge_dn10) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn10)), (((assign60330_e98008 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn11 } else { (-locals.var_ids_edge_dn11) }) * locals.var_ueff) + (assign60330_e98011 * locals.var_ueff_dn11)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11,)
    }
};
        locals.var_t0a = assign60330_e98015;
        locals.var_t0a_dn3 = assign60330_e98015_d_n3;
        locals.var_t0a_dn4 = assign60330_e98015_d_n4;
        locals.var_t0a_dn5 = assign60330_e98015_d_n5;
        locals.var_t0a_dn6 = assign60330_e98015_d_n6;
        locals.var_t0a_dn7 = assign60330_e98015_d_n7;
        locals.var_t0a_dn8 = assign60330_e98015_d_n8;
        locals.var_t0a_dn9 = assign60330_e98015_d_n9;
        locals.var_t0a_dn10 = assign60330_e98015_d_n10;
        locals.var_t0a_dn11 = assign60330_e98015_d_n11;
        locals.var_t0a_rv = 0.0;

        let (assign60340_e98028, assign60340_e98028_d_n3, assign60340_e98028_d_n4, assign60340_e98028_d_n5, assign60340_e98028_d_n6, assign60340_e98028_d_n7, assign60340_e98028_d_n8, assign60340_e98028_d_n9, assign60340_e98028_d_n10, assign60340_e98028_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60340_e98022: f64 = (1.602176462e-19 * locals.var_vt);
        let assign60340_e98024: f64 = (assign60340_e98022 * locals.var_ids_edge);
        let assign60340_e98026: f64 = (assign60340_e98024 * locals.var_ids_edge);
        (assign60340_e98026, (((assign60340_e98022 * locals.var_ids_edge_dn3) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn3)), (((((1.602176462e-19 * locals.var_vt_dn4) * locals.var_ids_edge) + (assign60340_e98022 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn4)), (((((1.602176462e-19 * locals.var_vt_dn5) * locals.var_ids_edge) + (assign60340_e98022 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn5)), (((assign60340_e98022 * locals.var_ids_edge_dn6) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn6)), (((assign60340_e98022 * locals.var_ids_edge_dn7) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn7)), (((assign60340_e98022 * locals.var_ids_edge_dn8) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn8)), (((assign60340_e98022 * locals.var_ids_edge_dn9) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn9)), (((assign60340_e98022 * locals.var_ids_edge_dn10) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn10)), (((assign60340_e98022 * locals.var_ids_edge_dn11) * locals.var_ids_edge) + (assign60340_e98024 * locals.var_ids_edge_dn11)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11,)
    }
};
        locals.var_t0b = assign60340_e98028;
        locals.var_t0b_dn3 = assign60340_e98028_d_n3;
        locals.var_t0b_dn4 = assign60340_e98028_d_n4;
        locals.var_t0b_dn5 = assign60340_e98028_d_n5;
        locals.var_t0b_dn6 = assign60340_e98028_d_n6;
        locals.var_t0b_dn7 = assign60340_e98028_d_n7;
        locals.var_t0b_dn8 = assign60340_e98028_d_n8;
        locals.var_t0b_dn9 = assign60340_e98028_d_n9;
        locals.var_t0b_dn10 = assign60340_e98028_d_n10;
        locals.var_t0b_dn11 = assign60340_e98028_d_n11;
        locals.var_t0b_rv = 0.0;

        let (assign60350_e98045, assign60350_e98045_d_n3, assign60350_e98045_d_n4, assign60350_e98045_d_n5, assign60350_e98045_d_n6, assign60350_e98045_d_n7, assign60350_e98045_d_n8, assign60350_e98045_d_n9, assign60350_e98045_d_n10, assign60350_e98045_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60350_e98036: f64 = (locals.var_noib_edge * locals.var_nl);
        let assign60350_e98037: f64 = (locals.var_noia_edge + assign60350_e98036);
        let assign60350_e98040: f64 = (locals.var_noic_edge * locals.var_nl);
        let assign60350_e98042: f64 = (assign60350_e98040 * locals.var_nl);
        let assign60350_e98043: f64 = (assign60350_e98037 + assign60350_e98042);
        (assign60350_e98043, ((locals.var_noib_edge * locals.var_nl_dn3) + (((locals.var_noic_edge * locals.var_nl_dn3) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn3))), ((locals.var_noib_edge * locals.var_nl_dn4) + (((locals.var_noic_edge * locals.var_nl_dn4) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn4))), ((locals.var_noib_edge * locals.var_nl_dn5) + (((locals.var_noic_edge * locals.var_nl_dn5) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn5))), ((locals.var_noib_edge * locals.var_nl_dn6) + (((locals.var_noic_edge * locals.var_nl_dn6) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn6))), ((locals.var_noib_edge * locals.var_nl_dn7) + (((locals.var_noic_edge * locals.var_nl_dn7) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn7))), ((locals.var_noib_edge * locals.var_nl_dn8) + (((locals.var_noic_edge * locals.var_nl_dn8) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn8))), ((locals.var_noib_edge * locals.var_nl_dn9) + (((locals.var_noic_edge * locals.var_nl_dn9) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn9))), ((locals.var_noib_edge * locals.var_nl_dn10) + (((locals.var_noic_edge * locals.var_nl_dn10) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn10))), ((locals.var_noib_edge * locals.var_nl_dn11) + (((locals.var_noic_edge * locals.var_nl_dn11) * locals.var_nl) + (assign60350_e98040 * locals.var_nl_dn11))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11,)
    }
};
        locals.var_t0c = assign60350_e98045;
        locals.var_t0c_dn3 = assign60350_e98045_d_n3;
        locals.var_t0c_dn4 = assign60350_e98045_d_n4;
        locals.var_t0c_dn5 = assign60350_e98045_d_n5;
        locals.var_t0c_dn6 = assign60350_e98045_d_n6;
        locals.var_t0c_dn7 = assign60350_e98045_d_n7;
        locals.var_t0c_dn8 = assign60350_e98045_d_n8;
        locals.var_t0c_dn9 = assign60350_e98045_d_n9;
        locals.var_t0c_dn10 = assign60350_e98045_d_n10;
        locals.var_t0c_dn11 = assign60350_e98045_d_n11;
        locals.var_t0c_rv = 0.0;

        let (assign60360_e98058, assign60360_e98058_d_n3, assign60360_e98058_d_n4, assign60360_e98058_d_n5, assign60360_e98058_d_n6, assign60360_e98058_d_n7, assign60360_e98058_d_n8, assign60360_e98058_d_n9, assign60360_e98058_d_n10, assign60360_e98058_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60360_e98052: f64 = (locals.var_nl + locals.var_nstar);
        let assign60360_e98055: f64 = (locals.var_nl + locals.var_nstar);
        let assign60360_e98056: f64 = (assign60360_e98052 * assign60360_e98055);
        (assign60360_e98056, (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign60360_e98055) + (assign60360_e98052 * (locals.var_nl_dn11 + locals.var_nstar_dn11))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11,)
    }
};
        locals.var_t0d = assign60360_e98058;
        locals.var_t0d_dn3 = assign60360_e98058_d_n3;
        locals.var_t0d_dn4 = assign60360_e98058_d_n4;
        locals.var_t0d_dn5 = assign60360_e98058_d_n5;
        locals.var_t0d_dn6 = assign60360_e98058_d_n6;
        locals.var_t0d_dn7 = assign60360_e98058_d_n7;
        locals.var_t0d_dn8 = assign60360_e98058_d_n8;
        locals.var_t0d_dn9 = assign60360_e98058_d_n9;
        locals.var_t0d_dn10 = assign60360_e98058_d_n10;
        locals.var_t0d_dn11 = assign60360_e98058_d_n11;
        locals.var_t0d_rv = 0.0;

        let (assign60370_e98069, assign60370_e98069_d_n4, assign60370_e98069_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60370_e98065: f64 = (locals.var_noia_edge * 1.602176462e-19);
        let assign60370_e98067: f64 = (assign60370_e98065 * locals.var_vt);
        (assign60370_e98067, (assign60370_e98065 * locals.var_vt_dn4), (assign60370_e98065 * locals.var_vt_dn5),)
    } else {
        (locals.var_t0e, locals.var_t0e_dn4, locals.var_t0e_dn5,)
    }
};
        locals.var_t0e = assign60370_e98069;
        locals.var_t0e_dn4 = assign60370_e98069_d_n4;
        locals.var_t0e_dn5 = assign60370_e98069_d_n5;
        locals.var_t0e_rv = 0.0;

        let (assign60380_e98086, assign60380_e98086_d_n3, assign60380_e98086_d_n4, assign60380_e98086_d_n5, assign60380_e98086_d_n6, assign60380_e98086_d_n7, assign60380_e98086_d_n8, assign60380_e98086_d_n9, assign60380_e98086_d_n10, assign60380_e98086_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60380_e98076: f64 = (2.0 * locals.var_nq_edge);
        let assign60380_e98078: f64 = (assign60380_e98076 * locals.var_cox);
        let assign60380_e98080: f64 = (assign60380_e98078 * locals.var_vt);
        let assign60380_e98082: f64 = (assign60380_e98080 * locals.var_qs_edge);
        let assign60380_e98084: f64 = (assign60380_e98082 / 1.602176462e-19);
        (assign60380_e98084, ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn3)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign60380_e98078 * locals.var_vt_dn4)) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn4)) / 1.602176462e-19), (((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) + (assign60380_e98078 * locals.var_vt_dn5)) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn5)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn6)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn7)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn8)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn9)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn10)) / 1.602176462e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign60380_e98080 * locals.var_qs_edge_dn11)) / 1.602176462e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11,)
    }
};
        locals.var_n0 = assign60380_e98086;
        locals.var_n0_dn3 = assign60380_e98086_d_n3;
        locals.var_n0_dn4 = assign60380_e98086_d_n4;
        locals.var_n0_dn5 = assign60380_e98086_d_n5;
        locals.var_n0_dn6 = assign60380_e98086_d_n6;
        locals.var_n0_dn7 = assign60380_e98086_d_n7;
        locals.var_n0_dn8 = assign60380_e98086_d_n8;
        locals.var_n0_dn9 = assign60380_e98086_d_n9;
        locals.var_n0_dn10 = assign60380_e98086_d_n10;
        locals.var_n0_dn11 = assign60380_e98086_d_n11;
        locals.var_n0_rv = 0.0;

        let (assign60390_e98104, assign60390_e98104_d_n3, assign60390_e98104_d_n4, assign60390_e98104_d_n5, assign60390_e98104_d_n6, assign60390_e98104_d_n7, assign60390_e98104_d_n8, assign60390_e98104_d_n9, assign60390_e98104_d_n10, assign60390_e98104_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60390_e98094: f64 = (locals.var_n0 + locals.var_nstar);
        let assign60390_e98097: f64 = (locals.var_nl + locals.var_nstar);
        let assign60390_e98098: f64 = (assign60390_e98094 / assign60390_e98097);
        let assign60390_e98100: f64 = (assign60390_e98098).max(1e-38);
        let assign60390_e98101: f64 = (assign60390_e98100).ln();
        let assign60390_e98102: f64 = (locals.var_noia_edge * assign60390_e98101);
        (assign60390_e98102, (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (locals.var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign60390_e98097) - (assign60390_e98094 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign60390_e98104;
        locals.var_t1_dn3 = assign60390_e98104_d_n3;
        locals.var_t1_dn4 = assign60390_e98104_d_n4;
        locals.var_t1_dn5 = assign60390_e98104_d_n5;
        locals.var_t1_dn6 = assign60390_e98104_d_n6;
        locals.var_t1_dn7 = assign60390_e98104_d_n7;
        locals.var_t1_dn8 = assign60390_e98104_d_n8;
        locals.var_t1_dn9 = assign60390_e98104_d_n9;
        locals.var_t1_dn10 = assign60390_e98104_d_n10;
        locals.var_t1_dn11 = assign60390_e98104_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign60400_e98115, assign60400_e98115_d_n3, assign60400_e98115_d_n4, assign60400_e98115_d_n5, assign60400_e98115_d_n6, assign60400_e98115_d_n7, assign60400_e98115_d_n8, assign60400_e98115_d_n9, assign60400_e98115_d_n10, assign60400_e98115_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60400_e98112: f64 = (locals.var_n0 - locals.var_nl);
        let assign60400_e98113: f64 = (locals.var_noib_edge * assign60400_e98112);
        (assign60400_e98113, (locals.var_noib_edge * (locals.var_n0_dn3 - locals.var_nl_dn3)), (locals.var_noib_edge * (locals.var_n0_dn4 - locals.var_nl_dn4)), (locals.var_noib_edge * (locals.var_n0_dn5 - locals.var_nl_dn5)), (locals.var_noib_edge * (locals.var_n0_dn6 - locals.var_nl_dn6)), (locals.var_noib_edge * (locals.var_n0_dn7 - locals.var_nl_dn7)), (locals.var_noib_edge * (locals.var_n0_dn8 - locals.var_nl_dn8)), (locals.var_noib_edge * (locals.var_n0_dn9 - locals.var_nl_dn9)), (locals.var_noib_edge * (locals.var_n0_dn10 - locals.var_nl_dn10)), (locals.var_noib_edge * (locals.var_n0_dn11 - locals.var_nl_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign60400_e98115;
        locals.var_t2_dn3 = assign60400_e98115_d_n3;
        locals.var_t2_dn4 = assign60400_e98115_d_n4;
        locals.var_t2_dn5 = assign60400_e98115_d_n5;
        locals.var_t2_dn6 = assign60400_e98115_d_n6;
        locals.var_t2_dn7 = assign60400_e98115_d_n7;
        locals.var_t2_dn8 = assign60400_e98115_d_n8;
        locals.var_t2_dn9 = assign60400_e98115_d_n9;
        locals.var_t2_dn10 = assign60400_e98115_d_n10;
        locals.var_t2_dn11 = assign60400_e98115_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign60410_e98132, assign60410_e98132_d_n3, assign60410_e98132_d_n4, assign60410_e98132_d_n5, assign60410_e98132_d_n6, assign60410_e98132_d_n7, assign60410_e98132_d_n8, assign60410_e98132_d_n9, assign60410_e98132_d_n10, assign60410_e98132_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60410_e98122: f64 = (0.5 * locals.var_noic_edge);
        let assign60410_e98125: f64 = (locals.var_n0 * locals.var_n0);
        let assign60410_e98128: f64 = (locals.var_nl * locals.var_nl);
        let assign60410_e98129: f64 = (assign60410_e98125 - assign60410_e98128);
        let assign60410_e98130: f64 = (assign60410_e98122 * assign60410_e98129);
        (assign60410_e98130, (assign60410_e98122 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign60410_e98122 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign60410_e98122 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign60410_e98122 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign60410_e98122 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign60410_e98122 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign60410_e98122 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign60410_e98122 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign60410_e98122 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign60410_e98132;
        locals.var_t3_dn3 = assign60410_e98132_d_n3;
        locals.var_t3_dn4 = assign60410_e98132_d_n4;
        locals.var_t3_dn5 = assign60410_e98132_d_n5;
        locals.var_t3_dn6 = assign60410_e98132_d_n6;
        locals.var_t3_dn7 = assign60410_e98132_d_n7;
        locals.var_t3_dn8 = assign60410_e98132_d_n8;
        locals.var_t3_dn9 = assign60410_e98132_d_n9;
        locals.var_t3_dn10 = assign60410_e98132_d_n10;
        locals.var_t3_dn11 = assign60410_e98132_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign60420_e98145, assign60420_e98145_d_n3, assign60420_e98145_d_n4, assign60420_e98145_d_n5, assign60420_e98145_d_n6, assign60420_e98145_d_n7, assign60420_e98145_d_n8, assign60420_e98145_d_n9, assign60420_e98145_d_n10, assign60420_e98145_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60420_e98139: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign60420_e98141: f64 = (assign60420_e98139 * p.p1147);
        let assign60420_e98143: f64 = (assign60420_e98141 * p.p2);
        (assign60420_e98143, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign60420_e98145;
        locals.var_t4_dn3 = assign60420_e98145_d_n3;
        locals.var_t4_dn4 = assign60420_e98145_d_n4;
        locals.var_t4_dn5 = assign60420_e98145_d_n5;
        locals.var_t4_dn6 = assign60420_e98145_d_n6;
        locals.var_t4_dn7 = assign60420_e98145_d_n7;
        locals.var_t4_dn8 = assign60420_e98145_d_n8;
        locals.var_t4_dn9 = assign60420_e98145_d_n9;
        locals.var_t4_dn10 = assign60420_e98145_d_n10;
        locals.var_t4_dn11 = assign60420_e98145_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign60430_e98170, assign60430_e98170_d_n3, assign60430_e98170_d_n4, assign60430_e98170_d_n5, assign60430_e98170_d_n6, assign60430_e98170_d_n7, assign60430_e98170_d_n8, assign60430_e98170_d_n9, assign60430_e98170_d_n10, assign60430_e98170_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60430_e98152: f64 = (locals.var_t0a / locals.var_t0);
        let assign60430_e98155: f64 = (locals.var_t1 + locals.var_t2);
        let assign60430_e98157: f64 = (assign60430_e98155 + locals.var_t3);
        let assign60430_e98158: f64 = (assign60430_e98152 * assign60430_e98157);
        let assign60430_e98161: f64 = (locals.var_t0b / locals.var_t4);
        let assign60430_e98163: f64 = (assign60430_e98161 * locals.var_delclm);
        let assign60430_e98165: f64 = (assign60430_e98163 * locals.var_t0c);
        let assign60430_e98167: f64 = (assign60430_e98165 / locals.var_t0d);
        let assign60430_e98168: f64 = (assign60430_e98158 + assign60430_e98167);
        (assign60430_e98168, ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign60430_e98161 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign60430_e98163 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign60430_e98165 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11,)
    }
};
        locals.var_ssi = assign60430_e98170;
        locals.var_ssi_dn3 = assign60430_e98170_d_n3;
        locals.var_ssi_dn4 = assign60430_e98170_d_n4;
        locals.var_ssi_dn5 = assign60430_e98170_d_n5;
        locals.var_ssi_dn6 = assign60430_e98170_d_n6;
        locals.var_ssi_dn7 = assign60430_e98170_d_n7;
        locals.var_ssi_dn8 = assign60430_e98170_d_n8;
        locals.var_ssi_dn9 = assign60430_e98170_d_n9;
        locals.var_ssi_dn10 = assign60430_e98170_d_n10;
        locals.var_ssi_dn11 = assign60430_e98170_d_n11;
        locals.var_ssi_rv = 0.0;

        let (assign60440_e98187, assign60440_e98187_d_n3, assign60440_e98187_d_n4, assign60440_e98187_d_n5, assign60440_e98187_d_n6, assign60440_e98187_d_n7, assign60440_e98187_d_n8, assign60440_e98187_d_n9, assign60440_e98187_d_n10, assign60440_e98187_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60440_e98177: f64 = (p.p1147 * p.p2);
        let assign60440_e98179: f64 = (assign60440_e98177 * locals.var_leffnoi_edge);
        let assign60440_e98181: f64 = (assign60440_e98179 * 10000000000.0);
        let assign60440_e98183: f64 = (assign60440_e98181 * locals.var_nstar);
        let assign60440_e98185: f64 = (assign60440_e98183 * locals.var_nstar);
        (assign60440_e98185, (((assign60440_e98181 * locals.var_nstar_dn3) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn3)), (((assign60440_e98181 * locals.var_nstar_dn4) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn4)), (((assign60440_e98181 * locals.var_nstar_dn5) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn5)), (((assign60440_e98181 * locals.var_nstar_dn6) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn6)), (((assign60440_e98181 * locals.var_nstar_dn7) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn7)), (((assign60440_e98181 * locals.var_nstar_dn8) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn8)), (((assign60440_e98181 * locals.var_nstar_dn9) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn9)), (((assign60440_e98181 * locals.var_nstar_dn10) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn10)), (((assign60440_e98181 * locals.var_nstar_dn11) * locals.var_nstar) + (assign60440_e98183 * locals.var_nstar_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign60440_e98187;
        locals.var_t5_dn3 = assign60440_e98187_d_n3;
        locals.var_t5_dn4 = assign60440_e98187_d_n4;
        locals.var_t5_dn5 = assign60440_e98187_d_n5;
        locals.var_t5_dn6 = assign60440_e98187_d_n6;
        locals.var_t5_dn7 = assign60440_e98187_d_n7;
        locals.var_t5_dn8 = assign60440_e98187_d_n8;
        locals.var_t5_dn9 = assign60440_e98187_d_n9;
        locals.var_t5_dn10 = assign60440_e98187_d_n10;
        locals.var_t5_dn11 = assign60440_e98187_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign60450_e98200, assign60450_e98200_d_n3, assign60450_e98200_d_n4, assign60450_e98200_d_n5, assign60450_e98200_d_n6, assign60450_e98200_d_n7, assign60450_e98200_d_n8, assign60450_e98200_d_n9, assign60450_e98200_d_n10, assign60450_e98200_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60450_e98194: f64 = (locals.var_t0e / locals.var_t5);
        let assign60450_e98196: f64 = (assign60450_e98194 * locals.var_ids_edge);
        let assign60450_e98198: f64 = (assign60450_e98196 * locals.var_ids_edge);
        (assign60450_e98198, (((((-((locals.var_t0e * locals.var_t5_dn3) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn5)), (((((-((locals.var_t0e * locals.var_t5_dn6) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn6)), (((((-((locals.var_t0e * locals.var_t5_dn7) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn7)), (((((-((locals.var_t0e * locals.var_t5_dn8) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn8)), (((((-((locals.var_t0e * locals.var_t5_dn9) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn9)), (((((-((locals.var_t0e * locals.var_t5_dn10) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn10)), (((((-((locals.var_t0e * locals.var_t5_dn11) / (locals.var_t5 * locals.var_t5))) * locals.var_ids_edge) + (assign60450_e98194 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign60450_e98196 * locals.var_ids_edge_dn11)),)
    } else {
        (locals.var_swi, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11,)
    }
};
        locals.var_swi = assign60450_e98200;
        locals.var_swi_dn3 = assign60450_e98200_d_n3;
        locals.var_swi_dn4 = assign60450_e98200_d_n4;
        locals.var_swi_dn5 = assign60450_e98200_d_n5;
        locals.var_swi_dn6 = assign60450_e98200_d_n6;
        locals.var_swi_dn7 = assign60450_e98200_d_n7;
        locals.var_swi_dn8 = assign60450_e98200_d_n8;
        locals.var_swi_dn9 = assign60450_e98200_d_n9;
        locals.var_swi_dn10 = assign60450_e98200_d_n10;
        locals.var_swi_dn11 = assign60450_e98200_d_n11;
        locals.var_swi_rv = 0.0;

        let (assign60460_e98209, assign60460_e98209_d_n3, assign60460_e98209_d_n4, assign60460_e98209_d_n5, assign60460_e98209_d_n6, assign60460_e98209_d_n7, assign60460_e98209_d_n8, assign60460_e98209_d_n9, assign60460_e98209_d_n10, assign60460_e98209_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) {
        let assign60460_e98207: f64 = (locals.var_swi + locals.var_ssi);
        (assign60460_e98207, (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign60460_e98209;
        locals.var_t6_dn3 = assign60460_e98209_d_n3;
        locals.var_t6_dn4 = assign60460_e98209_d_n4;
        locals.var_t6_dn5 = assign60460_e98209_d_n5;
        locals.var_t6_dn6 = assign60460_e98209_d_n6;
        locals.var_t6_dn7 = assign60460_e98209_d_n7;
        locals.var_t6_dn8 = assign60460_e98209_d_n8;
        locals.var_t6_dn9 = assign60460_e98209_d_n9;
        locals.var_t6_dn10 = assign60460_e98209_d_n10;
        locals.var_t6_dn11 = assign60460_e98209_d_n11;
        locals.var_t6_rv = 0.0;

        let assign60470_e98212: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign60470_e98212;
        locals.var_guard877_rv = 0.0;

        let (assign60480_e98225, assign60480_e98225_d_n3, assign60480_e98225_d_n4, assign60480_e98225_d_n5, assign60480_e98225_d_n6, assign60480_e98225_d_n7, assign60480_e98225_d_n8, assign60480_e98225_d_n9, assign60480_e98225_d_n10, assign60480_e98225_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign60480_e98221: f64 = (locals.var_ssi * locals.var_swi);
        let assign60480_e98223: f64 = (assign60480_e98221 / locals.var_t6);
        (assign60480_e98223, (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign60480_e98221 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign60480_e98225;
        locals.var_t7_dn3 = assign60480_e98225_d_n3;
        locals.var_t7_dn4 = assign60480_e98225_d_n4;
        locals.var_t7_dn5 = assign60480_e98225_d_n5;
        locals.var_t7_dn6 = assign60480_e98225_d_n6;
        locals.var_t7_dn7 = assign60480_e98225_d_n7;
        locals.var_t7_dn8 = assign60480_e98225_d_n8;
        locals.var_t7_dn9 = assign60480_e98225_d_n9;
        locals.var_t7_dn10 = assign60480_e98225_d_n10;
        locals.var_t7_dn11 = assign60480_e98225_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign60490_e98242, assign60490_e98242_d_n3, assign60490_e98242_d_n4, assign60490_e98242_d_n5, assign60490_e98242_d_n6, assign60490_e98242_d_n7, assign60490_e98242_d_n8, assign60490_e98242_d_n9, assign60490_e98242_d_n10, assign60490_e98242_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard867 != 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign60490_e98236: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign60490_e98238: f64 = (assign60490_e98236).powf(p.p1318);
        let assign60490_e98239: f64 = (p.p1317 * assign60490_e98238);
        let assign60490_e98240: f64 = (1.0 + assign60490_e98239);
        (assign60490_e98240, (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign60490_e98238 * (p.p1318 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign60490_e98236))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign60490_e98242;
        locals.var_t8_dn3 = assign60490_e98242_d_n3;
        locals.var_t8_dn4 = assign60490_e98242_d_n4;
        locals.var_t8_dn5 = assign60490_e98242_d_n5;
        locals.var_t8_dn6 = assign60490_e98242_d_n6;
        locals.var_t8_dn7 = assign60490_e98242_d_n7;
        locals.var_t8_dn8 = assign60490_e98242_d_n8;
        locals.var_t8_dn9 = assign60490_e98242_d_n9;
        locals.var_t8_dn10 = assign60490_e98242_d_n10;
        locals.var_t8_dn11 = assign60490_e98242_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign60520_e98276, assign60520_e98276_d_n3, assign60520_e98276_d_n4, assign60520_e98276_d_n5, assign60520_e98276_d_n6, assign60520_e98276_d_n7, assign60520_e98276_d_n8, assign60520_e98276_d_n9, assign60520_e98276_d_n10, assign60520_e98276_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60520_e98269: f64 = (locals.var_qbi + locals.var_qovb);
        let assign60520_e98271: f64 = (assign60520_e98269 + locals.var_qbsj);
        let assign60520_e98273: f64 = (assign60520_e98271 + locals.var_qbdj);
        let assign60520_e98274: f64 = (locals.var_devsign * assign60520_e98273);
        (assign60520_e98274, (locals.var_devsign * ((locals.var_qbi_dn3 + locals.var_qbsj_dn3) + locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qbi_dn4 + locals.var_qbsj_dn4) + locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qbi_dn5 + locals.var_qbsj_dn5) + locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qbi_dn6 + locals.var_qbsj_dn6) + locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qbi_dn7 + locals.var_qbsj_dn7) + locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qbi_dn8 + locals.var_qbsj_dn8) + locals.var_qbdj_dn8)), (locals.var_devsign * (((locals.var_qbi_dn9 + locals.var_qovb_dn9) + locals.var_qbsj_dn9) + locals.var_qbdj_dn9)), (locals.var_devsign * (((locals.var_qbi_dn10 + locals.var_qovb_dn10) + locals.var_qbsj_dn10) + locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qbi_dn11 + locals.var_qbsj_dn11) + locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qb_2, locals.var_qb_2_dn3, locals.var_qb_2_dn4, locals.var_qb_2_dn5, locals.var_qb_2_dn6, locals.var_qb_2_dn7, locals.var_qb_2_dn8, locals.var_qb_2_dn9, locals.var_qb_2_dn10, locals.var_qb_2_dn11,)
    }
};
        locals.var_qb_2 = assign60520_e98276;
        locals.var_qb_2_dn3 = assign60520_e98276_d_n3;
        locals.var_qb_2_dn4 = assign60520_e98276_d_n4;
        locals.var_qb_2_dn5 = assign60520_e98276_d_n5;
        locals.var_qb_2_dn6 = assign60520_e98276_d_n6;
        locals.var_qb_2_dn7 = assign60520_e98276_d_n7;
        locals.var_qb_2_dn8 = assign60520_e98276_d_n8;
        locals.var_qb_2_dn9 = assign60520_e98276_d_n9;
        locals.var_qb_2_dn10 = assign60520_e98276_d_n10;
        locals.var_qb_2_dn11 = assign60520_e98276_d_n11;
        locals.var_qb_2_rv = 0.0;

        let assign60530_e98279: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard878 = assign60530_e98279;
        locals.var_guard878_rv = 0.0;

        let (assign60540_e98288, assign60540_e98288_d_n3, assign60540_e98288_d_n4, assign60540_e98288_d_n5, assign60540_e98288_d_n6, assign60540_e98288_d_n7, assign60540_e98288_d_n8, assign60540_e98288_d_n9, assign60540_e98288_d_n10, assign60540_e98288_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60540_e98286: f64 = (locals.var_devsign * locals.var_qsi);
        (assign60540_e98286, (locals.var_devsign * locals.var_qsi_dn3), (locals.var_devsign * locals.var_qsi_dn4), (locals.var_devsign * locals.var_qsi_dn5), (locals.var_devsign * locals.var_qsi_dn6), (locals.var_devsign * locals.var_qsi_dn7), (locals.var_devsign * locals.var_qsi_dn8), (locals.var_devsign * locals.var_qsi_dn9), (locals.var_devsign * locals.var_qsi_dn10), (locals.var_devsign * locals.var_qsi_dn11),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11,)
    }
};
        locals.var_qsi_1 = assign60540_e98288;
        locals.var_qsi_1_dn3 = assign60540_e98288_d_n3;
        locals.var_qsi_1_dn4 = assign60540_e98288_d_n4;
        locals.var_qsi_1_dn5 = assign60540_e98288_d_n5;
        locals.var_qsi_1_dn6 = assign60540_e98288_d_n6;
        locals.var_qsi_1_dn7 = assign60540_e98288_d_n7;
        locals.var_qsi_1_dn8 = assign60540_e98288_d_n8;
        locals.var_qsi_1_dn9 = assign60540_e98288_d_n9;
        locals.var_qsi_1_dn10 = assign60540_e98288_d_n10;
        locals.var_qsi_1_dn11 = assign60540_e98288_d_n11;
        locals.var_qsi_1_rv = 0.0;

        let (assign60570_e98315, assign60570_e98315_d_n3, assign60570_e98315_d_n4, assign60570_e98315_d_n5, assign60570_e98315_d_n6, assign60570_e98315_d_n7, assign60570_e98315_d_n8, assign60570_e98315_d_n9, assign60570_e98315_d_n10, assign60570_e98315_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60570_e98313: f64 = (locals.var_devsign * locals.var_qdi);
        (assign60570_e98313, (locals.var_devsign * locals.var_qdi_dn3), (locals.var_devsign * locals.var_qdi_dn4), (locals.var_devsign * locals.var_qdi_dn5), (locals.var_devsign * locals.var_qdi_dn6), (locals.var_devsign * locals.var_qdi_dn7), (locals.var_devsign * locals.var_qdi_dn8), (locals.var_devsign * locals.var_qdi_dn9), (locals.var_devsign * locals.var_qdi_dn10), (locals.var_devsign * locals.var_qdi_dn11),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11,)
    }
};
        locals.var_qdi_1 = assign60570_e98315;
        locals.var_qdi_1_dn3 = assign60570_e98315_d_n3;
        locals.var_qdi_1_dn4 = assign60570_e98315_d_n4;
        locals.var_qdi_1_dn5 = assign60570_e98315_d_n5;
        locals.var_qdi_1_dn6 = assign60570_e98315_d_n6;
        locals.var_qdi_1_dn7 = assign60570_e98315_d_n7;
        locals.var_qdi_1_dn8 = assign60570_e98315_d_n8;
        locals.var_qdi_1_dn9 = assign60570_e98315_d_n9;
        locals.var_qdi_1_dn10 = assign60570_e98315_d_n10;
        locals.var_qdi_1_dn11 = assign60570_e98315_d_n11;
        locals.var_qdi_1_rv = 0.0;

        let (assign60600_e98346, assign60600_e98346_d_n3, assign60600_e98346_d_n4, assign60600_e98346_d_n5, assign60600_e98346_d_n6, assign60600_e98346_d_n7, assign60600_e98346_d_n8, assign60600_e98346_d_n9, assign60600_e98346_d_n10, assign60600_e98346_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60600_e98341: f64 = (locals.var_qsi + locals.var_qovs);
        let assign60600_e98343: f64 = (assign60600_e98341 - locals.var_qbsj);
        let assign60600_e98344: f64 = (locals.var_devsign * assign60600_e98343);
        (assign60600_e98344, (locals.var_devsign * ((locals.var_qsi_dn3 + locals.var_qovs_dn3) - locals.var_qbsj_dn3)), (locals.var_devsign * ((locals.var_qsi_dn4 + locals.var_qovs_dn4) - locals.var_qbsj_dn4)), (locals.var_devsign * ((locals.var_qsi_dn5 + locals.var_qovs_dn5) - locals.var_qbsj_dn5)), (locals.var_devsign * ((locals.var_qsi_dn6 + locals.var_qovs_dn6) - locals.var_qbsj_dn6)), (locals.var_devsign * ((locals.var_qsi_dn7 + locals.var_qovs_dn7) - locals.var_qbsj_dn7)), (locals.var_devsign * ((locals.var_qsi_dn8 + locals.var_qovs_dn8) - locals.var_qbsj_dn8)), (locals.var_devsign * ((locals.var_qsi_dn9 + locals.var_qovs_dn9) - locals.var_qbsj_dn9)), (locals.var_devsign * ((locals.var_qsi_dn10 + locals.var_qovs_dn10) - locals.var_qbsj_dn10)), (locals.var_devsign * ((locals.var_qsi_dn11 + locals.var_qovs_dn11) - locals.var_qbsj_dn11)),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign60600_e98346;
        locals.var_qs_2_dn3 = assign60600_e98346_d_n3;
        locals.var_qs_2_dn4 = assign60600_e98346_d_n4;
        locals.var_qs_2_dn5 = assign60600_e98346_d_n5;
        locals.var_qs_2_dn6 = assign60600_e98346_d_n6;
        locals.var_qs_2_dn7 = assign60600_e98346_d_n7;
        locals.var_qs_2_dn8 = assign60600_e98346_d_n8;
        locals.var_qs_2_dn9 = assign60600_e98346_d_n9;
        locals.var_qs_2_dn10 = assign60600_e98346_d_n10;
        locals.var_qs_2_dn11 = assign60600_e98346_d_n11;
        locals.var_qs_2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_212(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (assign60610_e98359, assign60610_e98359_d_n3, assign60610_e98359_d_n4, assign60610_e98359_d_n5, assign60610_e98359_d_n6, assign60610_e98359_d_n7, assign60610_e98359_d_n8, assign60610_e98359_d_n9, assign60610_e98359_d_n10, assign60610_e98359_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 != 0.0)) {
        let assign60610_e98354: f64 = (locals.var_qdi + locals.var_qovd);
        let assign60610_e98356: f64 = (assign60610_e98354 - locals.var_qbdj);
        let assign60610_e98357: f64 = (locals.var_devsign * assign60610_e98356);
        (assign60610_e98357, (locals.var_devsign * ((locals.var_qdi_dn3 + locals.var_qovd_dn3) - locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qdi_dn4 + locals.var_qovd_dn4) - locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qdi_dn5 + locals.var_qovd_dn5) - locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qdi_dn6 + locals.var_qovd_dn6) - locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qdi_dn7 + locals.var_qovd_dn7) - locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qdi_dn8 + locals.var_qovd_dn8) - locals.var_qbdj_dn8)), (locals.var_devsign * ((locals.var_qdi_dn9 + locals.var_qovd_dn9) - locals.var_qbdj_dn9)), (locals.var_devsign * ((locals.var_qdi_dn10 + locals.var_qovd_dn10) - locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qdi_dn11 + locals.var_qovd_dn11) - locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign60610_e98359;
        locals.var_qd_1_dn3 = assign60610_e98359_d_n3;
        locals.var_qd_1_dn4 = assign60610_e98359_d_n4;
        locals.var_qd_1_dn5 = assign60610_e98359_d_n5;
        locals.var_qd_1_dn6 = assign60610_e98359_d_n6;
        locals.var_qd_1_dn7 = assign60610_e98359_d_n7;
        locals.var_qd_1_dn8 = assign60610_e98359_d_n8;
        locals.var_qd_1_dn9 = assign60610_e98359_d_n9;
        locals.var_qd_1_dn10 = assign60610_e98359_d_n10;
        locals.var_qd_1_dn11 = assign60610_e98359_d_n11;
        locals.var_qd_1_rv = 0.0;

        let (assign60620_e98369, assign60620_e98369_d_n3, assign60620_e98369_d_n4, assign60620_e98369_d_n5, assign60620_e98369_d_n6, assign60620_e98369_d_n7, assign60620_e98369_d_n8, assign60620_e98369_d_n9, assign60620_e98369_d_n10, assign60620_e98369_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60620_e98367: f64 = (locals.var_devsign * locals.var_qdi);
        (assign60620_e98367, (locals.var_devsign * locals.var_qdi_dn3), (locals.var_devsign * locals.var_qdi_dn4), (locals.var_devsign * locals.var_qdi_dn5), (locals.var_devsign * locals.var_qdi_dn6), (locals.var_devsign * locals.var_qdi_dn7), (locals.var_devsign * locals.var_qdi_dn8), (locals.var_devsign * locals.var_qdi_dn9), (locals.var_devsign * locals.var_qdi_dn10), (locals.var_devsign * locals.var_qdi_dn11),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11,)
    }
};
        locals.var_qsi_1 = assign60620_e98369;
        locals.var_qsi_1_dn3 = assign60620_e98369_d_n3;
        locals.var_qsi_1_dn4 = assign60620_e98369_d_n4;
        locals.var_qsi_1_dn5 = assign60620_e98369_d_n5;
        locals.var_qsi_1_dn6 = assign60620_e98369_d_n6;
        locals.var_qsi_1_dn7 = assign60620_e98369_d_n7;
        locals.var_qsi_1_dn8 = assign60620_e98369_d_n8;
        locals.var_qsi_1_dn9 = assign60620_e98369_d_n9;
        locals.var_qsi_1_dn10 = assign60620_e98369_d_n10;
        locals.var_qsi_1_dn11 = assign60620_e98369_d_n11;
        locals.var_qsi_1_rv = 0.0;

        let (assign60650_e98399, assign60650_e98399_d_n3, assign60650_e98399_d_n4, assign60650_e98399_d_n5, assign60650_e98399_d_n6, assign60650_e98399_d_n7, assign60650_e98399_d_n8, assign60650_e98399_d_n9, assign60650_e98399_d_n10, assign60650_e98399_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60650_e98397: f64 = (locals.var_devsign * locals.var_qsi);
        (assign60650_e98397, (locals.var_devsign * locals.var_qsi_dn3), (locals.var_devsign * locals.var_qsi_dn4), (locals.var_devsign * locals.var_qsi_dn5), (locals.var_devsign * locals.var_qsi_dn6), (locals.var_devsign * locals.var_qsi_dn7), (locals.var_devsign * locals.var_qsi_dn8), (locals.var_devsign * locals.var_qsi_dn9), (locals.var_devsign * locals.var_qsi_dn10), (locals.var_devsign * locals.var_qsi_dn11),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11,)
    }
};
        locals.var_qdi_1 = assign60650_e98399;
        locals.var_qdi_1_dn3 = assign60650_e98399_d_n3;
        locals.var_qdi_1_dn4 = assign60650_e98399_d_n4;
        locals.var_qdi_1_dn5 = assign60650_e98399_d_n5;
        locals.var_qdi_1_dn6 = assign60650_e98399_d_n6;
        locals.var_qdi_1_dn7 = assign60650_e98399_d_n7;
        locals.var_qdi_1_dn8 = assign60650_e98399_d_n8;
        locals.var_qdi_1_dn9 = assign60650_e98399_d_n9;
        locals.var_qdi_1_dn10 = assign60650_e98399_d_n10;
        locals.var_qdi_1_dn11 = assign60650_e98399_d_n11;
        locals.var_qdi_1_rv = 0.0;

        let (assign60680_e98433, assign60680_e98433_d_n3, assign60680_e98433_d_n4, assign60680_e98433_d_n5, assign60680_e98433_d_n6, assign60680_e98433_d_n7, assign60680_e98433_d_n8, assign60680_e98433_d_n9, assign60680_e98433_d_n10, assign60680_e98433_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60680_e98428: f64 = (locals.var_qdi + locals.var_qovs);
        let assign60680_e98430: f64 = (assign60680_e98428 - locals.var_qbsj);
        let assign60680_e98431: f64 = (locals.var_devsign * assign60680_e98430);
        (assign60680_e98431, (locals.var_devsign * ((locals.var_qdi_dn3 + locals.var_qovs_dn3) - locals.var_qbsj_dn3)), (locals.var_devsign * ((locals.var_qdi_dn4 + locals.var_qovs_dn4) - locals.var_qbsj_dn4)), (locals.var_devsign * ((locals.var_qdi_dn5 + locals.var_qovs_dn5) - locals.var_qbsj_dn5)), (locals.var_devsign * ((locals.var_qdi_dn6 + locals.var_qovs_dn6) - locals.var_qbsj_dn6)), (locals.var_devsign * ((locals.var_qdi_dn7 + locals.var_qovs_dn7) - locals.var_qbsj_dn7)), (locals.var_devsign * ((locals.var_qdi_dn8 + locals.var_qovs_dn8) - locals.var_qbsj_dn8)), (locals.var_devsign * ((locals.var_qdi_dn9 + locals.var_qovs_dn9) - locals.var_qbsj_dn9)), (locals.var_devsign * ((locals.var_qdi_dn10 + locals.var_qovs_dn10) - locals.var_qbsj_dn10)), (locals.var_devsign * ((locals.var_qdi_dn11 + locals.var_qovs_dn11) - locals.var_qbsj_dn11)),)
    } else {
        (locals.var_qs_2, locals.var_qs_2_dn3, locals.var_qs_2_dn4, locals.var_qs_2_dn5, locals.var_qs_2_dn6, locals.var_qs_2_dn7, locals.var_qs_2_dn8, locals.var_qs_2_dn9, locals.var_qs_2_dn10, locals.var_qs_2_dn11,)
    }
};
        locals.var_qs_2 = assign60680_e98433;
        locals.var_qs_2_dn3 = assign60680_e98433_d_n3;
        locals.var_qs_2_dn4 = assign60680_e98433_d_n4;
        locals.var_qs_2_dn5 = assign60680_e98433_d_n5;
        locals.var_qs_2_dn6 = assign60680_e98433_d_n6;
        locals.var_qs_2_dn7 = assign60680_e98433_d_n7;
        locals.var_qs_2_dn8 = assign60680_e98433_d_n8;
        locals.var_qs_2_dn9 = assign60680_e98433_d_n9;
        locals.var_qs_2_dn10 = assign60680_e98433_d_n10;
        locals.var_qs_2_dn11 = assign60680_e98433_d_n11;
        locals.var_qs_2_rv = 0.0;

        let (assign60690_e98447, assign60690_e98447_d_n3, assign60690_e98447_d_n4, assign60690_e98447_d_n5, assign60690_e98447_d_n6, assign60690_e98447_d_n7, assign60690_e98447_d_n8, assign60690_e98447_d_n9, assign60690_e98447_d_n10, assign60690_e98447_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard878 == 0.0)) {
        let assign60690_e98442: f64 = (locals.var_qsi + locals.var_qovd);
        let assign60690_e98444: f64 = (assign60690_e98442 - locals.var_qbdj);
        let assign60690_e98445: f64 = (locals.var_devsign * assign60690_e98444);
        (assign60690_e98445, (locals.var_devsign * ((locals.var_qsi_dn3 + locals.var_qovd_dn3) - locals.var_qbdj_dn3)), (locals.var_devsign * ((locals.var_qsi_dn4 + locals.var_qovd_dn4) - locals.var_qbdj_dn4)), (locals.var_devsign * ((locals.var_qsi_dn5 + locals.var_qovd_dn5) - locals.var_qbdj_dn5)), (locals.var_devsign * ((locals.var_qsi_dn6 + locals.var_qovd_dn6) - locals.var_qbdj_dn6)), (locals.var_devsign * ((locals.var_qsi_dn7 + locals.var_qovd_dn7) - locals.var_qbdj_dn7)), (locals.var_devsign * ((locals.var_qsi_dn8 + locals.var_qovd_dn8) - locals.var_qbdj_dn8)), (locals.var_devsign * ((locals.var_qsi_dn9 + locals.var_qovd_dn9) - locals.var_qbdj_dn9)), (locals.var_devsign * ((locals.var_qsi_dn10 + locals.var_qovd_dn10) - locals.var_qbdj_dn10)), (locals.var_devsign * ((locals.var_qsi_dn11 + locals.var_qovd_dn11) - locals.var_qbdj_dn11)),)
    } else {
        (locals.var_qd_1, locals.var_qd_1_dn3, locals.var_qd_1_dn4, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, locals.var_qd_1_dn10, locals.var_qd_1_dn11,)
    }
};
        locals.var_qd_1 = assign60690_e98447;
        locals.var_qd_1_dn3 = assign60690_e98447_d_n3;
        locals.var_qd_1_dn4 = assign60690_e98447_d_n4;
        locals.var_qd_1_dn5 = assign60690_e98447_d_n5;
        locals.var_qd_1_dn6 = assign60690_e98447_d_n6;
        locals.var_qd_1_dn7 = assign60690_e98447_d_n7;
        locals.var_qd_1_dn8 = assign60690_e98447_d_n8;
        locals.var_qd_1_dn9 = assign60690_e98447_d_n9;
        locals.var_qd_1_dn10 = assign60690_e98447_d_n10;
        locals.var_qd_1_dn11 = assign60690_e98447_d_n11;
        locals.var_qd_1_rv = 0.0;

        let (assign60700_e98456, assign60700_e98456_d_n3, assign60700_e98456_d_n4, assign60700_e98456_d_n5, assign60700_e98456_d_n6, assign60700_e98456_d_n7, assign60700_e98456_d_n8, assign60700_e98456_d_n9, assign60700_e98456_d_n10, assign60700_e98456_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign60700_e98453: f64 = (locals.var_qgi + locals.var_qovg);
        let assign60700_e98454: f64 = (locals.var_devsign * assign60700_e98453);
        (assign60700_e98454, (locals.var_devsign * (locals.var_qgi_dn3 + locals.var_qovg_dn3)), (locals.var_devsign * (locals.var_qgi_dn4 + locals.var_qovg_dn4)), (locals.var_devsign * (locals.var_qgi_dn5 + locals.var_qovg_dn5)), (locals.var_devsign * (locals.var_qgi_dn6 + locals.var_qovg_dn6)), (locals.var_devsign * (locals.var_qgi_dn7 + locals.var_qovg_dn7)), (locals.var_devsign * (locals.var_qgi_dn8 + locals.var_qovg_dn8)), (locals.var_devsign * (locals.var_qgi_dn9 + locals.var_qovg_dn9)), (locals.var_devsign * (locals.var_qgi_dn10 + locals.var_qovg_dn10)), (locals.var_devsign * (locals.var_qgi_dn11 + locals.var_qovg_dn11)),)
    } else {
        (locals.var_qg, locals.var_qg_dn3, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11,)
    }
};
        locals.var_qg = assign60700_e98456;
        locals.var_qg_dn3 = assign60700_e98456_d_n3;
        locals.var_qg_dn4 = assign60700_e98456_d_n4;
        locals.var_qg_dn5 = assign60700_e98456_d_n5;
        locals.var_qg_dn6 = assign60700_e98456_d_n6;
        locals.var_qg_dn7 = assign60700_e98456_d_n7;
        locals.var_qg_dn8 = assign60700_e98456_d_n8;
        locals.var_qg_dn9 = assign60700_e98456_d_n9;
        locals.var_qg_dn10 = assign60700_e98456_d_n10;
        locals.var_qg_dn11 = assign60700_e98456_d_n11;
        locals.var_qg_rv = 0.0;

        locals.var_weff_1 = locals.var_weff;
        locals.var_weff_1_rv = 0.0;

        locals.var_leff_1 = locals.var_leff;
        locals.var_leff_1_rv = 0.0;

        let assign61510_e98885: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard893 = assign61510_e98885;
        locals.var_guard893_rv = 0.0;

        let (assign61520_e98895, assign61520_e98895_d_n0, assign61520_e98895_d_n2, assign61520_e98895_d_n3, assign61520_e98895_d_n4, assign61520_e98895_d_n5, assign61520_e98895_d_n6, assign61520_e98895_d_n7, assign61520_e98895_d_n8, assign61520_e98895_d_n9, assign61520_e98895_d_n10, assign61520_e98895_d_n11,) = {
    if (locals.var_guard893 != 0.0) {
        let assign61520_e98889: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign61520_e98891: f64 = (assign61520_e98889 * locals.var_ids);
        let assign61520_e98893: f64 = (assign61520_e98891 * (nv6 - nv7));
        (assign61520_e98893, 0.0, 0.0, ((assign61520_e98889 * locals.var_ids_dn3) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn4) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn5) * (nv6 - nv7)), (((assign61520_e98889 * locals.var_ids_dn6) * (nv6 - nv7)) + assign61520_e98891), (((assign61520_e98889 * locals.var_ids_dn7) * (nv6 - nv7)) + (-assign61520_e98891)), ((assign61520_e98889 * locals.var_ids_dn8) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn9) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn10) * (nv6 - nv7)), ((assign61520_e98889 * locals.var_ids_dn11) * (nv6 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61520_e98895;
        locals.var_pdiss_dn0 = assign61520_e98895_d_n0;
        locals.var_pdiss_dn2 = assign61520_e98895_d_n2;
        locals.var_pdiss_dn3 = assign61520_e98895_d_n3;
        locals.var_pdiss_dn4 = assign61520_e98895_d_n4;
        locals.var_pdiss_dn5 = assign61520_e98895_d_n5;
        locals.var_pdiss_dn6 = assign61520_e98895_d_n6;
        locals.var_pdiss_dn7 = assign61520_e98895_d_n7;
        locals.var_pdiss_dn8 = assign61520_e98895_d_n8;
        locals.var_pdiss_dn9 = assign61520_e98895_d_n9;
        locals.var_pdiss_dn10 = assign61520_e98895_d_n10;
        locals.var_pdiss_dn11 = assign61520_e98895_d_n11;
        locals.var_pdiss_rv = 0.0;

        let assign61530_e98902: f64 = if ((p.p33 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard894 = assign61530_e98902;
        locals.var_guard894_rv = 0.0;

        let (assign61540_e98914, assign61540_e98914_d_n0, assign61540_e98914_d_n2, assign61540_e98914_d_n3, assign61540_e98914_d_n4, assign61540_e98914_d_n5, assign61540_e98914_d_n6, assign61540_e98914_d_n7, assign61540_e98914_d_n8, assign61540_e98914_d_n9, assign61540_e98914_d_n10, assign61540_e98914_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard894 != 0.0)) {
        let assign61540_e98909: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign61540_e98911: f64 = (assign61540_e98909 / locals.var_rdrain);
        let assign61540_e98912: f64 = (locals.var_pdiss + assign61540_e98911);
        (assign61540_e98912, (locals.var_pdiss_dn0 + (((nv0 - nv6) + (nv0 - nv6)) / locals.var_rdrain)), locals.var_pdiss_dn2, (locals.var_pdiss_dn3 + (-((assign61540_e98909 * locals.var_rdrain_dn3) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn4 + (-((assign61540_e98909 * locals.var_rdrain_dn4) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn5 + (-((assign61540_e98909 * locals.var_rdrain_dn5) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn6 + (((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_rdrain) - (assign61540_e98909 * locals.var_rdrain_dn6)) / (locals.var_rdrain * locals.var_rdrain))), (locals.var_pdiss_dn7 + (-((assign61540_e98909 * locals.var_rdrain_dn7) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn8 + (-((assign61540_e98909 * locals.var_rdrain_dn8) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn9 + (-((assign61540_e98909 * locals.var_rdrain_dn9) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn10 + (-((assign61540_e98909 * locals.var_rdrain_dn10) / (locals.var_rdrain * locals.var_rdrain)))), (locals.var_pdiss_dn11 + (-((assign61540_e98909 * locals.var_rdrain_dn11) / (locals.var_rdrain * locals.var_rdrain)))),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61540_e98914;
        locals.var_pdiss_dn0 = assign61540_e98914_d_n0;
        locals.var_pdiss_dn2 = assign61540_e98914_d_n2;
        locals.var_pdiss_dn3 = assign61540_e98914_d_n3;
        locals.var_pdiss_dn4 = assign61540_e98914_d_n4;
        locals.var_pdiss_dn5 = assign61540_e98914_d_n5;
        locals.var_pdiss_dn6 = assign61540_e98914_d_n6;
        locals.var_pdiss_dn7 = assign61540_e98914_d_n7;
        locals.var_pdiss_dn8 = assign61540_e98914_d_n8;
        locals.var_pdiss_dn9 = assign61540_e98914_d_n9;
        locals.var_pdiss_dn10 = assign61540_e98914_d_n10;
        locals.var_pdiss_dn11 = assign61540_e98914_d_n11;
        locals.var_pdiss_rv = 0.0;

        let assign61550_e98921: f64 = if ((p.p33 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard895 = assign61550_e98921;
        locals.var_guard895_rv = 0.0;

        let (assign61560_e98933, assign61560_e98933_d_n0, assign61560_e98933_d_n2, assign61560_e98933_d_n3, assign61560_e98933_d_n4, assign61560_e98933_d_n5, assign61560_e98933_d_n6, assign61560_e98933_d_n7, assign61560_e98933_d_n8, assign61560_e98933_d_n9, assign61560_e98933_d_n10, assign61560_e98933_d_n11,) = {
    if ((locals.var_guard893 != 0.0) && (locals.var_guard895 != 0.0)) {
        let assign61560_e98928: f64 = ((nv2 - nv7) * (nv2 - nv7));
        let assign61560_e98930: f64 = (assign61560_e98928 / locals.var_rsource);
        let assign61560_e98931: f64 = (locals.var_pdiss + assign61560_e98930);
        (assign61560_e98931, locals.var_pdiss_dn0, (locals.var_pdiss_dn2 + (((nv2 - nv7) + (nv2 - nv7)) / locals.var_rsource)), (locals.var_pdiss_dn3 + (-((assign61560_e98928 * locals.var_rsource_dn3) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn4 + (-((assign61560_e98928 * locals.var_rsource_dn4) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn5 + (-((assign61560_e98928 * locals.var_rsource_dn5) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn6 + (-((assign61560_e98928 * locals.var_rsource_dn6) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn7 + (((((-(nv2 - nv7)) + (-(nv2 - nv7))) * locals.var_rsource) - (assign61560_e98928 * locals.var_rsource_dn7)) / (locals.var_rsource * locals.var_rsource))), (locals.var_pdiss_dn8 + (-((assign61560_e98928 * locals.var_rsource_dn8) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn9 + (-((assign61560_e98928 * locals.var_rsource_dn9) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn10 + (-((assign61560_e98928 * locals.var_rsource_dn10) / (locals.var_rsource * locals.var_rsource)))), (locals.var_pdiss_dn11 + (-((assign61560_e98928 * locals.var_rsource_dn11) / (locals.var_rsource * locals.var_rsource)))),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11,)
    }
};
        locals.var_pdiss = assign61560_e98933;
        locals.var_pdiss_dn0 = assign61560_e98933_d_n0;
        locals.var_pdiss_dn2 = assign61560_e98933_d_n2;
        locals.var_pdiss_dn3 = assign61560_e98933_d_n3;
        locals.var_pdiss_dn4 = assign61560_e98933_d_n4;
        locals.var_pdiss_dn5 = assign61560_e98933_d_n5;
        locals.var_pdiss_dn6 = assign61560_e98933_d_n6;
        locals.var_pdiss_dn7 = assign61560_e98933_d_n7;
        locals.var_pdiss_dn8 = assign61560_e98933_d_n8;
        locals.var_pdiss_dn9 = assign61560_e98933_d_n9;
        locals.var_pdiss_dn10 = assign61560_e98933_d_n10;
        locals.var_pdiss_dn11 = assign61560_e98933_d_n11;
        locals.var_pdiss_rv = 0.0;

        let assign61570_e98938: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard896 = assign61570_e98938;
        locals.var_guard896_rv = 0.0;

        let assign61580_e98940: f64 = 1.0;
        locals.var_guard897 = assign61580_e98940;
        locals.var_guard897_rv = 0.0;

        let assign61610_e98950: f64 = (p.p1359 * p.p1358);
        locals.var_rbodyext = assign61610_e98950;
        locals.var_rbodyext_rv = 0.0;

        let assign61620_e98958: f64 = if ((p.p43 == 0.0) || (!true)) { 1.0 } else { 0.0 };
        locals.var_guard900 = assign61620_e98958;
        locals.var_guard900_rv = 0.0;

        let assign61630_e98963: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard901 = assign61630_e98963;
        locals.var_guard901_rv = 0.0;

        let assign61640_e98966: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard902 = assign61640_e98966;
        locals.var_guard902_rv = 0.0;

        let (assign61650_e98994, assign61650_e98994_d_n3, assign61650_e98994_d_n4, assign61650_e98994_d_n5, assign61650_e98994_d_n6, assign61650_e98994_d_n7, assign61650_e98994_d_n8, assign61650_e98994_d_n9, assign61650_e98994_d_n10, assign61650_e98994_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) {
        let assign61650_e98976: f64 = (p.p1357 * p.p1356);
        let assign61650_e98978: f64 = (assign61650_e98976 * p.p1360);
        let assign61650_e98981: f64 = (2.0 * p.p1356);
        let assign61650_e98984: f64 = (p.p1360 * locals.var_leff_1);
        let assign61650_e98985: f64 = (assign61650_e98981 + assign61650_e98984);
        let assign61650_e98986: f64 = (assign61650_e98978 / assign61650_e98985);
        let assign61650_e98988: f64 = (assign61650_e98986 * locals.var_weff_1);
        let assign61650_e98990: f64 = (assign61650_e98988 / p.p1373);
        let assign61650_e98992: f64 = (assign61650_e98990 / p.p2);
        (assign61650_e98992, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbodyint, locals.var_rbodyint_dn3, locals.var_rbodyint_dn4, locals.var_rbodyint_dn5, locals.var_rbodyint_dn6, locals.var_rbodyint_dn7, locals.var_rbodyint_dn8, locals.var_rbodyint_dn9, locals.var_rbodyint_dn10, locals.var_rbodyint_dn11,)
    }
};
        locals.var_rbodyint = assign61650_e98994;
        locals.var_rbodyint_dn3 = assign61650_e98994_d_n3;
        locals.var_rbodyint_dn4 = assign61650_e98994_d_n4;
        locals.var_rbodyint_dn5 = assign61650_e98994_d_n5;
        locals.var_rbodyint_dn6 = assign61650_e98994_d_n6;
        locals.var_rbodyint_dn7 = assign61650_e98994_d_n7;
        locals.var_rbodyint_dn8 = assign61650_e98994_d_n8;
        locals.var_rbodyint_dn9 = assign61650_e98994_d_n9;
        locals.var_rbodyint_dn10 = assign61650_e98994_d_n10;
        locals.var_rbodyint_dn11 = assign61650_e98994_d_n11;
        locals.var_rbodyint_rv = 0.0;

        let assign61660_e98997: f64 = if locals.var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard903 = assign61660_e98997;
        locals.var_guard903_rv = 0.0;

        let assign61670_e99000: f64 = if locals.var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard904 = assign61670_e99000;
        locals.var_guard904_rv = 0.0;

        let (assign61680_e99016, assign61680_e99016_d_n3, assign61680_e99016_d_n4, assign61680_e99016_d_n5, assign61680_e99016_d_n6, assign61680_e99016_d_n7, assign61680_e99016_d_n8, assign61680_e99016_d_n9, assign61680_e99016_d_n10, assign61680_e99016_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) && (locals.var_guard904 != 0.0)) {
        let assign61680_e99014: f64 = (1.0 / 0.001);
        (assign61680_e99014, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61680_e99016;
        locals.var_t0_dn3 = assign61680_e99016_d_n3;
        locals.var_t0_dn4 = assign61680_e99016_d_n4;
        locals.var_t0_dn5 = assign61680_e99016_d_n5;
        locals.var_t0_dn6 = assign61680_e99016_d_n6;
        locals.var_t0_dn7 = assign61680_e99016_d_n7;
        locals.var_t0_dn8 = assign61680_e99016_d_n8;
        locals.var_t0_dn9 = assign61680_e99016_d_n9;
        locals.var_t0_dn10 = assign61680_e99016_d_n10;
        locals.var_t0_dn11 = assign61680_e99016_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign61690_e99033, assign61690_e99033_d_n3, assign61690_e99033_d_n4, assign61690_e99033_d_n5, assign61690_e99033_d_n6, assign61690_e99033_d_n7, assign61690_e99033_d_n8, assign61690_e99033_d_n9, assign61690_e99033_d_n10, assign61690_e99033_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 != 0.0)) && (locals.var_guard903 != 0.0)) && (locals.var_guard904 == 0.0)) {
        let assign61690_e99031: f64 = (1.0 / locals.var_rbodyext);
        (assign61690_e99031, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61690_e99033;
        locals.var_t0_dn3 = assign61690_e99033_d_n3;
        locals.var_t0_dn4 = assign61690_e99033_d_n4;
        locals.var_t0_dn5 = assign61690_e99033_d_n5;
        locals.var_t0_dn6 = assign61690_e99033_d_n6;
        locals.var_t0_dn7 = assign61690_e99033_d_n7;
        locals.var_t0_dn8 = assign61690_e99033_d_n8;
        locals.var_t0_dn9 = assign61690_e99033_d_n9;
        locals.var_t0_dn10 = assign61690_e99033_d_n10;
        locals.var_t0_dn11 = assign61690_e99033_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign61720_e99077, assign61720_e99077_d_n4, assign61720_e99077_d_n5,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61720_e99074: f64 = (locals.var_tratio).powf(locals.var_ubte_i);
        let assign61720_e99075: f64 = (locals.var_ub_i * assign61720_e99074);
        (assign61720_e99075, (locals.var_ub_i * if 0.0 == 0.0 && ((locals.var_ubte_i) as f64).is_finite() && ((locals.var_ubte_i) as f64).fract() == 0.0 { if locals.var_ubte_i == 0.0 { 0.0 } else { (locals.var_ubte_i * ((locals.var_tratio).powf(locals.var_ubte_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign61720_e99074 * (locals.var_ubte_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), (locals.var_ub_i * if 0.0 == 0.0 && ((locals.var_ubte_i) as f64).is_finite() && ((locals.var_ubte_i) as f64).fract() == 0.0 { if locals.var_ubte_i == 0.0 { 0.0 } else { (locals.var_ubte_i * ((locals.var_tratio).powf(locals.var_ubte_i - 1.0) * locals.var_tratio_dn5)) } } else { (assign61720_e99074 * (locals.var_ubte_i * (locals.var_tratio_dn5 / locals.var_tratio))) }),)
    } else {
        (locals.var_ub_t, locals.var_ub_t_dn4, locals.var_ub_t_dn5,)
    }
};
        locals.var_ub_t = assign61720_e99077;
        locals.var_ub_t_dn4 = assign61720_e99077_d_n4;
        locals.var_ub_t_dn5 = assign61720_e99077_d_n5;
        locals.var_ub_t_rv = 0.0;

        let (assign61730_e99095, assign61730_e99095_d_n3, assign61730_e99095_d_n4, assign61730_e99095_d_n5, assign61730_e99095_d_n6, assign61730_e99095_d_n7, assign61730_e99095_d_n8, assign61730_e99095_d_n9, assign61730_e99095_d_n10, assign61730_e99095_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61730_e99088: f64 = (locals.var_qbi + locals.var_qbsj);
        let assign61730_e99090: f64 = (assign61730_e99088 + locals.var_qbdj);
        let assign61730_e99091: f64 = (-assign61730_e99090);
        let assign61730_e99093: f64 = (assign61730_e99091 + locals.var_qsub);
        (assign61730_e99093, ((-((locals.var_qbi_dn3 + locals.var_qbsj_dn3) + locals.var_qbdj_dn3)) + locals.var_qsub_dn3), ((-((locals.var_qbi_dn4 + locals.var_qbsj_dn4) + locals.var_qbdj_dn4)) + locals.var_qsub_dn4), ((-((locals.var_qbi_dn5 + locals.var_qbsj_dn5) + locals.var_qbdj_dn5)) + locals.var_qsub_dn5), ((-((locals.var_qbi_dn6 + locals.var_qbsj_dn6) + locals.var_qbdj_dn6)) + locals.var_qsub_dn6), ((-((locals.var_qbi_dn7 + locals.var_qbsj_dn7) + locals.var_qbdj_dn7)) + locals.var_qsub_dn7), ((-((locals.var_qbi_dn8 + locals.var_qbsj_dn8) + locals.var_qbdj_dn8)) + locals.var_qsub_dn8), ((-((locals.var_qbi_dn9 + locals.var_qbsj_dn9) + locals.var_qbdj_dn9)) + locals.var_qsub_dn9), ((-((locals.var_qbi_dn10 + locals.var_qbsj_dn10) + locals.var_qbdj_dn10)) + locals.var_qsub_dn10), ((-((locals.var_qbi_dn11 + locals.var_qbsj_dn11) + locals.var_qbdj_dn11)) + locals.var_qsub_dn11),)
    } else {
        (locals.var_qb1, locals.var_qb1_dn3, locals.var_qb1_dn4, locals.var_qb1_dn5, locals.var_qb1_dn6, locals.var_qb1_dn7, locals.var_qb1_dn8, locals.var_qb1_dn9, locals.var_qb1_dn10, locals.var_qb1_dn11,)
    }
};
        locals.var_qb1 = assign61730_e99095;
        locals.var_qb1_dn3 = assign61730_e99095_d_n3;
        locals.var_qb1_dn4 = assign61730_e99095_d_n4;
        locals.var_qb1_dn5 = assign61730_e99095_d_n5;
        locals.var_qb1_dn6 = assign61730_e99095_d_n6;
        locals.var_qb1_dn7 = assign61730_e99095_d_n7;
        locals.var_qb1_dn8 = assign61730_e99095_d_n8;
        locals.var_qb1_dn9 = assign61730_e99095_d_n9;
        locals.var_qb1_dn10 = assign61730_e99095_d_n10;
        locals.var_qb1_dn11 = assign61730_e99095_d_n11;
        locals.var_qb1_rv = 0.0;

        let (assign61740_e99116, assign61740_e99116_d_n3, assign61740_e99116_d_n4, assign61740_e99116_d_n5, assign61740_e99116_d_n6, assign61740_e99116_d_n7, assign61740_e99116_d_n8, assign61740_e99116_d_n9, assign61740_e99116_d_n10, assign61740_e99116_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61740_e99106: f64 = (1.602176462e-19 * locals.var_neff_i);
        let assign61740_e99108: f64 = (assign61740_e99106 * p.p74);
        let assign61740_e99110: f64 = (assign61740_e99108 * locals.var_weff_1);
        let assign61740_e99112: f64 = (assign61740_e99110 * locals.var_leff_1);
        let assign61740_e99114: f64 = (assign61740_e99112 - locals.var_qb1);
        (assign61740_e99114, (-locals.var_qb1_dn3), (-locals.var_qb1_dn4), (-locals.var_qb1_dn5), (-locals.var_qb1_dn6), (-locals.var_qb1_dn7), (-locals.var_qb1_dn8), (-locals.var_qb1_dn9), (-locals.var_qb1_dn10), (-locals.var_qb1_dn11),)
    } else {
        (locals.var_qbody, locals.var_qbody_dn3, locals.var_qbody_dn4, locals.var_qbody_dn5, locals.var_qbody_dn6, locals.var_qbody_dn7, locals.var_qbody_dn8, locals.var_qbody_dn9, locals.var_qbody_dn10, locals.var_qbody_dn11,)
    }
};
        locals.var_qbody = assign61740_e99116;
        locals.var_qbody_dn3 = assign61740_e99116_d_n3;
        locals.var_qbody_dn4 = assign61740_e99116_d_n4;
        locals.var_qbody_dn5 = assign61740_e99116_d_n5;
        locals.var_qbody_dn6 = assign61740_e99116_d_n6;
        locals.var_qbody_dn7 = assign61740_e99116_d_n7;
        locals.var_qbody_dn8 = assign61740_e99116_d_n8;
        locals.var_qbody_dn9 = assign61740_e99116_d_n9;
        locals.var_qbody_dn10 = assign61740_e99116_d_n10;
        locals.var_qbody_dn11 = assign61740_e99116_d_n11;
        locals.var_qbody_rv = 0.0;

        let (assign61750_e99129, assign61750_e99129_d_n3, assign61750_e99129_d_n4, assign61750_e99129_d_n5, assign61750_e99129_d_n6, assign61750_e99129_d_n7, assign61750_e99129_d_n8, assign61750_e99129_d_n9, assign61750_e99129_d_n10, assign61750_e99129_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61750_e99127: f64 = (locals.var_ub_t * locals.var_qbody);
        (assign61750_e99127, (locals.var_ub_t * locals.var_qbody_dn3), ((locals.var_ub_t_dn4 * locals.var_qbody) + (locals.var_ub_t * locals.var_qbody_dn4)), ((locals.var_ub_t_dn5 * locals.var_qbody) + (locals.var_ub_t * locals.var_qbody_dn5)), (locals.var_ub_t * locals.var_qbody_dn6), (locals.var_ub_t * locals.var_qbody_dn7), (locals.var_ub_t * locals.var_qbody_dn8), (locals.var_ub_t * locals.var_qbody_dn9), (locals.var_ub_t * locals.var_qbody_dn10), (locals.var_ub_t * locals.var_qbody_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61750_e99129;
        locals.var_t0_dn3 = assign61750_e99129_d_n3;
        locals.var_t0_dn4 = assign61750_e99129_d_n4;
        locals.var_t0_dn5 = assign61750_e99129_d_n5;
        locals.var_t0_dn6 = assign61750_e99129_d_n6;
        locals.var_t0_dn7 = assign61750_e99129_d_n7;
        locals.var_t0_dn8 = assign61750_e99129_d_n8;
        locals.var_t0_dn9 = assign61750_e99129_d_n9;
        locals.var_t0_dn10 = assign61750_e99129_d_n10;
        locals.var_t0_dn11 = assign61750_e99129_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign61760_e99142, assign61760_e99142_d_n3, assign61760_e99142_d_n4, assign61760_e99142_d_n5, assign61760_e99142_d_n6, assign61760_e99142_d_n7, assign61760_e99142_d_n8, assign61760_e99142_d_n9, assign61760_e99142_d_n10, assign61760_e99142_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61760_e99140: f64 = (locals.var_weff_1 * locals.var_weff_1);
        (assign61760_e99140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign61760_e99142;
        locals.var_t1_dn3 = assign61760_e99142_d_n3;
        locals.var_t1_dn4 = assign61760_e99142_d_n4;
        locals.var_t1_dn5 = assign61760_e99142_d_n5;
        locals.var_t1_dn6 = assign61760_e99142_d_n6;
        locals.var_t1_dn7 = assign61760_e99142_d_n7;
        locals.var_t1_dn8 = assign61760_e99142_d_n8;
        locals.var_t1_dn9 = assign61760_e99142_d_n9;
        locals.var_t1_dn10 = assign61760_e99142_d_n10;
        locals.var_t1_dn11 = assign61760_e99142_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign61770_e99157, assign61770_e99157_d_n3, assign61770_e99157_d_n4, assign61770_e99157_d_n5, assign61770_e99157_d_n6, assign61770_e99157_d_n7, assign61770_e99157_d_n8, assign61770_e99157_d_n9, assign61770_e99157_d_n10, assign61770_e99157_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61770_e99153: f64 = (p.p2 * locals.var_t0);
        let assign61770_e99155: f64 = (assign61770_e99153 / locals.var_t1);
        (assign61770_e99155, ((((p.p2 * locals.var_t0_dn3) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn4) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn5) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn6) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn7) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn8) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn9) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn10) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), ((((p.p2 * locals.var_t0_dn11) * locals.var_t1) - (assign61770_e99153 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gbodyint, locals.var_gbodyint_dn3, locals.var_gbodyint_dn4, locals.var_gbodyint_dn5, locals.var_gbodyint_dn6, locals.var_gbodyint_dn7, locals.var_gbodyint_dn8, locals.var_gbodyint_dn9, locals.var_gbodyint_dn10, locals.var_gbodyint_dn11,)
    }
};
        locals.var_gbodyint = assign61770_e99157;
        locals.var_gbodyint_dn3 = assign61770_e99157_d_n3;
        locals.var_gbodyint_dn4 = assign61770_e99157_d_n4;
        locals.var_gbodyint_dn5 = assign61770_e99157_d_n5;
        locals.var_gbodyint_dn6 = assign61770_e99157_d_n6;
        locals.var_gbodyint_dn7 = assign61770_e99157_d_n7;
        locals.var_gbodyint_dn8 = assign61770_e99157_d_n8;
        locals.var_gbodyint_dn9 = assign61770_e99157_d_n9;
        locals.var_gbodyint_dn10 = assign61770_e99157_d_n10;
        locals.var_gbodyint_dn11 = assign61770_e99157_d_n11;
        locals.var_gbodyint_rv = 0.0;

        let (assign61780_e99170, assign61780_e99170_d_n3, assign61780_e99170_d_n4, assign61780_e99170_d_n5, assign61780_e99170_d_n6, assign61780_e99170_d_n7, assign61780_e99170_d_n8, assign61780_e99170_d_n9, assign61780_e99170_d_n10, assign61780_e99170_d_n11,) = {
    if (((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) {
        let assign61780_e99168: f64 = (1.0 / locals.var_gbodyint);
        (assign61780_e99168, (-(locals.var_gbodyint_dn3 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn4 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn5 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn6 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn7 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn8 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn9 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn10 / (locals.var_gbodyint * locals.var_gbodyint))), (-(locals.var_gbodyint_dn11 / (locals.var_gbodyint * locals.var_gbodyint))),)
    } else {
        (locals.var_rbodyint, locals.var_rbodyint_dn3, locals.var_rbodyint_dn4, locals.var_rbodyint_dn5, locals.var_rbodyint_dn6, locals.var_rbodyint_dn7, locals.var_rbodyint_dn8, locals.var_rbodyint_dn9, locals.var_rbodyint_dn10, locals.var_rbodyint_dn11,)
    }
};
        locals.var_rbodyint = assign61780_e99170;
        locals.var_rbodyint_dn3 = assign61780_e99170_d_n3;
        locals.var_rbodyint_dn4 = assign61780_e99170_d_n4;
        locals.var_rbodyint_dn5 = assign61780_e99170_d_n5;
        locals.var_rbodyint_dn6 = assign61780_e99170_d_n6;
        locals.var_rbodyint_dn7 = assign61780_e99170_d_n7;
        locals.var_rbodyint_dn8 = assign61780_e99170_d_n8;
        locals.var_rbodyint_dn9 = assign61780_e99170_d_n9;
        locals.var_rbodyint_dn10 = assign61780_e99170_d_n10;
        locals.var_rbodyint_dn11 = assign61780_e99170_d_n11;
        locals.var_rbodyint_rv = 0.0;

        let assign61790_e99173: f64 = if locals.var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard905 = assign61790_e99173;
        locals.var_guard905_rv = 0.0;

        let assign61800_e99176: f64 = if locals.var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard906 = assign61800_e99176;
        locals.var_guard906_rv = 0.0;

        let (assign61810_e99193, assign61810_e99193_d_n3, assign61810_e99193_d_n4, assign61810_e99193_d_n5, assign61810_e99193_d_n6, assign61810_e99193_d_n7, assign61810_e99193_d_n8, assign61810_e99193_d_n9, assign61810_e99193_d_n10, assign61810_e99193_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) && (locals.var_guard906 != 0.0)) {
        let assign61810_e99191: f64 = (1.0 / 0.001);
        (assign61810_e99191, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61810_e99193;
        locals.var_t0_dn3 = assign61810_e99193_d_n3;
        locals.var_t0_dn4 = assign61810_e99193_d_n4;
        locals.var_t0_dn5 = assign61810_e99193_d_n5;
        locals.var_t0_dn6 = assign61810_e99193_d_n6;
        locals.var_t0_dn7 = assign61810_e99193_d_n7;
        locals.var_t0_dn8 = assign61810_e99193_d_n8;
        locals.var_t0_dn9 = assign61810_e99193_d_n9;
        locals.var_t0_dn10 = assign61810_e99193_d_n10;
        locals.var_t0_dn11 = assign61810_e99193_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign61820_e99211, assign61820_e99211_d_n3, assign61820_e99211_d_n4, assign61820_e99211_d_n5, assign61820_e99211_d_n6, assign61820_e99211_d_n7, assign61820_e99211_d_n8, assign61820_e99211_d_n9, assign61820_e99211_d_n10, assign61820_e99211_d_n11,) = {
    if (((((locals.var_guard900 == 0.0) && (locals.var_guard901 == 0.0)) && (locals.var_guard902 == 0.0)) && (locals.var_guard905 != 0.0)) && (locals.var_guard906 == 0.0)) {
        let assign61820_e99209: f64 = (1.0 / locals.var_rbodyext);
        (assign61820_e99209, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61820_e99211;
        locals.var_t0_dn3 = assign61820_e99211_d_n3;
        locals.var_t0_dn4 = assign61820_e99211_d_n4;
        locals.var_t0_dn5 = assign61820_e99211_d_n5;
        locals.var_t0_dn6 = assign61820_e99211_d_n6;
        locals.var_t0_dn7 = assign61820_e99211_d_n7;
        locals.var_t0_dn8 = assign61820_e99211_d_n8;
        locals.var_t0_dn9 = assign61820_e99211_d_n9;
        locals.var_t0_dn10 = assign61820_e99211_d_n10;
        locals.var_t0_dn11 = assign61820_e99211_d_n11;
        locals.var_t0_rv = 0.0;

        let assign61870_e99254: f64 = if p.p1374 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard908 = assign61870_e99254;
        locals.var_guard908_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_213(
        locals: &mut StampLocals,
    ) {
        let (assign61880_e99260, assign61880_e99260_d_n3, assign61880_e99260_d_n4, assign61880_e99260_d_n5, assign61880_e99260_d_n6, assign61880_e99260_d_n7, assign61880_e99260_d_n8, assign61880_e99260_d_n9, assign61880_e99260_d_n10, assign61880_e99260_d_n11,) = {
    if (locals.var_guard908 != 0.0) {
        let assign61880_e99258: f64 = (1.0 / 0.001);
        (assign61880_e99258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign61880_e99260;
        locals.var_t0_dn3 = assign61880_e99260_d_n3;
        locals.var_t0_dn4 = assign61880_e99260_d_n4;
        locals.var_t0_dn5 = assign61880_e99260_d_n5;
        locals.var_t0_dn6 = assign61880_e99260_d_n6;
        locals.var_t0_dn7 = assign61880_e99260_d_n7;
        locals.var_t0_dn8 = assign61880_e99260_d_n8;
        locals.var_t0_dn9 = assign61880_e99260_d_n9;
        locals.var_t0_dn10 = assign61880_e99260_d_n10;
        locals.var_t0_dn11 = assign61880_e99260_d_n11;
        locals.var_t0_rv = 0.0;

    }
}
