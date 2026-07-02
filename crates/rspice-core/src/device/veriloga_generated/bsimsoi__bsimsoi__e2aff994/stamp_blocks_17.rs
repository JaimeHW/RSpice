#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_61(
        locals: &mut StampLocals,
    ) {
        let (assign22860_e34980, assign22860_e34980_d_n3, assign22860_e34980_d_n4, assign22860_e34980_d_n5, assign22860_e34980_d_n6, assign22860_e34980_d_n7, assign22860_e34980_d_n8, assign22860_e34980_d_n9, assign22860_e34980_d_n10, assign22860_e34980_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22860_e34978: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign22860_e34978, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign22860_e34980;
        locals.var_t4_dn3 = assign22860_e34980_d_n3;
        locals.var_t4_dn4 = assign22860_e34980_d_n4;
        locals.var_t4_dn5 = assign22860_e34980_d_n5;
        locals.var_t4_dn6 = assign22860_e34980_d_n6;
        locals.var_t4_dn7 = assign22860_e34980_d_n7;
        locals.var_t4_dn8 = assign22860_e34980_d_n8;
        locals.var_t4_dn9 = assign22860_e34980_d_n9;
        locals.var_t4_dn10 = assign22860_e34980_d_n10;
        locals.var_t4_dn11 = assign22860_e34980_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign22870_e34992, assign22870_e34992_d_n3, assign22870_e34992_d_n4, assign22870_e34992_d_n5, assign22870_e34992_d_n6, assign22870_e34992_d_n7, assign22870_e34992_d_n8, assign22870_e34992_d_n9, assign22870_e34992_d_n10, assign22870_e34992_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22870_e34989: f64 = (-locals.var_xs);
        let assign22870_e34990: f64 = { let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign22870_e34990, ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn3)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn4)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn5)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn6)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn7)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn8)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn9)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn10)), ({ let limited_exp_arg = assign22870_e34989; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign22870_e34992;
        locals.var_t6_dn3 = assign22870_e34992_d_n3;
        locals.var_t6_dn4 = assign22870_e34992_d_n4;
        locals.var_t6_dn5 = assign22870_e34992_d_n5;
        locals.var_t6_dn6 = assign22870_e34992_d_n6;
        locals.var_t6_dn7 = assign22870_e34992_d_n7;
        locals.var_t6_dn8 = assign22870_e34992_d_n8;
        locals.var_t6_dn9 = assign22870_e34992_d_n9;
        locals.var_t6_dn10 = assign22870_e34992_d_n10;
        locals.var_t6_dn11 = assign22870_e34992_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign22880_e35004, assign22880_e35004_d_n3, assign22880_e35004_d_n4, assign22880_e35004_d_n5, assign22880_e35004_d_n6, assign22880_e35004_d_n7, assign22880_e35004_d_n8, assign22880_e35004_d_n9, assign22880_e35004_d_n10, assign22880_e35004_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22880_e35002: f64 = (locals.var_xs * locals.var_xs);
        (assign22880_e35002, ((locals.var_xs_dn3 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn3)), ((locals.var_xs_dn4 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn4)), ((locals.var_xs_dn5 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn5)), ((locals.var_xs_dn6 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn6)), ((locals.var_xs_dn7 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn7)), ((locals.var_xs_dn8 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn8)), ((locals.var_xs_dn9 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn9)), ((locals.var_xs_dn10 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn10)), ((locals.var_xs_dn11 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn11)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign22880_e35004;
        locals.var_t7_dn3 = assign22880_e35004_d_n3;
        locals.var_t7_dn4 = assign22880_e35004_d_n4;
        locals.var_t7_dn5 = assign22880_e35004_d_n5;
        locals.var_t7_dn6 = assign22880_e35004_d_n6;
        locals.var_t7_dn7 = assign22880_e35004_d_n7;
        locals.var_t7_dn8 = assign22880_e35004_d_n8;
        locals.var_t7_dn9 = assign22880_e35004_d_n9;
        locals.var_t7_dn10 = assign22880_e35004_d_n10;
        locals.var_t7_dn11 = assign22880_e35004_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign22890_e35018, assign22890_e35018_d_n3, assign22890_e35018_d_n4, assign22890_e35018_d_n5, assign22890_e35018_d_n6, assign22890_e35018_d_n7, assign22890_e35018_d_n8, assign22890_e35018_d_n9, assign22890_e35018_d_n10, assign22890_e35018_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22890_e35015: f64 = (locals.var_t7 + 2.0);
        let assign22890_e35016: f64 = (1.0 / assign22890_e35015);
        (assign22890_e35016, (-(locals.var_t7_dn3 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn4 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn5 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn6 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn7 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn8 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn9 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn10 / (assign22890_e35015 * assign22890_e35015))), (-(locals.var_t7_dn11 / (assign22890_e35015 * assign22890_e35015))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign22890_e35018;
        locals.var_t8_dn3 = assign22890_e35018_d_n3;
        locals.var_t8_dn4 = assign22890_e35018_d_n4;
        locals.var_t8_dn5 = assign22890_e35018_d_n5;
        locals.var_t8_dn6 = assign22890_e35018_d_n6;
        locals.var_t8_dn7 = assign22890_e35018_d_n7;
        locals.var_t8_dn8 = assign22890_e35018_d_n8;
        locals.var_t8_dn9 = assign22890_e35018_d_n9;
        locals.var_t8_dn10 = assign22890_e35018_d_n10;
        locals.var_t8_dn11 = assign22890_e35018_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign22900_e35030, assign22900_e35030_d_n3, assign22900_e35030_d_n4, assign22900_e35030_d_n5, assign22900_e35030_d_n6, assign22900_e35030_d_n7, assign22900_e35030_d_n8, assign22900_e35030_d_n9, assign22900_e35030_d_n10, assign22900_e35030_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22900_e35027: f64 = (-locals.var_phidf);
        let assign22900_e35028: f64 = { let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign22900_e35028, ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn3)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn4)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn5)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn6)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn7)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn8)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn9)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn10)), ({ let limited_exp_arg = assign22900_e35027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn11)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign22900_e35030;
        locals.var_t9_dn3 = assign22900_e35030_d_n3;
        locals.var_t9_dn4 = assign22900_e35030_d_n4;
        locals.var_t9_dn5 = assign22900_e35030_d_n5;
        locals.var_t9_dn6 = assign22900_e35030_d_n6;
        locals.var_t9_dn7 = assign22900_e35030_d_n7;
        locals.var_t9_dn8 = assign22900_e35030_d_n8;
        locals.var_t9_dn9 = assign22900_e35030_d_n9;
        locals.var_t9_dn10 = assign22900_e35030_d_n10;
        locals.var_t9_dn11 = assign22900_e35030_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign22910_e35043, assign22910_e35043_d_n3, assign22910_e35043_d_n4, assign22910_e35043_d_n5, assign22910_e35043_d_n6, assign22910_e35043_d_n7, assign22910_e35043_d_n8, assign22910_e35043_d_n9, assign22910_e35043_d_n10, assign22910_e35043_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22910_e35040: f64 = (locals.var_xs - locals.var_phidf);
        let assign22910_e35041: f64 = { let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign22910_e35041, ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn3 - locals.var_phidf_dn3)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn4 - locals.var_phidf_dn4)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn5 - locals.var_phidf_dn5)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn6 - locals.var_phidf_dn6)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn7 - locals.var_phidf_dn7)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn8 - locals.var_phidf_dn8)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn9 - locals.var_phidf_dn9)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn10 - locals.var_phidf_dn10)), ({ let limited_exp_arg = assign22910_e35040; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn11 - locals.var_phidf_dn11)),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign22910_e35043;
        locals.var_t10_dn3 = assign22910_e35043_d_n3;
        locals.var_t10_dn4 = assign22910_e35043_d_n4;
        locals.var_t10_dn5 = assign22910_e35043_d_n5;
        locals.var_t10_dn6 = assign22910_e35043_d_n6;
        locals.var_t10_dn7 = assign22910_e35043_d_n7;
        locals.var_t10_dn8 = assign22910_e35043_d_n8;
        locals.var_t10_dn9 = assign22910_e35043_d_n9;
        locals.var_t10_dn10 = assign22910_e35043_d_n10;
        locals.var_t10_dn11 = assign22910_e35043_d_n11;
        locals.var_t10_rv = 0.0;

        let (assign22920_e35093, assign22920_e35093_d_n3, assign22920_e35093_d_n4, assign22920_e35093_d_n5, assign22920_e35093_d_n6, assign22920_e35093_d_n7, assign22920_e35093_d_n8, assign22920_e35093_d_n9, assign22920_e35093_d_n10, assign22920_e35093_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22920_e35053: f64 = (locals.var_vgfb1 - locals.var_xs);
        let assign22920_e35056: f64 = (locals.var_vgfb1 - locals.var_xs);
        let assign22920_e35057: f64 = (assign22920_e35053 * assign22920_e35056);
        let assign22920_e35060: f64 = (locals.var_rt * locals.var_rt);
        let assign22920_e35063: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign22920_e35064: f64 = (assign22920_e35060 * assign22920_e35063);
        let assign22920_e35067: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign22920_e35068: f64 = (assign22920_e35064 * assign22920_e35067);
        let assign22920_e35069: f64 = (assign22920_e35057 - assign22920_e35068);
        let assign22920_e35073: f64 = (locals.var_t6 - locals.var_t4);
        let assign22920_e35075: f64 = (assign22920_e35073 + locals.var_xs);
        let assign22920_e35077: f64 = (assign22920_e35075 + locals.var_t3);
        let assign22920_e35079: f64 = (assign22920_e35077 + locals.var_t10);
        let assign22920_e35083: f64 = (locals.var_xs + 1.0);
        let assign22920_e35086: f64 = (locals.var_t8 * locals.var_t7);
        let assign22920_e35087: f64 = (assign22920_e35083 + assign22920_e35086);
        let assign22920_e35088: f64 = (locals.var_t9 * assign22920_e35087);
        let assign22920_e35089: f64 = (assign22920_e35079 - assign22920_e35088);
        let assign22920_e35090: f64 = (locals.var_gam2 * assign22920_e35089);
        let assign22920_e35091: f64 = (assign22920_e35069 - assign22920_e35090);
        (assign22920_e35091, (((((locals.var_vgfb1_dn3 - locals.var_xs_dn3) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn3 - locals.var_xs_dn3))) - (((assign22920_e35060 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3)))) - ((locals.var_gam2_dn3 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn3 - locals.var_t4_dn3) + locals.var_xs_dn3) + locals.var_t3_dn3) + locals.var_t10_dn3) - ((locals.var_t9_dn3 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn3 + ((locals.var_t8_dn3 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn3))))))))), (((((locals.var_vgfb1_dn4 - locals.var_xs_dn4) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn4 - locals.var_xs_dn4))) - (((assign22920_e35060 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4)))) - ((locals.var_gam2_dn4 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn4 - locals.var_t4_dn4) + locals.var_xs_dn4) + locals.var_t3_dn4) + locals.var_t10_dn4) - ((locals.var_t9_dn4 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn4 + ((locals.var_t8_dn4 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn4))))))))), (((((locals.var_vgfb1_dn5 - locals.var_xs_dn5) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn5 - locals.var_xs_dn5))) - (((assign22920_e35060 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5)))) - ((locals.var_gam2_dn5 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn5 - locals.var_t4_dn5) + locals.var_xs_dn5) + locals.var_t3_dn5) + locals.var_t10_dn5) - ((locals.var_t9_dn5 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn5 + ((locals.var_t8_dn5 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn5))))))))), (((((locals.var_vgfb1_dn6 - locals.var_xs_dn6) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn6 - locals.var_xs_dn6))) - (((assign22920_e35060 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6)))) - ((locals.var_gam2_dn6 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn6 - locals.var_t4_dn6) + locals.var_xs_dn6) + locals.var_t3_dn6) + locals.var_t10_dn6) - ((locals.var_t9_dn6 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn6 + ((locals.var_t8_dn6 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn6))))))))), (((((locals.var_vgfb1_dn7 - locals.var_xs_dn7) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn7 - locals.var_xs_dn7))) - (((assign22920_e35060 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7)))) - ((locals.var_gam2_dn7 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn7 - locals.var_t4_dn7) + locals.var_xs_dn7) + locals.var_t3_dn7) + locals.var_t10_dn7) - ((locals.var_t9_dn7 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn7 + ((locals.var_t8_dn7 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn7))))))))), (((((locals.var_vgfb1_dn8 - locals.var_xs_dn8) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn8 - locals.var_xs_dn8))) - (((assign22920_e35060 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8)))) - ((locals.var_gam2_dn8 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn8 - locals.var_t4_dn8) + locals.var_xs_dn8) + locals.var_t3_dn8) + locals.var_t10_dn8) - ((locals.var_t9_dn8 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn8 + ((locals.var_t8_dn8 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn8))))))))), (((((locals.var_vgfb1_dn9 - locals.var_xs_dn9) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn9 - locals.var_xs_dn9))) - (((assign22920_e35060 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9)))) - ((locals.var_gam2_dn9 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn9 - locals.var_t4_dn9) + locals.var_xs_dn9) + locals.var_t3_dn9) + locals.var_t10_dn9) - ((locals.var_t9_dn9 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn9 + ((locals.var_t8_dn9 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn9))))))))), (((((locals.var_vgfb1_dn10 - locals.var_xs_dn10) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn10 - locals.var_xs_dn10))) - (((assign22920_e35060 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10)))) - ((locals.var_gam2_dn10 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn10 - locals.var_t4_dn10) + locals.var_xs_dn10) + locals.var_t3_dn10) + locals.var_t10_dn10) - ((locals.var_t9_dn10 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn10 + ((locals.var_t8_dn10 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn10))))))))), (((((locals.var_vgfb1_dn11 - locals.var_xs_dn11) * assign22920_e35056) + (assign22920_e35053 * (locals.var_vgfb1_dn11 - locals.var_xs_dn11))) - (((assign22920_e35060 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11)) * assign22920_e35067) + (assign22920_e35064 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11)))) - ((locals.var_gam2_dn11 * assign22920_e35089) + (locals.var_gam2 * (((((locals.var_t6_dn11 - locals.var_t4_dn11) + locals.var_xs_dn11) + locals.var_t3_dn11) + locals.var_t10_dn11) - ((locals.var_t9_dn11 * assign22920_e35087) + (locals.var_t9 * (locals.var_xs_dn11 + ((locals.var_t8_dn11 * locals.var_t7) + (locals.var_t8 * locals.var_t7_dn11))))))))),)
    } else {
        (locals.var_fx, locals.var_fx_dn3, locals.var_fx_dn4, locals.var_fx_dn5, locals.var_fx_dn6, locals.var_fx_dn7, locals.var_fx_dn8, locals.var_fx_dn9, locals.var_fx_dn10, locals.var_fx_dn11,)
    }
};
        locals.var_fx = assign22920_e35093;
        locals.var_fx_dn3 = assign22920_e35093_d_n3;
        locals.var_fx_dn4 = assign22920_e35093_d_n4;
        locals.var_fx_dn5 = assign22920_e35093_d_n5;
        locals.var_fx_dn6 = assign22920_e35093_d_n6;
        locals.var_fx_dn7 = assign22920_e35093_d_n7;
        locals.var_fx_dn8 = assign22920_e35093_d_n8;
        locals.var_fx_dn9 = assign22920_e35093_d_n9;
        locals.var_fx_dn10 = assign22920_e35093_d_n10;
        locals.var_fx_dn11 = assign22920_e35093_d_n11;
        locals.var_fx_rv = 0.0;

        let (assign22930_e35182, assign22930_e35182_d_n3, assign22930_e35182_d_n4, assign22930_e35182_d_n5, assign22930_e35182_d_n6, assign22930_e35182_d_n7, assign22930_e35182_d_n8, assign22930_e35182_d_n9, assign22930_e35182_d_n10, assign22930_e35182_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22930_e35103: f64 = (2.0 * locals.var_t0);
        let assign22930_e35106: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign22930_e35107: f64 = (assign22930_e35103 * assign22930_e35106);
        let assign22930_e35109: f64 = (assign22930_e35107 * locals.var_rt);
        let assign22930_e35111: f64 = (assign22930_e35109 * locals.var_rt);
        let assign22930_e35114: f64 = (1.0 + locals.var_rc);
        let assign22930_e35117: f64 = (1.0 + locals.var_t0);
        let assign22930_e35118: f64 = (assign22930_e35114 * assign22930_e35117);
        let assign22930_e35119: f64 = (assign22930_e35111 / assign22930_e35118);
        let assign22930_e35122: f64 = (2.0 * locals.var_vgfb1);
        let assign22930_e35123: f64 = (assign22930_e35119 - assign22930_e35122);
        let assign22930_e35126: f64 = (2.0 * locals.var_xs);
        let assign22930_e35127: f64 = (assign22930_e35123 + assign22930_e35126);
        let assign22930_e35132: f64 = (-2.0);
        let assign22930_e35134: f64 = (assign22930_e35132 * locals.var_xs);
        let assign22930_e35136: f64 = (assign22930_e35134 * locals.var_t8);
        let assign22930_e35139: f64 = (2.0 * locals.var_xs);
        let assign22930_e35141: f64 = (assign22930_e35139 * locals.var_xs);
        let assign22930_e35143: f64 = (assign22930_e35141 * locals.var_xs);
        let assign22930_e35145: f64 = (assign22930_e35143 * locals.var_t8);
        let assign22930_e35147: f64 = (assign22930_e35145 * locals.var_t8);
        let assign22930_e35148: f64 = (assign22930_e35136 + assign22930_e35147);
        let assign22930_e35150: f64 = (assign22930_e35148 - 1.0);
        let assign22930_e35151: f64 = (locals.var_t9 * assign22930_e35150);
        let assign22930_e35152: f64 = (locals.var_t10 + assign22930_e35151);
        let assign22930_e35154: f64 = (assign22930_e35152 - locals.var_t6);
        let assign22930_e35158: f64 = (1.0 + locals.var_rc);
        let assign22930_e35161: f64 = (1.0 + locals.var_t0);
        let assign22930_e35162: f64 = (assign22930_e35158 * assign22930_e35161);
        let assign22930_e35163: f64 = (locals.var_t0 / assign22930_e35162);
        let assign22930_e35164: f64 = (assign22930_e35154 - assign22930_e35163);
        let assign22930_e35167: f64 = (locals.var_t0 * locals.var_t4);
        let assign22930_e35170: f64 = (1.0 + locals.var_rc);
        let assign22930_e35173: f64 = (1.0 + locals.var_t0);
        let assign22930_e35174: f64 = (assign22930_e35170 * assign22930_e35173);
        let assign22930_e35175: f64 = (assign22930_e35167 / assign22930_e35174);
        let assign22930_e35176: f64 = (assign22930_e35164 + assign22930_e35175);
        let assign22930_e35178: f64 = (assign22930_e35176 + 1.0);
        let assign22930_e35179: f64 = (locals.var_gam2 * assign22930_e35178);
        let assign22930_e35180: f64 = (assign22930_e35127 - assign22930_e35179);
        (assign22930_e35180, (((((((((((2.0 * locals.var_t0_dn3) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn3))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn3)) + (2.0 * locals.var_xs_dn3)) - ((locals.var_gam2_dn3 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn3 + ((locals.var_t9_dn3 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn3) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn3)) + (((((((((2.0 * locals.var_xs_dn3) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn3)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn3)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn3)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn3)))))) - locals.var_t6_dn3) - (((locals.var_t0_dn3 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn3))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn3 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn3)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn3))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn4) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn4))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn4)) + (2.0 * locals.var_xs_dn4)) - ((locals.var_gam2_dn4 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn4 + ((locals.var_t9_dn4 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn4) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn4)) + (((((((((2.0 * locals.var_xs_dn4) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn4)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn4)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn4)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn4)))))) - locals.var_t6_dn4) - (((locals.var_t0_dn4 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn4))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn4 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn4)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn4))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn5) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn5))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn5)) + (2.0 * locals.var_xs_dn5)) - ((locals.var_gam2_dn5 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn5 + ((locals.var_t9_dn5 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn5) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn5)) + (((((((((2.0 * locals.var_xs_dn5) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn5)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn5)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn5)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn5)))))) - locals.var_t6_dn5) - (((locals.var_t0_dn5 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn5))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn5 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn5)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn5))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn6) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn6))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn6)) + (2.0 * locals.var_xs_dn6)) - ((locals.var_gam2_dn6 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn6 + ((locals.var_t9_dn6 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn6) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn6)) + (((((((((2.0 * locals.var_xs_dn6) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn6)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn6)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn6)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn6)))))) - locals.var_t6_dn6) - (((locals.var_t0_dn6 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn6))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn6 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn6)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn6))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn7) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn7))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn7)) + (2.0 * locals.var_xs_dn7)) - ((locals.var_gam2_dn7 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn7 + ((locals.var_t9_dn7 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn7) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn7)) + (((((((((2.0 * locals.var_xs_dn7) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn7)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn7)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn7)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn7)))))) - locals.var_t6_dn7) - (((locals.var_t0_dn7 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn7))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn7 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn7)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn7))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn8) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn8))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn8)) + (2.0 * locals.var_xs_dn8)) - ((locals.var_gam2_dn8 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn8 + ((locals.var_t9_dn8 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn8) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn8)) + (((((((((2.0 * locals.var_xs_dn8) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn8)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn8)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn8)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn8)))))) - locals.var_t6_dn8) - (((locals.var_t0_dn8 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn8))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn8 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn8)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn8))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn9) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn9))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn9)) + (2.0 * locals.var_xs_dn9)) - ((locals.var_gam2_dn9 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn9 + ((locals.var_t9_dn9 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn9) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn9)) + (((((((((2.0 * locals.var_xs_dn9) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn9)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn9)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn9)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn9)))))) - locals.var_t6_dn9) - (((locals.var_t0_dn9 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn9))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn9 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn9)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn9))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn10) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn10))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn10)) + (2.0 * locals.var_xs_dn10)) - ((locals.var_gam2_dn10 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn10 + ((locals.var_t9_dn10 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn10) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn10)) + (((((((((2.0 * locals.var_xs_dn10) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn10)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn10)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn10)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn10)))))) - locals.var_t6_dn10) - (((locals.var_t0_dn10 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn10))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn10 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn10)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn10))) / (assign22930_e35174 * assign22930_e35174)))))), (((((((((((2.0 * locals.var_t0_dn11) * assign22930_e35106) + (assign22930_e35103 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11))) * locals.var_rt) * locals.var_rt) * assign22930_e35118) - (assign22930_e35111 * (assign22930_e35114 * locals.var_t0_dn11))) / (assign22930_e35118 * assign22930_e35118)) - (2.0 * locals.var_vgfb1_dn11)) + (2.0 * locals.var_xs_dn11)) - ((locals.var_gam2_dn11 * assign22930_e35178) + (locals.var_gam2 * ((((locals.var_t10_dn11 + ((locals.var_t9_dn11 * assign22930_e35150) + (locals.var_t9 * ((((assign22930_e35132 * locals.var_xs_dn11) * locals.var_t8) + (assign22930_e35134 * locals.var_t8_dn11)) + (((((((((2.0 * locals.var_xs_dn11) * locals.var_xs) + (assign22930_e35139 * locals.var_xs_dn11)) * locals.var_xs) + (assign22930_e35141 * locals.var_xs_dn11)) * locals.var_t8) + (assign22930_e35143 * locals.var_t8_dn11)) * locals.var_t8) + (assign22930_e35145 * locals.var_t8_dn11)))))) - locals.var_t6_dn11) - (((locals.var_t0_dn11 * assign22930_e35162) - (locals.var_t0 * (assign22930_e35158 * locals.var_t0_dn11))) / (assign22930_e35162 * assign22930_e35162))) + (((((locals.var_t0_dn11 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn11)) * assign22930_e35174) - (assign22930_e35167 * (assign22930_e35170 * locals.var_t0_dn11))) / (assign22930_e35174 * assign22930_e35174)))))),)
    } else {
        (locals.var_f_x, locals.var_f_x_dn3, locals.var_f_x_dn4, locals.var_f_x_dn5, locals.var_f_x_dn6, locals.var_f_x_dn7, locals.var_f_x_dn8, locals.var_f_x_dn9, locals.var_f_x_dn10, locals.var_f_x_dn11,)
    }
};
        locals.var_f_x = assign22930_e35182;
        locals.var_f_x_dn3 = assign22930_e35182_d_n3;
        locals.var_f_x_dn4 = assign22930_e35182_d_n4;
        locals.var_f_x_dn5 = assign22930_e35182_d_n5;
        locals.var_f_x_dn6 = assign22930_e35182_d_n6;
        locals.var_f_x_dn7 = assign22930_e35182_d_n7;
        locals.var_f_x_dn8 = assign22930_e35182_d_n8;
        locals.var_f_x_dn9 = assign22930_e35182_d_n9;
        locals.var_f_x_dn10 = assign22930_e35182_d_n10;
        locals.var_f_x_dn11 = assign22930_e35182_d_n11;
        locals.var_f_x_rv = 0.0;

        let (assign22940_e35326, assign22940_e35326_d_n3, assign22940_e35326_d_n4, assign22940_e35326_d_n5, assign22940_e35326_d_n6, assign22940_e35326_d_n7, assign22940_e35326_d_n8, assign22940_e35326_d_n9, assign22940_e35326_d_n10, assign22940_e35326_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22940_e35192: f64 = (2.0 * locals.var_rt);
        let assign22940_e35194: f64 = (assign22940_e35192 * locals.var_rt);
        let assign22940_e35196: f64 = (assign22940_e35194 * locals.var_t0);
        let assign22940_e35199: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign22940_e35200: f64 = (assign22940_e35196 * assign22940_e35199);
        let assign22940_e35203: f64 = (1.0 + locals.var_rc);
        let assign22940_e35206: f64 = (1.0 + locals.var_t0);
        let assign22940_e35207: f64 = (assign22940_e35203 * assign22940_e35206);
        let assign22940_e35208: f64 = (assign22940_e35200 / assign22940_e35207);
        let assign22940_e35211: f64 = (2.0 * locals.var_rt);
        let assign22940_e35213: f64 = (assign22940_e35211 * locals.var_rt);
        let assign22940_e35215: f64 = (assign22940_e35213 * locals.var_t0);
        let assign22940_e35217: f64 = (assign22940_e35215 * locals.var_t0);
        let assign22940_e35220: f64 = (1.0 + locals.var_rc);
        let assign22940_e35223: f64 = (1.0 + locals.var_t0);
        let assign22940_e35224: f64 = (assign22940_e35220 * assign22940_e35223);
        let assign22940_e35227: f64 = (1.0 + locals.var_rc);
        let assign22940_e35228: f64 = (assign22940_e35224 * assign22940_e35227);
        let assign22940_e35231: f64 = (1.0 + locals.var_t0);
        let assign22940_e35232: f64 = (assign22940_e35228 * assign22940_e35231);
        let assign22940_e35233: f64 = (assign22940_e35217 / assign22940_e35232);
        let assign22940_e35234: f64 = (assign22940_e35208 - assign22940_e35233);
        let assign22940_e35238: f64 = (locals.var_t6 + locals.var_t10);
        let assign22940_e35241: f64 = (2.0 * locals.var_t9);
        let assign22940_e35243: f64 = (assign22940_e35241 * locals.var_t8);
        let assign22940_e35247: f64 = (locals.var_t7 * locals.var_t8);
        let assign22940_e35251: f64 = (4.0 * locals.var_t7);
        let assign22940_e35253: f64 = (assign22940_e35251 * locals.var_t8);
        let assign22940_e35254: f64 = (5.0 - assign22940_e35253);
        let assign22940_e35255: f64 = (assign22940_e35247 * assign22940_e35254);
        let assign22940_e35256: f64 = (1.0 - assign22940_e35255);
        let assign22940_e35257: f64 = (assign22940_e35243 * assign22940_e35256);
        let assign22940_e35258: f64 = (assign22940_e35238 - assign22940_e35257);
        let assign22940_e35262: f64 = (1.0 + locals.var_rc);
        let assign22940_e35265: f64 = (1.0 + locals.var_t0);
        let assign22940_e35266: f64 = (assign22940_e35262 * assign22940_e35265);
        let assign22940_e35267: f64 = (locals.var_t0 / assign22940_e35266);
        let assign22940_e35272: f64 = (1.0 + locals.var_t0);
        let assign22940_e35273: f64 = (locals.var_t0 / assign22940_e35272);
        let assign22940_e35274: f64 = (1.0 - assign22940_e35273);
        let assign22940_e35276: f64 = (assign22940_e35274 - locals.var_t4);
        let assign22940_e35279: f64 = (locals.var_t0 * locals.var_t4);
        let assign22940_e35282: f64 = (1.0 + locals.var_t0);
        let assign22940_e35283: f64 = (assign22940_e35279 / assign22940_e35282);
        let assign22940_e35288: f64 = (1.0 + locals.var_rc);
        let assign22940_e35289: f64 = (1.0 / assign22940_e35288);
        let assign22940_e35290: f64 = (1.0 + assign22940_e35289);
        let assign22940_e35291: f64 = (assign22940_e35283 * assign22940_e35290);
        let assign22940_e35292: f64 = (assign22940_e35276 + assign22940_e35291);
        let assign22940_e35293: f64 = (assign22940_e35267 * assign22940_e35292);
        let assign22940_e35294: f64 = (assign22940_e35258 - assign22940_e35293);
        let assign22940_e35295: f64 = (locals.var_gam2 * assign22940_e35294);
        let assign22940_e35296: f64 = (assign22940_e35234 - assign22940_e35295);
        let assign22940_e35299: f64 = (2.0 * locals.var_rt);
        let assign22940_e35301: f64 = (assign22940_e35299 * locals.var_rt);
        let assign22940_e35303: f64 = (assign22940_e35301 * locals.var_t0);
        let assign22940_e35305: f64 = (assign22940_e35303 * locals.var_t0);
        let assign22940_e35308: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign22940_e35309: f64 = (assign22940_e35305 * assign22940_e35308);
        let assign22940_e35312: f64 = (1.0 + locals.var_rc);
        let assign22940_e35315: f64 = (1.0 + locals.var_t0);
        let assign22940_e35316: f64 = (assign22940_e35312 * assign22940_e35315);
        let assign22940_e35319: f64 = (1.0 + locals.var_t0);
        let assign22940_e35320: f64 = (assign22940_e35316 * assign22940_e35319);
        let assign22940_e35321: f64 = (assign22940_e35309 / assign22940_e35320);
        let assign22940_e35322: f64 = (assign22940_e35296 - assign22940_e35321);
        let assign22940_e35324: f64 = (assign22940_e35322 + 2.0);
        (assign22940_e35324, (((((((((assign22940_e35194 * locals.var_t0_dn3) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn3))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn3) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn3)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn3) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn3)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn3 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn3 + locals.var_t10_dn3) - (((((2.0 * locals.var_t9_dn3) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn3)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn3 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn3)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn3) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn3))))))))) - (((((locals.var_t0_dn3 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn3))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn3 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn3)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn3) + ((((((locals.var_t0_dn3 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn3)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn3)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn3) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn3)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn3) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn3)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn4) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn4))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn4) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn4)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn4) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn4)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn4 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn4 + locals.var_t10_dn4) - (((((2.0 * locals.var_t9_dn4) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn4)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn4 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn4)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn4) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn4))))))))) - (((((locals.var_t0_dn4 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn4))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn4 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn4)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn4) + ((((((locals.var_t0_dn4 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn4)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn4)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn4) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn4)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn4) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn4)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn5) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn5))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn5) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn5)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn5) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn5)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn5 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn5 + locals.var_t10_dn5) - (((((2.0 * locals.var_t9_dn5) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn5)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn5 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn5)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn5) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn5))))))))) - (((((locals.var_t0_dn5 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn5))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn5 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn5)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn5) + ((((((locals.var_t0_dn5 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn5)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn5)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn5) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn5)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn5) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn5)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn6) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn6))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn6) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn6)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn6) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn6)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn6 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn6 + locals.var_t10_dn6) - (((((2.0 * locals.var_t9_dn6) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn6)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn6 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn6)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn6) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn6))))))))) - (((((locals.var_t0_dn6 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn6))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn6 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn6)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn6) + ((((((locals.var_t0_dn6 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn6)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn6)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn6) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn6)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn6) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn6)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn7) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn7))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn7) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn7)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn7) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn7)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn7 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn7 + locals.var_t10_dn7) - (((((2.0 * locals.var_t9_dn7) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn7)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn7 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn7)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn7) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn7))))))))) - (((((locals.var_t0_dn7 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn7))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn7 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn7)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn7) + ((((((locals.var_t0_dn7 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn7)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn7)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn7) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn7)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn7) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn7)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn8) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn8))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn8) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn8)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn8) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn8)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn8 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn8 + locals.var_t10_dn8) - (((((2.0 * locals.var_t9_dn8) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn8)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn8 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn8)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn8) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn8))))))))) - (((((locals.var_t0_dn8 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn8))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn8 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn8)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn8) + ((((((locals.var_t0_dn8 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn8)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn8)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn8) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn8)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn8) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn8)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn9) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn9))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn9) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn9)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn9) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn9)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn9 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn9 + locals.var_t10_dn9) - (((((2.0 * locals.var_t9_dn9) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn9)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn9 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn9)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn9) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn9))))))))) - (((((locals.var_t0_dn9 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn9))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn9 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn9)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn9) + ((((((locals.var_t0_dn9 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn9)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn9)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn9) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn9)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn9) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn9)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn10) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn10))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn10) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn10)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn10) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn10)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn10 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn10 + locals.var_t10_dn10) - (((((2.0 * locals.var_t9_dn10) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn10)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn10 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn10)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn10) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn10))))))))) - (((((locals.var_t0_dn10 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn10))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn10 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn10)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn10) + ((((((locals.var_t0_dn10 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn10)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn10)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn10) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn10)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn10) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn10)))) / (assign22940_e35320 * assign22940_e35320))), (((((((((assign22940_e35194 * locals.var_t0_dn11) * assign22940_e35199) + (assign22940_e35196 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11))) * assign22940_e35207) - (assign22940_e35200 * (assign22940_e35203 * locals.var_t0_dn11))) / (assign22940_e35207 * assign22940_e35207)) - ((((((assign22940_e35213 * locals.var_t0_dn11) * locals.var_t0) + (assign22940_e35215 * locals.var_t0_dn11)) * assign22940_e35232) - (assign22940_e35217 * ((((assign22940_e35220 * locals.var_t0_dn11) * assign22940_e35227) * assign22940_e35231) + (assign22940_e35228 * locals.var_t0_dn11)))) / (assign22940_e35232 * assign22940_e35232))) - ((locals.var_gam2_dn11 * assign22940_e35294) + (locals.var_gam2 * (((locals.var_t6_dn11 + locals.var_t10_dn11) - (((((2.0 * locals.var_t9_dn11) * locals.var_t8) + (assign22940_e35241 * locals.var_t8_dn11)) * assign22940_e35256) + (assign22940_e35243 * (-((((locals.var_t7_dn11 * locals.var_t8) + (locals.var_t7 * locals.var_t8_dn11)) * assign22940_e35254) + (assign22940_e35247 * (-(((4.0 * locals.var_t7_dn11) * locals.var_t8) + (assign22940_e35251 * locals.var_t8_dn11))))))))) - (((((locals.var_t0_dn11 * assign22940_e35266) - (locals.var_t0 * (assign22940_e35262 * locals.var_t0_dn11))) / (assign22940_e35266 * assign22940_e35266)) * assign22940_e35292) + (assign22940_e35267 * (((-(((locals.var_t0_dn11 * assign22940_e35272) - (locals.var_t0 * locals.var_t0_dn11)) / (assign22940_e35272 * assign22940_e35272))) - locals.var_t4_dn11) + ((((((locals.var_t0_dn11 * locals.var_t4) + (locals.var_t0 * locals.var_t4_dn11)) * assign22940_e35282) - (assign22940_e35279 * locals.var_t0_dn11)) / (assign22940_e35282 * assign22940_e35282)) * assign22940_e35290)))))))) - ((((((((assign22940_e35301 * locals.var_t0_dn11) * locals.var_t0) + (assign22940_e35303 * locals.var_t0_dn11)) * assign22940_e35308) + (assign22940_e35305 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11))) * assign22940_e35320) - (assign22940_e35309 * (((assign22940_e35312 * locals.var_t0_dn11) * assign22940_e35319) + (assign22940_e35316 * locals.var_t0_dn11)))) / (assign22940_e35320 * assign22940_e35320))),)
    } else {
        (locals.var_f_xx, locals.var_f_xx_dn3, locals.var_f_xx_dn4, locals.var_f_xx_dn5, locals.var_f_xx_dn6, locals.var_f_xx_dn7, locals.var_f_xx_dn8, locals.var_f_xx_dn9, locals.var_f_xx_dn10, locals.var_f_xx_dn11,)
    }
};
        locals.var_f_xx = assign22940_e35326;
        locals.var_f_xx_dn3 = assign22940_e35326_d_n3;
        locals.var_f_xx_dn4 = assign22940_e35326_d_n4;
        locals.var_f_xx_dn5 = assign22940_e35326_d_n5;
        locals.var_f_xx_dn6 = assign22940_e35326_d_n6;
        locals.var_f_xx_dn7 = assign22940_e35326_d_n7;
        locals.var_f_xx_dn8 = assign22940_e35326_d_n8;
        locals.var_f_xx_dn9 = assign22940_e35326_d_n9;
        locals.var_f_xx_dn10 = assign22940_e35326_d_n10;
        locals.var_f_xx_dn11 = assign22940_e35326_d_n11;
        locals.var_f_xx_rv = 0.0;

        let (assign22950_e35352, assign22950_e35352_d_n3, assign22950_e35352_d_n4, assign22950_e35352_d_n5, assign22950_e35352_d_n6, assign22950_e35352_d_n7, assign22950_e35352_d_n8, assign22950_e35352_d_n9, assign22950_e35352_d_n10, assign22950_e35352_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard520 == 0.0)) {
        let assign22950_e35337: f64 = (locals.var_fx / locals.var_f_x);
        let assign22950_e35341: f64 = (locals.var_fx * locals.var_f_xx);
        let assign22950_e35344: f64 = (2.0 * locals.var_f_x);
        let assign22950_e35346: f64 = (assign22950_e35344 * locals.var_f_x);
        let assign22950_e35347: f64 = (assign22950_e35341 / assign22950_e35346);
        let assign22950_e35348: f64 = (1.0 + assign22950_e35347);
        let assign22950_e35349: f64 = (assign22950_e35337 * assign22950_e35348);
        let assign22950_e35350: f64 = (locals.var_xs - assign22950_e35349);
        (assign22950_e35350, (locals.var_xs_dn3 - (((((locals.var_fx_dn3 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn3)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn3 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn3)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn3) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn3)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn4 - (((((locals.var_fx_dn4 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn4)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn4 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn4)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn4) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn4)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn5 - (((((locals.var_fx_dn5 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn5)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn5 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn5)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn5) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn5)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn6 - (((((locals.var_fx_dn6 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn6)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn6 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn6)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn6) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn6)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn7 - (((((locals.var_fx_dn7 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn7)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn7 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn7)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn7) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn7)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn8 - (((((locals.var_fx_dn8 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn8)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn8 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn8)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn8) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn8)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn9 - (((((locals.var_fx_dn9 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn9)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn9 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn9)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn9) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn9)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn10 - (((((locals.var_fx_dn10 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn10)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn10 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn10)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn10) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn10)))) / (assign22950_e35346 * assign22950_e35346))))), (locals.var_xs_dn11 - (((((locals.var_fx_dn11 * locals.var_f_x) - (locals.var_fx * locals.var_f_x_dn11)) / (locals.var_f_x * locals.var_f_x)) * assign22950_e35348) + (assign22950_e35337 * (((((locals.var_fx_dn11 * locals.var_f_xx) + (locals.var_fx * locals.var_f_xx_dn11)) * assign22950_e35346) - (assign22950_e35341 * (((2.0 * locals.var_f_x_dn11) * locals.var_f_x) + (assign22950_e35344 * locals.var_f_x_dn11)))) / (assign22950_e35346 * assign22950_e35346))))),)
    } else {
        (locals.var_x7_d, locals.var_x7_d_dn3, locals.var_x7_d_dn4, locals.var_x7_d_dn5, locals.var_x7_d_dn6, locals.var_x7_d_dn7, locals.var_x7_d_dn8, locals.var_x7_d_dn9, locals.var_x7_d_dn10, locals.var_x7_d_dn11,)
    }
};
        locals.var_x7_d = assign22950_e35352;
        locals.var_x7_d_dn3 = assign22950_e35352_d_n3;
        locals.var_x7_d_dn4 = assign22950_e35352_d_n4;
        locals.var_x7_d_dn5 = assign22950_e35352_d_n5;
        locals.var_x7_d_dn6 = assign22950_e35352_d_n6;
        locals.var_x7_d_dn7 = assign22950_e35352_d_n7;
        locals.var_x7_d_dn8 = assign22950_e35352_d_n8;
        locals.var_x7_d_dn9 = assign22950_e35352_d_n9;
        locals.var_x7_d_dn10 = assign22950_e35352_d_n10;
        locals.var_x7_d_dn11 = assign22950_e35352_d_n11;
        locals.var_x7_d_rv = 0.0;

        let (assign22960_e35361, assign22960_e35361_d_n3, assign22960_e35361_d_n4, assign22960_e35361_d_n5, assign22960_e35361_d_n6, assign22960_e35361_d_n7, assign22960_e35361_d_n8, assign22960_e35361_d_n9, assign22960_e35361_d_n10, assign22960_e35361_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign22960_e35359: f64 = (locals.var_x7_d - locals.var_x7_s);
        (assign22960_e35359, (locals.var_x7_d_dn3 - locals.var_x7_s_dn3), (locals.var_x7_d_dn4 - locals.var_x7_s_dn4), (locals.var_x7_d_dn5 - locals.var_x7_s_dn5), (locals.var_x7_d_dn6 - locals.var_x7_s_dn6), (locals.var_x7_d_dn7 - locals.var_x7_s_dn7), (locals.var_x7_d_dn8 - locals.var_x7_s_dn8), (locals.var_x7_d_dn9 - locals.var_x7_s_dn9), (locals.var_x7_d_dn10 - locals.var_x7_s_dn10), (locals.var_x7_d_dn11 - locals.var_x7_s_dn11),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn3, locals.var_x_ds_dn4, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9, locals.var_x_ds_dn10, locals.var_x_ds_dn11,)
    }
};
        locals.var_x_ds = assign22960_e35361;
        locals.var_x_ds_dn3 = assign22960_e35361_d_n3;
        locals.var_x_ds_dn4 = assign22960_e35361_d_n4;
        locals.var_x_ds_dn5 = assign22960_e35361_d_n5;
        locals.var_x_ds_dn6 = assign22960_e35361_d_n6;
        locals.var_x_ds_dn7 = assign22960_e35361_d_n7;
        locals.var_x_ds_dn8 = assign22960_e35361_d_n8;
        locals.var_x_ds_dn9 = assign22960_e35361_d_n9;
        locals.var_x_ds_dn10 = assign22960_e35361_d_n10;
        locals.var_x_ds_dn11 = assign22960_e35361_d_n11;
        locals.var_x_ds_rv = 0.0;

        let (assign22970_e35370, assign22970_e35370_d_n3, assign22970_e35370_d_n4, assign22970_e35370_d_n5, assign22970_e35370_d_n6, assign22970_e35370_d_n7, assign22970_e35370_d_n8, assign22970_e35370_d_n9, assign22970_e35370_d_n10, assign22970_e35370_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign22970_e35368: f64 = (locals.var_vds * locals.var_inv_nvt);
        (assign22970_e35368, (locals.var_vds * locals.var_inv_nvt_dn3), (locals.var_vds * locals.var_inv_nvt_dn4), (locals.var_vds * locals.var_inv_nvt_dn5), ((locals.var_vds_dn6 * locals.var_inv_nvt) + (locals.var_vds * locals.var_inv_nvt_dn6)), ((locals.var_vds_dn7 * locals.var_inv_nvt) + (locals.var_vds * locals.var_inv_nvt_dn7)), (locals.var_vds * locals.var_inv_nvt_dn8), (locals.var_vds * locals.var_inv_nvt_dn9), ((locals.var_vds_dn10 * locals.var_inv_nvt) + (locals.var_vds * locals.var_inv_nvt_dn10)), (locals.var_vds * locals.var_inv_nvt_dn11),)
    } else {
        (locals.var_udse, locals.var_udse_dn3, locals.var_udse_dn4, locals.var_udse_dn5, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8, locals.var_udse_dn9, locals.var_udse_dn10, locals.var_udse_dn11,)
    }
};
        locals.var_udse = assign22970_e35370;
        locals.var_udse_dn3 = assign22970_e35370_d_n3;
        locals.var_udse_dn4 = assign22970_e35370_d_n4;
        locals.var_udse_dn5 = assign22970_e35370_d_n5;
        locals.var_udse_dn6 = assign22970_e35370_d_n6;
        locals.var_udse_dn7 = assign22970_e35370_d_n7;
        locals.var_udse_dn8 = assign22970_e35370_d_n8;
        locals.var_udse_dn9 = assign22970_e35370_d_n9;
        locals.var_udse_dn10 = assign22970_e35370_d_n10;
        locals.var_udse_dn11 = assign22970_e35370_d_n11;
        locals.var_udse_rv = 0.0;

        let (assign22980_e35379, assign22980_e35379_d_n3, assign22980_e35379_d_n4, assign22980_e35379_d_n5, assign22980_e35379_d_n6, assign22980_e35379_d_n7, assign22980_e35379_d_n8, assign22980_e35379_d_n9, assign22980_e35379_d_n10, assign22980_e35379_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign22980_e35376: f64 = (-locals.var_udse);
        let assign22980_e35377: f64 = { let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign22980_e35377, ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn3)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn4)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn5)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn6)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn7)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn8)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn9)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn10)), ({ let limited_exp_arg = assign22980_e35376; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_udse_dn11)),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn3, locals.var_k_ds_dn4, locals.var_k_ds_dn5, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, locals.var_k_ds_dn9, locals.var_k_ds_dn10, locals.var_k_ds_dn11,)
    }
};
        locals.var_k_ds = assign22980_e35379;
        locals.var_k_ds_dn3 = assign22980_e35379_d_n3;
        locals.var_k_ds_dn4 = assign22980_e35379_d_n4;
        locals.var_k_ds_dn5 = assign22980_e35379_d_n5;
        locals.var_k_ds_dn6 = assign22980_e35379_d_n6;
        locals.var_k_ds_dn7 = assign22980_e35379_d_n7;
        locals.var_k_ds_dn8 = assign22980_e35379_d_n8;
        locals.var_k_ds_dn9 = assign22980_e35379_d_n9;
        locals.var_k_ds_dn10 = assign22980_e35379_d_n10;
        locals.var_k_ds_dn11 = assign22980_e35379_d_n11;
        locals.var_k_ds_rv = 0.0;

        let assign22990_e35382: f64 = if locals.var_x_ds < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard521 = assign22990_e35382;
        locals.var_guard521_rv = 0.0;

        let (assign23000_e35398, assign23000_e35398_d_n3, assign23000_e35398_d_n4, assign23000_e35398_d_n5, assign23000_e35398_d_n6, assign23000_e35398_d_n7, assign23000_e35398_d_n8, assign23000_e35398_d_n9, assign23000_e35398_d_n10, assign23000_e35398_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23000_e35391: f64 = (locals.var_xs * locals.var_nvt);
        let assign23000_e35393: f64 = (assign23000_e35391 - locals.var_phic_s);
        let assign23000_e35395: f64 = (assign23000_e35393 / locals.var_nvt);
        let assign23000_e35396: f64 = { let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23000_e35396, ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23000_e35395; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23000_e35393 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign23000_e35398;
        locals.var_t0_dn3 = assign23000_e35398_d_n3;
        locals.var_t0_dn4 = assign23000_e35398_d_n4;
        locals.var_t0_dn5 = assign23000_e35398_d_n5;
        locals.var_t0_dn6 = assign23000_e35398_d_n6;
        locals.var_t0_dn7 = assign23000_e35398_d_n7;
        locals.var_t0_dn8 = assign23000_e35398_d_n8;
        locals.var_t0_dn9 = assign23000_e35398_d_n9;
        locals.var_t0_dn10 = assign23000_e35398_d_n10;
        locals.var_t0_dn11 = assign23000_e35398_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign23010_e35414, assign23010_e35414_d_n3, assign23010_e35414_d_n4, assign23010_e35414_d_n5, assign23010_e35414_d_n6, assign23010_e35414_d_n7, assign23010_e35414_d_n8, assign23010_e35414_d_n9, assign23010_e35414_d_n10, assign23010_e35414_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23010_e35407: f64 = (locals.var_sp_vfb * locals.var_nvt);
        let assign23010_e35409: f64 = (assign23010_e35407 - locals.var_phic_s);
        let assign23010_e35411: f64 = (assign23010_e35409 / locals.var_nvt);
        let assign23010_e35412: f64 = { let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23010_e35412, ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn3 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn4 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn5 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn6 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn7 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn8 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn9 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn10 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23010_e35411; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_sp_vfb_dn11 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23010_e35409 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign23010_e35414;
        locals.var_t1_dn3 = assign23010_e35414_d_n3;
        locals.var_t1_dn4 = assign23010_e35414_d_n4;
        locals.var_t1_dn5 = assign23010_e35414_d_n5;
        locals.var_t1_dn6 = assign23010_e35414_d_n6;
        locals.var_t1_dn7 = assign23010_e35414_d_n7;
        locals.var_t1_dn8 = assign23010_e35414_d_n8;
        locals.var_t1_dn9 = assign23010_e35414_d_n9;
        locals.var_t1_dn10 = assign23010_e35414_d_n10;
        locals.var_t1_dn11 = assign23010_e35414_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        locals: &mut StampLocals,
    ) {
        let (assign23020_e35587, assign23020_e35587_d_n3, assign23020_e35587_d_n4, assign23020_e35587_d_n5, assign23020_e35587_d_n6, assign23020_e35587_d_n7, assign23020_e35587_d_n8, assign23020_e35587_d_n9, assign23020_e35587_d_n10, assign23020_e35587_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23020_e35423: f64 = (locals.var_xs * locals.var_nvt);
        let assign23020_e35425: f64 = (assign23020_e35423 - locals.var_phic_s);
        let assign23020_e35427: f64 = (assign23020_e35425 / locals.var_nvt);
        let assign23020_e35433: f64 = (locals.var_xs * locals.var_nvt);
        let assign23020_e35435: f64 = (assign23020_e35433 - locals.var_phic_s);
        let assign23020_e35437: f64 = (assign23020_e35435 / locals.var_nvt);
        let assign23020_e35439: f64 = (-37.0);
        let (assign23020_e35502, assign23020_e35502_d_n3, assign23020_e35502_d_n4, assign23020_e35502_d_n5, assign23020_e35502_d_n6, assign23020_e35502_d_n7, assign23020_e35502_d_n8, assign23020_e35502_d_n9, assign23020_e35502_d_n10, assign23020_e35502_d_n11,) = {
            if ((!(assign23020_e35427 > 37.0)) && (!(assign23020_e35437 < assign23020_e35439))) {
                let assign23020_e35446: f64 = (locals.var_xs * locals.var_nvt);
                let assign23020_e35448: f64 = (assign23020_e35446 - locals.var_phic_s);
                let assign23020_e35450: f64 = (assign23020_e35448 / locals.var_nvt);
                let assign23020_e35451: f64 = (assign23020_e35450).exp();
                let assign23020_e35452: f64 = (1.0 + assign23020_e35451);
                let assign23020_e35453: f64 = (assign23020_e35452).ln();
                (assign23020_e35453, ((assign23020_e35451 * ((((((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452), ((assign23020_e35451 * ((((((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23020_e35448 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35452),)
            } else {
                let assign23020_e35456: f64 = (locals.var_xs * locals.var_nvt);
                let assign23020_e35458: f64 = (assign23020_e35456 - locals.var_phic_s);
                let assign23020_e35460: f64 = (assign23020_e35458 / locals.var_nvt);
                let assign23020_e35466: f64 = (locals.var_xs * locals.var_nvt);
                let assign23020_e35468: f64 = (assign23020_e35466 - locals.var_phic_s);
                let assign23020_e35470: f64 = (assign23020_e35468 / locals.var_nvt);
                let assign23020_e35472: f64 = (-37.0);
                let (assign23020_e35501, assign23020_e35501_d_n3, assign23020_e35501_d_n4, assign23020_e35501_d_n5, assign23020_e35501_d_n6, assign23020_e35501_d_n7, assign23020_e35501_d_n8, assign23020_e35501_d_n9, assign23020_e35501_d_n10, assign23020_e35501_d_n11,) = {
                    if ((!(assign23020_e35460 > 37.0)) && (assign23020_e35470 < assign23020_e35472)) {
                        let assign23020_e35477: f64 = (locals.var_xs * locals.var_nvt);
                        let assign23020_e35479: f64 = (assign23020_e35477 - locals.var_phic_s);
                        let assign23020_e35481: f64 = (assign23020_e35479 / locals.var_nvt);
                        let assign23020_e35482: f64 = (assign23020_e35481).exp();
                        (assign23020_e35482, (assign23020_e35482 * ((((((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35482 * ((((((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23020_e35479 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))),)
                    } else {
                        let assign23020_e35485: f64 = (locals.var_xs * locals.var_nvt);
                        let assign23020_e35487: f64 = (assign23020_e35485 - locals.var_phic_s);
                        let assign23020_e35489: f64 = (assign23020_e35487 / locals.var_nvt);
                        let (assign23020_e35500, assign23020_e35500_d_n3, assign23020_e35500_d_n4, assign23020_e35500_d_n5, assign23020_e35500_d_n6, assign23020_e35500_d_n7, assign23020_e35500_d_n8, assign23020_e35500_d_n9, assign23020_e35500_d_n10, assign23020_e35500_d_n11,) = {
                            if (assign23020_e35489 > 37.0) {
                                let assign23020_e35494: f64 = (locals.var_xs * locals.var_nvt);
                                let assign23020_e35496: f64 = (assign23020_e35494 - locals.var_phic_s);
                                let assign23020_e35498: f64 = (assign23020_e35496 / locals.var_nvt);
                                (assign23020_e35498, ((((((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23020_e35496 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt)),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign23020_e35500, assign23020_e35500_d_n3, assign23020_e35500_d_n4, assign23020_e35500_d_n5, assign23020_e35500_d_n6, assign23020_e35500_d_n7, assign23020_e35500_d_n8, assign23020_e35500_d_n9, assign23020_e35500_d_n10, assign23020_e35500_d_n11,)
                    }
                };
                (assign23020_e35501, assign23020_e35501_d_n3, assign23020_e35501_d_n4, assign23020_e35501_d_n5, assign23020_e35501_d_n6, assign23020_e35501_d_n7, assign23020_e35501_d_n8, assign23020_e35501_d_n9, assign23020_e35501_d_n10, assign23020_e35501_d_n11,)
            }
        };
        let assign23020_e35505: f64 = (locals.var_sp_vfb * locals.var_nvt);
        let assign23020_e35507: f64 = (assign23020_e35505 - locals.var_phic_s);
        let assign23020_e35509: f64 = (assign23020_e35507 / locals.var_nvt);
        let assign23020_e35515: f64 = (locals.var_sp_vfb * locals.var_nvt);
        let assign23020_e35517: f64 = (assign23020_e35515 - locals.var_phic_s);
        let assign23020_e35519: f64 = (assign23020_e35517 / locals.var_nvt);
        let assign23020_e35521: f64 = (-37.0);
        let (assign23020_e35584, assign23020_e35584_d_n3, assign23020_e35584_d_n4, assign23020_e35584_d_n5, assign23020_e35584_d_n6, assign23020_e35584_d_n7, assign23020_e35584_d_n8, assign23020_e35584_d_n9, assign23020_e35584_d_n10, assign23020_e35584_d_n11,) = {
            if ((!(assign23020_e35509 > 37.0)) && (!(assign23020_e35519 < assign23020_e35521))) {
                let assign23020_e35528: f64 = (locals.var_sp_vfb * locals.var_nvt);
                let assign23020_e35530: f64 = (assign23020_e35528 - locals.var_phic_s);
                let assign23020_e35532: f64 = (assign23020_e35530 / locals.var_nvt);
                let assign23020_e35533: f64 = (assign23020_e35532).exp();
                let assign23020_e35534: f64 = (1.0 + assign23020_e35533);
                let assign23020_e35535: f64 = (assign23020_e35534).ln();
                (assign23020_e35535, ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn3 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn4 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn5 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn6 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn7 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn8 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn9 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn10 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534), ((assign23020_e35533 * ((((((locals.var_sp_vfb_dn11 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23020_e35530 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))) / assign23020_e35534),)
            } else {
                let assign23020_e35538: f64 = (locals.var_sp_vfb * locals.var_nvt);
                let assign23020_e35540: f64 = (assign23020_e35538 - locals.var_phic_s);
                let assign23020_e35542: f64 = (assign23020_e35540 / locals.var_nvt);
                let assign23020_e35548: f64 = (locals.var_sp_vfb * locals.var_nvt);
                let assign23020_e35550: f64 = (assign23020_e35548 - locals.var_phic_s);
                let assign23020_e35552: f64 = (assign23020_e35550 / locals.var_nvt);
                let assign23020_e35554: f64 = (-37.0);
                let (assign23020_e35583, assign23020_e35583_d_n3, assign23020_e35583_d_n4, assign23020_e35583_d_n5, assign23020_e35583_d_n6, assign23020_e35583_d_n7, assign23020_e35583_d_n8, assign23020_e35583_d_n9, assign23020_e35583_d_n10, assign23020_e35583_d_n11,) = {
                    if ((!(assign23020_e35542 > 37.0)) && (assign23020_e35552 < assign23020_e35554)) {
                        let assign23020_e35559: f64 = (locals.var_sp_vfb * locals.var_nvt);
                        let assign23020_e35561: f64 = (assign23020_e35559 - locals.var_phic_s);
                        let assign23020_e35563: f64 = (assign23020_e35561 / locals.var_nvt);
                        let assign23020_e35564: f64 = (assign23020_e35563).exp();
                        (assign23020_e35564, (assign23020_e35564 * ((((((locals.var_sp_vfb_dn3 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn4 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn5 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn6 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn7 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn8 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn9 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn10 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))), (assign23020_e35564 * ((((((locals.var_sp_vfb_dn11 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23020_e35561 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))),)
                    } else {
                        let assign23020_e35567: f64 = (locals.var_sp_vfb * locals.var_nvt);
                        let assign23020_e35569: f64 = (assign23020_e35567 - locals.var_phic_s);
                        let assign23020_e35571: f64 = (assign23020_e35569 / locals.var_nvt);
                        let (assign23020_e35582, assign23020_e35582_d_n3, assign23020_e35582_d_n4, assign23020_e35582_d_n5, assign23020_e35582_d_n6, assign23020_e35582_d_n7, assign23020_e35582_d_n8, assign23020_e35582_d_n9, assign23020_e35582_d_n10, assign23020_e35582_d_n11,) = {
                            if (assign23020_e35571 > 37.0) {
                                let assign23020_e35576: f64 = (locals.var_sp_vfb * locals.var_nvt);
                                let assign23020_e35578: f64 = (assign23020_e35576 - locals.var_phic_s);
                                let assign23020_e35580: f64 = (assign23020_e35578 / locals.var_nvt);
                                (assign23020_e35580, ((((((locals.var_sp_vfb_dn3 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn4 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn5 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn6 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn7 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn8 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn9 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn10 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt)), ((((((locals.var_sp_vfb_dn11 * locals.var_nvt) + (locals.var_sp_vfb * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23020_e35578 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt)),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign23020_e35582, assign23020_e35582_d_n3, assign23020_e35582_d_n4, assign23020_e35582_d_n5, assign23020_e35582_d_n6, assign23020_e35582_d_n7, assign23020_e35582_d_n8, assign23020_e35582_d_n9, assign23020_e35582_d_n10, assign23020_e35582_d_n11,)
                    }
                };
                (assign23020_e35583, assign23020_e35583_d_n3, assign23020_e35583_d_n4, assign23020_e35583_d_n5, assign23020_e35583_d_n6, assign23020_e35583_d_n7, assign23020_e35583_d_n8, assign23020_e35583_d_n9, assign23020_e35583_d_n10, assign23020_e35583_d_n11,)
            }
        };
        let assign23020_e35585: f64 = (assign23020_e35502 - assign23020_e35584);
        (assign23020_e35585, (assign23020_e35502_d_n3 - assign23020_e35584_d_n3), (assign23020_e35502_d_n4 - assign23020_e35584_d_n4), (assign23020_e35502_d_n5 - assign23020_e35584_d_n5), (assign23020_e35502_d_n6 - assign23020_e35584_d_n6), (assign23020_e35502_d_n7 - assign23020_e35584_d_n7), (assign23020_e35502_d_n8 - assign23020_e35584_d_n8), (assign23020_e35502_d_n9 - assign23020_e35584_d_n9), (assign23020_e35502_d_n10 - assign23020_e35584_d_n10), (assign23020_e35502_d_n11 - assign23020_e35584_d_n11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign23020_e35587;
        locals.var_t2_dn3 = assign23020_e35587_d_n3;
        locals.var_t2_dn4 = assign23020_e35587_d_n4;
        locals.var_t2_dn5 = assign23020_e35587_d_n5;
        locals.var_t2_dn6 = assign23020_e35587_d_n6;
        locals.var_t2_dn7 = assign23020_e35587_d_n7;
        locals.var_t2_dn8 = assign23020_e35587_d_n8;
        locals.var_t2_dn9 = assign23020_e35587_d_n9;
        locals.var_t2_dn10 = assign23020_e35587_d_n10;
        locals.var_t2_dn11 = assign23020_e35587_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign23030_e35605, assign23030_e35605_d_n3, assign23030_e35605_d_n4, assign23030_e35605_d_n5, assign23030_e35605_d_n6, assign23030_e35605_d_n7, assign23030_e35605_d_n8, assign23030_e35605_d_n9, assign23030_e35605_d_n10, assign23030_e35605_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23030_e35596: f64 = (locals.var_phisb0 / locals.var_nvt);
        let assign23030_e35600: f64 = (1.0 + locals.var_rc);
        let assign23030_e35601: f64 = (locals.var_t2 / assign23030_e35600);
        let assign23030_e35602: f64 = (assign23030_e35596 + assign23030_e35601);
        let assign23030_e35603: f64 = (-assign23030_e35602);
        (assign23030_e35603, (-((((locals.var_phisb0_dn3 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn3 / assign23030_e35600))), (-((((locals.var_phisb0_dn4 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn4 / assign23030_e35600))), (-((((locals.var_phisb0_dn5 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn5 / assign23030_e35600))), (-((((locals.var_phisb0_dn6 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn6 / assign23030_e35600))), (-((((locals.var_phisb0_dn7 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn7 / assign23030_e35600))), (-((((locals.var_phisb0_dn8 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn8 / assign23030_e35600))), (-((((locals.var_phisb0_dn9 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn9 / assign23030_e35600))), (-((((locals.var_phisb0_dn10 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn10 / assign23030_e35600))), (-((((locals.var_phisb0_dn11 * locals.var_nvt) - (locals.var_phisb0 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt)) + (locals.var_t2_dn11 / assign23030_e35600))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign23030_e35605;
        locals.var_t3_dn3 = assign23030_e35605_d_n3;
        locals.var_t3_dn4 = assign23030_e35605_d_n4;
        locals.var_t3_dn5 = assign23030_e35605_d_n5;
        locals.var_t3_dn6 = assign23030_e35605_d_n6;
        locals.var_t3_dn7 = assign23030_e35605_d_n7;
        locals.var_t3_dn8 = assign23030_e35605_d_n8;
        locals.var_t3_dn9 = assign23030_e35605_d_n9;
        locals.var_t3_dn10 = assign23030_e35605_d_n10;
        locals.var_t3_dn11 = assign23030_e35605_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign23040_e35615, assign23040_e35615_d_n3, assign23040_e35615_d_n4, assign23040_e35615_d_n5, assign23040_e35615_d_n6, assign23040_e35615_d_n7, assign23040_e35615_d_n8, assign23040_e35615_d_n9, assign23040_e35615_d_n10, assign23040_e35615_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23040_e35613: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23040_e35613, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign23040_e35615;
        locals.var_t4_dn3 = assign23040_e35615_d_n3;
        locals.var_t4_dn4 = assign23040_e35615_d_n4;
        locals.var_t4_dn5 = assign23040_e35615_d_n5;
        locals.var_t4_dn6 = assign23040_e35615_d_n6;
        locals.var_t4_dn7 = assign23040_e35615_d_n7;
        locals.var_t4_dn8 = assign23040_e35615_d_n8;
        locals.var_t4_dn9 = assign23040_e35615_d_n9;
        locals.var_t4_dn10 = assign23040_e35615_d_n10;
        locals.var_t4_dn11 = assign23040_e35615_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign23050_e35633, assign23050_e35633_d_n3, assign23050_e35633_d_n4, assign23050_e35633_d_n5, assign23050_e35633_d_n6, assign23050_e35633_d_n7, assign23050_e35633_d_n8, assign23050_e35633_d_n9, assign23050_e35633_d_n10, assign23050_e35633_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23050_e35625: f64 = (locals.var_xs * locals.var_nvt);
        let assign23050_e35627: f64 = (assign23050_e35625 - locals.var_phic_s);
        let assign23050_e35629: f64 = (assign23050_e35627 / locals.var_nvt);
        let assign23050_e35630: f64 = (locals.var_t3 + assign23050_e35629);
        let assign23050_e35631: f64 = { let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23050_e35631, ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn3 + ((((((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn4 + ((((((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn5 + ((((((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn6 + ((((((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn7 + ((((((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn8 + ((((((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn9 + ((((((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn10 + ((((((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt)))), ({ let limited_exp_arg = assign23050_e35630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t3_dn11 + ((((((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11) * locals.var_nvt) - (assign23050_e35627 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign23050_e35633;
        locals.var_t5_dn3 = assign23050_e35633_d_n3;
        locals.var_t5_dn4 = assign23050_e35633_d_n4;
        locals.var_t5_dn5 = assign23050_e35633_d_n5;
        locals.var_t5_dn6 = assign23050_e35633_d_n6;
        locals.var_t5_dn7 = assign23050_e35633_d_n7;
        locals.var_t5_dn8 = assign23050_e35633_d_n8;
        locals.var_t5_dn9 = assign23050_e35633_d_n9;
        locals.var_t5_dn10 = assign23050_e35633_d_n10;
        locals.var_t5_dn11 = assign23050_e35633_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign23060_e35644, assign23060_e35644_d_n3, assign23060_e35644_d_n4, assign23060_e35644_d_n5, assign23060_e35644_d_n6, assign23060_e35644_d_n7, assign23060_e35644_d_n8, assign23060_e35644_d_n9, assign23060_e35644_d_n10, assign23060_e35644_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23060_e35641: f64 = (-locals.var_xs);
        let assign23060_e35642: f64 = { let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23060_e35642, ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn3)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn4)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn5)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn6)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn7)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn8)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn9)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn10)), ({ let limited_exp_arg = assign23060_e35641; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_xs_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign23060_e35644;
        locals.var_t6_dn3 = assign23060_e35644_d_n3;
        locals.var_t6_dn4 = assign23060_e35644_d_n4;
        locals.var_t6_dn5 = assign23060_e35644_d_n5;
        locals.var_t6_dn6 = assign23060_e35644_d_n6;
        locals.var_t6_dn7 = assign23060_e35644_d_n7;
        locals.var_t6_dn8 = assign23060_e35644_d_n8;
        locals.var_t6_dn9 = assign23060_e35644_d_n9;
        locals.var_t6_dn10 = assign23060_e35644_d_n10;
        locals.var_t6_dn11 = assign23060_e35644_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign23070_e35655, assign23070_e35655_d_n3, assign23070_e35655_d_n4, assign23070_e35655_d_n5, assign23070_e35655_d_n6, assign23070_e35655_d_n7, assign23070_e35655_d_n8, assign23070_e35655_d_n9, assign23070_e35655_d_n10, assign23070_e35655_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23070_e35653: f64 = (locals.var_xs * locals.var_xs);
        (assign23070_e35653, ((locals.var_xs_dn3 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn3)), ((locals.var_xs_dn4 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn4)), ((locals.var_xs_dn5 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn5)), ((locals.var_xs_dn6 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn6)), ((locals.var_xs_dn7 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn7)), ((locals.var_xs_dn8 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn8)), ((locals.var_xs_dn9 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn9)), ((locals.var_xs_dn10 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn10)), ((locals.var_xs_dn11 * locals.var_xs) + (locals.var_xs * locals.var_xs_dn11)),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign23070_e35655;
        locals.var_t7_dn3 = assign23070_e35655_d_n3;
        locals.var_t7_dn4 = assign23070_e35655_d_n4;
        locals.var_t7_dn5 = assign23070_e35655_d_n5;
        locals.var_t7_dn6 = assign23070_e35655_d_n6;
        locals.var_t7_dn7 = assign23070_e35655_d_n7;
        locals.var_t7_dn8 = assign23070_e35655_d_n8;
        locals.var_t7_dn9 = assign23070_e35655_d_n9;
        locals.var_t7_dn10 = assign23070_e35655_d_n10;
        locals.var_t7_dn11 = assign23070_e35655_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign23080_e35668, assign23080_e35668_d_n3, assign23080_e35668_d_n4, assign23080_e35668_d_n5, assign23080_e35668_d_n6, assign23080_e35668_d_n7, assign23080_e35668_d_n8, assign23080_e35668_d_n9, assign23080_e35668_d_n10, assign23080_e35668_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23080_e35665: f64 = (locals.var_t7 + 2.0);
        let assign23080_e35666: f64 = (1.0 / assign23080_e35665);
        (assign23080_e35666, (-(locals.var_t7_dn3 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn4 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn5 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn6 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn7 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn8 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn9 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn10 / (assign23080_e35665 * assign23080_e35665))), (-(locals.var_t7_dn11 / (assign23080_e35665 * assign23080_e35665))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11,)
    }
};
        locals.var_t8 = assign23080_e35668;
        locals.var_t8_dn3 = assign23080_e35668_d_n3;
        locals.var_t8_dn4 = assign23080_e35668_d_n4;
        locals.var_t8_dn5 = assign23080_e35668_d_n5;
        locals.var_t8_dn6 = assign23080_e35668_d_n6;
        locals.var_t8_dn7 = assign23080_e35668_d_n7;
        locals.var_t8_dn8 = assign23080_e35668_d_n8;
        locals.var_t8_dn9 = assign23080_e35668_d_n9;
        locals.var_t8_dn10 = assign23080_e35668_d_n10;
        locals.var_t8_dn11 = assign23080_e35668_d_n11;
        locals.var_t8_rv = 0.0;

        let (assign23090_e35680, assign23090_e35680_d_n3, assign23090_e35680_d_n4, assign23090_e35680_d_n5, assign23090_e35680_d_n6, assign23090_e35680_d_n7, assign23090_e35680_d_n8, assign23090_e35680_d_n9, assign23090_e35680_d_n10, assign23090_e35680_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23090_e35677: f64 = (locals.var_xs - locals.var_phidf);
        let assign23090_e35678: f64 = { let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23090_e35678, ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn3 - locals.var_phidf_dn3)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn4 - locals.var_phidf_dn4)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn5 - locals.var_phidf_dn5)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn6 - locals.var_phidf_dn6)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn7 - locals.var_phidf_dn7)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn8 - locals.var_phidf_dn8)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn9 - locals.var_phidf_dn9)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn10 - locals.var_phidf_dn10)), ({ let limited_exp_arg = assign23090_e35677; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_xs_dn11 - locals.var_phidf_dn11)),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign23090_e35680;
        locals.var_t10_dn3 = assign23090_e35680_d_n3;
        locals.var_t10_dn4 = assign23090_e35680_d_n4;
        locals.var_t10_dn5 = assign23090_e35680_d_n5;
        locals.var_t10_dn6 = assign23090_e35680_d_n6;
        locals.var_t10_dn7 = assign23090_e35680_d_n7;
        locals.var_t10_dn8 = assign23090_e35680_d_n8;
        locals.var_t10_dn9 = assign23090_e35680_d_n9;
        locals.var_t10_dn10 = assign23090_e35680_d_n10;
        locals.var_t10_dn11 = assign23090_e35680_d_n11;
        locals.var_t10_rv = 0.0;

        let (assign23100_e35698, assign23100_e35698_d_n3, assign23100_e35698_d_n4, assign23100_e35698_d_n5, assign23100_e35698_d_n6, assign23100_e35698_d_n7, assign23100_e35698_d_n8, assign23100_e35698_d_n9, assign23100_e35698_d_n10, assign23100_e35698_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23100_e35690: f64 = (locals.var_xs * locals.var_nvt);
        let assign23100_e35692: f64 = (assign23100_e35690 - locals.var_phic_s);
        let assign23100_e35693: f64 = (2.0 * assign23100_e35692);
        let assign23100_e35695: f64 = (assign23100_e35693 / locals.var_nvt);
        let assign23100_e35696: f64 = { let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23100_e35696, ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt))), ({ let limited_exp_arg = assign23100_e35695; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((2.0 * (((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11)) * locals.var_nvt) - (assign23100_e35693 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign23100_e35698;
        locals.var_t11_dn3 = assign23100_e35698_d_n3;
        locals.var_t11_dn4 = assign23100_e35698_d_n4;
        locals.var_t11_dn5 = assign23100_e35698_d_n5;
        locals.var_t11_dn6 = assign23100_e35698_d_n6;
        locals.var_t11_dn7 = assign23100_e35698_d_n7;
        locals.var_t11_dn8 = assign23100_e35698_d_n8;
        locals.var_t11_dn9 = assign23100_e35698_d_n9;
        locals.var_t11_dn10 = assign23100_e35698_d_n10;
        locals.var_t11_dn11 = assign23100_e35698_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign23110_e35718, assign23110_e35718_d_n3, assign23110_e35718_d_n4, assign23110_e35718_d_n5, assign23110_e35718_d_n6, assign23110_e35718_d_n7, assign23110_e35718_d_n8, assign23110_e35718_d_n9, assign23110_e35718_d_n10, assign23110_e35718_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23110_e35708: f64 = (locals.var_xs * locals.var_nvt);
        let assign23110_e35710: f64 = (assign23110_e35708 - locals.var_phic_s);
        let assign23110_e35711: f64 = (2.0 * assign23110_e35710);
        let assign23110_e35713: f64 = (assign23110_e35711 / locals.var_nvt);
        let assign23110_e35715: f64 = (assign23110_e35713 + locals.var_t3);
        let assign23110_e35716: f64 = { let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23110_e35716, ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn3 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn3)) - locals.var_phic_s_dn3)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn3)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn3)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn4 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn4)) - locals.var_phic_s_dn4)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn4)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn4)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn5 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn5)) - locals.var_phic_s_dn5)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn5)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn5)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn6 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn6)) - locals.var_phic_s_dn6)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn6)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn6)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn7 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn7)) - locals.var_phic_s_dn7)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn7)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn7)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn8 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn8)) - locals.var_phic_s_dn8)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn8)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn8)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn9 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn9)) - locals.var_phic_s_dn9)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn9)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn9)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn10 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn10)) - locals.var_phic_s_dn10)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn10)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn10)), ({ let limited_exp_arg = assign23110_e35715; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((((2.0 * (((locals.var_xs_dn11 * locals.var_nvt) + (locals.var_xs * locals.var_nvt_dn11)) - locals.var_phic_s_dn11)) * locals.var_nvt) - (assign23110_e35711 * locals.var_nvt_dn11)) / (locals.var_nvt * locals.var_nvt)) + locals.var_t3_dn11)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign23110_e35718;
        locals.var_t12_dn3 = assign23110_e35718_d_n3;
        locals.var_t12_dn4 = assign23110_e35718_d_n4;
        locals.var_t12_dn5 = assign23110_e35718_d_n5;
        locals.var_t12_dn6 = assign23110_e35718_d_n6;
        locals.var_t12_dn7 = assign23110_e35718_d_n7;
        locals.var_t12_dn8 = assign23110_e35718_d_n8;
        locals.var_t12_dn9 = assign23110_e35718_d_n9;
        locals.var_t12_dn10 = assign23110_e35718_d_n10;
        locals.var_t12_dn11 = assign23110_e35718_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign23120_e35814, assign23120_e35814_d_n3, assign23120_e35814_d_n4, assign23120_e35814_d_n5, assign23120_e35814_d_n6, assign23120_e35814_d_n7, assign23120_e35814_d_n8, assign23120_e35814_d_n9, assign23120_e35814_d_n10, assign23120_e35814_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23120_e35727: f64 = (2.0 * locals.var_t0);
        let assign23120_e35730: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign23120_e35731: f64 = (assign23120_e35727 * assign23120_e35730);
        let assign23120_e35733: f64 = (assign23120_e35731 * locals.var_rt);
        let assign23120_e35735: f64 = (assign23120_e35733 * locals.var_rt);
        let assign23120_e35738: f64 = (locals.var_rc + 1.0);
        let assign23120_e35741: f64 = (locals.var_t0 + 1.0);
        let assign23120_e35742: f64 = (assign23120_e35738 * assign23120_e35741);
        let assign23120_e35743: f64 = (assign23120_e35735 / assign23120_e35742);
        let assign23120_e35746: f64 = (2.0 * locals.var_vgfb1);
        let assign23120_e35747: f64 = (assign23120_e35743 - assign23120_e35746);
        let assign23120_e35750: f64 = (2.0 * locals.var_xs);
        let assign23120_e35751: f64 = (assign23120_e35747 + assign23120_e35750);
        let assign23120_e35755: f64 = (locals.var_xs - locals.var_udse);
        let assign23120_e35757: f64 = (assign23120_e35755 - locals.var_phidf);
        let assign23120_e35758: f64 = { let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23120_e35760: f64 = (-locals.var_udse);
        let assign23120_e35762: f64 = (assign23120_e35760 - locals.var_phidf);
        let assign23120_e35763: f64 = { let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23120_e35765: f64 = (-2.0);
        let assign23120_e35767: f64 = (assign23120_e35765 * locals.var_xs);
        let assign23120_e35769: f64 = (assign23120_e35767 * locals.var_t8);
        let assign23120_e35772: f64 = (2.0 * locals.var_xs);
        let assign23120_e35774: f64 = (assign23120_e35772 * locals.var_xs);
        let assign23120_e35776: f64 = (assign23120_e35774 * locals.var_xs);
        let assign23120_e35778: f64 = (assign23120_e35776 * locals.var_t8);
        let assign23120_e35780: f64 = (assign23120_e35778 * locals.var_t8);
        let assign23120_e35781: f64 = (assign23120_e35769 + assign23120_e35780);
        let assign23120_e35783: f64 = (assign23120_e35781 - 1.0);
        let assign23120_e35784: f64 = (assign23120_e35763 * assign23120_e35783);
        let assign23120_e35785: f64 = (assign23120_e35758 + assign23120_e35784);
        let assign23120_e35787: f64 = (assign23120_e35785 - locals.var_t6);
        let assign23120_e35791: f64 = (locals.var_rc + 1.0);
        let assign23120_e35794: f64 = (locals.var_t0 + 1.0);
        let assign23120_e35795: f64 = (assign23120_e35791 * assign23120_e35794);
        let assign23120_e35796: f64 = (locals.var_t0 / assign23120_e35795);
        let assign23120_e35797: f64 = (assign23120_e35787 - assign23120_e35796);
        let assign23120_e35801: f64 = (locals.var_rc + 1.0);
        let assign23120_e35804: f64 = (locals.var_t0 + 1.0);
        let assign23120_e35805: f64 = (assign23120_e35801 * assign23120_e35804);
        let assign23120_e35806: f64 = (locals.var_t5 / assign23120_e35805);
        let assign23120_e35807: f64 = (assign23120_e35797 + assign23120_e35806);
        let assign23120_e35809: f64 = (assign23120_e35807 + 1.0);
        let assign23120_e35810: f64 = (locals.var_gam2 * assign23120_e35809);
        let assign23120_e35811: f64 = (assign23120_e35751 - assign23120_e35810);
        let assign23120_e35812: f64 = (-assign23120_e35811);
        (assign23120_e35812, (-(((((((((((2.0 * locals.var_t0_dn3) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn3))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn3)) + (2.0 * locals.var_xs_dn3)) - ((locals.var_gam2_dn3 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn3 - locals.var_udse_dn3) - locals.var_phidf_dn3)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn3) - locals.var_phidf_dn3)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn3) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn3)) + (((((((((2.0 * locals.var_xs_dn3) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn3)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn3)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn3)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn3)))))) - locals.var_t6_dn3) - (((locals.var_t0_dn3 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn3))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn3 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn3))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn4) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn4))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn4)) + (2.0 * locals.var_xs_dn4)) - ((locals.var_gam2_dn4 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn4 - locals.var_udse_dn4) - locals.var_phidf_dn4)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn4) - locals.var_phidf_dn4)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn4) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn4)) + (((((((((2.0 * locals.var_xs_dn4) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn4)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn4)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn4)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn4)))))) - locals.var_t6_dn4) - (((locals.var_t0_dn4 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn4))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn4 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn4))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn5) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn5))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn5)) + (2.0 * locals.var_xs_dn5)) - ((locals.var_gam2_dn5 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn5 - locals.var_udse_dn5) - locals.var_phidf_dn5)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn5) - locals.var_phidf_dn5)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn5) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn5)) + (((((((((2.0 * locals.var_xs_dn5) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn5)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn5)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn5)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn5)))))) - locals.var_t6_dn5) - (((locals.var_t0_dn5 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn5))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn5 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn5))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn6) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn6))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn6)) + (2.0 * locals.var_xs_dn6)) - ((locals.var_gam2_dn6 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn6 - locals.var_udse_dn6) - locals.var_phidf_dn6)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn6) - locals.var_phidf_dn6)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn6) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn6)) + (((((((((2.0 * locals.var_xs_dn6) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn6)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn6)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn6)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn6)))))) - locals.var_t6_dn6) - (((locals.var_t0_dn6 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn6))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn6 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn6))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn7) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn7))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn7)) + (2.0 * locals.var_xs_dn7)) - ((locals.var_gam2_dn7 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn7 - locals.var_udse_dn7) - locals.var_phidf_dn7)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn7) - locals.var_phidf_dn7)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn7) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn7)) + (((((((((2.0 * locals.var_xs_dn7) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn7)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn7)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn7)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn7)))))) - locals.var_t6_dn7) - (((locals.var_t0_dn7 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn7))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn7 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn7))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn8) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn8))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn8)) + (2.0 * locals.var_xs_dn8)) - ((locals.var_gam2_dn8 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn8 - locals.var_udse_dn8) - locals.var_phidf_dn8)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn8) - locals.var_phidf_dn8)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn8) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn8)) + (((((((((2.0 * locals.var_xs_dn8) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn8)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn8)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn8)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn8)))))) - locals.var_t6_dn8) - (((locals.var_t0_dn8 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn8))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn8 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn8))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn9) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn9))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn9)) + (2.0 * locals.var_xs_dn9)) - ((locals.var_gam2_dn9 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn9 - locals.var_udse_dn9) - locals.var_phidf_dn9)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn9) - locals.var_phidf_dn9)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn9) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn9)) + (((((((((2.0 * locals.var_xs_dn9) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn9)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn9)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn9)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn9)))))) - locals.var_t6_dn9) - (((locals.var_t0_dn9 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn9))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn9 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn9))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn10) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn10))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn10)) + (2.0 * locals.var_xs_dn10)) - ((locals.var_gam2_dn10 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn10 - locals.var_udse_dn10) - locals.var_phidf_dn10)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn10) - locals.var_phidf_dn10)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn10) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn10)) + (((((((((2.0 * locals.var_xs_dn10) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn10)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn10)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn10)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn10)))))) - locals.var_t6_dn10) - (((locals.var_t0_dn10 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn10))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn10 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn10))) / (assign23120_e35805 * assign23120_e35805))))))), (-(((((((((((2.0 * locals.var_t0_dn11) * assign23120_e35730) + (assign23120_e35727 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11))) * locals.var_rt) * locals.var_rt) * assign23120_e35742) - (assign23120_e35735 * (assign23120_e35738 * locals.var_t0_dn11))) / (assign23120_e35742 * assign23120_e35742)) - (2.0 * locals.var_vgfb1_dn11)) + (2.0 * locals.var_xs_dn11)) - ((locals.var_gam2_dn11 * assign23120_e35809) + (locals.var_gam2 * ((((({ let limited_exp_arg = assign23120_e35757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn11 - locals.var_udse_dn11) - locals.var_phidf_dn11)) + ((({ let limited_exp_arg = assign23120_e35762; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_udse_dn11) - locals.var_phidf_dn11)) * assign23120_e35783) + (assign23120_e35763 * ((((assign23120_e35765 * locals.var_xs_dn11) * locals.var_t8) + (assign23120_e35767 * locals.var_t8_dn11)) + (((((((((2.0 * locals.var_xs_dn11) * locals.var_xs) + (assign23120_e35772 * locals.var_xs_dn11)) * locals.var_xs) + (assign23120_e35774 * locals.var_xs_dn11)) * locals.var_t8) + (assign23120_e35776 * locals.var_t8_dn11)) * locals.var_t8) + (assign23120_e35778 * locals.var_t8_dn11)))))) - locals.var_t6_dn11) - (((locals.var_t0_dn11 * assign23120_e35795) - (locals.var_t0 * (assign23120_e35791 * locals.var_t0_dn11))) / (assign23120_e35795 * assign23120_e35795))) + (((locals.var_t5_dn11 * assign23120_e35805) - (locals.var_t5 * (assign23120_e35801 * locals.var_t0_dn11))) / (assign23120_e35805 * assign23120_e35805))))))),)
    } else {
        (locals.var_pc, locals.var_pc_dn3, locals.var_pc_dn4, locals.var_pc_dn5, locals.var_pc_dn6, locals.var_pc_dn7, locals.var_pc_dn8, locals.var_pc_dn9, locals.var_pc_dn10, locals.var_pc_dn11,)
    }
};
        locals.var_pc = assign23120_e35814;
        locals.var_pc_dn3 = assign23120_e35814_d_n3;
        locals.var_pc_dn4 = assign23120_e35814_d_n4;
        locals.var_pc_dn5 = assign23120_e35814_d_n5;
        locals.var_pc_dn6 = assign23120_e35814_d_n6;
        locals.var_pc_dn7 = assign23120_e35814_d_n7;
        locals.var_pc_dn8 = assign23120_e35814_d_n8;
        locals.var_pc_dn9 = assign23120_e35814_d_n9;
        locals.var_pc_dn10 = assign23120_e35814_d_n10;
        locals.var_pc_dn11 = assign23120_e35814_d_n11;
        locals.var_pc_rv = 0.0;

        let (assign23130_e35829, assign23130_e35829_d_n3, assign23130_e35829_d_n4, assign23130_e35829_d_n5, assign23130_e35829_d_n6, assign23130_e35829_d_n7, assign23130_e35829_d_n8, assign23130_e35829_d_n9, assign23130_e35829_d_n10, assign23130_e35829_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23130_e35824: f64 = (1.0 - locals.var_k_ds);
        let assign23130_e35825: f64 = (locals.var_gam2 * assign23130_e35824);
        let assign23130_e35827: f64 = (assign23130_e35825 * locals.var_ds);
        (assign23130_e35827, ((((locals.var_gam2_dn3 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn3))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn3)), ((((locals.var_gam2_dn4 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn4))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn4)), ((((locals.var_gam2_dn5 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn5))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn5)), ((((locals.var_gam2_dn6 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn6))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn6)), ((((locals.var_gam2_dn7 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn7))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn7)), ((((locals.var_gam2_dn8 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn8))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn8)), ((((locals.var_gam2_dn9 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn9))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn9)), ((((locals.var_gam2_dn10 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn10))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn10)), ((((locals.var_gam2_dn11 * assign23130_e35824) + (locals.var_gam2 * (-locals.var_k_ds_dn11))) * locals.var_ds) + (assign23130_e35825 * locals.var_ds_dn11)),)
    } else {
        (locals.var_qc, locals.var_qc_dn3, locals.var_qc_dn4, locals.var_qc_dn5, locals.var_qc_dn6, locals.var_qc_dn7, locals.var_qc_dn8, locals.var_qc_dn9, locals.var_qc_dn10, locals.var_qc_dn11,)
    }
};
        locals.var_qc = assign23130_e35829;
        locals.var_qc_dn3 = assign23130_e35829_d_n3;
        locals.var_qc_dn4 = assign23130_e35829_d_n4;
        locals.var_qc_dn5 = assign23130_e35829_d_n5;
        locals.var_qc_dn6 = assign23130_e35829_d_n6;
        locals.var_qc_dn7 = assign23130_e35829_d_n7;
        locals.var_qc_dn8 = assign23130_e35829_d_n8;
        locals.var_qc_dn9 = assign23130_e35829_d_n9;
        locals.var_qc_dn10 = assign23130_e35829_d_n10;
        locals.var_qc_dn11 = assign23130_e35829_d_n11;
        locals.var_qc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        locals: &mut StampLocals,
    ) {
        let (assign23140_e36020, assign23140_e36020_d_n3, assign23140_e36020_d_n4, assign23140_e36020_d_n5, assign23140_e36020_d_n6, assign23140_e36020_d_n7, assign23140_e36020_d_n8, assign23140_e36020_d_n9, assign23140_e36020_d_n10, assign23140_e36020_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23140_e35838: f64 = (2.0 * locals.var_rt);
        let assign23140_e35840: f64 = (assign23140_e35838 * locals.var_rt);
        let assign23140_e35842: f64 = (assign23140_e35840 * locals.var_t0);
        let assign23140_e35845: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign23140_e35846: f64 = (assign23140_e35842 * assign23140_e35845);
        let assign23140_e35849: f64 = (1.0 + locals.var_rc);
        let assign23140_e35852: f64 = (1.0 + locals.var_t0);
        let assign23140_e35853: f64 = (assign23140_e35849 * assign23140_e35852);
        let assign23140_e35854: f64 = (assign23140_e35846 / assign23140_e35853);
        let assign23140_e35857: f64 = (2.0 * locals.var_rt);
        let assign23140_e35859: f64 = (assign23140_e35857 * locals.var_rt);
        let assign23140_e35861: f64 = (assign23140_e35859 * locals.var_t11);
        let assign23140_e35864: f64 = (1.0 + locals.var_rc);
        let assign23140_e35867: f64 = (1.0 + locals.var_rc);
        let assign23140_e35868: f64 = (assign23140_e35864 * assign23140_e35867);
        let assign23140_e35872: f64 = (2.0 * locals.var_t0);
        let assign23140_e35873: f64 = (1.0 + assign23140_e35872);
        let assign23140_e35875: f64 = (assign23140_e35873 + locals.var_t11);
        let assign23140_e35876: f64 = (assign23140_e35868 * assign23140_e35875);
        let assign23140_e35877: f64 = (assign23140_e35861 / assign23140_e35876);
        let assign23140_e35878: f64 = (assign23140_e35854 - assign23140_e35877);
        let assign23140_e35883: f64 = (locals.var_xs - locals.var_phidf);
        let assign23140_e35885: f64 = (assign23140_e35883 - locals.var_udse);
        let assign23140_e35886: f64 = { let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23140_e35887: f64 = (locals.var_t6 + assign23140_e35886);
        let assign23140_e35889: f64 = (-locals.var_phidf);
        let assign23140_e35891: f64 = (assign23140_e35889 - locals.var_udse);
        let assign23140_e35892: f64 = { let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23140_e35894: f64 = (-2.0);
        let assign23140_e35896: f64 = (assign23140_e35894 * locals.var_t8);
        let assign23140_e35899: f64 = (10.0 * locals.var_xs);
        let assign23140_e35901: f64 = (assign23140_e35899 * locals.var_xs);
        let assign23140_e35903: f64 = (assign23140_e35901 * locals.var_t8);
        let assign23140_e35905: f64 = (assign23140_e35903 * locals.var_t8);
        let assign23140_e35906: f64 = (assign23140_e35896 + assign23140_e35905);
        let assign23140_e35909: f64 = (8.0 * locals.var_xs);
        let assign23140_e35911: f64 = (assign23140_e35909 * locals.var_xs);
        let assign23140_e35913: f64 = (assign23140_e35911 * locals.var_xs);
        let assign23140_e35915: f64 = (assign23140_e35913 * locals.var_xs);
        let assign23140_e35917: f64 = (assign23140_e35915 * locals.var_t8);
        let assign23140_e35919: f64 = (assign23140_e35917 * locals.var_t8);
        let assign23140_e35921: f64 = (assign23140_e35919 * locals.var_t8);
        let assign23140_e35922: f64 = (assign23140_e35906 - assign23140_e35921);
        let assign23140_e35923: f64 = (assign23140_e35892 * assign23140_e35922);
        let assign23140_e35924: f64 = (assign23140_e35887 + assign23140_e35923);
        let assign23140_e35928: f64 = (1.0 + locals.var_rc);
        let assign23140_e35931: f64 = (1.0 + locals.var_t0);
        let assign23140_e35932: f64 = (assign23140_e35928 * assign23140_e35931);
        let assign23140_e35933: f64 = (locals.var_t0 / assign23140_e35932);
        let assign23140_e35934: f64 = (assign23140_e35924 - assign23140_e35933);
        let assign23140_e35938: f64 = (1.0 + locals.var_rc);
        let assign23140_e35942: f64 = (2.0 * locals.var_t0);
        let assign23140_e35943: f64 = (1.0 + assign23140_e35942);
        let assign23140_e35945: f64 = (assign23140_e35943 + locals.var_t11);
        let assign23140_e35946: f64 = (assign23140_e35938 * assign23140_e35945);
        let assign23140_e35947: f64 = (locals.var_t11 / assign23140_e35946);
        let assign23140_e35948: f64 = (assign23140_e35934 + assign23140_e35947);
        let assign23140_e35952: f64 = (1.0 + locals.var_rc);
        let assign23140_e35955: f64 = (1.0 + locals.var_t0);
        let assign23140_e35956: f64 = (assign23140_e35952 * assign23140_e35955);
        let assign23140_e35957: f64 = (locals.var_t5 / assign23140_e35956);
        let assign23140_e35958: f64 = (assign23140_e35948 + assign23140_e35957);
        let assign23140_e35962: f64 = (1.0 + locals.var_rc);
        let assign23140_e35966: f64 = (2.0 * locals.var_t0);
        let assign23140_e35967: f64 = (1.0 + assign23140_e35966);
        let assign23140_e35969: f64 = (assign23140_e35967 + locals.var_t11);
        let assign23140_e35970: f64 = (assign23140_e35962 * assign23140_e35969);
        let assign23140_e35971: f64 = (locals.var_t12 / assign23140_e35970);
        let assign23140_e35972: f64 = (assign23140_e35958 - assign23140_e35971);
        let assign23140_e35976: f64 = (1.0 + locals.var_rc);
        let assign23140_e35979: f64 = (1.0 + locals.var_rc);
        let assign23140_e35980: f64 = (assign23140_e35976 * assign23140_e35979);
        let assign23140_e35984: f64 = (2.0 * locals.var_t0);
        let assign23140_e35985: f64 = (1.0 + assign23140_e35984);
        let assign23140_e35987: f64 = (assign23140_e35985 + locals.var_t11);
        let assign23140_e35988: f64 = (assign23140_e35980 * assign23140_e35987);
        let assign23140_e35989: f64 = (locals.var_t12 / assign23140_e35988);
        let assign23140_e35990: f64 = (assign23140_e35972 - assign23140_e35989);
        let assign23140_e35991: f64 = (locals.var_gam2 * assign23140_e35990);
        let assign23140_e35992: f64 = (assign23140_e35878 - assign23140_e35991);
        let assign23140_e35995: f64 = (2.0 * locals.var_rt);
        let assign23140_e35997: f64 = (assign23140_e35995 * locals.var_rt);
        let assign23140_e35999: f64 = (assign23140_e35997 * locals.var_t11);
        let assign23140_e36002: f64 = (locals.var_vgfbb + locals.var_t3);
        let assign23140_e36003: f64 = (assign23140_e35999 * assign23140_e36002);
        let assign23140_e36006: f64 = (1.0 + locals.var_rc);
        let assign23140_e36010: f64 = (2.0 * locals.var_t0);
        let assign23140_e36011: f64 = (1.0 + assign23140_e36010);
        let assign23140_e36013: f64 = (assign23140_e36011 + locals.var_t11);
        let assign23140_e36014: f64 = (assign23140_e36006 * assign23140_e36013);
        let assign23140_e36015: f64 = (assign23140_e36003 / assign23140_e36014);
        let assign23140_e36016: f64 = (assign23140_e35992 - assign23140_e36015);
        let assign23140_e36018: f64 = (assign23140_e36016 + 2.0);
        (assign23140_e36018, (((((((((assign23140_e35840 * locals.var_t0_dn3) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn3))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn3) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn3) + locals.var_t11_dn3)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn3 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn3 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn3 - locals.var_phidf_dn3) - locals.var_udse_dn3))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn3) - locals.var_udse_dn3)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn3) + (((((((10.0 * locals.var_xs_dn3) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn3)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn3)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn3))) - (((((((((((((8.0 * locals.var_xs_dn3) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn3)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn3)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn3)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn3)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn3)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn3)))))) - (((locals.var_t0_dn3 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn3))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn3 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn3) + locals.var_t11_dn3)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn3 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn3))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn3 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn3) + locals.var_t11_dn3)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn3 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn3) + locals.var_t11_dn3)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn3) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn3 + locals.var_t3_dn3))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn3) + locals.var_t11_dn3)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn4) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn4))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn4) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn4) + locals.var_t11_dn4)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn4 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn4 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn4 - locals.var_phidf_dn4) - locals.var_udse_dn4))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn4) - locals.var_udse_dn4)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn4) + (((((((10.0 * locals.var_xs_dn4) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn4)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn4)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn4))) - (((((((((((((8.0 * locals.var_xs_dn4) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn4)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn4)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn4)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn4)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn4)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn4)))))) - (((locals.var_t0_dn4 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn4))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn4 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn4) + locals.var_t11_dn4)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn4 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn4))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn4 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn4) + locals.var_t11_dn4)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn4 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn4) + locals.var_t11_dn4)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn4) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn4 + locals.var_t3_dn4))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn4) + locals.var_t11_dn4)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn5) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn5))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn5) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn5) + locals.var_t11_dn5)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn5 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn5 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn5 - locals.var_phidf_dn5) - locals.var_udse_dn5))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn5) - locals.var_udse_dn5)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn5) + (((((((10.0 * locals.var_xs_dn5) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn5)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn5)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn5))) - (((((((((((((8.0 * locals.var_xs_dn5) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn5)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn5)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn5)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn5)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn5)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn5)))))) - (((locals.var_t0_dn5 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn5))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn5 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn5) + locals.var_t11_dn5)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn5 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn5))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn5 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn5) + locals.var_t11_dn5)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn5 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn5) + locals.var_t11_dn5)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn5) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn5 + locals.var_t3_dn5))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn5) + locals.var_t11_dn5)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn6) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn6))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn6) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn6) + locals.var_t11_dn6)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn6 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn6 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn6 - locals.var_phidf_dn6) - locals.var_udse_dn6))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn6) - locals.var_udse_dn6)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn6) + (((((((10.0 * locals.var_xs_dn6) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn6)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn6)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn6))) - (((((((((((((8.0 * locals.var_xs_dn6) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn6)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn6)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn6)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn6)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn6)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn6)))))) - (((locals.var_t0_dn6 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn6))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn6 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn6) + locals.var_t11_dn6)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn6 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn6))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn6 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn6) + locals.var_t11_dn6)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn6 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn6) + locals.var_t11_dn6)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn6) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn6 + locals.var_t3_dn6))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn6) + locals.var_t11_dn6)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn7) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn7))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn7) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn7) + locals.var_t11_dn7)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn7 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn7 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn7 - locals.var_phidf_dn7) - locals.var_udse_dn7))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn7) - locals.var_udse_dn7)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn7) + (((((((10.0 * locals.var_xs_dn7) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn7)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn7)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn7))) - (((((((((((((8.0 * locals.var_xs_dn7) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn7)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn7)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn7)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn7)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn7)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn7)))))) - (((locals.var_t0_dn7 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn7))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn7 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn7) + locals.var_t11_dn7)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn7 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn7))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn7 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn7) + locals.var_t11_dn7)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn7 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn7) + locals.var_t11_dn7)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn7) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn7 + locals.var_t3_dn7))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn7) + locals.var_t11_dn7)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn8) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn8))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn8) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn8) + locals.var_t11_dn8)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn8 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn8 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn8 - locals.var_phidf_dn8) - locals.var_udse_dn8))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn8) - locals.var_udse_dn8)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn8) + (((((((10.0 * locals.var_xs_dn8) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn8)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn8)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn8))) - (((((((((((((8.0 * locals.var_xs_dn8) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn8)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn8)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn8)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn8)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn8)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn8)))))) - (((locals.var_t0_dn8 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn8))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn8 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn8) + locals.var_t11_dn8)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn8 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn8))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn8 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn8) + locals.var_t11_dn8)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn8 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn8) + locals.var_t11_dn8)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn8) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn8 + locals.var_t3_dn8))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn8) + locals.var_t11_dn8)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn9) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn9))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn9) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn9) + locals.var_t11_dn9)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn9 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn9 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn9 - locals.var_phidf_dn9) - locals.var_udse_dn9))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn9) - locals.var_udse_dn9)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn9) + (((((((10.0 * locals.var_xs_dn9) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn9)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn9)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn9))) - (((((((((((((8.0 * locals.var_xs_dn9) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn9)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn9)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn9)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn9)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn9)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn9)))))) - (((locals.var_t0_dn9 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn9))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn9 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn9) + locals.var_t11_dn9)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn9 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn9))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn9 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn9) + locals.var_t11_dn9)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn9 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn9) + locals.var_t11_dn9)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn9) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn9 + locals.var_t3_dn9))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn9) + locals.var_t11_dn9)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn10) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn10))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn10) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn10) + locals.var_t11_dn10)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn10 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn10 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn10 - locals.var_phidf_dn10) - locals.var_udse_dn10))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn10) - locals.var_udse_dn10)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn10) + (((((((10.0 * locals.var_xs_dn10) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn10)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn10)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn10))) - (((((((((((((8.0 * locals.var_xs_dn10) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn10)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn10)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn10)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn10)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn10)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn10)))))) - (((locals.var_t0_dn10 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn10))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn10 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn10) + locals.var_t11_dn10)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn10 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn10))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn10 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn10) + locals.var_t11_dn10)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn10 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn10) + locals.var_t11_dn10)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn10) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn10 + locals.var_t3_dn10))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn10) + locals.var_t11_dn10)))) / (assign23140_e36014 * assign23140_e36014))), (((((((((assign23140_e35840 * locals.var_t0_dn11) * assign23140_e35845) + (assign23140_e35842 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11))) * assign23140_e35853) - (assign23140_e35846 * (assign23140_e35849 * locals.var_t0_dn11))) / (assign23140_e35853 * assign23140_e35853)) - ((((assign23140_e35859 * locals.var_t11_dn11) * assign23140_e35876) - (assign23140_e35861 * (assign23140_e35868 * ((2.0 * locals.var_t0_dn11) + locals.var_t11_dn11)))) / (assign23140_e35876 * assign23140_e35876))) - ((locals.var_gam2_dn11 * assign23140_e35990) + (locals.var_gam2 * (((((((locals.var_t6_dn11 + ({ let limited_exp_arg = assign23140_e35885; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_xs_dn11 - locals.var_phidf_dn11) - locals.var_udse_dn11))) + ((({ let limited_exp_arg = assign23140_e35891; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_phidf_dn11) - locals.var_udse_dn11)) * assign23140_e35922) + (assign23140_e35892 * (((assign23140_e35894 * locals.var_t8_dn11) + (((((((10.0 * locals.var_xs_dn11) * locals.var_xs) + (assign23140_e35899 * locals.var_xs_dn11)) * locals.var_t8) + (assign23140_e35901 * locals.var_t8_dn11)) * locals.var_t8) + (assign23140_e35903 * locals.var_t8_dn11))) - (((((((((((((8.0 * locals.var_xs_dn11) * locals.var_xs) + (assign23140_e35909 * locals.var_xs_dn11)) * locals.var_xs) + (assign23140_e35911 * locals.var_xs_dn11)) * locals.var_xs) + (assign23140_e35913 * locals.var_xs_dn11)) * locals.var_t8) + (assign23140_e35915 * locals.var_t8_dn11)) * locals.var_t8) + (assign23140_e35917 * locals.var_t8_dn11)) * locals.var_t8) + (assign23140_e35919 * locals.var_t8_dn11)))))) - (((locals.var_t0_dn11 * assign23140_e35932) - (locals.var_t0 * (assign23140_e35928 * locals.var_t0_dn11))) / (assign23140_e35932 * assign23140_e35932))) + (((locals.var_t11_dn11 * assign23140_e35946) - (locals.var_t11 * (assign23140_e35938 * ((2.0 * locals.var_t0_dn11) + locals.var_t11_dn11)))) / (assign23140_e35946 * assign23140_e35946))) + (((locals.var_t5_dn11 * assign23140_e35956) - (locals.var_t5 * (assign23140_e35952 * locals.var_t0_dn11))) / (assign23140_e35956 * assign23140_e35956))) - (((locals.var_t12_dn11 * assign23140_e35970) - (locals.var_t12 * (assign23140_e35962 * ((2.0 * locals.var_t0_dn11) + locals.var_t11_dn11)))) / (assign23140_e35970 * assign23140_e35970))) - (((locals.var_t12_dn11 * assign23140_e35988) - (locals.var_t12 * (assign23140_e35980 * ((2.0 * locals.var_t0_dn11) + locals.var_t11_dn11)))) / (assign23140_e35988 * assign23140_e35988)))))) - ((((((assign23140_e35997 * locals.var_t11_dn11) * assign23140_e36002) + (assign23140_e35999 * (locals.var_vgfbb_dn11 + locals.var_t3_dn11))) * assign23140_e36014) - (assign23140_e36003 * (assign23140_e36006 * ((2.0 * locals.var_t0_dn11) + locals.var_t11_dn11)))) / (assign23140_e36014 * assign23140_e36014))),)
    } else {
        (locals.var_tempc, locals.var_tempc_dn3, locals.var_tempc_dn4, locals.var_tempc_dn5, locals.var_tempc_dn6, locals.var_tempc_dn7, locals.var_tempc_dn8, locals.var_tempc_dn9, locals.var_tempc_dn10, locals.var_tempc_dn11,)
    }
};
        locals.var_tempc = assign23140_e36020;
        locals.var_tempc_dn3 = assign23140_e36020_d_n3;
        locals.var_tempc_dn4 = assign23140_e36020_d_n4;
        locals.var_tempc_dn5 = assign23140_e36020_d_n5;
        locals.var_tempc_dn6 = assign23140_e36020_d_n6;
        locals.var_tempc_dn7 = assign23140_e36020_d_n7;
        locals.var_tempc_dn8 = assign23140_e36020_d_n8;
        locals.var_tempc_dn9 = assign23140_e36020_d_n9;
        locals.var_tempc_dn10 = assign23140_e36020_d_n10;
        locals.var_tempc_dn11 = assign23140_e36020_d_n11;
        locals.var_tempc_rv = 0.0;

        let (assign23150_e36037, assign23150_e36037_d_n3, assign23150_e36037_d_n4, assign23150_e36037_d_n5, assign23150_e36037_d_n6, assign23150_e36037_d_n7, assign23150_e36037_d_n8, assign23150_e36037_d_n9, assign23150_e36037_d_n10, assign23150_e36037_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23150_e36029: f64 = (locals.var_pc * locals.var_pc);
        let assign23150_e36033: f64 = (locals.var_tempc * locals.var_qc);
        let assign23150_e36034: f64 = (2.0 * assign23150_e36033);
        let assign23150_e36035: f64 = (assign23150_e36029 - assign23150_e36034);
        (assign23150_e36035, (((locals.var_pc_dn3 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn3)) - (2.0 * ((locals.var_tempc_dn3 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn3)))), (((locals.var_pc_dn4 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn4)) - (2.0 * ((locals.var_tempc_dn4 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn4)))), (((locals.var_pc_dn5 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn5)) - (2.0 * ((locals.var_tempc_dn5 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn5)))), (((locals.var_pc_dn6 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn6)) - (2.0 * ((locals.var_tempc_dn6 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn6)))), (((locals.var_pc_dn7 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn7)) - (2.0 * ((locals.var_tempc_dn7 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn7)))), (((locals.var_pc_dn8 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn8)) - (2.0 * ((locals.var_tempc_dn8 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn8)))), (((locals.var_pc_dn9 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn9)) - (2.0 * ((locals.var_tempc_dn9 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn9)))), (((locals.var_pc_dn10 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn10)) - (2.0 * ((locals.var_tempc_dn10 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn10)))), (((locals.var_pc_dn11 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn11)) - (2.0 * ((locals.var_tempc_dn11 * locals.var_qc) + (locals.var_tempc * locals.var_qc_dn11)))),)
    } else {
        (locals.var_tempc, locals.var_tempc_dn3, locals.var_tempc_dn4, locals.var_tempc_dn5, locals.var_tempc_dn6, locals.var_tempc_dn7, locals.var_tempc_dn8, locals.var_tempc_dn9, locals.var_tempc_dn10, locals.var_tempc_dn11,)
    }
};
        locals.var_tempc = assign23150_e36037;
        locals.var_tempc_dn3 = assign23150_e36037_d_n3;
        locals.var_tempc_dn4 = assign23150_e36037_d_n4;
        locals.var_tempc_dn5 = assign23150_e36037_d_n5;
        locals.var_tempc_dn6 = assign23150_e36037_d_n6;
        locals.var_tempc_dn7 = assign23150_e36037_d_n7;
        locals.var_tempc_dn8 = assign23150_e36037_d_n8;
        locals.var_tempc_dn9 = assign23150_e36037_d_n9;
        locals.var_tempc_dn10 = assign23150_e36037_d_n10;
        locals.var_tempc_dn11 = assign23150_e36037_d_n11;
        locals.var_tempc_rv = 0.0;

        let assign23160_e36040: f64 = if locals.var_tempc >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard522 = assign23160_e36040;
        locals.var_guard522_rv = 0.0;

        let (assign23170_e36058, assign23170_e36058_d_n3, assign23170_e36058_d_n4, assign23170_e36058_d_n5, assign23170_e36058_d_n6, assign23170_e36058_d_n7, assign23170_e36058_d_n8, assign23170_e36058_d_n9, assign23170_e36058_d_n10, assign23170_e36058_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) && (locals.var_guard522 != 0.0)) {
        let assign23170_e36053: f64 = (locals.var_tempc).sqrt();
        let assign23170_e36054: f64 = (locals.var_pc + assign23170_e36053);
        let assign23170_e36055: f64 = (locals.var_qc / assign23170_e36054);
        let assign23170_e36056: f64 = (2.0 * assign23170_e36055);
        (assign23170_e36056, (2.0 * (((locals.var_qc_dn3 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn3 + (locals.var_tempc_dn3 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn4 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn4 + (locals.var_tempc_dn4 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn5 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn5 + (locals.var_tempc_dn5 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn6 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn6 + (locals.var_tempc_dn6 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn7 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn7 + (locals.var_tempc_dn7 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn8 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn8 + (locals.var_tempc_dn8 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn9 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn9 + (locals.var_tempc_dn9 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn10 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn10 + (locals.var_tempc_dn10 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))), (2.0 * (((locals.var_qc_dn11 * assign23170_e36054) - (locals.var_qc * (locals.var_pc_dn11 + (locals.var_tempc_dn11 / (2.0 * assign23170_e36053))))) / (assign23170_e36054 * assign23170_e36054))),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn3, locals.var_x_ds_dn4, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9, locals.var_x_ds_dn10, locals.var_x_ds_dn11,)
    }
};
        locals.var_x_ds = assign23170_e36058;
        locals.var_x_ds_dn3 = assign23170_e36058_d_n3;
        locals.var_x_ds_dn4 = assign23170_e36058_d_n4;
        locals.var_x_ds_dn5 = assign23170_e36058_d_n5;
        locals.var_x_ds_dn6 = assign23170_e36058_d_n6;
        locals.var_x_ds_dn7 = assign23170_e36058_d_n7;
        locals.var_x_ds_dn8 = assign23170_e36058_d_n8;
        locals.var_x_ds_dn9 = assign23170_e36058_d_n9;
        locals.var_x_ds_dn10 = assign23170_e36058_d_n10;
        locals.var_x_ds_dn11 = assign23170_e36058_d_n11;
        locals.var_x_ds_rv = 0.0;

        let (assign23180_e36069, assign23180_e36069_d_n3, assign23180_e36069_d_n4, assign23180_e36069_d_n5, assign23180_e36069_d_n6, assign23180_e36069_d_n7, assign23180_e36069_d_n8, assign23180_e36069_d_n9, assign23180_e36069_d_n10, assign23180_e36069_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard521 != 0.0)) {
        let assign23180_e36067: f64 = (locals.var_x7_s + locals.var_x_ds);
        (assign23180_e36067, (locals.var_x7_s_dn3 + locals.var_x_ds_dn3), (locals.var_x7_s_dn4 + locals.var_x_ds_dn4), (locals.var_x7_s_dn5 + locals.var_x_ds_dn5), (locals.var_x7_s_dn6 + locals.var_x_ds_dn6), (locals.var_x7_s_dn7 + locals.var_x_ds_dn7), (locals.var_x7_s_dn8 + locals.var_x_ds_dn8), (locals.var_x7_s_dn9 + locals.var_x_ds_dn9), (locals.var_x7_s_dn10 + locals.var_x_ds_dn10), (locals.var_x7_s_dn11 + locals.var_x_ds_dn11),)
    } else {
        (locals.var_x7_d, locals.var_x7_d_dn3, locals.var_x7_d_dn4, locals.var_x7_d_dn5, locals.var_x7_d_dn6, locals.var_x7_d_dn7, locals.var_x7_d_dn8, locals.var_x7_d_dn9, locals.var_x7_d_dn10, locals.var_x7_d_dn11,)
    }
};
        locals.var_x7_d = assign23180_e36069;
        locals.var_x7_d_dn3 = assign23180_e36069_d_n3;
        locals.var_x7_d_dn4 = assign23180_e36069_d_n4;
        locals.var_x7_d_dn5 = assign23180_e36069_d_n5;
        locals.var_x7_d_dn6 = assign23180_e36069_d_n6;
        locals.var_x7_d_dn7 = assign23180_e36069_d_n7;
        locals.var_x7_d_dn8 = assign23180_e36069_d_n8;
        locals.var_x7_d_dn9 = assign23180_e36069_d_n9;
        locals.var_x7_d_dn10 = assign23180_e36069_d_n10;
        locals.var_x7_d_dn11 = assign23180_e36069_d_n11;
        locals.var_x7_d_rv = 0.0;

        let (assign23190_e36078, assign23190_e36078_d_n3, assign23190_e36078_d_n4, assign23190_e36078_d_n5, assign23190_e36078_d_n6, assign23190_e36078_d_n7, assign23190_e36078_d_n8, assign23190_e36078_d_n9, assign23190_e36078_d_n10, assign23190_e36078_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23190_e36076: f64 = (locals.var_x_ds * locals.var_nvt);
        (assign23190_e36076, ((locals.var_x_ds_dn3 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn3)), ((locals.var_x_ds_dn4 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn4)), ((locals.var_x_ds_dn5 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn5)), ((locals.var_x_ds_dn6 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn6)), ((locals.var_x_ds_dn7 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn7)), ((locals.var_x_ds_dn8 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn8)), ((locals.var_x_ds_dn9 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn9)), ((locals.var_x_ds_dn10 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn10)), ((locals.var_x_ds_dn11 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn11)),)
    } else {
        (locals.var_dps, locals.var_dps_dn3, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11,)
    }
};
        locals.var_dps = assign23190_e36078;
        locals.var_dps_dn3 = assign23190_e36078_d_n3;
        locals.var_dps_dn4 = assign23190_e36078_d_n4;
        locals.var_dps_dn5 = assign23190_e36078_d_n5;
        locals.var_dps_dn6 = assign23190_e36078_d_n6;
        locals.var_dps_dn7 = assign23190_e36078_d_n7;
        locals.var_dps_dn8 = assign23190_e36078_d_n8;
        locals.var_dps_dn9 = assign23190_e36078_d_n9;
        locals.var_dps_dn10 = assign23190_e36078_d_n10;
        locals.var_dps_dn11 = assign23190_e36078_d_n11;
        locals.var_dps_rv = 0.0;

        let (assign23200_e36093, assign23200_e36093_d_n3, assign23200_e36093_d_n4, assign23200_e36093_d_n5, assign23200_e36093_d_n6, assign23200_e36093_d_n7, assign23200_e36093_d_n8, assign23200_e36093_d_n9, assign23200_e36093_d_n10, assign23200_e36093_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23200_e36085: f64 = (locals.var_x7_d * locals.var_x7_d);
        let assign23200_e36089: f64 = (locals.var_x7_d * locals.var_x7_d);
        let assign23200_e36090: f64 = (2.0 + assign23200_e36089);
        let assign23200_e36091: f64 = (assign23200_e36085 / assign23200_e36090);
        (assign23200_e36091, (((((locals.var_x7_d_dn3 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn3)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn3 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn3)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn4 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn4)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn4 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn4)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn5 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn5)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn5 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn5)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn6 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn6)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn6 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn6)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn7 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn7)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn7 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn7)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn8 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn8)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn8 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn8)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn9 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn9)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn9 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn9)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn10 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn10)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn10 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn10)))) / (assign23200_e36090 * assign23200_e36090)), (((((locals.var_x7_d_dn11 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn11)) * assign23200_e36090) - (assign23200_e36085 * ((locals.var_x7_d_dn11 * locals.var_x7_d) + (locals.var_x7_d * locals.var_x7_d_dn11)))) / (assign23200_e36090 * assign23200_e36090)),)
    } else {
        (locals.var_xi0d, locals.var_xi0d_dn3, locals.var_xi0d_dn4, locals.var_xi0d_dn5, locals.var_xi0d_dn6, locals.var_xi0d_dn7, locals.var_xi0d_dn8, locals.var_xi0d_dn9, locals.var_xi0d_dn10, locals.var_xi0d_dn11,)
    }
};
        locals.var_xi0d = assign23200_e36093;
        locals.var_xi0d_dn3 = assign23200_e36093_d_n3;
        locals.var_xi0d_dn4 = assign23200_e36093_d_n4;
        locals.var_xi0d_dn5 = assign23200_e36093_d_n5;
        locals.var_xi0d_dn6 = assign23200_e36093_d_n6;
        locals.var_xi0d_dn7 = assign23200_e36093_d_n7;
        locals.var_xi0d_dn8 = assign23200_e36093_d_n8;
        locals.var_xi0d_dn9 = assign23200_e36093_d_n9;
        locals.var_xi0d_dn10 = assign23200_e36093_d_n10;
        locals.var_xi0d_dn11 = assign23200_e36093_d_n11;
        locals.var_xi0d_rv = 0.0;

        let (assign23210_e36102, assign23210_e36102_d_n3, assign23210_e36102_d_n4, assign23210_e36102_d_n5, assign23210_e36102_d_n6, assign23210_e36102_d_n7, assign23210_e36102_d_n8, assign23210_e36102_d_n9, assign23210_e36102_d_n10, assign23210_e36102_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23210_e36099: f64 = (-locals.var_x7_d);
        let assign23210_e36100: f64 = { let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23210_e36100, ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn3)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn4)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn5)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn6)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn7)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn8)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn9)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn10)), ({ let limited_exp_arg = assign23210_e36099; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_x7_d_dn11)),)
    } else {
        (locals.var_ed, locals.var_ed_dn3, locals.var_ed_dn4, locals.var_ed_dn5, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9, locals.var_ed_dn10, locals.var_ed_dn11,)
    }
};
        locals.var_ed = assign23210_e36102;
        locals.var_ed_dn3 = assign23210_e36102_d_n3;
        locals.var_ed_dn4 = assign23210_e36102_d_n4;
        locals.var_ed_dn5 = assign23210_e36102_d_n5;
        locals.var_ed_dn6 = assign23210_e36102_d_n6;
        locals.var_ed_dn7 = assign23210_e36102_d_n7;
        locals.var_ed_dn8 = assign23210_e36102_d_n8;
        locals.var_ed_dn9 = assign23210_e36102_d_n9;
        locals.var_ed_dn10 = assign23210_e36102_d_n10;
        locals.var_ed_dn11 = assign23210_e36102_d_n11;
        locals.var_ed_rv = 0.0;

        let (assign23220_e36122, assign23220_e36122_d_n3, assign23220_e36122_d_n4, assign23220_e36122_d_n5, assign23220_e36122_d_n6, assign23220_e36122_d_n7, assign23220_e36122_d_n8, assign23220_e36122_d_n9, assign23220_e36122_d_n10, assign23220_e36122_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23220_e36109: f64 = (locals.var_x7_d - locals.var_phidf);
        let assign23220_e36110: f64 = { let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23220_e36112: f64 = (-locals.var_phidf);
        let assign23220_e36113: f64 = { let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23220_e36116: f64 = (locals.var_x7_d + 1.0);
        let assign23220_e36118: f64 = (assign23220_e36116 + locals.var_xi0d);
        let assign23220_e36119: f64 = (assign23220_e36113 * assign23220_e36118);
        let assign23220_e36120: f64 = (assign23220_e36110 - assign23220_e36119);
        (assign23220_e36120, (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn3 - locals.var_phidf_dn3)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn3)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn3 + locals.var_xi0d_dn3)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn4 - locals.var_phidf_dn4)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn4)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn4 + locals.var_xi0d_dn4)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn5 - locals.var_phidf_dn5)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn5)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn5 + locals.var_xi0d_dn5)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn6 - locals.var_phidf_dn6)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn6)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn6 + locals.var_xi0d_dn6)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn7 - locals.var_phidf_dn7)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn7)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn7 + locals.var_xi0d_dn7)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn8 - locals.var_phidf_dn8)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn8)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn8 + locals.var_xi0d_dn8)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn9 - locals.var_phidf_dn9)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn9)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn9 + locals.var_xi0d_dn9)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn10 - locals.var_phidf_dn10)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn10)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn10 + locals.var_xi0d_dn10)))), (({ let limited_exp_arg = assign23220_e36109; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_x7_d_dn11 - locals.var_phidf_dn11)) - ((({ let limited_exp_arg = assign23220_e36112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_phidf_dn11)) * assign23220_e36118) + (assign23220_e36113 * (locals.var_x7_d_dn11 + locals.var_xi0d_dn11)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn3, locals.var_dd_dn4, locals.var_dd_dn5, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9, locals.var_dd_dn10, locals.var_dd_dn11,)
    }
};
        locals.var_dd = assign23220_e36122;
        locals.var_dd_dn3 = assign23220_e36122_d_n3;
        locals.var_dd_dn4 = assign23220_e36122_d_n4;
        locals.var_dd_dn5 = assign23220_e36122_d_n5;
        locals.var_dd_dn6 = assign23220_e36122_d_n6;
        locals.var_dd_dn7 = assign23220_e36122_d_n7;
        locals.var_dd_dn8 = assign23220_e36122_d_n8;
        locals.var_dd_dn9 = assign23220_e36122_d_n9;
        locals.var_dd_dn10 = assign23220_e36122_d_n10;
        locals.var_dd_dn11 = assign23220_e36122_d_n11;
        locals.var_dd_rv = 0.0;

        let (assign23230_e36141, assign23230_e36141_d_n3, assign23230_e36141_d_n4, assign23230_e36141_d_n5, assign23230_e36141_d_n6, assign23230_e36141_d_n7, assign23230_e36141_d_n8, assign23230_e36141_d_n9, assign23230_e36141_d_n10, assign23230_e36141_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23230_e36129: f64 = (locals.var_vgfb1 - locals.var_x7_d);
        let assign23230_e36132: f64 = (locals.var_vgfb1 - locals.var_x7_d);
        let assign23230_e36133: f64 = (assign23230_e36129 * assign23230_e36132);
        let assign23230_e36136: f64 = (1.0 / locals.var_gam2);
        let assign23230_e36137: f64 = (assign23230_e36133 * assign23230_e36136);
        let assign23230_e36139: f64 = (assign23230_e36137 - locals.var_dd);
        (assign23230_e36139, ((((((locals.var_vgfb1_dn3 - locals.var_x7_d_dn3) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn3 - locals.var_x7_d_dn3))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn3 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn3), ((((((locals.var_vgfb1_dn4 - locals.var_x7_d_dn4) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn4 - locals.var_x7_d_dn4))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn4 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn4), ((((((locals.var_vgfb1_dn5 - locals.var_x7_d_dn5) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn5 - locals.var_x7_d_dn5))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn5 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn5), ((((((locals.var_vgfb1_dn6 - locals.var_x7_d_dn6) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn6 - locals.var_x7_d_dn6))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn6 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn6), ((((((locals.var_vgfb1_dn7 - locals.var_x7_d_dn7) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn7 - locals.var_x7_d_dn7))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn7 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn7), ((((((locals.var_vgfb1_dn8 - locals.var_x7_d_dn8) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn8 - locals.var_x7_d_dn8))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn8 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn8), ((((((locals.var_vgfb1_dn9 - locals.var_x7_d_dn9) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn9 - locals.var_x7_d_dn9))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn9 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn9), ((((((locals.var_vgfb1_dn10 - locals.var_x7_d_dn10) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn10 - locals.var_x7_d_dn10))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn10 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn10), ((((((locals.var_vgfb1_dn11 - locals.var_x7_d_dn11) * assign23230_e36132) + (assign23230_e36129 * (locals.var_vgfb1_dn11 - locals.var_x7_d_dn11))) * assign23230_e36136) + (assign23230_e36133 * (-(locals.var_gam2_dn11 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dd_dn11),)
    } else {
        (locals.var_pd, locals.var_pd_dn3, locals.var_pd_dn4, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9, locals.var_pd_dn10, locals.var_pd_dn11,)
    }
};
        locals.var_pd = assign23230_e36141;
        locals.var_pd_dn3 = assign23230_e36141_d_n3;
        locals.var_pd_dn4 = assign23230_e36141_d_n4;
        locals.var_pd_dn5 = assign23230_e36141_d_n5;
        locals.var_pd_dn6 = assign23230_e36141_d_n6;
        locals.var_pd_dn7 = assign23230_e36141_d_n7;
        locals.var_pd_dn8 = assign23230_e36141_d_n8;
        locals.var_pd_dn9 = assign23230_e36141_d_n9;
        locals.var_pd_dn10 = assign23230_e36141_d_n10;
        locals.var_pd_dn11 = assign23230_e36141_d_n11;
        locals.var_pd_rv = 0.0;

        let (assign23240_e36169, assign23240_e36169_d_n3, assign23240_e36169_d_n4, assign23240_e36169_d_n5, assign23240_e36169_d_n6, assign23240_e36169_d_n7, assign23240_e36169_d_n8, assign23240_e36169_d_n9, assign23240_e36169_d_n10, assign23240_e36169_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23240_e36149: f64 = (locals.var_pd - 0.001);
        let assign23240_e36152: f64 = (locals.var_pd - 0.001);
        let assign23240_e36155: f64 = (locals.var_pd - 0.001);
        let assign23240_e36156: f64 = (assign23240_e36152 * assign23240_e36155);
        let assign23240_e36159: f64 = (4.0 * 1e-5);
        let assign23240_e36161: f64 = (assign23240_e36159 * 1e-5);
        let assign23240_e36162: f64 = (assign23240_e36156 + assign23240_e36161);
        let assign23240_e36163: f64 = (assign23240_e36162).sqrt();
        let assign23240_e36164: f64 = (assign23240_e36149 + assign23240_e36163);
        let assign23240_e36165: f64 = (0.5 * assign23240_e36164);
        let assign23240_e36167: f64 = (assign23240_e36165 + 0.001);
        (assign23240_e36167, (0.5 * (locals.var_pd_dn3 + (((locals.var_pd_dn3 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn3)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn4 + (((locals.var_pd_dn4 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn4)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn5 + (((locals.var_pd_dn5 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn5)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn6 + (((locals.var_pd_dn6 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn6)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn7 + (((locals.var_pd_dn7 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn7)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn8 + (((locals.var_pd_dn8 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn8)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn9 + (((locals.var_pd_dn9 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn9)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn10 + (((locals.var_pd_dn10 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn10)) / (2.0 * assign23240_e36163)))), (0.5 * (locals.var_pd_dn11 + (((locals.var_pd_dn11 * assign23240_e36155) + (assign23240_e36152 * locals.var_pd_dn11)) / (2.0 * assign23240_e36163)))),)
    } else {
        (locals.var_pd, locals.var_pd_dn3, locals.var_pd_dn4, locals.var_pd_dn5, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9, locals.var_pd_dn10, locals.var_pd_dn11,)
    }
};
        locals.var_pd = assign23240_e36169;
        locals.var_pd_dn3 = assign23240_e36169_d_n3;
        locals.var_pd_dn4 = assign23240_e36169_d_n4;
        locals.var_pd_dn5 = assign23240_e36169_d_n5;
        locals.var_pd_dn6 = assign23240_e36169_d_n6;
        locals.var_pd_dn7 = assign23240_e36169_d_n7;
        locals.var_pd_dn8 = assign23240_e36169_d_n8;
        locals.var_pd_dn9 = assign23240_e36169_d_n9;
        locals.var_pd_dn10 = assign23240_e36169_d_n10;
        locals.var_pd_dn11 = assign23240_e36169_d_n11;
        locals.var_pd_rv = 0.0;

        let (assign23250_e36177, assign23250_e36177_d_n3, assign23250_e36177_d_n4, assign23250_e36177_d_n5, assign23250_e36177_d_n6, assign23250_e36177_d_n7, assign23250_e36177_d_n8, assign23250_e36177_d_n9, assign23250_e36177_d_n10, assign23250_e36177_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23250_e36175: f64 = (locals.var_pd).sqrt();
        (assign23250_e36175, (locals.var_pd_dn3 / (2.0 * assign23250_e36175)), (locals.var_pd_dn4 / (2.0 * assign23250_e36175)), (locals.var_pd_dn5 / (2.0 * assign23250_e36175)), (locals.var_pd_dn6 / (2.0 * assign23250_e36175)), (locals.var_pd_dn7 / (2.0 * assign23250_e36175)), (locals.var_pd_dn8 / (2.0 * assign23250_e36175)), (locals.var_pd_dn9 / (2.0 * assign23250_e36175)), (locals.var_pd_dn10 / (2.0 * assign23250_e36175)), (locals.var_pd_dn11 / (2.0 * assign23250_e36175)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn3, locals.var_sqd_dn4, locals.var_sqd_dn5, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9, locals.var_sqd_dn10, locals.var_sqd_dn11,)
    }
};
        locals.var_sqd = assign23250_e36177;
        locals.var_sqd_dn3 = assign23250_e36177_d_n3;
        locals.var_sqd_dn4 = assign23250_e36177_d_n4;
        locals.var_sqd_dn5 = assign23250_e36177_d_n5;
        locals.var_sqd_dn6 = assign23250_e36177_d_n6;
        locals.var_sqd_dn7 = assign23250_e36177_d_n7;
        locals.var_sqd_dn8 = assign23250_e36177_d_n8;
        locals.var_sqd_dn9 = assign23250_e36177_d_n9;
        locals.var_sqd_dn10 = assign23250_e36177_d_n10;
        locals.var_sqd_dn11 = assign23250_e36177_d_n11;
        locals.var_sqd_rv = 0.0;

        let (assign23260_e36189, assign23260_e36189_d_n3, assign23260_e36189_d_n4, assign23260_e36189_d_n5, assign23260_e36189_d_n6, assign23260_e36189_d_n7, assign23260_e36189_d_n8, assign23260_e36189_d_n9, assign23260_e36189_d_n10, assign23260_e36189_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23260_e36185: f64 = (locals.var_pd + locals.var_dd);
        let assign23260_e36186: f64 = (assign23260_e36185).sqrt();
        let assign23260_e36187: f64 = (locals.var_gam * assign23260_e36186);
        (assign23260_e36187, ((locals.var_gam_dn3 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn3 + locals.var_dd_dn3) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn4 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn4 + locals.var_dd_dn4) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn5 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn5 + locals.var_dd_dn5) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn6 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn6 + locals.var_dd_dn6) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn7 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn7 + locals.var_dd_dn7) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn8 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn8 + locals.var_dd_dn8) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn9 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn9 + locals.var_dd_dn9) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn10 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn10 + locals.var_dd_dn10) / (2.0 * assign23260_e36186)))), ((locals.var_gam_dn11 * assign23260_e36186) + (locals.var_gam * ((locals.var_pd_dn11 + locals.var_dd_dn11) / (2.0 * assign23260_e36186)))),)
    } else {
        (locals.var_xgd, locals.var_xgd_dn3, locals.var_xgd_dn4, locals.var_xgd_dn5, locals.var_xgd_dn6, locals.var_xgd_dn7, locals.var_xgd_dn8, locals.var_xgd_dn9, locals.var_xgd_dn10, locals.var_xgd_dn11,)
    }
};
        locals.var_xgd = assign23260_e36189;
        locals.var_xgd_dn3 = assign23260_e36189_d_n3;
        locals.var_xgd_dn4 = assign23260_e36189_d_n4;
        locals.var_xgd_dn5 = assign23260_e36189_d_n5;
        locals.var_xgd_dn6 = assign23260_e36189_d_n6;
        locals.var_xgd_dn7 = assign23260_e36189_d_n7;
        locals.var_xgd_dn8 = assign23260_e36189_d_n8;
        locals.var_xgd_dn9 = assign23260_e36189_d_n9;
        locals.var_xgd_dn10 = assign23260_e36189_d_n10;
        locals.var_xgd_dn11 = assign23260_e36189_d_n11;
        locals.var_xgd_rv = 0.0;

        let (assign23270_e36206, assign23270_e36206_d_n3, assign23270_e36206_d_n4, assign23270_e36206_d_n5, assign23270_e36206_d_n6, assign23270_e36206_d_n7, assign23270_e36206_d_n8, assign23270_e36206_d_n9, assign23270_e36206_d_n10, assign23270_e36206_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23270_e36196: f64 = (locals.var_gam2 * locals.var_dd);
        let assign23270_e36198: f64 = (assign23270_e36196 * locals.var_nvt);
        let assign23270_e36202: f64 = (locals.var_gam * locals.var_sqd);
        let assign23270_e36203: f64 = (locals.var_xgd + assign23270_e36202);
        let assign23270_e36204: f64 = (assign23270_e36198 / assign23270_e36203);
        (assign23270_e36204, (((((((locals.var_gam2_dn3 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn3)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn3)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn3 + ((locals.var_gam_dn3 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn3))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn4 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn4)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn4)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn4 + ((locals.var_gam_dn4 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn4))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn5 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn5)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn5)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn5 + ((locals.var_gam_dn5 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn5))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn6 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn6)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn6)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn6 + ((locals.var_gam_dn6 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn6))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn7 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn7)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn7)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn7 + ((locals.var_gam_dn7 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn7))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn8 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn8)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn8)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn8 + ((locals.var_gam_dn8 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn8))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn9 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn9)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn9)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn9 + ((locals.var_gam_dn9 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn9))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn10 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn10)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn10)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn10 + ((locals.var_gam_dn10 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn10))))) / (assign23270_e36203 * assign23270_e36203)), (((((((locals.var_gam2_dn11 * locals.var_dd) + (locals.var_gam2 * locals.var_dd_dn11)) * locals.var_nvt) + (assign23270_e36196 * locals.var_nvt_dn11)) * assign23270_e36203) - (assign23270_e36198 * (locals.var_xgd_dn11 + ((locals.var_gam_dn11 * locals.var_sqd) + (locals.var_gam * locals.var_sqd_dn11))))) / (assign23270_e36203 * assign23270_e36203)),)
    } else {
        (locals.var_qid, locals.var_qid_dn3, locals.var_qid_dn4, locals.var_qid_dn5, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9, locals.var_qid_dn10, locals.var_qid_dn11,)
    }
};
        locals.var_qid = assign23270_e36206;
        locals.var_qid_dn3 = assign23270_e36206_d_n3;
        locals.var_qid_dn4 = assign23270_e36206_d_n4;
        locals.var_qid_dn5 = assign23270_e36206_d_n5;
        locals.var_qid_dn6 = assign23270_e36206_d_n6;
        locals.var_qid_dn7 = assign23270_e36206_d_n7;
        locals.var_qid_dn8 = assign23270_e36206_d_n8;
        locals.var_qid_dn9 = assign23270_e36206_d_n9;
        locals.var_qid_dn10 = assign23270_e36206_d_n10;
        locals.var_qid_dn11 = assign23270_e36206_d_n11;
        locals.var_qid_rv = 0.0;

        let (assign23280_e36217, assign23280_e36217_d_n3, assign23280_e36217_d_n4, assign23280_e36217_d_n5, assign23280_e36217_d_n6, assign23280_e36217_d_n7, assign23280_e36217_d_n8, assign23280_e36217_d_n9, assign23280_e36217_d_n10, assign23280_e36217_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23280_e36214: f64 = (locals.var_x7_s + locals.var_x7_d);
        let assign23280_e36215: f64 = (0.5 * assign23280_e36214);
        (assign23280_e36215, (0.5 * (locals.var_x7_s_dn3 + locals.var_x7_d_dn3)), (0.5 * (locals.var_x7_s_dn4 + locals.var_x7_d_dn4)), (0.5 * (locals.var_x7_s_dn5 + locals.var_x7_d_dn5)), (0.5 * (locals.var_x7_s_dn6 + locals.var_x7_d_dn6)), (0.5 * (locals.var_x7_s_dn7 + locals.var_x7_d_dn7)), (0.5 * (locals.var_x7_s_dn8 + locals.var_x7_d_dn8)), (0.5 * (locals.var_x7_s_dn9 + locals.var_x7_d_dn9)), (0.5 * (locals.var_x7_s_dn10 + locals.var_x7_d_dn10)), (0.5 * (locals.var_x7_s_dn11 + locals.var_x7_d_dn11)),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn3, locals.var_x_m_dn4, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9, locals.var_x_m_dn10, locals.var_x_m_dn11,)
    }
};
        locals.var_x_m = assign23280_e36217;
        locals.var_x_m_dn3 = assign23280_e36217_d_n3;
        locals.var_x_m_dn4 = assign23280_e36217_d_n4;
        locals.var_x_m_dn5 = assign23280_e36217_d_n5;
        locals.var_x_m_dn6 = assign23280_e36217_d_n6;
        locals.var_x_m_dn7 = assign23280_e36217_d_n7;
        locals.var_x_m_dn8 = assign23280_e36217_d_n8;
        locals.var_x_m_dn9 = assign23280_e36217_d_n9;
        locals.var_x_m_dn10 = assign23280_e36217_d_n10;
        locals.var_x_m_dn11 = assign23280_e36217_d_n11;
        locals.var_x_m_rv = 0.0;

        let (assign23290_e36227, assign23290_e36227_d_n3, assign23290_e36227_d_n4, assign23290_e36227_d_n5, assign23290_e36227_d_n6, assign23290_e36227_d_n7, assign23290_e36227_d_n8, assign23290_e36227_d_n9, assign23290_e36227_d_n10, assign23290_e36227_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23290_e36224: f64 = (locals.var_ed * locals.var_es1);
        let assign23290_e36225: f64 = (assign23290_e36224).abs();
        (assign23290_e36225, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn3 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn3)) } else { (-((locals.var_ed_dn3 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn3))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn4 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn4)) } else { (-((locals.var_ed_dn4 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn4))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn5 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn5)) } else { (-((locals.var_ed_dn5 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn5))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn6 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn6)) } else { (-((locals.var_ed_dn6 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn6))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn7 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn7)) } else { (-((locals.var_ed_dn7 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn7))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn8 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn8)) } else { (-((locals.var_ed_dn8 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn8))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn9 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn9)) } else { (-((locals.var_ed_dn9 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn9))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn10 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn10)) } else { (-((locals.var_ed_dn10 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn10))) }, if assign23290_e36224 >= 0.0 { ((locals.var_ed_dn11 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn11)) } else { (-((locals.var_ed_dn11 * locals.var_es1) + (locals.var_ed * locals.var_es1_dn11))) },)
    } else {
        (locals.var_tempc, locals.var_tempc_dn3, locals.var_tempc_dn4, locals.var_tempc_dn5, locals.var_tempc_dn6, locals.var_tempc_dn7, locals.var_tempc_dn8, locals.var_tempc_dn9, locals.var_tempc_dn10, locals.var_tempc_dn11,)
    }
};
        locals.var_tempc = assign23290_e36227;
        locals.var_tempc_dn3 = assign23290_e36227_d_n3;
        locals.var_tempc_dn4 = assign23290_e36227_d_n4;
        locals.var_tempc_dn5 = assign23290_e36227_d_n5;
        locals.var_tempc_dn6 = assign23290_e36227_d_n6;
        locals.var_tempc_dn7 = assign23290_e36227_d_n7;
        locals.var_tempc_dn8 = assign23290_e36227_d_n8;
        locals.var_tempc_dn9 = assign23290_e36227_d_n9;
        locals.var_tempc_dn10 = assign23290_e36227_d_n10;
        locals.var_tempc_dn11 = assign23290_e36227_d_n11;
        locals.var_tempc_rv = 0.0;

        let (assign23300_e36235, assign23300_e36235_d_n3, assign23300_e36235_d_n4, assign23300_e36235_d_n5, assign23300_e36235_d_n6, assign23300_e36235_d_n7, assign23300_e36235_d_n8, assign23300_e36235_d_n9, assign23300_e36235_d_n10, assign23300_e36235_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23300_e36233: f64 = (locals.var_tempc).sqrt();
        (assign23300_e36233, (locals.var_tempc_dn3 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn4 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn5 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn6 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn7 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn8 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn9 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn10 / (2.0 * assign23300_e36233)), (locals.var_tempc_dn11 / (2.0 * assign23300_e36233)),)
    } else {
        (locals.var_em, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11,)
    }
};
        locals.var_em = assign23300_e36235;
        locals.var_em_dn3 = assign23300_e36235_d_n3;
        locals.var_em_dn4 = assign23300_e36235_d_n4;
        locals.var_em_dn5 = assign23300_e36235_d_n5;
        locals.var_em_dn6 = assign23300_e36235_d_n6;
        locals.var_em_dn7 = assign23300_e36235_d_n7;
        locals.var_em_dn8 = assign23300_e36235_d_n8;
        locals.var_em_dn9 = assign23300_e36235_d_n9;
        locals.var_em_dn10 = assign23300_e36235_d_n10;
        locals.var_em_dn11 = assign23300_e36235_d_n11;
        locals.var_em_rv = 0.0;

        let (assign23310_e36246, assign23310_e36246_d_n3, assign23310_e36246_d_n4, assign23310_e36246_d_n5, assign23310_e36246_d_n6, assign23310_e36246_d_n7, assign23310_e36246_d_n8, assign23310_e36246_d_n9, assign23310_e36246_d_n10, assign23310_e36246_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23310_e36243: f64 = (locals.var_ds + locals.var_dd);
        let assign23310_e36244: f64 = (0.5 * assign23310_e36243);
        (assign23310_e36244, (0.5 * (locals.var_ds_dn3 + locals.var_dd_dn3)), (0.5 * (locals.var_ds_dn4 + locals.var_dd_dn4)), (0.5 * (locals.var_ds_dn5 + locals.var_dd_dn5)), (0.5 * (locals.var_ds_dn6 + locals.var_dd_dn6)), (0.5 * (locals.var_ds_dn7 + locals.var_dd_dn7)), (0.5 * (locals.var_ds_dn8 + locals.var_dd_dn8)), (0.5 * (locals.var_ds_dn9 + locals.var_dd_dn9)), (0.5 * (locals.var_ds_dn10 + locals.var_dd_dn10)), (0.5 * (locals.var_ds_dn11 + locals.var_dd_dn11)),)
    } else {
        (locals.var_d_bar, locals.var_d_bar_dn3, locals.var_d_bar_dn4, locals.var_d_bar_dn5, locals.var_d_bar_dn6, locals.var_d_bar_dn7, locals.var_d_bar_dn8, locals.var_d_bar_dn9, locals.var_d_bar_dn10, locals.var_d_bar_dn11,)
    }
};
        locals.var_d_bar = assign23310_e36246;
        locals.var_d_bar_dn3 = assign23310_e36246_d_n3;
        locals.var_d_bar_dn4 = assign23310_e36246_d_n4;
        locals.var_d_bar_dn5 = assign23310_e36246_d_n5;
        locals.var_d_bar_dn6 = assign23310_e36246_d_n6;
        locals.var_d_bar_dn7 = assign23310_e36246_d_n7;
        locals.var_d_bar_dn8 = assign23310_e36246_d_n8;
        locals.var_d_bar_dn9 = assign23310_e36246_d_n9;
        locals.var_d_bar_dn10 = assign23310_e36246_d_n10;
        locals.var_d_bar_dn11 = assign23310_e36246_d_n11;
        locals.var_d_bar_rv = 0.0;

        let (assign23320_e36265, assign23320_e36265_d_n3, assign23320_e36265_d_n4, assign23320_e36265_d_n5, assign23320_e36265_d_n6, assign23320_e36265_d_n7, assign23320_e36265_d_n8, assign23320_e36265_d_n9, assign23320_e36265_d_n10, assign23320_e36265_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23320_e36255: f64 = (locals.var_x_ds * locals.var_x_ds);
        let assign23320_e36259: f64 = (2.0 * locals.var_inv_gam2);
        let assign23320_e36260: f64 = (locals.var_em - assign23320_e36259);
        let assign23320_e36261: f64 = (assign23320_e36255 * assign23320_e36260);
        let assign23320_e36262: f64 = (0.125 * assign23320_e36261);
        let assign23320_e36263: f64 = (locals.var_d_bar + assign23320_e36262);
        (assign23320_e36263, (locals.var_d_bar_dn3 + (0.125 * ((((locals.var_x_ds_dn3 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn3)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn3 - (2.0 * locals.var_inv_gam2_dn3)))))), (locals.var_d_bar_dn4 + (0.125 * ((((locals.var_x_ds_dn4 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn4)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn4 - (2.0 * locals.var_inv_gam2_dn4)))))), (locals.var_d_bar_dn5 + (0.125 * ((((locals.var_x_ds_dn5 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn5)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn5 - (2.0 * locals.var_inv_gam2_dn5)))))), (locals.var_d_bar_dn6 + (0.125 * ((((locals.var_x_ds_dn6 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn6)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn6 - (2.0 * locals.var_inv_gam2_dn6)))))), (locals.var_d_bar_dn7 + (0.125 * ((((locals.var_x_ds_dn7 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn7)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn7 - (2.0 * locals.var_inv_gam2_dn7)))))), (locals.var_d_bar_dn8 + (0.125 * ((((locals.var_x_ds_dn8 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn8)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn8 - (2.0 * locals.var_inv_gam2_dn8)))))), (locals.var_d_bar_dn9 + (0.125 * ((((locals.var_x_ds_dn9 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn9)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn9 - (2.0 * locals.var_inv_gam2_dn9)))))), (locals.var_d_bar_dn10 + (0.125 * ((((locals.var_x_ds_dn10 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn10)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn10 - (2.0 * locals.var_inv_gam2_dn10)))))), (locals.var_d_bar_dn11 + (0.125 * ((((locals.var_x_ds_dn11 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn11)) * assign23320_e36260) + (assign23320_e36255 * (locals.var_em_dn11 - (2.0 * locals.var_inv_gam2_dn11)))))),)
    } else {
        (locals.var_dm, locals.var_dm_dn3, locals.var_dm_dn4, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9, locals.var_dm_dn10, locals.var_dm_dn11,)
    }
};
        locals.var_dm = assign23320_e36265;
        locals.var_dm_dn3 = assign23320_e36265_d_n3;
        locals.var_dm_dn4 = assign23320_e36265_d_n4;
        locals.var_dm_dn5 = assign23320_e36265_d_n5;
        locals.var_dm_dn6 = assign23320_e36265_d_n6;
        locals.var_dm_dn7 = assign23320_e36265_d_n7;
        locals.var_dm_dn8 = assign23320_e36265_d_n8;
        locals.var_dm_dn9 = assign23320_e36265_d_n9;
        locals.var_dm_dn10 = assign23320_e36265_d_n10;
        locals.var_dm_dn11 = assign23320_e36265_d_n11;
        locals.var_dm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23330_e36284, assign23330_e36284_d_n3, assign23330_e36284_d_n4, assign23330_e36284_d_n5, assign23330_e36284_d_n6, assign23330_e36284_d_n7, assign23330_e36284_d_n8, assign23330_e36284_d_n9, assign23330_e36284_d_n10, assign23330_e36284_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23330_e36272: f64 = (locals.var_vgfb1 - locals.var_x_m);
        let assign23330_e36275: f64 = (locals.var_vgfb1 - locals.var_x_m);
        let assign23330_e36276: f64 = (assign23330_e36272 * assign23330_e36275);
        let assign23330_e36279: f64 = (1.0 / locals.var_gam2);
        let assign23330_e36280: f64 = (assign23330_e36276 * assign23330_e36279);
        let assign23330_e36282: f64 = (assign23330_e36280 - locals.var_dm);
        (assign23330_e36282, ((((((locals.var_vgfb1_dn3 - locals.var_x_m_dn3) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn3 - locals.var_x_m_dn3))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn3 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn3), ((((((locals.var_vgfb1_dn4 - locals.var_x_m_dn4) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn4 - locals.var_x_m_dn4))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn4 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn4), ((((((locals.var_vgfb1_dn5 - locals.var_x_m_dn5) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn5 - locals.var_x_m_dn5))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn5 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn5), ((((((locals.var_vgfb1_dn6 - locals.var_x_m_dn6) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn6 - locals.var_x_m_dn6))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn6 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn6), ((((((locals.var_vgfb1_dn7 - locals.var_x_m_dn7) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn7 - locals.var_x_m_dn7))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn7 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn7), ((((((locals.var_vgfb1_dn8 - locals.var_x_m_dn8) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn8 - locals.var_x_m_dn8))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn8 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn8), ((((((locals.var_vgfb1_dn9 - locals.var_x_m_dn9) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn9 - locals.var_x_m_dn9))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn9 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn9), ((((((locals.var_vgfb1_dn10 - locals.var_x_m_dn10) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn10 - locals.var_x_m_dn10))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn10 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn10), ((((((locals.var_vgfb1_dn11 - locals.var_x_m_dn11) * assign23330_e36275) + (assign23330_e36272 * (locals.var_vgfb1_dn11 - locals.var_x_m_dn11))) * assign23330_e36279) + (assign23330_e36276 * (-(locals.var_gam2_dn11 / (locals.var_gam2 * locals.var_gam2))))) - locals.var_dm_dn11),)
    } else {
        (locals.var_pm, locals.var_pm_dn3, locals.var_pm_dn4, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9, locals.var_pm_dn10, locals.var_pm_dn11,)
    }
};
        locals.var_pm = assign23330_e36284;
        locals.var_pm_dn3 = assign23330_e36284_d_n3;
        locals.var_pm_dn4 = assign23330_e36284_d_n4;
        locals.var_pm_dn5 = assign23330_e36284_d_n5;
        locals.var_pm_dn6 = assign23330_e36284_d_n6;
        locals.var_pm_dn7 = assign23330_e36284_d_n7;
        locals.var_pm_dn8 = assign23330_e36284_d_n8;
        locals.var_pm_dn9 = assign23330_e36284_d_n9;
        locals.var_pm_dn10 = assign23330_e36284_d_n10;
        locals.var_pm_dn11 = assign23330_e36284_d_n11;
        locals.var_pm_rv = 0.0;

        let (assign23340_e36296, assign23340_e36296_d_n3, assign23340_e36296_d_n4, assign23340_e36296_d_n5, assign23340_e36296_d_n6, assign23340_e36296_d_n7, assign23340_e36296_d_n8, assign23340_e36296_d_n9, assign23340_e36296_d_n10, assign23340_e36296_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23340_e36292: f64 = (locals.var_dm + locals.var_pm);
        let assign23340_e36293: f64 = (assign23340_e36292).sqrt();
        let assign23340_e36294: f64 = (locals.var_gam * assign23340_e36293);
        (assign23340_e36294, ((locals.var_gam_dn3 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn3 + locals.var_pm_dn3) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn4 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn5 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn6 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn7 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn8 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn9 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn10 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn10 + locals.var_pm_dn10) / (2.0 * assign23340_e36293)))), ((locals.var_gam_dn11 * assign23340_e36293) + (locals.var_gam * ((locals.var_dm_dn11 + locals.var_pm_dn11) / (2.0 * assign23340_e36293)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn3, locals.var_xgm_dn4, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9, locals.var_xgm_dn10, locals.var_xgm_dn11,)
    }
};
        locals.var_xgm = assign23340_e36296;
        locals.var_xgm_dn3 = assign23340_e36296_d_n3;
        locals.var_xgm_dn4 = assign23340_e36296_d_n4;
        locals.var_xgm_dn5 = assign23340_e36296_d_n5;
        locals.var_xgm_dn6 = assign23340_e36296_d_n6;
        locals.var_xgm_dn7 = assign23340_e36296_d_n7;
        locals.var_xgm_dn8 = assign23340_e36296_d_n8;
        locals.var_xgm_dn9 = assign23340_e36296_d_n9;
        locals.var_xgm_dn10 = assign23340_e36296_d_n10;
        locals.var_xgm_dn11 = assign23340_e36296_d_n11;
        locals.var_xgm_rv = 0.0;

        let (assign23350_e36324, assign23350_e36324_d_n3, assign23350_e36324_d_n4, assign23350_e36324_d_n5, assign23350_e36324_d_n6, assign23350_e36324_d_n7, assign23350_e36324_d_n8, assign23350_e36324_d_n9, assign23350_e36324_d_n10, assign23350_e36324_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23350_e36304: f64 = (locals.var_pm - 0.001);
        let assign23350_e36307: f64 = (locals.var_pm - 0.001);
        let assign23350_e36310: f64 = (locals.var_pm - 0.001);
        let assign23350_e36311: f64 = (assign23350_e36307 * assign23350_e36310);
        let assign23350_e36314: f64 = (4.0 * 1e-5);
        let assign23350_e36316: f64 = (assign23350_e36314 * 1e-5);
        let assign23350_e36317: f64 = (assign23350_e36311 + assign23350_e36316);
        let assign23350_e36318: f64 = (assign23350_e36317).sqrt();
        let assign23350_e36319: f64 = (assign23350_e36304 + assign23350_e36318);
        let assign23350_e36320: f64 = (0.5 * assign23350_e36319);
        let assign23350_e36322: f64 = (assign23350_e36320 + 0.001);
        (assign23350_e36322, (0.5 * (locals.var_pm_dn3 + (((locals.var_pm_dn3 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn3)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn4 + (((locals.var_pm_dn4 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn4)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn5 + (((locals.var_pm_dn5 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn5)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn6 + (((locals.var_pm_dn6 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn6)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn7 + (((locals.var_pm_dn7 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn7)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn8 + (((locals.var_pm_dn8 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn8)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn9 + (((locals.var_pm_dn9 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn9)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn10 + (((locals.var_pm_dn10 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn10)) / (2.0 * assign23350_e36318)))), (0.5 * (locals.var_pm_dn11 + (((locals.var_pm_dn11 * assign23350_e36310) + (assign23350_e36307 * locals.var_pm_dn11)) / (2.0 * assign23350_e36318)))),)
    } else {
        (locals.var_pm, locals.var_pm_dn3, locals.var_pm_dn4, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9, locals.var_pm_dn10, locals.var_pm_dn11,)
    }
};
        locals.var_pm = assign23350_e36324;
        locals.var_pm_dn3 = assign23350_e36324_d_n3;
        locals.var_pm_dn4 = assign23350_e36324_d_n4;
        locals.var_pm_dn5 = assign23350_e36324_d_n5;
        locals.var_pm_dn6 = assign23350_e36324_d_n6;
        locals.var_pm_dn7 = assign23350_e36324_d_n7;
        locals.var_pm_dn8 = assign23350_e36324_d_n8;
        locals.var_pm_dn9 = assign23350_e36324_d_n9;
        locals.var_pm_dn10 = assign23350_e36324_d_n10;
        locals.var_pm_dn11 = assign23350_e36324_d_n11;
        locals.var_pm_rv = 0.0;

        let (assign23360_e36332, assign23360_e36332_d_n3, assign23360_e36332_d_n4, assign23360_e36332_d_n5, assign23360_e36332_d_n6, assign23360_e36332_d_n7, assign23360_e36332_d_n8, assign23360_e36332_d_n9, assign23360_e36332_d_n10, assign23360_e36332_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23360_e36330: f64 = (locals.var_pm).sqrt();
        (assign23360_e36330, (locals.var_pm_dn3 / (2.0 * assign23360_e36330)), (locals.var_pm_dn4 / (2.0 * assign23360_e36330)), (locals.var_pm_dn5 / (2.0 * assign23360_e36330)), (locals.var_pm_dn6 / (2.0 * assign23360_e36330)), (locals.var_pm_dn7 / (2.0 * assign23360_e36330)), (locals.var_pm_dn8 / (2.0 * assign23360_e36330)), (locals.var_pm_dn9 / (2.0 * assign23360_e36330)), (locals.var_pm_dn10 / (2.0 * assign23360_e36330)), (locals.var_pm_dn11 / (2.0 * assign23360_e36330)),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn3, locals.var_sqm_dn4, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, locals.var_sqm_dn9, locals.var_sqm_dn10, locals.var_sqm_dn11,)
    }
};
        locals.var_sqm = assign23360_e36332;
        locals.var_sqm_dn3 = assign23360_e36332_d_n3;
        locals.var_sqm_dn4 = assign23360_e36332_d_n4;
        locals.var_sqm_dn5 = assign23360_e36332_d_n5;
        locals.var_sqm_dn6 = assign23360_e36332_d_n6;
        locals.var_sqm_dn7 = assign23360_e36332_d_n7;
        locals.var_sqm_dn8 = assign23360_e36332_d_n8;
        locals.var_sqm_dn9 = assign23360_e36332_d_n9;
        locals.var_sqm_dn10 = assign23360_e36332_d_n10;
        locals.var_sqm_dn11 = assign23360_e36332_d_n11;
        locals.var_sqm_rv = 0.0;

        let assign23370_e36335: f64 = if p.p46 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign23370_e36335;
        locals.var_guard523_rv = 0.0;

        let (assign23380_e36356, assign23380_e36356_d_n3, assign23380_e36356_d_n4, assign23380_e36356_d_n5, assign23380_e36356_d_n6, assign23380_e36356_d_n7, assign23380_e36356_d_n8, assign23380_e36356_d_n9, assign23380_e36356_d_n10, assign23380_e36356_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23380_e36344: f64 = (2.0 * locals.var_cox);
        let assign23380_e36346: f64 = (assign23380_e36344 * locals.var_cox);
        let assign23380_e36348: f64 = (assign23380_e36346 * locals.var_nvt);
        let assign23380_e36351: f64 = (1.602176462e-19 * locals.var_epssi);
        let assign23380_e36353: f64 = (assign23380_e36351 * locals.var_ngate_i);
        let assign23380_e36354: f64 = (assign23380_e36348 / assign23380_e36353);
        (assign23380_e36354, ((assign23380_e36346 * locals.var_nvt_dn3) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn4) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn5) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn6) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn7) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn8) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn9) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn10) / assign23380_e36353), ((assign23380_e36346 * locals.var_nvt_dn11) / assign23380_e36353),)
    } else {
        (locals.var_pdef, locals.var_pdef_dn3, locals.var_pdef_dn4, locals.var_pdef_dn5, locals.var_pdef_dn6, locals.var_pdef_dn7, locals.var_pdef_dn8, locals.var_pdef_dn9, locals.var_pdef_dn10, locals.var_pdef_dn11,)
    }
};
        locals.var_pdef = assign23380_e36356;
        locals.var_pdef_dn3 = assign23380_e36356_d_n3;
        locals.var_pdef_dn4 = assign23380_e36356_d_n4;
        locals.var_pdef_dn5 = assign23380_e36356_d_n5;
        locals.var_pdef_dn6 = assign23380_e36356_d_n6;
        locals.var_pdef_dn7 = assign23380_e36356_d_n7;
        locals.var_pdef_dn8 = assign23380_e36356_d_n8;
        locals.var_pdef_dn9 = assign23380_e36356_d_n9;
        locals.var_pdef_dn10 = assign23380_e36356_d_n10;
        locals.var_pdef_dn11 = assign23380_e36356_d_n11;
        locals.var_pdef_rv = 0.0;

        let (assign23390_e36375, assign23390_e36375_d_n3, assign23390_e36375_d_n4, assign23390_e36375_d_n5, assign23390_e36375_d_n6, assign23390_e36375_d_n7, assign23390_e36375_d_n8, assign23390_e36375_d_n9, assign23390_e36375_d_n10, assign23390_e36375_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23390_e36365: f64 = (1.0 - locals.var_em);
        let assign23390_e36370: f64 = (1.0 / locals.var_gam2);
        let assign23390_e36371: f64 = (locals.var_xgm * assign23390_e36370);
        let assign23390_e36372: f64 = (2.0 * assign23390_e36371);
        let assign23390_e36373: f64 = (assign23390_e36365 + assign23390_e36372);
        (assign23390_e36373, ((-locals.var_em_dn3) + (2.0 * ((locals.var_xgm_dn3 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn3 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn4) + (2.0 * ((locals.var_xgm_dn4 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn4 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn5) + (2.0 * ((locals.var_xgm_dn5 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn5 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn6) + (2.0 * ((locals.var_xgm_dn6 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn6 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn7) + (2.0 * ((locals.var_xgm_dn7 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn7 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn8) + (2.0 * ((locals.var_xgm_dn8 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn8 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn9) + (2.0 * ((locals.var_xgm_dn9 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn9 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn10) + (2.0 * ((locals.var_xgm_dn10 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn10 / (locals.var_gam2 * locals.var_gam2))))))), ((-locals.var_em_dn11) + (2.0 * ((locals.var_xgm_dn11 * assign23390_e36370) + (locals.var_xgm * (-(locals.var_gam2_dn11 / (locals.var_gam2 * locals.var_gam2))))))),)
    } else {
        (locals.var_d0, locals.var_d0_dn3, locals.var_d0_dn4, locals.var_d0_dn5, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn8, locals.var_d0_dn9, locals.var_d0_dn10, locals.var_d0_dn11,)
    }
};
        locals.var_d0 = assign23390_e36375;
        locals.var_d0_dn3 = assign23390_e36375_d_n3;
        locals.var_d0_dn4 = assign23390_e36375_d_n4;
        locals.var_d0_dn5 = assign23390_e36375_d_n5;
        locals.var_d0_dn6 = assign23390_e36375_d_n6;
        locals.var_d0_dn7 = assign23390_e36375_d_n7;
        locals.var_d0_dn8 = assign23390_e36375_d_n8;
        locals.var_d0_dn9 = assign23390_e36375_d_n9;
        locals.var_d0_dn10 = assign23390_e36375_d_n10;
        locals.var_d0_dn11 = assign23390_e36375_d_n11;
        locals.var_d0_rv = 0.0;

        let (assign23400_e36391, assign23400_e36391_d_n3, assign23400_e36391_d_n4, assign23400_e36391_d_n5, assign23400_e36391_d_n6, assign23400_e36391_d_n7, assign23400_e36391_d_n8, assign23400_e36391_d_n9, assign23400_e36391_d_n10, assign23400_e36391_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23400_e36386: f64 = (locals.var_pdef * locals.var_xgm);
        let assign23400_e36387: f64 = (1.0 + assign23400_e36386);
        let assign23400_e36388: f64 = (assign23400_e36387).sqrt();
        let assign23400_e36389: f64 = (1.0 / assign23400_e36388);
        (assign23400_e36389, (-((((locals.var_pdef_dn3 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn3)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn4 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn4)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn5 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn5)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn6 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn6)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn7 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn7)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn8 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn8)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn9 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn9)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn10 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn10)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))), (-((((locals.var_pdef_dn11 * locals.var_xgm) + (locals.var_pdef * locals.var_xgm_dn11)) / (2.0 * assign23400_e36388)) / (assign23400_e36388 * assign23400_e36388))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn3, locals.var_eta_p_dn4, locals.var_eta_p_dn5, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, locals.var_eta_p_dn9, locals.var_eta_p_dn10, locals.var_eta_p_dn11,)
    }
};
        locals.var_eta_p = assign23400_e36391;
        locals.var_eta_p_dn3 = assign23400_e36391_d_n3;
        locals.var_eta_p_dn4 = assign23400_e36391_d_n4;
        locals.var_eta_p_dn5 = assign23400_e36391_d_n5;
        locals.var_eta_p_dn6 = assign23400_e36391_d_n6;
        locals.var_eta_p_dn7 = assign23400_e36391_d_n7;
        locals.var_eta_p_dn8 = assign23400_e36391_d_n8;
        locals.var_eta_p_dn9 = assign23400_e36391_d_n9;
        locals.var_eta_p_dn10 = assign23400_e36391_d_n10;
        locals.var_eta_p_dn11 = assign23400_e36391_d_n11;
        locals.var_eta_p_rv = 0.0;

        let (assign23410_e36404, assign23410_e36404_d_n3, assign23410_e36404_d_n4, assign23410_e36404_d_n5, assign23410_e36404_d_n6, assign23410_e36404_d_n7, assign23410_e36404_d_n8, assign23410_e36404_d_n9, assign23410_e36404_d_n10, assign23410_e36404_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23410_e36401: f64 = (locals.var_eta_p + 1.0);
        let assign23410_e36402: f64 = (locals.var_eta_p / assign23410_e36401);
        (assign23410_e36402, (((locals.var_eta_p_dn3 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn3)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn4 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn4)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn5 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn5)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn6 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn6)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn7 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn7)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn8 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn8)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn9 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn9)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn10 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn10)) / (assign23410_e36401 * assign23410_e36401)), (((locals.var_eta_p_dn11 * assign23410_e36401) - (locals.var_eta_p * locals.var_eta_p_dn11)) / (assign23410_e36401 * assign23410_e36401)),)
    } else {
        (locals.var_tempc, locals.var_tempc_dn3, locals.var_tempc_dn4, locals.var_tempc_dn5, locals.var_tempc_dn6, locals.var_tempc_dn7, locals.var_tempc_dn8, locals.var_tempc_dn9, locals.var_tempc_dn10, locals.var_tempc_dn11,)
    }
};
        locals.var_tempc = assign23410_e36404;
        locals.var_tempc_dn3 = assign23410_e36404_d_n3;
        locals.var_tempc_dn4 = assign23410_e36404_d_n4;
        locals.var_tempc_dn5 = assign23410_e36404_d_n5;
        locals.var_tempc_dn6 = assign23410_e36404_d_n6;
        locals.var_tempc_dn7 = assign23410_e36404_d_n7;
        locals.var_tempc_dn8 = assign23410_e36404_d_n8;
        locals.var_tempc_dn9 = assign23410_e36404_d_n9;
        locals.var_tempc_dn10 = assign23410_e36404_d_n10;
        locals.var_tempc_dn11 = assign23410_e36404_d_n11;
        locals.var_tempc_rv = 0.0;

        let (assign23420_e36427, assign23420_e36427_d_n3, assign23420_e36427_d_n4, assign23420_e36427_d_n5, assign23420_e36427_d_n6, assign23420_e36427_d_n7, assign23420_e36427_d_n8, assign23420_e36427_d_n9, assign23420_e36427_d_n10, assign23420_e36427_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23420_e36414: f64 = (locals.var_tempc * locals.var_tempc);
        let assign23420_e36416: f64 = (assign23420_e36414 * locals.var_xgm);
        let assign23420_e36418: f64 = (assign23420_e36416 * locals.var_xgm);
        let assign23420_e36419: f64 = (locals.var_pdef * assign23420_e36418);
        let assign23420_e36423: f64 = (locals.var_dm + locals.var_pm);
        let assign23420_e36424: f64 = (locals.var_dm / assign23420_e36423);
        let assign23420_e36425: f64 = (assign23420_e36419 * assign23420_e36424);
        (assign23420_e36425, ((((locals.var_pdef_dn3 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn3 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn3)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn3)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn3)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn3 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn3 + locals.var_pm_dn3))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn4 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn4 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn4)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn4)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn4)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn4 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn4 + locals.var_pm_dn4))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn5 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn5 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn5)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn5)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn5)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn5 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn5 + locals.var_pm_dn5))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn6 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn6 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn6)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn6)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn6)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn6 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn6 + locals.var_pm_dn6))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn7 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn7 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn7)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn7)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn7)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn7 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn7 + locals.var_pm_dn7))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn8 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn8 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn8)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn8)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn8)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn8 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn8 + locals.var_pm_dn8))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn9 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn9 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn9)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn9)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn9)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn9 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn9 + locals.var_pm_dn9))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn10 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn10 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn10)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn10)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn10)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn10 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn10 + locals.var_pm_dn10))) / (assign23420_e36423 * assign23420_e36423)))), ((((locals.var_pdef_dn11 * assign23420_e36418) + (locals.var_pdef * ((((((locals.var_tempc_dn11 * locals.var_tempc) + (locals.var_tempc * locals.var_tempc_dn11)) * locals.var_xgm) + (assign23420_e36414 * locals.var_xgm_dn11)) * locals.var_xgm) + (assign23420_e36416 * locals.var_xgm_dn11)))) * assign23420_e36424) + (assign23420_e36419 * (((locals.var_dm_dn11 * assign23420_e36423) - (locals.var_dm * (locals.var_dm_dn11 + locals.var_pm_dn11))) / (assign23420_e36423 * assign23420_e36423)))),)
    } else {
        (locals.var_x_pm, locals.var_x_pm_dn3, locals.var_x_pm_dn4, locals.var_x_pm_dn5, locals.var_x_pm_dn6, locals.var_x_pm_dn7, locals.var_x_pm_dn8, locals.var_x_pm_dn9, locals.var_x_pm_dn10, locals.var_x_pm_dn11,)
    }
};
        locals.var_x_pm = assign23420_e36427;
        locals.var_x_pm_dn3 = assign23420_e36427_d_n3;
        locals.var_x_pm_dn4 = assign23420_e36427_d_n4;
        locals.var_x_pm_dn5 = assign23420_e36427_d_n5;
        locals.var_x_pm_dn6 = assign23420_e36427_d_n6;
        locals.var_x_pm_dn7 = assign23420_e36427_d_n7;
        locals.var_x_pm_dn8 = assign23420_e36427_d_n8;
        locals.var_x_pm_dn9 = assign23420_e36427_d_n9;
        locals.var_x_pm_dn10 = assign23420_e36427_d_n10;
        locals.var_x_pm_dn11 = assign23420_e36427_d_n11;
        locals.var_x_pm_rv = 0.0;

        let (assign23430_e36448, assign23430_e36448_d_n3, assign23430_e36448_d_n4, assign23430_e36448_d_n5, assign23430_e36448_d_n6, assign23430_e36448_d_n7, assign23430_e36448_d_n8, assign23430_e36448_d_n9, assign23430_e36448_d_n10, assign23430_e36448_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23430_e36437: f64 = (locals.var_xgm - locals.var_x_pm);
        let assign23430_e36438: f64 = (2.0 * assign23430_e36437);
        let assign23430_e36442: f64 = (1.0 - locals.var_em);
        let assign23430_e36444: f64 = (assign23430_e36442 + locals.var_dm);
        let assign23430_e36445: f64 = (locals.var_gam2 * assign23430_e36444);
        let assign23430_e36446: f64 = (assign23430_e36438 + assign23430_e36445);
        (assign23430_e36446, ((2.0 * (locals.var_xgm_dn3 - locals.var_x_pm_dn3)) + ((locals.var_gam2_dn3 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn3) + locals.var_dm_dn3)))), ((2.0 * (locals.var_xgm_dn4 - locals.var_x_pm_dn4)) + ((locals.var_gam2_dn4 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn4) + locals.var_dm_dn4)))), ((2.0 * (locals.var_xgm_dn5 - locals.var_x_pm_dn5)) + ((locals.var_gam2_dn5 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn5) + locals.var_dm_dn5)))), ((2.0 * (locals.var_xgm_dn6 - locals.var_x_pm_dn6)) + ((locals.var_gam2_dn6 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn6) + locals.var_dm_dn6)))), ((2.0 * (locals.var_xgm_dn7 - locals.var_x_pm_dn7)) + ((locals.var_gam2_dn7 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn7) + locals.var_dm_dn7)))), ((2.0 * (locals.var_xgm_dn8 - locals.var_x_pm_dn8)) + ((locals.var_gam2_dn8 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn8) + locals.var_dm_dn8)))), ((2.0 * (locals.var_xgm_dn9 - locals.var_x_pm_dn9)) + ((locals.var_gam2_dn9 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn9) + locals.var_dm_dn9)))), ((2.0 * (locals.var_xgm_dn10 - locals.var_x_pm_dn10)) + ((locals.var_gam2_dn10 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn10) + locals.var_dm_dn10)))), ((2.0 * (locals.var_xgm_dn11 - locals.var_x_pm_dn11)) + ((locals.var_gam2_dn11 * assign23430_e36444) + (locals.var_gam2 * ((-locals.var_em_dn11) + locals.var_dm_dn11)))),)
    } else {
        (locals.var_p_pd, locals.var_p_pd_dn3, locals.var_p_pd_dn4, locals.var_p_pd_dn5, locals.var_p_pd_dn6, locals.var_p_pd_dn7, locals.var_p_pd_dn8, locals.var_p_pd_dn9, locals.var_p_pd_dn10, locals.var_p_pd_dn11,)
    }
};
        locals.var_p_pd = assign23430_e36448;
        locals.var_p_pd_dn3 = assign23430_e36448_d_n3;
        locals.var_p_pd_dn4 = assign23430_e36448_d_n4;
        locals.var_p_pd_dn5 = assign23430_e36448_d_n5;
        locals.var_p_pd_dn6 = assign23430_e36448_d_n6;
        locals.var_p_pd_dn7 = assign23430_e36448_d_n7;
        locals.var_p_pd_dn8 = assign23430_e36448_d_n8;
        locals.var_p_pd_dn9 = assign23430_e36448_d_n9;
        locals.var_p_pd_dn10 = assign23430_e36448_d_n10;
        locals.var_p_pd_dn11 = assign23430_e36448_d_n11;
        locals.var_p_pd_rv = 0.0;

        let (assign23440_e36463, assign23440_e36463_d_n3, assign23440_e36463_d_n4, assign23440_e36463_d_n5, assign23440_e36463_d_n6, assign23440_e36463_d_n7, assign23440_e36463_d_n8, assign23440_e36463_d_n9, assign23440_e36463_d_n10, assign23440_e36463_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23440_e36459: f64 = (2.0 * locals.var_xgm);
        let assign23440_e36460: f64 = (locals.var_x_pm - assign23440_e36459);
        let assign23440_e36461: f64 = (locals.var_x_pm * assign23440_e36460);
        (assign23440_e36461, ((locals.var_x_pm_dn3 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn3 - (2.0 * locals.var_xgm_dn3)))), ((locals.var_x_pm_dn4 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn4 - (2.0 * locals.var_xgm_dn4)))), ((locals.var_x_pm_dn5 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn5 - (2.0 * locals.var_xgm_dn5)))), ((locals.var_x_pm_dn6 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn6 - (2.0 * locals.var_xgm_dn6)))), ((locals.var_x_pm_dn7 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn7 - (2.0 * locals.var_xgm_dn7)))), ((locals.var_x_pm_dn8 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn8 - (2.0 * locals.var_xgm_dn8)))), ((locals.var_x_pm_dn9 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn9 - (2.0 * locals.var_xgm_dn9)))), ((locals.var_x_pm_dn10 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn10 - (2.0 * locals.var_xgm_dn10)))), ((locals.var_x_pm_dn11 * assign23440_e36460) + (locals.var_x_pm * (locals.var_x_pm_dn11 - (2.0 * locals.var_xgm_dn11)))),)
    } else {
        (locals.var_q_pd, locals.var_q_pd_dn3, locals.var_q_pd_dn4, locals.var_q_pd_dn5, locals.var_q_pd_dn6, locals.var_q_pd_dn7, locals.var_q_pd_dn8, locals.var_q_pd_dn9, locals.var_q_pd_dn10, locals.var_q_pd_dn11,)
    }
};
        locals.var_q_pd = assign23440_e36463;
        locals.var_q_pd_dn3 = assign23440_e36463_d_n3;
        locals.var_q_pd_dn4 = assign23440_e36463_d_n4;
        locals.var_q_pd_dn5 = assign23440_e36463_d_n5;
        locals.var_q_pd_dn6 = assign23440_e36463_d_n6;
        locals.var_q_pd_dn7 = assign23440_e36463_d_n7;
        locals.var_q_pd_dn8 = assign23440_e36463_d_n8;
        locals.var_q_pd_dn9 = assign23440_e36463_d_n9;
        locals.var_q_pd_dn10 = assign23440_e36463_d_n10;
        locals.var_q_pd_dn11 = assign23440_e36463_d_n11;
        locals.var_q_pd_rv = 0.0;

        let (assign23450_e36480, assign23450_e36480_d_n3, assign23450_e36480_d_n4, assign23450_e36480_d_n5, assign23450_e36480_d_n6, assign23450_e36480_d_n7, assign23450_e36480_d_n8, assign23450_e36480_d_n9, assign23450_e36480_d_n10, assign23450_e36480_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23450_e36475: f64 = (locals.var_em + locals.var_dm);
        let assign23450_e36476: f64 = (locals.var_gam2 * assign23450_e36475);
        let assign23450_e36477: f64 = (0.5 * assign23450_e36476);
        let assign23450_e36478: f64 = (1.0 - assign23450_e36477);
        (assign23450_e36478, (-(0.5 * ((locals.var_gam2_dn3 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn3 + locals.var_dm_dn3))))), (-(0.5 * ((locals.var_gam2_dn4 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn4 + locals.var_dm_dn4))))), (-(0.5 * ((locals.var_gam2_dn5 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn5 + locals.var_dm_dn5))))), (-(0.5 * ((locals.var_gam2_dn6 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn6 + locals.var_dm_dn6))))), (-(0.5 * ((locals.var_gam2_dn7 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn7 + locals.var_dm_dn7))))), (-(0.5 * ((locals.var_gam2_dn8 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn8 + locals.var_dm_dn8))))), (-(0.5 * ((locals.var_gam2_dn9 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn9 + locals.var_dm_dn9))))), (-(0.5 * ((locals.var_gam2_dn10 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn10 + locals.var_dm_dn10))))), (-(0.5 * ((locals.var_gam2_dn11 * assign23450_e36475) + (locals.var_gam2 * (locals.var_em_dn11 + locals.var_dm_dn11))))),)
    } else {
        (locals.var_xi_pd, locals.var_xi_pd_dn3, locals.var_xi_pd_dn4, locals.var_xi_pd_dn5, locals.var_xi_pd_dn6, locals.var_xi_pd_dn7, locals.var_xi_pd_dn8, locals.var_xi_pd_dn9, locals.var_xi_pd_dn10, locals.var_xi_pd_dn11,)
    }
};
        locals.var_xi_pd = assign23450_e36480;
        locals.var_xi_pd_dn3 = assign23450_e36480_d_n3;
        locals.var_xi_pd_dn4 = assign23450_e36480_d_n4;
        locals.var_xi_pd_dn5 = assign23450_e36480_d_n5;
        locals.var_xi_pd_dn6 = assign23450_e36480_d_n6;
        locals.var_xi_pd_dn7 = assign23450_e36480_d_n7;
        locals.var_xi_pd_dn8 = assign23450_e36480_d_n8;
        locals.var_xi_pd_dn9 = assign23450_e36480_d_n9;
        locals.var_xi_pd_dn10 = assign23450_e36480_d_n10;
        locals.var_xi_pd_dn11 = assign23450_e36480_d_n11;
        locals.var_xi_pd_rv = 0.0;

        let (assign23460_e36499, assign23460_e36499_d_n3, assign23460_e36499_d_n4, assign23460_e36499_d_n5, assign23460_e36499_d_n6, assign23460_e36499_d_n7, assign23460_e36499_d_n8, assign23460_e36499_d_n9, assign23460_e36499_d_n10, assign23460_e36499_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23460_e36489: f64 = (locals.var_q_pd * locals.var_p_pd);
        let assign23460_e36492: f64 = (locals.var_p_pd * locals.var_p_pd);
        let assign23460_e36495: f64 = (locals.var_xi_pd * locals.var_q_pd);
        let assign23460_e36496: f64 = (assign23460_e36492 - assign23460_e36495);
        let assign23460_e36497: f64 = (assign23460_e36489 / assign23460_e36496);
        (assign23460_e36497, (((((locals.var_q_pd_dn3 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn3)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn3 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn3)) - ((locals.var_xi_pd_dn3 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn3))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn4 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn4)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn4 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn4)) - ((locals.var_xi_pd_dn4 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn4))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn5 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn5)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn5 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn5)) - ((locals.var_xi_pd_dn5 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn5))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn6 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn6)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn6 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn6)) - ((locals.var_xi_pd_dn6 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn6))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn7 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn7)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn7 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn7)) - ((locals.var_xi_pd_dn7 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn7))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn8 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn8)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn8 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn8)) - ((locals.var_xi_pd_dn8 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn8))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn9 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn9)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn9 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn9)) - ((locals.var_xi_pd_dn9 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn9))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn10 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn10)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn10 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn10)) - ((locals.var_xi_pd_dn10 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn10))))) / (assign23460_e36496 * assign23460_e36496)), (((((locals.var_q_pd_dn11 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn11)) * assign23460_e36496) - (assign23460_e36489 * (((locals.var_p_pd_dn11 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn11)) - ((locals.var_xi_pd_dn11 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn11))))) / (assign23460_e36496 * assign23460_e36496)),)
    } else {
        (locals.var_u_pd, locals.var_u_pd_dn3, locals.var_u_pd_dn4, locals.var_u_pd_dn5, locals.var_u_pd_dn6, locals.var_u_pd_dn7, locals.var_u_pd_dn8, locals.var_u_pd_dn9, locals.var_u_pd_dn10, locals.var_u_pd_dn11,)
    }
};
        locals.var_u_pd = assign23460_e36499;
        locals.var_u_pd_dn3 = assign23460_e36499_d_n3;
        locals.var_u_pd_dn4 = assign23460_e36499_d_n4;
        locals.var_u_pd_dn5 = assign23460_e36499_d_n5;
        locals.var_u_pd_dn6 = assign23460_e36499_d_n6;
        locals.var_u_pd_dn7 = assign23460_e36499_d_n7;
        locals.var_u_pd_dn8 = assign23460_e36499_d_n8;
        locals.var_u_pd_dn9 = assign23460_e36499_d_n9;
        locals.var_u_pd_dn10 = assign23460_e36499_d_n10;
        locals.var_u_pd_dn11 = assign23460_e36499_d_n11;
        locals.var_u_pd_rv = 0.0;

        let (assign23470_e36510, assign23470_e36510_d_n3, assign23470_e36510_d_n4, assign23470_e36510_d_n5, assign23470_e36510_d_n6, assign23470_e36510_d_n7, assign23470_e36510_d_n8, assign23470_e36510_d_n9, assign23470_e36510_d_n10, assign23470_e36510_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23470_e36508: f64 = (locals.var_x_m + locals.var_u_pd);
        (assign23470_e36508, (locals.var_x_m_dn3 + locals.var_u_pd_dn3), (locals.var_x_m_dn4 + locals.var_u_pd_dn4), (locals.var_x_m_dn5 + locals.var_u_pd_dn5), (locals.var_x_m_dn6 + locals.var_u_pd_dn6), (locals.var_x_m_dn7 + locals.var_u_pd_dn7), (locals.var_x_m_dn8 + locals.var_u_pd_dn8), (locals.var_x_m_dn9 + locals.var_u_pd_dn9), (locals.var_x_m_dn10 + locals.var_u_pd_dn10), (locals.var_x_m_dn11 + locals.var_u_pd_dn11),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn3, locals.var_x_m_dn4, locals.var_x_m_dn5, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9, locals.var_x_m_dn10, locals.var_x_m_dn11,)
    }
};
        locals.var_x_m = assign23470_e36510;
        locals.var_x_m_dn3 = assign23470_e36510_d_n3;
        locals.var_x_m_dn4 = assign23470_e36510_d_n4;
        locals.var_x_m_dn5 = assign23470_e36510_d_n5;
        locals.var_x_m_dn6 = assign23470_e36510_d_n6;
        locals.var_x_m_dn7 = assign23470_e36510_d_n7;
        locals.var_x_m_dn8 = assign23470_e36510_d_n8;
        locals.var_x_m_dn9 = assign23470_e36510_d_n9;
        locals.var_x_m_dn10 = assign23470_e36510_d_n10;
        locals.var_x_m_dn11 = assign23470_e36510_d_n11;
        locals.var_x_m_rv = 0.0;

        let (assign23480_e36520, assign23480_e36520_d_n3, assign23480_e36520_d_n4, assign23480_e36520_d_n5, assign23480_e36520_d_n6, assign23480_e36520_d_n7, assign23480_e36520_d_n8, assign23480_e36520_d_n9, assign23480_e36520_d_n10, assign23480_e36520_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23480_e36518: f64 = { let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23480_e36518, ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn3), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn4), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn5), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn6), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn7), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn8), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn9), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn10), ({ let limited_exp_arg = locals.var_u_pd; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_u_pd_dn11),)
    } else {
        (locals.var_km, locals.var_km_dn3, locals.var_km_dn4, locals.var_km_dn5, locals.var_km_dn6, locals.var_km_dn7, locals.var_km_dn8, locals.var_km_dn9, locals.var_km_dn10, locals.var_km_dn11,)
    }
};
        locals.var_km = assign23480_e36520;
        locals.var_km_dn3 = assign23480_e36520_d_n3;
        locals.var_km_dn4 = assign23480_e36520_d_n4;
        locals.var_km_dn5 = assign23480_e36520_d_n5;
        locals.var_km_dn6 = assign23480_e36520_d_n6;
        locals.var_km_dn7 = assign23480_e36520_d_n7;
        locals.var_km_dn8 = assign23480_e36520_d_n8;
        locals.var_km_dn9 = assign23480_e36520_d_n9;
        locals.var_km_dn10 = assign23480_e36520_d_n10;
        locals.var_km_dn11 = assign23480_e36520_d_n11;
        locals.var_km_rv = 0.0;

        let (assign23490_e36531, assign23490_e36531_d_n3, assign23490_e36531_d_n4, assign23490_e36531_d_n5, assign23490_e36531_d_n6, assign23490_e36531_d_n7, assign23490_e36531_d_n8, assign23490_e36531_d_n9, assign23490_e36531_d_n10, assign23490_e36531_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23490_e36529: f64 = (locals.var_em / locals.var_km);
        (assign23490_e36529, (((locals.var_em_dn3 * locals.var_km) - (locals.var_em * locals.var_km_dn3)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn4 * locals.var_km) - (locals.var_em * locals.var_km_dn4)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn5 * locals.var_km) - (locals.var_em * locals.var_km_dn5)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn6 * locals.var_km) - (locals.var_em * locals.var_km_dn6)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn7 * locals.var_km) - (locals.var_em * locals.var_km_dn7)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn8 * locals.var_km) - (locals.var_em * locals.var_km_dn8)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn9 * locals.var_km) - (locals.var_em * locals.var_km_dn9)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn10 * locals.var_km) - (locals.var_em * locals.var_km_dn10)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn11 * locals.var_km) - (locals.var_em * locals.var_km_dn11)) / (locals.var_km * locals.var_km)),)
    } else {
        (locals.var_em, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11,)
    }
};
        locals.var_em = assign23490_e36531;
        locals.var_em_dn3 = assign23490_e36531_d_n3;
        locals.var_em_dn4 = assign23490_e36531_d_n4;
        locals.var_em_dn5 = assign23490_e36531_d_n5;
        locals.var_em_dn6 = assign23490_e36531_d_n6;
        locals.var_em_dn7 = assign23490_e36531_d_n7;
        locals.var_em_dn8 = assign23490_e36531_d_n8;
        locals.var_em_dn9 = assign23490_e36531_d_n9;
        locals.var_em_dn10 = assign23490_e36531_d_n10;
        locals.var_em_dn11 = assign23490_e36531_d_n11;
        locals.var_em_rv = 0.0;

        let (assign23500_e36542, assign23500_e36542_d_n3, assign23500_e36542_d_n4, assign23500_e36542_d_n5, assign23500_e36542_d_n6, assign23500_e36542_d_n7, assign23500_e36542_d_n8, assign23500_e36542_d_n9, assign23500_e36542_d_n10, assign23500_e36542_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23500_e36540: f64 = (locals.var_dm * locals.var_km);
        (assign23500_e36540, ((locals.var_dm_dn3 * locals.var_km) + (locals.var_dm * locals.var_km_dn3)), ((locals.var_dm_dn4 * locals.var_km) + (locals.var_dm * locals.var_km_dn4)), ((locals.var_dm_dn5 * locals.var_km) + (locals.var_dm * locals.var_km_dn5)), ((locals.var_dm_dn6 * locals.var_km) + (locals.var_dm * locals.var_km_dn6)), ((locals.var_dm_dn7 * locals.var_km) + (locals.var_dm * locals.var_km_dn7)), ((locals.var_dm_dn8 * locals.var_km) + (locals.var_dm * locals.var_km_dn8)), ((locals.var_dm_dn9 * locals.var_km) + (locals.var_dm * locals.var_km_dn9)), ((locals.var_dm_dn10 * locals.var_km) + (locals.var_dm * locals.var_km_dn10)), ((locals.var_dm_dn11 * locals.var_km) + (locals.var_dm * locals.var_km_dn11)),)
    } else {
        (locals.var_dm, locals.var_dm_dn3, locals.var_dm_dn4, locals.var_dm_dn5, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9, locals.var_dm_dn10, locals.var_dm_dn11,)
    }
};
        locals.var_dm = assign23500_e36542;
        locals.var_dm_dn3 = assign23500_e36542_d_n3;
        locals.var_dm_dn4 = assign23500_e36542_d_n4;
        locals.var_dm_dn5 = assign23500_e36542_d_n5;
        locals.var_dm_dn6 = assign23500_e36542_d_n6;
        locals.var_dm_dn7 = assign23500_e36542_d_n7;
        locals.var_dm_dn8 = assign23500_e36542_d_n8;
        locals.var_dm_dn9 = assign23500_e36542_d_n9;
        locals.var_dm_dn10 = assign23500_e36542_d_n10;
        locals.var_dm_dn11 = assign23500_e36542_d_n11;
        locals.var_dm_rv = 0.0;

        let (assign23510_e36569, assign23510_e36569_d_n3, assign23510_e36569_d_n4, assign23510_e36569_d_n5, assign23510_e36569_d_n6, assign23510_e36569_d_n7, assign23510_e36569_d_n8, assign23510_e36569_d_n9, assign23510_e36569_d_n10, assign23510_e36569_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23510_e36551: f64 = (locals.var_vgfb1 - locals.var_x_m);
        let assign23510_e36553: f64 = (assign23510_e36551 + locals.var_u_pd);
        let assign23510_e36556: f64 = (locals.var_vgfb1 - locals.var_x_m);
        let assign23510_e36558: f64 = (assign23510_e36556 + locals.var_u_pd);
        let assign23510_e36559: f64 = (assign23510_e36553 * assign23510_e36558);
        let assign23510_e36562: f64 = (1.0 / locals.var_gam2);
        let assign23510_e36563: f64 = (assign23510_e36559 * assign23510_e36562);
        let assign23510_e36566: f64 = (locals.var_dm / locals.var_km);
        let assign23510_e36567: f64 = (assign23510_e36563 - assign23510_e36566);
        (assign23510_e36567, (((((((locals.var_vgfb1_dn3 - locals.var_x_m_dn3) + locals.var_u_pd_dn3) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn3 - locals.var_x_m_dn3) + locals.var_u_pd_dn3))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn3 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn3 * locals.var_km) - (locals.var_dm * locals.var_km_dn3)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn4 - locals.var_x_m_dn4) + locals.var_u_pd_dn4) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn4 - locals.var_x_m_dn4) + locals.var_u_pd_dn4))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn4 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn4 * locals.var_km) - (locals.var_dm * locals.var_km_dn4)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn5 - locals.var_x_m_dn5) + locals.var_u_pd_dn5) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn5 - locals.var_x_m_dn5) + locals.var_u_pd_dn5))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn5 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn5 * locals.var_km) - (locals.var_dm * locals.var_km_dn5)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn6 - locals.var_x_m_dn6) + locals.var_u_pd_dn6) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn6 - locals.var_x_m_dn6) + locals.var_u_pd_dn6))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn6 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn6 * locals.var_km) - (locals.var_dm * locals.var_km_dn6)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn7 - locals.var_x_m_dn7) + locals.var_u_pd_dn7) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn7 - locals.var_x_m_dn7) + locals.var_u_pd_dn7))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn7 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn7 * locals.var_km) - (locals.var_dm * locals.var_km_dn7)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn8 - locals.var_x_m_dn8) + locals.var_u_pd_dn8) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn8 - locals.var_x_m_dn8) + locals.var_u_pd_dn8))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn8 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn8 * locals.var_km) - (locals.var_dm * locals.var_km_dn8)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn9 - locals.var_x_m_dn9) + locals.var_u_pd_dn9) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn9 - locals.var_x_m_dn9) + locals.var_u_pd_dn9))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn9 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn9 * locals.var_km) - (locals.var_dm * locals.var_km_dn9)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn10 - locals.var_x_m_dn10) + locals.var_u_pd_dn10) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn10 - locals.var_x_m_dn10) + locals.var_u_pd_dn10))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn10 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn10 * locals.var_km) - (locals.var_dm * locals.var_km_dn10)) / (locals.var_km * locals.var_km))), (((((((locals.var_vgfb1_dn11 - locals.var_x_m_dn11) + locals.var_u_pd_dn11) * assign23510_e36558) + (assign23510_e36553 * ((locals.var_vgfb1_dn11 - locals.var_x_m_dn11) + locals.var_u_pd_dn11))) * assign23510_e36562) + (assign23510_e36559 * (-(locals.var_gam2_dn11 / (locals.var_gam2 * locals.var_gam2))))) - (((locals.var_dm_dn11 * locals.var_km) - (locals.var_dm * locals.var_km_dn11)) / (locals.var_km * locals.var_km))),)
    } else {
        (locals.var_pm, locals.var_pm_dn3, locals.var_pm_dn4, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9, locals.var_pm_dn10, locals.var_pm_dn11,)
    }
};
        locals.var_pm = assign23510_e36569;
        locals.var_pm_dn3 = assign23510_e36569_d_n3;
        locals.var_pm_dn4 = assign23510_e36569_d_n4;
        locals.var_pm_dn5 = assign23510_e36569_d_n5;
        locals.var_pm_dn6 = assign23510_e36569_d_n6;
        locals.var_pm_dn7 = assign23510_e36569_d_n7;
        locals.var_pm_dn8 = assign23510_e36569_d_n8;
        locals.var_pm_dn9 = assign23510_e36569_d_n9;
        locals.var_pm_dn10 = assign23510_e36569_d_n10;
        locals.var_pm_dn11 = assign23510_e36569_d_n11;
        locals.var_pm_rv = 0.0;

        let (assign23520_e36583, assign23520_e36583_d_n3, assign23520_e36583_d_n4, assign23520_e36583_d_n5, assign23520_e36583_d_n6, assign23520_e36583_d_n7, assign23520_e36583_d_n8, assign23520_e36583_d_n9, assign23520_e36583_d_n10, assign23520_e36583_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23520_e36579: f64 = (locals.var_dm + locals.var_pm);
        let assign23520_e36580: f64 = (assign23520_e36579).sqrt();
        let assign23520_e36581: f64 = (locals.var_gam * assign23520_e36580);
        (assign23520_e36581, ((locals.var_gam_dn3 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn3 + locals.var_pm_dn3) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn4 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn5 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn5 + locals.var_pm_dn5) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn6 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn7 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn8 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn9 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn10 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn10 + locals.var_pm_dn10) / (2.0 * assign23520_e36580)))), ((locals.var_gam_dn11 * assign23520_e36580) + (locals.var_gam * ((locals.var_dm_dn11 + locals.var_pm_dn11) / (2.0 * assign23520_e36580)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn3, locals.var_xgm_dn4, locals.var_xgm_dn5, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9, locals.var_xgm_dn10, locals.var_xgm_dn11,)
    }
};
        locals.var_xgm = assign23520_e36583;
        locals.var_xgm_dn3 = assign23520_e36583_d_n3;
        locals.var_xgm_dn4 = assign23520_e36583_d_n4;
        locals.var_xgm_dn5 = assign23520_e36583_d_n5;
        locals.var_xgm_dn6 = assign23520_e36583_d_n6;
        locals.var_xgm_dn7 = assign23520_e36583_d_n7;
        locals.var_xgm_dn8 = assign23520_e36583_d_n8;
        locals.var_xgm_dn9 = assign23520_e36583_d_n9;
        locals.var_xgm_dn10 = assign23520_e36583_d_n10;
        locals.var_xgm_dn11 = assign23520_e36583_d_n11;
        locals.var_xgm_rv = 0.0;

        let (assign23530_e36602, assign23530_e36602_d_n3, assign23530_e36602_d_n4, assign23530_e36602_d_n5, assign23530_e36602_d_n6, assign23530_e36602_d_n7, assign23530_e36602_d_n8, assign23530_e36602_d_n9, assign23530_e36602_d_n10, assign23530_e36602_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23530_e36592: f64 = (1.0 - locals.var_em);
        let assign23530_e36596: f64 = (locals.var_xgm * locals.var_eta_p);
        let assign23530_e36598: f64 = (assign23530_e36596 * locals.var_inv_gam2);
        let assign23530_e36599: f64 = (2.0 * assign23530_e36598);
        let assign23530_e36600: f64 = (assign23530_e36592 + assign23530_e36599);
        (assign23530_e36600, ((-locals.var_em_dn3) + (2.0 * ((((locals.var_xgm_dn3 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn3)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn3)))), ((-locals.var_em_dn4) + (2.0 * ((((locals.var_xgm_dn4 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn4)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn4)))), ((-locals.var_em_dn5) + (2.0 * ((((locals.var_xgm_dn5 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn5)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn5)))), ((-locals.var_em_dn6) + (2.0 * ((((locals.var_xgm_dn6 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn6)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((((locals.var_xgm_dn7 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn7)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((((locals.var_xgm_dn8 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn8)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn8)))), ((-locals.var_em_dn9) + (2.0 * ((((locals.var_xgm_dn9 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn9)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn9)))), ((-locals.var_em_dn10) + (2.0 * ((((locals.var_xgm_dn10 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn10)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn10)))), ((-locals.var_em_dn11) + (2.0 * ((((locals.var_xgm_dn11 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn11)) * locals.var_inv_gam2) + (assign23530_e36596 * locals.var_inv_gam2_dn11)))),)
    } else {
        (locals.var_km0, locals.var_km0_dn3, locals.var_km0_dn4, locals.var_km0_dn5, locals.var_km0_dn6, locals.var_km0_dn7, locals.var_km0_dn8, locals.var_km0_dn9, locals.var_km0_dn10, locals.var_km0_dn11,)
    }
};
        locals.var_km0 = assign23530_e36602;
        locals.var_km0_dn3 = assign23530_e36602_d_n3;
        locals.var_km0_dn4 = assign23530_e36602_d_n4;
        locals.var_km0_dn5 = assign23530_e36602_d_n5;
        locals.var_km0_dn6 = assign23530_e36602_d_n6;
        locals.var_km0_dn7 = assign23530_e36602_d_n7;
        locals.var_km0_dn8 = assign23530_e36602_d_n8;
        locals.var_km0_dn9 = assign23530_e36602_d_n9;
        locals.var_km0_dn10 = assign23530_e36602_d_n10;
        locals.var_km0_dn11 = assign23530_e36602_d_n11;
        locals.var_km0_rv = 0.0;

        let (assign23540_e36623, assign23540_e36623_d_n3, assign23540_e36623_d_n4, assign23540_e36623_d_n5, assign23540_e36623_d_n6, assign23540_e36623_d_n7, assign23540_e36623_d_n8, assign23540_e36623_d_n9, assign23540_e36623_d_n10, assign23540_e36623_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23540_e36611: f64 = (locals.var_x_ds * locals.var_km);
        let assign23540_e36614: f64 = (locals.var_d0 + locals.var_d_bar);
        let assign23540_e36615: f64 = (assign23540_e36611 * assign23540_e36614);
        let assign23540_e36619: f64 = (locals.var_km * locals.var_d_bar);
        let assign23540_e36620: f64 = (locals.var_km0 + assign23540_e36619);
        let assign23540_e36621: f64 = (assign23540_e36615 / assign23540_e36620);
        (assign23540_e36621, (((((((locals.var_x_ds_dn3 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn3)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn3 + locals.var_d_bar_dn3))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn3 + ((locals.var_km_dn3 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn3))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn4 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn4)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn4 + locals.var_d_bar_dn4))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn4 + ((locals.var_km_dn4 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn4))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn5 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn5)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn5 + locals.var_d_bar_dn5))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn5 + ((locals.var_km_dn5 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn5))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn6 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn6)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn6 + locals.var_d_bar_dn6))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn6 + ((locals.var_km_dn6 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn6))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn7 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn7)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn7 + locals.var_d_bar_dn7))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn7 + ((locals.var_km_dn7 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn7))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn8 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn8)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn8 + locals.var_d_bar_dn8))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn8 + ((locals.var_km_dn8 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn8))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn9 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn9)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn9 + locals.var_d_bar_dn9))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn9 + ((locals.var_km_dn9 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn9))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn10 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn10)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn10 + locals.var_d_bar_dn10))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn10 + ((locals.var_km_dn10 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn10))))) / (assign23540_e36620 * assign23540_e36620)), (((((((locals.var_x_ds_dn11 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn11)) * assign23540_e36614) + (assign23540_e36611 * (locals.var_d0_dn11 + locals.var_d_bar_dn11))) * assign23540_e36620) - (assign23540_e36615 * (locals.var_km0_dn11 + ((locals.var_km_dn11 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn11))))) / (assign23540_e36620 * assign23540_e36620)),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn3, locals.var_x_ds_dn4, locals.var_x_ds_dn5, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9, locals.var_x_ds_dn10, locals.var_x_ds_dn11,)
    }
};
        locals.var_x_ds = assign23540_e36623;
        locals.var_x_ds_dn3 = assign23540_e36623_d_n3;
        locals.var_x_ds_dn4 = assign23540_e36623_d_n4;
        locals.var_x_ds_dn5 = assign23540_e36623_d_n5;
        locals.var_x_ds_dn6 = assign23540_e36623_d_n6;
        locals.var_x_ds_dn7 = assign23540_e36623_d_n7;
        locals.var_x_ds_dn8 = assign23540_e36623_d_n8;
        locals.var_x_ds_dn9 = assign23540_e36623_d_n9;
        locals.var_x_ds_dn10 = assign23540_e36623_d_n10;
        locals.var_x_ds_dn11 = assign23540_e36623_d_n11;
        locals.var_x_ds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        locals: &mut StampLocals,
    ) {
        let (assign23550_e36634, assign23550_e36634_d_n3, assign23550_e36634_d_n4, assign23550_e36634_d_n5, assign23550_e36634_d_n6, assign23550_e36634_d_n7, assign23550_e36634_d_n8, assign23550_e36634_d_n9, assign23550_e36634_d_n10, assign23550_e36634_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23550_e36632: f64 = (locals.var_x_ds * locals.var_nvt);
        (assign23550_e36632, ((locals.var_x_ds_dn3 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn3)), ((locals.var_x_ds_dn4 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn4)), ((locals.var_x_ds_dn5 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn5)), ((locals.var_x_ds_dn6 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn6)), ((locals.var_x_ds_dn7 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn7)), ((locals.var_x_ds_dn8 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn8)), ((locals.var_x_ds_dn9 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn9)), ((locals.var_x_ds_dn10 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn10)), ((locals.var_x_ds_dn11 * locals.var_nvt) + (locals.var_x_ds * locals.var_nvt_dn11)),)
    } else {
        (locals.var_dps, locals.var_dps_dn3, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11,)
    }
};
        locals.var_dps = assign23550_e36634;
        locals.var_dps_dn3 = assign23550_e36634_d_n3;
        locals.var_dps_dn4 = assign23550_e36634_d_n4;
        locals.var_dps_dn5 = assign23550_e36634_d_n5;
        locals.var_dps_dn6 = assign23550_e36634_d_n6;
        locals.var_dps_dn7 = assign23550_e36634_d_n7;
        locals.var_dps_dn8 = assign23550_e36634_d_n8;
        locals.var_dps_dn9 = assign23550_e36634_d_n9;
        locals.var_dps_dn10 = assign23550_e36634_d_n10;
        locals.var_dps_dn11 = assign23550_e36634_d_n11;
        locals.var_dps_rv = 0.0;

        let (assign23560_e36664, assign23560_e36664_d_n3, assign23560_e36664_d_n4, assign23560_e36664_d_n5, assign23560_e36664_d_n6, assign23560_e36664_d_n7, assign23560_e36664_d_n8, assign23560_e36664_d_n9, assign23560_e36664_d_n10, assign23560_e36664_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23560_e36644: f64 = (locals.var_pm - 0.001);
        let assign23560_e36647: f64 = (locals.var_pm - 0.001);
        let assign23560_e36650: f64 = (locals.var_pm - 0.001);
        let assign23560_e36651: f64 = (assign23560_e36647 * assign23560_e36650);
        let assign23560_e36654: f64 = (4.0 * 1e-5);
        let assign23560_e36656: f64 = (assign23560_e36654 * 1e-5);
        let assign23560_e36657: f64 = (assign23560_e36651 + assign23560_e36656);
        let assign23560_e36658: f64 = (assign23560_e36657).sqrt();
        let assign23560_e36659: f64 = (assign23560_e36644 + assign23560_e36658);
        let assign23560_e36660: f64 = (0.5 * assign23560_e36659);
        let assign23560_e36662: f64 = (assign23560_e36660 + 0.001);
        (assign23560_e36662, (0.5 * (locals.var_pm_dn3 + (((locals.var_pm_dn3 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn3)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn4 + (((locals.var_pm_dn4 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn4)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn5 + (((locals.var_pm_dn5 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn5)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn6 + (((locals.var_pm_dn6 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn6)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn7 + (((locals.var_pm_dn7 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn7)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn8 + (((locals.var_pm_dn8 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn8)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn9 + (((locals.var_pm_dn9 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn9)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn10 + (((locals.var_pm_dn10 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn10)) / (2.0 * assign23560_e36658)))), (0.5 * (locals.var_pm_dn11 + (((locals.var_pm_dn11 * assign23560_e36650) + (assign23560_e36647 * locals.var_pm_dn11)) / (2.0 * assign23560_e36658)))),)
    } else {
        (locals.var_pm, locals.var_pm_dn3, locals.var_pm_dn4, locals.var_pm_dn5, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9, locals.var_pm_dn10, locals.var_pm_dn11,)
    }
};
        locals.var_pm = assign23560_e36664;
        locals.var_pm_dn3 = assign23560_e36664_d_n3;
        locals.var_pm_dn4 = assign23560_e36664_d_n4;
        locals.var_pm_dn5 = assign23560_e36664_d_n5;
        locals.var_pm_dn6 = assign23560_e36664_d_n6;
        locals.var_pm_dn7 = assign23560_e36664_d_n7;
        locals.var_pm_dn8 = assign23560_e36664_d_n8;
        locals.var_pm_dn9 = assign23560_e36664_d_n9;
        locals.var_pm_dn10 = assign23560_e36664_d_n10;
        locals.var_pm_dn11 = assign23560_e36664_d_n11;
        locals.var_pm_rv = 0.0;

        let (assign23570_e36674, assign23570_e36674_d_n3, assign23570_e36674_d_n4, assign23570_e36674_d_n5, assign23570_e36674_d_n6, assign23570_e36674_d_n7, assign23570_e36674_d_n8, assign23570_e36674_d_n9, assign23570_e36674_d_n10, assign23570_e36674_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard523 != 0.0)) {
        let assign23570_e36672: f64 = (locals.var_pm).sqrt();
        (assign23570_e36672, (locals.var_pm_dn3 / (2.0 * assign23570_e36672)), (locals.var_pm_dn4 / (2.0 * assign23570_e36672)), (locals.var_pm_dn5 / (2.0 * assign23570_e36672)), (locals.var_pm_dn6 / (2.0 * assign23570_e36672)), (locals.var_pm_dn7 / (2.0 * assign23570_e36672)), (locals.var_pm_dn8 / (2.0 * assign23570_e36672)), (locals.var_pm_dn9 / (2.0 * assign23570_e36672)), (locals.var_pm_dn10 / (2.0 * assign23570_e36672)), (locals.var_pm_dn11 / (2.0 * assign23570_e36672)),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn3, locals.var_sqm_dn4, locals.var_sqm_dn5, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, locals.var_sqm_dn9, locals.var_sqm_dn10, locals.var_sqm_dn11,)
    }
};
        locals.var_sqm = assign23570_e36674;
        locals.var_sqm_dn3 = assign23570_e36674_d_n3;
        locals.var_sqm_dn4 = assign23570_e36674_d_n4;
        locals.var_sqm_dn5 = assign23570_e36674_d_n5;
        locals.var_sqm_dn6 = assign23570_e36674_d_n6;
        locals.var_sqm_dn7 = assign23570_e36674_d_n7;
        locals.var_sqm_dn8 = assign23570_e36674_d_n8;
        locals.var_sqm_dn9 = assign23570_e36674_d_n9;
        locals.var_sqm_dn10 = assign23570_e36674_d_n10;
        locals.var_sqm_dn11 = assign23570_e36674_d_n11;
        locals.var_sqm_rv = 0.0;

        let assign23580_e36676: f64 = (locals.var_dps).abs();
        let assign23580_e36678: f64 = if assign23580_e36676 > 1e-35 { 1.0 } else { 0.0 };
        locals.var_guard524 = assign23580_e36678;
        locals.var_guard524_rv = 0.0;

        let (assign23590_e36691, assign23590_e36691_d_n3, assign23590_e36691_d_n4, assign23590_e36691_d_n5, assign23590_e36691_d_n6, assign23590_e36691_d_n7, assign23590_e36691_d_n8, assign23590_e36691_d_n9, assign23590_e36691_d_n10, assign23590_e36691_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard524 != 0.0)) {
        let assign23590_e36687: f64 = (locals.var_qis - locals.var_qid);
        let assign23590_e36689: f64 = (assign23590_e36687 / locals.var_dps);
        (assign23590_e36689, ((((locals.var_qis_dn3 - locals.var_qid_dn3) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn3)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn4 - locals.var_qid_dn4) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn4)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn5 - locals.var_qid_dn5) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn5)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn6 - locals.var_qid_dn6) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn6)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn7 - locals.var_qid_dn7) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn7)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn8 - locals.var_qid_dn8) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn8)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn9 - locals.var_qid_dn9) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn9)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn10 - locals.var_qid_dn10) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn10)) / (locals.var_dps * locals.var_dps)), ((((locals.var_qis_dn11 - locals.var_qid_dn11) * locals.var_dps) - (assign23590_e36687 * locals.var_dps_dn11)) / (locals.var_dps * locals.var_dps)),)
    } else {
        (locals.var_alpha_dd, locals.var_alpha_dd_dn3, locals.var_alpha_dd_dn4, locals.var_alpha_dd_dn5, locals.var_alpha_dd_dn6, locals.var_alpha_dd_dn7, locals.var_alpha_dd_dn8, locals.var_alpha_dd_dn9, locals.var_alpha_dd_dn10, locals.var_alpha_dd_dn11,)
    }
};
        locals.var_alpha_dd = assign23590_e36691;
        locals.var_alpha_dd_dn3 = assign23590_e36691_d_n3;
        locals.var_alpha_dd_dn4 = assign23590_e36691_d_n4;
        locals.var_alpha_dd_dn5 = assign23590_e36691_d_n5;
        locals.var_alpha_dd_dn6 = assign23590_e36691_d_n6;
        locals.var_alpha_dd_dn7 = assign23590_e36691_d_n7;
        locals.var_alpha_dd_dn8 = assign23590_e36691_d_n8;
        locals.var_alpha_dd_dn9 = assign23590_e36691_d_n9;
        locals.var_alpha_dd_dn10 = assign23590_e36691_d_n10;
        locals.var_alpha_dd_dn11 = assign23590_e36691_d_n11;
        locals.var_alpha_dd_rv = 0.0;

        let (assign23600_e36708, assign23600_e36708_d_n3, assign23600_e36708_d_n4, assign23600_e36708_d_n5, assign23600_e36708_d_n6, assign23600_e36708_d_n7, assign23600_e36708_d_n8, assign23600_e36708_d_n9, assign23600_e36708_d_n10, assign23600_e36708_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23600_e36699: f64 = (locals.var_gam2 * locals.var_dm);
        let assign23600_e36703: f64 = (locals.var_gam * locals.var_sqm);
        let assign23600_e36704: f64 = (locals.var_xgm + assign23600_e36703);
        let assign23600_e36705: f64 = (assign23600_e36699 / assign23600_e36704);
        let assign23600_e36706: f64 = (locals.var_nvt * assign23600_e36705);
        (assign23600_e36706, ((locals.var_nvt_dn3 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn3 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn3)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn3 + ((locals.var_gam_dn3 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn3))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn4 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn4 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn4)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn4 + ((locals.var_gam_dn4 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn4))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn5 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn5 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn5)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn5 + ((locals.var_gam_dn5 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn5))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn6 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn6 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn6)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn6 + ((locals.var_gam_dn6 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn6))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn7 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn7 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn7)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn7 + ((locals.var_gam_dn7 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn7))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn8 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn8 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn8)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn8 + ((locals.var_gam_dn8 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn8))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn9 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn9 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn9)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn9 + ((locals.var_gam_dn9 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn9))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn10 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn10 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn10)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn10 + ((locals.var_gam_dn10 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn10))))) / (assign23600_e36704 * assign23600_e36704)))), ((locals.var_nvt_dn11 * assign23600_e36705) + (locals.var_nvt * (((((locals.var_gam2_dn11 * locals.var_dm) + (locals.var_gam2 * locals.var_dm_dn11)) * assign23600_e36704) - (assign23600_e36699 * (locals.var_xgm_dn11 + ((locals.var_gam_dn11 * locals.var_sqm) + (locals.var_gam * locals.var_sqm_dn11))))) / (assign23600_e36704 * assign23600_e36704)))),)
    } else {
        (locals.var_qim, locals.var_qim_dn3, locals.var_qim_dn4, locals.var_qim_dn5, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8, locals.var_qim_dn9, locals.var_qim_dn10, locals.var_qim_dn11,)
    }
};
        locals.var_qim = assign23600_e36708;
        locals.var_qim_dn3 = assign23600_e36708_d_n3;
        locals.var_qim_dn4 = assign23600_e36708_d_n4;
        locals.var_qim_dn5 = assign23600_e36708_d_n5;
        locals.var_qim_dn6 = assign23600_e36708_d_n6;
        locals.var_qim_dn7 = assign23600_e36708_d_n7;
        locals.var_qim_dn8 = assign23600_e36708_d_n8;
        locals.var_qim_dn9 = assign23600_e36708_d_n9;
        locals.var_qim_dn10 = assign23600_e36708_d_n10;
        locals.var_qim_dn11 = assign23600_e36708_d_n11;
        locals.var_qim_rv = 0.0;

        let (assign23610_e36719, assign23610_e36719_d_n3, assign23610_e36719_d_n4, assign23610_e36719_d_n5, assign23610_e36719_d_n6, assign23610_e36719_d_n7, assign23610_e36719_d_n8, assign23610_e36719_d_n9, assign23610_e36719_d_n10, assign23610_e36719_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23610_e36715: f64 = (locals.var_sqm * locals.var_gam);
        let assign23610_e36717: f64 = (assign23610_e36715 * locals.var_nvt);
        (assign23610_e36717, ((((locals.var_sqm_dn3 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn3)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn3)), ((((locals.var_sqm_dn4 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn4)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn4)), ((((locals.var_sqm_dn5 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn5)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn5)), ((((locals.var_sqm_dn6 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn6)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn6)), ((((locals.var_sqm_dn7 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn7)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn7)), ((((locals.var_sqm_dn8 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn8)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn8)), ((((locals.var_sqm_dn9 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn9)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn9)), ((((locals.var_sqm_dn10 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn10)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn10)), ((((locals.var_sqm_dn11 * locals.var_gam) + (locals.var_sqm * locals.var_gam_dn11)) * locals.var_nvt) + (assign23610_e36715 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_qbm, locals.var_qbm_dn3, locals.var_qbm_dn4, locals.var_qbm_dn5, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8, locals.var_qbm_dn9, locals.var_qbm_dn10, locals.var_qbm_dn11,)
    }
};
        locals.var_qbm = assign23610_e36719;
        locals.var_qbm_dn3 = assign23610_e36719_d_n3;
        locals.var_qbm_dn4 = assign23610_e36719_d_n4;
        locals.var_qbm_dn5 = assign23610_e36719_d_n5;
        locals.var_qbm_dn6 = assign23610_e36719_d_n6;
        locals.var_qbm_dn7 = assign23610_e36719_d_n7;
        locals.var_qbm_dn8 = assign23610_e36719_d_n8;
        locals.var_qbm_dn9 = assign23610_e36719_d_n9;
        locals.var_qbm_dn10 = assign23610_e36719_d_n10;
        locals.var_qbm_dn11 = assign23610_e36719_d_n11;
        locals.var_qbm_rv = 0.0;

        let (assign23620_e36728, assign23620_e36728_d_n3, assign23620_e36728_d_n4, assign23620_e36728_d_n5, assign23620_e36728_d_n6, assign23620_e36728_d_n7, assign23620_e36728_d_n8, assign23620_e36728_d_n9, assign23620_e36728_d_n10, assign23620_e36728_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23620_e36726: f64 = (locals.var_xgm * locals.var_nvt);
        (assign23620_e36726, ((locals.var_xgm_dn3 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn3)), ((locals.var_xgm_dn4 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn4)), ((locals.var_xgm_dn5 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn5)), ((locals.var_xgm_dn6 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn6)), ((locals.var_xgm_dn7 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn7)), ((locals.var_xgm_dn8 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn8)), ((locals.var_xgm_dn9 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn9)), ((locals.var_xgm_dn10 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn10)), ((locals.var_xgm_dn11 * locals.var_nvt) + (locals.var_xgm * locals.var_nvt_dn11)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn3, locals.var_voxm_dn4, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9, locals.var_voxm_dn10, locals.var_voxm_dn11,)
    }
};
        locals.var_voxm = assign23620_e36728;
        locals.var_voxm_dn3 = assign23620_e36728_d_n3;
        locals.var_voxm_dn4 = assign23620_e36728_d_n4;
        locals.var_voxm_dn5 = assign23620_e36728_d_n5;
        locals.var_voxm_dn6 = assign23620_e36728_d_n6;
        locals.var_voxm_dn7 = assign23620_e36728_d_n7;
        locals.var_voxm_dn8 = assign23620_e36728_d_n8;
        locals.var_voxm_dn9 = assign23620_e36728_d_n9;
        locals.var_voxm_dn10 = assign23620_e36728_d_n10;
        locals.var_voxm_dn11 = assign23620_e36728_d_n11;
        locals.var_voxm_rv = 0.0;

        let (assign23630_e36741, assign23630_e36741_d_n3, assign23630_e36741_d_n4, assign23630_e36741_d_n5, assign23630_e36741_d_n6, assign23630_e36741_d_n7, assign23630_e36741_d_n8, assign23630_e36741_d_n9, assign23630_e36741_d_n10, assign23630_e36741_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23630_e36737: f64 = (locals.var_eta_mu * locals.var_qim);
        let assign23630_e36738: f64 = (locals.var_qbm + assign23630_e36737);
        let assign23630_e36739: f64 = (locals.var_eefffactor * assign23630_e36738);
        (assign23630_e36739, (locals.var_eefffactor * (locals.var_qbm_dn3 + (locals.var_eta_mu * locals.var_qim_dn3))), (locals.var_eefffactor * (locals.var_qbm_dn4 + (locals.var_eta_mu * locals.var_qim_dn4))), (locals.var_eefffactor * (locals.var_qbm_dn5 + (locals.var_eta_mu * locals.var_qim_dn5))), (locals.var_eefffactor * (locals.var_qbm_dn6 + (locals.var_eta_mu * locals.var_qim_dn6))), (locals.var_eefffactor * (locals.var_qbm_dn7 + (locals.var_eta_mu * locals.var_qim_dn7))), (locals.var_eefffactor * (locals.var_qbm_dn8 + (locals.var_eta_mu * locals.var_qim_dn8))), (locals.var_eefffactor * (locals.var_qbm_dn9 + (locals.var_eta_mu * locals.var_qim_dn9))), (locals.var_eefffactor * (locals.var_qbm_dn10 + (locals.var_eta_mu * locals.var_qim_dn10))), (locals.var_eefffactor * (locals.var_qbm_dn11 + (locals.var_eta_mu * locals.var_qim_dn11))),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn3, locals.var_eeffm_dn4, locals.var_eeffm_dn5, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8, locals.var_eeffm_dn9, locals.var_eeffm_dn10, locals.var_eeffm_dn11,)
    }
};
        locals.var_eeffm = assign23630_e36741;
        locals.var_eeffm_dn3 = assign23630_e36741_d_n3;
        locals.var_eeffm_dn4 = assign23630_e36741_d_n4;
        locals.var_eeffm_dn5 = assign23630_e36741_d_n5;
        locals.var_eeffm_dn6 = assign23630_e36741_d_n6;
        locals.var_eeffm_dn7 = assign23630_e36741_d_n7;
        locals.var_eeffm_dn8 = assign23630_e36741_d_n8;
        locals.var_eeffm_dn9 = assign23630_e36741_d_n9;
        locals.var_eeffm_dn10 = assign23630_e36741_d_n10;
        locals.var_eeffm_dn11 = assign23630_e36741_d_n11;
        locals.var_eeffm_rv = 0.0;

        let (assign23640_e36760, assign23640_e36760_d_n3, assign23640_e36760_d_n4, assign23640_e36760_d_n5, assign23640_e36760_d_n6, assign23640_e36760_d_n7, assign23640_e36760_d_n8, assign23640_e36760_d_n9, assign23640_e36760_d_n10, assign23640_e36760_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23640_e36751: f64 = (locals.var_qim / locals.var_qbm);
        let assign23640_e36752: f64 = (1.0 + assign23640_e36751);
        let assign23640_e36753: f64 = (0.5 * assign23640_e36752);
        let assign23640_e36755: f64 = (assign23640_e36753).max(1e-38);
        let assign23640_e36756: f64 = (assign23640_e36755).ln();
        let assign23640_e36757: f64 = (locals.var_ucs_a * assign23640_e36756);
        let assign23640_e36758: f64 = { let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23640_e36758, ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn3 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn3 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn3)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn4 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn4 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn4)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn5 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn5 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn5)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn6 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn6 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn6)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn7 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn7 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn7)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn8 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn8 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn8)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn9 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn9 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn9)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn10 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn10 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn10)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))), ({ let limited_exp_arg = assign23640_e36757; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_ucs_a_dn11 * assign23640_e36756) + (locals.var_ucs_a * (if assign23640_e36753 >= 1e-38 { (0.5 * (((locals.var_qim_dn11 * locals.var_qbm) - (locals.var_qim * locals.var_qbm_dn11)) / (locals.var_qbm * locals.var_qbm))) } else { 0.0 } / assign23640_e36755)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign23640_e36760;
        locals.var_t2_dn3 = assign23640_e36760_d_n3;
        locals.var_t2_dn4 = assign23640_e36760_d_n4;
        locals.var_t2_dn5 = assign23640_e36760_d_n5;
        locals.var_t2_dn6 = assign23640_e36760_d_n6;
        locals.var_t2_dn7 = assign23640_e36760_d_n7;
        locals.var_t2_dn8 = assign23640_e36760_d_n8;
        locals.var_t2_dn9 = assign23640_e36760_d_n9;
        locals.var_t2_dn10 = assign23640_e36760_d_n10;
        locals.var_t2_dn11 = assign23640_e36760_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign23650_e36779, assign23650_e36779_d_n3, assign23650_e36779_d_n4, assign23650_e36779_d_n5, assign23650_e36779_d_n6, assign23650_e36779_d_n7, assign23650_e36779_d_n8, assign23650_e36779_d_n9, assign23650_e36779_d_n10, assign23650_e36779_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23650_e36768: f64 = (locals.var_uc_a * locals.var_vbsx);
        let assign23650_e36769: f64 = (locals.var_ua_a + assign23650_e36768);
        let assign23650_e36772: f64 = (locals.var_eeffm).powf(locals.var_eu_t);
        let assign23650_e36773: f64 = (assign23650_e36769 * assign23650_e36772);
        let assign23650_e36776: f64 = (locals.var_ud_a / locals.var_t2);
        let assign23650_e36777: f64 = (assign23650_e36773 + assign23650_e36776);
        (assign23650_e36777, ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn3))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn3 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn3)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn3 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn3 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn3 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn4))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn4 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn4)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn4 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn4 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn4 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn5))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn5 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn5)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn5 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn5 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn5 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn6))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn6 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn6)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn6 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn6 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn6 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn7))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn7 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn7)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn7 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn7 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn7 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn8))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn8 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn8)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn8 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn8 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn8 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn9))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn9 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn9)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn9 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn9 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn9 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn10))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn10 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn10)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn10 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn10 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn10 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_vbsx) + (locals.var_uc_a * locals.var_vbsx_dn11))) * assign23650_e36772) + (assign23650_e36769 * if locals.var_eu_t_dn11 == 0.0 && ((locals.var_eu_t) as f64).is_finite() && ((locals.var_eu_t) as f64).fract() == 0.0 { if locals.var_eu_t == 0.0 { 0.0 } else { (locals.var_eu_t * ((locals.var_eeffm).powf(locals.var_eu_t - 1.0) * locals.var_eeffm_dn11)) } } else { (assign23650_e36772 * ((locals.var_eu_t_dn11 * (locals.var_eeffm).ln()) + (locals.var_eu_t * (locals.var_eeffm_dn11 / locals.var_eeffm)))) })) + (((locals.var_ud_a_dn11 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign23650_e36779;
        locals.var_t3_dn3 = assign23650_e36779_d_n3;
        locals.var_t3_dn4 = assign23650_e36779_d_n4;
        locals.var_t3_dn5 = assign23650_e36779_d_n5;
        locals.var_t3_dn6 = assign23650_e36779_d_n6;
        locals.var_t3_dn7 = assign23650_e36779_d_n7;
        locals.var_t3_dn8 = assign23650_e36779_d_n8;
        locals.var_t3_dn9 = assign23650_e36779_d_n9;
        locals.var_t3_dn10 = assign23650_e36779_d_n10;
        locals.var_t3_dn11 = assign23650_e36779_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign23660_e36788, assign23660_e36788_d_n3, assign23660_e36788_d_n4, assign23660_e36788_d_n5, assign23660_e36788_d_n6, assign23660_e36788_d_n7, assign23660_e36788_d_n8, assign23660_e36788_d_n9, assign23660_e36788_d_n10, assign23660_e36788_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23660_e36786: f64 = (1.0 + locals.var_t3);
        (assign23660_e36786, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign23660_e36788;
        locals.var_t4_dn3 = assign23660_e36788_d_n3;
        locals.var_t4_dn4 = assign23660_e36788_d_n4;
        locals.var_t4_dn5 = assign23660_e36788_d_n5;
        locals.var_t4_dn6 = assign23660_e36788_d_n6;
        locals.var_t4_dn7 = assign23660_e36788_d_n7;
        locals.var_t4_dn8 = assign23660_e36788_d_n8;
        locals.var_t4_dn9 = assign23660_e36788_d_n9;
        locals.var_t4_dn10 = assign23660_e36788_d_n10;
        locals.var_t4_dn11 = assign23660_e36788_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign23670_e36814, assign23670_e36814_d_n3, assign23670_e36814_d_n4, assign23670_e36814_d_n5, assign23670_e36814_d_n6, assign23670_e36814_d_n7, assign23670_e36814_d_n8, assign23670_e36814_d_n9, assign23670_e36814_d_n10, assign23670_e36814_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23670_e36796: f64 = (locals.var_t4 + 1.0);
        let assign23670_e36799: f64 = (locals.var_t4 - 1.0);
        let assign23670_e36802: f64 = (locals.var_t4 - 1.0);
        let assign23670_e36803: f64 = (assign23670_e36799 * assign23670_e36802);
        let assign23670_e36806: f64 = (0.25 * 0.0015);
        let assign23670_e36808: f64 = (assign23670_e36806 * 0.0015);
        let assign23670_e36809: f64 = (assign23670_e36803 + assign23670_e36808);
        let assign23670_e36810: f64 = (assign23670_e36809).sqrt();
        let assign23670_e36811: f64 = (assign23670_e36796 + assign23670_e36810);
        let assign23670_e36812: f64 = (0.5 * assign23670_e36811);
        (assign23670_e36812, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn3)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn4)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn5)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn6)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn7)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn8)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn9 + (((locals.var_t4_dn9 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn9)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn10 + (((locals.var_t4_dn10 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn10)) / (2.0 * assign23670_e36810)))), (0.5 * (locals.var_t4_dn11 + (((locals.var_t4_dn11 * assign23670_e36802) + (assign23670_e36799 * locals.var_t4_dn11)) / (2.0 * assign23670_e36810)))),)
    } else {
        (locals.var_dmob, locals.var_dmob_dn3, locals.var_dmob_dn4, locals.var_dmob_dn5, locals.var_dmob_dn6, locals.var_dmob_dn7, locals.var_dmob_dn8, locals.var_dmob_dn9, locals.var_dmob_dn10, locals.var_dmob_dn11,)
    }
};
        locals.var_dmob = assign23670_e36814;
        locals.var_dmob_dn3 = assign23670_e36814_d_n3;
        locals.var_dmob_dn4 = assign23670_e36814_d_n4;
        locals.var_dmob_dn5 = assign23670_e36814_d_n5;
        locals.var_dmob_dn6 = assign23670_e36814_d_n6;
        locals.var_dmob_dn7 = assign23670_e36814_d_n7;
        locals.var_dmob_dn8 = assign23670_e36814_d_n8;
        locals.var_dmob_dn9 = assign23670_e36814_d_n9;
        locals.var_dmob_dn10 = assign23670_e36814_d_n10;
        locals.var_dmob_dn11 = assign23670_e36814_d_n11;
        locals.var_dmob_rv = 0.0;

        let (assign23680_e36827, assign23680_e36827_d_n3, assign23680_e36827_d_n4, assign23680_e36827_d_n5, assign23680_e36827_d_n6, assign23680_e36827_d_n7, assign23680_e36827_d_n8, assign23680_e36827_d_n9, assign23680_e36827_d_n10, assign23680_e36827_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23680_e36821: f64 = (2.0 * locals.var_vsat_a);
        let assign23680_e36824: f64 = (locals.var_u0_a / locals.var_dmob);
        let assign23680_e36825: f64 = (assign23680_e36821 / assign23680_e36824);
        (assign23680_e36825, ((((2.0 * locals.var_vsat_a_dn3) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn3 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn3)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn4) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn4 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn5) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn5 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn5)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn6) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn6 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn6)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn7) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn7 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn7)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn8) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn8 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn8)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn9) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn9 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn9)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn10) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn10 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn10)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)), ((((2.0 * locals.var_vsat_a_dn11) * assign23680_e36824) - (assign23680_e36821 * (((locals.var_u0_a_dn11 * locals.var_dmob) - (locals.var_u0_a * locals.var_dmob_dn11)) / (locals.var_dmob * locals.var_dmob)))) / (assign23680_e36824 * assign23680_e36824)),)
    } else {
        (locals.var_esat, locals.var_esat_dn3, locals.var_esat_dn4, locals.var_esat_dn5, locals.var_esat_dn6, locals.var_esat_dn7, locals.var_esat_dn8, locals.var_esat_dn9, locals.var_esat_dn10, locals.var_esat_dn11,)
    }
};
        locals.var_esat = assign23680_e36827;
        locals.var_esat_dn3 = assign23680_e36827_d_n3;
        locals.var_esat_dn4 = assign23680_e36827_d_n4;
        locals.var_esat_dn5 = assign23680_e36827_d_n5;
        locals.var_esat_dn6 = assign23680_e36827_d_n6;
        locals.var_esat_dn7 = assign23680_e36827_d_n7;
        locals.var_esat_dn8 = assign23680_e36827_d_n8;
        locals.var_esat_dn9 = assign23680_e36827_d_n9;
        locals.var_esat_dn10 = assign23680_e36827_d_n10;
        locals.var_esat_dn11 = assign23680_e36827_d_n11;
        locals.var_esat_rv = 0.0;

        let (assign23690_e36836, assign23690_e36836_d_n3, assign23690_e36836_d_n4, assign23690_e36836_d_n5, assign23690_e36836_d_n6, assign23690_e36836_d_n7, assign23690_e36836_d_n8, assign23690_e36836_d_n9, assign23690_e36836_d_n10, assign23690_e36836_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23690_e36834: f64 = (locals.var_esat * locals.var_leff);
        (assign23690_e36834, (locals.var_esat_dn3 * locals.var_leff), (locals.var_esat_dn4 * locals.var_leff), (locals.var_esat_dn5 * locals.var_leff), (locals.var_esat_dn6 * locals.var_leff), (locals.var_esat_dn7 * locals.var_leff), (locals.var_esat_dn8 * locals.var_leff), (locals.var_esat_dn9 * locals.var_leff), (locals.var_esat_dn10 * locals.var_leff), (locals.var_esat_dn11 * locals.var_leff),)
    } else {
        (locals.var_esatl, locals.var_esatl_dn3, locals.var_esatl_dn4, locals.var_esatl_dn5, locals.var_esatl_dn6, locals.var_esatl_dn7, locals.var_esatl_dn8, locals.var_esatl_dn9, locals.var_esatl_dn10, locals.var_esatl_dn11,)
    }
};
        locals.var_esatl = assign23690_e36836;
        locals.var_esatl_dn3 = assign23690_e36836_d_n3;
        locals.var_esatl_dn4 = assign23690_e36836_d_n4;
        locals.var_esatl_dn5 = assign23690_e36836_d_n5;
        locals.var_esatl_dn6 = assign23690_e36836_d_n6;
        locals.var_esatl_dn7 = assign23690_e36836_d_n7;
        locals.var_esatl_dn8 = assign23690_e36836_d_n8;
        locals.var_esatl_dn9 = assign23690_e36836_d_n9;
        locals.var_esatl_dn10 = assign23690_e36836_d_n10;
        locals.var_esatl_dn11 = assign23690_e36836_d_n11;
        locals.var_esatl_rv = 0.0;

        let assign23700_e36839: f64 = if locals.var_pvag_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard525 = assign23700_e36839;
        locals.var_guard525_rv = 0.0;

        let (assign23710_e36854, assign23710_e36854_d_n3, assign23710_e36854_d_n4, assign23710_e36854_d_n5, assign23710_e36854_d_n6, assign23710_e36854_d_n7, assign23710_e36854_d_n8, assign23710_e36854_d_n9, assign23710_e36854_d_n10, assign23710_e36854_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard525 != 0.0)) {
        let assign23710_e36849: f64 = (locals.var_pvag_i * locals.var_qim);
        let assign23710_e36851: f64 = (assign23710_e36849 / locals.var_esatl);
        let assign23710_e36852: f64 = (1.0 + assign23710_e36851);
        (assign23710_e36852, ((((locals.var_pvag_i * locals.var_qim_dn3) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn4) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn5) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn6) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn7) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn8) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn9) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn10) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qim_dn11) * locals.var_esatl) - (assign23710_e36849 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_pvagfactor, locals.var_pvagfactor_dn3, locals.var_pvagfactor_dn4, locals.var_pvagfactor_dn5, locals.var_pvagfactor_dn6, locals.var_pvagfactor_dn7, locals.var_pvagfactor_dn8, locals.var_pvagfactor_dn9, locals.var_pvagfactor_dn10, locals.var_pvagfactor_dn11,)
    }
};
        locals.var_pvagfactor = assign23710_e36854;
        locals.var_pvagfactor_dn3 = assign23710_e36854_d_n3;
        locals.var_pvagfactor_dn4 = assign23710_e36854_d_n4;
        locals.var_pvagfactor_dn5 = assign23710_e36854_d_n5;
        locals.var_pvagfactor_dn6 = assign23710_e36854_d_n6;
        locals.var_pvagfactor_dn7 = assign23710_e36854_d_n7;
        locals.var_pvagfactor_dn8 = assign23710_e36854_d_n8;
        locals.var_pvagfactor_dn9 = assign23710_e36854_d_n9;
        locals.var_pvagfactor_dn10 = assign23710_e36854_d_n10;
        locals.var_pvagfactor_dn11 = assign23710_e36854_d_n11;
        locals.var_pvagfactor_rv = 0.0;

        let (assign23720_e36872, assign23720_e36872_d_n3, assign23720_e36872_d_n4, assign23720_e36872_d_n5, assign23720_e36872_d_n6, assign23720_e36872_d_n7, assign23720_e36872_d_n8, assign23720_e36872_d_n9, assign23720_e36872_d_n10, assign23720_e36872_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard525 == 0.0)) {
        let assign23720_e36866: f64 = (locals.var_pvag_i * locals.var_qim);
        let assign23720_e36868: f64 = (assign23720_e36866 / locals.var_esatl);
        let assign23720_e36869: f64 = (1.0 - assign23720_e36868);
        let assign23720_e36870: f64 = (1.0 / assign23720_e36869);
        (assign23720_e36870, (-((-((((locals.var_pvag_i * locals.var_qim_dn3) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn4) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn5) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn6) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn7) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn8) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn9) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn10) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))), (-((-((((locals.var_pvag_i * locals.var_qim_dn11) * locals.var_esatl) - (assign23720_e36866 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl))) / (assign23720_e36869 * assign23720_e36869))),)
    } else {
        (locals.var_pvagfactor, locals.var_pvagfactor_dn3, locals.var_pvagfactor_dn4, locals.var_pvagfactor_dn5, locals.var_pvagfactor_dn6, locals.var_pvagfactor_dn7, locals.var_pvagfactor_dn8, locals.var_pvagfactor_dn9, locals.var_pvagfactor_dn10, locals.var_pvagfactor_dn11,)
    }
};
        locals.var_pvagfactor = assign23720_e36872;
        locals.var_pvagfactor_dn3 = assign23720_e36872_d_n3;
        locals.var_pvagfactor_dn4 = assign23720_e36872_d_n4;
        locals.var_pvagfactor_dn5 = assign23720_e36872_d_n5;
        locals.var_pvagfactor_dn6 = assign23720_e36872_d_n6;
        locals.var_pvagfactor_dn7 = assign23720_e36872_d_n7;
        locals.var_pvagfactor_dn8 = assign23720_e36872_d_n8;
        locals.var_pvagfactor_dn9 = assign23720_e36872_d_n9;
        locals.var_pvagfactor_dn10 = assign23720_e36872_d_n10;
        locals.var_pvagfactor_dn11 = assign23720_e36872_d_n11;
        locals.var_pvagfactor_rv = 0.0;

        let (assign23730_e36879, assign23730_e36879_d_n3, assign23730_e36879_d_n4, assign23730_e36879_d_n5, assign23730_e36879_d_n6, assign23730_e36879_d_n7, assign23730_e36879_d_n8, assign23730_e36879_d_n9, assign23730_e36879_d_n10, assign23730_e36879_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        (locals.var_pdiblc_a, locals.var_pdiblc_a_dn3, locals.var_pdiblc_a_dn4, locals.var_pdiblc_a_dn5, locals.var_pdiblc_a_dn6, locals.var_pdiblc_a_dn7, locals.var_pdiblc_a_dn8, locals.var_pdiblc_a_dn9, locals.var_pdiblc_a_dn10, locals.var_pdiblc_a_dn11,)
    } else {
        (locals.var_diblfactor, locals.var_diblfactor_dn3, locals.var_diblfactor_dn4, locals.var_diblfactor_dn5, locals.var_diblfactor_dn6, locals.var_diblfactor_dn7, locals.var_diblfactor_dn8, locals.var_diblfactor_dn9, locals.var_diblfactor_dn10, locals.var_diblfactor_dn11,)
    }
};
        locals.var_diblfactor = assign23730_e36879;
        locals.var_diblfactor_dn3 = assign23730_e36879_d_n3;
        locals.var_diblfactor_dn4 = assign23730_e36879_d_n4;
        locals.var_diblfactor_dn5 = assign23730_e36879_d_n5;
        locals.var_diblfactor_dn6 = assign23730_e36879_d_n6;
        locals.var_diblfactor_dn7 = assign23730_e36879_d_n7;
        locals.var_diblfactor_dn8 = assign23730_e36879_d_n8;
        locals.var_diblfactor_dn9 = assign23730_e36879_d_n9;
        locals.var_diblfactor_dn10 = assign23730_e36879_d_n10;
        locals.var_diblfactor_dn11 = assign23730_e36879_d_n11;
        locals.var_diblfactor_rv = 0.0;

        let (assign23740_e36888, assign23740_e36888_d_n3, assign23740_e36888_d_n4, assign23740_e36888_d_n5, assign23740_e36888_d_n6, assign23740_e36888_d_n7, assign23740_e36888_d_n8, assign23740_e36888_d_n9, assign23740_e36888_d_n10, assign23740_e36888_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23740_e36886: f64 = (locals.var_vds - locals.var_vdseff);
        (assign23740_e36886, (-locals.var_vdseff_dn3), (-locals.var_vdseff_dn4), (-locals.var_vdseff_dn5), (locals.var_vds_dn6 - locals.var_vdseff_dn6), (locals.var_vds_dn7 - locals.var_vdseff_dn7), (-locals.var_vdseff_dn8), (-locals.var_vdseff_dn9), (locals.var_vds_dn10 - locals.var_vdseff_dn10), (-locals.var_vdseff_dn11),)
    } else {
        (locals.var_diffvds, locals.var_diffvds_dn3, locals.var_diffvds_dn4, locals.var_diffvds_dn5, locals.var_diffvds_dn6, locals.var_diffvds_dn7, locals.var_diffvds_dn8, locals.var_diffvds_dn9, locals.var_diffvds_dn10, locals.var_diffvds_dn11,)
    }
};
        locals.var_diffvds = assign23740_e36888;
        locals.var_diffvds_dn3 = assign23740_e36888_d_n3;
        locals.var_diffvds_dn4 = assign23740_e36888_d_n4;
        locals.var_diffvds_dn5 = assign23740_e36888_d_n5;
        locals.var_diffvds_dn6 = assign23740_e36888_d_n6;
        locals.var_diffvds_dn7 = assign23740_e36888_d_n7;
        locals.var_diffvds_dn8 = assign23740_e36888_d_n8;
        locals.var_diffvds_dn9 = assign23740_e36888_d_n9;
        locals.var_diffvds_dn10 = assign23740_e36888_d_n10;
        locals.var_diffvds_dn11 = assign23740_e36888_d_n11;
        locals.var_diffvds_rv = 0.0;

        let (assign23750_e36899, assign23750_e36899_d_n3, assign23750_e36899_d_n4, assign23750_e36899_d_n5, assign23750_e36899_d_n6, assign23750_e36899_d_n7, assign23750_e36899_d_n8, assign23750_e36899_d_n9, assign23750_e36899_d_n10, assign23750_e36899_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23750_e36896: f64 = (2.0 * locals.var_nvt);
        let assign23750_e36897: f64 = (locals.var_qim + assign23750_e36896);
        (assign23750_e36897, (locals.var_qim_dn3 + (2.0 * locals.var_nvt_dn3)), (locals.var_qim_dn4 + (2.0 * locals.var_nvt_dn4)), (locals.var_qim_dn5 + (2.0 * locals.var_nvt_dn5)), (locals.var_qim_dn6 + (2.0 * locals.var_nvt_dn6)), (locals.var_qim_dn7 + (2.0 * locals.var_nvt_dn7)), (locals.var_qim_dn8 + (2.0 * locals.var_nvt_dn8)), (locals.var_qim_dn9 + (2.0 * locals.var_nvt_dn9)), (locals.var_qim_dn10 + (2.0 * locals.var_nvt_dn10)), (locals.var_qim_dn11 + (2.0 * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vgst2vtm, locals.var_vgst2vtm_dn3, locals.var_vgst2vtm_dn4, locals.var_vgst2vtm_dn5, locals.var_vgst2vtm_dn6, locals.var_vgst2vtm_dn7, locals.var_vgst2vtm_dn8, locals.var_vgst2vtm_dn9, locals.var_vgst2vtm_dn10, locals.var_vgst2vtm_dn11,)
    }
};
        locals.var_vgst2vtm = assign23750_e36899;
        locals.var_vgst2vtm_dn3 = assign23750_e36899_d_n3;
        locals.var_vgst2vtm_dn4 = assign23750_e36899_d_n4;
        locals.var_vgst2vtm_dn5 = assign23750_e36899_d_n5;
        locals.var_vgst2vtm_dn6 = assign23750_e36899_d_n6;
        locals.var_vgst2vtm_dn7 = assign23750_e36899_d_n7;
        locals.var_vgst2vtm_dn8 = assign23750_e36899_d_n8;
        locals.var_vgst2vtm_dn9 = assign23750_e36899_d_n9;
        locals.var_vgst2vtm_dn10 = assign23750_e36899_d_n10;
        locals.var_vgst2vtm_dn11 = assign23750_e36899_d_n11;
        locals.var_vgst2vtm_rv = 0.0;

        let assign23760_e36902: f64 = if locals.var_diblfactor > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard526 = assign23760_e36902;
        locals.var_guard526_rv = 0.0;

        let (assign23770_e36915, assign23770_e36915_d_n3, assign23770_e36915_d_n4, assign23770_e36915_d_n5, assign23770_e36915_d_n6, assign23770_e36915_d_n7, assign23770_e36915_d_n8, assign23770_e36915_d_n9, assign23770_e36915_d_n10, assign23770_e36915_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign23770_e36912: f64 = (locals.var_vdsat + locals.var_vgst2vtm);
        let assign23770_e36913: f64 = (locals.var_vgst2vtm / assign23770_e36912);
        (assign23770_e36913, (((locals.var_vgst2vtm_dn3 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn3 + locals.var_vgst2vtm_dn3))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn4 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn4 + locals.var_vgst2vtm_dn4))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn5 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn5 + locals.var_vgst2vtm_dn5))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn6 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn6 + locals.var_vgst2vtm_dn6))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn7 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn7 + locals.var_vgst2vtm_dn7))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn8 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn8 + locals.var_vgst2vtm_dn8))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn9 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn9 + locals.var_vgst2vtm_dn9))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn10 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn10 + locals.var_vgst2vtm_dn10))) / (assign23770_e36912 * assign23770_e36912)), (((locals.var_vgst2vtm_dn11 * assign23770_e36912) - (locals.var_vgst2vtm * (locals.var_vdsat_dn11 + locals.var_vgst2vtm_dn11))) / (assign23770_e36912 * assign23770_e36912)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign23770_e36915;
        locals.var_t3_dn3 = assign23770_e36915_d_n3;
        locals.var_t3_dn4 = assign23770_e36915_d_n4;
        locals.var_t3_dn5 = assign23770_e36915_d_n5;
        locals.var_t3_dn6 = assign23770_e36915_d_n6;
        locals.var_t3_dn7 = assign23770_e36915_d_n7;
        locals.var_t3_dn8 = assign23770_e36915_d_n8;
        locals.var_t3_dn9 = assign23770_e36915_d_n9;
        locals.var_t3_dn10 = assign23770_e36915_d_n10;
        locals.var_t3_dn11 = assign23770_e36915_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign23780_e36949, assign23780_e36949_d_n3, assign23780_e36949_d_n4, assign23780_e36949_d_n5, assign23780_e36949_d_n6, assign23780_e36949_d_n7, assign23780_e36949_d_n8, assign23780_e36949_d_n9, assign23780_e36949_d_n10, assign23780_e36949_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign23780_e36926: f64 = (locals.var_pdiblcb_i * locals.var_vbsx);
        let assign23780_e36927: f64 = (1.0 + assign23780_e36926);
        let assign23780_e36931: f64 = (locals.var_pdiblcb_i * locals.var_vbsx);
        let assign23780_e36932: f64 = (1.0 + assign23780_e36931);
        let assign23780_e36936: f64 = (locals.var_pdiblcb_i * locals.var_vbsx);
        let assign23780_e36937: f64 = (1.0 + assign23780_e36936);
        let assign23780_e36938: f64 = (assign23780_e36932 * assign23780_e36937);
        let assign23780_e36941: f64 = (4.0 * 0.001);
        let assign23780_e36943: f64 = (assign23780_e36941 * 0.001);
        let assign23780_e36944: f64 = (assign23780_e36938 + assign23780_e36943);
        let assign23780_e36945: f64 = (assign23780_e36944).sqrt();
        let assign23780_e36946: f64 = (assign23780_e36927 + assign23780_e36945);
        let assign23780_e36947: f64 = (0.5 * assign23780_e36946);
        (assign23780_e36947, (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn3) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn3) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn3))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn4) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn4) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn4))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn5) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn5) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn5))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn6) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn6) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn6))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn7) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn7) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn7))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn8) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn8) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn8))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn9) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn9) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn9))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn10) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn10) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn10))) / (2.0 * assign23780_e36945)))), (0.5 * ((locals.var_pdiblcb_i * locals.var_vbsx_dn11) + ((((locals.var_pdiblcb_i * locals.var_vbsx_dn11) * assign23780_e36937) + (assign23780_e36932 * (locals.var_pdiblcb_i * locals.var_vbsx_dn11))) / (2.0 * assign23780_e36945)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign23780_e36949;
        locals.var_t4_dn3 = assign23780_e36949_d_n3;
        locals.var_t4_dn4 = assign23780_e36949_d_n4;
        locals.var_t4_dn5 = assign23780_e36949_d_n5;
        locals.var_t4_dn6 = assign23780_e36949_d_n6;
        locals.var_t4_dn7 = assign23780_e36949_d_n7;
        locals.var_t4_dn8 = assign23780_e36949_d_n8;
        locals.var_t4_dn9 = assign23780_e36949_d_n9;
        locals.var_t4_dn10 = assign23780_e36949_d_n10;
        locals.var_t4_dn11 = assign23780_e36949_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign23790_e36960, assign23790_e36960_d_n3, assign23790_e36960_d_n4, assign23790_e36960_d_n5, assign23790_e36960_d_n6, assign23790_e36960_d_n7, assign23790_e36960_d_n8, assign23790_e36960_d_n9, assign23790_e36960_d_n10, assign23790_e36960_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign23790_e36958: f64 = (1.0 / locals.var_t4);
        (assign23790_e36958, (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign23790_e36960;
        locals.var_t5_dn3 = assign23790_e36960_d_n3;
        locals.var_t5_dn4 = assign23790_e36960_d_n4;
        locals.var_t5_dn5 = assign23790_e36960_d_n5;
        locals.var_t5_dn6 = assign23790_e36960_d_n6;
        locals.var_t5_dn7 = assign23790_e36960_d_n7;
        locals.var_t5_dn8 = assign23790_e36960_d_n8;
        locals.var_t5_dn9 = assign23790_e36960_d_n9;
        locals.var_t5_dn10 = assign23790_e36960_d_n10;
        locals.var_t5_dn11 = assign23790_e36960_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23800_e36977, assign23800_e36977_d_n3, assign23800_e36977_d_n4, assign23800_e36977_d_n5, assign23800_e36977_d_n6, assign23800_e36977_d_n7, assign23800_e36977_d_n8, assign23800_e36977_d_n9, assign23800_e36977_d_n10, assign23800_e36977_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign23800_e36969: f64 = (locals.var_vgst2vtm / locals.var_diblfactor);
        let assign23800_e36971: f64 = (assign23800_e36969 * locals.var_t3);
        let assign23800_e36973: f64 = (assign23800_e36971 * locals.var_pvagfactor);
        let assign23800_e36975: f64 = (assign23800_e36973 * locals.var_t5);
        (assign23800_e36975, (((((((((locals.var_vgst2vtm_dn3 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn3)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn3)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn3)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn3)), (((((((((locals.var_vgst2vtm_dn4 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn4)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn4)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn4)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn4)), (((((((((locals.var_vgst2vtm_dn5 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn5)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn5)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn5)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn5)), (((((((((locals.var_vgst2vtm_dn6 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn6)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn6)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn6)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn6)), (((((((((locals.var_vgst2vtm_dn7 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn7)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn7)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn7)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn7)), (((((((((locals.var_vgst2vtm_dn8 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn8)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn8)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn8)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn8)), (((((((((locals.var_vgst2vtm_dn9 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn9)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn9)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn9)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn9)), (((((((((locals.var_vgst2vtm_dn10 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn10)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn10)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn10)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn10)), (((((((((locals.var_vgst2vtm_dn11 * locals.var_diblfactor) - (locals.var_vgst2vtm * locals.var_diblfactor_dn11)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign23800_e36969 * locals.var_t3_dn11)) * locals.var_pvagfactor) + (assign23800_e36971 * locals.var_pvagfactor_dn11)) * locals.var_t5) + (assign23800_e36973 * locals.var_t5_dn11)),)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8, locals.var_vadibl_dn9, locals.var_vadibl_dn10, locals.var_vadibl_dn11,)
    }
};
        locals.var_vadibl = assign23800_e36977;
        locals.var_vadibl_dn3 = assign23800_e36977_d_n3;
        locals.var_vadibl_dn4 = assign23800_e36977_d_n4;
        locals.var_vadibl_dn5 = assign23800_e36977_d_n5;
        locals.var_vadibl_dn6 = assign23800_e36977_d_n6;
        locals.var_vadibl_dn7 = assign23800_e36977_d_n7;
        locals.var_vadibl_dn8 = assign23800_e36977_d_n8;
        locals.var_vadibl_dn9 = assign23800_e36977_d_n9;
        locals.var_vadibl_dn10 = assign23800_e36977_d_n10;
        locals.var_vadibl_dn11 = assign23800_e36977_d_n11;
        locals.var_vadibl_rv = 0.0;

        let (assign23810_e36990, assign23810_e36990_d_n3, assign23810_e36990_d_n4, assign23810_e36990_d_n5, assign23810_e36990_d_n6, assign23810_e36990_d_n7, assign23810_e36990_d_n8, assign23810_e36990_d_n9, assign23810_e36990_d_n10, assign23810_e36990_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard526 != 0.0)) {
        let assign23810_e36987: f64 = (locals.var_diffvds / locals.var_vadibl);
        let assign23810_e36988: f64 = (1.0 + assign23810_e36987);
        (assign23810_e36988, (((locals.var_diffvds_dn3 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn3)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn4 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn4)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn5 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn5)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn6 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn6)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn7 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn7)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn8 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn8)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn9 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn9)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn10 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn10)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn11 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn11)) / (locals.var_vadibl * locals.var_vadibl)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign23810_e36990;
        locals.var_moc_dn3 = assign23810_e36990_d_n3;
        locals.var_moc_dn4 = assign23810_e36990_d_n4;
        locals.var_moc_dn5 = assign23810_e36990_d_n5;
        locals.var_moc_dn6 = assign23810_e36990_d_n6;
        locals.var_moc_dn7 = assign23810_e36990_d_n7;
        locals.var_moc_dn8 = assign23810_e36990_d_n8;
        locals.var_moc_dn9 = assign23810_e36990_d_n9;
        locals.var_moc_dn10 = assign23810_e36990_d_n10;
        locals.var_moc_dn11 = assign23810_e36990_d_n11;
        locals.var_moc_rv = 0.0;

        let (assign23820_e37000, assign23820_e37000_d_n3, assign23820_e37000_d_n4, assign23820_e37000_d_n5, assign23820_e37000_d_n6, assign23820_e37000_d_n7, assign23820_e37000_d_n8, assign23820_e37000_d_n9, assign23820_e37000_d_n10, assign23820_e37000_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard526 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign23820_e37000;
        locals.var_moc_dn3 = assign23820_e37000_d_n3;
        locals.var_moc_dn4 = assign23820_e37000_d_n4;
        locals.var_moc_dn5 = assign23820_e37000_d_n5;
        locals.var_moc_dn6 = assign23820_e37000_d_n6;
        locals.var_moc_dn7 = assign23820_e37000_d_n7;
        locals.var_moc_dn8 = assign23820_e37000_d_n8;
        locals.var_moc_dn9 = assign23820_e37000_d_n9;
        locals.var_moc_dn10 = assign23820_e37000_d_n10;
        locals.var_moc_dn11 = assign23820_e37000_d_n11;
        locals.var_moc_rv = 0.0;

        let assign23830_e37003: f64 = if locals.var_fprout_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard527 = assign23830_e37003;
        locals.var_guard527_rv = 0.0;

        let (assign23840_e37012, assign23840_e37012_d_n3, assign23840_e37012_d_n4, assign23840_e37012_d_n5, assign23840_e37012_d_n6, assign23840_e37012_d_n7, assign23840_e37012_d_n8, assign23840_e37012_d_n9, assign23840_e37012_d_n10, assign23840_e37012_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard527 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fp, locals.var_fp_dn3, locals.var_fp_dn4, locals.var_fp_dn5, locals.var_fp_dn6, locals.var_fp_dn7, locals.var_fp_dn8, locals.var_fp_dn9, locals.var_fp_dn10, locals.var_fp_dn11,)
    }
};
        locals.var_fp = assign23840_e37012;
        locals.var_fp_dn3 = assign23840_e37012_d_n3;
        locals.var_fp_dn4 = assign23840_e37012_d_n4;
        locals.var_fp_dn5 = assign23840_e37012_d_n5;
        locals.var_fp_dn6 = assign23840_e37012_d_n6;
        locals.var_fp_dn7 = assign23840_e37012_d_n7;
        locals.var_fp_dn8 = assign23840_e37012_d_n8;
        locals.var_fp_dn9 = assign23840_e37012_d_n9;
        locals.var_fp_dn10 = assign23840_e37012_d_n10;
        locals.var_fp_dn11 = assign23840_e37012_d_n11;
        locals.var_fp_rv = 0.0;

        let (assign23850_e37027, assign23850_e37027_d_n3, assign23850_e37027_d_n4, assign23850_e37027_d_n5, assign23850_e37027_d_n6, assign23850_e37027_d_n7, assign23850_e37027_d_n8, assign23850_e37027_d_n9, assign23850_e37027_d_n10, assign23850_e37027_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard527 == 0.0)) {
        let assign23850_e37022: f64 = (locals.var_leff).sqrt();
        let assign23850_e37023: f64 = (locals.var_fprout_i * assign23850_e37022);
        let assign23850_e37025: f64 = (assign23850_e37023 / locals.var_vgst2vtm);
        (assign23850_e37025, (-((assign23850_e37023 * locals.var_vgst2vtm_dn3) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn4) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn5) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn6) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn7) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn8) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn9) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn10) / (locals.var_vgst2vtm * locals.var_vgst2vtm))), (-((assign23850_e37023 * locals.var_vgst2vtm_dn11) / (locals.var_vgst2vtm * locals.var_vgst2vtm))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign23850_e37027;
        locals.var_t9_dn3 = assign23850_e37027_d_n3;
        locals.var_t9_dn4 = assign23850_e37027_d_n4;
        locals.var_t9_dn5 = assign23850_e37027_d_n5;
        locals.var_t9_dn6 = assign23850_e37027_d_n6;
        locals.var_t9_dn7 = assign23850_e37027_d_n7;
        locals.var_t9_dn8 = assign23850_e37027_d_n8;
        locals.var_t9_dn9 = assign23850_e37027_d_n9;
        locals.var_t9_dn10 = assign23850_e37027_d_n10;
        locals.var_t9_dn11 = assign23850_e37027_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign23860_e37041, assign23860_e37041_d_n3, assign23860_e37041_d_n4, assign23860_e37041_d_n5, assign23860_e37041_d_n6, assign23860_e37041_d_n7, assign23860_e37041_d_n8, assign23860_e37041_d_n9, assign23860_e37041_d_n10, assign23860_e37041_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard527 == 0.0)) {
        let assign23860_e37038: f64 = (1.0 + locals.var_t9);
        let assign23860_e37039: f64 = (1.0 / assign23860_e37038);
        (assign23860_e37039, (-(locals.var_t9_dn3 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn4 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn5 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn6 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn7 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn8 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn9 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn10 / (assign23860_e37038 * assign23860_e37038))), (-(locals.var_t9_dn11 / (assign23860_e37038 * assign23860_e37038))),)
    } else {
        (locals.var_fp, locals.var_fp_dn3, locals.var_fp_dn4, locals.var_fp_dn5, locals.var_fp_dn6, locals.var_fp_dn7, locals.var_fp_dn8, locals.var_fp_dn9, locals.var_fp_dn10, locals.var_fp_dn11,)
    }
};
        locals.var_fp = assign23860_e37041;
        locals.var_fp_dn3 = assign23860_e37041_d_n3;
        locals.var_fp_dn4 = assign23860_e37041_d_n4;
        locals.var_fp_dn5 = assign23860_e37041_d_n5;
        locals.var_fp_dn6 = assign23860_e37041_d_n6;
        locals.var_fp_dn7 = assign23860_e37041_d_n7;
        locals.var_fp_dn8 = assign23860_e37041_d_n8;
        locals.var_fp_dn9 = assign23860_e37041_d_n9;
        locals.var_fp_dn10 = assign23860_e37041_d_n10;
        locals.var_fp_dn11 = assign23860_e37041_d_n11;
        locals.var_fp_rv = 0.0;

        let (assign23870_e37050, assign23870_e37050_d_n3, assign23870_e37050_d_n4, assign23870_e37050_d_n5, assign23870_e37050_d_n6, assign23870_e37050_d_n7, assign23870_e37050_d_n8, assign23870_e37050_d_n9, assign23870_e37050_d_n10, assign23870_e37050_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23870_e37048: f64 = (locals.var_vdsat + locals.var_esatl);
        (assign23870_e37048, (locals.var_vdsat_dn3 + locals.var_esatl_dn3), (locals.var_vdsat_dn4 + locals.var_esatl_dn4), (locals.var_vdsat_dn5 + locals.var_esatl_dn5), (locals.var_vdsat_dn6 + locals.var_esatl_dn6), (locals.var_vdsat_dn7 + locals.var_esatl_dn7), (locals.var_vdsat_dn8 + locals.var_esatl_dn8), (locals.var_vdsat_dn9 + locals.var_esatl_dn9), (locals.var_vdsat_dn10 + locals.var_esatl_dn10), (locals.var_vdsat_dn11 + locals.var_esatl_dn11),)
    } else {
        (locals.var_vasat, locals.var_vasat_dn3, locals.var_vasat_dn4, locals.var_vasat_dn5, locals.var_vasat_dn6, locals.var_vasat_dn7, locals.var_vasat_dn8, locals.var_vasat_dn9, locals.var_vasat_dn10, locals.var_vasat_dn11,)
    }
};
        locals.var_vasat = assign23870_e37050;
        locals.var_vasat_dn3 = assign23870_e37050_d_n3;
        locals.var_vasat_dn4 = assign23870_e37050_d_n4;
        locals.var_vasat_dn5 = assign23870_e37050_d_n5;
        locals.var_vasat_dn6 = assign23870_e37050_d_n6;
        locals.var_vasat_dn7 = assign23870_e37050_d_n7;
        locals.var_vasat_dn8 = assign23870_e37050_d_n8;
        locals.var_vasat_dn9 = assign23870_e37050_d_n9;
        locals.var_vasat_dn10 = assign23870_e37050_d_n10;
        locals.var_vasat_dn11 = assign23870_e37050_d_n11;
        locals.var_vasat_rv = 0.0;

        let assign23880_e37053: f64 = if locals.var_pclm_a > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard528 = assign23880_e37053;
        locals.var_guard528_rv = 0.0;

        let assign23890_e37056: f64 = if p.p414 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard529 = assign23890_e37056;
        locals.var_guard529_rv = 0.0;

        let (assign23900_e37077, assign23900_e37077_d_n3, assign23900_e37077_d_n4, assign23900_e37077_d_n5, assign23900_e37077_d_n6, assign23900_e37077_d_n7, assign23900_e37077_d_n8, assign23900_e37077_d_n9, assign23900_e37077_d_n10, assign23900_e37077_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 != 0.0)) {
        let assign23900_e37069: f64 = (p.p414 * locals.var_qim);
        let assign23900_e37071: f64 = (assign23900_e37069 / locals.var_esatl);
        let assign23900_e37072: f64 = (1.0 - assign23900_e37071);
        let assign23900_e37073: f64 = (locals.var_pclm_a / assign23900_e37072);
        let assign23900_e37075: f64 = (assign23900_e37073 / locals.var_fp);
        (assign23900_e37075, ((((((locals.var_pclm_a_dn3 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn3) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn4 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn4) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn5 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn5) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn6 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn6) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn7 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn7) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn8 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn8) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn9 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn9) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn10 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn10) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn11 * assign23900_e37072) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn11) * locals.var_esatl) - (assign23900_e37069 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl))))) / (assign23900_e37072 * assign23900_e37072)) * locals.var_fp) - (assign23900_e37073 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign23900_e37077;
        locals.var_t1_dn3 = assign23900_e37077_d_n3;
        locals.var_t1_dn4 = assign23900_e37077_d_n4;
        locals.var_t1_dn5 = assign23900_e37077_d_n5;
        locals.var_t1_dn6 = assign23900_e37077_d_n6;
        locals.var_t1_dn7 = assign23900_e37077_d_n7;
        locals.var_t1_dn8 = assign23900_e37077_d_n8;
        locals.var_t1_dn9 = assign23900_e37077_d_n9;
        locals.var_t1_dn10 = assign23900_e37077_d_n10;
        locals.var_t1_dn11 = assign23900_e37077_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign23910_e37099, assign23910_e37099_d_n3, assign23910_e37099_d_n4, assign23910_e37099_d_n5, assign23910_e37099_d_n6, assign23910_e37099_d_n7, assign23910_e37099_d_n8, assign23910_e37099_d_n9, assign23910_e37099_d_n10, assign23910_e37099_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard528 != 0.0)) && (locals.var_guard529 == 0.0)) {
        let assign23910_e37091: f64 = (p.p414 * locals.var_qim);
        let assign23910_e37093: f64 = (assign23910_e37091 / locals.var_esatl);
        let assign23910_e37094: f64 = (1.0 + assign23910_e37093);
        let assign23910_e37095: f64 = (locals.var_pclm_a * assign23910_e37094);
        let assign23910_e37097: f64 = (assign23910_e37095 / locals.var_fp);
        (assign23910_e37097, (((((locals.var_pclm_a_dn3 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn3) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn4 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn4) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn5 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn5) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn6 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn6) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn7 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn7) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn8 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn8) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn9 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn9) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn10 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn10) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn11 * assign23910_e37094) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn11) * locals.var_esatl) - (assign23910_e37091 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23910_e37095 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign23910_e37099;
        locals.var_t1_dn3 = assign23910_e37099_d_n3;
        locals.var_t1_dn4 = assign23910_e37099_d_n4;
        locals.var_t1_dn5 = assign23910_e37099_d_n5;
        locals.var_t1_dn6 = assign23910_e37099_d_n6;
        locals.var_t1_dn7 = assign23910_e37099_d_n7;
        locals.var_t1_dn8 = assign23910_e37099_d_n8;
        locals.var_t1_dn9 = assign23910_e37099_d_n9;
        locals.var_t1_dn10 = assign23910_e37099_d_n10;
        locals.var_t1_dn11 = assign23910_e37099_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign23920_e37121, assign23920_e37121_d_n3, assign23920_e37121_d_n4, assign23920_e37121_d_n5, assign23920_e37121_d_n6, assign23920_e37121_d_n7, assign23920_e37121_d_n8, assign23920_e37121_d_n9, assign23920_e37121_d_n10, assign23920_e37121_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard528 != 0.0)) {
        let assign23920_e37111: f64 = (locals.var_diffvds / locals.var_t1);
        let assign23920_e37113: f64 = (assign23920_e37111 / locals.var_vasat);
        let assign23920_e37114: f64 = (1.0 + assign23920_e37113);
        let assign23920_e37116: f64 = (assign23920_e37114).max(1e-38);
        let assign23920_e37117: f64 = (assign23920_e37116).ln();
        let assign23920_e37118: f64 = (locals.var_t1 * assign23920_e37117);
        let assign23920_e37119: f64 = (1.0 + assign23920_e37118);
        (assign23920_e37119, ((locals.var_t1_dn3 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn3 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn3)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn4 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn4 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn4)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn5 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn5 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn5)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn6 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn6 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn6)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn7 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn7 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn7)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn8 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn8 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn8)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn9 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn9 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn9)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn10 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn10 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn10)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))), ((locals.var_t1_dn11 * assign23920_e37117) + (locals.var_t1 * (if assign23920_e37114 >= 1e-38 { ((((((locals.var_diffvds_dn11 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)) * locals.var_vasat) - (assign23920_e37111 * locals.var_vasat_dn11)) / (locals.var_vasat * locals.var_vasat)) } else { 0.0 } / assign23920_e37116))),)
    } else {
        (locals.var_mdl, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11,)
    }
};
        locals.var_mdl = assign23920_e37121;
        locals.var_mdl_dn3 = assign23920_e37121_d_n3;
        locals.var_mdl_dn4 = assign23920_e37121_d_n4;
        locals.var_mdl_dn5 = assign23920_e37121_d_n5;
        locals.var_mdl_dn6 = assign23920_e37121_d_n6;
        locals.var_mdl_dn7 = assign23920_e37121_d_n7;
        locals.var_mdl_dn8 = assign23920_e37121_d_n8;
        locals.var_mdl_dn9 = assign23920_e37121_d_n9;
        locals.var_mdl_dn10 = assign23920_e37121_d_n10;
        locals.var_mdl_dn11 = assign23920_e37121_d_n11;
        locals.var_mdl_rv = 0.0;

        let assign23930_e37124: f64 = if p.p414 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard530 = assign23930_e37124;
        locals.var_guard530_rv = 0.0;

        let (assign23940_e37146, assign23940_e37146_d_n3, assign23940_e37146_d_n4, assign23940_e37146_d_n5, assign23940_e37146_d_n6, assign23940_e37146_d_n7, assign23940_e37146_d_n8, assign23940_e37146_d_n9, assign23940_e37146_d_n10, assign23940_e37146_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard528 == 0.0)) && (locals.var_guard530 != 0.0)) {
        let assign23940_e37138: f64 = (p.p414 * locals.var_qim);
        let assign23940_e37140: f64 = (assign23940_e37138 / locals.var_esatl);
        let assign23940_e37141: f64 = (1.0 - assign23940_e37140);
        let assign23940_e37142: f64 = (locals.var_pclm_a / assign23940_e37141);
        let assign23940_e37144: f64 = (assign23940_e37142 / locals.var_fp);
        (assign23940_e37144, ((((((locals.var_pclm_a_dn3 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn3) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn4 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn4) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn5 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn5) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn6 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn6) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn7 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn7) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn8 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn8) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn9 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn9) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn10 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn10) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), ((((((locals.var_pclm_a_dn11 * assign23940_e37141) - (locals.var_pclm_a * (-((((p.p414 * locals.var_qim_dn11) * locals.var_esatl) - (assign23940_e37138 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl))))) / (assign23940_e37141 * assign23940_e37141)) * locals.var_fp) - (assign23940_e37142 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign23940_e37146;
        locals.var_t1_dn3 = assign23940_e37146_d_n3;
        locals.var_t1_dn4 = assign23940_e37146_d_n4;
        locals.var_t1_dn5 = assign23940_e37146_d_n5;
        locals.var_t1_dn6 = assign23940_e37146_d_n6;
        locals.var_t1_dn7 = assign23940_e37146_d_n7;
        locals.var_t1_dn8 = assign23940_e37146_d_n8;
        locals.var_t1_dn9 = assign23940_e37146_d_n9;
        locals.var_t1_dn10 = assign23940_e37146_d_n10;
        locals.var_t1_dn11 = assign23940_e37146_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign23950_e37169, assign23950_e37169_d_n3, assign23950_e37169_d_n4, assign23950_e37169_d_n5, assign23950_e37169_d_n6, assign23950_e37169_d_n7, assign23950_e37169_d_n8, assign23950_e37169_d_n9, assign23950_e37169_d_n10, assign23950_e37169_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard528 == 0.0)) && (locals.var_guard530 == 0.0)) {
        let assign23950_e37161: f64 = (p.p414 * locals.var_qim);
        let assign23950_e37163: f64 = (assign23950_e37161 / locals.var_esatl);
        let assign23950_e37164: f64 = (1.0 + assign23950_e37163);
        let assign23950_e37165: f64 = (locals.var_pclm_a * assign23950_e37164);
        let assign23950_e37167: f64 = (assign23950_e37165 / locals.var_fp);
        (assign23950_e37167, (((((locals.var_pclm_a_dn3 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn3) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn3)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn4 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn4) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn4)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn5 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn5) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn5)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn6 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn6) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn6)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn7 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn7) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn7)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn8 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn8) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn8)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn9 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn9) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn9)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn10 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn10) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn10)) / (locals.var_fp * locals.var_fp)), (((((locals.var_pclm_a_dn11 * assign23950_e37164) + (locals.var_pclm_a * ((((p.p414 * locals.var_qim_dn11) * locals.var_esatl) - (assign23950_e37161 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)))) * locals.var_fp) - (assign23950_e37165 * locals.var_fp_dn11)) / (locals.var_fp * locals.var_fp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign23950_e37169;
        locals.var_t1_dn3 = assign23950_e37169_d_n3;
        locals.var_t1_dn4 = assign23950_e37169_d_n4;
        locals.var_t1_dn5 = assign23950_e37169_d_n5;
        locals.var_t1_dn6 = assign23950_e37169_d_n6;
        locals.var_t1_dn7 = assign23950_e37169_d_n7;
        locals.var_t1_dn8 = assign23950_e37169_d_n8;
        locals.var_t1_dn9 = assign23950_e37169_d_n9;
        locals.var_t1_dn10 = assign23950_e37169_d_n10;
        locals.var_t1_dn11 = assign23950_e37169_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign23960_e37181, assign23960_e37181_d_n3, assign23960_e37181_d_n4, assign23960_e37181_d_n5, assign23960_e37181_d_n6, assign23960_e37181_d_n7, assign23960_e37181_d_n8, assign23960_e37181_d_n9, assign23960_e37181_d_n10, assign23960_e37181_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard528 == 0.0)) {
        let assign23960_e37179: f64 = (1.0 + locals.var_t1);
        (assign23960_e37179, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_mdl, locals.var_mdl_dn3, locals.var_mdl_dn4, locals.var_mdl_dn5, locals.var_mdl_dn6, locals.var_mdl_dn7, locals.var_mdl_dn8, locals.var_mdl_dn9, locals.var_mdl_dn10, locals.var_mdl_dn11,)
    }
};
        locals.var_mdl = assign23960_e37181;
        locals.var_mdl_dn3 = assign23960_e37181_d_n3;
        locals.var_mdl_dn4 = assign23960_e37181_d_n4;
        locals.var_mdl_dn5 = assign23960_e37181_d_n5;
        locals.var_mdl_dn6 = assign23960_e37181_d_n6;
        locals.var_mdl_dn7 = assign23960_e37181_d_n7;
        locals.var_mdl_dn8 = assign23960_e37181_d_n8;
        locals.var_mdl_dn9 = assign23960_e37181_d_n9;
        locals.var_mdl_dn10 = assign23960_e37181_d_n10;
        locals.var_mdl_dn11 = assign23960_e37181_d_n11;
        locals.var_mdl_rv = 0.0;

        let (assign23970_e37190, assign23970_e37190_d_n3, assign23970_e37190_d_n4, assign23970_e37190_d_n5, assign23970_e37190_d_n6, assign23970_e37190_d_n7, assign23970_e37190_d_n8, assign23970_e37190_d_n9, assign23970_e37190_d_n10, assign23970_e37190_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23970_e37188: f64 = (locals.var_moc * locals.var_mdl);
        (assign23970_e37188, ((locals.var_moc_dn3 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn3)), ((locals.var_moc_dn4 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn4)), ((locals.var_moc_dn5 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn5)), ((locals.var_moc_dn6 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn6)), ((locals.var_moc_dn7 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn7)), ((locals.var_moc_dn8 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn8)), ((locals.var_moc_dn9 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn9)), ((locals.var_moc_dn10 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn10)), ((locals.var_moc_dn11 * locals.var_mdl) + (locals.var_moc * locals.var_mdl_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign23970_e37190;
        locals.var_moc_dn3 = assign23970_e37190_d_n3;
        locals.var_moc_dn4 = assign23970_e37190_d_n4;
        locals.var_moc_dn5 = assign23970_e37190_d_n5;
        locals.var_moc_dn6 = assign23970_e37190_d_n6;
        locals.var_moc_dn7 = assign23970_e37190_d_n7;
        locals.var_moc_dn8 = assign23970_e37190_d_n8;
        locals.var_moc_dn9 = assign23970_e37190_d_n9;
        locals.var_moc_dn10 = assign23970_e37190_d_n10;
        locals.var_moc_dn11 = assign23970_e37190_d_n11;
        locals.var_moc_rv = 0.0;

        let (assign23980_e37200, assign23980_e37200_d_n3, assign23980_e37200_d_n4, assign23980_e37200_d_n5, assign23980_e37200_d_n6, assign23980_e37200_d_n7, assign23980_e37200_d_n8, assign23980_e37200_d_n9, assign23980_e37200_d_n10, assign23980_e37200_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign23980_e37197: f64 = (locals.var_pditsd_i * locals.var_vds);
        let assign23980_e37198: f64 = { let limited_exp_arg = assign23980_e37197; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23980_e37198, 0.0, 0.0, 0.0, ({ let limited_exp_arg = assign23980_e37197; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pditsd_i * locals.var_vds_dn6)), ({ let limited_exp_arg = assign23980_e37197; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pditsd_i * locals.var_vds_dn7)), 0.0, 0.0, ({ let limited_exp_arg = assign23980_e37197; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pditsd_i * locals.var_vds_dn10)), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign23980_e37200;
        locals.var_t1_dn3 = assign23980_e37200_d_n3;
        locals.var_t1_dn4 = assign23980_e37200_d_n4;
        locals.var_t1_dn5 = assign23980_e37200_d_n5;
        locals.var_t1_dn6 = assign23980_e37200_d_n6;
        locals.var_t1_dn7 = assign23980_e37200_d_n7;
        locals.var_t1_dn8 = assign23980_e37200_d_n8;
        locals.var_t1_dn9 = assign23980_e37200_d_n9;
        locals.var_t1_dn10 = assign23980_e37200_d_n10;
        locals.var_t1_dn11 = assign23980_e37200_d_n11;
        locals.var_t1_rv = 0.0;

        let assign23990_e37203: f64 = if locals.var_pdits_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign23990_e37203;
        locals.var_guard531_rv = 0.0;

        let (assign24000_e37216, assign24000_e37216_d_n3, assign24000_e37216_d_n4, assign24000_e37216_d_n5, assign24000_e37216_d_n6, assign24000_e37216_d_n7, assign24000_e37216_d_n8, assign24000_e37216_d_n9, assign24000_e37216_d_n10, assign24000_e37216_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard531 != 0.0)) {
        let assign24000_e37213: f64 = (p.p433 * locals.var_leff);
        let assign24000_e37214: f64 = (1.0 + assign24000_e37213);
        (assign24000_e37214, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24000_e37216;
        locals.var_t2_dn3 = assign24000_e37216_d_n3;
        locals.var_t2_dn4 = assign24000_e37216_d_n4;
        locals.var_t2_dn5 = assign24000_e37216_d_n5;
        locals.var_t2_dn6 = assign24000_e37216_d_n6;
        locals.var_t2_dn7 = assign24000_e37216_d_n7;
        locals.var_t2_dn8 = assign24000_e37216_d_n8;
        locals.var_t2_dn9 = assign24000_e37216_d_n9;
        locals.var_t2_dn10 = assign24000_e37216_d_n10;
        locals.var_t2_dn11 = assign24000_e37216_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign24010_e37231, assign24010_e37231_d_n3, assign24010_e37231_d_n4, assign24010_e37231_d_n5, assign24010_e37231_d_n6, assign24010_e37231_d_n7, assign24010_e37231_d_n8, assign24010_e37231_d_n9, assign24010_e37231_d_n10, assign24010_e37231_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard531 != 0.0)) {
        let assign24010_e37226: f64 = (locals.var_t2 * locals.var_t1);
        let assign24010_e37227: f64 = (1.0 + assign24010_e37226);
        let assign24010_e37229: f64 = (assign24010_e37227 / locals.var_pdits_i);
        (assign24010_e37229, (((locals.var_t2_dn3 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn3)) / locals.var_pdits_i), (((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)) / locals.var_pdits_i), (((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)) / locals.var_pdits_i), (((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)) / locals.var_pdits_i), (((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)) / locals.var_pdits_i), (((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)) / locals.var_pdits_i), (((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)) / locals.var_pdits_i), (((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)) / locals.var_pdits_i), (((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)) / locals.var_pdits_i),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign24010_e37231;
        locals.var_vadits_dn3 = assign24010_e37231_d_n3;
        locals.var_vadits_dn4 = assign24010_e37231_d_n4;
        locals.var_vadits_dn5 = assign24010_e37231_d_n5;
        locals.var_vadits_dn6 = assign24010_e37231_d_n6;
        locals.var_vadits_dn7 = assign24010_e37231_d_n7;
        locals.var_vadits_dn8 = assign24010_e37231_d_n8;
        locals.var_vadits_dn9 = assign24010_e37231_d_n9;
        locals.var_vadits_dn10 = assign24010_e37231_d_n10;
        locals.var_vadits_dn11 = assign24010_e37231_d_n11;
        locals.var_vadits_rv = 0.0;

        let (assign24020_e37242, assign24020_e37242_d_n3, assign24020_e37242_d_n4, assign24020_e37242_d_n5, assign24020_e37242_d_n6, assign24020_e37242_d_n7, assign24020_e37242_d_n8, assign24020_e37242_d_n9, assign24020_e37242_d_n10, assign24020_e37242_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard531 != 0.0)) {
        let assign24020_e37240: f64 = (locals.var_vadits * locals.var_fp);
        (assign24020_e37240, ((locals.var_vadits_dn3 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn3)), ((locals.var_vadits_dn4 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn4)), ((locals.var_vadits_dn5 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn5)), ((locals.var_vadits_dn6 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn6)), ((locals.var_vadits_dn7 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn7)), ((locals.var_vadits_dn8 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn8)), ((locals.var_vadits_dn9 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn9)), ((locals.var_vadits_dn10 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn10)), ((locals.var_vadits_dn11 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn11)),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign24020_e37242;
        locals.var_vadits_dn3 = assign24020_e37242_d_n3;
        locals.var_vadits_dn4 = assign24020_e37242_d_n4;
        locals.var_vadits_dn5 = assign24020_e37242_d_n5;
        locals.var_vadits_dn6 = assign24020_e37242_d_n6;
        locals.var_vadits_dn7 = assign24020_e37242_d_n7;
        locals.var_vadits_dn8 = assign24020_e37242_d_n8;
        locals.var_vadits_dn9 = assign24020_e37242_d_n9;
        locals.var_vadits_dn10 = assign24020_e37242_d_n10;
        locals.var_vadits_dn11 = assign24020_e37242_d_n11;
        locals.var_vadits_rv = 0.0;

        let (assign24030_e37252, assign24030_e37252_d_n3, assign24030_e37252_d_n4, assign24030_e37252_d_n5, assign24030_e37252_d_n6, assign24030_e37252_d_n7, assign24030_e37252_d_n8, assign24030_e37252_d_n9, assign24030_e37252_d_n10, assign24030_e37252_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard531 == 0.0)) {
        (5.540622384e34, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11,)
    }
};
        locals.var_vadits = assign24030_e37252;
        locals.var_vadits_dn3 = assign24030_e37252_d_n3;
        locals.var_vadits_dn4 = assign24030_e37252_d_n4;
        locals.var_vadits_dn5 = assign24030_e37252_d_n5;
        locals.var_vadits_dn6 = assign24030_e37252_d_n6;
        locals.var_vadits_dn7 = assign24030_e37252_d_n7;
        locals.var_vadits_dn8 = assign24030_e37252_d_n8;
        locals.var_vadits_dn9 = assign24030_e37252_d_n9;
        locals.var_vadits_dn10 = assign24030_e37252_d_n10;
        locals.var_vadits_dn11 = assign24030_e37252_d_n11;
        locals.var_vadits_rv = 0.0;

        let (assign24040_e37261, assign24040_e37261_d_n3, assign24040_e37261_d_n4, assign24040_e37261_d_n5, assign24040_e37261_d_n6, assign24040_e37261_d_n7, assign24040_e37261_d_n8, assign24040_e37261_d_n9, assign24040_e37261_d_n10, assign24040_e37261_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24040_e37259: f64 = (locals.var_diffvds / locals.var_vadits);
        (assign24040_e37259, (((locals.var_diffvds_dn3 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn3)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn4 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn4)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn5 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn5)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn6 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn6)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn7 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn7)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn8 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn8)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn9 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn9)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn10 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn10)) / (locals.var_vadits * locals.var_vadits)), (((locals.var_diffvds_dn11 * locals.var_vadits) - (locals.var_diffvds * locals.var_vadits_dn11)) / (locals.var_vadits * locals.var_vadits)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign24040_e37261;
        locals.var_t4_dn3 = assign24040_e37261_d_n3;
        locals.var_t4_dn4 = assign24040_e37261_d_n4;
        locals.var_t4_dn5 = assign24040_e37261_d_n5;
        locals.var_t4_dn6 = assign24040_e37261_d_n6;
        locals.var_t4_dn7 = assign24040_e37261_d_n7;
        locals.var_t4_dn8 = assign24040_e37261_d_n8;
        locals.var_t4_dn9 = assign24040_e37261_d_n9;
        locals.var_t4_dn10 = assign24040_e37261_d_n10;
        locals.var_t4_dn11 = assign24040_e37261_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign24050_e37270, assign24050_e37270_d_n3, assign24050_e37270_d_n4, assign24050_e37270_d_n5, assign24050_e37270_d_n6, assign24050_e37270_d_n7, assign24050_e37270_d_n8, assign24050_e37270_d_n9, assign24050_e37270_d_n10, assign24050_e37270_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24050_e37268: f64 = (1.0 + locals.var_t4);
        (assign24050_e37268, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24050_e37270;
        locals.var_t0_dn3 = assign24050_e37270_d_n3;
        locals.var_t0_dn4 = assign24050_e37270_d_n4;
        locals.var_t0_dn5 = assign24050_e37270_d_n5;
        locals.var_t0_dn6 = assign24050_e37270_d_n6;
        locals.var_t0_dn7 = assign24050_e37270_d_n7;
        locals.var_t0_dn8 = assign24050_e37270_d_n8;
        locals.var_t0_dn9 = assign24050_e37270_d_n9;
        locals.var_t0_dn10 = assign24050_e37270_d_n10;
        locals.var_t0_dn11 = assign24050_e37270_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24060_e37279, assign24060_e37279_d_n3, assign24060_e37279_d_n4, assign24060_e37279_d_n5, assign24060_e37279_d_n6, assign24060_e37279_d_n7, assign24060_e37279_d_n8, assign24060_e37279_d_n9, assign24060_e37279_d_n10, assign24060_e37279_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24060_e37277: f64 = (locals.var_moc * locals.var_t0);
        (assign24060_e37277, ((locals.var_moc_dn3 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn3)), ((locals.var_moc_dn4 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn4)), ((locals.var_moc_dn5 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn5)), ((locals.var_moc_dn6 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn6)), ((locals.var_moc_dn7 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn7)), ((locals.var_moc_dn8 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn8)), ((locals.var_moc_dn9 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn9)), ((locals.var_moc_dn10 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn10)), ((locals.var_moc_dn11 * locals.var_t0) + (locals.var_moc * locals.var_t0_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign24060_e37279;
        locals.var_moc_dn3 = assign24060_e37279_d_n3;
        locals.var_moc_dn4 = assign24060_e37279_d_n4;
        locals.var_moc_dn5 = assign24060_e37279_d_n5;
        locals.var_moc_dn6 = assign24060_e37279_d_n6;
        locals.var_moc_dn7 = assign24060_e37279_d_n7;
        locals.var_moc_dn8 = assign24060_e37279_d_n8;
        locals.var_moc_dn9 = assign24060_e37279_d_n9;
        locals.var_moc_dn10 = assign24060_e37279_d_n10;
        locals.var_moc_dn11 = assign24060_e37279_d_n11;
        locals.var_moc_rv = 0.0;

        let assign24070_e37282: f64 = if locals.var_pscbe2_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign24070_e37282;
        locals.var_guard532_rv = 0.0;

        let assign24080_e37286: f64 = (locals.var_pscbe1_i * locals.var_litl);
        let assign24080_e37288: f64 = (assign24080_e37286 / 80.0);
        let assign24080_e37289: f64 = if locals.var_diffvds > assign24080_e37288 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign24080_e37289;
        locals.var_guard533_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24090_e37304, assign24090_e37304_d_n3, assign24090_e37304_d_n4, assign24090_e37304_d_n5, assign24090_e37304_d_n6, assign24090_e37304_d_n7, assign24090_e37304_d_n8, assign24090_e37304_d_n9, assign24090_e37304_d_n10, assign24090_e37304_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 != 0.0)) {
        let assign24090_e37300: f64 = (locals.var_pscbe1_i * locals.var_litl);
        let assign24090_e37302: f64 = (assign24090_e37300 / locals.var_diffvds);
        (assign24090_e37302, (-((assign24090_e37300 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn4) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn5) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign24090_e37300 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24090_e37304;
        locals.var_t0_dn3 = assign24090_e37304_d_n3;
        locals.var_t0_dn4 = assign24090_e37304_d_n4;
        locals.var_t0_dn5 = assign24090_e37304_d_n5;
        locals.var_t0_dn6 = assign24090_e37304_d_n6;
        locals.var_t0_dn7 = assign24090_e37304_d_n7;
        locals.var_t0_dn8 = assign24090_e37304_d_n8;
        locals.var_t0_dn9 = assign24090_e37304_d_n9;
        locals.var_t0_dn10 = assign24090_e37304_d_n10;
        locals.var_t0_dn11 = assign24090_e37304_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24100_e37320, assign24100_e37320_d_n3, assign24100_e37320_d_n4, assign24100_e37320_d_n5, assign24100_e37320_d_n6, assign24100_e37320_d_n7, assign24100_e37320_d_n8, assign24100_e37320_d_n9, assign24100_e37320_d_n10, assign24100_e37320_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 != 0.0)) {
        let assign24100_e37315: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24100_e37316: f64 = (locals.var_leff * assign24100_e37315);
        let assign24100_e37318: f64 = (assign24100_e37316 / locals.var_pscbe2_i);
        (assign24100_e37318, ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10)) / locals.var_pscbe2_i), ((locals.var_leff * ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11)) / locals.var_pscbe2_i),)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign24100_e37320;
        locals.var_vascbe_dn3 = assign24100_e37320_d_n3;
        locals.var_vascbe_dn4 = assign24100_e37320_d_n4;
        locals.var_vascbe_dn5 = assign24100_e37320_d_n5;
        locals.var_vascbe_dn6 = assign24100_e37320_d_n6;
        locals.var_vascbe_dn7 = assign24100_e37320_d_n7;
        locals.var_vascbe_dn8 = assign24100_e37320_d_n8;
        locals.var_vascbe_dn9 = assign24100_e37320_d_n9;
        locals.var_vascbe_dn10 = assign24100_e37320_d_n10;
        locals.var_vascbe_dn11 = assign24100_e37320_d_n11;
        locals.var_vascbe_rv = 0.0;

        let (assign24110_e37336, assign24110_e37336_d_n3, assign24110_e37336_d_n4, assign24110_e37336_d_n5, assign24110_e37336_d_n6, assign24110_e37336_d_n7, assign24110_e37336_d_n8, assign24110_e37336_d_n9, assign24110_e37336_d_n10, assign24110_e37336_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard532 != 0.0)) && (locals.var_guard533 == 0.0)) {
        let assign24110_e37332: f64 = (5.540622384e34 * locals.var_leff);
        let assign24110_e37334: f64 = (assign24110_e37332 / locals.var_pscbe2_i);
        (assign24110_e37334, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign24110_e37336;
        locals.var_vascbe_dn3 = assign24110_e37336_d_n3;
        locals.var_vascbe_dn4 = assign24110_e37336_d_n4;
        locals.var_vascbe_dn5 = assign24110_e37336_d_n5;
        locals.var_vascbe_dn6 = assign24110_e37336_d_n6;
        locals.var_vascbe_dn7 = assign24110_e37336_d_n7;
        locals.var_vascbe_dn8 = assign24110_e37336_d_n8;
        locals.var_vascbe_dn9 = assign24110_e37336_d_n9;
        locals.var_vascbe_dn10 = assign24110_e37336_d_n10;
        locals.var_vascbe_dn11 = assign24110_e37336_d_n11;
        locals.var_vascbe_rv = 0.0;

        let (assign24120_e37346, assign24120_e37346_d_n3, assign24120_e37346_d_n4, assign24120_e37346_d_n5, assign24120_e37346_d_n6, assign24120_e37346_d_n7, assign24120_e37346_d_n8, assign24120_e37346_d_n9, assign24120_e37346_d_n10, assign24120_e37346_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard532 == 0.0)) {
        (5.540622384e34, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vascbe, locals.var_vascbe_dn3, locals.var_vascbe_dn4, locals.var_vascbe_dn5, locals.var_vascbe_dn6, locals.var_vascbe_dn7, locals.var_vascbe_dn8, locals.var_vascbe_dn9, locals.var_vascbe_dn10, locals.var_vascbe_dn11,)
    }
};
        locals.var_vascbe = assign24120_e37346;
        locals.var_vascbe_dn3 = assign24120_e37346_d_n3;
        locals.var_vascbe_dn4 = assign24120_e37346_d_n4;
        locals.var_vascbe_dn5 = assign24120_e37346_d_n5;
        locals.var_vascbe_dn6 = assign24120_e37346_d_n6;
        locals.var_vascbe_dn7 = assign24120_e37346_d_n7;
        locals.var_vascbe_dn8 = assign24120_e37346_d_n8;
        locals.var_vascbe_dn9 = assign24120_e37346_d_n9;
        locals.var_vascbe_dn10 = assign24120_e37346_d_n10;
        locals.var_vascbe_dn11 = assign24120_e37346_d_n11;
        locals.var_vascbe_rv = 0.0;

        let (assign24130_e37357, assign24130_e37357_d_n3, assign24130_e37357_d_n4, assign24130_e37357_d_n5, assign24130_e37357_d_n6, assign24130_e37357_d_n7, assign24130_e37357_d_n8, assign24130_e37357_d_n9, assign24130_e37357_d_n10, assign24130_e37357_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24130_e37354: f64 = (locals.var_diffvds / locals.var_vascbe);
        let assign24130_e37355: f64 = (1.0 + assign24130_e37354);
        (assign24130_e37355, (((locals.var_diffvds_dn3 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn3)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn4 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn4)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn5 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn5)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn6 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn6)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn7 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn7)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn8 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn8)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn9 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn9)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn10 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn10)) / (locals.var_vascbe * locals.var_vascbe)), (((locals.var_diffvds_dn11 * locals.var_vascbe) - (locals.var_diffvds * locals.var_vascbe_dn11)) / (locals.var_vascbe * locals.var_vascbe)),)
    } else {
        (locals.var_mscbe, locals.var_mscbe_dn3, locals.var_mscbe_dn4, locals.var_mscbe_dn5, locals.var_mscbe_dn6, locals.var_mscbe_dn7, locals.var_mscbe_dn8, locals.var_mscbe_dn9, locals.var_mscbe_dn10, locals.var_mscbe_dn11,)
    }
};
        locals.var_mscbe = assign24130_e37357;
        locals.var_mscbe_dn3 = assign24130_e37357_d_n3;
        locals.var_mscbe_dn4 = assign24130_e37357_d_n4;
        locals.var_mscbe_dn5 = assign24130_e37357_d_n5;
        locals.var_mscbe_dn6 = assign24130_e37357_d_n6;
        locals.var_mscbe_dn7 = assign24130_e37357_d_n7;
        locals.var_mscbe_dn8 = assign24130_e37357_d_n8;
        locals.var_mscbe_dn9 = assign24130_e37357_d_n9;
        locals.var_mscbe_dn10 = assign24130_e37357_d_n10;
        locals.var_mscbe_dn11 = assign24130_e37357_d_n11;
        locals.var_mscbe_rv = 0.0;

        let (assign24140_e37366, assign24140_e37366_d_n3, assign24140_e37366_d_n4, assign24140_e37366_d_n5, assign24140_e37366_d_n6, assign24140_e37366_d_n7, assign24140_e37366_d_n8, assign24140_e37366_d_n9, assign24140_e37366_d_n10, assign24140_e37366_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24140_e37364: f64 = (locals.var_moc * locals.var_mscbe);
        (assign24140_e37364, ((locals.var_moc_dn3 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn3)), ((locals.var_moc_dn4 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn4)), ((locals.var_moc_dn5 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn5)), ((locals.var_moc_dn6 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn6)), ((locals.var_moc_dn7 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn7)), ((locals.var_moc_dn8 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn8)), ((locals.var_moc_dn9 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn9)), ((locals.var_moc_dn10 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn10)), ((locals.var_moc_dn11 * locals.var_mscbe) + (locals.var_moc * locals.var_mscbe_dn11)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8, locals.var_moc_dn9, locals.var_moc_dn10, locals.var_moc_dn11,)
    }
};
        locals.var_moc = assign24140_e37366;
        locals.var_moc_dn3 = assign24140_e37366_d_n3;
        locals.var_moc_dn4 = assign24140_e37366_d_n4;
        locals.var_moc_dn5 = assign24140_e37366_d_n5;
        locals.var_moc_dn6 = assign24140_e37366_d_n6;
        locals.var_moc_dn7 = assign24140_e37366_d_n7;
        locals.var_moc_dn8 = assign24140_e37366_d_n8;
        locals.var_moc_dn9 = assign24140_e37366_d_n9;
        locals.var_moc_dn10 = assign24140_e37366_d_n10;
        locals.var_moc_dn11 = assign24140_e37366_d_n11;
        locals.var_moc_rv = 0.0;

        let assign24150_e37369: f64 = if locals.var_psatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign24150_e37369;
        locals.var_guard534_rv = 0.0;

        let (assign24160_e37384, assign24160_e37384_d_n3, assign24160_e37384_d_n4, assign24160_e37384_d_n5, assign24160_e37384_d_n6, assign24160_e37384_d_n7, assign24160_e37384_d_n8, assign24160_e37384_d_n9, assign24160_e37384_d_n10, assign24160_e37384_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard534 != 0.0)) {
        let assign24160_e37380: f64 = (locals.var_psatb_i * locals.var_vbsx);
        let assign24160_e37381: f64 = (1.0 - assign24160_e37380);
        let assign24160_e37382: f64 = (1.0 / assign24160_e37381);
        (assign24160_e37382, (-((-(locals.var_psatb_i * locals.var_vbsx_dn3)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn4)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn5)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn6)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn7)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn8)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn9)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn10)) / (assign24160_e37381 * assign24160_e37381))), (-((-(locals.var_psatb_i * locals.var_vbsx_dn11)) / (assign24160_e37381 * assign24160_e37381))),)
    } else {
        (locals.var_zetasb, locals.var_zetasb_dn3, locals.var_zetasb_dn4, locals.var_zetasb_dn5, locals.var_zetasb_dn6, locals.var_zetasb_dn7, locals.var_zetasb_dn8, locals.var_zetasb_dn9, locals.var_zetasb_dn10, locals.var_zetasb_dn11,)
    }
};
        locals.var_zetasb = assign24160_e37384;
        locals.var_zetasb_dn3 = assign24160_e37384_d_n3;
        locals.var_zetasb_dn4 = assign24160_e37384_d_n4;
        locals.var_zetasb_dn5 = assign24160_e37384_d_n5;
        locals.var_zetasb_dn6 = assign24160_e37384_d_n6;
        locals.var_zetasb_dn7 = assign24160_e37384_d_n7;
        locals.var_zetasb_dn8 = assign24160_e37384_d_n8;
        locals.var_zetasb_dn9 = assign24160_e37384_d_n9;
        locals.var_zetasb_dn10 = assign24160_e37384_d_n10;
        locals.var_zetasb_dn11 = assign24160_e37384_d_n11;
        locals.var_zetasb_rv = 0.0;

        let (assign24170_e37398, assign24170_e37398_d_n3, assign24170_e37398_d_n4, assign24170_e37398_d_n5, assign24170_e37398_d_n6, assign24170_e37398_d_n7, assign24170_e37398_d_n8, assign24170_e37398_d_n9, assign24170_e37398_d_n10, assign24170_e37398_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard534 == 0.0)) {
        let assign24170_e37395: f64 = (locals.var_psatb_i * locals.var_vbsx);
        let assign24170_e37396: f64 = (1.0 + assign24170_e37395);
        (assign24170_e37396, (locals.var_psatb_i * locals.var_vbsx_dn3), (locals.var_psatb_i * locals.var_vbsx_dn4), (locals.var_psatb_i * locals.var_vbsx_dn5), (locals.var_psatb_i * locals.var_vbsx_dn6), (locals.var_psatb_i * locals.var_vbsx_dn7), (locals.var_psatb_i * locals.var_vbsx_dn8), (locals.var_psatb_i * locals.var_vbsx_dn9), (locals.var_psatb_i * locals.var_vbsx_dn10), (locals.var_psatb_i * locals.var_vbsx_dn11),)
    } else {
        (locals.var_zetasb, locals.var_zetasb_dn3, locals.var_zetasb_dn4, locals.var_zetasb_dn5, locals.var_zetasb_dn6, locals.var_zetasb_dn7, locals.var_zetasb_dn8, locals.var_zetasb_dn9, locals.var_zetasb_dn10, locals.var_zetasb_dn11,)
    }
};
        locals.var_zetasb = assign24170_e37398;
        locals.var_zetasb_dn3 = assign24170_e37398_d_n3;
        locals.var_zetasb_dn4 = assign24170_e37398_d_n4;
        locals.var_zetasb_dn5 = assign24170_e37398_d_n5;
        locals.var_zetasb_dn6 = assign24170_e37398_d_n6;
        locals.var_zetasb_dn7 = assign24170_e37398_d_n7;
        locals.var_zetasb_dn8 = assign24170_e37398_d_n8;
        locals.var_zetasb_dn9 = assign24170_e37398_d_n9;
        locals.var_zetasb_dn10 = assign24170_e37398_d_n10;
        locals.var_zetasb_dn11 = assign24170_e37398_d_n11;
        locals.var_zetasb_rv = 0.0;

        let (assign24180_e37407, assign24180_e37407_d_n3, assign24180_e37407_d_n4, assign24180_e37407_d_n5, assign24180_e37407_d_n6, assign24180_e37407_d_n7, assign24180_e37407_d_n8, assign24180_e37407_d_n9, assign24180_e37407_d_n10, assign24180_e37407_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24180_e37405: f64 = (locals.var_qim * locals.var_zetasb);
        (assign24180_e37405, ((locals.var_qim_dn3 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn3)), ((locals.var_qim_dn4 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn4)), ((locals.var_qim_dn5 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn5)), ((locals.var_qim_dn6 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn6)), ((locals.var_qim_dn7 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn7)), ((locals.var_qim_dn8 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn8)), ((locals.var_qim_dn9 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn9)), ((locals.var_qim_dn10 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn10)), ((locals.var_qim_dn11 * locals.var_zetasb) + (locals.var_qim * locals.var_zetasb_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24180_e37407;
        locals.var_t0_dn3 = assign24180_e37407_d_n3;
        locals.var_t0_dn4 = assign24180_e37407_d_n4;
        locals.var_t0_dn5 = assign24180_e37407_d_n5;
        locals.var_t0_dn6 = assign24180_e37407_d_n6;
        locals.var_t0_dn7 = assign24180_e37407_d_n7;
        locals.var_t0_dn8 = assign24180_e37407_d_n8;
        locals.var_t0_dn9 = assign24180_e37407_d_n9;
        locals.var_t0_dn10 = assign24180_e37407_d_n10;
        locals.var_t0_dn11 = assign24180_e37407_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24190_e37420, assign24190_e37420_d_n3, assign24190_e37420_d_n4, assign24190_e37420_d_n5, assign24190_e37420_d_n6, assign24190_e37420_d_n7, assign24190_e37420_d_n8, assign24190_e37420_d_n9, assign24190_e37420_d_n10, assign24190_e37420_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24190_e37416: f64 = (100.0 + locals.var_t0);
        let assign24190_e37417: f64 = (locals.var_t0 / assign24190_e37416);
        let assign24190_e37418: f64 = (100.0 * assign24190_e37417);
        (assign24190_e37418, (100.0 * (((locals.var_t0_dn3 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn3)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn4 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn4)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn5 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn5)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn6 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn6)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn7 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn7)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn8 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn8)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn9 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn9)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn10 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn10)) / (assign24190_e37416 * assign24190_e37416))), (100.0 * (((locals.var_t0_dn11 * assign24190_e37416) - (locals.var_t0 * locals.var_t0_dn11)) / (assign24190_e37416 * assign24190_e37416))),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn3, locals.var_wsat_dn4, locals.var_wsat_dn5, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, locals.var_wsat_dn9, locals.var_wsat_dn10, locals.var_wsat_dn11,)
    }
};
        locals.var_wsat = assign24190_e37420;
        locals.var_wsat_dn3 = assign24190_e37420_d_n3;
        locals.var_wsat_dn4 = assign24190_e37420_d_n4;
        locals.var_wsat_dn5 = assign24190_e37420_d_n5;
        locals.var_wsat_dn6 = assign24190_e37420_d_n6;
        locals.var_wsat_dn7 = assign24190_e37420_d_n7;
        locals.var_wsat_dn8 = assign24190_e37420_d_n8;
        locals.var_wsat_dn9 = assign24190_e37420_d_n9;
        locals.var_wsat_dn10 = assign24190_e37420_d_n10;
        locals.var_wsat_dn11 = assign24190_e37420_d_n11;
        locals.var_wsat_rv = 0.0;

        let (assign24200_e37429,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24200_e37427: f64 = (1.0 / p.p503);
        (assign24200_e37427,)
    } else {
        (locals.var_inv_vp,)
    }
};
        locals.var_inv_vp = assign24200_e37429;
        locals.var_inv_vp_rv = 0.0;

        let (assign24210_e37451, assign24210_e37451_d_n3, assign24210_e37451_d_n4, assign24210_e37451_d_n5, assign24210_e37451_d_n6, assign24210_e37451_d_n7, assign24210_e37451_d_n8, assign24210_e37451_d_n9, assign24210_e37451_d_n10, assign24210_e37451_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24210_e37437: f64 = (locals.var_vds - locals.var_dps);
        let assign24210_e37439: f64 = (assign24210_e37437 * locals.var_inv_vp);
        let assign24210_e37440: f64 = (1.0 + assign24210_e37439);
        let assign24210_e37444: f64 = (locals.var_vdseff - locals.var_dps);
        let assign24210_e37446: f64 = (assign24210_e37444 * locals.var_inv_vp);
        let assign24210_e37447: f64 = (1.0 + assign24210_e37446);
        let assign24210_e37448: f64 = (assign24210_e37440 / assign24210_e37447);
        let assign24210_e37449: f64 = (assign24210_e37448).ln();
        (assign24210_e37449, ((((((-locals.var_dps_dn3) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn3 - locals.var_dps_dn3) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((-locals.var_dps_dn4) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn4 - locals.var_dps_dn4) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((-locals.var_dps_dn5) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn5 - locals.var_dps_dn5) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((locals.var_vds_dn6 - locals.var_dps_dn6) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn6 - locals.var_dps_dn6) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((locals.var_vds_dn7 - locals.var_dps_dn7) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn7 - locals.var_dps_dn7) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((-locals.var_dps_dn8) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn8 - locals.var_dps_dn8) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((-locals.var_dps_dn9) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn9 - locals.var_dps_dn9) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((locals.var_vds_dn10 - locals.var_dps_dn10) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn10 - locals.var_dps_dn10) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448), ((((((-locals.var_dps_dn11) * locals.var_inv_vp) * assign24210_e37447) - (assign24210_e37440 * ((locals.var_vdseff_dn11 - locals.var_dps_dn11) * locals.var_inv_vp))) / (assign24210_e37447 * assign24210_e37447)) / assign24210_e37448),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24210_e37451;
        locals.var_t0_dn3 = assign24210_e37451_d_n3;
        locals.var_t0_dn4 = assign24210_e37451_d_n4;
        locals.var_t0_dn5 = assign24210_e37451_d_n5;
        locals.var_t0_dn6 = assign24210_e37451_d_n6;
        locals.var_t0_dn7 = assign24210_e37451_d_n7;
        locals.var_t0_dn8 = assign24210_e37451_d_n8;
        locals.var_t0_dn9 = assign24210_e37451_d_n9;
        locals.var_t0_dn10 = assign24210_e37451_d_n10;
        locals.var_t0_dn11 = assign24210_e37451_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24220_e37460, assign24220_e37460_d_n3, assign24220_e37460_d_n4, assign24220_e37460_d_n5, assign24220_e37460_d_n6, assign24220_e37460_d_n7, assign24220_e37460_d_n8, assign24220_e37460_d_n9, assign24220_e37460_d_n10, assign24220_e37460_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24220_e37458: f64 = (p.p504 * locals.var_t0);
        (assign24220_e37458, (p.p504 * locals.var_t0_dn3), (p.p504 * locals.var_t0_dn4), (p.p504 * locals.var_t0_dn5), (p.p504 * locals.var_t0_dn6), (p.p504 * locals.var_t0_dn7), (p.p504 * locals.var_t0_dn8), (p.p504 * locals.var_t0_dn9), (p.p504 * locals.var_t0_dn10), (p.p504 * locals.var_t0_dn11),)
    } else {
        (locals.var_dl, locals.var_dl_dn3, locals.var_dl_dn4, locals.var_dl_dn5, locals.var_dl_dn6, locals.var_dl_dn7, locals.var_dl_dn8, locals.var_dl_dn9, locals.var_dl_dn10, locals.var_dl_dn11,)
    }
};
        locals.var_dl = assign24220_e37460;
        locals.var_dl_dn3 = assign24220_e37460_d_n3;
        locals.var_dl_dn4 = assign24220_e37460_d_n4;
        locals.var_dl_dn5 = assign24220_e37460_d_n5;
        locals.var_dl_dn6 = assign24220_e37460_d_n6;
        locals.var_dl_dn7 = assign24220_e37460_d_n7;
        locals.var_dl_dn8 = assign24220_e37460_d_n8;
        locals.var_dl_dn9 = assign24220_e37460_d_n9;
        locals.var_dl_dn10 = assign24220_e37460_d_n10;
        locals.var_dl_dn11 = assign24220_e37460_d_n11;
        locals.var_dl_rv = 0.0;

        let (assign24230_e37475, assign24230_e37475_d_n3, assign24230_e37475_d_n4, assign24230_e37475_d_n5, assign24230_e37475_d_n6, assign24230_e37475_d_n7, assign24230_e37475_d_n8, assign24230_e37475_d_n9, assign24230_e37475_d_n10, assign24230_e37475_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24230_e37468: f64 = (1.0 + locals.var_dl);
        let assign24230_e37471: f64 = (locals.var_dl * locals.var_dl);
        let assign24230_e37472: f64 = (assign24230_e37468 + assign24230_e37471);
        let assign24230_e37473: f64 = (1.0 / assign24230_e37472);
        (assign24230_e37473, (-((locals.var_dl_dn3 + ((locals.var_dl_dn3 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn3))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn4 + ((locals.var_dl_dn4 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn4))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn5 + ((locals.var_dl_dn5 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn5))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn6 + ((locals.var_dl_dn6 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn6))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn7 + ((locals.var_dl_dn7 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn7))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn8 + ((locals.var_dl_dn8 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn8))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn9 + ((locals.var_dl_dn9 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn9))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn10 + ((locals.var_dl_dn10 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn10))) / (assign24230_e37472 * assign24230_e37472))), (-((locals.var_dl_dn11 + ((locals.var_dl_dn11 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn11))) / (assign24230_e37472 * assign24230_e37472))),)
    } else {
        (locals.var_ddl, locals.var_ddl_dn3, locals.var_ddl_dn4, locals.var_ddl_dn5, locals.var_ddl_dn6, locals.var_ddl_dn7, locals.var_ddl_dn8, locals.var_ddl_dn9, locals.var_ddl_dn10, locals.var_ddl_dn11,)
    }
};
        locals.var_ddl = assign24230_e37475;
        locals.var_ddl_dn3 = assign24230_e37475_d_n3;
        locals.var_ddl_dn4 = assign24230_e37475_d_n4;
        locals.var_ddl_dn5 = assign24230_e37475_d_n5;
        locals.var_ddl_dn6 = assign24230_e37475_d_n6;
        locals.var_ddl_dn7 = assign24230_e37475_d_n7;
        locals.var_ddl_dn8 = assign24230_e37475_d_n8;
        locals.var_ddl_dn9 = assign24230_e37475_d_n9;
        locals.var_ddl_dn10 = assign24230_e37475_d_n10;
        locals.var_ddl_dn11 = assign24230_e37475_d_n11;
        locals.var_ddl_rv = 0.0;

        let (assign24240_e37484, assign24240_e37484_d_n3, assign24240_e37484_d_n4, assign24240_e37484_d_n5, assign24240_e37484_d_n6, assign24240_e37484_d_n7, assign24240_e37484_d_n8, assign24240_e37484_d_n9, assign24240_e37484_d_n10, assign24240_e37484_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24240_e37482: f64 = (locals.var_dmob * locals.var_ddl);
        (assign24240_e37482, ((locals.var_dmob_dn3 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn3)), ((locals.var_dmob_dn4 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn4)), ((locals.var_dmob_dn5 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn5)), ((locals.var_dmob_dn6 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn6)), ((locals.var_dmob_dn7 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn7)), ((locals.var_dmob_dn8 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn8)), ((locals.var_dmob_dn9 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn9)), ((locals.var_dmob_dn10 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn10)), ((locals.var_dmob_dn11 * locals.var_ddl) + (locals.var_dmob * locals.var_ddl_dn11)),)
    } else {
        (locals.var_dmob_dl, locals.var_dmob_dl_dn3, locals.var_dmob_dl_dn4, locals.var_dmob_dl_dn5, locals.var_dmob_dl_dn6, locals.var_dmob_dl_dn7, locals.var_dmob_dl_dn8, locals.var_dmob_dl_dn9, locals.var_dmob_dl_dn10, locals.var_dmob_dl_dn11,)
    }
};
        locals.var_dmob_dl = assign24240_e37484;
        locals.var_dmob_dl_dn3 = assign24240_e37484_d_n3;
        locals.var_dmob_dl_dn4 = assign24240_e37484_d_n4;
        locals.var_dmob_dl_dn5 = assign24240_e37484_d_n5;
        locals.var_dmob_dl_dn6 = assign24240_e37484_d_n6;
        locals.var_dmob_dl_dn7 = assign24240_e37484_d_n7;
        locals.var_dmob_dl_dn8 = assign24240_e37484_d_n8;
        locals.var_dmob_dl_dn9 = assign24240_e37484_d_n9;
        locals.var_dmob_dl_dn10 = assign24240_e37484_d_n10;
        locals.var_dmob_dl_dn11 = assign24240_e37484_d_n11;
        locals.var_dmob_dl_rv = 0.0;

        let assign24250_e37487: f64 = if locals.var_psat_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign24250_e37487;
        locals.var_guard535_rv = 0.0;

        let (assign24260_e37502, assign24260_e37502_d_n3, assign24260_e37502_d_n4, assign24260_e37502_d_n5, assign24260_e37502_d_n6, assign24260_e37502_d_n7, assign24260_e37502_d_n8, assign24260_e37502_d_n9, assign24260_e37502_d_n10, assign24260_e37502_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard535 != 0.0)) {
        let assign24260_e37498: f64 = (locals.var_psat_i * locals.var_wsat);
        let assign24260_e37499: f64 = (1.0 - assign24260_e37498);
        let assign24260_e37500: f64 = (1.0 / assign24260_e37499);
        (assign24260_e37500, (-((-(locals.var_psat_i * locals.var_wsat_dn3)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn4)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn5)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn6)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn7)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn8)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn9)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn10)) / (assign24260_e37499 * assign24260_e37499))), (-((-(locals.var_psat_i * locals.var_wsat_dn11)) / (assign24260_e37499 * assign24260_e37499))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24260_e37502;
        locals.var_t1_dn3 = assign24260_e37502_d_n3;
        locals.var_t1_dn4 = assign24260_e37502_d_n4;
        locals.var_t1_dn5 = assign24260_e37502_d_n5;
        locals.var_t1_dn6 = assign24260_e37502_d_n6;
        locals.var_t1_dn7 = assign24260_e37502_d_n7;
        locals.var_t1_dn8 = assign24260_e37502_d_n8;
        locals.var_t1_dn9 = assign24260_e37502_d_n9;
        locals.var_t1_dn10 = assign24260_e37502_d_n10;
        locals.var_t1_dn11 = assign24260_e37502_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign24270_e37516, assign24270_e37516_d_n3, assign24270_e37516_d_n4, assign24270_e37516_d_n5, assign24270_e37516_d_n6, assign24270_e37516_d_n7, assign24270_e37516_d_n8, assign24270_e37516_d_n9, assign24270_e37516_d_n10, assign24270_e37516_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard535 == 0.0)) {
        let assign24270_e37513: f64 = (locals.var_psat_i * locals.var_wsat);
        let assign24270_e37514: f64 = (1.0 + assign24270_e37513);
        (assign24270_e37514, (locals.var_psat_i * locals.var_wsat_dn3), (locals.var_psat_i * locals.var_wsat_dn4), (locals.var_psat_i * locals.var_wsat_dn5), (locals.var_psat_i * locals.var_wsat_dn6), (locals.var_psat_i * locals.var_wsat_dn7), (locals.var_psat_i * locals.var_wsat_dn8), (locals.var_psat_i * locals.var_wsat_dn9), (locals.var_psat_i * locals.var_wsat_dn10), (locals.var_psat_i * locals.var_wsat_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24270_e37516;
        locals.var_t1_dn3 = assign24270_e37516_d_n3;
        locals.var_t1_dn4 = assign24270_e37516_d_n4;
        locals.var_t1_dn5 = assign24270_e37516_d_n5;
        locals.var_t1_dn6 = assign24270_e37516_d_n6;
        locals.var_t1_dn7 = assign24270_e37516_d_n7;
        locals.var_t1_dn8 = assign24270_e37516_d_n8;
        locals.var_t1_dn9 = assign24270_e37516_d_n9;
        locals.var_t1_dn10 = assign24270_e37516_d_n10;
        locals.var_t1_dn11 = assign24270_e37516_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign24280_e37527, assign24280_e37527_d_n3, assign24280_e37527_d_n4, assign24280_e37527_d_n5, assign24280_e37527_d_n6, assign24280_e37527_d_n7, assign24280_e37527_d_n8, assign24280_e37527_d_n9, assign24280_e37527_d_n10, assign24280_e37527_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24280_e37524: f64 = (locals.var_t1 / locals.var_dmob_dl);
        let assign24280_e37525: f64 = (locals.var_thesat_i * assign24280_e37524);
        (assign24280_e37525, (locals.var_thesat_i * (((locals.var_t1_dn3 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn3)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn4 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn4)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn5 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn5)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn6 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn6)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn7 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn7)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn8 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn8)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn9 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn9)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn10 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn10)) / (locals.var_dmob_dl * locals.var_dmob_dl))), (locals.var_thesat_i * (((locals.var_t1_dn11 * locals.var_dmob_dl) - (locals.var_t1 * locals.var_dmob_dl_dn11)) / (locals.var_dmob_dl * locals.var_dmob_dl))),)
    } else {
        (locals.var_thesat1, locals.var_thesat1_dn3, locals.var_thesat1_dn4, locals.var_thesat1_dn5, locals.var_thesat1_dn6, locals.var_thesat1_dn7, locals.var_thesat1_dn8, locals.var_thesat1_dn9, locals.var_thesat1_dn10, locals.var_thesat1_dn11,)
    }
};
        locals.var_thesat1 = assign24280_e37527;
        locals.var_thesat1_dn3 = assign24280_e37527_d_n3;
        locals.var_thesat1_dn4 = assign24280_e37527_d_n4;
        locals.var_thesat1_dn5 = assign24280_e37527_d_n5;
        locals.var_thesat1_dn6 = assign24280_e37527_d_n6;
        locals.var_thesat1_dn7 = assign24280_e37527_d_n7;
        locals.var_thesat1_dn8 = assign24280_e37527_d_n8;
        locals.var_thesat1_dn9 = assign24280_e37527_d_n9;
        locals.var_thesat1_dn10 = assign24280_e37527_d_n10;
        locals.var_thesat1_dn11 = assign24280_e37527_d_n11;
        locals.var_thesat1_rv = 0.0;

        let (assign24290_e37540, assign24290_e37540_d_n3, assign24290_e37540_d_n4, assign24290_e37540_d_n5, assign24290_e37540_d_n6, assign24290_e37540_d_n7, assign24290_e37540_d_n8, assign24290_e37540_d_n9, assign24290_e37540_d_n10, assign24290_e37540_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24290_e37534: f64 = (locals.var_thesat1 * locals.var_thesat1);
        let assign24290_e37536: f64 = (assign24290_e37534 * locals.var_dps);
        let assign24290_e37538: f64 = (assign24290_e37536 * locals.var_dps);
        (assign24290_e37538, ((((((locals.var_thesat1_dn3 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn3)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn3)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn3)), ((((((locals.var_thesat1_dn4 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn4)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn4)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn4)), ((((((locals.var_thesat1_dn5 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn5)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn5)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn5)), ((((((locals.var_thesat1_dn6 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn6)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn6)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn6)), ((((((locals.var_thesat1_dn7 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn7)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn7)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn7)), ((((((locals.var_thesat1_dn8 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn8)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn8)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn8)), ((((((locals.var_thesat1_dn9 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn9)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn9)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn9)), ((((((locals.var_thesat1_dn10 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn10)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn10)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn10)), ((((((locals.var_thesat1_dn11 * locals.var_thesat1) + (locals.var_thesat1 * locals.var_thesat1_dn11)) * locals.var_dps) + (assign24290_e37534 * locals.var_dps_dn11)) * locals.var_dps) + (assign24290_e37536 * locals.var_dps_dn11)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn3, locals.var_zsat_dn4, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9, locals.var_zsat_dn10, locals.var_zsat_dn11,)
    }
};
        locals.var_zsat = assign24290_e37540;
        locals.var_zsat_dn3 = assign24290_e37540_d_n3;
        locals.var_zsat_dn4 = assign24290_e37540_d_n4;
        locals.var_zsat_dn5 = assign24290_e37540_d_n5;
        locals.var_zsat_dn6 = assign24290_e37540_d_n6;
        locals.var_zsat_dn7 = assign24290_e37540_d_n7;
        locals.var_zsat_dn8 = assign24290_e37540_d_n8;
        locals.var_zsat_dn9 = assign24290_e37540_d_n9;
        locals.var_zsat_dn10 = assign24290_e37540_d_n10;
        locals.var_zsat_dn11 = assign24290_e37540_d_n11;
        locals.var_zsat_rv = 0.0;

        let assign24300_e37543: f64 = (-1.0);
        let assign24300_e37544: f64 = if p.p30 == assign24300_e37543 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign24300_e37544;
        locals.var_guard536_rv = 0.0;

        let (assign24310_e37559, assign24310_e37559_d_n3, assign24310_e37559_d_n4, assign24310_e37559_d_n5, assign24310_e37559_d_n6, assign24310_e37559_d_n7, assign24310_e37559_d_n8, assign24310_e37559_d_n9, assign24310_e37559_d_n10, assign24310_e37559_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard536 != 0.0)) {
        let assign24310_e37555: f64 = (locals.var_thesat1 * locals.var_dps);
        let assign24310_e37556: f64 = (1.0 + assign24310_e37555);
        let assign24310_e37557: f64 = (locals.var_zsat / assign24310_e37556);
        (assign24310_e37557, (((locals.var_zsat_dn3 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn3 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn3)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn4 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn4 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn4)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn5 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn5 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn5)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn6 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn6 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn6)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn7 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn7 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn7)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn8 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn8 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn8)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn9 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn9 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn9)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn10 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn10 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn10)))) / (assign24310_e37556 * assign24310_e37556)), (((locals.var_zsat_dn11 * assign24310_e37556) - (locals.var_zsat * ((locals.var_thesat1_dn11 * locals.var_dps) + (locals.var_thesat1 * locals.var_dps_dn11)))) / (assign24310_e37556 * assign24310_e37556)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn3, locals.var_zsat_dn4, locals.var_zsat_dn5, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9, locals.var_zsat_dn10, locals.var_zsat_dn11,)
    }
};
        locals.var_zsat = assign24310_e37559;
        locals.var_zsat_dn3 = assign24310_e37559_d_n3;
        locals.var_zsat_dn4 = assign24310_e37559_d_n4;
        locals.var_zsat_dn5 = assign24310_e37559_d_n5;
        locals.var_zsat_dn6 = assign24310_e37559_d_n6;
        locals.var_zsat_dn7 = assign24310_e37559_d_n7;
        locals.var_zsat_dn8 = assign24310_e37559_d_n8;
        locals.var_zsat_dn9 = assign24310_e37559_d_n9;
        locals.var_zsat_dn10 = assign24310_e37559_d_n10;
        locals.var_zsat_dn11 = assign24310_e37559_d_n11;
        locals.var_zsat_rv = 0.0;

        let (assign24320_e37577, assign24320_e37577_d_n3, assign24320_e37577_d_n4, assign24320_e37577_d_n5, assign24320_e37577_d_n6, assign24320_e37577_d_n7, assign24320_e37577_d_n8, assign24320_e37577_d_n9, assign24320_e37577_d_n10, assign24320_e37577_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24320_e37570: f64 = (2.0 * locals.var_zsat);
        let assign24320_e37571: f64 = (1.0 + assign24320_e37570);
        let assign24320_e37572: f64 = (assign24320_e37571).sqrt();
        let assign24320_e37573: f64 = (1.0 + assign24320_e37572);
        let assign24320_e37574: f64 = (locals.var_dmob_dl * assign24320_e37573);
        let assign24320_e37575: f64 = (0.5 * assign24320_e37574);
        (assign24320_e37575, (0.5 * ((locals.var_dmob_dl_dn3 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn3) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn4 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn4) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn5 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn5) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn6 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn6) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn7 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn7) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn8 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn8) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn9 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn9) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn10 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn10) / (2.0 * assign24320_e37572))))), (0.5 * ((locals.var_dmob_dl_dn11 * assign24320_e37573) + (locals.var_dmob_dl * ((2.0 * locals.var_zsat_dn11) / (2.0 * assign24320_e37572))))),)
    } else {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    }
};
        locals.var_dvsat = assign24320_e37577;
        locals.var_dvsat_dn3 = assign24320_e37577_d_n3;
        locals.var_dvsat_dn4 = assign24320_e37577_d_n4;
        locals.var_dvsat_dn5 = assign24320_e37577_d_n5;
        locals.var_dvsat_dn6 = assign24320_e37577_d_n6;
        locals.var_dvsat_dn7 = assign24320_e37577_d_n7;
        locals.var_dvsat_dn8 = assign24320_e37577_d_n8;
        locals.var_dvsat_dn9 = assign24320_e37577_d_n9;
        locals.var_dvsat_dn10 = assign24320_e37577_d_n10;
        locals.var_dvsat_dn11 = assign24320_e37577_d_n11;
        locals.var_dvsat_rv = 0.0;

        let (assign24330_e37586, assign24330_e37586_d_n3, assign24330_e37586_d_n4, assign24330_e37586_d_n5, assign24330_e37586_d_n6, assign24330_e37586_d_n7, assign24330_e37586_d_n8, assign24330_e37586_d_n9, assign24330_e37586_d_n10, assign24330_e37586_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24330_e37584: f64 = (1.0 / locals.var_dvsat);
        (assign24330_e37584, (-(locals.var_dvsat_dn3 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn4 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn5 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn6 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn7 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn8 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn9 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn10 / (locals.var_dvsat * locals.var_dvsat))), (-(locals.var_dvsat_dn11 / (locals.var_dvsat * locals.var_dvsat))),)
    } else {
        (locals.var_dvsatinv, locals.var_dvsatinv_dn3, locals.var_dvsatinv_dn4, locals.var_dvsatinv_dn5, locals.var_dvsatinv_dn6, locals.var_dvsatinv_dn7, locals.var_dvsatinv_dn8, locals.var_dvsatinv_dn9, locals.var_dvsatinv_dn10, locals.var_dvsatinv_dn11,)
    }
};
        locals.var_dvsatinv = assign24330_e37586;
        locals.var_dvsatinv_dn3 = assign24330_e37586_d_n3;
        locals.var_dvsatinv_dn4 = assign24330_e37586_d_n4;
        locals.var_dvsatinv_dn5 = assign24330_e37586_d_n5;
        locals.var_dvsatinv_dn6 = assign24330_e37586_d_n6;
        locals.var_dvsatinv_dn7 = assign24330_e37586_d_n7;
        locals.var_dvsatinv_dn8 = assign24330_e37586_d_n8;
        locals.var_dvsatinv_dn9 = assign24330_e37586_d_n9;
        locals.var_dvsatinv_dn10 = assign24330_e37586_d_n10;
        locals.var_dvsatinv_dn11 = assign24330_e37586_d_n11;
        locals.var_dvsatinv_rv = 0.0;

        let (assign24340_e37593, assign24340_e37593_d_n3, assign24340_e37593_d_n4, assign24340_e37593_d_n5, assign24340_e37593_d_n6, assign24340_e37593_d_n7, assign24340_e37593_d_n8, assign24340_e37593_d_n9, assign24340_e37593_d_n10, assign24340_e37593_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign24340_e37593;
        locals.var_rsource_dn3 = assign24340_e37593_d_n3;
        locals.var_rsource_dn4 = assign24340_e37593_d_n4;
        locals.var_rsource_dn5 = assign24340_e37593_d_n5;
        locals.var_rsource_dn6 = assign24340_e37593_d_n6;
        locals.var_rsource_dn7 = assign24340_e37593_d_n7;
        locals.var_rsource_dn8 = assign24340_e37593_d_n8;
        locals.var_rsource_dn9 = assign24340_e37593_d_n9;
        locals.var_rsource_dn10 = assign24340_e37593_d_n10;
        locals.var_rsource_dn11 = assign24340_e37593_d_n11;
        locals.var_rsource_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24350_e37600, assign24350_e37600_d_n3, assign24350_e37600_d_n4, assign24350_e37600_d_n5, assign24350_e37600_d_n6, assign24350_e37600_d_n7, assign24350_e37600_d_n8, assign24350_e37600_d_n9, assign24350_e37600_d_n10, assign24350_e37600_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign24350_e37600;
        locals.var_rdrain_dn3 = assign24350_e37600_d_n3;
        locals.var_rdrain_dn4 = assign24350_e37600_d_n4;
        locals.var_rdrain_dn5 = assign24350_e37600_d_n5;
        locals.var_rdrain_dn6 = assign24350_e37600_d_n6;
        locals.var_rdrain_dn7 = assign24350_e37600_d_n7;
        locals.var_rdrain_dn8 = assign24350_e37600_d_n8;
        locals.var_rdrain_dn9 = assign24350_e37600_d_n9;
        locals.var_rdrain_dn10 = assign24350_e37600_d_n10;
        locals.var_rdrain_dn11 = assign24350_e37600_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign24360_e37609, assign24360_e37609_d_n3, assign24360_e37609_d_n4, assign24360_e37609_d_n5, assign24360_e37609_d_n6, assign24360_e37609_d_n7, assign24360_e37609_d_n8, assign24360_e37609_d_n9, assign24360_e37609_d_n10, assign24360_e37609_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24360_e37607: f64 = (locals.var_qis + locals.var_qid);
        (assign24360_e37607, (locals.var_qis_dn3 + locals.var_qid_dn3), (locals.var_qis_dn4 + locals.var_qid_dn4), (locals.var_qis_dn5 + locals.var_qid_dn5), (locals.var_qis_dn6 + locals.var_qid_dn6), (locals.var_qis_dn7 + locals.var_qid_dn7), (locals.var_qis_dn8 + locals.var_qid_dn8), (locals.var_qis_dn9 + locals.var_qid_dn9), (locals.var_qis_dn10 + locals.var_qid_dn10), (locals.var_qis_dn11 + locals.var_qid_dn11),)
    } else {
        (locals.var_qia, locals.var_qia_dn3, locals.var_qia_dn4, locals.var_qia_dn5, locals.var_qia_dn6, locals.var_qia_dn7, locals.var_qia_dn8, locals.var_qia_dn9, locals.var_qia_dn10, locals.var_qia_dn11,)
    }
};
        locals.var_qia = assign24360_e37609;
        locals.var_qia_dn3 = assign24360_e37609_d_n3;
        locals.var_qia_dn4 = assign24360_e37609_d_n4;
        locals.var_qia_dn5 = assign24360_e37609_d_n5;
        locals.var_qia_dn6 = assign24360_e37609_d_n6;
        locals.var_qia_dn7 = assign24360_e37609_d_n7;
        locals.var_qia_dn8 = assign24360_e37609_d_n8;
        locals.var_qia_dn9 = assign24360_e37609_d_n9;
        locals.var_qia_dn10 = assign24360_e37609_d_n10;
        locals.var_qia_dn11 = assign24360_e37609_d_n11;
        locals.var_qia_rv = 0.0;

        let assign24370_e37612: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard537 = assign24370_e37612;
        locals.var_guard537_rv = 0.0;

        let (assign24380_e37621, assign24380_e37621_d_n3, assign24380_e37621_d_n4, assign24380_e37621_d_n5, assign24380_e37621_d_n6, assign24380_e37621_d_n7, assign24380_e37621_d_n8, assign24380_e37621_d_n9, assign24380_e37621_d_n10, assign24380_e37621_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign24380_e37621;
        locals.var_rdsi_dn3 = assign24380_e37621_d_n3;
        locals.var_rdsi_dn4 = assign24380_e37621_d_n4;
        locals.var_rdsi_dn5 = assign24380_e37621_d_n5;
        locals.var_rdsi_dn6 = assign24380_e37621_d_n6;
        locals.var_rdsi_dn7 = assign24380_e37621_d_n7;
        locals.var_rdsi_dn8 = assign24380_e37621_d_n8;
        locals.var_rdsi_dn9 = assign24380_e37621_d_n9;
        locals.var_rdsi_dn10 = assign24380_e37621_d_n10;
        locals.var_rdsi_dn11 = assign24380_e37621_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign24390_e37630, assign24390_e37630_d_n3, assign24390_e37630_d_n4, assign24390_e37630_d_n5, assign24390_e37630_d_n6, assign24390_e37630_d_n7, assign24390_e37630_d_n8, assign24390_e37630_d_n9, assign24390_e37630_d_n10, assign24390_e37630_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign24390_e37630;
        locals.var_dr_dn3 = assign24390_e37630_d_n3;
        locals.var_dr_dn4 = assign24390_e37630_d_n4;
        locals.var_dr_dn5 = assign24390_e37630_d_n5;
        locals.var_dr_dn6 = assign24390_e37630_d_n6;
        locals.var_dr_dn7 = assign24390_e37630_d_n7;
        locals.var_dr_dn8 = assign24390_e37630_d_n8;
        locals.var_dr_dn9 = assign24390_e37630_d_n9;
        locals.var_dr_dn10 = assign24390_e37630_d_n10;
        locals.var_dr_dn11 = assign24390_e37630_d_n11;
        locals.var_dr_rv = 0.0;

        let (assign24400_e37641, assign24400_e37641_d_n3, assign24400_e37641_d_n4, assign24400_e37641_d_n5, assign24400_e37641_d_n6, assign24400_e37641_d_n7, assign24400_e37641_d_n8, assign24400_e37641_d_n9, assign24400_e37641_d_n10, assign24400_e37641_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24400_e37639: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign24400_e37639, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24400_e37641;
        locals.var_t2_dn3 = assign24400_e37641_d_n3;
        locals.var_t2_dn4 = assign24400_e37641_d_n4;
        locals.var_t2_dn5 = assign24400_e37641_d_n5;
        locals.var_t2_dn6 = assign24400_e37641_d_n6;
        locals.var_t2_dn7 = assign24400_e37641_d_n7;
        locals.var_t2_dn8 = assign24400_e37641_d_n8;
        locals.var_t2_dn9 = assign24400_e37641_d_n9;
        locals.var_t2_dn10 = assign24400_e37641_d_n10;
        locals.var_t2_dn11 = assign24400_e37641_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign24410_e37655, assign24410_e37655_d_n3, assign24410_e37655_d_n4, assign24410_e37655_d_n5, assign24410_e37655_d_n6, assign24410_e37655_d_n7, assign24410_e37655_d_n8, assign24410_e37655_d_n9, assign24410_e37655_d_n10, assign24410_e37655_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24410_e37650: f64 = (locals.var_t2 * locals.var_t2);
        let assign24410_e37652: f64 = (assign24410_e37650 + 0.01);
        let assign24410_e37653: f64 = (assign24410_e37652).sqrt();
        (assign24410_e37653, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24410_e37653)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24410_e37653)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24410_e37655;
        locals.var_t3_dn3 = assign24410_e37655_d_n3;
        locals.var_t3_dn4 = assign24410_e37655_d_n4;
        locals.var_t3_dn5 = assign24410_e37655_d_n5;
        locals.var_t3_dn6 = assign24410_e37655_d_n6;
        locals.var_t3_dn7 = assign24410_e37655_d_n7;
        locals.var_t3_dn8 = assign24410_e37655_d_n8;
        locals.var_t3_dn9 = assign24410_e37655_d_n9;
        locals.var_t3_dn10 = assign24410_e37655_d_n10;
        locals.var_t3_dn11 = assign24410_e37655_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign24420_e37668, assign24420_e37668_d_n3, assign24420_e37668_d_n4, assign24420_e37668_d_n5, assign24420_e37668_d_n6, assign24420_e37668_d_n7, assign24420_e37668_d_n8, assign24420_e37668_d_n9, assign24420_e37668_d_n10, assign24420_e37668_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24420_e37665: f64 = (locals.var_t2 + locals.var_t3);
        let assign24420_e37666: f64 = (0.5 * assign24420_e37665);
        (assign24420_e37666, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign24420_e37668;
        locals.var_vgs_eff_dn3 = assign24420_e37668_d_n3;
        locals.var_vgs_eff_dn4 = assign24420_e37668_d_n4;
        locals.var_vgs_eff_dn5 = assign24420_e37668_d_n5;
        locals.var_vgs_eff_dn6 = assign24420_e37668_d_n6;
        locals.var_vgs_eff_dn7 = assign24420_e37668_d_n7;
        locals.var_vgs_eff_dn8 = assign24420_e37668_d_n8;
        locals.var_vgs_eff_dn9 = assign24420_e37668_d_n9;
        locals.var_vgs_eff_dn10 = assign24420_e37668_d_n10;
        locals.var_vgs_eff_dn11 = assign24420_e37668_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let (assign24430_e37681, assign24430_e37681_d_n3, assign24430_e37681_d_n4, assign24430_e37681_d_n5, assign24430_e37681_d_n6, assign24430_e37681_d_n7, assign24430_e37681_d_n8, assign24430_e37681_d_n9, assign24430_e37681_d_n10, assign24430_e37681_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24430_e37678: f64 = (locals.var_prwg_i * locals.var_vgs_eff);
        let assign24430_e37679: f64 = (1.0 + assign24430_e37678);
        (assign24430_e37679, (locals.var_prwg_i * locals.var_vgs_eff_dn3), (locals.var_prwg_i * locals.var_vgs_eff_dn4), (locals.var_prwg_i * locals.var_vgs_eff_dn5), (locals.var_prwg_i * locals.var_vgs_eff_dn6), (locals.var_prwg_i * locals.var_vgs_eff_dn7), (locals.var_prwg_i * locals.var_vgs_eff_dn8), (locals.var_prwg_i * locals.var_vgs_eff_dn9), (locals.var_prwg_i * locals.var_vgs_eff_dn10), (locals.var_prwg_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign24430_e37681;
        locals.var_t5_dn3 = assign24430_e37681_d_n3;
        locals.var_t5_dn4 = assign24430_e37681_d_n4;
        locals.var_t5_dn5 = assign24430_e37681_d_n5;
        locals.var_t5_dn6 = assign24430_e37681_d_n6;
        locals.var_t5_dn7 = assign24430_e37681_d_n7;
        locals.var_t5_dn8 = assign24430_e37681_d_n8;
        locals.var_t5_dn9 = assign24430_e37681_d_n9;
        locals.var_t5_dn10 = assign24430_e37681_d_n10;
        locals.var_t5_dn11 = assign24430_e37681_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign24440_e37696, assign24440_e37696_d_n3, assign24440_e37696_d_n4, assign24440_e37696_d_n5, assign24440_e37696_d_n6, assign24440_e37696_d_n7, assign24440_e37696_d_n8, assign24440_e37696_d_n9, assign24440_e37696_d_n10, assign24440_e37696_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24440_e37690: f64 = (1.0 / locals.var_t5);
        let assign24440_e37693: f64 = (locals.var_prwb_i * locals.var_vsb_noswap);
        let assign24440_e37694: f64 = (assign24440_e37690 + assign24440_e37693);
        (assign24440_e37694, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign24440_e37696;
        locals.var_t6_dn3 = assign24440_e37696_d_n3;
        locals.var_t6_dn4 = assign24440_e37696_d_n4;
        locals.var_t6_dn5 = assign24440_e37696_d_n5;
        locals.var_t6_dn6 = assign24440_e37696_d_n6;
        locals.var_t6_dn7 = assign24440_e37696_d_n7;
        locals.var_t6_dn8 = assign24440_e37696_d_n8;
        locals.var_t6_dn9 = assign24440_e37696_d_n9;
        locals.var_t6_dn10 = assign24440_e37696_d_n10;
        locals.var_t6_dn11 = assign24440_e37696_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign24450_e37714, assign24450_e37714_d_n3, assign24450_e37714_d_n4, assign24450_e37714_d_n5, assign24450_e37714_d_n6, assign24450_e37714_d_n7, assign24450_e37714_d_n8, assign24450_e37714_d_n9, assign24450_e37714_d_n10, assign24450_e37714_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24450_e37707: f64 = (locals.var_t6 * locals.var_t6);
        let assign24450_e37709: f64 = (assign24450_e37707 + 0.01);
        let assign24450_e37710: f64 = (assign24450_e37709).sqrt();
        let assign24450_e37711: f64 = (locals.var_t6 + assign24450_e37710);
        let assign24450_e37712: f64 = (0.5 * assign24450_e37711);
        (assign24450_e37712, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign24450_e37710)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign24450_e37710)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign24450_e37714;
        locals.var_t4_dn3 = assign24450_e37714_d_n3;
        locals.var_t4_dn4 = assign24450_e37714_d_n4;
        locals.var_t4_dn5 = assign24450_e37714_d_n5;
        locals.var_t4_dn6 = assign24450_e37714_d_n6;
        locals.var_t4_dn7 = assign24450_e37714_d_n7;
        locals.var_t4_dn8 = assign24450_e37714_d_n8;
        locals.var_t4_dn9 = assign24450_e37714_d_n9;
        locals.var_t4_dn10 = assign24450_e37714_d_n10;
        locals.var_t4_dn11 = assign24450_e37714_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign24460_e37733, assign24460_e37733_d_n3, assign24460_e37733_d_n4, assign24460_e37733_d_n5, assign24460_e37733_d_n6, assign24460_e37733_d_n7, assign24460_e37733_d_n8, assign24460_e37733_d_n9, assign24460_e37733_d_n10, assign24460_e37733_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24460_e37726: f64 = (locals.var_rsw_i * locals.var_t4);
        let assign24460_e37727: f64 = (locals.var_rswmin_i + assign24460_e37726);
        let assign24460_e37729: f64 = (assign24460_e37727 * locals.var_weffwrfactor);
        let assign24460_e37730: f64 = (locals.var_rsourcegeo + assign24460_e37729);
        let assign24460_e37731: f64 = (locals.var_rdstemp * assign24460_e37730);
        (assign24460_e37731, (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign24460_e37730) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign24460_e37730) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign24460_e37733;
        locals.var_rsource_dn3 = assign24460_e37733_d_n3;
        locals.var_rsource_dn4 = assign24460_e37733_d_n4;
        locals.var_rsource_dn5 = assign24460_e37733_d_n5;
        locals.var_rsource_dn6 = assign24460_e37733_d_n6;
        locals.var_rsource_dn7 = assign24460_e37733_d_n7;
        locals.var_rsource_dn8 = assign24460_e37733_d_n8;
        locals.var_rsource_dn9 = assign24460_e37733_d_n9;
        locals.var_rsource_dn10 = assign24460_e37733_d_n10;
        locals.var_rsource_dn11 = assign24460_e37733_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign24470_e37744, assign24470_e37744_d_n3, assign24470_e37744_d_n4, assign24470_e37744_d_n5, assign24470_e37744_d_n6, assign24470_e37744_d_n7, assign24470_e37744_d_n8, assign24470_e37744_d_n9, assign24470_e37744_d_n10, assign24470_e37744_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24470_e37742: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign24470_e37742, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24470_e37744;
        locals.var_t2_dn3 = assign24470_e37744_d_n3;
        locals.var_t2_dn4 = assign24470_e37744_d_n4;
        locals.var_t2_dn5 = assign24470_e37744_d_n5;
        locals.var_t2_dn6 = assign24470_e37744_d_n6;
        locals.var_t2_dn7 = assign24470_e37744_d_n7;
        locals.var_t2_dn8 = assign24470_e37744_d_n8;
        locals.var_t2_dn9 = assign24470_e37744_d_n9;
        locals.var_t2_dn10 = assign24470_e37744_d_n10;
        locals.var_t2_dn11 = assign24470_e37744_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign24480_e37758, assign24480_e37758_d_n3, assign24480_e37758_d_n4, assign24480_e37758_d_n5, assign24480_e37758_d_n6, assign24480_e37758_d_n7, assign24480_e37758_d_n8, assign24480_e37758_d_n9, assign24480_e37758_d_n10, assign24480_e37758_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24480_e37753: f64 = (locals.var_t2 * locals.var_t2);
        let assign24480_e37755: f64 = (assign24480_e37753 + 0.01);
        let assign24480_e37756: f64 = (assign24480_e37755).sqrt();
        (assign24480_e37756, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24480_e37756)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24480_e37756)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24480_e37758;
        locals.var_t3_dn3 = assign24480_e37758_d_n3;
        locals.var_t3_dn4 = assign24480_e37758_d_n4;
        locals.var_t3_dn5 = assign24480_e37758_d_n5;
        locals.var_t3_dn6 = assign24480_e37758_d_n6;
        locals.var_t3_dn7 = assign24480_e37758_d_n7;
        locals.var_t3_dn8 = assign24480_e37758_d_n8;
        locals.var_t3_dn9 = assign24480_e37758_d_n9;
        locals.var_t3_dn10 = assign24480_e37758_d_n10;
        locals.var_t3_dn11 = assign24480_e37758_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign24490_e37771, assign24490_e37771_d_n3, assign24490_e37771_d_n4, assign24490_e37771_d_n5, assign24490_e37771_d_n6, assign24490_e37771_d_n7, assign24490_e37771_d_n8, assign24490_e37771_d_n9, assign24490_e37771_d_n10, assign24490_e37771_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24490_e37768: f64 = (locals.var_t2 + locals.var_t3);
        let assign24490_e37769: f64 = (0.5 * assign24490_e37768);
        (assign24490_e37769, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign24490_e37771;
        locals.var_vgd_eff_dn3 = assign24490_e37771_d_n3;
        locals.var_vgd_eff_dn4 = assign24490_e37771_d_n4;
        locals.var_vgd_eff_dn5 = assign24490_e37771_d_n5;
        locals.var_vgd_eff_dn6 = assign24490_e37771_d_n6;
        locals.var_vgd_eff_dn7 = assign24490_e37771_d_n7;
        locals.var_vgd_eff_dn8 = assign24490_e37771_d_n8;
        locals.var_vgd_eff_dn9 = assign24490_e37771_d_n9;
        locals.var_vgd_eff_dn10 = assign24490_e37771_d_n10;
        locals.var_vgd_eff_dn11 = assign24490_e37771_d_n11;
        locals.var_vgd_eff_rv = 0.0;

        let (assign24500_e37784, assign24500_e37784_d_n3, assign24500_e37784_d_n4, assign24500_e37784_d_n5, assign24500_e37784_d_n6, assign24500_e37784_d_n7, assign24500_e37784_d_n8, assign24500_e37784_d_n9, assign24500_e37784_d_n10, assign24500_e37784_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24500_e37781: f64 = (locals.var_prwg_i * locals.var_vgd_eff);
        let assign24500_e37782: f64 = (1.0 + assign24500_e37781);
        (assign24500_e37782, (locals.var_prwg_i * locals.var_vgd_eff_dn3), (locals.var_prwg_i * locals.var_vgd_eff_dn4), (locals.var_prwg_i * locals.var_vgd_eff_dn5), (locals.var_prwg_i * locals.var_vgd_eff_dn6), (locals.var_prwg_i * locals.var_vgd_eff_dn7), (locals.var_prwg_i * locals.var_vgd_eff_dn8), (locals.var_prwg_i * locals.var_vgd_eff_dn9), (locals.var_prwg_i * locals.var_vgd_eff_dn10), (locals.var_prwg_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign24500_e37784;
        locals.var_t5_dn3 = assign24500_e37784_d_n3;
        locals.var_t5_dn4 = assign24500_e37784_d_n4;
        locals.var_t5_dn5 = assign24500_e37784_d_n5;
        locals.var_t5_dn6 = assign24500_e37784_d_n6;
        locals.var_t5_dn7 = assign24500_e37784_d_n7;
        locals.var_t5_dn8 = assign24500_e37784_d_n8;
        locals.var_t5_dn9 = assign24500_e37784_d_n9;
        locals.var_t5_dn10 = assign24500_e37784_d_n10;
        locals.var_t5_dn11 = assign24500_e37784_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign24510_e37799, assign24510_e37799_d_n3, assign24510_e37799_d_n4, assign24510_e37799_d_n5, assign24510_e37799_d_n6, assign24510_e37799_d_n7, assign24510_e37799_d_n8, assign24510_e37799_d_n9, assign24510_e37799_d_n10, assign24510_e37799_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24510_e37793: f64 = (1.0 / locals.var_t5);
        let assign24510_e37796: f64 = (locals.var_prwb_i * locals.var_vdb_noswap);
        let assign24510_e37797: f64 = (assign24510_e37793 + assign24510_e37796);
        (assign24510_e37797, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign24510_e37799;
        locals.var_t6_dn3 = assign24510_e37799_d_n3;
        locals.var_t6_dn4 = assign24510_e37799_d_n4;
        locals.var_t6_dn5 = assign24510_e37799_d_n5;
        locals.var_t6_dn6 = assign24510_e37799_d_n6;
        locals.var_t6_dn7 = assign24510_e37799_d_n7;
        locals.var_t6_dn8 = assign24510_e37799_d_n8;
        locals.var_t6_dn9 = assign24510_e37799_d_n9;
        locals.var_t6_dn10 = assign24510_e37799_d_n10;
        locals.var_t6_dn11 = assign24510_e37799_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign24520_e37817, assign24520_e37817_d_n3, assign24520_e37817_d_n4, assign24520_e37817_d_n5, assign24520_e37817_d_n6, assign24520_e37817_d_n7, assign24520_e37817_d_n8, assign24520_e37817_d_n9, assign24520_e37817_d_n10, assign24520_e37817_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24520_e37810: f64 = (locals.var_t6 * locals.var_t6);
        let assign24520_e37812: f64 = (assign24520_e37810 + 0.01);
        let assign24520_e37813: f64 = (assign24520_e37812).sqrt();
        let assign24520_e37814: f64 = (locals.var_t6 + assign24520_e37813);
        let assign24520_e37815: f64 = (0.5 * assign24520_e37814);
        (assign24520_e37815, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign24520_e37813)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign24520_e37813)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign24520_e37817;
        locals.var_t4_dn3 = assign24520_e37817_d_n3;
        locals.var_t4_dn4 = assign24520_e37817_d_n4;
        locals.var_t4_dn5 = assign24520_e37817_d_n5;
        locals.var_t4_dn6 = assign24520_e37817_d_n6;
        locals.var_t4_dn7 = assign24520_e37817_d_n7;
        locals.var_t4_dn8 = assign24520_e37817_d_n8;
        locals.var_t4_dn9 = assign24520_e37817_d_n9;
        locals.var_t4_dn10 = assign24520_e37817_d_n10;
        locals.var_t4_dn11 = assign24520_e37817_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign24530_e37836, assign24530_e37836_d_n3, assign24530_e37836_d_n4, assign24530_e37836_d_n5, assign24530_e37836_d_n6, assign24530_e37836_d_n7, assign24530_e37836_d_n8, assign24530_e37836_d_n9, assign24530_e37836_d_n10, assign24530_e37836_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 != 0.0)) {
        let assign24530_e37829: f64 = (locals.var_rdw_i * locals.var_t4);
        let assign24530_e37830: f64 = (locals.var_rdwmin_i + assign24530_e37829);
        let assign24530_e37832: f64 = (assign24530_e37830 * locals.var_weffwrfactor);
        let assign24530_e37833: f64 = (locals.var_rdraingeo + assign24530_e37832);
        let assign24530_e37834: f64 = (locals.var_rdstemp * assign24530_e37833);
        (assign24530_e37834, (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign24530_e37833) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign24530_e37833) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign24530_e37836;
        locals.var_rdrain_dn3 = assign24530_e37836_d_n3;
        locals.var_rdrain_dn4 = assign24530_e37836_d_n4;
        locals.var_rdrain_dn5 = assign24530_e37836_d_n5;
        locals.var_rdrain_dn6 = assign24530_e37836_d_n6;
        locals.var_rdrain_dn7 = assign24530_e37836_d_n7;
        locals.var_rdrain_dn8 = assign24530_e37836_d_n8;
        locals.var_rdrain_dn9 = assign24530_e37836_d_n9;
        locals.var_rdrain_dn10 = assign24530_e37836_d_n10;
        locals.var_rdrain_dn11 = assign24530_e37836_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign24540_e37850, assign24540_e37850_d_n3, assign24540_e37850_d_n4, assign24540_e37850_d_n5, assign24540_e37850_d_n6, assign24540_e37850_d_n7, assign24540_e37850_d_n8, assign24540_e37850_d_n9, assign24540_e37850_d_n10, assign24540_e37850_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24540_e37847: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign24540_e37848: f64 = (1.0 + assign24540_e37847);
        (assign24540_e37848, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8), (locals.var_prwg_i * locals.var_qia_dn9), (locals.var_prwg_i * locals.var_qia_dn10), (locals.var_prwg_i * locals.var_qia_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24540_e37850;
        locals.var_t0_dn3 = assign24540_e37850_d_n3;
        locals.var_t0_dn4 = assign24540_e37850_d_n4;
        locals.var_t0_dn5 = assign24540_e37850_d_n5;
        locals.var_t0_dn6 = assign24540_e37850_d_n6;
        locals.var_t0_dn7 = assign24540_e37850_d_n7;
        locals.var_t0_dn8 = assign24540_e37850_d_n8;
        locals.var_t0_dn9 = assign24540_e37850_d_n9;
        locals.var_t0_dn10 = assign24540_e37850_d_n10;
        locals.var_t0_dn11 = assign24540_e37850_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24550_e37864, assign24550_e37864_d_n3, assign24550_e37864_d_n4, assign24550_e37864_d_n5, assign24550_e37864_d_n6, assign24550_e37864_d_n7, assign24550_e37864_d_n8, assign24550_e37864_d_n9, assign24550_e37864_d_n10, assign24550_e37864_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24550_e37861: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign24550_e37862: f64 = (locals.var_prwb_i * assign24550_e37861);
        (assign24550_e37862, (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24550_e37864;
        locals.var_t1_dn3 = assign24550_e37864_d_n3;
        locals.var_t1_dn4 = assign24550_e37864_d_n4;
        locals.var_t1_dn5 = assign24550_e37864_d_n5;
        locals.var_t1_dn6 = assign24550_e37864_d_n6;
        locals.var_t1_dn7 = assign24550_e37864_d_n7;
        locals.var_t1_dn8 = assign24550_e37864_d_n8;
        locals.var_t1_dn9 = assign24550_e37864_d_n9;
        locals.var_t1_dn10 = assign24550_e37864_d_n10;
        locals.var_t1_dn11 = assign24550_e37864_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign24560_e37878, assign24560_e37878_d_n3, assign24560_e37878_d_n4, assign24560_e37878_d_n5, assign24560_e37878_d_n6, assign24560_e37878_d_n7, assign24560_e37878_d_n8, assign24560_e37878_d_n9, assign24560_e37878_d_n10, assign24560_e37878_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24560_e37874: f64 = (1.0 / locals.var_t0);
        let assign24560_e37876: f64 = (assign24560_e37874 + locals.var_t1);
        (assign24560_e37876, ((-(locals.var_t0_dn3 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn3), ((-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn4), ((-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn5), ((-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn6), ((-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn7), ((-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn8), ((-(locals.var_t0_dn9 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn9), ((-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn10), ((-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24560_e37878;
        locals.var_t2_dn3 = assign24560_e37878_d_n3;
        locals.var_t2_dn4 = assign24560_e37878_d_n4;
        locals.var_t2_dn5 = assign24560_e37878_d_n5;
        locals.var_t2_dn6 = assign24560_e37878_d_n6;
        locals.var_t2_dn7 = assign24560_e37878_d_n7;
        locals.var_t2_dn8 = assign24560_e37878_d_n8;
        locals.var_t2_dn9 = assign24560_e37878_d_n9;
        locals.var_t2_dn10 = assign24560_e37878_d_n10;
        locals.var_t2_dn11 = assign24560_e37878_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign24570_e37897, assign24570_e37897_d_n3, assign24570_e37897_d_n4, assign24570_e37897_d_n5, assign24570_e37897_d_n6, assign24570_e37897_d_n7, assign24570_e37897_d_n8, assign24570_e37897_d_n9, assign24570_e37897_d_n10, assign24570_e37897_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24570_e37890: f64 = (locals.var_t2 * locals.var_t2);
        let assign24570_e37892: f64 = (assign24570_e37890 + 0.01);
        let assign24570_e37893: f64 = (assign24570_e37892).sqrt();
        let assign24570_e37894: f64 = (locals.var_t2 + assign24570_e37893);
        let assign24570_e37895: f64 = (0.5 * assign24570_e37894);
        (assign24570_e37895, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24570_e37893)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24570_e37893)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24570_e37897;
        locals.var_t3_dn3 = assign24570_e37897_d_n3;
        locals.var_t3_dn4 = assign24570_e37897_d_n4;
        locals.var_t3_dn5 = assign24570_e37897_d_n5;
        locals.var_t3_dn6 = assign24570_e37897_d_n6;
        locals.var_t3_dn7 = assign24570_e37897_d_n7;
        locals.var_t3_dn8 = assign24570_e37897_d_n8;
        locals.var_t3_dn9 = assign24570_e37897_d_n9;
        locals.var_t3_dn10 = assign24570_e37897_d_n10;
        locals.var_t3_dn11 = assign24570_e37897_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign24580_e37917, assign24580_e37917_d_n3, assign24580_e37917_d_n4, assign24580_e37917_d_n5, assign24580_e37917_d_n6, assign24580_e37917_d_n7, assign24580_e37917_d_n8, assign24580_e37917_d_n9, assign24580_e37917_d_n10, assign24580_e37917_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24580_e37909: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign24580_e37910: f64 = (locals.var_rdswmin_i + assign24580_e37909);
        let assign24580_e37911: f64 = (locals.var_rdstemp * assign24580_e37910);
        let assign24580_e37913: f64 = (assign24580_e37911 * locals.var_weffwrfactor);
        let assign24580_e37915: f64 = (assign24580_e37913 * p.p2);
        (assign24580_e37915, (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn3)) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn4 * assign24580_e37910) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn4))) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn5 * assign24580_e37910) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn5))) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn6)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn7)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn8)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn9)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn10)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn11)) * locals.var_weffwrfactor) * p.p2),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign24580_e37917;
        locals.var_rdsi_dn3 = assign24580_e37917_d_n3;
        locals.var_rdsi_dn4 = assign24580_e37917_d_n4;
        locals.var_rdsi_dn5 = assign24580_e37917_d_n5;
        locals.var_rdsi_dn6 = assign24580_e37917_d_n6;
        locals.var_rdsi_dn7 = assign24580_e37917_d_n7;
        locals.var_rdsi_dn8 = assign24580_e37917_d_n8;
        locals.var_rdsi_dn9 = assign24580_e37917_d_n9;
        locals.var_rdsi_dn10 = assign24580_e37917_d_n10;
        locals.var_rdsi_dn11 = assign24580_e37917_d_n11;
        locals.var_rdsi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24590_e37927, assign24590_e37927_d_n3, assign24590_e37927_d_n4, assign24590_e37927_d_n5, assign24590_e37927_d_n6, assign24590_e37927_d_n7, assign24590_e37927_d_n8, assign24590_e37927_d_n9, assign24590_e37927_d_n10, assign24590_e37927_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        (locals.var_rdraingeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign24590_e37927;
        locals.var_rdrain_dn3 = assign24590_e37927_d_n3;
        locals.var_rdrain_dn4 = assign24590_e37927_d_n4;
        locals.var_rdrain_dn5 = assign24590_e37927_d_n5;
        locals.var_rdrain_dn6 = assign24590_e37927_d_n6;
        locals.var_rdrain_dn7 = assign24590_e37927_d_n7;
        locals.var_rdrain_dn8 = assign24590_e37927_d_n8;
        locals.var_rdrain_dn9 = assign24590_e37927_d_n9;
        locals.var_rdrain_dn10 = assign24590_e37927_d_n10;
        locals.var_rdrain_dn11 = assign24590_e37927_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign24600_e37937, assign24600_e37937_d_n3, assign24600_e37937_d_n4, assign24600_e37937_d_n5, assign24600_e37937_d_n6, assign24600_e37937_d_n7, assign24600_e37937_d_n8, assign24600_e37937_d_n9, assign24600_e37937_d_n10, assign24600_e37937_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        (locals.var_rsourcegeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign24600_e37937;
        locals.var_rsource_dn3 = assign24600_e37937_d_n3;
        locals.var_rsource_dn4 = assign24600_e37937_d_n4;
        locals.var_rsource_dn5 = assign24600_e37937_d_n5;
        locals.var_rsource_dn6 = assign24600_e37937_d_n6;
        locals.var_rsource_dn7 = assign24600_e37937_d_n7;
        locals.var_rsource_dn8 = assign24600_e37937_d_n8;
        locals.var_rsource_dn9 = assign24600_e37937_d_n9;
        locals.var_rsource_dn10 = assign24600_e37937_d_n10;
        locals.var_rsource_dn11 = assign24600_e37937_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign24610_e37961, assign24610_e37961_d_n3, assign24610_e37961_d_n4, assign24610_e37961_d_n5, assign24610_e37961_d_n6, assign24610_e37961_d_n7, assign24610_e37961_d_n8, assign24610_e37961_d_n9, assign24610_e37961_d_n10, assign24610_e37961_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) {
        let assign24610_e37948: f64 = (locals.var_u0_a / locals.var_dvsat);
        let assign24610_e37950: f64 = (assign24610_e37948 * locals.var_cox);
        let assign24610_e37952: f64 = (assign24610_e37950 * locals.var_weff);
        let assign24610_e37954: f64 = (assign24610_e37952 / locals.var_leff);
        let assign24610_e37956: f64 = (assign24610_e37954 * locals.var_qia);
        let assign24610_e37958: f64 = (assign24610_e37956 * locals.var_rdsi);
        let assign24610_e37959: f64 = (1.0 + assign24610_e37958);
        (assign24610_e37959, ((((((((((locals.var_u0_a_dn3 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24610_e37954 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign24610_e37956 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign24610_e37961;
        locals.var_dr_dn3 = assign24610_e37961_d_n3;
        locals.var_dr_dn4 = assign24610_e37961_d_n4;
        locals.var_dr_dn5 = assign24610_e37961_d_n5;
        locals.var_dr_dn6 = assign24610_e37961_d_n6;
        locals.var_dr_dn7 = assign24610_e37961_d_n7;
        locals.var_dr_dn8 = assign24610_e37961_d_n8;
        locals.var_dr_dn9 = assign24610_e37961_d_n9;
        locals.var_dr_dn10 = assign24610_e37961_d_n10;
        locals.var_dr_dn11 = assign24610_e37961_d_n11;
        locals.var_dr_rv = 0.0;

        let assign24620_e37964: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign24620_e37964;
        locals.var_guard538_rv = 0.0;

        let (assign24630_e37990, assign24630_e37990_d_n3, assign24630_e37990_d_n4, assign24630_e37990_d_n5, assign24630_e37990_d_n6, assign24630_e37990_d_n7, assign24630_e37990_d_n8, assign24630_e37990_d_n9, assign24630_e37990_d_n10, assign24630_e37990_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign24630_e37979: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign24630_e37980: f64 = (locals.var_rdswmin_i + assign24630_e37979);
        let assign24630_e37982: f64 = (assign24630_e37980 * locals.var_weffwrfactor);
        let assign24630_e37984: f64 = (assign24630_e37982 * p.p2);
        let assign24630_e37985: f64 = (locals.var_rsourcegeo + assign24630_e37984);
        let assign24630_e37987: f64 = (assign24630_e37985 + locals.var_rdraingeo);
        let assign24630_e37988: f64 = (locals.var_rdstemp * assign24630_e37987);
        (assign24630_e37988, (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2)), ((locals.var_rdstemp_dn4 * assign24630_e37987) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2))), ((locals.var_rdstemp_dn5 * assign24630_e37987) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2))), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2)),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign24630_e37990;
        locals.var_rdsi_dn3 = assign24630_e37990_d_n3;
        locals.var_rdsi_dn4 = assign24630_e37990_d_n4;
        locals.var_rdsi_dn5 = assign24630_e37990_d_n5;
        locals.var_rdsi_dn6 = assign24630_e37990_d_n6;
        locals.var_rdsi_dn7 = assign24630_e37990_d_n7;
        locals.var_rdsi_dn8 = assign24630_e37990_d_n8;
        locals.var_rdsi_dn9 = assign24630_e37990_d_n9;
        locals.var_rdsi_dn10 = assign24630_e37990_d_n10;
        locals.var_rdsi_dn11 = assign24630_e37990_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign24640_e38002, assign24640_e38002_d_n3, assign24640_e38002_d_n4, assign24640_e38002_d_n5, assign24640_e38002_d_n6, assign24640_e38002_d_n7, assign24640_e38002_d_n8, assign24640_e38002_d_n9, assign24640_e38002_d_n10, assign24640_e38002_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign24640_e38002;
        locals.var_rdrain_dn3 = assign24640_e38002_d_n3;
        locals.var_rdrain_dn4 = assign24640_e38002_d_n4;
        locals.var_rdrain_dn5 = assign24640_e38002_d_n5;
        locals.var_rdrain_dn6 = assign24640_e38002_d_n6;
        locals.var_rdrain_dn7 = assign24640_e38002_d_n7;
        locals.var_rdrain_dn8 = assign24640_e38002_d_n8;
        locals.var_rdrain_dn9 = assign24640_e38002_d_n9;
        locals.var_rdrain_dn10 = assign24640_e38002_d_n10;
        locals.var_rdrain_dn11 = assign24640_e38002_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign24650_e38014, assign24650_e38014_d_n3, assign24650_e38014_d_n4, assign24650_e38014_d_n5, assign24650_e38014_d_n6, assign24650_e38014_d_n7, assign24650_e38014_d_n8, assign24650_e38014_d_n9, assign24650_e38014_d_n10, assign24650_e38014_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign24650_e38014;
        locals.var_rsource_dn3 = assign24650_e38014_d_n3;
        locals.var_rsource_dn4 = assign24650_e38014_d_n4;
        locals.var_rsource_dn5 = assign24650_e38014_d_n5;
        locals.var_rsource_dn6 = assign24650_e38014_d_n6;
        locals.var_rsource_dn7 = assign24650_e38014_d_n7;
        locals.var_rsource_dn8 = assign24650_e38014_d_n8;
        locals.var_rsource_dn9 = assign24650_e38014_d_n9;
        locals.var_rsource_dn10 = assign24650_e38014_d_n10;
        locals.var_rsource_dn11 = assign24650_e38014_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign24660_e38040, assign24660_e38040_d_n3, assign24660_e38040_d_n4, assign24660_e38040_d_n5, assign24660_e38040_d_n6, assign24660_e38040_d_n7, assign24660_e38040_d_n8, assign24660_e38040_d_n9, assign24660_e38040_d_n10, assign24660_e38040_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
        let assign24660_e38027: f64 = (locals.var_u0_a / locals.var_dvsat);
        let assign24660_e38029: f64 = (assign24660_e38027 * locals.var_cox);
        let assign24660_e38031: f64 = (assign24660_e38029 * locals.var_weff);
        let assign24660_e38033: f64 = (assign24660_e38031 / locals.var_leff);
        let assign24660_e38035: f64 = (assign24660_e38033 * locals.var_qia);
        let assign24660_e38037: f64 = (assign24660_e38035 * locals.var_rdsi);
        let assign24660_e38038: f64 = (1.0 + assign24660_e38037);
        (assign24660_e38038, ((((((((((locals.var_u0_a_dn3 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * locals.var_dvsat) - (locals.var_u0_a * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign24660_e38033 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign24660_e38035 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign24660_e38040;
        locals.var_dr_dn3 = assign24660_e38040_d_n3;
        locals.var_dr_dn4 = assign24660_e38040_d_n4;
        locals.var_dr_dn5 = assign24660_e38040_d_n5;
        locals.var_dr_dn6 = assign24660_e38040_d_n6;
        locals.var_dr_dn7 = assign24660_e38040_d_n7;
        locals.var_dr_dn8 = assign24660_e38040_d_n8;
        locals.var_dr_dn9 = assign24660_e38040_d_n9;
        locals.var_dr_dn10 = assign24660_e38040_d_n10;
        locals.var_dr_dn11 = assign24660_e38040_d_n11;
        locals.var_dr_rv = 0.0;

        let (assign24670_e38057, assign24670_e38057_d_n3, assign24670_e38057_d_n4, assign24670_e38057_d_n5, assign24670_e38057_d_n6, assign24670_e38057_d_n7, assign24670_e38057_d_n8, assign24670_e38057_d_n9, assign24670_e38057_d_n10, assign24670_e38057_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24670_e38050: f64 = (2.0 * locals.var_n);
        let assign24670_e38052: f64 = (assign24670_e38050 * locals.var_vtm);
        let assign24670_e38053: f64 = (locals.var_qia + assign24670_e38052);
        let assign24670_e38054: f64 = (locals.var_a2_t / assign24670_e38053);
        let assign24670_e38055: f64 = (locals.var_a1_t + assign24670_e38054);
        (assign24670_e38055, (-((locals.var_a2_t * (locals.var_qia_dn3 + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (locals.var_a1_t_dn4 + (((locals.var_a2_t_dn4 * assign24670_e38053) - (locals.var_a2_t * (locals.var_qia_dn4 + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign24670_e38050 * locals.var_vtm_dn4))))) / (assign24670_e38053 * assign24670_e38053))), (locals.var_a1_t_dn5 + (((locals.var_a2_t_dn5 * assign24670_e38053) - (locals.var_a2_t * (locals.var_qia_dn5 + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign24670_e38050 * locals.var_vtm_dn5))))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn6 + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn7 + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn8 + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn9 + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn10 + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))), (-((locals.var_a2_t * (locals.var_qia_dn11 + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign24670_e38053 * assign24670_e38053))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24670_e38057;
        locals.var_t0_dn3 = assign24670_e38057_d_n3;
        locals.var_t0_dn4 = assign24670_e38057_d_n4;
        locals.var_t0_dn5 = assign24670_e38057_d_n5;
        locals.var_t0_dn6 = assign24670_e38057_d_n6;
        locals.var_t0_dn7 = assign24670_e38057_d_n7;
        locals.var_t0_dn8 = assign24670_e38057_d_n8;
        locals.var_t0_dn9 = assign24670_e38057_d_n9;
        locals.var_t0_dn10 = assign24670_e38057_d_n10;
        locals.var_t0_dn11 = assign24670_e38057_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24680_e38066, assign24680_e38066_d_n3, assign24680_e38066_d_n4, assign24680_e38066_d_n5, assign24680_e38066_d_n6, assign24680_e38066_d_n7, assign24680_e38066_d_n8, assign24680_e38066_d_n9, assign24680_e38066_d_n10, assign24680_e38066_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24680_e38064: f64 = (locals.var_qis - locals.var_qid);
        (assign24680_e38064, (locals.var_qis_dn3 - locals.var_qid_dn3), (locals.var_qis_dn4 - locals.var_qid_dn4), (locals.var_qis_dn5 - locals.var_qid_dn5), (locals.var_qis_dn6 - locals.var_qid_dn6), (locals.var_qis_dn7 - locals.var_qid_dn7), (locals.var_qis_dn8 - locals.var_qid_dn8), (locals.var_qis_dn9 - locals.var_qid_dn9), (locals.var_qis_dn10 - locals.var_qid_dn10), (locals.var_qis_dn11 - locals.var_qid_dn11),)
    } else {
        (locals.var_dqsd, locals.var_dqsd_dn3, locals.var_dqsd_dn4, locals.var_dqsd_dn5, locals.var_dqsd_dn6, locals.var_dqsd_dn7, locals.var_dqsd_dn8, locals.var_dqsd_dn9, locals.var_dqsd_dn10, locals.var_dqsd_dn11,)
    }
};
        locals.var_dqsd = assign24680_e38066;
        locals.var_dqsd_dn3 = assign24680_e38066_d_n3;
        locals.var_dqsd_dn4 = assign24680_e38066_d_n4;
        locals.var_dqsd_dn5 = assign24680_e38066_d_n5;
        locals.var_dqsd_dn6 = assign24680_e38066_d_n6;
        locals.var_dqsd_dn7 = assign24680_e38066_d_n7;
        locals.var_dqsd_dn8 = assign24680_e38066_d_n8;
        locals.var_dqsd_dn9 = assign24680_e38066_d_n9;
        locals.var_dqsd_dn10 = assign24680_e38066_d_n10;
        locals.var_dqsd_dn11 = assign24680_e38066_d_n11;
        locals.var_dqsd_rv = 0.0;

        let (assign24690_e38077, assign24690_e38077_d_n3, assign24690_e38077_d_n4, assign24690_e38077_d_n5, assign24690_e38077_d_n6, assign24690_e38077_d_n7, assign24690_e38077_d_n8, assign24690_e38077_d_n9, assign24690_e38077_d_n10, assign24690_e38077_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24690_e38073: f64 = (locals.var_t0 * locals.var_dqsd);
        let assign24690_e38075: f64 = (assign24690_e38073 * locals.var_dqsd);
        (assign24690_e38075, ((((locals.var_t0_dn3 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn3)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn3)), ((((locals.var_t0_dn4 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn4)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn4)), ((((locals.var_t0_dn5 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn5)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn5)), ((((locals.var_t0_dn6 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn6)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn6)), ((((locals.var_t0_dn7 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn7)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn7)), ((((locals.var_t0_dn8 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn8)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn8)), ((((locals.var_t0_dn9 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn9)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn9)), ((((locals.var_t0_dn10 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn10)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn10)), ((((locals.var_t0_dn11 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn11)) * locals.var_dqsd) + (assign24690_e38073 * locals.var_dqsd_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24690_e38077;
        locals.var_t1_dn3 = assign24690_e38077_d_n3;
        locals.var_t1_dn4 = assign24690_e38077_d_n4;
        locals.var_t1_dn5 = assign24690_e38077_d_n5;
        locals.var_t1_dn6 = assign24690_e38077_d_n6;
        locals.var_t1_dn7 = assign24690_e38077_d_n7;
        locals.var_t1_dn8 = assign24690_e38077_d_n8;
        locals.var_t1_dn9 = assign24690_e38077_d_n9;
        locals.var_t1_dn10 = assign24690_e38077_d_n10;
        locals.var_t1_dn11 = assign24690_e38077_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign24700_e38088, assign24700_e38088_d_n3, assign24700_e38088_d_n4, assign24700_e38088_d_n5, assign24700_e38088_d_n6, assign24700_e38088_d_n7, assign24700_e38088_d_n8, assign24700_e38088_d_n9, assign24700_e38088_d_n10, assign24700_e38088_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24700_e38084: f64 = (locals.var_t1 + 1.0);
        let assign24700_e38086: f64 = (assign24700_e38084 - 0.001);
        (assign24700_e38086, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24700_e38088;
        locals.var_t2_dn3 = assign24700_e38088_d_n3;
        locals.var_t2_dn4 = assign24700_e38088_d_n4;
        locals.var_t2_dn5 = assign24700_e38088_d_n5;
        locals.var_t2_dn6 = assign24700_e38088_d_n6;
        locals.var_t2_dn7 = assign24700_e38088_d_n7;
        locals.var_t2_dn8 = assign24700_e38088_d_n8;
        locals.var_t2_dn9 = assign24700_e38088_d_n9;
        locals.var_t2_dn10 = assign24700_e38088_d_n10;
        locals.var_t2_dn11 = assign24700_e38088_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign24710_e38107, assign24710_e38107_d_n3, assign24710_e38107_d_n4, assign24710_e38107_d_n5, assign24710_e38107_d_n6, assign24710_e38107_d_n7, assign24710_e38107_d_n8, assign24710_e38107_d_n9, assign24710_e38107_d_n10, assign24710_e38107_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24710_e38094: f64 = (-1.0);
        let assign24710_e38099: f64 = (locals.var_t2 * locals.var_t2);
        let assign24710_e38101: f64 = (assign24710_e38099 + 0.004);
        let assign24710_e38102: f64 = (assign24710_e38101).sqrt();
        let assign24710_e38103: f64 = (locals.var_t2 + assign24710_e38102);
        let assign24710_e38104: f64 = (0.5 * assign24710_e38103);
        let assign24710_e38105: f64 = (assign24710_e38094 + assign24710_e38104);
        (assign24710_e38105, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24710_e38102)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24710_e38102)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24710_e38107;
        locals.var_t3_dn3 = assign24710_e38107_d_n3;
        locals.var_t3_dn4 = assign24710_e38107_d_n4;
        locals.var_t3_dn5 = assign24710_e38107_d_n5;
        locals.var_t3_dn6 = assign24710_e38107_d_n6;
        locals.var_t3_dn7 = assign24710_e38107_d_n7;
        locals.var_t3_dn8 = assign24710_e38107_d_n8;
        locals.var_t3_dn9 = assign24710_e38107_d_n9;
        locals.var_t3_dn10 = assign24710_e38107_d_n10;
        locals.var_t3_dn11 = assign24710_e38107_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign24720_e38121, assign24720_e38121_d_n3, assign24720_e38121_d_n4, assign24720_e38121_d_n5, assign24720_e38121_d_n6, assign24720_e38121_d_n7, assign24720_e38121_d_n8, assign24720_e38121_d_n9, assign24720_e38121_d_n10, assign24720_e38121_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24720_e38116: f64 = (1.0 + locals.var_t3);
        let assign24720_e38117: f64 = (assign24720_e38116).sqrt();
        let assign24720_e38118: f64 = (1.0 + assign24720_e38117);
        let assign24720_e38119: f64 = (0.5 * assign24720_e38118);
        (assign24720_e38119, (0.5 * (locals.var_t3_dn3 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn4 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn5 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn6 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn7 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn8 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn9 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn10 / (2.0 * assign24720_e38117))), (0.5 * (locals.var_t3_dn11 / (2.0 * assign24720_e38117))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign24720_e38121;
        locals.var_nsat_dn3 = assign24720_e38121_d_n3;
        locals.var_nsat_dn4 = assign24720_e38121_d_n4;
        locals.var_nsat_dn5 = assign24720_e38121_d_n5;
        locals.var_nsat_dn6 = assign24720_e38121_d_n6;
        locals.var_nsat_dn7 = assign24720_e38121_d_n7;
        locals.var_nsat_dn8 = assign24720_e38121_d_n8;
        locals.var_nsat_dn9 = assign24720_e38121_d_n9;
        locals.var_nsat_dn10 = assign24720_e38121_d_n10;
        locals.var_nsat_dn11 = assign24720_e38121_d_n11;
        locals.var_nsat_rv = 0.0;

        let (assign24730_e38151, assign24730_e38151_d_n3, assign24730_e38151_d_n4, assign24730_e38151_d_n5, assign24730_e38151_d_n6, assign24730_e38151_d_n7, assign24730_e38151_d_n8, assign24730_e38151_d_n9, assign24730_e38151_d_n10, assign24730_e38151_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24730_e38129: f64 = (locals.var_nsat + 1.0);
        let assign24730_e38132: f64 = (locals.var_nsat - 1.0);
        let assign24730_e38135: f64 = (locals.var_nsat - 1.0);
        let assign24730_e38136: f64 = (assign24730_e38132 * assign24730_e38135);
        let assign24730_e38139: f64 = (0.25 * 0.01);
        let assign24730_e38141: f64 = (assign24730_e38139 * 0.01);
        let assign24730_e38142: f64 = (assign24730_e38136 + assign24730_e38141);
        let assign24730_e38143: f64 = (assign24730_e38142).sqrt();
        let assign24730_e38144: f64 = (assign24730_e38129 - assign24730_e38143);
        let assign24730_e38145: f64 = (0.5 * assign24730_e38144);
        let assign24730_e38148: f64 = (0.25 * 0.01);
        let assign24730_e38149: f64 = (assign24730_e38145 + assign24730_e38148);
        (assign24730_e38149, (0.5 * (locals.var_nsat_dn3 - (((locals.var_nsat_dn3 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn3)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn4 - (((locals.var_nsat_dn4 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn4)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn5 - (((locals.var_nsat_dn5 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn5)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn6 - (((locals.var_nsat_dn6 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn6)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn7 - (((locals.var_nsat_dn7 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn7)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn8 - (((locals.var_nsat_dn8 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn8)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn9 - (((locals.var_nsat_dn9 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn9)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn10 - (((locals.var_nsat_dn10 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn10)) / (2.0 * assign24730_e38143)))), (0.5 * (locals.var_nsat_dn11 - (((locals.var_nsat_dn11 * assign24730_e38135) + (assign24730_e38132 * locals.var_nsat_dn11)) / (2.0 * assign24730_e38143)))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign24730_e38151;
        locals.var_nsat_dn3 = assign24730_e38151_d_n3;
        locals.var_nsat_dn4 = assign24730_e38151_d_n4;
        locals.var_nsat_dn5 = assign24730_e38151_d_n5;
        locals.var_nsat_dn6 = assign24730_e38151_d_n6;
        locals.var_nsat_dn7 = assign24730_e38151_d_n7;
        locals.var_nsat_dn8 = assign24730_e38151_d_n8;
        locals.var_nsat_dn9 = assign24730_e38151_d_n9;
        locals.var_nsat_dn10 = assign24730_e38151_d_n10;
        locals.var_nsat_dn11 = assign24730_e38151_d_n11;
        locals.var_nsat_rv = 0.0;

        let (assign24740_e38160, assign24740_e38160_d_n3, assign24740_e38160_d_n4, assign24740_e38160_d_n5, assign24740_e38160_d_n6, assign24740_e38160_d_n7, assign24740_e38160_d_n8, assign24740_e38160_d_n9, assign24740_e38160_d_n10, assign24740_e38160_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24740_e38158: f64 = (locals.var_qis + locals.var_qid);
        (assign24740_e38158, (locals.var_qis_dn3 + locals.var_qid_dn3), (locals.var_qis_dn4 + locals.var_qid_dn4), (locals.var_qis_dn5 + locals.var_qid_dn5), (locals.var_qis_dn6 + locals.var_qid_dn6), (locals.var_qis_dn7 + locals.var_qid_dn7), (locals.var_qis_dn8 + locals.var_qid_dn8), (locals.var_qis_dn9 + locals.var_qid_dn9), (locals.var_qis_dn10 + locals.var_qid_dn10), (locals.var_qis_dn11 + locals.var_qid_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24740_e38160;
        locals.var_t0_dn3 = assign24740_e38160_d_n3;
        locals.var_t0_dn4 = assign24740_e38160_d_n4;
        locals.var_t0_dn5 = assign24740_e38160_d_n5;
        locals.var_t0_dn6 = assign24740_e38160_d_n6;
        locals.var_t0_dn7 = assign24740_e38160_d_n7;
        locals.var_t0_dn8 = assign24740_e38160_d_n8;
        locals.var_t0_dn9 = assign24740_e38160_d_n9;
        locals.var_t0_dn10 = assign24740_e38160_d_n10;
        locals.var_t0_dn11 = assign24740_e38160_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24750_e38169, assign24750_e38169_d_n3, assign24750_e38169_d_n4, assign24750_e38169_d_n5, assign24750_e38169_d_n6, assign24750_e38169_d_n7, assign24750_e38169_d_n8, assign24750_e38169_d_n9, assign24750_e38169_d_n10, assign24750_e38169_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24750_e38167: f64 = (locals.var_qis - locals.var_qid);
        (assign24750_e38167, (locals.var_qis_dn3 - locals.var_qid_dn3), (locals.var_qis_dn4 - locals.var_qid_dn4), (locals.var_qis_dn5 - locals.var_qid_dn5), (locals.var_qis_dn6 - locals.var_qid_dn6), (locals.var_qis_dn7 - locals.var_qid_dn7), (locals.var_qis_dn8 - locals.var_qid_dn8), (locals.var_qis_dn9 - locals.var_qid_dn9), (locals.var_qis_dn10 - locals.var_qid_dn10), (locals.var_qis_dn11 - locals.var_qid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign24750_e38169;
        locals.var_t1_dn3 = assign24750_e38169_d_n3;
        locals.var_t1_dn4 = assign24750_e38169_d_n4;
        locals.var_t1_dn5 = assign24750_e38169_d_n5;
        locals.var_t1_dn6 = assign24750_e38169_d_n6;
        locals.var_t1_dn7 = assign24750_e38169_d_n7;
        locals.var_t1_dn8 = assign24750_e38169_d_n8;
        locals.var_t1_dn9 = assign24750_e38169_d_n9;
        locals.var_t1_dn10 = assign24750_e38169_d_n10;
        locals.var_t1_dn11 = assign24750_e38169_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign24760_e38180, assign24760_e38180_d_n3, assign24760_e38180_d_n4, assign24760_e38180_d_n5, assign24760_e38180_d_n6, assign24760_e38180_d_n7, assign24760_e38180_d_n8, assign24760_e38180_d_n9, assign24760_e38180_d_n10, assign24760_e38180_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24760_e38177: f64 = (locals.var_t0 + locals.var_m0_t);
        let assign24760_e38178: f64 = (locals.var_t1 / assign24760_e38177);
        (assign24760_e38178, (((locals.var_t1_dn3 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn3)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn4 * assign24760_e38177) - (locals.var_t1 * (locals.var_t0_dn4 + locals.var_m0_t_dn4))) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn5 * assign24760_e38177) - (locals.var_t1 * (locals.var_t0_dn5 + locals.var_m0_t_dn5))) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn6 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn6)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn7 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn7)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn8 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn8)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn9 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn9)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn10 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn10)) / (assign24760_e38177 * assign24760_e38177)), (((locals.var_t1_dn11 * assign24760_e38177) - (locals.var_t1 * locals.var_t0_dn11)) / (assign24760_e38177 * assign24760_e38177)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign24760_e38180;
        locals.var_t2_dn3 = assign24760_e38180_d_n3;
        locals.var_t2_dn4 = assign24760_e38180_d_n4;
        locals.var_t2_dn5 = assign24760_e38180_d_n5;
        locals.var_t2_dn6 = assign24760_e38180_d_n6;
        locals.var_t2_dn7 = assign24760_e38180_d_n7;
        locals.var_t2_dn8 = assign24760_e38180_d_n8;
        locals.var_t2_dn9 = assign24760_e38180_d_n9;
        locals.var_t2_dn10 = assign24760_e38180_d_n10;
        locals.var_t2_dn11 = assign24760_e38180_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign24770_e38191, assign24770_e38191_d_n3, assign24770_e38191_d_n4, assign24770_e38191_d_n5, assign24770_e38191_d_n6, assign24770_e38191_d_n7, assign24770_e38191_d_n8, assign24770_e38191_d_n9, assign24770_e38191_d_n10, assign24770_e38191_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24770_e38187: f64 = (locals.var_k0_t * locals.var_t2);
        let assign24770_e38189: f64 = (assign24770_e38187 * locals.var_t2);
        (assign24770_e38189, (((locals.var_k0_t * locals.var_t2_dn3) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn3)), ((((locals.var_k0_t_dn4 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn4)) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn4)), ((((locals.var_k0_t_dn5 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn5)) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn5)), (((locals.var_k0_t * locals.var_t2_dn6) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn6)), (((locals.var_k0_t * locals.var_t2_dn7) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn7)), (((locals.var_k0_t * locals.var_t2_dn8) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn8)), (((locals.var_k0_t * locals.var_t2_dn9) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn9)), (((locals.var_k0_t * locals.var_t2_dn10) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn10)), (((locals.var_k0_t * locals.var_t2_dn11) * locals.var_t2) + (assign24770_e38187 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign24770_e38191;
        locals.var_t3_dn3 = assign24770_e38191_d_n3;
        locals.var_t3_dn4 = assign24770_e38191_d_n4;
        locals.var_t3_dn5 = assign24770_e38191_d_n5;
        locals.var_t3_dn6 = assign24770_e38191_d_n6;
        locals.var_t3_dn7 = assign24770_e38191_d_n7;
        locals.var_t3_dn8 = assign24770_e38191_d_n8;
        locals.var_t3_dn9 = assign24770_e38191_d_n9;
        locals.var_t3_dn10 = assign24770_e38191_d_n10;
        locals.var_t3_dn11 = assign24770_e38191_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign24780_e38200, assign24780_e38200_d_n3, assign24780_e38200_d_n4, assign24780_e38200_d_n5, assign24780_e38200_d_n6, assign24780_e38200_d_n7, assign24780_e38200_d_n8, assign24780_e38200_d_n9, assign24780_e38200_d_n10, assign24780_e38200_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24780_e38198: f64 = (1.0 + locals.var_t3);
        (assign24780_e38198, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8, locals.var_mnud_dn9, locals.var_mnud_dn10, locals.var_mnud_dn11,)
    }
};
        locals.var_mnud = assign24780_e38200;
        locals.var_mnud_dn3 = assign24780_e38200_d_n3;
        locals.var_mnud_dn4 = assign24780_e38200_d_n4;
        locals.var_mnud_dn5 = assign24780_e38200_d_n5;
        locals.var_mnud_dn6 = assign24780_e38200_d_n6;
        locals.var_mnud_dn7 = assign24780_e38200_d_n7;
        locals.var_mnud_dn8 = assign24780_e38200_d_n8;
        locals.var_mnud_dn9 = assign24780_e38200_d_n9;
        locals.var_mnud_dn10 = assign24780_e38200_d_n10;
        locals.var_mnud_dn11 = assign24780_e38200_d_n11;
        locals.var_mnud_rv = 0.0;

        let (assign24790_e38225, assign24790_e38225_d_n3, assign24790_e38225_d_n4, assign24790_e38225_d_n5, assign24790_e38225_d_n6, assign24790_e38225_d_n7, assign24790_e38225_d_n8, assign24790_e38225_d_n9, assign24790_e38225_d_n10, assign24790_e38225_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24790_e38210: f64 = (locals.var_c0sisat_t * locals.var_t1);
        let assign24790_e38212: f64 = (assign24790_e38210 * locals.var_t1);
        let assign24790_e38213: f64 = (locals.var_c0si_t + assign24790_e38212);
        let assign24790_e38214: f64 = (0.0_f64).max(assign24790_e38213);
        let assign24790_e38216: f64 = (assign24790_e38214 * locals.var_t0);
        let assign24790_e38219: f64 = (2.0 * locals.var_n);
        let assign24790_e38221: f64 = (assign24790_e38219 * locals.var_vtm);
        let assign24790_e38222: f64 = (assign24790_e38216 + assign24790_e38221);
        let assign24790_e38223: f64 = (locals.var_c0_t / assign24790_e38222);
        (assign24790_e38223, (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn3) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn3)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn3)) + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (((locals.var_c0_t_dn4 * assign24790_e38222) - (locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (locals.var_c0si_t_dn4 + ((((locals.var_c0sisat_t_dn4 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn4)) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn4))) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn4)) + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign24790_e38219 * locals.var_vtm_dn4))))) / (assign24790_e38222 * assign24790_e38222)), (((locals.var_c0_t_dn5 * assign24790_e38222) - (locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (locals.var_c0si_t_dn5 + ((((locals.var_c0sisat_t_dn5 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn5)) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn5))) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn5)) + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign24790_e38219 * locals.var_vtm_dn5))))) / (assign24790_e38222 * assign24790_e38222)), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn6) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn6)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn6)) + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn7) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn7)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn7)) + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn8) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn8)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn8)) + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn9) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn9)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn9)) + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn10) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn10)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn10)) + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))), (-((locals.var_c0_t * (((if 0.0 >= assign24790_e38213 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn11) * locals.var_t1) + (assign24790_e38210 * locals.var_t1_dn11)) } * locals.var_t0) + (assign24790_e38214 * locals.var_t0_dn11)) + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign24790_e38222 * assign24790_e38222))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign24790_e38225;
        locals.var_t9_dn3 = assign24790_e38225_d_n3;
        locals.var_t9_dn4 = assign24790_e38225_d_n4;
        locals.var_t9_dn5 = assign24790_e38225_d_n5;
        locals.var_t9_dn6 = assign24790_e38225_d_n6;
        locals.var_t9_dn7 = assign24790_e38225_d_n7;
        locals.var_t9_dn8 = assign24790_e38225_d_n8;
        locals.var_t9_dn9 = assign24790_e38225_d_n9;
        locals.var_t9_dn10 = assign24790_e38225_d_n10;
        locals.var_t9_dn11 = assign24790_e38225_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign24800_e38234, assign24800_e38234_d_n3, assign24800_e38234_d_n4, assign24800_e38234_d_n5, assign24800_e38234_d_n6, assign24800_e38234_d_n7, assign24800_e38234_d_n8, assign24800_e38234_d_n9, assign24800_e38234_d_n10, assign24800_e38234_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24800_e38231: f64 = (-locals.var_t9);
        let assign24800_e38232: f64 = { let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign24800_e38232, ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn3)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn4)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn5)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn6)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn7)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn8)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn9)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn10)), ({ let limited_exp_arg = assign24800_e38231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn11)),)
    } else {
        (locals.var_mnud1, locals.var_mnud1_dn3, locals.var_mnud1_dn4, locals.var_mnud1_dn5, locals.var_mnud1_dn6, locals.var_mnud1_dn7, locals.var_mnud1_dn8, locals.var_mnud1_dn9, locals.var_mnud1_dn10, locals.var_mnud1_dn11,)
    }
};
        locals.var_mnud1 = assign24800_e38234;
        locals.var_mnud1_dn3 = assign24800_e38234_d_n3;
        locals.var_mnud1_dn4 = assign24800_e38234_d_n4;
        locals.var_mnud1_dn5 = assign24800_e38234_d_n5;
        locals.var_mnud1_dn6 = assign24800_e38234_d_n6;
        locals.var_mnud1_dn7 = assign24800_e38234_d_n7;
        locals.var_mnud1_dn8 = assign24800_e38234_d_n8;
        locals.var_mnud1_dn9 = assign24800_e38234_d_n9;
        locals.var_mnud1_dn10 = assign24800_e38234_d_n10;
        locals.var_mnud1_dn11 = assign24800_e38234_d_n11;
        locals.var_mnud1_rv = 0.0;

        let (assign24810_e38243, assign24810_e38243_d_n3, assign24810_e38243_d_n4, assign24810_e38243_d_n5, assign24810_e38243_d_n6, assign24810_e38243_d_n7, assign24810_e38243_d_n8, assign24810_e38243_d_n9, assign24810_e38243_d_n10, assign24810_e38243_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24810_e38241: f64 = (locals.var_dmob_dl * locals.var_dvsatinv);
        (assign24810_e38241, ((locals.var_dmob_dl_dn3 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn3)), ((locals.var_dmob_dl_dn4 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn4)), ((locals.var_dmob_dl_dn5 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn5)), ((locals.var_dmob_dl_dn6 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn6)), ((locals.var_dmob_dl_dn7 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn7)), ((locals.var_dmob_dl_dn8 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn8)), ((locals.var_dmob_dl_dn9 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn9)), ((locals.var_dmob_dl_dn10 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn10)), ((locals.var_dmob_dl_dn11 * locals.var_dvsatinv) + (locals.var_dmob_dl * locals.var_dvsatinv_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24810_e38243;
        locals.var_t0_dn3 = assign24810_e38243_d_n3;
        locals.var_t0_dn4 = assign24810_e38243_d_n4;
        locals.var_t0_dn5 = assign24810_e38243_d_n5;
        locals.var_t0_dn6 = assign24810_e38243_d_n6;
        locals.var_t0_dn7 = assign24810_e38243_d_n7;
        locals.var_t0_dn8 = assign24810_e38243_d_n8;
        locals.var_t0_dn9 = assign24810_e38243_d_n9;
        locals.var_t0_dn10 = assign24810_e38243_d_n10;
        locals.var_t0_dn11 = assign24810_e38243_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24820_e38260, assign24820_e38260_d_n3, assign24820_e38260_d_n4, assign24820_e38260_d_n5, assign24820_e38260_d_n6, assign24820_e38260_d_n7, assign24820_e38260_d_n8, assign24820_e38260_d_n9, assign24820_e38260_d_n10, assign24820_e38260_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24820_e38253: f64 = (locals.var_zsat * locals.var_t0);
        let assign24820_e38255: f64 = (assign24820_e38253 * locals.var_t0);
        let assign24820_e38256: f64 = (0.5 * assign24820_e38255);
        let assign24820_e38257: f64 = (1.0 + assign24820_e38256);
        let assign24820_e38258: f64 = (locals.var_alpha_dd * assign24820_e38257);
        (assign24820_e38258, ((locals.var_alpha_dd_dn3 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn3 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn3)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn3))))), ((locals.var_alpha_dd_dn4 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn4 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn4)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn4))))), ((locals.var_alpha_dd_dn5 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn5 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn5)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn5))))), ((locals.var_alpha_dd_dn6 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn6 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn6)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn6))))), ((locals.var_alpha_dd_dn7 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn7 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn7)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn7))))), ((locals.var_alpha_dd_dn8 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn8 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn8)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn8))))), ((locals.var_alpha_dd_dn9 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn9 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn9)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn9))))), ((locals.var_alpha_dd_dn10 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn10 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn10)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn10))))), ((locals.var_alpha_dd_dn11 * assign24820_e38257) + (locals.var_alpha_dd * (0.5 * ((((locals.var_zsat_dn11 * locals.var_t0) + (locals.var_zsat * locals.var_t0_dn11)) * locals.var_t0) + (assign24820_e38253 * locals.var_t0_dn11))))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10, locals.var_alpha1_dn11,)
    }
};
        locals.var_alpha1 = assign24820_e38260;
        locals.var_alpha1_dn3 = assign24820_e38260_d_n3;
        locals.var_alpha1_dn4 = assign24820_e38260_d_n4;
        locals.var_alpha1_dn5 = assign24820_e38260_d_n5;
        locals.var_alpha1_dn6 = assign24820_e38260_d_n6;
        locals.var_alpha1_dn7 = assign24820_e38260_d_n7;
        locals.var_alpha1_dn8 = assign24820_e38260_d_n8;
        locals.var_alpha1_dn9 = assign24820_e38260_d_n9;
        locals.var_alpha1_dn10 = assign24820_e38260_d_n10;
        locals.var_alpha1_dn11 = assign24820_e38260_d_n11;
        locals.var_alpha1_rv = 0.0;

        let (assign24830_e38288, assign24830_e38288_d_n3, assign24830_e38288_d_n4, assign24830_e38288_d_n5, assign24830_e38288_d_n6, assign24830_e38288_d_n7, assign24830_e38288_d_n8, assign24830_e38288_d_n9, assign24830_e38288_d_n10, assign24830_e38288_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24830_e38268: f64 = (locals.var_alpha1 - 0.001);
        let assign24830_e38271: f64 = (locals.var_alpha1 - 0.001);
        let assign24830_e38274: f64 = (locals.var_alpha1 - 0.001);
        let assign24830_e38275: f64 = (assign24830_e38271 * assign24830_e38274);
        let assign24830_e38278: f64 = (4.0 * 1e-5);
        let assign24830_e38280: f64 = (assign24830_e38278 * 1e-5);
        let assign24830_e38281: f64 = (assign24830_e38275 + assign24830_e38280);
        let assign24830_e38282: f64 = (assign24830_e38281).sqrt();
        let assign24830_e38283: f64 = (assign24830_e38268 + assign24830_e38282);
        let assign24830_e38284: f64 = (0.5 * assign24830_e38283);
        let assign24830_e38286: f64 = (assign24830_e38284 + 0.001);
        (assign24830_e38286, (0.5 * (locals.var_alpha1_dn3 + (((locals.var_alpha1_dn3 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn3)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn4 + (((locals.var_alpha1_dn4 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn4)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn5 + (((locals.var_alpha1_dn5 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn5)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn6 + (((locals.var_alpha1_dn6 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn6)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn7 + (((locals.var_alpha1_dn7 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn7)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn8 + (((locals.var_alpha1_dn8 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn8)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn9 + (((locals.var_alpha1_dn9 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn9)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn10 + (((locals.var_alpha1_dn10 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn10)) / (2.0 * assign24830_e38282)))), (0.5 * (locals.var_alpha1_dn11 + (((locals.var_alpha1_dn11 * assign24830_e38274) + (assign24830_e38271 * locals.var_alpha1_dn11)) / (2.0 * assign24830_e38282)))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn3, locals.var_alpha1_dn4, locals.var_alpha1_dn5, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9, locals.var_alpha1_dn10, locals.var_alpha1_dn11,)
    }
};
        locals.var_alpha1 = assign24830_e38288;
        locals.var_alpha1_dn3 = assign24830_e38288_d_n3;
        locals.var_alpha1_dn4 = assign24830_e38288_d_n4;
        locals.var_alpha1_dn5 = assign24830_e38288_d_n5;
        locals.var_alpha1_dn6 = assign24830_e38288_d_n6;
        locals.var_alpha1_dn7 = assign24830_e38288_d_n7;
        locals.var_alpha1_dn8 = assign24830_e38288_d_n8;
        locals.var_alpha1_dn9 = assign24830_e38288_d_n9;
        locals.var_alpha1_dn10 = assign24830_e38288_d_n10;
        locals.var_alpha1_dn11 = assign24830_e38288_d_n11;
        locals.var_alpha1_rv = 0.0;

        let (assign24840_e38299, assign24840_e38299_d_n3, assign24840_e38299_d_n4, assign24840_e38299_d_n5, assign24840_e38299_d_n6, assign24840_e38299_d_n7, assign24840_e38299_d_n8, assign24840_e38299_d_n9, assign24840_e38299_d_n10, assign24840_e38299_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24840_e38296: f64 = (locals.var_nvt * locals.var_alpha_dd);
        let assign24840_e38297: f64 = (locals.var_qim + assign24840_e38296);
        (assign24840_e38297, (locals.var_qim_dn3 + ((locals.var_nvt_dn3 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn3))), (locals.var_qim_dn4 + ((locals.var_nvt_dn4 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn4))), (locals.var_qim_dn5 + ((locals.var_nvt_dn5 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn5))), (locals.var_qim_dn6 + ((locals.var_nvt_dn6 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn6))), (locals.var_qim_dn7 + ((locals.var_nvt_dn7 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn7))), (locals.var_qim_dn8 + ((locals.var_nvt_dn8 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn8))), (locals.var_qim_dn9 + ((locals.var_nvt_dn9 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn9))), (locals.var_qim_dn10 + ((locals.var_nvt_dn10 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn10))), (locals.var_qim_dn11 + ((locals.var_nvt_dn11 * locals.var_alpha_dd) + (locals.var_nvt * locals.var_alpha_dd_dn11))),)
    } else {
        (locals.var_qim1, locals.var_qim1_dn3, locals.var_qim1_dn4, locals.var_qim1_dn5, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, locals.var_qim1_dn9, locals.var_qim1_dn10, locals.var_qim1_dn11,)
    }
};
        locals.var_qim1 = assign24840_e38299;
        locals.var_qim1_dn3 = assign24840_e38299_d_n3;
        locals.var_qim1_dn4 = assign24840_e38299_d_n4;
        locals.var_qim1_dn5 = assign24840_e38299_d_n5;
        locals.var_qim1_dn6 = assign24840_e38299_d_n6;
        locals.var_qim1_dn7 = assign24840_e38299_d_n7;
        locals.var_qim1_dn8 = assign24840_e38299_d_n8;
        locals.var_qim1_dn9 = assign24840_e38299_d_n9;
        locals.var_qim1_dn10 = assign24840_e38299_d_n10;
        locals.var_qim1_dn11 = assign24840_e38299_d_n11;
        locals.var_qim1_rv = 0.0;

        let (assign24850_e38312, assign24850_e38312_d_n3, assign24850_e38312_d_n4, assign24850_e38312_d_n5, assign24850_e38312_d_n6, assign24850_e38312_d_n7, assign24850_e38312_d_n8, assign24850_e38312_d_n9, assign24850_e38312_d_n10, assign24850_e38312_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard506 == 0.0)) {
        let assign24850_e38306: f64 = (locals.var_qim1 / locals.var_alpha1);
        let assign24850_e38309: f64 = (locals.var_dmob_dl / locals.var_dvsat);
        let assign24850_e38310: f64 = (assign24850_e38306 * assign24850_e38309);
        (assign24850_e38310, (((((locals.var_qim1_dn3 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn3)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn3 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn4 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn4)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn4 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn5 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn5)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn5 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn6 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn6 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn7 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn7 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn8 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn8 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn9 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn9)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn9 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn9)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn10 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn10)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn10 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn10)) / (locals.var_dvsat * locals.var_dvsat)))), (((((locals.var_qim1_dn11 * locals.var_alpha1) - (locals.var_qim1 * locals.var_alpha1_dn11)) / (locals.var_alpha1 * locals.var_alpha1)) * assign24850_e38309) + (assign24850_e38306 * (((locals.var_dmob_dl_dn11 * locals.var_dvsat) - (locals.var_dmob_dl * locals.var_dvsat_dn11)) / (locals.var_dvsat * locals.var_dvsat)))),)
    } else {
        (locals.var_h_fact, locals.var_h_fact_dn3, locals.var_h_fact_dn4, locals.var_h_fact_dn5, locals.var_h_fact_dn6, locals.var_h_fact_dn7, locals.var_h_fact_dn8, locals.var_h_fact_dn9, locals.var_h_fact_dn10, locals.var_h_fact_dn11,)
    }
};
        locals.var_h_fact = assign24850_e38312;
        locals.var_h_fact_dn3 = assign24850_e38312_d_n3;
        locals.var_h_fact_dn4 = assign24850_e38312_d_n4;
        locals.var_h_fact_dn5 = assign24850_e38312_d_n5;
        locals.var_h_fact_dn6 = assign24850_e38312_d_n6;
        locals.var_h_fact_dn7 = assign24850_e38312_d_n7;
        locals.var_h_fact_dn8 = assign24850_e38312_d_n8;
        locals.var_h_fact_dn9 = assign24850_e38312_d_n9;
        locals.var_h_fact_dn10 = assign24850_e38312_d_n10;
        locals.var_h_fact_dn11 = assign24850_e38312_d_n11;
        locals.var_h_fact_rv = 0.0;

        let (assign24860_e38342, assign24860_e38342_d_n3, assign24860_e38342_d_n4, assign24860_e38342_d_n5, assign24860_e38342_d_n6, assign24860_e38342_d_n7, assign24860_e38342_d_n8, assign24860_e38342_d_n9, assign24860_e38342_d_n10, assign24860_e38342_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign24860_e38316: f64 = (p.p2 * locals.var_u0_a);
        let assign24860_e38319: f64 = (locals.var_weff / locals.var_leff);
        let assign24860_e38320: f64 = (assign24860_e38316 * assign24860_e38319);
        let assign24860_e38322: f64 = (assign24860_e38320 * locals.var_cox);
        let assign24860_e38324: f64 = (assign24860_e38322 * locals.var_qim1);
        let assign24860_e38326: f64 = (assign24860_e38324 * locals.var_dps);
        let assign24860_e38329: f64 = (locals.var_ddl * locals.var_dvsatinv);
        let assign24860_e38331: f64 = (assign24860_e38329 / locals.var_dr);
        let assign24860_e38332: f64 = (assign24860_e38326 * assign24860_e38331);
        let assign24860_e38334: f64 = (assign24860_e38332 * locals.var_moc);
        let assign24860_e38336: f64 = (assign24860_e38334 / locals.var_nsat);
        let assign24860_e38338: f64 = (assign24860_e38336 * locals.var_mnud);
        let assign24860_e38340: f64 = (assign24860_e38338 * locals.var_mnud1);
        (assign24860_e38340, ((((((((((((((((((p.p2 * locals.var_u0_a_dn3) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn3)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn3)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn3 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn3)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn3)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn3)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn3)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn3)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn3)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn4) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn4)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn4)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn4 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn4)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn4)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn4)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn4)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn4)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn4)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn5) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn5)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn5)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn5 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn5)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn5)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn5)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn5)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn5)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn5)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn6) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn6)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn6)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn6 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn6)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn6)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn6)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn6)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn6)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn6)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn7) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn7)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn7)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn7 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn7)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn7)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn7)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn7)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn7)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn7)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn8) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn8)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn8)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn8 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn8)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn8)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn8)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn8)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn8)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn8)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn9) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn9)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn9)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn9 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn9)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn9)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn9)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn9)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn9)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn9)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn10) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn10)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn10)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn10 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn10)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn10)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn10)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn10)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn10)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn10)), ((((((((((((((((((p.p2 * locals.var_u0_a_dn11) * assign24860_e38319) * locals.var_cox) * locals.var_qim1) + (assign24860_e38322 * locals.var_qim1_dn11)) * locals.var_dps) + (assign24860_e38324 * locals.var_dps_dn11)) * assign24860_e38331) + (assign24860_e38326 * (((((locals.var_ddl_dn11 * locals.var_dvsatinv) + (locals.var_ddl * locals.var_dvsatinv_dn11)) * locals.var_dr) - (assign24860_e38329 * locals.var_dr_dn11)) / (locals.var_dr * locals.var_dr)))) * locals.var_moc) + (assign24860_e38332 * locals.var_moc_dn11)) * locals.var_nsat) - (assign24860_e38334 * locals.var_nsat_dn11)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign24860_e38336 * locals.var_mnud_dn11)) * locals.var_mnud1) + (assign24860_e38338 * locals.var_mnud1_dn11)),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign24860_e38342;
        locals.var_ids_dn3 = assign24860_e38342_d_n3;
        locals.var_ids_dn4 = assign24860_e38342_d_n4;
        locals.var_ids_dn5 = assign24860_e38342_d_n5;
        locals.var_ids_dn6 = assign24860_e38342_d_n6;
        locals.var_ids_dn7 = assign24860_e38342_d_n7;
        locals.var_ids_dn8 = assign24860_e38342_d_n8;
        locals.var_ids_dn9 = assign24860_e38342_d_n9;
        locals.var_ids_dn10 = assign24860_e38342_d_n10;
        locals.var_ids_dn11 = assign24860_e38342_d_n11;
        locals.var_ids_rv = 0.0;

        let (assign24870_e38350, assign24870_e38350_d_n3, assign24870_e38350_d_n4, assign24870_e38350_d_n5, assign24870_e38350_d_n6, assign24870_e38350_d_n7, assign24870_e38350_d_n8, assign24870_e38350_d_n9, assign24870_e38350_d_n10, assign24870_e38350_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign24870_e38346: f64 = (locals.var_dmob * locals.var_dvsat);
        let assign24870_e38348: f64 = (assign24870_e38346 * locals.var_dr);
        (assign24870_e38348, ((((locals.var_dmob_dn3 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn3)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn3)), ((((locals.var_dmob_dn4 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn4)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn4)), ((((locals.var_dmob_dn5 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn5)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn5)), ((((locals.var_dmob_dn6 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn6)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn6)), ((((locals.var_dmob_dn7 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn7)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn7)), ((((locals.var_dmob_dn8 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn8)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn8)), ((((locals.var_dmob_dn9 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn9)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn9)), ((((locals.var_dmob_dn10 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn10)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn10)), ((((locals.var_dmob_dn11 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn11)) * locals.var_dr) + (assign24870_e38346 * locals.var_dr_dn11)),)
    } else {
        (locals.var_dtot, locals.var_dtot_dn3, locals.var_dtot_dn4, locals.var_dtot_dn5, locals.var_dtot_dn6, locals.var_dtot_dn7, locals.var_dtot_dn8, locals.var_dtot_dn9, locals.var_dtot_dn10, locals.var_dtot_dn11,)
    }
};
        locals.var_dtot = assign24870_e38350;
        locals.var_dtot_dn3 = assign24870_e38350_d_n3;
        locals.var_dtot_dn4 = assign24870_e38350_d_n4;
        locals.var_dtot_dn5 = assign24870_e38350_d_n5;
        locals.var_dtot_dn6 = assign24870_e38350_d_n6;
        locals.var_dtot_dn7 = assign24870_e38350_d_n7;
        locals.var_dtot_dn8 = assign24870_e38350_d_n8;
        locals.var_dtot_dn9 = assign24870_e38350_d_n9;
        locals.var_dtot_dn10 = assign24870_e38350_d_n10;
        locals.var_dtot_dn11 = assign24870_e38350_d_n11;
        locals.var_dtot_rv = 0.0;

        let (assign24880_e38356, assign24880_e38356_d_n3, assign24880_e38356_d_n4, assign24880_e38356_d_n5, assign24880_e38356_d_n6, assign24880_e38356_d_n7, assign24880_e38356_d_n8, assign24880_e38356_d_n9, assign24880_e38356_d_n10, assign24880_e38356_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign24880_e38354: f64 = (locals.var_u0_a / locals.var_dtot);
        (assign24880_e38354, (((locals.var_u0_a_dn3 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn3)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn4 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn4)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn5 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn5)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn6 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn6)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn7 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn7)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn8 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn8)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn9 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn9)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn10 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn10)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn11 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn11)) / (locals.var_dtot * locals.var_dtot)),)
    } else {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    }
};
        locals.var_ueff = assign24880_e38356;
        locals.var_ueff_dn3 = assign24880_e38356_d_n3;
        locals.var_ueff_dn4 = assign24880_e38356_d_n4;
        locals.var_ueff_dn5 = assign24880_e38356_d_n5;
        locals.var_ueff_dn6 = assign24880_e38356_d_n6;
        locals.var_ueff_dn7 = assign24880_e38356_d_n7;
        locals.var_ueff_dn8 = assign24880_e38356_d_n8;
        locals.var_ueff_dn9 = assign24880_e38356_d_n9;
        locals.var_ueff_dn10 = assign24880_e38356_d_n10;
        locals.var_ueff_dn11 = assign24880_e38356_d_n11;
        locals.var_ueff_rv = 0.0;

        let (assign24890_e38360, assign24890_e38360_d_n3, assign24890_e38360_d_n4, assign24890_e38360_d_n5, assign24890_e38360_d_n6, assign24890_e38360_d_n7, assign24890_e38360_d_n8, assign24890_e38360_d_n9, assign24890_e38360_d_n10, assign24890_e38360_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign24890_e38360;
        locals.var_gcrg_dn3 = assign24890_e38360_d_n3;
        locals.var_gcrg_dn4 = assign24890_e38360_d_n4;
        locals.var_gcrg_dn5 = assign24890_e38360_d_n5;
        locals.var_gcrg_dn6 = assign24890_e38360_d_n6;
        locals.var_gcrg_dn7 = assign24890_e38360_d_n7;
        locals.var_gcrg_dn8 = assign24890_e38360_d_n8;
        locals.var_gcrg_dn9 = assign24890_e38360_d_n9;
        locals.var_gcrg_dn10 = assign24890_e38360_d_n10;
        locals.var_gcrg_dn11 = assign24890_e38360_d_n11;
        locals.var_gcrg_rv = 0.0;

        let assign24900_e38363: f64 = if p.p7 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign24900_e38363;
        locals.var_guard539_rv = 0.0;

        let (assign24910_e38377, assign24910_e38377_d_n3, assign24910_e38377_d_n4, assign24910_e38377_d_n5, assign24910_e38377_d_n6, assign24910_e38377_d_n7, assign24910_e38377_d_n8, assign24910_e38377_d_n9, assign24910_e38377_d_n10, assign24910_e38377_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24910_e38369: f64 = (locals.var_ueff * locals.var_weff);
        let assign24910_e38371: f64 = (assign24910_e38369 / locals.var_leff);
        let assign24910_e38373: f64 = (assign24910_e38371 * locals.var_cox);
        let assign24910_e38375: f64 = (assign24910_e38373 * locals.var_qia);
        (assign24910_e38375, (((((locals.var_ueff_dn3 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn3)), (((((locals.var_ueff_dn4 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn4)), (((((locals.var_ueff_dn5 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn5)), (((((locals.var_ueff_dn6 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn6)), (((((locals.var_ueff_dn7 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn7)), (((((locals.var_ueff_dn8 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn8)), (((((locals.var_ueff_dn9 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn9)), (((((locals.var_ueff_dn10 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn10)), (((((locals.var_ueff_dn11 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign24910_e38373 * locals.var_qia_dn11)),)
    } else {
        (locals.var_idsovvds, locals.var_idsovvds_dn3, locals.var_idsovvds_dn4, locals.var_idsovvds_dn5, locals.var_idsovvds_dn6, locals.var_idsovvds_dn7, locals.var_idsovvds_dn8, locals.var_idsovvds_dn9, locals.var_idsovvds_dn10, locals.var_idsovvds_dn11,)
    }
};
        locals.var_idsovvds = assign24910_e38377;
        locals.var_idsovvds_dn3 = assign24910_e38377_d_n3;
        locals.var_idsovvds_dn4 = assign24910_e38377_d_n4;
        locals.var_idsovvds_dn5 = assign24910_e38377_d_n5;
        locals.var_idsovvds_dn6 = assign24910_e38377_d_n6;
        locals.var_idsovvds_dn7 = assign24910_e38377_d_n7;
        locals.var_idsovvds_dn8 = assign24910_e38377_d_n8;
        locals.var_idsovvds_dn9 = assign24910_e38377_d_n9;
        locals.var_idsovvds_dn10 = assign24910_e38377_d_n10;
        locals.var_idsovvds_dn11 = assign24910_e38377_d_n11;
        locals.var_idsovvds_rv = 0.0;

        let (assign24920_e38385, assign24920_e38385_d_n3, assign24920_e38385_d_n4, assign24920_e38385_d_n5, assign24920_e38385_d_n6, assign24920_e38385_d_n7, assign24920_e38385_d_n8, assign24920_e38385_d_n9, assign24920_e38385_d_n10, assign24920_e38385_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24920_e38383: f64 = (p.p1009 * locals.var_vt);
        (assign24920_e38383, 0.0, (p.p1009 * locals.var_vt_dn4), (p.p1009 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign24920_e38385;
        locals.var_t9_dn3 = assign24920_e38385_d_n3;
        locals.var_t9_dn4 = assign24920_e38385_d_n4;
        locals.var_t9_dn5 = assign24920_e38385_d_n5;
        locals.var_t9_dn6 = assign24920_e38385_d_n6;
        locals.var_t9_dn7 = assign24920_e38385_d_n7;
        locals.var_t9_dn8 = assign24920_e38385_d_n8;
        locals.var_t9_dn9 = assign24920_e38385_d_n9;
        locals.var_t9_dn10 = assign24920_e38385_d_n10;
        locals.var_t9_dn11 = assign24920_e38385_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign24930_e38399, assign24930_e38399_d_n3, assign24930_e38399_d_n4, assign24930_e38399_d_n5, assign24930_e38399_d_n6, assign24930_e38399_d_n7, assign24930_e38399_d_n8, assign24930_e38399_d_n9, assign24930_e38399_d_n10, assign24930_e38399_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24930_e38391: f64 = (locals.var_t9 * locals.var_ueff);
        let assign24930_e38393: f64 = (assign24930_e38391 * locals.var_weff);
        let assign24930_e38395: f64 = (assign24930_e38393 / locals.var_leff);
        let assign24930_e38397: f64 = (assign24930_e38395 * locals.var_cox);
        (assign24930_e38397, (((((locals.var_t9_dn3 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn4 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn5 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn6 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn7 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn8 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn9 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn10 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn11 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign24930_e38399;
        locals.var_t0_dn3 = assign24930_e38399_d_n3;
        locals.var_t0_dn4 = assign24930_e38399_d_n4;
        locals.var_t0_dn5 = assign24930_e38399_d_n5;
        locals.var_t0_dn6 = assign24930_e38399_d_n6;
        locals.var_t0_dn7 = assign24930_e38399_d_n7;
        locals.var_t0_dn8 = assign24930_e38399_d_n8;
        locals.var_t0_dn9 = assign24930_e38399_d_n9;
        locals.var_t0_dn10 = assign24930_e38399_d_n10;
        locals.var_t0_dn11 = assign24930_e38399_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign24940_e38411, assign24940_e38411_d_n3, assign24940_e38411_d_n4, assign24940_e38411_d_n5, assign24940_e38411_d_n6, assign24940_e38411_d_n7, assign24940_e38411_d_n8, assign24940_e38411_d_n9, assign24940_e38411_d_n10, assign24940_e38411_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) {
        let assign24940_e38405: f64 = (p.p1008 * p.p2);
        let assign24940_e38408: f64 = (locals.var_t0 + locals.var_idsovvds);
        let assign24940_e38409: f64 = (assign24940_e38405 * assign24940_e38408);
        (assign24940_e38409, (assign24940_e38405 * (locals.var_t0_dn3 + locals.var_idsovvds_dn3)), (assign24940_e38405 * (locals.var_t0_dn4 + locals.var_idsovvds_dn4)), (assign24940_e38405 * (locals.var_t0_dn5 + locals.var_idsovvds_dn5)), (assign24940_e38405 * (locals.var_t0_dn6 + locals.var_idsovvds_dn6)), (assign24940_e38405 * (locals.var_t0_dn7 + locals.var_idsovvds_dn7)), (assign24940_e38405 * (locals.var_t0_dn8 + locals.var_idsovvds_dn8)), (assign24940_e38405 * (locals.var_t0_dn9 + locals.var_idsovvds_dn9)), (assign24940_e38405 * (locals.var_t0_dn10 + locals.var_idsovvds_dn10)), (assign24940_e38405 * (locals.var_t0_dn11 + locals.var_idsovvds_dn11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign24940_e38411;
        locals.var_gcrg_dn3 = assign24940_e38411_d_n3;
        locals.var_gcrg_dn4 = assign24940_e38411_d_n4;
        locals.var_gcrg_dn5 = assign24940_e38411_d_n5;
        locals.var_gcrg_dn6 = assign24940_e38411_d_n6;
        locals.var_gcrg_dn7 = assign24940_e38411_d_n7;
        locals.var_gcrg_dn8 = assign24940_e38411_d_n8;
        locals.var_gcrg_dn9 = assign24940_e38411_d_n9;
        locals.var_gcrg_dn10 = assign24940_e38411_d_n10;
        locals.var_gcrg_dn11 = assign24940_e38411_d_n11;
        locals.var_gcrg_rv = 0.0;

        let assign24950_e38414: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign24950_e38414;
        locals.var_guard540_rv = 0.0;

        let (assign24960_e38424,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign24960_e38422: f64 = (1.0 / locals.var_grgeltd);
        (assign24960_e38422,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign24960_e38424;
        locals.var_rgeltd_rv = 0.0;

        let assign24970_e38427: f64 = if locals.var_rgeltd < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign24970_e38427;
        locals.var_guard541_rv = 0.0;

        let (assign24980_e38437,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign24980_e38437;
        locals.var_rgeltd_rv = 0.0;

        let (assign24990_e38449,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) && (locals.var_guard541 != 0.0)) {
        let assign24990_e38447: f64 = (1.0 / locals.var_rgeltd);
        (assign24990_e38447,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign24990_e38449;
        locals.var_grgeltd_rv = 0.0;

        let (assign25000_e38459, assign25000_e38459_d_n3, assign25000_e38459_d_n4, assign25000_e38459_d_n5, assign25000_e38459_d_n6, assign25000_e38459_d_n7, assign25000_e38459_d_n8, assign25000_e38459_d_n9, assign25000_e38459_d_n10, assign25000_e38459_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign25000_e38457: f64 = (locals.var_grgeltd + locals.var_gcrg);
        (assign25000_e38457, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25000_e38459;
        locals.var_t11_dn3 = assign25000_e38459_d_n3;
        locals.var_t11_dn4 = assign25000_e38459_d_n4;
        locals.var_t11_dn5 = assign25000_e38459_d_n5;
        locals.var_t11_dn6 = assign25000_e38459_d_n6;
        locals.var_t11_dn7 = assign25000_e38459_d_n7;
        locals.var_t11_dn8 = assign25000_e38459_d_n8;
        locals.var_t11_dn9 = assign25000_e38459_d_n9;
        locals.var_t11_dn10 = assign25000_e38459_d_n10;
        locals.var_t11_dn11 = assign25000_e38459_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25010_e38471, assign25010_e38471_d_n3, assign25010_e38471_d_n4, assign25010_e38471_d_n5, assign25010_e38471_d_n6, assign25010_e38471_d_n7, assign25010_e38471_d_n8, assign25010_e38471_d_n9, assign25010_e38471_d_n10, assign25010_e38471_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard539 != 0.0)) && (locals.var_guard540 != 0.0)) {
        let assign25010_e38467: f64 = (locals.var_grgeltd * locals.var_gcrg);
        let assign25010_e38469: f64 = (assign25010_e38467 / locals.var_t11);
        (assign25010_e38469, ((((locals.var_grgeltd * locals.var_gcrg_dn3) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn4) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn5) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn6) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn7) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn8) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn9) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn10) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn11) * locals.var_t11) - (assign25010_e38467 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign25010_e38471;
        locals.var_gcrg_dn3 = assign25010_e38471_d_n3;
        locals.var_gcrg_dn4 = assign25010_e38471_d_n4;
        locals.var_gcrg_dn5 = assign25010_e38471_d_n5;
        locals.var_gcrg_dn6 = assign25010_e38471_d_n6;
        locals.var_gcrg_dn7 = assign25010_e38471_d_n7;
        locals.var_gcrg_dn8 = assign25010_e38471_d_n8;
        locals.var_gcrg_dn9 = assign25010_e38471_d_n9;
        locals.var_gcrg_dn10 = assign25010_e38471_d_n10;
        locals.var_gcrg_dn11 = assign25010_e38471_d_n11;
        locals.var_gcrg_rv = 0.0;

        let (assign25020_e38479,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25020_e38475: f64 = (locals.var_weff / p.p1373);
        let assign25020_e38477: f64 = (assign25020_e38475 + p.p1377);
        (assign25020_e38477,)
    } else {
        (locals.var_wdiod,)
    }
};
        locals.var_wdiod = assign25020_e38479;
        locals.var_wdiod_rv = 0.0;

        let (assign25030_e38487,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25030_e38483: f64 = (locals.var_weff / p.p1373);
        let assign25030_e38485: f64 = (assign25030_e38483 + p.p1378);
        (assign25030_e38485,)
    } else {
        (locals.var_wdios,)
    }
};
        locals.var_wdios = assign25030_e38487;
        locals.var_wdios_rv = 0.0;

        let (assign25040_e38493,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25040_e38491: f64 = (locals.var_wdios * p.p74);
        (assign25040_e38491,)
    } else {
        (locals.var_wstsi,)
    }
};
        locals.var_wstsi = assign25040_e38493;
        locals.var_wstsi_rv = 0.0;

        let (assign25050_e38499,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25050_e38497: f64 = (locals.var_wdiod * p.p74);
        (assign25050_e38497,)
    } else {
        (locals.var_wdtsi,)
    }
};
        locals.var_wdtsi = assign25050_e38499;
        locals.var_wdtsi_rv = 0.0;

        let (assign25060_e38505, assign25060_e38505_d_n4, assign25060_e38505_d_n5,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25060_e38503: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign25060_e38503, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5,)
    }
};
        locals.var_nvtm1 = assign25060_e38505;
        locals.var_nvtm1_dn4 = assign25060_e38505_d_n4;
        locals.var_nvtm1_dn5 = assign25060_e38505_d_n5;
        locals.var_nvtm1_rv = 0.0;

        let (assign25070_e38511, assign25070_e38511_d_n3, assign25070_e38511_d_n4, assign25070_e38511_d_n5, assign25070_e38511_d_n6, assign25070_e38511_d_n7, assign25070_e38511_d_n8, assign25070_e38511_d_n9, assign25070_e38511_d_n10, assign25070_e38511_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25070_e38509: f64 = (locals.var_vbs_jct / locals.var_nvtm1);
        (assign25070_e38509, 0.0, (-((locals.var_vbs_jct * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vbs_jct * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtm1), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtm1), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25070_e38511;
        locals.var_t0_dn3 = assign25070_e38511_d_n3;
        locals.var_t0_dn4 = assign25070_e38511_d_n4;
        locals.var_t0_dn5 = assign25070_e38511_d_n5;
        locals.var_t0_dn6 = assign25070_e38511_d_n6;
        locals.var_t0_dn7 = assign25070_e38511_d_n7;
        locals.var_t0_dn8 = assign25070_e38511_d_n8;
        locals.var_t0_dn9 = assign25070_e38511_d_n9;
        locals.var_t0_dn10 = assign25070_e38511_d_n10;
        locals.var_t0_dn11 = assign25070_e38511_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25080_e38516, assign25080_e38516_d_n3, assign25080_e38516_d_n4, assign25080_e38516_d_n5, assign25080_e38516_d_n6, assign25080_e38516_d_n7, assign25080_e38516_d_n8, assign25080_e38516_d_n9, assign25080_e38516_d_n10, assign25080_e38516_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25080_e38514: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25080_e38514, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11,)
    }
};
        locals.var_expvbsnvtm = assign25080_e38516;
        locals.var_expvbsnvtm_dn3 = assign25080_e38516_d_n3;
        locals.var_expvbsnvtm_dn4 = assign25080_e38516_d_n4;
        locals.var_expvbsnvtm_dn5 = assign25080_e38516_d_n5;
        locals.var_expvbsnvtm_dn6 = assign25080_e38516_d_n6;
        locals.var_expvbsnvtm_dn7 = assign25080_e38516_d_n7;
        locals.var_expvbsnvtm_dn8 = assign25080_e38516_d_n8;
        locals.var_expvbsnvtm_dn9 = assign25080_e38516_d_n9;
        locals.var_expvbsnvtm_dn10 = assign25080_e38516_d_n10;
        locals.var_expvbsnvtm_dn11 = assign25080_e38516_d_n11;
        locals.var_expvbsnvtm_rv = 0.0;

        let (assign25090_e38522, assign25090_e38522_d_n4, assign25090_e38522_d_n5,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25090_e38520: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign25090_e38520, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign25090_e38522;
        locals.var_nvtm2_dn4 = assign25090_e38522_d_n4;
        locals.var_nvtm2_dn5 = assign25090_e38522_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let (assign25100_e38528, assign25100_e38528_d_n3, assign25100_e38528_d_n4, assign25100_e38528_d_n5, assign25100_e38528_d_n6, assign25100_e38528_d_n7, assign25100_e38528_d_n8, assign25100_e38528_d_n9, assign25100_e38528_d_n10, assign25100_e38528_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25100_e38526: f64 = (locals.var_vbd_jct / locals.var_nvtm2);
        (assign25100_e38526, 0.0, (-((locals.var_vbd_jct * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))), (-((locals.var_vbd_jct * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))), (locals.var_vbd_jct_dn6 / locals.var_nvtm2), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtm2), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25100_e38528;
        locals.var_t0_dn3 = assign25100_e38528_d_n3;
        locals.var_t0_dn4 = assign25100_e38528_d_n4;
        locals.var_t0_dn5 = assign25100_e38528_d_n5;
        locals.var_t0_dn6 = assign25100_e38528_d_n6;
        locals.var_t0_dn7 = assign25100_e38528_d_n7;
        locals.var_t0_dn8 = assign25100_e38528_d_n8;
        locals.var_t0_dn9 = assign25100_e38528_d_n9;
        locals.var_t0_dn10 = assign25100_e38528_d_n10;
        locals.var_t0_dn11 = assign25100_e38528_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25110_e38533, assign25110_e38533_d_n3, assign25110_e38533_d_n4, assign25110_e38533_d_n5, assign25110_e38533_d_n6, assign25110_e38533_d_n7, assign25110_e38533_d_n8, assign25110_e38533_d_n9, assign25110_e38533_d_n10, assign25110_e38533_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25110_e38531: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25110_e38531, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbdnvtm, locals.var_expvbdnvtm_dn3, locals.var_expvbdnvtm_dn4, locals.var_expvbdnvtm_dn5, locals.var_expvbdnvtm_dn6, locals.var_expvbdnvtm_dn7, locals.var_expvbdnvtm_dn8, locals.var_expvbdnvtm_dn9, locals.var_expvbdnvtm_dn10, locals.var_expvbdnvtm_dn11,)
    }
};
        locals.var_expvbdnvtm = assign25110_e38533;
        locals.var_expvbdnvtm_dn3 = assign25110_e38533_d_n3;
        locals.var_expvbdnvtm_dn4 = assign25110_e38533_d_n4;
        locals.var_expvbdnvtm_dn5 = assign25110_e38533_d_n5;
        locals.var_expvbdnvtm_dn6 = assign25110_e38533_d_n6;
        locals.var_expvbdnvtm_dn7 = assign25110_e38533_d_n7;
        locals.var_expvbdnvtm_dn8 = assign25110_e38533_d_n8;
        locals.var_expvbdnvtm_dn9 = assign25110_e38533_d_n9;
        locals.var_expvbdnvtm_dn10 = assign25110_e38533_d_n10;
        locals.var_expvbdnvtm_dn11 = assign25110_e38533_d_n11;
        locals.var_expvbdnvtm_rv = 0.0;

        let (assign25120_e38543, assign25120_e38543_d_n3, assign25120_e38543_d_n4, assign25120_e38543_d_n5, assign25120_e38543_d_n6, assign25120_e38543_d_n7, assign25120_e38543_d_n8, assign25120_e38543_d_n9, assign25120_e38543_d_n10, assign25120_e38543_d_n11,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25120_e38537: f64 = (1.115 / locals.var_vtm);
        let assign25120_e38540: f64 = (locals.var_tratio - 1.0);
        let assign25120_e38541: f64 = (assign25120_e38537 * assign25120_e38540);
        (assign25120_e38541, 0.0, (((-((1.115 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) * assign25120_e38540) + (assign25120_e38537 * locals.var_tratio_dn4)), (((-((1.115 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) * assign25120_e38540) + (assign25120_e38537 * locals.var_tratio_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign25120_e38543;
        locals.var_t4_dn3 = assign25120_e38543_d_n3;
        locals.var_t4_dn4 = assign25120_e38543_d_n4;
        locals.var_t4_dn5 = assign25120_e38543_d_n5;
        locals.var_t4_dn6 = assign25120_e38543_d_n6;
        locals.var_t4_dn7 = assign25120_e38543_d_n7;
        locals.var_t4_dn8 = assign25120_e38543_d_n8;
        locals.var_t4_dn9 = assign25120_e38543_d_n9;
        locals.var_t4_dn10 = assign25120_e38543_d_n10;
        locals.var_t4_dn11 = assign25120_e38543_d_n11;
        locals.var_t4_rv = 0.0;

        let assign25130_e38546: f64 = if locals.var_isdif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign25130_e38546;
        locals.var_guard542_rv = 0.0;

        let (assign25150_e38563, assign25150_e38563_d_n3, assign25150_e38563_d_n4, assign25150_e38563_d_n5, assign25150_e38563_d_n6, assign25150_e38563_d_n7, assign25150_e38563_d_n8, assign25150_e38563_d_n9, assign25150_e38563_d_n10, assign25150_e38563_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25150_e38559: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign25150_e38561: f64 = (assign25150_e38559 / locals.var_ndiode_i);
        (assign25150_e38561, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25150_e38563;
        locals.var_t7_dn3 = assign25150_e38563_d_n3;
        locals.var_t7_dn4 = assign25150_e38563_d_n4;
        locals.var_t7_dn5 = assign25150_e38563_d_n5;
        locals.var_t7_dn6 = assign25150_e38563_d_n6;
        locals.var_t7_dn7 = assign25150_e38563_d_n7;
        locals.var_t7_dn8 = assign25150_e38563_d_n8;
        locals.var_t7_dn9 = assign25150_e38563_d_n9;
        locals.var_t7_dn10 = assign25150_e38563_d_n10;
        locals.var_t7_dn11 = assign25150_e38563_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign25160_e38571, assign25160_e38571_d_n3, assign25160_e38571_d_n4, assign25160_e38571_d_n5, assign25160_e38571_d_n6, assign25160_e38571_d_n7, assign25160_e38571_d_n8, assign25160_e38571_d_n9, assign25160_e38571_d_n10, assign25160_e38571_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25160_e38569: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25160_e38569, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25160_e38571;
        locals.var_t1_dn3 = assign25160_e38571_d_n3;
        locals.var_t1_dn4 = assign25160_e38571_d_n4;
        locals.var_t1_dn5 = assign25160_e38571_d_n5;
        locals.var_t1_dn6 = assign25160_e38571_d_n6;
        locals.var_t1_dn7 = assign25160_e38571_d_n7;
        locals.var_t1_dn8 = assign25160_e38571_d_n8;
        locals.var_t1_dn9 = assign25160_e38571_d_n9;
        locals.var_t1_dn10 = assign25160_e38571_d_n10;
        locals.var_t1_dn11 = assign25160_e38571_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25170_e38580, assign25170_e38580_d_n3, assign25170_e38580_d_n4, assign25170_e38580_d_n5, assign25170_e38580_d_n6, assign25170_e38580_d_n7, assign25170_e38580_d_n8, assign25170_e38580_d_n9, assign25170_e38580_d_n10, assign25170_e38580_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25170_e38578: f64 = (locals.var_isdif_i * locals.var_t1);
        (assign25170_e38578, (locals.var_isdif_i * locals.var_t1_dn3), (locals.var_isdif_i * locals.var_t1_dn4), (locals.var_isdif_i * locals.var_t1_dn5), (locals.var_isdif_i * locals.var_t1_dn6), (locals.var_isdif_i * locals.var_t1_dn7), (locals.var_isdif_i * locals.var_t1_dn8), (locals.var_isdif_i * locals.var_t1_dn9), (locals.var_isdif_i * locals.var_t1_dn10), (locals.var_isdif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11,)
    }
};
        locals.var_jdifs = assign25170_e38580;
        locals.var_jdifs_dn3 = assign25170_e38580_d_n3;
        locals.var_jdifs_dn4 = assign25170_e38580_d_n4;
        locals.var_jdifs_dn5 = assign25170_e38580_d_n5;
        locals.var_jdifs_dn6 = assign25170_e38580_d_n6;
        locals.var_jdifs_dn7 = assign25170_e38580_d_n7;
        locals.var_jdifs_dn8 = assign25170_e38580_d_n8;
        locals.var_jdifs_dn9 = assign25170_e38580_d_n9;
        locals.var_jdifs_dn10 = assign25170_e38580_d_n10;
        locals.var_jdifs_dn11 = assign25170_e38580_d_n11;
        locals.var_jdifs_rv = 0.0;

        let (assign25180_e38589, assign25180_e38589_d_n3, assign25180_e38589_d_n4, assign25180_e38589_d_n5, assign25180_e38589_d_n6, assign25180_e38589_d_n7, assign25180_e38589_d_n8, assign25180_e38589_d_n9, assign25180_e38589_d_n10, assign25180_e38589_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard542 == 0.0)) {
        let assign25180_e38587: f64 = (locals.var_wstsi * locals.var_jdifs);
        (assign25180_e38587, (locals.var_wstsi * locals.var_jdifs_dn3), (locals.var_wstsi * locals.var_jdifs_dn4), (locals.var_wstsi * locals.var_jdifs_dn5), (locals.var_wstsi * locals.var_jdifs_dn6), (locals.var_wstsi * locals.var_jdifs_dn7), (locals.var_wstsi * locals.var_jdifs_dn8), (locals.var_wstsi * locals.var_jdifs_dn9), (locals.var_wstsi * locals.var_jdifs_dn10), (locals.var_wstsi * locals.var_jdifs_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25180_e38589;
        locals.var_t0_dn3 = assign25180_e38589_d_n3;
        locals.var_t0_dn4 = assign25180_e38589_d_n4;
        locals.var_t0_dn5 = assign25180_e38589_d_n5;
        locals.var_t0_dn6 = assign25180_e38589_d_n6;
        locals.var_t0_dn7 = assign25180_e38589_d_n7;
        locals.var_t0_dn8 = assign25180_e38589_d_n8;
        locals.var_t0_dn9 = assign25180_e38589_d_n9;
        locals.var_t0_dn10 = assign25180_e38589_d_n10;
        locals.var_t0_dn11 = assign25180_e38589_d_n11;
        locals.var_t0_rv = 0.0;

        let assign25200_e38603: f64 = if locals.var_iddif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign25200_e38603;
        locals.var_guard543_rv = 0.0;

        let (assign25220_e38620, assign25220_e38620_d_n3, assign25220_e38620_d_n4, assign25220_e38620_d_n5, assign25220_e38620_d_n6, assign25220_e38620_d_n7, assign25220_e38620_d_n8, assign25220_e38620_d_n9, assign25220_e38620_d_n10, assign25220_e38620_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25220_e38616: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign25220_e38618: f64 = (assign25220_e38616 / locals.var_ndiode_i);
        (assign25220_e38618, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25220_e38620;
        locals.var_t7_dn3 = assign25220_e38620_d_n3;
        locals.var_t7_dn4 = assign25220_e38620_d_n4;
        locals.var_t7_dn5 = assign25220_e38620_d_n5;
        locals.var_t7_dn6 = assign25220_e38620_d_n6;
        locals.var_t7_dn7 = assign25220_e38620_d_n7;
        locals.var_t7_dn8 = assign25220_e38620_d_n8;
        locals.var_t7_dn9 = assign25220_e38620_d_n9;
        locals.var_t7_dn10 = assign25220_e38620_d_n10;
        locals.var_t7_dn11 = assign25220_e38620_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign25230_e38628, assign25230_e38628_d_n3, assign25230_e38628_d_n4, assign25230_e38628_d_n5, assign25230_e38628_d_n6, assign25230_e38628_d_n7, assign25230_e38628_d_n8, assign25230_e38628_d_n9, assign25230_e38628_d_n10, assign25230_e38628_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25230_e38626: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25230_e38626, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25230_e38628;
        locals.var_t1_dn3 = assign25230_e38628_d_n3;
        locals.var_t1_dn4 = assign25230_e38628_d_n4;
        locals.var_t1_dn5 = assign25230_e38628_d_n5;
        locals.var_t1_dn6 = assign25230_e38628_d_n6;
        locals.var_t1_dn7 = assign25230_e38628_d_n7;
        locals.var_t1_dn8 = assign25230_e38628_d_n8;
        locals.var_t1_dn9 = assign25230_e38628_d_n9;
        locals.var_t1_dn10 = assign25230_e38628_d_n10;
        locals.var_t1_dn11 = assign25230_e38628_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25240_e38637, assign25240_e38637_d_n3, assign25240_e38637_d_n4, assign25240_e38637_d_n5, assign25240_e38637_d_n6, assign25240_e38637_d_n7, assign25240_e38637_d_n8, assign25240_e38637_d_n9, assign25240_e38637_d_n10, assign25240_e38637_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25240_e38635: f64 = (locals.var_iddif_i * locals.var_t1);
        (assign25240_e38635, (locals.var_iddif_i * locals.var_t1_dn3), (locals.var_iddif_i * locals.var_t1_dn4), (locals.var_iddif_i * locals.var_t1_dn5), (locals.var_iddif_i * locals.var_t1_dn6), (locals.var_iddif_i * locals.var_t1_dn7), (locals.var_iddif_i * locals.var_t1_dn8), (locals.var_iddif_i * locals.var_t1_dn9), (locals.var_iddif_i * locals.var_t1_dn10), (locals.var_iddif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11,)
    }
};
        locals.var_jdifd = assign25240_e38637;
        locals.var_jdifd_dn3 = assign25240_e38637_d_n3;
        locals.var_jdifd_dn4 = assign25240_e38637_d_n4;
        locals.var_jdifd_dn5 = assign25240_e38637_d_n5;
        locals.var_jdifd_dn6 = assign25240_e38637_d_n6;
        locals.var_jdifd_dn7 = assign25240_e38637_d_n7;
        locals.var_jdifd_dn8 = assign25240_e38637_d_n8;
        locals.var_jdifd_dn9 = assign25240_e38637_d_n9;
        locals.var_jdifd_dn10 = assign25240_e38637_d_n10;
        locals.var_jdifd_dn11 = assign25240_e38637_d_n11;
        locals.var_jdifd_rv = 0.0;

        let (assign25250_e38646, assign25250_e38646_d_n3, assign25250_e38646_d_n4, assign25250_e38646_d_n5, assign25250_e38646_d_n6, assign25250_e38646_d_n7, assign25250_e38646_d_n8, assign25250_e38646_d_n9, assign25250_e38646_d_n10, assign25250_e38646_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard543 == 0.0)) {
        let assign25250_e38644: f64 = (locals.var_wdtsi * locals.var_jdifd);
        (assign25250_e38644, (locals.var_wdtsi * locals.var_jdifd_dn3), (locals.var_wdtsi * locals.var_jdifd_dn4), (locals.var_wdtsi * locals.var_jdifd_dn5), (locals.var_wdtsi * locals.var_jdifd_dn6), (locals.var_wdtsi * locals.var_jdifd_dn7), (locals.var_wdtsi * locals.var_jdifd_dn8), (locals.var_wdtsi * locals.var_jdifd_dn9), (locals.var_wdtsi * locals.var_jdifd_dn10), (locals.var_wdtsi * locals.var_jdifd_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25250_e38646;
        locals.var_t0_dn3 = assign25250_e38646_d_n3;
        locals.var_t0_dn4 = assign25250_e38646_d_n4;
        locals.var_t0_dn5 = assign25250_e38646_d_n5;
        locals.var_t0_dn6 = assign25250_e38646_d_n6;
        locals.var_t0_dn7 = assign25250_e38646_d_n7;
        locals.var_t0_dn8 = assign25250_e38646_d_n8;
        locals.var_t0_dn9 = assign25250_e38646_d_n9;
        locals.var_t0_dn10 = assign25250_e38646_d_n10;
        locals.var_t0_dn11 = assign25250_e38646_d_n11;
        locals.var_t0_rv = 0.0;

        let assign25270_e38660: f64 = if locals.var_isrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign25270_e38660;
        locals.var_guard544_rv = 0.0;

        let (assign25290_e38677, assign25290_e38677_d_n3, assign25290_e38677_d_n4, assign25290_e38677_d_n5, assign25290_e38677_d_n6, assign25290_e38677_d_n7, assign25290_e38677_d_n8, assign25290_e38677_d_n9, assign25290_e38677_d_n10, assign25290_e38677_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25290_e38673: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign25290_e38675: f64 = (assign25290_e38673 / locals.var_nrecf0_i);
        (assign25290_e38675, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25290_e38677;
        locals.var_t7_dn3 = assign25290_e38677_d_n3;
        locals.var_t7_dn4 = assign25290_e38677_d_n4;
        locals.var_t7_dn5 = assign25290_e38677_d_n5;
        locals.var_t7_dn6 = assign25290_e38677_d_n6;
        locals.var_t7_dn7 = assign25290_e38677_d_n7;
        locals.var_t7_dn8 = assign25290_e38677_d_n8;
        locals.var_t7_dn9 = assign25290_e38677_d_n9;
        locals.var_t7_dn10 = assign25290_e38677_d_n10;
        locals.var_t7_dn11 = assign25290_e38677_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign25300_e38685, assign25300_e38685_d_n3, assign25300_e38685_d_n4, assign25300_e38685_d_n5, assign25300_e38685_d_n6, assign25300_e38685_d_n7, assign25300_e38685_d_n8, assign25300_e38685_d_n9, assign25300_e38685_d_n10, assign25300_e38685_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25300_e38683: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25300_e38683, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign25300_e38685;
        locals.var_t2_dn3 = assign25300_e38685_d_n3;
        locals.var_t2_dn4 = assign25300_e38685_d_n4;
        locals.var_t2_dn5 = assign25300_e38685_d_n5;
        locals.var_t2_dn6 = assign25300_e38685_d_n6;
        locals.var_t2_dn7 = assign25300_e38685_d_n7;
        locals.var_t2_dn8 = assign25300_e38685_d_n8;
        locals.var_t2_dn9 = assign25300_e38685_d_n9;
        locals.var_t2_dn10 = assign25300_e38685_d_n10;
        locals.var_t2_dn11 = assign25300_e38685_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign25310_e38694, assign25310_e38694_d_n3, assign25310_e38694_d_n4, assign25310_e38694_d_n5, assign25310_e38694_d_n6, assign25310_e38694_d_n7, assign25310_e38694_d_n8, assign25310_e38694_d_n9, assign25310_e38694_d_n10, assign25310_e38694_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25310_e38692: f64 = (locals.var_isrec_i * locals.var_t2);
        (assign25310_e38692, (locals.var_isrec_i * locals.var_t2_dn3), (locals.var_isrec_i * locals.var_t2_dn4), (locals.var_isrec_i * locals.var_t2_dn5), (locals.var_isrec_i * locals.var_t2_dn6), (locals.var_isrec_i * locals.var_t2_dn7), (locals.var_isrec_i * locals.var_t2_dn8), (locals.var_isrec_i * locals.var_t2_dn9), (locals.var_isrec_i * locals.var_t2_dn10), (locals.var_isrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11,)
    }
};
        locals.var_jrecs = assign25310_e38694;
        locals.var_jrecs_dn3 = assign25310_e38694_d_n3;
        locals.var_jrecs_dn4 = assign25310_e38694_d_n4;
        locals.var_jrecs_dn5 = assign25310_e38694_d_n5;
        locals.var_jrecs_dn6 = assign25310_e38694_d_n6;
        locals.var_jrecs_dn7 = assign25310_e38694_d_n7;
        locals.var_jrecs_dn8 = assign25310_e38694_d_n8;
        locals.var_jrecs_dn9 = assign25310_e38694_d_n9;
        locals.var_jrecs_dn10 = assign25310_e38694_d_n10;
        locals.var_jrecs_dn11 = assign25310_e38694_d_n11;
        locals.var_jrecs_rv = 0.0;

        let (assign25320_e38711, assign25320_e38711_d_n4, assign25320_e38711_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25320_e38701: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign25320_e38706: f64 = (locals.var_tratio - 1.0);
        let assign25320_e38707: f64 = (locals.var_ntrecf_i * assign25320_e38706);
        let assign25320_e38708: f64 = (1.0 + assign25320_e38707);
        let assign25320_e38709: f64 = (assign25320_e38701 * assign25320_e38708);
        (assign25320_e38709, (assign25320_e38701 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign25320_e38701 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign25320_e38711;
        locals.var_nvtmf_dn4 = assign25320_e38711_d_n4;
        locals.var_nvtmf_dn5 = assign25320_e38711_d_n5;
        locals.var_nvtmf_rv = 0.0;

        let (assign25330_e38728, assign25330_e38728_d_n4, assign25330_e38728_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25330_e38718: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign25330_e38723: f64 = (locals.var_tratio - 1.0);
        let assign25330_e38724: f64 = (locals.var_ntrecr_i * assign25330_e38723);
        let assign25330_e38725: f64 = (1.0 + assign25330_e38724);
        let assign25330_e38726: f64 = (assign25330_e38718 * assign25330_e38725);
        (assign25330_e38726, (assign25330_e38718 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign25330_e38718 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign25330_e38728;
        locals.var_nvtmr_dn4 = assign25330_e38728_d_n4;
        locals.var_nvtmr_dn5 = assign25330_e38728_d_n5;
        locals.var_nvtmr_rv = 0.0;

        let (assign25340_e38737, assign25340_e38737_d_n3, assign25340_e38737_d_n4, assign25340_e38737_d_n5, assign25340_e38737_d_n6, assign25340_e38737_d_n7, assign25340_e38737_d_n8, assign25340_e38737_d_n9, assign25340_e38737_d_n10, assign25340_e38737_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25340_e38735: f64 = (locals.var_vbs_jct / locals.var_nvtmf);
        (assign25340_e38735, 0.0, (-((locals.var_vbs_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbs_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtmf), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25340_e38737;
        locals.var_t0_dn3 = assign25340_e38737_d_n3;
        locals.var_t0_dn4 = assign25340_e38737_d_n4;
        locals.var_t0_dn5 = assign25340_e38737_d_n5;
        locals.var_t0_dn6 = assign25340_e38737_d_n6;
        locals.var_t0_dn7 = assign25340_e38737_d_n7;
        locals.var_t0_dn8 = assign25340_e38737_d_n8;
        locals.var_t0_dn9 = assign25340_e38737_d_n9;
        locals.var_t0_dn10 = assign25340_e38737_d_n10;
        locals.var_t0_dn11 = assign25340_e38737_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25350_e38745, assign25350_e38745_d_n3, assign25350_e38745_d_n4, assign25350_e38745_d_n5, assign25350_e38745_d_n6, assign25350_e38745_d_n7, assign25350_e38745_d_n8, assign25350_e38745_d_n9, assign25350_e38745_d_n10, assign25350_e38745_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25350_e38743: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25350_e38743, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign25350_e38745;
        locals.var_t10_dn3 = assign25350_e38745_d_n3;
        locals.var_t10_dn4 = assign25350_e38745_d_n4;
        locals.var_t10_dn5 = assign25350_e38745_d_n5;
        locals.var_t10_dn6 = assign25350_e38745_d_n6;
        locals.var_t10_dn7 = assign25350_e38745_d_n7;
        locals.var_t10_dn8 = assign25350_e38745_d_n8;
        locals.var_t10_dn9 = assign25350_e38745_d_n9;
        locals.var_t10_dn10 = assign25350_e38745_d_n10;
        locals.var_t10_dn11 = assign25350_e38745_d_n11;
        locals.var_t10_rv = 0.0;

        let assign25360_e38748: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign25360_e38750: f64 = if assign25360_e38748 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign25360_e38750;
        locals.var_guard545_rv = 0.0;

        let (assign25370_e38759, assign25370_e38759_d_n3, assign25370_e38759_d_n4, assign25370_e38759_d_n5, assign25370_e38759_d_n6, assign25370_e38759_d_n7, assign25370_e38759_d_n8, assign25370_e38759_d_n9, assign25370_e38759_d_n10, assign25370_e38759_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25370_e38759;
        locals.var_t1_dn3 = assign25370_e38759_d_n3;
        locals.var_t1_dn4 = assign25370_e38759_d_n4;
        locals.var_t1_dn5 = assign25370_e38759_d_n5;
        locals.var_t1_dn6 = assign25370_e38759_d_n6;
        locals.var_t1_dn7 = assign25370_e38759_d_n7;
        locals.var_t1_dn8 = assign25370_e38759_d_n8;
        locals.var_t1_dn9 = assign25370_e38759_d_n9;
        locals.var_t1_dn10 = assign25370_e38759_d_n10;
        locals.var_t1_dn11 = assign25370_e38759_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25380_e38775, assign25380_e38775_d_n3, assign25380_e38775_d_n4, assign25380_e38775_d_n5, assign25380_e38775_d_n6, assign25380_e38775_d_n7, assign25380_e38775_d_n8, assign25380_e38775_d_n9, assign25380_e38775_d_n10, assign25380_e38775_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign25380_e38767: f64 = (-locals.var_vbs_jct);
        let assign25380_e38769: f64 = (assign25380_e38767 / locals.var_nvtmr);
        let assign25380_e38771: f64 = (assign25380_e38769 * locals.var_vrec0_i);
        let assign25380_e38773: f64 = (assign25380_e38771 * locals.var_t1);
        (assign25380_e38773, (assign25380_e38771 * locals.var_t1_dn3), ((((-((assign25380_e38767 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn4)), ((((-((assign25380_e38767 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn5)), (assign25380_e38771 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn7)), (assign25380_e38771 * locals.var_t1_dn8), (assign25380_e38771 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25380_e38771 * locals.var_t1_dn10)), (assign25380_e38771 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25380_e38775;
        locals.var_t0_dn3 = assign25380_e38775_d_n3;
        locals.var_t0_dn4 = assign25380_e38775_d_n4;
        locals.var_t0_dn5 = assign25380_e38775_d_n5;
        locals.var_t0_dn6 = assign25380_e38775_d_n6;
        locals.var_t0_dn7 = assign25380_e38775_d_n7;
        locals.var_t0_dn8 = assign25380_e38775_d_n8;
        locals.var_t0_dn9 = assign25380_e38775_d_n9;
        locals.var_t0_dn10 = assign25380_e38775_d_n10;
        locals.var_t0_dn11 = assign25380_e38775_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25390_e38785, assign25390_e38785_d_n3, assign25390_e38785_d_n4, assign25390_e38785_d_n5, assign25390_e38785_d_n6, assign25390_e38785_d_n7, assign25390_e38785_d_n8, assign25390_e38785_d_n9, assign25390_e38785_d_n10, assign25390_e38785_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign25390_e38783: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25390_e38783, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25390_e38785;
        locals.var_t11_dn3 = assign25390_e38785_d_n3;
        locals.var_t11_dn4 = assign25390_e38785_d_n4;
        locals.var_t11_dn5 = assign25390_e38785_d_n5;
        locals.var_t11_dn6 = assign25390_e38785_d_n6;
        locals.var_t11_dn7 = assign25390_e38785_d_n7;
        locals.var_t11_dn8 = assign25390_e38785_d_n8;
        locals.var_t11_dn9 = assign25390_e38785_d_n9;
        locals.var_t11_dn10 = assign25390_e38785_d_n10;
        locals.var_t11_dn11 = assign25390_e38785_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25400_e38795, assign25400_e38795_d_n3, assign25400_e38795_d_n4, assign25400_e38795_d_n5, assign25400_e38795_d_n6, assign25400_e38795_d_n7, assign25400_e38795_d_n8, assign25400_e38795_d_n9, assign25400_e38795_d_n10, assign25400_e38795_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 != 0.0)) {
        let assign25400_e38793: f64 = (-locals.var_t11);
        (assign25400_e38793, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25400_e38795;
        locals.var_t11_dn3 = assign25400_e38795_d_n3;
        locals.var_t11_dn4 = assign25400_e38795_d_n4;
        locals.var_t11_dn5 = assign25400_e38795_d_n5;
        locals.var_t11_dn6 = assign25400_e38795_d_n6;
        locals.var_t11_dn7 = assign25400_e38795_d_n7;
        locals.var_t11_dn8 = assign25400_e38795_d_n8;
        locals.var_t11_dn9 = assign25400_e38795_d_n9;
        locals.var_t11_dn10 = assign25400_e38795_d_n10;
        locals.var_t11_dn11 = assign25400_e38795_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25410_e38809, assign25410_e38809_d_n3, assign25410_e38809_d_n4, assign25410_e38809_d_n5, assign25410_e38809_d_n6, assign25410_e38809_d_n7, assign25410_e38809_d_n8, assign25410_e38809_d_n9, assign25410_e38809_d_n10, assign25410_e38809_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25410_e38806: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign25410_e38807: f64 = (1.0 / assign25410_e38806);
        (assign25410_e38807, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign25410_e38806 * assign25410_e38806))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign25410_e38806 * assign25410_e38806))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25410_e38809;
        locals.var_t1_dn3 = assign25410_e38809_d_n3;
        locals.var_t1_dn4 = assign25410_e38809_d_n4;
        locals.var_t1_dn5 = assign25410_e38809_d_n5;
        locals.var_t1_dn6 = assign25410_e38809_d_n6;
        locals.var_t1_dn7 = assign25410_e38809_d_n7;
        locals.var_t1_dn8 = assign25410_e38809_d_n8;
        locals.var_t1_dn9 = assign25410_e38809_d_n9;
        locals.var_t1_dn10 = assign25410_e38809_d_n10;
        locals.var_t1_dn11 = assign25410_e38809_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25420_e38826, assign25420_e38826_d_n3, assign25420_e38826_d_n4, assign25420_e38826_d_n5, assign25420_e38826_d_n6, assign25420_e38826_d_n7, assign25420_e38826_d_n8, assign25420_e38826_d_n9, assign25420_e38826_d_n10, assign25420_e38826_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25420_e38818: f64 = (-locals.var_vbs_jct);
        let assign25420_e38820: f64 = (assign25420_e38818 / locals.var_nvtmr);
        let assign25420_e38822: f64 = (assign25420_e38820 * locals.var_vrec0_i);
        let assign25420_e38824: f64 = (assign25420_e38822 * locals.var_t1);
        (assign25420_e38824, (assign25420_e38822 * locals.var_t1_dn3), ((((-((assign25420_e38818 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn4)), ((((-((assign25420_e38818 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn5)), (assign25420_e38822 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn7)), (assign25420_e38822 * locals.var_t1_dn8), (assign25420_e38822 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign25420_e38822 * locals.var_t1_dn10)), (assign25420_e38822 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25420_e38826;
        locals.var_t0_dn3 = assign25420_e38826_d_n3;
        locals.var_t0_dn4 = assign25420_e38826_d_n4;
        locals.var_t0_dn5 = assign25420_e38826_d_n5;
        locals.var_t0_dn6 = assign25420_e38826_d_n6;
        locals.var_t0_dn7 = assign25420_e38826_d_n7;
        locals.var_t0_dn8 = assign25420_e38826_d_n8;
        locals.var_t0_dn9 = assign25420_e38826_d_n9;
        locals.var_t0_dn10 = assign25420_e38826_d_n10;
        locals.var_t0_dn11 = assign25420_e38826_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25430_e38837, assign25430_e38837_d_n3, assign25430_e38837_d_n4, assign25430_e38837_d_n5, assign25430_e38837_d_n6, assign25430_e38837_d_n7, assign25430_e38837_d_n8, assign25430_e38837_d_n9, assign25430_e38837_d_n10, assign25430_e38837_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25430_e38835: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25430_e38835, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25430_e38837;
        locals.var_t11_dn3 = assign25430_e38837_d_n3;
        locals.var_t11_dn4 = assign25430_e38837_d_n4;
        locals.var_t11_dn5 = assign25430_e38837_d_n5;
        locals.var_t11_dn6 = assign25430_e38837_d_n6;
        locals.var_t11_dn7 = assign25430_e38837_d_n7;
        locals.var_t11_dn8 = assign25430_e38837_d_n8;
        locals.var_t11_dn9 = assign25430_e38837_d_n9;
        locals.var_t11_dn10 = assign25430_e38837_d_n10;
        locals.var_t11_dn11 = assign25430_e38837_d_n11;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25440_e38848, assign25440_e38848_d_n3, assign25440_e38848_d_n4, assign25440_e38848_d_n5, assign25440_e38848_d_n6, assign25440_e38848_d_n7, assign25440_e38848_d_n8, assign25440_e38848_d_n9, assign25440_e38848_d_n10, assign25440_e38848_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) && (locals.var_guard545 == 0.0)) {
        let assign25440_e38846: f64 = (-locals.var_t11);
        (assign25440_e38846, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25440_e38848;
        locals.var_t11_dn3 = assign25440_e38848_d_n3;
        locals.var_t11_dn4 = assign25440_e38848_d_n4;
        locals.var_t11_dn5 = assign25440_e38848_d_n5;
        locals.var_t11_dn6 = assign25440_e38848_d_n6;
        locals.var_t11_dn7 = assign25440_e38848_d_n7;
        locals.var_t11_dn8 = assign25440_e38848_d_n8;
        locals.var_t11_dn9 = assign25440_e38848_d_n9;
        locals.var_t11_dn10 = assign25440_e38848_d_n10;
        locals.var_t11_dn11 = assign25440_e38848_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25450_e38857, assign25450_e38857_d_n3, assign25450_e38857_d_n4, assign25450_e38857_d_n5, assign25450_e38857_d_n6, assign25450_e38857_d_n7, assign25450_e38857_d_n8, assign25450_e38857_d_n9, assign25450_e38857_d_n10, assign25450_e38857_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard544 == 0.0)) {
        let assign25450_e38855: f64 = (locals.var_wstsi * locals.var_jrecs);
        (assign25450_e38855, (locals.var_wstsi * locals.var_jrecs_dn3), (locals.var_wstsi * locals.var_jrecs_dn4), (locals.var_wstsi * locals.var_jrecs_dn5), (locals.var_wstsi * locals.var_jrecs_dn6), (locals.var_wstsi * locals.var_jrecs_dn7), (locals.var_wstsi * locals.var_jrecs_dn8), (locals.var_wstsi * locals.var_jrecs_dn9), (locals.var_wstsi * locals.var_jrecs_dn10), (locals.var_wstsi * locals.var_jrecs_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign25450_e38857;
        locals.var_t3_dn3 = assign25450_e38857_d_n3;
        locals.var_t3_dn4 = assign25450_e38857_d_n4;
        locals.var_t3_dn5 = assign25450_e38857_d_n5;
        locals.var_t3_dn6 = assign25450_e38857_d_n6;
        locals.var_t3_dn7 = assign25450_e38857_d_n7;
        locals.var_t3_dn8 = assign25450_e38857_d_n8;
        locals.var_t3_dn9 = assign25450_e38857_d_n9;
        locals.var_t3_dn10 = assign25450_e38857_d_n10;
        locals.var_t3_dn11 = assign25450_e38857_d_n11;
        locals.var_t3_rv = 0.0;

        let assign25470_e38871: f64 = if locals.var_idrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign25470_e38871;
        locals.var_guard546_rv = 0.0;

        let (assign25490_e38888, assign25490_e38888_d_n3, assign25490_e38888_d_n4, assign25490_e38888_d_n5, assign25490_e38888_d_n6, assign25490_e38888_d_n7, assign25490_e38888_d_n8, assign25490_e38888_d_n9, assign25490_e38888_d_n10, assign25490_e38888_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25490_e38884: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign25490_e38886: f64 = (assign25490_e38884 / locals.var_nrecf0_i);
        (assign25490_e38886, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25490_e38888;
        locals.var_t7_dn3 = assign25490_e38888_d_n3;
        locals.var_t7_dn4 = assign25490_e38888_d_n4;
        locals.var_t7_dn5 = assign25490_e38888_d_n5;
        locals.var_t7_dn6 = assign25490_e38888_d_n6;
        locals.var_t7_dn7 = assign25490_e38888_d_n7;
        locals.var_t7_dn8 = assign25490_e38888_d_n8;
        locals.var_t7_dn9 = assign25490_e38888_d_n9;
        locals.var_t7_dn10 = assign25490_e38888_d_n10;
        locals.var_t7_dn11 = assign25490_e38888_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign25500_e38896, assign25500_e38896_d_n3, assign25500_e38896_d_n4, assign25500_e38896_d_n5, assign25500_e38896_d_n6, assign25500_e38896_d_n7, assign25500_e38896_d_n8, assign25500_e38896_d_n9, assign25500_e38896_d_n10, assign25500_e38896_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25500_e38894: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25500_e38894, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign25500_e38896;
        locals.var_t2_dn3 = assign25500_e38896_d_n3;
        locals.var_t2_dn4 = assign25500_e38896_d_n4;
        locals.var_t2_dn5 = assign25500_e38896_d_n5;
        locals.var_t2_dn6 = assign25500_e38896_d_n6;
        locals.var_t2_dn7 = assign25500_e38896_d_n7;
        locals.var_t2_dn8 = assign25500_e38896_d_n8;
        locals.var_t2_dn9 = assign25500_e38896_d_n9;
        locals.var_t2_dn10 = assign25500_e38896_d_n10;
        locals.var_t2_dn11 = assign25500_e38896_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign25510_e38905, assign25510_e38905_d_n3, assign25510_e38905_d_n4, assign25510_e38905_d_n5, assign25510_e38905_d_n6, assign25510_e38905_d_n7, assign25510_e38905_d_n8, assign25510_e38905_d_n9, assign25510_e38905_d_n10, assign25510_e38905_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25510_e38903: f64 = (locals.var_idrec_i * locals.var_t2);
        (assign25510_e38903, (locals.var_idrec_i * locals.var_t2_dn3), (locals.var_idrec_i * locals.var_t2_dn4), (locals.var_idrec_i * locals.var_t2_dn5), (locals.var_idrec_i * locals.var_t2_dn6), (locals.var_idrec_i * locals.var_t2_dn7), (locals.var_idrec_i * locals.var_t2_dn8), (locals.var_idrec_i * locals.var_t2_dn9), (locals.var_idrec_i * locals.var_t2_dn10), (locals.var_idrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11,)
    }
};
        locals.var_jrecd = assign25510_e38905;
        locals.var_jrecd_dn3 = assign25510_e38905_d_n3;
        locals.var_jrecd_dn4 = assign25510_e38905_d_n4;
        locals.var_jrecd_dn5 = assign25510_e38905_d_n5;
        locals.var_jrecd_dn6 = assign25510_e38905_d_n6;
        locals.var_jrecd_dn7 = assign25510_e38905_d_n7;
        locals.var_jrecd_dn8 = assign25510_e38905_d_n8;
        locals.var_jrecd_dn9 = assign25510_e38905_d_n9;
        locals.var_jrecd_dn10 = assign25510_e38905_d_n10;
        locals.var_jrecd_dn11 = assign25510_e38905_d_n11;
        locals.var_jrecd_rv = 0.0;

        let (assign25520_e38922, assign25520_e38922_d_n4, assign25520_e38922_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25520_e38912: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign25520_e38917: f64 = (locals.var_tratio - 1.0);
        let assign25520_e38918: f64 = (locals.var_ntrecf_i * assign25520_e38917);
        let assign25520_e38919: f64 = (1.0 + assign25520_e38918);
        let assign25520_e38920: f64 = (assign25520_e38912 * assign25520_e38919);
        (assign25520_e38920, (assign25520_e38912 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign25520_e38912 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign25520_e38922;
        locals.var_nvtmf_dn4 = assign25520_e38922_d_n4;
        locals.var_nvtmf_dn5 = assign25520_e38922_d_n5;
        locals.var_nvtmf_rv = 0.0;

        let (assign25530_e38939, assign25530_e38939_d_n4, assign25530_e38939_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25530_e38929: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign25530_e38934: f64 = (locals.var_tratio - 1.0);
        let assign25530_e38935: f64 = (locals.var_ntrecr_i * assign25530_e38934);
        let assign25530_e38936: f64 = (1.0 + assign25530_e38935);
        let assign25530_e38937: f64 = (assign25530_e38929 * assign25530_e38936);
        (assign25530_e38937, (assign25530_e38929 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign25530_e38929 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign25530_e38939;
        locals.var_nvtmr_dn4 = assign25530_e38939_d_n4;
        locals.var_nvtmr_dn5 = assign25530_e38939_d_n5;
        locals.var_nvtmr_rv = 0.0;

        let (assign25540_e38948, assign25540_e38948_d_n3, assign25540_e38948_d_n4, assign25540_e38948_d_n5, assign25540_e38948_d_n6, assign25540_e38948_d_n7, assign25540_e38948_d_n8, assign25540_e38948_d_n9, assign25540_e38948_d_n10, assign25540_e38948_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25540_e38946: f64 = (locals.var_vbd_jct / locals.var_nvtmf);
        (assign25540_e38946, 0.0, (-((locals.var_vbd_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbd_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), (locals.var_vbd_jct_dn6 / locals.var_nvtmf), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25540_e38948;
        locals.var_t0_dn3 = assign25540_e38948_d_n3;
        locals.var_t0_dn4 = assign25540_e38948_d_n4;
        locals.var_t0_dn5 = assign25540_e38948_d_n5;
        locals.var_t0_dn6 = assign25540_e38948_d_n6;
        locals.var_t0_dn7 = assign25540_e38948_d_n7;
        locals.var_t0_dn8 = assign25540_e38948_d_n8;
        locals.var_t0_dn9 = assign25540_e38948_d_n9;
        locals.var_t0_dn10 = assign25540_e38948_d_n10;
        locals.var_t0_dn11 = assign25540_e38948_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25550_e38956, assign25550_e38956_d_n3, assign25550_e38956_d_n4, assign25550_e38956_d_n5, assign25550_e38956_d_n6, assign25550_e38956_d_n7, assign25550_e38956_d_n8, assign25550_e38956_d_n9, assign25550_e38956_d_n10, assign25550_e38956_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25550_e38954: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25550_e38954, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign25550_e38956;
        locals.var_t10_dn3 = assign25550_e38956_d_n3;
        locals.var_t10_dn4 = assign25550_e38956_d_n4;
        locals.var_t10_dn5 = assign25550_e38956_d_n5;
        locals.var_t10_dn6 = assign25550_e38956_d_n6;
        locals.var_t10_dn7 = assign25550_e38956_d_n7;
        locals.var_t10_dn8 = assign25550_e38956_d_n8;
        locals.var_t10_dn9 = assign25550_e38956_d_n9;
        locals.var_t10_dn10 = assign25550_e38956_d_n10;
        locals.var_t10_dn11 = assign25550_e38956_d_n11;
        locals.var_t10_rv = 0.0;

        let assign25560_e38959: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign25560_e38961: f64 = if assign25560_e38959 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign25560_e38961;
        locals.var_guard547_rv = 0.0;

        let (assign25570_e38970, assign25570_e38970_d_n3, assign25570_e38970_d_n4, assign25570_e38970_d_n5, assign25570_e38970_d_n6, assign25570_e38970_d_n7, assign25570_e38970_d_n8, assign25570_e38970_d_n9, assign25570_e38970_d_n10, assign25570_e38970_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25570_e38970;
        locals.var_t1_dn3 = assign25570_e38970_d_n3;
        locals.var_t1_dn4 = assign25570_e38970_d_n4;
        locals.var_t1_dn5 = assign25570_e38970_d_n5;
        locals.var_t1_dn6 = assign25570_e38970_d_n6;
        locals.var_t1_dn7 = assign25570_e38970_d_n7;
        locals.var_t1_dn8 = assign25570_e38970_d_n8;
        locals.var_t1_dn9 = assign25570_e38970_d_n9;
        locals.var_t1_dn10 = assign25570_e38970_d_n10;
        locals.var_t1_dn11 = assign25570_e38970_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25580_e38986, assign25580_e38986_d_n3, assign25580_e38986_d_n4, assign25580_e38986_d_n5, assign25580_e38986_d_n6, assign25580_e38986_d_n7, assign25580_e38986_d_n8, assign25580_e38986_d_n9, assign25580_e38986_d_n10, assign25580_e38986_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign25580_e38978: f64 = (-locals.var_vbd_jct);
        let assign25580_e38980: f64 = (assign25580_e38978 / locals.var_nvtmr);
        let assign25580_e38982: f64 = (assign25580_e38980 * locals.var_vrec0d_i);
        let assign25580_e38984: f64 = (assign25580_e38982 * locals.var_t1);
        (assign25580_e38984, (assign25580_e38982 * locals.var_t1_dn3), ((((-((assign25580_e38978 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn4)), ((((-((assign25580_e38978 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn6)), (assign25580_e38982 * locals.var_t1_dn7), (assign25580_e38982 * locals.var_t1_dn8), (assign25580_e38982 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25580_e38982 * locals.var_t1_dn10)), (assign25580_e38982 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25580_e38986;
        locals.var_t0_dn3 = assign25580_e38986_d_n3;
        locals.var_t0_dn4 = assign25580_e38986_d_n4;
        locals.var_t0_dn5 = assign25580_e38986_d_n5;
        locals.var_t0_dn6 = assign25580_e38986_d_n6;
        locals.var_t0_dn7 = assign25580_e38986_d_n7;
        locals.var_t0_dn8 = assign25580_e38986_d_n8;
        locals.var_t0_dn9 = assign25580_e38986_d_n9;
        locals.var_t0_dn10 = assign25580_e38986_d_n10;
        locals.var_t0_dn11 = assign25580_e38986_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25590_e38996, assign25590_e38996_d_n3, assign25590_e38996_d_n4, assign25590_e38996_d_n5, assign25590_e38996_d_n6, assign25590_e38996_d_n7, assign25590_e38996_d_n8, assign25590_e38996_d_n9, assign25590_e38996_d_n10, assign25590_e38996_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign25590_e38994: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25590_e38994, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25590_e38996;
        locals.var_t11_dn3 = assign25590_e38996_d_n3;
        locals.var_t11_dn4 = assign25590_e38996_d_n4;
        locals.var_t11_dn5 = assign25590_e38996_d_n5;
        locals.var_t11_dn6 = assign25590_e38996_d_n6;
        locals.var_t11_dn7 = assign25590_e38996_d_n7;
        locals.var_t11_dn8 = assign25590_e38996_d_n8;
        locals.var_t11_dn9 = assign25590_e38996_d_n9;
        locals.var_t11_dn10 = assign25590_e38996_d_n10;
        locals.var_t11_dn11 = assign25590_e38996_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25600_e39006, assign25600_e39006_d_n3, assign25600_e39006_d_n4, assign25600_e39006_d_n5, assign25600_e39006_d_n6, assign25600_e39006_d_n7, assign25600_e39006_d_n8, assign25600_e39006_d_n9, assign25600_e39006_d_n10, assign25600_e39006_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 != 0.0)) {
        let assign25600_e39004: f64 = (-locals.var_t11);
        (assign25600_e39004, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25600_e39006;
        locals.var_t11_dn3 = assign25600_e39006_d_n3;
        locals.var_t11_dn4 = assign25600_e39006_d_n4;
        locals.var_t11_dn5 = assign25600_e39006_d_n5;
        locals.var_t11_dn6 = assign25600_e39006_d_n6;
        locals.var_t11_dn7 = assign25600_e39006_d_n7;
        locals.var_t11_dn8 = assign25600_e39006_d_n8;
        locals.var_t11_dn9 = assign25600_e39006_d_n9;
        locals.var_t11_dn10 = assign25600_e39006_d_n10;
        locals.var_t11_dn11 = assign25600_e39006_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25610_e39020, assign25610_e39020_d_n3, assign25610_e39020_d_n4, assign25610_e39020_d_n5, assign25610_e39020_d_n6, assign25610_e39020_d_n7, assign25610_e39020_d_n8, assign25610_e39020_d_n9, assign25610_e39020_d_n10, assign25610_e39020_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25610_e39017: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign25610_e39018: f64 = (1.0 / assign25610_e39017);
        (assign25610_e39018, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign25610_e39017 * assign25610_e39017))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign25610_e39017 * assign25610_e39017))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25610_e39020;
        locals.var_t1_dn3 = assign25610_e39020_d_n3;
        locals.var_t1_dn4 = assign25610_e39020_d_n4;
        locals.var_t1_dn5 = assign25610_e39020_d_n5;
        locals.var_t1_dn6 = assign25610_e39020_d_n6;
        locals.var_t1_dn7 = assign25610_e39020_d_n7;
        locals.var_t1_dn8 = assign25610_e39020_d_n8;
        locals.var_t1_dn9 = assign25610_e39020_d_n9;
        locals.var_t1_dn10 = assign25610_e39020_d_n10;
        locals.var_t1_dn11 = assign25610_e39020_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25620_e39037, assign25620_e39037_d_n3, assign25620_e39037_d_n4, assign25620_e39037_d_n5, assign25620_e39037_d_n6, assign25620_e39037_d_n7, assign25620_e39037_d_n8, assign25620_e39037_d_n9, assign25620_e39037_d_n10, assign25620_e39037_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25620_e39029: f64 = (-locals.var_vbd_jct);
        let assign25620_e39031: f64 = (assign25620_e39029 / locals.var_nvtmr);
        let assign25620_e39033: f64 = (assign25620_e39031 * locals.var_vrec0d_i);
        let assign25620_e39035: f64 = (assign25620_e39033 * locals.var_t1);
        (assign25620_e39035, (assign25620_e39033 * locals.var_t1_dn3), ((((-((assign25620_e39029 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn4)), ((((-((assign25620_e39029 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn6)), (assign25620_e39033 * locals.var_t1_dn7), (assign25620_e39033 * locals.var_t1_dn8), (assign25620_e39033 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign25620_e39033 * locals.var_t1_dn10)), (assign25620_e39033 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25620_e39037;
        locals.var_t0_dn3 = assign25620_e39037_d_n3;
        locals.var_t0_dn4 = assign25620_e39037_d_n4;
        locals.var_t0_dn5 = assign25620_e39037_d_n5;
        locals.var_t0_dn6 = assign25620_e39037_d_n6;
        locals.var_t0_dn7 = assign25620_e39037_d_n7;
        locals.var_t0_dn8 = assign25620_e39037_d_n8;
        locals.var_t0_dn9 = assign25620_e39037_d_n9;
        locals.var_t0_dn10 = assign25620_e39037_d_n10;
        locals.var_t0_dn11 = assign25620_e39037_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25630_e39048, assign25630_e39048_d_n3, assign25630_e39048_d_n4, assign25630_e39048_d_n5, assign25630_e39048_d_n6, assign25630_e39048_d_n7, assign25630_e39048_d_n8, assign25630_e39048_d_n9, assign25630_e39048_d_n10, assign25630_e39048_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25630_e39046: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25630_e39046, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25630_e39048;
        locals.var_t11_dn3 = assign25630_e39048_d_n3;
        locals.var_t11_dn4 = assign25630_e39048_d_n4;
        locals.var_t11_dn5 = assign25630_e39048_d_n5;
        locals.var_t11_dn6 = assign25630_e39048_d_n6;
        locals.var_t11_dn7 = assign25630_e39048_d_n7;
        locals.var_t11_dn8 = assign25630_e39048_d_n8;
        locals.var_t11_dn9 = assign25630_e39048_d_n9;
        locals.var_t11_dn10 = assign25630_e39048_d_n10;
        locals.var_t11_dn11 = assign25630_e39048_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25640_e39059, assign25640_e39059_d_n3, assign25640_e39059_d_n4, assign25640_e39059_d_n5, assign25640_e39059_d_n6, assign25640_e39059_d_n7, assign25640_e39059_d_n8, assign25640_e39059_d_n9, assign25640_e39059_d_n10, assign25640_e39059_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) && (locals.var_guard547 == 0.0)) {
        let assign25640_e39057: f64 = (-locals.var_t11);
        (assign25640_e39057, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign25640_e39059;
        locals.var_t11_dn3 = assign25640_e39059_d_n3;
        locals.var_t11_dn4 = assign25640_e39059_d_n4;
        locals.var_t11_dn5 = assign25640_e39059_d_n5;
        locals.var_t11_dn6 = assign25640_e39059_d_n6;
        locals.var_t11_dn7 = assign25640_e39059_d_n7;
        locals.var_t11_dn8 = assign25640_e39059_d_n8;
        locals.var_t11_dn9 = assign25640_e39059_d_n9;
        locals.var_t11_dn10 = assign25640_e39059_d_n10;
        locals.var_t11_dn11 = assign25640_e39059_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign25650_e39068, assign25650_e39068_d_n3, assign25650_e39068_d_n4, assign25650_e39068_d_n5, assign25650_e39068_d_n6, assign25650_e39068_d_n7, assign25650_e39068_d_n8, assign25650_e39068_d_n9, assign25650_e39068_d_n10, assign25650_e39068_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard546 == 0.0)) {
        let assign25650_e39066: f64 = (locals.var_wdtsi * locals.var_jrecd);
        (assign25650_e39066, (locals.var_wdtsi * locals.var_jrecd_dn3), (locals.var_wdtsi * locals.var_jrecd_dn4), (locals.var_wdtsi * locals.var_jrecd_dn5), (locals.var_wdtsi * locals.var_jrecd_dn6), (locals.var_wdtsi * locals.var_jrecd_dn7), (locals.var_wdtsi * locals.var_jrecd_dn8), (locals.var_wdtsi * locals.var_jrecd_dn9), (locals.var_wdtsi * locals.var_jrecd_dn10), (locals.var_wdtsi * locals.var_jrecd_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign25650_e39068;
        locals.var_t3_dn3 = assign25650_e39068_d_n3;
        locals.var_t3_dn4 = assign25650_e39068_d_n4;
        locals.var_t3_dn5 = assign25650_e39068_d_n5;
        locals.var_t3_dn6 = assign25650_e39068_d_n6;
        locals.var_t3_dn7 = assign25650_e39068_d_n7;
        locals.var_t3_dn8 = assign25650_e39068_d_n8;
        locals.var_t3_dn9 = assign25650_e39068_d_n9;
        locals.var_t3_dn10 = assign25650_e39068_d_n10;
        locals.var_t3_dn11 = assign25650_e39068_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign25670_e39087,) = {
    if (locals.var_guard492 != 0.0) {
        let assign25670_e39083: f64 = (locals.var_weff / p.p1373);
        let assign25670_e39085: f64 = (assign25670_e39083 * p.p74);
        (assign25670_e39085,)
    } else {
        (locals.var_wtsi,)
    }
};
        locals.var_wtsi = assign25670_e39087;
        locals.var_wtsi_rv = 0.0;

        let assign25680_e39094: f64 = if ((locals.var_isbjt_i == 0.0) && (locals.var_idbjt_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard548 = assign25680_e39094;
        locals.var_guard548_rv = 0.0;

        let (assign25720_e39123, assign25720_e39123_d_n3, assign25720_e39123_d_n4, assign25720_e39123_d_n5, assign25720_e39123_d_n6, assign25720_e39123_d_n7, assign25720_e39123_d_n8, assign25720_e39123_d_n9, assign25720_e39123_d_n10, assign25720_e39123_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25720_e39119: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign25720_e39121: f64 = (assign25720_e39119 / locals.var_ndiode_i);
        (assign25720_e39121, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25720_e39123;
        locals.var_t7_dn3 = assign25720_e39123_d_n3;
        locals.var_t7_dn4 = assign25720_e39123_d_n4;
        locals.var_t7_dn5 = assign25720_e39123_d_n5;
        locals.var_t7_dn6 = assign25720_e39123_d_n6;
        locals.var_t7_dn7 = assign25720_e39123_d_n7;
        locals.var_t7_dn8 = assign25720_e39123_d_n8;
        locals.var_t7_dn9 = assign25720_e39123_d_n9;
        locals.var_t7_dn10 = assign25720_e39123_d_n10;
        locals.var_t7_dn11 = assign25720_e39123_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign25730_e39131, assign25730_e39131_d_n3, assign25730_e39131_d_n4, assign25730_e39131_d_n5, assign25730_e39131_d_n6, assign25730_e39131_d_n7, assign25730_e39131_d_n8, assign25730_e39131_d_n9, assign25730_e39131_d_n10, assign25730_e39131_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25730_e39129: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25730_e39129, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25730_e39131;
        locals.var_t0_dn3 = assign25730_e39131_d_n3;
        locals.var_t0_dn4 = assign25730_e39131_d_n4;
        locals.var_t0_dn5 = assign25730_e39131_d_n5;
        locals.var_t0_dn6 = assign25730_e39131_d_n6;
        locals.var_t0_dn7 = assign25730_e39131_d_n7;
        locals.var_t0_dn8 = assign25730_e39131_d_n8;
        locals.var_t0_dn9 = assign25730_e39131_d_n9;
        locals.var_t0_dn10 = assign25730_e39131_d_n10;
        locals.var_t0_dn11 = assign25730_e39131_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25740_e39140, assign25740_e39140_d_n3, assign25740_e39140_d_n4, assign25740_e39140_d_n5, assign25740_e39140_d_n6, assign25740_e39140_d_n7, assign25740_e39140_d_n8, assign25740_e39140_d_n9, assign25740_e39140_d_n10, assign25740_e39140_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25740_e39138: f64 = (locals.var_ahli_i * locals.var_t0);
        (assign25740_e39138, (locals.var_ahli_i * locals.var_t0_dn3), (locals.var_ahli_i * locals.var_t0_dn4), (locals.var_ahli_i * locals.var_t0_dn5), (locals.var_ahli_i * locals.var_t0_dn6), (locals.var_ahli_i * locals.var_t0_dn7), (locals.var_ahli_i * locals.var_t0_dn8), (locals.var_ahli_i * locals.var_t0_dn9), (locals.var_ahli_i * locals.var_t0_dn10), (locals.var_ahli_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11,)
    }
};
        locals.var_ahlis = assign25740_e39140;
        locals.var_ahlis_dn3 = assign25740_e39140_d_n3;
        locals.var_ahlis_dn4 = assign25740_e39140_d_n4;
        locals.var_ahlis_dn5 = assign25740_e39140_d_n5;
        locals.var_ahlis_dn6 = assign25740_e39140_d_n6;
        locals.var_ahlis_dn7 = assign25740_e39140_d_n7;
        locals.var_ahlis_dn8 = assign25740_e39140_d_n8;
        locals.var_ahlis_dn9 = assign25740_e39140_d_n9;
        locals.var_ahlis_dn10 = assign25740_e39140_d_n10;
        locals.var_ahlis_dn11 = assign25740_e39140_d_n11;
        locals.var_ahlis_rv = 0.0;

        let (assign25750_e39149, assign25750_e39149_d_n3, assign25750_e39149_d_n4, assign25750_e39149_d_n5, assign25750_e39149_d_n6, assign25750_e39149_d_n7, assign25750_e39149_d_n8, assign25750_e39149_d_n9, assign25750_e39149_d_n10, assign25750_e39149_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25750_e39147: f64 = (locals.var_isbjt_i * locals.var_t0);
        (assign25750_e39147, (locals.var_isbjt_i * locals.var_t0_dn3), (locals.var_isbjt_i * locals.var_t0_dn4), (locals.var_isbjt_i * locals.var_t0_dn5), (locals.var_isbjt_i * locals.var_t0_dn6), (locals.var_isbjt_i * locals.var_t0_dn7), (locals.var_isbjt_i * locals.var_t0_dn8), (locals.var_isbjt_i * locals.var_t0_dn9), (locals.var_isbjt_i * locals.var_t0_dn10), (locals.var_isbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11,)
    }
};
        locals.var_jbjts = assign25750_e39149;
        locals.var_jbjts_dn3 = assign25750_e39149_d_n3;
        locals.var_jbjts_dn4 = assign25750_e39149_d_n4;
        locals.var_jbjts_dn5 = assign25750_e39149_d_n5;
        locals.var_jbjts_dn6 = assign25750_e39149_d_n6;
        locals.var_jbjts_dn7 = assign25750_e39149_d_n7;
        locals.var_jbjts_dn8 = assign25750_e39149_d_n8;
        locals.var_jbjts_dn9 = assign25750_e39149_d_n9;
        locals.var_jbjts_dn10 = assign25750_e39149_d_n10;
        locals.var_jbjts_dn11 = assign25750_e39149_d_n11;
        locals.var_jbjts_rv = 0.0;

        let (assign25760_e39160, assign25760_e39160_d_n3, assign25760_e39160_d_n4, assign25760_e39160_d_n5, assign25760_e39160_d_n6, assign25760_e39160_d_n7, assign25760_e39160_d_n8, assign25760_e39160_d_n9, assign25760_e39160_d_n10, assign25760_e39160_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25760_e39156: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign25760_e39158: f64 = (assign25760_e39156 / locals.var_ndiode_i);
        (assign25760_e39158, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign25760_e39160;
        locals.var_t7_dn3 = assign25760_e39160_d_n3;
        locals.var_t7_dn4 = assign25760_e39160_d_n4;
        locals.var_t7_dn5 = assign25760_e39160_d_n5;
        locals.var_t7_dn6 = assign25760_e39160_d_n6;
        locals.var_t7_dn7 = assign25760_e39160_d_n7;
        locals.var_t7_dn8 = assign25760_e39160_d_n8;
        locals.var_t7_dn9 = assign25760_e39160_d_n9;
        locals.var_t7_dn10 = assign25760_e39160_d_n10;
        locals.var_t7_dn11 = assign25760_e39160_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign25770_e39168, assign25770_e39168_d_n3, assign25770_e39168_d_n4, assign25770_e39168_d_n5, assign25770_e39168_d_n6, assign25770_e39168_d_n7, assign25770_e39168_d_n8, assign25770_e39168_d_n9, assign25770_e39168_d_n10, assign25770_e39168_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25770_e39166: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25770_e39166, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25770_e39168;
        locals.var_t0_dn3 = assign25770_e39168_d_n3;
        locals.var_t0_dn4 = assign25770_e39168_d_n4;
        locals.var_t0_dn5 = assign25770_e39168_d_n5;
        locals.var_t0_dn6 = assign25770_e39168_d_n6;
        locals.var_t0_dn7 = assign25770_e39168_d_n7;
        locals.var_t0_dn8 = assign25770_e39168_d_n8;
        locals.var_t0_dn9 = assign25770_e39168_d_n9;
        locals.var_t0_dn10 = assign25770_e39168_d_n10;
        locals.var_t0_dn11 = assign25770_e39168_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25780_e39177, assign25780_e39177_d_n3, assign25780_e39177_d_n4, assign25780_e39177_d_n5, assign25780_e39177_d_n6, assign25780_e39177_d_n7, assign25780_e39177_d_n8, assign25780_e39177_d_n9, assign25780_e39177_d_n10, assign25780_e39177_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25780_e39175: f64 = (locals.var_ahlid_i * locals.var_t0);
        (assign25780_e39175, (locals.var_ahlid_i * locals.var_t0_dn3), (locals.var_ahlid_i * locals.var_t0_dn4), (locals.var_ahlid_i * locals.var_t0_dn5), (locals.var_ahlid_i * locals.var_t0_dn6), (locals.var_ahlid_i * locals.var_t0_dn7), (locals.var_ahlid_i * locals.var_t0_dn8), (locals.var_ahlid_i * locals.var_t0_dn9), (locals.var_ahlid_i * locals.var_t0_dn10), (locals.var_ahlid_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11,)
    }
};
        locals.var_ahlid = assign25780_e39177;
        locals.var_ahlid_dn3 = assign25780_e39177_d_n3;
        locals.var_ahlid_dn4 = assign25780_e39177_d_n4;
        locals.var_ahlid_dn5 = assign25780_e39177_d_n5;
        locals.var_ahlid_dn6 = assign25780_e39177_d_n6;
        locals.var_ahlid_dn7 = assign25780_e39177_d_n7;
        locals.var_ahlid_dn8 = assign25780_e39177_d_n8;
        locals.var_ahlid_dn9 = assign25780_e39177_d_n9;
        locals.var_ahlid_dn10 = assign25780_e39177_d_n10;
        locals.var_ahlid_dn11 = assign25780_e39177_d_n11;
        locals.var_ahlid_rv = 0.0;

        let (assign25790_e39186, assign25790_e39186_d_n3, assign25790_e39186_d_n4, assign25790_e39186_d_n5, assign25790_e39186_d_n6, assign25790_e39186_d_n7, assign25790_e39186_d_n8, assign25790_e39186_d_n9, assign25790_e39186_d_n10, assign25790_e39186_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25790_e39184: f64 = (locals.var_idbjt_i * locals.var_t0);
        (assign25790_e39184, (locals.var_idbjt_i * locals.var_t0_dn3), (locals.var_idbjt_i * locals.var_t0_dn4), (locals.var_idbjt_i * locals.var_t0_dn5), (locals.var_idbjt_i * locals.var_t0_dn6), (locals.var_idbjt_i * locals.var_t0_dn7), (locals.var_idbjt_i * locals.var_t0_dn8), (locals.var_idbjt_i * locals.var_t0_dn9), (locals.var_idbjt_i * locals.var_t0_dn10), (locals.var_idbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11,)
    }
};
        locals.var_jbjtd = assign25790_e39186;
        locals.var_jbjtd_dn3 = assign25790_e39186_d_n3;
        locals.var_jbjtd_dn4 = assign25790_e39186_d_n4;
        locals.var_jbjtd_dn5 = assign25790_e39186_d_n5;
        locals.var_jbjtd_dn6 = assign25790_e39186_d_n6;
        locals.var_jbjtd_dn7 = assign25790_e39186_d_n7;
        locals.var_jbjtd_dn8 = assign25790_e39186_d_n8;
        locals.var_jbjtd_dn9 = assign25790_e39186_d_n9;
        locals.var_jbjtd_dn10 = assign25790_e39186_d_n10;
        locals.var_jbjtd_dn11 = assign25790_e39186_d_n11;
        locals.var_jbjtd_rv = 0.0;

        let (assign25800_e39197, assign25800_e39197_d_n3, assign25800_e39197_d_n4, assign25800_e39197_d_n5, assign25800_e39197_d_n6, assign25800_e39197_d_n7, assign25800_e39197_d_n8, assign25800_e39197_d_n9, assign25800_e39197_d_n10, assign25800_e39197_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25800_e39194: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign25800_e39195: f64 = (locals.var_ahlis * assign25800_e39194);
        (assign25800_e39195, ((locals.var_ahlis_dn3 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn3)), ((locals.var_ahlis_dn4 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn4)), ((locals.var_ahlis_dn5 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn5)), ((locals.var_ahlis_dn6 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn6)), ((locals.var_ahlis_dn7 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn7)), ((locals.var_ahlis_dn8 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn8)), ((locals.var_ahlis_dn9 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn9)), ((locals.var_ahlis_dn10 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn10)), ((locals.var_ahlis_dn11 * assign25800_e39194) + (locals.var_ahlis * locals.var_expvbsnvtm_dn11)),)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign25800_e39197;
        locals.var_ehlis_dn3 = assign25800_e39197_d_n3;
        locals.var_ehlis_dn4 = assign25800_e39197_d_n4;
        locals.var_ehlis_dn5 = assign25800_e39197_d_n5;
        locals.var_ehlis_dn6 = assign25800_e39197_d_n6;
        locals.var_ehlis_dn7 = assign25800_e39197_d_n7;
        locals.var_ehlis_dn8 = assign25800_e39197_d_n8;
        locals.var_ehlis_dn9 = assign25800_e39197_d_n9;
        locals.var_ehlis_dn10 = assign25800_e39197_d_n10;
        locals.var_ehlis_dn11 = assign25800_e39197_d_n11;
        locals.var_ehlis_rv = 0.0;

        let assign25810_e39200: f64 = if locals.var_ehlis < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign25810_e39200;
        locals.var_guard549_rv = 0.0;

        let (assign25820_e39209, assign25820_e39209_d_n3, assign25820_e39209_d_n4, assign25820_e39209_d_n5, assign25820_e39209_d_n6, assign25820_e39209_d_n7, assign25820_e39209_d_n8, assign25820_e39209_d_n9, assign25820_e39209_d_n10, assign25820_e39209_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard549 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign25820_e39209;
        locals.var_ehlis_dn3 = assign25820_e39209_d_n3;
        locals.var_ehlis_dn4 = assign25820_e39209_d_n4;
        locals.var_ehlis_dn5 = assign25820_e39209_d_n5;
        locals.var_ehlis_dn6 = assign25820_e39209_d_n6;
        locals.var_ehlis_dn7 = assign25820_e39209_d_n7;
        locals.var_ehlis_dn8 = assign25820_e39209_d_n8;
        locals.var_ehlis_dn9 = assign25820_e39209_d_n9;
        locals.var_ehlis_dn10 = assign25820_e39209_d_n10;
        locals.var_ehlis_dn11 = assign25820_e39209_d_n11;
        locals.var_ehlis_rv = 0.0;

        let (assign25830_e39218, assign25830_e39218_d_n3, assign25830_e39218_d_n4, assign25830_e39218_d_n5, assign25830_e39218_d_n6, assign25830_e39218_d_n7, assign25830_e39218_d_n8, assign25830_e39218_d_n9, assign25830_e39218_d_n10, assign25830_e39218_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard549 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign25830_e39218;
        locals.var_ehlisfactor_dn3 = assign25830_e39218_d_n3;
        locals.var_ehlisfactor_dn4 = assign25830_e39218_d_n4;
        locals.var_ehlisfactor_dn5 = assign25830_e39218_d_n5;
        locals.var_ehlisfactor_dn6 = assign25830_e39218_d_n6;
        locals.var_ehlisfactor_dn7 = assign25830_e39218_d_n7;
        locals.var_ehlisfactor_dn8 = assign25830_e39218_d_n8;
        locals.var_ehlisfactor_dn9 = assign25830_e39218_d_n9;
        locals.var_ehlisfactor_dn10 = assign25830_e39218_d_n10;
        locals.var_ehlisfactor_dn11 = assign25830_e39218_d_n11;
        locals.var_ehlisfactor_rv = 0.0;

        let (assign25840_e39233, assign25840_e39233_d_n3, assign25840_e39233_d_n4, assign25840_e39233_d_n5, assign25840_e39233_d_n6, assign25840_e39233_d_n7, assign25840_e39233_d_n8, assign25840_e39233_d_n9, assign25840_e39233_d_n10, assign25840_e39233_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard549 == 0.0)) {
        let assign25840_e39229: f64 = (1.0 + locals.var_ehlis);
        let assign25840_e39230: f64 = (assign25840_e39229).sqrt();
        let assign25840_e39231: f64 = (1.0 / assign25840_e39230);
        (assign25840_e39231, (-((locals.var_ehlis_dn3 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn4 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn5 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn6 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn7 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn8 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn9 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn10 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))), (-((locals.var_ehlis_dn11 / (2.0 * assign25840_e39230)) / (assign25840_e39230 * assign25840_e39230))),)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign25840_e39233;
        locals.var_ehlisfactor_dn3 = assign25840_e39233_d_n3;
        locals.var_ehlisfactor_dn4 = assign25840_e39233_d_n4;
        locals.var_ehlisfactor_dn5 = assign25840_e39233_d_n5;
        locals.var_ehlisfactor_dn6 = assign25840_e39233_d_n6;
        locals.var_ehlisfactor_dn7 = assign25840_e39233_d_n7;
        locals.var_ehlisfactor_dn8 = assign25840_e39233_d_n8;
        locals.var_ehlisfactor_dn9 = assign25840_e39233_d_n9;
        locals.var_ehlisfactor_dn10 = assign25840_e39233_d_n10;
        locals.var_ehlisfactor_dn11 = assign25840_e39233_d_n11;
        locals.var_ehlisfactor_rv = 0.0;

        let (assign25850_e39244, assign25850_e39244_d_n3, assign25850_e39244_d_n4, assign25850_e39244_d_n5, assign25850_e39244_d_n6, assign25850_e39244_d_n7, assign25850_e39244_d_n8, assign25850_e39244_d_n9, assign25850_e39244_d_n10, assign25850_e39244_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25850_e39241: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign25850_e39242: f64 = (locals.var_ahlid * assign25850_e39241);
        (assign25850_e39242, ((locals.var_ahlid_dn3 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn3)), ((locals.var_ahlid_dn4 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn4)), ((locals.var_ahlid_dn5 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn5)), ((locals.var_ahlid_dn6 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn6)), ((locals.var_ahlid_dn7 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn7)), ((locals.var_ahlid_dn8 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn8)), ((locals.var_ahlid_dn9 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn9)), ((locals.var_ahlid_dn10 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn10)), ((locals.var_ahlid_dn11 * assign25850_e39241) + (locals.var_ahlid * locals.var_expvbdnvtm_dn11)),)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign25850_e39244;
        locals.var_ehlid_dn3 = assign25850_e39244_d_n3;
        locals.var_ehlid_dn4 = assign25850_e39244_d_n4;
        locals.var_ehlid_dn5 = assign25850_e39244_d_n5;
        locals.var_ehlid_dn6 = assign25850_e39244_d_n6;
        locals.var_ehlid_dn7 = assign25850_e39244_d_n7;
        locals.var_ehlid_dn8 = assign25850_e39244_d_n8;
        locals.var_ehlid_dn9 = assign25850_e39244_d_n9;
        locals.var_ehlid_dn10 = assign25850_e39244_d_n10;
        locals.var_ehlid_dn11 = assign25850_e39244_d_n11;
        locals.var_ehlid_rv = 0.0;

        let assign25860_e39247: f64 = if locals.var_ehlid < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign25860_e39247;
        locals.var_guard550_rv = 0.0;

        let (assign25870_e39256, assign25870_e39256_d_n3, assign25870_e39256_d_n4, assign25870_e39256_d_n5, assign25870_e39256_d_n6, assign25870_e39256_d_n7, assign25870_e39256_d_n8, assign25870_e39256_d_n9, assign25870_e39256_d_n10, assign25870_e39256_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard550 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign25870_e39256;
        locals.var_ehlid_dn3 = assign25870_e39256_d_n3;
        locals.var_ehlid_dn4 = assign25870_e39256_d_n4;
        locals.var_ehlid_dn5 = assign25870_e39256_d_n5;
        locals.var_ehlid_dn6 = assign25870_e39256_d_n6;
        locals.var_ehlid_dn7 = assign25870_e39256_d_n7;
        locals.var_ehlid_dn8 = assign25870_e39256_d_n8;
        locals.var_ehlid_dn9 = assign25870_e39256_d_n9;
        locals.var_ehlid_dn10 = assign25870_e39256_d_n10;
        locals.var_ehlid_dn11 = assign25870_e39256_d_n11;
        locals.var_ehlid_rv = 0.0;

        let (assign25880_e39265, assign25880_e39265_d_n3, assign25880_e39265_d_n4, assign25880_e39265_d_n5, assign25880_e39265_d_n6, assign25880_e39265_d_n7, assign25880_e39265_d_n8, assign25880_e39265_d_n9, assign25880_e39265_d_n10, assign25880_e39265_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard550 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign25880_e39265;
        locals.var_ehlidfactor_dn3 = assign25880_e39265_d_n3;
        locals.var_ehlidfactor_dn4 = assign25880_e39265_d_n4;
        locals.var_ehlidfactor_dn5 = assign25880_e39265_d_n5;
        locals.var_ehlidfactor_dn6 = assign25880_e39265_d_n6;
        locals.var_ehlidfactor_dn7 = assign25880_e39265_d_n7;
        locals.var_ehlidfactor_dn8 = assign25880_e39265_d_n8;
        locals.var_ehlidfactor_dn9 = assign25880_e39265_d_n9;
        locals.var_ehlidfactor_dn10 = assign25880_e39265_d_n10;
        locals.var_ehlidfactor_dn11 = assign25880_e39265_d_n11;
        locals.var_ehlidfactor_rv = 0.0;

        let (assign25890_e39280, assign25890_e39280_d_n3, assign25890_e39280_d_n4, assign25890_e39280_d_n5, assign25890_e39280_d_n6, assign25890_e39280_d_n7, assign25890_e39280_d_n8, assign25890_e39280_d_n9, assign25890_e39280_d_n10, assign25890_e39280_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard550 == 0.0)) {
        let assign25890_e39276: f64 = (1.0 + locals.var_ehlid);
        let assign25890_e39277: f64 = (assign25890_e39276).sqrt();
        let assign25890_e39278: f64 = (1.0 / assign25890_e39277);
        (assign25890_e39278, (-((locals.var_ehlid_dn3 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn4 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn5 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn6 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn7 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn8 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn9 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn10 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))), (-((locals.var_ehlid_dn11 / (2.0 * assign25890_e39277)) / (assign25890_e39277 * assign25890_e39277))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign25890_e39280;
        locals.var_ehlidfactor_dn3 = assign25890_e39280_d_n3;
        locals.var_ehlidfactor_dn4 = assign25890_e39280_d_n4;
        locals.var_ehlidfactor_dn5 = assign25890_e39280_d_n5;
        locals.var_ehlidfactor_dn6 = assign25890_e39280_d_n6;
        locals.var_ehlidfactor_dn7 = assign25890_e39280_d_n7;
        locals.var_ehlidfactor_dn8 = assign25890_e39280_d_n8;
        locals.var_ehlidfactor_dn9 = assign25890_e39280_d_n9;
        locals.var_ehlidfactor_dn10 = assign25890_e39280_d_n10;
        locals.var_ehlidfactor_dn11 = assign25890_e39280_d_n11;
        locals.var_ehlidfactor_rv = 0.0;

        let (assign25900_e39296, assign25900_e39296_d_n3, assign25900_e39296_d_n4, assign25900_e39296_d_n5, assign25900_e39296_d_n6, assign25900_e39296_d_n7, assign25900_e39296_d_n8, assign25900_e39296_d_n9, assign25900_e39296_d_n10, assign25900_e39296_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25900_e39286: f64 = (-0.5);
        let assign25900_e39288: f64 = (assign25900_e39286 * locals.var_leff);
        let assign25900_e39290: f64 = (assign25900_e39288 * locals.var_leff);
        let __rspice_inv_cse_0: f64 = 1.0 / p.p595;
        let assign25900_e39292: f64 = (assign25900_e39290 * __rspice_inv_cse_0);
        let assign25900_e39294: f64 = (assign25900_e39292 * __rspice_inv_cse_0);
        (assign25900_e39294, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25900_e39296;
        locals.var_t0_dn3 = assign25900_e39296_d_n3;
        locals.var_t0_dn4 = assign25900_e39296_d_n4;
        locals.var_t0_dn5 = assign25900_e39296_d_n5;
        locals.var_t0_dn6 = assign25900_e39296_d_n6;
        locals.var_t0_dn7 = assign25900_e39296_d_n7;
        locals.var_t0_dn8 = assign25900_e39296_d_n8;
        locals.var_t0_dn9 = assign25900_e39296_d_n9;
        locals.var_t0_dn10 = assign25900_e39296_d_n10;
        locals.var_t0_dn11 = assign25900_e39296_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25910_e39304, assign25910_e39304_d_n3, assign25910_e39304_d_n4, assign25910_e39304_d_n5, assign25910_e39304_d_n6, assign25910_e39304_d_n7, assign25910_e39304_d_n8, assign25910_e39304_d_n9, assign25910_e39304_d_n10, assign25910_e39304_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25910_e39302: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25910_e39302, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_alphabjt, locals.var_alphabjt_dn3, locals.var_alphabjt_dn4, locals.var_alphabjt_dn5, locals.var_alphabjt_dn6, locals.var_alphabjt_dn7, locals.var_alphabjt_dn8, locals.var_alphabjt_dn9, locals.var_alphabjt_dn10, locals.var_alphabjt_dn11,)
    }
};
        locals.var_alphabjt = assign25910_e39304;
        locals.var_alphabjt_dn3 = assign25910_e39304_d_n3;
        locals.var_alphabjt_dn4 = assign25910_e39304_d_n4;
        locals.var_alphabjt_dn5 = assign25910_e39304_d_n5;
        locals.var_alphabjt_dn6 = assign25910_e39304_d_n6;
        locals.var_alphabjt_dn7 = assign25910_e39304_d_n7;
        locals.var_alphabjt_dn8 = assign25910_e39304_d_n8;
        locals.var_alphabjt_dn9 = assign25910_e39304_d_n9;
        locals.var_alphabjt_dn10 = assign25910_e39304_d_n10;
        locals.var_alphabjt_dn11 = assign25910_e39304_d_n11;
        locals.var_alphabjt_rv = 0.0;

        let (assign25920_e39313, assign25920_e39313_d_n3, assign25920_e39313_d_n4, assign25920_e39313_d_n5, assign25920_e39313_d_n6, assign25920_e39313_d_n7, assign25920_e39313_d_n8, assign25920_e39313_d_n9, assign25920_e39313_d_n10, assign25920_e39313_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25920_e39311: f64 = (1.0 - locals.var_alphabjt);
        (assign25920_e39311, (-locals.var_alphabjt_dn3), (-locals.var_alphabjt_dn4), (-locals.var_alphabjt_dn5), (-locals.var_alphabjt_dn6), (-locals.var_alphabjt_dn7), (-locals.var_alphabjt_dn8), (-locals.var_alphabjt_dn9), (-locals.var_alphabjt_dn10), (-locals.var_alphabjt_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign25920_e39313;
        locals.var_t2_dn3 = assign25920_e39313_d_n3;
        locals.var_t2_dn4 = assign25920_e39313_d_n4;
        locals.var_t2_dn5 = assign25920_e39313_d_n5;
        locals.var_t2_dn6 = assign25920_e39313_d_n6;
        locals.var_t2_dn7 = assign25920_e39313_d_n7;
        locals.var_t2_dn8 = assign25920_e39313_d_n8;
        locals.var_t2_dn9 = assign25920_e39313_d_n9;
        locals.var_t2_dn10 = assign25920_e39313_d_n10;
        locals.var_t2_dn11 = assign25920_e39313_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign25930_e39328, assign25930_e39328_d_n3, assign25930_e39328_d_n4, assign25930_e39328_d_n5, assign25930_e39328_d_n6, assign25930_e39328_d_n7, assign25930_e39328_d_n8, assign25930_e39328_d_n9, assign25930_e39328_d_n10, assign25930_e39328_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25930_e39321: f64 = (1.0 / locals.var_leff);
        let assign25930_e39324: f64 = (1.0 / p.p595);
        let assign25930_e39325: f64 = (assign25930_e39321 + assign25930_e39324);
        let assign25930_e39326: f64 = (locals.var_lbjt0_i * assign25930_e39325);
        (assign25930_e39326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign25930_e39328;
        locals.var_t0_dn3 = assign25930_e39328_d_n3;
        locals.var_t0_dn4 = assign25930_e39328_d_n4;
        locals.var_t0_dn5 = assign25930_e39328_d_n5;
        locals.var_t0_dn6 = assign25930_e39328_d_n6;
        locals.var_t0_dn7 = assign25930_e39328_d_n7;
        locals.var_t0_dn8 = assign25930_e39328_d_n8;
        locals.var_t0_dn9 = assign25930_e39328_d_n9;
        locals.var_t0_dn10 = assign25930_e39328_d_n10;
        locals.var_t0_dn11 = assign25930_e39328_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign25940_e39337, assign25940_e39337_d_n3, assign25940_e39337_d_n4, assign25940_e39337_d_n5, assign25940_e39337_d_n6, assign25940_e39337_d_n7, assign25940_e39337_d_n8, assign25940_e39337_d_n9, assign25940_e39337_d_n10, assign25940_e39337_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25940_e39335: f64 = (locals.var_t0).powf(locals.var_nbjt_i);
        (assign25940_e39335, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn3)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn3 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn4)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn4 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn5)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn5 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn6)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn6 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn7)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn7 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn8)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn8 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn9)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn9 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn10)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn10 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn11)) } } else { (assign25940_e39335 * (locals.var_nbjt_i * (locals.var_t0_dn11 / locals.var_t0))) },)
    } else {
        (locals.var_lratio, locals.var_lratio_dn3, locals.var_lratio_dn4, locals.var_lratio_dn5, locals.var_lratio_dn6, locals.var_lratio_dn7, locals.var_lratio_dn8, locals.var_lratio_dn9, locals.var_lratio_dn10, locals.var_lratio_dn11,)
    }
};
        locals.var_lratio = assign25940_e39337;
        locals.var_lratio_dn3 = assign25940_e39337_d_n3;
        locals.var_lratio_dn4 = assign25940_e39337_d_n4;
        locals.var_lratio_dn5 = assign25940_e39337_d_n5;
        locals.var_lratio_dn6 = assign25940_e39337_d_n6;
        locals.var_lratio_dn7 = assign25940_e39337_d_n7;
        locals.var_lratio_dn8 = assign25940_e39337_d_n8;
        locals.var_lratio_dn9 = assign25940_e39337_d_n9;
        locals.var_lratio_dn10 = assign25940_e39337_d_n10;
        locals.var_lratio_dn11 = assign25940_e39337_d_n11;
        locals.var_lratio_rv = 0.0;

        let (assign25950_e39348, assign25950_e39348_d_n3, assign25950_e39348_d_n4, assign25950_e39348_d_n5, assign25950_e39348_d_n6, assign25950_e39348_d_n7, assign25950_e39348_d_n8, assign25950_e39348_d_n9, assign25950_e39348_d_n10, assign25950_e39348_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25950_e39344: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign25950_e39346: f64 = (assign25950_e39344 * locals.var_lratio);
        (assign25950_e39346, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratio) + (assign25950_e39344 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign25950_e39348;
        locals.var_ien_dn3 = assign25950_e39348_d_n3;
        locals.var_ien_dn4 = assign25950_e39348_d_n4;
        locals.var_ien_dn5 = assign25950_e39348_d_n5;
        locals.var_ien_dn6 = assign25950_e39348_d_n6;
        locals.var_ien_dn7 = assign25950_e39348_d_n7;
        locals.var_ien_dn8 = assign25950_e39348_d_n8;
        locals.var_ien_dn9 = assign25950_e39348_d_n9;
        locals.var_ien_dn10 = assign25950_e39348_d_n10;
        locals.var_ien_dn11 = assign25950_e39348_d_n11;
        locals.var_ien_rv = 0.0;

        let (assign25960_e39357, assign25960_e39357_d_n3, assign25960_e39357_d_n4, assign25960_e39357_d_n5, assign25960_e39357_d_n6, assign25960_e39357_d_n7, assign25960_e39357_d_n8, assign25960_e39357_d_n9, assign25960_e39357_d_n10, assign25960_e39357_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25960_e39355: f64 = (locals.var_t0 * locals.var_ien);
        (assign25960_e39355, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25960_e39357;
        locals.var_t1_dn3 = assign25960_e39357_d_n3;
        locals.var_t1_dn4 = assign25960_e39357_d_n4;
        locals.var_t1_dn5 = assign25960_e39357_d_n5;
        locals.var_t1_dn6 = assign25960_e39357_d_n6;
        locals.var_t1_dn7 = assign25960_e39357_d_n7;
        locals.var_t1_dn8 = assign25960_e39357_d_n8;
        locals.var_t1_dn9 = assign25960_e39357_d_n9;
        locals.var_t1_dn10 = assign25960_e39357_d_n10;
        locals.var_t1_dn11 = assign25960_e39357_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign25980_e39381, assign25980_e39381_d_n3, assign25980_e39381_d_n4, assign25980_e39381_d_n5, assign25980_e39381_d_n6, assign25980_e39381_d_n7, assign25980_e39381_d_n8, assign25980_e39381_d_n9, assign25980_e39381_d_n10, assign25980_e39381_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25980_e39377: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign25980_e39379: f64 = (assign25980_e39377 * locals.var_lratio);
        (assign25980_e39379, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratio) + (assign25980_e39377 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign25980_e39381;
        locals.var_ien_dn3 = assign25980_e39381_d_n3;
        locals.var_ien_dn4 = assign25980_e39381_d_n4;
        locals.var_ien_dn5 = assign25980_e39381_d_n5;
        locals.var_ien_dn6 = assign25980_e39381_d_n6;
        locals.var_ien_dn7 = assign25980_e39381_d_n7;
        locals.var_ien_dn8 = assign25980_e39381_d_n8;
        locals.var_ien_dn9 = assign25980_e39381_d_n9;
        locals.var_ien_dn10 = assign25980_e39381_d_n10;
        locals.var_ien_dn11 = assign25980_e39381_d_n11;
        locals.var_ien_rv = 0.0;

        let (assign25990_e39390, assign25990_e39390_d_n3, assign25990_e39390_d_n4, assign25990_e39390_d_n5, assign25990_e39390_d_n6, assign25990_e39390_d_n7, assign25990_e39390_d_n8, assign25990_e39390_d_n9, assign25990_e39390_d_n10, assign25990_e39390_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign25990_e39388: f64 = (locals.var_t0 * locals.var_ien);
        (assign25990_e39388, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign25990_e39390;
        locals.var_t1_dn3 = assign25990_e39390_d_n3;
        locals.var_t1_dn4 = assign25990_e39390_d_n4;
        locals.var_t1_dn5 = assign25990_e39390_d_n5;
        locals.var_t1_dn6 = assign25990_e39390_d_n6;
        locals.var_t1_dn7 = assign25990_e39390_d_n7;
        locals.var_t1_dn8 = assign25990_e39390_d_n8;
        locals.var_t1_dn9 = assign25990_e39390_d_n9;
        locals.var_t1_dn10 = assign25990_e39390_d_n10;
        locals.var_t1_dn11 = assign25990_e39390_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26010_e39416, assign26010_e39416_d_n3, assign26010_e39416_d_n4, assign26010_e39416_d_n5, assign26010_e39416_d_n6, assign26010_e39416_d_n7, assign26010_e39416_d_n8, assign26010_e39416_d_n9, assign26010_e39416_d_n10, assign26010_e39416_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26010_e39412: f64 = (locals.var_t0).powf(locals.var_ndif_i);
        let assign26010_e39413: f64 = (p.p920 * assign26010_e39412);
        let assign26010_e39414: f64 = (1.0 + assign26010_e39413);
        (assign26010_e39414, (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn3)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn3 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn4)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn4 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn5)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn5 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn6)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn6 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn7)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn7 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn8)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn8 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn9)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn9 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn10)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn10 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn11)) } } else { (assign26010_e39412 * (locals.var_ndif_i * (locals.var_t0_dn11 / locals.var_t0))) }),)
    } else {
        (locals.var_lratiodif, locals.var_lratiodif_dn3, locals.var_lratiodif_dn4, locals.var_lratiodif_dn5, locals.var_lratiodif_dn6, locals.var_lratiodif_dn7, locals.var_lratiodif_dn8, locals.var_lratiodif_dn9, locals.var_lratiodif_dn10, locals.var_lratiodif_dn11,)
    }
};
        locals.var_lratiodif = assign26010_e39416;
        locals.var_lratiodif_dn3 = assign26010_e39416_d_n3;
        locals.var_lratiodif_dn4 = assign26010_e39416_d_n4;
        locals.var_lratiodif_dn5 = assign26010_e39416_d_n5;
        locals.var_lratiodif_dn6 = assign26010_e39416_d_n6;
        locals.var_lratiodif_dn7 = assign26010_e39416_d_n7;
        locals.var_lratiodif_dn8 = assign26010_e39416_d_n8;
        locals.var_lratiodif_dn9 = assign26010_e39416_d_n9;
        locals.var_lratiodif_dn10 = assign26010_e39416_d_n10;
        locals.var_lratiodif_dn11 = assign26010_e39416_d_n11;
        locals.var_lratiodif_rv = 0.0;

        let (assign26020_e39427, assign26020_e39427_d_n3, assign26020_e39427_d_n4, assign26020_e39427_d_n5, assign26020_e39427_d_n6, assign26020_e39427_d_n7, assign26020_e39427_d_n8, assign26020_e39427_d_n9, assign26020_e39427_d_n10, assign26020_e39427_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26020_e39423: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign26020_e39425: f64 = (assign26020_e39423 * locals.var_lratiodif);
        (assign26020_e39425, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratiodif) + (assign26020_e39423 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign26020_e39427;
        locals.var_iendif_dn3 = assign26020_e39427_d_n3;
        locals.var_iendif_dn4 = assign26020_e39427_d_n4;
        locals.var_iendif_dn5 = assign26020_e39427_d_n5;
        locals.var_iendif_dn6 = assign26020_e39427_d_n6;
        locals.var_iendif_dn7 = assign26020_e39427_d_n7;
        locals.var_iendif_dn8 = assign26020_e39427_d_n8;
        locals.var_iendif_dn9 = assign26020_e39427_d_n9;
        locals.var_iendif_dn10 = assign26020_e39427_d_n10;
        locals.var_iendif_dn11 = assign26020_e39427_d_n11;
        locals.var_iendif_rv = 0.0;

        let (assign26030_e39440, assign26030_e39440_d_n3, assign26030_e39440_d_n4, assign26030_e39440_d_n5, assign26030_e39440_d_n6, assign26030_e39440_d_n7, assign26030_e39440_d_n8, assign26030_e39440_d_n9, assign26030_e39440_d_n10, assign26030_e39440_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26030_e39435: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign26030_e39436: f64 = (locals.var_iendif * assign26030_e39435);
        let assign26030_e39438: f64 = (assign26030_e39436 * locals.var_ehlisfactor);
        (assign26030_e39438, ((((locals.var_iendif_dn3 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign26030_e39435) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign26030_e39436 * locals.var_ehlisfactor_dn11)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11,)
    }
};
        locals.var_ibsdif = assign26030_e39440;
        locals.var_ibsdif_dn3 = assign26030_e39440_d_n3;
        locals.var_ibsdif_dn4 = assign26030_e39440_d_n4;
        locals.var_ibsdif_dn5 = assign26030_e39440_d_n5;
        locals.var_ibsdif_dn6 = assign26030_e39440_d_n6;
        locals.var_ibsdif_dn7 = assign26030_e39440_d_n7;
        locals.var_ibsdif_dn8 = assign26030_e39440_d_n8;
        locals.var_ibsdif_dn9 = assign26030_e39440_d_n9;
        locals.var_ibsdif_dn10 = assign26030_e39440_d_n10;
        locals.var_ibsdif_dn11 = assign26030_e39440_d_n11;
        locals.var_ibsdif_rv = 0.0;

        let (assign26040_e39451, assign26040_e39451_d_n3, assign26040_e39451_d_n4, assign26040_e39451_d_n5, assign26040_e39451_d_n6, assign26040_e39451_d_n7, assign26040_e39451_d_n8, assign26040_e39451_d_n9, assign26040_e39451_d_n10, assign26040_e39451_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26040_e39447: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign26040_e39449: f64 = (assign26040_e39447 * locals.var_lratiodif);
        (assign26040_e39449, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratiodif) + (assign26040_e39447 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign26040_e39451;
        locals.var_iendif_dn3 = assign26040_e39451_d_n3;
        locals.var_iendif_dn4 = assign26040_e39451_d_n4;
        locals.var_iendif_dn5 = assign26040_e39451_d_n5;
        locals.var_iendif_dn6 = assign26040_e39451_d_n6;
        locals.var_iendif_dn7 = assign26040_e39451_d_n7;
        locals.var_iendif_dn8 = assign26040_e39451_d_n8;
        locals.var_iendif_dn9 = assign26040_e39451_d_n9;
        locals.var_iendif_dn10 = assign26040_e39451_d_n10;
        locals.var_iendif_dn11 = assign26040_e39451_d_n11;
        locals.var_iendif_rv = 0.0;

        let (assign26050_e39464, assign26050_e39464_d_n3, assign26050_e39464_d_n4, assign26050_e39464_d_n5, assign26050_e39464_d_n6, assign26050_e39464_d_n7, assign26050_e39464_d_n8, assign26050_e39464_d_n9, assign26050_e39464_d_n10, assign26050_e39464_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26050_e39459: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign26050_e39460: f64 = (locals.var_iendif * assign26050_e39459);
        let assign26050_e39462: f64 = (assign26050_e39460 * locals.var_ehlidfactor);
        (assign26050_e39462, ((((locals.var_iendif_dn3 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign26050_e39459) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign26050_e39460 * locals.var_ehlidfactor_dn11)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11,)
    }
};
        locals.var_ibddif = assign26050_e39464;
        locals.var_ibddif_dn3 = assign26050_e39464_d_n3;
        locals.var_ibddif_dn4 = assign26050_e39464_d_n4;
        locals.var_ibddif_dn5 = assign26050_e39464_d_n5;
        locals.var_ibddif_dn6 = assign26050_e39464_d_n6;
        locals.var_ibddif_dn7 = assign26050_e39464_d_n7;
        locals.var_ibddif_dn8 = assign26050_e39464_d_n8;
        locals.var_ibddif_dn9 = assign26050_e39464_d_n9;
        locals.var_ibddif_dn10 = assign26050_e39464_d_n10;
        locals.var_ibddif_dn11 = assign26050_e39464_d_n11;
        locals.var_ibddif_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26060_e39475,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) {
        let assign26060_e39472: f64 = (locals.var_aely_i * locals.var_leff);
        let assign26060_e39473: f64 = (locals.var_vabjt_i + assign26060_e39472);
        (assign26060_e39473,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign26060_e39475;
        locals.var_vearly_rv = 0.0;

        let assign26070_e39478: f64 = if locals.var_vearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign26070_e39478;
        locals.var_guard551_rv = 0.0;

        let (assign26080_e39487,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard551 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign26080_e39487;
        locals.var_vearly_rv = 0.0;

        let assign26090_e39490: f64 = if p.p554 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign26090_e39490;
        locals.var_guard552_rv = 0.0;

        let (assign26110_e39515, assign26110_e39515_d_n3, assign26110_e39515_d_n4, assign26110_e39515_d_n5, assign26110_e39515_d_n6, assign26110_e39515_d_n7, assign26110_e39515_d_n8, assign26110_e39515_d_n9, assign26110_e39515_d_n10, assign26110_e39515_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26110_e39510: f64 = (locals.var_vbs_jct + locals.var_vbd_jct);
        let assign26110_e39512: f64 = (assign26110_e39510 / locals.var_vearly);
        let assign26110_e39513: f64 = (1.0 + assign26110_e39512);
        (assign26110_e39513, 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn6 / locals.var_vearly), (locals.var_vbs_jct_dn7 / locals.var_vearly), 0.0, 0.0, ((locals.var_vbs_jct_dn10 + locals.var_vbd_jct_dn10) / locals.var_vearly), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26110_e39515;
        locals.var_t0_dn3 = assign26110_e39515_d_n3;
        locals.var_t0_dn4 = assign26110_e39515_d_n4;
        locals.var_t0_dn5 = assign26110_e39515_d_n5;
        locals.var_t0_dn6 = assign26110_e39515_d_n6;
        locals.var_t0_dn7 = assign26110_e39515_d_n7;
        locals.var_t0_dn8 = assign26110_e39515_d_n8;
        locals.var_t0_dn9 = assign26110_e39515_d_n9;
        locals.var_t0_dn10 = assign26110_e39515_d_n10;
        locals.var_t0_dn11 = assign26110_e39515_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26120_e39527, assign26120_e39527_d_n3, assign26120_e39527_d_n4, assign26120_e39527_d_n5, assign26120_e39527_d_n6, assign26120_e39527_d_n7, assign26120_e39527_d_n8, assign26120_e39527_d_n9, assign26120_e39527_d_n10, assign26120_e39527_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26120_e39525: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign26120_e39525, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26120_e39527;
        locals.var_t1_dn3 = assign26120_e39527_d_n3;
        locals.var_t1_dn4 = assign26120_e39527_d_n4;
        locals.var_t1_dn5 = assign26120_e39527_d_n5;
        locals.var_t1_dn6 = assign26120_e39527_d_n6;
        locals.var_t1_dn7 = assign26120_e39527_d_n7;
        locals.var_t1_dn8 = assign26120_e39527_d_n8;
        locals.var_t1_dn9 = assign26120_e39527_d_n9;
        locals.var_t1_dn10 = assign26120_e39527_d_n10;
        locals.var_t1_dn11 = assign26120_e39527_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26130_e39544, assign26130_e39544_d_n3, assign26130_e39544_d_n4, assign26130_e39544_d_n5, assign26130_e39544_d_n6, assign26130_e39544_d_n7, assign26130_e39544_d_n8, assign26130_e39544_d_n9, assign26130_e39544_d_n10, assign26130_e39544_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26130_e39537: f64 = (locals.var_t0 * locals.var_t0);
        let assign26130_e39540: f64 = (4.0 * locals.var_t1);
        let assign26130_e39541: f64 = (assign26130_e39537 + assign26130_e39540);
        let assign26130_e39542: f64 = (assign26130_e39541).sqrt();
        (assign26130_e39542, ((((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + (4.0 * locals.var_t1_dn3)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + (4.0 * locals.var_t1_dn4)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + (4.0 * locals.var_t1_dn5)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + (4.0 * locals.var_t1_dn6)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + (4.0 * locals.var_t1_dn7)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + (4.0 * locals.var_t1_dn8)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + (4.0 * locals.var_t1_dn9)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + (4.0 * locals.var_t1_dn10)) / (2.0 * assign26130_e39542)), ((((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + (4.0 * locals.var_t1_dn11)) / (2.0 * assign26130_e39542)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26130_e39544;
        locals.var_t3_dn3 = assign26130_e39544_d_n3;
        locals.var_t3_dn4 = assign26130_e39544_d_n4;
        locals.var_t3_dn5 = assign26130_e39544_d_n5;
        locals.var_t3_dn6 = assign26130_e39544_d_n6;
        locals.var_t3_dn7 = assign26130_e39544_d_n7;
        locals.var_t3_dn8 = assign26130_e39544_d_n8;
        locals.var_t3_dn9 = assign26130_e39544_d_n9;
        locals.var_t3_dn10 = assign26130_e39544_d_n10;
        locals.var_t3_dn11 = assign26130_e39544_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26140_e39558, assign26140_e39558_d_n3, assign26140_e39558_d_n4, assign26140_e39558_d_n5, assign26140_e39558_d_n6, assign26140_e39558_d_n7, assign26140_e39558_d_n8, assign26140_e39558_d_n9, assign26140_e39558_d_n10, assign26140_e39558_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26140_e39554: f64 = (locals.var_t0 + locals.var_t3);
        let assign26140_e39556: f64 = (assign26140_e39554 / 2.0);
        (assign26140_e39556, ((locals.var_t0_dn3 + locals.var_t3_dn3) / 2.0), ((locals.var_t0_dn4 + locals.var_t3_dn4) / 2.0), ((locals.var_t0_dn5 + locals.var_t3_dn5) / 2.0), ((locals.var_t0_dn6 + locals.var_t3_dn6) / 2.0), ((locals.var_t0_dn7 + locals.var_t3_dn7) / 2.0), ((locals.var_t0_dn8 + locals.var_t3_dn8) / 2.0), ((locals.var_t0_dn9 + locals.var_t3_dn9) / 2.0), ((locals.var_t0_dn10 + locals.var_t3_dn10) / 2.0), ((locals.var_t0_dn11 + locals.var_t3_dn11) / 2.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26140_e39558;
        locals.var_t2_dn3 = assign26140_e39558_d_n3;
        locals.var_t2_dn4 = assign26140_e39558_d_n4;
        locals.var_t2_dn5 = assign26140_e39558_d_n5;
        locals.var_t2_dn6 = assign26140_e39558_d_n6;
        locals.var_t2_dn7 = assign26140_e39558_d_n7;
        locals.var_t2_dn8 = assign26140_e39558_d_n8;
        locals.var_t2_dn9 = assign26140_e39558_d_n9;
        locals.var_t2_dn10 = assign26140_e39558_d_n10;
        locals.var_t2_dn11 = assign26140_e39558_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign26180_e39600, assign26180_e39600_d_n3, assign26180_e39600_d_n4, assign26180_e39600_d_n5, assign26180_e39600_d_n6, assign26180_e39600_d_n7, assign26180_e39600_d_n8, assign26180_e39600_d_n9, assign26180_e39600_d_n10, assign26180_e39600_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard548 == 0.0)) && (locals.var_guard552 == 0.0)) {
        let assign26180_e39598: f64 = (locals.var_alphabjt * locals.var_ien);
        (assign26180_e39598, ((locals.var_alphabjt_dn3 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn3)), ((locals.var_alphabjt_dn4 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn4)), ((locals.var_alphabjt_dn5 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn5)), ((locals.var_alphabjt_dn6 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn6)), ((locals.var_alphabjt_dn7 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn7)), ((locals.var_alphabjt_dn8 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn8)), ((locals.var_alphabjt_dn9 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn9)), ((locals.var_alphabjt_dn10 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn10)), ((locals.var_alphabjt_dn11 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26180_e39600;
        locals.var_t0_dn3 = assign26180_e39600_d_n3;
        locals.var_t0_dn4 = assign26180_e39600_d_n4;
        locals.var_t0_dn5 = assign26180_e39600_d_n5;
        locals.var_t0_dn6 = assign26180_e39600_d_n6;
        locals.var_t0_dn7 = assign26180_e39600_d_n7;
        locals.var_t0_dn8 = assign26180_e39600_d_n8;
        locals.var_t0_dn9 = assign26180_e39600_d_n9;
        locals.var_t0_dn10 = assign26180_e39600_d_n10;
        locals.var_t0_dn11 = assign26180_e39600_d_n11;
        locals.var_t0_rv = 0.0;

        let assign26200_e39625: f64 = if ((locals.var_istun_i == 0.0) && (locals.var_idtun_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard554 = assign26200_e39625;
        locals.var_guard554_rv = 0.0;

        let (assign26230_e39648, assign26230_e39648_d_n3, assign26230_e39648_d_n4, assign26230_e39648_d_n5, assign26230_e39648_d_n6, assign26230_e39648_d_n7, assign26230_e39648_d_n8, assign26230_e39648_d_n9, assign26230_e39648_d_n10, assign26230_e39648_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26230_e39645: f64 = (locals.var_tratio - 1.0);
        let assign26230_e39646: f64 = (locals.var_xtun_i * assign26230_e39645);
        (assign26230_e39646, 0.0, (locals.var_xtun_i * locals.var_tratio_dn4), (locals.var_xtun_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign26230_e39648;
        locals.var_t7_dn3 = assign26230_e39648_d_n3;
        locals.var_t7_dn4 = assign26230_e39648_d_n4;
        locals.var_t7_dn5 = assign26230_e39648_d_n5;
        locals.var_t7_dn6 = assign26230_e39648_d_n6;
        locals.var_t7_dn7 = assign26230_e39648_d_n7;
        locals.var_t7_dn8 = assign26230_e39648_d_n8;
        locals.var_t7_dn9 = assign26230_e39648_d_n9;
        locals.var_t7_dn10 = assign26230_e39648_d_n10;
        locals.var_t7_dn11 = assign26230_e39648_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign26240_e39656, assign26240_e39656_d_n3, assign26240_e39656_d_n4, assign26240_e39656_d_n5, assign26240_e39656_d_n6, assign26240_e39656_d_n7, assign26240_e39656_d_n8, assign26240_e39656_d_n9, assign26240_e39656_d_n10, assign26240_e39656_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26240_e39654: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26240_e39654, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26240_e39656;
        locals.var_t0_dn3 = assign26240_e39656_d_n3;
        locals.var_t0_dn4 = assign26240_e39656_d_n4;
        locals.var_t0_dn5 = assign26240_e39656_d_n5;
        locals.var_t0_dn6 = assign26240_e39656_d_n6;
        locals.var_t0_dn7 = assign26240_e39656_d_n7;
        locals.var_t0_dn8 = assign26240_e39656_d_n8;
        locals.var_t0_dn9 = assign26240_e39656_d_n9;
        locals.var_t0_dn10 = assign26240_e39656_d_n10;
        locals.var_t0_dn11 = assign26240_e39656_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26250_e39665, assign26250_e39665_d_n3, assign26250_e39665_d_n4, assign26250_e39665_d_n5, assign26250_e39665_d_n6, assign26250_e39665_d_n7, assign26250_e39665_d_n8, assign26250_e39665_d_n9, assign26250_e39665_d_n10, assign26250_e39665_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26250_e39663: f64 = (locals.var_istun_i * locals.var_t0);
        (assign26250_e39663, (locals.var_istun_i * locals.var_t0_dn3), (locals.var_istun_i * locals.var_t0_dn4), (locals.var_istun_i * locals.var_t0_dn5), (locals.var_istun_i * locals.var_t0_dn6), (locals.var_istun_i * locals.var_t0_dn7), (locals.var_istun_i * locals.var_t0_dn8), (locals.var_istun_i * locals.var_t0_dn9), (locals.var_istun_i * locals.var_t0_dn10), (locals.var_istun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11,)
    }
};
        locals.var_jtuns = assign26250_e39665;
        locals.var_jtuns_dn3 = assign26250_e39665_d_n3;
        locals.var_jtuns_dn4 = assign26250_e39665_d_n4;
        locals.var_jtuns_dn5 = assign26250_e39665_d_n5;
        locals.var_jtuns_dn6 = assign26250_e39665_d_n6;
        locals.var_jtuns_dn7 = assign26250_e39665_d_n7;
        locals.var_jtuns_dn8 = assign26250_e39665_d_n8;
        locals.var_jtuns_dn9 = assign26250_e39665_d_n9;
        locals.var_jtuns_dn10 = assign26250_e39665_d_n10;
        locals.var_jtuns_dn11 = assign26250_e39665_d_n11;
        locals.var_jtuns_rv = 0.0;

        let (assign26260_e39676, assign26260_e39676_d_n3, assign26260_e39676_d_n4, assign26260_e39676_d_n5, assign26260_e39676_d_n6, assign26260_e39676_d_n7, assign26260_e39676_d_n8, assign26260_e39676_d_n9, assign26260_e39676_d_n10, assign26260_e39676_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26260_e39673: f64 = (locals.var_tratio - 1.0);
        let assign26260_e39674: f64 = (locals.var_xtund_i * assign26260_e39673);
        (assign26260_e39674, 0.0, (locals.var_xtund_i * locals.var_tratio_dn4), (locals.var_xtund_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign26260_e39676;
        locals.var_t7_dn3 = assign26260_e39676_d_n3;
        locals.var_t7_dn4 = assign26260_e39676_d_n4;
        locals.var_t7_dn5 = assign26260_e39676_d_n5;
        locals.var_t7_dn6 = assign26260_e39676_d_n6;
        locals.var_t7_dn7 = assign26260_e39676_d_n7;
        locals.var_t7_dn8 = assign26260_e39676_d_n8;
        locals.var_t7_dn9 = assign26260_e39676_d_n9;
        locals.var_t7_dn10 = assign26260_e39676_d_n10;
        locals.var_t7_dn11 = assign26260_e39676_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign26270_e39684, assign26270_e39684_d_n3, assign26270_e39684_d_n4, assign26270_e39684_d_n5, assign26270_e39684_d_n6, assign26270_e39684_d_n7, assign26270_e39684_d_n8, assign26270_e39684_d_n9, assign26270_e39684_d_n10, assign26270_e39684_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26270_e39682: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26270_e39682, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26270_e39684;
        locals.var_t0_dn3 = assign26270_e39684_d_n3;
        locals.var_t0_dn4 = assign26270_e39684_d_n4;
        locals.var_t0_dn5 = assign26270_e39684_d_n5;
        locals.var_t0_dn6 = assign26270_e39684_d_n6;
        locals.var_t0_dn7 = assign26270_e39684_d_n7;
        locals.var_t0_dn8 = assign26270_e39684_d_n8;
        locals.var_t0_dn9 = assign26270_e39684_d_n9;
        locals.var_t0_dn10 = assign26270_e39684_d_n10;
        locals.var_t0_dn11 = assign26270_e39684_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26280_e39693, assign26280_e39693_d_n3, assign26280_e39693_d_n4, assign26280_e39693_d_n5, assign26280_e39693_d_n6, assign26280_e39693_d_n7, assign26280_e39693_d_n8, assign26280_e39693_d_n9, assign26280_e39693_d_n10, assign26280_e39693_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26280_e39691: f64 = (locals.var_idtun_i * locals.var_t0);
        (assign26280_e39691, (locals.var_idtun_i * locals.var_t0_dn3), (locals.var_idtun_i * locals.var_t0_dn4), (locals.var_idtun_i * locals.var_t0_dn5), (locals.var_idtun_i * locals.var_t0_dn6), (locals.var_idtun_i * locals.var_t0_dn7), (locals.var_idtun_i * locals.var_t0_dn8), (locals.var_idtun_i * locals.var_t0_dn9), (locals.var_idtun_i * locals.var_t0_dn10), (locals.var_idtun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11,)
    }
};
        locals.var_jtund = assign26280_e39693;
        locals.var_jtund_dn3 = assign26280_e39693_d_n3;
        locals.var_jtund_dn4 = assign26280_e39693_d_n4;
        locals.var_jtund_dn5 = assign26280_e39693_d_n5;
        locals.var_jtund_dn6 = assign26280_e39693_d_n6;
        locals.var_jtund_dn7 = assign26280_e39693_d_n7;
        locals.var_jtund_dn8 = assign26280_e39693_d_n8;
        locals.var_jtund_dn9 = assign26280_e39693_d_n9;
        locals.var_jtund_dn10 = assign26280_e39693_d_n10;
        locals.var_jtund_dn11 = assign26280_e39693_d_n11;
        locals.var_jtund_rv = 0.0;

        let (assign26290_e39702, assign26290_e39702_d_n4, assign26290_e39702_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26290_e39700: f64 = (p.p925 * locals.var_ntun_i);
        (assign26290_e39700, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign26290_e39702;
        locals.var_nvtm2_dn4 = assign26290_e39702_d_n4;
        locals.var_nvtm2_dn5 = assign26290_e39702_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let assign26300_e39705: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign26300_e39707: f64 = if assign26300_e39705 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign26300_e39707;
        locals.var_guard555_rv = 0.0;

        let (assign26310_e39716, assign26310_e39716_d_n3, assign26310_e39716_d_n4, assign26310_e39716_d_n5, assign26310_e39716_d_n6, assign26310_e39716_d_n7, assign26310_e39716_d_n8, assign26310_e39716_d_n9, assign26310_e39716_d_n10, assign26310_e39716_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26310_e39716;
        locals.var_t1_dn3 = assign26310_e39716_d_n3;
        locals.var_t1_dn4 = assign26310_e39716_d_n4;
        locals.var_t1_dn5 = assign26310_e39716_d_n5;
        locals.var_t1_dn6 = assign26310_e39716_d_n6;
        locals.var_t1_dn7 = assign26310_e39716_d_n7;
        locals.var_t1_dn8 = assign26310_e39716_d_n8;
        locals.var_t1_dn9 = assign26310_e39716_d_n9;
        locals.var_t1_dn10 = assign26310_e39716_d_n10;
        locals.var_t1_dn11 = assign26310_e39716_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26320_e39732, assign26320_e39732_d_n3, assign26320_e39732_d_n4, assign26320_e39732_d_n5, assign26320_e39732_d_n6, assign26320_e39732_d_n7, assign26320_e39732_d_n8, assign26320_e39732_d_n9, assign26320_e39732_d_n10, assign26320_e39732_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        let assign26320_e39724: f64 = (-locals.var_vbs_jct);
        let assign26320_e39726: f64 = (assign26320_e39724 / locals.var_nvtm2);
        let assign26320_e39728: f64 = (assign26320_e39726 * locals.var_vtun0_i);
        let assign26320_e39730: f64 = (assign26320_e39728 * locals.var_t1);
        (assign26320_e39730, (assign26320_e39728 * locals.var_t1_dn3), ((((-((assign26320_e39724 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn4)), ((((-((assign26320_e39724 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn5)), (assign26320_e39728 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn7)), (assign26320_e39728 * locals.var_t1_dn8), (assign26320_e39728 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26320_e39728 * locals.var_t1_dn10)), (assign26320_e39728 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26320_e39732;
        locals.var_t0_dn3 = assign26320_e39732_d_n3;
        locals.var_t0_dn4 = assign26320_e39732_d_n4;
        locals.var_t0_dn5 = assign26320_e39732_d_n5;
        locals.var_t0_dn6 = assign26320_e39732_d_n6;
        locals.var_t0_dn7 = assign26320_e39732_d_n7;
        locals.var_t0_dn8 = assign26320_e39732_d_n8;
        locals.var_t0_dn9 = assign26320_e39732_d_n9;
        locals.var_t0_dn10 = assign26320_e39732_d_n10;
        locals.var_t0_dn11 = assign26320_e39732_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26330_e39742, assign26330_e39742_d_n3, assign26330_e39742_d_n4, assign26330_e39742_d_n5, assign26330_e39742_d_n6, assign26330_e39742_d_n7, assign26330_e39742_d_n8, assign26330_e39742_d_n9, assign26330_e39742_d_n10, assign26330_e39742_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        let assign26330_e39740: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26330_e39740, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26330_e39742;
        locals.var_t1_dn3 = assign26330_e39742_d_n3;
        locals.var_t1_dn4 = assign26330_e39742_d_n4;
        locals.var_t1_dn5 = assign26330_e39742_d_n5;
        locals.var_t1_dn6 = assign26330_e39742_d_n6;
        locals.var_t1_dn7 = assign26330_e39742_d_n7;
        locals.var_t1_dn8 = assign26330_e39742_d_n8;
        locals.var_t1_dn9 = assign26330_e39742_d_n9;
        locals.var_t1_dn10 = assign26330_e39742_d_n10;
        locals.var_t1_dn11 = assign26330_e39742_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26340_e39753, assign26340_e39753_d_n3, assign26340_e39753_d_n4, assign26340_e39753_d_n5, assign26340_e39753_d_n6, assign26340_e39753_d_n7, assign26340_e39753_d_n8, assign26340_e39753_d_n9, assign26340_e39753_d_n10, assign26340_e39753_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 != 0.0)) {
        let assign26340_e39751: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign26340_e39751, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26340_e39753;
        locals.var_t3_dn3 = assign26340_e39753_d_n3;
        locals.var_t3_dn4 = assign26340_e39753_d_n4;
        locals.var_t3_dn5 = assign26340_e39753_d_n5;
        locals.var_t3_dn6 = assign26340_e39753_d_n6;
        locals.var_t3_dn7 = assign26340_e39753_d_n7;
        locals.var_t3_dn8 = assign26340_e39753_d_n8;
        locals.var_t3_dn9 = assign26340_e39753_d_n9;
        locals.var_t3_dn10 = assign26340_e39753_d_n10;
        locals.var_t3_dn11 = assign26340_e39753_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26360_e39780, assign26360_e39780_d_n3, assign26360_e39780_d_n4, assign26360_e39780_d_n5, assign26360_e39780_d_n6, assign26360_e39780_d_n7, assign26360_e39780_d_n8, assign26360_e39780_d_n9, assign26360_e39780_d_n10, assign26360_e39780_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26360_e39777: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign26360_e39778: f64 = (1.0 / assign26360_e39777);
        (assign26360_e39778, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign26360_e39777 * assign26360_e39777))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign26360_e39777 * assign26360_e39777))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26360_e39780;
        locals.var_t1_dn3 = assign26360_e39780_d_n3;
        locals.var_t1_dn4 = assign26360_e39780_d_n4;
        locals.var_t1_dn5 = assign26360_e39780_d_n5;
        locals.var_t1_dn6 = assign26360_e39780_d_n6;
        locals.var_t1_dn7 = assign26360_e39780_d_n7;
        locals.var_t1_dn8 = assign26360_e39780_d_n8;
        locals.var_t1_dn9 = assign26360_e39780_d_n9;
        locals.var_t1_dn10 = assign26360_e39780_d_n10;
        locals.var_t1_dn11 = assign26360_e39780_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26370_e39797, assign26370_e39797_d_n3, assign26370_e39797_d_n4, assign26370_e39797_d_n5, assign26370_e39797_d_n6, assign26370_e39797_d_n7, assign26370_e39797_d_n8, assign26370_e39797_d_n9, assign26370_e39797_d_n10, assign26370_e39797_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26370_e39789: f64 = (-locals.var_vbs_jct);
        let assign26370_e39791: f64 = (assign26370_e39789 / locals.var_nvtm2);
        let assign26370_e39793: f64 = (assign26370_e39791 * locals.var_vtun0_i);
        let assign26370_e39795: f64 = (assign26370_e39793 * locals.var_t1);
        (assign26370_e39795, (assign26370_e39793 * locals.var_t1_dn3), ((((-((assign26370_e39789 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn4)), ((((-((assign26370_e39789 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn5)), (assign26370_e39793 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn7)), (assign26370_e39793 * locals.var_t1_dn8), (assign26370_e39793 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign26370_e39793 * locals.var_t1_dn10)), (assign26370_e39793 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26370_e39797;
        locals.var_t0_dn3 = assign26370_e39797_d_n3;
        locals.var_t0_dn4 = assign26370_e39797_d_n4;
        locals.var_t0_dn5 = assign26370_e39797_d_n5;
        locals.var_t0_dn6 = assign26370_e39797_d_n6;
        locals.var_t0_dn7 = assign26370_e39797_d_n7;
        locals.var_t0_dn8 = assign26370_e39797_d_n8;
        locals.var_t0_dn9 = assign26370_e39797_d_n9;
        locals.var_t0_dn10 = assign26370_e39797_d_n10;
        locals.var_t0_dn11 = assign26370_e39797_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26380_e39808, assign26380_e39808_d_n3, assign26380_e39808_d_n4, assign26380_e39808_d_n5, assign26380_e39808_d_n6, assign26380_e39808_d_n7, assign26380_e39808_d_n8, assign26380_e39808_d_n9, assign26380_e39808_d_n10, assign26380_e39808_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26380_e39806: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26380_e39806, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26380_e39808;
        locals.var_t1_dn3 = assign26380_e39808_d_n3;
        locals.var_t1_dn4 = assign26380_e39808_d_n4;
        locals.var_t1_dn5 = assign26380_e39808_d_n5;
        locals.var_t1_dn6 = assign26380_e39808_d_n6;
        locals.var_t1_dn7 = assign26380_e39808_d_n7;
        locals.var_t1_dn8 = assign26380_e39808_d_n8;
        locals.var_t1_dn9 = assign26380_e39808_d_n9;
        locals.var_t1_dn10 = assign26380_e39808_d_n10;
        locals.var_t1_dn11 = assign26380_e39808_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26390_e39820, assign26390_e39820_d_n3, assign26390_e39820_d_n4, assign26390_e39820_d_n5, assign26390_e39820_d_n6, assign26390_e39820_d_n7, assign26390_e39820_d_n8, assign26390_e39820_d_n9, assign26390_e39820_d_n10, assign26390_e39820_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard555 == 0.0)) {
        let assign26390_e39818: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign26390_e39818, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26390_e39820;
        locals.var_t3_dn3 = assign26390_e39820_d_n3;
        locals.var_t3_dn4 = assign26390_e39820_d_n4;
        locals.var_t3_dn5 = assign26390_e39820_d_n5;
        locals.var_t3_dn6 = assign26390_e39820_d_n6;
        locals.var_t3_dn7 = assign26390_e39820_d_n7;
        locals.var_t3_dn8 = assign26390_e39820_d_n8;
        locals.var_t3_dn9 = assign26390_e39820_d_n9;
        locals.var_t3_dn10 = assign26390_e39820_d_n10;
        locals.var_t3_dn11 = assign26390_e39820_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26410_e39843, assign26410_e39843_d_n4, assign26410_e39843_d_n5,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) {
        let assign26410_e39841: f64 = (p.p925 * locals.var_ntund_i);
        (assign26410_e39841, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign26410_e39843;
        locals.var_nvtm2_dn4 = assign26410_e39843_d_n4;
        locals.var_nvtm2_dn5 = assign26410_e39843_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let assign26420_e39846: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign26420_e39848: f64 = if assign26420_e39846 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign26420_e39848;
        locals.var_guard556_rv = 0.0;

        let (assign26430_e39857, assign26430_e39857_d_n3, assign26430_e39857_d_n4, assign26430_e39857_d_n5, assign26430_e39857_d_n6, assign26430_e39857_d_n7, assign26430_e39857_d_n8, assign26430_e39857_d_n9, assign26430_e39857_d_n10, assign26430_e39857_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26430_e39857;
        locals.var_t1_dn3 = assign26430_e39857_d_n3;
        locals.var_t1_dn4 = assign26430_e39857_d_n4;
        locals.var_t1_dn5 = assign26430_e39857_d_n5;
        locals.var_t1_dn6 = assign26430_e39857_d_n6;
        locals.var_t1_dn7 = assign26430_e39857_d_n7;
        locals.var_t1_dn8 = assign26430_e39857_d_n8;
        locals.var_t1_dn9 = assign26430_e39857_d_n9;
        locals.var_t1_dn10 = assign26430_e39857_d_n10;
        locals.var_t1_dn11 = assign26430_e39857_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26440_e39873, assign26440_e39873_d_n3, assign26440_e39873_d_n4, assign26440_e39873_d_n5, assign26440_e39873_d_n6, assign26440_e39873_d_n7, assign26440_e39873_d_n8, assign26440_e39873_d_n9, assign26440_e39873_d_n10, assign26440_e39873_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        let assign26440_e39865: f64 = (-locals.var_vbd_jct);
        let assign26440_e39867: f64 = (assign26440_e39865 / locals.var_nvtm2);
        let assign26440_e39869: f64 = (assign26440_e39867 * locals.var_vtun0d_i);
        let assign26440_e39871: f64 = (assign26440_e39869 * locals.var_t1);
        (assign26440_e39871, (assign26440_e39869 * locals.var_t1_dn3), ((((-((assign26440_e39865 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn4)), ((((-((assign26440_e39865 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn6)), (assign26440_e39869 * locals.var_t1_dn7), (assign26440_e39869 * locals.var_t1_dn8), (assign26440_e39869 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26440_e39869 * locals.var_t1_dn10)), (assign26440_e39869 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26440_e39873;
        locals.var_t0_dn3 = assign26440_e39873_d_n3;
        locals.var_t0_dn4 = assign26440_e39873_d_n4;
        locals.var_t0_dn5 = assign26440_e39873_d_n5;
        locals.var_t0_dn6 = assign26440_e39873_d_n6;
        locals.var_t0_dn7 = assign26440_e39873_d_n7;
        locals.var_t0_dn8 = assign26440_e39873_d_n8;
        locals.var_t0_dn9 = assign26440_e39873_d_n9;
        locals.var_t0_dn10 = assign26440_e39873_d_n10;
        locals.var_t0_dn11 = assign26440_e39873_d_n11;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26450_e39883, assign26450_e39883_d_n3, assign26450_e39883_d_n4, assign26450_e39883_d_n5, assign26450_e39883_d_n6, assign26450_e39883_d_n7, assign26450_e39883_d_n8, assign26450_e39883_d_n9, assign26450_e39883_d_n10, assign26450_e39883_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        let assign26450_e39881: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26450_e39881, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26450_e39883;
        locals.var_t1_dn3 = assign26450_e39883_d_n3;
        locals.var_t1_dn4 = assign26450_e39883_d_n4;
        locals.var_t1_dn5 = assign26450_e39883_d_n5;
        locals.var_t1_dn6 = assign26450_e39883_d_n6;
        locals.var_t1_dn7 = assign26450_e39883_d_n7;
        locals.var_t1_dn8 = assign26450_e39883_d_n8;
        locals.var_t1_dn9 = assign26450_e39883_d_n9;
        locals.var_t1_dn10 = assign26450_e39883_d_n10;
        locals.var_t1_dn11 = assign26450_e39883_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26460_e39894, assign26460_e39894_d_n3, assign26460_e39894_d_n4, assign26460_e39894_d_n5, assign26460_e39894_d_n6, assign26460_e39894_d_n7, assign26460_e39894_d_n8, assign26460_e39894_d_n9, assign26460_e39894_d_n10, assign26460_e39894_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 != 0.0)) {
        let assign26460_e39892: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign26460_e39892, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26460_e39894;
        locals.var_t3_dn3 = assign26460_e39894_d_n3;
        locals.var_t3_dn4 = assign26460_e39894_d_n4;
        locals.var_t3_dn5 = assign26460_e39894_d_n5;
        locals.var_t3_dn6 = assign26460_e39894_d_n6;
        locals.var_t3_dn7 = assign26460_e39894_d_n7;
        locals.var_t3_dn8 = assign26460_e39894_d_n8;
        locals.var_t3_dn9 = assign26460_e39894_d_n9;
        locals.var_t3_dn10 = assign26460_e39894_d_n10;
        locals.var_t3_dn11 = assign26460_e39894_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26480_e39921, assign26480_e39921_d_n3, assign26480_e39921_d_n4, assign26480_e39921_d_n5, assign26480_e39921_d_n6, assign26480_e39921_d_n7, assign26480_e39921_d_n8, assign26480_e39921_d_n9, assign26480_e39921_d_n10, assign26480_e39921_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26480_e39918: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign26480_e39919: f64 = (1.0 / assign26480_e39918);
        (assign26480_e39919, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign26480_e39918 * assign26480_e39918))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign26480_e39918 * assign26480_e39918))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26480_e39921;
        locals.var_t1_dn3 = assign26480_e39921_d_n3;
        locals.var_t1_dn4 = assign26480_e39921_d_n4;
        locals.var_t1_dn5 = assign26480_e39921_d_n5;
        locals.var_t1_dn6 = assign26480_e39921_d_n6;
        locals.var_t1_dn7 = assign26480_e39921_d_n7;
        locals.var_t1_dn8 = assign26480_e39921_d_n8;
        locals.var_t1_dn9 = assign26480_e39921_d_n9;
        locals.var_t1_dn10 = assign26480_e39921_d_n10;
        locals.var_t1_dn11 = assign26480_e39921_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26490_e39938, assign26490_e39938_d_n3, assign26490_e39938_d_n4, assign26490_e39938_d_n5, assign26490_e39938_d_n6, assign26490_e39938_d_n7, assign26490_e39938_d_n8, assign26490_e39938_d_n9, assign26490_e39938_d_n10, assign26490_e39938_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26490_e39930: f64 = (-locals.var_vbd_jct);
        let assign26490_e39932: f64 = (assign26490_e39930 / locals.var_nvtm2);
        let assign26490_e39934: f64 = (assign26490_e39932 * locals.var_vtun0d_i);
        let assign26490_e39936: f64 = (assign26490_e39934 * locals.var_t1);
        (assign26490_e39936, (assign26490_e39934 * locals.var_t1_dn3), ((((-((assign26490_e39930 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn4)), ((((-((assign26490_e39930 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn6)), (assign26490_e39934 * locals.var_t1_dn7), (assign26490_e39934 * locals.var_t1_dn8), (assign26490_e39934 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign26490_e39934 * locals.var_t1_dn10)), (assign26490_e39934 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26490_e39938;
        locals.var_t0_dn3 = assign26490_e39938_d_n3;
        locals.var_t0_dn4 = assign26490_e39938_d_n4;
        locals.var_t0_dn5 = assign26490_e39938_d_n5;
        locals.var_t0_dn6 = assign26490_e39938_d_n6;
        locals.var_t0_dn7 = assign26490_e39938_d_n7;
        locals.var_t0_dn8 = assign26490_e39938_d_n8;
        locals.var_t0_dn9 = assign26490_e39938_d_n9;
        locals.var_t0_dn10 = assign26490_e39938_d_n10;
        locals.var_t0_dn11 = assign26490_e39938_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26500_e39949, assign26500_e39949_d_n3, assign26500_e39949_d_n4, assign26500_e39949_d_n5, assign26500_e39949_d_n6, assign26500_e39949_d_n7, assign26500_e39949_d_n8, assign26500_e39949_d_n9, assign26500_e39949_d_n10, assign26500_e39949_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26500_e39947: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26500_e39947, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26500_e39949;
        locals.var_t1_dn3 = assign26500_e39949_d_n3;
        locals.var_t1_dn4 = assign26500_e39949_d_n4;
        locals.var_t1_dn5 = assign26500_e39949_d_n5;
        locals.var_t1_dn6 = assign26500_e39949_d_n6;
        locals.var_t1_dn7 = assign26500_e39949_d_n7;
        locals.var_t1_dn8 = assign26500_e39949_d_n8;
        locals.var_t1_dn9 = assign26500_e39949_d_n9;
        locals.var_t1_dn10 = assign26500_e39949_d_n10;
        locals.var_t1_dn11 = assign26500_e39949_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26510_e39961, assign26510_e39961_d_n3, assign26510_e39961_d_n4, assign26510_e39961_d_n5, assign26510_e39961_d_n6, assign26510_e39961_d_n7, assign26510_e39961_d_n8, assign26510_e39961_d_n9, assign26510_e39961_d_n10, assign26510_e39961_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard554 == 0.0)) && (locals.var_guard556 == 0.0)) {
        let assign26510_e39959: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign26510_e39959, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26510_e39961;
        locals.var_t3_dn3 = assign26510_e39961_d_n3;
        locals.var_t3_dn4 = assign26510_e39961_d_n4;
        locals.var_t3_dn5 = assign26510_e39961_d_n5;
        locals.var_t3_dn6 = assign26510_e39961_d_n6;
        locals.var_t3_dn7 = assign26510_e39961_d_n7;
        locals.var_t3_dn8 = assign26510_e39961_d_n8;
        locals.var_t3_dn9 = assign26510_e39961_d_n9;
        locals.var_t3_dn10 = assign26510_e39961_d_n10;
        locals.var_t3_dn11 = assign26510_e39961_d_n11;
        locals.var_t3_rv = 0.0;

        let assign26570_e40010: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign26570_e40010;
        locals.var_guard557_rv = 0.0;

        let (assign26580_e40018, assign26580_e40018_d_n3, assign26580_e40018_d_n4, assign26580_e40018_d_n5, assign26580_e40018_d_n6, assign26580_e40018_d_n7, assign26580_e40018_d_n8, assign26580_e40018_d_n9, assign26580_e40018_d_n10, assign26580_e40018_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) {
        let assign26580_e40016: f64 = (locals.var_epsratio * p.p76);
        (assign26580_e40016, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26580_e40018;
        locals.var_t0_dn3 = assign26580_e40018_d_n3;
        locals.var_t0_dn4 = assign26580_e40018_d_n4;
        locals.var_t0_dn5 = assign26580_e40018_d_n5;
        locals.var_t0_dn6 = assign26580_e40018_d_n6;
        locals.var_t0_dn7 = assign26580_e40018_d_n7;
        locals.var_t0_dn8 = assign26580_e40018_d_n8;
        locals.var_t0_dn9 = assign26580_e40018_d_n9;
        locals.var_t0_dn10 = assign26580_e40018_d_n10;
        locals.var_t0_dn11 = assign26580_e40018_d_n11;
        locals.var_t0_rv = 0.0;

        let assign26590_e40029: f64 = if (((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) || (locals.var_cgidl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard558 = assign26590_e40029;
        locals.var_guard558_rv = 0.0;

        let (assign26600_e40037, assign26600_e40037_d_n3, assign26600_e40037_d_n4, assign26600_e40037_d_n5, assign26600_e40037_d_n6, assign26600_e40037_d_n7, assign26600_e40037_d_n8, assign26600_e40037_d_n9, assign26600_e40037_d_n10, assign26600_e40037_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26600_e40037;
        locals.var_t6_dn3 = assign26600_e40037_d_n3;
        locals.var_t6_dn4 = assign26600_e40037_d_n4;
        locals.var_t6_dn5 = assign26600_e40037_d_n5;
        locals.var_t6_dn6 = assign26600_e40037_d_n6;
        locals.var_t6_dn7 = assign26600_e40037_d_n7;
        locals.var_t6_dn8 = assign26600_e40037_d_n8;
        locals.var_t6_dn9 = assign26600_e40037_d_n9;
        locals.var_t6_dn10 = assign26600_e40037_d_n10;
        locals.var_t6_dn11 = assign26600_e40037_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign26610_e40053, assign26610_e40053_d_n3, assign26610_e40053_d_n4, assign26610_e40053_d_n5, assign26610_e40053_d_n6, assign26610_e40053_d_n7, assign26610_e40053_d_n8, assign26610_e40053_d_n9, assign26610_e40053_d_n10, assign26610_e40053_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26610_e40045: f64 = (-locals.var_vgd_noswap);
        let assign26610_e40047: f64 = (assign26610_e40045 - locals.var_egidl_i);
        let assign26610_e40049: f64 = (assign26610_e40047 + locals.var_vfbsdr);
        let assign26610_e40051: f64 = (assign26610_e40049 / locals.var_t0);
        (assign26610_e40051, (-((assign26610_e40049 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn6) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn7) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn8) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign26610_e40049 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn10) * locals.var_t0) - (assign26610_e40049 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign26610_e40049 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26610_e40053;
        locals.var_t1_dn3 = assign26610_e40053_d_n3;
        locals.var_t1_dn4 = assign26610_e40053_d_n4;
        locals.var_t1_dn5 = assign26610_e40053_d_n5;
        locals.var_t1_dn6 = assign26610_e40053_d_n6;
        locals.var_t1_dn7 = assign26610_e40053_d_n7;
        locals.var_t1_dn8 = assign26610_e40053_d_n8;
        locals.var_t1_dn9 = assign26610_e40053_d_n9;
        locals.var_t1_dn10 = assign26610_e40053_d_n10;
        locals.var_t1_dn11 = assign26610_e40053_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26620_e40075, assign26620_e40075_d_n3, assign26620_e40075_d_n4, assign26620_e40075_d_n5, assign26620_e40075_d_n6, assign26620_e40075_d_n7, assign26620_e40075_d_n8, assign26620_e40075_d_n9, assign26620_e40075_d_n10, assign26620_e40075_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26620_e40064: f64 = (locals.var_t1 * locals.var_t1);
        let assign26620_e40067: f64 = (4.0 * 0.01);
        let assign26620_e40069: f64 = (assign26620_e40067 * 0.01);
        let assign26620_e40070: f64 = (assign26620_e40064 + assign26620_e40069);
        let assign26620_e40071: f64 = (assign26620_e40070).sqrt();
        let assign26620_e40072: f64 = (locals.var_t1 + assign26620_e40071);
        let assign26620_e40073: f64 = (0.5 * assign26620_e40072);
        (assign26620_e40073, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign26620_e40071)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign26620_e40071)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26620_e40075;
        locals.var_t1_dn3 = assign26620_e40075_d_n3;
        locals.var_t1_dn4 = assign26620_e40075_d_n4;
        locals.var_t1_dn5 = assign26620_e40075_d_n5;
        locals.var_t1_dn6 = assign26620_e40075_d_n6;
        locals.var_t1_dn7 = assign26620_e40075_d_n7;
        locals.var_t1_dn8 = assign26620_e40075_d_n8;
        locals.var_t1_dn9 = assign26620_e40075_d_n9;
        locals.var_t1_dn10 = assign26620_e40075_d_n10;
        locals.var_t1_dn11 = assign26620_e40075_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26630_e40088, assign26630_e40088_d_n3, assign26630_e40088_d_n4, assign26630_e40088_d_n5, assign26630_e40088_d_n6, assign26630_e40088_d_n7, assign26630_e40088_d_n8, assign26630_e40088_d_n9, assign26630_e40088_d_n10, assign26630_e40088_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26630_e40085: f64 = (locals.var_t1 + 0.001);
        let assign26630_e40086: f64 = (locals.var_bgidl_t / assign26630_e40085);
        (assign26630_e40086, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign26630_e40085 * assign26630_e40085))), (((locals.var_bgidl_t_dn4 * assign26630_e40085) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign26630_e40085 * assign26630_e40085)), (((locals.var_bgidl_t_dn5 * assign26630_e40085) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign26630_e40085 * assign26630_e40085)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign26630_e40085 * assign26630_e40085))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign26630_e40085 * assign26630_e40085))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26630_e40088;
        locals.var_t2_dn3 = assign26630_e40088_d_n3;
        locals.var_t2_dn4 = assign26630_e40088_d_n4;
        locals.var_t2_dn5 = assign26630_e40088_d_n5;
        locals.var_t2_dn6 = assign26630_e40088_d_n6;
        locals.var_t2_dn7 = assign26630_e40088_d_n7;
        locals.var_t2_dn8 = assign26630_e40088_d_n8;
        locals.var_t2_dn9 = assign26630_e40088_d_n9;
        locals.var_t2_dn10 = assign26630_e40088_d_n10;
        locals.var_t2_dn11 = assign26630_e40088_d_n11;
        locals.var_t2_rv = 0.0;

        let assign26640_e40091: f64 = if locals.var_cgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign26640_e40091;
        locals.var_guard559_rv = 0.0;

        let (assign26650_e40106, assign26650_e40106_d_n3, assign26650_e40106_d_n4, assign26650_e40106_d_n5, assign26650_e40106_d_n6, assign26650_e40106_d_n7, assign26650_e40106_d_n8, assign26650_e40106_d_n9, assign26650_e40106_d_n10, assign26650_e40106_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign26650_e40102: f64 = (locals.var_vdb_noswap * locals.var_vdb_noswap);
        let assign26650_e40104: f64 = (assign26650_e40102 * locals.var_vdb_noswap);
        (assign26650_e40104, 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn6 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn6)) * locals.var_vdb_noswap) + (assign26650_e40102 * locals.var_vdb_noswap_dn6)), ((((locals.var_vdb_noswap_dn7 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn7)) * locals.var_vdb_noswap) + (assign26650_e40102 * locals.var_vdb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vdb_noswap_dn10 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn10)) * locals.var_vdb_noswap) + (assign26650_e40102 * locals.var_vdb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26650_e40106;
        locals.var_t3_dn3 = assign26650_e40106_d_n3;
        locals.var_t3_dn4 = assign26650_e40106_d_n4;
        locals.var_t3_dn5 = assign26650_e40106_d_n5;
        locals.var_t3_dn6 = assign26650_e40106_d_n6;
        locals.var_t3_dn7 = assign26650_e40106_d_n7;
        locals.var_t3_dn8 = assign26650_e40106_d_n8;
        locals.var_t3_dn9 = assign26650_e40106_d_n9;
        locals.var_t3_dn10 = assign26650_e40106_d_n10;
        locals.var_t3_dn11 = assign26650_e40106_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26660_e40122, assign26660_e40122_d_n3, assign26660_e40122_d_n4, assign26660_e40122_d_n5, assign26660_e40122_d_n6, assign26660_e40122_d_n7, assign26660_e40122_d_n8, assign26660_e40122_d_n9, assign26660_e40122_d_n10, assign26660_e40122_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign26660_e40117: f64 = (locals.var_t3).abs();
        let assign26660_e40118: f64 = (locals.var_cgidl_i + assign26660_e40117);
        let assign26660_e40120: f64 = (assign26660_e40118 + 0.0001);
        (assign26660_e40120, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign26660_e40122;
        locals.var_t4_dn3 = assign26660_e40122_d_n3;
        locals.var_t4_dn4 = assign26660_e40122_d_n4;
        locals.var_t4_dn5 = assign26660_e40122_d_n5;
        locals.var_t4_dn6 = assign26660_e40122_d_n6;
        locals.var_t4_dn7 = assign26660_e40122_d_n7;
        locals.var_t4_dn8 = assign26660_e40122_d_n8;
        locals.var_t4_dn9 = assign26660_e40122_d_n9;
        locals.var_t4_dn10 = assign26660_e40122_d_n10;
        locals.var_t4_dn11 = assign26660_e40122_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign26670_e40154, assign26670_e40154_d_n3, assign26670_e40154_d_n4, assign26670_e40154_d_n5, assign26670_e40154_d_n6, assign26670_e40154_d_n7, assign26670_e40154_d_n8, assign26670_e40154_d_n9, assign26670_e40154_d_n10, assign26670_e40154_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign26670_e40134: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26670_e40137: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26670_e40140: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26670_e40141: f64 = (assign26670_e40137 * assign26670_e40140);
        let assign26670_e40144: f64 = (4.0 * 1e-6);
        let assign26670_e40146: f64 = (assign26670_e40144 * 1e-6);
        let assign26670_e40147: f64 = (assign26670_e40141 + assign26670_e40146);
        let assign26670_e40148: f64 = (assign26670_e40147).sqrt();
        let assign26670_e40149: f64 = (assign26670_e40134 + assign26670_e40148);
        let assign26670_e40150: f64 = (0.5 * assign26670_e40149);
        let assign26670_e40152: f64 = (assign26670_e40150 - 1e-6);
        (assign26670_e40152, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign26670_e40140) + (assign26670_e40137 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26670_e40148)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26670_e40154;
        locals.var_t5_dn3 = assign26670_e40154_d_n3;
        locals.var_t5_dn4 = assign26670_e40154_d_n4;
        locals.var_t5_dn5 = assign26670_e40154_d_n5;
        locals.var_t5_dn6 = assign26670_e40154_d_n6;
        locals.var_t5_dn7 = assign26670_e40154_d_n7;
        locals.var_t5_dn8 = assign26670_e40154_d_n8;
        locals.var_t5_dn9 = assign26670_e40154_d_n9;
        locals.var_t5_dn10 = assign26670_e40154_d_n10;
        locals.var_t5_dn11 = assign26670_e40154_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign26680_e40166, assign26680_e40166_d_n3, assign26680_e40166_d_n4, assign26680_e40166_d_n5, assign26680_e40166_d_n6, assign26680_e40166_d_n7, assign26680_e40166_d_n8, assign26680_e40166_d_n9, assign26680_e40166_d_n10, assign26680_e40166_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) && (locals.var_guard559 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26680_e40166;
        locals.var_t5_dn3 = assign26680_e40166_d_n3;
        locals.var_t5_dn4 = assign26680_e40166_d_n4;
        locals.var_t5_dn5 = assign26680_e40166_d_n5;
        locals.var_t5_dn6 = assign26680_e40166_d_n6;
        locals.var_t5_dn7 = assign26680_e40166_d_n7;
        locals.var_t5_dn8 = assign26680_e40166_d_n8;
        locals.var_t5_dn9 = assign26680_e40166_d_n9;
        locals.var_t5_dn10 = assign26680_e40166_d_n10;
        locals.var_t5_dn11 = assign26680_e40166_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign26690_e40185, assign26690_e40185_d_n3, assign26690_e40185_d_n4, assign26690_e40185_d_n5, assign26690_e40185_d_n6, assign26690_e40185_d_n7, assign26690_e40185_d_n8, assign26690_e40185_d_n9, assign26690_e40185_d_n10, assign26690_e40185_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard558 == 0.0)) {
        let assign26690_e40175: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign26690_e40177: f64 = (assign26690_e40175 * locals.var_t1);
        let assign26690_e40179: f64 = (-locals.var_t2);
        let assign26690_e40180: f64 = { let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26690_e40181: f64 = (assign26690_e40177 * assign26690_e40180);
        let assign26690_e40183: f64 = (assign26690_e40181 * locals.var_t5);
        (assign26690_e40183, (((((assign26690_e40175 * locals.var_t1_dn3) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn3)), (((((assign26690_e40175 * locals.var_t1_dn4) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn4)), (((((assign26690_e40175 * locals.var_t1_dn5) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn5)), (((((assign26690_e40175 * locals.var_t1_dn6) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn6)), (((((assign26690_e40175 * locals.var_t1_dn7) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn7)), (((((assign26690_e40175 * locals.var_t1_dn8) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn8)), (((((assign26690_e40175 * locals.var_t1_dn9) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn9)), (((((assign26690_e40175 * locals.var_t1_dn10) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn10)), (((((assign26690_e40175 * locals.var_t1_dn11) * assign26690_e40180) + (assign26690_e40177 * ({ let limited_exp_arg = assign26690_e40179; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign26690_e40181 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26690_e40185;
        locals.var_t6_dn3 = assign26690_e40185_d_n3;
        locals.var_t6_dn4 = assign26690_e40185_d_n4;
        locals.var_t6_dn5 = assign26690_e40185_d_n5;
        locals.var_t6_dn6 = assign26690_e40185_d_n6;
        locals.var_t6_dn7 = assign26690_e40185_d_n7;
        locals.var_t6_dn8 = assign26690_e40185_d_n8;
        locals.var_t6_dn9 = assign26690_e40185_d_n9;
        locals.var_t6_dn10 = assign26690_e40185_d_n10;
        locals.var_t6_dn11 = assign26690_e40185_d_n11;
        locals.var_t6_rv = 0.0;

        let assign26710_e40202: f64 = if (((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) || (locals.var_cgisl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign26710_e40202;
        locals.var_guard560_rv = 0.0;

        let (assign26720_e40210, assign26720_e40210_d_n3, assign26720_e40210_d_n4, assign26720_e40210_d_n5, assign26720_e40210_d_n6, assign26720_e40210_d_n7, assign26720_e40210_d_n8, assign26720_e40210_d_n9, assign26720_e40210_d_n10, assign26720_e40210_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26720_e40210;
        locals.var_t6_dn3 = assign26720_e40210_d_n3;
        locals.var_t6_dn4 = assign26720_e40210_d_n4;
        locals.var_t6_dn5 = assign26720_e40210_d_n5;
        locals.var_t6_dn6 = assign26720_e40210_d_n6;
        locals.var_t6_dn7 = assign26720_e40210_d_n7;
        locals.var_t6_dn8 = assign26720_e40210_d_n8;
        locals.var_t6_dn9 = assign26720_e40210_d_n9;
        locals.var_t6_dn10 = assign26720_e40210_d_n10;
        locals.var_t6_dn11 = assign26720_e40210_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign26730_e40226, assign26730_e40226_d_n3, assign26730_e40226_d_n4, assign26730_e40226_d_n5, assign26730_e40226_d_n6, assign26730_e40226_d_n7, assign26730_e40226_d_n8, assign26730_e40226_d_n9, assign26730_e40226_d_n10, assign26730_e40226_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26730_e40218: f64 = (-locals.var_vgs_noswap);
        let assign26730_e40220: f64 = (assign26730_e40218 - locals.var_egisl_i);
        let assign26730_e40222: f64 = (assign26730_e40220 + locals.var_vfbsdr);
        let assign26730_e40224: f64 = (assign26730_e40222 / locals.var_t0);
        (assign26730_e40224, (-((assign26730_e40222 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn6) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn7) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn8) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign26730_e40222 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn10) * locals.var_t0) - (assign26730_e40222 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign26730_e40222 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26730_e40226;
        locals.var_t1_dn3 = assign26730_e40226_d_n3;
        locals.var_t1_dn4 = assign26730_e40226_d_n4;
        locals.var_t1_dn5 = assign26730_e40226_d_n5;
        locals.var_t1_dn6 = assign26730_e40226_d_n6;
        locals.var_t1_dn7 = assign26730_e40226_d_n7;
        locals.var_t1_dn8 = assign26730_e40226_d_n8;
        locals.var_t1_dn9 = assign26730_e40226_d_n9;
        locals.var_t1_dn10 = assign26730_e40226_d_n10;
        locals.var_t1_dn11 = assign26730_e40226_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26740_e40248, assign26740_e40248_d_n3, assign26740_e40248_d_n4, assign26740_e40248_d_n5, assign26740_e40248_d_n6, assign26740_e40248_d_n7, assign26740_e40248_d_n8, assign26740_e40248_d_n9, assign26740_e40248_d_n10, assign26740_e40248_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26740_e40237: f64 = (locals.var_t1 * locals.var_t1);
        let assign26740_e40240: f64 = (4.0 * 0.01);
        let assign26740_e40242: f64 = (assign26740_e40240 * 0.01);
        let assign26740_e40243: f64 = (assign26740_e40237 + assign26740_e40242);
        let assign26740_e40244: f64 = (assign26740_e40243).sqrt();
        let assign26740_e40245: f64 = (locals.var_t1 + assign26740_e40244);
        let assign26740_e40246: f64 = (0.5 * assign26740_e40245);
        (assign26740_e40246, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign26740_e40244)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign26740_e40244)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26740_e40248;
        locals.var_t1_dn3 = assign26740_e40248_d_n3;
        locals.var_t1_dn4 = assign26740_e40248_d_n4;
        locals.var_t1_dn5 = assign26740_e40248_d_n5;
        locals.var_t1_dn6 = assign26740_e40248_d_n6;
        locals.var_t1_dn7 = assign26740_e40248_d_n7;
        locals.var_t1_dn8 = assign26740_e40248_d_n8;
        locals.var_t1_dn9 = assign26740_e40248_d_n9;
        locals.var_t1_dn10 = assign26740_e40248_d_n10;
        locals.var_t1_dn11 = assign26740_e40248_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26750_e40261, assign26750_e40261_d_n3, assign26750_e40261_d_n4, assign26750_e40261_d_n5, assign26750_e40261_d_n6, assign26750_e40261_d_n7, assign26750_e40261_d_n8, assign26750_e40261_d_n9, assign26750_e40261_d_n10, assign26750_e40261_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26750_e40258: f64 = (locals.var_t1 + 0.001);
        let assign26750_e40259: f64 = (locals.var_bgisl_t / assign26750_e40258);
        (assign26750_e40259, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign26750_e40258 * assign26750_e40258))), (((locals.var_bgisl_t_dn4 * assign26750_e40258) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign26750_e40258 * assign26750_e40258)), (((locals.var_bgisl_t_dn5 * assign26750_e40258) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign26750_e40258 * assign26750_e40258)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign26750_e40258 * assign26750_e40258))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign26750_e40258 * assign26750_e40258))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26750_e40261;
        locals.var_t2_dn3 = assign26750_e40261_d_n3;
        locals.var_t2_dn4 = assign26750_e40261_d_n4;
        locals.var_t2_dn5 = assign26750_e40261_d_n5;
        locals.var_t2_dn6 = assign26750_e40261_d_n6;
        locals.var_t2_dn7 = assign26750_e40261_d_n7;
        locals.var_t2_dn8 = assign26750_e40261_d_n8;
        locals.var_t2_dn9 = assign26750_e40261_d_n9;
        locals.var_t2_dn10 = assign26750_e40261_d_n10;
        locals.var_t2_dn11 = assign26750_e40261_d_n11;
        locals.var_t2_rv = 0.0;

        let assign26760_e40264: f64 = if locals.var_cgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign26760_e40264;
        locals.var_guard561_rv = 0.0;

        let (assign26770_e40279, assign26770_e40279_d_n3, assign26770_e40279_d_n4, assign26770_e40279_d_n5, assign26770_e40279_d_n6, assign26770_e40279_d_n7, assign26770_e40279_d_n8, assign26770_e40279_d_n9, assign26770_e40279_d_n10, assign26770_e40279_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign26770_e40275: f64 = (locals.var_vsb_noswap * locals.var_vsb_noswap);
        let assign26770_e40277: f64 = (assign26770_e40275 * locals.var_vsb_noswap);
        (assign26770_e40277, 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn6 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn6)) * locals.var_vsb_noswap) + (assign26770_e40275 * locals.var_vsb_noswap_dn6)), ((((locals.var_vsb_noswap_dn7 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn7)) * locals.var_vsb_noswap) + (assign26770_e40275 * locals.var_vsb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vsb_noswap_dn10 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn10)) * locals.var_vsb_noswap) + (assign26770_e40275 * locals.var_vsb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26770_e40279;
        locals.var_t3_dn3 = assign26770_e40279_d_n3;
        locals.var_t3_dn4 = assign26770_e40279_d_n4;
        locals.var_t3_dn5 = assign26770_e40279_d_n5;
        locals.var_t3_dn6 = assign26770_e40279_d_n6;
        locals.var_t3_dn7 = assign26770_e40279_d_n7;
        locals.var_t3_dn8 = assign26770_e40279_d_n8;
        locals.var_t3_dn9 = assign26770_e40279_d_n9;
        locals.var_t3_dn10 = assign26770_e40279_d_n10;
        locals.var_t3_dn11 = assign26770_e40279_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26780_e40295, assign26780_e40295_d_n3, assign26780_e40295_d_n4, assign26780_e40295_d_n5, assign26780_e40295_d_n6, assign26780_e40295_d_n7, assign26780_e40295_d_n8, assign26780_e40295_d_n9, assign26780_e40295_d_n10, assign26780_e40295_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign26780_e40290: f64 = (locals.var_t3).abs();
        let assign26780_e40291: f64 = (locals.var_cgisl_i + assign26780_e40290);
        let assign26780_e40293: f64 = (assign26780_e40291 + 0.0001);
        (assign26780_e40293, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign26780_e40295;
        locals.var_t4_dn3 = assign26780_e40295_d_n3;
        locals.var_t4_dn4 = assign26780_e40295_d_n4;
        locals.var_t4_dn5 = assign26780_e40295_d_n5;
        locals.var_t4_dn6 = assign26780_e40295_d_n6;
        locals.var_t4_dn7 = assign26780_e40295_d_n7;
        locals.var_t4_dn8 = assign26780_e40295_d_n8;
        locals.var_t4_dn9 = assign26780_e40295_d_n9;
        locals.var_t4_dn10 = assign26780_e40295_d_n10;
        locals.var_t4_dn11 = assign26780_e40295_d_n11;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26790_e40327, assign26790_e40327_d_n3, assign26790_e40327_d_n4, assign26790_e40327_d_n5, assign26790_e40327_d_n6, assign26790_e40327_d_n7, assign26790_e40327_d_n8, assign26790_e40327_d_n9, assign26790_e40327_d_n10, assign26790_e40327_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign26790_e40307: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26790_e40310: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26790_e40313: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign26790_e40314: f64 = (assign26790_e40310 * assign26790_e40313);
        let assign26790_e40317: f64 = (4.0 * 1e-6);
        let assign26790_e40319: f64 = (assign26790_e40317 * 1e-6);
        let assign26790_e40320: f64 = (assign26790_e40314 + assign26790_e40319);
        let assign26790_e40321: f64 = (assign26790_e40320).sqrt();
        let assign26790_e40322: f64 = (assign26790_e40307 + assign26790_e40321);
        let assign26790_e40323: f64 = (0.5 * assign26790_e40322);
        let assign26790_e40325: f64 = (assign26790_e40323 - 1e-6);
        (assign26790_e40325, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign26790_e40313) + (assign26790_e40310 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign26790_e40321)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26790_e40327;
        locals.var_t5_dn3 = assign26790_e40327_d_n3;
        locals.var_t5_dn4 = assign26790_e40327_d_n4;
        locals.var_t5_dn5 = assign26790_e40327_d_n5;
        locals.var_t5_dn6 = assign26790_e40327_d_n6;
        locals.var_t5_dn7 = assign26790_e40327_d_n7;
        locals.var_t5_dn8 = assign26790_e40327_d_n8;
        locals.var_t5_dn9 = assign26790_e40327_d_n9;
        locals.var_t5_dn10 = assign26790_e40327_d_n10;
        locals.var_t5_dn11 = assign26790_e40327_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign26800_e40339, assign26800_e40339_d_n3, assign26800_e40339_d_n4, assign26800_e40339_d_n5, assign26800_e40339_d_n6, assign26800_e40339_d_n7, assign26800_e40339_d_n8, assign26800_e40339_d_n9, assign26800_e40339_d_n10, assign26800_e40339_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) && (locals.var_guard561 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26800_e40339;
        locals.var_t5_dn3 = assign26800_e40339_d_n3;
        locals.var_t5_dn4 = assign26800_e40339_d_n4;
        locals.var_t5_dn5 = assign26800_e40339_d_n5;
        locals.var_t5_dn6 = assign26800_e40339_d_n6;
        locals.var_t5_dn7 = assign26800_e40339_d_n7;
        locals.var_t5_dn8 = assign26800_e40339_d_n8;
        locals.var_t5_dn9 = assign26800_e40339_d_n9;
        locals.var_t5_dn10 = assign26800_e40339_d_n10;
        locals.var_t5_dn11 = assign26800_e40339_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign26810_e40358, assign26810_e40358_d_n3, assign26810_e40358_d_n4, assign26810_e40358_d_n5, assign26810_e40358_d_n6, assign26810_e40358_d_n7, assign26810_e40358_d_n8, assign26810_e40358_d_n9, assign26810_e40358_d_n10, assign26810_e40358_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 != 0.0)) && (locals.var_guard560 == 0.0)) {
        let assign26810_e40348: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign26810_e40350: f64 = (assign26810_e40348 * locals.var_t1);
        let assign26810_e40352: f64 = (-locals.var_t2);
        let assign26810_e40353: f64 = { let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26810_e40354: f64 = (assign26810_e40350 * assign26810_e40353);
        let assign26810_e40356: f64 = (assign26810_e40354 * locals.var_t5);
        (assign26810_e40356, (((((assign26810_e40348 * locals.var_t1_dn3) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn3)), (((((assign26810_e40348 * locals.var_t1_dn4) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn4)), (((((assign26810_e40348 * locals.var_t1_dn5) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn5)), (((((assign26810_e40348 * locals.var_t1_dn6) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn6)), (((((assign26810_e40348 * locals.var_t1_dn7) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn7)), (((((assign26810_e40348 * locals.var_t1_dn8) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn8)), (((((assign26810_e40348 * locals.var_t1_dn9) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn9)), (((((assign26810_e40348 * locals.var_t1_dn10) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn10)), (((((assign26810_e40348 * locals.var_t1_dn11) * assign26810_e40353) + (assign26810_e40350 * ({ let limited_exp_arg = assign26810_e40352; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign26810_e40354 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26810_e40358;
        locals.var_t6_dn3 = assign26810_e40358_d_n3;
        locals.var_t6_dn4 = assign26810_e40358_d_n4;
        locals.var_t6_dn5 = assign26810_e40358_d_n5;
        locals.var_t6_dn6 = assign26810_e40358_d_n6;
        locals.var_t6_dn7 = assign26810_e40358_d_n7;
        locals.var_t6_dn8 = assign26810_e40358_d_n8;
        locals.var_t6_dn9 = assign26810_e40358_d_n9;
        locals.var_t6_dn10 = assign26810_e40358_d_n10;
        locals.var_t6_dn11 = assign26810_e40358_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign26830_e40373, assign26830_e40373_d_n3, assign26830_e40373_d_n4, assign26830_e40373_d_n5, assign26830_e40373_d_n6, assign26830_e40373_d_n7, assign26830_e40373_d_n8, assign26830_e40373_d_n9, assign26830_e40373_d_n10, assign26830_e40373_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26830_e40371: f64 = (locals.var_epsratio * p.p76);
        (assign26830_e40371, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign26830_e40373;
        locals.var_t0_dn3 = assign26830_e40373_d_n3;
        locals.var_t0_dn4 = assign26830_e40373_d_n4;
        locals.var_t0_dn5 = assign26830_e40373_d_n5;
        locals.var_t0_dn6 = assign26830_e40373_d_n6;
        locals.var_t0_dn7 = assign26830_e40373_d_n7;
        locals.var_t0_dn8 = assign26830_e40373_d_n8;
        locals.var_t0_dn9 = assign26830_e40373_d_n9;
        locals.var_t0_dn10 = assign26830_e40373_d_n10;
        locals.var_t0_dn11 = assign26830_e40373_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign26840_e40384, assign26840_e40384_d_n6, assign26840_e40384_d_n7, assign26840_e40384_d_n8, assign26840_e40384_d_n10,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26840_e40380: f64 = (locals.var_rgisl_i * locals.var_vg);
        let assign26840_e40382: f64 = (assign26840_e40380 - locals.var_vd);
        (assign26840_e40382, (-locals.var_vd_dn6), (-locals.var_vd_dn7), (locals.var_rgisl_i * locals.var_vg_dn8), ((locals.var_rgisl_i * locals.var_vg_dn10) - locals.var_vd_dn10),)
    } else {
        (locals.var_vgd_noswap_1, locals.var_vgd_noswap_1_dn6, locals.var_vgd_noswap_1_dn7, locals.var_vgd_noswap_1_dn8, locals.var_vgd_noswap_1_dn10,)
    }
};
        locals.var_vgd_noswap_1 = assign26840_e40384;
        locals.var_vgd_noswap_1_dn6 = assign26840_e40384_d_n6;
        locals.var_vgd_noswap_1_dn7 = assign26840_e40384_d_n7;
        locals.var_vgd_noswap_1_dn8 = assign26840_e40384_d_n8;
        locals.var_vgd_noswap_1_dn10 = assign26840_e40384_d_n10;
        locals.var_vgd_noswap_1_rv = 0.0;

        let (assign26850_e40395, assign26850_e40395_d_n6, assign26850_e40395_d_n7, assign26850_e40395_d_n8, assign26850_e40395_d_n10,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26850_e40391: f64 = (locals.var_rgidl_i * locals.var_vg);
        let assign26850_e40393: f64 = (assign26850_e40391 - locals.var_vs);
        (assign26850_e40393, (-locals.var_vs_dn6), (-locals.var_vs_dn7), (locals.var_rgidl_i * locals.var_vg_dn8), ((locals.var_rgidl_i * locals.var_vg_dn10) - locals.var_vs_dn10),)
    } else {
        (locals.var_vgs_noswap_1, locals.var_vgs_noswap_1_dn6, locals.var_vgs_noswap_1_dn7, locals.var_vgs_noswap_1_dn8, locals.var_vgs_noswap_1_dn10,)
    }
};
        locals.var_vgs_noswap_1 = assign26850_e40395;
        locals.var_vgs_noswap_1_dn6 = assign26850_e40395_d_n6;
        locals.var_vgs_noswap_1_dn7 = assign26850_e40395_d_n7;
        locals.var_vgs_noswap_1_dn8 = assign26850_e40395_d_n8;
        locals.var_vgs_noswap_1_dn10 = assign26850_e40395_d_n10;
        locals.var_vgs_noswap_1_rv = 0.0;

        let (assign26860_e40404, assign26860_e40404_d_n3, assign26860_e40404_d_n4, assign26860_e40404_d_n5, assign26860_e40404_d_n6, assign26860_e40404_d_n7, assign26860_e40404_d_n8, assign26860_e40404_d_n9, assign26860_e40404_d_n10, assign26860_e40404_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26860_e40402: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign26860_e40402, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26860_e40404;
        locals.var_t2_dn3 = assign26860_e40404_d_n3;
        locals.var_t2_dn4 = assign26860_e40404_d_n4;
        locals.var_t2_dn5 = assign26860_e40404_d_n5;
        locals.var_t2_dn6 = assign26860_e40404_d_n6;
        locals.var_t2_dn7 = assign26860_e40404_d_n7;
        locals.var_t2_dn8 = assign26860_e40404_d_n8;
        locals.var_t2_dn9 = assign26860_e40404_d_n9;
        locals.var_t2_dn10 = assign26860_e40404_d_n10;
        locals.var_t2_dn11 = assign26860_e40404_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign26870_e40416, assign26870_e40416_d_n3, assign26870_e40416_d_n4, assign26870_e40416_d_n5, assign26870_e40416_d_n6, assign26870_e40416_d_n7, assign26870_e40416_d_n8, assign26870_e40416_d_n9, assign26870_e40416_d_n10, assign26870_e40416_d_n11,) = {
    if ((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) {
        let assign26870_e40411: f64 = (locals.var_t2 * locals.var_t2);
        let assign26870_e40413: f64 = (assign26870_e40411 + 0.0001);
        let assign26870_e40414: f64 = (assign26870_e40413).sqrt();
        (assign26870_e40414, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign26870_e40414)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign26870_e40414)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign26870_e40416;
        locals.var_vgs_eff_dn3 = assign26870_e40416_d_n3;
        locals.var_vgs_eff_dn4 = assign26870_e40416_d_n4;
        locals.var_vgs_eff_dn5 = assign26870_e40416_d_n5;
        locals.var_vgs_eff_dn6 = assign26870_e40416_d_n6;
        locals.var_vgs_eff_dn7 = assign26870_e40416_d_n7;
        locals.var_vgs_eff_dn8 = assign26870_e40416_d_n8;
        locals.var_vgs_eff_dn9 = assign26870_e40416_d_n9;
        locals.var_vgs_eff_dn10 = assign26870_e40416_d_n10;
        locals.var_vgs_eff_dn11 = assign26870_e40416_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let assign26880_e40423: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard562 = assign26880_e40423;
        locals.var_guard562_rv = 0.0;

        let (assign26890_e40432, assign26890_e40432_d_n3, assign26890_e40432_d_n4, assign26890_e40432_d_n5, assign26890_e40432_d_n6, assign26890_e40432_d_n7, assign26890_e40432_d_n8, assign26890_e40432_d_n9, assign26890_e40432_d_n10, assign26890_e40432_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26890_e40432;
        locals.var_t6_dn3 = assign26890_e40432_d_n3;
        locals.var_t6_dn4 = assign26890_e40432_d_n4;
        locals.var_t6_dn5 = assign26890_e40432_d_n5;
        locals.var_t6_dn6 = assign26890_e40432_d_n6;
        locals.var_t6_dn7 = assign26890_e40432_d_n7;
        locals.var_t6_dn8 = assign26890_e40432_d_n8;
        locals.var_t6_dn9 = assign26890_e40432_d_n9;
        locals.var_t6_dn10 = assign26890_e40432_d_n10;
        locals.var_t6_dn11 = assign26890_e40432_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign26900_e40449, assign26900_e40449_d_n3, assign26900_e40449_d_n4, assign26900_e40449_d_n5, assign26900_e40449_d_n6, assign26900_e40449_d_n7, assign26900_e40449_d_n8, assign26900_e40449_d_n9, assign26900_e40449_d_n10, assign26900_e40449_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26900_e40441: f64 = (-locals.var_vgd_noswap_1);
        let assign26900_e40443: f64 = (assign26900_e40441 - locals.var_egidl_i);
        let assign26900_e40445: f64 = (assign26900_e40443 + locals.var_vfbsdr);
        let assign26900_e40447: f64 = (assign26900_e40445 / locals.var_t0);
        (assign26900_e40447, (-((assign26900_e40445 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn6) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn7) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn8) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign26900_e40445 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_1_dn10) * locals.var_t0) - (assign26900_e40445 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign26900_e40445 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26900_e40449;
        locals.var_t1_dn3 = assign26900_e40449_d_n3;
        locals.var_t1_dn4 = assign26900_e40449_d_n4;
        locals.var_t1_dn5 = assign26900_e40449_d_n5;
        locals.var_t1_dn6 = assign26900_e40449_d_n6;
        locals.var_t1_dn7 = assign26900_e40449_d_n7;
        locals.var_t1_dn8 = assign26900_e40449_d_n8;
        locals.var_t1_dn9 = assign26900_e40449_d_n9;
        locals.var_t1_dn10 = assign26900_e40449_d_n10;
        locals.var_t1_dn11 = assign26900_e40449_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26910_e40472, assign26910_e40472_d_n3, assign26910_e40472_d_n4, assign26910_e40472_d_n5, assign26910_e40472_d_n6, assign26910_e40472_d_n7, assign26910_e40472_d_n8, assign26910_e40472_d_n9, assign26910_e40472_d_n10, assign26910_e40472_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26910_e40461: f64 = (locals.var_t1 * locals.var_t1);
        let assign26910_e40464: f64 = (4.0 * 0.01);
        let assign26910_e40466: f64 = (assign26910_e40464 * 0.01);
        let assign26910_e40467: f64 = (assign26910_e40461 + assign26910_e40466);
        let assign26910_e40468: f64 = (assign26910_e40467).sqrt();
        let assign26910_e40469: f64 = (locals.var_t1 + assign26910_e40468);
        let assign26910_e40470: f64 = (0.5 * assign26910_e40469);
        (assign26910_e40470, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign26910_e40468)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign26910_e40468)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign26910_e40472;
        locals.var_t1_dn3 = assign26910_e40472_d_n3;
        locals.var_t1_dn4 = assign26910_e40472_d_n4;
        locals.var_t1_dn5 = assign26910_e40472_d_n5;
        locals.var_t1_dn6 = assign26910_e40472_d_n6;
        locals.var_t1_dn7 = assign26910_e40472_d_n7;
        locals.var_t1_dn8 = assign26910_e40472_d_n8;
        locals.var_t1_dn9 = assign26910_e40472_d_n9;
        locals.var_t1_dn10 = assign26910_e40472_d_n10;
        locals.var_t1_dn11 = assign26910_e40472_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign26920_e40486, assign26920_e40486_d_n3, assign26920_e40486_d_n4, assign26920_e40486_d_n5, assign26920_e40486_d_n6, assign26920_e40486_d_n7, assign26920_e40486_d_n8, assign26920_e40486_d_n9, assign26920_e40486_d_n10, assign26920_e40486_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26920_e40483: f64 = (locals.var_t1 + 0.001);
        let assign26920_e40484: f64 = (locals.var_bgidl_t / assign26920_e40483);
        (assign26920_e40484, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign26920_e40483 * assign26920_e40483))), (((locals.var_bgidl_t_dn4 * assign26920_e40483) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign26920_e40483 * assign26920_e40483)), (((locals.var_bgidl_t_dn5 * assign26920_e40483) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign26920_e40483 * assign26920_e40483)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign26920_e40483 * assign26920_e40483))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign26920_e40483 * assign26920_e40483))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign26920_e40486;
        locals.var_t2_dn3 = assign26920_e40486_d_n3;
        locals.var_t2_dn4 = assign26920_e40486_d_n4;
        locals.var_t2_dn5 = assign26920_e40486_d_n5;
        locals.var_t2_dn6 = assign26920_e40486_d_n6;
        locals.var_t2_dn7 = assign26920_e40486_d_n7;
        locals.var_t2_dn8 = assign26920_e40486_d_n8;
        locals.var_t2_dn9 = assign26920_e40486_d_n9;
        locals.var_t2_dn10 = assign26920_e40486_d_n10;
        locals.var_t2_dn11 = assign26920_e40486_d_n11;
        locals.var_t2_rv = 0.0;

        let assign26930_e40489: f64 = if locals.var_kgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign26930_e40489;
        locals.var_guard563_rv = 0.0;

        let (assign26940_e40504, assign26940_e40504_d_n3, assign26940_e40504_d_n4, assign26940_e40504_d_n5, assign26940_e40504_d_n6, assign26940_e40504_d_n7, assign26940_e40504_d_n8, assign26940_e40504_d_n9, assign26940_e40504_d_n10, assign26940_e40504_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign26940_e40500: f64 = (-locals.var_vdb_noswap);
        let assign26940_e40502: f64 = (assign26940_e40500 - locals.var_fgidl_i);
        (assign26940_e40502, 0.0, 0.0, 0.0, (-locals.var_vdb_noswap_dn6), (-locals.var_vdb_noswap_dn7), 0.0, 0.0, (-locals.var_vdb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign26940_e40504;
        locals.var_t3_dn3 = assign26940_e40504_d_n3;
        locals.var_t3_dn4 = assign26940_e40504_d_n4;
        locals.var_t3_dn5 = assign26940_e40504_d_n5;
        locals.var_t3_dn6 = assign26940_e40504_d_n6;
        locals.var_t3_dn7 = assign26940_e40504_d_n7;
        locals.var_t3_dn8 = assign26940_e40504_d_n8;
        locals.var_t3_dn9 = assign26940_e40504_d_n9;
        locals.var_t3_dn10 = assign26940_e40504_d_n10;
        locals.var_t3_dn11 = assign26940_e40504_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign26950_e40518, assign26950_e40518_d_n3, assign26950_e40518_d_n4, assign26950_e40518_d_n5, assign26950_e40518_d_n6, assign26950_e40518_d_n7, assign26950_e40518_d_n8, assign26950_e40518_d_n9, assign26950_e40518_d_n10, assign26950_e40518_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        let assign26950_e40516: f64 = (locals.var_t3 + 0.0001);
        (assign26950_e40516, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign26950_e40518;
        locals.var_t4_dn3 = assign26950_e40518_d_n3;
        locals.var_t4_dn4 = assign26950_e40518_d_n4;
        locals.var_t4_dn5 = assign26950_e40518_d_n5;
        locals.var_t4_dn6 = assign26950_e40518_d_n6;
        locals.var_t4_dn7 = assign26950_e40518_d_n7;
        locals.var_t4_dn8 = assign26950_e40518_d_n8;
        locals.var_t4_dn9 = assign26950_e40518_d_n9;
        locals.var_t4_dn10 = assign26950_e40518_d_n10;
        locals.var_t4_dn11 = assign26950_e40518_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign26960_e40551, assign26960_e40551_d_n3, assign26960_e40551_d_n4, assign26960_e40551_d_n5, assign26960_e40551_d_n6, assign26960_e40551_d_n7, assign26960_e40551_d_n8, assign26960_e40551_d_n9, assign26960_e40551_d_n10, assign26960_e40551_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign26960_e40531: f64 = (locals.var_kgidl_i * __rspice_inv_cse_1);
        let assign26960_e40534: f64 = (locals.var_kgidl_i * __rspice_inv_cse_1);
        let assign26960_e40537: f64 = (locals.var_kgidl_i * __rspice_inv_cse_1);
        let assign26960_e40538: f64 = (assign26960_e40534 * assign26960_e40537);
        let assign26960_e40541: f64 = (4.0 * 1e-6);
        let assign26960_e40543: f64 = (assign26960_e40541 * 1e-6);
        let assign26960_e40544: f64 = (assign26960_e40538 + assign26960_e40543);
        let assign26960_e40545: f64 = (assign26960_e40544).sqrt();
        let assign26960_e40546: f64 = (assign26960_e40531 + assign26960_e40545);
        let assign26960_e40547: f64 = (0.5 * assign26960_e40546);
        let assign26960_e40549: f64 = (assign26960_e40547 - 1e-6);
        (assign26960_e40549, (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign26960_e40537) + (assign26960_e40534 * (-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign26960_e40545)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26960_e40551;
        locals.var_t5_dn3 = assign26960_e40551_d_n3;
        locals.var_t5_dn4 = assign26960_e40551_d_n4;
        locals.var_t5_dn5 = assign26960_e40551_d_n5;
        locals.var_t5_dn6 = assign26960_e40551_d_n6;
        locals.var_t5_dn7 = assign26960_e40551_d_n7;
        locals.var_t5_dn8 = assign26960_e40551_d_n8;
        locals.var_t5_dn9 = assign26960_e40551_d_n9;
        locals.var_t5_dn10 = assign26960_e40551_d_n10;
        locals.var_t5_dn11 = assign26960_e40551_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign26970_e40564, assign26970_e40564_d_n3, assign26970_e40564_d_n4, assign26970_e40564_d_n5, assign26970_e40564_d_n6, assign26970_e40564_d_n7, assign26970_e40564_d_n8, assign26970_e40564_d_n9, assign26970_e40564_d_n10, assign26970_e40564_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign26970_e40564;
        locals.var_t5_dn3 = assign26970_e40564_d_n3;
        locals.var_t5_dn4 = assign26970_e40564_d_n4;
        locals.var_t5_dn5 = assign26970_e40564_d_n5;
        locals.var_t5_dn6 = assign26970_e40564_d_n6;
        locals.var_t5_dn7 = assign26970_e40564_d_n7;
        locals.var_t5_dn8 = assign26970_e40564_d_n8;
        locals.var_t5_dn9 = assign26970_e40564_d_n9;
        locals.var_t5_dn10 = assign26970_e40564_d_n10;
        locals.var_t5_dn11 = assign26970_e40564_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign26980_e40585, assign26980_e40585_d_n3, assign26980_e40585_d_n4, assign26980_e40585_d_n5, assign26980_e40585_d_n6, assign26980_e40585_d_n7, assign26980_e40585_d_n8, assign26980_e40585_d_n9, assign26980_e40585_d_n10, assign26980_e40585_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard562 == 0.0)) {
        let assign26980_e40574: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign26980_e40576: f64 = (assign26980_e40574 * locals.var_t1);
        let assign26980_e40578: f64 = (-locals.var_t2);
        let assign26980_e40579: f64 = { let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26980_e40580: f64 = (assign26980_e40576 * assign26980_e40579);
        let assign26980_e40582: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign26980_e40583: f64 = (assign26980_e40580 * assign26980_e40582);
        (assign26980_e40583, (((((assign26980_e40574 * locals.var_t1_dn3) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign26980_e40574 * locals.var_t1_dn4) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign26980_e40574 * locals.var_t1_dn5) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign26980_e40574 * locals.var_t1_dn6) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign26980_e40574 * locals.var_t1_dn7) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign26980_e40574 * locals.var_t1_dn8) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign26980_e40574 * locals.var_t1_dn9) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign26980_e40574 * locals.var_t1_dn10) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign26980_e40574 * locals.var_t1_dn11) * assign26980_e40579) + (assign26980_e40576 * ({ let limited_exp_arg = assign26980_e40578; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign26980_e40582) + (assign26980_e40580 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign26980_e40585;
        locals.var_t6_dn3 = assign26980_e40585_d_n3;
        locals.var_t6_dn4 = assign26980_e40585_d_n4;
        locals.var_t6_dn5 = assign26980_e40585_d_n5;
        locals.var_t6_dn6 = assign26980_e40585_d_n6;
        locals.var_t6_dn7 = assign26980_e40585_d_n7;
        locals.var_t6_dn8 = assign26980_e40585_d_n8;
        locals.var_t6_dn9 = assign26980_e40585_d_n9;
        locals.var_t6_dn10 = assign26980_e40585_d_n10;
        locals.var_t6_dn11 = assign26980_e40585_d_n11;
        locals.var_t6_rv = 0.0;

        let assign27000_e40599: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard564 = assign27000_e40599;
        locals.var_guard564_rv = 0.0;

        let (assign27010_e40608, assign27010_e40608_d_n3, assign27010_e40608_d_n4, assign27010_e40608_d_n5, assign27010_e40608_d_n6, assign27010_e40608_d_n7, assign27010_e40608_d_n8, assign27010_e40608_d_n9, assign27010_e40608_d_n10, assign27010_e40608_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign27010_e40608;
        locals.var_t6_dn3 = assign27010_e40608_d_n3;
        locals.var_t6_dn4 = assign27010_e40608_d_n4;
        locals.var_t6_dn5 = assign27010_e40608_d_n5;
        locals.var_t6_dn6 = assign27010_e40608_d_n6;
        locals.var_t6_dn7 = assign27010_e40608_d_n7;
        locals.var_t6_dn8 = assign27010_e40608_d_n8;
        locals.var_t6_dn9 = assign27010_e40608_d_n9;
        locals.var_t6_dn10 = assign27010_e40608_d_n10;
        locals.var_t6_dn11 = assign27010_e40608_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign27020_e40625, assign27020_e40625_d_n3, assign27020_e40625_d_n4, assign27020_e40625_d_n5, assign27020_e40625_d_n6, assign27020_e40625_d_n7, assign27020_e40625_d_n8, assign27020_e40625_d_n9, assign27020_e40625_d_n10, assign27020_e40625_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27020_e40617: f64 = (-locals.var_vgs_noswap_1);
        let assign27020_e40619: f64 = (assign27020_e40617 - locals.var_egisl_i);
        let assign27020_e40621: f64 = (assign27020_e40619 + locals.var_vfbsdr);
        let assign27020_e40623: f64 = (assign27020_e40621 / locals.var_t0);
        (assign27020_e40623, (-((assign27020_e40621 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn6) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn7) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn8) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign27020_e40621 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_1_dn10) * locals.var_t0) - (assign27020_e40621 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign27020_e40621 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27020_e40625;
        locals.var_t1_dn3 = assign27020_e40625_d_n3;
        locals.var_t1_dn4 = assign27020_e40625_d_n4;
        locals.var_t1_dn5 = assign27020_e40625_d_n5;
        locals.var_t1_dn6 = assign27020_e40625_d_n6;
        locals.var_t1_dn7 = assign27020_e40625_d_n7;
        locals.var_t1_dn8 = assign27020_e40625_d_n8;
        locals.var_t1_dn9 = assign27020_e40625_d_n9;
        locals.var_t1_dn10 = assign27020_e40625_d_n10;
        locals.var_t1_dn11 = assign27020_e40625_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27030_e40648, assign27030_e40648_d_n3, assign27030_e40648_d_n4, assign27030_e40648_d_n5, assign27030_e40648_d_n6, assign27030_e40648_d_n7, assign27030_e40648_d_n8, assign27030_e40648_d_n9, assign27030_e40648_d_n10, assign27030_e40648_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27030_e40637: f64 = (locals.var_t1 * locals.var_t1);
        let assign27030_e40640: f64 = (4.0 * 0.01);
        let assign27030_e40642: f64 = (assign27030_e40640 * 0.01);
        let assign27030_e40643: f64 = (assign27030_e40637 + assign27030_e40642);
        let assign27030_e40644: f64 = (assign27030_e40643).sqrt();
        let assign27030_e40645: f64 = (locals.var_t1 + assign27030_e40644);
        let assign27030_e40646: f64 = (0.5 * assign27030_e40645);
        (assign27030_e40646, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign27030_e40644)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign27030_e40644)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign27030_e40648;
        locals.var_t1_dn3 = assign27030_e40648_d_n3;
        locals.var_t1_dn4 = assign27030_e40648_d_n4;
        locals.var_t1_dn5 = assign27030_e40648_d_n5;
        locals.var_t1_dn6 = assign27030_e40648_d_n6;
        locals.var_t1_dn7 = assign27030_e40648_d_n7;
        locals.var_t1_dn8 = assign27030_e40648_d_n8;
        locals.var_t1_dn9 = assign27030_e40648_d_n9;
        locals.var_t1_dn10 = assign27030_e40648_d_n10;
        locals.var_t1_dn11 = assign27030_e40648_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign27040_e40662, assign27040_e40662_d_n3, assign27040_e40662_d_n4, assign27040_e40662_d_n5, assign27040_e40662_d_n6, assign27040_e40662_d_n7, assign27040_e40662_d_n8, assign27040_e40662_d_n9, assign27040_e40662_d_n10, assign27040_e40662_d_n11,) = {
    if (((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) {
        let assign27040_e40659: f64 = (locals.var_t1 + 0.001);
        let assign27040_e40660: f64 = (locals.var_bgisl_t / assign27040_e40659);
        (assign27040_e40660, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign27040_e40659 * assign27040_e40659))), (((locals.var_bgisl_t_dn4 * assign27040_e40659) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign27040_e40659 * assign27040_e40659)), (((locals.var_bgisl_t_dn5 * assign27040_e40659) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign27040_e40659 * assign27040_e40659)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign27040_e40659 * assign27040_e40659))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign27040_e40659 * assign27040_e40659))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign27040_e40662;
        locals.var_t2_dn3 = assign27040_e40662_d_n3;
        locals.var_t2_dn4 = assign27040_e40662_d_n4;
        locals.var_t2_dn5 = assign27040_e40662_d_n5;
        locals.var_t2_dn6 = assign27040_e40662_d_n6;
        locals.var_t2_dn7 = assign27040_e40662_d_n7;
        locals.var_t2_dn8 = assign27040_e40662_d_n8;
        locals.var_t2_dn9 = assign27040_e40662_d_n9;
        locals.var_t2_dn10 = assign27040_e40662_d_n10;
        locals.var_t2_dn11 = assign27040_e40662_d_n11;
        locals.var_t2_rv = 0.0;

        let assign27050_e40665: f64 = if locals.var_kgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign27050_e40665;
        locals.var_guard565_rv = 0.0;

        let (assign27060_e40680, assign27060_e40680_d_n3, assign27060_e40680_d_n4, assign27060_e40680_d_n5, assign27060_e40680_d_n6, assign27060_e40680_d_n7, assign27060_e40680_d_n8, assign27060_e40680_d_n9, assign27060_e40680_d_n10, assign27060_e40680_d_n11,) = {
    if ((((locals.var_guard492 != 0.0) && (locals.var_guard557 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        let assign27060_e40676: f64 = (-locals.var_vsb_noswap);
        let assign27060_e40678: f64 = (assign27060_e40676 - locals.var_fgisl_i);
        (assign27060_e40678, 0.0, 0.0, 0.0, (-locals.var_vsb_noswap_dn6), (-locals.var_vsb_noswap_dn7), 0.0, 0.0, (-locals.var_vsb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign27060_e40680;
        locals.var_t3_dn3 = assign27060_e40680_d_n3;
        locals.var_t3_dn4 = assign27060_e40680_d_n4;
        locals.var_t3_dn5 = assign27060_e40680_d_n5;
        locals.var_t3_dn6 = assign27060_e40680_d_n6;
        locals.var_t3_dn7 = assign27060_e40680_d_n7;
        locals.var_t3_dn8 = assign27060_e40680_d_n8;
        locals.var_t3_dn9 = assign27060_e40680_d_n9;
        locals.var_t3_dn10 = assign27060_e40680_d_n10;
        locals.var_t3_dn11 = assign27060_e40680_d_n11;
        locals.var_t3_rv = 0.0;

    }
}
